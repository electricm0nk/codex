# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Companion (Leech (Giant))`
- expected on screen: `Ultimate Magic`
- expected on screen: `Blood Drain`
- expected on screen: `without Companion Advancement (Leech (Giant))`
- agent: `sd29-companion-final-r1` · date: 2026-08-13T09:53:20Z
- HEAD: `3ce4a1d4`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-010/item8/um-giant-leech.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2, Bestiary 1, Bestiary 3, Bestiary 4, Ultimate Wilderness, Core Essentials, Core Rulebook, Ultimate Magic, Advanced Race Guide, Advanced Player's Guide and Book of the Damned, Volume 1 — 450 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
20:Ultimate Magic (10)
27:Companion (Leech (Giant))Small Companion
28:Ultimate Magic p.36
34:Blood Drain — SpecialAttack · Extraordinaryp.36
36:without Companion Advancement (Leech (Giant)): inflicting 1 point of Strength damage.
```
