import { buildCreateCharacterOutcomeSurface } from './buildCreateCharacterOutcomeSurface';
import { makeCharacterSummary } from '../testSupport/makeCharacterSummary';
import { assert, assertEqual } from '../testSupport/asserts';

async function main() {
  verifiesSavedOutcome();
  verifiesBlockedOutcomeShowsRealDiagnostics();
}

function verifiesSavedOutcome() {
  const surface = buildCreateCharacterOutcomeSurface({
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
  });

  assertEqual(surface.kind, 'saved', 'kind');
  assert(surface.headline.includes('Aldric'), 'headline should name the character');
  assertEqual(surface.diagnosticMessages.length, 0, 'saved outcome should carry no diagnostics');
  assert(surface.highlights.length > 0, 'saved outcome should carry highlights');
  const ac = surface.highlights.find((h) => h.label === 'Armor Class');
  assert(ac !== undefined && ac.value === '17', 'armor class highlight should be present and correct');
}

function verifiesBlockedOutcomeShowsRealDiagnostics() {
  const surface = buildCreateCharacterOutcomeSurface({
    kind: 'Blocked',
    diagnostics: [
      { id: 'class_feature.paladin.smite_evil.unsupported', message: 'Smite evil is not yet implemented.', claimBlocking: true },
      { id: 'class_spell.paladin.partial_caster.unsupported', message: 'Partial-caster spell slots are not yet computed.', claimBlocking: true },
    ],
  });

  assertEqual(surface.kind, 'blocked', 'kind');
  assertEqual(surface.highlights.length, 0, 'blocked outcome should carry no highlights');
  assertEqual(surface.diagnosticMessages.length, 2, 'blocked outcome should surface every diagnostic');
  assert(
    surface.diagnosticMessages.includes('Smite evil is not yet implemented.'),
    'blocked outcome should show the real diagnostic message verbatim, not a generic error'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
