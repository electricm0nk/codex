/**
 * Does the ingested race corpus actually carry everything character
 * creation needs, for all 18 races the Race Trait Catalog already browses?
 *
 * # Why this file exists
 *
 * Creation offered 7 races from a hardcoded `RACE_OPTIONS` table in
 * `characterHubModel.ts`. The corpus carries 18 (Core Rulebook's 7 +
 * Bestiary 1's 11). A previous assessment said widening creation "needs
 * per-race data nobody has". This file tested that claim field by field
 * instead of inheriting it, against the real on-disk corpus JSON — the same
 * records `codex::rules_core::race_resolver` reads, not a fixture — and
 * found the claim false for every field except height/weight.
 *
 * **The table is now gone.** `list_race_creation_roster`
 * (`character_hub.rs`) derives all 18 from these records, so there is no
 * hand-maintained mirror left to drift. What this file still pins is the
 * derivation: the identical shape one layer down
 * (`rules_tables/crb/race_tables.rs`) silently drifted from the corpus on
 * four races' ability modifiers for months, because PCGen states two ability
 * grants in one token (`BONUS:STAT|CON,WIS|2`) and a hand transcription read
 * only up to the comma.
 *
 * # What creation actually consumes
 *
 * Traced end to end (`CreateCharacterForm` → `composeCreateCharacterRequest`
 * → the `create_character` command → `compose_character_input`):
 *
 * | `RaceOption` field | consumed by | in the corpus? |
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
 * `body` is genuinely absent: PCGen keeps height/weight in
 * `<race>/<race>_biosettings.lst`, which this project has not ingested for
 * any book. It is also the one field creation does not depend on — height
 * and weight are rolled for display in the form and are not part of
 * `CreateCharacterRequest`, so nothing is persisted or computed from them.
 * `verifiesTheCorpusCarriesNoHeightOrWeightProfileForAnyRace` pins that
 * absence so it stays a checked fact.
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

/**
 * The books that carry race content, and the wire code each race is
 * attributed to. Mirrors `race_catalog.rs`'s own `RACE_CORPUS_BOOKS`.
 * `advanced_race_guide` is loaded but declares zero racial *defaults*
 * (asserted below), so it contributes nothing to a default race build.
 */
const RACE_BOOKS = ['core_rulebook', 'beastiary', 'advanced_race_guide'] as const;

interface RawBonusChain {
  qualifiers: string[];
}

interface RawToken {
  key: string;
  value: string;
}

interface ChassisRecord {
  key: string;
  name: string;
  base_size?: string | null;
  base_move_walk?: number | null;
  raw_tokens: RawToken[];
}

