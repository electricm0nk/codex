//! GE-06 pilot view-model contract over the merged headless receipt.
//!
//! Projects the bounded rules-core receipt into a machine-checkable UI-consumer
//! surface that preserves real computed values when available and explicit blocked
//! posture plus diagnostics when computation is blocked. This adapter adds no UI,
//! no transport, and no new rules truth.

use std::collections::BTreeMap;

use super::pilot_compute::{
    AbilityModifiers, BaseSaves, ComputationDiagnostic, ComputationExplanation,
    HeadlessReceiptStatus, PilotHeadlessReceipt, SelectedSkillModifiers,
};
use super::pilot_failure::{FailureClassifier, PrimaryOwner};
use super::spellbook::SpellbookCoverage;

/// Bounded GE-06 pilot view model derived from the merged headless receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotViewModel {
    pub case_id: Option<String>,
    pub source_package_id: String,
    pub status: HeadlessReceiptStatus,
    pub primary_owner: PrimaryOwner,
    pub snapshot: Option<PilotSnapshot>,
    pub explanations: Vec<ComputationExplanation>,
    pub diagnostics: Vec<ComputationDiagnostic>,
}

/// Computed pilot snapshot emitted only when the receipt is `Computed`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotSnapshot {
    pub ability_modifiers: AbilityModifiers,
    pub base_attack_bonus: i16,
    pub base_saves: BaseSaves,
    pub combat: PilotCombatViewModel,
    pub defense: PilotDefenseViewModel,
    pub skill: PilotSkillViewModel,
    /// The character's animal companion or mount, when this build grounds
    /// one -- `None` for every class that does not, never an empty or
    /// zeroed stat block. See `PilotCompanionViewModel`.
    pub companion: Option<PilotCompanionViewModel>,
    /// The character's spellbook coverage (spell save DCs, slots
    /// total/used), when `spellbook::compute_spellbook_coverage` grounded
    /// at least one spell slot or save DC for this build -- `None` for a
    /// non-caster or a build with no resolved spells, never a zeroed
    /// block.
    ///
    /// `None` unconditionally for `PilotSnapshot::from_receipt` below: the
    /// merged headless receipt carries no `SourcePackageContent`, and
    /// `compute_spellbook_coverage` requires the corpus to resolve a
    /// spell's school/level. The one production call site with a real
    /// corpus is `pf1_adapter::resolve_unified_pilot_snapshot`, which
    /// builds this field itself via `PilotSpellbookViewModel::from_coverage`
    /// (epic-31-spell-wiring; see that function's doc comment for the
    /// "twin problem" this closes -- `decisions.md §29.1`/§29.2`,
    /// `docs/release/SD-28-ultimate-book-content-ingestion/artifacts/e14-harness-widening.md`).
    pub spellbook: Option<PilotSpellbookViewModel>,
}

/// One resolved spell's magnitude, projected for display: its own level
/// (the figure the save DC and slot cost are both attributable to) and its
/// own save DC when that spell's class grounds one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotSpellSaveDc {
    pub class_id: String,
    pub dc: u8,
}

/// Bounded spellbook surface for the pilot view model
/// (epic-31-spell-wiring). Projects `spellbook::SpellbookCoverage` --
/// already a real, magnitude-bearing computation (`SpellEffect.level` read
/// from the canonical spell-list table, `slots_total`/`slots_used`/
/// `spell_save_dc` derived from it) -- into a display-ready surface. Adds
/// no rules truth of its own; every value here comes from the engine's own
/// `compute_spellbook_coverage`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotSpellbookViewModel {
    /// Total spell slots per spell level (0-9), from class table plus
    /// ability-score bonus slots. Only levels the character's class(es)
    /// actually grant appear.
    pub slots_total: BTreeMap<u8, u8>,
    /// Spell slots already filled by a prepared or known spell, per level.
    pub slots_used: BTreeMap<u8, u8>,
    /// The saving-throw DC for the highest-level spell of that class
    /// present in the character's spellbook (`10 + spell level + casting
    /// ability modifier`), keyed by the granting class id.
    pub spell_save_dc: Vec<PilotSpellSaveDc>,
}

