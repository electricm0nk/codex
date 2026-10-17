# CPR corebook v1.22 - Combat, Health, Vehicles inventory

Source: `/home/ubuntu/workspace/inbox/cyberpunk-red/pages/pNNN.txt`. All page files are under that dir.

**Page offset (verified):** printed page = pNNN - 1. Example: p169 prints "168". p168 is the chapter opener ("Friday night firefight", printed 167). The brief's "pp 167-194" maps to files p168-p195. Trauma Team opener is p220 (printed 219); content p221-p233.

Chapter map (file : content)
- p169-p170 : Initiative, actions list, Actions in Detail (Move, Split/ROF)
- p171 : Aimed Shots, drawing/stowing, Reload, ranged table legend
- p172 : Ranged Weapon table (11 types)
- p173 : Ranged resolution (REF + Skill + 1d10 vs DV)
- p174 : Single Shot DV by range, Autofire rules + Autofire DV table
- p175 : Arrows, Suppressive Fire, Shotgun Shells, Explosives
- p176-p177 : Melee weapon table, melee rules, Brawling damage table
- p178 : Grab / Choke / Throw
- p179-p181 : Martial Arts damage table, Forms and Special Moves
- p181-p182 : Fire, Drowning, Asphyxiation, Electrocution, Exposure, Falling, Poison, Drugs (generic), Radiation
- p183-p185 : Cover, cover HP, Shields, Human Shields, Armor rules
- p186 : Armor table
- p187 : Taking Damage steps, Wound States
- p188-p189 : Critical Injuries (Body, Head), Mortally Wounded, Death Saves
- p190-p193 : Vehicle Combat
- p194-p195 : Reputation, Facedowns
- p221-p233 : Trauma Team (wounds again, crit tables again, Stabilization, healing, Trauma Team org, hospital, Bodybank, Bodysculpting, Street Drugs, Therapy, Cyberpsychosis)

---

## 1. Action list (p169-p170; "Combat Actions in Brief")

Turn = 1 Move Action + 1 other Action (p169). Initiative = REF + 1d10 (p169). Turn ~3 s.

| Action | One-line effect (as printed) | File |
|---|---|---|
| Move Action | Move up to MOVE x 2 m/yds per Turn (or squares equal to MOVE) | p169 |
| Attack | Make a Melee or Ranged Attack | p169 |
| Choke | Choke an opponent you have Grabbed | p169, detail p178 |
| Equip/Drop Shield | Takes an Action | p169, p184 |
| Get into a Vehicle | Takes an Action (getting out is not an Action) | p169, p191 |
| Get Up | Get up after Prone; while Prone you cannot use a Move Action | p169 |
| Grab | Grab and hold an opponent or take an object they hold | p169, p178 |
| Hold Action | Hold until later in Initiative Queue; name trigger event or queue number, the Action, and target | p170 |
| Human Shield | Equip a Grabbed opponent as a Human Shield | p170, p185 |
| Reload | Fully reload and replace magazine with a single ammo type | p170, p171 |
| Run | Extra Move Action, only if a Move Action already taken this Turn | p170 |
| Start a Vehicle | Gain its MOVE and jump to top of Initiative Queue | p170, p191-p193 |
| Stabilize | Stabilize a target / pull out of Mortally Wounded | p170, p223 |
| Throw | Throw a grabbed opponent or an object | p170, p178 |
| Use NET Actions | Multiple NET Actions inside the NET | p170 (see netrunning inventory) |
| Use an Object | Manipulate an object not needing a Skill. Drawing easy weapon / dropping weapon is free; stowing is an Action | p170 |
| Use a Skill | Quick task with a Skill; longer tasks span Turns | p170 |
| Vehicle Maneuver | Action while driving to make a dangerous Maneuver | p170, p193 |

Count: 18 rows (manual enumeration of the p169-p170 table; the shell check below could only match 17 because of a regex gap on one indented label, so treat 18 as read-by-eye).

Other action rules:
- Split Movement and ROF (p170): you may Move, attack, Move. 2 ROF attacks can be split across the Move Action. Never more than two Attack Checks per Action. Cannot attack with two 1 ROF weapons in one Action even dual wielding.
- Prone (p170): cannot use Move Action until Get Up.
- Swim/Climb/running-start Jump cost 2 m/yds per m/yd; standing jump = half distance (p170).
- Aimed Shots (p171): max 1 ROF, takes entire Action, -8 to Check. Targets: Head (damage through head armor x2); Held Item (if 1 damage gets through body armor, target drops an item of your choice); Leg (if 1 damage gets through body armor, target suffers Broken Leg if any leg unbroken).
- Autofire cannot make Aimed Shots; Shotgun Shell cannot make Aimed Shots (p174, p175).

---

## 2. Ranged weapons

### 2a. Ranged Weapon table (p172) - what the sheet puts next to each weapon

