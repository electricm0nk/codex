import {
  alternatesForRaceId,
  buildAlternateTraitRows,
  creationSelectionWarnings,
  describeCreationSelection,
  pickerRaceForRaceId,
  retainSelectionsValidForRace,
} from './alternateTraitSelection';
import type {
  AlternateRacialTraitsResponse,
  AlternateTraitDto,
  RacePickerDto,
  RaceSelectionResponse,
} from '../boundary/loadAlternateRacialTraits';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * Taking an alternate racial trait during character creation.
 *
 * The payload shapes below are copied from real `race_trait_picker.rs` output
 * against the on-disk corpus. They are inputs to pure view functions; no screen
 * renders them, and none of them decides a rule — every suppression and every
 * lockout asserted here is one the backend reported.
 */

function alternate(overrides: Partial<AlternateTraitDto> = {}): AlternateTraitDto {
  return {
    key: 'Dwarf ~ Saltbeard',
    name: 'Saltbeard',
    book: 'ARG',
    description: 'Dwarves occasionally found iron cities along rugged seacoasts.',
    sourcePage: 'p.12',
    setsFlags: ['Dwarf_ReplaceDefensiveTraining', 'Dwarf_ReplaceGreed'],
    replaces: [{ key: 'Dwarf ~ Greed', name: 'Greed', flag: 'Dwarf_ReplaceGreed' }],
    grants: [{ key: 'Saltbeard ~ Dwarf ~ Greed', name: 'Greed', flag: 'Dwarf_ReplaceGreed' }],
    unmatchedFlags: [],
    exclusionGuardFlags: ['Dwarf_ReplaceDefensiveTraining'],
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
    alternates: [
      alternate(),
      alternate({ key: 'Dwarf ~ Minesight', name: 'Minesight', setsFlags: ['Dwarf_ReplaceVision'] }),
      alternate({ key: 'Dwarf ~ Ancient Enmity', name: 'Ancient Enmity', setsFlags: ['Dwarf_ReplaceHatred'] }),
    ],
    ...overrides,
  };
}

const MENU: AlternateRacialTraitsResponse = {
  races: [
    race(),
    race({
      raceId: 'HalfElf',
      raceKey: 'Half-Elf',
      raceName: 'Half-Elf',
      alternates: [alternate({ key: 'Half-Elf ~ Dual Minded', name: 'Dual Minded' })],
    }),
  ],
  adoptiveParentageOptions: [],
  adoptedRaceOptions: [],
  diagnostics: [],
  findings: [],
};

