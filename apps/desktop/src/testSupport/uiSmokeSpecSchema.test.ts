/**
 * Schema sanity check for `apps/desktop/scripts/ui-smoke/spec.json`. This
 * cannot launch the app (that's `npm run ui-smoke`'s job, and it needs a
 * live Xvfb + Tauri window this test suite must not depend on), but a
 * malformed spec row is a defect independent of whether the app is running
 * — a dangling `setup` reference or a row with nothing to assert would fail
 * silently or crash mid-run instead of failing fast here.
 */
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assert } from './asserts';

interface SpecRow {
  id: string;
  screen: string;
  setup?: string[];
  steps: unknown[];
  marker: string;
  expect?: string[];
  forbid?: string[];
  selectsNonEmpty?: string[];
  manual?: string;
  notes?: string;
}

const HERE = dirname(fileURLToPath(import.meta.url));
const SPEC_PATH = join(HERE, '..', '..', 'scripts', 'ui-smoke', 'spec.json');

function loadRows(): SpecRow[] {
  const parsed = JSON.parse(readFileSync(SPEC_PATH, 'utf8')) as { rows: SpecRow[] };
  return parsed.rows;
}

function verifiesIdsAreUnique(rows: SpecRow[]) {
  const seen = new Set<string>();
  for (const row of rows) {
    assert(!seen.has(row.id), `duplicate row id: '${row.id}'`);
    seen.add(row.id);
  }
}

function verifiesEverySetupRefExists(rows: SpecRow[]) {
  const ids = new Set(rows.map((row) => row.id));
  for (const row of rows) {
    for (const setupId of row.setup ?? []) {
      assert(ids.has(setupId), `row '${row.id}' has a setup ref to unknown row id '${setupId}'`);
    }
  }
}

// A `manual` row is never executed by run.mjs (it is recorded as
// status "manual" without touching the app), so it is exempt from the
// "has something to assert" rule below — its whole point is that this
// cycle could not confirm a marker/expect live. Every OTHER row must have
// something a real run can fail on.
function verifiesEveryAutomatedRowHasSomethingToAssert(rows: SpecRow[]) {
  for (const row of rows) {
    if (row.manual) continue;
    assert(Boolean(row.marker && row.marker.length > 0), `automated row '${row.id}' has no marker`);
    const hasExpect = Boolean(row.expect && row.expect.length > 0);
    const hasSelects = Boolean(row.selectsNonEmpty && row.selectsNonEmpty.length > 0);
    assert(hasExpect || hasSelects, `automated row '${row.id}' asserts nothing (no expect[] and no selectsNonEmpty[])`);
  }
}

function verifiesNoDuplicateStepOrderIsMalformed(rows: SpecRow[]) {
  for (const row of rows) {
    assert(Array.isArray(row.steps), `row '${row.id}'.steps must be an array`);
    for (const step of row.steps as Array<{ op?: string }>) {
      assert(typeof step === 'object' && step !== null && typeof step.op === 'string', `row '${row.id}' has a malformed step`);
    }
  }
}

function main() {
  const rows = loadRows();
  assert(rows.length > 0, 'spec.json must declare at least one row');
  verifiesIdsAreUnique(rows);
  verifiesEverySetupRefExists(rows);
  verifiesEveryAutomatedRowHasSomethingToAssert(rows);
  verifiesNoDuplicateStepOrderIsMalformed(rows);
}

main();
