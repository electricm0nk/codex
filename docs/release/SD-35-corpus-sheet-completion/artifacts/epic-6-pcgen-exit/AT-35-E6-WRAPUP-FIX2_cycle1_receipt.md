# Cycle AT-35-E6-WRAPUP-FIX2 cycle 1 — Epic 6, PCGen exit / wrap-up correction cycle (second)

Runs `workflow-instruction.md §10 step 0` for the **second** time on this epic. The first
correction cycle (`AT-35-E6-WRAPUP-FIX`) cleared four red stages; the re-gate that followed it
(`EPIC-6_wrapup_gate_report_regate5.md`, 7,937 s, every stage, no `--only`) came back with **one**
red, `shape-engine-boundary-selftest`. This cycle fixes it. Like its predecessor it runs **LOCAL
on the shared checkout and does commit and push**, unlike the isolated gate worker, which pushed
nothing.

- **Commit SHA:** `<COMMIT_SHA>`
- **Cycle start SHA:** `4b69eb7aab` (`git log --oneline -1`; identical to `origin/tranche/15` at
  cycle start, which is the tree re-gate 5 tested)
- **Scope gate:** `SCOPE_GATE: EXEMPT (wrap-up correction cycle)` — `decisions.md §2` /
  `workflow-instruction.md §6 step 1`: a wrap-up fix cycle closes zero units by design. **Not**
  exempt from the residue check, which ran at start and at end (below).
