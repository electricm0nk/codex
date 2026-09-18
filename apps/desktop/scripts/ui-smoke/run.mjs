#!/usr/bin/env node
// Headless UI smoke runner: drives the real Tauri + React app under Xvfb
// (via .claude/skills/run-desktop/driver.sh) and asserts against DOM truth
// read back from uiProbe.ts's live snapshot (via ui_probe.rs), instead of
// hard-coded pixel coordinates or a screenshot a human has to eyeball.
//
// Usage:
//   RUN_DESKTOP_AGENT=<unique> node scripts/ui-smoke/run.mjs [--only <id>]
//     [--from <id>] [--out <dir>] [--keep]
//
// See spec.json's own top-level "$comment" for the row schema, and
// SKILL.md / verify-on-screen.sh for the concurrency rules this runner
// inherits (RUN_DESKTOP_AGENT must be unique per concurrently-dispatched
// agent; never 'default').
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import * as driver from './lib/driver.mjs';
import { extractScreenText } from './lib/clipboard.mjs';
import { sendCommand } from './lib/commandChannel.mjs';
import {
  centerOf,
  findSelect,
  findTarget,
  pollForFreshProbe,
  readProbeFile,
  screenContains,
  sleepMs,
} from './lib/probe.mjs';

const HERE = dirname(fileURLToPath(import.meta.url));
const SPEC_PATH = join(HERE, 'spec.json');

// Applied to EVERY row on top of its own `forbid` list (spec.json's own
// top-level "$comment" documents this same list so it doesn't drift silently
// out of sync with the code that enforces it).
const GLOBAL_FORBID = [
  'could be read from the corpus',
  'failure',
  'Failed to load',
  'requires the desktop runtime',
  'undefined',
  'NaN',
  '[object Object]',
];

// Names that, when clicked in sequence, are expected to walk any screen back
// to the landing page. Reset-to-landing tries each, in order, against
// whatever is currently on screen, re-reading the probe between clicks.
const RESET_CLICK_NAMES = ['Back', 'Cancel', 'Close', '✕', '×', 'Close settings', 'Close character'];
// The candidates reset-to-landing clicks specifically to dismiss an OPEN
// dialog/modal (see resetToLanding's own comment) -- same idea as
// RESET_CLICK_NAMES but tried only after Escape, and only while the probe
// reports at least one open `role=dialog`.
const RESET_CLOSE_NAMES = ['Close settings', 'Close', '✕', '×', 'Cancel', 'Back'];
// Any one of these being present is treated as "we are on the landing screen".
const LANDING_TARGET_NAMES = ['New\nCharacter', 'Load\nCharacter'];
const LANDING_MARKER_PREFIX = 'Browse ';

function parseArgs(argv) {
  const args = { only: null, from: null, out: null, keep: false, xdotool: false };
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg === '--only') {
      args.only = argv[++i];
    } else if (arg === '--from') {
      args.from = argv[++i];
    } else if (arg === '--out') {
      args.out = argv[++i];
    } else if (arg === '--keep') {
      args.keep = true;
    } else if (arg === '--xdotool') {
      // Drives click/type/key via `driver.sh` (xdotool) instead of the DOM
      // command channel -- kept only for direct comparison against the
      // channel's own behavior; not the default path.
      args.xdotool = true;
    } else {
      throw new Error(`unrecognized argument: ${arg}`);
    }
  }
  return args;
}

function loadSpec() {
  const parsed = JSON.parse(readFileSync(SPEC_PATH, 'utf8'));
  const byId = new Map(parsed.rows.map((row) => [row.id, row]));
  return { rows: parsed.rows, byId };
}

function selectRows(spec, args) {
  let rows = spec.rows;
  if (args.only) {
    const row = spec.byId.get(args.only);
    if (!row) {
      throw new Error(`--only ${args.only}: no such row id in spec.json`);
    }
    return [row];
  }
  if (args.from) {
    const startIndex = rows.findIndex((row) => row.id === args.from);
    if (startIndex === -1) {
      throw new Error(`--from ${args.from}: no such row id in spec.json`);
    }
    rows = rows.slice(startIndex);
  }
  return rows;
}

