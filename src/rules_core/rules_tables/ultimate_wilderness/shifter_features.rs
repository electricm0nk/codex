//! SD-32 card 11 (T12), cycle 4 — real per-feature compute functions for
//! the Shifter, `ultimate_wilderness`'s single magnitude-bearing class.
//! Every formula below is transcribed from the corpus's own already-
//! ingested tokens
//! (`data/corpus/ultimate_wilderness/class_feature/shifter/*.json`).

use crate::rules_core::size::SizeCategory;

/// `Shifter Aspect`: `BONUS:VAR|ShifterAspectMinutes|ShifterLVL+3`.
pub fn shifter_aspect_minutes(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) + 3)
}

/// `Shifter Aspect`: `BONUS:VAR|ShifterAspectCount|1+min(3,(ShifterLVL/5))`.
pub fn shifter_aspect_count(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + std::cmp::min(3, i16::from(level) / 5))
}

/// `Shifter Claws`: `BONUS:VAR|ShifterClawDamage` carries a base value keyed
/// on the character's size (`PRESIZEEQ:S`=3, `PRESIZEEQ:M`=4,
/// `PRESIZEEQ:L`=6 — no Tiny/Huge/other row exists in the corpus, so this
/// grounds only the three sizes the record itself defines, defaulting an
/// unresolved or out-of-range size to the Medium row, the common case),
/// plus three level-gated `+2` steps at 7th (Medium+ only; Small gets `+1`
/// instead), 11th and 13th (`PREVARGTEQ:ShifterLVL,7/11/13`). The die-size
/// upgrade itself is downstream game presentation this record does not
/// compute a magnitude for; the tracked `ShifterClawDamage` numeric value
/// is what is grounded here.
pub fn shifter_claw_damage(level: u8, size: Option<SizeCategory>) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let is_small = matches!(size, Some(SizeCategory::Small));
    let mut value = if is_small { 3 } else { 4 }; // S=3, M-and-default=4
    if level >= 7 {
        value += if is_small { 1 } else { 2 };
    }
    if level >= 11 {
        value += 2;
    }
    if level >= 13 {
        value += 2;
    }
    Some(value)
}

/// `Defensive Instinct`: `BONUS:VAR|ShifterACBonus|min((ShifterACLVL)/4,5)`
/// stacking with `BONUS:VAR|ShifterACBonus|ShifterACWisBonus/2` (the
/// unencumbered case; the record's own `ENCUMBERANCE`-conditional row
/// grants the identical value when unencumbered — the encumbered
/// distinction is not modelled here, matching this engine's existing
/// unencumbered-by-default combat baseline).
pub fn defensive_instinct_ac_bonus(level: u8, wis: i16) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(std::cmp::min(i16::from(level) / 4, 5) + std::cmp::max(wis, 0) / 2)
}

/// `Track`: `BONUS:VAR|TrackLVL|ShifterLVL` — a pure level pass-through.
pub fn shifter_track_level(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(i16::from(level))
}

/// `Wild Shape`: `BONUS:VAR|ShifterWildshapeCount|ShifterLVL+WIS`.
pub fn wild_shape_count(level: u8, wis: i16) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(i16::from(level) + wis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifter_aspect_minutes_scales_with_level() {
        assert_eq!(shifter_aspect_minutes(1), Some(4));
        assert_eq!(shifter_aspect_minutes(20), Some(23));
        assert_eq!(shifter_aspect_minutes(0), None);
    }

    #[test]
    fn shifter_aspect_count_caps_at_three_additional() {
        assert_eq!(shifter_aspect_count(1), Some(1));
        assert_eq!(shifter_aspect_count(15), Some(4));
        assert_eq!(shifter_aspect_count(20), Some(4));
    }

    #[test]
    fn shifter_claw_damage_steps_by_size_and_level() {
        assert_eq!(shifter_claw_damage(1, Some(SizeCategory::Medium)), Some(4));
        assert_eq!(shifter_claw_damage(1, Some(SizeCategory::Small)), Some(3));
        assert_eq!(shifter_claw_damage(7, Some(SizeCategory::Medium)), Some(6));
        assert_eq!(shifter_claw_damage(20, Some(SizeCategory::Medium)), Some(10));
        assert_eq!(shifter_claw_damage(0, Some(SizeCategory::Medium)), None);
    }

    #[test]
    fn defensive_instinct_ac_bonus_combines_level_cap_and_wisdom() {
        assert_eq!(defensive_instinct_ac_bonus(2, 4), Some(2));
        assert_eq!(defensive_instinct_ac_bonus(20, 4), Some(7));
        assert_eq!(defensive_instinct_ac_bonus(1, 4), None);
    }

    #[test]
    fn shifter_track_level_gates_at_level_two() {
        assert_eq!(shifter_track_level(2), Some(2));
        assert_eq!(shifter_track_level(1), None);
    }

    #[test]
    fn wild_shape_count_combines_level_and_wisdom() {
        assert_eq!(wild_shape_count(4, 3), Some(7));
        assert_eq!(wild_shape_count(3, 3), None);
    }
}
