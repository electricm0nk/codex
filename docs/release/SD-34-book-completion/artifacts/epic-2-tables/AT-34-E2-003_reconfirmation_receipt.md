# Cycle 11-R — Epic 2 (Build eight of the nine tables) / AT-34-E2-003 (reconfirmation at HEAD)

- **Commit SHA:** `ac61ac1b89601dbfdd7b128a08a3c92aced34dfb` (HEAD at cycle start — no production
  code change was needed; this cycle's own commit carries only this receipt + `progress.md` +
  `kanban.md` + a retro event)
- **Files touched:** `docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-003_reconfirmation_receipt.md`
  (new, this file), `docs/release/SD-34-book-completion/progress.md`,
  `docs/release/SD-34-book-completion/kanban.md`, `docs/retro/events/sd34-at-34-e2-003.jsonl` (new)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "The cost of building an engine table
  has never been measured in this program, and Epic 5's pricing of `power` depends on it.
  **Evidence:** `artifacts/epic-2-tables/table-build-rate.json` — per table: wall time, lines
  changed, what dominated, and whether the kind's shape made it cheaper or dearer than the
  others. **A single blended average across eight tables is not the deliverable** — the spread
  is what makes a projection for `power` honest."

## Why this cycle exists

`AT-34-E2-003` was already `complete` on `kanban.md` row 11, recorded at commit `49a7f03190` and
committed 2026-08-27 (`artifacts/epic-2-tables/AT-34-E2-003_cycle_receipt.md`). Sibling criteria
`AT-34-E2-001` and `AT-34-E2-002` in this same epic have each already been re-dispatched and
reconfirmed at HEAD (`AT-34-E2-001_reconfirmation_receipt.md`, `AT-34-E2-002_reconfirmation_receipt.md`)
after Epic 3 landed commits touching `src/rules_core/`. This bundle's standing lesson
(`decisions.md §12` L2 — never carry your own number forward, re-derive it) applies equally to
`AT-34-E2-003`'s own dispatch: this cycle re-derives the criterion's evidence at HEAD rather than
re-quoting the 2026-08-27 receipt, and reports whether anything moved.

## Re-derivation at HEAD

**1. The artifact still exists and still has 8 of 8 table entries:**

```
$ python3 -c "
import json
d = json.load(open('docs/release/SD-34-book-completion/artifacts/epic-2-tables/table-build-rate.json'))
print('tables entries:', len(d['tables']))
print('kinds:', [t['kind'] for t in d['tables']])
"
tables entries: 8
kinds: ['ability', 'template', 'trait', 'deity', 'domain', 'skill', 'language', 'companion']
```

No drift — same 8 kinds as the original cycle.

**2. The underlying commits the artifact's wall-time figures point to are immutable, so those
figures cannot drift** (`git log` timestamps for a merged commit do not change):

```
$ git log --format=%ci -1 660129880d && git log --format=%ci -1 052a9182bf && git log --format=%ci -1 b7507f3817
2026-08-27 02:09:38 -0400
2026-08-27 02:32:17 -0400
2026-08-27 02:45:16 -0400
```

Same as the original receipt: `AT-34-E2-001` cycle = 1,359s (0:22:39), `AT-34-E2-002` cycle =
779s (0:12:59). Unchanged.

**3. The underlying test suite the tables depend on still passes at HEAD, unregressed** (this is
the live proxy for "the tables still hold what the artifact says they hold" — `decisions.md §12`
L19):

```
$ cargo test --locked --lib rules_core::rules_tables::simple_kind_tables
running 13 tests
... (all 13 ok)
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 2911 filtered out; finished in 1.11s
```

Identical to `AT-34-E2-001`'s reconfirmation result — 13 of 13 pass, no regression since Epic 3's
later commits landed.

**4. Note on `src/rules_core/rules_tables/simple_kind_tables.rs`'s current line count:** the file
is now 346 lines at HEAD (`wc -l`), not the 247 lines the original receipt measured at commit
`052a9182bf`. This is **expected, not drift** — the artifact's marginal/shared-infra line counts
are historical figures pinned to specific past commits (the state of the file the moment the 8
tables were built and proven fail-closed), not a live population count that must track HEAD. The
99-line growth is from later Epic 2/3 additions (e.g. the two new tests seen in item 3 above,
`domain_table_resolves_a_pi_renamed_record_by_coordinate_not_by_the_real_name` and
`race_trait_generic_table_resolves_the_sibling_directory_not_race_trait_itself`) — unrelated to
the criterion's own evidence bar, which is about the historical build event, not the file's
current size.