// ------------------------------------------------------------- probe access
let probePath;
let cmdPath;
let lastSeenTs = 0;
// Set from --xdotool in main() before any row runs. Default (false) routes
// click/type/select/key through the DOM command channel; true falls back to
// driver.sh (xdotool) for direct comparison -- see parseArgs' own comment.
let USE_XDOTOOL = false;

function latestSnapshot({ requireFresh = false, timeoutMs = 3000 } = {}) {
  const { snapshot, fresh } = pollForFreshProbe(probePath, requireFresh ? lastSeenTs : 0, timeoutMs);
  if (snapshot) {
    lastSeenTs = Math.max(lastSeenTs, snapshot.ts ?? 0);
  }
  return { snapshot, fresh };
}

// ----------------------------------------------------------- step execution
const VIEWPORT_SCROLL_THRESHOLD_Y = 1150;
// How long clickByName will keep polling for a target that isn't on screen
// YET before giving up. The screen just navigated to (by a preceding step in
// the same row) can mount its own content -- a search box, a tab bar -- a
// beat after the navigation-triggering mutation itself, e.g. behind an async
// catalog-data fetch; a single immediate read races that and throws a false
// TargetNotFoundError for an element that was about to exist. Observed
// directly: rows whose FIRST click (from the already-settled landing screen)
// succeeded but whose SECOND click (into the just-mounted next screen) did
// not find its target on the first try.
//
// Also has to absorb the command CHANNEL's own worst case, not just DOM
// mount delay: `clickByNameChannel` sends through `poll_ui_probe_command`,
// which the frontend only drains between one `invoke()` round trip and the
// next (uiProbe.ts's `startCommandChannel` -- deliberately serialized, one
// in-flight poll at a time). That same file documents this webview's
// input-to-paint / IPC latency as varying "anywhere from well under a
// second up to 60+ seconds for the SAME click on the SAME element in
// different sessions, with no observed correlation to system load" -- and a
// single such stall blocks the WHOLE poll chain, not just the one command
// racing it. A 10s budget was sized only for the mount-delay case above and
// is well short of that documented ceiling: reproduced directly (2026-09-18)
// as a cascade -- one command-channel stall mid-run left FOUR consecutive
// rows (settings-tab-bug, settings-tab-enhancement, settings-tab-developer,
// settings-close-esc) all timing out on an element that was genuinely on
// screen the whole time (each row's own resetToLanding/setup immediately
// re-enters the same still-stalled channel, so the cascade doesn't stop
// until the underlying stall itself clears), while the very next command
// sent once the stall cleared succeeded immediately. 75s was meant to
// comfortably clear the documented 60s+ ceiling, but the ceiling itself
// moved: reproduced directly (2026-09-18, equipment-catalog-open-and-search)
// as a single stall measured end-to-end at ~80s -- a lone `click` command
// (no retry loop involved) sent from a settled landing screen produced
// zero probe change for 78+ seconds and then the click's own navigation
// AND the next screen's full async data load both appeared in the very
// next poll, meaning the whole webview/IPC pipeline was frozen for that
// stretch, not merely slow to fetch data (a slow-but-alive pipeline would
// have shown the navigated-to screen's own "loading" state well before its
// data arrived; none did). Unrelated to which screen or how much data it
// loads -- the earlier settings-tab cascade this constant was first raised
// for carries no large payload either. 150s gives headroom above the newly
// observed ~80s instance the same way 75s gave headroom above the
// previously observed ceiling.
const CLICK_TARGET_WAIT_MS = 150000;

class TargetNotFoundError extends Error {
  constructor(name) {
    super(`target not found: '${name}'`);
    this.name = 'TargetNotFoundError';
  }
}

/**
 * Clicks the named target through the DOM command channel: the webview
 * itself finds the element (same name-matching the probe uses -- see
 * `uiProbe.ts`'s `matchCommandTarget`), scrolls it into view, and calls
 * `el.click()`. Retries the *send* (not just the wait) for up to
 * `CLICK_TARGET_WAIT_MS`, because the target this call is racing may not be
 * mounted yet at all (a screen the previous step just navigated to can
 * mount its own content -- a search box, a tab bar -- a beat later, see
 * `CLICK_TARGET_WAIT_MS`'s own comment); a single command's own
 * `sendCommand` ack timeout is much shorter and is not what's being waited
 * out here.
 */
