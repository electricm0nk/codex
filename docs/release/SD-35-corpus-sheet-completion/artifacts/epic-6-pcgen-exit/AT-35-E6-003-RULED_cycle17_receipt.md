# Cycle AT-35-E6-003-RULED cycle 17 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `PENDING_SHA` (the code, the regenerated converted artifact, the cycle-17 census
  script and its JSON, one retro `correction`, one retro `deferral`, and the folded
  shared-checkout artifact), cycle start `2fad97f6f5`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `2fad97f6f5`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 16's closing figure — nothing drifted between the two cycles:
  ```
  live_files=5 live_hits=5 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**

  **The live side — the seam module names no converter at all now:**
  - `src/rules_core/derived_evaluator_fixture_check.rs` — **one counted hit → ZERO; the file is
    cleared.** `use crate::pcgen_import::ingest_record;` is gone, and with it the **three**
    shipping calls behind it (cycle 16's own correction named them at lines 668, 1011 and 1380).
    The two `DURATION:`/`RANGE:` corpus walks (`load_spell_durations`, `load_spell_ranges`) are
    **deleted**; `all_spell_caster_level_durations` / `all_spell_caster_level_ranges` and the two
    spell bar-check halves read the settled tables instead, through the new private
    `settled_spell_formulas(repo_root)`. `SettledSpellFormulas` (the settled shape, its two
    tables and their two refusal sets), `settled_spell_key` / `split_settled_spell_key` and
    `SETTLED_SPELL_KEY_SEPARATOR` are declared here, beside the two formula types they carry.
    **Neither public function's signature, return type, key shape or refusal contract changed**,
    so `apps/` did not need one line written. The `kind=class_feature` bar-check half left this
    file entirely (below).
  - `src/rules_core/record_vars.rs` — `RecordVarPackage` gains `spell_formulas:
    SettledSpellFormulas`, `#[serde(default)]` like its `desc_templates` and `desc_arguments`
    siblings; no existing field was added to, removed from or renamed. New `package_at(root)`
    reads the artifact from an **explicit** repo root, cached once per distinct root, because the
    consumers this cycle moved are handed a root they discovered at run time and the corpus walks
    they replace honoured it. `package()` is unchanged.

  **The converter side — the same walk, once, at authoring time:**
  - `src/pcgen_import/spell_formula_settle.rs` — **new, +~205.** The settling scan: the live
    side's own recursive `data/corpus/<book>/spell/` walk, moved verbatim (same descent, same
    `data.key` identity, same first-token-wins reading through
    `ingest_record::first_token_value`), applying the two **live** parsers
    `parse_caster_level_linear_duration` and `spell_range_formula` — it does not restate a
    reading, it reuses the ones the live side already committed to. A record whose token the
    parser refuses lands in a `*_refused` set rather than being dropped, because "no such record"
    and "a duration the engine reads no formula out of" are different answers and the bar check
    reports them differently.
  - `src/pcgen_import/mod.rs`, `src/bin/gen_record_vars.rs` — the module registered; the
    generator fills `package.spell_formulas` and prints `spell_durations=` / `spell_ranges=` on
    its summary line. `--check` proves the rest of the artifact's bytes are unchanged by it.

  **The oracle side — the half with no production consumer moved, whole:**
  - `src/oracle_validation/class_feature_scaling_bar_check.rs` — **new, +~465, moved not
    rewritten.** `class_feature_corpus_dir_exists`, `load_class_feature_bonus_vars`,
    `find_level_var_alias` and `run_class_feature_bar_check` arrive **unchanged** — same control
    flow, same failure strings, same `BONUS:VAR|` reading through `ingest_record::token_values` —
    together with `ScratchClassFeatureRoot` and the six mutation proofs that drive the real
    function end to end over a synthetic root. This is the address and the reason `AT-35-E6-001`
    used for the `kind=race_trait` FORMULA half
    (`oracle_validation::race_trait_formula_bar_check`), and `run_bar_check` folds this report
    exactly as it folds that one, so the gate's reach is identical. What deliberately did **not**
    move: `parse_class_feature_level_scaling` (a live evaluator with live consumers) and
    `load_class_feature_fixtures` (it reads this repo's own committed fixture, not a corpus
    record).
  - `src/oracle_validation/mod.rs` — the module registered.

  **The data — one regenerated artifact, and not one corpus record rewritten:**
  - `data/converted/record_vars.json` — gains `spell_formulas`: **1,148** settled durations
    (797 refused) and **873** settled ranges (1,068 refused), spanning **10** books.
    **`git status --porcelain data/corpus/` lists nothing**: every ingested record, its licence
    block and its `pi_*` stamps are byte-identical, and no corpus walk, count or licence gate
    moved.

  **Tests:**
  - `pcgen_import::spell_formula_settle::tests::every_ingested_spell_records_duration_and_range_settle_to_what_the_parsers_read`
    — the whole-corpus parity proof: **1,948** records across **10** books re-read the way the
    live side read them until this cycle, compared against the settled tables key for key and
    value for value, **refusals included**, because a refusal that silently became a formula
    would print a fabricated duration on a sheet.
  - `…::no_book_or_record_key_in_this_corpus_contains_the_key_separator` — the key format's own
    precondition, asserted over the real corpus rather than assumed.
  - `…::a_record_is_either_settled_or_refused_never_both` — the two tables partition the records
    carrying the token, which is what keeps the bar check's two failure branches distinct.
  - The six `run_class_feature_bar_check` mutation proofs moved with the check and still drive
    the real function; the two spell scratch harnesses now write the settled artifact by running
    the **real** settling over the scratch corpus they already wrote, so those proofs stay end to
    end: corpus record → settling → bar check, nothing hand-derived in between.
  - `…/AT-35-E6-003-RULED_cycle17_runtime_import_census.py` / `.json` — **new.** Imports cycle
    16's census whole (which imports cycle 15's, … back to cycle 1's), **widens no regex and
    narrows none**, and adds its own assertions by file and symbol.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (`derived_at` restamped by this cycle's `completion_atlas.py --check`). Committed rather than
    filtered away, per the standing "clean tree = unfiltered `git status` empty" rule.

  **`docs/work-inventory.json` is byte-identical**, which is correct: Epic 6 closes zero corpus
  units and no corpus record changed.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff 2fad97f6f5 -- src/ | grep '^+' | grep -v '^+++'`, plus the two new Rust files in
  full), `grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` prints **0**. First run, no
  self-heal. The prescribed tranche-wide form of the same grep
  (`git diff --unified=0 fe5ae6cd4a...HEAD -- <scoped paths>`) prints **11**, every one a
  pre-existing `sd<N>_` test-file name or doc citation landed by an earlier cycle, unchanged here.

- **Wired-integration audit result:** OK_NO_TOKENS, first run, no self-heal. Nothing in this
  cycle's shipping code returns a "would have…" string, serves fixture data, or leaves a handler
  empty: the settling is exercised by a whole-corpus proof on the real corpus, and the moved bar
  check is exercised by the six mutation proofs that moved with it.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`, the criterion this
  `-RULED` variant carries under rulings B15/B16 — `decisions.md` §18, §19):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  The evidence sentence's first clause is **met and held**: `root apps/desktop files=0 hits=0`,
  `apps_desktop_hits=0 evidence_sentence_met=YES`. Its second and third clauses run at the epic
  wrap-up: **this cycle wrote no line under `apps/`** (`git diff --name-only 2fad97f6f5..HEAD --
  apps/` is empty), per §6 step 3 — and that is not an accident of scope. The two public
  functions the desktop spell catalog calls kept their names, their signatures and their return
  types; only what they read underneath changed. The `render_pcgen_desc` clause stays true of the
  live side (`pattern render_pcgen_desc files=0 hits=0`), and the renderer itself stays on the
  converter side, which `decisions.md` §11 requires.

