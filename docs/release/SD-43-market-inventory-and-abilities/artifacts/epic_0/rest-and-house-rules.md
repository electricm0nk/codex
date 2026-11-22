# E0.5 — Rest rules and House Rules per system

Census of what the corpus states about rest and recovery, per class (168 records) and per race (39 records), and the House Rules per game system. Inputs are corpus records only (read-only); nothing is written to live code. Text for the sheet is E2.4.

## 1. Method and denominators

- Denominator, classes: `ls data/corpus/*/class/*.json | awk 'END{print NR}'` -> **168** (one row each in section 3).
- Denominator, races: `ls data/corpus/*/race/*.json | awk 'END{print NR}'` -> **39** (one row each in section 4).
- A class is joined to its feature records by name: `data/corpus/*/class_feature/<dir>/*.json`, field `data.class` equal (case-insensitive) to the class record's `class_id`, else `name`, else `key`. A race is joined to `data/corpus/*/race_trait/<race>/*.json` by `data.race_key`.
- A feature is a **rest-rule hit** when its description matches one of four patterns (spell-preparation, rest-or-sleep, daily-refresh, natural-healing; the regexes are in the appendix script, which re-derives every figure here). One hit per feature (first category in that order).
- Class and race chassis records hold no rest text (`raw_tokens` of `class/wizard.json` are HD, BONUS, DEFINE, FACT; no description). All rest rules are in feature/trait descriptions.

| Class outcome | Count | Predicate |
|---|---|---|
| rows with at least one rest-rule hit | 32 | >=1 feature record whose description matches a pattern |
| feature records exist, no rest rule matched: "no rest rule stated" | 88 | >=1 feature record scanned, 0 hits |
| no feature records in the corpus: "no rest rule stated (no feature records)" | 48 | 0 class_feature records with that class name |
| total | 168 | equals the 168 denominator |

Races: 1 of 39 rows have a hit; 38 have trait records scanned with no hit (every race has >=1 `race_trait` record).

## 2. Reading the table

Source column = book directory + the record's `SOURCEPAGE` token (blank when the record carries none; `p.xx` is the corpus's own placeholder page, kept as stated). The features listed are the first three distinct feature names by pattern priority; the count is all distinct hits. Feature records include **archetype and alternate-class variants** (the `class` field names the base class), so a hit is evidence of a rest rule in the book for that class family, not proof that the base class itself has it.

## 3. Classes (168)

