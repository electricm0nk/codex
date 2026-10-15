# SD-38 E7.1 — Cyberpunk RED starter builds

The three Cyberpunk RED starter characters (CUI §6, `decisions.md §14`). Each is a creation request
(`apps/desktop/src-tauri/src/cpr_adapter.rs` `starter_requests`) built by the Create screen's own path
(`cpr_creation::create_at_root`) when the app seeds a fresh install and when **Restore starter
characters** finds its folder missing (`character_hub.rs` `starter_seeds` → `seed_starters_at`). Every die
is fixed by the request (a picked template row or a seed), so a restore rebuilds the same character
byte for byte.

| Starter | Id | Marker | Role, method | Dice |
|---|---|---|---|---|
| Jax Moreno | `00000000-0000-0000-0000-000000000004` | `.default_character_seeded_4` | Solo, Streetrat | template row 3 picked (p. 75); Lifepath seed 2020 |
| Nyx Calder | `00000000-0000-0000-0000-000000000005` | `.default_character_seeded_5` | Netrunner, Complete Package | none for STATs (typed allocation); Lifepath seed 2077 |
| Imani Okafor | `00000000-0000-0000-0000-000000000006` | `.default_character_seeded_6` | Medtech, Edgerunner | seed 2045: STAT rolls 5 8 9 3 5 6 7 1 7 1 and the Lifepath |

## How every number below was produced

