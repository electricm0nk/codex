// Self-executing, DOM-free test for resultsSkeleton.mjs's pure
// skeleton/merge/summary logic. Run directly: `node resultsSkeleton.test.mjs`
// (exits non-zero on the first failed assertion). No spec.json, no probe
// file, no running app -- this is the piece a regression in would make
// every run silently under-report its own denominator (see
// resultsSkeleton.mjs's own header comment).
import assert from 'node:assert/strict';
import { applyResult, buildResumeState, buildSkeleton, hasNotRun, summaryLine } from './resultsSkeleton.mjs';

const rows = [{ id: 'a' }, { id: 'b' }, { id: 'c' }];

// --- buildSkeleton -----------------------------------------------------
{
  const skeleton = buildSkeleton(rows);
  assert.equal(skeleton.length, rows.length, 'one entry per row');
  assert.deepEqual(
    skeleton.map((r) => r.id),
    ['a', 'b', 'c'],
    'skeleton preserves row order',
  );
  for (const entry of skeleton) {
    assert.equal(entry.status, 'not-run');
    assert.equal(entry.screenshot, null);
    assert.equal(entry.rendered_excerpt, null);
    assert.equal(entry.reason, 'not executed');
  }
  assert.ok(hasNotRun(skeleton), 'a fresh skeleton always has not-run rows');
}

// --- applyResult: replaces in place, preserves every other row/position ---
{
  const skeleton = buildSkeleton(rows);
  const afterA = applyResult(skeleton, { id: 'a', status: 'green', screenshot: 'a.png', rendered_excerpt: 'x', reason: null });
  assert.equal(afterA.length, 3, 'row count unchanged after one replace');
  assert.deepEqual(
    afterA.map((r) => r.id),
    ['a', 'b', 'c'],
    'row order unchanged after one replace',
  );
  assert.equal(afterA[0].status, 'green');
  assert.equal(afterA[1].status, 'not-run', 'row b untouched by replacing row a');
  assert.equal(afterA[2].status, 'not-run', 'row c untouched by replacing row a');
  // Original array must not have been mutated (pure function contract).
  assert.equal(skeleton[0].status, 'not-run', 'applyResult does not mutate its input');

  const afterB = applyResult(afterA, { id: 'b', status: 'red', screenshot: null, rendered_excerpt: null, reason: 'boom' });
  assert.equal(afterB[0].status, 'green', 'earlier replace (row a) survives a later one (row b)');
  assert.equal(afterB[1].status, 'red');

  // Defensive: an id with no existing entry is appended, not dropped.
  const appended = applyResult(afterB, { id: 'd', status: 'green', screenshot: null, rendered_excerpt: null, reason: null });
  assert.equal(appended.length, 4);
  assert.equal(appended[3].id, 'd');
}

// --- buildResumeState: no prior results -> everything fresh, everything to run ---
{
  const { results, toRun } = buildResumeState(rows, null);
  assert.equal(results.length, 3);
  assert.ok(results.every((r) => r.status === 'not-run'));
  assert.equal(toRun.length, 3, 'no prior results means every row is re-run');
  assert.deepEqual(
    toRun.map((r) => r.id),
    ['a', 'b', 'c'],
  );
}

// --- buildResumeState: green/manual preserved verbatim, red/not-run/missing re-run ---
{
  const previous = [
    { id: 'a', status: 'green', screenshot: 'a.png', rendered_excerpt: 'ok', reason: null },
    { id: 'b', status: 'red', screenshot: null, rendered_excerpt: null, reason: 'marker not found' },
    // 'c' missing entirely (e.g. this results.json was killed mid-run before reaching it).
  ];
  const { results, toRun } = buildResumeState(rows, previous);
  assert.equal(results.length, 3, 'full row coverage regardless of what the previous file held');
  assert.deepEqual(results[0], previous[0], 'green row a carried forward verbatim, byte for byte');
  assert.equal(results[1].status, 'not-run', 'red row b is reset to not-run for re-execution');
  assert.equal(results[1].reason, 'not executed', 're-run reset clears the stale red reason');
  assert.equal(results[2].status, 'not-run', 'row c missing from the previous file starts fresh');
  assert.deepEqual(
    toRun.map((r) => r.id),
    ['b', 'c'],
    'only the non-green/manual rows are selected to actually run',
  );
}

// --- buildResumeState: manual rows are preserved just like green ones ---
{
  const previous = [{ id: 'a', status: 'manual', screenshot: null, rendered_excerpt: null, reason: 'opens a native OS dialog' }];
  const { results, toRun } = buildResumeState(rows, previous);
  assert.equal(results[0].status, 'manual');
  assert.deepEqual(
    toRun.map((r) => r.id),
    ['b', 'c'],
    'manual rows are skipped on resume same as green ones',
  );
}

// --- buildResumeState: an id no longer present in `rows` is dropped, not kept ---
{
  const previous = [
    { id: 'a', status: 'green', screenshot: null, rendered_excerpt: null, reason: null },
    { id: 'zzz-retired-row', status: 'green', screenshot: null, rendered_excerpt: null, reason: null },
  ];
  const { results } = buildResumeState(rows, previous);
  assert.equal(results.length, 3, 'only current spec rows appear in the resumed results');
  assert.ok(!results.some((r) => r.id === 'zzz-retired-row'));
}

// --- summaryLine: exact wording, denominator always matches results.length ---
{
  const results = [
    { id: 'a', status: 'green' },
    { id: 'b', status: 'red' },
    { id: 'c', status: 'blocked' },
    { id: 'd', status: 'manual' },
    { id: 'e', status: 'not-run' },
  ];
  assert.equal(summaryLine(results), '1/5 green, 1 red, 1 blocked, 1 manual, 1 not-run (M = 5 rows)');
  assert.ok(hasNotRun(results));
}
{
  // All-green, zero not-run -- the shape a genuinely complete run must reach.
  const results = [
    { id: 'a', status: 'green' },
    { id: 'b', status: 'manual' },
  ];
  assert.equal(summaryLine(results), '1/2 green, 0 red, 0 blocked, 1 manual, 0 not-run (M = 2 rows)');
  assert.ok(!hasNotRun(results), 'a fully-executed run has no not-run rows left');
}
{
  assert.equal(summaryLine([]), '0/0 green, 0 red, 0 blocked, 0 manual, 0 not-run (M = 0 rows)');
  assert.ok(!hasNotRun([]));
}

console.log('resultsSkeleton.test.mjs OK');
