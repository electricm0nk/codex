# Cycle 8 — Epic 6 (PCGen exit) / AT-35-E6-003-SWEEP

Cycle 5 deliberately left 34 `equipment_gap_tables.rs` descriptions raw, on a
rule it wrote and then applied one notch too wide: `carries_an_unresolved_
magnitude`. Its own doc comment already drew the right line — *"Nor is a
`%CHOICE`/`%LIST` keyword: those stand in for a CHOICE the player already made
… not for a magnitude"* — but its code never reaches that line, because the
bare-`%` branch returns `true` before the argument tail is ever consulted. So
34 rows whose `%` stands for a **selection**, not a number, were filed under the
magnitude exemption and shipped `Cast % at will|%LIST` to a paper character
sheet. This cycle converts them, on the converter side, using the noun each
row's **own `CHOOSE:` token** names. **545 → 511 code hits, 34 cleared, one file
to zero, none risen.**

- **Commit SHA:** `1f425d5124` carries the cycle's work; this line is written
  into the immediately following docs commit (a receipt cannot name the commit
  that carries it). Cycle start `a2b128e613229b3a42ec21dfaa332fde42c67e6f`.

- **Cycle number:** this is cycle **8**, not the 7 the dispatch prompt named.
  Cycle 7 was already committed at `1d2e061717` with its receipt tracked at
  HEAD (`a2b128e613`), so writing a `cycle7` receipt would have overwritten a
  landed one. Recorded as `correction
  1789167529773-at-35-e6-003-sweep-91620b`. The *remainder* the prompt named
  (`BONUS:=128; TYPE==122; PRE[A-Z]+:=115; DESC:=66; %LIST=63;
  render_pcgen_desc=39; %CHOICE=13; raw_tokens=5; DEFINE:=1`) is cycle 7's end
  state and `pcgen_residue_gate.py --check` reproduced `live_files=65
  live_hits=545` exactly at cycle start — with one drift in the per-pattern
  split, `PRE[A-Z]+:` reading `files=29 hits=108` rather than `31`/`115`
  (the deduplicated `live_hits` total is unchanged at 545; the per-pattern
  lines double-count a line matching two patterns).

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units
  by design, decisions.md §2)`. Run anyway, for the record —
  `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  This criterion's **own** floor — the 500-code-hit currency cycle 1
  established — is **missed**: **34 code hits cleared of the 500 the floor asks
  for** (545 → 511). The scope taken was the **whole remainder** (545 hits in 65
  files, all of it examined mechanism by mechanism), which is what the floor
  rule requires of a cycle that cannot reach 500 any other way. The shortfall is
  named with its cause under **Refused tokens**: 361 of the 511 that remain are
  behind an operator ruling asked for by cycles 5 and 6 and still open, and no
  code work of any size reaches them.

- **Files touched:**
  - `src/bin/gen_equipment_gap_tables.rs` — the converter. Two new functions
    (`chosen_noun`, `substitute_chosen_selection`), `safe_description` gains a
    `choose: Option<&str>` parameter, `BaseFields` gains `choose` so a `.COPY=`
    row inherits its base's selection the same way it already inherits the base's
    prose, `parse_lst` reads the row's `CHOOSE:` token. New test module
    `chosen_selection_tests` (4 tests); the 9 existing `safe_description` call
    sites in `safe_description_tests` pass `None`.
  - `src/rules_core/rules_tables/equipment_gap_tables.rs` — **generated output,
    regenerated**: 34 lines changed, 34 descriptions, nothing else. `git diff
    --stat` → `34 insertions(+), 34 deletions(-)`.
  - `docs/release/SD-35-corpus-sheet-completion/` — this receipt, `progress.md`,
    `kanban.md`; `docs/retro/events/at-35-e6-003-sweep.jsonl`.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.** Run on the cycle's own
  diff (`workflow-instruction.md §6` step 4):
  ```bash
  git diff --unified=0 a2b128e613 -- src/rules_core src/pcgen_import src/bin \
      apps/desktop/src-tauri/src tests docs/release/SD-35-corpus-sheet-completion \
      ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ```
  → no match; the literal `OK_NO_BUNDLE_TAGS` fallback printed.

