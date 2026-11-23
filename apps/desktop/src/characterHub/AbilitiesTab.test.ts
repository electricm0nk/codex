import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { readFeatBudget } from './characterProgression';
import { ABILITY_SUB_TABS, AbilitiesTab, PENDING_ENGINE_CARD } from './AbilitiesTab';
import { FeatsTab, SHEET_TABS } from './CharacterSheet';

// SD-43 E3.1 (operator ruling Q-22.3, 2026-11-22): the Feats tab is renamed Abilities; Feats is one of
// its seven sub-tabs, in PCGen's order (technical-design.md §3.3).
const PCGEN_ORDER = ['Feats', 'Racial Abilities', 'Traits', 'Archetype', 'Background', 'Class Features', 'Misc'];
assertEqual(ABILITY_SUB_TABS.join(), PCGEN_ORDER.join(), 'seven sub-tabs in PCGen order');

const tabs: readonly string[] = SHEET_TABS;
assert(tabs.includes('Abilities') && !tabs.includes('Feats'), `the top tab is Abilities, not Feats: ${tabs.join(', ')}`);

// The Feats sub-tab is today's FeatsTab: it renders the engine's budget line, read from rows only.
const featBudget = readFeatBudget([
  { id: 'feats.budget.general.granted', value: 2 },
  { id: 'feats.budget.general.spent', value: 1 },
  { id: 'feats.budget.general.remaining', value: 1 },
]);
assert(featBudget.pools.length > 0, 'the fixture rows build a feat pool');
const budgetLine = featBudget.pools.map((pool) => `${pool.label}: ${pool.spent} of ${pool.granted}`).join(' · ');
const feats = createElement(FeatsTab, {
  selectedFeats: [],
  chosenFeatTargets: [],
  featBudget,
  admitted: [],
  onAddFeat: () => undefined,
  onRemoveFeat: () => undefined,
});
const html = renderToStaticMarkup(
  createElement(AbilitiesTab, {
    panels: { Feats: feats, 'Racial Abilities': null, Traits: null, Archetype: null, Background: null, 'Class Features': null, Misc: null },
  }),
);

// The sub-tab bar lists the seven labels, in order.
const labels = [...html.matchAll(/role="tab"[^>]*>([^<]+)<\/button>/g)].map((match) => match[1]);
assertEqual(labels.join(), PCGEN_ORDER.join(), 'the sub-tab bar renders the seven labels in order');
assert(html.includes('aria-selected="true"') && /aria-selected="true"[^>]*>Feats</.test(html), 'Feats is the sub-tab open by default');
assert(html.includes(budgetLine), `the Feats sub-tab renders the feat budget line "${budgetLine}"`);
assert(html.includes('Add Feat'), 'the Feats sub-tab keeps its Add Feat control');

// A sub-tab whose engine card has not landed names that card; Feats has no such notice.
// SD-43 E3.3: Traits is on the generic sub-tab with the engine's budget; it names no pending card.
// SD-43 E3.4: Class Features is on the generic sub-tab with the engine's pool grants; it names no pending card.
// SD-43 E3.6: Racial Abilities picks the race's alternate racial traits on the generic sub-tab; it names no pending card.
const PENDING = PCGEN_ORDER.slice(1).filter((name) => name !== 'Traits' && name !== 'Class Features' && name !== 'Racial Abilities');
for (const name of PENDING) {
  assert(typeof PENDING_ENGINE_CARD[name as keyof typeof PENDING_ENGINE_CARD] === 'string', `${name} names its pending engine card`);
}
assert(PENDING_ENGINE_CARD.Feats === undefined, 'Feats has an engine already');
assert(PENDING_ENGINE_CARD.Traits === undefined, 'Traits has an engine already (E3.2) and its UI (E3.3)');
assert(PENDING_ENGINE_CARD['Class Features'] === undefined, 'Class Features has its engine pools and UI (E3.4)');
assert(PENDING_ENGINE_CARD['Racial Abilities'] === undefined, 'Racial Abilities has its alternates, refusals and UI (E3.6)');
const traitsOpen = renderToStaticMarkup(
  createElement(AbilitiesTab, {
    initial: 'Traits',
    panels: { Feats: null, 'Racial Abilities': null, Traits: 'TRAITS-PANEL', Archetype: null, Background: null, 'Class Features': null, Misc: null },
  }),
);
assert(traitsOpen.includes('TRAITS-PANEL') && !traitsOpen.includes('engine card'), 'the Traits sub-tab shows its panel and no notice');
for (const name of PENDING) {
  const sub = renderToStaticMarkup(
    createElement(AbilitiesTab, {
      initial: name as (typeof ABILITY_SUB_TABS)[number],
      panels: { Feats: null, 'Racial Abilities': null, Traits: null, Archetype: null, Background: null, 'Class Features': null, Misc: null },
    }),
  );
  const card = PENDING_ENGINE_CARD[name as keyof typeof PENDING_ENGINE_CARD] ?? '';
  assert(sub.includes(card) && sub.includes('engine card'), `${name}: the notice names ${card}`);
  assert(!sub.includes('Add Feat'), `${name}: the Feats panel is not shown`);
}

console.log('AbilitiesTab tests passed');
