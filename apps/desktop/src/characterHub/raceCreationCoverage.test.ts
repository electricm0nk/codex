/**
 * Does the ingested race content actually carry everything character
 * creation needs, for all 18 races the Race Trait Catalog already browses?
 *
 * # Why this file exists
 *
 * Creation offered 7 races from a hardcoded `RACE_OPTIONS` table in
 * `characterHubModel.ts`. The corpus carries 18 (Core Rulebook's 7 +
 * Bestiary 1's 11). A previous assessment said widening creation "needs
 * per-race data nobody has". This file tested that claim field by field
 * instead of inheriting it, against real on-disk records — not a fixture —
 * and found the claim false for every field except height/weight.
 *
 * **The table is now gone.** `list_race_creation_roster`
 * (`character_hub.rs`) derives all 18 from these records, so there is no
 * hand-maintained mirror left to drift. What this file still pins is the
 * derivation: the identical shape one layer down
 * (`rules_tables/crb/race_tables.rs`) silently drifted from the source on
 * four races' ability modifiers for months, because one source row states
 * two ability grants at once and a hand transcription read only the first.
 *
 * # Where the rules-bearing values come from — SD-35 `AT-35-E6-003`
 *
 * They come from **`data/sheet_rules/`**, the converted package, and from
 * nowhere else. `decisions.md §11`: nothing on the live side may read the
 * ingest format's verbatim token arrays, and `apps/desktop/**` is the live
 * side in full — this file included, because `pcgen_residue_gate.py` scans
 * it. Until this cycle the four derivations below walked each corpus
 * record's own token arrays and re-implemented, in TypeScript, the parse the
 * converter already performs at ingest. Now each reads the converted
 * `SheetRule`'s own typed fields:
 *
 * | value | read from |
 * |---|---|
 * | racial ability adjustments | the rule's `target.Ability` + `value.Number.Const` |
 * | floating "+2 to one score" pool | `target.Pool === 'ability_bonus'`, magnitude from the rule's own `label` |
 * | effective size | the `Racial Size` rule's own `label` |
 * | vision | the rule's `prose` segment whose family is the `Senses` stat block |
 *
 * Each value is identical to what this file derived before the swap — the
 * 18 races, four fields, checked in both forms during `AT-35-E6-003` cycle
 * 10 — so this is a change of *source*, not of expectation.
 *
 * The corpus records are still read, for **identity and classification
 * only**: a record's `key`, `name`, `race_key`, `type_tokens`,
 * `is_racial_default` and the chassis' `base_size`. Those are our own
 * product fields, not the ingest's verbatim token arrays, and the counts
 * below are the population this file has always asserted.
 *
 * # What creation actually consumes
 *
 * Traced end to end (`CreateCharacterForm` → `composeCreateCharacterRequest`
 * → the `create_character` command → `compose_character_input`):
 *
 * | `RaceOption` field | consumed by | available? |
 * |---|---|---|
 * | `id` / `label` | the picker, and `raceId` on the wire | yes |
 * | `abilityAdjustments` | `applyRacialAbilityAdjustments`, **baked into the submitted scores** | yes |
 * | `floatingBonusPoints` | the point-allocation control + `abilityBonusTarget` | yes (see below) |
 * | `size` | `deriveRaceTraits` → the sheet's Details panel | yes (see below) |
 * | `vision` | `deriveRaceTraits` → the sheet's Details panel | yes |
 * | `body` | the height/weight roller in the form | **no** |
 *
 * The racial ability adjustment is applied *client-side* — the backend
 * receives already-adjusted scores and applies no racial adjustment of its
 * own (Human's floating +2 via `abilityBonusTarget` is the sole exception).
 * So `abilityAdjustments` is not cosmetic: a wrong value here is a wrong
 * character, silently.
 *
 * `body` is genuinely absent: height and weight live in PCGen's own
 * per-race bio settings, which this project has not ingested for any book,
 * and the converted schema has no field that could hold them.
 * `verifiesNoConvertedRaceRuleStatesAHeightOrWeightProfile` pins that
 * absence so it stays a checked fact. It is also the one field creation
 * does not depend on — height and weight are rolled for display in the form
 * and are not part of `CreateCharacterRequest`.
 */