| Weapon Type | Weapon Skill | Single Shot Damage | Std Magazine (ammo type) | ROF | Hands | Concealed? | Alt Fire / Special | Cost (category) |
|---|---|---|---|---|---|---|---|---|
| Medium Pistol | Handgun | 2d6 | 12 (M Pistol) | 2 | 1 | YES | None | 50eb (Costly) |
| Heavy Pistol | Handgun | 3d6 | 8 (H Pistol) | 2 | 1 | YES | None | 100eb (Premium) |
| Very Heavy Pistol | Handgun | 4d6 | 8 (VH Pistol) | 1 | 1 | NO | None | 100eb (Premium) |
| SMG | Handgun | 2d6 | 30 (M Pistol) | 1 | 1 | YES | Autofire (3), Suppressive Fire | 100eb (Premium) |
| Heavy SMG | Handgun | 3d6 | 40 (H Pistol) | 1 | 1 | NO | Autofire (3), Suppressive Fire | 100eb (Premium) |
| Shotgun | Shoulder Arms | 5d6 | 4 (Slug) | 1 | 2 | NO | Shotgun Shell | 500eb (Expensive) |
| Assault Rifle | Shoulder Arms | 5d6 | 25 (Rifle) | 1 | 2 | NO | Autofire (4), Suppressive Fire | 500eb (Expensive) |
| Sniper Rifle | Shoulder Arms | 5d6 | 4 (Rifle) | 1 | 2 | NO | None | 500eb (Expensive) |
| Bows & Crossbows | Archery | 4d6 | N/A (Arrow) | 1 | 2 | NO | Arrows | 100eb (Premium) |
| Grenade Launcher | Heavy Weapons | 6d6 | 2 (Grenade) | 1 | 2 | NO | Explosive | 500eb (Expensive) |
| Rocket Launcher | Heavy Weapons | 8d6 | 1 (Rocket) | 1 | 2 | NO | Explosive | 500eb (Expensive) |

Count: 11 rows. Cross-check: p172 column legend is in the column-header block; "Cost: ... Price Category listed below". Caveat: SMG and Heavy SMG use the "Handgun" skill as printed. This is the generic-weapon table; named/brand weapons live in the gear chapter (not in this inventory).

### 2b. Single Shot DVs by range (p174, "Single Shot DVs Based on Range") - EXACTLY as printed

| Weapon Type | 0-6 | 7-12 | 13-25 | 26-50 | 51-100 | 101-200 | 201-400 | 401-800 |
|---|---|---|---|---|---|---|---|---|
| Pistol | 13 | 15 | 20 | 25 | 30 | 30 | N/A | N/A |
| SMG | 15 | 13 | 15 | 20 | 25 | 25 | 30 | N/A |
| Shotgun (Slug) | 13 | 15 | 20 | 25 | 30 | 35 | N/A | N/A |
| Assault Rifle | 17 | 16 | 15 | 13 | 15 | 20 | 25 | 30 |
| Sniper Rifle | 30 | 25 | 25 | 20 | 15 | 16 | 17 | 20 |
| Bows & Crossbow | 15 | 13 | 15 | 17 | 20 | 22 | N/A | N/A |
| Grenade Launcher | 16 | 15 | 15 | 17 | 20 | 22 | 25 | N/A |
| Rocket Launcher | 17 | 16 | 15 | 15 | 20 | 20 | 25 | 30 |

Units m/yds. Count: 8 weapon-type rows x 8 bands = 64 cells (N/A cells included: 11).

Mapping gap (UNCLEAR, needs a rule decision): the weapon table has 11 types, the DV table has 8 rows. The text does not print the mapping. Reasonable reading: Medium/Heavy/Very Heavy Pistol -> "Pistol"; SMG and Heavy SMG -> "SMG". Not stated on p172/p174. Extraction check: the SMG row "15 13 15 20 25 25 30" reads as printed (0-6 is 15, 7-12 is 13); this is the real shape, not a column shift (cross-checked against the full-text file).

Reading rules:
- Ranged resolution: Attacker REF + Weapon Skill + 1d10 vs DV from range table, or Defender DEX + Evasion + 1d10 if Defender has REF 8+ and chooses to dodge. Defender wins ties (p173).
- Example on p171: Assault Rifle at 24 m/yds, DV15.

### 2c. Autofire DVs by range (p174) - as printed

| Weapon Type | 0-6 | 7-12 | 13-25 | 26-50 | 51-100 |
|---|---|---|---|---|---|
| SMGs | 20 | 17 | 20 | 25 | 30 |
| Assault Rifle | 22 | 20 | 17 | 20 | 25 |

Autofire rules (p174): Action + 10 bullets (need 10 in clip). Uses the Autofire Skill (not Weapon Skill). Roll 2d6, multiply by amount you beat the DV, up to the weapon's Autofire max (3 SMG, 4 Assault Rifle). Both dice 6 = Critical Injury. Armor reduces damage as normal. Targets with REF 8+ may dodge.

### 2d. Other ranged rules (p175)
- Arrows: never Reload a Bow/Crossbow; Basic Arrows can be retrieved.
- Suppressive Fire: Action + 10 bullets. Everyone on foot within 25 m/yds, out of cover, in line of sight rolls WILL + Concentration + 1d10 vs your REF + Autofire + 1d10. Failure: next Move Action must go to cover (plus Run if needed).
- Shotgun Shell: one Ranged Attack (REF + Shoulder Arms + 1d10) vs DV13. Hits every target in front within 6 m/yds (3 squares) for 3d6, one damage roll. No Aimed Shot. REF 8+ targets may dodge individually.
- Explosives: damage to all in 10 m/yd x 10 m/yd (5x5 squares), center a 2x2 target square. One damage roll. On a miss the GM relocates the blast square. REF 8+ may dodge to leave blast area. Cover not destroyed by the damage protects.
- Thrown objects (p178): DEX + Athletics + 1d10, max 25 m/yds, using Grenade Launcher DV entry.

---

## 3. Melee

### 3a. Melee Weapon table (p176)

