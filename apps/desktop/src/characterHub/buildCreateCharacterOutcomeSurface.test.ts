import { buildCreateCharacterOutcomeSurface } from './buildCreateCharacterOutcomeSurface';
import { makeCharacterSummary } from '../testSupport/makeCharacterSummary';
import { assert, assertEqual } from '../testSupport/asserts';

const FIGHTER_CONTEXT = { raceId: 'race:human', classId: 'class:fighter', classLabel: 'Fighter', supportLevel: 'full' as const };
const PALADIN_HUMAN_CONTEXT = {
  raceId: 'race:human',
  classId: 'class:paladin',
  classLabel: 'Paladin',
  supportLevel: 'partial-human-only' as const,
};
const PALADIN_DWARF_CONTEXT = {
  raceId: 'race:dwarf',
  classId: 'class:paladin',
  classLabel: 'Paladin',
  supportLevel: 'partial-human-only' as const,
};
const ROGUE_CONTEXT = { raceId: 'race:human', classId: 'class:rogue', classLabel: 'Rogue', supportLevel: 'none' as const };

async function main() {
  verifiesSavedOutcome();
  verifiesBlockedOutcomeGroupsNamedDiagnostics();
  verifiesBlockedOutcomeForUnsupportedClassShowsHonestSentence();
  verifiesBlockedOutcomeForWrongRaceShowsHumanOnlySentence();
  verifiesClaimBlockingFilterExcludesNonBlockingDiagnostics();
}

function verifiesSavedOutcome() {
  const surface = buildCreateCharacterOutcomeSurface(
    {
      kind: 'Saved',
      summary: makeCharacterSummary({ displayLabel: 'Aldric' }),
      snapshot: {
        abilityModifiers: { strength: 3, dexterity: 2, constitution: 2, intelligence: 0, wisdom: 1, charisma: -1 },
        baseAttackBonus: 1,
        baseSaves: { fortitude: 2, reflex: 0, will: 0 },
        baselineMeleeAttackBonus: 4,
        baselineArmorClass: 17,
        totalSaves: { fortitude: 4, reflex: 2, will: 0 },
        selectedSkillModifiers: { climb: 5, intimidate: -1, swim: 5 },
      },
      corpusDerived: { schoolCoverage: [], equippedItems: [] },
    },
    FIGHTER_CONTEXT
  );

  assertEqual(surface.kind, 'saved', 'kind');
  assert(surface.headline.includes('Aldric'), 'headline should name the character');
  assertEqual(surface.diagnosticGroups.length, 0, 'saved outcome should carry no diagnostic groups');
  assertEqual(surface.rawDiagnostics.length, 0, 'saved outcome should carry no raw diagnostics');
  assert(surface.highlights.length > 0, 'saved outcome should carry highlights');
  const ac = surface.highlights.find((h) => h.label === 'Armor Class');
  assert(ac !== undefined && ac.value === '17', 'armor class highlight should be present and correct');
}

function verifiesBlockedOutcomeGroupsNamedDiagnostics() {
  const surface = buildCreateCharacterOutcomeSurface(
    {
      kind: 'Blocked',
      diagnostics: [
        { id: 'class_feature.paladin.smite_evil.unsupported', message: 'Smite evil is not yet implemented.', claimBlocking: true },
        { id: 'class_spell.paladin.partial_caster.unsupported', message: 'Partial-caster spell slots are not yet computed.', claimBlocking: true },
        { id: 'class_chassis.unsupported', message: 'The base chassis is not yet computed.', claimBlocking: true },
      ],
    },
    PALADIN_HUMAN_CONTEXT
  );

  assertEqual(surface.kind, 'blocked', 'kind');
  assertEqual(surface.highlights.length, 0, 'blocked outcome should carry no highlights');
  assertEqual(surface.rawDiagnostics.length, 3, 'rawDiagnostics should carry every diagnostic verbatim, unfiltered');
  assertEqual(surface.diagnosticGroups.length, 2, 'named diagnostics should be grouped by class_feature/class_spell prefix');

  const featureGroup = surface.diagnosticGroups.find((g) => g.label === 'Class features not yet computed');
  assert(featureGroup !== undefined, 'a class_feature group should exist');
  assert(
    featureGroup!.messages.includes('Smite evil is not yet implemented.'),
    'the class_feature group should show the real diagnostic message verbatim'
  );

  const spellGroup = surface.diagnosticGroups.find((g) => g.label === 'Spellcasting not yet computed');
  assert(spellGroup !== undefined, 'a class_spell group should exist');
  assert(
    spellGroup!.messages.includes('Partial-caster spell slots are not yet computed.'),
    'the class_spell group should show the real diagnostic message verbatim'
  );

  const genericAppearsInAGroup = surface.diagnosticGroups.some((g) =>
    g.messages.includes('The base chassis is not yet computed.')
  );
  assert(!genericAppearsInAGroup, 'the generic class_chassis diagnostic should be suppressed once named diagnostics exist');
}

