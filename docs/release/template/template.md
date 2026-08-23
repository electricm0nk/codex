---
title: SD-NN — <Bundle Title> — Release Package
status: <planning | running | closed>
bundle_id: SD-NN
slug: <bundle-slug>
scope: docs/release/SD-NN
artifact_type: release-index
canonical_branch: <tranche/N>
kanban_board: <codex-tranche-N>
target_version: <major>.<tranche-base>.<build>
canonical_source: docs/release/SD-NN (this folder)
date: <ISO-8601>
---

# SD-NN — <Bundle Title> — Release Package

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
>
> **This bundle is operated via a `Workflow`-tool script, invoked from a live session — NOT `/loop /batch` and NOT a one-shot task.** `/loop /batch` cannot run unattended (`/batch` requires a human to type it per invocation). For how to actually **create** that script — phase structure, tiering, worked skeleton — see `workflow-instruction.md §2.4`. The full per-cycle procedure, orchestration mode, concurrency map, dual-audit gate, retro-event-logging discipline, and epic/bundle closure steps live in `workflow-instruction.md`'s body, authored from `../../governance/workflow-instruction-template.md`. The scope-draft ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what*; the workflow-instruction is the *how*.

This folder is the canonical surface for SD-NN. Everything the bundle needs is in this folder and the in-repo doc tree (sibling release folders, `../../doctrine-external/`, `../../architecture/`). The operator's workspace is referenced only at initial-package construction time; once the package lands here, the harness reads the repo-local copy and the workspace copy is no longer consulted.

## 0. Preamble

The bundle's intent, scope, and acceptance-evidence obligations live in [`scope-draft.md`](./scope-draft.md). The per-cycle launch form, eligibility checks, and self-heal mechanics live in [`workflow-instruction.md`](./workflow-instruction.md). This README is the index.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-NN |
| Slug | `<bundle-slug>` |
| Canonical branch | `<tranche/N>` (operator directive `<date>`) |
| Kanban board | `<codex-tranche-N>` (operator directive `<date>`) |
| Epics / criteria | `<N epics>` / `<N criteria>` |
| Target version | `<major>.<tranche-base>.<build>` |
| Dispatch mechanism | `Workflow` tool, invoked from a live session, per `workflow-instruction.md §2` |
| Cadence | N/A — dispatch is a live `Workflow` session, not a timer loop |
| Closure gate | `tranche/N → develop` PR; retrospective written + cited; worktree/branch sweep; release-notes generation; architecture-docs refresh (§6) — full sequence in `workflow-instruction.md §11` |

## 2. Files in this folder

| File | Job | Owner |
|---|---|---|
| `scope-draft.md` | Canonical handoff *what* — bundle intent, epics, criteria | operator |
| `workflow-instruction.md` | Per-cycle launch *how* — eligibility, self-heal, post-mortem schema | operator |
| `progress.md` | Live cycle-by-cycle progress + status matrix | loop (created on first cycle) |
| `decisions.md` | Bundle-specific ADRs | operator |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements | operator |
| `epic-breakdown.md` | Acceptance criteria 1-N grouped across epics | operator |
| `acceptance-and-verification.md` | Closure gates + verification commands + per-criterion artifact map | operator |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split; open override flags | operator |
| `technical-design.md` | Architectural surface; engine/API shapes; cross-book resolution patterns | operator |
| `content-unit-inventory.md` | Per-content-unit N-tuple (rust module / test fixture / cycle artifact / CommandName-or-ComponentName) | operator |
| `artifacts/` | Per-cycle evidence: parity fixtures, receipt comments, cycle receipts | loop (populated per cycle) |
| `artifacts/README.md` | Cycle-artifacts index (Epic-N subdirectories + closure-readiness-report) | operator-authored at package-construction time |
| `references/` | Doctrine pointers, skill pointers, sibling bundle pointers | operator |
| `references/README.md` | Doctrine / skill / sibling-bundle reference index | operator |

## 3. In-repo cross-references

Every reference below is repo-relative. No `~/workspace/...` or `programs/codex/requirements/...` paths — those live at initial-package-construction time only and are not load-bearing for the harness reading this folder.

- **Sibling release folders** — `../SD-MM/` for any other in-flight or historical bundle.
- **Repo-local doctrine mirrors** — `../../doctrine-external/identifier-discipline.md`, `../../doctrine-external/spec-domain-lifecycle.md`, `../../governance/no-stub-mvp-doctrine.md`, `../../governance/wired-integration-stubs-registry.md`.
- **Architecture docs** — `../../architecture/` (topic-by-topic; the closure epilogue §6 obligation re-verifies every touched topic).
- **Repo-local conduct surface** — `../../AGENTS.md` (Non-Negotiable Rules), `../../CLAUDE.md` (lightweight activation surface).

## 4. Relationship to other release folders

- **Sibling bundles** — `../SD-MM/` for any other in-flight or historical bundle.
- **Tranche branch posture** — this bundle ships on `<tranche/N>`.

## 5. Build version target

For SD-NN, the release version triple is `<major>.<tranche-base>.<build>`:

- **`major`** — `0` until first publish to `main`; increments by `1` per main-publish.
- **`tranche-base`** — base digit of the active tranche branch.
- **`build`** — monotonic counter across all builds across all branches (never resets).

SD-NN's first concrete value is `<major>.<tranche-base>.<current_build_at_launch>`.

## 6. Architecture-docs, graphify, and PR closure obligation (bundle closure epilogue, artifact sub-steps)

This section is the *artifact-level* half of the bundle closure epilogue — architecture docs,
graphify, the PR, and merge-conflict resolution. It fires **once**, as the bundle's own final
epic, never per-epic. The *procedural* half — writing and citing the bundle's retrospective, and
the full worktree/branch sweep — is defined in `workflow-instruction.md §11`; both halves are one
pipeline, split across these two files only because this file owns the chassis/artifact shape and
that one owns the per-cycle procedure. Do not duplicate either half's content into the other file;
cross-reference instead.

