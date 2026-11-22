# Cycle 2 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 1 named the twelve surviving `apps/desktop/` files' blocker as **one buildable thing** — a
catalog-mode prose renderer. This cycle **built it**, and then **tried the lookup it was supposed
to unblock** on one catalog. The renderer works, corpus-wide. The lookup does not yet, and the
reason is not the renderer: it is **row coverage in the converted package**. The swap was
reverted rather than shipped, and the measurement is the cycle's second deliverable.

- **Commit SHA:** `1143318c92` — the renderer, its corpus-wide gate, the two public word helpers,
  the measurement artifact and this cycle's four retro events. Cycle start `99174263c1`. A later
  commit carries this receipt and the `progress.md` / `kanban.md` rows (a receipt cannot name the
  commit that carries it).
- **Scope gate:**
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Run anyway, for the record — `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus reached `DONE 49438 of 49438` at `AT-35-E5-005`. Epic 6
  moves no unit; it takes the ingest format off the live side.
- **Files touched:** **5** (`git show --stat 1143318c92 | tail -1` → `5 files changed, 714
  insertions(+)`) — 3 modified, 2 added, 0 renamed, 0 deleted.
  - **`src/rules_core/sheet_rule_catalog.rs` (added, 499 lines)** — the catalog-mode renderer
    and its 11 tests, one of which is the corpus-wide gate.
  - **`src/rules_core/mod.rs` (+1)** — the module declaration.
  - **`src/rules_core/level_up_option_filter.rs` (+17)** — `expr_words` and `words_of_id`, the
    public form of the describer that module already owns and already tests. **No second
    describer was written**: a second place for the sheet's words to drift is the defect, not
    the fix.
  - **Artifact (added):**
    `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle2_converted-row-coverage.md`
    — the measurement, with every figure's re-derive command.
  - **Retro (modified):** `docs/retro/events/at-35-e6-003.jsonl` (+4).
  - **Zero `apps/` files changed. Zero `scripts/` files changed. Zero `data/` files changed.
    Zero `tests/` files changed.** `git status --porcelain -- apps/ scripts/ data/ tests/` empty
    at every checkpoint, which is why the desktop crate and the frontend correctly run at epic
    cadence (`decisions.md §3`) rather than here.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS for this cycle; 1 match against the develop
  base, pre-existing and not this cycle's.**
  ```
  CS=99174263c149f06d2b470443252ba9c87ec40180
  SC="src/rules_core src/pcgen_import src/bin src/oracle_validation apps/desktop/src-tauri/src apps/desktop/src"
  git diff --unified=0 $CS -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  (no output)  OK_NO_BUNDLE_TAGS
  BASE=$(git merge-base HEAD origin/develop)      # fe5ae6cd4a
  git diff --unified=0 "${BASE}...HEAD" -- $SC … | grep -cE '…'          ->  1
  awk '/(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8})/{n++} END{print n}' /tmp/e6003c2/base.diff -> 1
  ```
  Two independent implementations agree (`AGENTS.md §Concurrency`: derive counts with `awk`, not
  `grep -o`). **This cycle's contribution is 0.**
- **Wired-integration audit result:** **OK_NO_TOKENS for this cycle; 8 matches against the
  develop base by `grep -c`, 11 by an unanchored `awk`, all pre-existing, none a stub marker.**
  ```
  git diff --unified=0 $CS -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
  ->  (no output)  OK_NO_TOKENS
  git diff --unified=0 "${BASE}...HEAD" -- $SC … | grep -cE '…'   ->  8
  awk '/(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)/{n++} END{print n}' /tmp/e6003c2/base.diff -> 11
  ```
  The two figures differ because `grep -E` applies `\b` word boundaries and the `awk` form does
  not; **both are stated with the command that produced them** rather than one being quietly
  preferred. The matches are the domain word `placeholder` naming the source format's own
  `SOURCEPAGE:p.xx` non-citation and its `%LIST` counterpart, attributed by AT-35-E6-001/002
  cycles 1–6. Text unchanged, count unchanged by this cycle.
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  | clause | at HEAD | met? |
  |---|---|---|
  | desktop `raw_tokens` readers read `SheetRule.applies`/`prose` | 5 files still read the array; unchanged this cycle | **no** |
  | `render_pcgen_desc` deleted from the live side | 14 live files / 106 hits; unchanged this cycle | **no** |
  | **zero hits under `apps/desktop/`** | **12 files / 259 hits**, unchanged this cycle | **no** |
  | desktop crate suite green | epic cadence — `apps/` untouched (`decisions.md §3`); last run `575 passed; 0 failed` at cycle 1 | **carried** |
  | frontend suite green | epic cadence; last run `101/101 test files passed` at cycle 1 | **carried** |
  | the 19 on-screen tests still pass | epic cadence; last run `19 per-kind + 5 section tests passed` at cycle 1 | **carried** |

  **Status `partial`**, remainder named below and unchanged in size.
- **Receipt rows (mechanical):**
  ```
  since=99174263c149f06d2b470443252ba9c87ec40180 target_dir=/tmp/cargo-sd35-AT-35-E6-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=517 ratio=n/a builds_recorded=2 pcgen_live_files=208
  ```
  `closed=0` / `relabeled=0` is correct: the unit population was already `0` non-DONE at cycle
  start. `ratio` is `n/a`, a division by zero, never `0.0`. `pcgen_live_files` is **flat at
  208** — this cycle added a live module that contains no PCGen vocabulary and removed no live
  read, which is exactly what a flat count should mean.
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, at `1143318c92`):
  ```
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=12 hits=259
  identifier_files=17 identifier_hits=144
  live_files=208 live_hits=11673 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  **`verdict=PASS`, every axis flat.** Identical to cycle 1's closing line on every field. The
  new module adds **zero** hits: it is 499 lines of live code that names no token, no formula
  string and no ingest field — the property the criterion is ultimately asking for, demonstrated
  on a new file rather than argued.
