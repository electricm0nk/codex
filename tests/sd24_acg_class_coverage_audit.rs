//! SD-24 Epic 4 criterion 4.3 — Per-class audit: ACG classes (Arcanist,
//! Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer,
//! Swashbuckler, Warpriest — the real, corrected 10-class ACG roster;
//! see `rules_tables::acg::mod.rs`'s own roster-correction doc comment
//! for why criterion 4.3's header text's "Alchemist-side" is not real
//! ACG content and "Slayer" — omitted from that same header text — is).
//!
//! This is the standing regression test behind
//! `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_4/per-class-coverage-matrix.md`'s
//! ACG section and the ten per-class
//! `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_4/acg_<class>_coverage.md`
//! receipts: it makes the audit's numeric claims (chassis rows wired,
//! named-feature count, live pilot-compute integration) executable and
//! checked on every run, rather than a one-off hand count that could
//! silently drift from the code. Mirrors
//! `tests/sd24_apg_class_coverage_audit.rs` exactly (same three-finding
//! shape), scoped to `rules_tables::acg`.
//!
//! RED -> GREEN evidence (recorded in this cycle's receipt): before
//! `rules_tables::acg::mod::{AcgClassId::ALL, AcgClassCoverage,
//! class_coverage, coverage_report}` existed, this file did not compile
//! (RED — no such items). Adding that small, real (non-fabricated: every
//! field is computed from `class_table()` output, a `MAX_SUPPORTED_LEVEL`
//! const, or a documented corpus count) coverage-report API turned it
//! GREEN.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::rules_tables::acg::{class_coverage, coverage_report, AcgClassId};

/// Every real ACG class's chassis table is fully wired for its own
/// `MAXLEVEL:20` ceiling (SD-22 Epic 4's already-landed, LST-verified
/// BAB/save cut) — the audit's first, structural finding.
#[test]
fn all_ten_acg_classes_have_full_chassis_row_coverage() {
    let report = coverage_report();
    assert_eq!(report.len(), 10, "all ten real ACG classes should have a coverage row");
    for row in report {
        assert_eq!(
            row.chassis_rows_wired, row.chassis_rows_expected,
            "{:?}: chassis rows wired ({}) should equal chassis rows expected ({}) -- \
             SD-22 Epic 4 already ground the full BAB/save chassis for every ACG class",
            row.class_id, row.chassis_rows_wired, row.chassis_rows_expected
        );
        assert_eq!(
            row.chassis_rows_expected, 20,
            "{:?}: every real ACG class's MAXLEVEL is 20 per acg_classes.lst",
            row.class_id
        );
    }
}

