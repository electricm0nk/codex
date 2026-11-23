import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import type { AbilityOptionDto } from '../boundary/abilities';
import { buildAbilitySubTabModel } from './abilitySubTabModel';
import { AbilitySubTabView } from './AbilitySubTab';

// SD-43 E3.3: the generic Abilities sub-tab renders granted/spent/remaining from the engine's
// `abilities.budget.<subtab>.<pool>.*` rows only. The fixture rows are deliberately inconsistent
// (granted 7, spent 2, remaining 0; three picks held): a model that did arithmetic would show 5,
// 3 or 4 somewhere. It must show exactly the row values.
const budget = [
  { id: 'abilities.budget.traits.traits.granted', value: 7, detail: 'Granted by the Traits pool' },
  { id: 'abilities.budget.traits.traits.spent', value: 2, detail: 'Held picks charged to the traits pool' },
  { id: 'abilities.budget.traits.traits.remaining', value: 0, detail: 'Granted minus spent in the traits pool' },
  { id: 'abilities.budget.traits.drawbacks.granted', value: 1, detail: 'Granted by Ultimate Campaign' },
  { id: 'abilities.budget.traits.drawbacks.spent', value: 1, detail: 'Held picks charged to the drawbacks pool' },
  { id: 'abilities.budget.traits.drawbacks.remaining', value: 9, detail: 'Granted minus spent in the drawbacks pool' },
  { id: 'feats.budget.general.granted', value: 42, detail: 'another tab' },
];
const option = (id: string, label: string, pool: string, qualified = true, unmet: string[] = [], category: string | null = null): AbilityOptionDto => ({
  id,
  label,
  pool,
  category,
  qualified,
  unmet,
});
const options = [
  option('trait:trait_acrobat', 'Acrobat', 'traits', true, [], 'Social'),
  option('trait:trait_ambitious', 'Ambitious', 'traits', false, ['one Social trait only: Acrobat is held'], 'Social'),
  option('trait:trait_axe_to_grind', 'Axe to Grind', 'traits', true, [], 'Combat'),
  option('drawback:anxious', 'Anxious', 'drawbacks'),
];
const selectedIds = ['trait:trait_acrobat', 'trait:trait_axe_to_grind', 'trait:trait_unknown'];

const model = buildAbilitySubTabModel({ subtab: 'traits', budget, options, selectedIds, search: '', qualifiedOnly: false });

assertEqual(model.pools.map((pool) => pool.id).join(), 'traits,drawbacks', 'one pool per sub-tab pool, in row order; other tabs ignored');
const traits = model.pools[0];
assertEqual(traits.granted, 7, 'granted is the granted row');
assertEqual(traits.spent, 2, 'spent is the spent row, not the 3 held picks');
assertEqual(traits.remaining, 0, 'remaining is the remaining row, not granted minus spent');
assertEqual(model.pools[1].remaining, 9, 'drawbacks remaining is its row, not 0');

// Every number anywhere in the model is a row value: nothing is computed from anything but rows.
const rowValues = new Set(budget.filter((row) => row.id.startsWith('abilities.budget.traits.')).map((row) => row.value));
const numbers: number[] = [];
const walk = (value: unknown) => {
  if (typeof value === 'number') numbers.push(value);
  else if (Array.isArray(value)) value.forEach(walk);
  else if (value !== null && typeof value === 'object') Object.values(value).forEach(walk);
};
walk(model);
assert(numbers.length >= 6, `the model carries the six pool numbers (found ${numbers.length})`);
for (const n of numbers) {
  assert(rowValues.has(n), `model number ${n} is not a budget row value`);
}
const stringNumbers = JSON.stringify(model).match(/\d+/g) ?? [];
for (const n of stringNumbers) {
  assert(rowValues.has(Number(n)), `model text number ${n} is not a budget row value`);
}

// Selected: held picks, labelled from the options; an id the engine does not list keeps its id.
assertEqual(model.selected.map((row) => row.label).join(), 'Acrobat,Axe to Grind,trait:trait_unknown', 'selected rows in held order');
// Available: not held; Add disabled with the engine's reason.
assertEqual(model.available.map((row) => row.id).join(), 'trait:trait_ambitious,drawback:anxious', 'held picks are not offered again');
const ambitious = model.available[0];
assertEqual(ambitious.qualified, false, 'Ambitious is unqualified');
assertEqual(ambitious.addBlockedBy, 'one Social trait only: Acrobat is held', 'an unqualified option names the engine unmet reason');
assertEqual(model.available[1].addBlockedBy, null, 'drawbacks: the remaining row says 9, so Add is live');

// A full pool (remaining row 0) disables Add for a qualified option, naming the pool and its rows.
const roomless = buildAbilitySubTabModel({
  subtab: 'traits',
  budget,
  options: [option('trait:trait_bastard', 'Bastard', 'traits')],
  selectedIds: [],
  search: '',
  qualifiedOnly: false,
});
assertEqual(roomless.available[0].addBlockedBy, 'No room in the traits pool: 2 of 7 spent, 0 remaining.', 'a full pool names its rows');

// Search and the Qualified filter.
const searched = buildAbilitySubTabModel({ subtab: 'traits', budget, options, selectedIds: [], search: 'AXE', qualifiedOnly: false });
assertEqual(searched.available.map((row) => row.label).join(), 'Axe to Grind', 'search matches labels, ignoring case');
const qualifiedOnly = buildAbilitySubTabModel({ subtab: 'traits', budget, options, selectedIds: [], search: '', qualifiedOnly: true });
assert(!qualifiedOnly.available.some((row) => !row.qualified), 'Qualified hides unqualified options');
assertEqual(qualifiedOnly.available.length, 3, 'Qualified keeps the three qualified options');

// The view: the pool line, unqualified drawn differently, Add disabled with the reason as its title.
const html = renderToStaticMarkup(
  createElement(AbilitySubTabView, {
    title: 'Traits',
    model,
    search: '',
    qualifiedOnly: false,
    focusedId: 'trait:trait_ambitious',
    info: () => ({ description: 'You are ambitious.', grants: null, choices: [] }),
    busy: false,
    error: null,
    onSearch: () => undefined,
    onQualifiedOnly: () => undefined,
    onFocus: () => undefined,
    onAdd: () => undefined,
    onRemove: () => undefined,
  }),
);
assert(html.includes('Traits: 2 of 7 spent, 0 remaining'), 'the view prints the traits pool line from rows');
assert(html.includes('Drawbacks: 1 of 1 spent, 9 remaining'), 'the view prints the drawbacks pool line from rows');
assert(/data-qualified="false"[^>]*>[\s\S]*?Ambitious/.test(html), 'an unqualified option is drawn differently (data-qualified="false")');
assert(html.includes('title="one Social trait only: Acrobat is held"'), 'the disabled Add carries the engine reason');
assert(html.includes('You are ambitious.'), 'the Info pane shows the focused option');
assert(html.includes('Remove'), 'Selected rows have Remove');

console.log('abilitySubTabModel tests passed');
