#[allow(unused_imports)]
pub(crate) use super::*;

/// Race ids (lowercase, no `"race:"` prefix) for which this engine has ANY
/// real magnitude consumer of a race_trait record's numbers: either one of
/// the 7 Core Rulebook races' hardcoded `explain_<race>_race_seam`
/// (`explain_human_pilot_race_seam` for Human) functions, membership in
/// [`ALTERNATE_TRAIT_SAVE_BONUSES`]'s or
/// [`ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES`]'s own race-id column, or
/// [`SIZE_ONLY_RACE_TRAIT_BUNDLE`]'s 4 races (each has a real
/// `explain_size_only_race_trait_bundle` seam for their own `~ Size`
/// record specifically).
///
/// **Why this exists (SD31-W12-INTEGRATE-001)**: `v06_work_inventory.rs`'s
/// `probe_race_trait_corpus` promoted any LOADED, role-classified
/// `computed` race_trait record straight to `grounded` (-> board `done`)
/// regardless of whether anything downstream reads its magnitude at all --
/// confirmed by adversarial review as the identical "credit resting on a
/// DIFFERENT record's computation" shape wave 11 found elsewhere, one axis
/// over (a Dwarf-only seam silently crediting a Vanara record carrying the
/// byte-identical `BONUS:VAR` chain). Of the 555-unit standing population,
/// 314 belong to a race with NO seam of any kind and are therefore
/// DEFINITIVELY uncredited by anything this engine computes.
///
/// **Derived, not hand-duplicated** (this program's own "a measurement, not
/// a selection" standard, `ALTERNATE_TRAIT_SAVE_BONUSES`'s own doc comment):
/// the two alternate-trait tables are read directly rather than their
/// race-id columns being re-typed here, so this can never silently drift
/// from them the way a hand-copied list could. The 7 CRB races ARE
/// hand-listed, because their seam is 7 separate hardcoded
/// `explain_<race>_race_seam` functions with no single table to derive
/// from -- `race_ids_with_a_magnitude_consumer_tests::crb_seamed_race_
/// function_count_is_seven` (below) counts this file's own
/// `fn explain_.*_race_seam` definitions via `include_str!` and asserts the
/// count stays 7, so a future race gaining or losing a seam function fails
/// a test rather than silently drifting.
pub fn race_ids_with_a_magnitude_consumer() -> std::collections::BTreeSet<&'static str> {
    const CRB_SEAMED_RACES: &[&str] = &["dwarf", "elf", "gnome", "half-elf", "half-orc", "halfling", "human"];
    let mut set: std::collections::BTreeSet<&'static str> = CRB_SEAMED_RACES.iter().copied().collect();
    for (_, race_id, ..) in ALTERNATE_TRAIT_SAVE_BONUSES {
        set.insert(race_id.strip_prefix("race:").unwrap_or(race_id));
    }
    for (_, race_id, ..) in ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES {
        set.insert(race_id.strip_prefix("race:").unwrap_or(race_id));
    }
    // `SIZE_ONLY_RACE_TRAIT_BUNDLE`'s 4 races (Kobold, Svirfneblin, Goblin,
    // Grippli) have a real `explain_size_only_race_trait_bundle` seam for
    // their own `~ Size` record specifically -- §7's own worked example.
    for (race_id, ..) in SIZE_ONLY_RACE_TRAIT_BUNDLE {
        set.insert(race_id.strip_prefix("race:").unwrap_or(race_id));
    }
    // SD31-W25-RACETRAIT-001: Rougarou, Gillman and Vanara each have a real
    // `explain_<race>_flat_override_race_trait` seam (below, next to
    // `SIZE_ONLY_RACE_TRAIT_BUNDLE`'s own) grounding their flat speed/vision/
    // natural-weapon overrides -- the cross-book lever `OPEN-ISSUES.md` row
    // 353 named. Hand-listed rather than table-derived because, unlike the
    // size-only bundle, the three races' shapes genuinely differ (Gillman
    // also has a Swim speed override, Rougarou also has a natural weapon).
    // SD31-W27-RACETRAIT-001 adds Samsaran and Nagaji, each with FULL
    // per-record coverage of their `computed` population (verified in each
    // seam function's own doc comment, not just the flat speed/vision
    // shape) -- see `FLAT_OVERRIDE_RACE_TRAIT_RACES`'s own doc comment for
    // why full coverage is the load-bearing requirement here, not optional
    // polish.
    for race_id in FLAT_OVERRIDE_RACE_TRAIT_RACES {
        set.insert(race_id);
    }
    set
}

/// Race ids (lowercase, no `"race:"` prefix) with a real
/// `explain_<race>_flat_override_race_trait` seam, per
/// `race_ids_with_a_magnitude_consumer`'s own doc comment above.
///
/// SD31-W27-RACETRAIT-001 adds Samsaran and Nagaji. Both are single-book
/// (Advanced Race Guide, no cross-book alternates in the ingested corpus)
/// and BOTH have full per-record coverage of their `computed`-wiring-class
/// population, verified individually before this const was widened — see
/// `explain_samsaran_flat_override_race_trait`'s and
/// `explain_nagaji_flat_override_race_trait`'s own doc comments for the
/// exact accounting. This full-coverage discipline is a direct response to
/// wave 26's Undine finding (`OPEN-ISSUES.md` row 365, GAMED/not merged):
/// that lane added a race to a race-level seam list while its seam function
/// covered only 3 of 20 reachable records, silently free-crediting the
/// other 17 through the coarse race-level `is_seamed` gate this const still
/// feeds. Adding a race here without first writing an explicit,
/// individually-verified explanation for EVERY reachable `computed` record
/// of that race reproduces that exact gaming vector.
pub(super) const FLAT_OVERRIDE_RACE_TRAIT_RACES: &[&str] =
    &["rougarou", "gillman", "vanara", "samsaran", "nagaji"];

#[cfg(test)]
mod race_ids_with_a_magnitude_consumer_tests {
    use super::*;

    /// Pins the hand-listed CRB half of `race_ids_with_a_magnitude_consumer`
    /// against this file's OWN source text, counted at test time rather than
    /// asserted -- a future race gaining or losing an `explain_*_race_seam`
    /// function changes this count and this test goes red, rather than the
    /// hand-list silently drifting from what the file actually implements.
    #[test]
    fn crb_seamed_race_function_count_is_seven() {
        // SD-36 Epic C1: this test and the `explain_*_race_seam` functions it
        // counts used to live directly in `pilot_compute/mod.rs`. The C1
        // split moved both here, into `race_seams.rs` (a pure code move --
        // `decisions.md` §18/B15 -- so `include_str!` now points at THIS
        // file instead), and the moved functions gained a `pub(super)`
        // visibility qualifier they did not carry as private items of a
        // single flat module; the match below strips that qualifier before
        // checking the `fn explain_..._race_seam(` shape so the count still
        // reflects the functions themselves, not their visibility spelling.
        let source = include_str!("race_seams.rs");
        let count = source
            .lines()
            .filter(|line| {
                let trimmed = line.trim_start();
                let trimmed = trimmed
                    .strip_prefix("pub(super) ")
                    .or_else(|| trimmed.strip_prefix("pub(crate) "))
                    .or_else(|| trimmed.strip_prefix("pub "))
                    .unwrap_or(trimmed);
                trimmed.starts_with("fn explain_") && trimmed.contains("_race_seam(")
            })
            .count();
        assert_eq!(
            count, 7,
            "race_ids_with_a_magnitude_consumer's CRB_SEAMED_RACES lists exactly 7 races -- \
             if this fails, a race seam function was added or removed and the hand-list must \
             be updated to match"
        );
    }

    /// The union is exactly the 19 races this module has ANY seam for --
    /// the 16 pre-SD31-W27-RACETRAIT-001 races plus Samsaran and Nagaji,
    /// each now backed by a real `explain_<race>_flat_override_race_trait`
    /// seam, not a hand-copied name, plus Skinwalker (SD-33 Epic 6,
    /// 2026-08-26): `ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES`'s own 2 new
    /// rows (Werebear-Kin/Wereshark-Kin's `~ Animal-Minded`) are read
    /// straight into this union the SAME way Strix/Grippli/Goblin already
    /// are -- a narrow, measured slice of the race's alternates, not a
    /// full-coverage claim (that stricter bar is
    /// `FLAT_OVERRIDE_RACE_TRAIT_RACES`'s own, this table makes none).
    #[test]
    fn the_union_is_exactly_the_nineteen_seamed_races() {
        let races = race_ids_with_a_magnitude_consumer();
        let expected: std::collections::BTreeSet<&str> = [
            "dwarf", "elf", "gillman", "gnome", "goblin", "grippli", "half-elf", "half-orc",
            "halfling", "hobgoblin", "human", "kobold", "nagaji", "rougarou", "samsaran",
            "skinwalker", "strix", "svirfneblin", "vanara",
        ]
        .into_iter()
        .collect();
        assert_eq!(races, expected);
    }

    /// A race with no seam at all must NOT appear in the set. Nagaji moved
    /// OUT of this list as of SD31-W27-RACETRAIT-001 (it now has a real
    /// flat-override seam, pinned by
    /// `the_union_is_exactly_the_nineteen_seamed_races` above); Aasimar and
    /// Vishkanya remain genuinely unseamed and stay here as the negative
    /// control this test exists to prove can still fail.
    #[test]
    fn an_unseamed_race_is_absent() {
        let races = race_ids_with_a_magnitude_consumer();
        assert!(!races.contains("aasimar"));
        assert!(!races.contains("vishkanya"));
    }
}

/// The Fortitude / Reflex / Will contribution of this character's chosen
/// alternate racial traits.
///
/// Race-gated by construction: a trait key is matched only against its owning
/// race, so a selection copied onto another race contributes nothing.
pub(super) fn alternate_trait_save_bonuses(input: &CharacterInput) -> BaseSaves {
    let selected = selected_alternate_trait_keys(input);
    let mut total = BaseSaves { fortitude: 0, reflex: 0, will: 0 };
    for (key, race_id, fortitude, reflex, will) in ALTERNATE_TRAIT_SAVE_BONUSES {
        if input.chosen.race_id != *race_id || !selected.iter().any(|chosen| chosen == key) {
            continue;
        }
        total.fortitude += *fortitude;
        total.reflex += *reflex;
        total.will += *will;
    }
    total
}

/// SD13-E2/SD18 Half-Elf racial trait bundle explanation seam (mirroring the
/// Dwarf/Elf/Gnome recognition pattern for the fourth non-Human core race, but
/// with a choice-based ability bonus like Human's rather than a fixed pair).
///
/// Surfaces six grounded PF1 Core Rulebook Half-Elf racial trait dimensions
/// (chosen ability-bonus target, size, speed, senses, Keen Senses, Elven
/// Immunities) as explicit `ComputationExplanation` records so the Half-Elf
/// identity is legible on the runtime path rather than left behind the
/// generic `race.semantics.unverified` diagnostic every other non-Human race
/// still receives.
///
/// This function:
///   - runs only when `race_id == race:half-elf`; every other race is unaffected
///     (Human, Dwarf, Elf, and Gnome keep their own seams; every other non-Human
///     race keeps the generic `race.semantics.unverified` diagnostic),
///   - adds no new computed mechanical contribution beyond the flat Keen
///     Senses and Elven Immunities enchantment-save bonus magnitudes: the
///     ability-bonus-target record surfaces the already-computed modifier for
///     the chosen ability as recognition (mirroring
///     `race.human.ability_bonus_target`'s shape), the size/senses records
///     carry the grounded source value as identity only, and Elven
///     Immunities' sleep immunity is a bounded grant-only identity record
///     (no sleep-effect-resolution engine exists in this codebase),
///   - replaces the generic `race.semantics.unverified` diagnostic with a
///     Half-Elf-specific `race.half_elf.bounded_semantics` note naming the
///     still-unproven families explicitly (Adaptability, Multitalented),
///   - is bounded to race recognition only; it deliberately grounds no Half-Elf
///     class-chassis interaction, no other race, and no PF1 alternate ruleset.
pub(super) fn explain_half_elf_race_seam(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HALF_ELF_RACE_ID {
        return;
    }

    // ----- ability bonus (choice-based, like Human) -----
    if let Some(selection) = choice_selection(input, HALF_ELF_ABILITY_BONUS_CHOICE_ID) {
        let ability = selection
            .strip_prefix(ABILITY_SELECTION_PREFIX)
            .unwrap_or(selection);
        let modifier = ability_modifier_for(ability_modifiers, ability);
        explanations.push(ComputationExplanation {
            id: "race.half_elf.trait_bundle.ability_bonus_target".to_owned(),
            value: modifier,
            detail: format!(
                "Half-Elf racial trait bundle — ability bonus: PF1 Core Half-Elf grants a \
                 player-chosen +2 to any one ability score ({HALF_ELF_ABILITY_BONUS_CHOICE_ID} \
                 -> {selection}); the chosen {ability} score yields modifier {modifier:+}. This \
                 is a bounded recognition record naming the chosen target on the deterministic \
                 pilot seam; the chosen score is understood to already reflect the +2 \
                 adjustment, so this record performs no arithmetic beyond surfacing the \
                 already-computed modifier"
            ),
        });
    }

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.half_elf.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Half-Elf racial trait bundle — size: PF1 Core Half-Elf is \
             {HALF_ELF_SIZE_CATEGORY} size (cr_races.lst race:half-elf SIZE:MEDIUM). This is a \
             bounded recognition record naming the Half-Elf size category on the deterministic \
             pilot seam; it contributes no numeric effect to attack rolls, AC, skill checks, \
             ability checks, or any other computed value, so it carries no fabricated \
             mechanical value (+0)"
        ),
    });

    // ----- speed -----
    explanations.push(ComputationExplanation {
        id: "race.half_elf.trait_bundle.speed".to_owned(),
        value: HALF_ELF_BASE_SPEED_FEET,
        detail: format!(
            "Half-Elf racial trait bundle — speed: PF1 Core Half-Elf has a base land speed of \
             {HALF_ELF_BASE_SPEED_FEET} ft \
             (cr_races.lst race:half-elf GAIT:WALK|{HALF_ELF_BASE_SPEED_FEET}). This is a \
             grounded recognition value carrying the Half-Elf base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    explanations.push(ComputationExplanation {
        id: "race.half_elf.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Half-Elf racial trait bundle — senses: PF1 Core Half-Elf grants low-light \
                  vision (cr_races.lst race:half-elf SENSE:Low-Light Vision). This is a bounded \
                  recognition record naming the Half-Elf low-light vision identity on the \
                  deterministic pilot seam; it contributes no computed illumination or \
                  perception-derived effect to any chassis output, so it carries no fabricated \
                  mechanical value (+0)"
            .to_owned(),
    });

    // ----- Keen Senses -----
    explanations.push(ComputationExplanation {
        id: "race.half_elf.trait_bundle.keen_senses".to_owned(),
        value: 2,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — BONUS:SKILL|Perception|KeenSensesBonus|TYPE=Racial, BONUS:VAR|KeenSensesBonus|2
        detail: "Half-Elf racial trait bundle — Keen Senses: PF1 Core Half-Elf grants a flat +2 \
                  racial bonus on Perception skill checks \
                  (core_essentials/races/half_elf/halfelf_abilities_race.lst Keen Senses entry). \
                  This is a bounded recognition record naming only the flat racial-bonus magnitude \
                  on the deterministic pilot seam, not a Perception-check-total engine."
            .to_owned(),
    });

    // ----- Elven Immunities -----
    // Bundles two distinct sub-effects, both grounded honestly, mirroring the
    // already-landed Elf Elven Immunities idiom exactly:
    //   - immunity to magic sleep effects: a flat, no-magnitude grant-only
    //     identity record, mirroring the Monk Purity of Body / Diamond Body
    //     disease/poison-immunity idiom — no sleep-effect-resolution engine
    //     exists anywhere in this codebase to apply the immunity to;
    //   - a +2 racial saving throw bonus against enchantment spells and
    //     effects: a flat racial-bonus magnitude, mirroring the Keen Senses
    //     flat-bonus idiom (applied to a save category instead of a skill),
    //     not a saving-throw-total engine.
    // The record's numeric value (2) names only the save-bonus magnitude;
    // the sleep immunity is named in the detail text as a non-fabricated
    // grant-only fact, contributing no additional numeric value.
    explanations.push(ComputationExplanation {
        id: "race.half_elf.trait_bundle.elven_immunities".to_owned(),
        value: 2,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — DESC:\"Half-elves are immune to magic sleep effects and get a +2 racial saving throw
        //   bonus against enchantment spells and effects.\", ABILITY:Special
        //   Ability|AUTOMATIC|Immunity to Sleep, BONUS:VAR|SaveBonus_vs_Enchantments|2|TYPE=Racial
        detail: "Half-Elf racial trait bundle — Elven Immunities: PF1 Core Half-Elf is immune to \
                  magic sleep effects and gets a flat +2 racial saving throw bonus against \
                  enchantment spells and effects \
                  (core_essentials/races/half_elf/halfelf_abilities_race.lst Elven Immunities \
                  entry). The sleep immunity is a bounded grant-only identity record \
                  (non-fabricated): no sleep-effect- resolution engine exists anywhere in this \
                  codebase to apply the immunity to. The recognized numeric value (2) names only the \
                  flat enchantment saving-throw-bonus magnitude, not a saving-throw-total engine."
            .to_owned(),
    });

    // Bounded honesty: only the six named dimensions are grounded. This replaces
    // the generic race.semantics.unverified diagnostic for Half-Elf specifically
    // and stays non-claim-blocking so the deterministic pilot still reports
    // computed evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.half_elf.bounded_semantics".to_owned(),
        message: "Half-Elf race semantics are grounded for the deterministic pilot's chosen \
                  ability-bonus target, size, speed, senses, Keen Senses (Perception bonus), \
                  and Elven Immunities (sleep immunity plus enchantment save bonus) trait \
                  bundle; the remaining PF1 Core Half-Elf racial trait surface remains \
                  unverified: Adaptability (a bonus Skill Focus feat in a chosen skill at 1st \
                  level), and Multitalented (counting both parent classes as favored classes)."
            .to_owned(),
        claim_blocking: false,
    });
}

