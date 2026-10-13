# Cyberpunk RED corebook v1.22 - Netrunning inventory

Source: `/home/ubuntu/workspace/inbox/cyberpunk-red/pages/pNNN.txt`. Printed page = pNNN minus 1 on these pages (p204.txt prints "203"). The cited file is the pNNN file.
All commands below are run from `P=/home/ubuntu/workspace/inbox/cyberpunk-red/pages`.

Page map (Netrunning chapter = p196-p219):

| Files | Content |
|---|---|
| p196 | chapter opener |
| p197 | cyberdeck slot/cost table, Virtuality Goggles, Neural Link/Interface Plugs |
| p198 | Meat vs NET Actions, Interface rank -> NET Actions table |
| p199 | NET Action list, Jack In/Out, Interface Abilities intro |
| p200 | Scanner, Backdoor, Cloak, Control, Eye-Dee, Pathfinder |
| p201 | Pathfinder cont., Slide, Virus (+ virus examples) |
| p202 | Zap, NET combat, Derezzed, Programs intro |
| p203 | Program classes, how to read tables |
| p204-p205 | Boosters, Defenders, Attackers |
| p205-p208 | Black ICE rules, Black ICE table |
| p209 | Misc Upgrades, Cyberdeck Hardware |
| p210 | Doing a Netrun, example architecture, "Building" heading |
| p211 | Steps 0-3 incl. Difficulty table |
| p212 | Lobby table, Body table |
| p213 | Demons table |
| p213-p217 | Defenses (Active, Emplaced, Environmental) |
| p218-p219 | Home Security 2045 (buying an Architecture) |

Duplicate "gear shop" copies of the same data (clean, identical rows): Hardware p369, Programs p369-p372, Buy-an-Architecture p373, deck prices p352, deck slot text p354, Bodyweight Suit p098/p351, Cyberarm Cyberdeck option p365, Interface Plugs p360, Interface role ability p148.

## 1. Interface actions

Roll formula (p199): "Interface + 1d10 vs. DV" for all abilities except Zap (and Scanner is a Meat Action). Role summary table is on p148.

| Ability | Action cost | Effect (p) | Roll / DV |
|---|---|---|---|
| Jack In/Out | NET Action | Enter/leave an Architecture within 6 m/yds of an access point; a wall blocks it (p199) | no roll |
| Use Interface Ability | NET Action | wrapper action for the abilities below (p199) | n/a |
| Activate/Deactivate Program | NET Action | (p199, p202) | no roll |
| Miscellaneous | NET Action | GM-adjudicated (p199) | GM |
| Scanner | Meat Action (not a NET Action) | find Meatspace location of access points in area; higher check = more/farther (p200) | Interface + 1d10, GM discretion |
| Backdoor | NET Action | break a Password; auto-pass if you know it (p200) | vs Password DV |
| Cloak | NET Action | hide your traces and Virus; Pathfinder DV for others = your Cloak Check (p200) | Interface + 1d10 |
| Control | NET Action | take a Control Node; each attached thing needs a separate NET Action; node once/Turn; DV to wrest from another = their Control Check (p200) | vs Control Node DV |
| Eye-Dee | NET Action | identify a File and its value (p200) | vs File DV |
| Pathfinder | NET Action | reveal map; see floors equal to Check or up to first obstruction with DV above Check (p200-p201) | Interface + 1d10 |
| Slide | NET Action | flee a single non-Demon Black ICE to an adjacent floor, not past obstruction; once per Turn; "can't Slide preemptively" (p201) | Interface + 1d10 vs Black ICE PER + 1d10 |
| Virus | NET Actions (GM sets count) | at lowest floor, leave persistent change; DV to destroy = Virus Check (p201) | Interface + 1d10 vs GM-set DV |
| Zap | NET Action | attack a Program or Netrunner; 1d6 to Program REZ or direct to brain (p202) | Zap Check vs Program DEF + 1d10 or Netrunner Interface + 1d10 |

