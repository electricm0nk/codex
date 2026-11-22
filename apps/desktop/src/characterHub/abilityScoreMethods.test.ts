import { generateAbilityScorePool, pointBuyCost, rollStraightAbilityScores } from './abilityScoreMethods';
import { ABILITY_KEYS } from './characterHubModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * risks-and-open-questions.md item 25: real dice/point-buy logic, zero test
 * coverage anywhere before this.
 */
function verifiesPointBuyCostAtTableBoundariesAndOneInRangeValue() {
  assertEqual(pointBuyCost(7), -4, 'the minimum buyable score (7) costs -4 points (a refund)');
  assertEqual(pointBuyCost(18), 17, 'the maximum buyable score (18) costs 17 points');
  assertEqual(pointBuyCost(14), 5, 'an in-range score (14) costs 5 points');
}

function verifiesGenerateAbilityScorePoolEliteArrayIsTheExactFixedArray() {
  assertEqual(
    JSON.stringify(generateAbilityScorePool('eliteArray')),
    JSON.stringify([15, 14, 13, 12, 10, 8]),
    'Elite Array is a fixed, non-random pool with no luck involved'
  );
}

function verifiesGenerateAbilityScorePoolRandomizedMethodsReturnSixValues() {
  const randomizedMethods = ['standardRoll', 'roll2d6Plus6', 'roll4d6RerollOnes', 'roll4d6RerollIfWeak'] as const;
  for (const methodId of randomizedMethods) {
    assertEqual(generateAbilityScorePool(methodId).length, 6, `${methodId} generates exactly six values`);
  }
}

function verifiesRollStraightAbilityScoresMapsOntoAbilityKeysInDocumentedOrder() {
  const scores = rollStraightAbilityScores();
  assertEqual(
    Object.keys(scores).join(','),
    ABILITY_KEYS.join(','),
    'rollStraightAbilityScores assigns its six rolls onto ABILITY_KEYS in Str/Dex/Con/Int/Wis/Cha order, not any other order'
  );
  for (const key of ABILITY_KEYS) {
    assert(scores[key] >= 3 && scores[key] <= 18, `${key} is a valid 3d6 roll (3-18)`);
  }
}

async function main() {
  verifiesPointBuyCostAtTableBoundariesAndOneInRangeValue();
  verifiesGenerateAbilityScorePoolEliteArrayIsTheExactFixedArray();
  verifiesGenerateAbilityScorePoolRandomizedMethodsReturnSixValues();
  verifiesRollStraightAbilityScoresMapsOntoAbilityKeysInDocumentedOrder();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