pub(super) const HALF_ORC_RACE_ID: &str = "race:half-orc";

/// SD31-E4-F1-005 (§8, "wire it, don't retract"): all 6 of Wave 8's demoted
/// `race_trait` Small-size units (`kobold_size`, `svirfneblin_size`,
/// `goblin_size`, `grippli_size` here; `gnome_size`/`halfling_size` already
/// corrected by `SD31-E4-F1-004`) had, or now have, a
/// `race.<x>.trait_bundle.size` explanation citing the real mechanism.
/// Kobold, Svirfneblin, Goblin and Grippli have no dedicated
/// `explain_<race>_race_seam` (no ability-score adjustments are transcribed
/// for them anywhere in this engine, so this function deliberately claims
/// only creature size, nothing wider) -- confirmed empirically before
/// writing this: `race:grippli` reaches `compute_pilot_base_chassis` today
/// with ZERO claim-blocking diagnostics and `baseline_armor_class == 18`
/// (17 Medium baseline + the real +1 Small bonus), the same as the other
/// three, via only a non-blocking `race.semantics.unverified` note -- so
/// Grippli belongs in this table exactly like the other three, not left out
/// as a boundary.
///
/// `combat_size_modifiers` (`race_size_for_race_token`) already applies the
/// real PF1 AC/attack/CMB/CMD size term to ANY race string it resolves,
/// unconditionally, inside `compute_combat_baseline` -- proven for Kobold,
/// Svirfneblin and Goblin by `tests/sd27_size_modifiers_to_armor_class.rs`
/// and `tests/sd27_size_modifiers_to_touch_cmb_cmd_and_attack.rs` (`RACES`
/// tables in both), and for Grippli by this module's own
/// `size_only_race_trait_bundle_tests`. What was still missing was
/// record-level provability: each of these four characters' sheet showed
/// the correct total but had no explanation row saying why, one corpus
/// record deep. This closes that gap the same way `SD31-E4-F1-004` closed
/// it for Gnome/Halfling.
///
/// # Sourcing, verified against the pinned oracle directly, not inferred
///
/// Each of the four races' own `<race>_abilities_race.lst`
/// (`core_essentials/races/<race>/`, line 16 in every case) carries an
/// identically-shaped `KEY:<Race> ~ Size` record: `TEMPLATE:SIZE_S`,
/// `CATEGORY:Special Ability`, and a `DESC:` stating the same PF1 Table 8-1
/// Small-size shape verbatim ("+1 size bonus to their AC, a +1 size bonus
/// on attack rolls, a -1 penalty on combat maneuver checks and to their
/// Combat Maneuver Defense, and a +4 size bonus on Stealth checks") --
/// re-derived directly from `data/corpus/beastiary/race_trait/{kobold,
/// svirfneblin,goblin}/*_size.json` and `data/corpus/bestiary_2/race_trait/
/// grippli/grippli_size.json`'s own `description` field, not transcribed
/// from memory.
pub(super) const SIZE_ONLY_RACE_TRAIT_BUNDLE: &[(&str, &str, &str)] = &[
    ("race:kobold", "Kobold", "kobold_abilities_race.lst"),
    ("race:svirfneblin", "Svirfneblin", "svirfneblin_abilities_race.lst"),
    ("race:goblin", "Goblin", "goblin_abilities_race.lst"),
    ("race:grippli", "Grippli", "grippli_abilities_race.lst"),
];

pub(super) fn explain_size_only_race_trait_bundle(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some((_, race_label, source_file)) = SIZE_ONLY_RACE_TRAIT_BUNDLE
        .iter()
        .find(|(race_id, _, _)| *race_id == input.chosen.race_id)
    else {
        return;
    };
    let slug = race_label.to_lowercase();
    explanations.push(ComputationExplanation {
        id: format!("race.{slug}.trait_bundle.size"),
        value: 0,
        detail: format!(
            "{race_label} racial trait bundle — size: PF1 {race_label} is Small size \
             (core_essentials/races/{slug}/{source_file}'s `KEY:{race_label} ~ Size` record, \
             `TEMPLATE:SIZE_S`). This is a bounded recognition record naming the {race_label} \
             size category; it performs no arithmetic of its own (+0) so it does not \
             double-count the real PF1 Small-size effect (+1 AC, +1 attack rolls, -1 CMB/CMD, \
             and +4 Stealth). SD-27 (decisions.md §28 defect 1) wired the AC/attack/CMB/CMD \
             portion into this engine's general combat baseline (`combat_size_modifiers`, keyed \
             off `race_size_for_race_token`, which resolves {race_label} to Small \
             unconditionally): any {race_label} character who reaches \
             `compute_combat_baseline`'s supported posture gets the real +1 AC/attack and \
             -1 CMB/CMD from that shared term, not from this record. The +4 Stealth portion is \
             still not applied anywhere: no Stealth skill total exists in this engine yet \
             (`compute_selected_skill_modifiers` supports only Climb, Intimidate and Swim). \
             {race_label} has no other racial ability-score, speed, or sense adjustment \
             transcribed on this deterministic pilot seam; only its creature size is grounded \
             here"
        ),
    });
}

#[cfg(test)]
mod size_only_race_trait_bundle_tests {
    use super::compute_pilot_base_chassis;
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn input_for_race(slug: &str) -> CharacterInput {
        let text = FIGHTER_LEVEL_1_FIXTURE
            .replace("race_id=race:human", &format!("race_id=race:{slug}"));
        let loaded = load_character_input_fixture(&text);
        assert!(
            loaded.diagnostics.is_empty(),
            "{slug} fixture should load cleanly: {:?}",
            loaded.diagnostics
        );
        loaded.character_input.expect("valid fixture should produce a character input record")
    }

    /// Each of the three races gets its own real `race.<slug>.trait_bundle.size`
    /// explanation record, citing the real mechanism, not a stub or a copy-
    /// pasted placeholder -- and none of the three collide with each other's id.
    #[test]
    fn each_race_gets_its_own_named_size_explanation_citing_the_real_mechanism() {
        for slug in ["kobold", "svirfneblin", "goblin", "grippli"] {
            let computation = compute_pilot_base_chassis(&input_for_race(slug));
            let id = format!("race.{slug}.trait_bundle.size");
            let record = computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("{slug} must produce a {id} explanation"));
            assert_eq!(record.value, 0, "{slug}: a bounded recognition record, no arithmetic performed");
            assert!(
                record.detail.contains("Small"),
                "{slug}: size record must name the Small size category: {}",
                record.detail
            );
            assert!(
                record.detail.contains("combat_size_modifiers"),
                "{slug}: record must cite the real mechanism that applies AC/attack/CMB/CMD, \
                 not just describe the rule in the abstract: {}",
                record.detail
            );
            assert!(
                record.detail.contains("Stealth skill total exists"),
                "{slug}: record must still name the one genuinely unapplied piece (Stealth) \
                 honestly rather than implying full coverage: {}",
                record.detail
            );
        }
    }

    /// The new explanation must not shadow or duplicate `combat_size_modifiers`'s
    /// own real AC number -- the +1 must still land on `defense.baseline_armor_class`
    /// exactly as `tests/sd27_size_modifiers_to_armor_class.rs` already proves,
    /// unaffected by this cycle's own addition.
    #[test]
    fn the_new_explanation_does_not_change_the_real_armor_class_total() {
        for (slug, expected_ac) in
            [("kobold", 18), ("svirfneblin", 18), ("goblin", 18), ("grippli", 18)]
        {
            let computation = compute_pilot_base_chassis(&input_for_race(slug));
            assert_eq!(
                computation.baseline_armor_class, expected_ac,
                "{slug}: Small-size Armor Class (17 Medium baseline + 1) must be unaffected by \
                 the new record-level explanation this cycle adds"
            );
        }
    }

    /// A race outside the three-entry table (Human, the fixture's own default)
    /// gets no `trait_bundle.size` record from this function at all -- it must
    /// not fire generically for every race.
    #[test]
    fn a_race_outside_the_table_gets_no_record_from_this_function() {
        let computation = compute_pilot_base_chassis(&input_for_race("human"));
        assert!(
            !computation.explanations.iter().any(|e| e.id.starts_with("race.")
                && e.id.ends_with(".trait_bundle.size")
                && e.id != "race.human.trait_bundle.size"),
            "no stray size-only trait_bundle record should appear for Human"
        );
    }
}

/// Rougarou (Bestiary 6): `~ Speed` (`MOVE:Walk,30`, a flat, unconditional
/// land-speed statement — no alternate trait in this book's own corpus
/// replaces it), `~ Vision` (binary Low-Light Vision plus the Scent special
/// ability, neither a distance magnitude), and `~ Natural Weapon` (a fixed
/// 1d4 bite, secondary if the character also wields a manufactured weapon).
/// Source: `core_essentials/races/rougarou/rougarou_abilities_race.lst:17-20`
/// (`data/corpus/bestiary_6/race_trait/rougarou/rougarou_{speed,vision,
/// natural_weapon}.json`'s own `source.line` and ingest token array, read directly, not
/// transcribed from the ledger).
pub(super) fn explain_rougarou_flat_override_race_trait(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if input.chosen.race_id != ROUGAROU_RACE_ID {
        return;
    }

    explanations.push(ComputationExplanation {
        id: "race.rougarou.trait_bundle.speed".to_owned(),
        value: ROUGAROU_BASE_SPEED_FEET,
        detail: format!(
            "Rougarou racial trait bundle — speed: PF1 Bestiary 6 Rougarou has a base land \
             speed of {ROUGAROU_BASE_SPEED_FEET} ft (rougarou_abilities_race.lst:17 \
             MOVE:Walk,{ROUGAROU_BASE_SPEED_FEET}). This is a grounded recognition value \
             carrying the Rougarou base-speed identity on the deterministic pilot seam; it \
             contributes no computed speed-derived effect to any chassis output, skill \
             modifier, attack roll, or combat baseline — mirrors the Dwarf/Elf/Gnome/Half-Elf/ \
             Half-Orc/Halfling `trait_bundle.speed` idiom exactly, extended to a race with no \
             CRB seam function"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "race.rougarou.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Rougarou racial trait bundle — senses: PF1 Bestiary 6 Rougarou grants \
                  low-light vision and the Scent special ability (rougarou_abilities_race.lst:18 \
                  VISION:Low-Light Vision, ABILITY:Special Ability|AUTOMATIC|Universal Monster \
                  Rule ~ Scent). Both are binary traits, not a distance magnitude like Dwarf \
                  Darkvision; this is a bounded recognition record naming the Rougarou \
                  low-light-vision-and-scent identity on the deterministic pilot seam. It \
                  contributes no computed illumination-, perception-, or \
                  creature-detection-derived effect to any chassis output, so it carries no \
                  fabricated mechanical value (+0)"
            .to_owned(),
    });

    explanations.push(ComputationExplanation {
        id: "race.rougarou.trait_bundle.natural_weapon".to_owned(),
        value: ROUGAROU_BITE_DAMAGE_DIE,
        detail: format!(
            "Rougarou racial trait bundle — Natural Weapon: PF1 Bestiary 6 Rougarou has a bite \
             attack dealing 1d{ROUGAROU_BITE_DAMAGE_DIE} points of damage \
             (rougarou_abilities_race.lst:20 ABILITY:Internal|AUTOMATIC|Bite, \
             with the bite's damage die stepped down one size), a secondary attack if the \
             character also wields a manufactured weapon. This engine has no natural-attack \
             routine and computes no weapon-damage total anywhere (the same posture \
             `ground_alchemist_feral_mutagen_discovery`'s claw/bite damage-die records already \
             take), so the damage die is grounded as a standalone magnitude rather than folded \
             into a total that does not exist"
        ),
    });
}

/// Gillman (Advanced Race Guide): racial-DEFAULT `~ Speed` states BOTH a
/// {GILLMAN_BASE_SPEED_FEET} ft land speed and a
/// {GILLMAN_SWIM_SPEED_FEET} ft swim speed (`MOVE:Walk,30,Swim,30`,
/// `gillman_abilities_race.lst:17`); the selectable `Gillman ~ Throwback`
/// alternate trait sets `Gillman_ReplaceSpeed=True` and replaces it with
/// `Throwback ~ Gillman ~ Speed` — land speed only, no swim speed at all
/// (`arg_abilities_race.lst:881`, `data/corpus/advanced_race_guide/
/// race_trait/gillman/throwback_gillman_speed.json`). Confirmed genuinely
/// unconditional in both shapes: neither record's `DESC:` carries a
/// conditional cue (no "against", no damage type, no environmental state).
pub(super) fn explain_gillman_flat_override_race_trait(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if input.chosen.race_id != GILLMAN_RACE_ID {
        return;
    }

    if replaced_by_alternate_trait(input, GILLMAN_REPLACE_SPEED_FLAG) {
        let selected_throwback = selected_alternate_trait_keys(input)
            .iter()
            .any(|key| key == GILLMAN_THROWBACK_TRAIT_KEY);
        explanations.push(ComputationExplanation {
            id: "race.gillman.alternate_trait.throwback.speed".to_owned(),
            value: THROWBACK_GILLMAN_SPEED_FEET,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   , gated PREFACT:1,ABILITIES,{GILLMAN_REPLACE_SPEED_FLAG}=True, which `Gillman ~
                //   Throwback` sets
                "Gillman alternate racial trait — Throwback (Advanced Race Guide p.188): the chosen \
                 `Gillman ~ Throwback` alternate replaces the standard Gillman `~ Speed` record \
                 (arg_abilities_race.lst:881 `Throwback ~ Gillman ~ Speed`, \
                 MOVE:Walk,{THROWBACK_GILLMAN_SPEED_FEET}). Throwback gillmen have \
                 {THROWBACK_GILLMAN_SPEED_FEET} ft land speed only — no swim speed, unlike the \
                 standard racial default. The standard \
                 {GILLMAN_BASE_SPEED_FEET}/{GILLMAN_SWIM_SPEED_FEET} ft record is therefore NOT \
                 emitted for this character (selected_throwback={selected_throwback}, confirmed via \
                 the same flag `replaced_by_alternate_trait` checks). This is a grounded recognition \
                 value; it contributes no computed speed-derived effect to any chassis output"
            ),
        });
        return;
    }

    explanations.push(ComputationExplanation {
        id: "race.gillman.trait_bundle.speed".to_owned(),
        value: GILLMAN_BASE_SPEED_FEET,
        detail: format!(
            "Gillman racial trait bundle — speed: PF1 Advanced Race Guide Gillman has a base \
             land speed of {GILLMAN_BASE_SPEED_FEET} ft and a swim speed of \
             {GILLMAN_SWIM_SPEED_FEET} ft, can move in water without attempting Swim checks, \
             and always treats Swim as a class skill (gillman_abilities_race.lst:17 \
             MOVE:Walk,{GILLMAN_BASE_SPEED_FEET},Swim,{GILLMAN_SWIM_SPEED_FEET}). This is a \
             grounded recognition value carrying the Gillman base-speed identity (both land and \
             swim) on the deterministic pilot seam; it contributes no computed speed-derived \
             effect to any chassis output, skill modifier, attack roll, or combat baseline — no \
             swim-speed-consuming engine exists in this codebase either, so the Swim-as-class-\
             skill and no-check-required clauses are named but not applied"
        ),
    });
}

/// Vanara (Advanced Race Guide): racial-DEFAULT `~ Speed` states BOTH a
/// {VANARA_BASE_SPEED_FEET} ft land speed and a {VANARA_CLIMB_SPEED_FEET} ft
/// Climb speed (`MOVE:Walk,30,Climb,20`, `vanara_abilities_race.lst:17`);
/// the selectable `Vanara ~ Tree Stranger` alternate trait sets
/// `Vanara_ReplaceSpeed=True` and replaces it with `Tree Stranger ~ Vanara ~
/// Speed` — land speed only, no Climb speed at all (`arg_abilities_race.
/// lst:1241`, `data/corpus/advanced_race_guide/race_trait/vanara/
/// tree_stranger_vanara_speed.json`).
pub(super) fn explain_vanara_flat_override_race_trait(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if input.chosen.race_id != VANARA_RACE_ID {
        return;
    }

    if replaced_by_alternate_trait(input, VANARA_REPLACE_SPEED_FLAG) {
        let selected_tree_stranger = selected_alternate_trait_keys(input)
            .iter()
            .any(|key| key == VANARA_TREE_STRANGER_TRAIT_KEY);
        explanations.push(ComputationExplanation {
            id: "race.vanara.alternate_trait.tree_stranger.speed".to_owned(),
            value: TREE_STRANGER_VANARA_SPEED_FEET,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   , gated PREFACT:1,ABILITIES,{VANARA_REPLACE_SPEED_FLAG}=True, which `Vanara ~
                //   Tree Stranger` sets
                "Vanara alternate racial trait — Tree Stranger (Advanced Race Guide p.206): the \
                 chosen `Vanara ~ Tree Stranger` alternate replaces the standard Vanara `~ Speed` \
                 record (arg_abilities_race.lst:1241 `Tree Stranger ~ Vanara ~ Speed`, \
                 MOVE:Walk,{TREE_STRANGER_VANARA_SPEED_FEET}). Tree stranger vanaras have \
                 {TREE_STRANGER_VANARA_SPEED_FEET} ft land speed only — no Climb speed, unlike the \
                 standard racial default. The standard \
                 {VANARA_BASE_SPEED_FEET}/{VANARA_CLIMB_SPEED_FEET} ft record is therefore NOT \
                 emitted for this character (selected_tree_stranger={selected_tree_stranger}, \
                 confirmed via the same flag `replaced_by_alternate_trait` checks). This is a \
                 grounded recognition value; it contributes no computed speed-derived effect to any \
                 chassis output"
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "race.vanara.trait_bundle.speed".to_owned(),
            value: VANARA_BASE_SPEED_FEET,
            detail: format!(
                "Vanara racial trait bundle — speed: PF1 Advanced Race Guide Vanara has a base \
                 land speed of {VANARA_BASE_SPEED_FEET} ft and a Climb speed of \
                 {VANARA_CLIMB_SPEED_FEET} ft (vanara_abilities_race.lst:17 \
                 MOVE:Walk,{VANARA_BASE_SPEED_FEET},Climb,{VANARA_CLIMB_SPEED_FEET}). This is a \
                 grounded recognition value carrying the Vanara base-speed identity (both land \
                 and climb) on the deterministic pilot seam; it contributes no computed \
                 speed-derived effect to any chassis output, skill modifier, attack roll, or \
                 combat baseline — no Climb-check engine exists in this codebase either"
            ),
        });
    }

    explanations.push(ComputationExplanation {
        id: "race.vanara.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Vanara racial trait bundle — senses: PF1 Advanced Race Guide Vanara grants \
                  low-light vision (vanara_abilities_race.lst:18 VISION:Low-Light Vision). This \
                  is a bounded recognition record naming the Vanara low-light vision identity on \
                  the deterministic pilot seam, mirroring the already-grounded Elf low-light \
                  vision idiom exactly; it contributes no computed illumination or \
                  perception-derived effect to any chassis output, so it carries no fabricated \
                  mechanical value (+0)"
            .to_owned(),
    });
}

