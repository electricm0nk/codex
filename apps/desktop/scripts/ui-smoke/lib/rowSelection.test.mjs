// Self-executing, DOM-free test for rowSelection.mjs. Run directly:
// `node rowSelection.test.mjs` (exits non-zero on the first failed assertion).
// `--only` takes one id or a comma-separated list; an unknown id is an error,
// never a silently shorter run (the run's denominator is the ids asked for).
import assert from 'node:assert/strict';
import { parseOnlyIds, selectRows } from './rowSelection.mjs';

const rows = [{ id: 'a' }, { id: 'b' }, { id: 'c' }, { id: 'd' }];
const spec = { rows, byId: new Map(rows.map((row) => [row.id, row])) };

// --- parseOnlyIds ---------------------------------------------------------
assert.deepEqual(parseOnlyIds('b'), ['b'], 'a single id stays a single id');
assert.deepEqual(parseOnlyIds('b,d'), ['b', 'd'], 'comma-separated ids split');
assert.deepEqual(parseOnlyIds(' b , d '), ['b', 'd'], 'whitespace around ids is trimmed');
assert.throws(() => parseOnlyIds('b,,d'), /empty id/, 'an empty id is an error');
assert.throws(() => parseOnlyIds('b,b'), /more than once/, 'a repeated id is an error');

// --- selectRows -----------------------------------------------------------
assert.deepEqual(selectRows(spec, { only: null, from: null }).map((r) => r.id), ['a', 'b', 'c', 'd'], 'no filter: every row');
assert.deepEqual(selectRows(spec, { only: 'c', from: null }).map((r) => r.id), ['c'], '--only one id');
assert.deepEqual(
  selectRows(spec, { only: 'd,a', from: null }).map((r) => r.id),
  ['d', 'a'],
  '--only a list runs the rows in the order asked',
);
assert.throws(() => selectRows(spec, { only: 'a,zz', from: null }), /--only zz: no such row id/, 'an unknown id in the list is an error');
assert.deepEqual(selectRows(spec, { only: null, from: 'c' }).map((r) => r.id), ['c', 'd'], '--from slices');
assert.throws(() => selectRows(spec, { only: null, from: 'zz' }), /--from zz: no such row id/);

console.log('rowSelection.test.mjs: all assertions passed');