Count: 9 named abilities (Backdoor, Cloak, Control, Eye-Dee, Pathfinder, Scanner, Slide, Virus, Zap). Command: `awk '/^  (Backdoor|Cloak|Control|Eye-Dee|Pathfinder|Scanner|Slide|Virus|Zap) /{n++}END{print n}' $P/p148.txt` (expect 9). Plus 4 non-ability NET Actions on p199 (Jack In/Out, Use Interface Ability, Activate/Deactivate Program, Miscellaneous).

NET Actions per turn (p198, p148): Interface 1-3 = 2, 4-6 = 3, 7-9 = 4, 10 = 5. "On your Turn, you can take either a Meat Action or take as many NET Actions as your Interface ... allows." Move Action is kept either way. Moving in the Architecture and saving a File copy are not NET Actions (p199).

## 2. Programs (27 rows non-Demon + 3 Demons)

Class, ATK, DEF, REZ for non-Black ICE; PER, SPD, ATK, DEF, REZ for Black ICE. Costs in eb with price category. "-" = column not printed for that class.

### Boosters (4) - p204 (dup p369)
| Name | ATK | DEF | REZ | Effect | Cost |
|---|---|---|---|---|---|
| Eraser | 0 | 0 | 7 | +2 Cloak Checks while Rezzed | 20 (Everyday) |
| See Ya | 0 | 0 | 7 | +2 Pathfinder Checks | 20 (Everyday) |
| Speedy Gonzalvez | 0 | 0 | 7 | +2 Speed | 100 (Premium) |
| Worm | 0 | 0 | 7 | +2 Backdoor Checks | 50 (Costly) |

### Defenders (3) - p204 (dup p370)
| Name | ATK | DEF | REZ | Effect | Cost |
|---|---|---|---|---|---|
| Armor | 0 | 0 | 7 | "Lowers all brain damage you would receive by 4"; 1 copy running at a time; each copy once per Netrun | 50 (Costly) |
| Flak | 0 | 0 | 7 | Non-Black ICE Attacker ATK against you becomes 0; 1 copy at a time; once per Netrun | 50 (Costly) |
| Shield | 0 | 0 | 7 | Stops first successful non-Black ICE brain damage then Derezzes itself; 1 copy at a time; once per Netrun | 20 (Everyday) |

### Attackers (8) - p204-p205 (dup p370-p371)
| Name | Class | ATK | DEF | REZ | Effect | Cost |
|---|---|---|---|---|---|---|
| Banhammer | Anti-Program | 1 | 0 | 0 | 3d6 REZ to non-Black ICE, 2d6 to Black ICE | 50 (Costly) |
| Sword | Anti-Program | 1 | 0 | 0 | 3d6 REZ to Black ICE, 2d6 to non-Black ICE | 50 (Costly) |
| DeckKRASH | Anti-Personnel | 0 | 0 | 0 | forced unsafe Jack Out, suffer Rezzed Black ICE | 100 (Premium) |
| Hellbolt | Anti-Personnel | 2 | 0 | 0 | 2d6 direct to brain; deck+clothes catch fire (2 HP/turn until Meat Action) unless insulated | 100 (Premium) |
| Nervescrub | Anti-Personnel | 0 | 0 | 0 | INT, REF, DEX each -1d6 for 1 hour (min 1) | 100 (Premium) |
| Poison Flatline | Anti-Personnel | 0 | 0 | 0 | destroys one random non-Black ICE Program on target deck | 100 (Premium) |
| Superglue | Anti-Personnel | 2 | 0 | 0 | target cannot go deeper or Jack Out safely 1d6 Rounds; once per Netrun per copy | 100 (Premium) |
| Vrizzbolt | Anti-Personnel | 1 | 0 | 0 | 1d6 direct to brain; target NET Actions next Turn -1 (min 2) | 50 (Costly) |

