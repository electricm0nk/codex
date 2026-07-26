# APG Oracle — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

**Updated (v0.6 alpha swarm, risks item 8, Oracle full-build closure,
2026-07-25):** real known-spell posture (spontaneous, mirroring
Sorcerer's own shape) plus Mystery (Life/Healing Hands) and Curse
(Clouded Vision) are now genuinely wired. Oracle stays permanently
`Blocked` on its own `other_features_deferred` diagnostic (no MVP
narrowing was found for Cure Wounds/Inflict Wounds/Tongues this slice,
unlike Arcanist) — an honest outcome, not a regression. See
`docs/release/v0.6/oracle-apg-full-build-scoping.md` for the full record.

| Field | Value |
|---|---|
| `class_name` | Oracle |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_oracle` (chassis); `pilot_compute::ground_or_block_oracle_class_features`/`ground_or_block_oracle_mystery`/`ground_or_block_oracle_curse` (Mystery/Curse/known-spell posture) |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 19 (distinct `KEY:Oracle ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 2 (Mystery slot, Curse slot — known-spell posture/Orisons share the general spellcasting mechanism, not counted separately, mirroring Arcanist's Cantrips/Warpriest's Orisons) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_apg_class_chassis` in `pilot_compute.rs` recognizes all 6 APG classes; Oracle additionally gets its own known-spell/Mystery/Curse dispatch via `ground_or_block_oracle_class_features` |
| `level_up_wired` | No — no `level_up::oracle` module exists |
| `gap_priority` | P2 (real, verified partial coverage; permanently `Blocked` on the remaining named features) |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Battle Mystery, Bone Mystery, Cure Wounds, Deaf, Flame Mystery, Haunted, Heavens Mystery, Inflict Wounds, Lame, Lore Mystery, Nature Mystery, Orisons (folds into the general spellcasting build, not separately implemented), Stone Mystery, Tongues, Wasting, Waves Mystery, Winds Mystery.

Note: this list already names the other 9 Mysteries (Battle, Bone, Flame, Heavens, Lore, Nature, Stone, Waves, Winds) plus the other 4 Curses (Deaf, Haunted, Lame, Wasting) plus Cure Wounds/Inflict Wounds/Tongues/Orisons — Life Mystery (Healing Hands) and Clouded Vision Curse are now wired, so they're excluded from this gap list. Each Mystery's own per-level Revelation choices (~8 per Mystery) are a separate selectable-list layer, not counted in the 19 above.

## Verification

- Chassis math (three-quarter BAB; good Will; poor Fortitude/Reflex) cross-checked against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens for `CLASS:Oracle` — matches, no defect.
- Known-spells table (levels 1-3: `4/2/-/-`, `5/2/-/-`, `5/3/-/-`) verified directly against `apg_classes.lst`'s own `CAST:`/`KNOWN:` rows, cross-checked against an independent web search (not the self-contradictory `legacy.aonprd.com` fetch, per this segment's own "corpus is the tiebreaker" discipline).
- Healing Hands' `+4` Heal bonus verified against `apg_abilities_class.lst`'s own `BONUS:SKILL|Heal|4` tag; Clouded Vision's `30`-foot cap verified against `BONUS:VAR|OracleCloudedVisionRange|30`.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Oracle)` and the live `compute_pilot_base_chassis` seam; `pilot_compute::oracle_dispatch_widening_safety_tests` (9 tests) exercises the full known-spell/Mystery/Curse dispatch directly.
