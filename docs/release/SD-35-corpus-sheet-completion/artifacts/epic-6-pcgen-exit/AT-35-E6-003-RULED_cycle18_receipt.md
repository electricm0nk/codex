# Cycle AT-35-E6-003-RULED cycle 18 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `PENDING` (two commits: the envelope-payload split, then the pool-gate
  settling with the regenerated converted artifact, the cycle-18 census script and its JSON, one
  retro `correction`, and the folded shared-checkout artifact), cycle start `5bd0eda548`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `5bd0eda548`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 17's closing figure — nothing drifted between the two cycles:
  ```
  live_files=4 live_hits=4 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **This cycle closed the criterion's whole remainder — `live_files=0 live_hits=0`.** Both named
  refusal groups, four hits, four files. Neither was cheap and neither was gate-gaming; each is
  described below with the command that re-derives it.

## 1. `source_content_payload` (3 hits, 3 files) — the type was SPLIT, not moved

- `src/rules_core/source_content.rs` — the `pub use crate::pcgen_import::source_content_payload::
  SourceContentPayload;` is **gone**, and with it the reason the two resolvers named the converter
  at all. The module now declares its **own** `SourceContentPayload`, naming only rules-core types:
  `Spell(&CorpusSpellRecord)` (settled since cycle 8), `Equipment(&CorpusEquipmentRecord)` (cycles
  10 and 13), and `Unsettled { kind, name }` — the record's kind tag and its canonical name, which
  are identity, never ingest vocabulary. `kind_token()` / `source_slice()` keep their exact
  answers; `unsettled_name()` is the one addition.
- `src/rules_core/source_content.rs` (the envelope) — `SourceContentRecord`,
  `SourcePackageContent` and `SourceContentLoadResult` are **generic over the payload, defaulting
  to the live one**. That default is the whole trick: `SourceContentRecord<'a>` still means what
  it meant, so **not one of the ~80 files that name the envelope was written**. A private
  `PhantomData<&'a ()>` ties the lifetime for instantiations whose payload does not borrow; no
  caller constructs the struct literally (`grep -rn "SourceContentRecord *{" src/ tests/ apps/`
  prints nothing), so the private field costs no call site.
- `src/rules_core/spell_resolver.rs`, `src/rules_core/equipment_resolver.rs` — **one import line
  each, repointed at `rules_core::source_content`.** These are the two hits this criterion refused
  to "trim" for eleven cycles, and they are closed here for the right reason: the type they import
  is now a live type. Both files match only `Spell` / `Equipment`, which is why the narrowing costs
  nothing.
- `src/pcgen_import/ir_content_payload.rs` — **renamed from `source_content_payload.rs`**
  (`git mv`, so the history follows). The enum is `IrContentPayload` and **keeps all seven
  variants**, every parser borrow intact: `Class(&ClassEntry)`,
  `SpellcastingClass(&SpellcastingClassEntry)`, `Race(&RaceDeclaration)`,
  `Ability(&AbilityDeclaration)`, `Spell`, `Equipment`, `Metadata(&LstRecord)`. Added: the
  `IrContentRecord` / `IrPackageContent` aliases (the same envelope, instantiated with this
  payload), `canonical_name()`, and the boundary projection `to_live` / `record_to_live` /
  `package_to_live`. `b6_metadata_kind_to_canonical` and its inverse are unchanged.
- `src/pcgen_import/ir_converter.rs` — retyped to the converter's own envelope.
  `convert_spell_record` / `convert_equipment_record` **still return the live envelope** (their
  payloads are live-owned records already), so the 51 files that push their result into a live
  package needed no edit; `convert_spell_record_ir` / `convert_equipment_record_ir` are the IR
  siblings the per-document converters use.
- `src/pcgen_import/pcc_package_loader.rs` — `project_corpus_from_owned` accumulates an
  `IrPackageContent` and hands it to `package_to_live` **once, at the return**. That is the single
  boundary a live consumer sees: `rules_core::composed_input::compose` is unchanged, and
  `tests/sd18_preloop_consumer_compose.rs`'s "one Class record and one Race record" assertion
  reads `r.kind`, which survives the projection exactly.

