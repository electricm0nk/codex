# Bloodrager (#1) — Spellcasting Blocker Scoping

> Directed by the lead: check whether Bloodrager's spell list is a genuine
> reuse candidate (a `SPELLLIST:2|<X>|Bloodrager` token the way Oracle
> reused Cleric's) or needs its own independently-verified table the Oracle
> way; flag anything bloodline-specific that would complicate a bounded MVP.
>
> **Answer to the direct question: no reuse token exists.** The class line is
> `SPELLLIST:1|Bloodrager` — its own list, exactly one, no borrowed content.
> **But the standing "201 unique spell records, too big for a bounded slice"
> verdict is wrong on the specifics and wrong on the conclusion, and the
> reason is not the one anyone expected: there is no from-scratch ingestion
> here at all, and Bloodrager has no spellcasting below level 4.**

## The headline: Bloodrager cannot cast until 4th level

`acg_classes.lst` carries **no `CAST:`/`KNOWN:` row whatsoever for levels
1-3.** The table begins at level 4 (`CAST:0,1` / `KNOWN:0,2`) and the
caster-level token is itself gated:
`BONUS:CASTERLEVEL|Bloodrager|Caster_Level_Bloodrager|PRECLASS:1,Bloodrager=4`.
Spells, Blood Casting, and Eschew Materials are all granted at level 4.

The engine currently pushes
`class_feature.acg.bloodrager.spellcasting_deferred.unsupported` with
`claim_blocking: true` **unconditionally at every level** — see
`push_bloodrager_spellcasting_deferred_diagnostic` (`pilot_compute.rs:14076`),
called from all three exit paths of `ground_or_block_bloodrager_bloodrage`
(`:13985`, `:14006`, `:14064`), and documented in its own doc-comment as
"pushed unconditionally regardless of Bloodrage's own raging state."

**So a level 1-3 Bloodrager is currently claim-blocked for missing a
feature that RAW says it does not have yet.** Every other ACG/APG class
test in this codebase is anchored at level 1, which is exactly the band
where this is wrong. That is the cheapest and most honest half of task #1
and it needs no spell list at all.

## The spell list is far smaller than the "201 records" tag implied

> **Corrected 2026-07-27 (was 183, "100% grafts").** This section's own
> `CLASSES:Bloodrager=` substring grep only matches when Bloodrager is last
> in its comma group — the same bug later found and fixed on Witch (task
> #23). Real count is **200**: 183 `.MOD` grafts (as found here) **plus 17
> genuinely new, non-`.MOD` ACG spells** the substring grep couldn't see —
> so "100% grafts, no from-scratch ingestion cost" was wrong; there is a
> small new-content cost after all. Levels 1-4 stand corrected too:
> 55/49/54/42, not 47/44/51/41. See task #1's later correction (`0ca6fd89`)
> and `risks-and-open-questions.md` item 53.

Re-derived directly from `acg_spells.lst`:

| measure | real value |
|---|---|
| records naming `CLASSES:Bloodrager=` in ACG | **183** |
| of those, `.MOD` grafts onto existing spells | **183 (all of them)** |
| genuinely new spell records to ingest | **0** |
| unique spell names | 183 |
| spell levels present | **1st-4th only** (47 / 44 / 51 / 41) |

There is **no from-scratch ingestion cost** — unlike the Alchemist list
(13 new + 91 grafts), Bloodrager's list is 100% grafts. The "201" figure
does not match anything I can reproduce; the nearest real number is **220**,
which is the count across the *entire* PCGen tree including
`monster_codex`, `inner_sea_races`, `adventurers_guide`, and
`aquatic_adventures` — four books this repo does not ingest. Same
measurement-shape error as the Alchemist Ultimate-Magic hazard and the
ACG spell-list module's own corrected "145 → 144".

## But 40% of the list is unreachable in this repo's corpus

> **Corrected 2026-07-27**: against the true 200-entry list, 127 resolve
> and 73 remain unreachable (~36%, not 40%) — the 17 spells this doc
> missed were separately confirmed all corpus-reachable when the count was
> fixed, so the *unreachable* set itself (73) didn't change, only the
> denominator did. Per-level ACG counts corrected above to 55/49/54/42.

Matching all 183 names against the 1,075 spell keys actually ingested
under `data/corpus/`:

| spell level | in ACG | ingested here | missing |
|---|---|---|---|
| 1st | 47 | 28 | 19 |
| 2nd | 44 | 28 | 16 |
| 3rd | 51 | 29 | 22 |
| 4th | 41 | 25 | 16 |
| **all** | **183** | **110** | **73** |

I spot-checked where the missing base records live rather than assuming:
`Corrosive Touch`, `Frostbite`, `Shadow Weapon` → `ultimate_magic`;
`Ablative Barrier` → `ultimate_combat`; `Paragon Surge` →
`advanced_race_guide`. None of those three books is ingested here.

This is not a blocker — it is the exact situation the
`unresolved_spell_ids` / "not shown — outside demo corpus" idiom was
already built for (SWARM_TASKS rows for `647e52aa` / `5406e335`). But it
**must** be surfaced that way rather than silently dropped, and the
closure should state "110 of 183" plainly rather than implying full
coverage.

## Second real bug: the diagnostic's class-skill claim is false

The shipped diagnostic message asserts Bloodrager "has no class-skill
list." It does. `acg_abilities_class.lst:331`:

```
KEY:Bloodrager ~ Class Skills   CATEGORY:Internal
ABILITY:Class Skill|AUTOMATIC|Acrobatics|Climb|Craft|Handle Animal|
  Intimidate|Knowledge (Arcana)|Perception|Ride|Spellcraft|Survival|Swim
```

granted via `acg_abilities_globalvar.lst:85`. **Worth flagging how this is
encoded**: ACG class skills are *not* on a `CSKILL:` token on the class
line — `acg_classes.lst` contains zero `CSKILL:` tokens for any class — they
live on a separate `<Class> ~ Class Skills` internal ability as an
`ABILITY:Class Skill|AUTOMATIC|...` list. A `CSKILL:`-shaped search finds
nothing and would wrongly conclude the list is absent. (My own first pass
made exactly that mistake before I checked how Slayer's own list was
encoded.)

The list includes **all three of Climb, Intimidate, and Swim** — so this is
a **7th instance of the recurring class-skill-bonus widening bug**, the
same shape as Warpriest / Slayer / Brawler.

## Build-time hazards

1. **The leading `0` means something different here than it did for Oracle.**
   Oracle's `CAST:0,3` leading zero is the "orisons at will, no daily cap"
   sentinel. Bloodrager's `CAST:0,1` leading zero is a genuine **zero** —
   bloodragers get no 0-level spells at all, which is why `KNOWN:` also
   starts `0,2` and why the table tops out at 4th-level spells. Carrying
   Oracle's interpretation across would fabricate at-will cantrips.
2. **Max spell level is 4, not 9.** Only four spell columns ever appear.
3. **Caster level is zero below 4th** — gated on the class line itself, not
   derived. Don't compute a caster level for a level 1-3 Bloodrager.
4. **73 of 183 spells resolve to nothing here** — route through the existing
   unresolved-selection surface, never a silent drop.
5. **Bloodline bonus spells are real but entirely above the MVP band.**
   10 bloodlines (Aberrant, Abyssal, Arcane, Celestial, Destined, Draconic,
   Elemental, Fey, Infernal, Undead), each granting 4 bonus spells via
   `SPELLKNOWN:CLASS|Bloodrager=<spell level>`. The first is gated
   `if(Bloodrager_<BL>_BloodlineProgressionLVL>=7)` — verified directly, not
   assumed. All sit above the level-4 casting start, so they are safely
   deferrable, but they must be *named* in the narrowed diagnostic.
6. **The Elemental bloodline sub-gates on an element choice** —
   Fire/Earth/Water/Air select between four `Burning Hands` variants
   (`PREVAREQ:BloodragerElementalBloodline_<Element>,1`). A chooser inside a
   chooser, same shape as Oracle's Maneuver Mastery. Defer.
7. **Not all `SPELLKNOWN` records are bloodline ones** — 53 total vs the 40
   the 10 bloodlines account for. The remainder are archetypes, e.g.
   `Greenrager ~ Summoning Rager` (Summon Nature's Ally I at
   `BloodragerLVL>=6`). Filter on the bloodline records specifically rather
   than sweeping every `SPELLKNOWN:CLASS|Bloodrager=`.

## Recommended bounded MVP

In dependency order, cheapest and most honest first:

1. **Make the spellcasting diagnostic level-aware.** Below level 4 there is
   no spellcasting to defer; it should not claim-block. This alone is a real
   honesty fix and touches no table.
2. **Correct the diagnostic's false "no class-skill list" claim**, and
   ground the 11-skill list — 7th instance of the class-skill-bonus
   widening fix.
3. **Build the spells-known / per-day table for levels 4-20** (17 rows × 4
   spell columns). Note this is *easier* than Oracle's or Arcanist's: the
   rows are literal `CAST:`/`KNOWN:` tokens read straight off the class
   block, not formulas needing independent re-derivation. `MAX_SUPPORTED_LEVEL`
   for Bloodrager is already 20 (`class_bloodrager.rs:33`), so the chassis
   already supports the full range.
4. **Build the 110 reachable spell-list entries**, surfacing the 73
   unreachable through the existing unresolved-selection idiom.

Deferred and named: bloodline bonus spells (10 × 4), the Elemental element
sub-choice, archetype grants, Blood Casting, and Eschew Materials.

**Honest status expectation:** this does *not* reach Computed. Bloodrager
still has Fast Movement, Uncanny Dodge, Blood Sanctuary, Damage Reduction,
Greater/Tireless/Mighty Bloodrage, and the whole Bloodline slot unbuilt, so
it stays Blocked on a narrowed `other_features_deferred`-shaped diagnostic —
same posture as Oracle and Witch. `named_features_wired` goes 1 → 3
(Bloodrage + class skills + spellcasting), subject to your usual
cluster-collapsing call.

## Open questions for the lead

1. **Is the level-awareness fix in scope for #1, or its own item?** It is
   arguably a bug fix rather than a spellcasting build, and it is worth
   landing independently of the table work — a level 1-3 Bloodrager is
   being blocked for a feature RAW says it does not have.
2. **Is 110-of-183 coverage acceptable for the closure**, surfaced honestly,
   or would you rather the whole spell list wait until more books are
   ingested? My read is it clears the bar — the 110 are real, verified, and
   the gap has a shipped idiom for exactly this — but it is a 40% gap and
   that is your call to make, not mine to assume.
3. Does the false class-skill claim in the shipped diagnostic message want
   its own tracked row in `risks-and-open-questions.md`? It is user-visible
   text asserting something untrue about the corpus.
