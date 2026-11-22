# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Calikang`
- expected on screen: `Calikang`
- expected on screen: `Inner Sea World Guide`
- expected on screen: `Suspend Animation`
- agent: `sd29-monster-r5` · date: 2026-08-12T15:42:54Z
- HEAD: `37dba464`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E5-F2-004/item8/iswg-calikang.png`
- rendered lines containing the record/expectations:
```
4:Every real stat block the engine knows about, across Bestiary 1, Bonus Bestiary, Monster Codex, Book of the Damned, Volume 1, Book of the Damned, Volume 2 and Inner Sea World Guide — 80 monsters. Armor Class, hit points and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. Bestiary 1’s rows carry the land speed only; the other books’ carry every movement mode and their special abilities.
30:CalikangLarge Monstrous Humanoid
32:Speed 30 ft. · Inner Sea World Guide p.307 · Hit dice Monstrous Humanoid:15
40:-ft. line, d6 energy damage, Reflex DC half, usable /day. A calikang can choose what kind of energy damage its breath weapon inflicts when it uses this ability. Calikangs are particularly adept at using electricity in this manner, and inflict additional points of damage when they elect to inflict electricity damage with their breath weapon.
47:Suspend Animation — Special Quality (Su)p.308
```
