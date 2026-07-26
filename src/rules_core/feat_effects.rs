//! Feat-effects engine (v0.6 alpha swarm item 17, un-deferred by operator
//! directive 2026-07-24: "easy enough, you need to create a feats engine").
//!
//! Before this file, no feat except the three hardcoded into the fixed
//! creation-time loadout (Power Attack, Dodge, Weapon Focus) had any
//! computed mechanical effect anywhere in this crate -- QA verified
//! concretely (not reasoned abstractly): a Fighter with Toughness added to
//! `selected_feats` stayed at HP 12 instead of the correct 15, with zero
//! explanations mentioning it (`risks-and-open-questions.md` item 17).
//! Every other selectable feat in the 185-record CRB catalog was purely
//! recorded data with zero computed consequence.
//!
//! Deliberately scoped to exactly one feat for the first slice, not a
//! general resolver for all 185 (operator directive: land one real feat,
//! prove the pattern, then widen). [`hp_bonus_from_feats`] is shaped to
//! generalize without rework -- future feats extend the same "scan
//! `selected_feats`, sum real effects" idiom, one `if`/match arm at a time,
//! the same way this crate's other additive computations
//! (`skill_allocation.rs`'s class-skill lists, `durability.rs`'s per-class
//! hit-die table) started with one recognized case and grew.
//!
//! Widened (v0.6 alpha swarm, 2026-07-24) to the three CRB save-boosting
//! feats -- Great Fortitude, Iron Will, Lightning Reflexes -- proving the
//! pattern extends past a single feat. Deliberately NOT widened to Skill
//! Focus in the same pass: its own catalog record's `effect` qualifiers
//! (`["SKILL", "%LIST", "3", "TYPE=SkillFocus"]`) carry a `%LIST`
//! placeholder, PCGen's own marker for a player-chosen target -- unlike the
//! three save feats (each targets one fixed, named save) or Toughness (no
//! target at all), which skill Skill Focus boosts is a per-character choice
//! `selected_feats` has no slot to record (the same shape of gap Weapon
//! Focus's own `"feat:weapon_focus:weapon:longsword"` compound id works
//! around for the one hardcoded fixed-loadout case, but no general
//! chosen-target mechanism exists for a catalog-picked feat). Left for a
//! future slice, not guessed at here.

/// The real PF1 Core Rulebook feat catalog's key for Toughness
/// (`rules_tables::crb::feat_data::general`'s own
/// `FeatTableEntry { key: "Toughness", ... }` record) -- NOT a synthetic
/// `"feat:toughness"` id, and verified against the real selection pipeline
/// rather than assumed: `FeatCatalogEntryDto.key` (`feat_catalog.rs`) passes
/// the catalog key through verbatim with no transformation, and the
/// frontend's `handleAddFeat`/`handleAddFeatLevelUp` (`CharacterSheet.tsx`)
/// send `entry.key` directly as `addFeatSelection`'s `featId`, which
/// `apply_add_feat_selection` (`pf1_adapter.rs`) pushes onto
/// `selected_feats` unmodified. This exact string, not the `"feat:<name>"`
/// convention the three hardcoded fixed-loadout feats use (a separate,
/// bespoke id space that never runs through the catalog at all), is what a
/// real user's `selected_feats` entry actually contains after picking
/// Toughness through the live Feat picker.
const TOUGHNESS_FEAT_KEY: &str = "Toughness";

/// The real PF1 Core Rulebook Toughness feat benefit: a flat +3 hit points
/// ("Benefit: You gain +3 hit points.") -- not level-scaled, unlike the
/// D&D 3.5 predecessor version of this feat. Verified against the feat's
/// own catalog record (`feat_data/general.rs`): `FeatEffectBonus`'s
/// `qualifiers` cite `"HP"`/`"CURRENTMAX"`/`"max(3,TL)"`, i.e. the PCGen
/// corpus source itself already encodes this feat as a flat +3 (the
/// `max(3,TL)` qualifier is PCGen's own templating for a level-scaling
/// variant that never actually engages at the levels this crate computes;
/// `TL` -- total feat-granting levels -- never exceeds 1 for any class/
/// level this crate reaches `Computed` for today).
const TOUGHNESS_HP_BONUS: i16 = 3;

