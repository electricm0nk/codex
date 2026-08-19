import {
  BOOK_LABELS,
  DAMAGE_DICE_SOURCE_LABELS,
  SIZE_LABELS,
  SIZE_ORDER,
  formatAbilityHeading,
  formatBook,
  formatServedBooks,
  formatChallengeRating,
  formatCreatureType,
  formatLandSpeedClause,
  formatNaturalAttack,
  formatSize,
  formatSpeedClause,
  formatSpellLevel,
  formatSpellLikeAbility,
} from './MonsterCatalogScreen';
import type { MonsterCatalogEntryDto } from '../boundary/loadMonsterCatalog';
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
const SERVED_SIZE_CODES = ['D', 'T', 'S', 'M', 'L', 'H'] as const;

/**
 * The books the catalog serves, as `monster_catalog.rs`'s own wire codes.
 *
 * `BOTD1`/`BOTD2` (SD-29 Epic 5 extend, round 2) are the first codes wider than
 * two characters. They are the books' own `SOURCESHORT` tokens, like every
 * other code here.
 */
// SD-29 Epic 5 extend round 5 added `B3: 'Bestiary 3'` to `BOOK_LABELS` and did
// not add it here, which turned this assertion RED on `origin/tranche/9`
// (`9595bd82`) — `expected …,B2,BB,… got …,B2,B3,BB,…`. Closed by the companion
// lane's round 3 while merging that work, because it is the SAME hand-maintained
// -list-goes-stale defect that round found on the companion side
// (`decisions.md §54.5`), one commit apart in the sibling file: there the list
// was SHORT of the labels and nothing checked three books; here it is short of
// the labels and the gate says so. Same shape, opposite visibility.
// Round 6 added `B4: 'Bestiary 4'` and updated BOTH places in the same edit,
// which is the whole of the fix for the defect described above: this list and
// `BOOK_LABELS` are two hand-maintained copies of one fact, and nothing but
// this assertion couples them. Round 7 added `ISB: 'Inner Sea Bestiary'` the
// same way.
const SERVED_BOOKS = [
  'B1',
  'BB',
  'MC',
  'BOTD1',
  'BOTD2',
  'ISWG',
  'B2',
  'B3',
  'B4',
  'ISB',
  'ISG',
  // SD-29 Epic 5 extend, round 10. Ultimate Psionics, under the `UPSI` code the
  // app already serves this book's equipment and feats with rather than its own
  // `SOURCESHORT:UP` — see `monster_catalog.rs`'s `BOOK_UPSI` and
  // `decisions.md §64.2`.
  //
  // This constant is the TWELFTH registration point for a monster book and the
  // one a Rust-only sweep cannot see: it is a test constant, so it appears in
  // no production registry. Round 10 found it by a red `frontend-test` stage
  // after every Rust stage had passed (`decisions.md §64.4`).
  'UPSI',
] as const;

/** The wire values `NaturalAttackDto.damageDiceSource` can take. */
const SERVED_DICE_SOURCES = [
  'monsterRowToken',
  'corpusCrossReferenceToken',
  'publishedText',
  'notInCorpus',
] as const;

/** A minimal served row, for the formatters that read more than one field. */
function entry(overrides: Partial<MonsterCatalogEntryDto>): MonsterCatalogEntryDto {
  return {
    key: 'bonus_bestiary:monster:allip',
    book: 'BB',
    name: 'Allip',
    challengeRating: 3,
    size: 'M',
    speedFt: 0,
    raceType: 'Undead',
    raceSubtype: 'Incorporeal',
    sourcePage: 'p.4',
    naturalAttacks: [],
    speeds: [],
    monsterClass: 'Undead:4',
    abilities: [],
    externalAbilityRefs: [],
    spellLikeAbilityCasterLevel: null,
    spellLikeAbilities: [],
    ...overrides,
  };
}

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
  // Ultimate Psionics' Psicrystal states `CR:0` (`up_races.lst:47`). Before
  // round 10 this fell into the fraction branch and `Math.round(1 / 0)` printed
  // `CR 1/Infinity` on screen.
  assertEqual(formatChallengeRating(0), 'CR 0', "Psicrystal's genuine CR 0");
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

/**
 * The defect this pins: Allip's only movement is `MOVE:Fly,30`. Rendering the
 * land speed alone would print "No land speed" for a creature the book gives a
 * fly speed to — the same class of erasure as printing "0 ft." for Shark.
 */
