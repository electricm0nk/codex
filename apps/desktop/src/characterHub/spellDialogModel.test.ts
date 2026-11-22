import { assert, assertEqual } from '../testSupport/asserts';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import {
  creationSpellDialog,
  keepOfferedSpells,
  spellClassTokens,
  spellOptionsKey,
  spellPicksLeft,
  spellQuotaOverruns,
  spellSelectionsFromIds,
  type SpellDialogInput,
} from './spellDialogModel';

const row = (id: string, value: number): ExplanationDto => ({ id, value, detail: id });
const perDay = (cls: string, level: number, value: number, basis: 'total' | 'base' = 'total'): ExplanationDto =>
  row(`class_spell.${cls}.${basis}_spells_per_day.spell_level_${level}`, value);
const sorcererKnown = (level: number, value: number) => row(`class_chassis.sorcerer.spontaneous.spells_known.spell_level_${level}`, value);
const sorcererPerDay = (level: number, value: number) => row(`class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_${level}`, value);
const spellbook = (suffix: string, value: number) => row(`class_spell.wizard.spellbook_size.spell_level_${suffix}`, value);

const spell = (key: string, over: Partial<SpellCatalogEntryDto> = {}): SpellCatalogEntryDto =>
  ({ key, book: 'Crb', school: 'Evocation', level: 1, description: `${key} text.`, ...over }) as SpellCatalogEntryDto;

const classLevels = (classId: string, levels: Record<string, number>): ClassSpellLevelsDto =>
  ({ classId, known: true, spellcasting: 'listIngested', spellType: 'Arcane', entries: Object.entries(levels).map(([key, level]) => ({ key, level })) });

const catalog = [spell('Light'), spell('Daze'), spell('Magic Missile'), spell('Sleep'), spell('Invisibility'), spell('Fireball'), spell('Dancing Lights'), spell('Bless')];
const arcane = { Light: 0, Daze: 0, 'Magic Missile': 1, Sleep: 1, Invisibility: 2, Fireball: 3 };
const sorcerer = classLevels('class:sorcerer', arcane);
const wizard = classLevels('class:wizard', arcane);
const cleric = classLevels('class:cleric', { Light: 0, Bless: 1 });
const extracts = { 'Cure Light Wounds': 1, Shield: 1, 'Cat\'s Grace': 2 };

const input = (over: Partial<SpellDialogInput>): SpellDialogInput => ({
  perDay: [],
  known: [],
  classLevels: [
    sorcerer,
    wizard,
    cleric,
    classLevels('class:hunter', { Light: 0, Bless: 1 }),
    classLevels('class:skald', arcane),
    classLevels('class:alchemist', extracts),
    classLevels('class:investigator', extracts),
    classLevels('class:witch', arcane),
    classLevels('class:arcanist', arcane),
  ],
  catalog,
  innate: [],
  ...over,
});

// Brief 1 F4/F5: a sorcerer's rows are `class_chassis.sorcerer.spontaneous.*`, and its quota is the
// spells it KNOWS (4 cantrips, 2 1st at level 1), not the spells it casts per day.
function verifiesASorcererPicksItsSpellsKnownPerLevel() {
  const dialog = creationSpellDialog(input({ perDay: [sorcererPerDay(1, 4)], known: [sorcererKnown(0, 4), sorcererKnown(1, 2)] }));
  assertEqual(JSON.stringify(dialog.groupLimits), JSON.stringify({ 'sorcerer:0': 4, 'sorcerer:1': 2 }), 'one quota per spell level, from spells known');
  assertEqual(dialog.groupLabels['sorcerer:1'], 'Sorcerer level 1', 'with a readable name');
  assertEqual(dialog.items.filter((item) => item.group === 'sorcerer:0').map((item) => item.label).join(','), 'Light,Daze', 'cantrips under level 0');
  assertEqual(dialog.items.some((item) => item.label === 'Invisibility'), false, 'no 2nd-level spells known, none offered');
  assertEqual(dialog.readOnly.length, 0, 'a sorcerer gets a picker');
  assert(dialog.items.find((item) => item.label === 'Light')?.description?.includes('Light text.') === true, 'the description is carried');
}

