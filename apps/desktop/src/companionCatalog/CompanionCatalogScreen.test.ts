import {
  BOOK_LABELS,
  SIZE_LABELS,
  STAT_ADJUSTMENT_CAPTION,
  formatAbilityHeading,
  formatBook,
  formatCreatureType,
  formatName,
  formatNaturalAttack,
  formatReach,
  formatServedBooks,
  formatSize,
  formatSpeedClause,
  formatStatAdjustment,
} from './CompanionCatalogScreen';
import type { CompanionCatalogEntryDto } from '../boundary/loadCompanionCatalog';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The companion catalog screen's own presentation rules, pinned against the
 * real shapes `companion_catalog.rs` serves.
 *
 * The ones that matter most are the absences and the near-absences, because
 * each is a real corpus fact a plausible-looking default would erase:
 *
 * 1. `reachFeet === 0` is a genuine "reach 0 ft." — Inner Sea Intrigue's two
 *    Tiny familiars both carry `REACH:0` — and is not the same fact as `null`.
 * 2. `speeds === []` is a row that states no movement, not a row with speed 0.
 * 3. `facet === null` is a row whose `TYPE:` states no facet the chassis
 *    models; three Inner Sea Intrigue rows are in that state and inventing a
 *    label for them would assert something the corpus does not.
 * 4. A `BONUS:STAT` value is an ADJUSTMENT. `STR +6` under a heading reading
 *    "Ability scores" would be a lie about a Griffon's Strength.
 */

/** The book wire codes `companion_catalog.rs` serves, from its own `book_wire_code`. */
const SERVED_BOOK_CODES = ['ISC', 'MC', 'ISI', 'HA'] as const;

/** The `SIZE:` codes the registered creature rows actually carry. */
const SERVED_SIZE_CODES = ['T', 'M', 'L'] as const;

function entry(overrides: Partial<CompanionCatalogEntryDto>): CompanionCatalogEntryDto {
  return {
    key: 'inner_sea_combat:companion:companion_griffon',
    book: 'ISC',
    name: 'Companion (Griffon)',
    size: 'L',
    speeds: [{ mode: 'Walk', feet: 30 }],
    reachFeet: null,
    raceType: 'Magical Beast',
    raceSubtype: null,
    monsterClass: 'Companion:2',
    typeSegments: [],
    naturalAttacks: [],
    statAdjustments: [],
    naturalArmor: null,
    sourcePage: null,
    abilities: [],
    externalAbilityRefs: [],
    ...overrides,
  };
}

function testEveryServedBookHasARealName() {
  for (const code of SERVED_BOOK_CODES) {
    const label = BOOK_LABELS[code];
    assert(Boolean(label), `${code} has no book label; a row would name no book`);
    assert(label !== code, `${code}'s label is the code itself, which names nothing`);
  }
  assertEqual(formatBook('ZZ'), 'ZZ', 'an unmapped code falls through rather than vanishing');
}

function testEverySizeCodeTheRosterCarriesHasALabel() {
  for (const code of SERVED_SIZE_CODES) {
    assert(Boolean(SIZE_LABELS[code]), `${code} has no size label`);
  }
  assertEqual(formatSize('L'), 'Large', 'L reads as Large');
}

function testAnAbsentSizeIsStatedRatherThanRenderedBlank() {
  assertEqual(
    formatSize(null),
    'Size not stated',
    'a row with no SIZE: and no FACT:BaseSize must say so, not render an empty chip'
  );
}

function testAZeroReachIsAValueNotAnAbsence() {
  assertEqual(formatReach(0), 'reach 0 ft.', 'REACH:0 is a real corpus value on the Tiny familiars');
  assertEqual(formatReach(5), 'reach 5 ft.', 'a stated reach reads as a reach');
  assertEqual(formatReach(null), '', 'a row with no REACH: token contributes no clause');
}

function testEveryMovementModeOnTheRowReachesTheClause() {
  assertEqual(
    formatSpeedClause(entry({ speeds: [{ mode: 'Walk', feet: 30 }, { mode: 'Fly', feet: 40 }] })),
    'Walk 30 ft., fly 40 ft.',
    "the Griffon's fly speed is on its row and must reach the screen"
  );
  assertEqual(
    formatSpeedClause(entry({ speeds: [] })),
    'No movement stated on this row',
    'an empty MOVE: is said in words, never printed as 0 ft.'
  );
}

function testASubtypeIsParenthesisedAndAnAbsentOneIsOmitted() {
  assertEqual(
    formatCreatureType('Construct', 'Clockwork'),
    'Construct (Clockwork)',
    'a stated subtype is parenthesised'
  );
  assertEqual(
    formatCreatureType('Magical Beast', null),
    'Magical Beast',
    'a row with no RACESUBTYPE: shows the type alone, never an empty pair of brackets'
  );
}

