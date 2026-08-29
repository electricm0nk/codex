//! SD13-E4-F7 Sorcerer level-1 spontaneous spell-burden baseline proof.
//!
//! Proves the first truthful SD13-F7 spell-bearing slice: the live rules-core surface
//! ingests a deterministic Human `class:sorcerer:1` input, leaves direct computed
//! evidence that recognizes the Sorcerer level-1 spell-bearing class identity rather
//! than treating it as an undocumented packet placeholder, and yet stays explicitly
//! claim-blocked with two distinct diagnostics: one for the bloodline burden and one
//! for the spontaneous known-spell / slot posture burden. It also pins the matrix
//! reclassification of the Sorcerer row from `Unverified` / `Observed` to `Blocked` /
//! `Computed`, while proving Bard and Wizard stay `Unverified` / `Observed` and the
//! accepted Paladin/Ranger hybrid rows stay `Blocked` / `Computed`.
//!
//! It is intentionally not a spell engine. It fabricates no spell slots, spells known,
//! spell DCs, bonus spells, prepared posture, school choice, or general spell totals,
//! and it grounds no Sorcerer level 2+. It also preserves the accepted Human race seam
//! on the spell-bearing path.
//!
//! The SD13-E4 Sorcerer decomposition slice splits the F7 combined bloodline blocker
//! into two named diagnostics and grounds one of them for real: Eschew Materials, the
//! universal, bloodline-independent bonus feat every 1st-level Sorcerer receives (PF1
//! Core Rulebook: it lets a Sorcerer cast spells with material components costing 1 gp
//! or less without needing the material component). This promotes the matrix row from
//! `Blocked` to `Partial`, mirroring the SD13-E3 Ranger Track-grounding precedent: one
//! of several named pillars is grounded for real while the class remains far from fully
//! proven and the entire spontaneous spell burden stays untouched.
//!
//! The SD13-E5 Sorcerer bloodline-choice slice grounds the next honest pillar: the
//! canonical deterministic bloodline choice-slot selection
//! (`choice:sorcerer_bloodline -> bloodline:arcane`) is recognized as chosen input on
//! the compute seam, mirroring the Fighter bonus-feat choice-slot / Wizard Scribe
//! Scroll precedent. It is a recognition record only: the Arcane bloodline's level-1
//! power is Arcane Bond (a familiar or a bonded object — an execution engine, not a
//! flat number), so no power value is fabricated. The former `bloodline_power` blocker
//! is narrowed to `arcane_bond_and_bloodline_progression`, naming exactly what stays
//! unimplemented: Arcane Bond execution, the bloodline arcana (a conditional +1 spell
//! save DC on metamagic-raised spells), the bloodline class skill grant, and the bonus
//! spells/feats at 3rd+ level. The row stays `Partial` and the spontaneous spell burden
//! stays untouched.

use codex::rules_core::character_input::{
    AcquisitionMode, CharacterInput, SpellSelection, load_character_input_fixture,
};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, HeadlessReceiptStatus,
    PilotBaseChassisComputation, build_pilot_headless_receipt, compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};

const SORCERER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.sorcerer";
const ESCHEW_MATERIALS_EXPLANATION_ID: &str = "class_chassis.sorcerer.eschew_materials";
const BLOODLINE_CHOICE_EXPLANATION_ID: &str = "class_chassis.sorcerer.bloodline_choice";
const ARCANE_BOND_BLOCKER_ID: &str =
    "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported";
const RETIRED_BLOODLINE_POWER_BLOCKER_ID: &str =
    "class_feature.sorcerer.bloodline_power.unsupported";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.sorcerer.spontaneous.unsupported";

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

fn claim_blocking<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    let diag = computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        });
    assert!(
        diag.claim_blocking,
        "diagnostic '{id}' must be claim-blocking: {diag:?}"
    );
    diag
}

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Direct runtime evidence: the spell-bearing identity is acknowledged -----

