# SD-29 Decisions

**Status:** Operator-pinned, confirmed 2026-08-01/02 (Decisions 13–34).

## Decision 1 — Book list [SUPERSEDED — see §34]

**Status:** Pending operator confirmation; **superseded 2026-08-02** by Decision §34, which pins the wider seven-book cut (Bestiary 2-6, Bonus Bestiary, Monster Codex) in place of this decision's four-book candidate and closes out its "Operator-pinned values needed" list below.

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

## Decision 2 — Branch and board [SUPERSEDED — see §§13, 34]

**Status:** Pending operator confirmation; **superseded 2026-08-01/02** by Decisions §13 (branch `tranche/9`, no Hermes board) and §34 (sequential launch: `tranche/9` cut from the post-SD-28 tip, not concurrently with `tranche/6-1`/`tranche/6` as this candidate assumed).

**Candidate:** `tranche/6-1` branch + `codex-tranche-6-1` board.

**Rationale:** SD-28 proposes `tranche/6` (no dash). SD-29 follows the SD-21 → SD-22 dash-1-sub-release pattern (SD-21 on `tranche/4-1`, SD-22 on `tranche/5`; the next two bundles would be `tranche/6` and `tranche/6-1`). Operator-pinned pending.

**Alternative:** SD-29 could split per-book across four sub-tranches (e.g., `tranche/6-b2`, `tranche/6-b3`, `tranche/6-b4`, `tranche/6-b5`). Operator preference.

## Decision 3 — Build version target [SUPERSEDED — see §§14, 34]

**Status:** Pending operator confirmation; **superseded 2026-08-01/02** by Decision §14 (first concrete value `0.9.<build>`) and §34 (the scope this build target now covers is the seven-book cut, not the four-book candidate this decision was scoped against).

**Candidate:** `0.6.<build>` first concrete value.

**Rationale:** Same base digit as SD-28 because both packages land on the `tranche/6` family. Per the `<major>.<tranche-base>.<build>` scheme, tranche-base = 6 for `tranche/6` and `tranche/6-1`. Major stays `0` until first main-publish.

**Operator-pinned values needed:**

- Confirm the current build counter value (read from the version-bump contract in the repo's release workflow).

## Decision 4 — Epic structure

**Status:** Doctrine-of-record (per SD-22 doctrine).

9 epics / 30 criteria (superseded: 13 epics per epic-breakdown.md — see §34). Epic 1 = Code-Side Identifier Cleanup. Epic 2 = Operator Pre-Launch. Epic N = Closure Epilogue. Optional Epic 7 (DM Toolkit extension) per operator-pinned in-scope decision.

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

**Supersession note (2026-08-01, dated).** The "open operator question" above is
answered. The `("beastiary1", "monsters")` arm of `reach_gate.rs` (`:986` as of
2026-08-10; was `:840` — line pins drift) carries an executed reach claim in
place of the old `OPEN_FINDINGS` entry (the comment directly above it records
the replacement); `monsters_reach()` (`:1300` as of 2026-08-10) exercises
`build_monster_catalog()` for real. The IPC command `list_monster_catalog` is
registered in `apps/desktop/src-tauri/src/main.rs:57,197`, and
`MonsterCatalogScreen.tsx` is routed via `CharacterHubPage.tsx:104-105`, reachable
from a "Browse Monster Catalog" button at `LandingScreen.tsx:353`. The
2026-08-01 `verify-reach-reissue` retro event records this as a live catalog
search, not a stub. Bestiary 1's monster-surface question this decision left
open is therefore closed; the surviving `OPEN_FINDINGS` entry related to this
decision is unrelated to monster surfacing
(`("beastiary1", "race_traits", ...)`, the Duergar Spell-Like-Ability-Invisibility
record, upstream-blocked on `monster_codex` — see §34). *(Corrected 2026-08-10:
"the one surviving entry" is no longer accurate — SD-28 closure added seven
`<book>/archetypes` entries (SD-28 `decisions.md §60`/`§63`), so `OPEN_FINDINGS`
now carries eight; the archetype surface belongs to SD-30's
class_feature/archetype bundle.)*

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

**Decision:** SD-29 carries Bestiary 5 as one of its books per scope-draft.md, but Bestiary 5's ingest type is **player-options** (race, feat, companion-mod records) rather than **monster blocks**. Epic 6 (Bestiary 5 ingest cycle) is gated on cycle-0 inventory + trap-report output. If the inventory surfaces zero `monster` units (consistent with the 2026-07-30 shape finding), Epic 6's cycle runs the per-race / per-feat / per-companion-mod cycles instead. Bestiary 6 + Bonus Bestiary are recorded in `successor-forward-scope-register.md C2.x` as drop-in replacements if operator prefers them over Bestiary 5.

This is an in-bundle resolution, not an out-of-bundle deferral — the work is in scope, the cycle shape adapts to what the corpus actually contains.

## Decision 19 — Reach gate is the definition of done (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.** **Supersedes Decision §12 (the prior "Build no execution engines" rule).**

**Decision:** A record's ingest cycle is **not done** until it satisfies `apps/desktop/src-tauri/src/reach_gate.rs`. Reach is the operator-visible definition of done.

**Engine policy.**

- **Real-time engines are out of scope.** No cycle in this bundle builds an RNG, opponent-state, or turn-sequencing engine.
- **Rules-data engines are in scope and often unnecessary.** When a numerical effect can be pre-computed as data (e.g., a monster's damage die dropping a `2d6` posted as `12` for a confirmed CR), post the calculated value in the description; the player rolls physical dice.
- **Engine construction is permitted only when strictly necessary to satisfy reach.** If a record's effect cannot be represented as data without an unjustifiable loss of fidelity, the cycle may build a small rules engine to model it. The engine must be enumerable, testable, and observable from `reach_gate.rs`.

**What this changes.** §12's blanket "no engines" rule was too coarse. §19 narrows it to real-time engines. Reach remains the gate; pre-computed values are preferred.

**Bestiary 1 surface gap — closed, 2026-08-01.** Bestiary 1's 41 ingested monsters reached no surface as of this decision's original authoring (per `reach_gate.rs OPEN_FINDINGS`); the monster catalog command and browser have since shipped (see §10's supersession note above — `reach_gate.rs:986` as of 2026-08-10, was `:840`; `monster_catalog.rs`, `MonsterCatalogScreen.tsx` via `LandingScreen.tsx:353`), so the gate's Bestiary-1-monster-surface prerequisite is satisfied independent of Epic 7. The Epic 7 DM Toolkit extension (operator-pinned at Epics 5 and 6 closure) or a Class 3 (C3.1) retrofit per `successor-forward-scope-register.md C3.1` remain live decisions, but neither is a monster-surface prerequisite any longer. Cycles record `decision-blocked` in `progress.md` and move to the next ready card if a genuinely blocking gap is found.

**Authority:** operator verbatim 2026-08-01: "reach gate is the definition of done, if an engine is required to get there, then we generate the engine — that said, often an engine isn't strictly necessary."

## Decision 20 — Operator ack-chain recorded (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01** as a forward-leaning ack chain.

**Ack ledger.** SD-29's twelve-item directive (operator 2026-08-01) confirmed: book list (B2-B5) confirmed with cycle-0 shape gating (Item 1); `tranche/9` and `kanban.md + progress.md` confirmed (Items 2-3); "correct" and "correct for now" items acked without specific directives (Items 4, 6, 8-11); cross-book conflict rule (Item 5); bulk-modifications deferred (Item 7); reach-gate-doD doctrine (Item 12, supersedes §12).

## Decision 21 — Cross-reference

- `./scope-draft.md` — committed scope shape, four bestiaries + Cycle-0 trap-report gating.
- `./loop-instruction.md` — per-cycle procedure; updated for `tranche/9`, no-Hermes-board, local-file dispatch.
- `./successor-forward-scope-register.md` — successor work depending on SD-29's output.
- `./kanban.md` — local-file work queue (replaces Hermes board).
- `./epic-breakdown.md` — 13-epic structure, Closure Epilogue fires LAST.
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
4. **`decision-blocked` IS allowed.** Operator-decision points (Epic 7 DM Toolkit extension in-scope-vs-separate, Epics 5 and 6 closure operator-on-call) record `decision-blocked` in `progress.md` and proceed on the safe default per `successor-forward-scope-register.md C3.1` retrofit.
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

**Status:** Process alignment — the tooling already exists on `tranche/8` (`scripts/retro.py`, `docs/retro/schema.json`, `docs/retro/events/<actor>.jsonl`) — `tranche/9` is not yet cut (per §34, it launches sequentially from the post-SD-28 tip) — and this package already reads the log as data in `forward-scope-register.md`. What was missing was the write side wired into the cycle procedure.

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
- Content genuinely reaching a player surface, per `reach_gate.rs`'s `OPEN_FINDINGS` mechanism (spot-checked against the live IPC/UI path, not just the gate's exit code) — including the Epic 7 DM Toolkit consumer surface, if in scope. Mechanically, this means driving the running desktop app via `apps/desktop/.claude/skills/run-desktop/driver.sh` and reading the value off a screenshot, per `loop-instruction.md`'s Definition of done item 8, with `RUN_DESKTOP_AGENT` set to a value unique to this review (`apps/desktop/.claude/skills/run-desktop/SKILL.md` §"Concurrent agents").
- Test quality, not just count — per `docs/governance/book-ingestion-playbook.md §7.4`'s mutation-test pattern, a sample of new gates/tests is checked for a case that actually fails when the thing it protects is broken.
- No hand-authored rules data in the frontend (`apps/desktop/src/`).

**Mechanism — wired into what already exists, nothing invented fresh:** the review runs `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` (this bundle's standing per-cycle dual-audit) against the **whole-bundle diff**, not a single cycle's slice: `git diff origin/develop...HEAD` — the same merge-base triple-dot comparison both scripts already default to via `BASE_BRANCH=origin/develop`. No new grep/audit tooling is invented; Epic 10 reuses the standing per-cycle gates at bundle scope and adds the manual/agent-driven judgment a grep cannot do (corpus-correctness sampling, reach-claim spot-check, test-quality sampling).

**Findings are triaged, not auto-fixed.** Each finding records a severity and a disposition: `fixed-in-bundle` or `deferred`. A `deferred` finding names an owner (a person or a specific successor bundle) and lands in `successor-forward-scope-register.md` — an unowned deferral is not a valid disposition. Real defects are fixed in-bundle before Epic 8 fires.

**Operator escalation path, not a substitute:** the operator can separately trigger `/code-review ultra`, a multi-agent cloud review of the branch. That path is operator-triggered and billed — a cycle running under §22's unattended-mode protocol cannot launch it itself — so Epic 10 must stand on its own as the bundle's actual gate.

**Cross-reference:** `epic-breakdown.md` Epic 10; `acceptance-and-verification.md AT-29-013`; `docs/governance/no-stub-mvp-doctrine.md`; `docs/governance/book-ingestion-playbook.md §7.4`; `reach_gate.rs`; `kanban.md` card `epic-10-code-review`.

## Decision 28 — The display-value discriminator: compute the number, don't build the subsystem (operator directive 2026-08-01)

**Status:** Operator-pinned 2026-08-01. **Refines §19** — it does not supersede it. §19 says engines only when strictly necessary; this decision is the concrete test for "necessary", because §19's wording alone did not stop SD-27 deferring work that needed no engine at all.

**Decision:** A record whose rules text states a value **derived from data the engine already holds** — class level, an ability modifier, BAB, racial HD, or a constant defined on the same corpus row — is **display-value work, not engine work**. Compute the number and render it. Do not build the subsystem its noun implies.

**Operator verbatim (2026-08-01):**

> "You do not need a full blown engine for things like uses per day. You just need the ability to calculate the value that is displayed in the description or elsewhere in the UI. For example if you can do something x+y minutes a day where x = the class level and y = the attribute modifier, do the math. Maybe you get a boost from a feat - do the math. These are all just display values."

**The test, applied per record:**

| the record says | the inputs are | verdict |
|---|---|---|
| "usable %1 times per day", `%1` = class level + Cha mod | already computed | **display value — do the math** |
| "%1 rounds per day", `%1` = a same-row `DEFINE:`/`BONUS:VAR` constant | on the row itself | **display value — transcription, not interpretation** |
| a value that changes only with level/ability/feats already modelled | already computed | **display value** |
| an effect requiring expenditure, per-encounter state, or turn sequencing to be *correct* | not held anywhere | engine — and §19's "strictly necessary" bar applies |

**"Uses per day" is the canonical false positive.** It sounds like a resource-tracking subsystem — a pool, expenditure, a rest cycle. It is not. Displaying *"4 times per day"* requires the arithmetic and nothing else. Tracking how many a player has spent is a separate feature nobody asked for.

**What this cost SD-27, measured.** Multiple agents deferred PU class features and ARG feats as "blocked on engine dimensions that do not exist — SLA uses/day, luck budgets, fly manoeuvrability, companion levels." Every one of those is a display value. Once the discriminator was applied, PU class features went **41 → 58 of 64 accounted for (29 → 46 strict, then 52 hand-audited)** in a single pass, with zero regressions. `Unchained Rogue ~ Debilitating Injury` is the sharpest case: agents deferred it as "carries no numeric token" — true — while `rogue_features::prose_derived` **had already computed it and nothing consumed it.** The arithmetic existed; the display did not.

**Boundary, so this is not read as a licence to interpret.** `decisions.md §24.1` (SD-27) still forbids a general `BONUS:`/`DEFINE:`/`PREREQ:` formula interpreter. Reading a constant off the row that defines it, or substituting an already-computed value into display text, is **transcription**. Evaluating an arbitrary expression is **interpretation**. One unresolved case in SD-27 marks the line exactly: `Halfling ~ Adaptable Luck`'s second argument is `Halfling_AdaptableLuck_Bonus-1` — arithmetic on a variable, not a literal — and it remains open pending an operator ruling rather than being guessed. See SD-29 `forward-scope-register.md §7.2`.

**Cycle obligation.** Before deferring any record as "needs an engine", state **which input the engine does not have**. If every input is already computed, it is display-value work and the cycle does it. A deferral that names no missing input is not a deferral; it is unfinished work with a label on it.

**Authority:** operator directive 2026-08-01 (verbatim above); refines §19; bounded by SD-27 `decisions.md §24.1`; evidence in `docs/retro/tranche-7-retrospective.md`.

## Decision 29 — The four architectural traps SD-29 inherits from SD-27 (2026-08-01)

**Status:** Carried forward from tranche/7. Cross-cutting — each trap fires **per record**, not per book.

**Decision:** SD-29 inherits four architectural traps recorded in SD-27 `decisions.md §29`. They are **cited, not restated** — §29 is the authority and must not be allowed to drift. Each is named here because SD-27 hit every one *after* the work looked done, so the cost was rework rather than discovery.

| trap | SD-27 § | the rule |
|---|---|---|
| **Two compute twins** | §29.1 | `pilot_compute.rs` vs `pilot_compute_corpus.rs`; the character sheet reads the **corpus** twin. **A magnitude is not wired until it moves on the twin the player reads.** 15 of SD-27's 115 corrections were this class — wired into the hardcoded twin, tested green, changed nothing on screen. Use the shared seam (`feat_derived_pillar_contributions`) rather than adding a direct `feat_effects::` call to a pillar function; a structural test forbids it. |
| **A third twin, in TypeScript** | §29.2 | Any surface re-deriving a rules number instead of rendering an engine `explanations` row. Flat-footed AC lived only in `CharacterSheet.tsx` and broke PF1's dodge-denial rule. One live instance remains (`CharacterSheet.tsx:2945`). **If a number is computed in the view, it is unguarded.** |
| **Reach-gate blind spots, one permanent by construction** | §29.3 | `scanned_inventory()` reads `pub const NAME: &[Type]` slices; §24-shaped hand-modelled pure functions emit no slice and **can never be seen by a source scan**. **No content family may rest on a single discovery source** — the corpus directory is load-bearing. SD-27's gate passed 11 tests without ever asking about ARG's headline content. |
| **`p.xx` is a placeholder, not a page** | §29.4 | **Checked per row, never per content-kind.** 143 of SD-27's 175 trait rows carried `p.xx`; verbatim transcription would have manufactured 143 false citations. Generalising from the one book that had real pages is how this bites. |

**Why a decision and not a note.** All four were discovered *after* a passing test claimed the work was complete. They are not defects to fix once; they are shapes that recur per record, so a cycle that has not read §29 will reproduce them.

**Process half, same origin (`docs/retro/tranche-7-retrospective.md`):**

- **One writer per tree**, each with its own `CARGO_TARGET_DIR`, deleted when the cycle ends. 10 of SD-27's 34 incidents were shared-tree collisions — the largest single incident class — and an eleventh (rival bundle taxonomies) occurred during the retrospective itself. Never share a target dir between a worktree and the working tree: cargo serves the wrong tree's artifacts and yields a plausible wrong number.
- **`FILES YOU OWN` must be closed under the change it mandates.** Four SD-27 briefs named a scope narrower than the fix they demanded, forcing agents to breach scope or ship half a defect.
- **Every figure in a dispatching brief ships with the command that produced it.** Dispatching briefs were the largest single source of corrected claims in SD-27 — **41 of 115 (35.7%)** — and only 6 of 41 were caught before implementation began.
- **A verification stage red for more than one run is a blocker, not a background condition.** SD-27's `root-full` was red on 29 of 33 full runs, and that steady redness concealed that both of its own parity gates had never executed once.

**Already registered for this bundle.** `forward-scope-register.md §7.7` names the same four traps against SD-29's own records, and §7.1–§7.6 carry the measured corrections behind them (83 deferrals not 74; ledger row 03 open, not closed; seven rows understating progress; §2.3 being one third of a defect; the magnitude-predicate blocker; and the shared-ownership rule with SD-28 and SD-30). This decision is the doctrinal form of that register section — **read §7 for the per-record detail.**

**Authority:** SD-27 `decisions.md §29` (traps), `§30` (paths and artifacts), `docs/retro/tranche-7-retrospective.md` (measurements).

## Decision 30 — `<SameRowVar> ± <integer literal>` is transcription, not interpretation (resolved 2026-08-01)

**Status:** Operator-directed, resolved pre-dispatch. Closes ledger row 03 / `forward-scope-register.md §7.2`, which was recorded CLOSED while open. **Refines the §24.1 boundary; does not widen it.**

**The question was:** SD-27 `decisions.md §24.1` forbids a general `BONUS:`/`DEFINE:`/`PREREQ:` formula interpreter, and draws the line as *"reading a constant off the row that defines it is transcription; evaluating an arbitrary expression is interpretation."* `Halfling ~ Adaptable Luck`'s second `DESC:` argument is `Halfling_AdaptableLuck_Bonus-1` — a same-row variable minus an integer literal. Which side is it on?

### 30.1 The census that decides it

Every `DESC:` argument across the entire ingested corpus, classified:

| shape | count | status |
|---|---:|---|
| bare same-row variable (`Halfling_AdaptableLuck_Times`) | 24 | already resolved as transcription |
| `PRE`-family gate clause (`!PREABILITY:1,CATEGORY=…`) | 144 | already resolved — a gate, not a value |
| **arithmetic expression** | **1** | `Halfling_AdaptableLuck_Bonus-1` — this row, and nothing else |

**The population is one.** `§24.1` guards against an *unbounded* interpreter — a general evaluator that must handle arbitrary nesting, precedence and variable scope, and whose wrong answers are plausible numbers nobody checks. That risk does not arise from a closed form with a single instance.

### 30.2 The ruling

**`<SameRowVar> <+|-> <integer literal>` is transcription and is permitted.** Bounded to exactly that form:

- the left operand must be a variable **defined on the same row** (the existing `same_row_vars` rule);
- the operator is `+` or `-`;
- the right operand is an **integer literal**, not a variable, not a further expression;
- **anything else is refused, not approximated** — no nesting, no second operator, no variable on the right, no multiplication or division.

Implementations must **pin the refusal**, not just the acceptance: a test asserting that a form outside this grammar yields no number rather than a guess. That guard is the load-bearing half — it is what keeps this ruling from becoming an interpreter by increments.

### 30.3 Why this is the honest reading

The engine already resolves two things `§24.1` accepted as transcription: same-row constants (`same_row_vars`) and same-row constant *comparisons* (`eval_prevar_gate`, which evaluates `PREVAR*` gates by comparing two same-row values). **Comparing two constants is a strictly larger operation than subtracting a literal from one.** Ruling this out while permitting those would draw the boundary somewhere the existing code has already crossed.

The alternative — leaving it — is not neutral. The row **ships to players today** reading *"…if they choose to do so afterward, they only gain a bonus."* A sentence with the number deleted is not a conservative outcome; it is a wrong one, and `decisions.md §29.4`'s discipline (never manufacture, never silently drop) cuts against it.

### 30.4 What implementing it takes

`src/bin/ingest_race_traits_arg.rs` — extend `desc_prose`'s argument resolution to the grammar in §30.2, add the refusal test, regenerate `data/corpus/advanced_race_guide/race_trait/`, and re-pin the rendered sentence. The rendered text becomes *"…they only gain a +1 bonus"* at base, moving with the character's ARG luck feats exactly as the other two segments already do (SD-27 proved that path: `Three → 4 → 5`, `+2 → +4`).

**Scope: one row of 156, one binary, one regeneration.** It is a cycle task, not a bundle.

### 30.5 Ledger correction

`forward-scope-register.md §4.3` records row 03 as CLOSED, grouped with row 64. **Row 64 is closed; row 03 is this row and was not.** It was closed on its neighbour's evidence — the same generalise-from-one-sample shape `decisions.md §27.2` already records. §7.2 caught it; this decision resolves it.

**Authority:** operator directive 2026-08-01 ("resolve… before SD-30 starts", extended to this open ruling); bounded by SD-27 `decisions.md §24.1`; census derived by command over `data/corpus/**`.

## Decision 31 — The magnitude predicate: `source_record` on `ComputationExplanation` (2026-08-01)

**Status:** Cross-bundle. **Whichever of SD-28 / SD-29 / SD-30 dispatches first lands it; the other two consume it.** Blocks any coverage ratio any of the three publishes.

**The defect.** There is no stable predicate for *"this record carries a computed magnitude."* Magnitude rows carry no corpus key — only `detail` prose that *usually* repeats the record's name — so the question is answered today by substring-matching prose. Four reasonable variants of that match returned **48 / 49 / 51 / 52 on one unchanged tree** during SD-27, and every one was correctly derived. The tranche published 23, 32, 35, 46, 49, 51 and 52 for one property in a single session; each was a different *predicate*, not a different tree.

**Why it blocks all three bundles.** SD-28, SD-29 and SD-30 will each want a "% of records that reach a player" figure. Without a shared predicate, all three publish numbers that are individually defensible and **mutually incomparable** — and incomparable with SD-27's. A reader cannot tell progress from a change of definition.

**The fix, and it is small.** `ComputationExplanation` (`src/rules_core/pilot_compute.rs:209`) is `{ id, value, detail }`. Add an optional fourth field naming the corpus record the row came from:

```rust
pub struct ComputationExplanation {
    pub id: String,
    pub value: i16,
    pub detail: String,
    /// The corpus record key this magnitude was derived from, when it came
    /// from one. `None` for rows computed from the chassis rather than a
    /// record. Makes "does record X carry a magnitude?" a lookup instead of
    /// a prose substring match.
    pub source_record: Option<String>,
}
```

Then the predicate is a set membership test — exact, reproducible, and identical across bundles — instead of a regex over English.

**Ruling on how it lands:**

1. **Additive and optional.** `None` on every existing row; no call site is forced to change in the same commit.
2. **Populate it where a row is derived from a corpus record**, starting with the families the bundle is measuring. A partially-populated field still gives an exact answer for the populated part, which prose matching never does.
3. **Publish no coverage ratio until the families in that ratio are populated.** A ratio over a partially-populated field is the same defect wearing a new field name.
4. **State the predicate beside every ratio** regardless (`forward-scope-register.md §7.5`, and the dispatching-brief rule *"a ratio ships with its predicate"* now in `AGENTS.md`).

**This is SD-27 `decisions.md §27.1` recurring one layer up** — *625 mentions vs 271 settings; the arithmetic was never the defect, the label was.* There the label was ambiguous between two readings of one file; here it is ambiguous between four readings of one tree.

**Authority:** operator directive 2026-08-01; evidence in `docs/retro/tranche-7-retrospective.md` and `forward-scope-register.md §7.5`; cross-referenced from `../SD-28-ultimate-book-content-ingestion/forward-scope-register.md §C4.3` and `../SD-30-class-feature-archetype-bundle/forward-scope-register.md §C4.5` (renamed 2026-08-10 from `SD-30-occult-and-companion-content-ingestion`).

## Decision 32 — The "only writer" premise was false, and it lived in SD-27, not here (2026-08-01)

**Status:** New, correcting a premise this bundle would otherwise inherit silently rather than restating something already true here.

**Where the premise actually lives.** A search across all three `docs/release/SD-2[89]-*`/`SD-30-*` packages for `only writer` / `sole writer` returns **zero hits**. The premise, and its correction, both live in **SD-27**'s own `decisions.md §28` (`docs/release/SD-27-future-state-book-content-ingestion/decisions.md:499`), dated 2026-07-31: *"There is no concurrent cycle to collide with; this branch is the only writer."* That line justified lifting SD-27's own §8 file-touch partition on the premise that v0.6 had closed and nothing else was writing the tree. `docs/retro/events/size-modifier-agent.jsonl` records the same-day correction verbatim: *"decisions.md 28 declared 8's file-touch partition spent on the stated premise that 'this branch is the only writer' -- that premise is false in practice."* Ten of the tranche's 34 logged incidents (29%, retrospective §4.1) trace to exactly this false premise, four of them `git stash` swallowing a sibling's uncommitted work.

**Relationship to Decision 29's "one writer per tree" bullet, above.** That bullet already carries the process rule (own `CARGO_TARGET_DIR`, never shared between a worktree and the working tree). It is **cited here, not restated** — this decision adds what Decision 29 did not: naming exactly where the false premise text lives, and the staging/stash/preflight mechanics the premise's absence requires.

**Why it matters here even though the text is SD-27's, not this bundle's.** SD-28, SD-29 and SD-30 are three concurrently-launched bundles sharing one checkout and branch, each dispatched from a session that can itself be running alongside sibling sessions on the same box. The SD-27 mistake — asserting sole-writer status because no *specific, currently-known* concurrent bundle is active — reproduces immediately if this bundle assumes the same about SD-28, SD-30, or a human operator's own parallel session on the identical checkout.

**Ruling: the file-touch partition is necessary, not sufficient, and this bundle's own version of it must not rest on a sole-writer claim.**

- **Other writers exist, or may exist, concurrently — always.** No cycle in this bundle may assert sole-writer status as grounds for skipping a concurrency check; a partition is a courtesy between cooperating writers, not a lock.
- **`git status --porcelain` runs before every git write**, in every cycle, regardless of whether the cycle believes itself to be the only writer. A file listed that this cycle did not modify is a stop condition, reported per "Hard stops," never silently overwritten or attributed to this cycle's own change.
- **Staging is always explicit-path:** `git add <file> <file> ...`. Never `git add -A` or `git add .` — a wildcard add cannot distinguish this cycle's own changes from a sibling's uncommitted work sitting in the same tree.
- **`git stash` is never run, under any circumstance, in this repo.** The bare form stashes the *entire* working tree, not a subdirectory or a cycle's own changes, and has already destroyed a sibling's uncommitted work multiple times in this program (four of the ten shared-tree incidents above). To capture a HEAD baseline for comparison, use `git show HEAD:<path>` into a scratch file, or a separate `git worktree add` — never stash.
- **Any parallel *mutating* wave dispatches each agent with `isolation: 'worktree'`** — already required for cross-bundle/cross-epic concurrency by the OPERATING METHOD callout in `loop-instruction.md`; this decision confirms the same rule covers this bundle's own multi-book fan-outs (Epics 3-6, plus 7 if in scope), not only collision with SD-28/SD-30.

**Authority:** `docs/retro/tranche-7-retrospective.md` §4.1 and §6.1 (rules A1/A2); SD-27 `decisions.md §28` (where the false premise and its correction actually live — cited, not restated); `docs/retro/events/size-modifier-agent.jsonl` (the correcting incident, verbatim).


## Decision 33 — Automated disk reclamation is part of the cycle, not a manual afterthought (2026-08-01)

`docs/retro/tranche-7-retrospective.md` §4.1 records disk exhaustion as this program's **second-largest recorded orchestration failure mode — 5 of 34 logged incidents** (`/tmp` tmpfs at 91% → `ld terminated with signal 7 [Bus error]`, 20 minutes lost; `/` at 91%, 98%, 98%; `/home` at **100% used, 0 bytes available**, with "30+ per-agent `CARGO_TARGET_DIR`s under `~/.cache` totalling >600G, many 18-35G each," 25 minutes lost). The retrospective's own diagnosis is the design constraint this decision closes: *"The rule shipped in the brief; the matching `rm -rf` did not."* `AGENTS.md` and this bundle's own concurrency rules (Decision 32, above) correctly mandate a per-agent, per-source-tree `CARGO_TARGET_DIR` and tell agents to delete it when they finish — but nothing ever enforced or automated that deletion, so it did not happen at the rate the rule needed.

Two additions, landed in `scripts/` (shared across SD-28/SD-29/SD-30, not per-bundle code):

- **`scripts/reclaim.sh`** — dry-run by default; `--apply` required to delete anything. Four categories: abandoned `CARGO_TARGET_DIR`s (found under the Claude scratchpad root and this repo's `$HOME/.cache/codex-*` convention, confirmed by directory *shape* — `.rustc_info.json`/`debug`/`release` — not merely the presence of `CACHEDIR.TAG`, which fontconfig/uv/man-db also write and which a naive check flagged as a false positive on this script's own first dry run); stale `scripts/verify.sh` log directories; git worktrees whose branch is merged into `develop` or whose PR is closed/merged (`git worktree list --porcelain` + `gh pr list`); and local branches merged or gone from origin. Safety: never touches a target dir a live `cargo`/`rustc` process is using (checked via kernel-reported `comm` and `/proc/<pid>/environ`/`cwd`, not a self-matching `pgrep -f` — the self-match trap named explicitly in the brief that produced this script); never removes a worktree with uncommitted changes or unpushed commits; never touches this repo's own checkout or the `pcgen` oracle clone; never runs `git stash`. Emits a `retro.py incident` event (`recurrence-key disk-full`) whenever `--apply` actually reclaims something.
- **`scripts/verify.sh`'s new `preflight-disk` stage** — first in *both* the `--quick` and full stage sets, so it fails loudly and points at `reclaim.sh` **before** the ~490-binary `root-full` build starts, rather than only recording pressure after the fact the way the script's existing `emit_disk_pressure_event` (post-run, informational) already did.

**This bundle's `loop-instruction.md` Cycle mechanics now runs the preflight check at the start of each cycle and `scripts/reclaim.sh --apply` at cycle end.** The mandate is paired with the command, which is the entire lesson of §4.1 restated as a rule: a rule with no executable counterpart is the rule that produced 600G.

**Authority:** `docs/retro/tranche-7-retrospective.md` §4.1 (disk exhaustion, 5 of 34 incidents) and §6.1 rule A4 (`CARGO_TARGET_DIR` deletion + pre-sweep disk check); `AGENTS.md` "Concurrency and Measurement."

## Decision 34 — Scope width and planning-readiness resolved (operator directive 2026-08-02)

**Status:** Operator-pinned, **confirmed 2026-08-02.** Supersedes Decision §1's four-book list; resolves the "Pending operator confirmation" status carried by Decisions §2 and §3; resolves both of `README.md`'s "operator must resolve" sections ("Unresolved: planning-readiness" and "Unresolved: scope width (4 books vs. 7 books)").

**Decision:** SD-29's scope is the **wider seven-book cut** — Package B's `forward-scope-register.md` §0/§1.1 scope, not Package A's four-book `scope-draft.md` list:

1. **Bestiary 2** — 322 base `races.lst` rows; per-monster-block cycles.
2. **Bestiary 3** — 261 base `races.lst` rows; per-monster-block cycles.
3. **Bestiary 4** — 220 base `races.lst` rows; per-monster-block cycles.
4. **Bestiary 5** — 0 monsters; player-options dataset (races, feats, companion-mods).
5. **Bestiary 6** — 63 units total (22 class_feature, 13 race_trait, 2 spell, 26 companion); **zero monsters** — player-options, same shape as Bestiary 5.
6. **Bonus Bestiary** — 34 units total (3 class, 17 race_trait, 14 monster); 4 `.lst` files including `bb_races.lst`.
7. **Monster Codex** — 213 units total (72 class_feature, 32 feat, 24 spell, 45 equipment, 4 equipment_modifier, 19 race_trait, 15 companion, 2 monster); 18 `.lst` files + `support/`.

All seven books are present in `docs/work-inventory.json` as `future_state`. Base-race-declaration row counts for Bestiary 2/3/4 (322/261/220) plus the other four books' base-row counts re-derive `forward-scope-register.md §1.3`'s 819 seven-book total.

**Planning-readiness adjudicated.** `README.md`'s "Unresolved: planning-readiness" section recorded Package A (`canonical: true`, `status: planning-ready`) and Package B (`scope pass only, not planning-ready`, awaiting sign-off on the register) as contradictory. That contradiction is resolved: Package B's register is signed off, its seven-book scope stands, and the existing chassis (`scope-draft.md`, `decisions.md`, `epic-breakdown.md`, `kanban.md`, and the rest) is **planning-ready at seven-book width** — the chassis is widened in place, not re-authored from a blank sign-off cycle.

**Launch order is sequential, not concurrent.** SD-28 (`tranche/8`) runs to closure first. `tranche/9` is cut from the **post-SD-28 tip** — i.e., from `develop` after SD-28's tranche-promotion PR merges — not from `tranche/8` mid-flight, and SD-29 launches only from that point.

**What this leaves standing.** Decisions §13 (branch `tranche/9` + local-file dispatch), §14 (build version target `0.9.<build>`), §14a (Hermes board retired), and §16–§33 (cross-book conflict rule, engine policy, orchestration doctrine, retrospective/disk-reclamation process) are unaffected by the scope-width change and continue to govern the widened seven-book bundle. Decision §15 ("SD-29's book list is four bestiaries") is narrower than this decision and is superseded by it for book-count purposes, though its per-book shape descriptions for Bestiary 2-5 remain accurate and are extended, not replaced, by the three additional books above.

**Authority:** operator directive 2026-08-02; verified corpus figures per `forward-scope-register.md §1.1` and `§1.3`; `docs/work-inventory.json` (all seven books present as `future_state`).

## Decision 35 — Bestiary 1's "closed" foundation is ~4% proven; measured against the live dashboard, cross-referenced to SD-28's harness epic (2026-08-02)

**Status:** New. Reconciliation pass against `/home/ubuntu/swarm-observer/PF1e-dashboard.json` `work_inventory` (`generated_at: 2026-08-02T12:40:01Z`), done in response to an operator directive that all previously-started and Ultimate books reach 100% proven, which exposed that this bundle's premises need re-stating in measured terms rather than corrected outright — the numeric claims already in this package (41 ingested monster stat blocks, Bestiary 5/6 zero monsters) turned out to be accurate; what needed fixing was the framing.

**Measured starting state.** Command:

```
python3 -c "
import json
d = json.load(open('/home/ubuntu/swarm-observer/PF1e-dashboard.json'))
b = {x['id']: x for x in d['work_inventory']['books']}['bestiary']
print(b['units'], b['proven'], b['by_status'])
for r in b['reconciliation']: print(r)
"
```

Result: `bestiary` (Bestiary 1) is **42 proven of 1,027 units — 4.1%** (`by_status`: `grounded: 42`, `ingested-magnitude: 4`, `not-ingested: 981`). Per-kind reconciliation shows the monster kind specifically at **41 engine records of 326 declared** (`delta: 285`) — the figure this package already cites at `scope-draft.md:132` ("Bestiary 1's 41 ingested monster stat blocks reach no surface") is correct and is not being changed. What this decision corrects is the repeated shorthand elsewhere in this package — `README.md:224`, `README.md:250`, `scope-draft.md:112`, `technical-design.md:100` — that calls Bestiary 1 **"(closed in SD-22)"** without qualification. "Closed" describes SD-22's own procedural sign-off, not the corpus's proven state; read alongside this bundle's dependency on Bestiary 1 as a foundation, the unqualified phrase invites the false inference that Bestiary 1 is a complete, finished base. It is not: 96% of its units (985/1,027) are unproven. Each of those four locations is annotated in place with a pointer to this decision rather than rewritten, since none of them asserted the 100%-complete claim outright — the risk was omission, not misstatement.

**Dependency stated plainly.** SD-29's per-book work (Bestiary 2-6, Bonus Bestiary, Monster Codex) references Bestiary 1's canonical monster ids (`decisions.md §"Boundary with SD-22"`) but does not itself need Bestiary 1 fully proven to proceed — cross-book identity is a reference, not a completion gate (`successor-forward-scope-register.md:39`). What SD-29 does inherit from Bestiary 1's low proven rate is the same measurement ceiling: **`proven` = `grounded` + `text-complete` only**, excluding `ingested-magnitude` (`status_vocabulary`: *"The engine holds the record WITH its real numeric fields, but this generator observes no consumer delta for this kind (spells, equipment)"*). SD-29's own in-scope books carry meaningful spell/equipment surface (Monster Codex: 24 spell + 45 equipment + 4 equipment_modifier units of its 213), so those units cannot reach `proven` regardless of ingestion quality until the harness is widened. That widening is **SD-28 Epic 14** (`docs/release/SD-28-ultimate-book-content-ingestion/decisions.md §"Decision — new, recording an operator directive..."`, added 2026-08-02, commit `3eb11a18`), a hard dependency of SD-28's own Epics 23/25/28 and, by the same generator mechanism, a prerequisite for this bundle's spell/equipment-bearing books ever reading 100% proven in the dashboard even after ingestion is done.

**Bestiary 5/6 zero-monster finding — independently re-verified, not merely re-cited.** Decision §34 already states Bestiary 5 (0 monsters) and Bestiary 6 (63 units, zero monsters) correctly. Independent verification against the corpus:

```
ls ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_5/
ls ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_6/
```

Both directories contain only companion/PC-race, feat, spell, deity/domain, and skill `.lst` files (e.g. `b5_races_companion.lst`, `b5_races_pc.lst`, `b6_races_companion.lst`) — no monster-statblock `.lst` of the shape Bestiary 1/2/3/4 carry. Confirmed: **no scope error exists in Decision §34's seven-book list** — Bestiary 5 and 6 are correctly scoped as player-options ingestion, not monster ingestion, and the "monster" language in this package's title/framing should be read book-by-book per Decision §34's per-book shape notes, not assumed uniform across all seven.

**Launch-readiness assessment.** This package's chassis (scope, epics, decisions) is internally accurate and was already planning-ready per Decision §34. It is **not launch-ready in the sense of "the dependency it builds on is a finished base"** — it depends on a Bestiary 1 foundation that is 96% unproven by the same measure this bundle will itself be judged by, and its own spell/equipment-bearing books (Monster Codex) cannot reach 100% proven until SD-28 Epic 14 lands. Sequential launch order after SD-28 (Decision §34) is the correct mitigation already in place; this decision makes the reason explicit rather than assumed.

**Authority:** `/home/ubuntu/swarm-observer/PF1e-dashboard.json` `work_inventory` section, `generated_at: 2026-08-02T12:40:01Z`; `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_5/` and `bestiary_6/` directory listings (2026-08-02); `docs/release/SD-28-ultimate-book-content-ingestion/decisions.md` (E13-E30 completion epics, Epic 14 harness-widening decision, commit `3eb11a18`).

## Decision 36 — SD-29 is partitioned by *kind*, not by *book*; and `file_kind()`'s correction invalidates this package's `race_trait` figures (2026-08-10)

**Status:** New. Written on `tranche/8` from SD-28's session findings, at operator request
("build that into the sd-29 package"). Two coupled changes: a partitioning change (this
decision) and a data correction that forces it (below).

### 0. Reproduce every figure here

```bash
cd ~/workspace/repos/codex
python3 - <<'PY'
import json, collections
U = json.load(open('docs/work-inventory.json'))['units']
for b in ['bestiary','bestiary_2','bestiary_3','bestiary_4','bestiary_5',
          'bestiary_6','bonus_bestiary','monster_codex']:
    us = [u for u in U if u.get('book') == b]
    if us: print(f'{b:16}', len(us), dict(collections.Counter(u.get('kind') for u in us)))
PY
```

### 1. The data correction that forces this

SD-28 `§61` replaced `v06_work_inventory.rs`'s `file_kind()` filename-substring typing with
row-content classification, and added `Kind::MonsterAbility`. **Every `race_trait` figure in this
package predates that and is wrong.** Bestiary 1 went `620 race_trait` → **21 `race_trait` +
523 `monster_ability`**; the `_abilities_race.lst` files are monster special-ability libraries,
not racial traits. `§35`'s 4.1%-proven measurement of Bestiary 1 also predates the change.

Live per-book, per-kind state for this package's seven books:

| book | units | monster | monster_ability | race_trait | companion | other |
|---|---|---|---|---|---|---|
| bestiary (B1) | 951 | 330 | 523 | 21 | 59 | 18 |
| bestiary_2 | 974 | 316 | 466 | 162 | 16 | 14 |
| bestiary_3 | 1,194 | 261 | 40 | 799 | 85 | 9 |
| bestiary_4 | 1,218 | 220 | 768 | 86 | 76 | 68 |
| bestiary_5 | 165 | **0** | 39 | 63 | 57 | 6 |
| bestiary_6 | 59 | **0** | 13 | 0 | 26 | 20 |
| bonus_bestiary | 34 | 14 | 17 | 0 | 0 | 3 |
| monster_codex | 207 | **2** | 3 | 14 | 15 | 173 |

**Epics 3-6 and 11-13 are sized per book on assumptions this table contradicts.** Monster Codex
has 2 monsters and 68 `class_feature` units; Bestiary 5 and 6 have no monsters at all (`§34`'s
zero-monster finding, now visible as a kind distribution rather than a footnote); Bestiary 3 is
799 `race_trait` while Bestiary 4 is 768 `monster_ability`. There is no representative book.

### 2. Why kind is a better partition than book

Measured in SD-28: the per-book ingest tax is **per-file per-kind and constant regardless of
record count** — UM's 26 equipment records paid the same fixed sweep cost as UPsi's 439. Across
seven books a per-book partition pays that tax up to seven times per kind; per kind, once.

Book was never a real territory either. Each book has its own data table, but the contested
resource is the shared sweep files — so two workers on different books collide on the same
seventh file. The file structure already partitions by kind almost cleanly:

```
KIND-SCOPED (disjoint lanes)   feat_*.rs · equipment_resolver.rs + */equipment_tables.rs
                               */race_tables.rs · */[class]_spell_list.rs
SHARED (chokepoints, 4 files)  v06_work_inventory.rs · reach_gate.rs
                               corpus_ingest_diagnostic.rs · v06_content_state_dump.rs
```

Corpus hazards also transfer by kind, not by book — `.MOD` unconditional recovery (SD-28 `§46`),
`.MOD` conditional variant (`§48`), never-join (`§49`), `.COPY=` aliasing in two sub-shapes
(`§58`). Each was discovered in one kind and *rediscovered* in the next. A per-kind pass
front-loads that discovery once per kind rather than per book.

### 3. The chokepoint, measured — and smaller than first claimed in one half, larger in the other

```bash
for f in src/bin/v06_work_inventory.rs src/bin/v06_content_state_dump.rs \
         apps/desktop/src-tauri/src/reach_gate.rs \
         apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs; do
  printf '%-42s pins=%s book-literals=%s\n' "$(basename $f)" \
    "$(grep -cE 'assert_eq!\([^,]+, *[0-9]{2,}|== *[0-9]{3,}|: *[0-9]{3,}' $f)" \
    "$(grep -coE '"(core_rulebook|advanced_players_guide|advanced_class_guide|ultimate_[a-z]+|bestiary|pathfinder_unchained)"' $f)"
done
```

| file | count-pins | book-name literals |
|---|---|---|
| v06_work_inventory.rs | 7 | 59 |
| v06_content_state_dump.rs | 1 | 22 |
| reach_gate.rs | 7 | 42 |
| corpus_ingest_diagnostic.rs | 12 | 20 |
| **total** | **27** | **143** |

The original framing (hardcoded *counts* are the chokepoint) was wrong. Counts are 27 assertions
and demonstrably cheap to derive — SD-28 landed two such derivations in one sitting each
(`equipment_keys` in `646aea2b`, `equipment_catalog_books()` in `a68a4538`), and both caught a real
regression automatically within the hour. **The roster is the real work: 143 book-name literals,
load-bearing in `match` arms and dispatch tables, not merely assertions.**

### 4. Ruling

SD-29 runs as **kind lanes**, not book epics, with the prerequisite explicitly *not* bundled in:

1. **Lane structure.** One writer per kind lane; lanes run concurrently. Within a lane, fan out
   per-book extraction and serialize only the table landing and sweep. This needs no refactor and
   can start immediately.
2. **Derive the 27 count-pins** as a follow-on (small, proven twice).
3. **The 143-literal roster derivation is NOT a prerequisite** and must be scoped on its own
   evidence. It touches `reach_gate.rs` and the classifier — the two files most likely to produce a
   long tail — and calling it a prerequisite would smuggle a real project into a planning note.
4. **Run a grammar/hazard pass at the head of each lane** before ingestion, producing the
   enumeration up front (SD-28 `§49`, `§58`).

### 5. Open questions, not resolved here

- **Provenance.** Per-book receipts currently carry the OGL/licensing story. Kind lanes need a
  different provenance record, and licensing is not a place to improvise. **Blocking for lane 1.**
- **Cross-book KEY collisions** become easier to catch (all books in one kind examined together)
  but the check must move out of the per-book slice. It caught real duplication in three SD-28
  books; do not lose it in the move.
- **Epics 3-6 and 11-13 need re-cutting** along lanes, and `§35`'s Bestiary 1 baseline needs
  re-measuring post-`§61`. Neither is done here.
- `monster_ability` (1,869 units across these seven books) has **no ingest path and no engine
  table** — it is a new kind as of SD-28 `§61`, not an existing one with a gap.

## Decision 37 — The kind-lane re-cut, executed (operator directive 2026-08-10, supersedes the `corpus-work-channels.md §9.4` deferral)

**Status:** New. Operator directive, verbatim (2026-08-10): *"The SDs are our bodies of work. If
our plan for SD-29 needs to be completely rescoped to address something else, then that is where
we need to make those updates. After the PR is merged, we will start SD-29 in tranche/9. That
needs to be defined and recorded."*

**This decision executes `§36`** (SD-29 is partitioned by kind, not by book) and **supersedes the
deferral recorded in `../corpus-work-channels.md §9.4`**, which held the re-cut back pending 9.1–9.3
settling. The operator's directive above is the settling: re-cut now, in the package itself, not in
a further analysis document.

### 37.0 Every figure re-derived, not transcribed

```bash
cd ~/workspace/repos/codex
python3 - <<'PY'
import json, collections
U = json.load(open('docs/work-inventory.json'))['units']
books = ['bestiary_2','bestiary_3','bestiary_4','bestiary_5','bestiary_6','bonus_bestiary','monster_codex']
tot = collections.Counter()
for b in books:
    for u in (x for x in U if x.get('book') == b):
        tot[u.get('kind')] += 1
for k, c in tot.most_common(): print(k, c)
print('total', sum(tot.values()))
PY
```

Result — SD-29's seven books (Bestiary 2-6, Bonus Bestiary, Monster Codex; **Bestiary 1 excluded,
it is SD-22's**), by kind:

| kind | units | share |
|---|---:|---:|
| monster_ability | 1,346 | 35.0% |
| race_trait | 1,124 | 29.2% |
| monster | 813 | 21.1% |
| companion | 275 | 7.1% |
| class_feature | 90 | 2.3% |
| spell | 82 | 2.1% |
| equipment | 65 | 1.7% |
| feat | 32 | 0.8% |
| race | 12 | 0.3% |
| equipment_modifier | 9 | 0.2% |
| class | 3 | 0.1% |
| **total** | **3,851** | |

Every one of these 3,851 units is `not-started` — SD-29's own seven books have zero ingested
content of any kind (verified by the same query, adding `u.get('status')` to the counter; omitted
above for brevity, re-run with that addition to reproduce). This matters for Decision 36's
"monster is Channel B" framing below: the "path exercised" evidence is corpus-wide, from Bestiary
1 (SD-22's book, not SD-29's) — within SD-29's own scope, monster is exactly as unstarted as
`monster_ability`.

### 37.0.1 Correcting `corpus-work-channels.md §4`'s own arithmetic

That document's "SD-29's 7 books" table (§4) reads `monster_ability 1,869`, `monster 1,143`,
`race_trait 1,145`, `companion 334`. **Those four numbers are the *eight*-book sum, including
Bestiary 1**, not the seven-book sum:

```bash
python3 -c "
import json, collections
U = json.load(open('docs/work-inventory.json'))['units']
books8 = ['bestiary','bestiary_2','bestiary_3','bestiary_4','bestiary_5','bestiary_6','bonus_bestiary','monster_codex']
tot = collections.Counter()
for b in books8:
    for u in (x for x in U if x.get('book') == b): tot[u.get('kind')] += 1
for k in ['monster_ability','monster','race_trait','companion']: print(k, tot[k])
"
# → monster_ability 1869, monster 1143, race_trait 1145, companion 334 — matches §4 exactly
```

Bestiary 1 is not one of SD-29's seven books (`decisions.md §"Boundary with SD-22"`, `§34`); it is
referenced only for canonical monster ids. The corrected, in-scope figures are the ones in §37.0
above (`monster_ability` 1,346, not 1,869; `monster` 813, not 1,143; `race_trait` 1,124, not 1,145;
`companion` 275, not 334). This does not change any of `corpus-work-channels.md`'s rulings — the
channel assignments, the merge ruling (§9.2), and the defect-fix-alongside ruling (§9.3) are about
*kind*, not about a specific book count — but every lane size in this decision uses the corrected
seven-book number, cited by the command above, not the analysis document's number.

### 37.1 The re-cut: kind lanes replace book epics

Epics 3-6 and 11-13 (Bestiary 2, 3, 4, 5, 6, Bonus Bestiary, Monster Codex as seven separate
per-book epics) are **retired**. `epic-breakdown.md` is rewritten in full with an 11-epic
structure. New epic numbering (full detail in `epic-breakdown.md`):

| Epic | Name | Basis |
|---|---|---|
| 1 | Code-Side Identifier Cleanup | unchanged |
| 2 | Operator Pre-Launch | unchanged, corpus-wide (not per-4-book) cycle-0 shape gate |
| 3 | **Provenance Gate — PI-Screening for Kind-Lane Ingestion** | **new — see §37.3** |
| 4 | **Monster / Monster-Ability Chassis Lane** | merged per `corpus-work-channels.md §9.2`; 2,159 units (813 monster + 1,346 monster_ability); pilot-then-extend, see §37.2 |
| 5 | **Race-Trait Lane** | mechanism-build + defect-fix-alongside per `corpus-work-channels.md §9.3`; 1,124 units |
| 6 | **Companion Lane** | mechanism-build, no path anywhere in the corpus; 275 units |
| 7 | **Residual Proven-Path Content Lane** | Channel A/B kinds with a settled method: spell (82), equipment (65), feat (32), race (12), equipment_modifier (9), class (3) = 203 units. `class_feature` (90 units) is **excluded from this lane** — see §37.4 |
| 8 | DM Toolkit extension | was Epic 7; gated on Epic 4's pilot + extension landing, not on all lanes |
| 9 | Build Version Numbering | was Epic 9 |
| 10 | Bundle Code Review | was Epic 10 |
| 11 | Closure Epilogue | was Epic 8; **fires LAST**, unchanged position, renumbered |

Sequencing: `E1 → E2 → E3 → {E4, E5, E6, E7} (file-disjoint per kind, run concurrently) → E8 (gated) → E9 → E10 → E11`.

### 37.2 Monster + monster_ability: pilot-then-extend, not seven parallel books

Per `corpus-work-channels.md §5.3` and this brief's own instruction: extend the monster path
**deliberately** — one book end-to-end before committing to the rest, to get a real per-book cost
the way SD-28 got the archetype class-two delta (`§63`). Applied to SD-29's own seven books (all
of which are `not-started` — §37.0):

**Pilot book: Bonus Bestiary.** Smallest total footprint of any monster-bearing SD-29 book — 34
units (14 monster + 17 monster_ability + 3 class), verified above. Epic 4's first cycle-batch runs
the full chassis-plus-features build against Bonus Bestiary alone, reach-gated, before Epic 4's
remaining cycle-batches (Bestiary 2: 316 monster + 466 monster_ability; Bestiary 3: 261 + 40;
Bestiary 4: 220 + 768; Monster Codex: 2 + 3) are dispatched. Bestiary 5 (0 monster, 39
monster_ability) and Bestiary 6 (0 monster, 13 monster_ability) carry `monster_ability` with no
`monster` chassis in the same book — the lane still owns their `monster_ability` units; they are
scheduled after the pilot confirms the mechanism, same as the monster-bearing books.

**No representative book — verified, not assumed:**

```bash
python3 -c "
import json, collections
U = json.load(open('docs/work-inventory.json'))['units']
for b in ['bestiary_2','bestiary_3','bestiary_4','bestiary_5','bestiary_6','bonus_bestiary','monster_codex']:
    us=[u for u in U if u.get('book')==b]
    print(b, 'monster=', sum(1 for u in us if u['kind']=='monster'), 'monster_ability=', sum(1 for u in us if u['kind']=='monster_ability'), 'race_trait=', sum(1 for u in us if u['kind']=='race_trait'))
"
```

Confirms: Monster Codex has **2** monsters (of 207 total units — its weight is `class_feature`/
`feat`/`spell`/`equipment`); Bestiary 5 and 6 have **0** monsters; Bestiary 3 is 799 `race_trait`
of 1,194 units (mostly race-trait, not monster); Bestiary 4 is 768 `monster_ability` of 1,218
(mostly monster-ability). No book stands in for the other six.

### 37.3 Provenance — resolved for OGL/attribution, gated for PI-screening

`corpus-work-channels.md §6` marked provenance **"Blocking before the first channel runs."** The
license matrix (`docs/governance/license-matrix.md`, commit `314a7ad9`, 37 books, operator-
authorized 2026-08-10 explicitly "ahead of the move from book-scoped to kind-scoped ingestion
packages") is checked against that blocker rather than assumed to satisfy it:

**OGL / attribution / publisher provenance — satisfied for all seven SD-29 books.** Every one of
Bestiary 2, 3, 4, 5, 6, Bonus Bestiary and Monster Codex has, per the matrix's per-book table: a
real `OGL.txt`, an active `.pcc` `COPYRIGHT:` block, `ISOGL:YES`, and section-15 attribution
recoverable from `OGL.txt`. All seven are Paizo Inc., no Ultimate-Combat-shaped missing-file case
among them. **This closes the per-book-receipt half of the provenance question the old book-epic
structure carried implicitly** — a lane-scoped cycle can cite the matrix once for a book's
OGL/attribution status instead of re-discovering it per cycle.

**PI-screening — NOT satisfied, and this is the part that gates the first lane.** The matrix's
central finding is that `rules_tables/*.rs` (**Pipeline B — the exact pipeline every SD-29 kind
lane writes into**) has **zero files anywhere in the repo that call `pi_screening`,
`PI_BLACKLIST_TERMS`, or `classify_field`**, and a direct sweep already found three real, unredacted
Product-Identity hits reaching committed source in Pipeline B tables from other bundles (`Sarenrae`
in ACG's archetype table, `Asmodeus` in ARG's, a `Jarn` re-leak in ACG's spell list). All seven of
SD-29's books are marked **`unscreened`** for Pipeline B in the matrix's per-book table. This is not
a theoretical risk for a kind-scoped SD-29: `monster_ability` alone (1,346 units) is exactly the
kind of prose-bearing content (special-attack/special-quality descriptive text) most likely to
carry an unredacted deity name or NPC name, the same shape as the three leaks already found.

**Ruling: a new Epic 3 (Provenance Gate — PI-Screening for Kind-Lane Ingestion) gates every content
lane.** Epic 3 does not re-litigate OGL/attribution (closed, cite the matrix) and does not fix the
pre-existing Pipeline B leaks in other bundles' tables (out of this package's write scope — SD-28
and SD-30 own those books). It requires, before Epic 4/5/6/7's first content commit: (a) a
per-lane PI-blacklist sweep (`pi_screening.rs`'s 55-term list, or its `classify_field` call wired
into the lane's extraction step) run against that lane's own newly-generated content before it
lands in `rules_tables/`; (b) the sweep's output recorded in the lane's first cycle receipt; (c) any
hit treated as a hard stop for that record (per `loop-instruction.md` "Stop vs. press on" — a
gate failing for a real content finding is a STOP, not a thing to route around). See
`epic-breakdown.md` Epic 3 for the acceptance criteria and `acceptance-and-verification.md`
AT-29-003a.

### 37.4 `class_feature` — the one kind explicitly not folded into a lane

SD-29's 90 `class_feature` units are Channel D per `corpus-work-channels.md §3`/`§5.4`: blocked
behind the archetype mechanism and per-class chassis (SD-28 `§60`/`§63`), sizing funded corpus-wide
(`§9.1`) but not yet measured for **these specific classes**. Folding them into the Epic 7 residual
lane (which is scoped to kinds with a *settled, proven* method) would misrepresent 90 units as
ready-to-ingest content when they are not. **Disposition: excluded from every SD-29 lane, tracked
in `successor-forward-scope-register.md` as inheriting SD-28's per-class archetype-measurement
funding (`§9.1`) once it reaches the classes these 90 units belong to.** Not a silent drop — see
`epic-breakdown.md` Epic 7's scope note and the successor register entry.

### 37.5 What does not change

The seven-book list (`§34`), `canonical_branch: tranche/9`, `build_version_target: 0.9.<build>`,
sequential launch after SD-28 (`§34`), unattended-mode operating protocol (`§22`), and every
decision `§1`-`§36` not explicitly named above are unaffected by this re-cut. This decision changes
**partitioning** (kind lanes, not book epics) and **provenance gating** (a new Epic 3); it does not
change scope, branch, version scheme, or launch order.

**Authority:** operator directive 2026-08-10 (verbatim above); `decisions.md §36` (the partitioning
ruling this executes); `../corpus-work-channels.md` §§3-10 (channel analysis, superseded-as-deferral
only at §9.4); `docs/governance/license-matrix.md` (commit `314a7ad9`, provenance evidence);
`docs/work-inventory.json` (all figures re-derived above, commands included).

## Decision 38 — SD-29 becomes the corpus-wide lane bundle; the seven-book boundary is retired (operator directive 2026-08-10)

**Status:** New. Supersedes the seven-book boundary set by `§34` and carried through `§36`/`§37`,
and supersedes the deferral recorded at `../corpus-work-channels.md §9.4` a second time (§37 already
superseded it for the *partitioning* question; this decision supersedes it for the *book-list*
question `§9.4` also deferred). Operator directive, verbatim: *"What I'm really after is
establishing lanes that we can use to rapidly catch up all the books in parallel — both those we
have touched and those we have not touched."*

**What changes.** `§37` re-cut SD-29 into kind lanes but kept lane scope pinned to the seven
bestiary-line books (`§37.5`: "The seven-book list (`§34`) ... unaffected by this re-cut"). That
seven-book boundary was a constraint imposed while re-cutting the *partitioning*, not a ruling the
operator made about the *book list*. The operator has now ruled directly: SD-29's lanes run
**corpus-wide** — every book that carries units of a lane's kind, whether that book has ever been
touched by any prior bundle or not. The directory is renamed
(`docs/release/SD-29-corpus-wide-catch-up-lanes/`, via `git mv`, history preserved) to stop the
package's own name from re-asserting the retired boundary.

### 38.0 Every figure re-derived here, not transcribed from the brief that requested this decision

The brief driving this decision arrived with its own corpus totals and per-kind figures, flagged as
"my derivation, to be verified and corrected" — the brief's author noted the last two SD-29 briefs
both carried wrong figures. Re-derived independently below; discrepancies from the brief are called
out, not silently absorbed.

```bash
cd ~/workspace/repos/codex
python3 - <<'PY'
import json, collections
d = json.load(open('docs/work-inventory.json'))
U = d['units']
HELD = {'grounded','text-complete','ingested-magnitude'}
PROVEN = {'grounded','text-complete'}
total = len(U)
held = sum(1 for u in U if u.get('status') in HELD)
proven = sum(1 for u in U if u.get('status') in PROVEN)
print('corpus total', total, 'books', len(set(u['book'] for u in U)))
print('held (grounded+text-complete+ingested-magnitude)', held, round(100*held/total,1),'%')
print('proven (grounded+text-complete only)', proven)
print('untouched (total-held)', total - held)
PY
```

**Result: 38,536 units, 38 books.** Held (`grounded`+`text-complete`+`ingested-magnitude`) = **8,414
= 21.8%**. Proven (`grounded`+`text-complete` only) = 2,253. Untouched (total − held) = **30,122**.
The brief's corpus-wide figures (38,536 / 8,414 / 21.8% / 30,122) check out exactly.

*(Denominator note, 2026-08-10 audit: the tables in §38.0–§38.1 run over the full 38-book corpus
including `beginner_box` (19 units), while the product scope is 37 books / 38,517 units —
`beginner_box` is the sole exclusion per `../corpus-work-channels.md §10.2`. The 19-unit delta is
noise at lane scale and no table is re-derived for it; cycle-batches size from live re-derived
counts over in-scope books, not from this snapshot.)*

**The measurement caveat the brief names is real and re-verified.** `equipment` and `spell` have
zero `grounded` units relative to their `ingested-magnitude` population is false as stated —
`equipment` does have 133 `grounded` — but the *shape* is correct: both kinds are held almost
entirely at `ingested-magnitude` (equipment 4,638 of 5,064 held; spell 1,067 of 1,089 held), because
the generator observes no consumer delta for these kinds per `status_vocabulary.ingested-magnitude`
in `work-inventory.json` itself. Reporting `proven` alone for these two kinds would understate
progress by roughly 38x (equipment) and 49x (spell). Every figure below states which measure
(`held` or `proven`) it is using.

### 38.1 Per-kind figures, re-derived — corrections to the brief

```bash
python3 - <<'PY'
import json, collections
d = json.load(open('docs/work-inventory.json'))
U = d['units']
HELD = {'grounded','text-complete','ingested-magnitude'}
byk = collections.defaultdict(collections.Counter)
for u in U: byk[u['kind']][u.get('status')] += 1
for k in ['equipment','feat','spell','equipment_modifier','monster_ability','companion','race_trait','monster','class_feature']:
    c = byk[k]; tot = sum(c.values()); held = sum(c[s] for s in HELD)
    print(k, 'total', tot, 'held', held, 'remaining', tot-held)
PY
```

| kind | total | held | remaining |
|---|---:|---:|---:|
| equipment | 6,227 | 5,064 | 1,163 |
| feat | 2,610 | 1,260 | **1,350** |
| spell | 2,843 | 1,089 | 1,754 |
| equipment_modifier | 1,580 | 768 | 812 |
| monster_ability | 3,107 | 0 | 3,107 |
| companion | 1,683 | 0 | 1,683 |
| race_trait | 3,456 | 44 | 3,412 |
| monster | 1,270 | 46 | 1,224 |
| class_feature | 15,472 | 109 | 15,363 |

**One correction to the brief: `feat` remaining is 1,350, not 1,348.** The brief's 1,348 is the
count of `not-ingested` + `not-started` feat units only; it silently drops 2 `feat` units sitting at
`deferred-with-reason` (a real, distinct status per `status_vocabulary` — a claim-blocking engine
diagnostic named these two units, not an absence of any attempt). `1,163`/`1,754`/`812` (equipment,
spell, equipment_modifier remaining) and the 44/1,270 (monster held/total) and 0/1,683 (companion)
and 0/3,107 (monster_ability) all check out exactly against the brief.

**`class_feature`: 15,472 units confirmed, 40.2% of the 38,536-unit corpus** (the brief said "40%",
correct to one significant figure). 109 held, 15,363 remaining. Stays out of scope — §38.4 below.

### 38.2 Bestiary 1 — confirmed in scope, no longer owned by anyone

```bash
python3 -c "
import json, collections
U = json.load(open('docs/work-inventory.json'))['units']
us = [u for u in U if u['book']=='bestiary']
print(len(us), collections.Counter(u.get('status') for u in us))
"
# → 951 {'not-ingested': 901, 'grounded': 46, 'ingested-magnitude': 4}, proven = 46/951 = 4.8%
```

**Confirmed exactly: 951 units, 46 proven (4.8%), 901 `not-ingested`.** Bestiary 1 was `§37`'s
explicit exclusion ("Bestiary 1 excluded, it is SD-22's") because the seven-book cut only ever
covered Bestiary 2 onward. SD-22 is closed; no bundle currently owns Bestiary 1's remaining work.
Under corpus-wide lanes this ceases to be a question — Bestiary 1 is simply one more book with units
of `monster` (284 remaining), `monster_ability` (523 remaining), `race_trait` (21 remaining, per
SD-28 `§61`'s `file_kind()` correction already applied to this figure) and `companion` (59
remaining), and each lane picks it up the same way it picks up any other book that carries units of
that lane's kind. No separate epic, no separate receipt track, no book-boundary decision required.

### 38.3 The lane structure — corpus-wide, three tiers

**Tier 1 — proven-path, day-one parallel, no mechanism needed.** Method is settled (SD-28 landed
seven books of feats and four of equipment through it); every book with remaining units of these
kinds — touched or untouched — can run today:

| lane | held | remaining | books w/ remaining units |
|---|---:|---:|---:|
| equipment | 5,064 | 1,163 | re-derive at cycle-0 per book |
| feat | 1,260 | 1,350 | ″ |
| spell | 1,089 | 1,754 | ″ |
| equipment_modifier | 768 | 812 | ″ |

**Tier 2 — mechanism-build-then-sweep.** No working ingest path exists yet corpus-wide for these
kinds (or the existing path is defective); each needs its mechanism built once, then swept across
every book that carries the kind — pilot-then-extend per `../corpus-work-channels.md §5.3`, not
seven-book-parallel and not corpus-wide-parallel from cycle one:

- **Monster / Monster-Ability chassis** (merged per `../corpus-work-channels.md §9.2` — monsters are
  playable, so chassis + features is one system, the `race`/`race_trait` shape): `monster` 46 held /
  1,224 remaining across 14 books with remaining units; `monster_ability` 0 held / 3,107 remaining
  across 24 books. Combined 4,331 remaining units. **Pilot: Bonus Bestiary** (14 monster + 17
  monster_ability = 31 units, carried forward from `§37.2`'s reasoning — the smallest *non-degenerate*
  combination, i.e. neither count near zero). Corpus-wide, two smaller pairs exist —
  `occult_adventures` (1 monster + 3 monster_ability) and `monster_codex` (2 + 3) — but both are too
  thin to prove a chassis-plus-features mechanism; `book_of_the_damned_volume_2` (4 + 17 = 21) is a
  viable smaller alternative if the operator wants a cheaper pilot than Bonus Bestiary. Recorded, not
  substituted — Bonus Bestiary stays the pilot of record unless the operator says otherwise.
- **Race-Trait**: 44 held / 3,412 remaining across 27 books; defect-fix-alongside per
  `../corpus-work-channels.md §9.3` — `classify()`'s only grounding source is CRB's own hardcoded
  table, so a non-CRB trait grounds today only by name coincidence (SD-28 `§56`). **Pilot:
  `inner_sea_intrigue`** (9 remaining units, smallest non-degenerate book — `book_of_the_damned_volume_1/2`
  at 1 unit each are too thin to prove the fix).
- **Companion**: 0 held / 1,683 remaining across 17 books; no path anywhere in the corpus. **Pilot:
  `inner_sea_combat`** (10 remaining units, smallest non-degenerate book — `horror_adventures` and
  `inner_sea_intrigue` at 2 units each are too thin).

**Tier 3 — blocked, out of scope.** `class_feature`: 15,472 units, 40.2% of the corpus, 109 held /
15,363 remaining. Stays out — see §38.4.

### 38.4 `class_feature` stays out — `§63`'s reason, restated at corpus scale, with a named successor

SD-28 `§63` established that per-class archetype-slot sizing cannot be extrapolated: four
hand-verified classes spanned 5%–70% of named slots wire-able, with no formula connecting sample to
population. `../corpus-work-channels.md §9.1` funds the per-class hand-verification for the
remaining ~24 classes as its own effort, separate from any book-ingest bundle. That funded effort —
not SD-29, not SD-30 — is `class_feature`'s successor owner. It has not yet been assigned an SD
number; whoever picks up `../corpus-work-channels.md §9.1`'s funded measurement inherits these
15,472 units (up from the 90 units SD-29's prior seven-book scope tracked in
`successor-forward-scope-register.md` C1.3, now widened to the full corpus-wide count). Recorded as
an open item in `risks-and-open-questions.md`, not silently assigned.

### 38.5 SD-30 collision — flagged 2026-08-10; RESOLVED the same day (resolution recorded below)

`docs/release/SD-30-class-feature-archetype-bundle/decisions.md §1` pins a sixteen-book list
(Occult Adventures, Mythic Adventures, the eight-book Inner Sea line, Book of the Damned vol. 1/2,
Occult Origins, Haunted Heroes Handbook, and others) dispatched **per-book**, planning-ready, not
re-cut by this decision or by `§36`/`§37` — this decision's write scope is SD-29 only, per the brief
driving it. Every one of those sixteen books carries units of SD-29's now-corpus-wide lanes (spell,
equipment, monster_ability, race_trait, companion, etc.) — the same (kind, book) cells SD-29's lanes
now claim. **This is a live collision, not a hypothetical one:** if both packages dispatch cycles
against the same book's same kind, two writers land on the same table file. Recorded as an explicit
open item for the operator in `risks-and-open-questions.md` — not resolved here, per the brief's
explicit instruction not to re-scope SD-30 from this package.

**RESOLVED 2026-08-10, from SD-30's side, recorded here by cross-reference (reference-and-resolution
only, per that change's write-scope authorization — SD-29's own scope/epics are untouched by this
addition).** Operator directive: SD-30 becomes the `class_feature` bundle — the one kind lane this
package's `§38.4` explicitly leaves out. SD-30's sixteen-book list dissolves outright (not narrowed to
a `class_feature`-only subset of those sixteen books — SD-30's real `class_feature` population spans
23 books, most of them books this sixteen-book list never named, e.g. `advanced_class_guide` at 2,396
units alone exceeds the old list's ten Inner Sea modules combined). Full detail:
`docs/release/SD-30-class-feature-archetype-bundle/decisions.md §33-35` (renamed via `git mv` from
`SD-30-occult-and-companion-content-ingestion`, history preserved).

**Collision closed structurally, not by dispatch discipline.** After SD-30's re-scope, no kind is
claimed by both packages in any book: SD-29 claims every kind except `class_feature`, corpus-wide
(this package's own `§38`); SD-30 claims only `class_feature`, corpus-wide. The two-writer risk this
section flagged required a shared (kind, book) cell, and none remains.

### 38.6 What does not change

`canonical_branch: tranche/9` and `build_version_target: 0.9.<build>` (`§34`, restated `§37.5`);
sequential launch after SD-28; unattended-mode operating protocol (`§22`); Epic 3's PI-screening
gate, now stated corpus-wide rather than seven-book (`epic-breakdown.md` Epic 3); the reach gate as
definition-of-done (`§19`); every decision `§1`-`§37` not explicitly named above. This decision
changes **book-list scope** (corpus-wide, not seven books) and **lane sizing** (re-derived at
corpus scale); it does not change branch, version scheme, launch order, or the kind-lane
partitioning `§36`/`§37` already established.

**Authority:** operator directive 2026-08-10 (verbatim above, distinct from and later than the
directive `§37` executed); `decisions.md §§36-37` (partitioning, not superseded by this decision);
`../corpus-work-channels.md` (channel analysis, §9.4's deferral now superseded twice — once for
partitioning at `§37`, once for book-list scope here); `docs/work-inventory.json` (all figures
re-derived above, commands included).

## Decision 39 — A gate stage that fails twice with the same attribution is an incident, not an environment quirk (2026-08-10 retrospective, pre-launch)

**Finding, re-derived before acting (not transcribed from the retro brief that raised it):**
`python3 scripts/retro.py query --type incident --grep "normalized"` returns the tranche-8 incident
event (`tranche8-incident-retro`, 2026-08-01): `scripts/verify.sh`'s `root-full` stage was RED on
**29 of 33** full runs across the whole SD-27 tranche (2026-07-31T05:35 .. 2026-08-01T15:58), every
time attributed to the same "environmental `/home/ubuntu` fixture" bucket. That normalized red
concealed `tests/sd27_advanced_race_guide_parity.rs` and `tests/sd27_pathfinder_unchained_parity.rs`
— the two gates that prove SD-27's own headline claim — which **never executed once for the entire
tranche**. Both went green minutes after the foreign-home path was made `$HOME`-relative.
Cross-checked against `scripts/retro.py query --type verification --json` over the 60-day window:
114 verification runs recorded, 46 FAIL (not "48" as an earlier draft of this finding stated —
re-derive this number yourself before citing it, see the correction below), 36 of the 46 failures
are `root-full`. The brief that raised this finding cited "48 failing" for the 60-day window; the
re-derived count is **46**. Recorded here as a correction to that brief
(`scripts/retro.py correction --subject "pre-launch retro brief (2026-08-10 four-fixes)" --claimed "114 runs / 48 failing / 36 root-full" --actual "114 runs / 46 failing / 36 root-full" --verified-by "python3 scripts/retro.py query --type verification --json, filtered on result==FAIL"`).

**Rule:** a gate stage that fails **twice** with the **same attribution** (same stage name, same
cited cause, e.g. "environmental fixture") is an **incident**, not an environment quirk, and it
blocks the cycle until the attribution is *proven*. Proof means naming — by command, not assertion —
which tests did not execute (`comm -23` between the derived expected-suite list and the "Running"
lines a stage's own log actually produced; see `scripts/verify.sh`'s `root-full` stage, which now
performs exactly this check on every run per Decision 40 below). An attribution is not proof; a
named list of non-executing tests is.

**Enforcement:** on the SECOND occurrence of the same (stage, attribution) pair within a bundle, the
cycle emits `scripts/retro.py incident --recurrence-key <stage>-normalized-red --impact "<stage>
failed twice attributed to <cause> without naming which suites did not execute" --detected-by "<the
comm -23 / grep command that would prove or disprove the attribution>"` **before** treating the
failure as environmental. The cycle does not fabricate a pass and does not silently retry past it —
this is a `decision-blocked` per `loop-instruction.md`'s "Stop vs. press on," not a routine judgment
call, because "the same excuse twice" is exactly the shape that hid two un-run parity suites for a
full tranche.

**Wired into `loop-instruction.md`:** Cycle mechanics step 4 (Verify) and the Hard stops section both
now carry this rule directly, so a cycle hits it at the point it would actually apply rather than
only in this decision record.

**Authority:** `docs/retro/events/tranche8-incident-retro.jsonl` (`tranche8-incident-retro`
2026-08-01T19:04:57Z); `scripts/retro.py query --type verification --json` (re-derived 2026-08-10);
`scripts/verify-baselines.env`'s own "READ THIS FIRST" retraction block (the primary source's own
record of the same failure mode).

## Decision 40 — Non-execution is checked by name, derived from the filesystem, not by aggregate count (2026-08-10 retrospective, pre-launch)

**Problem:** `scripts/verify.sh`'s `root-full` stage already floors total tests passed and total
test binaries executed (`BASELINE_ROOT_FULL_TESTS`, `BASELINE_ROOT_TEST_BINARIES`), but a floor on a
total cannot catch one specific suite silently not running while a different suite starts running in
the same window — both totals hold steady and the stage reports green. That is the exact shape of
Decision 39's finding: two suites disappeared from execution for an entire tranche while the
aggregate "N passed across M suites" line looked unremarkable.

**Mechanism, landed in `scripts/verify.sh`'s `root-full` stage:** `expected_test_suites()` derives
the set of suites that must run directly from the filesystem — every top-level `tests/*.rs` file is
one cargo integration-test binary by cargo's own auto-discovery convention (`find "$REPO_ROOT/tests"
-maxdepth 1 -name '*.rs'`; subdirectories such as `tests/fixtures` and `tests/sd16-e5-f1` are not
auto-discovered and are correctly excluded by `-maxdepth 1`). `executed_test_suites()` extracts the
suite name cargo itself prints on its `Running tests/<name>.rs (...)` line for every binary it
actually ran. `comm -23` between the two names any suite present in `tests/` that produced no
`Running` line, and the stage fails naming them explicitly — not via a floor on a total. **Derived,
not hand-listed:** the expected set is recomputed from the current filesystem on every run, so it
cannot rot the way a hand-maintained "critical suites" list would (the same rot this program's
roster and allowlist failures already recorded).

**Verified to bite:** `sd24_release_notes_structure` was disabled via a temporary `[[test]] name =
"sd24_release_notes_structure" test = false` block in `Cargo.toml` (the target still exists on disk
under `tests/`, but cargo no longer builds or runs it — the same externally-observable shape as a
suite silently dropped from execution). `scripts/verify.sh --only root-full` FAILED, naming
`sd24_release_notes_structure` as never-executed. The mutation was reverted and the stage re-run
green. See the cycle receipt for the verbatim FAIL/PASS output.

**Authority:** Decision 39 (the finding this mechanism closes); `scripts/verify.sh` `root-full`
stage source, `expected_test_suites`/`executed_test_suites`.

## Decision 41 — Function-based naming is the repo's identifier convention; both SD-NN and GE-NN tag families are banned from file names, directory names, and identifiers (2026-08-11, operator directive)

**Operator directive, verbatim:** "references to SD and ge were to be replaced a few tranches ago
with function based naming. clean that up while you are at it."

**Ruling.** `docs/doctrine-external/identifier-discipline.md`'s headline — "Source-code identifiers
describe WHAT the artifact does, NOT which release or spec domain it came from" — binds **two** tag
families, not one: the SD-NN release-bundle tags and the GE-NN grand-epic tags. It binds the
**artifact**, not only the symbol: a FILE named `src/bin/sd27_gen_book_cache.rs` and a DIRECTORY
named `apps/desktop/src/sd16/update/` are the same violation as a struct named
`Ge08AuthoringWorkbenchRequest`. Landed by card `epic-1b-naming-sweep` (Order 2.5).

**Why it needed its own card rather than Epic 1.** Epic 1 hardened
`scripts/identifier-discipline-audit.sh`, and this work is what that gate was supposed to have been
preventing. Three whole classes escaped the hardened gate, each verified live against this repo:
(a) the GE-NN family was absent from the regex entirely; (b) the infix form is unmatchable by a
leading `\b` because `_` is a word character, so `kind_is_sd17_b3`,
`build_ge08_workbench_snapshot`, and `seeded_sd13_e1_f1_current_truth` all passed clean; (c) the
regex is identifier-shaped and scanned file *content* only, so no path tag was ever detectable.
All three are now covered by cases in `scripts/tests/test_identifier_discipline_audit.sh`.

**The documented exclusion class still stands** (identifier-discipline doctrine, SD-25 1.1): a doc
comment or string literal citing a REAL `tests/...` file by name is test-traceability grounding,
not a violation. The audit now strips such citations before matching rather than relying on cycles
to ignore the noise — `src/rules_core/support_state_matrix.rs` alone carries 319 tag-shaped hits,
nearly all of this class. Renaming a *cited* file obliges the citations to move with it; it does
not license mass-rewriting prose that cites a file nobody renamed, and `tests/` file names
themselves are out of scope for this card precisely because 531 of them are load-bearing citation
targets.

**Convention for successor lanes (binding on Epics 3-11).** Any new codegen binary is named for its
function — `gen_book_cache.rs`, never `sd29_gen_book_cache.rs`. The `src/bin/gen_cache_beastiary.rs`
precedent is the correct one to copy; `src/bin/sd27_gen_book_cache.rs` was not, and no longer
exists under that name. Same rule for modules, structs, consts, test module names, and directories.

**Authority:** operator directive 2026-08-11; `docs/doctrine-external/identifier-discipline.md`;
`scripts/identifier-discipline-audit.sh` + its self-test.

## Decision 42 — SD-29 is REOPENED; the companion lane and the monster / race-trait ingest halves are SD-29 scope, not a successor's (2026-08-11, operator directive)

**Operator directive, verbatim:** "this is part of sd-29's scope. sd-29 isn't done. let's get after
it."

**Ruling.** The bundle's closure of 2026-08-11 (`73f1421f`, `ac217788` — `epic-10-review` and
`epic-11-closure`) is **rescinded**. SD-29 is not closed. The closure was taken while three lanes
still had outstanding work, and it disposed of that work by routing it outward — the companion lane
to "a ready re-dispatch for a successor bundle," the monster extend and race-trait ingest halves to
`decision-blocked` rows carried as shipped "Known issues." The operator has ruled that disposition
wrong. The work is **in scope for SD-29** and does not transfer to SD-30, to
`successor-forward-scope-register.md`, or to any other successor.

**What this decision moves back in scope**, precisely:

1. **`epic-7-companion-lane-pilot`** — never started. Its cycle refused at Cycle-mechanics step 1c
   (`preflight-disk` EXIT=1 at 91% used) and correctly declined to claim the row. That refusal was
   an environmental condition, never a scope ruling, and the condition has cleared. SD-29 owes this
   card.
2. **`epic-7-companion-lane-extend`** — never eligible, because card 11 never completed. All
   corpus-wide `companion` units remain ungrounded. SD-29 owes this card.
3. **`epic-5-monster-lane-extend`** — the once-per-kind chassis is merged and pilot-proven, but the
   per-book **ingest** was never dispatched. The chassis is not the lane.
4. **`epic-6-race-trait-lane-pilot`** — the classifier defect fix landed; the per-book pilot
   **ingest** did not. Its re-pin (the pilot book carries zero true race traits) is SD-29's to make.
5. **`epic-6-race-trait-lane-extend`** — the companion mis-classification fix landed; the
   corpus-wide **ingest** did not.

**A `decision-blocked` row is not a completed lane.** This is the durable lesson, and it is the
reason the premature closure was possible at all: `kanban.md`'s status legend makes
`DECISION-BLOCKED` "a terminal state, not a wait ... the card is closed for this bundle," and
`epic-11-closure` applied that legend to rows whose *ingest* half had merely never been dispatched.
The legend is sound for a card that genuinely needs an operator ruling; it is not a disposal chute
for undelivered work. Where a card splits into a delivered half and an undelivered half, the
undelivered half stays `READY` under its own row — it does not inherit the delivered half's status.

**Structural blocks are unaffected by this ruling and remain real.** The race chassis ceiling
(`crb::race_traits()` models exactly 7 races) is a genuine finding about what the engine can ground,
not a scheduling excuse. Reopening the lane means SD-29 owns confronting that ceiling; it does not
mean the ceiling was imaginary or that a cycle may ground a race trait by inventing a race.

**PR #360 (`tranche/9` → `develop`) stays OPEN and unmerged.** It was opened by the rescinded
closure card. The operator merges it at *real* closure, and real closure now requires the five cards
above to reach a genuine terminal state.

**`epic-10-review` and `epic-11-closure` are reopened** with the lanes. Both ran against a bundle
state that is no longer final: a full-bundle diff review and a closure rollup are only meaningful
against the diff and the state that actually ship.

**Executed by** card `epic-12-reopen` (Order 17 on `kanban.md`), which corrected `kanban.md`,
`progress.md`, and `release-notes.md` to match this ruling and re-derived the affected denominators
from `docs/work-inventory.json`.

**Authority:** operator directive 2026-08-11 (verbatim above); supersedes the closure disposition
recorded in `progress.md` `## Cycle SD29-E11-F1-001` and in `release-notes.md` §Known issues 1 and 3.

---

## Decision 43 — The desktop driver, not the desktop app, is what broke on-screen verification; item 8 stands unweakened (2026-08-11, card `epic-13-desktop-driver-fix`)

**Ruling.** Definition-of-done item 8 (on-screen verification for player-visible families) is
**not** weakened, waived, or redefined for a headless host. This box has a usable display: `Xvfb`
and `xvfb-run` are installed, the driver already provisions one, and the app renders on it. The
reported blocker was a defect in `apps/desktop/.claude/skills/run-desktop/driver.sh`, and it is
fixed.

**The claim that was wrong.** Three run-1 cycles independently reported that
`driver.sh launch` "builds, logs `Running <target>/debug/codex-desktop`, then the binary EXITS
before any window appears," citing `libEGL warning: DRI3 error: Could not get DRI3 device` as the
only diagnostic. That diagnosis is false in both halves.

- The `libEGL`/DRI3 lines are **not** an error. They are emitted on every successful launch on this
  box, including the ones that produced the screenshots in
  `artifacts/desktop-driver-fix/`. They report the absence of hardware-accelerated rendering, which
  under Xvfb is expected; WebKitGTK falls back to software rendering and proceeds.
- The binary does not exit. Re-derived directly, bypassing the driver:
  `DISPLAY=:67 timeout 60 ./target/debug/codex-desktop; echo $?` → **124**, i.e. still running when
  the timeout killed it. Enumerating X windows during that run showed `WM_NAME=codex-desktop`
  immediately and `WM_NAME=Codex` — the window the driver searches for — appearing about **35s**
  after process start.

**Why every cycle nevertheless saw an empty process table.** `cmd_launch` sets
`trap 'cmd_stop || true' EXIT INT TERM`. Any launch failure therefore killed the app and the X
server *on the way out*, before the agent could look at either. Every post-mortem `pgrep -af
'codex-desktop$'` ran after the evidence had been destroyed, and an empty process table is
indistinguishable from a crash. The driver's failure path was manufacturing the very symptom that
was then attributed to the app. This is a diagnostic-destroys-evidence defect, and it cost three
cycles the same wrong conclusion.

**The three underlying defects, each now covered by a case in
`scripts/tests/test_run_desktop_driver.sh`:**

1. **Readiness detection was not scoped to the agent's own display.** `pgrep -f "target/debug/codex"`
   matched *any* agent's app process — the "known gap, still live" that `SKILL.md` documented but
   nothing tested. When a sibling agent's app was already running, the poll succeeded instantly and
   the window-search budget began counting while this agent's binary was still compiling, so the
   search could not possibly succeed. Run 1 dispatched six concurrent agents, which is exactly the
   condition that fires it. The path-shaped match had a second blind spot: a dispatched agent
   exporting `CARGO_TARGET_DIR` builds the binary *outside* `target/debug/`, where the pattern finds
   nothing at all. Now matched on the executable's own name and filtered by the candidate's own
   `DISPLAY` environ.
2. **The window-search budget had no headroom.** 90 iterations × `sleep 0.5` = **45s**, against a
   measured **~35s** cold WebKitGTK start on an *idle, uncontended* box. Run 1 drove this box to
   load average 9-12. Raised to 180s, overridable via `RUN_DESKTOP_WINDOW_TIMEOUT`.
3. **`cmd_stop` killed unrelated processes.** `pkill -9 -f "Xvfb :$DISPLAY_NUM "` matches any
   command line *containing* that text, not just an X server. Observed live twice during this card:
   it killed the shell that was invoking it, which then returned no output and no error — again
   indistinguishable from the app dying. Now matched on the process's executable name plus its
   actual display argument.

Additionally, every failure path now calls `cmd_diagnose` **before** cleanup runs, printing app
liveness, the full window inventory with `WM_NAME`s, and the launch-log tail; and the timeout paths
distinguish "app process exited" from "app is running but no window appeared", because those two
need different fixes and reporting the first when the second happened is what sent three cycles
after a nonexistent app bug.

**Gate coverage.** New `driver-selftest` stage in `scripts/verify.sh`, in **both** the full and
quick sets (no build, no display, seconds to run). Precedent and shape: the existing
`audit-selftest` stage, including its 0-cases-ran guard. The reason it is a gate stage rather than a
script someone remembers to run is that when this driver breaks, the entire class of defect that
only on-screen driving reaches stops being detectable and *nothing says so* — five agents invoked
`launch` during run 1, not one left a state file, and the gate stayed green throughout.

**Detection power was verified, not assumed** (the failure mode
`scripts/identifier-discipline-audit.sh`'s own header records twice). Each case was re-run against a
deliberately-regressed copy of the driver: un-scoping the readiness match fails case 1, restoring
the substring `pkill` fails case 3. That exercise caught **two** worthless assertions in the
self-test's first draft — `kill -0` returns success on a zombie, so a SIGKILLed child read as alive;
and `bash -c "echo …; sleep 300"` is exec-optimized so the decoy's command line never contained the
string it was supposed to be matched on. Both cases passed against a driver that was demonstrably
broken until this check exposed them.

**A real environmental constraint, recorded rather than worked around.** This box has **22 GiB RAM
and zero swap** (`free -h`). Launching the desktop app while a cargo build is running gets the vite
dev server OOM-killed — reproduced during this card while `verify.sh`'s `root-full` stage was
building at load average 21, where the new diagnostics correctly reported
`The "beforeDevCommand" terminated with a non-zero status code` and `Killed` rather than blaming the
binary. **On-screen verification and a full gate must not be run concurrently on this host.** This
is very likely a second contributor to run 1's driver failures, which ran six agents against 4 cores.

## Decision 43 — the race-trait lane's pilot book is re-pinned to `monster_codex`; and the "engine models exactly 7 races" ceiling is a *work-inventory probe* defect, not an engine limitation (2026-08-11)

**Card:** `epic-6-race-trait-lane-pilot` (Order 9). **Actor:** `sd29-racetrait-repin`.

### 43.1 The re-pin, and why every other candidate was rejected

`loop-instruction.md`'s "Epic ordering" pinned the race-trait pilot to `inner_sea_intrigue` (9
units). The prior cycle established that book carries **zero** genuine race traits — all 9 of its
`race_trait`-kinded units are Clockwork Familiar / Clockwork Spy construct-companion abilities that
`file_kind()` typed by filename — and the classifier fix has since reclassified them `companion`.

The candidate set was **re-derived from scratch this cycle**, not transcribed. Command:

```bash
python3 -c "
import json,collections
d=json.load(open('docs/work-inventory.json'))
nc=collections.Counter()
for u in d['units']:
    if u['kind']!='race_trait': continue
    if 'companion' not in (u.get('source_file') or '').lower(): nc[u['book']]+=1
for k,v in sorted(nc.items(), key=lambda x:(x[1],x[0])): print(v,k)
"
```

It reproduces the prior cycle's six candidates exactly — `ultimate_intrigue` 3, `ultimate_magic` 3,
`inner_sea_bestiary` 4, `ultimate_combat` 4, `monster_codex` 14, `bestiary` 21 — and adds two the
prior cycle did not list, `book_of_the_damned_volume_1` and `_2` at **1** each (its note said "2
units each"; the current inventory says 1). Both are too thin to pilot either way.

**The recommendation is adopted, but the prior cycle's reason was the weaker one.** It recommended
`monster_codex` because DoD item 6 expects that book to retire the standing `beastiary1/race_traits`
finding. True, and it happened (§43.3). The decisive reason is different and was not on the table:
**`monster_codex` is the only candidate that carries any player-race racial traits at all.** Reading
the actual unit keys rather than the counts:

| candidate | what its `race_trait`-kinded units really are |
|---|---|
| `ultimate_intrigue` (3) | eidolon subtype / base form / unchained evolution rows |
| `ultimate_magic` (3) | `Racial SLA ~ …` spell-like-ability rows |
| `ultimate_combat` (4) | `Gunslinger` / `Ninja` / `Samurai` favoured-class rows + one Racial SLA |
| `bestiary` (21) | monster racial abilities — Drow Noble, Rust Monster, Treant, Unicorn, `Template ~ +2 <Stat>` |
| `inner_sea_bestiary` (4) | out of scope for the pilot regardless — see §43.2 |
| **`monster_codex` (14)** | **Duergar 2 + Goblin 4 genuine alternate racial traits**, plus Ratfolk 6, one monster ability and one Racial SLA |

So five of the six candidates carry the *same* defect that disqualified `inner_sea_intrigue` — a
`_abilities_race` filename over rows that are not racial traits. A pilot on any of them would have
proven the mechanism against content the mechanism does not serve. **`monster_codex` is not the
best pilot; it is the only viable one.**

### 43.2 A correction to this cycle's own dispatch brief

The dispatch stated "`inner_sea_bestiary` is explicitly OUT of this bundle's scope per
`loop-instruction.md`". That is a transcription of a passage `loop-instruction.md` **strikes through
and corrects**: its "Corpus shape notes" bullet marked `~~Out-of-scope adjacents~~` is annotated
**CORRECTED 2026-08-10**, and `inner_sea_bestiary` is one of the 37 in-scope books. The book was not
picked, so nothing turned on it — recorded because a struck-through line was read as live text, and
that is a repeatable failure mode.

### 43.3 What the pilot landed

* **5 records** at `data/corpus/monster_codex/race_trait/{duergar,goblin}/` — Duergar 2, Goblin 3 —
  written by `src/bin/ingest_race_traits.rs`, PI-screened (0 hits), `LICENSE.json` at
  `records_processed: 5`.
* **`monster_codex` joins `race_catalog::RACE_CORPUS_BOOKS`** and gets a real reach claim,
  `("monster_codex", "race_traits") => race_traits_reach("MC", "monster_codex")`. **No new surface
  was built and none was needed**: `race_trait_picker` is book-agnostic and serves whatever the race
  corpus loads. The per-book cost of this lane is therefore an ingest, not a chassis.
* **The `beastiary1/race_traits` finding is RETIRED** — removed from both `OPEN_FINDINGS` and
  `UNREACHED_RECORD_FINDINGS`, satisfying DoD item 6's standing expectation. `Duergar ~ Ironskinned`
  (`mc_abilities_race.lst:16`) carries the only `FACT:Duergar_ReplaceSLAEnlargePerson|True` token in
  the upstream corpus, which is the positive `PREFACT` gate on B1's
  `Duergar ~ Spell-Like Ability ~ Invisibility`. **Fixed by data, not by code**: nothing in
  `race_resolver` changed. `tests/duergar_invisibility_sla_reaches_a_player_via_monster_codex.rs`
  replaces `tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs` and holds it closed in both
  directions.
* **The predecessor test would not have caught its own closure.** Its `corpus()` helper hardcoded
  three book roots, so ingesting a *fourth* book left it green while the fact it asserted became
  false. The replacement re-derives the loaded book list from `race_catalog.rs` itself and pins the
  two equal.
* **`src/bin/ingest_race_traits_arg.rs` → `src/bin/ingest_race_traits.rs`**, book-table-driven
  (`BOOK_SOURCES`). Adding a book is now a 3-field entry. The repo already carried one full copy of
  this binary (`ingest_apg_race_traits.rs`); it does not need a second. ARG's 156 records regenerate
  **byte-identical** apart from `ingested_at`, verified by regenerating and diffing.
* One shared-code change, derived not guessed: `KEY:` is **optional** in PCGen and a row without one
  is keyed by its display name (`mc_abilities_race.lst:30`/`:31`). The binary used to panic. The
  default matches the one `v06_work_inventory` already applies to those same two rows.

### 43.4 Six Ratfolk rows are deliberately NOT written

`mc_abilities_race.lst` carries 6 Ratfolk alternates. Ratfolk has no ingested race chassis, no
default traits and no picker entry. Writing them would create the repo's only Ratfolk content for a
race that does not exist in it — inventing content to make a count look better. They are counted and
reported by the ingest run, never emitted, exactly as ARG's out-of-scope races are.

### 43.5 The "race chassis ceiling" is narrower than `kanban.md` records it

Card 10 records the extend lane as blocked because "the engine models exactly **7** races (CRB's
hardcoded `race_traits()`)". **Re-derived, that is true of one instrument and false of the product.**

* The **player surface** — `race_resolver::load_race_corpus` → `race_trait_picker` →
  `list_alternate_racial_traits` — models **18** races, read off disk at runtime. Derivation:
  `ls -d data/corpus/{core_rulebook,beastiary,advanced_race_guide}/race_trait/*/ | xargs -n1 basename | sort -u | wc -l`
  → 18 (CRB 7 + B1 11; ARG's 18 dirs are the same union). Duergar and Goblin are both in it, which
  is why this pilot's records reach a player at all.
* **`v06_work_inventory`'s grounding probe** is the thing pinned to 7: `race_names` is built from
  `RaceId::ALL` and `race_trait_ids` solely from `crb::race_traits()`
  (`src/bin/v06_work_inventory.rs`), so every race trait outside CRB's 7 races reports
  `race_trait_race_not_modelled` **no matter how reachable it is**. All 5 of this pilot's records
  report that verdict while `reach_gate` executes a passing claim against the same records.

This is the doneness-instrument hierarchy in a live instance: `reach_gate` executes IPC,
`v06_work_inventory` answers a narrower question. **Card 10's real first task is repairing that
probe** — grounding a race trait against the corpus-driven resolver the app actually reads, not
against CRB's compiled table — and its blast radius is large (ARG's 156, B1's 108, APG's 1 and these
5 all currently read `not-ingested` while reaching a player). It is recorded here with its evidence
rather than attempted inside a pilot cycle, because it moves several hundred units' status across
the whole dashboard and belongs in the extend lane's own reviewable diff.

### 43.6 `RuleSetId::MonsterCodex`

Added, with `COMPILED_RULE_SETS` / `corpus_dir_for` / `rule_set_id` / `content_state_dump` arms. It
is the **first variant with no `rules_tables/<book>/` module**, because a race trait is never a
compiled table. Withholding it would have left all **207** of the book's units at `not-started` —
"nothing about this unit has been attempted" — for a book whose records a player can now select,
which is the exact defect `COMPILED_RULE_SETS`' own doc comment records ARG and PU suffering for
eleven days. After the change **0** monster_codex units remain `not-started` (DoD item 4).

### 43.7 Per-unit cost, for the extend cycles

**Do not extrapolate a per-record rate.** The pilot's cost was almost entirely once-per-*lane*:
the binary generalisation, the `RuleSetId` variant, the `RACE_CORPUS_BOOKS`/`book_code` wiring, the
finding retirement and its replacement test. The **per-book** cost that remains is one
`BOOK_SOURCES` entry, one `CORPUS_BOOK_IDS` entry, one `reach_of` arm, one `LICENSE.json`, one
`RuleSetId` variant — and, for any book whose traits belong to races outside the 18, nothing at all
to write. The dominant real cost for the extend lane is §43.5's probe repair, which is paid once.

---

## Decision 44 — Race-Trait Lane, extend: round 1 (2026-08-11, `sd29-racetrait-r1`, card `epic-6-race-trait-lane-extend`)

Card 10 is a **loop-until-dry** lane. Round 1 delivered §43.5's probe repair, closed one deferral
and one live stub, and stopped cleanly with a re-derived remainder. It did not finish the lane and
does not claim to.

### 44.1 The probe repair (§43.5), done — and it was two defects, not one

`v06_work_inventory` now grounds a race trait by asking **the race corpus the desktop app really
loads** whether it can apply the record to a player, and falls back to CRB's compiled
`race_traits()` table only when that says nothing.

* Book list is **read from the product**, not duplicated: `app_race_corpus_books()` parses
  `apps/desktop/src-tauri/src/race_catalog.rs`'s own `RACE_CORPUS_BOOKS`. An unreadable or
  unparseable declaration yields an EMPTY list, so a broken read under-claims rather than
  over-claims.
* The join key is `(<lst basename>, <line>)`, never the name. A race trait's display name is not
  unique corpus-wide — that is the whole reason `modelled_race_of_race_trait` exists — whereas the
  source coordinate is an identity the ingest writes verbatim. The join is exact: **337 corpus
  records ↔ 337 inventory units, 0 orphans on either side.**
* Grounding is on **applicability**, not presence on disk. `TraitRole::Unclassified` never applies,
  so `Oversized Goblin` is reported `not-ingested` with its own new evidence token
  `race_trait_record_loaded_but_never_applies` — the honest middle between "not ingested" and
  "grounded", and consistent with the `OPEN_FINDINGS` entry the pilot recorded for it.

**The second defect, found because the first one's number was still wrong.** The first regeneration
grounded **228**, not the 336 the on-disk record count predicted: `core_essentials` showed 67 of its
175 records grounded. Cause: `engine_book_for` keys on `corpus_dir_for`, which spells Bestiary 1's
directory `bestiary` — the PCGen **source** tree's name — while this repo's corpus directory is
`data/corpus/beastiary`. Every one of Bestiary 1's 108 loaded, applied, reachable race traits
resolved to no engine book and stayed `not-ingested`, silently. Fixed with a one-entry
`CORPUS_DIR_ALIASES` table, and pinned by
`every_corpus_book_with_race_traits_resolves_to_an_engine_book`, which also asserts `beastiary` is
the **only** book needing an alias. `reach_gate::CORPUS_BOOK_IDS` already records the same
divergence for the same reason; the two now agree.

**Result, re-derived by the command in the receipt:** `race_trait` grounded **21 → 336**.

### 44.2 A live stub the pilot shipped, and the reason nothing caught it

`race_trait_picker` offers every `TraitRole::Alternate` record the loaded corpus holds.
`pilot_compute::explain_selected_alternate_racial_traits` raises a **claim-blocking**
`race.alternate_trait.unknown` for any selection `race_resolver::ALTERNATE_TRAIT_REPLACE_FLAGS` does
not know. The pilot added Monster Codex's 4 alternates to the first set and not the second, so all
four were affordances a player could tick and `create_character` would refuse.

Nothing caught it because `race_resolver.rs`'s own test module loaded a **hardcoded**
`[crb(), b1(), arg()]`. Every assertion in that module reading "for every alternate in the corpus"
was silently scoped to three books while the app loaded five — including the pin test whose stated
job is that this table cannot drift from the corpus. This is the **identical** stale-hardcoded-roots
defect the pilot itself found and fixed one file over
(`tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs`); it survived here because nobody
pointed the same question at this module.

Fixed by deriving the module's roots from `RACE_CORPUS_BOOKS` and adding
`every_alternate_the_app_offers_is_one_the_engine_can_place`, which states the invariant directly
rather than as a count. Widening the roots turned 6 further assertions red at once — all of them
count pins and book-scoped claims that had been quietly narrow — and each was widened in **both**
directions rather than relaxed (the one unclassified row pinned by exact key; the orphan-flag list
pinned by exact flag with the same grant-proof applied to the new one).

### 44.3 APG's `Half-Orc ~ Plagueborn`: a deferral closed, not an oversight found

This cycle first read the un-landed APG race-trait ingest as a shipped binary whose output nobody
ran. **That reading was wrong and the docs were right** (`decisions.md §39`): the record was held
back deliberately, because `ALTERNATE_TRAIT_REPLACE_FLAGS` did not know its key and shipping the
record alone would have produced exactly the stub described in §44.2. Both halves landed together
here — the corpus record, the table row, the reach claim, and the picker/catalog/creation pins — so
the affordance is live. `character_hub`'s creation-acceptance sweep now accepts **94** alternates
for the 7 CRB races, up from 93, which is the assertion that would have caught a half-landing.

### 44.4 The lane's real ceiling, re-derived (this is the successor round's starting point)

Of the corpus's **3,447** `race_trait` units, only **553** carry a `TYPE:<Race> Racial Trait`
component naming one of the **18** races the product models. The other **2,894** belong to races
with no chassis — `bestiary_3` 799, `core_essentials` 661, `bestiary_2` 162, `ultimate_psionics` 159
and so on. **No amount of race-trait ingest grounds those**; `RaceCorpus::resolve` returns `None`
without a chassis, so they need a race-chassis lane, not this one. That is a scope finding, not a
blocker: it is recorded so a successor round does not spend itself discovering it.

Within the 553: **336 grounded**, and the honest remainder is

| book | units | note |
|---|---|---|
| `inner_sea_races` (`isr_abilities_race.lst`) | 72 | needs a `RuleSetId` variant; currently `not-started` |
| `core_essentials` (19 `<race>_abilities_race*.lst` files) | 48 | chassis already loaded; pure ingest |
| `horror_adventures` (`ha_abilities_race*.lst`) | 44 | needs a `RuleSetId` variant; one file is `PRECAMPAIGN`-gated on Occult Adventures |
| `bestiary` (`b1_abilities_race.lst`) | 3 | chassis already loaded; pure ingest |
| **total genuinely ingestable next** | **167** | |

Two residuals in the 553 are deliberately NOT gap: APG's 49 (same `KEY:` as an already-ingested ARG
record — republished, not new, `§39`) and Monster Codex's `Oversized Goblin` (mechanism-blocked, its
`OPEN_FINDINGS` entry names the remedy).

### 44.5 Two more instances of the stale-roots defect — deferred, then fixed in the same round

> **SUPERSEDED IN PART, same day, by the gate.** This section was written to defer both files to
> round 2, with the reasoning below. `root-full` then **failed inside one of them**
> (`sd27_alternate_racial_trait_reachability.rs`, two assertions reading the pure table), which
> removed the choice the deferral rested on: the options were no longer "leave them alone or churn",
> they were "bump two numbers and leave the narrow scoping in a file I am already editing" or "fix
> it properly". **Both files were widened to the app's own `RACE_CORPUS_BOOKS` in round 1**, four
> further assertions moved with their reasons, one green assertion correctly went red and was given
> the same grant-proof rather than an exemption, and three test names carrying `153` were renamed to
> carry no number. Evidence in `progress.md` §6b. The original reasoning is kept below unedited,
> because "why this was deferred" and "why the deferral did not survive contact with the gate" are
> both worth having.
>
> **The lesson is the second-order one.** The deferral's logic was sound and its conclusion was
> wrong, and what made it wrong was information the gate had and the reasoning did not. A deferral
> taken before the evidence lands is a prediction, not a decision.

Grepping for the §44.2 pattern after fixing it found the same hardcoded three-book corpus loader in
two integration tests:

* `tests/sd27_alternate_racial_trait_reachability.rs:75-79`
* `tests/sd27_aasimar_globalvar_gate_closes_the_dead_affordance.rs:53-57`

Both build `[core_rulebook, beastiary, advanced_race_guide]` by hand and both then assert `153`. They
are **green today and will stay green** through any number of new books, because they cannot see
them — which is exactly the property that let §44.2's stub ship.

They were left alone in round 1, deliberately, for two reasons. First, the product-level invariant
they were implicitly trusted for is now guarded directly and corpus-widely by
`race_resolver::every_alternate_the_app_offers_is_one_the_engine_can_place`, so the defect **class**
is closed even with these two still narrow. Second, the full gate was already running against the
committed tree when they were found, and editing them would have invalidated its result — a real
gate result on the tree that shipped is worth more than two test-scope widenings landed unverified.

~~**This is round 2's first item**, ahead of any ingest: widen both to `RACE_CORPUS_BOOKS` and move
their pins with their reasons, exactly as §44.2 did for the resolver's own module.~~ **Done in round
1** — see the callout at the top of this section. **Round 2's first item is therefore the ingest
itself**: `core_essentials`' 48 and `bestiary`'s 3, neither of which needs a new mechanism.

## Decision 45 — Race-Trait Lane, extend: round 2 (2026-08-11, `sd29-racetrait-r2`, card `epic-6-race-trait-lane-extend`)

Round 2 ingested **Inner Sea Races** end-to-end — 72 records, 71 of them grounded and reaching a
player — and, before doing so, **corrected the successor queue §44.4 handed it.** The lane is still
not dry and this round does not claim it is.

### 45.1 §44.4's queue was backwards, and the correction is the round's most reusable output

`§44.4` and `§44.5` closed round 1 by naming round 2's first item: *"`core_essentials`' 48 and
`bestiary`'s 3, neither of which needs a new mechanism"*, with `inner_sea_races` and
`horror_adventures` ranked behind them as the ones that *do* (a `RuleSetId` variant).

**Both halves of that are wrong, and the shape of the error is worth more than the fix.** Round 1
classified those books by what the *inventory* said they lacked (`no_compiled_rule_set_for_book` reads
like a bigger obstacle than `shared_library_record_held_by_no_ingested_host`) rather than by what the
*corpus rows* are. Re-derived this round, one row at a time, by the gate each row actually carries:

| book | in-scope rows | `Racial Default` | sets a replace flag (→ `Alternate`) | positive-gated (→ `FlagGranted`) | no readable gate (→ `Unclassified`) |
|---|---|---|---|---|---|
| `inner_sea_races` | 72 | 0 | **68** | 2 | 2 |
| `horror_adventures` | 43 + 1 | 0 | **42** | 0 | 2 |
| `core_essentials` (subrace files) | 48 | 0 | **0** | 48 | 0 |
| `bestiary` (`b1_abilities_race.lst`) | 3 | 0 | **0** | 0 | 3 |

Command, run against the PCGen source tree rather than any doc — and **checked in**, so a successor
round re-derives this rather than trusting the table:

```bash
python3 scripts/classify_race_trait_rows.py \
  isr_abilities_race.lst ha_abilities_race.lst ha_abilities_race_oa.lst \
  aasimar_abilities_race_subrace.lst tiefling_abilities_race_subrace.lst b1_abilities_race.lst
```

It mirrors `race_resolver::classify`'s predicates and precedence (`<Race> Racial Default` →
`Default`; `FACT:<Race>_Replace<Trait>|True` → `Alternate`; a *positive* `PREFACT`/`PREABILITY` →
`FlagGranted`; otherwise `Unclassified`) and `ingest_race_traits::parse_row`'s `.MOD` and
in-scope-race filters. Its output is quoted verbatim in `progress.md`. **This began as a scratchpad
one-off and was checked in mid-round after a sibling agent's write clobbered the scratchpad file this
section had just cited** — an ephemeral path is not a citation, and the derivation that redirected a
whole round is worth reproducing.

**Run it on a candidate book before committing a round to it.** That single step is what `§44.4`
skipped, and it costs seconds.

So:

* **`inner_sea_races` and `horror_adventures` need no new mechanism at all.** A `RuleSetId` variant is
  five one-line arms the compiler *forces* you to write; their rows are ordinary replace-flag
  alternates that the picker serving ARG, APG and Monster Codex already handles unchanged.
* **`core_essentials`' 48 and `bestiary`'s 3 are the mechanism-blocked ones.** The 48 are Aasimar and
  Tiefling *subrace* traits, every one gated on
  `PREABILITY:1,CATEGORY=Special Ability,<Race> ~ <Subrace>` — a shape `classify` does not read — and
  their 16 selector rows are not even `race_trait`-typed, so the ingest binary's parser skips them.
  The 3 are Drow **Noble** traits, a race variant with no chassis, carrying only a negative
  `!PREFACT` and therefore `Unclassified` by construction. Ingesting either set as-is would have
  produced records that load and never apply: **precisely the stub §44.2 was written about.**

**The general lesson, which is not about race traits.** A round-1 deferral note ranked four books by
the inventory's *evidence token* — a statement about what the engine has compiled — when the question
was what the *corpus rows* are. Those are different questions, and the cheaper-sounding token named
the harder work. `§44.5` already recorded that "a deferral taken before the evidence lands is a
prediction, not a decision"; this is the same finding one level up, and it is why this round re-derived
the queue before working it rather than after.

### 45.2 What landed

* `RuleSetId::Isr` + `COMPILED_RULE_SETS` + `corpus_dir_for`/`rule_set_id` arms. The exhaustive match
  did its designed job: adding the variant broke `v06_content_state_dump` until its arm was written.
* One `BOOK_SOURCES` row in `ingest_race_traits.rs` — the whole per-book cost, as that binary's module
  doc promises. 72 records at `data/corpus/inner_sea_races/race_trait/`, 0 PCGen-syntax leaks, 0
  unresolved `DESC:` args, 51 out-of-scope rows across 31 unmodelled races counted and skipped.
* `data/corpus/inner_sea_races/LICENSE.json`. **12 of the 72 descriptions were PI-redacted** by
  `pi_screening` — far more than any rulebook this repo has ingested, and exactly what a
  *campaign-setting* book should produce: Golarion nation and ethnicity names occur inside otherwise
  mechanical prose. The redaction is schema-preserving, so the mechanical payload is untouched.
* 68 rows in `ALTERNATE_TRAIT_REPLACE_FLAGS`, generated from the written records and re-derived from
  them by the existing pin test. They add **13** distinct flags to the corpus's 77, not 68 — a second
  book of alternates for the same 18 races mostly replaces standard traits ARG already replaced.
* A reach claim `("inner_sea_races", "race_traits")`, an `OPEN_FINDINGS` + `UNREACHED_RECORD_FINDINGS`
  pair for the one record that cannot be surfaced, and a claim test that pins the shortfall by exact
  key in both directions.
* `race_trait` grounded **336 → 407**; `not-started` **1,599 → 1,457**.

### 45.3 The RED, and the stale-scope defect this round closed in its own binary

Adding the book to `RACE_CORPUS_BOOKS` before adding the table rows reproduced §44.2's exact failure on
purpose: `every_alternate_the_app_offers_is_one_the_engine_can_place` went RED naming 68 keys the
picker offers and `pilot_compute` would refuse. Five further count pins went red with it. Every one was
moved *with its reason*, none relaxed; the two `Unclassified` rows and the third alternate naming the
truncated multi-flag gate are each pinned by exact key.

`ingest_race_traits.rs`'s own `no_committed_arg_trait_description_leaks_pcgen_syntax` was found to be a
**fourth** instance of the stale-hardcoded-roots defect (`§44.2`, `§44.5`): a test whose stated job is
"no committed record leaks PCGen syntax" that loaded one hardcoded book root while `BOOK_SOURCES` had
grown to three, and that would have stayed green through this ingest without ever reading it. It now
derives its roots from `BOOK_SOURCES` and counts each book by name.

### 45.4 The one record that does not reach, stated as a finding rather than rounded away

`Human ~ Tribalistic Languages` (`isr_abilities_race.lst:216`) is ingested, visible, and never applies.
Nothing upstream grants it: its row carries no gate of any kind, and
`grep -o 'ABILITY:[^\t]*Tribalistic Languages' isr_abilities_race.lst` returns nothing where the same
grep for `Junk Tinker ~ Skilled` one row-family over returns its granter. The alternate that owns it,
`Human ~ Tribalistic` (`:210`), IS selectable and correctly fires `Human_ReplaceLanguages`, suppressing
the standard `Human ~ Languages` row — and nothing replaces it. **An upstream data gap, not a wiring
gap**, evidenced by the fact that the engine's half of the transaction demonstrably works. Its
`OPEN_FINDINGS` entry names two candidate remedies, both new mechanisms.

### 45.5 The remainder, re-derived (this is round 3's starting point)

Round 2's own derivation reproduced round 1's `3,447 / 336 / 3,111` exactly before moving it, so the
two rounds' figures are commensurable. See `progress.md` for the command and the full table. The
genuinely ingestable remainder after this round is **95**, and its order is now the corrected one:

| book | units | what it needs |
|---|---|---|
| `horror_adventures` | 44 | **no new mechanism** — a `RuleSetId` variant + a `BOOK_SOURCES` row, exactly this round's shape. 42 of its 44 are replace-flag alternates. One file is `PRECAMPAIGN`-gated on Occult Adventures |
| `core_essentials` (Aasimar/Tiefling subraces) | 48 | a `PREABILITY`-grant mechanism **and** ingesting the 16 non-`race_trait`-typed subrace selector rows |
| `bestiary` (Drow Noble) | 3 | a race-variant chassis; `Unclassified` by construction without one |

Plus the residuals that are deliberately not gap: APG's 49 ARG-key collisions (`§39`), Monster Codex's
`Oversized Goblin`, and now ISR's `Human ~ Tribalistic Languages`.

## Decision 46 — Monster / Monster-Ability Lane, extend: round 2 (2026-08-12, `sd29-monster-r3`, card `epic-5-monster-lane-extend`)

Round 2 ingested **both Book of the Damned volumes** end-to-end — 62 units, all 62 grounded and
reaching the catalog — and, before doing so, **applied `§45.1`'s lesson to this kind and found a
ceiling that changes what "done" means for this card.** The lane is not dry and this round does not
claim it is.

### 46.1 The lane's REAL remainder is 2,906, not 4,233 — and the difference is structural

`§45.1` established that a lane must classify **corpus rows**, not read the work inventory's
*evidence token*, before committing a round to a book. Applied to `monster_ability`, the question is
not "has the engine compiled this book" but **"is there a monster row that owns this ability"** — a
`monster_ability` record reaches a player only underneath its owning monster (`monster_catalog.rs`
renders it inside that creature's row, per `corpus-work-channels.md §9.2`). An ability no monster row
claims is a record that loads and is never shown: the exact stub class `§44.2` was written about.

Round 1 derived this per book in a `/tmp/.../shape_all.py` that no longer exists. `§45.1`'s own
finding is that **an ephemeral path is not a citation**, so this round checked the derivation in:

```bash
python3 scripts/classify_monster_ability_rows.py
```

It mirrors `scripts/transcribe_monster_tables.py`'s own `parse_special_ability_refs` predicate and
its namespaced-`KEY:` prefix pass, so it predicts what a transcription of that book would really
produce — a looser rule would over-report reachability, which is the direction that ships stubs. Its
unit set is `docs/work-inventory.json`, never a line count over the `.lst`.

Run at this round's end, over the merged tree:

```
remaining monster+monster_ability units : 4233
orphan monster_ability rows             : 1327
  of which in ZERO-monster books        : 703 across 10 books (no monster in the book to own them)
reachable remainder (units - orphans)   : 2906
```

**1,327 of the 4,233 remaining units cannot be grounded by any per-monster cycle**, and 703 of those
sit in ten books that carry no monster row at all (`core_essentials` 380, `advanced_class_guide` 106,
`pathfinder_unchained` 72, `ultimate_wilderness` 52, `bestiary_5` 39, `mythic_adventures` 21,
`ultimate_magic` 13, `bestiary_6` 13, `ultimate_intrigue` 6, `advanced_race_guide` 1). Those ten are
`loop-instruction.md`'s named hard stop reproduced at scale: *a per-monster cycle against a
zero-monster book is a reportable hard stop, not something to force.* The other 624 orphans sit in
books that DO carry monsters — `bestiary_4` 152, `bestiary` 146, `inner_sea_gods` 84,
`ultimate_psionics` 66, `horror_adventures` 65, `bestiary_2` 64 — where the ability's owner is
elsewhere or is expressed in a link shape the chassis does not model.

**This is a scope finding, not a backlog item.** They need a surface decision — a screen that shows
an ability with no monster, or a cross-book owner resolution — which is an operator question, not an
ingest. `deferral` emitted naming both.

**One correction to round 1's table, recorded rather than folded in.** Round 1's `bestiary` row read
`mon 330`; this round's reads `284`. Both are right under their own predicate and neither is wrong:
round 1 counted every monster unit in the book, this round counts the **remaining** ones and Bestiary
1's 46 are already grounded from SD-22. The *link* is still resolved against every monster row
regardless of status, because a grounded monster owns its abilities as well as an ungrounded one —
restricting the link to remaining monsters reported 54 of Bestiary 1's abilities as orphans that are
not. The script says which predicate each column uses.

### 46.2 The two books this round took, and why they were the correct pair

`book_of_the_damned_volume_1` (41 units) and `_volume_2` (21) are **the only remaining books with
zero orphans** — 36 of 36 and 17 of 17 ability rows are named outright by a monster row of the same
book. They are not the densest books; they are the cheapest to finish *completely*, and a whole-book
reach claim is only meaningful for a book that has no orphans in the first place.

Both were `future_state`, so each paid the scope flip round 1 flagged as an unmeasured cost: adding a
`RuleSetId` moves every other kind in the book from `not-started` to `not-ingested`. Measured here
for the first time — v1's collateral is 49 units across 5 other kinds, v2's is 233 across 6. Neither
moves this lane's denominator (both statuses count as remaining); both move other lanes' `not-ingested`
figures, which is why it is recorded.

### 46.3 Two transcriber defects a third and fourth book found, both fixed at the source

Neither was findable by a test, and both are the reason `§45.1`'s "run it on a candidate book before
committing" generalizes: the *transcriber* also needs re-proving per book.

**A row may carry TWO `DESC:` tokens.** 15 of Volume 2's 17 ability rows do — one gated
`!PRERULE:1,DisplayFullAbility` (a one-line summary) and one gated `PRERULE:1,DisplayFullAbility`
(the complete rules text). `parse_desc` took the first match, which on those rows is the **summary**.
`Seraptis ~ Gaze of Despair` (`botd2_abilities_race.lst:17`) would have reached the catalog reading
*"fills the minds of those within %1 feet with overwhelming and soul-crushing despair"* and never
mentioned the Will save, the Charisma drain or the suicidal state the ability causes — a caption
where the rule belongs. The full-text token is now selected when a row states both; a row carrying
several `DESC:` tokens under some *other* gate stops the transcription rather than being resolved by
position. This is a choice between two verbatim corpus texts on a criterion the corpus itself states,
never a composition of one.

**A negated prerequisite was being recorded as a formula variable.** `parse_desc` filtered
`PRERULE:…` out of the trailing variable list and not `!PRERULE:…`, so
`!PRERULE:1,DisplayFullAbility` landed in `description_variables` — a field whose contract is "what
the `%N` in `description` refer to". Bonus Bestiary and Monster Codex carry the shape zero times,
which is why it survived two books. Corpus-wide it occurs on **650** `DESC:` tokens across the
`*_abilities_race*.lst` files (`grep -rhoE 'DESC:[^\t]*\|![A-Z]+[A-Z:]*' --include='*_abilities_race*.lst' .`
from the PCGen `data/` root), so it would have recurred in every remaining book in the lane.

Regenerating Monster Codex under both fixes reproduces its table byte-for-byte, which is the check
that neither fix moved an already-shipped book. Bonus Bestiary's records reproduce identically too;
its committed file is deliberately **not** regenerated, because the pilot hand-authored a module
header the generator does not produce and regenerating would delete prose for no data gain.

### 46.4 A campaign-setting book predicted a PI hit rate and did not have one

Inner Sea Races had 12 of 72 descriptions PI-redacted (`§45.2`) and the reasoning was that a
campaign-setting book's prose carries Golarion proper nouns inside mechanical sentences. Both Book of
the Damned volumes are `campaign_setting/` books and both read `records_redacted: 0`.

The derived reason is worth keeping: a campaign-setting book's *monster* rows are not its setting
prose. These records name devils and demons by their Open Game Content type names; the geography that
carries Product Identity lives in the book's chapters, not in a stat block. **"Campaign setting"
predicts a PI hit rate for `race_trait` and does not predict one for `monster`.**

### 46.5 The branch tip was RED before this round touched it, and one of the three reds was a PI leak

`origin/tranche/9`'s tip `e1f0bdd9` failed three `race_trait_picker` tests. **Proven, not asserted:**
a clean worktree at `e1f0bdd9` with none of this round's changes present reproduces all three
identically (`git worktree add … e1f0bdd9 --detach`, then
`cargo test --locked --bin codex-desktop race_trait_picker`). The merge commit says so itself —
*"Gate not yet re-run across the merge; the resumed lane cycle verifies."*

Two were count pins Inner Sea Races legitimately moved and `c8e2d6ad` missed, because they sit
**after** the aggregate pins it did move inside the same test functions and therefore never executed
until those passed. Fixing an assertion reveals the next one, and a suite run once after a fix is not
a suite run.

The third was not a pin. **`pi_screening` redacts `data.description` and deliberately leaves
`raw_tokens` verbatim — they are the provenance record, not player-facing text — and
`race_resolver::RaceTraitRecord::render_description` reads the tokens.** All 12 redacted Inner Sea
Races descriptions therefore rendered **un-redacted** on the alternate racial trait picker. The
redaction held on disk and was defeated on the screen, which is the only place it matters.

This round fixed it and pushed; **the race-trait lane's round 3 found and fixed the same defect
concurrently and independently**, landing `bd98b9fe` while this round's gate was in flight. The merge
resolved `race_resolver.rs` and `race_trait_picker.rs` in favour of that lane — it owns those files
and its fix is the same fix — so this package carries one implementation, not two. That two lanes
found it within hours of each other, and that neither found it until a book with a PI hit rate landed,
is the finding: **a redaction is only proven by a test that reads the rendered string**, and none
existed until now.

### 46.6 The gate is the only instrument that sees a merged tree, and it ran three times to say so

`VERIFY_EXIT=0`, all 14 stages, on the final tree — but only on the **third** run, and the two before
it are reported rather than discarded.

* **Run 1 found six failures on the post-merge tree.** Two were this cycle's (a PI term in a doc
  comment it had just written; an SD-30 test pinning all twelve campaign-setting books as
  `future_state`, which asserts that nobody will ever ingest one). Three were inherited from lanes
  that landed on `tranche/9` without a gate run across the merge. **And one this cycle created by its
  own merge resolution:** `git checkout --theirs` on a file *both* lanes had fixed kept the other
  lane's fix and silently dropped this one's. Nothing in the conflict markers showed that a
  non-conflicting hunk was being lost, and the pre-merge suite had been green with that fix in place.
* **Run 2 was killed deliberately** at `root-full`+1, because `origin/tranche/9` advanced twice while
  it was in flight. `verify.sh` reads the **working tree, not a commit**: merging mid-run yields a
  result whose early stages measured one tree and whose late stages measured another. That is not a
  weaker green, it is a green that answers no question. It had already proven the two stages run 1
  failed were fixed (`pi-sweep` clean, `root-full` 6200 across 543 suites, all 524 executed).
* **Run 3 is the result**, on `0588801a`.

**The transferable finding is about concurrency, not about this lane.** Two lanes ran in the same
files for the whole cycle and independently produced the *same* fix three separate times — the
PI-redaction bypass, the `pi-sweep` doc-comment rewording, and the SD-30 scope exemption. Each lane's
gate was green in isolation; several of the reds existed **only** in the merged tree and neither lane
could have seen them alone. Two rules follow, and round 3 of this lane should carry both:

1. **Never resolve a conflicted file both lanes edited by picking a side.** Union the additions, and
   read the non-conflicting hunks of the side you did not take.
2. **A gate run that spans a merge is void.** Re-run from the merged tree, and expect to pay for it.

## Decision 47 — Race-Trait Lane, extend: round 3 (2026-08-12, `sd29-racetrait-r3`, card `epic-6-race-trait-lane-extend`)

> **Numbered 47, not 46, and the collision is recorded rather than quietly fixed.** This round wrote
> its decision as `§46` and pushed two commits (`bd98b9fe`, `eee7f34c`) whose messages say `§46`,
> because the monster lane's round-2 cycle (`sd29-monster-r3`) was running concurrently in another
> worktree and claimed `§46` for its own decision on `origin/tranche/9` first. Neither cycle could
> see the other's claim before pushing — the same failure this bundle already recorded for the
> `SD29-E4-F1-001` cycle-id collision, now repeated for decision numbers.
>
> **Every code comment was updated to `§47`; the two commit messages were not, because they are
> already committed.** A reader who follows `§46` from one of them lands on the monster lane's
> section, which is why this note exists here rather than nowhere: the discrepancy is explained
> where the wrong reference resolves *and* where the right one does. A future concurrent split
> should reserve decision numbers at claim time, exactly as `kanban.md` now says cycle-ids should
> be suffixed.


Round 3 ingested **Horror Adventures** end-to-end — 43 records, **all 43 reaching a player**, the
first book in this lane with no shortfall — and, in reproducing round 2's gate before trusting it,
found that **round 2 had left `origin/tranche/9` RED**, that the **PI screen was being defeated on
the shipped surface**, and that **five alternates across two books were being offered while moving
no number on the sheet**. Two of those three are worth more than the ingest.

The lane is now **dry for no-new-mechanism work**, and this section says so with the command behind
the claim rather than as a judgement.

### 47.1 The ceiling and the queue, re-derived rather than inherited

`§44.4`'s ceiling was re-derived independently, against the PCGen source tree, joined to each unit's
own status — and it **reproduces exactly**: of the corpus's 3,447 `race_trait` units, exactly
**553** carry a `TYPE:<Race> Racial Trait` component naming one of the 18 modelled races. The other
**2,894** belong to races with no chassis and no amount of race-trait ingest grounds them
(`RaceCorpus::resolve` returns `None` without one).

The derivation walks each `race_trait` unit back to its own row by `(book, source_file,
source_line)`, applies `parse_row`'s `.MOD` filter on field 0 only, and reads the row's `TYPE:`
tokens — the same predicates `race_resolver::classify` and `ingest_race_traits::parse_row` use.
Script kept at `scripts/…` is deliberately NOT how this was run: it was a scratch derivation, and
what is durable is the table it produced, reproduced below at both ends of the round.

| book | before round 3 | after round 3 |
|---|---|---|
| `core_essentials` | 175 grounded / 48 not-ingested | unchanged |
| `advanced_race_guide` | 156 grounded | unchanged |
| `inner_sea_races` | 71 grounded / 1 not-ingested | unchanged |
| `advanced_players_guide` | 1 grounded / 49 not-ingested | unchanged |
| `horror_adventures` | **44 not-started** | **43 grounded / 1 not-ingested** |
| `monster_codex` | 4 grounded / 1 not-ingested | unchanged |
| `bestiary` | 3 not-ingested | unchanged |
| **total of the 553** | **407 grounded / 146 residual** | **450 grounded / 103 residual** |

`§45.1`'s method was applied before committing the round, which is the whole point of checking that
script in:

```bash
python3 scripts/classify_race_trait_rows.py ha_abilities_race.lst ha_abilities_race_oa.lst
```

→ `ha_abilities_race.lst`: **in-scope rows 43 | default 0 | alternate 41 | flag_granted 0 |
unclassified 2**; `support/ha_abilities_race_oa.lst`: **in-scope rows 1 | alternate 1**. `§45.5`'s
"44, 42 of them replace-flag alternates" is confirmed, with the 44 correctly split 43 + 1 across two
files — a split `§45.5` recorded and this round had to act on.

### 47.2 The second file is deliberately not ingested, and that is a scope ruling with evidence

`support/ha_abilities_race_oa.lst` is loaded by the book's pcc as

```
ABILITY:support/ha_abilities_race_oa.lst|PRECAMPAIGN:1,INCLUDES=Occult Adventures
```

(`_horror_adventures.pcc:91`). Occult Adventures is not ingested by this repo, so ingesting that
file's one in-scope row would attach a conditional record to the base book unconditionally — the
hazard `loop-instruction.md`'s "Conditional cross-book support files" note names. **The gate is on
the pcc load line, not inside the `.lst`**: `grep PRECAMPAIGN` over the `.lst` itself returns 0, so
a lane that checks the file for its own gate concludes, wrongly, that it is ungated. That is the
same trap `loop-instruction.md` records for `bestiary_5/support/*_oa.lst`, and this is the first
time this lane has actually stood in it.

Recorded as **1 unit of deliberate, evidenced non-scope**, not as gap. It becomes ordinary work the
day Occult Adventures is ingested, and not before.

### 47.3 Round 2 left the branch RED, and its receipt said the gate was "in flight"

**This is the round's most important finding and it is not about race traits.**

Round 2's own receipt records its full gate as in flight, and no result ever landed. Reproducing it
here found **eight failing assertions across three files** on the round-2 tip content:

| file | assertion | held | should have held |
|---|---|---|---|
| `tests/sd27_alternate_racial_trait_reachability.rs` | pure-table vs resolver | 158 | 226 |
| " | selectable keys | 158 | 226 |
| " | all loaded rows | 337 | 409 |
| " | colon-free key sweep | 337 | 409 |
| " | every alternate computes | 158 | 226 |
| " | the reachable-bonus set | 11 entries | 15 |
| `tests/sd27_aasimar_globalvar_gate_closes_the_dead_affordance.rs` | Aasimar alternates | 9 | 11 |
| " | offered-and-acceptable sweep | 158 | 226 |
| `apps/desktop/src-tauri/src/race_trait_picker.rs` | per-race table + sum | 158 | 226 |
| " | Aasimar alternates | 9 | 11 |
| " | changed-description list | 2 keys | 14 |

**The attribution is unambiguous and was checked rather than assumed.** Horror Adventures
contributes **0** Aasimar alternates and **0** entries to the changed-description list, and every
delta above is exactly Inner Sea Races' 72 records / 68 alternates. Round 3 owns fixing them; round 2
owns having shipped them.

Every pin is moved **with its reason**; none is relaxed and none is `#[ignore]`d. Two test names
carrying counts (`exactly_eleven_alternates_…`, `each_of_the_nine_aasimar_alternates_…`) are renamed
to carry none, for `§44.5`'s reason: a name that must be edited alongside its expectation invites
editing the expectation.

**The general lesson.** `§44.5` recorded that "a deferral taken before the evidence lands is a
prediction, not a decision". This is the same finding a third time, in its most expensive form: a
receipt that records a gate as *in flight* is a prediction that it will pass, and the board read it
as a result. A round that cannot land its gate should say the gate did not land — which round 2's
receipt did, honestly — **and the successor must therefore re-run it before building on the work**,
which is what happened here and is the practice to keep.

### 47.4 The PI screen was live and ineffective on the shipped surface

`pi_screening` redacts a record's `description` to `[redacted PI]` and records `pi_field` /
`pi_marker`. But `RaceTraitRecord::render_description` renders from the record's **`DESC:` raw
tokens**, which hold the upstream prose verbatim — and `race_trait_picker` renders every menu row
that way deliberately, because the stored string is an ingest-time collapse.

So for all **12** Inner Sea Races records the screen redacted, the Race Traits panel was rendering
back exactly the Product Identity the screen removed. Worked instance, read off disk:

```
data/corpus/inner_sea_races/race_trait/dwarf/dwarf_stoic_negotiator.json
  description : [redacted PI]
  raw DESC    : Some dwarves, especially those who hail from the town of Peddlegate in Druma, …
```

`Peddlegate` and `Druma` are precisely why the record was redacted, and both reached the panel.

**Redaction that only reaches the stored field is not redaction.** `RaceTraitRecord` now carries
`description_redacted`, read from `CorpusRecordV1`'s `pi_field`/`pi_marker` at load, and
`render_description` returns the stored marker for such a record instead of re-rendering its raw
tokens. `a_pi_redacted_description_is_never_rendered_back_from_its_raw_desc_tokens` pins the
property over all 12 in both directions, and pins the count so the test cannot pass by finding
nothing.

This was silent: nothing errored, the records looked redacted on disk, and the only reason it
surfaced is that the redaction made `rendered != stored` and a *different* test was counting that
difference. Emitted as a retro `incident` with `--silent`, recurrence key
`pi-redaction-defeated-downstream`. **Any other kind whose ingest screens a free-text field and
whose surface re-renders from raw tokens has the same defect shape** — that generalisation is a
finding for a successor, not something this card verified.

### 47.5 Five alternates were offered and moved nothing

The engine's alternate-trait **save** wiring was a single hardcoded constant,
`HALF_ELF_DUAL_MINDED_WILL_SAVE_BONUS`, whose doc comment called it *"the one alternate racial trait
across all 153 whose declared bonus lands on a saving throw this engine totals"*. That was true of
ARG and stopped being true the moment this lane added books.

* `Dwarf ~ Unstoppable` (ISR) — `BONUS:SAVE|Fortitude|1|TYPE=Racial`
* `Half-Elf ~ Mismatched` (HA) — `BONUS:SAVE|Reflex|-2`

and three ISR alternates were missing from the *skill* table for the same reason:
`Gnome ~ Intrepid Settler` (Climb +2, Swim +2 — the only alternate landing on two computed totals at
once), `Half-Elf ~ Sea Legs` (Swim +2), `Hobgoblin ~ Authoritative` (Intimidate +2).

All five were selectable in the picker, persisted on the character, and changed no number on the
sheet: **the browse-only stub class `§44.2` was written about**, shipped five more times. The
constant is now `ALTERNATE_TRAIT_SAVE_BONUSES`, the sibling of the existing
`ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES`, and the three save explanations name their contribution.

Two details worth keeping:

* **The saves sum where the skills maximise**, and that is not an inconsistency.
  `Half-Elf ~ Mismatched` is a **penalty**; maximising would discard it. Summing is correct only
  while no race moves one save twice, so that invariant is now derived from the corpus by
  `no_race_contributes_two_alternate_trait_bonuses_to_one_save` rather than assumed — a future book
  that breaks it fails there, naming the pair, instead of silently getting the wrong arithmetic.
* **The reachability test's own delta check had no arm for Fortitude or Reflex**, although its
  `computed_totals` map declared both. Its `other =>` arm panics rather than skipping, so the gap
  was fail-loud rather than silent — which is the only reason this was found rather than shipped as
  a green test that never checked anything. It is closed, not worked around.

### 47.6 What landed

* `RuleSetId::Ha` + `COMPILED_RULE_SETS` + `corpus_dir_for`/`rule_set_id` + the content-state-dump
  arm. The exhaustive match did its designed job again.
* One `BOOK_SOURCES` row — the whole per-book cost, as that binary's module doc promises. **43
  records**, 0 PCGen-syntax leaks, 0 unresolved `DESC:` args, **0 out-of-scope rows** (the first
  book in this lane with none).
* `data/corpus/horror_adventures/LICENSE.json`. **0 of 43 descriptions were PI-redacted**, against
  12 of 72 for ISR. That is the book *class* — a rules supplement (`BOOKTYPE:Supplement`) rather
  than a campaign setting — not a weaker screen; the identical screen ran over every field.
* **41** `ALTERNATE_TRAIT_REPLACE_FLAGS` rows, adding exactly **1** distinct flag to the corpus's 90
  (`Halfling_ReplaceLanguages`). 28 of this book's 29 flags were already declared by an ARG or ISR
  alternate replacing the same standard trait — the same shape ISR showed, and the reason a book's
  alternate count is a poor predictor of its flag count.
* A reach claim `("horror_adventures", "race_traits")` asserting a plain `Reach::Surfaced`, and
  **no** `OPEN_FINDINGS` / `UNREACHED_RECORD_FINDINGS` entry, because there is no shortfall.
* `race_trait` grounded **407 → 450**.

### 47.7 The one book in this lane with no unreachable record, and why that is a fact about upstream

HA's two non-`Alternate` rows, `Deep Jungle Halfling ~ Languages` and `~ Poison Use`, are
`TraitRole::FlagGranted`, not `Unclassified`: `Halfling ~ Deep Jungle` names both outright through

```
ABILITY:Halfling Racial Trait|AUTOMATIC|Deep Jungle Halfling ~ Languages|Deep Jungle Halfling ~ Poison Use
```

(`ha_abilities_race.lst:85`). That is the **completed** form of the transaction ISR's
`Human ~ Tribalistic Languages` leaves half-finished (`§45.4`): there, the alternate suppresses a
standard row and nothing replaces it; here, the alternate suppresses three and grants both
replacements by name.

So the absence of a finding for this book is evidence, not an omission — and the reach test asserts
`Reach::Surfaced` rather than tolerating a `NotSurfaced`, so a future record that stops reaching
fails by name instead of quietly widening an already-tolerated shortfall.

### 47.8 The remainder — this lane is DRY for no-new-mechanism work

Re-derived at the end of the round by the same join as `§47.1`. Of the 553-unit ceiling, **450 are
grounded** and **103 remain**, and **not one of the 103 is ordinary ingest**:

| book | units | what it needs | class |
|---|---|---|---|
| `core_essentials` | 48 | a `PREABILITY`-grant mechanism **and** the 16 non-`race_trait`-typed subrace selector rows | workable, needs a new mechanism |
| `bestiary` (Drow Noble) | 3 | a race-variant chassis; `Unclassified` by construction without one | workable, needs a chassis |
| `advanced_players_guide` | 49 | nothing — same `KEY:` as already-ingested ARG records (`§39`) | **not gap** |
| `inner_sea_races` | 1 | `Human ~ Tribalistic Languages`, upstream data gap (`§45.4`) | **not gap** |
| `monster_codex` | 1 | `Oversized Goblin`, ability-pool variant mechanism (`§43`) | **not gap** |
| `horror_adventures` | 1 | the `PRECAMPAIGN`-gated Occult Adventures row (`§47.2`) | **not gap** |
| | **103** | | **51 workable / 52 not gap** |

**Round 3 consumed the last book that needed no new mechanism.** `§45.5`'s queue had three entries;
`horror_adventures` was the only one of them not mechanism-blocked, and it is done. A successor
round on this card cannot make progress by ingesting — it must first build one of two mechanisms,
and that is a different shape of cycle than rounds 1-3 were.

**The 2,894 chassis-blocked units remain a scope finding for a race-chassis lane, not this card**,
unchanged from `§44.4` and re-verified here.

## Decision 48 — Companion Lane: the mechanism, and the pilot round (2026-08-12, `sd29-companion-r4`, cards `epic-7-companion-lane-pilot` / `-extend`)

The companion lane had **nothing landed**: round 1 refused at `preflight-disk` and round 2 died with
its workflow having produced no commits. This round built the kind's whole mechanism — chassis,
transcriber, corpus generator, work-inventory grounding, a Tauri catalog command, a browse screen and
reach claims — and ingested **four books, 38 units, all 38 grounded and all four books verified on
screen**. The lane is **not** dry and this round does not claim it is.

### 48.1 The lane's REAL ceiling is 888, not 1,696 — and 765 units can never be grounded by a per-creature cycle

`§45.1` ruled that a lane classifies **corpus rows** before committing a round to a book, and `§46.1`
applied it to `monster_ability` and found a structural ceiling. Applied to `companion`, the finding is
the same shape and larger.

First, `companion` is **not one kind**. `v06_work_inventory::file_kind` types three structurally
different `.lst` shapes as `Kind::Companion`:

* **creature** rows (`*_races_companion.lst`, `*_races_familiar.lst`) — the chassis.
* **ability** rows (`*_abilities_companion.lst`, `*_abilities_race_*companion*.lst`) — features that
  reach a player only underneath the creature that owns them, exactly as `monster_ability` does
  underneath `monster` (`corpus-work-channels.md §9.2`).
* **class** rows (`*_classes_companion.lst`) — the PCGen `Companion`/`Familiar` monster *classes* a
  creature row's `MONSTERCLASS:` token names. Hit-dice progressions; no registered book carries one.

Second, the ownership question. Checked in rather than run from a scratchpad, because `§45.1`'s own
finding is that **an ephemeral path is not a citation**:

```bash
python3 scripts/classify_companion_rows.py
```

It reads three ownership shapes, every one a token the row itself carries: **row-named** (a creature
row's `ABILITY:Special Ability|AUTOMATIC|<name>`), **prerace** (the ability row's own
`PRERACE:1,<Race>`), and **prefix** (a namespaced `KEY:<Owner> ~ <Leaf>` resolved through the
`Companion (<Species>)` / `Familiar (<Species>)` wrapper). A looser rule would over-report
reachability, which is the direction that ships stubs.

Run corpus-wide at this round's start:

```
total companion units in scope : 1696
orphan ability rows            : 808
reachable remainder            : 888
```

and, corpus-wide rather than per book, **765** of those 808 are claimed by no creature row in *any*
book — `ultimate_wilderness` 249, `advanced_players_guide` 188, `ultimate_magic` 132, `core_rulebook`
86, `core_essentials` 40, `book_of_the_damned_volume_1` 27, `advanced_race_guide` 18, `bestiary_3` 16,
`bestiary_4` 5, `bestiary` 4. They are generic companion/familiar/eidolon ability libraries — APG's
`Companion Bonus Skill`, UW's eidolon base forms and archetype abilities — that no creature row
names.

**This is a scope finding, not a backlog item.** They need a surface decision — a screen that shows an
ability with no creature, or a cross-book owner resolution — which is an operator question, not an
ingest. A `deferral` event names both.

**The dispatch brief's "~1,233 in-scope companion units" is corrected to 1,696 total / 888 reachable**
(`correction` event emitted, command in the receipt). 1,233 reproduces under no predicate this round
could find.

### 48.2 The four books this round took, and why they were the correct four

The registration predicate is `monster_chassis`'s, for its reason: **a book is registered when EVERY
one of its ability rows has an owner.** Seven books carry zero orphans; this round took the four
whose registration cost no unmeasured collateral or whose collateral it could measure:

| book | units | creatures | abilities | orphans | needed a `RuleSetId` |
|---|---|---|---|---|---|
| `inner_sea_combat` | 10 | 4 | 6 | 0 | yes (`Isc`) |
| `monster_codex` | 15 | 8 | 7 | 0 | no |
| `inner_sea_intrigue` | 11 | 2 | 9 | 0 | yes (`Isi`) |
| `horror_adventures` | 2 | 1 | 1 | 0 | no |
| **total** | **38** | **15** | **23** | **0** | |

**The package's pinned pilot book was re-confirmed, not trusted.** `inner_sea_combat` was pinned
before the race-trait lane's classifier fix moved 13 units into `companion` corpus-wide, and the
immediately preceding lane discovered its own pinned pilot carried none of the kind it was pinned for
(`kanban.md`'s Epic 6 note). Re-derived here, the pin is correct: 4 creature rows + 6 ability rows,
zero orphans.

**`inner_sea_intrigue`'s 11 units are the ones the race-trait lane handed back.** They were typed
`race_trait` by `file_kind`'s `_abilities_race` substring until that lane's round-2 fix retyped an
`_abilities_race*` basename carrying a `companion`/`familiar` marker as `Companion`. That fix moved
them and left them owned by no lane. This lane owns them, and taking them here coordinates with what
the race-trait lane landed rather than contradicting it.

**Deliberately NOT taken this round:** `bestiary_2` (16), `bestiary_5` (57) and `bestiary_6` (26) —
also zero-orphan, also ready, but each needs a new `RuleSetId` whose scope flip moves several hundred
units of OTHER kinds from `not-started` to `not-ingested`. That collateral is measured and cheap to
pay, but it is a count change, and this round already had two of them to sweep. They are round 2's
queue, named with their figures in `§48.6`.

### 48.3 The mechanism, and the one place it deliberately diverges from the monster chassis

`companion_chassis.rs` mirrors `monster_chassis.rs` — record types, a `COMPANION_BOOKS` registry every
consumer iterates rather than naming books, and the same both-directions link test — with three real
differences, each forced by the corpus:

1. **`PRERACE:` ownership has no monster analogue.** Every `TYPE:CompanionAdvancement` row states its
   owner in its own `PRERACE:1,Companion (<Species>)` gate rather than being named by the creature's
   row. A chassis that knew only the monster shape would have reported all 11 of the registered
   books' advancement rows as orphans and rejected every book.
2. **The prefix shape needs the species wrapper.** `Worg ~ Mastery` is owned by `Companion (Worg)`;
   `monster_chassis`'s bare-prefix rule (`<Owner>` is a monster key) does not match, and Inner Sea
   Combat would have been unregisterable over one row.
3. **`facet` is `Option`al, and `type_segments` keeps every segment verbatim.** Inner Sea Intrigue's
   three `TYPE:ClockworkFamiliarInstalledItem` rows carry no segment the chassis models.
   `transcribe_monster_tables.parse_type` hard-stops on that; here the row is recorded with
   `facet: None` and its segment kept, and the screen shows the verbatim segment where a facet label
   would go. Dropping the rows or forcing them into `SpecialQuality` would both assert something the
   corpus does not, and all three carry real rules text a player can read.

**One thing that is emphatically NOT a divergence: `BONUS:STAT` is transcribed as an *adjustment* and
never summed into a score.** A Griffon's row states `BONUS:STAT|STR|6` and a Griffon's Strength is not
6; PCGen computes the real score at runtime from a base plus this token plus the companion class's own
level advance. The wire carries the abbreviation and the signed adjustment, the screen's caption reads
*"Ability score adjustments (corpus BONUS:STAT tokens)"*, and a frontend test pins that the caption
says "adjustment". Under an "Ability scores" heading these numbers would be the quieter lie — the same
discipline `MonsterStatBlock::monster_class` states for the hit-dice token, which this ingest also
carries verbatim and does not expand into hit points, AC or saves.

The corpus writes **one** directory, `data/corpus/<book>/companion/`, holding both shapes with each
record stating its own `record_type`. Two directories would create a corpus family the inventory has
no kind for, and the two halves would then be judged against a denominator that does not exist.

### 48.4 The surface: a real catalog, because the Pets tab is a different thing and always was

Before this round the entire `companion` kind reached no surface. The engine's only companion content
was `pilot_compute::ground_wolf_companion_stat_block` and `ground_horse_companion_stat_block` — two
species whose values are Rust constants chosen for the pilot vertical slice, not corpus reads.
`monster_catalog.rs`'s module doc already said so in as many words: *"The Pets tab does not count and
never did."* That tab shows the computed companion of the character in front of you and can never show
a Griffon or a Clockwork Spy.

So: `companion_catalog.rs` (`list_companion_catalog`), `CompanionCatalogScreen.tsx`, and a
"Browse Companion Catalog" link on the hub, all in `monster_catalog.rs`'s shape rather than a third
convention. Abilities are served **attached to the creature that owns them**, which is how the screen
renders them and why the reach claim judges one denominator with two numerators.

`reach_gate.rs` gets **one claim per book**, not two — `("<book>", "companions")` — because the corpus
files both shapes under one kind. Each is `Reach::Surfaced` with all its records:
`inner_sea_combat` 10, `monster_codex` 15, `inner_sea_intrigue` 11, `horror_adventures` 2.
`every_ingested_companion_book_reaches_the_catalog_record_by_record` asserts the corpus denominator,
the served numerator and the claim independently, so a table that stopped reaching the wire fails
rather than agreeing with itself.

### 48.5 Item 8 caught a calibration error before it produced a false pass

All four books were driven on screen and all four PASS. The finding is in how the first run failed.

`verify-on-screen.sh`'s per-family `SEARCH_Y` is a live-calibrated constant. The companion screen
carries ONE facet-chip row (books) where the monster screen carries two, so `285` — the value every
other single-chip-row family uses — was the obvious analogy. It lands **below** this screen's search
box: the query never applied, and the harness's own filtered-count gate refused the run with *"still
shows 15 rows — filter did not apply"*. Calibrated live, the value is **247**.

Without that gate the run would have screenshotted the **unfiltered** 15-row list, found the record's
name in a select-all extraction of the whole page, and written a PASS artifact proving nothing about
the specific record. That is precisely the class of defect item 8 exists to catch, caught by item 8's
own harness on a family it had never seen. A `near-miss` event records it; the refused run's
`isc-companion-griffon.FAILED.verify.md` is kept beside the passing artifacts rather than deleted,
because the harness names a failure so it can never be cited as evidence and the failure is worth
having.

The four passing artifacts are under
`docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F1-002/item8/`.

### 48.6 The remainder, re-derived — this is round 2's starting point

Re-derived at the end of the round with the same commands that opened it, so the two are commensurable:

```bash
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); \
u=[x for x in d['units'] if x['kind']=='companion']; \
print(len(u), collections.Counter(x['status'] for x in u))"
python3 scripts/classify_companion_rows.py
```

`companion` totals **1,696 / 38 grounded / 1,658 remaining**. But **1,658 is not the lane's workload**:
subtracting the 765 corpus-wide orphans leaves **893 reachable-in-principle**, of which 38 are done, so
the honest remainder is **855** — and only part of that is ordinary ingest.

Round 2's queue, all zero-orphan and needing no new mechanism, only a `RuleSetId` and a `BOOKS` row:

| book | companion units | collateral of its scope flip |
|---|---|---|
| `bestiary_5` | 57 | its other kinds move `not-started` → `not-ingested` |
| `bestiary_6` | 26 | same |
| `bestiary_2` | 16 | same |
| **total** | **99** | |

After those, every remaining book carries orphans and the lane needs a per-book judgement (register the
book and record its orphans as an `OPEN_FINDINGS` shortfall, or wait on `§48.1`'s operator ruling)
rather than another repetition of this round's shape.

### 48.7 The scope flip's collateral, measured

Adding `RuleSetId::Isc` and `RuleSetId::Isi` moved both books from `future_state` to `in_scope`, which
moves every other kind in them from `not-started` to `not-ingested` — the cost `§46.2` first measured
for the monster lane. Measured here: `inner_sea_combat` **388** units across 3 other kinds,
`inner_sea_intrigue` **245** across 6. Neither moves this lane's denominator (both statuses count as
remaining); both move other lanes' `not-ingested` figures, which is why it is recorded.

It also turned `tests/v06_work_inventory.rs`'s `sd30_campaign_setting_books_appear_in_the_inventory_as_not_started_books`
RED for both books, exactly as Inner Sea Races turned it RED for the race-trait lane. Both were added
to `SD29_INGESTED_CAMPAIGN_SETTING_BOOKS` **with their reason**, which is `§47.3`'s ruling applied
again rather than relaxing the check: the roster assertion is about SD-30's sixteen books existing, not
about them staying un-ingested forever.

## Decision 49 — Race-Trait Lane, extend: round 4 (2026-08-12, `sd29-racetrait-r4`, card `epic-6-race-trait-lane-extend`)

> **Written as §48 and renumbered on merge.** The companion lane's
> `sd29-companion-r4` claimed Decision 48 concurrently in a separate worktree and
> pushed first, so this section is §49. The two commits that landed before the
> merge (`9176f869`, `c8416f33`) say §48 in their messages and cannot be
> rewritten; every code comment was updated to §49. This is the second instance
> of the class in this lane — `§47` carries the same note for the same reason —
> and `kanban.md` already records the fix: reserve a decision number at claim
> time, as cycle-ids are suffixed.
Round 4 ingested **Core Essentials' Aasimar and Tiefling heritages** — 64 records, all 64 reaching a
player — which was the whole of `§47.8`'s "workable, needs a new mechanism" queue except three
chassis-blocked rows. **It needed no new mechanism.** `§47.8`'s statement of what the book required
was wrong in both halves, and correcting it is this round's most reusable output. The lane is now
**dry**: the genuinely-workable remainder is **3 units**, and all three need a race *chassis*, which
is not this card.

### 49.1 The stated blocker was not the blocker, and the corpus said so in a third file

`§47.8` recorded `core_essentials`' 48 as needing *"a `PREABILITY`-grant mechanism **and** the 16
non-`race_trait`-typed subrace selector rows"*, and `§45.1` had already reasoned that ingesting them
as-is "would have produced records that load and never apply."

The second half is right. The first half named the wrong mechanism, and the reason is worth more
than the fix.

The 48 rows do carry a positive `PREABILITY:1,CATEGORY=Special Ability,<Race> ~ <Heritage>` gate, and
`race_resolver::classify` genuinely does not read that token. So a reader who classifies the rows by
**what is on them** concludes a `PREABILITY`-grant mechanism is required. What that reading cannot
see is that PCGen states the same transaction a second time, from the other end, in a file the
lane had never opened:

```text
CATEGORY=Special Ability|Aasimar ~ Agathion-Blooded.MOD
    ABILITY:Aasimar Racial Trait|AUTOMATIC|Agathion-Blooded ~ Ability Scores|PREVAREQ:Aasimar_ReplaceAbilityScores,0
    ABILITY:Aasimar Racial Trait|AUTOMATIC|Aasimar ~ Type|PREVAREQ:Aasimar_ReplaceType,0
    ...
```

(`core_essentials/races/aasimar/aasimar_abilities_globalvar_subrace.lst`.) That is
`ABILITY:<cat>|AUTOMATIC|<key>` — **the third grant shape `race_resolver`'s module doc already
documents and `link_automatic_grants` already resolves**, the one `Orc ~ Feral` → `Feral ~ Languages`
travels. The heritage names its replacements outright; nothing new is needed in the engine at all.
The `PREVAREQ:<flag>,0` qualifier on each grant names the standard trait being displaced, which
supplies the heritage's `sets_replace_flags` and makes it an ordinary `TraitRole::Alternate`.

So the whole book cost **one ingest-side reader** (`ingest_race_traits::subrace_grants`, ~40 lines)
and **zero resolver changes**. `race_resolver.rs`'s diff for this round is a table of 16 rows and
three moved count pins.

**The general lesson, and it is `§45.1`'s exactly one level further out.** `§45.1` established that a
lane must classify **corpus rows**, not the inventory's evidence token. Round 4 shows that
classifying the corpus rows *of one file* is still not the same as classifying the corpus: the rows
carried a gate the engine cannot read, and the file that made them ordinary was three directories
away with a name (`_abilities_globalvar_subrace.lst`) that no row-level scan would reach. The
question "what does this content need?" is answered by the **book**, not by the file the rows live
in — and `ingest_races::globalvar_gates` had been reading the non-subrace half of that very file
family since SD-27, which is the precedent a search would have found.

`scripts/classify_race_trait_rows.py` was run before the round committed, per `§45.1`, and its
output is what raised the question rather than settling it:

```bash
python3 scripts/classify_race_trait_rows.py \
  aasimar_abilities_race_subrace.lst tiefling_abilities_race_subrace.lst
```

→ `aasimar…`: **in-scope rows 18 | default 0 | alternate 0 | flag_granted 18 | unclassified 0**;
`tiefling…`: **in-scope rows 30 | default 0 | alternate 0 | flag_granted 30 | unclassified 0**.
`§47.8`'s 48 is confirmed, and "**0 of 48 need no new mechanism**" is exactly the output that should
send a round looking for the other end of the transaction rather than budgeting for a new engine
feature.

### 49.2 The ceiling is 571, not 553, and the 18 extra rows are a category the earlier derivation could not see

`§44.4`'s and `§47.1`'s ceiling — of the corpus's 3,447 `race_trait` units, **553** carry a
`TYPE:<Race> Racial Trait` component naming one of the 18 modelled races — **reproduces exactly**,
independently derived this round against the PCGen source tree over the bundle's own 38 book
directories:

```bash
python3 scripts/race_trait_ceiling.py          # checked in this round
```

→ `TYPE:<18 races> Racial Trait rows : 553`.

**And 553 was never the whole ceiling.** The predicate reads one TYPE suffix, and the heritage
*selector* rows carry a different one — `TYPE:Aasimar Subrace`, `TYPE:Tiefling Subrace`. There are
**18** such rows corpus-wide, all in `core_essentials`, and they are `race_trait`-kinded units in
`docs/work-inventory.json` exactly like the 553 because `file_kind()` types them by filename. They
were counted in the lane's *denominator* and excluded from its *ceiling*, which is the one
combination that makes a lane look further from done than it is.

The honest ceiling is therefore **571**, and the same script prints both halves so the split stays
visible rather than being folded into one number.

**Two of the 18 are deliberately not gap**, and they are the reason the selector category is not
simply "16 more records": `Aasimar ~ Default` and `Tiefling ~ Default`, in the books' *non*-subrace
`<race>_abilities_race.lst`, are the **no-heritage baseline**. `Tiefling ~ Default` states its own
role outright —
`PREMULT:1,[PREABILITY:1,…,Tiefling ~ Default],[!PREABILITY:1,CATEGORY=Special Ability,TYPE=Tiefling Subrace]`,
read *you have this unless you have some Tiefling subrace*. This engine's "no alternate selected"
state already **is** that record; ingesting it would ship a selectable heritage that sets no flag and
grants nothing, which is the browse-only stub `§44.2` is about. Recorded as evidenced non-scope.

### 49.3 Giving a storage directory a rule set demoted 155 records that had not changed

**This is the round's most important finding and it is not about heritages.**

`v06_work_inventory`'s `race_trait` verdict grounded a unit when the corpus probe's observed book
**equalled the unit's own book's rule set**:

```rust
if facts.race_trait_engine_book(unit) == Some(engine_book.as_str()) { /* grounded */ }
```

For every book whose `.lst` rows are filed under itself, that equality is free. `core_essentials` is
the one book where it is not — `race_trait_engine_book`'s own doc comment says so in as many words:
*"Race traits are the one kind whose `.lst` rows are routinely filed under a different book than the
one that ingested them."* While the book had no compiled rule set, the shared-library path resolved
`engine_book` to the record's real host (`core_rulebook`, `bestiary_1`) and the equality held.

Round 4 gave `core_essentials` a `RuleSetId` of its own, for the 64 records that genuinely belong to
it. **155 Core Rulebook and Bestiary 1 standard racial traits stored in that directory instantly
dropped from `grounded` to `race_trait_record_loaded_but_never_applies`** — an evidence token
asserting the *opposite* of what the probe had just observed — and `race_trait` grounded went
450 → 359 in the same run that added 64 records. Nothing about those 155 changed; only the directory
they live in gained an id.

Caught because the round re-derived the denominator after the ingest instead of only before it, and
the number moved the wrong way. **A count that moves in the wrong direction is the cheapest defect
detector this program has, and it only works if the count is taken twice.**

Fixed at the source: the probe's observation grounds on its own, and reports the observed book as the
attribution — exactly as a shared-library record was attributed before its own book was named. After
the fix, `race_trait` grounded is **514** (450 + this round's 64), which is the arithmetic the round
should produce.

**The generalisation is a live hazard for the other lanes.** Any kind whose units are stored under a
different book than the one that owns them will demote silently the day that storage directory is
given a rule set of its own. `race_trait` is the kind whose doc comment names the property; nothing
proves it is the only one.

### 49.4 A placeholder page cite is not a page

All **64** of the book's rows carry a `SOURCEPAGE` that is a placeholder — `p.xx` on all 40 Tiefling
rows, `xx` on all 24 Aasimar ones. The picker's own
`every_alternate_carries_real_book_attribution_and_prose` already refused `p.xx`, which caught the
Tiefling half; the Aasimar spelling `xx` would have sailed through and rendered "xx" beside the trait
as though it were a real citation.

None of the four books ingested before this one carries a placeholder at all:

```bash
grep -oh 'SOURCEPAGE:[^\t]*' \
  arg_abilities_race.lst mc_abilities_race.lst isr_abilities_race.lst ha_abilities_race.lst \
  | sort -u | grep -i x
```

→ no output. So `ingest_race_traits::is_placeholder_source_page` is an exact-match list of the two
spellings the corpus actually uses, not a pattern — a page cite is free text and a heuristic would
start discarding real ones. The rows still ship, with their name, prose and bonuses; they ship with
no page rather than a false one, and the picker's `pageless` pin now names all 18 rows by key.

### 49.5 The heritages are mutually exclusive, and the corpus says so in the token this round already reads

A heritage carries **no `PREMULT` self-exclusion guard** — upstream, only one can apply because a
heritage is a PCGen `SUBRACE` and a character has one. `race_trait_picker::exclusion_guard_flags` read
`PREMULT` and nothing else, so all 16 would have come back unguarded and a player could have ticked
`Aasimar ~ Angel-Blooded` and `Aasimar ~ Archon-Blooded` together and collected **both** ability-score
bonuses.

The constraint is stated on the grant: `…|AUTOMATIC|Angel-Blooded ~ Ability Scores|PREVAREQ:Aasimar_ReplaceAbilityScores,0`
reads *grant this while that standard trait has not already been replaced*, which is the same
"already set by someone else blocks me" relation the `PREMULT` branch expresses. So the ingest
carries the qualifier through verbatim and the picker reads it as a third spelling of the guard.
`every_alternate_has_a_readable_exclusion_guard_including_the_preability_spelling` — which pins the
unguarded set by exact key — went RED and came back green with the pin **unmoved**, which is the
evidence that these 16 are guarded rather than exempted.

### 49.6 Aasimar proves the read that Tiefling needs

Aasimar's six selector rows carry their own `FACT:Aasimar_Replace<Trait>|True` tokens. Tiefling's ten
carry **no `FACT:` token at all**. Both books' `_abilities_globalvar_subrace.lst` state the same
thing the same way, so the ingest asserts equality wherever both sources speak:

* Aasimar: declared `[Aasimar_ReplaceAbilityScores, Aasimar_ReplaceSkilled, Aasimar_ReplaceSpellLikeAbility]`
  == derived from the globalvar block. Six for six.
* Tiefling: nothing declared; derived
  `[Tiefling_ReplaceAbilityScores, Tiefling_ReplaceSkilled, Tiefling_ReplaceSpellLikeAbility]`.

That agreement is what licenses reading the globalvar file for the ten rows that say nothing — it is
the same discipline `ingest_races::globalvar_gates` documents for the base races ("checkable against
the first source and *is* checked"), applied one directory down. A contradiction fails the ingest run
rather than being resolved silently in either direction.

### 49.7 What landed

* `RuleSetId::Ce` + `COMPILED_RULE_SETS` + `corpus_dir_for`/`rule_set_id` + the content-state-dump
  arm. The exhaustive match did its designed job again.
* `BookSource.lst_relatives` is now a **list**: a book may declare its racial traits across more than
  one file, and two `BookSource` rows sharing one `corpus_book` would have had the second silently
  erase the first's records (`ingest_book` rebuilds the output tree per book). Horror Adventures
  already had two such files and dodged this by ingesting only one of them.
* `ingest_race_traits::subrace_grants` + `is_placeholder_source_page` + the `TYPE:<Race> Subrace`
  arm in `parse_row`. **64 records**, 0 PCGen-syntax leaks, 0 unresolved `DESC:` args, 0
  out-of-scope rows.
* `data/corpus/core_essentials/LICENSE.json`. **8 of 64 descriptions PI-redacted** — four Tiefling
  heritages named for outsider races that are Golarion Product Identity, each hitting twice because
  the heritage row and its Ability Scores replacement row carry the same prose. Aasimar's 24 hit 0.
  The book's OGL posture is the one row `docs/governance/license-matrix.md` marks **unestablished
  from its own file**, and the declaration cites that ruling rather than restating it: every
  `#ISOGL:YES`/`#COPYRIGHT:` line in `_core_essentials.pcc` is commented out, and the book is
  reachable only through `core_rulebook.pcc:43`'s unconditional `PCC:@…` inclusion — which is the
  exact path this ingest travels.
* **16** `ALTERNATE_TRAIT_REPLACE_FLAGS` rows, adding exactly **2** distinct flags to the corpus's 91
  (`Aasimar_ReplaceAbilityScores`, `Tiefling_ReplaceAbilityScores`). No alternate in any earlier book
  replaces a race's ability-score row, because an ordinary alternate never touches ability scores and
  a heritage always does; the other four flags were already declared by ARG and ISR alternates
  replacing the same standard rows. Both new flags are claimed by a real standard row, which is why
  the orphan-flag assertion did not move.
* A reach claim `("core_essentials", "race_traits")` asserting a plain `Reach::Surfaced`, and **no**
  `OPEN_FINDINGS` / `UNREACHED_RECORD_FINDINGS` entry, because there is no shortfall. Its test pins
  both halves — 64 records reached and 16 menu rows — because they fail independently: losing the
  grant link would leave 16 perfectly selectable records that change nothing.
* The `v06_work_inventory` attribution fix of `§49.3`.
* `race_trait` grounded **450 → 514**.

### 49.8 The remainder — this lane is DRY

Re-derived at the end of the round by the same join `§47.1` used, widened to the 571-row ceiling
(`scripts/race_trait_ceiling.py`, then joined to each unit's status by
`(book, source_file, source_line)`):

```
units matched into the ceiling : 571
by status                      : {'grounded': 514, 'not-ingested': 57}
```

| book | units | what it needs | class |
|---|---|---|---|
| `advanced_players_guide` | 49 | nothing — same `KEY:` as already-ingested ARG records (`§39`) | **not gap** |
| `bestiary` (Drow Noble) | 3 | a race-variant chassis; `Unclassified` by construction without one | **workable, needs a chassis** |
| `core_essentials` | 2 | nothing — `Aasimar ~ Default` / `Tiefling ~ Default`, the no-heritage baseline the engine's own no-selection state already is (`§49.2`) | **not gap** |
| `horror_adventures` | 1 | `Half-Elf ~ Starchild`, the `PRECAMPAIGN`-gated Occult Adventures row (`§47.2`) | **not gap** |
| `inner_sea_races` | 1 | `Human ~ Tribalistic Languages`, upstream data gap (`§45.4`) | **not gap** |
| `monster_codex` | 1 | `Oversized Goblin`, ability-pool variant mechanism (`§43`) | **not gap** |
| | **57** | | **3 workable / 54 not gap** |

**The genuinely-workable remainder of this card is 3 units, and they are not race-trait work.** Drow
Noble is a race variant with no chassis; `RaceCorpus::resolve` returns `None` for it, so ingesting
its three rows would produce records that load and never apply whatever this lane does next. That is
the race-chassis lane, and this card should be read as dry rather than as 3 short.

**The chassis-blocked residue is 3,447 − 571 = 2,876 units** (`bestiary_3` 799, `core_essentials`
645, `bestiary_2` 162, `ultimate_psionics` 159, and so on) — races the product does not model, for
which no amount of race-trait ingest changes anything. `§44.4`'s 2,894 was right on its own 553-row
ceiling and is superseded only by the 18 selector rows `§49.2` adds, not corrected.

**One scope finding for a successor, outside this bundle.** `scripts/race_trait_ceiling.py
--whole-tree` scans every book under the PCGen data root instead of the bundle's 38 directories and
returns **897**, against 571 in scope — **326 further rows**, and 291 of them are ordinary
Pathfinder alternate racial traits for the 18 races this product already models:

| tree | rows | note |
|---|---|---|
| `player_companion/` (12 books) | 288 | `blood_of_fiends` 102, `blood_of_angels` 101, `blood_of_shadows` 19, `bastards_of_golarion` 12, `legacy_of_the_first_world` 12, `legacy_of_dragons` 11, `kobolds_of_golarion` 10, `heroes_of_the_street` 7, `heroes_of_the_high_court` 7, `heroes_of_the_wild` 3, `agents_of_evil` 3, `dragon_empires_primer` 1 |
| `campaign_setting/` (2 books) | 3 | `rival_guide` 2, `dragon_empires_gazetteer` 1 |
| `starfinder/` | 35 | correctly out of scope for a Pathfinder product |

The 291 are **ordinary no-new-mechanism ingest of exactly the shape rounds 2 and 3 did**, and two of
those books individually exceed anything this lane has taken since ARG. They are not in
`corpus-work-channels.md §10.2`'s 37, so they are not this bundle's to take — but they are the
largest known block of cheap race-trait work anywhere in the tree, and a successor bundle should be
told they exist rather than rediscovering them a round at a time.

## Decision 50 — Monster / Monster-Ability Lane, extend: round 3 (2026-08-12, `sd29-monster-r5`, card `epic-5-monster-lane-extend`)

Round 3 ingested **Inner Sea World Guide** — 23 of its 44 monster-family units — and found, in the
process, that this program has never read the corpus's own per-record Product Identity declaration.
Two of the five rows that carry it would have shipped past the existing screen. The lane is not dry
and this round does not claim it is.

### 50.1 `NAMEISPI:YES` is an upstream PI declaration and nothing in this repo read it

`§46.4` closed round 2 with a derived generalisation: *"'Campaign setting' predicts a PI hit rate
for `race_trait` and does not predict one for `monster`"* — a monster row is a stat block, not
setting prose, and both Book of the Damned volumes read `records_redacted: 0`.

**That held for two books whose creatures are Open Game Content devil and demon types, and it does
not hold for a book whose creatures are named after Golarion places and deities.** Inner Sea World
Guide is a `campaign_setting/` book, and `gen_book_cache`'s PI screen hard-stopped on it at the
first attempt: four records carried `Urgathoa`.

Following that stop into the corpus rows found something the term list could not have:

```bash
grep -c 'NAMEISPI:YES' iswg_races.lst iswg_races_bestiary.lst   # -> 3, 2
```

| row | file:line | why it is PI on its face |
|---|---|---|
| `Daughter of Urgathoa` | `iswg_races.lst:13` | a named Golarion deity |
| `Sandpoint Devil` | `iswg_races.lst:14` | a named Golarion town |
| `Treerazer` | `iswg_races.lst:16` | a unique named NPC |
| `Boar (Sargavan)` | `iswg_races_bestiary.lst:13` | a named Golarion nation |
| `Herd Animal (Storval Aurochs)` | `iswg_races_bestiary.lst:14` | a named Golarion region |

`NAMEISPI:YES` is PCGen's own per-record marker that a record's NAME is Product Identity. **This
repository's entire PI apparatus — `pi_screening::PI_BLACKLIST_TERMS`, `pi_table_sweep`,
`gen_book_cache`'s hard stop — reads a hand-maintained 55-term list and nothing else.** Only three
of the five above are on that list. `Boar (Sargavan)` and `Herd Animal (Storval Aurochs)` carry
place names it does not contain and **would have shipped**.

**The marker and an independent reading agree**, and that agreement is the point rather than an
aside: a proxy is only usable once it has been checked where it makes its confident claim. Each of
the five is Product Identity under OGL §1(e) on inspection, not merely by upstream assertion. Had
they disagreed — had the marker flagged a generic species name — the correct conclusion would have
been that the token is not a PI oracle for this program, and this decision would read the other way.

### 50.2 One record already shipped with the marker set, in another lane's territory

The finding does not stop at this lane:

```bash
grep -rl 'NAMEISPI:YES' <every ingested book's .lst tree>
#   -> inner_sea_races/isr_abilities_race.lst:67   `Elf ~ Sovyrian-Born`
grep -rl 'Sovyrian' data/corpus/
#   -> data/corpus/inner_sea_races/race_trait/elf/elf_sovyrian_born.json
```

**That record is on `tranche/9` now.** "Sovyrian" is a Golarion place name, it is not in
`PI_BLACKLIST_TERMS`, and no screen fired. The four books already carrying the monster chassis are
clean (`grep -c` → 0 on all four), so this lane's own output is unaffected; the race-trait lane's is
not.

**Not fixed by this round, and the reason is a rule this bundle already paid for.** The remedy is a
`PI_BLACKLIST_TERMS` addition under `docs/governance/ogl-pi-blacklist.md` §3's per-book-override
template — the same mechanism that folded in `Jarn`. That is a **corpus-wide** change: it alters
what every book's generator redacts, and its blast radius cannot be verified inside this card's
write scope without re-running other lanes' generators. `§46.6` rule 1 exists because two lanes
working the same files cost this bundle an entire gate run. So it is reported with its command and
its file path rather than reached for. `incident` emitted with `recurrence-key
pi-declaration-token-unread`.

**The general form, which is not about this token.** A screen built from a list somebody maintains
is only as complete as that person's imagination. A screen that reads a *declaration the source data
already makes* is complete by construction for every record that carries it. This program had the
second kind of evidence available in every `.lst` file it has ever parsed and used the first kind
exclusively.

### 50.3 A key cannot be redacted, so PI rows are dropped rather than screened

`pi_screening` redacts a `description` and records `pi_field`. It has no answer for a record whose
*identity* is the Product Identity: `[redacted PI]` as a monster's key is a record nobody can look
up, and one whose `owners` links break.

So the transcriber **drops** the row and names it, with its file:line and its reason, in the
generated module header. Nothing is reclassified — reclassification is `ogl-pi-blacklist.md` §3's
per-book override and an operator decision. `deferral` emitted for all 21 excluded units.

Dropping a monster **cascades**: an ability whose only owner is gone reaches nothing either. This
book's orphan count went from 5 (the `iswg_templates.lst`-owned rows the classifier saw) to 13. The
transcriber therefore runs the PI screen **before** the orphan screen, and the ordering is
load-bearing rather than incidental: run it the other way and `Constant ~ Desecrate` is reported as
a PI hit on its *owner's* name when the true reason is that it has become an orphan. Each row is
reported under the reason that actually applies to it.

### 50.4 A screen that is too broad is a defect in the other direction

The first draft of the transcriber's term screen read **every token of the corpus row** — the
instinct being that over-inclusion is the safe direction for a PI screen. It dropped the Sandpoint
Devil for:

```
AUTO:LANG|Abyssal|Varisian
```

a language grant that never reaches a record, on the blacklist term `Varisia` matching as a
substring of `Varisian`. **Over-exclusion silently deletes corpus content nothing was going to
publish**, and it does so in a way no gate can see, because the missing record simply never exists.

The screen now reads exactly the values the transcription **emits**, which is exactly what
`gen_book_cache` serializes and screens in turn — the two are the same set by construction rather
than by coincidence. The Sandpoint Devil is excluded anyway, by `NAMEISPI:YES`, which is the right
reason.

The term list itself is now **parsed out of `src/rules_core/pi_screening.rs`** rather than repeated
in Python, and the parser refuses to run if it finds fewer than 20 terms. A copy would drift the
first time §3's per-book override adds one — which is exactly what `§50.2` will eventually need.

### 50.5 A book's monsters may live in two files, and the line numbers collide

Inner Sea World Guide splits its 14 monster rows 7/7 across `iswg_races.lst` (beside the book's
player races) and `iswg_races_bestiary.lst`. Their line numbers overlap:

```
iswg_races.lst:10          -> Aluum
iswg_races_bestiary.lst:10 -> Fennec (Firefoot)
```

`MonsterStatBlock` carried only a `source_line`, and `gen_book_cache` took the file from a single
per-book spec string. Under that model every row of one of the two files would have been
citation-checked against the other — and `verified_citation_line` compares the cited line's first
column against the record's name, so it would have surfaced as a citation failure rather than as the
modelling gap it is. `MonsterStatBlock::source_file` is new,
`MonsterBookSpec::races_lst` became `races_lsts: &'static [&'static str]`, and the generator looks
the file up per record and panics by name on a file the spec does not list.

**The four already-shipped books reproduce byte-for-byte under every change above except that one
new field** — checked by a unified diff of the two versions' table bodies, not by `git diff --stat`.
Bonus Bestiary's committed file keeps the pilot's hand-authored header (round 2's own recorded
treatment) and had the field inserted rather than being regenerated.

### 50.6 The card's `OPEN_FINDINGS` instruction is not satisfiable, and the divergence is recorded

`kanban.md`'s round-3 note says a book with orphans lands *"the claim scoped to the linked subset
with an `OPEN_FINDINGS` entry for the rest."* The first half is exactly what happened. The second is
mechanically unavailable, and pretending otherwise would have meant weakening a gate.

`reach_gate`'s own findings test asserts, in both directions, that a recorded finding names a family
that reaches **nothing**:

```
"these families now reach a surface — delete their OPEN_FINDINGS entries: {}"
```

`inner_sea_world_guide/monster_abilities` reaches `list_monster_catalog` for all 14 of its shipped
records, so an entry for it is `stale` by that test's own definition. The list is for a *family* that
does not reach, not for rows a cycle chose not to ingest — those are not records at all, so there is
nothing for the gate to find unreached.

The exclusions are held instead by four named tests
(`the_five_product_identity_names_are_not_records`, `no_shipped_ability_is_an_orphan`,
`every_owner_named_by_a_shipped_ability_is_a_shipped_monster`, and
`no_shipped_field_carries_a_product_identity_term`, which checks the property against the live
blacklist rather than a list of names), and by the generated header, per row, with reasons. **A
round that had followed the card literally would have added a finding the gate rejects; a round that
had ingested the rows to make the finding true would have shipped five Product Identity names.**

### 50.7 What landed, and the remainder

| book | monster units | ingested | monster_ability units | ingested |
|---|---|---|---|---|
| `inner_sea_world_guide` | 14 | **9** | 30 | **14** |

`records_redacted: 0` in the book's `LICENSE.json` — which is now a *weaker* statement than it looks,
and is why `§50.1` exists: zero redactions is what a book reads when its PI is in the keys.

Denominators, by the command rounds 1 and 2 both recorded (sum `not-ingested` + `not-started` for
both kinds over every non-`out_of_scope` book in `docs/work-inventory.json`):

* **Before:** `monster` 1,199 + `monster_ability` 3,034 = **4,233**. Round 2's closing figure,
  reproduced exactly before being moved.
* **After:** `monster` 1,190 + `monster_ability` 3,020 = **4,210**. `units_ingested` = **23**.
* Grounded: `monster` 71 → **80**, `monster_ability` 73 → **87**.

`scripts/classify_monster_ability_rows.py`, run at cycle start **as round 2 left it**, reproduces
`§46.1`'s **2,906** reachable remainder exactly — that figure is confirmed, not corrected.

**Then the instrument turned out to be over-reporting, and fixing it moves the ceiling more than
this round's ingest did.** The classifier resolved every ability's link against every monster ROW,
including rows that carry `NAMEISPI:YES` and can never be shipped. For this book it called 11 of
the 16 remaining abilities reachable when their owners are the five PI rows, and counted the 8 PI
rows themselves as reachable units — 16 units of phantom work in one book. It now reads Product
Identity first (both signals, the term list parsed out of `pi_screening.rs`) and resolves links
against **shippable** monsters only:

```
remaining monster+monster_ability units : 4210
orphan monster_ability rows             : 1405
  of which in ZERO-monster books        : 703 across 10 books
Product Identity rows (never shippable)  : 32
reachable remainder (units - orphans - PI): 2773
```

**The lane's REAL ceiling is 2,773, not 2,906.** The 133-unit difference is not this round's 23
ingest; it is work that was never available. And the effect is not confined to a campaign-setting
book: `bestiary_4`, the largest unstarted book in the lane, carries **14** PI rows whose removal
turns **73** more of its abilities into orphans (152 → 225).

**The general form.** `§45.1` ruled that a lane must classify corpus rows rather than trust an
inventory token, and `§46.1` built the instrument that does it. This round is the next turn of the
same screw: **an instrument that classifies rows is itself a proxy, and it needs validating where
it makes its confident claim.** It said "reachable" about 16 rows that a hard stop in the generator
refuses to write.

**Round 4's queue, and what it must check first.** the corrected classifier now answers this in one
command, and it inverts the ranking round 2 wrote:

| book | remaining units | orphans | PI | **reachable** |
|---|---|---|---|---|
| `bestiary_2` | 782 | 64 | **0** | **718** |
| `inner_sea_bestiary` | 230 | 26 | 7 | **197** |
| `inner_sea_world_guide` | 21 | 13 | 8 | **0** |

`bestiary_2` is a `roleplaying_game/` bestiary and carries **zero** PI rows — which
`ogl-pi-blacklist.md` §2's own note predicts, since classic SRD monster names are presumptively Open
Game Content. It is both the biggest and the cleanest remaining book, and it should be round 4's
target. `inner_sea_bestiary` is `campaign_setting/` and carries 7 PI rows, exactly as this book did.
The book round 2 ranked first (`inner_sea_world_guide`) is now finished as far as it can be
finished: 0 reachable units remain in it.

## Decision 51 — Companion Lane, extend: round 2 (2026-08-12, `sd29-companion-r5`, card `epic-7-companion-lane-extend`)

> **Header note on the number.** Drafted as §50 against a tree whose highest decision was §49, then
> **renumbered to §51 before landing**: the monster lane, running concurrently on the same branch,
> pushed its own round-3 record as §50 (`7bbce854`) while this round was in flight. The same
> collision produced the `§46`/`§47` and `§48`/`§49` pairs. Renumbering here rather than in the merge
> is the cheaper half of `§46.6` rule 1 — the collision was visible on `origin/tranche/9` before this
> section was appended, so it cost a `git log` instead of a conflict.

Round 2 took `§48.6`'s queue exactly as written — Bestiary 5, Bestiary 6 and Bestiary 2 — and
ingested **97 units end to end, all 97 grounded, none needing a new mechanism**. It also found that
`§48.6`'s ceiling was two different kinds of wrong, both in the same direction, and the finding is
worth more than the ingest.

### 51.1 The lane's REAL ceiling is 879, not 888 — and the missing 9 were being counted, not measured

`§48.1` established the discipline: classify the corpus rows before committing a round to a book, and
report the ceiling rather than the raw kind total. `§48.6` published **888 reachable / 855 honest
remainder**. Re-derived at this round's open, both halves reproduce and then move:

```
python3 scripts/classify_companion_rows.py
total companion units in scope              : 1696
orphan ability rows                         : 808
PRECAMPAIGN-gated on an uningested campaign : 2
reachable remainder                         : 886
```

**1,696 and 808 reproduce `§48.1` exactly** before being moved, so the two rounds' figures are
commensurable. The differences:

| adjustment | units | why it is not reachable |
|---|---|---|
| orphan ability rows | 808 | `§48.1` — no creature row claims them; a surface decision, not an ingest |
| `PRECAMPAIGN`-gated on a campaign this repo has not ingested | 2 | Bestiary 5's `Familiar (Brain Mole)` and `Familiar (Chuspiki)`; see `§51.2` |
| `*_classes_companion.lst` **class** rows | 7 | `core_rulebook` 2, `ultimate_magic` 3, `book_of_the_damned_volume_1` 2. `companion_chassis` models creature and ability rows only and says so in its module doc; `transcribe_companion_tables` raises `SystemExit` rather than emitting one. They were never in the reachable count and were never subtracted from it either |
| **REAL ceiling** | **879** | |
| grounded after this round | 135 | |
| **honest remainder** | **744** | |

So the dispatch brief's and `§48.6`'s **855 is corrected to 744** — 111 lower, of which 97 is this
round's ingest and 9 is measurement.

The shape of the 9-unit error is the reusable part. `classify_companion_rows.py` printed
`reachable remainder = total − orphans`, and *both* of the other two exclusions were already known to
the code: the chassis rejects class rows with a hard `SystemExit`, and the corpus-shape notes in
`loop-instruction.md` have documented `PRECAMPAIGN`-gated support files since Epic 2. **A ceiling
that subtracts one exclusion is not a ceiling; it is one exclusion.** The classifier now prints all
three lines and the reachable remainder subtracts all three.

**And this round made the same error once, against itself, which is the part worth keeping.** The
first correction emitted this cycle moved the ceiling 888 → **886**: it had found the two
`PRECAMPAIGN`-gated rows and stopped there, subtracting *two* exclusions instead of one and still not
three. The 7 class rows were found afterwards, while deriving round 3's queue, and a second
`correction` event carrying `--corrects <the first event id>` supersedes the first. The receipt,
`kanban.md` card 12 and every table in this section carry **879 / 744**; nothing downstream ever saw
886.

The failure mode is not arithmetic. It is that "subtract the exclusion you just discovered" feels
like completing the derivation, when the derivation is only complete once you have enumerated where
exclusions *come from* — here, three places that all already existed in the code and the docs. Both
`§45.1` and `§50`'s instrument fix are instances of the same thing at a larger scale, and this is the
smallest one on record: **a lane can reproduce the exact error it is in the middle of writing up.**

### 51.2 A classifier that could not read a row counted it anyway, and only a crash exposed it

`classify()` opened each unit's source file as `os.path.join(directory, unit["source_file"])` and, on
a miss, ran `if not os.path.exists(path): continue`.

`docs/work-inventory.json` records `source_file` as a **basename**, so a `.lst` PCGen loads out of a
subdirectory is not at `<book>/<basename>` at all. Bestiary 5 has two such units, in
`support/b5_races_companion_oa.lst`. The classifier counted them into `crea` (reporting 35, not 33),
never read their rows, and reported `ORPHAN 0` — a number that was correct, but not for a reason the
run had established.

**Nothing in the round's own procedure would have caught this.** What caught it was
`transcribe_companion_tables.py` raising `FileNotFoundError` on the identical path, because it had no
equivalent skip. A `near-miss` event records it. This is `§47.3`'s finding in a new place: a check
that silently measures less than it claims reads exactly like a check that passed.

Both scripts now share `resolve_source_file`, which resolves the basename anywhere under the book and
**raises** when it is nowhere or ambiguous. `classify`'s `named` count for Bestiary 5 is unchanged at
18 after the fix, which is the evidence that the two unread rows named nothing — a fact the round now
has rather than assumed.

### 51.3 Those two rows are excluded by a gate the corpus states, not by a list

`_bestiary_5.pcc:69` reads:

```
RACE:support/b5_races_companion_oa.lst|PRECAMPAIGN:1,Occult Adventures
```

Occult Adventures is not an ingested book. `decisions.md §47.2` already ruled exactly this for Horror
Adventures' `ha_abilities_race_oa.lst`, and `RuleSetId::Ha`'s doc comment records it; this is that
ruling applied to the same gate on a different kind.

Two things about **how** it is excluded matter more than the exclusion:

1. **The gate is read from the pcc, not hardcoded.** `precampaign_gates()` walks the book's `.pcc`
   files and reads the load line. `loop-instruction.md`'s corpus-shape notes are explicit that the
   gate lives on the pcc line and that `grep PRECAMPAIGN` over the `.lst` returns **0** — a lane that
   checks the file for its own gate concludes, wrongly, that it is ungated.
2. **"Gated" is not "out of scope."** Most `PRECAMPAIGN` gates in this corpus name books this repo
   HAS ingested (`INCLUDES=Bestiary 3`, `INCLUDES=Advanced Player's Guide`, `INCLUDES=Pathfinder
   Unchained`). Only `UNINGESTED_CAMPAIGN_GATES` — today one entry, `Occult Adventures` — excludes.
   A rule that dropped every gated file would have silently discarded reachable content.

The absence is pinned **by name**, not by a count: `rules_tables::bestiary_5`'s
`the_occult_adventures_gated_familiars_are_not_in_this_rule_set` asserts both keys are missing, so a
future transcriber change that started following `support/` unconditionally fails rather than quietly
adding two records. `reach_gate`'s per-record test states the same exception in its doc comment,
because `companions_reach`'s denominator is the corpus directory and would otherwise agree with
itself at 55.

### 51.4 What landed

| book | units | creatures | abilities | grounded | needed a `RuleSetId` |
|---|---|---|---|---|---|
| `bestiary_5` | 57 (55 in scope) | 33 | 22 | 55 | yes (`B5`) |
| `bestiary_6` | 26 | 14 | 12 | 26 | yes (`B6`) |
| `bestiary_2` | 16 | 15 | 1 | 16 | yes (`B2`) |
| **total** | **99 (97 in scope)** | **62** | **35** | **97** | |

* Three `RuleSetId` variants, three `COMPANION_BOOKS` rows, three `COMPANION_BOOK_SPECS` rows, three
  wire codes, three frontend labels, three `CORPUS_BOOK_IDS` rows, three `reach_gate` claims and
  three `corpus_ingest_diagnostic` rows — the per-book cost `§48.3` promised, paid three times with
  no mechanism change.
* `data/corpus/bestiary_{5,6,2}/companion/` — 55 + 26 + 16 = **97** records, plus a `LICENSE.json`
  each. **Zero PI redactions across all three**, which is what a rulebook (rather than a
  campaign-setting book) should produce: `§45.2` recorded 12 redactions on Inner Sea Races because
  Golarion nation and ethnicity names occur inside its mechanical prose, and a bestiary's companion
  stat blocks carry none.
* `corpus_ingest_diagnostic`'s three rows were written **in the same commit** that registers the
  books, because round 1 shipped its three books without them and
  `every_book_landed_in_rules_tables_is_reported` went red — the panel's caption says it shows every
  rule book landed in `rules_tables`, so a missing row reads to a tester as an un-ingested book.
* `v06_content_state_dump`'s exhaustive `RuleSetId` match got its three arms in the same commit, for
  the reason `§48`'s run 1 learned the expensive way: `cargo build --bin v06_work_inventory` does not
  reach that binary, and one broken bin is `0 passed across 0 suites` for the whole `root-full` stage.
* Corpus-wide `companion` grounded **38 → 135**; `not-started` **264 → 165**.

### 51.5 Bestiary 2 is the lane's first familiar book, and its rule set claims one family

Every companion book registered before this one contributes `*_races_companion.lst` animal-companion
rows. B2's 16 units are `b2_races_familiar.lst` + `b2_abilities_familiar_race.lst` — the same kind by
`file_kind`, the same two structural shapes, wizard/witch familiars rather than druid companions.
`every_creature_row_is_a_familiar` pins it, because a `*_races_companion.lst`-shaped reader would
have got this book quietly wrong.

B2 is also the first book in this lane that **another lane wants**: its 782 `monster` /
`monster_ability` units are the monster lane's round-3 third target (`§46`). `RuleSetId::B2`
compiles the `companion` family and nothing else, and its doc comment says so. The monster lane
adding its own tables to the same book id is the designed path, not a collision.

### 51.6 The scope flip's collateral, measured — and one unit it made honest

Registering the three rule sets moves their books `future_state` → `in_scope`, which moves every
other kind in them from `not-started` to `not-ingested` — the cost `§46.2` first measured and `§48.7`
paid for two books. Measured here, per book and per kind:

| book | other-kind units moved | breakdown |
|---|---|---|
| `bestiary_2` | 958 | `monster_ability` 466, `monster` 316, `race_trait` 162, `equipment` 8, `race` 6 |
| `bestiary_5` | 108 | `race_trait` 63, `monster_ability` 39, `race` 6 |
| `bestiary_6` | 33 | `class_feature` 18, `monster_ability` 13, `spell` 2 |
| **total** | **1,099** | |

None of it moves this lane's denominator (both statuses count as remaining); all of it moves other
lanes' `not-ingested` figures, which is why it is recorded.

**One unit did not move to `not-ingested` — it moved to `unknown`, and that is the flip working.**
`bestiary_6:class_feature:domain_power_serpent_companion` now reads
`class_feature_group_names_no_class_at_all`: its `Domain Power` group prefix names neither a class the
engine models nor one the corpus declares. That is an existing predicate reaching a record it could
not previously judge, with its reason stated, and it is left standing rather than suppressed.

### 51.7 The remainder, re-derived — and round 3 cannot repeat this round's shape

Re-derived at the end of the round with the same command that opened it. **Every remaining book
carries orphans**, which is the wall `§48.6` predicted:

| book | units | orphans | reachable | class rows |
|---|---|---|---|---|
| `ultimate_wilderness` | 575 | 249 (43%) | **326** | 0 |
| `core_essentials` | 145 | 51 (35%) | **94** | 0 |
| `core_rulebook` | 170 | 88 (52%) | **80** | 2 |
| `bestiary_4` | 80 | 5 (**6%**) | **75** | 0 |
| `bestiary_3` | 85 | 19 (22%) | **66** | 0 |
| `bestiary` | 59 | 5 (**8%**) | **54** | 0 |
| `ultimate_magic` | 170 | 138 (81%) | **29** | 3 |
| `advanced_race_guide` | 32 | 18 (56%) | **14** | 0 |
| `advanced_players_guide` | 212 | 208 (98%) | **4** | 0 |
| `book_of_the_damned_volume_1` | 31 | 27 (87%) | **2** | 2 |
| **total** | **1,559** | **808** | **744** | **7** |

**Round 3's first decision is a reach decision, not a book choice.** A whole-book `Reach::Surfaced`
claim is not available for a book with orphans; the claim must be scoped to the linked subset with an
`OPEN_FINDINGS` entry naming the rest, or the book waits on `§48.1`'s operator ruling. `§46`'s
round-3 note states the same rule for the monster lane, and the monster lane has already had to make
it once (Inner Sea World Guide, 5 template-namespaced orphans, `OPEN_FINDINGS`).

Ranked by orphan share rather than by size, `bestiary_4` (6%, 75 reachable) and `bestiary` (8%, 54
reachable) are the two cheapest — 129 units for two `OPEN_FINDINGS` entries of 5 records each. That
is the same disposition the monster lane took for ISWG, so the precedent exists inside this bundle.

**Two hazards for round 3, named so it does not pay to discover them.**

1. **`bestiary` is spelled `beastiary` on the engine side.** The inventory's book id is `bestiary`,
   `corpus_dir_for(RuleSetId::Bestiary1)` returns `"bestiary"`, and the rules-table module is
   `beastiary1`. `§44` records this exact spelling split silently under-reporting 108 Bestiary 1
   records once already. That book also already HAS a `RuleSetId`, so it is the first companion book
   that needs no scope flip — and the first whose registration touches a rule set another family
   already owns.
2. **`ultimate_wilderness`, `core_rulebook` and `ultimate_magic` carry `*_classes_companion.lst`
   rows** the chassis does not model. `transcribe_companion_tables` refuses the book outright with
   `"carries N *_classes_companion.lst rows; the chassis models creature and ability rows only. Widen
   it deliberately."` — a hard stop, not a silent drop, but a round that budgeted for an ordinary
   ingest will hit it.

### 51.8 The concurrent lane's `NAMEISPI:YES` finding was checked against this round's output, not assumed clear

`§50.1` (monster lane, round 3, landed on this branch mid-round) found that PCGen's own per-record
`NAMEISPI:YES` marker — a declaration that a record's NAME is Product Identity — is read by nothing
in this repository, and that two Inner Sea World Guide records carrying place names absent from
`PI_BLACKLIST_TERMS` **would have shipped**. It also found one already-shipped instance in the
race-trait lane's territory (`Elf ~ Sovyrian-Born`).

**That is a corpus-wide finding, so this round checked its own six source files rather than assuming
a bestiary is clean:**

```bash
grep -c 'NAMEISPI:YES' \
  bestiary_5/b5_races_companion.lst bestiary_5/b5_abilities_companion.lst \
  bestiary_6/b6_races_companion.lst bestiary_6/b6_abilities_companion.lst \
  bestiary_2/b2_races_familiar.lst  bestiary_2/b2_abilities_familiar_race.lst
#   -> 0 for all six
grep -rl 'NAMEISPI:YES' bestiary_5 bestiary_6 bestiary_2
#   -> bestiary_6/b6_deities.lst   (a deities file; carries no companion unit)
```

**Zero.** This round's 97 records are unaffected, and the three books' only `NAMEISPI:YES` rows are
in a file this lane does not read. Recorded because "our book was clean" is worth exactly as much as
the command behind it, and `§50.1`'s own lesson is that the marker was available in every `.lst` this
program has ever parsed and was never read.

## Decision 53 — Race-Trait Lane, extend: round 5 (2026-08-12, `sd29-racetrait-r6`, card `epic-6-race-trait-lane-extend`)

> **§53, after a live collision this round caught before merge rather than during it.** §51 was
> already held by the companion lane's round 2 when this round started, so this section was written as
> §52 — and the monster lane's round 4 claimed §52 concurrently, in a separate worktree, while this
> round was mid-flight. That makes **three** consecutive collisions in this bundle (`§47`, `§49`, and
> now this one), and it settles that "reserve the number at claim time" does not work when the claim
> is recorded only in the claimant's own worktree: `kanban.md` is the shared surface and none of the
> three claims reached it before the section was written. **The number a concurrent lane has not yet
> pushed is not reserved by anything.** This round moved rather than collide, on the safer-default
> rule, and every code comment it lands says 53.

`§49.8` closed round 4 by ruling the lane **dry**: 571-row ceiling, 514 grounded, 57 remaining, and
of those only 3 workable — Bestiary 1's Drow Noble, which needs a race *variant chassis* that is not
this card. **That ruling reproduces exactly and this round does not disturb it.** Re-derived at
round start, before any change, with the instrument round 4 checked in:

```bash
python3 scripts/race_trait_ceiling.py
```

→ `TYPE:<18 races> Racial Trait rows : 553` + `Subrace rows : 18` = **571**;
`by status : {'grounded': 514, 'not-ingested': 57}`; remaining by book
`advanced_players_guide 49 / bestiary 3 / core_essentials 2 / horror_adventures 1 /
inner_sea_races 1 / monster_codex 1`. Identical to `§49.8`'s table, cell for cell.

So this round did not ingest. **It fixed a defect in what the lane had already shipped**, and the
defect is the one the monster lane found in its own kind, reported as corpus-wide, and handed to this
lane by name (`kanban.md` card 8: *"a `PI_BLACKLIST_TERMS` addition is corpus-wide and sits in the
race-trait lane's territory"*).

### 53.1 This program's Product-Identity screen is a heuristic, and the corpus has been declaring the answer all along

`pi_screening::PI_BLACKLIST_TERMS` is 55 names — 20 Golarion deities, 34 place/nation names, and one
NPC an ACG retrofit found by hand. Its own module doc calls it *"a bounded, documented heuristic …
not an exhaustive legal review."* Every Pipeline A ingest path screens against it and nothing else.

PCGen states the same fact **per record**, in two tokens this program has parsed into `raw_tokens`
and never read:

* `NAMEISPI:YES` — this record's **name** is Product Identity;
* `DESCISPI:YES` — this record's **description** is Product Identity.

Derived over the shipped tree at round start, which is the whole finding in one command:

```bash
python3 -c "
import json,glob,collections
c=collections.Counter()
for p in glob.glob('data/corpus/*/race_trait/*/*.json'):
    d=json.load(open(p)); ks={t['key'].upper() for t in (d['data'].get('raw_tokens') or [])}
    for k in ('NAMEISPI','DESCISPI'):
        if k in ks: c[(k, d.get('pi_marker'))]+=1
print(dict(c))"
```

→ `{('DESCISPI', 'redacted'): 18, ('DESCISPI', None): 8, ('NAMEISPI', None): 1}`

**26 shipped `race_trait` records declare `DESCISPI:YES`. The blacklist redacted 18 of them by
coincidence** — their prose happens to contain a Golarion place name the list knows — **and published
the other 8.** Their Product Identity is `Kodar Mountains`, `Earthfall`, `Ekujae`, `Gogpodda`,
`Omesta`, `Droskar`, `Abaddon` and `Inner Sea`: eight names, none on a 55-term list assembled by
sampling. The list was never going to have them, and the row said so.

A heuristic that agrees with a declaration 69% of the time is not a screen; it is a coincidence with
a good track record. The two are now a **union** — the declaration redacts unconditionally, and an
undeclared row is still term-scanned, because `ogl-pi-blacklist.md` §2 is equally explicit that the
corpus's markers are incomplete.

### 53.2 A name cannot be redacted, so the row is dropped

One record declares `NAMEISPI:YES`: `Elf ~ Sovyrian-Born`
(`isr_abilities_race.lst:67`, shipped since round 2 on 2026-08-11).

A description can be replaced with `[redacted PI]` and the record still works — its key, flags,
bonuses and page cite are untouched. **A name cannot.** It is what the picker's checkbox says, what
the Race Traits panel prints, and half of the record's key. The only way not to publish it is not to
publish the row.

This is the identical ruling `§50` reached independently, from the other end of the corpus, for Inner
Sea World Guide's five `NAMEISPI:YES` monster rows — reached there by a lane that had no reason to
consult this one. Two lanes converging on the same rule from different kinds is the strongest
evidence available that it is the rule, and it is now written once, in `pi_screening`, rather than
twice in two hand-built tables.

Reclassifying a declared-PI row as shippable is `ogl-pi-blacklist.md` §3's per-book override — an
operator decision. Under unattended mode the safer default was taken and recorded: **drop**.

### 53.3 What that costs, stated exactly

`Elf ~ Sovyrian-Born` was a live, selectable alternate. Dropping it moved **nine** pinned counts
across six files, every one of them re-derived rather than decremented:

| pin | was | now |
|---|---|---|
| `race_resolver::ALTERNATE_TRAIT_REPLACE_FLAGS` rows / `TraitRole::Alternate` / `selectable_alternate_trait_keys()` | 283 | **282** |
| whole race corpus, all roles | 516 | **515** |
| `race_trait_picker` menu total / `race_catalog` alternates / `checked` | 283 | **282** |
| picker per-race, Elf | 28 | **27** |
| picker menu rows + standard rows | 456 | **455** |
| `character_hub` alternates creation accepts, 7 CRB races | 189 | **188** |
| `reach_gate` ISR ingested records / reached | 72 / 71 | **71 / 70** |
| `ingest_race_traits` per-book record count, `inner_sea_races` | 72 | **71** |
| `work-inventory` `race_trait` grounded | 514 | **513** |

The two distinct replace-flags the row fired (`Elf_ReplaceElvenMagic`, `Elf_ReplaceKeenSenses`) are
both still claimed by other ISR and ARG alternates, so **no flag became an orphan** and the
orphan-flag assertion did not move — checked, not assumed.

`race_trait` grounded going **down** is the correct direction here and is the second time in two
rounds that this lane's own count moved against the intuitive sign. `§49.3` caught a defect because a
count moved the wrong way; this round *expects* the drop and would have found a defect if it had not
appeared. **A denominator taken twice is this program's cheapest instrument either way.**

### 53.4 The ceiling is unchanged; the remainder gains a class

`scripts/race_trait_ceiling.py` still reports **571**, because the row is still in the corpus — it is
the *shipped* record that is gone. Re-derived after the change:

```
units matched into the ceiling : 571
by status                      : {'grounded': 513, 'not-ingested': 58}
```

The 58 is `§49.8`'s 57 plus `Elf ~ Sovyrian-Born`, and its class is new to this lane:

| book | units | class |
|---|---|---|
| `advanced_players_guide` | 49 | not gap — same `KEY:` as already-ingested ARG records (`§39`) |
| `bestiary` (Drow Noble) | 3 | **workable, needs a race-variant chassis — not this card** |
| `core_essentials` | 2 | not gap — the no-heritage baseline (`§49.2`) |
| `horror_adventures` | 1 | not gap — `PRECAMPAIGN`-gated on Occult Adventures (`§47.2`) |
| `inner_sea_races` | 1 | not gap — `Human ~ Tribalistic Languages`, upstream data gap (`§45.4`) |
| `inner_sea_races` | 1 | **not gap — `Elf ~ Sovyrian-Born`, declared Product Identity (this section)** |
| `monster_codex` | 1 | not gap — ability-pool variant mechanism (`§43`) |
| | **58** | **3 workable / 55 not gap** |

**The genuinely-workable remainder of this card is still 3, and still not race-trait work.** The
chassis-blocked residue is unchanged at **3,447 − 571 = 2,876**.

### 53.5 The finding this round could not close, and will not pretend it did

**`§8b`'s browse-screen render bug is still open and this lane still owns it.** Rounds 2, 3, 4 and
now 5 have not fixed it. Round 5 did read the code, and hands the next round a narrower starting
point than "the panel is stale", because the round-1 receipt's stated evidence does not survive
inspection:

> *"the right-hand column does update ('1 selected. 0 further options locked out.'), so the IPC round
> trip happened"*

That inference is **unsound**. `AlternateTraitPicker.tsx` renders that sentence from
`selected.length` (local React state, updated synchronously by the checkbox) and `blocked.size`,
which is **0 when `selection` is `null`**. So the observed right-hand text is exactly what renders
when the resolve call has *not* answered. The left panel being stale and the right panel reading
"1 selected. 0 locked out." are one symptom, not two, and "the IPC round trip happened" was never
established.

Two candidate causes remained: (a) the screenshot was captured inside the window between the click's
commit and the effect's `setSelection(null)`, in which case there is no product defect and the
*harness* needs a settle-wait; (b) the resolve genuinely returns no suppressions for this selection,
in which case the defect is in the backend and not in a render path at all.

**(b) is dead, and this round killed it with a test rather than an argument.**
`race_trait_picker::plagueborn_really_suppresses_both_standard_traits_its_flags_name_so_8b_is_not_a_backend_gap`
resolves the exact selection `§8b` screenshotted, at the DTO layer the screen reads:

```
before: 9 applied, 0 suppressions          # matches the screenshot's "9 traits apply"
after:  8 applied, suppressions = [Half-Orc ~ Intimidating, Half-Orc ~ Weapon Familiarity],
        blocked_alternates NON-EMPTY
```

The caption should read **8**, and the right panel's lock-out count should **not** be 0 — which is the
other half of the same evidence. A rendered *"1 selected. 0 further options locked out."* beside a
real selection is a `selection == null` render, not a resolved one. So `§8b`'s two symptoms are one
symptom, the backend is not implicated, and what survives is the timing reading.

**The label was wrong for three rounds and the cost was three rounds of deferral.** The general
lesson is `§45.1`'s in a new register: a receipt's *diagnosis* is inherited as readily as its
figures, and this program re-derives figures at the point of use while taking attributions on trust.
Round 6 should reproduce the timing reading live — click an alternate, wait, screenshot again — and
close or reopen `§8b` on that, not on this section.

### 53.6 What landed

* `pi_screening::{DeclaredProductIdentity, declared_product_identity, classify_optional_field_declared}`
  — the shared reader, with 7 unit tests including the explicit-`NO` case and the case-insensitive
  trimmed spelling. Placed in the shared module rather than in this lane's ingest binary, because the
  finding is corpus-wide and the monster lane's hand-built copy is the precedent for what happens
  otherwise.
* `ingest_race_traits`: `NAMEISPI:YES` rows dropped **before** the scope filter and reported by
  file:line in the run receipt (`dropped, NAMEISPI:YES : 1`); `DESCISPI:YES` descriptions redacted
  through the shared reader and counted (`descriptions redacted by DESCISPI:YES : 16` for ISR, `9`
  for Core Essentials). A row that vanishes without a line in the receipt is indistinguishable from
  an ingest bug.
* `tests/sd29_declared_product_identity_in_shipped_race_traits.rs` — the corpus-level gate, reading
  the **shipped files** rather than the source rows, so both ends read the same bytes. It went RED
  first and named all 9 offenders by key.
* `data/corpus/inner_sea_races/` and `data/corpus/core_essentials/` regenerated: 8 descriptions newly
  redacted, 1 record deleted. **Only these two books were regenerated**, because they are the only
  two whose sources carry either token — derived, not assumed:
  `grep -rl 'DESCISPI:YES' core_essentials/races/` → `tiefling_abilities_race_subrace.lst` (ingested)
  and `skinwalker_abilities_race_subrace.lst` (a race this product does not model);
  `grep -c NAMEISPI:YES` over ARG's, Monster Codex's and Horror Adventures' racial `.lst` files → 0
  for all three. `core_rulebook`'s 67 and `beastiary`'s 108 shipped records carry no `ISPI` token at
  all (scanned: 175 records, 0 hits), and their sources carry none either.
* The nine count re-pins of `§53.3`, each carrying the reason in its own assertion message.

### 53.7 One scope finding for a successor — this is not a race_trait-only defect

The reader is shared, but **only `ingest_race_traits` calls it.** Every other Pipeline A writer
(`gen_book_cache`, `ingest_races`, `ingest_pu_classes`, `gen_core_rulebook_cache`, `cache_gen::*`)
still screens on the term list alone, and `tests/sd29_declared_product_identity_in_shipped_race_traits.rs`
only walks `*/race_trait/`. The same command that found this, pointed at the whole corpus rather than
one kind, is the successor's first move — this round did not run it corpus-wide because widening the
gate to a kind whose ingest path cannot yet satisfy it would land a red gate on another lane's work,
which is `loop-instruction.md`'s "STOP — do not clobber another session's live work" rather than a
courtesy.

## Decision 52 — Monster / Monster-Ability Lane, extend: round 4 (2026-08-12, `sd29-monster-r6`, card `epic-5-monster-lane-extend`)

Round 4 took `bestiary_2`, the book `§50.7`'s corrected classifier ranked first, and ingested **715
units — 314 monsters and 401 monster abilities**. The five books already in
`monster_chassis::MONSTER_BOOKS` hold **34** monsters between them; adding Bestiary 1's 46 SD-22
records — served by the same catalog, though not through this chassis — the entire prior population
was 80. This round took the lane's grounded `monster` count from 80 to **394** and
`monster_ability` from 87 to **488**.

The round found **two** things the lane's instruments were getting wrong, both in the direction that
over-reports available work, and both of the same shape as `§50.7`: a screen that classifies rows is
itself a proxy, and it must be validated where it makes its confident claim.

### 52.1 A `.COPY=` row is a delta, not a stat block — and the generator said so before anyone asked

`gen_book_cache` refused this book's first transcription outright:

```
thread 'main' panicked at src/bin/gen_book_cache.rs:1501:5:
assertion `left == right` failed: pathfinder/paizo/roleplaying_game/bestiary_2/b2_races.lst:454
names "Gug.COPY=Gug Savant", not "Gug Savant" -- the table's recorded line is stale and must be
re-transcribed, not papered over here
```

`verified_citation_line` compares the cited row's first column to the record's name. A
`<Base>.COPY=<Variant>` row fails that comparison **because it is not a record definition**: PCGen
copies the base record whole and then applies the few tokens the copy row carries. The gate was
right, and the message even names the temptation it exists to refuse.

Transcribed verbatim — which is the only thing `scripts/transcribe_monster_tables.py` does — the two
rows produce a card carrying a challenge rating and nothing else: no size, no speed, no type, no
page, no attacks. That is a card a player opens to find blank, the stub class
`docs/governance/no-stub-mvp-doctrine.md` forbids.

**Resolving the delta is not transcription.** It composes values across two rows while
`MonsterStatBlock` carries ONE `source_file`/`source_line` pair, so every inherited field would ship
under a citation that does not contain it — exactly the stale-citation defect `verified_citation_line`
and `v06_corpus_trap_report --audit` exist to catch. A chassis that models inheritance needs a second
citation, deliberately widened. **Two records is not a reason to slip a chassis change into an ingest
round**, so the two rows are DROPPED, cited by line in the generated header, and pinned by a test.

**The population is exactly two, corpus-wide** — derived, not assumed:

```bash
python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
print(sum(1 for u in d['units'] if u['kind'] in ('monster','monster_ability')
and u.get('origin')=='copy'))"
#   -> 2
```

Both are in `bestiary_2` (`b2_races.lst:454`, `:594`). The book's races file carries **eight**
`.COPY=` rows (`grep -n '\.COPY=' b2_races.lst`); the other six never became inventory units at
all — the inventory's own trap filters drop them before this lane ever sees them, which is why the
class stayed latent through four rounds. It is now measured rather than latent.

`scripts/classify_monster_ability_rows.py` gained `is_copy_row` and a `COPY` column for the same
reason it gained the Product Identity read in `§50.7`: an unshippable monster cannot own anything,
and counting one as reachable over-reports the lane.

### 52.2 A case-insensitive PI screen over rules prose is a defect in the over-exclusion direction, and this book measured it at 13

`§50.4` recorded that an over-broad Product Identity screen is a real cost — a first draft there
dropped a monster for `AUTO:LANG|Abyssal|Varisian` matching `Varisia` as a substring. This round hit
the same class in a new place and the number is bigger.

A first draft of `no_shipped_monster_field_carries_a_product_identity_term` screened key, name AND
description **case-insensitively**. It reported **13** hits, every one a false positive on an
ordinary English word:

| term | what it actually matched | records |
|---|---|---|
| `Nex` | "the **nex**t round" | 12 |
| `Torag` | "s**torag**e" (`Mercane ~ Secret Chest`) | 1 |

`gen_book_cache::monster_record_pi_hits` — the authoritative screen, and a hard stop — uses
`serialized.contains(term)`, case-**sensitive**, and had already passed this book with zero hits. The
test was not finding something the generator missed; it was applying a looser predicate than the one
that governs.

**The rule this settles.** Identity fields (`key`, `name`) are screened case-insensitively: a deity
name reaching a key in any casing is a real hit, and this book has none in any casing. Rules TEXT is
screened with `pi_screening`'s own case-sensitive predicate, because prose is where a loosened screen
starts eating Open Game Content. Thirteen dropped monster abilities would have been thirteen records
withheld from players for the word "next".

### 52.3 Zero Product Identity in a `roleplaying_game/` bestiary is a prediction that held

`grep -c 'NAMEISPI:YES' b2_races.lst b2_abilities_race.lst` → `0` and `0`. Round 3 lost 5 of Inner
Sea World Guide's 14 monsters to that marker; this book loses none.

`ogl-pi-blacklist.md` §2 predicts the split and it held: Product Identity in a Pathfinder book is its
Golarion proper nouns, which live in the `campaign_setting/` line, while a `roleplaying_game/`
bestiary's monster names are presumptively Open Game Content. The absence is held by a test against
the live blacklist, not by the grep — the grep is a statement about today.

### 52.4 The `OPEN_FINDINGS` instruction is still not satisfiable, and round 3's divergence is followed

`§50.6` ruled that `reach_gate`'s findings test fails a recorded finding whose family DOES reach a
surface, so a book that ships a subset of its rows cannot carry an `OPEN_FINDINGS` entry for the
remainder. `bestiary_2/monsters` and `bestiary_2/monster_abilities` both reach
`list_monster_catalog` for every shipped record, so the same reasoning applies unchanged. The
exclusions are held by five named tests in `rules_tables::bestiary_2`
(`the_book_ships_every_stat_block_and_only_the_owned_abilities`,
`no_shipped_monster_ability_is_an_orphan`,
`every_owner_named_by_a_shipped_monster_ability_is_a_shipped_monster`,
`no_shipped_monster_field_carries_a_product_identity_term`,
`the_orphan_ability_rows_are_not_records`) and by the generated header, per row, with reasons.

### 52.5 A comment recording a FALSE positive instantiates the name as surely as one recording a removal

The first full gate came back `VERIFY_EXIT=1` with `pi-sweep`, `root-full` and `desktop` red, and
the first two were the same cause: **this round's own doc comments spelled three Product Identity
terms while explaining that the book carries none.** One named the setting whose proper nouns the
`roleplaying_game/` vs `campaign_setting/` split is about; two quoted the false positives §52.2
measured, so that a reader could see why they were false.

```
pi-sweep: UNBASELINED src/rules_core/rules_tables/bestiary_2/mod.rs:24  [<term>]
pi-sweep: UNBASELINED src/rules_core/rules_tables/bestiary_2/mod.rs:302 [<term>]
pi-sweep: UNBASELINED src/rules_core/rules_tables/bestiary_2/mod.rs:302 [<term>]
pi-sweep: FAIL — 3 unbaselined hit(s), 0 stale row(s).
```

`decisions.md §50` already wrote this rule, for a comment recording a **removal**: *"a comment
recording a removal has no need to instantiate the name it removed."* The generalisation this round
adds is that **the rule does not depend on why the name is there.** Neither `pi-sweep` nor
`pi_table_sweep` reads intent, and both are right not to: "this term is a false positive" is a claim
about a term, and the term is on the page either way. The comments now say why the terms are not
named in place and point at this section, which is outside the swept tree.

**It escaped to the shared branch, and that is the part worth recording.** `69e0dec8` was pushed to
`origin/tranche/9` before the gate came back — which is what this bundle's own instructions demand,
after two cycles died with their only copy on a worktree branch — and a **concurrent lane's** gate
went red on the merge. The fix landed 22 minutes later in `4524efa2` and was pushed immediately.
Push-early and gate-after are both correct rules and they interact: the cost of the first is
occasionally handing a sibling a red stage, which is cheaper than losing the work but is not free,
and the mitigation is to push the fix the moment it exists rather than batching it into a receipt
commit.

### 52.6 A test proxy that held for five books failed on the sixth's correct output

`desktop` was red on `bonus_bestiary_ability_keys_carry_the_namespace`, which asserted that no
ability key is served twice **anywhere** in the catalog response. It read 522 against 488.

Nothing was wrong. The catalog renders an ability underneath *each* monster that claims it — that is
the shape of the whole surface — so an ability with several owners is served once per owner **by
design**. The assertion had passed for five books because in every one of them each ability had
exactly one owner, which was a property of those books, not of the catalog. Bestiary 2 is the first
with any:

```
19 ability records carry more than one owner
34 extra served rows        (522 - 488 = 34, exactly)
```

The assertion was a **proxy** for the thing the test is named after: that a served key is the corpus
`KEY:` and not a display name. It now asserts that directly, in two parts that are both true of the
real catalog — no monster lists the same ability twice, and the number of DISTINCT served keys equals
the number of ability records the chassis registry holds. A key collapsed to a display name, or a
record served under two keys, still fails.

**This is the same finding as §52.1 and §50.7 in a third costume.** Each of them is an instrument
whose confident claim was validated only where it happened to be right: the classifier against books
with no Product Identity, the classifier again against books with no `.COPY=` rows, and this test
against books with no shared abilities. The lane's instruments keep being correct-by-coincidence
until a book arrives that distinguishes the proxy from the property.

### 52.7 A record deleted for Product Identity moved four numbers, and the deleting lane moved one

Round 4's gate ran three times. Runs 1 and 2 were red for this round's own reasons (`§52.5`,
`§52.6`). **Run 2 was also red for reasons belonging to a concurrent lane**, and that is the finding.

The race-trait lane's round 5 dropped one record — `Elf ~ Sovyrian-Born`, the `NAMEISPI:YES` row
`§50.2` reported and did not fix — and pushed it to `tranche/9`. Deleting it moved **four** numbers.
The lane moved one, the per-book map's `("inner_sea_races", 71)`. The other three were left red on
the shared branch, where every cycle running against it inherited them:

| artifact | stated | on disk |
|---|---|---|
| `ingest_race_traits.rs:1741` heritage total | 340 | **339** |
| `inner_sea_races/LICENSE.json` `records_processed` | 72 | **71** |
| `inner_sea_races/LICENSE.json` `records_redacted` | 12 | **18** |
| `core_essentials/LICENSE.json` `records_redacted` | 8 | **9** |

Two of the four were not even caused by the deletion — the redaction counts had drifted earlier and
only surfaced once a neighbouring assertion started failing, which is what
`ingest_race_traits`'s own message predicts in as many words: *"fixing one assertion reveals the next
one below it, which is the whole reason the test states both."*

**Fixed here rather than waited on**, and every figure restated to match the corpus rather than
worked around — which is what `sd27_book_license_record_counts`'s failure message instructs, because
a `LICENSE.json` is an OGL redistribution record and not a test fixture. The screening notes that
quote each figure, including the two that quote their own derivation command inline, are restated
with it.

**The general form, and it is a coordination rule rather than a code one.** A record that leaves the
corpus is not one edit. It is an edit plus every count pinned against it, and this program pins
counts deliberately and in several places precisely so a silent drift cannot happen. The lane that
removes a record owns all of them, and "my own gate was green" is not evidence that it did — the
race-trait lane's own run hit the first of the three at the same moment this one did. The cheap check
is to re-run the full gate after the deletion rather than after the fix that motivated it.

### 52.8 What landed, and the remainder

| book | monster units | ingested | monster_ability units | ingested |
|---|---|---|---|---|
| `bestiary_2` | 316 | **314** | 466 | **401** |

Denominators, by the command rounds 1-3 all recorded (sum `not-ingested` + `not-started` for both
kinds over every non-`out_of_scope` book in `docs/work-inventory.json`):

* **Before:** `monster` 1,190 + `monster_ability` 3,020 = **4,210**. Round 3's closing figure,
  reproduced exactly before being moved.
* **After:** `monster` 876 + `monster_ability` 2,619 = **3,495**. `units_ingested` = **715**.
* Grounded: `monster` 80 → **394**, `monster_ability` 87 → **488**.

The lane's REAL ceiling, by the corrected classifier:

```
remaining monster+monster_ability units     : 3495
orphan monster_ability rows                 : 1406
  of which in ZERO-monster books            : 703 across 10 books
Product Identity rows (never shippable)      : 32
`.COPY=` delta rows (no stat block of their own): 2
reachable remainder (units - orphans - PI - COPY): 2055
```

**2,055, down from `§50.7`'s 2,773.** The arithmetic closes exactly and is worth stating, because a
ceiling that moves for two reasons at once is where a lane loses track of itself:
`2773 − 715 (ingested) − 2 (COPY) − 1 (the ability whose only owner was a COPY row) = 2055`.

**Round 5's queue, from the corrected classifier in one command
(`python3 scripts/classify_monster_ability_rows.py`):**

| book | remaining units | orphans | PI | COPY | **reachable** |
|---|---|---|---|---|---|
| `bestiary_4` | 988 | 225 | 14 | 0 | **749** |
| `bestiary` | 807 | 146 | 0 | 0 | **661** |
| `bestiary_3` | 301 | 13 | 0 | 0 | **288** |
| `inner_sea_bestiary` | 230 | 26 | 7 | 0 | **197** |
| `inner_sea_gods` | 200 | 81 | 3 | 0 | **116** |

`bestiary_4` is the largest and carries the 14 PI rows `§50.7` predicted; `bestiary` (Bestiary 1) is
the second and is the only remaining book where the chassis meets an EXISTING ingest — its 46 SD-22
monsters are already grounded through `beastiary1`'s own tables and a round taking it must decide
whether the chassis absorbs them or sits alongside. **`bestiary_3` is the cleanest per unit of
work** (13 orphans against 301 units) and is the correct target for a round that wants a clean run.

Ten books hold 703 orphan abilities and **zero** monsters. No per-monster cycle can ground them, and
`loop-instruction.md`'s "Hard stops" names running one against a zero-monster book as a reportable
hard stop rather than something to force. That is the floor this lane cannot go below.

## Decision 54 — Companion Lane, extend: round 3 (2026-08-12, `sd29-companion-r7`, card `epic-7-companion-lane-extend`)

`§51.7` predicted this round's shape and got it exactly backwards, in the direction nobody budgets
for: it said **every remaining book carries orphans**, so round 3's first move would be a *reach*
decision — a scoped claim plus an `OPEN_FINDINGS` entry naming the rest. Round 3 opened by trying to
write that mechanism, looked at the five rows that made Bestiary 1 orphan-bearing, and found that
**none of them is an orphan**. The book was registerable all along under an ownership shape the
classifier did not read, and it landed with **59 units, all 59 grounded, zero `OPEN_FINDINGS`
shortfall and no orphan-drop mechanism written at all**.

The lane's ceiling moved **up** for the first time.

### 54.1 The granted-by ownership shape: an ability row can be what names another ability row

`classify_companion_rows.py` read three ownership shapes, and applied shape 1 — a
`ABILITY:Special Ability|AUTOMATIC|<name>` token — **only to creature rows**. Bestiary 1 is where
that restriction is wrong. `b1_abilities_companion.lst:7`:

```
Companion Advancement (Dinosaur (Ankylosaurus))  KEY:Companion Advancement ~ Dinosaur (Ankylosaurus)
  CATEGORY:Special Ability  TYPE:CompanionAdvancement  VISIBLE:DISPLAY
  PRERACE:1,Companion (Dinosaur (Ankylosaurus))
  ABILITY:Special Ability|AUTOMATIC|Ankylosaurus ~ Stun  BONUS:SIZEMOD|NUMBER|1
```

That row is an **ability** row. It is owned by `Companion (Dinosaur (Ankylosaurus))` through shape 2
(its own `PRERACE:`), and it — not the creature row — is what names `Ankylosaurus ~ Stun`. Under
shapes 1-3 the Stun row is claimed by nobody and the book reports 5 orphans; the corpus states its
owner one hop away.

**Shape 4, granted-by:** shape 1's own token, read on an ability row that shapes 1-3 have already
given an owner, propagating that row's owners to what it names. Run to a fixpoint, seeded only from
already-owned rows, so an orphan can never grant reachability to an orphan. It is not a looser rule —
it is the *same* token, read on a row the previous predicate skipped.

Corpus-wide, re-derived with `python3 scripts/classify_companion_rows.py`:

| | before shape 4 | after |
|---|---|---|
| orphan ability rows | 808 | **794** |
| `bestiary` | 5 | **0** |
| `core_rulebook` | 88 | **84** |
| `ultimate_magic` | 138 | **135** |
| `ultimate_wilderness` | 249 | **247** |

**14 units, and the ceiling moves the other way for the first time in this lane.** `§48.1` moved it
1,696 → 888, `§51.1` moved it 888 → 879; both were the instrument discovering it had been
*over*-claiming. This is the first correction where the instrument had been *under*-claiming, and the
asymmetry is the point: every prior tightening was found by asking "is this really reachable?", and
nothing in that habit ever asks the opposite question. The five rows sat in an `ORPHAN` column for
two rounds, in a book `§51.7` ranked as one of the two cheapest, and the column was believed because
under-claiming is the safe direction — which is true of what you ship and false of what you plan.

The five are pinned BY NAME, not by a count, in
`rules_tables::beastiary1::the_five_granted_by_advancement_abilities_resolve_to_their_creature`: a
regression that dropped shape 4 leaves those five with no owner and that test says which.

### 54.2 The ceiling instrument still did not subtract the exclusion `§51.1` said it subtracts

`§51.1` closes: *"The classifier now prints all three lines and the reachable remainder subtracts all
three."* Re-derived at this round's open, before any change:

```
total companion units in scope : 1696
orphan ability rows            : 808
PRECAMPAIGN-gated on an uningested campaign : 2
reachable remainder            : 886
```

886 = 1696 − 808 − 2. The 7 `*_classes_companion.lst` class rows are **not** subtracted. Every doc
downstream of `§51` — that section's own table, `kanban.md` card 12, the dispatch brief — carries the
hand-corrected **879**, so nothing shipped wrong; but the instrument and the prose disagreed by 7 for
a full round, and the prose is what a reader checks the instrument against. `§51.1`'s own words for
this are exactly right and were applied to everyone except itself: *"a ceiling that subtracts one
exclusion is not a ceiling; it is one exclusion."* The class-row line is now printed and subtracted
in the script.

**The lane's REAL ceiling, both corrections applied:**

| adjustment | units |
|---|---|
| total `companion` units in scope | 1,696 |
| − orphan ability rows (shape 4 applied) | 794 |
| − `PRECAMPAIGN`-gated on an uningested campaign | 2 |
| − `*_classes_companion.lst` class rows the chassis refuses | 7 |
| **REAL ceiling** | **893** |
| grounded after this round | 194 |
| **honest remainder** | **699** |

`879 + 14 = 893`, and `744 − 59 (this round) + 14 (measurement) = 699`. Both close exactly.

### 54.3 Bestiary 1 is spelled three ways, and the third one is where the grounding was lost

The book id hazard `§51.7` named is worse than two spellings:

| string | who uses it |
|---|---|
| `bestiary` | the PCGen source directory, and `docs/work-inventory.json`'s `book` field |
| `beastiary` | the `data/corpus/` directory (misspelled since SD-22) |
| `beastiary1` | the Rust module, and the ingest diagnostic's `book_id` |
| `bestiary_1` | `rule_set_id(RuleSetId::Bestiary1)` — the **engine book**, a fourth string |

`COMPANION_BOOKS.corpus_book` must be `beastiary`, because every consumer of that field reads a
`data/corpus/` directory: the generator's output root and `reach_gate::companions_reach`'s
denominator. Registering it as `bestiary` writes `data/corpus/bestiary/`, a second corpus directory
for a book that already has one, with its own `LICENSE.json` and a companion half that could never be
judged against the monster half.

But `v06_work_inventory`'s `Kind::Companion` verdict arm looks up `chassis_companion_keys` by the
**engine book** (`bestiary_1`), and the map was keyed by `book.corpus_book`. For the first seven
registered companion books those two strings are identical, so the key worked **by coincidence
rather than by rule**, and nothing said so. Untranslated, all 59 of this book's grounded records
would have reported `companion_content_has_no_engine_table` — a book fully ingested, fully reaching
the catalog, reported as having no engine table at all. That is `§44`'s spelling defect again, in a
new place, and the same size (§44: 108 records; here: 59).

The fix is a translation the code performs, not a spelling anyone is asked to remember:
`engine_book_for_corpus_dir` already existed, `CORPUS_DIR_ALIASES` already carried
`("beastiary", "bestiary")`, and the registry loop now uses them — and **panics** on a registered book
that resolves to no rule set, so the next such book fails the build instead of silently grounding
nothing.

### 54.4 The generator preserved the licence declaration and destroyed the licence *history*

`gen_companion_book` read a prior `LICENSE.json`, kept its `license_declaration` — and overwrote its
`screening_method_note` with a sentence about the current run.

`data/corpus/beastiary/LICENSE.json`'s note stated **three earlier PI-screening passes** by cycle,
date and record count: E2.0.9's 45 records (2026-07-27), `ingest_races`' 119 (2026-07-31), SD28-E16's
5 (2026-08-07), each with what was screened and why. After a companion run it read: *"Every field of
the 59 records this run wrote … records_processed is 228."* A file whose record count says 228 and
whose method note accounts for 59.

**It had already happened twice, unnoticed, in this lane's own round 1**: `monster_codex` (whose note
had covered its race-trait and monster passes) and `horror_adventures`, both clobbered by
`bac2f569`. Neither round noticed, because the field a generator overwrites looks exactly like the
field a generator wrote.

Fixed at the source — the note is now **append-only**, composed onto whatever was there, and
idempotent (a re-run of the same cycle replaces its own trailing pass rather than stacking copies,
verified by running the generator twice and comparing note length: 2,837 characters both times). The
two clobbered books' notes are restored from the commit that preceded the overwrite by
`scripts/tests/restore_clobbered_license_notes.py`, which reads the original bytes out of git rather
than retyping them, and the generator then appends this lane's pass to the restored text.

The wider point is `§47.3`'s again: **the preservation rule was already written and already
implemented — for the other half of the file.** `gen_companion_book`'s comment says in as many words
that clobbering a prior lane's sharper OGL citation "would replace a real derivation with a weaker
one." That reasoning is exactly as true of the screening note, and the code applied it to one field.

### 54.5 A hand-maintained frontend list had been stale for a round, and nothing could tell

`CompanionCatalogScreen.test.ts`'s `SERVED_BOOK_CODES` — the list the "every served book has a real
name" test iterates — was still round 1's four (`ISC`, `MC`, `ISI`, `HA`) after round 2 registered
three more. B5's, B6's and B2's labels were therefore checked by nothing: the test passed, over a
denominator that had stopped being the truth. Corrected to the eight the backend serves. The test
asserts about a hand-written list rather than about the wire, so it will go stale again; that is
recorded here rather than solved, because deriving it needs a fixture the frontend does not have.

### 54.6 The same stale-hand-list defect was RED on the branch, in the sibling file, one commit away

`§54.5` found `CompanionCatalogScreen.test.ts`'s `SERVED_BOOK_CODES` stale and *silent* — the test
passed over a denominator that had stopped being the truth. The first full gate on the merged tree
found the identical defect in `MonsterCatalogScreen.test.ts`, and there it was **loud**:

```
FAIL src/monsterCatalog/MonsterCatalogScreen.test.ts
Error: BOOK_LABELS names exactly the served books:
  expected B1,B2,BB,BOTD1,BOTD2,ISWG,MC, got B1,B2,B3,BB,BOTD1,BOTD2,ISWG,MC
```

**Proven inherited rather than assumed**, which is the part the doctrine asks for
(`loop-instruction.md`, "A gate stage that fails twice … it blocks the cycle until the attribution is
*proven*"). Read out of the monster lane's own commit rather than out of the merged tree:

```bash
git show 9595bd82:apps/desktop/src/monsterCatalog/MonsterCatalogScreen.test.ts | grep 'const SERVED_BOOKS'
#   -> ['B1', 'BB', 'MC', 'BOTD1', 'BOTD2', 'ISWG', 'B2']      (7)
git show 9595bd82:apps/desktop/src/monsterCatalog/MonsterCatalogScreen.tsx  | grep 'B3:'
#   -> B3: 'Bestiary 3',
```

`origin/tranche/9` was red on its own gate before this lane touched it. Fixed here — a mechanical
defect is an explicit PRESS ON case, not a `decision-blocked` — by adding `B3` to `SERVED_BOOKS` with
the attribution in a comment beside it.

**The pair is the finding.** The same hand-maintained frontend list went stale in two catalogs one
commit apart, and the two failed in opposite directions: the companion list was **short of the
labels** so three books were checked by nothing, and the monster list was **short of the labels** so
the gate refused. Identical cause, and only one of the two was visible. Nothing on the frontend side
derives these lists from `COMPANION_BOOKS`/`MONSTER_BOOKS`; every book registration leaves two lists
to remember, and remembering is what failed twice. Deriving them needs a fixture the frontend does
not have, so this is recorded rather than solved — but it is now recorded as a *pattern* with two
instances, not as one lane's oversight.

### 54.7 What landed

| book | units | creatures | abilities | grounded | needed a `RuleSetId` |
|---|---|---|---|---|---|
| `bestiary` (Bestiary 1) | 59 | 24 | 35 | **59** | **no** |

* **The first companion book needing no new `RuleSetId` and no scope flip.**
  `RuleSetId::Bestiary1` was already compiled for the book's monsters and equipment, so registering
  its companions moved **zero** units of any other kind — where `§51.6` measured 1,099 for round 2's
  three books. The per-book cost was a `COMPANION_BOOKS` row, a `COMPANION_BOOK_SPECS` row, a wire
  code, a frontend label, a `reach_gate` claim, and a widening of `beastiary1_counts()`.
* **The first book whose companion tables sit beside another family's tables in one module.**
  `rules_tables::beastiary1` now holds 46 monsters and 24 companions.
  `the_companion_rows_are_not_this_module_s_monster_rows` pins that the two populations never
  collide: `Companion (Wolf)` is an advanceable companion chassis row, `Wolf` is a stat block, both
  are Bestiary 1 and neither is the other. `§52.8` flagged this book to the monster lane as "the only
  remaining book where the chassis meets an EXISTING ingest"; the same is true here, and the answer
  is that they sit alongside under one `RuleSetId` with a test saying they are disjoint.
* `data/corpus/beastiary/companion/` — 59 records, **zero PI redactions**. The book's own
  `NAMEISPI:YES` audit (`§50.1`'s corpus-wide finding, checked rather than assumed):
  `grep -c 'NAMEISPI:YES' bestiary/b1_races_companion.lst bestiary/b1_abilities_companion.lst` → **0**
  for both.
* `reach_gate` claim `("beastiary1", "companions")` — a **second** claim for a book that already
  claimed `("beastiary1", "monsters")`, and the only claim in that file whose family id and corpus
  directory are different words.
* Corpus-wide `companion` grounded **135 → 194**.

### 54.8 The remainder, re-derived — and round 4's queue

Re-derived at the end of the round with the same command that opened it
(`python3 scripts/classify_companion_rows.py`), now carrying shape 4:

| book | units | orphans | class rows | **reachable** |
|---|---|---|---|---|
| `ultimate_wilderness` | 575 | 247 (43%) | 0 | **328** |
| `core_essentials` | 145 | 51 (35%) | 0 | **94** |
| `core_rulebook` | 170 | 84 (49%) | 2 | **84** |
| `bestiary_4` | 80 | 5 (**6%**) | 0 | **75** |
| `bestiary_3` | 85 | 19 (22%) | 0 | **66** |
| `ultimate_magic` | 170 | 135 (79%) | 3 | **32** |
| `advanced_race_guide` | 32 | 18 (56%) | 0 | **14** |
| `advanced_players_guide` | 212 | 208 (98%) | 0 | **4** |
| `book_of_the_damned_volume_1` | 31 | 27 (87%) | 2 | **2** |
| **total** | **1,500** | **794** | **7** | **699** |

**`§51.7`'s wall is still there for every one of these**, and round 3 did not build the machinery to
climb it — it found a book that never needed it. Round 4 has no such book left: the cheapest
remaining, `bestiary_4` at 6%, still needs a scoped `Reach::Surfaced` claim over the linked subset
plus an `OPEN_FINDINGS` entry naming its 5 orphans, which is the disposition the monster lane already
took for Inner Sea World Guide (`§50`).

**Four hazards named so round 4 does not pay to discover them.**

1. **`bestiary_4` is the concurrent monster lane's round-5 target too** (`§52.8` ranks it first at 749
   reachable monster units). Round 3 chose `bestiary` partly to avoid that collision, and confirmed
   mid-round that the monster lane had claimed `bestiary_3` on this branch. `§51.5`'s ruling stands —
   two lanes registering families of one book is the designed path — but the two rounds must not add
   the same `RuleSetId` variant in the same hour. Check `git log origin/tranche/9` before writing
   `RuleSetId::B4`.
2. **`ultimate_wilderness`, `core_rulebook`, `ultimate_magic` and `book_of_the_damned_volume_1` carry
   `*_classes_companion.lst` rows.** `transcribe_companion_tables` refuses those books outright.
   `§51.7` named the first three; `book_of_the_damned_volume_1` carries 2 more and `§51.1`'s own
   breakdown says so, but the hazard list left it out. The classifier's `clas` column has printed all
   four all along — the column was there and was read past, which is the same failure the `ORPHAN`
   column produced in `§54.1`. Nothing about the instrument changed here; only the list did.
3. **`bestiary_3` is now a free registration too.** The merge this round took brought in the monster
   lane's Bestiary 3 ingest, which moved the book `future_state` → `in_scope` under `RuleSetId::B3`.
   Its 85 companion units (66 reachable) therefore cost no scope flip, exactly as `bestiary`'s did —
   the second instance of `§51.5`'s designed cross-lane path, and this time the companion lane is the
   one being paid for rather than the one paying.
4. **`core_essentials` and `bestiary_3` need the spec widened to MULTIPLE source files per shape.**
   `CompanionBookSpec` carries one `races_lst` and one `abilities_lst`; `core_essentials` has six
   companion `.lst` files and `bestiary_3` four (`b3_races_companion.lst`, `b3_races_familiar.lst`,
   `b3_abilities_companion.lst`, `b3_abilities_familiar.lst`). Every book registered so far had
   exactly two, so the single-file spec has never been wrong before and reads as though it were
   general. Round 3 chose `bestiary` partly because it is the last two-file book left.
## Decision 55 — Monster / Monster-Ability Lane, extend: round 5 (2026-08-12, `sd29-monster-r7`, card `epic-5-monster-lane-extend`)

Round 5 ingested **Bestiary 3** end-to-end — **288 of its 301 units**: all 261 monster rows and 27
of its 40 ability rows, the other 13 being orphans no monster row of the book owns. It is the first
book in the lane that loses **no monster row at all**: zero `NAMEISPI:YES`, zero
`PI_BLACKLIST_TERMS` hits, zero `.COPY=` deltas. Grounded `monster` 394 → **655**, `monster_ability`
488 → **515**. The REAL ceiling moves 2,055 → **1,767**, and `2055 − 288 = 1767` closes exactly with
no residue.

`§53` is the race-trait lane's round 5 and `§54` the companion lane's round 3; all three landed the
same day on the same branch. This section is numbered 55 to avoid a collision, not because 54 is
missing.

### 55.1 341 units the lane's denominator does not count — and it inverts `§44.4` for this book

The card asked whether `monster_ability` carries a ceiling analogous to the race-trait lane's, where
`§44.4` found 2,894 of 3,447 units structurally unreachable. It carries the **opposite** problem, and
that is this round's most reusable output.

`b3_races.lst` carries **100** `ABILITY:Special Ability|AUTOMATIC|` tokens, and the classifier still
reports `row-named` **0**. Both are true, and the reason is a kind boundary rather than a link
failure:

```text
b3_abilities_race.lst:289  TYPE:SpecialQuality.Extraordinary.AdaroRacial        -> monster_ability
b3_abilities_race.lst:703  TYPE:AghashRacialAbility.SpecialQuality.Supernatural -> race_trait
```

`v06_work_inventory::file_kind` reads only the **first** `TYPE:` segment. Both rows are a monster's
special quality, namespaced to a monster of this book; they differ only in which segment the book
happened to write first. `b3_abilities_race.lst`'s 838 units split **798 `race_trait` / 40
`monster_ability`** on that basis alone.

| measure | count |
|---|---|
| `race_trait` units in `b3_abilities_race.lst` | 798 |
| …whose `KEY:` is namespaced `<X> ~ <Y>` | 778 |
| …whose `<X>` is a **bestiary_3 monster** | **341** |
| …and which also carry `SpecialQuality`/`SpecialAttack` in a later `TYPE:` segment | 340 |

**Two independent derivations, sharing no intermediate artifact, agree on 341.** The first joins
`race_trait` key prefixes from `docs/work-inventory.json` against the book's own monster
`corpus_key` set. The second never opens the inventory: it parses `b3_races.lst` and
`b3_abilities_race.lst` directly, takes every ability row whose *first* `TYPE:` segment is not a
facet, and joins its key prefix against the monster `KEY:`s read straight from the races file —
**261 monster keys, 341 owned rows**. That agreement is what makes this a finding rather than a
reading of one script, and it is the practice `validate-proxies-against-known-truth` asks for: test
the instrument where it makes its confident claim.

**This contradicts `§44.4` for this book.** That section counted Bestiary 3's 799 `race_trait` units
among the 2,894 "belonging to races with no chassis", concluding "no amount of race-trait ingest
grounds those" because `RaceCorpus::resolve` returns `None` without a chassis. That is correct for a
player race trait and **wrong for these 341**: their owners are monsters, and this round gives those
owners a chassis. They are reachable through the monster catalog's existing ability rendering — the
path this round's item-8 screenshot shows working for `Adaro ~ Poison` — not through a race chassis
that will never exist for a Bestiary 3 monster.

**They are deliberately NOT ingested.** Reclassifying them changes `file_kind`, which redraws the
`race_trait` and `monster_ability` denominators for every book in **two lanes at once**; doing that
inside an ingest round would leave this card's numbers unreconcilable against round 4's. Recorded
with its derivation so a successor can price it, which is what `§45.1` asks a round to do *before*
committing to a book.

**The scope of the claim, stated precisely: this round measured only the book it took.** The same
measurement should be run on `bestiary_4`, `bestiary` and `inner_sea_bestiary` before anyone treats
1,767 as the lane's true size. `§46.1` and `§50.7` both moved the ceiling *down* after finding the
instrument over-reported; this is the first evidence it may also under-report, and the two are not
symmetric — an over-report wastes a round, an under-report hides work from the plan entirely.

### 55.2 A refusal that fired on a row it was going to discard, and the wrong fix that regeneration caught

`parse_desc` refuses to pick among several `DESC:` texts when none is gated on `DisplayFullAbility`.
The refusal is right. Its *placement* was not: it raised `SystemExit` from inside
`ability_pi_reason`, which parses **every** ability row — including the orphans the very next pass
discards. `b3_abilities_race.lst:1663` (`Jiang-Shi Vampire`: 11 `DESC:` tokens describing an acquired
template in 11 sections) is an orphan, because the base creature row it templates is **commented
out** at `b3_races.lst:293`. So a row that was never going to be emitted aborted the transcription of
288 records.

The refusal is now **deferred, not weakened**: unscreenable rows are collected in a set, and the
transcription raises only if one **survives** to be emitted. A shape the parser cannot read still
cannot reach a player.

**The first fix attempted was wrong, and how it was caught is the transferable part.** Moving the
Product Identity screen to run *after* the orphan pass also fixes the crash, and is what reading the
code alone suggests — the existing ordering comment even argues for exactly that kind of move, one
step earlier. Regenerating all six previously ingested books showed it silently relabelled three
Inner Sea World Guide rows from "Product Identity" to "orphan" in that book's generated header. Those
three are *genuinely* PI, and PI is the stronger and more durable reason: it holds even if a future
round gives the row an owner, where "orphan" does not. The reorder was reverted for the narrow fix,
under which **all six previously ingested books regenerate byte-identically**.

**The general lesson.** A change that makes a failing case pass, and that the surrounding comments
appear to endorse, is still a behaviour change everywhere else the code runs. The cheap check is to
re-run the generator over everything it has ever produced and diff — seconds of work that separated
a correct fix from a plausible one. `§52`'s four findings were all "an instrument validated only
where it happened to be right"; this is the same shape applied to a *fix* rather than a measurement.

### 55.3 `source_page` is not guaranteed, and seven books said otherwise by accident

`monster_catalog::every_row_carries_the_fields_the_screen_renders` asserted every served monster
carries a non-empty `source_page`. It passed for seven books because all seven happened to state one.
`b3_races.lst:215` (`Owl (Giant)`) and `:265` (`Spider (Ogre)`) carry no `SOURCEPAGE:` token at all.

The transcriber emitted `None`, which is its documented and correct behaviour — a token the row does
not carry becomes `None` rather than an invented citation. Both records state everything else the
screen renders (name, size, type, challenge rating, speeds, natural attacks), so **dropping them
would withhold real content over a bibliographic field**, which is the over-exclusion cost `§50`
already warned about in the Product Identity context.

Resolution: they ship; the monster row renders its page clause **conditionally**, as the ability row
directly beneath it has always done — the old code interpolated an empty string and left the book
name with a dangling trailing space, a small live rendering defect this surfaced; and the two are
pinned by served key with their corpus lines, **with the assertion failing in both directions**, so a
pinned row silently *gaining* a page is caught as surely as a new row losing one.

### 55.4 The one gate stage this round turned red was a hand-pinned frontend denominator

`MonsterCatalogScreen.test.ts`'s `SERVED_BOOKS` still listed round 4's seven wire codes, so
`BOOK_LABELS names exactly the served books` failed on the eighth. **The concurrent companion lane
recorded the identical defect class the same day** (`§54.5`, a stale `SERVED_BOOK_CODES` that left
three books' labels checked by nothing). Two lanes, two catalogs, one shape: a frontend list that
must track a backend registry, pinned by hand in a second place with nothing deriving one from the
other.

Worth naming as a class rather than fixing twice in silence. Both catalogs' backends already expose
the served book set; neither test reads it. The cheap durable fix is for the frontend test to derive
its expected set from the served response rather than restate it — recorded here as forward scope,
not taken in an ingest round.

### 55.5 The lane's queue after this round

`python3 scripts/classify_monster_ability_rows.py`, raw remaining **3,207** (`monster` 615 +
`monster_ability` 2,592), REAL ceiling **1,767**:

| book | remaining units | orphans | PI | **reachable** |
|---|---|---|---|---|
| `bestiary_4` | 988 | 225 | 14 | **749** |
| `bestiary` | 807 | 146 | 0 | **661** |
| `inner_sea_bestiary` | 230 | 26 | 7 | **197** |
| `inner_sea_gods` | 200 | 81 | 3 | **116** |

`bestiary_4` is the biggest. `bestiary` remains the only book where the chassis meets an **existing**
ingest — its 46 SD-22 monsters are already grounded through `beastiary1`'s own tables, so a round
taking it must first rule on whether the chassis absorbs them or sits alongside them; that ruling is
the real cost of that book, not its 807 rows.

**Eleven books now hold 716 orphan abilities and zero remaining monsters**, up from ten and 703 — and
`bestiary_3` joined that list by ingesting *all* of its monsters rather than by having none, which is
worth distinguishing: the two shapes read identically in the classifier's output and mean opposite
things. Running a per-monster cycle against either remains a reportable hard stop.


---

## Decision 56 — Companion Lane, extend: round 4 (2026-08-12, `sd29-companion-r8`, card `epic-7-companion-lane-extend`)

Round 4 was dispatched with `§54`'s closing sentence as its marching order: **"round 4 has no
orphan-free book left"**, so its first move had to be the scoped-`Reach::Surfaced`-plus-
`OPEN_FINDINGS` disposition `§50` prescribes, or a wait on `§48.1`'s operator ruling.

That prediction was structurally identical to `§51.7`'s, which `§54` had just disproved one round
earlier. **It was wrong the same way, for the same reason, and this is now the second consecutive
round in which the orphan instrument was found to be UNDER-claiming.** The disposition was built
anyway — it is genuinely needed and it ships — and then Bestiary 3's 19 orphans turned out not to be
orphans. The book landed with **85 units, all 85 grounded, zero `OPEN_FINDINGS` shortfall**.

The lane's ceiling moved **up** again: **893 → 937**.

### 56.1 Ownership shape 5: the namespace prefix can be the creature's DISPLAY name

`classify_companion_rows.py` resolved a namespaced `KEY:<Owner> ~ <Leaf>` against two things: the
creature `KEY:` set, and `bare_species`, which unwraps the corpus's two companion wrappers
(`Companion (X)`, `Familiar (X)`). Bestiary 3 is where that is not enough.
`b3_races_familiar.lst:18`:

```
Kyton, Augur   SORTKEY:Kyton   KEY:Kyton (Augur)   OUTPUTNAME:Augur   ...
```

The creature's key is `Kyton (Augur)`. Its six ability rows are keyed `Augur ~ Spell-Like
Abilities`, `Augur ~ Unnerving Gaze`, and so on — namespaced to the **`OUTPUTNAME:`**, which is what
a player actually sees, not to the `KEY:`. Under shapes 1-4 nobody claims them.

Six of this book's 31 creature rows are shaped this way, and between them they own **all 19** rows
the classifier reported as orphans:

| creature `KEY:` | `OUTPUTNAME:` | orphan rows it owns |
|---|---|---|
| `Kyton (Augur)` | `Augur` | 2 |
| `Div (Doru)` | `Doru` | 2 |
| `Dragon (Faerie)` | `Faerie Dragon` | 3 |
| `Archon (Harbinger)` | `Harbinger Archon` | 5 |
| `Rakshasa (Raktavarna)` | `Raktavarna` | 4 |
| `Oni (Spirit)` | `Spirit Oni` | 3 |

**Shape 5, display-name:** the `<Owner>` of shape 3, resolved additionally through a map built from
each creature row's own `OUTPUTNAME:` token.

**The token is READ, never inferred.** The tempting fix is to generalise `bare_species` to unwrap
any `X (Y)` into `Y` — it produces `Augur` from `Kyton (Augur)` and would have made this book pass.
It is wrong. The same file carries `Familiar (Flying Squirrel)`, `Familiar (Fox)`, `Familiar (Goat)`,
`Familiar (Otter)`, `Familiar (Pig)`, `Familiar (Raccoon)`, where the parenthesis is a **wrapper**
and the inner word is the species; in `Kyton (Augur)` the parenthesis separates a **genus from a
species** and the display name is a rearrangement the corpus states explicitly
(`Archon (Harbinger)` → `Harbinger Archon`, which no string surgery over the key produces at all).
A generalised unwrap would have been right here by luck and wrong next door. `OUTPUTNAME:` is the
corpus's own statement of the answer, and the corpus is the only authority this lane accepts.

**Both instruments learned it.** `classify_companion_rows.py`'s ORPHAN column and what
`transcribe_companion_tables.py` drops must agree; `§54.2` records the round where they did not, and
the classifier printed 886 while every doc carried 879. The shape is implemented in both files, and
the four new chassis tests pin the result rather than the method.

**The habit this is the second instance of.** Every correction the lane's orphan instrument has taken
— `§48.1` 1,696 → 888, `§51.1` 888 → 879 — was the instrument finding it had OVER-claimed reach.
`§54.1` was the first that went the other way, and this is the second. The lane has now been wrong in
the same direction twice in a row, and both times the discovery came from *looking at the specific
rows* a book was about to lose rather than from trusting the count. **A round that accepts an orphan
figure without reading the rows behind it is making the mistake this decision documents.**
`§45.1` says classify before committing to a book; round 4 amends it: classify, and then read what
the classifier is about to throw away.

### 56.2 `CompanionBookSpec` was a single-file spec that read as though it were general

`§54`'s hazard (c) named this and it was correct: Bestiary 3 carries `b3_races_companion.lst` **and**
`b3_races_familiar.lst`, `b3_abilities_companion.lst` **and** `b3_abilities_familiar.lst`. Every book
registered before it had exactly one file per shape, so `races_lst: &'static str` had never been
wrong.

The failure it would have produced is not a compile error. `verified_citation_line(&races_file,
record.source_line, record.name)` checks that the cited line really carries the record's name — a
genuinely good guard, and the reason this lane's citations are trustworthy. With one file assumed and
four in play, it would have verified a `b3_races_familiar.lst` record's line number **against
`b3_races_companion.lst`**, and either panicked with a misleading message or, worse, matched a
different row that happened to share a name.

Fixed by following a precedent that already existed in the same file: `MonsterBookSpec::races_lsts`
has been a `&'static [&'static str]` since Inner Sea World Guide, and `gen_monster_book` keys a
`HashMap<&str, CorpusFile>` by file name and looks up **per record**. `CompanionBookSpec` now does
exactly that, for both shapes, and `CompanionRecord`/`CompanionAbilityRecord` carry a `source_file`
beside their `source_line`. The generator panics by name if a record cites a file its book's spec
does not list — "a citation this generator cannot verify is not a citation", the monster generator's
own words.

**All 8 previously-registered books were regenerated, not hand-edited**, and the diff was checked to
be purely additive before it was committed: `git diff -U0 -- '*/companion_data.rs' | grep -E
"^[+-].*(CompanionRecord \{|CompanionAbilityRecord \{)"` → **no output**, i.e. not one record was
added or lost in any of the eight.

### 56.3 The orphan-drop disposition ships, and is not exercised by this round's book

`§50`'s disposition — transcribe the linked subset, drop the orphans, name them in the generated
header, carry them as an `OPEN_FINDINGS` entry — **is implemented** in
`transcribe_companion_tables.py`, and the registration predicate in `companion_chassis`'s module doc
moves with it:

* **was:** a book is registerable when EVERY one of its ability rows has an owner (per-book).
* **now:** a book may leave rows behind, but it may never SHIP a row nothing can reach (per-row).

The second half is the one that matters and it is now a test —
`every_shipped_ability_row_is_owned_by_a_creature_of_its_own_book` — asserting over every registered
book that no shipped ability has an empty `owners`, and that every owner named is a creature row of
that same book.

**Bestiary 3 does not exercise the drop path**, because shape 5 left it with nothing to drop. This is
stated rather than hidden: the mechanism is live code on the path every future book takes, and
`bestiary_4` (5 orphans), `core_essentials` (26), `ultimate_wilderness` (247) will exercise it. A
successor should not read "the disposition is built" as "the disposition has been proven on a real
book" — it has not, and that is this round's honest shortfall.

### 56.4 Re-derived denominators, and where the 699 in the dispatch brief went

The dispatch brief carried `§54`'s **699** as the honest remainder and instructed that a disagreeing
derivation wins. It reproduced **exactly** before shape 5 was written — the same command over the same
nine books printed `reachable remainder : 699` — so `§54`'s figure was correct when written and is
superseded by a mechanism change, not by an error.

Corpus-wide, over all 17 books that carry companion units
(`python3 scripts/classify_companion_rows.py inner_sea_combat monster_codex inner_sea_intrigue
horror_adventures bestiary_5 bestiary_6 bestiary_2 bestiary bestiary_3 bestiary_4 core_essentials
ultimate_wilderness core_rulebook advanced_race_guide ultimate_magic book_of_the_damned_volume_1
advanced_players_guide`):

| measure | value |
|---|---|
| total companion units in scope | **1,696** |
| orphan ability rows | **750** (was 794; shape 5 recovered 44) |
| `PRECAMPAIGN`-gated on an uningested campaign | 2 |
| `*_classes_companion.lst` class rows the chassis refuses | 7 |
| **reachable remainder (the lane's REAL ceiling)** | **937** (was 893) |

Grounded, from the regenerated inventory
(`python3 -c "import json,collections; inv=json.load(open('docs/work-inventory.json'));
print(collections.Counter(x['status'] for x in inv['units'] if x['kind']=='companion'))"`):
**279 grounded**, 1,337 `not-ingested`, 80 `not-started`. That is 194 + 85, closing exactly on this
round's ingest.

**Honest remainder after round 4: `937 - 279 = 658`.**

The `1,696 - 279 = 1,417` a status reader would compute from the inventory alone is NOT the lane's
workload, and never was; 759 of it (750 orphans + 2 gated + 7 class rows) is a ceiling no ingest can
cross without a chassis change priced separately.

### 56.5 Round 5's queue, ranked by orphan share and re-derived this round

Every figure below is from this round's own classifier run, after shape 5:

| book | units | orphans | share | reachable |
|---|---|---|---|---|
| `bestiary_4` | 80 | 5 | 6% | **75** |
| `core_essentials` | 145 | 26 | 18% | **119** |
| `ultimate_wilderness` | 575 | 247 | 43% | **328** |
| `core_rulebook` | 170 | 84 | 49% | **84** |
| `advanced_race_guide` | 32 | 18 | 56% | **14** |
| `ultimate_magic` | 170 | 135 | 79% | **32** |
| `book_of_the_damned_volume_1` | 31 | 27 | 87% | **2** |
| `advanced_players_guide` | 212 | 208 | 98% | **4** |

The reachable column sums to **658**, which is the honest remainder `937 - 279` independently — the
two derivations close exactly. Note that a book's reachable count subtracts its class rows as well as
its orphans, which is why `core_rulebook` is 84 rather than 86 and `ultimate_magic` 32 rather than
35; the first draft of this table got all three of those wrong by subtracting orphans only, and the
sum is what caught it.

`core_essentials` improved most under shape 5 (51 orphans → 26), and is now the second-cleanest book
left rather than the third.

**Hazards carried forward, re-checked rather than copied:**

1. **`bestiary_4` is still the monster lane's next target** (`§52.8`), and it needs a new
   `RuleSetId::B4` — the collision `§54`'s hazard (a) named. It is unchanged and still real: check
   `git log origin/tranche/9` before writing `RuleSetId::B4`. Round 4 avoided it by taking
   `bestiary_3`, whose `RuleSetId::B3` the monster lane had already compiled.
2. **`ultimate_wilderness`, `core_rulebook`, `ultimate_magic` and `book_of_the_damned_volume_1` carry
   `*_classes_companion.lst` rows** the chassis refuses outright (7 in total). The transcriber raises
   on them by name; it does not silently drop them.
3. **`core_essentials` has 6 companion `.lst` files.** The multi-file spec built this round handles
   it — that hazard is now closed, not merely named.
4. **Shape 5 has not been swept over the already-registered books' ORPHAN counts for a REGRESSION**
   beyond confirming all 8 still report 0 orphans and that their record counts did not move. Both
   were checked and both hold.

---

## Decision 57 — Monster / Monster-Ability Lane, extend: round 6 (2026-08-12, `sd29-monster-r8`, card `epic-5-monster-lane-extend`)

Round 6 took `bestiary_4`, the largest reachable book left in the lane, and ingested **749 of its 988
units** — 206 of 220 monster rows and 543 of 768 ability rows. It is the largest single ingest this
lane has taken since round 4, and the first in which the exclusions are the interesting part.

**This decision does not claim the lane is done.** The REAL ceiling after this round is **1,018**.

### 57.0 Every figure, command first

The lane's REAL ceiling, **reproduced exactly at cycle start before being moved** — round 5's closing
figure confirmed, not corrected:

```
python3 scripts/classify_monster_ability_rows.py
```

→ `remaining … 3207`, `orphan … 1406`, `PI … 32`, `.COPY= … 2`, **`reachable remainder … 1767`**.

Lane denominators, over the regenerated `docs/work-inventory.json`, summing `not-ingested` +
`not-started` across every book whose `scope` is not `out_of_scope` — the command rounds 1-5 record:

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
oos = {b['id'] for b in d['books'] if b['scope'] == 'out_of_scope'}
for kind in ('monster', 'monster_ability'):
    rem = sum(1 for u in d['units'] if u['kind']==kind and u['book'] not in oos
              and u['status'] in ('not-ingested','not-started'))
    got = sum(1 for u in d['units'] if u['kind']==kind and u['book'] not in oos
              and u['status']=='grounded')
    print(kind, 'remaining', rem, 'grounded', got)"
```

| | before | after | Δ |
|---|---|---|---|
| `monster` remaining | 615 | **409** | −206 |
| `monster_ability` remaining | 2,592 | **2,049** | −543 |
| raw remaining total | 3,207 | **2,458** | −749 |
| `monster` grounded | 655 | **861** | +206 |
| `monster_ability` grounded | 515 | **1,058** | +543 |
| **REAL ceiling** | 1,767 | **1,018** | −749 |

`1767 − 749 = 1018` closes exactly, with **no residue** — like round 5 and unlike round 4, because
this book's `.COPY=` column is 0.

`206 + 543 = 749` is *also* exactly the classifier's own `reachable remainder` for the book
(`988 − 225 − 14 − 0`). What ships and what the ceiling says should ship are the same number arrived
at by two routes that share no intermediate artifact, and the equality is pinned by a test
(`the_shipped_total_is_the_classifiers_reachable_remainder`) rather than left as a coincidence in a
receipt.

**The dispatch brief's "monster ~305, monster_ability ~852, grounded 62 and 20" was wrong for the
FIFTH round running.** `§46.1`, `§50.7`, `§52` and `§55` each corrected the identical pair and the
round-6 brief repeated it verbatim again. Retro event emitted. The pair is near `bestiary`'s own book
subtotal (284/523), not the corpus-wide figure — a brief-template defect at this point, not a
per-round slip.

### 57.1 The round's largest finding: a Product Identity predicate two rounds got right by luck

`bestiary_4` is the **first `roleplaying_game/` bestiary in this lane that carries any
`NAMEISPI:YES` row**: `grep -c NAMEISPI:YES b4_races.lst b4_abilities_race.lst` → **14** and **0**.

Rounds 4 and 5 each recorded `ogl-pi-blacklist.md` §2's prediction in a **book-location** form — "a
`roleplaying_game/` bestiary carries zero PI rows" — and each was *right about its own book*.
Bestiary 2 and Bestiary 3 genuinely carry none. But the location form is not what the blacklist says.

§2.1's predicate is **per record**: a *generic SRD species name* ("Goblin", "Owlbear") is
presumptively Open Game Content, and the blacklist entry is for "*non-bestiary* uses of a monster's
proper name (e.g. a unique named NPC monster)". All 14 rows here are unique named personas:

| rows | `b4_races.lst` lines | what they are |
|---|---|---|
| 3 | 40, 41, 42 | Demon Lords |
| 3 | 66, 67, 68 | Empyreal Lords |
| 3 | 110, 111, 112 | Great Old Ones |
| 3 | 139, 140, 141 | Kaiju |
| 2 | 219, 222 | two named spawn |

Not one generic species among them. **The book-location form of the rule would have shipped all 14.**
The prediction is *refined, not contradicted* — and the transferable point is the one this lane keeps
re-learning under different names: an instrument validated only where it happens to be right
(`§52.6`, `§55.3`, and now this). The correction cost nothing here only because the corpus declares
the answer per row; a book that carried PI without `NAMEISPI:YES` would have shipped it.

Recorded as a **refinement of `ogl-pi-blacklist.md` §2's operational reading**, not a change to the
document — the file is DRAFT and operator-reviewable, and re-wording a governance doc is not an
ingest round's write scope. **Unattended-mode default taken: record the refinement where the lane
reads it (`rules_tables::bestiary_4`, `RuleSetId::B4`, this decision), leave the governance file to
its owner.**

### 57.2 225 orphans, 73 of them created by this round's own screen

The 225 excluded ability rows are not one class, and the split is derived rather than assumed:

| class | count |
|---|---|
| orphans in their own right — no monster row ever named them | 152 |
| **cascade** — namespaced to one of the 14 dropped PI monsters | **73** |
| total | 225 |

Derived by taking the transcriber's own orphan-key output and joining it against the 14 PI monster
keys read straight from the corpus rows that declare `NAMEISPI:YES` (script recorded in the receipt).
A cascade row is perfectly well-formed and perfectly owned; it is unreachable only because its owner
is Product Identity. **This independently reproduces the `152 → 225` figure the round-4 queue note
carried**, which until now was a number nobody had re-derived.

It also makes `every_owner_named_by_a_shipped_ability_is_a_shipped_monster` load-bearing for the
first time. Until this book, "owners is non-empty" and "every owner ships" could not diverge, because
no book had dropped a monster that owned anything. Here 73 rows would satisfy the weaker test and
still name a creature the catalog cannot render.

**83 of the 152 live in a second file this book ships nothing from.**
`b4_abilities_races_ce.lst` contributes 83 orphan rows and 0 shipped records; all 543 shipped
abilities come from `b4_abilities_race.lst`. That is *not* an artifact of the transcriber reading a
single file — it takes its unit set from the inventory across every source file, as it must for Inner
Sea World Guide's 7/7 monster split. Checked at the point of the confident claim rather than inferred:
`grep -c 'ABILITY:Special Ability|AUTOMATIC|Immunity to Calm Emotions' b4_races.lst` → **0**, and the
file's own second line reads `#This should probably go into ce_abilities_race.lst`. They are generic
reusable abilities no monster row names.

### 57.3 The `§55.1` measurement round 5 asked a successor to run — and a correction to its own number

`§55.1` asked that its measurement be run on `bestiary_4`, `bestiary` and `inner_sea_bestiary`
"before anyone treats 1,767 as the lane's true size". Round 6 ran it:

```
python3 -c "
import json
d=json.load(open('docs/work-inventory.json'))
for book in ('bestiary_3','bestiary_4','bestiary','inner_sea_bestiary'):
    units=[u for u in d['units'] if u['book']==book]
    keys={u['corpus_key'] for u in units if u['kind']=='monster'}
    names={u['name'] for u in units if u['kind']=='monster'}
    traits=[u for u in units if u['kind']=='race_trait']
    ns=[u for u in traits if u.get('corpus_key') and ' ~ ' in u['corpus_key']]
    by_key=[u for u in ns if u['corpus_key'].split(' ~ ',1)[0] in keys]
    both=[u for u in ns if u['corpus_key'].split(' ~ ',1)[0] in (keys|names)]
    print(book,'race_trait',len(traits),'ns',len(ns),'byKEY',len(by_key),'byEITHER',len(both))"
```

| book | `race_trait` units | namespaced | by `KEY:` | **by key or name** |
|---|---|---|---|---|
| `bestiary_4` | 86 | 79 | 61 | **61** |
| `bestiary` | 21 | 19 | 9 | **9** |
| `inner_sea_bestiary` | 4 | 3 | 2 | **2** |
| `bestiary_3` | 799 | 779 | 341 | **625** |

**The answer to round 5's question: the understatement is almost entirely `bestiary_3`'s.** The three
books it named contribute **72** mis-filed units between them. **1,767 did not need re-drawing**, and
1,018 does not either — the lane can keep working from it.

**Round 5's own 341 is corrected to 625, and the correction is a predicate rather than an arithmetic
slip.** 341 reproduces *exactly* under round 5's predicate — match the namespace prefix against a
monster's `KEY:` — so the figure was right for what it measured. But this corpus namespaces an
ability by the monster's **display name** while the monster's `KEY:` carries a taxonomic prefix:

```
race_trait `Aghash ~ …`      -> monster KEY `Div (Aghash)`
race_trait `Androsphinx ~ …` -> monster KEY `Sphinx (Androsphinx)`
race_trait `Bone Golem ~ …`  -> monster KEY `Golem (Bone)`
```

Matching on `KEY:` alone misses every monster whose key differs from its name — 284 further units in
`bestiary_3`. This is precisely the `key-differs-from-name` trap the trap report raises **1,009**
times on that book and 939 times on this one; the round-5 measurement met the trap and did not
recognise it.

Name-matching is the weaker predicate in general and was **checked before being used**: across all
four books exactly **one** monster display name is ambiguous (`Unfettered Eidolon`, twice in
`bestiary_3`) and the other three books have none. Recorded as a bounded caveat rather than treated
as clean.

**Still not reclassified**, for the reason round 5 gave and this round agrees with: moving them
changes `file_kind`, which redraws the `race_trait` and `monster_ability` denominators for every book
in two lanes at once. **Unattended-mode default taken: measure and report, do not reclassify.**

### 57.4 Registration

The full eight points, including a new `RuleSetId::B4` — `grep -rn "bestiary_4" --include='*.rs'
--include='*.py' --include='*.ts' --include='*.tsx' src apps scripts` at cycle start returned only
doc-comment mentions, so no other lane had touched this book. The exhaustive matches did their
designed job again: adding the variant broke `v06_content_state_dump` and `v06_work_inventory` until
their arms were written.

**Both frontend copies of the served-book list were updated in the same edit** — `BOOK_LABELS` in
`MonsterCatalogScreen.tsx` and `SERVED_BOOKS` in `MonsterCatalogScreen.test.ts`. That pair is the
defect `§55` recorded turning round 5's gate red and `§54.5` recorded on the companion side one
commit apart. It is two hand-maintained copies of one fact with nothing but an assertion coupling
them; round 6 did not re-pay it, and a comment in the test now says why.

### 57.5 An invisible character in shipped rules text — the gate doing its job

Run 1 of the full gate came back **13 of 14 green with `clippy` the only red**, and it was a real
defect in this round's own ingest rather than an environment quirk. Three Bestiary 4 `DESC:` texts
reached the generated table carrying **U+00AD SOFT HYPHEN** — an invisible character *inside a word*
— and `clippy::invisible_characters` is deny-by-default, so it failed the build rather than merely
looking wrong to a player.

```
10-foot<U+00AD>radius    free<U+00AD>willed    cone<U+00AD>shaped
```

**Normalized to a plain `-`, and the choice between replacing and deleting is the part that needed
checking.** Deleting yields "10-footradius" and "coneshaped", which are wrong. Every occurrence
stands where a real hyphen belongs, mangled by line-breaking in the source PDF, and **the book
corroborates the character itself**: its own ability row is keyed `Pod-Spawned ~ Loss of Magic` with
a plain hyphen, for the same creature whose `DESC:` text carries the soft one. No word, number or
token changes — a character-encoding normalisation of a known extraction artifact, not a rewrite of
rules text.

Applied in `read_row` rather than on the `DESC:` path, so it lands before **every** downstream reader
including the Product Identity screen.

**Scope derived, not assumed:** 5 occurrences in `b4_abilities_race.lst`, **0** in Bestiary 2's and
Bestiary 3's equivalents. Confirmed by regenerating all six other registered books —
`git status --porcelain -- 'src/rules_core/rules_tables/*/monster_data.rs'` lists **only**
`bestiary_4`, so the transcriber change is provably additive. Record counts unchanged at 206 + 543.

**Two transferable points.**

**(a) `grep -P '\xc2\xad'` reported `0` for a file that provably holds three.** The scoping had to be
redone in Python, which found them immediately. That is the shimmed-`grep` hazard `AGENTS.md`'s
"derive counts with `awk`, not `grep -o`" rule warns about, hit here on `-P` instead of `-o` — and a
`0` from a broken tool reads exactly like a clean result. Any count that scopes a fix needs two
implementations agreeing, which is what caught it.

**(b) The fix was pushed the moment it existed, not held to cycle end.** The bad characters were
already on `origin/tranche/9` in `52da4bc3`, where they turn a **concurrent** lane's `clippy` stage
red through no fault of its own — exactly the failure `§52.5` records costing a sibling lane 22
minutes, whose stated mitigation is precisely this. Push-early and gate-after remain both right; the
cost of the pairing is that a fix must chase the push immediately.

### 57.6 Two concurrency hazards, neither previously recorded

Both were hit by this round while a sibling lane ran alongside it, and both are properties of the
dispatch rather than of either lane's work.

**(a) The dispatch scratchpad is SHARED between concurrently dispatched agents, and generic filenames
collide.** This round wrote its draft receipt to `<scratchpad>/receipt.md`; the concurrent companion
lane overwrote it with its own receipt minutes later. No repo content and no figure was lost — the
draft was reconstructed from commands already recorded — but a cycle receipt is exactly the artifact
a round cannot afford to lose silently, and nothing warned. The directory holds `verify.log`,
`verify1.log`, `msg1.txt`, `kanban.py`, `patch1.py`…`patch23.py` and dozens of other generic names
from several rounds and several lanes.

`AGENTS.md`'s concurrency section mandates one `CARGO_TARGET_DIR` per agent *per source tree* and one
worktree per agent, and says nothing about the scratchpad, which every agent shares.
**Ruling for this bundle: actor-prefix any scratchpad file a cycle cannot cheaply regenerate**
(`sd29-monster-r8-receipt.md`). **Forward scope:** the dispatch should hand each agent an
actor-scoped scratchpad subdirectory the way it already hands out a target dir — a convention nobody
has to remember beats one everybody has to.

**(b) Two lanes drafted a "Decision 56" simultaneously.** This lane and the companion lane both read
`§55` as the last section and both numbered next. Caught before either landed rather than as a merge
conflict, by checking origin rather than the local file:

```
git cat-file -p origin/tranche/9:docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md \
  | grep -n '^## Decision 5[5-9]'
```

→ `55` only, at the time of writing.

**Ruling, deliberately deterministic rather than a race: the LAST lane to push renumbers.** A lane
re-fetches `origin/tranche/9` before its final documentation push and checks for an existing section
at its number; if one is there, it renumbers its own section and every internal `§N.x` reference in
`decisions.md`, `kanban.md` and its receipt as part of the merge — the standard "re-read, merge on
top of newer content, publish again" flow, never a force-push. Recorded because a duplicate section
number in a shared doc is **silent**: nothing in `verify.sh` reads `decisions.md`, so the gate cannot
catch it, and two lanes on one branch will keep meeting this.

### 57.7 Round-7 queue, from ONE command

`python3 scripts/classify_monster_ability_rows.py`, raw remaining **2,458**, REAL ceiling **1,018**:

| book | remaining units | orphans | PI | **reachable** |
|---|---|---|---|---|
| `bestiary` | 807 | 146 | 0 | **661** |
| `inner_sea_bestiary` | 230 | 26 | 7 | **197** |
| `inner_sea_gods` | 200 | 81 | 3 | **116** |

**`bestiary` (Bestiary 1) is now the whole of the lane's large remainder and it cannot be taken
without a ruling first.** Its 46 SD-22 monsters are already grounded through `beastiary1`'s own
tables, so round 7 must decide whether the chassis absorbs them or sits alongside them. That ruling
is the real cost of the book, not its 807 rows, and it is the last structural question this lane has
left — after it, the remainder is two small campaign-setting books.

**Still eleven books holding 716 orphan abilities and zero remaining monsters** — unchanged by this
round, and the figure was re-derived rather than incremented. **A draft of this decision said
"twelve", and the classifier's own output corrected it before it shipped.** `bestiary_4` did *not*
join that list: it retains **14** remaining monster rows, the Product Identity personas, which no
ingest will ever take. `§55` distinguished two shapes that read identically in the classifier's
output — a book with no monsters, and a book whose monsters have all been ingested — and this book is
a **third**: reachable-exhausted but not monster-exhausted.

That distinction is the practical one for round 7, because `bestiary_4` now reads **0 reachable**
(`239 remaining − 225 orphans − 14 PI`) and is **finished as far as it can be finished**, exactly as
`inner_sea_world_guide` was at `§52`. Its 239 remaining units are the lane's permanent floor, not
queued work. Running a per-monster cycle against it, or against any of the eleven zero-monster books,
remains a reportable hard stop.

## Decision 58 — Monster / Monster-Ability Lane, extend: round 7 (2026-08-12, `sd29-monster-r9`, card `epic-5-monster-lane-extend`)

Round 7 took `inner_sea_bestiary` and ingested **190 of its 230 remaining units** — 38 of 40 monster
rows and 152 of 190 ability rows. It also answered the structural question `§57.7` left for this
round, and the answer is not the one the queue assumed.

**This decision does not claim the lane is done.** The REAL ceiling after this round is **821**.

### 58.0 Every figure, command first

The lane's REAL ceiling, **reproduced exactly at cycle start before being moved** — round 6's closing
figure confirmed, not corrected:

```
python3 scripts/classify_monster_ability_rows.py
```

→ `remaining … 2458`, `orphan … 1406`, `PI … 32`, `.COPY= … 2`, **`reachable remainder … 1018`**.

Lane denominators, over the regenerated `docs/work-inventory.json`, summing `not-ingested` +
`not-started` across every book whose `scope` is not `out_of_scope` — the command rounds 1-6 record:

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
oos = {b['id'] for b in d['books'] if b['scope'] == 'out_of_scope'}
for kind in ('monster', 'monster_ability'):
    rem = sum(1 for u in d['units'] if u['kind']==kind and u['book'] not in oos
              and u['status'] in ('not-ingested','not-started'))
    got = sum(1 for u in d['units'] if u['kind']==kind and u['book'] not in oos
              and u['status']=='grounded')
    print(kind, 'remaining', rem, 'grounded', got)"
```

| | before | after | Δ |
|---|---|---|---|
| `monster` remaining | 409 | **371** | −38 |
| `monster_ability` remaining | 2,049 | **1,897** | −152 |
| raw remaining total | 2,458 | **2,268** | −190 |
| `monster` grounded | 861 | **899** | +38 |
| `monster_ability` grounded | 1,058 | **1,210** | +152 |
| classifier `reachable remainder` | 1,018 | **828** | −190 |
| **REAL ceiling** (`828 − 7`, §58.1) | 1,018 | **821** | −197 |

`1018 − 190 = 828` closes exactly, no residue. The further −7 is not this round's ingest; it is
§58.1's correction to the instrument that produces 828.

**The dispatch brief's "monster ~305, monster_ability ~852, against grounded 62 and 20" was wrong for
the SIXTH round running.** `§46.1`, `§50.7`, `§52`, `§55` and `§57.0` each corrected the identical
pair and the round-7 brief repeated it verbatim again. The brief also carried "The previous round
reported 1018 remaining", which **is** correct and was reproduced exactly. Retro event emitted for
the pair.

### 58.1 The lane's ceiling instrument over-reports, and this is the book that proves it

`classify_monster_ability_rows.py` reports **197 reachable** for this book (`230 − 26 orphans −
7 PI − 0 .COPY=`). What ships is **190**. The residue is exactly **7**, and it is a difference of
*predicate*, not a transcription shortfall:

* The **classifier** screens a monster row's own `KEY:` and name, plus `NAMEISPI:`.
* The **transcriber** screens the values it is about to **emit** — and a monster's emitted record
  carries an `ability_keys` array holding the KEYS of the abilities the row names.

Seven of this book's ability rows are namespaced to a Golarion deity's proper name. Two monster rows
name them. Neither of those two monsters carries a blacklisted term in its own key or name, so the
classifier counts both reachable; neither can be *emitted* without emitting the deity's name inside
its own record, so the transcriber drops both — and their 5 remaining abilities lose their only owner
and fall out through the orphan pass. `2 + 5 = 7`, with nothing left over.

**This is `§57.2`'s cascade running backwards.** There, a dropped Product Identity *monster* orphaned
73 well-formed abilities. Here, Product Identity *abilities* drop their own owning monsters. The
lane has now seen the ownership edge carry Product Identity in both directions.

**Measured corpus-wide rather than asserted for one book**, applying the transcriber's emitted-value
predicate to every book with remaining monster rows and joining the result against the classifier's
own:

```
python3 <<'PY'   # full script in the round-7 receipt; it imports both scripts as modules
# for every book: monster rows the CLASSIFIER counts reachable and the
# TRANSCRIBER's emitted-value screen drops, plus the abilities owned only by them
PY
```

→ `inner_sea_bestiary  remMon 2  extraPI 2  cascade 5  overcount 7`, and **every other book: zero**.
Corpus-wide over-count is **7**, all of it this book's, so the lane's REAL ceiling is
`828 − 7 = 821`.

**The classifier was NOT changed.** Narrowing its predicate means running the transcriber's emitted-
value screen — which needs the link pass, the size/type/CR/page parse and the natural-attack parse —
inside a script whose job is to rank a queue cheaply. The measurement it makes is still the right one
for ranking, it is wrong only in the safe direction (it over-reports remaining work, never
under-reports it), and the exact residue is now known and pinned by a test in
`rules_tables::inner_sea_bestiary`. **Unattended-mode default taken: measure, pin and report; do not
rewrite the queue instrument inside an ingest round.**

### 58.2 The continuation `DESC:` shape — a third shape, widened deliberately

Three of this book's *shipping* rows stopped the transcription outright:
`isb_abilities_race.lst:227`, `:228`, `:229` (`Moxix ~ Gush`, `~ Hopedrinker`, `~ Mindshatter`). Each
carries **two `DESC:` tokens, neither gated**, and `parse_desc` refuses to pick one by position — the
guard `§46` put in after Book of the Damned Volume 2's summary/full-text pair.

The guard was right to fire and the rows are not that shape. They are **one description the corpus
split across two tokens**, and the corpus says so itself: every continuation begins with a space, and
concatenating in row order yields running prose.

```
DESC:… blood and pus spews forth from the wound.
DESC: The blood is extremely slippery and sprays out in a 20-foot radius …
```

Taking the first token alone would have served the trigger and dropped the 20-foot radius, the DC 28
Reflex save and the duration — the same loss `§46` recorded, arrived at from the other direction.

**The widening predicate is deliberately narrow: every token must carry no pipe entry at all** (no
gate, no `%N` variable) **and every token after the first must begin with a space.** That is what
keeps it a concatenation of verbatim corpus texts in the corpus's own order rather than a
composition. It was checked against the rows it must still refuse: `isb_abilities_race.lst:203`,
`:204` and `:206` of this same file carry `%N` variables and state *alternatives* rather than a
continuation, and they are still refused.

**Scope derived, not assumed.** A scan of every registered book's ability rows found 15 rows
corpus-wide carrying several ungated `DESC:` tokens — 1 in `bestiary_3`, 8 in `bestiary_4`, 7 here —
and only 4 of the 15 are continuations. The other 11 use `&nl;` separators or state alternatives, and
every one of them is an orphan or Product Identity row that no book ships. **Proof that the change is
additive:** all eight previously registered books were re-transcribed after the widening and
`git status --porcelain -- 'src/rules_core/rules_tables/*/monster_data.rs'` listed **only this
round's new file**. Not one record of any earlier book moved.

### 58.3 The `bestiary` ruling `§57.7` asked round 7 to make

`§57.7` recorded that Bestiary 1 is the whole of the lane's large remainder (661 reachable by the
classifier) and "cannot be taken without a ruling first" on whether the chassis absorbs its 46
already-grounded SD-22 monsters or sits alongside them. Round 7 made the ruling, derived the work it
implies, and did **not** execute it in the same round. Both halves are deliberate.

**The ruling: the chassis SITS ALONGSIDE `rules_tables::beastiary1` and takes the book's
complement.** Derived:

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
u=[x for x in d['units'] if x['book']=='bestiary']
print(collections.Counter((x['kind'],x['status']) for x in u
                          if x['kind'] in ('monster','monster_ability')))"
```

→ `monster grounded 46`, `monster not-ingested 284`, `monster_ability not-ingested 523`.

Splitting the 523 ability rows by **which** monster owns them — the derivation nobody had run:

| class | count |
|---|---|
| owned by at least one of the 284 REMAINING monsters | **323** |
| owned ONLY by one of the 46 SD-22 monsters | **54** |
| orphans — owned by nothing in the book | 146 |
| total | 523 |

So "alongside" ships `284 + 323 = 607`, and the 54 become a new named exclusion class:
**cross-table owner** — well-formed, owned, and unreachable only because the owner lives in a
different table.

**Why alongside rather than absorb.** Absorbing means the chassis emits all 330 monster rows, which
duplicates 46 records the catalog already serves under the same wire code `B1` — a player-visible
defect — unless SD-22's `beastiary1` monster tables, their `cache_gen`, their
`natural_attack_provenance`, their `data/corpus/beastiary/monster/` records and the
`beastiary1:monster:<slug>` key space are retired with them. That is a cross-bundle retirement of
shipped, grounded, player-visible records; it is not an ingest round's write scope, and it churns
content that is already correct.

**The mechanism blocker, named by line so round 8 pays nothing to find it.** Registering `bestiary`
in `monster_chassis::MONSTER_BOOKS` is NOT sufficient and is, on its own, a **regression**:

* `v06_work_inventory.rs`'s `Kind::Monster if facts.chassis_monster_keys.contains_key(engine_book)`
  arm takes precedence for the whole book once it is registered, so the SD-22 fallback arm
  (`facts.monster_names`) is never reached; and
* `EngineFacts::holds_key` hard-codes `if book == "bestiary_1" { return
  self.monster_names.contains(&name.to_lowercase()); }`, which ignores the chassis table entirely.

Together those two make a naive registration report the 284 new chassis monsters as `not-ingested`
**and** demote the 46 SD-22 monsters to `not-ingested` as well. The fix is small and principled —
Bestiary 1 is the one book served by two tables, so both must ground it — but it is a change to the
inventory's verdict logic and it belongs in a round that can gate it.

**Why not this round.** The card is loop-until-dry and the honest trade was one gate run against two
books versus one gate run against one book plus a verdict-logic change to the instrument every other
lane's denominator is read from. Round 7 banked and pushed 190 units first; the ruling costs round 8
nothing to re-derive. **Unattended-mode default taken: rule, derive, record the blocker by line, and
leave the execution to a round that opens with it.**

### 58.4 A test the table cannot carry, recorded rather than shipped wrong

A first draft of `rules_tables::inner_sea_bestiary`'s test module asserted the classifier's
`row-named 157 / prefix 0` split by requiring that no shipped ability has its namespace prefix as its
only owner. It fails at **96 of 152 rows**, and every one of the 96 is correct: when a monster row
names an ability whose namespace is that same monster, the row-named pass records the owner first and
the prefix pass adds nothing, leaving `owners == [prefix]` — indistinguishable in the table from a
prefix-only reach. The split is a property of the two *passes*, not of their output.

Recorded in the module rather than deleted silently, for the reason this lane keeps re-learning: an
instrument validated only where it happens to be right (`§52.6`, `§55.3`, `§57.1`) is the lane's
most-repeated defect, and this is the same shape caught one step earlier — before it shipped, by
running it rather than reasoning about it.

### 58.5 Round-8 queue, from ONE command

`python3 scripts/classify_monster_ability_rows.py`, raw remaining **2,268**, classifier reachable
**828**, REAL ceiling **821**:

| book | remaining units | orphans | PI | classifier reachable | note |
|---|---|---|---|---|---|
| `bestiary` | 807 | 146 | 0 | **661** | ruled in `§58.3`; ships **607** under the ruling, and needs the `holds_key` fix FIRST |
| `inner_sea_gods` | 200 | 81 | 3 | **116** | needs `MonsterAbilityRecord` to carry a `source_file`: its rows live in **two** ability files (`isg_abilities_races.lst` 145, `support/isg_abilities_races_b4.lst` 16), and `MonsterBookSpec::abilities_lst` is singular. The support pair is `PRECAMPAIGN:1,INCLUDES=Bestiary 4` — a gate this repo now **satisfies**, unlike `RuleSetId::Ha`'s and `B5`'s Occult Adventures gate, so those 19 units are in scope rather than out of it. Also: the inventory records those units' `source_file` as a bare basename while the file lives under `support/`, so path resolution must search the book root. |
| `ultimate_psionics` | 100 | 66 | 0 | **34** | |
| `horror_adventures` | 74 | 65 | 0 | **9** | |

**`inner_sea_bestiary` now reads `2 remaining monsters / 38 remaining abilities / 26 orphans /
7 PI`, i.e. 7 classifier-reachable and 0 REAL.** It is the third book in `§57.7`'s taxonomy —
reachable-exhausted but not monster-exhausted — and its 40 remaining units are lane floor, not queued
work. **Still eleven books holding 716 orphan abilities and zero remaining monsters**, re-derived
this round rather than incremented.

### 58.6 Three gate stages this round's own ingest turned red, and none of them was environmental

`./scripts/verify.sh` run 1 came back **12 of 15 green** with `pi-sweep`, `root-full` and `clippy`
red. Each is a different class and each is worth naming, because "the gate went red on an ingest
round" is exactly the shape a lane learns to wave through.

**(a) `pi-sweep` — a comment explaining a Product Identity removal named the term.** Two doc comments
this round wrote, one in `rules_tables::inner_sea_bestiary` and one on `RuleSetId::Isb`, spelled the
deity's name while stating *why the records carrying it were dropped*. `§52.5` recorded exactly this
("a comment recording a FALSE positive instantiates the name as surely as one recording a removal")
and this round re-paid it anyway, in the very module documenting the removal. `pi-sweep` rejects a
term anywhere under `rules_tables/` and does not read intent. Both rewritten to name the **screen**
(`pi_screening::PI_BLACKLIST_TERMS`) rather than the term; the corpus line numbers already pinned in
`the_product_identity_rows_are_not_records` were always the better identifier.

**(b) `root-full` — one test of 6,288: an SD-30 roster assertion this book's ingest flipped.**
`sd30_campaign_setting_books_appear_in_the_inventory_as_not_started_books` asserts
`inner_sea_bestiary` is registered `future_state`; it is `in_scope` now. Closed the way `§47.3` ruled
and three lanes have closed it before — the book joins
`SD29_INGESTED_CAMPAIGN_SETTING_BOOKS` as its own stated claim with the partial-ingest arithmetic in
the comment, rather than the roster being relaxed or the book dropped. **This is the fourth time a
lane has hit this test**, which is itself the finding: the assertion is correct and the cost is one
line per book, but nothing tells an ingesting lane it exists until the gate does.

**(c) `clippy` — `root: 55 warnings exceeds recorded ceiling 54`.** `identity_op` on
`230 - 26 - 7 - 0`, where the `- 0` was the classifier's `.COPY=` term written out to keep the
four-term arithmetic legible against the receipt. Carried as a comment now.

**All three were fixed and pushed the moment they existed rather than held to cycle end**, because
`378b7b70` was already on `origin/tranche/9` where (a) and (c) turn a **concurrent** lane's gate red
through no fault of its own — the 22-minute cost `§52.5` records and the mitigation `§57.5(b)`
states. Each was verified individually before the gate was re-run, rather than re-running the whole
gate hopefully: `cargo test --locked --test pi_table_sweep` → 6 passed;
`cargo test --locked --test v06_work_inventory sd30_campaign_setting_books` → 1 passed;
`cargo clippy --locked --tests -j 2 | grep -c inner_sea_bestiary` → 0.

**Run 2: `VERIFY_EXIT=0`, 14 of 14 green** — `root-full` 6,289 passed across 544 suites with all 525
`tests/*.rs` suites executed, `reach` 27, `clippy` root:54 back at the ceiling. No stage failed twice
with the same attribution, so `§39`'s recurrence rule is not engaged and nothing was accepted as
environmental.
## Decision 59 — Companion Lane, extend: round 5 (2026-08-12, `sd29-companion-r9`, card `epic-7-companion-lane-extend`)

Round 5 took `bestiary_4`, the cleanest book left by the ranking `§56` published (80 units, 5
orphans, 6% orphan share) and the one `§56.3` had named as the first book that would actually
exercise `§50`'s orphan-drop disposition.

**It did not exercise it. For the THIRD consecutive round the orphan instrument was found to be
UNDER-claiming, and all five orphans turned out to be owned.** The book landed with **78 of its 80
units grounded** — its whole reachable remainder — with **zero orphans** dropped.

The two exclusions are a different thing entirely, and they are the round's second finding: they are
`.COPY=` **delta rows**, which the companion chassis had never screened for and which the monster
lane has screened for since Bestiary 2. That screen moves the lane's ceiling **down** for the first
time in four rounds.

Net: ceiling **937 → 923**. Shape 6 recovered **+15**; the delta screen removed **−29**.

### 58.1 Ownership shape 6: the owner can be stated across a row that is not a unit

`b4_races_companion.lst:22` is `Familiar (Giant Flea)`. It does **not** name
`Flea (Giant) ~ Disease`. What it carries is:

```
ABILITY:Internal|AUTOMATIC|Bite|Racial Traits ~ Flea (Giant)
```

and `b4_abilities_companion.lst:56` is:

```
Racial Traits ~ Flea (Giant)   CATEGORY:Internal
ABILITY:Special Ability|AUTOMATIC|Flea (Giant) ~ Disease|Flea (Giant) ~ Uncanny Leap|Immunity to Disease
```

The ownership is stated twice over, in the corpus, in plain tokens. Shape 4 (granted-by) is exactly
this closure — and it cannot see it, because **the middle row is not an inventory unit**.
`v06_work_inventory` does not count `CATEGORY:Internal` rows, so `Racial Traits ~ Flea (Giant)` is
absent from the `abilities` list shape 4 walks, and the chain has nothing to stand on.

**Shape 6, relay:** a non-unit row of this book's ability `.lst` files, reached from a creature row,
propagates its own `ABILITY:Special Ability|AUTOMATIC|` grants to the units it names. Two details are
load-bearing:

* **The first hop is read under ANY `ABILITY:<Category>|AUTOMATIC|` category.** The creature's token
  here says `Internal`, not `Special Ability`; PCGen's category segment names the category of the
  keys that follow. Shape 1 keeps its narrower `Special Ability` predicate for the unit-to-unit links
  it already governs — the widening applies only to resolving relays, so nothing that was a
  non-owner becomes an owner by a looser read of an existing link.
* **The relay is never emitted.** It is not a unit, so it has no record to be emitted as, and
  inventing one would put a row on the wire `docs/work-inventory.json` does not count. The grant is
  attributed to the CREATURE that reaches the relay, because `companion_chassis`'s both-directions
  link test types `owners` as creature keys.

It cannot manufacture reachability: a relay is reached only from a creature row of this book, and a
reached relay grants only what its own token names. A relay nothing reaches grants nothing.

`Familiar (Pipefox)` and `Familiar (Ratling)` reach the three `~ Constant` rows of
`b4_abilities_race_ce_companion.lst` by the same two hops. Those five rows — the whole ORPHAN list
the classifier printed for this book — are pinned by name in
`bestiary_4::companion_tests::the_five_relay_owned_rows_have_their_relay_owner`.

**The token was already being read.** `transcribe_companion_tables.parse_natural_attacks` has parsed
`ABILITY:Internal|AUTOMATIC|` since round 1, to pick up attack names, and explicitly skips entries
containing ` ~ `. Those skipped entries are precisely the relays. The lane had the data in hand for
five rounds and was throwing away the half of it that answered the ownership question.

Corpus-wide, shape 6 recovers **15** units: 5 in `bestiary_4` and 10 in `core_essentials`
(26 orphans → 16). It changes **no** already-registered book: all nine were regenerated and
`git status --porcelain` listed not one of their `companion_data.rs` files. `bestiary_3` reports
`relay 5`, but those five rows were already owned through shapes 3 and 5, and `owners` is an
append-if-absent list — so the finding is additive at the corpus level and inert at the record level,
which is the strongest form the claim can take.

### 58.2 `.COPY=` delta rows, and a ceiling that was adding its exclusions instead of unioning them

`gen_book_cache` refused the book on its first run:

```
b4_abilities_companion.lst:99 names "CATEGORY=Special Ability|Change Shape.COPY=Pooka ~ Change Shape",
not "Pooka ~ Change Shape" -- the table's recorded line is stale and must be re-transcribed
```

`verified_citation_line` caught it, which is what that check is for. `bestiary_4` is the first
companion book carrying `.COPY=` rows, and the disposition is not a judgement call: the monster lane
ruled it at `transcribe_monster_tables`'s `.COPY=` screen and the reasoning transfers unchanged. A
`<Base>.COPY=<Variant>` row states a **delta** on a base record that lives elsewhere. Transcribed
verbatim — all this program does — `Pooka ~ Change Shape` yields a record with an `ASPECT` and
nothing else: no `TYPE:`, no `DESC:`, no page. That is the blank card
`docs/governance/no-stub-mvp-doctrine.md` forbids. Resolving the delta is not transcription; it
composes values across two rows while `CompanionAbilityRecord` carries ONE `source_file`/
`source_line` pair, so every inherited field would ship under a citation that does not contain it —
the exact stale-citation defect `verified_citation_line` exists to catch.

So the companion transcriber now screens `origin in ("copy", "mod_only")` and drops those rows,
scrubbing them from their owners' `ability_keys` so the chassis link stays closed in both directions.
The `mod_only` half is **stated, not exercised**: no book registered through round 5 carries one
(`core_essentials` 4 and `ultimate_wilderness` 1 are where it will first bite). Same discipline
`§56.3` used for the disposition it built and did not need.

**The ceiling was also being computed wrong, and this is the fix `§54.2` half-made.** `§51.1` ruled
that a ceiling subtracting one exclusion is not a ceiling; `§54.2` moved the class-row subtraction
out of prose and into the instrument. Both left the arithmetic a **sum**, which is only correct while
the exclusion sets are disjoint. They are not: corpus-wide there are 735 orphans + 2 `PRECAMPAIGN`
rows + 7 class rows + 30 delta rows = 774, but exactly **one** row is both an orphan and a delta, so
**773** distinct rows are excluded. The classifier now reports the union and derives the remainder
from it.

Re-derived 2026-08-12 (`python3 scripts/classify_companion_rows.py`):

```
total companion units in scope : 1696
orphan ability rows            : 735
PRECAMPAIGN-gated on an uningested campaign : 2
`*_classes_companion.lst` class rows the chassis refuses : 7
`.COPY=`/`.MOD` delta rows the chassis refuses : 30
distinct excluded rows (the UNION, not the sum) : 773
reachable remainder            : 923
```

The two derivations still close exactly, which is the check that has caught a bad ceiling table in
each of the last two rounds: the nine grounded books' reachable counts sum to **279** — the grounded
count before this round, to the unit — and the eight remaining books' sum to **644**, and
`279 + 644 = 923`.

### 59.3 A generated table that differed run to run, and the coin flip was shipping a creature with no abilities

Found while renumbering this decision from §58 to §59 after a merge: regenerating
`bestiary_4/companion_data.rs` from an inventory proven byte-identical for this book produced a
DIFFERENT file. Three runs in three processes, diffed:

```
- owners: &["Nycar", "Familiar (Nycar)"],
+ owners: &["Familiar (Nycar)", "Nycar"],
```

The cause predates this round and has been latent since ownership shape 3 was written:

```python
creature_keys = {u["corpus_key"] for u in creatures}     # a SET
creature_species = {bare_species(k): k for k in creature_keys}
```

`bare_species` maps `Familiar (Almiraj)` to `Almiraj`, and **Bestiary 4 is the first book that ships
`Almiraj` AND `Familiar (Almiraj)` as separate creature rows** — seven such pairs (`Almiraj`,
`Beheaded`, `Isitoq`, `Nycar`, `Pipefox`, `Pooka`, `Ratling`). Two keys collide on one map entry, and
which survived was decided by set iteration order, i.e. by Python's per-process randomized string
hash. **The whole lane's "regenerate, never hand-edit, and diff to prove it additive" method rests on
that generator being a function of its input. For one book it was not.**

**The non-determinism was the smaller half.** Even when the coin landed the same way twice, the answer
was wrong: `Nycar ~ Poison` is reached from `Nycar` AND from `Familiar (Nycar)` — the corpus states
both rows, and the catalog serves an ability under whichever creature the player is looking at.
Attributing it to one was never a decision. In the committed table the losing side shipped visibly
broken: the creature row `Beheaded` had `ability_keys: &[]` while `Familiar (Beheaded)` held all six
of the Beheaded variants. A player opening `Beheaded` saw a creature with no abilities at all.

`species_index()` replaces the comprehension: `<species>` -> EVERY creature row claiming it, in
creature row order. Deterministic, and it states what the corpus states.

* Three consecutive regenerations in three separate processes are now byte-identical.
* **The nine already-registered books regenerate byte-identical too** — no other book in the lane
  ships a bare species and its wrapper as two rows, so the collision is Bestiary 4's alone and this
  fix is a correction to one book, not a corpus-wide re-cut.
* `Beheaded` now carries its six abilities and every colliding ability names both owners.

**The check that caught it was not a test.** It was regenerating a generated file and diffing, which
is the same habit `§56.2` used to prove the multi-file spec additive and `§59.1` used to prove shape 6
inert on registered books. A test asserting "the table has 44 ability records" passes on both sides of
a coin flip. **Whenever a round changes a generator, regenerate TWICE and diff** — the second run
costs seconds and is the only thing that would have caught this.

### 59.4 What round 6 inherits

`companion` grounded **279 → 357**; the honest remainder is **923 − 357 = 566** across **7** books.
Raw `not-ingested` is 1,339, and that number is not the workload.

Per-book, every figure from this round's own classifier run, ranked by reachable share:

| book | units | excluded | **reachable** |
|---|---|---|---|
| `core_essentials` | 145 | 42 | **103** |
| `core_rulebook` | 170 | 86 | **84** |
| `ultimate_magic` | 170 | 138 | **32** |
| `advanced_race_guide` | 32 | 18 | **14** |
| `advanced_players_guide` | 212 | 208 | **4** |
| `book_of_the_damned_volume_1` | 31 | 29 | **2** |
| `ultimate_wilderness` | 575 | 248 | **327** |

`103 + 84 + 32 + 14 + 4 + 2 + 327 = 566`.

Three hazards, each derived rather than remembered:

* **`core_essentials` is the cheapest real book left and the one shape 6 most changed** (26 orphans →
  16). It carries **6** companion `.lst` files, **22** `.COPY=` rows and **4** `mod_only` rows — so
  it is the book that will first exercise the `mod_only` half of the screen this round built and did
  not need. It needs a new `RuleSetId`; nothing in `src/` compiles it today.
* **`ultimate_wilderness` is the largest block left at 327** and needs no new `RuleSetId`
  (`RuleSetId::Uw` exists, SD-28 Epic 26). It carries 1 `.COPY=` and 1 `mod_only` row and **169**
  creature rows, more than every registered book combined.
* **`core_rulebook`, `ultimate_magic` and `book_of_the_damned_volume_1` carry the 7
  `*_classes_companion.lst` class rows the chassis refuses outright.** The transcriber raises on them
  by name rather than dropping them silently; that is unchanged and still correct.

**`advanced_players_guide` (4 reachable of 212) and `book_of_the_damned_volume_1` (2 of 31) are
effectively floors, not queued work** — the same reachable-exhausted shape `§57` recorded for
`bestiary_4`'s monster half. A round that takes either is paying a full book's registration cost for
a handful of records, and should say so in its receipt rather than discover it at the ceiling table.

## Decision 61 — Companion Lane, extend: round 6 (2026-08-12, `sd29-companion-r10`, card `epic-7-companion-lane-extend`)

> **Section number claimed at claim time, not at merge.** `§59` was the last written when this round
> started and the monster lane was in flight on the same branch (it pushed `e70d39fc`, Bestiary 1's
> monster chassis, while this round was building). If that lane also wrote a `§60`, this one WAS
> renumbered in the merge (the monster lane claimed §60 for its round 8, `0d9fb586`), and it is
> filed as **§61** and every reference below moves with it — the convention `§47`, `§49` and
> `§53` each recorded after paying for it.

**Ultimate Wilderness ingested — 327 of its 575 companion units (169 creature rows, 158 ability
rows), all 327 grounded, no new `RuleSetId`, and zero units of any other kind moved.** Companion
grounded **357 → 684**. It is the largest single block in the lane and the largest companion book in
the corpus: its 169 creature rows are more than every previously registered companion book combined
(166).

Every figure below is followed by the command that produced it, and each was re-derived on this
tree rather than transcribed from `§59.4`.

### 60.0 The dispatch brief was materially stale for the THIRD consecutive round

The brief said **"NOTHING has landed"**, that "all ~1,233 in-scope companion units are not-ingested,
0 grounded", that the lane is "a NEW MECHANISM with no corpus-wide precedent", and that this round
should "build the mechanism on a small pilot first" against a pinned `inner_sea_combat`. `§56 §0`
and `§59 §0` each record the *same* brief text one and two rounds earlier. Re-derived before
anything was built:

```
git log --oneline -3 origin/tranche/9
  -> e478cd15 chore(retro): companion round 5 cycle-end reclaim event
  -> 997ad0c4 docs(sd29): companion round 5 — VERIFY_EXIT=0 on the merged tree …
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json'));
  print(collections.Counter(u['status'] for u in d['units'] if u['kind']=='companion'))"
  -> Counter({'not-ingested': 1339, 'grounded': 357})
```

Five rounds had landed and ten books were registered. The card's real state in `kanban.md` was
**`READY (round 6)`**. The brief's one checkable figure — **566** remaining — was re-derived and
**reproduced EXACTLY** before this round's work superseded it (§61.5).

**The worktree was again cut from the wrong base.** `HEAD` was `7d9f1c4f`, which has no
`docs/release/` directory at all and is not an ancestor of `origin/tranche/9`; every required read
named by the brief was absent from the checkout. Fixed with `git fetch origin tranche/9 && git reset
--hard e478cd15` before any work. This is the *third* consecutive round handed that base
(`§56 §0`, `§59 §0`), so it is a standing dispatch defect rather than an accident.

### 60.1 A row can state its rules text once per condition, and this lane had been refusing the shape

`transcribe_companion_tables.parse_desc` refused any row carrying several `DESC:` tokens not
resolvable by PCGen's `PRERULE:1,DisplayFullAbility` gate: *"the transcriber refuses to pick one by
position. Widen it deliberately."* That refusal had never fired, because no candidate book carried
the shape. Ultimate Wilderness carries it 22 times, and refusing would have cost the book:

```
python3 - <<'PY'   # over this book's own inventory units, this round
multi rows: 22
Counter({'PREVARGTEQ': 36, 'PREVARLT': 12, 'PREALIGN': 5})
PY
```

The rows are `Poison`, `Constrict`, `Breath Weapon`, `Spray`, `Camouflage`, `Saber-Toothed Bite` —
the abilities that make a companion a companion. `Spitting Cobra ~ Poison` states its effect twice:
*blurred vision* below `PREVARLT:CompanionAdvancement,1` and *blindness* at or above it. Picking one
ships the wrong rules text to every character on the other side of the gate; dropping the row ships
a creature card whose abilities have no text.

**So all of them are carried, none is evaluated, and each keeps its own gate verbatim.**
`CompanionAbilityRecord::description_variants` is a list of
`CompanionDescriptionVariant { text, variables, conditions }` in row order. `description` stays the
row's single UNGATED token when it has exactly one — so every previously shipped record is
byte-identical — and is `None` when every token is conditional, which is the honest state for a row
that states no unconditional text. All 8 of the shipped Ultimate Wilderness rows are that second
shape.

**Each variant keeps ITS OWN `%N` argument list, not the row's**, which is the detail a flatter
model would have lost: `Spitting Cobra ~ Poison`'s two tokens carry `10+HD/2+CON` and
`10+HD/2+CON.` — the same formula with a stray full stop — and a single shared `description_variables`
field would have had to pick one.

The gate is rendered into prose on the wire by `companion_catalog::serve_desc_condition` over a
**closed set** — `PREVARGTEQ`, `PREVARLT`, `PREALIGN`, the three the book's rows actually carry —
and panics on anything else, so the next book's new gate kind surfaces instead of reaching a player
as a raw PCGen token. Variable names are split mechanically from camel case (`MasterLevel` → *master
level*); the nine alignment codes are a table, because `TN` split mechanically reads *"t n"*.

**8 shipped, 22 in the file, and the two numbers are the finding rather than a discrepancy.** The
other 14 multi-`DESC:` rows are archetype rows this chassis drops (§61.2). A test pinned to 22 would
be asserting a fact about a `.lst` file; the chassis test pins 8.

### 60.2 The first book in this lane whose shortfall is bigger than its ingest — and it is a different KIND

248 of Ultimate Wilderness's 575 units do not ship. Every earlier registered book had ZERO orphans
(`bestiary_3`, `bestiary_4` and `bestiary` each opened with orphans on the board and each found them
owned, `§54.1`/`§56.1`/`§59.1`). This one has 247, plus 1 delta row that is also an orphan and 2
delta rows in total:

```
python3 scripts/classify_companion_rows.py ultimate_wilderness | tail -6
  total companion units in scope : 575
  orphan ability rows            : 247
  `.COPY=`/`.MOD` delta rows the chassis refuses : 2
  distinct excluded rows (the UNION, not the sum) : 248
  reachable remainder            : 327
```

**`§45.1` as amended by `§56.1` was applied — the rows the classifier was about to throw away were
read before committing — and for the first time in four rounds it did NOT move the ceiling up.**
What it found instead is that the orphans are structured, and the structure says they are not
companion creatures' abilities at all:

```
python3 - <<'PY'   # the 247 orphans grouped by their key's namespace prefix
   39  Animal Trick            33  Animal Companion Feat
   16  Companion Archetype     14  Familiar Archetype
   12  Draconic Companion       7  Infiltrator / Mascot / Prankster / Valet  (7 each)
   …
PY
awk -F'\t' '/CATEGORY:Archetype/{…print KEY…}' uw_abilities_companion.lst | wc -l   -> 30
```

* **30** of the orphans ARE the archetype rows (`KEY:Familiar Archetype ~ Valet`,
  `KEY:Companion Archetype ~ Draconic Companion`), carried in the same file under
  `CATEGORY:Archetype`.
* **119** more are ability rows namespaced under those archetypes' DISPLAY names
  (`Valet ~ Deliver Aid`, `Draconic Companion ~ Breath Weapon`). This is ownership shape 5 exactly —
  except the owner is an archetype, not a creature.
* **72** are the generic option groups `Animal Trick ~ …` (39) and `Animal Companion Feat ~ …` (33),
  which attach to ANY animal companion rather than to one creature.

**That is a real ownership relation the corpus states, and this round deliberately did not take
it.** `CompanionRecord` is a creature: `SIZE:`, `MOVE:`, `MONSTERCLASS:`, natural attacks. An
archetype has none of those, and `CompanionCatalogScreen` has no section that would show one.
Widening shape 3/5 to accept an archetype owner would have made 149 rows "reachable" in the
classifier and shipped them under a creature they do not belong to — the stub class `§44.2`
describes, arriving by the exact route `§45.1` exists to prevent.

**They are also NOT a `reach_gate` `OPEN_FINDINGS` entry, and the transcriber's own generated
boilerplate had been claiming otherwise since round 4.** That list is keyed by (book, FAMILY) and
`unsurfaced_families_are_exactly_the_recorded_findings` fails an entry naming a family that DOES
reach a player — which `ultimate_wilderness/companions` does. A dropped row is also not an ingested
record, so it is outside the gate's denominator entirely. The sentence was never checkable while
every registered book had zero orphans; the first book with orphans is the first book that could
falsify it. Corrected at the source (the generator), so no future book ships the false claim.

The shortfall is counted where it is real: those 248 rows keep their honest `not-ingested` status in
`docs/work-inventory.json`, and `ultimate_wilderness/mod.rs` names the shape row by row.

### 60.3 `%%N` — a renderer and a guard that had contradicted each other for the whole program

`gen_book_cache` shipped the book, and the desktop crate then panicked on one record:

```
companion ability "Seaweed Leshy ~ Water Jet": rendered description still carries
unsubstituted '%N' argument reference. Raw token: "… must make a DC %%1 Fortitude save …"
```

`pcgen_desc::render_pcgen_desc_with_values` documented `%%` as *"never an argument reference, and
`%%1` would otherwise be misread as one"*, so it emitted a literal `%` followed by `1`.
`leaked_pcgen_syntax` then rejects `%1` as PCGen syntax on a player's screen. **Both are shipped
code and they cannot both be right.** Nothing had caught it because no ingested record carried the
shape:

```
grep -rl '%%[0-9]' --include='*.lst' ~/workspace/repos/pcgen/data/pathfinder/paizo/
  bestiary_3/b3_abilities_race.lst
  ultimate_wilderness/uw_abilities_companion.lst
  player_companion/familiar_folio/ff_abilities_race.lst          (4 tokens in all)
grep -rl '%%[0-9]' data/corpus/ | wc -l   -> 0   (before this round)
```

All four are the same sentence — `… must make a DC %%1 Fortitude save …|<DC variable>` — and each
row's argument list supplies exactly the argument the doubled escape is hiding. It is an upstream
escaping typo, and read as an escape the argument has no referent at all.

**The narrow reading ships:** `%%N` is an argument reference **only when argument N exists**;
otherwise `%%` stays a literal per cent sign. `20%% spell failure chance` is untouched, and so is a
hypothetical `20%%1 chance` with no argument tail. This required a change in TWO places and the
second is the one that matters — `max_arg_reference` skipped past `%%` without counting the digit,
so "does argument N exist" was decided by a function that had already discarded the question, and
the new branch in the renderer was unreachable until it stopped doing that. **The test asserting the
render was RED for exactly that reason before it was green**, which is the only reason the second
half was found.

**A second, quieter defect surfaced with it:** `companion_catalog::serve_ability_description` was
handing the renderer the `DESC:` PROSE ALONE, because the transcriber splits a token into
`description` and `description_variables` and the wire only ever read the first. For every `%N` in
every registered book that made no difference — none of their arguments is an integer literal
(`grep -rho 'description_variables: &\[[^]]*\]' src/rules_core/rules_tables/*/companion_data.rs`
returns formulas only), so the placeholder is dropped either way and the rendered text is
byte-identical. For `%%1` it made all the difference. The two halves are now rejoined before
rendering.

The one record's served text is `"… must make a DC Fortitude save …"` — the unresolvable formula
dropped, never guessed, exactly as `bestiary_2/monster_ability/aeon_aging_strike.json` has rendered
the same shape since SD-29 Epic 5. And the literal per cent one clause earlier
(`[20% miss chance]`) survives intact, which is the assertion that proves the reading is narrow
rather than a blanket rewrite.

### 60.4 `SpecialQuaility` — a typoed `TYPE:` segment in 15 rows, recorded rather than laundered

15 of the shipped 158 ability rows carry `TYPE:SpecialQuaility…` — one transposition away from the
`SpecialQuality` this chassis models — so `read_facet_and_delivery` leaves their `facet` as `None`.
That is 15 of the corpus-wide 20 unmodelled-facet records, and 121 of the 121 wire rows behind them
(the catalog nests an ability under every owning creature, and these are shared racial traits).

**Not corrected into the modelled facet, deliberately.** The transcriber's contract is that every
emitted value is a substring of the cited row; mapping a misspelling onto an enum variant is
inference, and the moment it is done silently the corpus's own spelling stops being visible to
anyone. `type_segments` carries `SpecialQuaility` verbatim to the screen, so a reader sees the
book's text and can act on it. Both the chassis test and the wire test pin the count AND the
spelling, so a successor that decides to model it must delete an assertion deliberately rather than
discover a mystery failure.

### 60.5 Denominators, every one re-derived this round

```
python3 scripts/classify_companion_rows.py | tail -7
  total companion units in scope : 1696
  orphan ability rows            : 735
  PRECAMPAIGN-gated on an uningested campaign : 2
  `*_classes_companion.lst` class rows the chassis refuses : 7
  `.COPY=`/`.MOD` delta rows the chassis refuses : 30
  distinct excluded rows (the UNION, not the sum) : 773
  reachable remainder            : 923

python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json'));
  print(collections.Counter(u['status'] for u in d['units'] if u['kind']=='companion'))"
  -> Counter({'not-ingested': 1012, 'grounded': 684})
```

The ceiling is **unchanged at 923** — the first round in four that did not move it, and §61.2 is
why. **Honest remainder `923 − 684` = 239** across **6** books. Raw `not-ingested` is 1,012 and that
is NOT the workload.

The two derivations close exactly, the check that caught a bad ceiling table in three of the last
four rounds:

```
python3 scripts/classify_companion_rows.py core_essentials core_rulebook ultimate_magic \
        advanced_race_guide advanced_players_guide book_of_the_damned_volume_1 | tail -2
  distinct excluded rows (the UNION, not the sum) : 521
  reachable remainder            : 239
```

760 units across the six, 521 excluded, **239 reachable** — and `923 − 684 = 239` from the other
direction.

**A `PRECAMPAIGN` gate this round checked and did NOT count.** `_ultimate_wilderness.pcc:92` loads
`support/uw_abilities_companion_pu.lst` under `PRECAMPAIGN:1,INCLUDES=Pathfinder Unchained`, and the
classifier's `UNINGESTED_CAMPAIGN_GATES` names only `Occult Adventures`. That is correct here rather
than a gap: `pathfinder_unchained` IS ingested (`ls data/corpus/pathfinder_unchained`,
`RuleSetId::Pu` at `rules_tables/mod.rs:55`), so its 17 rows are in scope — and all 17 are orphans
anyway, which is why the book's spec names one abilities file rather than two. The book's five
`support/uw_races_companion_{arg,b3,b4,b5,b6}.lst` files are `.MOD` overlays and are not inventory
units at all.

### 60.6 Round 7's queue

| book | units | excluded | **reachable** |
|---|---|---|---|
| `core_essentials` | 145 | 42 | **103** |
| `core_rulebook` | 170 | 86 | **84** |
| `ultimate_magic` | 170 | 138 | **32** |
| `advanced_race_guide` | 32 | 18 | **14** |
| `advanced_players_guide` | 212 | 208 | **4** |
| `book_of_the_damned_volume_1` | 31 | 29 | **2** |

`103 + 84 + 32 + 14 + 4 + 2 = 239`.

* **`core_essentials` (103) is the largest and the cheapest real book left.** It carries 6 companion
  `.lst` files, 22 `.COPY=` rows and 4 `mod_only` rows — the first book that will exercise the
  `mod_only` half of `§59.2`'s delta screen, which is **stated, not exercised** to this day. It
  needs a NEW `RuleSetId`; nothing in `src/` compiles it (`grep -n 'Ce,' rules_tables/mod.rs` before
  writing one — the race-trait lane added `RuleSetId::Ce` in `§49`, so **check, do not assume**).
* `core_rulebook`, `ultimate_magic` and `book_of_the_damned_volume_1` carry the 7
  `*_classes_companion.lst` class rows the chassis refuses outright, by name rather than silently.
* **`advanced_players_guide` (4 of 212) and `book_of_the_damned_volume_1` (2 of 31) are FLOORS, not
  queued work** — unchanged from `§59.4`, and still true.

**The archetype block is the biggest single thing this lane now knows about and cannot take**: 149
Ultimate Wilderness rows plus whatever the other five books carry of the same shape. It is a NEW
RECORD TYPE (`CompanionArchetypeRecord`) plus a screen section, not a wider ownership predicate, and
a round that takes it should say so up front rather than discover it at the ceiling table.
