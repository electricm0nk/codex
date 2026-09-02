---
title: v0.8 UI catch-up sprint — closeout
status: complete
branch: tranche/14-ui
date: 2026-09-01
---

# Closeout

Per brief §6, this sprint stops when the dispatchable punch list is exhausted. It is: every
remaining item in `scout-gap-audit.md` is an engine gap or gated behind one, and repo-root `src/`
is out of scope (§4.2). No PR, no merge to `develop`, no architecture-docs refresh — those are
separate operator decisions.

**Final state:** working tree clean, 18 commits on `tranche/14-ui`, every commit individually
passing its own verification.

| Suite | Result |
|---|---|
| `npm run typecheck` (apps/desktop) | 0 errors |
| `npm test` (apps/desktop) | 110/110 test files |
| `npm run tauri:check` | clean |
| `cargo test` (src-tauri) | 605 passed / 3 failed — all three named below |

The three reds are accounted for and none is sprint-caused:
1. `corpus_ingest_diagnostic::...license_artifacts` — pre-existing corpus count drift (1271 vs
   1267), reproduced on a clean worktree carrying no sprint code.
2–3. The two B-4 choice-trait tests, **deliberately left red** as the definition of done for
   blocker B13. They go green untouched when the engine lands its fix; do not weaken or `#[ignore]`
   them.

## Shipped — 24 tickets

**Creation flow.** Standard racial traits show on race pick (F-2). The nine bio fields typed at
creation persist instead of being discarded, with a bio-save failure reported as a warning rather
than a false creation failure (F-1, F-14). Class preview shows BAB and base saves at the chosen
level (F-11). The aging preview the form never submitted is gone, along with its TS rules table
(F-12).

**The sheet.** Real base-vs-total saves replace a placeholder (F-4). Selected traits render (F-3).
Melee attack bonus is shown (F-9). Export works from the sheet menu (F-8). Two dead surfaces are
deleted: the `Overrides` tab and the Attack panel's permanently-empty "Spell Res." tile (F-5, F-13).

**Equipment.** Gear beyond arms and armor is buyable — rope, rations, holy symbols (F-6). Rows show
cost and weight before purchase, with a null rendering nothing rather than a misleading zero
(F-7, F-15). Items can be stowed and re-equipped (B-5).

**Bridge.** Ability scores come from the engine, fixing odd scores displaying a point low
(B-2). Skill allocations are read back instead of re-seeding a constant every open (B-3). Traits
can be added and removed (B-4, flat traits). Class choices can reach the engine at creation, with a
caller entry replacing the seeded default rather than being silently ignored (B-6). Equipment
weight is exposed and proven against encumbrance's independent read (B-8). Spellcasting status is
reported from the corpus token, which let the UI delete its hand-assembled caster list
(B-9, F-16, F-10).

## Escalated — three engine handoffs

1. **`engine-handoff-trait-choice-save.md` (B13) — user-facing data loss, live today.** Any
   character with a `%LIST` trait (Criminal, Fiend Blood, Harvester) plus a chosen skill **cannot be
   saved at all**. `trait_effects.rs:517` emits a 3-segment `choice_set_id`; `local_store.rs:254`
   rejects anything but 2. The shipped create form sends that payload today. Two one-line candidate
   fixes are written up. **This is the most urgent item on the list.**
2. **`engine-handoff-skill-points-ingestion.md` — data dropped in translation.** Class skill points
   live on a second `CLASS:<Name>` continuation line in `cr_classes.lst` that the importer never
   captured; all 28 `STARTSKILLPTS` lines reach the corpus as nothing. This is why
   `characterProgression.ts:11`'s hand-authored `CLASS_SKILL_POINTS` — a PF1e rule in TypeScript —
   cannot yet be deleted. Worth checking what else those 28 records lost.
3. **`engine-handoff-wizard-specialization-gate.md` — 8 of 9 wizard schools unsaveable.** Only
   Evocation/{Necromancy, Transmutation} produces a saveable wizard. Invisible today only because
   the engine silently seeds that triple.

Plus the 12 blockers scout filed (B1–B12) and the engine gaps in
`choice-pool-resolver-audit.md`.

## The finding that shaped the sprint

**"Resolves" is not "computes."** A choice the engine recognises can still claim-block the save.
B-7 found roughly 66 of 73 Cleric domains blocking; B-11 found 8 of 9 wizard schools blocking, and
in doing so **corrected B-7's own earlier estimate** — which had inferred support from grounded
explanation rows rather than from executing the save path.

This is why no class-choice picker was built. A picker offering choices that produce unsaveable
characters is the stub-shape the no-stub doctrine targets: it demos perfectly and fails on use.
`list_wizard_school_options` was still built and committed, because it discovers support by probing
the engine rather than declaring it — so it widens automatically when the engine does.

## What made the output trustworthy

- **Implementers refused defective instructions.** Four of the orchestrator's ticket texts were
  factually wrong about this codebase (weight on `list_equipment`, skill points on the class
  catalog, the equipment category enum, the `(Base)` weight semantics). Each was caught by checking
  the source rather than complying, shipped honestly half-done, and turned into a real ticket.
  The orchestrator's instructions were the least reliable artifact in the sprint.
- **A teammate disclosed a §3.3 violation in its own already-passed work** (F-10's hand-assembled
  caster list) when silence would have cost it nothing. That disclosure became B-9, fixed at the
  data source, with a regression test that pins the code's *refusal to special-case*.
- **QA on a different model verified claims, not descriptions.** It instrumented a parity test to
  prove it compared 3309 rows rather than trivially passing; re-derived a corpus survey from raw
  JSON rather than through the code under review; traced a data-loss argument across three layers;
  and confirmed a sanctioned red test failed for the *right* reason. It refused five times to let
  unreviewed code ride along in a diff it was approving.
