//! SD-32 card 11 (T12), cycle 3 — real per-feature compute functions for
//! the Aegis, the first of this cycle's four `ultimate_psionics` classes
//! (`decisions.md §17`/`§27b`: novelty of shape is grounds for sizing, not
//! exclusion; `up_abilities_class.lst`/`up_classes.lst` is the same source
//! file pair the prior cycle already read for Cryptic/Dread/Marksman/
//! Psychic Warrior/Soulknife).
//!
//! Every formula below is transcribed from the corpus's own already-
//! ingested `BONUS:VAR`/`BONUS:ABILITYPOOL` tokens
//! (`data/corpus/ultimate_psionics/class_feature/aegis/*.json`, each
//! record's own `raw_tokens`), not from memory of the printed rulebook.
//! Aegis's prime stat (`AegisPrimeStat`, `up_classes.lst:20`) is
//! Intelligence for every one of the two records below that read it.
//! `AegisCL`/`AegisDRLVL` (`up_classes.lst:23,25`) both resolve to plain
//! class level for a base (non-archetype) Aegis, so this module takes
//! `level` alone for the level term.

/// `up_abilities_class.lst:24`, `Astral Repair`:
/// `BONUS:VAR|AstralRepairHP|2` — a flat 2 hit points restored, not
/// level-scaled. `None` below level 1 (the roster's own `min_level`).
pub fn astral_repair_hp(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2)
}

/// `up_abilities_class.lst:29`, `Damage Reduction`:
/// `BONUS:VAR|AegisDR|floor((AegisDRLVL+4)/3)`, `AegisDRLVL = AegisLVL`
/// (`up_classes.lst:25`). `None` below level 1 (the roster's own
/// `min_level`).
pub fn damage_reduction(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 4) / 3)
}

/// `up_abilities_class.lst:25`, `Form Astral Suit`:
/// `BONUS:VAR|CustomPoints|2+AegisCL` combined with the second
/// `BONUS:VAR|CustomPoints|floor((AegisCL+1)/5)` row (`up_classes.lst:24`
/// — PCGen sums stacking `BONUS:VAR` rows on the same variable);
/// `AegisCL = AegisLVL` for a base Aegis. `None` below level 1.
pub fn form_astral_suit_custom_points(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let lvl = i16::from(level);
    Some(2 + lvl + (lvl + 1) / 5)
}

/// `up_abilities_class.lst:26`, `Craftsman`:
/// `BONUS:VAR|CraftsmanBonus|floor((AegisLVL+2)/4)`. `None` below level 2
/// (the roster's own `min_level`).
pub fn craftsman_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some((i16::from(level) + 2) / 4)
}

/// `up_abilities_class.lst:31`, `Reconfigure`: the roster's own tracked var
/// is `ReconfigureTimes` (`BONUS:VAR|ReconfigureTimes|floor((AegisLVL-1)/2)`)
/// — a sibling `ReconfigurePoints` token (`AegisPrimeStat`) exists on the
/// same record but is not the roster's tracked var, so it is not grounded
/// here (same "ground exactly the var the roster names" discipline the
/// prior cycle documented for Soulknife/Cryptic). `None` below level 3
/// (the roster's own `min_level`).
pub fn reconfigure_times_per_day(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some((i16::from(level) - 1) / 2)
}

/// `up_abilities_class.lst:32`, `Augment Suit`: the roster's own tracked
/// var is `AugmentSuitDuration` (`BONUS:VAR|AugmentSuitDuration|
/// AegisPrimeStat`) — a pure Intelligence-modifier value with no level
/// term at all (the same shape Dread's Shadow Twin used). `None` below
/// level 4 (the roster's own `min_level`).
pub fn augment_suit_duration_rounds(level: u8, intelligence: i16) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(intelligence)
}

/// `up_abilities_class.lst:33`, `Cannibalize Suit`:
/// `BONUS:VAR|CannibalizeTimes|floor((AegisLVL-10)/2)`. `None` below
/// level 12 (the roster's own `min_level`).
pub fn cannibalize_suit_times_per_day(level: u8) -> Option<i16> {
    if level < 12 {
        return None;
    }
    Some((i16::from(level) - 10) / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn astral_repair_is_a_flat_two_hit_points() {
        assert_eq!(astral_repair_hp(1), Some(2));
        assert_eq!(astral_repair_hp(20), Some(2));
        assert_eq!(astral_repair_hp(0), None);
    }

    #[test]
    fn damage_reduction_steps_every_three_levels_from_level_one() {
        assert_eq!(damage_reduction(1), Some(1));
        assert_eq!(damage_reduction(2), Some(2));
        assert_eq!(damage_reduction(20), Some(8));
        assert_eq!(damage_reduction(0), None);
    }

    #[test]
    fn form_astral_suit_custom_points_combines_two_stacking_bonus_var_rows() {
        assert_eq!(form_astral_suit_custom_points(1), Some(2 + 1 + 0));
        assert_eq!(form_astral_suit_custom_points(20), Some(2 + 20 + 4));
        assert_eq!(form_astral_suit_custom_points(0), None);
    }

    #[test]
    fn craftsman_bonus_steps_every_four_levels_from_level_two() {
        assert_eq!(craftsman_bonus(2), Some(1));
        assert_eq!(craftsman_bonus(20), Some(5));
        assert_eq!(craftsman_bonus(1), None);
    }

    #[test]
    fn reconfigure_times_steps_every_two_levels_from_level_three() {
        assert_eq!(reconfigure_times_per_day(3), Some(1));
        assert_eq!(reconfigure_times_per_day(20), Some(9));
        assert_eq!(reconfigure_times_per_day(2), None);
    }

    #[test]
    fn augment_suit_duration_reads_intelligence_only_no_level_term() {
        assert_eq!(augment_suit_duration_rounds(4, 3), Some(3));
        assert_eq!(augment_suit_duration_rounds(20, 5), Some(5));
        assert_eq!(augment_suit_duration_rounds(3, 3), None);
    }

    #[test]
    fn cannibalize_suit_times_is_gated_to_the_capstone() {
        assert_eq!(cannibalize_suit_times_per_day(12), Some(1));
        assert_eq!(cannibalize_suit_times_per_day(20), Some(5));
        assert_eq!(cannibalize_suit_times_per_day(11), None);
    }
}
