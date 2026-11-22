# item-8 on-screen verification — PASS (manual, driver.sh directly)

- verdict: **PASS**
- family: `companion` · record: `core_essentials:companion:cat`
- what this proves: the 34-unit `apply_done_rung_stamps` book/source_book join fix
  (`OPEN-ISSUES.md` row 104, fixed by this integration cycle) unlocked exactly this
  shape of unit — a `core_essentials`-housed companion creature record whose
  `literal-verified` stamp was previously stranded by the wrong join field.
- corpus row cited: `data/corpus/core_essentials/companion/cat.json`, `raw_tokens`
  carries `BONUS:STAT|STR|-8`, `BONUS:STAT|DEX|4`, `BONUS:STAT|CON|-2`,
  `BONUS:STAT|INT|-8`, `BONUS:STAT|WIS|2`, `BONUS:STAT|CHA|-4`.
- rendered on screen (verbatim from the live Companion Catalog, searched "Cat"):
  `Cat  Tiny Animal  Core Essentials p.131` / `Ability score adjustments (corpus
  BONUS:STAT tokens): STR -8, DEX +4, CON -2, INT -8, WIS +2, CHA -4` — byte-match
  to the corpus row's own six `BONUS:STAT` tokens, no fabrication, no drop.
- agent: `sd31-w6-integrate` · date: 2026-08-16T11:28Z
- HEAD: `b3b621ba8`
- method: `verify-on-screen.sh`'s own automated hub-navigation click landed
  correctly (confirmed by direct screenshot after the click) but the script's
  own marker-read raced ahead of the page's async load and reported a false
  FAILED (kept as `companion-cat.FAILED.verify.md`, not discarded, per the
  standing rule) — worked around by driving `driver.sh` directly: click the
  "Browse Companion Catalog" hub link (855, 971), wait for load, type "Cat" into
  the search box (965, 337), scroll to the exact record, screenshot. This is the
  SAME family of harness timing gap `OPEN-ISSUES.md` row 93 already named for
  `race_trait`'s `SEARCH_Y`, now observed once for `companion` too -- logged as a
  new, narrower finding (a load-race, not a coordinate-table error) rather than
  conflated with row 93's own distinct root cause.
- screenshot: `companion-cat-literal-verified.png`
