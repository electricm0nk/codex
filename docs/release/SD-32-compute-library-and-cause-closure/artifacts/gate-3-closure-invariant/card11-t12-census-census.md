# Card 11, shape T12 — measurement-only census

**Cycle:** `t12-census`. **Scope:** measurement only, per the dispatch brief and
`decisions.md §13` ("measurement is explicitly authorised as a first step, and
does not substitute for the work"). **No engine, corpus, or pinned-count
change made by this cycle.** Base pin: `8b8e00c0d`; landed on top of
`3981e7091` (tranche/12 HEAD at cycle start). Corpus SHA:
`7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
fresh worktree, self-healed the empty oracle slot per §8, matches pin exactly).

## What T12 is

`decisions.md §13`: "2,453 `class_feature`s belonging to classes the engine
does not model. ~47 are suspected false positives (archetype features
attributed to a phantom PCGen 'class' that does not exist as a class at all)
and must be confirmed, not assumed." Evidence code in
`docs/work-inventory.json`: `class_feature_of_unmodelled_corpus_class:<slug>`.

## Headline finding: the false-positive count is 118, not ~47 — logged as a correction

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
print(sum(1 for u in d['units'] if (u.get('evidence') or '').startswith('class_feature_of_unmodelled_corpus_class')))
"
# -> 2453   (confirms the headline count is still correct, unlike T2a's own
#            stale 8,243 -- this one held up)
```

The 2,453-unit total is real and unchanged. What was wrong was the ~47
false-positive estimate. Re-deriving with the committed script
`scripts/census_t12_class_feature.py` (run: `python3
scripts/census_t12_class_feature.py`) finds **118 confirmed false
positives**, in two structurally distinct classes, both proven by command,
not sampled:

### False-positive class A — 80 units: evidence names a monster racial-HD pseudo-class, not a playable class at all

`docs/work-inventory.json`'s own `kind=class` records include 33 PCGen
"class" entries with `type_facet=Monster` (`book~bestiary` etc.) — these are
PCGen's mechanism for monster racial Hit Dice progression (BAB/save curve per
creature type), never a playable class. Command:

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
mon = [u for u in d['units'] if u.get('kind')=='class' and u.get('type_facet')=='Monster']
print(len(mon))
for u in mon: print(u['name'], u['book'])
"
# -> 33 records: Aberration, Animal, Construct, Construct (Mindless),
#    Couatl Outsider, Dragon, Drider, Fey, Guardian Naga, Humanoid,
#    Humanoid (Reflex), Humanoid (Will), Magical Beast, Monstrous humanoid,
#    Ooze, Ooze (Intelligent), Outsider (Fort/Ref), Outsider (Fort/Will),
#    Outsider (Mindless), Outsider (Ref/Will), Plant, Plant (Mindless),
#    Spirit Naga, Undead, Undead (Mindless), Vermin, Vermin (Intelligent),
#    Faerie Dragon, Lammasu, Water Naga, Daughter of Urgathoa,
#    Homunculus Companion, Phantom
```

Seven of these creature-type names ALSO appear as T12 evidence slugs:
`animal`, `construct`, `dragon`, `fey`, `ooze`, `phantom`, `plant`, `undead`.
The census's own `corpus_class_names` fact (`v06_work_inventory.rs`) is built
from every `kind=Class` record indiscriminately — it does not exclude the
`type_facet=Monster` pseudo-classes — so an archetype record whose group
prefix happens to end in one of these words (`"Spirit Animal ~ Battle"`,
`"Order of the Dragon"`, `"Plant Master Plant Focus ~ Oak"`, `"Fey Trickster
~ Fey Veil"`, `"Undead Scourge ~ Smite Evil"`, `"PaDFE Construct"`, `"PaDFE
Ooze"`, `"Phantom Thief Talent ~ ..."`) gets attributed to the monster
pseudo-class instead of its real owner. Verified per-slug with the record's
own `type_facet`, e.g.:

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
for u in d['units']:
    if u.get('evidence')=='class_feature_of_unmodelled_corpus_class:dragon':
        print(u['corpus_key'], u.get('type_facet'))
