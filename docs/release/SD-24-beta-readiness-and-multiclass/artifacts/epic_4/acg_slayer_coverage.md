# ACG Slayer — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

| Field | Value |
|---|---|
| `class_name` | Slayer |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_slayer` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 15 (distinct `KEY:Slayer ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 4 (Sneak Attack, Trap Sense, Trapfinding, Track — see update below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognize single-class Slayer (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::slayer` module exists |
| `gap_priority` | P1 |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

**Correction (v0.6 alpha swarm, risks item 8, Slayer full-build closure, 2026-07-25):** this list was incomplete (7 of the real 15 records). Re-derived directly against `acg_abilities_class.lst`'s own `KEY:Slayer ~ ...` records (15 total, matching `class_features_expected` above): Class Skills, Improved Quarry, Master Slayer, Quarry Output, Slayer Talents, Slayer's Advance, Sneak Attack (~~wired~~), Stalker, Studied Target (Slayer's own real name for its marquee ability -- there is no separate "Quarry" record), Swift Tracker, Track (~~wired~~), Trap Sense (~~wired~~), Trapfinding (~~wired~~), Weapon and Armor Proficiency.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_slayer.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Slayer)` and the live `compute_pilot_base_chassis` seam.

## Update (v0.6 alpha swarm, risks item 8, Slayer full-build closure, seventh ACG/APG class-specific closure, 2026-07-25)

Slayer's own chassis-integration gate (`is_supported_slayer_single_class` in `pilot_compute.rs`) is now genuinely wired -- the seventh widening of `has_supported_class_chassis`, and the first with zero spellcasting scope at all (confirmed: no `SPELLSTAT` token on `CLASS:Slayer`). Grounds four real, flat formulas, each verified directly against the corpus's own `BONUS:VAR` tokens: Sneak Attack dice (`level/3`), Trap Sense (`max(1, level/3)`), Trapfinding (`level/2` on Perception/Disable Device), and Track (`max(level/2, 1)` on Survival). None of the four integrate into an existing total in this engine (no trap-AC/save pillar, no sneak-attack-damage total, and Perception/Disable Device/Survival aren't among the three skills `compute_selected_skill_modifiers` tracks) -- each grounds as a standalone flat record instead, mirroring an already-established idiom: Barbarian's own `class_feature.barbarian.trap_sense` and Rogue's own `class_feature.rogue.trap_sense` are both already grounded the same way. Studied Target (Slayer's own real name for its marquee ability) stays confirmed opponent-dependent, the same wall that already excluded it from the single-ability scan.

Also fixes a real, independently-confirmed bug, the THIRD class needing this exact widening: Slayer's own class-skill list genuinely includes Climb/Intimidate/Swim, so `selected_skill_class_skill_bonus_applies` needed real widening again -- proven with a dedicated failing test written before the fix landed, per the lead's own instruction.

Slayer still does not reach `Computed` -- the new, narrower `class_feature.acg.slayer.other_features_deferred.unsupported` diagnostic (replacing the generic `class_feature.acg.slayer.unsupported`) names Studied Target and Slayer Talents (a chooser-list) as the genuinely still-missing pieces. See `docs/release/v0.6/second-full-class-build-comparative-scoping.md` for the full corpus verification and scope record, and `pilot_compute.rs`'s `slayer_dispatch_widening_safety_tests` module for the full test coverage.

