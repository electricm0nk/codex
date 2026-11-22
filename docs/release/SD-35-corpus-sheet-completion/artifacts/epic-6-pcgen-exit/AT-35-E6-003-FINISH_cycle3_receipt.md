# Cycle AT-35-E6-003-FINISH cycle 3 — Epic 6 PCGen exit / AT-35-E6-003-FINISH

- **Commit SHA:** `8fc2c53ca8`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `948a1d8e21`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree:
  ```
  live_files=45 live_hits=300 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Dispatched as "cycle 2", renumbered to 3.** The dispatch prompt named cycle 2 and described
  cycle 1's remainder. Cycle 2 had already landed at `e570ba506c` two and a half hours earlier
  and closed class B. Re-deriving the state at HEAD rather than working from the prompt's figures
  is what found it; this cycle took cycle 2's own named next-cycle scope instead.
  `correction 1789235054698-at-35-e6-003-finish-e88d35` also records what cycle 2 left undone.

- **Files touched:**
  - `src/rules_core/race_resolver.rs` — the `use crate::pcgen_import::pcgen_desc::{…}` import
    **deleted**. `same_row_display_values` reads the converted `_vars` tables instead of the
    row's `DEFINE` bases and `BONUS:VAR` amounts; `render_description` renders the converted
    rule's own prose instead of handing the record's `DESC:` tokens to the PCGen renderer;
    `display_values_with` and the three return types follow. New private `converted_rule_id`.
  - `src/rules_core/pilot_compute/resolved_prose.rs` — `+228 −1`. New `RenderedProse`,
    `render_description` (the drop-and-report sibling of `resolved_description`),
    `same_row_values`, `DisplayValues::set_id` / `len`, and one gate correction:
    `Applies::Holds { what: Holdable::MissingRule { .. } }` is **decided false**, not undecided.
  - `tests/sd35_race_trait_prose_comes_from_the_converted_package.rs` — **new, 208 lines.** The
    corpus-wide parity gate and three siblings.
  - `…/AT-35-E6-003-FINISH_cycle2_receipt.md` — three placeholders resolved (self-heal, below).
  - `…/AT-35-E6-003-FINISH_cycle3_residue_census.json` — **new**, the class A/B/C census
    re-derived at this cycle's HEAD by cycle 1's own script (which writes to a fixed path;
    cycle 1's own JSON was restored from git and is unchanged).
  - `progress.md` (cycle 2's missing entry **and** this cycle's), `kanban.md` (rows 77 and 78),
    `docs/retro/events/at-35-e6-003-finish.jsonl`, this receipt.
  - Folded from the shared checkout, not written by this cycle:
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (a
    `derived_at` stamp moved by this cycle's own `completion_atlas.py --check`) and
    `docs/retro/events/sd31-transcribe.jsonl` (another session's live append).

  No `data/` file changed and no corpus record was touched. **No file under `apps/` was
  changed** — the desktop crate compiles and passes against the changed library unmodified,
  which is the point of keeping the two field names.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. `git diff --unified=0 -- src/ tests/ | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns two lines, both the **test
  filename** quoted inside doc-comment prose
  (`sd35_race_trait_prose_comes_from_the_converted_package`). No identifier in shipping code
  carries a bundle tag — the same disposition cycles 1 and 2 recorded for the same shape.

- **Wired-integration audit result:** OK_NO_TOKENS. The same diff against
  `\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b` counts **0**.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Met for `src/rules_core/race_resolver.rs`, not for the whole criterion.** Two more of the
  six live `render_pcgen_desc*` call sites are gone — four at cycle 2's start, two now — and the
  file that held them reads no ingest format to render prose at all. **Two remain**, both in
  `src/rules_core/pilot_compute/class_feature_grant_consumer.rs`, and they are this cycle's
  named remainder.

- **Receipt rows (mechanical):**
  ```
  since=948a1d8e21e175338b6ed46208e5251d20408ef4 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=596 ratio=n/a builds_recorded=1 pcgen_live_files=45
  ```