#[test]
fn sorcerer_level1_leaves_direct_spell_baseline_recognition_evidence() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Sorcerer spell-bearing identity is recognized
    // on the compute path, not silently dropped as an undocumented packet placeholder.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:sorcerer") && recognition.detail.contains("level 1"),
        "sorcerer recognition must name the class:sorcerer:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("spell"),
        "sorcerer recognition must name the spell-bearing identity: {}",
        recognition.detail
    );

    // It is recognition only: it must carry no fabricated mechanical value (+0).
    assert_eq!(
        recognition.value, 0,
        "sorcerer spell baseline recognition must carry no fabricated value (+0)"
    );
    // (v0.6 alpha swarm, risks item 8) `table_class_id` was widened to recognize
    // Sorcerer, so the integrated `class_chassis.base_attack_bonus` explanation now
    // genuinely exists -- but Sorcerer's real 1/2-BAB progression still floors to 0 at
    // level 1, so the value coincidentally stays 0 (see
    // `sd13_sorcerer_base_attack_and_saves.rs` for the same flip in full).
    assert_eq!(
        computation.base_attack_bonus, 0,
        "sorcerer level 1's real 1/2-BAB progression (classlevel / 2) is 0"
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "sorcerer base-attack bonus is now a genuinely integrated chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (CHA 17 -> +3).
    assert_eq!(computation.ability_modifiers.charisma, 4);
}

#[test]
fn sorcerer_level1_fabricates_no_spell_math() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // No explanation may fabricate spell slots, spells known, DCs, bonus spells, prepared
    // posture, school choice, or general spell totals. The recognition record and — as
    // of the further SD13-E5 access-ladder slice
    // (tests/sd13_sorcerer_spell_level_thresholds.rs) — the spell-level ACCESS record
    // (value 1 at level 1, because a sorcerer casts 1st-level spells from level 1 per
    // the raw table row "3/—/…"; access only, never per-day counts) are the only
    // spell-tagged explanations.
    for explanation in &computation.explanations {
        assert!(
            explanation.id == RECOGNITION_ID
                || explanation.id == "class_chassis.sorcerer.spontaneous.spell_level_access"
                // The base_spells_per_day family (a further SD13-E5 slice,
                // tests/sd13_sorcerer_spells_per_day_counts.rs) fires at every
                // supported level as literal table records; allowing it by
                // prefix keeps this control accurate without weakening it.
                || explanation
                    .id
                    .starts_with("class_chassis.sorcerer.spontaneous.base_spells_per_day.")
                // The spell_save_dc family (a further SD13-E5 slice,
                // tests/sd13_sorcerer_spell_save_dcs.rs): base DC arithmetic
                // records, allowed by prefix like the per-day family.
                || explanation
                    .id
                    .starts_with("class_chassis.sorcerer.spontaneous.spell_save_dc.")
                // The spells_known family (a further SD13-E5 slice,
                // tests/sd13_sorcerer_spells_known_counts.rs): base
                // known-count table records, allowed by prefix.
                || explanation
                    .id
                    .starts_with("class_chassis.sorcerer.spontaneous.spells_known.")
                // The bonus_spells_per_day family (a further SD13-E5 slice,
                // tests/sd13_sorcerer_bonus_spells.rs): Charisma bonus-slot
                // counts from the shared PF1 table, allowed by prefix.
                || explanation
                    .id
                    .starts_with("class_chassis.sorcerer.spontaneous.bonus_spells_per_day.")
                // The total_spells_per_day family (a further SD13-E5 slice,
                // tests/sd13_sorcerer_total_spells_per_day.rs): the pure sum
                // of the grounded base and bonus records, allowed by prefix.
                || explanation
                    .id
                    .starts_with("class_chassis.sorcerer.spontaneous.total_spells_per_day.")
                // (v0.6 alpha swarm, risks item 8) class_spell.sorcerer.known_spells is
                // the real, non-fabricated known-spell-selection record --
                // `ground_sorcerer_known_spells` now runs once the posture is genuinely
                // valid (this fixture has zero known spells, so the record honestly
                // reports 0, not a fabricated value).
                || explanation.id == "class_spell.sorcerer.known_spells"
                // SD-34 decisions.md section 18: widened BY CONSTRUCTION, not narrowed --
                // class_feature_grant_consumer now emits real, citation-backed corpus_record
                // ids for Sorcerer (previously wholesale-excluded); this shape carve-out
                // admits them without weakening the substring check for anything else.
                || explanation.id.starts_with("class_feature.sorcerer.corpus_record.")
                // SD-34 bucket-B batch cycle: found stale, unrelated to this cycle's own
                // three mechanisms -- the pre-existing (SD-32 T12 Epic 8, `decisions.md
                // §17`) generic bloodline-power pass emits one real, citation-backed
                // `class_feature.sorcerer.bloodline.generic.<bloodline>.<power>.<var>` id per
                // corpus power the selected bloodline carries; several real PCGen var names
                // (e.g. `SorcererBloodlineFeatImprovedCounterspell`) contain "spell" as a
                // substring, tripping this stale substring catch-all. Scoped by prefix, same
                // shape as the corpus_record carve-out above, admitting only this one
                // namespace.
                || explanation.id.starts_with("class_feature.sorcerer.bloodline.generic.")
                || !explanation.id.contains("spell"),
            "no fabricated spell explanation is allowed beyond the +0 recognition and the \
             access-ladder record: {explanation:?}"
        );
    }
    // The recognition itself asserts it fabricates no spell math.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert_eq!(recognition.value, 0);
}

