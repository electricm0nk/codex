---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-07-25; bundle authored from spec-domain-bundle-authoring skill + forward-scope register from v0.6 + SD-26 docs)
date: 2026-07-25
canonical_branch: tranche/7 (operator directive forthcoming)
kanban_board: codex-tranche-7 (governed convention slug; operator override on file)
companion_to: ./scope-draft.md
mirror_of: ./scope-draft.md
---

# SD-27 — Decision Record

## 1. SD-27 commits to the 19 future-state book ingests (operator directive 2026-07-25, refined 2026-07-27)

**Decision (operator-pinned 2026-07-25, refined 2026-07-27):** SD-27 ships the resolution of the 19 future-state book stubs that SD-26 registered in Epic 4. **Beginner Box and Core Essentials were removed from scope per operator directive 2026-07-27** (redundant to other tomes; will not be brought in); the bundle's two in-scope future-state books (Advanced Race Guide, Pathfinder Unchained) are the only per-book cycles SD-27 dispatches, with the remaining 17 deferred future-state books landing in SD-28+ (Adventurer's Guide among them, routed to SD-30 per the operator's dashboard). The payload is content ingestion for an existing engine — no new chassis, no new rule mechanics, no new class engines.

**Authority surfaces cited:**

- `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/release-notes.md:29-31` — the **21** stubs registered (verified 2026-07-27: the release note reads "21 PF1 books registered"; 19 is the post-2026-07-27 in-universe count after Beginner Box and Core Essentials were descoped, not the registered count).
- `docs/governance/wired-integration-stubs-registry.md` entries **#0003-#0023** — the 21 `book_stub` entries (verified 2026-07-27; #0001-#0002 are not book stubs). All 21 currently carry `planned_resolution_bundle: "SD-27+ (unscheduled)"`; cycle 2.0 resolves the label.
- `data/stubs/*.json` (**21 files** on disk, verified 2026-07-27) — stub manifests with `content_kind_counts: null`. 19 are in the bundle's universe; `beginner_box.json` and `core_essentials.json` are descoped but still present.
- `docs/release/v0.6/risks-and-open-questions.md` §"Open questions" item 2 (line 102, verified 2026-07-27) — the v0.6 swarm's own decision that the label discrepancy is "out of scope for this swarm — it's an SD-26/SD-27 bundle-labeling question, not a v0.6 alpha content question, and the swarm's `data/stubs/` work doesn't depend on which label is 'correct.' Leave both as-is; whoever launches SD-27 reconciles it then."

## 2. Bundle label resolution gates Epic 2 (operator-pinned 2026-07-25)

**Decision:** The `SD-27` vs. `SD-27+ (unscheduled)` discrepancy is the bundle's first cycle (Criterion 2.0). The lead does not pick a side; the operator picks. Both resolutions are internally consistent; the lead's job is to propagate the operator's choice across all 20 surfaces (19 `data/stubs/*.json` + `decisions.md:102`) before any further cycle dispatches.

**Why this is a per-cycle blocking decision.** Per `forward-scope-register.md` §"Class 0," every cycle that touches a `data/stubs/*.json` file or a registry entry blocks on the resolved label. The bundle's first cycle (2.0) lands the resolution.

**Authority:** `forward-scope-register.md` §"Class 1.2" + `forward-scope-register.md` §"Class 0.1."

**Corroboration (found 2026-07-27, independent multi-agent review):** SD-26's own closure notes already
flagged this exact discrepancy as unresolved. `origin/develop`'s `docs/release/SD-26-.../progress.md`
`## DISCOVERED` section states: *"`decisions.md §10` + `risks-and-open-questions.md §4` Q2 pin the
default to `"SD-27"`, but all 21 landed E4 book_stub entries instead carry `"SD-27+ (unscheduled)"`...
Operator must either (a) correct `decisions.md §10`/Q2 to `"SD-27+ (unscheduled)"`, or (b) confirm
`"SD-27"` and correct all 21 `data/stubs/*.json` entries."* This is the same fork cycle 2.0 resolves;
SD-26 punted it forward rather than resolving it, so it lands here as the first cycle regardless of
which side the operator eventually picks.

## 3. SD-27 inherits the Workflow-orchestrated dispatch (operator directive 2026-07-21)

**Decision:** SD-27's dispatch shape is the `Workflow` orchestrator per `docs/governance/loop-instruction-template.md §2` + skill `workflow-orchestrated-dispatch`. Same shape as SD-25 / SD-26's orchestrator with different per-epic concurrency + tiering.

> **Superseded on mechanism by §19** (added 2026-07-27). This directive predates SD-26's 2026-07-22 dispatch-mechanism correction. The *shape* stands; the *dispatcher* is the in-harness `Workflow` tool plus `scripts/sd27-workflow.py`, not a headless shell script. See §19.

## 4. Per-epic concurrency + tiering map

| Epic | Parallel? | Subagent tier | Notes |
|------|----------|---------------|-------|
| E1 Identifier Cleanup | no | Sonnet | Single cycle |
| E2.0 Label Resolution | no | Sonnet | Per-cycle blocking decision; gates 2.0.5+ |
| E2.0.5 Shape B v1 license-stripping pre-flight | no | Sonnet | Schema bump + per-book `LICENSE.json` + PI-blacklist + redaction policy. Gates 2.0.6+ |
| E2.0.6-2.0.9 4 in-scope book license retro-fit | yes (file-disjoint) | Sonnet | CRB, APG, ACG, Bestiary 1. Gates 2.0.10 |
| E2.0.10 All-23-books license-conformance verify | no | Sonnet | Dual-audit gate across all 23 books (4 in-scope + 2 pre-built future-state + 17 deferred future-state stubs). Gates 2.1+ |
| E2.1-2.2 Per-book pre-build + verify cycles (2 future-state) | yes (file-disjoint, serial-operator-gated) | Sonnet (or operator-approved free/discounted model) | 2 cycles (ARG + PU), each as a pre-build + verify pair. The pre-build creates `data/corpus/<book>/`; the verify confirms. **The other 17 future-state books are deferred to SD-28+, operator-gated on SD-27 closing cleanly.** |
| E3 PCGen Parity Baseline | yes after 2.0 begins | Sonnet (or operator-approved free/discounted model) | 2 cycles, one per in-scope future-state book (ARG + PU); file-disjoint with E2 |
| E4 Closure Epilogue | no | Haiku (4.3, 4.4); Sonnet (4.1, 4.5); Opus (4.2) | Subagent tiering per criterion |

**Tier-1 model swap authority.** The operator authorized on 2026-07-25 that a free or discounted model may run the per-book cycle bodies (E2.1-2.2, E3.1-3.2) provided the dispatch instructions are tight enough that the model does not need to think beyond templated execution. The architectural pattern is templated; the per-book content is mechanically extractable from the LST corpus. The boundary is the dual-audit gate — the model runs RED → GREEN → REFACTOR + the four-check audit, period.

## 5. Build counter inheritance

**Decision (per skill `identifier-discipline` + `docs/governance/loop-instruction-template.md §1 item 7`):** SD-27's first concrete build is **`0.6.0`** (the v0.6-alpha release-swarm is the first concrete line; SD-27 lands the next concrete build after SD-26's promotion to develop). Per-criterion tiering's Housekeeping-Haiku on the version-bump step is the same as SD-26 E6.4.

The scheme is `<major>.<tranche-base>.<build>`:

- **major**: 0 (no main-publish yet).
- **tranche-base**: 6 (SD-27 launches on `tranche/7`, but the bundle's first concrete build is on the v0.6 truncation line; SD-27's closure bumps to `0.6.1`).
- **build**: monotonic counter, never resets.

