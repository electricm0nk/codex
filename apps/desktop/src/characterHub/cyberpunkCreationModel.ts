import type {
  CyberpunkCreateRequest,
  CyberpunkCreationMethod,
  CyberpunkCreationPreview,
  CyberpunkCreationProblem,
  CyberpunkCreationRequest,
  CyberpunkCreationRow,
  CyberpunkLifepathRequest,
  CyberpunkStatValue,
} from '../boundary/cprCreation';
import type { CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';

/**
 * Pure state for the Cyberpunk RED creation flow (`CyberpunkCreateForm.tsx`): the method picker
 * and the STAT step. The form holds only the player's choices (name, Role, method, the picked
 * Streetrat row, the Complete Package allocation) and the roller's seed; every number it shows
 * (each STAT, the points spent and to spend, the derived rows) is the engine's answer from
 * `preview_cyberpunk_character`. Nothing here is a Cyberpunk RED rules table.
 *
 * The Lifepath (E3.3): the draft holds only the player's picks, counts, not-taken branch tables
 * and Cultural Origin language; the tables offered and their rows are the `lifepath_table`
 * records, and every answer not picked is the engine's roll.
 */

export interface CyberpunkCreationDraft {
  displayLabel: string;
  /** A `role` record name, e.g. `Solo`; null until chosen. */
  role: string | null;
  method: CyberpunkCreationMethod;
  /** The roller's seed: null asks the engine to draw one; kept once drawn so create rolls the same dice. */
  seed: number | null;
  /** Streetrat: the picked template row; null rolls it on 1d10. */
  templateRoll: number | null;
  /** Complete Package: the allocation keyed by STAT id. */
  stats: Record<string, number>;
  /** The Lifepath choices; everything not chosen is rolled with the seed. */
  lifepath: CyberpunkLifepathRequest;
}

export const EMPTY_LIFEPATH: CyberpunkLifepathRequest = { picks: [], counts: {}, skips: [], culturalLanguage: null };

export const EMPTY_CYBERPUNK_DRAFT: CyberpunkCreationDraft = {
  displayLabel: '',
  role: null,
  method: 'streetrat',
  seed: null,
  templateRoll: null,
  stats: {},
  lifepath: EMPTY_LIFEPATH,
};

/** The three ways to make STATs (p. 73), in the book's order. */
export const CYBERPUNK_CREATION_METHODS: readonly { id: CyberpunkCreationMethod; label: string; detail: string }[] = [
  { id: 'streetrat', label: 'Streetrat', detail: "One row of your Role's template, rolled on 1d10 or picked." },
  { id: 'edgerunner', label: 'Edgerunner', detail: "Roll 1d10 for each STAT on your Role's template." },
  { id: 'completePackage', label: 'Complete Package', detail: 'Spend the STAT points yourself.' },
];

/** The STAT-step request for the draft; null until a Role is chosen. */
export function buildCyberpunkPreviewRequest(draft: CyberpunkCreationDraft): CyberpunkCreationRequest | null {
  if (draft.role === null) {
    return null;
  }
  return {
    role: draft.role,
    method: draft.method,
    seed: draft.seed,
    templateRoll: draft.method === 'streetrat' ? draft.templateRoll : null,
    stats: draft.method === 'completePackage' ? { ...draft.stats } : null,
    lifepath: {
      picks: draft.lifepath.picks.map((pick) => ({ ...pick })),
      counts: { ...draft.lifepath.counts },
      skips: [...draft.lifepath.skips],
      culturalLanguage: draft.lifepath.culturalLanguage,
    },
  };
}

/** The create request: a fresh identity plus exactly the previewed STAT step (same seed, same dice). */
export function buildCyberpunkCreateRequest(
  draft: CyberpunkCreationDraft,
  deps: { generateId: () => string; now: () => string },
): CyberpunkCreateRequest | null {
  const creation = buildCyberpunkPreviewRequest(draft);
  if (creation === null) {
    return null;
  }
  return {
    characterId: deps.generateId(),
    displayLabel: draft.displayLabel.trim(),
    savedAt: deps.now(),
    creation,
  };
}

/** Keeps the seed the engine drew; the same seed returns the same draft (no preview loop). */
export function withPreviewSeed(draft: CyberpunkCreationDraft, preview: Pick<CyberpunkCreationPreview, 'seed'>): CyberpunkCreationDraft {
  return draft.seed === preview.seed ? draft : { ...draft, seed: preview.seed };
}

/** Drops the seed so the next preview draws fresh dice. */
export function rollAgain(draft: CyberpunkCreationDraft): CyberpunkCreationDraft {
  return { ...draft, seed: null };
}

/**
 * Changes the method: the dice and the Streetrat row reset; a Complete Package with no allocation
 * yet starts from the STATs on screen (`shown`, the engine's preview), which the player then moves.
 */
export function selectMethod(
  draft: CyberpunkCreationDraft,
  method: CyberpunkCreationMethod,
  shown: readonly CyberpunkStatValue[],
): CyberpunkCreationDraft {
  if (draft.method === method) {
    return draft;
  }
  const startFromShown = method === 'completePackage' && Object.keys(draft.stats).length === 0;
  const stats = startFromShown ? Object.fromEntries(shown.map((stat) => [stat.id, stat.value])) : draft.stats;
  return { ...draft, method, seed: null, templateRoll: null, stats };
}

/** One point up or down on a STAT, never below zero; the 2-8 range and the total are the engine's to refuse. */
export function adjustStat(stats: Record<string, number>, id: string, delta: number): Record<string, number> {
  return { ...stats, [id]: Math.max(0, (stats[id] ?? 0) + delta) };
}

export interface TemplateRowOption {
  roll: number;
  label: string;
}

/** The chosen Role's Streetrat rows, read from its `streetrat_template` records, by roll. */
export function templateRowOptions(records: readonly CyberpunkRecord[], role: string | null): TemplateRowOption[] {
  if (role === null) {
    return [];
  }
  return records
    .filter((record) => record.role === role && typeof record.roll === 'number')
    .map((record) => ({
      roll: record.roll as number,
      label: typeof record.text === 'string' ? record.text : `Roll ${String(record.roll)}`,
    }))
    .sort((a, b) => a.roll - b.roll);
}

/** The problems that keep the character from being created. */
export function blockingProblems(preview: Pick<CyberpunkCreationPreview, 'problems'>): CyberpunkCreationProblem[] {
  return preview.problems.filter((problem) => problem.claimBlocking);
}

export interface CyberpunkTile {
  id: string;
  label: string;
  value: number;
}

/** The derived rows the Create screen shows, in sheet order. */
const DERIVED_ROWS: readonly { id: string; label: string }[] = [
  { id: 'cpr.derived.hit_points', label: 'Hit Points' },
  { id: 'cpr.derived.seriously_wounded', label: 'Seriously Wounded' },
  { id: 'cpr.derived.death_save', label: 'Death Save' },
  { id: 'cpr.derived.humanity_max', label: 'Humanity' },
  { id: 'cpr.derived.emp_current', label: 'Current EMP' },
];

/** One tile per `cpr.derived.*` row the engine returned; a row it did not return gets no tile. */
export function cyberpunkDerivedTiles(rows: readonly CyberpunkCreationRow[]): CyberpunkTile[] {
  return DERIVED_ROWS.flatMap((wanted) => {
    const found = rows.find((row) => row.id === wanted.id);
    return found ? [{ id: found.id, label: wanted.label, value: found.value }] : [];
  });
}

/** One tile per `cpr.stat.*` row the engine returned, labelled by the engine's STAT list. */
export function cyberpunkStatTiles(rows: readonly CyberpunkCreationRow[], stats: readonly CyberpunkStatValue[]): CyberpunkTile[] {
  return stats.flatMap((stat) => {
    const found = rows.find((row) => row.id === `cpr.stat.${stat.id}`);
    return found ? [{ id: found.id, label: stat.label, value: found.value }] : [];
  });
}

// ---- Lifepath (E3.3) ----

/** The Lifepath tables offered for `role`: every general `lifepath_table` record, then the Role's, corpus order. */
export function lifepathTablesFor(records: readonly CyberpunkRecord[], role: string | null): CyberpunkRecord[] {
  if (role === null) {
    return [];
  }
  return [...records.filter((r) => r.group === 'general'), ...records.filter((r) => r.group === 'role' && r.role === role)];
}

interface LifepathRecordRow {
  roll: number | string;
  cells: string[];
}

function recordRows(record: CyberpunkRecord): LifepathRecordRow[] {
  return Array.isArray(record.rows) ? (record.rows as LifepathRecordRow[]) : [];
}

function perColumn(record: CyberpunkRecord): boolean {
  const instruction = typeof record.roll_instruction === 'string' ? record.roll_instruction : '';
  return instruction.includes('each column') || instruction.includes('each of the columns');
}

export interface LifepathRowOption {
  /** The roll a pick sends (the first roll of a "1-2" row). */
  roll: number;
  label: string;
}

/**
 * The rows a Lifepath answer can be picked from, read from the record: the printed roll and the
 * cell of `column` for a table rolled "for each column", the whole row otherwise.
 */
export function lifepathRowOptions(record: CyberpunkRecord, column: number): LifepathRowOption[] {
  const each = perColumn(record);
  return recordRows(record).map((row) => {
    const printed = String(row.roll);
    const cells = each ? [row.cells[column] ?? ''] : row.cells;
    return { roll: Number(printed.split('-')[0]), label: `${printed}: ${cells.join(' / ')}` };
  });
}

/** The Cultural Origins record's languages for the rolled or picked region. */
export function culturalLanguageOptions(record: CyberpunkRecord, regionRoll: number | null): string[] {
  if (regionRoll === null || typeof record.languages_by_roll !== 'object' || record.languages_by_roll === null) {
    return [];
  }
  const listed = (record.languages_by_roll as Record<string, unknown>)[String(regionRoll)];
  return Array.isArray(listed) ? listed.map(String) : [];
}

/** Picks one answer's row; null drops the pick so the engine rolls it. The seed is kept. */
export function setLifepathPick(
  draft: CyberpunkCreationDraft,
  table: string,
  entry: number,
  column: number,
  roll: number | null,
): CyberpunkCreationDraft {
  const others = draft.lifepath.picks.filter((p) => !(p.table === table && p.entry === entry && p.column === column));
  const picks = roll === null ? others : [...others, { table, entry, column, roll }];
  return { ...draft, lifepath: { ...draft.lifepath, picks } };
}

/** Picks how many entries a count table has; null rolls it. A smaller count drops the picks of the removed entries. */
export function setLifepathCount(draft: CyberpunkCreationDraft, table: string, count: number | null): CyberpunkCreationDraft {
  const counts = { ...draft.lifepath.counts };
  let picks = draft.lifepath.picks;
  if (count === null) {
    delete counts[table];
  } else {
    counts[table] = count;
    picks = picks.filter((p) => p.table !== table || p.entry <= count);
  }
  return { ...draft, lifepath: { ...draft.lifepath, counts, picks } };
}

/** Marks a table behind a printed choice as not taken (or taken again). */
export function setLifepathSkip(draft: CyberpunkCreationDraft, table: string, skipped: boolean): CyberpunkCreationDraft {
  const others = draft.lifepath.skips.filter((name) => name !== table);
  return { ...draft, lifepath: { ...draft.lifepath, skips: skipped ? [...others, table] : others } };
}

/** The Cultural Origin language; empty rolls one of the region's. */
export function setCulturalLanguage(draft: CyberpunkCreationDraft, name: string | null): CyberpunkCreationDraft {
  const trimmed = name?.trim() ?? '';
  return { ...draft, lifepath: { ...draft.lifepath, culturalLanguage: trimmed === '' ? null : trimmed } };
}

/** Changes the Role: fresh dice, no template row, and only the general Lifepath choices kept. */
export function selectRole(draft: CyberpunkCreationDraft, role: string | null, lifepathRecords: readonly CyberpunkRecord[]): CyberpunkCreationDraft {
  const general = new Set(lifepathRecords.filter((r) => r.group === 'general').map((r) => String(r.name)));
  const counts = Object.fromEntries(Object.entries(draft.lifepath.counts).filter(([table]) => general.has(table)));
  return {
    ...draft,
    role,
    seed: null,
    templateRoll: null,
    lifepath: {
      ...draft.lifepath,
      picks: draft.lifepath.picks.filter((p) => general.has(p.table)),
      counts,
      skips: draft.lifepath.skips.filter((name) => general.has(name)),
    },
  };
}

/** The Cultural Origin language's level tile: the engine's `cpr.skill_level.language.*` row, or none. */
export function culturalLanguageTile(preview: Pick<CyberpunkCreationPreview, 'culturalLanguage' | 'explanations'>): CyberpunkTile | null {
  const language = preview.culturalLanguage;
  if (language === null) {
    return null;
  }
  const found = preview.explanations.find((row) => row.id.startsWith('cpr.skill_level.language.'));
  return found ? { id: found.id, label: language.skill, value: found.value } : null;
}
