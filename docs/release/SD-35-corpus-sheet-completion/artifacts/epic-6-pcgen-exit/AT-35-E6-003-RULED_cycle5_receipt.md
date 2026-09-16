# Cycle AT-35-E6-003-RULED cycle 5 — Epic 6 PCGen exit / AT-35-E6-003-RULED

- **Commit SHA:** `5d80721087` (the code), `8ca7a923fb` (this receipt, the census and the
  retro events), cycle start `fe0a51417e`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `fe0a51417e`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 4's closing figure — nothing drifted between the two cycles:
  ```
  live_files=19 live_hits=37 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/rules_core/sheet_rule.rs` — **+25 lines.** New index on `SheetRulePackage`,
    `by_closure_row: BTreeMap<String, Vec<RuleId>>`, built in `finish()` beside the three
    indexes already there, with one public accessor:
    `rules_for_closure_row(path, line) -> &[RuleId]`. **This is the join a live caller holding
    a `data/corpus/**/*.json` record needs to ask the converted package what that record
    became.** The corpus record states its own origin row (`source.path`, `source.line`); every
    rule the converter wrote from that row already names the same `path:line` in
    `provenance.closure_rows`. Nothing indexed it, so nothing could use it.
  - `src/rules_core/trait_pool.rs` — **+105 lines, −12.** The join's first consumer. The
    module read a corpus record's `TYPE:` token array through
    `pcgen_import::ingest_record::type_token_suffix` and held the ingest prefix string
    `"Trait.RaceTrait."` in live code as `RACE_TRAIT_TYPE_PREFIX`. Both are gone. The pool name
    now comes off the converted rule's `tags` — the converter writes the whole `TYPE:` chain
    out as tags, so `["Trait","RaceTrait","<X> Race Trait"]` yields `<X> Race Trait` and
    `["Trait","BasicTrait","RaceTrait","BloodlineTrait"]` correctly yields `None`. `use
    crate::pcgen_import::ingest_record;` is deleted; the module names no ingest vocabulary at
    all. `load_trait_pool`'s public signature is unchanged (it reads
    `corpus_loader::live_sheet_rules()`, the established pattern for a `rules_core` caller with
    no package to pass); `load_trait_pool_with_rules(roots, Option<&SheetRulePackage>)` is the
    explicit-package form. **With no converted package the pool is empty and says so — it never
    falls back to reading the ingest format.**
  - `…/AT-35-E6-003-RULED_cycle5_runtime_import_census.py` / `.json` — **new.** Imports cycle
    4's census whole (which imports cycle 3's, which imports cycle 2's, which imports cycle
    1's) and rewrites two groups' reasons with this cycle's measurements.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`, 1 `incident`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from this cycle's own instruments, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one
    field, `derived_at`, restamped by this cycle's `completion_atlas.py --check`) and
    `docs/retro/events/sd31-transcribe.jsonl` (this cycle's `verify.sh --only pi-sweep` event,
    misfiled under the wrong actor — see Discoveries). Committed rather than filtered away, per
    the standing "clean tree = unfiltered `git status` empty" rule.

  **No `data/` file and no corpus record was changed**, so `data/sheet_rules/` and
  `docs/work-inventory.json` are byte-identical to the cycle's start tree
  (`diff -q <(git show fe0a51417e:docs/work-inventory.json) docs/work-inventory.json` → no
  difference). **`apps/` was NOT touched**, so the desktop crate runs at the epic wrap-up
  (`§6` step 3), not here.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff --unified=0 fe0a51417e -- src/rules_core/ | grep '^+'`),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing.

- **Wired-integration audit result:** OK_NO_TOKENS, first run, no self-heal. The same added
  lines under `grep -nEi '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  return nothing. No `"Would …"` string, no inline mock, no fixture-only data path: the new
  index is built from the live package and its only consumer is a shipping loader the desktop
  picker calls.

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
  first met in cycle 4 and not regressed here — this cycle wrote no `apps/` file). The criterion
  as a whole is **not** met: 36 hits across 18 files remain under `src/rules_core/`, and
  `render_pcgen_desc_with_values` is still called there.

