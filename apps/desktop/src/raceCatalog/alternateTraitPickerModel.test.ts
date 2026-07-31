import {
  blocksByAlternateKey,
  describeBlock,
  describePicker,
  describeReplacement,
  describeSelectionOutcome,
  orderRacesByAlternateCount,
  selectionWarnings,
  suppressionsByTraitKey,
  toggleSelection,
} from './alternateTraitPickerModel';
import { RACE_CATALOG_VIEWS } from './RaceCatalogScreen';
import { NO_RUNTIME_MESSAGE } from './alternateTraitPickerRuntime';
import type {
  AlternateRacialTraitsResponse,
  AlternateTraitDto,
  RacePickerDto,
  RaceSelectionResponse,
} from '../boundary/loadAlternateRacialTraits';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The Alternate Racial Traits picker must present the backend's answers and
 * add none of its own.
 *
 * The shapes below are copied from real payloads produced by
 * `race_trait_picker.rs` against the on-disk corpus (`Dwarf ~ Saltbeard` sets
 * four flags, replaces four standard traits and grants ARG's own Greed;
 * `Aasimar ~ Halo`'s flag matches no standard trait because Aasimar's upstream
 * file declares no gates at all). They are test inputs for pure view functions,
 * never data any screen renders.
 */

function alternate(overrides: Partial<AlternateTraitDto> = {}): AlternateTraitDto {
  return {
    key: 'Dwarf ~ Saltbeard',
    name: 'Saltbeard',
    book: 'ARG',
    description: 'Dwarves occasionally found iron cities along rugged seacoasts.',
    sourcePage: 'p.12',
    setsFlags: [
      'Dwarf_ReplaceDefensiveTraining',
      'Dwarf_ReplaceHatred',
      'Dwarf_ReplaceStonecunning',
      'Dwarf_ReplaceGreed',
    ],
    replaces: [
      { key: 'Dwarf ~ Defensive Training', name: 'Defensive Training', flag: 'Dwarf_ReplaceDefensiveTraining' },
      { key: 'Dwarf ~ Hatred', name: 'Hatred', flag: 'Dwarf_ReplaceHatred' },
      { key: 'Dwarf ~ Stonecunning', name: 'Stonecunning', flag: 'Dwarf_ReplaceStonecunning' },
      { key: 'Dwarf ~ Greed', name: 'Greed', flag: 'Dwarf_ReplaceGreed' },
    ],
    grants: [{ key: 'Saltbeard ~ Dwarf ~ Greed', name: 'Greed', flag: 'Dwarf_ReplaceGreed' }],
    unmatchedFlags: [],
    exclusionGuardFlags: ['Dwarf_ReplaceDefensiveTraining', 'Dwarf_ReplaceHatred', 'Dwarf_ReplaceStonecunning'],
    ...overrides,
  };
}

function race(overrides: Partial<RacePickerDto> = {}): RacePickerDto {
  return {
    raceId: 'Dwarf',
    raceKey: 'Dwarf',
    raceName: 'Dwarf',
    book: 'CRB',
    standardTraits: [],
    alternates: [alternate()],
    ...overrides,
  };
}

function selectionResponse(overrides: Partial<RaceSelectionResponse> = {}): RaceSelectionResponse {
  return {
    raceId: 'Dwarf',
    raceKey: 'Dwarf',
    raceName: 'Dwarf',
    book: 'CRB',
    appliedTraits: [],
    suppressions: [],
    firedFlags: [],
    inertFlags: [],
    unmatchedSelections: [],
    blockedAlternates: [],
    conflictingSelections: [],
    errors: [],
    ...overrides,
  };
}

// --- selection state -------------------------------------------------------

const empty: string[] = [];
const one = toggleSelection(empty, 'Dwarf ~ Saltbeard');
assertEqual(one.length, 1, 'toggling an unselected key selects it');
assertEqual(toggleSelection(one, 'Dwarf ~ Saltbeard').length, 0, 'toggling a selected key clears it');
assertEqual(empty.length, 0, 'toggleSelection does not mutate its input');

// --- summary lines are derived, never hardcoded ---------------------------

const menu: AlternateRacialTraitsResponse = {
  races: [race(), race({ raceKey: 'Elf', raceName: 'Elf', alternates: [alternate({ key: 'Elf ~ A' }), alternate({ key: 'Elf ~ B' })] })],
  diagnostics: [],
  findings: [],
};
assertEqual(describePicker(menu), '3 alternate racial traits across 2 races', 'summary counts what arrived');
assertEqual(
  describePicker({ races: [race()], diagnostics: [], findings: [] }),
  '1 alternate racial trait across 1 race',
  'singular wording',
);

// --- what an alternate replaces -------------------------------------------

const saltbeardLine = describeReplacement(alternate());
assert(
  saltbeardLine.startsWith('Replaces Defensive Training, Hatred, Stonecunning, Greed'),
  `names every replaced trait: ${saltbeardLine}`,
);
assert(saltbeardLine.includes('Grants Greed'), `names what it grants: ${saltbeardLine}`);

/**
 * The case that must never render blank. Nine of ARG's 153 alternates (all
 * Aasimar's) set a flag no standard trait declares, because Aasimar's upstream
 * file carries no gates. Showing "Replaces —" would read as a display bug;
 * the line has to say what is actually true.
 */
const orphan = describeReplacement(
  alternate({
    key: 'Aasimar ~ Halo',
    name: 'Halo',
    setsFlags: ['Aasimar_ReplaceSkilled'],
    replaces: [],
    grants: [],
    unmatchedFlags: ['Aasimar_ReplaceSkilled'],
    exclusionGuardFlags: ['Aasimar_ReplaceSkilled'],
  }),
);
assert(orphan.includes('Aasimar_ReplaceSkilled'), `names the unmatched flag: ${orphan}`);
assert(orphan.includes('corpus finding'), `labels it a finding, not a gap: ${orphan}`);
assertEqual(orphan.includes('Replaces nothing in the loaded books'), true, 'states the fact plainly');

