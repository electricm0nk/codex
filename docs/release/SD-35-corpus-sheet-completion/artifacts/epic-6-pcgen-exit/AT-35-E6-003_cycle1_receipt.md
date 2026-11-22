# Cycle 1 — Epic 6 (PCGen exit) / AT-35-E6-003

Thirty-nine of the fifty-one `apps/desktop/` files the residue gate counted leave PCGen behind.
The twelve that remain are exactly the twelve whose **code** still reads a PCGen token; rewriting
their prose first would have moved the grep without moving the fact.

- **Commit SHA:** `b4b11d362b` — the 39 rewritten `apps/desktop/` files, the provenance artifact
  and this cycle's four retro events. Cycle start `481bfca01e`. A later commit carries this
  receipt and the `progress.md` / `kanban.md` rows (a receipt cannot name the commit that carries
  it).
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
- **Files touched:** **42** (`git show --stat b4b11d362b | tail -1` → `42 files changed, 608
  insertions(+), 198 deletions(-)` (the provenance artifact and the retro log are 411 of the
  insertions; the 39 rewritten source files are 197 changed lines, +197/−197)) — 41 modified, 1 added, 0 renamed, 0 deleted.
  - **`apps/desktop/` source, 39 files** — 9 under `src-tauri/src/` (`browser_handoff.rs`,
    `character_hub.rs`, `class_catalog_generic.rs`, `corpus_ingest_diagnostic.rs`, `main.rs`,
    `pf1_adapter.rs`, `race_catalog.rs`, `reach_gate.rs`, `trait_picker.rs`) and 30 under
    `src/` (the 8 `boundary/load*.ts`, 15 `characterHub/`, 3 `companionCatalog/`, 2
    `equipmentCatalog/`, 2 `monsterCatalog/`, 1 `raceCatalog/`).
  - **Artifact, 1 file (added):**
    `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle1_desktop-prose-provenance.md`
    — the before/after table for all **197 changed lines across 39 files**, preserving every
    original wording verbatim on the documentation side, where the gate does not scan.
  - **Retro, 1 file (added):** `docs/retro/events/at-35-e6-003.jsonl`.
  - **Folded working-tree append, 1 file:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the
    `derived_at` re-stamp `completion_atlas.py --check` writes; diff is that one line).
  - **Zero `src/` files changed. Zero `scripts/` files changed. Zero `data/` files changed.
    Zero `tests/` files changed.** `git status --porcelain -- src/ scripts/ data/ tests/` empty
    at every checkpoint; `git show --stat b4b11d362b` lists none.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS for this cycle; 23 matches against the
  develop base, attributed, pre-existing and not this cycle's.**
  ```
  CS=481bfca01e53798bee83cdff9b6df44630cd85ef
  SC="src/rules_core src/pcgen_import src/bin src/oracle_validation apps/desktop/src-tauri/src apps/desktop/src"
  git diff --unified=0 $CS -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  (no output)  OK_NO_BUNDLE_TAGS
  BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47   # git merge-base HEAD origin/develop
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  23
  ```
  The base-window figure is larger than earlier Epic 6 receipts' because this cycle's scoped-path
  set **adds `apps/desktop/src/`**, which carries the `SD31-D7-PROSE-003` / `SD31-W29-` /
  `SD31-W15-` wave tags the desktop lane has always used in its own prose. This cycle's
  contribution is **0**: none of the 197 rewritten lines introduces a tag.
