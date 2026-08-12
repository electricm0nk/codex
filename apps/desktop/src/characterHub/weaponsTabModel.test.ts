import { ABSENT, buildWeaponsTabSurface } from './weaponsTabModel';
import { assert, assertEqual } from '../testSupport/asserts';
import type { WeaponDamageDto } from '../boundary/loadSavedCharacterDetail';
import type { CorpusDerivedDto } from '../boundary/loadCreateCharacter';

/** The engine's real output for the bundled `Longsword (Base)` fixture. */
function longsword(overrides: Partial<WeaponDamageDto> = {}): WeaponDamageDto {
  return {
    weaponItemId: 'item:longsword',
    weaponRecordKey: 'Longsword (Base)',
    baseDice: { count: 1, dieSize: 8 },
    strDamageModifier: 4,
    wieldCategory: 'OneHanded',
    enhancementAttackBonus: null,
    enhancementDamageBonus: null,
    criticalThreatRange: [19, 20],
    criticalMultiplier: 2,
    featEffects: [],
    ...overrides,
  };
}

const CORPUS_DERIVED = {
  equippedItems: [
    {
      itemId: 'item:longsword',
      equipmentRecordName: 'Longsword',
      equipmentRecordKey: 'Longsword (Base)',
      grounded: true,
      appliedModifiers: [],
    },
  ],
} as unknown as CorpusDerivedDto;

function verifiesAnEquippedLongswordProducesAPopulatedRow() {
  const surface = buildWeaponsTabSurface([longsword()], CORPUS_DERIVED);

  assertEqual(surface.isEmpty, false, 'an equipped weapon is not an empty state');
  assertEqual(surface.rows.length, 1, 'exactly one weapon row');
  const row = surface.rows[0];
  assertEqual(row.itemId, 'item:longsword', 'keyed by the real selection id');
  assertEqual(row.name, 'Longsword', 'named from the resolved corpus record');
  assertEqual(row.baseDice, '1d8', 'the corpus DAMAGE: token');
  assertEqual(row.strDamage, '+4', 'the STR contribution alone');
  assertEqual(row.critical, '19-20/x2', 'CRITRANGE:2 plus CRITMULT:x2');
  assertEqual(row.wield, 'One Handed', 'the corpus WIELD: token, spaced');
  assertEqual(row.enhancementDamage, ABSENT, 'no enhancement on a plain longsword');
}

function verifiesTheFacetsAreNeverSummedIntoADamageTotal() {
  const row = buildWeaponsTabSurface(
    [
      longsword({
        enhancementDamageBonus: 1,
        featEffects: [{ featKey: 'Weapon Specialization', damageBonus: 2 }],
      }),
    ],
    CORPUS_DERIVED
  ).rows[0];

  // 1d8 + 4 + 1 + 2 is NOT computed anywhere: the wield multiplier that
  // would make such a total honest is unknown to the engine, and
  // `contract.rs`'s "no fabricated damage total" boundary note stands.
  const rendered = [row.baseDice, row.strDamage, row.enhancementDamage, ...row.featEffects].join(' ');
  assert(!rendered.includes('1d8+7'), 'no summed total is produced');
  assertEqual(row.baseDice, '1d8', 'base dice stay their own column');
  assertEqual(row.strDamage, '+4', 'STR stays its own column');
  assertEqual(row.enhancementDamage, '+1', 'enhancement stays its own column');
  assertEqual(
    row.featEffects.join(' | '),
    'Weapon Specialization +2',
    'feat effects stay their own column, named'
  );
}

function verifiesAnUngroundedFacetRendersAsAbsentNotZero() {
  const row = buildWeaponsTabSurface(
    [longsword({ strDamageModifier: null, criticalMultiplier: null })],
    CORPUS_DERIVED
  ).rows[0];

  assertEqual(row.strDamage, ABSENT, 'a null STR modifier is absent, not +0');
  assertEqual(
    row.critical,
    '19-20',
    'a grounded threat range with no multiplier omits the multiplier rather than defaulting it to x2'
  );
}

function verifiesAGenuinelyComputedZeroStillRendersAsZero() {
  const row = buildWeaponsTabSurface([longsword({ strDamageModifier: 0 })], CORPUS_DERIVED).rows[0];

  assertEqual(row.strDamage, '+0', 'a real zero is a real value, distinct from absence');
}

function verifiesANegativeStrengthPenaltyKeepsItsSign() {
  const row = buildWeaponsTabSurface([longsword({ strDamageModifier: -1 })], CORPUS_DERIVED).rows[0];

  assertEqual(row.strDamage, '-1', 'a penalty is not shown as a bonus');
}

function verifiesASingleNumberThreatRangeCollapses() {
  const row = buildWeaponsTabSurface(
    [longsword({ criticalThreatRange: [20, 20] })],
    CORPUS_DERIVED
  ).rows[0];

  assertEqual(row.critical, '20/x2', 'a width-1 threat range reads as 20, not 20-20');
}

function verifiesTheDisplayNameFallsBackRatherThanBeingInvented() {
  const noCorpus = buildWeaponsTabSurface([longsword()], null).rows[0];
  assertEqual(noCorpus.name, 'Longsword (Base)', 'falls back to the engine record key');

  const unresolved = buildWeaponsTabSurface([longsword({ weaponRecordKey: null })], null).rows[0];
  assertEqual(unresolved.name, 'item:longsword', 'falls back to the raw id, never a guessed name');
}

function verifiesAnHonestEmptyStateWhenNothingEquippedIsAWeapon() {
  const surface = buildWeaponsTabSurface([], CORPUS_DERIVED);

  assertEqual(surface.isEmpty, true, 'no weapons equipped is a real, distinct state');
  assertEqual(surface.rows.length, 0, 'and produces no placeholder rows');
}

async function main() {
  verifiesAnEquippedLongswordProducesAPopulatedRow();
  verifiesTheFacetsAreNeverSummedIntoADamageTotal();
  verifiesAnUngroundedFacetRendersAsAbsentNotZero();
  verifiesAGenuinelyComputedZeroStillRendersAsZero();
  verifiesANegativeStrengthPenaltyKeepsItsSign();
  verifiesASingleNumberThreatRangeCollapses();
  verifiesTheDisplayNameFallsBackRatherThanBeingInvented();
  verifiesAnHonestEmptyStateWhenNothingEquippedIsAWeapon();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