function clickByNameChannel(name) {
  const deadline = Date.now() + CLICK_TARGET_WAIT_MS;
  let result;
  for (;;) {
    result = sendCommand(cmdPath, probePath, { op: 'click', target: name }, { timeoutMs: 2000 });
    if (result.ok || Date.now() >= deadline) {
      break;
    }
    sleepMs(150);
  }
  if (!result.ok) {
    throw new TargetNotFoundError(`${name}${result.error ? ` (${result.error})` : ''}`);
  }
}

/** Clicks the named target via xdotool, scrolling it into view first if the probe says it's below the fold. Only reached with --xdotool. */
function clickByNameXdotool(name) {
  const deadline = Date.now() + CLICK_TARGET_WAIT_MS;
  let snapshot;
  let target;
  for (;;) {
    ({ snapshot } = latestSnapshot());
    target = findTarget(snapshot, name);
    if (target || Date.now() >= deadline) {
      break;
    }
    sleepMs(150);
  }
  if (target && target.rect.y > VIEWPORT_SCROLL_THRESHOLD_Y) {
    driver.scroll(960, 600, 6);
    ({ snapshot } = latestSnapshot({ requireFresh: true, timeoutMs: 3000 }));
    target = findTarget(snapshot, name);
  }
  if (!target) {
    throw new TargetNotFoundError(name);
  }
  const { x, y } = centerOf(target.rect);
  const baselineTs = snapshot?.ts ?? 0;
  driver.click(x, y);
  // The first click issued after any idle gap in this webview (the very
  // first click of a fresh launch; also, empirically, a row's own first
  // click after the gap since the previous row) can be silently swallowed
  // -- no exception, no DOM mutation, and an identical click moments later
  // succeeds with nothing else different. A same-coordinate immediate
  // re-click was tried as a fix and reverted: firing two clicks ~1.2s apart
  // was caught putting WebKitGTK into a stuck state where every further
  // click AND key event stopped registering for the rest of the process's
  // life. 2000ms is well outside any double-click gesture window (GTK's own
  // default multi-click timeout is a few hundred ms), so a single retry
  // after a full 2s wait targets the same swallowed-first-click pattern
  // without that risk.
  const { fresh } = pollForFreshProbe(probePath, baselineTs, 2000, 100);
  if (!fresh) {
    driver.click(x, y);
  }
}

function clickByName(name) {
  return USE_XDOTOOL ? clickByNameXdotool(name) : clickByNameChannel(name);
}

/**
 * Types into whatever currently has focus -- always preceded by a `click`
 * step in every row (see spec.json). Retries like `clickByNameChannel`/
 * `selectByName` (up to `CLICK_TARGET_WAIT_MS`, not a single 5s shot): this
 * command rides the exact same command channel and is subject to the exact
 * same documented webview stall (see `CLICK_TARGET_WAIT_MS`'s own comment --
 * "anywhere from well under a second up to 60+ seconds ... with no observed
 * correlation to system load", confirmed directly on multiple unrelated
 * screens), so giving it a short non-retried timeout while `click`/`select`
 * get a long retried one is an inconsistency in the harness, not a real
 * per-op difference -- whichever command happens to be in flight when a
 * stall hits was failing fast here while its neighbors rode it out.
 */
function typeText(text) {
  if (USE_XDOTOOL) {
    driver.type(text);
    return;
  }
  const deadline = Date.now() + CLICK_TARGET_WAIT_MS;
  let result;
  for (;;) {
    result = sendCommand(cmdPath, probePath, { op: 'type', text }, { timeoutMs: 2000 });
    if (result.ok || Date.now() >= deadline) {
      break;
    }
    sleepMs(150);
  }
  if (!result.ok) {
    throw new Error(`type command failed: ${result.error ?? 'unknown error'}`);
  }
}

