# Cycle AT-35-E6-003-RULED cycle 2 — Epic 6 PCGen exit / AT-35-E6-003-RULED

- **Commit SHA:** `PENDING_SHA`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `a3b23bbc99`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed:
  ```
  live_files=25 live_hits=58 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `apps/desktop/src-tauri/src/reach_gate.rs` — the whole `provenance_prose` group, cleared. The
    five `src/pcgen_import/sheet_rule/` module paths written **inside string literals** (the
    remedy sentence an `UNREACHED_RECORD_FINDINGS` row prints) now name the runnable converter,
    `cargo run --bin sheet_rule_convert`. Nothing else in those rows changed: the same finding,
    the same counts, the same remedy, pointed at a command a reader can run instead of at a
    module path shipping code has no business naming. `AGENTS.md` rule 9 is better served by the
    invocation than by the path.
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — `+382` lines net. A `book`
    field on `ClassFeatureRecordTokens` (the `data/corpus/<book>/` directory the record was read
    from, first-book-wins exactly as each table's own duplicate-key policy already is), a
    `#[cfg(test)] impl` carrying `converted_rule_id`, and in the module's `#[cfg(test)]` region:
    the two **converted candidate** renderers and the corpus-wide parity census that measured
    them. `for_each_class_feature_record` hands its visitor the book dir — the one corpus walk
    both tables share, so they cannot disagree about it.
  - `src/rules_core/pilot_compute/resolved_prose.rs` — `+31` lines. `first_desc_slot_value`, the
    converted restatement of "`%1`'s own resolved value": the first slot in the rule's `Desc`
    prose, skipping gate-decided-FALSE segments exactly as `render_description` skips them, so
    the two can never disagree about which sentence the number came out of.
  - `…/AT-35-E6-003-RULED_cycle2_runtime_import_census.py` / `.json` — **new.** Imports cycle 1's
    census whole rather than copying it, and corrects the one reason cycle 1 got wrong.
  - `…/AT-35-E6-003-RULED_cycle2_prose_parity_census.json` — **new**, 309 KB: the whole parity
    measurement, its three disagreement shapes, the distinct-key count of each, and a spread
    sample.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not this cycle's work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one field,
    `derived_at`, restamped by this cycle's own `completion_atlas.py --check`) and
    `docs/retro/events/sd31-transcribe.jsonl` (one appended line from another session). Committed
    rather than filtered away, per the standing "clean tree = unfiltered `git status` empty" rule.

  **No `data/` file and no corpus record was changed**, so the converted package and the work
  inventory are byte-identical to the cycle's start tree.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff --unified=0 -- src/ apps/ | grep '^+'`),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing. The same grep over
  the cumulative `merge-base ... HEAD` diff of the scoped paths returns 102 matches and **every
  one is a `tests/sdNN_*.rs` integration-test FILENAME quoted in prose** — 16
  `sd35_rendered_prose_carries_no_ingest_vocabulary`, 14
  `sd27_pu_class_feature_descriptions_carry_the_characters_numbers`, 14
  `sd27_feat_prerequisite_enforcement`, and so on down a tail of real test files. That is the
  disposition every Epic 6 receipt has recorded.

- **Wired-integration audit result:** OK_NO_TOKENS. The same added-line set against
  `\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b` returns nothing. No `"Would
  …"` string, no inline mock, no fixture-only data path: the converted candidates are real
  implementations that run against the real `data/sheet_rules/` package, and the reason they are
  not live is a measured disagreement, not an unfinished body.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  Plus the `-RULED` dispatch's own bar, which is the two rulings applied **and the call sites the
  corrected gate now sees cleared**.

  **Not met. One of the six remaining groups is cleared and the largest one is now measured
  rather than assumed.** `apps/desktop/` is `files=3 hits=7`, not zero.

- **Receipt rows (mechanical):**
  ```
  since=a3b23bbc998ae5b0472bd3590dca011d4f5975bb residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=382 ratio=n/a builds_recorded=1 pcgen_live_files=24
  ```

