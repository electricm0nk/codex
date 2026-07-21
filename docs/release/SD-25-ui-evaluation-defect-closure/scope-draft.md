---
title: SD-25 — UI-Evaluation Defect Closure + Hub-of-Hubs + PCGen Runner + Ingest Diagnostic Sketch (Tranche-5-3 Bundle)
status: planning-ready (operator directives 2026-07-21; bundle authored from /governance/loop-instruction-template.md + skill workflow-orchestrated-dispatch; 8 epics / ~24 criteria; Workflow-orchestrated dispatch)
date: 2026-07-21
canonical_branch: tranche/5-3 (dash from tranche/5-2; SD-24 closed on tranche/5-2 → develop; SD-25 sequence number)
kanban_board: codex-tranche-5 (reused after SD-24 closure PR lands)
companion_to: ./decisions.md
mirror_of: ./decisions.md
loop_launch_form: scripts/workflow-dispatch.sh (Workflow orchestrator, per operator directive 2026-07-21)
cycle_dispatch_model: deterministic-seeded-then-dynamic (per SD-24 doctrine; SD-25 inherits via /governance/loop-instruction-template.md)
publish_mode: move-not-copy from workspace planning surface to docs/release/SD-25-ui-evaluation-defect-closure/ on tranche/5-3; workspace copy deleted on publish commit
first_concrete_build_value: 0.5.98 (develop at 0.5.97; per template §1 item 7)
---

# SD-25 — UI-Evaluation Defect Closure, Hub-of-Hubs, PCGen Runner, Ingest Diagnostic Sketch

> ## ⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️
>
> This bundle dispatches via a `Workflow` orchestrator script (`scripts/workflow-dispatch.sh`), **not** via `/loop 60m /batch /goal ./loop-instruction.md`. The legacy form does not run unattended (operator directive 2026-07-21; `/governance/loop-instruction-template.md` + skill `workflow-orchestrated-dispatch`).
>
> `/loop` is canonical as a 60-minute tick floor; `/batch` requires a human invocation and is not used.
> `/goal <loop-instruction.md>` becomes `/task <cycle_doc>` per cycle; the orchestrator spawns the right subagent.
> Claude Code drives the loop dispatch.
>
> Per operator directive 2026-07-21: **kanban is the durable receipt layer only** — cards minted on `codex-tranche-5` *after* each cycle's artifact is written. Kanban does NOT dispatch work.
>
> A coding harness picking up this bundle reads this callout + `loop-instruction.md` + `scripts/workflow-dispatch.sh` + the per-criterion `cycles/<epic>_<criterion>.md` before reaching any other prose.
>
> **Pre-launch checklist (operator action only, before first dispatch):** captured in `README.md §1` with verbatim command output as required by `/governance/loop-instruction-template.md §1`.

## 0. Preamble

SD-25 carries four loads in one bundle:

1. **Hub-of-Hubs refactor** (Epic 3) — `apps/desktop/src-tauri/src/character_hub.rs` and `apps/desktop/src/characterHub/` run through a `RuleSystemAdapter` trait so PF1 + future rule systems (D&D 5e, Pathfinder 2e, D&D 3.5, custom homebrew) each implement the same interface. Unlocks SD-26's parallel per-class cycle pattern.
2. **PCGen Runner Scaffolding** (Epic 4) — Bash + Gradle + Python wrapper that takes a character input, invokes PCGen headless via `/home/ubuntu/workspace/repos/pcgen/gradlew` against `code/testsuite/base-xml.ftl`, normalizes the XML output, and writes it to `tests/fixtures/oracle_validation/pcgen_outputs/`. No library build yet — that's SD-26's job. SD-25 just proves the runner works.
3. **Corpus Ingest Diagnostic Sketch** (Epic 5) — Tauri command + UI panel route returning per-book ingest status. Sketch only: status flags + per-book counters. SD-26 fans out the full status table once the JSON cache lands.
4. **UI-Evaluation Discovered Backend Defects** (Epic 6) — discovery-dominant epic driven by the operator's UI-eval session on 2026-07-21. Per `## DISCOVERED` discipline of SD-24 doctrine.

The bundle's shape: 8 epics / ~24 criteria. E1 + E2 are governance + gating as canonical. E3/E4/E5 are structural work. E6 is discovery-driven. E7 absorbs SD-22/SD-24 per-class residue. E8 is the closure epilogue.

Working against `tranche/5-3`. Per-cycle concurrency shape is decided at authoring time per the loop-instruction's §3 table, not derived live.

## 1. SD-25 — 8 epics

### 1.1 Epic 1 — Code-Side Identifier Cleanup (governance base; fires FIRST)

Under the identifier-discipline doctrine (`/governance/identifier-discipline.md` + skill `identifier-discipline`), audit and remove bundle-tag identifier leaks. Post-SD-23 cleanup, scope is defensive: any remaining `sd<N>_*` leaks from prior sessions, plus `sd25_*` patterns the loop introduces. Per template §1 item 6: skills are doctrine docs not hermes-skill-loaded; the audit is enforced inline by grep in `loop-instruction.md §6`.

