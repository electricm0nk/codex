import {
  BOOK_LABELS,
  SIZE_LABELS,
  DAMAGE_BONUS_CAPTION,
  SKILL_BONUS_CAPTION,
  STAT_ADJUSTMENT_CAPTION,
  formatAbilityHeading,
  formatBook,
  formatCreatureType,
  formatDamageBonus,
  formatName,
  formatNaturalAttack,
  formatReach,
  formatServedBooks,
  formatSize,
  formatSkillBonus,
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
// Every wire code `companion_catalog::book_wire_code` can emit, i.e. one per
// row of `companion_chassis::COMPANION_BOOKS`. Hand-maintained on this side of
// the wire, so it goes stale silently: round 2 registered B5/B6/B2 and left
// this list at round 1's four, which is why B5's, B6's and B2's labels were
// never checked by anything (`decisions.md §54.5`). Corrected here to the
// eight the backend serves, B1 included.
const SERVED_BOOK_CODES = [
  'ISC',
  'MC',
  'ISI',
  'HA',
  'B5',
  'B6',
  'B2',
  'B1',
  'B3',
  'B4',
  'UW',
  'CE',
  // SD-29 Epic 7 round 8 (`decisions.md §65`). Core Rulebook's 38 companion
  // creatures. This roster is what makes `testEveryServedBookHasARealName`
  // mean anything: the label map gained `CRB` and this list did not, so the
  // whole frontend suite stayed green (99/99) while the running app rendered
  // the bare code `CRB` beside twelve spelled-out book names. The on-screen
  // item-8 check caught it; no test could, because the test only asks about
  // codes named here.
  'CRB',
] as const;

/**
 * The `SIZE:` codes the registered creature rows actually carry.
 *
 * Widened for Ultimate Wilderness (round 6), whose 169 creature rows are the
 * first to carry `D` and `S`. Derived, not guessed:
 * `grep -o 'size: Some("[A-Z]")' src/rules_core/rules_tables/ultimate_wilderness/companion_data.rs
 *  | sort | uniq -c` -> D 28, L 2, M 24, S 48, T 67.
 */
const SERVED_SIZE_CODES = ['D', 'T', 'S', 'M', 'L'] as const;

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
    naturalAttackDamageBonuses: [],
    skillAbilityDiffBonuses: [],
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
      descriptionVariants: [],
      statAdjustments: [],
      saveDcFormulas: [],
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
      descriptionVariants: [],
      statAdjustments: [],
      saveDcFormulas: [],
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
      descriptionVariants: [],
      statAdjustments: [],
      saveDcFormulas: [],
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
      descriptionVariants: [],
      statAdjustments: [],
      saveDcFormulas: [],
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

/**
 * The damage-bonus block. The corpus states a RULE here, not a number, and the
 * two shapes the screen must keep apart are "the engine read the formula" and
 * "the engine refused to interpret it" — collapsing them would tell the player
 * a formula was understood when it was not.
 */
function testADamageBonusPrintsTheRuleAndAnUnparsedOneSaysSo() {
  assertEqual(
    formatDamageBonus({
      attack: 'Bite',
      bonus: '+1/2 Str modifier (minimum +0)',
      unparsedFormula: null,
    }),
    'Bite +1/2 Str modifier (minimum +0)',
    'a parsed token prints the rule the corpus states'
  );
  assertEqual(
    formatDamageBonus({ attack: 'Tail', bonus: 'STR/2', unparsedFormula: 'STR/2' }),
    'Tail STR/2 (formula not interpreted)',
    'a refused formula prints verbatim AND says it was not interpreted'
  );
  assert(
    DAMAGE_BONUS_CAPTION.includes('BONUS:WEAPONPROF'),
    'the caption names the corpus token so the reader knows it is a corpus fact'
  );
}

/**
 * The skill-bonus block's twin of the damage-bonus test above: the same
 * parsed-vs-refused distinction, and the skills list joined into the label.
 */
function testASkillBonusPrintsTheRuleAndAnUnparsedOneSaysSo() {
  assertEqual(
    formatSkillBonus({
      skills: ['Climb', 'Swim'],
      bonus: 'Dex modifier − Str modifier',
      unparsedFormula: null,
    }),
    'Climb, Swim: Dex modifier − Str modifier',
    'a parsed token prints the rule the corpus states, with every named skill'
  );
  assertEqual(
    formatSkillBonus({ skills: ['Climb'], bonus: 'CON-INT-WIS', unparsedFormula: 'CON-INT-WIS' }),
    'Climb: CON-INT-WIS (formula not interpreted)',
    'a refused formula prints verbatim AND says it was not interpreted'
  );
  assert(
    SKILL_BONUS_CAPTION.includes('BONUS:SKILL'),
    'the caption names the corpus token so the reader knows it is a corpus fact'
  );
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
  testADamageBonusPrintsTheRuleAndAnUnparsedOneSaysSo();
  testASkillBonusPrintsTheRuleAndAnUnparsedOneSaysSo();
  testAnAbilityHeadingReadsTheWayTheBookPrintsIt();
  testAnUnmodelledFacetFallsBackToItsVerbatimSegments();
  testTheBlurbNamesTheBooksTheResponseActuallyContains();
}

main();