- **Wired-integration audit result:** **OK_NO_TOKENS.** Same diff,
  `grep -nE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → no match.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` — this sweep criterion
  carries Epic 6's own closure bar, `AT-35-E6-004`):

  > ### AT-35-E6-004 — the gate reads zero
  >
  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11
  > (`decisions.md` §17). […]
  >
  > **Evidence:** `python3 scripts/pcgen_residue_gate.py --check --closure` →
  > `live_files=0 live_hits=0 verdict=PASS` […]

  **Not met.** `live_files=64 live_hits=511` at HEAD. The remainder is named and
  summing under **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  since=a2b128e613229b3a42ec21dfaa332fde42c67e6f target_dir=/tmp/cargo-sd35-AT-35-E6-003-SWEEP residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=288 ratio=n/a builds_recorded=1 pcgen_live_files=64
  ```
  `closed=0` is correct and by design: Epic 6 moves no corpus unit.
  `pcgen_live_files` falls **65 → 64** — `equipment_gap_tables.rs` reached zero.

- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`, at end of
  cycle:
  ```
  pattern raw_tokens files=1 hits=5
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=3 hits=39
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=17 hits=128
  pattern DEFINE: files=1 hits=1
  pattern PRE[A-Z]+: files=29 hits=108
  pattern SAB: files=0 hits=0
  pattern DESC: files=27 hits=66
  pattern %CHOICE files=3 hits=13
  pattern %LIST files=7 hits=29
  pattern TYPE= files=25 hits=122
  root src/rules_core files=63 hits=504
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=7
  identifier_files=4 identifier_hits=44
  live_files=64 live_hits=511 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Never above the previous receipt's `live_files=65 live_hits=545`. The count
  fell **only** because tokens left the live side:
  `scripts/pcgen-residue-baseline.env` was **not** edited, `--rebaseline` was
  **not** run, and no pattern, root or exclusion was touched.

- **Oracle parity:** **N/A — no `Number` mapping was added and no converted rule
  moved.** The cycle changed one generator's prose rendering and its generated
  table; `data/corpus/` was not written, `data/sheet_rules/` was not written,
  and `sheet_rule_convert --check` reproduces cycles 3–7's line field for field
  (below). `PCGEN_ORACLE_SHA` unchanged.

- **Movement, four buckets:**
  - **closure:** none by corpus unit (Epic 6 closes none). By this criterion's
    own currency: **34 code hits**, all in one file, all outside any
    `#[cfg(test)]` region. The test-region census separates them from
    reclassification: `hits_outside` falls **184 → 150** while
    `hits_inside_cfg_test` is **unmoved at 361**.
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** none. The gate was not touched.

- **Per-file movement — every file that moved.** Re-derived by applying the
  gate's own regexes (comment lines excluded) to `git show a2b128e613:<file>`
  and to the file at HEAD, over the union of every live-root source file at
  either tree. **One file moved; no file rose.**

  | file | before | after | cleared |
  |---|---:|---:|---:|
  | `rules_tables/equipment_gap_tables.rs` | 34 | **0** | 34 |
  | | | | **34** |

- **What the player's sheet says now.** The 34 rows, by the noun their own
  `CHOOSE:` token supplied (30 `EQBUILDER.SPELL`, 4 `SKILL`; no row carried any
  other shape, and a row that had would have fallen through untouched):

  | was, shipped verbatim | is |
  |---|---|
  | `Cast % at will\|%LIST` | Cast the chosen spell at will |
  | `Cast % 3/day\|%LIST` | Cast the chosen spell 3/day |
  | `% 1/day\|%LIST` | The chosen spell 1/day |
  | `Item has 5 ranks in %\|%LIST` | Item has 5 ranks in the chosen skill |
  | `…the next check she attempts with % before…\|%LIST` | …the next check she attempts with the chosen skill before… |