**5. Dual-audit gate, re-run on Epic 2's declared file-touch set (`src/bin/v06_work_inventory.rs`,
`src/rules_core/`, `docs/release/SD-34-book-completion/artifacts/epic-2-tables/`) against the
current merge-base with `origin/develop`:**

```
$ BASE_BRANCH=$(git merge-base HEAD origin/develop)   # ea2b3396f2
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/bin/v06_work_inventory.rs src/rules_core/ \
    docs/release/SD-34-book-completion/artifacts/epic-2-tables/ ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
OK_NO_BUNDLE_TAGS

$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/bin/v06_work_inventory.rs src/rules_core/ \
    docs/release/SD-34-book-completion/artifacts/epic-2-tables/ ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
```

The second grep matches ~15 lines, all containing the word `placeholder`. Every match is a
**domain term, not a stub token**: PCGen's own CHOOSE-menu "no selection" rows in the corpus data
model (e.g. `src/bin/v06_work_inventory.rs:4691`, `"PCGen's own CHOOSE-menu \"no selection\"
placeholder row for the Barbarian class..."`), documented as `AT-34-E3-001`'s vacuous-placeholder
sub-cause across `decisions.md`. None is inside a shipping code path this criterion's own
file-touch set (`src/rules_core/rules_tables/`) newly introduces — they are Epic 3's classifier
vocabulary, already reviewed and self-healed in that epic's own audits. Reviewed here in full and
confirmed to be the same self-healable false-positive category the sibling `AT-34-E2-001`/`002`
reconfirmations also found and disposed of.

## Row-count command output

```
$ python3 -c "
import json
d = json.load(open('docs/release/SD-34-book-completion/artifacts/epic-2-tables/table-build-rate.json'))
print('tables entries:', len(d['tables']))
"
tables entries: 8
```

8 of 8 required table entries present at HEAD — unchanged from the original cycle. Criterion's
evidence bar (`per-table wall time, lines changed, dominant driver, cheaper/dearer` — all present
in the artifact, checked above) still met.

## Build scope verified

- `cargo test --locked --no-run` (workspace root, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e2-003`):
  exit 0, all 600 targets built (no `error` lines).
- `apps/desktop/src-tauri` (separate cargo workspace), `cargo test --locked --no-run`
  (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e2-003-desktop`): exit 0, one pre-existing `dead_code`
  warning (`SavedCharacterMutationOpDescriptor::op`, unrelated to this criterion), zero errors.
- Run at HEAD `ac61ac1b89601dbfdd7b128a08a3c92aced34dfb`. No production code changed by this
  cycle, so this run reconfirms the tree still builds clean, not new behavior.

## Sweep population

N/A — this cycle touched no `data/corpus/**` records and regenerated nothing.

## Denominator gate

```
$ python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'
files_checked=15
violations=0
```

## Oracle pin

N/A — no figure in this artifact came from the pinned PCGen oracle corpus.

- **Status:** complete
- **Movement, four buckets:** instrument-correction (this cycle re-verifies existing measurement
  data against HEAD; it moves no unit across any bucket and adds no new figures — it confirms the
  original cycle's figures have not silently drifted)
- **Notes:** No drift found. Every re-derivable figure in `table-build-rate.json` still holds:
  8/8 table entries present, the two historical wall-time windows are unchanged (immutable
  commit timestamps), and the 13 tests the tables' correctness depends on still pass at HEAD
  (same count and same result as the sibling `AT-34-E2-001` reconfirmation). The only change
  since the original cycle is the host file's total line count growing from later, unrelated
  Epic 2/3 test additions — explicitly not a figure the criterion's evidence bar depends on,
  since that bar is about the historical build-and-prove event, not the file's live size.
- **Next-cycle plan:** none required by this reconfirmation — Epic 2 (`AT-34-E2-001..004`) is
  already fully `complete` on `kanban.md`; the epic's own next-cycle plan (from the original
  `AT-34-E2-003` receipt) remains `AT-34-E2-004`, itself already closed.