- **Oracle parity:** **N/A for this cycle.** No `Number` mapping was added and no computed value
  moved. Epic 6 touches a live path, so the row is owed an answer: the new module is **additive**
  — no existing function's body changed, and the two helpers added to
  `level_up_option_filter.rs` delegate to its own unchanged `describe_expr`/`pretty`. The
  library suite is unchanged except for the 11 new tests (see Build scope).
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the unit population was already 0 non-DONE.
  - **relabel (bucket to bucket):** 0.
  - **reachability:** 0. This cycle evaluated nothing and converted nothing.
  - **instrument-correction:** 0. `scripts/pcgen_residue_gate.py` was **not touched**
    (`git status --porcelain -- scripts/` empty), and the residue line is flat.
- **Refused tokens:** **none.** This cycle added no converter mapping row and cleared none; the
  refused set is unchanged at **142** records, one shape, `refused_non_done=0`
  (`token_coverage.py --check`). §8's "more than 10 distinct refused token types" escalation does
  not apply. The remainder below is live-side ingest-format usage, not a converter refusal.
- **What was built, and how it is gated.**
  `src/rules_core/sheet_rule_catalog.rs` renders a converted `SheetRule`'s `prose` with **no
  character in hand**. A `ProsePiece::Slot` whose `Expr` folds to a constant still prints its
  number; a slot standing on a character term prints **that term's own words**. The motivating
  record, end to end:

  | path | ARG, *Absorbing Inhalation* |
  |---|---|
  | ingest format | `…for up to %1 rounds…\|CASTERLEVEL` |
  | `sheet_rule::evaluate`, no character | `…for up to **0** rounds…` — a wrong number |
  | `sheet_rule_catalog::catalog_prose` | `…for up to **caster level** rounds…` — the rule's words |

  The gate is **corpus-wide over the live `data/sheet_rules/` directory, not a fixture**
  (`decisions.md §4`): every rule whose prose carries a slot no character settles is rendered
  both ways and the two must differ.
  `every_unsettled_slot_in_the_live_package_renders_as_words_not_as_the_characterless_zero`
  prints its own population and asserts on it.