| Melee Weapon Type | Examples | Hands | Damage | ROF | Concealed? | Cost |
|---|---|---|---|---|---|---|
| Light Melee | Combat Knife, Tomahawk | Varies by type | 1d6 | 2 | YES | 50eb (Costly) |
| Medium Melee | Baseball Bat, Crowbar, Machete | Varies by type | 2d6 | 2 | NO | 50eb (Costly) |
| Heavy Melee | Lead Pipe, Sword, Spiked Bat | Varies by type | 3d6 | 2 | NO | 100eb (Premium) |
| Very Heavy Melee | Chainsaw, Sledgehammer, Helicopter Blades, Naginata | Varies by type | 4d6 | 1 | NO | 500eb (Expensive) |

Count: 4 rows. Per-weapon hands are NOT in this chapter ("Varies by type").

### 3b. Melee rules (p177)
- Melee: attacker DEX + Melee Weapon Skill + 1d10 vs defender DEX + Evasion + 1d10. Defender wins ties. Reach 2 m/yds.
- All melee weapons ignore half of the Defender's armor, round up (p177; example p176: SP11 treated as SP6).
- Very Heavy is 1 ROF; all other melee 2 ROF.
- BODY 8+ may wield a two-handed melee weapon in one hand.
- Thrown melee weapons deal stated damage but do NOT halve SP (p178).

### 3c. Brawling damage by BODY (p177) - Brawling does NOT ignore half armor

| BODY | 4 or Under | 5 to 6 (or <=4 with Cyberarm) | 7 to 10 | 11 or Higher |
|---|---|---|---|---|
| Damage | 1d6 | 2d6 | 3d6 | 4d6 |

Cyberarm clause (p177): Brawling damage with a Cyberarm is always at least 2d6.

### 3d. Martial Arts damage by BODY (p179) - Martial Arts DOES ignore half armor (round up)

| BODY | 4 or Under | 5 to 6 | 7 to 10 | 11 or Higher |
|---|---|---|---|---|
| Damage | 1d6 | 2d6 | 3d6 | 4d6 |

2 ROF, needs at least 1 point in the Skill. Resolution: DEX + Form Skill + 1d10 vs defender DEX + Evasion + 1d10, or a DV for some Special Moves.

### 3e. Martial Arts Special Moves (p180-p181)

All Forms use Shared Special Moves; each Form has two unique ones. Need at least 1 point in the Form. Count: 1 shared + 4 forms x 2 = 9 moves printed.

| Form | Move | Requirement | DV | Effect (short) | File |
|---|---|---|---|---|---|
| Shared | Recovery | none | 13 | Get Up Action costs no Action | p180 |
| Aikido | Disarming Combination | Hit same target with Brawling and Martial Arts Attack this Turn | 15 | Object in target's hands is yours or on floor | p180 |
| Aikido | Iron Grip | Target Grabbed, not already Iron Gripped | 15 | Target at extra -2 to escape; cannot make Ranged Attacks | p180 |
| Karate | Armor Breaking Combination | Hit same target with Melee Weapon and Martial Arts Attack this Turn | 15 | Target's worn armor ablated an extra 2 | p180 |
| Karate | Bone Breaking Strike | WILL 8+ | resolution (no DV printed) | Instead of 2 attacks; target gets Broken Ribs. With -8: Cracked Skull instead | p180 |
| Judo | Counter Throw | Dodged all Melee Attacks since last Turn | 15 | Throw one target whose attack you dodged; unavoidable | p180 |
| Judo | Grab Escape | Hit target grappling you with 2 Melee Attacks this Turn | 15 | End grapple; target gets Broken Arm (your pick) | p180 |
| Taekwondo | Pressure Point Strike | WILL 8+ | resolution (no DV printed) | Instead of 2 attacks; target gets Spinal Injury. With -8: Brain Injury instead | p181 |
| Taekwondo | Flying Kick | MOVE 8+ and moved 4+ m/yds this Turn | resolution (no DV printed) | Uses Action + remaining move; damage to body; target Prone and removed from open vehicle | p181 |

UNCLEAR: Karate/Bone Breaking Strike, Taekwondo/Pressure Point Strike and Flying Kick say "use the Martial Arts Special Move Resolution against a single target"; that resolution is the Form-skill vs Evasion contest above, so no fixed DV. Not stated explicitly as "no DV".

### 3f. Grapple, Choke, Throw (p178)
- Grab: both roll DEX + Brawling + 1d10; winner either grabs (Grapple, both -2 to all Actions) or takes an object. Needs a free hand. Grappled defender cannot Move, is dragged. No two-handed weapons in the Grapple. Attacker may end Grapple free; others must win a Grab to break it.
- Choke: Action; deals BODY directly to HP, ignores armor, no ablation. If it would drop a target with >1 HP below 0 it leaves them at 1 HP and Unconscious. Choked 3 Rounds in a row = Unconscious regardless of HP.
- Throw: Action; BODY direct to HP, ignores armor; ends Grapple; target Prone.

---

## 4. Damage, armor, cover, hazards

### 4a. Taking Damage (p187)
1. Attacker rolls damage.
2. Subtract armor SP at that location (body unless Aimed Shot to head). Remainder comes off HP.
3. If any damage got through, that location's armor is ablated by 1 SP until repaired.
Poison and fire bypass armor. SP does not stack: only the highest source in a location counts; all worn armor in a location ablates together (p185). Heavier armor: apply the single worst Armor Penalty once (p185).

### 4b. Armor table (p186)

