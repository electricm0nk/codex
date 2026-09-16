# Cycle 9 — Epic 6 (PCGen exit) / AT-35-E6-003

Cycle 8 handed this cycle a named remainder of **97 hits across 4 `apps/desktop` files** and one
plan: build the facts renderer cycle 7 measured, then take the four structural readers with it.
This cycle measured each of the four **before** writing a swap, exactly as cycle 7 did for
`reference_library_catalog.rs`, and found that three of them are blocked by the **same** shape —
the converted package does not hold the facts they read — and that the reason was not the readers
and not the renderer.

**It was the converter, and it was one line.** `record_from_json` built `shipped_tokens` from the
corpus record's `raw_tokens` alone. The ingest stores a `.lst` row's `BONUS:` clauses in a
**second** array, `raw_bonus_chains`; and a shipped token list **replaces** the base row's in
`PinnedTree::closure`. So for every record that ships tokens at all, its own `BONUS:` clauses were
dropped on the floor — silently, because a token nobody reads is not a refusal. **1,679 of 1,750**
joined corpus records lost every BONUS clause they state.

`Dwarf ~ Ability Scores` is the shape: it states `BONUS:STAT|CON,WIS|2|TYPE=Racial` and
`BONUS:STAT|CHA|-2|TYPE=Racial`, and its converted rule carried `value: Text`, no `target` and no
`bonus_type`. The racial ability adjustment — the number a player's sheet is built on — was absent
from the package the sheet is printed from, and nothing said so.

- **Commit SHA:** `c3075955c0` — `src/pcgen_import/sheet_rule/mod.rs` and the regenerated
  `data/sheet_rules/`. Cycle start `aaeae79cab`. This receipt, the parity artifact, the
  `progress.md` entry, the `kanban.md` row and this cycle's four retro events ride the following
  commit — a receipt cannot name the commit that carries it.
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
- **Files touched:** **1 tracked source path, the regenerated package, three generated ledgers,
  plus this receipt, the parity artifact and the board rows. No live-side source, no `apps/`, no
  corpus record, no inventory.**
  - **`src/pcgen_import/sheet_rule/mod.rs`** — `record_from_json` appends each
    `raw_bonus_chains` entry to `shipped_tokens` as the `BONUS:<sub>|<target>|<value>|<extras>`
    value the mapping table already addresses, traversed through the converter's own named
    reader `ingest_record::bonus_chain_qualifiers` rather than open-coded (AT-35-E6-002 cycle 4's
    standing correction). New per-kind gate
    `every_ingested_bonus_chain_reaches_the_converter` reads the **live corpus directory** and
    the converter's own census, never a fixture (`decisions.md` §4).
  - **`data/sheet_rules/`** — regenerated once (`sheet_rule_convert`, then `-- --check`).
    1,330 rule files changed, 12 var tables added.
  - **`docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-coverage.json`**,
    **`docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`**
    (`derived_at` stamp only), **`scripts/oracle_harness/var_names.json`** (+11 names) — all three
    rewritten by their own generators.
  - **`…/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle9_sheet-parity-after.json`**, this receipt,
    `docs/retro/events/at-35-e6-003.jsonl` (+4: three `correction`, one `deferral`), the
    `progress.md` entry and the `kanban.md` row.
  - **Not committed, and deliberately:** `.worktrees/ci-trait-choice` shows in an unfiltered
    `git status --porcelain` as untracked. It is a **git worktree** — another checkout of this
    repository — present before this cycle started.
- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.**
  ```bash
  SC="src/rules_core src/pcgen_import apps/desktop/src-tauri/src apps/desktop/src \
      docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit"
  git diff --unified=0 aaeae79cab -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'      -> 0
  ```
