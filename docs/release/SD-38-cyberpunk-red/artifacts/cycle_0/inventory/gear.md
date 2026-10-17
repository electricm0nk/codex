# SD-38 inventory: Cyberpunk RED weapons, armor, gear, Night Market

Source: `/home/ubuntu/workspace/inbox/cyberpunk-red/pages/pNNN.txt` (v1.22). All figures below were read from those files.
Run every command from `/home/ubuntu/workspace/inbox/cyberpunk-red/pages`.

## 0. Page map (read this first)

The brief said "PDF pages ~91-98". The real mapping is: `pNNN` file = printed page + 1.
Proof: `grep -E '^\s*[0-9]{2,3}\s*$' p100.txt | tail -1` prints `99`. The same offset holds for `p351` (printed 350) and `p386` (printed 385).
All in-text page refs ("pg. 340") are PRINTED numbers. Add 1 to get the file.

| Topic | Printed pg | File(s) |
|---|---|---|
| Weapon categories + melee table (chargen chapter) | 91-92 | p092, p093 |
| Ranged weapon table + basic ammo | 93-94 | p094, p095 |
| Exotic weapons summary | 95 | p096 |
| Attachments / special ammo pointer, Armor text | 96 | p097 |
| Armor table | 97 | p098 |
| Starting weapons/armor per Role (Streetrat/Edgerunner) | 98-99 | p099, p100 |
| Outfit intro + gear table | 99-101 | p100, p101, p102 |
| Fashion table | 102 | p103 |
| Starting outfit per Role | 103 | p104 |
| Complete Package budget (2,550eb) + example | 104 | p105 |
| Free fashion (800eb), lifestyle at chargen | 105 | p106 |
| Night Market intro, generator tables | 338-339 | p339, p340 |
| Midnight Market + appendix index + melee table | 340 | p341 |
| Ranged table (appendix) | 341 | p342 |
| Weapon quality + brand examples + attachments intro | 342 | p343 |
| Attachment list and text | 343-344 | p344, p345 |
| Ammunition list and text | 344-347 | p345 (Clip Chart + intro), p346, p347, p348 |
| Exotic weapon list and text | 347-349 | p348, p349, p350 |
| Master Armor List | 350-351 | p351, p352 |
| Master Gear List + descriptions | 351-355 | p352, p353, p354, p355, p356 |
| Fashion (appendix) | 356 | p357 |
| Street Drugs | 357-358 | p358, p359 |
| Services and Entertainment | 376 | p377 |
| Lifestyle and Housing, Real Estate | 377-380 | p378, p379, p380, p381 |
| Buying and Selling + Price Categories | 385 | p386 |
| Vehicles (NOT in appendix) | 190-191 | p191 (land, sea), p192 (air) |
| Armor rules in combat, shields | 183-186 | p184, p185, p186, p187 |
| Sample character sheet (gear fields) | 408 per its own ref | p121 |

Domains not covered here: cyberware tables (p360-369 and p113-117), programs/ICE (p370-372), cyberdeck hardware (p369), Home Defenses / NET architecture (p373-376). Those belong to other inventories.

## 1. Price category scale (item 7)

Source: `p386.txt`, printed 385, under "Buying and Selling". Exactly as printed:

| Price Category | Market Price |
|---|---|
| Cheap | 10eb |
| Everyday | 20eb |
| Costly | 50eb |
| Premium | 100eb |
| Expensive | 500eb |
| Very Expensive | 1,000eb |
| Luxury | 5,000eb |
| Super Luxury | 10,000eb and up |

Printed rules on the same page:
- "Everything you buy in Cyberpunk RED falls into one of eight Price Categories".
- "Without the help of an experienced Fixer or one of their Night Markets, only items up to Premium can be purchased."
- "If it's Expensive and you don't know a Fixer, forget about getting it at all."

Label variance: tables print "V. Expensive" (p101, p098, p103) and "Very Expensive" (p352, p351). Same tier.
Count is 8: the sentence "eight Price Categories" is printed on p386, and the table has 8 rows.

## 2. Melee weapons (item 1)

Printed twice, identical: `p093.txt` (printed 92) and `p341.txt` (printed 340).
Columns as printed: Melee Weapon Type, Example Melee Weapons, # of Hands Required, Damage, ROF, Can be Concealed?, Cost.

| Type | Examples (printed) | Hands | Damage | ROF | Conceal | Cost |
|---|---|---|---|---|---|---|
| Light Melee Weapon | Combat Knife, Tomahawk | Varies by type | 1d6 | 2 | YES | 50eb (Costly) |
| Medium Melee Weapon | Baseball Bat, Crowbar, Machete | Varies by type | 2d6 | 2 | NO | 50eb (Costly) |
| Heavy Melee Weapon | Lead Pipe, Sword, Spiked Bat | Varies by type | 3d6 | 2 | NO | 100eb (Premium) |
| Very Heavy Melee Weapon | Chainsaw, Sledgehammer, Helicopter Blades, Naginata | Varies by type | 4d6 | 1 | NO | 500eb (Expensive) |

Count: 4 rows. Command (run): `grep -cE '^\s+(Light Melee|Medium Melee|Heavy Melee|Very Heavy)\s' p093.txt p341.txt` which printed 4 and 4 (once per file).

Notes:
- "Number of Hands Required": "Varies by type". The hand count is per weapon and is NOT given in the table. UNCLEAR. No per-weapon hands list exists in these pages. The only hand data printed is in exotics (Stun Baton "One-Handed", Kendachi Mono-Three "Two-Handed").
- Melee is "generally linked to the DEX STAT" (p092). Skill name on the sheet is Melee Weapon (DEX) (p121).
- No melee weapon has a magazine. No named melee weapon list exists beyond the examples above.

## 3. Ranged weapons (item 1)

Printed twice, identical: `p095.txt` (printed 94) and `p342.txt` (printed 341).
Columns as printed: Weapon Type, Weapon Skill, Single Shot Damage, Standard Magazine (with ammo type in parentheses), Rate of Fire (ROF), Number of Hands Required, Can be Concealed?, Cost. A second line under each row holds "Alt. Fire Modes & Special Features".

| Type | Skill | Dmg | Std Mag (ammo) | ROF | Hands | Conceal | Cost | Alt Fire / Special |
|---|---|---|---|---|---|---|---|---|
| Medium Pistol | Handgun | 2d6 | 12 (M Pistol) | 2 | 1 | YES | 50eb (Costly) | None |
| Heavy Pistol | Handgun | 3d6 | 8 (H Pistol) | 2 | 1 | YES | 100eb (Premium) | None |
| Very Heavy Pistol | Handgun | 4d6 | 8 (VH Pistol) | 1 | 1 | NO | 100eb (Premium) | None |
| SMG | Handgun | 2d6 | 30 (M Pistol) | 1 | 1 | YES | 100eb (Premium) | Autofire (3) / Suppressive Fire |
| Heavy SMG | Handgun | 3d6 | 40 (H Pistol) | 1 | 1 | NO | 100eb (Premium) | Autofire (3) / Suppressive Fire |
| Shotgun | Shoulder Arms | 5d6 | 4 (Slug) | 1 | 2 | NO | 500eb (Expensive) | Shotgun Shell |
| Assault Rifle | Shoulder Arms | 5d6 | 25 (Rifle) | 1 | 2 | NO | 500eb (Expensive) | Autofire (4) / Suppressive Fire |
| Sniper Rifle | Shoulder Arms | 5d6 | 4 (Rifle) | 1 | 2 | NO | 500eb (Expensive) | None |
| Bows & Crossbows | Archery | 4d6 | N/A (Arrow) | 1 | 2 | NO | 100eb (Premium) | Arrows |
| Grenade Launcher | Heavy Weapons | 6d6 | 2 (Grenade) | 1 | 2 | NO | 500eb (Expensive) | Explosive |
| Rocket Launcher | Heavy Weapons | 8d6 | 1 (Rocket) | 1 | 2 | NO | 500eb (Expensive) | Explosive |

