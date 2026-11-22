---
title: SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit (Tranche-5 Bundle)
stc_id: STC-CODEX-SD-22
canonical: true
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical_path: programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: tranche/5 (operator-pinned 2026-07-18; replaces the TBD placeholder; not inherited from SD-21's tranche/4-1)
  board: codex-tranche-5 (operator-pinned 2026-07-18; re-uses the dead-state board from the prior 2026-07-16 SD-21 launch; the loop's Step 10 mint uses `--board codex-tranche-5` explicitly)
  write_scope: documentary-only updates inside this source STC bundle plus control-plane sync in `programs/codex/requirements/README.md`; no repo implementation-code, GitHub branch-protection, release, or update-index write authority until a derived stage-specific execution handoff grants exact paths and verification
review_state: draft
last_reviewed_at: 2026-07-18
supersedes: (none — first issuance; bundles the prior SD-22 scope from decisions §9 of SD-21 + scope expansion 2026-07-17 to APG + ACG + Bestiary 1 + DM toolkit, with the operator's 2026-07-18 clarification that "ACG, APG are the two advanced guides")
upstream_targets:
  - ~/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md §9 (operator directive 2026-07-15: "With SD-22, we should include at least beastiary 1 rules"; scope expansion 2026-07-17 #1: "those need to move to SD-22" + #2: "APG and ACG have moved to SD-22"; operator clarification 2026-07-18: "ACG, APG are the two advanced guides"; branch pin 2026-07-18: "tranche/5")
  - ~/workspace/governance/spec-domain-lifecycle.md (sibling doctrine; governs post-tranche-3 SD-22 launches)
  - ~/workspace/governance/identifier-discipline.md (sibling doctrine; governs identifier-cleanup governance for SD-22's per-book epic cycles)
date: 2026-07-15
---

# SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit (Tranche-5 Bundle)

This is the planning-only source STC for SD-22. It is not a direct implementation prompt and must not be handed wholesale to a coding harness. The next truthful move is to derive bounded execution handoffs from `epic-breakdown.md`, beginning with the smallest prerequisite slice that establishes content-source ingest without faking full content.

## What this bundle is

SD-22 is the bundle for **content-source ingest for APG + ACG + Bestiary 1** plus the **DM toolkit** (encounter builder, party-CR math, diagnostic surfaces). Per the operator directives:

- **2026-07-15**: original scope was DM toolkit + encounter builder + Bestiary 1 ingestion.
- **2026-07-17 #1**: "those need to move to SD-22" (referring to the two advanced guides).
- **2026-07-17 #2**: "APG and ACG have moved to SD-22" (explicit re-affirmation of the advanced-guide migration).
- **2026-07-18 clarification**: "ACG, APG are the two advanced guides" — the two books referenced are the Advanced Player's Guide (APG) and the Advanced Class Guide (ACG), not the "Ultimate Combat / Ultimate Magic" pair. (A prior-turn default-and-flag introducing Ultimate Combat + Ultimate Magic was operator-corrected; the Ultimate books are NOT in SD-22's scope.)

**Net scope-of-record:** content-source ingest for APG + ACG + Bestiary 1 + DM toolkit. SD-22 owns **three** book lines, not four. Future operator-pinned additions (Ultimate-line books, Advanced Race Guide, etc.) are addendum decisions to `decisions.md §1`, not re-authorings.

## Why this bundle

The full content-source ingest lane is a coherent surface on its own; SD-22's bundle can grow that as its primary surface. SD-22 will also need its own tranche + kanban board (operator-pinned 2026-07-18 to `tranche/5` / `codex-tranche-5`). The bundle is now substantial: content-source ingest for APG + ACG + Bestiary 1 (three books, each with per-class or per-monster-block work units), DM toolkit (encounter builder + party-CR math surfaces), governance epics (Identifier Cleanup + Closure Epilogue + Build Version Numbering), and the operator-pre-launch epic.

## Out of scope (recorded explicitly)

- *Per-character rules-engine work.* That's SD-20 (Tranche-4) and SD-21 (Tranche-4-1). SD-22 doesn't compete with that lane; SD-22 *feeds* it by populating content.
- *Campaign Manager GUI screens.* Outside the bundle per SD-21's `decisions.md` §6.
- *DM-toolkit GUI screens.* Outside the bundle; if a separate GUI-bundle is needed for the encounter-builder or party-CR dashboard, that's a future `SD-23` or similar.
- *Ultimate Combat / Ultimate Magic / Ultimate-line books.* NOT in scope (per the 2026-07-18 operator clarification that "the two advanced guides" = APG + ACG, not Ultimate books).
- *Advanced Race Guide / non-(APG|ACG|Bestiary 1) books.* NOT in scope; future operator-pinned only.
- *Tranche lane.* Pinned at `tranche/5` per operator directive 2026-07-18 (replacing the prior TBD placeholder).

## Authority and scope

- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/README.md`
- parent scopes: `programs/codex`
- source artifacts: `~/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md` §9 (SD-22 scope record from prior sessions); `decisions.md` §1 (this bundle's operator-pinned scope doctrine).
- related artifacts: `~/workspace/governance/spec-domain-lifecycle.md` (governance), `~/workspace/governance/identifier-discipline.md` (identifier doctrine), `~/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/` (Tranche-3 corpus-source ingest pattern), `~/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/` (per-character rules-engine), `~/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/` (Tranche-4-1 sibling).

## Document map

- `decisions.md` — decision record (3 decisions: §1 scope, §2 tranche-lane + board, §3 deferred shape decisions). 21+ items at full bundle maturity.
- `scope-draft.md` — canonical handoff; carries the prominent-early `/loop /batch /goal` OPERATING METHOD callout mirroring SD-21's new pattern.
- `epic-breakdown.md` — Spec Domain/Epic decomposition; 8 epics + 2 promotion gates; 30+ criteria at full bundle maturity.
- `acceptance-and-verification.md` — closure gates (gates 1-N).
- `risks-and-open-questions.md` — risk register.
- `technical-design.md` — architecture response (encounter builder, party-CR math, content-source ingest patterns).
- `technical-requirements.md` — pre-loop prerequisites.
- `artifacts/` — evidence-ledger directory; see `artifacts/README.md` for the per-receipt contract.
- `artifacts/corpus/` — on-disk source-shape stubs for Epic 3/4/5/6 ingest cycles (APG/ACG/Bestiary 1 spell/equipment tables); see `artifacts/corpus/README.md` for the schema-of-record and the operator-supplied swap procedure.
- `ingest.md` — canonical process doctrine for content-source ingest (per operator directive 2026-07-19); the per-cycle RED → GREEN → cycle-artifact → commit pipeline. Every Epic 3/4/5/6 cycle reads this file before the GREEN phase.
- `corpus-source-inventory.md` (added 2026-07-19) — load-bearing content inventory binding each content unit to its Rust-module path / test-fixture path / cycle-artifact path / `RuleSetId` four-tuple; the per-criterion artifact map's source-of-truth.

## Closure state

Generated 2026-07-15; scope expanded 2026-07-17 to APG + ACG + advanced guides; clarified 2026-07-18 that "the two advanced guides" = APG + ACG; lane pinned 2026-07-18 to `tranche/5` / `codex-tranche-5`. Bundle is planning-only source STC; not a direct implementation prompt. The first derived execution handoff from this source STC is the place where the operator pre-launch checklist, pre-cycle validation, and operator-recorded Epic 1+2 sub-stories land.

The next truthful move is to derive bounded execution handoffs from `epic-breakdown.md`, beginning with the smallest prerequisite slice that establishes content-source ingest for one book (e.g. APG 1.0 — Alchemist) without faking full content for the rest.

## Expected output artifacts

SD-22's `epic-breakdown.md` is the canonical artifact surface for expected output paths. Concretely:

- `repos/codex/src/rules_core/rules_tables/apg/` — APG content (Advanced Player's Guide classes: Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch — corrected 2026-07-19; Gunslinger/Magus are Ultimate Combat/Ultimate Magic, not APG — plus spells, equipment, races). One Rust file per class table per cycle; the directory mirrors SD-19's `rules_tables/crb/` pattern (sibling directories per SD-19 §9).
- `repos/codex/src/rules_core/rules_tables/acg/` — ACG content (Advanced Class Guide classes like Alchemist, Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Swashbuckler, Warpriest, plus spells, equipment, archetypes). Same shape as APG.
- `repos/codex/src/rules_core/rules_tables/beastiary1/` — Bestiary 1 monster data (stat blocks by CR-by-environment-by-name structure, derived from the Paizo publisher's Bestiary 1). One Rust module per monster-block subset; resolves via `RuleSetId::Bestiary1`.
- `repos/codex/src/rules_core/encounters.rs` — DM-toolkit encounter-math + party-CR computation; consumed by SD-21's Epic 2 (Campaign Manager + Drive) campaign-shape boundary contract.
- `repos/codex/src/rules_core/party_cr.rs` — DM-toolkit party-CR-computation, separate from encounter-math for testability.
- `docs/SD-22/dm-toolkit-architecture.md` — DM-toolkit architecture (per `governance/` patterns).
- `docs/SD-22/apg-ingest-plan.md` — operator-named (epic-cycle ordering and per-class schedule).
- `docs/SD-22/acg-ingest-plan.md` — operator-named (epic-cycle ordering and per-class schedule).
- `docs/SD-22/beastiary1-ingest-plan.md` — operator-named (epic-cycle ordering and per-monster-block schedule).
- `docs/release/SD-22/release-closure-checklist.md` — the per-position bump-process checklist (mirroring SD-21's Epic 5 criterion 27 with the `<major>.<tranche-base>.<build>` triple). This planning-stage entry originally said `docs/SD-22/` (a stray one-level-too-shallow path, unlike SD-20's deliberately-placed `boundary-contract.md`); corrected 2026-07-20 as part of a repo-wide docs-structure cleanup, along with SD-21's matching file.

**Not in expected artifacts:** `repos/codex/src/rules_core/rules_tables/ultimate_combat/` and `repos/codex/src/rules_core/rules_tables/ultimate_magic/` directories. Per operator's 2026-07-18 clarification, Ultimate-line books are NOT in SD-22's scope. Prior-turn `decisions.md §1` references to "Ultimate Combat + Ultimate Magic" as the two advanced guides were operator-corrected and are superseded.