| # | Class record | Type token | Class page | Rest-rule rows (feature, book, page: category) | Hits |
|---|---|---|---|---|---|
| 1 | arcanist (advanced_class_guide) | Base.PC | p.8 | Arcane Reservoir (advanced_class_guide p.9: spell-preparation); Cantrips (advanced_class_guide p.12: spell-preparation); School Savant (advanced_class_guide p.78: spell-preparation) | 7 |
| 2 | bloodrager (advanced_class_guide) | Base.PC | p.15 | Spells (advanced_class_guide p.16: spell-preparation); Bloodrage (advanced_class_guide p.15: rest-or-sleep) | 2 |
| 3 | brawler (advanced_class_guide) | Base.PC | p.23 | no rest rule stated (208 feature records scanned) | 0 |
| 4 | Ex-Warpriest (advanced_class_guide) | Base.PC | p.63 | no rest rule stated (no feature records) | 0 |
| 5 | hunter (advanced_class_guide) | Base.PC | p.26 | Orisons (advanced_class_guide p.26: spell-preparation); Master Hunter (ultimate_wilderness p.55: spell-preparation); Master Verminous Hunter (advanced_class_guide p.98: daily-refresh) | 3 |
| 6 | investigator (advanced_class_guide) | Base.PC | p.30 | Alchemy (advanced_class_guide p.31: spell-preparation); A Quiet Word (advanced_class_guide p.101: spell-preparation); Limited Alchemy (horror_adventures p.54: spell-preparation) | 5 |
| 7 | shaman (advanced_class_guide) | Base.PC | p.35 | Orisons (advanced_class_guide p.35: spell-preparation); Spirit Animal (advanced_class_guide p.35: spell-preparation); Wandering Hex (advanced_class_guide p.37: spell-preparation) | 8 |
| 8 | skald (advanced_class_guide) | Base.PC | p.49 | no rest rule stated (94 feature records scanned) | 0 |
| 9 | slayer (advanced_class_guide) | Base.PC | p.53 | no rest rule stated (163 feature records scanned) | 0 |
| 10 | swashbuckler (advanced_class_guide) | Base.PC | p.56 | Thief's Confidence (adventurers_guide p.50: daily-refresh) | 1 |
| 11 | warpriest (advanced_class_guide) | Base.PC | p.60 | Orisons (advanced_class_guide p.60: spell-preparation); Raging Absorption (advanced_class_guide p.154: daily-refresh); Recovered Rage (advanced_class_guide p.155: daily-refresh) | 3 |
| 12 | alchemist (advanced_players_guide) | Base.PC | p.26 | Mixologist Master (inner_sea_taverns p.58: spell-preparation); Diminished Alchemy (ultimate_intrigue p.18: spell-preparation); Infused Curative (ultimate_magic p.18: spell-preparation) | 4 |
| 13 | Antipaladin (advanced_players_guide) | Base.PC | p.118 | no rest rule stated (44 feature records scanned) | 0 |
| 14 | Battle Herald (advanced_players_guide) | PC.Prestige | p.260 | no rest rule stated (11 feature records scanned) | 0 |
| 15 | cavalier (advanced_players_guide) | Base.PC.Cavalier | p.32 | no rest rule stated (188 feature records scanned) | 0 |
| 16 | Eidolon (advanced_players_guide) | Monster.Companion | p.309 | no rest rule stated (16 feature records scanned) | 0 |
| 17 | Ex-Antipaladin (advanced_players_guide) | Base.PC | p.118 | no rest rule stated (no feature records) | 0 |
| 18 | Ex-Inquisitor (advanced_players_guide) | Base.PC | - | no rest rule stated (no feature records) | 0 |
| 19 | Holy Vindicator (advanced_players_guide) | PC.Prestige | p.263 | no rest rule stated (10 feature records scanned) | 0 |
| 20 | Horizon Walker (advanced_players_guide) | PC.Prestige | p.265 | no rest rule stated (4 feature records scanned) | 0 |
| 21 | inquisitor (advanced_players_guide) | Base.PC.SpontaneousDivine.Spontaneous | p.38 | Orisons (advanced_players_guide p.40: spell-preparation); Spells (horror_adventures p.53: spell-preparation); Misdirection (ultimate_magic p.45: spell-preparation) | 4 |
| 22 | Master Chymist (advanced_players_guide) | PC.Prestige | p.267 | no rest rule stated (7 feature records scanned) | 0 |
| 23 | Master Spy (advanced_players_guide) | PC.Prestige | p.270 | no rest rule stated (16 feature records scanned) | 0 |
| 24 | Nature Warden (advanced_players_guide) | PC.Prestige | p.272 | no rest rule stated (14 feature records scanned) | 0 |
| 25 | oracle (advanced_players_guide) | Base.PC.SpontaneousDivine.Spontaneous | p.42 | Orisons (advanced_players_guide p.45: spell-preparation); Ki Pool (advanced_race_guide p.164: rest-or-sleep); Inspiration (advanced_class_guide p.106: daily-refresh) | 4 |
| 26 | Rage Prophet (advanced_players_guide) | PC.Prestige | p.275 | no rest rule stated (11 feature records scanned) | 0 |
| 27 | Stalwart Defender (advanced_players_guide) | PC.Prestige | p.277 | Defensive Stance (advanced_players_guide no page: rest-or-sleep) | 1 |
| 28 | summoner (advanced_players_guide) | Base.PC.SpontaneousArcane.Spontaneous | p.54 | no rest rule stated (99 feature records scanned) | 0 |
| 29 | witch (advanced_players_guide) | Base.PC | p.65 | Cantrips (advanced_players_guide p.65: spell-preparation); Bonded Item (advanced_race_guide p.43: spell-preparation); Fetish Mask (advanced_race_guide p.140: spell-preparation) | 6 |
| 30 | Argent Dramaturge (adventurers_guide) | PC.Prestige | p.172 | no rest rule stated (2 feature records scanned) | 0 |
| 31 | Asavir (adventurers_guide) | PC.Prestige | p.10 | no rest rule stated (20 feature records scanned) | 0 |
| 32 | Cyphermage (adventurers_guide) | PC.Prestige | p.56 | no rest rule stated (17 feature records scanned) | 0 |
| 33 | Death Slayer (adventurers_guide) | PC.Prestige | p.152 | no rest rule stated (no feature records) | 0 |
| 34 | Gray Corsair (adventurers_guide) | PC.Prestige | p.68 | no rest rule stated (9 feature records scanned) | 0 |
| 35 | Mammoth Rider (adventurers_guide) | PC.Prestige | p.128 | no rest rule stated (15 feature records scanned) | 0 |
| 36 | Sanguine Angel (adventurers_guide) | PC.Prestige | p.80 | no rest rule stated (17 feature records scanned) | 0 |
| 37 | Student of Perfection (adventurers_guide) | PC.Prestige | p.102 | no rest rule stated (no feature records) | 0 |
| 38 | Student of War (adventurers_guide) | PC.Prestige | p.142 | no rest rule stated (11 feature records scanned) | 0 |
| 39 | Aberration (beastiary) | Monster | p.306 | no rest rule stated (no feature records) | 0 |
| 40 | Animal (beastiary) | Monster | p.306 | no rest rule stated (18 feature records scanned) | 0 |
| 41 | Construct (beastiary) | Monster | p.307 | no rest rule stated (2 feature records scanned) | 0 |
| 42 | Construct (Mindless) (beastiary) | Monster | p.307 | no rest rule stated (no feature records) | 0 |
| 43 | Couatl Outsider (beastiary) | Monster | p.309 | no rest rule stated (no feature records) | 0 |
| 44 | Dragon (beastiary) | Monster | p.307 | no rest rule stated (6 feature records scanned) | 0 |
| 45 | Drider (beastiary) | Monster | p.306 | no rest rule stated (no feature records) | 0 |
| 46 | Fey (beastiary) | Monster | p.307 | no rest rule stated (5 feature records scanned) | 0 |
| 47 | Guardian Naga (beastiary) | Monster | p.213 | no rest rule stated (no feature records) | 0 |
| 48 | Humanoid (beastiary) | Monster | p.308 | no rest rule stated (no feature records) | 0 |
| 49 | Humanoid (Reflex) (beastiary) | Monster | p.308 | no rest rule stated (no feature records) | 0 |
| 50 | Humanoid (Will) (beastiary) | Monster | p.308 | no rest rule stated (no feature records) | 0 |
| 51 | Magical Beast (beastiary) | Monster | p.308 | no rest rule stated (no feature records) | 0 |
| 52 | Monstrous humanoid (beastiary) | Monster | p.308 | no rest rule stated (no feature records) | 0 |
| 53 | Ooze (beastiary) | Monster | p.308 | no rest rule stated (1 feature records scanned) | 0 |
| 54 | Ooze (Intelligent) (beastiary) | Monster | p.308 | no rest rule stated (no feature records) | 0 |
| 55 | Outsider (Fort/Ref) (beastiary) | Monster | p.309 | no rest rule stated (no feature records) | 0 |
| 56 | Outsider (Fort/Will) (beastiary) | Monster | p.309 | no rest rule stated (no feature records) | 0 |
| 57 | Outsider (Mindless) (beastiary) | Monster | p.309 | no rest rule stated (no feature records) | 0 |
| 58 | Outsider (Ref/Will) (beastiary) | Monster | p.309 | no rest rule stated (no feature records) | 0 |
| 59 | Plant (beastiary) | Monster | p.309 | no rest rule stated (9 feature records scanned) | 0 |
| 60 | Plant (Mindless) (beastiary) | Monster | p.309 | no rest rule stated (no feature records) | 0 |
| 61 | Sorcerer/Cleric (Arcane) (beastiary) | - | - | no rest rule stated (no feature records) | 0 |
| 62 | Spirit Naga (beastiary) | Monster | p.215 | no rest rule stated (no feature records) | 0 |
| 63 | Undead (beastiary) | Monster | p.309 | no rest rule stated (8 feature records scanned) | 0 |
| 64 | Undead (Mindless) (beastiary) | Monster | p.309 | no rest rule stated (no feature records) | 0 |
| 65 | Vermin (beastiary) | Monster | p.310 | no rest rule stated (no feature records) | 0 |
| 66 | Vermin (Intelligent) (beastiary) | Monster | p.310 | no rest rule stated (no feature records) | 0 |
| 67 | Faerie Dragon (bonus_bestiary) | Monster | - | no rest rule stated (no feature records) | 0 |
| 68 | Lammasu (bonus_bestiary) | Monster | - | no rest rule stated (no feature records) | 0 |
| 69 | Water Naga (bonus_bestiary) | Monster | - | no rest rule stated (no feature records) | 0 |
| 70 | Diabolist (book_of_the_damned_volume_1) | PC.Prestige | p.44 | no rest rule stated (10 feature records scanned) | 0 |
| 71 | Demoniac (book_of_the_damned_volume_2) | PC.Prestige | p.46 | no rest rule stated (7 feature records scanned) | 0 |
| 72 | Adept (core_rulebook) | Base.NPC | p.448 | no rest rule stated (10 feature records scanned) | 0 |
| 73 | Arcane Archer (core_rulebook) | PC.Prestige | p.374 | Enhance Arrow (core_rulebook p.375: spell-preparation) | 1 |
| 74 | Arcane Trickster (core_rulebook) | PC.Prestige | p.376 | no rest rule stated (8 feature records scanned) | 0 |
| 75 | Aristocrat (core_rulebook) | Base.NPC | p.449 | no rest rule stated (2 feature records scanned) | 0 |
| 76 | Assassin (core_rulebook) | PC.Prestige | p.378 | no rest rule stated (17 feature records scanned) | 0 |
| 77 | Barbarian (core_rulebook) | Base.PC | p.31 | Rage (core_rulebook p.32: rest-or-sleep) | 1 |
| 78 | Bard (core_rulebook) | Base.PC.SpontaneousArcane.Spontaneous | p.34 | no rest rule stated (299 feature records scanned) | 0 |
| 79 | Cleric (core_rulebook) | Base.PC | p.38 | Orisons (core_rulebook p.41: spell-preparation); Focused Domain (ultimate_magic p.32: spell-preparation) | 2 |
| 80 | Commoner (core_rulebook) | Base.NPC | p.449 | no rest rule stated (2 feature records scanned) | 0 |
| 81 | Dragon Disciple (core_rulebook) | PC.Prestige | p.380 | no rest rule stated (16 feature records scanned) | 0 |
| 82 | Druid (core_rulebook) | Base.PC | p.48 | Inspiration (advanced_class_guide p.92: spell-preparation); Orisons (core_rulebook p.49: spell-preparation); Pack Bond (ultimate_magic p.38: spell-preparation) | 3 |
| 83 | Duelist (core_rulebook) | PC.Prestige | p.382 | no rest rule stated (15 feature records scanned) | 0 |
| 84 | Eldritch Knight (core_rulebook) | PC.Prestige | p.384 | no rest rule stated (4 feature records scanned) | 0 |
| 85 | Ex-Barbarian (core_rulebook) | Base.PC | - | no rest rule stated (no feature records) | 0 |
| 86 | Ex-Paladin (core_rulebook) | Base.PC | - | no rest rule stated (no feature records) | 0 |
| 87 | Expert (core_rulebook) | Base.NPC | p.450 | no rest rule stated (3 feature records scanned) | 0 |
| 88 | Fighter (core_rulebook) | Base.PC | p.55 | no rest rule stated (304 feature records scanned) | 0 |
| 89 | Loremaster (core_rulebook) | PC.Prestige | p.385 | no rest rule stated (6 feature records scanned) | 0 |
| 90 | Monk (core_rulebook) | Base.PC | p.56 | Mystic Visions (advanced_players_guide p.111: rest-or-sleep); Burn (ultimate_wilderness p.65: rest-or-sleep) | 2 |
| 91 | Mystic Theurge (core_rulebook) | PC.Prestige | p.387 | Combined Spells (core_rulebook p.388: spell-preparation) | 1 |
| 92 | Paladin (core_rulebook) | Base.PC | p.60 | Smite Evil (advanced_players_guide p.60: rest-or-sleep) | 1 |
| 93 | Pathfinder Chronicler (core_rulebook) | PC.Prestige | p.388 | no rest rule stated (14 feature records scanned) | 0 |
| 94 | Ranger (core_rulebook) | Base.PC | p.64 | Spells (ultimate_wilderness p.72: spell-preparation); Nature's Mercy (ultimate_wilderness p.74: spell-preparation) | 2 |
| 95 | Rogue (core_rulebook) | Base.PC.Rogue | p.67 | Misdirection (ultimate_combat p.71: daily-refresh); Ki Pool (ultimate_combat p.69: daily-refresh); Inspiration (ultimate_intrigue p.47: daily-refresh) | 3 |
| 96 | Shadowdancer (core_rulebook) | PC.Prestige | p.391 | no rest rule stated (16 feature records scanned) | 0 |
| 97 | Sorcerer (core_rulebook) | Base.PC.SpontaneousArcane.Spontaneous | p.70 | Mongrel Reservoir (advanced_class_guide p.122: daily-refresh); Bloodline Arcana (ultimate_magic p.73: daily-refresh); Fast Healer (advanced_players_guide p.160: natural-healing) | 3 |
| 98 | Warrior (core_rulebook) | Base.NPC | p.450 | no rest rule stated (49 feature records scanned) | 0 |
| 99 | Wizard (core_rulebook) | Base.PC | p.77 | Arcane Reservoir (advanced_class_guide p.133: spell-preparation); Bonded Object (core_rulebook p.78: spell-preparation); Arcane School (core_rulebook p.78: spell-preparation) | 7 |
| 100 | Undead Phantom (horror_adventures) | - | - | no rest rule stated (no feature records) | 0 |
| 101 | Pure Legion Enforcer (inner_sea_combat) | PC.Prestige | p.32 | Improved Godless Healing (inner_sea_combat p.31: natural-healing) | 1 |
| 102 | Ulfen Guard (inner_sea_combat) | PC.Prestige | p.34 | no rest rule stated (8 feature records scanned) | 0 |
| 103 | Evangelist (inner_sea_gods) | PC.Prestige | p.198 | no rest rule stated (1 feature records scanned) | 0 |
| 104 | Exalted (inner_sea_gods) | PC.Prestige | p.200 | no rest rule stated (no feature records) | 0 |
| 105 | Sentinel (inner_sea_gods) | PC.Prestige | p.202 | no rest rule stated (1 feature records scanned) | 0 |
| 106 | Enchanting Courtesan (inner_sea_intrigue) | PC.Prestige | p.34 | no rest rule stated (12 feature records scanned) | 0 |
| 107 | Lion Blade (inner_sea_intrigue) | PC.Prestige | p.36 | no rest rule stated (15 feature records scanned) | 0 |
| 108 | Divine Scion (inner_sea_magic) | PC.Prestige | p.46 | no rest rule stated (46 feature records scanned) | 0 |
| 109 | Eidolon (Fey) (inner_sea_magic) | Monster.Companion | p.35 | no rest rule stated (no feature records) | 0 |
| 110 | Low Templar (inner_sea_world_guide) | PC.Prestige | p.280 | no rest rule stated (no feature records) | 0 |
| 111 | Homunculus Companion (occult_adventures) | Monster | p.113 | no rest rule stated (no feature records) | 0 |
| 112 | Kineticist (occult_adventures) | Base.PC.Psychic | p.10 | Burn (occult_adventures p.11: rest-or-sleep); Kinetic Healer (occult_adventures p.26: rest-or-sleep); Internal Buffer (occult_adventures p.13: daily-refresh) | 3 |
| 113 | Medium (occult_adventures) | Base.PC.Spontaneous.Psychic | p.30 | Hierophant Seance Boon (occult_adventures p.35: natural-healing) | 1 |
| 114 | Mesmerist (occult_adventures) | Base.PC.Spontaneous.Psychic | p.38 | Injection Improvement (occult_adventures p.98: spell-preparation) | 1 |
| 115 | Occultist (occult_adventures) | Base.PC.Spontaneous.Psychic | p.46 | Mental Focus (occult_adventures p.48: rest-or-sleep) | 1 |
| 116 | Phantom (occult_adventures) | Monster | p.78 | no rest rule stated (24 feature records scanned) | 0 |
| 117 | Psychic (occult_adventures) | Base.PC.Spontaneous.Psychic | p.60 | Spells (occult_adventures p.60: spell-preparation); Mnemonic Cache (occult_adventures p.: spell-preparation); Formulaic Powers (ultimate_psionics p.295: spell-preparation) | 7 |
| 118 | Psychic Detective (occult_adventures) | - | - | no rest rule stated (18 feature records scanned) | 0 |
| 119 | Spiritualist (occult_adventures) | Base.PC.Spontaneous.Psychic | p.72 | no rest rule stated (88 feature records scanned) | 0 |
| 120 | Unchained Barbarian (pathfinder_unchained) | Barbarian Class Selection.AltBarbarianChoice | - | no rest rule stated (54 feature records scanned) | 0 |
| 121 | Unchained Monk (pathfinder_unchained) | Monk Class Selection.AltMonkChoice | - | no rest rule stated (no feature records) | 0 |
| 122 | Unchained Rogue (pathfinder_unchained) | Rogue Class Selection.AltRogueChoice | - | no rest rule stated (no feature records) | 0 |
| 123 | Unchained Summoner (pathfinder_unchained) | Summoner Class Selection.SummonerAlternative | - | no rest rule stated (no feature records) | 0 |
| 124 | Gunslinger (ultimate_combat) | Base.PC | p.9 | no rest rule stated (125 feature records scanned) | 0 |
| 125 | Ninja (ultimate_combat) | Base.PC.Rogue | p.26 | no rest rule stated (60 feature records scanned) | 0 |
| 126 | Samurai (ultimate_combat) | Base.PC.Cavalier | p.9 | no rest rule stated (25 feature records scanned) | 0 |
| 127 | VCabalist (ultimate_intrigue) | Support | - | no rest rule stated (no feature records) | 0 |
| 128 | Vigilante (ultimate_intrigue) | Base.PC | p.9 | Spellcasting (ultimate_intrigue p.56: spell-preparation) | 1 |
| 129 | VWarlock (ultimate_intrigue) | Support | - | no rest rule stated (no feature records) | 0 |
| 130 | Magus (ultimate_magic) | Base.PC | p.9 | Infernal Mortification (advanced_race_guide p.170: spell-preparation); Spells (occult_adventures p.121: spell-preparation); Arcane Pool (ultimate_magic p.9: spell-preparation) | 5 |
| 131 | Adaptive Warrior (ultimate_psionics) | PC.Prestige.Psionic | p.349 | no rest rule stated (18 feature records scanned) | 0 |
| 132 | Aegis (ultimate_psionics) | Base.Psionic.PC.Intuitive | p.28 | no rest rule stated (134 feature records scanned) | 0 |
| 133 | Astral Warrior (ultimate_psionics) | Monster.Companion | p.263 | no rest rule stated (no feature records) | 0 |
| 134 | Body Snatcher (ultimate_psionics) | PC.Prestige.Psionic | p.351 | no rest rule stated (19 feature records scanned) | 0 |
| 135 | Cerebremancer (ultimate_psionics) | PC.Prestige.Psionic | p.353 | no rest rule stated (10 feature records scanned) | 0 |
| 136 | Cryptic (ultimate_psionics) | Base.Psionic.PC.Intuitive | p.34 | no rest rule stated (36 feature records scanned) | 0 |
| 137 | Dark Tempest (ultimate_psionics) | PC.Prestige.Psionic | p.355 | no rest rule stated (6 feature records scanned) | 0 |
| 138 | Dread (ultimate_psionics) | Base.Psionic.PC.Self-Taught | p.43 | no rest rule stated (18 feature records scanned) | 0 |
| 139 | Elocater (ultimate_psionics) | PC.Prestige.Psionic | p.357 | no rest rule stated (21 feature records scanned) | 0 |
| 140 | Gifted Blade (ultimate_psionics) | Psionic | - | no rest rule stated (3 feature records scanned) | 0 |
| 141 | Gifted Blade Marksman Power List (ultimate_psionics) | Psionic | - | no rest rule stated (no feature records) | 0 |
| 142 | Horror (ultimate_psionics) | Monster.Companion | p.70 | no rest rule stated (no feature records) | 0 |
| 143 | Horror (Fort/Ref) (ultimate_psionics) | Monster.Companion | p.70 | no rest rule stated (no feature records) | 0 |
| 144 | Horror (Fort/Will) (ultimate_psionics) | Monster.Companion | p.70 | no rest rule stated (no feature records) | 0 |
| 145 | Horror (Ref/Will) (ultimate_psionics) | Monster.Companion | p.70 | no rest rule stated (no feature records) | 0 |
| 146 | Marksman (ultimate_psionics) | Base.Psionic.PC.Self-Taught | p.44 | no rest rule stated (23 feature records scanned) | 0 |
| 147 | Metaforge (ultimate_psionics) | PC.Prestige.Psionic | p.359 | no rest rule stated (6 feature records scanned) | 0 |
| 148 | Metamind (ultimate_psionics) | PC.Prestige.Psionic | p.360 | no rest rule stated (11 feature records scanned) | 0 |
| 149 | Metamorph (ultimate_psionics) | PC.Prestige.Psionic | p.362 | no rest rule stated (19 feature records scanned) | 0 |
| 150 | Mystic Archer (ultimate_psionics) | PC.Prestige.Psionic | p.364 | no rest rule stated (7 feature records scanned) | 0 |
| 151 | Phrenic Slayer (ultimate_psionics) | PC.Prestige.Psionic | p.365 | no rest rule stated (47 feature records scanned) | 0 |
| 152 | Psicrystal Imprinter (ultimate_psionics) | PC.Prestige.Psionic | p.367 | no rest rule stated (14 feature records scanned) | 0 |
| 153 | Psion (ultimate_psionics) | Base.Psionic.PC.Trained | p.49 | no rest rule stated (17 feature records scanned) | 0 |
| 154 | Psion Uncarnate (ultimate_psionics) | PC.Prestige.Psionic | p.370 | no rest rule stated (19 feature records scanned) | 0 |
| 155 | Psychic Fist (ultimate_psionics) | PC.Prestige.Psionic | p.166 | no rest rule stated (18 feature records scanned) | 0 |
| 156 | Psychic Warrior (ultimate_psionics) | Base.Psionic.PC.Self-Taught | p.53 | no rest rule stated (18 feature records scanned) | 0 |
| 157 | Pyrokineticist (ultimate_psionics) | PC.Prestige.Psionic | p.374 | no rest rule stated (13 feature records scanned) | 0 |
| 158 | Sighted Seeker (ultimate_psionics) | PC.Prestige.Psionic | p.378 | no rest rule stated (25 feature records scanned) | 0 |
| 159 | Soul Archer (ultimate_psionics) | PC.Prestige.Psionic | p.379 | no rest rule stated (7 feature records scanned) | 0 |
| 160 | Soulknife (ultimate_psionics) | Base.Psionic.PC.Intuitive | p.57 | no rest rule stated (19 feature records scanned) | 0 |
| 161 | Tactician (ultimate_psionics) | Base.Psionic.PC.Trained | p.64 | no rest rule stated (24 feature records scanned) | 0 |
| 162 | Telekinetic Weaponmaster (ultimate_psionics) | PC.Prestige.Psionic | p.381 | no rest rule stated (12 feature records scanned) | 0 |
| 163 | Thrallherd (ultimate_psionics) | PC.Prestige.Psionic | p.382 | no rest rule stated (21 feature records scanned) | 0 |
| 164 | Unlocked Talent (ultimate_psionics) | Psionic | - | no rest rule stated (no feature records) | 0 |
| 165 | Vitalist (ultimate_psionics) | Base.Psionic.PC.Trained | p.69 | no rest rule stated (21 feature records scanned) | 0 |
| 166 | War Mind (ultimate_psionics) | PC.Prestige.Psionic | p.385 | no rest rule stated (17 feature records scanned) | 0 |
| 167 | Wilder (ultimate_psionics) | Base.Psionic.PC.Intuitive | p.75 | no rest rule stated (26 feature records scanned) | 0 |
| 168 | Shifter (ultimate_wilderness) | Base.PC | p.26 | Fluidic Body (ultimate_wilderness p.81: rest-or-sleep) | 1 |

