import { buildAttackTiles } from './attackPanelModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * F-9 (scout audit item 40, first half): `PilotSnapshotDto.baselineMeleeAttackBonus`
 * arrives on every load and on every recompute and was never rendered on
 * the sheet. These pin the Attack panel's tiles: BAB and Melee are the
 * engine's numbers verbatim (signed); CMB/CMD read the engine explanations
 * and show "—" when absent. No ranged or per-weapon total appears here —
 * that is blocker B5, not something to derive in TS.
 */
const tiles = buildAttackTiles({ baseAttackBonus: 3, melee: 5, cmb: 6, cmd: 18 });
assertEqual(tiles.map((t) => t.label).join(','), 'BAB,Melee,CMB,CMD', 'tile order');
assertEqual(tiles[0].value, '+3', 'BAB signed');
assertEqual(tiles[1].value, '+5', 'melee is the engine value, signed');
assertEqual(tiles[2].value, '+6', 'CMB signed');
assertEqual(tiles[3].value, '18', 'CMD is a plain total');

const absent = buildAttackTiles({ baseAttackBonus: 0, melee: -1, cmb: null, cmd: null });
assertEqual(absent[1].value, '-1', 'negative melee keeps its sign');
assertEqual(absent[2].value, '—', 'CMB absent → dash');
assertEqual(absent[3].value, '—', 'CMD absent → dash');
assert(!tiles.some((t) => /ranged/i.test(t.label)), 'no ranged tile (blocker B5)');

console.log('attackPanelModel tests passed');
