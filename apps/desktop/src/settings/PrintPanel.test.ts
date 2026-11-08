import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { SETTINGS_TABS, SettingsModal } from './SettingsModal';
import { PrintPanel } from './PrintPanel';

/**
 * Settings, Print (operator ruling R3): a Print tab whose one row is the paper, A4 or US Letter,
 * shown with the paper the store holds. The modal draws the panel itself, so no caller can leave
 * the tab empty.
 */

assert(SETTINGS_TABS.some((tab) => tab.id === 'print' && tab.label === 'Print'), 'Settings lists a Print tab');

const a4 = renderToStaticMarkup(createElement(PrintPanel, { initialPaper: 'a4' }));
assert(a4.includes('Print paper'), 'the row is named Print paper');
assert(/<option value="a4" selected="">A4<\/option>/.test(a4), 'A4 is selected when the setting is A4');
assert(a4.includes('<option value="letter">US Letter</option>'), 'US Letter is offered');

const letter = renderToStaticMarkup(createElement(PrintPanel, { initialPaper: 'letter' }));
assert(/<option value="letter" selected="">US Letter<\/option>/.test(letter), 'US Letter is selected when the setting is US Letter');

const modal = renderToStaticMarkup(
  createElement(SettingsModal, {
    open: true,
    activeTab: 'print',
    onTabChange: () => undefined,
    onClose: () => undefined,
    panels: { appearance: null, 'google-drive': null, update: null, bug: null, enhancement: null, developer: null },
  }),
);
assert(modal.includes('Print paper'), 'the modal draws the Print panel on the Print tab');
assertEqual((modal.match(/<select/g) ?? []).length, 1, 'one control on the tab');

console.log('PrintPanel.test.ts: all assertions passed');
