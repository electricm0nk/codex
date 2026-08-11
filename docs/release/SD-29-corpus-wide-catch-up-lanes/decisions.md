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
