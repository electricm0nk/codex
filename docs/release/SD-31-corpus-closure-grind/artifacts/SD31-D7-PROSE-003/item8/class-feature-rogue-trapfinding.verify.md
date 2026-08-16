# DoD-8 on-screen verification — class_feature render surface

- **Cycle:** SD31-D7-PROSE-003 (`RETRO_ACTOR=sd31-cf-surface`)
- **When:** 2026-08-16T12:38:16Z
- **HEAD at capture:** `b8c36417dd6dff1bad090d65e3b958f8f39177b2` (pre-commit tip this
  cycle branched from; the cycle's own commits land after this receipt)
- **RUN_DESKTOP_AGENT:** `sd31-cf-surface`
- **Method:** `apps/desktop/.claude/skills/run-desktop/driver.sh` driven directly
  (no `verify-on-screen.sh` family exists for `class_feature` or `companion`,
  per the mandate's own note)

## What this proves

Loaded the real saved character "Sneaky Pete" (Human Rogue 11) via **Load
Character**, opened the **Actions** tab (`ActionsTab` in `CharacterSheet.tsx`),
and scrolled to the Rogue's **Trapfinding** row.

The row renders TWO separate paragraphs, exactly as designed:

1. The engine's own computed derivation (`ExplanationDto.detail`, unchanged,
   pre-existing): *"Rogue Trapfinding class feature: adds a bonus equal to
   max(rogue level / 2, 1)..."*
2. **NEW** — the real corpus `DESC:` text, italicized, joined via
   `classFeaturesModel.ts`'s `matchesCorpusFeature` against
   `list_class_feature_descriptions` (`class_feature_descriptions.rs`):
   > *"You add to Perception skill checks made to locate traps and to Disable
   > Device skill checks. You can use the Disable Device skill to disarm
   > magical traps."*

## Byte-match confirmation, by direct file read

`data/corpus/core_rulebook/class_feature/rogue/trapfinding.json`:

```
"description": "You add +%1 to Perception skill checks made to locate traps
and to Disable Device skill checks. You can use the Disable Device skill to
disarm magical traps.|TrapfindingBonus"
```

`render_pcgen_desc` drops the unresolved `%1` argument (its introducing `+`
sign taken with it — the standing, approved no-fabrication contract; the
`TrapfindingBonus` value is not computed by this engine for display, so it is
honestly omitted rather than guessed) — producing exactly the on-screen text
above. **Byte-for-byte match**, not paraphrase, confirmed by direct read, not
assumed from the render.

## Artifacts

- `class-feature-rogue-trapfinding.png` — full-tab screenshot (Trapfinding row
  visible alongside Sneak Attack, Evasion, Trap Sense, Uncanny Dodge, Improved
  Uncanny Dodge and Master Strike — Master Strike shows the SAME two-paragraph
  shape for a second record, corroborating the mechanism generally, not just
  for one row).
- `class-feature-rogue-trapfinding-crop.png` — tight crop of the Trapfinding
  row alone.
