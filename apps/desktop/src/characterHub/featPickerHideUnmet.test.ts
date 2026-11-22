import {
  HIDE_UNMET_FEATS_LABEL,
  filterItemPickerEntries,
  mapFeatCatalogEntries,
  selectableItemPickerEntry,
  visibleItemPickerEntries,
} from './itemPickerFilter';
import { buildItemPickerConfig } from './CharacterSheet';
import type { FeatCatalogEntryDto } from '../boundary/listFeats';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The Add Feat picker's "Hide feats I don't qualify for" (R10): ticked, a feat
 * whose prerequisites are unmet is not listed; unticked, it is listed greyed
 * with what is missing, and Add stays disabled for it.
 *
 * The rows are the wire shape `list_feats_for_character` sends, with the
 * engine's reason strings (`featPickerEligibility.test.ts` holds the same
 * Improved Two-Weapon Fighting row). The prerequisite verdict itself is the
 * engine's and is not re-derived here.
 */
const UNMET = [
  'requires the Two-Weapon Fighting feat (you have 0 of the 1 needed)',
  'requires base attack bonus +6 (you have +1)',
];

function entry(key: string, eligibility: FeatCatalogEntryDto['eligibility']): FeatCatalogEntryDto {
  return { key, category: 'Combat', name: key, description: null, source: 'Crb', chooserTargetKind: null, eligibility };
}

const ROWS = mapFeatCatalogEntries([
  entry('Improved Two-Weapon Fighting', {
    eligible: false,
    unavailableReason: UNMET.join('; '),
    met: [],
    unmet: UNMET,
    unverified: [],
    prerequisiteCount: 2,
  }),
  entry('Toughness', { eligible: true, unavailableReason: null, met: [], unmet: [], unverified: [], prerequisiteCount: 0 }),
  // Selectable with a note: an unverified prerequisite never blocks, so it must not hide either.
  entry('Combat Expertise', {
    eligible: true,
    unavailableReason: null,
    met: [],
    unmet: [],
    unverified: ['not verified: the referenced rules variable has no converted table'],
    prerequisiteCount: 1,
  }),
  // The character-less catalog: no verdict, nothing to hide.
  entry('Power Attack', undefined),
]);

const names = (rows: typeof ROWS) => rows.map((row) => row.name).join(',');

assertEqual(HIDE_UNMET_FEATS_LABEL, "Hide feats I don't qualify for", 'the label the operator named');

// Ticked: unmet hidden, everything the character may take stays.
const ticked = visibleItemPickerEntries(ROWS, true);
assertEqual(names(ticked), 'Toughness,Combat Expertise,Power Attack', 'ticked hides only the unmet feat');
assertEqual(
  filterItemPickerEntries(ticked, 'two-weapon').length,
  0,
  'a search for the hidden feat finds nothing while ticked'
);

// Unticked: every row is listed, the unmet one greyed with all it lacks.
const unticked = visibleItemPickerEntries(ROWS, false);
assertEqual(unticked.length, ROWS.length, 'unticked lists every row');
const unmetRow = unticked.find((row) => row.name === 'Improved Two-Weapon Fighting');
assert(unmetRow !== undefined, 'the unmet feat is listed when unticked');
assertEqual(unmetRow!.disabled, true, 'the unmet feat is greyed');
for (const line of UNMET) {
  assert(unmetRow!.disabledReason?.includes(line) === true, `the greyed row names: ${line}`);
}

// Add follows the selection: a greyed row can never become it, even from a stale key.
assertEqual(selectableItemPickerEntry(unticked, 'Improved Two-Weapon Fighting'), null, 'Add stays disabled on the unmet feat');
assertEqual(selectableItemPickerEntry(unticked, 'Toughness')?.name, 'Toughness', 'Add is live on a feat the character may take');
assertEqual(selectableItemPickerEntry(unticked, null), null, 'nothing selected, nothing to add');
assertEqual(selectableItemPickerEntry(ticked, 'Improved Two-Weapon Fighting'), null, 'a key hidden by ticking the box cannot be added');

// Only the feat picker offers the box. A level-up pick opens the same 'feat' kind.
const noLoad = () => Promise.resolve([]);
const noSelect = () => undefined;
const deps = {
  loadEquipment: noLoad,
  loadSpells: noLoad,
  loadFeats: noLoad,
  loadFeatTargets: noLoad,
  onSelectSpell: noSelect,
  onSelectFeat: noSelect,
  onSelectFeatTarget: noSelect,
  onSelectModifier: noSelect,
};
assertEqual(buildItemPickerConfig('feat', deps)?.offersHideUnavailable, true, 'Add Feat offers the box');
for (const kind of ['spell', 'featTarget', 'modifier'] as const) {
  assertEqual(buildItemPickerConfig(kind, deps)?.offersHideUnavailable, undefined, `${kind} does not`);
}

// Under the prerequisites override the same unmet row can be added, and still says what it lacks.
const ADMITTED = mapFeatCatalogEntries(
  [
    entry('Improved Two-Weapon Fighting', {
      eligible: false,
      unavailableReason: 'requires base attack bonus at least 6',
      met: [],
      unmet: ['base attack bonus at least 6'],
      unverified: [],
      prerequisiteCount: 1,
    }),
  ],
  true
);
assertEqual(ADMITTED[0].disabled, false, 'the override lets the unmet feat be picked');
assert(ADMITTED[0].unverifiedNote?.includes('base attack bonus at least 6') === true, 'and the row still names what is missing');

console.log('featPickerHideUnmet.test.ts: all assertions passed');
