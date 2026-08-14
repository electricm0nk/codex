# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `spell` · record: `Barkskin`
- expected on screen: `Barkskin`
- expected on screen: `CRB`
- expected on screen: `Transmutation`
- expected on screen: `Level 2`
- agent: `probe-spell-ground` · date: 2026-08-13T21:28:20Z
- HEAD: `d1593801`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-30-class-feature-archetype-bundle/artifacts/ground-spell-units/crb-barkskin-grounded.png`
- rendered lines containing the record/expectations:
```
7:CRB (652)
20:Transmutation (321)
25:BarkskinCRBTransmutation
26:Level 2
27:Barkskin toughens a creature's skin. The effect grants a +2 enhancement bonus to the creature's existing natural armor bonus. This enhancement bonus increases by 1 for every three caster levels above 3rd, to a maximum of +5 at 12th level. The enhancement bonus provided by barkskin stacks with the target's natural armor bonus, but not with other enhancement bonuses to natural armor. A creature without natural armor has an effective natural armor bonus of +0.
```
