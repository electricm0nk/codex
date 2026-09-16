# Cycle AT-35-E6-003-SWEEP cycle 11 — Epic 6 PCGen exit / AT-35-E6-003-SWEEP

- **Commit SHA:** `208ebf1e21`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by
  design; decisions.md §2, workflow-instruction.md §6 step 1)`

  The residue check, which is **not** exempt, ran first and passed at the cycle's
  start tree `e92ae64f5e`:
  ```
  live_files=56 live_hits=413 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

  **This dispatch's cycle number was wrong.** It said "CYCLE NUMBER FOR THIS
  CRITERION: 10"; cycle 10's receipt is already committed at `e92ae64f5e` and the
  refused-token line the dispatch handed on **is** cycle 10's result, summing to
  the 413 the gate reported at that tree:
  ```
  BONUS:=92 TYPE==107 PRE[A-Z]+:=66 DESC:=63 render_pcgen_desc=39 %LIST=28 %CHOICE=13 raw_tokens=5
  ```
  This is cycle 11. (`correction 1789179191019-at-35-e6-003-sweep-031187`.)

- **Files touched:**
  - `tests/sd35_rendered_prose_carries_no_ingest_vocabulary.rs` — the standing
    gate's `SCANNED` list extended by the three shipped content tables (RED first).
  - `src/rules_core/rules_tables/bestiary/monster_data.rs` — 20 description
    arguments.
  - `src/rules_core/rules_tables/bestiary_3/monster_data.rs` — 4 leaked `DESC:`
    markers in one description.
  - `src/rules_core/rules_tables/inner_sea_world_guide/monster_data.rs` — 2
    description arguments.
  - `src/pcgen_import/ingest_record.rs` — `rebuild_bonus_token`, plus its
    round-trip test.
  - `src/rules_core/corpus_loader.rs` — calls it instead of formatting the ingest
    token name on the live side.
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle11_description_argument_words.py`
    — the converter script, committed, with `--check` and `--apply`.
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    — `derived_at` stamp only, written by `completion_atlas.py --check`.
  - `docs/release/SD-35-corpus-sheet-completion/{progress.md,kanban.md}`, this
    receipt, `docs/retro/events/at-35-e6-003-sweep.jsonl`.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.**
  `scripts/identifier-discipline-audit.sh` at HEAD prints `OK_NO_BUNDLE_TAGS`.
  On this cycle's own diff,
  ```bash
  git diff --unified=0 e92ae64f5e -- src/ apps/ tests/ ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ```
  → three matches, all of them the diff's own `diff --git` / `---` / `+++`
  headers for `tests/sd35_rendered_prose_carries_no_ingest_vocabulary.rs`. No
  identifier in shipping code carries a bundle tag. Same documented
  citation-exclusion class cycles 9 and 10 recorded.

- **Wired-integration audit result:** **OK_NO_TOKENS.** Same diff,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → no match, before and after the work.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` — this sweep criterion
  carries Epic 6's closure bar, `AT-35-E6-004`):

  > ### AT-35-E6-004 — the gate reads zero
  >
  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11
  > (`decisions.md §17`). […]
  >
  > **Evidence:** `python3 scripts/pcgen_residue_gate.py --check --closure` →
  > `live_files=0 live_hits=0 verdict=PASS` […]

  **Not met.** `live_files=53 live_hits=386` at HEAD. The remainder is named and
  summing under **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  since=e92ae64f5e target_dir=/tmp/cargo-sd35-AT-35-E6-003-SWEEP residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=113 ratio=n/a builds_recorded=1 pcgen_live_files=53
  ```
  `closed=0` is correct and by design: Epic 6 moves no corpus unit, which is why
  the cycle claims the floor exemption. `pcgen_live_files` falls **56 → 53** —
  three files reached zero and none rose.

- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`, at the end
  of the cycle:
  ```
  pattern raw_tokens files=1 hits=5
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=3 hits=39
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=14 hits=91
  pattern DEFINE: files=0 hits=0
  pattern PRE[A-Z]+: files=20 hits=66
  pattern SAB: files=0 hits=0
  pattern DESC: files=26 hits=59
  pattern %CHOICE files=2 hits=8
  pattern %LIST files=5 hits=13
  pattern TYPE= files=22 hits=105
  root src/rules_core files=52 hits=379
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=7
  identifier_files=4 identifier_hits=44
  live_files=53 live_hits=386 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Never above the previous receipt's (`56` / `413`). Per-pattern deltas, all
  negative or flat: `%LIST` −15, `%CHOICE` −5, `DESC:` −4, `TYPE=` −2,
  `BONUS:` −1; everything else unmoved. They sum to the **27** hits this cycle
  removed.

- **Oracle parity:** **N/A.** No `Number` mapping was added and no computed value
  moved. The one live-side code change is a call-for-call relocation whose
  byte-equality is pinned by a new round-trip test
  (`rebuilding_a_bonus_chain_is_the_inverse_of_splitting_it`); the other 26 edits
  are to `description` / `description_variables` strings, which every consumer
  reads through `filter_map(parse_*)` — a string that did not parse as a formula
  before still does not parse as one, and the arrays' lengths are unchanged, so
  `derived_evaluator_fixture_check::monster_ability_save_dc`'s positional index
  reads the same slot it read before.

- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. Epic 6 closes no corpus unit.
  - **relabel (bucket to bucket):** none. `regressed=0 added=0 dropped=0`;
    `docs/work-inventory.json` is byte-identical to `e92ae64f5e`'s
    (`git diff --stat docs/work-inventory.json` empty).
  - **reachability:** 27 live-side PCGen hits removed, 3 live files to zero. The
    reachable (non-`#[cfg(test)]`) remainder falls 61 → 34 and the files carrying
    it 18 → 14.
  - **instrument-correction:** the gate test's `SCANNED` list grew from 5 files to
    8. That is a **widening**, not a weakening: it put three previously unwatched
    shipped tables under the standing gate and they went RED immediately (23
    lines / 26 hits) before a single byte was edited.

