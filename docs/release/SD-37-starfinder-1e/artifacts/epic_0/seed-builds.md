---
canonical: true
bundle_id: SD-37
card: E0.4
artifact_type: seed-builds
fetched: 2026-10-02
review_status: pending independent Opus review (E0.4 reviewer step)
---

# SD-37 Starfinder seed builds (E0.4 deliverable 1)

The four Starfinder (SF1e) merge-check seeds, fully specified. The skeleton (class, level, race,
key ability) is fixed by `content-unit-inventory.md §4`. Everything else here is E0.4's choice,
made under the constraints in `decisions.md §8`, `§9` and `§18`. The derived numbers (BAB, saves,
HP, Stamina, Resolve, EAC, KAC, skill totals, spells per day) are in `seed-hand-values.md`.

**Source rule.** Every rules value below was transcribed on 2026-10-02 from the Starfinder Reference
Document as hosted by Archives of Nethys (`https://www.aonsrd.com/`), from the page cited next to
it. No value was taken from the PCGen `.lst` files or from recall. The `.lst` files were used for
**names only**: every pick below resolves to a record name in the pinned oracle
(`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`; the check command is in
§6). The URL, byte count and sha256 of every page fetched are in `E0.4-srd-fetch-log.txt`.

**Baseline: the Core Rulebook, not Starfinder Enhanced.** AoN's Technomancer and Envoy pages mix
in optional *Starfinder Enhanced* features (marked "E", for example "technomantic talentE",
"inspiring comboE"). The pinned oracle encodes the Core Rulebook, so these seeds take only Core
Rulebook features and options. None of the Enhanced features changes a value in
`seed-hand-values.md` (BAB, saves, HP, SP, spells per day/known are the same columns), but no seed
selects one.

## 0. Rules used by every seed

| Rule | Value used | Source (URL, section) |
|---|---|---|
| Point buy | Start at 10 in each ability, add race, add theme, then spend exactly 10 points; no score above 18 at creation | https://www.aonsrd.com/Rules.aspx?ID=42 (Buying Ability Scores, steps 1–4) |
| Ability modifier | Table 2-1: 8–9 → −1, 10–11 → +0, 12–13 → +1, 14–15 → +2, 16–17 → +3, 18–19 → +4 | https://www.aonsrd.com/Rules.aspx?ID=42 (Table 2-1) |
| 5th-level ability increase | Choose four scores; +1 if the score is 17 or higher, else +2; the increase applies retroactively to skill ranks, SP and RP | https://www.aonsrd.com/Rules.aspx?ID=57 (Step 1: Apply any Ability Increases) |
| Feats | One feat at 1st level and at every odd level (1st, 3rd, 5th), plus class bonus feats; humans get one extra feat at 1st | https://www.aonsrd.com/Rules.aspx?ID=59 ; https://www.aonsrd.com/Rules.aspx?ID=56 (Table 2-4) ; https://www.aonsrd.com/Races.aspx?ItemName=Human (Bonus Feat) |
| Skill ranks | Class value + Int modifier per level (minimum 1); ranks in one skill ≤ character level | https://www.aonsrd.com/Rules.aspx?ID=77 (Acquiring Skills) |
| Starting wealth above 1st level | Table 11-5 Character Wealth per Level: 3rd = 4,000 credits, 5th = 9,000 credits | https://www.aonsrd.com/Rules.aspx?ID=230 (Table 11-5) |
| Bulk | Light (L) items: every 10 count as 1 bulk, fractions don't count; negligible (—) items don't count; encumbered above ½ Strength score | https://www.aonsrd.com/Equipment.aspx (Carrying Capacity: Item Bulk, Bulk Limits) |

