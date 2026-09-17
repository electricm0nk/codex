import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for adding a character trait/drawback selection to a
 * saved character.
 *
 * Invokes the `add_trait_selection` Tauri command, which loads the saved
 * character, records the trait (with its skill choice for a `%LIST` trait),
 * recomputes via the real rules-core engine, and either re-saves and returns
 * the fresh `Saved` envelope or leaves the on-disk character untouched and
 * returns `Blocked` with the real diagnostics — same invariant as
 * `addFeatSelection`/`addEquipmentSelection`.
 */

export interface AddTraitSelectionRequest {
  characterId: string;
  /** `trait_effects`' own wire id (e.g. `"trait:trait_acrobat"`). */
  traitId: string;
  /**
   * For a `%LIST` trait (a picker option carrying a `choiceSetId`), the
   * chosen skill id from that option's `skillOptions`. Omit for a flat trait.
   */
  skillChoice?: string | null;
  savedAt: string;
}

export async function addTraitSelection(request: AddTraitSelectionRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for adding a trait selection');
  }

  try {
    return await invoke<CreateCharacterOutcome>('add_trait_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to add trait selection: ${formatError(cause)}`);
  }
}
