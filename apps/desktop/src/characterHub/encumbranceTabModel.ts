import type { EncumbranceDto, ResolvedEquipmentEffectDto } from '../boundary/loadCreateCharacter';

/**
 * Pure presentation model for the character sheet's carried-weight and
 * AC-by-source views. Mirrors `featsTabModel.ts`'s posture exactly: the
 * engine numbers arrive already correct, and this module's only job is to
 * make them legible without inventing any of them.
 *
 * Nothing here computes a rule. Carrying-capacity thresholds, the load
 * tier, and the tier's max-Dex/armor-check penalties are all computed in
 * `src/rules_core/encumbrance.rs` against the real PCGen Pathfinder
 * `load.lst` table and PCGen's own engine behaviour, and cross the IPC
 * boundary as `EncumbranceDto`.
 */

/** One penalty the current load tier imposes, ready to render. */
export interface LoadPenalty {
  label: string;
  value: string;
}

export interface DescribedEncumbrance {
  levelLabel: string;
  totalWeightLabel: string;
  totalCostLabel: string;
  capacityLabel: string;
  /** Pounds still carryable before exceeding the heavy maximum; floors at 0. */
  remainingLbs: number;
  /** Fraction of the heavy maximum currently carried, clamped to 0..1. */
  fractionOfCapacity: number;
  overCapacity: boolean;
  penalties: LoadPenalty[];
  unresolvedCount: number;
}

const LEVEL_LABELS: Record<string, string> = {
  Light: 'Light Load',
  Medium: 'Medium Load',
  Heavy: 'Heavy Load',
  OverHeavyCapacity: 'Over Capacity',
};

/**
 * Render a number without inventing precision: real corpus weights and
 * prices are frequently fractional (an arrow costs 0.05 gp), so a blanket
 * `toFixed(1)` would both lie about whole numbers and truncate small ones.
 */
function trim(value: number): string {
  return Number.isInteger(value) ? String(value) : String(Number(value.toFixed(2)));
}

export function describeEncumbrance(encumbrance: EncumbranceDto): DescribedEncumbrance {
  const {
    totalCarriedWeightLbs,
    totalCarriedCostGp,
    lightMaxLbs,
    mediumMaxLbs,
    heavyMaxLbs,
    level,
    loadMaxDexCap,
    loadArmorCheckPenalty,
    unresolvedItemIds,
  } = encumbrance;

  const penalties: LoadPenalty[] = [];
  // A light load genuinely imposes neither penalty, so the absence of
  // tiles here is the honest state rather than a missing render.
  if (loadMaxDexCap !== undefined) {
    penalties.push({ label: 'Max Dex', value: `+${loadMaxDexCap}` });
  }
  if (loadArmorCheckPenalty !== 0) {
    penalties.push({ label: 'Armor Check', value: String(loadArmorCheckPenalty) });
  }

  const remainingLbs = Math.max(0, heavyMaxLbs - totalCarriedWeightLbs);

  return {
    levelLabel: LEVEL_LABELS[level] ?? level,
    totalWeightLabel: `${trim(totalCarriedWeightLbs)} lb`,
    totalCostLabel: `${trim(totalCarriedCostGp)} gp`,
    capacityLabel: `${trim(lightMaxLbs)} / ${trim(mediumMaxLbs)} / ${trim(heavyMaxLbs)} lb`,
    remainingLbs,
    fractionOfCapacity:
      heavyMaxLbs > 0 ? Math.min(1, Math.max(0, totalCarriedWeightLbs / heavyMaxLbs)) : 0,
    overCapacity: level === 'OverHeavyCapacity',
    penalties,
    unresolvedCount: unresolvedItemIds.length,
  };
}

/** One equipped item's contribution to AC, ready to render. */
export interface AcSourceRow {
  itemId: string;
  label: string;
  armorClassBonus: number;
  maxDex?: number;
  armorCheckPenalty?: number;
  spellFailure?: number;
}

/**
 * Strip the corpus's `(Base)` template suffix for display. The key is the
 * real corpus `KEY:` token (e.g. `"Chain Shirt (Base)"`); the parenthetical
 * marks the record as a base template, which is an ingestion detail rather
 * than something to show a player.
 */
function displayLabel(equipmentRecordKey: string): string {
  return equipmentRecordKey.replace(/\s*\(Base\)\s*$/, '').trim();
}

/**
 * The rows behind an "AC breakdown by source" view: only items that
 * genuinely contribute an armor bonus. An equipped longsword has no
 * `armorClassBonus` at all, and must be omitted rather than shown as a
 * fabricated `0` row.
 */
export function buildAcBySourceRows(perItem: ResolvedEquipmentEffectDto[]): AcSourceRow[] {
  return perItem
    .filter((effect) => effect.armorClassBonus !== undefined)
    .map((effect) => ({
      itemId: effect.itemId,
      label: displayLabel(effect.equipmentRecordKey),
      armorClassBonus: effect.armorClassBonus as number,
      maxDex: effect.maxDex,
      armorCheckPenalty: effect.armorCheckPenalty,
      spellFailure: effect.spellFailure,
    }));
}

/**
 * PF1's effective max-Dex cap is the *tighter* of what the worn armor
 * allows and what the current load tier allows — the two constrain
 * independently and never sum. Mirrors PCGen's own resolution
 * (`PlayerCharacter.java:5374-5385`, which walks equipped items keeping the
 * lower cap). `undefined` from both sources means genuinely uncapped.
 */
export function effectiveMaxDexCap(
  armorMaxDex: number | undefined,
  loadMaxDex: number | undefined,
): number | undefined {
  if (armorMaxDex === undefined) {
    return loadMaxDex;
  }
  if (loadMaxDex === undefined) {
    return armorMaxDex;
  }
  return Math.min(armorMaxDex, loadMaxDex);
}
