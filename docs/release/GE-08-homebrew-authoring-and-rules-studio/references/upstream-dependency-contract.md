---
title: GE-08 Upstream Dependency Contract
stc_id: STC-CODEX-GE-08
artifact_type: reference
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/references
source_stc: ../README.md
---

# GE-08 Upstream Dependency Contract

## Objective
Record what upstream surfaces authorize for GE-08 and what they explicitly do not authorize.

## Dependency contract
| Upstream surface | GE-08 may rely on | GE-08 must not infer |
|---|---|---|
| `GE-00` program governance + quality doctrine | no-counterfeit-completion posture, anti-scope-creep posture, evidence gates, no-arbitrary-scripting instinct | implementation authority, plugin approval, or UX decisions |
| `GE-02` source STC + artifacts | canonical package/model homes, stable IDs, provenance/source-map obligations, diagnostics classes, authoring-versus-compiled-IR boundary, expression decision criteria | final editor UX, plugin ABI, runtime engine implementation, or broad schema finality |
| `GE-04` source STC + artifacts | compute/explanation/diagnostic truth that authored content must ultimately feed, preview/explanation expectations, structured diagnostics posture | that preview UX is settled, that engine semantics are already implemented for all authoring cases, or that authoring itself owns compute behavior |
| `GE-06` source STC | current integrated pilot proof boundary and current evidence ceiling for what later authored changes may reasonably target first | that the pilot is fully proven, that broad authoring claims are safe, or that UI/editor flow is already resolved |
| `GE-07` source STC | planning-ready presentation-layer authority for command-boundary, diagnostics/explanation visibility, and future editor/workbench surface requirements | final editor architecture, code authority for product-visible implementation, or any permission to let GE-08 own shell semantics |

## Operational rule
When a future GE-08 slice is proposed:
1. read this contract first
2. read the cited upstream surfaces second
3. refuse any handoff that smuggles unresolved GE-07 implementation details or unresolved GE-06 truth in through implication
