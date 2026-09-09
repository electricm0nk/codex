/**
 * DM Toolkit record store (v0.8 D-1).
 *
 * The console's records — the six NoDA kinds, each a short prose body plus
 * thin per-kind fields plus typed links to other records — live in
 * localStorage keyed per campaign, mirroring `campaign/campaignModel.ts`:
 * localStorage is the source of truth, reads are guarded so a corrupt or
 * absent value reads as "no records" rather than an exception, and every
 * mutation writes the whole list back. (File-backed storage is the first
 * v0.9 decision; see `docs/release/v0.8/dm-toolkit-build.md` Q-DM1.)
 *
 * Storage is injected so the model is testable where no localStorage
 * exists (the Node test runtime) and so the export path can read the same
 * store the console writes. Production callers omit it and get the
 * browser's localStorage; if even that is missing, an in-memory store
 * keeps the console working for the session.
 *
 * Nothing here is computed from rules: every value is text the DM typed.
 */

export const DM_RECORD_KINDS = ['World', 'Timeline', 'Place', 'Person', 'Clue', 'Scene', 'Rule'] as const;
export type DmRecordKind = (typeof DM_RECORD_KINDS)[number];

/** Human labels for each kind's tab and its "New …" affordance. */
export const DM_KIND_LABELS: Record<DmRecordKind, { tab: string; singular: string }> = {
  World: { tab: 'World', singular: 'World note' },
  Timeline: { tab: 'Timeline', singular: 'Timeline entry' },
  Place: { tab: 'Places', singular: 'Place' },
  Person: { tab: 'People', singular: 'Person' },
  Clue: { tab: 'Clues', singular: 'Clue' },
  Scene: { tab: 'Scenes', singular: 'Scene' },
  Rule: { tab: 'Rules', singular: 'Rule' },
};

/** A thin per-kind extra field. Kept deliberately small in v1 (Q-DM2). */
export interface DmFieldSpec {
  key: string;
  label: string;
  /** Multi-line text (stat block, read-aloud) vs a single line. */
  multiline: boolean;
}

export const DM_KIND_FIELDS: Record<DmRecordKind, DmFieldSpec[]> = {
  World: [],
  Timeline: [{ key: 'when', label: 'When', multiline: false }],
  Place: [
    { key: 'gridRef', label: 'Map grid ref', multiline: false },
    { key: 'role', label: 'Role', multiline: false },
  ],
  Person: [
    { key: 'role', label: 'Role', multiline: false },
    { key: 'motivation', label: 'Motivation', multiline: false },
    { key: 'statBlock', label: 'Stat block', multiline: true },
  ],
  Clue: [
    { key: 'trigger', label: 'Revealed when', multiline: false },
  ],
  Scene: [
    { key: 'trigger', label: 'Trigger', multiline: false },
    { key: 'gmBrief', label: 'GM brief', multiline: true },
    { key: 'readAloud', label: 'Read aloud', multiline: true },
    /** v0.8 G-4: a rated encounter as the engine wrote it (see `encounterRecord.ts`). */
    { key: 'encounter', label: 'Encounter', multiline: true },
  ],
  Rule: [],
};

/** Suggested relation labels per kind; a link may carry any label. */
export const DM_LINK_LABELS: Record<DmRecordKind, string[]> = {
  World: ['Related'],
  Timeline: ['Involves', 'Location', 'Related'],
  Place: ['Resident', 'Clue held here', 'Part of', 'Related'],
  Person: ['Home base', 'Ally', 'Rival', 'Reveals clue', 'Part of', 'Related'],
  Clue: ['Found at', 'Revealed by', 'Scene', 'Related'],
  Scene: ['Location', 'NPC', 'Clue', 'Related'],
  Rule: ['Related'],
};

/** A typed, directed link stored on its source record. */
export interface DmRecordLink {
  targetId: string;
  /** The relation, e.g. `"Home base"`. */
  label: string;
}

/**
 * Who a record is for. The World Anvil "secrets" model: a DM authors one set of
 * records and can hand players a copy with the GM-only ones stripped (D-5's
 * player handout). Defaults to `'gm'` everywhere -- prep is private until the DM
 * deliberately shares it, because a tool that leaks prep by default is a trust
 * failure, not a cosmetic one.
 */
export type DmVisibility = 'gm' | 'players';

export interface DmRecord {
  id: string;
  kind: DmRecordKind;
  title: string;
  /** One line, shown in the list and under the title. */
  summary: string;
  /** Markdown body. */
  body: string;
  /** Values for `DM_KIND_FIELDS[kind]`, by key. */
  fields: Record<string, string>;
  links: DmRecordLink[];
  /** `'gm'` (default) hides the record from the player handout. */
  visibility: DmVisibility;
  createdAt: string;
  updatedAt: string;
}

export interface DmStorage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

/** In-memory storage: the test double, and the fallback when the host has no localStorage. */
export function createMemoryDmStorage(): DmStorage {
  const map = new Map<string, string>();
  return {
    getItem: (key) => map.get(key) ?? null,
    setItem: (key, value) => {
      map.set(key, value);
    },
    removeItem: (key) => {
      map.delete(key);
    },
  };
}

let fallbackStorage: DmStorage | null = null;

function defaultStorage(): DmStorage {
  try {
    if (typeof localStorage !== 'undefined') {
      return localStorage;
    }
  } catch {
    /* some hosts throw on access; fall through */
  }
  fallbackStorage ??= createMemoryDmStorage();
  return fallbackStorage;
}

