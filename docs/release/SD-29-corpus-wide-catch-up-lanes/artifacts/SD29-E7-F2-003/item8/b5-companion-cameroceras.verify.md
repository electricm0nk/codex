# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Cameroceras`
- expected on screen: `Bestiary 5`
- expected on screen: `jet 90 ft.`
- expected on screen: `Companion Advancement (Cameroceras)`
- agent: `sd29-companion-r5` · date: 2026-08-12T15:42:37Z
- HEAD: `5164bf36`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-003/item8/b5-companion-cameroceras.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6 and Bestiary 2 — 77 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
11:Bestiary 5 (33)
17:Companion (Cameroceras)Medium Companion (Aquatic)
18:Bestiary 5 p.312
19:Walk 5 ft., swim 20 ft., jet 90 ft. · reach 5 ft. · Hit dice Companion:2 · Natural armor +1
22:Companion Advancement (Cameroceras) — CompanionAdvancement
25:Also has, defined in another book: Cameroceras ~ Pressure Adaptation, Grab.
```
