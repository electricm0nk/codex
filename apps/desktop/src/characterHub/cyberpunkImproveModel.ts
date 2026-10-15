import type {
  CyberpunkImprovePlan,
  CyberpunkImprovePreview,
  CyberpunkImproveRow,
  CyberpunkImproveSkillOption,
  CyberpunkPurchase,
} from '../boundary/cprImprove';

/**
 * The Cyberpunk RED improve dialog's model (SD-38 E5.1). It holds the plan the engine prices -- the
 * IP entered and the purchases, one Skill Level or Role Rank each -- and finds each figure the
 * dialog shows among the engine's preview rows by id. It prices, sums and subtracts nothing: the
 * costs, the IP available, the plan's cost and the IP left are `cpr_improve.rs` rows.
 */

export const EMPTY_CYBERPUNK_IMPROVE_PLAN: CyberpunkImprovePlan = { ipAwarded: 0, purchases: [] };

/** The plan with the IP entered set from the field's text; a non-whole or negative entry is refused. */
export function improvePlanFromIpText(plan: CyberpunkImprovePlan, text: string): { plan: CyberpunkImprovePlan; error: string | null } {
  const trimmed = text.trim();
  if (trimmed === '') {
    return { plan: { ...plan, ipAwarded: 0 }, error: null };
  }
  if (!/^\d+$/.test(trimmed)) {
    return { plan, error: 'IP entered must be a whole number of 0 or more' };
  }
  return { plan: { ...plan, ipAwarded: Number(trimmed) }, error: null };
}

/** The key a purchase of `option` names: the skill, or `<skill> (<specialty>)`; null when a needed specialty is blank. */
export function skillPurchaseKey(option: CyberpunkImproveSkillOption, specialty: string): string | null {
  if (!option.specialtyRequired) {
    return option.key;
  }
  const named = specialty.trim();
  return named === '' ? null : `${option.name} (${named})`;
}

type Target = { kind: 'skill'; key: string } | { kind: 'roleRank' };

/** The Level/Rank a new purchase asks for: one above the plan's last purchase of it, else above `current`. */
export function nextTarget(plan: CyberpunkImprovePlan, target: Target, current: number): number {
  let from = current;
  for (const purchase of plan.purchases) {
    if (purchase.kind === target.kind && (purchase.kind === 'roleRank' || (target.kind === 'skill' && purchase.key === target.key))) {
      from = purchase.to;
    }
  }
  return from + 1;
}

export function addPurchase(plan: CyberpunkImprovePlan, purchase: CyberpunkPurchase): CyberpunkImprovePlan {
  return { ...plan, purchases: [...plan.purchases, purchase] };
}

export function removePurchase(plan: CyberpunkImprovePlan, index: number): CyberpunkImprovePlan {
  return { ...plan, purchases: plan.purchases.filter((_, i) => i !== index) };
}

export function purchaseLabel(purchase: CyberpunkPurchase): string {
  return purchase.kind === 'skill' ? `${purchase.key} to Level ${purchase.to}` : `Role Ability to Rank ${purchase.to}`;
}

function rowOf(preview: CyberpunkImprovePreview | null, id: string): CyberpunkImproveRow | null {
  return preview?.rows.find((row) => row.id === id) ?? null;
}

/** The engine's price of purchase `index` of the plan. */
export function purchaseCost(preview: CyberpunkImprovePreview | null, index: number): CyberpunkImproveRow | null {
  return rowOf(preview, `cpr.improve.purchase.${index}.cost`);
}

/** The engine's IP available, plan cost and IP left. */
export function improveTotals(preview: CyberpunkImprovePreview | null): {
  available: CyberpunkImproveRow | null;
  cost: CyberpunkImproveRow | null;
  remaining: CyberpunkImproveRow | null;
} {
  return {
    available: rowOf(preview, 'cpr.improve.ip.available'),
    cost: rowOf(preview, 'cpr.improve.ip.cost'),
    remaining: rowOf(preview, 'cpr.improve.ip.remaining'),
  };
}

/** Spend is offered only for a plan with something in it that the engine priced without refusal. */
export function canSpend(plan: CyberpunkImprovePlan, preview: CyberpunkImprovePreview | null): boolean {
  return preview !== null && preview.refusal === null && (plan.purchases.length > 0 || plan.ipAwarded > 0);
}
