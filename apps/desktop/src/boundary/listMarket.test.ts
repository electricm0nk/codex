import { listMarket } from './listMarket';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same no-runtime failure path
 * every other boundary loader's test exercises (see `listEquipment.test.ts`). The list itself is
 * proved in `equipment_catalog.rs`.
 */
async function testNoRuntimeThrowsDescriptiveError() {
  let thrown: unknown;
  try {
    await listMarket({ category: 'armor' });
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(message, 'Tauri runtime not available for listing the market', 'no-runtime failure is descriptive');
}

async function main() {
  await testNoRuntimeThrowsDescriptiveError();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
