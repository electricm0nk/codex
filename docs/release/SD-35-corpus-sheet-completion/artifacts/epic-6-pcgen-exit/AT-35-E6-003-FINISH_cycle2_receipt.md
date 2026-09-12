# Cycle AT-35-E6-003-FINISH cycle 2 — Epic 6 PCGen exit / AT-35-E6-003-FINISH

- **Commit SHA:** `COMMIT_SHA_PLACEHOLDER`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `dc960b6548`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree:
  ```
  live_files=45 live_hits=304 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/rules_core/pilot_compute/resolved_prose.rs` — **new, 313 lines.** A converted rule's
    own words rendered with this character's numbers, out of `data/sheet_rules/`. Reads our own
    `ProseSegment` / `ProsePiece::Slot` / `Applies` schema and nothing else.
  - `src/rules_core/pilot_compute/mod.rs` — deleted `PuResolvableDescription`,
    `PU_RESOLVABLE_DESCRIPTIONS` and the four `PU_*_DESC_TOKEN` constants (109 lines of verbatim
    PCGen `DESC:` token text), deleted the
    `use crate::pcgen_import::pcgen_desc::{leaked_pcgen_syntax, render_pcgen_desc_tokens,
    PcgenDisplayValues}` import, and rewrote `pu_display_values` / `pu_resolved_description` over
    the converted package.
  - `src/pcgen_import/pcgen_desc.rs` — `desc_token_arguments` widened `pub(crate)` → `pub` so the
    parity **oracle** in `tests/` can seed both environments identically. Tool side only.
  - `tests/sd27_pu_class_feature_descriptions_carry_the_characters_numbers.rs` — the two
    transcription-pin tests replaced by the converter-parity gate and its re-derived denominator.
  - `tests/sd35_rendered_prose_carries_no_ingest_vocabulary.rs` — the PU transcription **exemption
    and its skip machinery removed**, because there is no longer anything to exempt.
  - `…/AT-35-E6-003-FINISH_cycle2_residue_census.json`, this receipt, `progress.md`,
    `kanban.md`, `docs/retro/events/at-35-e6-003-finish.jsonl`.

  No `data/` file changed and no corpus record was touched.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. `git diff --unified=0 -- src tests | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns only the two **test filenames**
  quoted inside doc-comment prose (`sd27_pu_class_feature_descriptions_…`,
  `sd35_rendered_prose_…`) and the diff's own `+++ b/tests/…` headers. No identifier in shipping
  code carries a bundle tag — the same disposition cycle 1 recorded for the same two filenames.

- **Wired-integration audit result:** OK_NO_TOKENS. The same diff against
  `\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b` returns nothing, and the new
  file returns nothing on either pattern.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Met for the `src/rules_core/pilot_compute` half, not for the whole criterion.** Two of the
  six live `render_pcgen_desc*` call sites are gone and the last PCGen token text in executable
  code with them; four call sites remain, in two files, and are this cycle's named remainder.

- **Receipt rows (mechanical):**
  ```
  since=dc960b6548e7b016b873d04b2e1f30ad3e5d2a18 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=411 ratio=n/a builds_recorded=0 pcgen_live_files=45
  ```

- **PCGen residue:** `live_files=45 live_hits=300 baseline_files=260 baseline_hits=12736
  verdict=PASS` — **down 4 from cycle 1's 304**, and not raised on either axis. `live_files`
  does not move because all 45 files still carry class-A `#[cfg(test)]` hits.

- **Oracle parity:** `compared=56 agree=56 disagree=0`. Epic 6 touched a live path, so the
  parity run is required. The oracle is the PCGen renderer itself, kept exactly where
  `decisions.md` §11 says it belongs — in `tests/`: for each of the 7 Pathfinder Unchained
  `class_feature` records carrying a `%N`, read off `data/corpus/` at run time, over 8 value
  tables each, `render_pcgen_desc_tokens(corpus DESC tokens, values)` and
  `resolved_prose::resolved_description(converted rule, values)` must render byte-identical text
  — including the two mutually exclusive `PREVAR` branches of `Unchained Rogue ~ Rogues Edge` and
  the undecidable `PREABILITY` gate of `Unchained Rage`. Not one disagreement.