### Black ICE (12) - p207-p208 (dup p371-p372). Occupy 2 slots in a deck (p204, p371 side note).
| Name | Class | PER | SPD | ATK | DEF | REZ | Effect | Cost |
|---|---|---|---|---|---|---|---|---|
| Asp | Anti-Personnel | 4 | 6 | 2 | 2 | 15 | destroys one random Program on target deck | 100 (Premium) |
| Giant | Anti-Personnel | 2 | 2 | 8 | 4 | 25 | 3d6 direct to brain; forced unsafe Jack Out | 1,000 (V. Expensive) |
| Hellhound | Anti-Personnel | 6 | 6 | 6 | 2 | 20 | 2d6 direct to brain; fire effect 2 HP/turn | 500 (Expensive) |
| Kraken | Anti-Personnel | 6 | 2 | 8 | 4 | 30 | 3d6 direct to brain; cannot go deeper/Jack Out safely until end of next Turn | 1,000 (V. Expensive) |
| Liche | Anti-Personnel | 8 | 2 | 6 | 2 | 25 | INT, REF, DEX -1d6 for 1 hour (min 1) | 500 (Expensive) |
| Raven | Anti-Personnel | 6 | 4 | 4 | 2 | 15 | Derezzes a random Rezzed Defender, then 1d6 to brain | 50 (Costly) |
| Scorpion | Anti-Personnel | 2 | 6 | 2 | 2 | 15 | MOVE -1d6 for 1 hour (min 1) | 100 (Premium) |
| Skunk | Anti-Personnel | 2 | 4 | 4 | 2 | 10 | Slide Checks -2 until Skunk Derezzed; stacks across Skunks | 500 (Expensive) |
| Wisp | Anti-Personnel | 4 | 4 | 4 | 2 | 15 | 1d6 direct to brain; NET Actions next Turn -1 (min 2) | 50 (Costly) |
| Dragon | Anti-Program | 6 | 4 | 6 | 6 | 30 | 6d6 to a Program; Derezz becomes Destroy | 1,000 (V. Expensive) |
| Killer | Anti-Program | 4 | 8 | 6 | 2 | 20 | 4d6 to a Program; Derezz becomes Destroy | 500 (Expensive) |
| Sabertooth | Anti-Program | 8 | 6 | 6 | 2 | 25 | 6d6 to a Program; Derezz becomes Destroy | 1,000 (V. Expensive) |

### Demons (3) - p213 (dup p373). Not loadable: "too big for Cyberdecks" (p206). No SPD, PER, DEF.
| Name | REZ | Interface | NET Actions | Combat Number | Cost (p373) |
|---|---|---|---|---|---|
| Imp | 15 | 3 | 2 | 14 | 1,000 (V. Expensive) |
| Efreet | 25 | 4 | 3 | 14 | 5,000 (Luxury) |
| Balron | 30 | 7 | 4 | 14 | 10,000 (Super Luxury) |

Note: p213 itself prints no Demon cost; costs are on p219 (clean table) and p373. Demon Combat Number is 14 for all three; Demon "Interface" is the NET Actions stat equivalent.

Counts (commands):
- Program rows with an Icon line (p204,p205,p207,p208): `cat $P/p204.txt $P/p205.txt $P/p207.txt $P/p208.txt | grep -c "Icon: [A-Z]"` prints 28 and includes one "How to read" legend line, so 27 programs. Same data duplicate: `cat $P/p369.txt $P/p370.txt $P/p371.txt $P/p372.txt | awk '/Icon:/{n++}END{print n}'` prints 27 (no legend line there).
- By class (hand tally, verified against the two outputs above): Boosters 4 + Defenders 3 + Attackers 8 + Black ICE 12 = 27.
- Black ICE name lines: `cat $P/p207.txt $P/p208.txt | awk '/^ *(Asp|Giant|Hellhound|Kraken|Liche|Raven|Scorpion|Skunk|Wisp|Dragon|Killer|Sabertooth) /{n++}END{print n}'` prints 12.
- Demons: `awk '/Icon:/{n++}END{print n}' $P/p213.txt` prints 3.