Count: 11 rows. Command: `grep -c 'Alt. Fire Modes' p095.txt p342.txt` prints 11 and 11.
Pdftotext quirk: the Medium Pistol row prints ROF "2" and Hands "1", columns appear in the order "Mag, ROF, Hands" (e.g. `12 / 2 / 1`). The printed header order is Mag, ROF, Hands, Conceal. I read left to right; matches the headers. Cross-check: Heavy Pistol ROF 2, Very Heavy ROF 1 matches the usual game. Marked verified by two identical printings, not by outside source.

Open items:
- Ammo labels in the magazine column: "M Pistol", "H Pistol", "VH Pistol", "Slug", "Rifle", "Arrow", "Grenade", "Rocket". Shotgun Shell is a separate "Alt Fire" and the printed std mag 4 is "Slug". The Clip Chart (section 5) lists Shotgun 4/8/16.
- Autofire (3) and (4): multiplier numbers per p174 (printed 173): "Autofire (3 for SMGS, 4 for Assault Rifles)". Autofire costs an Action and 10 bullets (p174). These live in play rules, not sheet totals.
- Hit locations and damage multipliers: p171: Head aimed shot "Multiply the damage that gets through your target's head armor by 2". Aimed shot is -8.

### Weapon quality (p343, printed 342)

Poor Quality: "malfunction whenever you roll a Critical Failure (a 1 on your Attack Check)". Excellent: "+1 to your Attack Check". Exotic weapons are always Average Quality unless stated. Cost by quality:

| Standard cost | Poor | Excellent |
|---|---|---|
| 50eb (Costly) | 20eb (Everyday) | 100eb (Premium) |
| 100eb (Premium) | 50eb (Costly) | 500eb (Expensive) |
| 500eb (Expensive) | 100eb (Premium) | 1,000eb (Very Expensive) |

Brand examples (12 rows) printed on p343 for Poor/Standard/Excellent. Count command: `awk '/Pistol|Bow|Crossbow|SMG|Shotgun|Rifle|Launcher/ && /"|Arms|Militech|GunMart|Eagletech/{c++} END{print c}' p343.txt` is fragile. ESTIMATE: 12 rows by reading (Medium Pistol, Heavy Pistol, Very Heavy Pistol, Bow, Crossbow, SMG, Heavy SMG, Shotgun, Assault Rifle, Sniper Rifle, Grenade Launcher, Rocket Launcher). Brand names are flavor. Print only.

## 4. Weapon attachments (item 2)

Source: list `p344.txt` (printed 343), text `p344` and `p345`. Intro: `p343` bottom and `p097` (printed 96).
Rule: "Each Non-Exotic Ranged Weapon has 3 Attachment Slots". You pay the slot cost of each attachment. Same attachment twice does nothing.

| Attachment | Cost | Eligible | Slots | Effect (short) |
|---|---|---|---|---|
| Bayonet | 100eb (Premium) | Non-Exotic Ranged fired with Shoulder Arms | 1 (not stated) | Usable as Light Melee Weapon; cannot be concealed |
| Drum Magazine | 500eb (Expensive) | All Non-Exotic Ranged except Bows/Crossbows | 1 (not stated) | Mag = Drum column of Clip Chart; cannot be concealed |
| Extended Magazine | 100eb (Premium) | Same as Drum | 1 (not stated) | Mag = Extended column; cannot be concealed |
| Grenade Launcher Underbarrel | 500eb (Expensive) | Shoulder Arms Non-Exotic | 2 | Two-hand: 1 grenade; cannot be concealed |
| Infrared Nightvision Scope | 500eb (Expensive) | All Non-Exotic Ranged | 1 (not stated) | Darkness/smoke/fog penalties reduced to 0 |
| Shotgun Underbarrel | 500eb (Expensive) | Shoulder Arms Non-Exotic | 2 | Two-hand: Shotgun, 2 shots; cannot be concealed |
| Smartgun Link | 500eb (Expensive) | All Non-Exotic Ranged | 2 | +1 to Ranged Attack Check; needs Neural Link + plugs/grip |
| Sniping Scope | 100eb (Premium) | All Non-Exotic Ranged | 1 (not stated) | +1 at 51m+ single shot or Aimed Shot; sight to 800m/yds |

Count: 8. Command: `cat p344.txt p345.txt | awk '/^ *▶ (Bayonet|Drum|Extended|Grenade|Infrared|Shotgun|Smartgun|Sniping)/{c++} END{print c}'` printed 8.
UNCLEAR: slot cost is printed only where it is above one ("Requires 2 Attachment Slots" appears on three entries). "1 slot" for the others is inferred. The text for Bayonet, Drums, Extended, Scope never states "1 slot". Mark ESTIMATE for the five 1-slot values.
Table print quirk (p344): the table is two columns of four. "Infrared / Nightvision Scope" prints as two lines; the name is "Infrared Nightvision Scope" per the heading. "Grenade Launcher / Underbarrel" likewise.

### Clip Chart (p345, printed 344)

| Type | Standard | Extended | Drum |
|---|---|---|---|
| Medium Pistol | 12 | 18 | 36 |
| Heavy Pistol | 8 | 14 | 28 |
| Very Heavy Pistol | 8 | 14 | 28 |
| SMG | 30 | 40 | 50 |
| Heavy SMG | 40 | 50 | 60 |
| Shotgun | 4 | 8 | 16 |
| Assault Rifle | 25 | 35 | 45 |
| Sniper Rifle | 4 | 8 | 12 |
| Grenade Launcher | 2 | 4 | 6 |
| Rocket Launcher | 1 | 2 | 3 |

Count: 10 rows. Command: `sed -n '/Clip Chart/,/Ammunition/p' p345.txt | grep -cE '^ +(Medium|Heavy|Very|SMG|Shotgun|Assault|Sniper|Grenade|Rocket)'` printed 10 (run).

## 5. Ammunition (item 2)

Table: `p346.txt` (printed 345). Per-type text: p346, p347, p348. Basic ammo also in `p095` (printed 94).
Pricing rule (p345): "Grenades and Rockets are purchased individually. All other ammunition is purchased in increments of 10." Basic: "10eb per 10 units (Cheap)" (p095).
Count: 13 rows. Command: `grep -cE '^\s+[A-Z][A-Za-z-]+ Ammunition\s+[0-9]' p346.txt` printed 13. Detail headings: `cat p346.txt p347.txt p348.txt | awk '/^ *▶ .*Ammunition/{c++} END{print c}'` printed 13.