pub(super) fn explain_samsaran_flat_override_race_trait(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if input.chosen.race_id != SAMSARAN_RACE_ID {
        return;
    }

    explanations.push(ComputationExplanation {
        id: "race.samsaran.trait_bundle.speed".to_owned(),
        value: SAMSARAN_BASE_SPEED_FEET,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|MOVEBASE|{SAMSARAN_BASE_SPEED_FEET}
            "Samsaran racial trait bundle — speed: PF1 Advanced Race Guide Samsaran has a base land \
             speed of {SAMSARAN_BASE_SPEED_FEET} ft (samsaran_abilities_race.lst:17). This is a \
             grounded recognition value carrying the Samsaran base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline — no speed-consuming \
             engine exists in this codebase, mirroring the Rougarou/Gillman/Vanara \
             `trait_bundle.speed` idiom exactly"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "race.samsaran.trait_bundle.senses".to_owned(),
        value: 0,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   BONUS:VAR|HasRacialVision|1
        detail: "Samsaran racial trait bundle — senses: PF1 Advanced Race Guide Samsaran can see \
                  twice as far as humans in conditions of dim light \
                  (samsaran_abilities_race.lst:18), a binary low-light-vision trait, not a distance \
                  magnitude. This is a bounded recognition record naming the Samsaran \
                  low-light-vision identity on the deterministic pilot seam; it contributes no \
                  computed illumination or perception-derived effect to any chassis output, so it \
                  carries no fabricated mechanical value (+0)"
            .to_owned(),
    });

    explanations.push(ComputationExplanation {
        id: "race.samsaran.trait_bundle.lifebound".to_owned(),
        value: SAMSARAN_LIFEBOUND_SAVE_BONUS,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|SaveBonus_vs_DeathEffects,SaveBonus_vs_NegativeEnergy,FortSave_vs_NegativeLevels|{SAMSARAN_LIFEBOUND_SAVE_BONUS}|TYPE=Racial
            "Samsaran racial trait bundle — Lifebound: PF1 Advanced Race Guide Samsaran gains a \
             {SAMSARAN_LIFEBOUND_SAVE_BONUS:+} racial bonus on saving throws against death effects, \
             saving throws against negative energy effects, Fortitude saves to remove negative \
             levels, and Constitution checks to stabilize (samsaran_abilities_race.lst:19). \
             CONDITIONAL under Decision 7 REFINED (a named effect-type subset, not a universal sheet \
             modifier) — no save-bonus-vs-condition-type consuming total exists anywhere in this \
             codebase (`BaseSaves`/`total_saves` carry no per-condition breakdown), so this is \
             grounded as a standalone recognition value rather than folded into a total that does \
             not exist"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "race.samsaran.trait_bundle.shards_of_the_past".to_owned(),
        value: SAMSARAN_SHARDS_OF_THE_PAST_SKILL_BONUS,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:SKILL|LIST|{SAMSARAN_SHARDS_OF_THE_PAST_SKILL_BONUS}|TYPE=Racial
            //   The `LIST` target is a player-chosen pair of skills this engine models no chooser
            //   for — the same `%LIST`/`LIST` shape `ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES`'s own
            //   doc comment already declines to guess at — and `SelectedSkillModifiers` totals only
            //   Climb, Intimidate, and Swim, none of which this trait can be proven to target
            //   without a chooser.
            "Samsaran racial trait bundle — Shards of the Past: PF1 Advanced Race Guide Samsaran \
             chooses two skills and gains a {SAMSARAN_SHARDS_OF_THE_PAST_SKILL_BONUS:+} racial bonus \
             on both, always as class skills (samsaran_abilities_race.lst:21). Grounded as a \
             standalone recognition value, not folded into a skill total this engine cannot resolve"
        ),
    });
}

pub(super) fn explain_nagaji_flat_override_race_trait(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if input.chosen.race_id != NAGAJI_RACE_ID {
        return;
    }

    explanations.push(ComputationExplanation {
        id: "race.nagaji.trait_bundle.speed".to_owned(),
        value: NAGAJI_BASE_SPEED_FEET,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|MOVEBASE|{NAGAJI_BASE_SPEED_FEET}
            "Nagaji racial trait bundle — speed: PF1 Advanced Race Guide Nagaji has a base land \
             speed of {NAGAJI_BASE_SPEED_FEET} ft (nagaji_abilities_race.lst:17). This is a grounded \
             recognition value carrying the Nagaji base-speed identity on the deterministic pilot \
             seam; it contributes no computed speed-derived effect to any chassis output, skill \
             modifier, attack roll, or combat baseline — no speed-consuming engine exists in this \
             codebase"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "race.nagaji.trait_bundle.senses".to_owned(),
        value: 0,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   BONUS:VAR|HasRacialVision|1
        detail: "Nagaji racial trait bundle — senses: PF1 Advanced Race Guide Nagaji can see twice \
                  as far as humans in conditions of dim light (nagaji_abilities_race.lst:18), a \
                  binary low-light-vision trait, not a distance magnitude. This is a bounded \
                  recognition record naming the Nagaji low-light-vision identity on the \
                  deterministic pilot seam; it contributes no computed illumination or \
                  perception-derived effect to any chassis output, so it carries no fabricated \
                  mechanical value (+0)"
            .to_owned(),
    });

    explanations.push(ComputationExplanation {
        id: "race.nagaji.trait_bundle.armored_scales".to_owned(),
        value: NAGAJI_ARMORED_SCALES_NATURAL_ARMOR,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|AC_Natural_Armor|{NAGAJI_ARMORED_SCALES_NATURAL_ARMOR}|TYPE=Base
            "Nagaji racial trait bundle — Armored Scales: PF1 Advanced Race Guide Nagaji has a \
             {NAGAJI_ARMORED_SCALES_NATURAL_ARMOR:+} natural armor bonus from scaly flesh \
             (nagaji_abilities_race.lst:19). A UNIVERSAL, unconditional AC modifier under Decision 7 \
             REFINED. A real natural-armor consuming total DOES exist in this codebase \
             (`FeatDerivedPillarContributions::natural_armor_bonus`, already summed into both the \
             armor-class total and the touch-AC exclusion, and already accepting contributions from \
             three other sources — Alchemist Mutagen, Sorcerer Draconic Bloodline, ARG Armor of the \
             Pit) — but this race trait is not wired into it; that is a genuine, separate follow-on \
             (logged to OPEN-ISSUES.md), not a missing consumer. Grounded here as a standalone \
             recognition value, not folded into an AC total this specific trait does not yet feed"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "race.nagaji.trait_bundle.resistant".to_owned(),
        value: NAGAJI_RESISTANT_SAVE_BONUS,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|SaveBonus_vs_MindAffecting,SaveBonus_vs_Poison|{NAGAJI_RESISTANT_SAVE_BONUS}|TYPE=Racial
            "Nagaji racial trait bundle — Resistant: PF1 Advanced Race Guide Nagaji gains a \
             {NAGAJI_RESISTANT_SAVE_BONUS:+} racial saving throw bonus against mind-affecting \
             effects and poison (nagaji_abilities_race.lst:20). CONDITIONAL under Decision 7 REFINED \
             (a named effect-type subset), and no save-bonus-vs-condition-type consuming total \
             exists anywhere in this codebase. Grounded as a standalone recognition value"
        ),
    });

    // `~ Hypnotic Gaze` REPLACES `~ Serpent's Sense` (both are gated on the
    // same `Nagaji_ReplaceSerpentsSense` flag `race_resolver.rs`'s
    // `ALTERNATE_TRAIT_REPLACE_FLAGS` table registers for the `"Nagaji ~
    // Hypnotic Gaze"` selection key) — the same mutually-exclusive shape
    // Gillman's Throwback and Vanara's Tree Stranger use for Speed above.
    // Fixed during the wave-27 integration cycle: the original code emitted
    // both records unconditionally for every nagaji.
    if replaced_by_alternate_trait(input, NAGAJI_REPLACE_SERPENTS_SENSE_FLAG) {
        let total_character_level: i16 =
            input.chosen.class_levels.iter().map(|c| i16::from(c.level)).sum();
        let hypnotic_gaze_dc = nagaji_hypnotic_gaze_dc(ability_modifiers.charisma);
        explanations.push(ComputationExplanation {
            id: "race.nagaji.alternate_trait.hypnotic_gaze.dc".to_owned(),
            value: hypnotic_gaze_dc,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   , gated PREFACT:1,ABILITIES,{NAGAJI_REPLACE_SERPENTS_SENSE_FLAG}=True, which
                //   `Nagaji ~ Hypnotic Gaze` sets
                //   DESC:\"...The DC of this effect is equal to 11 + the nagaji's Charisma
                //   modifier...\"
                "Nagaji alternate racial trait — Hypnotic Gaze (Advanced Race Guide p.??): the \
                 chosen `Nagaji ~ Hypnotic Gaze` alternate replaces the standard Nagaji `~ Serpent's \
                 Sense` record (arg_abilities_race.lst:985). Its spell-like ability (as hypnotism) \
                 has a DC equal to {NAGAJI_HYPNOTIC_GAZE_DC_BASE} + the nagaji's Charisma modifier. \
                 This character's Charisma modifier is {cha_mod:+}, so DC = \
                 {NAGAJI_HYPNOTIC_GAZE_DC_BASE} + ({cha_mod:+}) = {hypnotic_gaze_dc}. A REAL \
                 computed value, not a recognition record — mirrors the \
                 `alchemist_extract_save_dc`/`witch_hex_save_dc` DC-function idiom this file already \
                 uses for class features. The standard Serpent's Sense record is therefore NOT \
                 emitted for this character",
                cha_mod = ability_modifiers.charisma,
            ),
        });

        explanations.push(ComputationExplanation {
            id: "race.nagaji.alternate_trait.hypnotic_gaze.caster_level".to_owned(),
            value: total_character_level,
            detail: format!(
                "Nagaji alternate racial trait — Hypnotic Gaze caster level: has \"caster level \
                 equal to the nagaji's Hit Dice\" (arg_abilities_race.lst:985). A player \
                 character's Hit Dice total equals character level (PF1 core rule; a nagaji PC \
                 has no separate racial Hit Dice progression), so this is modelled as the \
                 character's total class level ({total_character_level}, summed across \
                 `input.chosen.class_levels`). A REAL computed value, not a recognition record"
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "race.nagaji.trait_bundle.serpents_sense".to_owned(),
            value: NAGAJI_SERPENTS_SENSE_SKILL_BONUS,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:SITUATION|Handle Animal=against
                //   reptiles|{NAGAJI_SERPENTS_SENSE_SKILL_BONUS},
                //   BONUS:SKILL|Perception|{NAGAJI_SERPENTS_SENSE_SKILL_BONUS}|TYPE=Racial
                "Nagaji racial trait bundle — Serpent's Sense: PF1 Advanced Race Guide Nagaji gains \
                 a {NAGAJI_SERPENTS_SENSE_SKILL_BONUS:+} racial bonus on Handle Animal checks \
                 against reptiles and a {NAGAJI_SERPENTS_SENSE_SKILL_BONUS:+} racial bonus on \
                 Perception checks (nagaji_abilities_race.lst:21). Handle Animal and Perception are \
                 both named in `ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES`'s own doc comment as skills \
                 this codebase computes no total for; `SelectedSkillModifiers` totals only Climb, \
                 Intimidate, and Swim. Grounded as a standalone recognition value, not folded into a \
                 skill total this engine cannot resolve. This is the default record; NOT emitted if \
                 the character has selected the `Nagaji ~ Hypnotic Gaze` alternate trait, which \
                 replaces it (see above)"
            ),
        });
    }
}

/// Undine (Advanced Race Guide): three selectable alternate racial traits, each replacing the
/// racial-default Spell-Like Ability trait, each stating a real PCGen arithmetic formula over
/// total character level and one ability modifier. Ruling §18 (option pools show ONLY VALID
/// CHOICES): these three are mutually exclusive with each other and with the default — this
/// function computes a magnitude ONLY for the alternate the player actually selected
/// (`selected_alternate_trait_keys`, the same `RACE_ALTERNATE_TRAIT_CHOICE_ID` mechanism
/// Gillman's Throwback and Vanara's Tree Stranger already use above); an unselected alternate
/// produces no record at all, never a zero or a placeholder.
pub(super) fn explain_undine_formula_race_trait(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if input.chosen.race_id != UNDINE_RACE_ID {
        return;
    }
    use crate::rules_core::sheet_rule::{evaluate_expr_from_facts, Ability, CharacterFacts};

    let total_level: i64 = input.chosen.class_levels.iter().map(|c| i64::from(c.level)).sum();
    let mut facts = CharacterFacts { level: total_level, ..CharacterFacts::default() };
    facts.ability_mods[ability_index_of(Ability::Con)] = i64::from(ability_modifiers.constitution);
    facts.ability_mods[ability_index_of(Ability::Cha)] = i64::from(ability_modifiers.charisma);

    let selected = selected_alternate_trait_keys(input);

    // SD-35 `AT-35-E6-001` (`decisions.md` §11): each field's arithmetic is a CONVERTED
    // `sheet_rule::Expr`, evaluated by the sheet renderer's own evaluator. The source's own
    // formula TEXT is no longer read here at all -- it lives on the converter/oracle side
    // ([`UNDINE_RACE_TRAIT_FORMULAS`] above), which is
    // what `derived_evaluator_fixture_check`'s race_trait_formula bar still compares against.
    let eval = |field: &str| -> Option<i16> {
        i16::try_from(evaluate_expr_from_facts(&undine_expr(field), &facts).trunc()).ok()
    };

    if selected.iter().any(|k| k == UNDINE_ACID_BREATH_TRAIT_KEY)
        && let (Some(times), Some(dice), Some(dc)) =
            (eval("Undine_AcidBreath_Times"), eval("Undine_AcidBreath_Dice"), eval("Undine_AcidBreath_DC"))
        {
            explanations.push(ComputationExplanation {
                id: "race.undine.alternate_trait.acid_breath".to_owned(),
                value: dice,
                detail: format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   arg_abilities_race.lst:776
                    //   BONUS:VAR|Undine_AcidBreath_Dice|min(floor((TL+1)/2),5),
                    //   BONUS:VAR|Undine_AcidBreath_DC|10+(TL/2)+CON,
                    "Undine alternate racial trait — Acid Breath (Advanced Race Guide p.174): a \
                     {times}/day 5-ft cone breath weapon dealing {dice}d8 acid damage, Reflex DC \
                     {dc} for half (evaluated at total character level {total_level} and \
                     Constitution modifier {con:+} by the sheet evaluator over the converted \
                     arithmetic, gated by `derived_evaluator_fixture_check`'s race_trait_formula \
                     bar)",
                    con = ability_modifiers.constitution,
                ),
            });
        }

    if selected.iter().any(|k| k == UNDINE_NEREID_FASCINATION_TRAIT_KEY)
        && let (Some(times), Some(duration), Some(dc)) = (
            eval("Undine_NereidFascination_Times"),
            eval("Undine_NereidFascination_Duration"),
            eval("Undine_NereidFascination_DC"),
        ) {
            explanations.push(ComputationExplanation {
                id: "race.undine.alternate_trait.nereid_fascination".to_owned(),
                value: duration,
                detail: format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   arg_abilities_race.lst:781
                    //   BONUS:VAR|Undine_NereidFascination_Duration|max((TL/2),1),
                    //   BONUS:VAR|Undine_NereidFascination_DC|10+(TL/2)+CHA,
                    "Undine alternate racial trait — Nereid Fascination (Advanced Race Guide p.175): \
                     {times}/day as a standard action, a 20-ft-radius aura fascinates humanoids \
                     within it for {duration} rounds, Will DC {dc} negates (evaluated at total \
                     character level {total_level} and Charisma modifier {cha:+} by the sheet \
                     evaluator over the converted arithmetic, gated by \
                     `derived_evaluator_fixture_check`'s race_trait_formula bar)",
                    cha = ability_modifiers.charisma,
                ),
            });
        }

    if selected.iter().any(|k| k == UNDINE_OOZE_BREATH_TRAIT_KEY)
        && let (Some(times), Some(dice), Some(dc)) =
            (eval("Undine_OozeBreath_Times"), eval("Undine_OozeBreath_Dice"), eval("Undine_OozeBreath_DC"))
        {
            explanations.push(ComputationExplanation {
                id: "race.undine.alternate_trait.ooze_breath".to_owned(),
                value: dice,
                detail: format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   arg_abilities_race.lst:782
                    //   BONUS:VAR|Undine_OozeBreath_Dice|min(floor((TL+1/2)),5),
                    //   BONUS:VAR|Undine_OozeBreath_DC|10+(TL/2)+CON,
                    "Undine alternate racial trait — Ooze Breath (Advanced Race Guide p.175): a \
                     {times}/day 5-ft cone breath weapon dealing {dice}d4 acid damage and sickening \
                     for 3 rounds, Reflex DC {dc} halves and negates sickened (evaluated at total \
                     character level {total_level} and Constitution modifier {con:+} by the sheet \
                     evaluator over the converted arithmetic, gated by \
                     `derived_evaluator_fixture_check`'s race_trait_formula bar). This record's \
                     `Dice` formula really is `TL+1/2`, not `(TL+1)/2` — real upstream PCGen \
                     arithmetic, transcribed faithfully rather than \"corrected\" to match Acid \
                     Breath's shape",
                    con = ability_modifiers.constitution,
                ),
            });
        }
}

