# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Ankheg`
- expected on screen: `CR 3`
- expected on screen: `Bestiary 1 p.15`
- agent: `item8-harness` · date: 2026-08-11T22:29:56Z
- HEAD: `8b621552`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/item8-harness/monster-b1-ankheg.png`
- rendered lines containing the record/expectations:
```
30:AnkhegLarge Magical Beast
31:CR 3
32:Speed 30 ft. · Bestiary 1 p.15
34:This monster's row names the attack with `ABILITY:Internal|AUTOMATIC|Bite` and supplies no dice at any hop, so the dice are grounded from the published Bestiary 1 text ("bite +5 (2d6+4 plus 1d4 acid and grab)"), corroborated against https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Ankheg and https://www.d20pfsrd.com/bestiary/monster-listings/magical-beasts/ankheg/ on 2026-07-29.
```