/// Sums the real, computed hit-point bonus from every feat effect this
/// engine currently grounds, across a character's `selected_feats`. Callers
/// add this to whatever base max-HP total they already have (e.g.
/// `durability::compute_max_hp`'s result) -- this function only contributes
/// the feat-driven delta, it is not itself a full max-HP computation.
/// Returns `0` (not a fabricated value) when no grounded feat is present,
/// or for any feat this engine does not yet ground.
pub fn hp_bonus_from_feats(selected_feats: &[String]) -> i16 {
    let mut bonus = 0;
    if selected_feats.iter().any(|feat| feat == TOUGHNESS_FEAT_KEY) {
        bonus += TOUGHNESS_HP_BONUS;
    }
    bonus
}

/// The real PF1 Core Rulebook catalog keys for the three save-boosting
/// feats this engine grounds, verified against `feat_data/general.rs`
/// exactly the same way `TOUGHNESS_FEAT_KEY` was (`FeatTableEntry.key`,
/// passed through the real selection pipeline unmodified).
const GREAT_FORTITUDE_FEAT_KEY: &str = "Great Fortitude";
const IRON_WILL_FEAT_KEY: &str = "Iron Will";
const LIGHTNING_REFLEXES_FEAT_KEY: &str = "Lightning Reflexes";

/// The real PF1 Core Rulebook benefit shared by all three: a flat +2 to
/// exactly one named save. Verified against each feat's own catalog
/// record: Great Fortitude carries `["SAVE", "Fortitude", "2"]`, Iron Will
/// `["SAVE", "Will", "2"]`, Lightning Reflexes `["SAVE", "Reflex", "2"]` --
/// same magnitude, different target save, no level-scaling or other
/// qualifier on any of the three.
const SAVE_FEAT_BONUS: i16 = 2;

/// One character's feat-derived bonus to each of the three saves. Callers
/// add each field to whatever base total they already have for that save
/// (e.g. `compute_total_saves`'s own Fortitude/Reflex/Will totals) -- this
/// struct only carries the feat-driven delta, not a full save computation.
/// Deliberately not `pilot_compute::BaseSaves` (a distinct, local type) so
/// this module stays a dependency-free leaf, importing nothing from
/// `pilot_compute.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SaveBonusesFromFeats {
    pub fortitude: i16,
    pub reflex: i16,
    pub will: i16,
}

/// Sums the real, computed save bonus from every grounded save-boosting
/// feat, across a character's `selected_feats`. Each of the three fields
/// is `0` (not fabricated) when its feat is absent, or when
/// `selected_feats` carries a feat this engine does not yet ground.
pub fn save_bonuses_from_feats(selected_feats: &[String]) -> SaveBonusesFromFeats {
    SaveBonusesFromFeats {
        fortitude: if selected_feats.iter().any(|feat| feat == GREAT_FORTITUDE_FEAT_KEY) {
            SAVE_FEAT_BONUS
        } else {
            0
        },
        reflex: if selected_feats.iter().any(|feat| feat == LIGHTNING_REFLEXES_FEAT_KEY) {
            SAVE_FEAT_BONUS
        } else {
            0
        },
        will: if selected_feats.iter().any(|feat| feat == IRON_WILL_FEAT_KEY) {
            SAVE_FEAT_BONUS
        } else {
            0
        },
    }
}

/// The real PF1 Core Rulebook catalog keys for the three skill-boosting
/// feats this engine grounds, verified against `feat_data/general.rs`
/// (`Athletic`/`Persuasive`) and `feat_data/combat.rs`
/// (`Intimidating Prowess`) exactly the same way `TOUGHNESS_FEAT_KEY` was
/// (`FeatTableEntry.key`, passed through the real selection pipeline
/// unmodified).
const ATHLETIC_FEAT_KEY: &str = "Athletic";
const PERSUASIVE_FEAT_KEY: &str = "Persuasive";
const INTIMIDATING_PROWESS_FEAT_KEY: &str = "Intimidating Prowess";

