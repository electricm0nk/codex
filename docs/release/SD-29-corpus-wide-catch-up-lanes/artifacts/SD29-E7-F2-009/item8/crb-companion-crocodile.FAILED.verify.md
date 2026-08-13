# item-8 on-screen verification — FAILED

- verdict: **FAILED** — record is rendered but expected value(s) missing from screen: 'Core Rulebook' 
- family: `companion` · record: `Companion (Crocodile (Alligator))`
- expected on screen: `Core Rulebook`
- expected on screen: `Tail Slap`
- expected on screen: `p.56`
- agent: `sd29-companion-r12` · date: 2026-08-13T03:09:34Z
- HEAD: `736dfd3d`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Companion Catalog
Back
Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2, Bestiary 1, Bestiary 3, Bestiary 4, Ultimate Wilderness, Core Essentials and CRB — 431 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.

All books (431)
Inner Sea Combat (4)
Monster Codex (8)
Inner Sea Intrigue (2)
Horror Adventures (1)
Bestiary 5 (33)
Bestiary 6 (14)
Bestiary 2 (15)
Bestiary 1 (24)
Bestiary 3 (31)
Bestiary 4 (34)
Ultimate Wilderness (169)
Core Essentials (58)
CRB (38)

1 matching companion.

Companion (Crocodile (Alligator))Small Companion
CRB p.56
Walk 20 ft., swim 30 ft. · reach 5 ft. · Hit dice Companion:2 · Natural armor +4
Ability score adjustments (corpus BONUS:STAT tokens): STR +4, DEX +4, CON +4, INT -10, WIS +2, CHA -8
Attacks: Bite
Hold Breath — SpecialQuality · Extraordinaryp.51
A crocodile can hold its breath for rounds before it risks drowning.
Companion Advancement (Crocodile (Alligator)) — CompanionAdvancement
The corpus row states this ability’s name and type but carries no rules text.
Death Roll — SpecialAttack · Extraordinaryp.51
When grappling a foe of its size or smaller, a crocodile can perform a death roll upon making a successful grapple check. As it clings to its foe, it tucks in its legs and rolls rapidly, twisting and wrenching its victim. The crocodile inflicts its bite damage and knocks the creature prone. If successful, the crocodile maintains its grapple.
Sprint — SpecialQuality · Extraordinaryp.51
Once per minute a crocodile may sprint, increasing its land speed to 40 feet for 1 round.
Tail Slap — NaturalAttack · NaturalAttackSecondary · Secondaryp.301
The corpus row states this ability’s name and type but carries no rules text.```
