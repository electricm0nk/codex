---
title: SD-18 — Risks and Open Questions
status: draft (operator review required)
date: 2026-07-12
companion_to: /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
---

# SD-18 — Risks and Open Questions

Risks the loop can self-heal (§4.2 of the scope doc); risks it cannot; open questions the bundle has not yet resolved.

## Self-healable conditions

These are recoverable conditions the loop handles inline without exiting `FAIL`:

| Condition | Detection | Self-heal |
|---|---|---|
| Feature branch diverged from `tranche/3` mid-iteration | `git fetch origin tranche/3` reveals new commits | `git rebase origin/tranche/3` in worktree, re-run tests, re-push, re-merge |
| Merge conflict in auto-merge | `git merge` reports conflicts | If mechanical (import ordering, unrelated additions), resolve inline; re-commit the merge |
| Cargo build cache corruption | `cargo build` reports stale state | `cargo clean`, rebuild |
| Disk pressure from `target/` | Worktree disk usage high | `rm -rf $WT/target` (per matured SD-13 §5 pattern) |
| Stale worktree from prior cycle | `git worktree list` shows orphaned paths | `git worktree remove --force` |
| Coordination file drift | Progress doc snapshot > 5 commits behind `tranche/3` | Read live matrix, refresh progress doc snapshot, retry |

## Non-self-healable conditions (write to `## Open blockers`, exit `FAIL`)

These are conditions the loop cannot resolve autonomously:

| Condition | Detection | Why not self-heal |
|---|---|---|
| Conflict requires a domain decision (e.g. which side wins on a class-feature semantics question) | Merge conflict has overlap on a question with no mechanical resolution | The operator must decide which semantics are canonical |
| Slice branch needs manual rebase | `git rebase` reports conflicts the auto-resolver cannot handle | Manual operator action required |
| Two live claude processes would both touch `pilot_compute.rs` | `ps -eo pid,etime,stat,cmd \| grep claude` shows multiple in-flight on the same file set | Mature SD-13 hard-stop rule; loop refuses to advance |
| Chosen burden requires a new subsystem (feat-prerequisite engine, spellbook engine, damage-total engine) | Slice scope requires extending `pilot_compute.rs` beyond the existing seam functions | New subsystem is a tranche-level decision, not a slice decision |
| Disk at 100% with no `target/`-strip remedy | `df` reports full disk; no worktree `target/` strips would free space | Disk pressure outside the loop's control |

## Bundle-level risks (not loop-cycle risks)

### Risk: §1.1 pre-loop slice reveals an SD17-E type gap

**Description:** The consumer-side composition slice (§1.1) is the first time the rules engine is asked to consume corpus-side records. If `pilot_compute.rs`'s input requirements don't match what SD17-E's `SourcePackageContent` carries, §1.1 reveals a gap between SD17-E and the compute path.

**Likelihood:** Medium. SD17-E was authored with the canonical IR substrate in mind, but §1.1 is the first integration test of that substrate against the engine.

**Mitigation:** §1.2 of the scope doc is intentionally open. If §1.1 surfaces a gap, §1.2 lands as a card-routed slice to author whatever is missing — most likely a new field on `SourcePackageContent` or a thin translation layer between corpus-side and compute-side. The bundle's STC remains in this file so the open-status is visible until §1.1 lands.

**Operator action:** When §1.1 ships, scan the slice's review for new contract artifacts surfaced by the integration. If any surface, mint §1.2 immediately.

### Risk: §3.4 strict-school partitioning doesn't match the corpus

**Description:** The scope doc names 9 PF1 strict schools (the canonical PF1 taxonomy). The corpus's `SCHOOL:` tags in `core_rulebook/cr_spells.lst` use sub-school granularity (Transmutation → Polymorph / Calling, Illusion → Figment / Glamer / Shadow / Phantasm / Pattern). Distribution of sub-schools across strict schools may not be uniform.