#[cfg(test)]
mod formula_race_trait_tests {
    use super::compute_pilot_base_chassis;
    use super::{race_alternate_trait_selection_id, RACE_ALTERNATE_TRAIT_CHOICE_ID};
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// `constitution:14` (modifier +2) and `charisma:8` (modifier -1) come straight from the
    /// shared fixture; only race, level, and the selected alternate trait are overridden.
    fn undine_input(level: u8, alternates: &[&str]) -> CharacterInput {
        let text = FIGHTER_LEVEL_1_FIXTURE
            .replace("race_id=race:human", "race_id=race:undine")
            .replace("class_level=class:fighter:1", &format!("class_level=class:fighter:{level}"));
        let loaded = load_character_input_fixture(&text);
        assert!(loaded.diagnostics.is_empty(), "undine fixture should load cleanly: {:?}", loaded.diagnostics);
        let mut input =
            loaded.character_input.expect("valid fixture should produce a character input record");
        for key in alternates {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: RACE_ALTERNATE_TRAIT_CHOICE_ID.to_owned(),
                selection_id: race_alternate_trait_selection_id(key),
            });
        }
        input
    }

    /// Level 5, Constitution modifier +2 (score 14): Dice = min(floor(6/2),5) = 3, DC =
    /// trunc(10 + 2.5 + 2) = 14.
    #[test]
    fn undine_acid_breath_computes_the_real_formula_at_level_5() {
        let computation = compute_pilot_base_chassis(&undine_input(5, &["Undine ~ Acid Breath"]));
        let record = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.undine.alternate_trait.acid_breath")
            .expect("undine with Acid Breath selected must produce an explanation");
        assert_eq!(record.value, 3, "Dice = min(floor((5+1)/2),5) = 3");
        assert!(record.detail.contains("DC 14"), "DC = trunc(10 + 5/2 + 2) = 14: {}", record.detail);
        assert!(record.detail.contains("3d8"));
    }

    /// Level 5, Charisma modifier -1 (score 8): Duration = trunc(max(2.5,1)) = 2, DC =
    /// trunc(10 + 2.5 - 1) = 11.
    #[test]
    fn undine_nereid_fascination_computes_the_real_formula_at_level_5() {
        let computation =
            compute_pilot_base_chassis(&undine_input(5, &["Undine ~ Nereid Fascination"]));
        let record = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.undine.alternate_trait.nereid_fascination")
            .expect("undine with Nereid Fascination selected must produce an explanation");
        assert_eq!(record.value, 2, "Duration = trunc(max(5/2,1)) = 2");
        assert!(record.detail.contains("DC 11"), "DC = trunc(10 + 5/2 - 1) = 11: {}", record.detail);
    }

    /// Level 5, Constitution modifier +2: Dice = min(floor(5+0.5),5) = 5 (the `TL+1/2`, not
    /// `(TL+1)/2`, shape — one dice higher than Acid Breath's at this exact level, confirming
    /// the two formulas are not accidentally identical in this seam).
    #[test]
    fn undine_ooze_breath_computes_its_own_distinct_formula_at_level_5() {
        let computation = compute_pilot_base_chassis(&undine_input(5, &["Undine ~ Ooze Breath"]));
        let record = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.undine.alternate_trait.ooze_breath")
            .expect("undine with Ooze Breath selected must produce an explanation");
        assert_eq!(record.value, 5, "Dice = min(floor(5+1/2),5) = 5");
        assert!(record.detail.contains("DC 14"));
        assert!(record.detail.contains("5d4"));
    }

    /// Selecting none of the three alternates (the racial-default Spell-Like Ability trait
    /// applies instead) must produce NONE of the three records — never a zero, never a
    /// placeholder (Ruling §18).
    #[test]
    fn no_alternate_selected_produces_no_formula_record() {
        let computation = compute_pilot_base_chassis(&undine_input(5, &[]));
        assert!(
            !computation.explanations.iter().any(|e| e.id.starts_with("race.undine.alternate_trait.")),
            "no formula record should appear when no Undine alternate trait is selected"
        );
    }

    /// A race outside this seam (Human, the fixture's own default) gets no stray record from it.
    #[test]
    fn a_race_outside_undine_gets_no_record_from_this_seam() {
        let loaded = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let input = loaded.character_input.expect("fixture should load");
        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation.explanations.iter().any(|e| e.id.starts_with("race.undine.")),
            "no stray Undine record should appear for Human"
        );
    }
}

#[cfg(test)]
mod flat_override_race_trait_tests {
    use super::compute_pilot_base_chassis;
    use super::{
        race_alternate_trait_selection_id, RACE_ALTERNATE_TRAIT_CHOICE_ID,
        THROWBACK_GILLMAN_SPEED_FEET, TREE_STRANGER_VANARA_SPEED_FEET,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn input_for_race(slug: &str, alternates: &[&str]) -> CharacterInput {
        let text = FIGHTER_LEVEL_1_FIXTURE
            .replace("race_id=race:human", &format!("race_id=race:{slug}"));
        let loaded = load_character_input_fixture(&text);
        assert!(
            loaded.diagnostics.is_empty(),
            "{slug} fixture should load cleanly: {:?}",
            loaded.diagnostics
        );
        let mut input =
            loaded.character_input.expect("valid fixture should produce a character input record");
        for key in alternates {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: RACE_ALTERNATE_TRAIT_CHOICE_ID.to_owned(),
                selection_id: race_alternate_trait_selection_id(key),
            });
        }
        input
    }

    /// Rougarou gets all three real, non-zero-value explanation records
    /// (speed, senses, natural weapon), each citing the real mechanism.
    #[test]
    fn rougarou_gets_speed_senses_and_natural_weapon_explanations() {
        let computation = compute_pilot_base_chassis(&input_for_race("rougarou", &[]));
        let speed = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.rougarou.trait_bundle.speed")
            .expect("rougarou must produce a speed explanation");
        assert_eq!(speed.value, 30);
        assert!(speed.detail.contains("MOVE:Walk,30"));

        let senses = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.rougarou.trait_bundle.senses")
            .expect("rougarou must produce a senses explanation");
        assert!(senses.detail.contains("Scent"));

        let natural_weapon = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.rougarou.trait_bundle.natural_weapon")
            .expect("rougarou must produce a natural weapon explanation");
        assert_eq!(natural_weapon.value, 4);
        assert!(natural_weapon.detail.contains("1d4"));
    }

    /// Gillman's racial default speed states BOTH land and swim; the value
    /// reported is the land speed and the detail names the swim speed too.
    #[test]
    fn gillman_default_speed_names_both_land_and_swim() {
        let computation = compute_pilot_base_chassis(&input_for_race("gillman", &[]));
        let speed = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.gillman.trait_bundle.speed")
            .expect("gillman must produce a default speed explanation");
        assert_eq!(speed.value, 30);
        assert!(speed.detail.contains("swim speed of 30"));
    }

    /// Selecting `Gillman ~ Throwback` REPLACES the default speed record
    /// with the Throwback-specific one -- a real override, not an addition:
    /// the default's own id must be absent once Throwback is selected.
    #[test]
    fn gillman_throwback_alternate_replaces_the_default_speed_record() {
        let computation =
            compute_pilot_base_chassis(&input_for_race("gillman", &["Gillman ~ Throwback"]));
        assert!(
            !computation.explanations.iter().any(|e| e.id == "race.gillman.trait_bundle.speed"),
            "the default speed record must not also appear once Throwback is selected"
        );
        let throwback = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.gillman.alternate_trait.throwback.speed")
            .expect("gillman must produce the Throwback override explanation");
        assert_eq!(throwback.value, THROWBACK_GILLMAN_SPEED_FEET);
        assert!(
            !throwback.detail.contains("swim speed of 30"),
            "Throwback gillmen have no swim speed; the override text must not claim one"
        );
    }

    /// Vanara's racial default speed states BOTH land and climb; senses is a
    /// separate zero-value low-light-vision recognition record.
    #[test]
    fn vanara_default_speed_and_senses_are_both_grounded() {
        let computation = compute_pilot_base_chassis(&input_for_race("vanara", &[]));
        let speed = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.vanara.trait_bundle.speed")
            .expect("vanara must produce a default speed explanation");
        assert_eq!(speed.value, 30);
        assert!(speed.detail.contains("Climb speed of 20"));

        let senses = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.vanara.trait_bundle.senses")
            .expect("vanara must produce a senses explanation");
        assert_eq!(senses.value, 0);
        assert!(senses.detail.contains("low-light vision"));
    }

    /// Selecting `Vanara ~ Tree Stranger` replaces the default speed record
    /// exactly as Gillman's Throwback does.
    #[test]
    fn vanara_tree_stranger_alternate_replaces_the_default_speed_record() {
        let computation =
            compute_pilot_base_chassis(&input_for_race("vanara", &["Vanara ~ Tree Stranger"]));
        assert!(
            !computation.explanations.iter().any(|e| e.id == "race.vanara.trait_bundle.speed"),
            "the default speed record must not also appear once Tree Stranger is selected"
        );
        let tree_stranger = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.vanara.alternate_trait.tree_stranger.speed")
            .expect("vanara must produce the Tree Stranger override explanation");
        assert_eq!(tree_stranger.value, TREE_STRANGER_VANARA_SPEED_FEET);
        assert!(
            !tree_stranger.detail.contains("Climb speed of 20"),
            "Tree stranger vanaras have no Climb speed; the override text must not claim one"
        );
        // Senses is untouched by the speed-only alternate.
        assert!(
            computation.explanations.iter().any(|e| e.id == "race.vanara.trait_bundle.senses"),
            "senses must still be emitted independently of the speed alternate"
        );
    }

    /// A race outside this bundle (Human, the fixture's own default) gets no
    /// stray record from any of the five functions.
    #[test]
    fn a_race_outside_the_bundle_gets_no_record_from_this_seam() {
        let computation = compute_pilot_base_chassis(&input_for_race("human", &[]));
        assert!(
            !computation.explanations.iter().any(|e| e.id.starts_with("race.rougarou.")
                || e.id.starts_with("race.gillman.")
                || e.id.starts_with("race.vanara.")
                || e.id.starts_with("race.samsaran.")
                || e.id.starts_with("race.nagaji.")),
            "no stray flat-override record should appear for Human"
        );
    }

    /// Samsaran gets all four real explanation records (speed, senses,
    /// Lifebound, Shards of the Past), each citing the real mechanism.
    #[test]
    fn samsaran_gets_speed_senses_lifebound_and_shards_of_the_past_explanations() {
        let computation = compute_pilot_base_chassis(&input_for_race("samsaran", &[]));

        let speed = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.samsaran.trait_bundle.speed")
            .expect("samsaran must produce a speed explanation");
        assert_eq!(speed.value, 30);
        // Retargeted by SD-35 AT-35-E6-003-SWEEP cycle 4. This used to assert
        // the sheet line quoted the corpus token (`MOVEBASE`). A sheet line is
        // a final number or the rule's words, never ingest vocabulary
        // (`decisions.md` §1), so the token now lives in the `//` provenance
        // comment beside the record and the line states the speed itself.
        // Same shape as the `senses`/`lifebound` assertions just below.
        assert!(speed.detail.contains("base land speed of 30 ft"));
        assert!(
            !speed.detail.contains("MOVEBASE"),
            "no ingest token may reach the rendered sheet line: {}",
            speed.detail
        );

        let senses = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.samsaran.trait_bundle.senses")
            .expect("samsaran must produce a senses explanation");
        assert_eq!(senses.value, 0);
        assert!(senses.detail.contains("dim light"));

        let lifebound = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.samsaran.trait_bundle.lifebound")
            .expect("samsaran must produce a Lifebound explanation");
        assert_eq!(lifebound.value, 2);
        assert!(lifebound.detail.contains("death effects"));
        assert!(lifebound.detail.contains("CONDITIONAL"));

        let shards = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.samsaran.trait_bundle.shards_of_the_past")
            .expect("samsaran must produce a Shards of the Past explanation");
        assert_eq!(shards.value, 2);
        // Retargeted by SD-35 AT-35-E6-003-SWEEP cycle 4, same reason as the
        // `speed` assertion above: the sheet line states the rule in words
        // ("chooses two skills"), and the `BONUS:SKILL|LIST|…` token it used to
        // quote now sits in the `//` provenance comment on the record itself.
        assert!(shards.detail.contains("chooses two skills"));
        assert!(
            !shards.detail.contains("LIST"),
            "no ingest token may reach the rendered sheet line: {}",
            shards.detail
        );
    }

    /// Nagaji with NO alternate trait selected gets the five default
    /// recognition records (speed, senses, Armored Scales, Resistant,
    /// Serpent's Sense) and NO Hypnotic Gaze record — Hypnotic Gaze is an
    /// alternate trait that REPLACES Serpent's Sense, not a default one.
    /// Fixed during the wave-27 integration cycle: the original test built
    /// exactly this input (no alternates) and asserted Hypnotic Gaze was
    /// present anyway, which pinned the bug rather than catching it.
    #[test]
    fn nagaji_default_gets_five_recognition_records_and_no_hypnotic_gaze() {
        let computation = compute_pilot_base_chassis(&input_for_race("nagaji", &[]));

        let speed = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.nagaji.trait_bundle.speed")
            .expect("nagaji must produce a speed explanation");
        assert_eq!(speed.value, 30);

        let senses = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.nagaji.trait_bundle.senses")
            .expect("nagaji must produce a senses explanation");
        assert_eq!(senses.value, 0);

        let armored_scales = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.nagaji.trait_bundle.armored_scales")
            .expect("nagaji must produce an Armored Scales explanation");
        assert_eq!(armored_scales.value, 1);
        assert!(armored_scales.detail.contains("natural armor"));

        let resistant = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.nagaji.trait_bundle.resistant")
            .expect("nagaji must produce a Resistant explanation");
        assert_eq!(resistant.value, 2);
        assert!(resistant.detail.contains("mind-affecting"));

        let serpents_sense = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.nagaji.trait_bundle.serpents_sense")
            .expect("nagaji must produce a Serpent's Sense explanation by default");
        assert_eq!(serpents_sense.value, 2);
        assert!(serpents_sense.detail.contains("Perception"));

        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("race.nagaji.alternate_trait.hypnotic_gaze")),
            "Hypnotic Gaze must NOT appear for a nagaji who has not selected it"
        );
    }

    /// Selecting `Nagaji ~ Hypnotic Gaze` REPLACES the default Serpent's
    /// Sense record with the real two-value Hypnotic Gaze computation (DC
    /// and caster level) -- a real override, not an addition: Serpent's
    /// Sense's own id must be absent once Hypnotic Gaze is selected.
    #[test]
    fn nagaji_hypnotic_gaze_alternate_replaces_the_default_serpents_sense_record() {
        let computation =
            compute_pilot_base_chassis(&input_for_race("nagaji", &["Nagaji ~ Hypnotic Gaze"]));

        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == "race.nagaji.trait_bundle.serpents_sense"),
            "Serpent's Sense must NOT appear once Hypnotic Gaze is selected"
        );

        // Hypnotic Gaze DC = 11 + Charisma modifier. The fixture's Charisma
        // score is 8 (floor(8/2) - 5 = -1), so DC = 11 + (-1) = 10.
        let dc = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.nagaji.alternate_trait.hypnotic_gaze.dc")
            .expect("nagaji must produce a Hypnotic Gaze DC explanation once selected");
        assert_eq!(dc.value, 10);
        assert!(dc.detail.contains("REAL computed value"));

        // Caster level = total character level. The fixture is a single
        // fighter level 1, so caster level = 1.
        let cl = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.nagaji.alternate_trait.hypnotic_gaze.caster_level")
            .expect("nagaji must produce a Hypnotic Gaze caster level explanation once selected");
        assert_eq!(cl.value, 1);
    }

    /// A different Charisma score changes the Hypnotic Gaze DC -- proving
    /// this is a real per-character computation, not a hardcoded literal.
    #[test]
    fn nagaji_hypnotic_gaze_dc_tracks_a_different_charisma_score() {
        let mut input = input_for_race("nagaji", &["Nagaji ~ Hypnotic Gaze"]);
        input.chosen.ability_scores.charisma = 18; // modifier = floor(18/2) - 5 = 4
        let computation = compute_pilot_base_chassis(&input);
        let dc = computation
            .explanations
            .iter()
            .find(|e| e.id == "race.nagaji.alternate_trait.hypnotic_gaze.dc")
            .expect("nagaji must produce a Hypnotic Gaze DC explanation");
        assert_eq!(dc.value, 15, "DC = 11 + 4 = 15 with an 18 Charisma score");
    }
}

