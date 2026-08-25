---
title: SD-N Workflow Instruction Template (Workflow-Orchestrated Dispatch)
stc_id: GOV-WORKFLOW-INSTRUCTION-TEMPLATE
canonical: true
owner: Todd Hintzmann
scope: universal
status: active
review_state: accepted
last_reviewed_at: 2026-07-22
canonical_source: ~/workspace/repos/codex/docs/governance/workflow-instruction-template.md (this file)
upstream_targets:
  - programs/codex/requirements/SD-N-.../workflow-instruction.md (every future bundle authors from this template)
related_artifacts:
  - ./no-stub-mvp-doctrine.md (referenced by §6's dual-audit gate)
  - ./wired-integration-stubs-registry.md
date: 2026-07-22
---

# SD-N Workflow Instruction Template — Workflow-Orchestrated Dispatch

> **Naming note:** this template was `loop-instruction-template.md` before 2026-08-22. Every
> bundle launched before that date still names its per-cycle file `loop-instruction.md`; those
> files are not renamed retroactively. Every bundle launched on or after 2026-08-22 names it
> `workflow-instruction.md`, per this template.

Every SD-N bundle authors its `workflow-instruction.md` from this template, not from a prior bundle's copy-pasted instance. This file states the current, desired dispatch procedure only — it is not a changelog and does not narrate why any rule exists.

## 0. Bundle at a glance

Fill in for the new bundle:

- **Branch:** `tranche/<N>-<M>`
- **Board:** kanban board slug (reused or new)
- **Cadence:** N/A — dispatch is a live `Workflow` session, not a timer loop (see §2)
- **Epics / criteria:** `<count>` / `<count>`
- **First concrete build value:** captured in §8, not left as a template placeholder

## 1. Pre-launch checklist

Every command in this section must be run for real during drafting, with its actual output pasted below the command, before the bundle is marked planning-ready. A command written from memory or assumption is not a verified precondition.

1. **Kanban board reachable.** Run `hermes kanban boards`; confirm the target board slug is in the list. Paste output. **Note (added 2026-08-15, `SD-31-corpus-closure-grind` launch-readiness remediation Step 5, drift D11):** the Hermes kanban board was retired 2026-08-01 (`SD-30-class-feature-archetype-bundle/decisions.md` Decision 14a) — SD-30 and its successors use a local-file `kanban.md` paired with `progress.md` instead, with cycle dispatch reading `kanban.md` at the top of each tick. This checklist item, and the two below that assume a live Hermes board, are this template's own stale surface; a bundle drafted after 2026-08-01 substitutes the local-`kanban.md` equivalent (confirm the file exists and is readable) rather than running the command literally.
2. **Bundle branch is on origin, pushed and ahead of develop.** `git ls-remote --heads origin <branch>` + `git log origin/develop --oneline | head -5`. Paste output.
3. **Predecessor bundle's closure PR is merged to develop** (if this bundle has a Tier-1 launch-gate dependency). `git log origin/develop --oneline | head -5`, confirm the closure commit is HEAD or in HEAD's ancestry. Paste output.
4. **PAT present** at whatever path the kanban CLI expects. `test -f <path> && echo PAT_PRESENT`. Paste output.
5. **Working tree clean** on the bundle branch. `git status --porcelain | wc -l` — expect `0`. Paste output.
6. **Doctrine gates.** `identifier-discipline` and `wired-integration-discipline` exist as real files at `~/.hermes/profiles/<profile>/skills/devops/{identifier-discipline,wired-integration-discipline}/SKILL.md`, loadable by path — but are not registered in `hermes skills list` (raw profile files, not hub-installed skills). Cross-reference the doctrine docs `docs/governance/no-stub-mvp-doctrine.md` and `docs/doctrine-external/identifier-discipline.md` as the audit-content source of record; the inline grep commands in §6 are what actually enforces this, not any skill-loader check. **Note (added 2026-08-15, drift D11, same retirement as §1 above):** the Hermes profile-path check is moot post-2026-08-01 for the same reason — the doctrine docs (`docs/governance/no-stub-mvp-doctrine.md`, `docs/doctrine-external/identifier-discipline.md`) and the inline grep commands remain the live gate; skip the `~/.hermes/` path check for any bundle drafted after the retirement.
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

Every dispatched agent must also claim its per-cycle `CARGO_TARGET_DIR` immediately after establishing it: `mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"` (see commit c8ff0885, `scripts/reclaim.sh`). Between builds, no passive liveness signal protects a live agent's directory; without the claim file, a 27G target directory is protected only by an age heuristic, and a sibling's reclaim sweep can silently delete work in progress.

### 2.2 Execution boundary — the launching session is always the orchestrator, never the executor

The session that plans, scopes, or launches a bundle is the orchestrator. §6's per-cycle procedure — steps 1 through 9, especially step 3 ("implement the criterion TDD-style") — describes what happens **inside a dispatched `agent()`/`Workflow` call**, never what the orchestrating session does with its own `Edit`/`Write`/`Bash` tool calls. This holds with no exceptions: not for a "quick" one-file fix, not mid-investigation when the context is already loaded, not because Plan Mode approval already authorized the underlying change (approval authorizes the *work*, not a shortcut around the *mechanism*).

Discovering that a cycle's real scope differs from what the cycle doc assumed is common and expected (see §4) — it is a reason to **pause, record the corrected scope, and dispatch (or re-dispatch) an `agent()` call with that scope**, never a license to keep executing inline because the investigation already surfaced the fix.

**Self-check before any `Edit`/`Write`/`Bash`-that-mutates-a-file call while driving a bundle:** is the target path under the bundle's implementation trees (e.g. `apps/desktop/`, `apps/desktop/src-tauri/`, `src/`, `scripts/`) or otherwise part of a criterion's RED→GREEN work? If yes, stop — that call belongs inside a dispatched `agent()`, not here. The orchestrating session's own direct tool calls are reserved for: read-only investigation/scoping, authoring or correcting this bundle's own planning docs (`workflow-instruction.md`, `epic-breakdown.md`, `decisions.md`, `cycles/*.md`), and git plumbing on those planning-doc commits — never on the shipped-code diff itself.

**Corollary:** mint kanban done-receipts inside the dispatched agent, not from the orchestrating session's own `Bash` calls. Kanban card creation is one more §6 per-cycle step; it happens inside the dispatched agent's scoped task, never as a bare orchestrating-session Bash call.

**Dispatch first, report second.** SD-31 lost four full stalls this way — twice the operator had
to say *"you look idle"* / *"you stopped working again"* before work resumed. A wave finished, the
orchestrating session wrote a summary, and the turn ended without dispatching the next phase. The
summary **feels** like the deliverable. It is not. Before ending any turn while the bundle has
ready, undispatched work, dispatch it first — the summary then describes something that already
exists, rather than substituting for it.

### 2.3 Retrospective event logging (every cycle)

Every dispatched cycle emits retro events during the work itself, not just a written summary at
the end — per `AGENTS.md`'s "Retrospective Logging" discipline. Git records what landed; it says
nothing about what nearly landed wrong or who caught it, and nothing survives the run except this
log. SD-31's 1,940-event log is the reason its own retrospective
(`docs/retro/sd31-retrospective.md`) is grounded in numbers instead of recollection.

- **When you catch an error, hit an incident, defer work, or redo something**, emit the event via
  `scripts/retro.py` at the moment it happens — not batched at cycle end. A "batch it at the end"
  habit is how events silently never get written.
- **Correction:** `scripts/retro.py correction --subject <who-was-wrong> --claimed <value> --actual <value> --verified-by <command-or-check>`. `--verified-by` is required — an unverified correction is a competing assertion, not a finding.
- **Incident / deferral / rework:** `scripts/retro.py <type> ...` — run `python3 scripts/retro.py help <type>` for that type's required fields.
- **`RETRO_ACTOR` must already be set** (§2.1) so the by-actor breakdown resolves to a real role
  name instead of an opaque worktree path once the run ends.
- Full vocabulary and field contract: `docs/retro/schema.json`. The tool builds its own CLI from
  that schema, so it cannot drift from what it accepts.

### 2.4 Creating the Workflow script

The bundle's dispatch is a script passed to the `Workflow` tool from the live orchestrating
session — plain JavaScript (no TypeScript syntax), not a shell script and not a `/loop`
invocation. If the bundle has a `scripts/workflow-dispatch.sh`, it is read as **data**: its
`EPIC_PARALLEL` / `EPIC_SUBAGENT` / `PARALLEL_OVERRIDE` / `SUBAGENT_OVERRIDE` maps supply the
concurrency and tiering values the script below plugs in. It is never executed as a standalone
process.

Every `Workflow` script for this project's bundles follows the same shape:

1. `export const meta = { name, description, phases }` — one `phases` entry per epic/gate row in
   §3's parallel/sequential map, same titles, so the progress display groups cleanly.
2. One `phase('<epic/gate title>')` call per §3 row, fired in the gated order that table states —
   never out of order, even when two epics are each individually parallel-safe.
3. Inside a phase, default to `pipeline()` for a chain of cycles within one criterion (RED → GREEN
   → audit → receipt is a chain, not independent work); use `parallel()` only when §3 marked that
   epic `parallel: yes` for genuinely disjoint files, and then every agent in that call gets
   `isolation: 'worktree'` per §3's rule.
4. Every `agent()` call sets `model` explicitly, per §2's tiering table (Sonnet for build/
   integration, Opus only for adversarial verification, Haiku for housekeeping). **Never omit
   `model`** — an omitted `model` silently inherits the orchestrating session's own model. SD-31
   wave 18 did this and burned 97% of a week's Opus quota on six inherited-Opus build lanes in
   three hours.
