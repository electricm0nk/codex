# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Familiar (Giant Flea)`
- expected on screen: `Uncanny Leap`
- expected on screen: `Bestiary 4`
- agent: `sd29-companion-r9` · date: 2026-08-12T22:08:45Z
- HEAD: `c9fb0fc6`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-006/item8/b4-companion-familiar-giant-flea.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2, Bestiary 1, Bestiary 3 and Bestiary 4 — 166 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
16:Bestiary 4 (34)
20:Familiar (Giant Flea)Small Vermin (Familiar, Augmented Magical Beast, FamiliarBase)
21:Bestiary 4 p.99
27:Uncanny Leap — SpecialQuality · Extraordinary
```
