import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the engine's per-class sheet facts (SD-36 Epic F6a).
 *
 * Invokes `list_class_facts` (`src-tauri/src/class_facts.rs`), which answers, for each
 * `(class id, class level)` a character holds, what the engine says about that class: weapon
 * proficiency (the static `CLASS_WEAPON_PROFICIENCIES` row, else the converted-record reader),
 * caster level (the class's own caster-level rule, evaluated), class skills (the class-skill
 * reader) and the hit die the hit-point fold reads. The desktop keeps no class table of its own;
 * an answer the engine cannot give arrives as `status: 'unknown'` with its reason.
 */

export interface WeaponSetDto {
  label: string;
  members: string[];
}

export interface WeaponProficiencyFactsDto {
  status: 'known' | 'unknown';
  source: 'staticRow' | 'convertedRecord' | null;
  /** `Simple`, `Martial`, `Exotic`, each at most once. */
  tiers: string[];
  named: string[];
  groups: string[];
  sets: WeaponSetDto[];
  /** Printed, never counted: gated grants and picks the class level cannot settle. */
  printed: string[];
  reason: string | null;
}

export interface CasterLevelFactsDto {
  status: 'caster' | 'notACaster' | 'unknown';
  value: number | null;
  /** The rule id stating it (`caster`), or the reason (`notACaster` / `unknown`). */
  source: string;
}

export interface ClassSkillFactsDto {
  status: 'known' | 'unknown';
  /** Converted-package skill ids (`climb`, `knowledge_nature`). */
  skills: string[];
  /** Whole families (`Craft`, `Knowledge`). */
  groups: string[];
  reason: string | null;
}

export interface ClassFactsDto {
  classId: string;
  level: number;
  weaponProficiency: WeaponProficiencyFactsDto;
  casterLevel: CasterLevelFactsDto;
  classSkills: ClassSkillFactsDto;
  hitDie: number | null;
}

export interface ListClassFactsResponse {
  /** One entry per requested class, in request order. */
  classes: ClassFactsDto[];
}

export async function listClassFacts(classes: Array<{ classId: string; level: number }>): Promise<ListClassFactsResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading class facts');
  }
  try {
    return await invoke<ListClassFactsResponse>('list_class_facts', { request: { classes } });
  } catch (cause: unknown) {
    throw new Error(`Failed to load class facts: ${formatError(cause)}`);
  }
}
