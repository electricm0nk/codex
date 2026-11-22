import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for atomically recording AND preparing a spell for
 * a Wizard-sourced class in one mutation.
 *
 * Invokes the `record_and_prepare_spell_selection` Tauri command, which
 * appends both a `Known` and a `Prepared` `SpellSelection` entry for the
 * same spell in a single mutation — breaks the bootstrap deadlock plain
 * `add_spell_selection` cannot: `unmet_wizard_spellbook_conditions`
 * requires a non-empty recorded (Known) set AND a non-empty prepared set
 * simultaneously, so no sequence of single-mode `add_spell_selection` calls
 * can ever satisfy it (each call is independently gated on reaching
 * `Computed` before persisting, and neither mode alone does). Same
 * `CreateCharacterOutcome` union every other mutation command returns.
 */

export interface RecordAndPrepareSpellSelectionRequest {
  characterId: string;
  spellId: string;
  sourceClassId: string;
  savedAt: string;
}

export async function recordAndPrepareSpellSelection(
  request: RecordAndPrepareSpellSelectionRequest
): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for recording and preparing a spell selection');
  }

  try {
    return await invoke<CreateCharacterOutcome>('record_and_prepare_spell_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to record and prepare spell selection: ${formatError(cause)}`);
  }
}