## 6. Publish mode is move-not-copy (operator directive 2026-07-21)

Same as SD-25 / SD-26: the workspace-side copy is deleted on the publish commit. **That move has already happened** — `docs/release/SD-27-future-state-book-content-ingestion/` is the canonical, repo-resident home and the only one that exists. There is no `programs/` tree in this repo; any citation of a workspace-side author path is stale by construction. Promotion was performed via the `release-package-promotion` skill.

## 7. Tier-1 launch-gate dependency

**Decision:** SD-27 cannot dispatch Epic 2.1+ until SD-26's `tranche/5-4 → develop` PR lands. The tier-1 gate is enforced by Criterion 2.1's verification cycle reading SD-26's tier-1 gate. SD-26 ships the Shape B JSON cache schema + the PCGen parity harness; SD-27 consumes those.

**Specifically:** Criterion 2.1 reads `data/corpus/core_rulebook/_schema/` (or whichever surface SD-26 establishes as the canonical schema authority) to verify the per-book cycle's cache matches the established Shape B schema. If the schema authority surface does not exist, the cycle blocks.

## 8. File-touch partition (mandatory, revised 2026-07-25)

**Decision (operator directive 2026-07-25, revised):** SD-27 cycles share the live repo with v0.6's active class/race breadth work. The partition is:

