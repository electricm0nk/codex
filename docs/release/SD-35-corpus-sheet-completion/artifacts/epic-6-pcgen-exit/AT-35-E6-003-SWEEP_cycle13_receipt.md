# Cycle AT-35-E6-003-SWEEP cycle 13 — Epic 6 PCGen exit / AT-35-E6-003-SWEEP

- **Commit SHA:** `88b4490e16`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by
  design; decisions.md §2, workflow-instruction.md §6 step 1)`

  Both gates ran anyway. The corpus floor, for the record, at the cycle's start
  tree `bb937e57e1`:
  ```
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree:
  ```
  live_files=49 live_hits=373 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

  **The dispatch's cycle number was wrong for the third consecutive cycle.** It
  said "CYCLE NUMBER FOR THIS CRITERION: 12"; cycle 12's receipt is committed at
  `bb937e57e1` and the refused-token line the dispatch handed on **is** cycle
  12's result, summing to the 373 the gate reported at that tree. This is cycle
  13 (`correction 1789189827898-at-35-e6-003-sweep-5a451a`). Cycles 11 and 12
  each recorded the same defect on their own dispatch; that is three in a row, so
  it is a missing mechanism and not bad luck (`AGENTS.md` rule 8). **The
  mechanism:** the orchestrator should derive the number as
  `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle*_receipt.md | wc -l`
  plus one, at dispatch time, rather than carry it in prose. It has now been
  asked for three times.

- **Files touched:**
  - `src/rules_core/rules_tables/companion_chassis.rs` — the new
    `ExternalAbilityRefCondition` struct, the `CompanionRecord
    .external_ability_ref_conditions` field, the reworded doc comment on
    `external_ability_refs`, and the closure test
    `every_external_ability_ref_condition_names_a_ref_the_row_carries`.
  - 16 shipped `*/companion_data.rs` files — all **452** `CompanionRecord`
    literals gain `external_ability_ref_conditions:`; **3** of them (CRB
    Hippopotamus, Megafauna (Arsinoitherium), Megafauna (Gylptodon)) lose the
    guard element from `external_ability_refs` and gain the typed condition.
  - `src/pcgen_import/companion_pcgen_guards.rs` — the new
    `ExternalAbilityRefsBefore` type, the
    `COMPANION_EXTERNAL_ABILITY_REFS_BEFORE` table holding the three verbatim
    pre-conversion arrays, `rebuild_external_ability_refs`, and three tests.
  - `src/bin/gen_book_cache.rs` — the companion record's
    `"external_ability_refs"` wire field is now written from
    `rebuild_external_ability_refs`, so the book cache is byte-identical across
    the conversion.
  - `tests/sd35_rendered_prose_carries_no_ingest_vocabulary.rs` — the standing
    gate's `SCANNED` list extended by `crb/companion_data.rs` (RED first).
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle13_external_ability_ref_guards.py`
    — the converter script, committed, with `--check`, `--apply-field`,
    `--apply-guards` and `--emit-before-table`.
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    — `derived_at` stamp only, written by `completion_atlas.py --check`.
  - `docs/retro/events/at-35-e6-003-sweep.jsonl`,
    `docs/release/SD-35-corpus-sheet-completion/{progress.md,kanban.md}`, this receipt.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.** On this cycle's own diff,
  ```bash
  git diff --unified=0 bb937e57e1 -- src/rules_core src/pcgen_import src/bin \
    apps/desktop/src-tauri/src tests ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ```
  → three matches, all of them the diff's own `diff --git` / `---` / `+++`
  headers for the pre-existing `sd35_rendered_prose_carries_no_ingest_vocabulary.rs`.
  No identifier in shipping code carries a bundle tag. Same documented
  citation-exclusion class cycles 9–12 recorded.

- **Wired-integration audit result:** **OK_NO_TOKENS**, first run, no fix needed.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` — this sweep criterion
  carries Epic 6's closure bar, `AT-35-E6-004`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge /
  > `reach_gate.rs` readers of `raw_tokens` read `SheetRule.applies` and
  > `SheetRule.prose` instead. `render_pcgen_desc` is deleted from the live side;
  > its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under
  > `apps/desktop/`; desktop crate and frontend suites green; the 19 on-screen
  > tests still pass.

  and `AT-35-E6-004`'s closure bar, which this sweep inherits:

  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11
  > (`decisions.md §17`). […] **Evidence:** `python3
  > scripts/pcgen_residue_gate.py --check --closure` → `live_files=0
  > live_hits=0 verdict=PASS` […]

  Not met at HEAD. The remainder is named by mechanism and by count below and
  sums under **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  since=bb937e57e1 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=711 ratio=n/a builds_recorded=0 pcgen_live_files=48
  ```

