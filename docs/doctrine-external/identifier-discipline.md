---
title: Identifier Discipline
status: stub (operator directive 2026-07-19)
scope: docs/doctrine-external
artifact_type: stub
canonical_source: ~/workspace/governance/identifier-discipline.md
---

# Identifier Discipline (stub)

This file resolves the `../doctrine-external/identifier-discipline.md` cross-references in `docs/release/SD-NN/*.md`. The canonical content lives at `~/workspace/governance/identifier-discipline.md` in the operator's workspace and is NOT bundled in this repo.

A cold-cloud-clone harness: stop and request the canonical governance bundle from the operator. The stub below records only the doctrine-name so the cross-references resolve.

## Authoritative headline

> Source-code identifiers describe WHAT the artifact does, NOT which release or spec domain it came from. PascalCase for functions/methods/constants/properties/Tauri commands; lowercase camelCase for variables. Forbidden patterns are tracked in Honcho duracon "Identifier discipline: forbidden source-identifier patterns." Skill `~/.hermes/profiles/<profile>/skills/devops/identifier-discipline/SKILL.md` carries the audit + rename cycle.

## Recorded

Created 2026-07-19 during the `docs/release/` tree seed (operator directive 2026-07-18).

## TODO (2026-07-21, SD-25 criterion 1.1)

The canonical file gained a new section, "Documented exclusion class: real test/fixture-file citations in comments (SD-25 1.1)", recording that a doc-comment/string-literal citing a real `tests/...` file by name (test-traceability grounding, e.g. in `src/rules_core/support_state_matrix.rs`) is NOT an identifier-discipline violation even though it matches the RED grep's lexical pattern — only the *identifier itself* carrying a bundle-tag prefix is a violation. A future sync of this stub should pull that section in verbatim so cold-clone harnesses see it without needing operator workspace access. See `docs/release/SD-25-ui-evaluation-defect-closure/artifacts/epic_1/identifier-audit-cycle_receipt.md` for the worked example.