impl PilotSpellbookViewModel {
    /// `None` when the coverage carries no slot or save-DC magnitude at
    /// all (a non-caster, or a caster with no spell yet resolved against
    /// the corpus) -- never an empty-but-present block, matching the
    /// `damage_reduction`/`companion` "absent, not zeroed" convention
    /// already established on this struct.
    pub fn from_coverage(coverage: &SpellbookCoverage) -> Option<Self> {
        if coverage.slots_total.is_empty()
            && coverage.slots_used.is_empty()
            && coverage.spell_save_dc.is_empty()
        {
            return None;
        }
        Some(Self {
            slots_total: coverage.slots_total.clone(),
            slots_used: coverage.slots_used.clone(),
            spell_save_dc: coverage
                .spell_save_dc
                .iter()
                .map(|(class_id, &dc)| PilotSpellSaveDc {
                    class_id: class_id.clone(),
                    dc,
                })
                .collect(),
        })
    }
}

/// One grounded companion statistic, projected from its own
/// `ComputationExplanation` with the value and the engine's own derivation
/// prose carried verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotCompanionStat {
    /// The honest name of what `value` actually is -- see
    /// `COMPANION_STAT_ROWS` for why this is not simply the record's id.
    pub label: String,
    pub value: i16,
    /// The explanation record's `detail`, verbatim. Never rewritten: it is
    /// the engine's own corpus-cited derivation, and a paraphrase here
    /// would be a second, unverified source of rules prose.
    pub detail: String,
}

/// A character's animal companion or mount, projected from the family of
/// `class_chassis.<class>.<role>.*` explanation records the engine already
/// grounds (Druid's and Hunter's Wolf animal companion, Cavalier's Horse
/// mount), across all twenty master levels.
///
/// Promoted to a first-class `PilotSnapshot` field for the same reason
/// `PilotDefenseViewModel::damage_reduction` was: the data was fully
/// computed and correct, but lived only as raw explanation records, so no
/// consumer downstream of the engine had a structured surface to render.
///
/// This projection adds no rules truth. Every value and every line of
/// prose here comes verbatim from a record the engine already emitted; the
/// projection only selects, labels and orders them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotCompanionViewModel {
    /// The owning class, e.g. `"Druid"` -- read from the record id, not
    /// from the character's class list, so it always names the class that
    /// actually grounded this companion.
    pub owner_class_label: String,
    /// What this class calls the creature: `"Animal Companion"` or
    /// `"Mount"`.
    pub role_label: String,
    /// The canonical species this bounded seam grounds: `"Wolf"` or
    /// `"Horse"`.
    pub species: String,
    /// The recognition record's own detail -- the companion's summary line,
    /// including the "a wholly separate creature, none of these values are
    /// applied to the character's own totals" framing that keeps a reader
    /// from mistaking the block for the character's own statistics.
    pub summary_detail: String,
    /// Every grounded statistic, in canonical reading order. Only rows the
    /// engine actually emitted appear -- a missing record is omitted, never
    /// zero-filled.
    pub stats: Vec<PilotCompanionStat>,
    /// Details of the companion's provably-vacuous named abilities (Link,
    /// Share Spells), in emission order.
    pub notes: Vec<String>,
    /// The engine's own non-blocking `advancement_absent` diagnostic
    /// message: the honest list of companion columns deliberately left
    /// ungrounded because nothing in this codebase consumes them (bonus
    /// tricks, companion skills and feats, the player-chosen stat increase
    /// at master levels 4/9/14/20, the size advance, and the named
    /// abilities Evasion/Devotion/Multiattack and friends).
    ///
    /// It travels here rather than with the diagnostics because it is
    /// non-blocking, and a `Computed` load reports no diagnostics at all
    /// (`character_hub.rs`'s `load_saved_character` returns `Vec::new()` on
    /// the success path) -- so without this field the player would never
    /// see it.
    pub advancement_note: Option<String>,
}

