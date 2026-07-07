import { buildCharacterHubListSurface } from './buildCharacterHubListSurface';
import { makeCharacterSummary } from '../testSupport/makeCharacterSummary';
import { assert, assertEqual } from '../testSupport/asserts';

async function main() {
  verifiesEmptyState();
  verifiesPopulatedStateSortedNewestFirst();
  verifiesUnreadableNotice();
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

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
