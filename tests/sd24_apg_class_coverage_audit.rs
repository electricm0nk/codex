//! SD-24 Epic 4 criterion 4.2 — Per-class audit: APG classes (Alchemist,
//! Cavalier, Inquisitor, Oracle, Summoner, Witch).
//!
//! This is the standing regression test behind
//! `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_4/per-class-coverage-matrix.md`'s
//! APG section and the six per-class
//! `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_4/apg_<class>_coverage.md`
//! receipts: it makes the audit's numeric claims (chassis rows wired,
//! named-feature count, live pilot-compute integration) executable and
//! checked on every run, rather than a one-off hand count that could
//! silently drift from the code.
//!
//! RED -> GREEN evidence (recorded in this cycle's receipt): before
//! `rules_tables::apg::mod::{ApgClassId::ALL, ApgClassCoverage, class_coverage,
//! coverage_report}` existed, this file did not compile (RED — no such
//! items). Adding that small, real (non-fabricated: every field is computed
//! from `class_table()` output, a `MAX_SUPPORTED_LEVEL` const, or a
//! documented corpus count) coverage-report API turned it GREEN.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::rules_tables::apg::{ApgClassId, class_coverage, coverage_report};

/// Every real APG class's chassis table is fully wired for its own
/// `MAXLEVEL:20` ceiling (SD-22 Epic 3's already-landed, LST-verified
/// BAB/save cut) — the audit's first, structural finding.
#[test]
fn all_six_apg_classes_have_full_chassis_row_coverage() {
    let report = coverage_report();
    assert_eq!(report.len(), 6, "all six real APG classes should have a coverage row");
    for row in report {
        assert_eq!(
            row.chassis_rows_wired, row.chassis_rows_expected,
            "{:?}: chassis rows wired ({}) should equal chassis rows expected ({}) -- \
             SD-22 Epic 3 already ground the full BAB/save chassis for every APG class",
            row.class_id, row.chassis_rows_wired, row.chassis_rows_expected
        );
        assert_eq!(
            row.chassis_rows_expected, 20,
            "{:?}: every real APG class's MAXLEVEL is 20 per apg_classes.lst",
            row.class_id
        );
    }
}

/// The audit's second finding: zero named/narrative class features (Bombs,
/// Discoveries, Hexes, Judgments, Mystery Revelations, Eidolon, Challenge,
/// ...) are wired for any APG class yet -- this is a real gap, not a
/// placeholder value, and this test is a canary: if a future cycle starts
/// wiring named features, it must consciously update this assertion (and
/// the coverage-matrix artifact) rather than silently leaving the audit
/// stale.
///
/// **Updated (v0.6 alpha swarm, risks item 8, Cavalier Mount / Alchemist
/// Mutagen / Inquisitor Judgment closures, 2026-07-25):** exactly this
/// canary fired -- Cavalier's Mount, Alchemist's Mutagen, and
/// Inquisitor's Justice judgment are now genuinely wired, the first three
/// named APG class features this repo computes. All three are carved out
/// of the "stays 0" loop below and given their own dedicated assertions
/// (`named_features_wired == 1` each), per this test's own documented
/// update instruction. Every other APG class remains at 0, unchanged.
///
/// **Updated again (Oracle full-build closure, 2026-07-25):** Oracle's
/// Mystery (Life/Healing Hands) and Curse (Clouded Vision) are also now
/// genuinely wired -- carved out below with its own dedicated
/// `named_features_wired == 2` assertion (Mystery slot + Curse slot; the
/// known-spell posture and Orisons are not counted separately, the same
/// "shares the general spellcasting mechanism" reasoning already applied
/// to Arcanist's Cantrips and Warpriest's Orisons).
#[test]
fn zero_named_class_features_are_wired_for_any_apg_class_except_cavaliers_mount_alchemists_mutagen_inquisitors_judgment_and_oracles_mystery_and_curse()
{
    for row in coverage_report() {
        if matches!(
            row.class_id,
            ApgClassId::Cavalier | ApgClassId::Alchemist | ApgClassId::Inquisitor | ApgClassId::Oracle
        ) {
            continue;
        }
        assert_eq!(
            row.named_features_wired, 0,
            "{:?}: named_features_wired should be 0 (documented SD-24 Epic 4 finding); \
             if this now fails, a real feature landed -- update this canary and the \
             per-class coverage artifact together",
            row.class_id
        );
        assert!(
            row.named_features_expected > 0,
            "{:?}: named_features_expected should be a real positive corpus count, not 0",
            row.class_id
        );
    }

    for (class_id, feature_name, expected_wired) in [
        (ApgClassId::Cavalier, "Mount", 1),
        (ApgClassId::Alchemist, "Mutagen", 1),
        (ApgClassId::Inquisitor, "Justice judgment", 1),
        (ApgClassId::Oracle, "Mystery+Curse", 2),
    ] {
        let row = class_coverage(class_id);
        assert_eq!(
            row.named_features_wired, expected_wired,
            "{class_id:?}'s {feature_name} is now genuinely wired -- update this assertion (and \
             the coverage-matrix artifact) if this count changes again"
        );
        assert!(
            row.named_features_expected > row.named_features_wired,
            "{class_id:?}: named_features_expected should still exceed named_features_wired \
             (every other named feature besides {feature_name} remains ungrounded)"
        );
    }
}

