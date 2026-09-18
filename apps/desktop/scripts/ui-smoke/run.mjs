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
// Any one of these being present is treated as "we are on the landing screen".
const LANDING_TARGET_NAMES = ['New\nCharacter', 'Load\nCharacter'];
const LANDING_MARKER_PREFIX = 'Browse ';

function parseArgs(argv) {
  const args = { only: null, from: null, out: null, keep: false };
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
let lastSeenTs = 0;

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
const CLICK_TARGET_WAIT_MS = 10000;

class TargetNotFoundError extends Error {
  constructor(name) {
    super(`target not found: '${name}'`);
    this.name = 'TargetNotFoundError';
  }
}

/** Clicks the named target, scrolling it into view first if the probe says it's below the fold. */
function clickByName(name) {
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

function runStep(step) {
  switch (step.op) {
    case 'click':
      clickByName(step.target);
      break;
    case 'type':
      driver.type(step.text);
      break;
    case 'key':
      driver.key(step.key);
      break;
    case 'scroll':
      driver.scroll(960, 600, step.ticks ?? 5);
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
    const clickable = RESET_CLICK_NAMES.map((name) => findTarget(snapshot, name)).find(Boolean);
    if (!clickable) {
      // Nothing we recognize to click — try Escape once (closes any open
      // dialog/modal) then re-check before giving up this attempt.
      driver.key('Escape');
      sleepMs(220);
      ({ snapshot } = latestSnapshot());
      continue;
    }
    const { x, y } = centerOf(clickable.rect);
    driver.click(x, y);
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
function waitForMarker(marker, timeoutMs = 12000) {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    const { snapshot } = latestSnapshot({ requireFresh: false, timeoutMs: 500 });
    if (screenContains(snapshot, marker)) {
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