### 1.2 Epic 2 — Operator Pre-Launch (gating epic; fires after E1)

Five pre-flight checks per the template's exact shape: kanban reachable, branch on origin, SD-24 closure PR merged to develop, PAT present, working tree clean. The `Progress.md ## Status matrix` row 2.3 is the Tier-1 launch gate — the loop refuses to dispatch Epic 3+ until SD-24 closure is verified in develop.

### 1.3 Epic 3 — Character Hub as Hub of Hubs (Rule-System Adapter Interface)

This is the structural refactor that unlocks SD-26's parallel-per-class pattern. Five criteria + four cycles parallel-eligible:

- **Criterion 3.1** — `RuleSystemAdapter` trait definition in `apps/desktop/src-tauri/src/rule_system_adapter.rs`. The interface that PF1 + future rule systems implement. Methods: `chassis_resolve`, `level_up`, `save_character`, `append_to_character`, `recompute`, `list_saved_characters`, `load_saved_character`.
- **Criterion 3.2** — `Pf1Adapter` extraction from `apps/desktop/src-tauri/src/character_hub.rs`. The existing implementation moves under the trait. Existing tests pass.
- **Criterion 3.3** — `StubAdapter` future-system stub: returns "Would render for system X; not yet implemented" results. Operator-granted stub per the wired-integration doctrine — `governance/wired-integration-stubs-registry.md` gets one entry per future system.
- **Criterion 3.4** — Tauri command-surface routes through the hub-of-hubs. `append_to_character`, `recompute_character`, `re_save_character` accept a `rule_system_id` argument and dispatch through the trait. (Serial — touches multiple Tauri command files.)
- **Criterion 3.5** — UI panel adapter-aware: `apps/desktop/src/characterHub/` reads the active rule-system adapter and routes interactions through it.

**Concurrency:** 3.1, 3.2, 3.3, 3.5 = `parallel: yes` (different file each); 3.4 = `parallel: no`.

### 1.4 Epic 4 — PCGen Runner Scaffolding

The PCGen runner proves the script + gradle + normalize pipeline works for one case. SD-26 builds the library on top of it. Four criteria + three parallel:

- **Criterion 4.1** — `scripts/pcgen-run-character.sh` — Bash + Gradle + jq wrapper that takes a `character_input_ref`, runs `/home/ubuntu/workspace/repos/pcgen/gradlew` against `code/testsuite/base-xml.ftl`, and emits the raw XML output.
- **Criterion 4.2** — `scripts/pcgen-normalize-output.py` — Python script that reads the raw XML, extracts the selected parity dimensions (mirroring `src/oracle_validation/selected_parity_dimensions.rs`), and writes the normalized JSON to `tests/fixtures/oracle_validation/pcgen_outputs/<case>.json` with a SHA-256 frontmatter.
- **Criterion 4.3** — `tests/oracle_validation/pcgen_runner_smoke.rs` — Rust-side smoke test that calls the runner against `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` (the pilot case) and verifies the normalized output has the expected SHA.
- **Criterion 4.4** — verification cycle: run all three for one case (pilot), confirm output is grounded against `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`. (Serial — reads + writes multiple artifacts.)

**Concurrency:** 4.1, 4.2, 4.3 = `parallel: yes`; 4.4 = `parallel: no`.

### 1.5 Epic 5 — Corpus Ingest Diagnostic Sketch

One cycle. One Tauri command `corpus_ingest_diagnostic` returning `Vec<BookIngestStatus>` (book_id, status, last_ingested_at, content_kind_counts). One UI panel route. **Sketch only**: SD-26 fans out the full status table + flags + ETA against the JSON cache. SD-25 ships the shape.

### 1.6 Epic 6 — UI-Evaluation Discovered Backend Defects (dynamic-dominant)

The bulk of SD-25's actual work. The cycle picker reads `## DISCOVERED` for UI-eval session findings, applies the dual-audit gate + TDD per defect, emits per-defect `artifacts/epic_6/<defect-id>_cycle_receipt.md`. Closed by the closure-readiness scan in E8.

### 1.7 Epic 7 — Deferred Per-Class Work & SD-22/SD-24 Coverage Backlog

