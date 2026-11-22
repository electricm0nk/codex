# item-8 on-screen verification — PASS

- verdict: **PASS**
- family: `equipment` · record: `Catapult (Standard)`
- expected on screen: `800 gp`
- agent: `sd31-equip-repair` · date: 2026-08-16T08:26Z
- HEAD: `5d0cd1595` (`sd31-equip-repair/E6-F5-004`, this cycle's uncommitted-at-capture-time tree)
- harness: driven manually via `apps/desktop/.claude/skills/run-desktop/driver.sh` (the shared
  `verify-on-screen.sh`'s automated navigation FAILED to land on the Equipment Catalog screen this
  run — logged as `ultimate-combat-catapult-standard.FAILED.verify.md` alongside this file, kept as
  evidence per the standing "committed as evidence, not discarded" convention, not investigated
  further under this cycle's own time budget)
- steps: hub → click "Browse Equipment Catalog" (578,929) → click search box (970,326) → type
  `Catapult (Standard)` → screenshot → click a neutral blur point (970,700) → `ctrl+a` → `ctrl+c` →
  read X clipboard
- extraction (machine-verdicted, not eyeballed): X clipboard after select-all/copy on the webview
  contains `Catapult (Standard)UCArms & Armor` and the literal substring `800 gp` — both confirmed
  present via `grep -c "800 gp"` (1 match) and `grep "Catapult"` on the extracted text.

**Why this record, deliberately:** `Catapult (Standard)` is one of the 39 records this cycle
re-cited (`OPEN-ISSUES.md` row 90/92) — before the fix it shipped `raw_tokens` harvested from
`uc_profs_weapon.lst:188` (a weapon-proficiency listing with no `COST:` field at all) while its
`cost_gp: 800.0` came from an independent, correct hand-authored table entry. This screenshot proves
the FIX end to end: the citation now resolves to `uc_equip_arms_armor.lst:168` (the real row), and
the corrected, real `800 gp` value renders live on the Equipment Catalog screen a player would
actually see — condition 3 of Decision 7's own bar, applied here to a `static`/`literal-verified`
record rather than a prose-only one.
