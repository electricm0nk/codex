# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Companion (Imp)`
- expected on screen: `Book of the Damned`
- expected on screen: `Poison`
- agent: `sd29-companion-final-r1` · date: 2026-08-13T09:58:07Z
- HEAD: `3ce4a1d4`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-010/item8/botd1-imp.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2, Bestiary 1, Bestiary 3, Bestiary 4, Ultimate Wilderness, Core Essentials, Core Rulebook, Ultimate Magic, Advanced Race Guide, Advanced Player's Guide and Book of the Damned, Volume 1 — 450 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
23:Book of the Damned, Volume 1 (1)
27:Companion (Imp)Tiny Outsider (Devil, Lawful, Evil)
28:Book of the Damned, Volume 1 p.78
32:Poison — SpecialAttack · Extraordinaryp.78
```
