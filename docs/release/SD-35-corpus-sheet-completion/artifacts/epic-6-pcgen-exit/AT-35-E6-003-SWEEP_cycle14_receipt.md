# Cycle AT-35-E6-003-SWEEP cycle 14 — Epic 6 PCGen exit / AT-35-E6-003-SWEEP

- **Commit SHA:** `0edcc614ee`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by
  design; decisions.md §2, workflow-instruction.md §6 step 1)`

  Both gates ran anyway. The corpus floor, at the cycle's start tree
  `bdae51f9f6`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree:
  ```
  live_files=48 live_hits=370 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

  **The dispatch's cycle number was wrong for the fourth consecutive cycle.** It
  said "CYCLE NUMBER FOR THIS CRITERION: 13", and it handed on cycle **12**'s
  refused-token line (`TYPE==104, BONUS:=91, PRE[A-Z]+:=63, DESC:=59,
  render_pcgen_desc=39, %CHOICE=8, raw_tokens=5, %LIST=1`, summing to 373).
  Cycle 13's receipt is committed at `bdae51f9f6` and its own result is the
  **370** the residue gate reported at that tree. This is cycle 14. Cycles 11,
  12 and 13 each recorded this same defect on their own dispatch; that is now
  four in a row, which `AGENTS.md` rule 8 says is a missing mechanism and not
  bad luck. **The mechanism, asked for a fourth time:** derive it at dispatch
  time rather than carry it in prose —
  ```bash
  ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle*_receipt.md | wc -l
  ```
  plus one, and take the refused-token line from the newest of those receipts
  rather than from the dispatch text.

- **Files touched:**
  - `src/pcgen_import/equipment_bonus_reader.rs` — **new.** The converter-side
    home for the two equipment bonus-TYPE classifications, with the corpus-wide
    round-trip gate and two predicate-difference tests.
  - `src/pcgen_import/mod.rs` — registers it.
  - `src/pcgen_import/race_trait_tokens.rs` — gains
    `ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX` (moved from `rules_core`),
    `adopted_race_pool_suffix`, the private
    `SKINWALKER_CHANGE_SHAPE_POOL_PREFIX` and `skinwalker_change_shape_kin`.
  - `src/rules_core/equipment_effects/arms_armor.rs` — the circumstance
    exclusion now asks the converter.
  - `src/rules_core/equipment_effects/equipmods.rs` — the enhancement match now
    asks the converter.
  - `src/rules_core/race_resolver.rs` — the `pub const` is gone; the call site
    asks `race_trait_tokens::adopted_race_pool_suffix`.
  - `src/rules_core/skinwalker_change_shape.rs` — the `const` is gone; the call
    site asks `race_trait_tokens::skinwalker_change_shape_kin`.
  - `src/bin/ingest_race_traits.rs` — imports the moved constant from its new
    tool-side home (this bin is tool side).
  - `tests/sd35_live_side_names_no_ingest_qualifier.rs` — **new standing gate**
    (RED first, see below).
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    — `derived_at` stamp only, written by `completion_atlas.py --check`.
  - `docs/retro/events/at-35-e6-003-sweep.jsonl`,
    `docs/release/SD-35-corpus-sheet-completion/{progress.md,kanban.md}`, this receipt.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.** On this cycle's own diff,
  ```bash
  git diff --unified=0 bdae51f9f6 -- src/rules_core src/pcgen_import src/bin \
    apps/desktop/src-tauri/src tests ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ```
  → the only matches are the diff's own `diff --git` / `---` / `+++` headers
  naming the new test file `tests/sd35_live_side_names_no_ingest_qualifier.rs`.
  No **identifier** in shipping code carries a bundle tag; a test *file name* in
  a diff header is the same documented citation-exclusion class cycles 9–13
  recorded.

- **Wired-integration audit result:** **OK_NO_TOKENS**, first run, no fix
  needed.

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
  since=bdae51f9f6 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=599 ratio=n/a builds_recorded=0 pcgen_live_files=47
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
  pattern TYPE= files=20 hits=100
  live_files=47 live_hits=366 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Down from cycle 13's `live_files=48 live_hits=370`; never above it. The whole
  movement is in one pattern — `TYPE=` 104 → 100, in 21 → 20 files — which is
  exactly the four literals this cycle moved, and is the check that no hit was
  banked by reclassification into some other pattern.

