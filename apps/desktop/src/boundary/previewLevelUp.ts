import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over "what does the next level in this class
 * grant?".
 *
 * Invokes the `preview_level_up` Tauri command, a pass-through to Epic 7's
 * real per-class level-up engine
 * (`level_up::compute_level_up_grants_for_class`). Persists nothing.
 *
 * This replaces the hand-authored class-feature table that used to live
 * in `characterProgression.ts` — bare labels (`'Bravery +1'`,
 * `'Bonus combat feat'`) with no magnitudes and no provenance, duplicating
 * and drifting from the engine's own grounded class tables.
 *
 * An empty `automaticFeatures` is a real answer, not a failure. The
 * per-class level-up modules are individually gated, and the command
 * returns an honestly-empty plan for any class outside the eleven PF1 Core
 * classes the engine grounds. Render that absence as absence.
 */

export interface PreviewLevelUpRequest {
  characterId: string;
  classId: string;
}

export interface LevelUpGrantEffectDto {
  /** The engine's own description text — render verbatim. */
  description: string;
  value: number;
}

export interface LevelUpGrantDto {
  name: string;
  effects: LevelUpGrantEffectDto[];
}

export interface LevelUpPickCandidateDto {
  id: string;
  name: string;
}

export interface LevelUpPickListDto {
  /** `'Feat'`, `'Spell'` or `'RagePower'`. */
  category: string;
  count: number;
  candidates: LevelUpPickCandidateDto[];
  filter: string | null;
}

export interface LevelUpResourcePoolDeltaDto {
  poolId: string;
  fromValue: number;
  toValue: number;
}

export interface PreviewLevelUpResponse {
  /** The class's own level before this transition (0 for a fresh dip). */
  fromLevel: number;
  toLevel: number;
  /** The character's total level after this transition. */
  characterLevel: number;
  automaticFeatures: LevelUpGrantDto[];
  pickFromLists: LevelUpPickListDto[];
  resourcePoolChanges: LevelUpResourcePoolDeltaDto[];
  capstoneThreshold: boolean;
}

export async function previewLevelUp(
  request: PreviewLevelUpRequest
): Promise<PreviewLevelUpResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for previewing a level up');
  }

  try {
    return await invoke<PreviewLevelUpResponse>('preview_level_up', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to preview level up: ${formatError(cause)}`);
  }
}
