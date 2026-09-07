---
name: release-package-authoring
description: Author, fill out, or audit a work bundle's release package (a scoped delivery chassis) under a repo's release-packages tree. Use when creating a new bundle, filling in its chassis files from predecessor docs, reviewing an existing package for content completeness, evaluating launch readiness, checking a workflow-instruction.md / kanban.md dispatch cycle for protocol violations, writing a dispatch script for a bundle, or checking whether an epic's or bundle's closure (retrospective written and cited, worktree/branch cleanup, merge request) actually happened. Also use when the user asks to "evaluate workflow-instruction.md for launch readiness," or to review a release package for gaps.
---

# Release Package Authoring

A release package is the scoped chassis a work bundle plans and executes from: one folder holding
its scope, its per-cycle dispatch procedure, its progress record, and its closure evidence. This
skill authors and audits that chassis. It follows a fixed shape — read the reasoning once, then
apply the condensed checklist below on every bundle.

**Two canonical templates.** Every bundle is authored from these, never forked:

- `../../templates/release-package-template.md` — the chassis shape: which files exist, what each
  is for, the bundle-snapshot table, the closure obligation.
- `../../templates/workflow-instruction-template.md` — the per-cycle dispatch procedure every
  bundle's own `workflow-instruction.md` is authored from.

## The 13 required canonical files

`release-package-template.md §7` lists these; a bundle is not complete without all of them:

```
README.md, scope-draft.md, workflow-instruction.md, progress.md,
epic-breakdown.md, decisions.md, risks-and-open-questions.md,
acceptance-and-verification.md, content-unit-inventory.md,
artifacts/, artifacts/README.md, references/, references/README.md
```

`technical-requirements.md` and `technical-design.md` are also standard in practice even though
the template's §7 list omits them — check sibling packages in this repo's release-packages tree
for the current convention before treating a package missing them as complete.

## When generating a new bundle's chassis

1. Read both templates in full before writing anything.
2. Read every predecessor document the operator hands you (README/scope-draft/epic-breakdown or
   similar handoff artifacts from a prior bundle's closing session, or from initial planning). The
   chassis fill-out is a **transcription-and-organization task, not a generation task** — the
   content comes from what the predecessor material already contains, not from a one-sentence
   prompt.
3. Emit the files verbatim from the templates, replacing only bundle-specific fields (slug,
   branch, build version, owner). A `<...>`-style placeholder is for a human to resolve, never for
   you to invent content for.
4. Flag every remaining placeholder explicitly, and name where it resolves (e.g.
   `0.12.<build_at_launch>` — resolved at the first cycle per `decisions.md §9` and
   `workflow-instruction.md §11`). Do not ship an unflagged placeholder.
5. Run the placeholder gate before calling it done:
   ```bash
   grep -rn '<[a-z_-]*>' <path-to-release-packages>/<PREFIX-NN>-<slug>/*.md
   ```
   Every match must be a documented schema/receipt placeholder, a documented deferred value with a
   named resolution point, or a bug to fix.

## When auditing an existing package for content completeness

This is what "review for content completeness" or "evaluate for launch readiness" means in
practice:

1. `find <path-to-release-packages>/<PREFIX-NN>-<slug> -type f | sort` — diff against the 13-file
   list above.
2. Read every file. A file that exists but is a bare template skeleton with no real content is the
   same defect as a missing file — check status frontmatter (`status: planning-ready` vs `draft`)
   and look for template markers the placeholder gate would catch.
3. Cross-check content against the package's own source documents (README, scope-draft,
   epic-breakdown, and any handoff/artifacts files) — a fact stated in one file and never
   propagated to `kanban.md`, `references/README.md`, or a forward-scope register is a gap even
   when every file individually looks complete. Concretely: does every named piece of follow-up
   work (an "orphaned branch," a named lesson, a retrospective) have an actual queue entry or
   citation, not just a mention in prose?
4. Check whether a sibling package's retrospective or predecessor-package lessons doc exists and
   is cited from this package's `references/README.md`. It is easy for a chassis fill-out to carry
   the *content* of a predecessor's lessons without ever linking the *source document* — grep for
   it:
   ```bash
   grep -rn "retro" <path-to-release-packages>/<PREFIX-NN>-<slug>/references/README.md
   ```
5. If the package under audit has itself closed (or claims to), check that *its own* closure
   actually ran the full epilogue, not just the merge request: a retrospective file exists and is
   cited from `references/README.md`; `progress.md`/`decisions.md` records a worktree/branch sweep
   with a real count (not just "merge request opened"); and the retrospective's own "changes for
   next bundle" section, if any, actually landed somewhere (this skill, the templates, or the
   successor bundle's `decisions.md`) rather than sitting unread. See "Epic and bundle closure"
   below for the full checklist.
