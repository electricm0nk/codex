---
title: GE-08 Homebrew Authoring Surface Specification
stc_id: STC-CODEX-GE-08
artifact_type: generated-artifact
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts
source_stc: ../README.md
---

# Homebrew Authoring Surface Specification

## Purpose
Define the minimum ordinary-homebrew surfaces Codex must eventually expose before the project can claim it replaced routine LST editing.

## Minimum ordinary authoring surfaces
- package manifest surface
- authored object identity and object-kind surface
- structured fields for effects, prerequisites, formulas, and choice records where applicable
- validation/diagnostic surface
- preview/explanation surface
- save/diff/import/export lifecycle surface

## Minimum ordinary edit verbs
- create package
- open/edit package
- add/edit bounded object
- add/edit bounded rule record
- validate package
- preview package effect on a bounded target
- inspect explanation/diagnostics
- save/export package

## First-proof rule
The first proof surface must be narrow. It should prove that a user can change one bounded rule outcome safely without requiring a broad studio environment.

The selected first proof case is the GE08-E1 package-local feat-like authored object documented in `ge08-e1-minimum-proof-object-selection-2026-06-22.md`, riding one bounded GE-06 pilot variant and one armor-class preview family.
