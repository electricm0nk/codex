# Cycle 4 — Epic 6 (PCGen exit) / AT-35-E6-002

Cycle 1 delivered the criterion's **body** and its second Evidence clause. Cycles 2 and 3 took
the first Evidence clause — *zero `raw_tokens` hits under `src/rules_core/`* — from **202 → 110
→ 40**, and cycle 3 took the production reads to **0**. **This cycle takes the clause to 0**:
`grep -rho '\braw_tokens\b' --include=*.rs src/rules_core/ | wc -l` prints **0**, over **0**
files. Status is nonetheless **partial**, and for a reason cycle 3 could not have written down:
the residue gate measures **one of the ingest format's two arrays**. Its sibling,
`raw_bonus_chains`, is in no gate pattern and stands at **31 hits across 11
`src/rules_core/` files**, including a production traversal this cycle removed. That is cycle
5's named remainder — a number to close, never an exemption.

- **Commit SHA:** `b2839b631a` — the new tool-side module, the constant and the two fixture
  builders, the new `ingest_record` accessor, the 16 consumer re-pointings, and this cycle's
  `correction` + `deferral` events. Cycle start `66a6c76948`. A later commit carries this
  receipt and the `progress.md` / `kanban.md` rows (a receipt cannot name the commit that
  carries it).
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
- **Files touched:** **18** (`git show --stat b2839b631a | tail -1` → `18 files changed, 558
  insertions(+), 477 deletions(-)`) — 16 modified, 1 added, 0 renamed, 0 deleted, plus the
  folded atlas `derived_at` re-stamp.
  - **Added, 1 file (tool side):** `src/pcgen_import/ingest_payload.rs` (464 lines).
  - **Module declaration, 1 file:** `src/pcgen_import/mod.rs` (one `pub mod` with its
    `decisions.md §11` / `technical-design.md §0` citation).
  - **Tool-side accessor widened, 1 file:** `src/pcgen_import/ingest_record.rs`
    (`bonus_chain_qualifiers`).
  - **Tool-side import followed, 1 file:** `src/pcgen_import/race_trait_tokens.rs`.
  - **Live modules changed, 7 files:** `src/rules_core/{shape_b_v1, corpus_loader,
    derived_evaluator_fixture_check, pi_screening, damage_total, race_resolver,
    race_creation}.rs`.
  - **Generators' import paths followed, 4 files:** `src/bin/{ingest_races, ingest_race_traits,
    ingest_apg_race_traits, ingest_pu_classes}.rs`.
  - **Desktop, 1 file:** `apps/desktop/src-tauri/src/race_catalog.rs` (import path only).
  - **Retro, 1 file:** `docs/retro/events/at-35-e6-002.jsonl`.
  - **Folded working-tree append, 1 file:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the
    `derived_at` re-stamp `completion_atlas.py --check` writes).
  - **Zero `data/` files changed.** `git status --porcelain -- data/` is empty.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS in shipping code — **1 match, attributed, not
  a violation, and not this cycle's.**
  ```
  BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47   # git merge-base HEAD origin/develop
  CODE="src/rules_core src/pcgen_import src/bin src/oracle_validation apps/desktop/src-tauri/src"
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  +/// (`tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs`); this function is its
  ```
  The same doc-comment citation of a real test file's path in `src/rules_core/racial_sla.rs`
  that every `AT-35-E5-*` and `AT-35-E6-*` receipt has recorded — pre-existing and unchanged
  here. This cycle's own diff (`git diff --unified=0 66a6c76948..HEAD -- $CODE`) contributes
  **0**.
- **Wired-integration audit result:** OK_NO_TOKENS for this cycle's own diff. The
  `${BASE_BRANCH}...HEAD` form reports **3 pre-existing matches**, all of the domain word
  `placeholder` describing PCGen's own `%LIST` / `p.xx` "no selection" rows, none of them a
  stub marker; this cycle's own diff adds none and removes two of them along with the doc
  comments that moved to `src/pcgen_import/`.
  ```
  git diff --unified=0 66a6c76948..HEAD -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
  ->  only `-` lines (the two moved `placeholder` doc comments); no `+` line matches
  ```
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-002`):
  > `src/rules_core/cache_gen/**` and `wiring_class.rs` relocate to `src/pcgen_import/`
  > behavior-identically (they are converter code that lives on the wrong side). Every `src/bin`
  > generator's import path follows.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero `raw_tokens` hits under
  > `src/rules_core/`; `gen_book_cache` output byte-identical before and after on one book.

  | clause | at HEAD | met? |
  |---|---|---|
  | `cache_gen/**` relocated to `src/pcgen_import/` | 16 files, cycle 1 | **yes** |
  | `wiring_class.rs` relocated | `src/pcgen_import/wiring_class.rs`, cycle 1 | **yes** |
  | behaviour-identical | see **Build scope verified** | **yes** |
  | every `src/bin` generator's import path follows | cycle 1's 63 files, cycle 2's 9, cycle 3's 1, this cycle's 4, 0 residual | **yes** |
  | `gen_book_cache` byte-identical on one book | cycle 1: 2,207 records, same manifest sha256 | **yes** |
  | zero `raw_tokens` hits under `src/rules_core/` | **0 files, 0 matches** (was 5 / 40) | **yes** |