- **Discoveries: four, each also a `correction` retro event (`§7`).**
  1. **Cycle 1's own "one buildable thing" claim was wrong** —
     `correction 1789093661118-at-35-e6-003-5d341d`. The renderer is necessary and not
     sufficient. Swapping `spell_catalog::serve_description` to a book-scoped
     `data/sheet_rules/` lookup at all 28 of its call sites produced **6 failures of 580
     desktop tests, three of them real losses**: CRB's `Nondetection (self only)` and 20 APG
     rows lose their description, and `reach_gate::bare_records_are_exactly_the_recorded_findings`
     reports the three `Threefold Aspect (<age>)` rows reaching their surface carrying only a
     key. The swap was **reverted, not shipped** — `workflow-instruction.md §8` lists RED→GREEN
     not preserved as non-self-healable, and a wrong number or a missing sentence on a player's
     screen is the thing this bundle exists to stop.
  2. **`inner_sea_world_guide:spell:ancestral_memory`'s converted prose carries
     `(70+CASTERLEVEL)%`** — an unconverted PCGen variable name in player-facing prose.
     `correction 1789093674266-at-35-e6-003-32a3f3`. The standing
     `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/` reads `0` **and is
     right to** — `CASTERLEVEL` is in none of its patterns. A clean gate over the wrong
     vocabulary is the `presence-gates-vs-correctness-gates` shape.
  3. **`core_rulebook:spell:teleport`'s converted prose writes one of its three `d%`
     percentile-dice notations as `d %`**, with an inserted space, which the live leak checker's
     `is_percentile_dice_notation` no longer recognises.
     `correction 1789093674392-at-35-e6-003-3300bc`.

  4. **`sheet_rule_convert`'s `rules=69344` is not the number the live side holds** —
     `correction 1789094375625-at-35-e6-003-af8026`. `corpus_loader::load_sheet_rules` builds a
     map of **68,976**, 368 fewer, and the corpus-wide gate this cycle added is what printed
     that number beside the converter's. The cause: **305 rule ids are written more than once**
     across `data/sheet_rules/`, every one a `#natural<N>` sibling suffix
     (`inner_sea_world_guide:monster:treerazer#natural0` appears three times), so the
     converter's sibling suffix collides for a record with several natural attacks at the same
     index and `insert_rule` keeps the last. Nothing reported it: the converter counts objects
     written, the loader counts distinct ids, and no gate compared the two. **A receipt quoting
     `rules=69344` as the live rule population is quoting the wrong denominator.**

  Neither `token-coverage.json` nor the atlas could have predicted any of the four: both
  measure corpus **units**, and all four are properties of converted **prose strings**, of the
  join between a compiled table's keys and the converted record ids, and of the converter's own
  id space. Both instruments are at
  `verdict=PASS` with `non_done=0`.
- **The remainder — 12 files, 259 hits, unchanged.**
  `deferral 1789093763760-at-35-e6-003-8e7666`. By pattern, re-derived at HEAD, not quoted:
  `DESC:`=81, `render_pcgen_desc`=50, `PRE[A-Z]+:`=46, `BONUS:`=31, `raw_tokens`=28,
  `raw_bonus_chains`=10, `%CHOICE`=5, `TYPE=`=5, `%LIST`=3. By file: `companion_catalog.rs`=52,
  `race_trait_picker.rs`=33, `feat_catalog.rs`=30, `intelligent_item_catalog.rs`=28,
  `equipment_catalog.rs`=25, `raceCreationCoverage.test.ts`=21, `monster_catalog.rs`=16,
  `reference_library_catalog.rs`=15, `class_feature_feat_bridge.rs`=11, `spell_catalog.rs`=10,
  `class_feature_descriptions.rs`=10, `companion_pool_catalog.rs`=8.