/// The recognition-record suffixes that mark a grounded companion stat
/// block, each paired with the species it names. A record family is only
/// recognized through one of these, so an unrelated `class_chassis.*`
/// record can never be mistaken for a companion.
const COMPANION_RECOGNITION_SUFFIXES: [(&str, &str); 2] =
    [(".wolf_stat_block", "Wolf"), (".horse_stat_block", "Horse")];

/// Every companion statistic this projection renders, in canonical reading
/// order, each paired with the honest label for the value that record
/// actually carries.
///
/// The labels are deliberately not mechanical title-casings of the record
/// ids, because two ids would then read as something they are not:
///
/// * `base_attack_bonus` carries the companion's Hit-Dice-derived base
///   attack bonus **plus its Strength modifier** (`ground_wolf_companion_stat_block`
///   computes `companion_base_attack_bonus + strength_modifier`), so the
///   honest label is `"Attack Bonus"`; and
/// * `bite_attack`/`hoof_attack` carry only the flat 1.5x-Strength damage
///   bonus -- the die itself is named in the detail text and no attack or
///   damage is ever rolled -- so the label says "Damage Bonus" rather than
///   implying a resolved attack.
///
/// A class grounds either a bite or a hoof, never both, so both rows can
/// sit in one ordered table.
const COMPANION_STAT_ROWS: [(&str, &str); 8] = [
    ("hit_points", "Hit Points"),
    ("armor_class", "Armor Class"),
    ("base_attack_bonus", "Attack Bonus"),
    ("bite_attack", "Bite Damage Bonus"),
    ("hoof_attack", "Hoof Damage Bonus"),
    ("base_save.fortitude", "Fortitude Save"),
    ("base_save.reflex", "Reflex Save"),
    ("base_save.will", "Will Save"),
];

/// `"animal_companion"` -> `"Animal Companion"`, `"druid"` -> `"Druid"`.
fn title_case_identifier(raw: &str) -> String {
    raw.split('_')
        .map(|word| {
            let mut characters = word.chars();
            match characters.next() {
                Some(first) => first.to_uppercase().collect::<String>() + characters.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

impl PilotCompanionViewModel {
    /// `pub` because `PilotSnapshot::from_receipt` is not the only place a
    /// `PilotSnapshot` is built: the desktop crate's
    /// `pf1_adapter::resolve_unified_pilot_snapshot` -- the path
    /// `load_saved_character` actually takes -- assembles its own from the
    /// same receipt. That call site must project the companion through
    /// this one function rather than re-deriving it, or the live app and
    /// the view model would disagree about what the character has.
    pub fn from_receipt(receipt: &PilotHeadlessReceipt) -> Option<Self> {
        let explanations = &receipt.computation.explanations;

        // The recognition record is what makes a companion a companion --
        // find it first, and take both the id prefix every sibling record
        // shares and the species from it.
        let (chassis_prefix, species, summary_detail) =
            explanations.iter().find_map(|explanation| {
                if !explanation.id.starts_with("class_chassis.") {
                    return None;
                }
                COMPANION_RECOGNITION_SUFFIXES
                    .iter()
                    .find_map(|(suffix, species)| {
                        explanation.id.strip_suffix(suffix).map(|prefix| {
                            (
                                prefix.to_owned(),
                                (*species).to_owned(),
                                explanation.detail.clone(),
                            )
                        })
                    })
            })?;

        // `class_chassis.druid.animal_companion` -> `druid`, `animal_companion`.
        let (owner_slug, role_slug) = chassis_prefix
            .strip_prefix("class_chassis.")
            .and_then(|remainder| remainder.split_once('.'))?;
        // The named-ability and advancement records the engine emits for
        // the same creature live under `class_feature.`, not
        // `class_chassis.`.
        let feature_prefix = format!("class_feature.{owner_slug}.{role_slug}");

        let stats = COMPANION_STAT_ROWS
            .iter()
            .filter_map(|(suffix, label)| {
                let id = format!("{chassis_prefix}.{suffix}");
                explanations
                    .iter()
                    .find(|explanation| explanation.id == id)
                    .map(|explanation| PilotCompanionStat {
                        label: (*label).to_owned(),
                        value: explanation.value,
                        detail: explanation.detail.clone(),
                    })
            })
            .collect();

        let notes = explanations
            .iter()
            .filter(|explanation| {
                explanation.id.starts_with(&feature_prefix) && explanation.id.ends_with("_vacuous")
            })
            .map(|explanation| explanation.detail.clone())
            .collect();

        let advancement_note = receipt
            .computation
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.id == format!("{feature_prefix}.advancement_absent"))
            .map(|diagnostic| diagnostic.message.clone());

        Some(Self {
            owner_class_label: title_case_identifier(owner_slug),
            role_label: title_case_identifier(role_slug),
            species,
            summary_detail,
            stats,
            notes,
            advancement_note,
        })
    }
}

/// Bounded combat surface for the pilot view model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotCombatViewModel {
    pub baseline_melee_attack_bonus: i16,
}

/// Bounded defense surface for the pilot view model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotDefenseViewModel {
    pub baseline_armor_class: i16,
    pub total_save: BaseSaves,
    /// The flat DR magnitude from a grounded class-feature DR explanation
    /// (currently only Barbarian's `class_feature.barbarian.damage_reduction`
    /// -- v0.6 alpha swarm, risks-and-open-questions.md item 6), or `None`
    /// when no such record is present or its magnitude is the level-gate
    /// absence value of 0 (real PF1 has no "DR 0"; omitted, not zeroed, per
    /// this codebase's existing convention for absent-vs-zero facts). This
    /// is a bounded display-only value: no damage-resolution engine applies
    /// it to any actual incoming-damage total.
    pub damage_reduction: Option<i16>,
}

/// Bounded skill surface for the pilot view model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotSkillViewModel {
    pub selected_modifier: SelectedSkillModifiers,
}