**Safe default taken (SD-g scope, logged in `progress.md`).** The Core Rulebook gives 1,000 credits
for a 1st-level character (https://www.aonsrd.com/Rules.aspx?ID=39) and no separate
"starting gear above 1st level" rule. These seeds use the GM table of wealth per level
(Table 11-5) as each seed's total gear budget. Alternative not taken: building every seed with
1,000 credits of gear (unrealistic gear for a 3rd- or 5th-level sheet, and it would not exercise
the Soldier's heavy armour).

## 1. SF-Soldier-3 — Human Soldier 3

| Field | Value | Source (URL, section) |
|---|---|---|
| Race | Human (Medium humanoid); racial HP 4; +2 to any one ability (→ Str); bonus feat at 1st; Skilled: +1 skill rank per level | https://www.aonsrd.com/Races.aspx?ItemName=Human |
| Theme | Mercenary: +1 Str; theme knowledge: Athletics is a class skill, or +1 to Athletics checks if it already is (it is, for soldiers) | https://www.aonsrd.com/Themes.aspx?ItemName=Mercenary (Theme Knowledge, 1st Level) |
| Class / level | Soldier 3; HP 7, SP 7 per level; skill ranks 4 + Int per level | https://www.aonsrd.com/Classes.aspx?ItemName=Soldier |
| Key ability | Strength (soldier chooses Str or Dex; Str chosen) | https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (Key Ability Score - Str\|Dex) |
| Class features taken | 1st: primary fighting style **Sharpshoot**, primary style technique **Sniper's Aim**; 2nd: combat feat; 3rd: gear boost **Laser Accuracy**, weapon specialization | https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (class table) ; https://www.aonsrd.com/FightingStyles.aspx?ItemName=Sharpshoot ; https://www.aonsrd.com/GearBoosts.aspx?ItemName=All (Laser Accuracy) |

**Ability scores (point buy).**

| | Str | Dex | Con | Int | Wis | Cha |
|---|---|---|---|---|---|---|
| Base | 10 | 10 | 10 | 10 | 10 | 10 |
| Race (human, +2 any → Str) | +2 | | | | | |
| Theme (mercenary) | +1 | | | | | |
| Points spent (total 10) | +3 | +4 | +2 | +1 | 0 | 0 |
| **Final** | **16** | **14** | **12** | **11** | **10** | **10** |
| Modifier | +3 | +2 | **+1** | +0 | +0 | +0 |

Points: 3 + 4 + 2 + 1 = 10. Con modifier +1 (≠ 0, `decisions.md §8`).

**Skill ranks** (4 + Int 0 + human Skilled 1 = 5 per level × 3 = 15; max 3 per skill):
Athletics 3, Intimidate 3, Medicine 3, Piloting 3, Survival 3 (all soldier class skills). Total 15.

**Feats** (character feats at 1st and 3rd, human bonus feat at 1st, soldier combat feat at 2nd):

| Feat | When | Prerequisite met | Source |
|---|---|---|---|
| Weapon Focus (longarms) | 1st (character feat) | proficiency with longarms (soldier) | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Weapon%20Focus |
| Quick Draw | 1st (human bonus feat) | BAB +1 (soldier 1st: +1) | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Quick%20Draw |
| Deadly Aim | 2nd (soldier combat feat) | BAB +1 | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Deadly%20Aim |
| Coordinated Shot | 3rd (character feat) | BAB +1 | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Coordinated%20Shot |
| Weapon Specialization (each soldier weapon type) | 3rd (class feature) | — | https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (Weapon Specialization) |

None of these feats changes BAB, a save, HP, SP, RP, EAC, KAC or a skill total (attack and
damage only).

**Armour and weapons** (all Core Rulebook):

| Item | Level | Price | Stats used | Bulk | Source |
|---|---|---|---|---|---|
| Defiance Series, Squad (heavy armour, worn) | 3 | 1,220 | EAC +5, KAC +8, **max Dex +1**, ACP −4, speed −10 ft. | 3 | https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Squad&Family=Defiance%20Series |
| Laser Rifle, Azimuth (longarm) | 1 | 425 | 1d8 F, range 120 ft., 20 charges | 1 | https://www.aonsrd.com/WeaponDisplay.aspx?ItemName=Azimuth&Family=Laser%20Rifle |
| Baton, Tactical (basic melee) | 1 | 90 | 1d4 B | L | https://www.aonsrd.com/WeaponDisplay.aspx?ItemName=Tactical&Family=Baton |

**Max-Dex constraint (`decisions.md §9`): the cap binds.** Dex modifier +2 > armour max Dex +1, so
EAC/KAC add +1, not +2.

**Other gear:**

| Item | Qty | Price each | Bulk each | Source |
|---|---|---|---|---|
| Battery, Standard (20 charges) | 2 | 60 | — | https://www.aonsrd.com/WeaponDisplay.aspx?ItemName=Standard&Family=Battery |
| Serum of Healing, Mk 1 | 2 | 50 | L | https://www.aonsrd.com/MagicItems.aspx?ItemName=Mk%201&Family=Serum%20of%20Healing |

**Credits:** 4,000 − (1,220 + 425 + 90 + 2 × 60 + 2 × 50) = 4,000 − 1,955 = **2,045 remaining**.

**Bulk:** 3 (armour) + 1 (rifle) + 3 L items (baton, 2 serums → 0) + batteries (—) = **4**.
Limit ½ × Str 16 = 8, so the seed is not encumbered.

## 2. SF-Mystic-5 — Lashunta (damaya) Mystic 5

| Field | Value | Source (URL, section) |
|---|---|---|
| Race | Lashunta, **damaya** subrace: +2 Cha, +2 Int, −2 Con; racial HP 4; Lashunta Magic (at will: daze, psychokinetic hand; 1/day: detect thoughts); Limited Telepathy; Student: +2 racial bonus to two chosen skills (→ Diplomacy, Medicine) | https://www.aonsrd.com/Races.aspx?ItemName=Lashunta |
| Theme | Priest: +1 Wis; Mysticism is a class skill, or +1 to Mysticism checks if it already is (it is, for mystics) | https://www.aonsrd.com/Themes.aspx?ItemName=Priest (Theme Knowledge, 1st Level) |
| Class / level | Mystic 5; HP 6, SP 6 per level; skill ranks 6 + Int per level | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic |
| Key ability | Wisdom | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Key Ability Score - Wis) |
| Connection | **Empath** (associated skills Perception and Sense Motive; connection powers Empathy at 1st, Greater Mindlink at 3rd) | https://www.aonsrd.com/MysticConnections.aspx?ItemName=Empath |
| Class features | 1st: connection, connection power, connection spell, healing touch; 2nd: channel skill +1, mindlink; 3rd: connection power, weapon specialization; 4th: connection spell; 5th: channel skill +2 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (class table) |

**Ability scores (point buy, then the 5th-level increase).**

| | Str | Dex | Con | Int | Wis | Cha |
|---|---|---|---|---|---|---|
| Base | 10 | 10 | 10 | 10 | 10 | 10 |
| Race (damaya) | | | −2 | +2 | | +2 |
| Theme (priest) | | | | | +1 | |
| Points spent (total 10) | 0 | +2 | 0 | 0 | +7 | +1 |
| At 1st level | 10 | 12 | 8 | 12 | 18 | 13 |
| 5th-level increase (four scores) | | +2 | | +2 | +1 (was ≥ 17) | +2 |
| **Final (5th level)** | **10** | **14** | **8** | **14** | **19** | **15** |
| Modifier | +0 | +2 | **−1** | +2 | +4 | +2 |

Points: 2 + 7 + 1 = 10. Con is **not** raised at 5th level, so the Con modifier stays −1
(≠ 0, `decisions.md §8`). Stamina per level = 6 + (−1) = 5, which is not below 0
(https://www.aonsrd.com/Rules.aspx?ID=49).

**Skill ranks** (6 + Int 2 = 8 per level × 5 = 40, Int increase retroactive; max 5 per skill):
Bluff 5, Culture 5, Diplomacy 5, Life Science 5, Medicine 5, Mysticism 5, Perception 5,
Sense Motive 5 (all mystic class skills). Total 40.

**Feats** (character feats at 1st, 3rd, 5th):

| Feat | When | Prerequisite met | Source |
|---|---|---|---|
| Spell Penetration | 1st | none | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Spell%20Penetration |
| Spell Focus | 3rd | casts spells, character level 3rd | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Spell%20Focus |
| Quick Draw | 5th | BAB +1 (mystic 5th: +3) | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Quick%20Draw |

None of these feats changes a value in `seed-hand-values.md`.

**Spells known** (mystic table at 5th: 0-level 6, 1st 4, 2nd 3; plus one connection spell per
castable spell level, https://www.aonsrd.com/Classes.aspx?ItemName=Mystic, Spells Known and
Connection Spell). Every spell page names *Starfinder Core Rulebook* and lists the spell for
mystics at that level (URL pattern `https://www.aonsrd.com/SpellDisplay.aspx?ItemName=<name>`).

| Spell level | Spells (from the table) | Connection spell |
|---|---|---|
| 0 (6) | detect affliction, detect magic, ghost sound, grave words, stabilize, telepathic message | — |
| 1st (4 + 1) | charm person, command, mystic cure (1st), share language | detect thoughts (Empath 1st) |
| 2nd (3 + 1) | hold person, remove condition, status | zone of truth (Empath 2nd) |

**Armour and weapons:**

| Item | Level | Price | Stats used | Bulk | Source |
|---|---|---|---|---|---|
| Lashunta Tempweave, Basic (light armour, worn) | 4 | 1,950 | EAC +4, KAC +4, max Dex +5, ACP — | L | https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Basic&Family=Lashunta%20Tempweave |
| Laser Pistol, Azimuth (small arm) | 1 | 350 | 1d4 F, range 80 ft., 20 charges | L | https://www.aonsrd.com/WeaponDisplay.aspx?ItemName=Azimuth&Family=Laser%20Pistol |
| Baton, Tactical (basic melee) | 1 | 90 | 1d4 B | L | https://www.aonsrd.com/WeaponDisplay.aspx?ItemName=Tactical&Family=Baton |

**Other gear:**

| Item | Qty | Price each | Bulk each | Source |
|---|---|---|---|---|
| Battery, Standard | 1 | 60 | — | https://www.aonsrd.com/WeaponDisplay.aspx?ItemName=Standard&Family=Battery |
| Medkit, Basic | 1 | 100 | 1 | https://www.aonsrd.com/TechItems.aspx?ItemName=Basic&Family=Medkit |
| Serum of Healing, Mk 1 | 2 | 50 | L | https://www.aonsrd.com/MagicItems.aspx?ItemName=Mk%201&Family=Serum%20of%20Healing |

**Credits:** 9,000 − (1,950 + 350 + 90 + 60 + 100 + 2 × 50) = 9,000 − 2,650 = **6,350 remaining**.

**Bulk:** 1 (medkit) + 5 L items (armour, pistol, baton, 2 serums → 0) = **1**. Limit ½ × Str 10 = 5.

## 3. SF-Technomancer-5 — Android Technomancer 5

| Field | Value | Source (URL, section) |
|---|---|---|
| Race | Android: +2 Dex, +2 Int, −2 Cha; racial HP 4; Constructed (+2 racial vs disease, mind-affecting, poison, sleep — situational, printed); Exceptional Vision; **Flat Affect: −2 to Sense Motive checks**; Upgrade Slot | https://www.aonsrd.com/Races.aspx?ItemName=Android |
| Theme | Scholar: +1 Int; chosen skill **Physical Science** (specialty: physics); it is already a technomancer class skill → +1 to Physical Science checks | https://www.aonsrd.com/Themes.aspx?ItemName=Scholar (Theme Knowledge, 1st Level) |
| Class / level | Technomancer 5; HP 5, SP 5 per level; skill ranks 4 + Int per level | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer |
| Key ability | Intelligence | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Key Ability Score - Int) |
| Class features (Core Rulebook) | 1st: spell cache; 2nd: magic hack **Harmful Spells**; 3rd: Spell Focus (bonus feat), techlore +1 (insight bonus to Computers and Mysticism), weapon specialization; 5th: magic hack **Selective Targeting** | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Spell Cache, Magic Hack, Spell Focus, Techlore) ; https://www.aonsrd.com/MagicHacks.aspx?ItemName=All |