- **RED → GREEN, in that order.** The new standing gate
  `tests/sd35_live_side_names_no_ingest_qualifier.rs` was written and run
  **before** a line of source was edited, and failed with exactly the four hits
  the python residue gate attributes to those files:
  ```
  4 executable line(s) on the live side still name an ingest qualifier
  src/rules_core/equipment_effects/arms_armor.rs:142: `TYPE=` -- && !qualifiers.iter().any(|q| q == "TYPE=Circumstance");
  src/rules_core/equipment_effects/equipmods.rs:231: `TYPE=` -- if qualifiers.len() >= 4 && qualifiers[3].eq_ignore_ascii_case("TYPE=Enhancement") {
  src/rules_core/skinwalker_change_shape.rs:78: `TYPE=` -- const POOL_PREFIX: &str = "TYPE=Skinwalker Change Shape ";
  src/rules_core/race_resolver.rs:1102: `TYPE=` -- pub const ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX: &str = "ABILITYSELECTION|Special Ability|TYPE=";
  ```
  Green after the move: `test result: ok. 2 passed; 0 failed`.

  The gate is a **ratchet**, and its module doc says why it is a per-file list
  rather than a whole-tree scan: the whole-tree scan already exists
  (`scripts/pcgen_residue_gate.py`) and is the authority, but a Rust test
  asserting zero across the tree would be red on arrival and stay red until the
  `#[cfg(test)]` operator ruling lands, which makes it a wish rather than a
  gate. Every future cycle that clears a file appends it, and `cargo test` then
  holds it clear without the python gate being run by hand.

- **Oracle parity:** **not run, and why.** No `Number` mapping was added
  (`workflow-instruction.md §6` step 3 runs the oracle comparison when one is),
  no corpus record changed, and no converter mapping row was added. This cycle
  changes **which module owns a string comparison**, and the comparison itself
  is pinned unchanged against the whole live corpus by the round-trip gate
  below, which is a stronger statement about this particular change than a
  29-character roster is: it covers **2,223** chains rather than the handful a
  roster reaches. Cycle 13's `…_cycle13_sheet-parity-after.json` remains the
  standing parity artifact and nothing in this cycle can move it.

- **What this cycle actually fixed, and why it is not a relocation dodge.**

  This is the distinction the cycle turns on, so it is stated plainly. Moving a
  literal from `src/rules_core/` to `src/pcgen_import/` reduces the residue
  gate's count whether or not it fixes anything — that is precisely the
  "banking a hit by relocation" failure cycle 13 warned about. The test that
  separates the two is **whether the live call site still has to know the
  ingest grammar to be written**. `src/pcgen_import/bonus_chain_reader.rs`'s
  own module doc states the bar: after it, "no live module names a qualifier
  position, a chain keyword … or the ingest field itself", and what the live
  side gets back is "narrowed, already-classified values … not the chains under
  another name".

  All four sites clear that bar:

  | before (live side knew the grammar) | after (live side asks a rules question) |
  |---|---|
  | `!qualifiers.iter().any(\|q\| q == "TYPE=Circumstance")` | `!equipment_bonus_reader::declares_circumstance_bonus_type(bonus)` |
  | `qualifiers.len() >= 4 && qualifiers[3].eq_ignore_ascii_case("TYPE=Enhancement")` | `equipment_bonus_reader::roll_bonus_carries_enhancement_type(bonus)` |
  | `choice_pool_suffix(&record.data, ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX)` | `race_trait_tokens::adopted_race_pool_suffix(&record.data)` |
  | `grant.strip_prefix(POOL_PREFIX)` | `race_trait_tokens::skinwalker_change_shape_kin(&grant)` |

  In every row the live side stops naming a **qualifier position** (`[3]`), a
  **bonus-type spelling** (`Circumstance`, `Enhancement`), a **`CHOOSE:`
  payload shape**, and a **pool prefix**, and starts asking *is this a
  circumstance bonus?* / *is this an enhancement bonus on a roll?* / *which
  trait pool does this selector adopt from?* / *which kin is this pool for?* —
  each answerable without ever having read a `.lst` file. The two prefix cases
  also move the `starts_with` / `strip_prefix` **arithmetic** across, not just
  the string: `race_resolver` previously passed the prefix *into* a converter
  helper, which is the live side supplying the grammar and is the shape §11
  forbids regardless of which module runs the byte comparison.

  The contrast case is recorded below under the refused remainder:
  `render_pcgen_desc` was **deliberately not moved**, because moving it would
  leave the live side still holding and passing a raw `DESC:` token, which
  would lower the count without changing anything real.

