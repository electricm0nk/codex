---
title: SD-18 — Decision Record ("Why we did that")
status: draft (operator review required)
date: 2026-07-12
companion_to: /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
---

# SD-18 — Decision Record

This file captures the deliberate choices made in the SD-18 planning conversation on 2026-07-12. Each item includes the decision, the alternative it displaced, and the reason the alternative was rejected. Future sessions that ask "why is SD-18 shaped this way" should find the answer here without re-litigating it.

## 1. Loop routing, not card-routed dispatch

**Decision:** SD-18's breadth-coverage work runs through an operator-driven claude-code loop, not the standard card-routing dispatch path (operator mints card → tech-priest executes → kanban completes).

**Displaced alternative:** Autonomous kanban workers dispatching each of 34 acceptance criteria as separate CODE slices.

**Reason:** Tranche-2 was supposed to deliver full PF1 core at L10 via autonomous dispatch and didn't. The autonomy pattern repeats the failure: 34 parallel autonomous workers collide on `pilot_compute.rs` and produce counterfeit completion. Loop routing concentrates the work under operator-driven execution with file-touch partitioning and self-healing at iteration boundaries.

## 2. Inheritance from the matured SD-13 model, not the as-written prompt

**Decision:** SD-18 inherits the *matured* SD-13 operator-loop model that emerged at the end of tranche-2-6, not the `sd13-class-uplift-loop-prompt.md` file as written.

**Displaced alternative:** Copy the as-written SD-13 prompt verbatim into SD-18's loop instruction.

**Reason:** The as-written prompt's "operator reviews every PR" was tightened by end of tranche-2-6 to "operator reviews PRs to develop; tranche-branch PRs auto-merge." The prompt file just never got patched. SD-18 inherits the corrected posture and the differences are listed in §6 of the scope doc. Duplicating the as-written text would re-introduce the wrong review posture.

## 3. Auto-merge to `tranche/3` per iteration

**Decision:** Each loop iteration creates a feature branch off `tranche/3`, lands the slice, and auto-merges back to `tranche/3`. Operator review happens at the `tranche/3 → develop` promotion PR, not at the slice PRs.

**Displaced alternative:** Operator review at every slice PR before merge to tranche.

**Reason:** Operator review at every slice PR is incompatible with the multi-day unattended-loop execution model. Tranche branches don't carry global-state risk the way develop does, and the matured SD-13 model has proven this posture workable. Operator review concentrates where it actually buys safety: at the promotion to develop.

## 4. Self-healing as a structural requirement

**Decision:** The loop self-heals wherever the failure is mechanically resolvable. Non-self-healable failures write to `## Open blockers` and exit `FAIL`. The loop does not stop on the first unresolvable problem.

**Displaced alternative:** The loop stops on any failure and exits.

**Reason:** Per operator directive 2026-07-12: "I much prefer to come back after 3 days and find a list of problems to address rather than to learn the process stopped after an hour because of something you could have handled on your own." The self-healing doctrine is the structural mechanism that produces the list-of-problems posture.

## 5. Kanban cards are post-mortem records

**Decision:** SD-18's `codex-tranche-3` cards are minted *after* each loop iteration completes a chunk of work, with `status=done`. The card body carries the merge receipt, branch SHA, cargo test summary, clippy signal, cycle timing, self-heals applied, and next-uplift recommendation — i.e. everything needed to reconstruct what happened in a 3-day-later audit.

**Displaced alternative:** Cards pre-minted as autonomous dispatch units (status=ready → running → done).

**Reason:** Per operator directive 2026-07-12: "All work needs to be logged in kanban so that we can do a post mortum if needed." Cards-as-records invert the relationship between kanban and loop: the loop drives execution, kanban captures audit-grade state.

## 6. Pre-loop consumer-side composition gate

**Decision:** §1.1 of the scope doc (consumer-side composition) is the *only* SD-18 card that ships via standard card-routing. All §3 work is loop-routed. The pre-loop card lands through tech-priest's normal execution model.

**Displaced alternative:** Mint all 34 acceptance criteria uniformly — either all loop-routed or all card-routed.

