# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Ankheg`
- expected on screen: `Ankheg`
- expected on screen: `Magical Beast`
- agent: `sd29-monster-r10` · date: 2026-08-13T00:02:52Z
- HEAD: `0b4b3703`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E5-F2-009/item8/bestiary-1-ankheg-sd22-half.png`
- rendered lines containing the record/expectations:
```
20:Magical Beast (114)
30:AnkhegLarge Magical Beast
34:This monster's row names the attack with `ABILITY:Internal|AUTOMATIC|Bite` and supplies no dice at any hop, so the dice are grounded from the published Bestiary 1 text ("bite +5 (2d6+4 plus 1d4 acid and grab)"), corroborated against https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Ankheg and https://www.d20pfsrd.com/bestiary/monster-listings/magical-beasts/ankheg/ on 2026-07-29.
```