- **How losslessness is proved** — one gate over the live corpus, not a
  fixture. `equipment_bonus_reader::tests::bonus_type_qualifiers_are_unchanged_by_this_module`
  walks **every** `data/corpus/<book>/equipment*/**/*.json` record, and for
  every bonus chain evaluates *both* the old predicate — transcribed verbatim
  from `arms_armor.rs:142` and `equipmods.rs:231` as they stood at
  `bdae51f9f6`, deliberately transcribed rather than referenced, because a
  round trip proved against a paraphrase proves nothing — and the new function,
  then asserts they agree. Result: **0 disagreements over 2,223 chains on 1,371
  records**, with 68 chains classified circumstance and 277 classified
  enhancement-roll-typed. The test also refuses to pass on an empty walk: both
  populations must be non-empty, so a walk that silently stopped finding
  records cannot agree with itself about nothing.

  Two further tests pin the parts a tidy-up would most plausibly break:
  `the_two_predicates_keep_their_deliberate_differences` holds that
  circumstance scans every position while enhancement is positional, and that
  enhancement is case-insensitive while circumstance is not — two asymmetries
  that look like oversights and are not; `the_token_and_slice_entry_points_agree`
  holds the `BonusToken` and bare-slice entry points together so the live side
  and the corpus gate cannot drift.

  The two prefix moves carry no new round-trip test because neither changed a
  value: `adopted_race_pool_suffix` is `choice_pool_suffix` with the one prefix
  the live side used to pass in, and `skinwalker_change_shape_kin` is the same
  `strip_prefix`. Both are covered by the existing suites over those modules,
  which stayed green.

- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. Epic 6 closes no corpus unit.
  - **relabel (bucket to bucket):** none. `regressed=0 added=0 dropped=0`;
    `docs/work-inventory.json` is byte-identical to `bdae51f9f6`'s.
  - **reachability:** 4 live-side PCGen hits removed, 1 live file to zero. The
    reachable (non-`#[cfg(test)]`) remainder falls **19 → 15** and the files
    carrying it **9 → 5**.
  - **instrument-correction:** a new standing gate exists that did not before,
    holding 4 live files at zero executable ingest-qualifier hits. A
    **widening**: it went RED on arrival, on real hits, before any source edit.

