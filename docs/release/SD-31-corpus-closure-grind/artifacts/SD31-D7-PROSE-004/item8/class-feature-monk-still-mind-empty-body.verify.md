# DoD item 8 — on-screen verification, SD31-D7-PROSE-004

**Cycle:** SD31-D7-PROSE-004 (`RETRO_ACTOR=sd31-cf-prose2`)
**Family:** `class_feature` (no `verify-on-screen.sh` family — supports only
`equipment`/`spell`/`race_trait`/`monster` — driven directly via `driver.sh`,
per the mandate's own documented fallback, same as `SD31-D7-PROSE-003`'s own
item-8 receipt).
**Records:** `core_rulebook:class_feature:monk_still_mind`,
`core_rulebook:class_feature:monk_empty_body` — two of the 13 `class_feature`
units this cycle's discriminator moved `grounded` → `text-complete`/`done`
under Decision 7 REFINED (both CONDITIONAL: effect-type and resource/duration
respectively, per `decisions.md §7` REFINED).
**Time (UTC):** 2026-08-16T16:13Z (approx, screenshot mtime)
**HEAD at capture:** 17ba8be5304a4f760af775f57b4e5800dc0a8548 (cycle start;
working tree carries this cycle's uncommitted `v06_work_inventory.rs` changes)
**RUN_DESKTOP_AGENT:** `sd31-cf-prose2`

## Steps

```
export RUN_DESKTOP_AGENT=sd31-cf-prose2
./.claude/skills/run-desktop/driver.sh launch
./.claude/skills/run-desktop/driver.sh click 958 457   # Load Character
./.claude/skills/run-desktop/driver.sh click 570 891   # "Picker Monk 20" row
./.claude/skills/run-desktop/driver.sh click 452 1117  # Load
./.claude/skills/run-desktop/driver.sh click 748 519   # Actions tab
./.claude/skills/run-desktop/driver.sh scroll 900 700 15 down
./.claude/skills/run-desktop/driver.sh screenshot .../class-feature-monk-still-mind.png
./.claude/skills/run-desktop/driver.sh scroll 900 700 30 down
./.claude/skills/run-desktop/driver.sh screenshot .../class-feature-monk-empty-body.png
./.claude/skills/run-desktop/driver.sh stop
```

## What the screenshots show

**`class-feature-monk-still-mind.png`** — "Picker Monk 20" (Human Monk 20),
Actions tab, the **Still Mind 2** row. Two paragraphs render:

1. The engine's own computed derivation (unchanged, black text): "Monk Still
   Mind granted at monk level 20 (PF1 Core Rulebook, 3rd-level monk class
   feature): a monk of 3rd level or higher gains a flat +2 bonus on saving
   throws against enchantment spells and effects...".
2. **NEW — the real corpus `DESC:` text, italicized:** *"You gain a +2 bonus
   on saving throws against enchantment spells and effects."*

Byte-matched by direct file read:
`data/corpus/core_rulebook/class_feature/monk/still_mind.json`'s
`data.description` is `"You gain a +2 bonus on saving throws against
enchantment spells and effects."` — identical, verbatim, to the rendered
italic paragraph.

**`class-feature-monk-empty-body.png`** — same character/tab, scrolled
further, the **Empty Body 0** row. Same two-paragraph shape; the italic
corpus paragraph reads: *"You can assume an ethereal state for 1 minute as
though using the spell Etherealness. Using this ability is a move action
that consumes 3 points for your Ki pool. This ability only affects you and
cannot be used to make other creatures ethereal."*

Byte-matched: `data/corpus/core_rulebook/class_feature/monk/empty_body.json`'s
`data.description` is identical, verbatim.

## What this proves

The render surface (`class_feature_descriptions.rs` →
`list_class_feature_descriptions` Tauri command →
`loadClassFeatureDescriptions.ts` → `classFeaturesModel.ts`'s
`buildClassFeatureSurface`/`findCorpusDescription` → `CharacterSheet.tsx`'s
`ActionsTab`) was built by the prior cycle (`SD31-D7-PROSE-003`) and is
**unchanged by this cycle** — this screenshot proves it generalizes to TWO
MORE records this cycle's own discriminator work newly promoted to `done`,
not merely the `Rogue ~ Trapfinding`/`Master Strike` pair the prior cycle's
own item-8 receipt already proved. Decision 7's condition 3 ("the prose is
available to print in the description on the character sheet") is satisfied
for both records: real corpus text, proven on-screen, byte-matched by direct
file read — not inferred from a green code gate.