import { readFileSync, readdirSync, existsSync } from 'node:fs';
import { join, dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { ABILITY_KEYS } from './characterHubModel';
import type { AbilityKey } from './characterHubModel';
import { RACE_BODY_PROFILES } from './raceRoster';
import { assert, assertEqual } from '../testSupport/asserts';

/** `apps/desktop/src/characterHub/` → the repo root. */
const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../../../..');
const CORPUS_ROOT = join(REPO_ROOT, 'data/corpus');
const SHEET_RULES_ROOT = join(REPO_ROOT, 'data/sheet_rules');

/**
 * The books that carry race content, and the wire code each race is
 * attributed to. Mirrors `race_catalog.rs`'s own `RACE_CORPUS_BOOKS`.
 * `advanced_race_guide` is loaded but declares zero racial *defaults* in
 * Core Rulebook/Bestiary terms (asserted below), so it contributes nothing
 * to a default build of those 18.
 */
const RACE_BOOKS = ['core_rulebook', 'beastiary', 'advanced_race_guide'] as const;

/**
 * `beastiary` is the corpus directory's historical spelling of the book the
 * converter writes as `bestiary` — the same one-line fold
 * `converted_prose::converted_id` carries on the Rust side.
 */
function packageBook(book: string): string {
  return book === 'beastiary' ? 'bestiary' : book;
}

interface ChassisRecord {
  key: string;
  name: string;
  base_size?: string | null;
  base_move_walk?: number | null;
}

interface TraitRecord {
  key: string;
  name: string;
  race_key: string;
  type_tokens: string[];
  is_racial_default: boolean;
  sets_replace_flags: string[];
  description?: string | null;
}

// ---------------------------------------------------------------------------
// The converted package — `data/sheet_rules/<book>/race_trait/<slug>.json`
// ---------------------------------------------------------------------------

/** A `SheetValue`: either the literal string `"Text"` or a typed value. */
type ConvertedValue = 'Text' | { Number?: { Const?: number } };

/** One `ProsePiece`; only the plain-text variant carries words. */
interface ProsePiece {
  Text?: string;
}

/** One `ProseSegment`. `family` is `"Desc"`, `{ StatBlock: 'Senses' }`, … */
interface ProseSegment {
  family: string | Record<string, string>;
  pieces: ProsePiece[];
}

/** The subset of `SheetRule` this file reads. */
interface ConvertedRule {
  id: string;
  label: string;
  value?: ConvertedValue;
  prose?: ProseSegment[];
  target?: Record<string, unknown>;
  tags?: string[];
  print?: boolean;
}

function readJsonRecords<T>(dir: string): T[] {
  if (!existsSync(dir)) {
    return [];
  }
  const out: T[] = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      out.push(...readJsonRecords<T>(full));
    } else if (entry.name.endsWith('.json')) {
      out.push(JSON.parse(readFileSync(full, 'utf8')).data as T);
    }
  }
  return out;
}

function loadChassis(): ChassisRecord[] {
  return RACE_BOOKS.flatMap((book) => readJsonRecords<ChassisRecord>(join(CORPUS_ROOT, book, 'race')));
}

function loadTraits(): TraitRecord[] {
  return RACE_BOOKS.flatMap((book) => readJsonRecords<TraitRecord>(join(CORPUS_ROOT, book, 'race_trait')));
}

/**
 * The two books that declare races of their own. ARG declares none of the
 * 18 — its records are alternates and flag-granted replacements layered over
 * these — so a *standard* racial trait is by definition one of these.
 */
function loadStandardTraits(): TraitRecord[] {
  return (['core_rulebook', 'beastiary'] as const).flatMap((book) =>
    readJsonRecords<TraitRecord>(join(CORPUS_ROOT, book, 'race_trait'))
  );
}