| Armor | SP | Armor Penalty | Cost | Note |
|---|---|---|---|---|
| Leathers | 4 | None | 20eb (Everyday) | |
| Kevlar | 7 | None | 50eb (Costly) | |
| Light Armorjack | 11 | None | 100eb (Premium) | |
| Bodyweight Suit | 11 | None | 1,000eb (V. Expensive) | stores Cyberdeck, supports Interface Plugs |
| Medium Armorjack | 12 | -2 REF, DEX, MOVE | 100eb (Premium) | |
| Heavy Armorjack | 13 | -2 REF, DEX, MOVE | 500eb (Expensive) | |
| Flak | 15 | -4 REF, DEX, MOVE | 500eb (Expensive) | |
| Metalgear | 18 | -4 REF, DEX and MOVE | 5,000eb (Luxury) | |

Count: 8 rows. UNCLEAR: the table does not split Head vs Body pieces; the text says armor "can be purchased for either the head or body locations" (p185). Head/body pricing split is not on this page.

### 4c. Cover (p183-p184)
- In cover only when fully behind something that could stop a bullet and enemy has no line of sight. No partial cover.
- Cover has HP; a 2x2 m/yd (1 square) section can be attacked. At 0 HP cover is destroyed; excess damage lost.
- Explosives: not hurt if damage cannot destroy cover; if it destroys cover, full damage (p175).
- Steel cover cannot be damaged by Martial Arts or Brawling unless Cyberarm or BODY 10+.

Cover HP table (p183):

| Material | Thick HP | Thin HP |
|---|---|---|
| Steel | 50 | 25 |
| Stone | 40 | 20 |
| Bulletproof Glass | 30 | 15 |
| Concrete | 25 | 10 |
| Wood | 20 | 5 |
| Plaster/Foam/Plastic | 15 | 0 (Not Cover) |

Count: 6 material rows. Example list p184 has 24 named examples (Bank Vault Door ... Wooden Door); it is a lookup, low value for a sheet.

### 4d. Shields (p184-p185)
- Equip/drop shield = Action. Shield in hand = in cover while HP > 0. Interposing means no dodge. Shield takes the whole attack.
- Bulletproof Shield: 10 HP, 100eb (Premium). Corpse Shield: HP = BODY the corpse had, cost "Depends on how many bullets you went through" (p185). Count: 2 shield rows.
- Human Shield (p185): Action after Grapple; counts as cover; cannot block Melee or head Aimed Shots; shield cannot dodge; if it dies it becomes a shield with HP = its BODY.

### 4e. Head shots
Aimed Shot to head: -8, entire Action, 1 ROF max; damage that gets through head armor x2 (p171). Cracked Skull crit changes it to x3 (p189). Non-aimed attacks always hit the body location (p187).

### 4f. Other damage (p181-p182)
| Hazard | Rule | File |
|---|---|---|
| On Fire | End of your Turn, direct to HP, no ablation, until Action to put out. Mild (wood fire) 2; Strong (gasoline) 4; Deadly (thermite) 6 | p181 |
| Drowning | Hold breath BODY minutes; then BODY direct to HP at start of each Turn, ignores armor | p181 |
| Asphyxiation | As drowning; in space also 1d6 to INT, REF, DEX each Turn end; INT 0 = dead | p181 |
| Electrocution | 6d6 on contact, soaked by armor; repeats each Turn end while in contact | p181 |
| Exposure | 1d6 direct to HP per day; no natural healing while exposed | p182 |
| Falling | Fall 40 m/yds per Turn end; DV15 Athletics to catch ledge; landing from 10+ m/yds: 2d6 per 10 m/yds (body armor soaks); fail DV15 Athletics = Broken Leg. 2 Cyberlegs negate unless fall > 30 m/yds | p182 |
| Poison (Mild/Strong/Deadly) | Resist Torture/Drugs DV 11/13/15; 1d6/2d6/3d6 direct to HP; armor not ablated | p182 |
| Drugs (generic) | Mild/Strong/Deadly DV 11/13/15; Inebriation / Suggestibility / Designer's Intention | p182 |
| Radiation | Treated as Mildly On Fire each Turn in hot zone (Deadly in extreme zone); cannot put out while there | p182 |

---

## 4g. Critical Injuries

Trigger: two or more damage dice come up 6 on a Melee or Ranged Attack (p188, p221). Roll 2d6 on Body table unless Aimed Shot at head; reroll until you get an injury the target does not have. Every crit deals 5 Bonus Damage direct to HP, no ablation, even if no damage got through SP. Mortally Wounded targets take a crit on every Melee/Ranged damage (p187, p189).

Tables appear TWICE: p188-p189 (combat chapter) and p222-p223 (Trauma Team). Contents match on the extraction; treat p222-p223 as the canonical copy (same rows, same columns). Row-for-row comparison done by eye only (no diff run).

Quick Fix = removes effect for the rest of the day, 1 minute per attempt, can be done on self. Treatment = removes permanently, 4 hours per attempt, not on self (p224).

### Body table (p188 / p222) - 11 rows

