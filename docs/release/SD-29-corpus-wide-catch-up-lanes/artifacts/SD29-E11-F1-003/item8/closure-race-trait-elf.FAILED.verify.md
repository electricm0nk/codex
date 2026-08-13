# item-8 on-screen verification — FAILED

- verdict: **FAILED** — search for 'Elf' still shows 20 rows — filter did not apply (search click missed the box) or the query is too broad; the record cannot be proven in the screenshot viewport
- family: `race_trait` · record: `Elf`
- expected on screen: `Elf (`
- agent: `sd29-closure-r3` · date: 2026-08-13T10:48:20Z
- HEAD: `267b84d1`
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

20 matching rows.

ElfDrow
Drow are humanoids with the elf subtype.
+2 Dexterity, +2 Intelligence, -2 ConstitutionElf
Elves are nimble, both in body and mind, but their form is frail.
Elven ImmunitiesElf
+2
Elves are immune to magic sleep effects and get a +2 racial saving throw bonus against enchantment spells and effects.
Elven MagicElf
+2
Elves receive a +2 racial bonus on caster level checks made to overcome spell resistance. In addition, elves receive a +2 racial bonus on Spellcraft skill checks made to identify the properties of magic items.
Keen SensesElf
```
