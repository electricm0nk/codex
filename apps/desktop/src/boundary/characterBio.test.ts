import { BLANK_CHARACTER_BIO, loadCharacterBio, updateCharacterBio } from './characterBio';
import { assertEqual } from '../testSupport/asserts';

/**
 * risks-and-open-questions.md item 25: zero test files referenced
 * `characterBio.ts` before this. `loadCharacterBio`'s no-runtime behavior is
 * the one thing that differs from every other boundary loader — it resolves
 * to the blank default (matching the Rust command's own "no bio.json yet"
 * default) rather than throwing.
 */
async function testLoadWithNoRuntimeResolvesToBlankDefault() {
  const bio = await loadCharacterBio('char-test');
  // `loadCharacterBio` returns a fresh spread copy, not the shared constant
  // itself, so this compares structure (JSON.stringify) rather than
  // reference — `assertEqual`'s `!==` would otherwise always fail here.
  assertEqual(
    JSON.stringify(bio),
    JSON.stringify(BLANK_CHARACTER_BIO),
    'loadCharacterBio outside a Tauri runtime resolves to the blank default, not an error'
  );
}

/** `updateCharacterBio` has no such default — writing with nowhere to write throws, like every other boundary mutator. */
async function testUpdateWithNoRuntimeThrowsDescriptiveError() {
  let thrown: unknown;
  try {
    await updateCharacterBio('char-test', { ...BLANK_CHARACTER_BIO, alignment: 'Lawful Good' });
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(
    message,
    'Tauri runtime not available for saving a character bio',
    'no-runtime failure is descriptive'
  );
}

async function main() {
  await testLoadWithNoRuntimeResolvesToBlankDefault();
  await testUpdateWithNoRuntimeThrowsDescriptiveError();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
