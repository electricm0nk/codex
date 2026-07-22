import { loadSupportStateMatrix } from './loadSupportStateMatrix';
import { assertEqual } from '../testSupport/asserts';

/**
 * Pins the descriptive export name (`loadSupportStateMatrix`)
 * that the SD-25 criterion 1.1 identifier-cleanup file rename landed on. No Tauri
 * runtime stub is set up, so this exercises only the no-runtime failure
 * path (the same path every other boundary loader's no-runtime test
 * exercises); the invoke-string itself is untouched (already renamed to
 * `load_support_state_matrix` in the prior Rust-side cycle).
 */
async function testNoRuntimeThrowsDescriptiveError() {
  let thrown: unknown;
  try {
    await loadSupportStateMatrix();
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(
    message,
    'Tauri runtime not available for the support-state matrix',
    'no-runtime failure is descriptive and carries no bundle-tagged identifier'
  );
}

async function main() {
  await testNoRuntimeThrowsDescriptiveError();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
