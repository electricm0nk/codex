# Cycle EPIC-4-WRAPUP-CORRECTION — Resolve and verify / epic wrap-up correction (workflow-instruction.md §10 step 0)

- **Commit SHA:** work `1f4c0ad9fe` (the three fixes, the gate worker's report, all four retro-event files); docs `__DOCS_SHA__` (this receipt, `progress.md`, `kanban.md`). The full gate was run on the working tree of `1f4c0ad9fe`; the docs commit changes no gated input.
- **Scope gate:** `SCOPE_GATE: EXEMPT (wrap-up correction cycle)` — `decisions.md §2`. A wrap-up
  fix cycle closes zero units by design; the batch floor does not apply. **Not** exempt from the
  residue check, which ran at start and at end (below).
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-003_cycle1_receipt.md` (the red stage's two violations)
  - `docs/release/SD-35-corpus-sheet-completion/workflow-instruction.md` (§6 step 3 — the mechanical control)
  - `scripts/verify-baselines.env` (`BASELINE_ROOT_TEST_BINARIES` 411 → 412, dated note)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/EPIC-4_wrapup_gate_report.md` (**new** — the gate worker's, committed here)
  - `docs/retro/events/at-35-e4-wrapup.jsonl` (**new** — the gate worker's 3 events, committed here)
  - `docs/retro/events/epic4-wrapup-gate.jsonl` (**new** — `verify.sh`'s own event for the gate run)
  - `docs/retro/events/at-35-e4-wrapup-fix.jsonl` (**new** — this cycle's 1 correction + 1 note)
  - this receipt; `progress.md`, `kanban.md`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0 HEAD -- docs/release/SD-35-corpus-sheet-completion scripts/verify-baselines.env docs/retro/events ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match on this cycle's own diff. (The same grep over the whole `fe5ae6cd4a...HEAD` range returns pre-existing hits from AT-35-E1-003 and AT-35-E2-002, already itemised in their receipts: `tests/sd18_widening/` and `tests/sd13_progression/` directory names, which are real paths, not bundle tags.)
- **Wired-integration audit result:** `OK_NO_TOKENS` — same pathspec, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`. Run with `git add -N` applied to this cycle's four new files first, so the diff was not empty (the trap AT-35-E4-003 recorded as correction `1788959112531-at-35-e4-003-d78240`).
- **Acceptance criterion:** this cycle has no criterion card of its own. Its bar is
  `workflow-instruction.md §10` step 0: *the epic's full `scripts/verify.sh` gate is green, and
  it must land before Epic 5's second cycle dispatches.*
- **Receipt rows (mechanical):** `closed=0 relabeled=0 rust_lines_changed=0 ratio=null builds_recorded=1 pcgen_live_files=260`.
  `ratio` is **null**, not `0.0`: 0 Rust lines over 0 units closed is a division by zero. This
  cycle changed **no** `.rs` file — `git diff --stat HEAD -- '*.rs'` is empty; the only
  non-`docs/` file touched is `scripts/verify-baselines.env`, which is a shell env file.
- **PCGen residue:** unchanged, start and end —
  `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`
  (`python3 scripts/pcgen_residue_gate.py --check`, run before the first edit and after the last).
  It did not rise. This cycle touches no live path.
- **Oracle parity:** N/A — no `Number` mapping added, no live path touched.
- **Movement, four buckets:** closure **0** · relabel **0** · reachability **0** ·
  instrument-correction **3** — (1) the `figure-provenance` violations in
  `AT-35-E4-003_cycle1_receipt.md`, (2) the per-cycle gate block that could never catch them,
  (3) the stale `BASELINE_ROOT_TEST_BINARIES` floor.
- **Refused tokens:** none. This cycle converts nothing and refuses nothing, so it emits **no
  `deferral`** of its own. The **15 open deferrals** in the window are Epic 1–3 dispositions and
  remain open; `§11` step 1 forbids closing over them, and this cycle does not.
- **Discoveries:** one, and it is about the gate, not the corpus. **`denominator_gate.py
  --check-provenance` verifies that a re-derive command is _present_ and that its script path
  _resolves_ — never that the command _runs_.** Two of the three commands I first wrote to clear
  the red stage made the stage green (`violations=0`) while erroring on execution: the
  work-inventory row read a nonexistent unit field (`x['state']` → `KeyError`; units carry
  `status` / `wiring_class`, and `DONE` is a `completion_atlas` bucket, not a unit field), and the
  population row's regex dropped the `V` in `4,334 + V 392` (→ `AttributeError` on `None.group`).
  A green `figure-provenance` can therefore sit on top of broken provenance. Caught by executing
  each command instead of trusting the gate; the third row, which I had invented and could not
  source honestly, was **dropped** rather than guessed at. Emitted as a `correction` retro event
  (`1788965350822-at-35-e4-wrapup-fix-3de362`) and written into `§6` step 3 as a standing
  instruction, per `AGENTS.md` rule 8 — a caution in a dispatch prompt is not a control.

- **Second discovery — the gate worker's own report would have turned a stage red.** Its
  SELF-CHECK line reads: *"I ran `--check-provenance` over the default paths WITH my report
  present — `files_checked` 174 → 175, violations still 2. My report adds no violation."* That is
  true, and incomplete: it never ran the **other** flag over its own report. The report carried
  **2 violations under `denominator_gate.py --check`** — the separate `denominator-gate` stage —
  at line 44 (`reachable ceiling 100.00%`, no denominator on the line) and line 224 (the `df -h /`
  percentages). Committing it unedited would have turned `denominator-gate` red immediately after
  this cycle declared the epic green. **This is the very two-flag confusion the worker diagnosed
  as `figure-provenance`'s root cause, reproduced one level up in its own self-check** — which is
  the strongest argument for the `§6` step 3 control landing here. Fixed by supplying real
  denominators, not by rephrasing around the gate: `100.00% (49,438 of 49,438)` re-derived with
  `python3 scripts/reachability_audit.py`, and the `df` readings stated against the 1.5 T root
  filesystem (`df -h /`) and explicitly marked **estimates** per `AGENTS.md` rule 9, since a
  point-in-time sample of a shared box is not reproducible after the fact. Correction
  `1788965662874-at-35-e4-wrapup-fix-88e6b2`. *(Note I also got this wrong once en route: I first
  wrote the filesystem total as "1,465 G" from memory rather than measurement; `df -h /` says
  `1.5T`, and the figure was corrected before commit.)*

## What was fixed, and why each fix is a re-pin rather than a cover-up

**1. The red stage — `figure-provenance`, `violations=2`.** Both violations were one two-line
sentence in `AT-35-E4-003_cycle1_receipt.md` (lines 129/131) introducing that receipt's figure
table. It carried three inline figures — `0 units non-DONE`, `49,438`, `4,726` — sourced by a
cross-reference ("see the ledger's `totals.denominator`") instead of a same-line command.

The fix **moves those figures into the table as rows, each with the command that produces it**,
and leaves the header sentence figure-free. It does **not** add the line to an ignore list, does
not narrow the gate's globs, and does not delete the figures. Each of the three commands was
executed and prints the stated value:

| figure | value | command as run | printed |
|---|---|---|---|
| scoped denominator | 0 units non-DONE of 49,438 | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['totals']['denominator'])"` | the denominator string, which states both numbers |
| corpus total at pin `07e29075b4` | 49,438 | `git show 07e29075b4:docs/work-inventory.json > /tmp/wi-e4start.json && python3 -c "import json;print(json.load(open('/tmp/wi-e4start.json'))['totals']['units'])"` | `49438` |
| Epic 4 authoring-time population | 4,726 | `python3 -c "import json,re;d=json.load(open('…/rate-ledger.json'))['totals']['denominator'];print(re.search(r'4,334 . V 392 = ([0-9,]+)',d).group(1))"` | `4,726` |

**2. The control (`AGENTS.md` rule 8).** Incident key
`figure-provenance-command-on-next-line` has now fired **3 times** — the Epic 2, Epic 3 and
Epic 4 wrap-ups — and the gate worker found the root cause, which is **not** authoring
sloppiness: `workflow-instruction.md §6` step 3's per-cycle gate block ran
`denominator_gate.py --check`, while `verify.sh`'s `figure-provenance` stage runs the **different
flag** `--check-provenance`. No cycle had ever run `--check-provenance` locally, so every instance
of the shape escaped the cycle and surfaced only on the ~90-minute wrap-up gate.

I verified the claim against the repo before acting on it: the two flags are distinct
`argparse` options and drive different functions (`find_provenance_violations`, which scopes to
lines *inside* a "Figures + their re-derive commands" section, versus `find_violations`).

`§6` step 3 now runs **both**, with the second annotated as a different flag whose nonzero exit
blocks the push, plus the standing instruction from **Discoveries** that the author must execute
each figure command because the gate will not. Python only, no build, seconds per cycle.

**3. The stale baseline — a floor RAISED, not lowered.** `BASELINE_ROOT_TEST_BINARIES` recorded
`411`; the gate measured `412`. I did not copy the measured number in blind. The attribution:

```
git log --diff-filter=A --name-only --format='%h %s' e0280a8fea..HEAD -- 'src/bin/*.rs' 'tests/*.rs'
# -> 2645a3c85a  AT-35-E4-002 cycle 1
#    src/bin/sheet_rule_bucket_v_render.rs
```

`e0280a8fea` is the commit that set the `411` floor. **Exactly one** new bin-or-test file exists
in that range, so it is the only possible source of the `+1` — this is an exhaustive check over
the range, not a plausible-looking match. The file contains **zero** `#[test]` functions
(`grep -c '#\[test\]' src/bin/sheet_rule_bucket_v_render.rs` → `0`), which is precisely why it
moves `BASELINE_ROOT_TEST_BINARIES` (+1) while leaving `BASELINE_ROOT_FULL_TESTS` alone:
`cargo test` builds a test harness for **every** bin target, so a new bin emits one further
`Running` line ("running 0 tests") and contributes no passing test.

This is a **floor raise** (`check_floor` asserts measured ≥ baseline), so it tightens the gate
rather than hiding a regression. The `AT-35-E5-001` note immediately above it in
`verify-baselines.env` had deliberately left the `+1` unraised because that cycle could not
attribute it and declined to take credit for a suite it did not add; this cycle can attribute it,
so it is banked here with the evidence.

- **Figures + their re-derive commands.** Every number in this receipt, with the command that
  produces it:

  | figure | value | command |
  |---|---|---|
  | `figure-provenance` violations at gate time | **2** of `figures_examined=228`, `files_checked=174` | `EPIC-4_wrapup_gate_report.md`, stage table; re-derive on the pre-fix tree with `git stash`-free checkout: `git show 5e2c0c8c5b:docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-003_cycle1_receipt.md \| sed -n '129p;131p'` |
  | `figure-provenance` violations after the fix | **0** of `figures_examined=230`, `files_checked=177` | `python3 scripts/denominator_gate.py --check-provenance` |
  | `denominator_gate.py --check` over the package | **0** of `files_checked=59` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | commands I added that were present-and-resolvable but did **not** run | **2 of 3** | executed each row's command literally; see **Discoveries** |
  | new bin/test files since the `411` floor commit `e0280a8fea` | **1** (`src/bin/sheet_rule_bucket_v_render.rs`) | `git log --diff-filter=A --name-only --format='%h %s' e0280a8fea..HEAD -- 'src/bin/*.rs' 'tests/*.rs'` |
  | `#[test]` fns in that file | **0** | `grep -c '#\[test\]' src/bin/sheet_rule_bucket_v_render.rs` |
  | `BASELINE_ROOT_TEST_BINARIES` | **411 → 412** | `grep -n '^BASELINE_ROOT_TEST_BINARIES' scripts/verify-baselines.env \| tail -1` |
  | PCGen live-side files, start and end | **260** (baseline 260) | `python3 scripts/pcgen_residue_gate.py --check` |
  | Rust lines changed by this cycle | **0** | `git diff --stat HEAD -- '*.rs'` (empty) |
  | gate-worker artifacts committed here | **3** (+1 of this cycle's own) | `git show --stat 1f4c0ad9fe -- docs/retro/events docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/EPIC-4_wrapup_gate_report.md` |
  | open deferrals in the retro window, unchanged by this cycle | **15** | `python3 scripts/retro.py summary --since 2026-09-07T21:24:13-04:00` |

- **Build scope verified:** `full `scripts/verify.sh -j 3`, **every stage, no `--only`**, run on the shared checkout at
  the working tree of `1f4c0ad9fe` with `CARGO_TARGET_DIR=/tmp/cargo-sd35-at-35-e4-wrapup-fix`,
  `CARGO_INCREMENTAL=0`. **`RESULT: PASS` — 48 of 48 stages passed, 0 failed**, wall time
  **5,288 s = 88 m 8 s** (10:49:15 → 12:17:23 -04:00). Run log `/tmp/sd35-e4-fix-verify.log`;
  stage logs `/tmp/codex-verify-ChslDH`. The stages this cycle is about:
  `denominator-gate` **PASS** (`files_checked=248 violations=0`) and
  `figure-provenance` **PASS** (`files_checked=178 figures_examined=230 violations=0`) — the red
  stage is green, and it is green because the figures are sourced, not because anything was
  silenced. Load-bearing rest: `root-full` 8,730 passed across **412 suites** (all 361 `tests/*.rs`
  executed), `root-lib` 3,220, `desktop` 574, `clippy` root:0 desktop:0, `corpus-sweep` 0 findings
  of 48,706 records examined, `sheet-rules-check` `records=49438 converted=49296 refused=142
  verdict=PASS`, `token-coverage` `verdict=PASS`, `pcgen-residue-gate` `verdict=PASS`,
  `reachability-audit` ceiling 100.00%, `frontend` 101/101 + `tsc` clean, `class-dump` 31/31.
  **`root-full` measured exactly 412 suites against the newly-recorded 412 floor, and the run
  emitted no stale-baseline warning** — the `BASELINE_ROOT_TEST_BINARIES` bump is confirmed by the
  same instrument that reported it stale.`
- **Sweep population:** N/A — no corpus record changed.
- **Oracle pin:** N/A — no figure in this receipt came from the pinned corpus.
- **Status:** `**complete** — the epic's full gate is green (48 of 48), all three named fixes landed, the residue gate did not rise (`live_files=260` start and end), and `§10` step 0 is satisfied, so Epic 5's second cycle is unblocked.`
- **Notes:** The gate worker pushed nothing, so its report and both retro-event files existed only
  as untracked files in worktree `wf_291be5c8-5f3-27`. All three were located, their content read
  and checked against the report's own claims, and committed here — the retro log is append-only
  and survives the run only if someone commits it. The worktree sweep is **not** this cycle's
  step: `§10` step 2 is the gate worker's, and it recorded 0 Epic 4 worktrees found and 0 removed;
  the 10 remaining `wf_291be5c8-5f3-*` trees are Epic 1–3 leftovers due at the `§11` step 3
  bundle-closure sweep.
- **Next-cycle scope:** Epic 4 is green and closed; Epic 5's second cycle is unblocked
  (`§10` step 0 satisfied). Epic 5 rows 22 (`bucket-x-choice-filter`) and 23
  (`corpus-49438-of-49438`) remain `in-progress` and are untouched by this cycle.
