import { BLANK_CHARACTER_BIO, loadCharacterBio, parseLanguages, updateCharacterBio } from './characterBio';
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

/** SD-40 E2.27: the Languages field is a comma list; blanks and stray spaces are not languages. */
function testParseLanguagesSplitsACommaList() {
  assertEqual(JSON.stringify(parseLanguages(' Common,  Elven ,,Dwarven ')), JSON.stringify(['Common', 'Elven', 'Dwarven']), 'comma list, trimmed, in order');
  assertEqual(JSON.stringify(parseLanguages('  ')), '[]', 'an empty field stores nothing (the engine then reads Common)');
  assertEqual(JSON.stringify(BLANK_CHARACTER_BIO.languages), '[]', 'a blank bio stores no languages');
}

/** SD-40 E3.6: Biography and Notes are free text carried with the bio; a blank bio holds neither. */
function testBlankBioHasEmptyBiographyAndNotes() {
  assertEqual(BLANK_CHARACTER_BIO.biography, '', 'a blank bio has no Biography');
  assertEqual(BLANK_CHARACTER_BIO.notes, '', 'a blank bio has no Notes');
}

async function main() {
  testParseLanguagesSplitsACommaList();
  testBlankBioHasEmptyBiographyAndNotes();
  await testLoadWithNoRuntimeResolvesToBlankDefault();
  await testUpdateWithNoRuntimeThrowsDescriptiveError();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