- **Wired-integration audit result:** **OK_NO_TOKENS.**
  ```bash
  git diff --unified=0 aaeae79cab -- $SC ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -ciE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'  -> 0
  ```
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
  > `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
  > deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  **Not met.** `apps/desktop/` is still at 4 files / 97 hits, unchanged by this cycle. This cycle
  did not touch `apps/`: it removed the reason the last four readers cannot be swapped, rather
  than swapping one against a package that does not yet hold what it reads.
- **Receipt rows (mechanical):**
  ```
  CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E6-003 \
  python3 scripts/cycle_scope_gate.py --receipt --since aaeae79cab3619ecb85427b934cc83e5eed758c3 \
    --before /tmp/wi-before-AT-35-E6-003.json --after docs/work-inventory.json
  since=aaeae79cab3619ecb85427b934cc83e5eed758c3 target_dir=/tmp/cargo-sd35-AT-35-E6-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=121 ratio=n/a builds_recorded=2 pcgen_live_files=200
  ```
  `closed=0` is correct and by design (`decisions.md` §2): Epic 6 moves no unit. `pcgen_live_files`
  held at 200 — this cycle did not raise it and did not lower it, because it wrote nothing on the
  live side.
- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`
  ```
  root src/rules_core files=196 hits=11414
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=4 hits=97
  identifier_files=9 identifier_hits=98
  live_files=200 live_hits=11511 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Identical to cycle 8's closing figure on every field, at cycle start and at cycle end. Never
  above the baseline.
- **Oracle parity:** **run, and it moved.** `sheet_rule_parity` over the 29-character fixture
  roster, joined to the committed PCGen BatchExporter exports at
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`:

  | | cycle 2 baseline | this cycle |
  |---|---:|---:|
  | lines compared | 146 | **156** |
  | lines agree | 145 | **154** |
  | lines disagree | 1 | **2** |
  | lines unverifiable | 16 | 67 |
  | chassis compared / agree / disagree | 382 / 376 / 6 | 382 / 376 / 6 |

  **Ten more rendered `Number` lines now reach the comparison**, nine of them agreeing with PCGen
  outright. **Every disagreement, named.** Six of the eight are the halfling / paladin save
  totals and one is the `Weapon Focus` attack line — all seven carried over unchanged from the
  cycle 2 baseline, and none is this cycle's. **Exactly one is new**, and it is a frame mismatch
  rather than a wrong magnitude:
  `half_elf_fighter_l1 · target:Pool:favored_class · ours=1 · oracle=2 (POOL.7.SIZE)`. Our line
  states **`core_rulebook:race_trait:half_elf_multitalented`'s own contribution** — `+1`, which is
  exactly what the corpus row's `BONUS:ABILITYPOOL|Favored Class|1` states; PCGen's `POOL.7.SIZE`
  states the **resulting pool total** (the character's one favored class plus this trait's +1).
  A per-rule contribution compared against a total disagrees by construction. Recorded as a
  `correction` event against the harness's pool frame, not against the number.
  Artifact: `AT-35-E6-003_cycle9_sheet-parity-after.json`.
- **Movement, four buckets:**
  - **closure (into DONE):** none — Epic 6 closes no unit by design.
  - **relabel:** none.
  - **reachability:** the cycle's whole content, measured over the package, corpus-wide.
    - **BONUS clauses reaching the converter: 71 of 1,750 → 1,750 of 1,750.** The gate's own
      figure, before and after, over the live corpus directory.
    - **Rules written 69,346 → 70,135 (+789).** A multi-target `BONUS:` becomes one rule per
      target: `Dwarf ~ Ability Scores` is now three rules — `Number(2) → Ability(Con)`,
      `Number(2) → Ability(Wis)`, `Number(-2) → Ability(Cha)`, each `bonus_type: Racial`.
    - **`race_trait` rules 2,706 → 2,870; those carrying a `target` 530 → 922 (+392); those
      carrying an `Ability` target 173 → 248 (+75).**
    - **Var-table contributions 18,134 → 18,970 (+836); tables carrying any 4,630 → 4,665.**
      The Intelligent Item Ego variable (`v027321c791a2c8bd`) went from **0 contributions to
      175** — which is precisely the mechanics table `intelligent_item_catalog.rs` builds by hand
      from `raw_bonus_chains` today, now held in our own schema.
    - **Var tables 5,281 → 5,293 (+12).**
  - **instrument-correction:** **two.**
    1. The converter finding itself, recorded as a `correction`: the converted package did **not**
       hold every BONUS clause a corpus record states, and no instrument said so, because a
       dropped token is not a refusal. The new gate is the instrument that makes it impossible to
       drop one again.
    2. This cycle's own first gate draft reported "10 of 1750" offenders — that 10 was the
       **display cap** on the example list, not the count. Re-derived to 1,679 with a separate
       counter before any figure left the cycle. Self-healed in the same cycle; named here
       because a capped example list masquerading as a total is exactly the shape
       `AGENTS.md` rule 9 is about.
  - A third `correction` — this cycle's first retro call inherited a stale `RETRO_ACTOR` from the
    shell environment and landed one event in `docs/retro/events/sd31-transcribe.jsonl`. The log is
    append-only, so the stray line stays and the event was re-emitted under the right actor.
    `RETRO_ACTOR` is now set per invocation, never inherited.
