# Cycle 6 — Epic 6 (PCGen exit) / AT-35-E6-003-SWEEP

Cycle 5 fixed ingest tokens printing on the sheet from the shipped tables and
named 798 code hits as the remainder. This cycle took three of those mechanisms
corpus-wide and cleared each in full: the Ultimate Magic catalog's verbatim
`effect` token array moved off the live side; `CASTER_LEVEL_RULES`' two verbatim
token fields — which a test had **required** on the rendered sheet line since
v0.6 — were demoted to `//` provenance; and the prose demoter gained one narrow
frame that answers where cycles 4 and 5 could only mask. **798 → 665 code hits,
133 cleared, no file risen.**

- **Commit SHA:** `2bf2537060` carries the cycle's work; this line is written
  into it by the immediately following docs commit (a receipt cannot name the
  commit that carries it). Cycle start
  `1b799159def07210fdf22cffe2962e4a9a9f347e`.

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by
  design, decisions.md §2)`. Run anyway, for the record —
  `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  This criterion's **own** floor — the 500-code-hit currency cycle 1 established
  — is **missed**: **133 code hits cleared of the 500 the floor asks for** (the
  gate's own currency, 798 → 665). The scope taken was the **whole remainder**
  (798 hits in 69 files, all of it), which is what the floor rule requires of a
  cycle that cannot reach 500 any other way. What was not reached is the
  *clearing* target, and the shortfall is named with its cause under **Notes**
  and **Refused tokens**.

- **Files touched:**
  - `src/pcgen_import/feat_effect_tokens.rs` — **new**. The 43 relocated
    `BONUS:`/`DEFINE:` token rows of the Ultimate Magic feat catalog, addressed
    by `(RuleSetId::Um, index, key)`, plus three round-trip tests.
  - `src/pcgen_import/mod.rs` — registers that module.
  - `src/rules_core/rules_tables/ultimate_magic/feat_tables.rs` — the
    `effect: Option<&'static [&'static str]>` field and all 144 of its rows
    removed (75 hits → 0); its three tests read the relocated table.
  - `src/rules_core/pilot_compute/mod.rs` — `CasterLevelRule` loses `token` and
    `resolution`, gains `names_class_level_directly: bool`; 17 rows carry their
    verbatim tokens as `//` provenance; three rendered strings rewritten to the
    rule's words; seven prose blocks cut by the cycle-6 frame (116 hits → 58).
  - `tests/v06_caster_level_every_casting_class.rs` — the assertion that
    required `BONUS:CASTERLEVEL` on the rendered sheet line is inverted (see
    **Discoveries**).
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle6_prose_citation_demote.py`
    — **new**, cycle 5's tool plus one narrowing frame.
  - `…_cycle6_relocate_um_effect_tokens.py` — **new**, the relocation generator
    and its `--check`.
  - `…_cycle6_caster_level_provenance.py` — **new**, the table half of the
    caster-level demotion.
  - `docs/release/SD-35-corpus-sheet-completion/` — this receipt, `progress.md`,
    `kanban.md`; `docs/retro/events/at-35-e6-003-sweep.jsonl`.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.** Run on the final committed
  diff (`workflow-instruction.md §6` step 4):
  ```bash
  git diff --unified=0 1b799159de..HEAD -- src/rules_core src/pcgen_import src/bin \
      apps/desktop/src-tauri/src tests docs/release/SD-35-corpus-sheet-completion \
      ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   -> 0
  ```

- **Wired-integration audit result:** **OK_NO_TOKENS** for shipping code. Same
  diff, `grep -cE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → `6` added lines carrying it, and all six are the English word *placeholder* inside
  `…_cycle6_prose_citation_demote.py`'s docstring and comments, describing the
  refusal rule ("a sentence carrying a `{...}` format placeholder is KEPT").
  That file is not shipping code and lives under `docs/release/`. The same
  disposition cycles 4 and 5 recorded; no Rust line added by this cycle carries
  any of the tokens (`grep` over the diff restricted to `*.rs` → `0`).

- **Acceptance criterion** (verbatim, `epic-breakdown.md` — this sweep criterion
  carries Epic 6's own closure bar, `AT-35-E6-004`):

  > ### AT-35-E6-004 — the gate reads zero
  >
  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11
  > (`decisions.md` §17). A live-side doc comment that quotes an ingest-format
  > token is provenance, not a read, and the gate no longer counts one. […]
  >
  > **Evidence:** `python3 scripts/pcgen_residue_gate.py --check --closure` →
  > `live_files=0 live_hits=0 verdict=PASS`, wired as the stage's closure mode
  > from this cycle on. […]

  **Not met.** `live_files=69 live_hits=665` at HEAD. The remainder is named and
  summing under **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  since=1b799159def07210fdf22cffe2962e4a9a9f347e target_dir=/tmp/cargo-sd35-AT-35-E6-003-SWEEP residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=707 ratio=n/a builds_recorded=3 pcgen_live_files=69
  ```
  `closed=0` is correct and by design: Epic 6 moves no corpus unit.
  `pcgen_live_files` is **69, unchanged** — this cycle cleared 133 hits without
  taking any file to zero, because the two files it emptied of one mechanism
  each still carry others: `ultimate_magic/feat_tables.rs` falls 80 → 5 (the 5
  are `DESC:`/`%LIST` inside its own `benefit:` prose and two test assertions),
  and `pilot_compute/mod.rs` falls 116 → 58. **None rose.**

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
  pattern PRE[A-Z]+: files=35 hits=160
  pattern SAB: files=0 hits=0
  pattern DESC: files=27 hits=66
  pattern %CHOICE files=3 hits=13
  pattern %LIST files=8 hits=63
  pattern TYPE= files=29 hits=190
  root src/rules_core files=68 hits=658
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=7
  identifier_files=4 identifier_hits=44
  live_files=69 live_hits=665 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Never above the previous receipt's `live_files=69 live_hits=798`. The count
  fell **only** because tokens left the live side:
  `scripts/pcgen-residue-baseline.env` was **not** edited, `--rebaseline` was
  **not** run, and no pattern, root or exclusion was touched.

- **Oracle parity:** **N/A — no `Number` mapping was added and no converter input
  moved.** The cycle relocated a token array between two source files, rewrote
  three rendered strings, and cut citation spans out of prose; no
  `data/corpus/` record and no converted rule moved, which
  `sheet_rule_convert --check` reproducing cycle 5's line field for field is the
  standing proof of. `PCGEN_ORACLE_SHA` unchanged and the local checkout on-pin
  (`7f818006e3…`).

- **Movement, four buckets:**
  - **closure:** none by corpus unit (Epic 6 closes none). By this criterion's
    own currency: **133 code hits**, all of them outside `#[cfg(test)]` modules
    — the census's `hits_outside` falls 438 → 305 while `hits_inside_cfg_test`
    stays at exactly 360, which is the proof that nothing was cleared by
    reclassification. Per file: `ultimate_magic/feat_tables.rs` 80 → 5 (75 cleared),
    `pilot_compute/mod.rs` 116 → 58 (58 cleared); 75 + 58 = 133, the whole
    movement. **No file rose.**
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** none. The gate was not touched.

- **Refused tokens:** `TYPE==190; PRE[A-Z]+:=160; BONUS:=128; DESC:=66;
  %LIST=63; render_pcgen_desc=39; %CHOICE=13; raw_tokens=5; DEFINE:=1` — **665
  code hits in 69 files**, summing to the gate's `live_hits` line exactly. Nine
  token types, under `workflow-instruction.md` §8's limit of ten. By mechanism,
  largest first:

  | mechanism | files | code hits | shape |
  |---|---|---|---|
  | hits inside a `#[cfg(test)]` module of a live file | 29 files are test-only | **360** | **Unmoved by this cycle, and unmovable by any cycle.** The residue gate scans whole files, so an assertion written inside a live file counts while the identical assertion in `tests/` does not (all of `tests/**` is exempt for being test code). 29 of the 69 remaining files carry **no** hit outside a test module. This needs an **operator ruling**, exactly as B14 did — see **Open question for the operator** below. |
  | `FeatEffectBonus { qualifiers: &[…] }` PRE/`TYPE=` elements | ~10 | ~110 | The converter-side **classification** job cycle 5's Discoveries named. Deliberately not attempted here: `tests/sd27_arg_and_pu_feat_effects.rs::bonus_is_conditioned` decides conditioned-vs-unconditional by `q.starts_with("PRE")`, and that decision produces the published 133/5/49 ARG split — a cycle that stripped the qualifiers by rule would silently destroy a derived count while the gate applauded. |
  | `src/rules_core/pcgen_desc.rs` | 1 | 49 | The live prose renderer `AT-35-E6-003` says to delete. Replacement exists and is proven (`sheet_rule_catalog::catalog_description_or_fields`); blocked on `class_feature_pool_catalog.rs` reading the sheet-rule package instead of the ingest cache. |
  | `equipment_gap_tables.rs` raw `%` magnitudes | 1 | 34 | **Kept raw on purpose** by cycle 5's `carries_an_unresolved_magnitude` rule: a `%` standing in for a NUMBER renders away to a plausible wrong sheet line (`+%d10` → `d10`). Not clearable without the magnitude the row does not have. |
  | `crb/class_skill_tables.rs` `"TYPE=Craft"` selectors | 1 | 21 | Live data the skill engine reads: a PCGen skill-type selector standing for a whole family. Needs the converter to expand the family, not a text edit. |
  | `description_variables` / `description_variants` `%CHOICE`/`%LIST` slots | ~4 | ~36 | Live-read `%N` substitution slots (`derived_evaluator_fixture_check`, `companion_catalog`). A converter job, not a relocation: unlike `UmFeatEntry.effect`, these have real live readers. |
  | `raw_tokens` on the desktop side | 1 | 5 (7 gate hits) | `apps/desktop/src-tauri/src/race_trait_picker.rs` — real `PREVAREQ`/`PREMULT`/`PREABILITY`/`!PREFACT` parsing that decides alternate-trait exclusion. Needs `SheetRule.applies` to carry the exclusion-guard relation. |
  | unframed prose citation (no `.lst:<line>` marker, no enclosing paren) | ~8 | ~50 | A token named mid-sentence with no bounded frame. Three rules for these have now been built, measured against the real files and **removed** across cycles 5 and 6 for producing ungrammatical sheet prose. Hand work. |

- **Discoveries:** **one, and it explains why four cycles walked past a
  48-hit defect in the largest live file.**
  `tests/v06_caster_level_every_casting_class.rs::the_record_cites_its_corpus_source_and_disclaims_the_spell_math_it_does_not_compute`
  **asserted that the rendered caster-level sheet line must contain
  `BONUS:CASTERLEVEL`** — a test that *required* ingest vocabulary on a player's
  paper character sheet, and had done since v0.6 slice 1. Every casting
  character's sheet printed the class's `BONUS:CASTERLEVEL` token and the whole
  `BONUS:VAR` chain it names, in full. This is the same defect cycles 4 and 5
  cleared in `ComputationExplanation.detail` elsewhere, and it survived them
  because a green test stood over it: a residue hit protected by a passing
  assertion does not look like a defect to a cycle reading the suite. The
  assertion is inverted — the sheet keeps the `cr_classes.lst:281` **source
  citation** (a reference a reader can chase, not ingest syntax; the gate does
  not count it) and must now carry no `BONUS:`/`PRECLASS:`/`DEFINE`/`SPELLSTAT:`
  vocabulary. Emitted as a `correction` retro event
  (`1789159880605-at-35-e6-003-sweep-30d7b0`). **The lesson generalises:** the
  remaining residue should be read against the tests that pin it, not only
  against the gate.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=69 live_hits=665` (end) | all source files under the five live roots, comment lines excluded | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=69 live_hits=798` (start) | same, at `1b799159de` | same command, at that tree |
  | `133 code hits cleared`, `0` files risen | of the 798 the gate counted at cycle start, over the 69 files carrying them | `798 - 665`; per-file movement from `python3 /tmp/census6.py`-shaped scan (the loop is `scan()`'s own, reproduced in the sibling census tool's `_iter_live_source_files` usage) against `git show 1b799159de:<file>` |
  | `hits_inside_cfg_test=360` unchanged, `hits_outside` 438 → 305 | the 69 live files carrying hits | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` (unchanged tool, run at both trees) |
  | `80 → 5` hits in `ultimate_magic/feat_tables.rs` | that file's own code-hit count, by the gate's own regexes, comment lines excluded | the per-file scan above at each tree |
  | `116 → 58` hits in `pilot_compute/mod.rs` | same | same |
  | `144 records, 43 carrying effect tokens` | the live UM feat table | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle6_relocate_um_effect_tokens.py --check` |
  | `17` caster-level rows demoted | `CASTER_LEVEL_RULES`' own rows | `python3 …_cycle6_caster_level_provenance.py --check` → `rows=17` |
  | `10` hits cut by the cycle-6 prose frame, `7` blocks | all 69 live files | `python3 …_cycle6_prose_citation_demote.py $(cat <the 69 files>)` — idempotent, re-running at HEAD prints `TOTAL cleared=0` |
  | `0` other readers of `UmFeatEntry.effect` | all of `src`, `apps/desktop/src-tauri/src`, `tests` | `grep -rn "\.effect\b" src apps/desktop/src-tauri/src tests --include=*.rs \| grep -v "effect: " \| grep -v effect_text \| grep -v "\.effect\.spell_id"` — every surviving hit is `crb::feats::FeatTableEntry.effect`, a typed `FeatEffectBonus` slice |
  | `0` files in `data/sheet_rules/` carrying ingest vocabulary | all of `data/sheet_rules/` | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` → 0 |
  | `rust_lines_changed`, `pcgen_live_files=69` | the cycle's own diff | `python3 scripts/cycle_scope_gate.py --receipt --since 1b799159de…` |

- **Build scope verified:** run **once**, at the end, after the last
  figure-moving edit.
  - `cargo build --locked --lib -j 6` → clean, 0 warnings.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → `3,316 passed; 0 failed; 15 ignored`.
  - `cargo test --locked --no-fail-fast -j 6` → **414 targets, 8,830 passed, 0 failed, 68 ignored, 0 `test result: FAILED` lines**, `FULL_EXIT=0`.
  - `cargo clippy --locked --tests -j 6` → **0 warnings, 0 errors** (`grep -cE '^warning' -> 0`), `CLIPPY_EXIT=0`.
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, not risen
    (fell by 133).
  - `cargo run --locked --bin sheet_rule_convert -- --check` → `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS (115.7s)` — identical to cycles 3, 4 and 5 on every field; the 142 refusals are all `no_corpus_record`
    — this is the standing proof that no converted rule moved.
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
    'docs/release/SD-35-…/artifacts/**/*.md'` → `files_checked=114 violations=0`.
  - `python3 scripts/denominator_gate.py --check-provenance` → `files_checked=231
    figures_examined=573 violations=0`.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (`passed: 1 pi-sweep`).
  - **Desktop crate and frontend: not run — epic cadence.** This cycle touched
    **no** file under `apps/`; `git diff --stat 1b799159de..HEAD -- apps/` is
    empty.
  - `v06_work_inventory` and `corpus_literal_sweep`: **not run.** No corpus
    record and no classifier changed; `docs/work-inventory.json` is
    byte-identical to `/tmp/wi-before-AT-35-E6-003-SWEEP.json`
    (`regressed=0 added=0 dropped=0` in the receipt rows proves it).

- **RED→GREEN preserved, recorded:** both code-bearing changes were written
  test-first.
  1. The relocation: `src/pcgen_import/feat_effect_tokens.rs` was written with
     its real declared counts and an **empty** token table, and its tests failed
     for the intended reason before the generator filled it —
     ```
     ---- pcgen_import::feat_effect_tokens::tests::the_live_table_is_the_length_the_relocation_was_taken_from stdout ----
     assertion `left == right` failed
       left: 0
      right: 43
     test result: FAILED. 1 passed; 2 failed
     ```
  2. The caster-level demotion: the inverted assertion in
     `tests/v06_caster_level_every_casting_class.rs` was written **first** and
     failed against the old renderer, printing the defect in full —
     ```
     the rendered sheet line must carry no `BONUS:` ingest vocabulary: Wizard
     caster level at Wizard level 10: 10, transcribed from the corpus's own
     `BONUS:CASTERLEVEL|Wizard|Caster_Level_BL_Stripped_Wizard`
     (core_rulebook/cr_classes.lst:281). That token's variable resolves through
     cr_classes.lst:277 BONUS:VAR|Caster_Level_BL_Stripped_Wizard|…
     test result: FAILED. 7 passed; 1 failed
     ```
     — before the table and the three format strings were rewritten.

- **Suite result:** **414 targets, 8,830 passed, 0 failed, 68 ignored, 0 `test result: FAILED` lines**, `FULL_EXIT=0` — one run, at the final tree, started AFTER
  the last figure-moving edit.

- **Sweep population:** N/A for `corpus_literal_sweep` — no corpus record
  changed.

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
  unchanged; `scripts/pcgen-oracle-pin.env` untouched.

- **Status:** **partial.** The criterion's population is not zero at HEAD: 665
  code hits in 69 files remain, named and summing above.

- **Open question for the operator — the `#[cfg(test)]` ruling, unanswered for a
  second cycle.** 360 of the 665 remaining hits (54%) and 29 of the 69 remaining
  files sit inside `#[cfg(test)]` modules of live files, and **no amount of code
  work moves them.** Cycle 5 asked for this ruling first in its next-cycle
  scope; this cycle could not obtain one and did not presume to make it. The
  parallel to ruling B14 is exact:

  * `tests/**` is already wholly exempt from the gate, for being test code;
  * a `#[cfg(test)]` module is compiled out of the shipping library, so it does
    not execute in anything a player runs — the same "it does not execute"
    reasoning B14 used for a comment;
  * the only difference between a counted hit and an uncounted one is **which
    file the assertion was written in**, which is a code-organisation accident,
    not a statement about live code.

  If the ruling is **YES** (a `#[cfg(test)]` module does not count), that is an
  **instrument correction that closes nothing** and must be recorded as one
  (`instrument-correction-is-not-closure`): it would clear 29 files and 360 hits
  from the gate while leaving the code exactly as finished as it is now.
  If the ruling is **NO**, the 29 files need their test modules moved to
  `tests/` — a 29-file mechanical refactor whose cost should be authorised
  explicitly rather than discovered. Recorded as a `deferral`
  (`1789159872638-at-35-e6-003-sweep-1991ec`, corrected by
  `1789159920798-at-35-e6-003-sweep-51ccdd` for its pre-run figures).

- **Notes:** Three.

  **A green test was hiding the defect.** The caster-level finding under
  **Discoveries** is the most transferable thing in this receipt. Four cycles
  read `pilot_compute/mod.rs`, the largest live file, and none of them looked at
  the 48 hits in `CASTER_LEVEL_RULES` — because a passing assertion sat on top
  of them saying the token *belonged* there. The gate can only say a hit exists;
  it cannot say whether the surrounding code thinks the hit is correct. Reading
  the remainder against its tests, not only against the gate, is what the next
  cycle should do first for the mechanisms it takes.

  **Why 133 and not 500.** 360 of the 665 remaining hits are behind an operator
  ruling this cycle could not obtain. Of the 305 that are not, 49 are the
  `pcgen_desc.rs` deletion (blocked on a catalog rewire), ~110 are the
  classification move cycle 5 proved destructive to grind, 34 are deliberately
  kept raw, and 21 are live selector data needing converter expansion. What was
  left that this cycle could do safely and corpus-wide, it did — all three
  mechanisms in full, in one pass, each with its own idempotent tool.

  **The relocation is a move, not a removal.** `decisions.md` §11 keeps the
  converter side for Starfinder. All 43 token rows are in
  `src/pcgen_import/feat_effect_tokens.rs`, byte for byte, and three tests assert
  the round trip: every relocated row still names its own live record by index
  *and* key, the live table is still the length the relocation was taken from,
  and a mismatched key panics rather than silently returning another record's
  tokens. The same three properties cycle 3's `feat_prereq_tokens` move
  established, for the same reason.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus
  units by design)`; floor 500 code hits. Dispatch in this order:
  1. **The `#[cfg(test)]` ruling, before anything else** — 360 of 665 hits and
     29 of 69 files hang on it, and it is the only thing that can put this
     criterion within reach of its own floor. Two cycles have now asked.
  2. **`pcgen_desc.rs` (49) + `render_pcgen_desc` (39, overlapping)** — rewire
     `class_feature_pool_catalog.rs` to the sheet-rule package, then delete the
     live renderer. This is `AT-35-E6-003`'s own Evidence sentence.
  3. **The feat-qualifier classification moves to the converter** (~110) — move
     `bonus_is_conditioned`'s decision onto the converted record, re-derive the
     133/5/49 split and the 24-of-49 ledger from the converted side, and only
     then drop the `PRE`/`TYPE=` elements from the live arrays.
  4. **The desktop `raw_tokens` reader (5)** — `SheetRule.applies` must carry the
     exclusion-guard relation; the desktop crate and frontend suites run with it.

  Still open and **not** an Epic 6 item, carried forward from cycles 3, 4 and 5:
  the equipmods 658-vs-676 corpus drift needs a named owner in the equipment
  lane.
