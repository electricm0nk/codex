# Hunter (#43) — Spellcasting Scoping

> Scoped before build, per the four questions in the brief. Every table and
> count below re-derived from the corpus with a real `CLASSES:` parse (not
> substring matching).
>
> **Headline: this is the first class whose `SPELLLIST:N` union is fully
> satisfiable from already-shipped data — no new spell-list module at all.
> But the union is not a simple merge: 27 spells appear on both source lists
> at different levels, and that collision rule decides correctness.**

## Q1 — `SPELLLIST:2|Druid|Ranger` semantics: union, with two shipped precedents

`SPELLLIST:N|<list>…` means *"this class's spell list is composed of these N
named lists"* — a **union**, confirmed against both existing precedents in
this codebase rather than assumed:

- **Skald** `SPELLLIST:1|Bard` — the single-list case; Skald borrows Bard's
  list directly.
- **Oracle** `SPELLLIST:2|Cleric|Oracle` — the two-list case, implemented via
  `cleric_spell_list::cleric_spell_level(spell_id)`, with the Oracle-specific
  portion explicitly deferred because no such content existed.

**Hunter is the first case where *both* named lists are already built** —
Druid (271) and Ranger (114), both corrected and expanded this session
(#29, #26). Nothing needs ingesting.

| measure | value |
|---|---|
| Druid list | 271 |
| Ranger list | 114 |
| **Union** | **300** |
| overlap (on both lists) | 85 |
| **overlap at *different* levels** | **27** ← see hazard 1 |
| **castable by Hunter (levels 0-6)** | **255** |
| out of Hunter's range (levels 7-9) | 45 |

## Q2 — Table and grant level: **Hunter casts from level 1**

The brief expected casting to start above level 1. **It doesn't** — the
class table carries `CAST:`/`KNOWN:` rows from level 1, unlike Bloodrager
(which genuinely has none below 4).

Full verified table (max spell level **6**):

| lvl | CAST | KNOWN | | lvl | CAST | KNOWN |
|---|---|---|---|---|---|---|
| 1 | 0,1 | 4,3 | | 11 | 0,5,4,4,2 | 6,7,6,5,4 |
| 2 | 0,2 | 5,4 | | 12 | 0,5,5,4,3 | 6,7,6,5,5 |
| 3 | 0,3 | 6,5 | | 13 | 0,5,5,4,3,1 | 6,7,6,6,5,3 |
| 4 | 0,3,1 | 6,5,3 | | 14 | 0,5,5,4,4,2 | 6,7,7,6,5,4 |
| 5 | 0,4,2 | 6,5,4 | | 15 | 0,5,5,5,4,3 | 6,7,7,6,5,5 |
| 6 | 0,4,3 | 6,5,5 | | 16 | 0,5,5,5,4,3,1 | 6,7,7,6,6,5,3 |
| 7 | 0,4,3,1 | 6,6,5,3 | | 17 | 0,5,5,5,4,4,2 | 6,7,7,7,6,5,4 |
| 8 | 0,4,4,2 | 6,6,5,4 | | 18 | 0,5,5,5,5,4,3 | 6,7,7,7,6,5,5 |
| 9 | 0,5,4,3 | 6,6,5,5 | | 19 | 0,5,5,5,5,5,4 | 6,7,7,7,6,6,5 |
| 10 | 0,5,4,3,1 | 6,6,6,5,3 | | 20 | 0,5,5,5,5,5,5 | 6,7,7,7,7,6,6 |

**The leading `0` is the at-will-orisons sentinel (Oracle reading), not
Bloodrager's genuine zero.** Reasoned, not assumed: `KNOWN:4` at level 1
means the hunter *knows four 0-level spells*, which would be meaningless if
`CAST:0` meant zero daily casts. Same resolution Oracle reached; opposite of
Bloodrager's.

## Q3 — Shape: spontaneous (Sorcerer/Oracle), **not** the Known-backs-Prepared machinery

`MEMORIZE:NO` makes Hunter **spontaneous**. The shared
Alchemist/Investigator prepared machinery — and Witch's Known-backs-Prepared
reuse of it (item 57) — is the **wrong shape here.**

The right precedent is **Oracle**: spontaneous, divine, `SPELLSTAT` on a
non-INT stat, its own per-level `KNOWN` table, and a `SPELLLIST:2` union.
Mirror `unmet_oracle_known_spell_conditions` / `ground_oracle_known_spells`,
substituting the Druid∪Ranger membership check for Oracle's Cleric one.

## Q4 — Summon Nature's Ally auto-knowns: yes, separate handling

`KNOWNSPELLS:Summon Nature's Ally I|…|VI` grants those six automatically,
**on top of** the `KNOWN` table's counts. Verified levels:

| spell | Druid | Ranger |
|---|---|---|
| SNA I-IV | 1, 2, 3, 4 | same |
| SNA V, VI | 5, 6 | — |

**Internally consistent:** the corpus grants exactly I-VI and stops. SNA
VII-IX are Druid 7-9, outside Hunter's 6-level ceiling — so the auto-grant
list is already correctly bounded and needs no filtering of its own.

**Note the token has two forms in this corpus** — Witch's
`KNOWNSPELLS:LEVEL=0` ("all spells of level 0") versus Hunter's
`KNOWNSPELLS:<named spells>` ("these specific spells"). Same token, different
grammar; don't assume one parser path handles both.

## Build-time hazards

1. **The 27 different-level overlaps decide correctness.** Of 85 spells on
   both lists, 27 carry different levels — e.g. Protection from Energy
   (Druid 3 / Ranger 2), Glide (Druid 2 / Ranger 1), Tree Stride (Druid 5 /
   Ranger 4). **Take-the-lower is the coherent rule and fits most cases, but
   it is not "always use Ranger's":** `Detect Poison` is **Druid 0 / Ranger
   1**, running the other way. So a naive "prefer the Ranger level" or
   "prefer the Druid level" is wrong in one direction or the other.
   **This needs an independent RAW cross-check before building** — I am
   deliberately not asserting the rule from memory, since a wrong choice
   here mis-levels 27 spells silently. The union figures above use
   take-the-lower as the working assumption, flagged as such.
2. **45 of the 300 union spells are levels 7-9 and uncastable.** Hunter's
   ceiling is 6. The list must be filtered, or a Hunter will appear able to
   know spells she can never cast.
3. **The leading-`0` sentinel** (hazard 2 from the Bloodrager pass, opposite
   resolution here). Carrying Bloodrager's reading across would strip the
   hunter's orisons; carrying Oracle's is correct.
4. **Both source lists were corrected *this session*** (Ranger 51→114, Druid
   169→271). Build against the current modules, and do not reconcile against
   any pre-correction count in an older doc.

## Recommended scope

Mirror Oracle's spontaneous shape with a Druid∪Ranger membership function
over the two shipped modules; ground the 20-level CAST/KNOWN table; save DC
`10 + spell level + WIS`; and grant the six SNA spells as automatic knowns
on top of the table. **No new spell-list module, no ingestion.**

**Honest expectation:** this closes Hunter's last named gap. Hunter's other
remaining items (Hunter Tactics, Improved/Greater Empathic Link, Master
Hunter) are separate; whether it reaches Computed depends on its
`other_features_deferred` diagnostic, which this slice does not clear on its
own.

## Open question

**Only hazard 1**, and it genuinely gates correctness: what is the rule for
a spell appearing on both lists at different levels? Take-the-lower is my
working assumption and matches the direction of most of the 27, but
`Detect Poison` proves it isn't a simple list-preference rule. Worth
resolving from a primary source before backend writes the membership
function, since the failure is silent.
