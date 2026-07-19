import { loadSupportStateMatrix } from './loadSd13SupportStateMatrix';
import { assertEqual } from '../testSupport/asserts';

/**
 * Pins the descriptive post-rename export name (`loadSupportStateMatrix`)
 * criterion E1.2 introduces in place of the `Sd13`-tagged name. No Tauri
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
    'no-runtime failure is descriptive and carries no Sd13-tagged identifier'
  );
}

async function main() {
  await testNoRuntimeThrowsDescriptiveError();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
