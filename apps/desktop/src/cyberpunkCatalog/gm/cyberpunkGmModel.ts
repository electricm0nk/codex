import type { CyberpunkRecord } from '../../boundary/loadCyberpunkCatalog';
import { describeValue, fieldLabel, recordName } from '../cyberpunkCatalogModel';

/**
 * SD-38 E6.4 -- the GM reference pages (every `print_only` kind of the Cyberpunk RED corpus) as
 * read-only blocks. Holds no record and computes no number: a page is the corpus's own rows,
 * laid out as the book prints them (scalars as a fact line, a list of rows as a table, a list of
 * strings as a list). The generators the book describes (NET Architecture, Night Market, Mooks)
 * are not built here; the tables print so the GM rolls on them.
 */

export interface GmTable {
  caption: string;
  columns: string[];
  rows: string[][];
}

export interface GmEntry {
  heading: string;
  page: string | null;
  facts: { label: string; text: string }[];
  paragraphs: string[];
  lists: { label: string; items: string[] }[];
  tables: GmTable[];
}

/** Fields the entry shows in its heading or paragraph slots, not as facts. */
const BODY_FIELDS = new Set(['name', 'kind', 'source_page', 'text', 'intro_text', 'worked_example', 'generation_steps', 'transcription_note']);

/** A table cell: a cell object that prints a `text` shows it, anything else prints field by field. */
function cell(value: unknown): string {
  if (value !== null && typeof value === 'object' && !Array.isArray(value)) {
    const text = (value as Record<string, unknown>).text;
    if (typeof text === 'string') {
      return text;
    }
  }
  return describeValue(value);
}

/** A list of row objects as a table; nested objects (`values`, `floors`) flatten into their own columns. */
export function rowsToTable(caption: string, rows: readonly Record<string, unknown>[], declared?: readonly string[]): GmTable {
  const flat = rows.map((row) => {
    const out = new Map<string, string>();
    for (const [key, value] of Object.entries(row)) {
      if (value !== null && typeof value === 'object' && !Array.isArray(value) && !('text' in (value as object))) {
        for (const [inner, innerValue] of Object.entries(value as Record<string, unknown>)) {
          out.set(inner, cell(innerValue));
        }
      } else {
        out.set(fieldLabel(key), cell(value));
      }
    }
    return out;
  });
  const columns: string[] = [];
  for (const name of declared ?? []) {
    columns.push(name);
  }
  for (const row of flat) {
    for (const key of row.keys()) {
      if (!columns.includes(key) && !columns.map((c) => fieldLabel(c)).includes(key)) {
        columns.push(key);
      }
    }
  }
  const lookup = (row: Map<string, string>, column: string) => row.get(column) ?? row.get(fieldLabel(column)) ?? '';
  return { caption, columns, rows: flat.map((row) => columns.map((column) => lookup(row, column))) };
}

function isRowList(value: unknown): value is Record<string, unknown>[] {
  return Array.isArray(value) && value.length > 0 && value.every((item) => item !== null && typeof item === 'object' && !Array.isArray(item));
}

function isStringList(value: unknown): value is string[] {
  return Array.isArray(value) && value.length > 0 && value.every((item) => typeof item === 'string');
}

function isScalarObject(value: unknown): value is Record<string, unknown> {
  return (
    value !== null &&
    typeof value === 'object' &&
    !Array.isArray(value) &&
    Object.values(value as object).every((item) => item === null || typeof item !== 'object')
  );
}

/** One record as a GM entry. */
export function gmEntry(record: CyberpunkRecord): GmEntry {
  const entry: GmEntry = {
    heading: recordName(record),
    page: typeof record.source_page === 'string' ? record.source_page.replace(/^p0*/, '') : null,
    facts: [],
    paragraphs: [],
    lists: [],
    tables: [],
  };
  for (const key of ['intro_text', 'text', 'worked_example'] as const) {
    const value = record[key];
    if (typeof value === 'string' && value.length > 0) {
      entry.paragraphs.push(value);
    }
  }
  if (isStringList(record.generation_steps)) {
    entry.lists.push({ label: 'Generation steps', items: record.generation_steps });
  }
  for (const [field, value] of Object.entries(record)) {
    if (BODY_FIELDS.has(field) || field === 'also_on' || field === 'continues_on') {
      continue;
    }
    const label = fieldLabel(field);
    if (isRowList(value)) {
      entry.tables.push(rowsToTable(label, value, field === 'rows' && isStringList(record.columns) ? ['Roll', ...record.columns] : undefined));
    } else if (isStringList(value)) {
      entry.lists.push({ label, items: value });
    } else if (isScalarObject(value) && field === 'columns') {
      entry.tables.push({ caption: label, columns: Object.keys(value), rows: [Object.values(value).map(cell)] });
    } else if (isScalarObject(value)) {
      entry.tables.push({ caption: label, columns: Object.keys(value).map(fieldLabel), rows: [Object.values(value).map(cell)] });
    } else {
      const text = describeValue(value);
      if (text.length > 0) {
        entry.facts.push({ label, text });
      }
    }
  }
  if (typeof record.transcription_note === 'string' && record.transcription_note.length > 0) {
    entry.paragraphs.push(`Transcription note: ${record.transcription_note}`);
  }
  return entry;
}
