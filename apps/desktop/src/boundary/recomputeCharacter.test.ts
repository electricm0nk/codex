import { recomputeCharacter } from './recomputeCharacter';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same
 * no-runtime failure path every other boundary loader's test exercises
 * (see `loadDeleteCharacter.test.ts`). The success path is exercised by the
 * Rust-side `recompute_character_via_rule_system_dispatches_pf1_through_the_trait`
 * test in `characterHub/recomputeCharacter.rs` (SD-25 Criterion 3.4) — this
 * file's job is to prove the boundary wrapper is a real `invoke()` call
 * site wired to the `recompute_character` command, not decorative plumbing.
 */
async function testNoRuntimeThrowsDescriptiveError() {
  let thrown: unknown;
  try {
    await recomputeCharacter({ characterId: 'char-test', ruleSystemId: 'pf1' });
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(
    message,
    'Tauri runtime not available for recomputing a character',
    'no-runtime failure is descriptive'
  );
}

async function main() {
  await testNoRuntimeThrowsDescriptiveError();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
