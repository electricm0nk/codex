# Cycle 7 — Epic 6 (PCGen exit) / AT-35-E6-003-SWEEP

Cycle 6 cleared 133 hits and named `~110` feat-qualifier hits as the mechanism
it deliberately did **not** grind, because the only test deciding
conditioned-vs-unconditional did it by `q.starts_with("PRE")` and a text edit
would have destroyed a published count while the gate applauded. This cycle
took that mechanism corpus-wide by **converting** it instead of stripping it —
the tail became typed fields in this crate's own schema — and took the
class-skill family wildcard the same way. Both cleared in full. **665 → 545
code hits, 120 cleared, 4 files to zero, none risen.**

- **Commit SHA:** `1d2e061717` carries the cycle's work; this line is written
  into the immediately following docs commit (a receipt cannot name the commit
  that carries it). Cycle start `46f683c024f0fefc631a025846b4dc205a1f5b6d`.

- **Cycle number:** this is cycle **7**, not the 6 the dispatch prompt named.
  Cycle 6 was already committed at `2bf2537060` with its receipt tracked at
  HEAD, so writing a `cycle6` receipt would have overwritten a landed one.
  Recorded as `correction 1789166597487-at-35-e6-003-sweep-5455a6`. The
  *remainder* the prompt named (`TYPE==190; PRE[A-Z]+:=160; BONUS:=128;
  DESC:=66; %LIST=63; render_pcgen_desc=39; %CHOICE=13; raw_tokens=5;
  DEFINE:=1`, 665 hits) was itself correct — it is cycle 6's end state, and
  `pcgen_residue_gate.py --check` reproduced it exactly at cycle start.

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
  established — is **missed**: **120 code hits cleared of the 500 the floor asks
  for** (the gate's own currency, 665 → 545). The scope taken was the **whole
  remainder** (665 hits in 69 files, all of it), which is what the floor rule
  requires of a cycle that cannot reach 500 any other way. What was not reached
  is the *clearing* target; the shortfall is named with its cause under
  **Refused tokens**.

- **Files touched:**
  - `src/rules_core/rules_tables/crb/feats.rs` — `FeatEffectBonus` gains
    `bonus_type: Option<&'static str>` and `conditions: &'static
    [EffectCondition]`; the new `EffectCondition` and `ConditionItem` types are
    declared here and shared by all four catalogs.
  - `src/rules_core/rules_tables/advanced_race_guide/feats.rs` — the same two
    fields on ARG's own `FeatEffectBonus`, re-exporting the CRB condition types
    rather than re-declaring them.
  - `crb/feat_data/{combat,general,metamagic}.rs`,
    `apg/feat_data/{combat,general}.rs`,
    `acg/feat_data/{combat,general,panache}.rs`,
    `advanced_race_guide/feat_data/{combat,general,teamwork}.rs` — **282
    `FeatEffectBonus` literals rewritten**, 75 of them carrying a tail.
  - `src/pcgen_import/feat_effect_conditions.rs` — **new**, generated. The 75
    verbatim ingest tails, addressed by `(catalog, feat key, index)`, plus the
    two round-trip tests.
  - `src/pcgen_import/mod.rs` — registers that module.
  - `src/rules_core/rules_tables/crb/class_skill_tables.rs` — new
    `ClassSkillEntry::{Named, Family}`; all 10 rows retyped (21 hits → 0 outside
    tests); the corpus verification test rebuilds each row's `CSKILL:` token
    from the typed entries.
  - `src/rules_core/skill_allocation.rs` — `FULL_WIZARD_CLASS_SKILLS`,
    `full_fighter_class_skills`, `is_full_class_skill` and
    `expand_raw_class_skill_list` all take `ClassSkillEntry`; both
    `strip_prefix("TYPE=")` call sites became matches.
  - `src/rules_core/damage_total.rs` — the behaviour-preserving guard (see
    **Discoveries**), and four test constructors.
  - `tests/sd27_arg_and_pu_feat_effects.rs` — `bonus_is_conditioned` reads the
    field instead of the token prefix.
  - `tests/sd19_feat_catalog.rs`, `tests/v06_apg_acg_feat_catalog.rs` — four
    pinned literals gain the two new fields.
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle7_type_feat_effect_tails.py`
    — **new**, the converter and its `--check`.
  - `docs/release/SD-35-corpus-sheet-completion/` — this receipt, `progress.md`,
    `kanban.md`; `docs/retro/events/at-35-e6-003-sweep.jsonl`.
  - Folded from the shared checkout, not this cycle's own work:
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (the `derived_at` stamp this cycle's `completion_atlas.py --check` moved)
    and `docs/retro/events/sd31-transcribe.jsonl` (one event-log append).

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.** Run on the cycle's own
  diff (`workflow-instruction.md §6` step 4):
  ```bash
  git diff --unified=0 46f683c024..HEAD -- src/rules_core src/pcgen_import src/bin \
      apps/desktop/src-tauri/src tests docs/release/SD-35-corpus-sheet-completion \
      ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   -> 2
  ```
  Both `2` are diff **file headers** (`+++ b/tests/sd19_feat_catalog.rs`,
  `+++ b/tests/sd27_arg_and_pu_feat_effects.rs`) — pre-existing filenames this
  cycle edited, not added code. No added line carries a bundle tag.

- **Wired-integration audit result:** **OK_NO_TOKENS.** Same diff,
  `grep -cE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → `0`. Unlike cycles 4–6 this cycle's generator docstring uses none of them.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` — this sweep criterion
  carries Epic 6's own closure bar, `AT-35-E6-004`):

  > ### AT-35-E6-004 — the gate reads zero
  >
  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11
  > (`decisions.md` §17). […]
  >
  > **Evidence:** `python3 scripts/pcgen_residue_gate.py --check --closure` →
  > `live_files=0 live_hits=0 verdict=PASS` […]

  **Not met.** `live_files=65 live_hits=545` at HEAD. The remainder is named and
  summing under **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  since=46f683c024f0fefc631a025846b4dc205a1f5b6d target_dir=/tmp/cargo-sd35-AT-35-E6-003-SWEEP residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1624 ratio=n/a builds_recorded=3 pcgen_live_files=65
  ```
  `closed=0` is correct and by design: Epic 6 moves no corpus unit.
  `pcgen_live_files` falls **69 → 65** — four files reached zero.

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
  pattern PRE[A-Z]+: files=31 hits=115
  pattern SAB: files=0 hits=0
  pattern DESC: files=27 hits=66
  pattern %CHOICE files=3 hits=13
  pattern %LIST files=8 hits=63
  pattern TYPE= files=25 hits=122
  root src/rules_core files=64 hits=538
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=7
  identifier_files=4 identifier_hits=44
  live_files=65 live_hits=545 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Never above the previous receipt's `live_files=69 live_hits=665`. The count
  fell **only** because tokens left the live side:
  `scripts/pcgen-residue-baseline.env` was **not** edited, `--rebaseline` was
  **not** run, and no pattern, root or exclusion was touched.

- **Oracle parity:** **N/A — no `Number` mapping was added and no converter
  input moved.** No `data/corpus/` record and no converted rule moved, which
  `sheet_rule_convert --check` reproducing cycles 3–6's line field for field is
  the standing proof of. `PCGEN_ORACLE_SHA` unchanged; `scripts/pcgen-oracle-pin.env`
  untouched.

- **Movement, four buckets:**
  - **closure:** none by corpus unit (Epic 6 closes none). By this criterion's
    own currency: **120 code hits**. The test-region census separates them from
    reclassification: `hits_outside` falls **305 → 184** (121 cleared) while
    `hits_inside_cfg_test` rises **360 → 361**. That **+1 is real and is
    named**: `class_skill_lists_match_their_own_corpus_records` now writes
    `format!("TYPE={family}")` to rebuild the corpus token it verifies against.
    It is the round-trip proof itself, inside a `#[cfg(test)]` module, and
    spelling it around the gate would be the masking cycles 4 and 5 were burned
    by. Net across both regions: 665 − 545 = **120**.
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** none. The gate was not touched.

- **Per-file movement — every file that moved, summing to 120.** Re-derived by
  applying the gate's own regexes (comment lines excluded) to
  `git show 46f683c024:<file>` and to the file at HEAD. **No file rose.**

  | file | before | after | cleared |
  |---|---:|---:|---:|
  | `advanced_race_guide/feat_data/general.rs` | 48 | 1 | 47 |
  | `crb/feat_data/combat.rs` | 34 | 5 | 29 |
  | `crb/class_skill_tables.rs` | 21 | 1 | 20 |
  | `crb/feat_data/general.rs` | 10 | 4 | 6 |
  | `apg/feat_data/combat.rs` | 6 | **0** | 6 |
  | `skill_allocation.rs` | 8 | 3 | 5 |
  | `apg/feat_data/general.rs` | 4 | **0** | 4 |
  | `advanced_race_guide/feat_data/combat.rs` | 2 | **0** | 2 |
  | `acg/feat_data/general.rs` | 1 | **0** | 1 |
  | | | | **120** |

  **4 files reached zero.** The residues in the five that did not are all
  `#[cfg(test)]`-region assertions or `DESC:`/`%LIST` inside a row's own
  `benefit:` prose — a different mechanism from the one this cycle took, and
  `crb/class_skill_tables.rs`'s remaining `1` is the round-trip proof's own
  `format!("TYPE={family}")`.

- **Refused tokens:** `BONUS:=128; TYPE==122; PRE[A-Z]+:=115; DESC:=66;
  %LIST=63; render_pcgen_desc=39; %CHOICE=13; raw_tokens=5; DEFINE:=1` — **545
  code hits in 65 files**, summing to the gate's `live_hits` line exactly. Nine
  token types, under `workflow-instruction.md` §8's limit of ten. By mechanism,
  largest first:

  | mechanism | files | code hits | shape |
  |---|---|---|---|
  | hits inside a `#[cfg(test)]` module of a live file | 29 files are test-only | **361** | **Unmoved, and unmovable by any cycle.** The residue gate scans whole files, so an assertion inside a live file counts while the identical assertion in `tests/` does not (all of `tests/**` is exempt for being test code). This needs an **operator ruling**, asked for by cycles 5 and 6 and still open — see `progress.md`'s `## Open blockers`. |
  | `src/rules_core/pcgen_desc.rs` | 1 | 49 | The live prose renderer `AT-35-E6-003` says to delete. Replacement exists and is proven (`sheet_rule_catalog::catalog_description_or_fields`); blocked on `class_feature_pool_catalog.rs` reading the sheet-rule package instead of the ingest cache. |
  | `equipment_gap_tables.rs` raw `%` magnitudes | 1 | 34 | **Kept raw on purpose** by cycle 5's `carries_an_unresolved_magnitude` rule: a `%` standing in for a NUMBER renders away to a plausible wrong sheet line (`+%d10` → `d10`). Not clearable without the magnitude the row does not have. |
  | `CompanionDescriptionVariant.conditions` / `NaturalAttackDamageBonus.formula` / `ability_grants` PRE guards | 5 | ~33 | **Attempted and refused this cycle, for a mechanism, not for time.** The conversion is the same shape the feat tails took — but `src/bin/gen_book_cache.rs:1938` serialises `v.conditions` **straight into the book cache**, and `AT-35-E6-002`'s own Evidence sentence pins `gen_book_cache` output byte-identical. Converting the field changes the wire format, so it is a book-cache regeneration cycle, not a table edit. |
  | `bestiary/monster_data.rs` `%CHOICE`/`%LIST` slots | 1 | 20 | Live-read `%N` substitution slots. A converter job, not a relocation: these have real live readers. |
  | `pilot_compute/mod.rs` unframed prose citation | 1 | 38 | A token named mid-sentence with no bounded frame. Three rules for these were built, measured against the real files and **removed** across cycles 5 and 6 for producing ungrammatical sheet prose. Hand work. |
  | `raw_tokens` on the desktop side | 1 | 5 (7 gate hits) | `apps/desktop/src-tauri/src/race_trait_picker.rs` — real `PREVAREQ`/`PREMULT`/`PREABILITY`/`!PREFACT` parsing that decides alternate-trait exclusion. Needs `SheetRule.applies` to carry the exclusion-guard relation. |
  | the rest, one to six hits each | ~25 | ~45 | Single `DESC:`/`PRE` citations scattered across book `mod.rs` and table files; no shared frame, so no single rule reaches them. |

- **Discoveries:** **one, and it is a trap the conversion could have walked
  into silently.** `damage_total::constant_damage_bonus` decides whether a feat
  bonus is a flat constant by `qualifiers.len() != 3`. Before this cycle the
  stacking label and every guard **were** extra `qualifiers` elements, so that
  one length check excluded every typed or conditioned bonus as a side effect.
  Moving the tail into its own fields would have shortened those lists to
  exactly 3 and **silently admitted** typed/guarded bonuses into the damage
  total — a rules change wearing a refactor's clothes, with a green suite,
  because no test pinned the exclusion by name. The explicit guard
  `bonus.bonus_type.is_some() || !bonus.conditions.is_empty()` restores the
  excluded set exactly, and says in the code that whether a typed damage bonus
  *should* contribute is a rules question this exit cycle does not answer.
  **The lesson generalises:** when a conversion changes the shape of a
  collection, find every predicate that reads the collection's *length* — those
  are the call sites a type change moves without mentioning it.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=65 live_hits=545` (end) | all source files under the five live roots, comment lines excluded | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=69 live_hits=665` (start) | same, at `46f683c024` | same command, at that tree |
  | `120 code hits cleared`, `0` files risen, `4` to zero | of the 665 the gate counted at cycle start, over the 69 files carrying them | `665 - 545`; per-file movement from the gate's own regexes applied to `git show 46f683c024:<file>` against the working file |
  | `hits_outside` 305 → 184, `hits_inside_cfg_test` 360 → 361 | the live files carrying hits | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` (unchanged tool, run at both trees) |
  | `282` literals rewritten, `75` carrying a tail | every `FeatEffectBonus { qualifiers: &[…] }` in `rules_tables/*/feat_data/*.rs` and `rules_tables/*/feats.rs` | `python3 …_cycle7_type_feat_effect_tails.py --check` → on an applied tree, `untyped_literals=0 … already applied` |
  | `75` tails round-trip losslessly | the 75 converted bonuses | `cargo test --locked --lib pcgen_import::feat_effect_conditions` → 2 passed |
  | ARG split still `133/5/49` | ARG's 187 shipped feats | `cargo test --locked --test sd27_arg_and_pu_feat_effects args_187_feats_split_133_prose_only_5_pre_gated_and_49_unconditionally_bonused` |
  | `10` class-skill rows retyped, token rebuilt exactly | `CLASS_SKILL_LISTS`' own rows against their live corpus records | `cargo test --locked --lib class_skill_lists_match_their_own_corpus_records` |
  | `0` files in `data/sheet_rules/` carrying ingest vocabulary | all of `data/sheet_rules/` | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` → 0 |
  | `rust_lines_changed=1624`, `pcgen_live_files=65` | the cycle's own diff | `python3 scripts/cycle_scope_gate.py --receipt --since 46f683c024… --before /tmp/wi-before-AT-35-E6-003-SWEEP.json --after docs/work-inventory.json` |

- **Build scope verified:** run **once**, at the end, after the last
  figure-moving edit.
  - `cargo build --locked --lib -j 6` → clean, 0 warnings.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → `3,318 passed; 0 failed; 15 ignored`.
  - `cargo test --locked --no-fail-fast -j 6` → **414 targets, 8,832 passed, 0 failed, 68 ignored, 0 `test result: FAILED` lines**, `FULL_EXIT=0`. (8,830 → 8,832 is exactly the two new round-trip tests.)
  - `cargo clippy --locked --tests -j 6` → `CLIPPY_EXIT=0`, 0 warnings.
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, not risen
    (fell by 120).
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS (114.7s)`
    — identical to cycles 3–6 on every field; the 142 refusals are all
    `no_corpus_record`. This is the standing proof that no converted rule moved.
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`.
  - `python3 scripts/completion_atlas.py --check` → `done_evidence_violations=0
    missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`,
    `EXIT=0`.
  - `python3 scripts/token_coverage.py --check` → `non_done=0 … token_types=233
    shapes=1 verdict=PASS`, `EXIT=0`.
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, `EXIT=0`.
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, `EXIT=0`.
  - `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-…/*.md'
    'docs/release/SD-35-…/artifacts/**/*.md'` → `files_checked=115 violations=0`.
  - `python3 scripts/denominator_gate.py --check-provenance` → `files_checked=231
    figures_examined=573 violations=0`.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (`passed: 1 pi-sweep`).
  - **Desktop crate and frontend: not run — epic cadence.** This cycle touched
    **no** file under `apps/`; `git diff --stat 46f683c024..HEAD -- apps/` is
    empty.
  - `v06_work_inventory` and `corpus_literal_sweep`: **not run.** No corpus
    record and no classifier changed; `docs/work-inventory.json` is
    byte-identical to `/tmp/wi-before-AT-35-E6-003-SWEEP.json` (`diff -q` →
    identical; `regressed=0 added=0 dropped=0` in the receipt rows agrees).

- **RED→GREEN preserved, recorded.** The round-trip oracle was written with an
  **empty** table and failed for the intended reason before the generator filled
  it:
  ```
  ---- pcgen_import::feat_effect_conditions::tests::every_converted_tail_round_trips_from_the_live_typed_form stdout ----
  panicked at src/pcgen_import/feat_effect_conditions.rs:113:9:
  the round-trip table must not be empty -- an empty table proves nothing

  ---- pcgen_import::feat_effect_conditions::tests::the_table_addresses_exactly_the_live_bonuses_that_carry_a_tail stdout ----
  assertion `left == right` failed
    left: []
   right: [("acg", "Steadfast Personality", 1), ("apg", "Breadth of Experience", 1), … 75 rows …]

  test result: FAILED. 0 passed; 2 failed
  ```
  Restoring the generated table turned both green with no other change.

- **Retro events emitted:**
  `correction 1789166597487-at-35-e6-003-sweep-5455a6` (the dispatch's cycle
  number), `deferral 1789166611125-at-35-e6-003-sweep-a71781` (the 545-hit
  remainder, mechanism by mechanism).

- **Status: `partial`.** The criterion's population is not zero at HEAD.
