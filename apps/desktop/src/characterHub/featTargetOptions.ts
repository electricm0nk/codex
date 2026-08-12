import type { WeaponTargetDto } from '../boundary/listWeaponTargets';
import type { ItemPickerEntry } from './ItemPickerModal';
import { SKILLS } from './skillsModel';

/**
 * The option lists offered for the "which target?" step of adding a chooser
 * feat, one per `ChooserTargetKind` the engine reports.
 *
 * Each list is the real set the engine can honour, not a convenient subset:
 * a target the player can pick but no producer reads would record a choice
 * that silently computes nothing.
 */

/**
 * The eight PF1 schools of magic.
 *
 * Universal is deliberately absent. Spell Focus's corpus token is
 * `CHOOSE:SCHOOLS|ABILITY=FEAT[Spell Focus]` over the eight real schools;
 * Universal is not a school a spell can be focused in, so offering it would
 * let a player record a target that grounds nothing.
 */
export const SPELL_SCHOOLS: readonly string[] = [
  'Abjuration',
  'Conjuration',
  'Divination',
  'Enchantment',
  'Evocation',
  'Illusion',
  'Necromancy',
  'Transmutation',
];

export function weaponTargetOptions(weapons: WeaponTargetDto[]): ItemPickerEntry[] {
  return weapons.map((weapon) => ({ key: weapon.key, name: weapon.key, detail: weapon.detail }));
}

export function skillTargetOptions(): ItemPickerEntry[] {
  return SKILLS.map((skill) => ({
    key: skill.name,
    name: skill.name,
    detail: `Governed by ${skill.ability}`,
  }));
}

export function spellSchoolTargetOptions(): ItemPickerEntry[] {
  return SPELL_SCHOOLS.map((school) => ({ key: school, name: school, detail: 'School of magic' }));
}

/**
 * A human-readable prompt naming both the feat and what it needs, so the
 * second picker step never reads as an unexplained second dialog.
 */
export function featTargetPickerTitle(featName: string, targetKind: string): string {
  switch (targetKind) {
    case 'Weapon':
      return `${featName} — choose a weapon`;
    case 'Skill':
      return `${featName} — choose a skill`;
    case 'SpellSchool':
      return `${featName} — choose a school`;
    default:
      return `${featName} — choose a target`;
  }
}
