import { CLASS_OPTIONS } from './characterHubModel';

/**
 * Pathfinder 1e level progression: per-class skill points and class features,
 * plus the universal PF1 benefits (a feat at every odd level, an ability score
 * increase every 4th level). Used by the character sheet to list what each
 * current level granted and what the next level of each held class offers.
 */

/** Base skill ranks per level for each class, before the Intelligence modifier. */
const CLASS_SKILL_POINTS: Record<string, number> = {
  'class:arcanist': 3,
  'class:barbarian': 4,
  'class:bard': 6,
  'class:cleric': 2,
  'class:druid': 4,
  'class:fighter': 2,
  'class:monk': 4,
  'class:paladin': 2,
  'class:ranger': 6,
  'class:rogue': 8,
  'class:sorcerer': 2,
  'class:wizard': 2,
  /*
   * Pathfinder Unchained (SD-27, 2026-07-31). Each is its base class's
   * value, and that is a corpus fact rather than a copy: none of the four
   * `data/corpus/pathfinder_unchained/class/*.json` records carries a
   * `STARTSKILLPTS` token, i.e. the selection ability overrides no skill
   * ranks per level, so the base `CLASS:` record's own value stands.
   *
   * `class:unchained_summoner` is deliberately absent: the APG Summoner it
   * replaces is itself absent from this table and falls through to the
   * default of 2, which is its correct value. Adding one and not the other
   * would make the pair disagree for no reason.
   */
  'class:unchained_barbarian': 4,
  'class:unchained_monk': 4,
  'class:unchained_rogue': 8,
};

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
 * points per class, the universal PF1 benefits (a feat at every odd
 * character level, an ability score increase every 4th), class-summary
 * parsing, and hit points.
 */

export interface WeaponProficiency {
  simple: boolean;
  martial: boolean;
  exotic: boolean;
}

// PF1 martial-weapon classes. Exotic weapons always require a feat, so no class
// grants them by default. Restricted-list casters (wizard, druid, monk) are
// approximated as simple-proficient at the category level.
// `class:unchained_barbarian` joins the set on the same evidence as the
// rest: its own corpus proficiency record grants `Weapon Prof ~ Simple`
// AND `Weapon Prof ~ Martial` (SD-27, 2026-07-31). The other three
// Unchained classes are deliberately absent -- the Unchained Monk and
// Rogue and Summoner grant no Martial tier, exactly like their namesakes.
const MARTIAL_WEAPON_CLASSES = new Set([
  'class:fighter',
  'class:barbarian',
  'class:paladin',
  'class:ranger',
  'class:unchained_barbarian',
]);

export function classWeaponProficiency(classId: string): WeaponProficiency {
  return { simple: true, martial: MARTIAL_WEAPON_CLASSES.has(classId), exotic: false };
}

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
  /** Base skill ranks for this class before the Intelligence modifier. */
  skillPointsBase: number;
  /**
   * The universal PF1 benefits this *character* level grants — a feat at
   * every odd level, an ability score increase every 4th. Both are general
   * rules keyed to total character level, not entries from any class table.
   *
   * Class features are deliberately not here: they come from the engine's
   * own `class_feature.*` / `class_chassis.*` records (see
   * `classFeaturesModel.ts`) rather than from a table hand-authored in the
   * frontend.
   */
  features: string[];
}

export function classSkillPointsBase(classId: string): number {
  return CLASS_SKILL_POINTS[classId] ?? 2;
}

/**
 * PF1 grants a feat at every odd character level and an ability score increase
 * every 4th character level (both keyed to total level, not class level).
 */
function generalBenefits(characterLevel: number): string[] {
  const benefits: string[] = [];
  if (characterLevel % 2 === 1) {
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
  const option = CLASS_OPTIONS.find((entry) => entry.id === classId);
  const classLabel = held?.classLabel ?? option?.label ?? 'Adventurer';
  return makeLevelEntry(classId, classLabel, (held?.level ?? 0) + 1, totalLevel + 1);
}

/**
 * True when a level grants a feat pick, so `LevelUpDialog`'s accept flow
 * knows to collect a real one before persisting the level-up.
 *
 * Two independent sources, OR'd:
 *
 *   - `features` — the universal odd-character-level feat from
 *     `generalBenefits`. A PF1 general rule, not class-table data.
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

function parseOneClass(segment: string): HeldClass {
  const parts = segment.split(':');
  const level = Number(parts[parts.length - 1]) || 1;
  const classId = parts.slice(0, -1).join(':');
  const option = CLASS_OPTIONS.find((entry) => entry.id === classId);
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

// PF1 full spellcasting classes — their levels sum into the caster level.
// Arcanist (ACG) is a full arcane caster like Wizard: caster level equals its
// class level, so it belongs here the moment it becomes selectable.
const CASTER_CLASSES = new Set([
  'class:wizard',
  'class:sorcerer',
  'class:cleric',
  'class:druid',
  'class:bard',
  'class:arcanist',
]);

/** Caster level: total levels in full spellcasting classes (0 for a non-caster). */
export function casterLevel(classSummary: string): number {
  return parseHeldClasses(classSummary)
    .filter((held) => CASTER_CLASSES.has(held.classId))
    .reduce((sum, held) => sum + held.level, 0);
}

export function classHitDie(classId: string): number {
  return CLASS_OPTIONS.find((option) => option.id === classId)?.hitDie ?? 8;
}

/**
 * PF1 max HP: the very first character level takes the maximum hit die; every
 * level after takes the class hit die's average (half + 1). The constitution
 * modifier applies at every level. Floored at 1.
 */
export function maxHitPoints(heldClasses: HeldClass[], constitutionModifier: number): number {
  let hitPoints = 0;
  let isFirstLevel = true;
  for (const held of heldClasses) {
    const hitDie = classHitDie(held.classId);
    for (let levelIndex = 0; levelIndex < held.level; levelIndex += 1) {
      hitPoints += isFirstLevel ? hitDie : Math.floor(hitDie / 2) + 1;
      hitPoints += constitutionModifier;
      isFirstLevel = false;
    }
  }
  return Math.max(1, hitPoints);
}

/** Total skill ranks per level for this character: base + Int modifier (+1 for humans), floored at 1. */
export function totalSkillPoints(base: number, intelligenceModifier: number, isHuman: boolean): number {
  return Math.max(1, base + intelligenceModifier + (isHuman ? 1 : 0));
}