// ----- Grounded for real: Eschew Materials, the bloodline-independent level-1 grant -----

#[test]
fn sorcerer_level1_grounds_eschew_materials_as_a_real_bonus_feat_grant() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Eschew Materials is granted to every 1st-level Sorcerer regardless of bloodline;
    // it must be a real, named, grounded explanation rather than a claim-blocking
    // placeholder.
    let eschew = explanation(&computation, ESCHEW_MATERIALS_EXPLANATION_ID);
    assert!(
        eschew.detail.contains("Eschew Materials"),
        "eschew materials explanation must name the feat: {}",
        eschew.detail
    );
    assert!(
        eschew.detail.contains("material component"),
        "eschew materials explanation must describe the material-component grant: {}",
        eschew.detail
    );
    assert!(
        eschew.detail.contains("bloodline"),
        "eschew materials explanation must note it is bloodline-independent: {}",
        eschew.detail
    );

    // It is a boolean feat grant, not a numeric formula: it must fabricate no spell math
    // or bloodline-power effect, so it carries no non-zero value.
    assert_eq!(
        eschew.value, 0,
        "eschew materials grant is a boolean feat grant, not a numeric bonus, so it must \
         carry no fabricated value"
    );

    // It must no longer appear as a claim-blocking diagnostic.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.sorcerer.eschew_materials.unsupported"),
        "eschew materials must no longer be claim-blocked: {:?}",
        computation.diagnostics
    );
}

// ----- Grounded for real: the canonical bloodline choice-slot selection -----

#[test]
fn sorcerer_level1_grounds_the_bloodline_choice_recognition() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The canonical deterministic bloodline choice-slot selection must be recognized as
    // chosen input, mirroring the Fighter bonus-feat choice-slot / Wizard Scribe Scroll
    // precedent: a real, named, grounded explanation rather than part of a claim-blocking
    // placeholder.
    let bloodline_choice = explanation(&computation, BLOODLINE_CHOICE_EXPLANATION_ID);
    assert!(
        bloodline_choice.detail.contains("choice:sorcerer_bloodline")
            && bloodline_choice.detail.contains("bloodline:arcane"),
        "bloodline choice recognition must name the canonical selection \
         (choice:sorcerer_bloodline -> bloodline:arcane): {}",
        bloodline_choice.detail
    );

    // Honesty constraint: the Arcane bloodline's level-1 power is Arcane Bond (a
    // familiar or a bonded object), an execution engine rather than a flat number. The
    // recognition record must name this explicitly and fabricate no power value.
    assert!(
        bloodline_choice.detail.contains("Arcane Bond"),
        "bloodline choice recognition must name Arcane Bond as the ungroundable-by-flat-value \
         level-1 power: {}",
        bloodline_choice.detail
    );
    assert_eq!(
        bloodline_choice.value, 0,
        "bloodline choice recognition is a choice-slot recognition record, not a numeric \
         bonus, so it must carry no fabricated value"
    );

    // The former combined bloodline-power blocker must be retired, replaced by the
    // narrowed arcane-bond-and-bloodline-progression blocker asserted below.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == RETIRED_BLOODLINE_POWER_BLOCKER_ID),
        "the combined bloodline_power blocker must be retired once the bloodline choice \
         is grounded: {:?}",
        computation.diagnostics
    );
}