- **What blocks it now, named as three mechanisms rather than one.**
  1. **Converted-row coverage.** The compiled tables carry one row per printed variant; the
     converter writes one record per record. Spells resolve 1,194 of 1,198 table keys; equipment
     2,727 of 3,446 (the 719 sit under `equipment_modifier/`, a **kind** mismatch, not an absent
     record); feats 673 of 673. **No live-side matcher may close this** — this repo's own
     standing rule, in `feat_catalog`'s words, is that a shared name never implies a shared
     thing. The fix is `src/pcgen_import/`, which is where `decisions.md §11` puts it.
  2. **The two converted-prose defects above.**
  3. **`race_trait` `BONUS:STAT` converts to no `SheetRule.target`.** Only **6 of 217**
     `core_rulebook` `race_trait` records carry a `target`, and all six are the standalone
     `2_<ability>` rows; `core_rulebook:race_trait:dwarf_ability_scores` carries `+2
     Constitution, +2 Wisdom, -2 Charisma` **in its label only**. So
     `apps/desktop/src/characterHub/raceCreationCoverage.test.ts` cannot move to the converted
     package today without **deleting** its ability-adjustment assertion — the one thing it was
     written to protect, after a hand transcription read `BONUS:STAT|CON,WIS|2` only up to the
     comma for months. Moving it would be a gate weakening dressed as a residue win.
