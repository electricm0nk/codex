import { buildRacialTraitsSurface } from './racialTraitsModel';
import type { RaceSelectionResponse } from '../boundary/loadAlternateRacialTraits';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * Projecting `load_saved_character`'s `resolvedRacialTraits` payload onto the
 * character sheet.
 *
 * Every payload below is shaped exactly like real `race_trait_picker.rs`
 * output, and the two `Halfling ~ Adaptable Luck` sentences are the engine's
 * own — the racial base and the rendering for a halfling holding ARG's
 * `Fortunate One`, verbatim from
 * `character_hub::tests::a_loaded_characters_racial_trait_prose_states_the_number_its_own_feats_produce`.
 *
 * **This module decides no rules.** It does not choose a number, does not
 * decide which trait applies, and does not compose a sentence: the engine
 * rendered all three and this only groups them for the screen. The tests below
 * are therefore all of the form "what the engine said arrives intact, and what
 * it did not say is never invented".
 */

const BASE_LUCK =
  'Some halflings have greater control over their innate luck. Three times per day, a halfling can ' +
  'gain a +2 luck bonus on an ability check, attack roll, saving throw, or skill check. If halflings ' +
  'choose to use the ability before they make the roll or check, they gain the full +2 bonus; if they ' +
  'choose to do so afterward, they only gain a +1 bonus.';

const FED_LUCK =
  'Some halflings have greater control over their innate luck. 4 times per day, a halfling can ' +
  'gain a +2 luck bonus on an ability check, attack roll, saving throw, or skill check. If halflings ' +
  'choose to use the ability before they make the roll or check, they gain the full +2 bonus; if they ' +
  'choose to do so afterward, they only gain a +1 bonus.';

const KEEN_SENSES = 'Halflings receive a +2 racial bonus on Perception skill checks.';

function resolution(overrides: Partial<RaceSelectionResponse> = {}): RaceSelectionResponse {
  return {
    raceId: 'Halfling',
    raceKey: 'Halfling',
    raceName: 'Halfling',
    book: 'CRB',
    appliedTraits: [
      { key: 'Halfling ~ Keen Senses', name: 'Keen Senses', book: 'CRB', role: 'default', description: KEEN_SENSES },
      {
        key: 'Halfling ~ Adaptable Luck',
        name: 'Adaptable Luck',
        book: 'ARG',
        role: 'alternate',
        description: BASE_LUCK,
      },
    ],
    suppressions: [
      {
        suppressedTraitKey: 'Halfling ~ Halfling Luck',
        suppressedTraitName: 'Halfling Luck',
        flag: 'Halfling_ReplaceHalflingLuck',
        setByTraitKey: 'Halfling ~ Adaptable Luck',
        setByTraitName: 'Adaptable Luck',
      },
    ],
    firedFlags: ['Halfling_ReplaceHalflingLuck'],
    inertFlags: [],
    unmatchedSelections: [],
    blockedAlternates: [],
    conflictingSelections: [],
    renderedTraitDescriptions: [
      {
        key: 'Halfling ~ Keen Senses',
        name: 'Keen Senses',
        text: KEEN_SENSES,
        droppedArgs: [],
        movedByFeats: false,
      },
      {
        key: 'Halfling ~ Adaptable Luck',
        name: 'Adaptable Luck',
        text: BASE_LUCK,
        droppedArgs: [],
        movedByFeats: false,
      },
    ],
    displayValueFeats: [],
    errors: [],
    ...overrides,
  };
}

/** The same payload as a character holding `Fortunate One` receives. */
function withLuckFeat(): RaceSelectionResponse {
  const base = resolution();
  return {
    ...base,
    appliedTraits: base.appliedTraits.map((applied) =>
      applied.key === 'Halfling ~ Adaptable Luck' ? { ...applied, description: FED_LUCK } : applied,
    ),
    renderedTraitDescriptions: base.renderedTraitDescriptions.map((row) =>
      row.key === 'Halfling ~ Adaptable Luck' ? { ...row, text: FED_LUCK, movedByFeats: true } : row,
    ),
    displayValueFeats: ['Fortunate One'],
  };
}

async function main() {
  everyAppliedTraitReachesTheSheetWithItsProse();
  theSameRecordShowsADifferentSentenceForACharacterHoldingTheFeat();
  namesWhatEachAlternateReplaced();
  reportsAnUnresolvableRaceRatherThanShowingAnEmptyTraitList();
  reportsAnIncompleteDescriptionRatherThanPresentingItAsWhole();
  console.log('racialTraitsModel: all assertions passed');
}

/**
 * The gap this module closes: the sheet used to render one name-only card per
 * chosen trait key. A card with a name and no prose passes any "the key
 * survived" test, so every assertion here is about the *text*.
 */
