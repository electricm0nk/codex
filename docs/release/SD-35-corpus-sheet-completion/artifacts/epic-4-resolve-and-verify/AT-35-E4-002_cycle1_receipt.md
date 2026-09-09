# Cycle 1 — Epic 4, Resolve and verify / AT-35-E4-002

- **Commit SHA:** `2645a3c85a` (cycle start `cdcfc897ea`)
- **Scope gate:**
  ```
  inventory=docs/work-inventory.json
  scope=bucket=V
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  Bucket V was already 0 at the cycle start — `AT-35-E3-002_cycle1_receipt.md` (`26bdfa8d5b`)
  closed V 392 → 0 — and so was every other bucket, so the bundled scope the dispatch mandates
  **is** the whole remainder: `PASS_WHOLE_REMAINDER`, not a floor exemption and not an
  under-floor cycle. No other criterion's units were available to bundle in because none exist.
  What was outstanding on this card was the second half of the Evidence sentence: the
  corpus-wide oracle run, which had never been made (kanban row 17, deferral
  `1788922132640-at-35-e3-002-ac4da5`). This cycle makes it.
- **Files touched:**
  - `scripts/oracle_harness/bucket_v_parity.py` (new — the run: `units` / `carriers` / `export` / `compare`)
  - `scripts/oracle_harness/bucket-v-abilities.txt.ftl` (new — the BatchExporter template)
  - `src/bin/sheet_rule_bucket_v_render.rs` (new — the engine side, the live evaluator)
  - `scripts/tests/test_bucket_v_parity.py` (new — 16 tests)
  - `artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_bucket-v-parity.json` (the run's verdicts)
  - `artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_bucket-v-units.json` (the 392 units with their pinned rows)
  - `artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_ours.json` (the rendered lines)
  - `artifacts/epic-4-resolve-and-verify/bucket-v-carriers/{manifest.json,carrier_character.txt}`
  - `docs/retro/events/at-35-e4-002.jsonl` (4 `correction`, 1 `deferral`)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the
    atlas instrument's own `derived_at` stamp, rewritten by `completion_atlas.py --check`)
  - `docs/retro/events/{codex,root,sd31-transcribe}.jsonl` (live appends on the shared
    checkout, folded — the tree is clean unfiltered)

  **No live-side file was touched.** `data/sheet_rules/`, `src/rules_core/`, `src/pcgen_import/`
  and `apps/` are byte-identical to the cycle start: this is a verification cycle.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS.
  `BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47` (`git merge-base HEAD origin/develop`);
  `git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <the file-touch set + this cycle's four new files> ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no match, and the same grep over this cycle's own diff → no match.
