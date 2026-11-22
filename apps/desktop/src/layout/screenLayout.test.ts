import { assert, assertEqual } from '../testSupport/asserts';
import { APP_SHELL_STYLE, READING_FRAME_MAX_WIDTH, SETTINGS_DIALOG_SIZE, hubFrameStyle } from './screenLayout';

// The shell used to cap every page at 1100px, which is why Create a character left most of a
// maximized window empty. The shell now spans the window; each screen picks its own frame.

function verifiesShellDoesNotCapTheWindow() {
  assertEqual(APP_SHELL_STYLE.maxWidth, 'none', 'the app shell must not cap page width');
  assert(String(APP_SHELL_STYLE.padding ?? '').length > 0, 'the shell keeps its own padding');
}

function verifiesReadingFrameKeepsTheOldWidthForOrdinaryScreens() {
  const frame = hubFrameStyle(false);
  assertEqual(frame.maxWidth, READING_FRAME_MAX_WIDTH, 'ordinary screens keep the reading width');
  assertEqual(READING_FRAME_MAX_WIDTH, 1100, 'reading width is unchanged from before');
  assertEqual(frame.margin, '0 auto', 'ordinary screens stay centered');
}

function verifiesWideFrameUsesTheWholeWindow() {
  const frame = hubFrameStyle(true);
  assertEqual(frame.maxWidth, 'none', 'the wide frame has no width cap');
  assertEqual(frame.width, '100%', 'the wide frame fills the shell');
}

function verifiesSettingsDialogIsSizedFromTheViewport() {
  const pxCap = (value: string) => Number(/min\((\d+)px/.exec(value)?.[1] ?? '0');
  assert(SETTINGS_DIALOG_SIZE.width.endsWith('vw)'), `dialog width must follow the viewport, got ${SETTINGS_DIALOG_SIZE.width}`);
  assert(SETTINGS_DIALOG_SIZE.height.endsWith('vh)'), `dialog height must follow the viewport, got ${SETTINGS_DIALOG_SIZE.height}`);
  assert(pxCap(SETTINGS_DIALOG_SIZE.width) >= 1400, 'dialog may grow to at least 1400px wide (was 900)');
  assert(pxCap(SETTINGS_DIALOG_SIZE.height) >= 900, 'dialog may grow to at least 900px tall (was 620)');
  assert(SETTINGS_DIALOG_SIZE.minWidth <= 480, 'a small window can still shrink it');
}

verifiesShellDoesNotCapTheWindow();
verifiesReadingFrameKeepsTheOldWidthForOrdinaryScreens();
verifiesWideFrameUsesTheWholeWindow();
verifiesSettingsDialogIsSizedFromTheViewport();
console.log('screenLayout.test.ts: all assertions passed');