- **Movement, four buckets:**
  - **closure:** the PCGen **token text in executable live code**, `4 → 0`. Refused-type counts
    `PRE[A-Z]+:` 51 → 47; `TYPE=`, `BONUS:`, `DESC:`, `%CHOICE` unchanged (all class A). Two of
    the six live `render_pcgen_desc*` call sites closed, and the `use
    crate::pcgen_import::pcgen_desc::{…}` import of `src/rules_core/pilot_compute/mod.rs` with
    them. Class C: 100 lines / 28 files → 93 lines / 27 files.
  - **relabel:** none. No corpus unit moved bucket; `closed=0` is correct and expected.
  - **reachability:** none — the rendered text is byte-identical, proved above, so nothing the
    player reads moved.
  - **instrument-correction:** none applied. Cycle 1's finding that the gate's
    `\brender_pcgen_desc\b` pattern cannot see `render_pcgen_desc_tokens` /
    `render_pcgen_desc_with_values` still stands and is **still not fixed**, for the reason given
    under **Notes**.

- **Refused tokens** — four types, summing to **300**, the gate's own `live_hits`:
  ```
  TYPE==100, BONUS:=91, DESC:=59, PRE[A-Z]+:=47
  ```
  plus `%CHOICE=3`, which is inside that 300 (five types listed, four above the type named in the
  closure row). All 300 are **class A** — inside a `#[cfg(test)]` module of a live file,
  compiled out of the shipping library, blocked on operator ruling **B15**, asked for the
  eleventh time. Class B — executable product code — is **0**. Recorded as a `deferral` event.

  | class | cycle 1 | now | what it needs |
  |---|---|---|---|
  | **A** — `#[cfg(test)]` fixtures carrying verbatim corpus tokens | 300 | **300** | operator ruling B15 |
  | **B** — executable product code | 4 | **0** | *closed by this cycle* |
  | **C** — run-time reads of `src/pcgen_import` no gate pattern matches | 100 lines / 28 files | **93 lines / 27 files** | the ruling on whether the gate counts them, then the work |

