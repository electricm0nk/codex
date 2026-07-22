import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Write desktop boundary for `recompute_character` (SD-24 Epic 7 Criterion
 * 7.2; routed through the rule-system-adapter dispatch seam by SD-25
 * Criterion 3.4). Loads the saved character named by `characterId` from
 * disk and returns its freshly recomputed derived stats — never mutates or
 * re-saves the on-disk envelope.
 *
 * `ruleSystemId` is the wire-level id `characterHubRuntime.ts`'s
 * `resolveRuleSystemId` resolves from the panel's active `RuleSetId` — the
 * real UI call site closing SD-25 Criterion 3.5's register A3 obligation
 * (a real UI affordance wired to one of `append_to_character` /
 * `recompute_character` / `re_save_character`, matching SD-24 Criterion
 * 7.4's Add-Weapon/Add-Armor/Add-Spell precedent).
 */

export interface RecomputeCharacterRequest {
  characterId: string;
  ruleSystemId: string;
}

/** Mirrors `BaseSavesDto` in `characterHub/recomputeCharacter.rs`. */
export interface RecomputeBaseSavesDto {
  fortitude: number;
  reflex: number;
  will: number;
}

/** Mirrors `CharacterSnapshotDto` in `characterHub/recomputeCharacter.rs`. */
export interface RecomputedCharacterSnapshotDto {
  characterId: string;
  baseAttackBonus: number;
  baseSaves: RecomputeBaseSavesDto;
  baselineMeleeAttackBonus: number;
  baselineArmorClass: number;
  totalSaves: RecomputeBaseSavesDto;
}

export interface RecomputeCharacterResponse {
  success: boolean;
  character: RecomputedCharacterSnapshotDto | null;
  error: string | null;
}

export async function recomputeCharacter(
  request: RecomputeCharacterRequest
): Promise<RecomputeCharacterResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for recomputing a character');
  }

  try {
    return await invoke<RecomputeCharacterResponse>('recompute_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to recompute character: ${formatError(cause)}`);
  }
}