**Ability scores (point buy, then the 5th-level increase).**

| | Str | Dex | Con | Int | Wis | Cha |
|---|---|---|---|---|---|---|
| Base | 10 | 10 | 10 | 10 | 10 | 10 |
| Race (android) | | +2 | | +2 | | −2 |
| Theme (scholar) | | | | +1 | | |
| Points spent (total 10) | 0 | +2 | +2 | +5 | +1 | 0 |
| At 1st level | 10 | 14 | 12 | 18 | 11 | 8 |
| 5th-level increase (four scores) | | +2 | +2 | +1 (was ≥ 17) | +2 | |
| **Final (5th level)** | **10** | **16** | **14** | **19** | **13** | **8** |
| Modifier | +0 | +3 | **+2** | +4 | +1 | −1 |

Points: 2 + 2 + 5 + 1 = 10. Con modifier +2 (≠ 0, `decisions.md §8`); it was +1 at levels 1–4 and
the increase is retroactive for Stamina (https://www.aonsrd.com/Rules.aspx?ID=57).

**Skill ranks** (4 + Int 4 = 8 per level × 5 = 40; max 5 per skill): Computers 5, Engineering 5,
Life Science 5, Mysticism 5, Physical Science 5, Piloting 5, Sleight of Hand 5 (technomancer
class skills) and Perception 5 (not a class skill). Total 40.

**Feats** (character feats at 1st, 3rd, 5th; class bonus feat at 3rd):

| Feat | When | Prerequisite met | Source |
|---|---|---|---|
| Spell Penetration | 1st | none | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Spell%20Penetration |
| Spell Focus | 3rd (technomancer bonus feat) | granted by the class | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Spell Focus - 3rd Level) |
| Mobility | 3rd | Dex 13 (Dex 14) | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Mobility |
| Quick Draw | 5th | BAB +1 (technomancer 5th: +3) | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Quick%20Draw |

