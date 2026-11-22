# Cycle AT-35-E6-003-FINISH cycle 1 — Epic 6 PCGen exit / AT-35-E6-003-FINISH

- **Commit SHA:** `244c5c8c1c`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by
  design; decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `4cf0ba58b3`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and
  again at HEAD, unchanged both times:
  ```
  live_files=45 live_hits=304 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-FINISH_cycle1_residue_census.py` — **new.**
    The measurement this cycle was dispatched to take, before any edit.
  - `…/AT-35-E6-003-FINISH_cycle1_residue_census.json` — its output.
  - this receipt, `progress.md`, `kanban.md`, `docs/retro/events/at-35-e6-003-finish.jsonl`.

  **No `src/`, `apps/` or `data/` file was changed.** That is the cycle's
  substantive decision and it is argued under **Notes**.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS — the cycle adds no shipping
  code. `git diff --unified=0 4cf0ba58b3...HEAD -- src apps` is empty; the run
  over the epic's file-touch set returns only pre-existing `tests/sd27_*` /
  `tests/sd35_*` **filenames quoted inside earlier cycles' receipts**, which is
  prose about a test file, not an identifier in shipping code.

- **Wired-integration audit result:** OK_NO_TOKENS — same empty shipping diff.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge /
  > `reach_gate.rs` readers of `raw_tokens` read `SheetRule.applies` and
  > `SheetRule.prose` instead. `render_pcgen_desc` is deleted from the live
  > side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under
  > `apps/desktop/`; desktop crate and frontend suites green; the 19 on-screen
  > tests still pass.

- **Receipt rows (mechanical):**
  ```
  since=4cf0ba58b3fa96f5a671993852dd1273c2f4e92d residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=45
  ```

- **PCGen residue:** `live_files=45 live_hits=304 baseline_files=260
  baseline_hits=12736 verdict=PASS` — identical to cycle 17's. Not lowered, and
  **not raised**: no live-side read was added.

- **Oracle parity:** N/A — no live path was touched and no `Number` mapping was
  added.

- **Movement, four buckets:**
  - **closure:** none. Zero units moved into `DONE`; zero hits cleared.
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** **the finding, and it is not applied.** The census
    shows `pcgen_residue_gate.py` is under-counting the live-side PCGen surface.
    Applying the correction would raise `live_hits` above the baseline and the
    gate refuses an upward rebaseline by design (it is a ratchet). Widening the
    gate's patterns is therefore an operator decision, not a cycle's — recorded
    here and escalated, **not** made.

- **Refused tokens** — five types, summing to **304**, the gate's own `live_hits`:
  ```
  TYPE==100, BONUS:=91, DESC:=59, PRE[A-Z]+:=51, %CHOICE=3
  ```
  Recorded as `deferral 1789221241602-at-35-e6-003-finish-ea491d`. The census
  splits them into two classes that need different things, and adds a third the
  gate does not count at all.

  | class | hits | files | what it is | what it needs |
  |---|---|---|---|---|
  | **A** — inside a `#[cfg(test)]` module of a live file | **300** | 45 | Unit tests that build fixtures from **real verbatim corpus token text** and feed the parsed record to a live function (`parse_equipment_entries(text)` → `compute_arms_armor_effect(record)`), or assert on a transcribed `BONUS:` string. Compiled out of the shipping binary. | **Operator ruling B15** on whether a `#[cfg(test)]` region is live code. Asked for since cycle 10 — cycle 17 counted nine askings, so this is the **tenth**. |
  | **B** — executable product code | **4** | 1 | `src/rules_core/pilot_compute/mod.rs`'s `PU_RESOLVABLE_DESCRIPTIONS`: verbatim Pathfinder Unchained `DESC:` strings carrying `PREABILITY:` / `PREVAREQ:` / `PREVARGT:` gate syntax, handed to the PCGen renderer at run time by `pu_resolved_description`. | The **same one mechanism** as class C below. See **Notes**. |
  | **C** — run-time reads **no gate pattern matches** | *uncounted* — **100 lines, 28 files** | — | Live roots calling `src/pcgen_import::` at run time: the renderer, `ingest_record::token_pairs` / `bonus_chain_qualifiers` / `rebuild_bonus_token`, `lst_parser::*`, `ir_converter::*`, `race_trait_tokens`, `pool_member_tokens`. **17 of the 100 are under `apps/desktop/`**, where the gate prints `files=0 hits=0`. | A ruling on whether the gate must count them, then the work. |

