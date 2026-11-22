# item-8 on-screen verification — FAILED

- verdict: **FAILED** — no 'N matching' counter in rendered text — either the search matched NOTHING (the screen swaps the counter for a 'No … match.' notice; record 'Zzyzx Nonexistent' is absent) or the filter never applied (search-box coordinates drifted)
- family: `monster` · record: `Zzyzx Nonexistent`
- expected on screen: `CR 99`
- agent: `item8-harness` · date: 2026-08-11T22:32:09Z
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

No monsters match.

```
