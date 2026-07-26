# ACG Investigator — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

**Updated (v0.6 alpha swarm, risks item 8, Investigator full-build
closure, 10th ACG/APG class-specific closure, no-spellcasting MVP,
2026-07-26):** Trapfinding, Trap Sense, and Inspiration's flat pool-size
fact are now genuinely wired, plus the class-skill-bonus widening fix
(Climb/Intimidate genuinely earn the bonus, Swim genuinely does not --
the first REAL PARTIAL class-skill match on the whole roster, which
forced `selected_skill_class_skill_bonus_applies`'s single scalar to
split into three independent per-skill functions). Prepared extract
spellcasting (reusing the Alchemist formula list) is deliberately
deferred to its own follow-on slice -- no Alchemist spell-list mapping
exists anywhere in this codebase yet, a genuinely new data-ingestion
cost none of the 9 prior closures needed. Investigator stays permanently
`Blocked` on its own `other_features_deferred` diagnostic. See
`docs/release/v0.6/investigator-acg-full-build-scoping.md` for the full
record.

| Field | Value |
|---|---|
| `class_name` | Investigator |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_investigator` (chassis); `pilot_compute::ground_or_block_investigator_class_features` (Trapfinding/Trap Sense/Inspiration pool-size) |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 95 (distinct `KEY:Investigator ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 5 (Trapfinding, Trap Sense, Inspiration pool-size, Poison Resistance, Alchemy — five structurally independent mechanisms, no shared table linking them — see update below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognizes all 10 ACG classes; Investigator additionally gets its own Trapfinding/Trap Sense/Inspiration dispatch via `ground_or_block_investigator_class_features` |
| `level_up_wired` | No — no `level_up::investigator` module exists |
| `gap_priority` | P2 (real, verified partial coverage; permanently `Blocked` on the remaining named features, including spellcasting) |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

Alchemist Discovery, ~~Alchemy~~ (the flat Craft (alchemy) competence bonus is wired below; prepared extract spellcasting, reusing the Alchemist formula list, is a separate, still-deferred mechanic pending a new `alchemist_spell_list.rs` ingestion), Amazing Inspiration, Combat Inspiration, Device Talent, Discovery (+ ~20 individual Discovery sub-choices), Inspiration's actual spend (only its flat pool-size fact is grounded), Investigator Talents (+ its many sub-talent choices, including the large Rogue Talent family), Keen Recollection, ~~Poison Resistance~~ (wired below), Studied Combat, Studied Strike, Swift Alchemy, Unfailing Logic.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_investigator.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- Trapfinding's `max(InvestigatorLVL/2,1)` and Trap Sense's `InvestigatorLVL/3` (no floor) both verified directly against `acg_abilities_class.lst`'s own `BONUS:VAR` tokens — a real "swapped floors" hazard versus Slayer's own two features (Investigator's Trapfinding has the floor, Trap Sense doesn't; Slayer's Trap Sense has the floor, Trapfinding doesn't), verified independently rather than copied.
- Inspiration's pool-size formula `max(1,InvestigatorLVL/2+INT)` and its `1d6` die verified directly against the corpus's own `BONUS:VAR` tokens.
- Investigator's own real class-skill list confirmed a genuine 2-of-3 partial match (Climb/Intimidate present, Swim absent) — the first on the whole roster, forcing `selected_skill_class_skill_bonus_applies` to split into `selected_skill_climb_is_class_skill`/`..._intimidate_.../..._swim_...`, behavior-preserving for all 6 prior classes (each already cleanly all-three or none-of-three).
- Confirmed by grep/find that no Alchemist class spell-list mapping exists anywhere in this codebase (only per-spell catalogs, not class-to-spell mappings) — the real raw source (104 `Alchemist=N` records in `apg_spells.lst`) is buildable but deferred to its own slice.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Investigator)` and the live `compute_pilot_base_chassis` seam; `pilot_compute::investigator_dispatch_widening_safety_tests` (4 tests) exercises the full Trapfinding/Trap Sense/Inspiration dispatch directly.

Note: the 95 count is the largest of any ACG class because `acg_abilities_class.lst` tags each individual Discovery/Talent sub-choice with its own `KEY:Investigator ~ Discovery ~ ...` record; see `AcgClassCoverage::named_features_expected`'s doc comment for the "floor, not ceiling" caveat this shares with APG's Alchemist row.

## Update (v0.6 alpha swarm, risks item 8, Investigator deepening, 2026-07-26, task #8)

A fresh pass under the corrected standalone-grounding bar (`docs/release/v0.6/investigator-remaining-features-scoping.md`) found two clear missed wins among the previously-deferred features: **Poison Resistance** (`BONUS:VAR|InvestigatorPoisonResistanceBonus|2`, gated in three stacking tiers by `InvestigatorPoisonLVL` -- a situational-save standalone magnitude, the same shape as Bard's Well-Versed/Inquisitor's Purity judgment) and **Alchemy** (`BONUS:VAR|InvestigatorAlchemyCreationBonus|InvestigatorLVL` -- a flat Craft (alchemy) competence bonus, the same shape as Bard's Bardic Knowledge). Both are genuinely wired now. **A real correction to the scoping doc's own claim, caught by reading the raw corpus tier-gating tokens directly rather than trusting the summary**: the doc described Poison Resistance as "+2/+4/+6/+8 scaling, immunity at 20th," but the corpus has no level-20 tier and no +8 step at all -- the real progression is +2 at level 2, +4 at level 5, +6 at level 8, then full immunity at level 10 (a qualitatively different fact, not a fourth numeric tier). Studied Combat/Studied Strike were also re-examined and confirmed genuinely opponent-dependent (both bonuses only apply "vs a studied target," an interaction this engine models nowhere) -- correctly ruled deferred, consistently with Slayer's own Studied Target, pending a future opponent-tracking pillar (task #13). Prepared extract spellcasting (the shared Alchemist formula spell list, ~104 real `Alchemist=N` records in `apg_spells.lst`, also unblocking Alchemist's own deferred spellcasting) remains its own separate, larger follow-on slice, not bundled into this pre-slice. `named_features_wired` rises from 3 to 5. See `pilot_compute.rs`'s `investigator_dispatch_widening_safety_tests` module (3 new/updated tests, including a dedicated tier-progression test proving the corrected formula against the raw corpus) for the full test coverage.
