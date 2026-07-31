import {
  CLASS_OPTIONS,
  MAX_CLASS_LEVEL,
  RACE_OPTIONS,
  UNKNOWN_RACE_TRAIT,
  canTakeAnotherLevelIn,
  clampLevelForClass,
  deriveRaceTraits,
  describeClassSupportLevel,
  getLevelOptionsForClass,
} from './characterHubModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * `levelOptions` is the app's honest claim about which levels of a class a
 * player can actually build. Its ground truth is
 * `cargo run --bin v06_class_state_dump`, which sweeps every class over
 * levels 1-20 through the real `build_pilot_headless_receipt` pipeline under
 * the exact posture `compose_character_input` produces. The run of
 * 2026-07-29 reports **all 27 classes `Computed` at every level 1-20**
 * (`computed_count: 27`, every row `levels_blocked: []`).
 *
 * These tests pin exactly that. A class either offers the full 1-20 range
 * because the engine computes it there, or it offers only level 1 (the "let
 * the player see the real blocking diagnostics" posture) because the engine
 * computes it nowhere. Today no offered class is in the second group, but
 * the split is still asserted both ways so re-introducing a blocked class
 * cannot silently widen it.
 */

/** The classes the engine dump reports `Computed` at every level 1-20. */
const FULLY_COMPUTED_CLASS_IDS = [
  // CRB
  'class:barbarian',
  'class:bard',
  'class:cleric',
  'class:druid',
  'class:fighter',
  'class:monk',
  'class:paladin',
  'class:ranger',
  'class:rogue',
  'class:sorcerer',
  'class:wizard',
  // APG
  'class:alchemist',
  'class:cavalier',
  'class:inquisitor',
  'class:oracle',
  'class:summoner',
  'class:witch',
  // ACG
  'class:arcanist',
  'class:bloodrager',
  'class:brawler',
  'class:hunter',
  'class:investigator',
  'class:shaman',
  'class:skald',
  'class:slayer',
  'class:swashbuckler',
  'class:warpriest',
];

/**
 * Each class's PF1 hit die, from the `HD:` token on its own real `CLASS:`
 * record — `core_rulebook/cr_classes.lst`, `advanced_players_guide/
 * apg_classes.lst`, `advanced_class_guide/acg_classes.lst`. Cross-checked
 * against the engine's own per-class constants (`crb::class_tables`'s
 * `CLASS_META.hit_die` for the CRB eleven, and the `HIT_DIE` constant in
 * each `rules_tables/{apg,acg}/class_*.rs` for the rest), which now agree
 * everywhere. Monk was the lone exception until 2026-07-29 — see below.
 */
const HIT_DIE_BY_CLASS_ID: Record<string, number> = {
  'class:barbarian': 12,
  'class:bard': 8,
  'class:cleric': 8,
  'class:druid': 8,
  'class:fighter': 10,
  // NOTE: this 8 knowingly departs from the CORPUS (and only the corpus).
  // `cr_classes.lst`'s `CLASS:Monk` line carries `HD:10`, but PF1's
  // published Core Rulebook p.56 — the page that record's own SOURCEPAGE
  // cites — gives the Monk a d8. The operator ruled d8 on 2026-07-29
  // (risks item 91), and `CLASS_META` was corrected from 10 to 8 to match,
  // so this table and the engine now agree; only the corpus dissents, and
  // it is deliberately left unedited because it is the parity oracle.
  // Do not "fix" this back to 10.
  'class:monk': 8,
  'class:paladin': 10,
  'class:ranger': 10,
  'class:rogue': 8,
  'class:sorcerer': 6,
  'class:wizard': 6,
  'class:alchemist': 8,
  'class:cavalier': 10,
  'class:inquisitor': 8,
  'class:oracle': 8,
  'class:summoner': 8,
  'class:witch': 6,
  'class:arcanist': 6,
  'class:bloodrager': 10,
  'class:brawler': 10,
  'class:hunter': 8,
  'class:investigator': 8,
  'class:shaman': 8,
  'class:skald': 8,
  'class:slayer': 10,
  'class:swashbuckler': 10,
  'class:warpriest': 8,
};

/**
 * The Character Sheet prints Vision and Size under the fixed caption
 * "Vision and Size are calculated from race and aren't editable". It used to
 * read them as `race?.size ?? 'Medium'` / `race?.vision ?? 'Normal'` against
 * `RACE_OPTIONS`, which carries only the 7 Core Rulebook races. Any saved
 * character whose `raceId` is outside that list — a clone, a sheet written
 * by a later build, or any race added once SD-27 widens coverage past the
 * CRB 7 — therefore had "Medium" and "Normal" asserted for it as a
 * calculated result. For a kobold or a svirfneblin (both Small, both with
 * darkvision) that is a wrong rules value presented as a derived one.
 *
 * `deriveRaceTraits` must instead say it does not know.
 */
function verifiesKnownRacesStillReportTheirRealSizeAndVision() {
  for (const option of RACE_OPTIONS) {
    const traits = deriveRaceTraits(option.id);
    assertEqual(traits.size, option.size, `${option.label} keeps its real size`);
    assertEqual(traits.vision, option.vision, `${option.label} keeps its real vision`);
  }
}

function verifiesAnUnprofiledRaceIsNotGivenAFabricatedSizeOrVision() {
  // Bestiary 1 races SD-27 brings into the race catalog. Neither is Medium
  // and neither has normal vision, so the old defaults were not merely
  // vague — they were wrong.
  for (const raceId of ['race:kobold', 'race:svirfneblin', 'race:tengu']) {
    const traits = deriveRaceTraits(raceId);
    assertEqual(traits.size, UNKNOWN_RACE_TRAIT, `${raceId} reports an unknown size rather than "Medium"`);
    assertEqual(traits.vision, UNKNOWN_RACE_TRAIT, `${raceId} reports unknown vision rather than "Normal"`);
  }
}

function verifiesAMissingRaceIdIsAlsoUnknownRatherThanDefaulted() {
  for (const raceId of [null, undefined, '']) {
    const traits = deriveRaceTraits(raceId);
    assertEqual(traits.size, UNKNOWN_RACE_TRAIT, 'a missing raceId claims no size');
    assertEqual(traits.vision, UNKNOWN_RACE_TRAIT, 'a missing raceId claims no vision');
  }
}

function verifiesTheUnknownMarkerIsNotItselfARulesValue() {
  const sizes = new Set(RACE_OPTIONS.map((option) => option.size));
  const visions = new Set(RACE_OPTIONS.map((option) => option.vision));
  assert(!sizes.has(UNKNOWN_RACE_TRAIT as never), 'the unknown marker is not a real PF1 size');
  assert(!visions.has(UNKNOWN_RACE_TRAIT), 'the unknown marker is not a real PF1 vision string');
}

async function main() {
  verifiesKnownRacesStillReportTheirRealSizeAndVision();
  verifiesAnUnprofiledRaceIsNotGivenAFabricatedSizeOrVision();
  verifiesAMissingRaceIdIsAlsoUnknownRatherThanDefaulted();
  verifiesTheUnknownMarkerIsNotItselfARulesValue();
  verifiesEveryEngineComputedClassOffersAllTwentyLevels();
  verifiesEveryEngineComputedClassIsActuallyOffered();
  verifiesEveryOfferedClassCarriesItsCorpusHitDie();
  verifiesEngineBlockedClassesStayAtLevelOneOnly();
  verifiesUnknownClassFallsBackToLevelOneOnly();
  verifiesClampLevelForClassKeepsLevelsInsideTheClassRange();
  verifiesCanTakeAnotherLevelInStopsAtTheVerifiedCeiling();
  verifiesSupportLevelCopyPerLevel();
}

function verifiesEveryEngineComputedClassOffersAllTwentyLevels() {
  assertEqual(MAX_CLASS_LEVEL, 20, 'PF1 class ceiling');
  for (const classId of FULLY_COMPUTED_CLASS_IDS) {
    const levels = getLevelOptionsForClass(classId);
    assertEqual(levels.length, 20, `${classId} level option count`);
    assertEqual(levels[0], 1, `${classId} lowest level option`);
    assertEqual(levels[19], 20, `${classId} highest level option`);
    for (let index = 0; index < 20; index += 1) {
      assertEqual(levels[index], index + 1, `${classId} level option ${index}`);
    }
  }
}

/**
 * The gap this whole entry exists to close: a class the engine computes at
 * every level but the picker never names is unreachable by any player. That
 * was true of Arcanist once, and of fifteen APG/ACG classes after it, each
 * of which the dump reported `Computed` 1-20 while no player could select
 * it. So "the engine computes it" and "the app offers it" are asserted as
 * the same set, not merely as overlapping ones.
 */
function verifiesEveryEngineComputedClassIsActuallyOffered() {
  const offeredIds = CLASS_OPTIONS.map((option) => option.id);
  for (const classId of FULLY_COMPUTED_CLASS_IDS) {
    assert(
      offeredIds.includes(classId),
      `${classId} computes at every level but is not offered in the class picker`
    );
  }
  assertEqual(
    offeredIds.length,
    FULLY_COMPUTED_CLASS_IDS.length,
    'the picker must offer exactly the classes the engine dump computes'
  );
  // Ids must be unique -- a duplicated entry would render twice in the
  // dropdown and make `CLASS_OPTIONS.find` ambiguous.
  assertEqual(new Set(offeredIds).size, offeredIds.length, 'class option ids must be unique');
  for (const option of CLASS_OPTIONS) {
    assertEqual(option.supportLevel, 'full', `${option.id} support level`);
    assert(option.label.length > 0, `${option.id} must carry a label`);
  }
}

/**
 * `hitDie` drives the player-facing HP preview and the character sheet's own
 * max-HP figure, so a wrong value is a wrong character, not a cosmetic slip.
 * Pinned against the corpus `HD:` tokens transcribed in `HIT_DIE_BY_CLASS_ID`.
 */
function verifiesEveryOfferedClassCarriesItsCorpusHitDie() {
  for (const option of CLASS_OPTIONS) {
    const expected = HIT_DIE_BY_CLASS_ID[option.id];
    assert(expected !== undefined, `${option.id} has no recorded corpus hit die`);
    assertEqual(option.hitDie, expected, `${option.id} hit die`);
  }
}

function verifiesEngineBlockedClassesStayAtLevelOneOnly() {
  for (const option of CLASS_OPTIONS) {
    if (FULLY_COMPUTED_CLASS_IDS.includes(option.id)) {
      continue;
    }
    const levels = getLevelOptionsForClass(option.id);
    assertEqual(levels.length, 1, `${option.id} level option count`);
    assertEqual(levels[0], 1, `${option.id} level option 0`);
  }
}

function verifiesUnknownClassFallsBackToLevelOneOnly() {
  const levels = getLevelOptionsForClass('class:does-not-exist');
  assertEqual(levels.length, 1, 'unknown class level option count');
  assertEqual(levels[0], 1, 'unknown class level option 0');
}

// Switching the class picker must never leave a level selected that the newly
// chosen class does not actually offer. Every offered class now spans 1-20, so
// switching between any two of them preserves the player's level rather than
// resetting it -- the clamp only bites for an unknown class or an
// out-of-range level. It is kept (and still tested) because it is the guard
// that makes re-introducing a narrower class safe.
function verifiesClampLevelForClassKeepsLevelsInsideTheClassRange() {
  assertEqual(clampLevelForClass('class:fighter', 20), 20, 'fighter keeps level 20');
  assertEqual(clampLevelForClass('class:monk', 20), 20, 'monk now spans the full range');
  assertEqual(clampLevelForClass('class:swashbuckler', 14), 14, 'swashbuckler keeps a mid level');
  assertEqual(clampLevelForClass('class:arcanist', 12), 12, 'arcanist keeps a mid level');
  assertEqual(clampLevelForClass('class:does-not-exist', 7), 1, 'unknown class clamps to 1');
  assertEqual(clampLevelForClass('class:wizard', 0), 1, 'a level below the range clamps up to 1');
}

// The level-up dialog must not offer a class level past what the engine dump
// verified. PF1 itself stops at 20, and the dump sweeps exactly 1-20, so a
// 21st level is not "supported but untested" -- it is unverified reach.
function verifiesCanTakeAnotherLevelInStopsAtTheVerifiedCeiling() {
  assert(canTakeAnotherLevelIn('class:fighter', 0), 'a brand-new Fighter level is available');
  assert(canTakeAnotherLevelIn('class:fighter', 19), 'Fighter 19 -> 20 is available');
  assert(!canTakeAnotherLevelIn('class:fighter', 20), 'Fighter 20 -> 21 must not be offered');
  assert(canTakeAnotherLevelIn('class:monk', 0), 'a first Monk level is available');
  assert(canTakeAnotherLevelIn('class:monk', 1), 'Monk 1 -> 2 is now available');
  assert(!canTakeAnotherLevelIn('class:monk', 20), 'Monk 20 -> 21 must not be offered');
  assert(canTakeAnotherLevelIn('class:arcanist', 5), 'Arcanist 5 -> 6 is available');
  // The ceiling is a property of every offered class, not just Fighter's.
  for (const option of CLASS_OPTIONS) {
    assert(canTakeAnotherLevelIn(option.id, 0), `a first ${option.id} level must be available`);
    assert(canTakeAnotherLevelIn(option.id, 19), `${option.id} 19 -> 20 must be available`);
    assert(!canTakeAnotherLevelIn(option.id, 20), `${option.id} 20 -> 21 must not be offered`);
  }
}

function verifiesSupportLevelCopyPerLevel() {
  assert(describeClassSupportLevel('full', 'Fighter').includes('Fighter'), 'full copy should name the class');
  assert(
    describeClassSupportLevel('partial-human-only', 'Wizard').toLowerCase().includes('human'),
    'partial-human-only copy should mention Human'
  );
  assert(
    describeClassSupportLevel('human-diagnostics-only', 'Monk').toLowerCase().includes('human'),
    'human-diagnostics-only copy should still mention Human (it gets named diagnostics)'
  );
  assert(
    describeClassSupportLevel('human-diagnostics-only', 'Monk').toLowerCase().includes('for any race yet, including human'),
    'human-diagnostics-only copy should be explicit that Human never computes either'
  );
  assert(
    describeClassSupportLevel('none', 'Rogue').toLowerCase().includes("isn't computed"),
    'none copy should say it is not computed'
  );
  assert(
    describeClassSupportLevel('full-except-human-level-1', 'Ranger').toLowerCase().includes('human'),
    'full-except-human-level-1 copy should mention Human'
  );
  assert(
    describeClassSupportLevel('full-except-human-level-1', 'Ranger').toLowerCase().includes('level 1'),
    'full-except-human-level-1 copy should be explicit that the exception is level 1 specifically'
  );
  assert(
    describeClassSupportLevel('headless-only', 'Sorcerer').toLowerCase().includes('sorcerer'),
    'headless-only copy should name the class'
  );
  assert(
    describeClassSupportLevel('headless-only', 'Sorcerer').toLowerCase().includes('picker'),
    'headless-only copy should name the real cause (a missing picker), not a vague "not computed"'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
