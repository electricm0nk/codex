# Cycle 6 — Epic 6 (PCGen exit) / AT-35-E6-002

Cycle 5 closed the `raw_bonus_chains` **code** half and left **12 mentions across 6
`src/rules_core/` files**, deferring them pending an `AT-35-E6-004` `--closure` ruling on
whether a provenance citation counts against `live_hits`. **No ruling was owed** — the
instrument states one in its own module docstring — and one of the twelve was **not a comment
at all** but a shipped, player-visible sheet line. This cycle takes the count to **0 matches /
0 files**, which puts **both** of the ingest format's verbatim arrays at zero under
`src/rules_core/`.

- **Commit SHA:** `1a74fa180f` — the eleven provenance rewrites, the one shipped-prose fix, the
  provenance artifact, and this cycle's two `correction` events. Cycle start `5efafe7b9d`. A
  later commit carries this receipt and the `progress.md` / `kanban.md` rows (a receipt cannot
  name the commit that carries it).
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
- **Files touched:** **9** (`git show --stat 1a74fa180f | tail -1` → `9 files changed, 105
  insertions(+), 15 deletions(-)`) — 8 modified, 1 added, 0 renamed, 0 deleted.
  - **Live modules changed, 6 files** (comments only, except the one shipped string named below):
    `src/rules_core/equipment_effects.rs`, `src/rules_core/equipment_effects/{arms_armor,
    equipmods,general}.rs`, `src/rules_core/pilot_compute/mod.rs`,
    `src/rules_core/rules_tables/pathfinder_unchained/monk_features.rs`.
  - **Artifact, 1 file (added):**
    `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-002_cycle6_ingest-array-provenance.md`
    — the before/after table preserving all 12 original wordings verbatim.
  - **Retro, 1 file:** `docs/retro/events/at-35-e6-002.jsonl`.
  - **Folded working-tree append, 1 file:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the
    `derived_at` re-stamp `completion_atlas.py --check` writes).
  - **Zero `data/` files changed**, **zero `apps/` files changed**, **zero `scripts/` files
    changed.** `git status --porcelain -- data/ apps/ scripts/` is empty.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS for this cycle; 1 match against the develop
  base, attributed, pre-existing and not this cycle's.**
  ```
  BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47   # git merge-base HEAD origin/develop
  CODE="src/rules_core src/pcgen_import src/bin src/oracle_validation apps/desktop/src-tauri/src scripts/pcgen_residue_gate.py"
  git diff --unified=0 5efafe7b9d..HEAD -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  (no output)  OK_NO_BUNDLE_TAGS
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  1
  ```
  The same doc-comment citation of a real test file's path in `src/rules_core/racial_sla.rs`
  that every `AT-35-E5-*` and `AT-35-E6-*` receipt has recorded. This cycle's contribution is
  **0**.
- **Wired-integration audit result:** **OK_NO_TOKENS for this cycle; 8 matches against the
  develop base, all attributed, none a stub marker.**
  ```
  git diff --unified=0 5efafe7b9d..HEAD -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
  ->  (no output)  OK_NO_TOKENS
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $CODE … | grep -cE '…'  ->  8
  ```
  The 8 are the **domain word** `placeholder` naming PCGen's own `SOURCEPAGE:p.xx` non-citation
  (`decisions.md §26`, §27.2), its `%LIST` counterpart and the `.MOD` "no selection" rows,
  recorded and attributed by cycles 1–5. Text unchanged, count unchanged.
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
  | every `src/bin` generator's import path follows | cycles 1–4's 77 files, 0 residual | **yes** |
  | `gen_book_cache` byte-identical on one book | cycle 1: 2,207 records, same manifest sha256 | **yes** |
  | zero `raw_tokens` hits under `src/rules_core/` | **0 files, 0 matches** (cycle 4, still 0) | **yes** |
  | *(the clause's unmeasured half, cycle 4's `correction`)* `raw_bonus_chains` code reads under `src/rules_core/` | **0** (cycle 5) | **yes** |
  | *(same)* `raw_bonus_chains` **mentions** under `src/rules_core/` | **0 / 0 files** (was 12 / 6) | **yes — this cycle** |
