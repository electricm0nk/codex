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

**Decision (operator-pinned 2026-07-25, refined 2026-07-27):** SD-27 ships the resolution of the 19 future-state book stubs that SD-26 registered in Epic 4. **Beginner Box and Core Essentials were removed from scope per operator directive 2026-07-27** (redundant to other tomes; will not be brought in); the bundle's two in-scope future-state books (Advanced Race Guide, Adventurer's Guide) are the only per-book cycles SD-27 dispatches, with the remaining 17 deferred future-state books landing in SD-28+. The payload is content ingestion for an existing engine — no new chassis, no new rule mechanics, no new class engines.

**Authority surfaces cited:**

- `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/release-notes.md:29-31` — the 19 stubs registered.
- `docs/governance/wired-integration-stubs-registry.md` entries #0003-#0021 — registry entries with `planned_resolution_bundle: "SD-27+ (unscheduled)"`.
- `data/stubs/*.json` (19 files) — stub manifests with `content_kind_counts: null`.
- `docs/release/v0.6/risks-and-open-questions.md:80` — the v0.6 swarm's own decision that the label discrepancy is "out of scope for this swarm — it's an SD-26/SD-27 bundle-labeling question, not a v0.6 alpha content question, and the swarm's `data/stubs/` work doesn't depend on which label is 'correct.' Leave both as-is; whoever launches SD-27 reconciles it then."

## 2. Bundle label resolution gates Epic 2 (operator-pinned 2026-07-25)

**Decision:** The `SD-27` vs. `SD-27+ (unscheduled)` discrepancy is the bundle's first cycle (Criterion 2.0). The lead does not pick a side; the operator picks. Both resolutions are internally consistent; the lead's job is to propagate the operator's choice across all 20 surfaces (19 `data/stubs/*.json` + `decisions.md:102`) before any further cycle dispatches.

**Why this is a per-cycle blocking decision.** Per `forward-scope-register.md` §"Class 0," every cycle that touches a `data/stubs/*.json` file or a registry entry blocks on the resolved label. The bundle's first cycle (2.0) lands the resolution.

**Authority:** `forward-scope-register.md` §"Class 1.2" + `forward-scope-register.md` §"Class 0.1."

## 3. SD-27 inherits the Workflow-orchestrated dispatch (operator directive 2026-07-21)

**Decision:** SD-27's dispatch shape is the `Workflow` orchestrator per `docs/governance/loop-instruction-template.md §2` + skill `workflow-orchestrated-dispatch`. Same shape as SD-25 / SD-26's orchestrator with different per-epic concurrency + tiering.

## 4. Per-epic concurrency + tiering map

| Epic | Parallel? | Subagent tier | Notes |
|------|----------|---------------|-------|
| E1 Identifier Cleanup | no | Sonnet | Single cycle |
| E2.0 Label Resolution | no | Sonnet | Per-cycle blocking decision; gates 2.0.5+ |
| E2.0.5 Shape B v1 license-stripping pre-flight | no | Sonnet | Schema bump + per-book `LICENSE.json` + PI-blacklist + redaction policy. Gates 2.0.6+ |
| E2.0.6-2.0.9 4 in-scope book license retro-fit | yes (file-disjoint) | Sonnet | CRB, APG, ACG, Bestiary 1. Gates 2.0.10 |
| E2.0.10 All-23-books license-conformance verify | no | Sonnet | Dual-audit gate across all 23 books (4 in-scope + 2 pre-built future-state + 17 deferred future-state stubs). Gates 2.1+ |
| E2.1-2.2 Per-book pre-build + verify cycles (2 future-state) | yes (file-disjoint, serial-operator-gated) | Sonnet (or operator-approved free/discounted model) | 2 cycles (ARG + AG), each as a pre-build + verify pair. The pre-build creates `data/corpus/<book>/`; the verify confirms. **The other 19 future-state books are deferred to SD-28+, operator-gated on SD-27 closing cleanly.** |
| E3 PCGen Parity Baseline | yes after 2.0 begins | Sonnet (or operator-approved free/discounted model) | 2 cycles, one per in-scope future-state book (ARG + AG); file-disjoint with E2 |
| E4 Closure Epilogue | no | Haiku (4.3, 4.4); Sonnet (4.1, 4.5); Opus (4.2) | Subagent tiering per criterion |

**Tier-1 model swap authority.** The operator authorized on 2026-07-25 that a free or discounted model may run the per-book cycle bodies (E2.1-2.2, E3.1-3.2) provided the dispatch instructions are tight enough that the model does not need to think beyond templated execution. The architectural pattern is templated; the per-book content is mechanically extractable from the LST corpus. The boundary is the dual-audit gate — the model runs RED → GREEN → REFACTOR + the four-check audit, period.

## 5. Build counter inheritance

**Decision (per `docs/governance/identifier-discipline.md` + `docs/governance/loop-instruction-template.md §1 item 7`):** SD-27's first concrete build is **`0.6.0`** (the v0.6-alpha release-swarm is the first concrete line; SD-27 lands the next concrete build after SD-26's promotion to develop). Per-criterion tiering's Housekeeping-Haiku on the version-bump step is the same as SD-26 E6.4.