- **May write to (per-book cache cycles 2.1-2.2):** `data/corpus/<book>/` for the 2 in-scope future-state books only (advanced_race_guide, pathfinder_unchained); `docs/governance/wired-integration-stubs-registry.md` (serial); `data/stubs/<book>.json` (serial); `src/bin/sd27_gen_book_cache.rs` (new); `tests/sd27_*` (new); `docs/release/SD-27-future-state-book-content-ingestion/` (the bundle's own docs).
- **May write to (license-stripping pre-flight 2.0.5 + 4 in-scope retro-fit 2.0.6-2.0.9 + all-23 verify 2.0.10):** the 4 in-scope `data/corpus/{core_rulebook,advanced_players_guide,advanced_class_guide,beastiary}/` directories (note: the corpus directory is `beastiary`, singular-no-digit; only the rules-engine tree uses `rules_tables/beastiary1/`) (per-book `LICENSE.json` + per-record `license` field + PI redaction); per-book `data/corpus/<book>/LICENSE.json` for the 2 in-scope future-state books; `src/rules_core/shape_b_v1.rs` (new Shape B schema authority); `tests/sd27_license_stripping_*` (new dual-audit gate); the wired-integration stubs registry (serial, for the `book_stub` license-status update).
- **Must not touch:** `src/rules_core/pilot_compute.rs`; `src/rules_core/rules_tables/<book>/` for any book (license-stripping is shape-b-only, does not modify the rules-engine); `docs/release/v0.6/`; `src/oracle_validation/`.

A 4-grep dual-audit (`identifier-discipline` + `wired-integration`) is the load-bearing enforcement. The cycle fails if it breaches the partition. **The license-stripping cycles additionally run a 5th audit: the PI-blacklist grep** — for each inlined field that matches the PI-blacklist (e.g. `deity_name`, `npc`, `place_name`), the record's `pi_marker` must be `"redacted"` and the value must be `"[redacted PI]"` or absent.

**Worktree isolation.** SD-26 E3 used `isolation: 'worktree'` per book. SD-27 E2.1-2.2 do not need worktree isolation because each cycle writes to a different `data/corpus/<book>/` directory. The 4 in-scope retro-fit cycles (2.0.6-2.0.9) are file-disjoint (each touches only `data/corpus/<book>/` for one in-scope book) and can run in parallel. Cycles serialize on the shared `docs/governance/wired-integration-stubs-registry.md` file (one cycle at a time).

## 9. Tier-1 / Tier-2 partition for the 19 future-state books

**Decision (per operator directive 2026-07-25, refined 2026-07-27):** The 19 future-state books fan out in two tiers. **Beginner Box and Core Essentials are removed from scope per operator directive 2026-07-27** (redundant to other tomes; will not be brought in). Tier-1 / Tier-2 split:

- **Tier-1 (13 books):** advanced_race_guide, pathfinder_unchained, adventurers_guide, bestiary_2, bestiary_3, bestiary_4, bestiary_5, bestiary_6, bonus_bestiary, horror_adventures, monster_codex, mythic_adventures, occult_adventures. Mechanically similar to the 4 in-scope books; templated Shape B pattern. The first two (ARG, PU) are SD-27's; the rest are deferred.
- **Tier-2 (6 books):** ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness. Mechanically dense; denser cache files.

Tier-1 fans out first. Tier-2 launches after Tier-1 reaches the parity-baseline phase. Both tiers run in parallel with v0.6's class work.

## 10. CG-03 baseline inherited, not chased

**Decision:** SD-27's per-book parity baseline assertion is "match rate at the time of cycle close," not "9-of-9 fully oracle-checked." The CG-03 (Human ability-modifier bug at `src/rules_core/pilot_compute.rs:4743-4767`) is v0.6's lane. SD-27 documents the inherited baseline in each cycle's parity-cycle receipt and proceeds.

**Authority:** `forward-scope-register.md` §"Class 0.3."

## 11. Per-cycle tier model: free/discounted model authority

**Decision (operator directive 2026-07-25):** The operator may swap the per-book cycle tier (Sonnet default) for a free or discounted model provided:

- The dispatch instruction pack is tight enough that the model does not need to think beyond templated execution.
- The dual-audit gate is the load-bearing enforcement. The model runs RED → GREEN → REFACTOR + the four-check audit, period.
- The model does not introduce new files outside the per-book directory.
- The model does not modify any file outside the partition (§8).

**Authority:** `forward-scope-register.md` §"Class 1.1" + the operator's 2026-07-25 directive "I would only want to do this if we have a very solid set of instructions that doesn't require that model to think too much."

## 12. SD-27 does not own class/race breadth (operator directive 2026-07-25)

**Decision:** SD-27's scope is content ingestion only. The 8 of 11 CRB classes without chassis support (Barbarian, Bard, Cleric, Druid, Monk, Paladin, Ranger, Sorcerer) are v0.6's lane. The operator confirmed on 2026-07-25 that Bard + Bardic Performances are in progress inside v0.6. SD-27 inherits v0.6's content for the 4 in-scope books; SD-27 does not author class engines for the 19 future-state books.

**Authority:** `forward-scope-register.md` §"Class 2.4" + the operator's 2026-07-25 directive "the full class/race chassis breath is being worked on by v0.6."

## 13. Forward-scope register is the planning entry point

**Decision:** The forward-scope register at `forward-scope-register.md` is the planning entry point for SD-27 (per `spec-domain-bundle-authoring` v1.2.0, §"Forward-scope extraction from predecessor bundle/lane docs"). The register separates predecessor-deferred work into three classes; the scope-draft tightens the register's class-1 commitments into a concrete bundle shape. The register is the disagreement surface; the scope-draft is the committed shape.

**This is non-trivial.** The register's class-2 and class-3 lists are explicitly **deferred** work, not rejected — they are NOT in scope, and SD-27 does not own them. The operator can override this decision on a per-item basis by editing the register before cycles dispatch.

## 14. Identified decisions pending operator authorization

These are decisions the lead has flagged but the operator has not yet signed off on:

- **Branch:** `tranche/7` (convention slug). Operator override recorded in §15.
- **Board:** `codex-tranche-7` (convention slug). Operator override recorded in §15.
- **Bundle label:** `SD-27` vs. `SD-27+ (unscheduled)` (the resolution is Epic 2.0's first cycle, but the operator pulls the lever).

## 15. Operator override slot (intentional gap)

The bundle's branch and board are operator-pinned by convention; the operator overrides them as they see fit. Recorded here for the audit trail.

**Convention slug:** `tranche/7` + `codex-tranche-7`. Operator override on file: `[pending]`. If the operator overrides, the override slug is recorded here with the operator's verbatim directive and the prior bundle's slug being inherited.

## 16. Hard-stop conditions

Per `forward-scope-register.md` §"Pitfalls" + `scope-draft.md` §8. The lead does not pick a side; the lead reports the blocker.

## 17. License-stripping doctrine (OGL/PI), operator-pinned 2026-07-25

**Decision:** SD-27 ships Shape B v1, a license-aware extension of SD-26's Shape B. Every record carries a `license: "OGL" | "PI" | "PI-REDACTED"` field; PI-tagged values are redacted to `"[redacted PI]"` markers (preserves schema, downstream code reads one branch per field). Per-book `LICENSE.json` declares the OGL/PI split and redaction policy.

**Authority surface:** The OGL 1.0a's "Product Identity" section (Paizo's published list) is the source-of-truth for what is inlinable. The PI-blacklist is per-book, not per-record — Paizo's PI list varies by book (e.g. named deities are PI in CRB, OGL in some bestiaries).

**PI-blacklist (initial, per `forward-scope-register.md §1.4`):**

- `deity`, `deity_name` — Product Identity in CRB; varies per book.
- `npc`, `npc_name` — Product Identity in most CRB/APG/ACG/Bestiary 1.
- `monster_name` (non-bestiary) — Product Identity; OGL stat-blocks are inlinable but names often are not.
- `place_name`, `faction_name`, `deity_portfolio` — Product Identity.
- `art_url`, `fiction_text`, `book_cover` — Product Identity (art, fiction, covers).
- `monster_description` (flavor) — Product Identity; mechanical stat-blocks are OGL.

**PI-blacklist (NOT PI, OGL-inlinable):**

- Class features, spell mechanics, equipment stats, feat mechanics — OGL.
- Bestiary stat-blocks (without names) — OGL.
- Numerical tables, formulas, dice expressions — OGL.

**Redaction-to-marker policy (operator-pinned 2026-07-25):** PI-tagged field values become `"[redacted PI]"` rather than being omitted. The record's schema is preserved; downstream code reads one branch per field ("is this a marker? render generic label"). Omission (alternative) was considered and rejected — heterogeneous consumers would force conditional-everywhere code.

**The 4 in-scope books (CRB, APG, ACG, Bestiary 1) are retro-fitted in cycles 2.0.6-2.0.9, not omitted.** Per the operator's 2026-07-25 OGL review, the in-scope books were ingested under SD-22 with inlined OGL content; the PI-stripping retro-fit is real, bounded, and a per-book cycle (one cycle per in-scope book, file-disjoint, parallel-safe).

**The PI-blacklist is a per-cycle pre-flight, not a build-time constant.** Cycle 2.0.5 lands the schema + the initial PI-blacklist. Cycles 2.0.6-2.0.9 apply the blacklist against the in-scope books; discovered-PI-fields that are not in the initial blacklist get added to the blacklist (one source-of-truth file, versioned per cycle). Cycle 2.0.10's dual-audit gate verifies the blacklist is exhaustive — for each book, every Shape B record's field list is intersected with the blacklist, and any field that's neither OGL-inlinable nor blacklist-PI-tagged is a defect.

**5th audit (PI-blacklist grep):** The license-stripping cycles run a 5th dual-audit grep: for every record in `data/corpus/<book>/`, every field value that matches a PI-blacklist pattern must have `license: "PI" | "PI-REDACTED"` and `pi_marker: "redacted"`. A record with a PI-matching value and `license: "OGL"` is a license-defect and the cycle fails.

## 18. Cross-reference

- `./forward-scope-register.md` — planning entry point; load before this decision record.
- `./scope-draft.md` — companion, the committed scope.
- `./technical-design.md` — architectural surface, including the Shape B application + per-book ingestion pipeline.
- `./technical-requirements.md` — pre-loop prerequisites + normative requirements + out-of-scope.
- `./epic-breakdown.md` — per-cycle stories keyed to the epic structure.
- `./loop-instruction.md` — per-cycle procedure.
- `../SD-26-ingest-strategy-and-rule-system-plumbing/` — predecessor, the canonical source for the 19 stubs + the Shape B schema.
- `docs/release/v0.6/` — active sidecar; partition enforcement.
- `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md` — predecessor's canonical Shape B schema definition.
- skill `identifier-discipline` — identifier-disclosure doctrine. (Machine-local, not repo-resident; there is no `docs/governance/identifier-discipline.md`. The runnable gate is vendored at `scripts/identifier-discipline-audit.sh`.)
- `docs/governance/no-stub-mvp-doctrine.md` — wired-integration doctrine.
- `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/loop-instruction.md` — predecessor's loop-instruction template.
- `skill:spec-domain-bundle-authoring` — bundle-authoring discipline.
- `skill:workflow-orchestrated-dispatch` — dispatch shape.

## 19. Dispatch is session-driven `Workflow`-tool orchestration, not a headless script (adopted from SD-26 `decisions.md §13` at pre-launch review 2026-07-27)

**Decision:** SD-27 dispatches via the **in-harness `Workflow` tool, driven from a live session** — not via `scripts/workflow-dispatch.sh` running unattended. That script's `claude code --profile … --task …` invocation does not exist in the live CLI (`claude --help` shows no `code` subcommand and no `--profile`/`--task` flags), and **the script is not present in this repository on any ref** (verified across all heads and remotes, 2026-07-27). As shipped, its dispatch step would fail silently and its `main_loop` would spin on a no-op branch forever.

The deterministic half of dispatch — the manifest seed, the claim/complete state machine, the `depends_on` ordering, and every write to the operator's reporting JSON — lives in **`scripts/sd27-workflow.py`**, which is repo-local and runnable. The `Workflow` tool drives cycle *bodies*; `sd27-workflow.py` owns cycle *state*. Neither improvises the other's job.

**Reasoning:** SD-27's §3 above was written 2026-07-21, one day *before* SD-26's 2026-07-22 pre-launch review caught this same gap and recorded it as its `decisions.md §13` (itself carrying forward SD-25's already-verified `decisions.md §10`). SD-27's `README.md`/`scope-draft.md` OPERATING-METHOD callouts consequently reproduced the pre-correction framing while describing themselves as a "mirror of SD-26's callout" — they mirrored the version SD-26 had already superseded. Rather than re-derive the finding, this ADR adopts it directly: the CLI gap is a live-tooling fact independent of which bundle is running.

**Consequence:** Both OPERATING-METHOD callouts now name the `Workflow` tool and `scripts/sd27-workflow.py`. `scripts/workflow-dispatch.sh` is **not** vendored into this repo — SD-26 kept it only as a concurrency/tiering reference spec, and SD-27's equivalent lives in §4's tiering map plus the workflow script's own `depends_on` chains. Section §3 stands as the original operator directive; where §3 and §19 disagree on mechanism, **§19 governs**.

## 20. The reporting JSON is a first-class cycle output (operator directive 2026-07-27)

**Decision:** Every SD-27 cycle reports into the operator's dashboard at `PF1E_JSON_PATH` (default `/home/todd/hermes-home/swarm-observer/PF1e-dashboard.json`), manifest `sd27_book_pre_build`. Reporting is not optional bookkeeping appended after the fact — a cycle that ran but did not report is treated as incomplete. The full contract is `loop-instruction.md §8`.

**Reasoning:** The dashboard already carried an `sd27_book_pre_build` manifest with `managed_by: "orchestrator"` and `items: []`, and the sanctioned writer's own docstring states its public surface is what *"the loop-instruction imports"* — but this bundle documented none of it. The gap mattered because the writer fails **silently** in the unreachable case: `read_json()` returns `None` and `list_pending_items()` converts that to `[]`, so a misconfigured run reports "nothing to do" and looks successful while writing nothing.

**Consequence:** `scripts/sd27-workflow.py` hard-fails on an unreachable or corrupt dashboard rather than proceeding, and `preflight` is a launch gate. All mutation goes through the orchestrator helper's API — never a direct file write, which the producer would discard on its next tick anyway.

## 21. Repo tooling supersedes parts of this bundle's hand-composed process (2026-07-30)

> **Merge note (2026-07-30 cross-copy merge):** this section and §22 were merged in from the planning-tree copy at `programs/codex/requirements/SD-27-future-state-book-content-ingestion/`, which had diverged from this file's §1-§20 in ways that could not be safely auto-merged (see the merge report). Numbered §18/§19 there, renumbered here to avoid colliding with this file's existing §18-§20.

**Decision (process only — no scope change, no epic added, no criterion changed).**
Four tools landed on `tranche/6` after this package was authored. Where a tool
now enforces something this package described in prose, the package points at
the tool.

| Superseded | By | Where recorded |
|---|---|---|
| `cargo test --workspace --locked` as the bundle-level check | `./scripts/verify.sh` | `acceptance-and-verification.md §0`, `loop-instruction.md §8.1`, `technical-requirements.md §2.2a`, `scope-draft.md §7` |
| Hand-maintained per-book corpus figures | `v06_work_inventory` → `docs/work-inventory.json` | `content-unit-inventory.md §0` |
| Per-book corpus-trap rediscovery | `v06_corpus_trap_report` (pre-ingest) and its `--audit` mode | `loop-instruction.md §8.2`, `§3.3.1` Operations 0/0b |
| "did the ingest reach anyone" left implicit | `reach_gate.rs` | `loop-instruction.md §8.4` — **carries an unresolved scope question, see §22** |

**Why `--workspace` was never sufficient**, since this package named it six
times: the repo root has no `[workspace]` table, so `--workspace` from the root
never reaches `apps/desktop/src-tauri`. That crate is separate and bin-only, and
it shipped un-compilable twice under exactly that command. Three further
structural false-greens are in `scripts/verify.sh --help`.

**Authority:** `scripts/verify.sh`, `scripts/verify-baselines.env`,
`src/pcgen_import/corpus_traps.rs`, `src/bin/v06_corpus_trap_report.rs`,
`src/bin/v06_work_inventory.rs`, `apps/desktop/src-tauri/src/reach_gate.rs`,
`docs/governance/book-ingestion-playbook.md`.

## 22. Conflicts flagged for operator resolution (2026-07-30)

These are **recorded decisions that the new tooling puts under tension**.
Recorded decisions have precedence; none is overridden here.

**22.1 — Content-only scope vs. the reach gate.** §12 and `README.md §1` record
SD-27 as content ingestion with no engine work, and `technical-design.md:156`
records "No new engines". Nothing in the package's process files mentions a
player surface, IPC or the desktop app. But `loop-instruction.md:243` and
`epic-breakdown.md:53` both have the per-book cycle **generate
`src/rules_core/rules_tables/<book>/`** — the exact tree `reach_gate.rs` scans
for `pub const <NAME>: &[<RecordType>]` slices. Either the generated module is
the "thin layer over LST reader" `technical-design.md §2.3` describes, with
`sd27_gen_book_cache` as its only consumer (in which case an `OPEN_FINDINGS`
entry with a named remedy is the honest outcome), or it declares real record
slices (in which case a reach claim belongs in the same cycle). **Operator picks.**

**22.2 — §8's partition says "must not touch `src/rules_core/rules_tables/<book>/`
for any book", but the per-book cycles are required to write it.** §8's
"Must not touch" list reads `src/rules_core/rules_tables/<book>/` for **any**
book, with the parenthetical "license-stripping is shape-b-only, does not modify
the rules-engine" — so the intent is plainly to scope that clause to the
license-stripping cycles (2.0.5-2.0.10). Meanwhile `loop-instruction.md:243`
makes generating that tree step 3 of E2.1, and the partition audit command at
`loop-instruction.md:399` **explicitly allows** `^src/rules_core/rules_tables/<book>/`.
The three surfaces are readable as consistent only by inferring the scope of
§8's clause. **A cycle that reads §8 literally will refuse work the loop
instruction requires.** Recommend the operator tighten §8's wording to
"...for the 4 in-scope books" or "...in the license-stripping cycles";
nothing is changed here.

**22.3 — `core_essentials` is a shared library, not an unused tome.** The
2026-07-27 directive removed Beginner Box and Core Essentials as "redundant to
other tomes". The generator confirms Beginner Box (`out_of_scope`, included by
nothing) but classifies Core Essentials as `shared_library`, included by nine
books — among them the Core Rulebook, Bestiary 1, and this bundle's own in-scope
Advanced Race Guide. The directive is not contradicted (its content does arrive,
through the tomes that include it), but the two books were removed under one
rationale that fits only one of them. Detail at `content-unit-inventory.md §5`
Flag 1. **Operator decides whether the directive's wording needs to distinguish
them.**

**22.4 — "23 books" vs "25 books" is used inconsistently.** Criterion 2.0.10's
heading (`loop-instruction.md:174`) says "All-25-books" while its body (lines
182-210) and `acceptance-and-verification.md §2.2.10` say 23. Both totals are
derivable — 25 corpus directories, of which 23 carry a bundle of record — but the
criterion means one of them. Detail at `content-unit-inventory.md §5` Flag 2.

**22.5 — ACG's measured ceiling is stated two ways inside this package.**
`content-unit-inventory.md §1.4` says ACG was "not touched by SD-25's pass —
verify real ceiling independently"; `technical-design.md:41` states a measured
`ACG equipment 98.1%`. One is wrong. Whoever resolves it should cite the SD-26
receipt the figure came from, or delete it.

**22.6 — RESOLVED 2026-07-31: SD-27's two in-scope books are Advanced Race Guide + Pathfinder Unchained.** Operator ruling, verbatim: *"it's the pu. we moved adventurers guide out."* Adventurer's Guide is **not** SD-27's; it stays routed out of this bundle, as §1/§4/§8/§9 and every other file in this directory already state.

**This repo copy is authoritative on this point. The planning-tree copy is stale and must not be treated as current.** For the record of what the disagreement was: the planning-tree copy at `~/workspace/programs/codex/requirements/SD-27-future-state-book-content-ingestion/` names Advanced Race Guide + Adventurer's Guide as the pairing, with Pathfinder Unchained deferred to SD-28+ — the opposite of this file — and propagates that through its own `scope-draft.md`, `content-unit-inventory.md`, and `acceptance-and-verification.md`. The 2026-07-30 merge that added §21/§22 here deliberately left this file's ARG+PU pairing untouched rather than guess, and raised the conflict instead; that was the right call, and the ruling above confirms this side was correct.

No corrective work follows from this ruling. Every SD-27 doc in this directory already reads ARG+PU (verified by grep 2026-07-31: `release-notes.md`, `progress.md`, `loop-instruction.md`, `scope-draft.md`, `content-unit-inventory.md` and this file all route Adventurer's Guide out), and every build artifact that exists — `data/corpus/pathfinder_unchained/` (59 records), `src/rules_core/rules_tables/pathfinder_unchained/`, `RuleSetId::Pu`, PU's PCGen parity test, and PU's rows in the live feat (17) and equipment (42) catalogs — is PU's. Adventurer's Guide has no corpus directory, no `rules_tables` module, and no records anywhere in this repo, so nothing needs unwinding.

## 23. Race ingestion scope (operator ruling, 2026-07-31)

**23.1 — ARG's races are ingested via the shared `core_essentials` race library, not from ARG's own directory.**

Verified against the real corpus before the ruling was sought: `advanced_race_guide/arg_races.lst`
carries **39 real (non-comment) lines, 37 of them `.MOD`** — it *modifies* race records rather than
declaring them. The actual race chassis lives in
`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/`, which holds
**51 race directories**. `docs/work-inventory.json` independently classifies `core_essentials` as
`scope: shared_library` with an `included_by` naming nine books, Core Rulebook and Bestiary 1 among
them. So "ingest ARG's races" is not a thing that can be done inside ARG's own directory.

**Operator ruling: ingest the shared library once, then apply ARG's `.MOD` layer on top.** This
matches how the corpus is actually structured rather than fighting it, and the one-time cost serves
every including book instead of only ARG.

**Known, accepted consequence:** Core Rulebook's 7 races are presently hardcoded as a 7-variant
`RaceId` enum in `src/rules_core/rules_tables/crb/race_tables.rs`, consumed by exactly one call site
(`pilot_compute.rs`'s flat `"race:human" => RaceId::Human` match). Ingesting the shared library
replaces that with corpus-driven data — a real change to already-shipped CRB behavior. It is to be
done under test, with CRB's existing 7 races' resolved traits pinned before and after so any drift
is a caught failure rather than a silent regression.

This ruling resolves the "race roster" half of
`docs/release/v0.6/book-agnostic-backend-gaps-scoping.md` Finding 2.

## 24. Formula-token content is hand-modelled per feature (operator ruling, 2026-07-31)

**24.1 — No formula interpreter. Each feature is a hand-written, corpus-verified pure function.**

PU's 4 Unchained classes (`pu_abilities_class.lst`, 1,344 real lines, 424 of them `.MOD`, 62 distinct
named features) and ARG's Alternate Racial Traits (`arg_abilities_race.lst`, 2,214 lines) are both
PCGen `BONUS:`/`DEFINE:`/`PREREQ:` formula-token content — the content-kind no book in this repo has
ever ingested. Across the 17 still-deferred books the same shape totals roughly 17,000 lines.

**Operator ruling: follow the pattern v0.6 already proved at scale**, not a generic interpreter.
Every one of the 27 existing classes was built this way — `warpriest_fervor_uses_per_day`,
`slayer_sneak_attack_dice`, `monk_scorpion_style_dc` and their siblings are small pure functions
whose formulas were verified byte-exact against the corpus and pinned by tests. PU's 62 features get
the same treatment.

**Why, explicitly:** an interpreter is the highest-risk option for *silently* wrong answers, and this
codebase's own history is a list of wrong numbers that survived because nothing failed loudly. A
hand-modelled formula that is wrong is a failing test; a misinterpreted token is a plausible number
nobody checks. The cost — linear growth with content — is accepted deliberately in exchange for that
property. Prior cycles' repeated decision to stop short of an interpreter is upheld, not overturned.

**24.2 — This supersedes the bundle's "no new engine work" framing.** `README.md §1` and
`technical-design.md:156` describe SD-27 as content ingestion with "no new engine work, new class
chassis, or new rule mechanics." The operator's 2026-07-30 directive redefined SD-27's definition of
done as full player reachability for both books — races, classes, equipment, spells and feats — which
necessarily includes class-feature grounding and an ART choice mechanic. **The operator's directive
governs; the content-only framing is superseded on this point.** This also resolves the contradiction
recorded at `artifacts/cross-bundle-findings-2026-07-30.md` item 1.2.

## 25. §23 CORRECTED — core_essentials stays out of scope; races attribute to their true source book

**25.1 — §23 was wrong, and is superseded by this section.** §23 directed ingesting the
`core_essentials` race library "as a shared library in its own right." That directly contradicts §1,
which records Core Essentials as **removed from project scope on 2026-07-27** as redundant to other
tomes. The operator caught the contradiction. §23's *conclusion* is withdrawn; its verified corpus
facts (arg_races.lst is 39 real lines / 37 `.MOD`; the chassis is not in ARG's own directory) stand
and are the basis for what follows.

**25.2 — ARG declares zero races of its own. All 37 are reprints.** `advanced_race_guide.pcc`
enumerates every race it pulls in, and PCGen's own section comments state each one's provenance. The
counts sum exactly to arg_races.lst's 37 `.MOD` lines, so this is a complete accounting, not a sample:

| PCC section | Races | True source book | Ingested here? |
|---|---:|---|---|
| `# Core Races` | 7 | Core Rulebook | **yes** |
| `# B1 races` | 11 | Bestiary 1 | **yes** |
| `# B2 races` | 7 | Bestiary 2 | no — SD-28 |
| `# B3 races` | 5 | Bestiary 3 | no — SD-28 |
| `# B4 races` | 5 | Bestiary 4 | no — SD-28 |
| `#ISWG races` | 2 | Inner Sea World Guide | no — unscheduled |

`core_essentials/races/` is **where PCGen physically stores shared race files, not a book**. The
operator's characterisation is exactly right: it is a reprint aggregation, and several of the books
it reprints are not ingested yet. Provenance therefore attaches to the true source book named in
PCGen's comments — never to `core_essentials`, which acquires no corpus directory, no `RuleSetId`
variant, and no `data/stubs/` entry.

**25.3 — In-scope for SD-27: the 18 races whose source book is already ingested** (Core Rulebook's 7,
Bestiary 1's 11). Their chassis is read out of `core_essentials/races/<name>/` and filed under
`data/corpus/core_rulebook/race/` and `data/corpus/beastiary/race/` respectively.

**Deferred to SD-28, with a real reason rather than a punt:** the other 19. Ingesting a B2/B3/B4/ISWG
race here would mean creating that book's first content while the book itself is unregistered —
inventing provenance for a tome nobody has audited. They land when their source book lands.

**25.4 — ARG's genuine own contribution is fully in scope and is the point of the book.** Not races:
the `.MOD` layer over all 37, plus `arg_abilities_race.lst` — **1,359 real lines**, of which 595 are
`CATEGORY:Special Ability` and 82 `CATEGORY:Choice`. That is the Alternate Racial Traits corpus, and
it is what makes ARG *the Advanced Race Guide*. SD-27 delivers it for the 18 in-scope races.

**25.5 — Zero race content is currently ingested for any book.** Verified 2026-07-31: no
`data/corpus/*/race/` directory exists anywhere. Core Rulebook's 7 races live *only* as a hardcoded
7-variant `RaceId` enum in `src/rules_core/rules_tables/crb/race_tables.rs` (512 lines), surfaced by
`apps/desktop/src-tauri/src/race_catalog.rs` (93 lines), which imports that CRB table directly. The
§23 commitment to pin CRB's existing 7 races before/after the corpus-driven swap carries forward
unchanged — that is still the guard against silently regressing shipped behaviour.

## 26. The ART swap mechanic is an explicit PCGen protocol, not an invented one

**Finding, verified 2026-07-31.** Task #5 was scoped as "design and build the Alternate Racial Traits
swap/choice engine," on the assumption the mutual-exclusion mechanic would have to be designed. It
does not. PCGen already encodes it declaratively, and the engine's job is faithful transcription.

**The protocol.** Every standard racial trait is gated on a negated fact-check naming its own
replace-flag. From `core_essentials/races/dwarf/dwarf_abilities_race.lst`:

```
Greed  KEY:Dwarf ~ Greed  CATEGORY:Special Ability
       TYPE:RacialTraits.Dwarf Racial Trait.Dwarf Racial Default.SpecialQuality
       !PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True
       BONUS:SITUATION|Appraise=to assess nonmagical metals or gemstones|2|TYPE=Racial
```

Read: *Greed applies unless `Dwarf_ReplaceGreed` is set.* ARG's alternate racial traits are precisely
what set those flags — `arg_abilities_race.lst` contains **625 replace-flag settings spanning 36
races** (Dwarf alone: 9 `ReplaceDefensiveTraining`, 11 `ReplaceHatred`, 10 `ReplaceStonecunning`, 7
`ReplaceGreed`, 6 each `ReplaceHardy`/`ReplaceStability`, 5 `ReplaceVision`, 2 `ReplaceLanguages`).

**Why this matters.** The swap is a data relationship already stated in the corpus, so the engine
models a protocol rather than guessing at one, and every swap is verifiable against the source line
that declares it. It also confirms §24's hand-modelling ruling is the right shape here: each trait is
a small pure function plus a declared replace-flag, and a trait that fails to swap is a failing test
rather than a silently-doubled bonus. Standard traits are additionally self-identifying via
`TYPE:...Dwarf Racial Default...`, so the default set is readable from the corpus, not assumed.

**Corpus-quality note, recorded not fixed:** the race chassis rows in `core_essentials/races/*/
*_races.lst` carry a placeholder `SOURCEPAGE:p.xx` rather than a real page. The trait rows carry real
citations (`SOURCEPAGE:p.21` for Dwarf). Provenance therefore comes off the trait rows; the chassis
row's page is not trustworthy and must not be transcribed as though it were.

## 27. §26 CORRECTED — two published numbers were wrong (2026-07-31)

Both errors are mine, were caught by the ingestion agents, and are re-verified here by command
rather than accepted on the agents' word.

**27.1 — "625 replace-flag settings" conflated *mentions* with *settings*.** The real figures:

| measure | count | command |
|---|---:|---|
| flags actually **set** | **271** | `grep -oE "FACT:[A-Za-z]+_Replace[A-Za-z]+\|True" \| wc -l` |
| flags **mentioned** anywhere | 625 | `grep -oE "[A-Za-z]+_Replace[A-Za-z]+" \| wc -l` |
| `PREFACT` clauses **reading** flags | 257 | `grep -oE "!?PREFACT:[^\t]*_Replace[^\t]*" \| wc -l` |

The setting token is `FACT:<Race>_Replace<Trait>|True`, always trailing. §26's 625 counted every textual
mention — a flag set once and then read by several sibling traits' mutual-exclusion guards was counted
each time. §26's per-Dwarf breakdown is wrong for the same reason; set-vs-mentioned, verified:
`ReplaceHatred` 5 set / 11 mentioned, `ReplaceStonecunning` 5 / 10, `ReplaceDefensiveTraining` 4 / 9,
`ReplaceGreed` 3 / 7, `ReplaceHardy` 2 / 6, `ReplaceStability` 2 / 6, `ReplaceVision` 2 / 5,
`ReplaceLanguages` 1 / 2.

*(An agent reported the mention count as 627 and called 625 wrong. It is not — 625 reproduces exactly
under the pattern §26 used; 627 comes from a looser `[A-Za-z]*` pattern matching two additional
degenerate spans. The arithmetic was never the defect. **The label was.**)*

**§26's protocol description remains correct** — standard traits are gated by
`!PREFACT:1,ABILITIES,<Flag>=True`, alternates set the flag, and the engine models a declared
relationship rather than an invented one. Only the counts were wrong.

**27.2 — "the trait rows carry real citations" is false, and generalised from the worst possible
sample.** Across the 18 in-scope races' 175 standard trait rows: **143 carry the placeholder
`SOURCEPAGE:p.xx`; only 32 carry a real page** — Dwarf `p.21` (12), Half-Orc `p.24`/`p.25` (9),
Aasimar `p.7` (9), Duergar `p.117` (2). Dwarf is one of just **4 races out of 18** with genuine
citations, and §26 inferred the general rule from it.

Transcribing `SOURCEPAGE:` verbatim would therefore have manufactured **143 false citations**.
`src/bin/ingest_races.rs` maps `p.xx` → `null`, so a populated `source_page` always means a real page,
with the raw token still preserved in `raw_tokens`; a test pins this.

**The §25/§26 rule that the chassis row's page is untrustworthy still stands — it was simply not
specific enough.** The honest statement is: *placeholder pages are pervasive across both chassis and
trait rows; a page is trustworthy only when it is not `p.xx`, and that must be checked per row rather
than assumed per content-kind.*

**Process note.** §26 was written from Dwarf alone because Dwarf was the exemplar I had open. Both
defects are the same failure — publishing a general rule from a single unverified sample — and both
were caught only because the ingestion agents were instructed to derive counts by command instead of
trusting the brief. That instruction earned its keep here.

## 28. §8's file-touch partition is spent; `pilot_compute.rs` is in scope (2026-07-31)

**§8 forbids touching `src/rules_core/pilot_compute.rs` and `src/rules_core/rules_tables/<book>/`. Both
prohibitions have outlived their stated reason and no longer bind SD-27.**

**Why §8 said it.** §8 is dated 2026-07-25 and opens with its own rationale verbatim: *"SD-27 cycles share
the live repo with v0.6's active class/race breadth work."* It is a **concurrency partition** — a rule for
keeping simultaneous cycles from colliding — not a judgement that these files are unsafe. The
`rules_tables/<book>/` line carries its own scope note for the same reason: *"license-stripping is
shape-b-only, does not modify the rules-engine"*, i.e. it constrains the license-stripping cycles
specifically.

**Why it no longer applies.** v0.6 closed. Its work merged to `develop` and SD-27's own tranche PR #342
merged as `88a0011e`. There is no concurrent cycle to collide with; this branch is the only writer. A
partition with nothing to partition against is not a safety property, it is a stale constraint.

**It is also already overtaken in practice.** The PU class work landed
`rules_tables/pathfinder_unchained/{barbarian,monk,rogue,summoner}_features.rs` — squarely inside the
`rules_tables/<book>/` line — because §24's hand-modelling ruling *directs* that content there. §24 and
§8 cannot both be obeyed; §24 is later, operator-pinned, and specific to this content.

**Why it must lift now.** The operator's definition of done is player reachability: *"all data is
ingested, compute is available, and can reach the end user through the ui. there is not a single thing
left to be done for that thing to be utilized by a user."* Two open defects cannot be closed without
`pilot_compute.rs`:

1. **Size modifiers to AC / touch AC / CMB / CMD do not exist for any race.** A live Goblin fighter shows
   AC 18 / touch 14 / CMB +3 / CMD 17; PF1's Small values at those stats are **19 / 15 / +2 / 16**. This
   is wrong arithmetic on a player's sheet, and it **pre-dates** the 18-race widening — Gnome and Halfling
   shipped with it.
2. **PU's 4 Unchained classes are grounded but unwired.** 4 class + 64 class_feature records and 69
   passing library tests exist; no player can select one, and no sheet changes because of them.

**Ruling: `pilot_compute.rs` and `rules_tables/<book>/` are in scope for SD-27 from this point.** §8's
partition is recorded as **spent**, not wrong — it was correct for the concurrency it was written for.
The remaining §8 prohibitions (`docs/release/v0.6/`, `src/oracle_validation/`) stand: those are a different
concern entirely, and nothing in the reachability work needs them.

**Standing guard, unchanged:** `pilot_compute.rs` is the engine's most load-bearing file. Every change to
it lands with a test pinning the before/after per affected race or class, so drift is a caught failure
rather than a silent recomputation — the same discipline §25.5 imposed on the CRB race swap.

## 29. Architectural findings from the reachability tranche (2026-08-01)

Four findings that outlived the cycles that produced them. Each is recorded because it is a *shape*
that will recur on the next book, not a bug that was fixed and is done. Every number below was
re-derived by command at closure; none is carried over from a cycle report.

### 29.1 — The two-compute-twins trap, and the seam that closes it

**The trap.** This engine derives the same pillars twice, in two files:

| twin | pillar functions | what it is |
|---|---|---|
| `src/rules_core/pilot_compute.rs` | `compute_combat_baseline`, `compute_selected_skill_modifiers` | hardcoded Chain-Shirt arithmetic; **most of the test suite exercises this one** |
| `src/rules_core/pilot_compute_corpus.rs` | `compute_combat_baseline_from_corpus`, `compute_selected_skill_modifiers_from_corpus` | real corpus-resolved equipment; the pair `pf1_adapter::resolve_unified_pilot_snapshot` gates on, so **the one whose numbers reach a player's sheet** |

Measured before the fix, and recorded verbatim in
`tests/sd27_feat_effects_reach_both_compute_paths.rs`:

```text
grep -o 'feat_effects::[a-z_]*' src/rules_core/pilot_compute.rs        | sort -u | wc -l  -> 34
grep -o 'feat_effects::[a-z_]*' src/rules_core/pilot_compute_corpus.rs | sort -u | wc -l  ->  0
```

The corpus twin consumed **zero** feat effects and hand-inlined Dodge as its only feat awareness.

**Why this is worse than an ordinary bug: it is a false-green generator.** A feat wired into
`pilot_compute.rs` gets a passing test and changes nothing on screen. The work looks done, the gate
agrees, and the player sees the old number. Five feats were live in exactly that state:

| feat | book | cell | hardcoded twin said | the sheet said |
|---|---|---|---|---|
| Athletic | CRB | Climb / Swim | 7 / 7 | 5 / 5 |
| Persuasive | CRB | Intimidate | 5 | 3 |
| Intimidating Prowess | CRB | Intimidate | 6 | 3 |
| Armor of the Pit | ARG | Armor Class | 19 | 17 |
| Sure and Fleet | ARG | Climb | 7 | 5 |

Three of the five are Core Rulebook feats. This pre-dated SD-27 and was found by it.

**The seam.** `pilot_compute::feat_derived_pillar_contributions` is now the *sole* `feat_effects`
reader for every pillar the two twins derive independently, and both twins consume it. Two guards,
deliberately different in kind:

* **structural** — `tests/sd27_feat_effects_reach_both_compute_paths.rs` reads the two source files
  and fails if either twin's pillar functions name `feat_effects::` at all. This catches the *shape*
  that produces divergence, including for a producer no catalog feat reaches yet.
* **behavioural** — `pilot_compute_corpus::every_catalog_feat_moves_both_compute_paths_identically`
  sweeps the live 690-record catalog (CRB + APG + ACG + ARG + PU) and pins all nine shared cells
  equal across the two paths, feat by feat.

**The rule for the next book: a magnitude is not wired until it moves on the twin the player reads.**
A test against `pilot_compute.rs` alone is evidence of nothing.

### 29.2 — There is a third twin, and it is in TypeScript

Closing the Rust seam does not close the class of defect, because `CharacterSheet.tsx` computes
sheet cells of its own. Three were moved into the engine this tranche — `defense.touch_armor_class`,
`combat.combat_maneuver_bonus`, `defense.combat_maneuver_defense` — precisely because a React-local
formula could not see the size modifier. **One remains, and it is measured, not suspected:**

* `CharacterSheet.tsx:2945` computes the headline HIT POINTS panel as
  `maxHitPoints(heldClasses, abilities.constitution)`
  (`apps/desktop/src/characterHub/characterProgression.ts:287`) — a frontend-local formula that
  never reads `feat_effects::hp_bonus_from_feats`.
* The Defense tab's MAX HP comes from `character_hub.rs:3254`, which *does* add it.

So a character holding Toughness reads **two different maximum hit point values on the same sheet**.
Observed live during the removal cycle: headline 13/13 while the Defense tab moved 13 → 16 → 13
across an add and a remove. Pre-existing, out of that cycle's scope, and logged rather than glossed.

**The generalisation, which is the actual finding:** the twin problem is not "two Rust functions."
It is *any* surface that re-derives a rules number instead of rendering an engine explanation. The
engine has a name for the correct shape — an `explanations` row with an id, a value and a detail —
and every cell that does not read one is a candidate twin. `flat_footed_armor_class` was moved into
the engine this tranche for exactly this reason and is now read from
`defense.flat_footed_armor_class`.

### 29.3 — The reach gate has two blind spots, and both are shaped like "the scan cannot see it"

`reach_gate::full_inventory()` unions three independent discovery sources: the shipped ingest
diagnostic, a source scan of `src/rules_core/rules_tables/`, and the `data/corpus/` directory tree.
The source scan is the weak one, twice over.

**Blind spot 1 — function-wrapped tables.** `scanned_inventory()` originally matched column-zero
`pub const NAME: &[Type]` declarations only. Pathfinder Unchained emits its records inside accessor
function bodies instead:

```
src/rules_core/rules_tables/pathfinder_unchained/equipment_tables.rs:70:pub fn equipment_tables() -> &'static [EquipmentTableEntry]
src/rules_core/rules_tables/pathfinder_unchained/feat_tables.rs:107:pub fn feat_tables()      -> &'static [FeatTableEntry]
```

PU was therefore invisible to the source scan — and, at the same moment, absent from the ingest
diagnostic's four-book list. **The gate asserted nothing about the book in either direction while its
17 feats and 42 equipmods were already reaching live catalog commands.** A gate that is silent about a
book is indistinguishable from a gate that has cleared it. Closed by teaching `slice_element_type`
the `pub fn name() -> &'static [Type]` shape *and* by adding the book to the diagnostic, so PU no
longer rests on one source.

**Blind spot 2 — hand-modelled tables emit no slice at all.** `decisions.md §24` directs formula
content to be hand-written pure functions. A pure function is not a record slice, so it is invisible
to the scan **by construction, permanently**. Within one book:

| PU class-feature module | shape | seen by the source scan? |
|---|---|---|
| `barbarian_features.rs` | `pub fn features() -> &'static [UnchainedBarbarianFeature]` | yes |
| `monk_features.rs` | `pub fn features() -> &'static [UnchainedMonkFeature]` | yes |
| `rogue_features.rs` | pure functions; only `pub fn class_skills() -> &'static [&'static str]` | **no** |
| `summoner_features.rs` | pure functions; `class_skills()`, `eidolon_subtypes()` | **no** |

Half of `pathfinder_unchained/class_features` reaches the inventory *only* through
`data/corpus/pathfinder_unchained/class_feature/`. **§24 and the source scan are in permanent
tension: the more faithfully a book follows the hand-modelling ruling, the less of it the scanner can
see.** The corpus directory is the load-bearing discovery source for §24-shaped content, and any
future cycle that changes what gets written to `data/corpus/` must treat that as a change to the
gate's coverage.

**Consequence for the union:** discovery must stay plural. The rule is not "fix the scanner" — it is
that no family may depend on a single source, and a family that appears in only one is a finding.

### 29.4 — `SOURCEPAGE:p.xx` is a placeholder, and provenance must be checked per row

**The rule.** PCGen writes `SOURCEPAGE:p.xx` where a page number is unknown. Transcribing it verbatim
manufactures a citation. Every ingest binary therefore maps `p.xx` → `None` for `source_page`, while
preserving the raw token in `raw_tokens`, so a populated `source_page` always means a real page and
nothing is lost. Implemented in `src/bin/ingest_races.rs` (`PLACEHOLDER_SOURCE_PAGE`, line 78) and
`src/bin/ingest_pu_classes.rs` (line 101), and pinned — `ingest_races.rs:1567`: *"the p.xx placeholder
must never be stored as a citation."*

**Why it needed a ruling rather than a fix.** §26 asserted that trait rows "carry real citations."
§27.2 corrected it: across the 18 in-scope races' 175 standard trait rows, **143 carry `p.xx` and only
32 carry a real page** — Dwarf `p.21` (12), Half-Orc (9), Aasimar (9), Duergar (2). Four races out of
eighteen. Transcribing verbatim would have manufactured **143 false citations**.

**The generalised rule, which is the part worth keeping:** *placeholder pages are pervasive across
both chassis and trait rows; a page is trustworthy only when it is not `p.xx`, and that must be
checked per row rather than assumed per content-kind.* §26's error was not arithmetic — it was
inferring a general rule from Dwarf, the single exemplar that happened to be open. That failure mode
recurred often enough this tranche (85 `correction` events in `docs/retro/events/`) that it is the
tranche's most reproducible finding, and the countermeasure that worked was mechanical: **every brief
instructed agents to derive counts by command rather than trust the brief, and the briefs were wrong
repeatedly.**

## 30. Two path conventions, operator-ruled (2026-08-01)

Both rulings arrived after a full tranche was spent treating their absence as an environmental fact.
They are recorded here in the operator's own words because the paraphrase is what failed: every prior
agent understood "the fixtures aren't on this box" and none understood "the default names another
machine's home directory."

### 30.1 — `$HOME`-relative, never a hardcoded user path

> "If you use ~/workspace you will always be right, no matter which machine you are working from. I'm
> putting workspace in the home directory and keeping it synced with syncthing."

**The convention.** Any default that points at the operator's workspace resolves `$HOME` at runtime.
In shell that is `"${HOME}/workspace/..."`; in Rust it is `std::env::var("HOME")` joined with the
relative remainder — never a `~`-prefixed string literal, because Rust does not expand `~` and such a
literal is a relative directory named `~` that silently does not exist. Environment overrides
(`PCGEN_REPO_DIR`, `PCGEN_CORPUS_ROOT`, `CORPUS_ROOT`, `CODEX_REPO_ROOT`) still win; only the fallback
changed.

**Why it is a convention and not a preference.** `workspace/` is Syncthing-synced across the
operator's machines, so the *same relative path under `$HOME`* is correct on every one of them. An
absolute `/home/<someone>/workspace/...` is correct on exactly one machine and quietly wrong on all
the others — and "quietly" is the whole problem, because the failure it produces reads as a missing
environment rather than as a bad literal.

**Enforced, not just documented:** `tests/no_foreign_home_paths.rs` fails the build if a foreign
absolute home path reappears anywhere under `tests/`, `src/` or `scripts/`, and separately if any Rust
string literal starts a path with `~`. It carries a third test that proves the walk actually reads
files, so a broken scan cannot make the guard pass forever while checking nothing.

### 30.2 — Build artifacts live in the build's own artifact folder

> "This is why we always include artifacts needed for the build in the artifact folder for the build
> instead of referring to an external source."

**The convention.** If a build or a test needs a file, that file is vendored into the build's own
artifact folder and committed. It is not read from a sibling checkout, a home-directory scratch tree,
or any other path outside the repository. The two GE-05 pilot `.pcg` saves now live at
`docs/release/GE-05-oracle-validation-and-parity-harness/artifacts/`, pinned by sha256 in
`tests/ge05_vendored_pcg_fixtures.rs` so a silent substitution fails loudly. The pre-existing
`data/corpus/{advanced_race_guide,pathfinder_unchained}/_parity/*.pcg` fixtures are the precedent this
follows.

**Why a sha pin and not just a copy.** A vendored fixture that nothing verifies is a copy that can
drift. The pin was negative-tested rather than asserted: appending a single byte to each fixture makes
all three guards fire with a naming message, and the digests were re-verified after restoring.

### 30.3 — What the two missing conventions actually cost this bundle

Not a build inconvenience. **SD-27's own per-book parity gates never ran, once, during the entire
tranche.** `tests/sd27_advanced_race_guide_parity.rs` and
`tests/sd27_pathfinder_unchained_parity.rs` are the E3.x cycle's proof that this bundle's two books
produce the same character sheet as a real PCGen engine run. Across eight recorded verification
sweeps, both reported `0 passed; 1 failed` every time. The parity claim for Advanced Race Guide and
for Pathfinder Unchained was therefore **unverified for the whole bundle** — and not because the data
was missing, the corpus was incomplete, or PCGen was unavailable. All three were present and correct.
One `const` in `src/oracle_validation/pcgen_runner.rs` named another machine's home directory, and
`PcgenRunOptions::new` routes both suites through it.

The cost compounds in a specific way worth naming: because the failure looked environmental, each
sweep dutifully re-recorded it as environmental in `scripts/verify-baselines.env` and moved on. The
misdiagnosis was not one agent's mistake — it was carried forward, with citations, eight times. The
countermeasure that worked was the one this bundle already applies to content: **derive it by command,
and treat a brief's framing as a claim to be tested rather than a fact to be inherited.**

### 30.4 — What the two parity suites, now that they run, actually proved

Measured on 2026-08-01 with no PCGen environment variables set, against real PCGen 6.09.08.RC1 engine
invocations (7.1 s and 6.8 s of real JVM work respectively):

| pilot case | dimensions compared | matched | mismatched |
|---|---|---|---|
| `pf1-arg-human-fighter-level1` | 15 | 14 | 1 |
| `pf-pathfinder_unchained-human-fighter-level1` | 15 | 14 | 1 |

**No new parity defect in either book.** The single mismatch is identical in both, and identical to
the one the CRB pilot has carried since SD-26: `combat.baseline_melee_attack_bonus`, PCGen 5 vs.
Codex 6 — the already-diagnosed weapon-agnostic-versus-weapon-specific melee-total discrepancy
(Codex's figure legitimately includes Weapon Focus (Longsword); whether PCGen's compared export field
is a different quantity or the harness maps the wrong field is the open item SD-26 forwarded). It is a
harness/oracle-semantics question, not book content, and it reproduces on the Core Rulebook pilot that
has nothing to do with either of this bundle's books.

**What did get proven is book-specific and real.** Both suites carry a genuine record from their own
book's Shape B cache through the full pipeline, and the two books' encumbrance totals differ by
exactly the amount that record weighs:

- ARG: `encumbrance.total_carried_weight_lbs` = **30** on both sides. The +1 lb over the shared GE-06
  posture is `data/corpus/advanced_race_guide/equipment/arms_armor/dogslicer.json` (`WT:1`), plus the
  ARG feat `defiant_luck.json` carried on both the `.pcg` and the Codex input.
- PU: the same dimension is **29** on both sides — the same posture without the Dogslicer, plus
  Pathfinder Unchained's own Wound Threshold variant of `endurance.json`.

A real ARG equipment record resolved to the same weight in the real PCGen engine and in Codex's
corpus-aware compute path. That is the thing the E3.x cycles were written to demonstrate, and it is
the first time it has actually been observed rather than assumed.

**Conclusion for SD-27: no conclusion changes, and that is now a measured result rather than an
untested assumption.** The distinction matters — before this run, "ARG and PU are at parity" was a
claim resting on a suite that had never executed.