- **Refused tokens** — eight types, summing to **386**, which is the gate's own
  `live_hits`:
  ```
  TYPE==105, BONUS:=91, PRE[A-Z]+:=66, DESC:=59, render_pcgen_desc=39, %LIST=13, %CHOICE=8, raw_tokens=5
  ```

  Split by what can move without an operator ruling
  (`python3 .../AT-35-E6-003-SWEEP_cycle5_test_region_census.py`-shaped walk,
  re-run this cycle: `total_hits 386 in_test 352 non_test 34`):

  | where | hits | files |
  |---|---|---|
  | inside a `#[cfg(test)]` module in a live file | 352 | 39 |
  | reachable by code work | **34** | **14** |

  The reachable 34, by the job that owns each — every one a separate cycle:

  | job | hits | files |
  |---|---|---|
  | `FeatEffectBonus` selection-target typing (`"WEAPONPROF=%LIST"`, `var("SKILLRANK=%LIST")`, `count(…,"TYPE=FavoredClassBonus")`) | 12 | `crb/feat_data/{combat,general}.rs`, `acg/feat_data/combat.rs`, `advanced_race_guide/feat_data/general.rs` |
  | `race_trait_picker.rs` exclusion guards read off `raw_tokens` | 7 | `apps/desktop/src-tauri/src/race_trait_picker.rs` |
  | the `render_pcgen_desc` / `pcgen_desc.rs` catalog rewire | 4 | `class_feature_pool_catalog.rs`, `pcgen_desc.rs`, `pilot_compute/class_feature_grant_consumer.rs` |
  | the `PU_*_DESC_TOKEN` verbatim corpus transcriptions | 4 | `pilot_compute/mod.rs` |
  | `external_ability_refs` carrying `!PRETEMPLATE:` guard tails | 3 | `crb/companion_data.rs` |
  | `equipment_effects` comparing a qualifier to `TYPE=Circumstance` / `TYPE=Enhancement` | 2 | `equipment_effects/{arms_armor,equipmods}.rs` |
  | `CHOOSE:`/pool `TYPE=` prefix constants | 2 | `race_resolver.rs`, `skinwalker_change_shape.rs` |

  **Why none of the seven was folded into this cycle**, stated so the next one
  does not re-decide it:

  * The `FeatEffectBonus` job **moves a computed value**. `constant_damage_bonus`
    (`src/rules_core/damage_total.rs:746`) accepts a chain only at
    `qualifiers.len() == 3`; Weapon Specialization's
    `["WEAPONPROF=%LIST", "DAMAGE", "2"]` is such a chain and contributes `+2`
    today. Typing the selection target out of slot 0 shortens the chain and
    silently stops that contribution. It needs the matching rewrite **and** an
    oracle comparison in the same cycle — which is exactly what cycle 10's
    receipt named it as.
  * `race_trait_picker.rs` needs `SheetRule.applies` to carry the
    exclusion-guard relation before the picker can stop parsing `PREVAREQ:` /
    `PREMULT` / `!PREFACT` itself, and it is the only file under `apps/`, so it
    also carries the desktop-crate and frontend suites.
  * The `render_pcgen_desc` rewire is blocked on the converter-side prose
    carrier documented in
    `artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle5_converter-prose-blocker.md`.
  * The four `PU_*_DESC_TOKEN` constants are the book's own words, pinned
    byte-for-byte against the `.lst` files by
    `sd27_pu_class_feature_descriptions_carry_the_characters_numbers`. Cycle 10
    declined to reword them and this cycle declines for the same reason. They are
    **counted** in the remainder, never exempted.
  * The last three (`external_ability_refs`, `equipment_effects`,
    the two prefix constants) are each a live-side read of an ingest qualifier
    string. Moving the literal into a converter-side constant would delete the
    gate's hit **without moving the read**, which is the masking shape cycles 4
    and 5 were burned by. Each needs its field typed at the corpus boundary —
    real work, deliberately not started at the end of a cycle.

  **352 of the 386 cannot move by any code work.** That remains the single
  highest-value action for this criterion and it is not a cycle: it is the
  operator ruling on whether a `#[cfg(test)]` module inside a live file is live
  code, filed by cycle 10 under `progress.md`'s `## Open blockers`.