/// The audit's third finding, proven empirically rather than by inspection
/// alone. **Superseded (v0.6 alpha swarm, risks item 8, second slice):**
/// `pilot_compute::compute_pilot_base_chassis` now genuinely wires real
/// BAB/save/HP for all six APG classes via `compute_apg_class_chassis`
/// (verified against the actual `apg_classes.lst` corpus tokens), so the old
/// universal `class_chassis.unsupported` diagnostic no longer fires for any
/// of them -- that claim is stale. Each class instead trips its own real,
/// unconditional `class_feature.apg.<class>.unsupported` diagnostic: the
/// class-skill list, named class features, and spellcasting (5 of the 6
/// cast) are all still genuinely ungrounded, the same shape as Ranger's own
/// `class_spell.ranger.partial_caster.unsupported` (risks item 8's first
/// slice). This remains the "no-stub" doctrine's own worked example -- an
/// honest, named gap, not silent fabrication -- just a narrower gap than
/// before.
#[test]
fn apg_classes_ground_real_bab_save_but_stay_blocked_on_the_unconditional_diagnostic() {
    for class_id in ApgClassId::ALL {
        let input = minimal_input_for(*class_id);
        let computation = compute_pilot_base_chassis(&input);

        // Real level-1 base attack bonus per class (matches
        // `pilot_compute.rs`'s own `all_six_apg_classes_ground_real_bab_save_and_hp_at_level_1`
        // reference test, itself verified against `apg_classes.lst`): every
        // class is 0 except Cavalier, whose full-BAB progression already
        // gives +1 at level 1.
        let expected_bab = if *class_id == ApgClassId::Cavalier { 1 } else { 0 };
        assert_eq!(
            computation.base_attack_bonus, expected_bab,
            "{:?}: base attack bonus is now genuinely computed from the real chassis, not a \
             fabricated/blocked zero",
            class_id
        );

        let expected_diagnostic_id = if *class_id == ApgClassId::Cavalier {
            "class_feature.apg.cavalier.other_features_deferred.unsupported".to_owned()
        } else if *class_id == ApgClassId::Alchemist {
            "class_feature.apg.alchemist.spellcasting_deferred.unsupported".to_owned()
        } else if *class_id == ApgClassId::Inquisitor {
            "class_feature.apg.inquisitor.other_features_deferred.unsupported".to_owned()
        } else if *class_id == ApgClassId::Oracle {
            "class_feature.apg.oracle.other_features_deferred.unsupported".to_owned()
        } else {
            format!("class_feature.apg.{}.unsupported", class_id.name())
        };
        let unsupported = computation
            .diagnostics
            .iter()
            .find(|d| d.id == expected_diagnostic_id)
            .unwrap_or_else(|| {
                panic!(
                    "{:?}: expected the real '{expected_diagnostic_id}' diagnostic -- if this \
                     now fails, pilot_compute.rs's class-feature/skill/spellcasting coverage for \
                     this APG class has changed again and this test (plus the coverage-matrix \
                     artifact) needs updating",
                    class_id
                )
            });
        assert!(
            unsupported.claim_blocking,
            "{:?}: '{expected_diagnostic_id}' must remain claim_blocking: true",
            class_id
        );
        if matches!(
            *class_id,
            ApgClassId::Cavalier | ApgClassId::Alchemist | ApgClassId::Inquisitor | ApgClassId::Oracle
        ) {
            let retired_diagnostic_id = format!("class_feature.apg.{}.unsupported", class_id.name());
            assert!(
                !computation.diagnostics.iter().any(|d| d.id == retired_diagnostic_id),
                "the retired generic diagnostic must never appear for {:?}: {:?}",
                class_id,
                computation.diagnostics
            );
        }

        let row = class_coverage(*class_id);
        assert!(
            row.pilot_compute_integrated,
            "{:?}: coverage row's pilot_compute_integrated should now be true -- the BAB/save \
             chassis is genuinely wired into compute_pilot_base_chassis",
            class_id
        );
    }
}

fn minimal_input_for(class_id: ApgClassId) -> CharacterInput {
    CharacterInput {
        case_id: Some(format!("sd24_apg_class_coverage_audit.{}", class_id.name())),
        source_package_id: "pf1.advanced_players_guide".to_string(),
        chosen: ChosenCharacterState {
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: format!("class:{}", class_id.name()),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}
