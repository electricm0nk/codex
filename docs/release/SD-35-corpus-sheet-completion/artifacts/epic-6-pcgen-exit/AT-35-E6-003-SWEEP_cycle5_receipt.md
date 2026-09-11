# Cycle 5 — Epic 6 (PCGen exit) / AT-35-E6-003-SWEEP

Cycle 4 found ingest tokens printing on the player's sheet through
`ComputationExplanation.detail` and fixed 402 of them. This cycle found the same
defect in a second place — **the shipped equipment and archetype tables** — where
`%CHOICE`, `%LIST` and `|PREVARLT:FighterLVL,9` were reaching the sheet inside
`description:` strings that are served verbatim, with no render pass anywhere
between the table and the page. `equipment_gap_tables.rs` alone carried 73 such
hits across its 1,973 shipped rows, and the cause was one line in its generator:
`safe_description` rendered each description **only to decide whether to keep
it**, then stored the raw one.

- **Commit SHA:** `6fe6131922` carries the cycle's work; this line is written into
  it by the immediately following docs commit (a receipt cannot name the commit
  that carries it). Cycle start `b069f01962667a3cf35fc731fb3460e2b7bb0e0e`.

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
  This cycle's **own** floor — the 500-code-hit currency cycle 1 established —
  is **missed**: **99 code hits cleared of the 500 the floor asks for** (the
  gate's own currency, 897 → 798). The scope taken was the **whole remainder**
  (897 hits, all of it), which is what the floor rule requires of a cycle that
  cannot reach 500 any other way; what was not reached is the *clearing* target,
  and the shortfall is named with its cause under **Notes** and **Refused
  tokens**.

- **Files touched:**
  - `src/bin/gen_equipment_gap_tables.rs` — `safe_description` now ships the
    RENDERED text plus a new `trim_dangling_connective`; three tests
    (one retargeted, two new).
  - `src/rules_core/rules_tables/equipment_gap_tables.rs` — **regenerated**
    (73 hits → 34). Row count unchanged at 1,973. It does **not** reach zero, and
    deliberately so: 34 rows keep a raw `%` that stands in for a number the
    table cannot supply (see `carries_an_unresolved_magnitude`).
  - `src/rules_core/rules_tables/acg/archetype_tables.rs`,
    `acg/equipment_data/equipmods.rs`, `acg/feat_data/combat.rs`,
    `ultimate_combat/archetype_tables.rs`,
    `ultimate_equipment/equipment_tables.rs`,
    `ultimate_magic/archetype_tables.rs`,
    `ultimate_psionics/equipment_tables.rs`,
    `ultimate_wilderness/archetype_tables.rs` — 24 hand-authored
    `description:` literals rendered (42 hits).
  - `src/rules_core/rules_tables/crb/race_tables.rs` — 5 `detail:` rows, 17 hits.
  - `src/rules_core/pilot_compute/mod.rs` — 7 hits (123 → 116), inline `.lst:<line>`
    citations demoted to provenance comments.
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_prose_citation_demote.py`
    — **new**, cycle 4's tool plus one frame and single-line prose blocks.
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_table_description_render.py`
    — **new**, the table-literal renderer, with its refusal rules.
  - `docs/release/SD-35-corpus-sheet-completion/` — this receipt, `progress.md`,
    `kanban.md`; `docs/retro/events/at-35-e6-003-sweep.jsonl`.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.**
  Run on the final committed diff (`workflow-instruction.md §6` step 4):
  ```bash
  git diff --unified=0 b069f01962..HEAD -- src/rules_core src/bin apps/desktop/src-tauri/src \
      docs/release/SD-35-corpus-sheet-completion ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   -> 0
  ```

- **Wired-integration audit result:** **OK_NO_TOKENS** for shipping code. Same
  diff (`b069f01962..HEAD`), `grep -cE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → `4`, and all four are the English word *placeholder* inside the two Python
  analysis tools' docstrings, describing the refusal rule ("a sentence carrying a
  `{...}` format placeholder is KEPT"). Neither tool is shipping code; both live
  under `docs/release/`. The same four-of-a-kind disposition cycle 4 recorded.
  No Rust line added by this cycle carries any of the tokens.

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

  **Not met.** `live_files=69 live_hits=798` at HEAD. The remainder is named and
  summing under **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  python3 scripts/cycle_scope_gate.py --receipt --since b069f01962667a3cf35fc731fb3460e2b7bb0e0e \
    --before /tmp/wi-before-AT-35-E6-003-SWEEP.json --after docs/work-inventory.json
  since=b069f01962667a3cf35fc731fb3460e2b7bb0e0e target_dir=/tmp/cargo-sd35-AT-35-E6-003-SWEEP residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=371 ratio=n/a builds_recorded=3 pcgen_live_files=69
  ```
  `closed=0` is correct and by design: Epic 6 moves no corpus unit.
  `pcgen_live_files` **fell** 73 → 69; four files reached zero and seven more fell without reaching it; **none rose**.

- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`, at end of cycle:
  ```
  pattern raw_tokens files=1 hits=5
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=3 hits=39
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=17 hits=243
  pattern DEFINE: files=1 hits=1
  pattern PRE[A-Z]+: files=35 hits=163
  pattern SAB: files=0 hits=0
  pattern DESC: files=27 hits=66
  pattern %CHOICE files=3 hits=13
  pattern %LIST files=9 hits=74
  pattern TYPE= files=30 hits=194
  root src/rules_core files=68 hits=791
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=7
  identifier_files=4 identifier_hits=44
  live_files=69 live_hits=798 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Never above the previous receipt's `live_files=73 live_hits=897`. The count
  fell **only** because rendered tokens went away: `scripts/pcgen-residue-baseline.env`
  was **not** edited, `--rebaseline` was **not** run, and no pattern, root or
  exclusion was touched.

- **Oracle parity:** **N/A — no `Number` mapping was added and no converter input
  moved.** The cycle changed what a generator *stores* (rendered rather than raw
  prose) and rewrote rendered prose literals; no `data/corpus/` record and no
  converted rule moved, which `sheet_rule_convert --check` reproducing cycle 4's
  line byte-for-byte is the standing proof of. `PCGEN_ORACLE_SHA` unchanged and
  the local checkout on-pin (`7f818006e3…`, confirmed by
  `git -C $HOME/workspace/repos/pcgen rev-parse HEAD`).

- **Movement, four buckets:**
  - **closure:** none by corpus unit (Epic 6 closes none). By this criterion's
    own currency: **99 code hits and 4 whole files** —
    `acg/equipment_data/equipmods.rs`, `crb/race_tables.rs`,
    `ultimate_equipment/equipment_tables.rs` and
    `ultimate_psionics/equipment_tables.rs` all reach zero; seven more files fell
    without reaching it, and **no file rose**.
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** none. The gate was not touched.

- **Refused tokens:** `BONUS:=243; TYPE==194; PRE[A-Z]+:=163; %LIST=74; DESC:=66;
  render_pcgen_desc=39; %CHOICE=13; raw_tokens=5; DEFINE:=1` — **798 code hits in
  69 files**, summing to the gate's `live_hits` line exactly. Nine token types,
  under `workflow-instruction.md` §8's limit of ten. By mechanism, largest first:

  | mechanism | files | code hits | shape |
  |---|---|---|---|
  | hits inside a `#[cfg(test)]` module of a live file | 29 files are test-only | **360** | The residue gate scans whole files, so an assertion written inside a live file counts while the identical assertion in `tests/` does not (all of `tests/**` is exempt for being test code). 29 of the 69 remaining files carry **no** hit outside a test module, and only 40 carry one. This is the same shape as ruling B14 and needs the same thing — an **operator ruling**, not a cycle. Re-derive with the census command below. |
  | `FeatEffectBonus { qualifiers: &[…] }` PRE/`TYPE=` elements | ~10 | ~110 | The mapping job, and **not** what cycle 4's receipt said it was — see **Discoveries**. |
  | `render_pcgen_desc` | 3 | 39 | Replacement exists and is proven: `sheet_rule_catalog::catalog_description_or_fields`. Blocked on `class_feature_pool_catalog.rs` reading the sheet-rule package instead of the ingest cache. |
  | `raw_tokens` on the desktop side | 1 | 5 (7 gate hits) | `apps/desktop/src-tauri/src/race_trait_picker.rs:539,545,556,595,690,732,735` — real `PREVAREQ`/`PREMULT`/`PREABILITY`/`!PREFACT` parsing that decides alternate-trait exclusion. Needs `SheetRule.applies` to carry the exclusion-guard relation. |
  | unframed prose citation (no `.lst:<line>` marker) | ~8 | ~100 | A token named mid-sentence with no source marker. Two rules for these were built, measured against the real file and **removed** for producing ungrammatical sheet prose — see the tool's docstring. Hand work. |

- **Discoveries:** **one, and it corrects the previous receipt.** Cycle 4 named
  the `FeatEffectBonus.qualifiers` arrays "read by a live engine, so this is a
  converter-side **mapping** job". They are not read by a live engine. The only
  live consumer is `damage_total.rs::constant_damage_bonus`, which returns `None`
  unless `qualifiers.len() == 3` — it **rejects** every array carrying a `TYPE=`
  or `PRE` qualifier. The real readers are **tests**:
  `tests/sd27_arg_and_pu_feat_effects.rs::bonus_is_conditioned` decides
  conditioned-vs-unconditional by `q.starts_with("PRE")`, and that decision
  produces the published 133/5/49 ARG split and the 24-of-49 ledger. So the job
  is not "map the token to a converted effect" — it is "move that
  **classification** to the converter first", and a cycle that deleted the
  qualifiers by regex would silently destroy a derived count while the gate
  applauded. Emitted as a `correction` retro event
  (`1789155420495-at-35-e6-003-sweep-b1f138`).

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=69 live_hits=798` (end) | all source files under the five live roots, comment lines excluded | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=73 live_hits=897` (start) | same, at `b069f01962` | same command, at that tree |
  | `99 code hits cleared`, `4` files to zero, `0` files risen | of the 897 the gate counted at cycle start, over the 73 files carrying them | `897 - 798`; the per-file movement from the same scan run against `git show b069f01962:<file>` for every live-root source file — the exact loop is reproduced in the receipt's sibling census tool's `_iter_live_source_files` usage |
  | `73 hits` in `equipment_gap_tables.rs` before, `34` after | that file's own code-hit count, by the gate's own regexes, comment lines excluded | `git show b069f01962:src/rules_core/rules_tables/equipment_gap_tables.rs > /tmp/before.rs`, then for each of `/tmp/before.rs` and the working file: `python3 -c "import importlib.util,sys; spec=importlib.util.spec_from_file_location('g','scripts/pcgen_residue_gate.py'); g=importlib.util.module_from_spec(spec); spec.loader.exec_module(g); t=g.code_only(open(sys.argv[1]).read()); print(sum(len(rx.findall(t)) for rx in g._COMPILED.values()))" <file>` → 73, then 34. **A line count is not a hit count** — `grep -c` on the same file prints 70, because three lines carry two hits each. |
  | `1,973 rows` unchanged | `EquipmentGapRow` literals in that file | `grep -c '    EquipmentGapRow {' src/rules_core/rules_tables/equipment_gap_tables.rs` → 1973 (the leading indent excludes the `pub struct` line, which an unindented grep counts, giving 1974) |
  | `0` shipped gap descriptions end on a dangling connective | all `description: Some("…")` in that file | `grep -oE 'description: Some\("[^"]*"\)' src/rules_core/rules_tables/equipment_gap_tables.rs \| sed 's/description: Some("//;s/")$//' \| grep -cE '\b(of\|to\|against\|with\|by\|for\|in\|on\|from\|than\|and\|or)$'` → 0 |
  | `24 rendered, 5 refused` hand-authored table literals | every `description: Some("…")` under `src/rules_core/rules_tables/` carrying ingest vocabulary | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_table_description_render.py $(grep -rl 'description: Some("' src/rules_core/rules_tables/ --include=*.rs)` (dry run; prints `TOTAL rendered= refused=`) |
  | `7 hits` demoted in `pilot_compute/mod.rs` | that file's continuation and single-line prose blocks | `python3 …_cycle5_prose_citation_demote.py src/rules_core/pilot_compute/mod.rs` (dry run prints `cleared=`) |
  | `17 hits` demoted in `crb/race_tables.rs` | that file's `detail:` rows | same tool, that file |
  | both tools are idempotent | all 69 remaining live files | re-running either dry run at HEAD prints `TOTAL cleared=0` / `TOTAL rendered=0` |
  | `hits_inside_cfg_test=360 hits_outside=438 files_test_only=29 files_with_non_test_hits=40` | the 69 remaining live files | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` |
  | `rust_lines_changed=371`, `pcgen_live_files=69` | the cycle's own diff | `python3 scripts/cycle_scope_gate.py --receipt --since b069f01962…` |
  | `0` files in `data/sheet_rules/` carrying ingest vocabulary | all of `data/sheet_rules/` | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` → 0 |

- **Build scope verified:** run **once**, at the end, after the last
  figure-moving edit.
  - `cargo build --locked --lib -j 6` → clean.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → `3,313 passed; 0 failed; 15 ignored`.
  - `cargo test --locked --no-fail-fast -j 6` → **414 targets, 8,827 passed, 0 failed, 68 ignored, 0 `test result: FAILED` lines**, `FULL_EXIT=0`.
  - `cargo clippy --locked --tests -j 6` → **0 warnings, 0 errors** (`grep -cE '^warning' -> 0`), `CLIPPY_EXIT=0`.
  - `cargo test --locked --bin gen_equipment_gap_tables -j 6` → `27 passed; 0 failed`
    (the RED→GREEN pin for this cycle's generator change; RED first, recorded
    below).
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, not risen
    (fell by 99).
  - `cargo run --locked --bin sheet_rule_convert -- --check` → `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS (115.0s)` — identical to cycles 3 and 4 on every field; the 142 refusals are all `no_corpus_record`. This is the standing proof that no converted rule moved.
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`.
  - `python3 scripts/completion_atlas.py --check` → `done_evidence_violations=0
    missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`.
  - `python3 scripts/token_coverage.py --check` → `non_done=0 … token_types=233
    shapes=1 verdict=PASS`.
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`.
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`.
  - `python3 scripts/denominator_gate.py --check …` → `files_checked=113 violations=0`.
  - `python3 scripts/denominator_gate.py --check-provenance` → `files_checked=230
    figures_examined=572 violations=0`.
  - `./scripts/publish-site-dashboard.sh --check-pin` → pin matches.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (`passed: 1 pi-sweep`).
  - **Desktop crate and frontend: not run — epic cadence.** This cycle touched
    **no** file under `apps/`; `git diff --stat b069f01962..HEAD -- apps/` is empty.
  - `v06_work_inventory` and `corpus_literal_sweep`: **not run.** No corpus
    record and no classifier changed; `docs/work-inventory.json` is
    byte-identical to `/tmp/wi-before-AT-35-E6-003-SWEEP.json`
    (`regressed=0 added=0 dropped=0` above proves it).

- **RED→GREEN preserved, recorded:** the generator change was written test-first.
  `a_bare_choice_keyword_with_no_pipe_tail_ships_rendered_not_raw` and
  `a_pipe_argument_tail_never_reaches_the_shipped_row` were written against the
  old `safe_description` and failed for the intended reason —
  ```
  left: Some("Enhancement bonus to ability %CHOICE")
  right: Some("Enhancement bonus to ability")
  shipped row still carries ingest vocabulary: "Cast % at will|%LIST"
  test result: FAILED. 3 passed; 2 failed
  ```
  — before `Some(description)` became `Some(trim_dangling_connective(&rendered.text))`.

- **Suite result:** one run, at the final committed tree (`6fe6131922`), started AFTER the last figure-moving edit: **414 targets, 8,827 passed, 0 failed, 68 ignored**. +3 passed on cycle 4's 8,824, which is this cycle's three new `safe_description` tests. An earlier run of the same suite was started against an intermediate tree and **discarded** when the generator's refusal rule changed the table again — a stale green is not a verification (`workflow-instruction.md §6` step 3: once, after the last figure-moving commit).

- **Sweep population:** N/A for `corpus_literal_sweep` — no corpus record changed.

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
  unchanged and matched by the local checkout. The regeneration of
  `equipment_gap_tables.rs` read that pinned corpus via
  `PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data`.

- **Status:** **partial.** The criterion's population is not zero at HEAD: 798
  code hits in 69 files remain, named and summing above.

- **Notes:** Three things.

  **The table stops short of zero on purpose, and that is the most important
  line in this receipt.** The first version of the generator fix rendered every
  description, and it reached `0` hits — a clean-looking number. Reading the
  diff showed what it had bought: `+%d10 additional fire damage` had become
  `d10 additional fire damage`, and `Darkvision % ft.` had become `Darkvision
  ft.` — 55 of 1,014 descriptions in which a `%` standing in for a NUMBER was
  rendered away, leaving prose that reads as a complete rule with the magnitude
  silently gone. `d10` looks like valid dice notation; `%d10` does not. The
  sheet rule asks for *dice in final form*, and a plausible wrong number is the
  failure `AGENTS.md` rule 7 exists to name, so `safe_description` now keeps the
  RAW text whenever a `%` stands in for a number the row cannot supply
  (`carries_an_unresolved_magnitude`, pinned by
  `a_percent_standing_in_for_a_number_is_left_raw_not_rendered`). A `%%`
  escaped literal and a `%CHOICE`/`%LIST` keyword still render — those stand in
  for a choice the sheet names elsewhere, not a magnitude. **The cost is 34 hits
  this cycle could have reported as cleared and did not**, and the gate counts
  every one of them.

  **Why 99 and not 500.** Three of the five remaining mechanisms are not
  regex-shaped work at all. The largest, 360 hits, is a question about the
  instrument (does a `#[cfg(test)]` module inside a live file count?) that only
  the operator can answer, exactly as B14 was. The second, ~110 hits, turned out
  on measurement to be a *different* job from the one the previous receipt named
  (see **Discoveries**) and would destroy a published derived count if ground out
  by rule. The third, ~100 hits of unframed prose, defeated two rules this cycle
  wrote, measured and deleted. What was left that a rule could do safely, this
  cycle did — all of it, corpus-wide, in one pass, and both tools are idempotent
  at HEAD.

  **Two rules were built and then removed, deliberately.** The prose demoter's
  first draft also cut bare connective runs and backtick-quoted tokens. Both
  compiled, both passed every safety gate the tool has, and both produced
  ungrammatical sheet lines in the real file (*"the record carries no -- and
  cross-checked against"*). A sheet line that reads as a truncation is worse than
  the citation it replaced, so they were deleted rather than shipped, and the
  docstring records why. Likewise the table renderer's first `|PRE…` tail regex
  was greedy and swallowed the second variant of a two-variant `SPROP:`
  (Heartstaff, Crown of Chaos); a `;` boundary fixed it, and the comment records
  the failure so the next reader does not re-widen it.

  **One near-miss, caught before it shipped.** Allowing the prose demoter to look
  at single lines let it reach inside a live data array in
  `advanced_race_guide/feat_data/general.rs` and rewrite
  `count("ABILITIES","TYPE=FavoredClassBonus")` to `count("ABILITIES","")` — a
  silent change to what the engine reads — while emitting Rust `\`-continuations
  outside a string literal, which does not compile. It was caught by inspecting
  the proposed diff before applying it, and the control is now a whole-line shape
  test (`PROSE_LINE_RX`) that a data array can never match.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus
  units by design)`; floor 500 code hits. Dispatch in this order:
  1. **Escalate the `#[cfg(test)]` question to the operator before anything
     else** — 360 of the 798 remaining hits, 29 of the 69 remaining files, hang
     on it, and no amount of code work moves them. The parallel to B14 is exact:
     `tests/**` is already exempt for being test code, and a `#[cfg(test)]`
     module is compiled out of the shipping library. If the ruling is YES (test
     modules do not count), that is an **instrument correction that closes
     nothing** and must be recorded as one.
  2. **The feat-qualifier classification moves to the converter** (~110 hits).
     Move `bonus_is_conditioned`'s decision — is this bonus gated by its own
     inline `PRE*`? — onto the converted record, re-derive the 133/5/49 split
     and the 24-of-49 ledger from the converted side, and only then drop the
     `PRE`/`TYPE=` elements from the live arrays. Over the floor on its own once
     bundled with (3).
  3. **`render_pcgen_desc` (39) + the desktop `raw_tokens` reader (5)** —
     together these close AT-35-E6-003's own Evidence sentence (`zero hits under
     apps/desktop/`), and the desktop crate and frontend suites run with them.

  Still open and **not** an Epic 6 item, carried forward from cycles 3 and 4:
  the equipmods 658-vs-676 corpus drift needs a named owner in the equipment lane.