- **Discoveries:**
  1. **`&nl;` is ingest vocabulary the residue gate has never counted.** The
     gate's `TOKEN_SYNTAX_PATTERNS` names fourteen shapes; PCGen's newline entity
     escape is not one of them, and it is shipped verbatim on **140** non-comment
     lines under `src/rules_core/rules_tables/`. It reaches a player wherever the
     description is not routed through `pcgen_desc::render_pcgen_desc`, which
     decodes it. This cycle's mechanism-4 edit deliberately left it alone —
     dropping the leaked `DESC:` marker beside it and no more — because repairing
     it is a different mechanism over a different population and half-doing it
     here would have hidden the size of it.
     `correction 1789179200765-at-35-e6-003-sweep-7f5dfd`, with its own count
     corrected from an estimate by
     `correction 1789179208869-at-35-e6-003-sweep-3a59fa`.
  2. `description_variables` is prose, not plumbing. Every earlier cycle of this
     criterion scanned explanation strings and diagnostic messages; the arrays
     that supply the words `%1` is replaced with were never scanned, and they were
     carrying the substitution token itself. That is why the gate test's `SCANNED`
     list grew.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=53 live_hits=386` (end) | all source files under the five live roots, comment lines excluded | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=56 live_hits=413` (start, at `e92ae64f5e`) | same, at that tree | `git worktree add --detach <tmp> e92ae64f5e && python3 scripts/pcgen_residue_gate.py --check --root <tmp>` |
  | `27` hits removed, `3` files to zero, `0` risen | of the 413 the gate counted at cycle start, over the 56 files carrying them | `413 - 386`; the per-file set from `pcgen_residue_gate.scan()` run at both trees and differenced |
  | `total_hits 386 in_test 352 non_test 34`, files `53` / `14` | the live files carrying hits | the `#[cfg(test)]` brace-balance walk of `AT-35-E6-003-SWEEP_cycle5_test_region_census.py`, run at HEAD |
  | `26` table edits applied, in 4 mechanisms | every `description` / `description_variables` string in the three shipped tables | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle11_description_argument_words.py --check` → `would apply=26` |
  | `23` findings in the RED state, `0` in GREEN | the 8 scanned files | `cargo test --locked --test sd35_rendered_prose_carries_no_ingest_vocabulary` before the transform vs after |
  | `140` lines carrying `&nl;` | non-comment lines under `src/rules_core/rules_tables/` | `grep -rn '&nl;' src/rules_core/rules_tables --include=*.rs \| grep -v '^[^:]*:[0-9]*: *//' \| wc -l` → 140 |
  | `0` files in `data/sheet_rules/` carrying ingest vocabulary | all of `data/sheet_rules/` | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` → 0 |
  | `rust_lines_changed=113`, `pcgen_live_files=53`, `builds_recorded=1` | the cycle's own diff | `python3 scripts/cycle_scope_gate.py --receipt --since e92ae64f5e --before /tmp/wi-before-AT-35-E6-003-SWEEP.json --after docs/work-inventory.json` |
  | `pi-sweep` `11` hits over `11` baseline rows | `src/rules_core/rules_tables` | `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` |