- **Receipt rows (mechanical):**
  ```
  since=fe0a51417e1adde5bb002b3cab33d8e35707e1e1 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=200 ratio=n/a builds_recorded=1 pcgen_live_files=18
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.

- **PCGen residue:** `live_files=18 live_hits=36 baseline_files=260 baseline_hits=12736
  verdict=PASS` — down from cycle 4's `19 / 37` on both axes, and **the instrument was not
  touched this cycle** (`git diff --name-only fe0a51417e..HEAD -- scripts/` is empty), so the
  `−1 / −1` is entirely code. Per-root:
  ```
  root src/rules_core        files=19 hits=37  ->  files=18 hits=36
  root src/saved_character   files=0  hits=0
  root src/campaign          files=0  hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop          files=0  hits=0   (unchanged — no apps/ file written)
  ```
  **One hit, one whole file.** Not gate-gaming by the test cycle 3 set — nothing was renamed to
  duck a regex, no path exempted, no `use` collapsed. The shipping behaviour changed: the
  adopted-race trait pool a player picks from is now read out of `data/sheet_rules/` and is
  empty-and-honest without it, where before it was read out of the corpus record's PCGen token
  array.

- **Oracle parity:** N/A for the PCGen oracle — no `Number` mapping was added and no rendered
  value changed. The **converted-vs-retired-read** parity, which is the measurement this swap
  turns on, is `records=487 agree=487 disagree=0 unresolved=0` over the live
  `data/corpus/*/trait_generic/` population; the retired ingest-token read is computed in-test
  as the oracle, so the two cannot drift silently. **Mutation-proved:** changing `tags[2]` to
  `tags[1]` makes it red with `20 of 487 records disagree`, naming real records
  (`trait_animal_friend` → `converted=Some("RaceTrait") ingest_tokens=Some("Gnome Race Trait")`).

- **Movement, four buckets:**
  - **closure:** **none in corpus units** (Epic 6 closes zero by design). On the B16 population:
    `src/rules_core` `37 → 36` hits and `19 → 18` files; by group, `ingest_record_tokens`
    `7 → 6`. `trait_pool.rs` leaves the census entirely.
  - **relabel:** none. No hit moved between files or groups.
  - **reachability:** none — no rendered sheet line moved. The adopted-race pool resolves to the
    same 120 pooled records from the same 487, by a different source of truth.
  - **instrument-correction:** **none.** The gate script, its baseline file and its patterns are
    untouched; the census's group table is imported from cycle 4, not rewritten. Two group
    *reasons* were rewritten — that is a corrected explanation, and it moved no number.

- **Refused tokens:** **36 hits across 18 files, six groups**, the same six as cycle 4:
  ```
  renderer=5, lst_parser_types=12, ingest_record_tokens=6, trait_and_pool_tokens=4,
  ir_converter=4, source_content_payload=5
  ```
  `5+12+6+4+4+5 = 36`. Six groups, under `§8`'s limit of ten. Recorded as
  `deferral 1789261516421-at-35-e6-003-ruled-2bea07`; every line named with file, line and
  reason in `…_cycle5_runtime_import_census.json` and re-derivable by its script.

  **Why each group did not go.** `renderer` (5) is refused by cycle 2's measurement, not by
  difficulty: the converted candidate exists, runs, and disagrees with the live path on 97,332
  of 660,320 renderings across 2,443 record keys, every shape of it converter-side.
  `lst_parser_types` (12) + `ingest_record_tokens` (6) + `ir_converter` (4) +
  `source_content_payload` (5) = **27 hits are one piece of work**, and **the new join does not
  move them**: those callers do not want a *fact about* a record, they **own**
  `EquipmentRecord` / `LstSpellRecord` / `SourceContentPayload` as their own data types across
  13 files and read the PCGen `BONUS:` chains and `KEY:VAL` tokens on them directly. That clears
  when `sheet_rule_convert` emits an equipment/equipment-modifier rule shape
  `equipment_effects` can read, not when a lookup exists. `trait_and_pool_tokens` (4) is
  unchanged and is stated, not netted: this cycle's one cleared hit was classified under
  `ingest_record_tokens`, not here.

- **Discoveries:** two, emitted as retro events.
  - `1789261501792-at-35-e6-003-ruled-0dfa46` (`correction`) — **cycle 4's census was wrong
    about why the per-record token readers were stuck, and wrong in the direction cycle 4 itself
    named as the family's recurring error.** The recorded reason was that they need "the
    converted package to carry the same facts keyed by `VarId`", which "does not exist yet". The
    package already carries them. **What was missing was the join.** `find(kind, slug)` joins on
    the converter's `slug(name)` file name, which is not the corpus file name — measured over
    `kind: trait`, **131 of 487** corpus records do not resolve (the `codex_named_unit_*` rows,
    whose corpus key is synthetic). The record's own `source.path`/`source.line` does resolve:
    **8,486 of 8,486** records across all seven Epic-2 kinds (`ability`, `template`, `trait`,
    `deity`, `domain`, `skill`, `language`). Three cycles priced a whole group as blocked on
    converter work; part of it was blocked on a `BTreeMap` nobody had built. The lesson is the
    same one cycle 4 wrote down and is worth writing again in its general form: **a group's
    stated reason is an assertion about a dependency, and an assertion about a dependency has to
    be checked against what the caller needs, not against the module it names.**
  - `1789263208882-at-35-e6-003-ruled-93b3af` (`incident`, recurrence key
    `retro-actor-not-exported`) — **the same misfiled-actor failure as cycle 4, one cycle
    later.** `scripts/verify.sh --only pi-sweep` emitted its own derived event into
    `docs/retro/events/sd31-transcribe.jsonl` because `RETRO_ACTOR` does not survive between
    this harness's `Bash` calls. Cycle 4 recorded it and wrote the control down as a habit
    ("export in the same call"); **a habit is not a control** (`AGENTS.md` rule 8) and it did
    not survive one cycle. The mechanical fix is one condition in `scripts/verify.sh`: refuse to
    emit a retro event when `RETRO_ACTOR` is unset, rather than silently inheriting a stale
    default. Named here rather than left to look like another session's event.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=18 live_hits=36 baseline_files=260 baseline_hits=12736 verdict=PASS` | every source file (`.rs .ts .tsx .js .jsx .mjs .cjs`) under the five live roots, comment lines excluded (B14) and `#[cfg(test)]` regions excluded (B15) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `root src/rules_core files=18 hits=36`; `root apps/desktop files=0 hits=0` | the same, restricted to that root | `python3 scripts/pcgen_residue_gate.py --check` |
  | `pcgen_import_hits=36 files=18`; `apps_desktop_hits=0 evidence_sentence_met=YES`; the six group sizes | the shipping lines under the five live roots naming `pcgen_import` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle5_runtime_import_census.py` |
  | the gate script and its baseline are absent from this cycle's diff | `scripts/` | `git diff --name-only fe0a51417e1adde5bb002b3cab33d8e35707e1e1..HEAD -- scripts/` |
  | `records=487 agree=487 disagree=0 unresolved=0` (converted pool vs the retired ingest-token read) | every `data/corpus/*/trait_generic/**/*.json` record | `cargo test --locked --lib -j 6 -- --nocapture rules_core::trait_pool::tests::every_live_trait_record_gets_the_same_pool_from_the_converted_package` |
  | the same test red with `20 of 487 records disagree` under `tags[2]` → `tags[1]` | the same | the mutation, applied and reverted in-cycle |
  | `records=8486 resolved=8486` for the closure-row join across `ability`, `template`, `trait`, `deity`, `domain`, `skill`, `language` | every corpus record of the seven Epic-2 kinds | the python measurement quoted verbatim in the cycle-5 census script's module docstring |
  | `131 of 487` `kind: trait` corpus records unresolved by the `find(kind, slug)` join | the same 487 | the same measurement |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` (115.1 s) | the whole converted package | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `0` files under `data/sheet_rules/` carrying ingest syntax | the whole converted package | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | `stale_derived_at=False citation_failures=0` | the completion atlas | `python3 scripts/completion_atlas.py --check` |
  | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS` | token coverage | `python3 scripts/token_coverage.py --check` |
  | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | shape/engine boundary | `python3 scripts/shape_engine_boundary.py --check` |
  | `population=0 kinds=0 citation_failures=0` | missing engine tables | `python3 scripts/missing_engine_tables.py --check` |
  | `files_checked=135 violations=0` | the bundle package's markdown | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | `Ran 27 tests ... OK` | the residue gate's own unit tests, which pin B15 and B16 | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` |
  | `RESULT: PASS` (`pi-sweep`) | the Product-Identity sweep stage | `bash scripts/verify.sh --only pi-sweep` |
  | `NO_RUN_EXIT=0`; lib `3344 passed; 0 failed; 16 ignored` (46.3 s); full workspace ``FULL_EXIT=0` — 418 targets, **8,873 passed, 0 failed, 69 ignored**, zero `test result: FAILED` lines (cycle 4 recorded 8,872; the `+1` is exactly this cycle's one new gate test)` | the whole root workspace | `cargo test --locked --no-run -j 6`; `cargo test --locked --lib -j 6`; `cargo test --locked --no-fail-fast -j 6` |
  | root-workspace clippy **0 warnings** | the root workspace with tests | `cargo clippy --locked --tests -j 6` |
  | `closed=0 relabeled=0 rust_lines_changed=200 ratio=n/a builds_recorded=1 pcgen_live_files=18` | `docs/work-inventory.json` before vs after | `python3 scripts/cycle_scope_gate.py --receipt --since fe0a51417e1adde5bb002b3cab33d8e35707e1e1 --before /tmp/wi-before-AT-35-E6-003-RULED.json --after docs/work-inventory.json` |

  The lib count moved `3343 → 3344`: exactly this cycle's one new corpus-wide gate test, and
  nothing else in the lib moved.