- **Refused tokens:** **97 hits across the same 4 `apps/desktop` files, unchanged** — by pattern
  `PRE[A-Z]+:`=27, `raw_tokens`=24, `DESC:`=14, `BONUS:`=11, `raw_bonus_chains`=10,
  `render_pcgen_desc`=8, `TYPE=`=3; by file `race_trait_picker.rs`=33,
  `intelligent_item_catalog.rs`=28, `raceCreationCoverage.test.ts`=21,
  `reference_library_catalog.rs`=15. Both partitions sum to 97, and both agree with the gate's own
  `root apps/desktop files=4 hits=97`. **7 distinct token types**, under
  `workflow-instruction.md §8`'s limit of 10. Recorded as a `deferral` event with its reason.

  **What each of the four actually needs, measured this cycle rather than assumed:**

  | file | hits | what blocks the swap | the number |
  |---|---:|---|---|
  | `race_trait_picker.rs` | 33 | its exclusion guard resolves to sibling **rules** in the converted `applies`, and 16 ARG heritage selectors have no converted rule | **343 of 415** alternates' blocker sets already agree exactly; the other **72** would lose their guard, and every one of the 72 loses it to those same 16 records |
  | `intelligent_item_catalog.rs` | 28 | needed the component `BONUS:VAR` chains in the package | **unblocked by this cycle** — the Ego var alone went 0 → 175 contributions |
  | `raceCreationCoverage.test.ts` | 21 | ability adjustments needed `target`/`value` on the converted rule | **unblocked for the 18 races it covers** (all agree); the 12 disagreements are ARG races, same 178-record cause |
  | `reference_library_catalog.rs` | 15 | cycle 7's measured loss, unchanged | **1,150 of 9,679** descriptions |

  **The blocker is one mechanism, and it is inventory scope, not this criterion's.** **178** ARG
  `race_trait` corpus records are **not units of `docs/work-inventory.json`** (`advanced_race_guide`
  has 315 `race_trait` units; the corpus has 493), and `sheet_rule::load_population` walks the
  inventory's units, so the converter's population never sees them. This is the **same** mechanism
  cycle 7 recorded for 489 `ability` records. Admitting them moves the bundle-wide denominator
  49,438. **Reported, not excused.**
- **Discoveries:** **three.**
  1. **A dropped token is not a refusal, and no instrument in this bundle was watching for one.**
     `token_coverage.py`, `_refused.json` and the census all read what the converter *saw*. A
     clause it never saw is invisible to every one of them — the ledgers were internally
     consistent and materially incomplete for 1,679 records. The general lesson: a coverage
     instrument built over the reader's own input cannot detect input the reader never received;
     it needs a second, independent enumeration of the source. That is what the new gate is.
  2. **Two token types surface unmapped for the first time — `BONUS:LOADMULT` (4 records) and
     `BONUS:SPELLCASTMULT` (4).** Both are **multipliers**, and no `BonusTarget` variant holds a
     multiplier; writing one as an additive target would put a wrong number on a sheet. They
     degrade honestly instead — the rule prints its words (`decisions.md` §1 form 3) — and
     degraded records rose 398 → 423 for that reason. `token_coverage.py --check` is `PASS` with
     `refused_non_done=0`. Named, not exempted: the mapping-table question is "what sheet total
     does a multiplier feed", and it has no answer yet.
  3. **The converter reads the pinned `.lst` rows for *everything except* the record's own base
     row, which it takes from the corpus JSON.** That hybrid is deliberate (the corpus row is
     product-identity-screened), and it is exactly where this defect lived: the screened copy is
     not token-complete, and nothing asserted that it was. Any future ingest field that splits a
     row's tokens into another array will reproduce this bug shape. Worth one predicate:
     *does the shipped token list account for every clause on the source row?*