| Ammunition | Cost | Types available | Effect (short) |
|---|---|---|---|
| Basic | 10eb (Cheap) | All except Grenades and Rockets | None |
| Armor-Piercing | 100eb (Premium) | All except Shotgun Shells | Ablates armor by 2 instead of 1 |
| Biotoxin | 500eb (Expensive) | Arrows, Grenades only | No damage; DV15 Resist Torture/Drugs; fail = 3d6 direct to HP |
| EMP | 500eb (Expensive) | Grenades only | No damage; DV15 Cybertech; GM picks 2 cyberware/electronics off 1 min |
| Expansive | 100eb (Premium) | Arrows, Bullets, Slugs | Foreign Object crit rolls a second crit |
| Flashbang | 100eb (Premium) | Grenades only | DV15 Resist Torture/Drugs; fail = Damaged Eye and Ear 1 min |
| Incendiary | 100eb (Premium) | Arrows, Bullets, Grenades, Shotgun Shells | Ignite: 2 dmg direct to HP each turn end |
| Poison | 100eb (Premium) | Arrows, Grenades only | DV13 Resist Torture/Drugs; fail = 2d6 direct to HP |
| Rubber | 10eb (Cheap) | Arrows, Bullets, Slugs | No crits, no ablation, cannot drop target below 1 HP |
| Sleep | 500eb (Expensive) | Arrows, Grenades | DV13 Resist Torture/Drugs; fail = Prone and Unconscious 1 min |
| Smart | 500eb (Expensive) | Arrows, Bullets, Rockets | Needs Targeting Scope cyberware; second chance to hit on near miss |
| Smoke | 50eb (Costly) | Grenades only | 10m by 10m smoke 1 min; -4 |
| Teargas | 50eb (Costly) | Grenades only | DV13 Resist Torture/Drugs; fail = Damaged Eye 1 min |

Grenade ammo (bought one at a time), from the "Ammo Types Available" lines: Armor-Piercing (printed "All except Shotgun Shells", so it includes Grenades), Biotoxin, EMP, Flashbang, Incendiary, Poison, Sleep, Smoke, Teargas = 9 types. Derived by reading, no command. Arrow-capable: Basic, AP, Biotoxin, Expansive, Incendiary, Poison, Rubber, Sleep, Smart.
Starting kits name Flashbang Grenade, Teargas Grenade, Smoke Grenade (p099). They map to Grenade Launcher ammo types; they can also be thrown (p345 margin note: "Grenade ammunition can be either thrown or loaded into a Grenade Launcher"). Thrown-grenade skill and damage: UNCLEAR in these pages.
Explosive damage is "10m/yd by 10m/yd" (p175, outside domain).

## 6. Exotic weapons (item 2)

Summary table: `p096.txt` (printed 95), columns Weapon / Description and Data / Cost. Appendix list p348 (printed 347); full text p348, p349, p350.
Rule (p348): "All Exotic Weapons are Average Weapon Quality and are incompatible with all Weapon Attachments and Non-Basic Ammunition unless otherwise noted."
Count: 14 rows. Command: `awk '/[0-9]eb \(/{c++} END{print c}' p096.txt` printed 14. Detail headings (14 exotics + Smoke + Teargas headings share the span): `cat p348.txt p349.txt p350.txt | awk '/^ *▶ /{c++} END{print c}'` printed 16, which is 14 exotics + the Smoke and Teargas ammo headings that sit on p348.

| Exotic | Cost | Base weapon / key mechanics (from text) |
|---|---|---|
| Air Pistol | 100eb (Premium) | Exotic Medium Pistol; paint balls, no damage; acid lowers armor SP by 1 per hit; no crits. (p096 summary says "Very Heavy Pistol"; p348 text says "Medium Pistol". CONFLICT, text on p348 governs by detail; UNCLEAR which is right.) |
| Battleglove | 1,000eb (Very Expensive) | Gauntlet; 3 Cyberarm/Cyberlimb option slots; cannot be concealed |
| Constitution Arms Hurricane Assault Weapon | 5,000eb (Luxury) | Exotic 2 ROF Shotgun; drum 16; BODY 11+ unless mounted; 2 Actions to reload; no Aimed Shots |
| Dartgun | 100eb (Premium) | Exotic Very Heavy Pistol; clip 8 Non-Basic Arrows |
| Flamethrower | 500eb (Expensive) | Exotic Shotgun, Heavy Weapons skill; incendiary shells only; 4 dmg to HP per turn while ignited; no crits, no Aimed Shots |
| Kendachi Mono-Three | 5,000eb (Luxury) | Excellent Quality Two-Handed Very Heavy Melee; ignores armor under SP11 with biometric key |
| Malorian Arms 3516 | 10,000eb (Super Luxury) | Excellent Quality Very Heavy Pistol; 5d6; permanent Smartgun Link |
| Microwaver | 500eb (Expensive) | Exotic Very Heavy Pistol; DV15 Cybertech instead of damage; battery pack per 8 shots, 50eb (Costly) |
| Militech "Cowboy" U-56 Grenade Launcher | 5,000eb (Luxury) | Exotic 2 ROF Grenade Launcher; magazine 4; BODY 11+ unless mounted; takes all grenade ammo |
| Rhinemetall EMG-86 Railgun | 5,000eb (Luxury) | Exotic Assault Rifle; Heavy Weapons skill; holds 4; ignores armor under SP11; BODY 11+; no Autofire/Aimed Shot |
| Shrieker | 500eb (Expensive) | Exotic Very Heavy Pistol; DV15 Resist Torture/Drugs; Damaged Ear; battery pack 50eb (Costly) |
| Stun Baton | 100eb (Premium) | One-Handed Exotic Medium Melee; unconscious at 1 HP; no crits, no ablation |
| Stun Gun | 100eb (Premium) | Exotic Heavy Pistol; unconscious at 1 HP; battery pack 50eb (Costly) |
| Tsunami Arms Helix | 5,000eb (Luxury) | Exotic Assault Rifle; Autofire skill only; holds 40; 20 bullets per attack; BODY 11+ |

Printed GM note (p096): "Allowing Players access to Luxury or higher Price Category items at Character Generation could break your game."
Pdftotext quirk, p096: the margin note "You can find more information about weapon qualities on Page 342" interleaves with the Cowboy and Railgun rows. The Railgun description ("Assault Rifle that ignore armor lower than SP 11. Fired with the Heavy / Weapons Skill. Requires BODY 11+ to fire.") wraps across two lines.

## 7. Armor (item 3)

Printed three times with identical numbers: `p098.txt` (printed 97, with descriptions), `p186.txt` (printed 185, combat chapter), `p351.txt` (printed 350, "Master Armor List", with expanded text). Shield table also in `p185.txt` (printed 184).
Columns: Armor Type, [Description], Damage Stopping Power (SP), Armor Penalty (Minimum 0), Cost.

| Armor | SP | Armor Penalty | Cost |
|---|---|---|---|
| Leathers | 4 | None | 20eb (Everyday) |
| Kevlar | 7 | None | 50eb (Costly) |
| Light Armorjack | 11 | None | 100eb (Premium) |
| Bodyweight Suit | 11 | None | 1,000eb (Very Expensive) |
| Medium Armorjack | 12 | -2 REF, DEX, and MOVE | 100eb (Premium) |
| Heavy Armorjack | 13 | -2 REF, DEX, and MOVE | 500eb (Expensive) |
| Flak | 15 | -4 REF, DEX, and MOVE | 500eb (Expensive) |
| Metalgear | 18 | -4 REF, DEX and MOVE | 5,000eb (Luxury) |
| Bulletproof Shield | "10 HP, which is reduced by damage" | None, but always takes up one arm | 100eb (Premium) |

Count: 9 rows (8 armors + 1 shield). Command: `sed -n 1,18p p351.txt | awk '/[0-9]eb ?\(/{c++} END{print c}'` printed 8 (it misses the Shield, whose cost prints on a later line). Row total by reading: 9. Bodyweight Suit and Heavy/Medium rows: 8 SP-bearing armors confirmed in all three printings.
Typos: `p351.txt` prints "100eb(Premium)" for Medium Armorjack (no space). "Metalgear   ®" and "Kevlar ®" print the registered mark. "Metalgear" penalty reads "DEX and MOVE" without the Oxford comma on p098 and p351 and p186.

