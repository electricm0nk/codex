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
//! [`evaluate_catalog_feat_prerequisites`] below is the real check. It reads the CONVERTED
//! gate off each record's `data/sheet_rules/` rule and decides it through the same
//! [`evaluate_applies`](crate::rules_core::sheet_rule::evaluate_applies) the sheet renders
//! through (SD-35 `AT-35-E6-001`; `decisions.md` §11 -- nothing on the live side reads the
//! ingest format). Until that cycle it parsed the `PRE`-family token text at run time; the
//! parser it used is converter and oracle code now, kept under `src/pcgen_import/` and read
//! by the converter and the oracle harness, never by the product.
//!
//! The three-outcome contract is unchanged and is `feat_prereqs::converted_gate`'s own
//! subject: only a definitively unmet term blocks, and a term over a fact the character
//! record does not carry is reported, never refused.
//!
//! ## Why the SD-20 functions are still here
//!
//! They are a different, narrower question with their own callers and their
//! own tests, and deleting them is not this cycle's job. Nothing new calls
//! them; `evaluate_catalog_feat_prerequisites` does not route through them.

pub mod combat;
pub mod converted_gate;
pub mod general;
pub mod item_creation;
pub mod metamagic;

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::pilot_compute::PilotBaseChassisComputation;
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::feats::FeatCategory;
use crate::rules_core::rules_tables::feats_all::{all_feat_tables, FeatCatalogRecord};
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::sheet_rule::{
    held_set, split_rule_id, Applies, CharacterFacts, HeldSeed, HeldSet, SheetRule,
    SheetRulePackage,
};
use converted_gate::TermVerdict;

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
    /// How many top-level terms the record's CONVERTED gate carries (SD-35 `AT-35-E6-001`;
    /// before that cycle, how many `PRE`-family tokens the source record carried). `0` means
    /// the record genuinely has no prerequisites -- or, when [`Self::converted`] is false,
    /// that there was no gate to count.
    pub prerequisite_token_count: usize,
    /// Whether `data/sheet_rules/` carries a converted rule for this record at all. `false`
    /// is a number to report, never an exemption: the record is still offered, with one
    /// `unverified` note saying so, because an absent conversion is a statement about this
    /// repo and not about the character.
    pub converted: bool,
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

/// One character's prerequisite context, entirely on the live side (SD-35 `AT-35-E6-001`).
///
/// The three things a converted gate is decided against: the `data/sheet_rules/` package, the
/// character's held set, and their facts — the same three the sheet itself is rendered from
/// (`character_hub::sheet_lines_for`, `level_up_option_filter::filter_option_pool`). One
/// evaluator, several consumers; no ingest-format token is read to build or use it.
///
/// This replaces the token-side `CharacterPrereqFacts` snapshot, which held a hand-modelled
/// projection of the character purely so the `PRE*` parser had something to compare against.
pub struct PrereqFacts {
    package: &'static SheetRulePackage,
    held: HeldSet,
    facts: CharacterFacts,
    /// Folded catalog identity -> the converted rule that record's gate lives on. Built once
    /// per context: `evaluate_every_catalog_feat` asks it 2,227 times, and a linear scan of the
    /// package's 2,753 converted feat rules per question is 6M string folds per character.
    feat_index: std::collections::BTreeMap<String, crate::rules_core::sheet_rule::RuleId>,
}

