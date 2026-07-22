---
title: SD-N Loop Instruction Template (Workflow-Orchestrated Dispatch)
stc_id: GOV-LOOP-INSTRUCTION-TEMPLATE
canonical: true
owner: Todd Hintzmann
scope: universal
status: active
review_state: accepted
last_reviewed_at: 2026-07-21
canonical_source: ~/workspace/repos/codex/governance/loop-instruction-template.md (this file)
supersedes: (none — first issuance; retrofits the pattern used by SD-16 through SD-24's launch docs)
upstream_targets:
  - programs/codex/requirements/SD-N-.../loop-instruction.md (every future bundle authors from this template)
related_artifacts:
  - ./no-stub-mvp-doctrine.md (referenced by §6's dual-audit gate)
  - ~/workspace/governance/identifier-discipline.md (referenced by §6's dual-audit gate)
  - ~/workspace/governance/wired-integration-stubs-registry.md
  - docs/release/SD-24-beta-readiness-and-multiclass/loop-instruction.md (the retrofitted instance this template was extracted from)
date: 2026-07-21
---

# SD-N Loop Instruction Template — Workflow-Orchestrated Dispatch

## Origin

SD-24's `loop-instruction.md` was authored to launch via `/loop 1m /batch /goal ./loop-instruction.md`. On launch day, `/batch` turned out to be `disable-model-invocation: true` — it can only fire from a human typing it in the CLI, once, per invocation. That's incompatible with the document's own stated goal ("one launch command, run to closure," "no perceptible timer"). The actual working mechanism was the `Workflow` tool: an orchestrator script the model authors once, that fans agents out in parallel where files are genuinely disjoint and serializes where they aren't.

This template encodes that mechanism, plus seven other defects found in the same retrofit: unverified checklist commands, file paths that didn't match the real repo tree, a script bug in the dual-audit gate, references to skills that don't exist, an unresolved template placeholder, no concurrent-git-write protocol, and no standing orchestration/model-tiering section. Every future SD-N bundle should author its `loop-instruction.md` from this template, not from the prior bundle's copy-pasted instance.

## 0. Bundle at a glance

Fill in for the new bundle:

- **Branch:** `tranche/<N>-<M>`
- **Board:** kanban board slug (reused or new)
- **Cadence:** N/A under this template — see §2, dispatch is a Workflow script, not a timer loop
- **Epics / criteria:** `<count>` / `<count>`
- **First concrete build value:** captured in §8, not left as a template placeholder

## 1. Pre-launch checklist

**Rule:** every command in this section must be run for real during drafting, with its actual output pasted below the command, before the bundle is marked planning-ready. A command written from memory or assumption is not a verified precondition — SD-24 shipped `hermes kanban list-boards` (real subcommand: `boards`) and `hermes skills --profile god-emporer --list` (flag doesn't exist) as checklist items, and neither was ever run before publish.

1. **Kanban board reachable.** Run `hermes kanban boards`; confirm the target board slug is in the list. Paste output.
2. **Bundle branch is on origin, pushed and ahead of develop.** `git ls-remote --heads origin <branch>` + `git log origin/develop --oneline | head -5`. Paste output.
3. **Predecessor bundle's closure PR is merged to develop** (if this bundle has a Tier-1 launch-gate dependency). `git log origin/develop --oneline | head -5`, confirm the closure commit is HEAD or in HEAD's ancestry. Paste output.
4. **PAT present** at whatever path the kanban CLI expects. `test -f <path> && echo PAT_PRESENT`. Paste output.
5. **Working tree clean** on the bundle branch. `git status --porcelain | wc -l` — expect `0`. Paste output.
6. **Doctrine gates.** Do not write "verify skill X is loaded" unless skill X is actually installed (`hermes skills` / `~/.claude/skills/` / project `.claude/skills/`). As of 2026-07-21, `identifier-discipline` and `wired-integration-discipline` are **doctrine documents**, not installed skills — real files at `~/workspace/governance/identifier-discipline.md` and `governance/no-stub-mvp-doctrine.md` (repo-local), enforced via the inline grep commands in §6, not via a skill-loader check. State this plainly rather than gating on a command that doesn't exist.
7. **Build counter captured**, not left as a template placeholder. Read the real version source of truth for this repo (`apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json` — **not** root `Cargo.toml`, which stays pinned at `0.1.0`) and write the literal next value into §8, e.g. "develop is at `0.5.97`; this bundle's first concrete build is `0.5.98`." A bundle is not planning-ready with `0.5.<current_build>` still in the text.
8. **Artifact directories exist and are empty**, one per epic.

## 2. Orchestration mode

Standing policy (pulled from the operator's global model-selection tiering — state it here once per bundle rather than re-deriving it mid-launch):

- **Dispatch mechanism:** the `Workflow` tool, invoked as an orchestrator script authored against this bundle's epic-breakdown. Not `/loop /batch` — `/batch` requires a human to type it per invocation and cannot run unattended.
- **Default subagent model:** Sonnet (inherits session model unless overridden).
- **Tiering exceptions:**
  - Housekeeping (release notes, changelog, version bump, lint fixes) → Haiku.
  - Adversarial verification / final completeness scan / judge-panel steps → Opus (or Fable while it remains on-subscription for planning-tier work).
  - Everything else (real implementation, TDD cycles, audits, remediation) → Sonnet.
- **Concurrency shape:** decided explicitly per epic in §3 below, at authoring time — not derived live by whichever model launches the bundle.

## 3. Per-epic parallel/sequential map

Fill in one row per epic, only after completing §4's path verification. A `parallel: yes` row is only valid if the epic's criteria touch genuinely disjoint files (verified, not assumed).

| Epic | Criteria | Parallel? | File-touch set (verified) | Gated on |
|---|---|---|---|---|
| `<N>` | `<x.1-x.n>` | yes/no | `<verified real paths>` | `<prior epic, or none>` |

When `parallel: yes`, the orchestrator script should give each parallel agent `isolation: 'worktree'` — agents are mutating a shared checkout and will otherwise step on each other's working-directory state even on disjoint files.

## 4. File-touch verification (required before §3 is filled in)

Run `ls` / `find` on every path named in this document, `content-unit-inventory.md`, and `technical-design.md`. Confirm it exists as written. If it doesn't:

- Find the real analogous file/module.
- Correct the path in this document before publish (don't ship a known-wrong path and rely on cycle agents to discover the mismatch later).
- If the mismatch reflects a genuine design decision still open, note it explicitly rather than silently assuming either the plan or the repo is right.

SD-24's instance shipped three unverified mismatches: per-class files assumed (`class_fighter.rs`) where the repo uses per-corpus tables (`class_tables.rs`); a flat `equipment/*.rs` assumed where the repo is per-corpus (`equipment_tables.rs` + `equipment_data/`); and a frontend runtime file located under `src-tauri/src/` in the doc when the real file is under `src/` (backend/frontend directory mixup).

## 5. Concurrent-write protocol

Every cycle that commits and pushes to the shared bundle branch must use this exact retry protocol — do not invent a per-bundle variant:

```bash
git fetch origin <branch> && git rebase origin/<branch> && git push origin HEAD:<branch>
```

On non-fast-forward rejection, repeat up to 5 times. If it still fails after 5 attempts, stop and report a `CLAIM-EXISTS` blocker — do not force-push. This applies to both the code commit and any shared-state file every cycle touches (e.g. `progress.md`): re-fetch and re-read the file's current content immediately before editing it, so a concurrent cycle's append isn't clobbered.

## 6. Per-cycle procedure

1. Ensure the working tree/worktree is based on the latest bundle branch (§5's fetch+rebase).
2. `BASE_BRANCH=$(git merge-base HEAD origin/develop)` — **define this before either grep block**, not between them (SD-24 shipped a version where the wired-integration grep referenced `${BASE_BRANCH}` before the identifier-discipline grep that defined it).
   ```bash
   BASE_BRANCH=$(git merge-base HEAD origin/develop)

   # Identifier audit — bundle-tag leaks in diff
   # NOTE: trailing \b is deliberately omitted -- \b never matches between `_` and a
   # following word character, so a trailing \b silently fails to catch real identifiers
   # like `sd19_class_catalog` and only matches a bare standalone token. Found live during
   # SD-24 (2026-07-21): the buggy pattern returned 0 hits against a repo that actually had
   # 6 bundle-tagged modules. Do not add the trailing \b back.
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'

   # Wired-integration four-check audit — forbidden patterns in shipping code
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
   ```
3. Implement the criterion TDD-style: RED → confirm it fails for the intended reason → GREEN → run the relevant test suite.
4. Re-run the dual-audit gate on the final diff; both must show `OK_*`. A single-token violation is self-healable inline; re-audit and continue.
5. Write the cycle receipt to `artifacts/<epic>/<cycle-id>_cycle_receipt.md` (schema unchanged from prior bundles).
6. Commit, then push via §5's retry protocol.
7. Update the shared progress doc in place via §5's protocol.
8. Mint the kanban card as a done-receipt (per the bundle's assignee/daemon-hazard doctrine, if one applies).
9. Report: criterion, files touched, commit SHA(s), dual-audit results, RED→GREEN evidence, receipt path, kanban card ID, discoveries, next-cycle plan.

## 7. Self-heal posture

Carry forward the self-healable / non-self-healable split from the prior bundle's instance (dirty tree, single-token audit violation, unrelated test-setup breakage → self-heal inline; diverged branch needing manual rebase, two live processes on conflicting files, launch-gate dependency not actually merged → write a blocker and stop). No changes needed here — this part of the pattern held up.

## 8. Placeholder-resolution checklist (final gate before "planning-ready")

Grep the whole bundle directory for `<...>`-style placeholders and template markers (e.g. `0.5.<current_build>`) before publish:

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-N-<slug>/*.md
```

Every match must be resolved to a real value, or explicitly justified as intentionally deferred (e.g., "filled in by Epic 8's cycle, not at authoring time").

## Cross-references

- `../docs/release/SD-24-beta-readiness-and-multiclass/loop-instruction.md` — the retrofitted instance this template was extracted from; useful as a worked example, not as a copy-paste source (it still contains SD-24-specific epic content).
- `./no-stub-mvp-doctrine.md`, `~/workspace/governance/identifier-discipline.md` — the two doctrine docs §6's dual-audit gate enforces inline.
- `~/workspace/governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions.