// ----- Still blocked: two distinct honest, class-specific burden diagnostics -----

#[test]
fn sorcerer_level1_stays_blocked_on_arcane_bond_and_bloodline_progression_burden() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // With the bloodline CHOICE grounded, the blocker narrows to what actually remains
    // unimplemented for this fixture (an Arcane-bloodline Sorcerer that recorded no
    // Arcane Bond choice): Arcane Bond execution, the bloodline arcana, and the
    // bloodline class skill grant.
    //
    // Corrected 2026-07-29: this token list also required "bonus spells" and "bonus
    // feats", which was accurate while the bloodline's 3rd+-level progression was
    // unimplemented. `ground_sorcerer_arcane_bloodline_progression` now transcribes
    // that whole progression from the corpus, so a blocker still naming those as a
    // remaining burden would be asserting a gap that has closed. The negative
    // assertion below replaces them, so this stays a real check rather than a
    // silently loosened one.
    let arcane_bond = claim_blocking(&computation, ARCANE_BOND_BLOCKER_ID);
    assert!(
        arcane_bond.message.contains("Arcane Bond"),
        "sorcerer arcane-bond blocker must name the Arcane Bond execution burden: {}",
        arcane_bond.message
    );
    for token in ["arcana", "class skill"] {
        assert!(
            arcane_bond.message.contains(token),
            "arcane-bond blocker must name the remaining '{token}' bloodline burden: {}",
            arcane_bond.message
        );
    }
    assert!(
        !arcane_bond.message.contains("bonus spells and bonus feats at 3rd+ level are not \
             implemented"),
        "the bloodline bonus spells/feats are grounded now; the blocker must not keep claiming \
         them as unimplemented: {}",
        arcane_bond.message
    );
    assert!(
        !arcane_bond.message.contains("Eschew Materials"),
        "arcane-bond blocker must not fold in the now-grounded Eschew Materials grant: {}",
        arcane_bond.message
    );
    assert!(
        !arcane_bond.message.contains("bloodline selection"),
        "arcane-bond blocker must not still claim the now-grounded bloodline selection is \
         unimplemented: {}",
        arcane_bond.message
    );
}

// ----- Honesty control: the Arcane Bond blocker must not overclaim Arcane-specific facts
// ----- for a character whose chosen bloodline this seam did not recognize -----

#[test]
fn sorcerer_level1_with_non_arcane_bloodline_choice_stays_bloodline_agnostic() {
    // A non-canonical bloodline selection (e.g. Draconic) is present, but not recognized as
    // the canonical Arcane selection. The choice-slot recognition explanation must still
    // appear (the slot is present), but the arcane-bond blocker must not assert
    // Arcane-specific facts ("Arcane Bond", "Arcane bloodline") for a character who did not
    // choose the Arcane bloodline.
    let fixture = SORCERER_FIXTURE.replace("bloodline:arcane", "bloodline:draconic");
    let input = load(&fixture);
    let computation = compute_pilot_base_chassis(&input);

    let bloodline_choice = explanation(&computation, BLOODLINE_CHOICE_EXPLANATION_ID);
    assert!(
        bloodline_choice.detail.contains("bloodline:draconic"),
        "bloodline choice recognition must name the non-canonical selection present: {}",
        bloodline_choice.detail
    );
    assert_eq!(bloodline_choice.value, 0);

    let arcane_bond = claim_blocking(&computation, ARCANE_BOND_BLOCKER_ID);
    assert!(
        !arcane_bond.message.contains("Arcane Bond") && !arcane_bond.message.contains("Arcane"),
        "arcane-bond blocker must not claim Arcane-specific facts for a non-Arcane bloodline \
         selection: {}",
        arcane_bond.message
    );
}

