import { readFileSync } from 'node:fs';
import { assert } from '../testSupport/asserts';

/**
 * SD-43 E1.13 (Q-9.1): the Gear sell window returns money per its own option even when the session
 * Market mode is Cashless. The window always sends its total, so the request must not carry the
 * session `priceMode` (Cashless pays 0 and ignores the total) when a total is given.
 */
const sheet = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
const start = sheet.indexOf('async function handleGearSell(');
assert(start >= 0, 'handleGearSell is defined');
const body = sheet.slice(start, sheet.indexOf('\n  }\n', start));

assert(
  body.includes("priceMode: negotiatedGp === undefined ? priceMode : 'standard',"),
  "a window total sends priceMode 'standard', not the session mode",
);
assert(body.includes('negotiatedPriceCopper: gpToCopper(negotiatedGp)'), 'the window total still goes as copper');
console.log('gearSellHoldsUnderCashless.test.ts: all assertions passed');