function everyAppliedTraitReachesTheSheetWithItsProse() {
  const surface = buildRacialTraitsSurface(resolution());

  assertEqual(surface.unavailableReason, null, 'a clean resolution is available');
  assertEqual(surface.rows.length, 2, 'one row per applied trait');
  assertEqual(surface.rows[0].name, 'Keen Senses', "the resolver's own order is kept");
  for (const row of surface.rows) {
    assert(row.text.trim().length > 0, `${row.key} carries prose`);
    assert(!row.text.includes('%1'), `${row.key} must not leak an unrendered DESC argument`);
  }
  // The book and the role are the resolver's classification, passed through.
  assertEqual(surface.rows[1].book, 'ARG', "the alternate's own book");
  assertEqual(surface.rows[1].role, 'alternate', 'the resolver classified it, not this module');
  assertEqual(surface.rows[1].roleLabel, 'Alternate racial trait', 'a readable label for the same fact');
  assertEqual(surface.rows[0].roleLabel, 'Racial trait', 'defaults read as plain racial traits');
}

/**
 * **The proof, at the surface a player lives in.** One corpus record, two
 * characters, two sentences — and the difference is stated as the engine's own
 * `movedByFeats` / `displayValueFeats`, never as a claim this module composed.
 */
function theSameRecordShowsADifferentSentenceForACharacterHoldingTheFeat() {
  const base = buildRacialTraitsSurface(resolution());
  const fed = buildRacialTraitsSurface(withLuckFeat());

  const baseLuck = base.rows.find((row) => row.key === 'Halfling ~ Adaptable Luck');
  const fedLuck = fed.rows.find((row) => row.key === 'Halfling ~ Adaptable Luck');
  assert(baseLuck !== undefined && fedLuck !== undefined, 'both surfaces carry the record');
  assert(baseLuck!.text.includes('Three times per day'), `racial base: ${baseLuck!.text}`);
  assert(fedLuck!.text.includes('4 times per day'), `with the feat: ${fedLuck!.text}`);
  assert(baseLuck!.text !== fedLuck!.text, 'same record, different sentence');

  assertEqual(baseLuck!.movedByFeats, false, 'nothing moved for a character holding no such feat');
  assertEqual(fedLuck!.movedByFeats, true, 'the engine reported the move');
  assertEqual(base.displayValueFeats.length, 0, 'no feat is credited when none moved a number');
  assertEqual(fed.displayValueFeats[0], 'Fortunate One', 'the screen can say why the number differs');
}

/** The swap itself, in the resolver's words rather than re-derived here. */
function namesWhatEachAlternateReplaced() {
  const surface = buildRacialTraitsSurface(resolution());

  assertEqual(surface.replaced.length, 1, 'one suppression fired');
  assertEqual(surface.replaced[0].name, 'Halfling Luck', 'the trait that stopped applying');
  assertEqual(surface.replaced[0].byName, 'Adaptable Luck', 'and what replaced it');

  const luck = surface.rows.find((row) => row.key === 'Halfling ~ Adaptable Luck');
  assertEqual(luck!.replaces[0], 'Halfling Luck', 'the row itself names what it swapped out');
  const keen = surface.rows.find((row) => row.key === 'Halfling ~ Keen Senses');
  assertEqual(keen!.replaces.length, 0, 'a trait that replaced nothing claims nothing');
}

/**
 * An unresolvable race must say so. An empty trait list rendered silently
 * would read as "this race has no racial traits", which is a different and
 * false claim.
 */
function reportsAnUnresolvableRaceRatherThanShowingAnEmptyTraitList() {
  const missing = buildRacialTraitsSurface(null);
  assertEqual(missing.rows.length, 0, 'nothing to show');
  assert(missing.unavailableReason !== null, 'and it says so');

  const errored = buildRacialTraitsSurface(
    resolution({ appliedTraits: [], errors: ['no race in the loaded corpus matches "Balor"'] }),
  );
  assertEqual(errored.rows.length, 0, 'no rows');
  assertEqual(
    errored.unavailableReason,
    'no race in the loaded corpus matches "Balor"',
    "the engine's own words, not a paraphrase",
  );
}

/**
 * A description the engine could only partly resolve is shown as partly
 * resolved. Rendering it as though it were whole is the same
 * missing-number defect one step smaller.
 */
function reportsAnIncompleteDescriptionRatherThanPresentingItAsWhole() {
  const surface = buildRacialTraitsSurface(
    resolution({
      renderedTraitDescriptions: resolution().renderedTraitDescriptions.map((row) =>
        row.key === 'Halfling ~ Keen Senses' ? { ...row, droppedArgs: ['Halfling_KeenSenses_Bonus'] } : row,
      ),
    }),
  );

  const keen = surface.rows.find((row) => row.key === 'Halfling ~ Keen Senses');
  assertEqual(keen!.droppedArgs[0], 'Halfling_KeenSenses_Bonus', 'carried per row');
  assertEqual(surface.incompleteRows.length, 1, 'and summarised for the screen');
  assertEqual(surface.incompleteRows[0].name, 'Keen Senses', 'named by the trait, not the id');
}

main().catch((cause: unknown) => {
  console.error(cause);
  process.exit(1);
});
