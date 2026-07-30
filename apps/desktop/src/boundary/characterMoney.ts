import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read/write desktop boundary over a saved character's spendable money
 * balance. Persisted as a `money.json` sidecar file (see
 * `load_character_money`/`adjust_character_money` in `character_hub.rs`),
 * mirroring the bio boundary's sidecar-file pattern. The canonical value is
 * `totalCopper`; `platinum`/`gold`/`silver`/`copper` are the same balance's
 * derived denomination breakdown, computed server-side via
 * `codex::rules_core::money::copper_to_denominations` so the frontend never
 * re-implements that conversion.
 */

export interface CharacterMoneyDto {
  totalCopper: number;
  platinum: number;
  gold: number;
  silver: number;
  copper: number;
}

const ZERO_MONEY: CharacterMoneyDto = { totalCopper: 0, platinum: 0, gold: 0, silver: 0, copper: 0 };

/** Standard PF1/d20 ratio (1gp = 100cp) — mirrors `money::gp_to_copper` on the Rust side exactly, since `adjust_character_money`'s wire request takes a copper delta, not gold. */
const COPPER_PER_GOLD = 100;

export function gpToCopper(valueInGp: number): number {
  return Math.round(valueInGp * COPPER_PER_GOLD);
}

/** Never throws for the common "no money saved yet" case — resolves to a zero balance instead, matching the Rust command's own default-when-absent behavior. Outside a Tauri runtime, also resolves to zero. */
export async function loadCharacterMoney(characterId: string): Promise<CharacterMoneyDto> {
  if (!hasTauriRuntime()) {
    return { ...ZERO_MONEY };
  }

  try {
    return await invoke<CharacterMoneyDto>('load_character_money', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to load character money: ${formatError(cause)}`);
  }
}

/** `deltaCopper` is positive to add funds, negative to spend. Rejects (with a real insufficient-funds message) rather than allowing a negative balance. */
export async function adjustCharacterMoney(characterId: string, deltaCopper: number): Promise<CharacterMoneyDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for adjusting character money');
  }

  try {
    return await invoke<CharacterMoneyDto>('adjust_character_money', { request: { characterId, deltaCopper } });
  } catch (cause: unknown) {
    throw new Error(`Failed to adjust character money: ${formatError(cause)}`);
  }
}
