# Cycle 11 — Epic 2 (Build eight of the nine tables) / AT-34-E2-003

- **Commit SHA:** PENDING (filled by follow-up commit after push, per prior-cycle convention — see AT-34-E2-001/002)
- **Files touched:** `docs/release/SD-34-book-completion/artifacts/epic-2-tables/table-build-rate.json` (new), this receipt, `docs/release/SD-34-book-completion/kanban.md`, `docs/release/SD-34-book-completion/progress.md`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "The cost of building an engine table has never been measured in this program, and Epic 5's pricing of `power` depends on it. **Evidence:** `artifacts/epic-2-tables/table-build-rate.json` — per table: wall time, lines changed, what dominated, and whether the kind's shape made it cheaper or dearer than the others. **A single blended average across eight tables is not the deliverable** — the spread is what makes a projection for `power` honest."

## What this cycle recorded

No production code changes. This is a measurement-recording cycle: it derives the real cost
figures for the 8 tables Epic 2 already built (`AT-34-E2-001` commit `052a9182bf`, `AT-34-E2-002`
commit `b7507f3817`) and writes them to `artifacts/epic-2-tables/table-build-rate.json`.

**Honest limit on "wall time," stated up front:** the 7 new-table kinds (`ability`, `template`,
`trait`, `deity`, `domain`, `skill`, `language`) were all built through **one shared generic
loader** (`load_simple_kind_table`, `src/rules_core/rules_tables/simple_kind_tables.rs`) in a
**single commit**. There were never 7 separate build sessions to time independently — inventing 7
distinct stopwatch readings for one shared implementation would fabricate precision that does not
exist (`AGENTS.md` rule 9: mark an estimate as an estimate, never state it as measured). What the
artifact actually contains:

1. **Measured, real, re-derivable:** whole-cycle wall time for `AT-34-E2-001` (1,359s /
   0:22:39, `git log --format=%ci -1 660129880d` → `git log --format=%ci -1 052a9182bf`) and
   `AT-34-E2-002` (779s / 0:12:59, `052a9182bf` → `b7507f3817`).
2. **Measured, real, re-derivable:** exact marginal lines added per kind, counted directly off
   the committed file (`SEVEN_KIND_DIRS` entry + macro-invocation lines + any kind-specific test
   or doc lines), and the shared-infrastructure line count (208 of 247 total file lines) that
   every kind reuses without modification.
3. **Explicitly labeled ESTIMATE:** per-table wall time, pro-rated from the measured marginal
   lines against the measured cycle wall time. Never presented as independently timed.

## The spread (the deliverable's actual point)

Marginal lines per kind range from **2** (`domain`, `skill`, `language` — a single one-line macro
invocation plus a one-line directory-table entry) to **12** (`trait` — same two lines, plus a
7-line dedicated regression test and 3 lines of module doc explaining the `trait_generic`
directory-name mismatch), a **6× spread**, with `companion` at **0** production lines (its table
predates this bundle — SD-29 — and its only cost this bundle is a 21-line fail-closed test).

**Finding, stated for Epic 5's benefit:** record count does not predict cost. `ability` (4,337
records) and `domain` (183 records) cost nearly the same (7 vs. 2 marginal lines) because both
reuse the shared loader unmodified. The real cost driver is **whether the kind's corpus
directory name matches its kind name** — `trait`'s mismatch (`trait_generic/` vs. `trait`) is what
made it dearer, not its record count. `power`'s true price for Epic 5 depends on whether
`ultimate_psionics`'s corpus shape matches this generic per-`(book, key)` loader or needs its own
directory/shape handling the way `trait` did.

## Figures + their re-derive commands (every one, with its denominator)

