# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Ankheg`
- expected on screen: `Magical Beast`
- expected on screen: `CR 3`
- agent: `sd31-spell-monster` · date: 2026-08-16T08:47:46Z
- HEAD: `eac9df2d6`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `../../docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E6-F2-004/item8/monster-ankheg.png`
- rendered lines containing the record/expectations:
```
20:Magical Beast (114)
30:AnkhegLarge Magical Beast
31:CR 3
34:This monster's row names the attack with `ABILITY:Internal|AUTOMATIC|Bite` and supplies no dice at any hop, so the dice are grounded from the published Bestiary 1 text ("bite +5 (2d6+4 plus 1d4 acid and grab)"), corroborated against https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Ankheg and https://www.d20pfsrd.com/bestiary/monster-listings/magical-beasts/ankheg/ on 2026-07-29.
```
