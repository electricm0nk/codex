# Cycle 3 — Epic 6 (PCGen exit) / AT-35-E6-003-SWEEP

Cycle 2's `Next-cycle scope` named one mechanism and told this cycle to take it whole: the feat
prerequisites. It is taken whole — every `prerequisites` field on every feat record type in the
repository, all 2,030 token-bearing rows across 29 books, plus the three backfill tables and both
helper functions that fed them, moved to `src/pcgen_import/`. Nothing on the live side reads a
`PRE`-family feat token any more.

- **Commit SHA:** `68d030837f` carries the cycle's work; this line was written into it by the
  immediately following docs commit (a receipt cannot name the commit that carries it, so the two
  are split rather than guessed). Cycle start `98d8d96c765854523bab6375a7612b09d3cb169c`.

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design,
  decisions.md §2)`. Run anyway, for the record —
  `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  This cycle's **own** floor — 500 code hits, the currency cycle 1 proved was right — is met at
  **10.6×**: **5,320 code hits cleared** (6,619 → 1,299).

- **Files touched:**
  - `src/pcgen_import/feat_prereq_tokens.rs` — **new**, 1,429 relocated hand-authored rows, both
    index-addressed lookups, the joined-catalog router, the checked-and-empty roster, and the
    eight gates.
  - `src/pcgen_import/feat_gap_prereq_tokens.rs` — **new and generated**, 601 relocated gap rows.
  - `src/pcgen_import/mod.rs` — registers both.
  - `src/pcgen_import/cache_gen/feat_gap.rs`, `…/hand_authored_feat_dump.rs` — the two consumers,
    now reading the relocated tokens by `(rule_set, index)`.
  - `src/bin/gen_feat_gap_tables.rs` — writes both generated files in one pass off the same
    parsed records, so they cannot drift.
  - `src/rules_core/rules_tables/feats_all.rs` — the field, the three
    `ARG_`/`PU_`/`UCA_FEAT_PREREQUISITES` tables, `gathered`, `gathered_prerequisites` and the
    four tests that read them, removed with a relocation note in each place.
  - `src/rules_core/rules_tables/crb/feats.rs`,
    `{ultimate_combat,ultimate_magic,ultimate_psionics,ultimate_wilderness,ultimate_intrigue}/feat_tables.rs`
    — the six field declarations.
  - `src/rules_core/rules_tables/feat_gap_tables.rs` and the 17
    `{crb,apg,acg}/feat_data/*.rs` / `*/feat_tables.rs` data files — 2,000 `prerequisites:` field
    values removed.
  - `tests/sd27_feat_prerequisite_enforcement.rs`, `tests/v06_apg_acg_feat_catalog.rs`,
    `tests/feat_gap_tables.rs` — retargeted onto the relocated tables.
  - `docs/release/SD-35-corpus-sheet-completion/` — this receipt, `progress.md`, `kanban.md`;
    `docs/retro/events/at-35-e6-003-sweep.jsonl` (+5: 3 corrections, 1 incident, 1 deferral),
    `docs/retro/events/sd31-transcribe.jsonl` (+1, see Notes);
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (`derived_at` stamp, written by `completion_atlas.py --check`).

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.**
  ```bash
  git diff --unified=0 98d8d96c76 -- src/rules_core src/pcgen_import apps/desktop/src-tauri/src \
      src/bin scripts tests docs/release/SD-35-corpus-sheet-completion \
      ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   -> 1
  ```
  The single hit is the diff's own `+++ b/tests/sd27_feat_prerequisite_enforcement.rs` header
  line — a pre-existing filename, not added content. No added line in the cycle's diff carries a
  bundle tag.

