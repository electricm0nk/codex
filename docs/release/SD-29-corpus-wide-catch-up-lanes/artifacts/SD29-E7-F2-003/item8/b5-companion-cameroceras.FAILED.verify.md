# item-8 on-screen verification — FAILED

- verdict: **FAILED** — no 'N matching' counter in rendered text — either the search matched NOTHING (the screen swaps the counter for a 'No … match.' notice; record 'Cameroceras' is absent) or the filter never applied (search-box coordinates drifted)
- family: `companion` · record: `Cameroceras`
- expected on screen: `Cameroceras`
- expected on screen: `Bestiary 5`
- expected on screen: `Jet`
- agent: `sd29-companion-r5` · date: 2026-08-12T15:40:47Z
- HEAD: `5164bf36`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Companion Catalog
Back
Loading catalog…```