- **Build scope verified:** **the whole root workspace**, at the final tree. `apps/` is absent
  from this cycle's diff (`git diff --name-only fe0a51417e..HEAD -- apps/` is empty), which is
  the condition `§6` step 3 states for leaving the desktop crate to the epic wrap-up.

  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → `3344 passed; 0 failed; 16 ignored` (46.3 s).
  - `cargo test --locked --no-fail-fast -j 6` → ``FULL_EXIT=0` — 418 targets, **8,873 passed, 0 failed, 69 ignored**, zero `test result: FAILED` lines (cycle 4 recorded 8,872; the `+1` is exactly this cycle's one new gate test)`.
  - `cargo clippy --locked --tests -j 6` → **0 warnings**, after one self-heal: the first run
    raised `collapsible_if` on the new gate test's nested `if let`; fixed in-cycle with a
    let-chain and the command now emits no `warning:` or `error:` line at all.

- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` would
  re-examine a byte-identical `data/`.

- **Oracle pin:** N/A. No figure in this receipt came from the pinned PCGen checkout;
  `scripts/pcgen-oracle-pin.env` is unchanged.

- **Status:** **partial.** The criterion's population is not zero at HEAD: 36 hits across 18
  files remain under `src/rules_core/`.

- **Notes:**

  The cycle's product is not the one hit. It is the join: a corpus record can now ask the
  converted package what it became, totally, for every Epic-2 kind. Three cycles had recorded
  that lookup as missing converter work; it was a missing index. The honest counterweight is in
  the refused row — the join does **not** unblock the 27 hits that own `EquipmentRecord` as a
  data type, and saying so is the point of stating `trait_and_pool_tokens=4` unchanged rather
  than folding this cycle's one clearance into it.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`.
  The remainder is 36 hits / 18 files, all under `src/rules_core/`, in two pieces:

  1. **The `renderer` group (5) is a CONVERTER cycle.** Three named populations, enumerated by
     key in `AT-35-E6-003-RULED_cycle2_prose_parity_census.json`: 1,351 keys whose converted
     rule carries extra `Desc` segments; 718 keys the converter renders and the live path does
     not; 374 the converted rule cannot render. The live swap in
     `class_feature_grant_consumer.rs` is one commit once that census reads `disagree=0`.
  2. **`lst_parser_types` (12) + `ingest_record_tokens` (6) + `ir_converter` (4) +
     `source_content_payload` (5) = 27 hits are ONE piece of work**, and it is the largest single
     item left in Epic 6: `sheet_rule_convert` must emit an equipment / equipment-modifier /
     spell rule shape the live side owns, and `equipment_effects`, `encumbrance`,
     `damage_total`, `equipment_resolver`, `spell_resolver` and `corpus_loader` must read that
     instead of `EquipmentRecord.tokens` / `.bonus_chains`. The converted package already holds
     6,223 equipment and 1,532 equipment-modifier records, and the closure-row join built this
     cycle is how those 13 files will reach them. `trait_and_pool_tokens` (4) rides on the same
     change for pool members and race-trait bonus chains.