- **The builds** are the saved files of a Restore on an empty isolated app-data root:
  `cd apps/desktop/src-tauri && CPR_STARTER_DUMP_OUT=<dir> cargo test --locked -j 8 --bins starter_seed_tests::restore_writes_the_three -- --test-threads=8`
  (copies each starter's `cpr_*.txt` and its `load_saved_character` response `loaded.json`), rendered here by
  `python3 -I artifacts/epic_7/E7.1/builds_tables.py <dir> data/cyberpunk-red/corpus` (it also asserts
  each Skill Base it prints equals the sheet's `cpr.skill.*` row).
- **Every number on each loaded sheet** — 231 rows for Jax, 213 for Nyx, 223 for Imani — is re-derived
  from the book's formulas and the corpus records, with no engine code, by
  `python3 -I artifacts/epic_7/E7.1/crosscheck.py <dir> data/cyberpunk-red/corpus` → `CROSSCHECK_OK`
  (231/231, 213/213, 223/223 equal; 0 missing, 0 unclassified). It re-rolls Imani's ten STAT dice with
  its own SplitMix64 (the creation roller's recipe) and reads her STATs from the Medtech table columns;
  it reads Jax's row 3 from `streetrat_template.json`; it checks Nyx's typed allocation sums to the
  default 62 of `complete_package_points.json`. A planted wrong value (`cpr.cash` 460 → 461 in a copy)
  prints `WRONG cpr.cash` and `CROSSCHECK_FAILED 1`.
- The Rust test `restore_writes_the_three_cyberpunk_starters_and_a_second_restore_writes_nothing`
  asserts the headline values below as hand-computed constants.

## Hand values (the arithmetic)

| Figure | Jax Moreno | Nyx Calder | Imani Okafor | Rule |
|---|---|---|---|---|
| HP | 10 + 5 × ceil((8 + 7) / 2) = **50** | 10 + 5 × ceil((6 + 5) / 2) = **40** | 10 + 5 × ceil((7 + 5) / 2) = **40** | p. 79 |
| Seriously Wounded | ceil(50 / 2) = **25** | **20** | **20** | p. 79, 187 |
| Death Save | BODY **8** | BODY **6** | BODY **7** | p. 79 |
| Humanity Loss | Biomonitor 0 + Neural Link 7 + Sandevistan 7 = **14** | Neural Link 7 + Interface Plugs 7 = **14** | Biomonitor 0 + Cybereye 7 + Nasal Filters 2 + TeleOptics 3 = **12** | preset loss at creation, p. 111; pack totals p. 118 (CUI F-66: Solo 14, Medtech 12) |
| Humanity | 10 × 5 − 14 = **36** / 50 | 10 × 7 − 14 = **56** / 70 | 10 × 7 − 12 = **58** / 70 | p. 80 |
| Current EMP | floor(36 / 10) = **3** | floor(56 / 10) = **5** | floor(58 / 10) = **5** | p. 80 (D-6) |
| Maximum Humanity | 50 − 2 × 2 = **46** | 70 − 2 × 2 = **66** | 70 − 3 × 2 = **64** | p. 230: −2 per piece with a loss |
| Role Ability | Combat Awareness Rank 4, 4 points: Initiative Reaction 2 + Threat Detection 2 | Interface Rank 4 → **3** NET Actions | Medicine Rank 4, 4 points: Surgery 2, Pharmaceuticals 1 (Speedheal), Cryosystem Operation 1 | p. 142–151; D-7 |
| Deck | — | Cyberdeck (Standard Quality) **7** slots; Sword ×2, Armor, Worm, See Ya, Eraser = **6** one-slot programs | — | p. 196, 204 |
| Armour | Light Armorjack head and body, SP **11**, penalty 0 | same | same + Bulletproof Shield **10** HP | p. 351 |
| Cash | fixed kit **500**eb | 2,550 − 2,090 = **460**eb | fixed kit **500**eb | p. 99–106 |
| Facedown | COOL 7 + Rep 0 = **7** | COOL 6 + 0 = **6** | COOL 5 + 0 = **5** | p. 194 |

Nyx's 2,090eb: Cyberdeck (Standard Quality) 500 + Sword 50 × 2 + Armor 50 + Worm 50 + See Ya 20 + Eraser 20
+ Neural Link 500 + Interface Plugs 500 + Heavy Pistol 100 + 50 Basic H Pistol rounds 50 (10eb per 10,
p. 345) + Light Armorjack Body 100 + Head 100 = 2,090. Her skill points: 13 Basic skills (Athletics 2,
Brawling 2, Concentration 4, Conversation 2, Education 4, Evasion 4, First Aid 2, Human Perception 2,
Language (Streetslang) 2, Local Expert (Your Home) 4, Perception 4, Persuasion 2, Stealth 4 = 38) +
Electronics/Security Tech 6 (×2 = 12) + Basic Tech 6 + Cybertech 6 + Handgun 6 + Library Search 6 +
Cryptography 4 + Conceal/Reveal Object 4 + Streetwise 4 = 38 + 12 + 24 + 12 = **86** (p. 90); the Cultural
Origin language at 4 is free.

Imani's Edgerunner skills: the Medtech list at its printed levels (86 points, CUI F-25) with Shoulder Arms
6 → 4 (−2) and First Aid 2 → 4 (+2): still 86 (p. 88). Her "Science (choose 1)" line is Science (Biology).

Jax's Streetrat skills are the Solo list at its printed levels (86 points), unchanged (p. 86).

## Does not cover

- The sheet's printed lines (dice, rule text, Lifepath words) are checked for presence (the Lifepath
  prints), not word for word; the numbers are what `crosscheck.py` re-derives.
- Only these three builds; other Roles' starters, in-play dice and improvement after creation are not
  part of a starter.

## The builds

### Jax Moreno — Streetrat Solo (`00000000-0000-0000-0000-000000000004`)

**STATs** (p. 72):

| INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP |
|---|---|---|---|---|---|---|---|---|---|
| 5 | 8 | 7 | 4 | 7 | 7 | 6 | 7 | 8 | 5 |

**Skills** (level → Skill Base = current STAT + level, p. 80/128; every other skill is level 0):

| Skill | Level | Base |
|---|---|---|
| Athletics | 2 | DEX 7 + 2 = 9 |
| Brawling | 2 | DEX 7 + 2 = 9 |
| Concentration | 2 | WILL 7 + 2 = 9 |
| Conversation | 2 | EMP 3 + 2 = 5 |
| Education | 2 | INT 5 + 2 = 7 |
| Evasion | 6 | DEX 7 + 6 = 13 |
| First Aid | 6 | TECH 4 + 6 = 10 |
| Human Perception | 2 | EMP 3 + 2 = 5 |
| Language (Streetslang) | 2 | INT 5 + 2 = 7 |
| Local Expert (Your Home) | 2 | INT 5 + 2 = 7 |
| Perception | 6 | INT 5 + 6 = 11 |
| Persuasion | 2 | COOL 7 + 2 = 9 |
| Stealth | 2 | DEX 7 + 2 = 9 |
| Autofire | 6 | REF 8 + 6 = 14 |
| Handgun | 6 | REF 8 + 6 = 14 |
| Interrogation | 6 | COOL 7 + 6 = 13 |
| Melee Weapon | 6 | DEX 7 + 6 = 13 |
| Resist Torture/Drugs | 6 | WILL 7 + 6 = 13 |
| Shoulder Arms | 6 | REF 8 + 6 = 14 |
| Tactics | 6 | INT 5 + 6 = 11 |
| Language (Italian) | 4 | INT 5 + 4 = 9 |

**Saved sheet choices** (`cpr_sheet_choices.txt`, non-skill lines):

- `role = Solo`
- `role.rank = 4`
- `armor.body = Light Armorjack`
- `armor.head = Light Armorjack`
- `cyberware = Biomonitor`
- `cyberware = Neural Link`
- `cyberware = Sandevistan`
- `weapon = Assault Rifle`
- `weapon = Very Heavy Pistol`
- `weapon = Heavy Melee Weapon`
- `cash = 500`
- `ammo = 30 Basic VH Pistol Ammunition`
- `ammo = 70 Basic Rifle Ammunition`

**Role Ability sub-choices** (`cpr_role_choices.txt`): `points.Initiative Reaction = 2`, `points.Threat Detection = 2`

**Kit** (`cpr_kit.txt`, section | quantity | item):

- weapons_armor | 1 | Assault Rifle
- weapons_armor | 1 | Very Heavy Pistol
- weapons_armor | 1 | Heavy Melee Weapon
- weapons_armor | 30 | Basic VH Pistol Ammunition
- weapons_armor | 70 | Basic Rifle Ammunition
- weapons_armor | 1 | Light Armorjack Body Armor (SP11)
- weapons_armor | 1 | Light Armorjack Head Armor (SP11)
- outfit | 1 | Agent
- outfit | 2 | Leisurewear Footwear
- outfit | 3 | Leisurewear Jacket
- outfit | 1 | Leisurewear Mirrorshades
- outfit | 2 | Leisurewear Bottoms
- outfit | 2 | Leisurewear Top
- cyberware | 1 | Biomonitor
- cyberware | 1 | Neural Link
- cyberware | 1 | Sandevistan Speedware

**Lifepath** (`cpr_lifepath.txt`, table | entry | column | answer):

- Cultural Origins [0] Your (General) Cultural Region: Western European
- Your Personality [0] What Are You Like?: Stable and serious
- Dress and Personal Style: Clothing Style and Hairstyle [0] Clothing Style: Businesswear (Leadership, Presence, Authority)
- Dress and Personal Style: Clothing Style and Hairstyle [0] Hairstyle: Short and spiked
- Affectation You Are Never Without [0] Affectation You Are Never Without: Fingerless gloves
- Your Motivations and Relationships: What Do You Value Most? and How Do You Feel About Most People? [0] What Do You Value Most?: Knowledge
- Your Motivations and Relationships: What Do You Value Most? and How Do You Feel About Most People? [0] How Do You Feel About Most People?: People are untrustworthy. Don't depend on anyone.
- Things You Value the Most: Most Valued Person in Your Life? [0] Most Valued Person in Your Life?: A brother or sister
- Things You Value the Most: Most Valued Possession You Own? [0] Most Valued Possession You Own?: A tool
- Your Original Family Background [0] Original Background: Reclaimers
- Your Environment [0] Childhood Environment: In the heart of the Combat Zone, living in a wrecked building or other squat.
- Your Family Crisis [0] Background: Your family vanished. You are the only remaining member.
- Sweet Revenge! [0] What are You/They Gonna do About it?: Set out to murder or maim them.
- Your Tragic Love Affair(s) [1] What Happened?: Your lover went insane or cyberpsycho.
- Your Tragic Love Affair(s) [2] What Happened?: Your lover was kidnapped.
- Your Tragic Love Affair(s) [3] What Happened?: It just didn't work out.
- Your Life Goals [0] Life Goals: Gain power and control.
- Solo: What Kind of Solo are You? [0] Type: Corporate or Freelance Black Ops Agent
- Solo: What's Your Moral Compass Like? [0] Moral Compass: Will occasionally slip and do unethical or bad things, but it's rare.
- Solo: Who's Gunning for You? [0] Who?: A Fixer who sees you as a threat.
- Solo: What's Your Operational Territory? [0] Territory: The territory of a single Corporation

### Nyx Calder — Complete Package Netrunner (`00000000-0000-0000-0000-000000000005`)

**STATs** (p. 72):

| INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP |
|---|---|---|---|---|---|---|---|---|---|
| 7 | 6 | 6 | 7 | 6 | 5 | 6 | 6 | 6 | 7 |

**Skills** (level → Skill Base = current STAT + level, p. 80/128; every other skill is level 0):

| Skill | Level | Base |
|---|---|---|
| Athletics | 2 | DEX 6 + 2 = 8 |
| Brawling | 2 | DEX 6 + 2 = 8 |
| Concentration | 4 | WILL 5 + 4 = 9 |
| Conversation | 2 | EMP 5 + 2 = 7 |
| Education | 4 | INT 7 + 4 = 11 |
| Evasion | 4 | DEX 6 + 4 = 10 |
| First Aid | 2 | TECH 7 + 2 = 9 |
| Human Perception | 2 | EMP 5 + 2 = 7 |
| Language (Streetslang) | 2 | INT 7 + 2 = 9 |
| Local Expert (Your Home) | 4 | INT 7 + 4 = 11 |
| Perception | 4 | INT 7 + 4 = 11 |
| Persuasion | 2 | COOL 6 + 2 = 8 |
| Stealth | 4 | DEX 6 + 4 = 10 |
| Basic Tech | 6 | TECH 7 + 6 = 13 |
| Conceal/Reveal Object | 4 | INT 7 + 4 = 11 |
| Cryptography | 4 | INT 7 + 4 = 11 |
| Cybertech | 6 | TECH 7 + 6 = 13 |
| Electronics/Security Tech | 6 | TECH 7 + 6 = 13 |
| Handgun | 6 | REF 6 + 6 = 12 |
| Library Search | 6 | INT 7 + 6 = 13 |
| Streetwise | 4 | COOL 6 + 4 = 10 |
| Language (English) | 4 | INT 7 + 4 = 11 |

**Saved sheet choices** (`cpr_sheet_choices.txt`, non-skill lines):

- `role = Netrunner`
- `role.rank = 4`
- `armor.body = Light Armorjack`
- `armor.head = Light Armorjack`
- `cyberware = Neural Link`
- `cyberware = Interface Plugs`
- `weapon = Heavy Pistol`
- `deck = Cyberdeck (Standard Quality)`
- `deck.program = Sword`
- `deck.program = Sword`
- `deck.program = Armor`
- `deck.program = Worm`
- `deck.program = See Ya`
- `deck.program = Eraser`
- `cash = 460`
- `ammo = 50 Basic H Pistol Ammunition`

**Role Ability sub-choices** (`cpr_role_choices.txt`): none (the Netrunner's Interface has no sub-choice at Rank 4)

**Kit** (`cpr_kit.txt`, section | quantity | item):

- bought | 1 | Cyberdeck (Standard Quality)
- bought | 2 | Sword
- bought | 1 | Armor
- bought | 1 | Worm
- bought | 1 | See Ya
- bought | 1 | Eraser
- bought | 1 | Neural Link
- bought | 1 | Interface Plugs
- bought | 1 | Heavy Pistol
- bought | 50 | Basic H Pistol Ammunition
- bought | 1 | Light Armorjack Body Armor (SP11)
- bought | 1 | Light Armorjack Head Armor (SP11)

**Lifepath** (`cpr_lifepath.txt`, table | entry | column | answer):

- Cultural Origins [0] Your (General) Cultural Region: North American
- Your Personality [0] What Are You Like?: Stable and serious
- Dress and Personal Style: Clothing Style and Hairstyle [0] Clothing Style: Businesswear (Leadership, Presence, Authority)
- Dress and Personal Style: Clothing Style and Hairstyle [0] Hairstyle: Long and ratty
- Affectation You Are Never Without [0] Affectation You Are Never Without: Mirrorshades
- Your Motivations and Relationships: What Do You Value Most? and How Do You Feel About Most People? [0] What Do You Value Most?: Honesty
- Your Motivations and Relationships: What Do You Value Most? and How Do You Feel About Most People? [0] How Do You Feel About Most People?: I like almost everyone.
- Things You Value the Most: Most Valued Person in Your Life? [0] Most Valued Person in Your Life?: Yourself
- Things You Value the Most: Most Valued Possession You Own? [0] Most Valued Possession You Own?: A book or diary
- Your Original Family Background [0] Original Background: Edgerunners
- Your Environment [0] Childhood Environment: In a huge "megastructure" building controlled by a Corp or the City.
- Your Family Crisis [0] Background: Your family is cursed with a hereditary feud that has lasted for generations.
- Sweet Revenge! [0] What are You/They Gonna do About it?: Backstab them indirectly.
- Your Life Goals [0] Life Goals: Gain fame and recognition.
- Netrunner: What Kind of Runner are You? [0] Type: Part of a regular team of freelancers.
- Netrunner: What's Your Workspace Like? [0] Workspace: There are screens everywhere.
- Netrunner: If You Have a Partner, Who are They? [0] Who?: Family member
- Netrunner: Who are Some of Your Other Clients? [0] Who?: Local Fixers who send you clients.
- Netrunner: Where Do You Get Your Programs? [0] Where?: You have backdoors into a few Corporate warehouses.
- Netrunner: Who's Gunning for You? [0] Who?: Rival Netrunners who just don't like you.

### Imani Okafor — Edgerunner Medtech (`00000000-0000-0000-0000-000000000006`)

**STATs** (p. 72):

| INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP |
|---|---|---|---|---|---|---|---|---|---|
| 6 | 5 | 7 | 8 | 5 | 5 | 8 | 5 | 7 | 7 |

**Skills** (level → Skill Base = current STAT + level, p. 80/128; every other skill is level 0):

| Skill | Level | Base |
|---|---|---|
| Athletics | 2 | DEX 7 + 2 = 9 |
| Brawling | 2 | DEX 7 + 2 = 9 |
| Concentration | 2 | WILL 5 + 2 = 7 |
| Conversation | 6 | EMP 5 + 6 = 11 |
| Education | 6 | INT 6 + 6 = 12 |
| Evasion | 6 | DEX 7 + 6 = 13 |
| First Aid | 4 | TECH 8 + 4 = 12 |
| Human Perception | 6 | EMP 5 + 6 = 11 |
| Language (Streetslang) | 2 | INT 6 + 2 = 8 |
| Local Expert (Your Home) | 2 | INT 6 + 2 = 8 |
| Perception | 2 | INT 6 + 2 = 8 |
| Persuasion | 2 | COOL 5 + 2 = 7 |
| Stealth | 2 | DEX 7 + 2 = 9 |
| Basic Tech | 6 | TECH 8 + 6 = 14 |
| Cybertech | 4 | TECH 8 + 4 = 12 |
| Deduction | 6 | INT 6 + 6 = 12 |
| Paramedic | 6 | TECH 8 + 6 = 14 |
| Resist Torture/Drugs | 4 | WILL 5 + 4 = 9 |
| Science (Biology) | 6 | INT 6 + 6 = 12 |
| Shoulder Arms | 4 | REF 5 + 4 = 9 |
| Language (German) | 4 | INT 6 + 4 = 10 |

**Saved sheet choices** (`cpr_sheet_choices.txt`, non-skill lines):

- `role = Medtech`
- `role.rank = 4`
- `armor.body = Light Armorjack`
- `armor.head = Light Armorjack`
- `armor.shield = Bulletproof Shield`
- `cyberware = Biomonitor`
- `cyberware = Cybereye`
- `cyberware = Nasal Filters`
- `cyberware = TeleOptics @ eye 0`
- `weapon = Assault Rifle`
- `cash = 500`
- `ammo = 100 Basic Rifle Ammunition`
- `ammo = 10 Incendiary Rifle Ammunition`
- `ammo = 2 Smoke Grenade`

**Role Ability sub-choices** (`cpr_role_choices.txt`): `points.Medical Tech (Cryosystem Operation) = 1`, `points.Medical Tech (Pharmaceuticals) = 1`, `points.Surgery = 2`, `pick = Speedheal`

**Kit** (`cpr_kit.txt`, section | quantity | item):

- weapons_armor | 1 | Assault Rifle
- weapons_armor | 100 | Basic Rifle Ammunition
- weapons_armor | 10 | Incendiary Rifle Ammunition
- weapons_armor | 2 | Smoke Grenade
- weapons_armor | 1 | Light Armorjack Body Armor (SP11)
- weapons_armor | 1 | Light Armorjack Head Armor (SP11)
- weapons_armor | 1 | Bulletproof Shield
- outfit | 1 | Agent
- outfit | 1 | Airhypo
- outfit | 1 | Handcuffs
- outfit | 1 | Flashlight
- outfit | 3 | Generic Chic Jacket
- outfit | 1 | Glow Paint
- outfit | 1 | Medtech Bag
- outfit | 1 | Leisurewear Footwear
- outfit | 3 | Leisurewear Bottoms
- outfit | 5 | Leisurewear Top
- cyberware | 1 | Biomonitor
- cyberware | 1 | Cybereye
- cyberware | 1 | Nasal Filters
- cyberware | 1 | TeleOptics

**Lifepath** (`cpr_lifepath.txt`, table | entry | column | answer):

- Cultural Origins [0] Your (General) Cultural Region: South/Central American
- Your Personality [0] What Are You Like?: Rebellious, antisocial, and violent
- Dress and Personal Style: Clothing Style and Hairstyle [0] Clothing Style: High Fashion (Exclusive, Designer, Couture)
- Dress and Personal Style: Clothing Style and Hairstyle [0] Hairstyle: Short and curly
- Affectation You Are Never Without [0] Affectation You Are Never Without: Spiked boots or heels
- Your Motivations and Relationships: What Do You Value Most? and How Do You Feel About Most People? [0] What Do You Value Most?: Honor
- Your Motivations and Relationships: What Do You Value Most? and How Do You Feel About Most People? [0] How Do You Feel About Most People?: I hate almost everyone.
- Things You Value the Most: Most Valued Person in Your Life? [0] Most Valued Person in Your Life?: A friend
- Things You Value the Most: Most Valued Possession You Own? [0] Most Valued Possession You Own?: A photograph
- Your Original Family Background [0] Original Background: Edgerunners
- Your Environment [0] Childhood Environment: In a decaying, once upscale neighborhood, now holding off the boosters to survive.
- Your Family Crisis [0] Background: Your family was scattered to the winds due to misfortune.
- Your Friends [1] Friend's Relationship to You: Like a younger sibling to you.
- Your Enemies [1] Enemy: Partner or coworker
- Your Enemies [1] What Caused it? Who's been Wronged? (choose): One of you was a romantic rival.
- Your Enemies [1] What Can They Throw at You?: An entire city or government or agency.
- Sweet Revenge! [0] What are You/They Gonna do About it?: Verbally attack them.
- Your Life Goals [0] Life Goals: Hunt down those responsible for your miserable life and make them pay.
- Medtech: What Kind of Medtech are You? [0] Type: General Practitioner
- Medtech: Tell Us About Your Partner(s). [0] Who?: Secret partner with Corporate connections
- Medtech: What's Your Workspace Like? [0] Workspace: It's not state-of-the-art anymore, but it's comfortable to you.
- Medtech: Who are Your Main Clients? [0] Who?: Local Solos or other combat types who use you for medical help.
- Medtech: Where Do You Get Your Supplies? [0] Where?: Corporate Execs or Trauma Team supply you with stuff in exchange for your services.
