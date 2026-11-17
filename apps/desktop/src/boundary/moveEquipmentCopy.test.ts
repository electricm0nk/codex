import { moveEquipmentCopy } from './moveEquipmentCopy';
import { activateEquipmentSet, createEquipmentSet, deleteEquipmentSet, renameEquipmentSet } from './equipmentSets';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the no-runtime failure path every
 * boundary's test exercises (see `purchaseEquipment.test.ts`).
 */
async function failure(run: () => Promise<unknown>): Promise<string> {
  try {
    await run();
  } catch (cause) {
    return cause instanceof Error ? cause.message : String(cause);
  }
  return 'resolved';
}

async function main() {
  const copy = { itemId: 'Arrows (20)', bonus: 0, ordinal: 0 };
  const savedAt = '2026-11-14T00:00:00Z';
  assertEqual(
    await failure(() => moveEquipmentCopy({ characterId: 'c', copy, location: { kind: 'notCarried' }, quantity: 20, savedAt })),
    'Tauri runtime not available for moving equipment',
    'move: no-runtime failure is descriptive',
  );
  assertEqual(await failure(() => createEquipmentSet({ characterId: 'c', name: 'Camp', copyActive: true })), 'Tauri runtime not available for creating an equipment set', 'create');
  assertEqual(await failure(() => renameEquipmentSet({ characterId: 'c', from: 'Camp', to: 'Travel' })), 'Tauri runtime not available for renaming an equipment set', 'rename');
  assertEqual(await failure(() => deleteEquipmentSet({ characterId: 'c', name: 'Camp', savedAt })), 'Tauri runtime not available for deleting an equipment set', 'delete');
  assertEqual(await failure(() => activateEquipmentSet({ characterId: 'c', name: 'Camp', savedAt })), 'Tauri runtime not available for activating an equipment set', 'activate');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
