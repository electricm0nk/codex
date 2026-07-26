# ACG Warpriest — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

| Field | Value |
|---|---|
| `class_name` | Warpriest |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_warpriest` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 18 (distinct `KEY:Warpriest ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 2 (Blessings, Sacred Weapon — see update below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognize single-class Warpriest (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::warpriest` module exists |
| `gap_priority` | P1 |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

**Correction (v0.6 alpha swarm, risks item 8, Warpriest full-build closure, 2026-07-25):** this list was approximate/inaccurate (invented entries -- "Sacred Weapon/Armor Enhancement", "Second Blessing", "Channel Energy-analog" -- none of which are real `KEY:Warpriest ~ ...` records). Re-derived directly against `acg_abilities_class.lst`'s own `KEY:Warpriest ~ ...` records (18 total, matching `class_features_expected` above): Aspect of War, Aura, Blessings (~~wired~~), Bonus Feats, Bonus Languages, Channel Energy, Channel Negative Energy, Channel Positive Energy, Class Skills, Fervor, Focus Weapon, IUS No, IUS Yes, Orisons, Sacred Armor, Sacred Weapon (~~wired~~), Spontaneous Casting, Weapon and Armor Proficiency.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_warpriest.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Warpriest)` and the live `compute_pilot_base_chassis` seam.

## Update (v0.6 alpha swarm, risks item 8, Warpriest full-build closure, sixth ACG/APG class-specific closure, 2026-07-25)

Warpriest's own chassis-integration gate (`is_supported_warpriest_single_class` in `pilot_compute.rs`) is now genuinely wired -- the sixth widening of `has_supported_class_chassis`. Grounds: Blessings' flat uses-per-day (`level/2+3`) and save DC (`level/2+10+WIS`), both unconditional; Sacred Weapon's base-damage-die formula (unconditional, genuinely near-zero value at this level for this fixture's Longsword -- 1d6 vs. the Longsword's own better native 1d8, named honestly rather than suppressed); Destruction Blessing's own Destructive Attacks minor power (self-application-only, activation-gated, mirroring Cleric's own Touch of Good precedent exactly -- the one canonical Blessing this closure grounds out of ~20 real types); and a genuine prepared-spellbook posture, own independently-verified per-level table (`warpriest_base_spells_per_day`, sourced directly from `acg_classes.lst`'s own real `CAST:` rows, not a derived formula) -- verified to match Cleric's own table exactly at levels 1-2 but genuinely diverge from level 3 on (Cleric grants a 2nd-level slot at level 3; Warpriest not until level 4, and at a lower count), even though the spell-list CONTENT and casting SHAPE (prepared, `SPELLLIST:1|Cleric`) are genuinely shared.

Also fixes a real, independently-confirmed bug in `selected_skill_class_skill_bonus_applies`: Warpriest's own class-skill list genuinely includes Climb/Intimidate/Swim (unlike Wizard/Arcanist, whose lists include none of the three), so this function needed real widening to admit Warpriest, not just a "verify it already answers correctly" check -- the mirror-image of the original Wizard class-skill-modifier bug (false positive there, false negative here).

Warpriest still does not reach `Computed` for a bare posture (no Blessing chosen, no spells) -- the new, narrower `class_feature.acg.warpriest.other_features_deferred.unsupported` diagnostic (replacing the generic `class_feature.acg.warpriest.unsupported`) names the other 19 Blessing types, Sacred Weapon's own active weapon-enhancement mechanic, Fervor, Channel Energy, Sacred Armor, Aspect of War, Spontaneous Casting, Aura, Focus Weapon, and Bonus Languages as the genuinely still-missing pieces. See `docs/release/v0.6/warpriest-acg-full-build-scoping.md` for the full corpus verification and scope record, and `pilot_compute.rs`'s `warpriest_dispatch_widening_safety_tests` module for the full test coverage.

