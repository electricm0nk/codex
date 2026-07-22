---
title: SD-26 — Ingest Strategy Revision + Rule-System Plumbing (Tranche-5-4 Bundle)
status: planning-ready (operator directives 2026-07-21: scope cross — PF1 + future-state rule systems, not deferred; 6 epics / 17 declarative + 21 dynamic criteria (38 total); Workflow-orchestrated dispatch; JSON cache for 4 in-scope books + 21 future-state book stubs; PCGen library build on top of SD-25's runner scaffolding; doctrine-revision to cut per-class gate-cost from ~20 minutes to ~6 minutes; build counter inheritance 0.5.99)
date: 2026-07-21
canonical_branch: tranche/5-4 (dash from tranche/5-3; SD-25 closes on tranche/5-3 → develop; SD-26 sequence number)
kanban_board: codex-tranche-5 (reused after SD-25 closure PR lands)
companion_to: ./decisions.md
mirror_of: ./decisions.md
loop_launch_form: scripts/workflow-dispatch.sh (Workflow orchestrator, per /governance/loop-instruction-template.md §2)
cycle_dispatch_model: deterministic-seeded-then-dynamic (per SD-24 / SD-25 inheritance)
publish_mode: move-not-copy
first_concrete_build_value: 0.5.99 (develop at 0.5.97 after SD-24 closure; SD-25 closure bumps to 0.5.98; SD-26 first concrete lands at 0.5.99)
---

# SD-26 — Ingest Strategy Revision + Rule-System Plumbing (Tranche-5-4 Bundle)

> ## ⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️
>
> Dispatches via `Workflow` orchestrator at `scripts/workflow-dispatch.sh` (NOT `/loop /batch`). Per `/governance/loop-instruction-template.md §2` + skill `workflow-orchestrated-dispatch` + `AGENTS.md §7`.
>
> Per operator directive 2026-07-21: **scope cross** — bundles PF1 + future-state rule systems, not deferred. **Pinned hypothesis:** if SD-26 splits the work along Anthropic-architecture vs. mechanical-fanout axes, the Anthropic bundle drives architecture + rules while a budget model like Qwen (if available) handles mechanical fan-out.
>
> Kanban is the durable receipt layer only — cards on `codex-tranche-5` after each cycle's artifact is written. Kanban does NOT dispatch.

## 0. Bundle at a glance

- **Branch:** `tranche/5-4`
- **Board:** `codex-tranche-5` (reused after SD-25 closure PR)
- **Dispatch:** `Workflow` orchestrator
- **Subagent tiering:** Sonnet default; Haiku for housekeeping (E6 release-notes / version-bump); Opus for adversarial verification (E6 architecture closure-pipeline)
- **Epics / criteria:** 6 / 17 declarative + 21 dynamic (38 total)
- **First concrete build:** `0.5.99`
- **Tier-1 launch-gate dependency:** SD-25 closure PR merged to develop (which carries the Hub-of-Hubs interface + PCGen runner scaffolding)

## 1. Pre-launch checklist (operator action only, before first dispatch)

Verified during drafting on 2026-07-21:

1. **Kanban board reachable.** `hermes kanban boards` shows `codex-tranche-5` (live-confirmed before SD-25 publish; same persisted board).
2. **`tranche/5-4` branch pushed to origin.** (Branch exists in workspace; will be created on origin by another agent at bundle publish.)
3. **SD-25 closure PR merged to develop.** Tier-1 launch-gate dependency. SD-25 ships the Hub-of-Hubs interface + PCGen runner scaffolding; SD-26 consumes those.
4. **PAT present** at `~/.config/gh/.claude_gh_token`.
5. **Working tree clean** on `tranche/5-4`.
6. **Doctrines loaded.** Skills loaded at `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` + doctrine docs at `governance/identifier-discipline.md` + `governance/no-stub-mvp-doctrine.md`.
7. **Build counter.** Develop at `0.5.97` (post-SD-24); SD-25 closure lands at `0.5.98`; SD-26's first concrete value is `0.5.99`. Read from `apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json`.
8. **Artifact directories** `artifacts/{epic_1,epic_2,epic_3,epic_4,epic_5,epic_6}/` exist and are empty.

## 2. Files in this folder

| File | Purpose |
|---|---|
| `README.md` | This file — index + pre-launch + orchestrator pointer. |
| `scope-draft.md` | Canonical handoff *what* — 6 epics, scope-cross posture, contract. |
| `loop-instruction.md` | Per-cycle launch *how* — dual-audit, RED→GREEN, receipt schema, concurrent-write. |
| `decisions.md` | Bundle-specific ADRs. |
| `epic-breakdown.md` | 6 epics / 17 declarative + 21 dynamic criteria (38 total) / per-cycle stories. |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split + override flags. |
| `acceptance-and-verification.md` | Closure gates + per-criterion artifact map. |
| `content-unit-inventory.md` | Per-content-unit N-tuple (per-book / per-class / per-rule-system). |
| `technical-design.md` | Architectural surface — JSON cache schema, oracle-harness comparator, book-stub-manifest, rule-system plumbing. |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements + out-of-scope. |
| `progress.md` | Live: cycle log + `## TODO` + `## DONE` + `## DISCOVERED` + `## Status matrix` + `## Open blockers`. |
| `release-notes.md` | Generated at E6 (placeholder). |
| `scripts/workflow-dispatch.sh` | The `Workflow` orchestrator. |
| `cycles/<epic>_<criterion>.md` | Per-criterion task documents. |
| `artifacts/<epic>/<cycle-id>_cycle_receipt.md` | Per-cycle durable receipts. |
| `artifacts/README.md` | Cycle-artifacts index. |
| `references/README.md` | Doctrine + skill + sibling-bundle pointers. |
| `data/corpus/` | JSON cache durable artifacts (per-book / per-content-kind / per-content-id). Per the operator-pinned doctrine `2026-07-21 15:36:12` ("ready to go, durable artifact that we can point to via all these json files that persist from SD to SD"). |
| `data/stubs/` | Future-state book-stub manifests. |

## 3. Why `data/` is repo-resident

Per operator 2026-07-21 17:39:26: **JSON cache is repo-resident** (commit to the codex repo, evolving with each ingest bundle). The cache lives under `data/corpus/` and `data/stubs/`, repo-resident, durable across SD-to-SD handoff. The hermaphroditic `'schemas/' + corpus' + 'stubs/'` split honors the operator's "in-scope books no stubs, future-state books knowingly stub" doctrine.

## 4. Cross-references

- `/governance/loop-instruction-template.md` — canonical template.
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` — orchestrator skill.
- `governance/no-stub-mvp-doctrine.md` + `governance/wired-integration-stubs-registry.md`.
- `../docs/release/SD-25-ui-evaluation-defect-closure/` — Tier-1 launch-gate dependency (closure PR → develop).
- `../docs/release/SD-24-beta-readiness-and-multiclass/` — closed predecessor.
- `src/oracle_validation/{golden_fixture,selected_parity_dimensions}.rs` — Oracle-harness schema (E2 reads + writes).
- `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` — pilot case for oracle-harness build-out.

## 5. Why scope-cross (operator-pinned 2026-07-21)

"If SD-26 splits the work along Anthropic-architecture vs. mechanical-fanout axes, … make the separation now before it gets more complicated" — operator directive 2026-07-21 17:39:26. SD-26 produces a working character-hub-of-hubs + JSON cache + oracle-harness + book-stub-manifest for PF1 + the structural surface for additional rule systems. Future rule-system implementations land in SD-27+ as separate bundles. The separation lands in SD-26 so the complexity budget is bounded.

## 6. Hard-stop conditions

- Working tree diverged from `tranche/5-4` needs manual rebase.
- `## DISCOVERED` queue > 10 entries (operator override required to clear).
- Tier-1 launch gate unsatisfied (SD-25 closure PR not in develop).
- Two live orchestrators on the same `tranche/5-4`.
- RED → GREEN transition not preserved in cycle receipt.
- Cycle finds `success: true` from a fake operation; inline mock in shipping module; "Would …" string in shipping code (template-forbidden; cycle rejected).
