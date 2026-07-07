import type { CharacterSummaryDto, ListSavedCharactersResponse } from '../boundary/loadListSavedCharacters';
import { GAME_SYSTEM_LABELS } from './characterHubModel';

export interface CharacterHubListRowSurface {
  characterId: string;
  displayLabel: string;
  gameSystemLabel: string;
  raceLabel: string;
  classSummary: string;
  savedAtLabel: string;
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

function toRowSurface(summary: CharacterSummaryDto): CharacterHubListRowSurface {
  return {
    characterId: summary.characterId,
    displayLabel: summary.displayLabel,
    gameSystemLabel: GAME_SYSTEM_LABELS[summary.gameSystem] ?? summary.gameSystem,
    raceLabel: formatRaceLabel(summary.raceId),
    classSummary: summary.classSummary,
    savedAtLabel: formatSavedAt(summary.savedAt),
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
