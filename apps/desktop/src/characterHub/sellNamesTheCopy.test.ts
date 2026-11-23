import { readFileSync } from 'node:fs';
import { assert } from '../testSupport/asserts';

/**
 * SD-43 E1.12 (E0.4 `gear-repro.md` §12.2): the Gear screen's Sell and the Market's send the copy
 * the player clicked (the row's `copies[0]`), so the engine sells that one and not the first
 * copy of the same item and state. Drop, Discard and Move already do.
 */
const sheet = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
const boundary = readFileSync(new URL('../boundary/removeSelection.ts', import.meta.url), 'utf8');

function body(name: string): string {
  const start = sheet.indexOf(`async function ${name}(`);
  assert(start >= 0, `${name} is defined`);
  return sheet.slice(start, sheet.indexOf('\n  }\n', start));
}

assert(/copy\?: CopyKeyDto;/.test(boundary), 'the Sell request carries a copy key');
assert(body('handleGearSell').includes('copy: held.copies[0],'), "the Gear screen's Sell sends row.held.copies[0]");
assert(body('handleMarketSell').includes('copy.copies?.[0]'), "the Market's Sell sends the inventory row's copies[0]");
assert(body('handleRemoveEquipment').includes('{ copy }'), 'handleRemoveEquipment puts the copy on the request');
console.log('sellNamesTheCopy.test.ts: all assertions passed');