impl PrereqFacts {
    /// From an already-loaded package plus the chassis computation the caller has in hand.
    ///
    /// `extra_race_traits` is the race resolver's applied-trait key list, exactly as
    /// `PilotBaseChassisComputation::with_sheet_rules` takes it; pass `&[]` when the caller
    /// has not resolved them (a racial-trait gate then reports unmet rather than met, the
    /// same way it does on the sheet).
    pub fn new(
        package: &'static SheetRulePackage,
        input: &CharacterInput,
        computation: &PilotBaseChassisComputation,
        extra_race_traits: &[String],
    ) -> PrereqFacts {
        let mut seed = HeldSeed::from_character(input, computation);
        seed.race_traits.extend(extra_race_traits.iter().cloned());
        // A feat recorded with its sub-choice -- `"Weapon Focus (Longbow)"`, the shape the
        // catalog picker sends -- names the same converted record as `weapon_focus`. The
        // seed carries both so a gate over the feat resolves for either shape; a record that
        // genuinely is its own feat keeps its own slug and is unaffected.
        let bases: Vec<String> = input
            .chosen
            .selected_feats
            .iter()
            .filter_map(|f| f.split_once('(').map(|(base, _)| base.trim().to_owned()))
            .map(|base| crate::rules_core::sheet_rule::id_slug(&base))
            .collect();
        seed.feats.extend(bases);
        let facts = CharacterFacts::from_character(input, computation);
        let held = held_set(package, &seed, &facts);
        PrereqFacts { package, held, facts, feat_index: feat_rule_index(package) }
    }

    /// The same, reading this checkout's own `data/sheet_rules/` package
    /// ([`corpus_loader::live_sheet_rules`](crate::rules_core::corpus_loader::live_sheet_rules)).
    /// `None` when that package is unavailable — never a verdict.
    pub fn from_character(
        input: &CharacterInput,
        computation: &PilotBaseChassisComputation,
        extra_race_traits: &[String],
    ) -> Option<PrereqFacts> {
        let package = crate::rules_core::corpus_loader::live_sheet_rules()?;
        Some(PrereqFacts::new(package, input, computation, extra_race_traits))
    }

    pub fn package(&self) -> &'static SheetRulePackage {
        self.package
    }

    pub fn held(&self) -> &HeldSet {
        &self.held
    }

    pub fn facts(&self) -> &CharacterFacts {
        &self.facts
    }
}

/// Every converted feat rule, keyed by the folded identity a catalog record is matched on.
///
/// The fold is [`feat_identity::fold`](crate::rules_core::feat_identity::fold) -- the one place
/// in this codebase that decides whether two feat identifiers name the same feat. A record's
/// slug and its display label are both indexed, because a catalog `key` is genuinely either
/// shape. A `#`-suffixed sibling is never a principal record and is skipped, and an entry
/// already present is never overwritten: `core_rulebook` sorts first, so the CRB printing of a
/// re-listed feat wins, the same preference [`SheetRulePackage::find`] applies.
fn feat_rule_index(
    package: &SheetRulePackage,
) -> std::collections::BTreeMap<String, crate::rules_core::sheet_rule::RuleId> {
    use crate::rules_core::feat_identity;
    let mut out = std::collections::BTreeMap::new();
    for rule in package.rules_of_kind("feat") {
        if rule.id.contains('#') {
            continue;
        }
        let (_, _, slug) = split_rule_id(&rule.id);
        out.entry(feat_identity::fold(slug)).or_insert_with(|| rule.id.clone());
        if !rule.label.is_empty() {
            out.entry(feat_identity::fold(&rule.label)).or_insert_with(|| rule.id.clone());
        }
    }
    out
}

/// The converted rule for one catalog record, or `None` when the package carries none.
fn converted_feat_rule<'a>(
    facts: &'a PrereqFacts,
    feat_key: &str,
) -> Option<&'a SheetRule> {
    use crate::rules_core::feat_identity;
    let id = facts.feat_index.get(&feat_identity::fold(feat_key))?;
    facts.package.rule(id)
}

