# SD-25 — UI-Evaluation Defect Closure, Character-Hub-of-Hubs Refactor, PCGen Runner Scaffolding, Corpus Ingest Diagnostic Sketch

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️**
>
> This bundle is dispatched via a **`Workflow` orchestrator script** (authored at `scripts/workflow-dispatch.sh` per `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` and `/governance/loop-instruction-template.md` §2), **not** via the legacy `/loop 60m /batch /goal ./loop-instruction.md` form. The legacy form requires a human to type it per invocation and is not unattended-runnable. The script is the canonical mechanism.
>
> The bundle's `loop-instruction.md` is the operator-edited maintenance manual; `scripts/workflow-dispatch.sh` is the dispatch driver. Claude Code invokes the orchestrator script. Kanban is the durable receipt layer only — cards are minted on `codex-tranche-5` only with a done receipt; kanban does NOT dispatch work.
>
> A coding harness picking up this bundle reads this callout + `loop-instruction.md` + the orchestrator script before reaching any other prose.

## 0. Bundle at a glance

- **Branch:** `tranche/5-3` (dash from `tranche/5-2`; SD-24 closed on `tranche/5-2 → develop`)
- **Board:** `codex-tranche-5` (reused after SD-24 closure PR)
- **Dispatch mechanism:** `Workflow` orchestrator (see `scripts/workflow-dispatch.sh` and `loop-instruction.md §2`)
- **Subagent tiering:** Sonnet (default) for all real implementation; Haiku for housekeeping (release notes, version bump); Opus for adversarial verification + final completeness scan
- **Epics / criteria:** 8 / ~24
- **First concrete build value:** develop is at `0.5.97`; this bundle's first concrete value lands at `0.5.98` (per template §1 §7)
- **Tier-1 launch-gate dependency:** SD-24 closure PR merged to develop

## 1. Pre-launch checklist (operator action only, before first dispatch)

Verified during drafting on 2026-07-21; output captured below each command.