Program rules to carry (p202): each loaded Program Activated once per Meatspace Round; Rezzed program cannot be Activated again until Deactivated; multiple copies stack; Install/Uninstall takes an hour. Derezzed at 0 REZ; needs Deactivate + Activate (2 NET Actions) to restore; Destroyed = erased. Black ICE: your own can be loaded (2 slots) and activated lying-in-wait or in combat (p204-p205).

## 3. Cyberdeck hardware and upgrades

### Cyberdecks (3) - p197 table; dup gear p352 (price), p354 (slot text)
| Type | Cost | Slots |
|---|---|---|
| Poor Quality Cyberdeck | 100 (Premium) | 5 |
| Cyberdeck (Standard Quality, p354) | 500 (Expensive) | 7 |
| Excellent Quality Cyberdeck | 1,000 (V. Expensive) | 9 |

The p197 table is a two-line-per-row layout; row-to-cost-to-slot alignment is read from layout, and confirmed by p352 (prices) and p354 (5 / 7 / 9 slots). The p197 row for the middle deck prints only "Cyberdeck" (no "Standard"). Slots hold Programs and Hardware alike: "Both Programs and Hardware take up the same limited slots in your Cyberdeck" (p197). Black ICE = 2 slots; all other Programs 1 slot (p371 margin note). All Hardware takes 1 slot "unless otherwise noted" (p209). A Netrunner "can only be plugged into one Cyberdeck at a time. Switching Cyberdecks requires a Meat Action" (p197 margin). Required: Neural Link + Interface Plugs (p197, p354, p360). Sample character deck: "Cyberdeck (7 slots: Sword, Sword, Killer, Worm, Worm, Armor)" (p158); starting package lists "Banhammer, DeckKRASH, Eraser, Hellbolt, Shield, Sword, Worm" (p415).

### Misc upgrades (2) - p209
| Name | Effect | Cost |
|---|---|---|
| Bodyweight Suit | SP11 head and body; "Adds 1 extra Hardware only Option Slot" (p209); hardware in it only takes 1 slot and is unusable if armor not worn (p351) | 1,000 (V. Expensive) |
| Cyberdeck Cyberarm Option | cyberdeck in a Cyberarm; "Adds 1 extra Program/Hardware Slot" (p209, p365); Clinic; takes 3 Option Slots of the Cyberarm; Humanity Loss 1d6 (p365) | 500 (Expensive) |

### Hardware (7) - p209 (dup p369)
| Name | Effect | Slots | Cost |
|---|---|---|---|
| Backup Drive | saves Destroyed non-Black ICE Attacker/Defender/Booster Programs; Meat Action to reinstall | 2 | 100 (Premium) |
| DNA Lock | biometric lock; else DV17 Electronics/Security Tech | 2 | 100 (Premium) |
| Hardened Circuitry | immune to EMP and non-Black ICE Program disable/destroy effects | 1 (default) | 100 (Premium) |
| Insulated Wiring | cannot catch fire from Program effect | 1 (default) | 100 (Premium) |
| KRASH Barrier | immune to forced Jack Out, safe or unsafe | 2 | 100 (Premium) |
| Range Upgrade | connect to access point from up to 8 m (default 6 m/yds) | 1 (default) | 100 (Premium) |

That is 6 Hardware rows; with 2 misc upgrades the section lists 8 items. Command: `cat $P/p209.txt | awk '/^ +(Backup Drive|DNA Lock|Hardened|Insulated|KRASH Barrier|Range)/{n++}END{print n}'` (expect 6). The Netrunner's "deck-slot rule" in one line: total slots = deck quality (5/7/9) + Bodyweight Suit hardware-only slot (+1) + Cyberarm deck (+1, any); each Program 1 slot, Black ICE 2 slots, Hardware 1 or 2.