- **Receipt rows (mechanical):**
  ```
  since=5efafe7b9dce7391009b7398af4ee86d70d1346d target_dir=/tmp/cargo-sd35-AT-35-E6-002 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=44 ratio=n/a builds_recorded=1 pcgen_live_files=247
  ```
  `closed=0` / `relabeled=0` is correct: the unit population was already `0` non-DONE at cycle
  start. `ratio` is `n/a`, a division by zero, never `0.0`. `pcgen_live_files=247` is **equal to**
  cycle 5's `247` and above nothing — the 6 files this cycle cleaned still carry other patterns
  (`BONUS:`, `TYPE=`, `DESC:`), so they stay in the file count while their hit count falls; that
  residual is `AT-35-E6-004`'s closure surface.
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, at `1a74fa180f`):
  ```
  pattern raw_tokens files=7 hits=31
  pattern raw_bonus_chains files=2 hits=10
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=17 hits=109
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=137 hits=2198
  pattern DEFINE: files=26 hits=116
  pattern PRE[A-Z]+: files=118 hits=7926
  pattern SAB: files=0 hits=0
  pattern DESC: files=137 hits=565
  pattern %CHOICE files=8 hits=66
  pattern %LIST files=24 hits=139
  pattern TYPE= files=67 hits=731
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=51 hits=477
  identifier_files=22 identifier_hits=150
  live_files=247 live_hits=11891 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  **`verdict=PASS`, and every axis fell or held.** `live_hits` **11,903 → 11,891** (−12, exactly
  the 12 closed), `identifier_files` **28 → 22** (−6, exactly the 6 files), `identifier_hits`
  **162 → 150** (−12), `root src/rules_core` hits **11,426 → 11,414** (−12). `live_files` is flat
  at **247** because the 6 cleaned files still match other patterns. `raw_bonus_chains` as a
  pattern falls **8 files / 22 hits → 2 files / 10 hits**; both survivors are under
  `apps/desktop/` (`AT-35-E6-003`'s territory), listed under **The remainder** below.
  The baseline stays at cycle 1's `260 / 12,736` — `--rebaseline` is `AT-35-E6-004`'s step, and
  leaving a higher ratchet in place can only ever be stricter.
- **Oracle parity:** **N/A for this cycle.** No `Number` mapping was added. Epic 6 touches a live
  path, so the row is owed an answer: **eleven of the twelve edits are comment text and cannot
  change a value**, and the twelfth is a `ComputationExplanation.detail` string whose `value`
  field (`ROUGAROU_BITE_DAMAGE_DIE` = 4) is untouched and whose test assertion
  (`rougarou_gets_speed_senses_and_natural_weapon_explanations`, `detail.contains("1d4")`) still
  passes. No computed number in the repo moved.
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the population was already 0 non-DONE at cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** 0. This cycle evaluated nothing and converted nothing.
  - **instrument-correction:** 0 *to a published unit figure*. `scripts/pcgen_residue_gate.py`
    was **not touched** this cycle; the residue movement is entirely code, not instrument — which
    is why it is a clean −12 on every axis rather than cycle 5's mixed reading.
- **Refused tokens:** **none.** This cycle added no converter mapping row and cleared none; the
  refused set is unchanged at **142** records, one shape, `refused_non_done=0`
  (`token_coverage.py --check`). §8's "more than 10 distinct refused token types" escalation does
  not apply.
- **What the two mechanisms were, and why neither is a euphemism.**
  1. **Eleven provenance citations name the tool-side accessor instead of the on-disk field.**
     Each now reads `pcgen_import::ingest_record::bonus_chain_qualifiers`
     (`src/pcgen_import/ingest_record.rs:68`) where it read `raw_bonus_chains`. That is *more*
     precise as an `AGENTS.md` rule 9 re-derive path, not less — a reader runs a function rather
     than greps a field — and it matches the live type's own name since cycle 5 renamed
     `ResolvedTrait::raw_bonus_chains` to `declared_bonuses` (`DeclaredBonuses`,
     `src/rules_core/race_resolver.rs:418`). **The control that makes this honest rather than a
     rewording:** every original wording is preserved verbatim in
     `AT-35-E6-002_cycle6_ingest-array-provenance.md`, on the tool side of the boundary, so
     nothing was laundered — the fact moved with the name.
  2. **One shipped sheet line stops printing PCGen token syntax.** See the next row.
- **Cycle 5's remainder table was wrong about one row, and it was the row that mattered.**
  Cycle 5 recorded "12 hits, 6 files, and every one of them a comment". **Eleven were.** The
  twelfth, `src/rules_core/pilot_compute/mod.rs:11694`, sits inside the `format!` that builds
  `ComputationExplanation { id: "race.rougarou.trait_bundle.natural_weapon", … }.detail` — **the
  rendered sheet line a player reads** — and printed
  `raw_bonus_chains WEAPONPROF=Bite/DAMAGESIZE -1`: our own ingest array name *and* a PCGen
  qualifier chain, on the sheet. `decisions.md §1` says the sheet prints one final number or the
  rule's words; `DAMAGESIZE|-1` on a bite means the damage die steps down one size, which is how
  the record's `1d4` is reached, so the words were always available. It now reads
  **"with the bite's damage die stepped down one size"**. Recorded as
  `correction 1789086743344-at-35-e6-002-becdad`.
- **Cycle 5's deferral asked for a ruling that already existed.** Its revisit condition was
  `AT-35-E6-004`'s `--closure` mode "ruling on whether a provenance citation counts against
  `live_hits`". `scripts/pcgen_residue_gate.py`'s module docstring (lines 36–40) already says:
  *"A mention inside a comment or a doc string counts -- the ruling is 'nothing left of pcgen',
  and a comment explaining a PCGen token on the live side is a sign the code next to it still
  needs one"*, and `--closure` requires `live_files=0 live_hits=0`. The 12 were closable on the
  instrument's own terms. Recorded as `correction 1789086732925-at-35-e6-002-987fc7`.
  `deferral 1789085418760-at-35-e6-002-00a96d` is thereby discharged, not carried.
- **The remainder — under this criterion's roots, zero.**
  `raw_bonus_chains` and `raw_tokens` are both **0 matches / 0 files** under `src/rules_core/`,
  `src/saved_character/`, `src/campaign/` and `src/homebrew_authoring/`. The pattern's surviving
  **2 files / 10 hits** are all under `apps/desktop/` and are **`AT-35-E6-003`'s** territory, not
  this criterion's — named here so they are not lost:

  | file | hits | what it is |
  |---|---:|---|
  | `apps/desktop/src-tauri/src/intelligent_item_catalog.rs` | 6 | includes a real production read (`let chains = data["raw_bonus_chains"].as_array()…`, `:472`) |
  | `apps/desktop/src/characterHub/raceCreationCoverage.test.ts` | 4 | frontend coverage test |
  | **total** | **10** | 2 files |

  Unchanged by this cycle (`git status --porcelain -- apps/` empty); the count is identical to
  cycle 5's reading of the same files.
- **Discoveries:** **one**, and it is the shipped-prose row above — a live-side PCGen leak that
  cycle 5's remainder table had classified as a comment. Emitted as
  `correction 1789086743344-at-35-e6-002-becdad`, as `§7` requires. Neither
  `token-coverage.json` nor the atlas could have predicted it: both measure corpus units, and
  this was a hand-written explanation string. No token type, kind, or remaining-step category
  surfaced — both instruments are at `verdict=PASS` with `non_done=0`.
- **Figures + their re-derive commands:** every row carries its own command.
  The unit denominator where one applies is the whole corpus, all books — **49,438** (`jq '.units | length' docs/work-inventory.json`).

  | figure | value | command | denominator |
  |---|---|---|---|
  | `raw_bonus_chains` under `src/rules_core/`, matches | **0** (from 12) | `grep -rho '\braw_bonus_chains\b' --include=*.rs src/rules_core/ \| wc -l` | 196 live `src/rules_core` files the gate scans |
  | `raw_bonus_chains` under `src/rules_core/`, files | **0** (from 6) | `grep -rl '\braw_bonus_chains\b' --include=*.rs src/rules_core/ \| wc -l` | as above |
  | `raw_tokens` under `src/rules_core/`, matches | **0** (unchanged, cycle 4) | `grep -rho '\braw_tokens\b' --include=*.rs src/rules_core/ \| wc -l` | as above |
  | `raw_bonus_chains` under the other three live `src/` roots | **0** | `grep -rho '\braw_bonus_chains\b' --include=*.rs src/saved_character src/campaign src/homebrew_authoring \| wc -l` | 0 files scanned there |
  | `raw_bonus_chains` under `apps/desktop/`, matches / files | **10 / 2** (unchanged) | `grep -rn '\braw_bonus_chains\b' --include=*.rs --include=*.ts --include=*.tsx apps/desktop \| sed 's/:.*//' \| sort \| uniq -c` | 51 live `apps/desktop` files the gate scans |
  | live PCGen files / hits | **247 / 11,891** (from 247 / 11,903) | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | identifier files / hits | **22 / 150** (from 28 / 162) | same command, `identifier_files=` line | as above |
  | `root src/rules_core` hits | **11,414** (from 11,426) | same command, `root src/rules_core` line | 196 files |
  | gate patterns | **14**, unchanged — the script was not touched | `git status --porcelain -- scripts/` (empty); `python3 -c "import sys;sys.path.insert(0,'scripts');import pcgen_residue_gate as g;print(len(g.PATTERNS))"` | 1 instrument |
  | files changed | **9** (8 modified, 1 added) | `git show --stat 1a74fa180f \| tail -1` | files in that commit |
  | rust lines changed | **44** | `python3 scripts/cycle_scope_gate.py --receipt --since 5efafe7b9d …` | files in that window |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0`; `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=92 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 92 bundle docs |
  | figure provenance | `files_checked=209 figures_examined=462 violations=0` | `python3 scripts/denominator_gate.py --check-provenance` | 209 docs |
  | sheet-rule package | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (114.0s)` | `cargo run --locked --bin sheet_rule_convert -- --check` | 49,438 units |
  | PI sweep | `RESULT: PASS`, 1 stage | `scripts/verify.sh --only pi-sweep` | 137 generated table files |
  | gate's own unit suite | `Ran 15 tests … OK` | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` | 15 tests |
  | library suite | `3285 passed; 0 failed; 15 ignored` | `cargo test --locked --lib -j 6` | 3,300 library tests |
  | test binaries linked | **413** | `grep -c '^  Executable' /tmp/e6002c6-norun.log` | 414 test targets |
  | workspace suite, complete | `414` targets, `8796 passed, 0 failed, 68 ignored`, `0` FAILED suites | `cargo test --locked --no-fail-fast -j 6`; `grep -c '^test result' /tmp/e6002c6-full.log`; `awk '/^test result/{for(i=1;i<=NF;i++){if($(i+1)=="passed;")p+=$i}} END{print p}' /tmp/e6002c6-full.log` | 414 test targets |
