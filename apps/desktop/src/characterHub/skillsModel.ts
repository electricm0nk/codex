import type { AbilityScoresDto } from '../boundary/loadCreateCharacter';
import type { ClassSkillFactsDto } from '../boundary/listClassFacts';
import { buildLevelEntries, totalSkillPoints, type HeldClass } from './characterProgression';
import type { ClassFactsState } from './classFactsModel';

/** The full PF1 core rulebook skill list with governing ability. */
export const SKILLS: ReadonlyArray<{ name: string; ability: keyof AbilityScoresDto }> = [
  { name: 'Acrobatics', ability: 'dexterity' },
  { name: 'Appraise', ability: 'intelligence' },
  { name: 'Bluff', ability: 'charisma' },
  { name: 'Climb', ability: 'strength' },
  { name: 'Craft', ability: 'intelligence' },
  { name: 'Diplomacy', ability: 'charisma' },
  { name: 'Disable Device', ability: 'dexterity' },
  { name: 'Disguise', ability: 'charisma' },
  { name: 'Escape Artist', ability: 'dexterity' },
  { name: 'Fly', ability: 'dexterity' },
  { name: 'Handle Animal', ability: 'charisma' },
  { name: 'Heal', ability: 'wisdom' },
  { name: 'Intimidate', ability: 'charisma' },
  { name: 'Knowledge (Arcana)', ability: 'intelligence' },
  { name: 'Knowledge (Dungeoneering)', ability: 'intelligence' },
  { name: 'Knowledge (Engineering)', ability: 'intelligence' },
  { name: 'Knowledge (Geography)', ability: 'intelligence' },
  { name: 'Knowledge (History)', ability: 'intelligence' },
  { name: 'Knowledge (Local)', ability: 'intelligence' },
  { name: 'Knowledge (Nature)', ability: 'intelligence' },
  { name: 'Knowledge (Nobility)', ability: 'intelligence' },
  { name: 'Knowledge (Planes)', ability: 'intelligence' },
  { name: 'Knowledge (Religion)', ability: 'intelligence' },
  { name: 'Linguistics', ability: 'intelligence' },
  { name: 'Perception', ability: 'wisdom' },
  { name: 'Perform', ability: 'charisma' },
  { name: 'Profession', ability: 'wisdom' },
  { name: 'Ride', ability: 'dexterity' },
  { name: 'Sense Motive', ability: 'wisdom' },
  { name: 'Sleight of Hand', ability: 'dexterity' },
  { name: 'Spellcraft', ability: 'intelligence' },
  { name: 'Stealth', ability: 'dexterity' },
  { name: 'Survival', ability: 'wisdom' },
  { name: 'Swim', ability: 'strength' },
  { name: 'Use Magic Device', ability: 'charisma' },
];

/**
 * Maps a `SKILLS` display name to the `skill:<snake_case>` wire id the
 * `set_skill_allocations` Tauri command expects (`SkillAllocation.skill_id`
 * in `character_input.rs`). Only 5 ids are actually recognized by the
 * compute engine today (`skill:climb`, `skill:swim`, `skill:intimidate`,
 * `skill:diplomacy`, `skill:disable_device` — see
 * `src/rules_core/skill_allocation.rs`'s `skill_key_ability_modifier`), and
 * those 5 confirm this exact convention (lowercase, spaces/parens to
 * underscores). The other 30 ids are this same convention extended by
 * inference, not confirmed against any canonical backend list — backend
 * flagged the same uncertainty from their side when they shipped the
 * command. Unrecognized ids are inert on the backend (no modifier
 * fabricated, no rejection), so sending them is safe either way.
 */
export function skillIdFor(skillName: string): string {
  const normalized = skillName
    .toLowerCase()
    .replace(/[()]/g, '')
    .trim()
    .replace(/[^a-z0-9]+/g, '_')
    .replace(/^_+|_+$/g, '');
  return `skill:${normalized}`;
}

/**
 * Whether the served class-skill answer grants `skillName` (a `SKILLS` display name): named
 * directly by its package id (`skillIdFor` without the `skill:` prefix), or a member of a granted
 * family (`Knowledge (Nature)` under `Knowledge`, `Craft` itself under `Craft`) -- the engine's own
 * `ClassSkillView::contains` rule.
 */
