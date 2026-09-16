# Cycle AT-35-E6-003-RULED cycle 10 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `db1fe3a04c` (the code, the census script and its JSON, the retro events),
  cycle start `e61bb2cf75`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `e61bb2cf75`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 9's closing figure — nothing drifted between the two cycles:
  ```
  live_files=16 live_hits=29 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/rules_core/equipment_record.rs` — **new, live.** `CorpusEquipmentRecord`, the
    equipment kind's own converted shape, the move cycle 8 made for spells
    (`rules_core::spell_record::CorpusSpellRecord`). Six settled fields — `identity`, `name`,
    `weight_lbs`, `cost_gp`, `ability_score_bonus`, `intelligent_item` — and **no token array
    and no bonus-chain array**. It carries this cycle's parity test.
  - `src/pcgen_import/ir_converter.rs` — **+~120.** New converter-side
    `equipment_record_to_corpus`, plus its two private helpers `ability_score_bonus_of` and
    `intelligent_item_contribution_of`. These are the three live reads this cycle retired,
    moved verbatim to the side of the boundary that owns the ingest vocabulary.
    `convert_equipment_record` builds the converted record and interns it (`Box::leak`),
    exactly as the spell path has done since cycle 8 — **one call, doing more, not a second
    call**, which is why the `ir_converter` hit count did not rise.
  - `src/pcgen_import/source_content_payload.rs` — `SourceContentPayload::Equipment` is a
    **pair**, `(&EquipmentRecord, &CorpusEquipmentRecord)`. The parser half stays only for the
    consumers that have not moved, and the variant collapses to the converted half alone when
    the last one does. `Eq` is dropped from the enum (a settled weight in pounds is an `f64`);
    `PartialEq` is what every caller uses, and the same one-line change follows on
    `SourceContentRecord`.
  - `src/rules_core/equipment_resolver.rs` — the three resolution passes become one private
    `resolve_equipment_pair`; `equipment_id_resolve` keeps its exact signature and answer, and
    the new **live** `equipment_converted_resolve` answers with the converted record.
    Identity, ordering and every fallback are one function, so the two can never answer with
    different records.
  - `src/rules_core/encumbrance.rs` — **`pcgen_import` import GONE.**
    `weight_and_cost_from_record`, which read the `WT:`/`COST:` tokens off the parser row, is
    deleted; `compute_encumbrance` and `equipment_key_resolves_a_carried_weight` read
    `CorpusEquipmentRecord::weight_and_cost` / `.weight_lbs`.
  - `src/rules_core/equipment_effects/magic_items.rs` — **`pcgen_import` import GONE.** The
    `BONUS:STAT` chain scan and the `TEMPBONUS:` fallback are converter-side;
    `compute_magic_items_effect` reads `record.ability_score_bonus`. `AbilityScoreBonus` stays
    the module's own public type.
  - `src/rules_core/equipment_effects/intelligent_item.rs` — **`pcgen_import` import GONE.**
    The `BONUS:VAR|IntItemStat*` family scan is converter-side;
    `compute_intelligent_item_effect` reads `record.intelligent_item`.
    `IntelligentItemContribution` and `ItemAlignment` stay the module's own public types.
  - `src/rules_core/equipment_effects.rs` — the two call sites resolve the converted record by
    the same item id they already resolved the parser row by. This file keeps its own hit; it
    is one of the seven consumers that have not moved.
  - `src/rules_core/mod.rs`, `src/rules_core/source_content.rs` — module registration and the
    `Eq` line above.
  - `apps/desktop/src-tauri/src/corpus_fixtures.rs`, `tests/sd17_{c,d,e}_*.rs` — pattern arity
    only, four match sites (`Equipment(x)` → `Equipment(x, _)` / `Equipment(..)`).
  - `…/AT-35-E6-003-RULED_cycle10_runtime_import_census.py` / `.json` — **new.** Imports cycle
    9's census whole (which imports cycle 8's, … back to cycle 1's) and rewrites three groups'
    reasons with this cycle's measurements.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one
    field, `derived_at`, restamped by this cycle's `completion_atlas.py --check`) and
    `docs/retro/events/sd31-transcribe.jsonl` (one appended event from another session).
    Committed rather than filtered away, per the standing "clean tree = unfiltered
    `git status` empty" rule.

  **No `data/` file and no corpus record was changed** (`git status --porcelain data/` empty),
  so `data/sheet_rules/` and `docs/work-inventory.json` are byte-identical to the cycle's start
  tree. **`apps/` WAS touched**, so the desktop crate and the frontend ran here, not at the
  epic wrap-up.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff -- src/ apps/ tests/ | grep '^+'` plus the two new files whole),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing. The three
  `+++ b/tests/sd17_*.rs` lines a naive run reports are **diff headers naming pre-existing
  files**, not identifiers in added code. First run, no self-heal.

- **Wired-integration audit result:** OK_NO_TOKENS, **after one self-heal**. The first run
  returned exactly one hit: the word *placeholder* in `equipment_record.rs`'s own module doc
  comment, in the sentence stating that nothing on the struct is one. Reworded ("Nothing here
  stands in for work not done"); the second run is clean. No `"Would …"` string, no inline
  mock, no fixture-only data path: the converter reads the same real corpus records the live
  side read, and the parity test asserts the real values over the whole live corpus.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  Plus the `-RULED` dispatch's own bar, which is the two rulings applied **and the call sites the
  corrected gate now sees cleared**.

  The Evidence sentence's `apps/desktop` clause stays met (`root apps/desktop files=0 hits=0`,
  first met in cycle 4, not regressed here even though this cycle wrote an `apps/` file), and
  the desktop crate and the frontend both ran green at this cycle's final tree. The criterion as
  a whole is **not** met: 26 hits across 13 files remain under `src/rules_core/`, and
  `render_pcgen_desc_with_values` is still called there.

- **Receipt rows (mechanical):**
  ```
  since=e61bb2cf75a3a750a0e1980748ece412e6a040f7 target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=747 ratio=n/a builds_recorded=1 pcgen_live_files=13
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.
  `pcgen_live_files=13` is the **file** count, down 3 from cycle 9's 16 — the first cycle since
  cycle 7 to move it. The hit count moved with it: 29 → 26.