/** Every converted `race_trait` rule across the three race books. */
function loadConvertedRaceTraits(): ConvertedRule[] {
  const out: ConvertedRule[] = [];
  for (const book of RACE_BOOKS) {
    const dir = join(SHEET_RULES_ROOT, packageBook(book), 'race_trait');
    if (!existsSync(dir)) {
      continue;
    }
    for (const entry of readdirSync(dir)) {
      if (entry.endsWith('.json')) {
        out.push(...(JSON.parse(readFileSync(join(dir, entry), 'utf8')) as ConvertedRule[]));
      }
    }
  }
  return out;
}

/**
 * A plain member of the race: every converted rule the package tags
 * `"<race> Racial Default"`. That tag is the converter's rendering of the
 * same classification the corpus record carries as `is_racial_default`, so
 * this is exactly the `TraitRole::Default` set the resolver applies when no
 * alternate is selected — read off the package's own field, not
 * re-derived.
 */
function convertedDefaultsFor(rules: ConvertedRule[], raceKey: string): ConvertedRule[] {
  return rules.filter((r) => (r.tags ?? []).includes(`${raceKey} Racial Default`));
}

/** Every race the package holds default rules for. */
function racesInPackage(rules: ConvertedRule[]): string[] {
  const out = new Set<string>();
  for (const rule of rules) {
    for (const tag of rule.tags ?? []) {
      if (tag.endsWith(' Racial Default')) {
        out.add(tag.slice(0, -' Racial Default'.length));
      }
    }
  }
  return [...out].sort();
}

const ABILITY_TARGET_TO_KEY: Record<string, AbilityKey> = {
  Str: 'strength',
  Dex: 'dexterity',
  Con: 'constitution',
  Int: 'intelligence',
  Wis: 'wisdom',
  Cha: 'charisma',
};

function abilityScoreRules(defaults: ConvertedRule[]): ConvertedRule[] {
  return defaults.filter((r) => (r.tags ?? []).includes('Racial Ability Scores'));
}

/** A rule's `value` as a plain integer, or `undefined` when it states words. */
function numberValue(rule: ConvertedRule): number | undefined {
  const value = rule.value;
  if (value === undefined || value === 'Text') {
    return undefined;
  }
  return value.Number?.Const;
}

/**
 * The race's fixed ability adjustments, summed over the converted rules'
 * own `target.Ability` + `value.Number.Const` — never from a display name.
 *
 * A single source row states *two* abilities at once
 * (`Dwarf ~ Ability Scores` grants +2 Con and +2 Wis together); the
 * converter writes one rule per target, so crediting every rule is exactly
 * the defect this derivation exists to avoid, now structurally impossible
 * to get wrong.
 */
function convertedAbilityAdjustments(defaults: ConvertedRule[]): Partial<Record<AbilityKey, number>> {
  const out: Partial<Record<AbilityKey, number>> = {};
  for (const rule of abilityScoreRules(defaults)) {
    const ability = ABILITY_TARGET_TO_KEY[String((rule.target ?? {}).Ability ?? '')];
    if (ability === undefined) {
      continue;
    }
    const magnitude = numberValue(rule);
    assert(
      magnitude !== undefined && Number.isInteger(magnitude),
      `${rule.id}: an ability-targeted rule must state an integer magnitude`
    );
    out[ability] = (out[ability] ?? 0) + magnitude!;
  }
  return out;
}

/**
 * The freely-distributed "+2 to one ability score" points.
 *
 * Two parts, because the source states them separately and the converter
 * preserves that split: the *number of picks* is the rule's own numeric
 * value against the `ability_bonus` pool, while the *magnitude per pick*
 * appears only in the rule's own `label` (`+2 to One Ability Score`). The
 * label is parsed strictly — a label that does not match the shape yields
 * no points instead of a guess.
 */
function convertedFloatingBonusPoints(defaults: ConvertedRule[]): number {
  let picks = 0;
  let label: string | undefined;
  for (const rule of abilityScoreRules(defaults)) {
    if ((rule.target ?? {}).Pool !== 'ability_bonus') {
      continue;
    }
    picks += numberValue(rule) ?? 0;
    label = rule.label;
  }
  if (picks === 0) {
    return 0;
  }
  const magnitude = /^\+(\d+) to One Ability Score$/.exec(label ?? '');
  assert(magnitude !== null, `an ability pool rule must state its magnitude in its label, got ${String(label)}`);
  return picks * Number(magnitude![1]);
}