/// Evaluates one catalog record's real prerequisites, read off its CONVERTED
/// [`Applies`] gate, against `facts`.
///
/// A record whose converted gate is [`Applies::Always`] is eligible with an empty report --
/// that is the corpus saying the feat has no prerequisites, and 91 of the catalog's 690
/// records really are like that.
///
/// A record the package carries no converted rule for is eligible with ONE `unverified`
/// entry naming that. It is never a refusal: an absent conversion is a statement about this
/// repo, not about the character.
pub fn evaluate_catalog_feat_prerequisites(
    record: &FeatCatalogRecord,
    rule_set: RuleSetId,
    facts: &PrereqFacts,
) -> FeatPrerequisiteReport {
    let Some(rule) = converted_feat_rule(facts, record.key) else {
        return FeatPrerequisiteReport {
            feat_key: record.key.to_owned(),
            rule_set,
            is_eligible: true,
            prerequisite_token_count: 0,
            converted: false,
            met: Vec::new(),
            unmet: Vec::new(),
            unverified: vec![PrerequisiteWarning {
                message: format!(
                    "not verified: no converted rule for this record in data/sheet_rules/ ({})",
                    record.key
                ),
            }],
        };
    };
    report_from_gate(record.key, rule_set, &rule.applies, facts)
}

/// The shared projection of a converted gate onto the report shape.
fn report_from_gate(
    feat_key: &str,
    rule_set: RuleSetId,
    gate: &Applies,
    facts: &PrereqFacts,
) -> FeatPrerequisiteReport {
    let verdicts = converted_gate::verdicts(facts.package, &facts.held, &facts.facts, gate);
    let mut met = Vec::new();
    let mut unmet = Vec::new();
    let mut unverified = Vec::new();
    for verdict in &verdicts {
        match verdict {
            TermVerdict::Met(words) => met.push(words.clone()),
            TermVerdict::Unmet(reason) => unmet.push(FailedPrerequisite { reason: reason.clone() }),
            TermVerdict::Unverified(note) => {
                unverified.push(PrerequisiteWarning { message: note.clone() })
            }
            TermVerdict::Informational(_) => {}
        }
    }
    FeatPrerequisiteReport {
        feat_key: feat_key.to_owned(),
        rule_set,
        is_eligible: unmet.is_empty(),
        prerequisite_token_count: verdicts.len(),
        converted: true,
        met,
        unmet,
        unverified,
    }
}

/// Every catalog record's verdict for one character, in book order --
/// what a feat picker needs to render 690 rows with the unavailable ones
/// greyed and reasoned.
pub fn evaluate_every_catalog_feat(facts: &PrereqFacts) -> Vec<FeatPrerequisiteReport> {
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
    facts: &PrereqFacts,
) -> Option<FeatPrerequisiteReport> {
    use crate::rules_core::feat_identity;

    all_feat_tables().iter().find_map(|book| {
        book.entries
            .iter()
            .find(|entry| feat_identity::same(entry.key, feat_key))
            .map(|entry| evaluate_catalog_feat_prerequisites(entry, book.rule_set, facts))
    })
}