Mobility's +4 applies only against attacks of opportunity from leaving a threatened square, so it is
printed, not added to EAC/KAC.

**Spells known** (technomancer table at 5th: 0-level 6, 1st 4, 2nd 3; no connection spells;
https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer, Spells Known):

| Spell level | Spells |
|---|---|
| 0 (6) | dancing lights, detect magic, energy ray, mending, token spell, transfer charge |
| 1st (4) | detect tech, magic missile, overheat, supercharge weapon |
| 2nd (3) | invisibility, knock, mirror image |

**Armour and weapons:**

| Item | Level | Price | Stats used | Bulk | Source |
|---|---|---|---|---|---|
| D-Suit I (light armour, worn) | 5 | 2,980 | EAC +5, KAC +6, max Dex +5, ACP — | L | https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=I&Family=D-Suit |
| Laser Pistol, Azimuth (small arm) | 1 | 350 | 1d4 F, range 80 ft., 20 charges | L | https://www.aonsrd.com/WeaponDisplay.aspx?ItemName=Azimuth&Family=Laser%20Pistol |

**Other gear:**

| Item | Qty | Price each | Bulk each | Source |
|---|---|---|---|---|
| Battery, Standard | 2 | 60 | — | https://www.aonsrd.com/WeaponDisplay.aspx?ItemName=Standard&Family=Battery |
| Medkit, Basic | 1 | 100 | 1 | https://www.aonsrd.com/TechItems.aspx?ItemName=Basic&Family=Medkit |
| Serum of Healing, Mk 1 | 2 | 50 | L | https://www.aonsrd.com/MagicItems.aspx?ItemName=Mk%201&Family=Serum%20of%20Healing |

