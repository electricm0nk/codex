# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Abaia`
- expected on screen: `Abaia`
- expected on screen: `Bestiary 4`
- expected on screen: `Eldritch Gizzard`
- agent: `sd29-monster-r8` · date: 2026-08-12T20:22:26Z
- HEAD: `52da4bc3`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E5-F2-007/item8/monster-abaia.png`
- rendered lines containing the record/expectations:
```
4:Every real stat block the engine knows about, across Bestiary 1, Bonus Bestiary, Monster Codex, Book of the Damned, Volume 1, Book of the Damned, Volume 2, Inner Sea World Guide, Bestiary 2, Bestiary 3 and Bestiary 4 — 861 monsters. Armor Class, hit points and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. Bestiary 1’s rows carry the land speed only; the other books’ carry every movement mode and their special abilities.
30:AbaiaHuge Magical Beast (Aquatic)
32:Speed 20 ft., swim 80 ft. · Bestiary 4 p.7 · Hit dice Magical Beast:14
35:Eldritch Gizzard — Special Quality (Su)
```
