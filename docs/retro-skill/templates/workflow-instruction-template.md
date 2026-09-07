---
title: Workflow Instruction Template (Scripted-Dispatch Orchestration)
canonical: true
scope: universal
status: active
review_state: accepted
canonical_source: docs/retro-skill/templates/workflow-instruction-template.md (this file)
upstream_targets:
  - every future release package's own workflow-instruction.md authors from this template
related_artifacts:
  - ../conduct/shipping-code-doctrine.md (referenced by §6's dual-audit gate)
  - ../conduct/shipping-code-doctrine.md
---

# Workflow Instruction Template — Scripted-Dispatch Orchestration

Every work bundle authors its `workflow-instruction.md` from this template, not from a prior
bundle's copy-pasted instance. This file states the current, desired dispatch procedure only — it
is not a changelog and does not narrate why any rule exists beyond the reasoning needed to apply
it correctly.

## 0. Bundle at a glance

Fill in for the new bundle:

- **Branch:** `<branch-name>`
- **Board:** board slug (reused or new)
- **Cadence:** N/A — dispatch is a live session, not a timer loop (see §2)
- **Epics / criteria:** `<count>` / `<count>`
- **First concrete build value:** captured in §8, not left as a template placeholder

## 1. Pre-launch checklist

Every command in this section must be run for real during drafting, with its actual output pasted
below the command, before the bundle is marked planning-ready. A command written from memory or
assumption is not a verified precondition.

1. **Board reachable.** Confirm the target board (whatever tracker this project uses — a kanban
   tool, or a local `kanban.md` paired with `progress.md`) is reachable and the target board/file
   exists. Paste output.
2. **Bundle branch is on the remote, pushed and ahead of the trunk branch.** Check the remote ref
   for the bundle branch, and the trunk branch's recent history. Paste output.
3. **Predecessor bundle's closure merge request is merged to the trunk branch** (if this bundle
   has a launch-gate dependency on it). Confirm the closure commit is in the trunk's recent
   history. Paste output.
4. **Any required credential is present** at whatever path the board/tracker CLI expects. Paste
   output.
5. **Working tree clean** on the bundle branch. Expect zero pending changes. Paste output.
6. **Doctrine gates.** Confirm the identifier-discipline and no-stub-shipping doctrine docs exist
   as real, loadable files, and cross-reference them as the audit-content source of record — the
   inline grep commands in §6 are what actually enforces this, not any skill-loader or doc-index
   check.
7. **Build counter captured**, not left as a template placeholder. Read the real version source of
   truth for this repo and write the literal next value into §8, e.g. "the trunk branch is at
   `0.5.98`; this bundle's first concrete build is `0.5.99`." A bundle is not planning-ready with a
   template placeholder still in the text where a real number belongs.
8. **Artifact directories exist and are empty**, one per epic.

## 2. Orchestration mode

Standing policy (pulled from the operator's own model-selection tiering — state it here once per
bundle rather than re-deriving it mid-launch):

- **Dispatch mechanism:** an in-harness scripted-dispatch tool, invoked from a live session,
  reading the bundle's `scripts/workflow-dispatch.sh` as a concurrency/tiering **spec** — not an
  unattended timer loop (which cannot run without a human re-triggering it) and not
  `scripts/workflow-dispatch.sh` executed as a standalone background process. Before trusting any
  bundle's `scripts/workflow-dispatch.sh` as containing a runnable subagent-invocation form, verify
  that form actually exists in this environment — if it doesn't, dispatch via the scripted-dispatch
  tool's own calls from the live session, reading the script's `EPIC_PARALLEL`/`EPIC_SUBAGENT`/
  `PARALLEL_OVERRIDE`/`SUBAGENT_OVERRIDE` maps as the concurrency/tiering source of truth.
- **Default subagent model:** your project's standard mid-tier model (inherits session model
  unless overridden).
- **Tiering exceptions:**
  - Housekeeping (release notes, changelog, version bump, lint fixes) → your lightest-tier model.
  - Adversarial verification / final completeness scan / judge-panel steps → your top-tier model.
  - Everything else (real implementation, test-first cycles, audits, remediation) → your standard
    mid-tier model.
- **Concurrency shape:** decided explicitly per epic in §3 below, at authoring time — not derived
  live by whichever session launches the bundle.

