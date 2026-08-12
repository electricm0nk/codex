import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for replacing a saved character's skill-point
 * allocation.
 *
 * Invokes the `set_skill_allocations` Tauri command, which replaces
 * `chosen.skill_allocations` wholesale (not an append — callers must send
 * the complete allocation set every time), recomputes via the real
 * rules-core engine, and either re-saves and returns the fresh `Saved`
 * envelope or leaves the on-disk character untouched and returns `Blocked`
 * with the real diagnostics — same invariant as `levelUpCharacter` /
 * `addSpellSelection`. Returns the same tagged `CreateCharacterOutcome`
 * union those commands return, so callers share one outcome-mapping path
 * via `toCharacterMutationRefresh`.
 */

export interface SkillAllocationEntryDto {
  skillId: string;
  ranks: number;
}

export interface SetSkillAllocationsRequest {
  characterId: string;
  skillAllocations: SkillAllocationEntryDto[];
  savedAt: string;
}

export async function setSkillAllocations(
  request: SetSkillAllocationsRequest
): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for setting skill allocations');
  }

  try {
    return await invoke<CreateCharacterOutcome>('set_skill_allocations', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to set skill allocations: ${formatError(cause)}`);
  }
}