| 2d6 | Injury | Effect | Quick Fix | Treatment |
|---|---|---|---|---|
| 2 | Dismembered Arm | Arm gone; drop items in that hand; Base Death Save Penalty +1 | N/A | Surgery DV17 |
| 3 | Dismembered Hand | Hand gone; drop items in that hand; Base DSP +1 | N/A | Surgery DV17 |
| 4 | Collapsed Lung | -2 MOVE (min 1); Base DSP +1 | Paramedic DV15 | Surgery DV15 |
| 5 | Broken Ribs | End of every Turn you move > 4 m/yds on foot, re-suffer Bonus Damage to HP | Paramedic DV13 | Paramedic DV15 or Surgery DV13 |
| 6 | Broken Arm | Arm cannot be used; drop items in that hand | Paramedic DV13 | Paramedic DV15 or Surgery DV13 |
| 7 | Foreign Object | End of every Turn you move > 4 m/yds on foot, re-suffer Bonus Damage to HP | First Aid or Paramedic DV13 | Quick Fix removes Injury Effect permanently |
| 8 | Broken Leg | -4 MOVE (min 1) | Paramedic DV13 | Paramedic DV15 or Surgery DV13 |
| 9 | Torn Muscle | -2 to Melee Attacks | First Aid or Paramedic DV13 | Quick Fix removes Injury Effect permanently |
| 10 | Spinal Injury | Next Turn no Action, Move Action allowed; Base DSP +1 | Paramedic DV15 | Surgery DV15 |
| 11 | Crushed Fingers | -4 to all Actions involving that hand | Paramedic DV13 | Surgery DV15 |
| 12 | Dismembered Leg | Leg gone; -6 MOVE (min 1); cannot dodge; Base DSP +1 | N/A | Surgery DV17 |

### Head table (p189 / p222-p223) - 11 rows

| 2d6 | Injury | Effect | Quick Fix | Treatment |
|---|---|---|---|---|
| 2 | Lost Eye | Eye gone; -4 Ranged Attacks and vision Perception; Base DSP +1 | N/A | Surgery DV17 |
| 3 | Brain Injury | -2 to all Actions; Base DSP +1 | N/A | Surgery DV17 |
| 4 | Damaged Eye | -2 Ranged Attacks and vision Perception | Paramedic DV15 | Surgery DV13 |
| 5 | Concussion | -2 to all Actions | First Aid or Paramedic DV13 | Quick Fix removes Injury Effect permanently |
| 6 | Broken Jaw | -4 to all Actions involving speech | Paramedic DV13 | Paramedic or Surgery DV13 |
| 7 | Foreign Object | End of every Turn you move > 4 m/yds on foot, re-suffer Bonus Damage | First Aid or Paramedic DV13 | Quick Fix removes Injury Effect permanently |
| 8 | Whiplash | Base DSP +1 | Paramedic DV13 | Paramedic or Surgery DV13 |
| 9 | Cracked Skull | Aimed Shots to your head multiply damage through SP by 3 instead of 2; Base DSP +1 | Paramedic DV15 | Paramedic or Surgery DV15 |
| 10 | Damaged Ear | If you move > 4 m/yds in a Turn, no Move Action next Turn; -2 hearing Perception | Paramedic DV13 | Surgery DV13 |
| 11 | Crushed Windpipe | Cannot speak; Base DSP +1 | N/A | Surgery DV15 |
| 12 | Lost Ear | Ear gone; no Move Action next Turn after moving > 4 m/yds; -4 hearing Perception; Base DSP +1 | N/A | Surgery DV17 |

Count: 22 crit rows total. UNCLEAR on both tables: the "Treatment" cell for Foreign Object, Torn Muscle and Concussion reads "Quick Fix removes Injury Effect permanently" in the Treatment column; the layout merges two header cells (Quick Fix | Treatment). Treat it as: Quick Fix = First Aid or Paramedic DV13, and the Quick Fix itself is permanent (no separate Treatment). This reading follows the printed sentence; confirm against the PDF image if a sheet prints it.

---

## 5. Wound states, Death Saves, stabilization, healing

HP formula (p080, character creation): HP = 10 + 5 x (average of BODY and WILL, rounded up). The p080 table is the same formula. Seriously Wounded threshold = half of HP, rounded up. Death Save = BODY. (p080)

Wound State table (p187 and again p221). Verbatim fragments:

| State | Threshold | Effect | Stabilization DV |
|---|---|---|---|
| Lightly Wounded | "Less than Full HP" | None | DV10 |
| Seriously Wounded | "Less than 1/2 HP (round up)" | -2 to all Actions | DV13 |
| Mortally Wounded | "Less than 1 HP" | -4 to all Actions; -6 to MOVE (Minimum 1); must make a Death Save at start of each of their Turns; suffer a Critical Injury whenever damaged by a Melee or Ranged Attack; Death Save Penalty increases by 1 | DV15 to heal back to 1 HP, and Unconscious ("Gone from the world for 1 minute") |
| Dead | "One failed Death Save" | Death | "Never coming back" |

Each new Wound State replaces the previous one's effect (p187).

Note on a source inconsistency: p080 says at zero HP the character "enter[s] the Death State" while p187/p221 say Mortally Wounded is "Less than 1 HP". The combat chapter wording ("Less than 1 HP", so 0 or below) is the rule to print. Use the combat chapter.

Death Save (p189, p223): at the start of each of your Turns while Mortally Wounded roll d10. Roll under BODY (plus Death Save Penalty as an added +1 on the roll each time) to live; a roll of 10 always fails. Any failed save = dead. Penalty rises each save and resets to Base Death Save Penalty on Stabilization back to 1 HP. Base Death Save Penalty goes up with some Critical Injuries (see tables).

Rule text is ambiguous on one point, quote: "Roll a d10. If you roll under your BODY, you live ... each future Death Save you roll is made with an additional +1". Read: roll d10 + Death Save Penalty, must be under BODY. UNCLEAR whether the first save carries a penalty; the text says Mortally Wounded raises the penalty by 1 on entry (p189 "their Death Save Penalty increases by 1").

Stabilization (p223): DV10/13/15 by state. It takes an Action. Roll TECH + First Aid or Paramedic + 1d10. Stabilized targets heal HP equal to BODY per full day of rest until Full HP. If the patient pushes it: no HP that day, wounds reopen, must be stabilized again. Mortally Wounded + stabilized: heal to 1 HP and Unconscious for exactly 1 minute (p224); Death Saves still apply if hurt to Mortally Wounded while unconscious.

