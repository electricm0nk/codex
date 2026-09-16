# Cycle 12 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 11 handed this cycle a 48-hit, 2-file remainder and one buildable next step:
`reference_library_catalog.rs` "needs a generic *a rule with no prose, as words from its typed
fields* renderer", sized at 4,242 of 9,697 records. This cycle built it, swapped the module, and
found the sizing was wrong by an order of magnitude — in the closable direction.

- **Commit SHA:** `798bf8ebdad4ba25124bbfde40dd6e858bfdab40` (the renderer, the converter scrub
  widening, the desktop swap, the recorded findings, the regenerated pair and six retro events).
  Cycle start `8d5d3fc53e6c41c5242caa876d4da365fdc7fd88`. This receipt, the `progress.md` entry
  and the `kanban.md` row ride the commit after it — a receipt cannot name the commit that
  carries it.
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
- **Files touched:** **10 tracked source paths**, plus this receipt, its census script and the board rows.
  - **`src/rules_core/sheet_rule_catalog.rs`** — the cycle's substance.
    [`catalog_field_summary`] renders a rule that has **no prose at all** from its typed fields,
    in one fixed order: the value, the second numbers beside it, the sheet total it feeds and
    its stacking type, its tags, the condition it applies under, the choice it offers, what
    holding it does to the fact set, and who hands it out. Every clause goes through
    `expr_words`/`describe_gate` — the vocabulary a prerequisite line already prints with, so
    there is no second describer to drift. It then reads **two edges backwards**, and those are
    where most of the population actually lives:
    - `SheetRulePackage::granted_from` — what this rule hands out. A variant row whose entire
      content is *"this is the base creature with the Fiendish Creature template on it"* carries
      that fact on the **template's** `granted_by`, never in its own fields.
    - the labelled variable tables — a record whose entire content is a bonus to, or the
      declaration of, a named rules variable. Consulted **only** when the rule's own fields said
      nothing, so the linear scan runs for a few hundred rules rather than every rule a catalog
      loads.

    [`catalog_description_or_fields`] tries authored prose, then the stat block, then those
    fields, and returns which tier answered, so a caller can still tell a rendered stat line
    from the record's own sentence.
  - **`src/rules_core/sheet_rule.rs`** — `SheetRulePackage::granted_from`, a public read of the
    `Granter::Rule` reverse index `finish()` already builds and the evaluator already uses.
  - **`src/rules_core/level_up_option_filter.rs`** — `describe_prof`, `ability_word`,
    `save_word` and `label_of` made public, each with the single-vocabulary reason written on it.
    No behaviour change.
  - **`src/pcgen_import/sheet_rule/prose.rs`** — `group_is_editorial_marker` now also matches an
    **annotation head**: a bracketed group opening with the word `note` and a colon. The head is
    required, so a parenthetical the rule itself wrote survives. See instrument-correction 2.
  - **`apps/desktop/src-tauri/src/reference_library_catalog.rs`** — the swap. **15 residue hits
    → 0.** Identity (`key`, `name`) still comes from the corpus record, because neither is a
    rule; only the description moved. Join is on the source row **both sides record**, with a
    second index for the row a *different* rule's closure absorbed.
  - **`apps/desktop/src-tauri/src/companion_pool_catalog.rs`** — its `.COPY=` branch was the one
    remaining importer of the deleted token-dump helper; it now reads
    `catalog_description_or_fields` for the same rows.
  - **`apps/desktop/src-tauri/src/reach_gate.rs`** — `BARE_RECORD_FINDINGS` **6 families / 22
    keys → 40 / 395**, and `bare_records_are_exactly_the_recorded_findings` restructured to
    judge every family before asserting (instrument-correction 3).
  - **`data/sheet_rules/`** — regenerated once. **Exactly 3 files changed**: two rule files whose
    prose carried the annotation, and `_defects/editorial-marker-in-prose.json`. `_report.json`,
    `_refused.json` and `_tokens.json` are **byte-identical**.
  - **`docs/retro/events/at-35-e6-003.jsonl`** (+5, three corrections and two deferrals),
    `docs/retro/events/sd31-transcribe.jsonl` (+1, a `verify.sh`-derived event misfiled by the
    `~/.bashrc` hazard cycle 11 escalated — still unfixable from inside a cycle),
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (`derived_at` stamp only, rewritten by its own generator).
  - **Not committed, and deliberately:** `.worktrees/ci-trait-choice` shows in an unfiltered
    `git status --porcelain` as untracked. It is a **git worktree** — another checkout of this
    repository — present before this cycle started.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.**
  ```bash
  SC="src/rules_core src/pcgen_import apps/desktop/src-tauri/src apps/desktop/src \
      docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit"
  git diff --unified=0 8d5d3fc53e -- $SC ':!**/__tests__/**' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'      -> 0
  ```
