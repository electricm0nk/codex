# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Eidolon`
- expected on screen: `Advanced Player's Guide`
- expected on screen: `SkillChoice`
- agent: `sd29-companion-final-r1` · date: 2026-08-13T09:59:35Z
- HEAD: `3ce4a1d4`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-010/item8/apg-eidolon.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2, Bestiary 1, Bestiary 3, Bestiary 4, Ultimate Wilderness, Core Essentials, Core Rulebook, Ultimate Magic, Advanced Race Guide, Advanced Player's Guide and Book of the Damned, Volume 1 — 450 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
22:Advanced Player's Guide (1)
27:EidolonMedium Eidolon
28:Advanced Player's Guide p.56
29:Walk 20 ft. · reach 5 ft. · Hit dice Eidolon:1 · Natural armor +2
36:Skills — SkillChoice
```
