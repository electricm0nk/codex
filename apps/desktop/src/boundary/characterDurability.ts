import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read/write desktop boundary over a saved character's live HP/durability
 * tracking. Persisted as an `hp.json` sidecar (see `load_character_durability`/
 * `adjust_character_hp` in `character_hub.rs`), mirroring the bio/money
 * sidecar-file pattern — except unlike bio/money, a rejection here is a real,
 * structural "this build isn't durability-supported" outcome (only
 * single-class Fighter/Wizard/Rogue at a recognized level), not an
 * expected empty-first-load state — callers should treat it as "not
 * available for this build" (same as the Defense tab's DR-absent case),
 * not as an unexpected failure to alarm the user with.
 *
 * `status` is a pre-computed label from the real PF1 injury/death rules
 * (`Normal` / `Staggered` / `Disabled` / `Unconscious` / `Dying` / `Dead`) —
 * safe to render directly, no client-side threshold re-derivation.
 */

export interface CharacterDurabilityDto {
  maxHp: number;
  currentHp: number;
  nonlethalDamage: number;
  status: string;
}

export async function loadCharacterDurability(characterId: string): Promise<CharacterDurabilityDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading character durability');
  }

  try {
    return await invoke<CharacterDurabilityDto>('load_character_durability', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(formatError(cause));
  }
}

/**
 * `deltaHp` is positive to heal, negative to take lethal damage;
 * `deltaNonlethal` is positive to take nonlethal damage, negative to
 * recover from it. Already atomic and delta-based server-side — clamps
 * `currentHp` at `maxHp` on healing and `nonlethalDamage` at 0 on recovery,
 * and reclassifies `status` in the same call. A "set HP to an absolute
 * value" control just computes `deltaHp = newValue - currentHp` before
 * calling this — still one round trip.
 */
export async function adjustCharacterHp(
  characterId: string,
  deltaHp: number,
  deltaNonlethal: number
): Promise<CharacterDurabilityDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for adjusting character HP');
  }

  try {
    return await invoke<CharacterDurabilityDto>('adjust_character_hp', {
      request: { characterId, deltaHp, deltaNonlethal },
    });
  } catch (cause: unknown) {
    throw new Error(formatError(cause));
  }
}
