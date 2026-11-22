import { addFeatSelection } from './addFeatSelection';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same
 * no-runtime failure path every other boundary loader's test exercises
 * (see `addSpellSelection.test.ts`). risks-and-open-questions.md item 25.
 */
async function testNoRuntimeThrowsDescriptiveError() {
  let thrown: unknown;
  try {
    await addFeatSelection({
      characterId: 'char-test',
      featId: 'feat:toughness',
      savedAt: '2026-07-24T00:00:00Z',
    });
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(message, 'Tauri runtime not available for adding a feat selection', 'no-runtime failure is descriptive');
}

async function main() {
  await testNoRuntimeThrowsDescriptiveError();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