### 2.1 Agent environment setup

Every dispatched agent should have an actor-identity environment variable set to its role name
(e.g. `RETRO_ACTOR=catalog-import-agent`). Without this, the retrospective log's by-actor breakdown
falls back to opaque worktree directory names, which become meaningless after the run ends. The
retro tool resolves actor identity in this order: an explicit `--actor` flag → the environment
variable → worktree name → source-control user config. For per-role agents running in predictable
worktrees, setting the actor variable in the dispatch call preserves role identity for
retrospective analysis.

Every dispatched agent must also claim its per-cycle build-cache directory immediately after
establishing it (e.g. `mkdir -p "$BUILD_CACHE_DIR" && echo $$ > "$BUILD_CACHE_DIR/.reclaim-claim"`).
Between builds, no passive liveness signal protects a live agent's directory; without the claim
file, a large cache directory is protected only by an age heuristic, and a sibling's reclaim sweep
can silently delete work in progress.

### 2.2 Execution boundary — the launching session is always the orchestrator, never the executor

The session that plans, scopes, or launches a bundle is the orchestrator. §6's per-cycle procedure
— steps 1 through 9, especially step 3 ("implement the criterion test-first") — describes what
happens **inside a dispatched agent call**, never what the orchestrating session does with its own
direct edit/write/shell tool calls. This holds with no exceptions: not for a "quick" one-file fix,
not mid-investigation when the context is already loaded, not because an earlier approval already
authorized the underlying change (approval authorizes the *work*, not a shortcut around the
*mechanism*).

Discovering that a cycle's real scope differs from what the cycle doc assumed is common and
expected (see §4) — it is a reason to **pause, record the corrected scope, and dispatch (or
re-dispatch) an agent call with that scope**, never a license to keep executing inline because the
investigation already surfaced the fix.

**Self-check before any direct file-mutating call while driving a bundle:** is the target path
under the bundle's implementation trees or otherwise part of a criterion's red-to-green work? If
yes, stop — that call belongs inside a dispatched agent, not here. The orchestrating session's own
direct tool calls are reserved for: read-only investigation/scoping, authoring or correcting this
bundle's own planning docs (`workflow-instruction.md`, `epic-breakdown.md`, `decisions.md`,
`cycles/*.md`), and source-control plumbing on those planning-doc commits — never on the shipped
diff itself.

**Corollary:** mint board done-receipts inside the dispatched agent, not from the orchestrating
session's own shell calls. Board-card creation is one more §6 per-cycle step; it happens inside the
dispatched agent's scoped task, never as a bare orchestrating-session shell call.

**Dispatch first, report second.** A prior bundle lost four full stalls this way — twice the
operator had to say "you look idle" / "you stopped working again" before work resumed. A wave
finished, the orchestrating session wrote a summary, and the turn ended without dispatching the
next phase. The summary **feels** like the deliverable. It is not. Before ending any turn while the
bundle has ready, undispatched work, dispatch it first — the summary then describes something that
already exists, rather than substituting for it.

### 2.3 Retrospective event logging (every cycle)

Every dispatched cycle emits retro events during the work itself, not just a written summary at
the end — per this project's retrospective-logging discipline. Source control records what landed;
it says nothing about what nearly landed wrong or who caught it, and nothing survives the run
except this log. One prior bundle's near-2,000-event log is the reason its own retrospective is
grounded in numbers instead of recollection.

- **When you catch an error, hit an incident, defer work, or redo something**, emit the event via
  the retro tool at the moment it happens — not batched at cycle end. A "batch it at the end" habit
  is how events silently never get written.
- **Correction:** `retro correction --subject <who-was-wrong> --claimed <value> --actual <value>
  --verified-by <command-or-check>`. `--verified-by` is required — an unverified correction is a
  competing assertion, not a finding.
- **Incident / deferral / rework:** `retro <type> ...` — check that type's help for its required
  fields.
- **The actor-identity variable must already be set** (§2.1) so the by-actor breakdown resolves to
  a real role name instead of an opaque worktree path once the run ends.
- Full vocabulary and field contract: a versioned event schema. The tool builds its own CLI from
  that schema, so it cannot drift from what it accepts.

### 2.4 Creating the dispatch script

