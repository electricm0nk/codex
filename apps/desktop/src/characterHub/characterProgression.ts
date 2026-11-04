import type { ProgressionChoicesDto, ProgressionLevelDto } from '../boundary/listProgressionLedger';
import { knownClass } from './classCatalog';
import { SKILLS, skillIdFor } from './skillsModel';

/**
 * Pathfinder 1e level progression: per-class skill points and class features,
 * plus the universal PF1 benefits (a feat at the levels the engine names, an
 * ability score increase every 4th level). Used by the character sheet to list
 * what each current level granted and what the next level of each held class offers.
 * The sheet's feat counts are the engine's `feats.budget.*` rows, read by `readFeatBudget`.
 */

/*
 * SD-36 F4c: skill ranks per level, hit dice and class labels are read off the class catalog
 * (`classCatalog.ts`), which holds the engine's served roster: every census class's converted
 * `Skill ranks per level` and `Hit die` rows. The per-class skill-points table that lived here
 * was wrong for 33 of the 59 roster classes (Arcanist 3 against the record's 2; every class it
 * did not list silently defaulted to 2) and is gone. A class the catalog does not know states
 * nothing: its figures are `null` and the sheet prints Unknown.
 */

/*
 * Two hand-authored rules tables used to live here and are deliberately
 * gone:
 *
 *   - a per-class-level class-feature map of bare labels (`'Bravery +1'`,
 *     `'Bonus combat feat'`, `'Armor training 1'`) covering Fighter and
 *     Wizard only, with no magnitudes and no provenance. It duplicated 411
 *     cited `class_feature.*` / `class_chassis.*` records the engine
 *     computes on every load.
 *   - a hardcoded Wizard-only spells-per-day table
 *     covering class levels 1-9, standing in for real
 *     `class_spell.*.total_spells_per_day.*` records the engine grounds for
 *     every supported caster.
 *
 * Rules data hand-authored in the frontend is exactly the debt
 * `docs/governance/no-stub-mvp-doctrine.md` forbids: a second, uncited
 * source of rules truth that drifts silently from the engine's. Both
 * surfaces now read the engine's own records —
 * `classFeaturesModel.ts` and `spellsPerDayModel.ts` project them from
 * `LoadSavedCharacterResponse.explanations`, and `boundary/previewLevelUp.ts`
 * answers "what does the next level grant" from Epic 7's real per-class
 * level-up engine.
 *
 * What remains in this module is deliberately not class-table data: skill
 * points per class (read off the served roster), the ability score increase every 4th level,
 * class-summary parsing, and hit points. Which levels grant a feat is the engine's
 * (`list_feat_levels`, installed with the roster).
 */

/*
 * SD-36 F6a: the hand-kept `MARTIAL_WEAPON_CLASSES` (5 ids) and `CASTER_CLASSES` (6 ids) tables
 * that lived here are gone. With 59 classes offered they marked Samurai, Warrior, Magus and every
 * other newly offered martial class "✗ Martial Weapons" and printed "—" for every caster outside
 * the six. Weapon proficiency and caster level now come from the engine per held class
 * (`list_class_facts`, folded by `classFactsModel.ts`).
 */

export interface HeldClass {
  classId: string;
  classLabel: string;
  level: number;
}

export interface LevelEntry {
  /** Character level at which this class level was taken (drives feats / ability increases). */
  characterLevel: number;
  classId: string;
  classLabel: string;
  /** Which level of this class this entry represents (drives class features & spells). */
  classLevel: number;
  /** Base skill ranks for this class before the Intelligence modifier; `null` when not stated. */
  skillPointsBase: number | null;
  /**
   * The universal PF1 benefits this *character* level grants — a feat at the
   * levels the engine names, an ability score increase every 4th. Both are
   * general rules keyed to total character level, not entries from any class table.
   *
   * Class features are deliberately not here: they come from the engine's
   * own `class_feature.*` / `class_chassis.*` records (see
   * `classFeaturesModel.ts`) rather than from a table hand-authored in the
   * frontend.
   */
  features: string[];
}

