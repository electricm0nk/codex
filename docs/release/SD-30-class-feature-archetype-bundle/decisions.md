# SD-30 Decisions

**Status:** Planning-ready (operator directives 2026-08-01; canonical chassis landing this cycle). Refined 2026-08-01 (book list confirmed; tranche/10 + no-Hermes-board + 0.10.<build> + reach-gate DoD doctrine applied per the 2026-08-01 amendments shared with SD-28/SD-29).

## Decision 1 — Book list CONFIRMED 2026-08-01

**Status:** Operator-pinned, **confirmed 2026-08-01.**

**Decision:** SD-30 ships content-source ingest for the following sixteen books, with NPC Codex + Planar Adventures + Occult Origins + Haunted Heroes deferred to `forward-scope-register.md C2.x`:

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

**Deferred (NOT in scope):** NPC Codex and Planar Adventures per the 2026-08-01 absent-book rule — genuinely absent from the whole PCGen corpus (verified across all publishers 2026-08-01). Occult Origins and Haunted Heroes Handbook by **explicit operator choice 2026-08-01**: both ARE present in the corpus under `player_companion/` (the 07-30 "absent" finding searched the wrong subtree and, for HHH, the wrong identifier — see `scope-draft.md` §"Shape finding … RESOLVED" and `forward-scope-register.md C2.3/C2.4`), so the absent-book rule does not apply to them; the operator keeps the sixteen-book pin regardless. All four recorded in `forward-scope-register.md C2.x`.

**Per-book path locations under `src/rules_core/rules_tables/<book>/`** are in the §"Book list" table in `scope-draft.md`.

## Decision 2 — Branch and board [SUPERSEDED — see §13 and §14a]

**Status:** Doctrine-of-record (per SD-22 doctrine); **superseded 2026-08-01** by Decision §13 (branch) and §14a (board retirement), which tighten the rule.

**Original text:** SD-30 launches on `tranche/6-2` branch + `codex-tranche-6-2` board.

**Why superseded.** SD-28 broke the `tranche/6` family on 2026-08-01 (`tranche/8`). SD-29 followed at `tranche/9`. SD-30 takes `tranche/10`. The `codex-tranche-<N>` slug is reserved-as-form, not as-instance (the Hermes board is retired).

## Decision 3 — Build version target [SUPERSEDED — see §15]

**Status:** Doctrine-of-record (per SD-22 doctrine); **superseded 2026-08-01** by Decision §15, which applies the 2026-07-17 build-version amendment.

**Original text:** `0.6.<build>` first concrete value. tranche-base = 6 per `<major>.<tranche-base>.<build>` scheme.

**Why superseded.** SD-30's tranche-base is 10, not 6. First concrete value is `0.10.<build>` per Decision §15.

## Decision 2 (original text — SUPERSEDED, see §13 and §14a) — Branch and board

> **Retained as the audit record of what was originally proposed. The values below are superseded and must not be acted on** — SD-30's branch is `tranche/10` and its build target `0.10.<build>`, per §13/§15. Heading disambiguated 2026-08-01; text unchanged.

**Status:** Pending operator confirmation.

**Candidate:** `tranche/6-2` branch + `codex-tranche-6-2` board.

**Rationale:** SD-28 proposes `tranche/6`, SD-29 proposes `tranche/6-1`. SD-30 follows the dash-1 sub-release pattern at `tranche/6-2`. Operator-pinned pending.

**Alternative:** SD-30 could split per-book across sub-tranches (e.g., `tranche/6-2-oa`, `tranche/6-2-oo`, `tranche/6-2-ha`). Operator preference.

## Decision 3 (original text — SUPERSEDED, see §15) — Build version target

> **Retained as the audit record of what was originally proposed. The values below are superseded and must not be acted on** — SD-30's branch is `tranche/10` and its build target `0.10.<build>`, per §14/§15. Heading disambiguated 2026-08-01; text unchanged.

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

**Status:** Operator-pinned, **confirmed 2026-08-01.** Cross-cutting — affects Decision §8 (operating form), §13 above, and the loop-instruction pre-launch checklist.

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
- `./epic-breakdown.md` — 21 epics (matching `kanban.md`'s 21 cards) × ~3-4 criteria each.
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
- Content genuinely reaching a player surface, per `reach_gate.rs`'s `OPEN_FINDINGS` mechanism (spot-checked against the live IPC/UI path, not just the gate's exit code) — including the Mythic Adventures reach-surface prerequisite called out in `epic-breakdown.md`'s "Recommended sequencing" (and `forward-scope-register.md C3.1`). Mechanically, this means driving the running desktop app via `apps/desktop/.claude/skills/run-desktop/driver.sh` and reading the value off a screenshot, per `loop-instruction.md`'s Definition of done item 8, with `RUN_DESKTOP_AGENT` set to a value unique to this review (`apps/desktop/.claude/skills/run-desktop/SKILL.md` §"Concurrent agents").
- Test quality, not just count — per `docs/governance/book-ingestion-playbook.md §7.4`'s mutation-test pattern, a sample of new gates/tests is checked for a case that actually fails when the thing it protects is broken.
- No hand-authored rules data in the frontend (`apps/desktop/src/`).

