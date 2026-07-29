import { buildClassFeatureSurface } from './classFeaturesModel';
import { assert, assertEqual } from '../testSupport/asserts';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import type { HeldClass } from './characterProgression';

const ROGUE: HeldClass[] = [{ classId: 'class:rogue', classLabel: 'Rogue', level: 11 }];

const SNEAK_ATTACK_DETAIL =
  'Rogue level 11 sneak attack from the PF1 Core Rulebook Rogue class table: the sneak ' +
  'attack die count increases by 1 every two rogue levels (1d6 at levels 1-2, 2d6 at level ' +
  '3+): (level + 1) / 2 = (11 + 1) / 2 = 6, i.e. 6d6 sneak attack damage die';

function explanation(id: string, value: number, detail = 'engine detail'): ExplanationDto {
  return { id, value, detail };
}

function verifiesALevel11RoguesSneakAttackKeepsItsMagnitudeAndCitation() {
  const surface = buildClassFeatureSurface(
    [explanation('class_chassis.rogue.sneak_attack', 6, SNEAK_ATTACK_DETAIL)],
    ROGUE
  );

  assertEqual(surface.features.length, 1, 'the sneak-attack record must reach the surface');
  assertEqual(surface.features[0].id, 'class_chassis.rogue.sneak_attack', 'id carried verbatim');
  assertEqual(surface.features[0].classToken, 'rogue', 'attributed to the held Rogue class');
  assertEqual(surface.features[0].label, 'Sneak Attack', 'label humanised from the id');
  assertEqual(surface.features[0].value, 6, 'PF1 sneak attack at Rogue 11 is 6 dice');
}

function verifiesTheDetailTextIsNeverRewritten() {
  const surface = buildClassFeatureSurface(
    [explanation('class_chassis.rogue.sneak_attack', 6, SNEAK_ATTACK_DETAIL)],
    ROGUE
  );

  assertEqual(
    surface.features[0].detail,
    SNEAK_ATTACK_DETAIL,
    'the engine detail is the rules citation and must cross byte-identical'
  );
  assert(
    surface.features[0].detail.includes('6d6'),
    'the dice expression the player needs is inside the engine detail'
  );
}

function verifiesNonClassRecordsAreIgnored() {
  const surface = buildClassFeatureSurface(
    [
      explanation('ability_modifier.strength', 4, 'STR 18 -> +4'),
      explanation('race.human.ability_bonus_applied', 18, 'Human +2 racial'),
      explanation('class_feature.rogue.evasion', 1, 'Rogue level 2 Evasion'),
    ],
    ROGUE
  );

  assertEqual(surface.features.length, 1, 'only the class record is a class feature');
  assertEqual(surface.features[0].id, 'class_feature.rogue.evasion', 'the class record survives');
}

function verifiesUnsupportedNoticesNeverRenderTheirFillerZeroAsAMagnitude() {
  const surface = buildClassFeatureSurface(
    [
      explanation('class_chassis.barbarian.rage_rounds_per_day', 8, 'Barbarian level 2 rage'),
      explanation(
        'class_chassis.barbarian.rage_rounds_per_day.unsupported',
        0,
        'no rage-round budget is grounded for this posture'
      ),
    ],
    [{ classId: 'class:barbarian', classLabel: 'Barbarian', level: 2 }]
  );

  assertEqual(surface.features.length, 1, 'only the grounded record is a feature row');
  assertEqual(
    surface.features[0].id,
    'class_chassis.barbarian.rage_rounds_per_day',
    'the grounded record keeps its magnitude'
  );
  assertEqual(surface.notComputed.length, 1, 'the unsupported record becomes a notice');
  assertEqual(surface.notComputed[0].label, 'Rage Rounds Per Day', 'notice label drops the suffix');
  assertEqual(
    surface.notComputed[0].detail,
    'no rage-round budget is grounded for this posture',
    'the engine explains the absence in its own words'
  );
  assert(
    !Object.prototype.hasOwnProperty.call(surface.notComputed[0], 'value'),
    'a notice carries no value at all, so no caller can render the filler zero'
  );
}

function verifiesPreNamespacingChassisIdsGetNoGuessedOwner() {
  const surface = buildClassFeatureSurface(
    [
      explanation('class_chassis.base_attack_bonus', 5, 'Fighter level 5 BAB'),
      explanation('class_chassis.base_save.will', 1, 'Fighter level 5 Will'),
    ],
    [{ classId: 'class:fighter', classLabel: 'Fighter', level: 5 }]
  );

  assertEqual(surface.features[0].classToken, null, 'no class segment means no owner is invented');
  assertEqual(surface.features[0].label, 'Base Attack Bonus', 'the whole remainder is the label');
  assertEqual(surface.features[1].label, 'Base Save Will', 'dotted segments humanise too');
}

function verifiesARecordIsOnlyAttributedToAClassTheCharacterHolds() {
  // `wizard` is a real class segment, but this character is a Rogue — the
  // segment must not be silently stripped as if it were the owner.
  const surface = buildClassFeatureSurface(
    [explanation('class_chassis.wizard.scribe_scroll', 1, 'Wizard Scribe Scroll')],
    ROGUE
  );

  assertEqual(surface.features[0].classToken, null, 'an unheld class is not treated as the owner');
  assertEqual(surface.features[0].label, 'Wizard Scribe Scroll', 'the segment stays in the label');
}

function verifiesTheEngineEmissionOrderIsPreserved() {
  const surface = buildClassFeatureSurface(
    [
      explanation('class_chassis.rogue.base_attack_bonus', 8),
      explanation('class_chassis.rogue.trapfinding', 5),
      explanation('class_chassis.rogue.sneak_attack', 6),
    ],
    ROGUE
  );

  assertEqual(
    surface.features.map((row) => row.label).join(' | '),
    'Base Attack Bonus | Trapfinding | Sneak Attack',
    'rows stay in the order the engine emitted them'
  );
}

function verifiesABuildWithNoClassRecordsGetsCleanEmptySurfaces() {
  const surface = buildClassFeatureSurface([], ROGUE);

  assertEqual(surface.features.length, 0, 'no features');
  assertEqual(surface.notComputed.length, 0, 'no notices');
}

async function main() {
  verifiesALevel11RoguesSneakAttackKeepsItsMagnitudeAndCitation();
  verifiesTheDetailTextIsNeverRewritten();
  verifiesNonClassRecordsAreIgnored();
  verifiesUnsupportedNoticesNeverRenderTheirFillerZeroAsAMagnitude();
  verifiesPreNamespacingChassisIdsGetNoGuessedOwner();
  verifiesARecordIsOnlyAttributedToAClassTheCharacterHolds();
  verifiesTheEngineEmissionOrderIsPreserved();
  verifiesABuildWithNoClassRecordsGetsCleanEmptySurfaces();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