- **PCGen residue:**
  ```
  pattern raw_tokens files=1 hits=5
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=3 hits=39
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=14 hits=91
  pattern DEFINE: files=0 hits=0
  pattern PRE[A-Z]+: files=19 hits=63
  pattern SAB: files=0 hits=0
  pattern DESC: files=26 hits=59
  pattern %CHOICE files=2 hits=8
  pattern %LIST files=1 hits=1
  pattern TYPE= files=21 hits=104
  live_files=48 live_hits=370 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Down from cycle 12's `live_files=49 live_hits=373`; never above it. The whole
  movement is in one pattern — `PRE[A-Z]+:` 66 → 63, in 20 → 19 files — which is
  exactly the three guards this cycle typed, and is the check that no hit was
  banked by reclassification.

- **Oracle parity:** **run, and it did not move — byte-identical to cycles 11 and 12.**
  The conversion changes a slice the companion panel renders, not a magnitude,
  so the expectation was no movement; it was still run rather than assumed,
  because "no number moves" is the claim under test.
  ```
  sheet_parity: lines compared=156 agree=154 disagree=2 unverifiable=67;
                chassis compared=382 agree=376 disagree=6 unverifiable=140;
                characters=29 lines=505
  ```
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. Compared field
  by field against `AT-35-E6-003-SWEEP_cycle12_sheet-parity-after.json`:
  `roster`, `summary`, `disagreements` and `results` are **all identical**, and
  so is the document as a whole. The eight named disagreements are the same
  eight, unchanged — the halfling and paladin save totals (6),
  `deterministic_human_fighter_l1 target:WeaponAttack ours=0 oracle=1`, and
  `half_elf_fighter_l1 target:Pool:favored_class ours=1 oracle=2`. None is this
  cycle's. Artifact: `…_cycle13_sheet-parity-after.json`.

- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. Epic 6 closes no corpus unit.
  - **relabel (bucket to bucket):** none. `regressed=0 added=0 dropped=0`;
    `docs/work-inventory.json` is byte-identical to `bb937e57e1`'s.
  - **reachability:** 3 live-side PCGen hits removed, 1 live file to zero. The
    reachable (non-`#[cfg(test)]`) remainder falls **22 → 19** and the files
    carrying it **10 → 9**.
  - **instrument-correction:** the standing gate's `SCANNED` list grew from 12
    files to 13. A **widening**, not a weakening: it put
    `crb/companion_data.rs` under the gate and the file went RED immediately
    (3 lines / 3 hits) before a byte of data was edited.

- **What this cycle actually fixed, and why it was not cosmetic.**
  `CompanionRecord::external_ability_refs` is documented as *ability names this
  row cites that this book does not define*, and
  `apps/desktop/src-tauri/src/companion_catalog.rs:585` copies that slice
  straight into the DTO the player's companion panel renders. Three CRB creature
  rows carried, as a fourth **ability name**, the guard the ingest format had
  appended to the grant token:

  ```
  Special Ability|AUTOMATIC|Hippopotamus Companion Natural Attack|!PRETEMPLATE:1,Hippopotamus Companion Advancement
  ```
  (`data/corpus/core_rulebook/companion/companion_hippopotamus.json`, and the
  same shape for Arsinoitherium and Gylptodon.) So `!PRETEMPLATE:1,Hippopotamus
  Companion Advancement` was reaching the screen in a list of ability names —
  the sheet rule's failure mode exactly (`decisions.md §1`), not merely a token
  sitting in a source file.

  The guard now lives in `external_ability_ref_conditions`, keyed by the ability
  it gates, in cycle 7's `EffectCondition` schema — the same conversion cycle 9
  applied to `CompanionAbilityGrant.conditions`,
  `CompanionDescriptionVariant.conditions` and
  `NaturalAttackDamageBonus.formula`. Nothing was re-interpreted on the way in:
  `negated: true`, `family: "TEMPLATE"`, `items: ["1", "<template name>"]` — no
  count inferred, no comparison parsed out of the family name.

- **How losslessness is proved**, three tests, all in
  `src/pcgen_import/companion_pcgen_guards.rs` (converter side; no live root
  reads it):
  1. `every_converted_external_ability_ref_array_round_trips` — the **whole
     array**, in order, rebuilt from the live typed pair and compared against
     `COMPANION_EXTERNAL_ABILITY_REFS_BEFORE`. The whole array rather than the
     tail alone, because the field lost an element *and* gained a companion
     field: a tail-only record cannot state that the pair rebuilds the original
     order.
  2. `the_rebuilt_array_matches_the_shipped_book_cache` — the rebuild compared
     against `data/corpus/core_rulebook/companion/*.json`, the shipped
     pre-conversion generator output. This is the byte-identity evidence
     `AT-35-E6-002` asks for, taken against the artifact itself rather than
     against a hand-copied expectation.
  3. `no_live_external_ability_ref_is_an_ingest_token` — the other direction,
     over **every registered companion book**: no row's name slice may contain a
     `PRE…:`-shaped string, and the live guarded-row population must equal the
     recorded one. This is what catches a *new* guard arriving with a future
     book rather than only pinning today's three.

  Plus, on the live side,
  `companion_chassis::every_external_ability_ref_condition_names_a_ref_the_row_carries`:
  a name-keyed side field can name something the row does not list, which an
  inline tail could not, so the new failure mode is closed by a test in the same
  cycle that created it.

