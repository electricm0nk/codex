import { FIRST_TIME_SECTIONS, getPrintSections, setPrintSections } from './printSections';
import type { PrintPaperStorage } from './printPaper';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The Print dialog's ticks, remembered per character (SD-40 `decisions.md §23.4`, §24.9): the first
 * time, Primary + Abilities and Feats + Inventory; then whatever the player last ticked for that
 * character, under `codex.print.sections.<characterId>`. A value that does not parse is the default.
 * Storage is injected because the Node test runtime has no localStorage.
 */
function memoryStorage(): PrintPaperStorage & { values: Map<string, string> } {
  const values = new Map<string, string>();
  return {
    values,
    getItem: (key) => values.get(key) ?? null,
    setItem: (key, value) => void values.set(key, value),
  };
}

const show = (sections: object) => JSON.stringify(sections);

assertEqual(
  show(FIRST_TIME_SECTIONS),
  show({ main: true, feats: true, inventory: true, spellbook: false, bio: false, progression: false, math: false, notes: false }),
  'first time: Primary, Abilities and Feats, Inventory; Notes off',
);

const storage = memoryStorage();
assertEqual(show(getPrintSections('char-aldric', storage)), show(FIRST_TIME_SECTIONS), 'never set: the first-time ticks');

const withBio = { ...FIRST_TIME_SECTIONS, bio: true };
setPrintSections('char-aldric', withBio, storage);
assertEqual(show(getPrintSections('char-aldric', storage)), show(withBio), "Aldric's ticks are remembered");
assertEqual(show(getPrintSections('char-elowen', storage)), show(FIRST_TIME_SECTIONS), 'per character: Elowen still has the first-time ticks');
assert(storage.values.has('codex.print.sections.char-aldric'), 'under codex.print.sections.<characterId>');

const spellbookOnly = { ...FIRST_TIME_SECTIONS, main: false, feats: false, inventory: false, spellbook: true };
setPrintSections('char-elowen', spellbookOnly, storage);
assertEqual(show(getPrintSections('char-elowen', storage)), show(spellbookOnly), "Elowen's own ticks");
assertEqual(show(getPrintSections('char-aldric', storage)), show(withBio), "and Aldric's are untouched");

const nothing = { ...FIRST_TIME_SECTIONS, main: false, feats: false, inventory: false };
setPrintSections('char-tomas', nothing, storage);
assertEqual(show(getPrintSections('char-tomas', storage)), show(nothing), 'nothing ticked is remembered as it is (the dialog disables its buttons)');

for (const [tag, bad] of [
  ['not JSON', 'spellbook'],
  ['not an object', '[true,true]'],
  ['null', 'null'],
  ['a section missing', JSON.stringify({ main: true, feats: true, inventory: true, spellbook: false, bio: false, progression: false, math: false })],
  ['a section not a boolean', JSON.stringify({ ...FIRST_TIME_SECTIONS, bio: 'yes' })],
] as const) {
  const garbled = memoryStorage();
  garbled.setItem('codex.print.sections.char-aldric', bad);
  assertEqual(show(getPrintSections('char-aldric', garbled)), show(FIRST_TIME_SECTIONS), `${tag}: reads as the first-time ticks`);
}

const broken: PrintPaperStorage = {
  getItem: () => {
    throw new Error('storage unavailable');
  },
  setItem: () => {
    throw new Error('storage unavailable');
  },
};
assertEqual(show(getPrintSections('char-aldric', broken)), show(FIRST_TIME_SECTIONS), 'a store that throws reads as the first-time ticks');
setPrintSections('char-aldric', withBio, broken);

console.log('printSections.test.ts: all assertions passed');