## 4. Races (39)

| # | Race record | Race page | Rest-rule rows | Trait records scanned |
|---|---|---|---|---|
| 1 | Catfolk (advanced_race_guide) | p.xx | no rest rule stated | 17 |
| 2 | Changeling (advanced_race_guide) | p.xx | no rest rule stated | 9 |
| 3 | Gillman (advanced_race_guide) | p.xx | no rest rule stated | 15 |
| 4 | Kitsune (advanced_race_guide) | p.xx | no rest rule stated | 13 |
| 5 | Nagaji (advanced_race_guide) | p.xx | no rest rule stated | 11 |
| 6 | Ratfolk (advanced_race_guide) | p.xx | no rest rule stated | 21 |
| 7 | Samsaran (advanced_race_guide) | p.xx | no rest rule stated | 9 |
| 8 | Strix (advanced_race_guide) | p.xx | no rest rule stated | 18 |
| 9 | Suli (advanced_race_guide) | p.xx | no rest rule stated | 17 |
| 10 | Vanara (advanced_race_guide) | p.xx | no rest rule stated | 13 |
| 11 | Vishkanya (advanced_race_guide) | p.xx | no rest rule stated | 17 |
| 12 | Wayang (advanced_race_guide) | p.xx | no rest rule stated | 12 |
| 13 | Aasimar (beastiary) | p.7 | no rest rule stated | 45 |
| 14 | Drow (beastiary) | p.xx | no rest rule stated | 21 |
| 15 | Duergar (beastiary) | p.xx | no rest rule stated | 20 |
| 16 | Goblin (beastiary) | p.xx | no rest rule stated | 19 |
| 17 | Hobgoblin (beastiary) | p.xx | no rest rule stated | 17 |
| 18 | Kobold (beastiary) | p.xx | no rest rule stated | 14 |
| 19 | Merfolk (beastiary) | p.xx | no rest rule stated | 14 |
| 20 | Orc (beastiary) | p.xx | no rest rule stated | 16 |
| 21 | Svirfneblin (beastiary) | p.xx | no rest rule stated | 16 |
| 22 | Tengu (beastiary) | p.xx | no rest rule stated | 15 |
| 23 | Tiefling (beastiary) | p.xx | no rest rule stated | 61 |
| 24 | Dhampir (bestiary_2) | p.xx | no rest rule stated | 13 |
| 25 | Fetchling (bestiary_2) | p.xx | no rest rule stated | 21 |
| 26 | Grippli (bestiary_2) | p.xx | no rest rule stated | 17 |
| 27 | Ifrit (bestiary_2) | p.xx | no rest rule stated | 20 |
| 28 | Oread (bestiary_2) | p.xx | no rest rule stated | 21 |
| 29 | Sylph (bestiary_2) | p.xx | no rest rule stated | 20 |
| 30 | Undine (bestiary_2) | p.xx | no rest rule stated | 21 |
| 31 | Skinwalker (bestiary_5) | p.xx | no rest rule stated | 75 |
| 32 | Rougarou (bestiary_6) | p.xx | no rest rule stated | 9 |
| 33 | Dwarf (core_rulebook) | p.xx | no rest rule stated | 45 |
| 34 | Elf (core_rulebook) | p.xx | no rest rule stated | 38 |
| 35 | Gnome (core_rulebook) | p.xx | Vivacious (inner_sea_races p.211: natural-healing - These gnomes recover 50% more hit points (minimum 1) whenever they recover hit points from rest.) | 37 |
| 36 | Half-Elf (core_rulebook) | p.xx | no rest rule stated | 31 |
| 37 | Half-Orc (core_rulebook) | p.xx | no rest rule stated | 38 |
| 38 | Halfling (core_rulebook) | p.xx | no rest rule stated | 40 |
| 39 | Human (core_rulebook) | p.xx | no rest rule stated | 43 |

