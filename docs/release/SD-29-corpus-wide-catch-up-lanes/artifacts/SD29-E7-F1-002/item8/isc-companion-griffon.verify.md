# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Companion (Griffon)`
- expected on screen: `Companion:2`
- expected on screen: `Magical Beast`
- expected on screen: `fly 40 ft.`
- expected on screen: `STR +6`
- expected on screen: `Companion Advancement (Griffon)`
- agent: `sd29-companion-r4` · date: 2026-08-12T13:37:55Z
- HEAD: `03acb5a5`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F1-002/item8/isc-companion-griffon.png`
- rendered lines containing the record/expectations:
```
14:Companion (Griffon)Large Magical Beast
16:Walk 30 ft., fly 40 ft. · Hit dice Companion:2 · Natural armor +4
17:Ability score adjustments (corpus BONUS:STAT tokens): STR +6, DEX +4, CON +6, INT -6, WIS +2, CHA -2
21:Companion Advancement (Griffon) — CompanionAdvancement
```