**Nothing was deleted and no proof was lost.** `tests/sd17_c`, `sd17_d` and `sd17_e` still exercise
every one of the seven variants and read the parser entry's own fields through them; the only
change in those files is the enum's name, plus five calls repointed at the `_ir` converters so
they keep proving the IR envelope rather than the live one. `decisions.md` §11's KEPT list —
converter, parser, generators, oracle harness — is intact.

## 2. `trait_and_pool_tokens` (1 hit, 1 file) — settled at ingest

- `src/rules_core/class_feature_pool_catalog.rs` — the top-level
  `use crate::pcgen_import::pool_member_tokens;` is gone. Behind that one counted line sat **three
  gates in shipping code**, asked of the corpus row itself on every process start, on the way to a
  character sheet: `has_no_engine_effect_token` (does the row carry `ABILITY`/`BONUS`/`SELECT`/…),
  `is_archetype_locked` (does a `PREABILITY` name `CATEGORY=Archetype`), and the multi-`DESC:`
  pair. Each is a question about the **ingest format**, which `§11` says is answered once, at
  ingest, into our own schema.
- `src/pcgen_import/pool_gate_settle.rs` — **new, +~245.** The settling scan: the same recursive
  `data/corpus/<book>/class_feature/` descent the catalog performs, applying the **same four
  predicates in the same order** — it does not restate a gate, it calls the ones the catalog
  called. Verdicts are `admitted` or `refused` **with the name of the first gate that refused**,
  because "this record carries a mechanic" and "this record's `DESC:` segments are a branch set"
  are different answers and the catalog's own census reports them separately.
- `src/rules_core/class_feature_pool_catalog.rs` — declares `SettledPoolGates`
  (`admitted: BTreeSet`, `refused: BTreeMap<key, reason>`), `settled_pool_gate_key(book, key)` and
  its separator, beside the walk that consumes them. The shipping walk's three `if` blocks became
  one: `if !pool_gates.admits(&book, key) { continue; }`. The table is **fail-closed** — `admits`
  is false for a key it does not hold — so a missing or stale artifact serves **fewer** pool
  options, never an unvetted one. That is the same direction the converted-prose join already
  fails in.
- `src/rules_core/record_vars.rs`, `src/bin/gen_record_vars.rs` — `RecordVarPackage` gains
  `pool_gates: SettledPoolGates`, `#[serde(default)]` like its `spell_formulas`, `desc_templates`
  and `desc_arguments` siblings; **no existing field was added to, removed from or renamed.** The
  generator fills it and prints `pool_gates_admitted=` / `pool_gates_refused=`.
- The `pool_member_tokens` calls that remain are inside `#[cfg(test)]`, and the `use` moved in
  there with them. That is not a dodge, it is the point: a `#[cfg(test)]` region is not live code
  (`decisions.md` §18 / ruling B15), and asking the converter's predicates **directly** is exactly
  what makes the catalog's own census an independent check on the settled table rather than a
  restatement of it. The census asserts the kept import is still there
  (`cfg_test_independent_check=KEPT`), so a later cycle cannot quietly delete the cross-check.

## The data, and what did not move

- `data/converted/record_vars.json` — gains `pool_gates`: **5,813** admitted and **6,924** refused
  across **23** books, the refusals split `engine_effect_token_present=6459`,
  `multi_desc_segment_not_regenerated=387`, `archetype_locked=78`.
- **`git status --porcelain data/corpus/` lists nothing**: every ingested record, its licence block
  and its `pi_*` stamps are byte-identical, and no corpus walk, count or licence gate moved.
- **`docs/work-inventory.json` is byte-identical** (`cmp`), which is correct: Epic 6 closes zero
  corpus units and no corpus record changed.