- **Wired-integration audit result:** **OK_NO_TOKENS for this cycle; 25 matches against the
  develop base, all attributed, none a stub marker.**
  ```
  git diff --unified=0 $CS -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
  ->  (no output)  OK_NO_TOKENS
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $SC … | grep -cE '…'  ->  25
  ```
  The 25 are the **domain word** `placeholder` naming PCGen's own `SOURCEPAGE:p.xx`
  non-citation (`decisions.md §26`, §27.2), its `%LIST` counterpart, the `.MOD` "no selection"
  rows, and the desktop lane's own "placeholder chooser wrapper" refusal — recorded and
  attributed by AT-35-E6-001/002 cycles 1–6. Text unchanged, count unchanged.
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  | clause | at HEAD | met? |
  |---|---|---|
  | desktop `raw_tokens` readers read `SheetRule.applies`/`prose` | 5 files still read the array (`class_feature_feat_bridge.rs`, `reference_library_catalog.rs`, `intelligent_item_catalog.rs`, `race_trait_picker.rs`, `raceCreationCoverage.test.ts`) | **no** |
  | `render_pcgen_desc` deleted from the live side | 14 live files / 106 hits (9 under `apps/desktop/`, 5 under `src/rules_core/`) | **no** |
  | **zero hits under `apps/desktop/`** | **12 files / 259 hits** (from 51 / 477) | **no — 46 % of the hits and 76 % of the files closed** |
  | desktop crate suite green | `575 passed; 0 failed; 0 ignored` | **yes** |
  | frontend suite green | `101/101 test files passed`; `tsc --noEmit` exit 0 | **yes** |
  | the 19 on-screen tests still pass | `rulesAndFeaturesSection: 19 per-kind tests + 5 section tests passed` | **yes** |

  Three of six clauses met. **Status `partial`**, remainder named below.