/** The class's skill ranks per level, off the served roster; `null` when not known. */
export function classSkillPointsBase(classId: string): number | null {
  return knownClass(classId)?.skillRanksPerLevel ?? null;
}

let featLevels: ReadonlySet<number> = new Set();

/** The character levels that grant a feat, as `list_feat_levels` serves them. Empty until installed. */
export function installFeatLevels(levels: readonly number[]): void {
  featLevels = new Set(levels);
}

/**
 * A feat at the levels the engine names, and an ability score increase every 4th character level
 * (both keyed to total level, not class level).
 */
function generalBenefits(characterLevel: number): string[] {
  const benefits: string[] = [];
  if (featLevels.has(characterLevel)) {
    benefits.push('Feat');
  }
  if (characterLevel % 4 === 0) {
    benefits.push('Ability score increase');
  }
  return benefits;
}

function makeLevelEntry(classId: string, classLabel: string, classLevel: number, characterLevel: number): LevelEntry {
  return {
    characterLevel,
    classId,
    classLabel,
    classLevel,
    skillPointsBase: classSkillPointsBase(classId),
    features: generalBenefits(characterLevel),
  };
}

/** Ordered entries for every class level already taken, numbered by character level. */
export function buildLevelEntries(heldClasses: HeldClass[]): LevelEntry[] {
  const entries: LevelEntry[] = [];
  let characterLevel = 0;
  for (const held of heldClasses) {
    for (let classLevel = 1; classLevel <= held.level; classLevel += 1) {
      characterLevel += 1;
      entries.push(makeLevelEntry(held.classId, held.classLabel, classLevel, characterLevel));
    }
  }
  return entries;
}

/**
 * One level's gains as a line for the Create screen's per-level summary (Brief 2 L4):
 * `Monk 1: +1 feat, +5 skills`, `Fighter 2: +3 skills, +1 ability score`. Skill ranks are
 * `totalSkillPoints` for this class (roster base + Int modifier, +1 for a human, floored at 1);
 * `skills Unknown` when the roster states no base. Class features are not part of this line.
 */
export function describeLevelGains(entry: LevelEntry, intelligenceModifier: number, isHuman: boolean): string {
  const gains: string[] = [];
  if (entry.features.includes('Feat')) {
    gains.push('+1 feat');
  }
  const skills = totalSkillPoints(entry.skillPointsBase, intelligenceModifier, isHuman);
  gains.push(skills === null ? 'skills Unknown' : `+${skills} skills`);
  if (entry.features.includes('Ability score increase')) {
    gains.push('+1 ability score');
  }
  return `${entry.classLabel} ${entry.classLevel}: ${gains.join(', ')}`;
}

/** The next level available for each held class — all at the next character level. */
export function buildNextEntries(heldClasses: HeldClass[]): LevelEntry[] {
  const totalLevel = heldClasses.reduce((sum, held) => sum + held.level, 0);
  return heldClasses.map((held) => makeLevelEntry(held.classId, held.classLabel, held.level + 1, totalLevel + 1));
}

/**
 * What taking the next character level in `classId` would grant, whether it's
 * a class the character already holds (levels up by one) or a brand-new class
 * (starts at class level 1) — either way at the next total character level.
 */
export function previewLevelUp(heldClasses: HeldClass[], classId: string): LevelEntry {
  const totalLevel = heldClasses.reduce((sum, held) => sum + held.level, 0);
  const held = heldClasses.find((entry) => entry.classId === classId);
  const classLabel = held?.classLabel ?? knownClass(classId)?.label ?? 'Adventurer';
  return makeLevelEntry(classId, classLabel, (held?.level ?? 0) + 1, totalLevel + 1);
}