The living architecture documentation at `../../architecture/` (repo-relative) is
part of this bundle's closure gate. The pipeline is sequential — every
sub-step fires regardless of diff content:

1. **All acceptance criteria done, and every epic/kanban card at `complete`?** If not, self-heal and dispatch more cycles.

   **This is a hard gate, and a filed blocker does not satisfy it** (`../../governance/blocker-closure-doctrine.md`). A `## Open blockers` entry is a request for an operator ruling — not a disposition and never a closure path; filing one **pauses the bundle**. A blocker standing between the bundle and 100% of its Definition of Done gets **cleared** (decompose it and run the cycles — a large blocker is a sequencing problem, not an exemption) or **escalated to the operator** with the specific ruling, write scope, or precondition named — never deferred, never handed to a successor bundle on the cycle's own authority. Never write a closure criterion as "complete *or* filed under `## Open blockers`": that phrasing is the defect this doctrine removes.

   If anything is short, **the closure epilogue stops here** — no retrospective, no sweep, **no PR**. Report what is short with the command that shows it, and exit. That is a correct outcome for a closure cycle, not a failure.

   **Between step 1 and step 2, `workflow-instruction.md §11` steps 2–3 fire**: write and cite the
   bundle's retrospective, then run the full worktree/branch sweep. Both must be done before
   step 2 below opens the PR — a retrospective or a stray worktree found *after* the PR is open is
   a correction cycle, not a clean closure.
2. **Architecture docs updated?** If not, run the truth-up script at
   `~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py`
   with `--integration-target <target> --receipts-md <this-folder>/receipts.md --bundle <SD-NN>`.
   The script edits touched docs in place, removes obsolete statements, refreshes
   `Last verified:` headers, runs the maintenance contract's two verification
   one-liners, and appends a YAML receipt to `receipts.md`. Empty diffs still
   write a receipt — the receipt IS the audit evidence that the gate fired.
3. **Graphify run?** If not, run the graphify-update script at
   `~/.hermes/profiles/god-emporer/skills/devops/graphify-update/scripts/update_graphify.py`
   with `--integration-target <target> --receipts-md <this-folder>/receipts.md --bundle <SD-NN>`.
   The script invokes graphify against the codex repo, captures stdout/stderr/exit-code,
   and appends a `graphify:update` receipt to `receipts.md`. Graphify non-zero exit does
   NOT refuse the closure pipeline — the failure receipt is the audit trail; operator
   decides retry-vs-proceed.
4. **PR open?** If not, open it. (PR creation is a bash-level command in the workflow-instruction.)
5. **Merge conflicts resolved?** If any, fix them via the merge-conflict-resolution script at
   `~/.hermes/profiles/god-emporer/skills/devops/merge-conflict-resolution/scripts/resolve_merge_conflicts.py`.
   The script runs `git pull --rebase origin <target>` (pre-flight mode) or queries the GitHub
   API for the PR's `mergeable` state (post-pr mode). On conflicts, the script emits a
   `merge_conflict:*` receipt and exits non-zero — the loop self-heals, operator resolves
   manually, loop re-runs until clean.
6. **Stop — closure is complete.**

The full rules and procedure live in `../../architecture/README.md`
§Maintenance contract. Skills: `architecture-truth-up` (sub-step 2), `graphify-update`
(sub-step 3), `merge-conflict-resolution` (sub-step 5). The receipt block in
`<this-folder>/receipts.md` is the durable audit trail; without it, the bundle
did not run through the closure pipeline in a verifiable way.

## 7. Initial-package construction (operator-only, before launch)

Before this folder exists, the operator constructs the STC package from initial requirements at `~/workspace/programs/codex/requirements/SD-N-<slug>/`. The promotion skill (`release-package-promotion`, see `~/.hermes/profiles/god-emporer/skills/devops/release-package-promotion/SKILL.md`) takes the finished package and copies it into this folder deterministically. The workspace copy is not consulted again after the promotion.

Required canonical files (the promotion skill refuses to copy if any are missing):

- `README.md`, `scope-draft.md`, `workflow-instruction.md`, `progress.md`
- `epic-breakdown.md`, `decisions.md`, `risks-and-open-questions.md`, `acceptance-and-verification.md`
- `content-unit-inventory.md`, `artifacts/`, `artifacts/README.md`, `references/`, `references/README.md`

## 8. Cross-reference

- `../../governance/workflow-instruction-template.md` — the per-cycle dispatch procedure `workflow-instruction.md` is authored from. Distinct scope: this template covers the release-folder's file index and bundle-snapshot table; that one covers the per-cycle dispatch procedure, including §10's epic wrap-up and §11's bundle closure epilogue (the retro-write and worktree-sweep half of §6 above). Both must agree on the dispatch mechanism (`Workflow` tool, not `/loop /batch`) — if one changes, check the other.
- `../../governance/blocker-closure-doctrine.md` — a blocker on the Definition of Done is cleared or escalated, never deferred; `## Open blockers` is a request for an operator ruling, not a closure path. Gates §6 step 1 above.
- `../../governance/deferral-revisit-doctrine.md` — the sibling rule for a *planned capability deferral*. The test that separates the two: was this scope in the Definition of Done at launch?
- `docs/retro/` — retrospectives written at bundle closure (§6 / `workflow-instruction.md §11`). `docs/retro/sd31-retrospective.md` is the worked example.
- `.claude/skills/stc-authoring/SKILL.md` — the Claude-Code-native rendering of this template plus `workflow-instruction-template.md`, for a session authoring or auditing a bundle directly in this repo.
