# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Kyton, Augur`
- expected on screen: `Unnerving Gaze`
- expected on screen: `Bestiary 3`
- agent: `sd29-companion-r8` · date: 2026-08-12T21:13:33Z
- HEAD: `9905926b`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-005/item8/b3-companion-kyton-augur.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2, Bestiary 1 and Bestiary 3 — 132 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
15:Bestiary 3 (31)
19:Kyton, AugurTiny Outsider (Evil, Extraplanar, Kyton, Lawful)
20:Bestiary 3 p.170
24:Unnerving Gaze — SpecialQuality · Extraordinaryp.171
```
