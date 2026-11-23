import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { MarketButton } from './MarketButton';

const source = (name: string) => readFileSync(new URL(name, import.meta.url), 'utf8');
const count = (text: string, needle: string) => text.split(needle).length - 1;

const html = renderToStaticMarkup(createElement(MarketButton, { onClick: () => undefined, testId: 'x-market' }));
assert(html.includes('>Market<') && html.includes('data-testid="x-market"'), `renders a Market button: ${html}`);
assert(html.includes('var(--color-accent)'), 'accent colour');

const pf = source('CharacterSheet.tsx');
assertEqual(count(pf, '<MarketButton'), 3, 'Weapons, Defense and Gear each render one MarketButton');
assertEqual(count(pf, 'PriceModeControl'), 0, 'no PriceModeControl in the sheet');
assertEqual(count(pf, 'props.onPrint'), 0, 'no panel Print');
assert(source('MarketDialog.tsx').includes('PriceModeControl'), 'pricing control lives in the Market dialog');
for (const f of ['StarfinderCharacterSheet.tsx', 'CyberpunkCharacterSheet.tsx']) {
  assertEqual(count(source(f), '<MarketButton'), 1, `${f} renders one MarketButton`);
}
