import { recordAndPrepareSpellSelection } from './recordAndPrepareSpellSelection';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same
 * no-runtime failure path every other boundary loader's test exercises
 * (see `addSpellSelection.test.ts`). risks-and-open-questions.md item 25.
 */
async function testNoRuntimeThrowsDescriptiveError() {
  let thrown: unknown;
  try {
    await recordAndPrepareSpellSelection({
      characterId: 'char-test',
      spellId: 'Light',
      sourceClassId: 'class:wizard',
      savedAt: '2026-07-24T00:00:00Z',
    });
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(
    message,
    'Tauri runtime not available for recording and preparing a spell selection',
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
