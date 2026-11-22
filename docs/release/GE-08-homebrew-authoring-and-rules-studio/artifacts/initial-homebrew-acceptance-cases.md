---
title: GE-08 Initial Homebrew Acceptance Cases
stc_id: STC-CODEX-GE-08
artifact_type: generated-artifact
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts
source_stc: ../README.md
related:
  - ./ge08-e1-minimum-proof-object-selection-2026-06-22.md
  - ./package-file-lifecycle-requirements.md
  - ./validation-and-preview-workflow-requirements.md
---

# Initial Homebrew Acceptance Cases

## Purpose
Name the first bounded cases that later GE-08 implementation work must satisfy.

## Selected bounded cases
1. **Homebrew feat proof case**
   - load the GE-06 Human Fighter level 1 deterministic pilot as the base case
   - substitute the Human bonus feat `Dodge` with the GE08-E1 homebrew feat-like authored object from `ge08-e1-minimum-proof-object-selection-2026-06-22.md`
   - prove the authored package validates and the bounded armor-class preview path remains inspectable without LST or plugins
2. **Validation-negative case family**
   - author malformed variants of the same package and prove the workflow refuses counterfeit success with actionable, machine-readable, claim-blocking diagnostics
   - the minimum refusal set is:
     - duplicate or missing `StableId`
     - missing or malformed `manifest.yaml` identity/dependency/proof-binding fields
     - bad armor-class effect target or broken feat-to-effect reference
     - malformed prerequisite structure if prerequisite parity is attempted
     - missing provenance required for explanation/export claims
     - widened unsupported formula/selector/plugin semantics for the first proof posture
   - each refusal case must name which stage failed (`load`, `validate`, `prepare`, `preview`, `explain`, or `export`) and which claims were blocked
3. **Explanation case**
   - inspect the explanation path from Human bonus feat slot -> authored feat object -> authored effect -> armor-class derived value
   - prove authored-package provenance survives the explanation path
   - prove blocked-path explanation remains visible when preview is refused
4. **Lifecycle case**
   - save, diff, export, re-import, and re-validate the authored package without losing package identity, authored object identity, or diagnostics
   - prove invalid or deferred packages are saveable as local source but refused for export and proof claims

## Anti-scope-creep rule
These cases are intended to prove honest value quickly. They do not authorize a broad studio build, public sharing system, plugin runtime, general formula editor, or product-visible editor implementation.