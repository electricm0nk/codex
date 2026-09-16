# Cycle AT-35-E6-003-SWEEP cycle 16 — Epic 6 PCGen exit / AT-35-E6-003-SWEEP

- **Commit SHA:** `dca9c80fe3`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by
  design; decisions.md §2, workflow-instruction.md §6 step 1)`

  Both gates ran anyway, at the cycle's start tree `8cbb052583`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree:
  ```
  live_files=46 live_hits=359 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

  **The dispatch's cycle number was wrong for the sixth consecutive cycle.** It
  said "CYCLE NUMBER FOR THIS CRITERION: 14". Fifteen receipts are on disk, so
  this is cycle **16**. Its refused-token line (summing to 359) was, for the
  first time in six cycles, correct — it is cycle 15's, and cycle 15's remainder
  is still standing. Re-derived, not assumed:
  ```bash
  ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle*_receipt.md | wc -l
  # -> 15, plus one
  python3 scripts/pcgen_residue_gate.py --check   # at 8cbb052583 -> live_hits=359
  ```
  Recorded as `correction 1789201545876-at-35-e6-003-sweep-06bb38`. Cycles 11–15
  each recorded this same defect. `AGENTS.md` rule 8: a recurrence is a missing
  mechanism, not bad luck. The mechanism, asked for a sixth time, is one line —
  derive the number from the receipt directory at dispatch time.

- **What this cycle did, and why it is not another sweep pass.**

  Cycle 15 closed with an explicit instruction: *"Dispatching 'the next sweep
  cycle' against this remainder will produce nothing. The next dispatch should
  name (1) as its own criterion and build the artifact"* — (1) being the
  **converter prose carrier**, the named blocker behind the
  `render_pcgen_desc` hits. The dispatch arrived as a sweep anyway. Under
  `AGENTS.md`'s blocker doctrine ("a blocker bigger than one cycle is a
  sequencing problem, not an exemption — decompose it and run the cycles") this
  cycle took the blocker rather than re-running the sweep.

  The first move was to **re-derive the blocker at HEAD instead of inheriting
  it**, and it had drifted. That is this cycle's headline correction.

- **Correction: the blocker document's mechanism was stale, and the real defect
  is eight times larger.**

  `AT-35-E6-003_cycle5_converter-prose-blocker.md` §6 item 1 named the mechanism
  as `FORMULA:CL-no-owner` refusing the whole `DESC` row, measured at 30 feat
  rows. **Cycle 6 fixed that path** (`prose::words_for_unlowerable`, which
  renders an unlowerable `|`-argument into words rather than refusing the row),
  and the document was never re-derived against the tree afterwards.

  The defect standing at HEAD is a different one, one level up:
  **`convert_record` takes prose from `DESC:` / `BENEFIT:` / `SPROP:` / `SAB:` /
  `TEMPDESC:` rows and from nowhere else.** A record that carries structured
  tokens *and* a `description` field, but no prose row, converted to a rule set
  with **no prose at all**, and the book's own sentence was dropped on the floor.

  `data/corpus/advanced_players_guide/spell/blindness_deafness_only_cause_blindness.json`
  is the clean shape — its only tokens are `CLASSES` and `DOMAINS`, and its
  record states *"You call upon the powers of unlife to render the subject
  blinded or deafened, as you choose."* The converted file carried a full
  `granted_by` and `provenance` and no `prose` key.

  Measured, not estimated: **241 records** of the **7,619** whose corpus row
  states a printable description, across **every** kind — equipment,
  equipment_modifier and spell, not the feat/spell pair the document named.
  Recorded as `correction 1789201546010-at-35-e6-003-sweep-d53d4d`.

- **Files touched:**
  - `src/pcgen_import/sheet_rule/mod.rs` — `printable_description` extracted out
    of `description_only_rules` (the five refusal conditions unchanged, so the
    two doors apply one bar), and the token path gains the fallback.
  - `tests/sheet_rule_convert_gate.rs` — the new corpus-wide gate
    `a_converted_record_never_drops_the_description_its_corpus_row_states`
    (RED first).
  - `data/sheet_rules/**` — **310 regenerated rule files**, the package's only
    change.
  - `docs/retro/events/at-35-e6-003-sweep.jsonl`,
    `docs/release/SD-35-corpus-sheet-completion/{progress.md,kanban.md}`, this receipt.

  **No live-side file was touched.** The fix is entirely on the converter side,
  which is where `workflow-instruction.md §8` says it belongs.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS**, on this cycle's own diff:
  ```bash
  git diff --unified=0 8cbb052583 -- src/rules_core src/pcgen_import src/bin \
    apps/desktop/src-tauri/src tests ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ```
  → no output at all, not even a diff header. Unchanged when re-run on the final
  diff (step 4). The cumulative `merge-base ... HEAD` form still reports the
  documented citation-exclusion class (a test *file name* quoted inside a doc
  comment, and `tests/sd13_progression/` rename headers) that cycles 9–15
  recorded; none of it is this cycle's and none is an identifier in shipping
  code.