Armor text rules (p097 printed 96; p185 printed 184; p351):
- "Armor must be purchased individually for either the head or body locations."
- "Wearing even a single piece of heavier armor will lower your REF, DEX, and MOVE by the most punishing Armor Penalty of armor you are wearing. You take this penalty only once even though you are likely wearing armor on both your body and head."
- "This penalty can even leave your Character (at a minimum of MOVE 0) completely immobile."
- "SP gained by armor does not 'stack;' Only your highest source of SP in a location determines your SP for that location."
- "All your worn armor in a location is ablated (SP lowered by one) simultaneously whenever you take damage."
- Bodyweight Suit (p351): "isn't bought in two pieces, and must always be worn on both your body and head location. Each location has its own SP11. ... You can't wear more than one Bodyweight Suit. Wearing a Bodyweight Suit adds one Hardware only Option Slot to a Cyberdeck connected to it."
- Ablation (p187, printed 186): "If you ended up taking any damage, your armor on that location is still ablated, reducing its SP by 1 point, until it is repaired." Ablation is play-time state, not a sheet total. Armor-Piercing ammo ablates 2 (p346). Rubber and stun weapons do not ablate.
- Head hits: Aimed Shot at the head multiplies damage that gets through head armor by 2 (p171). Non-aimed hits always strike the body location (p187).
- Shield (p184 printed 183): equipping/dropping takes an Action; interposing a shield means you cannot dodge; shield takes the whole attack to its HP; a shield at 0 HP is destroyed. Shield ties up one hand, so a Two-Handed weapon cannot be used with it (implied by "always takes up one arm"; the rule text on p184 says "a hand to carry a shield, it cannot be used to do anything else").
- Other shield: Corpse shield, HP = BODY STAT the corpse had; Human Shield HP = BODY on death (p185). Not purchasable. Print or skip.

The sample character sheet (`p121.txt`) has armor boxes: Head, Body, Shield, each with "sp" and "penalty", plus the printed line "penalty applies to REF, DEX & MOVE".

No helmets-as-separate-items exist. "Head Armor" and "Body Armor" are the same armor type bought once per location. Starting kits say e.g. "Light Armorjack Body Armor (SP11)" and "Light Armorjack Head Armor (SP11)" (p099).

## 8. Gear

### 8a. Starter gear table (chargen chapter)

`p101.txt` (printed 100) and `p102.txt` (printed 101). Columns: Item, Description & Data, Cost (price category printed under cost). One-line description each.
Count: 47 items. Command: `awk '{n=gsub(/[0-9]eb ?\(/,"&"); c+=n} END{print c}' p101.txt p102.txt` printed 47.
Typo, p102: "Road Flare ... 10b (Cheap)" (should be 10eb).

### 8b. Master Gear List

`p352.txt` (printed 351). Two item/cost column pairs. Full descriptions p353-p356.
Count: 53 items. Command: `awk '{n=gsub(/[0-9]eb ?\(/,"&"); c+=n} END{print c}' p352.txt` printed 53. All 53 have a description entry: I checked every name with `grep -cF` against `cat p353.txt p354.txt p355.txt p356.txt` (no "NO DESC" lines).
Master minus starter = 6 extra: Cryopump, Cryotank, Linear Frame Beta, Linear Frame Sigma, plus Cyberdeck splits into Excellent/Poor/Standard (3 rows where the starter table has 1; 2 extra). 53 - 47 = 6 = 4 + 2.

| Item | Cost | Category | Sheet-relevant text |
|---|---|---|---|
| Agent | 100eb (Premium) | Tech | +2 Library Search; +2 Wardrobe & Style only if you wear what it suggests; no stacking |
| Airhypo | 50eb (Costly) | Medical | Delivers one dose as an Action |
| Anti-Smog Breathing Mask | 20eb (Everyday) | Survival | Immune to inhaled toxins |
| Audio Recorder | 100eb (Premium) | Electronics | 24 hours on a Memory Chip |
| Auto Level Dampening Ear Protectors | 1,000eb (Very Expensive) | Survival | Immune to deafness from loud noise |
| Binoculars | 50eb (Costly) | Optics | x2 or x3 |
| Braindance Viewer | 1,000eb (Very Expensive) | Electronics | Plays braindance |
| Bug Detector | 500eb (Expensive) | Electronics | Beeps within 2m/yds of a listening device |
| Carryall | 20eb (Everyday) | Storage | Bag |
| Chemical Analyzer | 1,000eb (Very Expensive) | Science | Tests substances as an Action |
| Computer | 50eb (Costly) | Electronics | Laptop or desktop |
| Cryopump | 5,000eb (Luxury) | Medical | 1 charge; refuel 50eb (Costly); only a Medtech operates |
| Cryotank | 5,000eb (Luxury) | Medical | DV13 Medical Tech; heals at double rate; 30 HP cover |
| Cyberdeck (Excellent Quality) | 1,000eb (Very Expensive) | Netrunning | 9 slots |
| Cyberdeck (Poor Quality) | 100eb (Premium) | Netrunning | 5 slots |
| Cyberdeck (Standard Quality) | 500eb (Expensive) | Netrunning | 7 slots |
| Disposable Cell Phone | 50eb (Costly) | Electronics | Phone |
| Drum Synthesizer | 500eb (Expensive) | Music | Needs amp |
| Duct Tape | 20eb (Everyday) | Tools | Optionally glows |
| Electric Guitar or another Instrument | 500eb (Expensive) | Music | Needs amp |
| Flashlight | 20eb (Everyday) | Light | 100m/yd beam, 10 hours |
| Food Stick | 10eb (Cheap) | Food | One meal |
| Glow Paint | 20eb (Everyday) | Tools | Spray can |
| Glow Stick | 10eb (Cheap) | Light | 4m/yd, 10 hours, one use |
| Grapple Gun | 100eb (Premium) | Tools | 30m/yds; line 10 HP |
| Handcuffs | 50eb (Costly) | Tools | Broken by BODY above 10 |
| Homing Tracer | 500eb (Expensive) | Electronics | 1 mile; extra tracers 50eb |
| Inflatable Bed & Sleep-bag | 20eb (Everyday) | Survival | Folds to 6"x6" |
| Kibble Pack | 10eb (Cheap) | Food | One meal |
| Linear Frame Beta | 5,000eb (Luxury) | Exoskeleton | BODY becomes 14 while plugged in; cannot exceed 14; HP and Death Save unchanged; needs 2 Interface Plugs installs |
| Linear Frame Sigma | 1,000eb (Very Expensive) | Exoskeleton | BODY becomes 12 while plugged in; HP and Death Save unchanged; needs 1 Interface Plugs install. (Text on p355 says "using two installations of Interface Plugs" in the bullet and "Requires 1 installations" in the next. CONFLICT inside one entry; UNCLEAR.) |
| Lock Picking Set | 20eb (Everyday) | Tools | Mechanical locks |
| Medscanner | 1,000eb (Very Expensive) | Medical | +2 First Aid and Paramedic; no stacking |
| Medtech Bag | 100eb (Premium) | Medical | Toolkit |
| Memory Chips | 10eb (Cheap) | Electronics | Data storage |
| MRE | 10eb (Cheap) | Food | Self-heating meal |
| Personal CarePak | 20eb (Everyday) | Survival | Hygiene |
| Pocket Amplifier | 50eb (Costly) | Music | 100m/yd, 6 hours, two instruments |
| Radar Detector | 500eb (Expensive) | Electronics | 100m/yds |
| Radio Communicator | 100eb (Premium) | Electronics | 1 mile |
| Radio Scanner/Music Player | 50eb (Costly) | Electronics | 1 mile of bands |
| Road Flare | 10eb (Cheap) | Light | 100m/yd, 1 hour |
| Rope (60m/yds) | 20eb (Everyday) | Tools | 800 lbs |
| Scrambler/Descrambler | 500eb (Expensive) | Electronics | Comms |
| Smart Glasses | 500eb (Expensive) | Cybereye gear | Two Cybereye option slots |
| Tech Bag | 500eb (Expensive) | Tools | Tools bag |
| Techscanner | 1,000eb (Very Expensive) | Tech | +2 Basic Tech, Cybertech, Land/Sea/Air Vehicle Tech, Electronics/Security Tech, Weaponstech; no stacking |
| Techtool | 100eb (Premium) | Tools | Multitool |
| Tent & Camping Equipment | 50eb (Costly) | Survival | One person |
| Vial of Biotoxin | 500eb (Expensive) | Weapon | On a Light Melee Weapon for 30 min; DV15 Resist Torture/Drugs; 3d6 direct to HP |
| Vial of Poison | 100eb (Premium) | Weapon | Same, DV13, 2d6 |
| Video Camera | 100eb (Premium) | Electronics | 12 hours |
| Virtuality Goggles | 100eb (Premium) | Electronics | Cyberspace overlay |