/// Names every alternate racial trait this character has taken, and the
/// standard-trait replace-flags those choices fired.
///
/// Runs for every race, including the eleven with no hand-modelled seam of
/// their own: a chosen alternate is a real, persisted player decision, and a
/// sheet that shows nothing for it is the "selection vanished" failure this
/// repo has already shipped twice (`unresolved_spell_ids`,
/// `unresolved_equipment_item_ids`).
///
/// The record carries `value = 0` deliberately. Which *numbers* change is
/// decided per trait by the race seams above (today: Dwarf Minesight's
/// darkvision range, Half-Elf Dual Minded's Will save, and the removal of every
/// standard-trait record whose flag fired); summing anything here would be a
/// fabricated aggregate.
///
/// A selection key the pinned table does not know raises a **claim-blocking**
/// diagnostic rather than being dropped: a saved character naming a trait this
/// engine cannot place has an unknown racial trait bundle, and every number
/// derived from it is unproven.
pub(super) fn explain_selected_alternate_racial_traits(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let selected = selected_alternate_trait_keys(input);
    if selected.is_empty() {
        return;
    }

    let unknown = crate::rules_core::race_resolver::unknown_alternate_trait_keys(&selected);
    if !unknown.is_empty() {
        diagnostics.push(ComputationDiagnostic {
            id: "race.alternate_trait.unknown".to_owned(),
            message: format!(
                "chosen alternate racial trait(s) {unknown:?} name no record this engine knows; \
                 the ARG alternate-trait table covers the 153 selectable records across the 18 \
                 in-scope races (decisions.md §25.3), so nothing was replaced for these and the \
                 character's racial trait bundle is not fully grounded"
            ),
            claim_blocking: true,
        });
    }

    let fired = crate::rules_core::race_resolver::replace_flags_fired_by(&selected);
    explanations.push(ComputationExplanation {
        id: "race.alternate_trait.selected".to_owned(),
        value: 0,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   !PREFACT:1,ABILITIES,<flag>=True
            "Alternate racial traits chosen for {}: {}. Firing replace-flag(s) {}, each of which \
             suppresses the standard racial trait its own exclusion gate names \
             (decisions.md §26 — the swap is a \
             relationship the PCGen corpus declares, not one this engine invents). This record \
             names the selection itself and carries no mechanical value (+0); the numbers that \
             actually change are emitted by the affected per-trait records",
            input.chosen.race_id,
            selected.join(", "),
            if fired.is_empty() { "(none)".to_owned() } else { fired.join(", ") }
        ),
    });
}

/// v0.6 alpha swarm: QA found this was a systemic gap, not an Elf-only
/// oversight -- 4 of the game's core races (Elf, Dwarf, Gnome, Halfling)
/// each have a real PF1 3-stat racial ability adjustment (2 up, 1 down),
/// but this engine's race-seam functions only ever grounded 2 of the 3 for
/// each of them, apparently built from a "2-stat template" that never
/// accounted for the third. Elf was missing +2 Intelligence (its own
/// explanation text even mislabeled it "the alternate... variant... out of
/// scope", which was wrong per the real corpus); Dwarf was missing +2
/// Wisdom; Gnome was missing +2 Charisma; Halfling was missing +2
/// Charisma. Half-Elf/Half-Orc (floating +2, no fixed 3-stat set) and
/// Human (no racial ability adjustment at all) are unaffected -- verified
/// against the real PCGen corpus, not memory, before writing each
/// constant (see each race's own doc comment above for its specific
/// citation). Also folds in a related but separate finding: Gnome's and
/// Halfling's size-recognition record text claimed Small size "contributes
/// no numeric effect to attack rolls, AC, skill checks..." -- true in this
/// codebase today only because no size-modifier term exists anywhere in
/// the combat baseline for ANY race yet, not because PF1 Small size has no
/// real effect (it does: +1 AC, +1 attack, -1 CMB/CMD, +4 Stealth). Text
/// corrected to state the real rule and explain why it isn't applied,
/// rather than falsely implying Small size has no mechanical weight in
/// PF1 itself. Not wiring size into AC/attack now -- that's real future
/// scope overlapping the already-dropped AC/attack-bonus architecture
/// item.
///
/// Inline here for the same reason as this file's other inline test
/// modules (`tests/**` is QA's owned surface for this swarm); mirrors each
/// race's own `tests/sd13_*_race_semantics*.rs` substring-based assertion
/// style exactly, reusing their fixtures.
#[cfg(test)]
mod race_ability_modifier_parity_tests {
    use super::compute_pilot_base_chassis;
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};

    const ELF_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_elf_fighter_level1_sd13_deterministic_input.txt"
    );
    const DWARF_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_dwarf_fighter_level1_sd13_deterministic_input.txt"
    );
    const GNOME_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_gnome_fighter_level1_sd13_race_semantics_recognition_input.txt"
    );
    const HALFLING_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_halfling_fighter_level1_sd13_deterministic_input.txt"
    );

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

    fn ability_modifiers_record<'a>(
        computation: &'a super::PilotBaseChassisComputation,
        id: &str,
    ) -> &'a super::ComputationExplanation {
        computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .unwrap_or_else(|| panic!("expected explanation {id} to be grounded: {computation:?}"))
    }

    #[test]
    fn elf_ability_modifiers_record_now_names_the_intelligence_adjustment_too() {
        let input = load(ELF_FIXTURE);
        let computation = compute_pilot_base_chassis(&input);

        let ability = ability_modifiers_record(&computation, "race.elf.trait_bundle.ability_modifiers");

        assert!(
            ability.detail.contains("+2") && ability.detail.contains("Intelligence"),
            "Elf ability modifiers record must now name the +2 Intelligence adjustment: {}",
            ability.detail
        );
        assert!(
            !ability.detail.contains("out of scope"),
            "the stale \"out of scope\"/\"alternate variant\" framing must be gone: {}",
            ability.detail
        );
        assert_eq!(ability.value, 0, "still a bounded recognition record, no arithmetic performed");
    }

    #[test]
    fn dwarf_ability_modifiers_record_now_names_the_wisdom_adjustment_too() {
        let input = load(DWARF_FIXTURE);
        let computation = compute_pilot_base_chassis(&input);

        let ability = ability_modifiers_record(&computation, "race.dwarf.trait_bundle.ability_modifiers");

        assert!(
            ability.detail.contains("+2") && ability.detail.contains("Wisdom"),
            "Dwarf ability modifiers record must now name the +2 Wisdom adjustment: {}",
            ability.detail
        );
        assert_eq!(ability.value, 0);
    }

    #[test]
    fn gnome_ability_modifiers_record_now_names_the_charisma_adjustment_too() {
        let input = load(GNOME_FIXTURE);
        let computation = compute_pilot_base_chassis(&input);

        let ability = ability_modifiers_record(&computation, "race.gnome.trait_bundle.ability_modifiers");

        assert!(
            ability.detail.contains("+2") && ability.detail.contains("Charisma"),
            "Gnome ability modifiers record must now name the +2 Charisma adjustment: {}",
            ability.detail
        );
        assert_eq!(ability.value, 0);
    }

    #[test]
    fn halfling_ability_modifiers_record_now_names_the_charisma_adjustment_too() {
        let input = load(HALFLING_FIXTURE);
        let computation = compute_pilot_base_chassis(&input);

        let ability =
            ability_modifiers_record(&computation, "race.halfling.trait_bundle.ability_modifiers");

        assert!(
            ability.detail.contains("+2") && ability.detail.contains("Charisma"),
            "Halfling ability modifiers record must now name the +2 Charisma adjustment: {}",
            ability.detail
        );
        assert_eq!(ability.value, 0);
    }

    /// `SD31-E4-F1-004`: SD-27 (decisions.md §28 defect 1) wired the
    /// AC/attack-roll/CMB/CMD portion of PF1 Small size into this
    /// engine's general combat baseline months after this record's own
    /// text was written, leaving the record's own claim stale ("no
    /// size-modifier term exists anywhere in this engine's combat
    /// baseline for any race yet") -- provably false for a Gnome or
    /// Halfling Fighter/Wizard who reaches `compute_combat_baseline`'s
    /// supported posture. This test locks in the correction: the record
    /// now names the real mechanism it defers to, rather than denying
    /// one exists. Only Stealth genuinely remains unapplied (no Stealth
    /// skill total exists anywhere in this engine), and that boundary
    /// must still be named honestly, not silently dropped.
    #[test]
    fn gnome_and_halfling_size_records_now_cite_the_real_combat_baseline_mechanism() {
        let gnome_size = ability_modifiers_record(
            &compute_pilot_base_chassis(&load(GNOME_FIXTURE)),
            "race.gnome.trait_bundle.size",
        )
        .detail
        .clone();
        let halfling_size = ability_modifiers_record(
            &compute_pilot_base_chassis(&load(HALFLING_FIXTURE)),
            "race.halfling.trait_bundle.size",
        )
        .detail
        .clone();

        for detail in [&gnome_size, &halfling_size] {
            assert!(
                detail.contains("Small"),
                "size record must still name the Small size category: {detail}"
            );
            assert!(
                !detail.contains("no size-modifier term exists anywhere"),
                "the stale claim that no size-modifier term exists in this engine must be \
                 gone now that SD-27 wired one: {detail}"
            );
            assert!(
                detail.contains("+1 AC") && detail.contains("Stealth"),
                "the real PF1 Small-size effect must still be named: {detail}"
            );
            assert!(
                detail.contains("combat_size_modifiers"),
                "the record must now cite the real mechanism that applies AC/attack/CMB/CMD, \
                 not just describe the rule in the abstract: {detail}"
            );
            assert!(
                detail.contains("Stealth skill total exists"),
                "the record must still name the one genuinely unapplied piece (Stealth) \
                 honestly rather than implying full coverage: {detail}"
            );
        }
    }
}


