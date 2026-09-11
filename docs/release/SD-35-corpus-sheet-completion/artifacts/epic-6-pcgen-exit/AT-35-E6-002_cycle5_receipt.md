# Cycle 5 — Epic 6 (PCGen exit) / AT-35-E6-002

Cycle 4 took the criterion's Evidence clause — *zero `raw_tokens` hits under `src/rules_core/`* —
to **0 files / 0 matches**, and in doing so discovered that the clause measures **one of the
ingest format's two verbatim arrays**. Its sibling `raw_bonus_chains` was in no gate pattern and
stood at **31 hits across 11 `src/rules_core/` files**, including production traversals. **This
cycle takes that array's code half to zero** — `grep -rn '\braw_bonus_chains\b' --include=*.rs
src/rules_core/ | grep -vE ':\s*(///|//|\*)' | wc -l` prints **0** — and **widens
`pcgen_residue_gate.py` to name the array**, so the population can never go unmeasured again.

- **Commit SHA:** `ef7d54daf1` — the new converter-side module, the `ResolvedTrait` field
  narrowing, the six live/desktop re-pointings, the three ground-truth substitutions, the gate's
  new pattern with its test, and this cycle's `deferral` event. Cycle start `efeccf8c3b`. A later
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
- **Files touched:** **15** (`git show --stat ef7d54daf1 | tail -1` → `15 files changed, 445
  insertions(+), 222 deletions(-)`) — 14 modified, 1 added, 0 renamed, 0 deleted.
  - **Added, 1 file (tool side):** `src/pcgen_import/bonus_chain_reader.rs` (328 lines,
    `wc -l src/pcgen_import/bonus_chain_reader.rs`).
  - **Module declaration, 1 file:** `src/pcgen_import/mod.rs` (one `pub mod` with its
    `decisions.md §11` / `technical-design.md §0` citation).
  - **Live modules changed, 5 files:** `src/rules_core/{race_resolver, race_creation,
    corpus_loader}.rs` and `src/rules_core/rules_tables/pathfinder_unchained/{monk_features,
    barbarian_features, rogue_features}.rs` — six files, of which `corpus_loader.rs` is a test
    assertion message only.
  - **Desktop, 1 file:** `apps/desktop/src-tauri/src/race_catalog.rs`.
  - **Tool-side test following the type, 1 file:** `tests/sd27_crb_race_corpus_pin.rs`.
  - **Instrument, 2 files:** `scripts/pcgen_residue_gate.py` (the new pattern and its header
    row) and `scripts/tests/test_pcgen_residue_gate.py` (the planted case and the
    `apps/desktop` row it moves, 12 → 13).
  - **Retro, 2 files:** `docs/retro/events/at-35-e6-002.jsonl`, `docs/retro/events/root.jsonl`.
  - **Folded working-tree append, 1 file:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the
    `derived_at` re-stamp `completion_atlas.py --check` writes).
  - **Zero `data/` files changed.** `git status --porcelain -- data/` is empty.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS in shipping code — **1 match against the develop
  base, attributed, not a violation, and not this cycle's; and 1 match in this cycle's own diff
  that is a diff header, not code.**
  ```
  BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47   # git merge-base HEAD origin/develop
  CODE="src/rules_core src/pcgen_import src/bin src/oracle_validation apps/desktop/src-tauri/src scripts/pcgen_residue_gate.py"
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  +/// (`tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs`); this function is its
  ```
  The same doc-comment citation of a real test file's path in `src/rules_core/racial_sla.rs`
  that every `AT-35-E5-*` and `AT-35-E6-*` receipt has recorded — pre-existing and unchanged
  here. Adding `tests` to the audit surface adds exactly one line, and it is the diff's own
  header naming the file this cycle edited:
  ```
  git diff --unified=0 efeccf8c3b..HEAD -- $CODE tests ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  391:+++ b/tests/sd27_crb_race_corpus_pin.rs
  ```
  No shipping-code identifier. This cycle's contribution to the develop-base grep is **0**.
- **Wired-integration audit result:** **OK_NO_TOKENS for this cycle; 8 matches against the
  develop base, all attributed, none a stub marker.**
  ```
  git diff --unified=0 efeccf8c3b..HEAD -- $CODE tests ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
  ->  (no output)  OK_NO_TOKENS
  ```
  The `${BASE_BRANCH}...HEAD` form reports the 8 lines cycles 1–4 already recorded and
  attributed: the **domain word** `placeholder` naming PCGen's own `SOURCEPAGE:p.xx`
  non-citation (`decisions.md §26`, §27.2), its `%LIST` "whatever the player chose" counterpart,
  and the `.MOD` "no selection" rows. Text unchanged, count unchanged, no shipping-code stub.
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
  | every `src/bin` generator's import path follows | cycles 1–4's 77 files, this cycle's 0, 0 residual | **yes** |
  | `gen_book_cache` byte-identical on one book | cycle 1: 2,207 records, same manifest sha256 | **yes** |
  | zero `raw_tokens` hits under `src/rules_core/` | **0 files, 0 matches** (cycle 4) | **yes** |
  | *(the clause's unmeasured half, cycle 4's `correction`)* `raw_bonus_chains` code reads under `src/rules_core/` | **0** (was 19 of 31 hits) | **yes** |
  | *(same)* `raw_bonus_chains` mentions under `src/rules_core/` | **12 / 6 files**, all comments | **no — the deferred remainder** |
- **Receipt rows (mechanical):**
  ```
  since=efeccf8c3bfe12ad9816c3a8bf07962ea0dadba0 target_dir=/tmp/cargo-sd35-AT-35-E6-002 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=647 ratio=n/a builds_recorded=1 pcgen_live_files=247
  ```
  `closed=0` / `relabeled=0` is correct: the unit population was already `0` non-DONE at cycle
  start. `ratio` is `n/a`, a division by zero, never `0.0`. `pcgen_live_files=247` is **equal to**
  cycle 4's `247` and above nothing — see the residue row for why equality is the right outcome
  when a cycle both removes reads and widens the pattern that counts them.
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, at `ef7d54daf1`):
  ```
  pattern raw_tokens files=7 hits=31
  pattern raw_bonus_chains files=8 hits=22
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
  root src/rules_core files=196 hits=11426
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=51 hits=477
  identifier_files=28 identifier_hits=162
  live_files=247 live_hits=11903 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  **`verdict=PASS`, and the axis that §8 escalates on — `live_files` — is flat at 247, not
  above it.** `live_hits` reads `11,903` against cycle 4's `11,900`, and the three are **the
  instrument, not the code**: this cycle added a fourteenth pattern that counts 22 hits nothing
  counted before, while its code changes removed 19 (`BONUS:` 2,214 → 2,198, `TYPE=` 734 → 731).
  Measured both ways rather than argued: at this same tree, `git stash`-free, the gate's own
  pattern list without the new row would print `live_hits=11881` — **19 below** cycle 4
  (`python3 -c` over the same scan with `raw_bonus_chains` removed from `IDENTIFIER_PATTERNS`).
  A rise that is entirely a new measurement of an old population is exactly what `AGENTS.md`
  rule 8 asks a cycle to build, and it is why the baseline is **deliberately NOT rebaselined**
  here: the ratchet stays at cycle 1's `260 / 12,736`, `--rebaseline` is `AT-35-E6-004`'s step,
  and leaving a higher baseline in place can only ever be stricter than resetting it.