The scheme is `<major>.<tranche-base>.<build>`:

- **major**: 0 (no main-publish yet).
- **tranche-base**: 6 (SD-27 launches on `tranche/7`, but the bundle's first concrete build is on the v0.6 truncation line; SD-27's closure bumps to `0.6.1`).
- **build**: monotonic counter, never resets.

## 6. Publish mode is move-not-copy (operator directive 2026-07-21)

Same as SD-25 / SD-26: workspace-side copy deleted on the publish commit. SD-27's documents live at `programs/codex/requirements/SD-27-future-state-book-content-ingestion/` until the closure PR is opened, then promoted to `docs/release/SD-27-future-state-book-content-ingestion/` via the `release-package-promotion` skill.

## 7. Tier-1 launch-gate dependency

**Decision:** SD-27 cannot dispatch Epic 2.1+ until SD-26's `tranche/5-4 → develop` PR lands. The tier-1 gate is enforced by Criterion 2.1's verification cycle reading SD-26's tier-1 gate. SD-26 ships the Shape B JSON cache schema + the PCGen parity harness; SD-27 consumes those.

**Specifically:** Criterion 2.1 reads `data/corpus/core_rulebook/_schema/` (or whichever surface SD-26 establishes as the canonical schema authority) to verify the per-book cycle's cache matches the established Shape B schema. If the schema authority surface does not exist, the cycle blocks.

## 8. File-touch partition (mandatory, revised 2026-07-25)

**Decision (operator directive 2026-07-25, revised):** SD-27 cycles share the live repo with v0.6's active class/race breadth work. The partition is:

- **May write to (per-book cache cycles 2.1-2.2):** `data/corpus/<book>/` for the 2 in-scope future-state books only (advanced_race_guide, adventurers_guide); `docs/governance/wired-integration-stubs-registry.md` (serial); `data/stubs/<book>.json` (serial); `src/bin/sd27_gen_book_cache.rs` (new); `tests/sd27_*` (new); `docs/release/SD-27-future-state-book-content-ingestion/` (the bundle's own docs).
- **May write to (license-stripping pre-flight 2.0.5 + 4 in-scope retro-fit 2.0.6-2.0.9 + all-23 verify 2.0.10):** the 4 in-scope `data/corpus/{core_rulebook,advanced_players_guide,advanced_class_guide,beastiary1}/` directories (per-book `LICENSE.json` + per-record `license` field + PI redaction); per-book `data/corpus/<book>/LICENSE.json` for the 2 in-scope future-state books; `src/rules_core/shape_b_v1.rs` (new Shape B schema authority); `tests/sd27_license_stripping_*` (new dual-audit gate); the wired-integration stubs registry (serial, for the `book_stub` license-status update).
- **Must not touch:** `src/rules_core/pilot_compute.rs`; `src/rules_core/rules_tables/<book>/` for any book (license-stripping is shape-b-only, does not modify the rules-engine); `docs/release/v0.6/`; `src/oracle_validation/`.

A 4-grep dual-audit (`identifier-discipline` + `wired-integration`) is the load-bearing enforcement. The cycle fails if it breaches the partition. **The license-stripping cycles additionally run a 5th audit: the PI-blacklist grep** — for each inlined field that matches the PI-blacklist (e.g. `deity_name`, `npc`, `place_name`), the record's `pi_marker` must be `"redacted"` and the value must be `"[redacted PI]"` or absent.

**Worktree isolation.** SD-26 E3 used `isolation: 'worktree'` per book. SD-27 E2.1-2.2 do not need worktree isolation because each cycle writes to a different `data/corpus/<book>/` directory. The 4 in-scope retro-fit cycles (2.0.6-2.0.9) are file-disjoint (each touches only `data/corpus/<book>/` for one in-scope book) and can run in parallel. Cycles serialize on the shared `docs/governance/wired-integration-stubs-registry.md` file (one cycle at a time).

## 9. Tier-1 / Tier-2 partition for the 19 future-state books

**Decision (per operator directive 2026-07-25, refined 2026-07-27):** The 19 future-state books fan out in two tiers. **Beginner Box and Core Essentials are removed from scope per operator directive 2026-07-27** (redundant to other tomes; will not be brought in). Tier-1 / Tier-2 split:

- **Tier-1 (13 books):** advanced_race_guide, adventurers_guide, bestiary_2, bestiary_3, bestiary_4, bestiary_5, bestiary_6, bonus_bestiary, horror_adventures, monster_codex, mythic_adventures, occult_adventures, pathfinder_unchained. Mechanically similar to the 4 in-scope books; templated Shape B pattern.
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
- `docs/governance/identifier-discipline.md` — identifier-disclosure doctrine.
- `docs/governance/no-stub-mvp-doctrine.md` — wired-integration doctrine.
- `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/loop-instruction.md` — predecessor's loop-instruction template.
- `skill:spec-domain-bundle-authoring` — bundle-authoring discipline.
- `skill:workflow-orchestrated-dispatch` — dispatch shape.
