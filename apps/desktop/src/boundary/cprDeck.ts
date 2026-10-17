import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the Netrunner deck calculator (SD-38 E6.1):
 * `compute_cyberpunk_deck` (`apps/desktop/src-tauri/src/cpr_catalog.rs`). Names resolve
 * to corpus records in Rust; slots used/available, NET Actions and the over-capacity
 * refusal are the engine's `cpr.netrunning.*` answers (A-7). Nothing here sums a slot.
 */

export interface CyberpunkDeckExtras {
  /** Wearing a Bodyweight Suit: one extra slot that only 1-slot Hardware may use. */
  bodyweightSuit: boolean;
  /** The Cyberdeck Cyberarm option: one extra Program/Hardware slot. */
  cyberarmDeck: boolean;
}

export interface CyberpunkDeckRow {
  id: string;
  value: number;
  detail: string;
}

export interface CyberpunkDeckResponse {
  deck: string;
  slotsAvailable: number;
  slotsUsed: number;
  bodyweightSuitHardware: string | null;
  /** The `cpr.netrunning.net_actions` row, present when an Interface rank was asked for. */
  netActions: CyberpunkDeckRow | null;
  rows: CyberpunkDeckRow[];
}

export interface CyberpunkDeckRequest {
  deck: string;
  programs: string[];
  hardware: string[];
  extras: CyberpunkDeckExtras;
  interfaceRank?: number | null;
}

export async function computeCyberpunkDeck(request: CyberpunkDeckRequest): Promise<CyberpunkDeckResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for computing the Cyberpunk RED deck');
  }

  try {
    return await invoke<CyberpunkDeckResponse>('compute_cyberpunk_deck', {
      deck: request.deck,
      programs: request.programs,
      hardware: request.hardware,
      extras: request.extras,
      interfaceRank: request.interfaceRank ?? null,
    });
  } catch (cause: unknown) {
    throw new Error(`Failed to compute the Cyberpunk RED deck: ${formatError(cause)}`);
  }
}
