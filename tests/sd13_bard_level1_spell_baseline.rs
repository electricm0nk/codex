//! SD13-E4-F7 Bard level-1 spontaneous arcane spell-bearing baseline proof.
//!
//! Proves the second honest SD13-F7 spell-bearing slice after Sorcerer: the live
//! rules-core surface ingests a deterministic Human `class:bard:1` input, leaves
//! direct computed evidence that recognizes the Bard level-1 spell-bearing class
//! identity rather than treating it as an undocumented packet placeholder, and yet
//! stays explicitly claim-blocked. It also pins the matrix reclassification of the
//! Bard row from `Unverified` / `Observed` to `Blocked` / `Computed`, while proving
//! Sorcerer stays `Blocked` / `Computed`, Wizard stays `Unverified` / `Observed`,
//! and the accepted Paladin/Ranger hybrid rows stay `Blocked` / `Computed`.
//!
//! The SD13-E4 Bard decomposition slice further splits the original combined
//! Bardic Knowledge + Bardic Music chassis-class-feature blocker into two named
//! diagnostics and grounds one of them for real: Bardic Knowledge (PF1 Core
//! Rulebook: a flat competence bonus on Knowledge checks equal to half the bard's
//! level, minimum +1, that also lets the bard make any Knowledge check untrained)
//! is computed as `class_chassis.bard.bardic_knowledge`. This promotes the matrix
//! row from `Blocked` to `Partial` / `Computed`.
//!
//! The SD13-E5 Bardic Performance grounding slice then grounds the flat Bardic
//! Performance surface for real: the rounds-per-day budget (PF1 Core Rulebook: a
//! level-1 bard can use bardic performance for 4 + Charisma modifier rounds per
//! day) is computed as `class_chassis.bard.bardic_performance_rounds_per_day`,
//! and the Inspire Courage flat level-1 magnitude (+1 competence bonus on attack
//! and weapon damage rolls, +1 morale bonus on saves against charm and fear
//! effects) is computed as `class_chassis.bard.inspire_courage_bonus`. The
//! performance-state engine (start/maintain action economy, round tracking and
//! consumption) and the other level-1 performances (countersong, distraction,
//! fascinate) stay claim-blocked by the narrowed
//! `class_feature.bard.bardic_performance_execution.unsupported` diagnostic. The
//! matrix row stays `Partial` / `Computed`.
//!
//! It is intentionally not a Bard-class-feature engine and not a spell engine. It
//! fabricates no bardic performance execution (no start/maintain action economy,
//! no round tracking or consumption, no countersong, distraction, or fascinate
//! resolution), no full Knowledge-check resolution (no skill ranks, no ability
//! modifier, no untrained-check gate), no spell slots, no spells known, no spell
//! DCs, no bonus spells, no prepared posture, no school choice, and no general
//! spell totals, and it grounds no Bard level 2+. It also preserves the accepted
//! Human race seam on the spell-bearing path.

use codex::rules_core::character_input::{
    AcquisitionMode,
    ActiveState,
    ClassAbilityActivation,
    SpellSelection,
};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    HeadlessReceiptStatus,
    PilotBaseChassisComputation,
    build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const BARD_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.bard";
const BARDIC_KNOWLEDGE_ID: &str = "class_chassis.bard.bardic_knowledge";
const BARDIC_PERFORMANCE_ROUNDS_ID: &str = "class_chassis.bard.bardic_performance_rounds_per_day";
const INSPIRE_COURAGE_ID: &str = "class_chassis.bard.inspire_courage_bonus";
const FASCINATE_DC_ID: &str = "class_chassis.bard.fascinate_dc";
const FASCINATE_AFFECTED_CREATURES_ID: &str = "class_chassis.bard.fascinate_affected_creatures";
const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const SPELL_LEVEL_ACCESS_ID: &str = "class_chassis.bard.spontaneous.spell_level_access";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.bard.spontaneous_known_and_per_day.unsupported";

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

// ----- Direct runtime evidence: the spell-bearing identity is acknowledged -----

