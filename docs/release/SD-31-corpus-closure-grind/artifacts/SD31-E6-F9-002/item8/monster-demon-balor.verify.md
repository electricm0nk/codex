# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Demon (Balor)`
- expected on screen: `Vorpal Strike`
- expected on screen: `gains the vorpal weapon quality`
- agent: `sd31monster2` · date: 2026-08-17T01:17:24Z
- HEAD: `a9426b760`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `../../docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E6-F9-002/item8/monster-demon-balor.png`
- rendered lines containing the record/expectations:
```
30:Demon (Balor)Large Outsider (Chaotic, Demon, Evil, Extraplanar)
40:Vorpal Strike — Special Quality (Su)p.58
41:Any slashing weapon a balor wields (including its standard longsword and whip) gains the vorpal weapon quality. Weapons retain this quality for one hour after the balor releases the weapon, but after this the weapon reverts to its standard magical qualities, if any.
```
