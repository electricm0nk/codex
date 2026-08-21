---
canonical: true
purpose: Structural work that unblocks many units at once. Ranked by units-per-unit-of-effort, not
  by size. This is where wave targets come from.
---

# Levers

| # | Lever | Unblocks | Status |
|---|---|---|---|
| L0 | **Prestige-class entry-requirement gating.** Does not exist anywhere in the codebase. 77 of the 157 not-done classes need it — every CRB prestige class (Arcane Archer, Assassin, Duelist, Shadowdancer, Eldritch Knight...) plus APG's and Ultimate Psionics'. The single largest structural gap found by the wave-27 census. | 77 classes, and every feature they grant | NOT STARTED |
| L1 | **Class chassis.** Only 11 classes can be built (the CRB base classes). `class` is 28 done of 185. Every prestige class, every ACG/APG class, every archetype hangs off this. Wave 26 proved features for unbuildable classes compute correctly and reach nobody. | gates `class_feature`, which is 60% of everything remaining | **S1 SWEPT, zero Monk-shaped left.** Remaining work is L0 (77 prestige), 18 untabled base classes, 28 unstarted books. See `sweeps.md`. |
| L2 | **Domain-power grounding past its allowlist.** `ground_or_block_cleric_domain_power` grounded only Good and Healing because each domain needed a hand-written function. Wave 25 salvage added War and Strength. With the interpreter, the allowlist should become unnecessary. | every Cleric/Inquisitor domain in every book; 15 of Bestiary 6's 18 class_feature units alone | IN PROGRESS |
| L3 | **Monster-chassis ↔ companion-ability bridge.** Named by the Bestiary 6 ledger as a general fix appearing across many books. | unknown; recurs across books | NOT STARTED |
| L4 | **Race-trait flat-override compute seam.** Landed for three races in wave 25 salvage; race_trait moved for the first time in six waves. | race_trait, ~2,980 not done | IN PROGRESS |
| L5 | **Option-pool catalog scaling.** Produced +109 from Rage Powers alone — the largest single class_feature win so far. ~10,000 units sit in pools no grant parser can reach because they are chosen, not granted. Constrained by ruling 18: open pools only. | up to ~10,000, minus exclusive pools | PARTIALLY BUILT (2 of 27 pools registered) |
| L6 | **The 36 grant facts with no DESC at all.** Their rules text lives on the class progression table, not a per-feature description. No `%N` mechanism will ever reach them. | 36 known, likely many more corpus-wide | NOT ANALYSED |

## Levers that turned out not to be

* **Bulk description ingest** (wave 19 thesis) — REFUTED. "not-ingested" means the engine emits no explanation naming the record, not that text is missing. The records already exist with real prose.
* **Generic roster without grant data** (wave 20) — REFUTED, returned GAMED. The emission loop is generic; the data it needs was not.