The "Category" column above is MY grouping to help sort; the book prints no category column for gear. Mark it ESTIMATE-by-judgment. Do not ship it as book data.
Description lengths: starter-table descriptions are one short sentence each (p101-102). Master-list descriptions in p353-356 run from one line (Computer, Carryall) to about 20 lines (Agent on p353, Cryopump and Cryotank on p354). I did not measure characters per item. ESTIMATE; to measure, split p353-356 on the "Name:" tokens.
Starter-table typo flagged: Cyberdeck is "Basic deck for netrunning. 7 slots. See pg. 353 ... 500eb (Expensive)" in p101, matching Standard Quality.

### 8c. Starting outfit per Role (p104, printed 103) and weapons/armor per Role (p099, printed 98)

Pure printed lists (10 Roles x outfit; 10 Roles x weapons/armor). Examples of structure: "Leisurewear: Footwear x2, Jacket x3, Mirrorshades, Bottoms x2, Top x2". Choice rows: "Shotgun or Assault Rifle", "Heavy Pistol or Very Heavy Pistol". Print only. Fixed rules: Streetrat/Edgerunner also get 500eb (p099). Complete Package gets 2,550eb and an extra 800eb fashion-only budget that is lost if unspent (p105, p106).
Count of Role columns: `grep -o 'Weapons/Armor' p099.txt | wc -l` printed 10 (10 Roles).

## 9. Fashion (item 6)

Printed twice: `p103.txt` (printed 102) and `p357.txt` (printed 356), identical.
Columns (9): Bottoms, Top, Jacket, Footwear, Jewelry, Mirrorshades, Glasses, Contact Lenses, Hats. Rows are 10 style families, each with 3 to 4 synonym keywords, one price per column. 10 x 9 = 90 prices.
Count: 10 style families. Command: `awk '/^ +(Bag Lady|Gang Colors|Generic Chic|Bohemian|Leisurewear|Nomad|Asia Pop|Urban Flash|Businesswear|High Fashion)/{c++} END{print c}' p357.txt` printed 10.

| Style (keywords) | Bottoms | Top | Jacket | Footwear | Jewelry | Mirrorshades | Glasses | Contacts | Hats |
|---|---|---|---|---|---|---|---|---|---|
| Bag Lady Chic (Homeless, Ragged, Vagrant) | 20 | 10 | 20 | 20 | 20 | 20 | 10 | 10 | 10 |
| Gang Colors (Dangerous, Violent, Rebellious) | 50 | 20 | 50 | 20 | 50 | 20 | 20 | 10 | 10 |
| Generic Chic (Standard, Colorful, Modular) | 50 | 20 | 50 | 20 | 50 | 20 | 20 | 10 | 10 |
| Bohemian (Folksy, Retro, Free Spirited) | 50 | 20 | 50 | 50 | 100 | 50 | 50 | 10 | 10 |
| Leisurewear (Comfort, Agility, Athleticism) | 100 | 20 | 100 | 50 | 100 | 50 | 50 | 20 | 50 |
| Nomad Leathers (Western, Rugged, Tribal) | 100 | 20 | 100 | 100 | 100 | 50 | 50 | 20 | 100 |
| Asia Pop (Bright, Costume-like, Youthful) | 100 | 20 | 100 | 100 | 100 | 100 | 100 | 100 | 100 |
| Urban Flash (Flashy, Technological, Streetwear) | 100 | 20 | 100 | 100 | 100 | 100 | 100 | 100 | 100 |
| Businesswear (Leadership, Presence, Authority) | 500 | 50 | 500 | 500 | 5,000 | 500 | 500 | 100 | 500 |
| High Fashion (Exclusive, Designer, Couture) | 1,000 | 500 | 1,000 | 5,000 | 50,000 | 1,000 | 1,000 | 1,000 | 5,000 |

Prices in eb. The category label (Cheap, Everyday, Costly, Premium, Expensive, Very Expensive, Luxury) prints under each price; every price maps to the scale above. Exception to check: High Fashion Jewelry prints "50,000eb (Luxury)"; the scale says Luxury is 5,000eb. UNCLEAR whether 50,000 is a misprint or an intentional above-scale price. Businesswear Jewelry "5,000eb (Luxury)" fits the scale.
Pdftotext quirk (p103 and p357): the leftmost column holds the style name plus three keyword lines stacked above/below the price row. Because the keywords are split on multiple lines, a naive row parser can mis-attach a keyword to a neighboring style (Gang Colors block sits between "Chic ... Vagrant" and "Generic Chic"). Mapping above is by price-row order and was cross-checked against the style order in `p329.txt` ("Bag Lady Chic Homeless Ragged Vagrant", "Gang Colors Dangerous Violent Rebellious", ...).
"Learn more about what you wear in 2045 on Page 327" (p103) points at Everyday Life, file `p329.txt`. Fashion culture prose is outside this domain.

Fashionware (cyberware, belongs to the cyberware inventory): Biomonitor, Chemskin, EMP Threading, Light Tattoo, Shift Tacts etc. on p359 and following. Out of scope here; noted because the Night Market Generator lists Fashionware.

## 10. Street Drugs (item 6)

Source: `p358.txt` (printed 357) and `p359.txt` (printed 358). Columns: Drug, Primary Effect Duration, Secondary Effect DV, Cost per Dose.
Count: 5 rows. Command: `sed -n 1,12p p358.txt | awk '/(Costly|Everyday|Cheap)\)$/{c++} END{print c}'` printed 5.

| Drug | Duration | Secondary DV | Cost per Dose |
|---|---|---|---|
| Black Lace | 24 Hours | 17 | 50eb (Costly) |
| Blue Glass | 4 Hours | 15 | 20eb (Everyday) |
| Boost | 24 Hours | 17 | 50eb (Costly) |
| Smash | 4 Hours | 15 | 10eb (Cheap) |
| Synthcoke | 4 Hours | 15 | 20eb (Everyday) |

Mechanics that touch sheet numbers (all conditional/temporary, so print):
- Black Lace: 2d6 Humanity Loss on dose; ignores Seriously Wounded effects; addicted = REF -2 unless on Primary.
- Boost: INT +2 (may exceed 8); addicted = INT -2.
- Smash: +2 Dance, Contortionist, Conversation, Human Perception, Persuasion, Acting; addicted = -2 to the same.
- Synthcoke: REF +1 (may exceed 8); addicted = REF -2 unless on Primary.
- Blue Glass: "flash out" lose an Action; addicted version changes.
Layout quirk: the p358 page prints the per-drug blocks in two columns (Black Lace and Blue Glass left, Boost and Smash right) with a margin note in the middle. The Smash secondary effect continues on p359. Read order matters.
Printed on p353: Airhypo "See pg. 357 to score some street drugs". No other drug lists in this domain. The Night Market Generator mentions "Street Drugs of 20eb or less" and "exactly 50eb" (p340).

