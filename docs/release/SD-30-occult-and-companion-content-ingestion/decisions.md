# SD-30 Decisions

**Status:** Planning-ready (operator directives 2026-08-01; canonical chassis landing this cycle). Refined 2026-08-01 (book list confirmed; tranche/10 + no-Hermes-board + 0.10.<build> + reach-gate DoD doctrine applied per the 2026-08-01 amendments shared with SD-28/SD-29).

## Decision 1 — Book list CONFIRMED 2026-08-01

**Status:** Operator-pinned, **confirmed 2026-08-01.**

**Decision:** SD-30 ships content-source ingest for the following sixteen books, with NPC Codex + Planar Adventures + Occult Origins + Haunted Heroes deferred to `forward-scope-register.md C2.x` per the 2026-08-01 absent-book rule:

1. **Occult Adventures** — per-class cycles + per-monster-block cycles + per-psychic-discipline cycles. Corpus dir: `roleplaying_game/occult_adventures/`.
2. **Horror Adventures** — per-monster-block cycles + per-haunt-block cycles + per-corruption-mechanic cycles. Corpus dir: `roleplaying_game/horror_adventures/`.
3. **Mythic Adventures** — per-mythic-path cycles + per-monster-block cycles. Corpus dir: `roleplaying_game/mythic_adventures/`.
4. **Monster Codex** — per-monster-block cycles. Corpus dir: `roleplaying_game/monster_codex/`.
5. **Book of the Damned Vol. 1** — per-archetype cycles + per-monster-block cycles + per-tactic cycles. Corpus dir: `campaign_setting/book_of_the_damned_volume_1/`.
6. **Book of the Damned Vol. 2** — same as vol. 1. Corpus dir: `campaign_setting/book_of_the_damned_volume_2/`.
7. **Inner Sea World Guide** — per-trait cycles + per-feat cycles + per-region cycles. Corpus dir: `campaign_setting/inner_sea_world_guide/`.
8. **Inner Sea Combat** — per-trait cycles + per-option cycles. Corpus dir: `campaign_setting/inner_sea_combat/`.
9. **Inner Sea Faiths** — per-deity cycles + per-trait cycles + per-option cycles. Corpus dir: `campaign_setting/inner_sea_faiths/`.
10. **Inner Sea Gods** — per-deity cycles + per-domain cycles. Corpus dir: `campaign_setting/inner_sea_gods/`.
11. **Inner Sea Magic** — per-spell cycles + per-magic-trait cycles. Corpus dir: `campaign_setting/inner_sea_magic/`.
12. **Inner Sea Races** — per-race cycles + per-archetype cycles. Corpus dir: `campaign_setting/inner_sea_races/`.
13. **Inner Sea Temples** — per-temple cycles + per-trait cycles. Corpus dir: `campaign_setting/inner_sea_temples/`.
14. **Inner Sea Taverns** — per-tavern cycles + per-event cycles. Corpus dir: `campaign_setting/inner_sea_taverns/`.
15. **Inner Sea Bestiary** — per-monster-block cycles. Corpus dir: `campaign_setting/inner_sea_bestiary/`.
16. **Inner Sea Intrigue** — per-trait cycles + per-faction cycles + per-rule cycles. Corpus dir: `campaign_setting/inner_sea_intrigue/`.

**Deferred (NOT in scope):** NPC Codex, Planar Adventures, Occult Origins, Haunted Heroes Handbook. Recorded in `forward-scope-register.md C2.x` as future-acquisition candidates. Per the 2026-08-01 absent-book rule, these drop from scope when the corpus directory does not exist.

**Per-book path locations under `src/rules_core/rules_tables/<book>/`** are in the §"Book list" table in `scope-draft.md`.

## Decision 2 — Branch and board [SUPERSEDED — see §14]

**Status:** Doctrine-of-record (per SD-22 doctrine); **superseded 2026-08-01** by Decision §14, which tightens the rule.

**Original text:** SD-30 launches on `tranche/6-2` branch + `codex-tranche-6-2` board.

