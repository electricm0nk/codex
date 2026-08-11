# item-8 on-screen verification — FAILED

- verdict: **FAILED** — record is rendered but expected value(s) missing from screen: 'CR 17' 
- family: `monster` · record: `Ankheg`
- expected on screen: `CR 17`
- agent: `item8-harness` · date: 2026-08-11T22:32:21Z
- HEAD: `8b621552`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Monster Catalog
Back
Every real stat block the engine knows about, across Bestiary 1 and Bonus Bestiary — 60 monsters. Armor Class, hit points and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. Bestiary 1’s rows carry the land speed only; Bonus Bestiary’s carry every movement mode and their special abilities.

All sizes (60)
Diminutive (2)
Tiny (2)
Small (10)
Medium (28)
Large (17)
Huge (1)
All types (60)
Aberration (4)
Animal (14)
Construct (2)
Dragon (1)
Fey (2)
Humanoid (8)
Magical Beast (8)
Monstrous Humanoid (6)
Ooze (1)
Outsider (3)
Plant (3)
Undead (4)
Vermin (4)

1 matching monster.

AnkhegLarge Magical Beast
CR 3
Speed 30 ft. · Bestiary 1 p.15
Bite 2d6(grounded from published text)
This monster's row names the attack with `ABILITY:Internal|AUTOMATIC|Bite` and supplies no dice at any hop, so the dice are grounded from the published Bestiary 1 text ("bite +5 (2d6+4 plus 1d4 acid and grab)"), corroborated against https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Ankheg and https://www.d20pfsrd.com/bestiary/monster-listings/magical-beasts/ankheg/ on 2026-07-29.```
