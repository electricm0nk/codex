---
title: SD-N Loop Instruction Template (Workflow-Orchestrated Dispatch)
stc_id: GOV-LOOP-INSTRUCTION-TEMPLATE
canonical: true
owner: Todd Hintzmann
scope: universal
status: active
review_state: accepted
last_reviewed_at: 2026-07-22 (§2.0 added: dispatch-mechanism CLI-verification snag; §2.1 corollary added: kanban-inside-dispatch; §3 hardened: isolation:'worktree' is load-bearing not advisory; §6 added: hermes kanban --board flag-order gotcha; §7 added: proactive disk check between parallel waves — all from SD-25's actual launch/execution)
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
6. **Doctrine gates.** Do not write "verify skill X is loaded" unless skill X is actually installed. Checked precisely as of 2026-07-22: `identifier-discipline` and `wired-integration-discipline` exist as real files at `~/.hermes/profiles/<profile>/skills/devops/{identifier-discipline,wired-integration-discipline}/SKILL.md` — readable and useful when referenced by their raw path — but `hermes skills list` does **not** list either by name (they are not registered hub-installed skills, just raw profile files). Both distinctions matter: don't claim they're "not installed anywhere" (the files are real and load fine by path — SD-24's `docs/release/SD-24-beta-readiness-and-multiclass/missing_skills.md` first flagged this gap; SD-25 verified the files' continued existence and used them successfully by path throughout), and don't claim they're "installed skills" in the `hermes skills list` sense either (they aren't registered there). Cross-reference the doctrine docs `governance/identifier-discipline.md`/`governance/no-stub-mvp-doctrine.md` (repo-local) as the audit-content source of record either way — the inline grep commands in §6 are what actually enforces this, not any skill-loader check.
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

### 2.0 Verify the dispatch mechanism actually works before trusting it (added 2026-07-22, SD-25 launch snag)

**A bundle's `scripts/workflow-dispatch.sh` is a concurrency/tiering SPEC, not a script you should assume runs.** SD-25's own copy of this file invoked a subagent via `claude code --profile X --model Y --isolation worktree --task F` — a CLI surface that does not exist (`claude --help` has no `code` subcommand, no `--profile`/`--task` flags). This was carried forward verbatim from the template's worked example without ever being run once. As written, the script's dispatch step would have failed silently every time and the orchestrator's `sleep`/retry loop would have spun forever with zero cycles ever dispatching — discovered only because the launching session happened to check `claude --help` during Plan Mode, before the bundle's first real dispatch.

**Before trusting any bundle's `scripts/workflow-dispatch.sh` as a literal, runnable process:** run whatever CLI-invocation form the script assumes (e.g. `claude --help`) and confirm the flags exist. If they don't — which, as of 2026-07-22, they don't for the `claude code --profile/--model/--isolation/--task` form — the actual dispatch mechanism for this bundle is **the in-harness `Workflow` tool, invoked from a live session**, reading the script's `EPIC_PARALLEL`/`EPIC_SUBAGENT`/`PARALLEL_OVERRIDE`/`SUBAGENT_OVERRIDE` maps as its concurrency/tiering source of truth, not running the script as a standalone background process. Say this explicitly in the bundle's own `loop-instruction.md §2` and in the script's own header comment, rather than silently assuming a future reader will re-discover the same gap.

### 2.1 Execution boundary — the launching session is always the orchestrator, never the executor

**Rule:** the session that plans, scopes, or launches a bundle is the orchestrator. §6's per-cycle procedure — steps 1 through 9, especially step 3 ("implement the criterion TDD-style") — describes what happens **inside a dispatched `agent()`/`Workflow` call**, never what the orchestrating session does with its own `Edit`/`Write`/`Bash` tool calls. This holds with no exceptions: not for a "quick" one-file fix, not mid-investigation when the context is already loaded, not because Plan Mode approval already authorized the underlying change (approval authorizes the *work*, not a shortcut around the *mechanism*).

**The specific failure this codifies (SD-25 criterion 1.1, 2026-07-21):** the orchestrating session ran a cycle's RED check (an identifier-audit grep), found the real scope was ~15x larger than the cycle doc assumed (764 hits across 54 files, not one known file), and — because it was already mid-investigation with full context loaded — kept going and executed the rename directly via `sed`/`git mv`/`Edit` instead of stopping to re-dispatch. The operator had to interrupt mid-turn to redirect back to subagent orchestration. Discovering that a cycle's real scope differs from what the cycle doc assumed is common and expected (see §4) — it is a reason to **pause, record the corrected scope, and dispatch (or re-dispatch) an `agent()` call with that scope**, never a license to keep executing inline because the investigation already surfaced the fix.

**Self-check before any `Edit`/`Write`/`Bash`-that-mutates-a-file call while driving a bundle:** is the target path under the bundle's implementation trees (e.g. `apps/desktop/`, `apps/desktop/src-tauri/`, `src/`, `scripts/`) or otherwise part of a criterion's RED→GREEN work? If yes, stop — that call belongs inside a dispatched `agent()`, not here. The orchestrating session's own direct tool calls are reserved for: read-only investigation/scoping, authoring or correcting this bundle's own planning docs (`loop-instruction.md`, `epic-breakdown.md`, `decisions.md`, `cycles/*.md`), and git plumbing on those planning-doc commits — never on the shipped-code diff itself.

**Corollary: mint kanban done-receipts inside the dispatched agent, not from the orchestrating session's own `Bash` calls (added 2026-07-22).** On SD-25, the launching session's own attempt to run `hermes kanban create` directly from Bash was blocked by the permission-classifier as an unauthorized external-system write ("creating kanban tickets on the shared board... not clearly authorized by the current instruction"). The identical `hermes kanban create` command, issued from inside a dispatched `agent()` call as one step of that cycle's own per-cycle procedure (§6 step 8), ran without friction — because it's scoped, expected, receipt-only work the cycle's own task description already authorizes. Treat kanban-card creation the same as any other §6 per-cycle step: it happens inside the dispatched agent, never as a bare orchestrating-session Bash call, both because that's the correct division of labor (§2.1's own rule) and because it avoids unnecessary permission friction.

## 3. Per-epic parallel/sequential map

Fill in one row per epic, only after completing §4's path verification. A `parallel: yes` row is only valid if the epic's criteria touch genuinely disjoint files (verified, not assumed).

| Epic | Criteria | Parallel? | File-touch set (verified) | Gated on |
|---|---|---|---|---|
| `<N>` | `<x.1-x.n>` | yes/no | `<verified real paths>` | `<prior epic, or none>` |

When `parallel: yes`, the orchestrator script should give each parallel agent `isolation: 'worktree'` — agents are mutating a shared checkout and will otherwise step on each other's working-directory state even on disjoint files.

**This is not optional advice — treat a missing `isolation: 'worktree'` on any multi-agent `parallel()`/`Promise.all` call that mutates files as a bug in the dispatch script itself, not a minor omission.** On SD-25, two separate `parallel()` calls were dispatched without `isolation: 'worktree'` (both single-shot precheck/cleanup fixes, judged "small enough not to need it") — in both cases, one agent's in-flight, not-yet-committed edit was visible to its sibling in the same shared checkout, and the sibling (correctly, per its own instructions) halted rather than guess at ownership, reporting a false "new blocker" that required a full second dispatch to resolve. The failure mode is not silent corruption (both agents behaved safely) but wasted retries and false-alarm reports that look identical to a real regression until manually investigated. **Rule: any `parallel()`/`Promise.all` call with more than one agent that will `Edit`/`Write`/commit inside the SAME shared (non-worktree) checkout must either (a) pass `isolation: 'worktree'` to each, or (b) be re-scoped to run those agents serially instead.** A shared-checkout parallel dispatch is only safe when every agent in it is genuinely read-only.

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

**This procedure runs inside a dispatched `agent()`/`Workflow` call — see §2.1.** The orchestrating session never performs steps 1–9 itself with its own tool calls.

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
8. Mint the kanban card as a done-receipt, from inside this dispatched agent (per §2.1's corollary, per the bundle's assignee/daemon-hazard doctrine if one applies). **CLI gotcha (found twice independently on SD-25):** `hermes kanban create <title> --board <slug> ...` fails with `unrecognized arguments: --board <slug>` — `--board` is a global flag that must precede the subcommand: `hermes kanban --board <slug> create <title> ...`, then `hermes kanban --board <slug> complete <id>`.
9. Report: criterion, files touched, commit SHA(s), dual-audit results, RED→GREEN evidence, receipt path, kanban card ID, discoveries, next-cycle plan.

## 7. Self-heal posture

Carry forward the self-healable / non-self-healable split from the prior bundle's instance (dirty tree, single-token audit violation, unrelated test-setup breakage → self-heal inline; diverged branch needing manual rebase, two live processes on conflicting files, launch-gate dependency not actually merged → write a blocker and stop). No changes needed here — this part of the pattern held up.

**Proactive disk check between parallel-worktree waves (added 2026-07-22, upgraded from reactive to proactive after a real incident).** SD-24's carry-forward register already flagged that dense `parallel: yes` phases accumulate per-worktree `target/`-style build-cache directories, but treated it as a reactive "clean up when you notice" risk. On SD-25 this reached **100% disk usage mid-session** (12MB free, actively breaking in-flight `cargo test` runs in two agents simultaneously) before anyone checked — the reactive framing let it run all the way to a real outage rather than catching it early. **Rule: after every wave of `parallel: yes` cycles completes (not just when something breaks), run `df -h /` and `git worktree list`; if usage is climbing toward the disk's ceiling, prune merged worktrees and their `target`/build-cache directories immediately** — don't wait for a build to fail with `ENOSPC` first. Never remove a worktree that's still `locked` (an agent is actively using it); confirm via `git worktree list`'s lock annotation and via `git status --porcelain`/`git log <branch>..origin/<branch>` showing no unmerged, uncommitted work before removing anything.

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