The bundle's dispatch is a script passed to the scripted-dispatch tool from the live orchestrating
session — plain JavaScript (no TypeScript syntax), not a shell script and not a timer-loop
invocation. If the bundle has a `scripts/workflow-dispatch.sh`, it is read as **data**: its
`EPIC_PARALLEL` / `EPIC_SUBAGENT` / `PARALLEL_OVERRIDE` / `SUBAGENT_OVERRIDE` maps supply the
concurrency and tiering values the script below plugs in. It is never executed as a standalone
process.

Every dispatch script for this project's bundles follows the same shape:

1. `export const meta = { name, description, phases }` — one `phases` entry per epic/gate row in
   §3's parallel/sequential map, same titles, so the progress display groups cleanly.
2. One `phase('<epic/gate title>')` call per §3 row, fired in the gated order that table states —
   never out of order, even when two epics are each individually parallel-safe.
3. Inside a phase, default to `pipeline()` for a chain of cycles within one criterion (red → green
   → audit → receipt is a chain, not independent work); use `parallel()` only when §3 marked that
   epic `parallel: yes` for genuinely disjoint files, and then every agent in that call gets an
   isolated worktree per §3's rule.
4. Every `agent()` call sets `model` explicitly, per §2's tiering table (your standard tier for
   build/integration, top tier only for adversarial verification, lightest tier for housekeeping).
   **Never omit `model`** — an omitted `model` silently inherits the orchestrating session's own
   model. A prior bundle did this and burned most of a week's top-tier-model budget on six
   inherited-tier build lanes in three hours.
5. Every `agent()` prompt for a criterion cycle embeds §6's procedure verbatim (or points at this
   file plus the specific criterion) — the dispatched agent starts with zero context of this
   bundle.

**Worked skeleton** (adapt phase titles/counts to this bundle's own §3 table — never copy another
bundle's script verbatim any more than you'd copy its `workflow-instruction.md` verbatim):

```javascript
export const meta = {
  name: 'bundle-dispatch',
  description: '<bundle one-line>',
  phases: [
    { title: 'Epic 1 — <name>' },
    { title: 'Epic 2 — <name>' },
  ],
}

phase('Epic 1 — <name>')
const epic1Results = await pipeline(
  criteriaForEpic1,                          // [{id, prompt}, ...] from epic-breakdown.md
  c => agent(cycleProcedurePrompt(c), { model: 'standard-tier', phase: 'Epic 1 — <name>' }),
)

phase('Epic 2 — <name>')
// parallel: yes per §3 — genuinely disjoint files, each agent gets its own worktree
const epic2Results = await parallel(
  criteriaForEpic2.map(c => () =>
    agent(cycleProcedurePrompt(c), {
      model: 'standard-tier',
      phase: 'Epic 2 — <name>',
      isolation: 'worktree',
    })),
)
```

Validate the returned result against a schema, and gate on its structured fields — never on a
substring match against the agent's prose. A cycle result should be a typed object (e.g.
`{ status: 'complete' | 'partial' | 'blocked-escalated', filesTouched: [...], auditResult: {...} }`),
and the gate check reads `result.status === 'complete'`, not a regex over a free-text summary.

## 2.5 A dispatched agent is never resumed — never end a turn waiting

**A dispatched agent call gets exactly one turn.** Nothing wakes it up: there is no monitor, no
completion notification, and no re-invocation. An agent that backgrounds a long command and ends
its turn to "resume when it reports" loses its entire cycle, and the orchestrator gets a status
line instead of work.

This has real cost. In one prior bundle, two consecutive lanes returned exactly this and landed
**zero** commits between them:

> *"I'll end this turn now and wait for the monitor's completion notification before continuing."*
> *"Waiting for the verification background process to finish before completing the cycle. I'll
> resume once it reports."*

Both were caught only because the orchestrator checked the repo rather than reading the summary —
the commit log showed nothing new and the target files were untouched.

**Put this in every dispatch prompt**, alongside the environment block:

- Wait for slow work **inside** the turn — run it in the foreground, or poll a background job in a
  loop with sleeps between checks. Do not end the turn expecting to be re-invoked.
- **Scope test runs.** On a contended machine a full unscoped test sweep over hundreds of targets
  may never finish. Name the targeted modules/binaries the change touches, plus the
  workspace-level suites, and say explicitly which sweeps *not* to run.
