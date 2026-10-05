import {
  buildListSavedCharactersArgs,
  buildRecomputeCharacterRequest,
  characterCreationGate,
  resolveRuleSystemId,
} from './characterHubRuntime';
import { RULE_SETS } from './LandingScreen';
import { toRowSurface } from './buildCharacterHubListSurface';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-25 Criterion 3.5 RED (`cycles/3_5.md`): before `resolveRuleSystemId` /
 * `buildRecomputeCharacterRequest` existed in this module, the panel had no
 * concept of "the active rule-system adapter" at all — every mutation call
 * site was hard-wired to whatever the Rust command defaulted to, and the
 * `RuleSetId` the landing screen already lets the operator pick
 * (`LandingScreen.tsx`) never flowed anywhere past that screen's own local
 * state. GREEN (this file, present tense): `resolveRuleSystemId` maps the
 * UI's `RuleSetId` to the wire-level `ruleSystemId` the Rust
 * `resolve_rule_system_adapter` dispatch seam understands (`cycles/3_4.md`),
 * and `buildRecomputeCharacterRequest` proves a real call site threads that
 * resolved id through rather than hardcoding `"pf1"`.
 */

function testResolveRuleSystemIdMapsPathfinderToTheRealAdapterId() {
  assertEqual(
    resolveRuleSystemId('pathfinder-1e'),
    'pf1',
    'pathfinder-1e is the only rule set with a real adapter today, so it resolves to "pf1"'
  );
}

function testResolveRuleSystemIdPassesThroughUnimplementedRuleSetsHonestly() {
  // Every other RuleSetId must NOT be silently rewritten to "pf1" — that
  // would make an unimplemented rule set masquerade as PF1 behavior server
  // side. Passing the id through unchanged means it honestly routes to
  // `StubAdapter` (the shared `resolve_rule_system_adapter`'s `other` arm).
  assertEqual(
    resolveRuleSystemId('traveller'),
    'traveller',
    'an unimplemented rule set must pass through unchanged, not borrow pf1 silently'
  );
}

function testBuildRecomputeCharacterRequestRoutesPf1ThroughTheRealAdapter() {
  const request = buildRecomputeCharacterRequest('char-1', 'pathfinder-1e');
  assertEqual(request.characterId, 'char-1', 'characterId is carried through verbatim');
  assertEqual(request.ruleSystemId, 'pf1', 'the active adapter for pathfinder-1e resolves to pf1');
}

function testBuildRecomputeCharacterRequestRoutesOtherRuleSetsToStubAdapterHonestly() {
  const request = buildRecomputeCharacterRequest('char-2', 'cyberpunk');
  assertEqual(request.characterId, 'char-2', 'characterId is carried through verbatim');
  assertEqual(
    request.ruleSystemId,
    'cyberpunk',
    'an unimplemented rule set is not silently rewritten to pf1 in the outgoing request'
  );
}

/**
 * SD-37 E6.1: the landing's Starfinder 1e chip is selectable, and selecting it
 * routes to the Rust `StarfinderAdapter`. `resolve_rule_system_adapter`
 * (`apps/desktop/src-tauri/src/rule_system_adapter.rs`) maps exactly the wire id
 * `"starfinder-1e"` (`sf_adapter::STARFINDER_RULE_SYSTEM_ID`) to that adapter.
 */
function testStarfinderIsSelectableOnTheLandingAndRoutesToTheStarfinderAdapter() {
  const starfinder = RULE_SETS.find((ruleSet) => ruleSet.id === 'starfinder-1e');
  assert(starfinder !== undefined, 'the landing lists Starfinder 1e');
  assertEqual(starfinder?.available, true, 'the Starfinder 1e chip is selectable');
  assertEqual(
    resolveRuleSystemId('starfinder-1e'),
    'starfinder-1e',
    'starfinder-1e resolves to the wire id the Rust resolver maps to StarfinderAdapter'
  );
  const request = buildRecomputeCharacterRequest('sf-char', 'starfinder-1e');
  assertEqual(request.ruleSystemId, 'starfinder-1e', 'a Starfinder sheet recompute routes to StarfinderAdapter');
  // Only Pathfinder and Starfinder have adapters; the other chips stay unselectable.
  const selectable = RULE_SETS.filter((ruleSet) => ruleSet.available).map((ruleSet) => ruleSet.id);
  assertEqual(selectable.join(','), 'pathfinder-1e,starfinder-1e', 'exactly the two systems with an adapter are selectable');
}

/** The Load list asks the active system's adapter for its characters. */
function testTheLoadListRoutesThroughTheActiveRuleSystem() {
  assertEqual(
    buildListSavedCharactersArgs('starfinder-1e').ruleSystemId,
    'starfinder-1e',
    'with Starfinder selected the list comes from StarfinderAdapter'
  );
  assertEqual(
    buildListSavedCharactersArgs('pathfinder-1e').ruleSystemId,
    'pf1',
    'with Pathfinder selected the list comes from Pf1Adapter'
  );
}

/** A saved Starfinder character's row names its system. */
function testAStarfinderRowNamesItsSystem() {
  const row = toRowSurface({
    characterId: 'sf-char',
    displayLabel: 'SF',
    gameSystem: 'starfinder-1e',
    schemaVersion: 2,
    savedAt: '2026-10-05T00:00:00Z',
    raceId: 'core:race:human',
    classSummary: 'core:class:soldier:3',
  });
  assertEqual(row.gameSystemLabel, 'Starfinder 1st Edition', 'the row label names Starfinder');
}

/**
 * Pathfinder creation stays open. Starfinder has no creation flow on this
 * screen yet, so "New Character" is disabled with its reason rather than
 * opening the Pathfinder form for a Starfinder selection.
 */
function testCreationGateNeverOpensThePathfinderFormForStarfinder() {
  assertEqual(characterCreationGate('pathfinder-1e').enabled, true, 'Pathfinder creation is open');
  const gate = characterCreationGate('starfinder-1e');
  assertEqual(gate.enabled, false, 'the Pathfinder creation form never opens for Starfinder');
  assert((gate.disabledHint ?? '').includes('Starfinder'), 'the disabled banner names why');
}

async function main() {
  testStarfinderIsSelectableOnTheLandingAndRoutesToTheStarfinderAdapter();
  testTheLoadListRoutesThroughTheActiveRuleSystem();
  testAStarfinderRowNamesItsSystem();
  testCreationGateNeverOpensThePathfinderFormForStarfinder();
  testResolveRuleSystemIdMapsPathfinderToTheRealAdapterId();
  testResolveRuleSystemIdPassesThroughUnimplementedRuleSetsHonestly();
  testBuildRecomputeCharacterRequestRoutesPf1ThroughTheRealAdapter();
  testBuildRecomputeCharacterRequestRoutesOtherRuleSetsToStubAdapterHonestly();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
