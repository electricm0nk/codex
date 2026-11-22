# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Achaierai`
- expected on screen: `Achaierai`
- expected on screen: `Bestiary 2`
- expected on screen: `Black Cloud`
- agent: `sd29-monster-r6` · date: 2026-08-12T17:42:51Z
- HEAD: `595e1e87`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E5-F2-005/item8/monster-achaierai.png`
- rendered lines containing the record/expectations:
```
4:Every real stat block the engine knows about, across Bestiary 1, Bonus Bestiary, Monster Codex, Book of the Damned, Volume 1, Book of the Damned, Volume 2, Inner Sea World Guide and Bestiary 2 — 394 monsters. Armor Class, hit points and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. Bestiary 1’s rows carry the land speed only; the other books’ carry every movement mode and their special abilities.
30:AchaieraiLarge Outsider (Evil, Extraplanar, Lawful)
32:Speed 50 ft. · Bestiary 2 p.7 · Hit dice Outsider (Fort/Ref):7
35:Black Cloud — Special Attack (Su)p.7
36:An achaierai can exhale a cloud of choking, toxic smoke three times per day. All creatures within 10 feet of the achaierai immediately take 2d6 points of damage as their flesh melts and rots away. The cloud erodes sanity as well as flesh, and anyone who takes damage from the black cloud must also make a DC Fortitude save or become confused. Every round, the victim may attempt another DC Fortitude save to recover from the confusion; otherwise it persists, lasting indefinitely until the condition is removed or the victim eventually makes her saving throw. The confusion element of a black cloud is a mind-affecting effect. This is a poison effect. Achaierais are immune to this ability
```
