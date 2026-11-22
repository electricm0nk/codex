# SD-23 Loop Instruction — Character Mutation and Wired Integration

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
>
> Operating form: `/loop 60m /batch /goal programs/codex/requirements/SD-23-character-mutation-and-wired-integration/loop-instruction.md`.
>
> This file is the supervisor's goal document. The loop ticks every 60 minutes; each tick runs one cycle, gated by the eligibility check below. `/batch` is the concurrency primitive — file-touch partition enforces one cycle per file at a time.

## Pre-launch checklist (verify before cycle 1)

1. SD-22 closure PR merged to develop. Run `git log origin/develop --oneline | head -5` and confirm the SD-22 closure commit is HEAD of develop.
2. `git branch tranche/5-1 origin/develop` — fresh dash branch off post-SD22-closure develop.
3. `codex-tranche-5` board exists. Run `hermes kanban list-boards` and confirm `codex-tranche-5` is reachable.
4. Classic PAT present at `~/.config/gh/.claude_gh_token` for ruleset/branch-protection admin.
5. Working tree clean: `git status --porcelain` returns empty.
6. Doctrines loaded: `identifier-discipline` and `wired-integration-discipline` skills are in the loop's skill list.
7. Build counter captured in `decisions.md` §3 from develop's `Cargo.toml` workspace version.
8. Artifact directories verified: `programs/codex/requirements/SD-23-character-mutation-and-wired-integration/artifacts/{epic_3,epic_4,epic_5,epic_6,epic_7}/` exist and are empty. The first cycle of each epic writes its receipt there.

## Per-cycle mechanics

### Cadence

- `/loop 60m` — supervisor tick every 60 minutes.
- One cycle per tick. The cycle either completes (`complete`) or returns to the cycle-backlog.
- `/batch` partitions cycles across worker batches by file-touch — one worker per file at a time.

### Eligibility

A kanban card is eligible for this cycle's tick when ALL of:

- Card is on board `codex-tranche-5`.
- Card status is `ready` (or `blocked` with the blocker explicitly resolved in the card's comments).
- Card's `touched_files` set does not overlap any in-flight cycle's `touched_files` set (file-touch partition).
- Card has a `--assignee` profile set (default-assignee footgun: never `default`, never `vanderspeigle`).
- Branch `tranche/5-1` is checked out and clean.

### Per-cycle steps

1. **Pull the eligible card from `codex-tranche-5`.** Read card's comments stream for prior-cycle context.
2. **Read the cycle's per-cycle story** in `epic-breakdown.md`. Pull the relevant acceptance criterion text.
3. **Run the four-check audit on the current diff** against the base branch:
   ```bash
   BASE_BRANCH=$(git merge-base HEAD origin/develop)
   # Check 1: forbidden tokens
   git diff --unified=0 "${BASE_BRANCH}...HEAD" \
     -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' \
     ':!**/__tests__/**' ':!**/*.test.ts' ':!**/*.test.tsx' ':!**/*.test.rs' \
     | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' \
     || echo 'OK_NO_TOKENS'
   # Check 2: empty event handlers
   git diff --unified=0 "${BASE_BRANCH}...HEAD" \
     -- 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' \
     | grep -nE 'onClick=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}|onClick=\{undefined' \
     || echo 'OK_NO_NOOP_HANDLERS'
   # Check 3: mock leaks
   git diff --unified=0 "${BASE_BRANCH}...HEAD" \
     -- 'apps/desktop/**/*.{ts,tsx,jsx,rs}' \
     ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE 'mockResolvedValue|mockReturnValue\(|vi\.mock\(|__mocks__' \
     || echo 'OK_NO_MOCK_LEAKS'
   # Check 4: "Would …" strings
   git diff --unified=0 "${BASE_BRANCH}...HEAD" \
     -- 'apps/desktop/**/*.{ts,tsx}' 'src/**/*.rs' \
     | grep -nE '"Would [^"]*"' \
     || echo 'OK_NO_WOULD_STRINGS'
   ```
4. **Capture audit output** in the kanban card's comments stream per `kanban-claude-code-execution-receipt/SKILL.md`. If any check fails, the cycle returns to the cycle-backlog with audit output as the failure reason; do NOT mark `complete`.
5. **Implement the cycle** per the per-cycle story in `epic-breakdown.md`. TDD is mandatory per repo `AGENTS.md` §"Non-Negotiable Rules."
6. **Re-run the four-check audit** on the updated diff.
7. **Commit with cycle SHA capture** and update `progress.md` with commit SHA + kanban card id.
8. **Mark card `complete`** via `hermes kanban complete --result "<commit-sha> | <criterion-number>"`.
9. **Self-heal loop continues** until the bundle's 33 acceptance criteria are satisfied.

### Hard stops

- A cycle that fails the four-check audit CANNOT mark `complete` — it returns to the cycle-backlog with audit output as the failure reason.
- A cycle that lands an identifier-discipline violation (e.g., `sd23_*` in source) CANNOT mark `complete`. The cycle reverts the violation, then re-runs.
- A cycle that claims `success: true` from an operation that did not actually do the work fails the audit and returns to the cycle-backlog.
- A cycle that introduces a stub without an entry in `../../governance/wired-integration-stubs-registry.md` fails the doctrine — it returns to the cycle-backlog.

### Post-mortem schema (per cycle, captured in `progress.md`)

```
### Cycle <number> — <epic-name> / Criterion <n>
- **Card ID:** t_<hex>
- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Audit result:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS
- **Acceptance criterion:** <verbatim from epic-breakdown.md>
- **Status:** complete | returned-to-backlog
- **Notes:** <judgment calls, deferred items, audit-exclusion requests>
```

### Self-heal

Open-ended self-healing cycles until the goal is met. Judgment calls logged in `progress.md` Notes for possible remediation in the next bundle.

## Per-cycle story index

Per-cycle stories are in `epic-breakdown.md` under each criterion. Each criterion has:
- The verbatim acceptance text.
- The files expected to be touched.
- The test contract.
- The audit-exclusion list (if any — for in-flight cleanup).

## Pre-cycle assumption checks

Before cycle 1 of any epic, verify the epic's prerequisites in `epic-breakdown.md` §"Dependencies" are met. A cycle whose prerequisites are not met returns to the cycle-backlog with a "prerequisites unmet" reason.

## Cross-references

- `scope-draft.md` — bundle scope and operator rulings
- `epic-breakdown.md` — 7 epics / 33 criteria / per-cycle story
- `decisions.md` — decision log
- `risks-and-open-questions.md` — latent risks and deferred questions
- `acceptance-and-verification.md` — closure-gate list
- `progress.md` — cycle log
- `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md` — per-cycle audit skill
- `~/.hermes/profiles/god-emporer/skills/devops/identifier-discipline/SKILL.md` — sibling doctrine skill
- `~/.hermes/profiles/god-emporer/skills/devops/kanban-claude-code-execution-receipt/SKILL.md` — receipt capture pattern
