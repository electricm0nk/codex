---
canonical: true
purpose: Structural work that unblocks many units at once. Ranked by units-per-unit-of-effort, not
  by size. This is where wave targets come from.
---

# Levers

| # | Lever | Unblocks | Status |
|---|---|---|---|
| L0 | **Prestige-class entry-requirement gating.** Does not exist anywhere in the codebase. 77 of the 157 not-done classes need it — every CRB prestige class (Arcane Archer, Assassin, Duelist, Shadowdancer, Eldritch Knight...) plus APG's and Ultimate Psionics'. The single largest structural gap found by the wave-27 census. | 77 classes, and every feature they grant | NOT STARTED |
| L1 | **Class chassis.** Only 11 classes can be built (the CRB base classes). `class` is 28 done of 185. Every prestige class, every ACG/APG class, every archetype hangs off this. Wave 26 proved features for unbuildable classes compute correctly and reach nobody. | gates `class_feature`, which is 60% of everything remaining | **S1 SWEPT, zero Monk-shaped left.** Remaining work is L0 (77 prestige), ≥20 untabled base classes (corrected wave 28, was 18 — see `sweeps.md`), 28 unstarted books. **`class` (157 units) and `race` (60 units) were the two kinds NO wave-28 lane examined — `race` in particular has no census on file at all; worth a dedicated look before the next class-chassis push.** |
| L2 | **Domain-power grounding past its allowlist.** `ground_or_block_cleric_domain_power` grounded only Good and Healing because each domain needed a hand-written function. Wave 25 salvage added War and Strength. With the interpreter, the allowlist should become unnecessary. | every Cleric/Inquisitor domain in every book; 15 of Bestiary 6's 18 class_feature units alone | IN PROGRESS |
| L3 | **Monster-chassis ↔ companion-ability bridge.** Named by the Bestiary 6 ledger as a general fix appearing across many books. | 28 units (sized wave 28 via a grep-based exact-string cross-reference of `companion::external_ability_refs` — 377 distinct strings — against `monster_ability` corpus_keys; down from "unknown") | NOT STARTED — now sized |
| L4 | **Race-trait flat-override compute seam.** Landed for three races in wave 25 salvage; race_trait moved for the first time in six waves. | race_trait, 234 units currently correctly held back with a proven seam pattern (G6, wave-28 census) — 21 races/heritage variants named individually, each needs its own `explain_<race>_flat_override_race_trait` seam, per-record-verified before crediting (post-Undine discipline) | IN PROGRESS |
| L5 | **Option-pool catalog scaling.** Produced +109 from Rage Powers alone — the largest single class_feature win so far. ~10,000 units sit in pools no grant parser can reach because they are chosen, not granted. Constrained by ruling 18: open pools only. | up to ~10,000, minus exclusive pools; wave 28 census: class_feature's own G1 alone is 3,347 units/817 pool names, only 6 names (161 units) classified OPEN vs EXCLUSIVE so far | PARTIALLY BUILT (2 of 27 pools registered) |
| L6 | **The 36 grant facts with no DESC at all.** Their rules text lives on the class progression table, not a per-feature description. No `%N` mechanism will ever reach them. | **corrected wave 28: the real population is ≥1,764 within `class_feature` alone** (class_feature's own G3, magnitude-bearing units with no consumer function, is 2,583; of these, a shallow-but-unverified scan suggests roughly 1,764 lack a %N-reachable description entirely — flagged as needing a real re-derivation, not the "36 known" figure this lever was filed at) | NOT ANALYSED |
| L7 | **(new, wave 28) Template-application mechanism.** PF1e creature/familiar TEMPLATES (Ogrekin, Animated Object, Dread Lord, Celestial/Fiendish, Mana Wastes Mutant, Fungal Creature, Petitioner...) have no single stat block for the current per-creature chassis model to hold. | **≥479 units** (corrected wave 28 via adversarial review from a filed ≥547 — 430 monster_ability + 49 companion, only 2 of the pile's 3 kinds checked, so this is a floor) | NOT STARTED — a genuinely new engine mechanism, not data transcription |

## Levers that turned out not to be

* **Bulk description ingest** (wave 19 thesis) — REFUTED. "not-ingested" means the engine emits no explanation naming the record, not that text is missing. The records already exist with real prose.
* **Generic roster without grant data** (wave 20) — REFUTED, returned GAMED. The emission loop is generic; the data it needs was not.
