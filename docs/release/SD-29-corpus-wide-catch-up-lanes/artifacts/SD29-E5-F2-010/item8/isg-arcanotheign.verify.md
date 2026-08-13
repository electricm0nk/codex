# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Arcanotheign`
- expected on screen: `Arcanotheign`
- expected on screen: `Inner Sea Gods`
- expected on screen: `Change Shape`
- agent: `sd29-monster-r11` · date: 2026-08-13T01:38:41Z
- HEAD: `1c7d8ef9`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E5-F2-010/item8/isg-arcanotheign.png`
- rendered lines containing the record/expectations:
```
4:Every real stat block the engine knows about, across Bestiary 1, Bonus Bestiary, Monster Codex, Book of the Damned, Volume 1, Book of the Damned, Volume 2, Inner Sea World Guide, Bestiary 2, Bestiary 3, Bestiary 4, Inner Sea Bestiary and Inner Sea Gods — 1218 monsters. Armor Class, hit points and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. Bestiary 1’s rows carry the land speed only; the other books’ carry every movement mode and their special abilities.
30:ArcanotheignMedium Outsider (Herald, Incorporeal)
32:Speed 40 ft., fly 60 ft. · Inner Sea Gods p.298 · Hit dice Outsider (Fort/Will):17
36:The Arcanotheign has resistance against the first 30 points of damage from a divine source (such as Flame Strike).
37:Change Shape — Special Quality (Su)
44:The Arcanotheign is constantly under the effects of the following spells: Arcane Sight, Protection from Chaos/Evil/Good/Law.
```
