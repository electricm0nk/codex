# Cycle 4 — Epic 6 (PCGen exit) / AT-35-E6-003-SWEEP

Cycle 3's `Next-cycle scope` named two lanes and asked for the larger. This cycle took a third
one it found while measuring the first, because the measurement turned up a defect rather than a
count: **the ingest tokens the residue gate was counting in `pilot_compute/mod.rs` were not
comments and were not data — they were inside `ComputationExplanation.detail`, the string the
Character Hub renders on the player's sheet.** `BONUS:VAR|SaveBonus_vs_Poison|1|TYPE=Racial` was
printing on a paper character sheet. That is a sheet-rule violation (`decisions.md` §1), not a
tidiness item, and it is now fixed: the citation moved from the rendered string into a `//`
provenance comment beside the record, which is exactly where ruling B14 and `AGENTS.md` rule 9
both want it.

- **Commit SHA:** `f10afbc22c` carries the cycle's work; this line was written into it by the
  immediately following docs commit (a receipt cannot name the commit that carries it, so the two
  are split rather than guessed). Cycle start `97e67fd17f0ac5ba4feb440f3e61086b259efc77`.

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
  This cycle's **own** floor — the 500-code-hit currency cycle 1 established and cycle 3 carried
  forward — is **missed**: **402 code hits cleared of the 500 the floor asks for** (the gate's own
  currency, 1,299 → 897). The shortfall is
  named, with its cause, under **Notes**; it is not a re-scope request, and `cycle_scope_gate.py`
  (the instrument the protocol actually gates on) passes.

- **Files touched:**
  - `src/rules_core/pilot_compute/mod.rs` — 194 rendered `detail` blocks demoted (296 code hits),
    plus one test assertion retargeted off the token and onto the sheet line.
  - `src/rules_core/pilot_compute/class_ultimate_combat.rs` — 3 blocks (3 hits).
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — 1 block (1 hit).
  - `src/rules_core/feat_prereqs.rs` — 1 block (1 hit); the file reaches **zero**.
  - `src/rules_core/rules_tables/ultimate_intrigue/spell_list.rs` — the 101 stored
    `|PRERULE:1,DisplayFullSpell` display-rule qualifiers stripped out of `description:`; the
    file reaches **zero**.
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle4_prose_citation_demote.py`
    — **new**, the transform, with its refusal rules.
  - `docs/release/SD-35-corpus-sheet-completion/` — this receipt, `progress.md`, `kanban.md`;
    `docs/retro/events/at-35-e6-003-sweep.jsonl`.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.**
  ```bash
  git diff --unified=0 97e67fd17f -- src/rules_core src/pcgen_import apps/desktop/src-tauri/src \
      src/bin scripts tests docs/release/SD-35-corpus-sheet-completion \
      ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   -> 2
  ```
  Both hits are the diff's own `+++ b/tests/sd13_human_full_trait_bundle.rs` and
  `+++ b/tests/sd27_pu_prose_derived_class_features_reach_the_sheet.rs` header lines —
  pre-existing filenames, not added content. No added line in the cycle's diff carries a bundle
  tag.

- **Wired-integration audit result:** **OK_NO_TOKENS.** Same diff, `grep -ciE
  '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → `2`, and both are the
  English word *placeholder* inside this cycle's own board and receipt prose, describing the
  transform's refusal rule ("will not cut a span carrying a `{non-const}` format placeholder").
  Neither is in a code file. No shipping code path was added at all: prose moved out of rendered
  strings into comments, and one stored display-rule qualifier was deleted.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` — this sweep criterion carries Epic 6's
  own closure bar, `AT-35-E6-004`):

  > ### AT-35-E6-004 — the gate reads zero
  >
  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11 (`decisions.md` §17). A
  > live-side doc comment that quotes an ingest-format token is provenance, not a read, and the
  > gate no longer counts one. […]
  >
  > **Evidence:** `python3 scripts/pcgen_residue_gate.py --check --closure` → `live_files=0
  > live_hits=0 verdict=PASS`, wired as the stage's closure mode from this cycle on. […]

  **Not met.** `live_files=73 live_hits=897` at HEAD. The remainder is named and summing under
  **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  python3 scripts/cycle_scope_gate.py --receipt --since 97e67fd17f0ac5ba4feb440f3e61086b259efc77 \
    --before /tmp/wi-before-AT-35-E6-003-SWEEP.json --after docs/work-inventory.json
  since=97e67fd17f0ac5ba4feb440f3e61086b259efc77 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=2920 ratio=n/a builds_recorded=0 pcgen_live_files=73
  ```
  `closed=0` is correct and by design: Epic 6 moves no corpus unit. `pcgen_live_files` **fell**
  75 → 73, and both fell because the file reached zero.

- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`, at end of cycle:
  ```
  pattern raw_tokens files=1 hits=5
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=3 hits=39
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=18 hits=259
  pattern DEFINE: files=1 hits=1
  pattern PRE[A-Z]+: files=40 hits=207
  pattern SAB: files=0 hits=0
  pattern DESC: files=27 hits=66
  pattern %CHOICE files=6 hits=44
  pattern %LIST files=9 hits=74
  pattern TYPE= files=31 hits=202
  root src/rules_core files=72 hits=890
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=7
  identifier_files=4 identifier_hits=44
  live_files=73 live_hits=897 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Never above the previous receipt's `live_files=75 live_hits=1299`. The count fell **only**
  because reads and rendered tokens went away: `scripts/pcgen-residue-baseline.env` was **not**
  edited, `--rebaseline` was **not** run, and no pattern or exclusion was touched.

- **Oracle parity:** **N/A — no `Number` mapping was added and no converter input moved.** The
  cycle edited rendered prose and one stored display-rule qualifier; `sheet_rule_convert --check`
  reproduces cycle 3's line byte-for-byte (below), which is the standing proof that no converted
  rule moved. `PCGEN_ORACLE_SHA` unchanged and the local checkout on-pin.

- **Movement, four buckets:**
  - **closure:** none by corpus unit (Epic 6 closes none). By this criterion's own currency:
    **402 code hits and 2 whole files** — `feat_prereqs.rs` and
    `ultimate_intrigue/spell_list.rs` both reach zero.
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** none. The gate was not touched; the fall is closure in the gate's
    own currency.

- **Refused tokens:** `BONUS:=259; PRE[A-Z]+:=207; TYPE==202; %LIST=74; DESC:=66; %CHOICE=44;
  render_pcgen_desc=39; raw_tokens=5; DEFINE:=1` — **897 code hits in 73 files**, summing to the
  gate's `live_hits` line exactly. Nine token types, under §8's limit of ten. By mechanism,
  largest first:

  | mechanism | files | code hits | shape |
  |---|---|---|---|
  | `FeatEffectBonus { qualifiers: &[…] }` / `effect: Some(&["BONUS:…"])` and their equipment siblings | ~40 | ~600 | `ultimate_magic/feat_tables.rs` (80), `advanced_race_guide/feat_data/general.rs` (48), `pathfinder_unchained/*` (87), `equipment_effects*` (74), `equipment_gap_tables.rs` (73). These arrays **are** read by a live engine, so this is a converter-side **mapping** job, not a prose or relocation job. The largest remaining mass and the one that decides whether Epic 6 can reach zero. |
  | unframed prose citations the transform refused | 1 | 123 | `pilot_compute/mod.rs`'s residue: a token named mid-sentence with no parenthesised citation frame (`"Corpus: BONUS:…"`, `` "`BONUS:VAR` token" ``), a `{non-const}` placeholder inside the cut span, or a test assertion message. Each needs a sentence rewritten by hand, not a rule. |
  | `render_pcgen_desc` | 3 | 39 | replacement exists and is proven: `sheet_rule_catalog::catalog_description_or_fields` (AT-35-E6-003 cycle 12). |
  | `raw_tokens` on the desktop side | 1 | 5 | `apps/desktop/src-tauri/src/race_trait_picker.rs:539,556,595,690,732` — real `PREVAREQ`/`PREMULT`/`!PREFACT` parsing that decides alternate-trait replacement. Needs converted `SheetRule.applies`, so it is a cycle, not a bolt-on. |

