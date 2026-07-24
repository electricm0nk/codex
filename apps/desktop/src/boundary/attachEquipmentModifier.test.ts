import { attachEquipmentModifier } from './attachEquipmentModifier';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same
 * no-runtime failure path every other boundary loader's test exercises
 * (see `purchaseEquipment.test.ts`, which `attachEquipmentModifier`
 * mirrors). items-1-and-27-scoping.md sub-task 6.
 */
async function testNoRuntimeThrowsDescriptiveError() {
  let thrown: unknown;
  try {
    await attachEquipmentModifier({
      characterId: 'char-test',
      itemId: 'item:longsword',
      modifierItemId: 'Special Ability ~ +1 ~ Weapon',
      savedAt: '2026-07-24T00:00:00Z',
    });
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(
    message,
    'Tauri runtime not available for attaching an equipment modifier',
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
