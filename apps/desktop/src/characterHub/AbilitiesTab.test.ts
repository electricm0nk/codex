import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { readFeatBudget } from './characterProgression';
import { ABILITY_SUB_TABS, AbilitiesTab } from './AbilitiesTab';
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

// SD-43 E3.9: every sub-tab has its engine (Traits E3.2/E3.3, Class Features E3.4, Racial Abilities E3.6,
// Archetype E3.7, Background and Misc E3.9): none renders a pending-engine notice; each shows its panel.
for (const name of PCGEN_ORDER) {
  const sub = renderToStaticMarkup(
    createElement(AbilitiesTab, {
      initial: name as (typeof ABILITY_SUB_TABS)[number],
      panels: { Feats: 'P-Feats', 'Racial Abilities': 'P-Racial Abilities', Traits: 'P-Traits', Archetype: 'P-Archetype', Background: 'P-Background', 'Class Features': 'P-Class Features', Misc: 'P-Misc' },
    }),
  );
  assert(sub.includes(`P-${name}`) && !sub.includes('engine card'), `${name}: its panel and no pending-engine notice`);
}

console.log('AbilitiesTab tests passed');
