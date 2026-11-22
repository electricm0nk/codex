# Cycle AT-35-E6-003-SWEEP cycle 15 — Epic 6 PCGen exit / AT-35-E6-003-SWEEP

- **Commit SHA:** `01d927a306`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by
  design; decisions.md §2, workflow-instruction.md §6 step 1)`

  Both gates ran anyway. The corpus floor, at the cycle's start tree
  `94b4db5306`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree:
  ```
  live_files=47 live_hits=366 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

  **The dispatch's cycle number was wrong for the fifth consecutive cycle.** It
  said "CYCLE NUMBER FOR THIS CRITERION: 13" and handed on cycle **12**'s
  refused-token line, summing to 373. This is cycle **15**, and the remainder at
  the start tree was **366**. Re-derived, not assumed:
  ```bash
  ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle*_receipt.md | wc -l
  # -> 14, plus one
  python3 scripts/pcgen_residue_gate.py --check   # at 94b4db5306 -> live_hits=366
  ```
  Cycles 11–14 each recorded this same defect on their own dispatch. That is
  five in a row, and `AGENTS.md` rule 8 says a recurrence is a missing
  mechanism, not bad luck. Recorded as
  `correction 1789196772150-at-35-e6-003-sweep-b37c50`.

- **Files touched:**
  - `src/pcgen_import/race_trait_tokens.rs` — gains `exclusion_guard_flags`,
    `negated_fact_gates`, `declares_preability_negated_guard`, the two private
    parsers `negated_bracket_groups` / `negated_prefact_flags`, and the
    corpus-wide round-trip gate plus a narrowness test.
  - `apps/desktop/src-tauri/src/race_trait_picker.rs` — the three readings now
    ask the converter; 116 lines of ingest-grammar parsing leave the crate.
  - `apps/desktop/src-tauri/src/equipment_catalog.rs` — three stale pins,
    red since cycle 5, self-healed with their attribution (see below).
  - `tests/sd35_live_side_names_no_ingest_qualifier.rs` — the ratchet gains
    `raw_tokens` to its scanned vocabulary and the picker to its cleared list
    (RED first).
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle14_receipt.md`
    — one quoted refused-token line reformatted into a fenced block so
    `denominator_gate.py` stops reading it as an unsourced figure. No figure
    changed.
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    — `derived_at` stamp only, written by `completion_atlas.py --check`.
  - `docs/retro/events/at-35-e6-003-sweep.jsonl`,
    `docs/retro/events/sd31-transcribe.jsonl` (a shared-checkout append folded,
    per the clean-tree rule),
    `docs/release/SD-35-corpus-sheet-completion/{progress.md,kanban.md}`, this receipt.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.** On this cycle's own diff,
  ```bash
  git diff --unified=0 94b4db5306 -- src/rules_core src/pcgen_import src/bin \
    apps/desktop/src-tauri/src tests ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ```
  → the only matches are the diff's own `diff --git` / `---` / `+++` headers
  naming `tests/sd35_live_side_names_no_ingest_qualifier.rs`. No **identifier**
  in shipping code carries a bundle tag; a test *file name* in a diff header is
  the documented citation-exclusion class cycles 9–14 recorded. Re-run
  unchanged on the final diff (step 4).

- **Wired-integration audit result:** **OK_NO_TOKENS**, both runs, no fix needed.

- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge /
  > `reach_gate.rs` readers of `raw_tokens` read `SheetRule.applies` and
  > `SheetRule.prose` instead. `render_pcgen_desc` is deleted from the live side;
  > its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under
  > `apps/desktop/`; desktop crate and frontend suites green; the 19 on-screen
  > tests still pass.

  and `AT-35-E6-004`'s closure bar, which this sweep inherits:

  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11
  > (`decisions.md §17`). […] **Evidence:** `python3
  > scripts/pcgen_residue_gate.py --check --closure` → `live_files=0
  > live_hits=0 verdict=PASS` […]

  **`AT-35-E6-003`'s own three evidence clauses are all met at HEAD** — zero
  hits under `apps/desktop/`, desktop crate and frontend suites green, the
  on-screen tests passing. `AT-35-E6-004`'s closure bar is not: the remainder is
  named by mechanism and by count below and sums under **Refused tokens**, and
  the criterion's status stays `partial` because this sweep carries that bar.

- **Receipt rows (mechanical):**
  ```
  since=94b4db5306 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=684 ratio=n/a builds_recorded=1 pcgen_live_files=46
  ```

- **PCGen residue:**
  ```
  pattern raw_tokens files=0 hits=0
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=3 hits=39
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=14 hits=91
  pattern DEFINE: files=0 hits=0
  pattern PRE[A-Z]+: files=18 hits=61
  pattern SAB: files=0 hits=0
  pattern DESC: files=26 hits=59
  pattern %CHOICE files=2 hits=8
  pattern %LIST files=1 hits=1
  pattern TYPE= files=20 hits=100
  root src/rules_core files=46 hits=359
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=0 hits=0
  identifier_files=3 identifier_hits=39
  live_files=46 live_hits=359 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Down from cycle 14's `live_files=47 live_hits=366`; never above it. **`root
  apps/desktop files=0 hits=0`** and **`pattern raw_tokens files=0 hits=0`** are
  the two lines this cycle exists to produce. The movement is confined to two
  patterns — `raw_tokens` 5 → 0 and `PRE[A-Z]+:` 63 → 61, in 19 → 18 files —
  which is exactly the 7 hits this cycle moved (5 `raw_tokens`, 2 `PRE…:`), and
  is the check that no hit was banked by reclassification into another pattern.

- **RED → GREEN, in that order.** `tests/sd35_live_side_names_no_ingest_qualifier.rs`
  was widened and run **before** a line of source was edited, and failed with
  exactly the 7 hits the python residue gate attributes to that file:
  ```
  7 executable line(s) on the live side still name an ingest qualifier
  apps/desktop/src-tauri/src/race_trait_picker.rs:539: `raw_tokens` -- for token in record.data.raw_tokens.iter().filter(|token| token.key == "ABILITY") {
  apps/desktop/src-tauri/src/race_trait_picker.rs:545: `PREVAREQ:` -- let Some(rest) = clause.trim().strip_prefix("PREVAREQ:") else { continue };
  apps/desktop/src-tauri/src/race_trait_picker.rs:556: `raw_tokens` -- for token in record.data.raw_tokens.iter().filter(|token| token.key == "PREMULT") {
  apps/desktop/src-tauri/src/race_trait_picker.rs:595: `raw_tokens` -- && record.data.raw_tokens.iter().any(|token| token.key == "PREABILITY")
  apps/desktop/src-tauri/src/race_trait_picker.rs:690: `raw_tokens` -- for token in record.data.raw_tokens.iter().filter(|token| token.key == "!PREFACT") {
  apps/desktop/src-tauri/src/race_trait_picker.rs:732: `raw_tokens` -- .raw_tokens
  apps/desktop/src-tauri/src/race_trait_picker.rs:735: `PREABILITY:` -- .any(|token| negated_bracket_groups(&token.value).iter().any(|g| g.starts_with("PREABILITY:")));
  ```
  Green after the move: `test result: ok. 2 passed; 0 failed`.

  **The gate was widened, not merely appended to.** `raw_tokens` — the ingest
  *field*, not a qualifier — joined its scanned vocabulary, because
  `token.key == "PREMULT"` names a PCGen token and carries no colon, so the
  `PRE<UPPER>+:` family walk cannot see it. The file's own header already stated
  the bar it now enforces: after the remedy "no live module names a qualifier
  position, a chain keyword … **or the ingest field itself**". Scanning the field
  every such traversal must start from closes that gap without guessing at token
  spellings. The four files the ratchet already held stayed green under the
  widening (none of them names `raw_tokens`), so the widening cost nothing and
  is now load-bearing for every file added after it.

- **What this cycle actually fixed, and why it is not a relocation dodge.**

  This is the distinction cycles 13 and 14 turned on, and it applies here too.
  Moving a literal from `apps/` into `src/pcgen_import/` lowers the residue
  gate's count whether or not it fixes anything. The test that separates a real
  move from a count-lowering one is **whether the live call site still has to
  know the ingest grammar to be written**.

  | before (live side knew the grammar) | after (live side asks a rules question) |
  |---|---|
  | walks `raw_tokens` for `ABILITY`, splits on `\|`, checks position 1 is `AUTOMATIC`, strips `PREVAREQ:`, `rsplit_once(',')`, tests `,0` | `race_trait_tokens::exclusion_guard_flags(&record.data)` |
  | walks `raw_tokens` for `PREMULT`, scans `[...]` groups for a leading `!`, splits clauses on `=` | *(same call)* |
  | walks `raw_tokens` for `PREABILITY` as a positive-dependency marker | *(same call)* |
  | walks `raw_tokens` for `!PREFACT` and parses `1,ABILITIES,A=True,B=True` | `race_trait_tokens::negated_fact_gates(&record.data)` |
  | walks `raw_tokens` for `PREMULT` and tests each negated group for a `PREABILITY:` prefix | `race_trait_tokens::declares_preability_negated_guard(&record.data)` |

  In every row the picker stops naming a **token key**, a **bracket branch**, a
  **qualifier prefix** and a **field position**, and starts asking *which flags,
  already set by another selection, block this one?* / *what flags does this
  row's suppression gate name?* / *does this row spell its guard the slipped
  way?* — each answerable without ever having read a `.lst` file. The four
  spellings, and the ordering between them (branch 4 is a **fallback**, never an
  addition), moved across with the code they justify; the picker no longer has
  to know the corpus states one relation four ways.

  The contrast case, again refused: `render_pcgen_desc` was **not** relocated,
  for the reason cycle 14 re-verified and this cycle did not re-litigate — its
  signature takes a raw `DESC:` token, so moving the renderer would leave the
  live side holding and passing the token and lower the count by 4 while
  changing nothing.

- **How losslessness is proved** — one gate over the live corpus, not a fixture
  (`decisions.md §4`).
  `race_trait_tokens::moved_from_race_trait_picker_tests::the_exclusion_guard_readings_are_unchanged_by_this_module`
  walks **every** `data/corpus/<book>/race_trait/**/*.json` record and, for each,
  evaluates all three readings twice: once with the picker's code as it stood at
  `94b4db5306` — transcribed inline, deliberately transcribed rather than
  referenced, because a round trip proved against a paraphrase proves nothing —
  and once with the new functions, then asserts they agree.

  Result: **0 disagreements over 919 records**, of which **415** come back
  carrying an exclusion guard, **4** spell their guard the `!PREABILITY` way,
  and **2** declare a multi-flag `!PREFACT` gate. The test refuses to pass on an
  empty walk: all four populations must be non-empty, so a walk that silently
  stopped finding records cannot agree with itself about nothing. The figures
  are printed, never asserted as constants — ingesting a book moves them and
  this file must not have to change.

  A second test, `the_readings_keep_their_deliberate_narrowness`, pins the four
  things a tidy-up would most plausibly widen: `PREVAREQ:<flag>,1` is the
  opposite statement to `,0` and is not a guard; branch 4 stays a fallback and
  does not also fire on a row the earlier branches guarded; a positive `PREMULT`
  branch contributes nothing; and **two one-flag gates are not one two-flag
  gate**, which is why `negated_fact_gates` returns `Vec<Vec<String>>` rather
  than a flattened list — flattening would have made the two indistinguishable
  and silently changed which rows the multi-flag finding reports.

- **The desktop crate was red, and had been for ten cycles.** This is the
  cycle's second finding and the more uncomfortable one.

  `apps/` is on epic cadence (`workflow-instruction.md §6` step 3: the desktop
  crate runs only when a cycle touches `apps/`). This cycle touched it, so it
  ran — and `equipment_catalog` failed two tests. Before assuming this cycle
  caused them, they were reproduced at the cycle's own start tree, then
  attributed by bisection over a separate worktree rather than by reading commit
  subjects:

  | tree | `equipment_catalog` |
  |---|---|
  | `6fe6131922~1` (`b069f01962`) | **green**, 18 passed |
  | `6fe6131922` | **red**, `left: 311 / right: 307` |
  | `1f425d5124~1`, `1f425d5124`, `bdae51f9f6` (cycle 13) | red, same figures |
  | `94b4db5306` (this cycle's start) | red, same figures |

  `6fe6131922` is `AT-35-E6-003-SWEEP` **cycle 5**, 2026-09-11: it moved
  `gen_equipment_gap_tables.rs::safe_description` from rendering a description
  only to *decide* whether to keep it, then storing the raw one, to storing the
  rendered text. ACG's four Equipmods rows were exactly that case — their stored
  strings leaked a bare `%` onto the player's sheet and were refused a
  description downstream. They now carry rendered prose.

  So the code was right and the **pins** were stale, in three places that move
  together: the raw-leak profile loses its `("ACG", "Equipmods") => 4` bucket
  (total 59 → 55), `with_description("ACG")` rises by the same 4 (307 → 311),
  and the catalog total rises by the same 4 (5390 → 5394). Three assertions
  moving by one number in one direction is what says this is one real fix
  landing, not three unattributed drifts. All three updated with that attribution
  written beside them, per self-heal posture (`workflow-instruction.md §8`,
  "a count assertion … moved by a deliberate change").

  The mechanism question this raises is not the pins. It is that **epic cadence
  let a red crate stay red across cycles 5–14 while every one of them shipped
  reporting "desktop crate: epic cadence"**. The redness turned out benign, but
  nothing in the process could have told the difference. Recorded as
  `incident 1789198679282-at-35-e6-003-sweep-01dc50` and
  `correction 1789198672899-at-35-e6-003-sweep-7c3ef7`.

- **Oracle parity:** **not run, and why.** No `Number` mapping was added
  (`workflow-instruction.md §6` step 3 runs the oracle comparison when one is),
  no corpus record changed, and no converter mapping row was added. This cycle
  changes **which module owns a set of string traversals**, and the traversals
  themselves are pinned unchanged against the whole live corpus by the
  round-trip gate above — a stronger statement about this particular change
  than a 29-character roster is, since it covers **919 records** rather than the
  handful a roster reaches. Cycle 13's `…_cycle13_sheet-parity-after.json`
  remains the standing parity artifact and nothing in this cycle can move it.

- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. Epic 6 closes no corpus unit.
  - **relabel (bucket to bucket):** none. `regressed=0 added=0 dropped=0`;
    `docs/work-inventory.json` is byte-identical to `94b4db5306`'s.
  - **reachability:** 7 live-side PCGen hits removed, one live file to zero, and
    it is the **only file under `apps/` the gate ever listed** — so
    `AT-35-E6-003`'s `apps/desktop` evidence clause is met rather than advanced.
    The whole `raw_tokens` pattern is now 0. The reachable (non-`#[cfg(test)]`)
    remainder falls **15 → 8** and the files carrying it **5 → 3**.
  - **instrument-correction:** the ratchet gate was widened (a new pattern,
    `raw_tokens`) and gained a fifth held file. Separately, three stale pins in
    the desktop crate were corrected, which closes nothing but makes a
    ten-cycle-old real improvement visible for the first time.

- **Refused tokens** — seven types, summing to **359**, which is the gate's own
  `live_hits`:
  ```
  TYPE==100, BONUS:=91, PRE[A-Z]+:=61, DESC:=59, render_pcgen_desc=39, %CHOICE=8, %LIST=1
  ```
  `raw_tokens` leaves this list entirely (5 → 0). Recorded as
  `deferral 1789196783797-at-35-e6-003-sweep-6431e1`.

  Split by what can move without an operator ruling
  (`python3 .../AT-35-E6-003-SWEEP_cycle5_test_region_census.py`, re-run at
  HEAD: `live_hits=359 hits_inside_cfg_test=351 hits_outside=8`):

  | where | hits | files |
  |---|---|---|
  | inside a `#[cfg(test)]` module in a live file | 351 | 42 |
  | reachable by code work | **8** | **3** |

  The reachable 8, by the job that owns each:

  | job | hits | files |
  |---|---|---|
  | the `render_pcgen_desc` / `pcgen_desc.rs` catalog rewire | 4 | `class_feature_pool_catalog.rs`, `pcgen_desc.rs`, `pilot_compute/class_feature_grant_consumer.rs` |
  | the `PU_*_DESC_TOKEN` verbatim corpus transcriptions | 4 | `pilot_compute/mod.rs` |

  **This cycle took the whole of the one job cycle 14 costed as unblocked-once-
  scoped**, and it was the largest of the three it named (7 of 15 hits). The two
  that remain are each blocked on a named artifact, unchanged from cycles 13 and
  14 and not re-litigated here:
  * `render_pcgen_desc`'s real fix is the converter-side prose carrier
    (`…/AT-35-E6-003_cycle5_converter-prose-blocker.md`): the converter emits
    resolved prose and the live side never sees a `DESC:` token at all.
  * The four `PU_*_DESC_TOKEN` constants are the book's own words, pinned
    byte-for-byte against the `.lst` files by
    `sd27_pu_class_feature_descriptions_carry_the_characters_numbers`. Cycles
    10–14 declined to reword them and this cycle declines for the same reason.
    They are **counted** in the remainder, never exempted.

- **Discoveries:** two, both recorded as `correction` events.
  1. The dispatch's cycle number and refused-token line, wrong for the fifth
     consecutive cycle (`…-b37c50`). Mechanism, asked for a fifth time: derive
     both at dispatch time from the receipt directory rather than carry them in
     prose.
  2. The desktop crate had been red since cycle 5 and epic cadence hid it
     (`…-7c3ef7`, plus `incident …-01dc50`). Neither was predicted by
     `token-coverage.json` or the atlas, because neither instrument watches a
     crate that is not run.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=46 live_hits=359`, `apps/desktop files=0 hits=0` | every source file under the five live roots, code lines only (ruling B14) | `python3 scripts/pcgen_residue_gate.py --check` |
  | 366 → 359, 47 → 46 files | same | the same command at `94b4db5306` and at HEAD |
  | `raw_tokens` 5 → 0; `PRE[A-Z]+:` 63 → 61 in 19 → 18 files | the same scan, one pattern each | `python3 scripts/pcgen_residue_gate.py --check` |
  | reachable 15 → 8 in 5 → 3 files | the 359 hits, split on `#[cfg(test)]` membership | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` |
  | 7 RED lines, then 0 | the one live file the ratchet newly lists as cleared | `cargo test --locked --test sd35_live_side_names_no_ingest_qualifier` (RED at `94b4db5306` + the widened test, GREEN at HEAD) |
  | 919 records / 415 guarded / 4 `!PREABILITY` / 2 multi-flag gates, 0 disagreements | every `data/corpus/<book>/race_trait/**/*.json` record | `cargo test --locked --lib race_trait_tokens -- --nocapture` |
  | green at `6fe6131922~1`, red at `6fe6131922` and every tree after | the desktop crate's `equipment_catalog` tests | `git worktree add /tmp/wt <sha> && cd /tmp/wt/apps/desktop/src-tauri && cargo test --locked -j 4 equipment_catalog` |
  | ACG 307 → 311; leak total 59 → 55; catalog total 5390 → 5394 | every row the equipment catalog serves | `cd apps/desktop/src-tauri && cargo test --locked -j 4 equipment_catalog` (each assertion's own `left`) |
  | `rust_lines_changed=684` | the cycle's own diff since `94b4db5306`, measured at HEAD with every file tracked | `python3 scripts/cycle_scope_gate.py --receipt --since 94b4db5306 --before /tmp/wi-before-at-35-e6-003-sweep.json --after docs/work-inventory.json` |
  | `records=49438 converted=49296 refused=142` | every corpus record the sheet-rule converter reads | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | 14 prior receipts, so this is cycle 15 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle*_receipt.md \| wc -l` |

- **Build scope verified** — **once**, after the last figure-moving edit:

  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → `LIB_EXIT=0`
  - `cargo test --locked --no-fail-fast -j 6` → **416 targets / 8,856 passed /
    0 failed / 68 ignored / `FULL_EXIT=0`**, zero `FAILED` lines
    (`grep -c FAILED` → 0). Cycle 14 recorded 416 targets and 8,854 passed; the
    +2 are this cycle's two new `race_trait_tokens` tests and no target was
    added, which is the arithmetic that says nothing else moved.
  - `cargo clippy --locked --tests -j 6` → **`CLIPPY_EXIT=0`, 0 warnings, 0 errors**
  - **desktop crate, run here because this cycle touched `apps/`:**
    `cd apps/desktop/src-tauri && cargo test --locked -j 4` → **575 passed /
    0 failed / `DESKTOP_EXIT=0`**, and `cargo clippy --locked --tests -j 4` →
    `DESKTOP_CLIPPY_EXIT=0`, 0 warnings. The 19 on-screen tests are inside that
    575 and pass.
  - **frontend, same reason:** `npm test` → **101/101 test files passed**,
    `FRONTEND_EXIT=0`; `npm run typecheck` → `TYPECHECK_EXIT=0`.
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293
    verdict=PASS`, identical to cycles 3–14
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l`
    → **0**
  - `python3 scripts/pcgen_residue_gate.py --check` → `live_files=46
    live_hits=359 … verdict=PASS`
  - `python3 scripts/completion_atlas.py --check` → `unclassified=0 overlap=0
    done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
    citation_failures=0`, exit 0
  - `python3 scripts/token_coverage.py --check` → `non_done=0 tokened=0
    token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1
    verdict=PASS`, exit 0
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, exit 0
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, exit 0
  - `python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'`
    → `files_checked=124 violations=0` at HEAD (123 when it first ran, before this receipt existed). It was **red on arrival** with one
    violation, in cycle 14's receipt at a line the "fold cycle 14's own figure
    correction" commit added after cycle 14 ran the gate; self-healed by fencing
    the quoted line, no figure changed.
  - `python3 scripts/denominator_gate.py --check-provenance` →
    `files_checked=241 figures_examined=575 violations=0`
  - `scripts/verify.sh --only pi-sweep` → **RESULT: PASS** (1 stage passed,
    `PISWEEP_EXIT=0`)
  - `cargo run --locked --bin corpus_literal_sweep` — **not run, and why**: no
    corpus record changed this cycle (`git status --porcelain data/` empty), and
    `workflow-instruction.md §6` step 3 runs it only when they do.

- **Sweep population:** N/A — no corpus record changed. `data/corpus/**` is
  byte-identical at HEAD (`git status --porcelain data/` empty). The corpus was
  **read** by the new round-trip gate, never written.

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

- **Status: `partial`.** `AT-35-E6-003`'s own three evidence clauses are met;
  `AT-35-E6-004`'s closure bar, which this sweep carries, is not — the
  criterion's population is 359, not zero, at HEAD.

- **Notes:** the judgment call was spending four desktop-crate builds bisecting
  an inherited red rather than updating three pins on the assumption that a
  fix had landed. The assumption would have been right; making it would also
  have been indistinguishable from papering over a regression this cycle caused,
  which is the only reason the attribution was worth the wall time.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle)`; the whole
  remainder, **359 code hits in 46 files**, of which **8 hits in 3 files** are
  reachable without the `#[cfg(test)]` ruling. **There is no unblocked code job
  left, and this time the statement is exact rather than inherited** — cycle 14
  named three reachable jobs and this cycle took the one that was doable; the
  two that remain are the two it said were blocked.
  1. the **converter prose carrier**
     (`…/AT-35-E6-003_cycle5_converter-prose-blocker.md`) — unblocks
     `render_pcgen_desc`, 4 hits. Its binding blocker, measured in that
     document, is that the converter writes no descriptive prose at all for a
     record whose description carries an argument the formula side refuses.
  2. the `PU_*_DESC_TOKEN` transcriptions, 4 hits, which are the book's words
     and move only when `pcgen_desc.rs` goes.

  Dispatching "the next sweep cycle" against this remainder will produce
  nothing. **The next dispatch should name (1) as its own criterion and build
  the artifact.** And the operator ruling on `#[cfg(test)]` regions still stands
  between the gate and **351 of the 359**; it has now been asked by seven cycles
  and no amount of dispatching will move it.
