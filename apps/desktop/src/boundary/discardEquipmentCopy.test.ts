import { discardEquipmentCopy } from './discardEquipmentCopy';
import { renameEquipmentCopy } from './renameEquipmentCopy';
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
  const copy = { itemId: 'Arrows (Flight/20)', bonus: 0, ordinal: 0 };
  assertEqual(
    await failure(() => discardEquipmentCopy({ characterId: 'c', copy, quantity: 20, savedAt: '2026-11-15T00:00:00Z' })),
    'Tauri runtime not available for discarding equipment',
    'discard: no-runtime failure is descriptive',
  );
  assertEqual(
    await failure(() => renameEquipmentCopy({ characterId: 'c', copy, name: 'Quiver arrows' })),
    'Tauri runtime not available for renaming equipment',
    'rename: no-runtime failure is descriptive',
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
