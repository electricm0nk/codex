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
> **This bundle is operated via `Workflow`-orchestrated dispatch, invoked from a live session — NOT `/loop /batch` and NOT a one-shot task.** `/loop /batch` cannot run unattended (`/batch` requires a human to type it per invocation). The full per-cycle procedure, orchestration mode, concurrency map, and dual-audit gate live in `loop-instruction.md`'s body, authored from `../../governance/loop-instruction-template.md`. The scope-draft ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what*; the loop-instruction is the *how*.

This folder is the canonical surface for SD-NN. Everything the bundle needs is in this folder and the in-repo doc tree (sibling release folders, `../../doctrine-external/`, `../../architecture/`). The operator's workspace is referenced only at initial-package construction time; once the package lands here, the harness reads the repo-local copy and the workspace copy is no longer consulted.

## 0. Preamble

The bundle's intent, scope, and acceptance-evidence obligations live in [`scope-draft.md`](./scope-draft.md). The per-cycle launch form, eligibility checks, and self-heal mechanics live in [`loop-instruction.md`](./loop-instruction.md). This README is the index.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-NN |
| Slug | `<bundle-slug>` |
| Canonical branch | `<tranche/N>` (operator directive `<date>`) |
| Kanban board | `<codex-tranche-N>` (operator directive `<date>`) |
| Epics / criteria | `<N epics>` / `<N criteria>` |
| Target version | `<major>.<tranche-base>.<build>` |
| Dispatch mechanism | `Workflow` tool, invoked from a live session, per `loop-instruction.md §2` |
| Cadence | N/A — dispatch is a live `Workflow` session, not a timer loop |
| Closure gate | `tranche/N → develop` PR; release-notes generation; architecture-docs refresh (§6) |

## 2. Files in this folder

| File | Job | Owner |
|---|---|---|
| `scope-draft.md` | Canonical handoff *what* — bundle intent, epics, criteria | operator |
| `loop-instruction.md` | Per-cycle launch *how* — eligibility, self-heal, post-mortem schema | operator |
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

## 6. Architecture-docs closure obligation (Epic Closure sub-step)

The living architecture documentation at `../../architecture/` (repo-relative) is
part of this bundle's closure gate. The Epic Closure pipeline is sequential — every
sub-step fires regardless of diff content:

1. **All acceptance criteria done?** If not, self-heal and run more loops.
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
4. **PR open?** If not, open it. (PR creation is a bash-level command in the loop-instruction.)
5. **Merge conflicts resolved?** If any, fix them via the merge-conflict-resolution script at
   `~/.hermes/profiles/god-emporer/skills/devops/merge-conflict-resolution/scripts/resolve_merge_conflicts.py`.
   The script runs `git pull --rebase origin <target>` (pre-flight mode) or queries the GitHub
   API for the PR's `mergeable` state (post-pr mode). On conflicts, the script emits a
   `merge_conflict:*` receipt and exits non-zero — the loop self-heals, operator resolves
   manually, loop re-runs until clean.
6. **Stop the loop.**

The full rules and procedure live in `../../architecture/README.md`
§Maintenance contract. Skills: `architecture-truth-up` (sub-step 2), `graphify-update`
(sub-step 3), `merge-conflict-resolution` (sub-step 5). The receipt block in
`<this-folder>/receipts.md` is the durable audit trail; without it, the bundle
did not run through the closure pipeline in a verifiable way.

## 7. Initial-package construction (operator-only, before launch)

Before this folder exists, the operator constructs the STC package from initial requirements at `~/workspace/programs/codex/requirements/SD-N-<slug>/`. The promotion skill (`release-package-promotion`, see `~/.hermes/profiles/god-emporer/skills/devops/release-package-promotion/SKILL.md`) takes the finished package and copies it into this folder deterministically. The workspace copy is not consulted again after the promotion.

Required canonical files (the promotion skill refuses to copy if any are missing):

- `README.md`, `scope-draft.md`, `loop-instruction.md`, `progress.md`
- `epic-breakdown.md`, `decisions.md`, `risks-and-open-questions.md`, `acceptance-and-verification.md`
- `content-unit-inventory.md`, `artifacts/`, `artifacts/README.md`, `references/`, `references/README.md`

## 8. Cross-reference

- `../../governance/loop-instruction-template.md` — the per-cycle dispatch procedure `loop-instruction.md` is authored from. Distinct scope: this template covers the release-folder's file index and bundle-snapshot table; that one covers the per-cycle dispatch procedure. Both must agree on the dispatch mechanism (`Workflow` tool, not `/loop /batch`) — if one changes, check the other.
