---
canonical: true
owner: god-emporer
bundle_id: SD-35
status: planning-ready — not launched (launch-readiness audit passed 2026-09-08 00:10Z)
date: 2026-09-07
---

# SD-35 Progress

Live cycle-by-cycle record. Cycles **prepend** their entry (newest first) and update
`kanban.md` in the same commit, via `workflow-instruction.md §5`'s retry protocol.

Every cycle entry carries, verbatim from its receipt: the `cycle_scope_gate.py --min 500` line,
the `--receipt` rows (closed / relabeled / rust_lines_changed / ratio / builds_recorded /
pcgen_live_files), and the refused-token remainder. **An entry without the scope-gate line is a
process defect** recorded by the epic wrap-up.

## Open blockers

*(none — an entry here pauses the bundle and is a request for an operator ruling;
`decisions.md §6`)*

## Status matrix

| Epic | Criteria | Complete | In progress | Not started |
|---|---:|---:|---:|---:|
| 1 — Tax cut | 6 | 6 | 0 | 0 |
| 2 — Sheet rule | 5 | 5 | 0 | 0 |
| 3 — Place and surface | 4 | 0 | 1 | 3 |
| 4 — Resolve and verify | 3 | 0 | 0 | 3 |
| 5 — Residues | 5 | 0 | 0 | 5 |
| 6 — PCGen exit | 4 | 0 | 0 | 4 |
| 7 — Closure | 3 | 0 | 0 | 3 |
| **Total** | **30** | **11** | **1** | **18** |

Corpus at the `tranche/15` cut (2026-09-07, `4c6c57eb9f`, identical to authoring at `5f6b18f4e3`):
`DONE=26123 of 49438`; non-DONE 23,315 of 49,438. Live-side PCGen residue at authoring: 78 files by coarse grep
(`content-unit-inventory.md §6`); the exact baseline is AT-35-E1-005's first run. Both
re-measured at the cut by the launch-readiness audit.

## Cycle log

### 2026-09-08 — AT-35-E2-004 cycle 2 — `token-coverage-ledger` — **complete** (re-dispatch of a closed criterion; re-verified at HEAD, and it corrected one stale figure in its own cycle-1 receipt)

AT-35-E2-004 was dispatched a second time after cycle 1 had landed (`344f18d1e1`, board row 10
already `complete`). The criterion was at zero on arrival, so this cycle re-derived every clause of
its `Evidence:` sentence at HEAD `9f1b27dcdf` rather than re-doing work, and **changed no code, no
data and no script** — `rust_lines_changed=0`, nothing outside `docs/` written. Receipt:
`artifacts/epic-2-sheet-rule/AT-35-E2-004_cycle2_receipt.md`.

`SCOPE_GATE: EXEMPT (ledger-building cycle — closes zero units by design)` (`decisions.md §2`; the
pass that moves units is AT-35-E2-005, which has since run — and the criterion is additionally
already at zero). Receipt rows: `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a
builds_recorded=2 pcgen_live_files=260`. Residue `live_files=260 live_hits=12736 baseline_files=260
baseline_hits=12736 verdict=PASS`, identical at start and end, to cycle 1's, and to all three
preceding re-dispatches'.

Evidence re-derived, clause by clause. `python3 scripts/token_coverage.py --check` exits 0 in 2.47 s
with **`non_done=1404 tokened=1399 token_less=5 refused=1810 refused_non_done=659 token_types=231
shapes=81 verdict=PASS`** and all six sub-checks `ok=True` (`population`, `double_count`,
`coverage` with `uncovered=0`, `refused_set` with `census_refused=1810 refused_json=1810
union_over_token_types=1810`, `shape_totals`, `partition`) — the two sums the criterion names hold
exactly. The **RED→GREEN on a planted double-count** is `RedGreen.test_a_planted_duplicate_census_
entry_fails_the_check` and `..._duplicate_token_on_one_record_...` (each asserts
`verdict=FAIL_DOUBLE_COUNT` exit 1, then `verdict=PASS` exit 0 with the plant removed); the file
runs `14 tests … OK`. The **`verify.sh` wiring** is live in both `ALL_STAGES` and `QUICK_STAGES`
(48 stages, unchanged): `--only token-coverage-selftest --only token-coverage` → `RESULT: PASS`.
`--check` rewrote nothing — the ledger and `data/sheet_rules/_tokens.json` (49,438 entries,
14,631,801 bytes) are byte-identical to cycle 1's.

**The one correction** (`1788889623100-at-35-e2-004-c2ecac`): cycle 1 recorded **44** of 231 token
types carrying ≥500 non-DONE units, the `decisions.md §2` batch floor. At HEAD it is **7** — `TYPE`
1126, `CATEGORY` 1002, the `SOURCEPAGE`-family 913, `KEY` 904, `DESC` 605, `ABILITY` 564,
`BONUS:VAR` 502 — because AT-35-E2-005 dropped non-DONE from 23,315 to 1,404; only 164 of 231 types
carry any non-DONE unit at all. Cycle 1's figure was right at its tree; it is the shape of stale
scoping figure `AGENTS.md` rule 9 exists for, so **the ledger is the only admissible source for a
batch scope from here on, never a prior receipt's list.** The counterpart finding is that the
**refused set is invariant**: the remainder by token type is identical to cycle 1's, type for type
and count for count — 49 types, sum with multiplicity 850, over the **same** 659 distinct non-DONE
refused units (now of 1,404 non-DONE, not 23,315). AT-35-E2-005 closed 21,911 units and **not one
came out of the refused set**, so the ledger's remainder is a standing work list, not a decaying
one. `unmapped_token_types=24`, `shapes=81`, `token_types=231` all unchanged.

Widest build scope: `--no-run` exit 0 (2 min 48 s, cold target dir), `--lib` **3217 passed / 0
failed / 14 ignored** (unchanged from AT-35-E2-003 cycle 2), `--no-fail-fast` **412 binaries, 412
ok, 8,721 passed, 0 failed, 67 ignored** (`FULL_EXIT=0`, derived twice and agreeing), clippy on the
lib, `v06_work_inventory`, `sheet_rule_convert` and the convert gate **0 warnings**; `apps/`
untouched, so desktop and frontend stay at epic cadence. Whole chain 46 min 42 s, every step exit 0.
Gates: `sheet_rule_convert -- --check` → `records=49438 converted=47628 refused=1810 rules=66514
var_tables=5081 verdict=PASS (110.6s)`; literal scan over `data/sheet_rules/` **0** files;
`completion_atlas.py` `population=49438 buckets=10 unclassified=0 overlap=0 …
done_evidence_violations=0 stale_derived_at=False citation_failures=0`;
`shape_engine_boundary.py` `magnitude_bearing=26396 not_held_by_engine=363`;
`missing_engine_tables.py` `population=1 citation_failures=0`; `denominator_gate.py`
`files_checked=43 violations=0`; `verify.sh --only pi-sweep` `RESULT: PASS`; `corpus_literal_sweep`
skipped (no corpus record changed). Audits on the final diff: `OK_NO_BUNDLE_TAGS`;
wired-integration **9 diff lines / 4 files**, every one rulebook prose or PCGen's own editorial
wording in `data/sheet_rules/**` (3) and `docs/work-inventory.json` (6, the three
`empty_selection_standard_*` `reason` fields) — none in `src/`, `scripts/`, `apps/` or `tests/`,
matching AT-35-E2-003 cycle 2's accounting exactly. **Refused tokens: the ledger's 49-type
remainder, unchanged from cycle 1** (full list in the receipt).

### 2026-09-08 — AT-35-E2-003 cycle 2 — `sheet-complete-status` — **complete** (re-dispatch of a closed criterion; re-verified at HEAD, and it corrected one stale figure in its own cycle-1 receipt)

AT-35-E2-003 was dispatched a second time after cycle 1 had landed (`a81c2a005c`, board row 9
already `complete`). The criterion was at zero on arrival, so this cycle re-derived every clause of
its `Evidence:` sentence at HEAD `8274054e34` rather than re-doing work, and **changed no code, no
data and no script** — `rust_lines_changed=0`, nothing outside `docs/` written. Receipt:
`artifacts/epic-2-sheet-rule/AT-35-E2-003_cycle2_receipt.md`.

`SCOPE_GATE: EXEMPT (status-vocabulary cycle — closes zero units by design)` (`decisions.md §2`;
the pass that moves units is AT-35-E2-005, which has since run — and the criterion is additionally
already at zero). Receipt rows: `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a
builds_recorded=1 pcgen_live_files=260`. Residue `live_files=260 live_hits=12736 baseline_files=260
baseline_hits=12736 verdict=PASS`, identical at start and end, to cycle 1's, and to the two
preceding re-dispatches'.

Evidence re-derived, clause by clause. **The census clause now holds as a strict superset, which
cycle 1 could not yet show** — this cycle's one correction (`1788886876100-at-35-e2-003-d79ecd`).
Cycle 1 recorded before=6 / after=7 with one *before-only* file
(`src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`, which named the oracle words in
a pin narrative but not yet `sheet-complete`). At HEAD:
`grep -rln "oracle-unverifiable" src scripts apps tests | wc -l` → **6**;
`grep -rln "sheet-complete" src scripts apps tests | wc -l` → **10**; before-only files **0**
(`comm -23` over the sorted sets). The 4 after-only files are the status's own RED→GREEN tests plus
the two consumers AT-35-E2-004/005 added. The only vocabulary reader outside the after-set is
`scripts/reachability_audit.py`, which reads `status_vocabulary` from the document instead of
hard-coding it; its run reports `unmeasurable_unknown_status_units: 0`. **Cycle 1's projection was
exact**: it projected the rung would move **21,911** units at the next regeneration, and
`docs/work-inventory.json` now carries exactly **21,911** `sheet-complete` units, by rendered form
`words 16614 / number 4400 / dice 897` — a 12-word vocabulary carrying the `technical-design.md §3`
meaning verbatim. The atlas clause, verbatim: `completion_atlas.py --check` → `population=49438
buckets=10 unclassified=0 overlap=0` … `done_evidence_violations=0 missing_clearing_mechanisms=0
stale_derived_at=False citation_failures=0`, with the `sheet_rule_rendered:<number|dice|words>`
DONE-evidence rule at `completion_atlas.py:116,307-308,390-404`.

Widest build scope: `--no-run` exit 0 (3 min 04 s, cold target dir, max RSS 2,384,876 kB),
`--lib` **3217 passed / 0 failed / 14 ignored**, `--no-fail-fast` **412 binaries, 412 ok, 8,721
passed, 0 failed, 67 ignored** (`FULL_EXIT=0`, derived twice and agreeing), clippy on the lib and
`v06_work_inventory` **0 warnings**; `apps/` untouched, so desktop and frontend stay at epic
cadence. Gates: `sheet_rule_convert -- --check` → `records=49438 converted=47628 refused=1810
rules=66514 var_tables=5081 verdict=PASS`; literal scan over `data/sheet_rules/` **0** files;
`token_coverage.py --check` `non_done=1404 tokened=1399 token_less=5 refused=1810
refused_non_done=659 token_types=231 shapes=81 verdict=PASS`, all six sub-checks `ok=True`;
`shape_engine_boundary.py` `magnitude_bearing=26396 not_held_by_engine=363`;
`missing_engine_tables.py` `population=1 citation_failures=0`; `denominator_gate.py`
`files_checked=42 violations=0` at verification time, `files_checked=43 violations=0` once this receipt was written; `verify.sh --only pi-sweep` `RESULT: PASS`; python consumer suites
`Ran 146 tests … OK`. Audits on the final diff: `OK_NO_BUNDLE_TAGS`; wired-integration **9 diff
lines / 7 sites**, every one rulebook prose or the source's own editorial wording in
`data/sheet_rules/**` and `docs/work-inventory.json`, all previously attributed by AT-35-E2-002
cycle 2 — none in `src/`, `scripts/` or `apps/`. **Refused tokens: none.**

