/**
 * The class catalog every class lookup in the Character Hub reads: which classes the Create
 * picker offers, and the label / hit die / skill ranks of every class a character can hold.
 *
 * SD-36 Epic F4c. The catalog is installed by `classRoster.ts` from the engine's
 * `list_class_creation_roster` command. Until that command answers, the catalog is `loading`:
 * nothing is offered and no lookup falls back to a compiled-in list. When the command fails,
 * `classRoster.ts` installs `characterHubModel.CLASS_OPTIONS_FALLBACK` WITH a notice naming the
 * failure; every surface that offers classes prints that notice (never a silent stub).
 *
 * This module holds state only and imports no values, so `characterHubModel.ts` (lookups) and
 * `classRoster.ts` (installation) can both depend on it without an import cycle.
 */

import type { ClassOption } from './characterHubModel';

/** What the hub knows about one class id, whether or not it is offered at creation. */
export interface KnownClass {
  id: string;
  label: string;
  /** `null` when no converted record states it: HP then prints Unknown, never an assumed die. */
  hitDie: number | null;
  /** `null` when not stated (or in the fallback): skill points then print Unknown. */
  skillRanksPerLevel: number | null;
}

export type ClassCatalogSource = 'loading' | 'roster' | 'fallback';

export interface ClassCatalogState {
  source: ClassCatalogSource;
  /** The Create picker's options, in served order (grouped by family for a roster). */
  options: readonly ClassOption[];
  /** Every class id the hub can label: offered and withheld (prestige, Ex-*) alike. */
  known: ReadonlyMap<string, KnownClass>;
  /** `class roster unavailable: <diagnostic>` for the fallback; `null` otherwise. */
  notice: string | null;
}

export const LOADING_CLASS_CATALOG: ClassCatalogState = Object.freeze({
  source: 'loading',
  options: Object.freeze([]) as readonly ClassOption[],
  known: new Map<string, KnownClass>(),
  notice: null,
}) as ClassCatalogState;

let current: ClassCatalogState = LOADING_CLASS_CATALOG;
const listeners = new Set<() => void>();

export function getClassCatalog(): ClassCatalogState {
  return current;
}

export function setClassCatalog(next: ClassCatalogState): void {
  current = next;
  for (const listener of listeners) {
    listener();
  }
}

export function subscribeClassCatalog(listener: () => void): () => void {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
}

/** The Create-picker option for `classId`, or `undefined` when the catalog does not offer it. */
export function findClassOption(classId: string): ClassOption | undefined {
  return current.options.find((option) => option.id === classId);
}

/** What the catalog knows about `classId` (offered or withheld), or `undefined`. */
export function knownClass(classId: string): KnownClass | undefined {
  return current.known.get(classId);
}
