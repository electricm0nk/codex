# item-8 on-screen verification — FAILED

- verdict: **FAILED** — search for 'Dwarf' still shows 13 rows — filter did not apply (search click missed the box) or the query is too broad; the record cannot be proven in the screenshot viewport
- family: `race_trait` · record: `Dwarf`
- expected on screen: `Barrow Warden`
- expected on screen: `dodge bonus to their AC against undead`
- expected on screen: `Sense Aberration`
- agent: `sd29-racetrait-r3` · date: 2026-08-12T12:28:52Z
- HEAD: `a0bc4fc9`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Race Traits
Back
Standard traits
Alternate racial traits
Every real corpus-grounded racial trait the engine knows about — 173 trait rows across 18 races. Not what any one character has selected.

All (173)
Aasimar (9)
Drow (13)
Duergar (10)
Dwarf (12)
Elf (9)
Gnome (12)
Goblin (7)
Half-Elf (10)
Half-Orc (9)
Halfling (9)
Hobgoblin (7)
Human (6)
Kobold (9)
Merfolk (9)
Orc (9)
Svirfneblin (13)
Tengu (10)
Tiefling (10)

13 matching rows.

DwarfDuergar
Duergar are humanoids with the dwarf subtype.
+2 Constitution, +2 Wisdom, -2 CharismaDwarf
Dwarves are both tough and wise, but also a bit gruff.
DarkvisionDwarf
+60
Dwarves can see in the dark up to 60 feet.
Defensive TrainingDwarf
+4
Dwarves get a +4 dodge bonus to AC against monsters of the giant subtype.
GreedDwarf
```
