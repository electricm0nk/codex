# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Ankheg`
- expected on screen: `Acid Bite`
- expected on screen: `additional 1d4 acid damage`
- agent: `w23-monster` · date: 2026-08-20T15:25:10Z
- HEAD: `3229f9e2b`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-31-corpus-closure-grind/artifacts/SD31-W23-MONSTER-001/item8/monster-ankheg.png`
- rendered lines containing the record/expectations:
```
30:AnkhegLarge Magical Beast
34:This monster's row names the attack with `ABILITY:Internal|AUTOMATIC|Bite` and supplies no dice at any hop, so the dice are grounded from the published Bestiary 1 text ("bite +5 (2d6+4 plus 1d4 acid and grab)"), corroborated against https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Ankheg and https://www.d20pfsrd.com/bestiary/monster-listings/magical-beasts/ankheg/ on 2026-07-29.
37:Acid Bite — Special Attack (Ex)p.15
38:An Ankheg's bite does an additional 1d4 acid damage unless it has recently used it's spit acid ability.
```