/// The real PF1 Core Rulebook benefit shared by Athletic and Persuasive: a
/// flat +2 to each of two named skills ("You get a +2 bonus on Climb and
/// Swim checks" / "...on Diplomacy and Intimidate checks"). Verified against
/// each feat's own catalog record: Athletic carries
/// `["SKILL","Climb","if(skillinfo(\"TOTALRANK\",\"Climb\")>=10,4,2)"]` (and
/// the same on Swim), Persuasive the same shape on Diplomacy/Intimidate. The
/// corpus's `if(TOTALRANK>=10,4,2)` conditional grants +4 only at 10+ ranks in
/// that skill; the deterministic selected-skill posture pins skill rank at 1
/// (`pilot_compute::SELECTED_SKILL_RANK`, and ranks never exceed character
/// level), so `TOTALRANK>=10` is provably false for every character this crate
/// reaches `Computed` for -- the value is +2, not a fabricated simplification,
/// exactly the way `TOUGHNESS_HP_BONUS`'s own `max(3,TL)` resolves to a flat 3
/// at supported levels. The +4 tier is deferred until the engine computes a
/// character with 10+ ranks (well beyond current coverage).
const TWO_SKILL_FEAT_BONUS: i16 = 2;

/// One character's feat-derived bonus to each of the three skills the engine
/// computes today (`pilot_compute::SelectedSkillModifiers`'s own
/// Climb/Intimidate/Swim). Callers add each field to whatever base total they
/// already have for that skill -- this struct only carries the feat-driven
/// delta, not a full skill-modifier computation. Deliberately shaped to those
/// exact three skills (not a general per-skill map) so every field is a value
/// the engine actually consumes -- no unwired half-effects. Persuasive's real
/// +2 Diplomacy half is intentionally absent: Diplomacy is not a computed
/// skill, so grounding it here would be a bonus with no live consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkillBonusesFromFeats {
    pub climb: i16,
    pub intimidate: i16,
    pub swim: i16,
}

