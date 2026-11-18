import type { WeaponDamageDto } from '../boundary/loadSavedCharacterDetail';
import type { CorpusDerivedDto } from '../boundary/loadCreateCharacter';
import type { ActiveStateDto } from '../boundary/setEquipmentActiveState';

/**
 * Projects the engine's per-weapon damage breakdown
 * (`damage_total::resolve_weapon_damage_breakdown_with_carried`) into the
 * Weapons tab's rows: in hand, then carried, then the Unarmed row last
 * (SD-41 `decisions.md §9, §22`), and 20 ammunition boxes under each weapon
 * that fires ammunition (`decisions.md §23`).
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

/** Tick boxes under a weapon that fires ammunition: a write-in, not a count (`decisions.md §23`). */
export const AMMUNITION_BOXES = 20;

export type WeaponGroup = 'inHand' | 'carried' | 'unarmed';

const GROUP_ORDER: readonly WeaponGroup[] = ['inHand', 'carried', 'unarmed'];

export interface WeaponRow {
  /** Stable row key — the character's own `EquipmentSelection.item_id`. */
  itemId: string;
  /**
   * Display name: the corpus record's own name when the item resolved,
   * otherwise the raw `item_id`. Never a prettified guess.
   */
  name: string;
  /** e.g. `'1d8'`; the copy's own die when its file writes one (as written when it is not plain dice). */
  baseDice: string;
  /** The copy's properties, printed under the row as text. */
  properties: string[];
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
  /** In hand, carried (stowed) or the Unarmed row; the surface lists them in that order. */
  group: WeaponGroup;
  /** `'(carried)'`, `'(nonlethal)'`, or `null`: the words shown beside the name. */
  mark: string | null;
  /** The ammunition boxes and the engine's count, or `null` for a weapon that fires none. */
  ammunition: { boxes: number; carried: string | null } | null;
  /**
   * The `set_equipment_active_state` request Drop sends: this row's own copy (its `+N`, the state
   * the row shows it in) to Not Carried (`decisions.md §27`); `null` on the Unarmed row.
   */
  drop: { itemId: string; bonus: number; fromState: ActiveStateDto; activeState: 'Absent' } | null;
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

function groupOf(weapon: WeaponDamageDto): WeaponGroup {
  return weapon.unarmed ? 'unarmed' : weapon.inHand ? 'inHand' : 'carried';
}

function markOf(weapon: WeaponDamageDto): string | null {
  if (!weapon.inHand) {
    return '(carried)';
  }
  return weapon.nonlethal ? '(nonlethal)' : null;
}

/** `'Arrow: 20 carried'` when the engine counts any; the record's own family word, never a guess. */
function ammunitionOf(weapon: WeaponDamageDto): WeaponRow['ammunition'] {
  if (!weapon.usesAmmunition) {
    return null;
  }
  const carried =
    weapon.ammunitionCarried > 0 ? `${weapon.ammunitionFamily ?? 'Ammunition'}: ${weapon.ammunitionCarried} carried` : null;
  return { boxes: AMMUNITION_BOXES, carried };
}

export function buildWeaponsTabSurface(
  weaponDamage: readonly WeaponDamageDto[],
  corpusDerived: CorpusDerivedDto | null
): WeaponsTabSurface {
  const ordered = GROUP_ORDER.flatMap((group) => weaponDamage.filter((weapon) => groupOf(weapon) === group));
  const rows = ordered.map((weapon) => ({
    itemId: weapon.weaponItemId,
    name: displayName(weapon, corpusDerived),
    baseDice:
      weapon.baseDice === null ? weapon.damageOverride ?? ABSENT : `${weapon.baseDice.count}d${weapon.baseDice.dieSize}`,
    properties: weapon.properties ?? [],
    strDamage: signed(weapon.strDamageModifier),
    enhancementDamage: signed(weapon.enhancementDamageBonus),
    enhancementAttack: signed(weapon.enhancementAttackBonus),
    critical: formatCritical(weapon),
    wield: weapon.wieldCategory === null ? ABSENT : spaceCamelCase(weapon.wieldCategory),
    featEffects: weapon.featEffects.map(
      (effect) => `${effect.featKey} ${effect.damageBonus >= 0 ? '+' : ''}${effect.damageBonus}`
    ),
    group: groupOf(weapon),
    mark: markOf(weapon),
    ammunition: ammunitionOf(weapon),
    drop: weapon.unarmed
      ? null
      : {
          itemId: weapon.weaponItemId,
          // The copy's `+N` is its enhancement to damage; masterwork adds to attack only, so it never reads as a `+1`.
          bonus: weapon.enhancementDamageBonus ?? 0,
          fromState: (weapon.inHand ? 'EquippedActive' : 'SelectedInactive') as ActiveStateDto,
          activeState: 'Absent' as const,
        },
  }));

  return { rows, isEmpty: rows.length === 0 };
}
