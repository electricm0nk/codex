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
  artifacts (scope-draft.md, loop-instruction.md, progress.md pointer, the
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
- `SD-22/` — PLANNING bundle, awaiting cycle launch (operator directive 2026-07-18; branch `tranche/5`, board `codex-tranche-5`; 9 epics / 31 criteria including Epic 9 — Closure Readiness added 2026-07-19).
- `SD-21/` — CLOSED bundle (operator directive 2026-07-17; branch `tranche/4-1`; 7 epics / 30 criteria; snapshot as of 9206ad0).
- `SD-20/` — CLOSED bundle (per-character rules engine; landed on `tranche/4` then promoted to `develop`; closure cycle 7 `integration:epic_wiring_closure` 2026-07-17).
- `SD-19/` — CLOSED bundle (corpus-aware compute seam; landed on `tranche/3`).
- `SD-18/` — CLOSED bundle (core-rules breadth; landed on `tranche/3`).

## Cross-reference

- The strategic/intake authority for every bundle lives at `/home/workspace/programs/codex/requirements/<bundle-id>/` in the operator's workspace. The repo-local copy here is the cold-cloud-clone surface — the same doctrine, with the operator's home-directory paths rewritten to repo-relative paths where required. See [`../doctrine-external/spec-domain-lifecycle.md`](../doctrine-external/spec-domain-lifecycle.md) for the lifecycle shape.
- The operator's loop-instruction files at `~/workspace/SD-NN-...-loop-instruction.md` remain the launch strings (because `/loop /batch /goal` resolves them as absolute paths). They mirror the repo-local copies here as a one-way sync from the operator's editor of record.

## How a release script uses this subtree

A `tools/release-workflow.sh`-style script should be able to:

1. Take a `bundle_id` argument (e.g. `SD-22`).
2. Run `mkdir -p docs/release/<bundle_id>` (idempotent — `template/` already exists).
3. Copy `template/template.md` to `docs/release/<bundle_id>/README.md` and front-fill the index links from the operator's `~/workspace/SD-<bundle_id>-...-scope-draft.md`.
4. Copy the mirrorable artifacts (`scope-draft.md`, `loop-instruction.md`, `decisions.md`, `technical-requirements.md`, `epic-breakdown.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, `technical-design.md`) from `~/workspace/programs/codex/requirements/<bundle_id>/` — strip `~/workspace/...` paths and rewrite to `../<sibling-bundle>/...` form.
5. Create `docs/release/<bundle_id>/artifacts/` (empty; loop populates per cycle).
6. NOT touch `docs/release/<bundle_id>/progress.md` — that file is the loop's owned surface and is created on first cycle.