#[test]
fn bard_level1_leaves_direct_spell_baseline_recognition_evidence() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Bard spell-bearing identity is recognized
    // on the compute path, not silently dropped as an undocumented packet placeholder.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:bard") && recognition.detail.contains("level 1"),
        "bard recognition must name the class:bard:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("spell"),
        "bard recognition must name the spell-bearing identity: {}",
        recognition.detail
    );

    // It is recognition only: it must carry no fabricated mechanical value (+0) and must
    // not fabricate a Fighter-style computed chassis.
    assert_eq!(
        recognition.value, 0,
        "bard spell baseline recognition must carry no fabricated value (+0)"
    );
    // (v0.6 alpha swarm, risks item 8) Bard is now recognized by
    // table_class_id (3/4 BAB), so the generic class-chassis explanation IS
    // surfaced; the value still floors to 0 at level 1, only presence changed.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "bard level 1's real 3/4 base attack bonus formula floors to 0 (floor(3/4 * 1) = 0)"
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "bard is now recognized by table_class_id and must surface its base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (CHA 15 -> +2).
    assert_eq!(computation.ability_modifiers.charisma, 3);
}

#[test]
fn bard_level1_fabricates_no_spell_or_class_feature_math() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // No explanation may fabricate spell slots, spells known, DCs, bonus spells, prepared
    // posture, school choice, or general spell totals, and none may fabricate bardic
    // performance execution. The recognition record and the grounded flat pillars
    // (Bardic Knowledge, the bardic performance rounds-per-day budget, the Inspire
    // Courage flat magnitude, the Fascinate flat Will-save DC and
    // affected-creature-count formulas, and — as of a further SD13-E5 slice — the
    // Inspire Competence level-gate record, correctly absent (value 0) at level 1, and
    // — as of the further SD13-E5 access-ladder slice — the spontaneous spell-level
    // ACCESS record, which carries value 1 at level 1 because a bard casts 1st-level
    // spells from level 1 per the raw table row "1/—/…", grounding access only, never
    // per-day counts) are the only allowed spell/bardic-tagged explanations. Fascinate's own resolution (an
    // actual Will save, targeting, range/attention checking) stays ungrounded; only its
    // two flat numbers are allowed.
    let allowed_ids = [
        RECOGNITION_ID,
        BARDIC_KNOWLEDGE_ID,
        BARDIC_PERFORMANCE_ROUNDS_ID,
        INSPIRE_COURAGE_ID,
        FASCINATE_DC_ID,
        FASCINATE_AFFECTED_CREATURES_ID,
        INSPIRE_COMPETENCE_ID,
        SPELL_LEVEL_ACCESS_ID,
        // (v0.6 alpha swarm, risks item 8) the bare fixture has no bardic
        // performance activation, a genuinely valid "not performing"
        // posture, so ground_or_block_bard_bardic_performance's honest
        // (value 0) inactive-branch record now surfaces too.
        "class_feature.bard.bardic_performance_execution.not_performing",
        // (v0.6 alpha swarm, risks item 8, known-spell closure) the bare
        // fixture has zero known spells, a genuinely valid posture, so the
        // real known-spell-count record (honestly 0) now surfaces too.
        "class_spell.bard.known_spells",
    ];
    for explanation in &computation.explanations {
        assert!(
            allowed_ids.contains(&explanation.id.as_str())
                // The base_spells_per_day family (a further SD13-E5 slice,
                // tests/sd13_bard_spells_per_day_counts.rs) fires at every
                // supported level as literal table records; allowing it by
                // prefix keeps this control accurate without weakening it.
                || explanation
                    .id
                    .starts_with("class_chassis.bard.spontaneous.base_spells_per_day.")
                // The spell_save_dc family (a further SD13-E5 slice,
                // tests/sd13_bard_spell_save_dcs.rs): base DC arithmetic
                // records, allowed by prefix like the per-day family.
                || explanation
                    .id
                    .starts_with("class_chassis.bard.spontaneous.spell_save_dc.")
                // The spells_known family (a further SD13-E5 slice,
                // tests/sd13_bard_spells_known_counts.rs): base known-count
                // table records, allowed by prefix like the other families.
                || explanation
                    .id
                    .starts_with("class_chassis.bard.spontaneous.spells_known.")
                // The bonus_spells_per_day family (a further SD13-E5 slice,
                // tests/sd13_bard_bonus_spells.rs): Charisma bonus-slot
                // counts from the shared PF1 table, allowed by prefix.
                || explanation
                    .id
                    .starts_with("class_chassis.bard.spontaneous.bonus_spells_per_day.")
                // The total_spells_per_day family (a further SD13-E5 slice,
                // tests/sd13_bard_total_spells_per_day.rs): the pure sum of
                // the grounded base and bonus records, allowed by prefix.
                || explanation
                    .id
                    .starts_with("class_chassis.bard.spontaneous.total_spells_per_day.")
                || (!explanation.id.contains("spell")
                    && !explanation.id.contains("bardic")
                    && !explanation.id.contains("music")
                    && !explanation.id.contains("inspire")
                    && !explanation.id.contains("fascinate"))
                // SD-34 bucket-B batch cycle (this gate was missed by the same-shaped
                // carve-out `sd13_bard_level4..8_progression.rs` already applied
                // elsewhere): `class_feature_grant_consumer` now emits real,
                // citation-backed `class_feature.bard.corpus_record.*` roster ids for
                // Bard (`decisions.md` section 18) -- a flat "this class feature exists,
                // granted from level N" fact, never a fabricated spell/performance
                // MAGNITUDE. This additive prefix carve-out admits that shape without
                // touching any existing `allowed_ids` entry or weakening this control
                // for any spell/bardic-tagged id outside that one namespace.
                || explanation.id.starts_with("class_feature.bard.corpus_record."),
            "no fabricated spell or bardic-class-feature explanation is allowed beyond the \
             +0 recognition and the grounded flat pillars: {explanation:?}"
        );
    }
    // Countersong and Distraction stay fully unproven under any name — both require an
    // opposed Perform-check-vs-effect substitution resolution, not a flat number, so
    // neither is attempted this slice. Fascinate itself is now partially grounded (DC and
    // affected-creature count only), so it is no longer part of this negative control.
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| { e.id.contains("countersong") || e.id.contains("distraction") }),
        "no explanation may fabricate countersong / distraction math: {:?}",
        computation.explanations
    );
    // The recognition itself asserts it fabricates no spell math and no performance execution.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert_eq!(recognition.value, 0);
}