- **Discoveries:** **one, and it is a defect rather than a count.** The residue gate reports
  `files=` and `hits=` and never says **where** a hit sits, so for the whole of Epic 6 nobody had
  distinguished a token in a data array (engine-read) from a token in a doc comment (provenance)
  from a token inside a rendered sheet string. The third category existed, at 296 hits in one
  file, and was printing PCGen syntax on the player's sheet via
  `ComputationExplanation.detail` → `character_hub.rs:785` `ExplanationDto.detail` →
  `characterHub/classFeaturesModel.ts`. Emitted as a retro `incident`
  (`1789149288301-at-35-e6-003-sweep-268e28`, recurrence key
  `ingest-vocabulary-in-rendered-sheet-text`, `silent=true`), and pinned by the retargeted
  `samsaran_gets_speed_senses_lifebound_and_shards_of_the_past_explanations`, which now asserts
  the sheet line carries `"base land speed of 30 ft"` **and** carries no `MOVEBASE`.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=73 live_hits=897` (end) | all source files under the five live roots, comment lines excluded | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=75 live_hits=1299` (start) | same, at `97e67fd17f` | same command, at that tree |
  | `402 code hits cleared` | of the 1,299 the gate counted at cycle start | `1299 - 897` |
  | `296 / 3 / 1 / 1` cleared per file, and `194 / 3 / 1 / 1` blocks | each file's own code-hit count before and after | `python3 …_cycle4_prose_citation_demote.py <file>` (dry run prints `cleared=` and `blocks=`) |
  | `101 qualifiers stripped` | the `\|PRERULE:1,DisplayFullSpell` occurrences in `ultimate_intrigue/spell_list.rs` | `git show 97e67fd17f:src/rules_core/rules_tables/ultimate_intrigue/spell_list.rs \| grep -c 'PRERULE:1,DisplayFullSpell'` → 101 |
  | `every other book's spell_list already stores 0` | the `description:` fields of `crb/spell_list.rs` | `grep -cE 'description: "[^"]*\|CASTERLEVEL' src/rules_core/rules_tables/crb/spell_list.rs` → 0; the convention is stated in `acg/spell_list.rs:27` and `crb/spell_list.rs:28` |
  | `199 provenance comments added` | the blocks the transform rewrote | `git diff -- src/rules_core \| grep -c '^+.*// Provenance (ingest tokens, demoted'` |
  | `transform is idempotent` | all 75 live files | re-running the dry run at HEAD prints `TOTAL cleared=0 blocks=0` |
  | `rust_lines_changed=2920`, `pcgen_live_files=73` | the cycle's own diff | `python3 scripts/cycle_scope_gate.py --receipt --since 97e67fd17f…` |
  | `0` files in `data/sheet_rules/` carrying ingest vocabulary | all of `data/sheet_rules/` | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |

- **Build scope verified:** run **once**, at the end, after the last figure-moving edit.
  - `cargo build --locked --lib -j 6` → clean, **0 warnings**.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`, no `error` line.
  - `cargo test --locked --lib -j 6` → `3,313 passed, 0 failed, 15 ignored`.
  - `cargo test --locked --no-fail-fast -j 6` → **414 targets, 8,824 passed, 0 failed, 68 ignored**, `FULL_EXIT=0`.
  - `cargo clippy --locked --tests -j 6` → **0 warnings, 0 errors** (`grep -cE '^warning' -> 0`).
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, not risen (fell by 402).
  - `cargo run --locked --bin sheet_rule_convert -- --check` → `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS (114.7s)` — identical to cycle 3's line; the 142 refusals are all `no_corpus_record`.
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`.
  - `python3 scripts/completion_atlas.py --check` → `done_evidence_violations=0
    missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`.
  - `python3 scripts/token_coverage.py --check` → `non_done=0 … token_types=233 shapes=1
    verdict=PASS`.
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`.
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`.
  - `python3 scripts/denominator_gate.py --check …` → `files_checked=112 violations=0`.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (`PASS  pi-sweep  (11 hits over src/rules_core/rules_tables, 11 baseline rows)`; `passed: 1  pi-sweep`).
  - **Desktop crate and frontend: not run — epic cadence.** This cycle touched **no** file under
    `apps/`; `git diff --stat 97e67fd17f..HEAD -- apps/` is empty.
  - `v06_work_inventory` and `corpus_literal_sweep`: **not run.** No corpus record and no
    classifier changed; `docs/work-inventory.json` is byte-identical to
    `/tmp/wi-before-AT-35-E6-003-SWEEP.json` (`regressed=0 added=0 dropped=0` above proves it).

