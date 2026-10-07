import { assert, assertEqual } from '../testSupport/asserts';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import { creationSpellDialog, keepOfferedSpells, spellQuotaOverruns, spellSelectionsFromIds } from './spellDialogModel';

const perDay = (cls: string, level: number, value: number, basis: 'total' | 'base' = 'total'): ExplanationDto => ({
  id: `class_spell.${cls}.${basis}_spells_per_day.spell_level_${level}`,
  value,
  detail: `${cls} level ${level}`,
});

const spell = (key: string, over: Partial<SpellCatalogEntryDto> = {}): SpellCatalogEntryDto =>
  ({ key, book: 'Crb', school: 'Evocation', level: 1, description: `${key} text.`, ...over }) as SpellCatalogEntryDto;

const classLevels = (classId: string, levels: Record<string, number>): ClassSpellLevelsDto =>
  ({ classId, known: true, spellcasting: 'listIngested', spellType: 'Divine', entries: Object.entries(levels).map(([key, level]) => ({ key, level })) });

const catalog = [spell('Light'), spell('Daze'), spell('Bless'), spell('Cure Light Wounds'), spell('Dancing Lights')];
const cleric = classLevels('class:cleric', { Light: 0, Daze: 0, Bless: 1, 'Cure Light Wounds': 1 });

function verifiesEachSpellLevelGetsItsOwnQuotaFromTheEngine() {
  const dialog = creationSpellDialog({
    perDay: [perDay('cleric', 0, 3), perDay('cleric', 1, 2)],
    classLevels: [cleric],
    catalog,
    innate: [],
  });
  assertEqual(JSON.stringify(dialog.groupLimits), JSON.stringify({ 'cleric:0': 3, 'cleric:1': 2 }), 'one quota per spell level');
  assertEqual(dialog.groupLabels['cleric:1'], 'Cleric level 1', 'with a readable name');
  const light = dialog.items.find((item) => item.label === 'Light');
  assertEqual(light?.group, 'cleric:0', 'a cantrip counts against level 0');
  assertEqual(dialog.items.filter((item) => item.group === 'cleric:1').length, 2, 'level-1 spells are listed under level 1');
  assert(light?.description?.includes('Light text.') === true, 'the description is carried');
}

function verifiesALevelWithNoSlotsIsNotOffered() {
  const dialog = creationSpellDialog({ perDay: [perDay('cleric', 0, 3), perDay('cleric', 1, 0)], classLevels: [cleric], catalog, innate: [] });
  assertEqual(dialog.items.some((item) => item.label === 'Bless'), false, 'no level-1 slots, so no level-1 spells');
  assertEqual('cleric:1' in dialog.groupLimits, false, 'and no quota row for it');
}

function verifiesTheTotalRowWinsOverTheBaseRow() {
  const dialog = creationSpellDialog({ perDay: [perDay('cleric', 1, 1, 'base'), perDay('cleric', 1, 2, 'total')], classLevels: [cleric], catalog, innate: [] });
  assertEqual(dialog.groupLimits['cleric:1'], 2, 'the total (base plus bonus) is the quota');
}

function verifiesInnateSpellsAreListedButNeverSelectable() {
  const dialog = creationSpellDialog({ perDay: [perDay('cleric', 0, 3)], classLevels: [cleric], catalog, innate: ['Dancing Lights', 'Unlisted Spell'] });
  const innate = dialog.items.filter((item) => item.innate === true);
  assertEqual(innate.map((item) => item.label).join(','), 'Dancing Lights,Unlisted Spell', 'every innate spell is shown, even one the catalog lacks');
  assertEqual(innate[0]!.group, undefined, 'innate spells use no slot');
  assert((innate[1]!.description ?? '').length > 0, 'and a missing description is stated, not blank');
}

function verifiesSelectionsBecomeSpellRequests() {
  const dialog = creationSpellDialog({ perDay: [perDay('cleric', 0, 3), perDay('cleric', 1, 2)], classLevels: [cleric], catalog, innate: [] });
  const ids = dialog.items.filter((item) => item.label === 'Light' || item.label === 'Bless').map((item) => item.id);
  const requests = spellSelectionsFromIds(ids);
  assertEqual(requests.map((r) => `${r.sourceClassId}:${r.spellId}:${r.acquisitionMode}`).sort().join('|'), 'class:cleric:Bless:Known|class:cleric:Light:Known', 'class and spell recovered from the id');
}

// A class or level change can leave picks that are no longer offered, or more picks than slots.
function verifiesStalePicksAreDroppedAndOverrunsNamed() {
  const dialog = creationSpellDialog({ perDay: [perDay('cleric', 0, 1)], classLevels: [cleric], catalog, innate: ['Dancing Lights'] });
  const light = dialog.items.find((item) => item.label === 'Light')!.id;
  const daze = dialog.items.find((item) => item.label === 'Daze')!.id;
  const kept = keepOfferedSpells(dialog.items, [light, 'class:wizard|Mage Armor', 'innate|Dancing Lights']);
  assertEqual(kept.join(','), light, 'a spell that is no longer offered, and innate ids, are dropped');
  assertEqual(spellQuotaOverruns(dialog, [light]).length, 0, 'within the quota');
  const over = spellQuotaOverruns(dialog, [light, daze]);
  assertEqual(over.length, 1, 'two cantrips in a one-cantrip quota');
  assertEqual(over[0], 'Cleric level 0: 2 chosen, 1 allowed', 'named with both numbers');
}

verifiesEachSpellLevelGetsItsOwnQuotaFromTheEngine();
verifiesStalePicksAreDroppedAndOverrunsNamed();
verifiesALevelWithNoSlotsIsNotOffered();
verifiesTheTotalRowWinsOverTheBaseRow();
verifiesInnateSpellsAreListedButNeverSelectable();
verifiesSelectionsBecomeSpellRequests();
console.log('spellDialogModel.test.ts: all assertions passed');
