# Cycle 10 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 9 handed this cycle a named remainder of **97 hits across 4 `apps/desktop` files** and an
order: `intelligent_item_catalog.rs` first, then `raceCreationCoverage.test.ts`. This cycle
measured both against the package **before** writing a swap, as cycles 7 and 9 did, and found
them in different states. One swapped. The other named a blocker that is not the reader's and
not this criterion's.

And, as in cycle 9, the measurement turned up a converter defect underneath — this time in what
the package says a character sheet should **print**.

**PCGen applies a `.COPY=` record as `copied base -> the copy row's own tokens`.** The corpus
ingest flattens both into one `raw_tokens` array with the copy row's own tokens **first**, and
`closure::PinnedTree::closure` uses that array in place of the base row's. `convert_token`
assigns last-wins for every metadata head, so the flattening silently handed the **inherited**
value to every head the copy row overrides. `Intelligent Item ~ Alignment / Lawful Good.COPY=
Intelligent Item Alignment (LG)` is the shape: the copy states `VISIBLE:NO`, the row it copies
states `VISIBLE:QUALIFY`, and the converted rule came out `print: true`. **527 rules across 473
records** — PCGen's own bookkeeping shadows, rows a player never sees in PCGen's item builder
either — were in the package marked as belonging on a character sheet.

