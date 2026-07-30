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

Alchemist Discovery, ~~Alchemy~~ (the flat Craft (alchemy) competence bonus is wired below), Amazing Inspiration, Combat Inspiration, Device Talent, Discovery (+ ~20 individual Discovery sub-choices), Inspiration's actual spend (only its flat pool-size fact is grounded), Investigator Talents (+ its many sub-talent choices, including the large Rogue Talent family), Keen Recollection, ~~Poison Resistance~~ (wired below), ~~prepared extract spellcasting~~ (own real posture validation wired below, via the shared `alchemist_spell_list` module -- see Third update), Studied Combat, Studied Strike, Swift Alchemy, Unfailing Logic.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_investigator.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- Trapfinding's `max(InvestigatorLVL/2,1)` and Trap Sense's `InvestigatorLVL/3` (no floor) both verified directly against `acg_abilities_class.lst`'s own `BONUS:VAR` tokens — a real "swapped floors" hazard versus Slayer's own two features (Investigator's Trapfinding has the floor, Trap Sense doesn't; Slayer's Trap Sense has the floor, Trapfinding doesn't), verified independently rather than copied.
- Inspiration's pool-size formula `max(1,InvestigatorLVL/2+INT)` and its `1d6` die verified directly against the corpus's own `BONUS:VAR` tokens.
- Investigator's own real class-skill list confirmed a genuine 2-of-3 partial match (Climb/Intimidate present, Swim absent) — the first on the whole roster, forcing `selected_skill_class_skill_bonus_applies` to split into `selected_skill_climb_is_class_skill`/`..._intimidate_.../..._swim_...`, behavior-preserving for all 6 prior classes (each already cleanly all-three or none-of-three).
- The Alchemist class spell-list mapping (104 real `Alchemist=N` records in `apg_spells.lst`) is now built -- see the Third update below (`rules_tables::apg::alchemist_spell_list`).
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Investigator)` and the live `compute_pilot_base_chassis` seam; `pilot_compute::investigator_dispatch_widening_safety_tests` exercises the full Trapfinding/Trap Sense/Inspiration/Poison Resistance/Alchemy/prepared-extract dispatch directly.

Note: the 95 count is the largest of any ACG class because `acg_abilities_class.lst` tags each individual Discovery/Talent sub-choice with its own `KEY:Investigator ~ Discovery ~ ...` record; see `AcgClassCoverage::named_features_expected`'s doc comment for the "floor, not ceiling" caveat this shares with APG's Alchemist row.

## Update (v0.6 alpha swarm, risks item 8, Investigator deepening, 2026-07-26, task #8)

A fresh pass under the corrected standalone-grounding bar (`docs/release/v0.6/investigator-remaining-features-scoping.md`) found two clear missed wins among the previously-deferred features: **Poison Resistance** (`BONUS:VAR|InvestigatorPoisonResistanceBonus|2`, gated in three stacking tiers by `InvestigatorPoisonLVL` -- a situational-save standalone magnitude, the same shape as Bard's Well-Versed/Inquisitor's Purity judgment) and **Alchemy** (`BONUS:VAR|InvestigatorAlchemyCreationBonus|InvestigatorLVL` -- a flat Craft (alchemy) competence bonus, the same shape as Bard's Bardic Knowledge). Both are genuinely wired now. **A real correction to the scoping doc's own claim, caught by reading the raw corpus tier-gating tokens directly rather than trusting the summary**: the doc described Poison Resistance as "+2/+4/+6/+8 scaling, immunity at 20th," but the corpus has no level-20 tier and no +8 step at all -- the real progression is +2 at level 2, +4 at level 5, +6 at level 8, then full immunity at level 10 (a qualitatively different fact, not a fourth numeric tier). Studied Combat/Studied Strike were also re-examined and confirmed genuinely opponent-dependent (both bonuses only apply "vs a studied target," an interaction this engine models nowhere) -- correctly ruled deferred, consistently with Slayer's own Studied Target, pending a future opponent-tracking pillar (task #13). Prepared extract spellcasting (the shared Alchemist formula spell list, ~104 real `Alchemist=N` records in `apg_spells.lst`, also unblocking Alchemist's own deferred spellcasting) remains its own separate, larger follow-on slice, not bundled into this pre-slice. `named_features_wired` rises from 3 to 5. See `pilot_compute.rs`'s `investigator_dispatch_widening_safety_tests` module (3 new/updated tests, including a dedicated tier-progression test proving the corrected formula against the raw corpus) for the full test coverage.

## Third update (v0.6 alpha swarm, risks item 8, Investigator spellcasting subsystem, 2026-07-26, task #8's main body)

Prepared extract spellcasting is now genuinely validated, closing task #8's main body. New shared module `rules_tables::apg::alchemist_spell_list` (`ALCHEMIST_SPELL_LIST`, 104 real `(name, level)` records extracted directly from `apg_spells.lst` -- 13 genuinely new Alchemist-only spells plus 91 `.MOD` records that graft `Alchemist=N` onto existing Core Rulebook spells, `.MOD` stripped to recover the real spell name; level breakdown 14/25/20/16/14/15 across levels 1-6, matching the raw corpus token count exactly, 0 duplicates) is now the real, shared spell-list source both Investigator (this closure) and Alchemist's own future closure (task #4) can consume. Investigator's own casting shape confirmed as a prepared caster (`SPELLSTAT:INT MEMORIZE:YES SPELLBOOK:YES SPELLLIST:1|Alchemist`, no arcane-school mechanic, so 1 slot per prepared extract like Arcanist, not Wizard's 2-slot opposed-school rule). The corpus carries no per-level `CAST:`/`KNOWN:` rows for Investigator at all (the same external-source caveat Hunter/Arcanist/Warpriest already had) -- the real Extracts Prepared table was verified via THREE independent sources (aonprd.com's Investigator page, d20pfsrd.com's Investigator page, d20pfsrd.com's separate Alchemist page, which also independently confirmed a general claim that Investigator's own table exactly mirrors Alchemist's), all agreeing byte-for-byte. **Deliberately deviates from Wizard's/Arcanist's own level-1-3-bounded convention**: that bound was confirmed (in review) to be an idiom inherited from the original GE-06 pilot slice, not a genuine verification limit, and Investigator's own base chassis already supports all 20 levels with real, fully-verified data in hand for every one -- this closure grounds the complete 1-20 Extracts Prepared table. `unmet_investigator_extract_conditions`/`ground_investigator_prepared_extracts` mirror `unmet_arcanist_spellbook_conditions`/`ground_arcanist_prepared_spellbook`'s exact shape (recorded vs. prepared, no unrecorded prepared extract, no over-preparation, no inaccessible-level extract), plus the real extract save DC (`10 + extract level + Intelligence modifier`, matching Arcanist's own DC formula shape). The prior `other_features_deferred` diagnostic's own claim that spellcasting is "entirely ungrounded" is corrected -- a valid extract posture (including zero known extracts, a valid PF1 posture per Bard's/Arcanist's own precedent) now clears its own dedicated `class_spell.acg.investigator.prepared_extracts.unsupported` diagnostic, though Investigator still does not reach `Computed` overall (every other named feature above remains claim-blocked via `other_features_deferred`, unaffected by this closure). `named_features_wired` is unaffected (spellcasting doesn't add a count, per the established spellcasting-sharing convention). 7 new dispatch-widening tests plus 6 new `alchemist_spell_list`-internal tests, both SD-24 ACG audit tests confirmed unchanged and green. See `docs/release/v0.6/investigator-alchemist-spell-list-scoping.md` for the full corpus/table verification record.