/**
 * Picks `text` (an <option>'s own text/value) on the `<select>` located by
 * `target`. DOM-command-channel only (`executeCommand`'s own `select` case
 * in uiProbe.ts already supported this op; run.mjs just never exposed it
 * before now) -- added because CreateCharacterForm's race/class selects
 * carry no accessible name of their own (`nameOf()` reports a `<select>`'s
 * full concatenated option text, since it has non-empty textContent, never
 * its `id`), so `target` must be a substring unique to the WANTED select's
 * option list (e.g. an option's own text), not the select's id. No
 * `--xdotool` fallback: this op did not exist before, so there is no prior
 * xdotool-based behavior to preserve parity with.
 */
function selectByName(target, text) {
  const deadline = Date.now() + CLICK_TARGET_WAIT_MS;
  let result;
  for (;;) {
    result = sendCommand(cmdPath, probePath, { op: 'select', target, text }, { timeoutMs: 2000 });
    if (result.ok || Date.now() >= deadline) {
      break;
    }
    sleepMs(150);
  }
  if (!result.ok) {
    throw new TargetNotFoundError(`${target}${result.error ? ` (${result.error})` : ''}`);
  }
}

/**
 * Dispatches a key (e.g. `Escape`) against `document.activeElement`.
 *
 * Retries like `clickByNameChannel`/`selectByName`/`typeText` (up to
 * `CLICK_TARGET_WAIT_MS`), for the same reason `typeText` does (see its own
 * comment): this is the exact defect reproduced live 2026-09-18 against
 * `sheet-action-add-armor-gear-picker`, whose own spec.json notes describe
 * "the 'key: Escape' command's own ack times out (5000ms) after Add Armor's
 * picker opens" and treat it as a suspected app-side main-thread block
 * specific to the armor catalog. It is not: `buildItemPickerConfig` routes
 * Add Weapon and Add Armor through the IDENTICAL `loadEquipment('ArmsArmor')`
 * query and the same catalog-agnostic `ItemPickerModal`/`mapEquipmentCatalogEntries`
 * (a plain, uncomputed `.map()`, nothing prerequisite-checked or O(n^2)) --
 * there is no armor-specific render path for anything to be slow in.
 * Reproduced directly instead: a lone `key: Escape` command genuinely
 * un-acknowledged for 24+ seconds (a fresh command sent moments later
 * completed in ~100ms), matching this file's own already-documented,
 * screen-independent webview stall (`CLICK_TARGET_WAIT_MS`'s comment). The
 * bug was this function alone never getting the same retry budget every
 * other command-channel op already has, so the one command that happened to
 * be in flight when a stall hit failed the whole row (and, via a dialog left
 * open behind it, blocked every row after it) while an identical stall
 * hitting a `click` step would have been silently ridden out.
 */
function pressKey(key) {
  if (USE_XDOTOOL) {
    driver.key(key);
    return;
  }
  const deadline = Date.now() + CLICK_TARGET_WAIT_MS;
  let result;
  for (;;) {
    result = sendCommand(cmdPath, probePath, { op: 'key', key }, { timeoutMs: 2000 });
    if (result.ok || Date.now() >= deadline) {
      break;
    }
    sleepMs(150);
  }
  if (!result.ok) {
    throw new Error(`key command failed: ${result.error ?? 'unknown error'}`);
  }
}

function runStep(step) {
  switch (step.op) {
    case 'click':
      clickByName(step.target);
      break;
    case 'type':
      typeText(step.text);
      break;
    case 'select':
      selectByName(step.target, step.text);
      break;
    case 'key':
      pressKey(step.key);
      break;
    case 'scroll':
      driver.scroll(960, 600, step.ticks ?? 5, step.direction ?? 'down');
      break;
    case 'wait':
      sleepMs(step.ms ?? 500);
      break;
    default:
      throw new Error(`unknown step op: ${step.op}`);
  }
  // Let the debounced MutationObserver (~150ms) settle before the next step
  // reads the probe.
  sleepMs(220);
}

function runSteps(steps) {
  for (const step of steps) {
    runStep(step);
  }
}