Not found / UNCLEAR: whether a Bodyweight Suit slot and Cyberarm slot can both apply to one deck (nothing prohibits it; p351 and p365 are the only statements). Interface Plugs (500 eb, 7 Humanity Loss 2d6, p360) and Neural Link are cyberware, not in this domain.

## 4. NET combat and brain damage (quotes)

- Stat: "Interface is the Netrunner Role Ability. Without it, you cannot Netrun." (p198). "The higher a Netrunner's Interface Rank, the more NET Actions they can take on their Turn." (p198)
- Attack formula (p202): "Your Interface + Program ATK + 1d10 or Black ICE ATK + 1d10 vs. Target's Interface + 1d10 or Program DEF/Black ICE DEF + 1d10. On a Hit, the Target suffers the Effect of the Program."
- Encounter check (p205): "you roll your Interface + any SPEED bonus you have active + 1d10 vs. the Black ICE's SPEED + 1d10. If it beats your Check, you ... suffer its effect immediately." Black ICE then enters the Initiative Queue "at the top, one number above the entity with the previously highest Initiative."
- Speed check source: Speedy Gonzalvez "Increases your Speed by +2" (p204). Perception: PER = "How hard the Black ICE is to Slide away from" (p207); Slide compares Interface to PER (p201).
- Brain damage: Zap deals "1d6 damage ... directly to the Netrunner's brain" (p202). Attackers/Black ICE: "damage direct to ... brain" (Hellbolt 2d6, Vrizzbolt 1d6, Giant 3d6, Hellhound 2d6, Kraken 3d6, Raven 1d6, Wisp 1d6). Armor program: "Lowers all brain damage you would receive by 4". Shield: stops the first non-Black ICE brain damage. UNCLEAR: the NET chapter pages (p196-p219) do not state what "brain damage" is applied against (Hit Points presumably) or the effect of reaching 0. `grep -n -i "brain" $P/p19[6-9].txt $P/p2[01]*.txt` returns only the lines above. The wider book may state it elsewhere (not searched beyond this domain).
- Jack in/out (p199): "Using a NET Action, you can Jack In to a NET Architecture while within range (within 6 m/yds, blocked by walls) of one of its access points." "Moving out of the access point's range while jacked in ... jacks you out ... but leaves you vulnerable: You suffer the effect of all remaining enemy Black ICE you've encountered, but not Derezzed." "Jacking Out 'resets' the defenses of a NET Architecture." "All your Programs leave the Architecture with you when you Jack Out."
- Black ICE targeting (p205-p206): Anti-Program Black ICE hit a random active Program and use Program DEF + 1d10 as the target number.
- Derezz (p202): 0 REZ; stays "running"; 2 NET Actions to reset.

## 5. NET Architecture generation (GM content, print-only)

p210-p212. Command refs: `awk '/All Other Floors/{s=1} s && $1 ~ /^[0-9]+$/ && $1>=3 && $1<=18 {n++} END{print n}' $P/p212.txt` prints 16.

Step 0, Difficulty table (p211): 4 columns x 3 rows.
| Row | Basic | Standard | Uncommon | Advanced |
|---|---|---|---|---|
| Password/File/Control Node DV | 6 | 8 | 10 | 12 |
| Interface with a fighting chance | 2 | 4 | 6 | 8 |
| Interface who might die | N/A | 2 or less | 4 or less | 6 or less |

Step 1 (p211): floors = 3d6. Roll d10, 7+ = a branch; repeat until a roll under 7. "A NET Architecture cannot branch until after the second floor in the main branch." A longest branch is the bottom (Virus site).
Step 2 (p211): first two floors use Lobby table; remaining floors use Body table for the difficulty. Re-roll Program/Password entries already rolled (that phrase appears twice on p211).
Step 3 (p211-p212): explain Files/Control Nodes; Demon optional; "feel free to replace any Program ... with another Program of the same Cost"; optional Boss Floor from one difficulty higher.

