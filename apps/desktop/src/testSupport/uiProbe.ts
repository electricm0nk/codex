/**
 * Dev-only UI probe for the headless smoke harness
 * (`apps/desktop/scripts/ui-smoke/`).
 *
 * `installUiProbe()` watches `document.body` (see the note in
 * `installUiProbe` on why not just `#root`) with a debounced
 * `MutationObserver` (plus an initial fire) and reports a JSON snapshot of
 * "what is actually on screen" — headings, body text, and every interactive element's name and
 * window-relative rect — to the Rust side via the `record_ui_probe` command.
 * The runner then reads that file back and clicks by name instead of by
 * hard-coded pixel coordinates, so UI layout drift fails loudly (element not
 * found) instead of silently clicking the wrong thing.
 *
 * Only ever wired up from `main.tsx` when `import.meta.env.DEV` is true, and
 * only does anything when a real Tauri runtime is present — this file must
 * be a safe no-op in a production build and in any non-Tauri (e.g. plain
 * browser/test) context.
 */
import { invoke } from '@tauri-apps/api/core';
import { hasTauriRuntime } from '../boundary/runtime';

export interface UiProbeRect {
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface UiProbeTarget {
  name: string;
  tag: string;
  rect: UiProbeRect;
  disabled: boolean;
}

export interface UiProbeSelect {
  name: string;
  optionCount: number;
}

/**
 * One queued instruction from the runner's DOM command channel
 * (`scripts/ui-smoke/lib/commandChannel.mjs` writes these,
 * `poll_ui_probe_command` in `ui_probe.rs` hands them over). `op` picks
 * which of `target`/`text`/`key`/`index` apply — see `executeCommand`.
 */
export interface UiProbeCommand {
  id: string;
  op: 'click' | 'type' | 'select' | 'key' | 'scroll';
  target?: string;
  text?: string;
  key?: string;
  index?: number;
}

/** What happened the last time a queued command was executed, echoed back on the next probe report. */
export interface UiProbeLastCommand {
  id: string;
  ok: boolean;
  error?: string;
  matchedName?: string;
}

export interface UiProbeSnapshot {
  ts: number;
  mode?: string;
  headings: string[];
  bodyText: string;
  targets: UiProbeTarget[];
  selects: UiProbeSelect[];
  dialogs: string[];
  lastCommand?: UiProbeLastCommand;
}

const INTERACTIVE_SELECTOR = [
  'button',
  'a',
  'input',
  'select',
  'textarea',
  '[role="tab"]',
  '[role="menuitem"]',
].join(', ');

/**
 * An element's "name" for the probe: the same trimmed textContent ||
 * aria-label || placeholder || title chain the harness spec calls for, with
 * one addition — an `id` fallback for form controls (character-name,
 * player-name, …) that carry no visible/accessible label of their own. Without
 * it those fields would report an empty name and be untargetable by name at
 * all, which defeats the "DOM truth instead of pixel coordinates" point of
 * this probe.
 */
function nameOf(el: Element): string {
  const text = (el.textContent ?? '').trim();
  if (text) return text;
  const ariaLabel = el.getAttribute('aria-label');
  if (ariaLabel && ariaLabel.trim()) return ariaLabel.trim();
  const placeholder = el.getAttribute('placeholder');
  if (placeholder && placeholder.trim()) return placeholder.trim();
  const title = el.getAttribute('title');
  if (title && title.trim()) return title.trim();
  const id = el.getAttribute('id');
  if (id && id.trim()) return id.trim();
  return '';
}

/**
 * Picks which candidate name the DOM command channel's `target` string
 * refers to, in three tiers -- exact match first, then prefix, then plain
 * substring -- falling through to the next tier only when the current one
 * has no candidates at all. `index` (default 0) selects the nth candidate
 * *within whichever tier matched*, so a screen with two same-named controls
 * (e.g. a disabled/enabled pair across a re-render boundary, or a repeated
 * row action) is still addressable by position without the caller needing
 * to know which tier its name will land in.
 *
 * Pure and DOM-free on purpose: this is the piece a regression here would
 * make every `click`/`type` step in the harness target silently wrong
 * elements, so it is unit-tested directly (`uiSmokeCommandChannel.test.ts`)
 * without needing a real DOM or a running app.
 *
 * Returns -1 when nothing in `names` matches at any tier.
 */
export function matchCommandTarget(names: string[], target: string, index = 0): number {
  const exact: number[] = [];
  const prefix: number[] = [];
  const contains: number[] = [];
  names.forEach((name, i) => {
    if (name === target) {
      exact.push(i);
    } else if (name.startsWith(target)) {
      prefix.push(i);
    } else if (name.includes(target)) {
      contains.push(i);
    }
  });
  for (const tier of [exact, prefix, contains]) {
    if (tier.length > 0) {
      return tier[index] ?? tier[0]!;
    }
  }
  return -1;
}

function isDisabled(el: Element): boolean {
  if (el instanceof HTMLButtonElement || el instanceof HTMLInputElement ||
      el instanceof HTMLSelectElement || el instanceof HTMLTextAreaElement) {
    return el.disabled;
  }
  return el.getAttribute('aria-disabled') === 'true';
}

function collectTargets(root: Element): UiProbeTarget[] {
  const targets: UiProbeTarget[] = [];
  root.querySelectorAll(INTERACTIVE_SELECTOR).forEach((el) => {
    const rect = el.getBoundingClientRect();
    targets.push({
      name: nameOf(el),
      tag: el.tagName.toLowerCase(),
      rect: { x: Math.round(rect.x), y: Math.round(rect.y), w: Math.round(rect.width), h: Math.round(rect.height) },
      disabled: isDisabled(el),
    });
  });
  return targets;
}

function collectSelects(root: Element): UiProbeSelect[] {
  const selects: UiProbeSelect[] = [];
  let index = 0;
  root.querySelectorAll('select').forEach((el) => {
    index += 1;
    const ariaLabel = el.getAttribute('aria-label');
    // A handful of `<select>`s in this app (e.g. the Appearance panel's
    // base-color-scheme and theme pickers) carry no aria-label or id at
    // all. `select#<n>` (1-based document order among selects currently on
    // screen) is a last-resort but STABLE name for those — stable because
    // each such screen only ever renders one fixed, ordered set of
    // anonymous selects, not because it would survive a reorder.
    const name = (ariaLabel && ariaLabel.trim()) || el.getAttribute('id') || `select#${index}`;
    selects.push({ name, optionCount: el.querySelectorAll('option').length });
  });
  return selects;
}

function collectDialogs(root: Element): string[] {
  const dialogs: string[] = [];
  root.querySelectorAll('[role="dialog"]').forEach((el) => {
    const ariaLabel = el.getAttribute('aria-label');
    if (ariaLabel && ariaLabel.trim()) {
      dialogs.push(ariaLabel.trim());
      return;
    }
    const heading = el.querySelector('h1, h2, h3, h4, h5, h6');
    dialogs.push((heading?.textContent ?? '').trim());
  });
  return dialogs;
}

function collectHeadings(root: Element): string[] {
  const headings: string[] = [];
  root.querySelectorAll('h1, h2, h3').forEach((el) => {
    const text = (el.textContent ?? '').trim();
    if (text) headings.push(text);
  });
  return headings;
}

/**
 * Best-effort screen label derived from the visible headings — every
 * catalog/panel screen renders its own name as an `<h2>` (see
 * `CharacterHubPage.tsx`'s `Mode` union and each screen's own header), so
 * this is a plain first-match lookup, not a parser. Returns undefined when
 * nothing recognizable is on screen (e.g. mid-transition).
 */
function deriveMode(headings: string[]): string | undefined {
  return headings.length > 0 ? headings[0] : undefined;
}

function buildSnapshot(root: Element): UiProbeSnapshot {
  const headings = collectHeadings(root);
  return {
    ts: Date.now(),
    mode: deriveMode(headings),
    headings,
    bodyText: (root as HTMLElement).innerText ?? '',
    targets: collectTargets(root),
    selects: collectSelects(root),
    dialogs: collectDialogs(root),
  };
}

/** Every interactive element currently in the DOM, in document order -- the same set/order `collectTargets` reports. */
function collectInteractiveElements(): Element[] {
  return Array.from(document.body.querySelectorAll(INTERACTIVE_SELECTOR));
}

/** Resolves a command's `target`/`index` against the live DOM via `matchCommandTarget`. `null` when nothing matches. */
function findElementByName(target: string, index: number): { el: Element; matchedName: string } | null {
  const elements = collectInteractiveElements();
  const names = elements.map(nameOf);
  const matchIndex = matchCommandTarget(names, target, index);
  if (matchIndex === -1) {
    return null;
  }
  return { el: elements[matchIndex]!, matchedName: names[matchIndex]! };
}

/**
 * Sets a form control's value through its native property setter, then
 * dispatches `input`/`change` — the two-step sequence React 18's own
 * controlled-input tracking needs to notice the change at all. Setting
 * `.value` directly (or through the React-patched setter the DOM exposes
 * once React has mounted) leaves React's internal "last known value" out of
 * sync, so the very next real keystroke-shaped event is the one that
 * appears to fire, not this one; going through the ORIGINAL, unpatched
 * prototype setter avoids that entirely.
 */
function setNativeValue(el: HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement, value: string): void {
  const proto =
    el instanceof HTMLTextAreaElement
      ? HTMLTextAreaElement.prototype
      : el instanceof HTMLSelectElement
        ? HTMLSelectElement.prototype
        : HTMLInputElement.prototype;
  const setter = Object.getOwnPropertyDescriptor(proto, 'value')?.set;
  if (setter) {
    setter.call(el, value);
  } else {
    // No native setter found (should not happen for these three element
    // types) -- fall back to a plain assignment rather than throwing, so
    // the command still has a chance of working.
    (el as HTMLInputElement).value = value;
  }
  el.dispatchEvent(new Event('input', { bubbles: true }));
  el.dispatchEvent(new Event('change', { bubbles: true }));
}

/** Dispatches a real `keydown` then `keyup` `KeyboardEvent` for `keyName` on `document.activeElement` (falling back to `document`). */
function dispatchKey(keyName: string): void {
  const target: EventTarget = document.activeElement ?? document;
  const init: KeyboardEventInit = { bubbles: true, cancelable: true, key: keyName };
  target.dispatchEvent(new KeyboardEvent('keydown', init));
  target.dispatchEvent(new KeyboardEvent('keyup', init));
}

/**
 * Executes one queued `UiProbeCommand` against the real DOM and reports what
 * happened. Every op dispatches a genuine DOM event/method call (`el.click()`,
 * a native-setter-driven `input`/`change`, a real `KeyboardEvent`) -- never a
 * synthetic OS-level input event -- which is the whole point of this channel:
 * deterministic where `xdotool` under Xvfb+WebKitGTK is not (see this
 * module's own header comment).
 */
function executeCommand(cmd: UiProbeCommand): UiProbeLastCommand {
  try {
    switch (cmd.op) {
      case 'click': {
        const found = findElementByName(cmd.target ?? '', cmd.index ?? 0);
        if (!found) {
          return { id: cmd.id, ok: false, error: `no element named '${cmd.target ?? ''}'` };
        }
        const { el, matchedName } = found;
        (el as HTMLElement).scrollIntoView({ block: 'center' });
        // A real user click focuses a text-entry control as part of the
        // mousedown that precedes it; `el.click()` alone dispatches only the
        // click event and does not reliably carry that side effect in this
        // webview (observed directly: a `type` command right after a `click`
        // on a search box's `<input>` found no focused element at all).
        // Focusing explicitly first makes `click` -> `type` behave like the
        // two-step user gesture it stands in for, for every field the runner
        // clicks before typing into.
        if (el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement || el instanceof HTMLSelectElement) {
          el.focus();
        }
        (el as HTMLElement).click();
        return { id: cmd.id, ok: true, matchedName };
      }
      case 'type': {
        // No `target`: types into whatever currently has focus, matching
        // `driver.sh type`'s own contract -- the runner always issues a
        // `click` step on the field first, exactly as it did for xdotool.
        const active = document.activeElement;
        if (
          !(active instanceof HTMLInputElement) &&
          !(active instanceof HTMLTextAreaElement) &&
          !(active instanceof HTMLSelectElement)
        ) {
          return { id: cmd.id, ok: false, error: 'no focused input/textarea/select to type into' };
        }
        setNativeValue(active, cmd.text ?? '');
        return { id: cmd.id, ok: true, matchedName: nameOf(active) };
      }
      case 'select': {
        const found = findElementByName(cmd.target ?? '', cmd.index ?? 0);
        if (!found || !(found.el instanceof HTMLSelectElement)) {
          return { id: cmd.id, ok: false, error: `no <select> named '${cmd.target ?? ''}'` };
        }
        const select = found.el;
        const wanted = cmd.text ?? '';
        const option = Array.from(select.options).find(
          (candidate) => candidate.text === wanted || candidate.value === wanted,
        );
        if (!option) {
          return { id: cmd.id, ok: false, error: `no option '${wanted}' in '${cmd.target ?? ''}'` };
        }
        setNativeValue(select, option.value);
        return { id: cmd.id, ok: true, matchedName: found.matchedName };
      }
      case 'key': {
        dispatchKey(cmd.key ?? '');
        return { id: cmd.id, ok: true };
      }
      case 'scroll': {
        window.scrollBy(0, cmd.index ?? 400);
        return { id: cmd.id, ok: true };
      }
      default:
        return { id: cmd.id, ok: false, error: `unknown op: ${String((cmd as { op: unknown }).op)}` };
    }
  } catch (cause: unknown) {
    return { id: cmd.id, ok: false, error: cause instanceof Error ? cause.message : String(cause) };
  }
}

/**
 * Starts the DOM command channel's polling loop: ask the Rust side
 * (`poll_ui_probe_command`) for one queued command, execute it if present,
 * then wait 100ms and poll again. Runs for the page's whole lifetime (this
 * hook is only ever installed once, from `main.tsx`, and never torn down) --
 * there is no cleanup path to wire it into.
 *
 * **Deliberately a self-rescheduling `setTimeout` chain, not `setInterval`.**
 * `setInterval(fn, 100)` fires `fn` every 100ms *regardless of whether the
 * previous call finished* -- it does not wait on the `invoke()` promise
 * inside. This webview's own documented IPC/paint latency varies wildly
 * (well under a second up to 60+ seconds for the same call in different
 * sessions -- see `run.mjs`'s own comments), so an interval-based poll can
 * queue a new `invoke('poll_ui_probe_command')` every 100ms while an earlier
 * one is still pending. Under real load this was observed to snowball into
 * an unbounded backlog of in-flight IPC calls that starved the whole
 * channel for tens of seconds at a stretch (`sendCommand` timeouts on
 * ordinary `key`/`click` commands, ten-plus rows in a row, self-resolving
 * only once nothing more was being sent) -- a self-inflicted flood, not a
 * genuine app hang. Scheduling the next poll only after the current one
 * settles caps in-flight polls at exactly one, so a slow round trip merely
 * delays the next poll instead of compounding with it.
 */
function startCommandChannel(onExecuted: (result: UiProbeLastCommand) => void): void {
  const pollOnce = () => {
    invoke<string | null>('poll_ui_probe_command')
      .then((raw) => {
        if (!raw) {
          return;
        }
        let cmd: UiProbeCommand;
        try {
          cmd = JSON.parse(raw) as UiProbeCommand;
        } catch {
          return;
        }
        onExecuted(executeCommand(cmd));
      })
      .catch(() => {
        // Same posture as record_ui_probe's own failure handling: never let
        // a probe-channel hiccup disturb the app it is observing.
      })
      .finally(() => {
        setTimeout(pollOnce, 100);
      });
  };
  pollOnce();
}

/**
 * Installs the probe. Safe to call unconditionally from `main.tsx` — it
 * checks `hasTauriRuntime()` itself and does nothing outside a real Tauri
 * window (e.g. `vite` alone, or a unit test importing this module).
 */
export function installUiProbe(): void {
  if (!hasTauriRuntime()) {
    return;
  }
  const root = document.getElementById('root');
  if (!root) {
    return;
  }

  // Observe `document.body`, not `#root` itself: several of this app's own
  // dialogs (ItemPickerModal, LevelUpDialog, SkillAllocationDialog,
  // ThemeBrowserModal — all `createPortal(..., document.body)`) mount as
  // DOM siblings of `#root`, not descendants of it. A `#root`-scoped
  // observer and snapshot would silently never see the Add Weapon/Armor/
  // Gear/Spell/Feat picker, the level-up dialog, the skill-allocation
  // dialog, or the theme browser — exactly the affordances this harness
  // exists to smoke-test. `document.body` contains `#root` as a subtree, so
  // nothing that would have been captured under the narrower scope is lost.
  const observedRoot: Element = document.body;

  let debounceHandle: ReturnType<typeof setTimeout> | null = null;
  // The DOM command channel's own last-executed-command result, echoed on
  // every subsequent report until the next command overwrites it -- the
  // runner correlates this against the command `id` it just sent.
  let lastCommand: UiProbeLastCommand | undefined;
  const report = () => {
    const snapshot = buildSnapshot(observedRoot);
    if (lastCommand) {
      snapshot.lastCommand = lastCommand;
    }
    invoke('record_ui_probe', { payload: JSON.stringify(snapshot) }).catch(() => {
      // The probe must never crash the app it is observing. A write
      // failure (e.g. CODEX_UI_PROBE_FILE unset outside a smoke run) is
      // silently ignored — the Rust side no-ops in that case too.
    });
  };

  const scheduleReport = () => {
    if (debounceHandle !== null) {
      clearTimeout(debounceHandle);
    }
    debounceHandle = setTimeout(report, 150);
  };

  const observer = new MutationObserver(scheduleReport);
  observer.observe(observedRoot, { attributes: true, characterData: true, childList: true, subtree: true });

  // Initial fire so the very first screen (before any mutation happens) is
  // still captured.
  report();

  // DOM command channel: executing a command (e.g. `key Escape` against a
  // screen with nothing open) does not always produce a DOM mutation, so it
  // cannot rely on `scheduleReport`'s debounce to ever fire -- report
  // immediately, every time, so the runner's wait for `lastCommand.id`
  // reliably resolves instead of occasionally riding out to its own timeout.
  startCommandChannel((result) => {
    lastCommand = result;
    report();
  });
}

// Exported for the harness's own unit tests (target-name derivation and the
// command channel's name-matching tiers must not silently regress) — not
// part of the runtime's own call surface.
export const __testables = { nameOf, deriveMode, matchCommandTarget };
