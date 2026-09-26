// Row selection for run.mjs's --only / --from flags. Pure and DOM-free so it
// is unit-tested directly (rowSelection.test.mjs) without a running app.
//
// `--only` takes one row id or a comma-separated list, run in the order
// given. Every id must name a spec row: an unknown, empty or repeated id is an
// error, never a silently shorter run, because results.json's denominator is
// the ids the caller asked for (see resultsSkeleton.mjs).

export function parseOnlyIds(only) {
  const ids = String(only)
    .split(',')
    .map((id) => id.trim());
  const seen = new Set();
  for (const id of ids) {
    if (!id) {
      throw new Error(`--only ${only}: empty id in the list`);
    }
    if (seen.has(id)) {
      throw new Error(`--only ${only}: '${id}' is listed more than once`);
    }
    seen.add(id);
  }
  return ids;
}

export function selectRows(spec, args) {
  if (args.only) {
    return parseOnlyIds(args.only).map((id) => {
      const row = spec.byId.get(id);
      if (!row) {
        throw new Error(`--only ${id}: no such row id in spec.json`);
      }
      return row;
    });
  }
  let rows = spec.rows;
  if (args.from) {
    const startIndex = rows.findIndex((row) => row.id === args.from);
    if (startIndex === -1) {
      throw new Error(`--from ${args.from}: no such row id in spec.json`);
    }
    rows = rows.slice(startIndex);
  }
  return rows;
}
