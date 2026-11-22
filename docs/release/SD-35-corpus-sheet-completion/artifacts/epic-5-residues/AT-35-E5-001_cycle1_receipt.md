# Cycle AT-35-E5-001_cycle1 — Epic 5 Residues / AT-35-E5-001

- **Commit SHA:** `7a0bf64bbf` (code + artifact), `9123db2955` (baselines + progress/kanban/receipt). Cycle start `5e2c0c8c5b`.
- **Scope gate:**
  ```
  inventory=docs/work-inventory.json
  scope=bucket=A
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  Run as `python3 scripts/cycle_scope_gate.py --min 500 --bucket A`. The gate does not
  refuse: `scoped=0` **is** the whole remainder (`remaining_non_done=0` for the entire
  corpus, not merely for bucket A), so the verdict is `PASS_WHOLE_REMAINDER`, not
  `FAIL_UNDER_FLOOR`. The dispatch's mandatory-bundling instruction is therefore moot —
  `python3 scripts/cycle_scope_gate.py --min 500` with no scope flags returns the identical
  line. There were no units left anywhere to bundle in. Not an exemption claim: the gate ran
  and passed.
- **Files touched:**
  - `src/bin/v06_work_inventory.rs` — `--epic5-table-transcript`, `EPIC5_TABLES`,
    `epic5_table_transcript_pair`, `epic5_applies_variant`, `epic5_pcgen_markers_in`, and three
    tests in `apply_sheet_complete_rung_tests`
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/table-proofs.md` (new — the criterion's named artifact)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-001_cycle1_receipt.md` (new — this file)
  - `docs/retro/events/at-35-e5-001.jsonl` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` — `derived_at` stamp only, moved by running `completion_atlas.py --check`
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `kanban.md`
  - `scripts/verify-baselines.env` — `BASELINE_ROOT_FULL_TESTS` 8727 → 8730, with the +3 attribution (`§8` self-heal; see **Build scope verified**)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — on the full
  `git diff --unified=0 "$(git merge-base HEAD origin/develop)...HEAD"` over the epic's
  scoped paths, and on this cycle's own change in isolation.
- **Wired-integration audit result:** `OK_NO_TOKENS` **on this cycle's own change**. The
  bundle-wide diff over the scoped paths reports 6 hits, all pre-existing and all in
  generated data, never in code: `data/sheet_rules/` 3 and `docs/work-inventory.json` 3
  (the same 3 records, in both files). All 6 are English rule prose transcribed from the
  corpus — "creatures must hack or force a way through" (`core_rulebook:spell:plant_growth`,
  and the same word in `bestiary_3:monster_ability:tophet_swallow_whole`), PCGen's own
  `Empty Selection ~ Standard <class>` rows which our converter *names* as placeholder rows,
  and the upstream editorial note `[Change to magical beast and stacking restriction not yet
  implemented]` in `ultimate_intrigue:class_feature:courtly_hunter_courtly_companion`.
  Proven corpus-sourced, not injected by us:
  `grep -rl "not yet implemented" data/corpus/ultimate_intrigue/` finds
  `data/corpus/ultimate_intrigue/class_feature/courtly_hunter/courtly_companion.json`, and
  `grep -rn "not yet implemented" --include=*.rs src/` finds nothing. No stub in shipping code.
  Per-file counts re-derivable with
  `git diff --unified=0 "$(git merge-base HEAD origin/develop)...HEAD" -- <path> | grep -cE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`.

  Re-run at step 4 over this cycle's own new files (with `git add -N`, per
  `AT-35-E4-003`'s correction `1788959112531-at-35-e4-003-d78240`: an untracked file is
  invisible to `git diff`, so the audit run as written on a new-files-only cycle reads a
  false `OK_NO_TOKENS` over an empty diff). Over
  `artifacts/epic-5-residues/`: **0 hits in `table-proofs.md`**, and 5 hits in this receipt —
  all of them in this very audit row, which quotes the token list and the offending corpus
  phrases in order to characterise them. Self-referential prose describing the audit, the
  same shape `AT-35-E4-003`'s receipt recorded.
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E5-001`):
  > ### AT-35-E5-001 — bucket A reaches zero: the `power` and `companion` tables
  >
  > `power` (421, `ultimate_psionics`) and `companion` widening (28, `bestiary`). The tables load
  > `SheetRule.applies`, not tokens. Fail-closed: real record or named refusal.
  >
  > **Evidence:** `python3 scripts/missing_engine_tables.py --check` reports `population=0`; the
  > refusal/success transcript pair.
- **Receipt rows (mechanical):**
  ```
  since=5e2c0c8c5bac24cf1ffdb84df1badb0ed09da49a target_dir=... residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=252 ratio=n/a builds_recorded=0 pcgen_live_files=260
  ```
  `closed=0` is correct and expected: the criterion's units were closed at `406003afc3` by
  `AT-35-E3-001` cycle 2's bundle. This cycle moved no unit; it paid the unpaid half of the
  Evidence sentence. `ratio=n/a` for the same reason — there is no units-closed denominator
  in this cycle, and `decisions.md §4`'s ratio review does not apply to a cycle that closes
  nothing. `builds_recorded=0` is an instrument artefact: the row reads a `target_dir` that
  is not this cycle's `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E5-001`; the builds this cycle
  actually ran are listed under **Build scope verified**.
