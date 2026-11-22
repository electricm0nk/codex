import { buildDmConsoleView, nextSelectionForKind } from './dmConsoleModel';
import type { DmRecord } from './dmRecordModel';
import { assertEqual } from '../testSupport/asserts';

/**
 * v0.8 D-2: the console shell's pure view logic — which records the
 * left list shows for the active tab and search, which one the right pane
 * shows, and what an empty list says. Empty must be honest: "no records of
 * this kind yet" and "none match this search" are different facts.
 */
function rec(id: string, kind: DmRecord['kind'], title: string, summary = ''): DmRecord {
  return { id, kind, title, summary, body: '', fields: {}, links: [], visibility: 'gm', createdAt: '2026-09-01T00:00:00Z', updatedAt: '2026-09-01T00:00:00Z' };
}
const records = [rec('a', 'Person', 'Rogue', 'Fixer'), rec('b', 'Person', 'Viktor', 'Ripperdoc'), rec('c', 'Place', 'Afterlife')];

const people = buildDmConsoleView(records, 'Person', '', 'b');
assertEqual(people.list.map((r) => r.id).join(), 'a,b', 'list is the kind, in stored order');
assertEqual(people.selected?.id, 'b', 'the selected record is the one asked for');
assertEqual(people.emptyMessage, null, 'a non-empty list has no empty message');
assertEqual(people.countLabel, '2 people', 'count uses the kind label');

const searched = buildDmConsoleView(records, 'Person', 'vik', 'a');
assertEqual(searched.list.map((r) => r.id).join(), 'b', 'search narrows the list');
assertEqual(searched.selected, null, 'a selection outside the narrowed list is not shown as if it matched');
assertEqual(searched.countLabel, '1 of 2 people', 'count shows the narrowing');

const noMatch = buildDmConsoleView(records, 'Person', 'zzz', null);
assertEqual(noMatch.emptyMessage, 'No people match “zzz”.', 'no-match is stated as a search result');

const none = buildDmConsoleView(records, 'Scene', '', null);
assertEqual(none.emptyMessage, 'No scenes yet.', 'an empty kind says so');
assertEqual(none.countLabel, '0 scenes', 'zero count');

assertEqual(buildDmConsoleView(records, 'World', '', null).emptyMessage, 'No world notes yet.', 'kind-specific wording');

assertEqual(nextSelectionForKind(records, 'Person', 'c'), 'a', 'switching to a tab whose selection is another kind picks the first record of the new kind');
assertEqual(nextSelectionForKind(records, 'Person', 'b'), 'b', 'a selection already of that kind is kept');
assertEqual(nextSelectionForKind(records, 'Scene', 'b'), null, 'an empty kind selects nothing');

console.log('dmConsoleModel tests passed');