- **PCGen residue:** `live_files=13 live_hits=26 baseline_files=260 baseline_hits=12736
  verdict=PASS` — down from cycle 9's `16 / 29` on **both** counts, **and the instrument was not
  touched this cycle** (`git diff --name-only e61bb2cf75..HEAD -- scripts/` is empty), so the
  `−3 / −3` is entirely code. Per-root:
  ```
  root src/rules_core         files=16 hits=29  ->  files=13 hits=26
  root src/saved_character    files=0  hits=0
  root src/campaign           files=0  hits=0
  root src/homebrew_authoring files=0  hits=0
  root apps/desktop           files=0  hits=0   (unchanged, though one apps/ file was written)
  ```
  **Three hits and three whole files, and all three are closures, not relabels.** The read does
  not move to another live file — it stops being a token read. Not gate-gaming: nothing renamed
  to duck a regex, no path exempted, no rebaseline, and the `use`-collapse still available in
  `source_content_payload` was **refused for the fourth cycle running**.

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no
  converter mapping row changed, and `data/sheet_rules/` is byte-identical
  (`sheet_rule_convert -- --check` `verdict=PASS`). The parity that **was** required for this
  change is the moved reads' own, and it is the widest this criterion has run:
  `every_live_corpus_equipment_record_carries_the_same_values_the_token_reads_produced` loads
  **every** book under `data/corpus/` — **7,803 equipment records** — re-derives all three reads
  the way the live modules derived them, straight off the parser row still paired with the
  converted record in the canonical envelope, and compares. **0 disagreements.** The test was
  **mutation-proved**: adding `+ 1.0` to the converter's weight read turns it red, and the
  mutation was reverted and re-verified green.

- **Movement, four buckets:**
  - **closure:** 3 live `pcgen_import` hits and 3 whole files, named by row —
    `encumbrance.rs:60`, `equipment_effects/magic_items.rs:22` and
    `equipment_effects/intelligent_item.rs:66`, all three
    `use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;`. **All three are
    closures:** each file's token reading is deleted from the live side, not moved to another
    live file — cycle 4 booked a similar move as a *relabel* precisely because the calls
    reappeared elsewhere, and these do not. The census asserts it by file
    (`cleared_by_cycle10=3`). **Zero corpus units**, as Epic 6 closes none.
  - **relabel:** none. No hit moved from one file or group to another.
  - **reachability:** none.
  - **instrument-correction:** none. The gate script and
    `scripts/pcgen-residue-baseline.env` are absent from this cycle's diff.