const RECORDS_KEY_PREFIX = 'codex.dmToolkit.records.';

export function dmRecordsStorageKey(campaignId: string): string {
  return RECORDS_KEY_PREFIX + campaignId;
}

export function getDmRecords(campaignId: string, storage: DmStorage = defaultStorage()): DmRecord[] {
  try {
    const raw = storage.getItem(dmRecordsStorageKey(campaignId));
    if (!raw) {
      return [];
    }
    const parsed: unknown = JSON.parse(raw);
    if (!Array.isArray(parsed)) {
      return [];
    }
    // Records written before visibility existed carry no such field. They load
    // as 'gm' -- never as visible -- so an upgrade cannot retroactively expose a
    // DM's existing prep to players.
    return (parsed as DmRecord[]).map((record) => ({
      ...record,
      visibility: record.visibility === 'players' ? 'players' : 'gm',
    }));
  } catch {
    return [];
  }
}

function saveDmRecords(campaignId: string, records: DmRecord[], storage: DmStorage): void {
  try {
    storage.setItem(dmRecordsStorageKey(campaignId), JSON.stringify(records));
  } catch {
    /* non-persistent environments still get the change applied this session */
  }
}

export function getDmRecord(campaignId: string, id: string, storage: DmStorage = defaultStorage()): DmRecord | null {
  return getDmRecords(campaignId, storage).find((record) => record.id === id) ?? null;
}

export interface CreateDmRecordInput {
  kind: DmRecordKind;
  title: string;
  summary?: string;
  body?: string;
  fields?: Record<string, string>;
  links?: DmRecordLink[];
  visibility?: DmVisibility;
}

export function createDmRecord(
  campaignId: string,
  input: CreateDmRecordInput,
  storage: DmStorage = defaultStorage(),
): DmRecord {
  const now = new Date().toISOString();
  const record: DmRecord = {
    id: crypto.randomUUID(),
    kind: input.kind,
    title: input.title,
    summary: input.summary ?? '',
    body: input.body ?? '',
    fields: { ...(input.fields ?? {}) },
    links: [...(input.links ?? [])],
    visibility: input.visibility ?? 'gm',
    createdAt: now,
    updatedAt: now,
  };
  saveDmRecords(campaignId, [...getDmRecords(campaignId, storage), record], storage);
  return record;
}

export type DmRecordChanges = Partial<Pick<DmRecord, 'title' | 'summary' | 'body' | 'fields' | 'links' | 'visibility'>>;

export function updateDmRecord(
  campaignId: string,
  id: string,
  changes: DmRecordChanges,
  storage: DmStorage = defaultStorage(),
): DmRecord | null {
  const records = getDmRecords(campaignId, storage);
  const index = records.findIndex((record) => record.id === id);
  if (index === -1) {
    return null;
  }
  const updated: DmRecord = { ...records[index], ...changes, updatedAt: new Date().toISOString() };
  records[index] = updated;
  saveDmRecords(campaignId, records, storage);
  return updated;
}

/**
 * Deletes the record and every link any other record held to it. A link
 * to a record that no longer exists would render as a button that focuses
 * nothing — exactly the affordance the no-stub doctrine forbids — so the
 * store removes it at the source rather than leaving the UI to hide it.
 */
export function deleteDmRecord(campaignId: string, id: string, storage: DmStorage = defaultStorage()): void {
  const remaining = getDmRecords(campaignId, storage)
    .filter((record) => record.id !== id)
    .map((record) =>
      record.links.some((link) => link.targetId === id)
        ? { ...record, links: record.links.filter((link) => link.targetId !== id) }
        : record,
    );
  saveDmRecords(campaignId, remaining, storage);
}

export function linkDmRecords(
  campaignId: string,
  sourceId: string,
  targetId: string,
  label: string,
  storage: DmStorage = defaultStorage(),
): DmRecord | null {
  const source = getDmRecord(campaignId, sourceId, storage);
  if (!source || sourceId === targetId || !getDmRecord(campaignId, targetId, storage)) {
    return null;
  }
  if (source.links.some((link) => link.targetId === targetId && link.label === label)) {
    return source;
  }
  return updateDmRecord(campaignId, sourceId, { links: [...source.links, { targetId, label }] }, storage);
}

export function unlinkDmRecords(
  campaignId: string,
  sourceId: string,
  targetId: string,
  storage: DmStorage = defaultStorage(),
): DmRecord | null {
  const source = getDmRecord(campaignId, sourceId, storage);
  if (!source) {
    return null;
  }
  return updateDmRecord(campaignId, sourceId, { links: source.links.filter((link) => link.targetId !== targetId) }, storage);
}

/** Records that hold a link to `targetId` — derived, never stored twice. */
export function backlinksTo(records: readonly DmRecord[], targetId: string): DmRecord[] {
  return records.filter((record) => record.links.some((link) => link.targetId === targetId));
}

/** Case-insensitive search within one kind over title, summary, body and field values. Blank lists all. */
export function searchDmRecords(records: readonly DmRecord[], kind: DmRecordKind, query: string): DmRecord[] {
  const needle = query.trim().toLowerCase();
  return records.filter((record) => {
    if (record.kind !== kind) {
      return false;
    }
    if (needle === '') {
      return true;
    }
    const haystack = [record.title, record.summary, record.body, ...Object.values(record.fields)].join('\n').toLowerCase();
    return haystack.includes(needle);
  });
}