/**
 * True when a level grants a feat pick, so `LevelUpDialog`'s accept flow
 * knows to collect a real one before persisting the level-up.
 *
 * Two independent sources, OR'd:
 *
 *   - `features` — the character-level feat from `generalBenefits`. A
 *     PF1 general rule the engine serves, not class-table data.
 *   - `engineGrantNames` — the grant names Epic 7's real per-class
 *     level-up engine reports for this transition (`previewLevelUp`).
 *
 * **Known gap, deliberately not papered over.** Fighter's bonus combat
 * feat at every even class level does not reach either source today:
 * `level_up/fighter.rs`'s own module doc records that `pick_from_lists`
 * stays empty for its ten Bonus Feat slots (building a real candidate list
 * needs PF1 Combat-Feat eligibility filtering plus per-candidate
 * prerequisite evaluation, left as that cycle's `next_required_uplift`),
 * and `class_feature.fighter.level_N_bonus_feat` only fires *after*
 * `choice:fighter_bonus_feat_N` has been selected. The previous
 * hand-authored class-feature string `'Bonus combat feat'` covered this
 * by asserting a rule the engine never verified; re-adding it would be the
 * uncited-rules-data debt this module just shed. The gap belongs in the
 * engine, and closing it there fixes this call site for free.
 */
export function levelGrantsFeat(features: string[], engineGrantNames: readonly string[] = []): boolean {
  // `\bfeat\b`, not `/feat/i` -- "features" contains "feat" as a substring,
  // which a plain substring match would wrongly treat as a feat grant.
  const grantsFeat = (text: string) => /\bfeat\b/i.test(text);
  return features.some(grantsFeat) || engineGrantNames.some(grantsFeat);
}

/** One feat pool off the engine's `feats.budget.<pool>.{granted,spent,remaining}` rows. */
export interface FeatBudgetPool {
  pool: string;
  label: string;
  granted: number;
  spent: number;
  remaining: number;
}

export interface FeatBudgetView {
  pools: FeatBudgetPool[];
  /** How many held feats the pools cannot fit: the rows' negative `remaining`. */
  overBy: number;
  /** False when the save is over budget or no pool has room. True when the engine served no rows: the add path decides. */
  canAdd: boolean;
  /** Why Add is disabled, or `null` when it is not. */
  addBlockedReason: string | null;
}

const FEAT_BUDGET_ROW = /^feats\.budget\.(.+)\.(granted|spent|remaining)$/;

/** Reads the budget rows, in served order. Decides nothing the rows do not say. */
export function readFeatBudget(rows: ReadonlyArray<{ id: string; value: number }>): FeatBudgetView {
  const pools = new Map<string, FeatBudgetPool>();
  for (const row of rows) {
    const match = FEAT_BUDGET_ROW.exec(row.id);
    if (!match) {
      continue;
    }
    const pool = pools.get(match[1]) ?? { pool: match[1], label: match[1].replace(/_/g, ' '), granted: 0, spent: 0, remaining: 0 };
    pool[match[2] as 'granted' | 'spent' | 'remaining'] = row.value;
    pools.set(match[1], pool);
  }
  const list = [...pools.values()];
  const overBy = list.reduce((sum, pool) => sum + Math.max(0, -pool.remaining), 0);
  const hasRoom = list.some((pool) => pool.remaining > 0);
  const addBlockedReason =
    overBy > 0
      ? `Over budget by ${overBy}: remove a feat before adding another.`
      : list.length > 0 && !hasRoom
        ? 'No feat pool has room.'
        : null;
  return { pools: list, overBy, canAdd: addBlockedReason === null, addBlockedReason };
}

/** The pools any feat can fill: what Create lets the player pick. Class bonus feats are chosen on the sheet. */
const OPEN_FEAT_POOLS: ReadonlySet<string> = new Set(['level', 'racial', 'bonus']);

/** How many feats Create lets a draft pick, off its budget rows. */
export function openFeatSlots(rows: ReadonlyArray<{ id: string; value: number }>): number {
  return readFeatBudget(rows)
    .pools.filter((pool) => OPEN_FEAT_POOLS.has(pool.pool))
    .reduce((sum, pool) => sum + pool.granted, 0);
}

