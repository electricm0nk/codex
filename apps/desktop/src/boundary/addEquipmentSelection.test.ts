import { addEquipmentSelection } from './addEquipmentSelection';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same
 * no-runtime failure path every other boundary loader's test exercises
 * (see `levelUpCharacter.test.ts`).
 */
async function testNoRuntimeThrowsDescriptiveError() {
  let thrown: unknown;
  try {
    await addEquipmentSelection({
      characterId: 'char-test',
      itemId: 'equipment:longsword',
      activeState: 'EquippedActive',
      savedAt: '2026-07-20T00:00:00Z',
    });
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(
    message,
    'Tauri runtime not available for adding an equipment selection',
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