Healing skills (p224): Cybertech (no stabilize; quick fix/treat cyberware crits), First Aid (always at least +2; stabilize; common crits), Paramedic (x2 cost Skill; stabilize; all but deadliest crits), Surgery (Medtech Role Ability only; the deadliest crits).
Cyberware and crits (p224): still take the crit effect and Bonus Damage; Cybertech replaces First Aid/Paramedic/Surgery for fixing it, same DV and time.

Hospital bill by highest DV of Stabilization or Treatment (p226) - 4 rows: DV17+ 1,000eb (V. Expensive); DV15 500eb (Expensive); DV13 100eb (Premium); DV10 50eb (Costly). Bed 100eb (Premium) per night.

Trauma Team (p224-p225):
- Plans: Silver 500eb/month; Executive 1,000eb/month. Silver pays hospital price for Surgery Treatments; Executive covers them.
- Calling: an Action (or free via Agent with linked Biomonitor when HP < BODY or on dismembering crit). Roll d6 for Rounds to arrival; they join top of Initiative.
- Team of 5 arrives in an AV-4. Stat block (p225):

| Member | Combat Number | SP | HP | MOVE and BODY |
|---|---|---|---|---|
| Doctor | 10 | 11 | 20 | 4 |
| Medical Assistant | 10 | 7 | 25 | 6 |
| Pilot | 10 | 7 | 25 | 6 |
| Security Officer (x2) | 10 | 13 | 30 | 4 |

UNCLEAR: the last column header reads "MOVE & BODY" with one number per row; the extraction does not show if MOVE and BODY are equal. Assume equal (4/4, 6/6); confirm on the image.

Bodybank and replacement (p227): replacement limb cost is included in the hospital visit; own spare limb 50eb (Costly). Cyberware install surgery: Mall DV13 / 100eb; Clinic DV15 / 500eb; Hospital DV17 / 1,000eb (p227). Bodysculpting: Standard 500eb, Clinic, DV15, materials 100eb; Exotic 1,000eb, Hospital, 4d6 Humanity Loss, DV17, materials 500eb (p228).

---

## 6. Street Drugs (p228-p230) - 5 drugs

Mechanics (p228): taken by Airhypo (Action, one dose) or a Melee Weapon Attack on an unwilling target. Primary Effect is automatic. When it ends, roll WILL + Resist Torture/Drugs + 1d10 vs Secondary Effect DV; secondary effects are permanent/habit-forming until therapy. More doses extend the primary effect by its full duration.

| Drug | Cost per dose | Duration | Primary effect | Secondary effect (addiction DV) | File |
|---|---|---|---|---|---|
| Black Lace | 50eb (Costly) | 24 h | Ignore Seriously Wounded effects; 2d6 Humanity Loss on taking a dose (returned if not hit by secondary) | DV17: Humanity not returned; addicted; while addicted and not under primary effect REF -2 | p228 |
| Blue Glass | 20eb (Everyday) | 4 h | GM calls "flash out" hallucinations; lose Action that Turn | DV15: addicted; while addicted flash out unless under primary effect (primary effect flips: immune to flashing while dosed) | p229 |
| Boost | 50eb (Costly) | 24 h | INT +2 (can exceed 8) | DV17: addicted; while addicted INT -2 | p229 |
| Smash | 10eb (Cheap) | 4 h | +2 to Dance, Contortionist, Conversation, Human Perception, Persuasion, Acting | DV15: addicted; -2 to the same six Skills; roleplay cravings | p230 |
| Synthcoke | 20eb (Everyday) | 4 h | REF +1 (can exceed 8); paranoid ideation | DV15: addicted; REF -2 unless under primary effect; roleplay cravings | p230 |

Count: 5 (command below). Therapy (p230-p231): 1 week per therapy, Medtech cannot self-treat. Addiction Therapy 1,000eb (V. Expensive), DV15, materials 500eb (Expensive), patient auto-fails secondary roll vs that drug for 1 year. Standard Humanity Loss Therapy 500eb, DV15, materials 100eb, regain 2d6 Humanity. Extreme Humanity Loss Therapy 1,000eb, DV17, materials 500eb, regain 4d6 Humanity. Humanity max drops 2 per cyberware piece, 4 per borgware piece.

Cyberpsychosis (p233): EMP 3+ none; 2 borderline; 1 borderline cyberpsychosis (3 Hare traits); 0 cyberpsychosis (5 traits); 0 and negative Humanity = extreme (GM plays). Trauma Humanity Loss table (p232): torture 1d6; horrific death witnessed 1d6; long-term mental stress 2d6; long-term environmental stress 2d6.

---

## 7. Vehicle combat (p190-p193)

Vehicle stats table columns: Description, SDP, Seats, Speed (Combat) in MOVE, Speed (Narrative) MPH/KPH, Cost. No handling stat printed. Count: 14 vehicles (5 land, 4 sea, 5 air).