- **Wired-integration audit result:** **OK_NO_TOKENS.**
  ```bash
  git diff --unified=0 8d5d3fc53e -- $SC ':!**/__tests__/**' \
    | grep -ciE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'  -> 0
  ```
  `':!**/*.test.*'` is left **off**, as cycles 10 and 11 did: excluding test files would audit
  everything except part of the work.
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met.** `apps/desktop/` is at **1 file / 33 hits**, down from 2 / 48. The desktop crate
  (575 passed), the frontend suite (101/101 files) and the 19 on-screen tests are all green
  (below). `render_pcgen_desc` no longer has a caller in `apps/desktop/` at all. Zero hits is
  not reached: one file remains.
- **Receipt rows (mechanical):**
  ```
  CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E6-003 \
  python3 scripts/cycle_scope_gate.py --receipt --since 8d5d3fc53e6c41c5242caa876d4da365fdc7fd88 \
    --before /tmp/wi-before-AT-35-E6-003.json --after docs/work-inventory.json
  since=8d5d3fc53e6c41c5242caa876d4da365fdc7fd88 target_dir=/tmp/cargo-sd35-AT-35-E6-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1978 ratio=n/a builds_recorded=3 pcgen_live_files=197
  ```
  `closed=0` is correct and by design (`decisions.md §2`): Epic 6 moves no unit.
  **`pcgen_live_files` 198 → 197** — the third consecutive fall.
- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`
  ```
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=33
  identifier_files=6 identifier_hits=64
  live_files=197 live_hits=11447 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Cycle start was `apps/desktop files=2 hits=48`, `live_files=198 live_hits=11462`. Never above
  the baseline. **The gate caught this cycle raising it, once**: the first draft of the two new
  corpus-wide gates listed the ingest format's literal token heads in a `const` array, which is
  itself a live-side occurrence of the ingest format (`live_files` 198 → 199). Both gates were
  rewritten to detect the vocabulary by **shape** — an all-capitals run of four or more letters
  followed by `:` or `=`, or a `%` followed by a digit or a capital — which is both compliant and
  a wider check than the list was.
- **Oracle parity:** **N/A this cycle, and the reason is exact.** No `Number` mapping was added
  and no live path that feeds a sheet total was touched: the converter change removes an
  editorial aside from two records' **prose**, and the live change renders words for screens that
  have no character in hand. `data/sheet_rules/` changed by exactly 3 files, none of them
  carrying a `value`, `target` or `also` delta (`git show --numstat 798bf8ebda -- data/sheet_rules`,
  and `_report.json` byte-identical). The last parity run at `AT-35-E6-003_cycle11_sheet-parity-after.json`
  therefore still describes HEAD; re-running it would have re-derived the same 8 named
  disagreements at the cost of the cycle's remaining build budget.
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
- **Movement, four buckets:**
  - **closure (into DONE):** none — Epic 6 closes no unit by design.
  - **relabel:** none. `_report.json` is byte-identical: `records`, `converted`, `refused`,
    `degraded_records`, `rules` and `var_tables` all unchanged.
  - **reachability:** **one `apps/desktop` file left the ingest format; six records gained a
    description they were recorded as unable to have; 395 lost a token dump that was never a
    description.**
    - **`reference_library_catalog.rs`: 15 hits → 0**, over a population of **9,697** records in
      12 kind directories, **0** join misses.
    - **The tier census, by converted rule** (9,939 rules of those kinds):
      **prose 3,785 / stat block 397 / typed fields 5,342 / identity only 415.**
    - **Six records went from *recorded as impossible* to described.** SD-32 row 19 cycle 4 put
      `Hydra (Cryohydra)`, `Hydra (Pyrohydra)` and the four Iron Cobra material variants in
      `BARE_RECORD_FINDINGS` with the finding that *"nothing beyond the bare key/name exists
      anywhere in the corpus record"*. Read from the package, all six describe themselves. The
      entry is **deleted**, not relaxed.
    - **395 records across 40 families are now identity-only and recorded key by key** (was 22
      across 6). Below, with the mechanism and the remedy.
  - **instrument-correction:** **three**, all emitted as retro events.
    1. **Cycle 11 sized the reference-library gap at 4,242 records; it is 403**
       (`1789138505702-at-35-e6-003-5f9046`). Cycle 11 bucketed each joined rule's
       `prose[].family` and called a rule with no `Desc`/`Benefit`/`Special` family undescribable.
       That measured one of five places a converted rule keeps its content. The other four — the
       stat-block families, the typed fields, the grant edge read backwards, and the labelled
       variable tables — describe **3,839** of the 4,242. The predicate worth keeping: *a gap
       measured against one field of a schema is a gap in the measurement.*
    2. **The converter's editorial scrub was narrower than its own doc comment claimed**
       (`1789138505827-at-35-e6-003-ea70be`). `strip_editorial_not_implemented_markers` removed
       upstream's not-implemented admission and nothing else, so an annotation head addressed to
       the source data's maintainers survived into two records' converted prose and reached a
       catalog screen inside the rule's own words. Found by the new corpus-wide gate on its
       first run, not by inspection. Widening `group_is_editorial_marker` and regenerating
       changed **exactly those two rule files** and the defect ledger.
    3. **`bare_records_are_exactly_the_recorded_findings` asserted inside its per-family loop**
       (`1789138505960-at-35-e6-003-555c26`), so one run reported one family and hid the other
       35. This cycle paid for it three times at roughly two minutes a build — `acg/abilities`,
       then `acg/templates`, then `apg/templates` — before restructuring the test to judge every
       family and assert once. The next run listed all 36. `AGENTS.md` rule 8 in its cheapest
       possible form: the mechanism was one loop away the whole time.
