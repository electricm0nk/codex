# item-8 on-screen verification — FAILED

- verdict: **FAILED** — no 'N matching' counter in rendered text — either the search matched NOTHING (the screen swaps the counter for a 'No … match.' notice; record 'Akashic Form' is absent) or the filter never applied (search-box coordinates drifted)
- family: `spell` · record: `Akashic Form`
- expected on screen: `Akashic Form`
- expected on screen: `Akashic Record`
- agent: `sd31-e6-f2-005` · date: 2026-08-16T12:28:26Z
- HEAD: `b8c36417d`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Create a character
Back
Character name

Player name

Race

Class

Fighter is fully computed at every level offered here.

Level

HP
13
Alignment

Deity

Physical Attributes
Size
Medium
Sex

Vision
Darkvision 60 ft.
Height
4'4"
🎲
Weight
171 lb
🎲
Age

Eyes

Hair

```
