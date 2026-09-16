import { historyFor } from './dmHistoryModel';
import type { DmRecord } from './dmRecordModel';
import { assertEqual } from '../testSupport/asserts';

/**
 * v0.8 D-8: a record's history is the Timeline entries linked to it, in
 * INSERTION order. `when` is a display label, never parsed: "later that
 * night" has no sort key, and a parser that worked on tidy demo data would
 * scramble a real DM's entries silently. Derived from links that already
 * exist — no new storage.
 */
function rec(id: string, kind: DmRecord['kind'], title: string, extra: Partial<DmRecord> = {}): DmRecord {
  return { id, kind, title, summary: '', body: '', fields: {}, links: [], visibility: 'gm', createdAt: '', updatedAt: '', ...extra };
}
const rogue = rec('rogue', 'Person', 'Rogue', { links: [{ targetId: 't3', label: 'Involves' }] });
const t1 = rec('t1', 'Timeline', 'Night 1 — 22:00', { fields: { when: 'Night 1, 22:00' }, links: [{ targetId: 'rogue', label: 'Involves' }] });
const t2 = rec('t2', 'Timeline', 'Later that night', { fields: { when: 'later that night' }, links: [{ targetId: 'rogue', label: 'Involves' }, { targetId: 'bar', label: 'Location' }] });
const t3 = rec('t3', 'Timeline', 'Three days earlier', { fields: { when: 'three days earlier' } });
const bar = rec('bar', 'Place', 'Afterlife');
const unrelated = rec('t4', 'Timeline', 'Elsewhere');
const all = [t3, rogue, t1, t2, bar, unrelated];

assertEqual(historyFor(rogue, all).map((r) => r.id).join(), 't3,t1,t2', 'entries linked in either direction, in stored (insertion) order — not parsed from `when`');
assertEqual(historyFor(bar, all).map((r) => r.id).join(), 't2', 'a place gets only the entries that link to it');
assertEqual(historyFor(unrelated, all).length, 0, 'a Timeline entry has no history of its own');
assertEqual(historyFor(rec('x', 'Rule', 'Checks'), all).length, 0, 'nothing linked → empty, never fabricated');
assertEqual(historyFor(rogue, [rogue, t1]).map((r) => r.id).join(), 't1', 'derived against whatever record set is passed — the handout passes its filtered subset');

console.log('dmHistoryModel tests passed');