5. Every `agent()` prompt for a criterion cycle embeds §6's procedure verbatim (or points at this
   file plus the specific criterion) — the dispatched agent starts with zero context of this
   bundle.

**Worked skeleton** (adapt phase titles/counts to this bundle's own §3 table — never copy
another bundle's script verbatim any more than you'd copy its `workflow-instruction.md`
verbatim, per `docs/governance/STC-Skill-Creation.md` Rule 1):

```javascript
export const meta = {
  name: 'sd-nn-dispatch',
  description: '<bundle one-line>',
  phases: [
    { title: 'Epic 1 — <name>' },
    { title: 'Epic 2 — <name>' },
  ],
}

phase('Epic 1 — <name>')
const epic1Results = await pipeline(
  criteriaForEpic1,                          // [{id, prompt}, ...] from epic-breakdown.md
  c => agent(cycleProcedurePrompt(c), { model: 'sonnet', phase: 'Epic 1 — <name>' }),
)

phase('Epic 2 — <name>')
// parallel: yes per §3 — genuinely disjoint files, each agent gets its own worktree
const epic2Results = await parallel(
  criteriaForEpic2.map(c => () =>
    agent(cycleProcedurePrompt(c), { model: 'sonnet', phase: 'Epic 2 — <name>', isolation: 'worktree' })),
)
```