impl PilotViewModel {
    /// Project the merged headless receipt into the bounded pilot view-model contract.
    pub fn from_receipt(receipt: &PilotHeadlessReceipt) -> Self {
        let primary_owner = FailureClassifier::new(receipt).primary_owner();
        let snapshot = match receipt.status {
            HeadlessReceiptStatus::Computed => Some(PilotSnapshot::from_receipt(receipt)),
            HeadlessReceiptStatus::Blocked => None,
        };

        Self {
            case_id: receipt.case_id.clone(),
            source_package_id: receipt.source_package_id.clone(),
            status: receipt.status,
            primary_owner,
            snapshot,
            explanations: receipt.computation.explanations.clone(),
            diagnostics: receipt.computation.diagnostics.clone(),
        }
    }
}

impl PilotSnapshot {
    fn from_receipt(receipt: &PilotHeadlessReceipt) -> Self {
        Self {
            ability_modifiers: receipt.computation.ability_modifiers,
            base_attack_bonus: receipt.computation.base_attack_bonus,
            base_saves: receipt.computation.base_saves,
            combat: PilotCombatViewModel {
                baseline_melee_attack_bonus: receipt.computation.baseline_melee_attack_bonus,
            },
            defense: PilotDefenseViewModel {
                baseline_armor_class: receipt.computation.baseline_armor_class,
                total_save: receipt.computation.total_saves,
                damage_reduction: receipt
                    .computation
                    .explanations
                    .iter()
                    .find(|explanation| explanation.id == "class_feature.barbarian.damage_reduction")
                    .map(|explanation| explanation.value)
                    .filter(|&value| value > 0),
            },
            skill: PilotSkillViewModel {
                selected_modifier: receipt.computation.selected_skill_modifiers,
            },
            companion: PilotCompanionViewModel::from_receipt(receipt),
            spellbook: None,
        }
    }
}

/// v0.6 alpha swarm (risks-and-open-questions.md item 6): DR data already
/// existed as a raw `ComputationExplanation` (Barbarian's
/// `class_feature.barbarian.damage_reduction`, `pilot_compute.rs`) but
/// wasn't promoted to a first-class `PilotSnapshot` field, so the Defense
/// tab had nothing structured to render. These tests exercise
/// `PilotSnapshot::from_receipt` directly with a synthetic receipt (the
/// production compute path never happens to produce this state today,
/// since Barbarian isn't yet a chassis-dispatch-supported class -- see
/// `has_supported_class_chassis` -- so this proves the wiring itself,
/// independent of when/whether Barbarian becomes chassis-supported).
#[cfg(test)]
mod damage_reduction_exposure_tests {
    use super::*;
    use crate::rules_core::pilot_compute::PilotBaseChassisComputation;

