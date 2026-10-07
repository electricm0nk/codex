import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert } from '../testSupport/asserts';
import { ManageBox } from './ManageBox';

const render = (over: Record<string, unknown> = {}) =>
  renderToStaticMarkup(createElement(ManageBox, { title: 'Feats', summary: ['Power Attack', 'Dodge'], remaining: '1 of 3 remaining', onManage: () => {}, ...over }));

function verifiesTheBoxShowsItsTitleSummaryAndManageButton() {
  const html = render();
  assert(html.includes('Feats'), 'title');
  assert(html.includes('Power Attack') && html.includes('Dodge'), 'what is chosen');
  assert(html.includes('1 of 3 remaining'), 'what is left to pick');
  assert(html.includes('>Manage<'), 'a Manage button');
}

function verifiesAnEmptyBoxSaysNothingIsChosenAndADisabledBoxExplainsWhy() {
  assert(render({ summary: [] }).includes('Nothing chosen yet'), 'empty summary');
  const disabled = render({ disabledReason: 'Loading feats…' });
  assert(/disabled/.test(disabled), 'the button is disabled');
  assert(disabled.includes('Loading feats…'), 'with the reason shown');
}

verifiesTheBoxShowsItsTitleSummaryAndManageButton();
verifiesAnEmptyBoxSaysNothingIsChosenAndADisabledBoxExplainsWhy();
console.log('ManageBox.test.ts: all assertions passed');
