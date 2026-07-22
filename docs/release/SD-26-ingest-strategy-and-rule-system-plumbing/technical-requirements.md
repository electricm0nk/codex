# SD-26 — Technical Requirements

> **Operating method:** see `./scope-draft.md`. Pre-loop prerequisites + normative requirements + out-of-scope deferrals.

## 1. Pre-loop prerequisites

Per `/governance/loop-instruction-template.md §1`:

### 1.1 Environment
- `codex-tranche-5` kanban board reachable.
- `tranche/5-4` branch on origin.
- Working tree clean on `tranche/5-4`.
- Skill `workflow-orchestrated-dispatch` loaded.

### 1.2 Tier-1 launch-gate dependency
- SD-25 closure PR merged to develop. SD-25 ships the Hub-of-Hubs interface + PCGen runner scaffolding; SD-26 consumes both.

### 1.3 Credentials
- Classic PAT present at `~/.config/gh/.claude_gh_token`.

### 1.4 Doctrine files
- `governance/identifier-discipline.md` + `governance/no-stub-mvp-doctrine.md` + `governance/wired-integration-stubs-registry.md` (the last gains a `book_stub` kind in E4.1).

### 1.5 Build counter
- Develop at `0.5.97` post-SD-24; SD-25 closure at `0.5.98`; SD-26 first concrete `0.5.99`. Read from `apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json`.

### 1.6 Artifact directories
- `artifacts/{epic_1,epic_2,epic_3,epic_4,epic_5,epic_6}/` + `data/corpus/` + `data/stubs/`.

## 2. Normative requirements (per cycle)

### 2.1 TDD is mandatory
RED → GREEN → re-audit. Per repo `AGENTS.md §1`.

### 2.2 Dual-audit gate
Per `loop-instruction.md §6` + template §6. Both `OK_*` required.

### 2.3 Identifier discipline
No bundle-tag leaks; PascalCase / camelCase per `identifier-discipline SKILL.md v1.5.0`. SD-26 scopes the identifier-audit grep to `apps/desktop/`, `apps/desktop/src-tauri/`, `src/`, `scripts/`, `data/`, plus the new `governance/wired-integration-stubs-registry.md` file.

### 2.4 Wired-integration discipline
Per `wired-integration-discipline SKILL.md v1.1.0`. Real calls, real results, real UI updates, real state re-fetch.

### 2.5 Cycle artifact schema
Per `loop-instruction.md §7`.

### 2.6 Kanban card mint AFTER done receipt
Per `loop-instruction.md §6 step 8`.

### 2.7 Operator identity in commits
`Todd Hintzmann <todd@hintzmann.net>`.

### 2.8 Concurrent-write protocol
Per `loop-instruction.md §5`: `git fetch && git rebase origin/<branch> && git push origin HEAD:<branch>`. Retry up to 5 times; then `CLAIM-EXISTS`. Applies to code commits, `progress.md`, `receipts.md`, `data/corpus/**/*.json`, `data/stubs/**/*.json`, and `governance/wired-integration-stubs-registry.md`.

## 3. Hard requirements (bundle-level)

### 3.1 Workflow orchestrator, not /loop /batch
Per `/governance/loop-instruction-template.md §2` + `AGENTS.md §7`.

### 3.2 JSON cache is repo-resident
Per operator directive 2026-07-21 17:39:26. `data/corpus/` + `data/stubs/` are committed to the codex repo.

### 3.3 In-scope books (Core+APG+ACG+B1) carry no stubs
Per operator directive 2026-07-21 15:41:03. The 4 in-scope books have full JSON cache content; the 21 future-state books carry operator-granted stub entries in the Stubs Registry.

### 3.4 Build version
First concrete: `0.5.99`. Per `decisions.md §4`.

### 3.5 Publish mode
Move-not-copy from workspace-side to `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/` on `tranche/5-4`. Workspace-side copy deleted on the publish commit.

### 3.6 Stubs Registry `book_stub` kind
Per `decisions.md §8`. 21 entries (one per future-state book). Each carries `book_id`, `book_name`, `status: stubbed`, `planned_resolution_bundle: "SD-27"`, `registered_by`, `registered_at`, `operator_granted: true`.

## 4. Out-of-scope (deferred to follow-on bundles)

- **Rule-system implementations beyond Pf1Adapter.** The trait exists in SD-25 (Hub-of-Hubs); concrete implementations land in SD-27+.
- **PCGen library build beyond the 4 in-scope books.** (If SD-26's E3 reveals the manual-vs-library ratio inverts for the future-state books, that's an SD-27 question.)
- **Equipment corpus extension** beyond PF1 core + APG + ACG + Bestiary 1.
- **Storage-tier structural convergence.**
- **Identifier-discipline directory renames.**
- **Inline mocks / "Would …" strings outside bundle's file-touch.**

## 5. Cross-reference

- `./scope-draft.md` — bundle intent
- `./decisions.md` — bundle-specific ADRs
- `./loop-instruction.md` — cycle mechanics
- `./acceptance-and-verification.md` — closure gates
- `./risks-and-open-questions.md` — risks + override flags
- `/governance/loop-instruction-template.md` — canonical template
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` — orchestrator skill
