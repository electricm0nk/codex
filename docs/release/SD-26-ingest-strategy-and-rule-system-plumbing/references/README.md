# SD-26 — References Index

## 1. Repo-local doctrine (REPO-LOCAL CANONICAL)

- `docs/governance/loop-instruction-template.md` — canonical loop-instruction template (operator-pinned 2026-07-21).
- `docs/governance/no-stub-mvp-doctrine.md` — wired-integration parent doctrine.
- `docs/doctrine-external/identifier-discipline.md` — identifier-discipline sibling doctrine.
- `docs/governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions; **gains a new `book_stub` kind in E4.1**.
- `docs/doctrine-external/spec-domain-lifecycle.md` — spec-domain lifecycle routing.

## 2. Skills (Hermes-profile-scoped)

- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/SKILL.md` — the canonical dispatch shape.
- `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md` — dual-audit gate partner.
- `~/.hermes/profiles/god-emporer/skills/devops/identifier-discipline/SKILL.md` — dual-audit gate partner.
- `~/.hermes/profiles/god-emporer/skills/devops/kanban-claude-code-execution-receipt/SKILL.md` — receipt schema.

## 3. Sibling bundles (REPO-LOCAL)

- `../docs/release/SD-25-ui-evaluation-defect-closure/` — Tier-1 launch-gate dependency. SD-25 ships the Hub-of-Hubs interface + PCGen runner; SD-26 consumes both.
- `../docs/release/SD-24-beta-readiness-and-multiclass/` — closed predecessor.
- `../docs/release/SD-23-character-mutation-and-wired-integration/` — closed; canonical cycle-receipt shape.

## 4. Cross-cutting operator directives (Honcho duracons, 2026-07-21)

- **2026-07-21 18:27:40** — `docs/governance/loop-instruction-template.md` is the canonical source for every new bundle's loop-instruction.
- **2026-07-21 17:48:31** — operator's "process all 26 books" override (operator pin: SD-26 ships per-book for every PF1 directory).
- **2026-07-21 17:39:26** — operator pin: scope cross for SD-26; JSON cache is repo-resident.
- **2026-07-21 17:25:25** — operator pin: character hub as hub of hubs (routing layer for future rule systems).
- **2026-07-21 15:41:03** — operator pin: in-scope books (Core+APG+ACG+B1) carry no stubs; future-state books carry operator-granted stub entries.
- **2026-07-21 15:36:12** — operator pin: durable JSON artifacts that persist SD-to-SD.
- **2026-07-21 15:25:43** — operator pin: throughput problem is real (doctrine-cost, not work-cost).
- **2026-07-21 15:15:42** — operator pin: SD-26 focuses on ingest + testing strategy.

## 5. External

- `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` — 25 PF1 book directories (4 in-scope + 21 future-state).
- `~/workspace/repos/pcgen/gradlew` + `code/testsuite/base-xml.ftl` — PCGen Gradle headless route.
- `tests/fixtures/rules_core/pf1_*_level*_*.txt` — ~30 deterministic character inputs (cache inputs).
- `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` — pilot case for E2 verification.
- `src/oracle_validation/{golden_fixture,selected_parity_dimensions}.rs` — Oracle-harness schema (E2 reads + extends).
