# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Companion (Yzobu)`
- expected on screen: `Stampede`
- expected on screen: `SpecialAttack`
- expected on screen: `Monster Codex`
- agent: `sd29-companion-r4` · date: 2026-08-12T13:38:33Z
- HEAD: `03acb5a5`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F1-002/item8/mc-companion-yzobu.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue and Horror Adventures — 15 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
8:Monster Codex (8)
14:Companion (Yzobu)Medium Companion
15:Monster Codex p.124
21:Stampede — SpecialAttack · Extraordinary
```