- **PCGen residue:**
  ```
  live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Unchanged from the previous receipt's 260. Nothing new on the live side reads a PCGen
  token; the transcript reads the converted sheet-rule package, and
  `src/bin/` is not a residue root in any case.
- **Oracle parity:** N/A — this cycle added no `Number` mapping and touched no live path.
  The transcript renders through the existing evaluator without changing it.
- **Movement, four buckets:**
  - *closure (into DONE, by id-set):* none this cycle. Bucket A's 449 units closed earlier:
    421 `power` (`ultimate_psionics`) + 28 `companion` (`bestiary`), all at `406003afc3`.
  - *relabel (bucket to bucket):* none.
  - *reachability:* none.
  - *instrument-correction:* none to any counting instrument. One board correction: kanban
    row 19 read `complete` while its second Evidence clause was unpaid — emitted as retro
    `correction` `1788960015819-at-35-e5-001-d79575`.
- **Refused tokens:** none.
- **Discoveries:** one, and it is the reason this cycle exists — a criterion's row can reach
  `complete` on the strength of one clause of a two-clause Evidence sentence, because the
  clause that has a `--check` command gets run and the clause that names an *artifact* does
  not. Nothing mechanical was watching
  `acceptance-and-verification.md`'s artifact column. Emitted as the `correction` event
  above; not a token-shaped or atlas-shaped discovery, so no re-derivation was needed.
- **Figures + their re-derive commands:**
  | Figure | Value | Denominator | Command |
  |---|---|---|---|
  | bucket A population | 0 | of 49,438 corpus units | `python3 scripts/missing_engine_tables.py --check` |
  | bucket A kinds | 0 | of 2 cited (`power`, `companion`) | same |
  | citation failures | 0 | of 2 content anchors | same |
  | corpus DONE | 49438 | of 49438 | `python3 scripts/completion_atlas.py --check` |
  | `power` units, `ultimate_psionics` | 421, all `sheet-complete` | of 421 (412 `words` + 9 `number`) | command A in `table-proofs.md` §2 |
  | `companion` units `sheet-complete`, `bestiary` | 28 | of 154 `bestiary` `companion` units | command A in `table-proofs.md` §2 |
  | `power` rules in the package | 447 | 421 principal + 26 `#suffix` siblings | the transcript's `records=` field |
  | `companion` rules in the package | 450 | 154 principal + 296 siblings | the transcript's `records=` field |
  | rust lines changed | 252 | this cycle, since `5e2c0c8c5b` | `python3 scripts/cycle_scope_gate.py --receipt --since 5e2c0c8c5b ...` |
  | source-format markers in `data/sheet_rules/` | 0 | files matching | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | denominator gate | `violations=0` | of 58 files checked | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
- **Build scope verified:** `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E5-001`, `CARGO_INCREMENTAL=0`, `-j 6`, run at `7a0bf64bbf`.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --no-fail-fast -j 6` → `FULL_EXIT=0`; **8730 passed, 0 failed, 67 ignored across 412 suites executed** (413 `test result:` lines incl. doc-tests), 0 `FAILED` lines. Measured with `verify.sh`'s own `count_passed` / `count_running` functions over the run log.
    Raised `BASELINE_ROOT_FULL_TESTS` 8727 → **8730** in `scripts/verify-baselines.env` — `+3`, exactly this cycle's three new `#[test]` functions (`git show 7a0bf64bbf -- '*.rs' | grep -c '^+\s*#\[test\]'` → 3). `BASELINE_ROOT_TEST_BINARIES` deliberately NOT raised: 412 measured vs 411 recorded, and that `+1` predates this cycle (no new test FILE was added — the three tests are in an existing bin's existing test module). `workflow-instruction.md §8` self-heal: "a count assertion your own change moved (update it in the same commit)".
  - `cargo test --locked --bin v06_work_inventory -j 6 epic5` → 3 passed, 0 failed
  - `cargo clippy --locked --tests -j 6 --bin v06_work_inventory` → exit 0, 0 warnings, 0 errors
  - `cargo run --locked --bin sheet_rule_convert -- --check` → exit 0, `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS`
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (11 hits over `src/rules_core/rules_tables`, 11 baseline rows)
  - desktop crate + frontend: **epic cadence** — this cycle touched no `apps/` path.
  - Build count actually run: 1 cold library build, 1 test build, 3 mutation test builds, 1 full-workspace test build.
- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` was not
  re-run (`workflow-instruction.md §6` step 3 runs it only when corpus records changed).
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — carried in
  every transcribed record's `provenance.oracle_pin`; no figure in this receipt was derived
  from the pinned corpus.
- **Status:** complete
- **Notes:** The criterion's first Evidence clause was already true at dispatch; the second
  ("the refusal/success transcript pair", artifact
  `artifacts/epic-5-residues/table-proofs.md` per `acceptance-and-verification.md`) was
  unpaid — the artifact directory held only `.gitkeep`. This cycle paid it and left the
  first clause untouched. `citation_failures=0` alongside `population=0` is not a
  contradiction: the two `engine_does_not_hold("<kind>_content_has_no_engine_table")` arms
  still exist and still resolve; no unit reaches them because the `sheet-complete` rung
  fires first. No other criterion's population was emptied by this cycle, so no other
  kanban row was closed.
- **Next-cycle scope:** criterion at zero.