// -------------------------------------------------------- reset-to-landing
function isOnLanding(snapshot) {
  if (!snapshot) return false;
  // A modal (e.g. SettingsModal) overlays the landing screen without
  // unmounting it -- LandingScreen's own targets (New/Load Character, the
  // Browse-* links) stay present in the DOM, just visually covered. Without
  // this check, isOnLanding() false-positives "already on landing" while a
  // dialog is still open from a previous row, so resetToLanding() skips
  // closing it. The next row's own "open settings" click then lands on the
  // now-covering modal backdrop (position:fixed, zIndex 1000, onClick=
  // onClose) instead of the real gear button underneath (zIndex 950),
  // closing the modal instead of opening it -- observed directly as a
  // strict pass/fail alternation across consecutive runs of the same row.
  if (Array.isArray(snapshot.dialogs) && snapshot.dialogs.length > 0) {
    return false;
  }
  if (LANDING_TARGET_NAMES.some((name) => findTarget(snapshot, name))) {
    return true;
  }
  return (snapshot.targets ?? []).some((t) => t.name.startsWith(LANDING_MARKER_PREFIX));
}

/** Returns true on success, false if landing was not reached within the try budget (row should be blocked). */
function resetToLanding(maxAttempts = 6) {
  let { snapshot } = latestSnapshot();
  for (let attempt = 0; attempt < maxAttempts; attempt += 1) {
    if (isOnLanding(snapshot)) {
      return true;
    }
    if (Array.isArray(snapshot?.dialogs) && snapshot.dialogs.length > 0) {
      // A dialog/modal is open (isOnLanding's own dialog guard is exactly
      // why this branch, not the plain click-a-recognized-name branch
      // below, runs first): Escape closes most of this app's dialogs
      // outright (LevelUpDialog, SkillAllocationDialog, ItemPickerModal,
      // ThemeBrowserModal all wire a keydown handler for it), but
      // SettingsModal historically did not reliably respond to a lone
      // Escape from this harness -- so this also tries a named close/
      // cancel control every attempt, not only as a last resort once
      // nothing else is recognized (the previous behavior, which is what
      // let a still-open SettingsModal block every row after it for the
      // rest of a run -- see docs/release/SD-36-consolidation/artifacts/
      // ui-smoke/final/RECEIPT.md's "4 blocked" rows).
      //
      // `pressKey` can throw (a `key` command that never gets acknowledged
      // within its own timeout, e.g. under a transient command-channel
      // backlog) -- reset-to-landing is a best-effort retry loop, so a
      // single failed Escape must fall through to the next attempt rather
      // than aborting the whole row the way an uncaught exception would.
      try {
        pressKey('Escape');
      } catch {
        // fall through to the next attempt regardless
      }
      sleepMs(220);
      ({ snapshot } = latestSnapshot());
      if (isOnLanding(snapshot)) {
        return true;
      }
      const closeName = RESET_CLOSE_NAMES.find((name) => findTarget(snapshot, name));
      if (closeName) {
        try {
          clickByName(closeName);
        } catch {
          // The close/cancel control the probe just reported can still
          // vanish before the click lands (another close path won the
          // race) -- fall through to the next attempt either way.
        }
        sleepMs(300);
      }
      ({ snapshot } = latestSnapshot({ requireFresh: false, timeoutMs: 2000 }));
      continue;
    }
    const clickName = RESET_CLICK_NAMES.find((name) => findTarget(snapshot, name));
    if (!clickName) {
      // Nothing we recognize to click and no dialog reported — try Escape
      // once anyway (covers a dialog the probe hasn't caught up to yet)
      // then re-check before giving up this attempt. Same best-effort
      // posture as the dialog branch above: a failed Escape falls through
      // rather than aborting the row.
      try {
        pressKey('Escape');
      } catch {
        // fall through to the next attempt regardless
      }
      sleepMs(220);
      ({ snapshot } = latestSnapshot());
      continue;
    }
    // `clickByName` (the command-channel path), not a raw `driver.click(x,
    // y)` off the probe's own rect: the row that just ran can leave the
    // sheet scrolled (e.g. sheet-action-add-armor-gear-picker's own Gear
    // tab), so a recognized close target's rect can report a negative
    // window-relative y -- off the top of the visible viewport. A direct
    // pixel click at that coordinate lands nowhere (or on whatever else is
    // actually there), so the target is never really clicked and every
    // subsequent attempt re-finds the same still-present target -- observed
    // directly as this exact loop exhausting all 6 attempts on
    // sheet-action-add-armor-gear-picker's own reset, with the probe
    // reporting the recognized '✕' target the whole time.
    // `clickByNameChannel` has the webview itself find the element by name
    // and scroll it into view before calling `el.click()` (see its own
    // comment), which is immune to this. Same best-effort posture as every
    // other branch in this loop: a failed click falls through to the next
    // attempt rather than aborting the row.
    try {
      clickByName(clickName);
    } catch {
      // fall through to the next attempt regardless
    }
    sleepMs(300);
    ({ snapshot } = latestSnapshot({ requireFresh: true, timeoutMs: 2000 }));
  }
  return isOnLanding(snapshot);
}

