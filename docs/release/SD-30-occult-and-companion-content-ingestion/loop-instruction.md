# SD-30 — Loop Instruction

> ⚠️ **OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
> This file is the operational loop-instruction for SD-30. The bundle is operated via:
>
> **Dispatch mechanism: the in-harness `Workflow` tool, driven from a live session** — not a headless `/loop` script and not a cron driver. Deterministic control flow (per-epic ordering, fan-out, `decision-blocked` handling) lives in this document and in `kanban.md`'s claim/complete state; model judgment lives inside the dispatched `agent()`/`Workflow` calls. Per `decisions.md §22` (adopted from SD-27 `decisions.md §19`, itself adopted from SD-26 `decisions.md §13`) and `docs/governance/loop-instruction-template.md §2`.
>
> `/batch` is **not** the default concurrency primitive for this bundle. `/batch` fans out into parallel isolated worktrees by default; SD-30's cycles mutate shared state on nearly every cycle (`progress.md`, `kanban.md`, `reach_gate.rs`'s `OPEN_FINDINGS`), so parallel dispatch is the exception, called out explicitly per-epic (see "Epic ordering" below), not the default. Any parallel wave dispatches each agent with `isolation: 'worktree'` (`loop-instruction-template.md §3`); a shared-checkout wave with more than one mutating agent and no worktree isolation is not a valid dispatch. Where cycles touch shared state, dispatch an explicit single-cycle procedure instead of reaching for `/batch`.
>
> Every dispatched agent gets `RETRO_ACTOR=<role-name>` set in its environment (`loop-instruction-template.md §2.1`) — no harness variable identifies an agent's role, and the fallback (worktree directory name) names a checkout, not a role, which makes the retrospective log's by-actor breakdown meaningless.
>
> The orchestrating session never implements directly — it dispatches, verifies, and rules (`loop-instruction-template.md §2.2`). Do NOT engage this bundle via ad-hoc single-task invocations; one Workflow-tool launch runs to closure.
>
> **Orchestrator model: Opus, low reasoning effort** (operator directive 2026-08-01, `decisions.md §25`) — Opus at low reasoning effort produced materially better orchestration results than Sonnet at high reasoning effort, and is the new normal for the *orchestrating session* on this program. This supersedes any prior "orchestration runs on Sonnet" guidance (none existed in this package before this pass). Dispatched sub-agents are unaffected — they keep task-matched tiers (Haiku for housekeeping, Sonnet for real implementation/debugging/review, Opus for adversarial verification/judge-panel steps only) per `loop-instruction-template.md §2`. A session cannot change its own model mid-run: setting Opus-low is a **pre-launch operator step**, done before this cycle session starts.
>
> **🟡 UNATTENDED MODE (operator directive 2026-08-01).** The operator is out of town. Cycles MUST NOT pause to ask the operator questions; the operator may not see the harness's output for days. The operating protocol for the duration of the bundle is:
>
> 1. **Default-and-flag, not ask.** When the cycle needs a decision, pick the safer default, capture it in the cycle's `progress.md` receipt, and continue. The operator reviews the receipts after return.
> 2. **No `clarify` tool calls.** Cycles must not invoke the operator clarification tool under any circumstance; this is a hard ban during unattended mode.
> 3. **Blockers are recorded, not raised.** If a cycle hits a true hard-block (auth failed, branch can't be created, identity conflict on disk), record the blocker in `progress.md` with the command and exit code, then attempt the next ready card per `kanban.md`. Do not halt the bundle.
> 4. **`decision-blocked` IS allowed.** Where the playbook calls for an operator decision (Mythic Adventures consumer surface in-scope-vs-separate; psychic-discipline consumer; Inner Sea campaign-tool surface), record `decision-blocked` in `progress.md` with the recorded reason and proceed on the safe default per `forward-scope-register.md C3.x` retrofit. Do not wait. See also "Stop vs. press on" below for the general rule this instance follows.
> 5. **Closure is a goal, not a stop signal.** The bundle runs to closure under the Workflow tool's own dispatch loop, not a human re-invoking a slash command per cycle. The operator's review happens after return; cycles do not pause for operator review.
> 6. **Operator's verbatim:** "include instructions to all 3 that indicate they will be running in unnattended mode since i will be out of town while this runs. They may not stop to ask questions - it might be days before i notice."

## Pre-launch checklist (must be true before any cycle fires)

1. **`kanban.md` exists and lists a ready queue.** (Operator-pinned 2026-08-01: Hermes board retired; work-queue artifact is `kanban.md` paired with `progress.md` inside this directory.)
2. **Branch pushed:** `tranche/10` is pushed to origin (`git push -u origin tranche/10`). (Operator-pinned 2026-08-01.)
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
   cycle; do not assume a template. **Confirm the book has a corpus directory
   at all** — two of this bundle's four candidates currently do not. Done once
   per book, not once per cycle.
0b. **Trap-report** the book, before writing a line of ingest code:
   `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>`. Record the
   output in the cycle receipt. See `decisions.md` Decision 10.
1. **Read** the doctrine-of-record (`scope-draft.md`, `decisions.md`, current `progress.md`).
2. **Claim** the highest-priority ready card on `kanban.md` (per `decisions.md §13` + §14a).
3. **Do** the bounded work (TDD per the repo's `AGENTS.md`: failing test → smallest change → green → refactor). **The player surface is part of the bounded work, not a follow-on** — see `decisions.md` Decision 11.
4. **Verify** with `./scripts/verify.sh` (full, not `--quick`), exit code captured
   directly and never through a pipe. Do not compose a substitute command set;
   `cargo test --workspace --locked` from the repo root does not reach
   `apps/desktop/src-tauri` at all. See `decisions.md` Decision 9.
5. **Commit** with a `feat(sd30): ...` or `fix(sd30): ...` prefix.
6. **Append** the cycle record directly to `progress.md` (no Hermes release —
   the board is retired). The cycle record carries the PR-id, branch-tip, and
   per-cycle test result. The supervisor reads `kanban.md` at top of the next
   cycle to find the next ready card.
7. **Append** the cycle record to `progress.md`, with the command behind every
   figure it publishes.
8. **Emit** a retro event for anything this cycle corrected, deferred, reworked,
   or narrowly avoided. See "Retrospective log" below — this step is part of
   the cycle, not an optional courtesy.

## Retrospective log

Every cycle emits at least one event to the running retrospective log
(`scripts/retro.py`, schema `docs/retro/schema.json`, shards
`docs/retro/events/<actor>.jsonl` — do not re-derive the flag syntax from
memory, run `python3 scripts/retro.py help <type>` for the real fields per
type).

- `RETRO_ACTOR` is exported for the dispatched agent per the OPERATING METHOD
  callout above; emissions from inside a cycle inherit it automatically.
- `./scripts/verify.sh` (Cycle mechanics step 4) auto-emits its own
  `verification` event on every run, pass or fail — the denominator is honest
  without anyone deciding to record it. Nothing to do here beyond not skipping
  `verify.sh`.
- A correction to a figure this package (or a prior cycle) stated is not
  folded silently into the next edit: emit
  `scripts/retro.py correction --subject <who/what stated it> --claimed <...> --actual <...> --verified-by <the command/file/count that established the true value>`
  alongside the doc fix. A correction without `--verified-by` is just a
  competing assertion.
- A deferral, a rework, or a near-miss verification caught is emitted at the
  point it happens, never narrated afterward from memory.

See `decisions.md §23`.

## Definition of done (per book-ingest cycle)

All of the following, each checkable by someone who was not present:

1. `./scripts/verify.sh` exits `0`. Exit code captured directly.
2. The `reach` stage passes **with a claim for this book's families**, not by
   the families being absent from the gate's inventory. `reach_gate` reporting
   0 matched tests is a hard failure — a gate running zero tests asserts nothing.
   A record kind the gate does not recognise (haunts, corruptions, psychic
   disciplines) fails until it is classified in `RECORD_TYPE_KINDS` with the
   surface that renders it, or in `SUPPORTING_RECORD_TYPES` with why it is a
   facet of an existing family.
3. `cargo run --locked --bin v06_corpus_trap_report -- --audit` exits `0`.
4. `cargo run --locked --bin v06_work_inventory` regenerates
   `docs/work-inventory.json`, the book's units leave `not-started`, and a
   second run changes only `generated_at`.
5. The four-check wired-integration audit
   (`docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit") is clean.
6. Any family that could not be surfaced has an `OPEN_FINDINGS` entry in
   `reach_gate.rs` naming its remedy — recorded as a cycle shortfall, not a pass.
7. Baseline movements in `scripts/verify-baselines.env`, if any, are a separate
   reviewable commit carrying `--show-actuals` output.

## Epic ordering

- **Epic 1 (Identifier Cleanup)** fires FIRST. No other epic may start until Epic 1 is closed.
- **Epic 2 (Operator Pre-Launch)** is the pre-launch gate. Pre-launch checklist verifies before any other epic starts.
- **Epics 3-N+ (per-book content-source ingest: occult + mythic + Monster Codex + Inner Sea + Book of the Damned ×2, sixteen books total)** may run in any order per the file-touch partition. Per-book epics may group Inner Sea's nine modules into one shared epic or split per book; epic-boundary decision is at Cycle 2's inventory gate.
- **Closure Epilogue** fires LAST. Tranche promotion PR fires only after all other epics close.
- **Build Version Numbering** fires after Epic 1, before Closure. First concrete value `0.10.<build>` per `decisions.md §15`.
- **Epic 21 (Bundle Code Review)** fires after Build Version Numbering and every content-ingest epic, before Closure. Reviews the whole bundle's diff against its branch point, not the closing cycle alone; `./scripts/verify.sh` passing is a precondition, not the review itself. Per `decisions.md §26`.

## Hard stops

- Stops and reports the blocker (per the repo's `AGENTS.md` hard-stop doctrine) when:
  - A single class / monster / discipline's ingest cycle fails to converge after 3 attempts.
  - The build crashes in a way that requires a non-book-list fix.
  - A cross-bundle reference yields a missing class / monster id that the source bundle's progress file shows as not yet landed.
  - The operator-pinned branch / board diverges from the in-flight branch / board.
  - **A book on the recorded list has no corpus directory to ingest from.** The cycle reports; the operator re-pins the book list. Known instances as of 2026-07-30: Occult Origins and Haunted Heroes Handbook.
  - **A record family cannot be surfaced without work outside this bundle's epic structure** (Decision 11's open question). The cycle reports the gap; it does not add an epic and it does not ingest without a reach claim.
  - **A figure derived this cycle disagrees with a figure recorded in this package.** Investigate which is wrong and report; do not overwrite either on the assumption that the newer one wins.

## Stop vs. press on

The bullets above are this bundle's concrete STOP instances. This section
states the general rule they follow, so a case not on that list is still
classified correctly. Getting this backwards is expensive in both directions:
stopping on the wrong things stalls the bundle for days under unattended mode;
pressing on through the wrong things ships a defect or clobbers someone else's
work.

**STOP — record `decision-blocked` per the UNATTENDED MODE protocol above; do
not fabricate a pass:**

- A gate fails for a reason that is a real finding about content or scope —
  the reach gate flagging genuinely unsurfaced content is the paradigm case,
  not an exception to route around. Never weaken, skip, `#[ignore]`, or
  exclude a gate to get green, and never invent a surface or a number to
  satisfy one.
- Two authorities disagree on scope (e.g. this package and a sibling bundle's
  docs naming different in-scope books/records).
- The work would revert or clobber another session's live work on the shared
  branch.
- Proceeding would require inventing data not present in the corpus.

None of these mean "ask the operator and wait" — under unattended mode,
`decision-blocked` in `progress.md` with the reason **is** the stop; the
supervisor moves to the next ready card in `kanban.md` rather than idling the
bundle.

**PRESS ON — without asking, without recording `decision-blocked`:**

- This package's own stated figure or premise turns out wrong — correct it in
  place and continue. Correcting the brief/decisions/scope docs is expected,
  not insubordination (emit the correction event per "Retrospective log"
  above).
- The scope turns out larger than expected — no scope is too big to just do.
  Size alone is never a stop reason.
- A mechanical defect (duplicate module after a merge, stale fixture label,
  lint fix) — fix it and continue; this is what "Self-heal" below already
  covers for the cases it names.
- A routine judgment call with a conventional default — pick it, state it in
  the cycle receipt/`progress.md`, and move on.

See `decisions.md §24`.

## Eligibility

A cycle is eligible to fire when:

- The pre-launch checklist is fully green.
- All parent cards the cycle depends on are `complete`.
- The current `progress.md` corresponds to the operator-pinned branch tip.

## Self-heal

- A flaky test that fails once but passes on a clean re-run is annotated in the cycle record and not re-fired.
- A code-side identifier that leaks the `sd30_` pattern is renamed in-cycle (per the identifier-discipline doctrine).
- A cross-bundle reference that yields a missing-class / missing-monster error is filed as a blocker against the source bundle and the cycle pauses.

## Cross-bundle references

SD-30 references the following bundles:

- **SD-22 (closed):** APG + ACG + Bestiary 1 + DM toolkit. Reference is doctrinal read-only. Do not pull from `~/workspace/SD-22-...-*.md` files; pull from SD-22's repo canonical (`~/workspace/repos/codex/docs/release/SD-22/`).
- **SD-28 (planned):** Ultimate book content-source ingest. Class overlap (Occultist, Spiritualist, Medium, Mesmerist in Ultimate Intrigue) is canonical-to-SD-30; SD-28 references the canonical class id only.
- **SD-29 (planned):** Bestiary 2-3-4-5 content-source ingest. Monster overlap (occult monsters in later Bestiary books) is canonical-to-SD-30; SD-29 references the canonical monster id only.

## Decision record

See `decisions.md` for the running decision record. Each decision is dated, named, and stable.

## Per-bundle progress file

`~/workspace/programs/codex/requirements/SD-30-occult-and-companion-content-ingestion/progress.md` carries the per-cycle receipt. Do not use a shared chassis-lane progress file; each bundle's progress is its own.
