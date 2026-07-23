import { toCharacterMutationRefresh } from './characterSheetRefresh';
import { makeCharacterSummary } from '../testSupport/makeCharacterSummary';
import { assert, assertEqual } from '../testSupport/asserts';

const SNAPSHOT = {
  abilityModifiers: { strength: 3, dexterity: 2, constitution: 2, intelligence: 0, wisdom: 1, charisma: -1 },
  baseAttackBonus: 2,
  baseSaves: { fortitude: 3, reflex: 0, will: 0 },
  baselineMeleeAttackBonus: 5,
  baselineArmorClass: 17,
  totalSaves: { fortitude: 5, reflex: 2, will: 0 },
  selectedSkillModifiers: { climb: 5, intimidate: -1, swim: 5 },
};

async function main() {
  verifiesSavedOutcomeRefreshesDetailWithNewLevel();
  verifiesBlockedOutcomeSurfacesRealDiagnosticVerbatim();
  verifiesBlockedOutcomeWithNoBlockingDiagnosticsFallsBackToHonestMessage();
}

/**
 * The whole point of the refresh path: after a level-up mutation succeeds,
 * the returned `classSummary` (which drives the Level box, class panel, and
 * Progression rail — all derived from `row.classSummary`, not `detail`)
 * reflects the incremented level.
 */
function verifiesSavedOutcomeRefreshesDetailWithNewLevel() {
  const summary = makeCharacterSummary({ classSummary: 'class:fighter:2' });
  const result = toCharacterMutationRefresh(
    {
      kind: 'Saved',
      summary,
      snapshot: SNAPSHOT,
      corpusDerived: { schoolCoverage: [], equippedItems: [] },
    },
    ['feat:power_attack']
  );

  assertEqual(result.kind, 'refreshed', 'kind');
  if (result.kind !== 'refreshed') return;
  assertEqual(result.detail.summary.classSummary, 'class:fighter:2', 'refreshed detail carries the incremented classSummary');
  assertEqual(result.detail.snapshot?.baselineArmorClass, 17, 'refreshed detail carries the fresh snapshot');
  assertEqual(result.detail.diagnostics.length, 0, 'a Saved outcome carries no diagnostics');
  assertEqual(result.detail.selectedFeats.length, 1, 'refreshed detail carries the caller-supplied selectedFeats verbatim');
  assertEqual(result.detail.selectedFeats[0], 'feat:power_attack', 'refreshed detail carries the caller-supplied selectedFeats verbatim');
}

function verifiesBlockedOutcomeSurfacesRealDiagnosticVerbatim() {
  const result = toCharacterMutationRefresh(
    {
      kind: 'Blocked',
      diagnostics: [
        { id: 'class_feature.paladin.smite_evil.unsupported', message: 'Smite evil is not yet implemented.', claimBlocking: true },
        { id: 'race.human.bounded_semantics', message: 'Human race semantics are bounded.', claimBlocking: false },
      ],
    },
    []
  );

  assertEqual(result.kind, 'blocked', 'kind');
  if (result.kind !== 'blocked') return;
  assert(result.message.includes('Smite evil is not yet implemented.'), 'the real claim-blocking diagnostic should be surfaced verbatim');
  assert(!result.message.includes('Human race semantics are bounded.'), 'non-claim-blocking diagnostics should not appear in the message');
}

function verifiesBlockedOutcomeWithNoBlockingDiagnosticsFallsBackToHonestMessage() {
  const result = toCharacterMutationRefresh({ kind: 'Blocked', diagnostics: [] }, []);

  assertEqual(result.kind, 'blocked', 'kind');
  if (result.kind !== 'blocked') return;
  assert(result.message.length > 0, 'a blocked outcome always carries a non-empty user-facing message');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
