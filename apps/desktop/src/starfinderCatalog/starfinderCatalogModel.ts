import type { StarfinderCatalogEntryDto, StarfinderCatalogKind } from '../boundary/loadStarfinderCatalog';

/**
 * SD-37 E6.4 -- the Starfinder catalogs' labels, order and filtering. Holds
 * no record and no number: every row is the engine's
 * (`list_starfinder_catalog`, `sf_catalog.rs`).
 */

export interface StarfinderCatalogTab {
  kind: StarfinderCatalogKind;
  /** The landing screen's link text. */
  link: string;
  /** The catalog screen's heading. */
  title: string;
}

export const STARFINDER_CATALOG_TABS: readonly StarfinderCatalogTab[] = [
  { kind: 'race', link: 'Browse Races', title: 'Races' },
  { kind: 'theme', link: 'Browse Themes', title: 'Themes' },
  { kind: 'class', link: 'Browse Classes', title: 'Classes' },
  { kind: 'feat', link: 'Browse Feats', title: 'Feats' },
  { kind: 'spell', link: 'Browse Spells', title: 'Spells' },
  { kind: 'equipment', link: 'Browse Equipment', title: 'Equipment' },
];

export function starfinderCatalogTab(kind: StarfinderCatalogKind): StarfinderCatalogTab {
  return STARFINDER_CATALOG_TABS.find((tab) => tab.kind === kind) ?? STARFINDER_CATALOG_TABS[0];
}

/** A book id as words: `alien_archive_2` -> `Alien Archive 2`. */
export function bookLabel(book: string): string {
  return book
    .split('_')
    .filter((word) => word.length > 0)
    .map((word) => word[0].toUpperCase() + word.slice(1))
    .join(' ');
}

/** The rows whose name, book, tags or description contain every word of `query` (case-insensitive). */
export function filterStarfinderCatalog(
  entries: readonly StarfinderCatalogEntryDto[],
  query: string
): StarfinderCatalogEntryDto[] {
  const words = query.toLowerCase().split(/\s+/).filter((word) => word.length > 0);
  if (words.length === 0) {
    return [...entries];
  }
  return entries.filter((entry) => {
    const haystack = [entry.name, bookLabel(entry.book), ...entry.tags, entry.description ?? '']
      .join('\n')
      .toLowerCase();
    return words.every((word) => haystack.includes(word));
  });
}

/** The screen's one-line summary of what it shows, from the rows that arrived. */
export function describeStarfinderCatalog(
  kind: StarfinderCatalogKind,
  entries: readonly StarfinderCatalogEntryDto[],
  shown: number
): string {
  const books = new Set(entries.map((entry) => entry.book)).size;
  const title = starfinderCatalogTab(kind).title.toLowerCase();
  const bookWord = books === 1 ? 'book' : 'books';
  const all = `${entries.length} ${title} from ${books} ${bookWord}`;
  return shown === entries.length ? all : `${shown} of ${all}`;
}
