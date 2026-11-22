# Cycle 11 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 10 handed this cycle a named remainder of **76 hits across 3 `apps/desktop` files** and
two blockers, each with a ruling attached: a display label on `VarTable` (converter side), or an
inventory ruling on 667 corpus records. This cycle asked for neither. `AGENTS.md`'s blocker
discipline has two dispositions and "wait for a ruling" is the second only when the fix is
outside the granted surface; the `VarTable` fix is **inside** this epic's file-touch set
(`workflow-instruction.md §3` names `src/rules_core/…` and the `src/pcgen_import/` tree they
moved to), so this cycle built it.

Then it measured the other blocker instead of repeating it, and the blocker was not what two
receipts said it was.

- **Commit SHA:** `81d8199a06` (the converter label, the live renderer, the desktop swap, the
  regenerated `_vars/`, the preview update and three retro events) and the self-heal commit
  that follows it (a count assertion this cycle's own change moved, one clippy suggestion, and
  the provenance-gate heal below). Cycle start `02d46f89e5`. This receipt, the `progress.md`
  entry and the `kanban.md` row ride the commit after that — a receipt cannot name the commit
  that carries it.
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
- **Files touched:** **8 tracked source paths, 5,293 regenerated variable tables, one retro
  log, plus this receipt, a parity artifact and the board rows.**
  - **`src/rules_core/sheet_rule.rs`** — `VarTable` gains `label`: *the words this variable is
    printed under when a sheet line has to name it.* `#[serde(default)]`, so an older package
    still deserializes and an empty label means the renderer falls back.
  - **`src/pcgen_import/sheet_rule/{ctx,formula,convert,mod}.rs`** — the converter carries each
    variable id's source name in its **original case** alongside the upper-cased one every
    index keys on, and `display_label` spaces it out at ingest: `_`/`-`/`.` become a space and
    a word break is inserted where a lower-case or digit character meets an upper-case one.
    `IntelligentItemEgo` → `Intelligent Item Ego`; `IntItemStatINT` → `Int Item Stat INT`; a
    run of capitals stays one word, so an all-capitals source name comes back unchanged apart
    from its separators. **Nothing is title-cased, translated, expanded or looked up in a
    table** — a label that reads oddly is the source name reading oddly, which is the honest
    outcome and the reason this is not a hand-written display map.
    `scripts/oracle_harness/var_names.json` is **byte-identical**: the upper-cased map every
    index and the oracle harness key on did not move.
  - **`src/rules_core/level_up_option_filter.rs`** — `describe_expr`'s `Expr::Var` arm printed
    the generic phrase `"a rules variable"` with the comment *"a corpus variable's id is a
    content hash, never a word — naming it would put a token on the sheet."* That was true and
    is now unnecessary: the words are written at ingest, so the arm prints the package's label
    and the sheet names the value a line moves. **This is the defect cycles 4 and 8 both
    recorded** (`AT-35-E6-003_cycle4_receipt.md` line 242: *"`catalog_description` prints `"a
    rules variable"` for a slot over an unnamed variable"*; cycle 8 line 150, Prophetic
    Visionary reading *"increases by a rules variable%"*). It is closed here, not by widening a
    live-side vocabulary list but by the package carrying the word.
  - **`apps/desktop/src-tauri/src/intelligent_item_catalog.rs`** — the swap. **28 residue hits
    → 0.**
  - **`apps/desktop/src-tauri/src/companion_catalog.rs`** — self-heal of a count assertion this
    cycle's own change moved, below.
  - **`apps/desktop/src/intelligentItemCatalog/intelligentItemCatalogRuntime.ts`** — the
    browser-preview fallback's five mechanic rows now carry the package's own variable ids and
    labels rather than the source names the backend no longer emits, and its doc comment says
    `data/sheet_rules/` and `152`.
  - **`data/sheet_rules/_vars/`** — regenerated once (`sheet_rule_convert`, then `-- --check`).
    **5,293 files changed, every one by exactly the added `label` field and nothing else**
    (proved field by field, below). **No rule file changed**; `_report.json`, `_refused.json`,
    `_tokens.json` and `_defects/` are byte-identical.
  - **`docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle10_receipt.md`**
    — one figures-table cell given its command, healing a `denominator_gate.py
    --check-provenance` violation that arrived with the cycle-start commit `02d46f89e5` (see
    "instrument-correction").
  - **`docs/retro/events/at-35-e6-003.jsonl`** (+4), `docs/retro/events/root.jsonl` (+1, an
    automated `reclaim.sh` line this checkout appended during the cycle),
    `docs/retro/events/sd31-transcribe.jsonl` (+1, the stray below),
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (`derived_at` stamp only, rewritten by its own generator).
  - **Not committed, and deliberately:** `.worktrees/ci-trait-choice` shows in an unfiltered
    `git status --porcelain` as untracked. It is a **git worktree** — another checkout of this
    repository — present before this cycle started.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.**
  ```bash
  SC="src/rules_core src/pcgen_import apps/desktop/src-tauri/src apps/desktop/src \
      docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit"
  git diff --unified=0 02d46f89e5 -- $SC ':!**/__tests__/**' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'      -> 0
  ```