| Figure | Command | Denominator |
|---|---|---|
| `AT-34-E2-001` cycle wall time: 1,359s (0:22:39) | `git log --format=%ci -1 660129880d && git log --format=%ci -1 052a9182bf` | whole cycle (all 7 new tables + docs + receipt together — not decomposable per table) |
| `AT-34-E2-002` cycle wall time: 779s (0:12:59) | `git log --format=%ci -1 052a9182bf && git log --format=%ci -1 b7507f3817` | whole cycle (all 8 fail-closed proofs + docs + receipt together) |
| File total: 247 lines | `wc -l src/rules_core/rules_tables/simple_kind_tables.rs` | the whole new module |
| Shared infra: 208 lines | `247` minus the sum of the 7 kinds' marginal lines below (`39`) | lines used by every kind, built once |
| `ability`/`template`/`deity` marginal: 7 lines each | `grep -n '^\s*("' …` (dir entry) + manual count of each 6-line macro block (lines 214-219, 220-225, 227-232) | per-kind addition to the shared file |
| `trait` marginal: 12 lines | dir entry (line 45) + 1-line invocation (line 226) + 7-line dedicated test (lines 171-178) + 3 doc lines (15, 19, 41) | per-kind addition; the one kind with directory-mismatch handling |
| `domain`/`skill`/`language` marginal: 2 lines each | dir entry + 1-line macro invocation (lines 233, 234, 235) | per-kind addition; cheapest of the 7 |
| `companion` marginal (table): 0 lines | table is `rules_tables::companion_chassis`, unmodified this bundle | pre-existing, SD-29 |
| `companion` fail-closed proof: 21 lines | `git show b7507f3817 --stat -- src/rules_core/rules_tables/companion_chassis.rs` | this bundle's only companion-related cost |
| 8 of 8 tables have a `tables[]` entry in the artifact | `python3 -c "import json; print(len(json.load(open('docs/release/SD-34-book-completion/artifacts/epic-2-tables/table-build-rate.json'))['tables']))"` | of the 8 kinds Epic 2 builds (`technical-design.md §4`) |

## Row-count command output

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

8 of 8 required table entries present — criterion's evidence bar met on a row count, not a
self-assessment.

## Build scope verified

- `cargo test --locked --no-run` (workspace root, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e2-003`):
  exit 0, 600 test executables built, zero `error` lines in the log.
- `apps/desktop/src-tauri` (separate cargo workspace), `cargo test --locked --no-run`
  (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e2-003-desktop`): exit 0, one pre-existing `dead_code`
  warning (unrelated field on `SavedCharacterMutationOpDescriptor`), zero errors.
- Run at HEAD `b7507f38178e41b3962ef3161ee525e5ad9ee9b0` (this cycle adds no production code, so
  the build-scope run is a re-confirmation that the docs-only change did not regress anything,
  not a proof of new behavior).

## Sweep population

N/A — this cycle touched no `data/corpus/**` records. `corpus_literal_sweep`'s
examined-population is unchanged (no regeneration performed).

## Oracle pin

N/A — no figure in this artifact came from the pinned PCGen oracle corpus.

- **Status:** complete
- **Movement, four buckets:** instrument-correction (this cycle measures and records cost data for
  work already landed by AT-34-E2-001/002; it moves no unit across any bucket and clears no bucket
  itself — it feeds Epic 5's pricing instrument)
- **Notes:** The criterion's evidence bar ("per table: wall time, lines changed, what dominated,
  whether cheaper/dearer") is satisfied per-table for lines-changed, dominant-driver, and
  cheaper/dearer (all measured, all with commands). Wall time is satisfied at the cycle level
  (measured) and at the per-table level as an explicitly labeled estimate — not fabricated as an
  independently-timed figure, because it was never independently timed. If a future reviewer wants
  true independent per-table wall time, the only way to get it is to build `power`'s table as its
  own timed cycle in Epic 5 and compare against this estimate.
- **Next-cycle plan:** AT-34-E2-004 — run `completion_atlas.py --check` for both `core_rulebook`
  and `ultimate_campaign` and confirm bucket A is 0 for both, closing Epic 2.