- **Build scope verified:** run **once**, after the last figure-moving edit.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`, wall clock `2:55.63`,
    max RSS `2,478,752 kB`.
  - `cargo test --locked --no-fail-fast -j 6` → `**415 targets (414 test binaries + 1 doc-test target), 8,842 passed, 0 failed, 68 ignored, 0 `test result: FAILED` lines**, `FULL_EXIT=0` — +1 test over cycle 10's 8,841, exactly the one new round-trip test`.
  - `cargo test --locked --lib -j 6` → ``test result: ok. 3323 passed; 0 failed; 15 ignored` — +1 over cycle 10's 3,322, the same one new test` (read out of the
    same `--no-fail-fast` run's `Running unittests src/lib.rs` target rather than
    rebuilt, which is the same binary).
  - `cargo clippy --locked --tests -j 6` → 0 warnings, 0 errors.
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `CONVERT_CHECK_EXIT=0`, every kind's `converted`/`refused` split unmoved
    (`monster_ability: records=3806 converted=3806 refused=0`).
  - `python3 scripts/completion_atlas.py --check` → `done_evidence_violations=0
    missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`,
    `EXIT=0`.
  - `python3 scripts/token_coverage.py --check` → `non_done=0 tokened=0
    token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1
    verdict=PASS`, `EXIT=0`.
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, `EXIT=0`.
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, `EXIT=0`.
  - `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-…/*.md'
    'docs/release/SD-35-…/artifacts/**/*.md'` → `files_checked=119 violations=0`.
  - `python3 scripts/denominator_gate.py --check-provenance` → `files_checked=236
    figures_examined=574 violations=0`.
  - `./scripts/publish-site-dashboard.sh --check-pin` → input pin matches
    `docs/work-inventory.json`, `EXIT=0`.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`.
  - **Desktop crate and frontend: not run — epic cadence.** This cycle touched no
    file under `apps/`; `git diff --stat e92ae64f5e -- apps/` is empty. That is
    also why the 7 `race_trait_picker.rs` hits were left again.
  - `v06_work_inventory` and `corpus_literal_sweep`: **not run.** No corpus record
    and no classifier changed — `git diff --stat e92ae64f5e -- data/` is empty,
    and `docs/work-inventory.json` is byte-identical to `e92ae64f5e`'s. The sweep
    compares `data/corpus/**/*.json` against the pinned `.lst` files and reads no
    `rules_tables` file, so nothing this cycle edited is in its population.

- **Sweep population:** N/A — no corpus record changed.

- **Oracle pin:** `PCGEN_ORACLE_SHA` unchanged
  (`7f818006e371188e5717fd18d74d18a420747fc6`). No figure in this receipt came
  from the pinned corpus. The one provenance read of it — confirming that
  `b3_abilities_race.lst:869` states the Flail Snail ability as repeated `DESC:`
  tokens on one line — was taken from this repo's own committed corpus record
  `data/corpus/bestiary_3/monster_ability/flail_snail_warp_magic.json`, whose
  `raw_tokens[1]` holds the same bytes and which `corpus_literal_sweep` already
  holds byte-equal to that `.lst` line.

- **RED→GREEN preserved, recorded.** The gate's `SCANNED` list was extended and
  the test run against the **untouched** tables first:
  ```
  ---- rendered_prose_carries_no_pcgen_ingest_vocabulary ----
  23 line(s) of prose this engine writes still print PCGen ingest vocabulary on
  a player's sheet (sheet rule, decisions.md §1). …
  src/rules_core/rules_tables/bestiary/monster_data.rs:11275: `%CHOICE` … description_variables: &["%CHOICE"],
  src/rules_core/rules_tables/bestiary/monster_data.rs:16315: `%LIST` … description_variables: &["RegenerationRate", "%LIST"],
  src/rules_core/rules_tables/bestiary_3/monster_data.rs:9881: `DESC:` … "…consult the following table. DESC:&nl; 1-3 Spell misfires…"
  src/rules_core/rules_tables/inner_sea_world_guide/monster_data.rs:592: `TYPE=` … description_variables: &["10+(HD/2)+CON", "TYPE=Base"],
  test result: FAILED. 0 passed; 1 failed
  ```
  23 lines carrying 26 hits (the `bestiary_3` line carries four), and green after
  the transform. The 26 the test listed and the 26 of the 27 the residue gate
  lost are the same 26; the 27th is `corpus_loader.rs`'s `BONUS:`, pinned
  separately by `rebuilding_a_bonus_chain_is_the_inverse_of_splitting_it`.

- **Retro events emitted:** `correction 1789179191019-at-35-e6-003-sweep-031187`
  (the dispatch's cycle number), `correction
  1789179200765-at-35-e6-003-sweep-7f5dfd` (the `&nl;` discovery),
  `correction 1789179208869-at-35-e6-003-sweep-3a59fa` and `correction
  1789179242420-at-35-e6-003-sweep-01b92e` (two figures of this cycle's own,
  corrected against their commands), `deferral
  1789179222370-at-35-e6-003-sweep-800361` (the 386-hit remainder, mechanism by
  mechanism).

- **Status: `partial`.** The criterion's population is not zero at HEAD.

- **Notes:** the judgment call was declining all seven remaining reachable jobs
  rather than taking the two cheapest. Each of the cheap ones would have removed
  a gate hit by relocating a string constant while the live-side read it guards
  stayed exactly where it was. The rule is "nothing on the live side reads a
  PCGen token", not "no live file contains the characters `TYPE=`", and a cycle
  that closes the second while leaving the first is the masking shape cycles 4
  and 5 already paid for.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle)`; the whole remainder,
  **386 code hits in 53 files**, of which **34 hits in 14 files** are reachable
  without the `#[cfg(test)]` ruling. The largest single reachable job, and the
  one that should be taken next because it is the only one that moves a number a
  player reads, is the **`FeatEffectBonus` selection-target typing (12 hits)** —
  dispatched with the `damage_total::constant_damage_bonus` matching rewrite and
  an oracle comparison in the same cycle, never without them. The **operator
  ruling on `#[cfg(test)]` regions still stands between the gate and 352 of the
  386** and no amount of dispatching will move it.