- **Wired-integration audit result:** **OK_NO_TOKENS.**
  ```bash
  git diff --unified=0 02d46f89e5 -- $SC ':!**/__tests__/**' \
    | grep -ciE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'  -> 0
  ```
  The path filter keeps `':!**/*.test.*'` **off** deliberately, as cycle 10's did: this cycle
  edits a `.test.ts` file and excluding it would audit everything except part of the work.
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met.** `apps/desktop/` is at **2 files / 48 hits**, down from 3 / 76. The desktop crate
  (577 passed), the frontend suite (101/101 files) and the 19 on-screen tests are all green
  (below). Zero hits is not reached.
- **Receipt rows (mechanical):**
  ```
  CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E6-003 \
  python3 scripts/cycle_scope_gate.py --receipt --since 02d46f89e5cc1a2584ffc52b43874c19251f35ab \
    --before /tmp/wi-before-AT-35-E6-003.json --after docs/work-inventory.json
  since=02d46f89e5cc1a2584ffc52b43874c19251f35ab target_dir=/tmp/cargo-sd35-AT-35-E6-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1170 ratio=n/a builds_recorded=3 pcgen_live_files=198
  ```
  `closed=0` is correct and by design (`decisions.md §2`): Epic 6 moves no unit.
  **`pcgen_live_files` 199 → 198** — the second fall, after cycle 10's first.
- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`
  ```
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=2 hits=48
  identifier_files=7 identifier_hits=76
  live_files=198 live_hits=11462 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Cycle start was `apps/desktop files=3 hits=76`, `live_files=199 live_hits=11490`. Never above
  the baseline.
