import { SHEET_TABS } from './CharacterSheet';
import { assert } from '../testSupport/asserts';

/**
 * F-5 (scout audit item 57): the sheet offered an `Overrides` tab whose only
 * behavior was the generic "Overrides — coming soon." placeholder. A visible
 * affordance with nothing behind it violates the no-stub doctrine
 * (`docs/governance/no-stub-mvp-doctrine.md`). The orchestrator ruled to
 * remove the tab rather than invent a feature to fill it. This pins that
 * every tab the sheet offers is one with a real panel behind it.
 */
const tabs: readonly string[] = SHEET_TABS;
assert(!tabs.includes('Overrides'), 'no Overrides tab: nothing is wired behind it');
assert(
  ['Weapons', 'Defense', 'Gear', 'Spells', 'Pets', 'Feats', 'Actions'].every((t) => tabs.includes(t)),
  'every wired tab is still offered',
);
console.log('sheetTabs tests passed');
