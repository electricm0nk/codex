# SD-29 Decisions

**Status:** Stub. Operator-pinned pending.

## Decision 1 — Book list

**Status:** Pending operator confirmation.

**Candidate:** Bestiary 2 + Bestiary 3 + Bestiary 4 + Bestiary 5.

**Source:** Operator message 2026-07-28 ("the beastiary books for 29"). Honcho context minimum: the four Paizo hardcover bestiary books after Bestiary 1 (which SD-22 took).

**Operator-pinned values needed when reviewing on a real computer:**

- Confirm the four books (2, 3, 4, 5).
- Confirm the per-book path locations under `src/rules_core/rules_tables/`.
- ~~Confirm the per-book entity count (each book has ~250-300 monsters; total ~1,000-1,200).~~
  **Withdrawn 2026-07-30.** Entity counts are no longer recorded in this package;
  they are derived (Decision 11). The estimate this line carried does not survive
  contact with the generator — Bestiary 5's PCGen dataset carries **no monster
  file at all**, so the four-book total is materially lower than "~1,000-1,200"
  and the shortfall is concentrated in one book rather than spread across four.
  See `scope-draft.md` §"Shape finding that affects this bundle's cycle plan".
  **The book list itself remains an operator decision and is not changed here.**
  **CORRECTED at package consolidation (2026-08-01) — old value → new value:**
  "~250-300 monsters each; ~1,000-1,200 total" → verified base `races.lst` row
  counts, re-derived against the PCGen checkout and documented in
  `forward-scope-register.md §1.3`: bestiary_2 **322**, bestiary_3 **261**,
  bestiary_4 **220** (the three books actually in scope here as
  per-monster-block cycles — 803 combined), bestiary_5 **0** (confirmed
  player-options only, no base `races.lst`). This package's own withdrawal note
  above correctly flagged the estimate as wrong but did not carry the
  replacement figure; `forward-scope-register.md` (the sibling package,
  consolidated into this directory) supplies it.

## Decision 2 — Branch and board

**Status:** Pending operator confirmation.

**Candidate:** `tranche/6-1` branch + `codex-tranche-6-1` board.

**Rationale:** SD-28 proposes `tranche/6` (no dash). SD-29 follows the SD-21 → SD-22 dash-1-sub-release pattern (SD-21 on `tranche/4-1`, SD-22 on `tranche/5`; the next two bundles would be `tranche/6` and `tranche/6-1`). Operator-pinned pending.

**Alternative:** SD-29 could split per-book across four sub-tranches (e.g., `tranche/6-b2`, `tranche/6-b3`, `tranche/6-b4`, `tranche/6-b5`). Operator preference.

## Decision 3 — Build version target

**Status:** Pending operator confirmation.

**Candidate:** `0.6.<build>` first concrete value.

**Rationale:** Same base digit as SD-28 because both packages land on the `tranche/6` family. Per the `<major>.<tranche-base>.<build>` scheme, tranche-base = 6 for `tranche/6` and `tranche/6-1`. Major stays `0` until first main-publish.

**Operator-pinned values needed:**

