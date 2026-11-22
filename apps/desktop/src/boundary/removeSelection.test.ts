import {
  removeEquipmentSelection,
  removeFeatSelection,
  removeSpellSelection,
} from './removeSelection';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same
 * no-runtime failure path every other boundary loader's test exercises
 * (see `addFeatSelection.test.ts`, the add-path twin of these three).
 *
 * Each of the three is asserted separately rather than once through a
 * helper: the point of the assertion is that the message names the right
 * operation, and a shared helper would let two of them drift onto the same
 * wording without the test noticing.
 */
async function messageFrom(call: () => Promise<unknown>): Promise<string> {
  let thrown: unknown;
  try {
    await call();
  } catch (cause) {
    thrown = cause;
  }
  return thrown instanceof Error ? thrown.message : String(thrown);
}

async function testRemoveFeatNoRuntime() {
  const message = await messageFrom(() =>
    removeFeatSelection({
      characterId: 'char-test',
      featId: 'feat:toughness',
      target: null,
      savedAt: '2026-08-01T00:00:00Z',
    }),
  );
  assertEqual(
    message,
    'Tauri runtime not available for removing a feat selection',
    'remove-feat no-runtime failure is descriptive',
  );
}

async function testRemoveSpellNoRuntime() {
  const message = await messageFrom(() =>
    removeSpellSelection({
      characterId: 'char-test',
      spellId: 'Magic Missile',
      sourceClassId: 'class:wizard',
      savedAt: '2026-08-01T00:00:00Z',
    }),
  );
  assertEqual(
    message,
    'Tauri runtime not available for removing a spell selection',
    'remove-spell no-runtime failure is descriptive',
  );
}

async function testRemoveEquipmentNoRuntime() {
  const message = await messageFrom(() =>
    removeEquipmentSelection({
      characterId: 'char-test',
      itemId: 'item:longsword',
      savedAt: '2026-08-01T00:00:00Z',
    }),
  );
  assertEqual(
    message,
    'Tauri runtime not available for removing an equipment selection',
    'remove-equipment no-runtime failure is descriptive',
  );
}

async function main() {
  await testRemoveFeatNoRuntime();
  await testRemoveSpellNoRuntime();
  await testRemoveEquipmentNoRuntime();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
