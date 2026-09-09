/**
 * Tiles for the sheet's Attack panel (v0.8 F-9). Every value is an engine
 * number formatted for display: `baseAttackBonus` and `melee` are
 * `PilotSnapshotDto.baseAttackBonus` / `.baselineMeleeAttackBonus` (or the
 * recompute equivalents); `cmb` / `cmd` are read off the engine's
 * explanations and are `null` until those arrive. Ranged and per-weapon
 * attack totals are deliberately absent — the engine does not expose them
 * yet (blocker B5) and they are not derived here.
 */
export interface AttackTile {
  label: 'BAB' | 'Melee' | 'CMB' | 'CMD';
  value: string;
}

function signed(value: number): string {
  return value < 0 ? String(value) : `+${value}`;
}

export function buildAttackTiles(input: {
  baseAttackBonus: number;
  melee: number;
  cmb: number | null;
  cmd: number | null;
}): AttackTile[] {
  return [
    { label: 'BAB', value: signed(input.baseAttackBonus) },
    { label: 'Melee', value: signed(input.melee) },
    { label: 'CMB', value: input.cmb === null ? '—' : signed(input.cmb) },
    { label: 'CMD', value: input.cmd === null ? '—' : String(input.cmd) },
  ];
}