- **Files touched:**
  - `scripts/tests/test_shape_engine_boundary.py` — **the fix.** The stale equality pin re-stated
    on the property (below).
  - `scripts/verify-baselines.env` — `BASELINE_ROOT_FULL_TESTS` **8919 → 8926**, raised to this
    run's own measured figure with attribution in the file's log. `BASELINE_ROOT_LIB_TESTS` and
    `BASELINE_ROOT_TEST_BINARIES` were **not** moved: this run measured them exactly at their
    recorded floors (3390, 419).
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_gate_report_regate5.md` (**new** — folded, the live report)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_gate_report_regate4.md` (**new** — folded, + SUPERSEDED banner)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_regate2_report.md` (**new** — folded, + SUPERSEDED banner; missed by the first correction cycle)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_regate1_report.md` (**new** — folded, + SUPERSEDED banner; missed by the first correction cycle)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_gate_report.md` (+ SUPERSEDED banner)
  - `docs/retro/events/sd35-e6-wrapup-gate5.jsonl`, `sd35-e6-wrapup-gate4.jsonl`,
    `at-35-e6-wrapup-regate3.jsonl`, `sd35-e6-wrapup.jsonl` (**new** — folded from `-94`)
  - `docs/retro/events/at-35-e6-wrapup-regate-2.jsonl`, `epic-6-wrapup.jsonl` (**new** — folded
    from `-90`), `at-35-e6-wrapup-regate-1.jsonl` (**new** — folded from `-91`)
  - `docs/retro/events/at-35-e6-wrapup-fix2.jsonl` (**new** — this cycle's own events)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-WRAPUP-FIX2_cycle1_receipt.md` (this file), `kanban.md`, `progress.md`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — on this cycle's own diff. Re-derive:
  `git diff --unified=0 4b69eb7aab -- scripts/tests/test_shape_engine_boundary.py scripts/verify-baselines.env | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — no output from the
  test file; `verify-baselines.env`'s hits are pre-existing `tests/sd*_*.rs` filenames inside its
  own historical log, none of them this cycle's.
- **Wired-integration audit result:** `OK_NO_TOKENS`. No shipping code was touched at all — the
  one code-adjacent edit is a Python selftest assertion; `git diff --stat 4b69eb7aab -- '*.rs'` is
  empty.
- **Acceptance criterion:** not a criterion cycle. `workflow-instruction.md §10 step 0`: *"Any red
  stage is fixed in a wrap-up correction cycle before the next epic's second cycle dispatches; the
  fix cycle is exempt from the batch floor, never from the residue check."*
- **Receipt rows (mechanical):**
  `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=0`
  (`python3 scripts/cycle_scope_gate.py --receipt --since 4b69eb7aab --before /tmp/claude-1000/e6fix/wi-before.json --after docs/work-inventory.json`).
  `ratio=n/a` because `closed=0` by design; `rust_lines_changed=0` because no Rust was touched.
- **PCGen residue:**
  - at start: `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`
  - at end: `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`
  - **unchanged — did not rise.** (`python3 scripts/pcgen_residue_gate.py --check`, tail line)
- **Oracle parity:** N/A — no `Number` mapping added, no live rules path touched.
- **Movement, four buckets:** closure **0** / relabel **0** / reachability **0** /
  instrument-correction **2** (the `shape-engine-boundary-selftest` pin, and the
  `BASELINE_ROOT_FULL_TESTS` floor). Nothing about the corpus moved.
- **Refused tokens:** none — this cycle converted no records. Separately, the NO-CARVE-OUTS ruling
  is satisfied at this HEAD: stage `token-coverage` reads
  `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS`
  — every one of the 142 refused token types belongs to a unit already DONE under the sheet rule.
  142 is a number, not an exemption.
- **Discoveries:** one, and it is the same process discovery the first correction cycle recorded,
  **firing again** — see **Worktree sweep** below. Emitted as incident
  `1789443362836-at-35-e6-wrapup-fix2-8bb2d8`, recurrence key
  `unfolded-gate-artifacts-die-with-the-worktree`, and as correction
  `1789443336590-at-35-e6-wrapup-fix2-32faa2`. No new refused token type, kind, or remaining-step
  category surfaced.

## The one red stage, and how it was fixed on the property

`scripts/tests/test_shape_engine_boundary.py:121` asserted
`self.assertEqual(len(mag), 26396)` and failed `AssertionError: 26397 != 26396`. **Not a
behavioural regression** — the very next stage, `shape-engine-boundary`, was already PASS at
`magnitude_bearing=26397 not_held_by_engine=0 citation_ok=True`. Epic 6 added one
magnitude-bearing unit and the hand-maintained equality went stale.

**Copying `26397` into the assertion was refused.** That is what put `26396` there, and before it
`9475`, which the file's own comment records as having been carried stale for six SD-34 waves. A
recurring failure gets a mechanism, not another edit (`AGENTS.md` rule 8). The assertion is now
two halves that together still fail **closed**:

1. **A floor.** `self.assertGreaterEqual(len(mag), 26396)`. This population only grows — a unit
   gains magnitude tokens at ingest and never loses them. Below the SD-34 wave-51 high-water mark
   means units *lost* their magnitude, or the instrument started under-counting. This is the same
   shape `verify.sh`'s own `check_floor` uses, and the same shape the sibling `not_held_by_engine`
   assertion already used (a ceiling on a draining population).
2. **A second implementation.** `self.assertEqual(len(mag), len(units) - len(zero_token))`, where
   `zero_token` is recomputed in the test as the **complement** — the units whose
   `magnitude_token_count` is zero or missing. `magnitude_bearing` is a filter; this is a
   different arithmetic path over the same inventory, so an over-count or an off-by-one breaks
   equality here with **no hand-maintained number pinned**. (`AGENTS.md` concurrency section: any
   number that moves a baseline needs two independent implementations agreeing.)

**MUTATION-PROVED, both halves, not asserted.** Re-derive with the mutation harness recorded in
the correction event:

| Mutation of `SEB.magnitude_bearing` | Result | Caught by |
|---|---|---|
| over-count by one (`real(units) + [units[0]]`) | `FAIL — AssertionError: 26398 != 26397` | half 2, the complement recount |
| under-count (`real(units)[:100]`) | `FAIL — AssertionError: 100 not greater than or equal to 26396` | half 1, the floor |
| unmutated | `Ran 15 tests ... OK` | — |

Nothing was silenced, skipped, ignore-listed, or lowered. The stale figure `26396` in the
adjacent re-derive comment was replaced with `<n>` for the same reason.

## Baseline floors re-pinned — raised, never lowered

Re-gate 5 flagged `BASELINE_ROOT_FULL_TESTS` as a **BASELINE NOTE** (stale, not a failure), so its
stage was PASS and the gate did not go red on it — but it is the same defect class as the pin
above, and `workflow-instruction.md §10 step 0` owes both re-pins in one commit.

**These figures are this cycle's own green run's, not re-gate 5's.** The first correction cycle
already recorded why copying a gate worker's numbers is wrong twice over (HEAD advances under
them), and this cycle folded four new docs under `docs/` between re-gate 5 and its own run, so
`figure-provenance` alone moved `files_checked` 284 → 287.

| Baseline | recorded | measured here | direction |
|---|---|---|---|
| `BASELINE_ROOT_LIB_TESTS` | `3390` | `3390` | **not moved** — exactly at the floor |
| `BASELINE_ROOT_FULL_TESTS` | `8919` | `8926` | **raised +7**, all growth |

Both are **floors** (`scripts/verify.sh::check_floor`, line 222): the stage fails only when
measured is **below** the recorded value. Raising one to a value the same green run measured
cannot mask a regression — that run *is* the evidence the floor holds. Neither was lowered.

## Verification — the full gate, run once, by this cycle

`scripts/verify.sh`, **every stage, no `--only`**, on the shared checkout with
`CARGO_TARGET_DIR=/tmp/cargo-sd35-SD35-E6-WRAPUP-FIX2` and `CARGO_INCREMENTAL=0`.

- **Result:** `RESULT: PASS`, **exit 0** — **49 of 49 stages PASS, 0 FAIL**
  (`grep -cE '^    PASS' /tmp/claude-1000/e6fix/verify_full.log` → 49; `grep -cE '^    FAIL' …` → 0)
- **Wall time:** **8,523 s = 2 h 22 m 3 s** (`start_epoch`/`end_epoch` in `/tmp/claude-1000/e6fix/`); per-stage logs `/tmp/codex-verify-pbJKpU/`
- **Log:** `/tmp/claude-1000/e6fix/verify_full.log`
- **The formerly-red stage:** `PASS shape-engine-boundary-selftest (15 cases passed)`, and its
  companion `PASS shape-engine-boundary (magnitude_bearing=26397 not_held_by_engine=0 citation_ok=True)`
- **The run's own derived `verification` event is MISFILED, and is not silently corrected.**
  `verify.sh` emits it with `RETRO_ACTOR` read from its **own process environment**; this cycle
  exported `RETRO_ACTOR` in the interactive shell but not inside the backgrounded run script, so
  the event landed in `docs/retro/events/sd31-transcribe.jsonl` under actor `sd31-transcribe` —
  `verification | PASS | mode=full | duration_seconds=8522 | log_dir=/tmp/codex-verify-pbJKpU`.
  Re-derive: `grep -rl codex-verify-pbJKpU docs/retro/events/`. The shard is **committed as-is**;
  the retro log is append-only and is never rewritten to look tidier. Recorded as correction
  `1789451471001-at-35-e6-wrapup-fix2-d23401` and incident
  `1789451471125-at-35-e6-wrapup-fix2-10d3a6`, recurrence key
  `retro-actor-lost-between-bash-calls` — a **4th** firing, and the first to contaminate another
  actor's shard rather than merely lose attribution. **Control for the next dispatch:** export
  `RETRO_ACTOR` *inside* any backgrounded run script, not only in the interactive shell.
- **Build scope verified:** the widest the repo has — `root-lib`, `root-full` (all test binaries),
  the desktop crate, frontend, clippy and `reach`, all inside the one run above.

## Worktree sweep — §10 step 2, performed here because the gate worker cannot perform it

Re-gate 5 escalated that `§10 step 2` assigns the sweep to the **one worker structurally incapable
of it**: an isolated worker's git operations are refused against a sibling worktree, so it can
census but never remove, and never even run the `git status --porcelain` cleanliness check that
protects uncommitted work. This is the **third consecutive epic** in which the sweep was censused
and not performed. This cycle is non-isolated, so it ran the check.

**It found five artifacts the FIRST correction cycle's sweep missed**, which had claimed
"5 gate reports + 11 retro event logs recovered from worktrees":

| Worktree | Unfolded path | Disposition |
|---|---|---|
| `-91` | `artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_regate1_report.md` | folded + SUPERSEDED banner |
| `-91` | `docs/retro/events/at-35-e6-wrapup-regate-1.jsonl` | folded |
| `-90` | `artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_regate2_report.md` | folded + SUPERSEDED banner |
| `-90` | `docs/retro/events/at-35-e6-wrapup-regate-2.jsonl` | folded |
| `-90` | `docs/retro/events/epic-6-wrapup.jsonl` | folded |
| `-94` | 6 paths (re-gate 4 + 5 reports, 4 retro shards) | folded — the handoff named these |
| `-89`, `-19` | 5 paths | already folded by the first correction cycle; byte-identical, verified by `diff -q` |

So **11** Epic 6 wrap-up artifacts needed folding across the epic, not the 6 the handoff named.
Re-derive:
`for w in 19 89 90 91 94; do (cd .claude/worktrees/wf_291be5c8-5f3-$w && git status --porcelain); done`,
then `diff -q` each untracked path against the main checkout.

**Removal: censused, not performed — and deliberately so.** Every one of these worktrees was
holding unfolded work at the moment the sweep ran, twice in a row now. Removing them is the
destructive step `§8` guards, and the fold has only just landed in *this* commit; removing them
before that commit is on `origin/tranche/15` would be removing the only other copy. They are left
in place, clean-to-remove, for the epic-closure step once this commit is pushed.

## Three live reports reduced to one

The handoff required both older reports marked superseded rather than three live verdicts standing
side by side. Five reports now exist and **four carry a SUPERSEDED banner at the top**, naming
`EPIC-6_wrapup_gate_report_regate5.md` as the live one and stating precisely what in each is
stale:

| Report | Status |
|---|---|
| `EPIC-6_wrapup_gate_report.md` | SUPERSEDED — describes `77e8d3919a`, four-red list wholly stale |
| `EPIC-6_wrapup_regate1_report.md` | SUPERSEDED — earlier tree, earlier red list |
| `EPIC-6_wrapup_regate2_report.md` | SUPERSEDED — earlier tree, earlier red list |
| `EPIC-6_wrapup_gate_report_regate4.md` | SUPERSEDED — verdict correct, **provenance** stale (declined to execute its own run) |
| `EPIC-6_wrapup_gate_report_regate5.md` | **LIVE** — executed its own 7,937 s full run at `4b69eb7aab` |

None were deleted: they are the record of how the red list was driven from four to one to zero.

## Operator escalations carried forward — neither blocks Epic 7

1. **`.gitignore` needs one line, `.worktrees/`.** Recurrence key
   `untracked-worktrees-dir-on-shared-checkout`, now carried a **12th** time. Verified unfixed at
   this HEAD: `grep -n worktrees .gitignore` → exit 1, no output. `.gitignore` is outside this
   cycle's granted file-touch set, so `AGENTS.md` rule 4 makes it an operator ruling, and this
   cycle prepared the change rather than making it.
2. **`§10 step 2` is assigned to the wrong worker.** Re-gate 5's escalation, restated with new
   evidence: the sweep found five artifacts the previous sweep missed, so the step is not merely
   unperformable by the gate worker — it is *also* not reliably performed by the correction cycle
   without a fold-first sub-step. **Ruling owed:** move `§10 step 2` to the wrap-up **correction**
   cycle (non-isolated by construction), and make its first sub-step "fold every untracked path in
   every epic worktree, diffing against the main checkout", with removal permitted only after that
   fold is pushed.
3. **`epic-wrapup-gate-red`, 7 firings, still a systemic class.** All seven reds across this
   epic's gates were stale pins or stale figures, never behavioural; this run produced two at
   once. Re-gate 5's escalation stands: retiring the key needs live-population constants
   **derived** rather than pinned, which is a `technical-design.md` change and therefore
   operator-scoped. This cycle did the derivable half where it had scope — the
   `magnitude_bearing` pin is now a property, not a constant — which is the pattern the ruling
   would generalise.

- **Sweep population:** N/A — no corpus records touched, `corpus_literal_sweep` unchanged.
- **Oracle pin:** N/A — no figure came from the pinned corpus.
- **Status:** `complete`
- **Notes:** Batch floor EXEMPT by `decisions.md §2`; residue checked at both ends and unchanged.
  No stage was silenced, skipped, ignore-listed, or lowered.
- **Next-cycle scope:** criterion at zero — Epic 6's deliverable is a residue driven to zero and
  `pcgen_residue_gate.py --check` reads `live_files=0 live_hits=0 verdict=PASS`. Epic 7's second
  dispatch is unblocked by `§10 step 0`.