function testEveryMovementModeOnTheRowReachesTheClause() {
  assertEqual(
    formatSpeedClause(entry({ speeds: [{ mode: 'Fly', feet: 30 }] })),
    'No land speed, fly 30 ft.',
    "Allip's fly-only row"
  );
  assertEqual(
    formatSpeedClause(
      entry({ speedFt: 30, speeds: [{ mode: 'Walk', feet: 30 }, { mode: 'Burrow', feet: 10 }] })
    ),
    'Speed 30 ft., burrow 10 ft.',
    "the Giant Ant Lion's two modes"
  );
  // A Bestiary 1 row carries no modes at all; it keeps the land-speed clause.
  assertEqual(
    formatSpeedClause(entry({ book: 'B1', speedFt: 30, speeds: [] })),
    'Speed 30 ft.',
    'a Bestiary 1 row falls back to the land-speed clause'
  );
}

function testEveryServedBookHasARealName() {
  assertEqual(
    Object.keys(BOOK_LABELS).sort().join(','),
    [...SERVED_BOOKS].sort().join(','),
    'BOOK_LABELS names exactly the served books'
  );
  assertEqual(formatBook('BB'), 'Bonus Bestiary', 'the wire code is never what a reader sees');
  assertEqual(formatBook('MC'), 'Monster Codex', 'the wire code is never what a reader sees');
  assertEqual(
    formatBook('BOTD1'),
    'Book of the Damned, Volume 1',
    'a wire code wider than two characters maps like any other'
  );
  assertEqual(
    formatBook('ISWG'),
    'Inner Sea World Guide',
    'the round-3 book, whose four-character code is mapped like any other'
  );
  assertEqual(
    formatBook('B2'),
    'Bestiary 2',
    'the round-4 book, which serves more monsters than every other book here combined'
  );
  assertEqual(formatBook('ZZ'), 'ZZ', 'an unserved code falls through as itself');
}

/**
 * The blurb above the catalog names its books. It used to name them in a
 * hand-written sentence ("across Bestiary 1 and Bonus Bestiary"), which was
 * already wrong the moment a third book was ingested — stale prose on a screen
 * a player reads, pinned by nothing. `formatServedBooks` derives the list from
 * the served rows instead, so this test is about the derivation, not the words.
 */
function testTheBlurbNamesTheBooksTheResponseActuallyContains() {
  assertEqual(formatServedBooks([]), 'no book', 'an empty response names no book');
  assertEqual(
    formatServedBooks([{ book: 'B1' }, { book: 'B1' }]),
    'Bestiary 1',
    'one book is named without a conjunction'
  );
  assertEqual(
    formatServedBooks([{ book: 'B1' }, { book: 'BB' }, { book: 'B1' }, { book: 'MC' }]),
    'Bestiary 1, Bonus Bestiary and Monster Codex',
    'every served book is named once, in first-appearance order'
  );
  assertEqual(
    formatServedBooks([{ book: 'B1' }, { book: 'BB' }]),
    'Bestiary 1 and Bonus Bestiary',
    'a book that stops being served stops being named'
  );
}

/**
 * `null` dice are not `'0'` dice. `'0'` is Cave Fisher's Filament — a real
 * attack that deals no damage. `null` is an attack whose damage the corpus
 * never states, which is 13 of Bonus Bestiary's 15 named attacks.
 */
function testAnAttackWithNoCorpusDicePrintsItsNameAlone() {
  assertEqual(
    formatNaturalAttack({
      name: 'Bite',
      damageDice: null,
      damageDiceSource: 'notInCorpus',
      groundingNote: 'no dice at any hop',
    }),
    'Bite',
    'the name alone, never a stand-in value'
  );
}

