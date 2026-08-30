//! SD-20 feat prerequisite engine — Epic 3 (`scope-draft.md` §1.3,
//! `technical-design.md` §2.2).
//!
//! Fourth and FINAL Epic-3 cycle, fourth work-unit per `scope-draft.md`
//! §1.3's cycle order (general feats, then combat, then `ItemCreation`, now
//! `Metamagic`). The first cycle (`b830769`) landed `FeatCategory::General`
//! after an earlier blocked cycle (`cycle-2026-07-17T1920`) found the SD-19
//! table store had no feat catalog at all — resolved at `04c3d08`, which
//! landed `rules_tables::crb::feats` (`feat_tables()`, 185 real CRB feat
//! records across four categories — General 50, Combat 110, ItemCreation 8,
//! Metamagic 17). The second cycle (`c15983d`) landed `FeatCategory::Combat`.
//! The third cycle (`ce4a251`) landed `FeatCategory::ItemCreation`. This
//! cycle lands the fourth and final category, `FeatCategory::Metamagic` —
//! see `feat_prereqs/metamagic.rs`, which mirrors `feat_prereqs/general.rs`,
//! `feat_prereqs/combat.rs`, and `feat_prereqs/item_creation.rs` exactly.
//! **This closes Epic 3**: every feat category in the landed CRB feat
//! catalog now has a landed per-category evaluation module.
//!
//! Reads the feat catalog directly (`rules_tables::crb::feats::feat_tables()`)
//! per `technical-design.md` §2.0's table-store access convention (no
//! `RulesTables` parameter of any kind; a direct, fully-qualified `use`
//! import of the specific table item, called inline) — the same pattern
//! Epic 2 (`spellbook.rs` / `spellbook/abjuration.rs`) and Epic 5
//! (`equipment_effects.rs` / `equipment_effects/arms_armor.rs`) already
//! converged on independently.
//!
//! `technical-design.md` §2.2's illustrative seam signature takes a
//! `feat: &FeatKey` parameter and a `character_history: &CharacterHistory`
//! parameter (a type sketched only in a comment: "feats taken, race,
//! class, ability scores, BAB, skills" — never defined anywhere in this
//! repo). This cycle drops `character_history`/`CharacterHistory`
//! entirely rather than inventing a parallel type or re-deriving a
//! duplicate of the already-landed `CharacterInput`
//! (`character_input.rs`) — the same "adapt illustrative doctrine types
//! to the real codebase shape, don't invent a parallel type" precedent
//! Epic 1's `contract.rs` and Epic 4's `skill_allocation.rs` both already
//! set. It is dropped (not adapted to `&CharacterInput`) because this
//! cycle's bounded General-feats evaluation needs no character state at
//! all — see `feat_prereqs/general.rs`'s doc comment for why. A future
//! category cycle that needs real character context (e.g. a Combat feat
//! gated on BAB, or a Metamagic feat needing known spells) should add a
//! `character: &CharacterInput` parameter back onto these functions at
//! that point, when there is a real field to read from it — not before.
//!
//! `FeatKey` is defined here (not sketched with fields anywhere in
//! `technical-design.md`) as the minimal identity a catalog lookup needs:
//! the feat's catalog id plus its category, reusing the already-landed
//! `rules_tables::crb::feats::FeatCategory` enum rather than re-deriving
//! a duplicate category taxonomy.

//! # SD-27: real prerequisite evaluation across all five books
//!
//! Everything above describes the SD-20 engine, which answers exactly one
//! question -- "is this feat id in the CRB catalog under this category?" --
//! and was the only prerequisite code in the product. The consequence was
//! player-visible and total: **a Fighter 1 with a +1 base attack bonus
//! could take Improved Two-Weapon Fighting**, which requires BAB +6, Dex 17
//! and the Two-Weapon Fighting feat. All 690 offered feats were accepted by
//! every character regardless of prerequisites.
//!
//! [`evaluate_catalog_feat_prerequisites`] below is the real check. It
//! reads the `PRE`-family tokens off the book-spanning catalog
//! (`rules_tables::feats_all`, which now carries them for all five books --
//! ARG's 187 rows and PU's 17 had never been gathered at all) and evaluates
//! them against the character through [`pre_tokens`], which is hand-modelled
//! per token kind per `decisions.md` §24.
//!
//! ## Why the SD-20 functions are still here
//!
//! They are a different, narrower question with their own callers and their
//! own tests, and deleting them is not this cycle's job. Nothing new calls
//! them; `evaluate_catalog_feat_prerequisites` does not route through them.

