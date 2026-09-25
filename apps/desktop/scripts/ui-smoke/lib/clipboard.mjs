// FALLBACK-only extraction, used exclusively when the probe file never
// appears at all (e.g. a build predating this harness, or the DEV env var
// gate somehow not firing) — never the primary path. Mirrors
// verify-on-screen.sh's own extract_screen_text: blur off any focused input
// (so ctrl+a selects the whole document, not just that input's text), then
// select-all + copy in the webview, then read the X clipboard back via
// read-clipboard.py (no xclip/xsel in this container; python3-gi does).
import { spawnSync } from 'node:child_process';
import * as driver from './driver.mjs';
import { sleepMs } from './probe.mjs';

// A neutral point outside every interactive element, matching
// verify-on-screen.sh's own BLUR_X/BLUR_Y choice.
const BLUR_X = 150;
const BLUR_Y = 600;

export function extractScreenText() {
  driver.click(BLUR_X, BLUR_Y);
  driver.key('ctrl+a');
  driver.key('ctrl+c');
  // Give the webview a moment to populate the X clipboard before reading it.
  sleepMs(1000);

  const displayNum = driver.displayNum();
  const result = spawnSync('python3', [driver.READ_CLIPBOARD_PY], {
    encoding: 'utf8',
    env: { ...process.env, DISPLAY: `:${displayNum}` },
  });
  if (result.status !== 0) {
    return null;
  }
  return result.stdout ?? null;
}
