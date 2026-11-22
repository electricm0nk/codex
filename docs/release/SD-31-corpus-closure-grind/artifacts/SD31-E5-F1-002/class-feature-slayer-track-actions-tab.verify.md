# DoD-8 on-screen verification — SD31-E5-F1-002

`verify-on-screen.sh` has no `class_feature` family (mandate's own note, confirmed by reading
`apps/desktop/.claude/skills/run-desktop/SKILL.md`'s Families list: equipment / spell / race_trait
/ monster only). Drove `driver.sh` directly instead, per the mandate's own instruction.

- **Agent:** `RUN_DESKTOP_AGENT=sd31cfground`, `DISPLAY=:77`
- **HEAD:** working tree at this cycle's commit (see progress.md receipt for exact SHA)
- **Steps:** launch -> New Character -> Race=Dwarf(CRB), Class=Slayer, Level=1, manual ability
  scores -> Create character -> Load Character -> select "SD31 E5F1002 S" -> Load -> Actions tab
  -> scroll to Slayer class-feature block.
- **Family:** `class_feature`
- **Record:** `advanced_class_guide:class_feature:Slayer ~ Track` (this cycle's own §1 trace
  subject) plus its 5 siblings this cycle's fix newly grounds (Studied Target Bonus/Count, Sneak
  Attack Dice, Trap Sense Bonus, Trapfinding Bonus, Weapon And Armor Proficiency).
- **Rendered, live, on screen** (`class-feature-slayer-track-actions-tab.png`):
  > **Track Bonus 1** — "Slayer level 1 Track: a +1 bonus on Survival checks made to follow tracks
  > (max(level/2, 1) = 1). Survival is not among the three tracked skills either, so this grounds
  > as a standalone flat record"
  (also visible in the same screenshot: Studied Target Bonus 1, Studied Target Count 1, Sneak
  Attack Dice 0, Trap Sense Bonus 1, Trapfinding Bonus 0, Weapon And Armor Proficiency 0 — every
  one of the same 6 evidence rows the SD31-E5-F1-002 artifact's §1/§4 name as newly grounded this
  cycle).
- **What this proves:** the record this cycle's `id_matches_feature_slug_after_known_magnitude_
  suffix_strip` fallback newly credits as `grounded`/`text-complete` genuinely reaches the player
  on the real, rendered character sheet — not merely a green code gate. Satisfies Decision 7
  condition 3 for `Slayer ~ Track`'s promotion to `text-complete`.
- **Result:** PASS.
