# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Demon (Seraptis)`
- expected on screen: `Book of the Damned, Volume 2 p.58`
- expected on screen: `Will save`
- expected on screen: `Charisma drain`
- agent: `sd29-monster-r3` · date: 2026-08-12T12:14:16Z
- HEAD: `b797cd85`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/sd29-monster-r3/item8/monster-botd2-seraptis.png`
- rendered lines containing the record/expectations:
```
30:Demon (Seraptis)Medium Outsider (Chaotic, Demon, Evil, Extraplanar)
32:Speed 50 ft. · Book of the Damned, Volume 2 p.58 · Hit dice Outsider (Fort/Will):15
42:A seraptis' gaze fills the minds of those within feet with overwhelming and soul-crushing despair. Anyone who fails a DC Will save upon being exposed to a seraptis's gaze immediately takes 1d6 points of Charisma drain and is staggered for 1d6 rounds. If the Charisma drain would normally reduce to creature's Charisma to 0, that creature instead succumbs to overwhelming suicidal urges and attempts to end its life by the most convenient method at hand, subject to GM discretion (in most cases, this effect causes a creature to make a coup de grace attempt on itself, but if a more dramatic method of self-destruction is available, the creature takes that action). Once a creature reaches this suicidal state of despair, it remains in that state until its Charisma score is restored to its normal maximum--if methods of restoring lost Charisma are not available, the suicidal victim must be restrained at all times to prevent attempts to kill itself. This is a mind-affecting effect. The save DC is Charisma-based.
```