1. **Kanban board reachable.** `hermes kanban boards` (real subcommand per `/governance/loop-instruction-template.md §1 §1`'s correction — `list-boards` is wrong):

   ```
   $ hermes kanban boards
   ...
       codex-tranche-5           Codex Tranche 5 (SD-21 campaign manager + Drive + APG + ACG)  done=56
   ...
   ```

   `codex-tranche-5` confirmed reachable.

2. **`tranche/5-3` branch is on origin.** (Branch exists in workspace at `/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_*.5/...` from prior bundle drafts; will be created on origin by another agent at bundle publish per SD-23 / SD-24 precedent.)

3. **SD-24 closure PR merged to develop.** (Per `tranche/5-2 → develop`; Tier-1 launch-gate.)

4. **Classic PAT present** at `~/.config/gh/.claude_gh_token`. (Per the kanban dispatcher's respawn-guard footgun doctrine at duracon 2026-07-04 12:41:37.)

5. **Working tree clean on `tranche/5-3`.** `git status --porcelain | wc -l` returns 0 expected.

6. **Doctrine gates.** Identifier-discipline and wired-integration-discipline are **doctrine documents** at `governance/identifier-discipline.md` and `governance/no-stub-mvp-doctrine.md`, enforced inline by the dual-audit grep in `loop-instruction.md §6`. NOT hermes-skill-loaded (per template §1 §6's correction — `hermes skills --list` is a fictional command). Skill `workflow-orchestrated-dispatch` IS hermes-skill-loaded at `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/SKILL.md`.

7. **Build counter.** Develop is at `0.5.97`; this bundle's first concrete value is **`0.5.98`** (read from `apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json`, NOT root `Cargo.toml` which stays pinned at `0.1.0` per template §1 §7).

8. **Artifact directories exist and are empty:** `artifacts/{epic_1,epic_2,epic_3,epic_4,epic_5,epic_6,epic_7,epic_8}/`.

## 2. Orchestrator script

The `Workflow` orchestrator at `scripts/workflow-dispatch.sh` is authored-once-at-launch and run continuously. It:

- Reads the loop-instruction's per-epic concurrency map.
- Picks the highest-priority unclaimed criterion from `progress.md §TODO + §DISCOVERED`.
- Dispatches it to a Sonnet subagent (Haiku for E8 housekeeping; Opus for the closure-pipeline adversarial-verify steps).
- Applies the canonical concurrent-write protocol (see §3).
- Loops.

## 3. Concurrent-write protocol (canonical)

Every cycle that commits and pushes to `tranche/5-3` uses **this exact protocol**:

```bash
git fetch origin tranche/5-3 && git rebase origin/tranche/5-3 && git push origin HEAD:tranche/5-3
```

On non-fast-forward rejection: repeat up to 5 times. If still failing after 5 attempts, write a `CLAIM-EXISTS` blocker to `progress.md` and stop. **Do not force-push.** Applies to both the code commit and any shared-state file (`progress.md`, `receipts.md`): re-fetch and re-read before editing.

## 4. File map

| File | Purpose |
|---|---|
| `README.md` | This file — bundle index + pre-launch checklist + orchestrator pointer. |
| `scope-draft.md` | Canonical handoff *what* — 8 epics, scope, contract. |
| `loop-instruction.md` | Per-cycle launch *how* — dual-audit, red-green, receipt schema. |
| `decisions.md` | Bundle-specific ADRs. |
| `epic-breakdown.md` | 8 epics / ~24 criteria / per-cycle stories. |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split + override flags. |
| `acceptance-and-verification.md` | Closure gates + per-criterion artifact map. |
| `content-unit-inventory.md` | Per-content-unit N-tuple (UI-eval defect / per-class residue / Tauri command / hub module / PCGen output). |
| `technical-design.md` | Architectural surface — hub-of-hubs interface, PCGen runner wiring, JSON cache shape, visibility surface. |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements + out-of-scope deferrals. |
| `progress.md` | Cycle log + `## TODO` + `## DONE` + `## DISCOVERED` + `## Status matrix` + `## Open blockers`. |
| `release-notes.md` | Generated at Epic 8 (placeholder). |
| `scripts/workflow-dispatch.sh` | The `Workflow` orchestrator script — author-once, run continuously. |
| `cycles/<epic>_<criterion>.md` | Per-criterion task documents consumed by the orchestrator. |
| `artifacts/<epic>/<cycle-id>_cycle_receipt.md` | Per-cycle durable receipts. |
| `artifacts/README.md` | Cycle-artifacts index. |
| `references/README.md` | Doctrine pointers, skill pointers, sibling bundle pointers. |

## 5. Cross-references

- `/governance/loop-instruction-template.md` — canonical loop-instruction template (REPO-LOCAL).
- `/governance/no-stub-mvp-doctrine.md` + skill `wired-integration-discipline` — wired-integration parent.
- `/governance/identifier-discipline.md` + skill `identifier-discipline` — identifier-discipline sibling.
- `/governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions.
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` — orchestrator skill.
- `../docs/release/SD-24-beta-readiness-and-multiclass/` — Tier-1 launch-gate dependency (closure PR → develop).
- `../docs/release/SD-23-character-mutation-and-wired-integration/` — closed predecessor; canonical cycle-receipt shape.
- `../docs/release/SD-22/` — closed predecessor; PCGen headless Gradle route referenced at `code/testsuite/base-xml.ftl`.

## 6. Why the Workflow orchestrator

`/loop 60m /batch /goal <file>` was the SD-16 through SD-24 launch form. On launch day, `/batch` proved incompatible with unattended operation (requires a human to type it per invocation; cannot run from cron or background processes). The working mechanism is the `Workflow` tool: an author-once orchestrator script that fans agents in parallel where files are genuinely disjoint, serializes where they aren't. Per operator directive 2026-07-21; per `AGENTS.md §7`; per `/governance/loop-instruction-template.md §2`. SD-16 through SD-24 are historical instances and are NOT retrofitted.