- **Refused tokens:** **33 hits in 1 `apps/desktop` file** — by pattern `PRE[A-Z]+:`=15,
  `DESC:`=9, `raw_tokens`=8, `BONUS:`=1; by file `race_trait_picker.rs`=33. Both partitions sum
  to 33 and agree with the gate's own `root apps/desktop files=1 hits=33`. **4 distinct token
  types**, under `workflow-instruction.md §8`'s limit of 10. Recorded as `deferral
  1789138521208-at-35-e6-003-3e9128`.

  **One mechanism blocks the one file, unchanged from cycle 9's measurement and re-stated here
  with its own denominator rather than copied:** `race_trait_picker.rs` reads the ingest format
  for the **replacement guard** — a variable-flag qualifier on the grant row and a negated
  bracket group on the alternate — and the converted package carries no equivalent. Cycle 9
  measured **343 of 415** alternates already agreeing and **72** losing their guard, all 72 to
  the same **16** records. **That figure is quoted from `AT-35-E6-003_cycle9_receipt.md` and was
  not re-derived this cycle**, and the honest consequence of instrument-correction 1 is that it
  may be as wrong as cycle 11's was: nothing has asked whether the guard's two halves are
  reconstructible from `granted_by`, `grants` and the variable tables the way the reference
  library's content turned out to be. Cycle 13's first act should be that question, not the swap.
- **The 395, and why they are a number rather than an exemption** (`decisions.md §27b`).
  Every one is a record whose converted rule states **nothing beyond its identity**: no prose, no
  stat-block line, no typed field, no grant edge in either direction, no labelled variable
  contribution. What the screen printed for them before this cycle was the ingest format's own
  token rows with the heads still attached — a visibility flag saying the row is not shown, a
  creature subtype, a size letter, a leg count, a starting-equipment kit reference. **216 of the
  403 identity-only corpus records are rows the source itself marks not-visible.** Removing that
  is the point of the criterion, not a regression against it; the records keep their surface and
  their keys, and the loss is pinned key by key in `BARE_RECORD_FINDINGS` where a later cycle
  that closes any of them will be *forced* to delete the entry.

  **Remedy, and it is converter-side by construction** (`workflow-instruction.md §8`: a live-side
  read is never the fix): carry the metadata heads that are genuinely facts about a record —
  size, legs, hands, creature subtype, subrace, alignment, region, favored weapon, starting kit —
  as stat-block lines on the record's own rule. It rewrites prose corpus-wide and needs its own
  build and its own parity run, which is why it is not in this cycle. Four of the ten ACG
  template rows state a visibility flag and nothing else and are a hard impossibility of source
  data. Recorded as `deferral 1789138521337-at-35-e6-003-f852a7`.
- **Discoveries:** **two.**
  1. **A converted record keeps its content in five places, and four of them are edges.** The
     fields on the rule are the obvious one. The other four — the stat-block prose families, what
     the rule **hands out** (`granted_from`, read backwards), what **hands it out**
     (`granted_by`), and the labelled variable tables that name it — hold the entire content of
     **5,342** of the 9,939 reference-library rules. Cycle 11 measured one place and sized the
     gap at 4,242; the true figure is 403. The general predicate: *when a record looks empty,
     ask what points at it before concluding the package does not hold it.* This is the same
     shape as cycle 11's own discovery 1 (a content hash is only opaque until the converter
     writes the word next to it) one level up: there, the producer knew and did not say; here,
     the package says it on the other end of an edge nobody read.
  2. **A gate that names what it forbids becomes an instance of it.** The first draft of both new
     corpus-wide gates listed the ingest format's literal token heads so a failure could name
     what leaked — and `pcgen_residue_gate.py` counted those literals, correctly, as two more
     live-side files reading the ingest format. Detecting the vocabulary by **shape** instead is
     both compliant and strictly wider: it catches a token head this bundle has never seen. The
     predicate: *a check written as a list of the forbidden thing is written in the forbidden
     thing; write the shape.*
- **Figures + their re-derive commands:**
  | figure | denominator | command |
  |---|---|---|
  | tier census **prose 3,785 / stat block 397 / fields 5,342 / identity only 415** | every converted rule whose `provenance.kind` is one of the twelve reference-library kinds (9,939) | `CARGO_TARGET_DIR=… cargo test --locked --lib no_reference_library_description_carries_ingest_format_vocabulary -- --nocapture` — the test prints the four counts and fails on any ingest vocabulary in any rendered description |
  | **9,697** reference-library corpus records, **0** join misses | every record under `data/corpus/*/{ability,class_generic,deity,domain,feat_generic,language,monster_generic,power,race_generic,skill,template,trait_generic}/**.json` carrying a `data.key` | in-crate: `cd apps/desktop/src-tauri && cargo test --locked -j 3 every_record_joins_a_converted_rule_on_its_own_source_row` (prints `records=… misses=…`) |
  | **403** corpus records / **418** rules identity-only, of which **216** are rows the source itself marks not-visible | the same 9,697 (the script prints its own `reference_library_records=`) | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle12_identity_only_census.py` (committed with this receipt; applies the same five content tests `catalog_field_summary` applies) |
  | `BARE_RECORD_FINDINGS` **6 families / 22 keys → 40 / 395** | the whole table | a bracket-matching `python3` parser over the `const` in `reach_gate.rs`, run against `git show 8d5d3fc53e:apps/desktop/src-tauri/src/reach_gate.rs` and against HEAD (a regex over the same text mis-counts: it merged rows and reported 4/28 for the before state, which is why the parser matches brackets) |
  | exactly **3** files changed by the regeneration, `_report.json`/`_refused.json`/`_tokens.json` byte-identical | every path under `data/sheet_rules/` | `git status --porcelain -- data/sheet_rules` after `cargo run --locked --release --bin sheet_rule_convert`, then `git diff --stat -- data/sheet_rules/_report.json data/sheet_rules/_refused.json data/sheet_rules/_tokens.json` → empty |
  | **2** records carried the annotation head into their prose | every generated rule file | `grep -rho "\[NOTE:[^]]*\]" data/sheet_rules/ \| sort \| uniq -c` → two distinct, one each |
  | 33 residue hits by token type and by file, both summing to 33 | the one remaining `apps/desktop` file | `python3` importing `scripts/pcgen_residue_gate.py`'s own `PATTERNS` table and running `re.findall` per file per pattern; cross-checked against the gate's own `root apps/desktop` line |
  | 343 of 415 race-trait alternates agree, 72 would lose their guard, to 16 records | every `data/corpus/*/race_trait/` alternate the picker serves | cycle 9's measurement, re-read from `AT-35-E6-003_cycle9_receipt.md`; **not re-derived this cycle** and marked as such, twice |
  | `data/sheet_rules/` source markers: 0 | every generated rule file | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | rust_lines_changed 1978 | `*.rs` since `8d5d3fc53e` | the `--receipt` invocation above |
- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003`, the desktop crate in
  `…-desktop`, the converter and its `--check` in `…-check` — one directory per agent **per source
  tree** (`AGENTS.md`).
  ```
  cargo test --locked --no-run -j 6                  NO_RUN_EXIT=0 (every test binary linked)
  cargo test --locked --lib -j 6                     ok. 3309 passed; 0 failed; 15 ignored
  cargo test --locked --no-fail-fast -j 6            413 targets, 8820 passed, 0 failed, 68 ignored,
                                                     0 FAILED suites, FULL_EXIT=0
  cargo clippy --locked --tests -j 6 (root)          0 warnings, 0 errors
  cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 3   0 warnings, 0 errors
  cd apps/desktop/src-tauri && cargo test --locked -j 3   575 passed; 0 failed; 0 ignored
  cd apps/desktop && npm run typecheck               tsc exit 0
  cd apps/desktop && npm test                        101/101 test files passed
    of which rulesAndFeaturesSection                 19 per-kind tests + 5 section tests passed
  cargo run --locked --release --bin sheet_rule_convert     3 files rewritten, report unchanged
  cargo run --locked --release --bin sheet_rule_convert -- --check   CHECK_EXIT=0
  python3 scripts/pcgen_residue_gate.py --check      verdict=PASS (live_files 198 -> 197)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l      0
  python3 scripts/completion_atlas.py --check        done_evidence_violations=0 missing_clearing_mechanisms=0
                                                     stale_derived_at=False citation_failures=0
  python3 scripts/token_coverage.py --check          non_done=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS
  python3 scripts/shape_engine_boundary.py --check   magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True
  python3 scripts/missing_engine_tables.py --check   population=0 kinds=0 citation_failures=0
  python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'      files_checked=108 violations=0
  python3 scripts/denominator_gate.py --check-provenance    files_checked=225 figures_examined=563 violations=0
  scripts/verify.sh --only pi-sweep                  RESULT: PASS
  ```
  The desktop crate and the frontend ran **here**, not at the epic wrap-up, because this cycle
  touched `apps/` (`workflow-instruction.md §6` step 3). The final `cargo clippy` fix (three
  collapsible `if`s, all in this cycle's own new code) landed after the full suite; the two lib
  test modules it touched were re-run green (`sheet_rule_catalog` 18/18, `prose` 10/10) and the
  change is expression-level with no behaviour in it. `corpus_literal_sweep` was **not** run — no
  corpus record changed (`git status --porcelain -- data/corpus` empty at every checkpoint).
  `v06_work_inventory` was **not** re-run: its inputs are unchanged and the inventory is already
  at `DONE 49438 of 49438`.
- **Sweep population:** N/A — no corpus record changed.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
- **Status:** **partial**
- **What this proof does not cover** (`AGENTS.md` rule 7): the swap is proved equal-or-better on
  the join (9,697 records, 0 misses), on the absence of ingest vocabulary from every rendered
  description (by shape, corpus-wide, both in-crate and in the lib), and on the exact set of
  records that lost a description (395, pinned key by key, in both directions). It is **not**
  proved that any particular field summary reads well to a player — `Tags: Dexterity, ACHECK,
  Base` is the record's own words and they are not a sentence. It is not proved that the 395 are
  the *smallest* identity-only set: a converter that carried the metadata heads would shrink it,
  and nothing here bounds by how much beyond the sizing above. The absorbing-closure fallback is
  proved only by the tests that exercise it; a row absorbed by a rule that is **not** the same
  record would describe a record with something else's words, and the only thing preventing that
  is that the fallback is keyed on the record's own source row. Nothing in this cycle touched or
  proves anything about `race_trait_picker.rs`, and its 415/343/72/16 figures are cycle 9's.
- **Open blocker for the operator (outside every cycle's write scope, third cycle standing):**
  `~/.bashrc` lines 135-137 export three SD-31-era `RETRO_ACTOR` values; the last wins, so every
  shell in this repo starts as `sd31-transcribe` and every tool that reads the environment
  misfiles its events. This cycle set `RETRO_ACTOR` inline on all five of its `retro.py` calls
  and `scripts/verify.sh` still misfiled its derived event, exactly as cycles 9, 10 and 11
  recorded. The fix is deleting those three lines; no cycle may write `~/.bashrc`.
- **Notes:** the converter change is two lines of predicate and was made because a gate this
  cycle wrote found a real leak, not as scope expansion — `decisions.md §11` puts the fix for a
  live-side leak on the converter side by construction.
- **Next-cycle scope:** **AT-35-E6-003 cycle 13**, on the 33-hit, 1-file remainder:
  `race_trait_picker.rs`. Its first act should be to ask whether the replacement guard's two
  halves are reconstructible from `granted_by`, `grants` and the labelled variable tables, the
  way this cycle's content turned out to be — cycle 9's 72-alternate figure has been quoted
  through four receipts without re-derivation, which is the exact shape cycle 11 named and this
  cycle disproved. Scope flags: `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design,
  decisions.md §2)`. Target: `root apps/desktop files=0 hits=0`.
