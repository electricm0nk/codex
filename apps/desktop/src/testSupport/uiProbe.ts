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

export interface UiProbeSnapshot {
  ts: number;
  mode?: string;
  headings: string[];
  bodyText: string;
  targets: UiProbeTarget[];
  selects: UiProbeSelect[];
  dialogs: string[];
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
  const report = () => {
    const snapshot = buildSnapshot(observedRoot);
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
}

// Exported for the harness's own unit test (target-name derivation must not
// silently regress) — not part of the runtime's own call surface.
export const __testables = { nameOf, deriveMode };