- **Figures + their re-derive commands:**
  | figure | denominator | command |
  |---|---|---|
  | 1,679 of 1,750 joined records dropped every BONUS clause (before); 0 (after); 1,895 corpus records carry chains; 145 are not inventory units | every `data/corpus/` record with a non-empty `raw_bonus_chains` | `cargo test --locked --lib -j 6 every_ingested_bonus_chain_reaches_the_converter` (before: at `aaeae79cab`) |
  | rules_written 69,346 → 70,135; var_tables 5,281 → 5,293; degraded 398 → 423; refused 142 both sides | every inventory unit (49,438) | `git show aaeae79cab:data/sheet_rules/_report.json` vs `data/sheet_rules/_report.json` |
  | `race_trait` rules 2,706 → 2,870; with a target 530 → 922; with an `Ability` target 173 → 248 | every rule under `data/sheet_rules/*/race_trait/` | `python3 -c "import json,glob,collections;c=collections.Counter();n=0` … counts `target` / `target.Ability` over that glob, run against a `git archive aaeae79cab` extract for the before |
  | var contributions 18,134 → 18,970; tables with any 4,630 → 4,665; Ego var 0 → 175 | every table under `data/sheet_rules/_vars/` | `python3 -c "import json,glob;f=glob.glob('data/sheet_rules/_vars/*.json');c=[len(json.load(open(x))['contributions']) for x in f];print(sum(c), sum(1 for n in c if n))"` for the after, the same line against a `git archive aaeae79cab data/sheet_rules \| tar -x -C <dir>` extract for the before; the Ego var alone is `python3 -c "import json;print(len(json.load(open('data/sheet_rules/_vars/v027321c791a2c8bd.json'))['contributions']))"` |
  | 343 of 415 alternates' blocker sets agree; 72 differ; 16 ARG heritage selectors carry no converted rule | every `race_trait` corpus record with a non-empty `sets_replace_flags` | the census script reproduces `exclusion_guard_flags` + `replacement_targets` against the converted `applies`' negated `Holds{Rule}` set; figures re-derivable from `data/corpus/*/race_trait/` and `data/sheet_rules/*/race_trait/` alone |
  | 178 ARG `race_trait` corpus records are not inventory units; ARG has 315 `race_trait` units | `data/corpus/advanced_race_guide/race_trait/` vs `docs/work-inventory.json` | `python3 -c "import json;wi=json.load(open('docs/work-inventory.json'));print(sum(1 for u in wi['units'] if u['book']=='advanced_race_guide' and u['kind']=='race_trait'))"` against the corpus file count |
  | 97 residue hits by token type and by file, both summing to 97 | the four `apps/desktop` files | `python3 scripts/pcgen_residue_gate.py --check`, and the per-file breakdown command recorded in the cycle 8 receipt |
  | oracle parity 156 / 154 / 2 / 67 and the one new disagreement | every rendered `Number` line over the 29-character roster | `cargo run --locked --release --bin sheet_rule_parity -- --roster docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/oracle-parity/roster --output <ours.json>` then `python3 scripts/oracle_harness/sheet_parity.py compare --ours <ours.json> --exports docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/oracle-parity/exports --output <parity.json>` |
  | `data/sheet_rules/` source markers: 0 | every generated rule file | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | rust_lines_changed 121 | `*.rs` since `aaeae79cab` | the `--receipt` invocation above |