- **Discoveries:** one, emitted as a `correction` retro event
  (`1789221228030-at-35-e6-003-finish-b64a81`), and it changes what "zero" means
  for `AT-35-E6-004`.

  **The gate cannot see the reads that are left.** It counts the identifier
  `render_pcgen_desc` with the regex `\brender_pcgen_desc\b`. The live side does
  not call a function by that name any more — it calls
  `render_pcgen_desc_tokens` and `render_pcgen_desc_with_values`, and the
  trailing `_` defeats the word boundary:
  ```
  \brender_pcgen_desc\b matches 'render_pcgen_desc':             True
  \brender_pcgen_desc\b matches 'render_pcgen_desc_tokens':      False
  \brender_pcgen_desc\b matches 'render_pcgen_desc_with_values': False
  ```
  So the gate prints `pattern render_pcgen_desc files=0 hits=0` while six live
  call sites in three `src/rules_core/` files hand PCGen `DESC:` token strings to
  the PCGen renderer at run time:
  ```
  src/rules_core/race_resolver.rs:90,303
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs:968,1081
  src/rules_core/pilot_compute/mod.rs:141,28791
  ```
  And it prints `root apps/desktop files=0 hits=0` — the literal sentence
  `AT-35-E6-003`'s Evidence row asks for — while 17 lines across
  `corpus_fixtures.rs`, `feat_catalog.rs`, `race_trait_picker.rs`,
  `reach_gate.rs` and two `apps/desktop/src/boundary/*.ts` files call the
  converter at run time.

  This is this repo's standing lesson in its exact recorded form —
  `validate-proxies-against-known-truth`, and `AGENTS.md` rule 7: *a proof is
  only as wide as the cases it covers*. The patterns were written against the
  names the code had in Epic 1. The code was refactored; the patterns were not.
  **`live_hits=0` under the present patterns would be a false closure, and
  `AT-35-E6-004` would certify it.** That is the thing worth stopping for.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=45 live_hits=304 … verdict=PASS` | every source file under the five live roots, code lines only (ruling B14) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `class_A_in_cfg_test_hits=300 files=45` | the same 304 hits, partitioned by brace-matched `#[cfg(test)]` item range | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-FINISH_cycle1_residue_census.py` |
  | `class_B_executable_hits=4 files=1` | the same 304 hits, the complement of class A; all four printed verbatim by the census | same command |
  | `class_C_lines=100 files=28`, `by_root=apps/desktop=17, src/rules_core=83` | every non-comment, non-`#[cfg(test)]` line under the five live roots naming a `src/pcgen_import/` symbol | same command |
  | the three regex-match booleans | the gate's own compiled `render_pcgen_desc` pattern | same command, final section |
  | `closed=0 … pcgen_live_files=45` | `docs/work-inventory.json` before vs after | `python3 scripts/cycle_scope_gate.py --receipt --since 4cf0ba58b3 --before /tmp/wi-before-AT-35-E6-003-FINISH.json --after docs/work-inventory.json` |
  | `citation_failures=0 stale_derived_at=False` | the completion atlas | `python3 scripts/completion_atlas.py --check` |
  | `files_checked=126 violations=0` | the bundle package's markdown | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |

- **Build scope verified:** **no build was run, and that is the point.** The
  cycle changed no Rust (`rust_lines_changed=0`), no `data/`, no `apps/`, and no
  classifier, so a workspace suite would compile the identical tree cycle 17
  already proved green at `4cf0ba58b3` and would re-prove nothing. The
  instruments that *can* move on a docs-only tree were all run at HEAD and are
  quoted above: `pcgen_residue_gate.py --check`, `completion_atlas.py --check`,
  `denominator_gate.py --check`. `sheet_rule_convert -- --check`,
  `corpus_literal_sweep`, `token_coverage.py` and the `data/sheet_rules/` grep
  are skipped because nothing they read changed at this tree.