// ----- Grounded: the Bardic Knowledge pillar is computed for real -----

#[test]
fn bard_level1_grounds_bardic_knowledge_class_feature_bonus() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Bardic Knowledge: a bard adds half his bard level (minimum 1) to
    // Knowledge skill checks and may make all Knowledge skill checks untrained. At bard
    // level 1 that flat competence bonus is max(1 / 2, 1) = 1. This does NOT bundle in the
    // Bard's Intelligence modifier: the INT modifier is already part of the ordinary
    // Knowledge skill check (rank + ability modifier + misc bonuses), not an additional
    // term inside the Bardic Knowledge class-feature bonus itself, so grounding it requires
    // no skill-rank state and no ability-modifier addition.
    let bardic_knowledge = explanation(&computation, BARDIC_KNOWLEDGE_ID);
    assert_eq!(
        bardic_knowledge.value, 1,
        "Bardic Knowledge at bard level 1 must equal max(level / 2, 1) = 1: {bardic_knowledge:?}"
    );
    assert!(
        bardic_knowledge.detail.contains("Knowledge"),
        "Bardic Knowledge explanation must name the Knowledge-check bonus it grants: {}",
        bardic_knowledge.detail
    );

    // Grounding Bardic Knowledge must not silently grant Fighter-style base-attack math or
    // any other unrelated computed chassis.
    assert_eq!(computation.base_attack_bonus, 0);
}

// ----- Grounded: the flat Bardic Performance surface is computed for real -----

#[test]
fn bard_level1_grounds_bardic_performance_rounds_per_day() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Bardic Performance: a level-1 bard can use bardic performance
    // for a number of rounds per day equal to 4 + his Charisma modifier. The fixture's
    // Charisma 15 + 2 Human racial (CG-03 fix) yields a +3 modifier, so the daily
    // budget is 4 + 3 = 7 rounds.
    let rounds = explanation(&computation, BARDIC_PERFORMANCE_ROUNDS_ID);
    assert_eq!(
        rounds.value, 7,
        "bardic performance rounds per day must equal 4 + CHA modifier (4 + 3 = 7): {rounds:?}"
    );
    assert!(
        rounds.detail.contains("4 + ") && rounds.detail.contains("Charisma"),
        "the rounds-per-day explanation must name the 4 + Charisma-modifier formula: {}",
        rounds.detail
    );

    // Grounding the daily budget must not fabricate the performance-state engine: the
    // explanation itself must disclaim round tracking/consumption and action economy.
    assert!(
        rounds.detail.contains("no round") || rounds.detail.contains("round tracking"),
        "the rounds-per-day explanation must disclaim the ungrounded round-tracking engine: {}",
        rounds.detail
    );
}