## 11. Services and Entertainment (item 6)

Source: `p377.txt` (printed 376). Two columns: Service/Entertainment, Cost.
Count: 33 rows. Command: `awk '/eb \(/{c++} END{print c}' p377.txt` printed 33.
Groups: Bodysculpting x2, Braindance, Drinks x4, Found Cyberware Installation x3, Hospital Treatment x4, Hotel x2, Interactive Braindance, Live Concert / Sporting Event, Movie, Professional Services x3, Restaurant Meal x4, Taxi, Therapy x3, Trauma Team x2, Video Game.
Example rows: "Bodysculpting (Standard) 500eb (Expensive)", "Hospital Treatment (DV10) 50eb (Costly)", "Trauma Team (Silver), Per Month 500eb (Expensive)", "Professional Services, World Class, Per Job 5,000eb (Luxury)". Print as a table. Not a sheet total, except Trauma Team and Rent are monthly recurring (see Lifestyle).

## 12. Lifestyle and Housing (item 6)

Source: `p378.txt` (printed 377), `p379.txt` (printed 378), `p380.txt`, `p381.txt`.

Lifestyle (p378), column "Monthly Cost": 4 rows. Command: `grep -cE '^\s+(Kibble|Generic|Good|Fresh)' p378.txt` is approximate; read count 4.

| Lifestyle | Monthly cost | Entails (short) |
|---|---|---|
| Kibble | 100eb | Horrible food; once a month a movie or braindance |
| Generic Prepak | 300eb | Better food; each weekend a good bar or sit-down meal |
| Good Prepak | 600eb | Restaurant-grade food; excellent bars; once a month a Live Concert or Sporting Event |
| Fresh Food | 1500eb | Real food; hotel room once a month; executive bar; once a month a world class restaurant |

Note the table prints "1500eb" without a comma while others use commas ("1,500eb" style elsewhere). Same inconsistency in Real Estate: "1500eb", "2500eb", "7500eb" (p379).

Real Estate (p379), columns: Real Estate, Cost to Rent (per month), Cost to Buy. 11 rows. Command: `sed -n 1,16p p379.txt | awk '/^ +(Living|Cube|Cargo|Studio|Two-|Corporate|Upscale|Luxury)/{c++} END{print c}'` printed 11.

| Real Estate | Rent per month | Buy |
|---|---|---|
| Living on The Street | N/A | N/A |
| Living on The Street in a Vehicle | N/A | N/A |
| Cube Hotel | 500eb | N/A |
| Cargo Container | 1,000eb | 15,000eb |
| Studio Apartment | 1500eb | 25,000eb |
| Two-Bedroom Apartment | 2500eb | 35,000eb |
| Corporate Conapt | Given to you by a Corp | N/A |
| Upscale Conapt | 7500eb | 85,000eb |
| Luxury Penthouse | 15,000eb | "50,0000eb" (as printed) |
| Corporate Beaverville House | Given to you by a Corp | 200,000eb |
| Corporate Beaverville McMansion | Given to you by a Corp | 500,000eb |

UNCLEAR: Luxury Penthouse buy cost prints "50,0000eb", a malformed number (extra zero). It could be 500,000 or 50,000. Do not guess. The text offers no price check. A Luxury Penthouse rents at 15,000/month while a Studio rents 1500 and buys 25,000, so 50,000 would be below a Beaverville House (200,000) and below a 3x ratio. Flag for the lead.
Housing rules: 6 hours of sleep per day or -2 to everything per day missed; uncomfortable sleep = fatigued, -2 to everything the next day (p378). Crammed means occupants greater than 1 + number of bedrooms. Chargen defaults (p106): non-Exec starts in a Cargo Container on Kibble, first month free, owes "1100eb on the 1st of next month". Exec starts in a Corporate Conapt on Good Prepak, costing 600eb/month.
Check of the 1100: Cargo Container 1,000eb + Kibble 100eb = 1,100eb. Command: arithmetic only.
Protected parking spaces by home: Studio 1, Two-Bedroom 2, Corporate Conapt 2, Upscale Conapt 3, Luxury Penthouse 4 (p380-381). Beaverville badges: House 4, McMansion 6.

## 13. Vehicles (item 5)

NOT in the Night Market Appendix. The appendix index (p341) has no Vehicles entry. The vehicle tables are in the combat chapter: `p191.txt` (printed 190, Land and Sea) and `p192.txt` (printed 191, Air). Column key from `p190.txt` (printed 189): Description, SDP (Structural Damage Points, "A vehicle's Hit Points"), Seats, Speed (Combat) as MOVE, Speed (Narrative) in MPH/KPH, Cost.
Count: 14 vehicles (5 land, 4 sea, 5 air). Command: `cat p191.txt p192.txt | grep -cE '[0-9]+ MOVE'` printed 14.

| Vehicle | Type | SDP | Seats | Combat | Narrative | Cost |
|---|---|---|---|---|---|---|
| Roadbike | Land | 35 | 2 | 20 MOVE | 100 MPH / 161 KPH | 20,000eb (Super Luxury) |
| Superbike | Land | 35 | 2 | 60 MOVE | 300 MPH / 483 KPH | 100,000eb (Super Luxury) |
| Compact Groundcar | Land | 50 | 4 | 20 MOVE | 100 MPH / 161 KPH | 30,000eb (Super Luxury) |
| High Performance Groundcar | Land | 50 | 4 | 40 MOVE | 200 MPH / 322 KPH | 50,000eb (Super Luxury) |
| Super Groundcar | Land | 50 | 2 | 60 MOVE | 300 MPH / 483 KPH | 100,000eb (Super Luxury) |
| Jetski | Sea | 35 | 2 | 20 MOVE | 60 MPH / 97 KPH | 20,000eb (Super Luxury) |
| Speedboat | Sea | 50 | 4 | 20 MOVE | 60 MPH / 97 KPH | 30,000eb (Super Luxury) |
| Cabin Cruiser | Sea | 60 | 2 per Room | 10 MOVE | 15 MPH / 24 KPH | 30,000eb (Super Luxury) per room below deck (minimum two rooms) |
| Yacht | Sea | 100 | 4 per Room | 10 MOVE | 15 MPH / 24 KPH | 50,000eb (Super Luxury) per room below or above deck (minimum four rooms) |
| Gyrocopter | Air | 35 | 2 | 20 MOVE | 100 MPH / 161 KPH | 20,000eb (Super Luxury) |
| Helicopter | Air | 60 | 4 | 40 MOVE | 200 MPH / 322 KPH | 40,000eb (Super Luxury) |
| AV-4 Multipurpose Aerodyne | Air | 100 | 6 | 40 MOVE | 200 MPH / 322 KPH | 50,000eb (Super Luxury) |
| AV-9 Super Aerodyne | Air | 60 | 2 | 60 MOVE | 300 MPH / 483 KPH | 100,000eb (Super Luxury) |
| Aerozep | Air | 100 | 2 per Room | 20 MOVE | 100 MPH / 161 KPH | 30,000eb (Super Luxury) per room below deck (minimum two rooms) |

Pdftotext quirk: the Cabin Cruiser, Yacht and Aerozep rows split cost over 3 lines. The cost per room reading is by joining those lines. The Aerozep speed "100 MPH/161 KPH" matches the row layout; read as printed.
Tied to the character sheet only if the Nomad Moto/vehicle role ability is in another inventory. Vehicle upgrades (Heavy Chassis +20 SDP, Bulletproof Glass, Security) are Nomad Role Ability text in `p165.txt`, not here.
Longer vehicle lore with different cost lines is in `p324.txt`-`p326.txt` (Everyday Life): e.g. "Cost: 20,000eb (Super Luxury)", "Cost: 100,000eb (Super Luxury)", "Price: 50,000eb (Super Luxury)". Not re-tabulated. UNCLEAR whether those are a second price list; prose only.