#[test]
fn sorcerer_level1_with_no_bloodline_choice_stays_bloodline_agnostic() {
    // No bloodline choice slot is present at all. No bloodline-choice recognition
    // explanation may appear, and the arcane-bond blocker must stay bloodline-agnostic
    // rather than asserting Arcane-specific facts for a character with no chosen bloodline.
    let fixture: String = SORCERER_FIXTURE
        .lines()
        .filter(|line| !line.starts_with("choice=choice:sorcerer_bloodline"))
        .collect::<Vec<_>>()
        .join("\n");
    let input = load(&fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, BLOODLINE_CHOICE_EXPLANATION_ID),
        "no bloodline-choice recognition may appear when no bloodline choice slot is present"
    );

    let arcane_bond = claim_blocking(&computation, ARCANE_BOND_BLOCKER_ID);
    assert!(
        !arcane_bond.message.contains("Arcane Bond") && !arcane_bond.message.contains("Arcane"),
        "arcane-bond blocker must not claim Arcane-specific facts when no bloodline is chosen: {}",
        arcane_bond.message
    );
}

#[test]
fn sorcerer_level1_stays_blocked_on_spontaneous_spell_posture_burden() {
    // (v0.6 alpha swarm, risks item 8) SPONTANEOUS_BLOCKER_ID is no longer
    // unconditional -- a bare fixture (zero known spells) is honestly a valid
    // posture and the blocker correctly does not fire. This test is specifically
    // about the blocker staying claim-blocking, so a genuinely invalid known-spell
    // selection is added: "Acid Arrow" is a real 2nd-level sorcerer spell
    // (`sorcerer_spell_list::SORCERER_SPELL_LIST`), not yet accessible at sorcerer
    // level 1 (access ceiling 1st).
    let mut input = load(SORCERER_FIXTURE);
    input.chosen.spells_selected.push(SpellSelection {
        spell_id: "Acid Arrow".to_owned(),
        source_class_id: "class:sorcerer".to_owned(),
        acquisition_mode: AcquisitionMode::Known,
    });
    let computation = compute_pilot_base_chassis(&input);

    // The spontaneous known-spell / slot posture burden must be a separate, explicit,
    // claim-blocking diagnostic.
    let spontaneous = claim_blocking(&computation, SPONTANEOUS_BLOCKER_ID);
    assert!(
        spontaneous.message.contains("spontaneous")
            && spontaneous.message.contains("known-spell")
            && spontaneous.message.contains("Acid Arrow")
            && spontaneous.message.contains("not yet accessible"),
        "sorcerer spell blocker must name the spontaneous known-spell posture burden and the \
         real unmet condition: {}",
        spontaneous.message
    );

    // The two remaining burdens are genuinely distinct diagnostics.
    assert_ne!(
        ARCANE_BOND_BLOCKER_ID, SPONTANEOUS_BLOCKER_ID,
        "arcane-bond and spontaneous burdens must be separate diagnostics"
    );
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("sorcerer"))
        .count();
    assert_eq!(
        distinct_blocking, 2,
        "sorcerer must leave exactly two class-specific claim-blocking diagnostics: {:?}",
        computation.diagnostics
    );
}

#[test]
fn sorcerer_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(SORCERER_FIXTURE);

    // The integrated posture is blocked, never a counterfeit computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked spell baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the spell-bearing path -----

#[test]
fn spell_baseline_preserves_human_race_seam() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "spell baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "spell baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "spell baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative controls: the spell baseline must not leak onto other classes/levels -----

#[test]
fn fighter_and_rogue_do_not_gain_sorcerer_recognition() {
    // A supported Fighter must not gain a sorcerer spell-baseline recognition record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a sorcerer spell-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("sorcerer")),
        "Fighter must not surface sorcerer burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    // A Rogue must stay a plain blocked negative control, never a sorcerer baseline.
    let rogue_fixture = SORCERER_FIXTURE.replace("class:sorcerer:1", "class:rogue:1");
    let rogue = load(&rogue_fixture);
    let rogue_computation = compute_pilot_base_chassis(&rogue);
    assert!(
        rogue_computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "Rogue chassis must remain claim-blocked"
    );
    assert!(
        !has_explanation(&rogue_computation, RECOGNITION_ID)
            && !rogue_computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("sorcerer")),
        "Rogue must not surface any sorcerer recognition or burden diagnostics: {:?}",
        rogue_computation.diagnostics
    );
}

