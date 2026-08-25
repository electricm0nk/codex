# Cycle AT-33-E6-001-suite-green — epic-6-closure / AT-33-E6-001 (Shortfall 1, remediation wave 7)

- **Files touched:** `scripts/observer/pf1e_dashboard_producer.py`,
  `scripts/tests/test_pf1e_dashboard_producer.py`,
  `src/rules_core/equipment_resolver.rs`,
  `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`, this receipt, `progress.md`,
  `kanban.md` (row 14 Notes only, status stays `complete`),
  `docs/retro/events/sd33-r7-suite-green.jsonl`.
- **Environment hazard (recorded, not acted on):** at start of this cycle the shared checkout at
  `/home/ubuntu/workspace/repos/codex` was 8 commits behind `origin/tranche/13` and carried ~154
  `git status --porcelain` entries this agent did not create, including a **staged revert** of
  the wave-6 corpus-extraction fix (139 corpus files, `src/bin/enrich_equipment_raw_tokens.rs`,
  and `progress.md` all reverted toward pre-`fbc945f198` content). Matches the identical hazard
  `AT-33-E6-001-attempt7` recorded. Per `AGENTS.md`'s "one writer per tree" rule, this cycle did
  not write there — all work ran in a fresh worktree
  (`.worktrees/sd33-r7-suite-green`, branch `sd33-r7-suite-green-work`) built from a clean
  `git fetch origin tranche/13` (`e6f3705b3e`, the same commit attempt 7 scanned).

## Criterion

`AT-33-E6-001`'s own Shortfall 1: `cargo test --locked --lib` must be green (`workflow-instruction.md`
§2.5, §6 step 3/4; `AGENTS.md` "Fix the source, not the symptom").

## Starting state (re-confirmed by execution, not taken from the dispatch brief on trust)

```
$ cargo test --locked --lib
test result: FAILED. 2832 passed; 4 failed; 14 ignored; 0 measured; 0 filtered out; finished in 28.58s

failures:
    rules_core::equipment_resolver::tests::catalog_rows_span_every_ingested_book_with_their_real_counts
    rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::a_subset_run_trips_the_population_mismatch_check
    rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::corpus_wide_scan_population_matches_the_closed_gate1_census
    rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census
```

Matches attempt 7's own figures exactly (2,832 of 2,836 pass, 4 of 2,836 fail).

## Group A — the unmapped `(ambiguous, unmeasurable)` doneness pair

### Diagnosis, re-verified by execution

```
$ python3 -c "import json,collections
u=json.load(open('docs/work-inventory.json'))['units']
c=collections.Counter((x.get('wiring_class'),x.get('status')) for x in u)
print('ambiguous+unmeasurable:', c[('ambiguous','unmeasurable')], 'of', len(u), 'work-inventory units')"
ambiguous+unmeasurable: 11 of 49438 work-inventory units
```

