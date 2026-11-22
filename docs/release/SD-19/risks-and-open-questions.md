---
title: SD-19 — Risks and Open Questions
status: draft (operator review required)
date: 2026-07-14
companion_to: /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md
---

# SD-19 — Risks and Open Questions

This file enumerates the risks, blockers, and open questions specific to SD-19. It is structured to mirror SD-18's `risks-and-open-questions.md` so a future operator reading both bundles can navigate them with the same mental model.

## Open override flags (require operator decision before capability slice ships)

These two flags were defaulted to a defensible choice (see `decisions.md` §2 and §3) because the operator did not provide an answer when asked. Each is a real architectural call that affects downstream artifacts. Override before the capability slice ships if the default is wrong; cost of override is 10 minutes of file patching per flag.

### Flag A — Equipment-id ↔ corpus-record matching convention

**Default chosen**: `equipment_id_resolve()` documented resolver with normalized `EquipmentRecord.name` as primary index and verbatim `KEY:` as secondary. See `decisions.md` §2 for the full rationale.

**Override alternatives**:
1. Raw `KEY:` token — every `CharacterInput.item_id` carries the exact corpus `KEY:` string verbatim. Forces every existing SD-18 fixture to change (`LONGSWORD_ITEM_ID = "item:longsword"` at `pilot_compute.rs:2283` becomes `"Longsword"`). Two identity namespaces inside the same compute path is worse than one bounded resolver.
2. Display-name match on `EquipmentRecord.name` only — same namespace problem plus cross-source collision risk.
3. Defer equipment entirely — contradicts the operator's stated directive that both §3.4 and §3.5 are in-scope.

**Override cost**: ~10 minutes. Touches `decisions.md` §2, `technical-design.md` §2.1, `acceptance-and-verification.md` gate 2(b).

### Flag B — Spell-content selection surface on `CharacterInput`

**Default chosen**: Top-level `spells_selected: Vec<SpellSelection>` with `acquisition_mode` field. See `decisions.md` §3 for the full rationale.

**Override alternatives**:
1. Class-scoped — each `class_levels` entry gets `.spells_selected`. Breaks `ChosenCharacterState` shape asymmetry (equipment top-level, spells nested).
2. Prepared-only at first — `prepared_spells: Vec<(class_id, spell_id)>`. Smallest possible first cut but locks in a known refactor when spontaneous-caster math lands.
3. Defer the choice — adds a tranche-level decision into SD-19's own scope; contradicts the operator's stated deliverable.

**Override cost**: ~15 minutes. Touches `decisions.md` §3, `technical-design.md` §3.1.

## Self-healable conditions (resolve inline, exit GREEN)

The SD-19 loop inherits SD-18's self-healing posture (see SD-18's `risks-and-open-questions.md` and the loop brief's `## Self-healing posture` section). SD-19-specific additions:

| Condition | Detection | Self-heal |
|---|---|---|
| Working tree dirty at cycle start | `git status --porcelain \| wc -l` returns non-zero | `git stash` (if previous unfinished attempt) or `git checkout -- .` (stray edit noise); re-verify clean; retry |
| Resolver normalization edge case surfaces mid-cycle | RED test asserts `equipment_id_resolve` returns `None` for a known-good fixture | Extend the normalization rule in `equipment_resolver.rs`; add the fixture's normalization pattern to the test suite |
| A cycle discovers a corpus record with a malformed `KEY:` token | RED test asserts `spell_id_resolve` returns `None` for a corpus record the corpus-side asserts is present | **NOT self-healable** — see non-self-healable list below |

## Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Detection | Why not self-heal |
|---|---|---|
| Corpus-derived contribution cannot be grounded for a school or category (corpus-side defect, missing corpus record, etc.) | RED test cannot be made green by extending the seam | The fix is corpus-side repair, which is out of scope for SD-19 (corpus-side work is SD-17's lane). The cycle writes to `## Open blockers` and the operator decides whether to route the corpus-side fix to SD-17 or accept the partial landing. |
| Slice branch needs manual rebase | `git rebase` reports conflicts the auto-resolver cannot handle | Manual operator action required |
| Two live `claude` processes would both touch `pilot_compute.rs` or `support_state_matrix.rs` | `ps -eo pid,etime,stat,cmd \| grep claude` shows multiple in-flight on the same file set | Structural: one-lane-at-a-time rule (per `technical-design.md` §6.1) |
| Progress doc and live matrix disagree on a row's `evidence_tier` (not just stale snapshot) | `support_state_matrix.rs` says `Supported/Product-visible` but the progress doc's row status is `open` (or vice versa) | Manual operator reconciliation required |
| `cargo test --tests` regresses on a row other than the one the cycle touched | Full suite regresses after a cycle's change | Sibling-preservation is a hard rule (per SD-18's loop brief, inherited) |
| A cycle's RED test depends on a corpus record that does not exist in the real PCGen corpus | RED test fails for "spell not found" / "item not found" after the corpus is queried against `CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data` | Corpus-side work is out of scope; the cycle writes to `## Open blockers` and exits FAIL |
| A cycle's RED test depends on a CRB table cell that does not exist in the foundation slice's table store | RED test fails because the `TableCellRef` lookup returns None for a corpus record that the table store should know about | The foundation slice is incomplete; the cycle writes to `## Open blockers` and the operator decides whether to extend the foundation slice or accept the partial landing |

## What does NOT risk SD-19 closure

- The SD-18 loop running concurrently — SD-19's `technical-requirements.md` §6 explicitly requires the SD-18 loop to pause during the SD-19 slice window, but SD-19 does not require SD-18 to be `done`.
- Spellbook engine / slot math / spell save DCs — explicitly out of scope per `decisions.md` §1.3 and `technical-design.md` §1.3. Their absence is not a closure blocker; their later addition is a future SD-N's deliverable.
- Equipment effect computation beyond the bounded baseline AC + attack bonus + max dex + spell failure that the per-cycle grounding pattern asserts — explicitly out of scope per the SD-18 §3.5 investigation cycle's findings. SD-19 grounds the corpus-derived contribution shape; it does not promise to compute every equipment effect that the corpus's records describe.

## Open architectural questions deferred to future SD-N

These were intentionally not answered by SD-19 because they are out of scope:

1. **Slot math.** Once the seam exists and the SD-19 loop has exercised it, the operator decides what slot math (spells prepared per day, bonus slots from high ability, DCs) is needed. SD-N+1 candidate.
2. **Equipment effect breadth.** Beyond the bounded baseline (AC, attack bonus, max dex, spell failure), the corpus describes many more equipment effects (weight, material, special abilities). SD-19 grounds the reachability shape; future cycles add per-effect grounding.
3. **Cross-source equipment identity.** The `KEY:` secondary index in `equipment_id_resolve` handles the basic collision case (`"Healing Potion"` appearing in core_rulebook and Ultimate Magic). A more sophisticated cross-source lookup (with explicit source-book qualification) is a future SD-N's deliverable if the operator decides it is needed.
4. **Homebrew equipment support.** SD-19 is core-rulebook only, matching SD-18's scope (`SD-18-core-rules-breadth/decisions.md` §7). Homebrew equipment is future SD-N.

## Cross-reference

- `decisions.md` §1.3 (what the seam does NOT do) — the spellbook engine / slot math / DCs question.
- `technical-design.md` §1.3 (what the seam does NOT do, restated with concrete file:line pointers), §6.5 (self-healing posture additions), §6.6 (hard stops additions).
- `acceptance-and-verification.md` — closure gates.
- `~/workspace/SD-18-core-rules-breadth-progress.md` the dated cycle-2026-07-15T0300 (§3.4) and cycle-2026-07-15T0400 (§3.5) headers — the blocker entries SD-19 exists to close.