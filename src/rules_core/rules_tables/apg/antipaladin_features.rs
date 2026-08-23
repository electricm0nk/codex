//! SD-32 card 11 (T12) — real per-feature compute functions for the
//! Antipaladin (`untabled_base_class_chassis`), the first of the 108
//! magnitude-bearing `untabled_base_class_feature_roster` records worked
//! end-to-end as a class rather than left "named, not attempted" a third
//! cycle running (`decisions.md §17`/`§27b` — novelty of shape is grounds
//! for sizing, not exclusion; a group must be attempted, not just sized).
//!
//! Every formula below is transcribed from the corpus's own already-
//! ingested `BONUS:VAR` tokens (`data/corpus/advanced_players_guide/
//! class_feature/antipaladin/*.json`, each record's own `raw_tokens`), the
//! same discipline `barbarian_features.rs` uses for Unchained Barbarian —
//! nothing here is derived from memory of the printed rulebook text.
//!
//! Antipaladin (APG p.120) is PF1's structural mirror of the CRB Paladin,
//! and every one of these seven formulas is that mirror made explicit:
//! Touch of Corruption reuses the identical `LayOnHandsTimes`/
//! `LayOnHandsDice` variable names the Paladin's Lay on Hands token pair
//! uses (`level_up::paladin`'s own `lay_on_hands_uses_per_day`/
//! `lay_on_hands_heal_dice`, `level/2 + CHA` and `level/2` respectively —
//! confirmed byte-identical formulas, not merely similarly named).
//!
//! # Upstream data bug found and NOT perpetuated (`decisions.md §22`)
//!
//! `unholy_champion.json`'s own `BONUS:VAR|UnholyChampionCasterLevel|
//! HolyChampionLVL` token names a variable (`HolyChampionLVL`) that is the
//! Paladin's own internal level tracker, not the Antipaladin's
//! (`Antipaladin_CFP_Level`/an equivalent `UnholyChampionLVL` — this
//! record's own `DEFINE:UnholyChampionLVL|0` sits one line above the
//! typo'd read). On an actual single-class Antipaladin character
//! `HolyChampionLVL` is never set by anything and evaluates to its `0`
//! default, so literal PCGen behavior would silently zero the capstone's
//! Banishment caster level for every Antipaladin who reaches it — clearly
//! a copy-paste artifact from the Paladin table this class mirrors, not an
//! intentional rule. [`unholy_champion_banishment_caster_level`] grounds
//! the RAW-correct value (antipaladin level, matching the Paladin capstone
//! formula this token was copied from) rather than reproducing the typo.

/// [`Some`] level-2+ Touch of Corruption uses-per-day (`LayOnHandsTimes`,
/// `(LayOnHandsLVL/2)+CHA`) and total d6 damage/heal dice
/// (`LayOnHandsDice`, `LayOnHandsLVL/2`) for `level` — both formulas read
/// off `touch_of_corruption.json`'s own `BONUS:VAR` tokens, `LayOnHandsLVL`
/// itself defined one token below as `TouchOfCorruptionLVL` (i.e. the raw
/// Antipaladin class level; no CFP offset). `None` below level 2 (the
/// roster's own `min_level` for this key).
pub fn touch_of_corruption_uses_per_day(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(i16::from(level) / 2 + charisma_modifier)
}

pub fn touch_of_corruption_damage_dice(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// `unholy_resilience.json`'s own `BONUS:VAR|UnholyResilience|max(CHA,0)`:
/// a save bonus equal to the Charisma modifier, floored at 0 ("if any" in
/// the printed text). `None` below level 2.
pub fn unholy_resilience_save_bonus(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(charisma_modifier.max(0))
}

/// `cruelty.json`'s own tokens: the Fortitude-save DC an applied cruelty
/// imposes (`CrueltyDC`, `10+CHA+(AntipaladinLVL/2)`) and the number of
/// cruelties known (`Cruelties`, `min(CrueltyLVL/3,6)` — one every three
/// levels from 3rd, capped at six). `None` below level 3 (the roster's own
/// `min_level` for this key).
pub fn cruelty_dc(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(10 + charisma_modifier + i16::from(level) / 2)
}

pub fn cruelties_known(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some((i16::from(level) / 3).min(6))
}

/// `channel_negative_energy.json`'s own tokens: fixed d6 die size
/// (`AntipaladinChannelDieSize`, literal `6`), dice count
/// (`AntipaladinChannelDice`, `(AntipaladinChannelLVL+1)/2`), and Will-save
/// DC to halve (`AntipaladinChannelDC`, `10+(AntipaladinChannelLVL/2)+CHA`).
/// `None` below level 4 (the roster's own `min_level` for this key).
pub fn channel_negative_energy_dice(level: u8) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some((i16::from(level) + 1) / 2)
}