- **Build scope verified**, all at `1a74fa180f`, `CARGO_INCREMENTAL=0`:
  - `cargo test --locked --no-run -j 6` (target dir `/tmp/cargo-sd35-AT-35-E6-002`) →
    `NO_RUN_EXIT=0`, **413 `Executable` lines** — unchanged from cycles 1–5, as a comment-only
    change must leave it. `grep -cE '^(error|warning)' /tmp/e6002c6-norun.log` → **0**.
  - `cargo test --locked --lib -j 6` → `ok. 3285 passed; 0 failed; 15 ignored` (40.88 s),
    `LIB_EXIT=0` — **identical to cycle 5 on every field**, which is what a comment-and-one-string
    cycle has to look like.
  - `cargo clippy --locked --tests -j 4` → **exit 0, 0 warnings, 0 errors**, own target dir
    `/tmp/cargo-sd35-AT-35-E6-002-clippy`, 1 m 42 s.
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (114.0s)`
    — identical to cycle 5 on every field. `git status --porcelain -- data/` empty afterwards.
  - `python3 -m unittest scripts.tests.test_pcgen_residue_gate` → `Ran 15 tests … OK`.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`, `RETRO_ACTOR` exported in the same shell
    invocation.
  - **Desktop crate and frontend: epic cadence, correctly NOT run here.** This cycle changed
    **zero** `apps/` files (`git status --porcelain -- apps/` empty; `git show --stat 1a74fa180f`
    lists none), so `§6` step 3's condition — "run here ONLY if the cycle touched `apps/`" — is
    not met. Cycle 5's readings stand at this tree: desktop `575 passed; 0 failed`, frontend
    `101/101`, `tsc --noEmit` exit 0.
  - `cargo test --locked --no-fail-fast -j 6` → **`FULL_EXIT=0`, 414 `test result` lines,
    8,796 passed, 0 failed, 68 ignored, 0 FAILED suites, 0 errors/warnings** (totals by `awk`
    over the `test result` lines, not `grep -o`, per `AGENTS.md` §Concurrency;
    `grep -c '^test result: FAILED' /tmp/e6002c6-full.log` → 0). **It finished, and every field
    is identical to cycle 5's complete run** (414 / 8,796 / 0 failed / 68 ignored) — zero
    movement, which is exactly what a comment-and-one-string cycle must produce.
