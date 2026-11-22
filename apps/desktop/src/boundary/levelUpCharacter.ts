import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';
import type { SkillAllocationEntryDto } from './setSkillAllocations';

/**
 * Write desktop boundary for leveling up a saved character.
 *
 * Invokes the `level_up_character` Tauri command, which increments (or
 * adds, for a fresh multiclass dip) the requested class's level on the
 * saved character, records any additional level-up choices (a hit-die
 * roll/average record, feat picks at feat-gaining levels) and an optional
 * skill-allocation replacement in the same mutation, recomputes via the
 * real rules-core engine, and either re-saves and returns the fresh
 * `Saved` envelope or leaves the on-disk character untouched and returns
 * `Blocked` with the real diagnostics — mirrors `create_character`'s
 * "never persist an unproven build" invariant. Returns the same tagged
 * `CreateCharacterOutcome` union `create_character` / `clone_character`
 * return, so callers can share one outcome-mapping path.
 */

/** Mirrors `SelectedChoiceDto` in `character_hub.rs`. Both fields must carry exactly the colon grammar `SavedCharacterStore::save`'s validator enforces — see `apply_level_up_choices`'s doc comment. */
export interface SelectedChoiceEntryDto {
  choiceSetId: string;
  selectionId: string;
}

export interface LevelUpCharacterRequest {
  characterId: string;
  classId: string;
  /** Empty by default — omit for the bare level increment with no additional recorded choices. */
  additionalChoices?: SelectedChoiceEntryDto[];
  /** When present, replaces `chosen.skill_allocations` wholesale (same semantics as `setSkillAllocations`). Omit to leave existing allocations untouched. */
  skillAllocations?: SkillAllocationEntryDto[];
  savedAt: string;
}

export async function levelUpCharacter(
  request: LevelUpCharacterRequest
): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for leveling up a character');
  }

  try {
    return await invoke<CreateCharacterOutcome>('level_up_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to level up character: ${formatError(cause)}`);
  }
}