- **Receipt rows (mechanical):**
  ```
  since=481bfca01e53798bee83cdff9b6df44630cd85ef target_dir=/tmp/cargo-sd35-AT-35-E6-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=238 ratio=n/a builds_recorded=1 pcgen_live_files=208
  ```
  `closed=0` / `relabeled=0` is correct: the unit population was already `0` non-DONE at cycle
  start. `ratio` is `n/a`, a division by zero, never `0.0`. `pcgen_live_files` **247 → 208**, the
  largest single-cycle fall of the epic (−39, exactly the 39 files). `rust_lines_changed=238`
  counts only the 9 `.rs` files; the 30 `.ts`/`.tsx` files the instrument does not count carry
  the other 104 changed lines (197 total, `git diff --numstat`).
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, at `b4b11d362b`):
  ```
  pattern raw_tokens files=5 hits=28
  pattern raw_bonus_chains files=2 hits=10
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=14 hits=106
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=120 hits=2096
  pattern DEFINE: files=24 hits=114
  pattern PRE[A-Z]+: files=113 hits=7908
  pattern SAB: files=0 hits=0
  pattern DESC: files=113 hits=502
  pattern %CHOICE files=7 hits=65
  pattern %LIST files=16 hits=123
  pattern TYPE= files=62 hits=721
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=12 hits=259
  identifier_files=17 identifier_hits=144
  live_files=208 live_hits=11673 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  **`verdict=PASS`, and every axis fell or held.** `live_files` **247 → 208** (−39),
  `live_hits` **11,891 → 11,673** (−218), `root apps/desktop` **51 files / 477 hits → 12 / 259**,
  `identifier_files` **22 → 17**, `identifier_hits` **150 → 144**. `root src/rules_core` is
  **flat at 196 / 11,414** — this cycle changed no file under `src/`, which is the check that
  the fall is entirely `apps/desktop/`'s. The baseline stays at cycle 1's `260 / 12,736`;
  `--rebaseline` is `AT-35-E6-004`'s step, and leaving a higher ratchet in place can only be
  stricter.
- **Oracle parity:** **N/A for this cycle.** No `Number` mapping was added and no computed value
  moved. Epic 6 touches a live path, so the row is owed an answer: **195 of the 197 changed lines
  are comment or doc-comment text**, one is a three-word caption suffix and one is a
  `.or_else(|closure|)` → `.or(value)` clippy fix over a `Copy` field (no evaluation-order
  change is observable: the argument is a plain `Option<i16>` field read). No computed number in
  the repo moved, and the full workspace suite is **identical to AT-35-E6-002 cycle 6's on every
  field** (414 targets, 8,796 passed, 0 failed, 68 ignored).
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the unit population was already 0 non-DONE at cycle
    start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** 0. This cycle evaluated nothing and converted nothing.
  - **instrument-correction:** 0. `scripts/pcgen_residue_gate.py` was **not touched**
    (`git status --porcelain -- scripts/` empty), so the −39 files / −218 hits is entirely code,
    not a moved measuring stick.
- **Refused tokens:** **none.** This cycle added no converter mapping row and cleared none; the
  refused set is unchanged at **142** records, one shape, `refused_non_done=0`
  (`token_coverage.py --check`). §8's "more than 10 distinct refused token types" escalation does
  not apply.
- **What the mechanism was, and why it is not laundering.**
  The rule this cycle applied, stated before the first edit and applied without exception: **a
  file is rewritten only when the code beside the prose reads no PCGen token.** That is the
  instrument's own ruling read forward rather than around —
  `scripts/pcgen_residue_gate.py`'s module docstring (lines 36–40): *"A mention inside a comment
  or a doc string counts — the ruling is 'nothing left of pcgen', and a comment explaining a
  PCGen token on the live side is a sign the code next to it still needs one."* Where the code no
  longer needs one, the comment is residue and closes; where it still does, the comment is
  documentation and stays. Partitioning the 51 files on that predicate gave **39 closable, 12
  not**, and the 12 are precisely the files carrying a live read.
  Two controls keep the 39 honest: every original wording is preserved verbatim in
  `AT-35-E6-003_cycle1_desktop-prose-provenance.md`, and the rewording names the **fact** (a
  saving-throw bonus, an ability-score adjustment, a description slot) rather than deleting it —
  so a reader chasing a provenance claim still lands on the same corpus row.
- **Two shipped defects, found by the audit, both fixed here.**
  1. **The Companion Catalog printed PCGen token names on screen.** Three exported captions read
     `'Ability score adjustments (corpus BONUS:STAT tokens)'`,
     `'Extra damage on attack (corpus BONUS:WEAPONPROF DAMAGE tokens)'` and
     `'Skill bonus from ability difference (corpus BONUS:SKILL tokens)'`. `decisions.md §1` says
     the sheet prints one final number, dice in final form, or **the rule's words** — never the
     source format's syntax. They now read `'(as the corpus states them)'` /
     `'(as the corpus states it)'`, and the two assertions that pinned the old substrings follow
     (a count assertion this cycle's own deliberate change moved, `§8` self-healable).
     `correction 1789090265986-at-35-e6-003-8651f6`.
  2. **A frontend fixture pinned a message no live path produces.**
     `featPickerEligibility.test.ts` asserted an unverified-note of the shape
     *"…(references a PCGen runtime variable this engine does not model) (PREMULT:1,[PREVARGTEQ:
     PreStatScore_INT,13],…)"*. `src/rules_core/feat_prereqs/converted_gate.rs:117` builds that
     note as `"not verified: {reason} ({words})"` from `describe_gate`'s **rule words**, and the
     PCGen wording survives only in `src/pcgen_import/pre_tokens.rs:945` — tool side, never
     served. The fixture was stale prose pinning a dead shape. Replaced with a current-shape
     message using a real `unverifiable_reason` string.
     `correction 1789090266113-at-35-e6-003-b767bb`.
- **Two of the cleared hits were the gate's own false positives, not PCGen.**
  `\bPRE[A-Z]+:` matches Rust type ascription on any identifier that begins a line-initial token
  `PRE…` — `const PREFIX: &str = "https://github.com/"` in `browser_handoff.rs` is a GitHub URL
  prefix with nothing to do with PCGen prerequisites. Renamed `GITHUB_URL_PREFIX` (the `_B`
  boundary stops the match), which is a better name anyway. Recorded here rather than as a gate
  change: **1 hit of 477 is not a reason to loosen a ratchet**, and a looser pattern would stop
  catching real `PREABILITY:`/`PREVARGTEQ:` text. The identically-shaped non-matches
  (`CHAIN_SHIRT_ARMOR_BONUS: i16`, `DEXTERITY_BONUS: i16`) were checked and do **not** match —
  `_B` is not a word boundary — so the class is small and already bounded.
- **One pre-existing warning fixed.** `cargo clippy --locked --tests` on the **desktop crate**
  (a separate cargo workspace, and not what the root-workspace clippy runs of cycles 1–6 covered)
  carried one warning at cycle start: `unnecessary_lazy_evaluations` at
  `race_catalog.rs:817`. Verified pre-existing (`git show HEAD:…` identical) and fixed in the same
  cycle, per `§6` step 3. The desktop crate's clippy is now **0 warnings**.
- **Discoveries:** **three**, each emitted as a `correction` retro event as `§7` requires — the
  two shipped defects above, and a self-correction of this cycle's own remainder table
  (`correction 1789090303989-at-35-e6-003-3b1252`): the per-pattern figures in
  `deferral 1789090281485-at-35-e6-003-c441c8` were typed from memory and five of nine were
  wrong (the total, 259, was right). Re-derived and corrected in the same cycle; the table below
  is the derived one. Neither `token-coverage.json` nor the atlas could have predicted any of the
  three: both measure corpus units, and all three are hand-written strings. No token type, kind,
  or remaining-step category surfaced — both instruments are at `verdict=PASS` with `non_done=0`.
- **The remainder — 12 files, 259 hits, every one a live PCGen read.**
  `deferral 1789090281485-at-35-e6-003-c441c8`, per-pattern figures corrected by
  `1789090303989-at-35-e6-003-3b1252`.

  | file | hits | why it could not close this cycle |
  |---|---:|---|
  | `src-tauri/src/companion_catalog.rs` | 52 | `render_pcgen_desc` ×2 **and** `serve_desc_condition`, a runtime PRE-token translator |
  | `src-tauri/src/race_trait_picker.rs` | 33 | 5 production `record.data.raw_tokens` reads (`ABILITY`, `PREMULT`, `PREABILITY`, `!PREFACT`) driving the alternate-trait exclusion guards |
  | `src-tauri/src/feat_catalog.rs` | 30 | `render_pcgen_desc` ×2 on the served feat description |
  | `src-tauri/src/intelligent_item_catalog.rs` | 28 | production `raw_tokens` + `raw_bonus_chains` reads and a runtime `PREALIGN:`/`PREVAR*:`/`TYPE=` parser |
  | `src-tauri/src/equipment_catalog.rs` | 25 | `render_pcgen_desc` on the served equipment description |
  | `src-tauri/src/monster_catalog.rs` | 16 | `render_pcgen_desc` on the served monster description |
  | `src-tauri/src/reference_library_catalog.rs` | 15 | 2 production `raw_tokens` reads **and** `render_pcgen_desc` |
  | `src-tauri/src/class_feature_feat_bridge.rs` | 11 | `sole_feat_grant_target(&data["raw_tokens"])`, the whole module's predicate |
  | `src-tauri/src/spell_catalog.rs` | 10 | `render_pcgen_desc` on the served spell description |
  | `src-tauri/src/class_feature_descriptions.rs` | 10 | `render_pcgen_desc` ×2 |
  | `src-tauri/src/companion_pool_catalog.rs` | 8 | `render_pcgen_desc` + `leaked_pcgen_syntax` |
  | `src/characterHub/raceCreationCoverage.test.ts` | 21 | a frontend coverage test reading `data/corpus/**` `raw_tokens`/`raw_bonus_chains` directly |
  | **total** | **259** | 12 files |

  By pattern (re-derived, not quoted): `DESC:`=81, `render_pcgen_desc`=50, `PRE[A-Z]+:`=46,
  `BONUS:`=31, `raw_tokens`=28, `raw_bonus_chains`=10, `%CHOICE`=5, `TYPE=`=5, `%LIST`=3.
- **What blocks the remainder, named as a mechanism, not as difficulty.**
  Nine of the twelve need the **same one thing**: a catalog-mode prose path. The converter
  already did the `%N` substitution the criterion's second sentence names —
  `data/sheet_rules/advanced_race_guide/spell/absorbing_inhalation.json` carries the ARG spell
  whose raw row reads *"for up to %1 rounds|CASTERLEVEL"* as clean prose with a typed
  `Slot(CasterLevel(Holder))` — so the text exists. What does not exist is a way to render it
  **with no character**: `sheet_rule::evaluate` prints an unresolved `Slot` as its numeric value,
  which on a catalog screen with no character is `0` — *"for up to 0 rounds"*, a wrong number
  where the sheet rule requires either a real number or the rule's words
  (`presence-gates-vs-correctness-gates`: a wrong computed number looks right). Cycle 2's first
  job is that renderer — slots as words when no character is in hand — after which the nine
  `render_pcgen_desc` call sites become a lookup. The other three need
  `SheetRule.applies`/`grants` shapes for exclusion guards and feat-grant bridges.
- **Figures + their re-derive commands:** every row carries its own command.
  The unit denominator where one applies is the whole corpus, all books — **49,438** (`jq '.units | length' docs/work-inventory.json`).

  | figure | value | command | denominator |
  |---|---|---|---|
  | `apps/desktop` residue, files / hits | **12 / 259** (from 51 / 477) | `python3 scripts/pcgen_residue_gate.py --check`, `root apps/desktop` line | 310 live `apps/desktop` source files the gate scans |
  | live PCGen files / hits | **208 / 11,673** (from 247 / 11,891) | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | identifier files / hits | **17 / 144** (from 22 / 150) | same command, `identifier_files=` line | as above |
  | `root src/rules_core` files / hits | **196 / 11,414**, unchanged | same command, `root src/rules_core` line | 196 files |
  | remainder by pattern | `DESC:`=81, `render_pcgen_desc`=50, `PRE[A-Z]+:`=46, `BONUS:`=31, `raw_tokens`=28, `raw_bonus_chains`=10, `%CHOICE`=5, `TYPE=`=5, `%LIST`=3 | `python3` over `pcgen_residue_gate._iter_live_source_files('.')` filtered to `apps/desktop`, summing `len(rx.findall(text))` per pattern | 259 hits |
  | files rewritten / lines rewritten | **39 / 197** | `python3` diff of `git show HEAD:<f>` against each file; artifact row count | 51 files at cycle start |
  | gate patterns | **14**, unchanged — the script was not touched | `git status --porcelain -- scripts/` (empty); `python3 -c "import sys;sys.path.insert(0,'scripts');import pcgen_residue_gate as g;print(len(g.PATTERNS))"` | 1 instrument |
  | files changed | **42** (41 modified, 1 added) | `git show --stat b4b11d362b \| tail -1` | files in that commit |
  | rust lines changed | **238** | `python3 scripts/cycle_scope_gate.py --receipt --since 481bfca01e …` | the 9 `.rs` files in that window |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0`; `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=94 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 94 bundle docs |
  | sheet-rule package | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (112.1s)` | `cargo run --locked --bin sheet_rule_convert -- --check` | 49,438 units |
  | PI sweep | `RESULT: PASS`, 1 stage | `scripts/verify.sh --only pi-sweep` | 137 generated table files |
  | library suite | `3285 passed; 0 failed; 15 ignored` | `cargo test --locked --lib -j 6` | 3,300 library tests |
  | test binaries linked | **413** | `grep -c '^  Executable' /tmp/e6003/norun.log` | 414 test targets |
  | workspace suite, complete | `414` targets, `8796 passed, 0 failed, 68 ignored`, `0` FAILED suites | `cargo test --locked --no-fail-fast -j 6`; `grep -c '^test result' /tmp/e6003/full.log`; `awk '/^test result/{…}'` | 414 test targets |
  | desktop crate suite | `575 passed; 0 failed; 0 ignored` (91 s) | `cd apps/desktop/src-tauri && cargo test --locked -j 6` | 575 desktop tests |
  | frontend suite | `101/101 test files passed` | `cd apps/desktop && npm test` | 101 frontend test files |
  | the 19 on-screen tests | `19 per-kind tests + 5 section tests passed` | `cd apps/desktop && npx tsx src/characterHub/rulesAndFeaturesSection.test.ts` | 19 kinds |
  | frontend typecheck | exit 0 | `cd apps/desktop && npm run typecheck` | whole frontend |
  | desktop clippy | **0 warnings, 0 errors** (from 1 warning) | `cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 4`; `grep -cE '^(error\|warning)' /tmp/e6003/clippy2.log` | desktop crate targets |
- **Build scope verified**, all at `b4b11d362b`, `CARGO_INCREMENTAL=0`:
  - `cargo test --locked --no-run -j 6` (target dir `/tmp/cargo-sd35-AT-35-E6-003`) →
    `NO_RUN_EXIT=0`, **413 `Executable` lines** — unchanged from AT-35-E6-001/002 cycles 1–6, as
    a cycle that changes no root-workspace file must leave it.
  - `cargo test --locked --lib -j 6` → `ok. 3285 passed; 0 failed; 15 ignored` (41.00 s),
    `LIB_EXIT=0` — **identical to AT-35-E6-002 cycle 6 on every field.**
  - `cargo test --locked --no-fail-fast -j 6` → **`FULL_EXIT=0`, 414 `test result` lines,
    8,796 passed, 0 failed, 68 ignored, 0 FAILED suites** (totals by `awk` over the `test result`
    lines, not `grep -o`, per `AGENTS.md` §Concurrency; `grep -c '^test result: FAILED'` → 0).
    **It finished, and every field is identical to cycle 6's complete run.** Run because §6's
    battery lists it, not because it could have moved: zero root-workspace files changed.
  - **Desktop crate and frontend ran HERE** — `apps/` is this cycle's entire surface:
    `cd apps/desktop/src-tauri && cargo test --locked -j 6` → `575 passed; 0 failed; 0 ignored`
    (91.47 s, `DESKTOP_EXIT=0`), re-run after the clippy fix with the identical result;
    `cd apps/desktop && npm test` → `101/101 test files passed`; `npm run typecheck` → exit 0;
    `npx tsx src/characterHub/rulesAndFeaturesSection.test.ts` → `19 per-kind tests + 5 section
    tests passed`.
  - `cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 4` → **exit 0, 0 warnings,
    0 errors**, own target dir `/tmp/cargo-sd35-AT-35-E6-003-clippy`.
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (112.1s)`
    — identical to cycle 6 on every field. `git status --porcelain -- data/` empty afterwards.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`, `RETRO_ACTOR` exported in the same
    shell invocation.
- **Sweep population:** N/A — no corpus record changed (`git status --porcelain -- data/` empty
  at every checkpoint), so `corpus_literal_sweep` was correctly not run (`§6` step 3's guard), and
  with it `v06_work_inventory`, which rebuilds its verification stamps from that sweep's report
  and would refuse to write without it (`--allow-stamp-loss` is forbidden).
  `git status --porcelain -- docs/work-inventory.json` is empty and the `--receipt` rows above
  were computed against the file on disk.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial.** Three of the criterion's six clauses are met at HEAD (desktop crate
  green, frontend green, the 19 on-screen tests green); the Evidence sentence's first clause —
  *zero hits under `apps/desktop/`* — stands at **12 files / 259 hits**, down from 51 / 477. The
  remainder is named by file and by mechanism above, sums to 259, and is blocked on one buildable
  thing (a catalog-mode prose renderer), not on judgment.
- **Notes:**
  - **`git status --porcelain` is non-empty for every cycle on `tranche/15`** because the shared
    checkout carries an untracked, un-gitignored `.worktrees/` directory holding another
    session's live git worktree. AT-35-E6-002 cycle 3 filed
    `incident 1789079245735-at-35-e6-002-507e75`, key `untracked-worktrees-dir-on-shared-checkout`;
    unchanged, still needs a one-line `.gitignore` ruling.
  - **What this cycle's proof does not cover** (`AGENTS.md` rule 7). It proves that 39
    `apps/desktop/` files carry no PCGen mention and that the desktop crate, the frontend and the
    19 on-screen tests are green with them rewritten. It proves **nothing** about whether the
    twelve survivors' reads are correct, nor about `src/rules_core/`'s 196 files / 11,414 hits,
    which are `AT-35-E6-004`'s closure surface and are **flat** across this cycle. It also does
    not prove the rewritten prose is as *useful* as what it replaced — only that every original
    wording survives, verbatim, in the provenance artifact, so a future reader can judge.
- **Next-cycle scope:** **AT-35-E6-003 cycle 2** — build the catalog-mode prose path
  (`SheetRule.prose` rendered with no character, unresolved slots as words rather than `0`), then
  move the nine `render_pcgen_desc` call sites to it and the five `raw_tokens`/`raw_bonus_chains`
  readers to `SheetRule.applies`/`grants`. Scope flags: `SCOPE_GATE: EXEMPT (Epic 6 cycle —
  closes zero units by design, decisions.md §2)`. Target: `root apps/desktop files=0 hits=0`.