- **Wired-integration audit result:** **OK_NO_TOKENS**, both runs, no fix needed.
  The cumulative form's hits are the word "placeholder" inside prose comments
  describing PCGen's `%N` slots — pre-existing, not this cycle's.

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

  `AT-35-E6-003`'s three evidence clauses were met at cycle 15 and are unmoved
  here. `AT-35-E6-004`'s closure bar (operator ruling B14, `decisions.md §17`:
  `live_files=0 live_hits=0`), which this sweep carries, is not met — the
  remainder is 359 and is named by mechanism under **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  since=8cbb052583 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=107 ratio=n/a builds_recorded=1 pcgen_live_files=46
  ```

- **PCGen residue: `live_files=46 live_hits=359`, identical to cycle 15.**
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
  **This cycle removed no residue hit, and says so plainly.** It removed the
  *blocker* standing in front of four of them. Stating that as movement would be
  the exact error `AGENTS.md` rule 9 names — the counter did not move and the
  receipt must not imply it did. The gate never rose, which is the bar
  `workflow-instruction.md §8` sets for a converter-side cycle.

- **RED → GREEN, in that order.** The gate was written and run **before** a line
  of `src/` was edited:
  ```
  converted rule files=49296 whose corpus record states a description=7619 dropping it=241
  thread '...' panicked: 241 converted record(s) of 7619 drop the description their
  corpus row states, e.g. ["advanced_players_guide:equipment:abacus.json",
  "advanced_players_guide:equipment:alchemist_s_kit.json", ...]
  test result: FAILED. 0 passed; 1 failed
  ```
  After the converter fix and one regeneration:
  ```
  converted rule files=49296 whose corpus record states a description=7619 dropping it=0
  test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  ```

- **How losslessness is proved** — one gate over the live package and the live
  corpus directory, not a fixture (`decisions.md §4`).

  The gate walks every file in `data/sheet_rules/`, joins each to its
  `data/corpus/<book>/<kind>/<slug>.json` record, and asserts that a record
  stating a printable description converts to a rule set carrying prose. It
  refuses to pass on an empty walk — both `examined > 1000` and
  `with_description > 100` must hold — so a walk that silently stopped finding
  records cannot agree with itself about nothing. The figures are printed, never
  asserted as constants: ingesting a book moves them and this file must not have
  to change.

- **What the proof does NOT cover** (`AGENTS.md` rule 7, stated because a narrow
  proof that passes is more dangerous than none).

  The fix regenerated **310** files; the gate can address only **241** of them.
  The other **69** are records whose corpus file slug differs from the rule-file
  slug (`…/equipment_modifier/crrsve_brst_m.json` has no corpus file at that
  path; `core_rulebook/equipment/barding_banded_mail.json` likewise), so the
  gate's path-join cannot reach them. The converter reaches them through
  `RecordRef.description`, which is the index's own join and does not go through
  a path. **The 69 are therefore fixed but ungated**, and a regression in them
  would not be caught by this test. Re-derive the split:
  ```bash
  git status --porcelain data/sheet_rules | wc -l            # -> 310
  # of those, the ones whose corpus data.description is absent at the joined path -> 69
  ```
  Building the slug-independent join is named as next-cycle work below rather
  than claimed here.

- **Why the fallback is a fallback and not an addition.** A record whose rows
  already state prose keeps exactly the prose those rows state. A `DESC:` row is
  the authored sheet line, with its `%N` slots; the `description` field is the
  same sentence in a second, unslotted form. Appending both would print the
  sentence twice on the sheet. The bar the fallback applies is
  `printable_description`'s — byte-identical to the one the no-source-row path
  has always applied (`pi_hit`, `license_pi`, `pi_fields`, `FORBIDDEN_LITERALS`,
  a glued `PRE<KIND>:` head, and a `%1` with no argument row to fill it), so a
  record cannot reach the sheet through the new door carrying words the old door
  would have refused. That the extraction changed nothing is proved by
  `package_carries_no_source_format_literal` and the `pi-sweep` stage staying
  green over a package with 310 newly-prose-bearing files.

- **Oracle parity:** **not run, and why.** No `Number` mapping was added
  (`workflow-instruction.md §6` step 3 runs the oracle comparison when one is);
  the fix adds no magnitude and no formula, only a `ProsePiece::Text`. No corpus
  record changed. Cycle 13's `…_cycle13_sheet-parity-after.json` remains the
  standing parity artifact and nothing in this cycle can move it.

- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. Epic 6 closes no corpus unit.
  - **relabel (bucket to bucket):** none. `regressed=0 added=0 dropped=0`;
    `docs/work-inventory.json` is byte-identical to `8cbb052583`'s.
  - **reachability:** **zero residue hits removed** — see the residue section.
    What moved is upstream of them: 310 converted records that carried no prose
    now carry the book's sentence, which is the artifact the
    `render_pcgen_desc` rewire has been waiting on since cycle 5.
  - **instrument-correction:** a new corpus-wide gate, and a blocker document's
    stated mechanism corrected against the tree.

- **Refused tokens** — seven types, summing to **359**, the gate's own
  `live_hits`, unchanged from cycle 15:
  ```
  TYPE==100, BONUS:=91, PRE[A-Z]+:=61, DESC:=59, render_pcgen_desc=39, %CHOICE=8, %LIST=1
  ```
  Recorded as `deferral 1789202636606-at-35-e6-003-sweep-a86fc1`.

  Split by what can move without an operator ruling
  (`AT-35-E6-003-SWEEP_cycle5_test_region_census.py` at HEAD):

  ```
  live_hits=359  live_files=46
  hits_inside_cfg_test=351  hits_outside=8
  ```

  | where | hits | files |
  |---|---|---|
  | inside a `#[cfg(test)]` module in a live file | 351 | — |
  | reachable by code work | **8** | **4** |

  Cycle 15 recorded the reachable remainder as "8 hits in 3 files". The hit
  count was right and the **file** count was one short: the census's own
  per-file breakdown lists four — `pilot_compute/mod.rs` (4),
  `class_feature_pool_catalog.rs` (2), `pcgen_desc.rs` (1) and
  `pilot_compute/class_feature_grant_consumer.rs` (1). Corrected here rather
  than carried forward.

  The reachable 8, by the job that owns each:

  | job | hits | files | blocked? |
  |---|---|---|---|
  | the `render_pcgen_desc` / `pcgen_desc.rs` catalog rewire | 4 | `class_feature_pool_catalog.rs`, `pcgen_desc.rs`, `pilot_compute/class_feature_grant_consumer.rs` | **no longer blocked on the converter** |
  | the `PU_*_DESC_TOKEN` verbatim corpus transcriptions | 4 | `pilot_compute/mod.rs` | no — these are the book's words |

  The four `PU_*_DESC_TOKEN` constants are the book's own words, pinned
  byte-for-byte against the `.lst` files by
  `sd27_pu_class_feature_descriptions_carry_the_characters_numbers`. Cycles
  10–15 declined to reword them and this cycle declines for the same reason.
  They are **counted** in the remainder, never exempted (`NO CARVE-OUTS`).