Traced the panic to source: `_doneness_verdict_uncapped`'s `ambiguous` branch explicitly lists
`("grounded", "text-complete", "ingested-magnitude", "literal-verified", "fixture-verified")` and
`raise ValueError` on anything else — `unmeasurable` was never added to that list. But the REAL
root cause sits one branch earlier: the function has a **checked-first** section (before any
`wiring_class` branch, deliberately, per its own doc comment — "an `unknown` status cannot be
measured against any bar, classifiable or not") that special-cases `status == "unknown"` →
`DONENESS_UNMEASURABLE`. `AT-33-E4-002` (`00ca087775`, this same branch's commit message: "the
remaining 318 genuinely irreducible units keep their disposition unchanged, renamed
`unknown` → `unmeasurable`... so the status string itself stops reading as 'nobody looked'")
renamed that STATUS_VOCABULARY word everywhere **except this one checked-first call site**.
Confirmed the rename is real and complete in the authoritative source:

```
$ awk '/const STATUS_VOCABULARY/,/^\];/' src/bin/v06_work_inventory.rs | grep -E '^\s*"[a-z-]+",$'
"grounded" "literal-verified" "fixture-verified" "ingested-magnitude" "text-complete"
"deferred-with-reason" "not-ingested" "not-started" "unmeasurable"
```

No `"unknown"` anywhere in the authoritative vocabulary or in any real unit
(`docs/work-inventory.json`'s own status-value census has zero `"unknown"` entries). Cross-tab of
every real `(wiring_class, status)` pair confirms `unmeasurable` occurs under exactly two
wiring classes:

```
('ambiguous', 'unmeasurable') 11
('display', 'unmeasurable') 310
```

`display`'s branch has a catch-all (`return ... else DONENESS_IN_PROGRESS`), so it never raised —
but it was silently returning `in-progress` for these 310 units instead of the honest
`unmeasurable`, the same defect in a non-crashing shape.

### Conclusion: **legitimate pair needing a mapping entry — not a misclassification**

The 11 units are not mis-classified; `ambiguous` + `unmeasurable` is exactly what Epic 4's own
fallback-widening (`AT-33-E4-002`) legitimately produces for a unit whose wiring class the
consumer-side determinator could not resolve AND whose magnitude the corpus itself cannot supply
evidence for. Confirmed no `docs/work-inventory.json` edit is warranted: the file was not touched,
only the mapper.

### Fix

`scripts/observer/pf1e_dashboard_producer.py`'s checked-first branch now accepts both spellings:

```python
if status in ("unknown", "unmeasurable"):
    return DONENESS_UNMEASURABLE
```

`"unknown"` is kept alongside `"unmeasurable"` rather than replaced outright — an older, already-
generated `work-inventory.json` snapshot from before the rename legitimately still carries the old
word, and the function's whole design ("raise on a pair with no rule, never guess") argues for
recognizing a documented legacy spelling explicitly rather than assuming every caller regenerated
today. Comments at both this branch and the constant's own declaration (~line 3702) updated to
name the rename and cite `AT-33-E4-002`.

**Did NOT** make the producer swallow unmapped pairs generally (the fail-closed `raise ValueError`
for every other genuinely-unmapped pair is untouched — confirmed by `test_unmapped_cell_raises`,
still green) and did NOT edit `docs/work-inventory.json`.

### Self-test coverage was itself stale — closed in the same cycle

`scripts/tests/test_pf1e_dashboard_producer.py`'s `DonenessVerdictGridTest` grids over
`WIRING_CLASS_VALUES x STATUS_WORDS` and asserts nothing lands in `doneness_unmapped` — its own
docstring claims this is "kept in sync BY HAND with `STATUS_VOCABULARY`". It wasn't: `STATUS_WORDS`
still listed `"unknown"`, the word the real generator stopped emitting at `AT-33-E4-002`, so this
"full grid" test had gone blind to the real vocabulary word and could not have caught this defect.
Fixed: `STATUS_WORDS`'s `"unknown"` entry replaced with `"unmeasurable"` (now truly matches
`STATUS_VOCABULARY`), plus two new targeted tests — `test_ambiguous_unmeasurable_is_unmeasurable`
(the real, live cell) and `test_unknown_is_still_accepted_as_unmeasurables_legacy_spelling` (grids
over all 5 `WIRING_CLASS_VALUES`, asserting `"unknown"` and `"unmeasurable"` resolve identically —
proves the backward-compat branch, not just the new one).

```
$ python3 -m unittest scripts.tests.test_pf1e_dashboard_producer -v 2>&1 | tail -3
Ran 21 tests in 14.5s
OK
```

### A second, deeper defect the crash had been masking

Fixing the crash let `f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census`
run to completion for the first time — and its own hardcoded expectation (`6,308`) turned out
ALSO stale, exposed only once execution could reach it:

```
$ cargo test --locked --lib rules_core::pilot_compute::formula_interpreter_corpus_wide -- --nocapture
...
f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census ... FAILED
left: 6278
right: 6308
```

Root cause, re-derived: this test's `6,308` pin landed in `347e9d1a34` (2026-08-24 23:56:11), and
`00ca087775` (`AT-33-E4-002`, 2026-08-25 00:39:59 — the very next commit to touch
`docs/work-inventory.json` on this branch, 44 minutes later) regenerated that file (4,224 units
reclassified + 3,985 units of disclosed unrelated SD-32-engine drift). F1's population is built
from `coverage_ledger.py`'s `not_done_population()`, gated on `doneness_verdict(unit) != DONE` over
`docs/work-inventory.json` — the same file Epic 4 regenerated. Re-derived fresh with the exact
command the test's own doc comment already names:

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json | grep "^  F1"
  F1      6278  Flat-constant magnitude (bare literal)
```

Matches the Rust test's own live `report.families["F1"].population` exactly — both walk the
identical `not_done_population()` gate, so they cannot honestly disagree. This move is **not**
sensitive to which non-`done` doneness word Group A's fix chose for the 11
`(ambiguous, unmeasurable)` units: `not_done_population()` only tests `verdict != DONE`, never
which specific non-`done` verdict a unit carries. 6,308 − 6,278 = 30 units moved off the
F1-eligible population by the regen — a real content shift, not a defect this cycle introduced.
Test's pin retargeted to `6_278`, with the derivation chain and both commit SHAs recorded in the
test's own doc comment.

## Group B — the stale `equipment_resolver.rs` catalog count

### Diagnosis, re-verified by execution

```
$ cargo test --locked --lib rules_core::equipment_resolver::tests::catalog_rows_span_every_ingested_book_with_their_real_counts
left: 8119
right: 8100
```

Traced to source: `equipment_catalog_rows()` chains `hand_authored_equipment_rows()` (asserted
`6_146` two lines above the failing assertion — that assertion PASSES, hand-authored is unchanged)
with `equipment_gap_tables::equipment_gap_rows()`. The generated gap table states its own total in
its file header:

```
$ head -25 src/rules_core/rules_tables/equipment_gap_tables.rs | grep "^//! Total:"
//! Total: 1973 rows.
```

`6,146 + 1,973 = 8,119` — matches the live `rows.len()` exactly.

### Attribution: inherited from the `tranche/13` cut, NOT caused by wave 6's corpus regeneration

```
$ git ls-tree -r --name-only f652db7ac7 -- data/corpus | grep -E '/(equipment|equipment_modifier)/.*\.json$' | wc -l
7808
$ git ls-tree -r --name-only HEAD -- data/corpus | grep -E '/(equipment|equipment_modifier)/.*\.json$' | wc -l
7808
$ git show f652db7ac7:src/rules_core/rules_tables/equipment_gap_tables.rs | grep '^//! Total:'
//! Total: 1973 rows.
$ git log f652db7ac7..HEAD -- src/rules_core/rules_tables/equipment_gap_tables.rs
(empty)
$ git show f652db7ac7:src/rules_core/equipment_resolver.rs | grep -n 'assert_eq!(rows.len()'
863:        assert_eq!(rows.len(), 8_100);
```

The generated table already said "1973 rows" AT the `tranche/13` cut commit itself, and no SD-33
commit — including wave 6's `data/corpus/**` regeneration (137 files, all `modify`, 7,808 records
unchanged, confirmed above) — has ever touched `equipment_gap_tables.rs`. The `8,100` pin was
**already stale before SD-33 began**: an untraced drift from whatever cycle last ran
`gen_equipment_gap_tables` before the cut, the identical shape to the `1953→1954` drift this same
test's own comment two paragraphs above already documents from an earlier cycle.

### Independent cross-confirmation (two other tables, computed differently, already agree)

- `tests/equipment_gap_tables.rs`'s own `EXPECTED_PER_BOOK` sum: `total, 1973` (already passing,
  untouched by this cycle).
- `apps/desktop/src-tauri/src/equipment_catalog.rs` (a **separate** cargo workspace, built via its
  own `build_equipment_catalog()`, not this file's `equipment_catalog_rows()`): already asserts
  `assert_eq!(response.entries.len(), 8119)`, landed by SD-32's own `sd32-desktop-count-resweep`
  (`beginner_box` ingestion, +19) — this crate's copy of the same figure was already correct;
  only `src/rules_core/equipment_resolver.rs`'s copy had never been swept.

Three independently-computed tables (`equipment_gap_tables.rs`'s generated header, its own test's
`EXPECTED_PER_BOOK` sum, and the desktop crate's `build_equipment_catalog()`) all agree on 1,973 /
8,119. Only `equipment_resolver.rs`'s pinned assertion disagreed.

### Fix

`assert_eq!(rows.len(), 8_100)` → `assert_eq!(rows.len(), 8_119)`, with the full derivation chain
(both commands above, both cross-confirming tables, the cut-commit attribution) recorded in the
test's own doc comment — never a bare number swap.

## The count-change sweep

`8100`/`8,100` and `8119`/`8,119` grepped recursively across `tests/`, `src/`, `apps/`, `scripts/`:

```
$ grep -rn --include='*.rs' --include='*.py' --include='*.ts' --include='*.tsx' --include='*.js' --include='*.json' -E '8[,_]?100\b' tests/ src/ apps/ scripts/
```

Six unrelated hits (`cycle-2026-07-15T8100` timestamp fragments in five `tests/sd1*`/`sd18_*`
doc comments and one `src/rules_core/support_state_matrix.rs` doc comment — a date-derived token,
not this count) plus this cycle's own two edited lines (the equipment_resolver.rs comment
explaining the old number). One real prior occurrence,
`apps/desktop/src-tauri/src/equipment_catalog.rs:992` ("8025 -> 8100"), is itself historical prose
inside a doc comment describing an OLDER intermediate value in that file's own derivation chain —
that file's live assertion is already `8119` (confirmed above), so nothing there needs to move.

```
$ grep -rn --include='*.rs' --include='*.py' --include='*.ts' --include='*.tsx' --include='*.js' --include='*.json' -E '8[,_]?119\b' tests/ src/ apps/ scripts/
```

Two hits before this cycle (`apps/desktop/src-tauri/src/equipment_catalog.rs:1000,1005` — the
already-correct desktop assertion and its doc comment) plus this cycle's own two new lines. **No
other file needed a matching update** — the desktop crate was already correct; only
`equipment_resolver.rs` was stale. `6308`/`6,308` and `6278`/`6,278` swept the same way: every
`6,308` hit outside my own edited file is inside `docs/release/SD-33-.../artifacts/epic-3-engine-coverage/`
or `progress.md` — closed historical cycle receipts and log entries describing what was true when
Epic 3 landed them (2026-08-24, before the Epic 4 regen) — correctly left as historical record,
not live assertions; none of them is read by any test or `verify.sh` stage.

## Finish line

```
$ cargo test --locked --lib
test result: ok. 2836 passed; 0 failed; 14 ignored; 0 measured; 0 filtered out; finished in 52.25s
```

**2,836 of 2,836 executed lib tests pass; 0 fail; 14 ignored** (unchanged count, pre-existing
ignores unrelated to this cycle).

### Workspace suite (`cargo test --locked`, root workspace, all targets)

```
error[E0609]: no field `affects` on type `&WeaponEnhancementBonus`
  --> tests/sd20_equipment_equipmods.rs:94:22
error[E0609]: no field `bonus` on type `&WeaponEnhancementBonus`
  --> tests/sd20_equipment_equipmods.rs:95,111:22
error[E0609]: no field `affects` on type `&WeaponEnhancementBonus`
  --> tests/sd20_equipment_equipmods.rs:110:22
error: could not compile `codex` (test "sd20_equipment_equipmods") due to 4 previous errors
```

**RED, for a reason unrelated to this cycle's diff.** `WeaponEnhancementBonus`
(`src/rules_core/equipment_effects/equipmods.rs`) was split into independent `tohit_bonus` /
`damage_bonus` fields by `7d439876b7` (`AT-33-E5-finalize-wave6`, the closed-and-not-to-be-
re-litigated Epic 5 fix) — but `tests/sd20_equipment_equipmods.rs` (a root-workspace integration
test, last touched at `eeaa876ddf`, long before wave 6) still references the old flat
`bonus`/`affects` fields and was never swept. Confirmed unrelated to this cycle's own diff:

```
$ git diff --stat HEAD -- tests/sd20_equipment_equipmods.rs src/rules_core/equipment_effects/equipmods.rs
(empty -- neither file touched by this cycle)
```

This is the exact `cargo build --lib` vs `cargo test` gap `AGENTS.md`'s Concurrency section names
("one broken bin meant 0 of 502 suites ran while the phase reported COMPLETE") — `cargo test`
fails to compile the whole binary before running ANY suite, so **0 of however-many integration
suites ran** in this invocation. Per this cycle's own dispatch ("if a suite outside your change is
red for an unrelated reason, report it with evidence rather than fixing it"), **not fixed** —
named here with file, lines, commit, and the `git diff --stat` proof it is untouched by this
cycle's own scope.

### `apps/desktop/src-tauri` (separate cargo workspace, tested explicitly)

```
$ cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd33-sd33-r7-suite-green-desktop cargo test --locked
test result: ok. 548 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 86.57s
```

**GREEN.** One test binary (this crate's lib target); no other targets. Zero `FAILED`/`error:`
lines anywhere in the full build+test log.

### `scripts/verify.sh` (full, all stages) — ran to completion, polled inside this turn

```
$ scripts/verify.sh --only denominator-gate
    PASS  denominator-gate  (files_checked=54 violations=0)
RESULT: PASS
```

Full run: **32 passed, 5 failed** (`RESULT: FAIL`). `denominator-gate` (the one stage this
cycle's own brief makes a hard requirement) **PASS**, `files_checked=55 violations=0` inside the
full run (55, not 54 — this receipt's own file joined the scanned set). `root-lib` (`cargo test
--locked --lib`, the same command this cycle's own criterion names) **PASS, 2836 passed** — this
independently reconfirms, through `verify.sh`'s own harness, the exact same green result reported
above under "Finish line".

**All 5 failures traced to source and confirmed unrelated to this cycle's own 4-file diff** (none
touches `scripts/observer/pf1e_dashboard_producer.py`, `scripts/tests/test_pf1e_dashboard_producer.py`,
`src/rules_core/equipment_resolver.rs`, or `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`):

| Stage | Root cause | Attribution |
|---|---|---|
| `root-full` (`cargo test --locked --no-fail-fast`) | `tests/sd20_equipment_equipmods.rs:94-111` references `WeaponEnhancementBonus` fields (`bonus`/`affects`) that `7d439876b7` (wave 6, closed Epic 5 work) replaced with `tohit_bonus`/`damage_bonus` | Already reported above under "Workspace suite"; confirmed via `git diff --stat` empty for both files |
| `clippy` (root half) | `error: invisible character detected` — a `\u{AD}` soft hyphen inside corpus-derived spell description text at `src/rules_core/rules_tables/monster_codex/spell_list.rs:83`, `#[deny(clippy::invisible_characters)]` | Pre-existing generated-table content; this cycle touched no `rules_tables/` file |
| `clippy` (desktop half) | `20 warnings exceeds recorded ceiling 7` (`clippy-desktop.log`) | `apps/desktop/src-tauri` crate; this cycle touched zero files under `apps/` |
| `corpus-sweep` (`corpus_literal_sweep`) | 105 findings across 10 records — `token not byte-present in corpus token closure` on records including `ultimate_equipment:{blade_of_the_rising_sun,blade_of_the_sword_saint,hammer_polarity,hellscourge}` and `inner_sea_gods:fugitive_finder` | All 5 named records were last touched by `fbc945f198` (wave 6's corpus-extraction-fix, `AT-33-E5-003`, inside the closed Epic-5/`data/corpus/**` territory this cycle's dispatch explicitly forbids touching — `git log --oneline -3 -- <each file>` confirms). A real, newly-surfaced gap in that already-closed cycle's regeneration, not caused by or fixable within this cycle's own granted scope (`src/rules_core/equipment_resolver.rs`, `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`, and the Python doneness table — never `data/corpus/**`) |
| `frontend-test` (`npm test`, `apps/desktop`) | `97/100` TypeScript test files pass | `apps/desktop` frontend; this cycle touched zero `.ts`/`.tsx` files |

Per this cycle's own dispatch ("if a suite outside your change is red for an unrelated reason,
report it with evidence rather than fixing it"): **none of the 5 fixed**, all 5 named here with
file, line, and commit attribution for a future cycle. The `corpus-sweep` finding is new to this
receipt (not named in `AT-33-E6-001-attempt7`, which checked `raw_tokens` length/license/PI
preservation on the same 137 files but did not run `corpus_literal_sweep` against them) and is
flagged for Epic 5's own future owner, not acted on here.

### Epic 5, re-confirmed undisturbed

```
$ python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json; echo EXIT=$?
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
EXIT=0
```

No epic-5-reverification results file touched by this cycle (`git diff --stat HEAD -- docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/` is empty).

## Movement, four buckets

- **Closure 0** — no `docs/work-inventory.json` unit's `status` field changed.
- **Reclassification 0** — no unit moved kind or population.
- **Reachability 0** — no new unit rowed against the oracle (out of this cycle's scope).
- **Instrument-correction 3** — `pf1e_dashboard_producer.py`'s doneness table (1 missing mapping
  entry, closed), `equipment_resolver.rs`'s pinned catalog count (1 stale assertion, retargeted to
  the proven total), `formula_interpreter_corpus_wide.rs`'s pinned F1 population (1 stale
  assertion, retargeted to the proven total, discovered only because the first fix let execution
  reach it).

## Identifier / wired-integration audits

```
$ BASE_BRANCH=$(git merge-base HEAD origin/develop)
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- scripts/observer/pf1e_dashboard_producer.py scripts/tests/test_pf1e_dashboard_producer.py src/rules_core/equipment_resolver.rs src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo OK_NO_BUNDLE_TAGS
OK_NO_BUNDLE_TAGS
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- scripts/observer/pf1e_dashboard_producer.py scripts/tests/test_pf1e_dashboard_producer.py src/rules_core/equipment_resolver.rs src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo OK_NO_TOKENS
OK_NO_TOKENS
```

## Acceptance criterion

Verbatim, this cycle's own dispatch: "`cargo test --locked --lib` must be GREEN." Met: 2,836 of
2,836 pass, 0 fail, 14 ignored.

- **Status:** complete
- **Notes:** kanban row 14 stays `complete` (per dispatch instruction) — its own regeneration
  commit (`00ca087775`) is what introduced the 11-unit unmapped pair, and this cycle closes the
  gap the mapper had against it rather than reopening row 14 itself. The `tests/sd20_equipment_equipmods.rs`
  compile break is a genuinely separate, named defect for a future cycle to own (out of this
  cycle's granted scope, per its own dispatch instruction).
- **Next-cycle plan:** (1) `tests/sd20_equipment_equipmods.rs`'s stale `WeaponEnhancementBonus`
  field references (`bonus`/`affects` → `tohit_bonus`/`damage_bonus`), blocking the full
  `cargo test --locked` workspace sweep at 0 of N suites run. (2) The 310 `(display, unmeasurable)`
  units now correctly read `unmeasurable` instead of the prior silent `in-progress` — a real
  content-neutral instrument correction this cycle's fix produced as a side effect; not
  independently re-verified against every downstream dashboard rollup that reads `doneness`
  totals, since none is in this cycle's own scope or covered by `cargo test --locked --lib`.