- **Oracle parity:** **run, and it did not move.** `sheet_rule_parity` over the 29-character
  fixture roster, joined to the committed PCGen BatchExporter exports at
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`:
  ```
  sheet_parity: lines compared=156 agree=154 disagree=2 unverifiable=67;
                chassis compared=382 agree=376 disagree=6 unverifiable=140;
                characters=29 exports_missing=0
  ```
  **Identical to cycles 9 and 10 on every field**, and the eight named disagreements are the
  same eight: the halfling and paladin save totals (6), the `Weapon Focus` attack line, and
  `half_elf_fighter_l1 · target:Pool:favored_class`. None is this cycle's, and that the figures
  did not move is the expected result and the point of running it — this cycle added a **word**
  to the package, never a number. Artifact:
  `AT-35-E6-003_cycle11_sheet-parity-after.json`.
- **Movement, four buckets:**
  - **closure (into DONE):** none — Epic 6 closes no unit by design.
  - **relabel:** none. `_report.json` is byte-identical: `records`, `converted`, `refused`,
    `degraded_records`, `rules` and `var_tables` all unchanged.
  - **reachability:** **one `apps/desktop` file left the ingest format; the package started
    naming its variables; and two rows stopped being offered as purchasable choices.**
    - **`intelligent_item_catalog.rs`: 28 hits → 0.** Served-vs-hidden now reads
      `SheetRule::print`; the description reads `catalog_description`; the mechanics read the
      package's `VarTable` contributions reverse-indexed by rule id, with the value through the
      same `expr_words` the sheet itself prints an unsettled term with and the condition through
      `describe_gate`. The record's own **non-rules** fields — book directory, key, name, price
      — still come from `data/corpus/`, because none of them is a rule; they are the identity of
      the row the screen is a catalog of. The join between the two is the source row **both
      sides record**, and it is exact: **171 records, 0 misses.**
    - **Every variable in the package now has a word.** 5,293 of 5,293 var tables carry a
      non-empty `label`. Two catalogs already read better for it without being touched: the
      Spitting Cobra's poison line now reads *"If Companion Advancement at least 1…"* where it
      read *"If a rules variable at least 1…"*.
    - **154 served → 152.** `Intelligent Item Purpose (Slay All)` and `Intelligent Item Purpose
      (Slay Creature Type)` are bookkeeping shadows of `Int Item / Defeat/slay all …` and `Int
      Item / Defeat/slay a particular creature type …`. Their hidden visibility is stated on a
      source row the ingested `raw_tokens` array does not carry, so the token reading served
      them as if they were purchasable choices for two years of this module's life. The package
      hides them. **This is the same defect class cycle 10 found and fixed in the converter**,
      surfacing a second time on the reader's side of the same boundary.
  - **instrument-correction:** **four**, all emitted as retro events.
    1. **The reference-library blocker was measured on a stale premise**
       (`1789129023515-at-35-e6-003-6ed5d6`). Cycles 7 and 10 both recorded that
       `reference_library_catalog.rs` is blocked because *489 `ability` corpus records are not
       inventory units*, so the package does not hold them, costing 1,150 of 9,679
       descriptions. Re-derived at HEAD: **all 9,697 records across the twelve
       reference-library kind directories join a converted rule on their own
       `source.path:line`. Zero misses.** The package holds every one. The real blocker is
       narrower, different, and nothing to do with the inventory — below.
    2. **This cycle's own deferral event carried cycle 10's token-type breakdown instead of
       re-deriving it** (`1789129055667-at-35-e6-003-49c732`). It claimed `PRE[A-Z]+:=21,
       raw_tokens=18, DESC:=6, TYPE==3`; the truth at HEAD is `PRE[A-Z]+:=16, raw_tokens=16,
       DESC:=11, render_pcgen_desc=4, BONUS:=1`. `TYPE=` is **zero** on the two remaining files
       — it was `intelligent_item_catalog.rs`'s and this cycle closed it. Caught before the
       receipt, the progress entry or the kanban row was written, by importing
       `pcgen_residue_gate.py`'s own `PATTERNS` table and running `re.findall` per file rather
       than subtracting the closed file from cycle 10's list. **The same shape as cycle 10's
       `grep -c` slip, one step worse**: that one counted the wrong thing, this one did not
       count at all.
    3. **`denominator_gate.py --check-provenance` was RED at cycle start**, one violation:
       `AT-35-E6-003_cycle10_receipt.md:231`, a figures-table row whose command cell read *"the
       same walk, computing the stable partition…"* with no command in it. It arrived with
       `02d46f89e5`, the cycle-start commit, which folded cycle 10's late figures into that
       receipt. Self-healed here by naming the command (`§8`: a cycle self-heals an audit
       violation), not by moving the figure. `files_checked=224 figures_examined=561
       violations=0`.
    4. **The stray `RETRO_ACTOR` is three stale exports in `~/.bashrc`, not an agent slip**
       (`1789132126934-at-35-e6-003-1d3534`). Cycles 9 and 10 both recorded a retro event
       landing in `docs/retro/events/sd31-transcribe.jsonl` and both diagnosed it as an
       `export` not surviving the harness's shell reset, with "set it inline per invocation" as
       the fix. This cycle set it inline on all three of its `retro.py` calls **and still
       misfiled one**, because `scripts/verify.sh` emits its own derived event and reads the
       environment (`actor_source: "env"`). The cause is `~/.bashrc` lines 135-137, three
       `RETRO_ACTOR=` assignments left behind by SD-31 wave work, the last of which is
       `sd31-transcribe`; every shell this repo opens starts with it. **Three cycles, one
       one-line fix, and no cycle could make it**: `~/.bashrc` is outside every cycle's write
       scope (`AGENTS.md` rule 4), so this is escalated rather than patched. `AGENTS.md` rule 8
       exactly — a recurrence treated as a per-run chore for three cycles rather than an
       unbuilt control. The stray line stays; the log is append-only.
- **Refused tokens:** **48 hits across 2 `apps/desktop` files** — by pattern
  `PRE[A-Z]+:`=16, `raw_tokens`=16, `DESC:`=11, `render_pcgen_desc`=4, `BONUS:`=1; by file
  `race_trait_picker.rs`=33, `reference_library_catalog.rs`=15. Both partitions sum to 48 and
  agree with the gate's own `root apps/desktop files=2 hits=48`. **5 distinct token types**,
  under `workflow-instruction.md §8`'s limit of 10. Recorded as `deferral
  1789129034843-at-35-e6-003-5e542f`, corrected by
  `1789129055667-at-35-e6-003-49c732`.

  **Two mechanisms block the two, and the reference-library one is newly named:**

  | file | hits | what blocks the swap | the number |
  |---|---:|---|---|
  | `race_trait_picker.rs` | 33 | the replacement guard has no package-side source | cycle 9's measurement, re-checked unchanged: **343 of 415** alternates already agree, **72** would lose their guard, all 72 to the same 16 records |
  | `reference_library_catalog.rs` | 15 | **the package holds every record but 4,242 of them have no prose at all**, and the module's tier-3 fallback is a render of the record's own token rows | **9,697** records join with **0** misses; by prose family **4,983** carry Desc/Benefit/Special, **472** carry only a stat-block family, **4,242** carry none |

  **On `reference_library_catalog.rs`, which two receipts called an inventory blocker.** It is
  not. The module resolves a description in three tiers, and tiers 1 and 2 (authored prose) map
  cleanly onto `catalog_description`. **Tier 3 is the problem**: for a record PCGen ships as a
  bare mechanical row — most of `skill`, `language`, `template`, `domain` — the module prints a
  summary of the record's own non-administrative token rows (`KEYSTAT: WIS`, `SIZE: M`). The
  package holds those facts, in `value`, `target`, `applies`, `tags`, `pool` and `grants`, but
  **there is no live-side renderer that turns a rule with no prose into words from its typed
  fields**. `catalog_prose` renders prose families and nothing else, so swapping today would
  serve `None` for 4,242 of 9,697 records — a description a player can read today and could
  not tomorrow. The fix is a **renderer addition** (a generic "typed rule fields as words",
  next to `catalog_description` and drawing on the same `expr_words`/`describe_gate`
  vocabulary), not a reader swap and not an inventory ruling. It is sized: 4,242 records, 12
  kind directories, one function.
- **Discoveries:** **three.**
  1. **A content hash is only opaque until the converter writes the word next to it.** The
     `Expr::Var` arm's comment — *"naming it would put a token on the sheet"* — had held for
     four cycles and was read as a property of the schema. It was a property of what the
     converter chose to emit. The general predicate worth keeping: *when a live-side renderer
     prints a generic phrase because it "cannot know" something, ask whether the producer
     knows it and simply does not say it.* Three separate cycles (4, 8, 10) recorded the
     consequence of this one omission — a catalog sentence reading *"increases by a rules
     variable%"*, a screen that could not print `Ego`, a companion line reading *"If a rules
     variable"* — without any of them naming the omission as the cause.
  2. **The same ingest defect shows up once per reader.** Cycle 10 found that the flattened
     token array loses `.COPY=` precedence and fixed it in the converter. This cycle found the
     reader-side twin: two records whose hidden visibility lives on a row the token array does
     not carry at all, which the token-reading module therefore served as purchasable options.
     Neither is visible from the other's side. The predicate: *a reader that re-implements the
     ingest parse inherits every defect of the ingest, including the ones already fixed
     upstream* — which is the argument for this whole epic, stated as a number for once (two
     rows, one screen).
  3. **A blocker copied forward twice was measured once.** Cycle 7 measured the
     reference-library gap, cycle 9 and cycle 10 restated it verbatim, and the restatement
     named an inventory ruling as the unblock. Re-deriving it took one corpus walk and showed
     the premise was gone — the package converts all 4,337 `ability` records. This is
     `AGENTS.md` rule 8's shape (*a warning copied forward is not a control*) applied to a
     blocker: **a blocker restated without being re-derived is a blocker nobody is measuring.**
- **Figures + their re-derive commands:**
  | figure | denominator | command |
  |---|---|---|
  | 171 Intelligent Item corpus records; **152** printed by the package, 19 hidden; the token array hides only **17** | every `data/corpus/*/equipment/equipmods/**.json` whose `data.key` contains `"Intelligent Item"` | a `python3` walk joining each record's `source.path:line` to `provenance.closure_rows[0]` over `data/sheet_rules/*/equipment_modifier/*.json`, counting `any(r["print"])` against `any(VISIBLE==NO)` in `data.raw_tokens` |
  | **0** join misses for those 171 | the same 171 records | the same walk; also pinned in-crate by `every_corpus_record_joins_a_converted_rule_on_its_own_source_row` (`cd apps/desktop/src-tauri && cargo test --locked -j 3 intelligent_item`) |
  | **9,697** reference-library records, **0** join misses; prose families desc **4,983** / stat-block only **472** / none **4,242** | every record under `data/corpus/*/{ability,class_generic,deity,domain,feat_generic,language,monster_generic,power,race_generic,skill,template,trait_generic}/**.json` carrying a `data.key` | a `python3 -c` source-row join of every such record against all of `data/sheet_rules/*/*/*.json`, bucketing each joined rule's `prose[].family` against Desc/Benefit/Special |
  | **5,293** var tables changed, every one by exactly the added `label` and nothing else; **0** with an empty label | every `data/sheet_rules/_vars/*.json` | `git archive 02d46f89e5 data/sheet_rules/_vars \| tar -x -C <dir>`, then a `python3` per-file compare that pops `label` from the new document and asserts equality with the old |
  | `scripts/oracle_harness/var_names.json` byte-identical | the whole file | `git status --porcelain -- scripts/oracle_harness/var_names.json` → empty |
  | no rule file changed | every path under `data/sheet_rules/` | `git show --numstat 81d8199a06 -- data/sheet_rules \| grep -v '_vars/'` → only the commit header |
  | 48 residue hits by token type and by file, both summing to 48 | the two remaining `apps/desktop` files | `python3` importing `scripts/pcgen_residue_gate.py`'s own `PATTERNS` table and running `re.findall` per file per pattern; cross-checked against `python3 scripts/pcgen_residue_gate.py --check`'s `root apps/desktop` line |
  | 343 of 415 race-trait alternates agree, 72 would lose their guard, to 16 records | every `data/corpus/*/race_trait/` alternate the picker serves | cycle 9's measurement, re-read from `AT-35-E6-003_cycle9_receipt.md`; **not re-derived this cycle** and marked as such |
  | `data/sheet_rules/` source markers: 0 | every generated rule file | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | rust_lines_changed 1170 | `*.rs` since `02d46f89e5` | the `--receipt` invocation above |
- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003`, the desktop crate
  in `…-desktop`, `sheet_rule_convert --check` in `…-check`, the release parity binary in
  `…-parity` — one directory per agent **per source tree** (`AGENTS.md`).
  ```
  cargo test --locked --no-run -j 6                  NO_RUN_EXIT=0 (every test binary linked)
  cargo test --locked --lib -j 6                     ok. 3301 passed; 0 failed; 15 ignored
  cargo test --locked --no-fail-fast -j 6            414 targets, 8812 passed, 0 failed, 68 ignored,
                                                     0 FAILED suites, FULL_EXIT=0
  cargo clippy --locked --tests -j 6 (root)          0 warnings, 0 errors
  cd apps/desktop/src-tauri && cargo clippy --locked --tests -j 3   0 warnings, 0 errors
  cd apps/desktop/src-tauri && cargo test --locked -j 3   577 passed; 0 failed; 0 ignored
  cd apps/desktop && npm run typecheck               tsc exit 0
  cd apps/desktop && npm test                        101/101 test files passed
    of which rulesAndFeaturesSection                 19 per-kind tests + 5 section tests passed
    of which IntelligentItemCatalogScreen.test.ts    PASS
  cargo run --locked --release --bin sheet_rule_convert     5,293 _vars files rewritten, report unchanged
  cargo run --locked --release --bin sheet_rule_convert -- --check   CHECK_EXIT=0
  python3 scripts/pcgen_residue_gate.py --check      verdict=PASS (live_files 199 -> 198)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l      0
  python3 scripts/completion_atlas.py --check        done_evidence_violations=0 missing_clearing_mechanisms=0
                                                     stale_derived_at=False citation_failures=0
  python3 scripts/token_coverage.py --check          non_done=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS
  python3 scripts/shape_engine_boundary.py --check   magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True
  python3 scripts/missing_engine_tables.py --check   population=0 kinds=0 citation_failures=0
  python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'      files_checked=107 violations=0
  python3 scripts/denominator_gate.py --check-provenance    files_checked=224 figures_examined=561 violations=0
  scripts/verify.sh --only pi-sweep                  RESULT: PASS
  ```
  The desktop crate and the frontend ran **here**, not at the epic wrap-up, because this cycle
  touched `apps/` (`workflow-instruction.md §6` step 3). `corpus_literal_sweep` was **not** run
  — no corpus record changed (`git status --porcelain -- data/corpus` empty at every
  checkpoint). `v06_work_inventory` was **not** re-run: its inputs are unchanged and the
  inventory is already at `DONE 49438 of 49438`.
