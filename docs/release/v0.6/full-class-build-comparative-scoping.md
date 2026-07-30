# Full Class Build — Comparative Scoping Across the 9 Untouched ACG/APG Classes

> Directed by the lead after the Inquisitor Judgment closure: the
> single-ability cheap-win scan across all 16 ACG/APG classes is
> genuinely exhausted. The next real value is a FULL build (class-skill
> list + named features + spellcasting for casters) for one of the 9
> fully-untouched classes -- Arcanist, Investigator, Shaman, Slayer,
> Swashbuckler, Warpriest, Oracle, Summoner, Witch -- the same scale as
> the original Cleric/Druid/Sorcerer CRB builds, not another single-
> ability slice. This is a scoping comparison only, not a build --
> corpus-verified per the lead's explicit request for "a real cost
> comparison, not a guess."

## Method

For each of the 9 classes, verified directly against the real PCGen
corpus checkout (`~/workspace/repos/pcgen/data/pathfinder/paizo/
roleplaying_game/{advanced_players_guide,advanced_class_guide}/`,
commit `7f818006e371188e5717fd18d74d18a420747fc6`):

- **Spell-list reuse**: grepped each class's own `CLASS:<Name>` line for
  a `SPELLLIST:N|<other-class>` token (the same mechanism Skald's
  `SPELLLIST:1|Bard` reuse and Investigator's `SPELLLIST:1|Alchemist`
  were found through), then checked whether the referenced class's OWN
  spellcasting is genuinely built in this codebase yet (not just its
  chassis).
- **Class-skill-list size**: counted `CSKILL:` tokens in the class's own
  `Class Skills` (or equivalent) ability record in `*_abilities_class.lst`.
- **Named-feature count**: cross-checked against this session's own
  already-published `named_features_expected` counts in the SD-24 Epic 4
  coverage docs, re-derived independently via each class's own
  `KEY:<Class> ~ ...` record list.
- **Named-feature shape**: read every KEY record's own name to classify
  flat/numeric (favorable) vs. chooser-list-with-real-variety
  (unfavorable, mirrors the reasoning that already ruled out
  Investigator/Swashbuckler/Arcanist's Exploits/etc. from the
  single-ability slice).

## Findings table

