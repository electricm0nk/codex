import { invoke } from '@tauri-apps/api/core';
import type { LoadSavedCharacterResponse } from './loadSavedCharacterDetail';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the Cyberpunk RED improve dialog (SD-38 E5.1):
 * `preview_cyberpunk_improvement` and `apply_cyberpunk_improvement`
 * (`apps/desktop/src-tauri/src/cpr_improve.rs`). The dialog sends its plan -- the IP entered and the
 * purchases, one Skill Level, Role Rank or Role switch each -- and the engine prices it on the p. 411 tables,
 * refuses a skipped Level or an overspend, and (on apply) saves through the store and answers with
 * the reloaded sheet. Nothing here prices or counts.
 */

/** One engine row: id, value and the engine's derivation text. */
export interface CyberpunkImproveRow {
  id: string;
  value: number;
  detail: string;
}

/** One Skill the dialog offers, at its Level, with the engine's price of the next Level. */
export interface CyberpunkImproveSkillOption {
  key: string;
  name: string;
  category: string;
  x2: boolean;
  specialtyRequired: boolean;
  level: number;
  next: CyberpunkImproveRow | null;
}

/** The character's Role at its Rank, with the engine's price of the next Rank. */
export interface CyberpunkImproveRoleOption {
  name: string;
  rank: number;
  next: CyberpunkImproveRow | null;
}

/** A Role the character does not hold: the engine's A-1 price of its Rank 1, or why it is locked (p. 143). */
export interface CyberpunkImproveSwitchOption {
  role: string;
  cost: CyberpunkImproveRow | null;
  refusal: string | null;
}

export interface CyberpunkImprovePreview {
  skills: CyberpunkImproveSkillOption[];
  role: CyberpunkImproveRoleOption | null;
  switch: CyberpunkImproveSwitchOption[];
  rows: CyberpunkImproveRow[];
  refusal: string | null;
}

export type CyberpunkPurchase =
  | { kind: 'skill'; key: string; to: number }
  | { kind: 'roleRank'; to: number }
  | { kind: 'roleSwitch'; role: string };

function wirePurchase(p: CyberpunkPurchase) {
  switch (p.kind) {
    case 'skill':
      return { kind: 'skill', key: p.key, to: p.to };
    case 'roleRank':
      return { kind: 'roleRank', to: p.to };
    case 'roleSwitch':
      return { kind: 'roleSwitch', role: p.role };
  }
}

export interface CyberpunkImprovePlan {
  ipAwarded: number;
  purchases: readonly CyberpunkPurchase[];
}

function request(characterId: string, plan: CyberpunkImprovePlan) {
  return {
    request: {
      characterId,
      ipAwarded: plan.ipAwarded,
      purchases: plan.purchases.map(wirePurchase),
    },
  };
}

export async function previewCyberpunkImprovement(characterId: string, plan: CyberpunkImprovePlan): Promise<CyberpunkImprovePreview> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for previewing a Cyberpunk RED improvement');
  }
  try {
    return await invoke<CyberpunkImprovePreview>('preview_cyberpunk_improvement', request(characterId, plan));
  } catch (cause: unknown) {
    throw new Error(`Failed to preview the Cyberpunk RED improvement: ${formatError(cause)}`);
  }
}

export async function applyCyberpunkImprovement(characterId: string, plan: CyberpunkImprovePlan): Promise<LoadSavedCharacterResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for spending Cyberpunk RED Improvement Points');
  }
  try {
    return await invoke<LoadSavedCharacterResponse>('apply_cyberpunk_improvement', request(characterId, plan));
  } catch (cause: unknown) {
    throw new Error(`Failed to spend Cyberpunk RED Improvement Points: ${formatError(cause)}`);
  }
}