/// Sums the real, computed skill bonus from every grounded skill-boosting
/// feat, across a character's `selected_feats`. Each field is `0` (not
/// fabricated) when no feat targeting that skill is present.
///
/// `strength_modifier` is the character's real Strength ability modifier,
/// consumed only by Intimidating Prowess (real CRB benefit: "Add your Strength
/// modifier to Intimidate skill checks in addition to your Charisma modifier",
/// corpus `["SKILL","Intimidate","STR"]`). It is threaded as a plain scalar,
/// not as a `pilot_compute::AbilityModifiers`, so this module stays the same
/// dependency-free leaf `save_bonuses_from_feats` is -- importing nothing from
/// `pilot_compute.rs`; the caller already holds `ability_modifiers.strength`.
/// Applied verbatim (no floor at 0): the corpus token adds the raw modifier,
/// so a negative Strength modifier would reduce Intimidate, and clamping it
/// would fabricate a value the corpus does not specify. Its Str 13 prerequisite
/// makes a negative modifier a non-realistic posture regardless.
pub fn skill_bonuses_from_feats(
    selected_feats: &[String],
    strength_modifier: i16,
) -> SkillBonusesFromFeats {
    let has = |key: &str| selected_feats.iter().any(|feat| feat == key);

    let athletic = has(ATHLETIC_FEAT_KEY);
    let persuasive_intimidate = if has(PERSUASIVE_FEAT_KEY) { TWO_SKILL_FEAT_BONUS } else { 0 };
    let intimidating_prowess_intimidate =
        if has(INTIMIDATING_PROWESS_FEAT_KEY) { strength_modifier } else { 0 };

    SkillBonusesFromFeats {
        climb: if athletic { TWO_SKILL_FEAT_BONUS } else { 0 },
        intimidate: persuasive_intimidate + intimidating_prowess_intimidate,
        swim: if athletic { TWO_SKILL_FEAT_BONUS } else { 0 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grants_no_bonus_when_toughness_is_not_selected() {
        let selected_feats = vec!["feat:power_attack".to_owned(), "Cleave".to_owned()];
        assert_eq!(hp_bonus_from_feats(&selected_feats), 0);
    }

    #[test]
    fn grants_no_bonus_for_an_empty_feat_list() {
        assert_eq!(hp_bonus_from_feats(&[]), 0);
    }

    #[test]
    fn grants_the_real_flat_plus_three_when_toughness_is_selected() {
        let selected_feats = vec!["feat:power_attack".to_owned(), "Toughness".to_owned()];
        assert_eq!(hp_bonus_from_feats(&selected_feats), 3);
    }

    #[test]
    fn does_not_match_the_old_synthetic_feat_toughness_convention() {
        // The three hardcoded fixed-loadout feats use a "feat:<name>"
        // convention (a bespoke id space, never catalog-derived); a real
        // Toughness pick never produces that shape. Confirms this engine
        // keys on the real catalog string, not the unrelated convention.
        let selected_feats = vec!["feat:toughness".to_owned()];
        assert_eq!(hp_bonus_from_feats(&selected_feats), 0);
    }

    #[test]
    fn is_not_double_counted_if_the_same_feat_string_somehow_appears_twice() {
        // Not a realistic selected_feats shape (nothing in this crate
        // pushes a duplicate), but the flat scan-and-match shape naturally
        // stays a single +3 rather than +3-per-occurrence, worth pinning.
        let selected_feats = vec!["Toughness".to_owned(), "Toughness".to_owned()];
        assert_eq!(hp_bonus_from_feats(&selected_feats), 3);
    }
}

#[cfg(test)]
mod save_bonuses_from_feats_tests {
    use super::*;

    #[test]
    fn grants_no_bonus_when_no_save_feat_is_selected() {
        let selected_feats = vec!["Toughness".to_owned(), "Cleave".to_owned()];
        assert_eq!(save_bonuses_from_feats(&selected_feats), SaveBonusesFromFeats::default());
    }

    #[test]
    fn grants_no_bonus_for_an_empty_feat_list() {
        assert_eq!(save_bonuses_from_feats(&[]), SaveBonusesFromFeats::default());
    }

    #[test]
    fn great_fortitude_grants_the_real_flat_plus_two_to_fortitude_only() {
        let selected_feats = vec!["Great Fortitude".to_owned()];
        assert_eq!(
            save_bonuses_from_feats(&selected_feats),
            SaveBonusesFromFeats { fortitude: 2, reflex: 0, will: 0 }
        );
    }

    #[test]
    fn lightning_reflexes_grants_the_real_flat_plus_two_to_reflex_only() {
        let selected_feats = vec!["Lightning Reflexes".to_owned()];
        assert_eq!(
            save_bonuses_from_feats(&selected_feats),
            SaveBonusesFromFeats { fortitude: 0, reflex: 2, will: 0 }
        );
    }

    #[test]
    fn iron_will_grants_the_real_flat_plus_two_to_will_only() {
        let selected_feats = vec!["Iron Will".to_owned()];
        assert_eq!(
            save_bonuses_from_feats(&selected_feats),
            SaveBonusesFromFeats { fortitude: 0, reflex: 0, will: 2 }
        );
    }

    #[test]
    fn all_three_stack_independently_when_all_selected() {
        let selected_feats =
            vec!["Great Fortitude".to_owned(), "Lightning Reflexes".to_owned(), "Iron Will".to_owned()];
        assert_eq!(
            save_bonuses_from_feats(&selected_feats),
            SaveBonusesFromFeats { fortitude: 2, reflex: 2, will: 2 }
        );
    }

    #[test]
    fn does_not_match_the_improved_variant_feats() {
        // "Improved Great Fortitude" etc. are distinct, real CRB feats with
        // no `effect` at all in the catalog (see feat_data/general.rs) --
        // confirms this engine matches the exact base-feat string, not a
        // substring/prefix.
        let selected_feats = vec![
            "Improved Great Fortitude".to_owned(),
            "Improved Iron Will".to_owned(),
            "Improved Lightning Reflexes".to_owned(),
        ];
        assert_eq!(save_bonuses_from_feats(&selected_feats), SaveBonusesFromFeats::default());
    }
}

#[cfg(test)]
mod skill_bonuses_from_feats_tests {
    use super::*;

    // A realistic positive Strength modifier for the Intimidating Prowess
    // cases (its Str 13 prerequisite guarantees a non-negative modifier for
    // any character that legally holds the feat). Only Intimidating Prowess
    // reads this argument; the other two feats ignore it entirely.
    const STR_MOD: i16 = 3;

    #[test]
    fn grants_no_bonus_when_no_skill_feat_is_selected() {
        let selected_feats = vec!["Toughness".to_owned(), "Great Fortitude".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats::default()
        );
    }

    #[test]
    fn grants_no_bonus_for_an_empty_feat_list() {
        assert_eq!(skill_bonuses_from_feats(&[], STR_MOD), SkillBonusesFromFeats::default());
    }

    #[test]
    fn athletic_grants_the_real_flat_plus_two_to_climb_and_swim_only() {
        let selected_feats = vec!["Athletic".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats { climb: 2, intimidate: 0, swim: 2 }
        );
    }

    #[test]
    fn persuasive_grants_the_real_flat_plus_two_to_intimidate_only() {
        // Persuasive's real CRB effect also grants +2 Diplomacy, but Diplomacy
        // is not one of the three skills the engine computes today, so this
        // engine grounds only the Intimidate half -- the half that has a live
        // consumer. The Diplomacy half is deferred, not fabricated.
        let selected_feats = vec!["Persuasive".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats { climb: 0, intimidate: 2, swim: 0 }
        );
    }

    #[test]
    fn intimidating_prowess_adds_the_strength_modifier_to_intimidate_only() {
        // Real CRB benefit: "Add your Strength modifier to Intimidate skill
        // checks in addition to your Charisma modifier." Corpus token
        // `BONUS:SKILL|Intimidate|STR` -- the raw Strength modifier, not a
        // flat constant, which is why this function takes a strength_modifier
        // argument at all.
        let selected_feats = vec!["Intimidating Prowess".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats { climb: 0, intimidate: STR_MOD, swim: 0 }
        );
    }

    #[test]
    fn persuasive_and_intimidating_prowess_stack_on_intimidate() {
        // Different real sources (Persuasive's flat +2 vs. Intimidating
        // Prowess's Strength-modifier add), so PF1 stacks them on Intimidate.
        let selected_feats = vec!["Persuasive".to_owned(), "Intimidating Prowess".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats { climb: 0, intimidate: 2 + STR_MOD, swim: 0 }
        );
    }

    #[test]
    fn all_three_stack_independently_when_all_selected() {
        let selected_feats = vec![
            "Athletic".to_owned(),
            "Persuasive".to_owned(),
            "Intimidating Prowess".to_owned(),
        ];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats { climb: 2, intimidate: 2 + STR_MOD, swim: 2 }
        );
    }

    #[test]
    fn intimidating_prowess_faithfully_adds_a_negative_strength_modifier() {
        // The corpus token `BONUS:SKILL|Intimidate|STR` is the raw modifier
        // with no floor, so a negative Strength modifier reduces Intimidate.
        // Not a realistic posture for a feat gated on Str 13, but pinning the
        // verbatim-corpus behavior rather than silently clamping to 0 (which
        // would fabricate a value the corpus does not specify).
        let selected_feats = vec!["Intimidating Prowess".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, -1),
            SkillBonusesFromFeats { climb: 0, intimidate: -1, swim: 0 }
        );
    }

    #[test]
    fn does_not_match_a_substring_or_prefix_of_a_feat_key() {
        // Confirms the exact-string match, not a prefix/substring one: a
        // synthetic "Athletic Steps" (which begins with the grounded "Athletic"
        // key) must not trigger Athletic's bonus, and "Acrobatic" (a real,
        // distinct CRB feat this engine does not ground) must not either.
        let selected_feats = vec!["Athletic Steps".to_owned(), "Acrobatic".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats::default()
        );
    }
}
