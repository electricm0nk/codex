import { loadCreateCharacter } from './loadCreateCharacter';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the same
 * no-runtime failure path every other boundary loader's test exercises
 * (see `addSpellSelection.test.ts`). risks-and-open-questions.md item 25.
 */
async function testNoRuntimeThrowsDescriptiveError() {
  let thrown: unknown;
  try {
    await loadCreateCharacter({
      characterId: 'char-test',
      displayLabel: 'Test Character',
      raceId: 'race:human',
      classId: 'class:fighter',
      level: 1,
      abilityScores: { strength: 16, dexterity: 14, constitution: 14, intelligence: 10, wisdom: 12, charisma: 8 },
      abilityBonusTarget: 'strength',
      savedAt: '2026-07-24T00:00:00Z',
      selectedAlternateTraitKeys: [],
      selectedTraits: [],
      traitSkillChoices: [],
    });
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(message, 'Tauri runtime not available for creating a character', 'no-runtime failure is descriptive');
}

async function main() {
  await testNoRuntimeThrowsDescriptiveError();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
