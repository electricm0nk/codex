# DoD-8 on-screen verification — SD31-W13-INTEGRATE-001

Driven directly via `apps/desktop/.claude/skills/run-desktop/driver.sh`
(`RUN_DESKTOP_AGENT=sd31-w13-integrate`), never `verify-on-screen.sh` (no
class_feature family, known race_trait coordinate bug — per the mandate's
own note).

1. `dod8-01-newchar-form.png` — Create-a-character form, Dwarf/Paladin
   level 4, ability scores computed (STR 16/DEX 14/CON 16/INT 10/WIS
   14/CHA 6 with the dwarf +2 CON/+2 WIS/-2 CHA modifiers applied live).
2. `dod8-02-paladin4-created.png` — "Test Paladi is ready" with real
   computed derived stats: AC 16, Melee +8, BAB +4, Fortitude +7, Reflex
   +3, Will +6.
3. `dod8-03-slayer3-actions-fixture-seam.png` — the fixture-seam lane's
   own pre-existing test character ("Fixture Seam T", Dwarf Slayer 3)
   loaded fresh and its Actions tab re-screenshotted at THIS wave's
   regenerated tip. Shows, live, in-app, with the engine's own stated
   derivation text: **Sneak Attack Dice 1** ("Slayer level 3 Sneak Attack
   dice: level/3 = 1d6"), **Trapfinding Bonus 1** ("level/2 = 1"), **Trap
   Sense Bonus 1**, **Studied Target Bonus 1** — four of this wave's own
   8 class_feature units that reached `done` via the new
   `derived`+`fixture-verified` seam. This is not a stub: the panel is
   built by walking the real corpus record and the real production
   `pilot_compute` explanation, and every number matches the corpus
   formula this cycle's fixture derivation independently verified against
   the pinned oracle.

Known gotcha hit and worked around: native `<select>` dropdowns (Race,
Class, Level) render solid black while open under this Xvfb/WebKitGTK
setup — `screenshot` mid-dropdown is unusable. Worked around by clicking
to open, then using `key <first-letter>` + `key Return` (keyboard-driven
selection) rather than clicking a rendered option, confirmed correct by
re-screenshotting after the dropdown closed each time.