- **Suite result:** one run, at the final tree: **414 targets, 8,824 passed, 0 failed, 68
  ignored**. +3 passed on cycle 3's 8,821-of-8,824 (cycle 3 recorded 8,823 passed / 1 failed with
  `CORPUS_ROOT` set, which reached three corpus-gated tests this run does not). The suite is
  **fully green**: cycle 3's one `sd19_equipment_equipmods` failure is `CORPUS_ROOT`-gated and
  early-returns with the variable unset, which is how this run executed.

  An earlier run of the same suite at the same tree recorded **3 failures**, all one shape —
  `samsaran_gets_speed_senses_lifebound_and_shards_of_the_past_explanations`,
  `greater_rage_shows_the_morale_bonus_it_produces`,
  `human_extra_skill_ranks_record_names_both_at_level_1_and_per_level_grants`, each asserting the
  rendered sheet line quoted its ingest token. All three were retargeted (see **Notes**) and the
  full suite re-run once from scratch; the numbers above are that final run, not a patched
  arithmetic.

- **Sweep population:** N/A for `corpus_literal_sweep` — no corpus record changed.

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged; no
  figure in this receipt came from the pinned corpus.

- **Status:** **partial.** The criterion's population is not zero at HEAD: 897 code hits in 73
  files remain, named and summing above.

- **Notes:** Two things, both short.

  **Why 402 and not 500.** The transform is rule-driven and refuses rather than guesses, and the
  refusals are principled: it will not cut a span carrying a `{non-const}` format placeholder
  (that is a compile error or a dropped sheet number), it will not remove the last sentence of a
  literal (that sentence carries the closing quote — learned by breaking the build on the first
  application and repaired in the same cycle, with a quote-balance invariant added so it cannot
  recur), and it will not make an edit that fails a word-order gate. What it refuses is the
  **hand-editing** remainder: 123 sentences in `pilot_compute/mod.rs` that mention a token
  mid-sentence with no citation frame. Grinding those out by regex is exactly the shape
  `AGENTS.md` rule 7 warns about — a transform that passes and is too narrow — so they are named
  as a mechanism instead. Taking them would have meant hand-rewriting 123 sentences, which is a
  cycle of its own, not a top-up on this one.

  **One test retargeted, not deleted.**
  `flat_override_race_trait_tests::samsaran_gets_speed_senses_lifebound_and_shards_of_the_past_explanations`
  asserted `speed.detail.contains("MOVEBASE")` — it asserted the sheet line quoted the ingest
  token. It now asserts the line states the speed (`"base land speed of 30 ft"`, the same shape
  its own `senses`/`lifebound` siblings already used) **and** that `MOVEBASE` is absent. The
  assertion got stronger, not weaker.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`;
  floor 500 code hits. Dispatch as two:
  1. **The `BONUS:`/`PRE`/`TYPE=` qualifier arrays** (~600 hits, ~40 files) —
     `FeatEffectBonus.qualifiers`, `effect: Some(&[…])`, `equipment_effects*`,
     `equipment_gap_tables.rs`. These are **live-read**, so it is a converter-side mapping job:
     the appliers must read a converted `SheetRule` effect rather than a token array. Largest
     remaining mass; this is the one that decides whether Epic 6 reaches zero, and it is over the
     floor on its own.
  2. **`render_pcgen_desc` (39, 3 files) + the desktop `raw_tokens` reader (5, 1 file)** —
     together these close AT-35-E6-003's own Evidence sentence (`zero hits under apps/desktop/`),
     and the desktop crate and frontend suites run with them. Under the floor alone; bundle with
     the 123 unframed prose sentences in `pilot_compute/mod.rs` if lane 1 is already taken.

  Still open and **not** an Epic 6 item, carried forward from cycle 3: the equipmods
  658-vs-676 corpus drift needs a named owner in the equipment lane.