### 2026-09-08 — AT-35-E2-002 cycle 2 — `live-evaluator-and-sheet-section` — **complete** (re-dispatch of a closed criterion; re-verified at HEAD, and it corrected one stale figure in its own cycle-1 receipt)

AT-35-E2-002 was dispatched a second time after cycle 1 had landed (`909bb0837c`, board row 8
already `complete`). The criterion was at zero on arrival, so this cycle re-derived every clause of
its `Evidence:` sentence at HEAD `bb785e568d` rather than re-doing work, and **changed no code, no
data and no script** — `rust_lines_changed=0`, nothing outside `docs/` written. Receipt:
`artifacts/epic-2-sheet-rule/AT-35-E2-002_cycle2_receipt.md`.

`SCOPE_GATE: EXEMPT (live-evaluator + sheet-section cycle — closes zero units by design)`
(`decisions.md §2`; AT-35-E2-003 is the status that moves units and AT-35-E2-005 the pass that
moves them — and the criterion is additionally already at zero). Receipt rows: `closed=0
relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260`. Residue
`live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`, identical at
start and end, to cycle 1's, and to AT-35-E2-001 cycle 2's.

Evidence re-derived, clause by clause. The evaluator is a match over the enum with **zero** PCGen
surface — `grep -cE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|raw_tokens|PcgenFormulaEvaluator|render_pcgen_desc'
src/rules_core/sheet_rule.rs` → `0` over its 2,359 lines. `cargo test --locked --lib -j 6
sheet_rule` → **31 passed / 0 failed**, carrying the three value-form proofs the criterion names.
The section, its grouping helpers and its mount are at `CharacterSheet.tsx:2041,2051,2064,2084,2087,2324`;
the `Not computed` lane's `noticeHasSheetRule` at `classFeaturesModel.ts:321,353`; the IPC reach
test at `reach_gate.rs:8195`. Frontend: `node scripts/run-tests.mjs` → `101/101 test files passed`
with `rulesAndFeaturesSection: 19 per-kind tests + 5 section tests passed` — the criterion's
"19 frontend tests, one per kind" clause, verbatim — and `tsc --noEmit` exit 0. Desktop crate,
tested explicitly because `apps/` is in scope: `574 passed / 0 failed`, clippy **0 warnings**.
Widest build scope: `--no-run` exit 0 (2 min 47 s warm), `--lib` **3217 passed / 0 failed**,
`--no-fail-fast` **412 binaries, 412 ok, 8,721 passed, 0 failed**, lib clippy 0 warnings. Gates:
`completion_atlas.py` `done_evidence_violations=0 citation_failures=0`; `token_coverage.py`
`non_done=1404 refused_non_done=659 token_types=231 shapes=81 verdict=PASS`;
`shape_engine_boundary.py` `magnitude_bearing=26396 not_held_by_engine=363`;
`missing_engine_tables.py` `population=1 citation_failures=0`; `denominator_gate.py`
`files_checked=41 violations=0`; `verify.sh --only pi-sweep` `RESULT: PASS`;
`sheet_rule_convert -- --check` `records=49438 converted=47628 refused=1810 verdict=PASS`, summing
exactly; `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → **0**.

**One correction, to this criterion's own cycle-1 receipt** (`1788883047652-at-35-e2-002-8f4a35`):
the fixture Human Fighter 1 renders **13 lines**, not the 45 cycle 1 recorded — across the *same*
five kinds, and with all 19 per-kind evaluation censuses byte-identical to cycle 1's. The cause is
named: AT-35-E2-005 cycle 2 (`33deab007b`) made a `#bonusN` sibling print only when its own
`applies` includes, removing the 32 unconditionally-printed siblings cycle 1 counted — exactly the
shape cycle 1's own Discovery (4) had flagged as open. A downstream improvement to the criterion's
clause, not a regression. Refused tokens: **none**.

### 2026-09-08 — AT-35-E2-001 cycle 2 — `sheet-rule-converter` — **complete** (re-dispatch of a closed criterion; re-verified at HEAD, and it corrected its own cycle-1 audit row)

AT-35-E2-001 was dispatched a second time after cycle 1 had landed (`72ad0be010`, board row 7
already `complete`). The criterion was at zero on arrival, so this cycle re-derived every clause
of its `Evidence:` sentence at HEAD `4510517993` rather than re-doing work, and **changed no code,
no data and no script** — `rust_lines_changed=0`, nothing outside `docs/` written. Receipt:
`artifacts/epic-2-sheet-rule/AT-35-E2-001_cycle2_receipt.md`.

`SCOPE_GATE: EXEMPT (converter-building cycle — closes zero units by design; AT-35-E2-005 is the
pass that moves the population)` (`decisions.md §2`; the criterion is additionally already at
zero). Receipt rows: `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0
pcgen_live_files=260`. Residue `live_files=260 live_hits=12736 baseline_files=260
baseline_hits=12736 verdict=PASS`, identical at start and end and to cycle 1's.

Evidence re-derived, clause by clause: `cargo run --locked --release --bin sheet_rule_convert --
--check` → `records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081 verdict=PASS
(33.8s)`, exit 0, and 47,628 + 1,810 = 49,438 exactly; `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL='
data/sheet_rules/ | wc -l` → **0**; `cargo test --locked --test sheet_rule_convert_gate -j 6` → 27
passed / 0 failed (the 19 per-kind gates over the live corpus directory, the three value-form
tests on real records, literal scan, freshness, determinism, token census); `cargo test --locked
--lib sheet_rule -j 6` → 31 passed / 0 failed; all 19 per-kind lines sum. Gates:
`completion_atlas.py --check` `unclassified=0 overlap=0 done_evidence_violations=0
citation_failures=0`; `token_coverage.py --check` `non_done=1404 refused_non_done=659
token_types=231 shapes=81 verdict=PASS`; `shape_engine_boundary.py --check`
`magnitude_bearing=26396 not_held_by_engine=363`; `missing_engine_tables.py --check`
`population=1 citation_failures=0`; `denominator_gate.py --check` `files_checked=40 violations=0`;
`verify.sh --only pi-sweep` `RESULT: PASS`. `cargo test --locked --no-fail-fast` was **not** run
and is not required — `§6` step 3 conditions it on `src/` or the classifier changing, and neither
did.

**Two corrections, both recorded** (`docs/retro/events/at-35-e2-001.jsonl`). First, cycle 1's
receipt claims `Wired-integration audit result: OK_NO_TOKENS`; re-run over the Epic 2 file-touch
set at HEAD it returns **4 hits**, every one attributed and **none a stub in shipping code** — the
word `hack` twice as ordinary Pathfinder rules prose ("hack or smash its way out", "hack or force
a way through") in two generated `data/sheet_rules/` records, `placeholder` six times in
`docs/work-inventory.json` `reason` fields describing the source's own CHOOSE-menu "no selection"
rows, and one `not yet implemented` inside a transcribed description. The audit's keyword class is
a grep over English as well as code; `§8`'s non-self-healable "stub, inline mock, or `\"Would …\"`
string in shipping code" is not met, so the criterion stands.

Second, that last hit is a real finding and is filed as a **table defect, not fixed here**:
`data/sheet_rules/ultimate_intrigue/class_feature/courtly_hunter_courtly_companion.json` prints the
source's editorial bracket `[Change to magical beast and stacking restriction not yet
implemented]` as sheet prose. The converter is behaving as specified — it is a faithful
transcription of the source description — and what is missing is a mapping-table row scrubbing
source editorial annotations out of printed prose. No such row exists in
`token-mapping/mapping-table.v1.json`, and **inventing one inside the cycle is precisely the
defect `decisions.md §15` forbids**, so it is recorded for the table's owner instead
(`1788882373210-at-35-e2-001-f81ef5`). Magnitude: 1 record of 47,628 converted; no computed value
and no count depends on it. Not an `## Open blockers` entry — the criterion's Definition of Done
does not require it and nothing downstream is paused.

