# Cycle EPIC-3-WRAPUP-FIX — Epic 3, Place and surface / wrap-up correction cycle

- **Commit SHA:** `2dc322ae32` — the single work commit carrying every change this cycle made
  (`git show --stat 2dc322ae32`: 44 files). This line was pinned by the follow-up docs commit, per
  `workflow-instruction.md §6`.
- **Cycle start SHA:** `e91b1d8873` — the tip of `origin/tranche/15` when this cycle began, **14
  commits above `07e29075b4`, the HEAD the RED wrap-up gate ran at.** The tree moved between the
  two runs (AT-35-E3-004 cycle 1 and AT-35-E4-001 cycle 1 landed), and that movement changed one
  of the gate's two findings — see **Discoveries**.
- **Scope gate:** `SCOPE_GATE: EXEMPT (wrap-up correction cycle)` — `decisions.md §2` and `§9` L6:
  a wrap-up fix cycle closes zero units by design. **Not** exempt from the residue check, which was
  run at start and at end.
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md` (8 figure lines)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/AT-35-E3-003_cycle1_receipt.md` (6 figure lines)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-001_cycle1_receipt.md` (2 figure lines — not in the gate's list; see **Discoveries**)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/EPIC-3_wrapup_gate_report.md` (committed from the gate worker's worktree; 3 denominator lines fixed)
  - `docs/retro/events/at-35-e3-wrapup.jsonl`, `docs/retro/events/epic-3-wrapup-gate.jsonl` (committed from the gate worker's worktree, unmodified)
  - `docs/retro/events/at-35-e3-wrapup-fix.jsonl` (this cycle's 6 events: 3 `correction`, 2 `resolution`, 1 `verification` — `wc -l < docs/retro/events/at-35-e3-wrapup-fix.jsonl`)
  - `scripts/verify-baselines.env` (`BASELINE_ROOT_FULL_TESTS` 8724 → 8727, with its derived attribution)
  - `site/dashboard/PF1e-dashboard.json`, `site/dashboard/PF1e-dashboard.json.last-good`,
    `site/dashboard/units/*.json` (8 files + `index.json`), `site/status-data.json`,
    `site/status-data/*.json` (20 books) — all regenerated, none hand-edited
  - `docs/release/SD-35-corpus-sheet-completion/kanban.md` (row 32), `progress.md` (this entry)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS — no live-side code was touched at all. The only
  non-doc, non-generated file is `scripts/verify-baselines.env`, a tool-side baseline record.
- **Acceptance criterion:** not a criterion. `workflow-instruction.md §10` step 0: the epic's full
  `scripts/verify.sh` gate came back RED, and the correction cycle drives it green before the next
  epic's second cycle dispatches.

## The gate worker's two red stages

### 1. `site-dashboard-check` — RED, now PASS

Reproduced first, at my own HEAD, before touching anything:

```
./scripts/publish-site-dashboard.sh --check
  -> site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh   (exit 1)
```

The gate's attribution is correct: AT-35-E3-001 c2 and AT-35-E3-002 c1 regenerated
`docs/work-inventory.json` (corpus to DONE 49,438 of 49,438) and never republished the dashboard
that reads it. Fixed by running the producer, not by touching the assertion:

```
./scripts/publish-site-dashboard.sh          # 75.2 s real
  -> wrote /home/ubuntu/workspace/repos/codex/site/dashboard/PF1e-dashboard.json
  -> Wrote site/status-data.json (30 books, overall 95.0%) and 30 book-detail files
     under site/status-data (46,074 items total)

./scripts/publish-site-dashboard.sh --check
  -> site/dashboard/PF1e-dashboard.json is current
  -> OK: status-data.json and status-data/*.json are up to date        (exit 0)
```

33 generated `site/` files changed. None was hand-edited.

### 2. `figure-provenance` — RED, now PASS

Reproduced, and it disagreed with the report:

```
python3 scripts/denominator_gate.py --check-provenance
  -> files_checked=169 figures_examined=217 violations=16     (gate worker reported 14)
```

All 16 were genuine `[unsourced]` — a figure inside a `Figures + their re-derive commands`
section on a line carrying no re-derive command. The gate's own rule
(`scripts/denominator_gate.py`, `_line_has_reachable_command`) scopes the obligation to the line,
so a figure that wrapped onto a continuation line while its command stayed on the line above is a
real violation of `AGENTS.md` rule 9 as this package enforces it, not a false positive.

Fixed by rewriting all 16 lines so each figure carries a runnable re-derive command **inline on
its own line**. No ignore list was widened, no line was deleted, no figure was changed, and the
stage's own logic was not touched. Where a wrapped bullet stated two figures from two different
commands, the bullet was re-flowed so each figure sits beside the command that produced it.

```
python3 scripts/denominator_gate.py --check-provenance
  -> files_checked=171 figures_examined=224 violations=0      (exit 0)
```

### A third stage went red *because of* this cycle, and was fixed

Committing the gate worker's own report moved `denominator-gate` from `files_checked=239
violations=0` to `files_checked=241 violations=3`: three bare percentages in
`EPIC-3_wrapup_gate_report.md` stated no denominator (`62% used`, `fail rate 11.43%`, `67%`). The
wrap-up report is itself gated content. Each now names its denominator (the 1.5T root filesystem;
16 failed runs **of 140**):

```
python3 scripts/denominator_gate.py --check
  -> files_checked=241 violations=0                            (exit 0)
```

### The baseline note (not a failure), and a correction to its stated cause

`check_floor` (`scripts/verify.sh:222`) is a **floor**: `8727 measured > 8724 recorded` is a
`note`, never a `FAIL`. It is updated here deliberately, in the same commit, as the gate worker
asked — and to the value **this green run** measured, per the rule the Epic 2 entry in
`scripts/verify-baselines.env` already sets.

The gate worker's stated cause is wrong. It credited all +3 to "AT-35-E3-003 c1's control tests".
Derived instead of assumed:

```
for c in $(git rev-list --reverse e0280a8fea..e91b1d8873); do \
  git show $c -- '*.rs' | grep -c '^+\s*#\[test\]'; done
```

  - `26bdfa8d5b` +1 — **AT-35-E3-002 c1**, "the whole remainder to DONE"
  - `5a361c9dc4` +1 — **AT-35-E3-002 c1**, sheet-rule rung ladder-position split
  - `0e0298d7fe` +1 — **AT-35-E3-003 c1**, "size" pinned out of the class-feature magnitude suffixes

Only one of the three is AT-35-E3-003's. `AT-35-E4-001` (`9bae2cfa1f`) added none, which is why
both the RED gate's run and this green one measured 8727 across a tree that moved 14 commits.
Correction event `1788947761532-at-35-e3-wrapup-fix-d04a67`. The derivation, and this command, are
written into `scripts/verify-baselines.env` beside the new value.

## The gate worker's uncommitted artifacts — found and committed

All three were located in the gate worker's worktree
`/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-21` (`git -C <wt> status
--porcelain` → three `??` entries, exactly the three named), content verified before copying:

| File | Verified as | Committed to |
|---|---|---|
| `EPIC-3_wrapup_gate_report.md` | 19,352 bytes, header states worker/worktree/HEAD `07e29075b4`, verdict RED | `artifacts/epic-3-place-and-surface/` |
| `at-35-e3-wrapup.jsonl` | 5 events, all `actor=AT-35-E3-WRAPUP`, parsed as JSON: 1 `correction` (`wrong-base-worktree`), 3 `incident`, 1 `deferral` | `docs/retro/events/` |
| `epic-3-wrapup-gate.jsonl` | 1 `verification` event, `actor=epic-3-wrapup-gate` — `verify.sh`'s own | `docs/retro/events/` |

Nothing was invented or substituted. The two `.jsonl` files are committed byte-for-byte as the
gate worker wrote them; only the report was edited, and only for the three denominator lines.

## Movement, four buckets

- **closure:** none — zero units, by design (`decisions.md §9` L6).
- **relabel:** none.
- **reachability:** none.
- **instrument-correction:** three. (a) `BASELINE_ROOT_FULL_TESTS` 8724 → 8727 with a derived,
  corrected attribution. (b) the site dashboard and public-status projections re-derived from the
  live inventory. (c) 16 receipt figure lines re-sourced so the provenance stage measures what it
  claims to.

## Refused tokens

none — this cycle converted nothing.

## Discoveries

1. **The gate report's `figure-provenance` count was stale at my HEAD: 14 claimed, 16 actual.** The
   extra two are in `AT-35-E4-001_cycle1_receipt.md` (lines 123 and 144), a lane that landed
   *after* the gate ran. The report's claim that "ALL 14 are in two Epic 3 receipts and no other
   file" was true at `07e29075b4` and false at `e91b1d8873`. Fixing only the 14 it named would have
   left the stage red. Correction `1788942545792-at-35-e3-wrapup-fix-71b095`.
2. **Committing a wrap-up gate report is not inert — the report is gated content.** It turned
   `denominator-gate` red on its own three percentages. Correction
   `1788942545933-at-35-e3-wrapup-fix-b6ddea`. Any future wrap-up gate worker should run
   `denominator_gate.py --check` against its *own* report before handing it over.
3. **The gate report's +3 test attribution named the wrong cycle** — see above.

## Figures + their re-derive commands

- **48 of 48 stages PASS**, `RESULT: PASS` — `scripts/verify.sh -j 6` at `e91b1d8873`, console
  transcript `/tmp/e3fix_verify.out`, logs `/tmp/codex-verify-5XWQBR`; tallied by
  `grep -cE '^    PASS' /tmp/e3fix_verify.out` → 48 and `grep -cE '^    FAIL' /tmp/e3fix_verify.out` → 0
- **wall time 5,184 s = 86 min 24 s** — `date +%s` either side, `echo $(( $(date +%s) - $(cat /tmp/e3fix_start) ))`
- **`figure-provenance` 16 → 0 violations of 224 figures examined in 171 files** — `python3 scripts/denominator_gate.py --check-provenance`
- **`denominator-gate` 0 violations of 241 files checked** — `python3 scripts/denominator_gate.py --check`
- **`root-full` 8,727 passed across 411 suites, all 361 `tests/*.rs` executed, 0 failed** — `grep -E 'PASS  root-full' /tmp/e3fix_verify.out`
- **`root-lib` 3,220 passed / `desktop` 574 / `reach` 32 / `frontend-test` 101 of 101 / `clippy` root 0 and desktop 0** — `grep -E '^    PASS' /tmp/e3fix_verify.out`
- **`token-coverage` `non_done=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS`** — `grep -E 'PASS  token-coverage ' /tmp/e3fix_verify.out`
- **`class-dump` 31 of 31 computing**, **`sheet-rules-check`** and **`corpus-sweep` 0 findings** — `grep -E '^    PASS' /tmp/e3fix_verify.out`
- **site producer output: 30 books, overall 95.0% of the 46,074 items projected, 30 book-detail files** — `./scripts/publish-site-dashboard.sh` (its own final line)
- **disk 62% used of the 1.5T root filesystem at start, 64% at peak, 525G free** — `df -h /`

## PCGen residue

Run at start and at end, per this cycle's own non-exemption. **Unchanged, and it could not have
moved: no live-side file was touched.**

```
live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
```

`python3 scripts/pcgen_residue_gate.py --check`, exit 0 — identical at cycle start and cycle end,
and identical to the AT-35-E1-005 baseline and to every Epic 2/3 receipt's `pcgen_live_files` row.
Nothing new on the live side (`decisions.md §11`); the converter, parser, generators and oracle
harness are all untouched and kept.

## Oracle parity

N/A — no Number mapping was added and no Epic 6 live path was touched. The `preflight-oracle` stage
passed at pin `7f818006e371188e5717fd18d74d18a420747fc6`.

## Build scope verified

`scripts/verify.sh -j 6`, every stage, no `--only`, ONE pass after all edits were in place
(`decisions.md §3`, and the batch-big/verify-once ruling). Run at `e91b1d8873` with
`CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E3-WRAPUP-FIX`, `CARGO_INCREMENTAL=0`. Widest scope the
repo has: root workspace (`root-lib`, `root-full`) **and** the separate `apps/desktop/src-tauri`
crate (`desktop`), plus `frontend-*` and `clippy` over both crates. `RESULT: PASS`.

## Sweep population

`corpus-sweep` ran as a `verify.sh` stage and reported **0 findings**. No separate
`corpus_literal_sweep` population was moved by this cycle.

## Oracle pin

`7f818006e371188e5717fd18d74d18a420747fc6` (`preflight-oracle`, PASS). No figure in this receipt
came from the pinned corpus.

## Status

**complete.**

## Notes

The one judgment call: two of the 16 provenance violations live in `AT-35-E4-001_cycle1_receipt.md`,
another criterion's receipt. They were fixed here rather than left for Epic 4, because leaving them
leaves this cycle's own gate red, and the alternative — narrowing the stage to skip that file —
is exactly the hiding this cycle was told not to do. The edits add re-derive commands and change no
figure and no claim.

## Next-cycle scope

Criterion at zero — this is not a criterion. Epic 3's four criteria are all `complete` and its
population is zero at this HEAD (`missing-engine-tables` population=0, `token-coverage`
`non_done=0 refused_non_done=0`, `completion_atlas.py --check` DONE 49,438 of 49,438). Epic 3's
wrap-up is closed. `workflow-instruction.md §10` step 3 opens no PR; the operator merges
`tranche/15` → `develop` at bundle closure.

## Carried forward, unresolved by this cycle

Two items from the gate worker's report are **not** this cycle's to close, and are named rather
than silently dropped:

1. **Three Epic 3 worktrees remain on disk** (`wf_291be5c8-5f3-14`, `-15`, `-17`). All nine agent
   worktrees are proven merged (`git merge-base --is-ancestor <sha> origin/tranche/15` → exit 0 for
   every one), so nothing is at risk; the gate worker's deferral names the orchestrator as owner,
   and the harness refuses a `git worktree remove` on a sibling path from inside a dispatched
   agent. Re-derive: `git worktree list`.
2. **`duplicate-criterion-dispatch` stands at 2 fires** and caused both of the window's `rework`
   events. Below the 3-fire threshold, escalated by name in the gate report. A third fire makes it
   a missing mechanism under `AGENTS.md` rule 8; the control would be a pre-dispatch reconciliation
   of the orchestrator's loop against `kanban.md`. Owner: the orchestrator.