/// Make the already-grounded Human pilot race seam explicit instead of leaving it an
/// incidental side effect of the numeric outputs.
///
/// This adds no new computed mechanic and no new input surface. It derives strictly
/// from existing chosen input — the `race:human` identity and the named
/// `choice:human_ability_bonus` and `choice:human_bonus_feat` selections — and from the
/// already-computed deterministic outputs — the ability modifiers and the grounded
/// Dodge armor-class contribution. It thereby surfaces the named Human ability-bonus and
/// bonus-feat interaction pressure as legible explanation records.
///
/// This function handles only the `race:human` branch of `explain_race_seam`;
/// non-Human routing (the bounded Half-Elf diagnostic and the
/// `race.semantics.unverified` catch-all) lives in the dispatcher. This slice
/// grounds no broader Human racial trait burden (size, speed, senses, extra
/// skill ranks).
pub(super) fn explain_human_pilot_race_seam(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HUMAN_RACE_ID {
        // Dwarf, Elf, Gnome, Half-Elf, Half-Orc, and Halfling carry their own
        // dedicated race-semantics seams (explain_dwarf_race_seam,
        // explain_elf_race_seam, explain_gnome_race_seam,
        // explain_half_elf_race_seam, explain_half_orc_race_seam,
        // explain_halfling_race_seam); they replace this generic diagnostic
        // rather than stacking alongside it. With Halfling landed, this branch
        // is unreachable for the seven-race SD-13 roster but stays as a
        // defensive fallback for any race identity outside that roster.
        if input.chosen.race_id != DWARF_RACE_ID
            && input.chosen.race_id != ELF_RACE_ID
            && input.chosen.race_id != GNOME_RACE_ID
            && input.chosen.race_id != HALF_ELF_RACE_ID
            && input.chosen.race_id != HALF_ORC_RACE_ID
            && input.chosen.race_id != HALFLING_RACE_ID
        {
            diagnostics.push(ComputationDiagnostic {
                id: "race.semantics.unverified".to_owned(),
                message: format!(
                    "race semantics are grounded only for {HUMAN_RACE_ID} on the deterministic pilot seam; \
                     chosen race {} has no grounded race semantics in this slice",
                    input.chosen.race_id
                ),
                claim_blocking: false,
            });
        }
        return;
    }

    // Human ability-bonus interaction: the named choice targets one ability. The
    // +2 racial adjustment was already applied to that ability's score BEFORE
    // ability modifiers were derived (see `apply_human_ability_bonus` and its
    // `race.human.ability_bonus_applied` explanation for the base-score
    // arithmetic). This record narrates the already-adjusted result only; it
    // performs no further arithmetic and does not re-apply the bonus.
    if let Some(selection) = choice_selection(input, HUMAN_ABILITY_BONUS_CHOICE_ID) {
        let ability = selection
            .strip_prefix(ABILITY_SELECTION_PREFIX)
            .unwrap_or(selection);
        let modifier = ability_modifier_for(ability_modifiers, ability);
        explanations.push(ComputationExplanation {
            id: "race.human.ability_bonus_target".to_owned(),
            value: modifier,
            detail: format!(
                "Human ability-bonus selection ({HUMAN_ABILITY_BONUS_CHOICE_ID} -> {selection}) targets \
                 {ability}; the racial-bonus-adjusted {ability} score (base chosen score + the +2 Human \
                 racial bonus, applied before ability modifiers were derived) yields modifier {modifier:+}"
            ),
        });
    }

    // Human bonus-feat interaction: the named choice grants a feat. Surface the grounded
    // Dodge armor-class contribution the deterministic baseline already relies on.
    if let Some(selection) = choice_selection(input, HUMAN_BONUS_FEAT_CHOICE_ID) {
        let (value, detail) = if selection == DODGE_FEAT_ID {
            (
                DODGE_AC_BONUS,
                format!(
                    "Human bonus-feat selection ({HUMAN_BONUS_FEAT_CHOICE_ID} -> {selection}) grants Dodge, \
                     the deterministic Dodge feat contributing {DODGE_AC_BONUS:+} to the baseline armor class"
                ),
            )
        } else {
            (
                0,
                format!(
                    "Human bonus-feat selection ({HUMAN_BONUS_FEAT_CHOICE_ID} -> {selection}) is a named Human \
                     bonus feat, but only the deterministic Dodge grant has a grounded computed contribution"
                ),
            )
        };
        explanations.push(ComputationExplanation {
            id: "race.human.bonus_feat_grant".to_owned(),
            value,
            detail,
        });
    }

    // Bounded honesty: only the named seam is grounded. This is explicit but
    // non-claim-blocking so the deterministic pilot still reports computed evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.human.bounded_semantics".to_owned(),
        message: "Human race semantics are grounded for the deterministic pilot's named \
                  ability-bonus and bonus-feat selections, and the SD13-E6-F3a trait bundle \
                  (size, speed, senses, extra skill ranks) is classified explicitly; the \
                  remaining PF1 Standard Human racial trait surface (alternate Human racial \
                  traits, variant Humans, half-Human heritages, and any ruleset-level effects \
                  outside the named deterministic pilot) remains unverified"
            .to_owned(),
        claim_blocking: false,
    });
}
/// SD13-E2 Dwarf racial trait bundle explanation seam (mirroring the SD13-E6-F3a
/// Human trait bundle pattern for the first non-Human core race), widened by the
/// SD18 dwarf-stonecunning, dwarf-greed, dwarf-hardy, dwarf-stability, and
/// dwarf-defensive-training cycles.
///
/// Surfaces nine grounded PF1 Core Rulebook Dwarf racial trait dimensions (ability
/// modifiers, size, speed, senses, Stonecunning, Greed, Hardy, Stability,
/// Defensive Training) as explicit `ComputationExplanation` records so the Dwarf
/// identity is legible on the runtime path rather than left behind the generic
/// `race.semantics.unverified` diagnostic every other non-Human race still receives.
///
/// This function:
///   - runs only when `race_id == race:dwarf`; every other race is unaffected
///     (Human keeps its own seam; every other non-Human race keeps the generic
///     `race.semantics.unverified` diagnostic from `explain_human_race_seam`),
///   - adds no new computed mechanical contribution beyond a flat, ungrounded-total
///     situational-bonus magnitude: the ability-modifiers record is recognition-only
///     (the chosen Constitution/Charisma scores are understood to already reflect
///     the fixed +2/-2 racial adjustment; no arithmetic is performed on this seam),
///     the size/senses records carry the grounded source value as identity only, and
///     the Stonecunning, Greed, Hardy, Stability, and Defensive Training records each
///     name only their own flat bonus magnitude, for the per-trait reasons set out
///     in the stale-justification correction below,
///   - replaces the generic `race.semantics.unverified` diagnostic with a
///     Dwarf-specific `race.dwarf.bounded_semantics` note naming the still-unproven
///     families explicitly (Hatred, weapon familiarity, and the explicit absence of
///     any Dwarf racial bonus feat),
///   - is bounded to race recognition only; it deliberately grounds no Dwarf
///     class-chassis interaction, no other race, and no PF1 alternate ruleset.
///
/// **Stale-justification correction (v0.6 Receipt-to-Sheet slice 1 item 5).**
/// This comment used to justify all five bonus records with a single sweeping
/// claim: "no Perception-check-total, Appraise-check-total,
/// stonework-detection, goods-valuation, saving-throw-total,
/// Combat-Maneuver-Defense-total, or Armor-Class-total engine exists". Three of
/// those seven are false against today's code — the same disease
/// `docs/release/v0.6/stale-deferral-sweep.md` found in the damage-reduction
/// family, and the same one task #88 already corrected on
/// `defense.baseline_armor_class` and `defense.total_save.*` elsewhere in this
/// file. What is actually true, per trait:
///   * **Stonecunning** (+2 Perception to notice unusual stonework) and
///     **Greed** (+2 Appraise on precious metals and gemstones): a
///     skill-check-total engine DOES exist —
///     `skill_allocation::allocate_skill_ranks`'s `SkillTotals`, wired into
///     `PilotReceipt.skills`. It does not recognize Perception or Appraise
///     (its universe is the bounded five in `skill_key_ability_modifier`), and
///     both bonuses are conditional on a sub-use a whole-skill total cannot
///     express, so they stay standalone — but not for the stated reason.
///   * **Hardy** (+2 vs poison, +2 vs spells and spell-like abilities): a
///     saving-throw-total engine DOES exist — `compute_total_saves` produces
///     `PilotBaseChassisComputation.total_saves` and the `defense.total_save.*`
///     records, and it already folds in unconditional feat bonuses via
///     `feat_effects::save_bonuses_from_feats`, so the plumbing for an
///     unconditional racial save bonus is present. Hardy is not one: it is
///     conditional on the save's SOURCE, and `BaseSaves` is three scalars
///     (Fortitude/Reflex/Will) with no by-source dimension. Adding it to the
///     total would apply it to every save, which is a wrong number.
///   * **Stability** (+4 CMD vs bull rush and trip while on the ground): no
///     Combat-Maneuver-Defense total exists anywhere in this codebase. This
///     part of the original claim is still TRUE.
///   * **Defensive Training** (+4 dodge to AC vs the giant subtype): an
///     Armor-Class-total engine DOES exist — `defense.baseline_armor_class`
///     from `compute_combat_baseline` (gated to the GE-06 equipment posture).
///     The bonus stays out of it because it is conditional on the opponent's
///     creature subtype, and `CharacterInput` models no opponent at all.
///
/// The pattern across all five: every one is correctly deferred, and every one
/// was deferred for a reason the text stated wrongly. A conditional bonus that
/// no total can express is a genuine deferral; "the total does not exist" was
/// not.
pub(super) fn explain_dwarf_race_seam(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != DWARF_RACE_ID {
        return;
    }

    // ----- ability modifiers -----
    // Recognition record only: PF1 Core Dwarf ability adjustments (+2 Con / -2 Cha)
    // are fixed, not a player choice. The chosen Constitution/Charisma scores are
    // understood to already reflect this adjustment; no arithmetic is performed here.
    explanations.push(ComputationExplanation {
        id: "race.dwarf.trait_bundle.ability_modifiers".to_owned(),
        value: 0,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   — BONUS:STAT|CON,WIS|2|TYPE=Racial, BONUS:STAT|CHA|-2|TYPE=Racial
            "Dwarf racial trait bundle — ability modifiers: PF1 Core Dwarf grants a fixed \
             {DWARF_CON_ADJUSTMENT:+} Constitution, {DWARF_WIS_ADJUSTMENT:+} Wisdom, and \
             {DWARF_CHA_ADJUSTMENT:+} Charisma racial adjustment (verified against the real PCGen \
             corpus: core_essentials/races/dwarf/dwarf_abilities_race.lst's \"Dwarf Racial Default\" \
             ability-score row). This is a bounded recognition record naming the fixed adjustment on \
             the deterministic pilot seam; the chosen Constitution, Wisdom, and Charisma scores are \
             understood to already reflect it, so this record performs no arithmetic and carries no \
             fabricated mechanical value (+0)"
        ),
    });

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.dwarf.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Dwarf racial trait bundle — size: PF1 Core Dwarf is {DWARF_SIZE_CATEGORY} size \
             (cr_races.lst race:dwarf SIZE:MEDIUM). This is a bounded recognition record naming \
             the Dwarf size category on the deterministic pilot seam; it contributes no numeric \
             effect to attack rolls, AC, skill checks, ability checks, or any other computed \
             value, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- speed -----
    // Recognition record for the 20 ft base land speed. PF1 Core Dwarf speed is
    // never reduced by armor or encumbrance, unlike most Medium races; this is
    // named explicitly as identity only — no computed speed-derived value is
    // fabricated.
    explanations.push(ComputationExplanation {
        id: "race.dwarf.trait_bundle.speed".to_owned(),
        value: DWARF_BASE_SPEED_FEET,
        detail: format!(
            "Dwarf racial trait bundle — speed: PF1 Core Dwarf has a base land speed of \
             {DWARF_BASE_SPEED_FEET} ft that is never reduced by armor or encumbrance \
             (cr_races.lst race:dwarf GAIT:WALK|{DWARF_BASE_SPEED_FEET}). This is a grounded \
             recognition value carrying the Dwarf base-speed identity on the deterministic pilot \
             seam; it contributes no computed speed-derived effect to any chassis output, skill \
             modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    // Recognition record for Darkvision 60 ft, distinct from Human's bounded
    // no-special-senses classification.
    //
    // SD-27 (alternate racial traits reach compute): the CRB `Dwarf ~ Vision`
    // row declares `!PREFACT:1,ABILITIES,Dwarf_ReplaceVision=True`, so it stops
    // applying the moment a selected ARG alternate sets that flag. Two
    // alternates do — `Dwarf ~ Minesight` and `Dwarf ~ Surface Survivalist` —
    // and Minesight replaces the sense with a *different range*, so this is the
    // one place on the Dwarf seam where a player's choice changes a grounded
    // number rather than only removing one.
    if replaced_by_alternate_trait(input, DWARF_REPLACE_VISION_FLAG) {
        if selected_alternate_trait_keys(input).iter().any(|key| key == DWARF_MINESIGHT_TRAIT_KEY) {
            explanations.push(ComputationExplanation {
                id: "race.dwarf.alternate_trait.minesight.senses".to_owned(),
                value: DWARF_MINESIGHT_DARKVISION_FEET,
                detail: format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   , gated !PREFACT:1,ABILITIES,{DWARF_REPLACE_VISION_FLAG}=True, which
                    //   Minesight sets
                    "Dwarf alternate racial trait — Minesight (Advanced Race Guide p.12): the chosen \
                     alternate replaces the standard Dwarf darkvision, increasing its range from \
                     {DWARF_DARKVISION_FEET} ft to {DWARF_MINESIGHT_DARKVISION_FEET} ft \
                     (arg_abilities_race.lst:39 KEY:Dwarf ~ Minesight, VISION:Darkvision \
                     ({DWARF_MINESIGHT_DARKVISION_FEET}); the standard row it replaces is \
                     dwarf_abilities_race.lst's Dwarf ~ Vision). The standard \
                     {DWARF_DARKVISION_FEET} ft record is therefore NOT emitted for this character. \
                     Minesight's own drawbacks — automatically dazzled in bright light, and a -2 \
                     penalty on saving throws against effects with the light descriptor — are \
                     carried in the trait's corpus description and are deliberately not folded into \
                     any save total, for the same reason Dwarf Hardy is not: they are conditional on \
                     what the save is against, and BaseSaves has no by-source dimension"
                ),
            });
        }
    } else {
        explanations.push(ComputationExplanation {
            id: "race.dwarf.trait_bundle.senses".to_owned(),
            value: DWARF_DARKVISION_FEET,
            detail: format!(
                "Dwarf racial trait bundle — senses: PF1 Core Dwarf grants Darkvision \
                 {DWARF_DARKVISION_FEET} ft (cr_races.lst race:dwarf SENSE:Darkvision \
                 ({DWARF_DARKVISION_FEET} ft)). This is a grounded recognition value carrying the \
                 Dwarf Darkvision identity on the deterministic pilot seam; it contributes no \
                 computed low-light or perception-derived effect to any chassis output"
            ),
        });
    }

    // ----- Stonecunning (SD18 dwarf-stonecunning cycle) -----
    // Grounded flat +2 situational bonus on Perception checks to potentially
    // notice unusual stonework, such as traps and hidden doors located in
    // stone walls or floors (core_essentials/races/dwarf/dwarf_abilities_race.lst:27
    // BONUS:SITUATION|Perception=to notice unusual stonework|2|TYPE=Racial;
    // dwarf_skills.lst:6 Perception.MOD SITUATION:to notice unusual
    // stonework). Mirrors the existing flat skill-bonus-magnitude idiom used
    // elsewhere on this seam (e.g. Bard Inspire Competence, Ranger Track): no
    // Perception-check-total or stonework-detection engine exists anywhere in
    // this codebase, so this names only the flat situational-bonus magnitude,
    // not a check-execution engine. Distinct from Greed (a separate +2
    // Appraise racial trait for assessing nonmagical precious-metal/gemstone
    // goods), which remains unground.
    if !replaced_by_alternate_trait(input, DWARF_REPLACE_STONECUNNING_FLAG) {
    explanations.push(ComputationExplanation {
            id: "race.dwarf.trait_bundle.stonecunning".to_owned(),
            value: DWARF_STONECUNNING_PERCEPTION_BONUS,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:SITUATION|Perception=to notice unusual
                //   stonework|{DWARF_STONECUNNING_PERCEPTION_BONUS}|TYPE=Racial; dwarf_skills.lst:6
                //   Perception.MOD SITUATION:to notice unusual stonework
                "Dwarf racial trait bundle — Stonecunning: PF1 Core Dwarf grants a flat \
                 {DWARF_STONECUNNING_PERCEPTION_BONUS:+} bonus on Perception checks to potentially \
                 notice unusual stonework, such as traps and hidden doors located in stone walls or \
                 floors (dwarf_abilities_race.lst:27). This is a bounded flat \
                 situational-bonus-magnitude recognition record naming the Stonecunning identity on \
                 the deterministic pilot seam; no Perception-check-total or stonework-detection \
                 engine exists anywhere in this codebase, so no check resolution is fabricated from \
                 this record"
            ),
        });
    }

    // ----- Greed (SD18 dwarf-greed cycle) -----
    // Grounded flat +2 situational bonus on Appraise checks made to
    // determine the price of nonmagical goods that contain precious metals
    // or gemstones (core_essentials/races/dwarf/dwarf_abilities_race.lst:23
    // BONUS:SITUATION|Appraise=to assess nonmagical metals or gemstones|2|TYPE=Racial;
    // dwarf_skills.lst:5 Appraise.MOD SITUATION:to assess nonmagical metals
    // or gemstones). Mirrors the already-grounded Stonecunning flat
    // skill-bonus-magnitude idiom on this same seam: no
    // Appraise-check-total or goods-valuation engine exists anywhere in
    // this codebase, so this names only the flat situational-bonus
    // magnitude, not a check-execution engine.
    if !replaced_by_alternate_trait(input, DWARF_REPLACE_GREED_FLAG) {
    explanations.push(ComputationExplanation {
            id: "race.dwarf.trait_bundle.greed".to_owned(),
            value: DWARF_GREED_APPRAISE_BONUS,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:SITUATION|Appraise=to assess nonmagical metals or
                //   gemstones|{DWARF_GREED_APPRAISE_BONUS}|TYPE=Racial; dwarf_skills.lst:5
                //   Appraise.MOD SITUATION:to assess nonmagical metals or gemstones
                "Dwarf racial trait bundle — Greed: PF1 Core Dwarf grants a flat \
                 {DWARF_GREED_APPRAISE_BONUS:+} bonus on Appraise checks made to determine the price \
                 of nonmagical goods that contain precious metals or gemstones \
                 (dwarf_abilities_race.lst:23). This is a bounded flat situational-bonus-magnitude \
                 recognition record naming the Greed identity on the deterministic pilot seam; no \
                 Appraise-check-total or goods-valuation engine exists anywhere in this codebase, so \
                 no check resolution is fabricated from this record"
            ),
        });
    }

    // ----- Hardy (SD18 dwarf-hardy cycle) -----
    // Bundles two distinct save categories, both grounded honestly, mirroring
    // the already-landed Elf Elven Immunities flat racial saving-throw-bonus
    // idiom exactly (applied to a save category instead of a skill):
    //   - a +2 racial saving throw bonus against poison
    //     (dwarf_abilities_race.lst:25 BONUS:VAR|SaveBonus_vs_Poison|2|TYPE=Racial);
    //   - a +2 racial saving throw bonus against spells and spell-like
    //     abilities (dwarf_abilities_race.lst:25
    //     BONUS:VAR|SaveBonus_vs_Spells|2|TYPE=Racial).
    // Both share the same flat magnitude, so this names only the flat
    // save-bonus magnitude and does not fold it into the save total.
    //
    // v0.6 Receipt-to-Sheet slice 1 item 5: the shipped detail below used to
    // justify that with "no saving-throw-total engine exists anywhere in this
    // codebase". That was false — `compute_total_saves` produces
    // `PilotBaseChassisComputation.total_saves` and the `defense.total_save.*`
    // records, and already folds in unconditional feat bonuses via
    // `feat_effects::save_bonuses_from_feats`. The deferral is nonetheless
    // correct, for the reason now stated instead: Hardy is conditional on what
    // the save is AGAINST, and `BaseSaves` is three by-category scalars with no
    // by-source dimension, so adding it would apply it to every save.
    if !replaced_by_alternate_trait(input, DWARF_REPLACE_HARDY_FLAG) {
    explanations.push(ComputationExplanation {
            id: "race.dwarf.trait_bundle.hardy".to_owned(),
            value: DWARF_HARDY_SAVE_BONUS,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|SaveBonus_vs_Poison|{DWARF_HARDY_SAVE_BONUS}|TYPE=Racial,
                //   BONUS:VAR|SaveBonus_vs_Spells|{DWARF_HARDY_SAVE_BONUS}|TYPE=Racial
                "Dwarf racial trait bundle — Hardy: PF1 Core Dwarf grants a flat \
                 {DWARF_HARDY_SAVE_BONUS:+} racial bonus on saving throws against poison, and a flat \
                 {DWARF_HARDY_SAVE_BONUS:+} racial bonus on saving throws against spells and \
                 spell-like abilities (dwarf_abilities_race.lst:25). This is a bounded flat \
                 saving-throw-bonus-magnitude recognition record naming the Hardy identity on the \
                 deterministic pilot seam, mirroring the already-grounded Elf Elven Immunities \
                 enchantment-save-bonus idiom. It is deliberately NOT added to this character's \
                 integrated save totals, which do exist (`total_saves`, surfaced as the \
                 `defense.total_save.*` records, and already folding in unconditional feat save \
                 bonuses): both halves of Hardy are conditional on what the saving throw is against, \
                 and a save total is three by-category scalars (Fortitude/Reflex/Will) with no \
                 by-source dimension, so folding a vs-poison or vs-spells bonus into one would \
                 wrongly apply it to every save of that category. The magnitude is therefore \
                 reported standalone, and no check resolution is fabricated from this record"
            ),
        });
    }

    // ----- Stability (SD18 dwarf-stability cycle) -----
    // Bundles two distinct combat-maneuver-defense categories, both grounded
    // honestly, mirroring the already-landed Dwarf Hardy flat racial-bonus
    // idiom exactly (a single flat magnitude applied to two named derived-
    // stat targets instead of two save categories):
    //   - a +4 racial bonus to Combat Maneuver Defense when resisting a bull
    //     rush attempt while standing on the ground
    //     (dwarf_abilities_race.lst:26 BONUS:VAR|CMD_BullRush|4|TYPE=Racial);
    //   - a +4 racial bonus to Combat Maneuver Defense when resisting a trip
    //     attempt while standing on the ground (dwarf_abilities_race.lst:26
    //     BONUS:VAR|CMD_Trip|4|TYPE=Racial).
    // Both share the same flat magnitude, so this names only the flat
    // CMD-bonus magnitude, not a Combat-Maneuver-Defense-total engine: no
    // such engine exists anywhere in this codebase, so no check resolution
    // is fabricated from this record.
    if !replaced_by_alternate_trait(input, DWARF_REPLACE_STABILITY_FLAG) {
    explanations.push(ComputationExplanation {
            id: "race.dwarf.trait_bundle.stability".to_owned(),
            value: DWARF_STABILITY_CMD_BONUS,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|CMD_BullRush,CMD_Trip|{DWARF_STABILITY_CMD_BONUS}|TYPE=Racial
                "Dwarf racial trait bundle — Stability: PF1 Core Dwarf grants a flat \
                 {DWARF_STABILITY_CMD_BONUS:+} racial bonus to Combat Maneuver Defense when \
                 resisting a bull rush attempt, and a flat {DWARF_STABILITY_CMD_BONUS:+} racial \
                 bonus to Combat Maneuver Defense when resisting a trip attempt, in both cases while \
                 standing on the ground (dwarf_abilities_race.lst:26). This is a bounded flat \
                 CMD-bonus-magnitude recognition record naming the Stability identity on the \
                 deterministic pilot seam, mirroring the already-grounded Dwarf Hardy \
                 two-save-category flat-bonus idiom; no Combat-Maneuver-Defense-total engine exists \
                 anywhere in this codebase, so no check resolution is fabricated from this record"
            ),
        });
    }

    // ----- Defensive Training (SD18 dwarf-defensive-training cycle) -----
    // Grounded flat +4 dodge bonus to Armor Class against monsters of the
    // giant subtype (core_essentials/races/dwarf/dwarf_abilities_race.lst:22
    // BONUS:VAR|RacialDefensiveTrainingBonus|4). Mirrors the already-grounded
    // Dwarf Stability flat-bonus idiom on this same seam (a single flat
    // magnitude applied to a single named derived-stat target): no
    // Armor-Class-total or giant-subtype-detection engine exists anywhere in
    // this codebase, so this names only the flat dodge-bonus magnitude, not
    // a check-execution engine.
    if !replaced_by_alternate_trait(input, DWARF_REPLACE_DEFENSIVE_TRAINING_FLAG) {
    explanations.push(ComputationExplanation {
            id: "race.dwarf.trait_bundle.defensive_training".to_owned(),
            value: DWARF_DEFENSIVE_TRAINING_DODGE_BONUS,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|RacialDefensiveTrainingBonus|{DWARF_DEFENSIVE_TRAINING_DODGE_BONUS}
                "Dwarf racial trait bundle — Defensive Training: PF1 Core Dwarf grants a flat \
                 {DWARF_DEFENSIVE_TRAINING_DODGE_BONUS:+} dodge bonus to Armor Class against \
                 monsters of the giant subtype (dwarf_abilities_race.lst:22). This is a bounded flat \
                 dodge-bonus-magnitude recognition record naming the Defensive Training identity on \
                 the deterministic pilot seam, mirroring the already-grounded Dwarf Stability \
                 flat-bonus idiom; no Armor-Class-total or giant-subtype-detection engine exists \
                 anywhere in this codebase, so no check resolution is fabricated from this record"
            ),
        });
    }

    // Bounded honesty: nine named dimensions are now grounded (ability
    // modifiers, size, speed, senses, Stonecunning, Greed, Hardy, Stability,
    // Defensive Training). This replaces the generic
    // race.semantics.unverified diagnostic for Dwarf specifically and stays
    // non-claim-blocking so the deterministic pilot still reports computed
    // evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.dwarf.bounded_semantics".to_owned(),
        message: "Dwarf race semantics are grounded for the deterministic pilot's ability \
                  modifiers, size, speed, senses, Stonecunning (flat +2 Perception \
                  situational bonus to notice unusual stonework), Greed (flat +2 \
                  Appraise situational bonus to assess nonmagical precious-metal/gemstone \
                  goods), Hardy (flat +2 racial bonus on saving throws against poison, \
                  spells, and spell-like abilities), Stability (flat +4 racial bonus to \
                  Combat Maneuver Defense against bull rush and trip attempts while standing \
                  on the ground), and Defensive Training (flat +4 dodge bonus to Armor Class \
                  against monsters of the giant subtype) trait bundle; the remaining PF1 \
                  Core Dwarf racial trait surface remains unverified: Hatred (bonus on \
                  attack rolls against orcs and goblinoids), and weapon familiarity \
                  (battleaxe, heavy pick, warhammer, dwarven waraxe, dwarven urgrosh). PF1 \
                  core Dwarves gain no racial bonus feat (unlike Human), so that family is \
                  explicitly not applicable rather than silently omitted."
            .to_owned(),
        claim_blocking: false,
    });
}
/// SD13-E2 Elf racial trait bundle explanation seam (mirroring the Dwarf pattern
/// for the second non-Human core race).
///
/// Surfaces four grounded PF1 Core Rulebook Elf racial trait dimensions (ability
/// modifiers, size, speed, senses) as explicit `ComputationExplanation` records so
/// the Elf identity is legible on the runtime path rather than left behind the
/// generic `race.semantics.unverified` diagnostic every other non-Human race still
/// receives.
///
/// This function:
///   - runs only when `race_id == race:elf`; every other race is unaffected
///     (Human and Dwarf keep their own seams; every other non-Human race keeps
///     the generic `race.semantics.unverified` diagnostic),
///   - adds no new computed mechanical contribution: the ability-modifiers record
///     is recognition-only (the chosen Dexterity/Constitution scores are
///     understood to already reflect the fixed +2/-2 racial adjustment; no
///     arithmetic is performed on this seam), and the size/senses records carry
///     the grounded source value as identity only,
///   - replaces the generic `race.semantics.unverified` diagnostic with an
///     Elf-specific `race.elf.bounded_semantics` note naming the still-unproven
///     families explicitly (weapon familiarity, bonus languages, and the
///     explicit absence of any Elf racial bonus feat) now that Keen Senses
///     and Elven Immunities are both grounded,
///   - is bounded to race recognition only; it deliberately grounds no Elf
///     class-chassis interaction, no other race, no alternate +2 Intelligence
///     ability variant, and no PF1 alternate ruleset.
pub(super) fn explain_elf_race_seam(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != ELF_RACE_ID {
        return;
    }

    // ----- ability modifiers -----
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.ability_modifiers".to_owned(),
        value: 0,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   — BONUS:STAT|DEX,INT|2|TYPE=Racial, BONUS:STAT|CON|-2|TYPE=Racial; cr_races.lst's
            //   own Elf.MOD record is only a source-page citation, not the real base-race data
            "Elf racial trait bundle — ability modifiers: PF1 Core Elf grants a fixed \
             {ELF_DEX_ADJUSTMENT:+} Dexterity, {ELF_INT_ADJUSTMENT:+} Intelligence, and \
             {ELF_CON_ADJUSTMENT:+} Constitution racial adjustment (verified against the real PCGen \
             corpus: core_essentials/races/elf/elf_abilities_race.lst's \"Elf Racial Default\" \
             ability-score row, not an alternate/optional variant). This is a bounded recognition \
             record naming the fixed adjustment on the deterministic pilot seam; the chosen \
             Dexterity, Intelligence, and Constitution scores are understood to already reflect it, \
             so this record performs no arithmetic and carries no fabricated mechanical value (+0)."
        ),
    });

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Elf racial trait bundle — size: PF1 Core Elf is {ELF_SIZE_CATEGORY} size \
             (cr_races.lst race:elf SIZE:MEDIUM). This is a bounded recognition record naming \
             the Elf size category on the deterministic pilot seam; it contributes no numeric \
             effect to attack rolls, AC, skill checks, ability checks, or any other computed \
             value, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- speed -----
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.speed".to_owned(),
        value: ELF_BASE_SPEED_FEET,
        detail: format!(
            "Elf racial trait bundle — speed: PF1 Core Elf has a base land speed of \
             {ELF_BASE_SPEED_FEET} ft (cr_races.lst race:elf GAIT:WALK|{ELF_BASE_SPEED_FEET}). \
             This is a grounded recognition value carrying the Elf base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    // Low-light vision is a binary trait (doubles effective light for vision
    // purposes), not a distance magnitude like Dwarf Darkvision; the recognition
    // value stays +0.
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Elf racial trait bundle — senses: PF1 Core Elf grants low-light vision \
                  (cr_races.lst race:elf SENSE:Low-Light Vision). This is a bounded recognition \
                  record naming the Elf low-light vision identity on the deterministic pilot \
                  seam; it contributes no computed illumination or perception-derived effect to \
                  any chassis output, so it carries no fabricated mechanical value (+0)"
            .to_owned(),
    });

    // ----- Keen Senses -----
    // Flat +2 racial bonus on Perception checks. Mirrors the Dwarf Stonecunning
    // idiom (flat skill-bonus-magnitude recognition record, not a Perception
    // check-total engine).
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.keen_senses".to_owned(),
        value: 2,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — BONUS:SKILL|Perception|KeenSensesBonus|TYPE=Racial, BONUS:VAR|KeenSensesBonus|2
        detail: "Elf racial trait bundle — Keen Senses: PF1 Core Elf grants a flat +2 racial bonus \
                  on Perception checks (core_essentials/races/elf/elf_abilities_race.lst Keen Senses \
                  entry). This is a bounded recognition record naming only the flat racial-bonus \
                  magnitude on the deterministic pilot seam, not a Perception-check-total engine."
            .to_owned(),
    });

    // ----- Elven Immunities -----
    // Bundles two distinct sub-effects, both grounded honestly:
    //   - immunity to magic sleep effects: a flat, no-magnitude grant-only
    //     identity record, mirroring the Monk Purity of Body / Diamond Body
    //     disease/poison-immunity idiom exactly — no sleep-effect-resolution
    //     engine exists anywhere in this codebase to apply the immunity to;
    //   - a +2 racial saving throw bonus against enchantment spells and
    //     effects: a flat racial-bonus magnitude, mirroring the Keen Senses /
    //     Stonecunning / Greed flat-bonus idiom (applied to a save category
    //     instead of a skill), not a saving-throw-total engine.
    // The record's numeric value (2) names only the save-bonus magnitude;
    // the sleep immunity is named in the detail text as a non-fabricated
    // grant-only fact, contributing no additional numeric value.
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.elven_immunities".to_owned(),
        value: 2,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — DESC:\"Elves are immune to magic sleep effects and get a +2 racial saving throw bonus
        //   against enchantment spells and effects.\", ABILITY:Special Ability|AUTOMATIC|Immunity
        //   to Sleep, BONUS:VAR|SaveBonus_vs_Enchantments|2|TYPE=Racial
        detail: "Elf racial trait bundle — Elven Immunities: PF1 Core Elf is immune to magic sleep \
                  effects and gets a flat +2 racial saving throw bonus against enchantment spells \
                  and effects (core_essentials/races/elf/elf_abilities_race.lst Elven Immunities \
                  entry). The sleep immunity is a bounded grant-only identity record \
                  (non-fabricated): no sleep-effect- resolution engine exists anywhere in this \
                  codebase to apply the immunity to. The recognized numeric value (2) names only the \
                  flat enchantment saving-throw-bonus magnitude, not a saving-throw-total engine."
            .to_owned(),
    });

    // ----- Elven Magic -----
    // Bundles two distinct sub-effects, both grounded honestly, mirroring the
    // Elven Immunities idiom exactly:
    //   - a +2 racial bonus on caster level checks made to overcome spell
    //     resistance: a flat, no-magnitude grant-only identity record (no
    //     caster-level-check / spell-resistance-resolution engine exists
    //     anywhere in this codebase to apply the bonus to);
    //   - a +2 racial bonus on Spellcraft skill checks made to identify the
    //     properties of magic items: a flat racial-bonus magnitude,
    //     mirroring the Keen Senses / Stonecunning / Greed flat-bonus idiom
    //     (applied to the Spellcraft skill), not a Spellcraft-check-total
    //     engine.
    // This trait was not previously named in the Elf row's unproven-family
    // list (which named only weapon familiarity and bonus language grants);
    // it is a genuine, distinct PF1 Core Elf racial trait present in the
    // corpus that this slice newly recognizes.
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.elven_magic".to_owned(),
        value: 2,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — DESC:\"Elves receive a +2 racial bonus on caster level checks made to overcome spell
        //   resistance. In addition, elves receive a +2 racial bonus on Spellcraft skill checks
        //   made to identify the properties of magic items.\", BONUS:SITUATION|Spellcraft=to
        //   identify magic item properties|2|TYPE=Racial
        detail: "Elf racial trait bundle — Elven Magic: PF1 Core Elf gets a flat +2 racial bonus on \
                  caster level checks made to overcome spell resistance, and a flat +2 racial bonus \
                  on Spellcraft skill checks made to identify the properties of magic items \
                  (core_essentials/races/elf/elf_abilities_race.lst Elven Magic entry). The \
                  caster-level-check-vs-spell-resistance bonus is a bounded grant-only identity \
                  record (non-fabricated): no caster-level-check or spell-resistance-resolution \
                  engine exists anywhere in this codebase to apply the bonus to. The recognized \
                  numeric value (2) names only the flat Spellcraft-identify racial-bonus magnitude, \
                  not a Spellcraft-check-total engine."
            .to_owned(),
    });

    // Bounded honesty: only the seven named dimensions are grounded. This
    // replaces the generic race.semantics.unverified diagnostic for Elf
    // specifically and stays non-claim-blocking so the deterministic pilot
    // still reports computed evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.elf.bounded_semantics".to_owned(),
        message: "Elf race semantics are grounded for the deterministic pilot's ability \
                  modifiers, size, speed, senses, Keen Senses (Perception bonus), Elven \
                  Immunities (sleep immunity plus enchantment save bonus), and Elven Magic \
                  (caster level check vs. spell resistance bonus plus Spellcraft identify \
                  bonus) trait bundle; the remaining PF1 Core Elf racial trait surface remains \
                  unverified: weapon familiarity (longbow, composite longbow, longsword, \
                  rapier, shortbow, composite shortbow), and bonus language grants. PF1 core \
                  Elves gain no racial bonus feat (unlike Human), so that family is explicitly \
                  not applicable rather than silently omitted."
            .to_owned(),
        claim_blocking: false,
    });
}
/// SD13-E2/SD18 Gnome racial trait bundle explanation seam (mirroring the
/// Dwarf/Elf pattern for the third non-Human core race).
///
/// Surfaces six grounded PF1 Core Rulebook Gnome racial trait dimensions
/// (ability modifiers, size, speed, senses, Keen Senses, Illusion
/// Resistance) as explicit `ComputationExplanation` records so the Gnome
/// identity is legible on the runtime path rather than left behind the
/// generic `race.semantics.unverified` diagnostic every other non-Human race
/// still receives.
///
/// This function:
///   - runs only when `race_id == race:gnome`; every other race is unaffected
///     (Human, Dwarf, and Elf keep their own seams; every other non-Human race
///     keeps the generic `race.semantics.unverified` diagnostic),
///   - adds no new computed mechanical contribution beyond the flat Keen
///     Senses and Illusion Resistance bonus magnitudes: the ability-modifiers
///     record is recognition-only (the chosen Constitution/Strength scores are
///     understood to already reflect the fixed +2/-2 racial adjustment; no
///     arithmetic is performed on this seam), and the size/senses records carry
///     the grounded source value as identity only,
///   - replaces the generic `race.semantics.unverified` diagnostic with a
///     Gnome-specific `race.gnome.bounded_semantics` note naming the
///     still-unproven families explicitly (Defensive Training, Hatred, Gnome
///     Magic, weapon familiarity, and the explicit absence of any Gnome
///     racial bonus feat),
///   - is bounded to race recognition only; it deliberately grounds no Gnome
///     class-chassis interaction, no other race, and no PF1 alternate ruleset.
pub(super) fn explain_gnome_race_seam(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != GNOME_RACE_ID {
        return;
    }

    // ----- ability modifiers -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.ability_modifiers".to_owned(),
        value: 0,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   — BONUS:STAT|CON,CHA|2|TYPE=Racial, BONUS:STAT|STR|-2|TYPE=Racial
            "Gnome racial trait bundle — ability modifiers: PF1 Core Gnome grants a fixed \
             {GNOME_CON_ADJUSTMENT:+} Constitution, {GNOME_CHA_ADJUSTMENT:+} Charisma, and \
             {GNOME_STR_ADJUSTMENT:+} Strength racial adjustment (verified against the real PCGen \
             corpus: core_essentials/races/gnome/gnome_abilities_race.lst's \"Gnome Racial Default\" \
             ability-score row). This is a bounded recognition record naming the fixed adjustment on \
             the deterministic pilot seam; the chosen Constitution, Charisma, and Strength scores \
             are understood to already reflect it, so this record performs no arithmetic and carries \
             no fabricated mechanical value (+0)"
        ),
    });

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Gnome racial trait bundle — size: PF1 Core Gnome is {GNOME_SIZE_CATEGORY} size \
             (cr_races.lst race:gnome SIZE:SMALL). This is a bounded recognition record naming \
             the Gnome size category on the deterministic pilot seam; it performs no arithmetic \
             of its own (+0) so it does not double-count the real PF1 Small-size effect (+1 AC, \
             +1 attack rolls, -1 CMB/CMD, and +4 Stealth). SD-27 (decisions.md §28 defect 1) \
             wired the AC/attack/CMB/CMD portion into this engine's general combat baseline \
             (`combat_size_modifiers`, keyed off `race_size_for_race_token`): any Gnome \
             character who reaches `compute_combat_baseline`'s supported posture gets the real \
             +1 AC/attack and -1 CMB/CMD from that shared term, not from this record. The \
             +4 Stealth portion is still not applied anywhere: no Stealth skill total exists in \
             this engine yet (`compute_selected_skill_modifiers` supports only Climb, \
             Intimidate and Swim)"
        ),
    });

    // ----- speed -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.speed".to_owned(),
        value: GNOME_BASE_SPEED_FEET,
        detail: format!(
            "Gnome racial trait bundle — speed: PF1 Core Gnome has a base land speed of \
             {GNOME_BASE_SPEED_FEET} ft (cr_races.lst race:gnome GAIT:WALK|{GNOME_BASE_SPEED_FEET}). \
             This is a grounded recognition value carrying the Gnome base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Gnome racial trait bundle — senses: PF1 Core Gnome grants low-light vision \
                  (cr_races.lst race:gnome SENSE:Low-Light Vision). This is a bounded \
                  recognition record naming the Gnome low-light vision identity on the \
                  deterministic pilot seam; it contributes no computed illumination or \
                  perception-derived effect to any chassis output, so it carries no fabricated \
                  mechanical value (+0)"
            .to_owned(),
    });

    // ----- Keen Senses -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.keen_senses".to_owned(),
        value: 2,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — BONUS:SKILL|Perception|KeenSensesBonus|TYPE=Racial, BONUS:VAR|KeenSensesBonus|2
        detail: "Gnome racial trait bundle — Keen Senses: PF1 Core Gnome grants a flat +2 racial \
                  bonus on Perception skill checks \
                  (core_essentials/races/gnome/gnome_abilities_race.lst Keen Senses entry). This is \
                  a bounded recognition record naming only the flat racial-bonus magnitude on the \
                  deterministic pilot seam, not a Perception-check-total engine."
            .to_owned(),
    });

    // ----- Illusion Resistance -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.illusion_resistance".to_owned(),
        value: 2,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — BONUS:VAR|SaveBonus_vs_Illusions|2|TYPE=Racial
        detail: "Gnome racial trait bundle — Illusion Resistance: PF1 Core Gnome grants a flat +2 \
                  racial saving throw bonus against illusion spells and effects \
                  (core_essentials/races/gnome/gnome_abilities_race.lst Illusion Resistance entry). \
                  This is a bounded recognition record naming only the flat racial-bonus magnitude \
                  on the deterministic pilot seam, not a saving-throw-total engine."
            .to_owned(),
    });

    // ----- Defensive Training -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.defensive_training".to_owned(),
        value: 4,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — BONUS:VAR|RacialDefensiveTrainingBonus|4
        detail: "Gnome racial trait bundle — Defensive Training: PF1 Core Gnome grants a flat +4 \
                  dodge bonus to AC against monsters of the giant subtype \
                  (core_essentials/races/gnome/gnome_abilities_race.lst Defensive Training entry). \
                  This is a bounded recognition record naming only the flat racial-bonus magnitude \
                  on the deterministic pilot seam, not an AC-total engine and not a \
                  giant-subtype-detection engine (no \"is the target a giant\" resolution is \
                  fabricated)."
            .to_owned(),
    });

    // ----- Hatred -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.hatred".to_owned(),
        value: 1,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — BONUS:VAR|Gnome_Hatred_AttackBonus|1
        detail: "Gnome racial trait bundle — Hatred: PF1 Core Gnome grants a flat +1 racial bonus on \
                  attack rolls against humanoid creatures of the reptilian and goblinoid subtypes \
                  (core_essentials/races/gnome/gnome_abilities_race.lst Hatred entry). This is a \
                  bounded recognition record naming only the flat racial-bonus magnitude on the \
                  deterministic pilot seam, not an attack-roll-total engine and not a \
                  reptilian/goblinoid-subtype- detection engine (no \"is the target a reptilian \
                  humanoid or goblinoid\" resolution is fabricated)."
            .to_owned(),
    });

    // Bounded honesty: only the eight named dimensions are grounded. This replaces
    // the generic race.semantics.unverified diagnostic for Gnome specifically and
    // stays non-claim-blocking so the deterministic pilot still reports computed
    // evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.gnome.bounded_semantics".to_owned(),
        message: "Gnome race semantics are grounded for the deterministic pilot's ability \
                  modifiers, size, speed, senses, Keen Senses (Perception bonus), Illusion \
                  Resistance (illusion-save bonus), Defensive Training (dodge bonus to AC \
                  against giants), and Hatred (attack-roll bonus against reptilian humanoids \
                  and goblinoids) trait bundle; the remaining PF1 Core Gnome racial trait \
                  surface remains unverified: Gnome Magic (spell-like abilities keyed to a high \
                  Charisma), and weapon familiarity (gnome hooked hammer). PF1 core Gnomes gain \
                  no racial bonus feat (unlike Human), so that family is explicitly not \
                  applicable rather than silently omitted."
            .to_owned(),
        claim_blocking: false,
    });
}
/// SD13-E2/SD18 Half-Orc racial trait bundle explanation seam (mirroring the
/// Half-Elf choice-based ability-bonus pattern for the fifth non-Human core
/// race, with Darkvision instead of low-light vision).
///
/// Surfaces five grounded PF1 Core Rulebook Half-Orc racial trait dimensions
/// (chosen ability-bonus target, size, speed, senses, Intimidating) as
/// explicit `ComputationExplanation` records so the Half-Orc identity is
/// legible on the runtime path rather than left behind the generic
/// `race.semantics.unverified` diagnostic every other non-Human race still
/// receives.
///
/// This function:
///   - runs only when `race_id == race:half-orc`; every other race is
///     unaffected (Human, Dwarf, Elf, Gnome, and Half-Elf keep their own seams;
///     every other non-Human race keeps the generic `race.semantics.unverified`
///     diagnostic),
///   - adds no new computed mechanical contribution beyond the flat
///     Intimidating skill-bonus magnitude (SD18): the ability-bonus-target
///     record surfaces the already-computed modifier for the chosen ability as
///     recognition, the size/senses records carry the grounded source value as
///     identity only, and Intimidating names only the flat racial-bonus
///     magnitude (mirroring the Dwarf Stonecunning / Elf/Gnome/Half-Elf Keen
///     Senses idiom) — no Intimidate-check-total engine is introduced,
///   - replaces the generic `race.semantics.unverified` diagnostic with a
///     Half-Orc-specific `race.half_orc.bounded_semantics` note naming the
///     still-unproven families explicitly (Orc Ferocity, weapon familiarity),
///   - is bounded to race recognition only; it deliberately grounds no Half-Orc
///     class-chassis interaction, no other race, and no PF1 alternate ruleset.
pub(super) fn explain_half_orc_race_seam(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HALF_ORC_RACE_ID {
        return;
    }

    // ----- ability bonus (choice-based, like Half-Elf) -----
    if let Some(selection) = choice_selection(input, HALF_ORC_ABILITY_BONUS_CHOICE_ID) {
        let ability = selection
            .strip_prefix(ABILITY_SELECTION_PREFIX)
            .unwrap_or(selection);
        let modifier = ability_modifier_for(ability_modifiers, ability);
        explanations.push(ComputationExplanation {
            id: "race.half_orc.trait_bundle.ability_bonus_target".to_owned(),
            value: modifier,
            detail: format!(
                "Half-Orc racial trait bundle — ability bonus: PF1 Core Half-Orc grants a \
                 player-chosen +2 to any one ability score \
                 ({HALF_ORC_ABILITY_BONUS_CHOICE_ID} -> {selection}); the chosen {ability} score \
                 yields modifier {modifier:+}. This is a bounded recognition record naming the \
                 chosen target on the deterministic pilot seam; the chosen score is understood \
                 to already reflect the +2 adjustment, so this record performs no arithmetic \
                 beyond surfacing the already-computed modifier"
            ),
        });
    }

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.half_orc.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Half-Orc racial trait bundle — size: PF1 Core Half-Orc is \
             {HALF_ORC_SIZE_CATEGORY} size (cr_races.lst race:half-orc SIZE:MEDIUM). This is a \
             bounded recognition record naming the Half-Orc size category on the deterministic \
             pilot seam; it contributes no numeric effect to attack rolls, AC, skill checks, \
             ability checks, or any other computed value, so it carries no fabricated \
             mechanical value (+0)"
        ),
    });

    // ----- speed -----
    explanations.push(ComputationExplanation {
        id: "race.half_orc.trait_bundle.speed".to_owned(),
        value: HALF_ORC_BASE_SPEED_FEET,
        detail: format!(
            "Half-Orc racial trait bundle — speed: PF1 Core Half-Orc has a base land speed of \
             {HALF_ORC_BASE_SPEED_FEET} ft \
             (cr_races.lst race:half-orc GAIT:WALK|{HALF_ORC_BASE_SPEED_FEET}). This is a \
             grounded recognition value carrying the Half-Orc base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    explanations.push(ComputationExplanation {
        id: "race.half_orc.trait_bundle.senses".to_owned(),
        value: HALF_ORC_DARKVISION_FEET,
        detail: format!(
            "Half-Orc racial trait bundle — senses: PF1 Core Half-Orc grants Darkvision \
             {HALF_ORC_DARKVISION_FEET} ft (cr_races.lst race:half-orc SENSE:Darkvision \
             ({HALF_ORC_DARKVISION_FEET} ft)). This is a grounded recognition value carrying \
             the Half-Orc Darkvision identity on the deterministic pilot seam; it contributes \
             no computed low-light or perception-derived effect to any chassis output"
        ),
    });

    // ----- intimidating (flat skill-bonus idiom, mirroring Dwarf Stonecunning /
    // Elf/Gnome/Half-Elf Keen Senses) -----
    explanations.push(ComputationExplanation {
        id: "race.half_orc.trait_bundle.intimidating".to_owned(),
        value: HALF_ORC_INTIMIDATING_BONUS,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   , BONUS:SKILL|Intimidate|{HALF_ORC_INTIMIDATING_BONUS}|TYPE=Racial
            "Half-Orc racial trait bundle — Intimidating: PF1 Core Half-Orc grants a flat \
             {HALF_ORC_INTIMIDATING_BONUS:+} racial bonus on Intimidate skill checks \
             (halforc_abilities_race.lst — Intimidating entry). This is a bounded recognition record \
             naming only the flat racial-bonus magnitude, mirroring the Dwarf Stonecunning / Elf \
             Keen Senses / Gnome Keen Senses / Half-Elf Keen Senses skill-bonus idiom already \
             established on this seam; it is deliberately NOT an Intimidate-check-total engine"
        ),
    });

    // Bounded honesty: five dimensions are now grounded. This replaces the
    // generic race.semantics.unverified diagnostic for Half-Orc specifically
    // and stays non-claim-blocking so the deterministic pilot still reports
    // computed evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.half_orc.bounded_semantics".to_owned(),
        message: "Half-Orc race semantics are grounded for the deterministic pilot's chosen \
                  ability-bonus target, size, speed, senses, and Intimidating trait bundle; the \
                  remaining PF1 Core Half-Orc racial trait surface remains unverified: Orc \
                  Ferocity (fighting on for one more round after being brought below 0 hit \
                  points), and weapon familiarity (orc double axe, falchion, and treating any \
                  weapon with 'orc' in its name as martial)."
            .to_owned(),
        claim_blocking: false,
    });
}
/// SD13-E2 Halfling racial trait bundle explanation seam (mirroring the
/// Dwarf/Elf/Gnome fixed-ability-pair pattern for the sixth and final
/// non-Human core race).
///
/// Surfaces six grounded PF1 Core Rulebook Halfling racial trait dimensions
/// (ability modifiers, size, speed, senses, Keen Senses, Sure-Footed) as
/// explicit `ComputationExplanation` records so the Halfling identity is
/// legible on the runtime path rather than left behind the generic
/// `race.semantics.unverified` diagnostic.
///
/// This function:
///   - runs only when `race_id == race:halfling`; every other race is
///     unaffected (Human, Dwarf, Elf, Gnome, Half-Elf, and Half-Orc keep their
///     own seams),
///   - adds no new computed mechanical contribution beyond the flat Keen
///     Senses / Sure-Footed skill-bonus magnitudes: the ability-modifiers
///     record is recognition-only (the chosen Dexterity/Strength scores are
///     understood to already reflect the fixed +2/-2 racial adjustment; no
///     arithmetic is performed on this seam), the size/senses records carry
///     the grounded source value as identity only, Keen Senses names only the
///     flat +2 Perception racial-bonus magnitude, and Sure-Footed names only
///     the flat +2 Acrobatics/Climb racial-bonus magnitude (both mirroring
///     the Dwarf Stonecunning / Elf/Gnome/Half-Elf Keen Senses / Half-Orc
///     Intimidating idiom, not a skill-check-total engine),
///   - replaces the generic `race.semantics.unverified` diagnostic with a
///     Halfling-specific `race.halfling.bounded_semantics` note naming the
///     still-unproven families explicitly (Fearless, Halfling Luck, weapon
///     familiarity, and the explicit absence of any Halfling racial bonus
///     feat),
///   - is bounded to race recognition only; it deliberately grounds no
///     Halfling class-chassis interaction, no other race, and no PF1
///     alternate ruleset.
pub(super) fn explain_halfling_race_seam(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HALFLING_RACE_ID {
        return;
    }

    // ----- ability modifiers -----
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.ability_modifiers".to_owned(),
        value: 0,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   — BONUS:STAT|DEX,CHA|2|TYPE=Racial, BONUS:STAT|STR|-2|TYPE=Racial
            "Halfling racial trait bundle — ability modifiers: PF1 Core Halfling grants a fixed \
             {HALFLING_DEX_ADJUSTMENT:+} Dexterity, {HALFLING_CHA_ADJUSTMENT:+} Charisma, and \
             {HALFLING_STR_ADJUSTMENT:+} Strength racial adjustment (verified against the real PCGen \
             corpus: core_essentials/races/halfling/halfling_abilities_race.lst's \"Halfling Racial \
             Default\" ability-score row). This is a bounded recognition record naming the fixed \
             adjustment on the deterministic pilot seam; the chosen Dexterity, Charisma, and \
             Strength scores are understood to already reflect it, so this record performs no \
             arithmetic and carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Halfling racial trait bundle — size: PF1 Core Halfling is \
             {HALFLING_SIZE_CATEGORY} size (cr_races.lst race:halfling SIZE:SMALL). This is a \
             bounded recognition record naming the Halfling size category on the deterministic \
             pilot seam; it performs no arithmetic of its own (+0) so it does not double-count \
             the real PF1 Small-size effect (+1 AC, +1 attack rolls, -1 CMB/CMD, and +4 \
             Stealth). SD-27 (decisions.md §28 defect 1) wired the AC/attack/CMB/CMD portion \
             into this engine's general combat baseline (`combat_size_modifiers`, keyed off \
             `race_size_for_race_token`): any Halfling character who reaches \
             `compute_combat_baseline`'s supported posture gets the real +1 AC/attack and \
             -1 CMB/CMD from that shared term, not from this record. The +4 Stealth portion is \
             still not applied anywhere: no Stealth skill total exists in this engine yet \
             (`compute_selected_skill_modifiers` supports only Climb, Intimidate and Swim)"
        ),
    });

    // ----- speed -----
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.speed".to_owned(),
        value: HALFLING_BASE_SPEED_FEET,
        detail: format!(
            "Halfling racial trait bundle — speed: PF1 Core Halfling has a base land speed of \
             {HALFLING_BASE_SPEED_FEET} ft \
             (cr_races.lst race:halfling GAIT:WALK|{HALFLING_BASE_SPEED_FEET}). This is a \
             grounded recognition value carrying the Halfling base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    // Bounded "no special senses" classification, mirroring Human's pattern:
    // PF1 Core Halflings have ordinary vision (no darkvision, no low-light vision).
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Halfling racial trait bundle — senses: PF1 Core Halfling grants no special \
                  senses (cr_races.lst race:halfling carries no SENSE tag; darkvision, \
                  low-light vision, and other sense bonuses are absent). This is a bounded \
                  no-effect classification record on the deterministic pilot seam; it carries \
                  no fabricated sense bonus and contributes no computed value (+0)"
            .to_owned(),
    });

    // ----- Keen Senses (flat skill-bonus idiom, mirroring Dwarf Stonecunning /
    // Elf/Gnome/Half-Elf Keen Senses / Half-Orc Intimidating) -----
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.keen_senses".to_owned(),
        value: 2,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — BONUS:SKILL|Perception|KeenSensesBonus|TYPE=Racial, BONUS:VAR|KeenSensesBonus|2
        detail: "Halfling racial trait bundle — Keen Senses: PF1 Core Halfling grants a flat +2 \
                  racial bonus on Perception skill checks \
                  (core_essentials/races/halfling/halfling_abilities_race.lst Keen Senses entry). \
                  This is a bounded recognition record naming only the flat racial-bonus magnitude \
                  on the deterministic pilot seam, mirroring the Dwarf Stonecunning / Elf Keen \
                  Senses / Gnome Keen Senses / Half-Elf Keen Senses / Half-Orc Intimidating \
                  skill-bonus idiom already established on this seam; it is deliberately NOT a \
                  Perception-check-total engine."
            .to_owned(),
    });

    // ----- Sure-Footed (flat skill-bonus idiom, mirroring Keen Senses above and
    // the Dwarf Stonecunning / Elf/Gnome/Half-Elf Keen Senses / Half-Orc
    // Intimidating skill-bonus idiom already established on this seam) -----
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.sure_footed".to_owned(),
        value: 2,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   — BONUS:SKILL|Acrobatics,Climb|2|TYPE=Racial
        detail: "Halfling racial trait bundle — Sure-Footed: PF1 Core Halfling grants a flat +2 \
                  racial bonus on Acrobatics and Climb skill checks \
                  (core_essentials/races/halfling/halfling_abilities_race.lst Sure-Footed entry). \
                  This is a bounded recognition record naming only the flat racial-bonus magnitude \
                  on the deterministic pilot seam, mirroring the Keen Senses / Dwarf Stonecunning / \
                  Elf Keen Senses / Gnome Keen Senses / Half-Elf Keen Senses / Half-Orc Intimidating \
                  skill-bonus idiom already established on this seam; it is deliberately NOT an \
                  Acrobatics/Climb-check-total engine."
            .to_owned(),
    });

    // ----- Fearless (flat racial saving-throw-bonus idiom, mirroring Dwarf
    // Hardy's flat +2 saving-throw-vs-poison/spells bonuses applied to a
    // Halfling-specific named save category) -----
    // Grounded flat +2 racial bonus on saving throws against fear
    // (core_essentials/races/halfling/halfling_abilities_race.lst Fearless
    // entry — BONUS:VAR|SaveBonus_vs_Fear|2|TYPE=Racial). This is deliberately
    // NOT a saving-throw-total engine: the recognized value names only the
    // flat racial-bonus magnitude on the deterministic pilot seam, mirroring
    // the already-grounded Dwarf Hardy flat racial saving-throw-bonus idiom
    // on this same seam shape. Halfling Luck (a separate +1 luck bonus on
    // ALL saving throws, which explicitly stacks with Fearless per the
    // corpus DESC text) remains a distinct, still-unproven family.
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.fearless".to_owned(),
        value: HALFLING_FEARLESS_SAVE_VS_FEAR_BONUS,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   — BONUS:VAR|SaveBonus_vs_Fear|{HALFLING_FEARLESS_SAVE_VS_FEAR_BONUS}|TYPE=Racial
            "Halfling racial trait bundle — Fearless: PF1 Core Halfling grants a flat \
             {HALFLING_FEARLESS_SAVE_VS_FEAR_BONUS:+} racial bonus on saving throws against fear \
             (halfling_abilities_race.lst Fearless entry). This is a bounded flat \
             saving-throw-bonus-magnitude recognition record naming the Fearless identity on the \
             deterministic pilot seam, mirroring the already-grounded Dwarf Hardy flat racial \
             saving-throw-bonus idiom; no saving-throw-total or fear-effect-resolution engine exists \
             anywhere in this codebase, so no check resolution is fabricated from this record"
        ),
    });

    // ----- Halfling Luck (flat racial saving-throw-bonus idiom, mirroring
    // Fearless above and the Dwarf Hardy flat racial saving-throw-bonus idiom
    // already established on this seam, applied to a broader "all saving
    // throws" scope rather than a single named save category) -----
    // Grounded flat +1 racial bonus on all saving throws
    // (core_essentials/races/halfling/halfling_abilities_race.lst Halfling
    // Luck entry — BONUS:VAR|Halfling_HalflingLuck_SaveBonus|1|TYPE=Racial).
    // This is deliberately NOT a saving-throw-total engine: the recognized
    // value names only the flat racial-bonus magnitude on the deterministic
    // pilot seam, mirroring the already-grounded Dwarf Hardy / Halfling
    // Fearless flat racial saving-throw-bonus idiom on this same seam shape.
    // Per the corpus DESC text, this bonus explicitly stacks with Fearless
    // (a distinct, already-grounded fear-specific save bonus).
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.halfling_luck".to_owned(),
        value: HALFLING_LUCK_ALL_SAVES_BONUS,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   —
            //   BONUS:VAR|Halfling_HalflingLuck_SaveBonus|{HALFLING_LUCK_ALL_SAVES_BONUS}|TYPE=Racial
            "Halfling racial trait bundle — Halfling Luck: PF1 Core Halfling grants a flat \
             {HALFLING_LUCK_ALL_SAVES_BONUS:+} racial bonus on all saving throws \
             (halfling_abilities_race.lst Halfling Luck entry). This is a bounded flat \
             saving-throw-bonus-magnitude recognition record naming the Halfling Luck identity on \
             the deterministic pilot seam, mirroring the already-grounded Dwarf Hardy / Halfling \
             Fearless flat racial saving-throw-bonus idiom; no saving-throw-total or \
             luck-effect-resolution engine exists anywhere in this codebase, so no check resolution \
             is fabricated from this record"
        ),
    });

    // Bounded honesty: only the eight named dimensions are grounded. This replaces
    // the generic race.semantics.unverified diagnostic for Halfling specifically
    // and stays non-claim-blocking so the deterministic pilot still reports
    // computed evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.halfling.bounded_semantics".to_owned(),
        message: "Halfling race semantics are grounded for the deterministic pilot's ability \
                  modifiers, size, speed, senses, Keen Senses (Perception bonus), \
                  Sure-Footed (Acrobatics/Climb bonus), Fearless (saving-throw-vs-fear \
                  bonus), and Halfling Luck (all-saving-throws bonus) trait bundle; the \
                  remaining PF1 Core Halfling racial trait surface remains unverified: weapon \
                  familiarity (sling and thrown weapons). PF1 core Halflings gain no racial \
                  bonus feat (unlike Human), so that family is explicitly not applicable \
                  rather than silently omitted."
            .to_owned(),
        claim_blocking: false,
    });
}