- **Figures + their re-derive commands:** every row carries its own command.
  The unit denominator where one applies is the whole corpus, all books — **49,438** (`jq '.units | length' docs/work-inventory.json`).

  | figure | value | command | denominator |
  |---|---|---|---|
  | `apps/desktop` residue, files / hits | **12 / 259**, flat | `python3 scripts/pcgen_residue_gate.py --check`, `root apps/desktop` line | 310 live `apps/desktop` source files the gate scans |
  | live PCGen files / hits | **208 / 11,673**, flat | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | `root src/rules_core` files / hits | **196 / 11,414**, flat | same command, `root src/rules_core` line | 196 files |
  | identifier files / hits | **17 / 144**, flat | same command, `identifier_files=` line | as above |
  | converted rules in the package | **68,976** | `cargo test --locked --lib sheet_rule_catalog -j 6 -- --nocapture` | 49,438 units → 68,976 rules (a record may carry siblings) |
  | rules whose prose carries an unsettled slot | **6,540** | `cargo test --locked --lib sheet_rule_catalog -j 6 -- --nocapture` | 68,976 rules |
  | of those, rendered differently with and without a character | **6,540 of 6,540 = 100 %** | `cargo test --locked --lib sheet_rule_catalog -j 6 -- --nocapture` | 6,540 rules |
  | spell table keys resolving a converted record | **1,194 of 1,198 = 99.7 %** | `python3 - < AT-35-E6-003_cycle2_converted-row-coverage.md`'s §2 block, `report('src/rules_core/rules_tables/*/spell_list.rs','spell')` | 1,198 table keys |
  | feat table keys resolving a converted record | **673 of 673 = 100 %** | `python3 - < AT-35-E6-003_cycle2_converted-row-coverage.md`'s §2 block, `report('src/rules_core/rules_tables/*/feat_data/*.rs','feat')` | 673 table keys |
  | equipment table keys resolving a converted record | **2,727 of 3,446 = 79.1 %** | `python3 - < AT-35-E6-003_cycle2_converted-row-coverage.md`'s §2 block, `report('src/rules_core/rules_tables/*/equipment_data/*.rs','equipment')` | 3,446 table keys |
  | rule objects written to disk | **69,344** | `python3 -c "import json,os;print(sum(len(json.load(open(os.path.join(r,f)))) for r,_,fs in os.walk('data/sheet_rules') for f in fs if f.endswith('.json') and not f.startswith('_')))"` | 49,296 record files |
  | rule ids written more than once | **305**, dropping **368 of 69,344 = 0.53 %** objects at load | `python3 - < AT-35-E6-003_cycle2_converted-row-coverage.md`'s §5 block (a `collections.Counter` over every written rule's `id`) | 69,344 rule objects |
  | `core_rulebook` `race_trait` records carrying a `target` | **6 of 217 = 2.8 %** | `python3 - < AT-35-E6-003_cycle2_converted-row-coverage.md`'s §4 block over `data/sheet_rules/core_rulebook/race_trait` | 217 records |
  | desktop tests failing on the reverted swap | **6 of 580** | `cd apps/desktop/src-tauri && cargo test --locked -j 6`, with the swap applied | 580 desktop tests |
  | files changed | **5** (3 modified, 2 added) | `git show --stat 1143318c92 \| tail -1` | files in that commit |
  | rust lines changed | **517** | `python3 scripts/cycle_scope_gate.py --receipt --since 99174263c1 …` | the 3 `.rs` files in that window |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0`; `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=96 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 96 bundle docs |
  | PI sweep | `RESULT: PASS`, 1 stage | `scripts/verify.sh --only pi-sweep` | 137 generated table files |
  | test binaries linked | **413** | `grep -c '^  Executable' /tmp/e6003c2/norun.log` | 414 test targets |
  | `--no-run` errors / warnings | **0** | `grep -cE '^(error\|warning)' /tmp/e6003c2/norun.log` | that log |
  | library suite | ``ok. 3296 passed; 0 failed; 15 ignored` (44.82 s), `LIB_EXIT=0` — **+11 exactly, the new module's tests**` | `cargo test --locked --lib -j 6` | the library tests |
  | workspace suite, complete | `**`FULL_EXIT=0`, 414 `test result` lines, 8,807 passed, 0 failed, 68 ignored, 0 FAILED suites** (totals by `awk` over the `test result` lines, not `grep -o`, per `AGENTS.md` §Concurrency; `grep -c '^test result: FAILED'` → 0) — +11 on cycle 1's 8,796, exactly this cycle's new tests` | `cargo test --locked --no-fail-fast -j 6` | 414 test targets |
  | workspace clippy | `**exit 0, 0 warnings, 0 errors** (`grep -cE '^(error|warning)' /tmp/e6003c2/clippy.log` → 0), own target dir `/tmp/cargo-sd35-AT-35-E6-003-clippy`` | `cargo clippy --locked --tests -j 6` (own target dir) | root-workspace targets |
  | sheet-rule package | ``records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (114.2s)` — identical to cycle 1 on every field` | `cargo run --locked --bin sheet_rule_convert -- --check` | 49,438 units |
- **Build scope verified**, all at `1143318c92`, `CARGO_INCREMENTAL=0`, `-j 6`, target dir
  `/tmp/cargo-sd35-AT-35-E6-003`:
  - `cargo test --locked --no-run -j 6` → **`NO_RUN_EXIT=0`, 413 `Executable` lines, 0 errors,
    0 warnings** — unchanged from cycles 1–6 of this epic, as a cycle that adds one library
    module and no test target must leave it.
  - `cargo test --locked --lib -j 6` → ``ok. 3296 passed; 0 failed; 15 ignored` (44.82 s), `LIB_EXIT=0` — **+11 exactly, the new module's tests**`.
  - `cargo test --locked --no-fail-fast -j 6` → `**`FULL_EXIT=0`, 414 `test result` lines, 8,807 passed, 0 failed, 68 ignored, 0 FAILED suites** (totals by `awk` over the `test result` lines, not `grep -o`, per `AGENTS.md` §Concurrency; `grep -c '^test result: FAILED'` → 0) — +11 on cycle 1's 8,796, exactly this cycle's new tests`. Run because `src/`
    changed.
  - `cargo clippy --locked --tests -j 6` → `**exit 0, 0 warnings, 0 errors** (`grep -cE '^(error|warning)' /tmp/e6003c2/clippy.log` → 0), own target dir `/tmp/cargo-sd35-AT-35-E6-003-clippy``.
  - `cargo run --locked --bin sheet_rule_convert -- --check` → ``records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (114.2s)` — identical to cycle 1 on every field`.
    `git status --porcelain -- data/` empty afterwards.
  - **Desktop crate and frontend at EPIC CADENCE** (`decisions.md §3`): `apps/` is untouched in
    the committed tree (`git status --porcelain -- apps/` empty; `git show --stat 1143318c92`
    lists no `apps/` path). The desktop suite *was* run mid-cycle, against the reverted
    experimental swap, and its output is the evidence for Discovery 1 — reported above as a
    measurement, not claimed as this cycle's verification.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`, 1 stage, `RETRO_ACTOR` exported in the
    same shell invocation.
- **Sweep population:** N/A — no corpus record changed (`git status --porcelain -- data/` empty
  at every checkpoint), so `corpus_literal_sweep` was correctly not run (`§6` step 3's guard),
  and with it `v06_work_inventory`, which rebuilds its verification stamps from that sweep's
  report and would refuse to write without it (`--allow-stamp-loss` is forbidden).
  `git status --porcelain -- docs/work-inventory.json` is empty and the `--receipt` rows above
  were computed against the file on disk.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial.** The criterion's Evidence sentence stands where cycle 1 left it —
  **12 files / 259 hits** under `apps/desktop/`. What moved is the **blocker**: cycle 1 named
  one, this cycle built it and proved there are three, each with a command, a count and a
  denominator. `pcgen_live_files` did not rise (`208`, flat), which `§8` makes the one
  non-negotiable direction.
- **Notes:**
  - **A cycle that ships nothing on its headline axis is a worse outcome than one that ships a
    regression only if the regression is invisible.** The swap was written, measured and
    reverted inside this turn; the alternative was 24 spell rows losing their text and three
    reaching a player's screen bare. `AGENTS.md` rule 2 and `§8`'s RED→GREEN line both point the
    same way.
  - **`git status --porcelain` is non-empty for every cycle on `tranche/15`** because the shared
    checkout carries an untracked, un-gitignored `.worktrees/` directory holding another
    session's live git worktree. AT-35-E6-002 cycle 3 filed
    `incident 1789079245735-at-35-e6-002-507e75`, key `untracked-worktrees-dir-on-shared-checkout`;
    unchanged, still needs a one-line `.gitignore` ruling.
  - **What this cycle's proof does not cover** (`AGENTS.md` rule 7). It proves that every one of
    the 6,540 converted rules carrying an unsettled slot renders as words rather than as the
    characterless number. It proves **nothing** about whether those words are the *right* words
    for every `Expr` variant — the describer is `level_up_option_filter`'s, already shipped and
    already tested, but its vocabulary was written for refusal lines, not for mid-sentence
    substitution, and only `Absorbing Inhalation` has been read end to end by eye. It proves
    nothing about the nine call sites, which were deliberately not changed. And the row-coverage
    figures are **table-key** coverage, not served-row coverage: `spell_catalog` serves from
    `spell_resolver::spell_catalog_rows()`, a wider roster than the four `spell_list.rs` tables,
    which is why the desktop run found 20 APG losses where the table measurement predicted 4.
- **Next-cycle scope:** **AT-35-E6-003 cycle 3** — the converter side first: emit the variant
  rows the compiled tables carry (or put the converted record id on the table rows), fix the two
  converted-prose defects, and convert `race_trait` `BONUS:STAT` to a `SheetRule.target`. Then
  the nine `render_pcgen_desc` call sites, **one catalog at a time, each with its own before/after
  served-row count** so a loss cannot ship silently. Scope flags: `SCOPE_GATE: EXEMPT (Epic 6
  cycle — closes zero units by design, decisions.md §2)`. Target: `root apps/desktop files=0
  hits=0`.
