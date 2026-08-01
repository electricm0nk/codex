import { mapFeatCatalogEntries } from './itemPickerFilter';
import type { FeatCatalogEntryDto } from '../boundary/listFeats';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-27: the Add Feat picker's greying-out behaviour.
 *
 * The defect: there was no feat prerequisite enforcement anywhere, so the
 * picker offered all 690 feats to every character and the mutation accepted
 * every one — a Fighter 1 with a +1 base attack bonus could take Improved
 * Two-Weapon Fighting (BAB +6, Dex 17, Two-Weapon Fighting).
 *
 * The requirement is specifically **greyed with the reason**, not filtered
 * away and not offered-then-refused. These entries are the real wire shape
 * `list_feats_for_character` sends, with the reason strings the Rust engine
 * actually produces.
 */
const CHARACTER_AWARE_ENTRIES: FeatCatalogEntryDto[] = [
  {
    key: 'Improved Two-Weapon Fighting',
    category: 'Combat',
    name: 'Improved Two-Weapon Fighting',
    description: 'You are skilled at fighting with two weapons.',
    source: 'Crb',
    chooserTargetKind: null,
    eligibility: {
      eligible: false,
      unavailableReason:
        'requires the Two-Weapon Fighting feat (you have 0 of the 1 needed); requires DEX 17 (you have 13); requires a feature waiving the Dex 17 requirement (you have 0); requires base attack bonus +6 (you have +1)',
      met: [],
      unmet: [
        'requires the Two-Weapon Fighting feat (you have 0 of the 1 needed)',
        'requires DEX 17 (you have 13); requires a feature waiving the Dex 17 requirement (you have 0)',
        'requires base attack bonus +6 (you have +1)',
      ],
      unverified: [],
      prerequisiteCount: 3,
    },
  },
  {
    key: 'Toughness',
    category: 'General',
    name: 'Toughness',
    description: 'You have enhanced physical stamina.',
    source: 'Crb',
    chooserTargetKind: null,
    eligibility: {
      eligible: true,
      unavailableReason: null,
      met: [],
      unmet: [],
      unverified: [],
      prerequisiteCount: 0,
    },
  },
  {
    key: 'Combat Expertise',
    category: 'Combat',
    name: 'Combat Expertise',
    description: 'You can increase your defense at the expense of your accuracy.',
    source: 'Crb',
    chooserTargetKind: null,
    eligibility: {
      eligible: true,
      unavailableReason: null,
      met: [],
      unmet: [],
      unverified: [
        'not verified: one of its alternatives could not be evaluated (references a PCGen runtime variable this engine does not model) (PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13])',
      ],
      prerequisiteCount: 1,
    },
  },
];

/** The character-less `list_feats` shape: no `eligibility` key at all. */
const CHARACTER_LESS_ENTRIES: FeatCatalogEntryDto[] = [
  {
    key: 'Improved Two-Weapon Fighting',
    category: 'Combat',
    name: 'Improved Two-Weapon Fighting',
    description: 'You are skilled at fighting with two weapons.',
    source: 'Crb',
    chooserTargetKind: null,
  },
];

function run() {
  // --- an ineligible feat is shown, disabled, and carries its reason -----
  const rows = mapFeatCatalogEntries(CHARACTER_AWARE_ENTRIES);
  assertEqual(rows.length, 3, 'every record must still reach the picker');

  const improved = rows.find((row) => row.key === 'Improved Two-Weapon Fighting');
  assert(improved !== undefined, 'the ineligible feat must not be filtered out of the picker');
  assertEqual(improved!.disabled, true, 'an ineligible feat must render disabled');
  assert(
    (improved!.disabledReason ?? '').includes('base attack bonus +6'),
    'the greyed row must say why, naming the unmet prerequisite'
  );
  assert(
    (improved!.disabledReason ?? '').includes('Two-Weapon Fighting'),
    'the greyed row must name every unmet prerequisite, not just the first'
  );

  // --- an eligible feat stays selectable with no reason attached ---------
  const toughness = rows.find((row) => row.key === 'Toughness');
  assertEqual(toughness!.disabled, false, 'an eligible feat must stay selectable');
  assertEqual(toughness!.disabledReason, undefined, 'an available feat must carry no denial text');
  assertEqual(toughness!.unverifiedNote, undefined, 'a fully-checked feat carries no note');

  // --- an unverifiable prerequisite notes but never blocks ---------------
  const expertise = rows.find((row) => row.key === 'Combat Expertise');
  assertEqual(
    expertise!.disabled,
    false,
    'a prerequisite the engine could not evaluate must never grey a row out'
  );
  assert(
    (expertise!.unverifiedNote ?? '').includes('not verified'),
    'an unverifiable prerequisite must be shown as a note, not silently passed'
  );

  // --- the character-less browse keeps its previous behaviour exactly ----
  const browseRows = mapFeatCatalogEntries(CHARACTER_LESS_ENTRIES);
  assertEqual(
    browseRows[0].disabled,
    undefined,
    'with no character to check against, no row may be greyed out'
  );
  assertEqual(browseRows[0].disabledReason, undefined, 'and no denial text either');

  console.log('featPickerEligibility.test.ts: all assertions passed');
}

run();