## 2.5 A dispatched agent is never resumed — never end a turn waiting

**A dispatched `agent()` call gets exactly one turn.** Nothing wakes it up: there is no monitor, no
completion notification, and no re-invocation. An agent that backgrounds a long command and ends its
turn to "resume when it reports" loses its entire cycle, and the orchestrator gets a status line
instead of work.

This has real cost. In SD-32 two consecutive lanes returned exactly this and landed **zero**
commits between them:

> *"I'll end this turn now and wait for the monitor's completion notification before continuing."*
> *"Waiting for the reach-gate verification background process to finish before completing the
> cycle. I'll resume once it reports."*

Both were caught only because the orchestrator checked the repo rather than reading the summary —
`git log` showed no commits and the target files were untouched.

**Put this in every dispatch prompt**, alongside the environment block:

- Wait for slow work **inside** the turn — run it in the foreground, or poll a background job in a
  loop with sleeps between checks. Do not end the turn expecting to be re-invoked.
- **Scope test runs.** On a contended machine a full unscoped `cargo test --locked --no-fail-fast`
  over hundreds of binaries may never finish. Name the targeted binaries/modules the change touches,
  plus the workspace-level suites, and say explicitly which sweeps *not* to run.
- If something genuinely will not finish, **report what was observed and why**, and commit the work
  anyway. A cycle that lands its change and reports one unfinished suite is a success; a cycle that
  lands nothing while waiting is a total loss.
