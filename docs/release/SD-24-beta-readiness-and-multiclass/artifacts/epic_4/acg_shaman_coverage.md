# ACG Shaman — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

**Updated (v0.6 alpha swarm, risks item 8, Shaman full-build closure,
12th ACG/APG class-specific closure, 2026-07-26):** Life Spirit's own
immediately-available Channel ability (flat uses-per-day/dice/DC facts)
is now genuinely wired. This closure's own MVP was corrected mid-scoping:
the original comparative pass proposed Life Spirit's Healer's Touch
revelation, but direct verification of the real `ABILITY:...AUTOMATIC`
grant line found it genuinely gated to level 8+ (`PREVARGTEQ:ShamanSpiritGreater,1`,
itself only set via `PRECLASS:1,Shaman=8`), not immediately available
the way Oracle's Healing Hands is -- caught and corrected before
building, lead independently confirmed. Shaman stays permanently
`Blocked` on `other_features_deferred`. See
`docs/release/v0.6/shaman-summoner-witch-comparative-scoping.md` for the
full record. **Also corrects a stale/inaccurate "Gap features" list**
below (the prior version invented entries -- "Versatile Performance-
analog", "Witch's Familiar-analog" -- that are not real `KEY:Shaman ~
...` records).

| Field | Value |
|---|---|
| `class_name` | Shaman |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_shaman` (chassis); `pilot_compute::ground_or_block_shaman_class_features` (Life Spirit's Channel) |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 10 (distinct `KEY:Shaman ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 1 (the Spirit slot alone -- Life Spirit's Channel is the only immediately-available power grounded under it) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognizes all 10 ACG classes; Shaman additionally gets its own Spirit-choice dispatch via `ground_or_block_shaman_class_features` |
| `level_up_wired` | No — no `level_up::shaman` module exists |
| `gap_priority` | P2 (real, verified partial coverage; permanently `Blocked` on the remaining named features, including spellcasting and the Familiar) |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

Fresh own-list spellcasting (no `SPELLLIST:` reuse token exists for Shaman, a genuinely new data-ingestion cost), Orisons (folds into the general spellcasting mechanism, same "not separately implemented" reasoning as Arcanist's Cantrips/Warpriest's Orisons), Spirit Animal (an unbuilt Familiar subsystem, mechanically distinct from the already-built Animal Companion Wolf stat block), Spirit Magic (spirit-granted bonus spells), Manifestation (a capstone ability), Hex/Wandering Hex/Wandering Spirit (a large chooser-list), the other 9 primary spirits and their own granted abilities, and Life Spirit's own higher-tier abilities beyond Channel (Healer's Touch, genuinely gated to level 8+; Quick Healing).

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_shaman.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- Life Spirit's own record (`KEY:Shaman Spirit ~ Life`) confirmed to grant Channel with zero `PREVARGTEQ` gate (immediately available at level 1), while Healer's Touch's own grant carries `PREVARGTEQ:ShamanSpiritGreater,1` (level 8+, via `PRECLASS:1,Shaman=8`) — verified directly, not assumed from the DESC text alone.
- Channel's formula (`SERVESAS:ABILITY=Special Ability|Channel Positive Energy`, uses/day `1+CHA`, dice `(ShamanSpiritLVL+1)/2` d6 where `ShamanSpiritLVL` resolves to `ShamanLVL`, DC `10+(ShamanSpiritLVL/2)+CHA`) verified directly against `acg_abilities_class.lst` — structurally identical to Cleric's own Channel Energy die-count formula, with the addition of a real DC term Cleric's own grounding deliberately does not compute.
- Shaman's own real class-skill list confirmed to exclude all three of Climb/Intimidate/Swim (the same "none of three" shape as Wizard/Arcanist/Oracle) — no class-skill-bonus bug to fix here.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Shaman)` and the live `compute_pilot_base_chassis` seam; `pilot_compute::shaman_dispatch_widening_safety_tests` (4 tests) exercises the full Life Spirit/Channel dispatch directly.

