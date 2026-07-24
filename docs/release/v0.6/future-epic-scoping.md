# v0.6 Alpha — Future-Epic Scoping (Consolidated)

> Synthesis only, no new investigation. Pulls together risks items 1, 17, 18
> (and the sweeps that closed around them: 14, 20, 21) from
> `risks-and-open-questions.md` into one picture for the operator's eventual
> review. Every claim here was independently established somewhere in that
> document; this file adds no new facts, only the cross-item comparison.
>
> Written 2026-07-24 after a systematic sweep (item 21) confirmed the swarm
> is out of further silent-correctness bugs in the three working classes
> (Fighter/Wizard/Rogue) and the remaining distance to the alpha bar is now
> architecture-level, not bug-level.
>
> **Update, 2026-07-24 (later same day):** the operator directed gap B
> (feat-effects engine) be built — no longer purely deferred. See risks
> item 17's own operator-directive note for the go-ahead and starting
> scope. This file's sizing/analysis of gap B below still stands as the
> scoping baseline; only its "not attempted this wave" framing is
> superseded.
>
> **Update, 2026-07-24 (later still) — gap A's dedicated design pass is
> done.** `item-1-architecture-wall-design.md` reads the real call chains
> and finds the sizing below was too coarse: most of the AC/attack-bonus
> math already exists and is already wired (the desktop equipment tab
> uses it directly); ACP is a few-line gap, not a corpus-access problem;
> attack-bonus enhancement has a genuine data-model gap (no
> weapon-to-modifier attachment relationship in `EquipmentSelection`) that
> is separate from corpus access entirely; and — the biggest correction —
> bridging the corpus wall by itself would not let any additional
> equipment loadout reach `Computed`, since the posture gate checks exact
> `item_id`s independent of what the math could resolve. See that doc for
> the full three-way fix-shape comparison and recommendation.

## The three gaps, side by side

| | What's blocked | Why | Fix shape | Rough size |
|---|---|---|---|---|
| **A. Headless/corpus-aware wall** | Real attack-bonus math for arbitrary weapon enhancement; real armor-check-penalty for arbitrary armor in the skill-posture gate | The `Computed`/`Blocked` gate runs in a **headless** compute layer (`build_pilot_headless_receipt` / `compute_pilot_base_chassis`) that takes no corpus parameter. The real per-item values (`WeaponEnhancementBonus`, `ACCHECK:`) only exist via corpus-resolved `EquipmentRecord`s, which this layer structurally can't reach. The static, corpus-free `equipment_tables()` carries neither field. | Either (a) thread a corpus parameter through the whole headless chassis call chain (~347 call sites), or (b) move the `Computed`/`Blocked` decision itself to the already-corpus-aware layer (`pilot_compute_corpus.rs`'s `compute_pilot_with_corpus`) | Large — comparable to or bigger than the Rogue chassis-widening task, a real architecture decision (which layer owns claim-gating), not an implementation detail |
| **B. Feat-effects engine absence** | Every selectable feat except Power Attack/Dodge/Weapon Focus (hardcoded into one fixed posture) has zero computed mechanical effect — e.g. Toughness doesn't add HP | There is no general feat-effects computation anywhere in `pilot_compute.rs`. Not disconnected math like (A) — the math itself was never written. Confirmed system-wide by grep: zero `selected_feats` references beyond the 2 hardcoded presence-checks. | A new subsystem: a per-feat effect resolver wired into HP/attack/skill/save totals for the 185-record CRB feat catalog | Large — new subsystem, not a wiring fix; scales with how many of the 185 feats need real effects vs. a smaller "top N commonly-selected feats" slice |
| **C. Wizard non-Human spell math absence** | Spell-save-DC grounding and the level-3 spellbook-ceiling enforcement never run for any non-Human Wizard | `explain_wizard_level1_prepared_spell_baseline` — the one function carrying both — returns immediately for any `race_id != HUMAN_RACE_ID`. Everything in it is currently reasoned in Human-specific terms. Chassis math (BAB/saves/HP) is unaffected; it's a separate, already race-agnostic path. | Widen the function to all races, which needs per-race re-verification of the same rigor already applied to Human (not a mechanical find-replace) | Medium — bounded to one function's worth of formulas, but the re-verification labor scales with race count |

