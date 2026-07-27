# SD-27 — References Index

## 1. Repo-local doctrine (REPO-LOCAL CANONICAL)

- `docs/governance/loop-instruction-template.md` — canonical loop-instruction template (operator-pinned 2026-07-21).
- `docs/governance/no-stub-mvp-doctrine.md` — wired-integration parent doctrine.
- `docs/governance/identifier-discipline.md` — identifier-discipline sibling doctrine.
- `docs/governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions; carries the 21 `book_stub` entries SD-26 registered (entries #0005 + #0012 marked out-of-scope per operator directive 2026-07-27).
- `docs/governance/ogl-pi-blacklist.md` (new, cycle 2.0.5) — the per-field PI blacklist; the 5th dual-audit (PI-blacklist grep) enforces it.
- `docs/governance/spec-domain-lifecycle.md` — spec-domain lifecycle routing.

## 2. Skills (Hermes-profile-scoped)

- `~/.hermes/profiles/god-emporer/skills/devops/release-package-promotion/SKILL.md` — workspace → repo publish; copy-with-normalize posture for SD-27 (the workspace copy remains the operator's editor-of-record).
- `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md` — dual-audit gate partner; the four-check audit is per-cycle.
- `~/.hermes/profiles/god-emporer/skills/devops/identifier-discipline/SKILL.md` — dual-audit gate partner.
- `~/.hermes/profiles/god-emporer/skills/devops/kanban-claude-code-execution-receipt/SKILL.md` — receipt schema (the durable proof a slice was executed).
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/SKILL.md` — the canonical dispatch shape; `Workflow` orchestrator at `scripts/workflow-dispatch.sh`.

## 3. Sibling bundles (REPO-LOCAL)

- `../docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/` — Tier-1 launch-gate dependency. SD-26 ships the Shape B v0 schema + the Stubs Registry `book_stub` kind + the 21 future-state book stubs. SD-27 consumes all three and adds the v1 license-aware schema bump (cycle 2.0.5).
- `../docs/release/SD-25-ui-evaluation-defect-closure/` — closed predecessor; Tier-1 launch-gate dependency for SD-26 (which is SD-27's).
- `../docs/release/SD-24-beta-readiness-and-multiclass/` — closed predecessor.
- `../docs/release/SD-23-character-mutation-and-wired-integration/` — closed; canonical cycle-receipt shape.

## 4. Cross-cutting operator directives (Honcho duracons, 2026-07-25 / 2026-07-27)

- **2026-07-25 17:39:26** — operator pin: shape cross for SD-26 (and inherited by SD-27); JSON cache is repo-resident at `data/corpus/<book>/`.
- **2026-07-25 18:00:00** — operator pin: SD-27 ships only 2 future-state books (Advanced Race Guide, Adventurer's Guide) per the "tune, then go wide" model; the other 19 future-state books are deferred to SD-28+.
- **2026-07-25 (subsequent, OGL review)** — operator pin: per-record `license` field on every Shape B v1 record; 5th dual-audit (PI-blacklist grep); redaction-to-marker policy (`"PI"` → `"PI-REDACTED"` with `[redacted PI]` value).
- **2026-07-25 18:30:00** — operator pin: per-cycle tier model; Sonnet default, free/discounted model operator-authorized for per-book cycle bodies (E2.1-2.2, E3.1-3.2).
- **2026-07-25 19:00:00** — operator pin: SD-27 does not author class engines; v0.6 owns the chassis breadth work (Fighter/Wizard/Rogue + 8 remaining CRB classes).
- **2026-07-27 (this turn)** — operator pin: Beginner Box and Core Essentials removed from scope (redundant to other tomes; will not be brought in). Their registry slots (#0005 and #0012) and stub manifests, if they exist on disk, are out-of-scope and may be deleted by the closure epilogue with operator authorization. Bundle book-count: 21 → 19 future-state; surface count: 22 → 20 (19 stubs + 1 decisions.md); all-23-books verify (4 in-scope + 2 in-scope future-state + 17 deferred).

## 5. External

- `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` — 25 PF1 book directories (4 in-scope + 19 future-state, after the 2 scope-removals; was 4 + 21 before 2026-07-27).
- `~/workspace/repos/pcgen/gradlew` + `code/testsuite/base-xml.ftl` — PCGen Gradle headless route.
- `tests/fixtures/rules_core/pf1_*_level*_*.txt` — ~30 deterministic character inputs (cache inputs).
- `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` — SD-26's pilot case for E2 verification; SD-27's E3.x cycles mirror this pattern for ARG and AG.