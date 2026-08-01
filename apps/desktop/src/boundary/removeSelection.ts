import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for the three selection-removal commands — the
 * inverses of `addFeatSelection`, `addSpellSelection` /
 * `recordAndPrepareSpellSelection`, and `addEquipmentSelection` /
 * `purchaseEquipment`.
 *
 * All three invoke a real Tauri command that loads the saved character,
 * removes the named selection, recomputes via the real rules-core engine,
 * and either re-saves and returns the fresh `Saved` envelope or leaves the
 * on-disk character untouched and returns `Blocked` with the engine's own
 * diagnostics — the same invariant the add commands hold, and the reason a
 * removal can never leave a stale computed value on disk.
 *
 * A removal the backend refuses outright (the character does not hold the
 * selection; another held feat's prerequisites depend on it) throws with
 * the backend's own reason. Refusals are errors rather than `Blocked`
 * because they are decisions about the request, not about whether the
 * resulting build computes.
 */

export interface RemoveFeatSelectionRequest {
  characterId: string;
  featId: string;
  /**
   * Which recorded target this removal takes with the feat — a weapon,
   * skill or school, WITHOUT a prefix, exactly as `addFeatSelection` takes
   * it. Omit to remove one held copy and, if it was the last, every target
   * that feat's chooser set still held.
   */
  target?: string | null;
  savedAt: string;
}

export async function removeFeatSelection(
  request: RemoveFeatSelectionRequest,
): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for removing a feat selection');
  }

  try {
    return await invoke<CreateCharacterOutcome>('remove_feat_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to remove feat selection: ${formatError(cause)}`);
  }
}

export interface RemoveSpellSelectionRequest {
  characterId: string;
  spellId: string;
  sourceClassId: string;
  savedAt: string;
}

export async function removeSpellSelection(
  request: RemoveSpellSelectionRequest,
): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for removing a spell selection');
  }

  try {
    return await invoke<CreateCharacterOutcome>('remove_spell_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to remove spell selection: ${formatError(cause)}`);
  }
}

export interface RemoveEquipmentSelectionRequest {
  characterId: string;
  itemId: string;
  savedAt: string;
}

export async function removeEquipmentSelection(
  request: RemoveEquipmentSelectionRequest,
): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for removing an equipment selection');
  }

  try {
    return await invoke<CreateCharacterOutcome>('remove_equipment_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to remove equipment selection: ${formatError(cause)}`);
  }
}
