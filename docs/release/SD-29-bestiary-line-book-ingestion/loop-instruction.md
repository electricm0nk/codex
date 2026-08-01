# SD-29 — Loop Instruction

> ⚠️ **OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
> This file is the operational loop-instruction for SD-29. The bundle is operated via:
>
> ```
> /loop 60m /batch /goal ~/workspace/programs/codex/requirements/SD-29-bestiary-2-3-4-5-content-ingestion/loop-instruction.md
> ```
>
> Do NOT engage this bundle via ad-hoc single-task invocations. The `/loop` cadence + `/batch` concurrency primitive is the supervisor's file-touch partition that enforces 1-cycle-at-a-time per file. One launch runs to closure.
>
> **🟡 UNATTENDED MODE (operator directive 2026-08-01).** The operator is out of town. Cycles MUST NOT pause to ask the operator questions; the operator may not see the harness's output for days. The operating protocol for the duration of the bundle is:
>
> 1. **Default-and-flag, not ask.** When the cycle needs a decision, pick the safer default, capture it in the cycle's `progress.md` receipt, and continue. The operator reviews the receipts after return.
> 2. **No `clarify` tool calls.** Cycles must not invoke the operator clarification tool under any circumstance; this is a hard ban during unattended mode.
> 3. **Blockers are recorded, not raised.** If a cycle hits a true hard-block (auth failed, branch can't be created, identity conflict on disk), record the blocker in `progress.md` with the command and exit code, then attempt the next ready card per `kanban.md`. Do not halt the bundle.
> 4. **`decision-blocked` IS allowed.** Where the playbook calls for an operator decision (Epic 7 DM Toolkit extension in-scope-vs-separate), record `decision-blocked` in `progress.md` with the recorded reason and proceed on the safe default per `successor-forward-scope-register.md C3.1` retrofit. Do not wait.
> 5. **Closure is a goal, not a stop signal.** The bundle runs to closure per `/loop` cadence. The operator's review happens after return; cycles do not pause for operator review.
> 6. **Operator's verbatim:** "include instructions to all 3 that indicate they will be running in unnattended mode since i will be out of town while this runs. They may not stop to ask questions - it might be days before i notice."

## Pre-launch checklist (must be true before any cycle fires)

1. **`kanban.md` exists and lists a ready queue.** (Operator-pinned 2026-08-01: Hermes board retired; work-queue artifact is `kanban.md` paired with `progress.md` inside this directory.)
2. **Branch pushed:** `tranche/9` is pushed to origin (`git push -u origin tranche/9`). (Operator-pinned 2026-08-01.)
3. **OAuth credentials valid:** the active harness has fresh GitHub OAuth credentials for `git push` operations to origin.
4. **Working tree clean:** no uncommitted work-in-progress from a prior bundle. Run `git status` from the repo root.

If any of these is false, the cycle refuses to launch and reports the gap.

## Cycle mechanics

Each cycle follows the SD-22 cycle shape, with the repo's ingestion tooling
inserted at the two points where this project has repeatedly shipped defects —
before any ingest code is written, and at verification. The full procedure is
`docs/governance/book-ingestion-playbook.md`; read it before the first cycle of
each book.

0. **Shape** the book. `cargo run --locked --bin v06_work_inventory`, then read
   the book's `books[]` entry in `docs/work-inventory.json` — `kinds`,
   `files_not_enumerated`, `trap_hits`, `reconciliation`. The shape decides the
   cycle; do not assume a template, and do not assume a bestiary contains
   monsters (Bestiary 5's does not). Done once per book, not once per cycle.
0b. **Trap-report** the book, before writing a line of ingest code:
   `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>`. Record the
   output in the cycle receipt. See `decisions.md` Decision 9.
1. **Read** the doctrine-of-record (`scope-draft.md`, `decisions.md`, current `progress.md`).
2. **Claim** the highest-priority ready card on `codex-tranche-6-1`.
3. **Do** the bounded work (TDD per the repo's `AGENTS.md`: failing test → smallest change → green → refactor). **The player surface is part of the bounded work, not a follow-on** — see `decisions.md` Decision 10.
4. **Verify** with `./scripts/verify.sh` (full, not `--quick`), exit code captured
   directly and never through a pipe. Do not compose a substitute command set;
   `cargo test --workspace --locked` from the repo root does not reach
   `apps/desktop/src-tauri` at all. See `decisions.md` Decision 8.
5. **Commit** with a `feat(sd29): ...` or `fix(sd29): ...` prefix.
6. **Append** the cycle record directly to `progress.md` (no Hermes release —
   the board is retired). The cycle record carries the PR-id, branch-tip, and
   per-cycle test result. The supervisor reads `kanban.md` at top of the next
   cycle to find the next ready card.
7. **Append** the cycle record to `progress.md`, with the command behind every
   figure it publishes.

## Definition of done (per book-ingest cycle)

All of the following, each checkable by someone who was not present:

1. `./scripts/verify.sh` exits `0`. Exit code captured directly.
2. The `reach` stage passes **with a claim for this book's families**, not by
   the families being absent from the gate's inventory. `reach_gate` reporting
   0 matched tests is a hard failure — a gate running zero tests asserts nothing.
3. `cargo run --locked --bin v06_corpus_trap_report -- --audit` exits `0`.
4. `cargo run --locked --bin v06_work_inventory` regenerates
   `docs/work-inventory.json`, the book's units leave `not-started`, and a
   second run changes only `generated_at`.
5. The four-check wired-integration audit
   (`docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit") is clean.
6. Any family that could not be surfaced has an `OPEN_FINDINGS` entry in
   `reach_gate.rs` naming its remedy — recorded as a cycle shortfall, not a pass.
   **For this bundle, `beastiary1/monsters` is already such an entry; ingesting
   a second bestiary while that entry stands means the bundle is accumulating
   the defect, not shipping content.**
7. Baseline movements in `scripts/verify-baselines.env`, if any, are a separate
   reviewable commit carrying `--show-actuals` output.

## Epic ordering

- **Epic 1 (Identifier Cleanup)** fires FIRST. No other epic may start until Epic 1 is closed.
- **Epic 2 (Operator Pre-Launch)** is the pre-launch gate. Pre-launch checklist verifies before any other epic starts.
- **Epics 3-6 (per-bestiary content-source ingest: B2, B3, B4, B5)** may run in any order, but each book is a single cycle-batch.
- **Epic 7 (DM Toolkit extension to consume Bestiary 2-5)** is optional-but-proposed. Per reach-gate doctrine of 2026-08-01, the toolkit extension either lands inside SD-29 (if cycles need the consumer surface to satisfy reach) or surfaces as a Class 1 retrofit in `successor-forward-scope-register.md`. Operator-pinned per-cycle at Epic 5/6 closure.
- **Epic 8 (Closure Epilogue)** fires LAST. Tranche promotion PR fires only after all other epics are closed.
- **Epic 9 (Build Version Numbering)** fires after Epic 1, before Epic 8. First concrete value `0.9.<build>` per the 2026-08-01 amendment.

## Hard stops

- Stops and reports the blocker (per the repo's `AGENTS.md` hard-stop doctrine) when:
  - A single monster block's ingest cycle fails to converge after 3 attempts.
  - The build crashes in a way that requires a non-book-list fix.
  - A cross-bundle reference yields a missing monster id that the source bundle's progress file shows as not yet landed.
  - The operator-pinned branch / board diverges from the in-flight branch / board.
  - **A record family cannot be surfaced without work outside this bundle's epic structure** (Decision 10's open question — the monster catalog and browser are the known instance). The cycle reports the gap; it does not add an epic and it does not ingest without a reach claim.
  - **A figure derived this cycle disagrees with a figure recorded in this package.** Investigate which is wrong and report; do not overwrite either on the assumption that the newer one wins.
  - **A book's derived shape contradicts its recorded ingest subtype** — e.g. a per-monster-block epic against a book the generator reports as carrying zero monsters. The cycle reports; the operator re-pins the book list.

## Eligibility

A cycle is eligible to fire when:

- The pre-launch checklist is fully green.
- All parent cards the cycle depends on are `complete`.
- The current `progress.md` corresponds to the operator-pinned branch tip.

## Self-heal

- A flaky test that fails once but passes on a clean re-run is annotated in the cycle record and not re-fired.
- A code-side identifier that leaks the `sd29_` pattern is renamed in-cycle (per the identifier-discipline doctrine).
- A cross-bundle reference that yields a missing-monster error is filed as a blocker against the source bundle and the cycle pauses.

## Cross-bundle references

SD-29 references the following bundles:

- **SD-22 (closed):** Bestiary 1 + DM toolkit. Reference is doctrinal read-only. Do not pull from `~/workspace/SD-22-...-*.md` files; pull from SD-22's repo canonical (`~/workspace/repos/codex/docs/release/SD-22/`).
- **SD-28 (planned):** Ultimate book content-source ingest. No live cross-reference until SD-28 is launched.
- **SD-30 (planned):** Occult Adventures + companions. No live cross-reference until SD-30 is launched.

## Decision record

See `decisions.md` for the running decision record. Each decision is dated, named, and stable.

## Per-bundle progress file

`~/workspace/programs/codex/requirements/SD-29-bestiary-2-3-4-5-content-ingestion/progress.md` carries the per-cycle receipt. Do not use a shared chassis-lane progress file; each bundle's progress is its own.