/**
 * The race's **effective** creature size, from the `Racial Size` rule's own
 * label.
 *
 * Deliberately not the chassis' own base size field. The playable size is
 * stated on the race's `Racial Size` trait, and for two races the two
 * disagree — see `verifiesTheChassisBaseSizeIsNotTheEffectiveSizeForEveryRace`.
 */
function convertedEffectiveSize(defaults: ConvertedRule[]): string | undefined {
  const rule = defaults.find((r) => (r.tags ?? []).includes('Racial Size'));
  return rule?.label;
}

/**
 * The race's vision, rendered the way the Character Sheet's Details panel
 * prints it: the converted rule's own `Senses` stat-block prose segment,
 * already reduced to the words a player writes down (`Darkvision 60 ft.`).
 * A race with no such segment honestly has normal vision.
 */
function convertedVision(defaults: ConvertedRule[]): string {
  const readings: string[] = [];
  for (const rule of defaults) {
    for (const segment of rule.prose ?? []) {
      const family = segment.family;
      if (typeof family === 'object' && family.StatBlock === 'Senses') {
        readings.push(segment.pieces.map((piece) => piece.Text ?? '').join(''));
      }
    }
  }
  return readings.length === 0 ? 'Normal' : readings.join(', ');
}

/**
 * The twelve Advanced Race Guide races whose own trait records are **not
 * units of `docs/work-inventory.json`**, so the converter's population never
 * sees them and the package holds no rule for any of them.
 *
 * Reported, never excused (`AT-35-E6-003` cycles 7, 9 and 10 all measured
 * the same mechanism: 178 ARG `race_trait` corpus records outside the
 * inventory, alongside 489 `ability` records in the same state). Admitting
 * them moves the bundle-wide denominator, which is an operator ruling, not
 * this file's. Until it lands, these twelve are pinned **by name** and
 * checked for the classification the corpus does carry — so the day the
 * ruling lands, this list failing is the reminder to widen the derivation
 * back over them.
 */
const RACES_NOT_YET_IN_THE_CONVERTED_PACKAGE = [
  'Catfolk',
  'Changeling',
  'Gillman',
  'Kitsune',
  'Nagaji',
  'Ratfolk',
  'Samsaran',
  'Strix',
  'Suli',
  'Vanara',
  'Vishkanya',
  'Wayang',
] as const;

// ---------------------------------------------------------------------------
// The tests
// ---------------------------------------------------------------------------

/**
 * Sanity first: if the record directories were missing or empty, every
 * assertion below would pass vacuously. Counts are asserted, not assumed.
 */
