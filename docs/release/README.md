---
title: Release Packages
status: active (operator directive 2026-07-18)
scope: docs/release
artifact_type: index
target_repo: https://github.com/electricm0nk/codex.git
purpose: "Each release bundle's full handoff package lives under this subtree. A release workflow script copies one of these folders whole into `repos/codex/docs/release/<bundle-id>/` to seed or refresh the bundle's repo-local surface."
layout_rule: |
  One folder per SD-N bundle, named exactly SD-NN.
  Each folder contains a `README.md` (bundle index), the canonical handoff
  artifacts (scope-draft.md, workflow-instruction.md, progress.md pointer, the
  src-STC pieces), and an `artifacts/` directory populated per-cycle by the loop.
  Older / closed bundles follow the same shape with a thin-pointer README
  rather than a full mirror.
nav:
  upstream_workspace_root: ~/workspace/
  upstream_authority_root: /home/workspace/programs/codex/requirements/ (operator-workspace absolute)
  upstream_progression_doc: ~/workspace/SD-NN-...-progress.md
---

# docs/release/

Each subfolder is a release bundle's repo-local handoff package. Layout is uniform across bundles so a release-workflow script can copy any one of them whole into a clean clone without rewriting anything.

## Folders

- `template/` — `template.md` is the canonical shape every new `SD-NN/README.md` follows. Copy it when minting a new release folder.
- `SD-22/` — CLOSED bundle (all 31 criteria complete 2026-07-20; branch `tranche/5`, board `codex-tranche-5`; merged to `develop` as PR #325, commit `f5e2b62`; version `0.5.96`).
- `SD-21/` — CLOSED bundle (operator directive 2026-07-17; branch `tranche/4-1`; 7 epics / 30 criteria; snapshot as of 9206ad0).
- `SD-20/` — CLOSED bundle (per-character rules engine; landed on `tranche/4` then promoted to `develop`; closure cycle 7 `integration:epic_wiring_closure` 2026-07-17).
- `SD-19/` — CLOSED bundle (corpus-aware compute seam; landed on `tranche/3`).
- `SD-18/` — CLOSED bundle (core-rules breadth; landed on `tranche/3`).

## Cross-reference

- The strategic/intake authority for every bundle lives at `/home/workspace/programs/codex/requirements/<bundle-id>/` in the operator's workspace. The repo-local copy here is the cold-cloud-clone surface — the same doctrine, with the operator's home-directory paths rewritten to repo-relative paths where required. See [`../doctrine-external/spec-domain-lifecycle.md`](../doctrine-external/spec-domain-lifecycle.md) for the lifecycle shape.
- The operator's workflow-instruction files at `~/workspace/SD-NN-...-workflow-instruction.md` are the editor of record; the repo-local copies here are a one-way sync from them. Dispatch is via the `Workflow` tool invoked from a live session (per `docs/governance/workflow-instruction-template.md §2`), not `/loop /batch` — the latter cannot run unattended.
- **Do not confuse `/home/workspace/programs/codex/requirements/` above (operator-workspace-only, doesn't exist in a git clone) with this repo's own `programs/codex/requirements/` directory (git-tracked, real, at the repo root).** They share a path suffix but serve different purposes: the operator-workspace one is the planning-doc intake authority described here; this repo's own `programs/codex/requirements/` holds only SD-13/16/17's legacy pre-`docs/release/`-convention `artifacts/`, frozen historical record — nothing new goes there.
- **[`../architecture/README.md`](../architecture/README.md)** is the living architecture-doc tree — current-state system documentation (compute engine, desktop app, release pipeline, persistence, etc.), distinct from this subtree's per-bundle release narratives. It is refreshed at every SD closure under its own closure obligation (see that file's §Maintenance contract), not copied or seeded by the release-workflow script described below.
- **`release-notes.md` lives at `docs/release/SD-NN/release-notes.md`** — a regex-locked CI/schema contract (`^docs/release/[^/]+/release-notes\.md$`) consumed by `tools/release/`, `scripts/release/`, and the `publish-tester-release.yml` workflow, alongside every other doc in this bundle's folder. **Corrected twice on 2026-07-20**: it first moved from `programs/codex/requirements/<bundle-slug>/` to here (its intended-but-missed location per this file's own release-script instructions, which never listed `release-notes.md` among the operator-workspace `programs/codex/requirements/` mirrorable artifacts), was briefly reverted back to `programs/codex/requirements/` on a mistaken belief that path was a permanent separate contract, then relocated here for good per full-consolidation operator directive — updating the schema regex, every CI workflow/script that enforced the old pattern, and the desktop app's own duplicate hardcoded regex. ~25 already-published `update-manifest.json` files on the live `update-index` branch still reference the pre-2026-07-20 path with a locked hash; not retroactively rewritten (that branch is CI-only-write).

## How a release script uses this subtree

A `tools/release-workflow.sh`-style script should be able to:

1. Take a `bundle_id` argument (e.g. `SD-22`).
2. Run `mkdir -p docs/release/<bundle_id>` (idempotent — `template/` already exists).
3. Copy `template/template.md` to `docs/release/<bundle_id>/README.md` and front-fill the index links from the operator's `~/workspace/SD-<bundle_id>-...-scope-draft.md`.
4. Copy the mirrorable artifacts (`scope-draft.md`, `workflow-instruction.md`, `decisions.md`, `technical-requirements.md`, `epic-breakdown.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, `technical-design.md`) from `~/workspace/programs/codex/requirements/<bundle_id>/` — strip `~/workspace/...` paths and rewrite to `../<sibling-bundle>/...` form.
5. Create `docs/release/<bundle_id>/artifacts/` (empty; loop populates per cycle).
6. NOT touch `docs/release/<bundle_id>/progress.md` — that file is the loop's owned surface and is created on first cycle.
