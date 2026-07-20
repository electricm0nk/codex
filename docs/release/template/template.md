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
> **This bundle is operated via the `/loop 60m /batch /goal` invocation model — NOT a one-shot task.** After exiting plan mode, the coding harness is **required** to launch SD-NN as:
>
> ```bash
> /loop 60m /batch /goal ./loop-instruction.md
> ```
>
> The `/loop` invocation uses a repo-relative path — the harness reads `./loop-instruction.md` from inside this folder. The full per-cycle procedure, file-touch partition, post-mortem card mint, and progress-doc update live in `loop-instruction.md`'s body. The scope-draft ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what*; the loop-instruction is the *how*.

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
| Loop launch form | `/loop 60m /batch /goal ./loop-instruction.md` (repo-relative) |
| Cycle cadence | 60m restart; `/batch` for shared-file concurrency |
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

## 6. Architecture-docs closure obligation

The living architecture documentation at `../../architecture/` (repo-relative) is
part of this bundle's closure gate. Before the `tranche/N → develop` closure PR
is opened, the closure epilogue MUST:

1. Review the bundle's merged diff against the architecture docs:
   `git diff develop...tranche/N --stat -- src apps schemas scripts tools .github`.
2. Map every changed path to its topic doc via the "source dirs" column of
   `../../architecture/README.md`'s index table, and update every touched doc so
   it describes the post-merge current state. Edits REPLACE outdated statements —
   never append history, changelogs, or "as of SD-NN" phrasing.
3. ALWAYS re-check `../../architecture/status.md` — stub graduations and newly
   landed capabilities are the most common change a bundle ships.
4. Refresh the `Last verified:` header line of every doc actually re-verified
   (date + short SHA of the closure tip).
5. Run the verification one-liners embedded in
   `../../architecture/README.md` §Maintenance contract (cited-path existence
   check + relative-link check) and fix anything they flag.

The full rules and procedure live in `../../architecture/README.md`
§Maintenance contract. This obligation blocks the develop PR the same way the
release-notes and version-increment obligations do.

## 7. Initial-package construction (operator-only, before launch)

Before this folder exists, the operator constructs the STC package from initial requirements at `~/workspace/programs/codex/requirements/SD-N-<slug>/`. The promotion skill (`release-package-promotion`, see `~/.hermes/profiles/god-emporer/skills/devops/release-package-promotion/SKILL.md`) takes the finished package and copies it into this folder deterministically. The workspace copy is not consulted again after the promotion.

Required canonical files (the promotion skill refuses to copy if any are missing):

- `README.md`, `scope-draft.md`, `loop-instruction.md`, `progress.md`
- `epic-breakdown.md`, `decisions.md`, `risks-and-open-questions.md`, `acceptance-and-verification.md`
- `content-unit-inventory.md`, `artifacts/`, `artifacts/README.md`, `references/`, `references/README.md`