- **Discoveries:** two, both recorded as `correction` events.
  1. The dispatch's cycle number, wrong for the sixth consecutive cycle
     (`…-06bb38`).
  2. The converter-prose blocker document's mechanism was stale by ten cycles,
     and the real defect was 241 records rather than 32, in three kinds the
     document did not name (`…-d53d4d`). The lesson is the standing one and it
     fired again: **a blocker document is prose, and prose is not a source of
     truth** — cycles 13, 14 and 15 each carried this document's diagnosis
     forward by quotation without re-deriving it.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=46 live_hits=359`, unchanged | every source file under the five live roots, code lines only (ruling B14) | `python3 scripts/pcgen_residue_gate.py --check` |
  | 241 dropping → 0, of 7,619 stating a description, over 49,296 rule files | every `data/sheet_rules/` file joined to its corpus record by path | `cargo test --locked --test sheet_rule_convert_gate a_converted_record_never_drops_the_description_its_corpus_row_states -- --nocapture` |
  | 310 package files regenerated; 239 equipment, 44 equipment_modifier, 27 spell | the working tree's own diff | `git status --porcelain data/sheet_rules \| wc -l` and `\| awk -F/ '{print $4}' \| sort \| uniq -c` |
  | 69 of the 310 are fixed but ungated | the 310, minus those the gate's path-join reaches | the reconciliation walk quoted under "What the proof does NOT cover" |
  | reachable 8 in 4 files | the 359 hits, split on `#[cfg(test)]` membership | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293` | every corpus record the sheet-rule converter reads | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `rust_lines_changed=107` | the cycle's own diff since `8cbb052583` | `python3 scripts/cycle_scope_gate.py --receipt --since 8cbb052583 --before /tmp/wi-before-at-35-e6-003-sweep.json --after docs/work-inventory.json` |
  | 15 prior receipts, so this is cycle 16 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle*_receipt.md \| wc -l` |

- **Build scope verified** — **once**, after the last figure-moving edit:

  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → **3,335 passed / 0 failed / 15 ignored**,
    `LIB_EXIT=0`
  - `cargo test --locked --no-fail-fast -j 6` → **416 targets / 8,857 passed /
    0 failed / 68 ignored / `FULL_EXIT=0`**, zero `FAILED` lines
    (`grep -c FAILED` → 0). Cycle 15 recorded 416 targets and 8,856 passed; the
    **+1** is this cycle's one new gate and **no target was added**, which is the
    arithmetic that says nothing else moved.
  - `cargo clippy --locked --tests -j 6` → **`CLIPPY_EXIT=0`, 0 warnings, 0 errors**
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293
    verdict=PASS`, identical to cycles 3–15 — **the fix adds prose to existing
    rules and converts no new record, which is why every one of those five
    figures is unmoved**
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l`
    → **0**, over a package with 310 newly-prose-bearing files
  - `python3 scripts/pcgen_residue_gate.py --check` → `live_files=46
    live_hits=359 … verdict=PASS`, never above the start tree
  - `python3 scripts/completion_atlas.py --check` → `unclassified=0 overlap=0
    done_evidence_violations=0 missing_clearing_mechanisms=0
    stale_derived_at=False citation_failures=0`, exit 0
  - `python3 scripts/token_coverage.py --check` → `non_done=0 tokened=0
    token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1
    verdict=PASS`, exit 0
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, exit 0
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, exit 0
  - `python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'`
    → `files_checked=124 violations=0`, exit 0
  - `python3 scripts/denominator_gate.py --check-provenance` →
    `files_checked=241 figures_examined=575 violations=0`
  - `scripts/verify.sh --only pi-sweep` → **RESULT: PASS** (1 stage passed)
  - **desktop crate and frontend — not run, and why**: this cycle touched no
    file under `apps/`. `workflow-instruction.md §6` step 3 runs them only when
    a cycle does; they ran at cycle 15, which is the most recent `apps/` touch,
    and were green there (575 passed / 101 frontend test files). Cycle 15's own
    finding — that epic cadence hid a red crate for ten cycles — is why this is
    recorded as a decision rather than a silence.
  - `cargo run --locked --bin corpus_literal_sweep` — **not run, and why**: no
    corpus record changed (`git status --porcelain data/corpus` empty), and
    `workflow-instruction.md §6` step 3 runs it only when they do.
  - `cargo run --locked --bin v06_work_inventory` — **not run, and why**: the
    inventory is derived from `data/corpus/`, which is byte-identical at HEAD.
    `docs/work-inventory.json` is unchanged and the receipt rows read
    `regressed=0 added=0 dropped=0` against the start-tree copy.

