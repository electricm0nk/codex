# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `spell` · record: `Ablative Barrier`
- expected on screen: `+2 armor bonus`
- expected on screen: `nonlethal damage`
- agent: `sd31-spell-monster` · date: 2026-08-16T08:48:13Z
- HEAD: `eac9df2d6`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `../../docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E6-F2-004/item8/spell-ablative-barrier.png`
- rendered lines containing the record/expectations:
```
28:Ablative BarrierUCConjuration
30:Invisible layers of solid force surround and protect the target, granting that target a +2 armor bonus to AC. Additionally, the first 5 points of lethal damage the target takes from each attack are converted into nonlethal damage. Against attacks that already deal nonlethal damage, the target gains DR 5/-. Once this spell has converted 5 points of damage to nonlethal damage per caster level (maximum 50 points), the spell is discharged.
```
