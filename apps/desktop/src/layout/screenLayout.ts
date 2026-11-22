import type { CSSProperties } from 'react';

/** Ordinary screens (landing, load, sheet, campaigns) keep the width they were designed for. */
export const READING_FRAME_MAX_WIDTH = 1100;

/**
 * The app shell spans the whole window. It used to cap every page at 1100px, which left most of a
 * maximized window empty on the Create screen; each screen now chooses its own frame instead.
 */
export const APP_SHELL_STYLE: CSSProperties = {
  fontFamily: 'Inter, system-ui, sans-serif',
  margin: 0,
  maxWidth: 'none',
  padding: '3rem 2rem',
};

/** Frame around a hub screen: `wide` uses the whole window, otherwise the centered reading width. */
export function hubFrameStyle(wide: boolean): CSSProperties {
  return wide
    ? { maxWidth: 'none', width: '100%' }
    : { margin: '0 auto', maxWidth: READING_FRAME_MAX_WIDTH, width: '100%' };
}

/** Settings (gear) dialog: sized from the viewport, with a larger ceiling than the old 900x620. */
export const SETTINGS_DIALOG_SIZE = {
  width: 'min(1500px, 96vw)',
  height: 'min(960px, 92vh)',
  minWidth: 480,
  minHeight: 360,
} as const;
