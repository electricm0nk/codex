import type { CyberpunkRecord } from '../../boundary/loadCyberpunkCatalog';
import type { CyberpunkDeckExtras, CyberpunkDeckRequest, CyberpunkDeckResponse } from '../../boundary/cprDeck';

/**
 * SD-38 E6.3 -- the Netrunning deck calculator's option lists and request. Holds no slot count and
 * no NET Action number: `compute_cyberpunk_deck` (`cpr_catalog.rs`, over the E2.9 engine) answers
 * both, and the screen prints its rows. The slot figure shown beside an option is the corpus
 * record's printed field, not a sum.
 */

/** Interface ranks a character can hold (p. 197: ranks 1 to 10); the engine refuses any other. */
export const INTERFACE_RANKS: readonly number[] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

export interface DeckOption {
  name: string;
  /** The printed slot cost, when the record prints one. */
  slots: number | null;
  /** Program class or hardware type, as printed. */
  detail: string;
}

export interface DeckOptions {
  decks: DeckOption[];
  programs: DeckOption[];
  hardware: DeckOption[];
}

export interface DeckSelection {
  deck: string;
  programs: string[];
  hardware: string[];
  extras: CyberpunkDeckExtras;
  interfaceRank: number;
}

function optionOf(record: CyberpunkRecord, detailField: string): DeckOption | null {
  if (typeof record.name !== 'string') {
    return null;
  }
  const detail = record[detailField];
  return {
    name: record.name,
    slots: typeof record.slots === 'number' ? record.slots : null,
    detail: typeof detail === 'string' ? detail : '',
  };
}

/**
 * The picker lists from the three catalogs. Hardware without a printed slot count (the Bodyweight
 * Suit and the Cyberarm option) are the deck's extras, not installable Hardware, so they are left
 * out of the Hardware list.
 */
export function buildDeckOptions(
  decks: readonly CyberpunkRecord[],
  programs: readonly CyberpunkRecord[],
  hardware: readonly CyberpunkRecord[],
): DeckOptions {
  const pick = (records: readonly CyberpunkRecord[], field: string) =>
    records.map((record) => optionOf(record, field)).filter((option): option is DeckOption => option !== null);
  return {
    decks: pick(decks, 'price_category'),
    programs: pick(programs, 'program_class'),
    hardware: pick(hardware, 'hardware_type').filter((option) => option.slots !== null),
  };
}

export function initialSelection(options: DeckOptions): DeckSelection {
  const standard = options.decks.find((deck) => deck.name.includes('Standard')) ?? options.decks[0];
  return {
    deck: standard ? standard.name : '',
    programs: [],
    hardware: [],
    extras: { bodyweightSuit: false, cyberarmDeck: false },
    interfaceRank: 1,
  };
}

export function deckRequest(selection: DeckSelection): CyberpunkDeckRequest {
  return {
    deck: selection.deck,
    programs: selection.programs,
    hardware: selection.hardware,
    extras: selection.extras,
    interfaceRank: selection.interfaceRank,
  };
}

/** The same list with the entry at `index` removed. */
export function withoutIndex(list: readonly string[], index: number): string[] {
  return list.filter((_, position) => position !== index);
}

/** The calculator's answer as the lines the screen prints, each a `cpr.netrunning.*` engine row. */
export function describeDeckLoad(response: CyberpunkDeckResponse): { id: string; value: number; detail: string }[] {
  const rows = [...response.rows];
  if (response.netActions !== null && !rows.some((row) => row.id === response.netActions?.id)) {
    rows.push(response.netActions);
  }
  return rows;
}