Lobby table (p212), 6 rows, roll column prints 1-6 (die type not printed: UNCLEAR, d6 implied): 1 File DV6, 2 Password DV6, 3 Password DV8, 4 Skunk, 5 Wisp, 6 Killer.
Command: `awk '/Lobby Floor/{s=1} /All Other Floors/{s=0} s && $1 ~ /^[1-6]$/ {n++} END{print n}' $P/p212.txt` (expect 6).

Body table (p212), 3d6 rows 3-18 = 16 rows x 4 difficulty columns = 64 cells:
| 3d6 | Basic | Standard | Uncommon | Advanced |
|---|---|---|---|---|
| 3 | Hellhound | Hellhound x2 | Kraken | Hellhound x3 |
| 4 | Sabertooth | Hellhound, Killer | Hellhound, Scorpion | Asp x2 |
| 5 | Raven x2 | Skunk x2 | Hellhound, Killer | Hellhound, Liche |
| 6 | Hellhound | Sabertooth | Raven x2 | Wisp x3 |
| 7 | Wisp | Scorpion | Sabertooth | Hellhound, Sabertooth |
| 8 | Raven | Hellhound | Hellhound | Kraken |
| 9 | Password DV6 | Password DV8 | Password DV10 | Password DV12 |
| 10 | File DV6 | File DV8 | File DV10 | File DV12 |
| 11 | Control Node DV6 | Control Node DV8 | Control Node DV10 | Control Node DV12 |
| 12 | Password DV6 | Password DV8 | Password DV10 | Password DV12 |
| 13 | Skunk | Asp | Killer | Giant |
| 14 | Asp | Killer | Liche | Dragon |
| 15 | Scorpion | Liche | Dragon | Killer, Scorpion |
| 16 | Killer, Skunk | Asp | Asp, Raven | Kraken |
| 17 | Wisp x3 | Raven x3 | Dragon, Wisp | Raven, Wisp, Hellhound |
| 18 | Liche | Liche, Raven | Giant | Dragon x2 |

The Body table has no printed "floor count" or "Black ICE per floor" column beyond the cell entries (those cells are the Black ICE per floor). Example architecture (p210): 7 floors, DVs 6/8/10, with Asp (5) and Hellhound (7).

Buying an Architecture / Home Security (p218-p219, dup p373):
| Floors | Max Control Nodes | Portable | Cost per floor |
|---|---|---|---|
| 3-6 | 2 | Yes | 1,000 |
| 7-12 | 3 | No | 5,000 |
| 13-18 | N/A | No | 10,000 |
DV feature price (Password/Control Node/File): DV6 500, DV8 1,000, DV10 5,000, DV12 10,000 (p218). Black ICE priced as for decks; 2 on a floor = double each, 3 = triple each (p219). Demons: one per six floors, no doubles (p219). Defense price by counter-DV: DV9 500, DV13 1,000, DV17 5,000, DV21 10,000 (p219). Fixer Rank 4+ Night Market with Personal Electronics (p218). Worked total: 13,500 eb (p219).

Defenses (p213-p217): Active 5 (Air Swarm Drone Cloud, Ground Drone, Large Air Drone, Mini Air Drone, Spider Walking Drone; p214), Emplaced 3 (Automated Blood Swarm, Automated Melee Weapon, Automated Turret; p215), Environmental 10 (Observation Cameras, Tanglefoot Flooring, Electrical Flooring, Laser Grid, Tip-floor, Goop, Ceiling/Wall Punchers on p216; Slip-floor, Stun Panels, Sleep Gas Elevator on p217). Total 18. Cross-check command: `cat $P/p214.txt $P/p215.txt $P/p216.txt $P/p217.txt | grep -c "Electronics/Security Tech, [0-9]* \?min"` prints 18 (the Spider drone line prints "5min" with no space, so a pattern requiring the space prints 17).

## 6. Sheet content vs GM/scene content

