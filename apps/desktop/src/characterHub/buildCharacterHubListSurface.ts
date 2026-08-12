import type { CharacterSummaryDto, ListSavedCharactersResponse } from '../boundary/loadListSavedCharacters';
import { GAME_SYSTEM_LABELS } from './characterHubModel';

export interface CharacterHubListRowSurface {
  characterId: string;
  displayLabel: string;
  gameSystemLabel: string;
  raceLabel: string;
  classSummary: string;
  savedAtLabel: string;
  /** Campaign name — not yet persisted by the backend, so absent for real characters. */
  campaign?: string;
}

export interface CharacterHubListSurface {
  rows: CharacterHubListRowSurface[];
  isEmpty: boolean;
  emptyStateMessage: string | null;
  unreadableNotice: string | null;
}

function formatSavedAt(savedAt: string): string {
  const parsed = new Date(savedAt);
  if (Number.isNaN(parsed.getTime())) {
    return savedAt;
  }
  return parsed.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
}

/** `race:half-orc` -> `Half-orc`. Good enough presentation without a full label catalogue. */
function formatRaceLabel(raceId: string): string {
  const withoutPrefix = raceId.replace(/^race:/, '');
  return withoutPrefix.charAt(0).toUpperCase() + withoutPrefix.slice(1);
}

/**
 * Maps one saved-character summary to its UI-ready row surface. Exported
 * (beyond this module's own list-building use) so a post-mutation refresh —
 * e.g. after `level_up_character` — can rebuild a single row from the fresh
 * summary a mutation response carries, without re-fetching the whole list.
 */
export function toRowSurface(summary: CharacterSummaryDto): CharacterHubListRowSurface {
  return {
    characterId: summary.characterId,
    displayLabel: summary.displayLabel,
    gameSystemLabel: GAME_SYSTEM_LABELS[summary.gameSystem] ?? summary.gameSystem,
    raceLabel: formatRaceLabel(summary.raceId),
    classSummary: summary.classSummary,
    savedAtLabel: formatSavedAt(summary.savedAt),
  };
}

/**
 * Replaces one row (by `characterId`) with a freshly rebuilt row, without a
 * full list re-fetch. Keeps the Load Character list's cached level/class
 * label in sync after a mutation on the currently open sheet (level-up,
 * etc.) — previously only the open sheet's own `row` state got this
 * treatment (see `CharacterHubPage.tsx`'s `onDetailRefreshed`), leaving the
 * list showing a stale label until the next full reload
 * (risks-and-open-questions.md item 26). A `characterId` not present in
 * `surface.rows` leaves it unchanged rather than fabricating a new row.
 */
export function replaceRowInSurface(
  surface: CharacterHubListSurface,
  updatedRow: CharacterHubListRowSurface
): CharacterHubListSurface {
  return {
    ...surface,
    rows: surface.rows.map((row) => (row.characterId === updatedRow.characterId ? updatedRow : row)),
  };
}

/** Maps the raw list snapshot to a UI-ready surface: newest-first, formatted dates/labels, empty/unreadable copy. */
export function buildCharacterHubListSurface(snapshot: ListSavedCharactersResponse): CharacterHubListSurface {
  const rows = [...snapshot.characters]
    .sort((a, b) => (a.savedAt < b.savedAt ? 1 : a.savedAt > b.savedAt ? -1 : 0))
    .map(toRowSurface);

  return {
    rows,
    isEmpty: rows.length === 0,
    emptyStateMessage: rows.length === 0 ? 'No characters yet — create your first one to get started.' : null,
    unreadableNotice:
      snapshot.unreadableCount > 0
        ? `${snapshot.unreadableCount} saved ${snapshot.unreadableCount === 1 ? 'character' : 'characters'} could not be read and ${
            snapshot.unreadableCount === 1 ? 'was' : 'were'
          } skipped.`
        : null,
  };
}
