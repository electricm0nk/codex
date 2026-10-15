import type {
  CyberpunkImproveCyberwareOption,
  CyberpunkImprovePlan,
  CyberpunkImprovePreview,
  CyberpunkImproveRow,
  CyberpunkImproveSkillOption,
  CyberpunkImproveSwitchOption,
  CyberpunkImproveTherapyOption,
  CyberpunkPurchase,
} from '../boundary/cprImprove';

/**
 * The Cyberpunk RED improve dialog's model (SD-38 E5.1). It holds the plan the engine prices -- the
 * IP entered and the purchases, one Skill Level, Role Rank, Role switch, cyberware install or therapy each -- and finds each figure the
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

/**
 * The Level/Rank a new purchase asks for: one above the plan's last purchase of it, else above
 * `current`. A Rank purchase raises the latest Role, so after a Role switch in the plan it asks for
 * Rank 2 of the new Role (the new Role starts at Rank 1, p. 143).
 */
export function nextTarget(plan: CyberpunkImprovePlan, target: Target, current: number): number {
  let from = current;
  for (const purchase of plan.purchases) {
    if (target.kind === 'roleRank' && purchase.kind === 'roleSwitch') {
      from = 1;
    } else if (purchase.kind === 'roleRank' && target.kind === 'roleRank') {
      from = purchase.to;
    } else if (purchase.kind === 'skill' && target.kind === 'skill' && purchase.key === target.key) {
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
  switch (purchase.kind) {
    case 'skill':
      return `${purchase.key} to Level ${purchase.to}`;
    case 'roleRank':
      return `Role Ability to Rank ${purchase.to}`;
    case 'roleSwitch':
      return `Switch Role to ${purchase.role} (Rank 1)`;
    case 'cyberware':
      return purchase.humanityLoss === null
        ? `Install ${purchase.name} (Humanity Loss rolled)`
        : `Install ${purchase.name} (Humanity Loss ${purchase.humanityLoss} entered)`;
    case 'therapy':
      return purchase.regained === null
        ? `Therapy: ${purchase.name} (Humanity regained rolled)`
        : `Therapy: ${purchase.name} (Humanity regained ${purchase.regained} entered)`;
  }
}

/** A therapy option as the dialog lists it: the engine's regain dice and range, and the printed cost and DV (p. 230). */
export function therapyOptionLabel(option: CyberpunkImproveTherapyOption): string {
  const regain = option.dice === null ? 'regains no Humanity' : `regains ${option.dice} (${option.min}-${option.max})`;
  return `${option.name} — ${regain}; ${option.costEb}eb (${option.priceCategory}), Medical Tech DV${option.medtechDv}`;
}

/** The Humanity regained field: blank rolls it (null); a whole number is entered; the engine checks the range and the cap. */
export function therapyRegainedFromText(text: string): { regained: number | null; error: string | null } {
  const trimmed = text.trim();
  if (trimmed === '') {
    return { regained: null, error: null };
  }
  if (!/^\d+$/.test(trimmed)) {
    return { regained: null, error: 'Humanity regained must be a whole number, or blank to roll it' };
  }
  return { regained: Number(trimmed), error: null };
}

/** A cyberware option as the dialog lists it: the engine's Humanity Loss dice and range (p. 111, A-3). */
export function cyberwareOptionLabel(option: CyberpunkImproveCyberwareOption): string {
  const loss = option.dice === null ? 'no Humanity Loss' : `Humanity Loss ${option.dice} (${option.min}-${option.max})`;
  return `${option.name} (${option.category}) — ${loss}`;
}

/** The Humanity Loss field: blank rolls it (null); a whole number is entered; the engine checks the range. */
export function humanityLossFromText(text: string): { humanityLoss: number | null; error: string | null } {
  const trimmed = text.trim();
  if (trimmed === '') {
    return { humanityLoss: null, error: null };
  }
  if (!/^\d+$/.test(trimmed)) {
    return { humanityLoss: null, error: 'Humanity Loss must be a whole number, or blank to roll it' };
  }
  return { humanityLoss: Number(trimmed), error: null };
}

/**
 * The plan with the engine's roller seed kept: once a preview answers, its seed is pinned so every later
 * preview and the apply roll the same blank Humanity Loss. A plan that already has a seed is returned as is.
 */
export function pinSeed(plan: CyberpunkImprovePlan, preview: CyberpunkImprovePreview): CyberpunkImprovePlan {
  return plan.seed === undefined || plan.seed === null ? { ...plan, seed: preview.seed } : plan;
}

/** A Role switch option as the dialog lists it: the engine's price of Rank 1, or that it is locked. */
export function switchOptionLabel(option: CyberpunkImproveSwitchOption): string {
  return option.cost === null ? `${option.role} — locked (p. 143)` : `${option.role} — Rank 1 for ${option.cost.value} IP`;
}

function rowOf(preview: CyberpunkImprovePreview | null, id: string): CyberpunkImproveRow | null {
  return preview?.rows.find((row) => row.id === id) ?? null;
}

/** The engine's price of purchase `index` of the plan. */
export function purchaseCost(preview: CyberpunkImprovePreview | null, index: number): CyberpunkImproveRow | null {
  return rowOf(preview, `cpr.improve.purchase.${index}.cost`);
}

/** The engine's Humanity Loss of install `index` of the plan (entered, or rolled on its dice). */
export function purchaseHumanityLoss(preview: CyberpunkImprovePreview | null, index: number): CyberpunkImproveRow | null {
  return rowOf(preview, `cpr.improve.purchase.${index}.humanity_loss`);
}

/** The engine's Humanity regained by therapy `index` of the plan (entered or rolled, stopped at the D-5 maximum). */
export function purchaseHumanityRegained(preview: CyberpunkImprovePreview | null, index: number): CyberpunkImproveRow | null {
  return rowOf(preview, `cpr.improve.purchase.${index}.humanity_regained`);
}

/** The engine's Humanity and EMP before and after the plan's installs and therapies (p. 80, p. 230); null without either. */
export function improveHumanity(preview: CyberpunkImprovePreview | null): {
  humanityBefore: CyberpunkImproveRow | null;
  humanityAfter: CyberpunkImproveRow | null;
  empBefore: CyberpunkImproveRow | null;
  empAfter: CyberpunkImproveRow | null;
} {
  return {
    humanityBefore: rowOf(preview, 'cpr.improve.humanity.before'),
    humanityAfter: rowOf(preview, 'cpr.improve.humanity.after'),
    empBefore: rowOf(preview, 'cpr.improve.emp.before'),
    empAfter: rowOf(preview, 'cpr.improve.emp.after'),
  };
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