Sheet content (a Netrunner player character's sheet):
- Interface Role Ability rank (p148, p198) and derived NET Actions per turn (2/3/4/5 by rank band). Derived value: lookup from rank.
- Cyberdeck: quality, total slots (5/7/9, plus Bodyweight Suit hardware slot +1, Cyberarm +1), cost (p197, p209, p365).
- Loaded Programs and Hardware list with slot use (Program 1, Black ICE 2, some Hardware 2); slots used vs available is the only sheet-total feeding computation here.
- Per-program printed stats (ATK/DEF/REZ, PER/SPD for Black ICE, effect text) can be printed as text on the program list (rule: print text, do not simulate).
- Neural Link, Interface Plugs, Virtuality Goggles listed as cyberware/gear (p197).
- Interface Ability names with their formula "Interface + 1d10 vs DV" (print rule, not computed).
- Equipped Booster modifiers are print-text: +2 Cloak/Pathfinder/Backdoor/Speed.
- Armor program "-4 brain damage" is print-text.

GM/scene content (not sheet): Architecture generation (Steps 0-3, Lobby/Body tables), Demons and Defenses, Home Security prices, Virus examples, example Architectures. Inventoried for completeness; print-only per standing rule.

Per the standing rule (only sheet-total-feeding values get computed): NET Actions per turn and deck slots used/available are the two candidates. Nothing in the NET combat resolution feeds a sheet total.

## 7. Counts (with commands)

| Item | Count | Command |
|---|---|---|
| Interface Abilities | 9 | see section 1 |
| Boosters | 4 | `cat $P/p369.txt \| awk '/Booster +0 +0/{n++}END{print n}'` (expect 4) |
| Defenders | 3 | hand count from p370 |
| Attackers | 8 | hand count from p370-p371 |
| Black ICE | 12 | section 2 |
| Non-Demon Programs | 27 | `cat $P/p369.txt $P/p370.txt $P/p371.txt $P/p372.txt \| awk '/Icon:/{n++}END{print n}'` printed 27 (includes the Booster icons on p369) |
| Demons | 3 | `awk '/Icon:/{n++}END{print n}' $P/p213.txt` printed 3 |
| Cyberdecks | 3 | prices at p352 |
| Hardware | 6 | section 3 |
| Misc upgrades | 2 | section 3 |
| Lobby rows | 6 | section 5 |
| Body table rows | 16 | section 5 |
| Body table cells | 64 | 16 x 4 (arithmetic) |
| Defense rows | 18 | section 5 (grep -c cross-check) |

Counts marked "hand count" were done by reading the page text; the two awk icon counts (27 and 3) agree with the hand tally. No independent second implementation was run for the Defender, Attacker, Hardware, and Defense counts, so treat those as hand counts.

## 8. Text-extraction quality

- p197: cyberdeck price/slot table is a staggered 2-line-per-row layout, with the "(Premium)/(Expensive)/(V. Expensive)" labels offset from the eb values. Mapping confirmed against p352 and p354.
- p198, p199, p200, p201: two-column prose and sidebar interleaving. Sidebars (Example, Virus Examples) are interleaved with body text. Rank table (p198) is clean.
- p204-p205, p207-p208: tables are readable but multi-line; rows start with name and class on separate lines. Cost column and price category split across lines. The Black ICE table repeats the same data cleanly on p371-p372 (header "Black ICE takes up 2 Option Slots" appears only on p371).
- p205 Black ICE intro and p206: the Slide/Encounter text is two-column; column order is mixed but readable.
- p207: sidebar quote ("The Black ICE I hate most?") is spaced oddly; not data.
- p209: Hardware descriptions wrap in cells; costs readable.
- p210: example-architecture table has large whitespace columns; the heading "Building a NET Architecture" is duplicated with letter spacing ("B u i l d i n g ...").
- p212: Body table clean but whitespace-aligned; verified 16 rows x 4 columns by awk.
- p213, p214, p215, p216, p217: Defense tables have cells split across many lines and the "Data" column values are separated from the type name. Verify rows by hand against the PDF before any use.
- p218-p219: tables clean; sidebar margin text interleaved.
- No page was blank or image-only.
