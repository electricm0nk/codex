import type { PriceMode } from './priceMode';

/**
 * The money arithmetic behind the Create screen's equipment dialog. A new character starts with the
 * maximum starting money for the class (`startingGp`; `null` when the class publishes none, which is
 * 0 gp with a note). Everything is counted in copper so 0.1 gp + 0.1 gp is exactly 0.2 gp. The
 * backend repeats these checks (`create.equipment_unaffordable`); this is the preview.
 */

export interface EquipmentBudgetInput {
  startingGp: number | null;
  mode: PriceMode;
  /** Catalog price of each item already chosen. */
  costsGp: readonly number[];
  /** Why the class has no starting money, when it has none. */
  note?: string | null;
}

export interface EquipmentBudget {
  spentGp: number;
  remainingGp: number;
  over: boolean;
  text: string;
}

const toCopper = (gp: number): number => Math.round(gp * 100);
const toGp = (copper: number): number => copper / 100;

export function equipmentBudget(input: EquipmentBudgetInput): EquipmentBudget {
  const startingCopper = toCopper(input.startingGp ?? 0);
  if (input.mode === 'cashless') {
    return { spentGp: 0, remainingGp: toGp(startingCopper), over: false, text: 'Cashless: items cost nothing and starting money is untouched.' };
  }
  const spentCopper = input.costsGp.reduce((sum, gp) => sum + toCopper(gp), 0);
  const remainingCopper = startingCopper - spentCopper;
  const noteText = input.startingGp === null && input.note ? ` ${input.note}` : '';
  return {
    spentGp: toGp(spentCopper),
    remainingGp: toGp(remainingCopper),
    over: remainingCopper < 0,
    text: `${toGp(remainingCopper)} gp left of ${toGp(startingCopper)} gp starting money.${noteText}`,
  };
}

/** Whether an item priced `costGp` (`null`: the catalog states no price) can be added now. */
export function canAfford(input: EquipmentBudgetInput, costGp: number | null): boolean {
  if (input.mode === 'cashless') {
    return true;
  }
  if (costGp === null) {
    return false;
  }
  return toCopper(costGp) <= toCopper(input.startingGp ?? 0) - input.costsGp.reduce((sum, gp) => sum + toCopper(gp), 0);
}