- **Refused tokens** — eight types, summing to **370**, which is the gate's own
  `live_hits`:
  ```
  TYPE==104, BONUS:=91, PRE[A-Z]+:=63, DESC:=59, render_pcgen_desc=39, %CHOICE=8, raw_tokens=5, %LIST=1
  ```

  Split by what can move without an operator ruling
  (`python3 .../AT-35-E6-003-SWEEP_cycle5_test_region_census.py`, re-run at
  HEAD: `live_hits=370 hits_inside_cfg_test=351 hits_outside=19`):

  | where | hits | files |
  |---|---|---|
  | inside a `#[cfg(test)]` module in a live file | 351 | 39 |
  | reachable by code work | **19** | **9** |

  The reachable 19, by the job that owns each — every one a separate cycle:

  | job | hits | files |
  |---|---|---|
  | `race_trait_picker.rs` exclusion guards read off `raw_tokens` | 7 | `apps/desktop/src-tauri/src/race_trait_picker.rs` |
  | the `PU_*_DESC_TOKEN` verbatim corpus transcriptions | 4 | `pilot_compute/mod.rs` |
  | the `render_pcgen_desc` / `pcgen_desc.rs` catalog rewire | 4 | `class_feature_pool_catalog.rs`, `pcgen_desc.rs`, `pilot_compute/class_feature_grant_consumer.rs` |
  | `equipment_effects` comparing a parsed qualifier to `TYPE=Circumstance` / `TYPE=Enhancement` | 2 | `equipment_effects/{arms_armor,equipmods}.rs` |
  | `CHOOSE:` / pool `TYPE=` prefix constants | 2 | `race_resolver.rs`, `skinwalker_change_shape.rs` |

  **Why none of the four remaining jobs was folded into this cycle.**
  * `race_trait_picker.rs` still needs `SheetRule.applies` to carry the
    exclusion-guard relation before the picker can stop parsing `PREVAREQ:` /
    `PREMULT` / `!PREFACT` itself. It is the only file under `apps/`, so it also
    carries the desktop-crate and frontend suites.
  * The four `PU_*_DESC_TOKEN` constants are the book's own words, pinned
    byte-for-byte against the `.lst` files by
    `sd27_pu_class_feature_descriptions_carry_the_characters_numbers`. Cycles
    10–12 declined to reword them and this cycle declines for the same reason.
    They are **counted** in the remainder, never exempted.
  * The `render_pcgen_desc` rewire is blocked on the converter-side prose
    carrier documented in `…/AT-35-E6-003_cycle5_converter-prose-blocker.md`.
  * The `equipment_effects` pair and the two prefix constants are **one
    mechanism, not five hits**: `equipment_effects` parses `.lst`-shaped
    equipment text at run time and compares the parsed `qualifiers` slice to
    ingest strings, and `race_resolver` / `skinwalker_change_shape` match a
    corpus record's own `CHOOSE:` / grant strings by prefix. Typing any of them
    means the converter must emit a typed qualifier on the equipment and
    trait-grant records — a converter-side widening the size of this cycle's,
    on a record type this cycle did not touch. Starting it at the end of a cycle
    would produce a half-typed field, which is worse than an untyped one.