function verifiesTheClassTokensComeFromChassisAndKnownRows() {
  const tokens = spellClassTokens([sorcererPerDay(1, 4), perDay('cleric', 1, 2)], [spellbook('0', 19)]);
  assertEqual(tokens.join(','), 'class:cleric,class:sorcerer,class:wizard', 'a sorcerer chassis id and a wizard spellbook row both name their class');
}

// A level-5 wizard with Int +3: every cantrip, 14 spells of 1st level or higher, at most 6 of them
// 2nd or higher and 2 of them 3rd (the spells gained at levels 3-5 and at level 5).
function verifiesAWizardPicksItsSpellbookWithSharedPools() {
  const dialog = creationSpellDialog(
    input({ known: [spellbook('0', 2), spellbook('1_and_higher', 14), spellbook('2_and_higher', 6), spellbook('3_and_higher', 2)] })
  );
  assertEqual(
    JSON.stringify(dialog.groupLimits),
    JSON.stringify({ 'wizard:0': 2, 'wizard:1': 14, 'wizard:2': 6, 'wizard:3': 2 }),
    'levels 0-3, each capped by its own bound'
  );
  assertEqual(
    JSON.stringify(dialog.sharedLimits.map((shared) => [shared.groups.join('+'), shared.limit])),
    JSON.stringify([['wizard:1+wizard:2+wizard:3', 14], ['wizard:2+wizard:3', 6]]),
    'a pool per bound that spans more than one level'
  );
  assertEqual(dialog.groupLabels['wizard:0'], 'Wizard level 0 (all cantrips)', 'the cantrip quota says it is every cantrip');
  assertEqual(JSON.stringify(spellPicksLeft(dialog, [])), JSON.stringify({ left: 16, total: 16 }), 'cantrips plus the 1st-or-higher pool, not the sum of the nested bounds');
  const fireball = dialog.items.find((item) => item.label === 'Fireball')!.id;
  const invisibility = dialog.items.find((item) => item.label === 'Invisibility')!.id;
  assertEqual(JSON.stringify(spellPicksLeft(dialog, [fireball, invisibility])), JSON.stringify({ left: 14, total: 16 }), 'each pick counts once');
}

// Operator ruling 2026-10-11: a caster that knows its whole list (Cleric, Druid ...) gets no picker,
// only the engine's spells per day as read-only text.
function verifiesAClericGetsReadOnlySpellsPerDayAndNoPicker() {
  const dialog = creationSpellDialog(input({ perDay: [perDay('cleric', 0, 3), perDay('cleric', 1, 2)] }));
  assertEqual(dialog.items.length, 0, 'nothing to pick');
  assertEqual(Object.keys(dialog.groupLimits).length, 0, 'and no quota');
  assertEqual(dialog.readOnly.length, 1, 'one read-only caster');
  assertEqual(dialog.readOnly[0]!.text, 'Cleric knows every spell on its list. Spells per day: level 0: 3, level 1: 2.', 'its per-day counts as text');
}

function verifiesACasterWithNoKnownCountIsReadOnlyAndSaysWhy() {
  const dialog = creationSpellDialog(input({ perDay: [perDay('apg.witch', 1, 2)] }));
  assertEqual(dialog.items.length, 0, 'per day is not offered as a known-spell quota');
  assertEqual(
    dialog.readOnly[0]!.text,
    'Witch: the engine does not state how many spells a witch knows yet, so its spells are chosen on the sheet. Spells per day: level 1: 2.',
    'the gap is stated'
  );
}

function verifiesALevelWithNoKnownSpellsIsNotOffered() {
  const dialog = creationSpellDialog(input({ known: [sorcererKnown(0, 4), sorcererKnown(1, 0)] }));
  assertEqual(dialog.items.some((item) => item.label === 'Sleep'), false, 'no level-1 spells known, so none offered');
  assertEqual('sorcerer:1' in dialog.groupLimits, false, 'and no quota row for it');
}