pub mod combat;
pub mod general;
pub mod item_creation;
pub mod metamagic;
pub mod pre_tokens;

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::feats::FeatCategory;
use crate::rules_core::rules_tables::feats_all::{all_feat_tables, FeatCatalogRecord};
use crate::rules_core::rules_tables::RuleSetId;
use pre_tokens::{evaluate_prerequisite_token, CharacterPrereqFacts, ClauseOutcome};

/// Identifies one catalog feat: its id (matches `FeatTableEntry.key` /
/// `.name`, and `CharacterInput.chosen.selected_feats` entries) plus the
/// category it is being evaluated under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatKey {
    pub feat_id: String,
    pub category: FeatCategory,
}

/// Result of checking whether a feat's prerequisites are met. See this
/// module's doc comment: for the landed `General` category, "met" means
/// "found in the catalog under the requested category" — the table store
/// carries no per-feat prerequisite chain data (yet) to evaluate more
/// specifically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrerequisiteEvaluation {
    pub is_eligible: bool,
    pub failing_prerequisites: Vec<FailedPrerequisite>,
    pub warnings: Vec<PrerequisiteWarning>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedPrerequisite {
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrerequisiteWarning {
    pub message: String,
}

/// The delta a feat contributes, per `technical-design.md` §2.2. Bounded
/// (like `spellbook::abjuration::AbjurationSpellEffect`) to what the
/// catalog table actually carries: the feat's `DESC:` text plus
/// `TableCellRef` provenance — no numeric derived-stat delta, since the
/// catalog carries no `BONUS:`-token data (unlike `EquipmentRecord`,
/// which exposes raw corpus tokens; `FeatTableEntry` only carries
/// `key`/`category`/`name`/`description`). `None` fields mean "not
/// resolved" (unknown feat id, or a category this engine has not landed
/// yet), never a fabricated value.
#[derive(Debug, Clone, PartialEq)]
pub struct FeatEffects {
    pub feat_id: String,
    pub description: Option<String>,
    pub table_cell: Option<TableCellRef>,
}

/// Dispatches by category to the per-category evaluation function, the
/// same dispatch shape `spellbook::compute_spellbook_coverage` and
/// `equipment_effects::compute_equipment_effects` already use. As of this
/// cycle all four `FeatCategory` variants — `General`, `Combat`,
/// `ItemCreation`, and `Metamagic` — have a landed per-category module,
/// closing Epic 3 (`scope-draft.md` §1.3): every feat category in
/// `rules_tables::crb::feats::feat_tables()` now has a landed evaluation
/// path.
pub fn evaluate_feat_prerequisites(feat: &FeatKey) -> PrerequisiteEvaluation {
    match feat.category {
        FeatCategory::General => {
            let result = general::evaluate_general_feat_prerequisites(&feat.feat_id);
            PrerequisiteEvaluation {
                is_eligible: result.is_eligible,
                failing_prerequisites: result
                    .failing_prerequisites
                    .into_iter()
                    .map(|reason| FailedPrerequisite { reason })
                    .collect(),
                warnings: Vec::new(),
            }
        }
        FeatCategory::Combat => {
            let result = combat::evaluate_combat_feat_prerequisites(&feat.feat_id);
            PrerequisiteEvaluation {
                is_eligible: result.is_eligible,
                failing_prerequisites: result
                    .failing_prerequisites
                    .into_iter()
                    .map(|reason| FailedPrerequisite { reason })
                    .collect(),
                warnings: Vec::new(),
            }
        }
        FeatCategory::ItemCreation => {
            let result = item_creation::evaluate_item_creation_feat_prerequisites(&feat.feat_id);
            PrerequisiteEvaluation {
                is_eligible: result.is_eligible,
                failing_prerequisites: result
                    .failing_prerequisites
                    .into_iter()
                    .map(|reason| FailedPrerequisite { reason })
                    .collect(),
                warnings: Vec::new(),
            }
        }
        FeatCategory::Metamagic => {
            let result = metamagic::evaluate_metamagic_feat_prerequisites(&feat.feat_id);
            PrerequisiteEvaluation {
                is_eligible: result.is_eligible,
                failing_prerequisites: result
                    .failing_prerequisites
                    .into_iter()
                    .map(|reason| FailedPrerequisite { reason })
                    .collect(),
                warnings: Vec::new(),
            }
        }
        // `Teamwork` and `Panache` exist only on APG/ACG records (see
        // `FeatCategory`'s own doc comment). Every per-category module
        // above evaluates against the *CRB* catalog
        // (`rules_tables::crb::feats::feat_tables()`), which by
        // construction holds no record of either category, so neither has
        // a landed evaluation path. Routing them through a CRB lookup
        // anyway would report all 11 real APG/ACG feats as "not a
        // recognized feat" -- a wrong reason dressed up as a real one.
        // This states what is actually true instead.
        FeatCategory::Teamwork | FeatCategory::Panache => PrerequisiteEvaluation {
            is_eligible: false,
            failing_prerequisites: vec![FailedPrerequisite {
                reason: format!(
                    "'{}' is a {:?}-category feat, which only APG/ACG records carry; \
                     this engine has no landed prerequisite-evaluation path for that \
                     category yet, so eligibility is unproven rather than denied on \
                     a real prerequisite",
                    feat.feat_id, feat.category
                ),
            }],
            warnings: Vec::new(),
        },
    }
}

pub fn compute_feat_effects(feat: &FeatKey) -> FeatEffects {
    match feat.category {
        FeatCategory::General => match general::resolve_general_feat_effect(&feat.feat_id) {
            Some(effect) => FeatEffects {
                feat_id: effect.feat_id,
                description: Some(effect.description),
                table_cell: Some(effect.table_cell),
            },
            None => FeatEffects {
                feat_id: feat.feat_id.clone(),
                description: None,
                table_cell: None,
            },
        },
        FeatCategory::Combat => match combat::resolve_combat_feat_effect(&feat.feat_id) {
            Some(effect) => FeatEffects {
                feat_id: effect.feat_id,
                description: Some(effect.description),
                table_cell: Some(effect.table_cell),
            },
            None => FeatEffects {
                feat_id: feat.feat_id.clone(),
                description: None,
                table_cell: None,
            },
        },
        FeatCategory::ItemCreation => {
            match item_creation::resolve_item_creation_feat_effect(&feat.feat_id) {
                Some(effect) => FeatEffects {
                    feat_id: effect.feat_id,
                    description: Some(effect.description),
                    table_cell: Some(effect.table_cell),
                },
                None => FeatEffects {
                    feat_id: feat.feat_id.clone(),
                    description: None,
                    table_cell: None,
                },
            }
        }
        FeatCategory::Metamagic => match metamagic::resolve_metamagic_feat_effect(&feat.feat_id) {
            Some(effect) => FeatEffects {
                feat_id: effect.feat_id,
                description: Some(effect.description),
                table_cell: Some(effect.table_cell),
            },
            None => FeatEffects {
                feat_id: feat.feat_id.clone(),
                description: None,
                table_cell: None,
            },
        },
        // APG/ACG-only categories with no landed CRB-catalog resolver --
        // see the matching arm in `evaluate_feat_prerequisites`. `None`
        // fields mean "not resolved", which is exactly the case here, and
        // is the same shape every other arm returns on a catalog miss.
        FeatCategory::Teamwork | FeatCategory::Panache => FeatEffects {
            feat_id: feat.feat_id.clone(),
            description: None,
            table_cell: None,
        },
    }
}

// ---------------------------------------------------------------------------
// SD-27: real, book-spanning prerequisite evaluation
// ---------------------------------------------------------------------------

/// One catalog feat's prerequisite verdict for one character.
///
/// The three lists are kept apart deliberately, because collapsing them is
/// how a checker starts lying. `unmet` is the only one that makes a feat
/// unavailable; `unverified` is the honest record of clauses this engine
/// could not evaluate (the feat stays offered, and the player is told what
/// was not checked); `met` is what the character does satisfy, so a picker
/// can show why a feat is available rather than only why it is not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatPrerequisiteReport {
    pub feat_key: String,
    pub rule_set: RuleSetId,
    /// True when **no** prerequisite clause is definitively unmet. A feat
    /// with only unverifiable clauses is eligible-with-a-note, never a
    /// silent denial.
    pub is_eligible: bool,
    /// How many top-level `PRE`-family tokens the corpus record carries.
    /// `0` means the record genuinely has no prerequisites.
    pub prerequisite_token_count: usize,
    pub met: Vec<String>,
    pub unmet: Vec<FailedPrerequisite>,
    pub unverified: Vec<PrerequisiteWarning>,
}

