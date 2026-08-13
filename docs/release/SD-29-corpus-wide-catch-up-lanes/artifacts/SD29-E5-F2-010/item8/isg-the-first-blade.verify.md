# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `The First Blade`
- expected on screen: `The First Blade`
- expected on screen: `Inner Sea Gods`
- expected on screen: `Outsider`
- agent: `sd29-monster-r11` · date: 2026-08-13T01:38:10Z
- HEAD: `1c7d8ef9`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E5-F2-010/item8/isg-the-first-blade.png`
- rendered lines containing the record/expectations:
```
4:Every real stat block the engine knows about, across Bestiary 1, Bonus Bestiary, Monster Codex, Book of the Damned, Volume 1, Book of the Damned, Volume 2, Inner Sea World Guide, Bestiary 2, Bestiary 3, Bestiary 4, Inner Sea Bestiary and Inner Sea Gods — 1218 monsters. Armor Class, hit points and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. Bestiary 1’s rows carry the land speed only; the other books’ carry every movement mode and their special abilities.
23:Outsider (321)
30:The First BladeLarge Outsider (Chaotic, Extraplanar, Herald)
32:Speed 30 ft. · Inner Sea Gods p.288 · Hit dice Outsider (Fort/Ref):18
```
