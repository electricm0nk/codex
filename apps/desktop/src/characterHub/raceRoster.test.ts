/**
 * The creation race roster must be whatever the backend actually serves,
 * never a list compiled into the frontend.
 *
 * `RACE_OPTIONS` was a seven-entry hand-written table of the Core Rulebook
 * races. The corpus carries 18 (CRB's 7 + Bestiary 1's 11) and
 * `raceCreationCoverage.test.ts` proves — against the same on-disk records —
 * that every one of them supplies every rules-bearing field creation reads.
 * The eleven Bestiary 1 races were ingested, resolvable and browsable in the
 * Race Trait Catalog; no player could make one.
 *
 * This file tests the mapping seam: what arrives on the wire becomes what
 * the picker offers, with nothing invented on the way. The corpus-vs-wire
 * agreement is proved separately and at a different altitude — in Rust, by
 * `character_hub::tests` against the resolver, and in TypeScript by
 * `raceCreationCoverage.test.ts` against the JSON.
 */

import {
  RACE_BODY_PROFILES,
  raceOptionsFromChassis,
  type RaceCreationChassisDto,
} from './raceRoster';
import { UNKNOWN_RACE_TRAIT, deriveRaceTraits } from './characterHubModel';
import { assert, assertEqual } from '../testSupport/asserts';

/** A wire row shaped exactly like `RaceCreationChassisDto` in `character_hub.rs`. */
function chassis(overrides: Partial<RaceCreationChassisDto> = {}): RaceCreationChassisDto {
  return {
    raceId: 'race:goblin',
    label: 'Goblin',
    book: 'B1',
    size: 'Small',
    vision: 'Darkvision 60 ft.',
    baseSpeedFt: 30,
    abilityAdjustments: { dexterity: 4, strength: -2, charisma: -2 },
    floatingBonusPoints: 0,
    ...overrides,
  };
}

/**
 * Nothing is dropped and nothing is added: a served race becomes an offered
 * race carrying its own served values.
 */
function verifiesEveryServedRaceBecomesAnOfferedRaceVerbatim() {
  const options = raceOptionsFromChassis([
    chassis(),
    chassis({ raceId: 'race:human', label: 'Human', book: 'CRB', size: 'Medium', vision: 'Normal', abilityAdjustments: {}, floatingBonusPoints: 2 }),
  ]);

  assertEqual(options.length, 2, 'both served races are offered');
  const [goblin, human] = options;

  assertEqual(goblin.id, 'race:goblin', 'the wire raceId is the option id submitted with the character');
  assertEqual(goblin.label, 'Goblin', 'label');
  assertEqual(goblin.book, 'B1', 'sourcebook code');
  assertEqual(goblin.size, 'Small', 'size');
  assertEqual(goblin.vision, 'Darkvision 60 ft.', 'vision');
  assertEqual(goblin.baseSpeedFt, 30, 'base land speed');
  assertEqual(goblin.floatingBonusPoints, 0, 'floating ability points');
  assertEqual(goblin.abilityAdjustments.dexterity, 4, 'Goblin DEX');
  assertEqual(goblin.abilityAdjustments.strength, -2, 'Goblin STR');
  assertEqual(goblin.abilityAdjustments.charisma, -2, 'Goblin CHA');

  assertEqual(human.floatingBonusPoints, 2, 'Human floating ability points');
  assertEqual(Object.keys(human.abilityAdjustments).length, 0, 'Human has no fixed adjustment');
}

/**
 * Height and weight are the one field the corpus genuinely does not carry,
 * for *any* race — PCGen keeps it in `<race>/<race>_biosettings.lst`, which
 * no book's ingest reads
 * (`verifiesTheCorpusCarriesNoHeightOrWeightProfileForAnyRace`). The seven
 * profiles that ship are hand-entered constants.
 *
 * So a race with no profile must carry `body: null` — an explicit absence
 * the form renders as "no profile", never a borrowed or averaged one. A
 * fabricated body profile would be indistinguishable on screen from a real
 * PF1 one.
 */
