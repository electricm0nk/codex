# Cycle AT-35-E6-003-RULED cycle 7 — Epic 6 PCGen exit / AT-35-E6-003-RULED

- **Commit SHA:** `f446ba681a` (the code, the two census/parity scripts and their JSON, the
  retro events), cycle start `287968b058`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `287968b058`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 6's closing figure — nothing drifted between the two cycles:
  ```
  live_files=17 live_hits=35 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/rules_core/skinwalker_change_shape.rs` — **−5 lines, +8 of comment.** The whole of the
    hit this cycle closes. The module obtained a record's automatic grants from the live
    accessor `RaceTraitRecord::automatic_trait_grants`, and then imported
    `pcgen_import::race_trait_tokens` for one call —
    `skinwalker_change_shape_kin(&grant)` — whose entire body is
    `grant.strip_prefix(SKINWALKER_CHANGE_SHAPE_POOL_PREFIX)`. The question it was asking
    (*which kin pool does this record own?*) is a rules question; only the prefix-stripping
    grammar was the converter's. The module no longer names `pcgen_import` at all.
  - `src/rules_core/race_resolver.rs` — **+22 lines of accessor + doc, +44 of test.** New
    `RaceTraitRecord::skinwalker_change_shape_kin() -> Option<String>`, declared beside
    `automatic_trait_grants`, the accessor the grants it derives from already came from. Same
    single-sourcing cycle 4 applied to the ARG picker's `exclusion_guard_flags`; the grammar
    stays on the converter side and is **KEPT for Starfinder** (`decisions.md` §11) — nothing
    was deleted, one call site moved.
  - `…/AT-35-E6-003-RULED_cycle7_runtime_import_census.py` / `.json` — **new.** Imports cycle
    6's census whole (which imports cycle 5's, … back to cycle 1's) and rewrites one group's
    reason with this cycle's measurements. It asserts its own total against the gate's
    (`gate_agreement=OK (34 == 34)`) so the two cannot disagree silently.
  - `…/AT-35-E6-003-RULED_cycle7_pool_guard_parity.py` / `.json` — **new.** The measurement
    that refutes cycle 6's named converter defect; see **Discoveries**.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from this cycle's own instruments, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one
    field, `derived_at`, restamped by this cycle's `completion_atlas.py --check`). Committed
    rather than filtered away, per the standing "clean tree = unfiltered `git status` empty"
    rule.

  **No `data/` file and no corpus record was changed** (`git status --porcelain data/` empty at
  the final tree), so `data/sheet_rules/` and `docs/work-inventory.json` are byte-identical to
  the cycle's start tree. **`apps/` was NOT touched**, so the desktop crate and the frontend run
  at the epic wrap-up (`§6` step 3), not here.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff 287968b058 -- src/ …/epic-6-pcgen-exit/ | grep -E '^\+'`),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing.

  The cumulative `${BASE_BRANCH}...HEAD` form (`BASE_BRANCH=fe5ae6cd4a`) over the epic's
  file-touch set returns only pre-existing `tests/sd27_*` / `tests/sd34_*` **filenames quoted
  inside earlier cycles' receipt prose** — the disposition every Epic 6 receipt has recorded. No
  identifier in shipping code carries a bundle tag.

- **Wired-integration audit result:** OK_NO_TOKENS, first run, no self-heal. This cycle's added
  Rust lines and both new Python instruments under
  `grep -nEi '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` return nothing.
  No `"Would …"` string, no inline mock, no fixture-only data path: the new accessor is a
  delegation whose result is pinned against the live `bestiary_5` corpus, and the parity
  instrument reads `data/sheet_rules/` and `data/corpus/` whole rather than a fixture.

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
  first met in cycle 4, not regressed here — this cycle wrote no `apps/` file). The criterion as
  a whole is **not** met: 34 hits across 16 files remain under `src/rules_core/`, and
  `render_pcgen_desc_with_values` is still called there.