- **Refused tokens:** `renderer=5, lst_parser_types=7, ingest_record_tokens=5,
  trait_and_pool_tokens=3, ir_converter=1, source_content_payload=5` — **26 hits / 13 files,
  summing, all under `src/rules_core/`.** Six groups, under this cycle's flag-cap of 10.

- **Discoveries:**
  - **The refusal cycles 8 and 9 made was right about the shape and wrong that it made the
    equipment half indivisible** (`correction 1789285081035-at-35-e6-003-ruled-b8ecc2`). Every
    census from cycle 1 on, and both of the last two receipts, recorded the equipment half as
    one piece blocked on the converter emitting equipment `SheetRule` rows, on the ground that
    *"a `CorpusEquipmentRecord` carrying those arrays would move PCGen token structures UNDER a
    live root, which is worse than the hit it clears."* The array clause is correct and is not
    relitigated. What the reason did not notice is that a record carrying **no** array clears
    consumers one at a time: the traversal moves the other way, onto the converter, and each
    consumer reads one settled value. Three moved in one cycle, and the path is now built and
    proved corpus-wide for the other seven.
  - **The converted rules already carry what the remaining consumers need, and that is a
    measured fact, not a hope.** `data/sheet_rules/core_rulebook/equipment/chain_shirt.json`
    states the armour bonus as `value: Number(Const 4)` with `target: Ac` and
    `bonus_type: Armor`, and its check penalty, max Dex and arcane spell failure as
    `StatBlock` prose families (`-2`, `4`, and `20` printed with a percent sign). `longsword.json` states
    `value: Dice{dice: "1d8"}`. Those are the `arms_armor` and `damage_total` reads, already
    settled, already shipped. Re-derive: `python3 -c` over either file.
  - **`SourceContentPayload` cannot be `Eq` once it carries a settled weight**, and that is the
    right trade. A price in gold and a weight in pounds are `f64`; `Eq` was never used by a
    caller (the workspace builds and all 419 targets pass without it).

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=13 live_hits=26`, was `16 / 29` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | 26 hits split `renderer=5, lst_parser_types=7, ingest_record_tokens=5, trait_and_pool_tokens=3, ir_converter=1, source_content_payload=5`; `gate_agreement=OK (26 == 26)` | the same 26 hits, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle10_runtime_import_census.py` |
  | 3 files cleared, by name (`cleared_by_cycle10=3`); `live_converted_equipment_record=present token_arrays_on_it=0`; `equipment_converted_resolve=present` | the 29 hits at cycle start vs the 26 at HEAD | the census's own row and file assertions, same command |
  | 7,803 equipment records compared, 0 disagreements | every `data/corpus/<book>/equipment/**/*.json` the live loader reads (a `data` object with a `key`) | `cargo test --locked --lib every_live_corpus_equipment_record_carries_the_same_values_the_token_reads_produced` |
  | that test can fail | the same population, with `+ 1.0` on the converter's weight read | the same command, after the mutation — recorded red, then reverted and re-verified green |
  | Chain Shirt's converted AC `Number(Const 4)` / `target: Ac`, and its `-2` / `4` / `20` stat-block lines; Longsword's `Dice 1d8` | the two records' own converted rule files | `python3 -c "import json;print(json.load(open('data/sheet_rules/core_rulebook/equipment/chain_shirt.json')))"` |
  | 7 `EquipmentRecord` imports remain | every live file naming the type | `grep -rln 'lst_parser::equipment::EquipmentRecord' --include=*.rs src/rules_core/` |
  | 9 prior receipts, so this is cycle 10 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle*_receipt.md \| wc -l` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, desktop
  crate in `/tmp/cargo-sd35-AT-35-E6-003-RULED-desktop`, once each, at the final tree.
  `apps/` was touched, so the desktop crate and the frontend ran **here**, not at the epic
  wrap-up.
  ```
  NO_RUN_EXIT=0
  cargo test --locked --lib -j 6      -> test result: ok. 3349 passed; 0 failed; 16 ignored
                                         (cycle 9's 3348 + this cycle's one new parity test)
  cargo test --locked --no-fail-fast  -> FULL_EXIT=0; 418 Running targets + 1 Doc-tests;
                                         8,878 passed; 0 failed; 69 ignored; zero
                                         `test result: FAILED` lines (cycle 9 recorded 8,877;
                                         the +1 is exactly this cycle's one new test)
  desktop crate (apps/desktop/src-tauri)
                                      -> DESKTOP_EXIT=0; 570 passed; 0 failed; 0 ignored
                                         (cycle 9's 570, unchanged)
  frontend (apps/desktop, npm test)   -> FRONTEND_EXIT=0; 101/101 test files passed
  cargo clippy --locked --tests -j 6  -> CLIPPY_EXIT=0, 0 warnings, after one self-heal
                                         (`collapsible_if` on this cycle's own parity test;
                                         rewritten as a match guard)
  desktop clippy --locked --tests     -> DCLIPPY_EXIT=0, 1 warning, PRE-EXISTING and named
                                         not swept: `vec_init_then_push`,
                                         `equipment_catalog.rs:889`, untouched file, the
                                         same warning cycles 2, 4 and 9 recorded
  cargo run --bin sheet_rule_convert -- --check
                                      -> records=49438 converted=49296 refused=142
                                         rules=70135 var_tables=5293 verdict=PASS (116.6s)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l   -> 0
  python3 scripts/pcgen_residue_gate.py --check   -> live_files=13 live_hits=26 verdict=PASS
  python3 -m unittest scripts.tests.test_pcgen_residue_gate -> Ran 27 tests, OK
  python3 scripts/completion_atlas.py --check     -> citation_failures=0 stale_derived_at=False
                                                     missing_clearing_mechanisms=0
  python3 scripts/token_coverage.py --check       -> non_done=0 refused=142 token_types=233 PASS
  python3 scripts/shape_engine_boundary.py --check-> magnitude_bearing=26396 not_held_by_engine=0
  python3 scripts/missing_engine_tables.py --check-> population=0 citation_failures=0
  python3 scripts/denominator_gate.py --check ... -> files_checked=139 violations=0
  scripts/verify.sh --only pi-sweep               -> RESULT: PASS
  corpus_literal_sweep                            -> not run; no corpus record changed
                                                     (`git status --porcelain data/` empty)
  ```

- **Sweep population:** N/A — `corpus_literal_sweep` runs only when corpus records changed, and
  no `data/` file was written this cycle.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.

- **Status:** `partial`.

- **Notes:** The `source_content_payload` trim — repointing `spell_resolver.rs`'s
  `SourceContentPayload` import at `rules_core::source_content`'s own re-export of the same
  enum, for `−1` and zero change in what the module depends on — is **refused for the fourth
  cycle running**, on the same reasoning cycles 7, 8 and 9 gave. Separately, on the pair: the
  `Equipment` payload carrying both the parser row and the converted record is a transition
  shape and is written down as one, in the variant's own doc comment and in the census. It is
  not a permanent design and it costs one `Box::leak` per equipment record, the same allocation
  the spell path has made since cycle 8.

- **Next-cycle scope:** **AT-35-E6-003-RULED cycle 11**, `SCOPE_GATE: EXEMPT (Epic 6 cycle)`,
  on the 26-hit remainder. The unblocked piece is the rest of the equipment shape — 18 hits
  across 11 files — and it is now a **sequence of consumer moves on a built path**, not one
  indivisible piece: `arms_armor` (AC / MAXDEX / ACCHECK / SPELLFAILURE, whose converted values
  this receipt showed already shipped), `damage_total` (damage dice, crit, wield), `equipmods`
  (weapon enhancement, SR, VAR references), `general` (skill and VAR bonuses),
  `equipment_resolver` (the key token), then `equipment_effects` and `corpus_loader`'s rebuild
  last, at which point the payload collapses to the converted half alone. The `renderer` group
  (5) stays a converter-parity cycle (cycle 2's `disagree=97,332 of 660,320`), and
  `trait_and_pool_tokens` (3) stays refused on cycle 7's corrected number.