    fn receipt_with_explanations(explanations: Vec<ComputationExplanation>) -> PilotHeadlessReceipt {
        PilotHeadlessReceipt {
            case_id: None,
            source_package_id: "test".to_owned(),
            status: HeadlessReceiptStatus::Computed,
            computation: PilotBaseChassisComputation {
                ability_modifiers: AbilityModifiers::default(),
                base_attack_bonus: 0,
                base_saves: BaseSaves::default(),
                baseline_melee_attack_bonus: 0,
                baseline_armor_class: 0,
                total_saves: BaseSaves::default(),
                selected_skill_modifiers: SelectedSkillModifiers::default(),
                explanations,
                diagnostics: Vec::new(),
            },
        }
    }

    #[test]
    fn surfaces_a_real_grounded_damage_reduction_value() {
        let receipt = receipt_with_explanations(vec![ComputationExplanation {
            id: "class_feature.barbarian.damage_reduction".to_owned(),
            value: 3,
            detail: "Barbarian DR 3/-".to_owned(),
        }]);

        let snapshot = PilotSnapshot::from_receipt(&receipt);

        assert_eq!(snapshot.defense.damage_reduction, Some(3));
    }

    #[test]
    fn omits_a_zero_level_gate_absence_value_rather_than_surfacing_a_fake_zero() {
        let receipt = receipt_with_explanations(vec![ComputationExplanation {
            id: "class_feature.barbarian.damage_reduction".to_owned(),
            value: 0,
            detail: "Barbarian DR absent below level 7".to_owned(),
        }]);

        let snapshot = PilotSnapshot::from_receipt(&receipt);

        assert_eq!(
            snapshot.defense.damage_reduction, None,
            "real PF1 has no \"DR 0\" -- the level-gate absence record must be omitted, not \
             surfaced as a fake zero"
        );
    }

    #[test]
    fn omits_damage_reduction_when_no_such_explanation_exists_at_all() {
        let receipt = receipt_with_explanations(Vec::new());

        let snapshot = PilotSnapshot::from_receipt(&receipt);

        assert_eq!(snapshot.defense.damage_reduction, None);
    }
}

/// v0.6 alpha swarm: the animal companion / mount stat block already
/// existed as a family of raw `ComputationExplanation` records
/// (`class_chassis.druid.animal_companion.*`,
/// `class_chassis.hunter.animal_companion.*`,
/// `class_chassis.cavalier.mount.*`, all grounded across every one of the
/// twenty master levels) but was never promoted to a first-class
/// `PilotSnapshot` field, so nothing downstream of the engine -- including
/// the desktop sheet's own Pets tab -- had a structured surface to render.
/// Exactly the shape `damage_reduction` already fixed for DR, one tab over.
///
/// These tests drive `PilotSnapshot::from_receipt` with synthetic receipts,
/// the same way the DR tests above do, so they pin the projection itself
/// independently of which classes happen to emit the records today.
#[cfg(test)]
mod companion_exposure_tests {
    use super::*;
    use crate::rules_core::pilot_compute::PilotBaseChassisComputation;

    fn receipt_with(
        explanations: Vec<ComputationExplanation>,
        diagnostics: Vec<ComputationDiagnostic>,
    ) -> PilotHeadlessReceipt {
        PilotHeadlessReceipt {
            case_id: None,
            source_package_id: "test".to_owned(),
            status: HeadlessReceiptStatus::Computed,
            computation: PilotBaseChassisComputation {
                ability_modifiers: AbilityModifiers::default(),
                base_attack_bonus: 0,
                base_saves: BaseSaves::default(),
                baseline_melee_attack_bonus: 0,
                baseline_armor_class: 0,
                total_saves: BaseSaves::default(),
                selected_skill_modifiers: SelectedSkillModifiers::default(),
                explanations,
                diagnostics,
            },
        }
    }