## 5. House Rules per system (settles the E0.5 input to Q-3.5)

All evidence below was read, not run. Commands are in the last column so a reader can re-derive it. Paths are under `apps/desktop/src-tauri/src/`.

Where the six flags are consumed: `git grep -n -i -E 'ignore_slot_limits|feats_without_prerequisites|weapons_without_proficiency|armor_without_proficiency|ambidextrous' -- src crates apps/desktop/src-tauri/src ':!*.json'` (matching lines per file) lists `character_hub.rs` (40 hits), `character_custom.rs` (7), `print/blocks/extras.rs` (4), `print/blocks/print_pf_blocks.rs` (1). `grep -n -i -E 'overrides' sf_*.rs cpr_*.rs` prints nothing: no Starfinder or Cyberpunk RED file reads `Overrides`. The consumers are `list_feats_for_character` (`character_hub.rs:5153`, flag at `:5214`), `wear_check_in` (`:6315`, flags at `:6320`-`:6328`) and `move_copies_at_root` (`:6602`, flag at `:6639`), which take the Pathfinder `CharacterInput` and equipment model.

| House Rule | Pathfinder (PF1e) | Starfinder (SF1e) counterpart in the engine | Cyberpunk RED | Command |
|---|---|---|---|---|
| Slot limit off | exists, flag `ignore_slot_limits` | no equipment body-slot rule found in `sf_*.rs` (the `slot` hits are creation choice slots and drone companion slots, not wear slots) | none | `grep -n -i 'slot' sf_creation.rs sf_adapter.rs` |
| HP at level-up (seven methods) | new (E6.4) | **no**: SF level-up previews fixed per-level Hit Points as an engine row (`sf_level_up.rs:19-22`, `Hit Points 11 -> 18`); no per-level HP choice exists to replace | none (CPR HP is not level-based) | `grep -n -i 'hit point' sf_level_up.rs sf_creation.rs` |
| Ignore feat prerequisites | exists, flag `feats_without_prerequisites` | **a prerequisite rule exists**: the engine evaluates each feat's prerequisite and refuses with `sf_choices.feat_prerequisite` (`sf_choices.rs:49`, `:26`); no override consumer exists | none | `grep -n -i 'prereq' sf_choices.rs` (11 hits) |
| Ignore weapon proficiency | exists, flag `weapons_without_proficiency` | weapon proficiency is a printed/granted rule (`sf_sheet_print.rs:11`, "granted to every proficient character"); no wear/equip refusal exists to override | none | `grep -n -i 'proficien' sf_sheet_print.rs sf_drone_print.rs` |
| Ignore armour proficiency | exists, flag `armor_without_proficiency` | no armour-proficiency rule found in `sf_*.rs` | none | `grep -n -i -E 'armor.*proficien\|proficien.*armor' sf_*.rs` prints nothing |
| Ambidextrous | new (E6.3) | no two-weapon or off-hand rows in `sf_*.rs` | none | `grep -n -i -E 'two.weapon\|off.hand\|offhand\|ambidex' sf_*.rs` prints nothing |

