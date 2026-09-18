// Pure, DOM-free result-bookkeeping logic for run.mjs's denominator
// discipline: every row the runner SELECTS gets an entry in results.json
// from the very first write, before any row has actually executed, so a
// killed/interrupted run (e.g. a shell call hitting its own timeout) leaves
// behind a file whose row COUNT always matches the run's own denominator --
// the missing work shows up as explicit `not-run` rows, not as a shorter
// array silently mistaken for a complete one (see this file's own
// `summaryLine` and run.mjs's `$comment`-documented history: cycle 2's
// results.json had only 47 of 69 rows and was read as "the whole suite" for
// exactly this reason).
//
// Unit-tested directly (`resultsSkeleton.test.mjs`, runnable standalone via
// `node`) without a real DOM, a spec.json, or a running app -- this is the
// piece a regression here would make every run silently under-report its
// own denominator.

/** One fresh 'not-run' entry, same shape runRow() itself returns for every other status. */
function notRunEntry(rowId) {
  return { id: rowId, status: 'not-run', screenshot: null, rendered_excerpt: null, reason: 'not executed' };
}

/** Builds the initial skeleton: one 'not-run' entry per row, in row order. */
export function buildSkeleton(rows) {
  return rows.map((row) => notRunEntry(row.id));
}

/**
 * Returns a NEW array with `result` placed at `result.id`'s position,
 * replacing whatever entry was already there (skeleton or a prior attempt),
 * every other entry left untouched and in place. Appends when `result.id`
 * is not already present (defensive -- should not happen once the run
 * starts from a full skeleton, but this keeps the function total rather
 * than throwing). Pure: never mutates `results`.
 */
export function applyResult(results, result) {
  const idx = results.findIndex((r) => r.id === result.id);
  if (idx === -1) {
    return [...results, result];
  }
  const next = results.slice();
  next[idx] = result;
  return next;
}

/**
 * `--resume` support. Given the full ordered row list this invocation would
 * otherwise run (`rows`, already narrowed by --only/--from) and whatever
 * `previousResults` a prior results.json at --out held (or `null`/`[]`),
 * returns:
 *   - `results`: one entry per row in `rows`, IN ROW ORDER -- a prior
 *     'green' or 'manual' entry is carried forward verbatim (preserved, not
 *     re-run); every other row (missing from `previousResults` entirely, or
 *     previously 'not-run'/'red'/'blocked') gets a fresh 'not-run' entry.
 *   - `toRun`: the subset of `rows` (same row objects, same order) whose
 *     entry was reset -- what the caller should actually execute this time.
 * Always full coverage of `rows` regardless of what `previousResults`
 * contained: an id no longer in `rows` (e.g. a spec row since removed) is
 * silently dropped, and an id in `rows` missing from `previousResults`
 * starts fresh, same as a non-resumed run.
 */
export function buildResumeState(rows, previousResults) {
  const prevById = new Map();
  for (const entry of previousResults ?? []) {
    if (entry && typeof entry.id === 'string') {
      prevById.set(entry.id, entry);
    }
  }
  const results = [];
  const toRun = [];
  for (const row of rows) {
    const prev = prevById.get(row.id);
    if (prev && (prev.status === 'green' || prev.status === 'manual')) {
      results.push(prev);
    } else {
      results.push(notRunEntry(row.id));
      toRun.push(row);
    }
  }
  return { results, toRun };
}

/**
 * The exact final summary line's shape, factored out so run.mjs and this
 * file's own test agree on the wording byte-for-byte:
 *   '<green>/<total> green, <red> red, <blocked> blocked, <manual> manual,
 *    <not-run> not-run (M = <total> rows)'
 * `total` is `results.length`, not a separately-tracked counter -- the two
 * can never drift apart.
 */
export function summaryLine(results) {
  const total = results.length;
  const counts = { green: 0, red: 0, blocked: 0, manual: 0, 'not-run': 0 };
  for (const entry of results) {
    if (Object.prototype.hasOwnProperty.call(counts, entry.status)) {
      counts[entry.status] += 1;
    }
  }
  return (
    `${counts.green}/${total} green, ${counts.red} red, ${counts.blocked} blocked, ` +
    `${counts.manual} manual, ${counts['not-run']} not-run (M = ${total} rows)`
  );
}

/** True when at least one entry is still 'not-run' -- run.mjs's own exit-code gate. */
export function hasNotRun(results) {
  return results.some((entry) => entry.status === 'not-run');
}