    fn explanation(id: &str, value: i16, detail: &str) -> ComputationExplanation {
        ComputationExplanation {
            id: id.to_owned(),
            value,
            detail: detail.to_owned(),
        }
    }

    /// A real level-1 Druid's own emitted record set, values verbatim from
    /// `pilot_compute.rs`'s
    /// `single_class_druid_level1_with_animal_companion_reaches_computed`.
    fn druid_level1_wolf_explanations() -> Vec<ComputationExplanation> {
        vec![
            explanation(
                "class_chassis.druid.animal_companion.wolf_stat_block",
                0,
                "Druid level 1 animal companion, Wolf",
            ),
            explanation(
                "class_chassis.druid.animal_companion.base_attack_bonus",
                2,
                "HD*3/4 = 1 plus Strength modifier +1",
            ),
            explanation(
                "class_chassis.druid.animal_companion.base_save.fortitude",
                3,
                "classlevel/2+2 = 3",
            ),
            explanation(
                "class_chassis.druid.animal_companion.base_save.reflex",
                3,
                "classlevel/2+2 = 3",
            ),
            explanation(
                "class_chassis.druid.animal_companion.base_save.will",
                0,
                "classlevel/3 = 0",
            ),
            explanation(
                "class_chassis.druid.animal_companion.armor_class",
                12,
                "base 10 + natural armor +2",
            ),
            explanation(
                "class_chassis.druid.animal_companion.bite_attack",
                1,
                "1d6 + 1 (1.5x Strength modifier, floored) plus Trip",
            ),
            explanation(
                "class_chassis.druid.animal_companion.hit_points",
                17,
                "maximized first Hit Die plus average for the remaining 1",
            ),
        ]
    }

    #[test]
    fn surfaces_a_druids_real_wolf_companion_stat_block() {
        let receipt = receipt_with(druid_level1_wolf_explanations(), Vec::new());

        let companion = PilotSnapshot::from_receipt(&receipt)
            .companion
            .expect("a receipt carrying the real companion records must surface a companion");

        assert_eq!(companion.owner_class_label, "Druid");
        assert_eq!(companion.role_label, "Animal Companion");
        assert_eq!(companion.species, "Wolf");
        assert_eq!(
            companion.summary_detail, "Druid level 1 animal companion, Wolf",
            "the recognition record's own detail is the companion's summary line"
        );
    }

    /// Ordering is the projection's own canonical one, not the order the
    /// engine happened to push the records in -- a sheet renders a stat
    /// block in a fixed reading order.
    #[test]
    fn projects_every_grounded_statistic_in_canonical_order_with_honest_labels() {
        let receipt = receipt_with(druid_level1_wolf_explanations(), Vec::new());

        let companion = PilotSnapshot::from_receipt(&receipt).companion.expect("companion");

        let rendered: Vec<(&str, i16)> = companion
            .stats
            .iter()
            .map(|stat| (stat.label.as_str(), stat.value))
            .collect();
        assert_eq!(
            rendered,
            vec![
                ("Hit Points", 17),
                ("Armor Class", 12),
                // The record id reads `base_attack_bonus`, but its value is
                // the companion's Hit-Dice-derived base attack bonus PLUS
                // its Strength modifier -- so the honest label is the total
                // it actually is, not the name of the id.
                ("Attack Bonus", 2),
                ("Bite Damage Bonus", 1),
                ("Fortitude Save", 3),
                ("Reflex Save", 3),
                ("Will Save", 0),
            ]
        );
        assert_eq!(
            companion.stats[0].detail,
            "maximized first Hit Die plus average for the remaining 1",
            "each row carries its own explanation detail verbatim -- the engine's own \
             derivation prose, never a rewritten one"
        );
    }

    /// A Will save of 0 at 2 HD is a real computed value (2/3 floors to 0),
    /// unlike DR 0 which is a level-gate absence. It must render, not be
    /// filtered out the way `damage_reduction` filters its own zero.
    #[test]
    fn keeps_a_genuinely_computed_zero_statistic() {
        let receipt = receipt_with(druid_level1_wolf_explanations(), Vec::new());

        let companion = PilotSnapshot::from_receipt(&receipt).companion.expect("companion");

        assert!(
            companion
                .stats
                .iter()
                .any(|stat| stat.label == "Will Save" && stat.value == 0),
            "a real floor-division 0 is a computed value, not an absence: {:?}",
            companion.stats
        );
    }

