---
canonical: true
purpose: Patterns that may apply beyond where they were found. A sweep closes only when the corpus
  has been checked and the remaining count is stated — never when one instance is fixed.
---

# Sweeps

**The rule:** fixing the instance you found is not closing the sweep. State the corpus-wide count.

| # | Sweep | Found | Status | Corpus-wide count | Notes |
|---|---|---|---|---|---|
| S1 | **The Monk case — data present, dispatch missing.** Monk's chassis table was complete for months; only the string→ClassId mapping in `table_class_id` was absent. **How many of the 157 not-done classes are in this same state?** | code comment 2026-07-29 | **CLOSED wave 27 — swept, answer is ZERO** | **0 of 157** | Pattern fully exhausted by earlier waves: all 34 classes that have a real chassis table anywhere (CRB 11, APG 6, ACG 10, PU 4, UC 3) already dispatch. Verified by grepping every `rules_tables/<book>/` for orphaned ClassId/class_table infrastructure — none exists. **This is what a closed sweep looks like: a corpus-wide count, not a fixed instance.** |
| S2 | **Generalise S1 beyond classes.** If a class could be computable-but-unreachable, so could a race, a companion, a monster. Wherever a table and a dispatch are separate, the same gap can exist. | derived from S1, 2026-08-21 | NOT STARTED | unknown | Operator's own framing: "as with monk, make a note to check that with all the objects." Do NOT assume S1's answer generalises — measure each kind. |
| S3 | **Race-trait key matcher.** `modelled_race_of_race_trait()` requires the key segment before the first `~` to EXACTLY equal a bare race name, so a compound key ("Elf Shaman Hex Range Choice ~ Chant") reports `race_trait_race_not_modelled` though the race is named. Wave 22 fixed hyphen/space normalisation; the compound-key case is still open. | wave 19 | OPEN — re-named in waves 23, 24, 25, 26, 27 without closure | ACG share was 137; corpus-wide never derived | The canonical example of why this directory exists. Six waves of naming it. Fixing it is a RECLASSIFICATION, not a doneness gain — report separately. |
| S4 | **Scope-carrying tokens with nowhere to carry the scope.** Wave 17 shipped a widening for Amulet of Mighty Fists whose type had no field for "natural attacks only", while the live consumer applied the bonus to every weapon. Caught and reverted; re-opened at wave 18 because the re-land guarded one consumer path and missed the one the app actually calls. | waves 17/18 | PARTIALLY SWEPT | unknown | Sweep question: how many other emitted types assert a scope the consumer cannot honour? Never asked corpus-wide. |
| S5 | **Generators emitting unscreened fields.** `cache_gen::class_feature` screened `data.description` for declared PI but shipped `raw_tokens`' own `DESC` entry completely unscreened — it had shipped clean by luck, not construction. | wave 19 | FIXED for that generator; NOT SWEPT | unknown | Sweep question: does every generator screen EVERY field it emits, or only the visible one? |
| S6 | **Self-erasing fixture generators.** Three of four generators in wave 15 selected units by a status that stamping then overwrites, so a second run wrote an empty fixture and would have silently withdrawn every banked unit at the next regen. No gate caught it. | wave 15 | FIXED for those four; NOT SWEPT | unknown | Sweep question: is every generator in the repo idempotent? Prove with two byte-identical runs. |
| S7 | **Bar checks that count without checking identity.** Wave 16 review found a companion bar check asserting *how many* abilities but never *which*. | wave 16 | FIXED for that check; NOT SWEPT | unknown | Sweep question: how many gates assert arithmetic only? A gate that cannot detect wrong contents is close to a gate that cannot fail. |
| S8 | **`ClassFeatureData.class` read from key text.** Ships `class: "Sigilus"` where the true granting class is Magus, across ~12,247 records. Wave 22 fixed the same defect for grant data; this field was out of its scope. | wave 22 | OPEN | never measured — wave 22 found one instance incidentally | Needs a `data/corpus` regen. May cause an honest DECREASE if credit rests on a wrong class. |

## Wave 27's replacement finding — the 157 classified by real shape

S1 came back zero, but the census that answered it is the more useful artifact. The 157 not-done
`class` units are not one problem; they are five, and only one is cheap:

| Shape | Count | What it actually needs |
|---|---:|---|
| Prestige classes (58 CRB/APG + 19 Ultimate Psionics) | **77** | An entry-requirement gating mechanism that **does not exist anywhere in the codebase**. Independently confirmed by a sibling lane's CRB-10 finding. Not a wiring task. |
| Structurally not player classes | **48** | 33 Monster creature-type HD progressions, 7 Monster.Companion (incl. Eidolon), 3 Psionic power-list menus, 3 untyped edge records, 2 Vigilante identity records. **Operator ruling requested** — see `blocked.md` B4. |
| Real base classes with zero table | **18** | Antipaladin, the 6 Occult Adventures classes, the 10 Ultimate Psionics classes. Net-new table construction. |
| Books with no compiled rule set | **28** | adventurers_guide 25, inner_sea_magic 3. From-scratch book work. |
| CRB NPC classes | **5** | Adept, Aristocrat, Commoner, Expert, Warrior. Real but untabled. |
| **Near-miss: Ninja + Samurai** | **2** | Complete, tested, correctly-dispatched chassis already. Blocked ONLY by a missing `CLASS_WEAPON_PROFICIENCIES` row in `weapon_tables.rs` — a file outside the lane's grant. **Cheapest known units in the program.** |

**S9 (new sweep):** the Ninja/Samurai shape — complete chassis blocked by one missing row in a
downstream table. How many other units corpus-wide are one table row from working? Nobody has asked.

