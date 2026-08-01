# SD-28 Decisions

**Status:** Stub. Operator-pinned pending.

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

8 epics / 30 criteria. Epic 1 = Code-Side Identifier Cleanup. Epic 2 = Operator Pre-Launch. Epic N = Closure Epilogue.

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

**Operator-on-file override.** The published SD-22 / SD-27 chassis templates use `codex-tranche-<N>` as the convention slug for the kanban-board name. SD-28 inherits the slug format (`codex-tranche-8`) and the corresponding Hermes-board-instance identifier — see Decision §15a below for the Hermes-board status.

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
- Content genuinely reaching a player surface, per `reach_gate.rs`'s `OPEN_FINDINGS` mechanism (spot-checked against the live IPC/UI path, not just the gate's exit code).
- Test quality, not just count — per `docs/governance/book-ingestion-playbook.md §7.4`'s mutation-test pattern, a sample of new gates/tests is checked for a case that actually fails when the thing it protects is broken.
- No hand-authored rules data in the frontend (`apps/desktop/src/`).

**Mechanism — wired into what already exists, nothing invented fresh:** the review runs `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` (this bundle's standing per-cycle dual-audit, per the `AT-28-004`/`decisions.md §6` identifier discipline and wired-integration doctrine already in force) against the **whole-bundle diff**, not a single cycle's slice: `git diff origin/develop...HEAD` — the same merge-base triple-dot comparison both scripts already default to via `BASE_BRANCH=origin/develop`. No new grep/audit tooling is invented; Epic 12 reuses the standing per-cycle gates at bundle scope and adds the manual/agent-driven judgment a grep cannot do (corpus-correctness sampling, reach-claim spot-check, test-quality sampling).

**Findings are triaged, not auto-fixed.** Each finding records a severity and a disposition: `fixed-in-bundle` or `deferred`. A `deferred` finding names an owner (a person or a specific successor bundle) and lands in `forward-scope-register.md` — an unowned deferral is not a valid disposition. Real defects are fixed in-bundle before Epic 10 fires.

**Operator escalation path, not a substitute:** the operator can separately trigger `/code-review ultra`, a multi-agent cloud review of the branch. That path is operator-triggered and billed — a cycle running under §21's unattended-mode protocol cannot launch it itself — so Epic 12 must stand on its own as the bundle's actual gate.

**Cross-reference:** `epic-breakdown.md` Epic 12; `acceptance-and-verification.md AT-28-013`; `docs/governance/no-stub-mvp-doctrine.md`; `docs/governance/book-ingestion-playbook.md §7.4`; `reach_gate.rs`; `kanban.md` card `epic-12-code-review`.