- **Receipt rows (mechanical):**
  ```
  since=66a6c7694824c6b8dac61deb8f1f0db86af68261 target_dir=/tmp/cargo-sd35-AT-35-E6-002 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1031 ratio=n/a builds_recorded=1 pcgen_live_files=247
  ```
  `closed=0` / `relabeled=0` is correct: the unit population was already `0` non-DONE at cycle
  start. `ratio` is `n/a`, a division by zero, never `0.0`. `pcgen_live_files=247` is **one
  below** cycle 3's `248` and above nothing.
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, at `b2839b631a`):
  ```
  pattern raw_tokens files=7 hits=31
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=17 hits=109
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=137 hits=2214
  pattern DEFINE: files=26 hits=116
  pattern PRE[A-Z]+: files=118 hits=7926
  pattern SAB: files=0 hits=0
  pattern DESC: files=137 hits=565
  pattern %CHOICE files=8 hits=66
  pattern %LIST files=24 hits=139
  pattern TYPE= files=67 hits=734
  root src/rules_core files=196 hits=11424
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=51 hits=476
  identifier_files=22 identifier_hits=140
  live_files=247 live_hits=11900 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  **Down on both axes and up on neither** — `248 → 247` files, `11,956 → 11,900` hits against
  cycle 3. The gate's own `raw_tokens` pattern fell **12 files / 71 hits → 7 / 31**, and the
  `src/rules_core/` share of it is **5 → 0 files, 40 → 0 hits**. The 31 that survive that
  pattern are all in `apps/desktop/` (25, `AT-35-E6-003`'s scope) and `src/bin` /
  `src/pcgen_import` (tool side, KEPT). The baseline file is deliberately NOT rebaselined;
  `--rebaseline` is `AT-35-E6-004`'s step. `pcgen_residue_gate.py` itself is untouched by this
  cycle (`git diff --stat 66a6c76948..HEAD -- scripts/pcgen_residue_gate.py` empty), so the
  fall is entirely code moving.
- **Oracle parity:** **N/A for this cycle, and correct by construction.** No `Number` mapping
  was added, so the fixture-roster oracle comparison is not triggered (`§6` step 3). Epic 6
  touches a live path, so the row is owed an answer, and this cycle's answer is stronger than
  agreement within tolerance: the relocated types are the *same types*, with the same serde
  derives, the same field order and the same `#[serde(default)]` attributes, so every on-disk
  `data/corpus/**` record deserializes and re-serializes byte-for-byte as before — the moved
  round-trip test (`real_pre_existing_equipment_json_deserializes_with_raw_token_fields_
  defaulting_to_empty`, which reads a verbatim on-disk CRB record) proves exactly that and
  still passes. `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the population was already 0 non-DONE at cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** 0. This cycle evaluated nothing and converted nothing.
  - **instrument-correction:** 0 *to a published unit figure*. The `correction` event this cycle
    filed is against an **instrument**, not a figure: `pcgen_residue_gate.py`'s pattern list.
    No previously published number changes value; a previously unmeasured population becomes
    visible. Recorded as `correction 1789082356496-at-35-e6-002-65186a`.
- **Refused tokens:** **none.** This cycle added no converter refusal and cleared none; the
  refused set is unchanged at **142** records, one shape, `refused_non_done=0`
  (`token_coverage.py --check`). It shipped no converter mapping row at all. The
  `raw_bonus_chains` remainder below is **live-side ingest-format usage, not a converter
  refusal**, so §8's "more than 10 distinct refused token types" escalation does not apply.
- **What the three mechanisms were, and why none is a rename.**
  1. **`src/pcgen_import/ingest_payload.rs` — the ingest cache's payload half leaves the live
     side (13 matches).** `shape_b_v1.rs` held two different things under one roof: the record
     *envelope* (`CorpusRecordV1<T>`, `License`, `Population`, `Completeness`, `CorpusSource`,
     the PI markers, `validate_license`) — genuinely ours, and generic over `T` — and the five
     per-content-kind `data` payloads (`EquipmentCacheData`, `RaceCacheData`,
     `RaceTraitCacheData`, `ClassVariantCacheData`, `ClassFeatureCacheData`), `ClassFeatureGrant`,
     and the two verbatim-token carriers `RawToken` / `RawBonusChain`. Every field on the second
     group is transcribed off a PCGen token by `src/pcgen_import/cache_gen/**` and
     `src/bin/ingest_*`; the group **is** the converter's output format, and it is the sole home
     of the five `pub raw_tokens: Vec<RawToken>` declarations. It moves, transcribed unchanged —
     type definitions, serde derives, field order, doc comments, and the round-trip test that
     reads a verbatim on-disk record. **This is a move, not a re-export:** `shape_b_v1.rs`
     carries no `pub use` of the moved names, so every consumer visibly imports from
     `pcgen_import::ingest_payload` and the dependency is a named cross-boundary call rather
     than a hidden one. 16 consumer files follow the import, including all four
     `src/bin/ingest_*` generators and the desktop `race_catalog`.
  2. **`INGEST_TOKENS_FIELD` + `ingest_tokens_value` + `ingest_record_json` — the wire name gets
     one definition, on the converter side (10 matches).** Seven live-side test fixtures
     hand-wrote a PCGen-shaped JSON literal to pin the on-disk record shape
     (`derived_evaluator_fixture_check.rs` × 6 scratch-corpus records,
     `corpus_loader.rs` × 1 `serde_json::json!` value), and `pi_screening.rs`'s
     screened-field-list assertion hard-coded the field name in a string. All eight now cite the
     constant or call the builder. This is not indirection for a grep: the ingest format's field
     name previously had **eight** independent spellings inside a live module and now has one,
     on the side that writes it.
  3. **`ingest_record::bonus_chain_qualifiers` — a production read cycle 2 could not see
     (1 line, and the discovery below).** `corpus_loader::equipment_record_from_json` open-coded
     `data.get("raw_bonus_chains").and_then(Value::as_array)` and re-implemented the qualifier
     walk inline — exactly the defect cycle 2 built `ingest_record` to remove for the token
     array, invisible to all three prior cycles because the gate's pattern names `raw_tokens`
     and not its sibling. The traversal moved verbatim (same skip-on-malformed behaviour) and is
     covered by the two existing live-corpus tests that exercise both branches: `Companion Stone
     (Diplomacy)`'s real `BONUS:SKILL|Diplomacy|4|TYPE=Competence` chain (non-empty) and
     `Arrow (Slaying)` (empty).
- **On comment rewrites, under cycle 2's own rule.** Cycle 2's rule was: *no comment is reworded
  in any file that still carries a code read.* Comments were reworded in `corpus_loader.rs` (11),
  `pi_screening.rs` (2), `derived_evaluator_fixture_check.rs` (1) and `damage_total.rs` (1) — in
  every one of them the file's code half was reduced to zero **first**, in the same commit, by
  mechanisms 1–3. The rewrites are not euphemism: each now names the ingest token array *and*
  points at the converter-side module that owns it, which is more information than the bare
  field name carried. `damage_total.rs`'s runnable `python3 -c` re-derive command — deliberately
  left by cycles 2 and 3 under `AGENTS.md` §9 — is **kept runnable** and now derives the field
  name from its one definition instead of spelling it out. Executed, not assumed:
  ```
  TOK=$(grep -oP '(?<=INGEST_TOKENS_FIELD: &str = ")[^"]+' src/pcgen_import/ingest_payload.rs)
  jq --arg t "$TOK" '.data[$t][] | select(.key=="DAMAGE")' \
    data/corpus/core_rulebook/equipment/arms_armor/light_crossbow_base.json
  ->  { "key": "DAMAGE", "value": "1d8" }
  ```
- **The remainder, enumerated — and it is a different array, not a leftover.** `raw_tokens`
  under `src/rules_core/` is **0 files / 0 matches**. What this cycle discovered, and defers
  with a name, is the ingest format's **other** array: **`raw_bonus_chains`, 31 hits across 11
  `src/rules_core/` files**, in **no** `pcgen_residue_gate.py` pattern and therefore invisible
  to cycles 1–3 and to the criterion's own Evidence sentence.

  | file (under `src/rules_core/`) | `raw_bonus_chains` hits | what it is |
  |---|---:|---|
  | `race_resolver.rs` | 6 | `ResolvedTrait::raw_bonus_chains: Vec<RawBonusChain>` — a live struct field, plus two production walks. The same shape cycle 3 narrowed away for the token array. |
  | `rules_tables/pathfinder_unchained/monk_features.rs` | 5 | open-coded `record["raw_bonus_chains"]` ground-truth traversals — `bonus_chain_qualifiers` is now the accessor they should call |
  | `rules_tables/pathfinder_unchained/rogue_features.rs` | 4 | as above |
  | `rules_tables/pathfinder_unchained/barbarian_features.rs` | 4 | as above |
  | `race_creation.rs` | 3 | two production walks + one construction |
  | `equipment_effects/general.rs` | 2 | doc comments |
  | `corpus_loader.rs` | 1 | a test assertion message (production side cleared this cycle) |
  | `equipment_effects.rs` | 1 | doc comment |
  | 3 further files | 5 | doc comments |
  | **total** | **31** | 11 files |

  Recorded as `deferral 1789082356639-at-35-e6-002-1e84c4`, not as a silent gap. **Nothing here
  is a carve-out: 31 is a number to close**, and closing it needs the gate's pattern list to name
  `raw_bonus_chains` so it can never go invisible again.
- **Discoveries:** **one, and it is a `correction` against an instrument.**
  `scripts/pcgen_residue_gate.py`'s pattern list names `raw_tokens` and not its sibling
  `raw_bonus_chains`, so **31 hits across 11 `src/rules_core/` files — including an open-coded
  production traversal in `corpus_loader.rs` — were invisible to every `AT-35-E6-002` cycle**,
  and to the criterion's Evidence sentence, which names `raw_tokens` only. No published figure
  changes value; a population that was never measured becomes visible. `correction
  1789082356496-at-35-e6-002-65186a`, verified by
  `grep -rho '\braw_bonus_chains\b' --include=*.rs src/rules_core/ | wc -l` (31) and
  `grep -rl … | wc -l` (11). Blast radius: cycles 1–3's receipts and the criterion's own
  Evidence sentence.
- **Figures + their re-derive commands:** every row carries its own command.
  The unit denominator where one applies is the whole corpus, all books — **49,438** (`jq '.units | length' docs/work-inventory.json`).

  | figure | value | command | denominator |
  |---|---|---|---|
  | `raw_tokens` under `src/rules_core/`, files | **0** (from 5) | `grep -rl '\braw_tokens\b' --include=*.rs src/rules_core/ \| wc -l` | 196 live `src/rules_core` files the gate scans |
  | `raw_tokens` under `src/rules_core/`, matches | **0** (from 40) | `grep -rho '\braw_tokens\b' --include=*.rs src/rules_core/ \| wc -l` | as above |
  | `raw_bonus_chains` under `src/rules_core/`, files | **11** | `grep -rl '\braw_bonus_chains\b' --include=*.rs src/rules_core/ \| wc -l` | as above |
  | `raw_bonus_chains` under `src/rules_core/`, matches | **31** | `grep -rho '\braw_bonus_chains\b' --include=*.rs src/rules_core/ \| wc -l` | as above |
  | `raw_tokens` under `apps/desktop/`, matches | **25** (unchanged) | `grep -rho '\braw_tokens\b' --include=*.rs apps/desktop/ \| wc -l` | 51 live `apps/desktop` files the gate scans |
  | live PCGen files / hits | **247 / 11,900** (from 248 / 11,956) | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | gate's `raw_tokens` pattern, all live roots | **7 files / 31 hits** (from 12 / 71) | same command, first line | 5 live roots |
  | consumer files following the import | **16** | `git diff --name-only 66a6c76948..HEAD -- src apps \| wc -l` | files in that window |
  | new tool-side lines | **498 insertions, 1 deletion, 0 files removed** | `git diff --numstat --find-renames 66a6c76948..HEAD -- src/pcgen_import scripts/oracle_harness src/oracle_validation`; `… --name-status … \| grep -c '^D'` | files in that window |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0`; `DONE: 49438`, every other bucket `0`; `missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=89 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 89 bundle docs |
  | figure provenance | `figures_examined=443 violations=0` | `python3 scripts/denominator_gate.py --check-provenance` | 206 docs |
  | site dashboard pin | `input pin matches docs/work-inventory.json (5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f)` | `./scripts/publish-site-dashboard.sh --check-pin` | 1 feed input |
  | `Light Crossbow (Base)` damage, the doc comment's own re-derive | `{"key": "DAMAGE", "value": "1d8"}` | the two-line `TOK=…; jq …` command quoted above | 1 corpus record |
- **Build scope verified**, all at `b2839b631a`, `CARGO_INCREMENTAL=0`:
  - `cargo test --locked --no-run -j 6` (target dir `/tmp/cargo-sd35-AT-35-E6-002`) →
    `NO_RUN_EXIT=0`, **413 `Executable` lines**
    (`grep -c '^  Executable' /tmp/e6002c4-norun.log`) — unchanged from cycles 1–3, as a module
    added inside an existing crate must leave it. `grep -cE '^(error|warning)'` → **0**.
  - `cargo test --locked --lib -j 6` → `ok. 3276 passed; 0 failed; 15 ignored` (50.06 s),
    `LIB_EXIT=0`. **Identical to cycle 3's 3,276 / 0 / 15** — a relocation that adds no test and
    breaks none, which is what "behaviour-identical" has to look like at this layer. The 16
    tests that moved to `src/pcgen_import/ingest_payload.rs`'s scope kept their names and their
    results.
  - `cargo clippy --locked --tests -j 6` → **exit 0, 0 warnings, 0 errors**, own target dir
    `/tmp/cargo-sd35-AT-35-E6-002-clippy`, 2 m 15 s.
  - **Frontend RAN HERE, not at the epic wrap-up**, because this cycle touched `apps/`
    (`git diff --name-only 66a6c76948..HEAD -- apps/` lists
    `apps/desktop/src-tauri/src/race_catalog.rs`): `cd apps/desktop && npm test` →
    `101/101 test files passed`, `FRONTEND_EXIT=0`; `npm run typecheck` → `TSC_EXIT=0`.
  - **Desktop crate RAN HERE too, and finished:** `cd apps/desktop/src-tauri &&
    cargo test --locked -j 4` → `ok. 576 passed; 0 failed; 0 ignored` (87.81 s),
    `DESKTOP_EXIT=0`, own target dir `/tmp/cargo-sd35-AT-35-E6-002-desktop`,
    `grep -c '^test result: FAILED'` → **0**. Identical to cycle 3's 576 / 0 / 0.
  - `cargo test --locked --no-fail-fast -j 6`: **did NOT finish inside this cycle's turn.** At
    the point the turn ended it had run **105 of 414** test targets with **4,516 passed,
    0 failed, 15 ignored**, `grep -c '^test result: FAILED'` → **0** and
    `grep -cE '^(error|warning)'` → **0**, and was still advancing. Totals derived with `awk`
    over the `test result` lines, not `grep -o` (`AGENTS.md` §Concurrency). **This is an
    incomplete observation, not a pass** — see Notes for the measured cause.
  - **Not run, and why:** `cargo run --locked --bin v06_work_inventory` — this cycle changed no
    corpus record, so `corpus_literal_sweep` is guarded off (`§6` step 3), so the two reports
    the binary rebuilds its verification stamps from do not exist for this tree; it would refuse
    to write, and `--allow-stamp-loss` is forbidden. `git status --porcelain --
    docs/work-inventory.json` is empty and the `--receipt` rows above were computed against the
    file on disk. `cargo run --locked --bin sheet_rule_convert -- --check` and
    `scripts/verify.sh --only pi-sweep` did not get a turn on the workspace target dir, which
    the full suite held for the whole cycle; `data/sheet_rules/` is byte-unchanged
    (`git status --porcelain -- data/` empty) and its token-leak grep prints **0**.
- **Sweep population:** N/A — no corpus record changed (`git status --porcelain -- data/` empty
  at every checkpoint), so `corpus_literal_sweep` was correctly not run (`§6` step 3's guard).
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial.** The criterion's body and both Evidence clauses are now met — the
  `raw_tokens` clause reads **0 files / 0 matches** under `src/rules_core/`. It is not
  `complete` because this cycle's own discovery says the Evidence sentence was measuring half
  the ingest format: **31 `raw_bonus_chains` hits across 11 live files** remain, and the
  workspace suite and desktop crate had not finished when the turn ended. Calling the criterion
  complete on a clause whose instrument this cycle just showed to be incomplete would be exactly
  the "presence gate over a correctness gate" failure the bundle exists to avoid.
- **Notes:**
  - **The workspace suite did not finish inside the turn, and the work was committed anyway**
    (`§6`'s standing instruction). It reached **105 of 414** targets — **4,516 passed, 0 failed,
    0 FAILED suites** — and was still advancing when the turn ended; it is re-run at the epic
    wrap-up (`§10`) and at cycle 5. **The measured cause is throughput, not this cycle's
    change.** `cargo` runs test binaries sequentially and the corpus-wide ones are
    single-threaded: exactly one test binary was consuming one whole core,
    `sd13_druid_level1_spell_baseline` (`ps aux --sort=-%cpu | head -6`), while the box's load
    average sat between five and eleven (`uptime`) with other sessions live on the same shared
    checkout — roughly one target per ten minutes over the observed window
    (`grep -c '^test result' /tmp/e6002c4-full.log`, sampled repeatedly). Four cargo lanes plus npm ran concurrently early in the
    cycle, above the standing three-lane cap, which cost the first half of the run.
    What *is* proven at this tree: all **413** test binaries link (`--no-run` exit 0, 0 errors,
    0 warnings), the whole library suite passes at cycle 3's exact figures, clippy is clean, the
    desktop crate passes at cycle 3's exact figures, and the frontend and typecheck are green.
    The 105 targets that did run include `rules_core::corpus_loader`'s own suite — the module
    this cycle changed the production path of.
  - **`git status --porcelain` is non-empty for every cycle on `tranche/15`** because the shared
    checkout carries an untracked, un-gitignored `.worktrees/` directory holding another
    session's live git worktree. Deliberately not committed (a git worktree is not repo content)
    and deliberately not `.gitignore`d (a shared root file outside this criterion's file-touch
    set). One `.worktrees/` line in `.gitignore` is the mechanism. Cycle 3 filed
    `incident 1789079245735-at-35-e6-002-507e75`, key
    `untracked-worktrees-dir-on-shared-checkout`; it is unchanged and still needs a ruling.
  - **No new unit tests were written for the three new tool-side functions, deliberately, and
    here is what that does and does not cover.** `bonus_chain_qualifiers` is exercised through
    its real call site by two live-corpus tests covering both of its branches (non-empty:
    `Companion Stone (Diplomacy)`'s `BONUS:SKILL` chain; empty: `Arrow (Slaying)`);
    `ingest_record_json` and `ingest_tokens_value` are exercised by the seven fixtures that now
    call them, each of which fails if the emitted JSON shape is wrong. **Not covered:**
    `bonus_chain_qualifiers`'s malformed-entry skip (an entry with no `qualifiers` array). The
    code it replaced skipped the same case with the same `continue`, and no test covered it
    before this cycle either, so this is behaviour-identical rather than a new gap — but it is a
    gap, and naming it is what `AGENTS.md` rule 7 asks for.
- **Next-cycle scope:** `AT-35-E6-002` **cycle 5**, still sequenced **with** `AT-35-E6-003`.
  Scope flags: `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Population is the `raw_bonus_chains` table above: **11 files / 31 hits**, of which the
  production reads are `race_resolver.rs`'s `ResolvedTrait::raw_bonus_chains` field and two
  walks, `race_creation.rs`'s two walks, and the three
  `rules_tables/pathfinder_unchained/*_features.rs` open-coded traversals — the last three are a
  straight substitution of this cycle's new `ingest_record::bonus_chain_qualifiers`. Take them
  in that order, then **widen `scripts/pcgen_residue_gate.py`'s pattern list to name
  `raw_bonus_chains`** so the population can never go unmeasured again, and rebaseline in the
  same commit. Re-run the workspace suite and the desktop crate at that cycle's tree, since
  neither completed here.
