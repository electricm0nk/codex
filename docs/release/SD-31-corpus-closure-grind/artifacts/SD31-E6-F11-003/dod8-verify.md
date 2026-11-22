# DoD-8 — on-screen verification, `SD31-E6-F11-003`

`verify-on-screen.sh` has no `class_feature` family (confirmed by reading its own Families
list) — drove `apps/desktop/.claude/skills/run-desktop/driver.sh` directly, per this card's
own instruction, never concurrently with `./scripts/verify.sh` (the one time this cycle
briefly did so by mistake, the launch was stopped before the app window appeared and is
recorded as a retro incident, `docs/retro/events/sd31-fixture-seam.jsonl`).

`RUN_DESKTOP_AGENT=sd31fixtureseam`, own DISPLAY (`:77`, driver-allocated).

## What was driven

Created a real Dwarf Slayer 3 character ("Fixture Seam T") through the actual app form
(race Dwarf (CRB), class Slayer, level 3 — chosen specifically because two of this cycle's
own 9 hand-derived fixtures, `advanced_class_guide:class_feature:slayer_sneak_attack` and
`advanced_class_guide:class_feature:slayer_trapfinding`, are both real, pre-existing
`pilot_compute` production consumers for the Slayer class). Loaded the full character
sheet, opened the **Actions** tab.

## What rendered, byte for byte

- **`class-feature-slayer3-sheet-header.png`** — the loaded sheet, header confirms
  "Fixture Seam T — Slayer 3", `Level 3`, class panel `Slayer 3`.
- **`class-feature-slayer3-actions-tab-derived-values.png`** — the Actions tab's Slayer
  section, showing (verbatim, live, computed by the real engine, not this screenshot's
  narration):
  - **`Sneak Attack Dice  1`** — "Slayer level 3 Sneak Attack dice: level/3 = 1d6." Matches
    this cycle's own fixture `advanced_class_guide:class_feature:slayer_sneak_attack`
    (`SlayerSneakAttackLVL/3`, divisor 3, no offset): 3/3 = 1. ✓
  - **`Trapfinding Bonus  1`** — "Slayer level 3 Trapfinding: a +1 bonus on Perception
    checks made to locate traps and Disable Device checks (level/2 = 1)." Matches this
    cycle's own fixture `advanced_class_guide:class_feature:slayer_trapfinding`
    (`SlayerTrapfindingLVL/2`, divisor 2, no offset): 3/2 = 1. ✓
  - (`Trap Sense Bonus 1`, also visible, is `advanced_class_guide:class_feature:
    slayer_trap_sense` — a `max(1, level/3)`-wrapped formula this cycle's seam deliberately
    does NOT cover, per `OPEN-ISSUES.md` row 222; shown here only as adjacent context, not
    claimed as this cycle's own fixture-verified evidence.)

Both `Sneak Attack Dice` and `Trapfinding Bonus` are units this cycle's guarded regen moved
`grounded` → `fixture-verified` (board `done`), and both are now provably visible on the real
screen a player would look at — not merely present in `docs/work-inventory.json`.

## Cleanup

`driver.sh stop` run after the screenshots. `git status --porcelain` in the worktree
confirms zero tracked-directory changes from character creation (the test character lives
only in the app's own local save directory).
