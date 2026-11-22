/**
 * The feat picker's "Hide feats I don't qualify for" setting. Ticked until the
 * player unticks it, and remembered in localStorage like the theme mode.
 */

export interface FeatPickerPreferenceStorage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
}

const HIDE_UNMET_KEY = 'codex.featPicker.hideUnmet';

export function getHideUnmetFeats(storage?: FeatPickerPreferenceStorage): boolean {
  try {
    return (storage ?? localStorage).getItem(HIDE_UNMET_KEY) !== 'false';
  } catch {
    return true;
  }
}

export function setHideUnmetFeats(hide: boolean, storage?: FeatPickerPreferenceStorage): void {
  try {
    (storage ?? localStorage).setItem(HIDE_UNMET_KEY, String(hide));
  } catch {
    /* non-persistent environments (e.g. private mode) still get the choice for this sheet */
  }
}
