# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Companion (Spitting Cobra)`
- expected on screen: `effect blindness`
- expected on screen: `companion advancement 1 or higher`
- expected on screen: `Ultimate Wilderness`
- agent: `sd29-companion-r10` · date: 2026-08-13T00:06:13Z
- HEAD: `e478cd15`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-007/item8/uw-companion-spitting-cobra.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2, Bestiary 1, Bestiary 3, Bestiary 4 and Ultimate Wilderness — 335 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
17:Ultimate Wilderness (169)
21:Companion (Spitting Cobra)Small Companion
22:Ultimate Wilderness p.182
35:companion advancement 1 or higher: Spit; frequency 1 round [6]; effect blindness 1 round; cure 1 save; Fort DC
```
