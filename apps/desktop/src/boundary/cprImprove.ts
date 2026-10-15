import { invoke } from '@tauri-apps/api/core';
import type { LoadSavedCharacterResponse } from './loadSavedCharacterDetail';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the Cyberpunk RED improve dialog (SD-38 E5.1):
 * `preview_cyberpunk_improvement` and `apply_cyberpunk_improvement`
 * (`apps/desktop/src-tauri/src/cpr_improve.rs`). The dialog sends its plan -- the IP entered and the
 * purchases, one Skill Level, Role Rank, Role switch or cyberware install each -- and the engine prices it on
 * the p. 411 tables, refuses a skipped Level or an overspend, checks an install's Humanity Loss against its
 * dice and its option slots (E5.3: p. 111, A-3, D-11; a blank loss is rolled with the plan's seed), checks a
 * therapy's Humanity regain against its dice and stops it at the D-5 maximum Humanity (E5.4: p. 230), and (on
 * apply) saves through the store and answers with the reloaded sheet. Nothing here prices or counts.
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

/** A piece of cyberware the dialog offers, with the engine's Humanity Loss dice and their range (p. 111). */
export interface CyberpunkImproveCyberwareOption {
  name: string;
  category: string;
  dice: string | null;
  min: number;
  max: number;
}

/**
 * A therapy of the p. 230 table, with the engine's Humanity regain dice and their range (none for Addiction,
 * whose effect is printed) and its printed cost, Medical Tech DV, materials, duration and effect.
 */
export interface CyberpunkImproveTherapyOption {
  name: string;
  dice: string | null;
  min: number;
  max: number;
  costEb: number;
  priceCategory: string;
  medtechDv: number;
  materialsEb: number;
  materialsPriceCategory: string;
  duration: string;
  text: string;
}

export interface CyberpunkImprovePreview {
  skills: CyberpunkImproveSkillOption[];
  role: CyberpunkImproveRoleOption | null;
  switch: CyberpunkImproveSwitchOption[];
  cyberware: CyberpunkImproveCyberwareOption[];
  therapy: CyberpunkImproveTherapyOption[];
  rows: CyberpunkImproveRow[];
  refusal: string | null;
  /** The seed the engine rolled any blank Humanity Loss with; send it back to keep those dice. */
  seed: number;
}

export type CyberpunkPurchase =
  | { kind: 'skill'; key: string; to: number }
  | { kind: 'roleRank'; to: number }
  | { kind: 'roleSwitch'; role: string }
  | { kind: 'cyberware'; name: string; humanityLoss: number | null }
  | { kind: 'therapy'; name: string; regained: number | null };

function wirePurchase(p: CyberpunkPurchase) {
  switch (p.kind) {
    case 'skill':
      return { kind: 'skill', key: p.key, to: p.to };
    case 'roleRank':
      return { kind: 'roleRank', to: p.to };
    case 'roleSwitch':
      return { kind: 'roleSwitch', role: p.role };
    case 'cyberware':
      return { kind: 'cyberware', name: p.name, humanityLoss: p.humanityLoss };
    case 'therapy':
      return { kind: 'therapy', name: p.name, regained: p.regained };
  }
}

export interface CyberpunkImprovePlan {
  ipAwarded: number;
  purchases: readonly CyberpunkPurchase[];
  /** The roller seed for blank Humanity Loss entries; absent, the engine draws one and answers with it. */
  seed?: number | null;
}

function request(characterId: string, plan: CyberpunkImprovePlan) {
  return {
    request: {
      characterId,
      ipAwarded: plan.ipAwarded,
      purchases: plan.purchases.map(wirePurchase),
      ...(plan.seed === undefined || plan.seed === null ? {} : { seed: plan.seed }),
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
