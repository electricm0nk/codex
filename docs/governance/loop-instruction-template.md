---
title: SD-N Loop Instruction Template (Workflow-Orchestrated Dispatch)
stc_id: GOV-LOOP-INSTRUCTION-TEMPLATE
canonical: true
owner: Todd Hintzmann
scope: universal
status: active
review_state: accepted
last_reviewed_at: 2026-07-22
canonical_source: ~/workspace/repos/codex/docs/governance/loop-instruction-template.md (this file)
upstream_targets:
  - programs/codex/requirements/SD-N-.../loop-instruction.md (every future bundle authors from this template)
related_artifacts:
  - ./no-stub-mvp-doctrine.md (referenced by §6's dual-audit gate)
  - ./wired-integration-stubs-registry.md
date: 2026-07-22
---

# SD-N Loop Instruction Template — Workflow-Orchestrated Dispatch

Every SD-N bundle authors its `loop-instruction.md` from this template, not from a prior bundle's copy-pasted instance. This file states the current, desired dispatch procedure only — it is not a changelog and does not narrate why any rule exists.

## 0. Bundle at a glance

Fill in for the new bundle:

- **Branch:** `tranche/<N>-<M>`
- **Board:** kanban board slug (reused or new)
- **Cadence:** N/A — dispatch is a live `Workflow` session, not a timer loop (see §2)
- **Epics / criteria:** `<count>` / `<count>`
- **First concrete build value:** captured in §8, not left as a template placeholder

## 1. Pre-launch checklist

Every command in this section must be run for real during drafting, with its actual output pasted below the command, before the bundle is marked planning-ready. A command written from memory or assumption is not a verified precondition.

1. **Kanban board reachable.** Run `hermes kanban boards`; confirm the target board slug is in the list. Paste output.
2. **Bundle branch is on origin, pushed and ahead of develop.** `git ls-remote --heads origin <branch>` + `git log origin/develop --oneline | head -5`. Paste output.
3. **Predecessor bundle's closure PR is merged to develop** (if this bundle has a Tier-1 launch-gate dependency). `git log origin/develop --oneline | head -5`, confirm the closure commit is HEAD or in HEAD's ancestry. Paste output.
4. **PAT present** at whatever path the kanban CLI expects. `test -f <path> && echo PAT_PRESENT`. Paste output.
5. **Working tree clean** on the bundle branch. `git status --porcelain | wc -l` — expect `0`. Paste output.
6. **Doctrine gates.** `identifier-discipline` and `wired-integration-discipline` exist as real files at `~/.hermes/profiles/<profile>/skills/devops/{identifier-discipline,wired-integration-discipline}/SKILL.md`, loadable by path — but are not registered in `hermes skills list` (raw profile files, not hub-installed skills). Cross-reference the doctrine docs `docs/governance/no-stub-mvp-doctrine.md` and `docs/doctrine-external/identifier-discipline.md` as the audit-content source of record; the inline grep commands in §6 are what actually enforces this, not any skill-loader check.
7. **Build counter captured**, not left as a template placeholder. Read the real version source of truth for this repo (`apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json` — **not** root `Cargo.toml`, which stays pinned at `0.1.0`) and write the literal next value into §8, e.g. "develop is at `0.5.98`; this bundle's first concrete build is `0.5.99`." A bundle is not planning-ready with `0.5.<current_build>` still in the text.
8. **Artifact directories exist and are empty**, one per epic.

## 2. Orchestration mode

Standing policy (pulled from the operator's global model-selection tiering — state it here once per bundle rather than re-deriving it mid-launch):

- **Dispatch mechanism:** the in-harness `Workflow` tool, invoked from a live session, reading the bundle's `scripts/workflow-dispatch.sh` as a concurrency/tiering **spec** — not `/loop /batch` (requires a human to type it per invocation, cannot run unattended) and not `scripts/workflow-dispatch.sh` as a standalone background process. Before trusting any bundle's `scripts/workflow-dispatch.sh` as containing a runnable subagent-invocation form, verify that form actually exists (e.g. `claude --help`) — if it doesn't, dispatch via `Workflow`/`agent()` calls from the live session, reading the script's `EPIC_PARALLEL`/`EPIC_SUBAGENT`/`PARALLEL_OVERRIDE`/`SUBAGENT_OVERRIDE` maps as the concurrency/tiering source of truth.
- **Default subagent model:** Sonnet (inherits session model unless overridden).
- **Tiering exceptions:**
  - Housekeeping (release notes, changelog, version bump, lint fixes) → Haiku.
  - Adversarial verification / final completeness scan / judge-panel steps → Opus (or Fable while it remains on-subscription for planning-tier work).
  - Everything else (real implementation, TDD cycles, audits, remediation) → Sonnet.
- **Concurrency shape:** decided explicitly per epic in §3 below, at authoring time — not derived live by whichever model launches the bundle.

### 2.1 Agent environment setup

Every dispatched agent should have `RETRO_ACTOR` set to its role name (e.g. `RETRO_ACTOR=apg-acg-feats`). Without this, the retrospective log's by-actor breakdown falls back to opaque worktree directory names, which become meaningless after the run ends. The `scripts/retro.py` tool resolves actor identity in this order: `--actor` flag → `$RETRO_ACTOR` → worktree name → git config. For per-role agents running in predictable worktrees, `RETRO_ACTOR` in the dispatch call preserves role identity for retrospective analysis.

### 2.2 Execution boundary — the launching session is always the orchestrator, never the executor

The session that plans, scopes, or launches a bundle is the orchestrator. §6's per-cycle procedure — steps 1 through 9, especially step 3 ("implement the criterion TDD-style") — describes what happens **inside a dispatched `agent()`/`Workflow` call**, never what the orchestrating session does with its own `Edit`/`Write`/`Bash` tool calls. This holds with no exceptions: not for a "quick" one-file fix, not mid-investigation when the context is already loaded, not because Plan Mode approval already authorized the underlying change (approval authorizes the *work*, not a shortcut around the *mechanism*).

Discovering that a cycle's real scope differs from what the cycle doc assumed is common and expected (see §4) — it is a reason to **pause, record the corrected scope, and dispatch (or re-dispatch) an `agent()` call with that scope**, never a license to keep executing inline because the investigation already surfaced the fix.

**Self-check before any `Edit`/`Write`/`Bash`-that-mutates-a-file call while driving a bundle:** is the target path under the bundle's implementation trees (e.g. `apps/desktop/`, `apps/desktop/src-tauri/`, `src/`, `scripts/`) or otherwise part of a criterion's RED→GREEN work? If yes, stop — that call belongs inside a dispatched `agent()`, not here. The orchestrating session's own direct tool calls are reserved for: read-only investigation/scoping, authoring or correcting this bundle's own planning docs (`loop-instruction.md`, `epic-breakdown.md`, `decisions.md`, `cycles/*.md`), and git plumbing on those planning-doc commits — never on the shipped-code diff itself.

**Corollary:** mint kanban done-receipts inside the dispatched agent, not from the orchestrating session's own `Bash` calls. Kanban card creation is one more §6 per-cycle step; it happens inside the dispatched agent's scoped task, never as a bare orchestrating-session Bash call.

## 3. Per-epic parallel/sequential map

Fill in one row per epic, only after completing §4's path verification. A `parallel: yes` row is only valid if the epic's criteria touch genuinely disjoint files (verified, not assumed).

| Epic | Criteria | Parallel? | File-touch set (verified) | Gated on |
|---|---|---|---|---|
| `<N>` | `<x.1-x.n>` | yes/no | `<verified real paths>` | `<prior epic, or none>` |

When `parallel: yes`, every parallel agent that mutates files must get `isolation: 'worktree'` — agents are mutating a shared checkout and will otherwise step on each other's working-directory state even on disjoint files. This is load-bearing, not advisory: any `parallel()`/`Promise.all` call with more than one agent that will `Edit`/`Write`/commit inside the SAME shared (non-worktree) checkout must either (a) pass `isolation: 'worktree'` to each, or (b) be re-scoped to run those agents serially instead. A shared-checkout parallel dispatch is only safe when every agent in it is genuinely read-only.

## 4. File-touch verification (required before §3 is filled in)

Run `ls` / `find` on every path named in this document, `content-unit-inventory.md`, and `technical-design.md`. Confirm it exists as written. If it doesn't:

- Find the real analogous file/module.
- Correct the path in this document before publish (don't ship a known-wrong path and rely on cycle agents to discover the mismatch later).
- If the mismatch reflects a genuine design decision still open, note it explicitly rather than silently assuming either the plan or the repo is right.

## 5. Concurrent-write protocol

Every cycle that commits and pushes to the shared bundle branch must use this exact retry protocol — do not invent a per-bundle variant:

```bash
git fetch origin <branch> && git rebase origin/<branch> && git push origin HEAD:<branch>
```

On non-fast-forward rejection, repeat up to 5 times. If it still fails after 5 attempts, stop and report a `CLAIM-EXISTS` blocker — do not force-push. This applies to both the code commit and any shared-state file every cycle touches (e.g. `progress.md`): re-fetch and re-read the file's current content immediately before editing it, so a concurrent cycle's append isn't clobbered.

## 6. Per-cycle procedure

**This procedure runs inside a dispatched `agent()`/`Workflow` call — see §2.2.** The orchestrating session never performs steps 1–9 itself with its own tool calls.

1. Ensure the working tree/worktree is based on the latest bundle branch (§5's fetch+rebase).
2. `BASE_BRANCH=$(git merge-base HEAD origin/develop)` — define this before either grep block below, not between them.
   ```bash
   BASE_BRANCH=$(git merge-base HEAD origin/develop)

   # Identifier audit — bundle-tag leaks in diff
   # Trailing \b is deliberately omitted -- \b never matches between `_` and a
   # following word character, so a trailing \b silently fails to catch real
   # identifiers like `sd19_class_catalog`. Do not add it back.
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'

   # Wired-integration four-check audit — forbidden patterns in shipping code
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
   ```
3. Implement the criterion TDD-style: RED → confirm it fails for the intended reason → GREEN → run the relevant test suite.
4. Re-run the dual-audit gate on the final diff; both must show `OK_*`. A single-token violation is self-healable inline; re-audit and continue.
5. Write the cycle receipt to `artifacts/<epic>/<cycle-id>_cycle_receipt.md` (schema in §7).
6. Commit, then push via §5's retry protocol.
7. Update the shared progress doc in place via §5's protocol.
8. Mint the kanban card as a done-receipt, from inside this dispatched agent (per §2.2's corollary; per the bundle's assignee/daemon-hazard doctrine if one applies). `--board` is a global flag that must precede the subcommand: `hermes kanban --board <slug> create <title> ...`, then `hermes kanban --board <slug> complete <id>`.
9. Report: criterion, files touched, commit SHA(s), dual-audit results, RED→GREEN evidence, receipt path, kanban card ID, discoveries, next-cycle plan.

## 7. Per-cycle receipt schema

```markdown
# Cycle <cycle-id> — <epic-name> / Criterion <n>

- **Card ID:** t_<hex>
- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violation list>
- **Wired-integration audit result:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS / <violation list>
- **Acceptance criterion:** <verbatim from epic-breakdown.md>
- **Status:** complete | returned-to-backlog | DISCOVERED-forked
- **Notes:** <judgment calls, deferred items, audit-exclusion requests>
- **Discovery forwards:** <list of ## DISCOVERED entries added>
- **Next-cycle plan:** <what the next cycle picks up>
```

## 8. Self-heal posture

- **Self-healable (resolve inline, exit GREEN):** dirty tree, single-token audit violation, unrelated test-setup breakage, build-counter out of sync, `## DISCOVERED` duplicates.
- **Non-self-healable (write `## Open blockers`, exit FAIL):** working tree diverged from the bundle branch needing manual rebase; two live cycles on conflicting files; a launch-gate dependency not actually merged; `## DISCOVERED` queue > 10 entries; RED → GREEN not preserved in the cycle receipt; a cycle finds `success: true` from a fake operation, an inline mock in a shipping module, or a "Would …" string in shipping code.

**Disk usage — check proactively, not reactively.** After every wave of `parallel: yes` cycles completes (not just when something breaks), run `df -h /` and `git worktree list`; if usage is climbing toward the disk's ceiling, prune merged worktrees and their `target`/build-cache directories immediately — don't wait for a build to fail with `ENOSPC` first. Never remove a worktree that's still `locked` (an agent is actively using it); confirm via `git worktree list`'s lock annotation and via `git status --porcelain`/`git log <branch>..origin/<branch>` showing no unmerged, uncommitted work before removing anything.

## 9. Placeholder-resolution checklist (final gate before "planning-ready")

Grep the whole bundle directory for `<...>`-style placeholders and template markers (e.g. `0.5.<current_build>`) before publish:

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-N-<slug>/*.md
```

Every match must be resolved to a real value, or explicitly justified as intentionally deferred (e.g., "filled in by Epic 8's cycle, not at authoring time").

## Cross-references

- `./no-stub-mvp-doctrine.md`, `docs/doctrine-external/identifier-discipline.md` — the two doctrine docs §6's dual-audit gate enforces inline.
- `./wired-integration-stubs-registry.md` — operator-granted stub exceptions.
- `docs/release/template/template.md` — the sibling template every bundle's own `README.md` (folder index) is authored from. Distinct scope: that template covers the release-folder's file index and bundle-snapshot table; this template covers the per-cycle dispatch procedure. Both must agree on the dispatch mechanism (`Workflow` tool, not `/loop /batch`) — if one changes, check the other.
