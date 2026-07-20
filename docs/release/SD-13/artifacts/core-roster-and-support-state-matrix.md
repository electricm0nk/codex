# SD-13 Core Class/Race Roster and Level-10 Progression Matrix

**Artifact type:** Support-state seed authority (planning-facing projection)
**Slice:** SD-13-E1-F1 (carrier root)
**Status:** Active — 21-row breadth roster
**Source of truth:** `src/rules_core/support_state_matrix.rs` (`seeded_sd13_e1_f1_current_truth`)
**Vocabulary:** `src/rules_core/support_state_matrix.rs` module docstring (SupportState / EvidenceTier / EvidenceFreshness enums)

---

## Authority and Reconciliation Doctrine

This document is the **planning-facing readable surface** for the SD-13 support-state matrix. It is **not** the source of truth.

The source of truth is the machine-usable carrier at `src/rules_core/support_state_matrix.rs`. When the two disagree — for any reason, including drift caused by an incomplete doc refresh — the carrier wins. Operator and downstream consumer code MUST treat the carrier as canonical and project from it.

Each row below was copied directly from the carrier at `develop` HEAD (`c82e2be`, post #86 / #102 / #105). The row order, support_state, evidence_tier, grounding_ref, blocker_or_lossiness_note, and next_required_uplift are taken verbatim from the carrier's `seeded_sd13_e1_f1_current_truth()` table. No values are invented in this document.

The vocabulary authority for `SupportState`, `EvidenceTier`, and `EvidenceFreshness` lives in the carrier's module docstring and the enum definitions in the same file. There is no separate `programs/codex/doctrine/support-state-vocabulary.md` artifact on this branch; if a future slice introduces one, this frontmatter should be updated to cite it instead.

---

## State Distribution (develop @ c82e2be)

- Total rows: **21** (7 race + 12 class + 2 interaction)
- `Partial` / `Computed`: **4**
- `Blocked` / `Computed`: **4**
- `Unverified` / `Observed`: **13**

---

## Race Roster (7)

| Row ID | Subject ID | Dimension | Support State | Evidence Tier | Grounding Ref | Blocker / Lossiness Note | Next Required Uplift |
|---|---|---|---|---|---|---|---|
| `race.human.pilot_semantics` | `race:human` | bounded Human pilot race semantics: the named Human ability-bonus (Strength) and Human bonus-feat (Dodge) selections exercised by the GE-06 deterministic proof | `Partial` | `Computed` | `src/rules_core/pilot_compute.rs` | the deterministic pilot grounds only the named Human ability-bonus and bonus-feat pressure; Human size, speed, senses, extra skill ranks, and the remaining racial trait burden are still unverified | classify the remaining Human racial trait burden (size, speed, senses, skill ranks) explicitly |
| `race.dwarf.bounded_semantics` | `race:dwarf` | bounded race semantics | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E2 race-semantic slice |
| `race.elf.bounded_semantics` | `race:elf` | bounded race semantics | `Unverified` | `Observed` | this matrix doc | no direct runtime evidence for any of the seven required Elf race-semantic families at the live evidence floor (2026-07-06): identity/provenance is observed-only via the SD-13 packet roster and the typed matrix row carrier, but ability-score modifiers (PF1 Core +2 Dex / -2 Con and the alternate +2 Int variant), size/speed/movement baseline (Medium, 30 ft base), senses (low-light vision), immunity to sleep, weapon familiarity (longbow / rapier / longsword / shortbow / shortsword), bonus languages, and other core racial traits (keen senses, elven magic / weapon training variants) remain unproven; pilot_compute.rs explicitly gates every non-Human race out of the compute path via `if input.chosen.race_id != HUMAN_RACE_ID`. No Elf fixture exists in tests/fixtures. Promotion above Unverified is counterfeit breadth until a later bounded slice lands grounded evidence for at least one of these families. | SD13-Elf bounded race-semantic classification artifact at `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-elf-bounded-race-semantics-classification-2026-07-06.md` names the seven required race-semantic families and the concrete acceptance criteria (new accepted fixture family, new typed module or expansion emitting computed evidence / explanation / claim-blocking diagnostic, new focused test pinning family evidence at Computed / Oracle-checked tier, updated row state with non-empty blocker note) required before this row may honestly move out of Unverified. |
| `race.gnome.bounded_semantics` | `race:gnome` | bounded race semantics | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E2 race-semantic slice |
| `race.half_elf.bounded_semantics` | `race:half-elf` | bounded race semantics | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E2 race-semantic slice |
| `race.half_orc.bounded_semantics` | `race:half-orc` | bounded race semantics | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E2 race-semantic slice |
| `race.halfling.bounded_semantics` | `race:halfling` | bounded race semantics | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E2 race-semantic slice |

---

## Class Roster (12 — Level-1 through Level-10)

| Row ID | Subject ID | Dimension | Support State | Evidence Tier | Grounding Ref | Blocker / Lossiness Note | Next Required Uplift |
|---|---|---|---|---|---|---|---|
| `class.fighter.level_1_pilot` | `class:fighter` | class progression through level 1 deterministic pilot surface | `Partial` | `Computed` | `tests/ge06_pilot_view_model.rs` | only the bounded Fighter level-1 deterministic pilot surface is proven; mandatory level-10 milestones remain unclassified | widen beyond level 1 and classify mandatory level-10 milestones |
| `class.fighter.levels_2_10` | `class:fighter` | class progression across levels 2-10: bounded milestone proof for levels 2 and 3 only, with levels 4-10 still unproven | `Partial` | `Computed` | `tests/sd13_fighter_level2_level3_progression.rs` | SD13-E3 proves only Fighter levels 2 and 3: base attack / base save progression, the level-2 bonus-feat progression seam, and the level-3 armor-training seam over the deterministic Human loadout. Levels 4-10 remain out of proof, along with level-4 ability-score progression, the repeated bonus-feat cadence, weapon training, later armor-training ranks, and any general feat-effect/prerequisite engine | later SD13-E3 slice widening Fighter beyond level 3 toward the level-10 milestones |
| `class.rogue.bounded_progression` | `class:rogue` | bounded class progression | `Blocked` | `Computed` | `tests/ge06_pilot_total_saves.rs` | `tests/ge06_pilot_total_saves.rs` (`unsupported_chassis_blocks_total_saves`) explicitly claim-blocks `class:rogue:1` under the current bounded compute path | SD13-E3 martial progression slice |
| `class.barbarian.bounded_progression` | `class:barbarian` | bounded class progression | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E3 martial progression slice |
| `class.bard.progression_and_spell_burden` | `class:bard` | bounded class progression and spell burden | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E4 spellcasting slice |
| `class.cleric.progression_and_spell_burden` | `class:cleric` | bounded class progression and spell burden | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E4 spellcasting slice |
| `class.druid.progression_and_spell_burden` | `class:druid` | bounded class progression and spell burden | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E4 spellcasting slice |
| `class.monk.bounded_progression` | `class:monk` | bounded class progression | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E3 martial progression slice |
| `class.paladin.hybrid_chassis_and_spell_burden` | `class:paladin` | bounded hybrid class progression: the deterministic Human Paladin level-1 chassis baseline, with the non-spell class-feature burden and the later spell burden still blocked | `Blocked` | `Computed` | `tests/sd13_hybrid_level1_chassis_baseline.rs` | SD13-E3-F6 leaves direct computed evidence that the deterministic Human Paladin level-1 hybrid chassis is recognized on the compute seam, but the row stays blocked: the non-spell class-feature burden (smite evil, lay on hands, divine grace, mercy) is not implemented, and the later paladin spell burden (spell slots, spell source, spells known/prepared) is deferred to SD13-E4. No Paladin level 2+ is proven | SD13-E3 paladin class-feature slice, then SD13-E4 spell burden |
| `class.ranger.hybrid_chassis_and_spell_burden` | `class:ranger` | bounded hybrid class progression: the deterministic Human Ranger level-1 chassis baseline, with the non-spell class-feature burden and the later spell burden still blocked | `Blocked` | `Computed` | `tests/sd13_hybrid_level1_chassis_baseline.rs` | SD13-E3-F6 leaves direct computed evidence that the deterministic Human Ranger level-1 hybrid chassis is recognized on the compute seam, but the row stays blocked: the non-spell class-feature burden (favored enemy, combat style, skill/tracking) is not implemented, and the later ranger spell burden (spell slots, spell source, spells known/prepared) is deferred to SD13-E4. No Ranger level 2+ is proven | SD13-E3 ranger class-feature slice, then SD13-E4 spell burden |
| `class.sorcerer.progression_and_spell_burden` | `class:sorcerer` | bounded spell-bearing class progression: the deterministic Human Sorcerer level-1 spell baseline, with the bloodline burden and the spontaneous known-spell / slot posture burden still blocked | `Blocked` | `Computed` | `tests/sd13_sorcerer_level1_spell_baseline.rs` | SD13-E4-F7 leaves direct computed evidence that the deterministic Human Sorcerer level-1 spontaneous arcane spell-bearing identity is recognized on the compute seam, but the row stays blocked: the bloodline burden (bloodline selection, level-1 bloodline power, bloodline arcana, bonus spells/feats/skills) is not implemented, and the spontaneous spell burden (spontaneous spells known, spell slots per day, bonus spell slots, spell save DCs) is not computed. No spell math is fabricated and no Sorcerer level 2+ is proven | SD13-E4 Sorcerer bloodline and spontaneous spell-slot slice, then level-2+ progression |
| `class.wizard.progression_and_spell_burden` | `class:wizard` | bounded class progression and spell burden | `Unverified` | `Observed` | this matrix doc | (none) | SD13-E4 spellcasting slice |

---

## Interaction Rows (2)

| Row ID | Subject ID | Dimension | Support State | Evidence Tier | Grounding Ref | Blocker / Lossiness Note | Next Required Uplift |
|---|---|---|---|---|---|---|---|
| `interaction.human_bonus_feat_ability_bonus.pilot_pressure` | `interaction:human-bonus-feat-ability-bonus` | named Human bonus-feat and ability-bonus interaction pressure on the deterministic pilot path | `Partial` | `Computed` | `tests/ge06_pilot_input_contract.rs` | only the named deterministic Human Fighter pilot seam is grounded: the `human_bonus_feat -> feat:dodge` and `human_ability_bonus -> ability:strength` selections now surfaced as explicit compute explanations; the general interaction-row model is not | SD13-E2 / SD13-E3 coupling |
| `interaction.non_human_any_class.progression_pressure` | `interaction:non-human-any-class-progression` | race/class interaction pressure beyond the pilot | `Unverified` | `Observed` | this matrix doc | (none) | add named interaction rows only where separate race and class rows are insufficient |

---

## Reconciliation Notes

- The previous version of this document used a 4-column summary (Row ID / Subject ID / Dimension / Notes) that papered over the post-tranche state distribution and left the Bard / Half-Elf / Wizard rows under-classified as "Unverified — awaiting SD13-E*" when the carrier still showed them at `Unverified/Observed` and several other rows at `Blocked/Computed`. That mismatch has been removed.
- Rows whose grounding reference is "this matrix doc" are `Observed`-tier only: their authority is the named roster scope in this artifact plus the SD-13 packet, with no runtime evidence yet.
- Rows whose grounding reference is a tests/ file are `RefreshableFromLiveProof` in the carrier's `EvidenceFreshness` axis (the SD13-E7-F13 audit posture). All 21 rows today are refresh-required; the carrier records no completed refresh checkpoint in this slice.
- Evidence freshness posture is not duplicated in the tables above. Audit consumers must read `EvidenceFreshness` directly from the carrier — it is the canonical audit axis and MUST NOT be projected independently into this doc.