// ------------------------------------------------------------- assertions
// This webview's input-to-paint latency under Xvfb/software rendering has
// been measured directly to vary wildly -- anywhere from well under a
// second up to 60+ seconds for the SAME click on the SAME element in
// different sessions, with no observed correlation to system load (CPU/RAM
// were idle-normal throughout). 12s is a practical compromise: generous
// enough to absorb most of the variance seen without making a full spec
// walk impractically slow; a row that still needs longer than this is
// flagged RED and the cause (harness timeout vs. real app defect) should be
// re-checked by hand rather than assumed either way.
//
// **Finding the marker is not the same as the screen being settled.** A
// screen whose marker is a static heading present from the very first paint
// (e.g. CreateCharacterForm's "Create a character", rendered by the parent
// page regardless of the form's own load state) used to make this function
// return within one poll interval of the triggering click -- long before any
// of that screen's OWN async content (a race roster, a class-catalog
// preview, an alternate-traits menu, all independently `useEffect`-fetched)
// had a chance to resolve. `assertRow`'s blanket 'Loading' forbid check then
// fired against a screen the row's own wait budget never actually spent any
// time waiting out -- observed directly: create-character-render read
// "'Loading' still present after the row's wait budget" while its own
// underlying fetch (`list_race_creation_roster`) in fact completed in well
// under a second once actually waited for (see the SD-36 UI-smoke DOM
// command channel cycle's own measurements). This function now keeps
// polling, within the SAME budget, until the marker is present AND no
// 'Loading' placeholder remains -- so the budget is genuinely spent waiting
// out real async content, not returned unused the moment a static marker
// happens to be there from the start.
function waitForMarker(marker, timeoutMs = 12000) {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    const { snapshot } = latestSnapshot({ requireFresh: false, timeoutMs: 500 });
    if (screenContains(snapshot, marker) && !screenContains(snapshot, 'Loading')) {
      return snapshot;
    }
    if (Date.now() >= deadline) {
      return snapshot;
    }
    sleepMs(300);
  }
}

function assertRow(row, snapshot) {
  const problems = [];
  if (!screenContains(snapshot, row.marker)) {
    problems.push(`marker not found on screen: '${row.marker}'`);
    return problems; // wrong screen entirely — no point checking further.
  }
  for (const needle of row.expect ?? []) {
    if (!screenContains(snapshot, needle)) {
      problems.push(`expected string missing: '${needle}'`);
    }
  }
  // allowGlobalForbid opts a row OUT of specific GLOBAL_FORBID entries whose
  // wording is a false positive on that one screen's own legitimate copy
  // (e.g. the Bug Report tab's own description text uses the word
  // "failure" to explain what the form is FOR, not to report one) — it
  // never touches the row's own `forbid` list, only which global entries
  // apply to this row.
  const allowed = new Set(row.allowGlobalForbid ?? []);
  const forbidList = [...GLOBAL_FORBID.filter((needle) => !allowed.has(needle)), ...(row.forbid ?? [])];
  for (const needle of forbidList) {
    if (screenContains(snapshot, needle)) {
      problems.push(`forbidden string present: '${needle}'`);
    }
  }
  if (screenContains(snapshot, 'Loading')) {
    problems.push(`'Loading' still present after the row's wait budget`);
  }
  for (const selectName of row.selectsNonEmpty ?? []) {
    const select = findSelect(snapshot, selectName);
    if (!select) {
      problems.push(`select not found: '${selectName}'`);
    } else if (select.optionCount <= 0) {
      problems.push(`select '${selectName}' has zero options`);
    }
  }
  return problems;
}

// ----------------------------------------------------------------- run row
function slugFor(row) {
  return row.id;
}

