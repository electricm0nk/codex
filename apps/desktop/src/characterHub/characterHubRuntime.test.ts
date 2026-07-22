import { buildRecomputeCharacterRequest, resolveRuleSystemId } from './characterHubRuntime';
import { assertEqual } from '../testSupport/asserts';

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
  // `StubAdapter` (per `resolve_rule_system_adapter`'s `other => StubAdapter`
  // arm in every SD-25 3.4 command module).
  assertEqual(
    resolveRuleSystemId('starfinder-1e'),
    'starfinder-1e',
    'an unimplemented rule set must pass through unchanged, not borrow pf1 silently'
  );
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

async function main() {
  testResolveRuleSystemIdMapsPathfinderToTheRealAdapterId();
  testResolveRuleSystemIdPassesThroughUnimplementedRuleSetsHonestly();
  testBuildRecomputeCharacterRequestRoutesPf1ThroughTheRealAdapter();
  testBuildRecomputeCharacterRequestRoutesOtherRuleSetsToStubAdapterHonestly();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
