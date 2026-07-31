/**
 * Does the ingested race corpus actually carry everything character
 * creation needs, for all 18 races the Race Trait Catalog already browses?
 *
 * # Why this file exists
 *
 * `RACE_OPTIONS` in `characterHubModel.ts` offers 7 races. The corpus
 * carries 18 (Core Rulebook's 7 + Bestiary 1's 11). A previous assessment
 * said widening creation "needs per-race data nobody has". This file tests
 * that claim field by field instead of inheriting it, against the real
 * on-disk corpus JSON — the same records
 * `codex::rules_core::race_resolver` reads, not a fixture.
 *
 * It also pins the 7 shipped `RACE_OPTIONS` entries against those records.
 * `RACE_OPTIONS` is a hand-maintained mirror of corpus facts with, until
 * now, no cross-check at all — and the identical shape one layer down
 * (`rules_tables/crb/race_tables.rs`) had silently drifted from the corpus
 * on four races' ability modifiers for months, because PCGen states two
 * ability grants in one token (`BONUS:STAT|CON,WIS|2`) and a hand
 * transcription read only up to the comma. This is the check that would
 * have caught it here.
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

import { ABILITY_KEYS, RACE_OPTIONS } from './characterHubModel';
import type { AbilityKey } from './characterHubModel';
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
 * 156 records are alternates and flag-granted replacements layered over
 * these — so a *standard* racial trait is by definition one of these.
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
  assertEqual(chassis.length, 18, 'race chassis records on disk');
  assertEqual(loadTraits().length, 331, 'race trait records across all three books (CRB 67 + B1 108 + ARG 156)');
  const standard = loadStandardTraits();
  assertEqual(standard.length, 175, 'standard racial trait records (CRB 67 + B1 108)');
  const defaults = standard.filter((t) => t.is_racial_default);
  // 173, not 175: two Duergar spell-like-ability rows carry no
  // `<Race> Racial Default` type token, so the resolver classifies them
  // `Unclassified` and never auto-applies them. Named here so the
  // two-record gap is a stated fact rather than an unexplained count.
  assertEqual(defaults.length, 173, 'racial default trait records across the 18 races');
  assertEqual(
    standard
      .filter((t) => !t.is_racial_default)
      .map((t) => t.key)
      .sort()
      .join('; '),
    'Duergar ~ Spell-Like Ability ~ Enlarge Person; Duergar ~ Spell-Like Ability ~ Invisibility',
    'the two standard records that are not racial defaults'
  );
  const argDefaults = readJsonRecords<TraitRecord>(
    join(CORPUS_ROOT, 'advanced_race_guide', 'race_trait')
  ).filter((t) => t.is_racial_default);
  assertEqual(argDefaults.length, 0, 'ARG declares zero racial defaults, so it changes no default build');
}

/**
 * **The pin.** Every field creation reads off `RACE_OPTIONS` for the 7
 * shipped races must match what the corpus says — derived from the
 * machine-readable tokens, never from a display string.
 */
function verifiesEveryShippedRaceOptionMatchesTheCorpus() {
  const traits = loadStandardTraits();
  const chassis = loadChassis();
  for (const option of RACE_OPTIONS) {
    const raceKey = option.label; // `RACE_OPTIONS` labels are the corpus race keys.
    assert(
      chassis.some((c) => c.key === raceKey),
      `${option.label} must have a corpus chassis record`
    );
    const defaults = defaultTraitsFor(traits, raceKey);
    assert(defaults.length > 0, `${option.label} must have corpus racial defaults`);

    const corpusAdjustments = corpusAbilityAdjustments(defaults);
    for (const ability of ABILITY_KEYS) {
      assertEqual(
        option.abilityAdjustments[ability] ?? 0,
        corpusAdjustments[ability] ?? 0,
        `${option.label} ${ability} racial adjustment (RACE_OPTIONS vs corpus BONUS:STAT)`
      );
    }
    assertEqual(
      option.floatingBonusPoints,
      corpusFloatingBonusPoints(defaults),
      `${option.label} floating ability points`
    );
    assertEqual(option.size, corpusEffectiveSize(defaults), `${option.label} size`);
    assertEqual(option.vision, corpusVision(defaults), `${option.label} vision`);
  }
}

/**
 * The answer to "what does creation need that the corpus does not
 * provide?", for the 11 races creation does not yet offer: **nothing,
 * except `body`.** Each of the four rules-bearing fields resolves to a real
 * value for every one of the 18, so the widening is not blocked on missing
 * race data.
 */
function verifiesTheCorpusSuppliesEveryRulesBearingFieldForAllEighteenRaces() {
  const traits = loadStandardTraits();
  const chassis = loadChassis();
  const offered = new Set(RACE_OPTIONS.map((option) => option.label));
  let notYetOffered = 0;

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

    if (!offered.has(race.key)) {
      notYetOffered += 1;
    }
  }
  assertEqual(notYetOffered, 11, 'Bestiary 1 races carrying full creation data that creation does not yet offer');
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
  const traits = loadStandardTraits();
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
  const traits = loadStandardTraits();
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
  // And every shipped option nonetheless has one, i.e. it came from
  // somewhere other than the corpus.
  for (const option of RACE_OPTIONS) {
    assert(option.body.male.baseHeightInches > 0, `${option.label} carries a hand-entered body profile`);
  }
}

function main() {
  verifiesTheCorpusIsReallyOnDiskAndCarriesEighteenRaces();
  verifiesEveryShippedRaceOptionMatchesTheCorpus();
  verifiesTheCorpusSuppliesEveryRulesBearingFieldForAllEighteenRaces();
  verifiesTheChassisBaseSizeTokenIsNotTheEffectiveSizeForEveryRace();
  verifiesTheCorpusCarriesNoHeightOrWeightProfileForAnyRace();
  console.log('raceCreationCoverage: ok');
}

main();
