# SD-13 Core Class/Race Roster and Level-10 Progression Matrix Technical Design

## Purpose
This document describes the design response for the SD-13 breadth lane: how bounded breadth truth should be represented, how support states should combine with evidence tiers, how current repo truth constrains the design, and how later execution slices can expand coverage without faking combinatorial completion.

## Design posture
SD-13 is not a request to “implement all core classes and races.”

It is a request to establish the control plane that makes later breadth work honest. The design response therefore prioritizes:
- explicit roster identity
- explicit support-state taxonomy
- explicit evidence tiering
- explicit current-state seeding
- explicit adjacent-lane boundaries
- explicit decomposition that can grow without claiming false product breadth

## Current executable boundary truth as of 2026-06-30
The live repo proves only a narrow bounded slice today:
- the deterministic GE-06 fixture is `race:human` plus `class:fighter:1`
- `tests/ge06_pilot_total_saves.rs` computes and explains Fighter level-1 total saves, but claim-blocks Rogue level 1 and Fighter level 2
- `tests/ge06_pilot_combat_baseline.rs` computes and explains a deterministic Fighter level-1 combat posture, but claim-blocks Fighter level 2 and unsupported loadout mutations
- the repo README still states the project is not yet a general character builder or broad Pathfinder product

Design consequence:
- SD-13 must seed breadth truth from a real level-1 Human Fighter ceiling rather than pretending the repo already contains a latent core-roster implementation waiting to be toggled on

## Truth model

### 1. Support state and evidence are separate axes
A useful breadth control plane needs two axes:
1. support state — `supported`, `partial`, `lossy`, `blocked`, `unverified`
2. evidence tier — aligned to Codex quality-gate policy (`Observed`, `Parsed`, `Converted`, `Computed`, `Oracle-checked`, `Product-visible`)

Why this matters:
- a class can be `partial` at `Computed`
- a class can be `blocked` at `Observed`
- a race can be `unverified` even if it exists in source content
- a UI can be `Product-visible` for a surface that is still only `partial` or `lossy`

The control plane must keep those facts separate instead of collapsing them into one optimistic badge.

### 2. Row types are narrower than full combinatorial breadth
A fake breadth matrix would demand all 77 race/class combinations immediately.

The truthful design uses three row types:
- race-semantic rows
- class-progression rows
- named interaction rows

This permits:
- race truth to advance separately from class truth
- class truth to advance separately from every possible race/class combination
- targeted interaction rows where the combination itself matters materially

A product-visible breadth claim then depends on the necessary combination of these narrower rows rather than on a premature 77-row completion theater.

### 3. Interaction rows are explicit, not universal by default
Not every race/class combination deserves its own first-wave implementation slice.

Instead, interaction rows should exist when a real seam changes support truth materially, such as:
- Human bonus feat or ability-bonus interactions with prerequisite or class-feature selection
- race-specific trait interactions that alter class progression or derived outputs
- future race/class seams whose support state cannot be inferred from the separate race and class rows alone

This keeps the packet honest without detonating scope.

## Artifact architecture inside the packet

### `artifacts/core-roster-and-support-state-matrix.md`
Owns:
- exact roster identity
- support-state definitions
- evidence-tier relationship
- seeded current-state matrix
- breadth-claim gate summary

### `artifacts/level-10-progression-validation-contract.md`
Owns:
- universal level-10 progression dimensions
- class-family splits
- class-specific required semantics through level 10
- what later slices must prove before a class can move upward in support state

### `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
Owns:
- the visible debt surface
- minimum ledger fields
- seeded current-state debt rows
- rules for who must see which kinds of debt

### `artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md`
Owns:
- the SD-11 / SD-12 / SD-14 seam map
- what those lanes may consume from SD-13
- what SD-13 does not authorize for them

### `references/upstream-dependency-contract.md`
Owns:
- what upstream strategic, repo, and adjacent-STC surfaces authorize
- what they explicitly do not authorize

## Breadth-claim composition rule
A later user-facing breadth claim should be assembled from the matrix rather than authored ad hoc.

Minimum composition rule:
1. identify the race row needed for the claim
2. identify the class row needed for the claim
3. identify any named interaction row needed for the claim
4. require all participating rows to be in an acceptable state for the exact claim being made
5. surface visible debt when any participating row is `partial`, `lossy`, `blocked`, or `unverified`

This prevents phrases like “core support” from becoming folklore.

## Level-10 validation strategy
The design does not require one giant monolithic test sprint.

Instead, later execution should separate:
- matrix and taxonomy seeding
- race-semantic slices
- martial/skill-driven class progression slices
- spellcasting and hybrid class progression slices
- cross-cutting prerequisite/skill/derived-stat slices
- visible debt/reporting slices
- breadth-claim audit and evidence-refresh slices

This preserves truthful progress increments:
- a class may move from `unverified` to `partial`
- a dimension may move from `blocked` to `partial`
- a spellcasting family may remain visibly behind a martial family without corrupting the matrix

## Spellcasting design consequence
Spellcasting classes impose a different burden from Fighter-like chassis.

Therefore the design separates them deliberately in later execution planning.

Why:
- slot and known/prepared progression are independent truth surfaces
- class-specific spellcasting choices such as domains, schools, bloodlines, or related bounded core surfaces create extra semantic pressure
- a class can appear selectable while still being unusable or misleading without those semantics

The breadth lane must therefore preserve an explicit spellcasting burden table rather than allowing class names alone to stand in for support.

## Adjacent-lane integration posture
SD-13 outputs should flow outward, but ownership stays separate:
- SD-11 may consume support states and debt language for tester-facing visibility, but it does not redefine the states
- SD-12 may ship builds that expose broader roster claims, but shipping a build does not upgrade support truth automatically
- SD-14 may later need the matrix to judge saved-state compatibility for broader class/race coverage, but persistence continuity does not prove breadth support

## Prohibited shortcuts
The design explicitly rejects these shortcuts:
- dropdown presence as breadth proof
- parsed content as semantic support proof
- a single class-family slice being reported as “core classes complete”
- human-only or fighter-only proof being described as race/class breadth
- a lossy executable path being promoted silently to `supported`
- full 7 x 11 matrix theater before the narrower row model is even grounded

## Later execution boundary
No coding harness should act directly from this document.

Each later execution slice must receive a dedicated handoff that names:
- exact repo paths
- exact allowed write scope
- exact required reads
- exact verification commands
- exact non-goals
- exact roster slice and semantic burden it is claiming

## Completion gate
- [ ] the design keeps support state and evidence tier separate
- [ ] the design avoids fake 77-combination completion pressure
- [ ] the design names spellcasting as a distinct proof burden
- [ ] the design preserves SD-11, SD-12, and SD-14 ownership boundaries
- [ ] no part of the design acts as code authority by itself
