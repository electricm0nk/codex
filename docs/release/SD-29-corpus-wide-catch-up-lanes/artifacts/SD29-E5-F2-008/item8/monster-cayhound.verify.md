# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Cayhound`
- expected on screen: `Cayhound`
- expected on screen: `Inner Sea Bestiary`
- expected on screen: `Thunderous Bark`
- agent: `sd29-monster-r9` · date: 2026-08-12T21:58:59Z
- HEAD: `92d346a7`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E5-F2-008/item8/monster-cayhound.png`
- rendered lines containing the record/expectations:
```
4:Every real stat block the engine knows about, across Bestiary 1, Bonus Bestiary, Monster Codex, Book of the Damned, Volume 1, Book of the Damned, Volume 2, Inner Sea World Guide, Bestiary 2, Bestiary 3, Bestiary 4 and Inner Sea Bestiary — 899 monsters. Armor Class, hit points and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. Bestiary 1’s rows carry the land speed only; the other books’ carry every movement mode and their special abilities.
30:CayhoundMedium Outsider
32:Speed 40 ft. · Inner Sea Bestiary p.8 · Hit dice Outsider (Ref/Will):6
38:Thunderous Bark — Special Attack (Su)p.8
39:Once every 1d6 rounds, a Cayhound can cause every creature within a 15 foot cone to take 3d8 sonic damage and be knocked prone (Fort DC 14 half and negates knocked prone).
41:Cayhounds move as though under a continuous Freedom of Movement spell and are immune to Dimensional Anchor although Dimensional Lock functions normally.
```