| Vehicle | Class | SDP | Seats | Combat MOVE | Narrative | Cost | File |
|---|---|---|---|---|---|---|---|
| Roadbike | Land | 35 | 2 | 20 | 100 MPH / 161 KPH | 20,000eb (Super Luxury) | p191 |
| Superbike | Land | 35 | 2 | 60 | 300 MPH / 483 KPH | 100,000eb | p191 |
| Compact Groundcar | Land | 50 | 4 | 20 | 100 MPH / 161 KPH | 30,000eb | p191 |
| High Performance Groundcar | Land | 50 | 4 | 40 | 200 MPH / 322 KPH | 50,000eb | p191 |
| Super Groundcar | Land | 50 | 2 | 60 | 300 MPH / 483 KPH | 100,000eb | p191 |
| Jetski | Sea | 35 | 2 | 20 | 60 MPH / 97 KPH | 20,000eb | p191 |
| Speedboat | Sea | 50 | 4 | 20 | 60 MPH / 97 KPH | 30,000eb | p191 |
| Cabin Cruiser | Sea | 60 | 2 per Room | 10 | 15 MPH / 24 KPH | 30,000eb per room below deck (min two rooms) | p191 |
| Yacht | Sea | 100 | 4 per Room | 10 | 15 MPH / 24 KPH | 50,000eb per room below or above deck (min four rooms) | p191 |
| Gyrocopter | Air | 35 | 2 | 20 | 100 MPH / 161 KPH | 20,000eb | p192 |
| Helicopter | Air | 60 | 4 | 40 | 200 MPH / 322 KPH | 40,000eb | p192 |
| AV-4 Multipurpose Aerodyne | Air | 100 | 6 | 40 | 200 MPH / 322 KPH | 50,000eb | p192 |
| AV-9 Super Aerodyne | Air | 60 | 2 | 60 | 300 MPH / 483 KPH | 100,000eb | p192 |
| Aerozep | Air | 100 | 2 per Room | 20 | 100 MPH / 161 KPH | 30,000eb per room below deck (min two rooms) | p192 |

All costs are Price Category "Super Luxury". Extraction caveat: Yacht/Cabin Cruiser "Cost" and "Seats" wrap across several lines in the text file; values above were joined by eye and match the visible row order.

Vehicle rules:
- SDP is vehicle HP. Alive while >= 1 SDP; at 0 it is Destroyed, no longer cover, cannot move until repaired with Vehicle Tech (p192).
- Vehicles cannot dodge; occupants can dodge attacks aimed at them. Ranged attacks use the range table. Glass has no HP and no cover; Bulletproof Glass is a Nomad upgrade (Nomad Role Ability, printed "pg. 163", so file p164) (p192).
- Weak point Aimed Shot: whole Action, -8, damage through vehicle SP x2. Against stationary vehicle with a melee attack: auto-hit; moving: DV13 and still -8 (p192).
- Driving: Get in = Action; get out = free. Start/Stop = Action. Without Interface Plugs, one hand on the wheel; taking it off means automatic Lose Control next Turn (p192).
- When started (p193): driver goes to top of Initiative; vehicle MOVE replaces driver MOVE (penalties ignored); driver cannot Run.
- Basic driving: if REF + Control Skill > 9 no Check; otherwise Action every Turn for DV10 REF + Control Skill + 1d10, failure = Lose Control.
- Maneuvers (Action + Move Action, REF + Control Skill + 1d10) DV table (p193): Swerve 13; Sharp Turn 13; Emergency Stop 13; Bootleg Turn 17; Do a Jump 17; Landing (Air) 13; Aerobatic Maneuver (Air) 17. Count: 7.
- Lose Control: GM decides movement; impact = ramming.
- Ramming: both vehicle and the rammed thing with HP take 6d6; everyone involved suffers Whiplash Critical Injury. If target's HP hits 0, movement continues; else it stops. Dodging a rammer on foot: DV13 DEX + Evasion + 1d10; success may land on the vehicle (p193).
- Vehicle weapons / mounts: not in this chapter (the Trauma Team text references a Vehicle Heavy Weapon Mount and Tsunami Arms Helix, p225 -> see p347 / gear inventory).
- Market appendix: this chapter lists the base vehicle types itself (p191-p192). Vehicle upgrade prices and Nomad vehicle details are elsewhere (Nomad role ability, gear/market chapter).

---

## 8. Reputation and Facedowns (p194-p195)

- Reputation (p194): a Level 1-10 awarded by the GM; a new deed replaces the old one only if the Level is higher. Most Characters start at 0. Meeting people: roll 1d10; under the Reputation Level = they have heard of you. Negative deeds can replace a positive Reputation (and then work against you).
- Reputation table (p194), 10 rows: 1 Anyone who was there; 2 immediate friends; 3 co-workers and casual acquaintances; 4 all over the local area; 5 beyond local area recognize name; 6 beyond local area know you on sight; 7 news story or two; 8 regularly headlines and screamsheets; 9 always screamsheets and TV; 10 known worldwide.
- Facedown (p195): both roll COOL + Reputation + 1d10. If your defining Reputation event is cowardice, Reputation Level is treated as negative. Tie: nothing. Loser either backs down, or takes -2 to any future Actions against that opponent due to fear until they defeat them once.

---

## 9. Counts and the exact commands (all run from `/home/ubuntu/workspace/inbox/cyberpunk-red/pages`)