- **Receipt rows (mechanical):**
  ```
  since=2fad97f6f55ef5c784f2392cdb69cb4ddaad91c4 target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=715 ratio=n/a builds_recorded=3 pcgen_live_files=4
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no corpus
  record changed, so `docs/work-inventory.json` is byte-identical before and after.
  `pcgen_live_files=4` is the **file** count, down one from cycle 16's 5 — the fourth consecutive
  cycle to clear a whole file. The **hit** count moved with it: 5 → 4.

- **PCGen residue:** `live_files=4 live_hits=4 baseline_files=260 baseline_hits=12736
  verdict=PASS` — files down 1 from cycle 16's 5, hits down 1 from 5, **and the instrument was
  not touched this cycle** (`git diff --name-only 2fad97f6f5..HEAD -- scripts/` is empty).
  Per-root:
  ```
  root src/rules_core         files=5 hits=5  ->  files=4 hits=4
  root src/saved_character    files=0 hits=0
  root src/campaign           files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop           files=0 hits=0   (unchanged; no Rust line under apps/ was written)
  ```
  Not gate-gaming: nothing renamed to duck a regex, no path exempted, no rebaseline, and the
  ingest-record group's regex is **byte-identical to cycle 16's** — it simply matches nothing
  under a live root any more. The one counted line covered **three** shipping calls, and this
  receipt books them separately (one settled, two moved) rather than netting them against a
  single hit. The `use`-collapse still available in `source_content_payload` was **refused for
  the eleventh cycle running**. The gate's own self-test is green (`Ran 27 tests, OK`).

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no
  converter mapping row changed, and `data/sheet_rules/` is byte-identical
  (`sheet_rule_convert -- --check` → `verdict=PASS`). The parity this change required is the
  whole-corpus settling proof above: every spell record in every ingested book, read both ways,
  compared key for key and value for value with the refusals included, **0 disagreements** on a
  population the test asserts floors for (`records_walked >= 1_900`, `books_walked >= 8`,
  `durations >= 1_000`, `ranges >= 800`).

- **Movement, four buckets:**
  - **closure:** 1 live `pcgen_import` hit and **1 whole file**
    (`derived_evaluator_fixture_check.rs`), asserted by file **and** symbol in the census
    (`closed_by_cycle17=1`, `cleared_files_by_cycle17=1`), plus the three shipping calls behind it
    booked individually: `settled_calls=1` (the two `first_token_value` walks, settled at ingest)
    and `moved_calls=1` (the `token_values` walk, moved to the oracle root with its report). The
    census asserts the file names `pcgen_import`, `ingest_record`, `first_token_value`,
    `token_values` and `raw_tokens` **nowhere** in shipping code rather than inferring it from the
    gate's file list. **Zero corpus units**, as Epic 6 closes none.
  - **relabel:** **none**, and booked explicitly as none (`relabelled_by_cycle17=0`). Nothing
    moved live-file to live-file: the spell reads left the live side for an authoring-time
    producer, and the bar-check half left for a non-live root under `decisions.md` §11's own
    boundary (the converter, the parser and the oracle harness are KEPT, beside each other).
  - **reachability:** none.
  - **instrument-correction:** none. `scripts/pcgen_residue_gate.py` and
    `scripts/pcgen-residue-baseline.env` are absent from this cycle's diff. (The SD-34 atlas
    artifact restamped by `completion_atlas.py --check` is *that* instrument re-deriving its own
    output on an unchanged claim; it closes nothing and is folded, not claimed.)

- **Refused tokens:** `source_content_payload=3, trait_and_pool_tokens=1` — **4 hits / 4 files,
  summing, all under `src/rules_core/`.** Two groups, under this cycle's flag-cap of 10, and one
  group fewer than cycle 16 had. Emitted as a `deferral` retro event (`1789331744215-at-35-e6-003-ruled-e363eb`) naming each mechanism and
  the measured number it is refused on.

- **Discoveries:**
  - **The gate's one counted line hid two different problems, and only one of them was the one
    cycle 16 named.** Cycle 16 filed this hit as "a relocation cycle, not a settling one". Two of
    its three calls were a **settling** problem and were on a **shipping desktop path**:
    `apps/desktop/src-tauri/src/spell_catalog.rs`'s `duration_for()` / `range_for()` are served by
    `all_spell_caster_level_durations` / `_ranges`, so a reader browsing the spell catalog made the
    live side walk `data/corpus/` and read raw `DURATION:` / `RANGE:` tokens on every process
    start. That is the thing `§11` forbids, not a gate's bookkeeping. Logged as a `correction`
    (`1789327410993-at-35-e6-003-ruled-006c94`) with the command that establishes it. The general lesson is ruling B16's own, one level
    further in: **a proxy read as the thing it proxies for loses not only the count but the
    KIND** — one hit, three calls, two different fixes, and the receipt that summarised them as
    one refused the wrong thing.
  - **A bar check is not automatically oracle work, and a production function is not
    automatically live-only.** The same 5,754-line module held both: two functions with a real
    desktop consumer and one with none. Sorting them by *who calls them* rather than by *what
    file they sit in* is what made the file clearable in one cycle — and it is what keeps the move
    honest, because moving the desktop's own readers to a validation root would have been the
    blind-spot shape ruling B16 exists to punish.
  - **The settling had to carry refusals, not only answers.** The old walk's map held every record
    with the token, parsed or not, and the bar check's two failure branches read that difference
    ("no such record in this book's ingest" vs "a token the evaluator reads no formula out of").
    A settled table of parsed formulas alone would have collapsed those into one, and a missing
    ingest would have started reporting as an unparseable token. The two `*_refused` sets keep
    them apart; the only thing deliberately **not** carried across is the raw token text a failure
    message used to quote, because that is ingest vocabulary and a diagnostic string is not worth
    a §11 exception.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=4 live_hits=4`, was `5 / 5` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | 4 hits split `source_content_payload=3, trait_and_pool_tokens=1`; `gate_agreement=OK (4 == 4)` | the same 4 hits, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle17_runtime_import_census.py` |
  | `closed_by_cycle17=1 settled_calls=1 moved_calls=1 relabelled_by_cycle17=0 cleared_files_by_cycle17=1`; `derived_evaluator_fixture_check_shipping_pcgen_import_hits=0 (was 1 counted, 3 calls)`; `spell_formula_settle=present reused_live_parsers=2`; `class_feature_bar_check=oracle_side folded_by_run_bar_check=YES mutation_proofs_moved=7`; cycles 10/11/12/14/15/16's cleared files still clear | the 5 hits at cycle start vs the 4 at HEAD | the census's own row, file and symbol assertions, same command |
  | `settled_spell_durations=1148 refused=797 settled_spell_ranges=873 refused=1068 books=10` | every `data/corpus/<book>/spell/**.json` record carrying a `DURATION:` or `RANGE:` token | the same census command |
  | `desktop_spell_catalog_consumes_settled_tables=YES desktop_shipping_pcgen_import_hits=0` | `apps/desktop/src-tauri/src/spell_catalog.rs`'s shipping lines | the same census command |
  | `spell_durations=1148 spell_ranges=873 … verdict=PASS` (the shipped artifact is not stale) | the whole conversion, re-run and compared byte for byte | `cargo run --locked --bin gen_record_vars -- --check` |
  | the whole-corpus settling proof green, 0 disagreements, over `records_walked=1948` across `books_walked=10` | every `data/corpus/<book>/spell/**.json` record carrying one of the two tokens | `cargo test --locked --lib -j 6` (`pcgen_import::spell_formula_settle::tests::*`) |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` — `data/sheet_rules/` byte-identical | every corpus record the converter reads | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | 0 files under `data/sheet_rules/` carry an ingest-format literal | the whole `data/sheet_rules/` tree | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | 3,383 lib tests pass, 16 ignored (cycle 16 recorded 3,380 passing; the delta is **exactly** this cycle's three new whole-corpus settling tests — the six moved `class_feature` mutation proofs are counted in the lib either side of the move) | the crate's own unit tests | `cargo test --locked --lib -j 6` |
  | 8,912 tests pass across 419 targets + 1 Doc-tests, 0 failed, 69 ignored (cycle 16 recorded 8,909; the same three tests, reached through the lib) | every target in the root workspace | `cargo test --locked --no-fail-fast -j 6` |
  | clippy clean: 0 warnings, 0 errors, after **one self-healed warning** (`empty line after doc comment` at `derived_evaluator_fixture_check.rs:1940` — a blank line this cycle's own insertion left between `ScratchRangeRoot`'s doc comment and the struct; the blank line was removed and clippy re-run to 0) | every target and every test target in the root workspace | `cargo clippy --locked --tests -j 6 2>&1 \| grep -cE '^(warning\|error)'` |
  | `citation_failures=0`; `non_done=0 refused=142 verdict=PASS`; `not_held_by_engine=0`; `population=0`; `files_checked=146 violations=0` | the atlas, token coverage, the shape/engine boundary, the missing-tables population, and this package's own prose | `completion_atlas.py --check`, `token_coverage.py --check`, `shape_engine_boundary.py --check`, `missing_engine_tables.py --check`, `denominator_gate.py --check …` |
  | the residue gate's own self-test green, `Ran 27 tests, OK` — B15 and B16 still pinned | the gate's test suite | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, once, at the
  final tree: `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`; `cargo test --locked --lib
  -j 6` → 3,383 passed / 0 failed / 16 ignored; `cargo test --locked --no-fail-fast -j 6` →
  `FULL_EXIT=0`, 8,912 passed / 0 failed / 69 ignored across 419 targets + Doc-tests.
  **ONE verification pass** — `decisions.md` §3 as written. Two things landed after it and
  neither moves a figure: the one-line clippy self-heal above (a blank line inside a doc comment)
  and this cycle's own test-floor correction (`records_walked >= 2_000` → `>= 1_900`, the only red
  in the pass — my own floor, stated above the real population of 1,948; every parity assertion in
  that test was green). The final tree was re-proved with `cargo test --locked --no-run -j 6`
  (exit 0) and `cargo clippy --locked --tests -j 6` (0 warnings) at the end. The desktop crate and
  frontend run at the **epic wrap-up**, not here, because this cycle wrote no line under `apps/`
  (`git diff --name-only 2fad97f6f5..HEAD -- apps/` is empty).

- **Sweep population:** N/A — `corpus_literal_sweep` did **not** run, and correctly so: it is
  required "only when corpus records changed", and **no file under `data/corpus/` was written**
  (`git diff --name-only 2fad97f6f5..HEAD -- data/corpus/` is empty). The one `data/` file this
  cycle changed is the generated `data/converted/record_vars.json`, which carries no corpus
  record, no licence block and no `pi_*` stamp. For the same reason
  `cargo run --locked --bin v06_work_inventory` did not run: with no corpus record changed the
  inventory cannot move, and `docs/work-inventory.json` is byte-identical before and after.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.

- **Status:** `partial`.

- **Notes:** The `source_content_payload` trim — repointing an import at
  `rules_core::source_content`'s own re-export of the same enum, for `−1` and zero change in what
  the module depends on — is **refused for the eleventh cycle running**, on the same reasoning
  cycles 7 through 16 gave.

- **Next-cycle scope:** **AT-35-E6-003-RULED cycle 18**, `SCOPE_GATE: EXEMPT (Epic 6 cycle)`, on
  the 4-hit remainder, and the remainder is now **two pieces, both measured, neither cheap.**
  **`source_content_payload` (3)** clears only when the enum itself moves to the live side, and it
  cannot while four of its seven variants borrow a parser entry type (`Class`,
  `SpellcastingClass`, `Race`/`Ability`, `Metadata`). The measurement this cycle adds to that
  refusal: those four variants are constructed **only** by `pcgen_import::ir_converter` and
  matched **only** by `tests/sd17_{c,d,e}` — no live file matches one — but
  `pcgen_import::pcc_package_loader::project_corpus_from_owned` builds a MIXED package that
  `rules_core::composed_input::compose` consumes, and `tests/sd18_preloop_consumer_compose.rs`
  asserts a Class record and a Race record are in it, so the four cannot simply be dropped from
  the live envelope. The two admissible paths, in order of cost: convert each of the four kinds to
  a live-owned settled record the way cycles 8/10/13 converted spell and equipment (one kind per
  cycle, four cycles), or split the envelope's payload type so the converter's own IR carries the
  parser-borrowing variants and the live envelope carries only what `corpus_loader` produces
  (one cycle, but it re-types `SourceContentRecord`/`SourcePackageContent` and touches
  `ir_converter` plus the three `sd17_*` integration tests). **`trait_and_pool_tokens` (1,
  `class_feature_pool_catalog`)** stays refused on cycle 7's corrected number (1,870 of 18,043
  P1, 864 P2, 89 P3).
