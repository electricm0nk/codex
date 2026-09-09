import { DM_KIND_FIELDS, type CreateDmRecordInput, type DmRecord, type DmRecordKind, type DmVisibility } from './dmRecordModel';

/**
 * The create/edit form's draft (v0.8 D-3): the fields the DM edits, kept
 * apart from the stored record so a cancelled edit changes nothing and a
 * saved one writes exactly what was typed (trimmed). Links are not part of
 * the draft — the link editor (D-4) writes them separately, so saving a
 * title edit can never clobber a record's links.
 */
export interface DmRecordDraft {
  kind: DmRecordKind;
  title: string;
  summary: string;
  body: string;
  /** One entry per `DM_KIND_FIELDS[kind]`, always present. */
  fields: Record<string, string>;
  /** D-3a: GM-only by default; the DM shares a record deliberately. */
  visibility: DmVisibility;
}

export function emptyDraft(kind: DmRecordKind): DmRecordDraft {
  return {
    kind,
    title: '',
    summary: '',
    body: '',
    fields: Object.fromEntries(DM_KIND_FIELDS[kind].map((spec) => [spec.key, ''])),
    visibility: 'gm',
  };
}

export function draftFromRecord(record: DmRecord): DmRecordDraft {
  return {
    kind: record.kind,
    title: record.title,
    summary: record.summary,
    body: record.body,
    fields: Object.fromEntries(DM_KIND_FIELDS[record.kind].map((spec) => [spec.key, record.fields[spec.key] ?? ''])),
    visibility: record.visibility,
  };
}

export function validateDraft(draft: DmRecordDraft): string[] {
  return draft.title.trim() === '' ? ['Title is required.'] : [];
}

/** Trimmed, with blank per-kind fields dropped; never carries `links`. */
export function draftToInput(draft: DmRecordDraft): Omit<CreateDmRecordInput, 'links'> {
  const fields = Object.fromEntries(
    Object.entries(draft.fields)
      .map(([key, value]) => [key, value.trim()] as const)
      .filter(([, value]) => value !== ''),
  );
  return { kind: draft.kind, title: draft.title.trim(), summary: draft.summary.trim(), body: draft.body, fields, visibility: draft.visibility };
}