- **PCGen residue:** `live_files=24 live_hits=53 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — down from cycle 1's `25 / 58` on both axes, and **the instrument was not touched this cycle**,
  so the `−5` is entirely code. Per-root, the figure the criterion's Evidence sentence asks for:
  ```
  root src/rules_core files=21 hits=46
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=4 hits=12   ->  files=3 hits=7
  ```
  `apps/desktop` is still **not** zero.

- **Oracle parity:** the class-feature description renderer, both ways, over the whole corpus.
  `records=16508 compared=660320 agree=562988 both_none=242160 disagree=97332`, and for the
  formula-only sibling `compared=660320 agree=648342 both_none=639148 disagree=11978`. No
  `PCGEN_ORACLE_SHA` figure: the oracle here is the live PCGen renderer over the corpus's own
  stored `DESC:` text, not the pinned checkout, which is unchanged. **This is the cycle's central
  result and it is a disagreement, not an agreement** — see Discoveries.

- **Movement, four buckets:**
  - **closure:** **none in corpus units** (Epic 6 closes zero by design). One refused token group
    closed: `provenance_prose`, `5 → 0`, `apps/desktop` `12 → 7` hits and `4 → 3` files.
  - **relabel:** none.
  - **reachability:** none — no rendered sheet line moved. The five edited strings are reach-gate
    *findings prose*, read by a developer, not by a player.
  - **instrument-correction:** **one, and it corrects a claim rather than a count.** Cycle 1's
    census recorded the `renderer` group as "The mechanism is proved". This cycle measured it and
    it is not; `AT-35-E6-003-RULED_cycle2_runtime_import_census.py` replaces that sentence with
    the measurement and the command that produced it. The hit counts did not move because of it.

- **Refused tokens:** **53 hits across 24 files, six groups**, down from seven:
  ```
  lst_parser_types=22, renderer=8, ingest_record_tokens=7, ir_converter=6,
  trait_and_pool_tokens=5, source_content_payload=5
  ```
  `22+8+7+6+5+5 = 53`. Six groups, under `§8`'s limit of ten. Recorded as
  `deferral 1789246450628-at-35-e6-003-ruled-e44e84`; every line named with file, line and reason
  in `…_cycle2_runtime_import_census.json` and re-derivable by its script.

  **Why `renderer` (8) did not go, stated as a number rather than a difficulty.** The converted
  replacement is written, compiles, and runs against the real package
  ([`tests::converted_resolved_description_for`] and its formula-only sibling). Rendering every
  `class_feature` record both ways at every level `1..=20` under two ability probes, the live path
  and the converted one **disagree on 97,332 of 660,320 comparisons — 14.7% — across 2,443
  distinct record keys**. Three shapes, none of them a tail:

  | shape | comparisons | distinct keys | what it is |
  |---|---|---|---|
  | `both_some_differ` | 53,698 | 1,351 | the converted rule carries `Desc` segments the stored description never had — a `.MOD` row merged from a second book appends a second sentence (`Apocalypse Mystery ~ Near Death`), or a bloodline's variant roster is appended to its own paragraph (`Bloodrager Bloodline ~ Draconic`) |
  | `old_none_new_some` | 28,720 | 718 | the converted rule renders where the live path refused — real text a player is not being shown today (`Cruelty ~ Stunned`, `Hangman ~ Bound to Truth`) |
  | `old_some_new_none` | 14,914 | 374 | the live path renders where the converted rule has no `Desc` prose, or a hole the engine cannot settle (`Aberrant Bloodline ~ Acidic Ray`, whose live rendering is itself truncated mid-sentence at `"deals 1d6"`) |

  **Every one of those three is on the CONVERTER side of `decisions.md` §11's line.** None is
  fixed by an edit in `class_feature_grant_consumer.rs`; each needs the converter to settle what
  a record's `Desc` prose is. Shipping the swap anyway would move 97,332 rendered sheet lines
  with nothing agreeing — the "a proof that passes and is too narrow ends scrutiny" failure
  `AGENTS.md` rule 7 names, in its worst form: no proof at all, presented as a completed exit.

  **Why the other five did not go.** Unchanged from cycle 1 and re-derived here: the live side
  still **owns** the ingest record structs (`EquipmentRecord`, `LstSpellRecord`) as its own data
  type across 13 files; `corpus_loader.rs` still **runs** `ir_converter::convert_equipment_record`
  at run time rather than reading its output; `SourceContentPayload`'s own module doc states why
  it cannot simply move (its variants hold borrowed references to the parser entry types, so
  relocating the enum alone forms an import cycle). Each is a type-ownership rewrite with a
  corpus-wide parity gate, not a line edit. `race_trait_picker.rs`'s three
  `race_trait_tokens::` calls read `PRE`-gate facts off the raw record and need the converted
  rule's `applies` gates instead — the same shape, one size down.

- **Discoveries:** one, emitted as a `correction` retro event.
  - `1789246442186-at-35-e6-003-ruled-c50bff` — **cycle 1's own next-cycle scope said the
    renderer group's mechanism was "proved twice over" and named a single undone piece (the
    formula-only resolver seeding from `%N` argument text). Measured, it is a disagreement of
    97,332 of 660,320 renderings whose cause is not that piece at all.** The seeding was never the
    problem; the converted prose and the stored prose are **different text** for 2,443 records.
    The claim was inherited from `AT-35-E6-003-FINISH` cycle 3, which proved the mechanism for
    **racial traits** — a population where `tests/sd35_race_trait_prose_comes_from_the_converted_
    package.rs` does assert byte-identical text. Carrying "proved" across from one record kind to
    another is the `validate-proxies-against-known-truth` shape: a proof cited where it was never
    run. The census is now the standing answer, in the code, at the call site.
  - A second finding, recorded here rather than as its own event because it is a consequence of
    the first: `resolved_prose::resolved_description` is **not** a general renderer and must not
    be used as one. It deliberately returns `None` for any rule whose `Desc` prose carries no
    `Slot`/`Dice` hole — a population predicate its own caller needs — and reading it as "the
    converted description" makes 9,195 record keys silently vanish. The sibling
    `render_description` is the general one. The first census run made exactly that mistake and
    reported `disagree=382300`; the number was right for the function called and meaningless for
    the question asked.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=24 live_hits=53 baseline_files=260 baseline_hits=12736 verdict=PASS` | every source file (`.rs .ts .tsx .js .jsx .mjs .cjs`) under the five live roots, comment lines excluded (B14) and `#[cfg(test)]` regions excluded (B15) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `root apps/desktop files=3 hits=7` | the same, restricted to `apps/desktop/**` | `python3 scripts/pcgen_residue_gate.py --check` |
  | `pcgen_import_hits=53 files=24`; `by_root=apps/desktop=7, src/rules_core=46`; the six group sizes; `provenance_prose=0 CLEARED` | the shipping lines under the five live roots naming `pcgen_import` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle2_runtime_import_census.py` |
  | `records=16508 compared=660320 agree=562988 both_none=242160 disagree=97332`; `formula_only compared=660320 agree=648342 both_none=639148 disagree=11978`; the three shapes and their distinct-key counts | every `class_feature` record `class_feature_record_tokens()` carries, × levels 1..=20 × 2 ability probes | `AT35_E6_PROSE_PARITY=docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle2_prose_parity_census.json cargo test --locked --lib -j 6 -- --ignored --nocapture class_feature_grant_consumer::tests::class_feature_prose_parity_census` |
  | `0` files under `data/sheet_rules/` carrying ingest syntax | the whole converted package | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | `citation_failures=0 stale_derived_at=False missing_clearing_mechanisms=0` | the completion atlas | `python3 scripts/completion_atlas.py --check` |
  | `non_done=0 refused=142 token_types=233 verdict=PASS` | token coverage | `python3 scripts/token_coverage.py --check` |
  | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | shape/engine boundary | `python3 scripts/shape_engine_boundary.py --check` |
  | `population=0 kinds=0 citation_failures=0` | missing engine tables | `python3 scripts/missing_engine_tables.py --check` |
  | `files_checked=131 violations=0` | the bundle package's markdown | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | `RESULT: PASS` (`pi-sweep`) | the Product-Identity sweep stage | `bash scripts/verify.sh --only pi-sweep` |
  | `since=a3b23bbc99… closed=0 relabeled=0 rust_lines_changed=382 ratio=n/a builds_recorded=1 pcgen_live_files=24` | `docs/work-inventory.json` before vs after | `python3 scripts/cycle_scope_gate.py --receipt --since a3b23bbc998ae5b0472bd3590dca011d4f5975bb --before /tmp/wi-before-AT-35-E6-003-RULED.json --after docs/work-inventory.json` |
  | `NO_RUN_EXIT=0`; lib `3341 passed; 0 failed; 16 ignored`; full workspace `FULL_EXIT=0` / **417 targets / 8,870 passed / 0 failed / 69 ignored / 0 `test result: FAILED`** | the whole root workspace | `cargo test --locked --no-run -j 6`; `cargo test --locked --lib -j 6`; `cargo test --locked --no-fail-fast -j 6` |
  | root-workspace clippy **0 warnings** | the root workspace with tests | `cargo clippy --locked --tests -j 6` |
  | desktop crate `569 passed; 0 failed; 0 ignored` (1509.4s); frontend `101/101 test files passed`; desktop clippy **1 warning, pre-existing and named below** | `apps/desktop/src-tauri` and `apps/desktop` | `cargo test --locked -j 6` and `cargo clippy --locked --tests -j 4` in `apps/desktop/src-tauri`; `npm test` in `apps/desktop` |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` (115.0s) | the whole converted package | `cargo run --locked --bin sheet_rule_convert -- --check` |