- **Sweep population:** N/A — no corpus record changed.

- **Oracle pin:** N/A — no figure came from the pinned corpus.

- **Status:** **partial** — the honest exit this dispatch names, taken on
  measurement rather than on fatigue.

- **Notes:**

  **Why no code was written, when the floor was zero.** The dispatch was explicit
  that trimming a handful of hits and paying a full suite for them is
  unacceptable, and equally explicit about the exit: *a token that is
  load-bearing in executable code, or a case needing an operator ruling* →
  census, `partial`, stop. The census found both, plus a third thing nobody
  asked for.

  Of the 304, **300 cannot be touched by code work at all** without ruling B15 —
  they are `#[cfg(test)]` fixtures whose entire value is that they carry *real
  verbatim corpus tokens* into a live function. Rewriting them to construct
  records by hand would delete the only thing they prove and would be this
  repo's standing anti-pattern, *a guard that is never asked about the real
  population*. Relocating them — a new directory the gate's `os.walk` does not
  reach — is an exclusion list wearing a directory's name, which this dispatch
  forbids in terms.

  That leaves **4**. Cycle 10's receipt already ruled on them and its reasoning
  survived this cycle's re-derivation: the four `PU_*_DESC_TOKEN` transcriptions
  are pinned byte-for-byte against `pu_abilities_class.lst` by
  `sd27_pu_class_feature_descriptions_carry_the_characters_numbers`, so *editing
  them would be a lie about what the book says*; they and the renderer that
  consumes them **leave together or not at all**. The renderer's live call sites
  are class C — and two of the six are in `src/rules_core/race_resolver.rs`,
  which is **not in this epic's file-touch set**. Doing the 4 alone would be
  exactly the forbidden trim; doing them properly requires writing outside the
  granted scope. `AGENTS.md` Blocker Discipline, disposition 2: prepare the
  change, escalate, wait.

  **What the operator is being asked for — two rulings and one mechanism, not a
  warning.** `AGENTS.md` rule 8 says a recurrence is a missing mechanism, and
  "does `#[cfg(test)]` count" is now nine cycles old. So it is put as a decision
  with its consequence attached:

  1. **B15 — is a `#[cfg(test)]` region live code?** If **no** (the ruling B14
     already made about comments: it does not execute, and `tests/**` is already
     never scanned), the one-line change is a `cfg_test_ranges` skip in
     `code_only()`, the same shape and the same place as B14's comment skip, and
     `live_hits` falls 304 → 4 as an **instrument correction that closes
     nothing**. If **yes**, the 300 need a relocation policy and this epic needs
     another two or three cycles for it.
  2. **Must the gate count run-time calls into `src/pcgen_import/`?** If yes —
     and the criterion's own words (*"`render_pcgen_desc` is deleted from the
     live side"*) say yes — then the patterns must widen to the class-C symbol
     list, `live_hits` **rises**, the ratchet must be re-baselined upward once
     under an explicit ruling, and `AT-35-E6-004` is further away than the gate
     currently says. If no, then `AT-35-E6-004`'s `live_hits=0` certifies a live
     side that still runs the PCGen parser, and the bundle should say so in
     those words rather than discover it in Starfinder.

  Whichever way (2) goes, **`AT-35-E6-004` must not be run against the present
  patterns.** That is the one thing this cycle asks not be skipped.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus
  units by design)`. The work is blocked, not sized: no `AT-35-E6-003` cycle
  should be dispatched until B15 and the class-C ruling land, because every
  available move either needs one of them or is the forbidden trim. When they
  land, the next cycle's scope is the class-C mechanism — the six renderer call
  sites and the `PU_RESOLVABLE_DESCRIPTIONS` table leaving together, which needs
  `src/rules_core/race_resolver.rs` added to the epic's file-touch set.
