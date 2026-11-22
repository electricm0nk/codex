# DoD item 8 — on-screen verification, `SD31-E5-F1-003`

Driven directly via `apps/desktop/.claude/skills/run-desktop/driver.sh` (`RUN_DESKTOP_AGENT=sd31invgaps`,
`DISPLAY=:71`) — `verify-on-screen.sh` has no `class_feature` family and a known `race_trait`
coordinate bug, per this card's own instruction.

## What was proven

Created a real Dwarf Unchained Rogue 5 character (`SD31 E5F1003 Unchained Rogue Test`) through the
real character-creation form (`Race: Dwarf (CRB)`, `Class: Unchained Rogue`, `Level: 5`, manual
ability scores STR16/DEX14/CON14/INT10/WIS12/CHA8), loaded its full character sheet via the real
`Load Character` flow, and opened the `Actions` tab.

`unchained-rogue-actions-tab.png` shows, rendering live with real computed values, several of the
exact units this cycle's guarded regen moved `not-started → done`/`held` (§4 of the cycle's own
`progress.md` receipt):

- **`Sneak Attack Dice  3`** — "Unchained Rogue level 5 Sneak Attack: 3d6 ((level + 1) / 2)."
  (`Unchained Rogue ~ Sneak Attack`, `literal-verified`)
- **`Trapfinding Bonus  2`** — "Unchained Rogue level 5 Trapfinding: +2 (max(level / 2, 1)) on
  Perception checks to locate traps and on Disable Device checks" (`Unchained Rogue ~ Trapfinding`,
  `grounded`)
- **`Danger Sense Bonus  1`**, **`Rogue Talents Known  2`**, **`Finesse Training Weapon Choices  1`**,
  **`Rogues Edge Skill Unlocks  1`**, **`Uncanny Dodge Flanking Level  5`**,
  **`Uncanny Dodge Tracker Steps  1`**, **`Debilitating Injury Penalty  -2`** — all real, distinct
  Unchained Rogue class features, each with a real corpus-derived rules-text explanation and a real
  computed magnitude, none of it fabricated or stubbed.

`unchained-rogue-sheet-overview.png` shows the full sheet header (`CLASS: Unchained Rogue 5`,
`HIT POINTS 43/43`, `AC 16`) confirming the class genuinely drives the sheet's other computed
values too (HP, saves, BAB), not merely the Actions-tab rows.

## Why this is the right proof for this cycle

This cycle's own code change is a classifier/registry-side correctness fix
(`modelled_class_books()` + `class_feature_owner`/`class_feature_exact_suffix_grounded`/
`diagnostic_id_names_feature` normalization) — it does not add a new render surface. The render
path these rows go through (`classFeaturesModel.ts`'s Chassis-gutter explanation rendering) already
shipped. What this screenshot proves is condition 3 of `decisions.md §7`'s own done-bar: the record
the inventory now marks `done`/`grounded` genuinely reaches the player-visible screen with its real,
non-empty, non-placeholder rules text and a real computed number — not merely that a code gate is
green.

## Environment

- `RUN_DESKTOP_AGENT=sd31invgaps`, `DISPLAY=:71`, window `2097155`, geometry `1920x1200`.
- App built and launched via `driver.sh launch` against the DEFAULT `apps/desktop/src-tauri/target`
  (no `CARGO_TARGET_DIR` override), so it did not contend with this cycle's own
  `CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-inventory-gaps` builds/tests running
  concurrently — confirmed no lock contention, both completed cleanly in parallel.
- `driver.sh stop` run after capture (see cycle receipt).
