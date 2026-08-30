import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the character trait/drawback picker
 * (`apps/desktop/src-tauri/src/trait_picker.rs`, AT-34-E4-002).
 *
 * A single command: `list_available_character_traits` returns the real,
 * corpus-derived roster of traits this cycle's compute path genuinely
 * supports (`ultimate_campaign`'s 31 flat `BONUS:SKILL` traits) — every
 * option returned really does grant its stated skill bonus once selected
 * and submitted on `CreateCharacterRequest.selectedTraits`
 * (`trait_effects::skill_bonuses_from_traits`). No "resolve" step exists
 * yet (unlike `loadAlternateRacialTraits`'s pair): a flat skill trait has
 * no alternate-swap exclusivity or per-character rendered prose to
 * compute ahead of submission.
 */

export interface CharacterTraitOptionDto {
  /** Echoes back on `CreateCharacterRequest.selectedTraits` verbatim. */
  id: string;
  name: string;
  description: string;
  /** Display-name skill(s) this trait's bonus applies to, e.g. `['Acrobatics']`. */
  skills: string[];
  bonus: number;
}

export async function loadCharacterTraits(): Promise<CharacterTraitOptionDto[]> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading character traits');
  }

  try {
    return await invoke<CharacterTraitOptionDto[]>('list_available_character_traits');
  } catch (cause: unknown) {
    throw new Error(`Failed to load character traits: ${formatError(cause)}`);
  }
}