## 14. Night Market Generator tables (p339, p340)

The generator is a pure random-draw table. Printed pages: p339 (printed 338) holds the type table and rows 0-5 through 21-25; p340 (printed 339) holds rows 26-30 through 96-100 without a repeated header.
Table 1 (roll 1d6 twice): 6 rows: Food and Drugs, Personal Electronics, Weapons and Armor, Cyberware, Clothing and Fashionware, Survival Gear.
Table 2 (d100): 20 rows x 6 columns = 120 cells. Command: `cat p339.txt p340.txt | grep -oE '^ *[0-9]+-[0-9]+' | tr -d ' ' | wc -l` printed 20 (the roll ranges 0-5, 6-10, ..., 96-100).
Midnight Market (p341): "1d10 + 5 rare and sought-after items", "at least a Rank 9 Fixer".
Not needed for a paper sheet. Mark: SKIP (GM tooling). The weapon/armor column cells reuse the already-tabulated items (e.g. "Armor of 100eb or less", "Weapon Attachments of 500eb or higher").
Typo, p340: "MRE 10eb (Cheap)" fine; "Live Chicken 50eb (Costly)", "Live Pigs 100eb (Premium)". Food prices (Prepak etc.) sit in table 2, not a separate food table.

## 15. Rules that change sheet numbers (item 8)

| Rule | Printed | Source |
|---|---|---|
| Armor SP is per location (head, body); only the highest SP in a location counts; no stacking | Yes | p097, p185, p351 |
| Bodyweight Suit occupies BOTH head and body at SP 11 in one purchase; one per character; +1 cyberdeck hardware-only slot | Yes | p351 |
| Armor penalty to REF, DEX, MOVE: the single worst penalty among worn armor, applied once; MOVE floor is 0 | Yes | p097, p185 |
| Penalty values: None / -2 / -4 (see armor table) | Yes | p098, p351 |
| Ablation: each damaging hit lowers the SP of ALL worn armor in that location by 1 until repaired | Yes | p097, p187 |
| Shield: 10 HP, takes up one arm, Action to equip/drop | Yes | p098, p184, p185 |
| Head Aimed Shot: damage through head armor x2 | Yes | p171 |
| Weapon damage dice (see melee/ranged tables) | Yes | p093, p095 |
| Excellent Quality: +1 to Attack Check; Poor: malfunction on a 1 | Yes | p343 |
| Smartgun Link: +1 to Ranged Attack Check; Sniping Scope: +1 at 51m+; stacks note: not with TeleOptics | Yes | p345 |
| Melee uses DEX; ranged uses REF | Yes | p092, p094 |
| Linear Frame: sets BODY to 12 or 14 while plugged in; HP and Death Save do not change | Yes | p355 |
| Agent, Medscanner, Techscanner: +2 to named Skills, no stacking | Yes | p353, p355, p356 |
| Drugs: INT +2, REF +1, addiction REF -2 / INT -2, Skill +/-2 | Yes | p358, p359 |
| Fatigue: -2 to everything; 6 hours sleep/day | Yes | p378 |
| Encumbrance / carry weight | NOT PRESENT | `grep -il encumbr p*.txt` printed no files |
| Starting money: Streetrat/Edgerunner 500eb; Complete Package 2,550eb plus 800eb fashion-only | Yes | p099, p105, p106 |
| Weapon attachment slots: 3 per Non-Exotic Ranged weapon | Yes | p097, p343 |
| Magazine capacity by attachment: Clip Chart | Yes | p345 |

Shield "always takes up one arm": Complete rule on whether a Two-Handed weapon excludes a shield is not printed as a sentence in the armor table. UNCLEAR.

The official character sheet (`p121.txt`) fields: Weapons and Armor box with columns Weapon, DMG, Ammo, ROF, Notes; Armor Head, Body, Shield with "sp" and "penalty", footnote "penalty applies to REF, DEX & MOVE"; Gear/Outfit box with Ammunition and Cash lines; Rent and Lifestyle; Fashion slots. It confirms what the sheet itself shows.

## 16. Counts summary (item 9)

| Count | Value | Command (run from the pages directory) | Run? |
|---|---|---|---|
| Melee weapon types | 4 | `grep -cE '^\s+(Light Melee\|Medium Melee\|Heavy Melee\|Very Heavy)\s' p341.txt` | yes, 8 for p093+p341 combined |
| Ranged weapon types | 11 | `grep -c 'Alt. Fire Modes' p342.txt` | yes |
| Ranged weapon (p095 copy) | 11 | `grep -c 'Alt. Fire Modes' p095.txt` | yes |
| Ammunition types | 13 | `grep -cE '^\s+[A-Z][A-Za-z-]+ Ammunition\s+[0-9]' p346.txt` | yes |
| Ammunition detail entries | 13 | `cat p346.txt p347.txt p348.txt \| awk '/^ *▶ .*Ammunition/{c++} END{print c}'` | yes |
| Weapon attachments | 8 | `cat p344.txt p345.txt \| awk '/^ *▶ (Bayonet\|Drum\|Extended\|Grenade\|Infrared\|Shotgun\|Smartgun\|Sniping)/{c++} END{print c}'` | yes |
| Clip Chart rows | 10 | `sed -n '/Clip Chart/,/^ *Ammunition/p' p345.txt \| grep -cE '^ +(Medium\|Heavy\|Very\|SMG\|Shotgun\|Assault\|Sniper\|Grenade\|Rocket)'` | yes |
| Exotic weapons | 14 | `awk '/[0-9]eb \(/{c++} END{print c}' p096.txt` | yes |
| Exotic detail headings (14 + Smoke/Teargas) | 16 | `cat p348.txt p349.txt p350.txt \| awk '/^ *▶ /{c++} END{print c}'` | yes |
| Armors (SP-bearing) | 8 | `sed -n 1,18p p351.txt \| awk '/[0-9]eb ?\(/{c++} END{print c}'` | yes |
| Armor table rows incl. Bulletproof Shield | 9 | by eye | no, ESTIMATE |
| Gear, starter table | 47 | `awk '{n=gsub(/[0-9]eb ?\(/,"&"); c+=n} END{print c}' p101.txt p102.txt` | yes |
| Gear, Master Gear List | 53 | `awk '{n=gsub(/[0-9]eb ?\(/,"&"); c+=n} END{print c}' p352.txt` | yes |
| Vehicles | 14 | `cat p191.txt p192.txt \| grep -cE '[0-9]+ MOVE'` | yes |
| Street drugs | 5 | `sed -n 1,12p p358.txt \| awk '/(Costly\|Everyday\|Cheap)\)$/{c++} END{print c}'` | yes |
| Fashion style families | 10 | `awk '/^ +(Bag Lady\|Gang Colors\|Generic Chic\|Bohemian\|Leisurewear\|Nomad\|Asia Pop\|Urban Flash\|Businesswear\|High Fashion)/{c++} END{print c}' p357.txt` | yes |
| Fashion price cells | 90 | 10 x 9 | arithmetic |
| Services and Entertainment | 33 | `awk '/eb \(/{c++} END{print c}' p377.txt` | yes |
| Lifestyle tiers | 4 | `grep -cE '^\s+(Kibble\|Generic\|Good\|Fresh)' p378.txt` | yes |
| Real Estate rows | 11 | `sed -n 1,16p p379.txt \| awk '/^ +(Living\|Cube\|Cargo\|Studio\|Two-\|Corporate\|Upscale\|Luxury)/{c++} END{print c}'` | yes |
| Night Market d100 rows | 20 | `cat p339.txt p340.txt \| grep -oE '^ *[0-9]+-[0-9]+' \| wc -l` | yes (via tr/awk; first run printed 20 with awk) |
| Price categories | 8 | printed sentence "eight Price Categories", p386 | by reading |
| Quality brand-example rows | 12 | by eye | no, ESTIMATE |