function verifiesTheRecordsAreReallyOnDiskAndCarryThirtyRaces() {
  const chassis = loadChassis();
  assertEqual(
    chassis.length,
    30,
    'race chassis records on disk (18 -> 24: ARG contributed 0 chassis of its own until ' +
      'SD-31-E6-F4-002, 2026-08-16, which added 6 -- Catfolk, Kitsune, Ratfolk, Strix, Suli, ' +
      'Wayang; 24 -> 28: SD31-E6-F4-004, 2026-08-17, added 4 more -- Gillman, Nagaji, Vanara, ' +
      'Vishkanya; 28 -> 30: SD31-E6-F4-007, 2026-08-17, added the last 2 -- Changeling, ' +
      'Samsaran, closing arg_races.lst\'s full 37-row playable-race roster -- this test only ' +
      'loads CRB/B1/ARG, so Bestiary 2/5\'s chassis stay out of its scope, but ARG\'s own is ' +
      'now real)',
  );
  assertEqual(
    loadTraits().length,
    605,
    'race trait records across all three books (CRB 76 + B1 108 + ARG 421; ARG 156 -> 201 by ' +
      'SD-31 Epic 1-F2, 2026-08-15, 201 -> 259 by SD-31-E6-F4-002\'s own 6-race chassis batch, ' +
      '259 -> 283 by SD-31-E6-F4-003\'s own 24-record alternate-trait batch for those same 6 ' +
      'races, both 2026-08-16, 283 -> 321 by SD31-E6-F4-004\'s own 4-race chassis batch ' +
      '(38 standard-tier records), 2026-08-17, 321 -> 332 by SD31-E6-F4-006\'s own 11-record ' +
      'alternate-trait batch for the same 4 races, 2026-08-17, 332 -> 350 by SD31-E6-F4-007\'s ' +
      'own 2-race chassis batch (18 standard-tier records: Changeling, Samsaran), 2026-08-17, ' +
      'closing arg_races.lst\'s full 37-row playable-race roster -- this test only loads ' +
      'CRB/B1/ARG, so Bestiary 2/5\'s new chassis is out of its scope, but ARG\'s own growth ' +
      'still moves this total; 350 -> 414 by the Core Essentials removal, 2026-08-18 ' +
      '(SD31-CE-COMPANION-001, decisions.md 9): Aasimar\'s and Tiefling\'s 64 heritage records ' +
      '-- 16 selectable heritages plus the 48 replacement rows they grant -- re-filed here from ' +
      'data/corpus/core_essentials/race_trait/, which this test never loaded; 414 -> 421 by ' +
      'SD-32 card-11 T2b lane, 2026-08-23 (decisions.md 16 item 2): the 7 ' +
      '`Human ~ Adoptive Parentage` CHOOSE-pool members, Drow/Dwarf/Elf/Gnome/Grippli/' +
      'Halfling/Orc; 596 -> 605 by SD-34 AT-34-E3-001\'s race_trait_absent_from_race_traits ' +
      'mechanism, 2026-08-27 (`ae25d75d7d`): 9 new core_rulebook rows -- 7 ' +
      '`Adopted Race ~ <Race>` selectors (Dwarf/Elf/Gnome/Half-Elf/Half-Orc/Halfling/Human) ' +
      'plus 2 `Human Ethnicity ~ None`/`~ Unknown` placeholder rows, a fifth row shape the ' +
      'CRB parser had never recognised before)',
  );
  const standard = loadStandardTraits();
  assertEqual(standard.length, 184, 'standard racial trait records (CRB 76 + B1 108; CRB 67 -> 76 by ' +
    'SD-34 AT-34-E3-001\'s race_trait_absent_from_race_traits mechanism, 2026-08-27 (`ae25d75d7d`), ' +
    'see loadTraits() assertion above for the 9-record breakdown)');
  const defaults = standard.filter((t) => t.is_racial_default);
  // 173, not 184: eleven standard-tier records carry no `<Race> Racial
  // Default` type token, so the resolver classifies them `Unclassified` and
  // never auto-applies them. Two are the original Duergar spell-like-ability
  // rows; the other nine are AT-34-E3-001's CRB additions (2026-08-27,
  // `ae25d75d7d`) -- 7 `Adopted Race ~ <Race>` CHOOSE selectors and 2
  // `Human Ethnicity ~ *` placeholders, none of which is a default trait
  // either. Named here so the gap is a stated fact rather than an
  // unexplained count.
  assertEqual(defaults.length, 173, 'racial default trait records across the 18 races');
  assertEqual(
    standard
      .filter((t) => !t.is_racial_default)
      .map((t) => t.key)
      .sort()
      .join('; '),
    'Adopted Race ~ Dwarf; Adopted Race ~ Elf; Adopted Race ~ Gnome; Adopted Race ~ Half-Elf; ' +
      'Adopted Race ~ Half-Orc; Adopted Race ~ Halfling; Adopted Race ~ Human; ' +
      'Duergar ~ Spell-Like Ability ~ Enlarge Person; Duergar ~ Spell-Like Ability ~ Invisibility; ' +
      'Human Ethnicity ~ None; Human Ethnicity ~ Unknown',
    'the eleven standard records that are not racial defaults'
  );
  const argDefaults = readJsonRecords<TraitRecord>(
    join(CORPUS_ROOT, 'advanced_race_guide', 'race_trait')
  ).filter((t) => t.is_racial_default);
  // 0 -> 58 by SD-31-E6-F4-002 (2026-08-16): ARG previously declared no
  // races of its own, so it changed no default build. It now does --
  // Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang -- each with its own
  // real racial defaults, correctly outside `standard`/`defaults` above
  // (which stay CRB/B1-only by design).
  assertEqual(
    argDefaults.length,
    114,
    "ARG's own 6-race batch (SD-31-E6-F4-002) contributes 58 racial defaults, plus " +
      "SD31-E6-F4-004's 4-race follow-on batch (Gillman, Nagaji, Vanara, Vishkanya) " +
      "contributes 38 more, plus SD31-E6-F4-007's 2-race follow-on batch (Changeling, " +
      "Samsaran, closing arg_races.lst's full 37-row playable-race roster) contributes " +
      "18 more (114 total), each with its own real default builds",
  );
  // And the converted package really loaded, or every derivation below
  // reads an empty array and proves nothing.
  const rules = loadConvertedRaceTraits();
  assert(rules.length > 800, `data/sheet_rules/ holds only ${rules.length} converted race_trait rules`);
  assertEqual(
    racesInPackage(rules).join(', '),
    'Aasimar, Drow, Duergar, Dwarf, Elf, Gnome, Goblin, Half-Elf, Half-Orc, Halfling, Hobgoblin, ' +
      'Human, Kobold, Merfolk, Orc, Svirfneblin, Tengu, Tiefling',
    'the 18 races the converted package holds racial defaults for'
  );
}

