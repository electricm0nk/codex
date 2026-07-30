import { buildAcBySourceRows, describeEncumbrance, effectiveMaxDexCap } from './encumbranceTabModel';
import { assert, assertEqual } from '../testSupport/asserts';
import type { EncumbranceDto, ResolvedEquipmentEffectDto } from '../boundary/loadCreateCharacter';

/**
 * A real Strength-10 light-load build. Thresholds are the genuine
 * `load.lst` row for Strength 10 (`LOAD:10|100`, so light 33 / medium 66 /
 * heavy 100); the carried weight is Chain Shirt (25 lbs) + Longsword (4
 * lbs), both real CRB `WT:` values.
 */
const LIGHT: EncumbranceDto = {
  totalCarriedWeightLbs: 29,
  totalCarriedCostGp: 115,
  lightMaxLbs: 33,
  mediumMaxLbs: 66,
  heavyMaxLbs: 100,
  level: 'Light',
  loadArmorCheckPenalty: 0,
  perItem: [
    { itemId: 'item:chain_shirt', weightLbs: 25, costGp: 100 },
    { itemId: 'item:longsword', weightLbs: 4, costGp: 15 },
  ],
  unresolvedItemIds: [],
};

/** The same loadout on a Strength-6 character (`LOAD:6|60`): 20/40/60. */
const MEDIUM: EncumbranceDto = {
  ...LIGHT,
  lightMaxLbs: 20,
  mediumMaxLbs: 40,
  heavyMaxLbs: 60,
  level: 'Medium',
  loadMaxDexCap: 3,
  loadArmorCheckPenalty: -3,
};

function verifiesALightLoadReportsNoPenalties() {
  const described = describeEncumbrance(LIGHT);
  assertEqual(described.levelLabel, 'Light Load', 'the tier reads as a human label, not the raw enum name');
  assertEqual(described.totalWeightLabel, '29 lb', 'whole pounds render without a trailing .0');
  assertEqual(described.capacityLabel, '33 / 66 / 100 lb', 'light / medium / heavy maxima');
  assertEqual(described.penalties.length, 0, 'a light load imposes neither a max-Dex cap nor a check penalty');
  assert(!described.overCapacity, 'a light load is not over capacity');
}

function verifiesAMediumLoadSurfacesBothRealPenalties() {
  const described = describeEncumbrance(MEDIUM);
  assertEqual(described.levelLabel, 'Medium Load', 'the medium tier reads as a human label');
  assertEqual(described.penalties.length, 2, 'medium load imposes both a max-Dex cap and a check penalty');
  assert(
    described.penalties.some((penalty) => penalty.label === 'Max Dex' && penalty.value === '+3'),
    'the load max-Dex cap of +3 must be shown',
  );
  assert(
    described.penalties.some((penalty) => penalty.label === 'Armor Check' && penalty.value === '-3'),
    'the load armor check penalty of -3 must be shown',
  );
}

function verifiesRemainingCapacityIsMeasuredAgainstTheHeavyMaximum() {
  const described = describeEncumbrance(LIGHT);
  assertEqual(described.remainingLbs, 71, '100 lb heavy maximum less 29 lb carried');
}

function verifiesGoingOverTheHeavyMaximumIsFlaggedRatherThanShownAsNegative() {
  const described = describeEncumbrance({
    ...MEDIUM,
    totalCarriedWeightLbs: 120,
    level: 'OverHeavyCapacity',
    loadMaxDexCap: 0,
    loadArmorCheckPenalty: -6,
  });
  assertEqual(described.levelLabel, 'Over Capacity', 'the fourth tier gets its own honest label');
  assert(described.overCapacity, 'past the heavy maximum must be flagged');
  assertEqual(described.remainingLbs, 0, 'remaining capacity floors at zero rather than going negative');
}

function verifiesFractionalWeightsAndPricesKeepTheirPrecision() {
  const described = describeEncumbrance({
    ...LIGHT,
    totalCarriedWeightLbs: 0.5,
    totalCarriedCostGp: 0.05,
  });
  assertEqual(described.totalWeightLabel, '0.5 lb', 'a real fractional corpus weight is not rounded away');
  assertEqual(described.totalCostLabel, '0.05 gp', 'a real fractional corpus price is not rounded away');
}

function verifiesUnresolvedCarriedItemsAreReportedRatherThanHidden() {
  const described = describeEncumbrance({ ...LIGHT, unresolvedItemIds: ['item:mystery'] });
  assertEqual(described.unresolvedCount, 1, 'an unweighable carried item must be surfaced, never silently dropped');
}

const EFFECTS: ResolvedEquipmentEffectDto[] = [
  {
    itemId: 'item:chain_shirt',
    equipmentRecordKey: 'Chain Shirt (Base)',
    category: 'ArmsArmor',
    armorClassBonus: 4,
    maxDex: 4,
    spellFailure: 20,
    armorCheckPenalty: -2,
  },
  { itemId: 'item:longsword', equipmentRecordKey: 'Longsword (Base)', category: 'ArmsArmor' },
];

function verifiesAcBySourceListsOnlyItemsThatActuallyContributeAc() {
  const rows = buildAcBySourceRows(EFFECTS);
  assertEqual(rows.length, 1, 'a longsword contributes no armor bonus and must not appear as a 0 row');
  assertEqual(rows[0].label, 'Chain Shirt', 'the "(Base)" template suffix is dropped for display');
  assertEqual(rows[0].armorClassBonus, 4, 'the real corpus AC bonus');
}

function verifiesAcBySourceIsEmptyRatherThanFabricatedWhenNothingIsEquipped() {
  assertEqual(buildAcBySourceRows([]).length, 0, 'nothing equipped yields no rows, not a placeholder row');
}

function verifiesEffectiveMaxDexTakesTheTighterOfArmorAndLoad() {
  assertEqual(effectiveMaxDexCap(4, 3), 3, 'a medium load (+3) is tighter than a chain shirt (+4)');
  assertEqual(effectiveMaxDexCap(2, 3), 2, 'the armor is tighter than the load here');
  assertEqual(effectiveMaxDexCap(4, undefined), 4, 'a light load imposes no cap of its own');
  assertEqual(effectiveMaxDexCap(undefined, 1), 1, 'a heavy load caps even with no armor worn');
  assertEqual(effectiveMaxDexCap(undefined, undefined), undefined, 'no cap from either source is genuinely no cap');
}

function main() {
  verifiesALightLoadReportsNoPenalties();
  verifiesAMediumLoadSurfacesBothRealPenalties();
  verifiesRemainingCapacityIsMeasuredAgainstTheHeavyMaximum();
  verifiesGoingOverTheHeavyMaximumIsFlaggedRatherThanShownAsNegative();
  verifiesFractionalWeightsAndPricesKeepTheirPrecision();
  verifiesUnresolvedCarriedItemsAreReportedRatherThanHidden();
  verifiesAcBySourceListsOnlyItemsThatActuallyContributeAc();
  verifiesAcBySourceIsEmptyRatherThanFabricatedWhenNothingIsEquipped();
  verifiesEffectiveMaxDexTakesTheTighterOfArmorAndLoad();
  console.log('encumbranceTabModel.test.ts: all assertions passed');
}

main();
