import { adjustCharacterHp, loadCharacterDurability } from './characterDurability';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same
 * no-runtime failure path every other boundary loader's test exercises
 * (see `addSpellSelection.test.ts`). Unlike bio/money, `loadCharacterDurability`
 * has no "resolve to a default" case — an unsupported build is a real Rust
 * `Err`, and outside a runtime entirely is the same kind of real failure, not
 * an expected empty state. risks-and-open-questions.md item 25.
 */
async function testLoadThrowsDescriptiveErrorWithNoRuntime() {
  let thrown: unknown;
  try {
    await loadCharacterDurability('char-test');
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(
    message,
    'Tauri runtime not available for loading character durability',
    'no-runtime failure is descriptive'
  );
}

async function testAdjustThrowsDescriptiveErrorWithNoRuntime() {
  let thrown: unknown;
  try {
    await adjustCharacterHp('char-test', -5, 0);
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(
    message,
    'Tauri runtime not available for adjusting character HP',
    'no-runtime failure is descriptive'
  );
}

async function main() {
  await testLoadThrowsDescriptiveErrorWithNoRuntime();
  await testAdjustThrowsDescriptiveErrorWithNoRuntime();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