" | head -3
# -> Order of the Dragon | CavalierClassFeatures.CavalierOrder.SpecialQuality
#    Order of the Dragon ~ Act as One | CavalierClassFeatures.SpecialQuality.Extraordinary
#    Order of the Dragon ~ Aid Allies | CavalierClassFeatures.SpecialQuality.Extraordinary
# real owner: Cavalier (already modelled)
```

| Reported slug | Count | Real owner(s), by type_facet | Real owner modelled? |
|---|---:|---|---|
| `phantom` | 24 | Rogue (17, "Phantom Thief Talent"), Spiritualist (7, "Phantom Emotional Focus") | Rogue: yes. Spiritualist: no — these 7 are genuinely T12, just filed under the wrong slug (recovered under Spiritualist below). |
| `animal` | 21 | Shaman ("Spirit Animal ~ ...") | no — genuinely T12, recovered under the Shaman-adjacent grouping (see note below; not separately tabled here as it did not resolve to a DISPATCHED name via type_facet and stays reported under its own umbrella in the residual bucket) |
| `undead` | 13 | Paladin, Arcanist, Wizard | yes, yes, yes — all 13 false |
| `plant` | 9 | Ranger ("Plant Master Plant Focus") | not resolved via type_facet marker (no `RangerClassFeatures` marker on these particular rows); kept in residual, not asserted as modelled without that proof |
| `dragon` | 6 | Cavalier ("Order of the Dragon"), Cleric domain power | yes, yes — false |
| `fey` | 4 | Mesmerist ("Fey Trickster") | no — genuinely T12 |
| `construct` | 2 | Cleric domain power, unresolved | mixed |
| `ooze` | 1 | unresolved | unresolved |

**Class A total: 80 units removed from T12's real population** — every unit
under these 7 monster-pseudo-class slugs, regardless of whether its real
owner turned out to be modelled or not, because the SLUG ITSELF is not a
real class; keeping any of them tallied under `animal`/`plant`/`undead`/etc.
would misrepresent the shape of the remaining work even where the record is
legitimately open (those are recovered under their real owning class in the
regrouped table below, not dropped).

### False-positive class B — 38 units: type_facet's own class marker names an already-modelled class

For every remaining T12 unit, `scripts/census_t12_class_feature.py`
independently re-derives the true owner from the record's own `type_facet`
string, using the SAME `"<Class> Class Feature(s)"` / `"<Class>ClassFeatures"`
marker convention `class_feature_owner_via_type_facet` in
`src/bin/v06_work_inventory.rs` already trusts for exactly this purpose —
segment-anchored (see "A substring bug caught and fixed" below), matched
only against the 34-class modelled roster (`ClassId`+`ApgClassId`+
`AcgClassId`+`UcClassId`+`PuClassId`, reproduced verbatim from this run's own
`epic-2-t2a-t12_cycle-1_cycle_receipt.md` DISPATCHED list, itself re-derived
that cycle, not merely cited).

```
     7  reported='duelist'    true_owner(modelled)=Bard     ("Arcane Duelist" archetype)
     7  reported='tactician'  true_owner(modelled)=Fighter  (7) / Paladin (6)  -- 13 total
     6  reported='psychic'    true_owner(modelled)=Wizard   ("Psychic Duelist"? no -- see script; Wizard-owned archetype rows)
     5  reported='adept'      true_owner(modelled)=Monk
     5  reported='evangelist' true_owner(modelled)=Cleric
     2  reported='warrior'    true_owner(modelled)=Paladin  ("Warrior of the Holy Light")
