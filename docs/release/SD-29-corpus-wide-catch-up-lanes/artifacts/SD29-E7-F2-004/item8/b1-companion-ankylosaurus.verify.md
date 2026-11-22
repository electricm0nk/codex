# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Companion (Dinosaur (Ankylosaurus))`
- expected on screen: `Stun`
- expected on screen: `Bestiary 1`
- agent: `sd29-companion-r7` · date: 2026-08-12T18:59:27Z
- HEAD: `df829763`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-004/item8/b1-companion-ankylosaurus.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2 and Bestiary 1 — 101 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
14:Bestiary 1 (24)
18:Companion (Dinosaur (Ankylosaurus))Medium Companion (AnimalCompanionDinosaur)
19:Bestiary 1 p.83
25:Stun — SpecialAttack · Extraordinaryp.83
```
