# SD-25 — References Index

> **Per `./scope-draft.md §4`.** Doctrine pointers, skill pointers, sibling-bundle pointers.

## 1. Repo-local doctrine (REPO-LOCAL CANONICAL)

- `/governance/loop-instruction-template.md` (REPO-LOCAL CANONICAL) — the canonical loop-instruction template every new SD-N authors from (operator-pinned 2026-07-21).
- `/governance/no-stub-mvp-doctrine.md` — wired-integration parent doctrine (skill: `wired-integration-discipline`).
- `/governance/identifier-discipline.md` — identifier-discipline sibling doctrine (skill: `identifier-discipline`).
- `/governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions (E3.3's StubAdapter entry lands here).
- `/governance/spec-domain-lifecycle.md` — spec-domain lifecycle routing.

## 2. Skills (Hermes-profile-scoped)

- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/SKILL.md` — the canonical dispatch shape (operator-pinned 2026-07-21).
- `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md` — dual-audit gate partner.
- `~/.hermes/profiles/god-emporer/skills/devops/identifier-discipline/SKILL.md` — dual-audit gate partner.
- `~/.hermes/profiles/god-emporer/skills/devops/kanban-claude-code-execution-receipt/SKILL.md` — per-cycle receipt schema.

## 3. Sibling bundles (REPO-LOCAL)

- `../docs/release/SD-24-beta-readiness-and-multiclass/` — Tier-1 launch-gate dependency (closure PR → develop).
- `../docs/release/SD-23-character-mutation-and-wired-integration/` — closed predecessor; canonical cycle-receipt shape; Tauri command-surface extension context.
- `../docs/release/SD-22/` — closed predecessor; PCGen headless Gradle route at `code/testsuite/base-xml.ftl`.

## 4. Cross-cutting operator directives (Honcho duracons, 2026-07-21)

- **2026-07-21 18:27:40** — operator directive to use the new `/governance/loop-instruction-template.md` going forward
- **2026-07-21 18:25:43** — operator's "agree with all 3 of your conclusions" / move Character-hub-of-hubs from SD-26 to SD-25
- **2026-07-21 18:17:52** — Qwen adapter acknowledged as contingency, not a binding plan input
- **2026-07-21 18:11:41** — operator's "library option alongside test cases" — TDD plus oracle-parity validation
- **2026-07-21 18:04:18** — operator's interest in scripting gradle-based PCGen output
- **2026-07-21 17:59:09** — operator's validation-goal framing (PCGen vs Codex oracle comparison)
- **2026-07-21 17:54:34** — operator asked for current contents of SD-25 (this was the SD-25 re-draft confirmation moment)
- **2026-07-21 17:48:31** — operator pin: ~15h Anthropic budget remaining; gets something running fast; 26 books crunch is a good use of remaining tokens
- **2026-07-21 17:39:26** — operator pin: scope cross for SD-26 (don't defer)
- **2026-07-21 17:25:25** — operator: "character hub as a hub of hubs so that each rule system can operate independently"
- **2026-07-21 15:41:03** — operator pin: stub visibility for future-state books; in-scope books (Core+APG+ACG+B1) carry no stubs
- **2026-07-21 15:36:12** — operator pin: durable JSON artifacts that persist SD-to-SD
- **2026-07-21 15:25:43** — operator pin: 20-min-per-class is doctrine-cost, not work-cost; throughput problem is real
- **2026-07-21 15:15:42** — operator pin: SD-26 to focus on ingest and testing strategy
- **2026-07-21 14:42:27** — operator pin: SD-24 launch form `/loop 1m /batch /goal ./loop-instruction.md` (pre-Workflow override)
- **2026-07-21 13:25:25** — operator pin: tranche/5-N cadence

## 5. External

- `/home/ubuntu/workspace/repos/pcgen/gradlew` — PCGen Gradle wrapper (verified present)
- `/home/ubuntu/workspace/repos/pcgen/code/testsuite/base-xml.ftl` — headless Gradle route for PCGen export
- `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` — pilot case for E4 verification
- `tests/fixtures/rules_core/pf1_*_level*_*.txt` — ~30 deterministic inputs already on disk (SD-26's library-build input set)
- `src/oracle_validation/{golden_fixture,selected_parity_dimensions}.rs` — Oracle-harness schema surface
