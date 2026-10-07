import { assert, assertEqual } from '../testSupport/asserts';
import { canAfford, equipmentBudget } from './creationEquipmentModel';

// Starting money is the class's maximum roll; what is bought comes off it unless the pricing is cashless.
function verifiesTheBudgetSubtractsWhatWasBought() {
  const budget = equipmentBudget({ startingGp: 120, mode: 'standard', costsGp: [2, 15.5] });
  assertEqual(budget.spentGp, 17.5, 'spent is the sum of catalog prices');
  assertEqual(budget.remainingGp, 102.5, 'remaining is what is left');
  assertEqual(budget.over, false, 'within the money');
  assert(budget.text.includes('102.5') && budget.text.includes('120'), `text states both numbers: ${budget.text}`);
}

function verifiesOverspendingIsFlaggedAndCashlessSpendsNothing() {
  assertEqual(equipmentBudget({ startingGp: 10, mode: 'standard', costsGp: [6, 6] }).over, true, 'twelve of ten is over');
  const cashless = equipmentBudget({ startingGp: 10, mode: 'cashless', costsGp: [600] });
  assertEqual(cashless.over, false, 'cashless is never over');
  assertEqual(cashless.spentGp, 0, 'and spends nothing');
  assert(cashless.text.toLowerCase().includes('cashless'), 'and says so');
}

function verifiesAClassWithNoPublishedWealthStartsWithZeroAndItsNote() {
  const budget = equipmentBudget({ startingGp: null, mode: 'standard', costsGp: [], note: 'No published figure.' });
  assertEqual(budget.remainingGp, 0, 'zero gp');
  assert(budget.text.includes('No published figure.'), 'with the reason');
  assertEqual(canAfford({ startingGp: null, mode: 'standard', costsGp: [] }, 1), false, 'nothing is affordable');
  assertEqual(canAfford({ startingGp: null, mode: 'cashless', costsGp: [] }, 1), true, 'unless cashless');
}

function verifiesWhetherAnItemCanBeBought() {
  const input = { startingGp: 10, mode: 'standard' as const, costsGp: [4] };
  assertEqual(canAfford(input, 6), true, 'exactly the money left');
  assertEqual(canAfford(input, 6.01), false, 'a copper over');
  assertEqual(canAfford(input, null), false, 'an item with no catalog price cannot be bought');
  assertEqual(canAfford({ ...input, mode: 'cashless' }, null), true, 'but is free when cashless');
  assertEqual(canAfford({ startingGp: 0.3, mode: 'standard', costsGp: [0.1, 0.1] }, 0.1), true, 'copper arithmetic is exact, not floating point');
}

verifiesTheBudgetSubtractsWhatWasBought();
verifiesOverspendingIsFlaggedAndCashlessSpendsNothing();
verifiesAClassWithNoPublishedWealthStartsWithZeroAndItsNote();
verifiesWhetherAnItemCanBeBought();
console.log('creationEquipmentModel.test.ts: all assertions passed');
