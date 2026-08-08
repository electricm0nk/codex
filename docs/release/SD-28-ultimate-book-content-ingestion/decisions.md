# SD-28 Decisions

**Status:** Operator-pinned, confirmed 2026-08-01 (Decisions 13–30).

## Decision 1 — Book list

**Status:** Pending operator confirmation.

**Candidate:** Ultimate Combat + Ultimate Magic + Ultimate Equipment + Ultimate Intrigue.

**Source:** Operator message 2026-07-28 ("all the ultimate books for 28"). Honcho context minimum: the four Paizo hardcover books in the "Ultimate" line.

**Operator-pinned values needed when reviewing on a real computer:**

- Confirm the four books.
- Confirm the per-book path locations under `src/rules_core/rules_tables/`.
- Confirm the per-book ingest subtype (per-class / per-monster-block / per-equipment-entry).

## Decision 2 — Branch and board

**Status:** Pending operator confirmation.

**Candidate:** `tranche/8` branch + `codex-tranche-8` board.


## Decision 3 — Build version target

**Status:** Pending operator confirmation.

**Candidate:** `0.6.<build>` first concrete value.

**Rationale:** Per the `<major>.<tranche-base>.<build>` scheme (SD-21 / SD-22 doctrine-of-record). tranche-base = 6 for `tranche/6`. Major stays `0` until first main-publish. Build is the monotonic counter from the current build-counter state.

**Operator-pinned values needed:**

