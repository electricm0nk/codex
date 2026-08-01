import {
  blocksByAlternateKey,
  describeBlock,
  describeCharacterContext,
  describePicker,
  describeReplacement,
  describeSelectionOutcome,
  descriptionsByTraitKey,
  traitDescription,
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
 * the orphan-flag case below is the shape ARG's nine Aasimar alternates had
 * on disk until 2026-07-31). They are test inputs for pure view functions,
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
    renderedTraitDescriptions: [],
    displayValueFeats: [],
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
 * The case that must never render blank: an alternate whose replace-flag no
 * standard trait declares. Showing "Replaces —" would read as a display bug;
 * the line has to say what is actually true.
 *
 * **No shipped record is in this state today.** ARG's nine Aasimar alternates
 * were, until `src/bin/ingest_races.rs` learned to read the gate PCGen states
 * in `aasimar_abilities_globalvar.lst` rather than on the trait row, and
 * `race_trait_picker`'s
 * `no_alternate_in_the_menu_can_ever_be_refused_for_an_unmatched_flag` now pins
 * that the live menu carries no unmatched flag at all. The rendering stays
 * because the next book can reintroduce the shape, and a synthetic input is
 * the honest way to keep testing it once the real one is gone.
 */
const orphan = describeReplacement(
  alternate({
    key: 'Aasimar ~ Halo (synthetic: the pre-2026-07-31 payload)',
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

// --- rendered descriptions are shown, never re-derived --------------------

/**
 * The payload shapes below are copied from a real
 * `resolve_race_alternate_selection` response — the same three sentences
 * `race_trait_picker.rs`'s
 * `the_payload_renders_a_different_sentence_for_a_character_holding_the_feats`
 * pins against the on-disk corpus. They are inputs to pure view functions; the
 * screen never assembles a sentence of its own, which is exactly what these
 * assertions check.
 */
const luckBase = 'Three times per day, a halfling can gain a +2 luck bonus…';
const luckBoth = '5 times per day, a halfling can gain a +4 luck bonus…';

const withCharacter = selectionResponse({
  displayValueFeats: ['Fortunate One', 'Adaptive Fortune'],
  renderedTraitDescriptions: [
    { key: 'Halfling ~ Adaptable Luck', name: 'Adaptable Luck', text: luckBoth, droppedArgs: [], movedByFeats: true },
    { key: 'Halfling ~ Keen Senses', name: 'Keen Senses', text: 'Halflings receive a +2 bonus on Perception checks.', droppedArgs: [], movedByFeats: false },
  ],
});

const byKey = descriptionsByTraitKey(withCharacter);
assertEqual(byKey.size, 2, 'every rendered row is indexed');
assertEqual(
  traitDescription(byKey, 'Halfling ~ Adaptable Luck', luckBase),
  luckBoth,
  "the character's rendered sentence wins over the menu's printed one",
);
assertEqual(
  traitDescription(byKey, 'Halfling ~ Unseen Trait', luckBase),
  luckBase,
  'a trait the engine did not render falls back to the menu prose, never to a blank',
);
assertEqual(
  traitDescription(descriptionsByTraitKey(null), 'Halfling ~ Adaptable Luck', luckBase),
  luckBase,
  'before the resolution returns the screen shows the printed value, not nothing',
);

// --- the context line credits only what the engine says moved -------------

assert(
  describeCharacterContext(null, withCharacter).includes('as the book prints it'),
  'with no character the screen says whose numbers these are',
);
const credited = describeCharacterContext('Bilbo', withCharacter);
assert(credited.includes('Fortunate One, Adaptive Fortune'), 'the feats that moved a value are named');
assert(credited.includes('1 trait'), 'the count of changed sentences is derived from the payload');
assertEqual(
  describeCharacterContext('Bilbo', selectionResponse()),
  "Showing Bilbo's numbers. None of their feats changes a racial trait's stated value.",
  'a character whose feats moved nothing is told so, not implied to have changed something',
);
assertEqual(
  describeCharacterContext('Bilbo', null),
  "Reading Bilbo's feats…",
  'before the engine answers, the screen claims nothing about what did or did not move',
);

console.log('alternateTraitPickerModel: all assertions passed');
