# Cycle 2 — Epic 4, Resolve and verify / AT-35-E4-002

- **Commit SHA:** `89e93ace82` (cycle start `18dbe0e1659eef8fe8608aa2c2c620e5ef39c74a`)
- **Scope gate:**
  ```
  inventory=docs/work-inventory.json
  scope=bucket=V
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The dispatch mandated bundling if `--bucket V` came back under the floor. It did not come back
  under the floor: it came back at **zero**, and the *whole* non-DONE remainder is zero too
  (`python3 scripts/cycle_scope_gate.py --min 500` with no flags → the same
  `scoped=0 remaining_non_done=0 verdict=PASS_WHOLE_REMAINDER`). There is nothing to bundle
  because no criterion in this bundle has a non-empty population at HEAD. This is
  `PASS_WHOLE_REMAINDER`, not a floor exemption and not an under-floor cycle.

  The dispatch also described this as **cycle 1** of AT-35-E4-002 with **392 units** scoped.
  Both figures are the authoring-time figures. Cycle 1 ran on 2026-09-09 at `2645a3c85a`
  (`AT-35-E4-002_cycle1_receipt.md`), paid the whole Evidence sentence, and `kanban.md` row 17
  has read `complete` since. Recorded as correction `1789054081017-at-35-e4-002-82fb71`.
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle2_receipt.md` (this file)
  - `docs/release/SD-35-corpus-sheet-completion/kanban.md` (row 35 — this cycle; row 17 gains a pointer to it, and the preamble's row count 33 → 35, stale since row 34 landed)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md` (prepend)
  - `docs/retro/events/at-35-e4-002.jsonl` (1 `correction`)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the atlas
    instrument's own `derived_at` stamp, rewritten by `completion_atlas.py --check`), and any
    live event-log appends on the shared checkout, folded so the tree is clean unfiltered

  **No code, no data, no inventory changed.** `src/`, `scripts/`, `apps/`, `data/sheet_rules/`
  and `docs/work-inventory.json` are byte-identical to the cycle start — this is a
  re-verification cycle over an already-closed criterion.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS on this cycle's own diff.
  `BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47` (`git merge-base HEAD origin/develop`);
  the epic file-touch set grep over `${BASE_BRANCH}...HEAD` returns one pre-existing hit, a prior
  receipt's own prose quoting its grep pattern and the real test-directory paths
  `tests/sd18_widening/` and `tests/sd13_progression/`
  (`EPIC-4_wrapup_correction_cycle_receipt.md`), already itemised there. This cycle adds no
  match.
- **Wired-integration audit result:** OK_NO_TOKENS on this cycle's own diff.
  The same range grep returns the pre-existing, already-itemised set: generated Paizo prose
  inside `data/sheet_rules/` containing the ordinary English word *hack*
  (`core_rulebook:spell:plant_growth`, `bestiary_3:monster_ability:tophet_swallow_whole`), and
  earlier receipts' own prose quoting the pattern. No shipping-code stub, mock or `"Would …"`
  string. This cycle adds no match.
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E4-002`):

  > ### AT-35-E4-002 — bucket V goes through the oracle harness once
  >
  > 392 units at authoring. One corpus-wide run of `scripts/oracle_harness/`, per-unit cost
  > measured on the first 50 and projected wall time stated before the full run. Verdicts booked as
  > `oracle-agree` / `oracle-unverifiable`. **This run compares the live evaluator's values, not the
  > old string-formula path's** — so it is also the parity proof for whatever Epic 6 has not yet
  > retired.
  >
  > **Evidence:** V at 0; the harness receipt with `PCGEN_ORACLE_SHA`; `oracle_disagreement=<n> of
  > 392`, every disagreement named.

  **Met at HEAD, by cycle 1's artifacts, re-derived here:**

  | Evidence clause | At HEAD | Command |
  |---|---|---|
  | V at 0 | `V: 0`, `DONE: 49438` | `python3 scripts/completion_atlas.py --check` |
  | one corpus-wide run of `scripts/oracle_harness/` | `AT-35-E4-002_cycle1_bucket-v-parity.json`, `population=392`, `units` 392 | `python3 -c "import json;d=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_bucket-v-parity.json'));print(d['population'],len(d['units']))"` |
  | the harness receipt with `PCGEN_ORACLE_SHA` | `7f818006e371188e5717fd18d74d18a420747fc6` | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_bucket-v-parity.json'))['PCGEN_ORACLE_SHA'])"` |
  | `oracle_disagreement=<n> of 392`, every disagreement named | **10 of 392**, all 10 carried as records with `id`, `book`, `corpus_key`, `form` and `cause` | `python3 -c "import json;d=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_bucket-v-parity.json'));print(d['counts'],len(d['disagreements']))"` |
  | verdicts booked as `oracle-agree` / `oracle-unverifiable` | `oracle-agree 184`, `oracle-disagree 10`, `oracle-unverifiable 198` | same command |

  No clause is outstanding, so this cycle adds no work to the criterion; it re-derives the
  evidence at HEAD and records that it still holds.
- **Receipt rows (mechanical):**
  ```
  since=18dbe0e1659eef8fe8608aa2c2c620e5ef39c74a target_dir=/tmp/cargo-sd35-AT-35-E4-002 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=253
  ```
  `closed=0` is correct and is not an under-floor cycle: the scoped population was already 0.
- **PCGen residue:** `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — identical to `AT-35-E4-001_cycle2_receipt.md`'s line; not above it.
- **Oracle parity:** `compared=392 oracle_agree=184 oracle_disagreement=10 oracle_unverifiable=198`
  at `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, export tier 286 of 392
  (`export:` 165 agree + 9 disagree + 112 unverifiable), source tier 106
  (19 + 1 + 86). Re-derived from the committed parity JSON, not re-run: cycle 1's corpus-wide
  run is the run this criterion asks for ("**once**"), the tree's converter and live evaluator
  are byte-identical to that run's tree on every path the run reads, and
  `sheet_rule_convert -- --check` is green at HEAD. Re-running the harness against an unchanged
  evaluator would repeat a clean gate.

  **The 10 disagreements, named:** 8 of form `number`, cause
  `value-role-number-the-oracle-never-prints-words-agree` —
  `advanced_players_guide:class_feature` *Antipaladin ~ Unholy Champion*,
  `bestiary_5:race_trait` *Clockwork Familiar ~ Item Installation*,
  `inner_sea_magic:class_feature` *Divine Scion ~ Domain Specialization*,
  `occult_adventures:class_feature` *Spiritualist ~ Shared Consciousness*,
  `occult_adventures:race_trait` *Emotional Focus / Zeal ~ Tracking*,
  `occult_adventures:race_trait` *Phantom Manifestation ~ Incorporeal*,
  `pathfinder_unchained:race_trait` *Unchained Evolution ~ Climb*,
  `pathfinder_unchained:race_trait` *Unchained Evolution ~ Swim*; and 2 of form `words`, cause
  `rendered-words-disagree` — `core_rulebook:class_feature` *Evocation School ~ Force Missile*,
  `monster_codex:race_trait` *Bat (Sootwing) ~ Paralysis*. All 10 are booked as Epic 6's parity
  baseline (`1788955474431-at-35-e4-002-0f136c`) by cycle 1; this cycle introduces none and
  fixes none.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — `added=0` in the mechanical rows.
  - **relabel (bucket to bucket):** 0 — `relabeled_moves=` empty.
  - **reachability:** 0 — no converter run, no `applies` change.
  - **instrument-correction:** 1, in prose only — the dispatch's "392 units at authoring / cycle
    1" against HEAD's "0 units / cycle 1 already complete"
    (`1789054081017-at-35-e4-002-82fb71`). No instrument's output changed.
- **Refused tokens:** none. `token_coverage.py --check` reports `refused=142 refused_non_done=0`
  — every converter-refused record is DONE, so no refusal holds a unit open, in bucket V or
  anywhere. No `deferral` event is owed: the cycle closed 0 of a scoped population of 0.
- **Discoveries:** none. The atlas, `token-coverage.json` and the parity JSON all reproduce the
  figures cycle 1 recorded; nothing surfaced that they did not predict.
- **Figures + their re-derive commands** (denominator: **49,438 units** = **49,438 corpus records**, all books, unless stated — `python3 -c "import json;print(len(json.load(open('docs/work-inventory.json'))['units']))"`):
  | figure | value | command |
  |---|---|---|
  | population, the denominator of every ratio below | **49,438** units | `python3 -c "import json;print(len(json.load(open('docs/work-inventory.json'))['units']))"` |
  | bucket V | **0** of 49,438 | `python3 scripts/completion_atlas.py --check` |
  | every non-DONE bucket (A B C D M V U X Z) | **0** each; `DONE: 49438` | `python3 scripts/completion_atlas.py --check` |
  | scoped population for this cycle | **0**, `verdict=PASS_WHOLE_REMAINDER` | `python3 scripts/cycle_scope_gate.py --min 500 --bucket V` |
  | whole non-DONE remainder, unscoped | **0**, `verdict=PASS_WHOLE_REMAINDER` | `python3 scripts/cycle_scope_gate.py --min 500` |
  | bucket-V units compared by the harness | **392** | `python3 -c "import json;d=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_bucket-v-parity.json'));print(d['population'],len(d['units']))"` |
  | oracle agree / disagree / unverifiable | **184 / 10 / 198** of 392 | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_bucket-v-parity.json'))['counts'])"` |
  | oracle verdicts by tier | `export` 165/9/112, `source` 19/1/86 | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_bucket-v-parity.json'))['by_tier'])"` |
  | unmapped token types | **0** of 231 | `python3 scripts/token_coverage.py --check` |
  | converter-refused records | **142**, one shape `no_corpus_record` | `python3 scripts/token_coverage.py --check` |
  | refused records that are non-DONE | **0** of 142 | `python3 scripts/token_coverage.py --check` |
  | converter census | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS` | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `data/sheet_rules/` source-format leaks | **0** files | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | PCGen live-side files | **253** (baseline 260) | `python3 scripts/pcgen_residue_gate.py --check` |
  | magnitude-bearing units not held by the engine | **0** of 26,396 | `python3 scripts/shape_engine_boundary.py --check` |
  | missing engine tables | **0** | `python3 scripts/missing_engine_tables.py --check` |
  | denominator gate, package + artifacts | `files_checked=79 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | figure-provenance gate | `files_checked=196 figures_examined=362 violations=0` | `python3 scripts/denominator_gate.py --check-provenance` |
  | dashboard feed pin | matches, `5a0a0787312b…e36f` | `./scripts/publish-site-dashboard.sh --check-pin` |
  | PI sweep | `PASS (11 hits over src/rules_core/rules_tables, 11 baseline rows)` | `scripts/verify.sh --only pi-sweep` |

  Wall times paid this cycle: `sheet_rule_convert -- --check` **113.5 s** (warm target dir);
  every other command above, seconds. No per-unit cost projection was owed — the cycle ran no
  population-scoped pass, because the population is zero.
- **Build scope verified:** `cargo run --locked --bin sheet_rule_convert -- --check` → `EXIT=0`,
  `verdict=PASS`, run at `18dbe0e165` (the tree this cycle commits is docs-only above it). The
  full `cargo test` matrix was **not** re-run and is not owed: this cycle changes no Rust, no
  `Cargo.lock`, no `data/`, no `docs/work-inventory.json`, so the last figure-moving commit is
  `18dbe0e165`'s and its build scope is recorded in `AT-35-E4-001_cycle2_receipt.md`. Desktop
  crate: epic cadence — `apps/` untouched.
- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` was correctly
  not run (`workflow-instruction.md §6` step 3 guards it on corpus change).
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
- **Status:** complete
- **Notes:** A re-dispatch of a criterion that was already `complete`. Nothing was rebuilt and
  the corpus-wide harness was not re-run — the criterion says "once", it ran once, and the
  evaluator it compared is byte-identical at HEAD. What this cycle adds is the re-derivation of
  every Evidence clause at `18dbe0e165` and the correction of the dispatch's stale figures. No
  other criterion's card was emptied by this cycle, so none was closed by it.
- **Next-cycle scope:** criterion at zero — bucket V is 0, the whole non-DONE remainder is 0,
  and AT-35-E4-002's Evidence sentence is paid in full. No next cycle.