/// Builds the prerequisite context from chosen input, computing the chassis this checkout's
/// own engine computes for it. The convenience the tests and the `*_at_root` desktop seams
/// share; a caller that already has a computation uses [`PrereqFacts::new`] instead and does
/// not compute twice.
pub fn character_prereq_facts(input: &CharacterInput) -> Option<PrereqFacts> {
    let computation = crate::rules_core::pilot_compute::compute_pilot_base_chassis(input);
    PrereqFacts::from_character(input, &computation, &[])
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
        let facts = character_prereq_facts(&input).expect("data/sheet_rules/ must be loadable in a repo checkout");
        let report = evaluate_feat_key_prerequisites("Improved Two-Weapon Fighting", &facts)
            .expect("the feat is in the catalog");

        assert!(!report.is_eligible);
        let reason = report.unavailable_reason().expect("an ineligible feat must state a reason");
        // SD-35 `AT-35-E6-001`: the same three requirements, now in the CONVERTED gate's own
        // words rather than the token evaluator's. The requirement AND the character's own
        // value both print -- a refusal a player cannot act on is as bad as no refusal.
        assert!(reason.contains("base attack bonus at least 6"), "{reason}");
        assert!(reason.contains("this character: 1"), "{reason}");
        assert!(reason.contains("Two-Weapon Fighting"), "{reason}");
        assert!(reason.contains("Dexterity"), "{reason}");
        assert!(reason.contains("17"), "{reason}");
    }

    /// ...and the build that legitimately qualifies is not blocked.
    #[test]
    fn a_fighter_6_with_dex_17_and_two_weapon_fighting_can_take_it() {
        let input = character(6, 17, &["Two-Weapon Fighting"]);
        let facts = character_prereq_facts(&input).expect("data/sheet_rules/ must be loadable in a repo checkout");
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
        let facts = character_prereq_facts(&input).expect("data/sheet_rules/ must be loadable in a repo checkout");
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
        // the converted gate reports rather than verifies, and therefore never
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
        // (`converted_gate::unverifiable_reason` names the fact), so a
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
        // cannot verify -- so the converted gate's own
        // "a term over a fact the record does not carry is reported, never refused"
        // rule reports rather than denies the whole clause, landing both in
        // `eligible` (unverified, not confirmed met) exactly like every
        // other unmodelled-alternative record already does.
        // +5 with SD-32 Gate 0 book-onboarding precondition's 9
        // inner_sea_taverns rows joined on: `Drinking Buddy`, `Extreme Mood
        // Swings`, `Implacable` and `Muddled Morals` carry no `PRE` token at
        // all; `Tavern Regular`'s `PREVARGTEQ:PreStatScore_CHA,14` names an
        // unmodelled variable this evaluator already treats as non-blocking
        // for every book (`converted_gate`'s own unverifiable classification),
        // so it reports rather than denies. The other 4
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
        // **755 -> 549 with SD-35 `AT-35-E6-001`**, which moved this gate from the ingest
        // format's `PRE`-family token text to each record's CONVERTED `applies`. Re-derived
        // by this test, not adjusted to fit. The direction is the point (`decisions.md` §1):
        // the converted gate decides shapes the token evaluator reported rather than
        // checked, so a build is offered fewer feats it never qualified for. Two families
        // account for nearly all of it, each verified against the rulebook:
        //  * **ability-score and rank thresholds the token evaluator passed.** `Combat
        //    Expertise` requires Int 13 and this Fighter has Int 10; `Desert Dweller`
        //    requires 1 Survival rank and Con 13 and this Fighter has 0 and 12. Each is now
        //    denied with the requirement AND the character's own value in the line.
        //  * **holdings named as one concrete record.** `Extra Rage Power`, `Extra
        //    Discovery`, `Extra Grit` and the Ultimate Psionics families name a class
        //    feature this Fighter does not hold; the held set grows through the package's
        //    own grant edges, so "you do not hold it" is a real verdict for a record the
        //    fixpoint could have granted.
        // What did NOT tighten, deliberately: a `Holds` counting a "special ability" POOL by
        // name stays reported rather than refused -- see `converted_gate::ROSTERED_POOLS`
        // and `a_class_feature_pool_holding_is_reported_not_refused` below for the measured
        // reason.
        assert_eq!(eligible, 549, "a starting Fighter's real eligible-feat count");
        // A catalog record `data/sheet_rules/` carries no converted rule for is a number to
        // report, never an exemption: it is still offered, with one "not verified" note.
        let unconverted = reports.iter().filter(|report| !report.converted).count();
        assert_eq!(unconverted, 21, "catalog records with no converted rule");

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

        let weak_facts = character_prereq_facts(&weak).expect("data/sheet_rules/ must be loadable in a repo checkout");
        let strong_facts = character_prereq_facts(&strong).expect("data/sheet_rules/ must be loadable in a repo checkout");

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
        // `Fey Foundling` (`isw_feats.lst`) joins it with SD-35 `AT-35-E6-001`: it carries
        // the same `PRELEVEL:MAX=1` ceiling ("you must take this feat at 1st level"), which
        // the converted gate states as `at least 1 of: requires Fey Foundling, character
        // level at most 1` and now enforces. The token evaluator listed maximum-level
        // ceilings as unmodelled, so a 6th-level character was offered a 1st-level-only feat
        // until this cycle. Re-derive:
        // `python3 -c "import json;print(json.load(open('data/sheet_rules/inner_sea_world_guide/feat/fey_foundling.json'))[0]['applies'])"`
        let known_level_ceiling_exceptions: std::collections::BTreeSet<&str> =
            ["Wilding", "Fey Foundling"].into_iter().collect();
        let lost: Vec<&String> = weak_keys
            .difference(&strong_keys)
            .filter(|key| !known_level_ceiling_exceptions.contains(key.as_str()))
            .collect();
        assert!(
            lost.is_empty(),
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   PRELEVEL:MAX exceptions
            "a stronger build lost access to feats the weaker one had (beyond the known): {lost:?}"
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
        let facts = character_prereq_facts(&input).expect("data/sheet_rules/ must be loadable in a repo checkout");
        for report in evaluate_every_catalog_feat(&facts) {
            if report.converted && report.prerequisite_token_count == 0 {
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
        let facts = character_prereq_facts(&human).expect("data/sheet_rules/ must be loadable in a repo checkout");
        let report = evaluate_feat_key_prerequisites("Armor of the Pit", &facts).unwrap();
        assert!(!report.is_eligible);
        assert!(report.unavailable_reason().unwrap().contains("Tiefling"));

        human.chosen.race_id = "race:tiefling".to_owned();
        let facts = character_prereq_facts(&human).expect("data/sheet_rules/ must be loadable in a repo checkout");
        let report = evaluate_feat_key_prerequisites("Armor of the Pit", &facts).unwrap();
        assert!(report.is_eligible, "unmet: {:?}", report.unmet);
    }

    /// The measured reason `converted_gate::ROSTERED_POOLS` leaves the "special ability"
    /// pool out, kept as a test so the decision cannot rot into a habit.
    ///
    /// The held set grows class features through the package's own grant edges, and for
    /// three of the four classes below that genuinely reaches the holding the feat asks
    /// for. It does not reach a Cleric's Channel Positive Energy, whose grant is
    /// conditioned on an alignment the character record does not carry -- so a pool count
    /// is a partial roster, and a partial roster cannot produce an honest refusal on a path
    /// that refuses a save. Every one of the four is therefore reported, not refused.
    #[test]
    fn a_class_feature_pool_holding_is_reported_not_refused() {
        for (class_id, level, feat) in [
            ("class:barbarian", 1u8, "Extra Rage"),
            ("class:bard", 1, "Extra Performance"),
            ("class:paladin", 2, "Extra Lay On Hands"),
            ("class:cleric", 1, "Extra Channel"),
        ] {
            let mut input = character(level, 13, &[]);
            input.chosen.class_levels[0].class_id = class_id.to_owned();
            input.chosen.class_levels[0].level = level;
            let facts = character_prereq_facts(&input)
                .expect("data/sheet_rules/ must be loadable in a repo checkout");
            let report = evaluate_feat_key_prerequisites(feat, &facts)
                .unwrap_or_else(|| panic!("{feat} is in the catalog"));
            assert!(
                report.is_eligible,
                "{class_id} must not be REFUSED {feat} over a pool the engine only partly \
                 rosters: {:?}",
                report.unmet
            );
            assert!(
                report.unverified.iter().any(|w| w.message.contains("roster of this pool")),
                "{class_id}/{feat} must SAY the pool count was not checked: {:?}",
                report.unverified
            );
        }
    }

    /// An unknown feat id resolves to `None` rather than to a fabricated
    /// pass or fail.
    #[test]
    fn an_unknown_feat_id_resolves_to_nothing() {
        let input = character(1, 13, &[]);
        let facts = character_prereq_facts(&input).expect("data/sheet_rules/ must be loadable in a repo checkout");
        assert_eq!(evaluate_feat_key_prerequisites("Not A Real Feat", &facts), None);
    }
}
