import { buildSaveRows } from './defenseSavesModel';
import type { BaseSavesDto } from '../boundary/loadCreateCharacter';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * F-4 (scout audit item 58): the Defense tab printed "Save modifiers by
 * source — coming soon." while `PilotSnapshotDto.baseSaves` and
 * `PilotSnapshotDto.totalSaves` both arrived on every load. These tests pin
 * the presentation: three rows in canonical Fort/Ref/Will order, every
 * number verbatim from the engine, written with an explicit sign. No
 * arithmetic happens here — the row never derives a "bonus" column from
 * total minus base, because that would author a rules value in TS.
 */

const base: BaseSavesDto = { fortitude: 3, reflex: 0, will: -1 };
const total: BaseSavesDto = { fortitude: 5, reflex: 2, will: 0 };

const rows = buildSaveRows(base, total);

assertEqual(rows.length, 3, 'one row per saving throw');
assertEqual(rows.map((r) => r.label).join(','), 'Fortitude,Reflex,Will', 'canonical order');

assertEqual(rows[0].base, 3, 'fort base verbatim');
assertEqual(rows[0].total, 5, 'fort total verbatim');
assertEqual(rows[0].renderedBase, '+3', 'positive base carries a sign');
assertEqual(rows[0].renderedTotal, '+5', 'positive total carries a sign');

assertEqual(rows[1].renderedBase, '+0', 'zero is written +0');
assertEqual(rows[1].renderedTotal, '+2', 'reflex total');

assertEqual(rows[2].renderedBase, '-1', 'negative base keeps its sign');
assertEqual(rows[2].renderedTotal, '+0', 'will total');

assert(
  !Object.keys(rows[0]).some((k) => /bonus|delta|diff|modifier/i.test(k)),
  'rows expose no derived difference column — that would be TS rules math',
);

console.log('defenseSavesModel tests passed');
