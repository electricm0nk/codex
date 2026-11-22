# item-8 on-screen verification — FAILED

- verdict: **FAILED** — search for 'Languages' still shows 10 rows — filter did not apply (search click missed the box) or the query is too broad; the record cannot be proven in the screenshot viewport
- family: `race_trait` · record: `Languages`
- expected on screen: `Dwarves begin play speaking Common and Dwarven`
- agent: `sd31-w5-integrate` · date: 2026-08-16T07:23:06Z
- HEAD: `248315c63`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Race Traits
Back
Standard traits
Alternate racial traits
Every real corpus-grounded racial trait the engine knows about — 239 trait rows across 25 races. Not what any one character has selected.

All (239)
Aasimar (9)
Drow (13)
Duergar (10)
Dwarf (12)
Elf (9)
Fetchling (11)
Gnome (12)
Goblin (7)
Grippli (10)
Half-Elf (10)
Half-Orc (9)
Halfling (9)
Hobgoblin (7)
Human (6)
Ifrit (9)
Kobold (9)
Merfolk (9)
Orc (9)
Oread (9)
Skinwalker (9)
Svirfneblin (13)
Sylph (9)
Tengu (10)
Tiefling (10)
Undine (9)

10 matching rows.

+2 Dexterity, +2 Intelligence, -2 CharismaTiefling
Tieflings are quick in body and mind, but are inherently unnerving.
DarkvisionTiefling
+60
```
