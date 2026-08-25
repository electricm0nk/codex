//! SD-32 card 11 (T12) — real per-feature compute functions for the
//! Soulknife (`untabled_base_class_chassis`), a sixth magnitude-bearing
//! `untabled_base_class_feature_roster` group worked end-to-end as a class
//! (`decisions.md §17`/`§27b` — novelty of shape is grounds for sizing,
//! not exclusion).
//!
//! Every formula below is transcribed from the corpus's own already-
//! ingested tokens (`data/corpus/ultimate_psionics/class_feature/
//! soulknife/*.json`, each record's own `raw_tokens`, sourced from
//! `up_abilities_class.lst` — the roster's own `source_file` for every
//! Soulknife record).

/// `up_abilities_class.lst:728`, `Form Mind Blade`: this row's own `DESC:`
/// carries no numeric substitution — the mind blade's only magnitude
/// output is `BONUS:VAR|MndBladeLVL|SoulknifeLVL`, the tracked level fed
/// to `enhanced_mind_blade_max_enhancement_bonus` below. `None` below
/// level 1 (the roster's own `min_level` for this key).
pub fn form_mind_blade_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst:738`, `Enhanced Mind Blade`:
/// `BONUS:VAR|MndBladeMxEnhancement|min((EnhancedMndBladeLVL/3),5)`,
/// `EnhancedMndBladeLVL = MndBladeLVL = SoulknifeLVL` — the maximum
/// enhancement bonus a mind blade can carry. `None` below level 1 (the
/// roster's own `min_level` for this key; the value itself is `0` until
/// level 3).
pub fn enhanced_mind_blade_max_enhancement_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) / 3).min(5))
}

/// `up_abilities_class.lst:739`, `Psychic Strike`:
/// `BONUS:VAR|PsychicStrikeDieType|8` — a flat d8 die size, not
/// level-scaled (the dice *count*, `PsychicStrikeDice =
/// (SoulknifeLVL+1)/4`, is this row's other token but not the roster's
/// tracked var). `None` below level 3 (the roster's own `min_level` for
/// this key).
pub fn psychic_strike_die_size(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(8)
}

/// `up_abilities_class.lst:740`, `Quick Draw`:
/// `BONUS:VAR|QuickDrawTimes|1` — a flat one manifestation per round, not
/// level-scaled. `None` below level 5 (the roster's own `min_level` for
/// this key).
pub fn quick_draw_uses_per_round(level: u8) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_mind_blade_level_equals_class_level() {
        assert_eq!(form_mind_blade_level(1), Some(1));
        assert_eq!(form_mind_blade_level(20), Some(20));
        assert_eq!(form_mind_blade_level(0), None);
    }

    #[test]
    fn enhanced_mind_blade_max_bonus_caps_at_five() {
        assert_eq!(enhanced_mind_blade_max_enhancement_bonus(1), Some(0));
        assert_eq!(enhanced_mind_blade_max_enhancement_bonus(9), Some(3));
        assert_eq!(enhanced_mind_blade_max_enhancement_bonus(20), Some(5));
    }

    #[test]
    fn psychic_strike_die_size_is_a_flat_eight() {
        assert_eq!(psychic_strike_die_size(3), Some(8));
        assert_eq!(psychic_strike_die_size(2), None);
    }

    #[test]
    fn quick_draw_is_a_flat_one_per_round_from_level_five() {
        assert_eq!(quick_draw_uses_per_round(5), Some(1));
        assert_eq!(quick_draw_uses_per_round(4), None);
    }
}