- **Wired-integration audit result:** **OK_NO_TOKENS.** Same diff, `grep -ciE
  '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → `0`. No shipping code
  path was added; data moved across the boundary and two consumers changed how they address it.

- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met.** `apps/desktop/` still stands at **1 file / 7 code hits**
  (`race_trait_picker.rs` — 5 `record.data.raw_tokens` reads plus 2 `PRE` literals), and
  `render_pcgen_desc` still has 39 code hits in 3 live files. Neither was this cycle's mechanism;
  both are named in **Refused tokens** with counts.

- **Receipt rows (mechanical):**
  ```
  python3 scripts/cycle_scope_gate.py --receipt --since 98d8d96c765854523bab6375a7612b09d3cb169c \
    --before /tmp/wi-before-AT-35-E6-003-SWEEP.json --after docs/work-inventory.json
  since=98d8d96c765854523bab6375a7612b09d3cb169c residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=4020 ratio=n/a builds_recorded=0 pcgen_live_files=75
  ```
  `closed=0` is correct and by design: Epic 6 moves no corpus unit. `pcgen_live_files` **fell**
  81 → 75, and every one of the six fell because a read went away.

- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`, at end of cycle:
  ```
  pattern raw_tokens files=1 hits=5
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=3 hits=39
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=18 hits=459
  pattern DEFINE: files=1 hits=2
  pattern PRE[A-Z]+: files=42 hits=349
  pattern SAB: files=0 hits=0
  pattern DESC: files=27 hits=73
  pattern %CHOICE files=6 hits=44
  pattern %LIST files=10 hits=76
  pattern TYPE= files=31 hits=252
  root src/rules_core files=74 hits=1292
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=7
  identifier_files=4 identifier_hits=44
  live_files=75 live_hits=1299 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Never above the previous receipt's `live_files=81 live_hits=6619`. The count fell **only**
  because reads went away: `scripts/pcgen-residue-baseline.env` was **not** edited, `--rebaseline`
  was **not** run, and no pattern or exclusion was touched.

- **Oracle parity:** **PASS, on the pinned corpus** (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
  local checkout verified on-pin). No `Number` mapping was added, but this cycle moved data the
  converter reads, so the move itself was oracle-checked three ways:
  - **601 of 601** gap rows regenerated byte-identically off the corpus:
    `cargo run --locked --bin gen_feat_gap_tables` rewrote both generated files, and every
    `(RuleSetId, index, key, tokens)` row matched the relocated one exactly (compared=601
    agree=601 disagree=0).
  - **204 of 204** ARG + PU records re-derived straight from `arg_feats.lst` / `pu_feats.lst`:
    `cargo test --test sd27_feat_prerequisite_enforcement -- --ignored` →
    `the_gathered_arg_and_pu_prerequisites_match_the_live_corpus ... ok` (3 passed, 0 failed).
  - the joined-catalog `PRE`-kind census still totals **4,697** modelled occurrences and **2,030**
    prerequisite-bearing records, read through the new lookup
    (`the_pre_kind_census_is_the_real_one`, `the_number_of_records_carrying_any_prerequisite_is_the_real_one`).

  **Not claimed:** the ingest cache under `data/corpus/` was not regenerated and is therefore not
  byte-proven. Regenerating it rewrites license and PI fields, which is its own cycle's work; the
  module doc says so in those words.

- **Movement, four buckets:**
  - **closure:** none by corpus unit (Epic 6 closes none). By this criterion's own currency:
    **5,320 code hits, 6 files and one whole mechanism** — every feat `prerequisites` field in the
    repository, 2,030 of 2,030 token-bearing rows, across 7 record types and 29 books.
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** none. The gate was not touched this cycle; the fall is closure in
    the gate's own currency.

- **Refused tokens:** `BONUS:=459; PRE[A-Z]+:=349; TYPE==252; %LIST=76; DESC:=73; %CHOICE=44;
  render_pcgen_desc=39; raw_tokens=5; DEFINE:=2` (semicolon-separated so no count reads as a
  percentage — `%LIST` and `%CHOICE` are pattern names, not units) — **1,299 code hits in 75
  files**, summing to the gate's `live_hits` line exactly. By mechanism, largest first:

  | mechanism | files | code hits | shape |
  |---|---|---|---|
  | `FeatEffectBonus { qualifiers: &["BONUS:…", …, "PRE…"] }` and its equipment siblings | ~40 | ~900 | `feat_effects.rs`, `equipment_effects*.rs`, `pilot_compute/mod.rs` (419), the `pathfinder_unchained` feature tables. Unlike the feat prerequisites, these arrays **are** read by a live engine — `bonus_stack_reader` / the effect appliers — so this is a converter-side mapping job, not a relocation. The largest single remaining mass. |
  | `description: Some("…\|VarName")` `%N` / `TYPE=` argument tails | ~30 | ~380 | the tails the converter already substitutes |
  | `PRE` literals still inside live gate code | 42 | 349 | now spread thin — the largest single file is `pilot_compute/mod.rs` at 52, down from a 1,454-hit leader |
  | `render_pcgen_desc` | 3 | 39 | replacement exists and is proven: `sheet_rule_catalog::catalog_description_or_fields` (AT-35-E6-003 cycle 12) |
  | `raw_tokens` on the desktop side | 1 | 5 | `apps/desktop/src-tauri/src/race_trait_picker.rs` — the last desktop reader |

  Nine token types, under §8's limit of ten.

- **Discoveries:** **one, and it changed the design.** `(rule_set, key)` — the addressing the
  previous receipt's `Next-cycle scope` prescribed — **collides**: CRB carries two distinct
  `"Combat Expertise"` records with different token sets (`crb/feat_data/combat.rs` lines 25–26,
  the plain feat and the Dirty Trickster variant), and Mythic Adventures deliberately reuses 142
  earlier-book keys. A key-only lookup would have silently paired 143 records with another
  record's prerequisites. The relocation is addressed by `(rule_set, index)` instead, with the
  key carried on every row and asserted at every lookup. Emitted as a `correction` retro event
  (`1789145057211-at-35-e6-003-sweep-d6f539`), and pinned by
  `feat_prereq_tokens::tests::the_colliding_crb_key_keeps_both_distinct_token_sets`.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=75 live_hits=1299` (end) | all source files under the five live roots, comment lines excluded | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=81 live_hits=6619` (start) | same, at `98d8d96c76` | same command, at that tree |
  | `5320 code hits cleared` | of the 6,619 the gate counted at cycle start | `6619 - 1299` |
  | `2030 rows relocated` (1,429 hand-authored + 601 gap) | of the joined catalog's 2,227 records | `feat_prereq_tokens::{HAND_ROW_COUNT, GAP_ROW_COUNT, JOINED_ROW_COUNT}`, asserted by `tests::the_relocated_row_counts_are_the_ones_the_receipt_states` |
  | `2000 field values removed` (1,135 inline + 865 own-line) | the `prerequisites:` record-field occurrences in the 18 live data files | `git diff 98d8d96c76 -- src/rules_core/rules_tables \| grep -cE '^-.*[,{] prerequisites: (Some\|None)'` → 1135, and `… \| grep -cE '^-\s+prerequisites: (Some\|None)'` → 865 (two layouts, no overlap; the 6 `pub prerequisites: Option<…>` declarations match neither) |
  | `601 of 601 gap rows regenerate byte-identically` | all gap rows carrying a `PRE` token | `cargo run --locked --bin gen_feat_gap_tables`, then compare `(RuleSetId, index, key, tokens)` rows before/after |
  | `204 of 204 ARG+PU records match the corpus` | ARG's 187 + PU's 17 | `cargo test --locked --test sd27_feat_prerequisite_enforcement -- --ignored` with `PCGEN_CORPUS_ROOT` set |
  | per-book coverage (130/143/125/187/14/23/98/127/247/135/200) | each book's own hand-authored feat table | `feat_prereq_tokens::tests::the_per_book_coverage_survived_the_move_unchanged` |
  | `rust_lines_changed=4020`, `pcgen_live_files=75` | the cycle's own diff | `python3 scripts/cycle_scope_gate.py --receipt --since 98d8d96c76…` |
  | `0` files in `data/sheet_rules/` carrying ingest vocabulary | all of `data/sheet_rules/` | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |

- **Build scope verified:** run **once**, at the end, after the last figure-moving edit.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`, no `error` line.
  - `cargo test --locked --no-fail-fast -j 6` (with `CORPUS_ROOT` and `PCGEN_CORPUS_ROOT` set) →
    **414 targets, 8,823 passed, 1 failed, 68 ignored**. The one failure is
    `tests/sd19_equipment_equipmods.rs::every_addressable_real_corpus_item_resolves_reaches_equipped_items_and_grounds_through_table_cell`
    — see **Notes**; it is attributed away from this cycle and filed as an incident.
  - `cargo test --locked --lib -j 6` → `3,313 passed, 0 failed, 15 ignored`; the eight new
    `feat_prereq_tokens` gates all pass.
  - `cargo test --locked --test sd27_feat_prerequisite_enforcement -- --ignored` → `3 passed,
    0 failed` — the corpus oracle named under **Oracle parity**.
  - `cargo clippy --locked --tests -j 6` → **0 warnings, 0 errors** (one `items after a test
    module` warning this cycle introduced was self-healed before commit by moving the three
    count constants above `mod tests`).
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, not risen (fell by 5,320).
  - `cargo run --locked --bin sheet_rule_convert -- --check` → `records=49438 converted=49296
    refused=142 rules=70135 var_tables=5293 verdict=PASS (115.5s)`; the 142 refusals are all
    `no_corpus_record`. Identical to cycle 2's line — this cycle moved no converted rule.
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`.
  - `python3 scripts/completion_atlas.py --check` → `done_evidence_violations=0
    missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`.
  - `python3 scripts/token_coverage.py --check` → `non_done=0 … token_types=233 shapes=1
    verdict=PASS`.
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`.
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`.
  - `python3 scripts/denominator_gate.py --check …` → `files_checked=111 violations=0`.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (`PASS  pi-sweep  (11 hits over
    src/rules_core/rules_tables, 11 baseline rows)`; `passed: 1  pi-sweep`).
  - **Desktop crate and frontend: not run — epic cadence.** This cycle touched **no** file under
    `apps/`; `git diff --stat 98d8d96c76..HEAD -- apps/` is empty. They run at the epic wrap-up,
    as §6 step 3 directs.
  - `v06_work_inventory` and `corpus_literal_sweep`: **not run.** No corpus record and no
    classifier changed; `docs/work-inventory.json` is byte-identical to
    `/tmp/wi-before-AT-35-E6-003-SWEEP.json` (`regressed=0 added=0 dropped=0` above proves it).

- **Suite result:** one run, at the final tree: **414 targets, 8,823 passed, 1 failed, 68
  ignored**. +3 passed on cycle 2's 8,820, and the delta is exactly the three
  `sd27_feat_prerequisite_enforcement` corpus-gated tests this run reached that cycle 2's did
  not; this cycle adds 8 lib tests and removes 4, a net that lands inside the lib target's own
  count rather than the target count.

- **Sweep population:** N/A for `corpus_literal_sweep` — no corpus record changed. The PI sweep
  over `src/rules_core/rules_tables` examined the same population before and after and found the
  same **11** baselined hits: the 2,000 removed field values carried no PI, and the generator's
  own pre-write screen reported `pi-screening: CLEAN (0 hits over the generated text)` for both
  files it wrote.

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, and the local
  checkout is on it (`git -C ~/workspace/repos/pcgen rev-parse HEAD`). Every **Oracle parity**
  figure came from that pin.

- **Status:** **partial.** The criterion's population is not zero at HEAD: 1,299 code hits in 75
  files remain, named and summing above. The cycle's own floor (500 code hits) is met at 10.6×,
  the mechanism cycle 2 named was taken **whole** rather than partially, and the remainder is
  named by mechanism — so this is `partial` and not `blocked-escalated`: §8's "under the floor
  and not the whole remainder" does not apply.

- **Notes:** Two things a reader should not have to dig for.

  **One failing test, attributed away from this cycle.**
  `tests/sd19_equipment_equipmods.rs:144` asserts 658 distinct equipmods records; the pinned
  corpus parses **676**. It is `CORPUS_ROOT`-gated and early-returns when the file is absent, so
  every prior SD-35 cycle — which ran without the corpus — recorded it as passing. This cycle set
  `CORPUS_ROOT` for the oracle work above and so is the first to actually execute it.
  It cannot have been caused here: `git diff --name-only 98d8d96c76 | grep -i equip` is empty, and
  the same target passes with `CORPUS_ROOT` unset. It is a latent corpus-vs-table drift in the
  **equipment** lane, outside this criterion's file-touch set. Filed as a retro `incident`
  (`1789147440438-at-35-e6-003-sweep-8e8b3d`, recurrence key
  `corpus-gated-test-never-actually-ran`) and named in **Next-cycle scope** so it gets an owner.
  The fix is to regenerate `equipment_data::equipmods` against the pin and re-derive 658.

  **One retro event landed in the wrong file.** This cycle's first `correction` was emitted from a
  shell where `RETRO_ACTOR` was unset, so `1789145046314` went to
  `docs/retro/events/sd31-transcribe.jsonl`. It was re-emitted under the right actor
  (`1789145057211`) and the misfiling itself emitted as a second `correction`
  (`1789145057371`). The log is append-only, so the stray line stays and is explained rather than
  removed.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`;
  floor 500 code hits. The remainder is no longer one mechanism, so dispatch it as two:
  1. **The `BONUS:` qualifier arrays and their `PRE`/`TYPE=` tails** (~900 hits, ~40 files) —
     `FeatEffectBonus.qualifiers`, `equipment_effects*`, `pilot_compute/mod.rs`. Unlike the feat
     prerequisites this data **is** live-read by the effect appliers, so it is a converter-side
     **mapping** job, not a relocation: the appliers must read a converted `SheetRule` effect
     rather than a token array. Largest remaining mass and the one that decides whether Epic 6
     can reach zero.
  2. **The description tails, `render_pcgen_desc` (39) and the desktop `raw_tokens` reader (5)**
     (~420 hits) — together these close AT-35-E6-003's own Evidence sentence, and the desktop
     crate and frontend suites run with them.

  Separately, and **not** an Epic 6 item: the equipmods 658-vs-676 drift above needs a named
  owner in the equipment lane.