- **Refused tokens** — eight types, summing to **366**, which is the gate's own
  `live_hits`:
  ```
  TYPE==100, BONUS:=91, PRE[A-Z]+:=63, DESC:=59, render_pcgen_desc=39, %CHOICE=8, raw_tokens=5, %LIST=1
  ```

  Split by what can move without an operator ruling
  (`python3 .../AT-35-E6-003-SWEEP_cycle5_test_region_census.py`, re-run at
  HEAD: `live_hits=366 hits_inside_cfg_test=351 hits_outside=15`):

  | where | hits | files |
  |---|---|---|
  | inside a `#[cfg(test)]` module in a live file | 351 | 42 |
  | reachable by code work | **15** | **5** |

  The reachable 15, by the job that owns each:

  | job | hits | files |
  |---|---|---|
  | `race_trait_picker.rs` exclusion guards read off `raw_tokens` | 7 | `apps/desktop/src-tauri/src/race_trait_picker.rs` |
  | the `PU_*_DESC_TOKEN` verbatim corpus transcriptions | 4 | `pilot_compute/mod.rs` |
  | the `render_pcgen_desc` / `pcgen_desc.rs` catalog rewire | 4 | `class_feature_pool_catalog.rs`, `pcgen_desc.rs`, `pilot_compute/class_feature_grant_consumer.rs` |

  **This cycle took the whole of the one job cycle 13 named as unblocked**, and
  the three that remain are each blocked on a named artifact. Per `AGENTS.md`
  rule 8 ("re-test a hazard before repeating it"), the `render_pcgen_desc`
  blocker was **re-verified this cycle rather than copied forward**, because
  the obvious move — relocating `src/rules_core/pcgen_desc.rs` wholesale into
  `src/pcgen_import/` — would clear 4 hits in one edit and is exactly the kind
  of thing a cycle under pressure does. It was rejected on inspection, and the
  evidence is the signature:

  ```rust
  pub fn render_pcgen_desc(raw: &str) -> RenderedPcgenDesc
  ```

  `raw` is a raw PCGen `DESC:` token, and `class_feature_pool_catalog.rs:448`
  and `class_feature_grant_consumer.rs:613` both hold one and pass it in at run
  time. Relocating the renderer would move the *rendering* to the tool side
  while leaving the live side holding the token — the count would fall by 4 and
  nothing real would change. That is banking a hit by relocation, the failure
  cycle 13 named, and it is the mirror image of what the four sites above did
  legitimately. The real fix remains the converter-side prose carrier
  documented in `…/AT-35-E6-003_cycle5_converter-prose-blocker.md`: the
  converter emits resolved prose and the live side never sees a `DESC:` token
  at all.

  The other two are unchanged from cycle 13 and were not re-litigated:
  * `race_trait_picker.rs` still needs `SheetRule.applies` to carry the
    exclusion-guard relation before the picker can stop parsing `PREVAREQ:` /
    `PREMULT` / `!PREFACT` itself. It is the only file under `apps/`, so it
    also carries the desktop-crate and frontend suites.
  * The four `PU_*_DESC_TOKEN` constants are the book's own words, pinned
    byte-for-byte against the `.lst` files by
    `sd27_pu_class_feature_descriptions_carry_the_characters_numbers`. Cycles
    10–13 declined to reword them and this cycle declines for the same reason.
    They are **counted** in the remainder, never exempted.

- **Discoveries:** one, recorded as a `correction`. Cycle 13's next-cycle scope
  costed this job as "4 hits, 4 files … a converter-side widening with its own
  round-trip oracle", and separately costed it (in its own Notes) as "a
  converter cycle of this cycle's own size on a record type this cycle never
  opened" — cycle 13 itself being a 711-line, 452-literal sweep. The actual
  cost was **110 Rust lines changed**, no corpus record touched and no data
  literal rewritten, because the equipment bonus type is already a **qualifier
  on a chain the record carries**, not a field that had to be added to 452
  literals. The estimate was taken from the previous cycle's shape rather than
  from the shape of the work in front of it — the
  `every-figure-states-its-denominator` error with the denominator being "what
  the last cycle cost".

  **A second correction, on this receipt's own figure.** It first recorded
  `rust_lines_changed=110`, measured before the commit — when
  `equipment_bonus_reader.rs` and the new standing gate were still untracked and
  therefore invisible to the `git diff` the receipt tool runs. Re-derived at HEAD
  with both files tracked, it is **599**
  (`correction 1789195888613-at-35-e6-003-sweep-ffd482`). The 110 figure is kept
  above as the pre-existing-file edit count, which is the number that made the
  cost-estimate point, but the receipt row is the tool's own output and is now
  599. The general shape: a receipt row computed before `git add` measures a
  different population than the same command after it.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=47 live_hits=366` | every source file under the five live roots, code lines only (ruling B14) | `python3 scripts/pcgen_residue_gate.py --check` |
  | 370 → 366, 48 → 47 files | same | the same command at `bdae51f9f6` and at HEAD |
  | `TYPE=` 104 → 100 in 21 → 20 files | the same scan, one pattern | `python3 scripts/pcgen_residue_gate.py --check` |
  | reachable 19 → 15 in 9 → 5 files | the 366 hits, split on `#[cfg(test)]` membership | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` |
  | 4 RED lines, then 0 | the 4 live files the new gate lists as cleared | `cargo test --locked --test sd35_live_side_names_no_ingest_qualifier` (RED at `bdae51f9f6` + the test file, GREEN at HEAD) |
  | 1,371 records / 2,223 chains / 68 circumstance / 277 enhancement, 0 disagreements | every `data/corpus/<book>/equipment*/**/*.json` record carrying a bonus chain | `cargo test --locked --lib equipment_bonus_reader` |
  | `rust_lines_changed=599` | the cycle's own diff since `bdae51f9f6`, measured at HEAD with both new files tracked | `python3 scripts/cycle_scope_gate.py --receipt --since bdae51f9f6 --before /tmp/wi-before-at-35-e6-003-sweep.json --after docs/work-inventory.json` |
  | `records=49438 converted=49296 refused=142` | every corpus record the sheet-rule converter reads | `cargo run --locked --bin sheet_rule_convert -- --check` |