- **Receipt rows (mechanical):**
  ```
  since=287968b058dd815daa4f85d7682d869d3b5ffd91 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=96 ratio=n/a builds_recorded=0 pcgen_live_files=16
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.
  `builds_recorded=0` is a counter fed by `verify.sh`'s own build stages; this cycle ran the
  cargo suites directly (`§6` step 3's explicit command list) and only `--only pi-sweep` through
  `verify.sh`, which records no build. The build evidence is the `FULL_EXIT=0` line below, not
  this counter.

- **PCGen residue:** `live_files=16 live_hits=34 baseline_files=260 baseline_hits=12736
  verdict=PASS` — down from cycle 6's `17 / 35` on both axes, and **the instrument was not
  touched this cycle** (`git diff --name-only 287968b058..HEAD -- scripts/` is empty), so the
  `−1 / −1` is entirely code. Per-root:
  ```
  root src/rules_core        files=17 hits=35  ->  files=16 hits=34
  root src/saved_character   files=0  hits=0
  root src/campaign          files=0  hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop          files=0  hits=0   (unchanged — no apps/ file written)
  ```
  **One hit, one whole file.** Not gate-gaming: nothing renamed to duck a regex, no path
  exempted, no `use` collapsed, no rebaseline. The 27 gate unit tests that pin B14, B15 and B16
  are still green.

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added and no
  rendered value changed. The parity this swap turns on is **the accessor against the reading it
  replaces**, measured over the whole live Skinwalker population rather than a fixture: for every
  one of the 75 `bestiary_5` Skinwalker race-trait rows (`ls
  data/corpus/bestiary_5/race_trait/skinwalker/*.json | wc -l`),
  `RaceTraitRecord::skinwalker_change_shape_kin()` equals the previous in-module reading
  recomputed in-test, and the ten kin master rows it names are pinned by name
  (`Default`, `Werebat-Kin`, `Werebear-Kin`, `Wereboar-Kin`, `Werecrocodile-Kin`,
  `Wereraptor-Kin`, `Wererat-Kin`, `Wereshark-Kin`, `Weretiger-Kin`, `Werewolf-Kin`). The
  kin list was **written wrong first and corrected by the red test**, not copied from prose: the
  first run named a `Werewrasse-Kin` the corpus does not carry and failed naming
  `Wereraptor-Kin`.

- **Movement, four buckets:**
  - **closure:** **none in corpus units** (Epic 6 closes zero by design). On the B16 population:
    `src/rules_core` `35 → 34` hits and `17 → 16` files; by group, `trait_and_pool_tokens`
    `4 → 3`. `skinwalker_change_shape.rs` leaves the census entirely.
  - **relabel:** none. No hit moved between files or groups; the accessor was added to a file
    that already imported `race_trait_tokens` for six other readings, so no hit reappeared.
  - **reachability:** none — no rendered sheet line moved. The nine kins resolve the same
    option sets from the same records (`all_nine_kins_resolve_real_nonempty_grants` and its two
    siblings green, unchanged).
  - **instrument-correction:** **none in the gate.** The gate script, its baseline file and its
    patterns are untouched. One *census reason* was rewritten, and that rewrite rests on a
    measurement that moved two of this cycle's own probe figures — see **Discoveries** — but it
    moved no gate number and closes nothing.

- **Refused tokens:** **34 hits across 16 files, six groups**, the same six as cycles 4–6:
  ```
  renderer=5, lst_parser_types=12, ingest_record_tokens=5, trait_and_pool_tokens=3,
  ir_converter=4, source_content_payload=5
  ```
  `5+12+5+3+4+5 = 34`. Six groups, under `§8`'s limit of ten. Recorded as
  `deferral 1789273485421-at-35-e6-003-ruled-eec71c`; every line named with file, line and
  reason in `…_cycle7_runtime_import_census.json` and re-derivable by its script.

  **Why each group did not go.**

  - `trait_and_pool_tokens` (3) — the `skinwalker_change_shape` hit **went**. The
    `class_feature_pool_catalog` hit is refused **on a re-measured number** (see
    **Discoveries**), not on cycle 6's stated reason, which this cycle refutes. The two
    `race_resolver` hits are this grammar's live consumer of record — the file that hosts the
    six accessors every other live caller now asks instead — and ride on item 1's race-trait
    rule shape.
  - `renderer` (5) — refused by cycle 2's measurement, not by difficulty: the converted
    candidate exists, runs, and disagrees with the live path on 97,332 of 660,320 renderings
    across 2,443 record keys, every shape of it converter-side.
  - `lst_parser_types` (12) + `ingest_record_tokens` (5) + `ir_converter` (4) +
    `source_content_payload` (5) = **26 hits are one piece of work**, unchanged in kind by this
    cycle: those callers do not want a *fact about* a record, they **own** `EquipmentRecord` /
    `LstSpellRecord` / `SourceContentPayload` as their own data types across 13 files and read
    the PCGen `BONUS:` chains and `KEY:VAL` tokens on them directly. That clears when
    `sheet_rule_convert` emits an equipment / equipment-modifier / spell rule shape
    `equipment_effects` can read, not when a lookup exists.

- **Discoveries:** one, emitted as a `correction` retro event.

  `1789270208595-at-35-e6-003-ruled-7518b3` (`correction`) — **cycle 6's named converter defect
  does not exist; its probe was reading the wrong side of the edge.**

  Cycle 6 refused the `class_feature_pool_catalog` swap on `P1 agree=11549 disagree=6494 of
  18043` and named the mechanism *"exact, not vague"*: that PCGen's `ABILITY:` token maps to
  `MapsTo::Applies` in `src/pcgen_import/sheet_rule/table.rs`, **i.e. to a prerequisite**, so a
  record granting two abilities automatically converts to `grants: null, target: None,
  value: "Text"` — byte-identical, on the converted side, to a prose-only record.

  **The mapping table's own `ABILITY` row says the opposite**, verbatim:
  *"`ABILITY:<cat>|<nature>|<target>…` on a holder record H is a **GRANT edge**: the rule
  `<target>` (joined by (cat, key)) gets `granted_by += Grant{by: Rule(H), when: …}`"*, with
  `applies_shape: Grant{…}`. And the converter implements exactly that:
  `sheet_rule/convert.rs:1266` pushes every resolved target to `out.grants_out`, which
  `sheet_rule/mod.rs:691-732` folds onto **the target rule's** `granted_by` — a different record
  id, in a different file. Cycle 6's probe read only `rules_for_closure_row(this record)`, so it
  was structurally incapable of seeing the converted form of the token it named. `MapsTo` is the
  row's *coarse disposition*, not the arm that implements it; reading a defect out of it is the
  error.

  **Checked on cycle 6's own worked example.**
  `data/sheet_rules/core_rulebook/feat/improved_unarmed_strike.json` carries
  `{"by": {"Rule": "occult_adventures:class_feature:elemental_ascetic_elemental_flurry"}, "when":
  "Always"}` in its `granted_by`, one of 27 there. The edge converted. It points the other way.

  `BONUS:` and `DEFINE:` were unread for the same reason at a different address: they land as
  contributor rows in `data/sheet_rules/_vars/<var>.json`, which cite the contributing
  `rule_id` and the declaring rule in `declared_by`. Of the 3,954 P1 "disagreements" that
  survived the grant fix, 2,304 were `BONUS` records and 1,650 `DEFINE`.

  **The corrected probe, and what it changes.** Three fixes — a reverse-grant index over every
  rule's `granted_by`, a var-contributor index over `_vars/`, and a `Holds` traversal that
  reaches the id (the serialized shape is `{"Holds": {"what": {"Rule": id}, "count": n}}`; a
  traversal looking for `{"Holds": {"Rule": …}}` finds none of them):

  | guard | population | cycle 6 disagree | cycle 7 disagree | verdict |
  |---|---|---|---|---|
  | `has_no_engine_effect_token` | 18,043 | 6,494 | **1,870** | REFUSED |
  | `is_archetype_locked` | 18,043 | 919 | **864** | REFUSED |
  | `carries_more_than_one_desc_segment` | 18,043 | 89 | 89 | REFUSED |

  **The swap stays refused, on the number — but on a different number, and the converter is no
  longer under the accusation.** Two things are now known that were not:

  1. `P2`'s disagreements **changed direction**. Every one of the 864 is now the *converted*
     side refusing a row the *ingest* guard admits, and the mechanism is a blind spot in the
     live guard, not in the converter: `is_archetype_locked` reads `token_values(data,
     "PREABILITY")` only, and this corpus also writes `CATEGORY=Archetype` **inside a `PREMULT`
     wrapper** — e.g. `acg_abilities_class.lst:2392`'s `PREMULT:1,[PREABILITY:1,
     CATEGORY=Archetype,Alchemist Archetype ~ Inspired Chemist],[…]`, which the ingest guard
     does not see and the converted `applies` resolves correctly to
     `Holds{what: Rule(advanced_class_guide:class_feature:arcanist_archetype_blade_adept),
     count: 1}` on the sibling case at `:2392`. **Whether that blind spot reaches a served pool
     member is not measured here and is not claimed.** It is named so the next cycle measures it
     rather than rediscovering it.
  2. `P1`'s residual 1,870 is now a bounded, nameable population rather than a third of the
     corpus.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=16 live_hits=34 baseline_files=260 baseline_hits=12736 verdict=PASS` | every source file (`.rs .ts .tsx .js .jsx .mjs .cjs`) under the five live roots, comment lines excluded (B14) and `#[cfg(test)]` regions excluded (B15) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `root src/rules_core files=16 hits=34`; `root apps/desktop files=0 hits=0` | the same, restricted to that root | `python3 scripts/pcgen_residue_gate.py --check` |
  | `pcgen_import_hits=34 files=16`; `apps_desktop_hits=0 evidence_sentence_met=YES`; the six group sizes; `gate_agreement=OK (34 == 34)` | the shipping lines under the five live roots naming `pcgen_import` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle7_runtime_import_census.py` |
  | the gate script and its baseline are absent from this cycle's diff | `scripts/` | `git diff --name-only 287968b058dd815daa4f85d7682d869d3b5ffd91..HEAD -- scripts/` |
  | `corpus_class_feature_records=18043 joined=18043 unjoined=31`; `P1 agree=16173 disagree=1870`, `P2 agree=17179 disagree=864`, `P3 agree=17954 disagree=89`; `swap_verdict=REFUSED` | every `data/corpus/*/class_feature/**/*.json` record carrying `raw_tokens` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle7_pool_guard_parity.py` |
  | the `ABILITY` row specifies a **grant edge**, not a prerequisite | the operator-ruled token-mapping input (`decisions.md` §15) | `python3 -c "import json; d=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json')); print([r['rule'] for r in d['rows'] if r['token_type']=='ABILITY'][0][:120])"` |
  | the converter implements it as a grant edge | the converter's `ABILITY` arm | `sed -n '1266,1295p' src/pcgen_import/sheet_rule/convert.rs` |
  | the grant edge is present in the shipped package on cycle 6's own worked example | one converted rule | `python3 -c "import json; print([g for r in json.load(open('data/sheet_rules/core_rulebook/feat/improved_unarmed_strike.json')) for g in r['granted_by'] if 'elemental_ascetic_elemental_flurry' in json.dumps(g)])"` |
  | the accessor equals the reading it replaces on every live Skinwalker row, and the ten kin master rows by name | every `bestiary_5` Skinwalker race-trait record | `cargo test --locked --lib -j 6 rules_core::race_resolver::tests::skinwalker_change_shape_kin_names_the_nine_kin_master_rows` |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` (114.9 s) | the whole converted package | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `0` files under `data/sheet_rules/` carrying ingest syntax | the whole converted package | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | `missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | the completion atlas | `python3 scripts/completion_atlas.py --check` |
  | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS` | token coverage | `python3 scripts/token_coverage.py --check` |
  | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | shape/engine boundary | `python3 scripts/shape_engine_boundary.py --check` |
  | `population=0 kinds=0 citation_failures=0` | missing engine tables | `python3 scripts/missing_engine_tables.py --check` |
  | `files_checked=136 violations=0` | the bundle package's markdown | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | `Ran 27 tests … OK` | the residue gate's own unit tests, which pin B14, B15 and B16 | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` |
  | `RESULT: PASS` (`pi-sweep`) | the Product-Identity sweep stage | `RETRO_ACTOR=AT-35-E6-003-RULED bash scripts/verify.sh --only pi-sweep` |
  | `NO_RUN_EXIT=0`; lib `3346 passed; 0 failed; 16 ignored` (42.4 s); full workspace `FULL_EXIT=0` — 418 targets, **8,875 passed, 0 failed, 69 ignored**, zero `test result: FAILED` lines | the whole root workspace | `cargo test --locked --no-run -j 6`; `cargo test --locked --lib -j 6`; `cargo test --locked --no-fail-fast -j 6` |
  | root-workspace clippy **0 warnings** | the root workspace with tests | `cargo clippy --locked --tests -j 6` |
  | `closed=0 relabeled=0 rust_lines_changed=96 ratio=n/a builds_recorded=0 pcgen_live_files=16` | `docs/work-inventory.json` before vs after | `python3 scripts/cycle_scope_gate.py --receipt --since 287968b058dd815daa4f85d7682d869d3b5ffd91 --before /tmp/wi-before-AT-35-E6-003-RULED.json --after docs/work-inventory.json` |

  The lib count moved `3345 → 3346` and the workspace total `8,874 → 8,875`: exactly this
  cycle's one new test, and nothing else moved.