/** A word's id segments as a player reads them: `skill:basket_weaving` -> `Basket Weaving`. */
function titleFromId(id: string): string {
  const words = id.split(':').pop() ?? id;
  return words
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}

function skillName(skillId: string): string {
  return SKILLS.find((skill) => skillIdFor(skill.name) === skillId.toLowerCase())?.name ?? titleFromId(skillId);
}

function abilityIncreaseLine(choices: ProgressionChoicesDto): string | null {
  return choices.abilityIncreases.length === 0
    ? null
    : `Ability increase: ${choices.abilityIncreases.map((row) => `${titleFromId(row.ability)} +${row.points}`).join(', ')}`;
}

function choiceLines(choices: ProgressionChoicesDto): string[] {
  const lines: string[] = [];
  if (choices.feats.length > 0) {
    lines.push(`Feats: ${choices.feats.map((feat) => (feat.target ? `${feat.name} (${feat.target})` : feat.name)).join(', ')}`);
  }
  if (choices.spells.length > 0) {
    lines.push(`Spells learned: ${choices.spells.map((spell) => spell.id).join(', ')}`);
  }
  if (choices.ranks.length > 0) {
    const spent = choices.ranks.reduce((sum, row) => sum + row.ranks, 0);
    lines.push(`Ranks spent: ${spent} (${choices.ranks.map((row) => `${skillName(row.skillId)} ${row.ranks}`).join(', ')})`);
  }
  return lines;
}

/** What holds no level, in the same words as a level's lines. */
export function describeProgressionChoices(choices: ProgressionChoicesDto): string[] {
  const increase = abilityIncreaseLine(choices);
  return increase === null ? choiceLines(choices) : [...choiceLines(choices), increase];
}

function counted(count: number, singular: string): string[] {
  return count > 0 ? [`${count} ${singular}${count === 1 ? '' : 's'}`] : [];
}

/**
 * One row of `list_progression_ledger` as the drawer prints it. It reads the row and decides nothing:
 * a line appears when the row states something, and an ability increase the level grants with nothing
 * recording it says so rather than counting as unspent, since the sheet has no pick to spend it on.
 */
export function describeProgressionLevel(
  level: ProgressionLevelDto,
  heldClasses: readonly HeldClass[]
): { heading: string; classLevel: string; lines: string[] } {
  const classLabel = heldClasses.find((held) => held.classId === level.classId)?.classLabel ?? knownClass(level.classId)?.label ?? level.classId;
  const lines: string[] = [];
  if (level.hitDieResult !== null) {
    lines.push(`Hit die result: ${level.hitDieResult}`);
  }
  if (level.classFeatures.length > 0) {
    lines.push(`Class features: ${level.classFeatures.map((row) => (row.raised ? `${row.label} ${row.value}` : row.label)).join(', ')}`);
  }
  lines.push(...choiceLines(level));
  const increase = abilityIncreaseLine(level);
  if (increase !== null) {
    lines.push(increase);
  } else if (level.abilityIncreaseDue) {
    lines.push('Ability increase: none recorded');
  }
  const unspent = [...counted(level.unspent.feats, 'feat'), ...counted(level.unspent.spells, 'spell'), ...counted(level.unspent.ranks, 'skill rank')];
  if (unspent.length > 0) {
    lines.push(`Unspent: ${unspent.join(', ')}`);
  }
  return { heading: `Level ${level.characterLevel}`, classLevel: `${classLabel} ${level.classLevel}`, lines };
}

function parseOneClass(segment: string): HeldClass {
  const parts = segment.split(':');
  const level = Number(parts[parts.length - 1]) || 1;
  const classId = parts.slice(0, -1).join(':');
  const option = knownClass(classId);
  const derivedLabel = parts
    .slice(1, -1)
    .join(' ')
    .replace(/\b\w/g, (c) => c.toUpperCase());
  const classLabel = option?.label ?? (derivedLabel || 'Adventurer');
  return { classId, classLabel, level };
}

