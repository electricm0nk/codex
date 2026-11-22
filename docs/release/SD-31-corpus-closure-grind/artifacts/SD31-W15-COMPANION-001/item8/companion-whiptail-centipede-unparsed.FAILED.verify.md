# item-8 on-screen verification — FAILED

- verdict: **FAILED** — search for 'Companion (Whiptail Centipede (Giant))' still shows 196 rows — filter did not apply (search click missed the box) or the query is too broad; the record cannot be proven in the screenshot viewport
- family: `companion` · record: `Companion (Whiptail Centipede (Giant))`
- expected on screen: `formula not interpreted`
- expected on screen: `PREVARLT:MasterLevel,7`
- agent: `sd31-w15-companion` · date: 2026-08-19T12:37:33Z
- HEAD: `3d75e9786`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Companion Catalog
Back
Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2, Bestiary 1, Bestiary 3, Bestiary 4, Ultimate Wilderness, Core Rulebook, Ultimate Magic, Advanced Race Guide, Advanced Player's Guide and Book of the Damned, Volume 1 — 450 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.

All books (450)
Inner Sea Combat (4)
Monster Codex (8)
Inner Sea Intrigue (2)
Horror Adventures (1)
Bestiary 5 (33)
Bestiary 6 (14)
Bestiary 2 (15)
Bestiary 1 (55)
Bestiary 3 (31)
Bestiary 4 (34)
Ultimate Wilderness (169)
Core Rulebook (38)
Ultimate Magic (29)
Advanced Race Guide (7)
Advanced Player's Guide (9)
Book of the Damned, Volume 1 (1)

196 matching companions.

Companion (Griffon)Large Magical Beast
Inner Sea Combat
Walk 30 ft., fly 40 ft. · Hit dice Companion:2 · Natural armor +4
Ability score adjustments (corpus BONUS:STAT tokens): STR +6, DEX +4, CON +6, INT -6, WIS +2, CHA -2
Attacks: Bite
Unable to carry a rider while flying — SpecialQuality
The corpus row states this ability’s name and type but carries no rules text.
Companion Advancement (Griffon) — CompanionAdvancement
The corpus row states this ability’s name and type but carries no rules text.
Ability score adjustments (corpus BONUS:STAT tokens): STR +2, CON +2
Also has, defined in another book: Scent.
Companion (Hippocampus)Large Magical Beast (Aquatic)
Inner Sea Combat
Walk 5 ft., swim 40 ft. · Hit dice Companion:2 · Natural armor +4
Ability score adjustments (corpus BONUS:STAT tokens): STR +6, DEX -2, CON +4, INT -8, WIS +2
```
