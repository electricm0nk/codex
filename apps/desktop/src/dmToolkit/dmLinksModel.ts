import type { DmRecord } from './dmRecordModel';

/** A stored link joined to its target record, ready to render as a focus button. */
export interface ResolvedDmLink {
  label: string;
  target: DmRecord;
}

/**
 * The record's links, each joined to its live target (v0.8 D-4). A link
 * whose target is absent is dropped: the store cascades deletes so this
 * should not happen, but a button that focuses nothing must never render.
 */
export function resolveLinks(record: DmRecord, records: readonly DmRecord[]): ResolvedDmLink[] {
  return record.links.flatMap((link) => {
    const target = records.find((candidate) => candidate.id === link.targetId);
    return target ? [{ label: link.label, target }] : [];
  });
}

/** Records a link may point at: everything but the record itself, in stored order. */
export function linkCandidates(record: DmRecord, records: readonly DmRecord[]): DmRecord[] {
  return records.filter((candidate) => candidate.id !== record.id);
}
