import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read/write desktop boundary over a saved character's bio/flavor fields
 * (alignment, deity, sex, age, height, weight, hair, eyes). Persisted as a
 * `bio.json` sidecar file alongside the character's existing saved files
 * (see `update_character_bio`/`load_character_bio` in `character_hub.rs`),
 * mirroring the portrait boundary's own sidecar-file pattern. Pure
 * passthrough — no rules-engine calculation reads any of these fields, so
 * there is no Blocked/diagnostics concept here, just success or a real I/O
 * error.
 */

export interface CharacterBioDto {
  alignment: string;
  deity: string;
  sex: string;
  age: string;
  height: string;
  weight: string;
  hair: string;
  eyes: string;
}

export const BLANK_CHARACTER_BIO: CharacterBioDto = {
  alignment: '',
  deity: '',
  sex: '',
  age: '',
  height: '',
  weight: '',
  hair: '',
  eyes: '',
};

/** Never throws for the common "no bio saved yet" case — resolves to all-empty fields instead, matching the Rust command's own default-when-absent behavior. Outside a Tauri runtime (browser preview), also resolves to the blank default rather than erroring. */
export async function loadCharacterBio(characterId: string): Promise<CharacterBioDto> {
  if (!hasTauriRuntime()) {
    return { ...BLANK_CHARACTER_BIO };
  }

  try {
    return await invoke<CharacterBioDto>('load_character_bio', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to load character bio: ${formatError(cause)}`);
  }
}

/** Always sends the character's complete bio field set (not a delta) — the caller's editor already holds every field's current value. */
export async function updateCharacterBio(characterId: string, bio: CharacterBioDto): Promise<void> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for saving a character bio');
  }

  try {
    await invoke('update_character_bio', { request: { characterId, bio } });
  } catch (cause: unknown) {
    throw new Error(`Failed to save character bio: ${formatError(cause)}`);
  }
}