## Why these three are grouped together (and what's already excluded)

All three are **completeness gaps, not correctness bugs** — the same distinction the lead drew explicitly when declining to fix item 18: a Fighter can select Toughness and reach `Computed` even though it does nothing (item 17); an Elf Wizard reaches `Computed` with no spell-save-DC computed at all rather than a wrong one (item 18); arbitrary gear shows correct AC in the desktop equipment tab already, just not through the compute-gate's own math (item A). None of the three produce a silently *wrong* number the way items 14 and 20 did before they were fixed — that category was swept clean (item 21) and is not what's being scoped here.

**Already excluded from this scoping, deliberately:**
- **Class-skill-list recognition** (the other half of item 1's original skill-posture sizing) — confirmed **not** blocked by wall (A). It needed hand-authored per-class data, not corpus access, and was fixed for Wizard/Rogue without touching the architecture question (items 14/20). If this scoping is used to plan a wall-(A) fix, don't expect it to buy anything here — it's already done.
- **AC itself** — the original trigger for finding wall (A) — was deliberately dropped as a standalone slice this swarm; the existing desktop equipment tab already shows correct AC for arbitrary gear via the corpus-aware path, so leaving the compute-gate's own AC math unwired is close to a no-op today, not an open user-facing gap. It's folded into wall (A) below only because a future fix to (A) would also close this cleanly, not because it's independently pressing.
- **Size-modifier wiring** (AC/attack/CMB/Stealth for Small races) — noted in item 16's addendum as overlapping wall (A) but not separately sized; treat as riding along with whichever attack-bonus/AC work eventually happens, not a fourth line item.

## Which gaps would a wall-(A) fix actually unlock

This is the question most likely to matter for prioritizing: if wall (A) is bridged, does it close gap (A) only, or does it help (B) and (C) too?

- **Attack-bonus enhancement math and skill-ACP**: **yes**, directly — these are exactly the two remaining sub-problems wall (A) blocks. Bridging the boundary (either fix shape) is the actual unlock for both at once, not two separate slices.
- **AC gate wiring**: yes, same mechanism, though as noted above the user-visible gap is already closed by the equipment tab.
- **Feat-effects engine (B)**: **no** — a fixed feat like Toughness needs a computed +3 HP rule, which has nothing to do with corpus access; the feat data itself (name, description) is already fully available headlessly. Bridging (A) buys nothing here.
- **Wizard non-Human spell math (C)**: **no** — this is a scope boundary in one function's own logic (`race_id` check), not a data-access problem; the function already runs headlessly for Human. Bridging (A) buys nothing here either.

**Bottom line for planning purposes:** wall (A) is a single, well-defined architecture decision whose payoff is bounded to two specific sub-problems (attack-bonus, skill-ACP), both already fully scoped above. Gaps (B) and (C) are independent efforts with no shared prerequisite — they can be picked up in any order, or in parallel with a wall-(A) fix, without waiting on each other.

## Recommendation

If the operator wants to close distance to the alpha bar's item 3 ("select feats... meaningfully") and item 4 ("spell slot allocation... matches PCGen" for any race), gaps (B) and (C) are the direct path — no architecture prerequisite, start whenever. Wall (A) is the more expensive, more architecturally consequential piece: worth a dedicated scoping/design pass before committing to either fix shape (corpus-threading vs. moving the gate), given the blast-radius estimate is comparable to the largest widening task already done this swarm. None of the three is a blocker for the others; sequencing is a pure prioritization call, not a technical dependency, except where noted above.