/// The audit's second finding: zero named/narrative class features
/// (Arcane Exploit, Bloodline, Martial Flexibility, Hunter's Trick,
/// Studied Combat, Spirit, Raging Song, Sneak Attack talents, Panache,
/// Blessings, ...) are wired for any ACG class yet -- this is a real gap,
/// not a placeholder value, and this test is a canary: if a future cycle
/// starts wiring named features, it must consciously update this
/// assertion (and the coverage-matrix artifact) rather than silently
/// leaving the audit stale.
///
/// **Updated (v0.6 alpha swarm, risks item 8, first through tenth
/// APG/ACG class-specific closures, 2026-07-25/26):** exactly this
/// canary fired -- Skald's Inspired Rage, Bloodrager's Bloodrage,
/// Brawler's AC Bonus, Hunter's Animal Companion, Arcanist's Arcane
/// Reservoir + Spells Prepared, Warpriest's Blessings + Sacred Weapon,
/// Slayer's Sneak Attack + Trap Sense + Trapfinding + Track,
/// Swashbuckler's Panache + Charmed Life + Nimble, and Investigator's
/// Trapfinding + Trap Sense + Inspiration pool-size are now genuinely
/// wired. All nine are carved out of the "stays 0" loop below and given
/// their own dedicated assertions, per this test's own documented
/// update instruction. Every other ACG class remains at 0, unchanged.
/// Arcanist's and Warpriest's own `named_features_wired == 2` each (not
/// 1, unlike Skald/Bloodrager/Brawler/Hunter) for related but distinct
/// reasons, and Slayer's/Swashbuckler's/Investigator's own `== 4`/`==
/// 3`/`== 3` for yet another reason -- see
/// `AcgClassCoverage::named_features_wired`'s own doc comment in
/// `rules_tables::acg::mod` for the full record: Arcanist's real
/// spellcasting build genuinely closes 1 more distinct `KEY:Arcanist ~
/// ...` record (`Spells Prepared`) beyond Arcane Reservoir, while
/// `Cantrips` does NOT add a third (not separately implemented);
/// Warpriest has NO general-spellcasting KEY record at all (only
/// `Orisons`, also not separately implemented), so its own count is
/// Blessings + Sacred Weapon, with Destruction Blessing's own
/// Destructive Attacks folded into the single Blessings slot (a
/// different corpus class-prefix, `KEY:Destruction Blessing ~ ...`, not
/// `KEY:Warpriest ~ ...`); Slayer's own four sub-features,
/// Swashbuckler's own three sub-features, and Investigator's own three
/// sub-features are each genuinely structurally independent (no shared
/// table/mechanism links any pair of them the way Cantrips/Orisons
/// shared their class's spellcasting table), so each one counts
/// honestly, not folded down. Investigator's own prepared extract
/// spellcasting (the Alchemist formula list) is explicitly deferred to
/// its own follow-on slice, so it does not add a fourth count.
#[test]
fn zero_named_class_features_are_wired_for_any_acg_class_except_the_nine_named_closures_landed_so_far()
{
    for row in coverage_report() {
        if matches!(
            row.class_id,
            AcgClassId::Skald
                | AcgClassId::Bloodrager
                | AcgClassId::Brawler
                | AcgClassId::Hunter
                | AcgClassId::Arcanist
                | AcgClassId::Warpriest
                | AcgClassId::Slayer
                | AcgClassId::Swashbuckler
                | AcgClassId::Investigator
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
        (AcgClassId::Skald, "Inspired Rage", 1),
        (AcgClassId::Bloodrager, "Bloodrage", 1),
        (AcgClassId::Brawler, "AC Bonus", 1),
        (AcgClassId::Hunter, "Animal Companion", 1),
        (AcgClassId::Arcanist, "Arcane Reservoir + Spells Prepared", 2),
        (AcgClassId::Warpriest, "Blessings + Sacred Weapon", 2),
        (
            AcgClassId::Slayer,
            "Sneak Attack + Trap Sense + Trapfinding + Track",
            4,
        ),
        (AcgClassId::Swashbuckler, "Panache + Charmed Life + Nimble", 3),
        (
            AcgClassId::Investigator,
            "Trapfinding + Trap Sense + Inspiration pool-size",
            3,
        ),
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
             (spellcasting/other named features besides {feature_name} remain ungrounded)"
        );
    }
}

