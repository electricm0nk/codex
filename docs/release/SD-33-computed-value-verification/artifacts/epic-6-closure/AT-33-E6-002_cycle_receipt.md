# Cycle AT-33-E6-002 — epic-6-closure / retrospective written and cited, worktree/branch sweep

- **Commit SHA:** this cycle's own landing commit (see `progress.md` entry `AT-33-E6-002`).
- **Worktree:** clean `git worktree add --detach` off `origin/tranche/13` = `4a84b567e0`
  (attempt 10's own landing SHA), outside the shared checkout.
- **Files touched:** `docs/retro/sd33-computed-value-verification-retrospective.md` (new);
  `docs/release/SD-33-computed-value-verification/references/README.md` (§1, new top bullet);
  `docs/release/SD-33-computed-value-verification/forward-scope-register.md` (new `## D1.x`
  section); `docs/release/SD-33-computed-value-verification/progress.md` (new cycle entry);
  `docs/release/SD-33-computed-value-verification/kanban.md` (row 20); this receipt.

## Environment finding — seventh consecutive wave

The shared checkout at `/home/ubuntu/workspace/repos/codex` sat **8 commits behind
`origin/tranche/13`** (local `HEAD` = `06e858b0e6`, origin = `4a84b567e0`) with **159 entries** in
`git status --porcelain` this agent did not create (137 staged `data/corpus/**` modifications, 7
deletions of Epic 5 artifacts, 15 untracked `.workflow.js` files) — the same recurring hazard
`AT-33-E6-001-build-green_cycle_receipt.md` (141/7/9-entry shape) and attempts 6–10
(154 → 158 → 158 → 159 entries) each independently found and worked around. Per `AGENTS.md`
"One writer per tree", nothing was written there and nothing was discarded. All writes above
happened in the clean detached worktree named above; the shared checkout was left exactly as
found.

## 1. Retrospective — written

`docs/retro/sd33-computed-value-verification-retrospective.md`, grounded in
`python3 scripts/retro.py summary --since 2026-08-24 --json`, read live before writing (not from
recollection or from any lane's prior report). Leads with the twelve real defects this bundle
found and fixed against a computed value or an instrument that had already been marked `done`
(spell-DC fixture / `dded72f0b4`; armor compute / `abc72f75ec`; AC-measurement method /
`a68fbeea3d`; `compute_equipmods_effect` multi-chain / `2f1d52f22d`; `equipment_id_resolve`;
campaign-KEY-vs-display-name harness; corpus-extraction `.MOD`-EQMOD gap /
`fbc945f198` + `7d439876b7`; two previously-unwired resolvers, `EQMWEAPON|DAMAGESIZE` and
`EQM|WEIGHTDIV`; Epic 4's doneness-pair regression, closed inside the bundle; the Epic-5
struct-rename that hid 543 of 543 integration targets; the two `.MOD`-chain closure-builder
defects the corpus sweep caught; and the scan's own scope defect), each named with its mechanism
and its commit. Second, the throughput arc with denominators (**32 → 6,940 → 7,939 → 8,255 →
8,263 → 8,291 → 8,330 of 8,330 examined**; disagreements **26 → 4 → 1 → 0**) and the ten-wave,
nine-correct-halt shape, including the two near-misses caught mechanically (wave 2's short-population
row, caught by row-counting; wave 7's mis-attribution, caught by re-deriving from `git`). Ten
lessons a–j, each with its enforcing mechanism per `decisions.md §4` — none entered as prose alone.

## 2. Cited — same cycle

`docs/release/SD-33-computed-value-verification/references/README.md` §1 gained a new top bullet
citing the retrospective, ahead of the SD-32 and SD-31 citations already there.

```
$ grep -n "sd33-computed-value-verification-retrospective" \
    docs/release/SD-33-computed-value-verification/references/README.md
14:- **`../../../retro/sd33-computed-value-verification-retrospective.md`** — **this bundle's own retrospective**, ...
```

## 3. Inherited debt — registered

`forward-scope-register.md` gained a new `## D1.x — Inherited debt, verified not assumed` section
with two rows, each carrying its proof command and both its counts and their denominators:

- **D1.1** — 31 of 599 test suites / 49 of 8,026 executed tests, proven pre-existing at the
  `tranche/13` cut (re-derived here, not re-quoted, matching attempt 10's own Check 3):

  ```
  $ for f in <each of the 31 failing target paths from AT-33-E6-001-attempt10_cycle_receipt.md>; do
      git log --oneline f652db7ac7..HEAD -- "$f" | wc -l
    done | awk '{s+=$1} END {print s}'
  0
  ```
  0 of 31 failing targets carry a single commit since the cut. This bundle **verified** the
  inheritance rather than assuming it — see the register row for the full target list.
- **D1.2** — `verify.sh`'s `site-dashboard-check` stage hang, root-caused (no timeout wrapper
  around `v06_work_inventory --summary` in either `verify.sh` or
  `scripts/publish-site-dashboard.sh`), reproduced identically across three separate diffs.

Neither row defers scope that was in SD-33's own Definition of Done; both are named with an owner
per the register's own "no unowned tidiness entries" rule.

## 4. Full worktree/branch sweep (`§11.3`)

**Worktrees.** `git worktree list` at cycle start (excluding the main checkout and this cycle's own
scratch worktree): **24 found**, all pre-existing litter from waves 1–10.

| Category | Count | Disposition |
|---|---:|---|
| Clean, HEAD merged into `origin/tranche/13`, no dirty files | 19 | Removed (`git worktree remove`) |
| Dirty with exactly 1 line (`docs/retro/events/sd31-transcribe.jsonl`, a per-worktree `verify.sh` auto-append breadcrumb; own HEAD merged) | 4 | Inspected (`git diff`, confirmed diagnostic-only), removed (`git worktree remove --force`) |
| Dirty with 911 lines (the same staged `data/corpus/**` revert hazard, own HEAD merged but the working-tree diff not independently verified safe to discard) | 1 (`wf_0e74dc1e-ba9-1`) | **Kept** — not locked, but carries uncommitted local modifications this cycle did not create and could not verify were junk; force-removing would discard them irreversibly |

**24 found, 23 removed, 1 kept** (`wf_0e74dc1e-ba9-1`). 0 locked worktrees existed
(`git worktree list --porcelain \| awk '/^worktree/{w=$2} /^locked/{print w}'` → empty, checked
before any removal). Every removal was preceded by
`git merge-base --is-ancestor <worktree HEAD> origin/tranche/13` — **all 24** returned true (the
committed history in every one of them, including the one kept, is fully merged; only the one
kept worktree's *uncommitted* diff is unverified).

**Branches.** `git branch -a` at cycle start (excluding `develop`/`tranche/13`, the two structural
branches): **29 found** — 25 `worktree-wf_*` orphan branches (created alongside the worktrees
above), `sd33-r6-e5-finalize-work` / `sd33-r7-suite-green-work` / `sd33-r8-build-green-work` (3,
leftover dispatch-lane branches), and `sd31/racetrait4-SD31-E6-F4-005` (1, a known SD-31 rescue
branch).

```
$ git branch --merged origin/tranche/13 | grep worktree-wf | wc -l
21    # (20 confirmed against origin/tranche/13 directly; 3 more — d17c6032-727-2/-4/-5 —
      #  confirmed the same way after `git branch -d` initially refused them against the
      #  local stale tranche/13 tip rather than origin's)
```

**Deleted: 24** (21 `worktree-wf_*` + `sd33-r6-e5-finalize-work` + `sd33-r7-suite-green-work` +
`sd33-r8-build-green-work` — every one independently confirmed
`git merge-base --is-ancestor <sha> origin/tranche/13` before deletion, not inferred from
`git branch -d`'s own refusal/acceptance against the locally stale `tranche/13` tip).

**Kept: 5**, all genuinely unmerged (`git branch --no-merged origin/tranche/13`):
`worktree-wf_13156488-c9b-1`, `worktree-wf_a45ece26-3fc-1`, `worktree-wf_be4660f2-72a-3`,
`worktree-wf_c1156061-e3f-5` (four SD-31-era branches, 500+ commits behind `origin/tranche/13` and
carrying 1–2 commits each not present there — real historical work, no live worktree pointed at
any of them even before this sweep), and `sd31/racetrait4-SD31-E6-F4-005` (the SD-31 rescue branch
explicitly flagged elsewhere as "must not be merged on trust" — an operator-review item, not
litter).

**Disk:** `df -h /` — **449G used of 968G total (47% of 968G)** after the sweep, down from **469G
used of 968G total (49% of 968G)** measured at cycle start — worktree removal freed roughly 20G.

## Movement, four buckets

Closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle writes
package docs and removes cleanup litter; no unit, gate, or instrument changed.

## Identifier / wired-integration audits

No `src/`/`scripts/`/`apps/` file touched this cycle — N/A by scope (docs and git-administrative
actions only).

- **Status:** complete
- **Notes:** One worktree (`wf_0e74dc1e-ba9-1`) and five branches were deliberately **not**
  removed, each for a stated reason above, not silently skipped. No `data/corpus/**` file was
  hand-edited or discarded — the kept worktree's 911-line diff includes such changes and is
  exactly why it was left alone rather than force-removed.
- **Next-cycle plan:** `AT-33-E6-003` — architecture docs, graphify, PR, and merge-conflict
  resolution (`workflow-instruction.md §11.4`–`§11.5`), gated on this cycle per §11's own ordering
  (steps 2–3 land before step 4 opens the PR). The kept worktree and the 5 kept branches are named
  here for that cycle (or an operator) to review, not carried silently.