- **PCGen residue:** `live_files=45 live_hits=300 baseline_files=260 baseline_hits=12736 verdict=PASS` — not raised on either axis. It does not **fall**, either,
  and the reason is the instrument, not the work: `pcgen_residue_gate.py`'s
  `\brender_pcgen_desc\b` pattern never matched `render_pcgen_desc_tokens`, so the two call
  sites this cycle removed were never among the 300 it counts. That blind spot is cycle 1's
  standing escalation and is deliberately **not** widened yet — widening it while two call sites
  remain would *raise* `live_hits`, which `workflow-instruction.md §8` names non-self-healable.

- **Oracle parity:** `compared=919 agree=919 disagree=0`. Epic 6 touched a live path, so the
  parity run is required. The oracle is the PCGen renderer itself, kept exactly where
  `decisions.md §11` says it belongs — in `tests/`: for **every racial-trait record every book
  under `data/corpus/` loads**, the live path (converted rule's prose + converted `_vars`) and
  the tool-side path (the record's own `DESC:` tokens through `render_pcgen_desc_tokens`, seeded
  by the tool-side reading of the same row) must render byte-identical text, PI-redaction and
  empty-prose fallbacks included. Not one disagreement.

  The first run of that gate reported **24**, and every one of them was a real defect in this
  cycle's own first implementation, fixed before commit:

  | disagreeing records | cause | fix |
  |---|---|---|
  | 20 | the join used all five steps of `converted_prose::rule_id_for`; its by-name and dropped-qualifier fallbacks resolved each `Skinwalker ~ Change Shape (<variant>)` row to the base `skinwalker_change_shape` rule | step 1 alone (`converted_id`, book-exact). A racial trait always has a real book directory; the fallbacks exist for catalog rows that do not |
  | 3 | repairing one dropped hole by collapsing the whole segment's whitespace destroyed the record's own paragraph breaks (`Suli ~ Energy Strike`, `Undine ~ Nereid Fascination`, `Nagaji ~ Hypnotic Gaze` each open a segment with a real newline) | close the gap **at the hole** — trim the following text piece's leading whitespace — instead of re-spacing the segment |
  | 1 | `Applies::Holds { Holdable::MissingRule }` was treated as undecidable, so `Elf ~ Elemental Resistance` printed all four mutually exclusive energy-type options as though the character had taken every one | decided **false**, which is the schema's own word (*Exclude*) and what `sheet_rule.rs`'s held-set evaluator already answers |

- **Movement, four buckets:**
  - **closure:** live `render_pcgen_desc*` call sites `4 → 2`; live files importing
    `pcgen_desc` for rendering `2 → 1`. Refused-type counts unchanged (all 300 are class A).
    Class C (run-time reads of `src/pcgen_import` no gate pattern matches): **93 lines / 27
    files → 82 lines / 27 files**, re-derived at HEAD rather than inherited.
  - **relabel:** none. No corpus unit moved bucket; `closed=0` is correct and expected.
  - **reachability:** none — the rendered text is byte-identical across all 919 racial-trait
    records, proved above, so nothing the player reads moved.
  - **instrument-correction:** none applied; cycle 1's `render_pcgen_desc` blind spot still
    stands, for the ratchet reason given above.

- **Refused tokens** — four types, summing to **300**, the gate's own `live_hits`:
  ```
  TYPE==100, BONUS:=91, DESC:=59, PRE[A-Z]+:=47
  ```
  plus `%CHOICE=3`, which is inside that 300. All 300 are **class A** — inside a `#[cfg(test)]`
  module of a live file, compiled out of the shipping library, blocked on operator ruling
  **B15**, now asked for the twelfth time. Class B — executable product code — remains **0**.
  Recorded as `deferral 1789239354746-at-35-e6-003-finish-270881`.

  | class | cycle 2 | now | what it needs |
  |---|---|---|---|
  | **A** — `#[cfg(test)]` fixtures carrying verbatim corpus tokens | 300 | **300** | operator ruling B15 |
  | **B** — executable product code | 0 | **0** | *closed by cycle 2* |
  | **C** — run-time reads of `src/pcgen_import` no gate pattern matches | 93 lines / 27 files | **82 lines / 27 files** | the ruling on whether the gate counts them, then the work |
  | **live `render_pcgen_desc*` call sites** (counted by no gate pattern at all) | 4 / 2 files | **2 / 1 file** | the mechanism this cycle proved, applied once more |

- **Discoveries:** three, all emitted as `correction` retro events, all found by this cycle's own
  parity gate before anything was committed:
  `1789234919524-at-35-e6-003-finish-86a5d4` (a `Holds` on a rule the converter could not resolve
  is **decided**, not unknown — cycle 2's own module doc said otherwise);
  `1789234919656-at-35-e6-003-finish-697c29` (a join built for catalog rows without a book is the
  wrong join for a record that has one);
  `1789235054698-at-35-e6-003-finish-e88d35` (cycle 2 landed its code and skipped its board rows
  and its receipt placeholders).

  **The finding under all three:** the converted package already carried more truth than the
  ingest path did, and reading it correctly — not converting anything new — was the whole job.
  `data/sheet_rules/_vars/v5b0c2ee048baad4f.json` already holds `Gnome ~ Hatred`'s attack bonus
  **with the two feat rows that raise it named as contributions**, which the old live code
  hand-modelled in a three-name `display_values_with` list. That is the same shape cycle 2
  recorded — *the conversion had already happened; only the reader had not moved* — and it has
  now been true twice in a row.

- **Self-heal applied (`§8`, self-healable: a dirty board):** cycle 2 landed its code at
  `e570ba506c` and skipped `§6` step 7 — no `progress.md` entry, no `kanban.md` row (the board
  ended at row 76) — and left `COMMIT_SHA_PLACEHOLDER`, `FULL_SUITE_RESULT_PLACEHOLDER` and
  `CLIPPY_RESULT_PLACEHOLDER` unresolved in its own receipt. This cycle wrote cycle 2's progress
  entry and kanban row 77, both marked as written by cycle 3, and stamped the commit SHA. The
  two **result** placeholders are resolved as *"not recorded by this cycle"* rather than filled:
  no output of cycle 2's own run survives, and inventing a figure for it would be exactly the
  failure `AGENTS.md` rule 9 names.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=45 live_hits=300 baseline_files=260 baseline_hits=12736 verdict=PASS` | every source file under the five live roots, code lines only (ruling B14) | `python3 scripts/pcgen_residue_gate.py --check` |
  | live `render_pcgen_desc*` call sites `= 2`, in 1 file | the five live roots minus `cache_gen`, comment lines excluded | `grep -rn 'render_pcgen_desc' src/rules_core/ src/saved_character/ src/campaign/ src/homebrew_authoring/ apps/desktop/src-tauri/src/ \| grep -v '^src/rules_core/cache_gen' \| grep -vE '^\S+:[0-9]+:\s*(///\|//!\|//\|\*)'` |
  | `class_A=300 files=45`, `class_B=0 files=0`, `class_C=82 lines / 27 files`, `counted_hits=300 counted_files=45` | the gate's own hits, partitioned by brace-matched `#[cfg(test)]` item range | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-FINISH_cycle1_residue_census.py` |
  | `compared=919 agree=919 disagree=0` | every racial-trait record every book under `data/corpus/` loads, reachable from a race | `cargo test --locked --test sd35_race_trait_prose_comes_from_the_converted_package` |
  | `since=948a1d8e21e175338b6ed46208e5251d20408ef4 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=596 ratio=n/a builds_recorded=1 pcgen_live_files=45` | `docs/work-inventory.json` before vs after | `python3 scripts/cycle_scope_gate.py --receipt --since 948a1d8e21e175338b6ed46208e5251d20408ef4 --before /tmp/wi-before-AT-35-E6-003-FINISH.json --after docs/work-inventory.json` |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` | the whole converted package | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `0` files under `data/sheet_rules/` carrying ingest syntax | the whole converted package | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | `citation_failures=0 stale_derived_at=False` | the completion atlas | `python3 scripts/completion_atlas.py --check` |
  | `non_done=0 refused=142 token_types=233 verdict=PASS` | token coverage | `python3 scripts/token_coverage.py --check` |
  | `magnitude_bearing=26396 not_held_by_engine=0` | shape/engine boundary | `python3 scripts/shape_engine_boundary.py --check` |
  | `population=0 kinds=0 citation_failures=0` | missing engine tables | `python3 scripts/missing_engine_tables.py --check` |
  | `files_checked=128 violations=0` | the bundle package's markdown | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | `pi-sweep PASS (11 hits, 11 baseline rows)` | `src/rules_core/rules_tables` | `scripts/verify.sh --only pi-sweep` |

- **Build scope verified:** run once, after the last figure-moving edit, at the tree this receipt
  commits. `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`. `cargo test --locked --lib -j 6`
  → `3341 passed; 0 failed; 15 ignored`. `cargo test --locked --no-fail-fast -j 6` →
  ``FULL_EXIT=0` — **418 targets / 8,870 passed / 0 failed**`. `cargo clippy --locked --tests -j 6` → `**0 warnings, 0 errors**`.
  **The desktop crate was run even though no `apps/` file changed** —
  `AT-35-E6-003-SWEEP` cycles 15, 16 and 17 each found it red after recording it as not-run on
  exactly that reasoning, and this cycle changes a library function the Race Traits panel calls:
  ``test result: ok. 569 passed; 0 failed; 0 ignored` (1,494s)`. The frontend suite is untouched (no `apps/desktop/src/` file changed) and runs
  at the epic wrap-up. `corpus_literal_sweep` is **skipped and named**: no corpus record changed
  (`git status` lists no `data/` path), so it would re-examine the identical tree.
  `v06_work_inventory` is **skipped and named**: the cycle changed no corpus record, no
  classifier and no `data/` file, so no unit can have moved — `closed=0 relabeled=0` above is
  read from the unchanged inventory, not asserted.

- **Sweep population:** N/A — no corpus record changed.

- **Oracle pin:** N/A for the pinned PCGen checkout. The parity oracle in this cycle is the
  in-repo tool-side renderer `src/pcgen_import/pcgen_desc.rs` run against `data/corpus/`, not
  `$PCGEN_CORPUS_ROOT`, so `scripts/pcgen-oracle-pin.env` is not a denominator for any figure
  here and is unchanged.

- **Status:** **partial** — the criterion's population is not zero at HEAD. It moved, and the
  remainder is named and sums.

- **Notes:**

  **Why `same_row_display_values` got rewritten and not just `render_description`.** The two are
  one fact seen twice. Leaving the values on the ingest side would have kept
  `bonus_chain_reader::declared_bonuses` and `race_trait_tokens::same_row_defines` as run-time
  reads in the file, and the converted prose addresses its holes by `VarId`, not by source name
  — so the ids would have had to be minted back from names the live side is forbidden to read.
  The `_vars` tables key by the id directly, which is why the rewrite is smaller than the thing
  it replaced.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`.
  The **two** remaining live `render_pcgen_desc*` call sites, both in
  `src/rules_core/pilot_compute/class_feature_grant_consumer.rs`:
  `resolved_description_for` (line 968) and
  `resolved_description_for_formula_only_desc_argument` (line 1081). Both render
  `record.raw_description` — the corpus record's stored `description` string, `%N` arguments
  intact — through `render_pcgen_desc_with_values`, and both already refuse to return anything
  unless every argument resolved. The mechanism is the one cycle 2 and this cycle both proved:
  join the record key to its converted rule (`converted_prose::rule_id_for("", "class_feature",
  key)` — this caller has no book, so the by-name step is the right one, unlike the racial-trait
  case above), seed `resolved_prose::DisplayValues` from the values each function already
  computes, render through `resolved_prose::render_description`, and gate it with a corpus-wide
  parity test against the tool-side renderer over a level × ability-modifier matrix. **The
  second function additionally reads `record_vars::package().desc_arguments`, keyed by the exact
  `%N` argument text; the converted prose's slots are `Expr`s over real variables instead, so
  that function's seeds must move from argument text to variable names** — the one piece of that
  cycle this one did not de-risk.

  Then, and only then, widen `pcgen_residue_gate.py`'s `render_pcgen_desc` pattern to a prefix
  match and re-run `--check`; it must stay at the same `live_hits`.

  **Class A (300 hits, 45 files) remains blocked on operator ruling B15** and no
  `AT-35-E6-003`/`AT-35-E6-004` cycle can take `live_hits` below 300 without it.
