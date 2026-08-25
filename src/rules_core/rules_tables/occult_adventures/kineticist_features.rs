//! SD-32 card 11 (T12), cycle 4 — real per-feature compute functions for
//! the Kineticist, one of the six `occult_adventures` classes sharing
//! `oa_abilities_class.lst`. Every formula below is transcribed from the
//! corpus's own already-ingested tokens
//! (`data/corpus/occult_adventures/class_feature/kineticist/*.json`), not
//! from memory of the printed rulebook.

/// `oa_abilities_class.lst:542`, `Burn`: no machine `BONUS:` token exists
/// for the burn cap (PCGen leaves it to the player to track), but the
/// record's own prose states the real formula unambiguously: "a kineticist
/// can't choose to accept burn if it would put her total number of points
/// of burn higher than 3 + her Constitution modifier". Grounded as a real
/// ability-modifier-plus-constant magnitude, not left text-only.
pub fn burn_max_points(level: u8, con: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(3 + con)
}

/// `oa_abilities_class.lst:540`, `Elemental Focus`: two independent
/// `BONUS:VAR` tokens on this record target different variables
/// (`Pool_KineticistElementalFocus|1`, a trivial flat pool grant, and
/// `KineticistLVL_Base|max(1,KineticistLVL/2)`, the class's own effective
/// level used elsewhere). Grounded the level-scaled one as the record's
/// real magnitude.
pub fn elemental_focus_level_base(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(std::cmp::max(1, i16::from(level) / 2))
}

/// `oa_abilities_class.lst:545`, `Infusion`:
/// `BONUS:VAR|Pool_KineticistInfusion|1+(KineticistLVL>=3)+(KineticistLVL>=5)+(KineticistLVL>=9)+(KineticistLVL>=11)+(KineticistLVL>=13)+(KineticistLVL>=17)+(KineticistLVL>=19)` —
/// a level-scaled step count, one additional infusion known per named
/// threshold.
pub fn infusion_pool(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let l = i16::from(level);
    let mut n = 1;
    for threshold in [3, 5, 9, 11, 13, 17, 19] {
        if l >= threshold {
            n += 1;
        }
    }
    Some(n)
}

/// `oa_abilities_class.lst:543`, `Kinetic Blast`: no `BONUS:` token at all
/// — the DESC states the flat literal directly ("unleash a kinetic blast
/// at a single target up to a range of 30 feet").
pub fn kinetic_blast_range_feet(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(30)
}

/// `oa_abilities_class.lst:541`, `Wild Talents`: the roster's own tracked
/// DC formula, `BONUS:VAR|WildTalentsDC|10+(KineticistLVL/2)` combined with
/// `BONUS:VAR|WildTalentsDC|WildTalentDCStat` and
/// `BONUS:VAR|WildTalentDCStat|CON` — the standard `10 + level/2 +
/// ability_modifier` DC shape, same as Vitalist's Steal Life / Marksman's
/// Cover Fire, this time on Constitution.
pub fn wild_talents_dc(level: u8, con: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 + i16::from(level) / 2 + con)
}

/// `oa_abilities_class.lst:551`, `Expanded Element`:
/// `BONUS:VAR|Pool_KineticistExpandedElement|1+(KineticistLVL>=15)`. `None`
/// below level 7 (the roster's own `min_level`).
pub fn expanded_element_pool(level: u8) -> Option<i16> {
    if level < 7 {
        return None;
    }
    Some(1 + i16::from(level >= 15))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burn_max_points_is_three_plus_constitution() {
        assert_eq!(burn_max_points(1, 3), Some(6));
        assert_eq!(burn_max_points(20, -1), Some(2));
        assert_eq!(burn_max_points(0, 3), None);
    }

    #[test]
    fn elemental_focus_level_base_floors_and_floors_at_one() {
        assert_eq!(elemental_focus_level_base(1), Some(1));
        assert_eq!(elemental_focus_level_base(20), Some(10));
        assert_eq!(elemental_focus_level_base(0), None);
    }

    #[test]
    fn infusion_pool_steps_at_each_named_threshold() {
        assert_eq!(infusion_pool(1), Some(1));
        assert_eq!(infusion_pool(3), Some(2));
        assert_eq!(infusion_pool(19), Some(8));
        assert_eq!(infusion_pool(20), Some(8));
        assert_eq!(infusion_pool(0), None);
    }

    #[test]
    fn kinetic_blast_range_is_a_flat_thirty_feet() {
        assert_eq!(kinetic_blast_range_feet(1), Some(30));
        assert_eq!(kinetic_blast_range_feet(0), None);
    }

    #[test]
    fn wild_talents_dc_combines_level_and_constitution() {
        assert_eq!(wild_talents_dc(20, 3), Some(23)); // 10+10+3
        assert_eq!(wild_talents_dc(0, 3), None);
    }

    #[test]
    fn expanded_element_pool_steps_at_fifteen() {
        assert_eq!(expanded_element_pool(7), Some(1));
        assert_eq!(expanded_element_pool(15), Some(2));
        assert_eq!(expanded_element_pool(6), None);
    }
}