/**
 * **The pin.** Every field creation reads must resolve, from the converted
 * package, for every race the creation roster offers — read off typed
 * fields, never parsed out of a display string.
 *
 * This used to compare against `RACE_OPTIONS`, a hand-written seven-entry
 * table in `characterHubModel.ts`. That table is gone: the roster is now
 * served by `list_race_creation_roster` from the same records. What is
 * still worth pinning is the derivation itself — that a source row granting
 * two abilities at once credits both, which is the exact defect that
 * silently drifted `race_tables.rs`.
 */
function verifiesTheAbilityDerivationCreditsEveryAbilityAMultiTargetRowNames() {
  const rules = loadConvertedRaceTraits();
  // Expectations verified against the named record in BOTH forms during
  // `AT-35-E6-003` cycle 10: the corpus record's own chains (the pre-swap
  // derivation) and the converted rules' `target`/`value` (this one).
  const expected: Record<string, Partial<Record<AbilityKey, number>>> = {
    // One source row states +2 Con and +2 Wis together; a second states -2 Cha.
    Dwarf: { constitution: 2, wisdom: 2, charisma: -2 },
    Goblin: { dexterity: 4, strength: -2, charisma: -2 },
    // Four abilities across two rows.
    Orc: { strength: 4, intelligence: -2, wisdom: -2, charisma: -2 },
    Svirfneblin: { dexterity: 2, wisdom: 2, strength: -2, charisma: -4 },
    // Floating-pool races state no fixed modifier at all.
    Human: {},
    'Half-Elf': {},
  };
  for (const [raceKey, adjustments] of Object.entries(expected)) {
    const derived = convertedAbilityAdjustments(convertedDefaultsFor(rules, raceKey));
    for (const ability of ABILITY_KEYS) {
      assertEqual(derived[ability] ?? 0, adjustments[ability] ?? 0, `${raceKey} ${ability} racial adjustment`);
    }
  }
  // Only Human, Half-Elf and Half-Orc carry a floating pool. Derived across
  // every race in the package rather than asserted for three, so a fourth
  // appearing is a failure rather than an invisible change.
  const floating = racesInPackage(rules)
    .filter((race) => convertedFloatingBonusPoints(convertedDefaultsFor(rules, race)) > 0)
    .sort()
    .join(', ');
  assertEqual(floating, 'Half-Elf, Half-Orc, Human', 'races with a floating ability pool');
  assertEqual(
    convertedFloatingBonusPoints(convertedDefaultsFor(rules, 'Human')),
    2,
    'Human gets one pick worth +2'
  );
}