| Item | Count | Command |
|---|---|---|
| Body crit rows | 11 | `awk '/Critical Injuries to the Body/{f=1} /Critical Injuries to the Head/{f=0} f && /^ +(2\|3\|4\|5\|6\|7\|8\|9\|10\|11\|12) +[A-Z]/' p222.txt \| wc -l` (verified: 11) |
| Head crit rows | 11 | `awk '/Critical Injuries to the Head/{f=1} f && /^ +(2\|3\|4\|5\|6\|7\|8\|9\|10\|11\|12) +[A-Z]/' p222.txt p223.txt \| wc -l` (verified: 11) |
| Street drugs | 5 | `cat p228.txt p229.txt p230.txt \| grep -c "Cost per Dose"` (verified: 5) |
| Ranged weapon types | 11 | `grep -c "Alt. Fire Modes" p172.txt` (verified: 11) |
| Melee weapon types | 4 | `grep -c -E "[0-9]d6 +[12] +(YES\|NO)" p176.txt` (verified: 4) |
| Armor rows | 8 | `grep -E " (None\|-[24] REF)" p186.txt \| wc -l` (verified: 8) |
| Cover material rows | 6 | awk over p183 (verified: 6; command in session, matches the 6 rows on p183) |
| Vehicles | 14 | `grep -c -E "[0-9]+ MOVE" p191.txt p192.txt` gives 9 + 5 (verified: 14) |
| Range DV table rows | 8 weapon types, 64 cells | ESTIMATE from reading p174 (a row-regex returned 9 including a wrapped Rocket Launcher label line) |
| Action rows | 18 | ESTIMATE (manual read of p169-p170; shell label check matched 17) |
| Cover examples | 24 | ESTIMATE (name-by-name grep matched 22 + `Bar`/`Tree` = 24) |
| Martial Arts moves | 9 | ESTIMATE (manual read of p180-p181) |
| Maneuvers | 7 | ESTIMATE (manual read of p193) |
| Trauma Team members | 5 (4 rows) | manual read of p225 |

---

## 10. Text-extraction quality (pages with garbling)

- p172 (ranged weapon table): readable but the SMG/Heavy SMG ammo "(M Pistol)/(H Pistol)" tags are easy to mis-assign; check by row. OK after reading by eye.
- p174 (range DV table): clean numerically. The "Rocket Launcher" label wraps to two lines.
- p176 (melee table + sidebar): table is interleaved with the sidebar example text; columns recoverable.
- p186 (armor): table fine; the artist byline "BY PEDRAM MOHAMMADI" lands inside the table rows.
- p187, p221 (wound states): the Mortally Wounded effect cell is split across 8+ lines; the Stabilization DV column is interleaved with it. Cross-checked between p187 and p221 (same text). The three Stabilization DVs (10/13/15) are confirmed by prose on p223.
- p188-p189, p222-p223 (crit tables): Quick Fix and Treatment columns are merged in places. "First Aid or Paramedic DV13" and "Quick Fix removes Injury Effect permanently" interleave across lines. Read-by-eye; see the UNCLEAR note in section 4g.
- p191-p192 (vehicles): Cabin Cruiser, Yacht and Aerozep cost/seat cells wrap across 4-6 lines; read by eye.
- p225 (Trauma Team stat block): MOVE & BODY column prints a single number per row; ambiguity noted in section 5.
- p231 (therapy table): wide multi-line cells; effect text for the two Humanity therapies is identical in the extraction (appears in the book too, with differing dice 2d6 vs 4d6).
- Sidebars with vertical text and "BY <artist>" bylines appear mid-page on p168, p173, p178, p179, p186, p194, p226, p229. Those lines are noise, not rules.
- p167 is not in this chapter (it is a screenshot of an in-world bookstore UI) despite the brief; chapter starts at p168.

---

## 11. Print vs compute split (standing rule: only sheet-total-feeding values get computed; text rules get printed)

COMPUTE (these feed sheet totals):
- Max HP = 10 + 5 x ceil((BODY + WILL) / 2). Source p080. (Verify the p080 table agrees: BODY 2 / WILL 2 = 20; BODY 15 / WILL 10 = 75.)
- Seriously Wounded threshold = ceil(HP / 2). p080, p187.
- Mortally Wounded threshold: 0 HP and below ("Less than 1 HP"). Fixed; print, not compute.
- Death Save target = BODY; plus the current Death Save Penalty (Base Death Save Penalty +1 per qualifying crit). p080, p189.
- Humanity = EMP x 10 (p081); effective EMP from Humanity tens digit (p230). Sheet total.
- Armor SP per location = highest single worn source; ablation tracked by hand. Penalty to REF, DEX, MOVE = worst single armor penalty. p185-p186. (Computed totals; the ablation counter is a write-in.)
- Brawling and Martial Arts damage die = lookup of BODY (and Cyberarm floor 2d6). p177, p179.
- Initiative base = REF (the d10 is rolled at the table). p169.
- Move Action distance = MOVE x 2 m/yds. p169.
- Stabilization check modifier = TECH + First Aid/Paramedic. p223.
- Range-to-DV: per weapon, the DV row from the weapon's type is a fixed lookup; print the row beside the weapon (the player reads the DV at the range band). Not a computed total.

PRINT (procedure text, no totals):
- Action list, Aimed Shot, Autofire/Suppressive Fire/Shotgun Shell/Explosives procedure, Grab/Choke/Throw, Martial Arts Special Moves, cover rules and cover HP table, Human Shields, Falling/Fire/Drowning/Radiation/Poison tables, critical-injury tables with Quick Fix and Treatment DVs, Death Save procedure, stabilization procedure and healing rate (BODY HP per day), hospital costs, Trauma Team, Street Drugs table, Therapy, Cyberpsychosis table, Vehicle stats and Maneuver DV table, ramming, Reputation and Facedown.
- Critical injury penalties applied as modifiers (-2 MOVE, -4 to Actions, etc.) are print and write-in: do NOT build an active-injury simulator.

Not covered by this inventory (what the proof does not reach): named brand weapons and per-weapon hands for melee (gear chapter); Armor head/body price splits; vehicle weapon mounts and upgrades; Role-specific combat Role Abilities (Solo Combat Awareness, Lawman Backup, etc.); Netrunning damage (BLACK ICE); Cyberware damage rules. Counts above are from this chapter only.