interface TraitRecord {
  key: string;
  name: string;
  race_key: string;
  type_tokens: string[];
  is_racial_default: boolean;
  sets_replace_flags: string[];
  description?: string | null;
  raw_tokens: RawToken[];
  raw_bonus_chains: RawBonusChain[];
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
 * The two books that declare races of their own. ARG declares none — its
 * 201 records (156 -> 201 by SD-31 Epic 1-F2, 2026-08-15) are alternates and
 * flag-granted replacements layered over these — so a *standard* racial
 * trait is by definition one of these.
 */
function loadStandardTraits(): TraitRecord[] {
  return (['core_rulebook', 'beastiary'] as const).flatMap((book) =>
    readJsonRecords<TraitRecord>(join(CORPUS_ROOT, book, 'race_trait'))
  );
}

/**
 * A plain member of the race: every trait record flagged
 * `is_racial_default`, with no alternate selected so nothing is suppressed.
 * This is exactly `RaceCorpus::resolve(race_key, &[])`'s `TraitRole::Default`
 * set — the resolver's own signal (`TYPE:...<Race> Racial Default...`), read
 * off the same field, not a re-derivation.
 */
function defaultTraitsFor(traits: TraitRecord[], raceKey: string): TraitRecord[] {
  return traits.filter((t) => t.race_key === raceKey && t.is_racial_default);
}

const STAT_CODE_TO_ABILITY: Record<string, AbilityKey> = {
  STR: 'strength',
  DEX: 'dexterity',
  CON: 'constitution',
  INT: 'intelligence',
  WIS: 'wisdom',
  CHA: 'charisma',
};

function abilityScoreTrait(defaults: TraitRecord[]): TraitRecord | undefined {
  return defaults.find((t) => t.type_tokens.includes('Racial Ability Scores'));
}

/**
 * The race's fixed ability adjustments, derived from the machine-readable
 * `BONUS:STAT|<stats>|<magnitude>` chains alone — never from the row's
 * display name. `BONUS:STAT|CON,WIS|2` names *two* abilities in one token;
 * reading only the first is the exact defect this derivation exists to
 * avoid.
 */
function corpusAbilityAdjustments(defaults: TraitRecord[]): Partial<Record<AbilityKey, number>> {
  const out: Partial<Record<AbilityKey, number>> = {};
  const trait = abilityScoreTrait(defaults);
  if (!trait) {
    return out;
  }
  for (const chain of trait.raw_bonus_chains) {
    if (chain.qualifiers[0] !== 'STAT') {
      continue;
    }
    const stats = chain.qualifiers[1] ?? '';
    const magnitude = Number(chain.qualifiers[2]);
    assert(Number.isInteger(magnitude), `${trait.key}: BONUS:STAT magnitude must be an integer`);
    for (const code of stats.split(',')) {
      const ability = STAT_CODE_TO_ABILITY[code.trim()];
      assert(ability !== undefined, `${trait.key}: unknown ability code ${code}`);
      out[ability] = (out[ability] ?? 0) + magnitude;
    }
  }
  return out;
}

/**
 * The freely-distributed "+2 to one ability score" points.
 *
 * Two sources, because PCGen splits the fact across two: the *number of
 * picks* is machine-readable (`BONUS:ABILITYPOOL|Ability Bonus|1`), but the
 * *magnitude per pick* appears only in the row's own display name
 * (`+2 to One Ability Score`) and its `DESC:` prose. That is stated here
 * rather than hidden, and the name is parsed strictly — a row that does not
 * match the shape yields no points instead of a guess.
 */
function corpusFloatingBonusPoints(defaults: TraitRecord[]): number {
  const trait = abilityScoreTrait(defaults);
  if (!trait) {
    return 0;
  }
  let picks = 0;
  for (const chain of trait.raw_bonus_chains) {
    if (chain.qualifiers[0] === 'ABILITYPOOL' && chain.qualifiers[1] === 'Ability Bonus') {
      picks += Number(chain.qualifiers[2] ?? 0);
    }
  }
  if (picks === 0) {
    return 0;
  }
  const magnitude = /^\+(\d+) to One Ability Score$/.exec(trait.name);
  assert(magnitude !== null, `${trait.key}: an ability pool row must state its magnitude in its name, got ${trait.name}`);
  return picks * Number(magnitude![1]);
}

/**
 * The race's **effective** creature size.
 *
 * Deliberately not the chassis' own `FACT:BaseSize`. PCGen states the
 * playable size on the race's `Racial Size` trait as `TEMPLATE:SIZE_<code>`,
 * and for two races the two disagree — see
 * `verifiesTheChassisBaseSizeTokenIsNotTheEffectiveSizeForEveryRace`.
 */
function corpusEffectiveSize(defaults: TraitRecord[]): string | undefined {
  const trait = defaults.find((t) => t.type_tokens.includes('Racial Size'));
  if (!trait) {
    return undefined;
  }
  const template = trait.raw_tokens.find((t) => t.key === 'TEMPLATE' && t.value.startsWith('SIZE_'));
  if (template) {
    return { SIZE_S: 'Small', SIZE_M: 'Medium' }[template.value];
  }
  // Human's row carries no TEMPLATE token; its own name states the size.
  return trait.name;
}

/**
 * The race's vision, rendered the way the Character Sheet's Details panel
 * prints it, from the `VISION:` tokens on the resolved default traits.
 * A race with no vision trait honestly has normal vision.
 */
function corpusVision(defaults: TraitRecord[]): string {
  const readings: string[] = [];
  for (const trait of defaults) {
    for (const token of trait.raw_tokens) {
      if (token.key !== 'VISION') {
        continue;
      }
      const darkvision = /^Darkvision \((\d+)\)$/.exec(token.value);
      if (darkvision) {
        readings.push(`Darkvision ${darkvision[1]} ft.`);
      } else if (token.value === 'Low-Light Vision') {
        readings.push('Low-light vision');
      } else {
        throw new Error(`${trait.key}: unrecognized VISION token ${token.value}`);
      }
    }
  }
  return readings.length === 0 ? 'Normal' : readings.join(', ');
}

// ---------------------------------------------------------------------------
// The tests
// ---------------------------------------------------------------------------

/**
 * Sanity first: if the corpus directories were missing or empty, every
 * assertion below would pass vacuously. Counts are asserted, not assumed.
 */
function verifiesTheCorpusIsReallyOnDiskAndCarriesEighteenRaces() {
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
}

/**
 * **The pin.** Every field creation reads must resolve, from the corpus, for
 * every race the creation roster offers — derived from the machine-readable
 * tokens, never from a display string.
 *
 * This used to compare the corpus against `RACE_OPTIONS`, a hand-written
 * seven-entry table in `characterHubModel.ts`. That table is gone: the
 * roster is now served by `list_race_creation_roster` from these same
 * records, so there is no second copy left to drift. What is still worth
 * pinning here is the derivation itself — that reading a `BONUS:STAT` chain
 * credits every ability it names, which is the exact defect that silently
 * drifted `race_tables.rs` from the corpus on four races.
 */
function verifiesTheAbilityDerivationCreditsEveryAbilityAMultiStatChainNames() {
  // `loadTraits()`, not `loadStandardTraits()`, so the floating-pool scan
  // below (which iterates ALL of `loadChassis()`, now including ARG's own
  // 6 races) resolves their real defaults instead of silently seeing none.
  const traits = loadTraits();
  // Corpus-verified expectations, each read off the named record's own
  // `raw_bonus_chains` (`data/corpus/<book>/race_trait/<race>/*_ability_scores.json`).
  const expected: Record<string, Partial<Record<AbilityKey, number>>> = {
    // `BONUS:STAT|CON,WIS|2` + `BONUS:STAT|CHA|-2` — two abilities in one token.
    Dwarf: { constitution: 2, wisdom: 2, charisma: -2 },
    // `BONUS:STAT|DEX|4` + `BONUS:STAT|STR,CHA|-2`.
    Goblin: { dexterity: 4, strength: -2, charisma: -2 },
    // Four abilities across two chains.
    Orc: { strength: 4, intelligence: -2, wisdom: -2, charisma: -2 },
    Svirfneblin: { dexterity: 2, wisdom: 2, strength: -2, charisma: -4 },
    // Floating-pool races state no fixed modifier at all.
    Human: {},
    'Half-Elf': {},
  };
  for (const [raceKey, adjustments] of Object.entries(expected)) {
    const derived = corpusAbilityAdjustments(defaultTraitsFor(traits, raceKey));
    for (const ability of ABILITY_KEYS) {
      assertEqual(derived[ability] ?? 0, adjustments[ability] ?? 0, `${raceKey} ${ability} racial adjustment`);
    }
  }
  // Only Human, Half-Elf and Half-Orc carry a floating pool. Derived across
  // all 18 rather than asserted for three, so a fourth appearing is a failure
  // rather than an invisible change.
  const floating = loadChassis()
    .filter((race) => corpusFloatingBonusPoints(defaultTraitsFor(traits, race.key)) > 0)
    .map((race) => race.key)
    .sort()
    .join(', ');
  assertEqual(floating, 'Half-Elf, Half-Orc, Human', 'races with a floating ability pool');
}

/**
 * The answer to "what does creation need that the corpus does not
 * provide?", for the 11 races creation does not yet offer: **nothing,
 * except `body`.** Each of the four rules-bearing fields resolves to a real
 * value for every one of the 18, so the widening is not blocked on missing
 * race data.
 *
 * **`loadTraits()`, not `loadStandardTraits()`, as of SD-31-E6-F4-002
 * (2026-08-16).** `defaultTraitsFor` filters to `is_racial_default` itself,
 * so passing the full RACE_BOOKS trait set (CRB/B1/ARG) is safe -- it still
 * resolves each race's own defaults only, never someone else's alternate.
 * Needed because `loadChassis()` (used for `chassis` below) now includes
 * ARG's own 6 races (Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang), whose
 * defaults live only in ARG's own `race_trait/` records --
 * `loadStandardTraits()`'s CRB/B1-only scope would report zero defaults for
 * every one of them and throw on the very next assertion, not because
 * their creation data is missing but because this function was looking in
 * the wrong two books.
 */
function verifiesTheCorpusSuppliesEveryRulesBearingFieldForAllEighteenRaces() {
  const traits = loadTraits();
  const chassis = loadChassis();
  let withFullCreationData = 0;

  for (const race of chassis) {
    const defaults = defaultTraitsFor(traits, race.key);
    assert(defaults.length > 0, `${race.key} must have corpus racial defaults`);

    const adjustments = corpusAbilityAdjustments(defaults);
    const floating = corpusFloatingBonusPoints(defaults);
    assert(
      Object.keys(adjustments).length > 0 || floating > 0,
      `${race.key} must state either fixed ability adjustments or a floating ability pool`
    );

    const size = corpusEffectiveSize(defaults);
    assert(size === 'Small' || size === 'Medium', `${race.key} must resolve a playable size, got ${String(size)}`);

    // Throws on an unrecognized VISION token rather than guessing.
    corpusVision(defaults);

    withFullCreationData += 1;
  }
  assertEqual(
    withFullCreationData,
    30,
    'races carrying a complete creation chassis (18 -> 24: ARG\'s own 6-race batch, ' +
      'SD-31-E6-F4-002, 2026-08-16; 24 -> 28: ARG\'s 4-race follow-on batch, ' +
      'SD31-E6-F4-004, 2026-08-17; 28 -> 30: ARG\'s 2-race follow-on batch (Changeling, ' +
      'Samsaran), SD31-E6-F4-007, 2026-08-17, closing arg_races.lst\'s full 37-row ' +
      'playable-race roster)',
  );
}

/**
 * The chassis' `FACT:BaseSize` is **not** the playable size, and saying so
 * costs two races their correct size.
 *
 * `ResolvedRace::size` in `race_resolver.rs` reads the chassis token only.
 * PCGen states the playable size on the `Racial Size` trait's
 * `TEMPLATE:SIZE_<code>`, and for Aasimar and Tiefling the two disagree:
 * chassis `S`, trait `SIZE_M`. Published PF1 makes both Medium, and PCGen's
 * own row prose agrees ("Aasimars are Medium creatures…"). Any consumer
 * reading the chassis token is wrong about those two, so this pins the
 * disagreement rather than letting it be discovered by a player.
 */
function verifiesTheChassisBaseSizeTokenIsNotTheEffectiveSizeForEveryRace() {
  // `loadTraits()`, not `loadStandardTraits()` -- same reason as the
  // function above: `chassis` now includes ARG's own 6 races, whose
  // defaults live only in ARG's own race_trait records.
  const traits = loadTraits();
  const chassis = loadChassis();
  const disagreeing: string[] = [];
  for (const race of chassis) {
    const chassisSize = { S: 'Small', M: 'Medium' }[race.base_size ?? ''];
    const effective = corpusEffectiveSize(defaultTraitsFor(traits, race.key));
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
 * PCGen carries it in `<race>/<race>_biosettings.lst` (`BASEHT`,
 * `HTDIEROLL`, `BASEWT`, `TOTALWT`), which no book's ingest reads. The 7
 * shipped `body` profiles are hand-entered constants with no corpus behind
 * them. This asserts the absence so that "the corpus has this" is never
 * assumed of it.
 */
function verifiesTheCorpusCarriesNoHeightOrWeightProfileForAnyRace() {
  const chassis = loadChassis();
  // `loadTraits()`, not `loadStandardTraits()`, so this "prove the
  // negative" scan also covers ARG's own 58 new standard-tier trait
  // records (SD-31-E6-F4-002, 2026-08-16), not only CRB/B1's.
  const traits = loadTraits();
  const bioTokenKeys = ['BASEHT', 'HTDIEROLL', 'BASEWT', 'WTDIEROLL', 'TOTALWT'];
  const carriers: string[] = [];
  for (const record of [
    ...chassis.map((c) => ({ key: c.key, tokens: c.raw_tokens })),
    ...traits.map((t) => ({ key: t.key, tokens: t.raw_tokens })),
  ]) {
    if (record.tokens.some((token) => bioTokenKeys.includes(token.key))) {
      carriers.push(record.key);
    }
  }
  assertEqual(carriers.length, 0, 'no corpus race record carries a height/weight profile');
  // And the profiles that ship nonetheless have real numbers, i.e. they came
  // from somewhere other than the corpus. Pinned to exactly the 7 Core
  // Rulebook races so the hand-entered set cannot quietly grow to cover the
  // 11 races the corpus has no body data for.
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
  verifiesTheCorpusIsReallyOnDiskAndCarriesEighteenRaces();
  verifiesTheAbilityDerivationCreditsEveryAbilityAMultiStatChainNames();
  verifiesTheCorpusSuppliesEveryRulesBearingFieldForAllEighteenRaces();
  verifiesTheChassisBaseSizeTokenIsNotTheEffectiveSizeForEveryRace();
  verifiesTheCorpusCarriesNoHeightOrWeightProfileForAnyRace();
  console.log('raceCreationCoverage: ok');
}

main();