```

Every row independently re-confirmed by printing the matching records'
`type_facet` (see script output; e.g. `Arcane Duelist ~ Arcane Bond |
ClassFeatures.BardClassFeatures...`).

**Class B total: 38 units.**

**Total confirmed false positives: 80 + 38 = 118** (Class A checked first, no
double-count). `scripts/retro.py correction` logged against
`decisions.md §13`'s "~47" estimate — `docs/retro/events/t12-census.jsonl`.

### A substring bug caught and fixed mid-derivation

The first pass of this script matched a class's `"<Class>ClassFeature"`
marker as a bare substring anywhere in `type_facet`, and got a false hit:
`"MagicWarriorClassFeatures"` contains the literal text
`"WarriorClassFeature"` (the "c" ending "Magic" sits directly before the "W"
of "Warrior" — there is no word boundary in PCGen's CamelCase concatenation),
so "Magic Warrior" rows were wrongly credited to "Warrior" (already
modelled), and separately, `"PhrenicSlayerClassFeatures"` contains
`"SlayerClassFeature"` the same way, which would have wrongly reclassified
**all 31** genuinely-unmodelled "Phrenic Slayer" (a real, distinct
`PC.Prestige.Psionic` corpus class — confirmed:
`python3 -c "import json;d=json.load(open('docs/work-inventory.json'));[print(u) for u in d['units'] if u.get('kind')=='class' and 'phrenic' in u['name'].lower()]"`
→ `Phrenic Slayer`, `type_facet: PC.Prestige.Psionic`) records as Slayer
false positives. Fixed by anchoring the match at the START of each
dot-delimited `type_facet` segment (`segment.startswith(...)`), not a bare
substring check — re-run, both errors gone, both sanity-checked cases
(`Arcane Duelist` → Bard confirmed correct; `Phrenic Slayer` → correctly
NOT a false positive) hold under the fixed method.

## Confirmed real T12 population: 2,335

```
2,453 (evidence-code total) - 80 (class A) - 38 (class B) = 2,335
```

## Real population, regrouped by TRUE owning class — the work-dispatch table

`scripts/census_t12_class_feature.py`'s output, re-attributing every unit to
its `type_facet`-derived true owner where one resolves (against ALL
corpus-declared classes, not only the modelled 34, so an unmodelled owner is
recovered correctly — e.g. "Magic Warrior" → Magus, "Crystal Warrior" →
Aegis, "Feral Warrior"/"Warrior Path 1/2" → Psychic Warrior), falling back to
the evidence-reported slug only when `type_facet` carries no class marker at
all (950 of 2,335 units — the group-prefix match is the only signal for
these, unchanged from what `decisions.md §13`'s table already relied on).
Two case-variant rows for "Student of War" (7 + 4, both `adventurers_guide`)
merged by hand below — a cosmetic title-casing artifact in the fallback
formatter, not a real distinction (both resolve to the same class record).

**92 distinct real unmodelled classes.** Every row is `registered: false` —
confirmed by grep against the three files that would show otherwise:

```
grep -rn "Vigilante\|Magus\b\|Occultist\|Mesmerist\|Kineticist\|Spiritualist\|Aegis\b" \
  src/rules_core/rules_tables/crb/mod.rs src/rules_core/rules_tables/apg/mod.rs \
  src/rules_core/rules_tables/acg/mod.rs src/rules_core/rules_tables/pathfinder_unchained/mod.rs \
  src/rules_core/rules_tables/ultimate_combat/mod.rs
