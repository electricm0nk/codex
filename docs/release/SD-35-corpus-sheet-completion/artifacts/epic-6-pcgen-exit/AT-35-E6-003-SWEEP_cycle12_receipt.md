# Cycle AT-35-E6-003-SWEEP cycle 12 — Epic 6 PCGen exit / AT-35-E6-003-SWEEP

- **Commit SHA:** `db0405eb18`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by
  design; decisions.md §2, workflow-instruction.md §6 step 1)`

  Both gates ran anyway. The corpus floor, for the record:
  ```
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the cycle's start
  tree `f8e4aa78e1`:
  ```
  live_files=53 live_hits=386 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

  **This dispatch's cycle number was wrong again.** It said "CYCLE NUMBER FOR
  THIS CRITERION: 11"; cycle 11's receipt is already committed at `f8e4aa78e1`
  and the refused-token line the dispatch handed on **is** cycle 11's result,
  summing to the 386 the gate reported at that tree. This is cycle 12.
  (`correction 1789180880544-at-35-e6-003-sweep-83042e`.) Cycle 11's receipt
  records the same off-by-one on its own dispatch, so this is the second
  consecutive occurrence, not a one-off — **a warning is not a control**
  (`AGENTS.md` rule 8): the orchestrator should derive the number from
  `ls artifacts/<epic>/<criterion>_cycle*_receipt.md` rather than carry it in
  prose.

- **Files touched:**
  - `src/rules_core/rules_tables/crb/feats.rs` — the `EffectSelection` enum
    (five variants, `sheet_words()`, `is_target()`) and the
    `FeatEffectBonus.selection` field.
  - `src/rules_core/rules_tables/advanced_race_guide/feats.rs` — the mirrored
    field; the enum is re-exported, not re-declared.
  - 11 shipped `feat_data/*.rs` files across `crb`, `apg`, `acg`,
    `advanced_race_guide` — all 290 `FeatEffectBonus` literals gain
    `selection:`; 12 of them lose their ingest marker slot.
  - `src/rules_core/damage_total.rs` — `constant_damage_bonus` rewritten for
    the shortened chain, plus its four `#[cfg(test)]` fixtures.
  - `src/pcgen_import/feat_effect_selections.rs` (**new**) — the verbatim
    pre-conversion chains and the slot each marker sat in, with the round-trip
    test. `src/pcgen_import/mod.rs` registers it.
  - `tests/sd35_rendered_prose_carries_no_ingest_vocabulary.rs` — the standing
    gate's `SCANNED` list extended by the four shipped feat tables (RED first).
  - `tests/sd19_feat_catalog.rs`, `tests/v06_apg_acg_feat_catalog.rs` — the
    struct-literal field, no assertion changed.
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle12_selection_typing.py`
    — the converter script, committed, with `--check` and `--apply`.
  - `…/AT-35-E6-003-SWEEP_cycle12_sheet-parity-after.json` — the oracle run.
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    — `derived_at` stamp only, written by `completion_atlas.py --check`.
  - `docs/retro/events/{at-35-e6-003-sweep,sd31-transcribe,root}.jsonl`,
    `docs/release/SD-35-corpus-sheet-completion/{progress.md,kanban.md}`, this receipt.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.** On this cycle's own diff,
  ```bash
  git diff --unified=0 f8e4aa78e1 -- src/rules_core src/pcgen_import \
    apps/desktop/src-tauri/src tests ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ```
  → six matches, all of them the diff's own `diff --git` / `---` / `+++`
  headers for the two pre-existing test files (`sd19_feat_catalog.rs`,
  `sd35_rendered_prose_carries_no_ingest_vocabulary.rs`). No identifier in
  shipping code carries a bundle tag. Same documented citation-exclusion class
  cycles 9–11 recorded.

- **Wired-integration audit result:** **OK_NO_TOKENS**, after a fix in this
  cycle. The first run of the same grep returned four matches on the word
  `placeholder`, all in new doc comments describing the *ingest format's*
  `%LIST` marker — not a stub, but a reader cannot tell that from the grep. The
  word was removed rather than rationalised: `stand-in` in the live doc
  comments, `marker` / `ingest_marker` in the converter module and its
  generator. The re-run is clean.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` — this sweep criterion
  carries Epic 6's closure bar, `AT-35-E6-004`):

  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11
  > (`decisions.md §17`). […] **Evidence:** `python3
  > scripts/pcgen_residue_gate.py --check --closure` → `live_files=0
  > live_hits=0 verdict=PASS`, wired as the stage's closure mode from this
  > cycle on. The oracle comparison at the end of the epic agrees with the one
  > at its start.

  **Not met.** `live_files=49 live_hits=373` at HEAD. The remainder is named and
  summing under **Refused tokens**. The oracle half of the Evidence sentence
  **is** met and is recorded below.

- **Receipt rows (mechanical):**
  ```
  since=f8e4aa78e17cfe42ff357cca2fac414ad6f2144f residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=801 ratio=n/a builds_recorded=0 pcgen_live_files=49
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
  pattern PRE[A-Z]+: files=20 hits=66
  pattern SAB: files=0 hits=0
  pattern DESC: files=26 hits=59
  pattern %CHOICE files=2 hits=8
  pattern %LIST files=1 hits=1
  pattern TYPE= files=21 hits=104
  live_files=49 live_hits=373 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Down from cycle 11's `live_files=53 live_hits=386`; never above it.

- **Oracle parity:** **run, and it did not move — byte-identical to cycle 11.**
  This is the cycle that had to run it: cycle 11's receipt named the
  `FeatEffectBonus` job as the one reachable job that **moves a computed value**
  and required "the matching rewrite **and** an oracle comparison in the same
  cycle, never without them".
  ```
  sheet_parity: lines compared=156 agree=154 disagree=2 unverifiable=67;
                chassis compared=382 agree=376 disagree=6 unverifiable=140;
                characters=29 exports_missing=0
  ```
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. Compared field by
  field against `AT-35-E6-003_cycle11_sheet-parity-after.json`, **`roster`,
  `summary`, `disagreements` and `results` are all identical** — not just the
  four summary counts but every per-line result. The eight named disagreements
  are the same eight, unchanged: the halfling and paladin save totals (6),
  `deterministic_human_fighter_l1 target:WeaponAttack:{"Chosen":
  "core_rulebook:feat:weapon_focus"} ours=0 oracle=1`, and
  `half_elf_fighter_l1 target:Pool:favored_class ours=1 oracle=2`. None is this
  cycle's, and that the figures did not move **is** the result: shortening
  Weapon Specialization's chain from three slots to two did not drop its `+2`.

- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. Epic 6 closes no corpus unit.
  - **relabel (bucket to bucket):** none. `regressed=0 added=0 dropped=0`;
    `docs/work-inventory.json` is byte-identical to `f8e4aa78e1`'s
    (`git diff --stat docs/work-inventory.json` empty).
  - **reachability:** 13 live-side PCGen hits removed, 4 live files to zero. The
    reachable (non-`#[cfg(test)]`) remainder falls **34 → 22** and the files
    carrying it **14 → 10**. `%LIST` falls 13 → 1.
  - **instrument-correction:** the gate test's `SCANNED` list grew from 8 files
    to 12. That is a **widening**, not a weakening: it put the four shipped feat
    tables under the standing gate and they went RED immediately (11 lines / 12
    hits) before a byte was edited.

- **Refused tokens** — eight types, summing to **373**, which is the gate's own
  `live_hits`:
  ```
  TYPE==104, BONUS:=91, PRE[A-Z]+:=66, DESC:=59, render_pcgen_desc=39, %CHOICE=8, raw_tokens=5, %LIST=1
  ```

  Split by what can move without an operator ruling
  (`python3 .../AT-35-E6-003-SWEEP_cycle5_test_region_census.py`, re-run at
  HEAD: `live_hits=373 hits_inside_cfg_test=351 hits_outside=22`):

  | where | hits | files |
  |---|---|---|
  | inside a `#[cfg(test)]` module in a live file | 351 | 39 |
  | reachable by code work | **22** | **10** |

  The reachable 22, by the job that owns each — every one a separate cycle:

  | job | hits | files |
  |---|---|---|
  | `race_trait_picker.rs` exclusion guards read off `raw_tokens` | 7 | `apps/desktop/src-tauri/src/race_trait_picker.rs` |
  | the `PU_*_DESC_TOKEN` verbatim corpus transcriptions | 4 | `pilot_compute/mod.rs` |
  | `external_ability_refs` carrying `!PRETEMPLATE:` guard tails | 3 | `crb/companion_data.rs` |
  | the `render_pcgen_desc` / `pcgen_desc.rs` catalog rewire | 4 | `class_feature_pool_catalog.rs`, `pcgen_desc.rs`, `pilot_compute/class_feature_grant_consumer.rs` |
  | `equipment_effects` comparing a qualifier to `TYPE=Circumstance` / `TYPE=Enhancement` | 2 | `equipment_effects/{arms_armor,equipmods}.rs` |
  | `CHOOSE:`/pool `TYPE=` prefix constants | 2 | `race_resolver.rs`, `skinwalker_change_shape.rs` |

  **Why none of the six was folded into this cycle.** Each was weighed, and the
  reason is the same one cycle 11 recorded for the two it called cheap: the rule
  is "nothing on the live side reads a PCGen token", not "no live file contains
  the characters `TYPE=`".

  * `race_trait_picker.rs` still needs `SheetRule.applies` to carry the
    exclusion-guard relation before the picker can stop parsing `PREVAREQ:` /
    `PREMULT` / `!PREFACT` itself. It is the only file under `apps/`, so it also
    carries the desktop-crate and frontend suites.
  * The four `PU_*_DESC_TOKEN` constants are the book's own words, pinned
    byte-for-byte against the `.lst` files by
    `sd27_pu_class_feature_descriptions_carry_the_characters_numbers`. Cycles 10
    and 11 declined to reword them and this cycle declines for the same reason.
    They are **counted** in the remainder, never exempted.
  * `crb/companion_data.rs`'s `external_ability_refs` is read as a list of
    ability keys by `companion_chassis.rs:929` and `monster_chassis.rs:766`, so
    the `!PRETEMPLATE:` tail needs a typed guard field on the **shared**
    companion/monster chassis — a sweep of the same 290-literal scale as this
    cycle's, across every book's `companion_data.rs` / `monster_data.rs`. It is
    a cycle, not a tail-end addition to one.
  * The `render_pcgen_desc` rewire is blocked on the converter-side prose
    carrier documented in `…/AT-35-E6-003_cycle5_converter-prose-blocker.md`.
  * `equipment_effects` and the two prefix constants each read an ingest
    qualifier string produced at run time by `lst_parser::equipment` and by the
    race pipeline's `automatic_trait_grants()`. Typing them means typing the
    field at the corpus boundary and rewriting the producers; relocating the
    literal alone would delete the gate's hit **without moving the read**, which
    is the masking shape cycles 4 and 5 were burned by.

  **351 of the 373 cannot move by any code work.** That remains the single
  highest-value action for this criterion and it is not a cycle: it is the
  operator ruling on whether a `#[cfg(test)]` module inside a live file is live
  code, filed by cycle 5 and asked again by cycles 6 and 10 under `progress.md`'s
  `## Open blockers`. The figure has now stood at 351–360 across eight cycles.

- **Discoveries:**
  - **The converter script's own idempotence was broken, and the round-trip
    oracle caught it.** `SELECTION_FIELD` matched only the inline field layout,
    so a second `--apply` read a multi-line literal as unconverted and reset it
    to `selection: None` — silently undoing the conversion on
    `damage_total.rs`'s Weapon Specialization fixture, the exact row whose `+2`
    the cycle exists to preserve. Two consecutive `--apply` runs now both print
    `converted_rows=13`. (`correction
    1789186513754-at-35-e6-003-sweep-687fd9`.) The generic lesson: a generator
    whose `--check` re-derives from its **own output** must be run twice before
    it is trusted once.
  - **`FeatEffectBonus.qualifiers` has exactly one positional live-side reader.**
    Re-derived, not assumed:
    ```bash
    grep -rn "\.qualifiers" src/ apps/ tests/ --include=*.rs | grep -v "^src/pcgen_import/"
    ```
    Every other hit is `lst_parser::equipment::BonusToken.qualifiers` (a
    different, `String`-based type) or a `#[cfg(test)]` assertion on a chain this
    cycle did not shorten. That is what made a length-changing conversion safe to
    attempt at all, and it is why the oracle run was still required rather than
    assumed.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=49 live_hits=373` | every source file under the five live roots, code lines only (B14) | `python3 scripts/pcgen_residue_gate.py --check` |
  | 386 → 373, 53 → 49 files | same | the same command at `f8e4aa78e1` and at `db0405eb18` |
  | reachable 34 → 22 in 14 → 10 files | the 373 hits, split on `#[cfg(test)]` membership | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` |
  | 12 hits cleared by this job / 11 RED lines | the four shipped feat tables scanned by the standing gate | `cargo test --locked --test sd35_rendered_prose_carries_no_ingest_vocabulary` (RED at the widening, GREEN after) |
  | 290 literals, 12 converted rows, 13 including the test fixture | every `FeatEffectBonus { … }` literal in the repo | `python3 .../AT-35-E6-003-SWEEP_cycle12_selection_typing.py --check` |
  | oracle 156 / 154 / 2 / 67 lines, 382 / 376 / 6 / 140 chassis, 29 characters | the fixture roster at `artifacts/epic-2-sheet-rule/oracle-parity/roster` | `cargo run --locked --release --bin sheet_rule_parity -- --roster <roster> --output <ours.json>` then `python3 scripts/oracle_harness/sheet_parity.py compare --ours <ours.json> --exports <exports> --output <parity.json>` |
  | `rust_lines_changed=801` | the cycle's own diff since `f8e4aa78e1` | `python3 scripts/cycle_scope_gate.py --receipt --since f8e4aa78e1 …` |
  | `records=49438 converted=49296 refused=142` | every corpus record the sheet-rule converter reads | `cargo run --locked --bin sheet_rule_convert -- --check` |

- **Build scope verified** — **once**, at `db0405eb18`, after the last
  figure-moving edit:
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → **3,326 passed / 0 failed / 15 ignored**
  - `cargo test --locked --no-fail-fast -j 6` → **414 targets / 8,845 passed /
    0 failed / 68 ignored / `FULL_EXIT=0`**, zero `FAILED` lines
  - `cargo clippy --locked --tests -j 6` → **0 warnings, 0 errors**
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `CONVERT_CHECK_EXIT=0`, every kind's `converted`/`refused` split unmoved
    (`records=49438 converted=49296 refused=142`)
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l`
    → **0**
  - `python3 scripts/completion_atlas.py --check` →
    `missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`
  - `python3 scripts/token_coverage.py --check` → `non_done=0 tokened=0
    token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1
    verdict=PASS`
  - `python3 scripts/shape_engine_boundary.py --check` →
    `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True`
  - `python3 scripts/missing_engine_tables.py --check` →
    `population=0 kinds=0 citation_failures=0`
  - `python3 scripts/denominator_gate.py --check …` →
    `files_checked=120 violations=0`
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (11 hits, 11 baseline
    rows)
  - **Desktop crate and frontend: epic cadence.** `apps/` is untouched by this
    cycle (`git diff --stat db0405eb18 f8e4aa78e1 -- apps/` empty).
  - **`docs/work-inventory.json` unchanged, and the guard is why.**
    `cargo run --locked --bin v06_work_inventory` **refused to write**: "this run
    would drop 7385 of the 32617 verification stamp(s) it currently carries …
    Set `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` … or
    pass `--allow-stamp-loss`". `--allow-stamp-loss` was **not** passed
    (`workflow-instruction.md §6` step 3). The refusal is the correct outcome for
    a cycle that regenerated no corpus artefacts, and the file is byte-identical
    to the cycle-start copy either way.

- **Sweep population:** N/A — no corpus record changed, so
  `corpus_literal_sweep` did not run (`workflow-instruction.md §6` step 3's
  guard).

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
  unchanged. The tool side is intact: `src/pcgen_import/` **gained** a module and
  lost none.

- **Retro events:** `correction 1789180880544-at-35-e6-003-sweep-83042e` (the
  dispatch's cycle number), `correction
  1789186513754-at-35-e6-003-sweep-687fd9` (the generator's idempotence bug),
  `deferral 1789188753496-at-35-e6-003-sweep-32ef4d` (the 373-hit remainder,
  mechanism by mechanism). One event misfiled to
  `docs/retro/events/sd31-transcribe.jsonl` before `--actor` was passed
  explicitly (shell state does not persist between tool calls, so the exported
  `RETRO_ACTOR` was gone); it is committed as-is rather than hand-edited out of
  an append-only log, and re-emitted correctly.

- **Status: `partial`.** The criterion's population is not zero at HEAD.

- **Notes:** the judgment call was stopping at one mechanism rather than also
  taking `crb/companion_data.rs`'s three hits, which look cheap. They are not:
  `external_ability_refs` is a field of the shared companion **and** monster
  chassis, read as an ability-key list by two traversals, and typing its guard
  tail is a sweep of this cycle's own size across every book. Starting it at the
  end of a cycle would have produced a half-typed field — worse than an untyped
  one, because the next cycle would have to establish which half.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle)`; the whole
  remainder, **373 code hits in 49 files**, of which **22 hits in 10 files** are
  reachable without the `#[cfg(test)]` ruling. The largest reachable job is now
  **`race_trait_picker.rs` (7 hits)** — but it is gated on `SheetRule.applies`
  carrying the exclusion-guard relation, so the one that should be taken next
  unblocked is the **`external_ability_refs` guard-tail typing (3 hits)**,
  dispatched as a chassis-wide sweep with its own round-trip oracle, exactly the
  shape this cycle used. The **operator ruling on `#[cfg(test)]` regions still
  stands between the gate and 351 of the 373** and no amount of dispatching will
  move it; it has now been asked by four cycles.
