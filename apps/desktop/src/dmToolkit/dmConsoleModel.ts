import { searchDmRecords, type DmRecord, type DmRecordKind } from './dmRecordModel';

/**
 * Pure view logic for the DM console shell (v0.8 D-2). The screen holds
 * the active tab, one search string per tab and the selected record id;
 * this turns those plus the stored records into what the two panes show.
 */
const PLURAL: Record<DmRecordKind, string> = {
  World: 'world notes',
  Timeline: 'timeline entries',
  Place: 'places',
  Person: 'people',
  Clue: 'clues',
  Scene: 'scenes',
  Rule: 'rules',
};

export interface DmConsoleView {
  list: DmRecord[];
  selected: DmRecord | null;
  /** Non-null exactly when `list` is empty; says why. */
  emptyMessage: string | null;
  /** e.g. `"2 people"` or `"1 of 2 people"` when a search narrows. */
  countLabel: string;
}

export function buildDmConsoleView(
  records: readonly DmRecord[],
  kind: DmRecordKind,
  query: string,
  selectedId: string | null,
): DmConsoleView {
  const all = records.filter((record) => record.kind === kind);
  const list = searchDmRecords(records, kind, query);
  const selected = list.find((record) => record.id === selectedId) ?? null;
  const trimmed = query.trim();
  const emptyMessage =
    list.length > 0
      ? null
      : all.length === 0
        ? `No ${PLURAL[kind]} yet.`
        : `No ${PLURAL[kind]} match “${trimmed}”.`;
  const countLabel = trimmed === '' || list.length === all.length ? `${all.length} ${PLURAL[kind]}` : `${list.length} of ${all.length} ${PLURAL[kind]}`;
  return { list, selected, emptyMessage, countLabel };
}

/** Selection to carry into a tab: keep it if it is of that kind, else the kind's first record, else none. */
export function nextSelectionForKind(records: readonly DmRecord[], kind: DmRecordKind, currentId: string | null): string | null {
  const current = records.find((record) => record.id === currentId);
  if (current && current.kind === kind) {
    return current.id;
  }
  return records.find((record) => record.kind === kind)?.id ?? null;
}

export function kindPlural(kind: DmRecordKind): string {
  return PLURAL[kind];
}