**Why superseded.** SD-28 broke the `tranche/6` family on 2026-08-01 (`tranche/8`). SD-29 followed at `tranche/9`. SD-30 takes `tranche/10`. The `codex-tranche-<N>` slug is reserved-as-form, not as-instance (the Hermes board is retired).

## Decision 3 — Build version target [SUPERSEDED — see §15]

**Status:** Doctrine-of-record (per SD-22 doctrine); **superseded 2026-08-01** by Decision §15, which applies the 2026-07-17 build-version amendment.

**Original text:** `0.6.<build>` first concrete value. tranche-base = 6 per `<major>.<tranche-base>.<build>` scheme.

**Why superseded.** SD-30's tranche-base is 10, not 6. First concrete value is `0.10.<build>` per Decision §15.

## Decision 2 — Branch and board

**Status:** Pending operator confirmation.

**Candidate:** `tranche/6-2` branch + `codex-tranche-6-2` board.

**Rationale:** SD-28 proposes `tranche/6`, SD-29 proposes `tranche/6-1`. SD-30 follows the dash-1 sub-release pattern at `tranche/6-2`. Operator-pinned pending.

**Alternative:** SD-30 could split per-book across sub-tranches (e.g., `tranche/6-2-oa`, `tranche/6-2-oo`, `tranche/6-2-ha`). Operator preference.

## Decision 3 — Build version target

**Status:** Pending operator confirmation.

**Candidate:** `0.6.<build>` first concrete value.

**Rationale:** Same base digit as SD-28 and SD-29 because all three packages land on the `tranche/6` family. Per the `<major>.<tranche-base>.<build>` scheme, tranche-base = 6 for `tranche/6`, `tranche/6-1`, `tranche/6-2`. Major stays `0` until first main-publish.

**Operator-pinned values needed:**

