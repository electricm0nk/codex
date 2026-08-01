import {
  DAMAGE_DICE_SOURCE_LABELS,
  SIZE_LABELS,
  SIZE_ORDER,
  formatChallengeRating,
  formatCreatureType,
  formatLandSpeedClause,
  formatNaturalAttack,
  formatSize,
} from './MonsterCatalogScreen';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The monster catalog screen's own presentation rules, pinned against the real
 * shapes `monster_catalog.rs` serves.
 *
 * The three that matter most are the absences, because each one is a real
 * corpus fact that a plausible-looking default would erase:
 *
 * 1. `speedFt === 0` is a genuine "Speed 0 ft." stat line (Shark, Squid,
 *    Vargouille carry no `Walk` pair at all), not a missing number.
 * 2. `raceSubtype === null` is a row with no `RACESUBTYPE:` token.
 * 3. `damageDice === '0'` is a real attack that deals no damage — Cave
 *    Fisher's Filament, whose corpus token ends `,*1,0`.
 */

/** The `SIZE:` codes the 41 served records actually use, derived from `data/corpus/beastiary/monster/`. */
const SERVED_SIZE_CODES = ['D', 'T', 'S', 'M', 'L'] as const;

/** The wire values `NaturalAttackDto.damageDiceSource` can take. */
const SERVED_DICE_SOURCES = [
  'monsterRowToken',
  'corpusCrossReferenceToken',
  'publishedText',
] as const;

function testSizeOrderCoversEverySizeCodeTheAdapterServes() {
  assertEqual(
    [...SIZE_ORDER].sort().join(','),
    [...SERVED_SIZE_CODES].sort().join(','),
    'SIZE_ORDER matches the served size codes'
  );
}

function testEverySizeCodeHasARealDisplayLabel() {
  for (const code of SIZE_ORDER) {
    const label = SIZE_LABELS[code];
    assert(
      typeof label === 'string' && label.length > 0,
      `size ${code} has a display label (a bare PCGen code names nothing)`
    );
    assert(label !== code, `size ${code}'s label is a real word, not the wire code`);
  }
}

function testSizeLabelsDefineNoCodeTheCatalogDoesNotServe() {
  assertEqual(
    Object.keys(SIZE_LABELS).sort().join(','),
    [...SERVED_SIZE_CODES].sort().join(','),
    'SIZE_LABELS defines exactly the served size codes'
  );
}

function testFormatSizeFallsBackToTheCodeRatherThanInventingALabel() {
  assertEqual(formatSize('M'), 'Medium', 'a known code');
  assertEqual(formatSize('G'), 'G', 'an unserved code falls through as itself');
}

function testChallengeRatingReadsAsTheBookPrintsIt() {
  assertEqual(formatChallengeRating(1), 'CR 1', 'CR 1');
  assertEqual(formatChallengeRating(3), 'CR 3', 'CR 3');
  // Not in the current roster, but the ingest's own type is `f32` and the
  // sub-CR-1 rows are real Bestiary 1 content: 0.5 must never print as "CR 0.5".
  assertEqual(formatChallengeRating(0.5), 'CR 1/2', 'a fractional rating');
  assertEqual(formatChallengeRating(1 / 3), 'CR 1/3', 'a third');
}

function testALandSpeedOfZeroIsStatedRatherThanPrintedAsZero() {
  assertEqual(formatLandSpeedClause(30), 'Speed 30 ft.', 'a real land speed');
  // The whole clause, not a fragment: the first version returned `'no land
  // speed'` for the caller to prefix with "Speed", and the Shark row reached
  // the screen reading "Speed no land speed".
  assertEqual(formatLandSpeedClause(0), 'No land speed', "Shark's genuine Speed 0 ft.");
}

function testAnAbsentSubtypeIsOmittedRatherThanFilledIn() {
  assertEqual(formatCreatureType('Humanoid', 'Gnoll'), 'Humanoid (Gnoll)', 'a real subtype');
  assertEqual(formatCreatureType('Undead', null), 'Undead', 'no RACESUBTYPE: token on the row');
}

function testAZeroDamageAttackSaysSoRatherThanPrintingZeroDice() {
  assertEqual(
    formatNaturalAttack({
      name: 'Bite',
      damageDice: '2d6',
      damageDiceSource: 'monsterRowToken',
      groundingNote: null,
    }),
    'Bite 2d6',
    'a normal attack'
  );
  assertEqual(
    formatNaturalAttack({
      name: 'Filament',
      damageDice: '0',
      damageDiceSource: 'monsterRowToken',
      groundingNote: null,
    }),
    'Filament (no damage)',
    "Cave Fisher's Filament, whose corpus token ends `,*1,0`"
  );
}

function testEveryDiceProvenanceTheAdapterCanServeHasALabel() {
  assertEqual(
    Object.keys(DAMAGE_DICE_SOURCE_LABELS).sort().join(','),
    [...SERVED_DICE_SOURCES].sort().join(','),
    'DAMAGE_DICE_SOURCE_LABELS covers exactly the served provenance values'
  );
  const labels = Object.values(DAMAGE_DICE_SOURCE_LABELS);
  assertEqual(
    new Set(labels).size,
    labels.length,
    'each provenance reads differently — collapsing them would hide which dice are grounded'
  );
}

function main() {
  testSizeOrderCoversEverySizeCodeTheAdapterServes();
  testEverySizeCodeHasARealDisplayLabel();
  testSizeLabelsDefineNoCodeTheCatalogDoesNotServe();
  testFormatSizeFallsBackToTheCodeRatherThanInventingALabel();
  testChallengeRatingReadsAsTheBookPrintsIt();
  testALandSpeedOfZeroIsStatedRatherThanPrintedAsZero();
  testAnAbsentSubtypeIsOmittedRatherThanFilledIn();
  testAZeroDamageAttackSaysSoRatherThanPrintingZeroDice();
  testEveryDiceProvenanceTheAdapterCanServeHasALabel();
}

main();
