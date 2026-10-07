import { assert, assertEqual } from '../testSupport/asserts';
import { DEFAULT_PRICE_MODE, PRICE_MODE_OPTIONS, refundPreviewCopper } from './priceMode';

// The wire values must match the backend's `PriceMode` serde names (camelCase).
function verifiesWireValuesAndOrder() {
  assertEqual(PRICE_MODE_OPTIONS.map((o) => o.value).join('|'), 'cashless|standard|characterBuild', 'option order and wire names');
}

function verifiesTheDefaultIsBuyAtFullSellAtHalf() {
  assertEqual(DEFAULT_PRICE_MODE, 'standard', 'the default is the standard mode');
  const standard = PRICE_MODE_OPTIONS.find((o) => o.value === DEFAULT_PRICE_MODE);
  assert(standard !== undefined && standard.label.includes('100%') && standard.label.includes('50%'), 'the default is labelled with its rates');
}

function verifiesEveryOptionExplainsItself() {
  for (const option of PRICE_MODE_OPTIONS) {
    assert(option.label.length > 0 && option.hint.length > 20, `${option.value} has a label and an explanation`);
  }
}

// Mirrors the backend's refund rule so the screen can say what a sale will return before it happens.
function verifiesRefundPreviewMatchesTheBackendRule() {
  assertEqual(refundPreviewCopper(200, 'standard'), 100, 'sell at 50%');
  assertEqual(refundPreviewCopper(200, 'characterBuild'), 200, 'sell at 100%');
  assertEqual(refundPreviewCopper(200, 'cashless'), 0, 'cashless returns nothing');
  assertEqual(refundPreviewCopper(5, 'standard'), 2, 'half rounds down');
}

verifiesWireValuesAndOrder();
verifiesTheDefaultIsBuyAtFullSellAtHalf();
verifiesEveryOptionExplainsItself();
verifiesRefundPreviewMatchesTheBackendRule();
console.log('priceMode.test.ts: all assertions passed');
