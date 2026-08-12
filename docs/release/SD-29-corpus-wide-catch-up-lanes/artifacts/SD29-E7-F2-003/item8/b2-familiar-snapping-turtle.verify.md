# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Snapping Turtle`
- expected on screen: `Bestiary 2`
- expected on screen: `Shell`
- expected on screen: `SpecialQuality`
- expected on screen: `armor bonus from natural armor increases by +4`
- agent: `sd29-companion-r5` · date: 2026-08-12T15:43:18Z
- HEAD: `5164bf36`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-003/item8/b2-familiar-snapping-turtle.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6 and Bestiary 2 — 77 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
13:Bestiary 2 (15)
17:Familiar (Snapping Turtle)Tiny Animal
18:Bestiary 2 p.273
22:Shell — SpecialQuality · Extraordinaryp.273
23:As a move action, a snapping turtle can pull its extremities and head into its shell. It cannot move or attack as long as it remains in this state, but its armor bonus from natural armor increases by +4 as long as it does.
```
