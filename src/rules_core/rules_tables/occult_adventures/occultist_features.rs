//! SD-32 card 11 (T12), cycle 4 — real per-feature compute functions for
//! the Occultist, one of the six `occult_adventures` classes sharing
//! `oa_abilities_class.lst`. Every formula below is transcribed from the
//! corpus's own already-ingested tokens
//! (`data/corpus/occult_adventures/class_feature/occultist/*.json`).
//!
//! `Mental Focus`'s own `BONUS:VAR` formula adds an
//! `if(OccultistMentalFocusFCB>=2,OccultistMentalFocusFCB/2,0)`
//! favored-class-bonus term this engine has no input for (see the same
//! omission documented in `mesmerist_features`'s module doc); grounded here
//! is the FCB-less base value.

/// `oa_abilities_class.lst:107`, `Focus Powers`:
/// `BONUS:VAR|OccultistFocusPower|(OccultistLVL+1)/2`.
pub fn focus_powers_count(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 1) / 2)
}

/// `oa_abilities_class.lst:107`, `Focus Powers`:
/// `BONUS:VAR|OccultistFocusPowerDC|10+OccultistLVL/2+INT`.
pub fn focus_powers_dc(level: u8, int: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 + i16::from(level) / 2 + int)
}

/// `oa_abilities_class.lst:108`, `Implements`:
/// `BONUS:VAR|OccultistImplementSchool|2+((OccultistLVL+2)/4)`.
pub fn implements_school_count(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 + (i16::from(level) + 2) / 4)
}

/// `oa_abilities_class.lst:110`, `Mental Focus`:
/// `BONUS:VAR|OccultistMentalFocus|OccultistLVL+INT+if(...FCB...)` (FCB
/// term dropped, see module doc).
pub fn mental_focus(level: u8, int: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) + int)
}

/// `oa_abilities_class.lst:111`, `Magic Item Skill`:
/// `BONUS:SKILL|Use Magic Device|OccultistLVL/2`.
pub fn magic_item_skill_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// `oa_abilities_class.lst:116`, `Outside Contact`:
/// `BONUS:VAR|OccultistOutsiderNum|1+(OccultistLVL-8)/4`.
pub fn outside_contact_count(level: u8) -> Option<i16> {
    if level < 8 {
        return None;
    }
    Some(1 + (i16::from(level) - 8) / 4)
}

/// `oa_abilities_class.lst:117`, `Binding Circles`:
/// `BONUS:VAR|OccultistCircleDC|10+OccultistLVL/2+INT`.
pub fn binding_circles_dc(level: u8, int: i16) -> Option<i16> {
    if level < 12 {
        return None;
    }
    Some(10 + i16::from(level) / 2 + int)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_powers_count_scales_with_level() {
        assert_eq!(focus_powers_count(1), Some(1));
        assert_eq!(focus_powers_count(20), Some(10));
        assert_eq!(focus_powers_count(0), None);
    }

    #[test]
    fn focus_powers_dc_combines_level_and_intelligence() {
        assert_eq!(focus_powers_dc(20, 3), Some(23));
    }

    #[test]
    fn implements_school_count_scales_with_level() {
        assert_eq!(implements_school_count(1), Some(2));
        assert_eq!(implements_school_count(20), Some(7));
    }

    #[test]
    fn mental_focus_combines_level_and_intelligence() {
        assert_eq!(mental_focus(20, 3), Some(23));
        assert_eq!(mental_focus(0, 3), None);
    }

    #[test]
    fn magic_item_skill_bonus_gates_at_level_two() {
        assert_eq!(magic_item_skill_bonus(2), Some(1));
        assert_eq!(magic_item_skill_bonus(1), None);
    }

    #[test]
    fn outside_contact_count_gates_at_level_eight() {
        assert_eq!(outside_contact_count(8), Some(1));
        assert_eq!(outside_contact_count(7), None);
    }

    #[test]
    fn binding_circles_dc_gates_at_level_twelve() {
        assert_eq!(binding_circles_dc(12, 3), Some(19));
        assert_eq!(binding_circles_dc(11, 3), None);
    }
}
