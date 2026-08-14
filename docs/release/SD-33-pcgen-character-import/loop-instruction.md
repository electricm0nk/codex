---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
---

# SD-31 Loop Instruction — Per-Cycle Procedure

The bundle runs unattended. This file is the per-cycle procedure; `kanban.md` is the queue and
`progress.md` is the receipt layer.

## Step 0 — Checkout hygiene (shared-checkout discipline)

- Run `git status --porcelain` **before any git write**. Another agent may own HEAD.
- Never `git stash` — the bare form stashes the whole repo even from a subdirectory.
- Never `git add -A`. Stage the cycle's own files by name.
- SD-29 and SD-30 are live in sibling worktrees. Confirm the working branch is `tranche/11`
  before writing.

## Step 1 — Claim

Read `kanban.md` top-down; take the first `READY` card whose gates are all `COMPLETE`. Mark it
`IN-FLIGHT` in the same commit as the first code change, not before.

## Step 2 — Re-derive, do not inherit

Before writing code, re-check the facts the card depends on **by command**:

- The `TR-31-001` partition against SD-29's and SD-30's current `technical-requirements.md`.
  Both are live bundles; their partitions can widen after this package was written.
- Any list of code entities the card acts on. Inventories decay silently on a shared checkout.
- Any claim quoted from a planning doc, including this package's own.

A cycle that acts on an inherited inventory without re-deriving it has already failed, whatever
the tests say.

## Step 3 — Build

Implement the card's feature seed per `epic-breakdown.md`, within the `TR-31-001` partition.

If the card touches a `TR-31-002` registration site: change only the registration line, and
capture the exact diff hunk for the receipt.

## Step 4 — Gate

```sh
cargo test --locked pcgen_character
cd apps/desktop && npm test && npx tsc --noEmit
```

Record exit codes and matched-test counts. Zero matched tests with exit 0 is a hard failure.

**Build hygiene on a contended box:** claim a scratch `CARGO_TARGET_DIR` under
`~/workspace/codex-target-sd31-<slug>` rather than sharing the default target dir with the
sibling bundles' builds, and remove it at cycle end. Cap parallelism (`-j 2`) while SD-29 is
building.

## Step 5 — Audit

- Wired-integration 4-grep audit (`TR-31-008`) — mandatory on any cycle touching the player surface.
- Identifier-discipline audit — no `sd31_*` in surface code.
- `git diff --name-only` against the partition. Any path outside `TR-31-001` fails the cycle.

## Step 6 — Receipt

Append to `progress.md`:

- Cycle id, card, date, actor.
- What landed, in one short section. **No narrative ceremony** — this bundle's receipts are
  evidence, not storytelling.
- Gate output: exit codes, matched-test counts, and this cycle's **test-count delta**
  (`TR-31-003` — do not edit the shared baseline).
- Any `TR-31-002` diff hunk.
- Deviations and deferrals, each with a reason.

## Step 7 — Commit and continue

Commit on `tranche/11` and push. Per-cycle commits and pushes on the tranche branch are
pre-authorized; the operator approves only the tranche → develop merge. Do not pause per cycle
to ask what to do next — take the next ready card.

## Stop vs. press on

**Stop and record a blocker** when:

- A fixture references content Codex has not ingested. That is an SD-29/SD-30 concern. Record it
  as an `UnresolvedReason::RecordNotIngested` case and a forward-scope entry — **never ingest
  corpus content from an SD-31 cycle** (`TR-31-001`).
- The sibling bundles' partitions have widened to overlap SD-31's.
- Oracle parity diverges and the cause is Codex's engine rather than the importer.

**Press on** for anything resolvable inside the partition.