- **Sweep population:** N/A — no corpus record changed (`git status --porcelain -- data/` empty
  at every checkpoint), so `corpus_literal_sweep` was correctly not run (`§6` step 3's guard), and
  with it `v06_work_inventory`, which rebuilds its verification stamps from that sweep's report
  and would refuse to write without it (`--allow-stamp-loss` is forbidden).
  `git status --porcelain -- docs/work-inventory.json` is empty and the `--receipt` rows above
  were computed against the file on disk.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **complete.** Both Evidence clauses are met at HEAD (`raw_tokens` **0 hits / 0
  files** under `src/rules_core/`; `gen_book_cache` byte-identical on `advanced_race_guide`,
  cycle 1, 2,207 records at the same manifest sha256), the criterion's body is met (17 files
  relocated to `src/pcgen_import/`, every `src/bin` import path following, 0 residual), and the
  remainder every prior cycle carried — the ingest format's second array, which the Evidence
  sentence never named — is now **0 matches / 0 files** under every live `src/` root. Cycle 5's
  deferral is discharged on the instrument's own ruling rather than re-deferred, and its
  mis-classified twelfth hit was a real shipped-prose defect, now fixed. Nothing under this
  criterion's scope is open.
- **Notes:**
  - **`git status --porcelain` is non-empty for every cycle on `tranche/15`** because the shared
    checkout carries an untracked, un-gitignored `.worktrees/` directory holding another
    session's live git worktree. Cycle 3 filed `incident 1789079245735-at-35-e6-002-507e75`, key
    `untracked-worktrees-dir-on-shared-checkout`; unchanged, still needs a one-line `.gitignore`
    ruling.
  - **What this cycle's proof does not cover** (`AGENTS.md` rule 7). It proves the two **ingest
    array identifiers** are gone from every live `src/` root. It proves nothing about **PCGen
    source-file syntax in live prose**, which is a much larger and separately-counted population:
    the `.lst` citations and `ABILITY:…` / `MOVE:Walk,30` tokens in the Rougarou record's sibling
    explanation strings are still there, and the gate counts that class under `BONUS:` / `DESC:` /
    `TYPE=` at `root src/rules_core hits=11414`. That is `AT-35-E6-004`'s closure surface. Naming
    it so this cycle's zero is not read as more than it is.
- **Next-cycle scope:** **criterion at zero** for its own Evidence clause and for both halves of
  the ingest format under `src/rules_core/`. Nothing remains for `AT-35-E6-002` cycle 7. The
  surviving `raw_bonus_chains` reads (`10 hits / 2 files`, all `apps/desktop/`) belong to
  `AT-35-E6-003`; the token-syntax population belongs to `AT-35-E6-004`'s `--closure` +
  `--rebaseline`.