function verifiesBlockedOutcomeForUnsupportedClassShowsHonestSentence() {
  const surface = buildCreateCharacterOutcomeSurface(
    {
      kind: 'Blocked',
      diagnostics: [
        { id: 'class_chassis.unsupported', message: 'The base chassis is not yet computed.', claimBlocking: true },
        { id: 'combat.baseline_unsupported', message: 'Baseline combat stats are not yet computed.', claimBlocking: true },
        { id: 'defense.total_save.unsupported', message: 'Total saves are not yet computed.', claimBlocking: true },
        { id: 'skill.selected_modifier.unsupported', message: 'Selected skill modifiers are not yet computed.', claimBlocking: true },
      ],
    },
    ROGUE_CONTEXT
  );

  assertEqual(surface.diagnosticGroups.length, 1, 'generic-only outcome should collapse to one honest group');
  assertEqual(surface.diagnosticGroups[0].messages.length, 1, 'generic-only outcome should show exactly one sentence');
  assert(
    surface.diagnosticGroups[0].messages[0].includes('Rogue'),
    'the honest sentence should name the class'
  );
  assert(
    surface.diagnosticGroups[0].messages[0].toLowerCase().includes("isn't computed"),
    'a supportLevel none class should say it is not computed at all'
  );
}

function verifiesBlockedOutcomeForWrongRaceShowsHumanOnlySentence() {
  const surface = buildCreateCharacterOutcomeSurface(
    {
      kind: 'Blocked',
      diagnostics: [
        { id: 'class_chassis.unsupported', message: 'The base chassis is not yet computed.', claimBlocking: true },
        { id: 'combat.baseline_unsupported', message: 'Baseline combat stats are not yet computed.', claimBlocking: true },
        { id: 'defense.total_save.unsupported', message: 'Total saves are not yet computed.', claimBlocking: true },
        { id: 'skill.selected_modifier.unsupported', message: 'Selected skill modifiers are not yet computed.', claimBlocking: true },
      ],
    },
    PALADIN_DWARF_CONTEXT
  );

  assertEqual(surface.diagnosticGroups.length, 1, 'generic-only outcome should collapse to one honest group');
  assert(
    surface.diagnosticGroups[0].messages[0].toLowerCase().includes('human'),
    'a partial-human-only class picked with a non-Human race should explicitly say Human is required'
  );
}

function verifiesClaimBlockingFilterExcludesNonBlockingDiagnostics() {
  const surface = buildCreateCharacterOutcomeSurface(
    {
      kind: 'Blocked',
      diagnostics: [
        { id: 'class_feature.paladin.smite_evil.unsupported', message: 'Smite evil is not yet implemented.', claimBlocking: true },
        { id: 'race.human.bounded_semantics', message: 'Human race semantics are bounded.', claimBlocking: false },
      ],
    },
    PALADIN_HUMAN_CONTEXT
  );

  assertEqual(surface.rawDiagnostics.length, 2, 'rawDiagnostics should still carry the non-blocking entry verbatim');
  const allGroupedMessages = surface.diagnosticGroups.flatMap((g) => g.messages);
  assert(
    !allGroupedMessages.includes('Human race semantics are bounded.'),
    'a non-claimBlocking diagnostic must never appear in a diagnosticGroup'
  );
  assert(
    allGroupedMessages.includes('Smite evil is not yet implemented.'),
    'the real claim-blocking diagnostic should still be grouped'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