**Concrete corpus state:** Spell corpus at `core_rulebook/cr_spells.lst` carries ~652 spell records (count = 652 SCHOOL: tags). Sub-school keywords observed in the corpus include Transmutation (124), Evocation (87), Abjuration (73), Necromancy (62), Compulsion (54), Divination (46), Summoning (40), Creation (31), Polymorph (28), Healing (25), Glamer (19), Teleportation (11), Figment (10), Shadow (8), Calling (7), Charm (6), Universal (5), Phantasm (5), Scrying (4), Pattern (4). The strict-school partition maps sub-school keywords to strict schools: Conjuration = Summoning + Creation + Calling + Teleportation + Healing (~115); Illusion = Figment + Glamer + Shadow + Phantasm + Pattern (~46); Transmutation = Transmutation + Polymorph (~152).

**Likelihood:** High that the corpus uses sub-school keywords rather than strict-school tags; the strict-school distribution must be derived.

**Mitigation:** §3.4 of the scope doc names "9 strict schools" but defers per-school count derivation to the loop. The loop reads the live corpus via the LST parser pipeline (`src/pcgen_import/lst_parser/spell.rs:488`), derives the strict-school partition from sub-school keywords, and uses that to scope per-cycle work. The scope doc's "9 cards" expectation is honored; the per-card count may differ.

**Operator action:** None required; the loop handles derivation.

### Risk: Bestiaries / Ultimate-* / non-CR sources are out of scope for SD-18

**Description:** SD-18's corpus target is `core_rulebook.pcc` only. The Core Rulebook PCC's direct includes are restricted to `core_essentials.pcc`, the 7 core-race sub-PCCs, and the cross-source `homebrew/conversion_support/conversion_support.pcc`. The corpus therefore does not contain PF1's class features from Ultimate Combat (fighter armor mastery beyond L7), Ultimate Magic (alchemist discoveries, magus arcana), Advanced Class Guide (most classes), or any Bestiary (monster stat blocks).

**Likelihood:** Certain. CR is bounded; the operator's 2026-07-12 directive explicitly scoped to CR.

**Mitigation:** Acceptance criteria for class rows (§3.2) may land as `partial/Computed` for class features that live in bestiaries/Ultimate-*/Advanced Class Guide. The scope doc accepts `partial/Computed` only with explicit grounding artifacts. Future tranches carry additional sources.

**Operator action:** None during SD-18; SD-18's closure posture must honestly call out which class features remain ungrounded because they live outside CR. A "supported class surface" listing grounded features per class is a defensible closure artifact even where CR doesn't carry everything.

### Risk: Loop runs unattended for days without operator checkpoints

**Description:** Per operator 2026-07-12 directive, the loop runs unattended for the duration of its multi-day run. Self-healing handles mechanical failures; the operator returns to a list of `## Open blockers` if anything unresolvable happened.

**Likelihood:** Some `## Open blockers` are expected over a multi-day run. The risk is not "blockers exist" but "blockers accumulate undetected."

**Mitigation:** Progress doc's `## Open blockers` section is the operator's first read on return. `decisions.md` §4 captures the rationale. The operator's review cadence (§7 of the scope doc) covers a full audit at closure.

**Operator action:** Read `## Open blockers` first. Address what's addressable. Mint new tranches if the blocker is large enough to deserve its own scope.

## Open questions not yet resolved

- **Q1:** Does the operator want a per-school strict-school count file emitted at loop-end (one-shot stats artifact), or does the loop stay silent on per-school totals?
- **Q2:** Does the consumer-side composition slice (§1.1) require a per-class or per-race fixture file, or are runtime random-fixture generations sufficient for verification?
- **Q3:** What is the operator's review cadence during a multi-day run? The bundle assumes "no in-loop review"; if you want weekly checkpoints, that needs to be added to the loop instruction explicitly.
- **Q4:** Spell card overlap with class card — does the operator want spells enumerated in both cards (current posture) or de-duplicated with a forwarding reference? Current bundle accepts both schemas; per-cycle card body schema would clarify.

These do not block SD-18 from proceeding; they are decisions the loop instruction document author (operator) and the loop's first cycle's `decisions.md` revision can resolve as they come up.
