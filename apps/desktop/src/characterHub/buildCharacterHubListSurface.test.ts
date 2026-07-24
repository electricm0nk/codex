import { buildCharacterHubListSurface, replaceRowInSurface, toRowSurface } from './buildCharacterHubListSurface';
import { makeCharacterSummary } from '../testSupport/makeCharacterSummary';
import { assert, assertEqual } from '../testSupport/asserts';

async function main() {
  verifiesEmptyState();
  verifiesPopulatedStateSortedNewestFirst();
  verifiesUnreadableNotice();
  verifiesReplaceRowInSurfaceUpdatesTheMatchingRowOnly();
  verifiesReplaceRowInSurfaceLeavesAnUnknownCharacterIdUnchanged();
}

function verifiesEmptyState() {
  const surface = buildCharacterHubListSurface({ characters: [], unreadableCount: 0 });

  assert(surface.isEmpty, 'surface should be empty with no characters');
  assertEqual(surface.rows.length, 0, 'rows length');
  assert(surface.emptyStateMessage !== null, 'empty state message should be present');
  assertEqual(surface.unreadableNotice, null, 'unreadable notice');
}

function verifiesPopulatedStateSortedNewestFirst() {
  const older = makeCharacterSummary({
    characterId: 'char-older',
    displayLabel: 'Older Character',
    savedAt: '2026-01-01T00:00:00Z',
  });
  const newer = makeCharacterSummary({
    characterId: 'char-newer',
    displayLabel: 'Newer Character',
    savedAt: '2026-07-01T00:00:00Z',
    gameSystem: 'pf1',
    raceId: 'race:half-orc',
  });

  const surface = buildCharacterHubListSurface({ characters: [older, newer], unreadableCount: 0 });

  assert(!surface.isEmpty, 'surface should not be empty');
  assertEqual(surface.rows.length, 2, 'rows length');
  assertEqual(surface.rows[0].characterId, 'char-newer', 'newest character should sort first');
  assertEqual(surface.rows[1].characterId, 'char-older', 'oldest character should sort last');
  assertEqual(surface.rows[0].gameSystemLabel, 'Pathfinder 1st Edition', 'game system label lookup');
  assertEqual(surface.rows[0].raceLabel, 'Half-orc', 'race label formatting');
  assertEqual(surface.emptyStateMessage, null, 'empty state message should be absent when populated');
}

function verifiesUnreadableNotice() {
  const surface = buildCharacterHubListSurface({
    characters: [makeCharacterSummary()],
    unreadableCount: 2,
  });

  assert(surface.unreadableNotice !== null, 'unreadable notice should be present');
  assert(surface.unreadableNotice!.includes('2'), 'unreadable notice should mention the count');
}

/**
 * risks-and-open-questions.md item 26: after a level-up on the open sheet,
 * the Load Character list's cached row for that character used to only
 * refresh on a full reload — this is the fix, patching the one matching row
 * in place instead.
 */
function verifiesReplaceRowInSurfaceUpdatesTheMatchingRowOnly() {
  const other = makeCharacterSummary({ characterId: 'char-other', classSummary: 'class:rogue:2' });
  const stale = makeCharacterSummary({ characterId: 'char-leveled', classSummary: 'class:fighter:1' });
  const surface = buildCharacterHubListSurface({ characters: [other, stale], unreadableCount: 0 });

  const freshRow = toRowSurface(makeCharacterSummary({ characterId: 'char-leveled', classSummary: 'class:fighter:2' }));
  const updated = replaceRowInSurface(surface, freshRow);

  const updatedRow = updated.rows.find((row) => row.characterId === 'char-leveled');
  assertEqual(updatedRow?.classSummary, 'class:fighter:2', 'the matching row picks up the fresh class summary');
  const untouchedRow = updated.rows.find((row) => row.characterId === 'char-other');
  assertEqual(untouchedRow?.classSummary, 'class:rogue:2', 'every other row is left exactly as it was');
}

function verifiesReplaceRowInSurfaceLeavesAnUnknownCharacterIdUnchanged() {
  const surface = buildCharacterHubListSurface({ characters: [makeCharacterSummary({ characterId: 'char-a' })], unreadableCount: 0 });
  const rowForAMissingCharacter = toRowSurface(makeCharacterSummary({ characterId: 'char-not-in-list' }));

  const updated = replaceRowInSurface(surface, rowForAMissingCharacter);

  assertEqual(updated.rows.length, 1, 'a characterId absent from the surface does not get appended as a new row');
  assertEqual(updated.rows[0].characterId, 'char-a', 'the existing row is untouched');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