function runRow(row, spec, outDir) {
  const slug = slugFor(row);
  const shotPath = join(outDir, `${slug}.png`);

  if (row.manual) {
    return { id: row.id, status: 'manual', screenshot: null, rendered_excerpt: null, reason: row.manual };
  }

  const landed = resetToLanding();
  if (!landed) {
    return {
      id: row.id,
      status: 'blocked',
      screenshot: null,
      rendered_excerpt: null,
      reason: 'could not reach the landing screen within 6 reset attempts',
    };
  }

  try {
    for (const setupId of row.setup ?? []) {
      const setupRow = spec.byId.get(setupId);
      if (!setupRow) {
        return {
          id: row.id,
          status: 'blocked',
          screenshot: null,
          rendered_excerpt: null,
          reason: `setup row not found: '${setupId}'`,
        };
      }
      runSteps(setupRow.steps ?? []);
    }
    runSteps(row.steps ?? []);
  } catch (cause) {
    if (cause instanceof TargetNotFoundError) {
      return { id: row.id, status: 'red', screenshot: null, rendered_excerpt: null, reason: cause.message };
    }
    throw cause;
  }

  const snapshot = waitForMarker(row.marker);
  const problems = assertRow(row, snapshot);
  const renderedExcerpt = (snapshot?.bodyText ?? '').slice(0, 400);

  driver.screenshot(shotPath);

  if (problems.length > 0) {
    const failedPath = join(outDir, `${slug}.FAILED.png`);
    if (existsSync(shotPath)) {
      renameSync(shotPath, failedPath);
    }
    return { id: row.id, status: 'red', screenshot: failedPath, rendered_excerpt: renderedExcerpt, reason: problems.join('; ') };
  }

  return { id: row.id, status: 'green', screenshot: shotPath, rendered_excerpt: renderedExcerpt, reason: null };
}