function verifiesInnateSpellsAreListedButNeverSelectable() {
  const dialog = creationSpellDialog(input({ known: [sorcererKnown(0, 4)], innate: ['Dancing Lights', 'Unlisted Spell'] }));
  const innate = dialog.items.filter((item) => item.innate === true);
  assertEqual(innate.map((item) => item.label).join(','), 'Dancing Lights,Unlisted Spell', 'every innate spell is shown, even one the catalog lacks');
  assertEqual(innate[0]!.group, undefined, 'innate spells use no slot');
  assert((innate[1]!.description ?? '').length > 0, 'and a missing description is stated, not blank');
}

function verifiesSelectionsBecomeSpellRequests() {
  const dialog = creationSpellDialog(input({ known: [sorcererKnown(0, 4), sorcererKnown(1, 2)] }));
  const ids = dialog.items.filter((item) => item.label === 'Light' || item.label === 'Sleep').map((item) => item.id);
  const requests = spellSelectionsFromIds(ids);
  assertEqual(
    requests.map((r) => `${r.sourceClassId}:${r.spellId}:${r.acquisitionMode}`).sort().join('|'),
    'class:sorcerer:Light:Known|class:sorcerer:Sleep:Known',
    'class and spell recovered from the id'
  );
}

// A class or level change can leave picks that are no longer offered, or more picks than slots.
function verifiesStalePicksAreDroppedAndOverrunsNamed() {
  const dialog = creationSpellDialog(input({ known: [sorcererKnown(0, 1)], innate: ['Dancing Lights'] }));
  const light = dialog.items.find((item) => item.label === 'Light')!.id;
  const daze = dialog.items.find((item) => item.label === 'Daze')!.id;
  const kept = keepOfferedSpells(dialog.items, [light, 'class:wizard|Mage Armor', 'innate|Dancing Lights']);
  assertEqual(kept.join(','), light, 'a spell that is no longer offered, and innate ids, are dropped');
  assertEqual(spellQuotaOverruns(dialog, [light]).length, 0, 'within the quota');
  const over = spellQuotaOverruns(dialog, [light, daze]);
  assertEqual(over.length, 1, 'two cantrips in a one-cantrip quota');
  assertEqual(over[0], 'Sorcerer level 0: 2 chosen, 1 allowed', 'named with both numbers');

  const book = creationSpellDialog(input({ known: [spellbook('1_and_higher', 2), spellbook('2_and_higher', 1)] }));
  const ids = ['Magic Missile', 'Sleep', 'Invisibility'].map((label) => book.items.find((item) => item.label === label)!.id);
  assertEqual(spellQuotaOverruns(book, ids.slice(0, 2)).length, 0, 'two 1st-level spells fit the pool of 2');
  assertEqual(
    spellQuotaOverruns(book, ids).join(' / '),
    'Wizard spells of level 1 or higher: 3 chosen, 2 allowed',
    'every level is within its own bound, but the shared pool is not'
  );
}

// Brief 1 F5: adding a level of a class the character already has must reload the quotas. The key
// that triggers the reload used to name the classes only, so a 13th Wizard level changed nothing.
function verifiesTheReloadKeyChangesWhenALevelIsAdded() {
  const scores = [10, 10, 10, 14, 10, 10];
  const one = spellOptionsKey([{ classId: 'class:wizard', level: 1 }], 'race:human', scores, []);
  const two = spellOptionsKey([{ classId: 'class:wizard', level: 2 }], 'race:human', scores, []);
  assert(one !== two, 'a second wizard level is a different key');
  assertEqual(one, spellOptionsKey([{ classId: 'class:wizard', level: 1 }], 'race:human', scores, []), 'the same draft is the same key');
}