- **Commit SHA:** `2b4fc3ded7` (the converter fix, the regenerated package, the frontend test
  swap) and `dc8ec4f998` (the desktop pin self-heal and this cycle's four retro events). Cycle
  start `a38716558e`. This receipt, the `progress.md` entry and the `kanban.md` row ride the
  following commit — a receipt cannot name the commit that carries it.
- **Scope gate:**
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Run anyway, for the record — `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus reached `DONE 49438 of 49438` at `AT-35-E5-005`. Epic 6
  moves no unit; it takes the ingest format off the live side.
- **Files touched:** **3 tracked source paths, the regenerated package, one generated ledger,
  two retro logs, plus this receipt and the board rows.**
  - **`src/pcgen_import/sheet_rule/closure.rs`** — new `copy_own_tokens_last`, applied where
    `closure()` builds the own-row token list. A **stable partition, never a rewrite**: every
    shipped `(key, value)` pair the copy row itself states moves to the end of the list in its
    own order; nothing else moves, nothing is added, nothing is dropped. That is why a
    PI-screened shipped list stays exactly as screened — the defect was ordering, not absence,
    and re-reading the unscreened pinned row for content would be a different and forbidden
    change.
  - **`src/pcgen_import/sheet_rule/mod.rs`** — new per-kind gate
    `a_copy_rows_own_visible_no_reaches_the_converted_rule`. It walks the **live corpus
    directory** and reads each record's base row out of the pinned tree by the
    `source.path:line` the record itself carries — an enumeration of the **source**, independent
    of the converter's own census, which is cycle 9's lesson applied a second time (a coverage
    instrument built over the reader's input cannot see input the reader mis-ordered). Never a
    fixture (`decisions.md` §4).
  - **`apps/desktop/src/characterHub/raceCreationCoverage.test.ts`** — all four rules-bearing
    derivations now read `data/sheet_rules/`. **21 residue hits → 0.**
  - **`apps/desktop/src-tauri/src/equipment_catalog.rs`** — self-heal of a count assertion,
    below.
  - **`data/sheet_rules/`** — regenerated once (`sheet_rule_convert`, then `-- --check`).
    473 rule files changed. `_report.json`, `_refused.json`, `_tokens.json`, `_vars/` and
    `scripts/oracle_harness/var_names.json` are **byte-identical**: no record's conversion,
    refusal, census entry or variable moved.
  - **`docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`**
    (`derived_at` stamp only, rewritten by its own generator),
    `docs/retro/events/at-35-e6-003.jsonl` (+4), `docs/retro/events/sd31-transcribe.jsonl`
    (+1, the stray event below).
  - **Not committed, and deliberately:** `.worktrees/ci-trait-choice` shows in an unfiltered
    `git status --porcelain` as untracked. It is a **git worktree** — another checkout of this
    repository — present before this cycle started.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.**
  ```bash
  SC="src/rules_core src/pcgen_import apps/desktop/src-tauri/src apps/desktop/src \
      docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit"
  git diff --unified=0 a38716558e -- $SC ':!**/__tests__/**' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'      -> 0
  ```
- **Wired-integration audit result:** **OK_NO_TOKENS.**
  ```bash
  git diff --unified=0 a38716558e -- $SC ':!**/__tests__/**' \
    | grep -ciE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'  -> 0
  ```
  Note the path filter keeps `':!**/*.test.*'` **off** deliberately: this cycle's largest edit
  *is* a `.test.ts` file, and excluding it would have audited everything except the work.
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met.** `apps/desktop/` is at **3 files / 76 hits**, down from 4 / 97. The desktop crate,
  the frontend suite and the 19 on-screen tests are all green (below). Zero hits is not reached.
- **Receipt rows (mechanical):**
  ```
  CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E6-003 \
  python3 scripts/cycle_scope_gate.py --receipt --since a38716558e81ab9deea0b56dc20499acbf23ad36 \
    --before /tmp/wi-before-AT-35-E6-003.json --after docs/work-inventory.json
  since=a38716558e81ab9deea0b56dc20499acbf23ad36 target_dir=/tmp/cargo-sd35-AT-35-E6-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=195 ratio=n/a builds_recorded=2 pcgen_live_files=199
  ```
  `closed=0` is correct and by design (`decisions.md` §2): Epic 6 moves no unit.
  **`pcgen_live_files` 200 → 199** — the first fall since cycle 1.
- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`
  ```
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=3 hits=76
  identifier_files=8 identifier_hits=88
  live_files=199 live_hits=11490 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Cycle start was `apps/desktop files=4 hits=97`, `live_files=200 live_hits=11511`. Never above
  the baseline.
- **Oracle parity:** **run, and it did not move.** `sheet_rule_parity` over the 29-character
  fixture roster, joined to the committed PCGen BatchExporter exports at
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`:
  ```
  sheet_parity: lines compared=156 agree=154 disagree=2 unverifiable=67;
                chassis compared=382 agree=376 disagree=6 unverifiable=140;
                characters=29 exports_missing=0
  ```
  **Identical to cycle 9 on every field**, and the eight named disagreements are the same
  eight, unchanged: the halfling and paladin save totals (6), the `Weapon Focus` attack line,
  and `half_elf_fighter_l1 · target:Pool:favored_class · ours=1 · oracle=2` — the per-rule
  contribution compared against a pool total that cycle 9 diagnosed as a frame mismatch. None
  is this cycle's. That the figures did not move is the expected result and the point of
  running it: 527 rules stopped printing, and **not one of them was a line any of the 29
  fixture characters puts on a sheet** — which is what a bookkeeping shadow row being hidden
  should look like. Artifact: `AT-35-E6-003_cycle10_sheet-parity-after.json`.
- **Movement, four buckets:**
  - **closure (into DONE):** none — Epic 6 closes no unit by design.
  - **relabel:** none. `_report.json` is byte-identical: `records`, `converted`, `refused`,
    `degraded_records`, `rules` and `var_tables` all unchanged.
  - **reachability:** **one `apps/desktop` file left the ingest format, and one class of rule
    stopped claiming a place on the sheet.**
    - **`raceCreationCoverage.test.ts`: 21 hits → 0.** All four rules-bearing derivations now
      read the converted rule's typed fields instead of re-implementing the ingest parse in
      TypeScript: racial ability adjustments from `target.Ability` + `value.Number.Const`, the
      floating "+2 to one score" pool from `target.Pool === 'ability_bonus'` with the magnitude
      from the rule's own `label`, effective size from the `Racial Size` rule's `label`, vision
      from the rule's `Senses` stat-block prose segment. Every derived value is **identical**
      to the pre-swap one for all 18 races the package holds, checked in both forms in this
      cycle before the swap was written.
    - **`print: true` → `false` on 527 rules across 473 records.** Those rows are `.COPY=`
      bookkeeping shadows; the package no longer says they belong on a sheet.
  - **instrument-correction:** **three**, all emitted as `correction` retro events.
    1. The converter finding itself (`1789125555530-at-35-e6-003-d1a2b6`): the package's
       `print` flag was wrong for every `.COPY=` record whose copy row overrides a last-wins
       head, and nothing was watching. The new gate is the instrument that makes it impossible
       to reintroduce.
    2. This cycle's own first retro call inherited a stale `RETRO_ACTOR` and landed one event
       in `docs/retro/events/sd31-transcribe.jsonl` (`1789125545778-sd31-transcribe-4adcfc`).
       **The same slip cycle 9 recorded**, and the reason is worth stating once rather than
       apologising for twice: the harness resets the shell between tool calls, so an `export`
       in the setup block does not survive to the next command. `RETRO_ACTOR` is set
       **inline, per invocation**, not exported. The log is append-only, so the stray line
       stays and the event was re-emitted under the right actor
       (`1789125555708-at-35-e6-003-dd6ea1`).
    3. The deferral event's own first breakdown reported `grep -c` figures — which count
       **lines** — where the gate counts **regex matches**
       (`1789125591713-at-35-e6-003-cef102`). Re-derived with `re.findall`; both partitions of
       the remainder now sum to the gate's own 76. Named because it is the same shape as cycle
       9's display-cap slip, and because the first draft of the new gate had it too: its
       message reported `printing.len()`, capped at 8, as the offender total. Fixed before any
       figure left the cycle — the count and the examples are now separate variables.
- **Refused tokens:** **76 hits across 3 `apps/desktop` files** — by pattern `PRE[A-Z]+:`=27,
  `raw_tokens`=18, `DESC:`=13, `render_pcgen_desc`=8, `raw_bonus_chains`=6, `TYPE=`=3,
  `BONUS:`=1; by file `race_trait_picker.rs`=33, `intelligent_item_catalog.rs`=28,
  `reference_library_catalog.rs`=15. Both partitions sum to 76 and agree with the gate's own
  `root apps/desktop files=3 hits=76`. **7 distinct token types**, under
  `workflow-instruction.md §8`'s limit of 10. Recorded as `deferral
  1789125570846-at-35-e6-003-8797a7`.

  **Two mechanisms block the three, and neither is the reader's:**

  | file | hits | what blocks the swap | the number |
  |---|---:|---|---|
  | `race_trait_picker.rs` | 33 | 178 ARG `race_trait` corpus records are not inventory units | cycle 9's measurement, unchanged: **343 of 415** alternates already agree, **72** would lose their guard, all 72 to the same 16 records |
  | `intelligent_item_catalog.rs` | 28 | **the converted package does not NAME its variables** | the screen's whole content is "which named variable does this component move" — **175** Ego contributions, 34 each for the three ability vars, all keyed by an opaque `VarId` with no label |
  | `reference_library_catalog.rs` | 15 | 489 `ability` corpus records are not inventory units | cycle 7's measurement, unchanged: **1,150 of 9,679** descriptions |

  **On `intelligent_item_catalog.rs`, which cycle 9 called unblocked.** Cycle 9 was right that
  the mechanics are now *in* the package — the Ego variable went from 0 contributions to 175.
  It is the **naming** that is missing. `VarTable` is keyed by `var_id(name)`, a one-way hash
  of the upper-cased source name, and `VarProvenance` carries no label; the source-name map is
  written to `scripts/oracle_harness/var_names.json`, **tool side, by design**
  (`sheet_rule/mod.rs`'s own module doc says so). A screen that prints `Ego +2` cannot get the
  word `Ego` out of the package, and minting the id from the PCGen variable name on the live
  side is exactly what `decisions.md` §11 forbids. **The fix belongs on the converter side** —
  a codex-neutral display label on `VarTable` — and that is a schema addition in AT-35-E2's
  territory, not a reader swap. Measured, not assumed: the 171 corpus records join the package
  on their own source row with **zero** misses, and the hidden-row filter those readers need is
  exactly what this cycle's `print` fix supplies.
- **Discoveries:** **three.**
  1. **The ingest's flattened token array loses `.COPY=` precedence, and every last-wins head
     is exposed to it.** The general shape: an ingest that merges two source rows into one
     ordered array must preserve the order the source format applies them in, or every
     assignment-semantics reader downstream silently takes the wrong one. `VISIBLE` is where it
     showed, because `print` is the only such head whose wrong value is visible on a sheet. The
     predicate worth keeping: *does the shipped token list preserve the order its source rows
     were applied in?* — the companion to cycle 9's *does it account for every clause?*
  2. **The blast radius was measurable in advance, and it mattered that it was.** Before the
     Rust change, a corpus-wide pass computed the same partition in Python and asked which
     heads' last-wins value would move: **`VISIBLE` (473 records), `EQMOD` (65), `ALTEQMOD`
     (2), `COST` (2)** and nothing else. The converter **reads and drops** `EQMOD`, `ALTEQMOD`
     and `COST` (`convert.rs` line 653's metadata arm), so the partition was provably
     output-equivalent except for `print`. The regenerated package then confirmed it field by
     field: 473 files, 527 rules, **one field**. A change this wide is only safe because its
     width was a number first.
  3. **A separate cargo workspace is a gate that does not run.** `description_coverage_is_
     pinned_per_book` in the desktop crate was red at `UPSI` 403 → 404, and it is **not this
     cycle's**: the regenerated package differs from `HEAD~1` in exactly one field, and
     `catalog_description` does not read `print`. Against cycle 9's parent the Ultimate
     Psionics subtree gained 57 rules — all `#bonusN` siblings — and gained and lost **zero**
     prose. `apps/desktop/src-tauri` is its own workspace, so the root `cargo test` never
     builds it, and `workflow-instruction.md` §6 runs it only for a cycle that touches
     `apps/`. Cycles 8 and 9 regenerated `data/sheet_rules/` and did not. Three cycles of drift
     surfaced at once because this cycle happened to touch `apps/`. Self-healed, both figures
     re-derived from the built catalog rather than adjusted by delta.
- **Figures + their re-derive commands:**
  | figure | denominator | command |
  |---|---|---|
  | 2,418 corpus records whose base row is a `.COPY=` row; 2,110 of them ship tokens | the 51,463 `data/corpus/` records carrying a `source.path` + `source.line` | a Python walk of `data/corpus/` reading each record's own base row out of `$PCGEN_CORPUS_ROOT`, testing `'.COPY=' in row.split('\t')[0]` |
  | heads whose last-wins value the partition moves: `VISIBLE` 473, `EQMOD` 65, `ALTEQMOD` 2, `COST` 2 — and no others | the 2,110 copy-base records that ship tokens | the same walk, computing the stable partition and diffing the last-wins map before and after |
  | `EQMOD` / `ALTEQMOD` / `COST` are read and dropped by the converter | the converter's own token match | `sed -n '649,658p' src/pcgen_import/sheet_rule/convert.rs` — the metadata arm |
  | package diff: 473 files, **527 rules**, exactly one field (`print`), all `true` → `false` | every changed file under `data/sheet_rules/` | `python3` over `git status --porcelain -- data/sheet_rules`, comparing each rule of `git show HEAD:<path>` to the working copy field by field |
  | gate RED **523** offending rules across **644** records; 648 hidden copy rows in the corpus; 4 not held by the package under their own id | every `data/corpus/` record whose base row is a `.COPY=` row stating `VISIBLE:NO` | `cargo test --locked --lib -j 6 a_copy_rows_own_visible_no_reaches_the_converted_rule` at `a38716558e` |
  | 18 of 30 chassis races derive every rules-bearing field from the package; the other 12 are ARG's | the 30 `data/corpus/*/race/` chassis records | `cd apps/desktop && npx tsx src/characterHub/raceCreationCoverage.test.ts` (the file asserts both, and names the 12) |
  | ability adjustments, floating pool, size and vision identical before and after the swap for all 18 | the 18 races the package holds `Racial Default` rules for | a Python prototype of both derivations run side by side over `data/corpus/*/race_trait/` and `data/sheet_rules/*/race_trait/` before the TypeScript was written |
  | Intelligent Item: 171 corpus records, 154 visible, 17 hidden; **0** join misses on the source row | every `data/corpus/*/equipment/equipmods/*.json` whose `data.key` contains `"Intelligent Item"` | a Python join of each record's `source.path:line` against `provenance.closure_rows[0]` over `data/sheet_rules/*/equipment_modifier/` |
  | Ego variable: 175 contributions, all keyed by `v027321c791a2c8bd`, no label anywhere in the package | `data/sheet_rules/_vars/v027321c791a2c8bd.json` | `python3 -c "import json;d=json.load(open('data/sheet_rules/_vars/v027321c791a2c8bd.json'));print(len(d['contributions']), sorted(d.keys()))"` |
  | 76 residue hits by token type and by file, both summing to 76 | the three `apps/desktop` files | `python3 scripts/pcgen_residue_gate.py --check --list-files`, and a per-file `re.findall` over the gate's own pattern table |
  | desktop `UPSI` 403 → 404 and the total 5389 → 5390, and no other book moved | every entry of `build_equipment_catalog()` | `cd apps/desktop/src-tauri && cargo test --locked -j 4` — each assertion's own `left` |
  | Ultimate Psionics gained 57 rules and zero prose since `aaeae79cab` | every `data/sheet_rules/ultimate_psionics/*/*.json` rule | `git archive aaeae79cab data/sheet_rules/ultimate_psionics \| tar -x -C <dir>`, then a Python id-set and prose-set diff |
  | `data/sheet_rules/` source markers: 0 | every generated rule file | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | rust_lines_changed 195 | `*.rs` since `a38716558e` | the `--receipt` invocation above |
- **Build scope verified:** run at `dc8ec4f998`. Root workspace in
  `/tmp/cargo-sd35-AT-35-E6-003`, the desktop crate in `…-desktop`, `sheet_rule_convert --check`
  in `…-check`, the release parity binary in `…-parity` — one directory per agent **per source
  tree** (`AGENTS.md`).
  ```
  cargo test --locked --no-run -j 6                  NO_RUN_EXIT=0 (every test binary linked)
  cargo test --locked --lib -j 4                     ok. 3301 passed; 0 failed; 15 ignored
  cargo test --locked --no-fail-fast -j 6            414 targets, 8812 passed, 0 failed, 68 ignored,
                                                     0 FAILED suites, FULL_EXIT=0
  cargo clippy --locked --tests -j 4 (root)          0 warnings, 0 errors
  cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 3   0 warnings, 0 errors
  cd apps/desktop/src-tauri && cargo test --locked -j 4   578 passed; 0 failed; 0 ignored
  cd apps/desktop && npm run typecheck               tsc exit 0
  cd apps/desktop && npm test                        101/101 test files passed
    of which rulesAndFeaturesSection                 19 per-kind tests + 5 section tests passed
  cargo run --locked --bin sheet_rule_convert        473 rule files rewritten, report unchanged
  cargo run --locked --bin sheet_rule_convert -- --check   CHECK_EXIT=0
  python3 scripts/pcgen_residue_gate.py --check      verdict=PASS (live_files 200 -> 199)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l      0
  python3 scripts/completion_atlas.py --check        population=49438 DONE 49438 unclassified=0 overlap=0
                                                     done_evidence_violations=0 missing_clearing_mechanisms=0 citation_failures=0
  python3 scripts/token_coverage.py --check          non_done=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS
  python3 scripts/shape_engine_boundary.py --check   magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True
  python3 scripts/missing_engine_tables.py --check   population=0 kinds=0 citation_failures=0
  python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'      files_checked=106 violations=0
  python3 scripts/denominator_gate.py --check-provenance    files_checked=223 figures_examined=559 violations=0
  scripts/verify.sh --only pi-sweep                  RESULT: PASS
  ```
  The desktop crate and the frontend ran **here**, not at the epic wrap-up, because this cycle
  touched `apps/` (`workflow-instruction.md` §6 step 3). `corpus_literal_sweep` was **not** run
  — no corpus record changed (`git status --porcelain -- data/corpus` empty at every
  checkpoint). `v06_work_inventory` was **not** re-run: its inputs are unchanged and the
  inventory is already at `DONE 49438 of 49438`.
- **Sweep population:** N/A — no corpus record changed.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
- **Status:** **partial**
- **What this proof does not cover** (`AGENTS.md` rule 7): the new gate proves that a `.COPY=`
  row's own `VISIBLE:NO` reaches `print`. It proves nothing about the **other** heads the same
  ordering defect could have corrupted — it was measured that only `EQMOD`, `ALTEQMOD` and
  `COST` move and that the converter drops all three, but that measurement is over **today's**
  corpus and today's metadata arm: a future token head moved out of that arm, or a future
  ingest field, reopens the question and the gate would not notice. The swapped race
  derivations are proven **equal to the old ones for 18 races**; the 12 ARG races now have no
  magnitude proof at all in this file, only a classification check, and that is the honest cost
  of the inventory blocker rather than a coverage gain. Nothing in this cycle touched or proves
  anything about the three remaining `apps/desktop` files.
- **Notes:** the audit greps deliberately drop `':!**/*.test.*'`. This cycle's largest single
  edit is a `.test.ts` file; auditing everything except the work would have been a gate that
  passes by construction.
- **Next-cycle scope:** **AT-35-E6-003 cycle 11**, on the 76-hit, 3-file remainder. Neither
  blocker is buildable inside this criterion, so the cycle needs one of two rulings first:
  (1) **a display label on `VarTable`** (converter side, AT-35-E2 territory) — after which
  `intelligent_item_catalog.rs` is a straight swap: its population joins the package with zero
  misses, its hidden-row filter is now `!rule.print`, its descriptions come from
  `converted_prose`, and its mechanics from the var tables' `contributions` reverse-indexed by
  `rule_id`; (2) **the inventory ruling** on the 667 corpus records that are not inventory
  units (178 ARG `race_trait` + 489 `ability`), which unblocks `race_trait_picker.rs` and
  `reference_library_catalog.rs` together and moves the bundle denominator 49,438. Scope flags:
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Target: `root apps/desktop files=1 hits=28` or `files=2 hits=48`, depending which ruling
  lands first, then zero.
