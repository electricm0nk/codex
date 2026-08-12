# APG Witch — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

**Updated (v0.6 alpha swarm, risks item 8, Witch full-build closure,
11th ACG/APG class-specific closure, 2026-07-26):** the Ward hex (a flat
deflection/resistance bonus) is now genuinely wired, plus the class-
skill-bonus widening fix (Intimidate genuinely earns the bonus, Climb
and Swim genuinely do not — a second, different partial-match shape
after Investigator's own). This closure was re-scoped after a third
comparative pass corrected the standing "blocked on the unbuilt
Familiar" verdict — Ward doesn't require the Familiar at all, confirmed
its own gate is genuinely immediate at level 1 (unlike Shaman's own
proposed MVP, Healer's Touch, which independent re-verification found
gated to level 8, not immediate at level 1 as the original comparative
scoping doc claimed -- flagged to the lead as a correction before
Shaman's own closure was built). Witch stays permanently `Blocked`
on `other_features_deferred` (fresh own-list spellcasting, the Familiar,
and the other ~18 hexes remain deferred). See
`docs/release/v0.6/shaman-summoner-witch-comparative-scoping.md` for the
full record.

| Field | Value |
|---|---|
| `class_name` | Witch |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_witch` (chassis); `pilot_compute::ground_or_block_witch_class_features` (Ward hex) |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 7 (distinct `KEY:Witch ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 1 (the Hex slot alone — unlike Oracle's Mystery+Curse, Witch has no second independent choice this slice grounds) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_apg_class_chassis` in `pilot_compute.rs` recognizes all 6 APG classes; Witch additionally gets its own Ward-hex dispatch via `ground_or_block_witch_class_features` |
| `level_up_wired` | No — no `level_up::witch` module exists |
| `gap_priority` | P2 (real, verified partial coverage; permanently `Blocked` on the remaining named features, including spellcasting and the Familiar) |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Cantrips, Familiar, Familiar Touch Spells, Patron Spells (fresh own-list spellcasting and the unbuilt Familiar subsystem — no `SPELLLIST:` reuse token exists for Witch, a genuinely new data-ingestion cost), Hex content beyond Ward (the other ~18 base hexes plus the Major Hex/Grand Hex tiers).

Note: the ~20 individual Hexes (Cackle, Evil Eye, Slumber, Ward, ...) and Major/Grand Hexes a Witch selects from are a separate selectable-list layer under a different `CATEGORY:Special Ability` chooser in `apg_abilities.lst`, not counted in the 7 above — this is the largest gap between "named feature slots wired" and "actual playable feature surface" of the six APG classes, since almost all of Witch's build-defining choices live in that unaudited chooser list.

## Verification

- Chassis math (half BAB; good Will; poor Fortitude/Reflex) cross-checked against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens for `CLASS:Witch` — matches, no defect.
- Ward's own gate (`PREVARGTEQ:WitchHexAbilityLVL,1`, where `WitchHexAbilityLVL` resolves to `WitchLVL` unconditionally, and `WitchMinorHexQualify` is set to `1` unconditionally too) confirmed genuinely immediate at level 1, not a delayed grant.
- Ward's formula (`BONUS:VAR|WitchWardBonus|2` base, `+1` at level 8, `+1` at level 16) verified directly against `apg_abilities_class.lst`.
- Witch's own real class-skill list confirmed a genuine partial match (Intimidate present, Climb and Swim both absent) — a second, different partial-match shape than Investigator's own (Climb+Intimidate present, Swim absent) — proving the per-skill class-skill split generalizes correctly.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Witch)` and the live `compute_pilot_base_chassis` seam; `pilot_compute::witch_dispatch_widening_safety_tests` (4 tests) exercises the full Ward-hex dispatch directly.
