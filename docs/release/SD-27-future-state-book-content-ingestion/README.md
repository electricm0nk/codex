# SD-27 — Future-State Book Content Ingestion

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️**
>
> `Workflow` orchestrator at `scripts/workflow-dispatch.sh`. **NOT** `/loop /batch`. Per `docs/governance/loop-instruction-template.md §2` + skill `workflow-orchestrated-dispatch`. Mirror of SD-26's operating-method callout.
>
> **Read first:** `forward-scope-register.md`. The scope-draft tightens the register's class-1 commitments into a concrete bundle shape. The register is the disagreement surface; the scope-draft is the committed shape.

## 0. Bundle at a glance

- **Slug:** `SD-27-future-state-book-content-ingestion`
- **Predecessor:** SD-26 (Ingest Strategy Revision + Rule-System Plumbing), closure-ready, PR #338 awaiting operator merge.
- **Sidecar:** v0.6-alpha release-swarm (active on `tranche/6`, not yet closed).
- **Branch:** `tranche/7` (operator directive forthcoming; SD-26's promotion to develop completes the merge-base for SD-27).
- **Board:** `codex-tranche-7` (governed convention slug; operator override on file).
- **Epics:** 4 / **Criteria:** 4 declarative + 9 dynamic (13 total; 2 per-book stub-resolution cycles + 2 per-book cache-build cycles + 2 per-book parity-baseline cycles + a single bundle-level label-resolution cycle + the canonical governance + closure epics; the remaining 17 future-state books' cycles are deferred to SD-28+).
- **First concrete build:** `0.6.0` (per `docs/governance/identifier-discipline.md` — v0.6-alpha is the post-SD-26 line; SD-27 lands the next concrete build).
- **Dispatch:** `Workflow` orchestrator per SD-25/26 doctrine.

## 1. What this bundle ships

SD-26's Epic 4 registered 19 PF1 sourcebooks as `book_stub` entries in the Stubs Registry and wrote `data/stubs/<book>.json` manifests for each, with `content_kind_counts: null` (honest gap) and a `planned_resolution_bundle` field pointing at SD-27. **Per operator directive 2026-07-27, Beginner Box and Core Essentials have been removed from scope** (redundant to other tomes; will not be brought in); their registry slots (#0005 and #0012) and stub manifests, if they exist on disk, are out-of-scope and may be deleted by the closure epilogue with operator authorization. SD-27's payload is the resolution of those 19 stubs into real content: each in-scope book gets a Shape B JSON cache at `data/corpus/<book>/` populated from the source LST corpus, the corresponding `book_stub` registry entry flips to "Resolved", and a PCGen parity baseline is laid down for each.

The bundle does not introduce new engine work, new class chassis, or new rule mechanics. It is content ingestion for an existing engine.

## 2. What this bundle does not ship

- v0.6's class/race chassis breadth (Fighter/Wizard/Rogue + 8 remaining CRB classes) — owned by v0.6.
- Equipment-attachment schema (`EquipmentSelection` modifier-attachment relationship) — owned by v0.6's active scoping.
- Feat-effects engine beyond the 4-feat scope v0.6 already landed — owned by v0.6.
- Class-skill recognition beyond Fighter/Wizard/Rogue — owned by v0.6.
- Starting wealth for non-CRB classes — owned by v0.6.
- Companion / animal / familiar stat-block engine — not in scope; future program.
- Parameterized feats (Skill Focus with a chosen skill, Teamwork feats, etc.) — not in scope; future program.
- Temporary HP / favored-class-bonus HP wiring — not in scope; durability implementation lives at the character-record level.
- UI affordances (arcane-school selector, unequip button) — not in scope; v0.6 backlog.

The full carve-out list with citations lives in `forward-scope-register.md` §"Class excluding."

## 3. Bundle shape

Per `spec-domain-bundle-authoring` v1.2.0, the bundle has the canonical 10-file chassis, all internal to `requirements/SD-27-future-state-book-content-ingestion/`. The order is:

1. `forward-scope-register.md` — planning entry point (operator, signed off before scope-draft).
2. `scope-draft.md` — committed scope (this file's companion; intentionally a separate artifact).
3. `decisions.md` — decision record (operator-pinned, per-cycle, per-epic).
4. `technical-design.md` — architectural surface (Shape B schema application + per-book ingestion pipeline).
5. `technical-requirements.md` — pre-loop prerequisites + normative requirements + out-of-scope.
6. `epic-breakdown.md` — per-cycle stories keyed to the epic structure.
7. `loop-instruction.md` — per-cycle procedure (dual-audit gate, etc.).
8. `progress.md` — live: cycle log + `## TODO` + `## DONE` + `## DISCOVERED` + `## Status matrix` + `## Open blockers`.
9. `release-notes.md` — generated at closure epic.
10. `acceptance-and-verification.md` — per-criterion acceptance + verification commands.

Subdirectories:

- `artifacts/epic_<n>/<cycle>_receipt.md` — per-cycle durable execution receipt.
- `artifacts/epic_n/.gitkeep` — directory preservation.

## 4. Cross-reference

- `./forward-scope-register.md` — the planning entry point; load before this scope-draft.
- `./scope-draft.md` — companion, the committed scope.
- `../../repos/codex/docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/` — predecessor, the canonical source for the 19 stubs + the Shape B schema.
- `../../repos/codex/docs/governance/identifier-discipline.md` — identifier-disclosure doctrine; SD-27 cycles must pass the audit.
- `../../repos/codex/docs/governance/no-stub-mvp-doctrine.md` — wired-integration doctrine; SD-27 cycles must pass the four-check audit.
- `../../repos/codex/docs/release/v0.6/` — active sidecar; file-touch partition applies.
- `skill:spec-domain-bundle-authoring` — bundle-authoring discipline.
- `skill:workflow-orchestrated-dispatch` — dispatch shape.
- `skill:identifier-discipline` — audit + rename cycle.
- `skill:wired-integration-discipline` — four-check audit + ceremony.
- `skill:release-package-promotion` — promotion from workspace to repo.
- `skill:dual-canonical-doctrine` — workspace-citation + repo-local canonical pattern.
- `skill:loop-instruction-doctrine` — Claude-Code-driven cycle bundle operational form.

## 5. Hard-stop conditions

SD-27 stops and reports the blocker instead of guessing when:

- The bundle label discrepancy (`SD-27` vs. `SD-27+ (unscheduled)`) is not resolved by the operator by the end of cycle 1.1.
- v0.6's class-skill / equipment-attachment / feat-effects work lands a discrete change to `data/corpus/<book>/` for any of the 4 in-scope books that conflicts with SD-27's per-book ingestion.
- A per-book cycle fails the dual-audit gate twice in a row (the 19 future-state books share the same architectural pattern; failure here is a schema-level defect, not a per-book issue).
- The CG-03 (Human ability-modifier bug) baseline shift causes a per-book parity cycle to fail with a "9-of-9 expected" assertion that the operator never signed off on.

## 6. Why this scope is the right size

The 19 future-state books share the same architectural pattern (Shape B JSON cache + per-book parity baseline). The cycle is templated. The 2 in-scope future-state books' cycles (E2.1-2.2, E3.1-3.2) serialize on the shared `data/corpus/<book>/` directories; the 17 deferred future-state books' cycles fan out in SD-28+. The operator's tier (Sonnet at the per-cycle level, with the option to swap to a free-or-discounted model for the per-book cycle bodies) is determined by the cycle's content, not by the bundle's design.

The bundle's natural bottlenecks are:

- Per-book PCGen export XML parsing (for the parity baseline) — templated, already working for CRB/APG/ACG/Bestiary 1.
- Hand-authored class-skill / equipment attribution content for the 19 future-state books — not in scope of SD-27; the SD-26 cache for the 4 in-scope books establishes the precedent.
- LST-source independence — the 19 future-state books live at `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/<book>/`. The dispatcher verifies the directory exists and picks records verbatim, no transformation.

The bundle is engineered to be the smallest plausible content-ingestion step that takes the predecessor's 19 stubs to a real state. Anything larger would get into engine work, which is v0.6's lane.