- If something genuinely will not finish, **report what was observed and why**, and commit the
  work anyway. A cycle that lands its change and reports one unfinished suite is a success; a
  cycle that lands nothing while waiting is a total loss.
- **Commit and push before ending the turn**, always — even for a partial result.

**Orchestrator's side of the control:** never accept a lane's final message as evidence that work
landed. Check the commit log and the target files. A return value that describes an intention
rather than an outcome is a stall, not a result.

## 3. Per-epic parallel/sequential map

Fill in one row per epic, only after completing §4's path verification. A `parallel: yes` row is
only valid if the epic's criteria touch genuinely disjoint files (verified, not assumed).

| Epic | Criteria | Parallel? | File-touch set (verified) | Gated on |
|---|---|---|---|---|
| `<N>` | `<x.1-x.n>` | yes/no | `<verified real paths>` | `<prior epic, or none>` |

When `parallel: yes`, every parallel agent that mutates files must get an isolated worktree —
agents are mutating a shared checkout and will otherwise step on each other's working-directory
state even on disjoint files. This is load-bearing, not advisory: any parallel dispatch with more
than one agent that will edit/write/commit inside the SAME shared (non-worktree) checkout must
either (a) give each agent its own worktree, or (b) be re-scoped to run those agents serially
instead. A shared-checkout parallel dispatch is only safe when every agent in it is genuinely
read-only.

## 4. File-touch verification (required before §3 is filled in)

Enumerate every path named in this document, `content-unit-inventory.md`, and
`technical-design.md`. Confirm it exists as written. If it doesn't:

- Find the real analogous file/module.
- Correct the path in this document before publish (don't ship a known-wrong path and rely on
  cycle agents to discover the mismatch later).
- If the mismatch reflects a genuine design decision still open, note it explicitly rather than
  silently assuming either the plan or the repo is right.

## 5. Concurrent-write protocol

Every cycle that commits and pushes to the shared bundle branch must use this exact retry
protocol — do not invent a per-bundle variant:

```bash
git fetch origin <branch> && git rebase origin/<branch> && git push origin HEAD:<branch>
```

On non-fast-forward rejection, repeat up to 5 times. If it still fails after 5 attempts, stop and
report a `CLAIM-EXISTS` blocker — do not force-push. This applies to both the code commit and any
shared-state file every cycle touches (e.g. `progress.md`): re-fetch and re-read the file's current
content immediately before editing it, so a concurrent cycle's append isn't clobbered.

## 6. Per-cycle procedure

**This procedure runs inside a dispatched agent call — see §2.2.** The orchestrating session never
performs steps 1–9 itself with its own tool calls.