/**
 * The answer to "what does creation need that the records do not provide?",
 * for the races creation does not yet offer: **nothing, except `body`.**
 * Each of the four rules-bearing fields resolves to a real value for every
 * one of the 18 races the converted package holds, so the widening is not
 * blocked on missing race data.
 *
 * The other 12 chassis races are ARG's own, and the package holds no rule
 * for any of them — `RACES_NOT_YET_IN_THE_CONVERTED_PACKAGE` states why and
 * pins them by name. They are **counted and named here, never skipped**:
 * their corpus records are checked for the classification they do carry, so
 * "the package cannot serve them yet" stays a measured number rather than a
 * quiet omission.
 */
function verifiesEveryRulesBearingFieldResolvesForEveryRaceThePackageHolds() {
  const rules = loadConvertedRaceTraits();
  const traits = loadTraits();
  const chassis = loadChassis();
  const inPackage = new Set(racesInPackage(rules));
  let withFullCreationData = 0;
  const outside: string[] = [];

  for (const race of chassis) {
    if (!inPackage.has(race.key)) {
      outside.push(race.key);
      // Not excused: the corpus records exist and are classified, so the
      // only thing missing is the converter's population. Checked, so this
      // remainder cannot silently become "these races have no data".
      const corpusDefaults = traits.filter((t) => t.race_key === race.key && t.is_racial_default);
      assert(corpusDefaults.length > 0, `${race.key} must have corpus racial defaults`);
      const tokens = new Set(corpusDefaults.flatMap((t) => t.type_tokens));
      assert(tokens.has('Racial Ability Scores'), `${race.key} must classify an ability-score trait`);
      assert(tokens.has('Racial Size'), `${race.key} must classify a size trait`);
      continue;
    }

    const defaults = convertedDefaultsFor(rules, race.key);
    assert(defaults.length > 0, `${race.key} must have converted racial defaults`);

    const adjustments = convertedAbilityAdjustments(defaults);
    const floating = convertedFloatingBonusPoints(defaults);
    assert(
      Object.keys(adjustments).length > 0 || floating > 0,
      `${race.key} must state either fixed ability adjustments or a floating ability pool`
    );

    const size = convertedEffectiveSize(defaults);
    assert(size === 'Small' || size === 'Medium', `${race.key} must resolve a playable size, got ${String(size)}`);

    // Every reading is already the words a player writes down.
    const vision = convertedVision(defaults);
    assert(vision.length > 0, `${race.key} must resolve a vision reading`);

    withFullCreationData += 1;
  }
  assertEqual(withFullCreationData, 18, 'races carrying a complete creation chassis in the converted package');
  assertEqual(
    outside.sort().join(', '),
    [...RACES_NOT_YET_IN_THE_CONVERTED_PACKAGE].join(', '),
    'the ARG races whose corpus records are not inventory units, so the converter never saw them'
  );
  assertEqual(withFullCreationData + outside.length, chassis.length, 'every chassis race is accounted for');
}

/**
 * Two races' vision readings, pinned by value, so "the derivation returns a
 * string" is never mistaken for "the derivation returns the right string".
 * Both are exactly what this file produced from the ingest tokens before
 * `AT-35-E6-003` cycle 10 swapped the source.
 */
function verifiesTheVisionReadingsAreTheWordsAPlayerWritesDown() {
  const rules = loadConvertedRaceTraits();
  assertEqual(convertedVision(convertedDefaultsFor(rules, 'Dwarf')), 'Darkvision 60 ft.', 'Dwarf vision');
  assertEqual(convertedVision(convertedDefaultsFor(rules, 'Elf')), 'Low-Light Vision', 'Elf vision');
  assertEqual(
    convertedVision(convertedDefaultsFor(rules, 'Svirfneblin')),
    'Darkvision 120 ft., Low-Light Vision',
    'Svirfneblin states two readings'
  );
  assertEqual(convertedVision(convertedDefaultsFor(rules, 'Human')), 'Normal', 'Human has no vision trait');
}

