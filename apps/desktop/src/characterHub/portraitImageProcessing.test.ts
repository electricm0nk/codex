import { MAX_PORTRAIT_UPLOAD_BYTES, PortraitUploadError, resizeAndCropPortrait } from './portraitImageProcessing';
import { assert } from '../testSupport/asserts';

/**
 * risks-and-open-questions.md item 25: deliberately partial coverage. Only
 * the two validation early-returns are exercised here — they run before any
 * canvas/Image work, so a plain fake `File` (just `type`/`size`) is enough.
 * The actual crop/resize path needs a real `<canvas>`, which this
 * `tsx`-based test runner doesn't provide (no DOM) — out of scope for this
 * pass, per QA's own scoping (risks-and-open-questions.md item 25).
 */
function fakeFile(overrides: { type?: string; size?: number }): File {
  return { type: overrides.type ?? 'image/png', size: overrides.size ?? 1024 } as unknown as File;
}

async function testRejectsAnUnsupportedImageType() {
  let thrown: unknown;
  try {
    await resizeAndCropPortrait(fakeFile({ type: 'image/bmp' }));
  } catch (cause) {
    thrown = cause;
  }
  assert(thrown instanceof PortraitUploadError, 'an unsupported MIME type throws PortraitUploadError, not a generic Error');
  assert(
    thrown instanceof Error && thrown.message.includes('image/bmp'),
    'the error message names the actual unsupported type'
  );
}

async function testRejectsAnOversizedImage() {
  let thrown: unknown;
  try {
    await resizeAndCropPortrait(fakeFile({ type: 'image/png', size: MAX_PORTRAIT_UPLOAD_BYTES + 1 }));
  } catch (cause) {
    thrown = cause;
  }
  assert(thrown instanceof PortraitUploadError, 'a file over the size limit throws PortraitUploadError, not a generic Error');
  assert(thrown instanceof Error && thrown.message.includes('too large'), 'the error message explains the file is too large');
}

async function main() {
  await testRejectsAnUnsupportedImageType();
  await testRejectsAnOversizedImage();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