Rule 7 caveat for these counts: awk/grep counts check that one pattern matched N lines. They do not prove the row set is complete. Cross-checks that agree: ranged 11 = 11 (two printings); armor values identical in three printings; starter gear 47 + 6 = master 53; exotic 14 = summary 14 = detail headings 14 (16 minus 2 ammo headings).
Counts NOT covered: the per-Role starting kits (p099, p104) and the Night Market Generator cells (120) were not tabulated item by item.

## 17. Text-extraction quality (item 10)

| File | Printed | Problem | Risk |
|---|---|---|---|
| p095 | 94 | Ranged table: each row is split across lines; "Alt. Fire Modes" sits on a second line; header row spreads over four lines; margin note "To learn more about Alt. Fire Modes flip to Page 173" intrudes | Medium. Same table in p342 is cleaner; use both |
| p096 | 95 | Exotic table: description wraps; margin note intrudes at Cowboy/Railgun; Air Pistol says Very Heavy here vs Medium on p348 | Medium |
| p098 | 97 | Armor table rows split description over 2-3 lines with SP in the middle line | Low. Use p351 |
| p099 | 98 | Role starting kit columns: 5 columns x 2 blocks interleaved; wrapped items; "Basic VH Pistol / Ammunition x50" splits an item across lines. Pairing items to Roles requires column position | HIGH |
| p103 | 102 | Fashion: style keywords stacked on 3-4 lines per row; prices tied by column position | HIGH |
| p104 | 103 | Starting outfit per Role: 5 columns x 2 blocks; wrapped items such as "Electric Guitar or Bug Detector" run together on one line, which looks like one item. "Electric Guitar or" and "Bug Detector" are two items in different columns | HIGH |
| p339, p340 | 338, 339 | d100 table spans both pages; header only on p339; each cell is two or three lines (name, price, price category); category names wrap (e.g. "Auto Level Dampening Ear Protectors") | HIGH |
| p352 | 351 | Master Gear List: two item/cost column pairs side by side; names wrap to two lines ("Anti-Smog Breathing Mask", "Linear Frame ß (Beta)") with Greek symbols ß and ∑ | Medium |
| p353-p356 | 352-355 | Two-column prose; sentences from the left column interleave with the right on some lines; margin notes inside text | Medium. Entries start at "Name:" |
| p357 | 356 | Same as p103 fashion | HIGH |
| p358 | 357 | Two-column drug blocks; margin note; Smash secondary effect continues on p359 | Medium |
| p191, p192 | 190, 191 | Cabin Cruiser, Yacht, Aerozep costs split across three lines | Medium |
| p121 | sheet | Sample character sheet text is spatial; 32 KB; words scatter across columns | HIGH. Do not parse; use the image or PDF |
| p379 | 378 | Real Estate: "50,0000eb" malformed | Data defect |
| p102 | 101 | "10b (Cheap)" for Road Flare | Typo |
| p351 | 350 | "100eb(Premium)" missing space | Typo (breaks naive regexes) |

Good extraction: p093, p341 (melee), p342 (ranged), p344-p348 (attachments, ammo), p351 (armor), p358 (drugs header table), p377 (services), p386 (price scale).
Not tested against the PDF image. Everything above is judged from the text files only.

## 18. Print vs compute (item 11)

Standing rule: only values that feed a sheet total get computed. Text rules get printed. No simulation engines.

COMPUTE (these change numbers the sheet already prints):
1. Armor penalty on REF, DEX, MOVE: take the worst penalty among worn armor pieces (head and body), apply once, MOVE floor 0. It lowers REF, DEX, MOVE and therefore every skill base that uses REF or DEX (BASE = LVL + STAT). Inputs: SP table row per worn piece. Rule at p097/p185.
2. Armor SP per location: take the maximum SP across worn armor for head and for body. No stacking. Bodyweight Suit supplies SP 11 to both. This feeds the SP boxes on the sheet.
3. Skill bases that depend on REF or DEX recompute after (1). Melee Weapon skill is DEX based; Handgun, Shoulder Arms, Archery, Heavy Weapons, Autofire are REF based.

PRINT (static copy from the book, no arithmetic):
- Weapon rows: damage dice, ROF, magazine, hands, concealable, cost, skill name, alt fire modes. The sheet has Weapon, DMG, Ammo, ROF, Notes columns (p121).
- Magazine size with an Extended or Drum magazine: look up the Clip Chart number, do not recompute. This is a table lookup.
- Attachment text, ammo-type effects, exotic weapon rules, Excellent/Poor quality notes, "+1 to Attack Check" from Excellent Quality or Smartgun Link, Sniping Scope +1.
- Gear lists and costs, the starter kits, the Agent / Medscanner / Techscanner +2 text, Linear Frame text.
- Armor ablation: a play-time marker. Print "SP now" box blank. Do not simulate.
- Drug effects.
- Fashion, services, housing, real estate, lifestyle costs.
- Vehicles (SDP, seats, speeds, costs) if vehicle block is wanted on the sheet.
- Shield HP 10 and "takes one arm".

UNCLEAR, needs a lead decision:
- Cash / budget math: starting eb (500 / 2,550 + 800 fashion) minus purchases would be a computed "Cash" value. The standing rule says only sheet-total-feeding values get computed. Cash is a sheet field (p121) but it is a hand-written box. Recommend PRINT the starting budget as text and leave Cash blank.
- Monthly cost (Rent + Lifestyle): both printed on the sheet as two fields. Sum is not a field. Recommend PRINT each; do not sum.
- Attack bonus (STAT + Skill + Quality bonus): the sheet prints skill BASE. Recommend PRINT only BASE, not an attack total.
- Whether to include Excellent/Poor variants as separate weapon rows. The book lists them as a cost table plus brand names only (p343).
- Armor weight / encumbrance: not in this book.

## 19. Quality notes and defects found (cross-source conflicts)

1. Air Pistol: p096 summary says "Very Heavy Pistol that fires paint (and acid!) balls"; p348 detail says "An Exotic Medium Pistol". Conflict. UNCLEAR which is printed truth.
2. Linear Frame Sigma: p355 first bullet "two installations of Interface Plugs", second bullet "Requires 1 installations". Conflict inside one entry.
3. Luxury Penthouse buy cost "50,0000eb" (p379).
4. High Fashion Jewelry "50,000eb (Luxury)" against the scale's 5,000eb for Luxury (p103, p357).
5. Road Flare "10b (Cheap)" (p102).
6. In-text page refs are printed numbers (file = printed + 1). Spot checks agree: "Buying and Selling on Page 385" is p386; "Night Market section on pg. 340" is the Appendix in p341. One stale-looking ref: p096 says Exotic details are "on page 347"; that is p348, so it agrees.
7. Appendix index (p341) lists "Home Defenses ... Page 372" and "Services & Entertainment ... Page 376": matches p377 (printed 376).
8. Exotic weapons use cost "5,000eb (Luxury)" for six items, "10,000eb (Super Luxury)" for one.
9. p096 GM note mentions Luxury items at chargen. Complete Package budget is 2,550eb, so only Premium-and-below plus a few Expensive items fit, unless the player buys only that item.
