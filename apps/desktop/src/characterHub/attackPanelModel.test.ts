import { buildAttackTiles } from './attackPanelModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * F-9 (scout audit item 40, first half): `PilotSnapshotDto.baselineMeleeAttackBonus`
 * arrives on every load and on every recompute and was never rendered on
 * the sheet. These pin the Attack panel's tiles: BAB and Melee are the
 * engine's numbers verbatim (signed); CMB/CMD read the engine explanations
 * and show "—" when absent. SD-42 E4.20: Ranged is the engine's
 * `combat.baseline_ranged_attack_bonus` row, beside Melee; with no row the
 * box is empty. No per-weapon total appears here (the Weapons block's).
 */
const tiles = buildAttackTiles({ baseAttackBonus: 3, melee: 5, ranged: 4, cmb: 6, cmd: 18 });
assertEqual(tiles.map((t) => t.label).join(','), 'BAB,Melee,Ranged,CMB,CMD', 'tile order: Ranged beside Melee');
assertEqual(tiles[0].value, '+3', 'BAB signed');
assertEqual(tiles[1].value, '+5', 'melee is the engine value, signed');
assertEqual(tiles[2].value, '+4', 'ranged is the engine row, signed');
assertEqual(tiles[3].value, '+6', 'CMB signed');
assertEqual(tiles[4].value, '18', 'CMD is a plain total');

const absent = buildAttackTiles({ baseAttackBonus: 0, melee: -1, ranged: null, cmb: null, cmd: null });
assertEqual(absent[1].value, '-1', 'negative melee keeps its sign');
assertEqual(absent[2].value, '', 'ranged absent → the empty box');
assertEqual(absent[3].value, '—', 'CMB absent → dash');
assertEqual(absent[4].value, '—', 'CMD absent → dash');
assertEqual(buildAttackTiles({ baseAttackBonus: 0, melee: 0, ranged: -2, cmb: null, cmd: null })[2].value, '-2', 'negative ranged keeps its sign');

console.log('attackPanelModel tests passed');