1. Ensure the working tree/worktree is based on the latest bundle branch (§5's fetch+rebase), **and
   verify the base is real rather than assuming it.** A prior bundle wrote a prose warning about
   wrong-base worktrees into every dispatch prompt from partway through the run onward; it still
   fired dozens of times, because a warning in a prompt is not a control. Run this and stop if it
   fails, before doing anything else:
   ```bash
   test -d <a directory this branch must have> && test -d <another> \
     || { echo 'WRONG BASE — reset before continuing'; exit 1; }
   ```
   On failure, reset hard to the pinned bundle-branch commit and re-verify. If the bundle has
   accumulated spent throwaway branches, delete them once merged — the actual fix is removing the
   branches that could be wrongly selected as a base, not a better-worded warning.
2. `BASE_BRANCH=$(git merge-base HEAD origin/<trunk>)` — define this before either grep block
   below, not between them.
   ```bash
   BASE_BRANCH=$(git merge-base HEAD origin/<trunk>)

   # Identifier audit — bundle-tag leaks in diff
   # Trailing \b is deliberately omitted -- \b never matches between `_` and a
   # following word character, so a trailing \b silently fails to catch real
   # identifiers like `wb19_catalog_import`. Do not add it back.
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(wb[0-9]+_|WB[0-9]+_|Wb[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'

   # No-stub-shipping audit — forbidden patterns in shipping code
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
   ```
3. Implement the criterion test-first: RED → confirm it fails for the intended reason → GREEN →
   run the relevant test suite.
4. Re-run the dual-audit gate on the final diff; both must show `OK_*`. A single-token violation is
   self-healable inline; re-audit and continue.
5. Write the cycle receipt to `artifacts/<epic>/<cycle-id>_cycle_receipt.md` (schema in §7).
6. Commit, then push via §5's retry protocol.
7. Update the shared progress doc in place via §5's protocol.
8. Mint the board card as a done-receipt, from inside this dispatched agent (per §2.2's corollary;
   per the bundle's assignee/daemon-hazard doctrine if one applies). Use whatever board CLI or
   local-file convention this bundle adopted; if the bundle uses a local `kanban.md`, "mint the
   card" means editing its row (`Status` → `COMPLETE`) and appending the receipt to `progress.md`.
9. Report: criterion, files touched, commit SHA(s), dual-audit results, red→green evidence, receipt
   path, board card ID, discoveries, next-cycle plan.

## 7. Per-cycle receipt schema

```markdown
# Cycle <cycle-id> — <epic-name> / Criterion <n>

- **Card ID:** <board-card-id>
- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violation list>
- **No-stub audit result:** OK_NO_TOKENS / <violation list>
- **Acceptance criterion:** <verbatim from the epic breakdown>
- **Figures + their re-derive commands:** <every number, with the command that produced it AND its denominator in the same construct>
- **Row-count command output:** <the literal output of the count run on THIS cycle's own artifact>
- **Build scope verified:** <what was run, its result, and the SHA it ran at>
- **Movement, four buckets:** closure / reclassification / reachability / measurement-correction
- **Status:** complete | partial | blocked-escalated
- **Remainder (partial only):** <every remaining item named by sub-cause, with populations that sum exactly to the stated total>
- **Notes:** <judgment calls>
- **Next-cycle plan:** <what the next cycle picks up>
```

**The four rows that do the work.** A receipt without them is a summary, and a summary is not
evidence:

- **Figures + commands.** A number with no command beside it is a recollection. A number with no
  denominator invites the reader to supply their own, and they will supply the wrong one.
- **Row-count output.** The cycle's `Status` is a mechanical function of this, never a
  self-assessment of effort. Paste the literal output, not a description of it.
- **Build scope + its SHA.** Run the widest verification **after the last write in the cycle that
  can move a figure an assertion depends on**. A green run followed by a regeneration is a true
  report of a tree that no longer exists.
- **Movement in four buckets.** Keeps a count that dropped because the *measurement* changed from
  being reported as work completed. They are not the same thing and conflating them is how a board
  drifts from reality.

**The three statuses:**

| Status | Meaning | Effect |
|---|---|---|
| `complete` | the whole assigned population reached the bar | the card closes |
| `partial` | part closed, **and every remaining item named by sub-cause with populations that sum exactly** | card stays `in-progress`; the dispatch continues |
| `blocked-escalated` | needs an operator ruling — a question this cycle may not decide | **pauses the bundle** |

**Needing more cycles is `partial`, never `blocked-escalated`** (`../conduct/blocker-doctrine.md`).
A `partial` whose sub-causes do not sum to its stated total is a `complete` claim in disguise and
fails the same way.

## 8. Self-heal posture

- **Self-healable (resolve inline, carry on):** dirty tree, single-token audit violation,
  unrelated test-setup breakage, build-counter out of sync, `## DISCOVERED` duplicates.
- **Non-self-healable (return `blocked-escalated` and write `## Open blockers`):** working tree diverged from the
  bundle branch needing manual rebase; two live cycles on conflicting files; a launch-gate
  dependency not actually merged; `## DISCOVERED` queue > 10 entries; red → green not preserved in
  the cycle receipt; a cycle finds `success: true` from a fake operation, an inline mock in a
  shipping module, or a "Would …" string in shipping code.

**A `## Open blockers` entry is a request for an operator ruling — not a disposition, and never a
closure path** (see the project's blocker-closure doctrine doc). Filing one **pauses the bundle**.
It does not authorise any later cycle to proceed past the blocked card, and only an operator ruling
may move blocked scope into a forward-scope register.

A blocker standing between the bundle and 100% of its definition of done has exactly **two**
dispositions:

1. **Clear it.** Do the work. If it is bigger than one cycle, decompose it and run the cycles — a
   large blocker is a sequencing problem, not an exemption.
2. **Raise your hand.** Escalate to the operator, naming what blocks you, what you already tried,
   and the specific ruling, write scope, or precondition you need. Then stop and wait.

"Filed with a named owner", "forwarded to a successor bundle", "deferred with reason", and "out of
scope for this cycle" are **not** dispositions for such a blocker — they are ways of writing down
that the bundle is not done. A cycle that correctly refuses to write outside its granted scope has
not failed; it has hit a blocker only the operator can clear, and the right move is to prepare the
exact change, escalate, and wait. Distinguish this from a planned **capability deferral** (the
project's deferral-revisit doctrine) by one test: **was this scope in the definition of done at
launch?** If yes, it is a blocker.

**Disk usage — check proactively, not reactively.** After every wave of `parallel: yes` cycles
completes (not just when something breaks), check available disk space and the live worktree list;
if usage is climbing toward the disk's ceiling, prune merged worktrees and their build-cache
directories immediately — don't wait for a build to fail with an out-of-space error first. Never
remove a worktree that's still locked (an agent is actively using it); confirm via the worktree
list's lock annotation and via the working-tree status/ahead-behind check showing no unmerged,
uncommitted work before removing anything.

## 9. Placeholder-resolution checklist (final gate before "planning-ready")

Grep the whole bundle directory for `<...>`-style placeholders and template markers (e.g.
`0.5.<current_build>`) before publish:

```bash
grep -rn '<[a-z_-]*>' <path-to-release-packages>/<PREFIX-NN>-<slug>/*.md
```

Every match must be resolved to a real value, or explicitly justified as intentionally deferred
(e.g., "filled in by Epic 8's cycle, not at authoring time").

## 10. Epic wrap-up (fires after every epic)

Lightweight, and distinct from §11's bundle-final closure — this runs at the end of **every**
epic, not just the last one:

1. **Retro summary for the epic's work window.** `retro summary --since <epic-start-date> --json`
   (or a tighter window if the epic's actual start is known more precisely). Read it — don't just
   run it and move on. Append a short "what the retro log shows for this epic" note to the epic's
   own closing cycle receipt: incident/correction/deferral counts, any recurrence key firing more
   than once.
2. **Worktree sweep for this epic's worktrees only.** List active worktrees; remove any worktree
   used by this epic's now-merged, no-longer-live cycles. Never remove one still locked (an agent
   is actively using it) or carrying unmerged commits — confirm via the lock annotation and the
   ahead-behind check showing nothing unmerged, per §8's existing safety rule.
3. **No merge request here.** The bundle's single branch-to-trunk merge request is §11's job, fired
   once, as the bundle's own final epic — an epic wrap-up is not a promotion event.

## 11. Bundle closure epilogue (fires once, as the bundle's final epic)

Every bundle's last epic is a Closure Epilogue:

1. **Final-acceptance scan.** Every acceptance criterion 1..N is `complete`, and every epic/board
   card is at `complete`. **Never write "complete *or* filed under `## Open blockers`"** — that
   phrasing is the defect the blocker-closure doctrine exists to remove; a gate that can be
   satisfied by writing down that you did not do the work is not a gate. A card at
   `in-progress`, `partial`, or `blocked-escalated` blocks closure, as does a card
   marked `complete` with half of its criterion explicitly deferred.

   **If anything is short, this cycle stops here.** Do not write the retrospective, do not sweep,
   **do not open the merge request**. Report what is short with the command that shows it, and
   exit — that is a correct and expected outcome for a closure cycle, not a failure. (One prior
   bundle's first closure cycle passed its own gate, closed over an open card, and opened a merge
   request the operator had to close.)
2. **Write the bundle's retrospective**, grounded in the event log, not recollection:
   ```bash
   retro summary --since <bundle-launch-date> --json
   ```
   Read the output and write `docs/retro/<bundle-slug>-retrospective.md`: the raw event tally,
   what the data says before interpretation, what worked, what didn't, and named changes for the
   next bundle. **Then cite it** from this bundle's own `references/README.md` — a retrospective
   that exists but is never linked from the package it's about is a gap that has bitten this
   project before and had to be fixed by hand.
3. **Full worktree/branch sweep** for the whole bundle (not just this epic's, per §10): enumerate
   every worktree and local branch tied to this bundle, report count found vs. removed, and leave
   anything outside this bundle's own lane untouched (out-of-scope cleanup is explicitly not this
   epic's job).
4. **Architecture-docs refresh, merge request, merge-conflict resolution** — the pipeline in
   `release-package-template.md §6`, unchanged by this section; that template covers the
   chassis-level closure steps, this file covers the per-cycle procedure they run inside.
5. **Release notes and version bump**, per this bundle's own versioning convention.

## 12. Standing lessons (carried from prior bundles' retrospectives)

These are rules, not context — read the relevant retrospective for the incidents behind them if
the reasoning matters for a judgment call.

- **A blocker on the definition of done gets attacked until cleared, or escalated — never
  deferred.** One prior bundle's first dispatch run met all four gates, filed its largest content
  epic's remaining scope under `## Open blockers` with a named owner ("a successor bundle"), marked
  another card `complete` with half its criterion deferred, and opened the merge request. Every
  step satisfied the criterion as written; the operator rejected the result and named the pattern
  as recurring. Deferral was the cheapest legal move, and the criterion made it legal. Two
  dispositions only: clear it, or raise your hand and wait. Full doctrine: the blocker-closure
  doctrine doc, applied in §8 and §11 step 1.
- **A ruling that defers a capability must name the condition under which it is revisited, and
  that condition must be checked, not remembered.** One prior bundle's deferral ruling sat
  unexamined for many cycles after its own stated precondition had already landed, because nobody
  re-read it. If `decisions.md` defers something, state the revisit trigger explicitly and check
  for it at every closure scan, not just when someone happens to recall the deferral.
- **A headline figure written before the wave meant to establish it has returned is provisional,
  and must be marked as such in the text — never settled fact.** A prior bundle wrote a specific
  count into a package's scope as fact before the measuring wave returned; two lanes then failed to
  reproduce it and it was retracted, but it had already been load-bearing scope text for a day. If
  a number is estimated rather than measured, say "estimated" in the sentence that states it.
- **Recurring incidents get a mechanical control, not a better-worded warning.** A prior bundle
  wrote a wrong-base-worktree warning into every dispatch prompt partway through the run onward; it
  fired dozens of times anyway. The actual fix (§6 step 1 of this file) is a command that stops the
  cycle, not prose asking an agent to be careful. When the same incident type recurs more than a
  handful of times across a bundle, the fix is a check with a nonzero exit code, not another
  sentence.
- **Sum the piles, always.** Any partition of a dataset into groups/kinds/epics is only
  trustworthy once something mechanically confirms the parts add up to the stated whole and
  nothing sits in two groups or none. A fail-closed-on-empty ledger script is the pattern; do not
  ship epic/gate arithmetic that "should" sum without a command proving it does.
- **Measurement work that banks zero units is still a legitimate deliverable**, and should be
  reported and judged as one — not treated as a stall because the board didn't move. A prior
  bundle's three highest-value waves banked almost nothing and changed the program's direction
  twice.

## Cross-references

- `../conduct/blocker-doctrine.md` — a blocker on the definition of done is cleared or escalated,
  never deferred; `## Open blockers` is a request for a ruling, not a closure path. Enforced by §8
  and §11 step 1.
- `../conduct/deferral-doctrine.md` — the sibling rule for a *planned capability deferral*
  (condition, checker, accepted cost). Easy to conflate with the above; the test is whether the
  scope was in the definition of done at launch.
- `../conduct/shipping-code-doctrine.md`, `../conduct/shipping-code-doctrine.md` — the two
  doctrine docs §6's dual-audit gate enforces inline.
- `../conduct/shipping-code-doctrine.md` — operator-granted stub exceptions.
- `release-package-template.md` — the sibling template every bundle's own `README.md` (folder
  index) is authored from. Distinct scope: that template covers the release-folder's file index
  and bundle-snapshot table; this template covers the per-cycle dispatch procedure. Both must agree
  on the dispatch mechanism — if one changes, check the other.
- `../conduct/AGENTS-retro-section.md`, `../tools/retro.py`, a versioned retro-event schema — the
  event-logging discipline §2.3 points at. Emission happens throughout every cycle; §10/§11 are
  where the accumulated log gets read and turned into a written retrospective.
- `../skills/release-package-authoring/SKILL.md` — the coding-agent-native rendering of this
  file plus `release-package-template.md`, for a session auditing or authoring a bundle directly in
  this repo.
