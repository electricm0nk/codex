# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Adaro`
- expected on screen: `Adaro`
- expected on screen: `Bestiary 3`
- agent: `sd29-monster-r7` · date: 2026-08-12T19:26:22Z
- HEAD: `7037c1dd`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E5-F2-006/item8/monster-adaro.png`
- rendered lines containing the record/expectations:
```
4:Every real stat block the engine knows about, across Bestiary 1, Bonus Bestiary, Monster Codex, Book of the Damned, Volume 1, Book of the Damned, Volume 2, Inner Sea World Guide, Bestiary 2 and Bestiary 3 — 655 monsters. Armor Class, hit points and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. Bestiary 1’s rows carry the land speed only; the other books’ carry every movement mode and their special abilities.
30:AdaroMedium Monstrous Humanoid (Aquatic)
32:Speed 10 ft., swim 50 ft. · Bestiary 3 p.7 · Hit dice Monstrous Humanoid:4
34:This monster's row names the attack, and the Bestiary 3 corpus carries no die expression for it at any hop. No value is shown because none was ingested.
36:Adaros favor a paralytic toxin secreted by the flying nettlefin pufferfish-a sticky venom that doesn't wash away in water. Nettlefin Toxin: Spear-injury; save Fort DC 15; frequency 1/minute for 4 minutes; effect paralyzed for 1 minute; cure 2 consecutive saves.
38:Adaros are skilled in the use of poison and never risk accidentally poisoning themselves.
40:Adaros revere storms, and their lust for blood is amplified exponentially while it is raining. While fighting in the rain or during other stormy weather, adaros act as though affected by the rage spell. An adaro gains this benefit even if it is underwater, but only as long as it remains within a move action away from the water's surface (50 feet for most adaros).
```