- **Oracle parity:** **N/A for this cycle, and correct by construction.** No `Number` mapping was
  added, so the fixture-roster oracle comparison is not triggered (`§6` step 3). Epic 6 touches a
  live path, so the row is owed an answer, and this cycle's answer is stronger than agreement
  within tolerance: every reading moved is a **transcription of the same qualifier list by the
  same code**, lifted verbatim into `bonus_chain_reader` (the `VAR` accumulation, the bare-integer
  dedup, the `STAT` codes/magnitude split, the `ABILITYPOOL` sum, the `TYPE=Boolean` flag test and
  its name conventions), so the same on-disk record yields the same values. The live-corpus tests
  that pin those values against real rows — `declared_bonus_magnitudes_reads_real_chains_
  including_the_indirect_var_form` (Dwarf Stonecunning `[2]`, Dwarf ability scores `[2, -2]`),
  `no_row_takes_its_display_value_from_an_internal_flag_chain` (swept over every served row) and
  `sd27_crb_race_corpus_pin`'s derivation of every CRB race's ability grant — all still pass
  unchanged. `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the population was already 0 non-DONE at cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** 0. This cycle evaluated nothing and converted nothing.
  - **instrument-correction:** 0 *to a published unit figure*. The instrument change this cycle
    ships is the **mechanism** cycle 4's `correction 1789082356496-at-35-e6-002-65186a` called
    for — `pcgen_residue_gate.py` now names `raw_bonus_chains` — so no previously published
    number changes value; a previously unmeasured population becomes permanently visible.
- **Refused tokens:** **none.** This cycle added no converter refusal and cleared none; the
  refused set is unchanged at **142** records, one shape, `refused_non_done=0`
  (`token_coverage.py --check`). It shipped no converter mapping row at all. The
  `raw_bonus_chains` remainder below is **live-side ingest-format usage, not a converter
  refusal**, so §8's "more than 10 distinct refused token types" escalation does not apply.
- **What the four mechanisms were, and why none is a rename.**
  1. **`src/pcgen_import/bonus_chain_reader.rs` — the chain readings leave the live side.**
     The sibling of `race_trait_tokens` for the ingest record's other array. What it hands back
     is **narrowed, already-classified values**: `magnitudes: Vec<i32>`,
     `magnitudes_excluding_flags: Vec<i32>`, `only_internal_flags: bool`,
     `ability_adjustments: Vec<AbilityAdjustment>`, `ability_pool_picks: u8`,
     `var_contributions: Vec<VarContribution>`. **This is why it is not a rename:** after it, no
     live module names a qualifier position, a chain keyword (`VAR`, `STAT`, `ABILITYPOOL`,
     `TYPE=Boolean`) or the ingest field. What crosses the boundary is typed values, not the
     chains under a different spelling — the failure mode a renamed field would have been.
     8 unit tests came with it, including the two the desktop crate gave up.
  2. **`ResolvedTrait::raw_bonus_chains` → `declared_bonuses` — a live struct stops holding a
     `pcgen_import` type.** The field was `Vec<RawBonusChain>`: the ingest type itself, carried
     through resolution into every downstream reader. It is now `DeclaredBonuses`, built once at
     resolution time by mechanism 1. Three walks in `race_resolver.rs`, two in `race_creation.rs`
     and two in the desktop `race_catalog.rs` are gone with it, as is that crate's
     `use codex::pcgen_import::ingest_payload::RawBonusChain`.
  3. **`is_internal_flag_chain` / `variable_name_is_flag_shaped` move to the converter with their
     corpus citations and their test.** Recognising an internal state-flag write means knowing
     that the source writes engine state with the same token it uses for real magnitudes — that
     is ingest-format knowledge, and `decisions.md §11` puts it on the tool side. The desktop
     keeps the **behaviour** test a player would notice
     (`no_row_takes_its_display_value_from_an_internal_flag_chain`, swept over every served row)
     and gives up only the unit test of the recognition rule, which now runs in
     `bonus_chain_reader` under the same name and the same corpus-backed cases.
  4. **The three `pathfinder_unchained/*_features.rs` ground-truth traversals call
     `ingest_record::bonus_chain_qualifiers`.** They open-coded
     `record["raw_bonus_chains"].as_array().expect(…)` beside `ingest_record::token_keys` and
     `token_pairs` calls they already made — the accessor cycle 4 built for exactly this and
     nothing had yet used. One behavioural note, stated rather than buried: the accessor **skips**
     a malformed entry where the open-coded form **panicked** on a non-array. No corpus record
     takes that branch (the assertions downstream read the same records and still pass), and it
     matches how `token_pairs` has always treated a malformed token.
- **On comment rewrites, under cycle 2's own rule.** Cycle 2's rule was: *no comment is reworded
  in any file that still carries a code read.* Comments were reworded only in files whose code
  half this cycle reduced to zero **first**, in the same commit: `race_resolver.rs`,
  `race_creation.rs` and the desktop `race_catalog.rs`. Each rewrite says more, not less — it
  names the converter-side function that now owns the reading. Three diagnostic strings in
  `race_creation.rs` stopped spelling `BONUS:STAT`, and that is accuracy rather than euphemism:
  the function no longer reads a `BONUS:STAT` chain, it reads an `AbilityAdjustment`, and naming
  what it holds is the truer message. No test asserts any of the three
  (`grep -rn 'is missing its codes or magnitude' --include=*.rs src/ tests/ apps/` → 1 hit, the
  definition).
- **The remainder, enumerated — 12 hits, 6 files, and every one of them a comment.**
  `raw_bonus_chains` **code** under `src/rules_core/` is **0**. What remains is provenance prose:

  | file (under `src/rules_core/`) | hits | what it is |
  |---|---:|---|
  | `pilot_compute/mod.rs` | 5 | derivation notes: "derived by scanning every ARG alternate's `raw_bonus_chains` against the engine's computed-total surface", ×3, plus two record citations |
  | `equipment_effects/arms_armor.rs` | 2 | doc comments citing which array a stated absence was confirmed against |
  | `equipment_effects/general.rs` | 2 | as above |
  | `equipment_effects.rs` | 1 | doc comment |
  | `equipment_effects/equipmods.rs` | 1 | doc comment |
  | `rules_tables/pathfinder_unchained/monk_features.rs` | 1 | a comment recording what the `.MOD` closure enrichment changed |
  | **total** | **12** | 6 files |

  **These are `AGENTS.md` rule 9 provenance.** Each names the on-disk array a stated figure was
  derived from, so the re-derive path a reader would run stays nameable. Rewording them would
  move the grep without moving the fact — the euphemism the residue gate exists to catch — so
  they are deferred with a name rather than laundered. Recorded as
  `deferral 1789085418760-at-35-e6-002-00a96d`, and its revisit condition is `AT-35-E6-004`'s
  `--closure` mode, which must rule on whether a provenance citation counts against `live_hits`.
  **Nothing here is a carve-out: 12 is a number to close or to rule on, never an exemption.**

  Outside this criterion's scope and named so it is not lost: **`apps/desktop/src-tauri/src/
  intelligent_item_catalog.rs` carries the other 6 `raw_bonus_chains` hits over 1 file, one of
  them a real production read** (`let chains = data["raw_bonus_chains"].as_array()…`, `:472`).
  That is `AT-35-E6-003`'s territory (the desktop crate), not this criterion's.
- **Discoveries:** **none.** Cycle 4's `correction` named the population; this cycle built the
  mechanism it called for and closed the code half. No token type, kind, or remaining-step
  category surfaced that `token-coverage.json` or the atlas did not predict — both are at
  `verdict=PASS` with `non_done=0`.
- **Figures + their re-derive commands:** every row carries its own command.
  The unit denominator where one applies is the whole corpus, all books — **49,438** (`jq '.units | length' docs/work-inventory.json`).

  | figure | value | command | denominator |
  |---|---|---|---|
  | `raw_bonus_chains` **code reads** under `src/rules_core/` | **0** (from 19) | `grep -rn '\braw_bonus_chains\b' --include=*.rs src/rules_core/ \| grep -vE ':\s*(///\|//\|\*)' \| wc -l` | 196 live `src/rules_core` files the gate scans |
  | `raw_bonus_chains` under `src/rules_core/`, matches | **12** (from 31) | `grep -rho '\braw_bonus_chains\b' --include=*.rs src/rules_core/ \| wc -l` | as above |
  | `raw_bonus_chains` under `src/rules_core/`, files | **6** (from 11) | `grep -rl '\braw_bonus_chains\b' --include=*.rs src/rules_core/ \| wc -l` | as above |
  | `raw_bonus_chains` under `apps/desktop/`, matches / files | **6 / 1** | `grep -rho '\braw_bonus_chains\b' --include=*.rs apps/desktop/ \| wc -l`; `grep -rl … \| wc -l` | 51 live `apps/desktop` files the gate scans |
  | `raw_tokens` under `src/rules_core/`, matches | **0** (unchanged, cycle 4) | `grep -rho '\braw_tokens\b' --include=*.rs src/rules_core/ \| wc -l` | 196 live files |
  | live PCGen files / hits | **247 / 11,903** (from 247 / 11,900) | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | the same, with the new pattern removed | **247 / 11,881** — 19 below cycle 4 | `python3 -c "import sys;sys.path.insert(0,'scripts');import pcgen_residue_gate as g;g.IDENTIFIER_PATTERNS.pop('raw_bonus_chains');g.PATTERNS=dict(g.IDENTIFIER_PATTERNS,**g.TOKEN_SYNTAX_PATTERNS);g._COMPILED={n:__import__('re').compile(r) for n,r in g.PATTERNS.items()};r=g.scan('.');print(r.live_files,r.live_hits)"` | as above |
  | gate patterns | **14** (from 13) | `python3 -c "import sys;sys.path.insert(0,'scripts');import pcgen_residue_gate as g;print(len(g.PATTERNS))"` | 1 instrument |
  | files changed | **15** (14 modified, 1 added) | `git show --stat ef7d54daf1 \| tail -1` | files in that commit |
  | tool-side lines added | **345 insertions, 4 deletions** | `git diff --numstat --find-renames efeccf8c3b..HEAD -- src/pcgen_import src/oracle_validation scripts \| awk '{a+=$1;d+=$2} END{print a,d}'` | files in that window |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0`; `DONE: 49438`, every other bucket `0`; `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=91 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 91 bundle docs |
  | figure provenance | `files_checked=208 figures_examined=462 violations=0` | `python3 scripts/denominator_gate.py --check-provenance` | 208 docs |
  | sheet-rule package | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (114.2s)` | `cargo run --locked --bin sheet_rule_convert -- --check` | 49,438 units |
  | site dashboard pin | `input pin matches docs/work-inventory.json (5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f)` | `./scripts/publish-site-dashboard.sh --check-pin` | 1 feed input |
  | PI sweep | `RESULT: PASS`, 1 stage | `scripts/verify.sh --only pi-sweep` | 137 generated table files |
  | gate's own unit suite | `Ran 15 tests … OK` | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` | 15 tests |
  | library suite | `3285 passed; 0 failed; 15 ignored` | `cargo test --locked --lib -j 6` | 3,300 library tests |
  | desktop crate | `575 passed; 0 failed; 0 ignored` | `cd apps/desktop/src-tauri && cargo test --locked -j 4` | 575 desktop tests |
  | frontend | `101/101 test files passed`; `tsc --noEmit` exit 0 | `cd apps/desktop && npm test`; `npm run typecheck` | 101 test files |
  | test binaries linked | **413** | `grep -c '^  Executable' /tmp/e6002c5-norun.log` | 414 test targets |
- **Build scope verified**, all at `ef7d54daf1`, `CARGO_INCREMENTAL=0`:
  - `cargo test --locked --no-run -j 6` (target dir `/tmp/cargo-sd35-AT-35-E6-002`) →
    `NO_RUN_EXIT=0`, **413 `Executable` lines** — unchanged from cycles 1–4, as a module added
    inside an existing crate must leave it. `grep -cE '^(error|warning)'` → **0**.
  - `cargo test --locked --lib -j 6` → `ok. 3285 passed; 0 failed; 15 ignored` (39.65 s),
    `LIB_EXIT=0`. **+9 against cycle 4's 3,276**, which is this cycle's 8 new
    `bonus_chain_reader` unit tests plus one landed by another cycle between `b2839b631a` and
    this cycle's start — the baseline at this tree is `efeccf8c3b`, not cycle 4's tree.
  - `cargo clippy --locked --tests -j 4` → **exit 0, 0 warnings, 0 errors**, own target dir
    `/tmp/cargo-sd35-AT-35-E6-002-clippy`, 1 m 26 s.
  - **Desktop crate RAN HERE, not at the epic wrap-up**, because this cycle touched `apps/`:
    `cd apps/desktop/src-tauri && cargo test --locked -j 4` → `ok. 575 passed; 0 failed;
    0 ignored` (91.26 s), `DESKTOP_EXIT=0`, own target dir
    `/tmp/cargo-sd35-AT-35-E6-002-desktop`. **575, not cycle 4's 576, and the count moved for one
    named reason:** `flag_shaped_variable_names_are_recognized_and_magnitude_names_are_not` moved
    to `src/pcgen_import/bonus_chain_reader.rs` with the function it tests, where it runs under
    the same name over the same cases. No assertion anywhere pins a desktop test count
    (`grep -rn '\b576\b' --include=*.rs --include=*.py --include=*.sh apps/desktop/src-tauri/src
    scripts/` → 1 hit, unrelated prose in `scripts/pi_scrub.py`).
  - **Frontend RAN HERE too:** `cd apps/desktop && npm test` → `101/101 test files passed`,
    `FRONTEND_EXIT=0`; `npm run typecheck` → `TSC_EXIT=0`.
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (114.2s)`
    — identical to cycle 4 on every field. `git status --porcelain -- data/` empty afterwards.
  - `python3 -m unittest scripts.tests.test_pcgen_residue_gate` → `Ran 15 tests … OK`. The gate's
    own suite is what proves the new pattern is counted rather than merely declared:
    `test_every_design_pattern_is_counted` asserts every entry in `PATTERNS` scores at least one
    hit against a synthetic tree, so a pattern that matched nothing would fail there.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`, `RETRO_ACTOR` exported in the same shell
    invocation so the derived `verification` event lands in this cycle's log.
  - `cargo test --locked --no-fail-fast -j 6`: see **Notes** — the observation recorded there is
    the figure this receipt stands behind, not a claim beyond it.
- **Sweep population:** N/A — no corpus record changed (`git status --porcelain -- data/` empty
  at every checkpoint), so `corpus_literal_sweep` was correctly not run (`§6` step 3's guard), and
  with it `v06_work_inventory`, which rebuilds its verification stamps from that sweep's report
  and would refuse to write without it (`--allow-stamp-loss` is forbidden).
  `git status --porcelain -- docs/work-inventory.json` is empty and the `--receipt` rows above
  were computed against the file on disk.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial.** Both Evidence clauses and the criterion's body are met, and the
  half of the ingest format the Evidence sentence never named is now **zero code reads** under
  `src/rules_core/` *and* permanently measured. It is not `complete` because **12
  `raw_bonus_chains` mentions across 6 live files** remain — every one a provenance comment, none
  a read — and ruling on whether a provenance citation counts against the live surface belongs to
  `AT-35-E6-004`'s `--closure` mode, not to this cycle's judgment.
- **Notes:**
  - **The workspace suite did not finish inside the turn, for the second cycle running, and the
    work was committed and pushed anyway** (`§6`'s standing instruction). At the point this
    receipt was written it had run **258 of 414** targets — **6,709 passed, 0 failed, 0 FAILED
    suites** (`grep -c '^test result' /tmp/e6002c5-full.log`; totals by `awk` over the
    `test result` lines, not `grep -o`, per `AGENTS.md` §Concurrency) — and was still advancing;
    the closing figure is in the report this cycle returns. **This is an incomplete observation,
    not a pass.** The cause is the one cycle 4 measured — cargo runs test binaries sequentially
    and the corpus-wide ones are single-threaded — and it is now a recurrence, which
    `AGENTS.md` rule 8 says is a missing mechanism rather than bad luck; cycle 4 filed
    `incident … workspace-suite-too-slow-for-one-turn` and it still needs a ruling.
    What *is* proven at this tree: all 413 test binaries link, the whole library suite passes,
    clippy is clean, the desktop crate and the frontend pass, and the 258 targets that did run
    include `sd27_crb_race_corpus_pin` — the integration test this cycle changed.
  - **`git status --porcelain` is non-empty for every cycle on `tranche/15`** because the shared
    checkout carries an untracked, un-gitignored `.worktrees/` directory holding another
    session's live git worktree. Cycle 3 filed
    `incident 1789079245735-at-35-e6-002-507e75`, key
    `untracked-worktrees-dir-on-shared-checkout`; unchanged, still needs a ruling.
  - **What the new module's tests do and do not cover** (`AGENTS.md` rule 7). Covered by unit
    test: source order and dedup of magnitudes; a variable-naming chain contributing no
    magnitude; a whole unparsed code list; a malformed adjustment reporting absence rather than a
    guess; pool picks summing only the ability pool; conditional-versus-plain variable amounts;
    the `TYPE=Boolean` and name-convention flag signals including a mixed chain; and an empty
    chain list. Covered through real corpus rows by the live tests named under **Oracle parity**.
    **Not covered:** a `u8` overflow in `ability_pool_picks` — the code it replaced summed the
    same way with the same risk, and no corpus row approaches it, so this is
    behaviour-identical rather than a new gap; but it is a gap, and naming it is what rule 7 asks.
- **Next-cycle scope:** `AT-35-E6-002` **cycle 6**, or the remainder folded into `AT-35-E6-004`.
  Scope flags: `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Population is the 12-hit / 6-file table above, all comments, plus the ruling the deferral asks
  for: does `--closure` count a provenance citation, or does it exempt one by kind? If it counts
  them, the 12 are rewrites in files with zero code beside them and the cycle is small; if it
  exempts them, the criterion is at zero and closes. Either way, **re-run the workspace suite at
  that cycle's tree** — it has not completed inside a turn since cycle 3.
