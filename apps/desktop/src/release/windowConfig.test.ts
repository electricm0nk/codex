import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assert, assertEqual } from '../testSupport/asserts';

// The app opens in a maximized (not fullscreen) window: decorations and the OS window controls stay,
// the window just fills the screen. `width`/`height` remain the size it restores to.

const conf = JSON.parse(
  readFileSync(join(dirname(fileURLToPath(import.meta.url)), '..', '..', 'src-tauri', 'tauri.conf.json'), 'utf8'),
) as { app: { windows: Array<Record<string, unknown>> } };
const main = conf.app.windows[0];

assertEqual(main.maximized, true, 'the main window opens maximized');
assertEqual(main.resizable, true, 'the window stays resizable (restore/minimize keep working)');
assert(main.fullscreen !== true, 'the window is maximized, not fullscreen');
assertEqual(main.label, 'main', 'this is the main window');
console.log('windowConfig.test.ts: all assertions passed');
