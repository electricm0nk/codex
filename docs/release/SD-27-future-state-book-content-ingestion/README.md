# SD-27 — Future-State Book Content Ingestion

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️**
>
> **The dispatcher is the in-harness `Workflow` tool, driven from a live session.** **NOT** `/loop /batch`, and **not** a headless `scripts/workflow-dispatch.sh` process — that script's `claude code --profile … --task …` invocation does not exist in the live CLI, and the script is not in this repo on any ref. Per `decisions.md §19`, adopting SD-26's `decisions.md §13` finding (which in turn carries forward SD-25's `decisions.md §10`).
>
> The deterministic half of dispatch — manifest seed, claim/complete state machine, dependency ordering, and every write to the operator's reporting JSON — lives in `scripts/sd27-workflow.py`. Run `python3 scripts/sd27-workflow.py preflight` before Epic 2.1. Per `docs/governance/loop-instruction-template.md §2` + skill `workflow-orchestrated-dispatch`.
>
> **Read first:** `forward-scope-register.md`. The scope-draft tightens the register's class-1 commitments into a concrete bundle shape. The register is the disagreement surface; the scope-draft is the committed shape.

## 0. Bundle at a glance

- **Slug:** `SD-27-future-state-book-content-ingestion`
- **Predecessor:** SD-26 (Ingest Strategy Revision + Rule-System Plumbing) — **landed on `develop`** as **PR #338** (`62e7b617`, confirmed ancestor of `origin/develop`, verified 2026-07-27). The Tier-1 launch gate is satisfied. (An earlier draft of this note said PR #339; that was a misread of commit `803ee60e`, which is a later, unrelated CG-03 bugfix.)
- **Sidecar:** v0.6-alpha release-swarm (active on `tranche/6`, not yet closed).
- **Branch:** `tranche/7` (operator directive forthcoming; SD-26's promotion to develop completes the merge-base for SD-27).
- **Board:** `codex-tranche-7` (governed convention slug; operator override on file).
- **In-scope books:** **Advanced Race Guide** + **Pathfinder Unchained** (2 of the 19 future-state books). Matches the operator's dashboard workchannel `SD-27 (ARG + PU)`. Adventurer's Guide is **not** SD-27's — it is routed to SD-30.
- **Epics:** 4 / **Criteria:** 4 declarative + 9 dynamic (13 total; 2 books × the 4-stage per-book cycle (license → pre-build → verify → parity) + a single bundle-level label-resolution cycle + the canonical governance + closure epics; the remaining 17 future-state books' cycles are deferred to SD-28+).
- **First concrete build:** `0.6.0` (per skill `identifier-discipline` — v0.6-alpha is the post-SD-26 line; SD-27 lands the next concrete build).
- **Dispatch:** `Workflow` orchestrator per SD-25/26 doctrine.

## 1. What this bundle ships

SD-26's Epic 4 registered **21** PF1 sourcebooks as `book_stub` entries in the Stubs Registry (`#0003`–`#0023`) and wrote `data/stubs/<book>.json` manifests for each, with `content_kind_counts: null` (honest gap) and a `planned_resolution_bundle` field pointing at SD-27. **Per operator directive 2026-07-27, Beginner Box and Core Essentials have been removed from scope** (redundant to other tomes; will not be brought in), leaving **19 future-state books** in the bundle's universe. Their registry slots (`#0005` and `#0012`) and stub manifests are still on disk — they are out-of-scope, not deleted, and may be removed by the closure epilogue with operator authorization.

SD-27's payload is the resolution of **2** of those 19 stubs into real content — Advanced Race Guide and Pathfinder Unchained. Each in-scope book gets a Shape B v1 JSON cache at `data/corpus/<book>/` populated from the source LST corpus, the corresponding `book_stub` registry entry flips to "Resolved", and a PCGen parity baseline is laid down. The other **17** are deferred to SD-28+ under the operator's 2026-07-25 "tune, then go wide" directive.

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

Per `spec-domain-bundle-authoring` v1.2.0, the bundle has the canonical 10-file chassis, all internal to `docs/release/SD-27-future-state-book-content-ingestion/` (the repo-local canonical home; the workspace-side author was moved here on the publish commit per `decisions.md §6`). The order is:

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
- `../SD-26-ingest-strategy-and-rule-system-plumbing/` — predecessor, the canonical source for the 21 registered stubs + the Shape B v0 schema.
- `docs/governance/no-stub-mvp-doctrine.md` — wired-integration doctrine; SD-27 cycles must pass the four-check audit.
- `docs/governance/loop-instruction-template.md` — the canonical loop-instruction template.
- `docs/governance/wired-integration-stubs-registry.md` — the `book_stub` entries SD-27 resolves.
- `../v0.6/` — active sidecar; file-touch partition applies.
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
- `python3 scripts/sd27-workflow.py preflight` fails, or the reporting JSON is unreachable. Per `loop-instruction.md §8`, a cycle never runs against an unreachable dashboard — an unreachable dashboard is indistinguishable from an empty one through the writer's API, and would be reported as a successful run that wrote nothing.
- The CG-03 (Human ability-modifier bug) baseline shift causes a per-book parity cycle to fail with a "9-of-9 expected" assertion that the operator never signed off on.

## 6. Why this scope is the right size

The 19 future-state books share the same architectural pattern (Shape B v1 JSON cache + per-book parity baseline). The cycle is templated. The 2 in-scope books (ARG, PU) are file-disjoint — each writes only its own `data/corpus/<book>/` — so their 4-stage chains are parallel-capable, serializing only on the shared stubs registry. In practice the operator's "tune, then go wide" model gates them serially (`scope-draft.md §1.2`): ARG closes or fails before PU dispatches. the 17 deferred future-state books' cycles fan out in SD-28+. The operator's tier (Sonnet at the per-cycle level, with the option to swap to a free-or-discounted model for the per-book cycle bodies) is determined by the cycle's content, not by the bundle's design.

The bundle's natural bottlenecks are:

- Per-book PCGen export XML parsing (for the parity baseline) — templated, already working for CRB/APG/ACG/Bestiary 1.
- Hand-authored class-skill / equipment attribution content for the 19 future-state books — not in scope of SD-27; the SD-26 cache for the 4 in-scope books establishes the precedent.
- LST-source independence — the 19 future-state books live at `$PCGEN_DATA_ROOT/<book>/` (default `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/`), which is **outside this repo**. `scripts/sd27-workflow.py preflight` verifies each in-scope book's directory exists and is non-empty before any cycle dispatches; records are picked verbatim, no transformation. Verified present: ARG (23 `.lst` files), PU (11).

The bundle is engineered to be the smallest plausible content-ingestion step that proves the per-book pattern on 2 books before it is applied to the remaining 17. Anything larger would get into engine work, which is v0.6's lane.