/// The audit's third finding, proven empirically rather than by
/// inspection alone. **Superseded (v0.6 alpha swarm, risks item 8, fourth
/// slice):** `pilot_compute::compute_pilot_base_chassis` now genuinely
/// wires real BAB/save/HP for all ten ACG classes via
/// `compute_acg_class_chassis` (verified against the actual
/// `acg_classes.lst` corpus tokens, cross-checked against
/// `pilot_compute.rs`'s own `all_ten_acg_classes_ground_real_bab_save_and_hp_at_level_1`
/// reference test), so the old universal `class_chassis.unsupported`
/// diagnostic no longer fires for any of them -- that claim is stale.
/// Each class instead trips its own real, unconditional
/// `class_feature.acg.<class>.unsupported` diagnostic: the class-skill
/// list, named class features, and spellcasting are all still genuinely
/// ungrounded, the same shape as the APG dispatch slice before it. This
/// remains the "no-stub" doctrine's own worked example -- an honest,
/// named gap, not silent fabrication -- just a narrower gap than before.
///
/// Worth stating plainly since the APG equivalent of this fix had only
/// one BAB surprise (Cavalier, full BAB): ACG has **four** -- Bloodrager,
/// Brawler, Slayer, and Swashbuckler are all real full-BAB classes (+1 at
/// level 1), not the 3/4- or 1/2-BAB shape most of the other six share
/// (which floor to +0 at level 1). Checked each of the ten individually
/// against the real corpus rather than assuming a single exception would
/// cover it.
///
/// **Updated (v0.6 alpha swarm, risks item 8, first APG/ACG class-specific
/// closure, 2026-07-25):** Skald is carved out of the per-class diagnostic
/// assertion below. Its own generic `class_feature.acg.skald.unsupported`
/// diagnostic was retired (its blanket "no named class-feature
/// computation... grounded anywhere" claim became false once Inspired Rage
/// was genuinely wired) and replaced with the narrower
/// `class_feature.acg.skald.spellcasting_deferred.unsupported` diagnostic,
/// naming only the pieces still genuinely missing. Every other ACG class
/// keeps the original, unmodified diagnostic.
#[test]
fn acg_classes_ground_real_bab_save_but_stay_blocked_on_the_unconditional_diagnostic() {
    for class_id in AcgClassId::ALL {
        let input = minimal_input_for(class_id);
        let computation = compute_pilot_base_chassis(&input);

        // Real level-1 base attack bonus per class (matches
        // `pilot_compute.rs`'s own `all_ten_acg_classes_ground_real_bab_save_and_hp_at_level_1`
        // reference test, itself verified against `acg_classes.lst`): every
        // class is 0 except the four real full-BAB classes.
        let expected_bab = if matches!(
            class_id,
            AcgClassId::Bloodrager
                | AcgClassId::Brawler
                | AcgClassId::Slayer
                | AcgClassId::Swashbuckler
        ) {
            1
        } else {
            0
        };
        assert_eq!(
            computation.base_attack_bonus, expected_bab,
            "{:?}: base attack bonus is now genuinely computed from the real chassis, not a \
             fabricated/blocked zero",
            class_id
        );

        let expected_diagnostic_id = match class_id {
            AcgClassId::Bloodrager | AcgClassId::Hunter => {
                format!("class_feature.acg.{}.spellcasting_deferred.unsupported", class_id.name())
            }
            AcgClassId::Brawler => {
                "class_feature.acg.brawler.other_features_deferred.unsupported".to_owned()
            }
            AcgClassId::Skald => {
                // v0.6 alpha swarm, risks item 8 (Skald spellcasting
                // closure): Skald's own spellcasting_deferred diagnostic
                // is now retired too -- known-spell posture is genuinely
                // validated (reusing Bard's own spell list/tables,
                // verified identical), so the remaining claim-blocking
                // diagnostic is renamed to other_features_deferred,
                // naming only Skald's OTHER still-ungrounded named
                // features (unlike Bard, whose own remaining features
                // were already built in an earlier SD13-E5 cycle, Skald
                // never reaches Computed this closure either).
                "class_feature.acg.skald.other_features_deferred.unsupported".to_owned()
            }
            AcgClassId::Arcanist => {
                // v0.6 alpha swarm, risks item 8 (Arcanist full-build
                // closure): a bare, minimal-input Arcanist (no spells
                // recorded) trips BOTH the new exploits_deferred
                // diagnostic (always pushed, unconditional on spellbook
                // state) and the prepared_spellbook diagnostic (since
                // this minimal fixture records no spells at all) --
                // checking for exploits_deferred here is sufficient to
                // prove the retired generic diagnostic is gone; the
                // dedicated pilot_compute.rs test module separately
                // proves prepared_spellbook is retired too once a real
                // spell is recorded.
                "class_feature.acg.arcanist.exploits_deferred.unsupported".to_owned()
            }
            AcgClassId::Warpriest => {
                // v0.6 alpha swarm, risks item 8 (Warpriest full-build
                // closure): a bare, minimal-input Warpriest (no
                // Blessing choice, no spells) trips other_features_deferred
                // (always pushed, unconditional), blessing_powers.unsupported
                // (no recognized Destruction Blessing), AND
                // prepared_spellbook.unsupported (no spells recorded) --
                // checking for other_features_deferred here is sufficient
                // to prove the retired generic diagnostic is gone; the
                // dedicated pilot_compute.rs test module separately
                // proves blessing_powers/prepared_spellbook are retired
                // too once a real choice/spell is recorded.
                "class_feature.acg.warpriest.other_features_deferred.unsupported".to_owned()
            }
            AcgClassId::Slayer => {
                // v0.6 alpha swarm, risks item 8 (Slayer full-build
                // closure): all four flat sub-feature formulas (Sneak
                // Attack, Trap Sense, Trapfinding, Track) ground
                // unconditionally regardless of this minimal fixture's
                // own posture, so the only diagnostic to check is the
                // narrowed other_features_deferred one.
                "class_feature.acg.slayer.other_features_deferred.unsupported".to_owned()
            }
            AcgClassId::Swashbuckler => {
                // v0.6 alpha swarm, risks item 8 (Swashbuckler full-build
                // closure): Panache's max and Nimble's dodge bonus ground
                // unconditionally, and this minimal level-1 fixture has
                // not yet gained Charmed Life (granted at level 2), so
                // the only diagnostic to check is the narrowed
                // other_features_deferred one.
                "class_feature.acg.swashbuckler.other_features_deferred.unsupported".to_owned()
            }
            AcgClassId::Investigator => {
                // v0.6 alpha swarm, risks item 8 (Investigator full-build
                // closure, no-spellcasting MVP): all three flat sub-
                // feature formulas (Trapfinding, Trap Sense, Inspiration
                // pool-size) ground unconditionally regardless of this
                // minimal fixture's own posture, so the only diagnostic
                // to check is the narrowed other_features_deferred one.
                "class_feature.acg.investigator.other_features_deferred.unsupported".to_owned()
            }
            _ => format!("class_feature.acg.{}.unsupported", class_id.name()),
        };
        let unsupported = computation
            .diagnostics
            .iter()
            .find(|d| d.id == expected_diagnostic_id)
            .unwrap_or_else(|| {
                panic!(
                    "{:?}: expected the real '{expected_diagnostic_id}' diagnostic -- if this \
                     now fails, pilot_compute.rs's class-feature/skill/spellcasting coverage for \
                     this ACG class has changed again and this test (plus the coverage-matrix \
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
            class_id,
            AcgClassId::Skald
                | AcgClassId::Bloodrager
                | AcgClassId::Brawler
                | AcgClassId::Hunter
                | AcgClassId::Arcanist
                | AcgClassId::Warpriest
                | AcgClassId::Slayer
                | AcgClassId::Swashbuckler
                | AcgClassId::Investigator
        ) {
            let retired_diagnostic_id = format!("class_feature.acg.{}.unsupported", class_id.name());
            assert!(
                !computation.diagnostics.iter().any(|d| d.id == retired_diagnostic_id),
                "the retired generic diagnostic must never appear for {:?}: {:?}",
                class_id,
                computation.diagnostics
            );
        }

        let row = class_coverage(class_id);
        assert!(
            row.pilot_compute_integrated,
            "{:?}: coverage row's pilot_compute_integrated should now be true -- the BAB/save \
             chassis is genuinely wired into compute_pilot_base_chassis",
            class_id
        );
    }
}

fn minimal_input_for(class_id: AcgClassId) -> CharacterInput {
    CharacterInput {
        case_id: Some(format!("sd24_acg_class_coverage_audit.{}", class_id.name())),
        source_package_id: "pf1.advanced_class_guide".to_string(),
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
