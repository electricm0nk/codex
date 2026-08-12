# Tester Workbench Surface Specification

## Objective
Define the required visible surfaces for the first bounded SD-11 tester workbench so later implementation can present real Codex truth, visible limits, and actionable feedback affordances without guessing.

## Required surfaces
| Surface | Purpose | Backing truth source | Must show | Must not imply |
|---|---|---|---|---|
| Session header | identify current build/channel/support posture and current bounded workflow | build metadata + operator/channel mapping + active workbench snapshot | build label, channel/support label, platform support tier, current bounded workflow name | broad product readiness or equal platform support |
| Bounded workflow panel | let testers exercise one real bounded flow | real Tauri command payload or visibly labeled placeholder/fallback surface | current state, key summary values, bounded scope messaging, workflow-specific actions | full character-builder breadth |
| Diagnostics and blocked-claim panel | let testers understand why something failed or is unsupported | diagnostic/explanation/provenance context from the snapshot | diagnostic severity/class, blocked claims, key explanation/provenance pointers | that unsupported behavior is just a generic bug |
| Feedback actions | let testers submit bug or enhancement issues with structured context | current workbench context + governed issue contracts | “Report bug” and “Request enhancement” actions, with issue-type distinction visible | unstructured free-form intake |
| Update/status panel | let testers understand current build/channel/support posture and later check/apply updates honestly | branch/channel mapping + future updater evidence | current build/channel/support state, update-available state, update outcome/failure state | raw git branch names as product UX |
| Unsupported-scope notice | preserve truth about what this workbench is not | source STC + current bounded workflow | concise visible limitations and the next higher-order boundary they belong to | that missing features are silently expected to work |

## First truthful bounded workflow rule
The first SD-11 tester workbench must anchor itself to exactly one real bounded workflow. It may begin narrower than SD-13 breadth, but it must satisfy all of the following:
- consumes real Codex data or a visibly labeled bounded placeholder during an intentionally transitional slice
- exposes enough output, diagnostics, and explanation context for a tester to understand what happened
- can feed structured bug/enhancement issue composition
- states clearly whether the workflow is a pilot character path, an authoring/workbench path, or another bounded proof slice

## Mandatory visible metadata
Every tester-visible workbench build must have a surface that can reveal or package at least:
- current build label/version
- current tester-facing channel/support label
- platform
- current bounded workflow identity
- current data source identity (real command result vs placeholder/fallback)
- current support-tier caveat when relevant

## Non-goals
- polished visual design system work
- silent hiding of unsupported semantics
- broad feature navigation beyond the bounded workbench needed for the first tester loop