// --------------------------------------------------------------------- main
function main() {
  const args = parseArgs(process.argv.slice(2));

  const agent = process.env.RUN_DESKTOP_AGENT;
  if (!agent || agent === 'default') {
    console.error("run.mjs: RUN_DESKTOP_AGENT must be exported to a per-run unique value (not unset, not 'default').");
    console.error('          Two agents sharing "default" share a display and kill each other\'s apps (see SKILL.md).');
    process.exitCode = 2;
    return;
  }

  probePath = `/tmp/run-desktop-driver-${agent}.ui-probe.json`;
  process.env.CODEX_UI_PROBE_FILE = probePath;
  cmdPath = `/tmp/run-desktop-driver-${agent}.ui-probe-cmd.json`;
  process.env.CODEX_UI_PROBE_CMD_FILE = cmdPath;
  USE_XDOTOOL = args.xdotool;

  const outDir = resolve(args.out ?? join(driver.APP_ROOT, 'scripts', 'ui-smoke', '.out', agent));
  mkdirSync(outDir, { recursive: true });

  const spec = loadSpec();
  const rows = selectRows(spec, args);

  const alreadyAlive = driver.isAlive();
  if (!alreadyAlive) {
    // A probe file from a PREVIOUS session (this agent's earlier launch, a
    // manual test edit, anything) can still be sitting at `probePath` —
    // driver.sh's `stop` kills processes, it does not clean up this file.
    // Capture its `ts` (0 if none) as a floor so the post-launch wait below
    // requires a snapshot strictly newer than whatever was already there,
    // rather than accepting stale leftover content as proof the NEW process
    // is ready. Without this, `sinceTs=0` is satisfied instantly by the old
    // file, rows start clicking real (but stale) coordinates against a
    // webview that has not painted its first real frame yet, and the
    // failure mode is a broad, confusing cascade of "could not reach the
    // landing screen" / "target not found" across nearly every row — not a
    // clean, attributable error. Observed directly: this exact cascade,
    // traced back to a leftover probe file from the prior negative-proof
    // run this same cycle.
    const staleTs = readProbeFile(probePath)?.ts ?? 0;

    console.log(`Launching app for agent '${agent}'...`);
    const launchResult = driver.launch();
    if (launchResult.status !== 0) {
      console.error('run.mjs: driver.sh launch failed:');
      console.error(launchResult.stderr || launchResult.stdout);
      process.exitCode = 1;
      return;
    }
    // Wait for the very first FRESH probe report (strictly newer than any
    // stale leftover — see above) — proof the DEV-only installUiProbe()
    // hook is wired up and the React app has painted at least once, not
    // just that the OS window exists. driver.sh's own window-wait only
    // proves the OS-level window was mapped; on a cold run that lands well
    // before Vite has finished cold-transforming this app's
    // (corpus-data-heavy) module graph for the browser, so the first real
    // paint can trail the window by a good while longer than a warm run's
    // near-instant one. Measured once on this box: ~61s after the window
    // appeared, on the very first launch of a session. Default here is
    // deliberately generous; override with RUN_DESKTOP_FIRST_PROBE_TIMEOUT_MS
    // if a slower box needs more.
    const firstProbeTimeoutMs = Number(process.env.RUN_DESKTOP_FIRST_PROBE_TIMEOUT_MS) || 180000;
    const { snapshot } = pollForFreshProbe(probePath, staleTs, firstProbeTimeoutMs, 250);
    if (!snapshot) {
      console.error(
        `run.mjs: app launched but no UI probe report arrived within ${firstProbeTimeoutMs}ms at ${probePath}. ` +
          `Either main.tsx's installUiProbe() call, ui_probe.rs's registration in generate_handler!, ` +
          `or CODEX_UI_PROBE_FILE propagation into the tauri dev child process is broken.`,
      );
      driver.diagnose();
      if (!args.keep) driver.stop();
      process.exitCode = 1;
      return;
    }
    lastSeenTs = snapshot.ts ?? 0;
    // A probe report proves React has painted, but not that WebKitGTK's
    // input pipeline is accepting synthetic (xdotool) events yet — observed
    // directly: the very first click issued immediately after this point
    // (e.g. the first row's own click step) can silently miss on a cold
    // launch even though the exact same click succeeds a moment later once
    // the app is "warm" (findTarget/coords were correct both times; only
    // the click's effect was missing). A short settle delay here costs
    // nothing on every row after the first but avoids a first-row false RED
    // on a fresh launch.
    sleepMs(1500);
  } else {
    console.log(`Reusing live app for agent '${agent}'.`);
    const existing = readProbeFile(probePath);
    lastSeenTs = existing?.ts ?? 0;
  }

  const results = [];
  let green = 0;
  let red = 0;
  let blocked = 0;
  let manual = 0;

  for (const row of rows) {
    let result;
    try {
      result = runRow(row, spec, outDir);
    } catch (cause) {
      result = {
        id: row.id,
        status: 'red',
        screenshot: null,
        rendered_excerpt: null,
        reason: `runner exception: ${cause instanceof Error ? cause.message : String(cause)}`,
      };
    }
    results.push(result);
    if (result.status === 'green') green += 1;
    else if (result.status === 'red') red += 1;
    else if (result.status === 'blocked') blocked += 1;
    else if (result.status === 'manual') manual += 1;

    const marker = { green: 'PASS', red: 'FAIL', blocked: 'BLOCKED', manual: 'MANUAL' }[result.status];
    console.log(`${marker}  ${result.id}${result.reason ? `  -- ${result.reason}` : ''}`);

    writeFileSync(join(outDir, 'results.json'), JSON.stringify(results, null, 2));
  }

  // Fallback clipboard extraction is only reachable when the probe file
  // never appeared at all for the whole run (checked once, not per-row,
  // since a per-row probe failure already surfaces as a specific assertion
  // failure above, which is more actionable than a raw clipboard dump).
  if (lastSeenTs === 0) {
    console.warn('run.mjs: the probe file never populated during this run — falling back to clipboard extraction for one diagnostic read.');
    const text = extractScreenText();
    if (text) {
      console.warn('--- fallback clipboard extraction (first 400 chars) ---');
      console.warn(text.slice(0, 400));
    }
  }

  console.log('');
  console.log(`${green}/${rows.length} green, ${red} red, ${blocked} blocked, ${manual} manual.`);
  console.log(`Evidence: ${outDir}`);

  if (!args.keep) {
    driver.stop();
  }

  process.exitCode = red > 0 || blocked > 0 ? 1 : 0;
}

main();
