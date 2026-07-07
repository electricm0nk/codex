import { composeCreateCharacterRequest } from './composeCreateCharacterRequest';
import { assertEqual } from '../testSupport/asserts';

async function main() {
  verifiesRequestShapeFromFormFields();
}

function verifiesRequestShapeFromFormFields() {
  const request = composeCreateCharacterRequest(
    {
      displayLabel: 'Aldric',
      raceId: 'race:human',
      classId: 'class:fighter',
      level: 1,
      abilityScores: {
        strength: 16,
        dexterity: 14,
        constitution: 14,
        intelligence: 10,
        wisdom: 12,
        charisma: 8,
      },
    },
    {
      generateId: () => 'char-fixed-id',
      now: () => '2026-07-08T00:00:00Z',
    }
  );

  assertEqual(request.characterId, 'char-fixed-id', 'characterId');
  assertEqual(request.displayLabel, 'Aldric', 'displayLabel');
  assertEqual(request.raceId, 'race:human', 'raceId');
  assertEqual(request.classId, 'class:fighter', 'classId');
  assertEqual(request.level, 1, 'level');
  assertEqual(request.savedAt, '2026-07-08T00:00:00Z', 'savedAt');
  assertEqual(request.abilityScores.strength, 16, 'ability strength');
  assertEqual(request.abilityScores.charisma, 8, 'ability charisma');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