- Confirm the current build counter value (read from the version-bump contract in the repo's release workflow).

## Decision 4 — Epic structure

**Status:** Doctrine-of-record (per SD-22 doctrine).

8 epics / 30 criteria (superseded: 12 epics / ~36 criteria per epic-breakdown.md — see Decision 26). Epic 1 = Code-Side Identifier Cleanup. Epic 2 = Operator Pre-Launch. Epic N = Closure Epilogue.

## Decision 5 — Cross-bundle class overlap

**Status:** Doctrine-of-record (per SD-22 doctrine).

For classes that appear in both Ultimate Intrigue and Occult Adventures (Occultist, Spiritualist, Medium, Mesmerist), the canonical class definition lives in whichever bundle owns the book's primary class definition. SD-30 (Occult Adventures) owns the canonical class id; SD-28 references the canonical class id from SD-30's progress file but does not redefine.

## Decision 6 — Identifier discipline

**Status:** Doctrine-of-record (per SD-22 doctrine).

- Source-code identifiers describe WHAT the artifact does, NOT which release / spec domain it came from.
- PascalCase for functions / methods / constants / properties / Tauri commands.
- lowercase camelCase for variables.
- Forbidden patterns: `sd28_*`, `SD28_*`, `Sd28*`, `sd28-*`, `t_<hex>`, `SD-28-Ex...`, `AV-PAY-N`.
- Doctrine-of-record at `~/workspace/governance/identifier-discipline.md`.

## Decision 7 — Operating form [SUPERSEDED — see §22]

**Status:** Doctrine-of-record (per SD-22 doctrine); **superseded 2026-08-01** by Decision §22, which replaces the dispatch mechanism named here with the `Workflow` tool.

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

**Open operator question this bundle cannot decide for itself.** SD-28's
epic structure contains no surface-building epic, and Ultimate Equipment is the
largest equipment book in the corpus while `equipment_catalog.rs` is still
CRB-only — a limitation already pinned in the gate's `OPEN_FINDINGS` for APG
and ACG equipment. Either the catalog widening lands inside SD-28, or it is a
named prerequisite outside it. **The operator picks; this package does not add
an epic on its own authority.** What is not available is skipping it: the gate
fails the cycle either way.

**Authority:** `apps/desktop/src-tauri/src/reach_gate.rs`,
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
strings still claim engines do not exist that do. The inventory is generated
from the corpus and cross-referenced against the engine, and is idempotent by
contract.

**Authority:** `src/bin/v06_work_inventory.rs`, `docs/work-inventory.json`,
`docs/governance/book-ingestion-playbook.md` §6.

## Decision 12 — Build no execution engines [SUPERSEDED — see §18]

**Status:** Doctrine-of-record (scoping verdict, 2026-07-29); **superseded 2026-08-01** by Decision §18, which tightens the rule.

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
both. §18 narrows the rule to forbid only the first kind while permitting the
second.

## Decision 13 — Book list confirmed (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.**

**Decision:** SD-28's book list is seven books:

1. **Ultimate Combat** — Paizo hardcover, 2011-08-01. Per-class cycles (Gunslinger, Ninja, Samurai, etc.).
2. **Ultimate Magic** — Paizo hardcover, 2011. Per-class + per-spell-subsystem cycles.
3. **Ultimate Equipment** — Paizo hardcover, 2012. Per-equipment-entry cycles.
4. **Ultimate Intrigue** — Paizo hardcover, date-of-record unclear (per operator 2026-08-01: "date not found"). Per-class + per-social-rule cycles.
5. **Ultimate Campaign** — Paizo hardcover, 2013. Player-options subsystems (downtime, kingdom-building, traits, retraining).
6. **Ultimate Wilderness** — Paizo hardcover, date-of-record unclear (per operator 2026-08-01: "date not found"). Per-class + per-Companion-rules cycles.
7. **Ultimate Psionics** — Dreamscarred Press hardcover, 2014. Third-party tier; conditional on licensing-conformance per Decision §17.

**Path locations:**
- `src/rules_core/rules_tables/ultimate_combat/` (Paizo)
- `src/rules_core/rules_tables/ultimate_magic/` (Paizo)
- `src/rules_core/rules_tables/ultimate_equipment/` (Paizo)
- `src/rules_core/rules_tables/ultimate_intrigue/` (Paizo)
- `src/rules_core/rules_tables/ultimate_campaign/` (Paizo)
- `src/rules_core/rules_tables/ultimate_wilderness/` (Paizo)
- `src/rules_core/rules_tables/ultimate_psionics/` (Dreamscarred Press)

**Corpus dirs:**
- `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_{combat,magic,equipment,intrigue,campaign,wilderness}/` — six Paizo dirs, all confirmed present 2026-07-30.
- `~/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/ultimate_psionics/` — Dreamscarred Press dir, confirmed present 2026-08-01; licensee of record under PF-OGL-compatible license (see §17).

**Ingest subtype per book** mirrors SD-22's pattern: per-class cycles for books with class content, per-monster-block for books with monster appendices, per-equipment-entry for UE.

## Decision 14 — Branch and board (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.**

**Decision:** SD-28 launches on `tranche/8` branch with `codex-tranche-8` board.

The prior candidate (per the 2026-07-28 stub) was `tranche/6` + `codex-tranche-6`; the operator moved SD-28 off the `tranche/6` family on 2026-08-01 so SD-29 (`tranche/6-1`) and SD-30 (`tranche/6-2`) can keep their dash-form sub-release positions unaltered. SD-28 takes its own tranche.

**Operator-on-file override.** The published SD-22 / SD-27 chassis templates use `codex-tranche-<N>` as the convention slug for the kanban-board name. SD-28 inherits the slug format (`codex-tranche-8`) and the corresponding Hermes-board-instance identifier — see Decision §15a below for the Hermes-board status. Board identity wording superseded by §15a: `codex-tranche-8` is a slug for the local `kanban.md` queue, not a Hermes board instance.

## Decision 15 — Build version target (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.**

**Decision:** SD-28's first concrete build value is `0.8.<build>`, where `<build>` is the current build-counter state at the time of cycle close.

Per the 2026-07-17 build-version amendment (doctrine-of-record at SD-21 Decision §5 / SD-22 / SD-27):
- **major** = 0 (no main-publish yet; first main-publish may move this to 1).
- **tranche-base** = 8 (the base digit of `tranche/8`, per the 2026-07-17 directive that tranche-base is *the base digit of the active working tranche*, not an increment counter).
- **build** = monotonic counter, never resets; first concrete value reads the current build counter (recorded in the repo's release workflow), increments per CI build.

**Closure Epilogue (Epic N — see §6 below):** tranche-promotion version increments the tranche position only when SD-28 promotes to `develop` (e.g. `0.8.<last_build>` remains the post-closure value; the next bundle may bump to `0.9.<build>` if its tranche-base is 9).

## Decision 15a — Hermes board retired (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.** Cross-cutting — affects Decision §7 (operating form), §14 above, and the loop-instruction pre-launch checklist.

**Decision:** The `codex-tranche-8` board is NOT a Hermes board. SD-28's pre-launch checklist no longer requires a Hermes-board instance; the work-queue artifact is a local-file `kanban.md` paired with `progress.md` (per the operator 2026-08-01 confirmation "we will stop using the hermes board" + "kanban.md + progress.md for the work-queue artifact").

**Operator-on-file override.** The 2026-07-18 loop-instruction doctrine-of-record includes a `/loop 60m /batch /goal <loop-instruction-file>` operating form predicated on Hermes-board card dispatch. SD-28 **inherited** the `/loop /batch /goal` cadence with the dispatch made local-file only — the supervisor reads `kanban.md` at top of each cycle (rather than Hermes-board card state) and writes cycle receipts to `progress.md`. **This paragraph is superseded on dispatch mechanism by §22** (2026-08-01): the local-file state source (`kanban.md`/`progress.md`) stands, but the dispatcher is the `Workflow` tool driven from a live session, not `/loop /batch`. The file-touch partition discipline (1 cycle per file at a time) is unchanged by the mechanism swap.

## Decision 16 — Cross-book conflict rule (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.**

**Decision:** When two books in SD-28's scope (or between SD-28 and a closed/in-flight SD-N on the same tier or on adjacent tiers) conflict on a record (e.g., a feats reprint, a spell erratum, a class-feature revision), **the newer book is doctrine and the older book is errata.**

This supersedes any prior cross-book conflict handling in the bundle, including the SD-22-Derived §5 cross-bundle class overlap rule for class grants, which is narrower than this one:

- **Class-grant overlaps** (Ultimate Intrigue vs. Occult Adventures, etc.) follow the existing rule from §5: canonical class definition lives in the bundle that owns the book's primary class definition; the other bundle references the canonical id only. Decision §16 does not displace this for class grants.
- **Record-level overlaps** (reprints, errata, identical spells re-presented with wording changes) follow §16: newer book wins, older book is errata.

The load-bearing ramification is on the choices per above decision document: when a class or feature appears in both an Ultimate book and a non-Ultimate book (PAIZO only, third-party not included in this rule per §17), the newer book is doctrine.

**Authority:** operator verbatim 2026-08-01: "in the event of cross-book conflict, we treat the newer book as doctrine and the older book as errata."

## Decision 17 — Dreamscarred Press tier is in-bounds for licensing (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01** for the inclusion of `ultimate_psionics` (Dreamscarred Press tier) under SD-28.

**Decision:** SD-28 may reference Dreamscarred Press as a third-party publisher tier, gated on licensing-conformance verification. The Honcho duracon title `Dl-13 Dreamscarred Press Psionics is open content` records that Dreamscarred Press's Psionics line is open content under a PF-OGL-compatible license; the operator confirmed 2026-08-01 that this permits SD-28 to ingest `ultimate_psionics`.

**Pre-cycle verification.** Cycle 0 runs the trap-report against `dreamscarred_press/ultimate_psionics/` and confirms that all records' licensing annotations match the open-content tier. Any record that fails the licensing audit is dropped from the per-cycle scope (recorded as a cycle finding, not a blocker).

## Decision 17a — Bulk modifications deferred (operator directive 2026-08-01)

**Status:** Operator-pinned, **forward-leaning acknowledgement.**

**Decision:** The per-cycle mode of operation (one record-at-a-time, file-touch partition, individual cycle receipts) is preserved for SD-28. Bulk-modification tooling (a separate bundle or a retroactive pass across already-ingested data) is *not* in scope. The operator may authorize a bulk-modification retrofit outside SD-28 if/when needed; such retrofits do not retroactively modify the SD-28 decision record.

**Why recorded as a decision rather than as an item left out.** The operator's verbatim "correct for now, we will make bulk modifications later" implies a future surface — recording it as a decision gives future-author a pointer to the operator's stated posture without forcing a decision-record entry for every future modification.

## Decision 18 — Reach gate is the definition of done (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.** **Supersedes Decision §12 (the prior "Build no execution engines" rule)** by tightening the engine-forbidden zone.

**Decision:** A record's ingest cycle is **not done** until it satisfies `apps/desktop/src-tauri/src/reach_gate.rs`. Reach is the operator-visible definition of done.

**Engine policy.**

- **Real-time engines are out of scope.** No cycle in this bundle builds an RNG, opponent-state, or turn-sequencing engine. Decision §12's prohibition on these stands.
- **Rules-data engines are in scope and often unnecessary.** When a numerical effect can be pre-computed as data (e.g., a fireball that is `1d6 per level` for a caster level of 6 produces `6d6`), post the calculated value in the spell description; the player rolls the actual `6d6` at the table with physical dice ("math rocks"). This is rule-data representation, not an execution engine. No runtime die-rolling code is needed.
- **Engine construction is permitted only when strictly necessary to satisfy reach.** If a record's effect cannot be represented as data without an unjustifiable loss of fidelity (e.g., a feat with branching conditions that depend on per-roll state outside what pre-compute captures), the cycle may build a small rules engine to model it. The engine must be enumerable, testable, and observable from `reach_gate.rs`.

**What this changes.** §12's blanket "no engines" rule was too coarse: it forbade legitimate rules-data work. §18 narrows it to real-time engines. Reach remains the gate; this means reaching a player surface is mandatory, but reaching it via pre-computed values is preferred over reaching it via a runtime engine.

**Cross-bundle impact.** SD-27's `decisions.md §19.1` records a related conflict ("content-only scope vs. the reach gate"). §18 resolves it: reach is non-negotiable; content-only scope is honored by pre-computation as data, with engines produced only when strictly necessary. SD-27's `artifacts/cross-bundle-findings-2026-07-30.md` should reflect this resolution in its next revision.

**Authority:** operator verbatim 2026-08-01: "reach gate is the definition of done, if an engine is required to get there, then we generate the engine — that said, often an engine isn't strictly necessary. We do not need to manage actual dice rolls, merely represent the rules. So if, for example, a fireball is 1d6 per level, and the caster level is 6, you would post in the spell description the calculated value of 6d6. you do not need to provide an engine to roll the actual 6d6 — that happens on the table with physical dice, aka math rocks."

---

## Decision 19 — Operator ack-chain recorded (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01** as a forward-leaning ack chain. Items 4, 6, 8-11 in the operator's twelve-item directive were verbal acks of items I previously proposed in conversation; they do not introduce new directives.

**Ack ledger.** Item 4 = Epic 8 Build Version Numbering shape (now captured by §15). Item 6 = Closure Epilogue fires LAST (§4 doctrine-of-record). Items 8-11 = misc operator reviews of in-scope details (no record change required; the per-cycle tooling table and per-cycle tier model are already captured by the loop-instruction and decisions §11 respectively).

**Why an explicit ack-chain decision rather than dispersed prose.** Operators audit decision-records for what changed; if a future-author sees "Epic 8 build version: 0.8.<build>" without context, the chain back to "operator confirmed 2026-08-01" belongs alongside. Records the audit trail without inflating the audit-table-of-contents.

---

## Decision 20 — Cross-reference

- `./scope-draft.md` — committed scope shape, seven books confirmed.
- `./loop-instruction.md` — per-cycle procedure; updated for `tranche/8`, no-Hermes-board, local-file dispatch.
- `./forward-scope-register.md` — successor work depending on SD-28's output.
- `~/workspace/programs/codex/requirements/SD-22-.../decisions.md` — predecessor doctrine for the Per-cycle repo tooling (§11 here ≡ SD-22 §"Per-cycle tooling").
- `~/workspace/governance/identifier-discipline.md` — identifier-discipline reference (doctrine-of-record).
- `docs/governance/book-ingestion-playbook.md` — playbook of record for the per-cycle procedure.
- `apps/desktop/src-tauri/src/reach_gate.rs` — definition-of-done surface for §18.
- `~/workspace/governance/pcgen-licenses.md` (forthcoming) — licensing-conformance surface for §17.
- `~/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/ultimate_psionics/` — corpus surface for §17.

## Decision 21 — Unattended mode authorization (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.** Load-bearing for the bundle's cycle dispatch.

**Decision:** This bundle operates in **unattended mode**. The operator is out of town and may not see the harness's output for days. Cycles MUST NOT pause to ask the operator questions; the operator's review happens after return.

**Operating protocol during unattended mode (codified in `loop-instruction.md` §"OPERATING METHOD" sub-callout).**

1. **Default-and-flag, not ask.** When a cycle needs a decision, pick the safer default, capture it in `progress.md`, and continue.
2. **No `clarify` tool calls.** The operator clarification tool is hard-banned under unattended mode.
3. **Blockers are recorded, not raised.** Hard-blocks (auth failure, branch creation conflict, identity conflict on disk) go in `progress.md` with the command and exit code. The bundle does not halt; the supervisor picks up the next ready card.
4. **`decision-blocked` IS allowed.** Operator-decision points (Epic 7 closure, Epic 1 fire order, cross-bundle-class overlap resolution) record `decision-blocked` in `progress.md` and proceed on the safe default per `forward-scope-register.md C3.x` retrofits.
5. **Closure is a goal, not a stop signal.** The bundle runs to closure under the dispatcher's own loop (the `Workflow` tool per §22, not a human re-invoking a slash command per cycle).

**Operator's verbatim:** "include instructions to all 3 that indicate they will be running in unnattended mode since i will be out of town while this runs. They may not stop to ask questions - it might be days before i notice."

**Cross-reference:** the doctrine is mirrored in `loop-instruction.md` (cycle supervisor reads it first) and `progress.md` (per-cycle receipt confirms the operator-on-record). The receipt chain is the operator's after-return review surface.

## Decision 22 — Dispatch is session-driven `Workflow`-tool orchestration, not `/loop` or `/batch` (adopted from SD-27 `decisions.md §19`, 2026-08-01)

**Status:** Operator-pinned by inheritance — SD-27 `decisions.md §19` records the correction ("adopted from SD-26 `decisions.md §13`"); this package had not yet propagated it before this pass. No new operator input required; this is process alignment, not a new ruling.

**Decision:** SD-28 dispatches via the **in-harness `Workflow` tool, driven from a live session** — not `scripts/workflow-dispatch.sh` or any headless script, and not a cron driver. Deterministic control flow (per-epic ordering, fan-out, `decision-blocked` handling) is written into `loop-instruction.md` and tracked as state in `kanban.md`'s claim/complete queue; model judgment lives inside the dispatched `agent()`/`Workflow` calls, never in the orchestrating session's own tool calls.

`/batch` defaults to parallel isolated-worktree fan-out. It is used only where an epic's criteria are genuinely file-disjoint (see `loop-instruction.md` "Epic ordering"); where cycles touch shared state — `progress.md`, `kanban.md`, `reach_gate.rs`'s `OPEN_FINDINGS` — the correct dispatch is an explicit single-cycle procedure, not `/batch`. Any parallel wave that does run passes `isolation: 'worktree'` to every mutating agent (`docs/governance/loop-instruction-template.md §3`).

The orchestrating session never implements directly — it dispatches, verifies, and rules (`loop-instruction-template.md §2.2`). This held across SD-27's launch and the CRB run before it; nothing about SD-28's shape is an exception.

**Reasoning:** `loop-instruction.md`'s OPERATING METHOD callout (authored before this correction propagated) still named `/loop 60m /batch /goal ...` as the dispatch command. That form requires a human to re-type a slash command per invocation and cannot run headless — directly contradicting §21's unattended-mode authorization, which requires the bundle to run to closure across days with nobody watching. A `Workflow`-tool session, not a slash-command invocation, is what can actually satisfy that requirement.

**Consequence:** `loop-instruction.md`'s OPERATING METHOD callout now names the `Workflow` tool; §21 point 5 ("closure is a goal, not a stop signal") is corrected to read "under the dispatcher's own loop" rather than "per `/loop` cadence."

**Cross-reference:** `docs/release/SD-27-future-state-book-content-ingestion/decisions.md §19` (the adopted correction, itself from SD-26 `decisions.md §13`); `docs/governance/loop-instruction-template.md §2` (orchestration mode), `§2.1` (`RETRO_ACTOR`), `§2.2` (execution boundary), `§3` (worktree-isolation requirement for parallel waves).

## Decision 23 — A running retrospective log is part of the cycle procedure, not an optional courtesy (2026-08-01)

**Status:** Process alignment — the tooling already exists on `tranche/8` (`scripts/retro.py`, `docs/retro/schema.json`, `docs/retro/events/<actor>.jsonl`); this package had not yet wired emission into its own cycle procedure.

**Decision:** Every SD-28 cycle emits at least one retrospective event via `scripts/retro.py`. The event vocabulary (`correction`, `incident`, `near_miss`, `deferral`, `rework`, `verification`, `note`) and the field contract live in `docs/retro/schema.json` and are not re-specified here — read `python3 scripts/retro.py help <type>` for the real flags before emitting.

- Every dispatched agent has `RETRO_ACTOR` set to its role name (`loop-instruction.md` OPERATING METHOD callout, per `loop-instruction-template.md §2.1`). The harness has no variable that identifies an agent's role; the fallback (worktree directory name) names a checkout, not a role, which makes the by-actor breakdown in `scripts/retro.py summary` meaningless.
- `./scripts/verify.sh` auto-emits its own `verification` event on every run, passing or failing, so the denominator of "how often did we actually check" is honest without anyone deciding to record it.
- A `correction` event requires `--verified-by` — an unverified correction is a competing assertion, not a finding.
- Emitting the event is cycle step 8 (`loop-instruction.md` "Cycle mechanics"), not a follow-on task a cycle can skip under time pressure.

**Cross-reference:** `loop-instruction.md` §"Retrospective log"; `scripts/retro.py`'s own `--help` docstring (do not hand-roll the emission syntax from memory).

## Decision 24 — Stop vs. press on: when a cycle halts and when it doesn't (2026-08-01)

**Status:** New — codifies a rule this bundle's `loop-instruction.md` "Hard stops" section applied implicitly (via its concrete instances) but never stated generally.

**Decision:** A cycle STOPS (records `decision-blocked` per §21's unattended-mode protocol, does not fabricate a pass) when:

- A gate fails for a reason that is a real finding about content or scope. Never weaken, skip, `#[ignore]`, or exclude a gate to get green, and never invent a surface or a number to satisfy one.
- Two authorities disagree on scope.
- The work would revert or clobber another session's live work.
- Proceeding would require inventing data not present in the corpus.

A cycle PRESSES ON, without recording `decision-blocked`, when:

- This package's own stated figure or premise is wrong — correcting it is expected, not insubordination.
- The scope is larger than expected — size alone is never a stop reason.
- A mechanical defect needs fixing (duplicate module, stale fixture, lint fix).
- A routine judgment call has a conventional default — pick it, record it, move on.

**Reasoning:** Under §21's unattended-mode authorization, the cost of stopping on the wrong things and the cost of pressing on through the wrong things are both real and asymmetric with a human days away. `decision-blocked` already gives cycles a way to stop without literally asking the operator; what was missing was a general rule for which situations qualify, so a cycle facing a case not on "Hard stops"'s concrete list still classifies it correctly.

**Cross-reference:** `loop-instruction.md` §"Stop vs. press on"; §21 (unattended-mode protocol, the mechanism a STOP actually invokes); `loop-instruction.md` "Hard stops" (this bundle's concrete STOP instances) and "Self-heal" (this bundle's concrete PRESS-ON instances).

## Decision 25 — Orchestrator model: Opus at low reasoning effort (operator directive 2026-08-01)

**Status:** New. **Checked first, per the operator's instruction:** this package named no orchestrator model anywhere before this pass — `decisions.md`, `loop-instruction.md`, and `scope-draft.md` had zero mentions of Sonnet, Opus, or reasoning effort. There is no prior "orchestration runs on Sonnet" statement to mark superseded; this decision is a fresh addition, not a correction.

**Decision:** The session driving this bundle's `Workflow`-tool orchestration (per §22) runs on **Opus, at low reasoning effort**. The operator observed that Opus at low reasoning effort produced materially better orchestration results than Sonnet at high reasoning effort, and pins this as the new normal for orchestration on this program.

This is a statement about the **orchestrating session only** — the session that dispatches, verifies, and rules per `loop-instruction-template.md §2.2`. It is not a blanket upgrade of every dispatched agent. Dispatched sub-agents keep task-matched tiers, unchanged by this directive:

- Cheap/mechanical work (housekeeping, lint fixes, release-notes/version-bump edits) → Haiku.
- Real implementation, debugging, and review → Sonnet.
- Adversarial verification / judge-panel steps → Opus.

`docs/governance/loop-instruction-template.md §2`'s "Default subagent model: Sonnet" is about dispatched *subagents*, a different role from the orchestrating session; it is not superseded by this decision and needs no correction.

**Mechanical caveat:** a session cannot change its own model mid-run. Setting the orchestrator to Opus at low reasoning effort is a **pre-launch operator step**, done before the cycle session starts (at the plan-approval prompt, or via `/model`), not an action a running cycle can take on itself.

**Cross-reference:** `loop-instruction.md` OPERATING METHOD callout (now names the orchestrator model); `decisions.md §22` (the `Workflow`-tool dispatch decision this pins the model for); `docs/governance/loop-instruction-template.md §2` (subagent tiering, unaffected).

## Decision 26 — A full code review is the bundle's final epic (operator directive 2026-08-01)

**Status:** New. The operator verified independently that zero files across SD-28/29/30 mentioned code review before this pass; the v0.6 CRB run closed without an end-of-run code review, and this corrects that gap going forward for all three bundles launching now.

**Decision:** Epic 12 (Bundle Code Review) is added as SD-28's last-numbered epic. Its dispatch slot is after every content-ingest epic (3-9) and Epic 11 (Build Version Numbering), and before Epic 10 (Closure Epilogue) — Closure Epilogue remains the true final step per `loop-instruction.md §"Epic ordering"` (unchanged by this decision), so any finding the review surfaces is fixed before the tranche-promotion PR (part of Epic 10) opens.

`./scripts/verify.sh` passing is a **precondition** for Epic 12 to fire, never the review itself: a green gate says the tests that exist pass, it says nothing about whether the code is right.

**Scope, at minimum:**

- Correctness of rules logic against the corpus (sampled, not exhaustively re-derived).
- No stubs or fixture-only data in production paths, per `docs/governance/no-stub-mvp-doctrine.md`.
- Content genuinely reaching a player surface, per `reach_gate.rs`'s `OPEN_FINDINGS` mechanism (spot-checked against the live IPC/UI path, not just the gate's exit code) — mechanically, this means driving the running desktop app via `apps/desktop/.claude/skills/run-desktop/driver.sh` and reading the value off a screenshot, per `loop-instruction.md`'s Definition of done item 8, with `RUN_DESKTOP_AGENT` set to a value unique to this review (`apps/desktop/.claude/skills/run-desktop/SKILL.md` §"Concurrent agents").
- Test quality, not just count — per `docs/governance/book-ingestion-playbook.md §7.4`'s mutation-test pattern, a sample of new gates/tests is checked for a case that actually fails when the thing it protects is broken.
- No hand-authored rules data in the frontend (`apps/desktop/src/`).

**Mechanism — wired into what already exists, nothing invented fresh:** the review runs `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` (this bundle's standing per-cycle dual-audit, per the `AT-28-004`/`decisions.md §6` identifier discipline and wired-integration doctrine already in force) against the **whole-bundle diff**, not a single cycle's slice: `git diff origin/develop...HEAD` — the same merge-base triple-dot comparison both scripts already default to via `BASE_BRANCH=origin/develop`. No new grep/audit tooling is invented; Epic 12 reuses the standing per-cycle gates at bundle scope and adds the manual/agent-driven judgment a grep cannot do (corpus-correctness sampling, reach-claim spot-check, test-quality sampling).

**Findings are triaged, not auto-fixed.** Each finding records a severity and a disposition: `fixed-in-bundle` or `deferred`. A `deferred` finding names an owner (a person or a specific successor bundle) and lands in `forward-scope-register.md` — an unowned deferral is not a valid disposition. Real defects are fixed in-bundle before Epic 10 fires.

**Operator escalation path, not a substitute:** the operator can separately trigger `/code-review ultra`, a multi-agent cloud review of the branch. That path is operator-triggered and billed — a cycle running under §21's unattended-mode protocol cannot launch it itself — so Epic 12 must stand on its own as the bundle's actual gate.

**Cross-reference:** `epic-breakdown.md` Epic 12; `acceptance-and-verification.md AT-28-013`; `docs/governance/no-stub-mvp-doctrine.md`; `docs/governance/book-ingestion-playbook.md §7.4`; `reach_gate.rs`; `kanban.md` card `epic-12-code-review`.

## Decision 27 — The display-value discriminator: compute the number, don't build the subsystem (operator directive 2026-08-01)

**Status:** Operator-pinned 2026-08-01. **Refines §18** — it does not supersede it. §18 says engines only when strictly necessary; this decision is the concrete test for "necessary", because §18's wording alone did not stop SD-27 deferring work that needed no engine at all.

**Decision:** A record whose rules text states a value **derived from data the engine already holds** — class level, an ability modifier, BAB, racial HD, or a constant defined on the same corpus row — is **display-value work, not engine work**. Compute the number and render it. Do not build the subsystem its noun implies.

**Operator verbatim (2026-08-01):**

> "You do not need a full blown engine for things like uses per day. You just need the ability to calculate the value that is displayed in the description or elsewhere in the UI. For example if you can do something x+y minutes a day where x = the class level and y = the attribute modifier, do the math. Maybe you get a boost from a feat - do the math. These are all just display values."

**The test, applied per record:**

| the record says | the inputs are | verdict |
|---|---|---|
| "usable %1 times per day", `%1` = class level + Cha mod | already computed | **display value — do the math** |
| "%1 rounds per day", `%1` = a same-row `DEFINE:`/`BONUS:VAR` constant | on the row itself | **display value — transcription, not interpretation** |
| a value that changes only with level/ability/feats already modelled | already computed | **display value** |
| an effect requiring expenditure, per-encounter state, or turn sequencing to be *correct* | not held anywhere | engine — and §18's "strictly necessary" bar applies |

**"Uses per day" is the canonical false positive.** It sounds like a resource-tracking subsystem — a pool, expenditure, a rest cycle. It is not. Displaying *"4 times per day"* requires the arithmetic and nothing else. Tracking how many a player has spent is a separate feature nobody asked for.

**What this cost SD-27, measured.** Multiple agents deferred PU class features and ARG feats as "blocked on engine dimensions that do not exist — SLA uses/day, luck budgets, fly manoeuvrability, companion levels." Every one of those is a display value. Once the discriminator was applied, PU class features went **41 → 58 of 64 accounted for (29 → 46 strict, then 52 hand-audited)** in a single pass, with zero regressions. `Unchained Rogue ~ Debilitating Injury` is the sharpest case: agents deferred it as "carries no numeric token" — true — while `rogue_features::prose_derived` **had already computed it and nothing consumed it.** The arithmetic existed; the display did not.

**Boundary, so this is not read as a licence to interpret.** `decisions.md §24.1` (SD-27) still forbids a general `BONUS:`/`DEFINE:`/`PREREQ:` formula interpreter. Reading a constant off the row that defines it, or substituting an already-computed value into display text, is **transcription**. Evaluating an arbitrary expression is **interpretation**. One unresolved case in SD-27 marks the line exactly: `Halfling ~ Adaptable Luck`'s second argument is `Halfling_AdaptableLuck_Bonus-1` — arithmetic on a variable, not a literal — and it remains open pending an operator ruling rather than being guessed. See SD-29 `forward-scope-register.md §7.2`.

**Cycle obligation.** Before deferring any record as "needs an engine", state **which input the engine does not have**. If every input is already computed, it is display-value work and the cycle does it. A deferral that names no missing input is not a deferral; it is unfinished work with a label on it.

**Authority:** operator directive 2026-08-01 (verbatim above); refines §18; bounded by SD-27 `decisions.md §24.1`; evidence in `docs/retro/tranche-7-retrospective.md`.

## Decision 28 — The four architectural traps SD-28 inherits from SD-27 (2026-08-01)

**Status:** Carried forward from tranche/7. Cross-cutting — each trap fires per record, not per book, so each fires six times harder here than it did in SD-27.

**Decision:** SD-28 inherits four architectural traps recorded in SD-27 `decisions.md §29`. They are **cited, not restated** — §29 is the authority and must not be allowed to drift. Each is named here because SD-27 hit every one *after* the work looked done, and the cost was rework rather than discovery.

| trap | SD-27 § | shape in SD-28 | the rule |
|---|---|---|---|
| **Two compute twins** | §29.1 | `pilot_compute.rs` vs `pilot_compute_corpus.rs`. The character sheet reads the **corpus** twin. | **A magnitude is not wired until it moves on the twin the player reads.** 15 of SD-27's 115 corrections were this class — feats wired into the hardcoded twin, tested green, and changed nothing on screen. The shared seam (`feat_derived_pillar_contributions`) plus its structural test exist; use them rather than adding a direct `feat_effects::` call to a pillar function. |
| **A third twin, in TypeScript** | §29.2 | Any surface that re-derives a rules number instead of rendering an engine `explanations` row. | Flat-footed AC lived only in `CharacterSheet.tsx` and broke PF1's dodge-denial rule. One live instance remains (`CharacterSheet.tsx:2945`, max HP). **If a number is computed in the view, it is unguarded.** |
| **Reach-gate blind spots, one permanent by construction** | §29.3 | `scanned_inventory()` reads `pub const NAME: &[Type]` slices. §24-shaped hand-modelled pure functions emit no slice and **can never be seen by a source scan.** | **No content family may rest on a single discovery source.** The corpus directory is load-bearing. SD-27's gate passed 11 tests without ever asking about ARG's headline content. |
| **`p.xx` is a placeholder, not a page** | §29.4 | Six books × per-record provenance. | **Checked per row, never per content-kind.** 143 of SD-27's 175 trait rows carried `p.xx`; verbatim transcription would have manufactured 143 false citations. Generalising from the one book that had real pages is how this bites. |

**Why this is a decision and not a note.** Every one of the four was discovered by an agent *after* a passing test claimed the work was complete. They are not defects to be fixed once; they are shapes that recur per record. A cycle that has not read §29 will reproduce them.

**Process half, same origin (`docs/retro/tranche-7-retrospective.md`):**

- **One writer per tree**, each with its own `CARGO_TARGET_DIR`, deleted when the cycle ends. 10 of SD-27's 34 incidents were shared-tree collisions — the largest single incident class — and an eleventh (rival bundle taxonomies) occurred during the retrospective itself. Never share a target dir between a worktree and the working tree: cargo will serve the wrong tree's artifacts and produce a plausible wrong number.
- **`FILES YOU OWN` must be closed under the change it mandates.** Four SD-27 briefs named a scope narrower than the fix they demanded, forcing agents to either breach scope or ship half a defect.
- **Every figure in a dispatching brief ships with the command that produced it.** Dispatching briefs were the largest single source of corrected claims in SD-27 — **41 of 115 (35.7%)**, ahead of shipped artifacts (40) and agents (32) — and only 6 of 41 were caught before implementation began.
- **A verification stage red for more than one run is a blocker, not a background condition.** SD-27's `root-full` was red on 29 of 33 full runs, and that steady redness concealed that both of its own parity gates had never executed once.

**Authority:** SD-27 `decisions.md §29` (traps), `§30` (paths and artifacts), `docs/retro/tranche-7-retrospective.md` (measurements).

## Decision 29 — The "only writer" premise was false, and it lived in SD-27, not here (2026-08-01)

**Status:** New, correcting a premise this bundle would otherwise inherit silently rather than restating something already true here.

**Where the premise actually lives.** A search across all three `docs/release/SD-2[89]-*`/`SD-30-*` packages for `only writer` / `sole writer` returns **zero hits**. The premise, and its correction, both live in **SD-27**'s own `decisions.md §28` (`docs/release/SD-27-future-state-book-content-ingestion/decisions.md:499`), dated 2026-07-31: *"There is no concurrent cycle to collide with; this branch is the only writer."* That line justified lifting SD-27's own §8 file-touch partition on the premise that v0.6 had closed and nothing else was writing the tree. `docs/retro/events/size-modifier-agent.jsonl` records the same-day correction verbatim: *"decisions.md 28 declared 8's file-touch partition spent on the stated premise that 'this branch is the only writer' -- that premise is false in practice."* Ten of the tranche's 34 logged incidents (29%, retrospective §4.1) trace to exactly this false premise, four of them `git stash` swallowing a sibling's uncommitted work.

**Why it matters here even though the text is SD-27's, not this bundle's.** SD-28, SD-29 and SD-30 are three concurrently-launched bundles sharing one checkout and branch, each dispatched from a session that can itself be running alongside sibling sessions on the same box (this bundle's own `loop-instruction.md` OPERATING METHOD callout already assumes parallel waves exist). The SD-27 mistake — asserting sole-writer status because no *specific, currently-known* concurrent bundle is active — reproduces immediately if this bundle assumes the same about SD-29, SD-30, or a human operator's own parallel session on the identical checkout.

**Ruling: the file-touch partition is necessary, not sufficient, and this bundle's own version of it must not rest on a sole-writer claim.**

- **Other writers exist, or may exist, concurrently — always.** No cycle in this bundle may assert sole-writer status as grounds for skipping a concurrency check; a partition is a courtesy between cooperating writers, not a lock.
- **`git status --porcelain` runs before every git write**, in every cycle, regardless of whether the cycle believes itself to be the only writer. A file listed that this cycle did not modify is a stop condition, reported per "Hard stops," never silently overwritten or attributed to this cycle's own change.
- **Staging is always explicit-path:** `git add <file> <file> ...`. Never `git add -A` or `git add .` — a wildcard add cannot distinguish this cycle's own changes from a sibling's uncommitted work sitting in the same tree.
- **`git stash` is never run, under any circumstance, in this repo.** The bare form stashes the *entire* working tree, not a subdirectory or a cycle's own changes, and has already destroyed a sibling's uncommitted work multiple times in this program (four of the ten shared-tree incidents above). To capture a HEAD baseline for comparison, use `git show HEAD:<path>` into a scratch file, or a separate `git worktree add` — never stash.
- **Any parallel *mutating* wave dispatches each agent with `isolation: 'worktree'`** — already required for cross-bundle/cross-epic concurrency by the OPERATING METHOD callout in `loop-instruction.md`; this decision confirms the same rule covers this bundle's own multi-book fan-outs (Epics 3-9), not only collision with SD-29/SD-30.

**Authority:** `docs/retro/tranche-7-retrospective.md` §4.1 and §6.1 (rules A1/A2); SD-27 `decisions.md §28` (where the false premise and its correction actually live — cited, not restated); `docs/retro/events/size-modifier-agent.jsonl` (the correcting incident, verbatim).


## Decision 30 — Automated disk reclamation is part of the cycle, not a manual afterthought (2026-08-01)

`docs/retro/tranche-7-retrospective.md` §4.1 records disk exhaustion as this program's **second-largest recorded orchestration failure mode — 5 of 34 logged incidents** (`/tmp` tmpfs at 91% → `ld terminated with signal 7 [Bus error]`, 20 minutes lost; `/` at 91%, 98%, 98%; `/home` at **100% used, 0 bytes available**, with "30+ per-agent `CARGO_TARGET_DIR`s under `~/.cache` totalling >600G, many 18-35G each," 25 minutes lost). The retrospective's own diagnosis is the design constraint this decision closes: *"The rule shipped in the brief; the matching `rm -rf` did not."* `AGENTS.md` and this bundle's own concurrency rules (Decision 29, above) correctly mandate a per-agent, per-source-tree `CARGO_TARGET_DIR` and tell agents to delete it when they finish — but nothing ever enforced or automated that deletion, so it did not happen at the rate the rule needed.

Two additions, landed in `scripts/` (shared across SD-28/SD-29/SD-30, not per-bundle code):

- **`scripts/reclaim.sh`** — dry-run by default; `--apply` required to delete anything. Four categories: abandoned `CARGO_TARGET_DIR`s (found under the Claude scratchpad root and this repo's `$HOME/.cache/codex-*` convention, confirmed by directory *shape* — `.rustc_info.json`/`debug`/`release` — not merely the presence of `CACHEDIR.TAG`, which fontconfig/uv/man-db also write and which a naive check flagged as a false positive on this script's own first dry run); stale `scripts/verify.sh` log directories; git worktrees whose branch is merged into `develop` or whose PR is closed/merged (`git worktree list --porcelain` + `gh pr list`); and local branches merged or gone from origin. Safety: never touches a target dir a live `cargo`/`rustc` process is using (checked via kernel-reported `comm` and `/proc/<pid>/environ`/`cwd`, not a self-matching `pgrep -f` — the self-match trap named explicitly in the brief that produced this script); never removes a worktree with uncommitted changes or unpushed commits; never touches this repo's own checkout or the `pcgen` oracle clone; never runs `git stash`. Emits a `retro.py incident` event (`recurrence-key disk-full`) whenever `--apply` actually reclaims something.
- **`scripts/verify.sh`'s new `preflight-disk` stage** — first in *both* the `--quick` and full stage sets, so it fails loudly and points at `reclaim.sh` **before** the ~490-binary `root-full` build starts, rather than only recording pressure after the fact the way the script's existing `emit_disk_pressure_event` (post-run, informational) already did.

**This bundle's `loop-instruction.md` Cycle mechanics now runs the preflight check at the start of each cycle and `scripts/reclaim.sh --apply` at cycle end.** The mandate is paired with the command, which is the entire lesson of §4.1 restated as a rule: a rule with no executable counterpart is the rule that produced 600G.

**Authority:** `docs/retro/tranche-7-retrospective.md` §4.1 (disk exhaustion, 5 of 34 incidents) and §6.1 rule A4 (`CARGO_TARGET_DIR` deletion + pre-sweep disk check); `AGENTS.md` "Concurrency and Measurement."

## Decision 31 — Per-book audit gate scope (2026-08-02, Precursor B)

**Status:** New, process correction applied in Precursor B cycle.

**Decision:** The Definition of done item 3 (`loop-instruction.md` §"Definition of done") is narrowed to **per-book scope**: a book epic passes when `cargo run --locked --bin v06_corpus_trap_report -- --audit` exits `0` **for that book's own records only**. A pre-existing defect in another bundle's content (e.g., ACG data referenced as a cross-bundle dependency) is recorded as a cross-bundle blocker against the owning bundle and does not block this book's completion.

**Why this is a decision and not a silent gate-implementation change.** Run 1 (2026-08-02) ended with all seven book epics (3–9) recorded as `decision-blocked` on a single cause: the audit gate was repo-wide, so nine pre-existing ACG `key-differs-from-name` defects (SD-22 content, not SD-28) halted all seven books at once. This was not a design choice; it was an unintended consequence of reading "the audit must exit 0" without the qualifier "for this book's records." The gate itself remains mandatory — a book cannot claim done if its own records are unclean — but the scope narrows from repo-wide to per-book to permit parallel progress on multiple books while documenting the cross-bundle dependencies for the responsible bundle's remediation.

**Scope of "this book's own records."** Records filed under the book's corpus directory (e.g., `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat/` for Ultimate Combat). Cross-bundle dependencies are out of scope for this gate.

**Cross-reference:** `loop-instruction.md` Definition of done item 3; `acceptance-and-verification.md` AT-28-003a; `progress.md` Precursor B (this entry) and Run 1 receipts (epic-3-uc, epic-4-um, epic-5-ue, epic-6-ui, epic-7-ucam, epic-8-uw, epic-9-upsi), which recorded the shared blocker and its cause.

## Decision 32 — 100% proven across 13 books; `proven` excludes `ingested-magnitude`; the anti-gaming rule (2026-08-02)

**Status:** New, recording an operator directive that raises this bundle's target and adds Epics 13–30 (`epic-breakdown.md §"Completion epics (E13–E30)"`, `kanban.md` cards `epic-13-calibration` … `epic-30-integrity`).

**The target.** The six — measurably **seven** — previously-started books and the seven Ultimate books reach **100% proven**: 32,061 of 32,061 units. Current proven across those books is **2,900**, a gap of **29,161 units**. Re-derived 2026-08-02T11:50:31Z from the `work_inventory` section of `/home/ubuntu/swarm-observer/PF1e-dashboard.json` (`cargo run --bin v06_work_inventory`); the exact command is recorded inline in `epic-breakdown.md` as `$WI`. The directive named six started books; the inventory reports seven with a compiled rule set and non-zero proven units — `core_essentials` (2,639 units, 46 proven) is the seventh and is included. If the operator intends it out of scope, drop Epic 21 and the gap becomes 26,568.

**`proven` = `grounded` + `text-complete` only, and that makes harness-widening a prerequisite.** The formula excludes `ingested-magnitude`, whose own `status_vocabulary` definition reads verbatim: *"The engine holds the record WITH its real numeric fields, but this generator observes no consumer delta for this kind (spells, equipment). Strictly weaker than `grounded` and deliberately a separate word: calling it grounded would be the same over-claim this inventory exists to prevent."* 4,050 units sit there — 2,700 `equipment`, 1,067 `spell`, 283 `equipment_modifier`. `core_rulebook` is the extreme case: 3,062 of its 5,716 units (54%) are already held by the engine with real numbers, yet it reports 912 proven (16%).

The cause is in the generator, not the content. `src/bin/v06_work_inventory.rs:1328 fn classify()` consults `facts.feat_effect_wired` — populated by `probe_feat_effect_wiring` (line 1227), a probe that observes a real computed delta — which is why feats can reach `grounded`. Its `Kind::Spell` and `Kind::Equipment` arms have **no probe at all**; they assign `ingested-magnitude` structurally from the presence of a spell-list entry or an equipment-table entry. **Therefore ingesting more content cannot by itself reach 100% proven.** Epic 14 (observation-harness widening) is a gating prerequisite for roughly 4,050 units and a hard dependency of Epics 23, 25 and 28; it is not an afterthought.

**Ruling: the anti-gaming rule, verbatim and binding on every epic in this set.**

> No epic may reach its target by reclassifying units, relaxing the classifier, broadening what counts as text-complete, weakening or skipping a gate, or editing the work-inventory generator to report more favourably. The only legitimate paths to proven are (a) the engine genuinely holds the record and a real consumer observes its magnitude, or (b) the corpus record genuinely carries no magnitude token (text-complete, per the operator's standing rule). Any unit that cannot reach proven honestly gets `deferred-with-reason` carrying the engine's own verbatim diagnostic, or an `OPEN_FINDINGS` entry — never a silent reclassification.

Epic 14 is the one epic that legitimately changes `v06_work_inventory.rs`, and is therefore the sharpest gaming risk in the set: widening the harness is exactly what a dishonest run would do to move numbers without doing work. Epic 30 (Completion Integrity Gate) reviews that diff line by line, classifying every change as *added observation* or *changed definition*, and treats any changed definition as a finding requiring explicit operator approval recorded here.

**Realism, recorded so it is not quietly dropped.** 2,900 units is the entire proven output of this program to date across every bundle that has run. The directive asks for roughly **ten times that**. Of the 29,161-unit gap: 12,415 units are in books with no compiled rule set; 8,492 are real gaps inside books already called finished; 4,172 cannot yet be costed because the generator could not classify them; 4,050 are already held by the engine and blocked solely on observation. **This scope dwarfs everything the program has done to date.** Accordingly, Epic 13 takes `ultimate_campaign` (23 units) end-to-end to 100% proven first and records a measured cost per unit; Epic 24 does the same for the first full-size book. **Until Epic 13 reports, every later epic's duration is explicitly unestimated**, and no schedule asserted before then is anything but a guess.

**Cross-reference:** `epic-breakdown.md` §"Completion epics (E13–E30)" (targets, definitions of done, per-epic progress commands); `kanban.md` cards `epic-13-calibration` … `epic-30-integrity`; `decisions.md §29` (worktree isolation, which E14/E15 concurrency requires); `§10` (equipment-catalog widening, a dependency of E14-F2); `§29.1` (a magnitude is not wired until it moves on the twin the player reads); `§29.3` (no content family may rest on a single discovery source).

## Decision 33 — SD28-E13 calibration: `PRETEXT:` precedent confirmed, and two undocumented corpus splices found (2026-08-03)

**Status:** New, recording the `epic-13-calibration` cycle's two on-evidence rulings against `ultimate_campaign`'s 23-record Story Feat catalog (`uca_feats.lst`).

**1. `PRETEXT:` is carried as display prerequisite text, never synthesised into a formal `PRE`-family token — and this is established precedent, not a fresh ruling.** All 23 `uca_feats.lst` records carry `PRETEXT:` prose ("Prerequisite:You must...") rather than a structured `PREABILITY:`/`PRESTAT:`/etc. token. Before writing `ultimate_campaign::feat_tables`, this cycle checked `feats_all::PU_FEAT_PREREQUISITES` and found it already carries raw `PRETEXT:` strings as prerequisite tokens for four Pathfinder Unchained records (`Combat Stamina`, `Extra Stamina`, `Push the Limits`, `Signature Skill`) and for three ARG "channel energy" records — landed before this cycle, with no prior decisions.md entry naming the choice explicitly. This decision makes that established practice a named, citable ruling: `PRETEXT:` tokens are carried through unedited as `["PRETEXT:<verbatim corpus text>"]` prerequisite entries; the engine never infers or fabricates a formal `PRE` token from prose it cannot mechanically verify. `UCA_FEAT_PREREQUISITES` (`feats_all.rs`) follows the same shape for all 23 UCA keys.

**2. One corpus splice beyond the one flagged going into this cycle — a second candidate was found and then cleared on further evidence; do not defer merely because text repeats.** The cycle brief named `Fearless Zeal` (`uca_feats.lst:66`) as a known corpus defect: its `.MOD BENEFIT:` row reads correctly through "...but" then splices verbatim into `Damned`'s own `BENEFIT:` row (`uca_feats.lst:37`), confirmed byte-for-byte, not merely similar phrasing. Per `loop-instruction.md` step 1b's re-derivation mandate, every field for all 23 records was re-extracted directly from the corpus (not transcribed from the brief) and cross-checked with a sliding 10-word shingle comparison across all 23 `BENEFIT:` rows to surface any other repeated sentence boundary. That comparison surfaced two candidates:

- **`Magnum Opus`** (`uca_feats.lst:74`) — a confirmed splice. The row's own sentence is grammatically truncated **in its own right, independent of any cross-row comparison**: "...or win the artistic Completion Benefit:..." has no object after "artistic" — a clause cut off mid-phrase. The corpus does not say what `Magnum Opus`'s own artistic-triumph goal was, so this record cannot be displayed honestly.
- **`Stronghold`** (`uca_feats.lst:76`) — **initially deferred in this cycle's first pass, then corrected back to text-complete** after independent review (team-lead, same day) asked for per-record evidence rather than accepting the shingle match alone. `Stronghold`'s row carries the same trailing sentence appended to `Magnum Opus` ("You gain the ability to reroll a failed saving throw once per day..."), but unlike `Magnum Opus`, `Stronghold`'s OWN sentence is grammatically complete and self-terminating on its own: "...you could grant your archers +2 on attack rolls while your front line gains a +2 bonus to AC." A repeated sentence is not, by itself, evidence of corruption in this corpus — `Damned` and `Fearless Zeal` (before its own fix) demonstrate the file can carry the same completion-tier text on two rows, and `Damned`'s row is genuinely undamaged. The discriminator is whether the record's OWN text is grammatically broken, not whether it shares wording with another record. `Stronghold`'s does not. **Correction: `benefit` carries `Stronghold`'s own text ONLY, trimmed at the point its own sentence ends** — no word added, guessed, or paraphrased; the trailing foreign sentence (proven to belong to `Magnum Opus`'s row, not this one) is excluded rather than attributed here. This is declining to attribute a different record's sentence to this one, not repairing or inventing text.

The one benign false positive the shingle comparison also surfaced (`Champion`/`Town Tamer` sharing "+1 dodge bonus to AC" and neighboring words) is ordinary repeated game-mechanics language, not a duplicated sentence boundary, and was excluded from the start.

**Correction against the brief, corrected again same-day.** The cycle brief's honest target read "22 text-complete + 1 deferred-with-reason = 23 accounted." This cycle's first pass, over-broad, read "20 text-complete + 3 deferred-with-reason." The actual, fully re-derived and per-record-justified target is **21 text-complete + 2 deferred-with-reason = 23 accounted**. Both moves emitted as `scripts/retro.py correction` events (subject: SD28-E13 cycle brief / this module's own first pass; verified-by: the 10-word shingle comparison script plus, for the second correction, direct re-reading of `Stronghold`'s own sentence boundary against `Magnum Opus`'s; cycle receipt `artifacts/e13-cost-calibration.md`).

**Ruling, per the operator's standing instruction on `Fearless Zeal`, extended and refined:** corrupted upstream text is never displayed to a player and never repaired by inventing replacement prose — and, symmetrically, a record whose own text is genuinely intact is not withheld from the player merely because it shares wording with another record. Both deferred records (`Fearless Zeal`, `Magnum Opus`) ingest as real records — `key`/`name`/`description`/`pretext`/`source_page` all populated and independently correct — with `benefit: None` and an engine-emitted verbatim diagnostic naming the file:line and the specific defect (`ultimate_campaign::feat_tables::DEFERRED_WITH_REASON`), surfaced to `v06_work_inventory`'s `deferred-with-reason` status and joined into the player-facing feat description (`feats_all::map_uca_entry`) so a consumer sees why the record has no benefit text rather than either a stub placeholder or invented prose. `Stronghold` ships text-complete with its own real, complete text.

**Cross-reference:** `ultimate_campaign::feat_tables` module doc comment (full citations, re-derivation method, and the `Stronghold` correction in full); `feats_all::UCA_FEAT_PREREQUISITES` module doc comment (`PRETEXT:` precedent); `artifacts/e13-cost-calibration.md` SD28-E13 receipt; `decisions.md §32` (anti-gaming rule this correction complies with — the honest target moved twice because re-derivation and then independent review each found a more accurate classification, not because the classifier was relaxed or tightened for convenience).

## Decision 34 — A new book cannot pass full `verify.sh` before its first commit; `last_ingested_at_is_a_real_git_derived_timestamp_when_available` has a real gap (2026-08-03)

**Status:** New, recording a structural finding that will recur for every remaining book epic (Epics 17, 18, 19-29) unless read here first.

**The failure mode.** `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`'s `last_ingested_at_is_a_real_git_derived_timestamp_when_available` test iterates every book `build_corpus_ingest_diagnostic()` reports and hard-`unwrap`s each one's `last_ingested_at`, panicking if any book has no git-derived timestamp. `last_ingested_at` is populated by `last_commit_iso_date`, which shells out to `git log -1 --format=%cI -- <repo_relative_dir>` and returns `None` when git has no history for that path — which is exactly the state of `src/rules_core/rules_tables/<new_book>/` between the moment the directory is created on disk and the moment it is first committed. During that window, `every_book_landed_in_rules_tables_is_reported`'s own drift guard (a **different**, correctly-designed test) already requires the new book to be registered in `corpus_ingest_diagnostic.rs` — so the new book must be reported by name before it has git history, and this test then fails on exactly that book until it is committed.

**Confirmed, not assumed.** Read the test body before acting on this (per the operator's standing instruction not to route around a test without understanding it first): the function's own name carries "when_available," which names the *field's* documented semantics (`last_ingested_at: Option<String>`, `None` when git isn't reachable — e.g. a packaged production build with no `.git` checkout). But the *test itself* does not tolerate `None` for any book — it `unwrap_or_else(|| panic!(...))`s unconditionally, with a comment asserting "this test runs inside the real repo checkout, so git history for every book directory must be reachable." That assumption is false for a book between directory-creation and first-commit. **This is a real gap in the test, not a misunderstanding of it** — the test's own name promises tolerance the test's own body does not implement.

**The correct cycle sequence for any book epic that adds a new `rules_tables/<book>/` directory: commit before the full `verify.sh` run, not after.** The normal cycle order (TDD green → full verify → commit) inverts for exactly this one gate on a book's first cycle: the directory must exist on disk (so `every_book_landed_in_rules_tables_is_reported` and the diagnostic registration are correct) and be committed (so `last_ingested_at_...` passes) before a full, clean `verify.sh` run can exit 0. A provisional commit immediately followed by a full `verify.sh` run is the correct shape for a book's first cycle, not a shortcut around the gate — every other check (targeted `cargo test`, `v06_corpus_trap_report --audit`, reach-gate, the four-check wired-integration audit, `v06_work_inventory` regeneration) is run and green *before* that commit, exactly per the ordinary cycle mechanics; only the one full-`verify.sh` run moves after the commit, for this one gate, for a book's first cycle only.

**This is not specific to `ultimate_campaign`.** Every remaining Ultimate book (Epics 17-29: `pathfinder_unchained` already exists so is unaffected, but `bestiary1`... no — every book that does NOT yet have a `src/rules_core/rules_tables/<book>/` directory today will hit this identically on its first cycle: Ultimate Combat, Ultimate Magic, Ultimate Equipment, Ultimate Intrigue, Ultimate Wilderness, Ultimate Psionics, and any future book. **Recorded here so the `epic-17-...` through `epic-29-...` agents find it before hitting the wall, per the operator's explicit instruction on this finding.**

**Fixing the test is out of scope for this cycle** (this cycle's granted write scope is `ultimate_campaign` ingestion, not a repo-wide test-infrastructure change affecting every other book's coverage) — recorded as an `OPEN_FINDINGS`-shaped gap for a future cycle: the test should either tolerate `None` for a book directory git cannot yet see history for (matching its own name), or assert only against books present at `develop`'s HEAD rather than every book the diagnostic currently reports, so a book's first cycle is not forced into the commit-before-verify inversion documented above.

**Authority:** confirmed directly against the test's own source (`corpus_ingest_diagnostic.rs`, `last_ingested_at_is_a_real_git_derived_timestamp_when_available` and `last_commit_iso_date`) and against the observed failure in this cycle's own `verify.sh` run, not inferred from the failure message alone.

**Cross-reference:** `progress.md` SD28-E13 receipt (cycle sequence actually followed); `loop-instruction.md` Cycle mechanics step 4 (this decision is the one documented exception to "verify before commit," scoped to a book's first cycle only).

## Decision 35 — SD28-E16: `_abilities_race.lst` whole-file classification inflates `race_trait` `not-ingested` counts with two non-trait row shapes; a measurement correction, not progress (2026-08-07)

**Status:** New. Ships with the fix in the same commit. This is the highest-blast-radius change made under SD-28 to date — it touches `v06_work_inventory.rs`'s classifier, which every book's `not-ingested`/`proven` figures are derived from — and is recorded here per the strict guardrails the operator set for editing this generator.

**The finding.** `file_kind()` (`src/bin/v06_work_inventory.rs`) classifies an entire `_abilities_race.lst` file as `Kind::RaceTrait`, with no row-level discrimination beyond the pre-existing `.MOD`/comment/directive/internal-namespace traps. Re-deriving ARG's reported 823-unit `race_trait` `not-ingested` figure row-by-row against `arg_abilities_race.lst` (979 real, non-`.MOD` rows, matching the dashboard's `corpus_units_declared: 979` to within 1) found:

- **156 rows** — genuine alternate racial traits for the 18 in-scope races (Core Rulebook's 7, Bestiary 1's 11 per `decisions.md §25.3`). **100% already ingested.** Zero real gap here.
- **87 rows** — genuine alternate racial traits for the 19 races ARG's `IN_SCOPE_RACES` roster excludes (Catfolk, Dhampir, Fetchling, Gillman, Grippli, Hag, Ifrit, Kitsune, Nagaji, Oread, Ratfolk, Samsaran, Strix, Suli, Sylph, Undine, Vanara, Vishkanya, Wayang). Verified their true source book: all 19 declare in `pathfinder/paizo/campaign_setting/inner_sea_races/isr_races.lst` (command: `grep -rl "^<Race>\b" --include="*_races.lst" .` against the full PCGen tree, run per race), a book outside SD-28's set entirely (not in Epics 3-9 or 17-29). **Structurally unclosable inside this bundle** — this belongs as an `OPEN_FINDINGS` entry naming Inner Sea Races ingestion as the remedy, but `OPEN_FINDINGS` (`apps/desktop/src-tauri/src/reach_gate.rs`) lives inside `apps/desktop/**`, which `epic-31-spell-wiring` holds uncommitted for the duration of this cycle per the standing collision boundary — this cycle's write scope does not reach it. Recorded here in full instead, as the handoff for whichever cycle next has write access to `reach_gate.rs`: add `("advanced_race_guide", "race_trait", "<the 19 races' KEYs>")`-shaped entries (or the file's existing per-family shape) stating the gap and "blocked on Inner Sea Races ingestion, a book outside SD-28's set."
- **291 rows** — `Favored Class Bonus` entries (`TYPE:...FavoredClassBonus...`, one row per race x class). A different game mechanic entirely; `race_trait_ids` is keyed on `<race>.<trait-slug>` pairs and can never hold an FCB identity, so these could never leave `not-ingested` regardless of ingestion effort. `ingest_race_traits_arg.rs`'s own pre-existing test (`rows_without_a_racial_trait_type_are_not_racial_traits`) already used an FCB row as its own "this is not a racial trait" example — the binary was already correctly refusing these; the inventory was the thing miscounting them.
- **60 rows** — `CATEGORY:Choice` sub-option rows belonging to an already-ingested parent trait (e.g. `Elf ~ Elemental Resistance`, already one of the 156, offers 4 `CATEGORY:Choice` rows — Acid/Cold/Electricity/Fire — as its `CHOOSE:` menu). Verified against the corpus: all 156 already-ingested ARG alternate traits carry `CATEGORY:Special Ability`; none carry `CATEGORY:Choice` (full scan of `data/corpus/advanced_race_guide/race_trait/*/*.json`). Counting the sub-option as a second unit double-counts the parent.
- **383 rows** — 23 more out-of-scope-race `CATEGORY:Choice` sub-options (same disposition as the 87 above) plus 361 rows with no race attribution at all, dominated by Race Builder point-buy plumbing (`Mystic Past Life ~ INT 1` through `INT 18`, 18 near-identical rows each stamping one possible ability-score tier's bonus — `BONUS:VAR|MysticPastLifeScoreINT|N|TYPE=Base`) and outright placeholder rows (`core_rulebook/cr_abilities_race.lst`: `"No Race Trait Available"`, `"Remove Excess Points from Pool"`). **Not fixed this cycle** — flagged below as unresolved, per the operator's instruction to report rather than silently narrow scope.

**Generalization check (per the operator's guardrail 5), sampled rather than assumed complete.** The `FavoredClassBonus`/`CATEGORY:Choice` pattern recurs in every other book's `_abilities_race.lst` checked: `advanced_class_guide` (190 rows, 70 FCB), `advanced_players_guide` (176 rows, 54 FCB + 2 `CATEGORY:Choice` + 50 genuine racial-trait-type rows), `core_rulebook` (131 rows, 0 FCB but includes the placeholder rows above and 119 `CATEGORY:Spell-Like Ability` picklist rows — a THIRD row shape, not yet resolved, see below), `pathfinder_unchained` (229 rows, all unclassified "other" — not yet sampled in detail), `bestiary`/`beastiary1` (620 rows, only 3 carry a `<Race> Racial Trait` TYPE marker — the overwhelming majority of bestiary's reported 620 `race_trait` units are almost certainly this same defect, unverified row-by-row this cycle). **This means a meaningful, currently unquantified fraction of the 5,899-unit `not-ingested` figure (and, upstream of that, the 29,161-unit 100%-proven-directive gap `decisions.md §32` records) is measurement noise, not missing content — the true size is not yet known.**

**The fix landed this cycle (evidence-based, reclassify-never-delete, per the operator's guardrails 1-2).** Two new named trap rules in `TRAP_RULES`, gated to `kind == Kind::RaceTrait` only (so `_abilities_class.lst`, `_companionmods.lst`, etc. are untouched — sampled `acg_abilities_class.lst`, `apg_abilities_class.lst`, `pu_abilities_class.lst` for the same `CATEGORY:Choice`/FCB shapes and found zero hits in any of the three, so the defect is confirmed *not* to generalize to `Kind::ClassFeature`'s file family, at least in this sample):

- `race_favored_class_bonus_row` — any row whose `TYPE:` field carries a `FavoredClassBonus` dot-component. Discriminator verified against every FCB row sampled across ARG/ACG/APG.
- `race_choice_suboption_row` — any row carrying the literal field `CATEGORY:Choice`. Discriminator verified against the 156-record already-correct ARG corpus (zero `CATEGORY:Choice` among them) plus the `Elf ~ Elemental Resistance` / `Half-Elf ~ Ancestral Arms` parent-trait examples.

Both are **exclusions, not deletions**: every hit is counted in `trap_hits` under its own named id, visible in every book's JSON output, exactly like the program's pre-existing `mod_record`/`comment_or_disabled`/`missing_classifying_token` traps. No `Kind` variant was added for FCB content this cycle (that is a larger, separate change — a new `Kind::FavoredClassBonus` touching every `by_kind`/`classify()`/dashboard-consumer match arm — deliberately deferred rather than rushed into this already-high-risk edit; recorded as an `OPEN_FINDINGS` entry).

**Ruling on sub-choice rows (per the operator's guardrail 4 — a decision, not a parser accident).** A `CATEGORY:Choice` row under a `_abilities_race.lst` parent trait is a **component of its parent's unit, never an independent unit**. Reasoning: the player does not select "Elemental Resistance (Acid)" as a purchase distinct from "Elemental Resistance" — they take the one trait "Elemental Resistance" and the game rules require picking an element as part of taking it; the corpus's own `CATEGORY:Special Ability` vs `CATEGORY:Choice` distinction already encodes exactly this "unit vs. its own configuration" boundary, and the already-shipped 156 ARG records never split a trait's choices into separate records. Applied consistently: no `CATEGORY:Choice` row under any `_abilities_race.lst` file is ever counted as its own `race_trait` unit, in any book, from this cycle forward.

**What this changes and what it does not — stated per the operator's explicit instruction, in these words: this is a measurement correction, not progress.** Before this fix, ARG's `race_trait` reconciliation reported `corpus_units_declared: 979`. After (re-derivation pending the blocked `v06_work_inventory` run — see `progress.md`'s SD28-E16 receipt for the observed before/after numbers and the exact HEAD they were measured against), the FCB and choice-suboption rows across every book drop out of the `not-ingested` denominator entirely. **No content was ingested by this change. No unit moved to `grounded`, `text-complete`, or `deferred-with-reason`. The count shrinks because the count was wrong**, not because SD-28 did content work. Reporting this drop as coverage gained would violate `decisions.md §32` even though the mechanism is a generator fix rather than a reclassification of content — the anti-gaming rule's substance ("a smaller honest number beats a larger one," but the number must reflect real state) applies to a measurement-tool fix exactly as it applies to a content classification.

**Unresolved, recorded rather than silently deferred, per guardrail 5:**

1. `core_rulebook`'s `CATEGORY:Spell-Like Ability` / `KEY:Racial SLA ~ <spell>` picklist rows (119 of its 131 `_abilities_race.lst` rows) — a third non-trait row shape, structurally the same defect (a shared spell-choice menu, not an independent racial trait) but with a different discriminating token (`CATEGORY:Spell-Like Ability` + `KEY:Racial SLA ~ ...` prefix, not yet verified as universal across books). Not fixed this cycle.
2. `pathfinder_unchained`'s 229 "other" rows — not sampled in detail this cycle; unknown what fraction is real vs. the same defect family.
3. `bestiary`'s 620 `race_trait`-classified rows, of which only 3 carry a `<Race> Racial Trait` TYPE marker — the single largest suspected instance of this defect in the program, unverified row-by-row.
4. `core_rulebook`'s literal placeholder rows (`"No Race Trait Available"`, `"Remove Excess Points from Pool"`) — Race Builder UI plumbing with no game content at all; only 2 rows found so far, likely more per book, no dedicated trap rule written for this shape yet.

**Cross-reference:** `progress.md` SD28-E16 receipt (before/after totals, per book and per kind, and the observed `v06_work_inventory` exit code); `epic-breakdown.md` Epic 16 section (its own 2026-08-07 correction, superseded in part by this decision); `decisions.md §25.3` (ARG's in-scope-race roster, the boundary this decision's Inner Sea Races finding respects); `decisions.md §32` (the anti-gaming rule this fix complies with — an evidence-based, transparently-logged, reclassify-never-delete correction, not a relaxed gate).

## Decision 36 — A recurring failure class: a hand-maintained roster/table sitting beside its real consumer, with nothing failing when they diverge (2026-08-07)

**Status:** New. Naming a pattern this codebase has independently rediscovered and independently fixed fifteen times, so the next agent recognizes the shape on sight rather than re-diagnosing it from scratch.

**The generalization, stated plainly: anywhere a hand-maintained list sits beside a derivable one, they diverge and nothing fails.** Not "might diverge" — every instance below is a case where it actually did, undetected until someone traced a symptom back to it. A reader who internalizes this sentence should be able to find the seventh instance without this decision's help: look for any place a fact about the corpus or engine is written down twice.

**The shape.** Some fact about the corpus or the engine is duplicated into a second, hand-written structure — a roster, an allowlist, a lookup table — that a consumer trusts *instead of* asking the real source of truth. Nothing mechanically ties the two together, so they silently drift: the real source grows or changes, the hand-written copy does not, and no compiler error, and often no test, catches the gap. The bug is never in either structure alone; it is in the missing link between them.

**Fifteen instances found in this codebase, oldest fix first:**

1. **`apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`'s Bestiary 1 monster roster.** Its own doc comment (line 132) records the fix: "This used to be a hand-maintained duplicate list (`ALL_BESTIARY1_MONSTERS`)... now replaced with `MonsterId::ALL`" — i.e. the fix was to stop hand-copying the roster and instead read the engine's own exhaustively-matched enum constant, so a roster change fails to *compile* at every call site instead of silently under-counting.
2. **The stub registry / `data/stubs/` divergence** (`tests/sd_governance_stub_registry_divergence.rs`, discovered 2026-08-07 during a stub-registry audit): `docs/governance/wired-integration-stubs-registry.md` (hand-maintained ledger) vs. `data/stubs/<book_id>.json` (the real artifacts) — 13 manifests shipped with no matching registry entry between SD-27 and SD-28/SD-30, caught only by a purpose-built set-equality test written after the fact, not by any preexisting mechanism.
3. **`KNOWN_KEY_MISMATCH_DEBT`** (`tests/v06_corpus_trap_report.rs`) — an *enumerated* exception list standing beside the real corpus-key data, kept deliberately small and named rather than silently trusted, per that test's own doc comment — the correct shape for when a hand-maintained list is unavoidable (bounded, named, itself tested), contrasted with the four other instances where the divergence went undetected.
4. **`forward-scope-register.md` §C3.1** (UE equipment-catalog widening) — closed as "stale, corrected 2026-08-06 by `epic-14-harness`": a scope-register entry that had drifted from the real, current state of the widening work and needed an explicit correction pass to re-sync.
5. **This decision's own finding: `crb::race_tables::race_traits()`.** A 50-entry, hand-written, CRB-only Rust table that `v06_work_inventory`'s `Kind::RaceTrait` classifier treats as the sole source of truth for whether *any* book's race trait is "grounded" — with no knowledge that `apps/desktop/src-tauri/src/race_trait_picker.rs` independently reads the JSON corpus directly and genuinely serves ARG's 156 alternate traits to players. The table and the JSON-driven picker never disagree about content (both are correct in their own domain), but the classifier that is supposed to report on both only consults one of them, so it reports genuine, shipped, player-reaching content as `not-ingested`. See `decisions.md`'s SD28-E16 backfill entries above for the full trace; the fix is deliberately **not** landed in this same commit (guardrail: do not compound two high-blast-radius classifier edits in one cycle) and is scoped as its own isolated cycle.

6. **`beastiary1::monster_key_resolve`'s hand-written string-literal match** (found and fixed the same day, landing Bestiary 1 monster subset 09). The function is a `match key { "beastiary1:monster:ghoul" => MonsterId::Ghoul, ... }` block maintained by hand, one arm per monster, entirely separate from `MonsterId::ALL` (already the correct, single-source-of-truth shape per instance 1 above) and from `monster_resolve`'s own exhaustive match on `MonsterId`. Adding subset 09's five monsters to `MonsterId`/`MonsterId::ALL`/`monster_resolve` (all correctly exhaustive-match-enforced) compiled clean; `monster_key_resolve` compiled clean too, because a `match` with a wildcard `_ => return None` arm never fails to compile when a case goes unhandled — it just silently returns `None` for the missing keys. Caught by `apps/desktop/src-tauri/src/monster_catalog.rs`'s `every_served_key_resolves_back_to_its_record` test, which calls `monster_key_resolve` for every catalog-served key rather than trusting it.

    **How it presented versus what it was — the part worth remembering.** The failing `verify.sh` run reported this alongside five other failures that were genuinely pinned-count drift (`the_catalog_serves_every_ingested_bestiary_1_monster` 41→46, the land-speed-zero and multi-subtype tests, two `corpus_ingest_diagnostic` counts). Read in that company, `every_served_key_resolves_back_to_its_record` looked like the same shape — "just another stale number." It was not: the panic message (`served key 'beastiary1:monster:shadow' resolves to no record`) named a *lookup failure*, not a count mismatch, and tracing it led to a second hand-maintained table, not a constant. The lesson: when a batch of failures shares one commit's blast radius, do not assume they share one root cause — read each panic message, because a mixed batch can hide a real defect inside a pile of expected count-drift.

    **This is the sharpest instance in the set**: unlike instances 1-5, which are hand-maintained tables a *human* forgot to update, this one is a hand-maintained table the *compiler itself cannot flag* even in principle, because the wildcard arm is exactly what makes the match total. The lesson is not "add a test" (that test already existed and did its job) but "prefer a construct the compiler can make exhaustive" wherever one is available — `MonsterId::ALL`-driven code fails to compile on drift; a `match ... => return None` fails silently and needs an external test to catch the same class of bug `MonsterId::ALL` prevents by construction.

    **Left hand-written, not yet derived — an open follow-up, named explicitly rather than left silent.** This cycle fixed the *symptom* (added the 5 missing key arms) but not the *shape*: `monster_key_resolve` is still a hand-written string-literal match today, so subset 10 will trip the identical failure the same way subset 09 just did — presenting as a confusing serving-path error (`every_served_key_resolves_back_to_its_record`) rather than an obviously-a-count-problem error, exactly because this instance evades the compiler. The consistent fix, matching instance 7's "derive, don't re-pin" resolution: iterate `MonsterId::ALL`, derive each variant's canonical `beastiary1:monster:<slug>` key the same way `monster_catalog.rs`'s own key-building does, and match on that instead of a literal per arm — with a test asserting every `MonsterId::ALL` variant round-trips through `monster_key_resolve`. Scoped as a small follow-up commit after this cycle's two, not landed here, so as not to touch a file mid-verification-round. Recording this rather than silently leaving `monster_key_resolve` unfixed while citing it as a textbook instance of the very pattern this decision warns about.

7. **`apps/desktop/src-tauri/src/monster_catalog.rs`'s `checked >= 41 * 4` assertion** (found the same day, during the count sweep for subset 09). A sanity floor on a test that sweeps every catalog field, meant to catch the sweep silently stopping early. Because the comparison is `>=` against a hardcoded literal, adding monsters makes `checked` *larger*, so the assertion kept passing right through the roster growing from 41 to 46 — a green result that was, at that moment, checking 20% less of the real catalog than the literal implied, and `verify.sh` could never flag it because nothing about a passing `>=` looks wrong. The instance-1 fix applies directly: derive from `MonsterId::ALL.len() * 4` instead of a literal, so the floor grows with the roster automatically. **The variant this instance adds to the pattern:** instances 1-6 are all cases of the wrong answer being served (a missing entry, a `None` where a record exists); this one is a *floor that stays satisfied while its coverage shrinks* — silent under-checking rather than a missing lookup, worth naming separately because "the test still passes" is exactly the property that makes this shape harder to notice than the others.

8. **`data/corpus/beastiary/LICENSE.json`'s `records_processed` field** (found the same day, via `tests/sd27_book_license_record_counts.rs`'s cross-book gate). Stated `164`; the real on-disk count after subset 09 landed 5 new monster files is `169` (4 equipment + 46 monster + 11 race + 108 race_trait). `sd27_gen_book_cache.rs`'s own doc comment already names this exact defect class for ARG/PU's `LICENSE.json` — a `records_processed` set from one generator's own write count, wrong the moment a *second* writer (a different `src/bin/` ingest binary) adds records into the same book directory without updating the file — and that binary was already fixed to derive the count from disk via `count_on_disk_records`, tracking whichever writer runs last. **Bestiary 1's `LICENSE.json` never got the same fix, because it was never that generator's problem to begin with**: `cache_gen::beastiary1` only ever wrote `monster/`+`equipment/`; `race/`+`race_trait/` are written by the separate `ingest_races.rs`; no single binary owned the whole book's count, and the file had been hand-edited on every past content addition instead (`classified_by_cycle: E2.0.9`'s own `screening_method_note` documents two manual passes). Fixed by adding the identical `count_on_disk_records` logic to `gen_cache_beastiary.rs` itself (duplicated rather than shared across the two `src/bin/` binaries, matching this program's existing convention of small, independently-readable generator binaries) and rewriting `records_processed` from the real disk count at the end of every generation run, operating on raw `serde_json::Value` rather than a typed struct so every other field in the file survives untouched (the same discipline `enrich_equipment_raw_tokens.rs` already established for exactly this "don't drop what you don't know about" reason). **A distinct sub-shape, not just another instance.** Instances 1-7 are all one hand-maintained structure sitting beside one derivable source — a roster next to `MonsterId::ALL`, a match beside an enum, a literal beside a `.len()`. Instance 8 has no such pair: it is an *aggregate with no owner*, a count meant to describe the union of what several independent writers (`cache_gen::beastiary1`, `ingest_races.rs`, and any future one) put into a shared directory, where none of those writers was ever responsible for the whole. That absence of a single point of truth is why this one went stale silently for longer than the others and why "make the generator derive it" was not the obvious fix until the specific generator responsible for the aggregate — not any one writer — was identified. A reader hunting instance nine should check for orphaned aggregates (a count of "everything in this directory," multiple writers, no reconciler) as well as for stale mirrors (one list beside one source). Fixing this one instance is not fixing the pattern — this decision's own existence, naming the shape rather than only patching each sighting, is what should make instance nine recognizable on sight instead of requiring its own investigation.

9. **`monster_key`, duplicated while fixing instance 6, caught before it shipped.** Deriving `monster_key_resolve` from `MonsterId::ALL` (instance 6's fix) needed a key-slug formula, and the obvious place to reach for one was `apps/desktop/src-tauri/src/monster_catalog.rs`'s existing `monster_key(block: &MonsterStatBlock) -> String` -- except that function lives in a different crate (the desktop Tauri crate, not the engine library), so rather than importing it, a second, private `fn monster_key(name: &str) -> String` was written in `beastiary1/mod.rs` with the identical formula (lowercase, spaces to underscores). Two private functions in two crates implementing the same slug rule is not sharing, it is a second copy that can drift the moment a name arrives that the two would slug differently -- and the existing `every_served_key_resolves_back_to_its_record` test would not have caught that drift, because it only proves the catalog's own key resolves, not that the catalog's formula agrees with the engine's. Caught in review before commit, not after: made `beastiary1::monster_key` `pub`, and `monster_catalog.rs`'s own `monster_key` became a one-line delegation (`beastiary1::monster_key(&block.name)`) instead of a re-implementation. **The lesson this instance adds:** the pattern is not only "an old mirror going stale" -- it is just as easy to create a *brand new* instance of it while fixing an existing one, especially across a crate boundary where "just write a local copy" is the path of least resistance. The check that would generalize this: before writing a small derivation/formatting helper, grep for its exact output shape (here, the literal string `"beastiary1:monster:"`) across the whole tree first: `grep -rn '"beastiary1:monster:' --include='*.rs' .` would have surfaced the existing formula in `monster_catalog.rs` before a second one was written.

10. **`race_trait_picker.rs`'s duplicated `RACE_CORPUS_BOOKS`, with a stale justification** (found while landing APG's ingest, SD28-E16, 2026-08-08). `race_catalog.rs` already declared `pub(crate) const RACE_CORPUS_BOOKS`; `race_trait_picker.rs` carried its own private copy, with a comment stating the reason: "duplicated rather than imported because that constant is private to its module and this cycle's write scope does not include editing it." **The constant was already `pub(crate)` when that comment was written** — the justification was false the moment it was committed, or went stale immediately after, and nobody re-checked it before this cycle. Verified the visibility claim directly (`grep RACE_CORPUS_BOOKS race_catalog.rs` → `pub(crate) const`) before acting, per the operator's instruction not to duplicate on an assumption. Fixed by deleting the copy and importing `race_catalog::RACE_CORPUS_BOOKS`, adding `advanced_players_guide` in the one remaining place.
11. **`race_trait_picker.rs`'s second, silently-duplicated `book_code()`, found in the same edit.** `race_catalog.rs` already declared `pub(crate) fn book_code()`; `race_trait_picker.rs` carried its own private copy with the identical match arms. Checked against instance 10's stale-justification shape rather than assumed to match it: instance 10's duplication carried an explicit (and false) justification comment; this `book_code()` duplication carried **no justification at all** — just a doc comment describing what the function does, with no note that a `pub(crate)` twin already existed one file away. **A different, arguably worse sub-shape**: instance 10 was a judgement call whose stated reason went stale; this one was never a judgement call in the record at all, just a second implementation nobody flagged as a duplicate. Finding a *second* duplicated symbol in the same file, in the same edit that fixed the first, is itself the finding: the original duplication was not a one-off lapse, it was a habit applied twice without either instance being caught by review at the time. Fixed the same way: deleted the copy, imported `race_catalog::book_code` instead.
12. **`reach_gate.rs`'s `full_inventory()` is half-derived, half-hand-maintained, and nothing reconciles the two halves** (found while landing APG's ingest). `corpus_inventory()` (one of `full_inventory()`'s three sources) walks `data/corpus/` on disk and auto-registers a `Family` for any book+kind directory it finds with real JSON in it — genuinely derived, no list to update. But `reach_of()`, the function that answers "does this family reach a player," is a hand-written `match` over `(book_id, kind)` pairs. The moment `ingest_apg_race_traits.rs` wrote `data/corpus/advanced_players_guide/race_trait/`, `corpus_inventory()` correctly and automatically produced `Family::new("apg", "race_traits")` — and `reach_of()` would have silently returned `None` for it (no match arm), which every one of this gate's own tests would have read as "family declared, no claim" rather than "record on disk, unasked-about." **The dangerous shape here is not "hand-maintained" in isolation — every instance 1-11 has a hand-maintained half. It is hand-maintained *sitting beside genuinely derived*, in the same function, where the derived half moves on its own and the hand-maintained half does not, and nothing compares them.** A purely hand-maintained pair (two lists, neither derived) at least drifts symmetrically and predictably; a derived/hand-maintained pair drifts asymmetrically, silently, and exactly when new content is the reason a hand-maintained match arm was needed. Fixed by adding the missing arm (`("apg", "race_traits") => race_traits_reach("APG", "advanced_players_guide")`) and a dedicated record-by-record reach test mirroring ARG's own, rather than trusting the generic gate tests to catch a missing arm on their own.
13. **Cross-book `KEY:` collision at the ingest layer — the largest finding of the APG cycle** (2026-08-08). `decisions.md §37`'s trace counted `apg_abilities_race.lst`'s 50 racial-trait-*shaped* rows correctly but never checked whether those rows' `KEY:` values were already ingested from another book's own corpus directory. A first full ingest wrote all 50 and broke three real, correct tests (`race_trait_picker`'s attribution and description-rendering tests, `character_hub`'s creation-acceptance test) by silently overwriting 49 of ARG's 156 already-shipped, already-correct traits with APG's own two-years-older wording for the identical `KEY:` — Paizo republished the bulk of APG's (`SOURCEDATE:2010-08`) 7-CRB-race alternates in ARG (`SOURCEDATE:2012-06`), and only `Half-Orc ~ Plagueborn` is a genuinely new key. Structurally the same blind spot as the classifier's cross-book gaps (`§35`, the ARG in-scope-race check) but one layer earlier: a classifier gap over-counts or under-counts a status; this one would have shipped *wrong content* to a player, silently, for records that had been correct before the ingest ran. **The durable fix, proposed and not built this cycle** (same guardrail as `§37`'s `file_kind()` deferral): a trap in the shared ingest path that refuses to emit — or at minimum loudly reports — any corpus record whose `KEY:` already exists under a different book's directory, so this class of collision surfaces at ingest time rather than at the reach gate. `ingest_apg_race_traits.rs` implements a narrow, book-specific version of this (`already_ingested_keys()`, reading ARG's on-disk key set directly rather than hand-listing the 49 collisions) rather than the general trap, scoped to what this cycle actually needed.
14. **`race_resolver::load_race_corpus` has no book-scoping in its key space — a real latent defect, recorded even though instance 13's fix avoided triggering it.** The resolver keys alternates by trait key alone across every book in `RACE_CORPUS_BOOKS`; nothing prevents two books from declaring the same key, and nothing detects it when they do — whichever book's directory loads last silently wins. Instance 13's collision happened to be caught by tests pinning specific traits' book attribution and prose, not by the resolver itself refusing or flagging the collision. Not fixed this cycle (per the operator: it wants designing, not patching under time pressure) — but the latent defect exists independently of whether any currently-ingested book pair happens to collide today.
15. **`race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS` — the actual blocker on APG's one genuinely new trait, found while trying to ship it.** `character_hub.rs`'s creation-acceptance path validates a selected alternate trait key against `race_resolver::unknown_alternate_trait_keys`, which checks membership in `ALTERNATE_TRAIT_REPLACE_FLAGS` — a hand-written `const &[(&str, &[&str])]` listing all 153 ARG alternate keys with their replace-flags, not derived from the corpus. `Half-Orc ~ Plagueborn` is a real, correctly-ingested, picker-visible record (`decisions.md §39`), but this table does not know it, so shipping the corpus record alone would offer it in the picker and then refuse it at character-save time — a stub, not real content. **Deliberately not fixed this cycle**: extending this table is a resolution-behavior change to `race_resolver.rs`, which the operator's own guardrail for this cycle explicitly excludes ("do not change resolution behaviour this cycle"). Plagueborn's corpus record was generated, verified correct, and then withheld from this cycle's commit rather than shipped as a stub — see `§39`'s own "not landed this cycle" note. The `reach_of()` match arm for `("apg", "race_traits")` is landed regardless (harmless with 0 records today, forward-compatible), so Plagueborn's follow-up needs only to extend this table, add the corpus record, and add one reach test.

**The recognizable symptom, for the next agent:** a count or a status that looks too low (or too high) for content everyone agrees is real, where tracing the check finds it reads a small, named, hand-written Rust/JSON structure instead of calling the same function or reading the same directory the real consumer uses — **or**, per instance 7, a bound that stays satisfied while what it is bounding grows past what the literal was ever meant to cover — **or**, per instance 8, a file whose count was correct once but has no writer responsible for keeping it correct as more than one generator feeds the same directory — **or**, per instance 12, a `match`/lookup whose *counterpart* source is genuinely derived, so the pair drifts the moment new content lands rather than gradually. The fix is never "add the missing entries by hand" (that just re-creates instance 3's shape without instance 3's discipline) — it is either (a) replace the hand-written structure with a live read of the real source (instance 1's fix, instance 7's, and instance 8's), (b) if a hand-written exception list is genuinely unavoidable, make it small, named, and itself covered by a divergence test (instance 3's shape, and the model to imitate), (c) prefer a construct the compiler can make exhaustive over a `match` with a catch-all arm (instance 6's lesson) wherever one is available, or (d) before writing a small helper function, grep for its exact output shape across the tree first — the check that would have prevented instance 11.

**Cross-reference:** this decision's own preceding entries (§35, the ARG `race_trait` reconciliation and the FCB/choice-suboption fix) for the concrete SD28-E16 instance; `tests/sd_governance_stub_registry_divergence.rs` for the divergence-test pattern to imitate when a hand-written list cannot be eliminated outright.

## Decision 37 — `SpellbookCoverage.slots_total`/`.slots_used` deleted, not populated: Decision 36's ninth instance (2026-08-07)

**Status:** Closed. epic-31-spell-wiring's own closure receipt honestly recorded that `spellbook.rs:106-107` declared `slots_total: BTreeMap<u8, u8>` and `slots_used: BTreeMap<u8, u8>` and never populated either — only `spell_save_dc` was real. The desktop UI already had a conditional "Spell slots" block waiting on them (`Object.keys(spellbook.slotsTotal).length === 0 ? null : ...`), so nothing false was ever displayed, but the maps were dead.

**Investigation before building.** The desktop app already renders a separate, real "Spells per day" section (`CharacterSheet.tsx`, driven by `spellsPerDayModel.ts`'s `buildSpellsPerDaySurface`), sourced from `pilot_compute.rs`'s per-class chassis computation: real PF1 base-spells-per-day tables (`arcanist_base_spells_per_day`, `warpriest_base_spells_per_day`, `witch_base_spells_per_day_table`, `shaman_base_spells_per_day_table`, plus the wizard/cleric/druid/sorcerer/bard/paladin/ranger equivalents) combined with real per-class ability-bonus-spell application, each emitting `class_spell.<book>.<class>.total_spells_per_day.spell_level_<n>` explanation records — one of the most heavily-tested surfaces in the codebase (`tests/sd13_*`, `tests/sd18_*_widening.rs`, dozens of files). Critically, this surface is keyed by `classToken` per spell level, so it already reads correctly for a character with more than one casting class; `SpellbookCoverage.slots_total` was keyed by level *only*, with no class dimension, so populating it for a multiclass caster would have silently blended two classes' independent slot pools into one number — a strictly worse answer than what already ships.

Populating `slots_total` from real PF1 rules would therefore have meant either (a) re-deriving the same base-spells-per-day + ability-bonus tables `pilot_compute.rs` already computes, correctly, per class — a second, hand-authored copy of rules data this codebase's own `decisions.md` Decision 36 names as a recurring failure class (eight prior instances, all "a hand-maintained structure sitting beside its real, derivable source, with nothing failing when they diverge") — or (b) reaching into `pilot_compute.rs`'s already-computed `ComputationExplanation` records from within `spellbook.rs`, which has no such coupling today and collapses per-class detail the moment it's flattened into `BTreeMap<u8, u8>`. Either path creates a second source of truth for the same magnitude, discoverable to diverge the same way all eight prior instances did, with no compiler check to catch it (a `BTreeMap<u8,u8>` insert can never fail to compile the way a `match` over an enum can).

**The stronger of the two reasons, stated plainly: this is not just avoidable duplication, the field's own shape cannot correctly represent the domain.** `slots_total: BTreeMap<u8, u8>` is keyed by spell level alone — there is no class dimension anywhere in `SpellbookCoverage` or `PilotSpellbookViewModel`. A single-class caster's level-1 slots and a second class's level-1 slots (a Wizard 5/Cleric 3, say) would collide on the same map key and overwrite or sum into one number that represents neither class's real pool — there is no `insert`/`entry` policy that produces a correct answer from that shape, because the shape itself discards the information needed to be correct. This is a stronger reason to delete than "it would duplicate an existing computation": duplication could in principle be fixed by making the duplicate correct; a key collision that erases which class a slot belongs to cannot be fixed without changing the type (`BTreeMap<(String, u8), u8>` or per-class nesting), which is a real redesign, not a fill-in. The already-shipped `spellsPerDayModel.ts` surface gets this right today precisely because it keys by `classToken` first. **Anyone tempted to resurrect `slots_total` in the future must redesign its key shape before populating it — filling in the old `BTreeMap<u8, u8>` as it stood would reintroduce this collision, not just re-duplicate a computation.**

**Decision: delete `slots_total`/`slots_used` from `SpellbookCoverage`, `PilotSpellbookViewModel`, `PilotSpellbookDto` (Rust + TS), and their `printed_sheet_cell_map`/`sheet.spellbook.slots_*.*` cell generation, plus the desktop "Spell slots" JSX block that read them.** `spell_save_dc` is untouched — it is real, is not computed anywhere else in the codebase, and is the field epic-31's own screenshot verified (`Wizard DC 11`, hand-computed `10 + 1 + 0`). `bonus_slots_from_ability` (Table 1-3 ability-bonus-slot math) is also untouched — it is real, PF1-Core-Rulebook-cited, tested (`tests/sd20_spellbook_*.rs`), and out of this gap's scope; nothing downstream reads it into a cell or the UI, so there is no duplication to resolve for it today.

**Why this beats populating, concretely:** `slots_used` had no independent source anywhere else (it is a genuinely new concept — "how many of the available slots does this build's `spells_prepared`/`spells_known` actually fill"), but it only has meaning paired with a trustworthy `slots_total`, and the UI's own guard (`slotsTotal` empty ⇒ render nothing) made that pairing structural, not incidental. Deleting the pair removes a compile-time-invisible drift risk and a redundant on-screen number, while `spellsPerDayModel.ts`'s existing, real, per-class total remains the one place a player reads "how many spells can I cast today."

**Files touched:** `src/rules_core/spellbook.rs` (struct fields removed), `src/rules_core/pilot_view_model.rs` (`PilotSpellbookViewModel` fields + `from_coverage` emptiness check), `src/rules_core/contract.rs` (two doc-comment blocks + the two `slots_total`/`slots_used` cell-generation loops in `printed_sheet_cell_map`), `apps/desktop/src-tauri/src/character_hub.rs` (`PilotSpellbookDto` fields + `map_pilot_spellbook_dto`), `apps/desktop/src-tauri/src/pf1_adapter.rs` (doc comments + one test rename, no field access changed — that test only ever asserted `spell_save_dc`), `apps/desktop/src/boundary/loadCreateCharacter.ts` (`PilotSpellbookDto` TS interface), `apps/desktop/src/characterHub/CharacterSheet.tsx` (deleted the dead "Spell slots" block), `tests/sd20_contract_spellbook_wiring.rs` (removed the now-nonexistent-field assertions and rewrote its module doc comment).

**Verification discipline:** the removal is enforced by the Rust compiler at every call site (a `BTreeMap` field access on a struct that no longer has it is a compile error), not by a new runtime test — matching Decision 36's own closing guidance to "prefer a construct the compiler can make exhaustive… wherever one is available." The pre-existing negative coverage (`resolve_unified_pilot_snapshot_surfaces_no_spellbook_for_a_non_caster`, `fighter_with_no_spells_produces_an_honestly_empty_spellbook_and_zero_spellbook_cells`) already proves a non-caster surfaces no spellbook block at all and needed no change.

**Cross-reference:** Decision 36 (the eight prior instances of this same pattern — this is the ninth); `docs/release/SD-28-ultimate-book-content-ingestion/progress.md`'s `Cycle SD28-E31-F1-001` receipt (epic-31's original closure, which correctly wired `spell_save_dc` and honestly flagged this gap rather than silently leaving it); `apps/desktop/src/characterHub/spellsPerDayModel.ts` (the real, single source of truth for spells-per-day totals this decision defers to).

## Decision 37 — SD28-E16: the corpus-wide `race_trait` figure re-derived across all six remaining books; the 5,899 target does not survive at face value (2026-08-07)

**Status:** New. Answers the question §35/§36 left open: whether ARG's classifier-noise pattern generalizes. It does not generalize as a single mechanism -- it generalizes as a *symptom* with at least three different causes, one per book (or combination), and the honest conclusion is that the corpus-wide `race_trait` "not-ingested" total cannot be trusted at face value for any of the six remaining books without this per-book trace.

**Method.** Every real (non-`.MOD`, non-directive) row in each book's `_abilities_race.lst` classified by content shape -- `TYPE:...FavoredClassBonus...` (FCB), `CATEGORY:Choice` (menu sub-option), `CATEGORY:Internal` (namespace plumbing), `CATEGORY:Special Ability` / `CATEGORY:Spell-Like Ability` (real monster/racial special-ability content, wrong `Kind`), and `<Race> Racial Trait` TYPE marker (genuine racial-trait content) -- with every row landing in exactly one bucket and the buckets summing to the file's total (reconciled exactly for all six books; commands re-run and re-verifiable, not transcribed). Cross-checked each book's real-racial-trait rows' already-ingested count directly against `data/corpus/<book>/race_trait/*.json`.

**The table:**

| Book | File | Real rows | Real racial-trait rows | Already ingested | Closable gap | FCB | `CATEGORY:Choice` | `CATEGORY:Internal` | Special Ability / SLA (real, wrong `Kind`) | Other/blocked |
|---|---|---|---|---|---|---|---|---|---|---|
| ARG | `arg_abilities_race.lst` | 979 | 243 (156 in-scope + 87 out-of-scope) | 156 | **0** | 291 | 82 | 0 | 0 | 87 out-of-scope-race (Inner Sea Races, `decisions.md §35`) + ~361 unattributed Race Builder formula-table rows (unresolved, `§35` unresolved item) |
| APG | `apg_abilities_race.lst` | 176 | 50 (7 CRB races' alternates) | 0 | **up to 50** | 54 | 2 | 12 | 58 (47 + 11) | 0 |
| ACG | `acg_abilities_race.lst` | 190 | 0 | 0 | **0** | 70 | 0 | 4 | 116 | 0 |
| CRB | `cr_abilities_race.lst` | 131 | 0 | 67 (attributed via `core_essentials`'s per-race files, not this file -- see below) | **0** | 0 | 0 | 0 | 127 (8 + 119) | 4 `CATEGORY:Background` placeholder rows (`Human Ethnicity ~ None/Unknown`, `Region ~ None/Unknown`) |
| Bestiary | `b1_abilities_race.lst` | 620 | 3 (all "Drow Noble") | 0 | **0** | 0 | 0 | 76 | 541 | 3 blocked on an unmodeled monster variant (Drow Noble, structurally the same shape as ARG's Inner Sea Races exclusion) |
| PU | `pu_abilities_race.lst` | 229 | 0 | 0 | **0** | 0 | 0 | 30 | 199 | 0 |

**The one finding that matters most: only APG has a real, non-zero, closable `race_trait` gap, and it is at most 50 units, not any of the headline hundreds each book's raw `not-ingested` figure implies.** Every other book's `race_trait` bucket is either already 100% done (ARG), or contains zero genuine racial-trait content at all (ACG, CRB's own file, Bestiary, PU) -- their entire `race_trait` `not-ingested` count is noise, wrong-kind real content, or content blocked on a bundle SD-28 does not own.

**Three distinct causes producing the same symptom, not one recurring defect:**

1. **ARG's cause (`§35`):** the file genuinely mixes racial-trait rows with Favored Class Bonus and choice-suboption rows for the *same* races. Fixed via two trap rules.
2. **APG's cause:** the same FCB/Choice mixing as ARG (54 FCB, 2 Choice -- the trap rules already fixed for ARG apply here unmodified, since they are gated on `kind == Kind::RaceTrait`, not on book), *plus* a genuine, currently-`not-ingested` 50-unit alternate-racial-trait chapter that is real, closable work -- the one book where "not-ingested" partially means what it says.
3. **ACG/CRB/Bestiary/PU's cause: whole-file misclassification.** These four books' `_abilities_race.lst` files are not alternate-racial-trait chapters at all -- they are monster/creature special-ability libraries (`CATEGORY:Special Ability`, `CATEGORY:Spell-Like Ability`) or internal PCGen bookkeeping (`CATEGORY:Internal`), swept into `Kind::RaceTrait` purely because `file_kind()` matches the filename substring `_abilities_race`, independent of what the rows inside actually contain. CRB's case is sharpest: its own file has *zero* real racial-trait rows -- CRB's 67 already-ingested race_trait records come entirely from `core_essentials`'s per-race `<race>_abilities_race.lst` files via the shared-library host-attribution `classify()` already does for `Race` kind, not from `cr_abilities_race.lst` at all.

**The `CATEGORY:Special Ability` / `CATEGORY:Spell-Like Ability` rows are real content, not noise to discard -- state this plainly so the table is never misread as "1,241 units of nothing."** 47+116+8+541+199 = 911 rows across the four affected books, plus APG's own 47+11=58, total 969 real monster/creature special-ability and spell-like-ability records, genuinely part of these books, genuinely not yet represented anywhere in the engine under a correct `Kind`. They are misfiled, not fictional. A future `Kind::MonsterAbility` (or similar) ingest epic could close real work here -- but that is new scope, a new `Kind`, and a `file_kind()` redesign question (see below), not a `race_trait` backfill.

**A second gap in the trap-rule set found while building this table, distinct from anything `§35`/`§36` already named:** the existing `internal_namespace` trap only matches a first-field literal `CATEGORY=Internal|...` (the PCGen encoding where the category is embedded in the row's own name field). Every `CATEGORY:Internal` row counted in this table (12+4+0+76+30 = 122 across APG/ACG/Bestiary/PU) is a *normal* row carrying a separate `CATEGORY:Internal` field -- a different shape the existing trap does not recognize at all. This is Decision 36's pattern again, in the trap-rule set itself: a trap that silently under-fires on a shape adjacent to the one it was built for is the same class of defect as an assertion that silently under-checks (`§36` instance 7). Not fixed this cycle -- named here so it is not rediscovered as new.

**`decisions.md §35`'s Inner Sea Races `OPEN_FINDINGS` handoff gets a sibling: Bestiary's 3 Drow Noble rows need the identical treatment** -- `OPEN_FINDINGS` entry in `reach_gate.rs` naming "blocked on Drow Noble, an unmodeled Bestiary 1 monster variant" as the remedy. Same collision-boundary handoff as before (that file is not this cycle's to edit right now, per the standing territory note).

**What this means for the epic's target.** The 5,899-unit corpus-wide `not-ingested` figure (`epic-breakdown.md`'s corrected Epic 16 section) was derived from the same `v06_work_inventory` classifier this decision has now shown to substantially overcount `race_trait` across every one of the six books it covers, via at least three distinct mechanisms. **This decision does not attempt to compute a corrected 5,899 replacement** -- that requires the same row-by-row treatment for every OTHER kind (`class_feature`, `companion`, `equipment_modifier`, `monster`, `spell`, `class`, `race`, `feat`, `equipment`) across all six books, which this cycle has not done, and `race_trait` alone (per the by-kind composition `epic-breakdown.md` already carries) was 3,276 of the original 8,492, i.e. a large share but not the whole figure. What is certain: **the `race_trait` portion of that total is now known to be almost entirely noise (six books, ~2,839 of ~2,889 non-ARG-already-done `race_trait` `not-ingested` units are not real closable content by this table), and no other kind in the 5,899 figure has been given the same scrutiny.** Recommend: do not launch a per-kind ingest epic against any book's `race_trait` `not-ingested` count without first running this same row-by-row trace for that book -- the ARG/APG FCB-and-choice trap-rule fix generalizes cleanly; the ACG/CRB/Bestiary/PU whole-file-misclassification cause does not, and needs its own remedy (see below) before any of those four books' `race_trait` figures mean anything.

**`file_kind()`'s design question, deliberately not answered here.** The root cause across ACG/CRB/Bestiary/PU is that `file_kind()` classifies by filename substring match, not by row content. The fix that would close every instance of this at once -- classifying `Kind::RaceTrait` vs a new `Kind::MonsterAbility` (or similar) by each row's own `TYPE`/`CATEGORY` shape rather than by which file it lives in -- is a bigger, single design decision that should be made once against the complete six-book picture this table now provides, not iterated per-book. Explicitly deferred, per the operator's standing guardrail against compounding classifier edits, and per the judgment that this decision should inform that design rather than pre-empt it.

**Cross-reference:** `decisions.md §35` (the ARG mechanism and its fix, applied unmodified to APG); `decisions.md §36` (the nine-instance hand-maintained-structure pattern; this decision's `internal_namespace` gap finding is effectively instance 10, named but not fixed); `epic-breakdown.md` Epic 16 section (needs its own correction pass citing this table, not yet done in this commit); `progress.md` SD28-E16 receipt (the commands behind every number in the table above).

## Decision 38 — SD28-E16: `class_feature`'s `not-ingested` bucket does not collapse like `race_trait`'s; a confirmed under-reporting instance found; `OPEN_FINDINGS` is the wrong home for not-ingested findings (2026-08-07)

**Status:** New. Corrects one of this cycle's own earlier recommendations (see below) and records a genuinely different outcome from `§37`'s six-book `race_trait` result.

**Method (per the operator's explicit steer away from re-implementing the classifier).** `docs/work-inventory.json`'s per-unit `units[]` array carries a real `evidence` string per unit, written by `v06_work_inventory`'s own `classify()` -- grouped `class_feature`/`not-ingested` units by `(book, evidence)` across ARG/APG/ACG/CRB/PU (Bestiary has zero `class_feature` units) rather than inferring a mechanism from row shapes. Two clean buckets, reconciling to the full 1,671-unit total (856+652+142+... -- the exact per-book counts are in the grouped output, not transcribed here):

```
no_explanation_id_and_no_diagnostic_names_this_feature   1,259
class_feature_of_unmodelled_corpus_class:<name>            ~700 (36 distinct class names)
```

**Bucket 1 does NOT collapse the way `race_trait` did.** Sampled 10 random non-`Archetype`-shaped units from the 999 (1,259 minus 260 `Archetype`-shaped `type_facet` rows, see below) and checked each against the real corpus and, where suspicious, against `pilot_compute.rs` directly. Most are genuine, currently-unwired granular class content -- Arcanist Exploits, Alchemist Discoveries, Swashbuckler Deeds, Inquisitor Judgments/domain selections -- matching the kind of feature-by-feature wiring this program's own retrospective log already documents as ongoing, real work (Inquisitor's Judgment/Stern Gaze/Monster Lore, Skald's Damage Reduction, etc., all cited in this package's own `decisions.md` history). **This is the first kind traced in SD28-E16 whose `not-ingested` figure is mostly real.**

**But sampling also found a confirmed false negative -- genuinely wired content reported `not-ingested`.** `core_rulebook:class_feature:cleric_channel_positive_energy` ("Cleric ~ Channel Positive Energy") carries `no_explanation_id_and_no_diagnostic_names_this_feature`, yet `pilot_compute.rs:38700` computes it directly: `id: "class_chassis.cleric.channel_energy_dice"`, a real level-scaling formula (`(level_value + 1) / 2`), discussed at length elsewhere in that file (die count, effective caster level interactions). The classifier's `Kind::ClassFeature` match requires an `explanation_id` to *end with* the corpus feature's own slug (`channel_positive_energy`); the engine's chosen id ends in `channel_energy_dice` -- a different string naming the same real feature. **Genuinely wired content, invisible to the classifier purely because the corpus slug and the engine's own explanation-id suffix disagree.**

**Verification discipline worth recording explicitly, since it is what makes the finding above credible.** Before landing on Cleric's Channel Positive Energy, the same suspicion was raised and *disconfirmed* for "Wizard ~ Arcane Bond" -- `pilot_compute.rs` does discuss "Arcane Bond" extensively, but tracing the actual test names (`single_class_sorcerer_with_arcane_bond_...`) showed the wired Arcane Bond is **Sorcerer's** bloodline power, a different class's version of a same-named feature, not Wizard's. Reporting the check that came back negative alongside the one that came back positive is the discipline this whole package has been built on since `§32`; a "found it" claim with no record of the "checked and it wasn't" siblings is not distinguishable from confirmation bias.

**This is instance eleven of `decisions.md §36`'s pattern, and it is the second instance in the *under-reporting* direction, not the over-reporting direction every prior instance was.** The first under-reporting instance was `§35`'s race_trait grounding blind spot (`race_tables::race_traits()`'s 50-entry CRB-only table couldn't see ARG's JSON-driven alternates). Both share the same shape, stated as the generalization a reader should carry forward: **the classifier's notion of "wired" can be narrower than reality, exactly as its notion of "a unit" can be broader than reality.** A pattern-catalogue that only named the over-counting direction would leave a reader unable to recognize this one on sight. Sizing the true false-negative rate is not yet done -- see "Not yet done" below.

**260 of the 1,259 units are `Archetype`-shaped `type_facet` rows (e.g. `Arcanist Archetype ~ Blade Adept`, `Arcanist Archetype ~ Occultist`) -- an archetype's own identity/registration record, not a base class feature.** Structurally analogous to ARG's alternate-racial-trait shape (a distinct, real content type riding inside the same `Kind`). **Open question, not resolved here:** does an archetype's own identity record count as a `class_feature` unit at all, or is it a different content family (the archetype itself, distinct from the features it swaps)? Recommendation, not a ruling: treat it the way `race_trait` treated `CATEGORY:Choice` sub-options -- a component of the archetype concept, not an independent feature -- but this needs the same operator-level sign-off `race_trait`'s sub-choice ruling got (`§35`), not a unilateral call.

**The 36-class `class_feature_of_unmodelled_corpus_class` bucket (~700 units) is heterogeneous and the remedy differs by sub-type, corrected against this cycle's own first-pass framing:** `magus` (222), `occultist`, `psychic`, `spiritualist`, `gunslinger` are real, unmodeled PF1 *base* classes; `arcane_trickster`, `arcane_archer`, `dragon_disciple`, `eldritch_knight`, `loremaster`, `mystic_theurge`, `duelist`, `assassin`, `shadowdancer`, `horizon_walker` are unmodeled *prestige* classes; `antipaladin`, `master_chymist`, `master_spy`, `nature_warden`, `holy_vindicator`, `rage_prophet`, `battle_herald`, `stalwart_defender`, `pathfinder_chronicler` are unmodeled alternate/NPC-facing classes; `adept`, `aristocrat`, `commoner`, `expert`, `warrior` are NPC classes; and `animal`, `dragon`, `fey`, `plant`, `undead`, `construct` are **suspicious** -- these read as creature *types*, not class names, and may be a `class_feature_owner()` false-positive match (a monster-template ability group whose prefix happens to string-match a corpus-declared "class" name) rather than a genuine unmodeled class. **Not verified this cycle** -- flagged rather than asserted either way.

**Correction to this cycle's own earlier recommendation: `reach_gate.rs`'s `OPEN_FINDINGS` is the wrong mechanism for any of this, and adding entries there would break the gate.** Read `unsurfaced_families_are_exactly_the_recorded_findings` directly before acting on the earlier "add an `OPEN_FINDINGS` entry" recommendation (this decision's own prior message to team-lead, and the parallel recommendation for Bestiary's Drow Noble rows in `§37`): that test's `live_unsurfaced` set is built from `full_inventory()`'s **declared claims over records already on disk**, and asserts the recorded set is *exactly* equal to it in both directions -- a recorded finding for a family with zero live unsurfaced state fails the "stale" half of that assertion immediately. Inner Sea Races' 87 rows, Bestiary's 3 Drow Noble rows, and this decision's 36 unmodeled classes were never written to `data/corpus/` at all (`ingest_race_traits_arg.rs` deliberately never emits the 19 excluded races' rows; no `class_feature` ingest binary has ever touched `magus` etc.) -- they are invisible to `reach_gate.rs` entirely, not merely unreached. `OPEN_FINDINGS` covers "ingested but doesn't reach a player"; these are "never ingested, blocked on a different bundle or an unmodeled class" -- a `not-ingested`-classifier-level fact, correctly homed in this decisions package (here, and `§35`'s Inner Sea Races paragraph) and in `progress.md`'s receipts, not in `reach_gate.rs`. **This correction applies retroactively to `§35`'s Inner Sea Races handoff too** -- that finding is fully and correctly recorded in `§35`'s own text; no `reach_gate.rs` edit was ever the right next step for it, and none should be made.

**Done, same day, per the operator's inversion method: engine ids enumerated once, cross-referenced, and deliberately NOT hand-verified past a bounded candidate list.** `grep -oE 'id: "(class_chassis|class_feature)\.[a-z0-9_.]+"' src/rules_core/pilot_compute.rs | sort -u` -> **567** distinct engine explanation ids. Cross-referenced against the 999 non-`Archetype` `not-ingested` `class_feature` units (same-class-prefix match + non-trivial token overlap between the corpus feature's own slug and the engine id's tail): **82 raw candidates**, **69** after excluding a small generic-word stopword list (`base`/`attack`/`combat`/`training`/`improved`/`bonus`/`feat` -- words common enough to produce spurious matches like `Druid ~ Wild Shape` against `wild_empathy`, a genuinely different feature sharing only "wild"). This independently reproduces the Channel Positive Energy instance above, and additionally surfaces two siblings not checked by hand: **`Cleric ~ Channel Energy` and `Cleric ~ Channel Negative Energy` both match the same `channel_energy_dice`/`channel_energy_uses_per_day` engine ids** -- three separate corpus names very likely naming one wired feature. That is a better illustration of the mechanism than a bare count: it suggests the durable fix is **identifier agreement between corpus and engine**, not a smarter matcher layered on top of disagreement.

**Deliberately stopped at the candidate list, not extrapolated into a corrected count.** Even if every one of the 69 confirmed, that is ~7% of the 999 -- the headline ("`class_feature`'s gap is substantially real") does not move, and hand-verifying each one would spend real time for no decision anyone is waiting on. The candidate list, the method (reproducible with the one command above plus the cross-reference script), and the one hand-confirmed instance are the durable outputs; the exact count is explicitly left as a bounded, reproducible follow-up, not a gap in this decision. Whoever eventually designs the identifier-agreement fix has a ready-made fixture set in these 69.

**A twelfth suspected instance, flagged and explicitly not verified.** The `animal`/`dragon`/`fey`/`plant`/`undead`/`construct` entries in the 36-class list above read as creature *types*, not class names -- a `class_feature_owner()` false-positive match (a monster-template ability group's prefix string-matching a corpus-declared "class" name) is the likely mechanism, structurally similar to instance 6's silent-`None`-on-drift shape but in the opposite direction (a spurious match rather than a spurious miss). Left as flagged-not-verified rather than asserted.

**Not fixed this cycle, by design:** the explanation-id naming-mismatch detection gap. Per the same guardrail `file_kind()`'s redesign question was deferred under (`§37`) -- the fix wants designing against the full candidate list this decision's "not yet done" item would produce, not against the one instance sampled here, and the better fix may not be matching-logic at all (the engine and corpus agreeing on identifiers is a different, and possibly better, conversation than a smarter matcher).

**Cross-reference:** `decisions.md §35` (the `race_trait` grounding blind spot, this decision's sibling under-reporting instance); `decisions.md §36` (the nine-instance catalogue; this decision's finding is its instance eleven); `decisions.md §37` (the six-book `race_trait` table this decision's method deliberately diverges from, per the operator's own steer, once the row-shape approach proved too weak a proxy for `class_feature`); `apps/desktop/src-tauri/src/reach_gate.rs`'s `OPEN_FINDINGS`/`unsurfaced_families_are_exactly_the_recorded_findings` (read, not edited, in the course of this correction).

## Decision 39 — SD28-E16: APG's real closable `race_trait` gap is 1, not 50 -- a cross-book `KEY:` collision, corrected before it shipped (2026-08-08)

**Status:** New. Corrects `decisions.md §37`'s own published figure for APG. The corpus-wide `race_trait` picture across all six remaining books is now **1 real closable unit identified, 0 landed this cycle** -- a further ~98% correction on top of `§37`'s own headline (3,276 originally reported -> 50 traced-real -> 1 confirmed-real), and that 1 unit is itself blocked on a separate hand-maintained table (`decisions.md §36` instance 15) rather than shipped.

**What happened.** `§37` traced all six books' `_abilities_race.lst` files and found APG the only one with real, non-zero racial-trait-shaped content: 50 rows carrying a genuine `<Race> Racial Trait` TYPE marker for the 7 CRB races, none previously ingested. `ingest_apg_race_traits.rs` ingested all 50. The desktop test suite caught the defect immediately: three real, previously-passing tests failed --

- `race_trait_picker::every_alternate_carries_arg_attribution_real_prose_and_a_real_page`: `Dwarf ~ Ancient Enmity`'s recorded book flipped from `"ARG"` to `"APG"`.
- `race_trait_picker::every_menu_row_has_a_rendered_description_and_none_leaks_pcgen_syntax`: the same trait's served description text changed.
- `character_hub::every_alternate_the_picker_offers_for_a_crb_race_is_one_creation_accepts`: `Half-Orc ~ Plagueborn` (a genuinely new APG trait) was offered by the picker and refused by character creation.

**Root cause, verified directly against both corpora, not assumed.** `arg_abilities_race.lst:33` declares `KEY:Dwarf ~ Ancient Enmity`, `SOURCEPAGE:p.11` -- the identical key, race, and page as the APG row just ingested, with near-identical text (APG: "+1 bonus ... replaces the hatred racial trait"; ARG: "+1 racial bonus ... replaces hatred"). Full comparison of both books' key sets: `ARG corpus keys: 156, APG corpus keys: 50, collisions: 49`. **Only `Half-Orc ~ Plagueborn` is unique to APG.** Paizo republished the bulk of APG's own 7-CRB-race alternate traits in ARG with minor copy-edit revisions -- confirmed by publication date, not memory: `apg_abilities_race.lst`'s `.pcc` states `SOURCEDATE:2010-08`; `arg_abilities_race.lst`'s states `SOURCEDATE:2012-06`. **ARG is the later book.** APG's wording is therefore the *older* text; the collision is ARG revising APG, not APG anticipating ARG, and shipping APG's copy over ARG's current wording would have been a two-year regression presented as new content.

**Why this reached the tests and not a build error.** `race_resolver::load_race_corpus` keys alternates by trait key alone across every book in `RACE_CORPUS_BOOKS`, with no book-scoping and no collision detection (`decisions.md §36` instance 14). Loading both ARG's and APG's `race_trait/` directories let APG's rows silently shadow ARG's wherever the keys matched -- correct-looking data replaced with different correct-looking data, the specific shape `decisions.md`'s own doctrine on corrupted/regressed text (`§33`'s `Fearless Zeal`/`Magnum Opus` ruling) exists to prevent, just arriving from a different mechanism (a second book's ingest, not a corpus splice).

**The fix.** `ingest_apg_race_traits.rs` now reads ARG's on-disk key set (`already_ingested_keys()`, walking `data/corpus/advanced_race_guide/race_trait/`'s real `data.key` fields, not a hand-listed exclusion set) and excludes any APG row whose key collides, before writing anything. Re-run: 1 record generated and verified correct (`Half-Orc ~ Plagueborn`), 49 collisions logged by name in the run's own output. `decisions.md §36` instances 13 and 14 record the general pattern (a durable ingest-time collision trap, and the resolver's latent lack of book-scoping) as proposed-not-built, per the same guardrail that deferred `§37`'s `file_kind()` redesign.

**Plagueborn itself is not landed this cycle either, for an unrelated reason found while trying to ship it.** `character_hub.rs`'s creation-acceptance path validates a selected alternate trait against `race_resolver::ALTERNATE_TRAIT_REPLACE_FLAGS` (`decisions.md §36` instance 15) — a hand-written, ARG-only table that does not know Plagueborn's key. Shipping the corpus record without that table knowing it would offer Plagueborn in the picker and refuse it at character-save time: a stub, not real content, exactly what this program's own doctrine forbids. `character_hub.rs` and `race_resolver.rs` became available mid-cycle, but a one-entry patch to `ALTERNATE_TRAIT_REPLACE_FLAGS` would **perpetuate instance 15 rather than fix it** -- it is the same hand-written-table shape every other instance in this decision names, and adding one more entry by hand teaches nothing that prevents a seventeenth. The corpus record was generated and verified but **deliberately withheld from this cycle's commit** rather than shipped as a one-off patch. Landed instead: the `RACE_CORPUS_BOOKS`/`book_code` dedup (instances 10-11), the `reach_of()` match arm for `("apg", "race_traits")` (harmless with 0 records today, forward-compatible), `ingest_apg_race_traits.rs` itself (real, tested, correctly emitting 0 shippable records today -- not dead code, a working tool whose one real output is blocked), and this decision's own correction — all real, all independent of whether Plagueborn ships this cycle or next.

**Explicit follow-up scope, not built this cycle:** derive `ALTERNATE_TRAIT_REPLACE_FLAGS` from the corpus the same way `already_ingested_keys()` now derives its exclusion set (reading each in-scope trait's real `sets_replace_flags` field off disk, across every book in `RACE_CORPUS_BOOKS`, rather than a hand-typed 153-entry ARG-only table), then run `ingest_apg_race_traits.rs` and add one reach test. That closes the loop this cycle opened rather than adding a 154th hand-typed entry to a table this decision has just spent four instances arguing against extending by hand. **`Half-Orc ~ Plagueborn` is, as of this decision, the one remaining real, closable `race_trait` unit in the entire six-book corpus this epic traced** -- name it as such when that follow-up lands, since it is the last piece of the headline number.

**`already_ingested_keys()` is instance-prevention, not instance-cataloguing, and is worth distinguishing from the other fourteen for that reason.** Every other instance in `§36` was found after the fact and either fixed once or proposed-not-built. This one is a function that, by construction, cannot go stale the way `ALTERNATE_TRAIT_REPLACE_FLAGS` has: it reads ARG's real on-disk keys at ingest time, so a future ARG revision changes what it excludes automatically, with no second place to remember to update — and `already_ingested_keys_reads_real_keys_off_disk_not_a_hand_list` pins that property directly rather than merely pinning today's count. This is the first artifact this cycle produced that stops a future instance rather than naming a past one.

**Four hand-maintained lists found in one feature area (the ARG/APG alternate-racial-trait surface) is itself worth stating as its own observation, not just four separate instances.** `race_trait_picker.rs`'s duplicated `RACE_CORPUS_BOOKS` (instance 10), its duplicated `book_code()` (instance 11), `reach_gate.rs`'s hand-written `reach_of()` match sitting beside a derived corpus scan (instance 12), and `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS` (instance 15) are four independent instances of the same pattern clustered in one player-facing feature, not four unrelated coincidences. A feature area that has accumulated this many hand-maintained mirrors of the same underlying corpus is a signal worth carrying forward on its own: the next book this feature area gains (a hypothetical third source of alternate racial traits) will need all four checked, not just the one that happens to break a test first.

**A structural note on how this was caught, worth keeping.** The bug was not caught by anything this cycle wrote to catch it -- it was caught by three *pre-existing* tests whose pinned values happened to name the exact records the collision touched. That is closer to luck than to a designed safety net: a collision touching a trait none of `race_trait_picker.rs`'s per-trait tests happened to pin (most of the 49 do not get individual assertions) would have shipped silently. This is the argument for instance 13's proposed general trap being worth building, not merely proposing -- the current safety margin for this class of defect is "however many of the colliding records a prior cycle happened to write a specific test for," which is not a designed property.

**Retro correction filed** against `decisions.md §37`'s own published "APG: up to 50" figure -- `--claimed "up to 50"`, `--actual "1, 49 of the 50 are cross-book KEY collisions with already-ingested ARG content"`, `--verified-by` the key-set diff and publication-date check above.

**Cross-reference:** `decisions.md §37` (the figure this decision corrects); `decisions.md §36` instances 13-14 (the general pattern and the resolver's latent defect); `decisions.md §33` (the doctrine on never letting regressed/older text overwrite correct text, applied here to a cross-book collision rather than a corpus splice).
