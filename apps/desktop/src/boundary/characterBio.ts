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
  /** The human at the table playing this character (v0.8 B-1 / F-14). Wire key `playerName`. */
  playerName: string;
  alignment: string;
  deity: string;
  sex: string;
  age: string;
  height: string;
  weight: string;
  hair: string;
  eyes: string;
  /** SD-40 E2.27: the languages the player recorded, in order; the engine's `languages.known` reads it (none → Common). */
  languages: string[];
  /** SD-40 E3.6: the Biography tab's free text (story); printed with the Bio section. */
  biography: string;
  /** SD-40 E3.6: the Notes tab's free text (notes to self, table rulings). */
  notes: string;
}

export const BLANK_CHARACTER_BIO: CharacterBioDto = {
  playerName: '',
  alignment: '',
  deity: '',
  sex: '',
  age: '',
  height: '',
  weight: '',
  hair: '',
  eyes: '',
  languages: [],
  biography: '',
  notes: '',
};

/** The Languages field's comma list as the stored list: trimmed names, blanks dropped. */
export function parseLanguages(text: string): string[] {
  return text.split(',').map((name) => name.trim()).filter((name) => name !== '');
}

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