#[test]
fn sorcerer_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_sorcerer_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid
    // level-range gate idiom); this negative control is superseded, not violated —
    // pin the new truth here too so this file stays internally consistent.
    let level_2 = SORCERER_FIXTURE.replace("class:sorcerer:1", "class:sorcerer:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-2 Sorcerer is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Sorcerer must stay claim-blocked in this slice"
    );
}

#[test]
fn sorcerer_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_sorcerer_level3_progression.rs) widened the level-range gate to
    // level 3 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Ranger level-range
    // gate idiom); this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent. Level 4 was in turn
    // widened by a further SD13-E5 slice, covered by
    // `sorcerer_level_4_was_later_widened_into_the_supported_tranche` below, and the
    // frontier is now level 5, covered by
    // `sorcerer_level_5_is_not_promoted_by_this_slice` in
    // `tests/sd13_sorcerer_level4_progression.rs`.
    let level_3 = SORCERER_FIXTURE.replace("class:sorcerer:1", "class:sorcerer:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-3 Sorcerer is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-3 Sorcerer must stay claim-blocked in this slice"
    );
}

#[test]
fn sorcerer_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_sorcerer_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Ranger level-range
    // gate idiom); this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent. Level 5 was in turn
    // widened by a further SD13-E5 slice, covered by
    // `sorcerer_level_5_was_later_widened_into_the_supported_tranche` below.
    let level_4 = SORCERER_FIXTURE.replace("class:sorcerer:1", "class:sorcerer:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-4 Sorcerer is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-4 Sorcerer must stay claim-blocked in this slice"
    );
}

