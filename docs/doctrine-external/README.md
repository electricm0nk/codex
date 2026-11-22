---
title: Doctrine (External / Operator Workspace)
status: stub (operator directive 2026-07-18; resolving relative cross-references in `docs/release/SD-NN/*.md`)
scope: docs/doctrine-external
artifact_type: stub
target_repo: https://github.com/electricm0nk/codex.git
purpose: |
  This directory exists in the repo so that `../doctrine-external/<doc>.md` cross-references in
  `docs/release/SD-NN/*.md` (mirrored from operator-workspace `~/workspace/governance/`)
  resolve to a real path. The doctrine files themselves do NOT live here — they live at
  `~/workspace/governance/` in the operator's workspace. A cold-cloud-clone harness
  operating without `~/workspace/governance/` access: stop and request the canonical
  governance bundle from the operator.
nav:
  operator_workspace_path: ~/workspace/governance/
  spec_domain_lifecycle: ../doctrine-external/spec-domain-lifecycle.md
  identifier_discipline: ../doctrine-external/identifier-discipline.md
---

# docs/doctrine-external/

**Not to be confused with `docs/governance/`** (sibling directory): `docs/governance/` holds this repo's own real, in-repo canonical doctrine content (`workflow-instruction-template.md`, `no-stub-mvp-doctrine.md`, `wired-integration-stubs-registry.md`) — actual files, not stubs. This directory (`docs/doctrine-external/`) holds only thin resolver stubs for doctrine that genuinely lives outside the repo, at `~/workspace/governance/` on the operator's machine.

## Purpose

Relative-link resolver for `../doctrine-external/<doc>.md` references in `docs/release/SD-NN/*.md`. Content lives upstream; this stub keeps the cross-references resolvable in a cold cloud clone.

## What lives here

Per-target stub files for governance files frequently cross-referenced:

- `spec-domain-lifecycle.md` — spec-domain lifecycle doctrine; governs every SD's closure flow.
- `identifier-discipline.md` — identifier-discipline doctrine; governs identifier cleanup cycles across every SD.

Each stub says: canonical content lives at `~/workspace/governance/<file>.md`; this stub exists only to resolve relative links in the release-package mirrors.

## Why a stub and not the actual files

Three reasons:

1. **Cold-cloud-clone portability.** A fresh `git clone` of this repo into a cloud harness gives the harness access to everything inside `repos/codex/`. Anything *outside* that boundary (operator's `~/workspace/`) is not accessible. The governance files frequently contain operator-internal notes, audit-ID conventions, and cross-program references not safe for general clone access.
2. **Authority separation.** Governance files are operator-policy artifacts; release-package files are bundle-scoped artifacts. Mixing them blurs the surface.
3. **Selective mirroring when ready.** If and when an operator-pinned directive lands governance files inside the repo (e.g. `docs/doctrine/governance-mirror/spec-domain-lifecycle.md`), the structure is ready.

## What to do if you are a cold-clone operator

Stop and request the canonical governance files from the operator. They live at `~/workspace/governance/` and are not bundled in this repo.

If you have been provided an updated authoritative governance bundle (or this is the operator's own workstation), keep this stub as-is.

## Recorded

Created 2026-07-19 during the `docs/release/` tree seed (operator directive 2026-07-18 "seed `/docs` with historical records"; Epic 9 doctrine addition 2026-07-19). Resolves broken `../doctrine-external/` relative links that the upstream-mirrored files in `docs/release/SD-21/` and `docs/release/SD-22/` carry after sed-rewriting `~/workspace/governance/` → `../doctrine-external/`.