// --- suppression comes from the engine's own report ------------------------

const resolved = selectionResponse({
  appliedTraits: [
    { key: 'Dwarf ~ Saltbeard', name: 'Saltbeard', book: 'ARG', role: 'alternate', description: '' },
    { key: 'Saltbeard ~ Dwarf ~ Greed', name: 'Greed', book: 'ARG', role: 'flagGranted', description: '' },
  ],
  suppressions: [
    {
      suppressedTraitKey: 'Dwarf ~ Greed',
      suppressedTraitName: 'Greed',
      flag: 'Dwarf_ReplaceGreed',
      setByTraitKey: 'Dwarf ~ Saltbeard',
      setByTraitName: 'Saltbeard',
    },
  ],
  firedFlags: ['Dwarf_ReplaceGreed'],
});
const suppressed = suppressionsByTraitKey(resolved);
assertEqual(suppressed.size, 1, 'one suppression indexed');
assertEqual(suppressed.get('Dwarf ~ Greed')?.setByTraitName, 'Saltbeard', 'indexed by the suppressed trait key');
assertEqual(suppressionsByTraitKey(null).size, 0, 'no selection resolved yet means nothing is struck through');

const outcome = describeSelectionOutcome(resolved);
assert(outcome.includes('Greed (by Saltbeard)'), `names both ends of the swap: ${outcome}`);
assertEqual(
  describeSelectionOutcome(selectionResponse({ appliedTraits: resolved.appliedTraits })),
  '2 traits apply. No alternate selected, so nothing is replaced.',
  'the no-selection case says so rather than implying a swap',
);
assertEqual(describeSelectionOutcome(null), 'Resolving…', 'the pre-response state is not a false zero');
assertEqual(
  describeSelectionOutcome(selectionResponse({ errors: ['no race in the loaded corpus matches "Balor"'] })),
  'no race in the loaded corpus matches "Balor"',
  'an error is shown, not swallowed',
);

// --- mutual exclusion -----------------------------------------------------

const blockedResponse = selectionResponse({
  blockedAlternates: [
    {
      key: 'Dwarf ~ Ancient Enmity',
      name: 'Ancient Enmity',
      flag: 'Dwarf_ReplaceHatred',
      blockedByKey: 'Dwarf ~ Saltbeard',
      blockedByName: 'Saltbeard',
    },
  ],
});
const blocks = blocksByAlternateKey(blockedResponse);
assertEqual(blocks.size, 1, 'one lock-out indexed');
const blockLine = describeBlock(blocks.get('Dwarf ~ Ancient Enmity')!);
assert(blockLine.includes('Saltbeard'), `names what locked it out: ${blockLine}`);
assert(blockLine.includes('Dwarf_ReplaceHatred'), `names the guard flag: ${blockLine}`);
assertEqual(blocksByAlternateKey(null).size, 0, 'nothing is locked out before a resolution arrives');

// --- warnings the player must see -----------------------------------------

const warnings = selectionWarnings(
  selectionResponse({
    unmatchedSelections: ['Dwarf ~ Saltbeerd'],
    conflictingSelections: [
      {
        key: 'Dwarf ~ Saltbeard',
        name: 'Saltbeard',
        flag: 'Dwarf_ReplaceHatred',
        blockedByKey: 'Dwarf ~ Ancient Enmity',
        blockedByName: 'Ancient Enmity',
      },
    ],
    inertFlags: ['Aasimar_ReplaceSkilled'],
  }),
);
assertEqual(warnings.length, 3, 'every problem class is surfaced');
assert(warnings[0].includes('Dwarf ~ Saltbeerd'), 'a typo’d selection is reported, not ignored');
assert(warnings[1].includes('cannot both be taken'), 'an illegal pair is reported');
assert(warnings[2].includes('replaced and granted nothing'), 'an inert flag is reported');
assertEqual(selectionWarnings(null).length, 0, 'no warnings before a resolution');

// --- race ordering keeps every race, including empty ones -----------------

const ordered = orderRacesByAlternateCount([
  race({ raceKey: 'Svirfneblin', raceName: 'Svirfneblin', alternates: [] }),
  race({ raceKey: 'Dwarf', raceName: 'Dwarf', alternates: [alternate(), alternate({ key: 'x' })] }),
  race({ raceKey: 'Elf', raceName: 'Elf', alternates: [alternate({ key: 'y' })] }),
]);
assertEqual(ordered.length, 3, 'a race with no alternates is kept, not hidden');
assertEqual(ordered[0].raceName, 'Dwarf', 'most alternates first');
assertEqual(ordered[2].raceName, 'Svirfneblin', 'the empty race sorts last but still appears');

// --- the screen offers both halves of the rule ----------------------------

assertEqual(RACE_CATALOG_VIEWS.length, 2, 'Race Traits carries both the catalog and the picker');
assertEqual(RACE_CATALOG_VIEWS[0].view, 'standard', 'the flat catalog stays the default view');
assertEqual(RACE_CATALOG_VIEWS[1].view, 'alternates', 'the picker is one click from it');

/**
 * Without the desktop runtime the picker refuses to invent a resolution.
 * A browser-preview fixture here would mean re-implementing the replace-flag
 * protocol in TypeScript — a second, divergent rules engine.
 */
assert(NO_RUNTIME_MESSAGE.includes('resolved by the'), 'the browser message explains why, not just that');
assert(!NO_RUNTIME_MESSAGE.includes('coming soon'), 'no promise stands in for the reason');

console.log('alternateTraitPickerModel: all assertions passed');