impl FeatPrerequisiteReport {
    /// A single player-facing line for why this feat is unavailable, or
    /// `None` when it is available. This is what a greyed-out picker row
    /// shows -- an unavailable affordance with no stated reason is the
    /// dead-affordance shape `no-stub-mvp-doctrine.md` forbids.
    pub fn unavailable_reason(&self) -> Option<String> {
        if self.is_eligible {
            return None;
        }
        Some(
            self.unmet
                .iter()
                .map(|failed| failed.reason.as_str())
                .collect::<Vec<_>>()
                .join("; "),
        )
    }
}

/// Evaluates one catalog record's real corpus prerequisites against
/// `facts`.
///
/// A record with `prerequisites: None` is eligible with an empty report --
/// that is the corpus saying the feat has no prerequisites, and 91 of the
/// catalog's 690 records really are like that.
pub fn evaluate_catalog_feat_prerequisites(
    record: &FeatCatalogRecord,
    rule_set: RuleSetId,
    facts: &CharacterPrereqFacts,
) -> FeatPrerequisiteReport {
    let tokens = record.prerequisites.unwrap_or(&[]);
    let mut met = Vec::new();
    let mut unmet = Vec::new();
    let mut unverified = Vec::new();

    for token in tokens {
        match evaluate_prerequisite_token(token, facts) {
            ClauseOutcome::Met { requirement } => met.push(requirement),
            ClauseOutcome::Unmet { reason, .. } => unmet.push(FailedPrerequisite { reason }),
            ClauseOutcome::Unmodelled { token, note } => unverified.push(PrerequisiteWarning {
                message: format!("not verified: {note} ({token})"),
            }),
            ClauseOutcome::Informational { .. } => {}
        }
    }

    FeatPrerequisiteReport {
        feat_key: record.key.to_owned(),
        rule_set,
        is_eligible: unmet.is_empty(),
        prerequisite_token_count: tokens.len(),
        met,
        unmet,
        unverified,
    }
}