function resolution(overrides: Partial<RaceSelectionResponse> = {}): RaceSelectionResponse {
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

async function main() {
  matchesTheCreationFormsRaceIdToThePickersOwnRaceKey();
  offersOnlyTheChosenRacesAlternatesAndDropsStaleOnesOnRaceChange();
  neverDisablesAnythingUntilTheBackendSaysSo();
  neverDisablesAnAlreadySelectedAlternate();
  reportsTheEnginesOwnSuppressionsRatherThanDerivingThem();
  surfacesEveryConditionThatWouldBlockTheSave();
  everyRowCarriesTheProseTheEngineRendered();
  aRowsSentenceIsTheOneRenderedForWhoeverTheResolutionNamed();
  console.log('alternateTraitSelection: all assertions passed');
}

/**
 * `race_catalog.rs` builds the creation roster's id as
 * `format!("race:{}", race_key.to_lowercase())`, so `race:half-elf` is
 * `Half-Elf`. Matched on that construction rule, not a mapping table.
 */
function matchesTheCreationFormsRaceIdToThePickersOwnRaceKey() {
  assertEqual(pickerRaceForRaceId(MENU, 'race:dwarf')?.raceKey, 'Dwarf', 'race:dwarf matches Dwarf');
  assertEqual(pickerRaceForRaceId(MENU, 'race:half-elf')?.raceKey, 'Half-Elf', 'the hyphen survives');
  assertEqual(pickerRaceForRaceId(MENU, 'Dwarf')?.raceKey, 'Dwarf', 'a bare key matches too');
  assertEqual(pickerRaceForRaceId(MENU, 'race:kobold'), null, 'a race the picker did not serve');
  assertEqual(pickerRaceForRaceId(null, 'race:dwarf'), null, 'no menu loaded yet');
}

function offersOnlyTheChosenRacesAlternatesAndDropsStaleOnesOnRaceChange() {
  const dwarf = alternatesForRaceId(MENU, 'race:dwarf').map((option) => option.name);
  assertEqual(dwarf.join(', '), 'Ancient Enmity, Minesight, Saltbeard', 'alphabetical by name');
  assertEqual(alternatesForRaceId(MENU, 'race:half-elf').length, 1, 'only the Half-Elf alternate');
  assertEqual(alternatesForRaceId(MENU, 'race:kobold').length, 0, 'a race the menu did not serve offers none');

  // Switching Dwarf -> Half-Elf must not carry the Dwarf's choice over: the
  // backend would refuse the save, and refusing is worse than clearing.
  assertEqual(
    retainSelectionsValidForRace(MENU, 'race:half-elf', ['Dwarf ~ Saltbeard', 'Half-Elf ~ Dual Minded']).join(),
    'Half-Elf ~ Dual Minded',
    "the previous race's choice is dropped"
  );
  assertEqual(
    retainSelectionsValidForRace(MENU, 'race:dwarf', ['Dwarf ~ Saltbeard']).join(),
    'Dwarf ~ Saltbeard',
    'a still-valid choice is kept'
  );
}

/**
 * Until the engine has resolved the current selection, nothing is disabled.
 * A lockout the backend has not confirmed would be the frontend deciding a
 * rule — which is exactly what `race_trait_picker.rs` exists to prevent.
 */
function neverDisablesAnythingUntilTheBackendSaysSo() {
  const rows = buildAlternateTraitRows(MENU, 'race:dwarf', ['Dwarf ~ Saltbeard'], null);
  assertEqual(rows.length, 3, 'one row per Dwarf alternate');
  assertEqual(rows.filter((row) => row.disabledReason !== null).length, 0, 'nothing disabled yet');
  assertEqual(rows.filter((row) => row.selected).length, 1, 'the chosen one is marked');
  assertEqual(rows.find((row) => row.selected)?.alternate.key, 'Dwarf ~ Saltbeard', 'and it is the right one');
}

function neverDisablesAnAlreadySelectedAlternate() {
  const blocked = resolution({
    blockedAlternates: [
      {
        key: 'Dwarf ~ Ancient Enmity',
        name: 'Ancient Enmity',
        flag: 'Dwarf_ReplaceHatred',
        blockedByKey: 'Dwarf ~ Saltbeard',
        blockedByName: 'Saltbeard',
      },
      // The backend never reports a selected alternate as blocked, but if it
      // did, the player must still be able to undo the choice.
      {
        key: 'Dwarf ~ Saltbeard',
        name: 'Saltbeard',
        flag: 'Dwarf_ReplaceGreed',
        blockedByKey: 'Dwarf ~ Minesight',
        blockedByName: 'Minesight',
      },
    ],
  });
  const rows = buildAlternateTraitRows(MENU, 'race:dwarf', ['Dwarf ~ Saltbeard'], blocked);
  const enmity = rows.find((row) => row.alternate.key === 'Dwarf ~ Ancient Enmity');
  assert(enmity?.disabledReason?.includes('Locked out by Saltbeard') ?? false, 'the rival is locked out');
  assert(enmity?.disabledReason?.includes('Dwarf_ReplaceHatred') ?? false, 'and the flag is named as evidence');
  assertEqual(
    rows.find((row) => row.alternate.key === 'Dwarf ~ Saltbeard')?.disabledReason,
    null,
    'a chosen alternate is never un-choosable',
  );
}

function reportsTheEnginesOwnSuppressionsRatherThanDerivingThem() {
  assert(
    describeCreationSelection([], null).includes('keeps every standard trait'),
    'the empty case says what it means',
  );
  assertEqual(
    describeCreationSelection(['Dwarf ~ Saltbeard'], null),
    'Resolving…',
    'no claim is made before the engine answers'
  );

  const swapped = resolution({
    suppressions: [
      {
        suppressedTraitKey: 'Dwarf ~ Greed',
        suppressedTraitName: 'Greed',
        flag: 'Dwarf_ReplaceGreed',
        setByTraitKey: 'Dwarf ~ Saltbeard',
        setByTraitName: 'Saltbeard',
      },
    ],
  });
  assertEqual(
    describeCreationSelection(['Dwarf ~ Saltbeard'], swapped),
    '1 alternate trait chosen. Replaces: Greed (by Saltbeard).',
    "the engine's own suppression, named on both ends"
  );

  // The real Aasimar case: the alternate applies and replaces nothing,
  // because its swap target's gate lives in an un-ingested upstream file.
  assertEqual(
    describeCreationSelection(['Aasimar ~ Halo'], resolution()),
    '1 alternate trait chosen, replacing nothing in the loaded books.',
    'replacing nothing is stated, not left blank'
  );
  assertEqual(
    describeCreationSelection(['x'], resolution({ errors: ['no such race'] })),
    'no such race',
    "the backend's own error is shown verbatim"
  );
}

/**
 * Every condition `create_character` refuses to persist must be visible before
 * the player presses the button. A form that hides the reason and then reports
 * "Blocked" is withholding an answer it already had.
 */
function surfacesEveryConditionThatWouldBlockTheSave() {
  assertEqual(creationSelectionWarnings(null).length, 0, 'no resolution yet, no warnings');
  assertEqual(creationSelectionWarnings(resolution()).length, 0, 'a clean resolution warns about nothing');

  const bad = resolution({
    unmatchedSelections: ['Dwarf ~ Saltbeerd'],
    conflictingSelections: [
      {
        key: 'Dwarf ~ Sky Sentinel',
        name: 'Sky Sentinel',
        flag: 'Dwarf_ReplaceStonecunning',
        blockedByKey: 'Dwarf ~ Saltbeard',
        blockedByName: 'Saltbeard',
      },
    ],
    inertFlags: ['Aasimar_ReplaceVision'],
  });
  const warnings = creationSelectionWarnings(bad);
  assertEqual(warnings.length, 3, 'one warning per blocking condition');
  assert(warnings[0].includes('Dwarf ~ Saltbeerd'), 'the typo is named');
  assert(warnings[1].includes('Sky Sentinel') && warnings[1].includes('Saltbeard'), 'both ends of the conflict');
  assert(warnings[2].includes('Aasimar_ReplaceVision'), 'the inert flag is named');
}

/**
 * **The creation form used to show a name, a page reference and "Replaces X",
 * and no numbers at all.** A player choosing between `Saltbeard` and
 * `Minesight` saw two names and could not tell what either one was worth.
 *
 * The prose was never missing from the engine — `race_trait_picker` renders
 * every alternate's `DESC:` tokens and ships the result on both the menu and
 * the resolve payload. It simply reached no part of this screen except a
 * `title` tooltip. These assertions are about the row carrying it.
 */
function everyRowCarriesTheProseTheEngineRendered() {
  // Before the engine answers, the menu's own rendering (the same renderer,
  // called with no feats) is what there is — never a blank cell.
  const early = buildAlternateTraitRows(MENU, 'race:dwarf', [], null);
  for (const row of early) {
    assertEqual(row.description, row.alternate.description, "the menu's rendering until one arrives");
    assertEqual(row.movedByFeats, false, 'no resolution, no claim that anything moved');
    assertEqual(row.droppedArgs.length, 0, 'and nothing claimed incomplete');
  }

  // Once it does, the resolution's rendering wins: it is the one rendered for
  // whoever the call named.
  const resolved = buildAlternateTraitRows(
    MENU,
    'race:dwarf',
    [],
    resolution({
      renderedTraitDescriptions: [
        {
          key: 'Dwarf ~ Saltbeard',
          name: 'Saltbeard',
          text: 'Saltbeard dwarves gain a +2 racial bonus on Profession (sailor) checks.',
          droppedArgs: [],
          movedByFeats: false,
        },
      ],
    }),
  );
  const saltbeard = resolved.find((row) => row.alternate.key === 'Dwarf ~ Saltbeard');
  assert(saltbeard!.description.includes('+2 racial bonus'), `the number reaches the row: ${saltbeard!.description}`);
  const minesight = resolved.find((row) => row.alternate.key === 'Dwarf ~ Minesight');
  assertEqual(
    minesight!.description,
    minesight!.alternate.description,
    'a record the resolution did not render keeps the menu sentence rather than blanking',
  );
}

/**
 * **The same-record-different-sentence proof, at the creation surface.**
 *
 * `resolveRaceAlternateSelection` takes the held feats as a sibling argument,
 * so the two payloads below are the same corpus record rendered for two
 * different characters. The row must show whichever one it was handed — a row
 * that reads the menu's static `description` instead would show the book's
 * printed number to a character whose feats have already changed it.
 */
function aRowsSentenceIsTheOneRenderedForWhoeverTheResolutionNamed() {
  const base = 'Three times per day, a halfling can gain a +2 luck bonus on an ability check.';
  const fed = '4 times per day, a halfling can gain a +2 luck bonus on an ability check.';
  const luckMenu: AlternateRacialTraitsResponse = {
    races: [
      race({
        raceId: 'Halfling',
        raceKey: 'Halfling',
        raceName: 'Halfling',
        alternates: [
          alternate({ key: 'Halfling ~ Adaptable Luck', name: 'Adaptable Luck', description: base }),
        ],
      }),
    ],
    adoptiveParentageOptions: [],
    adoptedRaceOptions: [],
    diagnostics: [],
    findings: [],
  };
  const forFeats = (text: string, movedByFeats: boolean, displayValueFeats: string[]) =>
    buildAlternateTraitRows(
      luckMenu,
      'race:halfling',
      [],
      resolution({
        raceId: 'Halfling',
        raceKey: 'Halfling',
        raceName: 'Halfling',
        displayValueFeats,
        renderedTraitDescriptions: [
          { key: 'Halfling ~ Adaptable Luck', name: 'Adaptable Luck', text, droppedArgs: [], movedByFeats },
        ],
      }),
    )[0];

  const withoutFeat = forFeats(base, false, []);
  const withFeat = forFeats(fed, true, ['Fortunate One']);

  assert(withoutFeat.description.includes('Three times per day'), withoutFeat.description);
  assert(withFeat.description.includes('4 times per day'), withFeat.description);
  assert(withoutFeat.description !== withFeat.description, 'same record, different sentence');
  assertEqual(withoutFeat.movedByFeats, false, 'the engine reported no move');
  assertEqual(withFeat.movedByFeats, true, 'and reported one here');
}

main().catch((cause: unknown) => {
  console.error(cause);
  process.exit(1);
});
