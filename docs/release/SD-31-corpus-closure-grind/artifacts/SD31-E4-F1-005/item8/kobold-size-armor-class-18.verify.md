# DoD-8 on-screen verification — SD31-E4-F1-005

**Card note followed**: `verify-on-screen.sh` has no `class_feature` family and a known
`race_trait` coordinate bug (per the dispatch brief). Drove `driver.sh` directly instead, as
instructed, and say so here.

## What this proves

`SIZE_ONLY_RACE_TRAIT_BUNDLE` / `explain_size_only_race_trait_bundle`
(`src/rules_core/pilot_compute/mod.rs`) makes Kobold's real universal Small-size record
provable one corpus row deep. This screenshot proves the underlying VALUE it describes —
the +1 Small-size Armor Class bonus `combat_size_modifiers` already applies, corpus-wide,
since SD-27 — genuinely renders on the real character sheet for a live, app-created Kobold
character, not just in a headless fixture test.

## Command sequence (real app, real Xvfb, real webview)

```
RUN_DESKTOP_AGENT=sd31e4f1005 apps/desktop/.claude/skills/run-desktop/driver.sh launch
RUN_DESKTOP_AGENT=sd31e4f1005 apps/desktop/.claude/skills/run-desktop/driver.sh click 958 280   # New Character
RUN_DESKTOP_AGENT=sd31e4f1005 apps/desktop/.claude/skills/run-desktop/driver.sh click 596 263   # Race dropdown
RUN_DESKTOP_AGENT=sd31e4f1005 apps/desktop/.claude/skills/run-desktop/driver.sh type "Kobold"   # type-ahead selects Kobold (B1)
RUN_DESKTOP_AGENT=sd31e4f1005 apps/desktop/.claude/skills/run-desktop/driver.sh click 596 189   # Character name field
RUN_DESKTOP_AGENT=sd31e4f1005 apps/desktop/.claude/skills/run-desktop/driver.sh type "SD31 Kobold Size Check"
RUN_DESKTOP_AGENT=sd31e4f1005 apps/desktop/.claude/skills/run-desktop/driver.sh click 504 1106  # Create character
RUN_DESKTOP_AGENT=sd31e4f1005 apps/desktop/.claude/skills/run-desktop/driver.sh scroll 960 700 14
RUN_DESKTOP_AGENT=sd31e4f1005 apps/desktop/.claude/skills/run-desktop/driver.sh screenshot .../kobold-size-armor-class-18.png
```

`DISPLAY=:77`, `RUN_DESKTOP_AGENT=sd31e4f1005`, `WINDOW_ID=2097155`.

## What the screenshot shows

- Race: `Kobold (B1)` (real corpus-driven picker, not the old 7-race hardcoded list —
  `raceOptionsFromChassis`/`build_race_creation_roster`).
- Physical Attributes → Size: `Small` (the app's own independent size resolution, agreeing
  with `race_size_for_race_token`).
- Ability scores: `Kobold racial modifiers: -4 STR, +2 DEX, -2 CON` — confirms Kobold has
  real ability-score modeling in this engine too, wider than this cycle assumed going in.
- Result panel, "SD31 Kobold Size Check is ready — Your character was computed and saved.":
  **Armor Class 18** — the real PF1 Table 8-1 Small-size +1 over the Medium 17 baseline
  (`tests/sd27_size_modifiers_to_armor_class.rs`'s own `BASELINE_ARMOR_CLASS_BEFORE_SIZE`
  constant), and **Melee Attack Bonus +4**, which also carries the same +1 size term
  (`compute_combat_baseline`'s `melee_attack_bonus` formula).

## Honest scope of what this DOES and does NOT prove

- **Does prove**: the AC/attack portion of the universal Small-size modifier is real,
  live, and player-visible for Kobold specifically, through the actual character-creation
  UI, not merely in a Rust unit test.
- **Does NOT prove**: the new `race.kobold.trait_bundle.size` explanation record itself
  rendering as its own line item anywhere in this screen — this app surface renders the
  numeric TOTALS (`explanations` feed the totals, not a line-by-line racial-trait list on
  this particular screen). The explanation record's existence and content are proven by
  `size_only_race_trait_bundle_tests` (3 tests, `src/rules_core/pilot_compute/mod.rs`)
  instead, matching the same split this program already accepts elsewhere between
  "the computed number is real and on-screen" (DoD-8) and "the record citing the
  mechanism is provable one corpus row deep" (unit tests, per §7 condition 3's own
  precedent).
- Stealth's +4 size bonus is still not applied anywhere in this engine (no Stealth skill
  total exists at all) and is not claimed here — named honestly in the explanation record
  itself and in this cycle's commit message.
