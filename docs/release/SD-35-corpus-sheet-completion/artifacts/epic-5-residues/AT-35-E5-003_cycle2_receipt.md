# Cycle AT-35-E5-003_cycle2 — Epic 5 Residues / AT-35-E5-003

- **Commit SHA:** pinned in the follow-up commit named at the end of this row. Cycle start
  `ccb9d8d515` — "pin AT-35-E5-002 cycle 2's commit SHA in its own receipt and kanban row".
  Kanban row 21 was already `complete` at dispatch; this is a **re-dispatch of an already-closed
  card**, and it redoes nothing. Nothing under `src/`, `data/`, `apps/` or `scripts/` changed.
- **Scope gate:**
  ```
  inventory=docs/work-inventory.json
  scope=bucket=U|Z
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  Run as `python3 scripts/cycle_scope_gate.py --min 500 --bucket U --bucket Z`, exit 0.
  **The dispatch's mandatory-bundling instruction is moot, not skipped**: `remaining_non_done=0`
  is the whole corpus and not merely buckets U and Z, so the gate returns
  `PASS_WHOLE_REMAINDER` rather than `FAIL_UNDER_FLOOR` and there is no unit anywhere to bundle
  in. This is not a floor exemption — the gate ran and passed.
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_cycle2_receipt.md` (new — this file)
  - `docs/retro/events/at-35-e5-003.jsonl` (1 `correction` event appended, plus the derived
    `verify.sh --only pi-sweep` run event this cycle's own run produced)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `docs/release/SD-35-corpus-sheet-completion/kanban.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` — the
    `derived_at` stamp only, moved by running `completion_atlas.py --check`

  **No Rust, no `data/`, no `apps/`, no `scripts/` file changed. This cycle moves no unit.**
- **Identifier audit result:** **9 matches at HEAD, all pre-existing, 0 from this cycle.** Every
  one is a reference to an **existing test filename** in prose or in a doc comment, a shape the
  pattern `sd[0-9]+_` cannot distinguish from a bundle-tagged identifier:
  - `src/rules_core/` **2** — `tests/sd27_feat_prerequisite_enforcement.rs` on a removed line and
    `tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs` on an added one.
  - `artifacts/epic-5-residues/` **7** — receipt prose in `AT-35-E5-001_cycle2_receipt.md`,
    `AT-35-E5-002_cycle2_receipt.md` and `AT-35-E5-004_cycle1_receipt.md` quoting those same
    filenames (and `tests/sd27_alternate_racial_trait_reachability.rs`) in order to characterise
    them as pre-existing.
  - `class_feature_pool_picker.rs`, `LevelUpDialog.tsx`, `data/corpus/beginner_box/`,
    `data/sheet_rules/`, `docs/work-inventory.json` — **0** each.

  `OK_NO_BUNDLE_TAGS` **on this cycle's own change**: this cycle introduces no identifier at all.
  The **2** further matches this receipt itself contributes are the audit row above, quoting those
  pre-existing test filenames to characterise them — the same self-referential audit prose
  `AT-35-E5-001_cycle2_receipt.md` and `AT-35-E5-002_cycle2_receipt.md` already recorded.
- **Wired-integration audit result:** **21 matches at HEAD, all pre-existing, 0 from this cycle**,
  attributed exhaustively, path by path:
  - `data/sheet_rules/` **2** — English rule prose transcribed verbatim from the corpus, on added
    lines: `bestiary_3:monster_ability:tophet_swallow_whole` ("Once swallowed by …") and
    `core_rulebook:spell:plant_growth` ("Plant growth has different effects …"). Rendered words,
    not a stub.
  - `docs/work-inventory.json` **3** — **removed** lines. They are PCGen's own CHOOSE-menu
    "no selection" placeholder rows for the Barbarian, Monk and Rogue classes, present in
    `develop`'s base `fe5ae6cd4a` and **deleted** by SD-35. `git show HEAD:docs/work-inventory.json
    | grep -cE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` returns **0**.
    (This is the attribution `AT-35-E5-002_cycle2_receipt.md`'s second correction established;
    re-derived here rather than carried forward.)
  - `artifacts/epic-5-residues/` **16** — receipt prose across the Epic 5 receipts that quotes the
    token alternation in order to characterise it.
  - `src/rules_core/`, `class_feature_pool_picker.rs`, `LevelUpDialog.tsx`,
    `data/corpus/beginner_box/` — **0** each.

  `OK_NO_TOKENS` **on this cycle's own change**: no stub, no inline mock, no "Would …" string,
  because this cycle ships no code. The further matches this receipt itself contributes are the
  audit row above and the **Figures** table's re-derive commands, both of which must quote the
  token alternation to be readable and runnable.
- **Acceptance criterion** (verbatim, `epic-breakdown.md ### AT-35-E5-003`):

  > ### AT-35-E5-003 — buckets U and Z reach zero
  >
  > U 202: per sub-cause, the instrument correction or a proven statement that the record carries
  > nothing a player reads. Z 19: `beginner_box` gets a compiled rule set through the guarded
  > generator path, then converts.
  >
  > **Evidence:** U and Z at 0; `corpus_literal_sweep` examined-count moved by exactly the
  > `beginner_box` record delta.

- **Receipt rows (mechanical):**
  ```
  since=ccb9d8d515 target_dir=/tmp/cargo-sd35-AT-35-E5-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=253
  ```
  `closed=0` is correct and expected: the criterion's units were emptied at `26bdfa8d5b`
  (`artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md`) and its Evidence clauses
  were paid by cycle 1. `builds_recorded=1` is this cycle's single cold compile session, which
  served both `cargo test --no-run` and `sheet_rule_convert --check`.
- **PCGen residue:**
  ```
  pattern TYPE= files=70 hits=761
  root src/rules_core files=202 hits=11779
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=51 hits=477
  identifier_files=54 identifier_hits=343
  live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  `live_files=253`, identical to `AT-35-E5-001_cycle2_receipt.md` and
  `AT-35-E5-002_cycle2_receipt.md`. Not raised; nothing new on the live side reads a PCGen token.
- **Oracle parity:** N/A — this cycle adds no `Number` mapping and touches no live path.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. U's 202 and Z's 19 closed at `26bdfa8d5b`, before
    this cycle's start.
  - **relabel (bucket to bucket):** none — `relabeled=0`, `regressed=0`, `added=0`, `dropped=0`.
  - **reachability:** unchanged. `shape_engine_boundary.py --check` →
    `magnitude_bearing=26396 not_held_by_engine=0`.
  - **instrument-correction:** one, and it is a **prose** correction, not an instrument change —
    the dispatch prompt's census. Emitted as `1789062448690-at-35-e5-003-bc688e`.
- **Refused tokens:** none. `python3 scripts/token_coverage.py --check` at HEAD →
  `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1
  verdict=PASS`. The 142 converter-refused records are all in DONE units (`refused_non_done=0`),
  so no refusal stands against this criterion's population.
- **Discoveries:** none. Both Evidence clauses re-derived byte-consistent with cycle 1; no token
  type, kind, or remaining-step category appeared that `token-coverage.json` or the atlas did not
  already predict.
- **Figures + their re-derive commands:**

  | Figure | Value | Command (denominator) |
  | --- | --- | --- |
  | bucket U at HEAD | **0** | `python3 scripts/cycle_scope_gate.py --min 500 --bucket U --bucket Z` — of 49,438 units in `docs/work-inventory.json` |
  | bucket Z at HEAD | **0** | same command; and `python3 .../AT-35-E5-003_buckets_u_z.py` → `bucket_U_at_HEAD=0 bucket_Z_at_HEAD=0 verdict=PASS` |
  | every bucket at HEAD | **DONE=49438, A/B/C/D/M/V/U/X/Z all 0** | `python3 scripts/completion_atlas.py --check` — of 49,438 |
  | U+Z units at the `tranche/15` cut | **221** (U 202 + Z 19) | `python3 .../AT-35-E5-003_buckets_u_z.py` → `== 4c6c57eb9f … U=202 Z=19` — of 49,438 at `4c6c57eb9f` |
  | U/Z sub-causes | **4** | `python3 .../AT-35-E5-003_buckets_u_z.py --transitions` → `sub_causes=4 uz_units=221` — of the 221 cut-state units |
  | U/Z units sheet-complete at HEAD | **221 of 221** | same command → `not_sheet_complete_at_HEAD=0 verdict=PASS`; 211 → `sheet_rule_rendered:words`, 10 → `sheet_rule_rendered:number` |
  | U/Z units whose converted rule is missing or label-less | **0** | `python3 .../AT-35-E5-003_buckets_u_z.py --proof` → `missing_rule_file=0 rules_without_a_label=0 verdict=PASS` — of 221 |
  | `beginner_box` record delta | **0** (19 → 19 corpus records) | `python3 .../AT-35-E5-003_buckets_u_z.py --sweep-delta` → `corpus_records_before=19 corpus_records_after=19 record_delta=0 corpus_files_changed=0 compiled_rule_files_at_head=19 verdict=PASS` |
  | `sheet_rule_convert --check` | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277` | `cargo run --locked --bin sheet_rule_convert -- --check` → `verdict=PASS (116.0s)` |
  | PCGen tokens under `data/sheet_rules/` | **0 files** | the `grep -rlE … data/sheet_rules/ … wc -l` invocation quoted in full in the **Build scope verified** row below — of 69,344 rules in 49,296 files |
  | live PCGen files | **253** (baseline 260) | `python3 scripts/pcgen_residue_gate.py --check` |
  | identifier-audit matches | **9**, 0 this cycle's | `git diff --unified=0 $(git merge-base HEAD origin/develop)...HEAD -- <epic-5 file-touch set> ':!**/__tests__/**' ':!**/*.test.*' \| grep -cE '\b(sd[0-9]+_\|SD[0-9]+_\|Sd[0-9]+\|t_[0-9a-f]{8,})'` |
  | wired-integration matches | **21**, 0 this cycle's | same diff `\| grep -cE '\b(STUB\|MOCK\|placeholder\|not yet implemented\|todo\|fixme\|hack)\b'` |
  | denominator gate | `files_checked=82 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | provenance gate | `files_checked=199 figures_examined=378 violations=0` | `python3 scripts/denominator_gate.py --check-provenance` |

  **On the `record_delta=0`.** The Evidence sentence asks that `corpus_literal_sweep`'s examined
  count move by *exactly* the `beginner_box` record delta. That delta is **0**, and cycle 1
  recorded the sweep's examined count as **48706 → 48706** — an exact match, and the clause is
  paid. Z's 19 units were never a *missing corpus record*; the 19 `beginner_box` records stood at
  the `tranche/15` cut. What was missing was the **compiled rule set**, which the guarded
  generator path produced: `compiled_rule_files_at_head=19` under `data/sheet_rules/beginner_box/`,
  `corpus_files_changed=0` under `data/corpus/beginner_box/`. **No hand edit under
  `data/corpus/**` was made or is required**, which is exactly what the criterion's Z branch
  demands.
- **Build scope verified:** `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E5-003`, `CARGO_INCREMENTAL=0`,
  `-j 6`, run at `ccb9d8d515`.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0` (cold, wall clock **2:53.72**, max RSS
    2,450,804 kB)
  - `cargo run --locked --bin sheet_rule_convert -- --check` → exit 0, `verdict=PASS (116.0s)`,
    wall clock 1:56.99 — the same `records/converted/refused/rules/var_tables` line as cycle 1
  - <code>grep -rlE 'BONUS:&#124;DEFINE:&#124;PRE[A-Z]+:&#124;%CHOICE&#124;CL=' data/sheet_rules/ &#124; wc -l</code> → `0`
  - `completion_atlas.py --check` exit 0 (`done_evidence_violations=0 missing_clearing_mechanisms=0
    stale_derived_at=False citation_failures=0`); `token_coverage.py --check` → `verdict=PASS`;
    `shape_engine_boundary.py --check` → `not_held_by_engine=0 citation_ok=True`;
    `missing_engine_tables.py --check` → `population=0 kinds=0 citation_failures=0`;
    `pcgen_residue_gate.py --check` → `verdict=PASS`; `denominator_gate.py --check` →
    `violations=0`; `denominator_gate.py --check-provenance` → `violations=0`;
    `./scripts/publish-site-dashboard.sh --check-pin` → pin matches
    (`5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f`), exit 0
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (1 stage, 11 hits over 11 baseline rows),
    logs `/tmp/codex-verify-ewZhIl`
  - `cargo clippy`: **not run — no Rust target changed**, so there is no touched target to lint.
  - **Full workspace suite: not re-run, by attribution, not by omission.** `git diff --name-only
    ccb9d8d515..HEAD` lists only `docs/` paths; this cycle changes no Rust, no `data/`, no `apps/`
    and no `scripts/` file, so there is no change a suite failure could attribute to. The last
    full green is the Epic 5 wrap-up correction cycle's **49 of 49 PASS**. Precedent:
    `AT-35-E5-001_cycle2_receipt.md` and `AT-35-E5-002_cycle2_receipt.md`, both docs-only, on the
    same reasoning.
  - desktop crate + frontend: **epic cadence** — this cycle touched no `apps/` path.
- **Sweep population:** N/A this cycle — no corpus record changed, so `corpus_literal_sweep` was
  not re-run (`corpus_files_changed=0` under `data/corpus/beginner_box/`). Cycle 1's recorded
  figure is **48706 → 48706**, matching the `beginner_box` record delta of **0** exactly.
- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.
- **Status:** complete
- **Notes:** A re-dispatch of an already-`complete` card (row 21, closed at cycle 1). Both Evidence
  clauses were re-derived at HEAD rather than cited from cycle 1, and both hold. Nothing was
  redone, no unit moved, and no other criterion's card was emptied by this cycle — there was
  nothing left to empty. One correction emitted for the dispatch prompt's stale census.
- **Next-cycle scope:** criterion at zero — `--bucket U --bucket Z` returns
  `scoped=0 remaining_non_done=0 verdict=PASS_WHOLE_REMAINDER`.