**Mechanism — wired into what already exists, nothing invented fresh:** the review runs `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` (this bundle's standing per-cycle dual-audit) against the **whole-bundle diff**, not a single cycle's slice: `git diff origin/develop...HEAD` — the same merge-base triple-dot comparison both scripts already default to via `BASE_BRANCH=origin/develop`. No new grep/audit tooling is invented; Epic 21 reuses the standing per-cycle gates at bundle scope and adds the manual/agent-driven judgment a grep cannot do (corpus-correctness sampling, reach-claim spot-check, test-quality sampling).

**Findings are triaged, not auto-fixed.** Each finding records a severity and a disposition: `fixed-in-bundle` or `deferred`. A `deferred` finding names an owner (a person or a specific successor bundle) and lands in `forward-scope-register.md` — an unowned deferral is not a valid disposition. Real defects are fixed in-bundle before Closure Epilogue fires.

**Operator escalation path, not a substitute:** the operator can separately trigger `/code-review ultra`, a multi-agent cloud review of the branch. That path is operator-triggered and billed — a cycle running under §21's unattended-mode protocol cannot launch it itself — so Epic 21 must stand on its own as the bundle's actual gate.

**Cross-reference:** `epic-breakdown.md` Epic 21; `acceptance-and-verification.md AT-30-013`; `docs/governance/no-stub-mvp-doctrine.md`; `docs/governance/book-ingestion-playbook.md §7.4`; `reach_gate.rs`; `kanban.md` card `epic-21-code-review`.

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

## Decision 28 — The four architectural traps SD-30 inherits from SD-27 (2026-08-01)

**Status:** Carried forward from tranche/7. Cross-cutting — each trap fires **per record**, not per book.

**Decision:** SD-30 inherits four architectural traps recorded in SD-27 `decisions.md §29`. They are **cited, not restated** — §29 is the authority and must not be allowed to drift. Each is named here because SD-27 hit every one *after* the work looked done, so the cost was rework rather than discovery.

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


**Authority:** SD-27 `decisions.md §29` (traps), `§30` (paths and artifacts), `docs/retro/tranche-7-retrospective.md` (measurements).

## Decision 29 — `.MOD` schema: description augmentation vs. class-list supplement (resolved 2026-08-01, before dispatch)

**Status:** Operator-directed, resolved pre-dispatch. Closes the open schema question raised in `forward-scope-register.md §C4.4`. **Also binds SD-28** — Ultimate Magic carries 538 rows of the same shape.

**The question was:** Shape B has no precedent for a record that exists only as a delta on another book's record. Mythic Adventures is predominantly such a layer. What schema represents it?

**It dissolves under measurement.** There is no single "`.MOD` record" shape. There are two, and each already has a home.

### 29.1 The measurement

`.MOD` spell rows across every book in the PCGen tree, classified by whether their tokens are prose-only or carry mechanics:

| book | `.MOD` rows | DESC-only | mechanics-bearing |
|---|---:|---:|---:|
| occult_adventures | 1526 | **0** | **1526** |
| advanced_players_guide | 1371 | 259 | 1112 |
| core_rulebook | 675 | 612 | 63 |
| advanced_class_guide | 598 | 134 | 464 |
| ultimate_magic | 538 | **0** | **538** |
| **mythic_adventures** | **269** | **269** | **0** |
| ultimate_combat | 159 | 144 | 15 |
| ultimate_intrigue | 101 | 101 | 0 |
| horror_adventures | 76 | 72 | 4 |
| ultimate_wilderness | 50 | 50 | 0 |
| advanced_race_guide | 6 | 0 | 6 |
| adventurers_guide / monster_codex | 1 / 1 | 1 / 0 | 0 / 1 |
| **TOTAL** | **5371** | **1642** | **3729** |

`.MOD` is a pervasive PCGen idiom, not a Mythic quirk. **Mythic is the cleanest case in the tree, not the hardest** — 100% DESC-only, zero mechanics.

### 29.2 Ruling A — DESC-only `.MOD` is a description augmentation, not a record

**All 269 of Mythic's `.MOD` rows carry exactly one token kind: `DESC`.** They append *"Mythic: …"* prose to a spell defined in another book. They change no school, level, class list, range, or duration.

**Ruling: augment the existing record; do not mint a new one.** Add an additive, `#[serde(default)]` field to the spell payload (`json_cache::SpellCacheData`, currently `key`/`school`/`level`/`description`) carrying variant descriptions with their own book and page — e.g. `variant_descriptions: Vec<VariantDescription { book, source_page, text }>`. Records written before the addition deserialize unchanged.

**Why not phantom records.** A `.MOD` row has no school, no level, no `CLASSES:`. Minting 269 records would put 269 rows into the spell catalog that no character can ever cast — dead affordances, forbidden by `docs/governance/no-stub-mvp-doctrine.md` — and would double-count in every coverage ratio (`forward-scope-register.md §C4.5`).

**Why not merge the prose into the base description.** That attributes Mythic Adventures' text to the Core Rulebook and destroys provenance, against SD-27 `decisions.md §25` (attribute to the true source book) and `§29.4` (provenance checked per row). **The variant must carry its own book and page.**

**This ruling covers 1642 rows across 10 books**, not just Mythic's 269.

### 29.3 Ruling B — mechanics-bearing `.MOD` on spells is a class-list supplement, and the pattern already exists

The 3729 mechanics-bearing rows are not arbitrary mutations. **All 1526 of Occult Adventures' carry `CLASSES:`** — they add psychic classes to existing spells' class lists. Ultimate Magic's 538 are `ITEM` (231), `CLASSES` (163) and `DESCRIPTOR` (139).

**A `CLASSES:`-bearing `.MOD` says "this existing spell is also a Psychic 3 spell." That is a per-class spell level, and SD-27 already built the pattern** (commit `f4dcb522`): a per-book supplement table chained into `rules_core::rules_tables::class_spell_levels`, exactly as `advanced_race_guide::class_spell_levels` (389 rows, 13 classes) supplements CRB/APG/ACG. Occult Adventures gets `occult_adventures::class_spell_levels`; the chain resolves it.

**Consequence for SD-30's sizing:** Occult Adventures' 1526 `.MOD` rows are **not** 1526 new spell records. They are class-level supplement entries against spells other books already define — plus its genuinely new psychic spells, which are a separate count. `§C4.4`'s "472 spell keys not in any ingested book" is the new-declaration figure and stands; the 1526 is additional and cheaper per row.

### 29.4 Mythic's genuine content, and one corpus defect

**9 rows in `ma_spells.lst` are real declarations** carrying full mechanics (`CASTTIME`, `CLASSES`, `COMPS`, `SCHOOL`, `RANGE`, `SAVEINFO`, `SPELLRES`, `TARGETAREA`): Ascension, Bleed Glory, Deathless, Lend Path, Mythic Severance, Restore Mythic Power, Share Glory, Steal Power, Terraform. These are ordinary spell records.

**`ma_spells.lst:98` is a corpus typo.** The row is named `Elemental Body IIIMOD` — a missing `.` in `Elemental Body III.MOD`. It carries only `DESC`, and `Elemental Body III` is genuinely defined in `core_rulebook`, `advanced_players_guide` and `core_essentials`. Parsed literally it declares a **phantom spell with no school, level or class list**.

**Handling, per SD-27 `decisions.md §25.4`'s precedent for the upstream `Wall of Thorms` misspelling:** treat it as the 270th description augmentation against `Elemental Body III`, preserve the upstream key verbatim in `raw_tokens`, and **do not silently rename the source**. Record it as an upstream defect. Ingesting it literally would ship an uncastable phantom into the spell catalog — the exact failure Ruling A exists to prevent.

### 29.5 Correction to `forward-scope-register.md §C4.4`

That section called Mythic Adventures a `.MOD` graft layer with *"no precedent in Shape B"* and flagged it as the schema hazard. **The measurement inverts that.** Mythic is the tree's cleanest `.MOD` case; **Occult Adventures — in this same bundle — is the hard one**, with 1526 mechanics-bearing rows against Mythic's zero. §C4.4's Occult warning was right for the wrong reason: the hazard is its `.MOD` class-list volume, not only its 472 new declarations.

**Authority:** operator directive 2026-08-01 ("resolve the mythic `.MOD` schema question before SD-30 starts"); measurements derived by command over the PCGen tree; precedent SD-27 `decisions.md §25.4`, `§29.4`, and commit `f4dcb522`.

## Decision 30 — The "only writer" premise was false, and it lived in SD-27, not here (2026-08-01)

**Status:** New, correcting a premise this bundle would otherwise inherit silently rather than restating something already true here.

**Where the premise actually lives.** A search across all three `docs/release/SD-2[89]-*`/`SD-30-*` packages for `only writer` / `sole writer` returns **zero hits**. The premise, and its correction, both live in **SD-27**'s own `decisions.md §28` (`docs/release/SD-27-future-state-book-content-ingestion/decisions.md:499`), dated 2026-07-31: *"There is no concurrent cycle to collide with; this branch is the only writer."* That line justified lifting SD-27's own §8 file-touch partition on the premise that v0.6 had closed and nothing else was writing the tree. `docs/retro/events/size-modifier-agent.jsonl` records the same-day correction verbatim: *"decisions.md 28 declared 8's file-touch partition spent on the stated premise that 'this branch is the only writer' -- that premise is false in practice."* Ten of the tranche's 34 logged incidents (29%, retrospective §4.1) trace to exactly this false premise, four of them `git stash` swallowing a sibling's uncommitted work.

**Relationship to Decision 28's "one writer per tree" bullet, above.** That bullet already carries the process rule (own `CARGO_TARGET_DIR`, never shared between a worktree and the working tree). It is **cited here, not restated** — this decision adds what Decision 28 did not: naming exactly where the false premise text lives, and the staging/stash/preflight mechanics the premise's absence requires.

**Why it matters here even though the text is SD-27's, not this bundle's.** SD-28, SD-29 and SD-30 are three concurrently-launched bundles sharing one checkout and branch, each dispatched from a session that can itself be running alongside sibling sessions on the same box. The SD-27 mistake — asserting sole-writer status because no *specific, currently-known* concurrent bundle is active — reproduces immediately if this bundle assumes the same about SD-28, SD-29, or a human operator's own parallel session on the identical checkout.

**Ruling: the file-touch partition is necessary, not sufficient, and this bundle's own version of it must not rest on a sole-writer claim.**

- **Other writers exist, or may exist, concurrently — always.** No cycle in this bundle may assert sole-writer status as grounds for skipping a concurrency check; a partition is a courtesy between cooperating writers, not a lock.
- **`git status --porcelain` runs before every git write**, in every cycle, regardless of whether the cycle believes itself to be the only writer. A file listed that this cycle did not modify is a stop condition, reported per "Hard stops," never silently overwritten or attributed to this cycle's own change.
- **Staging is always explicit-path:** `git add <file> <file> ...`. Never `git add -A` or `git add .` — a wildcard add cannot distinguish this cycle's own changes from a sibling's uncommitted work sitting in the same tree.
- **`git stash` is never run, under any circumstance, in this repo.** The bare form stashes the *entire* working tree, not a subdirectory or a cycle's own changes, and has already destroyed a sibling's uncommitted work multiple times in this program (four of the ten shared-tree incidents above). To capture a HEAD baseline for comparison, use `git show HEAD:<path>` into a scratch file, or a separate `git worktree add` — never stash.
- **Any parallel *mutating* wave dispatches each agent with `isolation: 'worktree'`** — already required for cross-bundle/cross-epic concurrency by the OPERATING METHOD callout in `loop-instruction.md`; this decision confirms the same rule covers this bundle's own multi-book fan-outs, not only collision with SD-28/SD-29.

**Authority:** `docs/retro/tranche-7-retrospective.md` §4.1 and §6.1 (rules A1/A2); SD-27 `decisions.md §28` (where the false premise and its correction actually live — cited, not restated); `docs/retro/events/size-modifier-agent.jsonl` (the correcting incident, verbatim).


## Decision 31 — Automated disk reclamation is part of the cycle, not a manual afterthought (2026-08-01)

`docs/retro/tranche-7-retrospective.md` §4.1 records disk exhaustion as this program's **second-largest recorded orchestration failure mode — 5 of 34 logged incidents** (`/tmp` tmpfs at 91% → `ld terminated with signal 7 [Bus error]`, 20 minutes lost; `/` at 91%, 98%, 98%; `/home` at **100% used, 0 bytes available**, with "30+ per-agent `CARGO_TARGET_DIR`s under `~/.cache` totalling >600G, many 18-35G each," 25 minutes lost). The retrospective's own diagnosis is the design constraint this decision closes: *"The rule shipped in the brief; the matching `rm -rf` did not."* `AGENTS.md` and this bundle's own concurrency rules (Decision 30, above) correctly mandate a per-agent, per-source-tree `CARGO_TARGET_DIR` and tell agents to delete it when they finish — but nothing ever enforced or automated that deletion, so it did not happen at the rate the rule needed.

Two additions, landed in `scripts/` (shared across SD-28/SD-29/SD-30, not per-bundle code):

- **`scripts/reclaim.sh`** — dry-run by default; `--apply` required to delete anything. Four categories: abandoned `CARGO_TARGET_DIR`s (found under the Claude scratchpad root and this repo's `$HOME/.cache/codex-*` convention, confirmed by directory *shape* — `.rustc_info.json`/`debug`/`release` — not merely the presence of `CACHEDIR.TAG`, which fontconfig/uv/man-db also write and which a naive check flagged as a false positive on this script's own first dry run); stale `scripts/verify.sh` log directories; git worktrees whose branch is merged into `develop` or whose PR is closed/merged (`git worktree list --porcelain` + `gh pr list`); and local branches merged or gone from origin. Safety: never touches a target dir a live `cargo`/`rustc` process is using (checked via kernel-reported `comm` and `/proc/<pid>/environ`/`cwd`, not a self-matching `pgrep -f` — the self-match trap named explicitly in the brief that produced this script); never removes a worktree with uncommitted changes or unpushed commits; never touches this repo's own checkout or the `pcgen` oracle clone; never runs `git stash`. Emits a `retro.py incident` event (`recurrence-key disk-full`) whenever `--apply` actually reclaims something.
- **`scripts/verify.sh`'s new `preflight-disk` stage** — first in *both* the `--quick` and full stage sets, so it fails loudly and points at `reclaim.sh` **before** the ~490-binary `root-full` build starts, rather than only recording pressure after the fact the way the script's existing `emit_disk_pressure_event` (post-run, informational) already did.

**This bundle's `loop-instruction.md` Cycle mechanics now runs the preflight check at the start of each cycle and `scripts/reclaim.sh --apply` at cycle end.** The mandate is paired with the command, which is the entire lesson of §4.1 restated as a rule: a rule with no executable counterpart is the rule that produced 600G.

**Authority:** `docs/retro/tranche-7-retrospective.md` §4.1 (disk exhaustion, 5 of 34 incidents) and §6.1 rule A4 (`CARGO_TARGET_DIR` deletion + pre-sweep disk check); `AGENTS.md` "Concurrency and Measurement."

## Decision 32 — Starting state is zero-proven across all sixteen books; `occult_adventures` is spell-heavy and hits the harness ceiling; the two-absent-book finding independently re-verified (2026-08-02)

**Status:** New. Reconciliation pass against `/home/ubuntu/swarm-observer/PF1e-dashboard.json` `work_inventory` (`generated_at: 2026-08-02T12:40:01Z`), done for the same reason as SD-29's `decisions.md §35` (cross-reference): the operator directive that previously-started and Ultimate books reach 100% proven exposed that this package's launch-readiness must be stated in measured terms.

**Measured starting state.** Command:

```
python3 -c "
import json
d = json.load(open('/home/ubuntu/swarm-observer/PF1e-dashboard.json'))
books = {b['id']: b for b in d['work_inventory']['books']}
for k in ['occult_adventures','horror_adventures','mythic_adventures','monster_codex',
          'book_of_the_damned_volume_1','book_of_the_damned_volume_2','inner_sea_world_guide',
          'inner_sea_combat','inner_sea_faiths','inner_sea_gods','inner_sea_magic',
          'inner_sea_races','inner_sea_temples','inner_sea_taverns','inner_sea_bestiary',
          'inner_sea_intrigue']:
    b = books[k]; print(k, b['units'], b['proven'], b.get('scope'))
"
```

Result: **all sixteen in-scope books read `0 proven` of a combined 12,246 units** (each carries `scope: future_state` in the dashboard and has not been touched by ingestion yet). This package's own chassis does not claim otherwise anywhere in `decisions.md`, `scope-draft.md`, or `README.md` — no correction to an existing false claim was needed here, unlike the "closed"-framing issue found in SD-29. This decision exists to make the zero-proven starting point and its consequence explicit rather than left implicit.

**`proven` excludes `ingested-magnitude`, and `occult_adventures` is spell-heavy — verified against the corpus, not assumed.** Command:

```
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/occult_adventures -iname "*.lst" | xargs -I{} basename {}
wc -l ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/occult_adventures/oa_spells*.lst | tail -1
```

Result: Occult Adventures carries **six separate spell-list `.lst` files** (`oa_spells.lst` plus `oa_spells_uc/um/acg/arg/ma.lst`, covering spells shared with Ultimate Combat/Magic/ACG/ARG/Mythic), totaling **2,170 lines in `oa_spells.lst` alone**. Per `docs/release/SD-28-ultimate-book-content-ingestion/decisions.md` (E13-E30 completion-epics decision, added 2026-08-02, commit `3eb11a18`): `status_vocabulary` defines `ingested-magnitude` as *"The engine holds the record WITH its real numeric fields, but this generator observes no consumer delta for this kind (spells, equipment)"* — `Kind::Spell` and `Kind::Equipment` have no wiring probe in `v06_work_inventory.rs`'s `classify()`, so no amount of correct spell ingestion in Occult Adventures can move it past `ingested-magnitude` into `proven` until **SD-28 Epic 14** (observation-harness widening) lands. This is the same measurement ceiling SD-29 inherits (`SD-29 decisions.md §35`), and it applies most acutely here: Occult Adventures is this package's largest single book (1,831 units) and its psychic-magic content is spell-dominant by design.

**Two-absent-book finding — independently re-verified, not merely re-cited.** This package's `scope-draft.md` §"Deferred" and `decisions.md §1` already correctly record NPC Codex and Planar Adventures as genuinely absent from the corpus (2026-08-01 absent-book rule) and Occult Origins/Haunted Heroes Handbook as present-but-deferred by operator choice. Independent re-verification:

```
find ~/workspace/repos/pcgen/data -iname "*npc_codex*" -o -iname "*npccodex*"
find ~/workspace/repos/pcgen/data -iname "*planar*"
```

`npc_codex` returns no hits anywhere in the corpus. `*planar*` returns only an unrelated 3.5e product (`35e/lions_den_press/secrets_of_the_planes/planar_magic/`), not Pathfinder's *Planar Adventures* — confirming it is genuinely absent, not misfiled. **Both absences confirmed; this package's existing framing is accurate and stands unchanged.**

**Launch-readiness assessment.** This package's own documents contain no false "predecessor books are complete" premise to correct — sixteen books at zero-proven does not create a false completeness claim the way SD-29's Bestiary-1 dependency did, because SD-30 does not claim any of its sixteen books, or an external predecessor, is a finished foundation. What this package was missing was the explicit statement that (a) it starts from zero measured progress across the whole scope, and (b) its largest book cannot reach 100% proven post-ingestion without SD-28 Epic 14. **Not launch-ready** in the sense of "ready to reach 100% proven on its own" — it is planning-ready (per its existing sixteen-book pin) but gated on the same cross-bundle harness prerequisite as SD-29 and SD-28's own remaining spell/equipment-heavy books.

**Authority:** `/home/ubuntu/swarm-observer/PF1e-dashboard.json` `work_inventory` section, `generated_at: 2026-08-02T12:40:01Z`; `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/occult_adventures/` directory listing (2026-08-02); `docs/release/SD-28-ultimate-book-content-ingestion/decisions.md` (E13-E30, Epic 14 harness decision, commit `3eb11a18`); `SD-29-corpus-wide-catch-up-lanes/decisions.md §35` (parallel reconciliation; directory renamed
2026-08-10 from `SD-29-bestiary-line-book-ingestion` when SD-29 was re-scoped corpus-wide,
`decisions.md §38` in that package).

---

# Decisions from the 2026-08-10 re-scope onward

**Everything above this line (Decisions 1-32) is the record of the sixteen-book content-bundle era
and is retained as history, not deleted.** Where a decision above conflicts with a decision below,
the decision below governs — the re-scope is later in time and higher in authority (a direct operator
directive). Branch (`tranche/10`), build-version scheme, Hermes-board retirement, reach-gate-as-DoD,
the `Workflow`-tool operating form, and the cross-bundle class-grant rule for Occultist/Spiritualist/
Medium/Mesmerist (§5) are **not** touched by the re-scope and continue to apply exactly as decided
above.

## Decision 33 — SD-30 becomes the `class_feature` bundle; every figure re-derived, one correction made (2026-08-10)

**Status:** New. Operator directive 2026-08-10, following SD-29's corpus-wide re-scope
(`SD-29-corpus-wide-catch-up-lanes/decisions.md §38`, commit `472acb4f`), which claims every kind
corpus-wide except `class_feature` and flags-but-does-not-resolve the resulting collision with
SD-30's sixteen-book list (`§38.5`, `risks-and-open-questions.md` R-29-009/OQ-29-004). The operator
has now ruled directly: **SD-30 becomes the `class_feature` bundle.** Its book list dissolves; its
scope becomes the one lane SD-29 cannot take.

**Every figure re-derived independently, not transcribed from the brief that requested this
decision** — the brief's author flagged that its last three briefs on this work each carried at
least one wrong figure.

```bash
cd ~/workspace/repos/codex
python3 - <<'PY'
import json, collections
d = json.load(open('docs/work-inventory.json'))
U = d['units']
cf = [u for u in U if u['kind']=='class_feature']
print('total class_feature units', len(cf))
print('books', len(set(u['book'] for u in cf)))
c = collections.Counter(u.get('status') for u in cf)
for k,v in sorted(c.items(), key=lambda x:-x[1]): print(k, v)
print('sum', sum(c.values()))
PY
```

Result: **15,472 `class_feature` units, 23 books.** By status: `not-ingested` 9,078, `not-started`
3,293, `unknown` 2,958, `grounded` 109, `deferred-with-reason` 34 (sums exactly to 15,472). This
matches the brief's figures **exactly** — 15,472 / 23 / 40.2% (`15,472 / 38,536` from
`SD-29-corpus-wide-catch-up-lanes/decisions.md §38.1`'s corpus total) / 9,078 / 3,293 / 2,958 / 109 /
34, with no correction needed this time. Grounded is **109 of 15,472 = 0.7%**, confirmed.

**The 23 books, by unit count** (re-derived, not carried over from the sixteen-book list — the old
list and the real `class_feature` book population diverge substantially):

```
2,396 advanced_class_guide       866 ultimate_wilderness        68 monster_codex
2,055 advanced_players_guide     777 ultimate_intrigue          18 bestiary_6
1,422 ultimate_psionics          700 adventurers_guide          11 inner_sea_taverns
1,412 ultimate_combat            645 advanced_race_guide        10 book_of_the_damned_volume_1
1,070 ultimate_magic             577 pathfinder_unchained         4 bestiary_4
  979 occult_adventures          419 horror_adventures
  959 core_rulebook              314 inner_sea_combat
                                  218 inner_sea_magic
                                  212 book_of_the_damned_volume_2
                                  171 inner_sea_world_guide
                                  169 inner_sea_intrigue
```

**Consequence for the old sixteen-book list:** it named the *books*, not the *kind*, and most of its
sixteen books' `class_feature` populations are small relative to books it never named
(`advanced_class_guide` alone carries more `class_feature` units, 2,396, than the old scope's ten
Inner Sea modules combined, 2,155). The book list was never a good proxy for `class_feature` scope
and dissolving it (Decision §35) is a correction, not just a re-alignment with SD-29.

**Authority:** operator directive 2026-08-10 (verbatim in the brief driving this decision);
`SD-29-corpus-wide-catch-up-lanes/decisions.md §38, §38.4, §38.5`; `docs/work-inventory.json`.

## Decision 34 — Inheritance from SD-28 `§60`/`§63`/`§64`: verified, not assumed (2026-08-10)

**Status:** New. `class_feature`'s status as a bundle rather than a lane rests entirely on SD-28's
`§63` finding, and SD-30 inherits real, already-landed groundwork from `§60` and `§64`. Both are
verified here by reading the decisions directly, not by trusting the brief's or the commit
messages' summary of them.

**`§60` (2026-08-09): the primitive.** `archetype_resolver::archetype_claims_slot` landed end-to-end
on Alchemist's Poison Resistance — a real "is this slot claimed by the character's archetype, and if
so does the archetype's own substitute compute" supersession check, proven reachable per `§43`'s
standard (a headless pilot receipt test, not a unit test on the resolver alone).

**`§63` (2026-08-10): the sample-size proof.** A second, structurally different class (Fighter,
non-resource-pool, unlike Alchemist) confirmed the primitive generalizes with zero changes to
`archetype_resolver.rs` itself, but the *wireable fraction* did not generalize: Fighter 1/22 (5%),
Alchemist 3/26 (12%), Paladin 16/33 (48%), Bard 23/33 (70%) — a 14x spread with no shared ratio and
no predictor found (not class type, not slot count, not total real-id count). This is the finding
that makes `class_feature` unschedulable by extrapolation and forces the bundle shape.

**`§64` (2026-08-10, operator-funded in direct response to `§63`): 25 of 28 archetype-bearing
classes hand-verified by direct evidence, no automated proxy** (three id-scan proxy iterations had
already failed on three different naming assumptions per `§63` itself):

```
TOTAL: 263 wired-able slots / 475 named slots -> 175 real mechanisms (collapsing duplicate
slot-tier names that supersede one shared computation, e.g. ChannelEnergy1..10 -> one
channel_energy_dice mechanism) x ~33 lines/mechanism = ~5,775 lines of production wiring.
```

Per-class spread confirmed by direct re-read of the decision (0% Companion to 100% Skald,
non-uniform, no blended ratio reported anywhere in `§64` by design). **Two named wiring shapes:**

1. **Supersession** (`archetype_claims_slot`) — 25 classes, 175 mechanisms, ~5,775 lines, a real
   floor. Proven end-to-end on Alchemist and Fighter.
2. **Chooser-interaction** — 3 classes excluded from the 175-mechanism total (Oracle, Arcanist,
   Sorcerer), because their archetype slots name "the thing picked at level N" (a mystery
   revelation, an exploit, a bloodline power), not one static computation. Real partial grounding
   exists for each (Oracle: 5 revelations across 5 mysteries; Arcanist: Metamagic Knowledge exploit;
   Sorcerer: 2+ bloodlines) but no slot-to-mechanism table can represent them honestly. **This shape
   has no primitive yet** — `archetype_claims_slot` answers a supersession question; these three
   classes need a "which options remain choosable, and does the substitute grant compute" chooser
   question, unproven by anything landed in SD-28.

`§64` also found **eleven unmodelled base-class features** incidentally (Druid wild shape, Barbarian
rage powers, Cavalier banner/charge/mount, Hunter 15/21 slots, Witch patron spells, Shaman
hex/spirit-magic, Cleric+Druid spontaneous casting, Wizard arcane bond/cantrips, Companion's entire
advancement subsystem) — engine gaps in core classes' signature mechanics, not archetype problems,
recorded here so SD-30 does not rediscover them as if new.

**Inherited whole into SD-30:** the primitive, both measurements, the 25-class/175-mechanism/two-
shape sizing, and the eleven-feature gap list. SD-30's Epic 4 (per-class measurement) extends
`§63`/`§64`'s method to the classes SD-28 did not reach (all `class_feature`-bearing classes outside
the 28 archetype-bearing ones `§64` enumerated — e.g. Occultist, Spiritualist, Medium, Mesmerist from
Occult Adventures, and any Inner Sea/Mythic class content) and to designing the chooser-interaction
primitive for the 3 excluded classes. SD-30's Epic 5 (archetype mechanism) builds out the measured
175-mechanism supersession shape.