- **Refused tokens:** `BONUS:=128; TYPE==122; PRE[A-Z]+:=108; DESC:=66;
  render_pcgen_desc=39; %LIST=29; %CHOICE=13; raw_tokens=5; DEFINE:=1` — **511
  code hits in 64 files** (the per-pattern lines double-count a line matching
  two patterns; the gate's deduplicated `live_hits` is 511). Nine token types,
  under `workflow-instruction.md` §8's limit of ten. By mechanism, largest
  first — re-derived at HEAD, not carried forward from cycle 7:

  | mechanism | files | code hits | shape |
  |---|---|---|---|
  | hits inside a `#[cfg(test)]` module of a live file | 31 files are test-only | **361** | **Unmoved, and unmovable by any cycle.** The residue gate scans whole files, so an assertion inside a live file counts while the identical assertion in `tests/` does not (all of `tests/**` is exempt for being test code). This needs an **operator ruling**, asked for by cycles 5 and 6 and still open — see `progress.md`'s `## Open blockers`. It is 71% of what remains. |
  | `src/rules_core/pcgen_desc.rs` | 1 | 45 (2 outside its test module) | The live prose renderer `AT-35-E6-003` says to delete. Replacement exists and is proven (`sheet_rule_catalog::catalog_description_or_fields`); blocked on `class_feature_pool_catalog.rs` reading the sheet-rule package instead of the ingest cache. |
  | `pilot_compute/mod.rs` unframed prose citation | 1 | 38 | A token named mid-sentence inside a shipped `explanation` string, with no bounded frame. Three rules for these were built, measured against the real files and **removed** across cycles 5 and 6 for producing ungrammatical sheet prose. Hand work. |
  | `bestiary/monster_data.rs` `%CHOICE`/`%LIST` slots | 1 | 20 | `description_variables: &["%CHOICE"]` — a variable-slot name that *is* the ingest pseudo-variable. **Measured this cycle and refused for a mechanism, not for time:** the field is `&'static [&'static str]` with **4,378 literals repo-wide** and is serialised straight into the book cache by `gen_book_cache.rs:1610` and `:1926`, so typing it (`DescriptionVariable::{Var, ChoiceSlot, ListSlot}`) is a 4,378-literal migration plus a wire-format round-trip, not a table edit. |
  | `CompanionDescriptionVariant.conditions` / `NaturalAttackDamageBonus.formula` / `ability_grants` / `external_ability_refs` PRE guards | 5 | ~33 | Unchanged from cycle 7, and re-confirmed: `gen_book_cache.rs:1938` serialises `v.conditions` straight into the book cache, and `external_ability_refs` reaches the desktop wire through `companion_catalog.rs:585`. A book-cache regeneration cycle. |
  | `FeatEffectBonus.qualifiers` selection targets (`WEAPONPROF=%LIST`, `SCHOOL.%LIST`, `SKILL`/`%LIST`) | 3 | 10 | The bonus *target* is the player's chosen weapon/school/skill. Same typing shape cycle 7 applied to the qualifier **tail**, but this is the target the bonus engine matches on, so it moves `damage_total` / `skill_allocation` / `pilot_compute` matching, not a literal. |
  | `race_trait_picker.rs` `raw_tokens` on the desktop side | 1 | 7 | `apps/desktop/src-tauri/src/race_trait_picker.rs` — real `PREVAREQ`/`PREMULT`/`PREABILITY`/`!PREFACT` parsing that decides alternate-trait exclusion. Needs `SheetRule.applies` to carry the exclusion-guard relation; relocating the parse into `src/pcgen_import` would launder the read, not end it. |
  | the rest, one to six hits each | ~21 | ~31 | Single `DESC:`/`PRE`/`TYPE=` citations scattered across book `mod.rs` and table files, plus `ultimate_combat/feat_tables.rs:2174`'s one shipped `\|%LIST` benefit tail; no shared frame, so no single rule reaches them. |

- **Discoveries:** **one.** Cycle 5's `carries_an_unresolved_magnitude` was
  **right about the rule and wrong about one branch of its own code.** The
  function's doc comment states the exemption correctly — a `%CHOICE`/`%LIST`
  keyword is a choice, not a magnitude — but the bare-`%` arm (`_ => return
  true`) fires on `"Cast % at will|%LIST"` before any branch looks at the tail,
  so every row in that shape was filed under the magnitude exemption it was
  explicitly not covered by. Nothing was wrong with the reasoning; the predicate
  simply never asked the question its own prose said to ask.
  **The lesson generalises:** when a refusal rule carries a written exception,
  test the exception, not just the rule — the case the doc comment names is
  exactly the case no test covered, and a green suite plus a correct-sounding
  comment kept it invisible for three cycles.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=64 live_hits=511` (end) | all source files under the five live roots, comment lines excluded | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=65 live_hits=545` (start) | same, at `a2b128e613` | same command, at that tree |
  | `34 code hits cleared`, `0` files risen, `1` to zero | of the 545 the gate counted at cycle start, over the 65 files carrying them | `545 - 511`; per-file movement from the gate's own regexes applied to `git show a2b128e613:<file>` against the working file, over the union of live-root source files at both trees |
  | `hits_outside` 184 → 150, `hits_inside_cfg_test` 361 (unmoved) | the live files carrying hits | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` (unchanged tool, run at both trees) |
  | `34` descriptions converted, `30` spell / `4` skill | every `EquipmentGapRow` whose description carried `%LIST` or `%CHOICE` | `git diff -U0 src/rules_core/rules_tables/equipment_gap_tables.rs` → `34 insertions(+), 34 deletions(-)`; the `CHOOSE:` split from each row's own `data/corpus/**/equipmods/*.json` record |
  | every one of the 34 carries a `CHOOSE:` token | the 34 rows | the census in this cycle's own working notes: each row's key looked up in `data/corpus/**/equipment/**/*.json`, `raw_tokens[key=="CHOOSE"]` present on all 34 |
  | `4,378` `description_variables` literals (why the monster slot family was refused) | all of `src/`, `apps/`, `tests/` | `grep -rh "description_variables:" --include=*.rs src/ apps/ tests/ \| wc -l` |
  | `0` files in `data/sheet_rules/` carrying ingest vocabulary | all of `data/sheet_rules/` | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` → 0 |
  | `rust_lines_changed=288`, `pcgen_live_files=64` | the cycle's own diff | `python3 scripts/cycle_scope_gate.py --receipt --since a2b128e613… --before /tmp/wi-before-AT-35-E6-003-SWEEP.json --after docs/work-inventory.json` |

- **Build scope verified:** run **once**, at the end, after the last
  figure-moving edit.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → ``3,318 passed; 0 failed; 15 ignored` (the 4 new tests live in a bin target, not the lib)`.
  - `cargo test --locked --no-fail-fast -j 6` → `**414 targets, 8,836 passed, 0 failed, 68 ignored, 0 `test result: FAILED` lines**, `FULL_EXIT=0`. (8,832 → 8,836 is exactly the four new `chosen_selection_tests`.)`.
  - `cargo clippy --locked --tests -j 6` → ``CLIPPY_EXIT=0`, 0 warnings.`.
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, not risen
    (fell by 34).
  - `cargo run --locked --bin sheet_rule_convert -- --check` → ``records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS (113.4s)` — identical to cycles 3–7 on every field; the 142 refusals are all `no_corpus_record`. This is the standing proof that no converted rule moved.`.
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`.
  - `python3 scripts/completion_atlas.py --check` → `missing_clearing_mechanisms=0
    stale_derived_at=False citation_failures=0`, `EXIT=0`.
  - `python3 scripts/token_coverage.py --check` → `non_done=0 … token_types=233
    shapes=1 verdict=PASS`, `EXIT=0`.
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, `EXIT=0`.
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, `EXIT=0`.
  - `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-…/*.md'
    'docs/release/SD-35-…/artifacts/**/*.md'` → `files_checked=116 violations=0`.
  - `scripts/verify.sh --only pi-sweep` → ``RESULT: PASS` (`passed: 1 pi-sweep`)`.
  - **Desktop crate and frontend: not run — epic cadence.** This cycle touched
    **no** file under `apps/`; `git diff --stat a2b128e613..HEAD -- apps/` is
    empty.
  - `v06_work_inventory` and `corpus_literal_sweep`: **not run.** No corpus
    record and no classifier changed; `docs/work-inventory.json` is
    byte-identical to `/tmp/wi-before-AT-35-E6-003-SWEEP.json` (`diff -q` →
    identical; `regressed=0 added=0 dropped=0` in the receipt rows agrees).

- **Sweep population:** N/A — no corpus record changed.

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
  verified on-pin before the regeneration (`git rev-parse HEAD` in
  `$PCGEN_REPO_DIR`). The generator reads `$PCGEN_CORPUS_ROOT` and the 34 rows'
  `CHOOSE:` tokens came from it.

- **RED→GREEN preserved, recorded.** The four new tests were run against the
  implementation disabled at its branch (`if false && let Some(noun) = …`) and
  failed for the intended reason:
  ```
  ---- chosen_selection_tests::a_list_tailed_percent_ships_the_noun_its_own_choose_token_names stdout ----
  assertion `left == right` failed
    left: Some("Cast % at will|%LIST")
   right: Some("Cast the chosen spell at will")

  ---- chosen_selection_tests::a_sentence_initial_percent_is_capitalised stdout ----
  assertion `left == right` failed
    left: Some("% 1/day|%LIST")
   right: Some("The chosen spell 1/day")

  ---- chosen_selection_tests::the_shipped_sentence_carries_no_ingest_vocabulary stdout ----
  Cast % 3/day|%LIST

  test result: FAILED. 1 passed; 3 failed
  ```
  The fourth (`every_shape_the_rule_does_not_claim_falls_through_untouched`)
  passes in both states **by design** — it asserts the negative space, so it
  must be green before the change as well as after.

- **Retro events emitted:**
  `correction 1789167529773-at-35-e6-003-sweep-91620b` (the dispatch's cycle
  number), `correction 1789167769730-at-35-e6-003-sweep-8458cc` (cycle 5's
  `carries_an_unresolved_magnitude` branch), `deferral 1789167781790-at-35-e6-003-sweep-54711e` (the 511-hit
  remainder, mechanism by mechanism).

- **Status: `partial`.** The criterion's population is not zero at HEAD.

- **Notes:** the cycle is small in hits and complete in mechanism: every row in
  the shape it took is converted, and the shape it took is the one cycle 5 named
  and left. The three largest remaining mechanisms are each blocked on something
  no cycle of this criterion can produce alone — an operator ruling (361), a
  book-cache wire-format regeneration (~53), and a `SheetRule.applies` relation
  the converter does not yet carry (7).

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle)`; the whole remainder,
  **511 code hits in 64 files**. The 361 behind the `#[cfg(test)]` ruling cannot
  move without it, so the reachable batch is **150 hits in 34 files** and the
  largest tractable mechanism in it is the book-cache-serialised
  `description_variables` / `conditions` family (~53 hits) — a cycle that
  regenerates the book cache, which is the shape `AT-35-E6-002`'s byte-identity
  Evidence was never meant to forbid.