- Confirm the current build counter value (read from the version-bump contract in the repo's release workflow).

## Decision 4 — Epic structure

**Status:** Doctrine-of-record (per SD-22 doctrine).

9 epics / 30 criteria. Epic 1 = Code-Side Identifier Cleanup. Epic 2 = Operator Pre-Launch. Epic N = Closure Epilogue. Optional Epic 7 (DM Toolkit extension) per operator-pinned in-scope decision.

## Decision 5 — Cross-bundle monster overlap

**Status:** Doctrine-of-record (per SD-22 doctrine).

For monsters that appear in multiple Bestiary books (reprints of famous monsters), the canonical monster definition lives in whichever book first introduces the monster (typically the lowest-numbered Bestiary). Later bestiaries reference the canonical id.

**Boundary with SD-22:** Bestiary 1 monsters live canonically in SD-22. SD-29 references Bestiary 1's canonical id only; does not redefine.

## Decision 6 — Identifier discipline

**Status:** Doctrine-of-record (per SD-22 doctrine).

- Source-code identifiers describe WHAT the artifact does, NOT which release / spec domain it came from.
- PascalCase for functions / methods / constants / properties / Tauri commands.
- lowercase camelCase for variables.
- Forbidden patterns: `sd29_*`, `SD29_*`, `Sd29*`, `sd29-*`, `t_<hex>`, `SD-29-Ex...`, `AV-PAY-N`.
- Doctrine-of-record at `~/workspace/governance/identifier-discipline.md`.

## Decision 7 — Operating form [SUPERSEDED — see §23]

**Status:** Doctrine-of-record (per SD-22 doctrine); **superseded 2026-08-01** by Decision §23, which replaces the dispatch mechanism named here with the `Workflow` tool.

`/loop 60m /batch /goal <loop-instruction-file>`. Not ad-hoc single-task invocations.

## Decision 8 — Verification is `./scripts/verify.sh`, not a hand-composed run

**Status:** Doctrine-of-record (repo tooling, landed 2026-07-30 on `tranche/6`).

**Decision:** Every cycle verifies with `./scripts/verify.sh` (full, not `--quick`)
and captures its exit code directly. No cycle composes its own verification
command set.

**This supersedes `cargo test --workspace --locked` as a bundle-level check.**
The repo root has no `[workspace]` table, so `--workspace` from the root does
**not** reach `apps/desktop/src-tauri` — a separate, bin-only cargo crate that
shipped un-compilable twice under exactly that command. Three further
structural false-greens are documented in `scripts/verify.sh --help`:
`cargo test` fail-fasts (a "green" run had executed 124 of 488 suites); piping
a command to `grep`/`tail` returns the pipe's exit status, not the command's;
and the frontend runner reports `0/0 test files passed.` and exits `0` when
`node_modules` is absent.

`scripts/verify-baselines.env` holds the recorded green-tree numbers. Test
counts are floors, clippy warnings a ceiling. A baseline that has to move moves
in its own reviewable commit with `--show-actuals` output in the message. A
floor that dropped means tests were deleted — that is the finding, not the fix.

**Authority:** `scripts/verify.sh` (its `--help` is the rationale of record),
`scripts/verify-baselines.env`, `docs/governance/book-ingestion-playbook.md` §4.

## Decision 9 — The pre-ingest trap report is mandatory

**Status:** Doctrine-of-record (repo tooling).

**Decision:** Before any ingest code is written for a book, the cycle runs

```sh
cargo run --locked --bin v06_corpus_trap_report -- <book_dir>
```

and records the output in the cycle receipt. It runs against a book that has
never been ingested, which is the point.

**Why this is a decision and not a suggestion.** Every ingestion cycle so far
hit the *same* corpus traps — `.MOD` rows counted as declarations, `#`-disabled
rows read as live, archetype-qualified `KEY:`s merged with the base record they
share only a display name with — rediscovered by hand, by a different agent,
every time, and nearly every resulting count was wrong on the first pass (396
missing feats where 301 was real; 207 bonus-bearing where 166 was real; 180
`BONUS:VAR` records where 86 was real, one record having carried 66 tokens).

Particularly load-bearing for this bundle: **a shared display name never
implies a shared record.** Decision 5's cross-book overlap rule must join on
`KEY:`, and the trap report's per-book `KEY:` namespace listing is what tells
you the right prefix to search under — a grep for a bare leaf name returns zero
and reads, wrongly, as "this monster is not in this book".

`cargo run --locked --bin v06_corpus_trap_report -- --audit` is additionally a
definition-of-done condition: it exits `2` when an already-ingested record cites
a corpus line that does not resolve.

**Authority:** `src/pcgen_import/corpus_traps.rs` (the trap catalogue and the
corpus evidence for each), `src/bin/v06_corpus_trap_report.rs`.

## Decision 10 — The reach gate is a definition-of-done condition

**Status:** Doctrine-of-record (repo tooling). **Carries an open operator question — see below.**

**Decision:** A book's ingest cycle is not done until every one of that book's
record families reaches a player surface, proven by a claim in
`apps/desktop/src-tauri/src/reach_gate.rs` that executes the real IPC builder.
Ingestion and surfacing are one unit of work, not two.

A count does not satisfy the gate (`corpus_ingest_diagnostic` already carries
every book's record count and renders nothing), and an identifier alone does
not (that is the Feats-tab defect, where the player saw `feat:deflect_arrows`
in place of a name and description).

**Open operator question this bundle cannot decide for itself.** Bestiary 1's
41 ingested monster stat blocks reach no player surface today — the React app
contains no monster reference at all, and the Pets tab does not count because
its companion stat block is computed by `pilot_compute`, not read from these
tables. The recorded remedy is "a monster catalog command and browser,
mirroring `spell_catalog.rs` + SpellCatalogScreen.tsx". SD-29's epic structure
contains no such epic; the proposed Epic 7 (DM Toolkit extension) is the
nearest existing consumer. **The operator decides whether that surface lands
inside SD-29 or as a named prerequisite outside it; this package does not add
an epic on its own authority.** Skipping it is not available — the gate fails
the cycle either way.

**Authority:** `apps/desktop/src-tauri/src/reach_gate.rs` (`OPEN_FINDINGS`),
`docs/governance/book-ingestion-playbook.md` §3.

## Decision 11 — Per-entity counts are generated, never hand-maintained

**Status:** Doctrine-of-record (repo tooling).

**Decision:** This package records no per-entity count. `scope-draft.md`
§"Book shape" points at `cargo run --locked --bin v06_work_inventory` and the
`docs/work-inventory.json` it generates. Cycle receipts cite the command that
produced any figure they publish.

**Why.** Every hand-maintained artifact in this project has drifted and then
actively misled — a dashboard claimed 12 finished classes when 5 was true; a
coverage matrix read 1 wired feature where the code had 6; shipped deferral
strings still claim engines do not exist that do. This package's own
"~250-300 monsters each; total ~1,000-1,200" estimate is a worked example: it
is wrong, and it was wrong in a way that would have produced an epic with zero
cycles in it. **See Decision 1's consolidation-time correction above for the
verified replacement figures** (322 / 261 / 220 / 0 across bestiary_2-5).

**Authority:** `src/bin/v06_work_inventory.rs`, `docs/work-inventory.json`,
`docs/governance/book-ingestion-playbook.md` §6.

## Decision 12 — Build no execution engines [SUPERSEDED — see §19]

**Status:** Doctrine-of-record (scoping verdict, 2026-07-29); **superseded 2026-08-01** by Decision §19, which tightens the rule.

**Original text:** No cycle in this bundle builds an execution engine. If a cycle's
plan calls for RNG, opponent state or turn sequencing, the plan is wrong.
`docs/release/v0.6/execution-engine-scoping.md` is the verdict with the
evidence: not one of the 252 "no `<X>` engine exists" deferrals requires an
execution engine for the correct number to reach the player.

**Why superseded.** The original rule said "no engines" without distinguishing a
real-time engine (RNG, turn sequencing, opponent state) from a rules-data engine
(e.g., a 6d6 fireball posted as `6d6` with the caster level, not a runtime die-
roller). Operators can pre-compute numerical spell effects and class-feature
outcomes as data without building a real-time engine; the original rule forbade
both. §19 narrows the rule to forbid only the first kind while permitting the
second.

## Decision 13 — Branch and board (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.**

**Decision:** SD-29 launches on `tranche/9` branch with `kanban.md` + `progress.md` local-file dispatch (no Hermes board).

The prior candidate (per the 2026-07-28 stub) was `tranche/6-1` + `codex-tranche-6-1`; SD-29 takes its own tranche (`tranche/9`) parallel to SD-28's `tranche/8`, deliberately off the `tranche/6` family that carries SD-22's Bestiary 1 baseline and SD-23/24. The Hermes board is retired per operator directive 2026-08-01, applied uniformly to SD-28 and SD-29.

## Decision 14 — Build version target (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.**

**Decision:** SD-29's first concrete build value is `0.9.<build>`, where `<build>` is the current build-counter state at the time of cycle close.

Per the 2026-07-17 build-version amendment:
- **major** = 0 (no main-publish yet).
- **tranche-base** = 9 (the base digit of `tranche/9`).
- **build** = monotonic counter, never resets.

Tranche-promotion increments only on `tranche/9 → develop` PR. The closure Epic 8's value is `0.9.<last_build>`.

## Decision 14a — Hermes board retired (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.** Cross-cutting — affects Decision §7 (operating form), §13 above, and the loop-instruction pre-launch checklist.

**Decision:** SD-29 has no Hermes kanban board. The work-queue artifact is a local-file `kanban.md` paired with `progress.md`. Cycle dispatch reads `kanban.md` at top of each tick; supervisor's file-touch partition enforces 1-cycle-per-file.

## Decision 15 — Book list confirmed (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01** for SD-29's four-bestiary scope.

**Decision:** SD-29's book list is four bestiaries:

1. **Bestiary 2** — Paizo hardcover, per-monster-block cycles.
2. **Bestiary 3** — Paizo hardcover, per-monster-block cycles.
3. **Bestiary 4** — Paizo hardcover, per-monster-block cycles.
4. **Bestiary 5** — Paizo hardcover, **player-options dataset, not a monster dataset** (per the 2026-07-30 shape finding; corpus: `b5_races_pc.lst`, `b5_abilities_race*.lst`, `b5_feats.lst`, `b5_companionmods.lst`, with `_bestiary_5_for_players.pcc` — no `monster` records).

**Cycle-0 dependency.** Epic 2's pre-flight runs the trap-report against each bestiary and confirms the per-book shape from `cargo run --locked --bin v06_work_inventory`. Bestiary 5's race/feat/companion-mod records fall under a different ingest subtype than the per-monster-block cycles applied to Bestiary 2-4. SD-29 may run Bestiary 5 with per-race / per-feat / per-companion-mod cycles as alternative work, OR drop Bestiary 5 in favor of Bestiary 6 + Bonus Bestiary if the operator prefers (per the 2026-07-30 shape finding in scope-draft.md). Operator-pinned per-cycle at Epic 5/6 dispatch.

## Decision 16 — Cross-book conflict rule (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01** (same doctrine as SD-28 §16).

**Decision:** When SD-29's books conflict with another book (cross-bundle or cross-SD-N) on a record, the **newer book is doctrine and the older book is errata**.

This supersedes any prior cross-book conflict handling in the bundle. The class-grant overlap rule (canonical class definition lives in the bundle that owns the book's primary class definition; the other bundle references the canonical id only) is the only exception.

Reprint cross-bundle overlaps in Pathfinder bestiaries are common: a famous monster in Bestiary 2 frequently reappears in Bestiary 3-6 with wording changes. SD-29 records the per-monster conflict as a finding and applies the newer-book-wins rule.

## Decision 17 — Bulk modifications deferred (operator directive 2026-08-01)

**Status:** Operator-pinned, **forward-leaning acknowledgement.**

**Decision:** The per-cycle mode of operation (one record-at-a-time, file-touch partition, individual cycle receipts) is preserved for SD-29. Bulk-modification tooling is not in scope; a future retrofit (e.g., wiring a Bestiary 2 monster catalog of 200+ entries in one cycle-batch) is reserved outside this bundle.

## Decision 18 — Bestiary 5 shape-resolved differently from Bestiary 2-4 (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01** with cycle-0 trap-report + work-inventory gating.

**Decision:** SD-29 carries Bestiary 5 as one of its four books per scope-draft.md, but Bestiary 5's ingest type is **player-options** (race, feat, companion-mod records) rather than **monster blocks**. Epic 5 (Bestiary 5 ingest cycle) is gated on cycle-0 inventory + trap-report output. If the inventory surfaces zero `monster` units (consistent with the 2026-07-30 shape finding), Epic 5's cycle runs the per-race / per-feat / per-companion-mod cycles instead. Bestiary 6 + Bonus Bestiary are recorded in `successor-forward-scope-register.md C2.x` as drop-in replacements if operator prefers them over Bestiary 5.

This is an in-bundle resolution, not an out-of-bundle deferral — the work is in scope, the cycle shape adapts to what the corpus actually contains.

## Decision 19 — Reach gate is the definition of done (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.** **Supersedes Decision §12 (the prior "Build no execution engines" rule).**

**Decision:** A record's ingest cycle is **not done** until it satisfies `apps/desktop/src-tauri/src/reach_gate.rs`. Reach is the operator-visible definition of done.

**Engine policy.**

- **Real-time engines are out of scope.** No cycle in this bundle builds an RNG, opponent-state, or turn-sequencing engine.
- **Rules-data engines are in scope and often unnecessary.** When a numerical effect can be pre-computed as data (e.g., a monster's damage die dropping a `2d6` posted as `12` for a confirmed CR), post the calculated value in the description; the player rolls physical dice.
- **Engine construction is permitted only when strictly necessary to satisfy reach.** If a record's effect cannot be represented as data without an unjustifiable loss of fidelity, the cycle may build a small rules engine to model it. The engine must be enumerable, testable, and observable from `reach_gate.rs`.

**What this changes.** §12's blanket "no engines" rule was too coarse. §19 narrows it to real-time engines. Reach remains the gate; pre-computed values are preferred.

**Bestiary 1 surface gap.** Bestiary 1's 41 ingested monsters reach no surface today (per `reach_gate.rs OPEN_FINDINGS`). SD-29's monster-surface prerequisite is the Epic 7 DM Toolkit extension (operator-pinned at Epic 5/6 closure) or a Class 1 retrofit per `successor-forward-scope-register.md C3.x`. Cycles pause on `decision-blocked` if the surface is absent.

**Authority:** operator verbatim 2026-08-01: "reach gate is the definition of done, if an engine is required to get there, then we generate the engine — that said, often an engine isn't strictly necessary."

## Decision 20 — Operator ack-chain recorded (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01** as a forward-leaning ack chain.

**Ack ledger.** SD-29's twelve-item directive (operator 2026-08-01) confirmed: book list (B2-B5) confirmed with cycle-0 shape gating (Item 1); `tranche/9` and `kanban.md + progress.md` confirmed (Items 2-3); "correct" and "correct for now" items acked without specific directives (Items 4, 6, 8-11); cross-book conflict rule (Item 5); bulk-modifications deferred (Item 7); reach-gate-doD doctrine (Item 12, supersedes §12).

## Decision 21 — Cross-reference

- `./scope-draft.md` — committed scope shape, four bestiaries + Cycle-0 trap-report gating.
- `./loop-instruction.md` — per-cycle procedure; updated for `tranche/9`, no-Hermes-board, local-file dispatch.
- `./successor-forward-scope-register.md` — successor work depending on SD-29's output.
- `./kanban.md` — local-file work queue (replaces Hermes board).
- `./epic-breakdown.md` — 9-epic structure, Closure Epilogue fires LAST.
- `~/workspace/programs/codex/requirements/SD-22-.../decisions.md` — predecessor doctrine for the per-book ingest pipeline.
- `apps/desktop/src-tauri/src/reach_gate.rs` — definition-of-done surface for §19.
- `docs/governance/book-ingestion-playbook.md` — playbook of record. This bears on the
proposed Epic 7 — a DM toolkit that computes party CR and encounter budgets
from finished stat blocks is a calculator and is fine; anything that resolves
an encounter is not.

## Decision 22 — Unattended mode authorization (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.** Load-bearing for the bundle's cycle dispatch.

**Decision:** This bundle operates in **unattended mode**. The operator is out of town and may not see the harness's output for days. Cycles MUST NOT pause to ask the operator questions; the operator's review happens after return.

**Operating protocol during unattended mode (codified in `loop-instruction.md` §"OPERATING METHOD" sub-callout).**

1. **Default-and-flag, not ask.** When a cycle needs a decision, pick the safer default, capture it in `progress.md`, and continue.
2. **No `clarify` tool calls.** The operator clarification tool is hard-banned under unattended mode.
3. **Blockers are recorded, not raised.** Hard-blocks (auth failure, branch creation conflict, identity conflict on disk) go in `progress.md` with the command and exit code. The bundle does not halt; the supervisor picks up the next ready card.
4. **`decision-blocked` IS allowed.** Operator-decision points (Epic 7 DM Toolkit extension in-scope-vs-separate, Epic 5/6 closure operator-on-call) record `decision-blocked` in `progress.md` and proceed on the safe default per `successor-forward-scope-register.md C3.1` retrofit.
5. **Closure is a goal, not a stop signal.** The bundle runs to closure under the dispatcher's own loop (the `Workflow` tool per §23, not a human re-invoking a slash command per cycle).

**Operator's verbatim:** "include instructions to all 3 that indicate they will be running in unnattended mode since i will be out of town while this runs. They may not stop to ask questions - it might be days before i notice."

**Cross-reference:** the doctrine is mirrored in `loop-instruction.md` (cycle supervisor reads it first) and `progress.md` (per-cycle receipt confirms the operator-on-record). The receipt chain is the operator's after-return review surface.

## Decision 23 — Dispatch is session-driven `Workflow`-tool orchestration, not `/loop` or `/batch` (adopted from SD-27 `decisions.md §19`, 2026-08-01)

**Status:** Operator-pinned by inheritance — SD-27 `decisions.md §19` records the correction ("adopted from SD-26 `decisions.md §13`"); this package had not yet propagated it before this pass. No new operator input required; this is process alignment, not a new ruling.

**Decision:** SD-29 dispatches via the **in-harness `Workflow` tool, driven from a live session** — not `scripts/workflow-dispatch.sh` or any headless script, and not a cron driver. Deterministic control flow (per-epic ordering, fan-out, `decision-blocked` handling) is written into `loop-instruction.md` and tracked as state in `kanban.md`'s claim/complete queue; model judgment lives inside the dispatched `agent()`/`Workflow` calls, never in the orchestrating session's own tool calls.

`/batch` defaults to parallel isolated-worktree fan-out. It is used only where an epic's criteria are genuinely file-disjoint (see `loop-instruction.md` "Epic ordering"); where cycles touch shared state — `progress.md`, `kanban.md`, `reach_gate.rs`'s `OPEN_FINDINGS` — the correct dispatch is an explicit single-cycle procedure, not `/batch`. Any parallel wave that does run passes `isolation: 'worktree'` to every mutating agent (`docs/governance/loop-instruction-template.md §3`).

The orchestrating session never implements directly — it dispatches, verifies, and rules (`loop-instruction-template.md §2.2`). This held across SD-27's launch and the CRB run before it; nothing about SD-29's shape is an exception.

**Reasoning:** `loop-instruction.md`'s OPERATING METHOD callout (authored before this correction propagated) still named `/loop 60m /batch /goal ...` as the dispatch command. That form requires a human to re-type a slash command per invocation and cannot run headless — directly contradicting §22's unattended-mode authorization, which requires the bundle to run to closure across days with nobody watching. A `Workflow`-tool session, not a slash-command invocation, is what can actually satisfy that requirement.

**Consequence:** `loop-instruction.md`'s OPERATING METHOD callout now names the `Workflow` tool; §22 point 5 ("closure is a goal, not a stop signal") is corrected to read "under the dispatcher's own loop" rather than "per `/loop` cadence."

**Cross-reference:** `docs/release/SD-27-future-state-book-content-ingestion/decisions.md §19` (the adopted correction, itself from SD-26 `decisions.md §13`); `docs/governance/loop-instruction-template.md §2` (orchestration mode), `§2.1` (`RETRO_ACTOR`), `§2.2` (execution boundary), `§3` (worktree-isolation requirement for parallel waves).

## Decision 24 — A running retrospective log is part of the cycle procedure, not an optional courtesy (2026-08-01)

**Status:** Process alignment — the tooling already exists on `tranche/9` (`scripts/retro.py`, `docs/retro/schema.json`, `docs/retro/events/<actor>.jsonl`), and this package already reads the log as data in `forward-scope-register.md`. What was missing was the write side wired into the cycle procedure.

**Decision:** Every SD-29 cycle emits at least one retrospective event via `scripts/retro.py`. The event vocabulary (`correction`, `incident`, `near_miss`, `deferral`, `rework`, `verification`, `note`) and the field contract live in `docs/retro/schema.json` and are not re-specified here — read `python3 scripts/retro.py help <type>` for the real flags before emitting.

- Every dispatched agent has `RETRO_ACTOR` set to its role name (`loop-instruction.md` OPERATING METHOD callout, per `loop-instruction-template.md §2.1`). The harness has no variable that identifies an agent's role; the fallback (worktree directory name) names a checkout, not a role, which makes the by-actor breakdown in `scripts/retro.py summary` meaningless.
- `./scripts/verify.sh` auto-emits its own `verification` event on every run, passing or failing, so the denominator of "how often did we actually check" is honest without anyone deciding to record it.
- A `correction` event requires `--verified-by` — an unverified correction is a competing assertion, not a finding.
- Emitting the event is cycle step 8 (`loop-instruction.md` "Cycle mechanics"), not a follow-on task a cycle can skip under time pressure.

**Cross-reference:** `loop-instruction.md` §"Retrospective log"; `forward-scope-register.md`'s existing read-side usage of the same log; `scripts/retro.py`'s own `--help` docstring (do not hand-roll the emission syntax from memory).

## Decision 25 — Stop vs. press on: when a cycle halts and when it doesn't (2026-08-01)

**Status:** New — codifies a rule this bundle's `loop-instruction.md` "Hard stops" section applied implicitly (via its concrete instances) but never stated generally.

**Decision:** A cycle STOPS (records `decision-blocked` per §22's unattended-mode protocol, does not fabricate a pass) when:

- A gate fails for a reason that is a real finding about content or scope. Never weaken, skip, `#[ignore]`, or exclude a gate to get green, and never invent a surface or a number to satisfy one.
- Two authorities disagree on scope.
- The work would revert or clobber another session's live work.
- Proceeding would require inventing data not present in the corpus.

A cycle PRESSES ON, without recording `decision-blocked`, when:

- This package's own stated figure or premise is wrong — correcting it is expected, not insubordination.
- The scope is larger than expected — size alone is never a stop reason.
- A mechanical defect needs fixing (duplicate module, stale fixture, lint fix).
- A routine judgment call has a conventional default — pick it, record it, move on.

**Reasoning:** Under §22's unattended-mode authorization, the cost of stopping on the wrong things and the cost of pressing on through the wrong things are both real and asymmetric with a human days away. `decision-blocked` already gives cycles a way to stop without literally asking the operator; what was missing was a general rule for which situations qualify, so a cycle facing a case not on "Hard stops"'s concrete list still classifies it correctly.

**Cross-reference:** `loop-instruction.md` §"Stop vs. press on"; §22 (unattended-mode protocol, the mechanism a STOP actually invokes); `loop-instruction.md` "Hard stops" (this bundle's concrete STOP instances) and "Self-heal" (this bundle's concrete PRESS-ON instances).

## Decision 26 — Orchestrator model: Opus at low reasoning effort (operator directive 2026-08-01)

**Status:** New. **Checked first, per the operator's instruction:** this package named no orchestrator model anywhere before this pass — `decisions.md`, `loop-instruction.md`, and `scope-draft.md` had zero mentions of Sonnet, Opus, or reasoning effort. There is no prior "orchestration runs on Sonnet" statement to mark superseded; this decision is a fresh addition, not a correction.

**Decision:** The session driving this bundle's `Workflow`-tool orchestration (per §23) runs on **Opus, at low reasoning effort**. The operator observed that Opus at low reasoning effort produced materially better orchestration results than Sonnet at high reasoning effort, and pins this as the new normal for orchestration on this program.

This is a statement about the **orchestrating session only** — the session that dispatches, verifies, and rules per `loop-instruction-template.md §2.2`. It is not a blanket upgrade of every dispatched agent. Dispatched sub-agents keep task-matched tiers, unchanged by this directive:

- Cheap/mechanical work (housekeeping, lint fixes, release-notes/version-bump edits) → Haiku.
- Real implementation, debugging, and review → Sonnet.
- Adversarial verification / judge-panel steps → Opus.

`docs/governance/loop-instruction-template.md §2`'s "Default subagent model: Sonnet" is about dispatched *subagents*, a different role from the orchestrating session; it is not superseded by this decision and needs no correction.

**Mechanical caveat:** a session cannot change its own model mid-run. Setting the orchestrator to Opus at low reasoning effort is a **pre-launch operator step**, done before the cycle session starts (at the plan-approval prompt, or via `/model`), not an action a running cycle can take on itself.

**Cross-reference:** `loop-instruction.md` OPERATING METHOD callout (now names the orchestrator model); `decisions.md §23` (the `Workflow`-tool dispatch decision this pins the model for); `docs/governance/loop-instruction-template.md §2` (subagent tiering, unaffected).

## Decision 27 — A full code review is the bundle's final epic (operator directive 2026-08-01)

**Status:** New. The operator verified independently that zero files across SD-28/29/30 mentioned code review before this pass; the v0.6 CRB run closed without an end-of-run code review, and this corrects that gap going forward for all three bundles launching now.

**Decision:** Epic 10 (Bundle Code Review) is added as SD-29's last-numbered epic. Its dispatch slot is after every content-ingest epic (3-6), Epic 7 (DM Toolkit extension, if in scope) and Epic 9 (Build Version Numbering), and before Epic 8 (Closure Epilogue) — Closure Epilogue remains the true final step per `loop-instruction.md §"Epic ordering"` (unchanged by this decision), so any finding the review surfaces is fixed before the tranche-promotion PR (part of Epic 8) opens.

`./scripts/verify.sh` passing is a **precondition** for Epic 10 to fire, never the review itself: a green gate says the tests that exist pass, it says nothing about whether the code is right.

**Scope, at minimum:**

- Correctness of rules logic against the corpus (sampled, not exhaustively re-derived).
- No stubs or fixture-only data in production paths, per `docs/governance/no-stub-mvp-doctrine.md`.
- Content genuinely reaching a player surface, per `reach_gate.rs`'s `OPEN_FINDINGS` mechanism (spot-checked against the live IPC/UI path, not just the gate's exit code) — including the Epic 7 DM Toolkit consumer surface, if in scope.
- Test quality, not just count — per `docs/governance/book-ingestion-playbook.md §7.4`'s mutation-test pattern, a sample of new gates/tests is checked for a case that actually fails when the thing it protects is broken.
- No hand-authored rules data in the frontend (`apps/desktop/src/`).

**Mechanism — wired into what already exists, nothing invented fresh:** the review runs `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` (this bundle's standing per-cycle dual-audit) against the **whole-bundle diff**, not a single cycle's slice: `git diff origin/develop...HEAD` — the same merge-base triple-dot comparison both scripts already default to via `BASE_BRANCH=origin/develop`. No new grep/audit tooling is invented; Epic 10 reuses the standing per-cycle gates at bundle scope and adds the manual/agent-driven judgment a grep cannot do (corpus-correctness sampling, reach-claim spot-check, test-quality sampling).

**Findings are triaged, not auto-fixed.** Each finding records a severity and a disposition: `fixed-in-bundle` or `deferred`. A `deferred` finding names an owner (a person or a specific successor bundle) and lands in `successor-forward-scope-register.md` — an unowned deferral is not a valid disposition. Real defects are fixed in-bundle before Epic 8 fires.

**Operator escalation path, not a substitute:** the operator can separately trigger `/code-review ultra`, a multi-agent cloud review of the branch. That path is operator-triggered and billed — a cycle running under §22's unattended-mode protocol cannot launch it itself — so Epic 10 must stand on its own as the bundle's actual gate.

**Cross-reference:** `epic-breakdown.md` Epic 10; `acceptance-and-verification.md AT-29-013`; `docs/governance/no-stub-mvp-doctrine.md`; `docs/governance/book-ingestion-playbook.md §7.4`; `reach_gate.rs`; `kanban.md` card `epic-10-code-review`.