**Reason:** Consumer-side composition is structural new-subsystem work (bridging chosen-state with corpus-side into the compute path's input). Loop-routed execution is for matrix-coverage churn, not for designing new substrate. Splitting §1.1 (card-routed, structural) from §3 (loop-routed, churn) preserves each execution model's appropriate substrate.

## 7. Core Rulebook subtree only

**Decision:** SD-18's corpus target is `pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc` and its include graph. Bestiaries, Ultimate-*, Advanced Class Guide, Occult Adventures, Unchained, and homebrew are out of scope.

**Displaced alternative:** Cross-source scoping — every spell across every PF1 source, every equipment item across every source, with subsequent tranches deduplicating overlap.

**Reason:** Per the on-disk corpus investigation 2026-07-12: PCC/LST is partitioned by source book at the PCC level. The `core_rulebook.pcc` entry-point's direct includes are:
- `PCC:@/homebrew/conversion_support/conversion_support.pcc`
- `PCC:@/pathfinder/paizo/roleplaying_game/core_essentials/_core_essentials.pcc`
- 7 `PCC:@/pathfinder/paizo/roleplaying_game/core_essentials/races/<race>/_race.pcc` directives (Dwarf, Elf, Gnome, Half-Elf, Half-Orc, Halfling, Human)

This subtree carries 35 LST files in `core_rulebook/`, 40 LST files in `core_essentials/`, plus ~60 per-race LST files under `core_essentials/races/<race>/`. It does NOT include Ultimate Magic (a separate PCC at `pathfinder/paizo/roleplaying_game/ultimate_magic/_ultimate_magic.pcc`) or any Bestiary PCC. Source-partitioned structure means "tranche-N = core rules, later tranches = additional tomes" maps cleanly to corpus shape. A cross-source tranche would force synthetic cross-PCC queries the corpus doesn't natively support.

## 8. Re-scope from 21 matrix rows to 34 acceptance criteria

**Decision:** SD-18's acceptance criteria break down as: 1 pre-loop gate + 7 race rows + 11 class rows + 2 interaction rows + 9 spell schools + 4 equipment categories = 34 criteria. Spell cards and equipment cards are explicit acceptance criteria in their own right, not subset of matrix rows.

**Displaced alternative:** 21 acceptance criteria (matrix rows only), with spells and equipment addressed by the consumer-side composition slice alone.

**Reason:** A level-20 character with no equipment rows grounded is a math object. A wizard with no spell rows grounded doesn't cast anything. The corpus shapes the addition:
- `core_rulebook/cr_spells.lst` carries ~652 spell records (count derived from `SCHOOL:` tag matches). The PF1 strict-school partition names 9 schools plus Universal. Spell cycles are the natural coverage unit for the corpus-side reachability proof.
- The 4 core-rulebook equipment LST files (`cr_equip_arms_armor.lst`, `cr_equip_general.lst`, `cr_equip_magic_items.lst`, `cr_equipmods.lst`) are the corpus-natural categorization. Each gets its own coverage cycle.

Spells and equipment are load-bearing object kinds for what an end user actually does at the table. The 34 criteria cover the corpus + matrix surface honestly. (Note: the SD-13 matrix itself documents 7 races + 11 classes + 2 interaction rows = **20** matrix rows. The +9 spell + 4 equipment bring the SD-18 total to 33 loop-routed criteria + 1 pre-loop gate = 34.)

## 9. School-card vs class-card overlap is acknowledged, not avoided

**Decision:** Spell-school cards prove the *corpus side* — every spell in the school parses and is reachable. Class cards prove the *behavior side* — the class casts spells and gets the right effect. The same spell may appear in both, but each card covers a distinct field.

**Displaced alternative:** Treat school and class work as substitutes for each other; eliminate overlap by partition.

**Reason:** Eliminating overlap requires either splitting spells by class (which breaks corpus shape) or splitting classes by school (which breaks class shape). Both would distort the natural partition. Acknowledging the overlap and assigning distinct fields to each card is the honest read; it also produces more acceptance evidence per spell, not less.

## 10. Scope doc lives at workspace root

**Decision:** `/home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md` is the canonical handoff doc, not under `programs/codex/requirements/SD-18-core-rules-breadth/`. The bundle under `programs/codex/` is the *doctrine record*; the doc at the workspace root is the *execution reference*.

**Displaced alternative:** Single canonical doc at `programs/codex/requirements/SD-18-core-rules-breadth/scope.md`.

**Reason:** Per operator directive 2026-07-12: "move that md file to the workspace root." Pattern matches SD-13: `sd13-class-uplift-loop-prompt.md` (workspace root, read by the loop) and `sd13-remaining-class-rows.md` (workspace root, working state). The workspace root holds operator-facing working docs; `programs/codex/requirements/` holds doctrine. Two audiences, two locations.

## 11. §1.2 stays open in the scope doc

**Decision:** §1.2 ("any new canonical IR or contract artifact") in the scope doc is intentionally left unspecified. Whether it lands depends on what the §1.1 pre-loop slice reveals about the gap between SD17-E's types and `pilot_compute.rs`'s input.

**Displaced alternative:** Pre-specify the §1.2 work up front so the bundle is "complete" before the pre-loop slice ships.

**Reason:** Pre-specifying creates a default that may not match what the pre-loop slice finds. Leaving §1.2 open is honest about the dependency; if the loop or the pre-loop card discovers the gap, that's work that lands under a §1.2 card; if not, §1.2 closes empty. The bundle's STC remains in `risks-and-open-questions.md` so the open-status is visible.

## Cross-reference

The technical-design.md file in this bundle re-states the lane split, branch lifecycle, and card schema that implement these decisions. The scope doc at the canonical handoff path is the execution reference for the loop itself.
