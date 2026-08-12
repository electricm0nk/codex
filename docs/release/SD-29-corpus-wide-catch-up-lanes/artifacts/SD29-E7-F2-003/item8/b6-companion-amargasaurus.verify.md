# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Amargasaurus`
- expected on screen: `Bestiary 6`
- expected on screen: `Hit dice Companion:2`
- expected on screen: `Companion Advancement (Amargasaurus)`
- agent: `sd29-companion-r5` · date: 2026-08-12T15:42:58Z
- HEAD: `5164bf36`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-003/item8/b6-companion-amargasaurus.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6 and Bestiary 2 — 77 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
12:Bestiary 6 (14)
17:Companion (Amargasaurus)Medium Companion (AnimalCompanionDinosaur)
18:Bestiary 6 p.311
19:Walk 30 ft. · reach 5 ft. · Hit dice Companion:2 · Natural armor +3
22:Companion Advancement (Amargasaurus) — CompanionAdvancement
```