function verifiesARaceWithNoHandEnteredBodyProfileCarriesAnExplicitAbsence() {
  const [goblin] = raceOptionsFromChassis([chassis()]);
  assertEqual(goblin.body, null, 'Goblin has no height/weight profile anywhere in this repo');

  const [dwarf] = raceOptionsFromChassis([
    chassis({ raceId: 'race:dwarf', label: 'Dwarf', book: 'CRB', size: 'Medium' }),
  ]);
  assert(dwarf.body !== null, 'Dwarf ships a hand-entered body profile');
  assertEqual(dwarf.body!.male.baseHeightInches, 45, 'Dwarf male base height, unchanged from the shipped table');
}

/**
 * The hand-entered body table is exactly the seven Core Rulebook races and
 * no more. Pinned so that "the corpus has this" can never quietly become
 * true of a race it was invented for.
 */
function verifiesTheHandEnteredBodyTableCoversTheSevenCoreRacesAndOnlyThose() {
  assertEqual(
    Object.keys(RACE_BODY_PROFILES).sort().join(' '),
    'race:dwarf race:elf race:gnome race:half-elf race:half-orc race:halfling race:human',
    'hand-entered body profiles'
  );
}

/**
 * `deriveRaceTraits` backs the Character Sheet's Details panel, captioned
 * "Vision and Size are calculated from race". It must answer for every race
 * the roster it is handed offers — a creatable Goblin whose sheet says
 * "Unknown" is a half-widening — and must still refuse to guess for a race
 * that roster does not carry.
 */
function verifiesTheSheetReadsSizeAndVisionOffTheSameRoster() {
  const roster = raceOptionsFromChassis([
    chassis(),
    chassis({ raceId: 'race:tengu', label: 'Tengu', size: 'Medium', vision: 'Low-light vision' }),
  ]);

  assertEqual(deriveRaceTraits('race:goblin', roster).size, 'Small', 'a Bestiary 1 race reports its real size');
  assertEqual(deriveRaceTraits('race:goblin', roster).vision, 'Darkvision 60 ft.', 'and its real vision');
  assertEqual(deriveRaceTraits('race:tengu', roster).vision, 'Low-light vision', 'Tengu vision');

  // Not in the roster handed in, and not defaulted to Medium/Normal.
  assertEqual(deriveRaceTraits('race:kobold', roster).size, UNKNOWN_RACE_TRAIT, 'an un-rostered race claims no size');
  assertEqual(deriveRaceTraits('race:kobold', roster).vision, UNKNOWN_RACE_TRAIT, 'an un-rostered race claims no vision');
  assertEqual(deriveRaceTraits(null, roster).size, UNKNOWN_RACE_TRAIT, 'a missing raceId claims no size');
  // An empty roster is the still-loading / backend-unavailable state. It
  // must read as "not known yet", not as Medium.
  assertEqual(deriveRaceTraits('race:goblin', []).size, UNKNOWN_RACE_TRAIT, 'an empty roster claims nothing');
}

/**
 * Speed too. The sheet's Speed panel printed a literal `30 ft.` for every
 * character, which is wrong for Dwarf, Gnome and Halfling among the races
 * already shipped, and wrong for Duergar, Svirfneblin and Merfolk (5 ft.)
 * among the new ones.
 */
function verifiesTheSheetReadsLandSpeedOffTheSameRosterToo() {
  const roster = raceOptionsFromChassis([
    chassis({ raceId: 'race:merfolk', label: 'Merfolk', size: 'Medium', vision: 'Low-light vision', baseSpeedFt: 5 }),
  ]);
  assertEqual(deriveRaceTraits('race:merfolk', roster).landSpeed, '5 ft.', "Merfolk's real land speed");
  assertEqual(deriveRaceTraits('race:kobold', roster).landSpeed, UNKNOWN_RACE_TRAIT, 'an un-rostered race claims no speed');
}

function main() {
  verifiesEveryServedRaceBecomesAnOfferedRaceVerbatim();
  verifiesARaceWithNoHandEnteredBodyProfileCarriesAnExplicitAbsence();
  verifiesTheHandEnteredBodyTableCoversTheSevenCoreRacesAndOnlyThose();
  verifiesTheSheetReadsSizeAndVisionOffTheSameRoster();
  verifiesTheSheetReadsLandSpeedOffTheSameRosterToo();
  console.log('raceRoster: all assertions passed');
}

main();
