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

/** One feat this character qualifies for at this level-up. */
export interface LevelUpOptionDto {
  /** `<book>:<kind>:<slug>` — the engine's own record id. */
  id: string;
  name: string;
  /**
   * The situational condition that prints on the line, when the gate included
   * the option situationally. `null` for an unconditional include.
   */
  condition: string | null;
}

/** One feat this character does not qualify for, and the requirement it failed. */
export interface LevelUpRefusedOptionDto {
  id: string;
  name: string;
  /**
   * The failing requirement in the rule's own words (`'requires Dodge'`).
   * Render verbatim — it is engine output, not text to paraphrase.
   */
  unmet: string;
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
  /**
   * The feat options THIS character qualifies for, filtered by the backend
   * against its own prerequisites (SD-34 `decisions.md §17`). The join runs on
   * the Rust side over `SheetRule.applies`; nothing here re-derives it.
   */
  featOptions: LevelUpOptionDto[];
  /**
   * The feat options this character does not qualify for, each carrying the
   * requirement it failed. Shown, not hidden: a player who cannot see why an
   * option is missing cannot plan toward it.
   */
  refusedFeatOptions: LevelUpRefusedOptionDto[];
  /**
   * Why both option lists are empty when the rules package could not be read.
   * `null` when it loaded — "unavailable" and "nothing qualifies" are
   * different claims and only the second is about the rules.
   */
  optionFilterUnavailableReason: string | null;
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
