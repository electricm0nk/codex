import type { AbilityOptionDto } from '../boundary/abilities';

/**
 * Pure model behind the generic Abilities sub-tab (SD-43 E3.3). Every number is an engine row
 * (`abilities.budget.<subtab>.<pool>.{granted,spent,remaining}`) copied as served: nothing here
 * adds, subtracts or counts (NR-1). Qualification and its reasons are the engine option's own.
 */
export interface AbilityPoolLine {
  id: string;
  label: string;
  granted: number;
  spent: number;
  remaining: number;
  /** The granted row's derivation text, verbatim. */
  detail: string;
}

export interface AbilityAvailableRow {
  id: string;
  label: string;
  pool: string;
  category: string | null;
  qualified: boolean;
  unmet: string[];
  /** The engine's rule text for the record, when it carries one. */
  description: string | null;
  /** Why Add is disabled, in the engine's words; `null` when Add is live. */
  addBlockedBy: string | null;
}

export interface AbilitySelectedRow {
  id: string;
  label: string;
  pool: string | null;
  category: string | null;
}

export interface AbilitySubTabModel {
  pools: AbilityPoolLine[];
  available: AbilityAvailableRow[];
  selected: AbilitySelectedRow[];
}

const FIELDS = ['granted', 'spent', 'remaining'] as const;

function capitalised(id: string): string {
  const words = id.replace(/_/g, ' ');
  return words.charAt(0).toUpperCase() + words.slice(1);
}

/** One pool's budget line as printed: `Traits: 2 of 2 spent, 0 remaining`. */
export function formatPoolLine(pool: AbilityPoolLine): string {
  return `${pool.label}: ${pool.spent} of ${pool.granted} spent, ${pool.remaining} remaining`;
}

export function buildAbilitySubTabModel(input: {
  subtab: string;
  budget: ReadonlyArray<{ id: string; value: number; detail?: string }>;
  options: readonly AbilityOptionDto[];
  selectedIds: readonly string[];
  search: string;
  qualifiedOnly: boolean;
}): AbilitySubTabModel {
  const prefix = `abilities.budget.${input.subtab}.`;
  const pools = new Map<string, AbilityPoolLine>();
  for (const row of input.budget) {
    if (!row.id.startsWith(prefix)) continue;
    const rest = row.id.slice(prefix.length);
    const dot = rest.lastIndexOf('.');
    const field = rest.slice(dot + 1) as (typeof FIELDS)[number];
    if (dot < 0 || !FIELDS.includes(field)) continue;
    const id = rest.slice(0, dot);
    const pool = pools.get(id) ?? { id, label: capitalised(id), granted: 0, spent: 0, remaining: 0, detail: '' };
    pool[field] = row.value;
    if (field === 'granted') pool.detail = row.detail ?? '';
    pools.set(id, pool);
  }

  const held = new Set(input.selectedIds);
  const query = input.search.trim().toLowerCase();
  const available = input.options
    .filter((option) => !held.has(option.id))
    .filter((option) => !input.qualifiedOnly || option.qualified)
    .filter((option) => query === '' || option.label.toLowerCase().includes(query))
    .map((option): AbilityAvailableRow => {
      const pool = pools.get(option.pool);
      const addBlockedBy = !option.qualified
        ? option.unmet.join('; ') || 'The engine marks this option unqualified.'
        : pool !== undefined && pool.remaining <= 0
          ? `No room in the ${option.pool} pool: ${pool.spent} of ${pool.granted} spent, ${pool.remaining} remaining.`
          : null;
      return { id: option.id, label: option.label, pool: option.pool, category: option.category, qualified: option.qualified, unmet: option.unmet, description: option.description ?? null, addBlockedBy };
    });

  const selected = input.selectedIds.map((id): AbilitySelectedRow => {
    const option = input.options.find((candidate) => candidate.id === id);
    return { id, label: option?.label ?? id, pool: option?.pool ?? null, category: option?.category ?? null };
  });

  return { pools: [...pools.values()], available, selected };
}