export function classSkillFactsGrant(facts: ClassSkillFactsDto, skillName: string): boolean {
  if (facts.status !== 'known') {
    return false;
  }
  const id = skillIdFor(skillName).slice('skill:'.length);
  return (
    facts.skills.includes(id) ||
    facts.groups.some((group) => {
      const family = group.toLowerCase();
      return id === family || id.startsWith(`${family}_`);
    })
  );
}

/**
 * SD-36 F6a: which skills are class skills for the classes a character holds, read from the
 * engine's class-skill reader (`list_class_facts`). PF1's union rule: a skill is a class skill when
 * ANY held class grants it. A held class the engine cannot answer contributes nothing and is named
 * in `unanswered` (the Skills panel prints it) -- never silently scored all-cross-class.
 */
export interface ClassSkillLookup {
  isClassSkill: (skillName: string) => boolean;
  /** `{ classLabel, reason }` for each held class with no served class-skill answer. */
  unanswered: Array<{ classLabel: string; reason: string }>;
}

export function classSkillLookup(heldClasses: readonly HeldClass[], state: ClassFactsState): ClassSkillLookup {
  const known: ClassSkillFactsDto[] = [];
  const unanswered: ClassSkillLookup['unanswered'] = [];
  for (const held of heldClasses) {
    const served = state.kind === 'loaded' ? state.byClassId.get(held.classId) : undefined;
    const facts = served && served.level === held.level ? served.classSkills : undefined;
    if (facts && facts.status === 'known') {
      known.push(facts);
    } else {
      const reason =
        facts?.reason ?? (state.kind === 'loading' ? 'loading' : state.kind === 'failed' ? state.notice : 'not served');
      unanswered.push({ classLabel: held.classLabel, reason });
    }
  }
  return {
    isClassSkill: (skillName) => known.some((facts) => classSkillFactsGrant(facts, skillName)),
    unanswered,
  };
}

/** Whether `skillName` is a class skill under `lookup` (see {@link classSkillLookup}). */
export function isClassSkill(lookup: ClassSkillLookup, skillName: string): boolean {
  return lookup.isClassSkill(skillName);
}

/** PF1: a skill's total modifier is ability mod + ranks + (a +3 class-skill bonus once at least 1 rank is invested). */
export function skillModifier(abilityModifier: number, ranks: number, classSkill: boolean): number {
  return abilityModifier + ranks + (classSkill && ranks > 0 ? 3 : 0);
}

/** Max ranks investable in a class skill at the given total character level. */
export function maxClassSkillRanks(characterLevel: number): number {
  return characterLevel + 3;
}

/** Max ranks investable in a cross-class skill — half the class-skill max, per PF1 core rules. */
export function maxCrossClassSkillRanks(characterLevel: number): number {
  return Math.floor((characterLevel + 3) / 2);
}

/** Points cost per rank: 1 for a class skill, 2 for cross-class. */
export function skillRankCost(classSkill: boolean): number {
  return classSkill ? 1 : 2;
}

/**
 * The fixed three-skill demo allocation every saved character currently
 * receives server-side (`compose_character_input` in character_hub.rs hard-
 * codes Climb/Intimidate/Swim at 1 rank each, regardless of the caller's
 * choices — there is no per-character allocation command yet). Used to seed
 * the allocation dialog with what's actually true today rather than a guess.
 */
export const DEFAULT_SKILL_ALLOCATION: Record<string, number> = {
  Climb: 1,
  Intimidate: 1,
  Swim: 1,
};

/**
 * Total skill points earned across every class level already taken; `null` when any level's
 * class states no skill ranks (the roster is loading, or the record states none).
 */
export function totalSkillPointsAvailable(heldClasses: HeldClass[], intelligenceModifier: number, isHuman: boolean): number | null {
  let total = 0;
  for (const entry of buildLevelEntries(heldClasses)) {
    const points = totalSkillPoints(entry.skillPointsBase, intelligenceModifier, isHuman);
    if (points === null) {
      return null;
    }
    total += points;
  }
  return total;
}
