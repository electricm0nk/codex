import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert } from '../testSupport/asserts';
import { PriceModeControl } from './PriceModeControl';
import { PRICE_MODE_OPTIONS } from './priceMode';

const render = (value: 'cashless' | 'standard' | 'characterBuild') =>
  renderToStaticMarkup(createElement(PriceModeControl, { value, onChange: () => {} }));

function verifiesAllThreeModesAreOffered() {
  const html = render('standard');
  for (const option of PRICE_MODE_OPTIONS) {
    assert(html.includes(option.label), `the "${option.label}" mode is offered`);
  }
}

function verifiesTheSelectedModeIsMarkedAndExplained() {
  const html = render('characterBuild');
  const checked = html.match(/checked=""/g) ?? [];
  assert(checked.length === 1, `exactly one mode is selected, found ${checked.length}`);
  const hint = PRICE_MODE_OPTIONS.find((o) => o.value === 'characterBuild')?.hint ?? '';
  assert(html.includes(hint), 'the selected mode explains itself');
  assert(!html.includes(PRICE_MODE_OPTIONS.find((o) => o.value === 'cashless')?.hint ?? 'x'), 'only the selected mode is explained');
}

verifiesAllThreeModesAreOffered();
verifiesTheSelectedModeIsMarkedAndExplained();
console.log('PriceModeControl.test.ts: all assertions passed');