6. Sanity-check any verification command that reads a file as structured data (`json.load`, `jq`,
   etc.) — confirm the target path is actually the right format. A command that tries to parse a
   markdown file as JSON is a real, verifiable defect, not a style nit.
7. Report gaps concretely (file + line + what's missing), then fix them as small, targeted edits —
   this is planning-doc authoring, not shipping-code work, so it's fine for the orchestrating
   session to make these edits directly (see the boundary rule below).

## The orchestrator/executor boundary — the rule most often violated

The session that plans, scopes, or authors/audits a bundle's **planning docs**
(`workflow-instruction.md`, `epic-breakdown.md`, `decisions.md`, `kanban.md`, `progress.md`,
`README.md`, etc.) may edit those files directly. It must **never** edit shipping code directly,
even for a one-line fix, even mid-investigation, even when an earlier approval already authorized
the underlying change. That work happens inside a dispatched agent call — see
`./workflow-instruction-template.md §2.2` for the full self-check. Discovering the
real scope of a fix while investigating is a reason to dispatch (or re-dispatch) an agent call with
the corrected scope, never a license to make the fix inline because the context is already loaded.

## Retro event logging and running the retro

Source control records what landed; it records nothing about what nearly landed wrong or who
caught it. This project's retrospective-logging discipline plus `../../tools/retro.py` is the
mechanism, and a well-grounded retrospective is grounded in numbers only because the logging
happened throughout the run, not written up once at the end.

- **Every dispatched cycle** emits a `correction` / `incident` / `deferral` / `rework` event via
  the retro tool the moment it happens, with the actor-identity variable set. `--verified-by` is
  required on a `correction` — an unverified one is a competing assertion, not a finding.
- **After every epic** (light touch): `retro summary --since <epic-start>`, folded into that
  epic's closing receipt.
- **At bundle closure** (full write-up): run the full-bundle summary, write
  `docs/retro/<bundle-slug>-retrospective.md` (raw event tally, what the data says, what worked,
  what didn't, named changes for the next bundle), and **cite it from `references/README.md` in
  the same closure cycle** — not as a follow-up.

## Creating the dispatch script

Dispatch is a script passed to the scripted-dispatch tool from the live orchestrating session —
plain JavaScript, not a shell script, not a timer loop.
`./workflow-instruction-template.md §2.4` has the full worked skeleton; condensed:

1. `export const meta = { name, description, phases }` — one phase per epic/gate row in the
   bundle's own `workflow-instruction.md §3` table, same titles.
2. `phase()` calls fire in the gated order that table states.
3. `pipeline()` by default for a chain of cycles within one criterion; `parallel()` only where §3
   marked `parallel: yes`, and then every agent gets an isolated worktree.
4. Every `agent()` call sets `model` explicitly — never omit it (see failure modes below).
5. Every dispatched agent's prompt embeds `workflow-instruction.md §6`'s procedure or points at it
   plus the specific criterion — it starts with zero context of this bundle.
6. The script validates each cycle's return value against a schema and gates on structured fields
   (e.g. `result.status === 'complete'`), never on a substring match against the agent's free-text
   summary.

## Epic and bundle closure

`workflow-instruction-template.md §10` (every epic) and `§11` (once, as the bundle's final epic)
define the full procedure; condensed:

- **After every epic:** retro summary for that epic's window (above) + a worktree sweep scoped to
  that epic's own worktrees only (never remove a locked one or one carrying unmerged commits). No
  merge request at this granularity.
- **Bundle closure (once):** final-acceptance scan of every criterion **and every epic/board card
  at `complete`** → write and cite the bundle's retrospective (above) → full worktree/branch sweep
  for the whole bundle → architecture-docs refresh + merge request + merge-conflict resolution
  (`release-package-template.md §6`) → release notes + version bump. The retro write-up and
  worktree sweep happen **before** the merge request opens — finding either one missing after the
  merge request is already open means the closure needs a correction cycle, not a clean pass.

**A filed blocker does not satisfy the final-acceptance scan**
(`../../conduct/blocker-doctrine.md`). An `## Open blockers` entry is a request for an operator
ruling — not a disposition, never a closure path; filing one **pauses the bundle**. A blocker
between the bundle and 100% of its definition of done gets **cleared** (decompose it and run the
cycles — a large blocker is a sequencing problem, not an exemption) or **escalated to the
operator** with the specific ruling, write scope, or precondition named. Never deferred, never
handed to a successor bundle on a cycle's own authority.

**When authoring or auditing a package, treat `"complete or filed under ## Open blockers"` in any
closure criterion as a defect to fix before launch** — that exact phrasing let one prior bundle's
first closure cycle pass its own gate over an open card and open a merge request the operator had
to close. If the scan is short, the closure cycle stops: no retrospective, no sweep, **no merge
request**; report what is short with the command that shows it. Separate this from a planned
*capability deferral* (`../../conduct/deferral-doctrine.md`) with one test: was this scope
in the definition of done at launch? If yes, it is a blocker.

## The dual-audit gate (every dispatched cycle)

Two greps run on every cycle's diff before it's allowed to commit:

```bash
BASE_BRANCH=$(git merge-base HEAD origin/<trunk>)

# Identifier audit — no trailing \b (it doesn't match _ -> word-char boundaries;
# adding it back silently stops catching real identifiers like wb19_catalog_import)
git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(wb[0-9]+_|WB[0-9]+_|Wb[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'

# No-stub-shipping audit
git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

A single-token violation is self-healable inline (fix, re-audit, continue). A real stub in
shipping code is **non-self-healable** — return `blocked-escalated` with an `## Open blockers` entry in
`progress.md`, don't silence the gate, don't skip it on a cycle that "looks clean," don't treat
five green cycles as license to skip the sixth.

## Failure modes to actively guard against

Each of these has a real, recorded incident in this project's history:

- **Counterfeit completion.** A closed cycle whose diff is empty or trivially green. Re-derive the
  claimed outcome against the live data and the audit gates before accepting it — don't transcribe
  a dispatched agent's self-report.
- **Laundering a blocker into a deferral.** Scope that was in the definition of done at launch
  reappears as a forward-scope-register row with a "named owner", and the board goes green over
  unfinished work. This is the single most-repeated procedural failure in this project's history.
  Recognise it by the phrasing — "filed with a named owner", "forwarded to a successor bundle",
  "deferred with reason", "out of scope for this cycle" — and by a criterion that permits it.
  Doctrine: `../../conduct/blocker-doctrine.md`.
- **Boundary bypass.** See above. Single most-violated rule in this project's history.
- **Stashing from a subdirectory.** Never — a bare stash can capture the whole shared checkout even
  when called from inside a package subdirectory. Read a baseline file from the base commit
  instead of stashing to inspect it.
- **Trusting a filesystem-timestamp check for freshness.** Clock skew between agent processes and
  the host can make timestamp-based freshness checks lie. Use a working-tree status check or an
  explicit content comparison instead.
- **Omitting `model` on a dispatched agent call.** Inherits the orchestrator's model, not the
  intended tier. A real incident burned most of a week's top-tier-model budget this way. Set it
  explicitly every time: your standard tier for build/integration, top tier only for adversarial
  verification, lightest tier for housekeeping.
- **Shipping `<...>` placeholders.** Run the placeholder gate (above) before calling any bundle
  planning-ready.
- **Trusting a "complete" cycle status without re-deriving.** The whole point of an integration
  cycle is to catch what the lane that produced the work got wrong.
- **Treating a written summary as the deliverable.** A prior bundle lost four full stalls this
  way — a wave finished, a summary got written, and the turn ended without dispatching the next
  phase. Dispatch first; the summary then describes something that already exists.
- **Writing a better-worded warning for a recurring incident instead of a control.** A
  wrong-base-worktree warning went into every dispatch prompt in one prior bundle from partway
  through the run onward and still fired dozens of times. When the same incident type recurs more
  than a handful of times, the fix is a command with a nonzero exit code, not another sentence of
  prose.

## Fixture and baseline discipline (when a cycle emits computed values)

- Every re-derived figure quoted in a receipt names the command that produced it and, if it came
  from a pinned reference dataset, the pin/commit identifier for that dataset. A figure without a
  stated source commit is not re-derived — it's a number that may have drifted.
- A fixture's expected value must be transcribed from bytes the code's own read path does **not**
  touch. A fixture built from the same file the code reads is a mirror, not a check — it will
  happily validate a fabricated value.

## Cross-references

- `../../templates/release-package-template.md` — the chassis template.
- `../../templates/workflow-instruction-template.md` — the per-cycle dispatch procedure template.
- `../../conduct/shipping-code-doctrine.md`, `../../conduct/shipping-code-doctrine.md` —
  doctrine-of-record for the dual-audit gate.
- `../../conduct/blocker-doctrine.md` — a blocker on the definition of done is cleared or
  escalated, never deferred; `## Open blockers` is a request for an operator ruling, not a closure
  path. Check every package under audit for a closure criterion that permits a filed blocker.
- `../../conduct/deferral-doctrine.md` — the sibling rule for a *planned capability
  deferral* (condition, checker, accepted cost).
- `../../tools/retro.py` and its versioned event schema — retrospectives and the event-logging
  discipline behind them. Check whether the package under audit cites the relevant one.