- **Wired-integration audit result:** OK_NO_TOKENS on this cycle's own four new files
  (`grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match)
  after one self-heal: the first draft of `bucket_v_parity.py` carried the word *placeholder*
  in a comment describing what is **not** a magnitude (`a DESC slot placeholder`); reworded to
  `a %1 slot marker` in the same commit (`workflow-instruction.md §8`, single-token audit
  violation). Over the whole file-touch set since `fe5ae6cd4a` the same grep matches only the
  **generated Paizo prose inside `data/sheet_rules/`** that `AT-35-E4-001_cycle1_receipt.md`
  and `AT-35-E3-002_cycle1_receipt.md` already recorded (10 hits, unchanged by this cycle,
  no shipping code path involved).
- **Acceptance criterion** (verbatim, `epic-breakdown.md § AT-35-E4-002`):
  > 392 units at authoring. One corpus-wide run of `scripts/oracle_harness/`, per-unit cost
  > measured on the first 50 and projected wall time stated before the full run. Verdicts
  > booked as `oracle-agree` / `oracle-unverifiable`. **This run compares the live evaluator's
  > values, not the old string-formula path's** — so it is also the parity proof for whatever
  > Epic 6 has not yet retired.
  >
  > **Evidence:** V at 0; the harness receipt with `PCGEN_ORACLE_SHA`; `oracle_disagreement=<n>
  > of 392`, every disagreement named.
  >
  > **Inherited from AT-35-E2-005 (2026-09-08, `decisions.md §16`, `### AT-35-E2-005-DISPOSITION`):**
  > the **391** non-refused bucket-V units of 1,404 non-DONE at `38b67db94e` —
  > `literal-verified` 388 + `fixture-verified` 3 (by kind class_feature 184, race_trait 152,
  > equipment 42, equipment_modifier 9, spell 3, feat 1). The 392nd V unit at HEAD is
  > converter-refused and is AT-35-E4-001's.
- **Receipt rows (mechanical):**
  ```
  since=cdcfc897ea31e832825f70ec13a2c41db6a0f9d5 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=171 ratio=n/a builds_recorded=1 pcgen_live_files=260
  ```
  `closed=0` / `relabeled=0` is correct and expected: the unit population was **already** 0
  non-DONE when the cycle started. `ratio` is `n/a`, a division by zero, never `0.0`. The 171
  Rust lines are the new engine-side binary; no existing Rust file changed.
- **PCGen residue:**
  ```
  live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Identical at start and at end. Everything this cycle added is tool-side
  (`scripts/oracle_harness/`, `src/bin/`); `src/bin/sheet_rule_bucket_v_render.rs` reads
  `data/sheet_rules/` through `corpus_loader` and calls `render_sheet` — no PCGen token, no
  formula string, no `raw_tokens`.
- **Oracle parity:**
  ```
  bucket_v_parity compare: compared=392 oracle_agree=184 oracle_disagreement=10 of 392
    oracle_unverifiable=198 PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6
  by tier: {'export:oracle-agree': 165, 'export:oracle-disagree': 9, 'export:oracle-unverifiable': 112,
            'source:oracle-agree': 19,  'source:oracle-disagree': 1,  'source:oracle-unverifiable': 86}
  ```
  **Two tiers, never conflated** (`AGENTS.md` rule 7 — a proof is only as wide as the cases it
  covers):
  - **`export` — 286 of 392 units (73.0%).** PCGen's **BatchExporter**, run over 21 carrier
    characters (one per campaign closure, each carrying every unit of that closure as an
    `ABILITY:` line), comparing our rendered value against the numbers PCGen's **engine**
    substituted into that ability's `DESCRIPTION`. This tier exercises PCGen, not just its data.
  - **`source` — 106 of 392 units (27.0%).** The record's own row in the pinned checkout,
    located by `source_file` + `source_line` and identity-checked against `corpus_key`. This
    tier reaches every unit but is a **data** oracle: it does not exercise PCGen's engine.
    It is the tier for the 57 units no carrier can hold (equipment 42, equipment_modifier 9,
    spell 3, feat 1, no `CATEGORY:` 2) plus the units a carrier did not grant.
  - **What neither tier covers:** a rendered value that depends on a character fact the
    carrier does not have. The engine side renders at the carrier's own context
    (`bucket-v-carriers/carrier_character.txt`, Human Fighter 20, every score 14, the twin of
    the `.pcg`), so level- and ability-dependent values are compared on matched characters —
    but only that one character. A value that would differ for a different race, class or
    level is unproven outside it.
- **The 10 disagreements, every one named** (`AT-35-E4-002_cycle1_bucket-v-parity.json`
  `disagreements[]`; `ours` / `oracle` are the comparable-token sets, `missing` is what ours
  carries and the oracle does not). Two mechanical causes:

  **Cause A — `value-role-number-the-oracle-never-prints-words-agree` (8).** Our line's
  *value column* carries a number PCGen never prints for the record, while every number in our
  *rendered words* is one PCGen prints too. The sheet's prose agrees with the oracle; the value
  column is the whole disagreement.

  | unit | ours | oracle | tier |
  |---|---|---|---|
  | `advanced_players_guide:class_feature:antipaladin_unholy_champion` | `+5` | `0, 10` | export |
  | `bestiary_5:race_trait:clockwork_familiar_item_installation` | `1` | `1d6` | export |
  | `inner_sea_magic:class_feature:divine_scion_domain_specialization` | `+1` | `2` | export |
  | `occult_adventures:class_feature:spiritualist_shared_consciousness` | `+1` | `4, 8` | export |
  | `occult_adventures:race_trait:emotional_focus_zeal_tracking` | `+0` | `1` | export |
  | `occult_adventures:race_trait:phantom_manifestation_incorporeal` | `+0` | `30, 301` | export |
  | `pathfinder_unchained:race_trait:unchained_evolution_climb` | `+0` | `20` | export |
  | `pathfinder_unchained:race_trait:unchained_evolution_swim` | `+0` | `20` | export |

  **Cause B — `rendered-words-disagree` (2).** A number inside our *rendered words* is not one
  the oracle prints. These are the two that touch what a player reads, and each has its own
  `correction` retro event.

  | unit | ours | oracle | tier | what differs |
  |---|---|---|---|---|
  | `core_rulebook:class_feature:evocation_school_force_missile` | `0, 1, 1d4` | `0, 1d4` | export | our `Desc` prose prints `1d4+0` and our `Ability Benefit` aspect prints `1d4+1` **on the same line, for the same character**; PCGen prints `1d4+0` in both. One `Var`, two rendered values. `1788955474174-at-35-e4-002-0d2ae3` |
  | `monster_codex:race_trait:bat_sootwing_paralysis` | `0, 0d0` | `1, 2, 3, 4` | source | our aspect renders `(0d0+0 rounds, DC 0, …)`; the record's own pinned row declares `BONUS:VAR\|UMR_Paralysis_DurationDice\|1`, `…DurationDieSize\|4`, `…DurationBonus\|1`, so the substitution owes `1d4+1`. `1788955474302-at-35-e4-002-077da1` |

  **Disposition:** the 10 are booked as **Epic 6's parity baseline**, not fixed here
  (`deferral 1788955474431-at-35-e4-002-0f136c`). AT-35-E4-002's bar is *"every disagreement
  named"*; the fixes are live-side evaluator changes, and AT-35-E6-001/E6-004 run the oracle
  **before and after** the PCGen exit precisely so a live-side change is measured against a
  named baseline. Fixing them in this cycle would move the baseline the exit is measured
  against. This is the same treatment `AT-35-E4-001_cycle1_receipt.md` gave its 7 pre-existing
  `sheet-parity` disagreements, and it is not a carve-out: every one of these 392 units is
  DONE under the sheet rule (its words render and they agree with the oracle's words).
- **The 198 `oracle-unverifiable` verdicts, by named reason** (no bucket, no "the rest"):

  | reason | n | what it means |
  |---|---|---|
  | `line-carries-no-number` | 79 | the sheet line renders words carrying no number at all. Under the sheet rule that is a finished line, not a gap: there is nothing to compare. |
  | `export-desc-has-no-number` | 63 | PCGen granted the ability and printed its description, and that description carries no number in the compared role. The oracle has nothing to assert. |
  | `rule-is-print-false-nothing-reaches-the-sheet` | 37 | the rule is `print: false` — a grouping/container record (28 `race_trait`, 8 `equipment_modifier`, 1 `class_feature`) whose children carry the sheet lines. All 37 are in the package; none is missing. |
  | `pinned-row-declares-no-number` | 19 | the record's own row at the pin declares no magnitude in any magnitude-bearing token. |
- **Movement, four buckets:**
  - **closure (into DONE):** 0 — the population was already 0 non-DONE at the cycle start
    (`completion_atlas.py --check` → `DONE: 49438`).
  - **relabel (bucket to bucket):** 0. No unit's status or evidence changed; `docs/work-inventory.json`
    is byte-identical at start and end (`regressed=0 added=0 dropped=0`).
  - **reachability:** 286 of 392 bucket-V units are now reachable by PCGen's **engine**, up
    from 1 (`ours.json` ∩ the pre-existing 29-member `sheet_parity` roster before this cycle:
    exactly one V unit). The other 106 are reachable by the pinned **data** oracle. 0 are
    unreachable by either.
  - **instrument-correction:** two, both in this cycle's own new harness and both caught by
    the run itself — the character-context mismatch (`1788955473919-…-afaf11`) and the
    `MAGNITUDE_TOKENS` gap that read no numbers out of `TARGETAREA:` / `SAVEINFO:` / `COMPS:` /
    `ASPECT:` (`1788955474049-…-37209c`, one false disagreement, 10 → 9 before the dice-token
    widening took it to 10 again on a different unit).
- **Refused tokens:** **none.** This cycle added no converter mapping row, no refusal and no
  `_refused.json` entry; `token_coverage.py --check` reports `refused=142 refused_non_done=0
  token_types=231`, identical to `AT-35-E4-001_cycle1_receipt.md`.
- **Discoveries:**
  1. **The value column and the words can disagree with each other inside one rendered line.**
     `evocation_school_force_missile` renders `1d4+0` in its `Desc` prose and `1d4+1` in its
     `Ability Benefit` aspect for the same character — the same `Var` resolved twice, to two
     values. Nothing in the package gates caught it because both renders are individually
     well-formed. `correction 1788955474174-at-35-e4-002-0d2ae3`.
  2. **A `.MOD` / `.COPY=` row is the record's own row, and 33 of 392 units point at one.**
     A first pass that compared `corpus_key` against the row's leading field alone read those
     33 as identity mismatches. PCGen's row syntaxes (`CATEGORY=<cat>|<name>.MOD`,
     `<name>.COPY=<KEY>`) carry the identity in the suffix; the harness now reads it there and
     all 392 units identity-match. Pinned as `PinnedRowTests` in the self-test.
  3. **Campaign closures are computable, not guessable.** `charbuild_remainder_generate.py`
     hand-wrote three closures and recorded six books that failed under a wrong one. Reading
     each `.pcc`'s own `PRECAMPAIGN:N,<clauses>` chain transitively produced a working closure
     for **21 of 21** books with **0 export failures** — including `Bestiary 5` (8 books) and
     `Adventurer's Guide` (9). That is what took the export tier from 4 carriers / 150 units to
     21 carriers / 335 units offered, 286 actually granted.
- **Figures + their re-derive commands** (denominator: **392 bucket-V units**, the atlas
  partition of `docs/work-inventory.json` at `38b67db94e`, unless stated):
  | figure | value | command |
  |---|---|---|
  | bucket V at HEAD | **0** of 49,438 units | `python3 scripts/completion_atlas.py --check` → `DONE: 49438` |
  | the bucket-V population | **392** (class_feature 185, race_trait 152, equipment 42, equipment_modifier 9, spell 3, feat 1) | `python3 scripts/oracle_harness/bucket_v_parity.py units --out <dir>` |
  | units located in the pinned data, identity-checked | **392 of 392** | same command, `located_in_pinned_data=392`; `identity_matches_corpus_key` true for all 392 in `AT-35-E4-002_cycle1_bucket-v-units.json` |
  | units the live evaluator rendered a line for | **355 of 392** (the other 37 are `print: false`) | `cargo run --locked --release --bin sheet_rule_bucket_v_render -- --units <dir>/ids.json --character <dir>/carriers/carrier_character.txt --output <dir>/ours.json` → `rendered=355` |
  | carriers built / exported / failed | **21 / 21 / 0** | `python3 scripts/oracle_harness/bucket_v_parity.py export --carriers <dir>/carriers --out <dir>/exports --jobs 3` |
  | units offered to a carrier / reached by the export tier | **335 / 286** | `carriers` (`units_carried=335`); `compare` `by_tier` export rows sum to 286 |
  | compared / agree / disagree / unverifiable | **392 / 184 / 10 / 198** | `python3 scripts/oracle_harness/bucket_v_parity.py compare --units <dir> --ours <dir>/ours.json --exports <dir>/exports --output <dir>/bucket-v-parity.json` |
  | `oracle_disagreement` | **10 of 392** | same |
  | PCGen live-side files | **260** (baseline 260) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | harness self-test | **16 passed, 0 failed** | `python3 -m unittest scripts.tests.test_bucket_v_parity` |

  **Per-unit cost, measured before the full run** (the criterion's own requirement), and the
  projection that was stated from it:
  | stage | measured | projection stated | actual |
  |---|---|---|---|
  | engine side (live evaluator) | first **50** units in **0.02 s** = **0.4 ms/unit** | 392 units ≈ 0.2 s render + ~3 s package load | `render=0.2s`, total **2.8 s** |
  | `source` oracle tier | 392 units in **1.5 s** = **3.8 ms/unit** | — | **1.5 s** |
  | `export` oracle tier | first **3 carriers (46 units)** in **47.0 s** at `--jobs 3` = **1.02 s/unit** | 21 carriers at `--jobs 3` ≈ 21/3 × 47 s ≈ **329 s (5.5 min)** | **264.1 s (4.4 min)**, 0 failures |
  | one BatchExporter JVM, Core-Rulebook-only closure | **20.74 s** | — | — |
- **Build scope verified**, run at `2645a3c85a`:
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → `ok. 3220 passed; 0 failed; 14 ignored`
  - `cargo test --locked --no-fail-fast -j 6` → see **Full-suite result** below
  - `cargo clippy --locked --tests -j 6` → **0** `warning:`/`error:` lines, after one self-heal
    (`clippy::ptr_arg` on the new binary's `&PathBuf` parameter, changed to `&Path` in the same
    commit)
  - `python3 -m unittest scripts.tests.test_bucket_v_parity` → `Ran 16 tests … OK`
  - `python3 scripts/pcgen_residue_gate.py --check` → PASS, 260 files (baseline 260)
  - `cargo run --locked --bin sheet_rule_convert -- --check` → exit 0
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
  - `python3 scripts/completion_atlas.py --check` → `population=49438 unclassified=0 overlap=0`,
    `DONE: 49438`, every other bucket 0, `citation_failures=0`
  - `python3 scripts/token_coverage.py --check` → `verdict=PASS`, `refused_non_done=0`
  - `python3 scripts/shape_engine_boundary.py --check` → `not_held_by_engine=0 citation_ok=True`
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 citation_failures=0`
  - `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` → `files_checked=55 violations=0`
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`
  - `cargo run --locked --bin corpus_literal_sweep` — **not run, and not owed**: no corpus
    record changed (`data/corpus/**` untouched, `git status --porcelain` clean there).
  - the desktop crate and the frontend did **not** run: this cycle touched no path under
    `apps/`. They run at the Epic 4 wrap-up (`workflow-instruction.md §10`).
- **Full-suite result:** `cargo test --locked --no-fail-fast -j 6` at `2645a3c85a` →
  **8,727 passed, 0 failed, 67 ignored over 412 targets executed / 413 `test result` lines**,
  process `EXIT=0`, wall ≈ 74 min. Zero failing suites; no self-heal was needed. Re-derive:
  `awk '/^test result: ok\./ {p+=$4; f+=$6; i+=$8} END {print p, f, i}' <log>` and
  `grep -c '^     Running ' <log>`. The 8,727 figure matches Epic 3's re-pinned
  `BASELINE_ROOT_FULL_TESTS` exactly — this cycle moved no test count.
- **Sweep population:** N/A — no corpus record was written this cycle.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`), the checkout resolved through `$PCGEN_REPO_DIR`, read-only.
  Every figure in the two oracle tiers comes from that pin.
- **Status:** complete
- **Notes:** The harness deliberately **does not** dissolve a dice expression into two scalars:
  under the sheet rule `1d6` is the printed value, so it compares as its own token. Adding that
  took 13 dice-form lines from `unverifiable` to compared (`agree` 180 → 184) and surfaced one
  further disagreement (`clockwork_familiar_item_installation`, ours `1` where the oracle
  prints `1d6`) — a widened proof that found something, which is the point. The read-only
  export ran on the shared checkout rather than an isolated worktree because the cycle needed
  the working tree's own `data/sheet_rules/` for the engine side; it wrote only under `/tmp`
  and the artifacts committed here, and pushed nothing of its own.
- **Next-cycle scope:** criterion at zero — bucket V is 0, and its Evidence sentence is now
  fully paid: the corpus-wide run exists, with `PCGEN_ORACLE_SHA` and
  `oracle_disagreement=10 of 392`, every disagreement named. No other criterion's kanban row
  changes state here: every other bucket was already 0 before this cycle started, and the two
  other `in-progress` rows (22 `AT-35-E5-004`, 23 `AT-35-E5-005`) own deliverables this cycle
  did not build.