    #[test]
    fn surfaces_a_cavaliers_horse_mount_under_its_own_role_and_species() {
        let receipt = receipt_with(
            vec![
                explanation(
                    "class_chassis.cavalier.mount.horse_stat_block",
                    0,
                    "Cavalier level 1 mount, Horse",
                ),
                explanation("class_chassis.cavalier.mount.armor_class", 14, "10 + 4"),
                explanation("class_chassis.cavalier.mount.hoof_attack", 4, "1d4 + 4"),
            ],
            Vec::new(),
        );

        let companion = PilotSnapshot::from_receipt(&receipt).companion.expect("companion");

        assert_eq!(companion.owner_class_label, "Cavalier");
        assert_eq!(companion.role_label, "Mount");
        assert_eq!(companion.species, "Horse");
        assert!(
            companion
                .stats
                .iter()
                .any(|stat| stat.label == "Hoof Damage Bonus" && stat.value == 4),
            "the Horse's own natural attack must be labelled as itself: {:?}",
            companion.stats
        );
    }

    /// The advancement-absent diagnostic is the engine's own honest,
    /// carefully-authored list of what deliberately is NOT grounded (bonus
    /// tricks, companion skills and feats, the player-chosen stat increase,
    /// the size advance, Evasion/Devotion/Multiattack). It is non-blocking,
    /// so a `Computed` load drops it before the wire -- it has to travel
    /// with the companion itself or the player never sees it.
    #[test]
    fn carries_the_engines_own_advancement_absent_note_verbatim() {
        let receipt = receipt_with(
            druid_level1_wolf_explanations(),
            vec![ComputationDiagnostic {
                id: "class_feature.druid.animal_companion.advancement_absent".to_owned(),
                message: "bonus tricks, companion skills and feats are not grounded".to_owned(),
                claim_blocking: false,
            }],
        );

        let companion = PilotSnapshot::from_receipt(&receipt).companion.expect("companion");

        assert_eq!(
            companion.advancement_note.as_deref(),
            Some("bonus tricks, companion skills and feats are not grounded")
        );
    }

    #[test]
    fn carries_the_provably_vacuous_ability_notes() {
        let mut explanations = druid_level1_wolf_explanations();
        explanations.push(explanation(
            "class_feature.druid.animal_companion.link_vacuous",
            0,
            "Link is vacuous: no Handle Animal check is ever computed",
        ));
        explanations.push(explanation(
            "class_feature.druid.animal_companion.share_spells_vacuous",
            0,
            "Share Spells is vacuous: no spell-resolution engine exists",
        ));
        let receipt = receipt_with(explanations, Vec::new());

        let companion = PilotSnapshot::from_receipt(&receipt).companion.expect("companion");

        assert_eq!(
            companion.notes,
            vec![
                "Link is vacuous: no Handle Animal check is ever computed".to_owned(),
                "Share Spells is vacuous: no spell-resolution engine exists".to_owned(),
            ]
        );
    }

    #[test]
    fn omits_the_companion_entirely_when_no_such_records_exist() {
        let receipt = receipt_with(Vec::new(), Vec::new());

        assert!(
            PilotSnapshot::from_receipt(&receipt).companion.is_none(),
            "a companion-less class must carry no companion at all -- never an empty or \
             zeroed stat block"
        );
    }

    /// A class that grounds unrelated `class_chassis.*` records must not be
    /// mistaken for one that has a companion.
    #[test]
    fn ignores_unrelated_class_chassis_records() {
        let receipt = receipt_with(
            vec![explanation(
                "class_chassis.fighter.base_attack_bonus",
                1,
                "Fighter full BAB",
            )],
            Vec::new(),
        );

        assert!(PilotSnapshot::from_receipt(&receipt).companion.is_none());
    }
}
