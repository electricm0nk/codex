/**
 * Tiles for the sheet's Attack panel (v0.8 F-9). Every value is an engine
 * number formatted for display: `baseAttackBonus` and `melee` are
 * `PilotSnapshotDto.baseAttackBonus` / `.baselineMeleeAttackBonus` (or the
 * recompute equivalents); `ranged` (SD-42 E4.20), `cmb` and `cmd` are read
 * off the engine's explanations (`combat.baseline_ranged_attack_bonus`, …)
 * and are `null` until those arrive — a sheet with no ranged row shows the
 * empty box. Per-weapon attack totals are the Weapons block's, not here.
 */
export interface AttackTile {
  label: 'BAB' | 'Melee' | 'Ranged' | 'CMB' | 'CMD';
  value: string;
}

function signed(value: number): string {
  return value < 0 ? String(value) : `+${value}`;
}

export function buildAttackTiles(input: {
  baseAttackBonus: number;
  melee: number;
  ranged: number | null;
  cmb: number | null;
  cmd: number | null;
}): AttackTile[] {
  return [
    { label: 'BAB', value: signed(input.baseAttackBonus) },
    { label: 'Melee', value: signed(input.melee) },
    { label: 'Ranged', value: input.ranged === null ? '' : signed(input.ranged) },
    { label: 'CMB', value: input.cmb === null ? '—' : signed(input.cmb) },
    { label: 'CMD', value: input.cmd === null ? '—' : String(input.cmd) },
  ];
}