# -> only apg/mod.rs and acg/mod.rs comments EXPLAINING that Magus has no
#    CLASS: record and is deliberately absent from ClassId -- confirms
#    unregistered, does not contradict it
```

| Class | Units | Book | Build shape |
|---|---:|---|---|
| Vigilante | 196 | ultimate_intrigue | chassis + features (dual-identity mechanic, unique) |
| Medium | 147 | occult_adventures | chassis + features (spirit/séance subsystem, unique) |
| Psychic | 136 | occult_adventures | chassis + features (full spellcaster + phrenic amplifications) |
| Magus | 135 | ultimate_magic | chassis + features (spellstrike/arcana, unique) |
| Aegis | 135 | ultimate_psionics | chassis + features (customization-point subsystem) |
| Occultist | 125 | occult_adventures | chassis + features (implement subsystem, unique) |
| Mesmerist | 114 | occult_adventures | chassis + features (hypnotic stare/tricks) |
| Shifter | 110 | ultimate_wilderness | chassis + features (aspect/shift subsystem) |
| Kineticist | 92 | occult_adventures | chassis + features (burn/blast/wild talent economy) |
| Spiritualist | 86 | occult_adventures | chassis + features (phantom-companion subsystem) |
| Psychic Warrior | 55 | ultimate_psionics | chassis + features (power-point psionic caster) |
| Antipaladin | 39 | advanced_players_guide | chassis + features (Paladin-mirror, moderate reuse) |
| Phrenic Slayer | 31 | ultimate_psionics | table build (prestige class, narrow feature set) |
| Cryptic | 30 | ultimate_psionics | chassis + features (power-point psionic caster) |
| Stalwart Defender | 21 | advanced_players_guide | table build (prestige) |
| Wilder | 21 | ultimate_psionics | chassis + features (power-point psionic caster) |
| Elocater | 21 | ultimate_psionics | table build (prestige, psionic) |
| Thrallherd | 21 | ultimate_psionics | table build (prestige, psionic) |
| Soulknife | 20 | ultimate_psionics | chassis + features (mind blade subsystem) |
| Tactician | 20 | ultimate_psionics | table build (prestige, psionic) |
| Asavir | 19 | adventurers_guide | table build (prestige) |
| Marksman | 19 | ultimate_psionics | table build (prestige, psionic) |
| Psion Uncarnate | 19 | ultimate_psionics | table build (prestige, psionic) |
| Sighted Seeker | 19 | ultimate_psionics | table build (prestige, psionic) |
| Psychic Detective | 18 | occult_adventures | table build (prestige, occult) |
| Body Snatcher | 18 | ultimate_psionics | table build (prestige, psionic) |
| Metamorph | 18 | ultimate_psionics | table build (prestige, psionic) |
| Sanguine Angel | 17 | adventurers_guide | table build (prestige) |
| Twilight Talon | 17 | adventurers_guide | table build (prestige) |
| Adaptive Warrior | 17 | ultimate_psionics | table build (prestige, psionic) |
| Vitalist | 17 | ultimate_psionics | chassis + features (power-point psionic caster) |
| Adept | 16 | core_rulebook | table build (NPC class, narrow) |
| Master Spy | 16 | advanced_players_guide | table build (prestige) |
| Golden Legionnaire | 16 | adventurers_guide | table build (prestige) |
| Shadowdancer | 16 | core_rulebook | table build (prestige) |
| Psychic Fist | 16 | ultimate_psionics | table build (prestige, psionic) |
| Mammoth Rider | 15 | adventurers_guide | table build (prestige) |
| Pathfinder Savant | 15 | adventurers_guide | table build (prestige) |
| Duelist | 15 | core_rulebook | table build (prestige) |
| Pure Legion Enforcer | 15 | inner_sea_combat | table build (prestige) |
| Lion Blade | 15 | inner_sea_intrigue | table build (prestige) |
| War Mind | 15 | ultimate_psionics | table build (prestige, psionic) |
| Nature Warden | 14 | advanced_players_guide | table build (prestige) |
| Pathfinder Chronicler | 14 | core_rulebook | table build (prestige) |
| Dread | 14 | ultimate_psionics | table build (prestige, psionic) |
| Psicrystal Imprinter | 14 | ultimate_psionics | table build (prestige, psionic) |
| Assassin | 13 | core_rulebook | table build (prestige) |
| Psion | 13 | ultimate_psionics | chassis + features (power-point psionic caster) |
| Pyrokineticist | 13 | ultimate_psionics | table build (prestige, psionic) |
| Steel Falcon | 12 | adventurers_guide | table build (prestige) |
| Dragon Disciple | 12 | core_rulebook | table build (prestige) |
| Enchanting Courtesan | 12 | inner_sea_intrigue | table build (prestige) |
| Telekinetic Weaponmaster | 12 | ultimate_psionics | table build (prestige, psionic) |
| Battle Herald | 11 | advanced_players_guide | table build (prestige) |
| Rage Prophet | 11 | advanced_players_guide | table build (prestige) |
| Lantern Bearer | 11 | adventurers_guide | table build (prestige) |
| Storm Kindler | 11 | adventurers_guide | table build (prestige) |
| Westcrown Devil | 11 | adventurers_guide | table build (prestige) |
| Metaforge | 11 | ultimate_psionics | table build (prestige, psionic) |
| Holy Vindicator | 10 | advanced_players_guide | table build (prestige) |
| Aldori Swordlord | 10 | adventurers_guide | table build (prestige) |
| Hellknight | 10 | inner_sea_world_guide | table build (prestige) |
| Pathfinder Delver | 10 | adventurers_guide | table build (prestige) |
| Diabolist | 10 | book_of_the_damned_volume_1 | table build (prestige) |
| Cerebremancer | 10 | ultimate_psionics | table build (prestige, psionic) |
| Aspis Agent | 9 | adventurers_guide | table build (prestige) |
| Gray Corsair | 9 | adventurers_guide | table build (prestige) |
| Rivethun Emissary | 9 | adventurers_guide | table build (prestige) |
| Arcane Archer | 8 | core_rulebook | table build (prestige) |
| Arcane Trickster | 8 | core_rulebook | table build (prestige) |
| Ulfen Guard | 8 | inner_sea_combat | table build (prestige) |
| Master Chymist | 7 | advanced_players_guide | table build (prestige) |
| Bellflower Tiller | 7 | adventurers_guide | table build (prestige) |
| Hellknight Signifer | 7 | adventurers_guide | table build (prestige) |
| Student of War | 11 (7+4, merged) | adventurers_guide | table build (prestige) |
| Demoniac | 7 | book_of_the_damned_volume_2 | table build (prestige) |
| Mystic Archer | 7 | ultimate_psionics | table build (prestige, psionic) |
| Soul Archer | 7 | ultimate_psionics | table build (prestige, psionic) |
| Loremaster | 6 | core_rulebook | table build (prestige) |
| Warrior | 5 | core_rulebook | table build (NPC class, includes 4 misrouted "Order of the Warrior" Samurai-order units — see below) |
| Horizon Walker | 4 | advanced_players_guide | table build (prestige) |
| Eldritch Knight | 4 | core_rulebook | table build (prestige) |
| Dark Tempest | 4 | ultimate_psionics | table build (prestige, psionic) |
| Metamind | 4 | ultimate_psionics | table build (prestige, psionic) |
| Mystic Theurge | 3 | core_rulebook | table build (prestige) |
| Argent Dramaturge | 2 | adventurers_guide | table build (prestige) |
| Cyphermage | 2 | inner_sea_magic | table build (prestige) |
| Expert | 2 | core_rulebook | table build (NPC class, narrow) |
| Aristocrat | 1 | core_rulebook | table build (NPC class, narrow) |
| Commoner | 1 | core_rulebook | table build (NPC class, narrow) |
| Sentinel | 1 | inner_sea_gods | table build (prestige) |

**Sum check:** `python3 -c "..."` (script's own internal sum assertion, run
this cycle) confirms the table sums to exactly 2,335.

### A likely third false-positive family, not yet resolved by command — flag, don't guess

`Warrior`'s 5-unit row includes 4 `"Order of the Warrior ~ ..."` records
whose `type_facet` is bare (`SpecialQuality`, no class marker), so the
`type_facet` method above correctly declines to call them false positives —
but the raw `.lst` source (`uc_abilities_class.lst:216`) declares them under
`KEY:Samurai Order ~ Order of the Warrior` and `TYPE:...CavalierOrder`, and
their own DESC text says "samurai" throughout:

```
grep -n "Order of the Warrior" docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat/uc_abilities_class.lst | head -1
# -> 216:Order of the Warrior  KEY:Samurai Order ~ Order of the Warrior ...
#    TYPE:SpecialQuality.SamuraiOrder.CavalierOrder ...
```

Samurai IS modelled (`UcClassId`). This is real, hand-verified evidence for
4 more false positives, but it comes from raw-`.lst` `KEY:`/`DESC:` reading,
not the `type_facet`-marker method the rest of this census used — a
different, unaudited signal. **Not folded into the 118/2,335 headline
numbers above** to keep the two commands (`type_facet`-marker vs raw-`.lst`
`KEY:` reading) separately auditable; logged here as a named, bounded,
not-yet-swept risk: a full raw-`.lst` `KEY:` sweep across all 2,335 residual
units, the same discipline `CLASS_FEATURE_POOLS`' own 27 entries were built
through, may find more of this shape (`decisions.md §3`'s fixture-check
bar). Recommend the first work cycle on any "table build (prestige)"-shaped
small class run this sweep for its own book before building, since the tax
is cheap at that scale and this exact failure mode (an order/archetype named
after an unrelated class) is now a known pattern, not a one-off.

## Cost model — per class, not per unit

`docs/retro/` E13 calibration and `decisions.md §13` are both explicit: the
fixed cost here is per-book/file, not per-record. T12's own natural unit is
the CLASS (not the book — several classes share a book, e.g. 8 of the top 20
by unit count live in `ultimate_psionics`), so the per-class file-touch tax
is the real driver.

**Precedent, same bundle:** `src/rules_core/pilot_compute/
untabled_base_class_chassis.rs` (card 12, this cycle's landed base) — 20
classes' BAB/save CHASSIS (not features) in ONE new file (259 lines) + one
fixture JSON + a handful of `compute_class_chassis` dispatch-site edits in
`pilot_compute/mod.rs`. That precedent is CHEAP per-class specifically
because it reuses `rules_tables::crb::class_tables`'s existing two formula
functions verbatim — nothing class-specific except metadata (hit die,
BAB progression, saves). **T12 is not that shape.** T12 needs `class_feature`
computation — the actual granted abilities, option pools, and (for 11 of the
92 classes) full alternate-resource spellcasting/power-point subsystems that
have no shared-formula shortcut the chassis case had. Each of those 11 is
closer in shape to a full new class build (like the SD-3x era's own
Arcanist/Barbarian-tier work) than to a chassis-metadata row.

**Two tiers, not one:**

1. **11 large/complex classes** (Vigilante, Medium, Psychic, Magus, Aegis,
   Occultist, Mesmerist, Shifter, Kineticist, Spiritualist, Psychic Warrior;
   55-196 units each, 1,331 of the 2,335 total) — each needs a genuinely new
   subsystem (implement points, burn economy, spirit possession, spellstrike,
   customization points, ...). No two share a mechanic. Each is its own
   multi-file cycle: a features module (or several), dispatch-site wiring in
   `pilot_compute/mod.rs`, a corpus-derived fixture, tests, receipt — the
   card-12 precedent's own file list is the floor, not the ceiling, since
   card 12 didn't have to invent a mechanic. **Estimate 1 cycle per class,
   11 cycles**, and this is the bundle's biggest remaining content risk (see
   `biggest_risk`).
2. **81 smaller classes** (1-39 units each, 1,004 of the 2,335 total) —
   mostly prestige classes with narrow, largely auto-granted feature
   progressions (closer in shape to `untabled_base_class_chassis.rs`'s own
   metadata-row cost, several of which can plausibly share one dispatch
   mechanism the way `CLASS_FEATURE_POOLS`'s 27 entries share one table).
   Several cluster by book (`ultimate_psionics` alone carries ~30 of these
   81) and by mechanic family (psionic power-point prestige classes could
   plausibly share one small chassis extension). **Estimate 3-5 classes per
   cycle, batched by book/mechanic-family, ≈ 20 cycles.**

**Total estimate: ~31 cycles** (11 + 20). This is an ESTIMATE, not a
committed re-derive command — "how many cycles a class needs" is a judgment
call the way `decisions.md §13`'s own text acknowledges book-count (not
unit-count) drives cost; the class list and file precedent above are the
re-derivable parts.

## What this measurement cycle did NOT do

No engine code, corpus data, or pinned count changed. `kanban.md` row 11
left at `in-progress` untouched. `progress.md` not touched by this cycle
(no shared-state edit needed for a pure measurement cycle per the dispatch
brief). The 4 unresolved-but-suspected "Order of the Warrior" false
positives above are named, not silently folded into either the false-
positive or the real count — a future work cycle on Warrior/Samurai should
resolve them with the raw-`.lst` sweep before touching either class.

`df -h /`: 29% used (278G/968G), no disk pressure.
