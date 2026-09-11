# Cycle 2 — Epic 6 (PCGen exit) / AT-35-E6-003-SWEEP

Cycle 1 measured the remainder and refused to start, escalating one question. Operator ruling
**B14** (2026-09-11) answered it: **no**, a live-side doc comment that quotes an ingest-format token
is not a PCGen read. This cycle is the two steps that ruling authorises — make the gate
comment-aware, then clear real code hits grouped by mechanism — in one batch, with one verify pass.

- **Commit SHA:** see `progress.md` (a receipt cannot name the commit that carries it). Cycle start
  `1d478e727b15478d5c979eaf97fc1ec4874ba6fb`.

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design,
  decisions.md §2)`. Run anyway, for the record — `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  This cycle's **own** floor — 500 CODE hits, the currency cycle 1 proved was right — is the one
  that binds, and it is met at **3.5×**: **1,771 code hits cleared**.

- **Files touched:**
  - `scripts/pcgen_residue_gate.py` — comment-aware read (step 1).
  - `scripts/tests/test_pcgen_residue_gate.py` — `TestCommentAwareness`, the RED→GREEN pin.
  - `src/pcgen_import/archetype_swap_prereq_tokens.rs` — **new**, 409 relocated rows.
  - `src/pcgen_import/mod.rs` — registers it.
  - `src/rules_core/rules_tables/archetype_swap.rs` — the dead field removed, with the note.
  - `src/rules_core/rules_tables/{acg,apg,advanced_race_guide,ultimate_combat,ultimate_magic,ultimate_wilderness,ultimate_psionics}/archetype_tables.rs`
    — 409 `prerequisites:` lines moved out.
  - `docs/release/SD-35-corpus-sheet-completion/decisions.md` (§17, ruling B14),
    `epic-breakdown.md` (AT-35-E6-004's evidence), this receipt, `progress.md`, `kanban.md`,
    `docs/retro/events/at-35-e6-003-sweep.jsonl` (+2).

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.**
  ```bash
  git diff --unified=0 1d478e727b -- src/rules_core src/pcgen_import apps/desktop/src-tauri/src \
      apps/desktop/src scripts docs/release/SD-35-corpus-sheet-completion \
      ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   -> 0
  ```

- **Wired-integration audit result:** **OK_NO_TOKENS.** Same diff, `grep -ciE
  '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → `0`. No shipping code
  path was added; one dead field was relocated and one instrument was corrected.

- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met.** `apps/desktop/` still stands at **1 file / 5 code hits**
  (`race_trait_picker.rs` — `record.data.raw_tokens` reads for `ABILITY`, `PREMULT`, `PREABILITY`,
  `!PREFACT`), and `render_pcgen_desc` still has 39 code hits in 3 live files. Neither was this
  cycle's mechanism; both are named in **Refused tokens** with counts.

- **Receipt rows (mechanical):**
  ```
  python3 scripts/cycle_scope_gate.py --receipt --since 1d478e727b15478d5c979eaf97fc1ec4874ba6fb \
    --before /tmp/wi-before-AT-35-E6-003-SWEEP.json --after docs/work-inventory.json
  since=1d478e727b15478d5c979eaf97fc1ec4874ba6fb residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=420 ratio=n/a builds_recorded=0 pcgen_live_files=81
  ```
  `closed=0` is correct and by design: Epic 6 moves no corpus unit. `pcgen_live_files=81` is the
  comment-aware count — **not** a fall from 197 by closure; see the four-bucket row.

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
  pattern PRE[A-Z]+: files=53 hits=5620
  pattern SAB: files=0 hits=0
  pattern DESC: files=27 hits=73
  pattern %CHOICE files=6 hits=44
  pattern %LIST files=10 hits=76
  pattern TYPE= files=38 hits=301
  root src/rules_core files=80 hits=6612
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=7
  identifier_files=4 identifier_hits=44
  live_files=81 live_hits=6619 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Never above the previous receipt's: cycle 1 recorded `live_files=197 live_hits=11447` under the
  old reading; under the same comment-aware reading this cycle's start was
  `live_files=81 live_hits=8390`, and it ends at `81 / 6619`. The count fell **only** because reads
  went away. `scripts/pcgen-residue-baseline.env` was **not** edited and `--rebaseline` was **not**
  run.

- **Oracle parity:** **N/A.** No `Number` mapping was added, and no live path that computes a sheet
  value was touched — the change removes a field nothing read and relocates its data. The removed
  field never reached a sheet line, so there is no value for the oracle to disagree about.

- **Movement, four buckets:**
  - **closure:** none by corpus unit (Epic 6 closes none). By this criterion's own currency:
    **1,771 code hits and one whole mechanism** — every `ArchetypeSwapEntry.prerequisites` row in
    the repository, 409 of 409, across all 7 books that carry an archetype-swap table.
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** **one.** Ruling B14 made `pcgen_residue_gate.py` comment-aware.
    `live_files` read **197 → 81** and `live_hits` **11,447 → 8,390** at an unchanged tree
    (`1d478e727b`). **This cleared no file and closed no unit.** The 81 code-bearing files are
    exactly as unfinished as they were before it was written; they are the same 83 cycle 1's census
    named (81 by the gate's own `\b`-anchored regexes, 83 by the census's looser ones — the gate is
    the authority). Reported as `files cleared` by no row here, and emitted as a `correction` retro
    event (`1789141003593-at-35-e6-003-sweep-efd5eb`, `claimed=197 actual=81`).

- **Refused tokens:** `PRE[A-Z]+:=5620, BONUS:=459, TYPE==301, %LIST=76, DESC:=73, %CHOICE=44,
  render_pcgen_desc=39, raw_tokens=5, DEFINE:=2` — **6,619 code hits in 81 files**, summing to the
  gate's `live_hits` line exactly. By mechanism, largest first:

  | mechanism | files | code hits | shape |
  |---|---|---|---|
  | `prerequisites: Some(&["PRE…"])` on the static **feat** tables | 20 | ~5,620 | `FeatCatalogRecord` plus five book-local entry types share the field; **both** converter consumers (`cache_gen::feat_gap`, `cache_gen::hand_authored_feat_dump`) read it, so the migration is atomic and is the next cycle's whole scope. Led by `feat_gap_tables.rs` (1,467), `ultimate_combat/feat_tables.rs` (794), `ultimate_psionics/feat_tables.rs` (597), `feats_all.rs` (529). |
  | `FeatEffectBonus { qualifiers: &["BONUS:…"] }` | 18 | 459 | live `BONUS:` qualifier arrays in `feat_effects.rs` / `equipment_effects*.rs` |
  | `description: Some("…\|VarName")` `%N`/`TYPE=` argument tails | ~38 | ~421 | the tails the converter already substitutes |
  | `render_pcgen_desc` | 3 | 39 | replacement exists and is proven: `sheet_rule_catalog::catalog_description_or_fields` (cycle 12) |
  | `raw_tokens` on the desktop side | 1 | 5 | `apps/desktop/src-tauri/src/race_trait_picker.rs` — the last desktop reader |

  Nine token types, under §8's limit of ten. `pilot_compute/mod.rs` (419) is a mixture of the
  second and third mechanisms.

- **Discoveries:** **one, and it is a confirmation rather than a surprise** —
  `ArchetypeSwapEntry.prerequisites` had **no consumer anywhere**, live *or* converter, unlike its
  feat-side sibling which two converter modules read. Cycle 1's mechanism table predicted the feat
  field's shape but did not separate the archetype field from it; that separation is what made a
  single-cycle, 1,771-hit mechanism available at all. Re-derive:
  `grep -rn "prerequisites" src apps/desktop/src-tauri/src --include=*.rs | grep -v "rules_tables/" | grep -v failing_prerequisites`
  — no hit names an archetype record. Not emitted as a separate `correction`: it refines cycle 1's
  table rather than contradicting a figure in it.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=81 live_hits=6619` (end) | all source files under the five live roots, comment lines excluded | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=81 live_hits=8390` (start) | same, at `1d478e727b` before the archetype move | same command, at that tree |
  | `1771 code hits cleared` | of the 8,390 the comment-aware gate counted at cycle start | `8390 - 6619` |
  | `197 → 81` / `11447 → 8390` | of the 197 files the pre-ruling gate counted | `git show 1d478e727b:scripts/pcgen_residue_gate.py` run vs the new one, same tree |
  | `409 rows moved` | of 409 `prerequisites:` lines across the 7 archetype tables (all `Some`, none `None`) | `grep -c '^    (' src/pcgen_import/archetype_swap_prereq_tokens.rs` and `git diff --stat 1d478e727b -- 'src/rules_core/rules_tables/*/archetype_tables.rs'` |
  | `comment_hits=2686 code_hits=3628`, `files_comment_only=114`, `max_files_clearable_by_code_work_alone=10` | the 197 pre-ruling live files | `python3 …/AT-35-E6-003-SWEEP_cycle1_residue_shape_census.py` at `18ef3d789f` (cycle 1's figure, cited not re-derived) |
  | `rust_lines_changed=420` | the cycle's own diff | `python3 scripts/cycle_scope_gate.py --receipt --since 1d478e727b…` |
  | `0` files in `data/sheet_rules/` carrying ingest vocabulary | all of `data/sheet_rules/` | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |

- **Build scope verified:** run **once**, at the end, after the last figure-moving edit, at tree
  `1d478e727b` + this cycle's diff.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`, no `error`/`error[...]` line.
  - `cargo test --locked --no-fail-fast -j 6` → `EXIT=0`, **414 targets, 8,820 passed, 0 failed,
    68 ignored, 0 `test result: FAILED`**.
  - `cargo clippy --locked --tests -j 6` → **0 warnings, 0 errors**.
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, not risen (fell by 1,771).
  - `python3 -m unittest scripts.tests.test_pcgen_residue_gate` → `Ran 18 tests … OK`, RED first
    (2 failures) with the comment-aware read absent.
  - `cargo run --locked --bin sheet_rule_convert -- --check` → `records=49438 converted=49296
    refused=142 rules=70135 var_tables=5293 verdict=PASS`; the 142 refusals are all
    `no_corpus_record`.
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`.
  - `python3 scripts/completion_atlas.py --check` → `missing_clearing_mechanisms=0
    stale_derived_at=False citation_failures=0`.
  - `python3 scripts/token_coverage.py --check` → `non_done=0 … token_types=233 shapes=1
    verdict=PASS`.
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`.
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`.
  - `python3 scripts/denominator_gate.py --check …` → `files_checked=110 violations=0` (one
    violation in this cycle's own §17 draft, self-healed before commit).
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (`passed: 1  pi-sweep`).
  - **Desktop crate and frontend: not run — epic cadence.** This cycle touched **no** file under
    `apps/`; `git diff --stat <cycle-start>..HEAD -- apps/` is empty. They run at the epic wrap-up,
    as §6 step 3 directs.
  - `v06_work_inventory` and `corpus_literal_sweep`: **not run.** No corpus record and no
    classifier changed; `docs/work-inventory.json` is byte-identical to
    `/tmp/wi-before-AT-35-E6-003-SWEEP.json` (`regressed=0 added=0 dropped=0` above proves it).

- **Suite result:** one run, at the final tree: **`EXIT=0`, 414 targets, 8,820 passed, 0 failed, 68
  ignored**, no `test result: FAILED` line. +24 on AT-35-E6-003 cycle 2's 8,796 — all from cycles
  landed between; this cycle adds no Rust test and removes none.

- **Sweep population:** N/A — no corpus record changed.

- **Oracle pin:** N/A — no figure here came from the pinned corpus.

- **Status:** **partial.** The criterion's population is not zero at HEAD: 6,619 code hits in 81
  files remain, named and summing above. The cycle's own floor (500 code hits) is met at 3.5×, one
  mechanism was taken **whole** rather than partially, and the remainder is named by mechanism, so
  this is `partial` and not `blocked-escalated`: §8's "under the floor and not the whole remainder"
  does not apply.

- **Notes:** The floor moved from files to code hits because cycle 1 proved files were the wrong
  currency, and the change is visible in the cadence: 1,771 hits and 409 rows in one pass against
  22 prior cycles averaging ~2.8 files. The remaining mass is one atomic migration, not a long tail.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`;
  floor 500 code hits. Dispatch at the **feat-prerequisites mechanism as one unit** — move every
  book's feat `prerequisites` arrays plus `ARG_/PU_/UCA_FEAT_PREREQUISITES` to
  `src/pcgen_import/`, drop the field from `FeatCatalogRecord` and the five book-local entry types,
  and give `cache_gen::feat_gap` and `cache_gen::hand_authored_feat_dump` a `(rule_set, key)`
  lookup (key-only would collide — `feats_all`'s own "Key collisions" section). ~5,620 code hits in
  20 files. Then the `BONUS:` qualifier arrays (459), the `%N`/`TYPE=` description tails (~421),
  `render_pcgen_desc` (39) and the desktop `raw_tokens` reader (5), which together close
  AT-35-E6-003's own Evidence sentence.
