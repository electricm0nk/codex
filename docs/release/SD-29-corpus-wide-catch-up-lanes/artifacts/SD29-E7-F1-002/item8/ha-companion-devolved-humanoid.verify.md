# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Companion (Devolved Humanoid)`
- expected on screen: `Horror Adventures`
- expected on screen: `climb 30 ft.`
- expected on screen: `Companion Advancement (Devolved Humanoid)`
- expected on screen: `p.50`
- agent: `sd29-companion-r4` · date: 2026-08-12T13:38:52Z
- HEAD: `03acb5a5`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F1-002/item8/ha-companion-devolved-humanoid.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue and Horror Adventures — 15 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
10:Horror Adventures (1)
14:Companion (Devolved Humanoid)Medium Companion
15:Horror Adventures p.50
16:Walk 30 ft., climb 30 ft. · reach 5 ft. · Hit dice Companion:2 · Natural armor +1
19:Companion Advancement (Devolved Humanoid) — CompanionAdvancement
```
