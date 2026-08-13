# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Companion (Crocodile (Alligator))`
- expected on screen: `Core Rulebook`
- expected on screen: `Tail Slap`
- expected on screen: `p.56`
- agent: `sd29-companion-r12` · date: 2026-08-13T03:16:28Z
- HEAD: `736dfd3d`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-009/item8/crb-companion-crocodile.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2, Bestiary 1, Bestiary 3, Bestiary 4, Ultimate Wilderness, Core Essentials and Core Rulebook — 431 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
19:Core Rulebook (38)
23:Companion (Crocodile (Alligator))Small Companion
24:Core Rulebook p.56
36:Tail Slap — NaturalAttack · NaturalAttackSecondary · Secondaryp.301
```