function testTheCompanionWrapperStaysInTheName() {
  assertEqual(
    formatName(entry({ name: 'Familiar (Clockwork Spy)' })),
    'Familiar (Clockwork Spy)',
    'stripping the wrapper would make a familiar indistinguishable from an animal companion'
  );
}

function testAStatAdjustmentKeepsItsSignAndItsCaption() {
  assertEqual(formatStatAdjustment({ ability: 'STR', amount: 6 }), 'STR +6', 'a positive adjustment is signed');
  assertEqual(formatStatAdjustment({ ability: 'INT', amount: -6 }), 'INT -6', 'a negative adjustment keeps its sign');
  assert(
    STAT_ADJUSTMENT_CAPTION.toLowerCase().includes('adjustment'),
    'the caption must say these are adjustments; under an "ability scores" heading they would be false'
  );
  assert(
    !STAT_ADJUSTMENT_CAPTION.toLowerCase().includes('ability score adjustments (corpus)'),
    'the caption names the corpus token it renders'
  );
}

function testAnAttackWithNoCorpusDicePrintsItsNameAlone() {
  assertEqual(
    formatNaturalAttack({ name: 'Bite', damageDice: null }),
    'Bite',
    'the corpus prices no companion bite; printing a stand-in die would be invented'
  );
  assertEqual(
    formatNaturalAttack({ name: 'Bite', damageDice: '1d6' }),
    'Bite 1d6',
    'a priced attack shows its dice'
  );
}

function testAnAbilityHeadingReadsTheWayTheBookPrintsIt() {
  assertEqual(
    formatAbilityHeading({
      key: 'k',
      name: 'Record Audio',
      facet: 'SpecialQuality',
      delivery: 'Supernatural',
      typeSegments: ['ClockworkSpyRacialAbility', 'SpecialQuality', 'Supernatural'],
      description: null,
      statAdjustments: [],
      sourcePage: null,
    }),
    'Record Audio — SpecialQuality · Supernatural',
    'a modelled facet reads as facet then delivery'
  );
  assertEqual(
    formatAbilityHeading({
      key: 'k',
      name: 'Companion Advancement (Griffon)',
      facet: 'CompanionAdvancement',
      delivery: null,
      typeSegments: ['CompanionAdvancement'],
      description: null,
      statAdjustments: [],
      sourcePage: null,
    }),
    'Companion Advancement (Griffon) — CompanionAdvancement',
    'a row with no delivery segment shows the facet alone, never a trailing separator'
  );
}

function testAnUnmodelledFacetFallsBackToItsVerbatimSegments() {
  assertEqual(
    formatAbilityHeading({
      key: 'k',
      name: 'Potion Installation',
      facet: null,
      delivery: null,
      typeSegments: ['ClockworkFamiliarInstalledItem'],
      description: 'text',
      statAdjustments: [],
      sourcePage: null,
    }),
    'Potion Installation — ClockworkFamiliarInstalledItem',
    'the corpus segment is shown rather than a facet label the row does not state'
  );
  assertEqual(
    formatAbilityHeading({
      key: 'k',
      name: 'Nameless',
      facet: null,
      delivery: null,
      typeSegments: [],
      description: null,
      statAdjustments: [],
      sourcePage: null,
    }),
    'Nameless',
    'a row stating nothing at all shows its name without a dangling dash'
  );
}

function testTheBlurbNamesTheBooksTheResponseActuallyContains() {
  assertEqual(
    formatServedBooks([{ book: 'ISC' }, { book: 'ISC' }, { book: 'MC' }]),
    'Inner Sea Combat and Monster Codex',
    'the blurb is derived from the served rows so it cannot go stale'
  );
  assertEqual(
    formatServedBooks([{ book: 'ISC' }, { book: 'MC' }, { book: 'ISI' }]),
    'Inner Sea Combat, Monster Codex and Inner Sea Intrigue',
    'three books read as a list'
  );
  assertEqual(formatServedBooks([]), 'no book', 'an empty response says so');
}

function main() {
  testEveryServedBookHasARealName();
  testEverySizeCodeTheRosterCarriesHasALabel();
  testAnAbsentSizeIsStatedRatherThanRenderedBlank();
  testAZeroReachIsAValueNotAnAbsence();
  testEveryMovementModeOnTheRowReachesTheClause();
  testASubtypeIsParenthesisedAndAnAbsentOneIsOmitted();
  testTheCompanionWrapperStaysInTheName();
  testAStatAdjustmentKeepsItsSignAndItsCaption();
  testAnAttackWithNoCorpusDicePrintsItsNameAlone();
  testAnAbilityHeadingReadsTheWayTheBookPrintsIt();
  testAnUnmodelledFacetFallsBackToItsVerbatimSegments();
  testTheBlurbNamesTheBooksTheResponseActuallyContains();
}

main();
