/**
 * Theme mode (dark / light) persistence and application.
 *
 * The Wasp palette in `theme.css` is dark by default (`:root`) with a
 * `:root[data-theme='light']` override. We drive that override by stamping
 * `data-theme` on the document element, and remember the choice in
 * localStorage so it survives reloads.
 */

export type ThemeMode = 'dark' | 'light';

const STORAGE_KEY = 'codex.theme.mode';

export function getStoredThemeMode(): ThemeMode {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    return stored === 'light' ? 'light' : 'dark';
  } catch {
    return 'dark';
  }
}

export function applyThemeMode(mode: ThemeMode): void {
  if (typeof document !== 'undefined') {
    document.documentElement.dataset.theme = mode;
  }
  try {
    localStorage.setItem(STORAGE_KEY, mode);
  } catch {
    /* non-persistent environments (e.g. private mode) still get the applied theme */
  }
}