// Brief 1 F7: Hunter and Skald pick from their spells-known rows; Alchemist and Investigator from
// their formula book (no 0-level extracts, nested pools like the wizard's spellbook).
function verifiesHunterSkaldAlchemistAndInvestigatorGetAPicker() {
  const spontaneous = creationSpellDialog(
    input({
      perDay: [row('class_chassis.hunter.spontaneous.base_spells_per_day.spell_level_1', 1), row('class_chassis.skald.spontaneous.base_spells_per_day.spell_level_1', 1)],
      known: [
        row('class_chassis.hunter.spontaneous.spells_known.spell_level_0', 4),
        row('class_chassis.hunter.spontaneous.spells_known.spell_level_1', 2),
        row('class_chassis.skald.spontaneous.spells_known.spell_level_1', 2),
      ],
    })
  );
  assertEqual(
    JSON.stringify(spontaneous.groupLimits),
    JSON.stringify({ 'hunter:0': 4, 'hunter:1': 2, 'skald:1': 2 }),
    'each class quota is its spells known'
  );
  assertEqual(spontaneous.readOnly.length, 0, 'no "not counted yet" text for them any more');

  for (const [book, token] of [['apg.alchemist', 'alchemist'], ['acg.investigator', 'investigator']] as const) {
    const formulae = creationSpellDialog(
      input({
        perDay: [row(`class_spell.${book}.total_extracts_per_day.extract_level_1`, 2)],
        known: [row(`class_spell.${book}.formula_book_size.extract_level_1_and_higher`, 7), row(`class_spell.${book}.formula_book_size.extract_level_2_and_higher`, 2)],
      })
    );
    assertEqual(JSON.stringify(formulae.groupLimits), JSON.stringify({ [`${token}:1`]: 7, [`${token}:2`]: 2 }), `${token}: a quota per extract level`);
    assertEqual(formulae.sharedLimits.map((shared) => `${shared.groups.join('+')}=${shared.limit}`).join(), `${token}:1+${token}:2=7`, `${token}: one book pool`);
    assertEqual(formulae.items.filter((item) => item.group === `${token}:2`).map((item) => item.label).join(), "Cat's Grace", `${token}: 2nd-level formulae listed`);
    assertEqual(formulae.readOnly.length, 0, `${token}: picker, not text`);
    assertEqual(spellClassTokens([], [row(`class_spell.${book}.formula_book_size.extract_level_1_and_higher`, 7)]).join(), `class:${token}`, `${token}: the book names the class`);
  }
}

// Brief 1 F7: the Witch's familiar and the Arcanist's spellbook use the wizard's book rule (all
// cantrips; nested pools), from their own `*.spellbook_size.*` rows.
function verifiesWitchAndArcanistPickFromTheirBooks() {
  for (const [prefix, token, name] of [['apg.witch', 'witch', 'Witch'], ['acg.arcanist', 'arcanist', 'Arcanist']] as const) {
    const dialog = creationSpellDialog(
      input({
        known: [
          row(`class_spell.${prefix}.spellbook_size.spell_level_0`, 2),
          row(`class_spell.${prefix}.spellbook_size.spell_level_1_and_higher`, 5),
          row(`class_spell.${prefix}.spellbook_size.spell_level_2_and_higher`, 2),
        ],
      })
    );
    assertEqual(JSON.stringify(dialog.groupLimits), JSON.stringify({ [`${token}:0`]: 2, [`${token}:1`]: 5, [`${token}:2`]: 2 }), `${token}: book quotas`);
    assertEqual(dialog.groupLabels[`${token}:0`], `${name} level 0 (all cantrips)`, `${token}: the cantrip quota says it is every cantrip`);
    assertEqual(dialog.sharedLimits.map((shared) => shared.label).join(), `${name} spells of level 1 or higher`, `${token}: one pool`);
    assertEqual(dialog.readOnly.length, 0, `${token}: picker, not text`);
  }
}

verifiesWitchAndArcanistPickFromTheirBooks();
verifiesHunterSkaldAlchemistAndInvestigatorGetAPicker();
verifiesTheReloadKeyChangesWhenALevelIsAdded();
verifiesASorcererPicksItsSpellsKnownPerLevel();
verifiesTheClassTokensComeFromChassisAndKnownRows();
verifiesAWizardPicksItsSpellbookWithSharedPools();
verifiesAClericGetsReadOnlySpellsPerDayAndNoPicker();
verifiesACasterWithNoKnownCountIsReadOnlyAndSaysWhy();
verifiesALevelWithNoKnownSpellsIsNotOffered();
verifiesInnateSpellsAreListedButNeverSelectable();
verifiesSelectionsBecomeSpellRequests();
verifiesStalePicksAreDroppedAndOverrunsNamed();
console.log('spellDialogModel.test.ts: all assertions passed');