- **Build scope verified** — **once**, after the last figure-moving edit:

  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --no-fail-fast -j 6` → **416 targets / 8,854 passed / 0 failed / 68 ignored / `FULL_EXIT=0`**, zero
    `FAILED` lines (`grep -c FAILED` → 0). Cycle 13 recorded 415 targets and
    8,849 passed; the +1 target is this cycle's new test file and the +5 passing
    are its 2 tests plus `equipment_bonus_reader`'s 3
  - `cargo clippy --locked --tests -j 6` → **`CLIPPY_EXIT=0`, 0 warnings, 0 errors**
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `records=49438 converted=49296 refused=142 rules=70135
    var_tables=5293 verdict=PASS`, identical to cycles 3–13
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l`
    → **0**
  - `python3 scripts/pcgen_residue_gate.py --check` → `live_files=47
    live_hits=366 … verdict=PASS`
  - `python3 scripts/completion_atlas.py --check` → `unclassified=0 overlap=0
    done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
    citation_failures=0`, exit 0
  - `python3 scripts/token_coverage.py --check` → `non_done=0 tokened=0
    token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1
    verdict=PASS`, exit 0
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, exit 0
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, exit 0
  - `python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'`
    → `files_checked=122 violations=0`
  - `python3 scripts/denominator_gate.py --check-provenance` →
    `files_checked=239 figures_examined=574 violations=0`
  - `scripts/verify.sh --only pi-sweep` → **RESULT: PASS** (1 stage passed, `PISWEEP_EXIT=0`)
  - `cargo run --locked --bin corpus_literal_sweep` — **not run, and why**: no
    corpus record changed this cycle (`git status --porcelain data/` empty), and
    `workflow-instruction.md §6` step 3 runs it only when they do.
  - **desktop crate and frontend: epic cadence.** `apps/` was not touched —
    `git diff --stat bdae51f9f6 -- apps/` is empty.

- **Sweep population:** N/A — no corpus record changed. `data/corpus/**` is
  byte-identical at HEAD (`git status --porcelain data/` empty). The corpus was
  **read** by the new round-trip gate, never written.

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

- **Status: `partial`.** The criterion's population is not zero at HEAD.

- **Notes:** the judgment call this cycle was declining to relocate
  `src/rules_core/pcgen_desc.rs`, which would have doubled the cycle's headline
  number from 4 hits to 8 in one `git mv`. The reason it was declined is
  written above in full rather than as a one-line deferral, because the same
  edit will look attractive to the next cycle too, and the argument against it
  is the argument that distinguishes every legitimate move this cycle made from
  a count-lowering one.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle)`; the whole
  remainder, **366 code hits in 47 files**, of which **15 hits in 5 files** are
  reachable without the `#[cfg(test)]` ruling. **There is no unblocked code job
  left.** All three remaining reachable jobs are blocked on a named artifact,
  and two of those artifacts are themselves cycle-sized converter work that
  nobody has been dispatched to build:
  1. the **converter prose carrier** (`…/AT-35-E6-003_cycle5_converter-prose-blocker.md`)
     — unblocks `render_pcgen_desc`, 4 hits;
  2. **`SheetRule.applies` carrying the exclusion-guard relation** — unblocks
     `race_trait_picker.rs`, 7 hits, and is the only `apps/` file left;
  3. the `PU_*_DESC_TOKEN` transcriptions, 4 hits, which are the book's words
     and move only when `pcgen_desc.rs` goes.

  Dispatching "the next sweep cycle" against this remainder will produce
  nothing. The next dispatch should name **(1)** or **(2)** as its own
  criterion and build the artifact. **The operator ruling on `#[cfg(test)]`
  regions stands between the gate and 351 of the 366** and no amount of
  dispatching will move it; it has now been asked by six cycles.
