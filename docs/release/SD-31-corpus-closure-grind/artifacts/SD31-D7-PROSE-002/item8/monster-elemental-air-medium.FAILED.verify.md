# item-8 on-screen verification — FAILED

- verdict: **FAILED** — search for 'Elemental (Air/Medium)' still shows 44 rows — filter did not apply (search click missed the box) or the query is too broad; the record cannot be proven in the screenshot viewport
- family: `monster` · record: `Elemental (Air/Medium)`
- expected on screen: `Air Mastery`
- expected on screen: `Airborne creatures take a -1 penalty on attack and damage rolls against an air elemental`
- agent: `sd31-prose-payout` · date: 2026-08-16T08:03:57Z
- HEAD: `5d0cd1595`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Monster Catalog
Back
Every real stat block the engine knows about, across Bestiary 1, Bonus Bestiary, Monster Codex, Book of the Damned, Volume 1, Book of the Damned, Volume 2, Inner Sea World Guide, Bestiary 2, Bestiary 3, Bestiary 4, Inner Sea Bestiary, Inner Sea Gods, Ultimate Psionics and HA — 1242 monsters. Armor Class, hit points and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. Bestiary 1’s rows carry the land speed only; the other books’ carry every movement mode and their special abilities.

All sizes (1242)
Diminutive (16)
Tiny (78)
Small (128)
Medium (417)
Large (334)
Huge (158)
All types (1242)
Aberration (83)
Animal (147)
Construct (78)
Dragon (56)
Fey (53)
Humanoid (62)
Magical Beast (114)
Monstrous Humanoid (67)
Ooze (28)
Outsider (321)
Plant (43)
Undead (81)
Vermin (109)

44 matching monsters.

Elemental (Ice/Small)Small Outsider (Air, Cold, Elemental, Extraplanar, Water)
CR 1
Speed 20 ft., swim 60 ft. · Bestiary 2 p.114 · Hit dice Outsider (Fort/Ref):2
Slam 1d4(corpus row)
Also has, defined in another book: Can't Be Tripped, Ice Elemental ~ Burrow, Ice Elemental ~ Cold, Ice Elemental ~ Ice Glide, Ice Elemental ~ Icewalking, Ice Elemental ~ Numbing Cold, Ice Elemental ~ Snow Vision.
Elemental (Ice/Medium)Medium Outsider (Air, Cold, Elemental, Extraplanar, Water)
CR 3
Speed 20 ft., swim 60 ft. · Bestiary 2 p.114 · Hit dice Outsider (Fort/Ref):4
Slam 1d6(corpus row)
Also has, defined in another book: Can't Be Tripped, Ice Elemental ~ Burrow, Ice Elemental ~ Cold, Ice Elemental ~ Ice Glide, Ice Elemental ~ Icewalking, Ice Elemental ~ Numbing Cold, Ice Elemental ~ Snow Vision.
Elemental (Ice/Large)Large Outsider (Air, Cold, Elemental, Extraplanar, Water)
```