- **Discoveries:** one, and it is the reason the cycle had work to do at all. Emitted as two
  `correction` retro events (`1789222614837-at-35-e6-003-finish-286d42`,
  `1789222622846-at-35-e6-003-finish-dcf34c`).

  **The conversion had already happened; only the reader had not moved.**
  `data/sheet_rules/pathfinder_unchained/class_feature/unchained_barbarian_rage.json` and its six
  siblings have carried these records' prose as plain-English `ProsePiece::Text` with typed
  `ProsePiece::Slot(Expr::Var(..))` holes, and each segment's `PREVAR*` gate as a converted
  `Applies::Compare`, for the whole of this epic. Cycle 1 refused class B on the premise that the
  four transcriptions "leave together with the renderer or not at all", and that the renderer
  could not leave because two of its call sites are in `src/rules_core/race_resolver.rs`, "outside
  this epic's file-touch set". **Both halves were wrong.** `race_resolver.rs` *is* in the set —
  `workflow-instruction.md`'s Epic 6 row names *"every live `PcgenFormulaEvaluator` /
  `render_pcgen_desc` / `raw_tokens` call site (78 files by **coarse grep**)"*, and a coarse grep
  for `render_pcgen_desc` matches `render_pcgen_desc_tokens`. And the transcriptions did not need
  the renderer to leave at all: they needed the converted package to be **read**.

  The standing lesson this is an instance of is `shipped-prose-is-not-a-source-of-truth`: a
  blocker recorded in a receipt was carried forward as settled for one cycle without being
  re-derived against the repo.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=45 live_hits=300 … verdict=PASS` | every source file under the five live roots, code lines only (ruling B14) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `class_B_executable_hits=0 files=0` (was 4 / 1) | the gate's own hits, partitioned by brace-matched `#[cfg(test)]` item range | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-FINISH_cycle1_residue_census.py` |
  | `class_A=300`, `class_C=93 lines / 27 files` | same | same command |
  | `compared=56 agree=56 disagree=0` | the 7 PU `class_feature` records carrying a `%N`, re-derived off `data/corpus/`, × 8 value tables | `cargo test --locked --test sd27_pu_class_feature_descriptions_carry_the_characters_numbers` (`the_converted_prose_renders_the_same_words_the_corpus_desc_tokens_do`) |
  | `7 of 64` PU records carry a `%N` | every ingested PU `class_feature` record on disk | same test (`the_rendered_set_is_exactly_the_pu_records_carrying_a_percent_n`) |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` | the whole converted package | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `0` files under `data/sheet_rules/` carrying ingest syntax | the whole converted package | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | `closed=0 … rust_lines_changed=411 pcgen_live_files=45` | `docs/work-inventory.json` before vs after | `python3 scripts/cycle_scope_gate.py --receipt --since dc960b6548e7b016b873d04b2e1f30ad3e5d2a18 --before /tmp/wi-before-AT-35-E6-003-FINISH.json --after docs/work-inventory.json` |
  | `citation_failures=0 stale_derived_at=False` | the completion atlas | `python3 scripts/completion_atlas.py --check` |
  | `non_done=0 … verdict=PASS` | token coverage | `python3 scripts/token_coverage.py --check` |
  | `magnitude_bearing=26396 not_held_by_engine=0` | shape/engine boundary | `python3 scripts/shape_engine_boundary.py --check` |
  | `files_checked=127 violations=0` | the bundle package's markdown | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |

- **Build scope verified:** run once, after the last figure-moving edit, at the tree this receipt
  commits. `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`. `cargo test --locked --lib -j 6`
  → `3341 passed; 0 failed; 15 ignored`. `cargo test --locked --no-fail-fast -j 6` →
  `FULL_SUITE_RESULT_PLACEHOLDER`. `cargo clippy --locked --tests -j 6` →
  `CLIPPY_RESULT_PLACEHOLDER`. `corpus_literal_sweep` is **skipped and named**: no corpus record
  changed (`git status` lists no `data/` path), so it would re-examine the identical tree.
  `v06_work_inventory` is **skipped and named**: the cycle changed no corpus record, no
  classifier and no `data/` file, so no unit can have moved — `closed=0 relabeled=0` above is
  read from the unchanged inventory, not asserted. The desktop crate and frontend run at the
  epic wrap-up: this cycle touched no `apps/` file.

- **Sweep population:** N/A — no corpus record changed.

- **Oracle pin:** N/A for the pinned PCGen checkout. The parity oracle in this cycle is the
  in-repo tool-side renderer `src/pcgen_import/pcgen_desc.rs` run against `data/corpus/`, not
  `$PCGEN_CORPUS_ROOT`, so `scripts/pcgen-oracle-pin.env` is not a denominator for any figure
  here and is unchanged.

- **Status:** **partial** — the criterion's population is not zero at HEAD. It moved, and the
  remainder is named and sums.

- **Notes:**

  **Why the gate's blind-spot pattern is still not widened.** Cycle 1's escalation stands: the
  `render_pcgen_desc` pattern cannot see the two names the code actually uses. This cycle closed
  two of the six call sites it hides; four remain
  (`src/rules_core/race_resolver.rs:90,303`,
  `src/rules_core/pilot_compute/class_feature_grant_consumer.rs:968,1081`). Widening the pattern
  now would **raise** `live_hits`, which `workflow-instruction.md` §8 names non-self-healable and
  which the ratchet refuses by design. Widening it *after* those four are gone costs nothing and
  the gate becomes honest in the same motion. That is the order, and it is why the next cycle's
  scope is those four rather than the instrument.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`.
  The four remaining live `render_pcgen_desc*` call sites, in two files, by the same mechanism
  this cycle proved: read the converted `prose` instead of the record's `DESC:` tokens, with a
  parity gate against the tool-side renderer as oracle.
  1. `src/rules_core/race_resolver.rs` — `RaceTraitRecord::render_description`, and with it
     `same_row_display_values` / `display_values_with`, which read `bonus_chain_reader` at run
     time and must read the converted `_vars` contributions instead. Its second consumer is
     `apps/desktop/src-tauri/src/race_trait_picker.rs:646,650`, so that cycle pays the desktop
     crate and frontend suites.
  2. `src/rules_core/pilot_compute/class_feature_grant_consumer.rs:968,1081` — the two
     `resolved_description_for*` functions, which render `record.raw_description` after
     resolving its arguments through `record_vars`.

  Then, and only then, widen `pcgen_residue_gate.py`'s `render_pcgen_desc` pattern to a prefix
  match and re-run `--check`; it must stay at the same `live_hits`.

  **Class A (300 hits, 45 files) remains blocked on operator ruling B15** and no
  `AT-35-E6-003`/`AT-35-E6-004` cycle can take `live_hits` below 300 without it.