- **Commit and push before ending the turn**, always — even for a partial result.

**Orchestrator's side of the control:** never accept a lane's final message as evidence that work
landed. Check `git log` and the target files. A return value that describes an intention rather than
an outcome is a stall, not a result.

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

1. Ensure the working tree/worktree is based on the latest bundle branch (§5's fetch+rebase), **and
   verify the base is real rather than assuming it.** SD-31 wrote a prose warning about wrong-base
   worktrees into every dispatch prompt from wave 15 onward; it still fired 27 times, because a
   warning in a prompt is not a control. Run this and stop if it fails, before doing anything else:
   ```bash
   test -d docs && test -d data && test -d scripts \
     || { echo 'WRONG BASE — reset before continuing'; exit 1; }
   ```
   On failure, `git reset --hard <pinned bundle-branch SHA>` and re-verify. If the bundle has
   accumulated spent `site-publish/*` (or similarly-shaped throwaway) branches, delete them once
   merged — SD-31's actual fix was removing the branches that could be wrongly selected as a base,
   not a better-worded warning.
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
8. Mint the kanban card as a done-receipt, from inside this dispatched agent (per §2.2's corollary; per the bundle's assignee/daemon-hazard doctrine if one applies). `--board` is a global flag that must precede the subcommand: `hermes kanban --board <slug> create <title> ...`, then `hermes kanban --board <slug> complete <id>`. **Note (added 2026-08-15, drift D11, same retirement as §1 above):** post-2026-08-01, "mint the kanban card" means editing the card's row in the bundle's local `kanban.md` (`Status` → `COMPLETE`) and appending the receipt to `progress.md` — there is no live Hermes board to mint a card on.
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

**A `## Open blockers` entry is a request for an operator ruling — not a disposition, and never a closure path** (`docs/governance/blocker-closure-doctrine.md`). Filing one **pauses the bundle**. It does not authorise any later cycle to proceed past the blocked card, and only an operator ruling may move blocked scope into a forward-scope register.

A blocker standing between the bundle and 100% of its Definition of Done has exactly **two** dispositions:

1. **Clear it.** Do the work. If it is bigger than one cycle, decompose it and run the cycles — a large blocker is a sequencing problem, not an exemption.
2. **Raise your hand.** Escalate to the operator, naming what blocks you, what you already tried, and the specific ruling, write scope, or precondition you need. Then stop and wait.

"Filed with a named owner", "forwarded to a successor bundle", "deferred with reason", and "out of scope for this cycle" are **not** dispositions for such a blocker — they are ways of writing down that the bundle is not done. A cycle that correctly refuses to write outside its granted scope has not failed; it has hit a blocker only the operator can clear, and the right move is to prepare the exact change, escalate, and wait. Distinguish this from a planned **capability deferral** (`docs/governance/deferral-revisit-doctrine.md`) by one test: **was this scope in the Definition of Done at launch?** If yes, it is a blocker.

**Disk usage — check proactively, not reactively.** After every wave of `parallel: yes` cycles completes (not just when something breaks), run `df -h /` and `git worktree list`; if usage is climbing toward the disk's ceiling, prune merged worktrees and their `target`/build-cache directories immediately — don't wait for a build to fail with `ENOSPC` first. Never remove a worktree that's still `locked` (an agent is actively using it); confirm via `git worktree list`'s lock annotation and via `git status --porcelain`/`git log <branch>..origin/<branch>` showing no unmerged, uncommitted work before removing anything.

## 9. Placeholder-resolution checklist (final gate before "planning-ready")

Grep the whole bundle directory for `<...>`-style placeholders and template markers (e.g. `0.5.<current_build>`) before publish:

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-N-<slug>/*.md
```

Every match must be resolved to a real value, or explicitly justified as intentionally deferred (e.g., "filled in by Epic 8's cycle, not at authoring time").

## 10. Epic wrap-up (fires after every epic)

Lightweight, and distinct from §11's bundle-final closure — this runs at the end of **every**
epic, not just the last one:

1. **Retro summary for the epic's work window.** `scripts/retro.py summary --since <epic-start-date> --json` (or a tighter window if the epic's actual start is known more precisely). Read it — don't just run it and move on. Append a short "what the retro log shows for this epic" note to the epic's own closing cycle receipt: incident/correction/deferral counts, any recurrence key firing more than once.
2. **Worktree sweep for this epic's worktrees only.** `git worktree list`; remove any worktree used by this epic's now-merged, no-longer-live cycles. Never remove one still `locked` (an agent is actively using it) or carrying unmerged commits — confirm via `git worktree list`'s lock annotation and `git log <branch>..origin/<branch>` showing nothing unmerged, per §8's existing safety rule.
3. **No PR here.** The bundle's single tranche→develop PR is §11's job, fired once, as the bundle's own final epic — an epic wrap-up is not a promotion event.

## 11. Bundle closure epilogue (fires once, as the bundle's final epic)

Every bundle's last epic is a Closure Epilogue — the pattern every prior bundle since SD-21 has
used (`SD-21-.../decisions.md §189`), extended here to also write the retrospective:

1. **Final-acceptance scan.** Every acceptance criterion 1..N is `complete`, and every epic/kanban card is at `complete`. **Never write "complete *or* filed under `## Open blockers`"** — that phrasing is the defect `docs/governance/blocker-closure-doctrine.md` exists to remove; a gate that can be satisfied by writing down that you did not do the work is not a gate. A card at `returned-to-backlog`, `in-progress`, or `DISCOVERED-forked` blocks closure, as does a card marked `complete` with a half of its criterion explicitly deferred.

   **If anything is short, this cycle stops here.** Do not write the retrospective, do not sweep, **do not open the PR**. Report what is short with the command that shows it, and exit — that is a correct and expected outcome for a closure cycle, not a failure. (SD-32's first closure cycle passed its own gate, closed over an open card, and opened a PR the operator had to close.)
2. **Write the bundle's retrospective**, grounded in the event log, not recollection:
   ```bash
   scripts/retro.py summary --since <bundle-launch-date> --json
   ```
   Read the output and write `docs/retro/<bundle-slug>-retrospective.md`, in the shape
   `docs/retro/sd31-retrospective.md` uses: the raw event tally, what the data says before
   interpretation, what worked, what didn't, and named changes for the next bundle. **Then cite it**
   from this bundle's own `references/README.md` — a retrospective that exists but is never linked
   from the package it's about is the exact gap an SD-32 chassis review found and had to fix by hand.
3. **Full worktree/branch sweep** for the whole bundle (not just this epic's, per §10) — the real
   worked example is `SD-22-.../progress.md` E7.22-26: enumerate every worktree and local branch
   tied to this bundle, report count found vs. removed, and leave anything outside this bundle's
   own lane untouched (out-of-scope cleanup is explicitly not this epic's job, per
   `SD-21-.../decisions.md §195`).
4. **Architecture-docs refresh, graphify, PR, merge-conflict resolution** — the pipeline in
   `docs/release/template/template.md §6`, unchanged by this section; that template covers the
   chassis-level closure steps, this file covers the per-cycle procedure they run inside.
5. **Release notes and version bump**, per this bundle's own versioning convention.

## 12. Standing lessons (carried from prior bundles' retrospectives)

These are rules, not context — read `docs/retro/sd31-retrospective.md` for the incidents behind
them if the reasoning matters for a judgment call.

- **A blocker on the Definition of Done gets attacked until cleared, or escalated — never
  deferred.** SD-32's first dispatch run met all four gates, filed its largest content epic's
  remaining ~16,000 units under `## Open blockers` with a named owner ("a successor bundle"),
  marked another card `complete` with half its criterion deferred, and opened the PR. Every step
  satisfied the criterion as written; the operator rejected the result and named the pattern as
  recurring. Deferral was the cheapest legal move, and the criterion made it legal. Two
  dispositions only: clear it, or raise your hand and wait. Full doctrine:
  `./blocker-closure-doctrine.md`, applied in §8 and §11 step 1.
- **A ruling that defers a capability must name the condition under which it is revisited, and
  that condition must be checked, not remembered.** SD-31's no-formula-interpreter ruling sat
  unexamined for ~18 waves after its own stated precondition (a fixture mechanism) had already
  landed, because nobody re-read it. If `decisions.md` defers something, state the revisit
  trigger explicitly and check for it at every closure scan, not just when someone happens to
  recall the deferral.
- **A headline figure written before the wave meant to establish it has returned is provisional,
  and must be marked as such in the text — never settled fact.** SD-31's own worst instance: "1,049
  formula shapes" was written into a package's scope as fact before the measuring wave returned;
  two lanes then failed to reproduce it and it was retracted, but it had already been load-bearing
  scope text for a day. If a number is estimated rather than measured, say "estimated" in the
  sentence that states it.
- **Recurring incidents get a mechanical control, not a better-worded warning.** SD-31 wrote a
  wrong-base-worktree warning into every dispatch prompt from wave 15 onward; it fired 27 times
  anyway. The actual fix (§6 step 1 of this file) is a command that stops the cycle, not prose
  asking an agent to be careful. When the same incident type recurs more than a handful of times
  across a bundle, the fix is a check with a nonzero exit code, not another sentence.
- **Sum the piles, always.** Any partition of a corpus into groups/kinds/epics is only trustworthy
  once something mechanically confirms the parts add up to the stated whole and nothing sits in
  two groups or none. `scripts/coverage_ledger.py`'s fail-closed-on-empty posture is the pattern;
  do not ship epic/gate arithmetic that "should" sum without a command proving it does.
- **Measurement work that bank zero units is still a legitimate deliverable**, and should be
  reported and judged as one — not treated as a stall because the board didn't move. SD-31's three
  highest-value waves banked almost nothing and changed the program's direction twice.

## Cross-references

- `./blocker-closure-doctrine.md` — a blocker on the Definition of Done is cleared or escalated, never deferred; `## Open blockers` is a request for a ruling, not a closure path. Enforced by §8 and §11 step 1.
- `./deferral-revisit-doctrine.md` — the sibling rule for a *planned capability deferral* (condition, checker, accepted cost). Easy to conflate with the above; the test is whether the scope was in the Definition of Done at launch.
- `./no-stub-mvp-doctrine.md`, `docs/doctrine-external/identifier-discipline.md` — the two doctrine docs §6's dual-audit gate enforces inline.
- `./wired-integration-stubs-registry.md` — operator-granted stub exceptions.
- `docs/release/template/template.md` — the sibling template every bundle's own `README.md` (folder index) is authored from. Distinct scope: that template covers the release-folder's file index and bundle-snapshot table; this template covers the per-cycle dispatch procedure. Both must agree on the dispatch mechanism (`Workflow` tool, not `/loop /batch`) — if one changes, check the other.
- `AGENTS.md §Retrospective Logging`, `scripts/retro.py`, `docs/retro/schema.json` — the event-logging discipline §2.3 points at. Emission happens throughout every cycle; §10/§11 are where the accumulated log gets read and turned into a written retrospective.
- `docs/retro/sd31-retrospective.md` — the worked example §11 step 2 follows, and the source of §12's standing lessons.
- `.claude/skills/stc-authoring/SKILL.md` — the Claude-Code-native rendering of this file plus `docs/release/template/template.md`, for a session auditing or authoring a bundle directly in this repo.