function testAnAbilityHeadingReadsTheWayTheBookPrintsIt() {
  assertEqual(
    formatAbilityHeading({
      key: 'bonus_bestiary:monster_ability:babble',
      name: 'Babble',
      facet: 'SpecialAttack',
      delivery: 'Supernatural',
      description: 'An allip constantly mutters...',
      sourcePage: 'p.4',
    }),
    'Babble — Special Attack (Su)',
    'facet and delivery both reach the reader'
  );
  assertEqual(
    formatAbilityHeading({
      key: 'bonus_bestiary:monster_ability:statue',
      name: 'Statue',
      facet: 'SpecialQuality',
      delivery: null,
      description: null,
      sourcePage: null,
    }),
    'Statue — Special Quality',
    'a row that states no delivery gets no invented one'
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

/**
 * The real Bestiary 1 Astral Deva grant, straight off `b1_races.lst`:
 * `SPELLS:Innate|TIMES=1|CASTERLEVEL=13|Blade Barrier,16+CHA|Heal`. Blade
 * barrier is a 6th-level spell, which is what `16 - 10` states and what
 * `cr_spells.lst`'s own `CLASSES:Cleric=6` independently confirms.
 */
function testASpellLikeAbilityReadsTheWayTheBookPrintsIt() {
  assertEqual(
    formatSpellLikeAbility({
      spell: 'Blade Barrier',
      times: '1',
      timeUnit: null,
      casterLevelToken: '13',
      saveDcToken: '16+CHA',
      derivedSpellLevel: 6,
      saveDcAbility: 'CHA',
    }),
    '1/day — blade barrier (6th, DC 16 + Cha)',
    'a granted spell states its frequency, its derived level and its DC formula'
  );
  assertEqual(
    formatSpellLikeAbility({
      spell: 'Continual Flame',
      times: 'ATWILL',
      timeUnit: null,
      casterLevelToken: '13',
      saveDcToken: null,
      derivedSpellLevel: null,
      saveDcAbility: null,
    }),
    'At will — continual flame',
    'a spell the row states no save for prints no DC clause at all, rather than a placeholder'
  );
  assertEqual(
    formatSpellLikeAbility({
      spell: 'Commune',
      times: '1',
      timeUnit: 'Week',
      casterLevelToken: '12',
      saveDcToken: null,
      derivedSpellLevel: null,
      saveDcAbility: null,
    }),
    '1/week — commune',
    "a row's own TIMEUNIT is honoured rather than assuming the per-day default"
  );
}

/**
 * THE ANTI-FABRICATION RULE, pinned as a test: a monster's ability SCORES are
 * not a corpus-stated fact in this repo, so the save DC must reach the screen
 * as the formula the row states and never as a resolved number. If this ever
 * renders a bare integer where an ability term belongs, a player is reading a
 * number nothing computed.
 */
function testTheSaveDcIsShownAsAFormulaAndNeverResolvedToANumber() {
  const rendered = formatSpellLikeAbility({
    spell: 'Holy Smite',
    times: 'ATWILL',
    timeUnit: null,
    casterLevelToken: '(max(TL-1,1))',
    saveDcToken: '14+CHA',
    derivedSpellLevel: 4,
    saveDcAbility: 'CHA',
  });
  assert(
    rendered.includes('DC 14 + Cha'),
    'the DC keeps its ability term, so the reader can see it is a formula'
  );
  assert(
    !/DC \d+\)/.test(rendered),
    'a DC stated with an ability term must never render as a bare resolved number'
  );
}

/** The engine's own refusals must stay legible, not become a rendered zero. */
function testAnUnreadableDcTokenStillRendersRatherThanVanishing() {
  assertEqual(
    formatSpellLikeAbility({
      spell: 'Suggestion',
      times: '3',
      timeUnit: null,
      casterLevelToken: 'SLA_CL',
      saveDcToken: '15',
      derivedSpellLevel: null,
      saveDcAbility: null,
    }),
    '3/day — suggestion (DC 15)',
    'a DC token the engine refuses to derive a level from still shows the number the row states'
  );
}

function testSpellLevelOrdinalsReadAsThePlayerWouldWriteThem() {
  const cases: [number, string][] = [
    [0, 'cantrip'],
    [1, '1st'],
    [2, '2nd'],
    [3, '3rd'],
    [4, '4th'],
    [9, '9th'],
    [11, '11th'],
    [12, '12th'],
    [13, '13th'],
    [21, '21st'],
  ];
  for (const [level, expected] of cases) {
    assertEqual(formatSpellLevel(level), expected, `spell level ${level} reads as ${expected}`);
  }
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
  testEveryMovementModeOnTheRowReachesTheClause();
  testEveryServedBookHasARealName();
testTheBlurbNamesTheBooksTheResponseActuallyContains();
  testAnAttackWithNoCorpusDicePrintsItsNameAlone();
  testAnAbilityHeadingReadsTheWayTheBookPrintsIt();
  testASpellLikeAbilityReadsTheWayTheBookPrintsIt();
  testTheSaveDcIsShownAsAFormulaAndNeverResolvedToANumber();
  testAnUnreadableDcTokenStillRendersRatherThanVanishing();
  testSpellLevelOrdinalsReadAsThePlayerWouldWriteThem();
}

main();
