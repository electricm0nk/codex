# Cycle AT-35-E5-005_cycle2 — Epic 5 — Residues / AT-35-E5-005

A re-dispatch of an already-closed criterion. Cycle 1 closed it at `103693b365` and `kanban.md`
row 23 has read `complete` since. This cycle writes **no code**, moves **no unit** and redoes
**nothing**: it re-derives all three of the criterion's Evidence clauses at HEAD, records that
each reproduces, and files the extra cycle as `§5` requires (one row per extra cycle).

- **Commit SHA:** `392fc95e33` (this receipt, `progress.md`, `kanban.md`, the two retro events,
  and the `derived_at` re-stamp `completion_atlas.py --check` writes into
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`). Cycle start
  `5e68380ac47bdd36f45a080254b4a008c18e4200` on `tranche/15`.
- **Scope gate:** the dispatch flagged this cycle
  `SCOPE_GATE: EXEMPT (closure-accounting cycle — it proves zero remains; if any bucket is non-zero this cycle STOPS and reports, it does not carve out)`.
  **The exemption was not taken.** The gate was run and passed, returning the stronger statement —
  the literal last line of `python3 scripts/cycle_scope_gate.py --min 500` at `5e68380ac4`
  (exit 0), with `scope=(whole remainder)`, `scoped_by_bucket=` empty and `scoped_by_kind=` empty:
  ```
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` is corpus-wide, not bucket-scoped: there was nothing anywhere to bundle
  in, so the floor's mandatory-bundling instruction is moot. `python3 scripts/pcgen_residue_gate.py --check`
  at cycle start: `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Files touched:** this receipt (new),
  `docs/release/SD-35-corpus-sheet-completion/progress.md` (prepended entry),
  `docs/release/SD-35-corpus-sheet-completion/kanban.md` (row 41 + header count),
  `docs/retro/events/at-35-e5-005.jsonl` (1 `correction` + 1 `deferral`),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (`derived_at`
  re-stamped `14f37178a0` → `5e68380ac4` by `completion_atlas.py --check`'s own write — a
  **one-line** diff, kept rather than reverted because reverting leaves the gate's
  `stale_derived_at` red; the same disposition `7557ab00fa` and cycle 1 took).
  **No file under `src/`, `apps/`, `data/`, `tests/` or `scripts/` was written**
  (`git diff --name-only 5e68380ac4..HEAD -- src apps data tests scripts` is empty).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS — **0 from this cycle**.
  `BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47` (`git merge-base HEAD origin/develop`).
  Over Epic 5's whole file-touch set the grep returns **15** at cycle start and **17** on the
  final diff at `392fc95e33`; the **+2** are this receipt's own two quotations of a pre-existing
  test filename, and `git diff --unified=0 5e68380ac4..HEAD -- src/ apps/ data/ | grep -c` over
  both audit patterns returns **0**. Attributed exhaustively (cycle-start figures): `src/rules_core/` **2**
  (`tests/sd27_feat_prerequisite_enforcement.rs` on a removed doc-comment line,
  `tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs` on an added one),
  `artifacts/epic-5-residues/` **13** (earlier receipts' prose quoting those same pre-existing
  test filenames), and **0** each for the picker, `LevelUpDialog.tsx`,
  `data/corpus/beginner_box/`, `data/sheet_rules/` and `docs/work-inventory.json`. Every one is a
  reference to a test file that already exists; none is a bundle tag on a new identifier. The
  rise from AT-35-E5-004 cycle 2's **12** is exactly the **3** such quotations that receipt itself
  added.
- **Wired-integration audit result:** OK_NO_TOKENS — **0 code hits from this cycle**, which ships
  no code. Over Epic 5's whole file-touch set the grep
  (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`) returns **27** at cycle
  start and **32** on the final diff at `392fc95e33`; the **+5** are this receipt's own
  self-referential quotations of the pattern, and this cycle's diff over `src/ apps/ data/`
  carries **0**. Attributed exhaustively (cycle-start figures): `data/sheet_rules/` **2** —
  published rulebook prose inside `ProsePiece::Text` (Tophet "hack or smash", Plant Growth
  "creatures must hack or force a way through"), which is the sheet rule working, not a stub;
  `docs/work-inventory.json` **3** — all on **removed** (`-`) lines carrying PCGen's own
  CHOOSE-menu "Empty Selection" placeholder reason strings, and
  `git show HEAD:docs/work-inventory.json | grep -cE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → **0**, so the inventory at HEAD carries none; `artifacts/epic-5-residues/` **22** — receipt
  prose, including the sentence you are reading, which quotes the pattern in order to characterise
  it. `src/rules_core/`, the picker, `LevelUpDialog.tsx` and `data/corpus/beginner_box/` return
  **0** each. **No stub, inline mock or "Would …" string in shipping code.** The rise from
  AT-35-E5-004 cycle 2's **24** is exactly the **3** self-referential quotations that receipt
  itself added, by the same mechanism.
- **Acceptance criterion:** verbatim from `epic-breakdown.md` `### AT-35-E5-005`:
  "**AT-35-E5-005 — the corpus reaches 49,438 of 49,438, and the capability register is closed.**
  **Evidence:** `completion_atlas.py --check` → `DONE=49438 of 49438`, every other bucket zero.
  `artifacts/epic-5-residues/completion-manifest.json` — one row per unit. SD-34's
  `capability-register.json` re-derived: every row `built: true` or
  `unnecessary-under-sheet-rule: <reason>`."
- **Receipt rows (mechanical):** the literal last line of
  `python3 scripts/cycle_scope_gate.py --receipt --since 5e68380ac47bdd36f45a080254b4a008c18e4200 --before /tmp/wi-before-AT-35-E5-005.json --after docs/work-inventory.json`
  (header `since=5e68380ac47bdd36f45a080254b4a008c18e4200 residue_gate=present`, `closed_by_kind=`
  and `relabeled_moves=` empty, `regressed=0 added=0 dropped=0`):
  ```
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=253
  ```
  `closed=0` over a **scoped population of 0** is the whole population, not a shortfall: no
  `deferral` is owed on that account, and none was emitted for it. (One `deferral` **was**
  emitted, for a different and pre-existing thing — see Discoveries.) `builds_recorded=1` is this
  cycle's single cold compile session in `/tmp/cargo-sd35-AT-35-E5-005`, spent re-deriving
  Evidence rather than changing anything, which is why `rust_lines_changed=0` sits beside it.
  `pcgen_live_files=253` is **below** cycle 1's `260` and identical to AT-35-E5-001/002/003/004
  cycle 2.
- **PCGen residue:** `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — identical at cycle start and at HEAD, and **below** the `260 / 12736` baseline cycle 1
  recorded. **Not risen.** This cycle writes no live-side file, so it could not raise it.
- **Oracle parity:** N/A — no `Number` mapping was added, no live path was touched, and no figure
  in this receipt was taken from the pinned tree. The bundle's bucket-V oracle comparison was paid
  once by AT-35-E4-002 (`compared=392 oracle_agree=184 oracle_disagreement=10 oracle_unverifiable=198`
  at `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`).
- **Movement, four buckets:**
  - **closure (into DONE, by id-set): 0.** Non-DONE was 0 of 49,438 at cycle start and 0 at HEAD.
  - **relabel (bucket to bucket): 0.**
  - **reachability: 0 regressions** — no unit left DONE (`regressed=0 added=0 dropped=0`).
  - **instrument-correction: 1**, emitted as a `correction` event with `--verified-by`.
- **Refused tokens:** none. `python3 scripts/token_coverage.py --check` at HEAD →
  `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS`
  — the 142 converter-refused records of 49,438 all sit in DONE units
  (`refused_non_done=0`), so no refusal stands against this criterion's population.
  **The named, summing residue cycle 1 reported is unchanged and still open, and it is neither a
  refused token nor a carve-out:** `desc_token_present_but_no_prose_on_the_sheet_rule=10` — see
  Discoveries.
- **Discoveries:** one, plus one re-affirmation.
  1. **`1789064219098-at-35-e5-005-45ad01`** (`correction`) — the dispatch prompt states
     `CYCLE NUMBER FOR THIS CRITERION: 1`, dispatching the criterion as if unstarted. It was
     closed by cycle 1 at `103693b365`; `AT-35-E5-005_cycle1_receipt.md` exists and `kanban.md`
     row 23 already reads `complete`. Recurrence key `stale-census-in-dispatch-prompt`, the
     **9th** criterion re-dispatched after closure (E3-002, E3-003, E4-001, E4-002, E5-001,
     E5-002, E5-003, E5-004, E5-005). Caught before any write; nothing propagated into the tree.
     Verified by
     `test -f docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_cycle1_receipt.md && grep -n 'AT-35-E5-005' docs/release/SD-35-corpus-sheet-completion/kanban.md`.
  2. **`1789064249278-at-35-e5-005-579de8`** (`deferral`, `--corrects 1788994100821-at-35-e5-005-5973cb`)
     — cycle 1's 10-unit hand-off is **still unclaimed** at HEAD.
     `AT-35-E5-005_desc_without_prose.py --check` still exits **1** with the same **10** units
     (`ability` 3, `class_feature` 7; 5 of them pointer rows whose `granted_by` rule is equally
     empty): units whose corpus record carries a real `DESC` token of 224–829 characters of
     published rules text while the compiled `SheetRule` carries no words at all. The fix is
     converter-side, in `src/pcgen_import/sheet_rule/`, which is **not** in Epic 5's file-touch
     set (`workflow-instruction.md §3`); editing it from a closure-accounting cycle is §8's "two
     live cycles on conflicting files". It is measured, named, gated red and handed on — **not**
     an exemption, and **no unit was excluded from any denominator on its account** (all 10 are
     inside the 49,438 and inside DONE). Epic 6 owns that path.
- **Figures + their re-derive commands:**
  | Figure | Value | Command | Denominator |
  |---|---|---|---|
  | corpus partition (**Evidence clause 1**) | `DONE 49438 / A 0 / B 0 / C 0 / D 0 / M 0 / V 0 / U 0 / X 0 / Z 0`, exit 0 | `python3 scripts/completion_atlas.py --check` | of 49,438 corpus units (`population=49438 buckets=10 unclassified=0 overlap=0`) |
  | atlas self-checks | `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | `python3 scripts/completion_atlas.py --check` | of the 49,438 classified units |
  | manifest rows (**Evidence clause 2**) | **49438**, `non_done=0`, exit 0 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_completion_manifest.py --check` | of 49,438 corpus units, one row each |
  | manifest histogram equals the atlas's | `{'DONE': 49438, 'A': 0, 'B': 0, 'C': 0, 'D': 0, 'M': 0, 'V': 0, 'U': 0, 'X': 0, 'Z': 0}` | `python3 .../AT-35-E5-005_completion_manifest.py --check` | of the 49,438 manifest rows |
  | manifest rows by status | `sheet-complete 23315 / text-complete 11599 / oracle-unverifiable 8491 / grounded 5222 / oracle-agree 811` | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/completion-manifest.json'))['summary']['by_status'])"` | of the 49,438 manifest rows |
  | manifest rows by sheet-rule content | `prose 29213 / label_only 8380 / prose+value 6117 / value_only 3680 / label_only_with_granted_by 1906 / no_rule 142` | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/completion-manifest.json'))['summary']['by_sheet_rule_content'])"` | of the 49,438 manifest rows |
  | manifest coverage | **37 books, 19 kinds, 155 distinct evidence strings** | `python3 -c "import json;s=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/completion-manifest.json'))['summary'];print(s['books'],s['kinds'],s['distinct_evidence_strings'])"` | of the 49,438 manifest rows |
  | capability register closed (**Evidence clause 3**) | `rows=11 built=5 unnecessary_under_sheet_rule=6 still_open=0 units_covered=11055 verdict=PASS`, exit 0 | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_capability_register.py --check` | of SD-34's 11 capability rows |
  | capability-register units not DONE at HEAD | **0** | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/capability-register-rederived.json'))['summary']['units_covered_not_done_at_head'])"` | of the 11,055 units the 11 rows name |
  | the 5 rows now `built: true` | `companion_table_shape_widening, cross_record_content_ownership_resolution, marker_stripping_for_pcgen_editorial_markers, per_character_choice_filter, power_engine_table` | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/capability-register-rederived.json'))['summary']['built_ids'])"` | of SD-34's 11 capability rows |
  | the 6 rows `unnecessary-under-sheet-rule` | `class_feature_deep_subsystem_modelling, companion_mount_advancement_table, corpus_content_extraction_for_uncaptured_records, master_side_ability_pool_record_type_or_cross_book_ownership, monster_class_hit_dice_progression_modelling, oracle_probe_surface_for_no_table_kinds` | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/capability-register-rederived.json'))['summary']['unnecessary_ids'])"` | of SD-34's 11 capability rows |
  | all three derived artifacts reproduced **byte-identical** | **0 files changed** | `git status --porcelain -- docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/` after re-running all three `--check` scripts | of the 3 artifacts (`completion-manifest.json`, `capability-register-rederived.json`, `desc-without-prose.json`) |
  | hollow words-form sheet lines (the open hand-off) | **10**, `by_kind={'ability': 3, 'class_feature': 7}`, `verdict=RESIDUE`, **exit 1** | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_desc_without_prose.py --check` | of the 23,315 `sheet-complete` units |
  | token ledger | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | of 49,438 corpus units |
  | engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | of 49,438 corpus units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | of the 2 kinds `ENGINE_SURFACE_CITATIONS` names |
  | package source-format markers | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | of the compiled package files under `data/sheet_rules/` |
  | denominator gate | `files_checked=84 violations=0` at cycle start; `files_checked=85 violations=0` once this receipt exists | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | of the SD-35 package's `.md` files — 84 before this receipt, 85 with it |
  | provenance gate | `figures_examined=263 violations=0` at cycle start; `figures_examined=274 violations=0` once this receipt exists | `python3 scripts/denominator_gate.py --check-provenance 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | of the figures in those files — 263 in 84, then 274 in 85; the **11** added are this receipt's own, each carrying its command |
  | PI sweep | `PASS pi-sweep (11 hits over src/rules_core/rules_tables, 11 baseline rows)` | `scripts/verify.sh --only pi-sweep` | of the 11-row PI blacklist baseline |
  | PCGen residue | `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS` | `python3 scripts/pcgen_residue_gate.py --check` | of the 260-file live-side baseline |
- **Build scope verified:** one cold compile session at the cycle's code HEAD `5e68380ac4`
  (this cycle adds **no** Rust, so its docs commits cannot move a build result),
  `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E5-005` (created empty), `CARGO_INCREMENTAL=0`, `-j 6`.
  See `## Build result` below for the literal output.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`, **413** executables linked
    (`grep -c 'Executable ' ` over the run's log)
  - `cargo run --locked --bin sheet_rule_convert -- --check` → `CONV_CHECK_EXIT=0`,
    `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS` —
    identical to cycle 1's, and its whole refusal set is the one line `refused 142 no_corpus_record`
  - `cargo clippy --locked --tests -j 6` → `CLIPPY_EXIT=0`, **0** lines matching
    `^warning|^error`
  - `cargo test --locked --lib -j 6` and `cargo test --locked --no-fail-fast -j 6` —
    **deliberately not run, and this is the rule, not an omission.**
    `workflow-instruction.md §6` step 3 requires the full run "when `src/` or the classifier
    changed", and this cycle changed neither:
    `git diff --name-only 5e68380ac4..HEAD -- src tests scripts data apps` is **empty**, and so is
    `git diff --name-only 53638610fc..HEAD -- src tests scripts data apps` — nothing under those
    trees has moved since AT-35-E5-004's cycle 2. The last full-workspace run is AT-35-E5-004's,
    **8,741 passed / 0 failed over 412 targets**, and it stands at this same tree.
  - `cargo run --locked --bin corpus_literal_sweep` — **not run and not claimed**: no corpus
    record changed (`git status --porcelain -- data/corpus` empty throughout,
    `git diff --name-only 5e68380ac4..HEAD -- data/corpus` empty).
  - the desktop crate and the frontend run at the **epic wrap-up** (`decisions.md §3`): this cycle
    touched nothing under `apps/`.
- **Sweep population:** N/A — `corpus_literal_sweep` did not run; no corpus record changed. Its
  last run is AT-35-E3-001's, `48,706 records examined of 51,476 read, 0 findings, CLEAN`.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — no figure in this
  receipt was taken from the pinned tree; the pin is recorded because every `data/sheet_rules/**`
  record the re-derivation read carries it as `provenance.oracle_pin`.
- **Status:** complete — the criterion's population is **0 of 49,438** at HEAD and all three
  Evidence clauses re-derive at HEAD: the atlas prints `DONE 49438` with every other bucket `0`
  and all four self-checks clean; `completion-manifest.json` carries one row per unit, 49,438 of
  them, `non_done=0`, its histogram equal to the atlas's; and all **11** of SD-34's capability
  rows are closed, **5** `built: true` and **6** `unnecessary-under-sheet-rule: <reason>`, with
  **0** still open over 11,055 units, none of them non-DONE. All three derived artifacts
  reproduced **byte-identical** to cycle 1's, which is the strongest available statement that
  neither the corpus nor the instruments drifted underneath them. **Nothing was redone.** The
  10-unit residue remains a reported number behind a red gate, not an exemption.
- **Notes:** the dispatch numbered this cycle 1, but `AT-35-E5-005_cycle1_receipt.md` already
  exists, so it is filed as cycle 2 and cycle 1 is left untouched. `kanban.md` row 23 stays
  `complete`; row 41 records this extra cycle (`§5`). The `desc-without-prose` gate is left
  **red on purpose** — a red `--check` is how the hand-off travels to Epic 6 as a mechanism rather
  than a note (`AGENTS.md` §8, "a warning is not a control").
- **Next-cycle scope:** criterion at zero; Epic 5 at zero; the corpus at zero.
  `python3 scripts/cycle_scope_gate.py --min 500` → `scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER`.
  **One named hand-off, not a carve-out:** a converter-side cycle owning
  `src/pcgen_import/sheet_rule/` takes the 10 units in
  `artifacts/epic-5-residues/desc-without-prose.json` and turns
  `AT-35-E5-005_desc_without_prose.py --check` green (currently exit 1). Epic 6 owns that path.

## Build result

Run at `5e68380ac4`, `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E5-005` (created empty),
`CARGO_INCREMENTAL=0`, `-j 6`.

```
cargo test --locked --no-run -j 6                        NO_RUN_EXIT=0
                                                        (413 test executables linked)
cargo run --locked --bin sheet_rule_convert -- --check   records=49438 converted=49296 refused=142
                                                        rules=69344 var_tables=5277 verdict=PASS (117.7s)
                                                        refused    142  no_corpus_record
                                                        CONV_CHECK_EXIT=0
cargo clippy --locked --tests -j 6                       Finished `dev` profile in 1m 27s
                                                        0 lines matching '^warning|^error'
                                                        CLIPPY_EXIT=0
```

Cycle 1 recorded `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS`
at `ac2165b393` and `CONV_CHECK_EXIT=0`. This run at `5e68380ac4` reproduces every figure, which
is what makes the "nothing drifted" claim above a measurement rather than an inference.