Refused tokens: none added by this cycle. The standing set is unchanged — 1,810 records, 81
shapes, 231 token types, of which 659 are non-DONE and are owned by AT-35-E4-001 under
`### AT-35-E2-005-DISPOSITION`'s hand-off table, not by this criterion. Next-cycle scope:
criterion at zero. Epic 2 is complete across rows 7–11 plus row 30; the live front is row 12
(AT-35-E3-001, `blocked-escalated` awaiting the orchestrator's re-scope).

### 2026-09-08 — AT-35-E1-004 cycle 2 — `ratio-row-and-gate-scope` — **complete** (re-dispatch that found a real gap: the default scan missed one SD-35 doc)

AT-35-E1-004 was dispatched a second time after cycle 1 had landed (`2bf452b038`, board row 4
already `complete`). The lane rebased to `942c8d3ae5`, re-verified the criterion, and **found the
evidence bar not actually met**: the criterion says a default `denominator-gate` run "lists every
SD-35 `.md` in `files_checked`", and it listed 39 of the 40 `.md` files under
`docs/release/SD-35-corpus-sheet-completion/`. `references/README.md` sits in neither the package
root nor `artifacts/`, so neither of cycle 1's two SD-35 glob entries matched it, and it was never
read by either stage. Cycle 1's own coverage test could not catch this: `_real_sd35_md()` built its
"every SD-35 `.md`" expected set by re-running the same two globs it then asserted `DEFAULT_GLOBS`
covered — an assertion that cannot fail for a file the globs miss. This cycle replaced that
expected set with a filesystem walk (RED, 2 failures, naming exactly `references/README.md`), then
added `SHEET_COMPLETION_BUNDLE_DIR/**/*.md` to **both** `DEFAULT_GLOBS` and
`PROVENANCE_DEFAULT_GLOBS` (GREEN). The widening is additive — cycle 1's two entries stay in both
lists, `expand_paths` deduplicates, and the criterion's "nothing already scanned stops being
scanned" invariant is still pinned by `test_nothing_already_scanned_stops_being_scanned`.

- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-retargeting cycle — closes zero units by design, decisions.md §2)`. `pcgen_residue_gate.py --check` at start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=260` — `python3 scripts/cycle_scope_gate.py --receipt --since 942c8d3ae5db5d447efd12400b2db3b8644e8be1 --before /tmp/wi-before-AT-35-E1-004.json --after docs/work-inventory.json` (`regressed=0 added=0 dropped=0`, `residue_gate=present`). No Rust, no corpus, no unit movement — the diff is two Python files under `scripts/`.
- **PCGen residue:** unchanged, `verdict=PASS` — not risen; no live path touched.
- **Evidence at HEAD:** `scripts/verify.sh --only denominator-gate` → `PASS (files_checked=227 violations=0)`, up from `225` and now covering all 41 SD-35 `.md` files (40 before this cycle's own receipt); `scripts/verify.sh --only figure-provenance` → `PASS (files_checked=157 figures_examined=173 violations=0)`, up from `155`; `python3 -m unittest discover -s scripts/tests -p test_denominator_gate.py` → `Ran 55 tests OK` (54 before; the new one, `test_expected_set_is_a_filesystem_walk_not_the_globs_under_test`, pins the anti-circularity fix so the coverage assertions cannot go vacuous again). The `--receipt` half of the criterion is unchanged and still carries `rust_lines_changed`, `ratio` and `pcgen_live_files`.
- **Other gates:** `cargo test --locked --no-run -j 6` exit 0; `cargo test --locked --lib -j 6` → `3217 passed; 0 failed; 14 ignored`; `sheet_rule_convert -- --check` exit 0; `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`; `completion_atlas.py`, `token_coverage.py` (`verdict=PASS`), `shape_engine_boundary.py`, `missing_engine_tables.py` all exit 0; `verify.sh --only pi-sweep` PASS. `--no-fail-fast`, `corpus_literal_sweep` and `clippy` not run — no `src/`, classifier, corpus or Rust target touched (`§6` step 3's own conditions).
- **Refused tokens:** none. **Discoveries:** one instrument-shaped — the self-referential coverage test — emitted as a `correction` retro event (`1788881408334-sd31-transcribe-957b44`; it landed in `sd31-transcribe.jsonl` because `RETRO_ACTOR` does not persist between this harness's shell calls, the same misfiling AT-35-E1-002 cycle 2 recorded).
- **Process note:** this is the fourth Epic 1 criterion re-dispatched after `kanban.md` already read `complete`. Unlike the other three it was **not** a no-op, which is the argument against treating a `complete` row as sufficient reason to skip the re-verify.
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-004_cycle2_receipt.md`.

### 2026-09-08 — AT-35-E1-001 **re-verification** (duplicate dispatch) — `batch-floor-gate` — **complete**, no new work

AT-35-E1-001 was dispatched a second time after it had already landed and pushed (code
`1d821cdc8d`, board rows `b826669560`, both ancestors of `origin/tranche/15`; `kanban.md` row 1
already `complete`). The lane rebased, found the criterion at zero, and **re-verified rather than
duplicating the work**. Only the receipt appendix, this entry, and one retro `incident`
(`recurrence-key duplicate-criterion-dispatch`) were written; no code, test, or `verify.sh`
change. Commit for this entry only.

- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-building cycle — this cycle CREATES cycle_scope_gate.py; it closes zero units by design, decisions.md §2)` — the exemption carried forward from cycle 1; the re-verification itself moved zero units. Live at HEAD `4e321d2c6c`: `python3 scripts/cycle_scope_gate.py --min 500` → `scoped=1404 remaining_non_done=1404 floor=500 verdict=PASS` exit 0; `--bucket A --kind companion` → `scoped=0 ... FAIL_UNDER_FLOOR` exit 1; `--bucket B --kind class_feature` → `scoped=214 ... FAIL_UNDER_FLOOR` exit 1. All three RED→GREEN shapes hold.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=260` — `python3 scripts/cycle_scope_gate.py --receipt --since 4e321d2c6c --before /tmp/wi-recheck-AT-35-E1-001.json --after docs/work-inventory.json` (`regressed=0 added=0 dropped=0`, `residue_gate=present`). **`pcgen_live_files` now resolves to a number** rather than cycle 1's `unavailable`, because AT-35-E1-005 has since landed the residue gate — the `--receipt` half of the criterion is proven end to end for the first time.
- **Refused tokens:** none.
- **Gates:** `python3 -m unittest scripts/tests/test_cycle_scope_gate.py` → `Ran 51 tests OK`; `verify.sh --only cycle-scope-gate-selftest` → `PASS (51 cases passed)`; `pcgen_residue_gate.py --check` → `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`; `completion_atlas.py --check`, `shape_engine_boundary.py --check`, `missing_engine_tables.py --check` all exit 0; `verify.sh --only pi-sweep` PASS. No cargo run — no `.rs` touched.
- **Cleared since cycle 1:** `denominator_gate.py --check` over the package globs is now `files_checked=38 violations=0`; cycle 1's 11 token-mapping violations are fixed (AT-35-E1-004).
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-001_cycle1_receipt.md`, "Re-verification appendix" section.

### 2026-09-08 — AT-35-E1-003 **re-dispatch** (no new cycle) — `test-families-table-driven` — **complete** (already closed at `03072aea0c`; re-verified at HEAD, zero change to code or baselines)

- **Scope gate:** `SCOPE_GATE: EXEMPT (build-time tax cut — closes zero corpus units by design, decisions.md §2)` — unchanged from cycle 1. `pcgen_residue_gate.py --check` at start of the re-dispatch: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** cycle 1's, unchanged and re-stated: `closed=0 relabeled=0 rust_lines_changed=1906 ratio=n/a builds_recorded=0 pcgen_live_files=260`. Re-running `cycle_scope_gate.py --receipt --since 53296d80f0 --before /tmp/wi-before-AT-35-E1-003.json --after docs/work-inventory.json` at HEAD `4e321d2c6c` prints `closed=21911 relabeled=0 rust_lines_changed=11339 ratio=0.52 builds_recorded=1 pcgen_live_files=260` — that window spans 30 commits of Epic 1/2/3 lanes, **not** this criterion's cycle; the criterion's own window ends at `03072aea0c`.
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` — not risen; nothing live-side touched.
- **Refused tokens:** none (no converter run).
- **Verification at HEAD `4e321d2c6c`:** `ls tests/sd18_*_widening.rs tests/sd13_*progression*.rs` → `No such file or directory` (0 standalone family binaries); `tests/sd18_widening/main.rs` + `tests/sd13_progression/main.rs` present; `BASELINE_ROOT_TEST_BINARIES=408` still the last assignment in `scripts/verify-baselines.env` (`:3484`); `CARGO_INCREMENTAL=0 cargo test --locked -j 6 --test sd18_widening --test sd13_progression` → exit 0, `980 passed; 0 failed` + `1239 passed; 0 failed` (2,219 family tests), **0 warnings**; §6 step 2 audits on the final diff with rename pairing (`-M`, pathspec `'tests/sd18_*' 'tests/sd13_*'`) → 13 identifier matches on added lines (the same 13 the receipt itemises, all citations/doc comments/env notes) and `OK_NO_TOKENS`; `git diff -M --summary` → `184` renames.
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-003_cycle1_receipt.md` (re-verification row appended). **Process defect:** the criterion was dispatched again although `kanban.md` row 3 already read `complete` — retro `rework` `1788880170541-at-35-e1-003-b1939e`; avoidable by grepping the criterion id in `kanban.md` for `complete` before dispatch.
### 2026-09-08 — AT-35-E1-002 cycle 2 — `content-anchored-citations` — **complete** (re-verification at `4e321d2c6c`; the criterion's own `verify.sh` stage was RED and is green again)

- **Scope gate:** `SCOPE_GATE: EXEMPT (instrument-hardening cycle — closes zero units by design, decisions.md §2)`. `pcgen_residue_gate.py --check` at start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 4e321d2c6c --before /tmp/wi-before-AT-35-E1-002-c2.json --after docs/work-inventory.json`; `regressed=0 added=0 dropped=0`). No Rust, no corpus, no build paid.
- **PCGen residue:** unchanged, `verdict=PASS` — not risen.
- **What this cycle found.** The criterion's deliverable **held**: between cycle 1 (`815139fadd`) and `4e321d2c6c`, Epic 2 rewrote the classifier in `src/bin/v06_work_inventory.rs` and all sixteen content anchors still resolve — `citation_failures=0` in `completion_atlas.py`, `shape_engine_boundary.py` and `missing_engine_tables.py`. That is the anchors proven against a real refactor rather than synthetic source. But the stage this criterion added, `shape-engine-boundary-selftest`, was **RED**: four SD-34-era *equality* pins on live populations that Epic 2's conversion legitimately drained (`not_held_by_engine` 8784 → 363, bucket A 449 → 1, `missing_engine_tables` population 449 → 1, kinds `{companion, power}` → `{power}`). Re-pinning to the new live value would repeat the six-wave staleness cycle 1's own comment records, so each is now a ceiling against the SD-34 high-water mark plus the structural invariant (kind set, book set) — still failing closed, no longer hand-maintained (`AGENTS.md` rule 8).
- **Verification:** `cd scripts && python3 -m unittest tests/test_completion_atlas.py tests/test_missing_engine_tables.py tests/test_shape_engine_boundary.py tests/test_denominator_gate.py` → `Ran 138 tests … OK` (was `FAILED (failures=4)`); `verify.sh --only shape-engine-boundary` / `shape-engine-boundary-selftest` / `missing-engine-tables` / `pi-sweep` all `RESULT: PASS`; `denominator_gate.py --check` `files_checked=39 violations=0`; fail-closed mutation proof 4 of 4.
- **Refused tokens:** none. **Discoveries:** the anchors-survive-a-real-refactor result, and one `correction` (`1788880325253-at-35-e1-002-c18322`) for the four stale pins; a misfiled duplicate of it sits in `docs/retro/events/sd31-transcribe.jsonl` (`…-8d1c42`) because `RETRO_ACTOR` was not exported in that shell.
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-002_cycle2_receipt.md`.

### 2026-09-08 — AT-35-E3-001 cycle 1 — `class-feature-b-zero` — **blocked-escalated** (§8 under-floor re-scope, not an operator ruling; the cycle did not start)

- **Scope gate:** `scoped=214 remaining_non_done=1404 floor=500 verdict=FAIL_UNDER_FLOOR` — `python3 scripts/cycle_scope_gate.py --min 500 --bucket B --kind class_feature` at `8cc4ea1516` (`scoped_by_bucket=B:214`, `scoped_by_kind=class_feature:214`); under the floor and not the whole 1,404 remainder, so the cycle did not start (`workflow-instruction.md §6` step 1, `§8`). `pcgen_residue_gate.py --check` at start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 8cc4ea1516 --before /tmp/wi-before-AT-35-E3-001.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E3-001`; `regressed=0 added=0 dropped=0`; docs only, no build paid).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` after — not risen (no live-side file touched).
- **Refused tokens:** the scoped **214 of 1,404 non-DONE of 49,438** are **214 of 214 converter-refused** (ids in `data/sheet_rules/_refused.json`; status `engine-does-not-hold`); by first refused type, summing to 214: `FORMULA:var(COUNT)=169, FORMULA:malformed (parser refusals)=11, BONUS:[redacted PI]=6, FORMULA:var(STAT)=6, BONUS:SITUATION (target shape)=2, FORMULA:var(SPELLFAILURE)=2, FORMULA:var(<export token>) (ENCUMBERANCE)=2, BONUS:STAT (target BASESPELLKNOWNSTAT;Class)=2`, and 14 types at 1 each — **24 distinct types** (>10, `§8`). Evidence families: owner-matched 154, option-pool-with-magnitude 42, option-pool 18 (sum 214). Deferral `1788879242003-at-35-e3-001-bf4043`.
- **Discoveries (1 `correction`, `docs/retro/events/at-35-e3-001.jsonl`):** `epic-breakdown.md` `### AT-35-E3-001` and the dispatch prompt carry 7,866 (authoring, pre-Epic 2); at HEAD the population is 214 and every unit of it is converter-refused — the `applies` widening has no non-refused unit left to move (AT-35-E2-005 cycle 1 closed them all; the DISPOSITION owner rule routes refused B to AT-35-E4-001). `1788879241856-at-35-e3-001-ccb13f`, caught before implementation.
- **Verification (docs gates only):** no build (`git diff --stat 8cc4ea1516..HEAD -- src scripts tests data apps` empty); atlas `population=49438 unclassified=0 overlap=0 done_evidence_violations=0` (SD-34 atlas `derived_at` re-stamp reverted); residue PASS; identifier audit OK_NO_BUNDLE_TAGS; wired-integration audit OK_NO_TOKENS on this cycle's diff; denominator gate `files_checked=38 violations=0`.
- **Receipt:** `artifacts/epic-3-place-and-surface/AT-35-E3-001_cycle1_receipt.md`. **Re-scope for the orchestrator:** `python3 scripts/cycle_scope_gate.py --min 500 --bucket A --bucket B --bucket C --bucket D --bucket M` → `scoped=623 remaining_non_done=1404 floor=500 verdict=PASS` — all 623 converter-refused, AT-35-E4-001's population by the DISPOSITION owner rule (mapping rows in `src/pcgen_import/sheet_rule/`, `FORMULA:var(COUNT)` first); or the whole 1,404 with no flags. AT-35-E3-001 closes at `class_feature` B = 0 once those rows land; SD-33 deferral 1 stays with it.

### 2026-09-08 — AT-35-E2-005-DISPOSITION cycle 1 — `e2-005-disposition` — **complete** (orchestrator re-scope recorded; row 11 → complete against its amended bar)

- **Scope gate:** `SCOPE_GATE: EXEMPT (disposition cycle — it moves no unit; it records where every remaining unit is owned)` — `decisions.md §2`. `pcgen_residue_gate.py --check` at start (`38b67db94e`): `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 38b67db94e --before /tmp/wi-before-AT-35-E2-005-DISPOSITION.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-005-DISPOSITION`; `regressed=0 added=0 dropped=0`; docs only, no build paid).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` after the cycle's work — not risen (no live-side file touched).
- **Refused tokens:** none by this cycle (no converter run). **The hand-off, re-derived at HEAD** (`python3 artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_handoff.py` → `non_done=1404 atlas_non_done=1404 refused_non_done=659 not_refused_non_done=745 owned_sum=1404 unowned=0 duplicate_ids=0 verdict=PASS`): of the **1,404 non-DONE of 49,438**, **659** converter-refused → **AT-35-E4-001** (A 1, B 437, C 79, D 43, M 63, U 4, V 1, X 31; 69 refusal strings / 81 shapes — `FORMULA:var(COUNT)=210, unmapped:STARTSKILLPTS=119, SPELLS (PI-redacted token)=66, BONUS:[redacted PI]=62, FORMULA:malformed=62, DEFINE (PI-redacted token)=40, unmapped:MODTOSKILLS=37, unmapped:SPELLSTAT=23, unmapped:MEMORIZE=19, …`; the 144 non-DONE `class` records of 182 refused `class` records first); **391** non-refused V (`literal-verified` 388 + `fixture-verified` 3) → **AT-35-E4-002**; **217** non-refused U 198 + Z 19 → **AT-35-E5-003**; **137** non-refused X → **AT-35-E5-004**. Deferral `1788878644195-at-35-e2-005-disposition-6bbb45`.
- **What landed:** `epic-breakdown.md` `### AT-35-E2-005` carries a dated amendment (original text kept): the bar is now the pass measured, the report and ledger re-derived, the oracle harness run and agreeing (`compared=42 agree=41 disagree=1` at `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, the one named), and zero mapping rows added — met at HEAD by cycles 1–4; a new `### AT-35-E2-005-DISPOSITION` section holds the owner rule and the hand-off table; AT-35-E4-001 / E4-002 / E5-003 / E5-004 each state the units they inherited. `decisions.md §16` records the re-scope, citing the four receipts and the reason (the criterion's own "No mapping row is added in this cycle" forbids the only mechanism that moves the 659; cycles 2–4 closed 0 each; cycle 4 `blocked-escalated` under `§8`'s >10-refused-type rule; a fifth cycle is byte-identical). `kanban.md` row 11 → `complete` (Epic column typo `4` → `2` fixed), row 30 added for this cycle. **No carve-out:** every non-DONE unit is owned by a named criterion and stays in AT-35-E5-005's 49,438 of 49,438.
- **Discoveries (1 `correction` event, `docs/retro/events/at-35-e2-005-disposition.jsonl`):** the four receipts' "745 = V 389 + 3, U 202, X 137, Z 19" sums to 750 — at HEAD 1 V and 4 U units are converter-refused (E4-001's), so the non-refused split is V 391 + U 198 + X 137 + Z 19 = 745 (`…-6224d1`; blast radius: four receipts, four progress entries, row 11, and this cycle's dispatch prompt). `### AT-35-E2-005-DISPOSITION` did not exist in `epic-breakdown.md` at cycle start; this cycle writes it.
- **Verification (docs gates only, `decisions.md §3`):** no build (`git diff --stat 38b67db94e..HEAD -- src scripts tests data apps` empty); atlas `population=49438 unclassified=0 overlap=0 done_evidence_violations=0` (SD-34 atlas `derived_at` re-stamp reverted); token-coverage `non_done=1404 refused_non_done=659 verdict=PASS`; residue PASS; denominator gate `files_checked=37 violations=0`; hand-off script `verdict=PASS`.
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_cycle1_receipt.md`. **Epic 2: 5 of 5 complete — the epic wrap-up (`§10`) if not yet run, then Epic 3 opens (`--bucket B --or --bucket C --or --bucket D` → `scoped=559`, or the whole 1,404) on the holdings gap the parity names; AT-35-E4-001 takes the 659 by refusal string, the 144 non-DONE `class` records first.**

### 2026-09-08 — AT-35-E2-005 cycle 4 — `first-corpus-wide-conversion` (remainder) — **blocked-escalated** (§8 re-scope, not an operator ruling)

- **Scope gate:** `scoped=1404 remaining_non_done=1404 floor=500 verdict=PASS` — `python3 scripts/cycle_scope_gate.py --min 500` (no flags: the whole remainder, all 37 books; `A:1 B:437 C:79 D:43 M:63 U:202 V:392 X:168 Z:19`) at `bf9594943f`. `pcgen_residue_gate.py --check` at start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since bf9594943f --before /tmp/wi-before-AT-35-E2-005.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-005`; `regressed=0 added=0 dropped=0`; no Rust touched, no build paid — cycle 3's warm target dir served every binary).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` after the cycle's work — not risen (no live-side file touched).
- **Refused tokens:** unchanged — nothing that feeds the converter, the classifier or the parity changed since cycle 3 (`git diff --stat c0f16fe417..bf9594943f -- src scripts data/sheet_rules docs/work-inventory.json apps` is empty), so the pass at HEAD is byte-identical (`records=49438 converted=47628 refused=1810`, 25.64 s; `--check` PASS 20.23 s), the guarded inventory regen (sweep `CLEAN` 147.48 s, derived 12.08 s, inventory 726.14 s) produced a `generated_at`-only diff (reverted), and the remainder is still **1,404 non-DONE of 49,438**: 659 refused by token type (`FORMULA:var(COUNT)=210, unmapped:STARTSKILLPTS=119, SPELLS (PI-redacted token)=66, FORMULA:malformed (parser refusals)=62, BONUS:[redacted PI]=62, DEFINE (PI-redacted token)=40, unmapped:MODTOSKILLS=37, unmapped:SPELLSTAT=23, unmapped:MEMORIZE=19, …`; 69 strings / 81 shapes, full list in the receipt) and 745 in non-promotable statuses (V 392, U 202, X 137, Z 19). Deferral `1788869018190-at-35-e2-005-ab58c2`; rework `1788869018329-at-35-e2-005-7d897d`. **No mapping row added.**
- **Oracle parity:** engine side re-run at HEAD (`characters=29 lines=270`, 5.27 s) and joined to cycle 3's 29 exports at `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`: **`compared=42 agree=41 disagree=1 unverifiable=5`**, `sheet-parity.json` byte-identical; the one disagreement is still Weapon Focus on the deterministic fighter (ours 0, PCGen 1 — `Var vb1e14268d73c2def`, fed only by `core_rulebook:class_feature:default`'s `Const(1)`, a holdings gap for Epic 3); chassis 376/6/140 unchanged.
- **Verification (one pass, `decisions.md §3`):** `cargo test --locked --no-run -j 6` exit 0 (1.17 s, fully cached); `--lib` → 3217 passed / 0 failed / 14 ignored (52.24 s); `--no-fail-fast` not run (`src/` unchanged — §6's condition); clippy not run (no Rust target touched); `python3 -m unittest scripts/tests/test_sheet_parity.py` → 24 OK; `sheet_rule_convert -- --check` PASS; literal scan 0; residue PASS; atlas `population=49438 unclassified=0 overlap=0 DONE: 48034 done_evidence_violations=0` before and after; token-coverage `non_done=1404 refused_non_done=659 shapes=81 verdict=PASS`; shape-engine-boundary `magnitude_bearing=26396 not_held_by_engine=363 citation_ok=True`; missing-engine-tables `citation_failures=0`; denominator-gate `files_checked=36 violations=0` (on the receipt, before this entry); pi-sweep `RESULT: PASS` (8.46 s). Desktop crate and frontend: epic cadence.
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-005_cycle4_receipt.md` — `cd3d64e578`. **Why blocked-escalated, not partial:** `workflow-instruction.md §8`'s non-self-healable ">10 distinct refused token types in one cycle — re-scope, do not grind" (69 strings over 659 units, unchanged across four cycles) on a criterion whose own text forbids the mapping rows they need; `partial` re-triggered this identical cycle once already. The criterion's evidence obligations are met at HEAD. **No `## Open blockers` entry** — the disposition is the orchestrator's re-scope: Epic 3 on `--bucket B --or --bucket C --or --bucket D` (`scoped=559`) or the whole 1,404; AT-35-E4-001 on the 659 by token string, the 182 `class` records first.

### 2026-09-08 — AT-35-E2-005 cycle 3 — `first-corpus-wide-conversion` (remainder) — **partial**

- **Scope gate:** `scoped=1404 remaining_non_done=1404 floor=500 verdict=PASS` — `python3 scripts/cycle_scope_gate.py --min 500` (no flags: the whole remainder, all 37 books; `A:1 B:437 C:79 D:43 M:63 U:202 V:392 X:168 Z:19`) at `b3f032a934`. `pcgen_residue_gate.py --check` at start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since b3f032a934 --before /tmp/wi-before-AT-35-E2-005.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-005` at `c0f16fe417`; `regressed=0 added=0 dropped=0`; no Rust touched, no build paid — cycle 2's warm target dir served every binary).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `c0f16fe417` — not risen (no live-side file touched; the new PCGen reads are in `scripts/oracle_harness/`, the test oracle).
- **Refused tokens:** unchanged — the conversion pass at HEAD is byte-identical (`records=49438 converted=47628 refused=1810`, 25.44 s; `--check` PASS 20.16 s), the guarded inventory regen (sweep `CLEAN` 141.47 s, derived 12.08 s, inventory 776.27 s) produced a `generated_at`-only diff (reverted), and the remainder is still **1,404 non-DONE of 49,438**: 659 refused by token type (`FORMULA:var(COUNT)=210, unmapped:STARTSKILLPTS=119, SPELLS (PI-redacted token)=66, FORMULA:malformed (parser refusals)=62, BONUS:[redacted PI]=62, DEFINE (PI-redacted token)=40, unmapped:MODTOSKILLS=37, unmapped:SPELLSTAT=23, unmapped:MEMORIZE=19, …`; full list in the receipt) and 745 in non-promotable statuses (V 392, U 202, X 137, Z 19). Deferral `1788868218173-at-35-e2-005-43630b`. **No mapping row added** — the criterion's own rule.
- **What landed:** the criterion's oracle check, widened on the tool side only (`decisions.md §11`). Cycle 2's `compared=8 agree=8` was an export-coverage floor: 34 of its 39 unverifiable `Number` values were reachable through PCGen's own export vocabulary. `scripts/oracle_harness/sheet-totals.txt.ftl` now emits every ability category's `ABILITYPOOL` total (`charbonusto`, the 230 categories the pinned Core Rulebook chain declares, filled into the template by the export step), the wielded weapon's own attack bonus, and every `SPELLMEM` row (uses / caster level / DC per spellbook — the `SPELLS:` token's spell-like abilities); `sheet_parity.py` joins `Pool` by category slug, `WeaponAttack` by `WEAPON.0.TOTALHIT-ATTACK.MELEE.TOTAL`, and a standalone value with no DESC number in its role by the same-named spell row in the same role (17 → 24 tests, RED first). **Parity at `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`: `compared=42 agree=41 disagree=1 unverifiable=5`** (cycle 2: 8/8/0/39; chassis 376/6/140 unchanged). **The disagreement, named:** `core_rulebook:feat:weapon_focus` on the deterministic fighter, ours **0** vs PCGen **1** — `Expr` `Var vb1e14268d73c2def`, fed only by `core_rulebook:class_feature:default`'s `Const(1)`, a rule the held set does not hold (Epic 3 holdings, not a mapping defect). The 5 unverifiable: 4 `SPELL-dc-not-numeric` (PCGen prints a blank DC for no-save spells) + 1 `no-component-export` (`Other:accheck`). Reverse census: PCGen exports a non-zero pool the sheet prints no `Pool` line for on **19** (character, category) pairs — the same holdings gap. Correction `1788867844870-at-35-e2-005-57c381`.
- **Verification (one pass, `decisions.md §3`):** `cargo test --locked --no-run -j 6` exit 0 (1.27 s, fully cached); `--lib` → 3217 passed / 0 failed / 14 ignored (51.78 s); `--no-fail-fast` not run (`src/` unchanged — §6's condition); clippy not run (no Rust target touched); python RED (4 failures + 3 errors) → GREEN (`Ran 24 tests … OK`); `sheet_rule_convert -- --check` PASS (20.33 s); literal scan 0; residue PASS; atlas `population=49438 unclassified=0 overlap=0 done_evidence_violations=0`; token-coverage `refused_non_done=659 verdict=PASS`; shape-engine-boundary / missing-engine-tables / pi-sweep (8.48 s) green; denominator gate on the bundle docs and this cycle's receipt: `files_checked=35 violations=0`; desktop and frontend at epic cadence (no `apps/` touch).
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-005_cycle3_receipt.md`. **Next:** a fourth remainder cycle on this criterion would repeat this one (it forbids the mapping rows the 659 need; `§8`'s >10-type rule says re-scope); Epic 3 opens (`--bucket B --or --bucket C --or --bucket D` → `scoped=559`, or the whole 1,404) on the holdings gap the parity now names twice; AT-35-E4-001 takes the 659 by token string, the 182 `class` records first.

### 2026-09-08 — AT-35-E2-005 cycle 2 — `first-corpus-wide-conversion` (remainder) — **partial**

- **Scope gate:** `scoped=1404 remaining_non_done=1404 floor=500 verdict=PASS` — `python3 scripts/cycle_scope_gate.py --min 500` (no flags: the whole remainder, all 37 books; `A:1 B:437 C:79 D:43 M:63 U:202 V:392 X:168 Z:19`) at `bfa6e81364`. `pcgen_residue_gate.py --check` at start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=120 ratio=n/a builds_recorded=2 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since bfa6e81364 --before /tmp/wi-before-AT-35-E2-005.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-005` at `33deab007b`; `regressed=0 added=0 dropped=0`; `ratio=n/a` — a zero-closure cycle has no denominator; the 120 Rust lines are the live evaluator's two mechanisms and three tests, and they bought 11 → 0 parity disagreements).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `33deab007b` — not risen (the one live-side file changed, `src/rules_core/sheet_rule.rs`, reads the package and the facts only).
- **Refused tokens:** unchanged — the converter did not change, the pass wrote a byte-identical package, and the remainder is still **1,404 non-DONE of 49,438**: 659 refused by token type (`FORMULA:var(COUNT)=210, unmapped:STARTSKILLPTS=119, SPELLS (PI-redacted token)=66, FORMULA:malformed (parser refusals)=62, BONUS:[redacted PI]=62, DEFINE (PI-redacted token)=40, unmapped:MODTOSKILLS=37, unmapped:SPELLSTAT=23, unmapped:MEMORIZE=19, …`; full list in the receipt) and 745 in non-promotable statuses (V 392, U 202, X 137, Z 19). Deferral `1788863079814-at-35-e2-005-291114`. **No mapping row added** — the criterion's own rule; this remainder cycle can close none of the 1,404 without one.
- **What landed:** the cycle spent its build on the two live-evaluator defects cycle 1's parity run found, and on two harness defects the re-run exposed. `src/rules_core/sheet_rule.rs` (live side, no PCGen): **(1)** a `#bonusN` sibling now prints only when its own `applies` includes (`render_sheet`) — cycle 1's 488 rendered lines carried **218 gated sibling lines** that printed although their gates excluded (Fighter Bonus Feats' 22 archetype −1 lines per fighter, Climb/Swim's +8 Racial and +3/+6 Skill Focus lines, the chain shirt's "Broken" −2); **(2)** a `class` kind rule id is held by the character's levels whether or not the package carries the class record (`HeldSet.classes`) — cycle 1's "the class rule `HeldSeed` never seeds" was a mis-attribution: `held_set` does seed classes, but the converter refuses all **182** `class` records (unmapped `STARTSKILLPTS`/`SPELLSTAT`/`MEMORIZE`/`SPELLLIST`/…, AT-35-E4-001), and the declared contribution is just `ClassLevel(<class>)`, computable from the facts. RED → GREEN on 3 new tests (the fighter's Climb siblings off the sheet; a synthetic class-level `Var`; Bardic Performance **7** / **25** rounds on the live package). Harness (tool side): the roster now pre-bakes each race's fixed ability adjustment into the engine fixture — the chassis's documented fixture contract, which cycle 1's roster generator broke (that, not a chassis defect, was 22 of the 23 chassis disagreements) — read from the pinned PCGen data's `<Race> ~ Ability Scores` row; the export template adds `ACCHECK` and `SKILL.n.ACHECK`; the comparator removes the armor check penalty from `SKILL.n.MISC` on armor-check skills and joins `also` values in their own role (`DC N`, `caster level N`, `N … per day`) instead of against every integer in the description (4 of cycle 1's 11 disagreements were that join's). 17 self-tests, RED on the HEAD module → GREEN. `technical-design.md §2` names both evaluator mechanisms. **Parity re-run at `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`: lines `compared=8 agree=8 disagree=0 unverifiable=39`** (cycle 1: 12 / 1 / 11 / 42) over 47 `Number` values in 270 lines; chassis (context) `compared=382 agree=376 disagree=6` (cycle 1: 359 / 23) — the 6 are Halfling Luck's +1 (a `Var` contribution on a `Text` race-trait rule with no held Save-target consumer) and Divine Grace's +3 (`print=false`, not seeded), both Epic 3 holdings. Conversion pass **24.72 s** (byte-identical package, `--check` PASS 20.47 s); guarded inventory regen **748.95 s** (sweep `CLEAN` 172 s, fixture check 15 s) moved **0** units — `generated_at` only, reverted. Export 221.7 s (29 characters, 3 jobs), engine side 6.9 s. Three corrections `…-21cf2f`, `…-7b04c9`, `…-660e06`. Artifacts: `artifacts/epic-2-sheet-rule/oracle-parity/` (roster, exports, `ours.json`, `sheet-parity.json`).
- **Verification (one pass, `decisions.md §3`):** `cargo test --locked --no-run -j 6` exit 0 (99.5 s); `--lib` → 3217 passed / 0 failed / 14 ignored (53.7 s; 3 new tests RED → GREEN); `--no-fail-fast -j 6` → **412 binaries, 412 ok, 8,721 passed, 0 failed, 67 ignored** (2,361 s); clippy 0 warnings on `--lib --bin sheet_rule_parity`; python RED on the HEAD module (4 failures + 3 errors) → GREEN (`Ran 17 tests … OK`); `sheet_rule_convert -- --check` PASS (20.5 s); literal scan 0; residue PASS; atlas `unclassified=0 overlap=0 done_evidence_violations=0`; token-coverage `verdict=PASS`; shape-engine-boundary / missing-engine-tables / denominator-gate (`files_checked=34 violations=0`) / pi-sweep green; desktop and frontend at epic cadence (no `apps/` touch).
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-005_cycle2_receipt.md`. **Next:** Epic 2 is complete as a set of instruments; Epic 3 opens (`--bucket B --or --bucket C --or --bucket D` → `scoped=559`, or the whole 1,404) on the holdings the parity now names; AT-35-E4-001 takes the 659 refused by token string, the 182 `class` records first.

### 2026-09-08 — AT-35-E2-005 cycle 1 — `first-corpus-wide-conversion` — **partial**

- **Scope gate:** `scoped=23315 remaining_non_done=23315 floor=500 verdict=PASS` — `python3 scripts/cycle_scope_gate.py --min 500` (no flags: the whole remainder, all 37 books) at `87647621a6`. `pcgen_residue_gate.py --check` at start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=21911 relabeled=0 rust_lines_changed=337 ratio=0.02 builds_recorded=4 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 87647621a6 --before /tmp/wi-before-AT-35-E2-005.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-005` at `51f91bba11`; `closed_by_kind=ability:1975 class:3 class_feature:12215 companion:661 deity:417 domain:147 equipment:165 equipment_modifier:433 feat:980 language:114 monster:25 monster_ability:13 power:420 race:59 race_trait:1183 skill:44 spell:946 template:1983 trait:128`, `regressed=0 added=0 dropped=0`).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `51f91bba11` — not risen (no live-side file changed; the new binary is `src/bin/`, tool side).
- **Refused tokens:** the remainder at HEAD is **1,404 non-DONE of 49,438**: **659 refused** by the converter (69 strings, multiplicity 851: `FORMULA:var(COUNT)=210, unmapped:STARTSKILLPTS=119, SPELLS (PI-redacted token)=66, FORMULA:malformed (parser refusals)=62, BONUS:[redacted PI]=62, DEFINE (PI-redacted token)=40, unmapped:MODTOSKILLS=37, unmapped:SPELLSTAT=23, unmapped:MEMORIZE=19, FORMULA:var(<export token>) (ENCUMBERANCE)=17, …` — full list in the receipt and `artifacts/epic-2-sheet-rule/token-coverage.json`) and **745** in statuses outside the `sheet-complete` rung's promotable set (V 392, U 202, X 137 non-refused, Z 19). Deferral `1788859084916-at-35-e2-005-4c14f4`. **No mapping row added** (the point of the cycle).
- **What landed:** the first corpus-wide pass. `sheet_rule_convert` re-ran over all 49,438 records in **24.6 s** (release; measured first on 3 `--one` samples at ~7.7 s fixed cost, projected ≈ 23 s) and wrote a package **byte-identical** to the committed one (`--check` PASS, 20.7 s); the inventory regenerated **once, guarded** (`corpus_literal_sweep --json-out` 142.8 s CLEAN + `derived_evaluator_fixture_check --json-out` 11.8 s, then `v06_work_inventory` **802.6 s** — a first unguarded run was refused by the stamp-loss guard, correctly). The rung stamped **21,911** units `sheet-complete` (`dice=897 number=4400 words=16614`), exactly AT-35-E2-003's projection: **DONE 26,123 → 48,034; non-DONE 23,315 → 1,404** by id-set diff (from B 11,152 / M 4,271 / C 4,101 / D 1,939 / A 448; by kind `class_feature` 12,215, `template` 1,983, `ability` 1,975, `race_trait` 1,183, `feat` 980, `spell` 946, `companion` 661, …; buckets at HEAD `A 1 B 437 C 79 D 43 M 63 U 202 V 392 X 168 Z 19`). `completion_atlas.py --check` before/after both `unclassified=0 overlap=0 done_evidence_violations=0`; `token_coverage.py --check` re-derived → `non_done=1404 tokened=1399 token_less=5 refused=1810 refused_non_done=659 token_types=231 shapes=81 verdict=PASS` (7 token types still carry ≥ 500 non-DONE units, down from 44). **B1 cleared and the oracle parity run made:** `scripts/oracle_harness/sheet-totals.txt.ftl` (skills, initiative, speed, vision, DR, SR, spells cast/known/DC per class×level, weapon lines, every SA/FEAT with its substituted DESC), `scripts/oracle_harness/sheet_parity.py` (roster / export / compare, 12 self-tests) and `src/bin/sheet_rule_parity.rs` (the engine side through `with_sheet_rules`, the desktop's path). Roster: 29 characters (the deterministic fighter + its GE-05 `.pcg` twin, 11 CRB classes × L1/L10, 6 other CRB races); PCGen export 211 s at 3 jobs (18.4 s per run), engine side 6.6 s. **`compared=12 agree=1 disagree=11 unverifiable=42` over 272 `Number` values in 488 rendered lines, `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`**; chassis totals (context) `compared=382 agree=359 disagree=23`. Every disagreement is named with its `Expr` and PCGen's value in the receipt; two root causes account for 9 of the 11: **(1) sibling `#bonusN` lines print regardless of their own `applies` gate** (`held_set` inserts siblings outright; `Evaluator::line` reads `applies` only for `Situational` text — the fighter's Climb folds to 17 vs PCGen 1), **(2) class-level `Var`s fold to 0** because their only declarer is the `class` kind rule `HeldSeed` never seeds (Bardic Performance 5 vs 7/25, Lay on Hands 3 vs 8, Detect Evil CL 0). Both are AT-35-E3-001's first mechanisms; corrections `…-5a8235`, `…-cfe0bc`, `…-961e3c` (the third: the chassis applies no non-human racial ability adjustments — 22 of the 23 chassis disagreements). Artifacts: `artifacts/epic-2-sheet-rule/oracle-parity/` (roster, exports, `ours.json`, `sheet-parity.json`).
- **Verification (one pass, `decisions.md §3`):** `cargo test --locked --no-run -j 6` exit 0 (121.9 s); `--lib` → 3212 passed / 2 failed (the two moved pins) / 14 ignored, the two re-run green at HEAD; `--no-fail-fast -j 6` → **412 binaries, 410 ok, 2 FAILED — 8,714 passed, 4 failed, 67 ignored** (2,367.9 s); the 4 failures are the four pins this cycle's own regen moved (2 lib, 2 in `tests/v06_work_inventory.rs`), re-derived and re-run green at HEAD (`--test v06_work_inventory` → 16 passed / 0 failed / 1 ignored); two population pins moved by this cycle's own regen and re-derived in the same commit (§8): `class_feature_owner_matched_non_excluded_remainder_is_24_and_named_by_subcause` 138/18/6 → 1/0/0 (`mechanism_units` 162 → 1) and F1 5,124 → **239** (`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus`); clippy 0 warnings on `--bin sheet_rule_parity`; python self-tests RED → GREEN (`Ran 12 tests … OK`); `sheet_rule_convert -- --check` PASS; literal scan 0; residue PASS; atlas / shape-engine-boundary (`magnitude_bearing=26396 not_held_by_engine=363 citation_ok=True`) / missing-engine-tables / denominator-gate (`files_checked=33 violations=0`) / pi-sweep green; desktop and frontend at epic cadence (no `apps/` touch).
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-005_cycle1_receipt.md`. Code `51f91bba11` (after `47bfea1a1d`, a fold of a live `sd31-transcribe` retro append). **Epic 2: 4 complete + this cycle partial — the epic wrap-up (`§10`) runs next; AT-35-E3-001 opens on the 559-unit B/C/D remainder (`--bucket B --or --bucket C --or --bucket D`) with the two evaluator findings above as its first mechanisms.**

### 2026-09-08 — AT-35-E2-004 cycle 1 — `token-coverage-ledger` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (ledger-building cycle — closes zero units by design)` — `decisions.md §2`. `pcgen_residue_gate.py --check` at start (`6ce95e2b87`): `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=295 ratio=n/a builds_recorded=3 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 6ce95e2b87 --before /tmp/wi-before-AT-35-E2-004.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-004` at `344f18d1e1`).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `344f18d1e1` — not risen (no live-side file changed).
- **Refused tokens:** the ledger's remainder by the token type each refusal arose under, non-DONE, 49 types summing (with multiplicity) to 850 over 659 distinct units of 23,315: `ABILITY=200, unmapped:STARTSKILLPTS=119, SPELLS (PI-redacted token)=66, BONUS:[redacted PI]=62, BONUS:VAR=60, DEFINE (PI-redacted token)=40, DESC=40, unmapped:MODTOSKILLS=37, unmapped:SPELLSTAT=23, BONUS:COMBAT=19, unmapped:MEMORIZE=19, BONUS:SKILL=15, …` (full list in the receipt and `artifacts/epic-2-sheet-rule/token-coverage.json`). No unit scoped, no deferral owed.
- **What landed:** the converter writes a **token census** (`data/sheet_rules/_tokens.json`, 49,438 entries, one per line, colon-escaped): per record, the mapping-table row key of every token its closure carried (`unmapped:<HEAD>` / `BONUS:<SUB>` with no row) and, per refusal shape, the token type it arose under (`token-less` for a record with no source row) — recorded at every refusal site of the convert loop (`ctx.carry` / `ctx.refuse_under`, `convert::token_key`, `mod::TokenCensus`). **`scripts/token_coverage.py --check`** derives `artifacts/epic-2-sheet-rule/token-coverage.json` from the census, `_refused.json`, `_report.json`, the atlas's DONE partition and `mapping-table.v1.json` — per token type: `carrying` / `carrying_non_done`, `converted_non_done`, `refused_non_done`, `refused_because_of_this_token` (all / non-DONE), `refusal_shapes`, `mapping_row`; a `refusal_shapes` section (B10); `unmapped_token_types` — and checks six sums (population, no double count, coverage, refused set == `_refused.json`, per-shape totals, per-token partition) plus committed-ledger freshness (rewrites on stale; exit 1). 14 self-tests carry the planted-double-count RED→GREEN. `verify.sh` gains `token-coverage-selftest` + `token-coverage` (46 → 48 stages; `decisions.md §3`, `technical-design.md §6` corrected). `v06_work_inventory` writes a `tokens` list on every unit from the census (lands at AT-35-E2-005's regen) so `cycle_scope_gate.py --token <type>` scopes by the converter's row. First ledger: **`non_done=23315 tokened=23308 token_less=7 refused=1810 refused_non_done=659 token_types=231 shapes=81`**, 24 token types unmapped, 44 types carry ≥ 500 non-DONE units.
- **Verification (one pass, `decisions.md §3`):** `cargo test --locked --no-run -j 6` exit 0 (121 s); `--lib` → 3214 passed / 0 failed / 14 ignored; `--no-fail-fast -j 6` → **411 binaries, 411 ok, 8,718 passed, 0 failed, 67 ignored** (2,356 s); clippy 0 warnings on `--lib --bin v06_work_inventory --bin sheet_rule_convert --test sheet_rule_convert_gate`; `sheet_rule_convert -- --check` → `records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081 verdict=PASS` (the regeneration added only `_tokens.json`); python RED (`ModuleNotFoundError`) → GREEN (`Ran 14 tests … OK`); Rust RED (9 `E0609` errors on the three missing fields) → GREEN; `verify.sh --only token-coverage-selftest --only token-coverage` → `RESULT: PASS`; atlas `unclassified=0 overlap=0 done_evidence_violations=0`; shape-engine-boundary / missing-engine-tables / denominator-gate (`files_checked=31 violations=0`) / pi-sweep green; literal scan 0; desktop and frontend at epic cadence (no `apps/` touch).
- **Discoveries (2 `correction` events, `docs/retro/events/at-35-e2-004.jsonl`):** E2-001's "82 refusal strings" is **81** (`…-df8cba`); the design's 47 stages is **48** under the selftest/gate pairing (`…-e91bf0`). Also: `FORMULA:var(COUNT)` (211) arises under `ABILITY` in 196 records — an `ABILITY` mapping question for AT-35-E4-001, not a `BONUS:VAR` one; only 7 of the 837 token-less records are non-DONE.
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-004_cycle1_receipt.md`. Code `344f18d1e1` (after `9fe67f8096`, a fold of a live `sd31-transcribe` retro append). **Epic 2: 4 of 5 complete — AT-35-E2-005 next.**

### 2026-09-08 — AT-35-E2-003 cycle 1 — `sheet-complete-status` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (status-vocabulary cycle — closes zero units by design)` — `decisions.md §2`. `pcgen_residue_gate.py --check` at start (`9c8a3abe3d`): `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=442 ratio=n/a builds_recorded=2 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 9c8a3abe3d --before /tmp/wi-before-AT-35-E2-003.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-003` at the tree of `a81c2a005c`).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `a81c2a005c` — not risen (the rung reads the converter's package and the evaluator's output only).
- **Refused tokens:** none (no converter run; `sheet_rule_convert -- --check` → `records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081 verdict=PASS`).
- **What landed:** `sheet-complete` in `v06_work_inventory`'s `status_vocabulary` with `technical-design.md §3`'s meaning; the rung `apply_sheet_complete_rung` (run last of the status passes) lifts `engine-does-not-hold`/`ingested-magnitude` units whose id has a `SheetRule` in `data/sheet_rules/`, is absent from `_refused.json`, whose kind has an on-screen test (list pinned to the frontend test's `KINDS` by reading the file), and which `rules_core::sheet_rule::evaluate` renders for the probe character (the deterministic Human Fighter 1 holding the rule outright) — evidence `sheet_rule_rendered:<number|dice|words>`; `sheet-complete` joins `DONE_RUNG_STAMP_STATUSES` (a regen on a tree missing the package fails loudly). Consumers, found by grep: `completion_atlas.py` (DONE; DONE-evidence requires the rendered-form marker), `pf1e_dashboard_producer.py` (`done` for every wiring class), `test_cycle_scope_gate.py`, `companion_chassis.rs` `HELD_STATUSES`. **The inventory is not regenerated here** — AT-35-E2-005 does that once; the rung over the live package renders all **47,628** top-level rules (number=8017 dice=1461 words=38150, 2.28 s) and the projection from the committed inventory is **21,911** units moving (17,640 `engine-does-not-hold` + 4,271 `ingested-magnitude`).
- **Verification (one pass, `decisions.md §3`):** `cargo test --locked --no-run -j 6` exit 0 (2 min 53 s); `--lib` → 3214 passed / 0 failed / 14 ignored; `--no-fail-fast -j 6` → **411 binaries, 411 ok, 8,715 passed, 0 failed, 67 ignored**; clippy 0 warnings on `--lib --bin v06_work_inventory`; python RED (`132 run, 5 failures + 1 error`) → GREEN (`143 run, OK`); atlas `unclassified=0 overlap=0 done_evidence_violations=0`; shape-engine-boundary / missing-engine-tables / denominator-gate / pi-sweep green; literal scan 0; `token_coverage.py` absent until AT-35-E2-004; desktop and frontend at epic cadence (no `apps/` touch).
- **Discoveries (2 `correction` events, `docs/retro/events/at-35-e2-003.jsonl`):** the census evidence is not a literal equality — before **6**, after **7**: the five status-branching files are in both sets, `formula_interpreter_corpus_wide.rs` (before-only) is an F1 pin-history narrative, the two after-only files are the RED→GREEN tests (`…-b92fcf`); the producer's grid test enumerated 9 of 11 vocabulary words (the two oracle words never added — now 10 of 12; `…-41c58b`). Also: 4,681 of 47,628 top-level rules are `print: false` (R1) and are stamped with their value's form.
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-003_cycle1_receipt.md`. Code `a81c2a005c` (after `f731af3759`, a fold of a live `sd31-transcribe` retro append). **Epic 2: 3 of 5 complete — AT-35-E2-004 next.**

### 2026-09-08 — AT-35-E2-002 cycle 1 — `live-evaluator-and-sheet-section` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (live-evaluator + sheet-section cycle — closes zero units by design)` — `decisions.md §2`. `pcgen_residue_gate.py --check` at start (`643cc89bba`): `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=1818 ratio=n/a builds_recorded=3 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 643cc89bba --before /tmp/wi-before-AT-35-E2-002.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-002` at `909bb0837c`).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `909bb0837c` — not risen (the evaluator, loader, DTO and section read `SheetRule`/`VarTable` JSON and the chassis output only).
- **Refused tokens:** none (no converter run; `sheet_rule_convert -- --check` → `records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081 verdict=PASS`).
- **What landed:** `src/rules_core/sheet_rule.rs` gains the evaluator (`technical-design.md §2`): exact-rational `Rat` with ONE truncation at the `SheetValue` boundary; `Expr` leaves over `CharacterFacts` (built from `CharacterInput` + `PilotBaseChassisComputation`); the two-valued `Applies` gate plus `Situational`; the `Var` contribution fold by bonus type (`STACKING_TYPES`, `Stack`, `Replace`); slot filling with family order and `pick_last` / `suppress_when_all_zero`; dice folding and the damage-die ladder; the held-set fixpoint over the seed (race, classes, feats, traits, equipment, spells, skills, the chassis' grounded `class_feature.*` records joined by `<class>_<feature>` slug, the race resolver's applied-trait keys) with `Rule`/`Class`/`Race`/`Deity`/`Choice` grants gated by `when` + `applies`, `FactDeclare`, `CountsAs`, `Waives`/`Revokes`; `render_sheet` sorted by kind then label. `corpus_loader::load_sheet_rules` reads `data/sheet_rules/` (47,628 rule files + 5,081 `_vars/` → 66,147 rules in 2.66 s debug, parallel). `PilotBaseChassisComputation.sheet_lines` + `with_sheet_rules`. Desktop: `SheetLineDto`, the package loaded once per process, `LoadSavedCharacterResponse.sheet_lines` / `sheet_rules_unavailable_reason` on both response constructors; `CharacterSheet.tsx` renders one generic **Rules and features** section grouped by kind in the Actions tab (a `words` line renders no number); `buildClassFeatureSurface(..., sheetLines)` keeps only records with no rule in the `Not computed` lane.
- **Verification (one pass, `decisions.md §3`):** `cargo test --locked --no-run -j 6` exit 0; `--lib` → 3214 passed / 0 failed / 14 ignored; `--no-fail-fast -j 6` → **411 binaries, 411 ok, 8,710 passed, 0 failed, 67 ignored**; `cargo test --locked --lib sheet_rule` → 28 passed (the three value forms on real records — Ill Omen `DC 13` at Cha 14 / `Resolved(15)` on the design shape at spell level 3; Longsword `1d8` / `1d8+2` / `1d8+4` (Str 18) / `1d10` (one step); Magical Knack `Words` → `Wizard`; the per-kind gate over all 66,147 rules of 19 kinds; fixpoint; var fold; the fixture fighter's 45 lines with Acrobatic `+4` at 10 ranks); frontend `101/101` files (`rulesAndFeaturesSection.test.ts`: 19 per-kind DOM tests + 5; `classFeaturesModel.test.ts` +1), `tsc --noEmit` clean; desktop crate `574 passed / 0 failed` incl. the reach-gate IPC test on a Human Fighter 3 created through `create_character_at_root`; clippy 0 warnings on the lib and the desktop tests (3 `should_implement_trait` fixed in-cycle); atlas / shape-engine-boundary / missing-engine-tables / denominator-gate / pi-sweep green; literal scan 0; `token_coverage.py` absent until AT-35-E2-004.
- **Discoveries (3 `correction` events, `docs/retro/events/at-35-e2-002.jsonl`):** the chassis output carries fewer leaf facts than `technical-design.md §2` claims (size from `race_tables::race_size` for the 7 CRB races, walk speed from the `race.<slug>.trait_bundle.speed` record, the rest 0 and named in `CharacterFacts::from_character`; `…-796340`); the package carries no `Granter::Class`/`Race` rows (`python3` census over `data/sheet_rules/{core_rulebook,advanced_players_guide,bestiary,ultimate_psionics}/*/*.json` `granted_by`: `Rule` 8,103, `ClassSpellList` 5,149, `Deity` 2,126) and 1,187 of 1,738 CRB `class_feature` rules have no `granted_by` (`…-decb80`) — placement is bridged from the engine until AT-35-E3-001; the §6 wired-integration grep matches 3 rulebook-prose lines in the generated package, none in code (`…-7cbeb2`). Also: templates are not universal (the first fixpoint held every ungranted `applies: Always` template — fixed, pinned); `fighter_bonus_feats` prints 23 lines (22 `#bonusN` siblings) — AT-35-E3-001's placement shape.
- **Gate self-heal:** `verify.sh --only figure-provenance` was red on one pre-existing AT-35-E2-001 receipt line (`~23 s` with no command) — command added in this cycle's docs commit; `violations=0` after.
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-002_cycle1_receipt.md`. Code `909bb0837c` (after `097e7c1aa5`, a fold of a live `sd31-transcribe` retro append). **Epic 2: 2 of 5 complete — AT-35-E2-003 next.**

### 2026-09-08 — AT-35-E2-001 cycle 1 — `sheet-rule-converter` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (converter-building cycle — closes zero units by design; AT-35-E2-005 is the pass that moves the population)` — `decisions.md §2`. `pcgen_residue_gate.py --check` at start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=6463 ratio=n/a builds_recorded=3 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 928272a444 --before /tmp/wi-before-AT-35-E2-001.json --after docs/work-inventory.json`; `builds_recorded` 3: a converter-building cycle iterates build → run → `--check` until the package is clean).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `72ad0be010` — not risen.
- **Refused tokens:** 1,810 of 49,438 records (659 non-DONE of 23,315) in 82 refusal strings / ~16 shapes — `no_corpus_record=829, FORMULA:var(COUNT)=211, unmapped:STARTSKILLPTS=162, unmapped:SLOTS=95, FORMULA:malformed=87, SPELLS (PI-redacted)=78, BONUS:[redacted PI]=62, unmapped:ALTTYPE=49, unmapped:SPELLSTAT=49, unmapped:MODTOSKILLS=41, DEFINE (PI-redacted)=40, unmapped:MEMORIZE=38, …` (full list: `data/sheet_rules/_refused.json`; deferral event `1788840316824-at-35-e2-001-de4597`).
- **What landed:** `src/rules_core/sheet_rule.rs` (schema v2, no source-format reading), `src/pcgen_import/sheet_rule/` (the converter: 249-row table transcribed and proven both ways; corrected corpus-wide `.MOD` index with `_pfs/` skipped and KEY matching — B9/R3; class continuation rows + level lines — B5; the no-`raw_tokens` row read — B7; the pinned DEFINE/BONUS:VAR index — B2; own formula parser; PRE → `Applies`; prose slots; PI term screen on every text), `src/bin/sheet_rule_convert.rs` (regenerate / `--check` / `--one`), `data/sheet_rules/` (`records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081`, ~23 s per pass; literal grep 0), `tests/sheet_rule_convert_gate.rs` (26 tests: Ill Omen's DC = `Sum([Const(10), Const(1), AbilityMod(Cha)])`, the longsword = `Dice{"1d8", None}`, Magical Knack = `Text` + `offers` + `ChoiceName`; 19 per-kind gates; literal scan; freshness; determinism), `verify.sh` stage `sheet-rules-check` (46 stages).
- **Verification:** `cargo test --locked --no-run` exit 0; `cargo test --locked --no-fail-fast -j 6` → 411 test binaries, 410 ok, 1 FAILED — the lib binary at the pre-fix tree (3203 passed, 3 failed: `sheet_rule::{ctx,formula,table}::tests` expectation drift, fixed in the same cycle); re-run at HEAD `cargo test --locked --lib -j 6` → 3206 passed, 0 failed, 14 ignored; 8,699 tests passed across the run; converter gate binary 26/26; clippy 0 warnings on the touched targets; `sheet_rule_convert --check` `verdict=PASS`; `completion_atlas.py --check` `unclassified=0` (atlas `derived_at` re-stamp reverted); `shape_engine_boundary.py`, `missing_engine_tables.py` `violations=0`; `denominator_gate.py` `files_checked=29 violations=0`; `verify.sh --only pi-sweep` PASS.
- **Discoveries:** 24 token heads outside the mapping table once the closure is read from the pinned tree (STARTSKILLPTS 162, SLOTS 95, ALTTYPE 49, SPELLSTAT 49, MODTOSKILLS 41, MEMORIZE 38, …; correction `1788840316691-at-35-e2-001-4e50ce`); the epic's `SpellLevel` is the table's `Const(<spell level>)` (`…-8ed3da`); no corpus record carries a `+N` die literal (`…-569089`); refusal shapes > 10 by construction of the converter-building cycle — the §8 signal for AT-35-E4-001's scoping, not a blocker on this criterion.
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-001_cycle1_receipt.md`. Code `72ad0be010`. **Epic 2: 1 of 5 complete — AT-35-E2-002 next.**

### 2026-09-08 — AT-35-E1-004 cycle 1 — `ratio-row-and-gate-scope` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-retargeting cycle — closes zero units by design, decisions.md §2)` — `cycle_scope_gate.py` present; exemption by design.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since d1b5738658 --before /tmp/wi-before-AT-35-E1-004.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E1-004` at `2bf452b038`).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `2bf452b038` — not risen.
- **Refused tokens:** none (no units scoped).
- **What moved:** `denominator_gate.py`'s `DEFAULT_GLOBS` and `PROVENANCE_DEFAULT_GLOBS` each gained SD-35's root `*.md` + `artifacts/**/*.md`; SD-33/SD-34 entries untouched (frozen pre-widening lists in the test). `verify.sh --only denominator-gate` → `files_checked=213 violations=0` (186 before + 27 of 27 SD-35 `.md`); `--only figure-provenance` → `files_checked=143 figures_examined=139 violations=0`. Tests 46 → 54 (RED 6 errors first). No stage added (45 / 39).
- **Discoveries:** the criterion's "never advanced to SD-34" was stale (AT-34-E1-006 had); SD-33 cannot enter the provenance default (44 violations of 137 figures in 78 files, out of write scope) — both emitted as `correction` events in `docs/retro/events/at-35-e1-004.jsonl`; the token-mapping denominator red E1-003/E1-005 saw was already cleared by E1-002 (`815139fadd`).
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-004_cycle1_receipt.md`. Code `2bf452b038`. **Epic 1 is 6 of 6 complete — wrap-up (§10) next.**

### 2026-09-08 — AT-35-E1-003 cycle 1 — `test-families-table-driven` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (build-time tax cut — closes zero corpus units by design, decisions.md §2)` — `cycle_scope_gate.py` was absent at cycle start (`53296d80f0`) and arrived via AT-35-E1-001 on the pre-push rebase.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=1906 ratio=n/a builds_recorded=0 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 53296d80f0 --before /tmp/wi-before-AT-35-E1-003.json --after docs/work-inventory.json` at `03072aea0c`; `builds_recorded=0` reads the deleted target dirs — the transcripts show 4 cold measurement compiles + 1 verify session, receipt row for the breakdown).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `03072aea0c` — not risen.
- **Refused tokens:** none (no converter run).
- **What moved:** 184 integration-test binaries → 2 (`tests/sd18_widening/`, `tests/sd13_progression/`, one module per original file behind a class-keyed roster table); `cargo test -- --list` name-by-name diff 8,723 = 8,723 `IDENTICAL`; `BASELINE_ROOT_TEST_BINARIES` 589 → 408, test floors unchanged (8,656 / 3,186). Cold `cargo test --locked --no-run -j 6`, paired quiet-box: 3:08.97 → 2:25.89 (−43.08 s of 188.97 s); full suite after: 8,656 passed / 0 failed / 67 ignored across 408 targets in 37.6 min; clippy on both targets 0 warnings.
- **Discoveries:** the package's `denominator_gate.py` artifacts glob was already red at cycle start (11 violations, all in `epic-2-sheet-rule/token-mapping/`, for AT-35-E1-004); the criterion's "~80k of 187k" is 74,932 of 180,260; 244 `src/`+`docs/architecture/` citations of the old test paths deferred to AT-35-E7-003 (retro events in `docs/retro/events/at-35-e1-003.jsonl`).
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-003_cycle1_receipt.md`; `build-time.json`, `test-list-diff.txt` beside it. Code `03072aea0c`.

### 2026-09-07 — AT-35-E1-002 cycle 1 — `content-anchored-citations` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (instrument-hardening cycle — closes zero units by design, decisions.md §2)` — `cycle_scope_gate.py` was absent at cycle start (`53296d80f0`) and arrived via AT-35-E1-001 on the pre-push rebase.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 53296d80f0 --before /tmp/wi-before-AT-35-E1-002.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E1-002`, at the rebased HEAD with AT-35-E1-005's gate present; coarse-grep stand-in read 78 at start and end).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` (`python3 scripts/pcgen_residue_gate.py --check` at the rebased HEAD) — equal to E1-005's first recording, not risen.
- **Refused tokens:** none
- **Commits:** `815139fadd` (code, artifacts, retro events; rebased onto AT-35-E1-001/E1-006/E1-005), plus this entry's commit (receipt, progress, kanban). Receipt: `artifacts/epic-1-tax-cut/AT-35-E1-002_cycle1_receipt.md`.
- **What landed:** 16 `file:line` pins → 16 content anchors across the three citation instruments, one shared resolver (`completion_atlas.resolve_content_anchor`); `--by-kind` / `--by-evidence`; `verify.sh` +3 stages (`shape-engine-boundary-selftest`, `shape-engine-boundary`, `missing-engine-tables`) — 42 → **45** after the rebases onto E1-001's `cycle-scope-gate-selftest` and E1-005's `pcgen-residue-gate` (`scripts/verify.sh --list | tail -n +2 | wc -l`); RED→GREEN transcript on the live engine source in `artifacts/epic-1-tax-cut/citation-anchor-proofs.md` (move 50 lines: all resolved lines +50, all green; change one condition each: all three fail closed and the two stages FAIL).
- **Discoveries (2 `correction` events):** `site-dashboard-check` already wrapped in `timeout` since AT-34-E6-001 wave 27 — nothing to add, D1.2 row updated; §6 step 3's denominator gate was red at cycle start on 11 pre-launch token-mapping lines (`artifacts/**` glob; the launch audit scanned the package root only — the same finding E1-001, E1-005 and E1-006 recorded and left) — **fixed here**, no figure changed, `denominator-gate` stage now `files_checked=186 violations=0`.

### 2026-09-07 — AT-35-E1-005 cycle 1 — the PCGen residue gate exists, baseline recorded — `complete`

- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-building cycle — this cycle CREATES pcgen_residue_gate.py; closes zero units by design, decisions.md §2)`. `scripts/cycle_scope_gate.py` was absent in the cycle's tree at `53296d80f0` (AT-35-E1-001 landed on `origin/tranche/15` while this cycle ran; the receipt rows below were re-derived with it after the rebase).
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260` — see the receipt for the mechanical `cycle_scope_gate.py --receipt` line run after the rebase, and the hand commands it agrees with.
- **PCGen residue (first recording, `53296d80f0`):** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` (`python3 scripts/pcgen_residue_gate.py --check`). `identifier_files=68 identifier_hits=514` of those 260 files are the five-identifier readers the authoring-time "78 files" figure was counting (that grep scanned `apps/desktop/src-tauri/src` only and included `cache_gen/`); the rest is token-syntax literals, 8,078 `PRE[A-Z]+:` + 2,403 `BONUS:` hits of 12,736, almost all in generated `src/rules_core/rules_tables/**`. Correction event `1788831658230-at-35-e1-005-0d124e`.
- **RED→GREEN on the real tree:** planted `src/rules_core/zz_planted_residue_probe.rs` (`r.raw_tokens.len()`) → `live_files=261 live_hits=12737 baseline_files=260 baseline_hits=12736 verdict=FAIL_INCREASED` exit 1; removed → `verdict=PASS` exit 0. `--check --closure` at the baseline → `live_files=260 live_hits=12736 verdict=FAIL` exit 1. `--rebaseline` at the baseline → `rebaseline=REFUSED ... verdict=FAIL_NOT_REDUCED` exit 1.
- **verify.sh:** stage `pcgen-residue-gate` in both sets after `figure-provenance` — `scripts/verify.sh --list` → 42 stages (36 quick) with AT-35-E1-001's `cycle-scope-gate-selftest` landed first; `--only pcgen-residue-gate` → `PASS`; `PCGEN_RESIDUE_GATE_CLOSURE=1 ... --only pcgen-residue-gate` → `FAIL` (closure mode, for AT-35-E6-004). `scripts/tests/test_pcgen_residue_gate.py` → `Ran 15 tests OK` (RED first: `ModuleNotFoundError`).
- **Build:** `cargo test --locked --no-run -j 6` exit 0; `--lib` 3186 passed / 0 failed (= floor); `--no-fail-fast`, desktop, frontend, clippy not run — no Rust touched (`git diff --stat 53296d80f0 -- '*.rs'` empty). Fast gates green: atlas, shape-engine-boundary, missing-engine-tables, pi-sweep; `data/sheet_rules/` grep → 0 (directory not yet created). **`denominator_gate.py --check` over `*.md` + `artifacts/**/*.md` is RED on inherited prose:** `files_checked=22 violations=11`, all 11 in `artifacts/epic-2-sheet-rule/token-mapping/` (committed `a232e27b03`, pre-cycle, outside Epic 1's touch set); this cycle's own three files → `violations=0`. Incident event emitted (the same finding AT-35-E1-001 and AT-35-E1-006 recorded); owner: the token-mapping synthesis / AT-35-E1-004.
- **Refused tokens:** none. Receipt: `artifacts/epic-1-tax-cut/AT-35-E1-005_cycle1_receipt.md` (names the code and docs SHAs); transcript: `artifacts/epic-1-tax-cut/pcgen-residue-first-run.txt`.

### Cycle — AT-35-E1-006 cycle 1 — SD-34's unrun closure folded: retrospective written and cited, 17 open rows and 29 open deferrals dispositioned — complete (2026-09-08)

**Status: complete.** Docs only; zero units moved by design. Work commit `9cc73dca76`, receipt
commit `800c363e42`; receipt `artifacts/epic-1-tax-cut/AT-35-E1-006_cycle1_receipt.md`.

- **Scope gate:** `SCOPE_GATE: EXEMPT (docs-only fold of SD-34's closure epilogue — closes zero units by design, decisions.md §2 and §12)` — `cycle_scope_gate.py` was absent at cycle start (`53296d80f0`) and arrived via AT-35-E1-001 on the pre-push rebase.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=unavailable` (`cycle_scope_gate.py --receipt --since 53296d80f0 --target-dir /tmp/cargo-sd35-AT-35-E1-006`; `pcgen_residue_gate.py` absent — AT-35-E1-005 builds it; coarse-grep stand-in 78 files, `content-unit-inventory.md §6`).
- **Refused tokens:** none.
- **Evidence:** `test -f docs/retro/sd34-book-completion-retrospective.md` → present; `grep -c sd34-book-completion-retrospective` → 1 in each `references/README.md`; row map 1,590 of 1,590 (core_rulebook 1,529 of 6,701 + ultimate_campaign 61 of 265, `completion_atlas.py --book <book> --check`) sum-checked; SD-34 `progress.md` `status: closed-by-fold`; 29 of 29 deferrals dispositioned (8 resolved with a SHA, 1 superseded by register C2.5, 20 mapped) — `retro.py summary --since 2026-08-27` now reads `deferrals.open=20`, all SD-35-owned.
- **Gates:** `completion_atlas.py --check` 0; `shape_engine_boundary.py --check` 0; `missing_engine_tables.py --check` 0; `verify.sh --only pi-sweep` PASS; **`denominator_gate.py --check` red — 11 violations in 4 pre-existing `artifacts/epic-2-sheet-rule/token-mapping/*.md` files this cycle did not touch** (incident `1788831974625-at-35-e1-006-ae4135`; owner: the token-mapping synthesis / AT-35-E2-001).
- **Carried one-liner** (`SD-34 forward-scope-register.md` C1.8, `358a71516f`): `monk_ki_pool` "size"-suffix — unit is bucket C at the cut, owned by AT-35-E3-003; not applied here (docs only).
- **Operator attention:** the two fable-review P1s (R11-01, R14-02) are unfixed and outside SD-35 scope per `forward-scope-register.md` C2.5 — dispositioned `superseded` (register), not resolved.

### 2026-09-07 — AT-35-E1-001 cycle 1 — `batch-floor-gate` — complete

- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-building cycle — this cycle CREATES cycle_scope_gate.py; it closes zero units by design, decisions.md §2)`
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=unavailable` (`pcgen_residue_gate.py` not yet in the tree — AT-35-E1-005)
- **Refused tokens:** none
- **Landed:** `scripts/cycle_scope_gate.py` (floor + `--receipt`), `scripts/tests/test_cycle_scope_gate.py` (51 cases), `verify.sh` stage `cycle-scope-gate-selftest` (stage count 40 → 41; `scripts/verify.sh --list`). Live at HEAD: `--bucket B --kind class_feature` → `scoped=7866 remaining_non_done=23315 verdict=PASS`; `--bucket A --kind companion` → `scoped=28 ... FAIL_UNDER_FLOOR` exit 1 (`python3 scripts/cycle_scope_gate.py --min 500 ...`).
- **Found, not fixed (outside file-touch set):** `denominator_gate.py --check` on the package is `violations=11` of `files_checked=21`, all in 4 pre-launch `artifacts/epic-2-sheet-rule/token-mapping/*.md` files — retro incident `denominator-gate-red-on-package-prose`; owner AT-35-E1-004.
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-001_cycle1_receipt.md` — code at `1d821cdc8d`.
