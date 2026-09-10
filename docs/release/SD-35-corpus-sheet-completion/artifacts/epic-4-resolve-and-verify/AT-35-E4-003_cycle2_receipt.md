# Cycle 2 — Epic 4, Resolve and verify / AT-35-E4-003

Dispatched as "cycle 1". AT-35-E4-003 cycle 1 already ran on 2026-09-09 and landed in
`1f4c0ad9fe` (`AT-35-E4-003_cycle1_receipt.md`, `rate-ledger.json` at 3 rows). This cycle is
numbered **2** so that cycle 1's receipt is not overwritten. What was actually outstanding is
what cycle 1 could not have known: **four more Epic 4 cycles landed after it**, so the ledger
carried 3 rows for 6 cycles and its `verified_at.completeness` claim had gone false. This cycle
closes zero units by design and moves no figure.

- **Commit SHA:** `<pinned by the follow-up commit below — the `AT-35-E3-004_cycle2` precedent>`
- **Cycle start SHA:** `5341630c0e5f7141ca84b91ffc13b97cff9920d8`
- **Scope gate:** `SCOPE_GATE: EXEMPT (ledger cycle — closes zero units by design)` — `decisions.md §2`.
  Recorded for completeness, the unflagged gate at the same sha:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The exemption is the one `decisions.md §2` names and this dispatch's scope flags state
  verbatim; the cycle is **not** exempt from the residue check, which ran at start and at end.
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json`
    (3 rows → 7; new `staleness_rule`; `reading_rule` extended; `verified_at` and `totals` re-derived)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-003_cycle2_receipt.md` (this file)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `kanban.md` (§6 step 7)
  - `docs/retro/events/at-35-e4-003.jsonl` (one `correction`, appended)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the atlas
    instrument's own `derived_at` stamp, rewritten by `completion_atlas.py --check`)
  - `docs/retro/events/sd31-transcribe.jsonl` (one append this cycle's own `verify.sh --only
    pi-sweep` emitted under the shared checkout's ambient `RETRO_ACTOR`; folded, not filtered
    away — precedent `c15e64bc3e`)

  No `src/`, no `data/`, no `scripts/`, no `docs/work-inventory.json`. `rust_lines_changed=0`.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS on this cycle's own diff.
  `BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47` (`git merge-base HEAD origin/develop`);
  `git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/pcgen_import/sheet_rule/ data/sheet_rules/ scripts/oracle_harness/ docs/work-inventory.json docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/ ':!**/__tests__/**' ':!**/*.test.*' | grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → **2** before this cycle and **3** after. The two pre-existing are prior receipts' own prose
  quoting the real test-directory paths `tests/sd18_widening/` and `tests/sd13_progression/`,
  already itemised in `EPIC-4_wrapup_correction_cycle_receipt.md`. The delta of **1** is **this
  receipt's own line naming those two paths** — the same recursive artifact every prior receipt in
  this epic produced. Bounded to this cycle alone
  (`git diff --unified=0 5341630c0e -- <the same pathspec>`), the grep returns exactly that one
  line and nothing else. **No identifier in any code, data or instrument file matches.**
- **Wired-integration audit result:** OK_NO_TOKENS on this cycle's own diff.
  Same pathspec, `grep -cE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` →
  **22** before this cycle and **27** after; the delta of **5** is this receipt's own five lines,
  itemised at the end of this bullet. Every one of the 22 is pre-existing and already itemised by
  `AT-35-E4-001_cycle1/2` and `AT-35-E3-002_cycle1`: generated Paizo prose inside
  `data/sheet_rules/` carrying the ordinary English word *hack* (`core_rulebook:spell:plant_growth`;
  `bestiary_3:monster_ability:tophet_swallow_whole`), the bracketed upstream editorial note
  *"[Change to magical beast and stacking restriction not yet implemented]"*, earlier receipts'
  prose quoting this very grep, and **removed** (`-`) `"no selection"` placeholder rows deleted
  from `docs/work-inventory.json`. No shipping-code stub, mock or `"Would …"` string. This cycle's
  own diff (`git diff --unified=0 5341630c0e -- <the same pathspec>`) returns exactly **5** lines
  and nothing else, and all five are **this receipt quoting the pattern in order to name where the
  22 pre-exist**: the audit grep itself, the *hack* in the two `data/sheet_rules/` prose records,
  the bracketed upstream note, and the removed `"no selection"` inventory rows. `rate-ledger.json`
  matches **0**. Nothing was added to a code path, a data record, or an instrument.
- **Acceptance criterion** (verbatim, `epic-breakdown.md § AT-35-E4-003`):
  > ### AT-35-E4-003 — the rate ledger
  >
  > `artifacts/epic-4-resolve-and-verify/rate-ledger.json`, same shape as E3-004.

  and, by reference, `### AT-35-E3-004`:
  > `artifacts/epic-3-place-and-surface/rate-ledger.json`: per cycle — mechanism, scoped
  > population, units closed, units relabeled, wall time, `rust_lines_changed / units_closed`,
  > `builds_recorded` (must be 1), `pcgen_live_files` (must not rise).

  **The bar, field by field, at HEAD.** Every one of the seven `cycles` rows carries all seven
  named fields plus `scope_gate`, `note` and `receipt`; the key set is identical across rows
  (`python3 -c "import json;c=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['cycles'];print(sorted(c[0])==sorted(c[-1]))"`
  → `True`). `rust_lines_changed / units_closed` is carried as `ratio` and is `null`, never `0.0`,
  in all seven rows — a division by zero, since every Epic 4 cycle closed 0 of a 0-unit scoped
  population. `builds_recorded` is 1 in exactly one row (`EPIC-4_wrapup_correction_cycle`) and 0 in
  six; `reading_rule` states why a 0 is the counter's reading and not a missed build. `pcgen_live_files`
  runs 260 → 260 → 260 → 260 → 253 → 253 → 253: it **fell** by seven and never rose, and
  `reading_rule` now records the fall as Epic 6 AT-35-E6-001's so no reader mistakes it for a
  discrepancy between two Epic 4 receipts.
- **Receipt rows (mechanical):**
  ```
  since=5341630c0e5f7141ca84b91ffc13b97cff9920d8 target_dir=/tmp/cargo-sd35-AT-35-E4-003 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=253
  ```
  `closed=0` / `relabeled=0` is the correct and only possible reading: the population was already
  0 non-DONE at the cycle start. `ratio` is `n/a` — a division by zero, never `0.0`.
  `rust_lines_changed=0` because this cycle writes no Rust. `pcgen_live_files=253`, seven **below**
  the 260 baseline (Epic 6's fall, never a rise).
- **PCGen residue:**
  ```
  live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Identical at cycle start and at cycle end, and identical to the previous receipt's line
  (`AT-35-E4-002_cycle2_receipt.md`) — not above it. Nothing on the live side was touched.
- **Oracle parity:** N/A — no `Number` mapping was added, no converter row changed, no live path
  touched. The epic's oracle evidence is unchanged and is carried per row: `AT-35-E4-001_cycle1`
  (146 lines compared / 145 agree / 1 disagree; 382 chassis / 376 / 6 — 7 total, reproduced byte
  for byte at `AT-35-E4-001_cycle2`) and `AT-35-E4-002_cycle1` (392 compared, 184 agree, 10
  disagree, 198 unverifiable), both at
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — `added=0` in the mechanical rows.
  - **relabel (bucket to bucket):** 0 — `relabeled_moves=` empty.
  - **reachability:** unchanged — `records=49438 converted=49296 refused=142` at the cycle end,
    the same figures the cycle started with.
  - **instrument-correction:** one, and it is the reason this cycle exists — cycle 1's
    `verified_at.completeness` claim, corrected under a new `staleness_rule`
    (`1789054612332-at-35-e4-003-555240`). No instrument's own output changed.
- **Refused tokens:** none. `token_coverage.py --check` reports `refused=142 refused_non_done=0`
  and `unmapped_token_types=0` of `token_types=231` — every converter-refused record is DONE, so
  no refusal holds a unit open anywhere. **No `deferral` event is owed:** the cycle closed 0 of a
  scoped population of 0, which is the whole of its scope, not a shortfall against it.
- **Discoveries:** none of the mechanism kind — no token type, kind, or remaining-step category
  that `token-coverage.json` or the atlas did not predict. One instrument-shaped finding, and it
  is about this ledger's own claim rather than the corpus: **a `verified_at.completeness`
  assertion is pinned to one sha and goes false silently the moment the epic takes another
  cycle.** Epic 3's ledger recorded the same failure at its cycle 2
  (`1789047488243-at-35-e3-004-c2-f20947`); Epic 4's ledger has now repeated it, which makes it a
  recurrence rather than an accident (`AGENTS.md` §8 — a warning is not a control). The control
  written here is mechanical, not a caution: `staleness_rule` names the one command that decides
  the question, `ls …/*_receipt.md | wc -l` against `len(cycles)`, so a later reader can falsify
  the claim in one line instead of trusting it.
- **Figures + their re-derive commands:**

  | Figure | Value | Command |
  |---|---|---|
  | committed Epic 4 receipts at cycle start | 6 | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/*_receipt.md \| wc -l` |
  | ledger rows before / after | 3 / 7 | `python3 -c "import json;print(len(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['cycles']))"` (after `git show 5341630c0e:<path>` for the before) |
  | cycle ordering (git, not mtime) | E4-001_c1 09-09 05:59 → E4-002_c1 08:53 → E4-003_c1 10:55 → wrap-up 12:18 → E4-001_c2 09-10 10:07 → E4-002_c2 11:32 | `git log -1 --format='%h %ad' --date=iso -- <receipt>` per receipt |
  | ledger totals (rows, closed, relabeled, rust, builds) | `7 0 0 302 2` | `python3 -c "import json;c=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['cycles'];print(len(c),sum(x['units_closed'] for x in c),sum(x['units_relabeled'] for x in c),sum(x['rust_lines_changed'] for x in c),sum(x['builds_recorded'] for x in c))"` |
  | `rust_lines_changed` for this cycle | 0 | `git diff --numstat 5341630c0e..HEAD -- '*.rs'` → empty |
  | corpus completion | `DONE: 49438`, every other bucket 0 | `python3 scripts/completion_atlas.py --check` |
  | token coverage | `non_done=0 refused=142 refused_non_done=0 token_types=231 verdict=PASS` | `python3 scripts/token_coverage.py --check` |
  | converter reachability | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS` (116.9 s) | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | PCGen live residue | `live_files=253 live_hits=12256` vs baseline `260 / 12736`, `verdict=PASS` | `python3 scripts/pcgen_residue_gate.py --check` |
  | token strings left in `data/sheet_rules/` | 0 files | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0` | `python3 scripts/shape_engine_boundary.py --check` |
  | missing engine tables | `population=0 kinds=0` | `python3 scripts/missing_engine_tables.py --check` |
  | denominator gate | `files_checked=79 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | PI sweep | `PASS — 11 hits over src/rules_core/rules_tables, 11 baseline rows` | `scripts/verify.sh --only pi-sweep` |

  **Denominator for every "0" above** (`python3 scripts/completion_atlas.py --check`)**:** 0 of the **49,438** corpus units the atlas partitions, of which 0 were non-DONE at this cycle's start sha `5341630c0e`.
- **Build scope verified**, run at `5341630c0e` (the tree this cycle commits is docs-only above it):
  - `cargo run --locked --bin sheet_rule_convert -- --check` → `EXIT=0`, `verdict=PASS`.
  - `cargo test --locked --no-run -j 6`, `--lib` and `--no-fail-fast` — **not run, and not owed.**
    `workflow-instruction.md §6` step 3 puts the verification pass after the *last figure-moving
    commit*; this cycle changes no `.rs` file, no `Cargo.lock`, no `data/`, and no
    `docs/work-inventory.json`, so the last figure-moving commit is still `18dbe0e165`'s and its
    build scope is recorded in `AT-35-E4-001_cycle2_receipt.md` (`NO_RUN_EXIT=0`; `--lib` 3,261
    passed). The full workspace ran green at the epic wrap-up
    (`EPIC-4_wrapup_correction_cycle_receipt.md`: 48 of 48 stages, `root-full` 8,730 passed over
    412 suites). Precedent: `AT-35-E4-002_cycle2`, `AT-35-E3-004_cycle2`.
  - `cargo clippy` — not owed; no Rust target changed.
  - Desktop crate and frontend: epic cadence — `apps/` untouched.
- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` was correctly
  not run (`§6` step 3 runs it "only when corpus records changed").
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, quoted from the two
  prior receipts' runs; no figure in this receipt was newly derived from the pinned corpus.
- **Status:** complete
- **Notes:** The dispatch's cycle number (1) and the criterion's real state disagreed; the receipt
  is numbered 2 rather than overwriting a committed one. One self-caught operator error, worth a
  line because it is a shared-checkout trap: the shell here carries an ambient
  `RETRO_ACTOR=sd31-transcribe`, so a `retro.py` call without an inline `RETRO_ACTOR=` files the
  event into another lane's log. The first `correction` landed in `sd31-transcribe.jsonl`, was
  reverted with a single-path `git checkout` after confirming the diff was exactly `1 0`, and
  re-emitted into `at-35-e4-003.jsonl`. The same ambient actor put this cycle's `verify.sh` event
  in that log a second time; that one is genuine output and was folded, not filtered.
- **Next-cycle scope:** criterion at zero. The ledger has one row per committed Epic 4 receipt,
  the totals re-sum from the rows, and `staleness_rule` names the command that re-decides
  completeness at any later sha. No further cycle is owed unless Epic 4 takes another one — in
  which case this criterion owes a row, not a re-derivation.