- **Sweep population:** N/A — no corpus record changed.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
- **Status:** **partial**
- **What this proof does not cover** (`AGENTS.md` rule 7): `display_label` is proved to be a
  mechanical spacing of the source name, and that every one of the 5,293 tables got one — it is
  **not** proved that any particular label reads well to a player, and it cannot be: a source
  name written as one unbroken capitalised run comes back as one unbroken capitalised run. The
  intelligent-item swap is proved equal-or-better on its own 171 records — join exact,
  descriptions leak-free, the Ego ladder re-derived from the converted expression rather than
  the formula string — but the two rows it **stops** serving are proved hidden only from the
  package's own `print` flag, which cycle 10's `.COPY=` gate pins against the pinned tree and
  nothing else. The 415-alternate race-trait figure is quoted from cycle 9, not re-derived.
  Nothing in this cycle touched or proves anything about `race_trait_picker.rs`.
- **Open blocker for the operator (outside every cycle's write scope):** `~/.bashrc` lines
  135-137 export three SD-31-era `RETRO_ACTOR` values; the last wins, so every shell in this
  repo starts as `sd31-transcribe` and every tool that reads the environment misfiles its
  events. Three cycles have now paid for it. The fix is deleting those three lines; no cycle
  may write `~/.bashrc`.
- **Notes:** the `VarTable` schema addition was cycle 10's "AT-35-E2 territory". It is inside
  this epic's own file-touch set, so this cycle built it rather than waiting on a ruling for a
  change it was already permitted to make. `docs/retro/events/sd31-transcribe.jsonl` (+1) and
  the atlas `derived_at` stamp are folded into this cycle's board commit, as cycle 10 folded
  theirs.
- **Next-cycle scope:** **AT-35-E6-003 cycle 12**, on the 48-hit, 2-file remainder, and one of
  the two is now buildable inside this criterion: **`reference_library_catalog.rs` needs a
  generic "a rule with no prose, as words from its typed fields" renderer** beside
  `catalog_description` — 4,242 of 9,697 records need it, the other 5,455 already have prose,
  and the join is exact. `race_trait_picker.rs` still needs a package-side source for the
  replacement guard its 72 alternates stand on. Scope flags: `SCOPE_GATE: EXEMPT (Epic 6 cycle
  — closes zero units by design, decisions.md §2)`. Target: `root apps/desktop files=1 hits=33`,
  then zero.