**Credits:** 9,000 − (2,980 + 350 + 2 × 60 + 100 + 2 × 50) = 9,000 − 3,650 = **5,350 remaining**.

**Bulk:** 1 (medkit) + 4 L items (armour, pistol, 2 serums → 0) = **1**. Limit ½ × Str 10 = 5.

## 4. SF-Envoy-3 — Ysoki Envoy 3

| Field | Value | Source (URL, section) |
|---|---|---|
| Race | Ysoki (Small humanoid): +2 Dex, +2 Int, −2 Str; racial HP 2; Cheek Pouches; Darkvision; Moxie; **Scrounger: +2 racial bonus to Engineering, Stealth and Survival** | https://www.aonsrd.com/Races.aspx?ItemName=Ysoki |
| Theme | Icon: +1 Cha; chosen Profession (vidcaster) gets +1 (untrained here, so nothing prints); Culture is a class skill, or +1 to Culture checks if it already is (it is, for envoys) | https://www.aonsrd.com/Themes.aspx?ItemName=Icon (Theme Knowledge, 1st Level) |
| Class / level | Envoy 3; HP 6, SP 6 per level; skill ranks 8 + Int per level | https://www.aonsrd.com/Classes.aspx?ItemName=Envoy |
| Key ability | Charisma | https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Key Ability Score - Cha) |
| Class features (Core Rulebook) | 1st: envoy improvisation **Inspiring Boost**, expertise (1d6, Sense Motive), skill expertise **Diplomacy**; 2nd: envoy improvisation **Get 'Em**; 3rd: expertise talent **Slick Customer** (Diplomacy), weapon specialization | https://www.aonsrd.com/Classes.aspx?ItemName=Envoy ; https://www.aonsrd.com/Improvisations.aspx?ItemName=All ; https://www.aonsrd.com/ExpertiseTalents.aspx?ItemName=All |

The expertise die (1d6) is a roll-time insight bonus, not a fixed bonus, so it is printed and not
added to the Sense Motive or Diplomacy totals.

**Ability scores (point buy).**