- **Folded from the shared checkout, not authored work:**
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (`derived_at`
  restamped by this cycle's `completion_atlas.py --check`). Committed rather than filtered away,
  per the standing "clean tree = unfiltered `git status` empty" rule.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS, after **one self-heal**. Over this cycle's own
  added lines (`git diff 5bd0eda548 -- src/ tests/ | grep '^+' | grep -v '^+++'`, plus the new Rust
  file in full), the first run printed **2** — both doc-comment citations of the real integration
  test files (`` `sd17_c` / `sd17_d` / `sd17_e` ``) in the two new module headers. Reworded to
  "slice-E integration suites" and re-run to **0**. The prescribed tranche-wide form of the same
  grep over the broader epic path set prints 168, every one a pre-existing `sd<N>_` test-file name
  or doc citation landed by an earlier cycle, unchanged here.

- **Wired-integration audit result:** OK_NO_TOKENS, first run, no self-heal. Nothing in this
  cycle's shipping code returns a "would have…" string, serves fixture data, or leaves a handler
  empty: the split is exercised by the boundary tests and by the unchanged slice-E suites, and the
  settling by a whole-corpus proof on the real corpus.

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
  wrap-up: **this cycle wrote no line under `apps/`** (`git diff --name-only 5bd0eda548..HEAD --
  apps/` is empty), per §6 step 3 — and that is not an accident of scope. The desktop crate's only
  reader of the payload enum, `apps/desktop/src-tauri/src/corpus_fixtures.rs`, already imported it
  from `codex::rules_core::source_content` and matches only `Spell` and `Equipment`, so the
  narrowing reached it without a line being written. The `render_pcgen_desc` clause stays true of
  the live side (`pattern render_pcgen_desc files=0 hits=0`), and the renderer itself stays on the
  converter side, which `decisions.md` §11 requires.

- **Receipt rows (mechanical):**
  ```
  since=5bd0eda548183a2e739985806c7e6fd79c442607 target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1165 ratio=n/a builds_recorded=2 pcgen_live_files=0
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no corpus
  record changed, so `docs/work-inventory.json` is byte-identical before and after.
  **`pcgen_live_files=0`** is the criterion's own population, and it is the first cycle to print
  it: 4 → 0 files, 4 → 0 hits.

- **PCGen residue:** `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736
  verdict=PASS` — **and the instrument was not touched this cycle**
  (`git diff --name-only 5bd0eda548..HEAD -- scripts/` is empty). Per-root:
  ```
  root src/rules_core         files=4 hits=4  ->  files=0 hits=0
  root src/saved_character    files=0 hits=0
  root src/campaign           files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop           files=0 hits=0   (unchanged; no Rust line under apps/ was written)
  ```
  Not gate-gaming: nothing was renamed to duck a regex (the renamed module is the **converter's**,
  and the gate matches the string `pcgen_import`, which that module's path still is), no path was
  exempted, no rebaseline, and every regex in the cycle-18 census is byte-identical to cycle 17's.
  The gate's own self-test is green (`Ran 27 tests, OK`).

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no converter
  mapping row changed, and `data/sheet_rules/` is byte-identical (`sheet_rule_convert -- --check`
  → `verdict=PASS`). The parity this change required is two proofs on the real corpus:
  - the whole-corpus pool-gate settling proof — every `class_feature` record carrying a
    description, in every ingested book, re-read through the converter's own four predicates in
    the catalog's own order and compared against the settled table **record for record, refusal
    reasons included**, **0 disagreements**, on a population the test asserts floors for
    (`books_walked >= 8`, `records_walked >= 10_000`);
  - the boundary projection proof — provenance, kind tag, `kind_token()` and `source_slice()`
    preserved for every kind across `record_to_live`, and record order plus diagnostics preserved
    across `package_to_live`.

- **Movement, four buckets:**
  - **closure:** **4 live hits and 4 whole files** — the criterion's entire remaining population.
    Asserted by file **and** symbol in the census (`closed_by_cycle18=4`,
    `cleared_files_by_cycle18=4`), split by mechanism rather than netted: `split_by_cycle18=1`
    (the payload type) and `settled_by_cycle18=1` (the pool gates). The census asserts each of the
    four files names `pcgen_import` **nowhere** in shipping code, rather than inferring it from
    the gate's file list, and re-asserts that every file cycles 10–17 cleared is still clear
    (`earlier_cleared_files_still_clear=YES`). **Zero corpus units**, as Epic 6 closes none.
  - **relabel:** **none**, and booked explicitly as none (`relabelled_by_cycle18=0`). Neither
    closure moved a read from one live file to another: one split a type so the live half names no
    parser entry, the other moved a run-time reading to authoring time.
  - **reachability:** none.
  - **instrument-correction:** none. `scripts/pcgen_residue_gate.py` and
    `scripts/pcgen-residue-baseline.env` are absent from this cycle's diff. (The SD-34 atlas
    artifact restamped by `completion_atlas.py --check` is *that* instrument re-deriving its own
    output on an unchanged claim; it closes nothing and is folded, not claimed.)

- **Refused tokens:** **none.** `refused_tokens=none`, `criterion_population=0`,
  `closure_reached=YES`. No `deferral` event is emitted, because nothing is deferred.

- **Discoveries:**
  - **The refusal had been priced against an assumption nobody had tested: that the envelope must
    keep one payload type.** Cycles 7–17 all refused the `source_content_payload` trim on the
    same reasoning — the enum cannot move because four variants borrow parser entries — and cycle
    17 priced the real fix at four cycles, or at one cycle that "re-types
    `SourceContentRecord`/`SourcePackageContent`". Both readings are correct *if* the envelope
    carries a single payload. Giving the envelope a payload type parameter **whose default is the
    live payload** re-typed nothing: every one of the ~80 files naming `SourceContentRecord<'a>`
    is absent from this cycle's diff. Logged as a `correction`
    (`1789333434263-at-35-e6-003-ruled-7653e7`). The general lesson, and it is the sharper half of
    `AGENTS.md`'s "challenge the category": **a refusal carried forward unchanged for eleven
    cycles stops being a measurement and becomes a premise.** Each cycle re-derived the *number*
    honestly and never re-derived the *shape of the fix*.
  - **A one-line gate hit can still be a shipping defect, twice in two cycles.** Cycle 17 found
    three shipping calls behind one counted `use`; cycle 18 found three shipping gates behind
    another. `class_feature_pool_catalog`'s three predicates ran on **every** corpus row the pool
    walk considered, on every process start — a reader opening a rogue-talent picker was making
    the live side read raw `ABILITY:` / `PREABILITY:` / `DESC:` token text. The gate counted that
    as **one**. Ruling B16's own lesson, one level further in: a proxy read as the thing it
    proxies for loses the magnitude as well as the kind.
  - **A settled gate must fail in the direction the un-settled one did.** The old gates were
    refusals: a row that failed one was dropped from the pool. If the settled table had been "a
    set of refused keys", a missing artifact would have **admitted** every record the gates exist
    to refuse — mechanic-bearing and archetype-locked options served as prose-only on a sheet.
    Carrying the admitted set instead, and answering `admits` from it, makes a missing artifact
    serve an empty pool. The direction of a gate's failure is part of the gate.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=0 live_hits=0`, was `4 / 4` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | `refused_tokens=none`; `gate_agreement=OK (0 == 0)`; `criterion_population=0 closure_reached=YES` | the same population, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle18_runtime_import_census.py` |
  | `cleared_files_by_cycle18=4 closed_by_cycle18=4 split_by_cycle18=1 settled_by_cycle18=1 relabelled_by_cycle18=0`; `earlier_cleared_files_still_clear=YES` | the 4 hits at cycle start vs the 0 at HEAD | the census's own row, file and symbol assertions, same command |
  | `live_payload_variants=3 ir_payload_variants=7 boundary_projection=present parser_types_on_live_side=0`; `live_resolvers_import_live_payload=YES` | the two payload enums and the two resolvers' shipping lines | the same census command |
  | `pool_gates_admitted=5813 pool_gates_refused=6924 books=23`, reasons `engine_effect_token_present=6459, multi_desc_segment_not_regenerated=387, archetype_locked=78` | every `data/corpus/<book>/class_feature/**.json` record carrying a `description` | the same census command |
  | `pool_catalog_reads_settled_verdict=YES pool_catalog_shipping_pcgen_import_hits=0 cfg_test_independent_check=KEPT` | `src/rules_core/class_feature_pool_catalog.rs`'s shipping lines | the same census command |
  | `pool_gates_admitted=5813 pool_gates_refused=6924 … verdict=PASS` (the shipped artifact is not stale) | the whole conversion, re-run and compared byte for byte | `cargo run --locked --bin gen_record_vars -- --check` |
  | the whole-corpus pool-gate proof green, 0 disagreements | every `class_feature` record with a description, in every ingested book | `cargo test --locked --lib -j 6` (`pcgen_import::pool_gate_settle::tests::*`) |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` — `data/sheet_rules/` byte-identical | every corpus record the converter reads | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | 0 files under `data/sheet_rules/` carry an ingest-format literal | the whole `data/sheet_rules/` tree | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | 3,390 lib tests pass, 16 ignored (cycle 17 recorded 3,383; the delta is **exactly** this cycle's seven new tests — three boundary-projection proofs and four pool-gate settling proofs) | the crate's own unit tests | `cargo test --locked --lib -j 6` |
  | 8,919 tests pass across 419 targets + 1 Doc-tests, 0 failed, 69 ignored (cycle 17 recorded 8,912; the delta is **exactly** this cycle's seven new tests, reached through the lib) | every target in the root workspace | `cargo test --locked --no-fail-fast -j 6` |
  | clippy clean: **0 warnings, 0 errors, first run, no self-heal** | every target and every test target in the root workspace | `cargo clippy --locked --tests -j 6 2>&1 \| grep -cE '^(warning\|error)'` |
  | `citation_failures=0`; `non_done=0 refused=142 verdict=PASS`; `not_held_by_engine=0`; `population=0`; `files_checked=148 violations=0` | the atlas, token coverage, the shape/engine boundary, the missing-tables population, and this package's own prose | `completion_atlas.py --check`, `token_coverage.py --check`, `shape_engine_boundary.py --check`, `missing_engine_tables.py --check`, `denominator_gate.py --check …` |
  | the residue gate's own self-test green, `Ran 27 tests, OK` — B15 and B16 still pinned | the gate's test suite | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, once, at the
  final tree. **ONE verification pass** — `decisions.md` §3 as written. The desktop crate and
  frontend run at the **epic wrap-up**, not here, because this cycle wrote no line under `apps/`
  (`git diff --name-only 5bd0eda548..HEAD -- apps/` is empty). The pass, in order, at the final tree:
  `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`; `cargo test --locked --lib -j 6` →
  3,390 passed / 0 failed / 16 ignored, `LIB_EXIT=0`; `cargo test --locked --no-fail-fast -j 6` →
  8,919 passed / **0 failed** / 69 ignored across **419** targets + Doc-tests, `FULL_EXIT=0`;
  `cargo clippy --locked --tests -j 6` → 0 warnings, 0 errors, `CLIPPY_EXIT=0`;
  `cargo run --locked --bin sheet_rule_convert -- --check` →
  `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS`;
  `cargo run --locked --bin gen_record_vars -- --check` → `verdict=PASS`. Three compile failures
  were self-healed **before** the pass, not during it (`tests/sd17_c`, `sd17_d` and `sd17_e`
  naming the renamed enum against the wrong envelope instantiation, and one import list that had
  lost `SourceContentSeverity`); the pass above ran once, whole, on the tree those fixes produced.

- **Sweep population:** N/A — `corpus_literal_sweep` did **not** run, and correctly so: it is
  required "only when corpus records changed", and **no file under `data/corpus/` was written**
  (`git diff --name-only 5bd0eda548..HEAD -- data/corpus/` is empty). The one `data/` file this
  cycle changed is the generated `data/converted/record_vars.json`, which carries no corpus
  record, no licence block and no `pi_*` stamp. For the same reason
  `cargo run --locked --bin v06_work_inventory` did not run: with no corpus record changed the
  inventory cannot move, and `docs/work-inventory.json` is byte-identical before and after.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.

- **Status:** `complete`. The criterion's population is **zero at HEAD**:
  `live_files=0 live_hits=0 verdict=PASS`, `criterion_population=0 closure_reached=YES`, and the
  evidence sentence's `apps/desktop` clause is met (`apps_desktop_hits=0
  evidence_sentence_met=YES`). Nothing is filed, deferred or forwarded.

- **Notes:** The trim that eleven cycles refused — repointing the two resolvers' import at
  `rules_core::source_content`'s re-export, for `−2` and zero change in what the modules depend on
  — was **never taken**. The re-export it pointed at no longer exists; the import the two
  resolvers now carry names a type `rules_core` itself declares.

- **Next-cycle scope:** none for this criterion. The remaining Epic 6 work is `AT-35-E6-004` (the
  gate reads zero, `--check --closure`), whose bar this cycle's figure now meets across all five
  live roots, and the epic wrap-up, which is where the desktop crate and frontend suites run.