#[test]
fn bard_level1_grounds_inspire_courage_flat_magnitude() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Inspire Courage at bard level 1: a +1 competence bonus on attack
    // and weapon damage rolls and a +1 morale bonus on saving throws against charm and
    // fear effects. Only the flat +1 magnitude is grounded here; the performance-state
    // engine that would apply it stays claim-blocked.
    let inspire_courage = explanation(&computation, INSPIRE_COURAGE_ID);
    assert_eq!(
        inspire_courage.value, 1,
        "inspire courage at bard level 1 must carry the flat +1 magnitude: {inspire_courage:?}"
    );
    for token in ["competence", "attack", "damage", "morale", "charm", "fear"] {
        assert!(
            inspire_courage.detail.contains(token),
            "the inspire courage explanation must name the '{token}' component of the PF1 \
             level-1 magnitude: {}",
            inspire_courage.detail
        );
    }

    // The flat magnitude must not leak into the integrated Fighter-style totals: nothing
    // applies the bonus, because the performance-state engine is not implemented.
    assert_eq!(computation.base_attack_bonus, 0);
    assert_eq!(
        computation.baseline_melee_attack_bonus, 0,
        "the flat inspire courage magnitude must not be applied to any combat total"
    );
}

// ----- Still blocked: performance execution and the spontaneous spell posture burden -----

#[test]
fn bard_level1_names_the_permanently_unmodeled_other_performances() {
    // (v0.6 alpha swarm, risks item 8) The old unconditional
    // "class_feature.bard.bardic_performance_execution.unsupported"
    // diagnostic is retired outright:
    // ground_or_block_bard_bardic_performance is now a real, conditional
    // engine (mirrors Barbarian's Rage). The permanently unconditional part
    // that remains is a NON-blocking diagnostic naming the other bardic
    // performances (Countersong, Distraction, etc.) that stay entirely
    // unexecuted regardless of activation state -- it does not block a
    // genuinely valid Inspire Courage posture.
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let other_performances = computation
        .diagnostics
        .iter()
        .find(|d| {
            d.id == "class_feature.bard.bardic_performance_execution.other_performances_not_modeled"
        })
        .expect("the other-performances-not-modeled diagnostic must always fire");
    assert!(
        !other_performances.claim_blocking,
        "the other-performances-not-modeled diagnostic must not block a valid Inspire Courage \
         posture: {other_performances:?}"
    );
    for token in ["Countersong", "Distraction"] {
        assert!(
            other_performances.message.contains(token),
            "the other-performances-not-modeled diagnostic must name '{token}': {}",
            other_performances.message
        );
    }
    assert!(
        !other_performances.message.contains("bardic knowledge"),
        "the other-performances-not-modeled diagnostic must not re-bundle the grounded Bardic \
         Knowledge pillar: {}",
        other_performances.message
    );

    // On this bare (not-performing) fixture, the honest inactive-branch
    // record grounds, not a claim-blocking diagnostic.
    let not_performing = explanation(
        &computation,
        "class_feature.bard.bardic_performance_execution.not_performing",
    );
    assert_eq!(
        not_performing.value, 0,
        "the not-performing record carries no fabricated mechanical value"
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.bard.bardic_performance_execution")
                && d.claim_blocking),
        "a genuinely valid not-performing posture must not claim-block on performance execution: {:?}",
        computation.diagnostics
    );

    // The old combined diagnostic id must no longer appear at all.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.bard.bardic_knowledge_and_music.unsupported"),
        "the old combined Bardic Knowledge + Music diagnostic must be split, not merely renamed"
    );
    // The pre-E5 broad bardic-music diagnostic id must be narrowed away, not kept alongside.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.bard.bardic_music.unsupported"),
        "the broad bardic-music diagnostic must be narrowed to performance execution now that \
         the flat performance surface is grounded"
    );
}

#[test]
fn bard_level1_stays_blocked_on_a_genuine_bardic_performance_execution_violation() {
    // (v0.6 alpha swarm, risks item 8) Proving the real, conditional
    // performance-execution engine still claim-blocks on a genuine posture
    // violation: an active bardic-performance activation whose
    // rounds_consumed_today exceeds the grounded rounds-per-day budget
    // (4 + Cha modifier +3 = 7 at level 1 on this fixture).
    let mut input = load(BARD_FIXTURE);
    input.chosen.class_ability_activations.push(ClassAbilityActivation {
        ability_id: "bardic_performance".to_owned(),
        active_state: ActiveState::EquippedActive,
        rounds_consumed_today: Some(8),
    });
    let computation = compute_pilot_base_chassis(&input);

    let execution = claim_blocking(
        &computation,
        "class_feature.bard.bardic_performance_execution.rounds_exceeded",
    );
    for token in ["rounds consumed", "exceeding", "rounds-per-day budget"] {
        assert!(
            execution.message.contains(token),
            "the performance-execution blocker must name the '{token}' burden: {}",
            execution.message
        );
    }

    // No performance bonus is fabricated for an over-budget activation.
    assert!(
        !has_explanation(&computation, "class_feature.bard.bardic_performance_execution.active"),
        "an over-budget performance activation must not ground the active-performance explanation record"
    );
}

