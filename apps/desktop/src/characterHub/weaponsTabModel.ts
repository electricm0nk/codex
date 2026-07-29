import type { WeaponDamageDto } from '../boundary/loadSavedCharacterDetail';
import type { CorpusDerivedDto } from '../boundary/loadCreateCharacter';

/**
 * Projects the engine's per-weapon damage breakdown
 * (`damage_total::resolve_weapon_damage_breakdown`) into the Weapons tab's
 * rows.
 *
 * Before this model the tab rendered its column headers and then
 * unconditionally printed "No weapons added yet." — there was no
 * row-rendering path at all, while the engine computed a full corpus-cited
 * breakdown for every equipped weapon on every load.
 *
 * **This module sums nothing.** Each facet is formatted into its own
 * column. No summed "damage roll total" formula exists anywhere in the
 * engine, and the wield multiplier needed to build one honestly is unknown
 * — see `contract.rs`'s `PilotReceipt::weapon_damage` boundary note, whose
 * "no fabricated damage total" decision stands. Adding base dice, the
 * Strength modifier, the enhancement bonus and feat bonuses together here
 * would invent a number the engine deliberately refuses to produce.
 *
 * **Absence stays absence.** A facet the engine returned as `null` (no
 * corpus token on that weapon) formats as `ABSENT`, never as `0`, `+0` or
 * a defaulted `x2`.
 */

/** What every column shows when the engine grounded no value for it. */
export const ABSENT = '—';

export interface WeaponRow {
  /** Stable row key — the character's own `EquipmentSelection.item_id`. */
  itemId: string;
  /**
   * Display name: the corpus record's own name when the item resolved,
   * otherwise the raw `item_id`. Never a prettified guess.
   */
  name: string;
  /** e.g. `'1d8'`. */
  baseDice: string;
  /** e.g. `'+4'` — the Strength modifier's contribution alone. */
  strDamage: string;
  /** e.g. `'+1'` — weapon enhancement damage. Attack side is separate. */
  enhancementDamage: string;
  enhancementAttack: string;
  /** e.g. `'19-20/x2'`. Shows only the halves the engine grounded. */
  critical: string;
  /** e.g. `'One Handed'`, from the corpus `WIELD:` token. */
  wield: string;
  /** e.g. `'Weapon Specialization +2'`, one entry per feat effect. */
  featEffects: string[];
}

export interface WeaponsTabSurface {
  rows: WeaponRow[];
  /**
   * True when the character has no equipped item the engine identifies as
   * a weapon. The tab's honest empty state — distinct from "the engine
   * failed", which surfaces as diagnostics elsewhere.
   */
  isEmpty: boolean;
}

function signed(value: number | null): string {
  return value === null ? ABSENT : `${value >= 0 ? '+' : ''}${value}`;
}

/** `'OneHanded'` -> `'One Handed'`. Spacing only; no vocabulary invented. */
function spaceCamelCase(raw: string): string {
  return raw.replace(/([a-z])([A-Z])/g, '$1 $2');
}

/**
 * `19-20/x2`. Each half is independent: a weapon whose corpus record
 * carries `CRITRANGE:` but no `CRITMULT:` shows the range and omits the
 * multiplier rather than defaulting it to the PF1-common `x2`.
 */
function formatCritical(weapon: WeaponDamageDto): string {
  const range =
    weapon.criticalThreatRange === null
      ? null
      : weapon.criticalThreatRange[0] === weapon.criticalThreatRange[1]
      ? `${weapon.criticalThreatRange[0]}`
      : `${weapon.criticalThreatRange[0]}-${weapon.criticalThreatRange[1]}`;
  const multiplier = weapon.criticalMultiplier === null ? null : `x${weapon.criticalMultiplier}`;
  if (range === null && multiplier === null) {
    return ABSENT;
  }
  if (range === null) {
    return multiplier as string;
  }
  if (multiplier === null) {
    return range;
  }
  return `${range}/${multiplier}`;
}

/**
 * Resolves a weapon's display name from the corpus-resolved equipment the
 * same response already carries, falling back to the engine's own record
 * key and then to the raw item id. Nothing here fabricates a name.
 */
function displayName(weapon: WeaponDamageDto, corpusDerived: CorpusDerivedDto | null): string {
  const resolved = corpusDerived?.equippedItems?.find((item) => item.itemId === weapon.weaponItemId);
  return resolved?.equipmentRecordName ?? weapon.weaponRecordKey ?? weapon.weaponItemId;
}

export function buildWeaponsTabSurface(
  weaponDamage: readonly WeaponDamageDto[],
  corpusDerived: CorpusDerivedDto | null
): WeaponsTabSurface {
  const rows = weaponDamage.map((weapon) => ({
    itemId: weapon.weaponItemId,
    name: displayName(weapon, corpusDerived),
    baseDice:
      weapon.baseDice === null ? ABSENT : `${weapon.baseDice.count}d${weapon.baseDice.dieSize}`,
    strDamage: signed(weapon.strDamageModifier),
    enhancementDamage: signed(weapon.enhancementDamageBonus),
    enhancementAttack: signed(weapon.enhancementAttackBonus),
    critical: formatCritical(weapon),
    wield: weapon.wieldCategory === null ? ABSENT : spaceCamelCase(weapon.wieldCategory),
    featEffects: weapon.featEffects.map(
      (effect) => `${effect.featKey} ${effect.damageBonus >= 0 ? '+' : ''}${effect.damageBonus}`
    ),
  }));

  return { rows, isEmpty: rows.length === 0 };
}