- Confirm the current build counter value (read from the version-bump contract in the repo's release workflow).

## Decision 4 — Epic structure

**Status:** Doctrine-of-record (per SD-22 doctrine).

8 epics / 30 criteria. Epic 1 = Code-Side Identifier Cleanup. Epic 2 = Operator Pre-Launch. Epic N = Closure Epilogue. Optional Epic 6 (Haunted Heroes Handbook) per operator-pinned in-scope decision.

## Decision 5 — Cross-bundle class overlap with SD-28

**Status:** Doctrine-of-record (per SD-22 doctrine).

For classes that appear in both Ultimate Intrigue and Occult Adventures (Occultist, Spiritualist, Medium, Mesmerist), the canonical class definition lives in SD-30 (Occult Adventures is the primary hardcover defining those classes). SD-28 references the canonical class id from SD-30's progress file but does not redefine.

## Decision 6 — Cross-bundle monster overlap with SD-29

**Status:** Doctrine-of-record (per SD-22 doctrine).

For monsters that appear in both Horror Adventures and Bestiary 2-5, the canonical monster definition lives in whichever book first introduces the monster. SD-29 references the canonical monster id from SD-30's progress file but does not redefine.

## Decision 7 — Identifier discipline

**Status:** Doctrine-of-record (per SD-22 doctrine).

- Source-code identifiers describe WHAT the artifact does, NOT which release / spec domain it came from.
- PascalCase for functions / methods / constants / properties / Tauri commands.
- lowercase camelCase for variables.
- Forbidden patterns: `sd30_*`, `SD30_*`, `Sd30*`, `sd30-*`, `t_<hex>`, `SD-30-Ex...`, `AV-PAY-N`.
- Doctrine-of-record at `~/workspace/governance/identifier-discipline.md`.

## Decision 8 — Operating form [SUPERSEDED — see §22]

**Status:** Doctrine-of-record (per SD-22 doctrine); **superseded 2026-08-01** by Decision §22, which replaces the dispatch mechanism named here with the `Workflow` tool.

`/loop 60m /batch /goal <loop-instruction-file>`. Not ad-hoc single-task invocations.

## Decision 9 — Verification is `./scripts/verify.sh`, not a hand-composed run

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

## Decision 10 — The pre-ingest trap report is mandatory

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

Two traps are especially load-bearing for this bundle. **Trap 4
(`ArchetypeScoped`)** — archetype records pose as base-class content, and
SD-30's ingest subtypes are heavy on archetypes and class options; the recorded
worked example is a Bloodrager `DEFINE:RagePowersLVL|0` that sits on
`KEY:Bloodrager Archetype ~ Primalist`, not on the base class. **Trap 5
(`SharedNameDistinctRecords`)** — `KEY:Bard ~ Lore Master` and
`KEY:Skald ~ Lore Master` are different records; Decision 6's cross-bundle
class-overlap rule must join on `KEY:`, never on display name.

`cargo run --locked --bin v06_corpus_trap_report -- --audit` is additionally a
definition-of-done condition: it exits `2` when an already-ingested record cites
a corpus line that does not resolve.

**Authority:** `src/pcgen_import/corpus_traps.rs` (the trap catalogue and the
corpus evidence for each), `src/bin/v06_corpus_trap_report.rs`.

## Decision 11 — The reach gate is a definition-of-done condition

**Status:** Doctrine-of-record (repo tooling). **Carries an open operator question — see below.**

**Decision:** A book's ingest cycle is not done until every one of that book's
record families reaches a player surface, proven by a claim in
`apps/desktop/src-tauri/src/reach_gate.rs` that executes the real IPC builder.
Ingestion and surfacing are one unit of work, not two.

A count does not satisfy the gate (`corpus_ingest_diagnostic` already carries
every book's record count and renders nothing), and an identifier alone does
not (that is the Feats-tab defect, where the player saw `feat:deflect_arrows`
in place of a name and description).

**Open operator question this bundle cannot decide for itself.** SD-30's epic
structure contains no surface-building epic, and two of its declared ingest
subtypes hit known gaps: no monster record reaches a player today (Bestiary 1's
41 stat blocks are pinned in `OPEN_FINDINGS`), and haunts / corruptions /
psychic disciplines are record kinds the gate does not yet know — an
unrecognized record type is a **hard failure by design**, because a genuinely
new kind of content needs a decision about where it reaches, not a default.
**The operator decides whether those surfaces land inside SD-30 or as named
prerequisites outside it; this package does not add an epic on its own
authority.** Skipping them is not available.

**Authority:** `apps/desktop/src-tauri/src/reach_gate.rs` (`OPEN_FINDINGS`,
`RECORD_TYPE_KINDS`), `docs/governance/book-ingestion-playbook.md` §3.

## Decision 12 — Per-entity counts are generated, never hand-maintained

**Status:** Doctrine-of-record (repo tooling).

**Decision:** This package records no per-entity count. `scope-draft.md`
§"Book shape" points at `cargo run --locked --bin v06_work_inventory` and the
`docs/work-inventory.json` it generates. Cycle receipts cite the command that
produced any figure they publish.

**Why.** Every hand-maintained artifact in this project has drifted and then
actively misled — a dashboard claimed 12 finished classes when 5 was true; a
coverage matrix read 1 wired feature where the code had 6; shipped deferral
strings still claim engines do not exist that do. The generator's own contract
is that it never invents a unit and never invents a status: a record it cannot
classify is emitted as `unknown` with the reason attached, because an honest
unknown beats a confident wrong entry.

**Authority:** `src/bin/v06_work_inventory.rs`, `docs/work-inventory.json`,
`docs/governance/book-ingestion-playbook.md` §6.

## Decision 13 — Branch and board (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.**

**Decision:** SD-30 launches on `tranche/10` branch with `kanban.md` + `progress.md` local-file dispatch (no Hermes board).

The prior candidate (per the 2026-07-28 stub) was `tranche/6-2` + `codex-tranche-6-2`; SD-30 takes its own tranche (`tranche/10`) parallel to SD-28's `tranche/8` and SD-29's `tranche/9`. The Hermes board is retired per operator directive 2026-08-01, applied uniformly to SD-28, SD-29, and SD-30.

**Why this differs from SD-22/SD-28's `tranche/8` / SD-29's `tranche/9`.** SD-30 handles a wider scope of content sources (occult + mythic + Inner Sea series + Monster Codex + Book of the Damned), making it the largest `forward-scope-relative-to-tranche-base` bundle of the post-2026-08-01 trio. The `tranche/10` slot is operator-pinned on 2026-08-01; it is not an increment of any prior tranche.

## Decision 14a — Hermes board retired (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.** Cross-cutting — affects Decision §7 (operating form), §13 above, and the loop-instruction pre-launch checklist.

**Decision:** SD-30 has no Hermes kanban board. The work-queue artifact is a local-file `kanban.md` paired with `progress.md`. Cycle dispatch reads `kanban.md` at top of each tick; supervisor's file-touch partition enforces 1-cycle-per-file.

## Decision 15 — Build version target (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.**

**Decision:** SD-30's first concrete build value is `0.10.<build>`, where `<build>` is the current build-counter state at the time of cycle close.

Per the 2026-07-17 build-version amendment:
- **major** = 0 (no main-publish yet).
- **tranche-base** = 10 (the base digit of `tranche/10`).
- **build** = monotonic counter, never resets.

Tranche-promotion increments only on `tranche/10 → develop` PR. The closure Epic (last in order) value is `0.10.<last_build>`.

## Decision 16 — Cross-book conflict rule (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.** (same doctrine as SD-28 §16 and SD-29 §16.)

**Decision:** When SD-30's books conflict with another book (cross-bundle or cross-SD-N) on a record, **the newer book is doctrine and the older book is errata.**

This supersedes any prior cross-book conflict handling in the bundle.

**Operator-confirmed precedence (operator 2026-08-01).** "Those recently published take precident." SD-30's most-recently-published reference points are SD-28 (tranche/8, 0.8.x) and SD-29 (tranche/9, 0.9.x). When SD-30's books (occult + mythic + Inner Sea) conflict with records those bundles publish, SD-28/SD-29's records are doctrine; SD-30 references the canonical id only.

This is the cross-bundle application of §16: in addition to record-level overlaps (reprints, errata), SD-30's cross-bundle conflicts with the already-published SD-28 + SD-29 surface resolve in favor of those surfaces, because those surfaces were published more recently.

The class-grant overlap rule (canonical class definition lives in the bundle that owns the book's primary class definition; the other bundle references the canonical id only) is the only exception. SD-30 owns canonical class definitions for the four shared classes that appear in both Ultimate Intrigue (SD-28's territory) and Occult Adventures (SD-30's territory): Occultist, Spiritualist, Medium, Mesmerist. SD-28 references the canonical class id from SD-30's progress; SD-30 does not redefine.

## Decision 17 — Bulk modifications deferred (operator directive 2026-08-01)

**Status:** Operator-pinned, **forward-leaning acknowledgement.**

**Decision:** The per-cycle mode of operation (one record-at-a-time, file-touch partition, individual cycle receipts) is preserved for SD-30. Bulk-modification tooling is not in scope; a future retrofit (e.g., wiring a Mythic Adventures mythic-path catalog of 30+ entries in one cycle-batch) is reserved outside this bundle.

## Decision 18 — Reach gate is the definition of done; engines only when strictly necessary (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01** as the **prime rule** for SD-30. **Supersedes Decision §12 (the prior "Build no execution engines" rule).** Cross-cutting — affects every per-book cycle.

**Decision:** A record's ingest cycle is **not done** until it satisfies `apps/desktop/src-tauri/src/reach_gate.rs`. Reach is the operator-visible definition of done.

**Engine policy.**

- **Real-time engines are out of scope.** No cycle in this bundle builds an RNG, opponent-state, or turn-sequencing engine.
- **Rules-data engines are in scope and often unnecessary.** When a numerical effect can be pre-computed as data (e.g., a Mythic tier's `+5` damage bonus posted as `15` total damage at the documented CL), post the calculated value in the description; the player rolls physical dice.
- **Engine construction is permitted only when strictly necessary to satisfy reach.** If a record's effect cannot be represented as data without an unjustifiable loss of fidelity, the cycle may build a small rules engine to model it. The engine must be enumerable, testable, and observable from `reach_gate.rs`.

**What this changes.** §12's blanket "no engines" rule was too coarse. §18 narrows it to real-time engines. Reach remains the gate; pre-computed values are preferred.

**Mythic Adventures reach prerequisite.** Mythic Adventures' reach surfaces are existential (the mythic path mechanics + tier features + monster stat blocks all require consumer integration). Per reach-gate = DoD doctrine, Mythic Adventures' ingest cycles pause on `decision-blocked` if no consumer surface reaches the gate; if `reach_gate.rs OPEN_FINDINGS` lists missing surfaces for Mythic Adventures, cycles either record the gap or build the missing surface (the latter is a runtime-engine or surface-extension decision, operator-pinned per cycle).

**Authority:** operator verbatim 2026-08-01: "reach gate is the definition of done, if an engine is required to get there, then we generate the engine — that said, often an engine isn't strictly necessary."

## Decision 19 — Operator ack-chain recorded (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01** as a forward-leaning ack chain.

**Ack ledger.** SD-30's twelve-item directive (operator 2026-08-01) confirmed: book list (sixteen books, four deferred) confirmed with cycle-0 trap-report + work-inventory gating (Item 1); `tranche/10` and no-Hermes-board confirmed (Items 2-3); "correct" and "correct for now" items acked without specific directives (Items 4, 8-10); cross-book conflict rule (Item 5); "recently published takes precident" rule (Item 6); bulk-modifications deferred (Item 7); reach-gate doctrine + the "prime rule" framing (Items 11, 13).

"Correct for now" items 8-10 are forward-leaning acks: per-record decisions remain operator-pinned at cycle dispatch. The "prime rule" framing (Item 11) is captured verbatim in §18.

## Decision 20 — Cross-reference

- `./scope-draft.md` — committed scope shape, sixteen books confirmed.
- `./loop-instruction.md` — per-cycle procedure; updated for `tranche/10`, no-Hermes-board, local-file dispatch.
- `./forward-scope-register.md` — successor work depending on SD-30's output.
- `./kanban.md` — local-file work queue (replaces Hermes board).
- `./epic-breakdown.md` — 9+ epics × ~3-4 criteria each.
- `~/workspace/programs/codex/requirements/SD-22-.../decisions.md` — predecessor doctrine for the per-book ingest pipeline.
- `~/workspace/programs/codex/requirements/SD-28-.../decisions.md` — sister bundle (cross-bundle class-grant doctrine).
- `~/workspace/programs/codex/requirements/SD-29-.../decisions.md` — sister bundle (bestiary 2-5 ingest pipeline).
- `apps/desktop/src-tauri/src/reach_gate.rs` — definition-of-done surface for §18.
- `docs/governance/book-ingestion-playbook.md` — playbook of record.
- `~/workspace/programs/codex/requirements/SD-30-.../artifacts/tranche-2-7-legacy/` (note: this directory is hosted by SD-22 at `docs/release/SD-22/artifacts/tranche-2-7-legacy/`) — historical receipts from the original PCGen corpus-ingestion tranche; SD-30 references when cycle-0 trap-report runs against the same `data/corpus/<book>/` paths.

## Decision 21 — Unattended mode authorization (operator directive 2026-08-01)

**Status:** Operator-pinned, **confirmed 2026-08-01.** Load-bearing for the bundle's cycle dispatch.

**Decision:** This bundle operates in **unattended mode**. The operator is out of town and may not see the harness's output for days. Cycles MUST NOT pause to ask the operator questions; the operator's review happens after return.

**Operating protocol during unattended mode (codified in `loop-instruction.md` §"OPERATING METHOD" sub-callout).**

1. **Default-and-flag, not ask.** When a cycle needs a decision, pick the safer default, capture it in `progress.md`, and continue.
2. **No `clarify` tool calls.** The operator clarification tool is hard-banned under unattended mode.
3. **Blockers are recorded, not raised.** Hard-blocks (auth failure, branch creation conflict, identity conflict on disk) go in `progress.md` with the command and exit code. The bundle does not halt; the supervisor picks up the next ready card.
4. **`decision-blocked` IS allowed.** Operator-decision points (Mythic Adventures consumer surface in-scope-vs-separate; psychic-discipline consumer surface; Inner Sea campaign-tool surface; closure operator-on-call) record `decision-blocked` in `progress.md` and proceed on the safe default per `forward-scope-register.md C3.x` retrofit.
5. **Closure is a goal, not a stop signal.** The bundle runs to closure under the dispatcher's own loop (the `Workflow` tool per §22, not a human re-invoking a slash command per cycle).

**Operator's verbatim:** "include instructions to all 3 that indicate they will be running in unnattended mode since i will be out of town while this runs. They may not stop to ask questions - it might be days before i notice."

**Cross-reference:** the doctrine is mirrored in `loop-instruction.md` (cycle supervisor reads it first) and `progress.md` (per-cycle receipt confirms the operator-on-record). The receipt chain is the operator's after-return review surface.

## Decision 22 — Dispatch is session-driven `Workflow`-tool orchestration, not `/loop` or `/batch` (adopted from SD-27 `decisions.md §19`, 2026-08-01)

**Status:** Operator-pinned by inheritance — SD-27 `decisions.md §19` records the correction ("adopted from SD-26 `decisions.md §13`"); this package had not yet propagated it before this pass. No new operator input required; this is process alignment, not a new ruling.

**Decision:** SD-30 dispatches via the **in-harness `Workflow` tool, driven from a live session** — not `scripts/workflow-dispatch.sh` or any headless script, and not a cron driver. Deterministic control flow (per-epic ordering, fan-out, `decision-blocked` handling) is written into `loop-instruction.md` and tracked as state in `kanban.md`'s claim/complete queue; model judgment lives inside the dispatched `agent()`/`Workflow` calls, never in the orchestrating session's own tool calls.

`/batch` defaults to parallel isolated-worktree fan-out. It is used only where an epic's criteria are genuinely file-disjoint (see `loop-instruction.md` "Epic ordering"); where cycles touch shared state — `progress.md`, `kanban.md`, `reach_gate.rs`'s `OPEN_FINDINGS` — the correct dispatch is an explicit single-cycle procedure, not `/batch`. Any parallel wave that does run passes `isolation: 'worktree'` to every mutating agent (`docs/governance/loop-instruction-template.md §3`).

The orchestrating session never implements directly — it dispatches, verifies, and rules (`loop-instruction-template.md §2.2`). This held across SD-27's launch and the CRB run before it; nothing about SD-30's shape is an exception.

**Reasoning:** `loop-instruction.md`'s OPERATING METHOD callout (authored before this correction propagated) still named `/loop 60m /batch /goal ...` as the dispatch command. That form requires a human to re-type a slash command per invocation and cannot run headless — directly contradicting §21's unattended-mode authorization, which requires the bundle to run to closure across days with nobody watching. A `Workflow`-tool session, not a slash-command invocation, is what can actually satisfy that requirement.

**Consequence:** `loop-instruction.md`'s OPERATING METHOD callout now names the `Workflow` tool; §21 point 5 ("closure is a goal, not a stop signal") is corrected to read "under the dispatcher's own loop" rather than "per `/loop` cadence."

**Cross-reference:** `docs/release/SD-27-future-state-book-content-ingestion/decisions.md §19` (the adopted correction, itself from SD-26 `decisions.md §13`); `docs/governance/loop-instruction-template.md §2` (orchestration mode), `§2.1` (`RETRO_ACTOR`), `§2.2` (execution boundary), `§3` (worktree-isolation requirement for parallel waves).

## Decision 23 — A running retrospective log is part of the cycle procedure, not an optional courtesy (2026-08-01)

**Status:** Process alignment — the tooling already exists on `tranche/10` (`scripts/retro.py`, `docs/retro/schema.json`, `docs/retro/events/<actor>.jsonl`); this package had not yet wired emission into its own cycle procedure.

**Decision:** Every SD-30 cycle emits at least one retrospective event via `scripts/retro.py`. The event vocabulary (`correction`, `incident`, `near_miss`, `deferral`, `rework`, `verification`, `note`) and the field contract live in `docs/retro/schema.json` and are not re-specified here — read `python3 scripts/retro.py help <type>` for the real flags before emitting.

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

**Decision:** Epic 21 (SD30-E21, Bundle Code Review) is added as SD-30's last-numbered epic (per `kanban.md`'s numbering, which already runs `epic-1` through `epic-20` for every epic including the previously-unnumbered "Closure Epilogue" (`epic-19-closure`) and "Build Version Numbering" (`epic-20-version`) headers in `epic-breakdown.md`). Its dispatch slot is after every content-ingest epic (3 through the Inner Sea/Book of the Damned set) and Build Version Numbering, and before Closure Epilogue — Closure Epilogue remains the true final step per `loop-instruction.md §"Epic ordering"` (unchanged by this decision), so any finding the review surfaces is fixed before the tranche-promotion PR (part of Closure Epilogue) opens.

`./scripts/verify.sh` passing is a **precondition** for Epic 21 to fire, never the review itself: a green gate says the tests that exist pass, it says nothing about whether the code is right.

**Scope, at minimum:**

- Correctness of rules logic against the corpus (sampled, not exhaustively re-derived) across the sixteen in-scope books.
- No stubs or fixture-only data in production paths, per `docs/governance/no-stub-mvp-doctrine.md`.
- Content genuinely reaching a player surface, per `reach_gate.rs`'s `OPEN_FINDINGS` mechanism (spot-checked against the live IPC/UI path, not just the gate's exit code) — including the Mythic Adventures reach-surface prerequisite called out in `loop-instruction.md`'s "Recommended sequencing".
- Test quality, not just count — per `docs/governance/book-ingestion-playbook.md §7.4`'s mutation-test pattern, a sample of new gates/tests is checked for a case that actually fails when the thing it protects is broken.
- No hand-authored rules data in the frontend (`apps/desktop/src/`).

**Mechanism — wired into what already exists, nothing invented fresh:** the review runs `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` (this bundle's standing per-cycle dual-audit) against the **whole-bundle diff**, not a single cycle's slice: `git diff origin/develop...HEAD` — the same merge-base triple-dot comparison both scripts already default to via `BASE_BRANCH=origin/develop`. No new grep/audit tooling is invented; Epic 21 reuses the standing per-cycle gates at bundle scope and adds the manual/agent-driven judgment a grep cannot do (corpus-correctness sampling, reach-claim spot-check, test-quality sampling).

**Findings are triaged, not auto-fixed.** Each finding records a severity and a disposition: `fixed-in-bundle` or `deferred`. A `deferred` finding names an owner (a person or a specific successor bundle) and lands in `forward-scope-register.md` — an unowned deferral is not a valid disposition. Real defects are fixed in-bundle before Closure Epilogue fires.

**Operator escalation path, not a substitute:** the operator can separately trigger `/code-review ultra`, a multi-agent cloud review of the branch. That path is operator-triggered and billed — a cycle running under §21's unattended-mode protocol cannot launch it itself — so Epic 21 must stand on its own as the bundle's actual gate.

**Cross-reference:** `epic-breakdown.md` Epic 21; `acceptance-and-verification.md AT-30-013`; `docs/governance/no-stub-mvp-doctrine.md`; `docs/governance/book-ingestion-playbook.md §7.4`; `reach_gate.rs`; `kanban.md` card `epic-21-code-review`.