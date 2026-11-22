import { listHeldEquipment } from './listHeldEquipment';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same no-runtime failure path
 * every other boundary loader's test exercises. The list itself is proved in `character_hub.rs`.
 */
async function main() {
  let thrown: unknown;
  try {
    await listHeldEquipment('someone');
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(message, 'Tauri runtime not available for listing held equipment', 'no-runtime failure is descriptive');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