/// Every catalog record's verdict for one character, in book order --
/// what a feat picker needs to render 690 rows with the unavailable ones
/// greyed and reasoned.
pub fn evaluate_every_catalog_feat(
    facts: &CharacterPrereqFacts,
) -> Vec<FeatPrerequisiteReport> {
    all_feat_tables()
        .iter()
        .flat_map(|book| {
            book.entries
                .iter()
                .map(move |entry| evaluate_catalog_feat_prerequisites(entry, book.rule_set, facts))
        })
        .collect()
}

/// The verdict for the catalog record identified by `feat_key`, in any of
/// the id shapes `chosen.selected_feats` really carries (`"Power Attack"`
/// or `"feat:power_attack"`), or `None` when no catalog record matches.
///
/// `None` is not "allowed": a caller enforcing prerequisites must decide
/// what an unknown feat id means at its own call site, so the decision
/// stays visible.
pub fn evaluate_feat_key_prerequisites(
    feat_key: &str,
    facts: &CharacterPrereqFacts,
) -> Option<FeatPrerequisiteReport> {
    use crate::rules_core::feat_identity;

    all_feat_tables().iter().find_map(|book| {
        book.entries
            .iter()
            .find(|entry| feat_identity::same(entry.key, feat_key))
            .map(|entry| evaluate_catalog_feat_prerequisites(entry, book.rule_set, facts))
    })
}