| | Str | Dex | Con | Int | Wis | Cha |
|---|---|---|---|---|---|---|
| Base | 10 | 10 | 10 | 10 | 10 | 10 |
| Race (ysoki) | −2 | +2 | | +2 | | |
| Theme (icon) | | | | | | +1 |
| Points spent (total 10) | 0 | +1 | +2 | 0 | 0 | +7 |
| **Final** | **8** | **13** | **12** | **12** | **10** | **18** |
| Modifier | −1 | +1 | **+1** | +1 | +0 | +4 |

Points: 1 + 2 + 7 = 10. Con modifier +1 (≠ 0, `decisions.md §8`).

**Skill ranks** (8 + Int 1 = 9 per level × 3 = 27; max 3 per skill): Bluff 3, Computers 3,
Culture 3, Diplomacy 3, Engineering 3, Intimidate 3, Perception 3, Sense Motive 3, Stealth 3 (all
envoy class skills). Total 27. **Ranks in 9 skills (≥ 6, `decisions.md §9`).**

**Feats** (character feats at 1st and 3rd):

| Feat | When | Prerequisite met | Source |
|---|---|---|---|
| Mobility | 1st | Dex 13 (Dex 13) | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Mobility |
| Quick Draw | 3rd | BAB +1 (envoy 3rd: +2) | https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Quick%20Draw |

**Armour and weapons:**

| Item | Level | Price | Stats used | Bulk | Source |
|---|---|---|---|---|---|
| Carbon Skin, Graphite (light armour, worn) | 3 | 1,220 | EAC +3, KAC +4, max Dex +4, **ACP −1** | 1 | https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Graphite&Family=Carbon%20Skin |
| Semi-Auto Pistol, Tactical (small arm) | 1 | 260 | 1d6 P, range 30 ft., 9 rounds | L | https://www.aonsrd.com/WeaponDisplay.aspx?ItemName=Tactical&Family=Semi-Auto%20Pistol |
| Baton, Tactical (basic melee) | 1 | 90 | 1d4 B | L | https://www.aonsrd.com/WeaponDisplay.aspx?ItemName=Tactical&Family=Baton |

Armour bought new includes sizing for a Small wearer (https://www.aonsrd.com/Rules.aspx?ID=87,
Armor Size).

**Other gear:**

| Item | Qty | Price each | Bulk each | Source |
|---|---|---|---|---|
| Serum of Healing, Mk 1 | 2 | 50 | L | https://www.aonsrd.com/MagicItems.aspx?ItemName=Mk%201&Family=Serum%20of%20Healing |

**Credits:** 4,000 − (1,220 + 260 + 90 + 2 × 50) = 4,000 − 1,670 = **2,330 remaining**.

**Bulk:** 1 (armour) + 4 L items (pistol, baton, 2 serums → 0) = **1**. Limit ½ × Str 8 = 4.

## 5. Constraint check (`decisions.md §8`, `§9`)

| Constraint | SF-Soldier-3 | SF-Mystic-5 | SF-Technomancer-5 | SF-Envoy-3 |
|---|---|---|---|---|
| Con modifier ≠ 0 | +1 (Con 12) | −1 (Con 8) | +2 (Con 14) | +1 (Con 12) |
| Soldier armour max Dex binds | Dex +2 > max Dex +1 → yes | n/a | n/a | n/a |
| Envoy ranks in ≥ 6 skills | n/a | n/a | n/a | 9 skills |
| Spells known at every castable level | n/a | 0, 1st, 2nd | 0, 1st, 2nd | n/a |
| Not encumbered (bulk ≤ ½ Str) | 4 ≤ 8 | 1 ≤ 5 | 1 ≤ 5 | 1 ≤ 4 |

## 6. Oracle name check (names only)

Every race, class, theme, feat, class option, spell and item name above was checked against the
pinned oracle's `data/starfinder/paizo/core/*.lst` first column. Command (from the repo root):

```bash
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"; D="$PCGEN_REPO_DIR/data/starfinder/paizo/core"
awk -F'\t' -v n="Defiance Series, Squad" 'tolower($1)==tolower(n){c++} END{print n, c+0}' "$D/scr_equip.lst"
```

Results on 2026-10-02: every name returned ≥ 1, with these oracle spellings: `Medkit (Basic)`,
`Serum of Healing Mk 1`, `Battery` (the standard battery), `CLASS:Soldier` / `CLASS:Mystic` /
`CLASS:Technomancer` / `CLASS:Envoy`, and the lashunta subrace as `Lashunta` + template `Damaya`.
No pick needed an SD-g substitution.
