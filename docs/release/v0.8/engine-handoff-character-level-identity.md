---
title: Engine handoff — "character level = sum of class levels" has no canonical exported home
status: open
severity: low (correctness fine today; drift risk across three independent copies)
found_by: backend teammate + orchestrator, v0.8 encounter-generator, ticket E-1
found_on: 2026-09-02
owner: the concurrent session that owns repo-root src/
scope_note: repo-root src/ is out of scope for this sprint (§4.2). Nothing changed; the bridge
  keeps a disclosed one-line copy.
---

# One rules identity, three independent copies

**"A character's level is the sum of its class levels, not the highest single class level."** That
is a PF1 rules identity, and it is the single input the encounter rating hangs on — get it wrong
and every difficulty tier is wrong while looking entirely plausible.

The engine states it and documents it, but **does not export it**:

| Location | Form | Note |
|---|---|---|
| `src/rules_core/skill_allocation.rs:633` | `fn character_level(input) -> u16` | **Canonical, and PRIVATE.** Its doc states the identity: "the sum, not any single class's level". |
| `src/rules_core/feat_prereqs/pre_tokens.rs:133` | inline in a struct construction | `u16` sum then `.min(u8::MAX)` |
| `apps/desktop/src-tauri/src/character_hub.rs:1930` | inline in a level-up preview | **Adds `+1`** for the level being gained — correct there, wrong to reuse |
| `apps/desktop/src-tauri/src/encounter_rating.rs:213` | inline fold (E-1, new) | `saturating_add` on `u8` |

## The ask

**Make `skill_allocation::character_level` public** (or move it somewhere more natural and export
it). One `pub` keyword lets `encounter_rating.rs` delete its copy, and gives the other sites
something to converge on.

## Why it matters even though nothing is broken

Correctness is fine today — the copies agree at every realistic input, and `qa` was asked to
confirm the `u16`-sum-then-clamp and `u8` `saturating_add` forms are semantically identical. The
problem is drift: the identity is stated in one place, implemented in four, and nothing fails if
one changes. The `character_hub.rs` variant already differs by design (`+1`), which is exactly how
a future reader picks the wrong one to copy.

This was caught because the orchestrator told `qa` to verify the bridge delegated to
`character_hub.rs:1930` "rather than summing class levels itself" — an instruction that was wrong
twice over: that site is not a reusable function, and reusing it would be off by one. The real
canonical home existed all along, one module away, private.

## Verified behaviour (not at risk, recorded for whoever changes this)

E-1 pins the identity against a **real build**, not a fixture: `create_character_at_root` a
Fighter 3, then `level_up_character_at_root` into a Wizard dip, and assert the encounter rating
sees **level 4 (sum), not 3 (highest)**. Any change to the canonical function should keep that test
green.
