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
