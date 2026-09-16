import type { DmRecord } from './dmRecordModel';

/**
 * A record's history (v0.8 D-8, World Anvil's auto-derived "History" and
 * NoDA's per-NPC timeline): every Timeline entry linked to it in either
 * direction, in the order the DM created them.
 *
 * Deliberately insertion order. `Timeline.when` is free text ("18:30",
 * "later that night", "three days earlier") and is shown as a label only;
 * parsing it would sort tidy demo data correctly and a real campaign
 * wrongly, without any error. An explicit sort key is a v0.9 question.
 *
 * Callers pass the record set they are rendering from — the player handout
 * passes its shared subset, so a GM-only entry cannot appear in a shared
 * record's history.
 */
export function historyFor(record: DmRecord, records: readonly DmRecord[]): DmRecord[] {
  if (record.kind === 'Timeline') {
    return [];
  }
  const outgoing = new Set(record.links.map((link) => link.targetId));
  return records.filter(
    (candidate) =>
      candidate.kind === 'Timeline' &&
      candidate.id !== record.id &&
      (outgoing.has(candidate.id) || candidate.links.some((link) => link.targetId === record.id)),
  );
}
