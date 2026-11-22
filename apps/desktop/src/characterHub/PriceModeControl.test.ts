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

function verifiesTwoControlsCanShowAtOnce() {
  assert(render('standard').includes('name="equipment-price-mode"'), 'the default group');
  const html = renderToStaticMarkup(createElement(PriceModeControl, { value: 'standard', onChange: () => {}, groupName: 'market-price-mode' }));
  assert(html.includes('name="market-price-mode"') && !html.includes('equipment-price-mode'), 'one radio group would uncheck the other control');
}

verifiesAllThreeModesAreOffered();
verifiesTwoControlsCanShowAtOnce();
verifiesTheSelectedModeIsMarkedAndExplained();
console.log('PriceModeControl.test.ts: all assertions passed');
