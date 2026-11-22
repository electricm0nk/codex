import { assertEqual } from '../testSupport/asserts';
import { loadCharacterHitPoints } from './characterHitPoints';

// No Tauri runtime under tsx: the loader resolves to "none saved" instead of failing, like the bio loader.
async function main() {
  const result = await loadCharacterHitPoints('char-test');
  assertEqual(result.levels.length, 0, 'outside the desktop runtime there are no saved results');
  console.log('characterHitPoints.test.ts: all assertions passed');
}
main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