**Reading for Q-3.5.** Rules that have an engine counterpart in Starfinder: only **Ignore feat prerequisites** (the engine's prerequisite check), and it needs a new consumer in `sf_choices.rs` before a flag could do anything. No flag has a working Starfinder consumer today. Starfinder would therefore show 0 House Rules unless E6.x adds that one consumer; showing a toggle whose flag nothing reads would be a stub, so the default recommended to the lead is: Starfinder shows no House Rules until the `sf_choices` consumer lands, or shows only Ignore feat prerequisites if E6.x wires it. This is a recommendation; the ruling is Q-3.5's.

**Cyberpunk RED:** none of the six has a counterpart (expected none, confirmed: `cpr_adapter.rs` and `cpr_catalog.rs` have no `Overrides`, prerequisite, proficiency or two-weapon rule; their `slot`/`hit point` hits are cyberware slots and the HP stat).

## 6. Does not cover

- **Class and race records the corpus does not describe.** 48 of 168 class records have no feature records at all; their "no rest rule stated" means the corpus holds no text for them, not that the book has none. Population: `class_feature` join count 0.
- **Base class versus archetype.** Feature records for a class include archetype and alternate-class features (field `class` is the base class name). A hit is not proof the base class has that rule; a miss on a base class does not rule out one in a feature the regexes do not phrase that way.
- **Regex recall.** Four patterns over descriptions; a rest rule written in other words (e.g. "at dawn", "after a night in the wild") is not found. No hand-curated ground-truth set was compared, so recall is unmeasured. Precision was spot-read on 8 base classes (Wizard, Cleric, Druid, Barbarian, Paladin, Monk, Sorcerer, Fighter) and some hits are archetype or incidental (e.g. Wizard's "Bonded Object" restore, Monk "Burn" from the Kineticist feature family).
- **`ability`, `race_trait_generic` and `template` records.** Not joined to a class or race here; an earlier looser scan (not kept; estimate) matched about 730 of 4,824 `ability` records, unattributed to a class.
- **Book prose beyond the corpus.** Rules in book text that PCGen did not ingest (core "Resting" and natural-healing rules: 1 HP per level per night, 8 hours) are not in the records and so are not rows; E2.4 writes those from the rules, not from this table.
- **Starfinder/CPR evidence** is a read of the source, not an execution; no app was launched.

## Appendix: the script that produced sections 3 and 4

Run from the repo root with `python3 -I census.py out.json`; it prints `classes, classes_with_hit, classes_without_feature_records, races, races_with_hit, races_without_trait_records`.

```python
import glob,json,re,collections,os,sys
CATS={
 'spell-preparation':r"\bprepar(e|es|ing)\b[^.]{0,30}\b(spells|extracts|formulae|orisons|cantrips)\b|\bwhen (he|she|you|they) prepares?\b|\bwhen preparing\b|\bspell preparation\b",
 'rest-or-sleep':r"\b(rest|rests|resting|sleep|sleeps) (for )?(at least )?(8|eight) hours\b|\b(8|eight) hours of (rest|sleep)\b|\bafter (a full night|resting|getting a full night)|\bnight's rest\b|\bnext time (he|she|you|they) rests?\b|\bwhen (he|she|you|they) rests\b|renewed after resting",
 'daily-refresh':r"\b(regain|regains|renew|renews|renewed|replenish\w*|recharge\w*|refresh\w*)\b[^.]{0,60}\b(each|every|per) (day|morning)\b|\b(each|every) (day|morning)\b[^.]{0,60}\b(regain|renew|refresh)",
 'natural-healing':r"\bnatural(ly)? heal|\bheals? (an additional|faster)|hit points? (from|per) rest|recovers? [^.]{0,40}from rest",
}
CR={k:re.compile(v,re.I) for k,v in CATS.items()}
def sent(t,m):
    t=t.replace('&nl;',' ');s=max(t.rfind('. ',0,m.start()),-1)+1;e=t.find('. ',m.end());e=len(t) if e<0 else e+1
    return re.sub(r'\s+',' ',t[s:e]).strip()[:170]
def page(d):
    for t in d.get('raw_tokens',[]):
        if t['key']=='SOURCEPAGE': return t['value']
    return d.get('source_page') or ''
def scan(files):
    out=[]
    for f in sorted(files):
        j=json.load(open(f));d=j['data'];t=d.get('description','') or ''
        for c,r in CR.items():
            m=r.search(t)
            if m: out.append((c,d.get('name'),page(d),f.split('/')[2],sent(t,m)));break
    return out
def cname(d): return d.get('class_id') or d.get('name') or d.get('key')
def ctype(d):
    ts=[t['value'] for t in d.get('raw_tokens',[]) if t['key']=='TYPE']
    return ts[0] if ts else ''
feat=collections.defaultdict(list)
for f in glob.glob('data/corpus/*/class_feature/*/*.json'):
    c=json.load(open(f))['data'].get('class')
    if c: feat[c.lower()].append(f)
rows=[]
for f in sorted(glob.glob('data/corpus/*/class/*.json')):
    j=json.load(open(f));d=j['data'];n=cname(d)
    fs=feat.get(n.lower(),[])
    rows.append((f,n,ctype(d),page(d),len(fs),scan(fs)))
rt=collections.defaultdict(list)
for f in glob.glob('data/corpus/*/race_trait/*/*.json'):
    d=json.load(open(f))['data'];rt[d.get('race_key','').lower()].append(f)
rrows=[]
for f in sorted(glob.glob('data/corpus/*/race/*.json')):
    d=json.load(open(f))['data'];k=d['key'];n=d['name'] if 'name' in d else k
    fs=rt.get(k.lower(),[])
    rrows.append((f,k,page(d),len(fs),scan(fs)))
json.dump({'classes':rows,'races':rrows},open(sys.argv[1],'w'))
print(len(rows),sum(1 for r in rows if r[5]),sum(1 for r in rows if not r[4]),len(rrows),sum(1 for r in rrows if r[4]),sum(1 for r in rrows if not r[3]))
```