#[test]
fn sorcerer_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 was not yet the frontier (level 1
    // was). A further SD13-E5 slice (tests/sd13_sorcerer_level5_progression.rs) widened
    // the level-range gate to level 5 (mirroring the Fighter/Paladin/Rogue/Barbarian/
    // Monk/Cleric/Bard/Druid/Ranger level-range gate idiom); pin the new truth here too
    // so this file stays internally consistent.
    let level_5 = SORCERER_FIXTURE.replace("class:sorcerer:1", "class:sorcerer:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-5 Sorcerer is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-5 Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix promotes the Sorcerer row to Partial/Computed -----

#[test]
fn matrix_sorcerer_row_is_partial_computed_and_names_choice_arcane_bond_and_spontaneous() {
    let matrix = seeded_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer row must exist");

    // The SD13-E4 Sorcerer decomposition slice grounded Eschew Materials for real,
    // and the SD13-E5 bloodline-choice slice grounds the canonical bloodline
    // choice-slot recognition. Later promoted to Supported/ProductVisible by
    // SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(sorcerer.support_state, SupportState::Supported);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    // grounding_ref stays the same test file citation; this slice only adds tests to it.
    assert!(
        sorcerer
            .grounding_ref
            .contains("sd13_sorcerer_level1_spell_baseline"),
        "sorcerer row must keep citing the SD13-F7/E4/E5 spell-baseline proof surface: {}",
        sorcerer.grounding_ref
    );
    // The note must name the now-grounded Eschew Materials grant and bloodline choice
    // recognition, the still-unproven Arcane Bond / bloodline progression burden, and
    // the still-unproven spontaneous spell posture.
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "sorcerer row must carry a note");
    for token in [
        "Eschew Materials",
        "choice:sorcerer_bloodline",
        "bloodline:arcane",
        "Arcane Bond",
        "spontaneous",
        "spells known",
        "spell slot",
    ] {
        assert!(
            note.contains(token),
            "sorcerer row note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_wizard_row_reflects_current_truth_and_preserves_bard_supported_state() {
    // After SD13-E4-F7 the Bard slice has landed first: the deterministic Human Bard
    // level-1 spontaneous arcane spell-bearing row is now Blocked/Computed/Refreshable
    // with both named burdens. The Sorcerer slice does not regress that state. Wizard
    // was Unverified/Observed at the time this test was first written, but the later
    // SD13-E4-R3 slice executed the Wizard row's own merge-receipt obligation,
    // promoting it to Blocked/Computed, a further SD13-E4 Wizard decomposition
    // slice grounded Scribe Scroll for real, promoting it again to Partial/Computed,
    // and SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17)
    // promoted it again to Supported/ProductVisible; this negative control now pins
    // that current truth.
    let matrix = seeded_current_truth();

    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");
    assert_eq!(
        wizard.support_state,
        SupportState::Supported,
        "wizard row must be Supported after the Class Progression Catalog browser UI-surfacing work"
    );
    assert_eq!(
        wizard.evidence_tier,
        EvidenceTier::ProductVisible,
        "wizard row must be ProductVisible after the Class Progression Catalog browser UI-surfacing work"
    );

    // Bard must not regress; it was later promoted to Partial/Computed by its own
    // SD13-E4 decomposition slice (Bardic Knowledge grounded for real), then to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-16).
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard row must exist");
    assert_eq!(
        bard.support_state,
        SupportState::Supported,
        "bard row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(
        bard.evidence_tier,
        EvidenceTier::ProductVisible,
        "bard row must be ProductVisible after the SD-19 class-row promotion"
    );
    assert_eq!(
        bard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof,
        "bard row must stay anchored to its live Bard proof after the Sorcerer slice"
    );
}

#[test]
fn matrix_preserves_paladin_hybrid_supported_product_visible_truth() {
    let matrix = seeded_current_truth();
    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences), then to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-17).
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Supported,
        "paladin hybrid row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(paladin.evidence_tier, EvidenceTier::ProductVisible);

    // Ranger was later promoted to Partial/Computed by its own SD13-E3 Ranger
    // decomposition slice (Track grounded for real).
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Supported,
        "ranger hybrid row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
}

#[test]
fn matrix_does_not_promote_any_row_to_supported_or_lossy() {
    let matrix = seeded_current_truth();
    assert!(
        !matrix
            .rows
            .iter()
            // school.abjuration/illusion.spell_reachability were later promoted to
            // Supported/Product-visible by SD-19's operator-driven UI-surfacing work
            // (2026-07-16) -- excluded here, not an unintended promotion by this slice.
            .any(|r| (r.support_state == SupportState::Supported
                && r.row_id != "school.abjuration.spell_reachability"
                && r.row_id != "school.illusion.spell_reachability"
                && r.row_id != "school.conjuration.spell_reachability"
                && r.row_id != "school.divination.spell_reachability"
                && r.row_id != "school.enchantment.spell_reachability"
                && r.row_id != "school.evocation.spell_reachability"
                && r.row_id != "school.necromancy.spell_reachability"
                && r.row_id != "school.transmutation.spell_reachability"
                && r.row_id != "school.universal.spell_reachability"
                && r.row_id != "equipment.arms_armor.equipment_reachability"
                && r.row_id != "equipment.general.equipment_reachability"
                && r.row_id != "equipment.magic_items.equipment_reachability"
                && r.row_id != "race.human.pilot_semantics"
                && r.row_id != "race.dwarf.bounded_semantics"
                && r.row_id != "race.elf.bounded_semantics"
                && r.row_id != "race.gnome.bounded_semantics"
                && r.row_id != "race.half_elf.bounded_semantics"
                && r.row_id != "race.half_orc.bounded_semantics"
                && r.row_id != "race.halfling.bounded_semantics"
                && r.row_id != "class.fighter.level_1_pilot"
                && r.row_id != "class.fighter.levels_2_10"
                && r.row_id != "class.monk.bounded_progression"
                && r.row_id != "class.druid.progression_and_spell_burden"
                && r.row_id != "class.barbarian.bounded_progression"
                && r.row_id != "class.cleric.progression_and_spell_burden"
                && r.row_id != "class.wizard.progression_and_spell_burden"
                && r.row_id != "class.rogue.bounded_progression"
                && r.row_id != "class.sorcerer.progression_and_spell_burden"
                && r.row_id != "class.bard.progression_and_spell_burden"
                && r.row_id != "class.paladin.hybrid_chassis_and_spell_burden"
                && r.row_id != "class.ranger.hybrid_chassis_and_spell_burden"
                && r.row_id != "interaction.human_bonus_feat_ability_bonus.pilot_pressure"
                && r.row_id != "equipment.equipmods.equipment_reachability")
                || r.support_state == SupportState::Lossy),
        "the Sorcerer slice must not promote any row to Supported or Lossy"
    );
}