- **Build scope verified:** **the widest scope the repo has, plus the desktop crate and the
  frontend, because `apps/` was touched.** At the final tree:

  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → `3341 passed; 0 failed; 16 ignored`. `passed` is identical
    to `AT-35-E6-003-FINISH` cycle 3's; `ignored` is `15 → 16`, and the one is this cycle's own
    parity census, which is `#[ignore]`d by design.
  - `cargo test --locked --no-fail-fast -j 6` → `FULL_EXIT=0`, **417 targets, 8,870 passed, 0
    failed, 69 ignored, and zero `test result: FAILED` lines**. `8,870 passed` is identical to the
    last full run; `+1 ignored` is the same census.
  - `cargo clippy --locked --tests -j 6` → **0 warnings.** Two fired first, both in this cycle's
    own new census code (`manual implementation of .is_multiple_of()`, `or_insert_with` to
    construct a default) and both fixed in the same cycle, then re-run clean.
  - **Desktop crate** `cargo test --locked -j 6` → `569 passed; 0 failed; 0 ignored`
    (1509.4s), identical to `AT-35-E6-003-FINISH` cycle 3's, including
    `reach_gate::tests::unsurfaced_families_are_exactly_the_recorded_findings` and
    `…::unreached_records_are_exactly_the_recorded_findings` — the two that read the rows this
    cycle edited. **Frontend** `npm test` → `101/101 test files passed`.
  - **Desktop clippy: 1 warning, and it is NOT this cycle's.** `vec_init_then_push` at
    `apps/desktop/src-tauri/src/equipment_catalog.rs:889` — a ~100-line `pinned.push((book,
    count))` block in a file this cycle's diff does not touch at all (the only `apps/` change is
    five string literals in `reach_gate.rs`), so it is pre-existing by construction. It is left
    alone deliberately: rewriting 100 hand-transcribed book/count pairs into a `vec![]` is
    exactly the "unrelated cleanup" `AGENTS.md` rule 3 forbids, and a silent transcription slip
    there would move a pinned count. **Named here rather than swept**, for whichever cycle owns
    `equipment_catalog.rs` next.

  The `sheet_rule_convert -- --check` run above and `verify.sh --only pi-sweep` (`RESULT: PASS`)
  both ran in full at the same tree even though `data/` did not change, because the receipt
  schema names them.

- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` would re-examine
  a byte-identical `data/`.

- **Oracle pin:** N/A. No figure in this receipt came from the pinned PCGen checkout;
  `scripts/pcgen-oracle-pin.env` is unchanged. The parity census's oracle is the live renderer
  over the corpus's own stored text, in-repo.

- **Status:** **partial.** The criterion's population is not zero at HEAD. One of seven refused
  groups is closed; the largest remaining one is measured for the first time and the measurement
  refuses it.

- **Notes:**

  **What was tried and reverted, deliberately, and why that is the finding.** The converted swap
  was written into the live functions first — `resolved_description_for` and
  `resolved_description_for_formula_only_desc_argument` both reading the converted rule — and the
  census was written to prove it. The census refused it. Both live functions were then restored
  byte-for-byte and the converted implementations kept as `#[cfg(test)]` candidates beside the
  census that judges them, so the next cycle inherits working code and a standing measurement
  instead of a claim. `§8` names "RED→GREEN not preserved" non-self-healable; shipping a swap
  that moves 97,332 rendered lines with no oracle is that, wearing a green build.

  **The three `#[cfg(test)]` additions are not a way of hiding hits.** They add zero to the gate
  because ruling B15 excludes `#[cfg(test)]` regions, and that is the correct reading, not a
  convenient one: none of them is in the shipping binary. The live count fell for one reason
  only — five shipping string literals stopped naming a converter module path.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`.
  The remainder is now **converter-first**, which is a change of shape, not of size:

  1. **The `renderer` group is a CONVERTER cycle, not a live one.** Three named populations, in
     descending size: 1,351 keys whose converted rule carries extra `Desc` segments the stored
     description never had (decide whether a `.MOD`-merged sentence belongs in the record's own
     paragraph); 718 keys the converter renders and the live path does not (free reachability, if
     the text is right); 374 keys the converted rule cannot render (the converter dropped a hole
     the stored `DESC:` still states). Each is enumerated by key in
     `AT-35-E6-003-RULED_cycle2_prose_parity_census.json`. The live swap is one commit once the
     census reads `disagree=0`.
  2. **`lst_parser_types` (22) + `ir_converter` (6) + `ingest_record_tokens` (7) +
     `source_content_payload` (5) = 40 hits, one piece of work:** a converted equipment/spell
     record shape the live side owns, so `EquipmentRecord` and `LstSpellRecord` stop being live
     types and `corpus_loader.rs` reads a converted artefact instead of running the conversion.
     `source_content_payload.rs`'s own module doc states the cycle that blocks moving the enum
     alone; the shape has to exist first.
  3. **`trait_and_pool_tokens` (5)** — `race_trait_picker.rs`'s three `PRE`-gate reads want the
     converted rule's `applies`; `class_feature_pool_catalog.rs` wants a converted pool-member
     table.

  `AT-35-E6-004`'s `--check --closure` bar is unchanged.
