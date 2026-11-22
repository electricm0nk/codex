# Cycle 1 — Epic 7 Closure epilogue / AT-35-E7-002 (retrospective + `§11` step 3 sweep)

- **Commit SHA:** cycle start `a57ae67cb5` (`tranche/15`, in sync with `origin/tranche/15`).
- **Scope gate:**
  `SCOPE_GATE: EXEMPT (Epic 7 closure-epilogue cycle — closes zero units by design, decisions.md §2)`
  ```
  python3 scripts/cycle_scope_gate.py --min 500
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
- **Build scope:** **no cargo stage run, and the SHA says why.** This cycle changes no `.rs`, no
  `data/`, no gate script and no `Cargo.toml`. `git diff --name-only 7753c29915..HEAD -- '*.rs' 'data/' 'Cargo.*'`
  is empty at the tree `AT-35-E7-001` cycle 4 ran the full `scripts/verify.sh` against (49 of 49
  PASS). A rebuild would compile a byte-identical crate, and `AGENTS.md` rule 9 forbids inventing a
  fresh figure for it. The two gates this cycle's own edits *can* break were run and are green
  (below).
- **Files touched:** `docs/retro/sd35-corpus-sheet-completion-retrospective.md` (new);
  `references/README.md` (the citation, same cycle); `progress.md` (one rewrap + this entry);
  `kanban.md` (row 110); this receipt; six artifacts folded out of dispatcher worktrees;
  `docs/retro/events/at-35-e7-002.jsonl`.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS — nothing shipped this cycle
- **Acceptance criterion:** `workflow-instruction.md §11` steps 2 and 3, in that order, both before
  the PR. Step 2: write the retrospective to
  `docs/retro/sd35-corpus-sheet-completion-retrospective.md`, grounded in
  `python3 scripts/retro.py summary --since 2026-09-08 --json`, in `docs/retro/sd31-retrospective.md`'s
  shape, stating build time before and after, units per cycle (min, median, max), the
  lines-per-unit distribution, the `Words` share per kind, and the PCGen residue count per epic —
  **and cite it from `references/README.md` in the same cycle**. Step 3: full worktree/branch sweep
  with counts found vs removed.
- **Receipt rows (mechanical):**
  ```
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=0
  ```
  `builds_recorded=0` is a measured count of compile sessions in this cycle's
  `CARGO_TARGET_DIR` (`/tmp/cargo-sd35-AT-35-E7-002`), not a claim: there were none, for the
  reason stated under Build scope.
- **PCGen residue:** `live_files=0 live_hits=0 shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11 verdict=PASS`
  (`python3 scripts/pcgen_residue_gate.py --check --closure`, exit 0) — re-run at this cycle's HEAD,
  not quoted from cycle 4.
- **Oracle parity:** N/A — no live path touched, no `Number` mapping added.
- **Movement, four buckets:** closure **0**; relabel **0**; reachability **0**;
  instrument-correction **1** (the `progress.md` denominator rewrap, below).
- **Refused tokens:** **none.**
- **Status:** complete
- **Next-cycle scope:** `AT-35-E7-003`'s remaining half — architecture docs, graphify, PR, release
  notes and version confirmation (`§11` step 4 and step 5, `../../template/template.md §6`) — plus
  the operator action named in the sweep escalation below.

---

## 1. Step 2 — the retrospective

`docs/retro/sd35-corpus-sheet-completion-retrospective.md`, **1 file, written this cycle**, in
`sd31-retrospective.md`'s shape (front-matter with `board:`, a raw event block, "What the data
says" / "What worked" / "What did not work" / "Changes for SD-36" / "The finding that reframed the
package"). Grounded in `python3 scripts/retro.py summary --since 2026-09-08 --json`: **731 events**
across 77 shards, 0 invalid lines — 225 correction, 201 verification, 93 deferral, 90 resolution,
75 incident, 41 note, 6 rework; git join 346 commits, 2.11 events per commit.

The five figures `§11` step 2 names, each with the command in the document beside it:

| required figure | what the retrospective states |
|---|---|
| **build time before and after** | `188.97 s → 145.89 s`, a saving of 43.08 s of 188.97 s, cold and **paired** on a quiet box (`build-time.json` runs 3 and 4); binaries 544 → 362; the test-entry list byte-identical at 8,723. Run 2 (the unpaired `after` that read 295.95 s under another lane's load 40) is reported, not hidden — it is the section's lesson. |
| **units per cycle (min, median, max)** | **min 0, median 0, max 21,911** over 127 receipts' `closed=` rows. Five cycles closed anything; `AT-35-E2-005` closed **21,911 of the 23,315** units not DONE at the cut. |
| **lines-per-unit distribution** | mean **1.42**, median **1**, p95 4, p99 9, max 201; full histogram 1–10 and `>10`; **44,840 of 49,450 units print exactly one line**. |
| **`Words` share per kind** | all 19 kinds tabulated over 70,317 rules: `deity` 459 of 459 and `language` 136 of 136 at 100%, down to `monster` 719 of 10,553; corpus-wide **39,941 of 70,317**. `Number` 27,198, `Dice` 3,173, `DiceBySize` 5. |
| **PCGen residue per epic** | `live_files` first-and-last receipt per epic: E1 260→260 (the gate was built there), E2–E5 260→253 (one shared −7), **E6 254→0**, E7 0→0. Epic 6 did the whole −254 across 67 receipts, monotonically: 254→208→197→81→45→25→16→8→4→0. |

**The citation landed in the same cycle** (`§11` step 2's explicit requirement):
`references/README.md`'s "Retrospectives" table row for SD-35 now names the author, the date, the
grounding command, and every one of the five figures with its value.

## 2. Step 3 — the sweep, counts found vs removed

| item | found | removed | kept |
|---|---|---|---|
| worktrees (`git worktree list`, minus the main checkout) | **16** | **0** | **16 — the classifier refuses `git worktree remove`** |
| `worktree-wf_*` local branches (`git branch`) | **23** | **8** | 15, each held checked out by one of those 16 worktrees |
| untracked files inside those worktrees | **30** | — | **6 folded into the main checkout**, 24 already present in main with main's copy the later one |
| modified tracked files inside those worktrees | ~14 per worktree | — | discarded deliberately, proven superseded |
| `test` / `update-index` | 2 | **0** | **never deleted, by standing rule** |
| remote branches (`git branch -r`) | **8** | 0 | none stale |
| disk (`df -h /`) | 61% used, 569G free of 1.5T | unchanged | 13G under `.claude/worktrees` not reclaimed |

**Losslessness established before the sweep, not asserted after it.**

- All 16 worktree `HEAD`s are ancestors of `tranche/15` (`git merge-base --is-ancestor <sha> HEAD`,
  16 of 16).
- 22 of the 23 branches are ancestors of `tranche/15`. The 23rd,
  `worktree-wf_291be5c8-5f3-96`, is **47 commits ahead** — every one contained in `origin/develop`
  (it is the v0.8 desktop line through PR #389), so deleting it lost nothing and it was deleted.
- Of 30 untracked files, 24 exist in main; each was byte-compared, and **main's copy was the later
  one in every case** (the worktree drafts predate the denominator-gate rewrites). The 6 that
  existed nowhere else were copied in and are committed here:
  `epic-1-tax-cut/epic-1_wrapup-gate_receipt.md`,
  `epic-3-place-and-surface/EPIC-3_wrapup_gate_report_regate.md`,
  `epic-6-pcgen-exit/EPIC-6_wrapup_gate_report_regate6.md`, and the retro shards
  `epic-1-wrapup-gate.jsonl`, `at-35-e3-regate.jsonl`, `e6-wrapup-regate-1.jsonl`.
  This is the **third** independent sweep to find artifacts the previous one missed
  (`unfolded-gate-artifacts-die-with-the-worktree`, now at 4 firings).
- The modified `.rs` files are a superseded earlier pass at Epic 6's `pcgen_import` relocation:
  they rewrite `use crate::rules_core::pilot_compute::formula_interpreter` to
  `use crate::pcgen_import::formula_interpreter`, and HEAD has already moved the module —
  `src/rules_core/pilot_compute/formula_interpreter.rs` does not exist and
  `src/pcgen_import/formula_interpreter.rs` does. Nothing to preserve.

**Escalation — the eleventh recurrence, and it is not an agent problem.** `git worktree remove` is
refused by the Claude Code auto-mode permission classifier for a dispatched agent on this checkout,
both as a loop over the 16 paths and as a single explicit path. Ten open deferrals from the Epic
2–6 wrap-ups record the same refusal, and `AT-35-E6-WRAPUP-FIX2`'s receipt already states removal
was *"attempted — refused by the harness permission layer, not by git"*. `git branch -D` is
permitted, which is how the 8 orphan branches came out. **`workflow-instruction.md §10 step 2` and
`§11 step 3` assign a step no agent in this program has ever been permitted to run.** It needs an
operator-run `git worktree remove --force` over the 16 paths, or a `Bash` permission rule, and it
belongs in SD-36's launch checklist. Recorded as
`deferral 1789503195144-at-35-e7-002-023beb` rather than silently re-attempted.

## 3. One correction, found by this cycle's own gate run

`AT-35-E7-001` cycle 4 reported `denominator-gate files_checked=360 violations=0` from a
`verify.sh` run at `7753c29915`, then wrote its `progress.md` entry at `17ea4c1595` — **after**
that run. The entry wraps `48,864 of 48,864 real data/corpus rules records` and `= 100%` onto
separate lines, leaving a percentage with no denominator marker on its own line.

```
python3 scripts/denominator_gate.py --check
files_checked=364 violations=1        # before the rewrap
files_checked=364 violations=0        # after
python3 scripts/denominator_gate.py --check-provenance
files_checked=294 figures_examined=627 violations=0
```

Fixed by moving the line break, **no figure altered**. Recorded as
`correction 1789503578833-at-35-e7-002-23ccb2`. It is the same class as the 4x
`figure-provenance-command-on-next-line` recurrence, and it generalises: **a closure cycle's own
`progress.md` and `kanban.md` entries are written after its gate run, so the gate never sees
them.** That is now a "Changes for SD-36" row.

## 4. Gates run by this cycle at HEAD

| gate | line | exit |
|---|---|---|
| `python3 scripts/cycle_scope_gate.py --min 500` | `scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER` | 0 |
| `python3 scripts/pcgen_residue_gate.py --check --closure` | `live_files=0 live_hits=0 shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11 verdict=PASS` | 0 |
| `python3 scripts/denominator_gate.py --check` | `files_checked=364 violations=0` | 0 |
| `python3 scripts/denominator_gate.py --check-provenance` | `files_checked=294 figures_examined=627 violations=0` | 0 |

The cargo and corpus stages are not re-run, for the reason given under **Build scope**: this cycle
changed no code and no data, and `AT-35-E7-001` cycle 4's full `scripts/verify.sh` — **49 of 49
PASS** — stands at the tree those stages would examine.