/// Builds the fact snapshot from chosen input plus the caller's already
/// computed base attack bonus. A thin re-export so callers need only this
/// module.
pub fn character_prereq_facts(
    input: &CharacterInput,
    base_attack_bonus: i16,
) -> CharacterPrereqFacts {
    CharacterPrereqFacts::from_character(input, base_attack_bonus)
}

#[cfg(test)]
mod prerequisite_tests {
    use super::*;
    use crate::rules_core::character_input::{
        AbilityScores, CharacterClassLevel, ChosenCharacterState,
    };

    fn character(level: u8, dexterity: i16, feats: &[&str]) -> CharacterInput {
        CharacterInput {
            case_id: None,
            source_package_id: "test".to_owned(),
            chosen: ChosenCharacterState {
                race_id: "race:human".to_owned(),
                class_levels: vec![CharacterClassLevel {
                    class_id: "class:fighter".to_owned(),
                    level,
                }],
                ability_scores: AbilityScores {
                    strength: 14,
                    dexterity,
                    constitution: 12,
                    intelligence: 10,
                    wisdom: 10,
                    charisma: 8,
                },
                selected_feats: feats.iter().map(|f| (*f).to_owned()).collect(),
                skill_allocations: Vec::new(),
                equipment_selections: Vec::new(),
                selected_choices: Vec::new(),
                selected_traits: Vec::new(),
                spells_selected: Vec::new(),
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    /// The defect, stated as the operator stated it.
    #[test]
    fn a_fighter_1_cannot_take_improved_two_weapon_fighting_and_is_told_why() {
        let input = character(1, 13, &[]);
        let facts = character_prereq_facts(&input, 1);
        let report = evaluate_feat_key_prerequisites("Improved Two-Weapon Fighting", &facts)
            .expect("the feat is in the catalog");

        assert!(!report.is_eligible);
        let reason = report.unavailable_reason().expect("an ineligible feat must state a reason");
        assert!(reason.contains("base attack bonus +6"), "{reason}");
        assert!(reason.contains("+1"), "{reason}");
        assert!(reason.contains("Two-Weapon Fighting"), "{reason}");
        assert!(reason.contains("DEX 17"), "{reason}");
    }

    /// ...and the build that legitimately qualifies is not blocked.
    #[test]
    fn a_fighter_6_with_dex_17_and_two_weapon_fighting_can_take_it() {
        let input = character(6, 17, &["Two-Weapon Fighting"]);
        let facts = character_prereq_facts(&input, 6);
        let report = evaluate_feat_key_prerequisites("Improved Two-Weapon Fighting", &facts)
            .expect("the feat is in the catalog");

        assert!(report.is_eligible, "unmet: {:?}", report.unmet);
        assert_eq!(report.unavailable_reason(), None);
        assert_eq!(report.prerequisite_token_count, 3);
        assert_eq!(report.met.len(), 3, "all three clauses satisfied: {:?}", report.met);
    }

    /// The catalog must not go dead, and every denial must carry a reason.
    ///
    /// The pinned number is the real one, derived by running this: a Human
    /// Fighter 1 with Str 14 / Dex 13 / Int 10, no feats and no allocated
    /// skill ranks qualifies for **509 of 1578** (was 386 of 1357 before
    /// SD28-E29 added Ultimate Psionics' 221 feats -- UPsi, like every
    /// Ultimate book before it, carries real mechanically-evaluable
    /// `PRE`-family tokens, checked genuinely: 123 of the 221 pass a
    /// starting Fighter's build -- most of UPsi's own `Psionic`-category
    /// feats gate on `PREVARGTEQ:IsPsionic,1`, which a non-psionic
    /// Fighter never satisfies, so the ratio is lower than UC's or UM's).
    /// The two dominant blockers among the remaining denials are clauses
    /// requiring another feat the character has not taken (`Cleave` needs
    /// `Power Attack`, `Mobility` needs `Dodge`) and race gates, most of
    /// which are ARG feats belonging to races other than Human.
    /// Spot-checked against the published rulebook for 25 well-known
    /// feats in `tests/sd27_feat_prerequisite_enforcement.rs`.
    #[test]
    fn a_starting_fighter_keeps_a_real_catalog_and_every_denial_states_why() {
        let input = character(1, 13, &[]);
        let facts = character_prereq_facts(&input, 1);
        let reports = evaluate_every_catalog_feat(&facts);

        // 1578 hand-authored records + the 649 corpus gap rows the feat gap
        // lane joined on (`SD31-E6-F8-001`'s original 83 + `SD31-E6-F8-002`'s
        // 242 + `SD31-E6-F2-007`'s 199 Mythic Adventures rows -- SD31-W10-
        // INTEGRATE-001 excluded 159 VISIBLE:EXPORT display-plumbing twins
        // from the original 358 -- + `SD31-E6-F8-003`'s 7 + SD-32 Gate 0
        // book-onboarding precondition's 9 inner_sea_taverns rows + SD-32 T9
        // onboarding's (card 11) 109: inner_sea_combat 23 + inner_sea_gods
        // 86). Every gap row's own `PRE`-family tokens are carried verbatim
        // into `FeatCatalogRecord::prerequisites`, so the new rows are
        // evaluated by this gate exactly like every other record — they are
        // not offered unconditionally.
        assert_eq!(reports.len(), 2227);
        let eligible = reports.iter().filter(|report| report.is_eligible).count();
        // 211 (of the original 690) + all 23 UCA Story Feats: every one of
        // UCA's records carries only a `PRETEXT:` prose prerequisite, which
        // `pre_tokens` cannot mechanically verify and therefore never
        // blocks -- so all 23 land in `met`/`unverified` rather than
        // `unmet`, exactly the same non-blocking treatment PU's own
        // `PRETEXT:` rows already get. Re-derived with this test after
        // SD28-E13 landed the UCA catalog (2026-08-03).
        // +44 with the feat gap lane's 83 corpus rows joined on (2026-08-11).
        // The load-bearing half of that figure is the other half: **39 of the
        // 83 new rows are NOT eligible** for a level-1 Fighter, each with a
        // stated reason, because the gap rows carry their corpus `PRE`-family
        // tokens verbatim. A lane that shipped rows the prerequisite gate
        // could not see would have moved this number by the full 83.
        // 553 with the gap rows alone; **552** once `PRESIZEGTEQ:` became a
        // modelled kind in the same cycle. `Awesome Blow` carries
        // `PRESIZEGTEQ:L`, and a Medium Fighter is now correctly DENIED it
        // with a stated reason instead of being offered it under an
        // unverifiable prerequisite. Modelling a token can only ever move
        // this number down, and that direction is the point.
        // +94 with `SD31-E6-F8-002`'s 242 more gap rows joined on
        // (2026-08-16): re-derived by this same test, not guessed.
        // +207 with `SD31-E6-F2-007`'s 358 Mythic Adventures rows joined on
        // (2026-08-17). Most of Mythic's own gate is `PREVARGTEQ:
        // MythicTierLevel,...` -- an unmodelled var this evaluator already
        // treats as non-blocking for every OTHER book's records
        // (`pre_tokens::tests::an_unrecognised_kind_never_blocks`), so a
        // level-1 Fighter is reported, not denied, on the mythic-tier gate
        // alone; a colliding row's OWN `PREABILITY:...,CATEGORY=FEAT,<key>`
        // clause (proven present for every collision by
        // `feats_all::tests::cross_book_key_collisions_are_exactly_the_known_set`)
        // is what still correctly denies a record whose base feat this
        // Fighter build does not hold.
        // -159 with `SD31-W10-INTEGRATE-001`'s exclusion of the
        // `VISIBLE:EXPORT` display-plumbing twins: every one of them carried
        // `prerequisites: None` (no `PRE` token at all), so every one of
        // them was trivially eligible and counted here -- removing them
        // moves this number down by exactly 159, the full twin population,
        // not a partial figure.
        // +2 with `SD31-E6-F8-003`'s 7 new gap rows joined on: 5 of the 7
        // (Greater Stylized Spell, Masked Renown, Stylized Spell Mastery,
        // Stylized Spontaneity, Demonic Obedience) are correctly DENIED --
        // each carries only modelled, AND-chained `PRE`-family tokens
        // (`PRESKILL`/`PREABILITY`/`PREDEITY`) a level-1 13-STR Fighter does
        // not meet. The other 2 (Convincing Persona, Masked Symbol) each
        // carry a `PREMULT` whose alternatives are `[PRESKILL:...]` OR
        // `[PREABILITY:1,CATEGORY=Special Ability,Vigilante ~ Dual
        // Identity]` -- an unmodelled special-ability category the engine
        // cannot verify -- so `pre_tokens`' own
        // `a_premult_with_an_unmodelled_alternative_reports_rather_than_denies`
        // rule reports rather than denies the whole clause, landing both in
        // `eligible` (unverified, not confirmed met) exactly like every
        // other unmodelled-alternative record already does.
        // +5 with SD-32 Gate 0 book-onboarding precondition's 9
        // inner_sea_taverns rows joined on: `Drinking Buddy`, `Extreme Mood
        // Swings`, `Implacable` and `Muddled Morals` carry no `PRE` token at
        // all; `Tavern Regular`'s `PREVARGTEQ:PreStatScore_CHA,14` names an
        // unmodelled variable this evaluator already treats as non-blocking
        // for every book (`pre_tokens::tests::an_unrecognised_kind_never_
        // blocks`), so it reports rather than denies. The other 4
        // (`Drunken God's Blessings`, `Drunken Sing-Along`, `Hardy Liver`,
        // `Read the Room`) each carry a modelled, AND-chained `PREDEITY`/
        // `PRESKILL`/`PREABILITY` clause this level-1 build does not meet,
        // and are correctly DENIED.
        // +54 with commit fb4f28dad's 109 new corpus gap rows joined
        // (inner_sea_combat 23 + inner_sea_gods 86, `decisions.md §17`/T9
        // card 11). Verified by class, not by trust: isolating exactly the
        // 109 keys that commit added and re-partitioning `reports` by that
        // set reproduces the pre-commit population untouched
        // (old_total_reports=2118, old_eligible=701, matching the values
        // this assertion carried before that commit) plus a clean 109-row
        // addition split 54 eligible / 55 denied -- every one of the 55
        // newly-denied rows still carries a stated reason via the
        // denial-reason loop below, run over the FULL joined `reports`,
        // covering old and new rows alike. That commit's own sweep updated
        // `reports.len()` (2118->2227, asserted above) but missed this
        // eligible-count sibling assertion -- the same class of stale
        // pinned-count-after-legitimate-growth defect `decisions.md §17a`
        // and this bundle's four prior corrections already fixed elsewhere,
        // not a real regression: most of the 109 new rows carry genuine
        // Combat-style/Aldori/Rage-class-feature `PRE`-family prerequisites
        // a fresh level-1 13-STR Fighter with no feats does not meet.
        assert_eq!(eligible, 755, "a starting Fighter's real eligible-feat count");

        for report in reports.iter().filter(|report| !report.is_eligible) {
            let reason = report.unavailable_reason().unwrap_or_default();
            assert!(
                !reason.trim().is_empty(),
                "'{}' is unavailable with no stated reason -- an unavailable affordance \
                 with no reason is the dead-affordance shape the doctrine forbids",
                report.feat_key
            );
        }
    }

    /// Meeting more prerequisites must open more feats, never fewer. Guards
    /// against an arm whose polarity is inverted: a sign error in any
    /// threshold comparison would show up as a build that qualifies for
    /// *less* as it grows.
    #[test]
    fn a_stronger_build_is_eligible_for_a_superset_of_a_weaker_ones_feats() {
        let weak = character(1, 13, &[]);
        let strong = character(6, 17, &["Power Attack", "Dodge", "Two-Weapon Fighting"]);

        let weak_facts = character_prereq_facts(&weak, 1);
        let strong_facts = character_prereq_facts(&strong, 6);

        let eligible_keys = |facts: &_| -> std::collections::BTreeSet<String> {
            evaluate_every_catalog_feat(facts)
                .into_iter()
                .filter(|report| report.is_eligible)
                .map(|report| report.feat_key)
                .collect()
        };

        let weak_keys = eligible_keys(&weak_facts);
        let strong_keys = eligible_keys(&strong_facts);
        // `Wilding` (uw_feats.lst:112) carries a real, deliberate PF1
        // ceiling -- `PRELEVEL:MAX=1`, "you were touched by nature at an
        // early age" -- available only to a 1st-level character. This is
        // not a sign error the way this test otherwise guards against: a
        // stronger (higher-level) build genuinely loses access to an
        // early-level-only feat, the one real exception to "more
        // prerequisites open strictly more feats" in the whole catalog.
        // Named explicitly rather than silently excluded from the
        // comparison, so a second such exception fails here instead of
        // being absorbed.
        let known_level_ceiling_exceptions: std::collections::BTreeSet<&str> =
            ["Wilding"].into_iter().collect();
        let lost: Vec<&String> = weak_keys
            .difference(&strong_keys)
            .filter(|key| !known_level_ceiling_exceptions.contains(key.as_str()))
            .collect();
        assert!(
            lost.is_empty(),
            "a stronger build lost access to feats the weaker one had (beyond the known \
             PRELEVEL:MAX exceptions): {lost:?}"
        );
        assert!(
            strong_keys.len() > weak_keys.len(),
            "the stronger build must open feats the weaker one could not take"
        );
    }

    /// Every record with no corpus prerequisite is unconditionally
    /// available -- the engine must not invent a gate where the corpus has
    /// none.
    #[test]
    fn records_with_no_corpus_prerequisite_are_always_eligible() {
        let input = character(1, 13, &[]);
        let facts = character_prereq_facts(&input, 1);
        for report in evaluate_every_catalog_feat(&facts) {
            if report.prerequisite_token_count == 0 {
                assert!(report.is_eligible, "'{}' has no prerequisites", report.feat_key);
                assert!(report.unmet.is_empty());
                assert!(report.unverified.is_empty());
            }
        }
    }

    /// ARG's feats are race-gated and that gate now bites: a Human cannot
    /// take Armor of the Pit, a Tiefling can.
    #[test]
    fn an_arg_race_gate_is_enforced_in_both_directions() {
        let mut human = character(1, 13, &[]);
        let facts = character_prereq_facts(&human, 1);
        let report = evaluate_feat_key_prerequisites("Armor of the Pit", &facts).unwrap();
        assert!(!report.is_eligible);
        assert!(report.unavailable_reason().unwrap().contains("Tiefling"));

        human.chosen.race_id = "race:tiefling".to_owned();
        let facts = character_prereq_facts(&human, 1);
        let report = evaluate_feat_key_prerequisites("Armor of the Pit", &facts).unwrap();
        assert!(report.is_eligible, "unmet: {:?}", report.unmet);
    }

    /// An unknown feat id resolves to `None` rather than to a fabricated
    /// pass or fail.
    #[test]
    fn an_unknown_feat_id_resolves_to_nothing() {
        let input = character(1, 13, &[]);
        let facts = character_prereq_facts(&input, 1);
        assert_eq!(evaluate_feat_key_prerequisites("Not A Real Feat", &facts), None);
    }
}