- **Sweep population:** N/A — no corpus record changed. `data/corpus/**` is
  byte-identical at HEAD. The corpus was **read** by the new gate, never written.

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

- **Status: `partial`.** The criterion's population is **359**, not zero, at
  HEAD. `AT-35-E6-003`'s own evidence clauses remain met; `AT-35-E6-004`'s
  closure bar, which this sweep carries, is not.

- **Notes:** the judgment call was refusing to run the sweep the dispatch asked
  for. Cycle 15 measured that another sweep pass would close nothing, and
  re-running it would have produced a green receipt describing no work. Taking
  the named blocker instead means this cycle's residue counter did not move at
  all — a worse-looking receipt for strictly more progress. The receipt says so
  in both directions rather than dressing the second as the first.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle)`; the whole
  remainder, **359 code hits in 46 files**, of which **8 hits in 4 files** are
  reachable without the `#[cfg(test)]` ruling.

  **The `render_pcgen_desc` rewire is now the unblocked job**, and it is the
  first time in four cycles that sentence can be written. The converter emits
  the record's own prose for 310 records that had none, which is the artifact
  `AT-35-E6-003_cycle5_converter-prose-blocker.md` §6 item 1 asked for. The
  remaining shape of that job, measured here rather than assumed:
  1. `src/rules_core/class_feature_pool_catalog.rs` and
     `pilot_compute/class_feature_grant_consumer.rs` read a **compiled table**
     keyed by book + `" ~ "`-split key, while the package addresses a rule by
     `"<book>:<kind>:<slug>"`. The join exists already, on the desktop side, as
     `apps/desktop/src-tauri/src/converted_prose.rs` (cycle 5, four match steps
     including the `<file>_lst_<line>` source-row step). That module — not a new
     one — is what these two call sites need, moved somewhere both crates reach.
  2. Then `pcgen_desc.rs` moves to `src/pcgen_import/`, which takes the
     `render_pcgen_desc` pattern to 0 and `live_files` from 46 toward 43.
  3. Separately and smaller: the new gate is blind to 69 of the 310 records it
     should cover, because it joins by path and those records' corpus slug
     differs. Give it the converter's own `RecordRef` join and it covers all 310.

  The operator ruling on `#[cfg(test)]` regions still stands between the gate
  and **351 of the 359**; eight cycles have now asked for it and no amount of
  dispatching will move it.
