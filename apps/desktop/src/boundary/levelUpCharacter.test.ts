import { levelUpCharacter } from './levelUpCharacter';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same
 * no-runtime failure path every other boundary loader's test exercises
 * (see `loadSupportStateMatrix.test.ts`). It also pins the descriptive,
 * non-tagged error message.
 */
async function testNoRuntimeThrowsDescriptiveError() {
  let thrown: unknown;
  try {
    await levelUpCharacter({ characterId: 'char-test', classId: 'class:fighter', savedAt: '2026-07-20T00:00:00Z' });
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(
    message,
    'Tauri runtime not available for leveling up a character',
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
