# item-8 on-screen verification — FAILED

- verdict: **FAILED** — search for 'Companion (Griffon)' still shows 15 rows — filter did not apply (search click missed the box) or the query is too broad; the record cannot be proven in the screenshot viewport
- family: `companion` · record: `Companion (Griffon)`
- expected on screen: `Companion:2`
- expected on screen: `Magical Beast`
- expected on screen: `Fly 40 ft.`
- agent: `sd29-companion-r4` · date: 2026-08-12T13:37:07Z
- HEAD: `03acb5a5`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Companion Catalog
Back
Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue and Horror Adventures — 15 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.

All books (15)
Inner Sea Combat (4)
Monster Codex (8)
Inner Sea Intrigue (2)
Horror Adventures (1)

15 matching companions.

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
Attacks: Bite, Tail Slap
Companion Advancement (Hippocampus) — CompanionAdvancement
The corpus row states this ability’s name and type but carries no rules text.
Ability score adjustments (corpus BONUS:STAT tokens): STR +2, DEX +4
Also has, defined in another book: Hippocampus ~ Water Dependency, Scent.
Companion (Hippogriff)Large Magical Beast
Inner Sea Combat
Walk 40 ft., fly 60 ft. · Hit dice Companion:2 · Natural armor +2
Ability score adjustments (corpus BONUS:STAT tokens): STR +4, DEX +4, CON +4, INT -8, WIS +2, CHA -2
Attacks: Bite
Companion Advancement (Hippogriff) — CompanionAdvancement
The corpus row states this ability’s name and type but carries no rules text.
```