- **Discoveries:** one, recorded as a `correction`
  (`1789189835346-at-35-e6-003-sweep-aafa26`). Cycle 12's next-cycle scope costed
  this job as "a sweep of this cycle's own size across every book", i.e. the
  290-literal scale. The **guarded** population is **3**, all in one file; the
  sweep is 452 `CompanionRecord` literals gaining one empty field, which is
  mechanical. The estimate was taken from the shared chassis's blast radius
  rather than from the population actually carrying a guard — the
  `every-figure-states-its-denominator` shape.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=48 live_hits=370` | every source file under the five live roots, code lines only (ruling B14) | `python3 scripts/pcgen_residue_gate.py --check` |
  | 373 → 370, 49 → 48 files | same | the same command at `bb937e57e1` and at HEAD |
  | `PRE[A-Z]+:` 66 → 63 in 20 → 19 files | the same scan, one pattern | `python3 scripts/pcgen_residue_gate.py --check` |
  | reachable 22 → 19 in 10 → 9 files | the 370 hits, split on `#[cfg(test)]` membership | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` |
  | 3 guarded refs, the whole corpus population | every `external_ability_refs` literal in the repo | `grep -rn external_ability_refs --include=*.rs src/ apps/ tests/ \| grep -E '!?PRE[A-Z]+:' \| wc -l` (3 before, 0 after) |
  | 452 `CompanionRecord` literals in 16 files | every `CompanionRecord { … }` literal outside the struct definition | `python3 .../AT-35-E6-003-SWEEP_cycle13_external_ability_ref_guards.py --check` |
  | 3 RED lines at the gate widening | `crb/companion_data.rs`, scanned by the standing gate | `cargo test --locked --test sd35_rendered_prose_carries_no_ingest_vocabulary` (RED before `--apply-guards`, GREEN after) |
  | `rust_lines_changed=711` | the cycle's own diff since `bb937e57e1` | `python3 scripts/cycle_scope_gate.py --receipt --since bb937e57e1 --before /tmp/wi-before-at-35-e6-003-sweep.json --after docs/work-inventory.json` |
  | `records=49438 converted=49296 refused=142` | every corpus record the sheet-rule converter reads | `cargo run --locked --bin sheet_rule_convert -- --check` |

- **Build scope verified** — **once**, after the last figure-moving edit:
  
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --no-fail-fast -j 6` → **415 targets / 8,849 passed /
    0 failed / 68 ignored / `FULL_EXIT=0`**, zero `FAILED` lines
    (`grep -c FAILED` → 0). Cycle 12 recorded 8,845 passed; the +4 are this
    cycle's own four new tests.
  - `cargo clippy --locked --tests -j 6` → **`CLIPPY_EXIT=0`, 0 warnings,
    0 errors**
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293
    verdict=PASS`, identical to cycles 3–12
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l`
    → **0**
  - `python3 scripts/pcgen_residue_gate.py --check` → `live_files=48
    live_hits=370 … verdict=PASS`
  - `python3 scripts/completion_atlas.py --check` → `unclassified=0 overlap=0
    done_evidence_violations=0 missing_clearing_mechanisms=0
    citation_failures=0`, exit 0
  - `python3 scripts/token_coverage.py --check` → `refused=142
    refused_non_done=0 token_types=233 verdict=PASS`, exit 0
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, exit 0
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, exit 0
  - `python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'`
    → `files_checked=121 violations=0`
  - `python3 scripts/denominator_gate.py --check-provenance` →
    `files_checked=238 figures_examined=574 violations=0`
  - `./scripts/publish-site-dashboard.sh --check-pin` → pin matches
    `docs/work-inventory.json`
  - `scripts/verify.sh --only pi-sweep` → **RESULT: PASS** (11 hits, 11 baseline
    rows)
  - `cargo run --locked --bin corpus_literal_sweep` — **not run, and why**: no
    corpus record changed this cycle (`git status --porcelain data/` empty), and
    `workflow-instruction.md §6` step 3 runs it only when they do.
  - **desktop crate and frontend: epic cadence.** `apps/` was not touched —
    `git diff --stat bb937e57e1 -- apps/` is empty. The conversion does change
    what `companion_catalog.rs` *serves* (one fewer entry on three records), but
    not a line of its code.

- **Sweep population:** N/A — no corpus record changed. `data/corpus/**` is
  byte-identical at HEAD (`git status --porcelain data/` empty), and the book
  cache's wire shape is held by
  `the_rebuilt_array_matches_the_shipped_book_cache`.

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

- **Status: `partial`.** The criterion's population is not zero at HEAD.

- **Notes:** the judgment call was not folding `equipment_effects` and the two
  prefix constants in as "five cheap hits". They are one mechanism — a live-side
  reader comparing a parsed ingest qualifier to an ingest string — and it needs a
  typed qualifier on the equipment and trait-grant records, which is a converter
  cycle of this cycle's own size on a record type this cycle never opened.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle)`; the whole remainder,
  **370 code hits in 48 files**, of which **19 hits in 9 files** are reachable
  without the `#[cfg(test)]` ruling. The one unblocked job left is the
  **`equipment_effects` / prefix-constant qualifier typing (4 hits, 4 files)**,
  dispatched as a converter-side widening with its own round-trip oracle, the
  shape cycles 9, 12 and 13 all used. The other three reachable jobs are each
  blocked on a named artifact (`SheetRule.applies` carrying the exclusion-guard
  relation; the converter prose carrier; the corpus transcriptions that are
  deliberately verbatim). **The operator ruling on `#[cfg(test)]` regions stands
  between the gate and 351 of the 370** and no amount of dispatching will move
  it; it has now been asked by five cycles.