- **Build scope verified:** run at `c3075955c0`, `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E6-003`
  for the root workspace (`AGENTS.md`: one directory per agent **per source tree**);
  `…-parity` for the release parity binary.
  ```
  cargo test --locked --no-run -j 6                  NO_RUN_EXIT=0 (every test binary linked)
  cargo test --locked --lib -j 6                     ok. 3300 passed; 0 failed; 15 ignored
  cargo test --locked --no-fail-fast -j 6            414 targets, 8811 passed, 0 failed, 68 ignored,
                                                     0 FAILED suites, FULL_EXIT=0
  cargo clippy --locked --tests -j 6 (root)          0 warnings, 0 errors
  cargo run --locked --bin sheet_rule_convert        records=49438 converted=49296 refused=142
                                                     rules=70135 var_tables=5293 files=54604
  cargo run --locked --bin sheet_rule_convert -- --check   EXIT=0 over the regenerated package
  python3 scripts/pcgen_residue_gate.py --check      verdict=PASS (live_files 200, unchanged)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l      0
  python3 scripts/completion_atlas.py --check        DONE 49438 of 49438, missing_clearing_mechanisms=0 citation_failures=0
  python3 scripts/token_coverage.py --check          non_done=0 refused=142 refused_non_done=0 shapes=1 verdict=PASS
  python3 scripts/shape_engine_boundary.py --check   magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True
  python3 scripts/missing_engine_tables.py --check   population=0 kinds=0 citation_failures=0
  python3 scripts/denominator_gate.py --check '…/*.md' '…/artifacts/**/*.md'      files_checked=106 violations=0
  python3 scripts/denominator_gate.py --check-provenance    files_checked=223 figures_examined=559 violations=0
  scripts/verify.sh --only pi-sweep                  RESULT: PASS
  ```
  The desktop crate and the frontend run at the epic wrap-up (`workflow-instruction.md §6`
  step 3): this cycle touched no file under `apps/`. `corpus_literal_sweep` was **not** run — no
  corpus record changed (`git status --porcelain -- data/corpus` empty at every checkpoint).
  `v06_work_inventory` was **not** re-run: its inputs are unchanged and the inventory is already
  at `DONE 49438 of 49438`.
- **Sweep population:** N/A — no corpus record changed.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. The parity figures
  above are the only ones in this receipt derived from the pinned corpus.
- **Status:** **partial**
- **What this proof does not cover** (`AGENTS.md` rule 7): the gate proves every ingested BONUS
  chain now *reaches* the converter, **not** that every one of them lowers to a correct value —
  a clause that arrives and converts wrongly passes it. The package-level figures (+789 rules,
  +836 contributions, +392 race-trait targets) are **counts**, not an id-set diff or a value
  check, so a wrong magnitude inside a right count would read as a gain. What does check values
  is the oracle comparison, and it covers **156 lines across 29 fixture characters** — one
  roster, mostly level 1 and level 10 single-class humans; the newly-visible equipment and
  equipment-modifier chains (1,971 records, the largest share by far) are **not exercised by any
  fixture in it**, so nothing here proves their values against PCGen. The 145 → 154 agreement
  gain is real and the frame of the one new disagreement is diagnosed, but the 67 unverifiable
  lines (up from 16, because more lines now render and more of them have no exported counterpart)
  are neither agreement nor disagreement. Nothing in this cycle touched or proves anything about
  the four `apps/desktop` files.
- **Notes:** the regenerated package changes 1,330 rule files in one commit. That is the
  one-build-per-cycle rule working as intended (`decisions.md` §3), not a scope excursion: the
  converter is one input and the package is its output, and a partial regeneration would be a
  package that matches no converter.
- **Next-cycle scope:** **AT-35-E6-003 cycle 10.** Two of the four readers are now unblocked and
  should be taken first, in this order:
  (1) **`intelligent_item_catalog.rs` (28)** — build the component's mechanics from the converted
  var tables' `contributions` (reverse-index `rule_id → [(var, expr, when)]`; the Ego var alone
  holds 175), and its description from `converted_prose`; 167 of its 171 components already join
  book-exact, and the 4 that miss are two duplicate-slug pairs the `#`-suffixed ids and the
  source-row join step already address.
  (2) **`raceCreationCoverage.test.ts` (21)** — the ability-adjustment derivation now agrees with
  the package for every race it covers; the size, vision and floating-pool derivations must each
  be measured against the package the same way before they are swapped, not assumed.
  Then **`race_trait_picker.rs` (33)** and **`reference_library_catalog.rs` (15)**, both of which
  need the **inventory ruling** first: 178 ARG `race_trait` records and cycle 7's 489 `ability`
  records are corpus records that are not inventory units, and until they are, the converter
  cannot hold what those two readers serve. That ruling moves the bundle denominator 49,438 and
  belongs to the operator, not to this criterion. Scope flags:
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Target: `root apps/desktop files=2 hits=48`, then zero.
