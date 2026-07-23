import type { AbilityScoresDto } from '../boundary/loadCreateCharacter';
import { buildLevelEntries, totalSkillPoints, type HeldClass } from './characterProgression';

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

/** PF1 core rulebook class skill lists, by class id (see characterHubModel's CLASS_OPTIONS). */
const CLASS_SKILLS: Record<string, ReadonlySet<string>> = {
  'class:barbarian': new Set([
    'Acrobatics', 'Climb', 'Craft', 'Handle Animal', 'Intimidate', 'Knowledge (Nature)', 'Perception', 'Ride', 'Survival', 'Swim',
  ]),
  'class:bard': new Set([
    'Acrobatics', 'Appraise', 'Bluff', 'Climb', 'Craft', 'Diplomacy', 'Disguise', 'Escape Artist', 'Fly', 'Handle Animal',
    'Knowledge (Arcana)', 'Knowledge (Dungeoneering)', 'Knowledge (Engineering)', 'Knowledge (Geography)', 'Knowledge (History)',
    'Knowledge (Local)', 'Knowledge (Nature)', 'Knowledge (Nobility)', 'Knowledge (Planes)', 'Knowledge (Religion)', 'Linguistics',
    'Perception', 'Perform', 'Profession', 'Sense Motive', 'Sleight of Hand', 'Spellcraft', 'Stealth', 'Use Magic Device',
  ]),
  'class:cleric': new Set([
    'Appraise', 'Craft', 'Diplomacy', 'Heal', 'Knowledge (Arcana)', 'Knowledge (History)', 'Knowledge (Nobility)',
    'Knowledge (Planes)', 'Knowledge (Religion)', 'Linguistics', 'Profession', 'Sense Motive', 'Spellcraft',
  ]),
  'class:druid': new Set([
    'Climb', 'Craft', 'Fly', 'Handle Animal', 'Heal', 'Knowledge (Geography)', 'Knowledge (Nature)', 'Perception', 'Profession',
    'Ride', 'Spellcraft', 'Survival', 'Swim',
  ]),
  'class:fighter': new Set([
    'Climb', 'Craft', 'Handle Animal', 'Intimidate', 'Knowledge (Dungeoneering)', 'Knowledge (Engineering)', 'Profession', 'Ride',
    'Survival', 'Swim',
  ]),
  'class:monk': new Set([
    'Acrobatics', 'Climb', 'Craft', 'Escape Artist', 'Handle Animal', 'Intimidate', 'Knowledge (History)', 'Knowledge (Religion)',
    'Perception', 'Profession', 'Ride', 'Sense Motive', 'Stealth', 'Swim',
  ]),
  'class:paladin': new Set([
    'Craft', 'Diplomacy', 'Handle Animal', 'Heal', 'Knowledge (Nobility)', 'Knowledge (Religion)', 'Profession', 'Ride',
    'Sense Motive', 'Spellcraft',
  ]),
  'class:ranger': new Set([
    'Climb', 'Craft', 'Handle Animal', 'Heal', 'Intimidate', 'Knowledge (Dungeoneering)', 'Knowledge (Geography)',
    'Knowledge (Nature)', 'Perception', 'Profession', 'Ride', 'Spellcraft', 'Stealth', 'Survival', 'Swim',
  ]),
  'class:rogue': new Set([
    'Acrobatics', 'Appraise', 'Bluff', 'Climb', 'Craft', 'Diplomacy', 'Disable Device', 'Disguise', 'Escape Artist',
    'Handle Animal', 'Intimidate', 'Knowledge (Dungeoneering)', 'Knowledge (Local)', 'Linguistics', 'Perception', 'Perform',
    'Profession', 'Ride', 'Sense Motive', 'Sleight of Hand', 'Stealth', 'Swim', 'Use Magic Device',
  ]),
  'class:sorcerer': new Set([
    'Appraise', 'Bluff', 'Craft', 'Fly', 'Intimidate', 'Knowledge (Arcana)', 'Profession', 'Spellcraft', 'Use Magic Device',
  ]),
  'class:wizard': new Set([
    'Appraise', 'Craft', 'Fly', 'Knowledge (Arcana)', 'Knowledge (Dungeoneering)', 'Knowledge (Engineering)',
    'Knowledge (Geography)', 'Knowledge (History)', 'Knowledge (Local)', 'Knowledge (Nature)', 'Knowledge (Nobility)',
    'Knowledge (Planes)', 'Knowledge (Religion)', 'Linguistics', 'Profession', 'Spellcraft',
  ]),
};

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

/** Whether `skillName` is a class skill for any class the character holds (multiclass union). */
export function isClassSkill(heldClasses: HeldClass[], skillName: string): boolean {
  return heldClasses.some((held) => CLASS_SKILLS[held.classId]?.has(skillName));
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

/** Total skill points earned across every class level already taken. */
export function totalSkillPointsAvailable(heldClasses: HeldClass[], intelligenceModifier: number, isHuman: boolean): number {
  return buildLevelEntries(heldClasses).reduce(
    (sum, entry) => sum + totalSkillPoints(entry.skillPointsBase, intelligenceModifier, isHuman),
    0
  );
}
