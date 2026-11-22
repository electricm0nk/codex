# SD31-W9-INTEGRATE-001 — DoD-8 on-screen verification

Driven directly via `apps/desktop/.claude/skills/run-desktop/driver.sh`
(`RUN_DESKTOP_AGENT=sd31w9integrate`), per this wave's own instruction —
`verify-on-screen.sh` has no `class_feature` family and a known `race_trait`
coordinate bug, so it was not used.

HEAD at drive time: `82fb6767d3abf68701ab56bc78eaa2610f375bee`.

## What this proves

1. **`app-hub-post-merge.png`** — the app builds and launches cleanly at the
   fully-merged tip (all 5 wave-9 lanes + this cycle's own fixes), confirming
   nothing in the merge or the fixes broke the desktop crate's compile or
   startup path.

2. **`race-traits-strix-alternates.png`** — the Race Traits screen's
   Alternate racial traits tab, filtered to Strix, rendering all 5 real
   alternate traits (Dayguard, Frightening, Nimble, Tough, Wing-Clipped)
   with their real corpus text and citations (ARG p.200), live from the
   engine. This is the direct render surface for this cycle's Strix
   `pilot_compute.rs` wiring fix (`OPEN-ISSUES.md` row 165) — the 4
   magnitude-bearing traits (Nimble/Tough/Frightening/Wing-Clipped) are the
   ones now genuinely wired to a computed save/skill total, and this screen
   confirms condition 3 (real prose reaches the player) holds for all 5,
   consistent with `tests/sd27_alternate_racial_trait_reachability.rs`'s own
   15/15 green run proving condition 2 (the computed delta) for the
   magnitude-bearing 4.

3. **`race-traits-human-alternates.png`** / **`class-progression-overview.png`**
   — further confirmation the broader Race Traits / Class Progression
   catalogs (349 alternate traits across 31 races; 300 class-progression
   rows across 15 classes) still render correctly post-merge, unaffected by
   this cycle's `class_feature` explanation-id matcher fix.

## What this does NOT independently re-prove

The `class_feature` §10 AMENDMENT / negation / compound-word matcher fix
(`OPEN-ISSUES.md` row 164) is a board-VERDICT correctness fix over an
explanation-id join, not a new render path — the record it most directly
concerns (`Slayer ~ Track`) already has a real, live on-screen proof from a
prior wave (`SD31-E5-F1-002`'s own DoD-8,
`artifacts/SD31-E5-F1-002/class-feature-slayer-track-actions-tab.png`,
committed on `tranche/11`), and that render path is untouched by this
cycle's fix (which only changes which explanation ids the classifier will
accept, not how a grounded record renders). Re-driving the full character-
creation flow to reach a Slayer's Actions tab a second time was judged lower
value than screenshotting the surfaces this cycle's own new/changed
served content (Strix) actually touches, given the wave's time budget;
named here rather than silently substituted.