/**
 * Parse the classes and levels a character holds from its `classSummary`.
 * Single class: `class:fighter:1`. Multiclass: comma-separated segments,
 * e.g. `class:fighter:3,class:wizard:1` — matches `summarize_envelope`'s
 * own `.join(",")` in `character_hub.rs` exactly (verified against a real
 * multiclass save; the previous `/`-separator assumption here never
 * matched the actual wire format, so every multiclass character's HP,
 * skill points, caster level, and Progression rail silently computed
 * against a single garbled pseudo-class instead of the real two).
 */
export function parseHeldClasses(classSummary: string): HeldClass[] {
  return classSummary
    .split(',')
    .map((segment) => parseOneClass(segment.trim()))
    .filter((held) => held.classId);
}

/** Combined class label, e.g. `Fighter 3 / Wizard 1`. */
export function formatHeldClasses(classSummary: string): string {
  return parseHeldClasses(classSummary)
    .map((held) => `${held.classLabel} ${held.level}`)
    .join(' / ');
}

/** Total character level across all held classes. */
export function totalCharacterLevel(classSummary: string): number {
  return parseHeldClasses(classSummary).reduce((sum, held) => sum + held.level, 0);
}

/** The class's hit die, off the served roster; `null` when not known (never an assumed d8). */
export function classHitDie(classId: string): number | null {
  return knownClass(classId)?.hitDie ?? null;
}

/**
 * PF1 max HP: the very first character level takes the maximum hit die; every
 * level after takes the class hit die's average (half + 1). The constitution
 * modifier applies at every level. Floored at 1.
 *
 * `null` when any held class's hit die is not known (the roster is still loading, or the class
 * states none): the sheet then prints Unknown rather than a total built on a guessed die.
 */
export function maxHitPoints(heldClasses: HeldClass[], constitutionModifier: number): number | null {
  let hitPoints = 0;
  let isFirstLevel = true;
  for (const held of heldClasses) {
    const hitDie = classHitDie(held.classId);
    if (hitDie === null) {
      return null;
    }
    for (let levelIndex = 0; levelIndex < held.level; levelIndex += 1) {
      hitPoints += isFirstLevel ? hitDie : Math.floor(hitDie / 2) + 1;
      hitPoints += constitutionModifier;
      isFirstLevel = false;
    }
  }
  return Math.max(1, hitPoints);
}

/**
 * Max HP when the character carries saved hit point results (the Create screen's Levels list saves
 * the die result for every level). Each saved result counts as it was saved, plus the Constitution
 * modifier, at least 1 per level. A level the character holds beyond its saved results (gained at
 * level-up) takes the class die's average, and the order the classes were added in does not matter:
 * only how many saved results each class has. Results beyond the levels held are ignored.
 *
 * With no saved results this is exactly `maxHitPoints`. `null` when an unsaved level belongs to a
 * class whose hit die is not known (never a guessed die).
 */
export function maxHitPointsWithSavedLevels(
  heldClasses: HeldClass[],
  constitutionModifier: number,
  saved: ReadonlyArray<{ classId: string; value: number }>
): number | null {
  if (saved.length === 0) {
    return maxHitPoints(heldClasses, constitutionModifier);
  }
  let total = 0;
  for (const held of heldClasses) {
    const results = saved.filter((entry) => entry.classId === held.classId).slice(0, held.level);
    for (const result of results) {
      total += Math.max(1, result.value + constitutionModifier);
    }
    const unsaved = held.level - results.length;
    if (unsaved > 0) {
      const hitDie = classHitDie(held.classId);
      if (hitDie === null) {
        return null;
      }
      total += unsaved * Math.max(1, Math.floor(hitDie / 2) + 1 + constitutionModifier);
    }
  }
  return total;
}

/**
 * Total skill ranks per level for this character: base + Int modifier (+1 for humans), floored
 * at 1. `null` when the class's base is not known.
 */
export function totalSkillPoints(base: number | null, intelligenceModifier: number, isHuman: boolean): number | null {
  if (base === null) {
    return null;
  }
  return Math.max(1, base + intelligenceModifier + (isHuman ? 1 : 0));
}