| Class | Caster? | Spell-list reuse | Class skills | Named features | Dominant shape |
|---|---|---|---|---|---|
| **Arcanist** | Yes (INT, prepared, spellbook) | **Wizard** (`SPELLLIST:1\|Wizard`) -- Wizard's own spellcasting IS genuinely built (prepared, spellbook, byte-identical casting shape) | 8 (smallest) | 9 (tied-smallest full-scope class) | Arcane Reservoir (flat pool: max `3+level`, daily fill `3+level/2`) + Exploits/Greater Exploits (chooser-list, defer) + 2 capstone abilities |
| Investigator | Yes (INT, prepared, spellbook) | Alchemist -- **false shortcut**: Alchemist's own spellcasting is NOT built (only Mutagen is) | 21 (largest) | 95 (dominant outlier) | Inspiration (chooser-list, already flagged hardest of the batch in the single-ability scan) |
| Shaman | Yes (WIS, prepared) | None -- own list, fresh build | 12 | 10 | Spirit (domain-like choice) + Spirit Animal (could reuse Wolf-companion code, proven pattern) + Hex (chooser-list) + 2 archetype-variant features |
| Slayer | No | n/a | 17 | 15 | Quarry/Studied Target (opponent-dependent -- confirmed below the FULL build still hits this wall) + Sneak Attack/Trap Sense/Trapfinding/Track (flat, Trapfinding could reuse Rogue's own already-built code) + Slayer Talents (chooser-list) |
| Swashbuckler | No | n/a | 16 | 29 (second-largest) | Panache resource pool gating 20+ separate Deed abilities -- confirms the original scan's "resource-pool-plus-multiple-abilities" verdict holds at full-build scale too |
| Warpriest | Yes (WIS, prepared) | **Cleric** (`SPELLLIST:1\|Cleric`), same prepared-casting shape as Cleric (no `MEMORIZE:NO`) -- genuine reuse, but capped at 6th-level spells (own progression table needed, not byte-identical to Cleric's own 9th-level table) | 14 | 18 | Blessings (domain-like, 2 chosen) + Fervor (its own resource pool) + Channel Energy (own formula `1+max(0,min(20,LVL)-2)/3` dice, drawn FROM Fervor -- checked directly, NOT a clean copy of Cleric's own already-built `channel_energy_dice`/`channel_energy_uses_per_day` code) + Sacred Weapon/Armor (flat scaling) |
| Oracle | Yes (CHA, spontaneous) | Partial: `SPELLLIST:2\|Cleric\|Oracle` (list CONTENT reuses Cleric's, but Oracle is spontaneous -- `MEMORIZE:NO` -- unlike Cleric's prepared shape, so the ACCESS mechanic needs its own fresh known-spells-per-day table, a 9th-level full-caster table -- the biggest table-verification task of the 9) | 9 | 19 | Dominated by Mystery (10 choices) + Curse (5 choices) -- narrows via the same "pick one canonical option" trick that closed Cleric's domain, but the spontaneous 9th-level table cost remains regardless |
| Summoner | Yes (CHA, spontaneous) | None -- own (short) list, fresh build | 9 | 17 | Eidolon dominates: a whole separate creature with its own evolution-point-buy system, effectively a second character sheet -- large, standalone scope |
| Witch | Yes (INT, own shape) | None -- own list, fresh build | 11 | **7 (smallest count)** | Weapon Proficiencies (trivial) + Cantrips/Patron Spells (spellcasting, fresh) + Familiar/Familiar Touch Spells (needs a BRAND NEW familiar stat-block -- checked directly, no familiar stat-block code exists anywhere in this codebase; Sorcerer's own Arcane Bond only recognizes `bond:familiar` as an identity choice, never built a real stat block) + Hex (chooser-list) |

## Why Arcanist is the cheapest real full build

Witch has the fewest named features on paper (7 vs. Arcanist's 9), but
its ACTUAL cost is higher once reuse is accounted for: Witch needs a
fresh spell list, a fresh known-spells table, AND fresh familiar
stat-block infrastructure that doesn't exist anywhere in this codebase
yet (Cavalier's Mount/Hunter's/Druid's Wolf all reuse ONE shared
companion-stat-block pattern; there is no equivalent "standard familiar"
table to reuse or extend from). Arcanist has none of these gaps:

1. **Cleanest spell-list-and-shape match of any candidate, including
   Skald/Bard.** Arcanist is `SPELLSTAT:INT`, `MEMORIZE:YES`,
   `SPELLBOOK:YES` -- not just the same LIST as Wizard, the same
   CASTING SHAPE (prepared, spellbook-gated) that Wizard's own
   already-built infrastructure (`compose_character_input`'s Wizard
   starter-spell seed, the prepared-spellbook validation) was built
   around. Skald/Bard were a spontaneous-known match; Investigator/
   Alchemist would have been a prepared-extract match if Alchemist's own
   casting were built (it isn't). Arcanist is the first case this
   session where the reused class's spellcasting infrastructure is
   BOTH already-built AND an exact shape match.
2. **Smallest class-skill list (8).**
3. **Fewest named features among classes with a genuine reuse
   opportunity (9).** The one real gap, Exploits (and Greater Exploits),
   is a chooser-list -- but deferring an unbuilt chooser-list without
   blocking an otherwise-valid posture is the exact pattern that already
   let Bard/Wizard/Rogue reach `Computed` despite their own unbuilt
   chooser-lists (other bardic performances, Discoveries, etc.). Arcane
   Reservoir itself is a flat, choice-free resource pool (`max = 3+level`,
   `daily fill = 3+level/2`) -- no new engine state, no chooser gate.
4. **Genuinely plausible path to `Computed`, not just `Blocked` with a
   narrower diagnostic** -- unlike every APG/ACG closure so far this
   session (Cavalier, Alchemist, Skald, Bloodrager, Brawler, Hunter,
   Inquisitor all stay `Blocked`), Arcanist's shape (chassis + skills +
   full spellcasting + a flat resource pool, with only a chooser-list
   deferred) mirrors Wizard's/Bard's/Rogue's own `Computed`-reaching
   shape more closely than any prior APG/ACG closure has.

## What would still stay explicitly deferred, even in a full Arcanist build

- Arcanist Exploits / Greater Arcanist Exploits (chooser-list, real
  variety -- named but not built, same "grant-only identity record"
  idiom as Bard's other performances).
- Consume Spells / Magical Supremacy (each has a real formula, but
  lower priority than the core chassis/skills/spellcasting/Reservoir
  scope -- worth a follow-on slice, not blocking this one).
- Whether Arcanist's own spells-known/per-day numeric table is
  byte-identical to Wizard's (plausible per known PF1 rules, but -- per
  this session's own established discipline (Skald's tables turned out
  byte-identical to Bard's, discovered DURING the build, not assumed
  during scoping) -- this needs real verification at build time, not
  assumed here.

## Runner-up, if Arcanist turns out more novel than expected

**Warpriest** is the next-cheapest: a genuine (if partial) Cleric
spell-list/shape reuse, and Sacred Weapon/Armor are flat scaling bonuses.
Its scope is larger than Arcanist's (18 named features vs. 9, Fervor is
its own resource-pool mechanic, Blessings is domain-like x2, Channel
Energy needs its own formula rather than reusing Cleric's own already-
built code), so it is the second choice, not the first.