Criterion 7.1 = "Per-class residue intake" (reads SD-24's `per-class-coverage-matrix.md`). Criteria 7.2..7.M = per-feature cycles, dynamically spawned as `## DISCOVERED` entries from the intake.

**Added 2026-07-21 (SD-24 closure findings):** Epic 7 also intakes SD-24's `progress.md ## Open blockers` directly — two real, corpus-data-limited equipment/spell coverage gaps (CRB equipment description 61.2%, APG equipment description 0%, APG spell full text 87.9%) — plus a plain scope gap SD-24's own orchestrator left out entirely (Bestiary 1 equipment + spells were never dispatched, despite being in SD-24's own declared book scope). Recommended resolution for the three real ceilings: a second-source web content pass against **d20pfsrd.com** / **aonprd.com** (Archives of Nethys) before accepting the ceiling or fabricating text, with identity-match verification and source-URL citation per record. See `epic-breakdown.md`'s Epic 7 "Equipment/spell corpus intake" criterion for the full recommendation and per-item numbers.

### 1.8 Epic 8 — Closure Epilogue (final scan + architecture-truth-up + graphify-update + release-notes + version increment; fires LAST)

Standard part-of-handoff doctrine: scans every prior criterion, runs the architecture-truth-up + graphify-update sub-steps (the latter with graceful failure), generates release notes (Haiku), bumps the version (Haiku), opens the `tranche/5-3 → develop` PR, runs the merge-conflict-resolution script per the template.

**Subagent tiering within E8 (Haiku-defaulted):**
- 8.3 release-notes = Haiku.
- 8.4 version-bump = Haiku.
- 8.1 final-criterion-scan = Sonnet.
- 8.2 architecture closure-pipeline = Opus (per template §2's "adversarial verification / judge-panel = Opus" line).
- 8.5 PR + merge = Sonnet.

## 2. Bundle at a glance

- **Slug:** `SD-25-ui-evaluation-defect-closure`
- **Branch:** `tranche/5-3` (operator directive 2026-07-21)
- **Board:** `codex-tranche-5` (reused)
- **Epics:** 8 / **Criteria:** ~24 (5 declarative per Epic 3, 4 per Epic 4, 5 per Epic 2, 1 per Epic 5, 1 cycle-shape per Epic 6 + ~5 dynamic defects, 1 cycle-shape per Epic 7 + ~3 dynamic per-class, 5 per Epic 8)
- **First concrete build:** `0.5.98`
- **Dispatch:** `Workflow` orchestrator (NOT `/loop /batch`)
- **Publish mode:** move-not-copy

## 3. Files in this folder

| File | Purpose |
|---|---|
| `README.md` | Bundle index + pre-launch checklist + orchestrator pointer |
| `scope-draft.md` | This file |
| `loop-instruction.md` | Per-cycle procedure (dual-audit, RED→GREEN, receipt schema) |
| `epic-breakdown.md` | 8 epics / ~24 acceptance criteria / per-cycle stories |
| `decisions.md` | Bundle-specific ADRs (§1 scope, §2 dispatch, §3 concurrency, §4 build, §5 publish, §6 tier-1 gate) |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split + override flags |
| `acceptance-and-verification.md` | Closure gates + per-criterion artifact map |
| `content-unit-inventory.md` | Per-content-unit N-tuple |
| `technical-design.md` | Architectural surface — hub-of-hubs interface, PCGen runner, JSON cache shape, visibility surface |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements + out-of-scope deferrals |
| `progress.md` | Live: cycle log + `## TODO` + `## DONE` + `## DISCOVERED` + `## Status matrix` + `## Open blockers` |
| `release-notes.md` | Generated at E8 (placeholder) |
| `scripts/workflow-dispatch.sh` | The Workflow orchestrator (author-once, run continuously) |
| `cycles/<epic>_<criterion>.md` | Per-criterion task documents (one per criterion) |
| `artifacts/<epic>/<cycle-id>_cycle_receipt.md` | Per-cycle durable receipts |
| `artifacts/README.md` | Cycle-artifacts index |
| `references/README.md` | Doctrine + skill + sibling-bundle pointers |

## 4. Cross-references

- `/governance/loop-instruction-template.md` — canonical loop-instruction template.
- `/governance/no-stub-mvp-doctrine.md` + skill `wired-integration-discipline`.
- `/governance/identifier-discipline.md` + skill `identifier-discipline`.
- `/governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions.
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` — orchestrator procedure.
- `../docs/release/SD-24-beta-readiness-and-multiclass/` — Tier-1 launch-gate dependency (closure PR → develop).
- `../docs/release/SD-23-character-mutation-and-wired-integration/` — closed predecessor; canonical cycle-receipt shape.
- `../docs/release/SD-22/` — closed predecessor; PCGen headless Gradle route at `code/testsuite/base-xml.ftl`.

## 5. Hard-stop conditions

- Working tree diverged from `tranche/5-3` needs manual rebase (template-defined).
- `## DISCOVERED` queue > 10 entries (operator override required to clear; pauses).
- Tier-1 launch gate unsatisfied (SD-24 closure PR not in develop).
- Two live orchestrators on the same `tranche/5-3` (first wins; second writes `CLAIM-EXISTS` blocker).
- RED → GREEN transition not preserved in cycle receipt.
- Cycle finds `success: true` from a fake operation; inline mock in shipping module; "Would …" string in shipping code (template-forbidden; cycle rejected).
