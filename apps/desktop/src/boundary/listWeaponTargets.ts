import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read desktop boundary for the corpus weapon list offered when a chooser
 * feat needs a weapon target.
 *
 * Backed by the same ingested `WEAPON_TABLE` the per-weapon attack, damage
 * and threat-range totals are computed from — deliberately not the
 * arms-and-armor equipment catalog, which mixes body armor in.
 */

export interface WeaponTargetDto {
  /** The weapon's corpus key — exactly what a chooser feat's target names. */
  key: string;
  /** e.g. `"1d8 · threat 19-20/x2"`. */
  detail: string;
}

export async function listWeaponTargets(): Promise<WeaponTargetDto[]> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing weapon targets');
  }

  try {
    return await invoke<WeaponTargetDto[]>('list_weapon_targets');
  } catch (cause: unknown) {
    throw new Error(`Failed to list weapon targets: ${formatError(cause)}`);
  }
}