- **Build scope verified:** **the whole root workspace**, at the final tree. `apps/` is absent
  from this cycle's diff (`git diff --name-only 287968b058..HEAD -- apps/` is empty), which is
  the condition `§6` step 3 states for leaving the desktop crate and the frontend to the epic
  wrap-up.

  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → `3346 passed; 0 failed; 16 ignored` (42.4 s).
  - `cargo test --locked --no-fail-fast -j 6` → `FULL_EXIT=0` — **418 targets, 8,875 passed,
    0 failed, 69 ignored**, zero `test result: FAILED` lines.
  - `cargo clippy --locked --tests -j 6` → **0 warnings**, first run, no self-heal.

- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` would
  re-examine a byte-identical `data/`.

- **Oracle pin:** N/A. No figure in this receipt came from the pinned PCGen checkout;
  `scripts/pcgen-oracle-pin.env` is unchanged.

- **Status:** **partial.** The criterion's population is not zero at HEAD: 34 hits across 16
  files remain under `src/rules_core/`.

- **Notes:**

  **One judgment call: the cycle refused a two-line trim that would have read as progress.**
  Two of the five `source_content_payload` hits — `equipment_resolver.rs:19` and
  `spell_resolver.rs:16` — import `crate::pcgen_import::source_content_payload::
  SourceContentPayload`, and `crate::rules_core::source_content` **re-exports that exact type**
  (`source_content.rs:50`). Rewriting those two `use` lines to the live re-export would have
  lowered the gate by 2 and changed not one byte of the shipping binary's dependency graph.
  That is the trim cycle 3 refused for `derived_evaluator_fixture_check.rs` and cycle 6 refused
  for `PU_RESOLVABLE_DESCRIPTIONS`' renderer; it is refused here for the same reason. The
  payload type itself genuinely cannot move — its variants hold borrowed B-family parser entry
  types, which is why `source_content.rs` re-exports rather than declares it — so the honest
  disposition is that all five stay, in item 1.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`.
  The remainder is 34 hits / 16 files, all under `src/rules_core/`, in **three** pieces:

  1. **`lst_parser_types` (12) + `ingest_record_tokens` (5) + `ir_converter` (4) +
     `source_content_payload` (5) = 26 hits are ONE piece of work**, and the largest item left in
     Epic 6: `sheet_rule_convert` must emit an equipment / equipment-modifier / spell rule shape
     the live side owns, and `equipment_effects`, `encumbrance`, `damage_total`,
     `equipment_resolver`, `spell_resolver` and `corpus_loader` must read that instead of
     `EquipmentRecord.tokens` / `.bonus_chains`. The converted package already holds 6,223
     equipment and 1,532 equipment-modifier records, and cycle 5's closure-row join is how those
     13 files reach them. **This is where the next cycle should go.**
  2. **The `renderer` group (5) is a CONVERTER cycle.** Three named populations, enumerated by
     key in `AT-35-E6-003-RULED_cycle2_prose_parity_census.json`: 1,351 keys whose converted rule
     carries extra `Desc` segments; 718 keys the converter renders and the live path does not;
     374 the converted rule cannot render. The live swap in `class_feature_grant_consumer.rs` is
     one commit once that census reads `disagree=0`.
  3. **`trait_and_pool_tokens` (3) is a MEASURED refusal with a corrected diagnosis, not a named
     converter defect.** Cycle 6's `ABILITY`-maps-to-a-prerequisite claim is refuted above; do
     not re-derive it. The live remainder is `P1` 1,870 / 18,043, and the one new lead is that
     `is_archetype_locked` misses `CATEGORY=Archetype` nested inside a `PREMULT` — a **live-guard**
     blind spot whose reach into served pool members is unmeasured and unclaimed.
