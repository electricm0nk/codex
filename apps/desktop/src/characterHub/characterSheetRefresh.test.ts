import { isWizardSpellBootstrap, toCharacterMutationRefresh } from './characterSheetRefresh';
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
  verifiesWizardSpellBootstrapDetection();
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
      corpusDerived: {
        schoolCoverage: [],
        equippedItems: [],
        equipmentEffects: { perItem: [], armorClassDelta: 0, armorCheckPenaltyTotal: 0 },
        // Nothing equipped: a real empty loadout at the Strength-10
        // load.lst row (LOAD:10|100 -> 33 / 66 / 100), which is genuinely a
        // light load with no penalties. These tests assert refresh/surface
        // plumbing, not encumbrance values.
        encumbrance: {
          totalCarriedWeightLbs: 0,
          totalCarriedCostGp: 0,
          lightMaxLbs: 33,
          mediumMaxLbs: 66,
          heavyMaxLbs: 100,
          level: 'Light',
          loadArmorCheckPenalty: 0,
          perItem: [],
          unresolvedItemIds: [],
        },
        unresolvedSpellIds: [],
        unresolvedEquipmentItemIds: [],
      },
    },
    ['feat:power_attack'],
    [{ spellId: 'Light', sourceClassId: 'class:wizard', acquisitionMode: 'Known' }]
  );

  assertEqual(result.kind, 'refreshed', 'kind');
  if (result.kind !== 'refreshed') return;
  assertEqual(result.detail.summary.classSummary, 'class:fighter:2', 'refreshed detail carries the incremented classSummary');
  assertEqual(result.detail.snapshot?.baselineArmorClass, 17, 'refreshed detail carries the fresh snapshot');
  assertEqual(result.detail.diagnostics.length, 0, 'a Saved outcome carries no diagnostics');
  assertEqual(result.detail.selectedFeats.length, 1, 'refreshed detail carries the caller-supplied selectedFeats verbatim');
  assertEqual(result.detail.selectedFeats[0], 'feat:power_attack', 'refreshed detail carries the caller-supplied selectedFeats verbatim');
  assertEqual(result.detail.spellsSelected.length, 1, 'refreshed detail carries the caller-supplied spellsSelected verbatim');
  assertEqual(result.detail.spellsSelected[0].spellId, 'Light', 'refreshed detail carries the caller-supplied spellsSelected verbatim');
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
    [],
    []
  );

  assertEqual(result.kind, 'blocked', 'kind');
  if (result.kind !== 'blocked') return;
  assert(result.message.includes('Smite evil is not yet implemented.'), 'the real claim-blocking diagnostic should be surfaced verbatim');
  assert(!result.message.includes('Human race semantics are bounded.'), 'non-claim-blocking diagnostics should not appear in the message');
}

function verifiesBlockedOutcomeWithNoBlockingDiagnosticsFallsBackToHonestMessage() {
  const result = toCharacterMutationRefresh({ kind: 'Blocked', diagnostics: [] }, [], []);

  assertEqual(result.kind, 'blocked', 'kind');
  if (result.kind !== 'blocked') return;
  assert(result.message.length > 0, 'a blocked outcome always carries a non-empty user-facing message');
}

/**
 * Regression guard for risks-and-open-questions.md item 9a: once a Wizard
 * already has one recorded spell, later picks should use the cheaper plain
 * `addSpellSelection` path, not the atomic bootstrap command every pick
 * previously took unconditionally.
 */
function verifiesWizardSpellBootstrapDetection() {
  assert(
    isWizardSpellBootstrap([], 'class:wizard', 'class:wizard'),
    'a Wizard with zero recorded spells is a genuine bootstrap'
  );
  assert(
    !isWizardSpellBootstrap(
      [{ spellId: 'Light', sourceClassId: 'class:wizard', acquisitionMode: 'Known' }],
      'class:wizard',
      'class:wizard'
    ),
    'a Wizard that already has one recorded spell is not a bootstrap'
  );
  assert(
    !isWizardSpellBootstrap(
      [{ spellId: 'feat:whatever', sourceClassId: 'class:rogue', acquisitionMode: 'Known' }],
      'class:rogue',
      'class:wizard'
    ),
    'a non-Wizard class pick is never a bootstrap'
  );
  assert(
    isWizardSpellBootstrap(
      [{ spellId: 'Cure Light Wounds', sourceClassId: 'class:cleric', acquisitionMode: 'Known' }],
      'class:wizard',
      'class:wizard'
    ),
    'existing spells from a different class do not count toward this Wizard bootstrap check'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