/**
 * The chassis' own base size is **not** the playable size, and saying so
 * costs two races their correct size.
 *
 * `ResolvedRace::size` in `race_resolver.rs` reads the chassis field only.
 * The playable size is stated on the `Racial Size` trait, and for Aasimar
 * and Tiefling the two disagree: chassis `S`, trait Medium. Published PF1
 * makes both Medium, and the trait's own prose agrees ("Aasimars are Medium
 * creatures…"). Any consumer reading the chassis field is wrong about those
 * two, so this pins the disagreement rather than letting a player discover
 * it.
 */
function verifiesTheChassisBaseSizeIsNotTheEffectiveSizeForEveryRace() {
  const rules = loadConvertedRaceTraits();
  const chassis = loadChassis();
  const inPackage = new Set(racesInPackage(rules));
  const disagreeing: string[] = [];
  for (const race of chassis) {
    if (!inPackage.has(race.key)) {
      continue;
    }
    const chassisSize = { S: 'Small', M: 'Medium' }[race.base_size ?? ''];
    const effective = convertedEffectiveSize(convertedDefaultsFor(rules, race.key));
    if (chassisSize !== effective) {
      disagreeing.push(`${race.key} (chassis ${String(chassisSize)} vs trait ${String(effective)})`);
    }
  }
  disagreeing.sort();
  assertEqual(
    disagreeing.join('; '),
    'Aasimar (chassis Small vs trait Medium); Tiefling (chassis Small vs trait Medium)',
    'exactly Aasimar and Tiefling have a chassis base size that is not their playable size'
  );
}

/**
 * The one field that is genuinely missing, for **every** race including the
 * 7 already shipped: height and weight.
 *
 * PCGen carries it in per-race bio settings, which no book's ingest reads,
 * and the converted schema has no field that could hold a height or a
 * weight at all — so the absence is structural, not accidental. This scans
 * every converted race rule for any stat-block or aspect segment that
 * states one, and asserts none exists, so "the records have this" is never
 * assumed of them.
 */
function verifiesNoConvertedRaceRuleStatesAHeightOrWeightProfile() {
  const rules = loadConvertedRaceTraits();
  assert(rules.length > 800, 'the converted package must be loaded, or this proves nothing');
  const bodyWords = ['height', 'weight', 'baseht', 'basewt'];
  const carriers: string[] = [];
  for (const rule of rules) {
    for (const segment of rule.prose ?? []) {
      const family = segment.family;
      const label = typeof family === 'string' ? family : Object.values(family).join(' ');
      if (bodyWords.some((word) => label.toLowerCase().includes(word))) {
        carriers.push(rule.id);
      }
    }
  }
  assertEqual(carriers.length, 0, 'no converted race rule states a height/weight profile');
  // And the profiles that ship nonetheless have real numbers, i.e. they came
  // from somewhere other than the ingested records. Pinned to exactly the 7
  // Core Rulebook races so the hand-entered set cannot quietly grow to cover
  // the races nothing has body data for.
  assertEqual(
    Object.keys(RACE_BODY_PROFILES).sort().join(' '),
    'race:dwarf race:elf race:gnome race:half-elf race:half-orc race:halfling race:human',
    'hand-entered body profiles'
  );
  for (const [raceId, profile] of Object.entries(RACE_BODY_PROFILES)) {
    assert(profile.male.baseHeightInches > 0, `${raceId} carries a hand-entered body profile`);
  }
}

function main() {
  verifiesTheRecordsAreReallyOnDiskAndCarryThirtyRaces();
  verifiesTheAbilityDerivationCreditsEveryAbilityAMultiTargetRowNames();
  verifiesEveryRulesBearingFieldResolvesForEveryRaceThePackageHolds();
  verifiesTheVisionReadingsAreTheWordsAPlayerWritesDown();
  verifiesTheChassisBaseSizeIsNotTheEffectiveSizeForEveryRace();
  verifiesNoConvertedRaceRuleStatesAHeightOrWeightProfile();
  console.log('raceCreationCoverage: ok');
}

main();