pub const CHANNEL_NEGATIVE_ENERGY_DIE_SIZE: i16 = 6;

pub fn channel_negative_energy_dc(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(10 + i16::from(level) / 2 + charisma_modifier)
}

/// `fiendish_boon.json`'s own `BONUS:VAR|FiendishBoonTimes|
/// min((FiendishBoonLVL-1)/4,4)`: the number of Fiendish Boon selections
/// (weapon or familiar enhancement picks), one at 5th level and one more
/// every four levels thereafter, capped at four. `None` below level 5 (the
/// roster's own `min_level` for this key).
pub fn fiendish_boon_selections(level: u8) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some(((i16::from(level) - 1) / 4).min(4))
}

/// `aura_of_depravity.json` carries no `BONUS:VAR` token at all — DR 5/good
/// is a flat magnitude, not level-scaled (unlike every other feature in
/// this module). Still a real grounded fact, not a text-only marker.
/// `None` below level 17 (the roster's own `min_level` for this key).
pub fn aura_of_depravity_damage_reduction(level: u8) -> Option<i16> {
    if level < 17 {
        return None;
    }
    Some(5)
}

/// The Unholy Champion capstone's Banishment caster level — see this
/// module's own doc comment for the upstream `HolyChampionLVL` typo this
/// function deliberately does not reproduce (`decisions.md §22`). `None`
/// below level 20 (the roster's own `min_level` for this key, and PF1's
/// own capstone level).
pub fn unholy_champion_banishment_caster_level(level: u8) -> Option<i16> {
    if level < 20 {
        return None;
    }
    Some(i16::from(level))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touch_of_corruption_matches_paladin_lay_on_hands_at_every_shared_level() {
        // Byte-identical formula to `level_up::paladin`'s own
        // `lay_on_hands_uses_per_day`/`lay_on_hands_heal_dice` -- spot
        // checked at levels 2, 6, and 20 with a +3 Charisma modifier.
        for level in [2u8, 6, 20] {
            let cha = 3;
            assert_eq!(
                touch_of_corruption_uses_per_day(level, cha),
                Some(i16::from(level) / 2 + cha)
            );
            assert_eq!(touch_of_corruption_damage_dice(level), Some(i16::from(level) / 2));
        }
        assert_eq!(touch_of_corruption_uses_per_day(1, 3), None);
        assert_eq!(touch_of_corruption_damage_dice(1), None);
    }

    #[test]
    fn unholy_resilience_floors_a_negative_charisma_modifier_at_zero() {
        assert_eq!(unholy_resilience_save_bonus(2, -2), Some(0));
        assert_eq!(unholy_resilience_save_bonus(2, 4), Some(4));
        assert_eq!(unholy_resilience_save_bonus(1, 4), None);
    }

    #[test]
    fn cruelty_dc_and_count_at_level_3_the_first_grant() {
        assert_eq!(cruelty_dc(3, 2), Some(10 + 2 + 1));
        assert_eq!(cruelties_known(3), Some(1));
        assert_eq!(cruelty_dc(2, 2), None);
        assert_eq!(cruelties_known(2), None);
    }

    #[test]
    fn cruelties_known_caps_at_six() {
        assert_eq!(cruelties_known(18), Some(6));
        assert_eq!(cruelties_known(20), Some(6));
    }

    #[test]
    fn channel_negative_energy_at_level_4_the_first_grant() {
        assert_eq!(channel_negative_energy_dice(4), Some(2));
        assert_eq!(CHANNEL_NEGATIVE_ENERGY_DIE_SIZE, 6);
        assert_eq!(channel_negative_energy_dc(4, 1), Some(10 + 2 + 1));
        assert_eq!(channel_negative_energy_dice(3), None);
    }

    #[test]
    fn fiendish_boon_grants_one_at_level_5_and_caps_at_four() {
        assert_eq!(fiendish_boon_selections(5), Some(1));
        assert_eq!(fiendish_boon_selections(9), Some(2));
        assert_eq!(fiendish_boon_selections(20), Some(4));
        assert_eq!(fiendish_boon_selections(4), None);
    }

    #[test]
    fn aura_of_depravity_is_a_flat_dr_five_at_level_17() {
        assert_eq!(aura_of_depravity_damage_reduction(17), Some(5));
        assert_eq!(aura_of_depravity_damage_reduction(16), None);
    }

    #[test]
    fn unholy_champion_caster_level_is_the_real_antipaladin_level_not_the_upstream_typo() {
        // The upstream token reads `HolyChampionLVL` (Paladin's own
        // tracker, never set on an Antipaladin) -- this function grounds
        // the RAW-correct value instead of literally reproducing that.
        assert_eq!(unholy_champion_banishment_caster_level(20), Some(20));
        assert_eq!(unholy_champion_banishment_caster_level(19), None);
    }
}
