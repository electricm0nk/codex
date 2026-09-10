# Cycle AT-35-E5-004_cycle2 — Epic 5 Residues / AT-35-E5-004

- **Commit SHA:** `<pinned in the follow-up commit below>` (this receipt, `progress.md`,
  `kanban.md` and the retro correction, in one commit), plus a SHA-pinning follow-up commit.
  Cycle start `53638610fc` — "pin AT-35-E5-003 cycle 2's commit SHA in its own receipt, kanban
  row and progress entry". Kanban row 22 was already `complete` at dispatch; this is a
  **re-dispatch of an already-closed card**, and it redoes nothing. Nothing under `src/`,
  `data/`, `apps/` or `scripts/` changed.
- **Scope gate:**
  ```
  inventory=docs/work-inventory.json
  scope=bucket=X
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  Run as `python3 scripts/cycle_scope_gate.py --min 500 --bucket X`, exit 0.
  **The dispatch's mandatory-bundling instruction is moot, not skipped**: `remaining_non_done=0`
  is the whole corpus and not merely bucket X, so the gate returns `PASS_WHOLE_REMAINDER` rather
  than `FAIL_UNDER_FLOOR` and there is no unit in any bucket to bundle in. This is **not** a
  floor exemption — the gate ran and passed.
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-004_cycle2_receipt.md` (new — this file)
  - `docs/retro/events/at-35-e5-004.jsonl` (new — 1 `correction` event, plus any derived
    `verify.sh --only pi-sweep` run event this cycle's own run appended)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `docs/release/SD-35-corpus-sheet-completion/kanban.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` — the
    `derived_at` stamp only, moved by running `completion_atlas.py --check`

  **No Rust, no `data/`, no `apps/`, no `scripts/` file changed. This cycle moves no unit.**
- **Identifier audit result:** **12 matches at HEAD, all pre-existing, 0 from this cycle.** Every
  one is a reference to an **existing test filename** in prose or in a doc comment, a shape the
  pattern `sd[0-9]+_` cannot distinguish from a bundle-tagged identifier:
  - `src/rules_core/` **2** — `tests/sd27_feat_prerequisite_enforcement.rs` on a removed line and
    `tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs` on an added one.
  - `artifacts/epic-5-residues/` **10** — receipt prose in `AT-35-E5-001_cycle2_receipt.md`,
    `AT-35-E5-002_cycle2_receipt.md`, `AT-35-E5-003_cycle2_receipt.md` and
    `AT-35-E5-004_cycle1_receipt.md` quoting those same filenames (and
    `tests/sd27_alternate_racial_trait_reachability.rs`) in order to characterise them as
    pre-existing. The count rose from `AT-35-E5-003_cycle2_receipt.md`'s **9** by exactly the
    **3** such quotations that receipt itself added — an accounted-for delta, not new drift.
  - `class_feature_pool_picker.rs`, `LevelUpDialog.tsx`, `data/corpus/beginner_box/`,
    `data/sheet_rules/`, `docs/work-inventory.json` — **0** each.

  `OK_NO_BUNDLE_TAGS` **on this cycle's own change**: this cycle introduces no identifier at all.
  The further matches this receipt itself contributes are the audit row above, quoting those
  pre-existing test filenames to characterise them.
- **Wired-integration audit result:** **24 matches at HEAD, all pre-existing, 0 from this cycle**,
  attributed exhaustively, path by path:
  - `data/sheet_rules/` **2** — English rule prose transcribed verbatim from the corpus, on added
    lines: `bestiary_3:monster_ability:tophet_swallow_whole` ("Once swallowed by …") and
    `core_rulebook:spell:plant_growth` ("Plant growth has different effects …"). Rendered words,
    not a stub.
  - `docs/work-inventory.json` **3** — **removed** lines. They are PCGen's own CHOOSE-menu
    "no selection" placeholder rows for the Barbarian, Monk and Rogue classes, present in
    `develop`'s base `fe5ae6cd4a` and **deleted** by SD-35. Re-derived at HEAD here rather than
    carried forward: <code>git show HEAD:docs/work-inventory.json &#124; grep -cE '\b(STUB&#124;MOCK&#124;placeholder&#124;not yet implemented&#124;todo&#124;fixme&#124;hack)\b'</code>
    returns **0**.
  - `artifacts/epic-5-residues/` **19** — receipt prose across the Epic 5 receipts that quotes the
    token alternation in order to characterise it. Up from `AT-35-E5-003_cycle2_receipt.md`'s
    **16** by exactly the **3** that receipt itself added.
  - `src/rules_core/`, `class_feature_pool_picker.rs`, `LevelUpDialog.tsx`,
    `data/corpus/beginner_box/` — **0** each.

  `OK_NO_TOKENS` **on this cycle's own change**: no stub, no inline mock, no "Would …" string,
  because this cycle ships no code. The further matches this receipt itself contributes are the
  audit row above and the **Figures** table's re-derive commands, both of which must quote the
  token alternation to be readable and runnable.
- **Acceptance criterion** (verbatim, `epic-breakdown.md ### AT-35-E5-004`):

  > ### AT-35-E5-004 — bucket X reaches zero: the per-character choice filter
  >
  > 168 at authoring. SD-34 `decisions.md §17`'s operator requirement stands: the backend filters
  > the valid options for *this* character at level-up. Build the join over `SheetRule.applies`
  > (prerequisites are `Applies`, converted from `PRE*` at ingest — no `pre_tokens` on the live
  > side) and expose it on the existing level-up IPC.
  >
  > **Evidence:** X at 0; a desktop test: a level-3 fixture's option list excludes a
  > failed-prereq option and includes a met one.

- **Receipt rows (mechanical):**
  ```
  since=53638610fcfc5b5345cc81c6f7937341fd5e49ec target_dir=/tmp/cargo-sd35-AT-35-E5-004 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=2 pcgen_live_files=253
  ```
  `closed=0` is correct and expected: the criterion's units were emptied at `26bdfa8d5b`
  (`artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md`, X 168 → 0) and its second
  Evidence clause was paid by cycle 1 at `a56096b861`. `builds_recorded=2` is this cycle's two
  compile sessions — the workspace target dir (`/tmp/cargo-sd35-AT-35-E5-004`, serving the
  `--lib` filter tests and `--no-run`) and the desktop-crate target dir
  (`/tmp/cargo-sd35-AT-35-E5-004-desktop`, serving the two `feat_option` tests). Both exist to
  **re-derive the Evidence, not to change anything**: `rust_lines_changed=0`.
- **PCGen residue:**
  ```
  pattern raw_tokens files=41 hits=234
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=17 hits=109
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  root src/rules_core files=202 hits=11779
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=51 hits=477
  identifier_files=54 identifier_hits=343
  live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  `live_files=253`, identical to `AT-35-E5-001_cycle2_receipt.md`,
  `AT-35-E5-002_cycle2_receipt.md` and `AT-35-E5-003_cycle2_receipt.md`. Not raised. **`pattern
  pre_tokens files=0 hits=0` is the criterion's own no-`pre_tokens`-on-the-live-side clause,
  re-derived at HEAD** — the filter reads `SheetRule.applies`, never a `PRE*` token.
- **Oracle parity:** N/A — this cycle adds no `Number` mapping and touches no live path. Cycle 1's
  converter change (`PreStatScore_<AB>` → `max(AbilityScore(ab), raisers)`) moved gate operands,
  not a `SheetValue::Number`, and is unchanged here.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. X's 168 closed at `26bdfa8d5b`, before this cycle's
    start; the inherited 137 non-refused units (class_feature 123, companion 12, feat 2) are all
    DONE at HEAD.
  - **relabel (bucket to bucket):** none — `relabeled=0`, `regressed=0`, `added=0`, `dropped=0`.
  - **reachability:** unchanged. `shape_engine_boundary.py --check` →
    `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True`.
  - **instrument-correction:** one, and it is a **prose** correction, not an instrument change —
    the dispatch prompt's census. Emitted as `1789062941113-at-35-e5-004-a733b2`.
- **Refused tokens:** none. `python3 scripts/token_coverage.py --check` at HEAD →
  `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1
  verdict=PASS`. The 142 converter-refused records are all in DONE units (`refused_non_done=0`),
  so no refusal stands against this criterion's population.
- **Discoveries:** none. Both Evidence clauses re-derived at HEAD, and the desktop census came
  back **byte-identical** to cycle 1's (`offered=718 refused=1745 considered=2463`) across the
  6,909 inserted / 4,485 deleted Rust lines that landed between `a56096b861` and this cycle's
  start — the strongest available statement that the filter did not silently drift. No token
  type, kind, or remaining-step category appeared that `token-coverage.json` or the atlas did not
  already predict.
- **Figures + their re-derive commands:**

  | Figure | Value | Command (denominator) |
  | --- | --- | --- |
  | bucket X at HEAD | **0** | `python3 scripts/cycle_scope_gate.py --min 500 --bucket X` → `scoped=0 remaining_non_done=0 verdict=PASS_WHOLE_REMAINDER` — of 49,438 units in `docs/work-inventory.json` |
  | every bucket at HEAD | **DONE=49438, A/B/C/D/M/V/U/X/Z all 0** | `python3 scripts/completion_atlas.py --check` — of 49,438 |
  | bucket X per kind at HEAD | **0 in all 19 kinds** (class_feature 18043, companion 1696, feat 2764 all `X=0(0.0%)`) | `python3 scripts/completion_atlas.py --by-kind` — of 49,438 across 19 kinds |
  | inherited non-refused X units | **137** (class_feature 123, companion 12, feat 2) | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_handoff.py` — of 1,404 non-DONE at `38b67db94e` |
  | new lib filter tests, green at HEAD | **7 passed; 0 failed** | `cargo test --locked --lib -j 6 level_up_option_filter -- --nocapture` — of 3,276 lib tests (3,269 filtered out) |
  | desktop Evidence tests, green at HEAD | **2 passed; 0 failed** | `cargo test --locked -j 4 feat_option -- --nocapture` in `apps/desktop/src-tauri` — of 576 desktop tests (574 filtered out) |
  | fixture option census at HEAD | `offered=718 refused=1745 considered=2463` | the same desktop invocation, `--nocapture` line `feat option census:` — of the 2,465 offerable `pool=feat` records |
  | live PCGen files | **253** (baseline 260) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `pre_tokens` on the live side | **0 files, 0 hits** | same command, `pattern pre_tokens` row — over `src/rules_core`, `src/saved_character`, `src/campaign`, `src/homebrew_authoring`, `apps/desktop` |
  | PCGen tokens under `data/sheet_rules/` | **0 files** | the `grep -rlE … data/sheet_rules/ … wc -l` invocation quoted in full in the **Build scope verified** row below — of 69,344 rules in 49,296 files |
  | identifier-audit matches | **12**, 0 this cycle's | `git diff --unified=0 $(git merge-base HEAD origin/develop)...HEAD -- <epic-5 file-touch set> ':!**/__tests__/**' ':!**/*.test.*' \| grep -cE '\b(sd[0-9]+_\|SD[0-9]+_\|Sd[0-9]+\|t_[0-9a-f]{8,})'` |
  | wired-integration matches | **24**, 0 this cycle's | same diff <code>&#124; grep -cE '\b(STUB&#124;MOCK&#124;placeholder&#124;not yet implemented&#124;todo&#124;fixme&#124;hack)\b'</code> |
  | token coverage | `non_done=0 refused=142 refused_non_done=0 token_types=231 shapes=1` | `python3 scripts/token_coverage.py --check` — of 49,438 |
  | denominator gate | `files_checked=83 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | provenance gate | `files_checked=200 figures_examined=382 violations=0` | `python3 scripts/denominator_gate.py --check-provenance` |
  | dashboard input pin | matches (`5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f`) | `./scripts/publish-site-dashboard.sh --check-pin` |

  **On re-running the Evidence rather than citing it.** `git diff --stat a56096b861 HEAD -- src/
  apps/ data/ scripts/ tests/` reports **2,128 files changed, 6,909 insertions, 4,485 deletions**,
  and the changed set includes `src/rules_core/level_up_option_filter.rs`,
  `src/rules_core/feat_prereqs.rs` and the new `src/rules_core/feat_prereqs/converted_gate.rs`.
  The filter's own inputs moved after cycle 1 proved it, so citing cycle 1's green would have been
  a stale agreement. Both suites were re-run at this cycle's start SHA and both are green.
- **Build scope verified:** `CARGO_INCREMENTAL=0`, run at `53638610fc`.
  - `cargo test --locked --lib -j 6 level_up_option_filter -- --nocapture`
    (`CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E5-004`, cold) → `test result: ok. 7 passed; 0
    failed; 0 ignored; 0 measured; 3269 filtered out`, wall clock **101.27 s**. The seven:
    `an_option_whose_prerequisite_is_met_is_offered`,
    `an_option_whose_prerequisite_is_unmet_is_refused_with_the_requirement_in_words`,
    `an_ability_score_one_point_short_refuses_and_names_the_score`,
    `a_compound_gate_names_only_the_failing_term`,
    `a_situational_gate_is_offered_and_carries_its_condition`,
    `a_narrowing_tag_bounds_the_pool`, `every_pool_option_is_either_offered_or_refused`.
  - **desktop crate** (`CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E5-004-desktop`, cold):
    `cargo test --locked -j 4 feat_option -- --nocapture` → `Finished 'test' profile … in 3m 58s`,
    `test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 574 filtered out`, wall clock
    **248.62 s**. `preview_level_up_filters_the_feat_options_by_this_characters_own_prerequisites`
    is the criterion's Evidence test: a `race:human` level-3 fixture previewing to level 4,
    `option_filter_unavailable_reason == None`, **Leadership refused** with `unmet` naming
    "character level" and "7" and absent from `feat_options` (the failed-prereq exclusion),
    **Improved Initiative offered** (ungated) and **Mobility offered** because this character
    selected Dodge (the met-prerequisite inclusion, and a per-character join rather than an
    ungated pass-through). `no_feat_option_is_both_offered_and_refused` printed
    `feat option census: offered=718 refused=1745 considered=2463`.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`, wall clock **152.60 s**, max RSS
    2,137,392 kB, **0** compiler warnings
  - <code>grep -rlE 'BONUS:&#124;DEFINE:&#124;PRE[A-Z]+:&#124;%CHOICE&#124;CL=' data/sheet_rules/ &#124; wc -l</code> → `0`
  - `completion_atlas.py --check` exit 0 (`population=49438 buckets=10 unclassified=0 overlap=0`,
    `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
    citation_failures=0`); `token_coverage.py --check` → `verdict=PASS`;
    `shape_engine_boundary.py --check` → `not_held_by_engine=0 citation_ok=True`;
    `missing_engine_tables.py --check` → `population=0 kinds=0 citation_failures=0`;
    `pcgen_residue_gate.py --check` → `verdict=PASS`; `denominator_gate.py --check` →
    `violations=0`; `denominator_gate.py --check-provenance` → `violations=0`;
    `./scripts/publish-site-dashboard.sh --check-pin` → pin matches, exit 0
  - `scripts/verify.sh --only pi-sweep` → `PASS pi-sweep (11 hits over src/rules_core/rules_tables,
    11 baseline rows)`, `RESULT: PASS` (1 stage), logs `/tmp/codex-verify-R8ALxm`
  - `cargo run --locked --bin sheet_rule_convert -- --check`: **not re-run.** `data/sheet_rules/`
    and every converter source under `src/pcgen_import/` are byte-identical to this cycle's start
    (`git diff --name-only 53638610fc..HEAD -- data/sheet_rules src/pcgen_import` is empty), and
    `AT-35-E5-003_cycle2_receipt.md` recorded `records=49438 converted=49296 refused=142
    rules=69344 var_tables=5277 verdict=PASS` at that same tree. The cheap invariant it guards —
    no source-format token in our data files — was re-run above and prints `0`.
  - `cargo clippy`: **not run — no Rust target changed**, so there is no touched target to lint.
  - **Full workspace suite: not re-run, by attribution, not by omission.**
    `git diff --name-only 53638610fc..HEAD` lists only `docs/` paths; this cycle changes no Rust,
    no `data/`, no `apps/` and no `scripts/` file, so there is no change a suite failure could
    attribute to. The two suites that *do* bear on this criterion's Evidence were run in full
    above. The last full green is the Epic 5 wrap-up correction cycle's **49 of 49 PASS**.
    Precedent: `AT-35-E5-001_cycle2_receipt.md`, `AT-35-E5-002_cycle2_receipt.md` and
    `AT-35-E5-003_cycle2_receipt.md`, on the same reasoning.
  - **frontend:** not re-run — `apps/desktop/src/` is unchanged since this cycle's start. Cycle 1
    recorded `npm run typecheck` clean and `npm test` → `101/101 test files passed`.
- **Sweep population:** N/A this cycle — no `data/corpus/` record changed
  (`git diff --name-only 53638610fc..HEAD -- data/corpus` is empty), so `corpus_literal_sweep`
  was not re-run. Cycle 1's recorded figure is **48706 → 48706 of 51,476 read, 0 findings**.
- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus. Cycle 1's
  converter row was derived at `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
- **Status:** complete
- **Notes:** A re-dispatch of an already-`complete` card (row 22, closed at cycle 1). Both
  Evidence clauses were re-derived at HEAD rather than cited from cycle 1 — and that mattered
  here in a way it did not for rows 19–21: 2,128 files of Rust changed between cycle 1's proof
  and this cycle's start, including the filter module itself, so a citation would have been a
  stale agreement. Nothing was redone, no unit moved, and no other criterion's card was emptied
  by this cycle — there was nothing left to empty. One correction emitted for the dispatch
  prompt's stale census (8th criterion re-dispatched after closure on that key).
- **Next-cycle scope:** criterion at zero — `--bucket X` returns
  `scoped=0 remaining_non_done=0 verdict=PASS_WHOLE_REMAINDER`.