#[test]
fn bard_level1_stays_blocked_on_a_genuine_spontaneous_spell_posture_violation() {
    // (v0.6 alpha swarm, risks item 8, known-spell closure) SPONTANEOUS_BLOCKER_ID
    // is no longer unconditional -- the bare fixture is a genuinely valid
    // posture (zero known spells), so this test (whose whole purpose is
    // proving the blocker still fires) now needs a real violation: "Alter
    // Self" is a real PF1 Core Rulebook 2nd-level bard spell, not yet
    // accessible at bard level 1 (access ceiling 1st level).
    let mut input = load(BARD_FIXTURE);
    input.chosen.spells_selected.push(SpellSelection {
        spell_id: "Alter Self".to_owned(),
        source_class_id: "class:bard".to_owned(),
        acquisition_mode: AcquisitionMode::Known,
    });
    let computation = compute_pilot_base_chassis(&input);

    let spontaneous = claim_blocking(&computation, SPONTANEOUS_BLOCKER_ID);
    assert!(
        spontaneous.message.contains("spontaneous") && spontaneous.message.contains("Alter Self"),
        "bard spell blocker must name the spontaneous posture burden and the violating spell: {}",
        spontaneous.message
    );
    assert!(
        spontaneous.message.contains("not yet accessible"),
        "bard spell blocker must explain why the spell is not yet accessible: {}",
        spontaneous.message
    );

    // No spell is fabricated for an invalid known-spell posture.
    assert!(
        !has_explanation(&computation, "class_spell.bard.known_spells"),
        "an invalid known-spell posture must not ground the known-spell-count explanation record"
    );

    // Only ONE class-specific claim-blocking diagnostic remains: this bare
    // fixture (aside from the injected violation) has no bardic performance
    // activation, a genuinely valid posture, so rounds_exceeded correctly
    // does not fire, and other_performances_not_modeled is non-blocking.
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("bard"))
        .count();
    assert_eq!(
        distinct_blocking, 1,
        "bard must leave exactly one class-specific claim-blocking diagnostic (the injected \
         spontaneous-spell violation) on a valid performance posture: {:?}",
        computation.diagnostics
    );
}

#[test]
fn bard_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(BARD_FIXTURE);

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
    let input = load(BARD_FIXTURE);
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
fn fighter_sorcerer_and_rogue_do_not_gain_bard_recognition() {
    // A supported Fighter must not gain a bard spell-baseline recognition record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a bard spell-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("bard")),
        "Fighter must not surface bard burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    // The accepted Sorcerer baseline must stay a Sorcerer baseline, never a Bard baseline.
    let sorcerer = load(include_str!(
        "fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt"
    ));
    let sorcerer_computation = compute_pilot_base_chassis(&sorcerer);
    assert!(
        !has_explanation(&sorcerer_computation, RECOGNITION_ID)
            && !sorcerer_computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("bard")),
        "Sorcerer must not surface any bard recognition or burden diagnostics: {:?}",
        sorcerer_computation.diagnostics
    );

    // A Rogue must stay a plain blocked negative control, never a Bard baseline.
    let rogue_fixture = BARD_FIXTURE.replace("class:bard:1", "class:rogue:1");
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
                .any(|d| d.id.contains("bard")),
        "Rogue must not surface any bard recognition or burden diagnostics: {:?}",
        rogue_computation.diagnostics
    );
}

#[test]
fn bard_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_bard_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric
    // level-range gate idiom) and confirmed every one of the formulas below
    // extends to level 2 unchanged or via the same formula; this negative
    // control is superseded, not violated — pin the new truth here too so this
    // file stays internally consistent.
    let level_2 = BARD_FIXTURE.replace("class:bard:1", "class:bard:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-2 Bard is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BARDIC_KNOWLEDGE_ID)
            && has_explanation(&computation, BARDIC_PERFORMANCE_ROUNDS_ID)
            && has_explanation(&computation, INSPIRE_COURAGE_ID),
        "level-2 Bard is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, FASCINATE_DC_ID)
            && has_explanation(&computation, FASCINATE_AFFECTED_CREATURES_ID),
        "level-2 Bard is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Bard must stay claim-blocked in this slice"
    );
}