**Authority:** `SD-28-ultimate-book-content-ingestion/decisions.md §59, §60, §63, §64` (read in full,
not summarized from commit `9b871bd0`'s message alone, though that commit — "25 of 28 archetype
classes hand-verified, 175 mechanisms (~5,775 lines), two wiring shapes named" — matches the decision
text exactly).

## Decision 35 — Collision with SD-29 closed: the sixteen-book list dissolves (2026-08-10)

**Status:** New, resolves `SD-29-corpus-wide-catch-up-lanes/decisions.md §38.5` and that package's
R-29-009/OQ-29-004 (recorded on both sides — see the mirrored resolution entries added to those
files in this same change).

**The collision, restated.** SD-29's re-scope made every one of SD-30's sixteen books' non-
`class_feature` kinds (spell, equipment, monster, monster_ability, race_trait, companion,
equipment_modifier, feat) part of SD-29's corpus-wide lanes — the same (kind, book) cells SD-30's old
per-book epics would have dispatched against. Two writers could have landed on the same table file.

**Resolution: SD-30's book list is retired outright, not narrowed.** SD-30 does not keep a reduced
book list scoped to "whatever `class_feature` content those sixteen books carry" — that would still
be a book-shaped scope competing conceptually with SD-29's kind-shaped scope, and Decision §33 above
already shows the sixteen-book list was a poor proxy for `class_feature`'s real 23-book population
(`advanced_class_guide` alone outweighs the old scope's ten Inner Sea modules combined). SD-30's
scope is now **the kind, corpus-wide, not the book list, not even narrowed.**

**What this means concretely:**

- Every one of the old sixteen books' non-`class_feature` kinds is SD-29's, full stop — SD-30 issues
  no cycles against `spell`/`equipment`/`monster`/`monster_ability`/`race_trait`/`companion`/
  `equipment_modifier`/`feat` in any book, including the four occult/mythic/Inner Sea books it used
  to consider its own territory.
- Every book (not just the old sixteen) that carries `class_feature` units is SD-30's, including
  eleven books the old scope never named (`advanced_class_guide`, `advanced_players_guide`,
  `ultimate_psionics`, `ultimate_combat`, `ultimate_magic`, `core_rulebook`, `ultimate_wilderness`,
  `ultimate_intrigue`, `adventurers_guide`, `advanced_race_guide`, `pathfinder_unchained`,
  `bestiary_6`, `bestiary_4` — thirteen, not eleven; recounted directly from Decision §33's table).
- The four deferred books from the old scope (NPC Codex, Planar Adventures, Occult Origins, Haunted
  Heroes Handbook) are no longer SD-30's to defer — they are SD-29's corpus-wide territory for
  whatever non-`class_feature` kinds they carry (NPC Codex and Planar Adventures remain genuinely
  absent from the corpus per Decision §32's re-verification, so this is moot for them regardless; if
  either is ever acquired, SD-29's corpus-wide lanes pick it up automatically, and SD-30 picks up
  only its `class_feature` units if any exist). `forward-scope-register.md`'s book-specific C2.x
  entries are retired accordingly — see that file.
- The class-grant boundary with SD-28 (Occultist/Spiritualist/Medium/Mesmerist canonical in SD-30,
  Decision §5) is **unchanged** — it was never book-scoped, it is class-identity-scoped, and it
  survives the re-scope untouched.

**No writer collision remains.** SD-29 never touches `class_feature` (its own `§38.4`); SD-30 never
touches anything else. The (kind, book) cell overlap that produced R-29-009 required both packages to
claim the same kind in the same book; after this decision, no kind is claimed by both packages in any
book.

**Authority:** operator directive 2026-08-10; `SD-29-corpus-wide-catch-up-lanes/decisions.md §38.4,
§38.5`; `risks-and-open-questions.md` R-29-009/OQ-29-004 in that package (mirrored resolution added).

## Decision 36 — Epic 14's harness widening does not move to SD-30 (2026-08-10)

**Status:** New. The brief driving this decision asked whether SD-28 Epic 14 (observation-harness
widening — making `spell` and `equipment` magnitudes observable reaching a real consumer, so the
4,050 units parked at `ingested-magnitude` can reach `grounded`) is an orphan SD-30 should absorb.

**Checked directly against Epic 14's own text** (`SD-28-ultimate-book-content-ingestion/epic-breakdown.md`
"Epic 14 (SD28-E14) — Observation-harness widening (spell + equipment consumers)"): Epic 14 is scoped
to `Kind::Spell` and `Kind::Equipment`/`Kind::EquipmentModifier` exclusively. It has no `class_feature`
surface at all — `classify()`'s `Kind::ClassFeature` arm is a completely separate code path from the
`Kind::Spell`/`Kind::Equipment` arms Epic 14 patches.

**Disposition: SD-30 does NOT absorb Epic 14.** `spell` and `equipment` are SD-29's kinds corpus-wide
per `SD-29-corpus-wide-catch-up-lanes/decisions.md §38`. Epic 14's natural home, by kind, is SD-29's
territory, not SD-30's — but this package's write scope does not extend to re-scoping SD-29 (the
brief's own hard constraint), so this decision records the finding and stops there: **Epic 14 stays
where it is (SD-28) until whoever owns spell/equipment corpus-wide formally claims it.** This is
recorded as an open item for the operator, not silently resolved by either package.

**Authority:** `SD-28-ultimate-book-content-ingestion/epic-breakdown.md` "Epic 14" (read in full);
`SD-29-corpus-wide-catch-up-lanes/decisions.md §38` (kind ownership).

## Decision 37 — Launch order: dependency-gated on measurement, not merely sequenced (2026-08-10)

**Status:** New.

SD-30 does not launch its per-class chassis sweep (Epic 6) the way the old per-book epics could have
launched — book-parallel, day one. `§63`'s finding is load-bearing here too: **no chassis-sweep cycle
can be honestly sized until Epic 4's measurement has covered the class it targets.** A cycle that
ingests `class_feature` records for an unmeasured class risks producing records with no wireable
archetype path at all (Companion: 0/7, per `§64`) or, conversely, under-provisioning a class whose
real wireable fraction turns out high (Skald: 100%).

**What must be true before SD-30's content epics (5, 6) can start:**

1. Epic 1 (identifier cleanup) and Epic 2 (operator pre-launch, including cycle-0 trap-report and
   work-inventory validation across the 23 books) complete — unchanged prerequisite from the old
   scope.
2. Epic 3 (PI-screening provenance gate) is wired in — a hard gate on every lane's first content
   commit per book, mirroring SD-29 Epic 3 exactly (same `rules_tables/*.rs` pipeline, same zero-
   PI-screening starting state, same three-leak precedent from `docs/governance/license-matrix.md`).
3. Epic 4 (per-class measurement) has produced a per-class `wired-able / named` figure — hand-
   verified, never proxied — for the class a given Epic 6 cycle targets, before that cycle is
   scheduled. Epic 4 does not need to be 100% complete corpus-wide before Epic 6 starts; it needs to
   be complete **for the specific class(es) the next Epic 6 cycle-batch claims.** This is a per-class
   gate, not a bundle-wide gate — the same shape SD-28's own dispatch used once `§64` landed.
4. Epic 5 (archetype mechanism) has landed the supersession primitive's wiring for a measured class
   before Epic 6 ingests that class's records as "reach-gate satisfied via archetype supersession" —
   ingestion and wiring are sequenced per class, not decoupled.

**Sequential position relative to SD-28/SD-29 restated:** SD-30 launches after SD-28 (source of the
inherited measurement and primitive) and after SD-29's re-scope (source of the now-closed collision).
Both are satisfied — SD-28 is published, SD-29's re-scope is published (`472acb4f`) and its collision
is closed by Decision §35 above. **The remaining precondition is internal to SD-30**, not cross-
bundle: Epic 4 must clear its own gate before Epic 6 can schedule the class it targets.

**Authority:** `SD-28-ultimate-book-content-ingestion/decisions.md §63, §64`; `epic-breakdown.md`
(this package, revised below).

## Decision 38 — `unknown` (2,958 units) is a classification/design question, not a raw ingest gap; owned by Epic 4 (2026-08-10)

**Status:** New. The brief flagged the 2,958-unit `unknown` bucket (~19% of `class_feature`'s
current-corpus population) as needing characterization before scoping, since `status_vocabulary`
defines `unknown` as *"Could not be classified. `reason` says why."* — distinct from `not-ingested`
(book started, this record not found) and `not-started` (book never touched).

**Characterized by reading SD-28's own prior work on this exact bucket, not re-derived from
scratch** — SD-28 Epic 15 spent multiple decisions (`§52`-`§56`, `§61`, `§62`) on precisely this
question for `class_feature`'s `unknown` population, at various points in the corpus's evolution
(the bucket's size moved as classifier defects were found and fixed: 4,172 -> 1,897 via the
zero-magnitude option-pool fix in the "SD28-E15" decision at line 780; then a later snapshot
characterized 2,989 as option-pool-dominated in `§52`). The current corpus-wide figure (2,958,
Decision §33 above) is a later snapshot of the same bucket, shaped by the same dynamics.

**What `unknown` actually means for `class_feature`, per SD-28's direct findings:**

- **The dominant shape (SD-28 `§52`, ~87% of the bucket at that snapshot) is option-pool sub-choice
  content** — named options inside a class-native chooser (Cleric/Inquisitor Domain, Sorcerer/
  Bloodrager Bloodline, Ranger Favored Enemy/Terrain, Alchemist Discovery, Barbarian Rage Power, Bard
  Performance, Rogue Talent, Oracle Mystery, Summoner Eidolon evolutions, and ~850 other named pools).
  This is **not an ingest gap** — the chooser mechanism itself is frequently real and wired (e.g.
  `choice:alchemist_discovery`); what's `unknown` is whether each *specific named option inside the
  pool* has its own magnitude computed. SD-28's own standing design ruling (`§52`, restated
  explicitly as NOT closing any unit's status) is that the engine's real design intent for these
  families is to ground one representative, well-evidenced option per chooser and defer the rest with
  a named diagnostic — **not** to eventually compute every option. Whether the remaining bulk should
  ever move to a status meaning "known, deliberately deferred" rather than "unclassified" is exactly
  the classifier-taxonomy question the brief anticipated, and SD-28 explicitly left it to whoever
  owns the classifier next.
- **A smaller, genuinely-unreachable subset needs new engine code, not reclassification** (`§53`):
  Vigilante's Social Grace/Refined Education (203 units) and Ultimate Psionics' discipline talent
  trees (Insight/Terror/Blade Skill/Path Power, 100 units) have **no chooser code at all** — 303
  units confirmed unreachable by the same reach-check standard used throughout SD-28, needing net-new
  chooser/grounding code to ever leave `unknown`.
- **A separate, smaller feat-side `unknown` bucket** (307 units at SD-28's snapshot) is `kind:feat`,
  not `kind:class_feature` — out of SD-30's scope entirely (SD-29's `feat` lane per Decision §35).
- **A residual unclustered remainder** (1,772 units at SD-28's snapshot, 908 distinct low-count `KEY:`
  prefixes) was left open, uncharacterized, by SD-28 — SD-30 inherits this as unfinished
  characterization work, not a closed question.

**Disposition: owned by Epic 4 (per-class measurement), not Epic 6 (chassis sweep).** The distinction
the brief asked for: `unknown`'s dominant shape is a *classification/design* question ("does this
option-pool family's engine design intend per-option grounding, and if not, what status name reflects
that honestly") answered by the same per-class, hand-verified, no-automated-proxy method Epic 4 already
uses for archetype-slot measurement — not a raw per-record ingest task Epic 6's per-book sweep is
shaped for. Epic 4's per-class measurement pass, when it reaches a class with option-pool `unknown`
units, characterizes that class's pools using SD-28's already-proven method before Epic 6 schedules
any ingest cycle against them. The 303-unit genuinely-unreachable subset and the 1,772-unit
unclustered remainder are recorded as Epic 4 backlog items, not silently dropped.

**Authority:** `SD-28-ultimate-book-content-ingestion/decisions.md` "SD28-E15" unknown-status decision
(line 780, 4,172->1,897 fix), `§52`-`§56`, `§61`, `§62`; `docs/work-inventory.json` `status_vocabulary`.

## Decision 39 — Declared-PI reading (`NAMEISPI`/`DESCISPI`) becomes owned, executable SD-30 scope, ahead of any `class_feature` ingest (2026-08-13, operator directive)

**Status:** New. Operator directive 2026-08-13, verbatim: *"add the pi fix to sd-30"*. Every figure
below was re-derived independently against this checkout at `tranche/9`, not transcribed from the
dispatching brief — the transcribed-instead-of-derived pattern is this program's rank-one recorded
defect class (`SD-29-corpus-wide-catch-up-lanes/decisions.md`, ~50 corrections of that shape).

### 39.1 The defect, as SD-29 found it, cited not re-argued

`SD-29-corpus-wide-catch-up-lanes/decisions.md §53` (race-trait lane, round 5) and `§50` (monster
lane, round 3) independently found the same rule: PCGen declares Product Identity **per record** via
`NAMEISPI:YES` (the record's name is PI) and `DESCISPI:YES` (its description is PI). Every Pipeline A
ingest path already parses these into `raw_tokens`; before `§53`, nothing read them. In their place,
`pi_screening::PI_BLACKLIST_TERMS` — a 55-term hand-maintained list, whose own module doc calls it
"a bounded, documented heuristic … not an exhaustive legal review" — screens shipped prose by term
match alone. `§53.1` measured the two disagreeing 69% of the time in `race_trait` alone: 26 shipped
records declared `DESCISPI:YES`; the blacklist redacted 18 by coincidence and shipped the other 8
(Kodar Mountains, Earthfall, Ekujae, Gogpodda, Omesta, Droskar, Abaddon, Inner Sea). `§53.2`/`§50.3`
independently converged on the same disposition for a `NAMEISPI:YES` row: a name cannot be redacted
without breaking the record's key and cross-references, so the row is **dropped**, not screened.

`§53` fixed this for `race_trait` only: the shared reader (`pi_screening::{DeclaredProductIdentity,
declared_product_identity, classify_optional_field_declared}`) was placed in the shared module, but
`§53.7` recorded explicitly that **only `ingest_race_traits` calls it** — every other Pipeline A
writer, and the entire Python transcription pipeline (Pipeline B: `transcribe_monster_tables.py`,
`transcribe_companion_tables.py`, `classify_monster_ability_rows.py`), still screens on the term list
alone or, for companion, not at all. Verified this checkout, not inherited:

```bash
grep -rln "declared_product_identity\|DeclaredProductIdentity" src/ --include=*.rs
#  -> src/rules_core/pi_screening.rs (the reader itself), src/bin/ingest_race_traits.rs (the only caller)
grep -n "DESCISPI" scripts/transcribe_monster_tables.py scripts/transcribe_companion_tables.py
#  -> no hits: the monster transcriber drops NAMEISPI:YES rows (script lines 780, 818) but has never
#     read DESCISPI:YES at all; the companion transcriber reads neither token
```

### 39.2 Re-derived corpus-wide exposure — commands and results, not transcribed figures

**Already-shipped exposure, all kinds, corpus-wide** (does the blacklist currently miss a declared-PI
row anywhere it has already shipped?):

```bash
python3 -c "
import json,glob,collections
c=collections.Counter()
for p in glob.glob('data/corpus/*/*/*/*.json'):
    d=json.load(open(p)); ks={t['key'].upper() for t in (d['data'].get('raw_tokens') or [])}
    for k in ('NAMEISPI','DESCISPI'):
        if k in ks: c[(k, d.get('pi_marker'))]+=1
print(dict(c))"
```

→ `{('DESCISPI', 'redacted'): 25}` over 4,281 shipped corpus files, all 25 in `core_essentials` (9)
and `inner_sea_races` (16) — exactly `§53`'s fix and nothing else. **The already-shipped exposure
outside `race_trait` is currently zero**, because no other kind's ingest path has yet shipped a
record that carries either token — not because any other path screens correctly. This is a
point-in-time fact, re-checkable at any future gate run, not a standing guarantee.

**`class_feature`'s future exposure — the number that matters to this bundle.** No `class_feature`
ingest path exists yet (`ls src/bin/ | grep ingest` and `ls scripts/*.py | grep -E
'ingest|transcribe'` show no `class_feature` writer). The exposure is therefore entirely upstream, in
PCGen source, scoped to SD-30's 23-book `class_feature` population (`decisions.md §33`, re-confirmed
this session via `docs/work-inventory.json`'s `units` where `kind=='class_feature'`, `books` set):

```bash
# per book, over every *abilities_class*/*classfeats*/*class_abilities* .lst file in that book's
# PCGen source directory (~/workspace/repos/pcgen/data/pathfinder/{paizo,dreamscarred_press}/...),
# counting rows (lines) carrying at least one of NAMEISPI:YES / DESCISPI:YES
```

| book | class-feature `.lst` files | rows w/ ≥1 ISPI token | `NAMEISPI:YES` (drop) | `DESCISPI:YES` (redact) |
|---|---|---|---|---|
| `adventurers_guide` | 1 | **276** | 49 | 268 |
| `inner_sea_magic` | 1 | 67 | 20 | 60 |
| `inner_sea_world_guide` | 2 | 49 | 29 | 29 |
| `inner_sea_intrigue` | 2 | 45 | 11 | 43 |
| `book_of_the_damned_volume_2` | 1 | 18 | 7 | 11 |
| `inner_sea_combat` | 2 | 9 | 8 | 1 |
| (other 17 of 23 books) | — | 0 | 0 | 0 |
| **total** | | **464** | **124** | **412** (some rows carry both) |

**464 of the 23-book `class_feature` population's source rows declare Product Identity that nothing
in this repo currently reads.** Concentrated in 6 books, dominated by `adventurers_guide` (276 rows,
adept archetype/rage-power/hex-style content that is exactly the "named ability + flavor description"
shape both `§50` and `§53` found the term list misses). `ultimate_psionics` is third-party
(`dreamscarred_press`, not Paizo) and carries none. **Zero of this is shipped yet** — it becomes real
exposure only when Epic 6's per-class chassis sweep ingests these 6 books, which is exactly why the
fix must land before that ingest, not after.

**The gate itself does not close this gap.** `scripts/verify.sh`'s `pi-sweep` stage
(`pi_sweep_rules_tables`) is Pipeline B's term-blacklist sweep over `src/rules_core/rules_tables` —
the same 55-term heuristic, downstream of ingest, described in this bundle's own Epic 3
(`epic-breakdown.md` SD30-E3-F1: *"per-class PI-blacklist sweep… or the 55-term blacklist sweep"*).
It does not read `NAMEISPI`/`DESCISPI` either. A green `pi-sweep` run says nothing about whether a
declared-PI row shipped.

### 39.3 Disposition: SD-30 is the right home, at the scale this measured

464 rows across 6 of SD-30's 23 books is real but bounded — it does not argue for a standalone
bundle the way, hypothetically, a five-figure corpus-wide count would. It sits naturally inside
`epic-3-pi-gate`, which already exists as the standing gate ahead of Epic 6's per-book ingest and
already cites `§53`'s sibling finding in SD-29 Epic 3 as "a Pipeline-B finding, not a kind-specific
one." Epic 3's acceptance criteria (`epic-breakdown.md` SD30-E3-F1) currently name only the
blacklist sweep — that is the gap this decision closes. Cards below (`kanban.md`, `epic-breakdown.md`)
make the declared-PI reader, the backfill sweep, and the regression gate real, ordered dispatch work
inside Epic 3, gating Epic 6 exactly as the blacklist sweep already does.

### 39.4 Acceptance shape carried into the cards

- **Production path, not a fixture.** The reader already exists and is unit-tested
  (`pi_screening::declared_product_identity`, 7 tests) — the fix is wiring it into whichever
  ingest/transcription path Epic 6 builds for `class_feature`, mirroring `ingest_race_traits`'s
  pattern (drop `NAMEISPI:YES` before the scope filter, redact `DESCISPI:YES` through the shared
  reader, both counted by file:line in the cycle's receipt), not a standalone script run by hand.
- **The two rulings, both required:** `DESCISPI:YES` → redact through `classify_optional_field_declared`
  (`§53.1`); `NAMEISPI:YES` → drop the row, name it with file:line in the generated module header or
  run receipt (`§50.3`/`§53.2` — independently converged, now the settled rule, not reargued per
  cycle). Reclassifying a declared-PI row as shippable remains `ogl-pi-blacklist.md` §3's per-book
  override, an operator decision, not a cycle's to make.
- **Backfill, corpus-wide, every already-shipped kind.** `§53.7`'s own scope finding: "only
  `ingest_race_traits` calls it… the same command that found this, pointed at the whole corpus rather
  than one kind, is the successor's first move." §39.2's corpus-wide sweep command *is* that first
  move, re-run at whatever point the backfill cycle executes (currently zero hits outside the already-
  fixed `race_trait`, but Pipeline B's monster/companion transcribers still lack `DESCISPI` handling
  entirely, so any future re-ingest of those kinds without this fix reopens the gap silently).
- **Regression gate.** `pi-sweep` (`scripts/verify.sh`, `pi_sweep_rules_tables`) and
  `docs/governance/ogl-pi-blacklist.md` are the existing PI machinery; both are term-blacklist only
  and neither reads the declared tokens. The regression gate belongs alongside them as a sibling
  check, not folded into either — reading a declaration and scanning for undeclared terms are
  different questions (`§53.1`'s "the two are now a union"), and `tests/sd29_declared_product_identity_in_shipped_race_traits.rs`
  is the precedent shape: a corpus-level test reading **shipped files**, extended to walk
  `*/class_feature/*` once that path exists, so a future `class_feature` ingest cannot reintroduce a
  declared-PI row without a red gate.

### 39.5 Ordering constraint, stated explicitly

**The declared-PI reader must be wired into `class_feature`'s ingest/transcription path, and the
corpus-wide backfill sweep run, before Epic 6 schedules its first per-book chassis-sweep cycle.**
Ingesting `class_feature` content first and screening for declared PI later manufactures exactly the
exposure this decision exists to prevent — `adventurers_guide`'s 276 rows would ship through the
55-term blacklist the same way `race_trait`'s 8 undeclared-term rows did before `§53`. `epic-3-pi-gate`
already gates `epic-6-chassis-sweep` in `kanban.md`; this decision adds the declared-PI work inside
that existing gate rather than creating a new one, so the ordering `kanban.md` already encodes now
carries the right acceptance criteria.

### 39.6 Cross-reference

`SD-29-corpus-wide-catch-up-lanes/successor-forward-scope-register.md` carries a pointer to this
decision rather than a duplicate scope entry — the authoritative PI-declaration scope, corpus-wide,
lives here.

**Authority:** `SD-29-corpus-wide-catch-up-lanes/decisions.md §50` (monster lane, `NAMEISPI:YES`
finding, round 3), `§53` (race-trait lane, full ruling + shared-reader landing, round 5);
`src/rules_core/pi_screening.rs` (reader + tests); `src/bin/ingest_race_traits.rs` (only current
caller); `scripts/transcribe_monster_tables.py`, `scripts/transcribe_companion_tables.py` (Pipeline B,
no/partial ISPI handling); `docs/governance/ogl-pi-blacklist.md` §§2-3; `scripts/verify.sh` `pi-sweep`
stage; `docs/work-inventory.json` (23-book `class_feature` population); PCGen source tree
`~/workspace/repos/pcgen/data/pathfinder/{paizo,dreamscarred_press}/` (per-book `.lst` counts, this
session, 2026-08-13).

## Decision 40 — Re-derived `class_feature` split (2026-08-13); Decision 33's by-status snapshot is superseded, not wrong

**Status:** New. Doc-maintenance pass, no code change. Every figure below re-derived independently
against `docs/work-inventory.json` on this checkout (`tranche/9`), not transcribed from the
dispatching brief (which itself flagged the pattern of transcribed-not-derived figures as this
program's rank-one recorded defect class).

```bash
cd ~/workspace/repos/codex
python3 - <<'PY'
import json, collections
d = json.load(open('docs/work-inventory.json'))
U = d['units']
cf = [u for u in U if u['kind']=='class_feature']
print('total', len(cf), 'books', len(set(u['book'] for u in cf)))
print('status', collections.Counter(u.get('status') for u in cf))
print('wiring_class', collections.Counter(u.get('wiring_class') for u in cf))
PY
```

Result: **15,472 `class_feature` units, 23 books** (unchanged from Decision §33 — the population
itself has not moved).

**`wiring_class` split (not previously recorded in this package):** `display` 7,227, `computed`
4,178, `derived` 1,792, `static` 1,191, `ambiguous` 1,084 (sums to 15,472).

**`by_status` split — superseded 2026-08-13:** Decision §33 (2026-08-10) recorded `not-ingested`
9,078, `not-started` 3,293, `unknown` 2,958, `grounded` 109, `deferred-with-reason` 34. Re-derived
today: `not-ingested` **10,203**, `not-started` **1,908**, `unknown` **3,218**, `grounded` **109**,
`deferred-with-reason` **34** (sums to 15,472). `grounded` and `deferred-with-reason` are unchanged;
`not-ingested` is up 1,125, `not-started` is down 1,385, `unknown` is up 260. The corpus population
did not change — `docs/work-inventory.json` was regenerated multiple times between 2026-08-10 and
2026-08-13 by sibling-bundle work (SD-32's `wiring_class` %N-placeholder fix, commit `99efb504`, and
its inventory-determinism fix, commit `44a6af61`), which moved some `class_feature` records across the
`not-ingested`/`not-started`/`unknown` boundary as a side effect of correcting the classifier and the
generator elsewhere in the corpus. **This is a correction to Decision §33's snapshot, not a
retraction** — §33's figures were an accurate read of the inventory as it stood 2026-08-10; the
inventory itself moved under a sibling bundle's unrelated fix. Re-derive `by_status` at the start of
Epic 4/Epic 6 cycles rather than citing either snapshot, per the standing "generated, never
hand-maintained" rule (`decisions.md §12`).

**Verified-by:** the command above, this session, 2026-08-13. `scripts/retro.py correction` emitted
for the `by_status` figures (subject: this package's own Decision §33; the orchestrator's
2026-08-13 dispatch brief transcribed the current split correctly and needed no correction).

## Decision 41 — Instrument coverage: SD-32's corpus-wide gates apply to `class_feature`'s future
`static`/`derived` shipments; the `computed` bucket still has none; SD-30 builds neither, uses both
(2026-08-13)

> **CLOSED UNDER THE FOLD — 2026-08-13, later same day (`decisions.md §43`, corrective doc pass).**
> This decision's closing paragraph and its "flagged for the operator, not decided here" question —
> whether a `class_feature` consumer-delta probe should be built inside SD-30 or requested of SD-32 —
> is **CLOSED**, not left open. Decision §43 folds SD-32 as a package into SD-30; there is no longer
> a second bundle to request the probe of, so the ownership question this decision deferred
> dissolves by construction. Building the still-missing `computed`-bucket consumer-delta probes
> (`class_feature` and any other kind that needs one) is now squarely SD-30's own scope — see
> Decision §43 point 2 and `epic-breakdown.md`'s Epic 0/Epic 4-5. The coverage table and analysis
> below remain accurate as a record of what existed on 2026-08-13 and are left uncorrected in body,
> per this project's correct-in-place convention.

**Status:** New (open question CLOSED by Decision §43, 2026-08-13, same day — see box above). Doc-maintenance pass, resolving whether SD-30's Epic 4/Epic 6 acceptance criteria
should plan to build a static-sweep or evaluator-vs-fixture instrument of their own, now that SD-32
has landed both, corpus-wide, on this branch.

**What SD-32 actually landed, verified by commit, not by its own README's prose:**

- `feat(verify): corpus-literal byte-equality sweep, the missing static-unit gate` (`3ad45909`) —
  `scripts/verify.sh` stages `corpus-sweep` / `corpus-sweep-selftest` run `corpus_literal_sweep`
  "over every shipped record in `data/corpus/`" (`scripts/verify.sh:876`) — **corpus-wide, all
  kinds**, not scoped to the equipment/spell content SD-32 itself was funded to move. Wired into
  both `ALL_STAGES` and `QUICK_STAGES` (`scripts/verify.sh:102-103`).
- `feat(instruments): evaluator-vs-fixture check, the missing 'derived' bar` (`527d1db6`) +
  `feat(instruments): corpus-derived fixtures for the derived evaluator check` (`7f70c45d`) —
  `tests/derived_evaluator_fixture_check.rs`, also corpus-wide by construction (fixtures are derived
  from the corpus record, not hand-authored per kind).
- `feat(spell): ground the 623 CRB spells...` (`90bd9975`) — a **spell-specific** consumer-delta
  probe (`probe_spell_effect_wiring`, `src/bin/v06_work_inventory.rs:2178`). This is a `computed`-
  bucket instrument, but it is scoped to `Kind::Spell` only (`classify()`'s `Kind::Spell` arm) — it
  is a **precedent for the shape of a `class_feature` consumer-delta probe, not itself one.**
  Confirmed by direct grep of `src/bin/v06_work_inventory.rs` this session: `probe_feat_effect_wiring`,
  `probe_race_trait_corpus`, `probe_equipment_effect_wiring`, `probe_spell_effect_wiring` all exist;
  no `probe_class_feature_effect_wiring` (or equivalent) exists anywhere in the file.
- `fix(rules_core): teach wiring_class the %N placeholder...` (`99efb504`) — a `wiring_class`
  classifier fix, corpus-wide, already reflected in Decision §40's re-derived split above (it is part
  of why `by_status` moved between 2026-08-10 and today).
- SD-32 `decisions.md §2` records that `static`/`derived` currently have **no `done` rung** in the
  dashboard's `doneness_verdict()` table regardless of instrument coverage — that question is
  explicitly OPEN, gates SD-32's own Epics 5/6 from moving units to `done`, and is unaffected by this
  decision. SD-30 citing these gates for verification does not require that question to be resolved.

**Disposition — no epic in this package builds a static-sweep or evaluator-vs-fixture gate.** Re-read
of `epic-breakdown.md` and `acceptance-and-verification.md` (this session) found **neither document
ever proposed building one** — Epic 4's method is hand-verification with no automated proxy by
design (`§63`/`§64`, inherited whole), and Epic 6's reach-gate is a different mechanism
(`apps/desktop/src-tauri/src/reach_gate.rs`, a player-surface check, not a corpus-literal or
evaluator check). There was no stale acceptance criterion to rewrite. What this decision does instead:
names, for the record, that once Epic 6 ships `class_feature` records, `scripts/verify.sh`'s
`corpus-sweep`/`corpus-sweep-selftest` stages and `tests/derived_evaluator_fixture_check.rs` already
cover those records' `static` (1,191 units, corpus-wide) and `derived` (1,792 units, corpus-wide)
`wiring_class` populations for free — Epic 6 does not need a bundle-specific sibling test, only to run
`./scripts/verify.sh` per its own standing Decision §18/`AT-30-002`.

**Coverage count, stated plainly:**

| `wiring_class` | units | existing corpus-wide gate | still needs building |
|---|---:|---|---|
| `display` | 7,227 | none (bar is `text-complete`, a display-content check, not this decision's subject) | — |
| `computed` | 4,178 | **none** — `corpus-sweep` and the derived-evaluator check do not cover `computed`; the spell probe is `Kind::Spell`-only | a `class_feature` consumer-delta probe, modeled on `probe_spell_effect_wiring`, is unbuilt — this is Epic 4/Epic 5's territory (measuring/wiring the archetype supersession and chooser mechanisms IS the consumer), not a new standalone gate |
| `derived` | 1,792 | `tests/derived_evaluator_fixture_check.rs` (corpus-wide) | none — gate exists |
| `static` | 1,191 | `scripts/verify.sh` `corpus-sweep`/`corpus-sweep-selftest` (corpus-wide) | none — gate exists |
| `ambiguous` | 1,084 | none (by definition — resolved by SD-32 Epic 4's classifier work or SD-30's own per-record read, not a gate) | — |

**Of the 15,472 `class_feature` units, 2,983 (`static` + `derived`) have an existing corpus-wide gate
they will pass through automatically once ingested; 4,178 (`computed`) have none and are not
gated by anything this decision can point to — Epic 5's supersession/chooser wiring plus the
reach-gate (`AT-30-002`) is the closest analog SD-30 already owns, and building a dedicated
consumer-delta probe (spell's shape, applied to `class_feature`) is named here as a candidate for
whoever schedules Epic 4/5 work, not decided as in-scope by this doc pass.**

**Flagged for the operator, not decided here:** whether a `class_feature` consumer-delta probe
(mirroring `probe_spell_effect_wiring`) should be built inside SD-30 (natural home — it is Epic 4/5's
own measurement territory) or requested of SD-32 (natural home — SD-32 already owns the sibling
probes for every other kind and the anti-gaming discipline, `decisions.md §1`, that governs how such
a probe must be judged). This document does not resolve that ownership question; see Decision §42
below for the boundary rule that applies regardless of which bundle eventually builds it.

**Authority:** `docs/work-inventory.json` (this session); `scripts/verify.sh` lines 102-103, 826-938
(read directly); `tests/derived_evaluator_fixture_check.rs` (existence confirmed, `find`/`grep` this
session); `src/bin/v06_work_inventory.rs` (probe function inventory, grepped this session);
`SD-32-instrument-coverage-and-consumer-wiring/decisions.md §2, §9` (read in full); commits
`3ad45909`, `527d1db6`, `7f70c45d`, `90bd9975`, `99efb504` (`git show --stat`, this session).

## Decision 42 — Boundary with SD-32: instruments vs. content, the same no-dual-ownership shape as the
SD-29 boundary (2026-08-13)

> **CORRECTED IN PLACE — 2026-08-13, later same day.** The operator has ruled that SD-32 should never
> have existed as a separate package — its creation was a dispatch error, not a durable scope split.
> This decision's entire premise (SD-32 as a coexisting sibling bundle with its own instrument-vs-
> content ownership lane) is **wrong** under that ruling. SD-32's *content* is not reverted and not
> in question — the corpus-literal sweep, the derived-evaluator check, the spell consumer-delta
> probe, the `wiring_class` %N fix, and the inventory-determinism fix all stand exactly as landed and
> exactly as described below. What changes is the **package boundary**: SD-32 folds into SD-30, which
> now owns this work's continuation directly, not across a cross-bundle boundary. There is no longer
> a "which bundle builds the `class_feature` consumer-delta probe" question of the kind this
> decision's closing paragraph flags — SD-30 owns it outright. See Decision §43 for the operator
> ruling and its consequences. Left visible below, uncorrected in its body text, per this project's
> correct-in-place convention (`decisions.md §12` normalizes this — never delete a decision, cite over
> it).

**Status:** New (superseded in part by Decision §43, 2026-08-13, same day — see box above). SD-30 has not previously mentioned SD-32 anywhere in this package. SD-32 landed on
this branch (`tranche/9`) today, 2026-08-13, and both bundles now touch the same
`docs/work-inventory.json`-derived surface (SD-32 by kind-agnostic instrument coverage, SD-30 by the
`class_feature` kind specifically), so the same collision shape Decision §35 resolved against SD-29
(`SD-29-corpus-wide-catch-up-lanes/decisions.md §38.4-§38.5`: no (kind, book) cell owned by both
bundles) needs a sibling statement here, structured the same way.

**The boundary, stated as a rule, not a cell list (instruments aren't kind/book-shaped, so §35's
exact table doesn't transfer — the axis here is measurement-tooling vs. content):**

- **SD-32 owns the instruments and gates themselves** — `corpus_literal_sweep`, `verify.sh`'s
  `corpus-sweep`/`corpus-sweep-selftest` stages, `tests/derived_evaluator_fixture_check.rs`, the
  `wiring_class` classifier (`src/rules_core/wiring_class.rs` and friends), the per-kind consumer-delta
  probes in `src/bin/v06_work_inventory.rs` (`probe_spell_effect_wiring`, `probe_equipment_effect_wiring`,
  `probe_race_trait_corpus`, `probe_feat_effect_wiring`), and the dashboard's `doneness_verdict()`
  table question (`SD-32 decisions.md §2`, still OPEN). Any future `class_feature` consumer-delta
  probe, if and when it is built, is instrument work under this same ownership rule regardless of
  which bundle's cycle writes it — see the flagged question at the end of Decision §41.
- **SD-30 owns `class_feature` content and consumption of those instruments** — the archetype
  supersession/chooser mechanism (`archetype_resolver.rs`, `pilot_compute.rs`'s `class_feature`
  branches), the per-class hand-verification measurement (Epic 4), the `class_feature` ingest itself
  (Epic 6), and running the existing gates against what it ships. SD-30 does not modify
  `wiring_class.rs`, `corpus_literal_sweep`, or any `probe_*` function in `v06_work_inventory.rs` —
  if a cycle's work would require touching one of those, per Decision §41 it is a flagged
  cross-bundle question, not a unilateral SD-30 edit.
- **Neither bundle builds the other's deliverable.** SD-32 does not ingest `class_feature` content
  (its own `decisions.md §7`: "`not-started` is content that is not in the engine... belongs to the
  SD-29/SD-30 lanes, not here"). SD-30 does not build or modify a corpus-wide instrument, even one
  scoped to `class_feature` alone, without the same flagged-not-decided treatment Decision §41 gives
  the `computed`-bucket probe question.
- **No writer collision today.** SD-32's write scope (`README.md` "Authority surface") is
  `src/bin/v06_work_inventory.rs`, `src/rules_core/**` equipment-effect surfaces, `tests/**`, and its
  own package — none of it is `class_feature`-specific code or `src/rules_core/rules_tables/<book>/`.
  SD-30's write scope (`AT-30-001`) is the `class_feature` rules-tables/archetype surface and its own
  package. The overlap is read-only: both bundles read `docs/work-inventory.json` and cite
  `scripts/verify.sh`.

**Authority:** `SD-29-corpus-wide-catch-up-lanes/decisions.md §38.4-§38.5` (the precedent shape, read
in full this session); `decisions.md §35` (this package's own prior resolution of an identical
collision); `SD-32-instrument-coverage-and-consumer-wiring/README.md` ("Authority surface");
`SD-32-instrument-coverage-and-consumer-wiring/decisions.md §7`. Cross-reference pointer added to
`SD-32-instrument-coverage-and-consumer-wiring/forward-scope-register.md` in this same change (small
pointer only, no scope duplicated there).

## Decision 43 — Operator ruling: SD-30 widens to drive ALL kinds to `done`, corpus-wide; SD-32's
package folds into SD-30, its content unreverted (2026-08-13)

**Status:** New. Operator ruling, issued after Decision §42 landed on `tranche/9`, superseding it in
part (see the correction box prepended to §42 above).

**The ruling, verbatim shape:**

1. SD-32 should never have existed as a separate package — its creation was a dispatch error, not a
   durable scope split. Its **content**, already merged on `tranche/9`, is **not reverted and not in
   question**: the corpus-literal byte-equality sweep and `corpus-sweep`/`corpus-sweep-selftest`
   `verify.sh` stages (`3ad45909`), the evaluator-vs-fixture derived check
   (`527d1db6`/`7f70c45d`), the spell consumer-delta probe grounding the 623 CRB spells (`90bd9975`),
   the `wiring_class` %N-placeholder fix (`99efb504`), and the `v06_work_inventory` determinism fix
   (`44a6af61`) all stand exactly as landed.
2. **SD-32 as a package folds into SD-30.** SD-30 now owns that work's continuation — building the
   still-missing `computed`-bucket consumer-delta probes (`class_feature` and any other kind that
   needs one, modeled on `probe_spell_effect_wiring`), the still-open `static`/`derived` "no `done`
   rung" dashboard-producer question named in former Decision §41, and any further corpus-wide
   instrument work — directly, not across a cross-bundle boundary. Decision §42's boundary rule (SD-32
   owns instruments, SD-30 owns content, neither touches the other's surface) is void: there is only
   one bundle now, so there is no boundary to keep.
3. **SD-30's charter widens** from `class_feature`-only to **driving ALL KINDS to `done`, corpus-wide**
   — not just `class_feature`. This is a superset, not a replacement: `class_feature`'s existing Epic
   1-9 structure, measurement gate, and 23-book scope (Decision §33) stand unchanged and continue: See
   `scope-draft.md`'s new "Widened charter" section and `README.md`'s widened Purpose/In-scope
   sections for the operative restatement.

**The crux this decision records, re-derived from live data, not the operator's transcribed figures
alone (see re-derivation below): `grounded` != `done`.** `grounded` means the engine holds the record
and the corpus-side value has been observed matching (`status_vocabulary`). `done` additionally
requires the unit to clear its own `wiring_class` bar — the dashboard producer's `doneness_verdict()`
table (transcribed and validated live, `SD-32-.../artifacts/derive-movable-mass.py`): a `display` unit
is `done` only at `text-complete` (grounded-but-not-text-complete is `held`, not `done`); a `computed`
unit is `done` only when a consumer has actually read the magnitude (`status == "grounded"` under the
`computed` wiring class specifically, gated per-kind by whether a consumer-delta probe exists at all —
`spell` and `companion` have none, so their `computed`/`in-progress` units are capped to `held`, not
counted `in-progress`, by the `NO_GROUNDING_PROBE` rule); a `static` or `derived` unit is `held`, never
`done`, until a `done` rung is added to the verdict table (the open question former Decision §41
flagged — still open, now SD-30's own question, not a cross-bundle one).

**Re-derivation, this session, against the live inventory — command run:**

```sh
cargo run --locked --bin v06_work_inventory   # regenerated docs/work-inventory.json, stamp 2026-08-13T20:45:47Z
python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py
```

validated (`transcription validated against live dashboard: True`) against the live dashboard's
`work_inventory.by_doneness` payload at the same inventory stamp — the re-derived `by_doneness` split
and the dashboard's own cache agree exactly:

`done` 3,464 · `held` 9,455 · `in-progress` 716 · `not-started` 21,303 · `unmeasurable` 3,547 ·
`deferred` 36 (sums to 38,521 units after the `beginner_box`-excluded-book convention). Separately,
overall `grounded`-status units (a `status`, not a `verdict`) = **5,349** — this is the figure the
operator's brief cites as "overall grounded"; it is not the same axis as `by_doneness` and the two do
not sum against each other (a `grounded` unit can resolve to `done`, `held`, or `in-progress`
depending on its `wiring_class` and kind).

**Per-kind `grounded` vs. `done`, re-derived this session (`grounded` = status count; `done` = verdict
count; `%` = done/total-units-of-that-kind):**

| kind | total units | grounded | done | done % |
|---|---:|---:|---:|---:|
| class | 185 | 27 | 27 | 14.6% |
| class_feature | 15,472 | 109 | 18 | 0.1% |
| companion | 1,696 | 922 | 416 | 24.5% |
| equipment | 6,208 | 145 | 277 | 4.5% |
| equipment_modifier | 1,580 | 55 | 896 | 56.7% |
| feat | 2,610 | 77 | 1,178 | 45.1% |
| monster | 1,270 | 1,242 | 7 | 0.6% |
| monster_ability | 3,107 | 1,629 | 334 | 10.7% |
| race | 103 | 7 | 0 | 0.0% |
| race_trait | 3,447 | 513 | 264 | 7.7% |
| spell | 2,843 | 623 | 47 | 1.7% |
| **TOTAL** | **38,521** | **5,349** | **3,464** | **9.0%** |

**Verification of the operator's cited figures (do not trust blindly, per this dispatch's own
instruction — checked against the table above):** overall grounded 5,349 (**exact match**) vs. done
~3,464 (**exact match**); races 7 grounded / 0 done (**exact match**); spells 623 grounded / ~46 done
(re-derived: 47 — **matches within the operator's own "~"**); classes ~15% done (re-derived: the
`class` kind, not `class_feature`, is 14.6% done — **matches**, and this decision flags explicitly
that "classes" in the operator's brief means the `class` kind, a distinct corpus kind from
`class_feature`, which is 0.1% done, not ~15%); races ~0% done (**exact match**); spells ~1.7% done
(**exact match**). **No operator figure was found wrong this session** — all verified against a fresh
`v06_work_inventory` regeneration and the live dashboard cache, not transcribed from a prior report.

**Recoverable-work split, re-derived and confirmed exact:** `held` 9,455 (engine holds real data,
unproven — the largest cheap lever, since `corpus_literal_sweep`, the derived-evaluator check, and the
spell-probe pattern already exist and need only be *applied*, not built); `not-ingested` 17,209 (needs
real per-book ingest, expensive — `status == "not-ingested"` summed corpus-wide); `unknown` 3,547
(unmeasurable by any instrument, `status_vocabulary`'s "could not classify" bucket — **not** assumed
uniformly unreachable; per-kind residue is `class_feature` 3,218, `feat` 329, with every other kind at
0, meaning the `unknown` bucket is **not evenly distributed** and is concentrated almost entirely in
the two kinds SD-30 already measures by hand, `class_feature`'s Decision §38 method applies to that
3,218 directly, `feat`'s 329 needs its own characterization pass — not yet done, flagged as new SD-30
scope below).

**Honest ceiling — how far `done` can go via instrument-application alone, no new ingest:** the 9,455
`held` units are exactly the units `done` can reach without new per-book ingest, *if and only if* every
kind's missing `done` rung and missing consumer-delta probe gets built and applied. That ceiling is
not uniform: `equipment`'s 4,676 `held` units are `static`/`computed` blocked on a `static`/`derived`
`done` rung (4,511 of them) or a compiled-table probe-universe gap (A1/A2, 715 units, `equipment`/
`equipment_modifier` combined); `monster`/`monster_ability`/`companion`'s 3,036 combined `held` units
are `derived`/`static` blocked the same way; `spell`'s 1,235 `held` are capped by the
`NO_GROUNDING_PROBE` rule (no consumer reads a spell magnitude corpus-wide, `grounded == 0` for that
kind regardless of `status`); `class_feature`'s 91 `held` are the same `static`/`derived` rung gap.
**Ceiling via instrument-application alone: `done` could rise from 3,464 to at most 3,464 + 9,455 =
12,919 (33.5% of 38,521)** — and only once every rung/probe gap above is closed; today, with none of
them closed, `held` is inert. Beyond that ceiling, the remaining 21,303 `not-started` (17,209
`not-ingested` + 4,094 `not-started`-status) and 3,547 `unmeasurable` units require real per-book
ingest and, for `unknown`, per-kind classification work — no instrument fixes those without new
content landing in the corpus.

**PI-gate discipline, restated, not relaxed:** the PI-screening provenance gate (`decisions.md §39`,
Epic 3 cards SD30-E3-F2/F3/F4) remains hard-blocking on all ingest regardless of this widened charter
or any closure-pressure argument. Widening the charter to all kinds does not create license to ingest
before F2 clears; it only widens which kinds' ingest cycles are subject to the same gate.

**Authority:** `cargo run --locked --bin v06_work_inventory` (this session, stamp
2026-08-13T20:45:47Z); `python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py`
(this session, output captured, validated against the live dashboard payload); operator ruling,
2026-08-13, transcribed in the dispatch brief for this doc pass.

## Decision 44 — Operator ruling: SD-29's per-book ingest lanes fold into SD-30 too, closing the
question Decision §43 left open (2026-08-13)

**Status:** New. Operator ruling, issued the same day as Decision §43, closing the "What this
widening does NOT authorize" flag §43 raised and `scope-draft.md`'s "Widened charter" section left
open pending a separate operator decision.

**The ruling, verbatim shape:** "yes, fold the ingest lanes into SD-30 too." Concretely: SD-30 now
owns not just instrument-application-to-`done` (Decision §43's widening) but also the per-book
*ingest* work that used to live in SD-29's corpus-wide lanes (`SD-29-corpus-wide-catch-up-lanes/decisions.md
§38`, the kind-lane re-cut). This closes the exact question §43 flagged and left undecided.

**Why — re-derived this session, not trusted from the dispatch brief's transcribed figures alone.**
Re-ran the inventory and re-confirmed §43's own table is still current (no material drift since the
20:45:47Z stamp — spot-checked `done`/`grounded`/`held` per kind against the live
`docs/work-inventory.json`, unchanged at the digit §43 already recorded):

- **The ceiling via instrument-application alone is 12,919 of 38,521 (33.5%)** — `done` 3,464 +
  `held` 9,455, per §43's own derivation. That ceiling is real but it is a ceiling: it cannot move
  `done` past it, because the remaining 21,303 `not-started`/`not-ingested` units and 3,547
  `unmeasurable` units have no data in the engine for any instrument to apply to. Only real per-book
  ingest closes that gap.
- **The kinds the operator is most unhappy about are precisely the ingest-blocked ones**, not the
  instrument-blocked ones — per §43's per-kind table: `monster` 1,242 grounded but only 7 `done`
  (0.6%; 1,235 of the 1,242 sit `held`, capped by the missing `derived`/`static` `done` rung, and the
  remaining ~28 are genuinely `not-started` — the *grounding* itself came from ingest, and most of
  the population, 21,303 corpus-wide-`not-started` units, never got that far); `spell` 47/2,843 done
  (1.7%, and per §43, `spell`'s `computed` bucket has no consumer-delta probe at all —
  `NO_GROUNDING_PROBE` caps it regardless of ingest); `race` 0/103 done (0.0%, 7 grounded, the
  smallest and most starkly ingest-starved kind in the corpus); `class_feature` 18/15,472 done
  (0.1%) — already SD-30's own Epic 6 chassis-sweep lane, unaffected by this fold, cited here only
  because it is the fourth kind the operator named. None of these four move meaningfully without new
  ingest; instrument-application alone (Decision §43) tops out at moving `held` units, and three of
  the four kinds above have most of their population sitting in `not-started`, not `held`.
- **SD-29 is CLOSED** (`SD-29-corpus-wide-catch-up-lanes/decisions.md §70`, "SD-29 IS CLOSED. Every
  lane is at a *measured* ceiling, not an argued one"). Its ingest lanes have had **no live owner**
  since that closure landed — SD-29's own closure decision states each lane closed at a measured
  ceiling, not that the corpus-wide ingest need was exhausted (§70's own table still shows
  `not-started`/chassis-blocked residue in every lane it measured). **SD-30 inherits these lanes by
  default, not because a new successor bundle was spun up to receive them** — there is no other
  package positioned to take them, and letting them sit ownerless while the operator's stated
  priority (closing exactly these kinds) goes unaddressed is worse than the widening this decision
  authorizes.

**What SD-30 inherits, concretely — SD-29's hard-won operating lessons, not a blank restart:**

1. **Raw remainder is not workload** (`SD-29-corpus-wide-catch-up-lanes/decisions.md §44.4`, refined
   by `§45.1`/`§49.2`): of the corpus's 3,447 `race_trait` units, only 553 carry a
   `TYPE:<Race> Racial Trait` component naming one of the 18 races the engine models; the other 2,894
   belong to races with no chassis and **no amount of ingest grounds them** — `RaceCorpus::resolve`
   returns `None` without a chassis. Every ingest-lane card SD-30 opens must run the same
   raw-vs-workable split before planning cycles against it, and record the command used.
2. **Screen before committing a round** (`SD-29-corpus-wide-catch-up-lanes/decisions.md §45.1`, "the
   queue was backwards, and the correction is the round's most reusable output"): run the checked-in
   classifiers — `scripts/classify_race_trait_rows.py`, `scripts/classify_companion_rows.py`,
   `scripts/screen_pcc_load_gates.py` (all three verified present in this repo's `scripts/` this
   session) — against a candidate book *before* committing a cycle to it, not after.
3. **Corpus shape traps are hard stops, not silent skips.** `SD-29-corpus-wide-catch-up-lanes/decisions.md
   §34`/`§36` (independently re-verified at `§35`, "not merely re-cited"): `bestiary_5` and
   `bestiary_6` carry **zero** monster records — both are player-options datasets (race/feat/
   companion-mod `.lst` files only). A per-monster cycle dispatched against either book is a
   reportable hard stop. Separately, `SD-29-corpus-wide-catch-up-lanes/decisions.md §68`/`§68.1`
   found negated PCC load gates exclude **719** units corpus-wide (`scripts/screen_pcc_load_gates.py`,
   verified present) — units a naive count would treat as workable but that PCGen's own load rules
   never surface; this exclusion gets *more* likely to fire as more books land, so every new ingest
   card must screen for it, not assume it stays constant.
4. **The PI gate stays hard-blocking — this fold makes it more important, not less.** This package's
   own `decisions.md §39` (Epic 3, cards SD30-E3-F2/F3/F4) already found 464 declared-PI (
   `NAMEISPI`/`DESCISPI`) rows across 6 books in the `class_feature` source alone that nothing in this
   repo currently reads, and that `scripts/verify.sh`'s `pi-sweep` stage does not catch them (it is a
   term-blacklist sweep, not a declared-PI reader). `SD-29-corpus-wide-catch-up-lanes/decisions.md
   §50.1` independently found the same gap corpus-wide (`NAMEISPI:YES` read by nothing in this repo).
   Folding SD-29's ingest lanes in widens which kinds' ingest cycles are subject to this gate — it
   does not relax it. No ingest card opened under this decision may claim before its book's
   declared-PI screen (SD30-E3-F2/F3) is clean.

**Disposition:** `epic-breakdown.md` and `kanban.md` gain a new Epic 10 (SD30-E10, "Corpus-Wide
Ingest Lanes, folded from SD-29") carrying dispatchable per-kind cards for `monster`, `spell`,
`race`, and `race_trait` (the kinds with the largest `not-started`/chassis-open residue per §43's
table that are not already covered by SD-30's own Epic 6 `class_feature` lane), each required to
apply lessons 1-4 above before its first cycle claims. `scope-draft.md` gains a restated combined
ceiling (instruments + ingest together) — see its "Combined ceiling" section, added this pass — which
does not claim 100% is reachable; a bounded chassis-blocked/mechanism-blocked/`unknown` residue
remains structurally unreachable regardless of ingest effort.

**Open-question closure:** this decision closes the question `scope-draft.md`'s "Widened charter"
section left open ("What this widening does NOT authorize... not decided here") and supersedes the
README.md `decisions.md §43` correction box's claim that "SD-29's per-book content-ingest ownership
for non-`class_feature` kinds is unchanged." Both files get a dated correction box pointing here; the
original text stays visible per this package's standing convention.

**Authority:** operator ruling, 2026-08-13 (same day as §43, later in the day), transcribed in the
dispatch brief for this doc pass; `SD-29-corpus-wide-catch-up-lanes/decisions.md §70` (closure
record), `§44.4`/`§45.1`/`§49.2` (race-trait chassis split), `§34`/`§35`/`§36` (zero-monster books),
`§68`/`§68.1` (negated-PCC-gate 719-unit finding), `§50.1` (declared-PI corpus-wide finding); this
package's own `decisions.md §39` (PI-screening gate, 464-row finding) and `§43` (ceiling table,
re-verified unchanged this session).