#[test]
fn bard_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_bard_level3_progression.rs) widened the level-range gate to
    // level 3 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/
    // Sorcerer/Wizard level-range gate idiom) and confirmed every one of the
    // formulas below extends to level 3 unchanged or via the same formula; this
    // negative control is superseded, not violated — pin the new truth here too
    // so this file stays internally consistent.
    let level_3 = BARD_FIXTURE.replace("class:bard:1", "class:bard:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-3 Bard is supported since the SD13-E5 level-3 slice"
    );
    assert!(
        has_explanation(&computation, BARDIC_PERFORMANCE_ROUNDS_ID),
        "level-3 Bard is supported since the SD13-E5 level-3 slice"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-3 Bard must stay claim-blocked in this slice"
    );
}

#[test]
fn bard_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_bard_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/
    // Sorcerer/Wizard level-range gate idiom) and confirmed every one of the
    // formulas below extends to level 4 unchanged or via the same formula; this
    // negative control is superseded, not violated — pin the new truth here too
    // so this file stays internally consistent.
    let level_4 = BARD_FIXTURE.replace("class:bard:1", "class:bard:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-4 Bard is supported since the SD13-E5 level-4 slice"
    );
    assert!(
        has_explanation(&computation, BARDIC_PERFORMANCE_ROUNDS_ID),
        "level-4 Bard is supported since the SD13-E5 level-4 slice"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-4 Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix promotes the Bard row to Partial/Computed -----

#[test]
fn matrix_bard_row_is_partial_computed_and_names_remaining_burdens() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard row must exist");

    // The SD13-E4 Bard decomposition slice grounded Bardic Knowledge (Blocked to
    // Partial), and the SD13-E5 Bardic Performance grounding slice grounds the flat
    // performance surface (rounds per day, inspire courage magnitude). SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-16) promoted the
    // row again to Supported/ProductVisible: every named grounded milestone above is
    // now surfaced live in the desktop app; the performance-execution engine and the
    // entire spontaneous spell burden remain named, honest gaps, not further
    // per-cycle widening targets.
    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        bard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        bard.grounding_ref
            .contains("sd13_bard_level1_spell_baseline"),
        "bard row must cite the SD13-F7/E4/E5 spell-baseline proof surface: {}",
        bard.grounding_ref
    );
    // The note must name the grounded Bardic Knowledge and flat Bardic Performance
    // pillars, the still-blocked performance-execution engine and remaining level-1
    // performances, and the still-entirely-unproven spontaneous spell posture.
    let note = bard.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "bard partial row must carry a note");
    for token in [
        "bardic knowledge",
        "bardic performance",
        "rounds per day",
        "inspire courage",
        "countersong",
        "distraction",
        "fascinate",
        "spontaneous",
        "spells known",
        "spells per day",
    ] {
        assert!(
            note.contains(token),
            "bard partial note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_wizard_row_reflects_current_truth_after_bard_slice() {
    // The Bard slice itself left Wizard Unverified/Observed; the later SD13-E4-R3
    // slice executed the Wizard row's own merge-receipt obligation, promoting it to
    // Blocked/Computed, a further SD13-E4 Wizard decomposition slice grounded
    // Scribe Scroll for real, promoting it again to Partial/Computed, and SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-17) promoted it
    // again to Supported/ProductVisible. This negative control now pins that
    // current truth rather than the Bard-slice-only snapshot.
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
}

#[test]
fn matrix_keeps_sorcerer_and_paladin_supported_after_bard_slice() {
    let matrix = seeded_current_truth();

    // Sorcerer was later promoted to Partial/Computed by its own SD13-E4 Sorcerer
    // decomposition slice (Eschew Materials grounded for real), then to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-17).
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer row must exist");
    assert_eq!(
        sorcerer.support_state,
        SupportState::Supported,
        "sorcerer row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);

    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences).
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Supported,
        "paladin row must be Supported after the SD-19 class-row promotion"
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
        "ranger row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
}

#[test]
fn matrix_does_not_promote_any_row_to_supported_or_lossy_after_bard_slice() {
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
        "the Bard slice must not promote any row to Supported or Lossy"
    );
}
