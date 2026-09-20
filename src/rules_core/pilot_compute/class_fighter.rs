#[allow(unused_imports)]
pub(crate) use super::*;

pub(crate) const FIGHTER_CLASS_ID: &str = "class:fighter";

/// v0.6 alpha swarm, risks item 8 (third APG/ACG closure): ACG Brawler, a
/// pure martial class (no `SPELLSTAT`, `ROLE:None`) whose AC Bonus class
/// feature ("when wearing light or no armor, a brawler adds %1 AC as a
/// dodge bonus") is a pure function of level with NO activation state and
/// NO choice -- structurally simpler than every other class built this
/// session, which all needed either a `class_ability_activations` entry
/// (Barbarian/Skald/Bloodrager) or a `selected_choices` entry (Sorcerer/
/// Cleric/Druid). The "not wearing Medium/Heavy armor" precondition is
/// provably vacuous in this codebase: no Medium- or Heavy-armor item id
/// exists anywhere for any class to equip (the only armor this codebase
/// can express at all is Chain Shirt, itself light armor), and the shared
/// combat-baseline posture gate already requires Chain Shirt
/// `EquippedActive` unconditionally, so the value is grounded
/// unconditionally on class ownership and level alone.
pub(super) const BRAWLER_CLASS_ID: &str = "class:brawler";

/// Brawler grant levels, read from the ACG class table.
pub(super) const BRAWLER_BONUS_FEAT_LEVEL: u8 = 2;

pub(super) const BRAWLER_MANEUVER_TRAINING_LEVEL: u8 = 3;

pub(super) const BRAWLER_KNOCKOUT_LEVEL: u8 = 4;

/// Brawler's Flurry self-applied attack penalty
/// (`BONUS:VAR|BrawlersFlurryAttackPenalty|-2`) -- a real token, unlike
/// Cavalier's DESC-only Challenge penalty.
pub(super) const BRAWLER_FLURRY_ATTACK_PENALTY: i16 = -2;

pub(crate) const FIGHTER_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat";

// Bounded SD13-E3/SD13-E5 Fighter milestone widening. The accepted level-1 pilot
// is now joined by levels 2 through 17 (SD18: level 11 widens the Armor
// Training pillar to rank 3, see FIGHTER_ARMOR_TRAINING_3_LEVEL below; level
// 12 widens the bonus-feat cadence with a sixth named slot, see
// FIGHTER_LEVEL_12_BONUS_FEAT_CHOICE_ID below; level 13 widens Weapon
// Training to rank 3 and its third chosen weapon group, see
// FIGHTER_WEAPON_TRAINING_GROUP_3_CHOICE_ID below; level 14 widens the
// bonus-feat cadence with a seventh named slot, see
// FIGHTER_LEVEL_14_BONUS_FEAT_CHOICE_ID below, and raises Bravery's
// already-generic magnitude formula to +4; level 15 widens the Armor
// Training pillar to rank 4, see FIGHTER_ARMOR_TRAINING_4_LEVEL below --
// level 15 is neither a bonus-feat cadence level nor a weapon-training
// rank-rise level, so no new choice slot is added there; level 16 widens
// the bonus-feat cadence with an eighth named slot, see
// FIGHTER_LEVEL_16_BONUS_FEAT_CHOICE_ID below -- level 16 is neither a
// weapon-training rank-rise level nor an armor-training rank-rise level
// (the PF1 Core Rulebook names no fifth Armor Training rank), so neither
// pillar widens there; level 17 (SD18 widening) widens Weapon Training to
// rank 4 and its fourth chosen weapon group, see
// FIGHTER_WEAPON_TRAINING_GROUP_4_CHOICE_ID below -- level 17 is neither a
// bonus-feat cadence level (1, 2, 4, 6, 8, 10, 12, 14, 16, ...) nor an
// armor-training rank-rise level, so neither of those pillars widens there;
// level 18 (SD18 widening) widens the bonus-feat cadence with a ninth named
// slot, see FIGHTER_LEVEL_18_BONUS_FEAT_CHOICE_ID below, and raises
// Bravery's already-generic magnitude formula to +5 -- level 18 is neither
// a weapon-training rank-rise level nor an armor-training rank-rise level,
// so neither pillar widens there; level 19 (SD18 widening) is neither a
// bonus-feat cadence level (1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20 -- 19 is
// absent), a weapon-training rank-rise level, nor an armor-training
// rank-rise level, so none of those three pillars widen there, but the PF1
// Core Rulebook level-19 Special column names a genuinely new class
// feature, Armor Mastery (DR 5/-- while wearing armor or using a shield),
// see FIGHTER_ARMOR_MASTERY_LEVEL below, grounded as a bounded
// flat-magnitude record only, mirroring the already-proven Barbarian
// Damage Reduction idiom); level 20 (SD18 widening, the FINAL level within
// PF1's 1-20 character-level cap) IS a bonus-feat cadence level (1, 2, 4,
// 6, 8, 10, 12, 14, 16, 18, 20), so this widens the bonus-feat cadence
// with a TENTH named slot, see FIGHTER_LEVEL_20_BONUS_FEAT_CHOICE_ID
// below, and the PF1 Core Rulebook level-20 Special column names a
// genuinely new capstone class feature, Weapon Mastery (automatic
// critical-threat confirmation and a +1 critical-multiplier increase for
// one chosen weapon, plus immunity to being disarmed of that weapon), see
// FIGHTER_WEAPON_MASTERY_LEVEL below, grounded as a bounded grant-only
// identity/magnitude record only, mirroring exactly the Armor Mastery
// idiom -- level 20 is neither a weapon-training rank-rise level nor an
// armor-training rank-rise level, so neither of those pillars widens
// there. Nothing here grounds the weapon-training damage-roll half, the
// Bravery Will-vs-fear bonus resolution, the Armor Mastery
// damage-reduction application, the Weapon Mastery critical-confirmation/
// damage-multiplier/disarm-immunity application, or any non-Fighter
// positive support. The generic PF1 ability-score-increase milestones
// (levels 4 and 8) need no separate seam: the chosen ability score is
// trusted at face value, like every other ability adjustment in this
// codebase.
pub(crate) const MAX_SUPPORTED_FIGHTER_LEVEL: u8 = 20;

// Fighter level-1 hit points. PF1 maximizes the hit die at 1st character level:
// the Fighter's d10 hit die grants 10 hit points at level 1, plus the
// Constitution modifier. This slice grounds only that level-1 value; hit points
// at levels 2+ (average/rolled hit-die policy), the favored-class +1 hp /
// +1 skill-rank choice (no input surface exists for it), and Toughness / feat
// hit-point interplay stay unproven.
pub(super) const FIGHTER_LEVEL_1_MAX_HIT_DIE_HIT_POINTS: i16 = 10;

// Fighter level-2 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 2; this slice surfaces the named selection as an explicit seam only
// and grounds no general feat-effect or prerequisite engine.
pub(super) const FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_2";

// Fighter level-4 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 4 (the cadence continues at 1, 2, 4, 6, 8, 10, ...); this slice
// surfaces the named selection as an explicit seam only and grounds no general
// feat-effect or prerequisite engine, mirroring the level-2 seam.
pub(super) const FIGHTER_LEVEL_4_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_4";

// Fighter level-6 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 6 (the cadence continues 1, 2, 4, 6, 8, 10, ...); this slice
// surfaces the named selection as an explicit seam only and grounds no general
// feat-effect or prerequisite engine, mirroring the level-2/level-4 seams.
pub(super) const FIGHTER_LEVEL_6_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_6";

// Fighter level-8 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 8 (the cadence continues 1, 2, 4, 6, 8, 10, ...); this slice
// surfaces the named selection as an explicit seam only and grounds no general
// feat-effect or prerequisite engine, mirroring the level-2/level-4/level-6 seams.
pub(super) const FIGHTER_LEVEL_8_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_8";

// Fighter level-10 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 10 (the cadence continues 1, 2, 4, 6, 8, 10); this slice surfaces
// the named selection as an explicit seam only and grounds no general feat-effect
// or prerequisite engine, mirroring the level-2 through level-8 seams. The
// canonical Greater Weapon Focus selection's prerequisites (Weapon Focus with the
// chosen weapon and fighter level 8) are honestly met by the canonical loadout:
// Weapon Focus (longsword) is the level-1 fighter bonus feat and the seam only
// exists at Fighter level 10.
pub(super) const FIGHTER_LEVEL_10_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_10";

// Fighter level-12 bonus-feat progression seam (SD18 widening). Fighter gains
// an additional bonus feat at level 12 (the cadence continues 1, 2, 4, 6, 8,
// 10, 12); this slice surfaces the named selection as an explicit seam only
// and grounds no general feat-effect or prerequisite engine, mirroring the
// level-2 through level-10 seams. The canonical Weapon Specialization
// selection's prerequisites (fighter level 4 and Weapon Focus with the
// chosen weapon) are honestly met by the canonical loadout: Weapon Focus
// (longsword) is the level-1 fighter bonus feat.
pub(super) const FIGHTER_LEVEL_12_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_12";

// Fighter level-14 bonus-feat progression seam (SD18 widening). Fighter gains
// an additional bonus feat at level 14 (the cadence continues 1, 2, 4, 6, 8,
// 10, 12, 14); this slice surfaces the named selection as an explicit seam
// only and grounds no general feat-effect or prerequisite engine, mirroring
// the level-2 through level-12 seams. The canonical Greater Weapon
// Specialization selection's prerequisites (fighter level 12, Weapon Focus
// and Weapon Specialization with the chosen weapon) are honestly met by the
// canonical loadout: Weapon Focus (longsword) is the level-1 fighter bonus
// feat and Weapon Specialization (longsword) is the level-12 fighter bonus
// feat.
pub(super) const FIGHTER_LEVEL_14_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_14";

// Fighter level-16 bonus-feat progression seam (SD18 widening). Fighter gains
// an additional bonus feat at level 16 (the cadence continues 1, 2, 4, 6, 8,
// 10, 12, 14, 16); this slice surfaces the named selection as an explicit
// seam only and grounds no general feat-effect or prerequisite engine,
// mirroring the level-2 through level-14 seams. The canonical Critical Focus
// selection's prerequisite (base attack bonus +9) is honestly met by the
// canonical loadout: the level-16 Fighter's own base attack bonus is +16.
pub(super) const FIGHTER_LEVEL_16_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_16";

// Fighter level-18 bonus-feat progression seam (SD18 widening). Fighter gains
// an additional bonus feat at level 18 (the cadence continues 1, 2, 4, 6, 8,
// 10, 12, 14, 16, 18); this slice surfaces the named selection as an
// explicit seam only and grounds no general feat-effect or prerequisite
// engine, mirroring the level-2 through level-16 seams. The canonical
// Staggering Critical selection's prerequisites (Critical Focus and base
// attack bonus +13) are honestly met by the canonical loadout: Critical
// Focus is the level-16 fighter bonus feat and the level-18 Fighter's own
// base attack bonus is +18.
pub(super) const FIGHTER_LEVEL_18_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_18";

// Fighter level-20 bonus-feat progression seam (SD18 widening, the FINAL
// bonus-feat cadence level within PF1's 1-20 character-level cap). Fighter
// gains an additional bonus feat at level 20 (the cadence continues 1, 2,
// 4, 6, 8, 10, 12, 14, 16, 18, 20); this slice surfaces the named
// selection as an explicit seam only and grounds no general feat-effect
// or prerequisite engine, mirroring the level-2 through level-18 seams.
// The canonical Critical Mastery selection's prerequisites (two other
// critical feats) are honestly met by the canonical loadout: Improved
// Critical, Critical Focus, and Staggering Critical are all
// already-selected Fighter bonus feats.
pub(super) const FIGHTER_LEVEL_20_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_20";

// Fighter Weapon Training, gained at level 5 with a new rank every four levels
// (rank = 1 + (level - 5) / 4): Weapon Training 1 at level 5, Weapon Training 2
// at level 9, Weapon Training 3 at level 13, Weapon Training 4 at level 17
// (SD18 widening). Each rank grants the first chosen weapon group +rank to
// attack and damage rolls; each later-chosen group sits one point lower. This
// slice grounds only the attack-roll half of the first group (folded into the
// baseline melee attack bonus for the deterministic Longsword, which falls
// under the canonical Heavy Blades group) and surfaces the second group
// (canonically Bows, chosen at level 9), the third group (canonically
// Polearms, chosen at level 13), and the fourth group (canonically Hammers,
// chosen at level 17) as explanation-only records; the damage-roll half is
// never computed for any Fighter level in this codebase, so it stays
// explicitly unproven rather than silently omitted.
pub(super) const FIGHTER_WEAPON_TRAINING_1_LEVEL: u8 = 5;

pub(super) const FIGHTER_WEAPON_TRAINING_RANK_LEVEL_STRIDE: u8 = 4;

pub(super) const FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID: &str = "choice:fighter_weapon_training_group";

pub(super) const FIGHTER_WEAPON_TRAINING_GROUP_2_CHOICE_ID: &str = "choice:fighter_weapon_training_group_2";

pub(super) const FIGHTER_WEAPON_TRAINING_GROUP_3_CHOICE_ID: &str = "choice:fighter_weapon_training_group_3";

pub(super) const FIGHTER_WEAPON_TRAINING_GROUP_4_CHOICE_ID: &str = "choice:fighter_weapon_training_group_4";

/// `AT-34-E3-001` (`decisions.md §14`, mechanism 3: `class_feature_option_pool_
/// record_with_magnitude_not_held_by_engine`): read-only bridge exposing
/// every `(tier, group)` pair the weapon-training computation below can now
/// genuinely ground to `v06_work_inventory`'s live-computation probe,
/// mirroring `domain_power::domain_power_probe_catalog`'s own shape. No
/// behavior change to the weapon-training computation itself -- every
/// tuple's own choice id and selection literal is drawn from the SAME
/// `WEAPON_TRAINING_GROUPS` constant the real computation reads below,
/// never a re-typed copy.
///
/// Each tuple is `(tier, corpus group-name suffix, choice id, selection,
/// explanation id)`. The corpus group-name suffix is the exact text
/// following `"Weapon Training <tier> "` in this book's own corpus key
/// (`"Weapon Training 1 Blades Heavy"`, `"Weapon Training 2 Bows"`, ...,
/// confirmed live against `docs/work-inventory.json`), never a guess.
///
/// `AT-34-E3-001` (mechanism 3 continuation, cycle 9): widened from 4
/// hardcoded canonical tuples to all `4 * 14 = 56` (tier, group)
/// combinations, mirroring the real computation's own generalization from
/// "one canonical group per tier" to "any of the 14 canonical groups per
/// tier" (see `WEAPON_TRAINING_GROUPS`'s own doc comment for why this is a
/// closed enumerable set, not an open-ended relaxation).
pub fn fighter_weapon_training_canonical_catalog()
-> Vec<(u8, &'static str, &'static str, &'static str, &'static str)> {
    const TIERS: [(u8, &str, &str); 4] = [
        (
            1,
            FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID,
            "class_feature.fighter.weapon_training",
        ),
        (
            2,
            FIGHTER_WEAPON_TRAINING_GROUP_2_CHOICE_ID,
            "class_feature.fighter.weapon_training_group_2",
        ),
        (
            3,
            FIGHTER_WEAPON_TRAINING_GROUP_3_CHOICE_ID,
            "class_feature.fighter.weapon_training_group_3",
        ),
        (
            4,
            FIGHTER_WEAPON_TRAINING_GROUP_4_CHOICE_ID,
            "class_feature.fighter.weapon_training_group_4",
        ),
    ];
    let mut catalog = Vec::with_capacity(TIERS.len() * WEAPON_TRAINING_GROUPS.len());
    for (tier, choice_id, explanation_id) in TIERS {
        for (group_suffix, selection) in WEAPON_TRAINING_GROUPS {
            catalog.push((tier, group_suffix, choice_id, selection, explanation_id));
        }
    }
    catalog
}

// Fighter armor training 1, gained at level 3. It reduces the worn armor's
// armor-check penalty by 1 (to a minimum of 0) and raises its maximum Dexterity
// bonus by 1. Grounded from cr_abilities_class.lst Fighter armor training; not
// oracle-checked parity.
pub(super) const FIGHTER_ARMOR_TRAINING_1_LEVEL: u8 = 3;

// Fighter armor training 2, gained at level 7. It further reduces the worn
// armor's armor-check penalty (to a minimum of 0, cumulative with Armor
// Training 1) and further raises its maximum Dexterity bonus. Grounded from
// cr_abilities_class.lst Fighter armor training; not oracle-checked parity.
pub(super) const FIGHTER_ARMOR_TRAINING_2_LEVEL: u8 = 7;

// Fighter armor training 3, gained at level 11 (SD18 widening). It further
// reduces the worn armor's armor-check penalty (to a minimum of 0, cumulative
// with Armor Training 1-2) and further raises its maximum Dexterity bonus.
// Grounded from cr_abilities_class.lst Fighter armor training; not
// oracle-checked parity. On the deterministic Chain Shirt fixture, the
// armor-check-penalty reduction was already capped at 0 by Armor Training 2,
// so this rise changes no derived Climb/Swim total; the raised maximum
// Dexterity bonus also changes no derived armor class, since the
// deterministic +2 Dexterity contribution is already well below both the old
// and new caps. The rank rise itself is still a genuine, non-fabricated
// magnitude widening on the already-grounded pillar.
pub(super) const FIGHTER_ARMOR_TRAINING_3_LEVEL: u8 = 11;

// Fighter armor training 4, gained at level 15 (SD18 widening). It further
// reduces the worn armor's armor-check penalty (to a minimum of 0, cumulative
// with Armor Training 1-3) and further raises its maximum Dexterity bonus.
// Grounded from cr_abilities_class.lst Fighter armor training; not
// oracle-checked parity. On the deterministic Chain Shirt fixture, the
// armor-check-penalty reduction was already capped at 0 by Armor Training 2,
// so this rise changes no derived Climb/Swim total; the raised maximum
// Dexterity bonus also changes no derived armor class, since the
// deterministic +2 Dexterity contribution is already well below both the old
// and new caps. The rank rise itself is still a genuine, non-fabricated
// magnitude widening on the already-grounded pillar.
pub(super) const FIGHTER_ARMOR_TRAINING_4_LEVEL: u8 = 15;

// Fighter Armor Mastery, gained at level 19 (SD18 widening). Verified
// independently against d20pfsrd.com/classes/core-classes/fighter/ and
// aonprd.com/ClassDisplay.aspx?ItemName=Fighter (byte-for-byte agreement):
// "At 19th level, a fighter gains DR 5/-- whenever he is wearing armor or
// using a shield." This is a genuinely new named class feature (NOT another
// Armor Training rank -- the PF1 Core Rulebook names no fifth Armor
// Training rank, and Armor Mastery is a distinct feature from the Armor
// Training pillar). Grounded as a bounded flat-magnitude record only,
// mirroring exactly how the Barbarian's own Damage Reduction class feature
// was grounded: no damage-resolution engine and no incoming-damage total
// exists anywhere in this codebase to apply it, and no
// worn-armor-or-shield condition check is computed, so this grounds no
// actual damage reduction.
pub(super) const FIGHTER_ARMOR_MASTERY_LEVEL: u8 = 19;

// Fighter Weapon Mastery, gained at level 20 (SD18 widening, the Fighter
// capstone -- the FINAL level within PF1's 1-20 character-level cap).
// Verified independently against d20pfsrd.com/classes/core-classes/fighter/
// and aonprd.com/ClassDisplay.aspx?ItemName=Fighter (byte-for-byte
// agreement): "At 20th level, a fighter chooses one weapon, such as the
// longsword, greataxe, or longbow. Any attacks made with that weapon
// automatically confirm all critical threats and have their damage
// multiplier increased by 1 (x2 becomes x3, for example). In addition, he
// cannot be disarmed while wielding a weapon of this type." This is a
// genuinely new named class feature. Grounded as a bounded grant-only
// identity/magnitude record only, mirroring exactly how the Fighter's own
// Armor Mastery class feature was grounded: no critical-hit-confirmation
// engine, no damage-multiplier-application engine, and no
// disarm-resolution engine exists anywhere in this codebase to apply it,
// so this grounds no actual automatic-critical-confirmation, no actual
// damage-multiplier change, and no actual disarm immunity.
pub(super) const FIGHTER_WEAPON_MASTERY_LEVEL: u8 = 20;

// Fighter Bravery, gained at level 2 with an additional +1 every four Fighter
// levels thereafter (level 6, level 10, ...): +1 Will save vs fear at level 2,
// +2 at level 6, +3 at level 10, per PF1 Core Rulebook. This slice grounds only
// the flat bonus magnitude as a standalone explanation record, mirroring the
// Weapon Training attack-bonus-rank idiom; no fear-condition or save-resolution
// engine exists anywhere in this codebase, so the bonus is never folded into the
// unconditional Will save total.
pub(super) const FIGHTER_BRAVERY_LEVEL: u8 = 2;

pub(super) const FIGHTER_BRAVERY_RANK_LEVEL_STRIDE: u8 = 4;

/// The bounded Fighter milestone level this surface grounds, if any. Returns the
/// single Fighter level when the chosen input is exactly a single-class Fighter at
/// one of the supported milestone levels (1, 2, or 3). Returns `None` for no
/// Fighter, a non-Fighter class, a multiclass mix, or a level-4+ Fighter this slice
/// does not yet ground — each of which stays claim-blocked as before.
pub(crate) fn supported_fighter_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == FIGHTER_CLASS_ID
                && (1..=MAX_SUPPORTED_FIGHTER_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Fighter armor-training profile for a given Fighter level. Armor training 1 is
/// gained at level 3, armor training 2 at level 7, armor training 3 at level
/// 11, and armor training 4 at level 15 (SD18 widening); before level 3 there
/// is no armor-training effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FighterArmorTraining {
    /// Armor-training rank (0 before level 3, 1 from level 3, 2 from level 7,
    /// 3 from level 11, 4 from level 15).
    pub(crate) rank: u8,
    /// Reduction applied to the worn armor's armor-check penalty (moves it toward 0).
    pub(crate) armor_check_reduction: i16,
    /// Increase applied to the worn armor's maximum Dexterity bonus.
    pub(crate) max_dex_increase: i16,
}

pub(crate) fn fighter_armor_training(level: u8) -> FighterArmorTraining {
    if level >= FIGHTER_ARMOR_TRAINING_4_LEVEL {
        FighterArmorTraining {
            rank: 4,
            armor_check_reduction: ARMOR_TRAINING_4_ARMOR_CHECK_REDUCTION,
            max_dex_increase: ARMOR_TRAINING_4_MAX_DEX_INCREASE,
        }
    } else if level >= FIGHTER_ARMOR_TRAINING_3_LEVEL {
        FighterArmorTraining {
            rank: 3,
            armor_check_reduction: ARMOR_TRAINING_3_ARMOR_CHECK_REDUCTION,
            max_dex_increase: ARMOR_TRAINING_3_MAX_DEX_INCREASE,
        }
    } else if level >= FIGHTER_ARMOR_TRAINING_2_LEVEL {
        FighterArmorTraining {
            rank: 2,
            armor_check_reduction: ARMOR_TRAINING_2_ARMOR_CHECK_REDUCTION,
            max_dex_increase: ARMOR_TRAINING_2_MAX_DEX_INCREASE,
        }
    } else if level >= FIGHTER_ARMOR_TRAINING_1_LEVEL {
        FighterArmorTraining {
            rank: 1,
            armor_check_reduction: ARMOR_TRAINING_1_ARMOR_CHECK_REDUCTION,
            max_dex_increase: ARMOR_TRAINING_1_MAX_DEX_INCREASE,
        }
    } else {
        FighterArmorTraining {
            rank: 0,
            armor_check_reduction: 0,
            max_dex_increase: 0,
        }
    }
}

/// The Fighter weapon-training rank at the given level: 0 before level 5, then
/// 1 + (level - 5) / 4 (Weapon Training 1 at level 5, Weapon Training 2 at
/// level 9 within this bounded levels-1-10 surface).
pub(super) fn fighter_weapon_training_rank(level: u8) -> i16 {
    if level < FIGHTER_WEAPON_TRAINING_1_LEVEL {
        return 0;
    }
    i16::from(1 + (level - FIGHTER_WEAPON_TRAINING_1_LEVEL) / FIGHTER_WEAPON_TRAINING_RANK_LEVEL_STRIDE)
}

/// The weapon-training attack-roll bonus for the first chosen weapon group at
/// the given Fighter level, gated on the canonical
/// `choice:fighter_weapon_training_group -> group:heavy_blades` selection (the
/// group the deterministic Longsword falls under). The bonus equals the
/// weapon-training rank: +1 at levels 5-8, +2 at levels 9-10. Returns 0 before
/// level 5 or when the group choice is absent — the canonical-choice validator
/// (`CANONICAL_FIGHTER_FEAT_CHOICES`) separately claim-blocks a
/// present-but-non-canonical selection, so this function only needs to
/// distinguish "canonical" from "absent or anything else."
pub(crate) fn fighter_weapon_training_attack_bonus(input: &CharacterInput, level: u8) -> i16 {
    if choice_selection(input, FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID)
        == Some(HEAVY_BLADES_GROUP_SELECTION)
    {
        fighter_weapon_training_rank(level)
    } else {
        0
    }
}

/// The Fighter Bravery Will-save-vs-fear bonus magnitude at the given level: 0
/// before level 2, then 1 + (level - 2) / 4 (+1 at level 2, +2 at level 6, +3 at
/// level 10 within this bounded levels-1-10 surface). A flat magnitude only —
/// no fear-condition or save-resolution engine exists on this compute surface.
pub(super) fn fighter_bravery_bonus(level: u8) -> i16 {
    if level < FIGHTER_BRAVERY_LEVEL {
        return 0;
    }
    i16::from(1 + (level - FIGHTER_BRAVERY_LEVEL) / FIGHTER_BRAVERY_RANK_LEVEL_STRIDE)
}

/// Compute the bounded Fighter base chassis for the supported milestone levels
/// (1, 2, or 3), or block the claim if the input is not a supported single-class
/// Fighter posture for this narrow slice.
pub(super) fn compute_fighter_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> (i16, BaseSaves) {
    let Some(level) = supported_fighter_level(input) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis is only supported for a single-class {FIGHTER_CLASS_ID} at \
                 levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL}; chosen class levels {:?} do not provide it, \
                 so no chassis values were computed",
                input.chosen.class_levels
            ),
            claim_blocking: true,
        });
        return (0, BaseSaves::default());
    };

    // Grounded Fighter base progression from cr_classes.lst:139, evaluated at the
    // chosen level:
    //   BONUS:COMBAT|BASEAB|classlevel                -> level (full base attack)
    //   BONUS:SAVE|BASE.Fortitude|classlevel/2+2      -> level/2 + 2 (good save)
    //   BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3 -> level/3 (poor saves)
    let level_value = i16::from(level);
    let base_attack_bonus = level_value;
    let base_saves = BaseSaves {
        fortitude: level_value / 2 + 2,
        reflex: level_value / 3,
        will: level_value / 3,
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   from cr_classes.lst:139 BONUS:COMBAT|BASEAB|classlevel
            "Fighter level {level} base attack bonus = {base_attack_bonus}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: base_saves.fortitude,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   from cr_classes.lst:139 BONUS:SAVE|BASE.Fortitude|classlevel/2+2
            "Fighter level {level} base Fortitude save = {}",
            base_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: base_saves.reflex,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   from cr_classes.lst:139 BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3
            "Fighter level {level} base Reflex save = {}",
            base_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: base_saves.will,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   from cr_classes.lst:139 BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3
            "Fighter level {level} base Will save = {}",
            base_saves.will
        ),
    });

    (base_attack_bonus, base_saves)
}

/// v0.6 alpha swarm, risks item 8 (third APG/ACG closure): whether `input`
/// is a single-class Brawler at a level within
/// `acg::class_chassis_resolve`'s declared ceiling for Brawler --
/// mirrors `is_supported_skald_single_class`/
/// `is_supported_bloodrager_single_class` exactly, including the same
/// exact-match discipline (`== Some(AcgClassId::Brawler)`, not a broad
/// `.is_some()` that would admit any of the 10 ACG classes).
pub(super) fn is_supported_brawler_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if AcgClassId::from_class_id_str(&class_level.class_id) != Some(AcgClassId::Brawler) {
        return false;
    }
    acg::class_chassis_resolve(AcgClassId::Brawler, class_level.level, RuleSetId::Acg).is_some()
}

/// Brawler's AC Bonus dodge-bonus progression: `(level>3) + (level>8) +
/// (level>12) + (level>17)` (v0.6 alpha swarm, risks item 8, third
/// APG/ACG closure), verified against the PCGen corpus `BONUS:VAR`
/// formula -- +0 at levels 1-3, +1 at 4-8, +2 at 9-12, +3 at 13-17, +4 at
/// 18-20. A pure function of level, unlike every Rage-shaped mechanic
/// this session built: no activation state, no choice, no rounds-per-day
/// budget -- Brawler's AC Bonus is always on while the (provably vacuous
/// in this codebase, see `BRAWLER_CLASS_ID`'s own doc comment) light-or-
/// no-armor precondition holds.
pub(super) fn brawler_ac_bonus(level: u8) -> i16 {
    i16::from(level > 3) + i16::from(level > 8) + i16::from(level > 12) + i16::from(level > 17)
}

/// Whether `input` is a Brawler, and if so, its AC Bonus value at its own
/// level (v0.6 alpha swarm, risks item 8, third APG/ACG closure).
/// Class-ownership-gated by construction, mirroring every other
/// `active_<class>_<ability>_bonus` query function this session:
/// only returns `Some` when `class_levels` actually contains Brawler.
/// Unlike the Rage-shaped equivalents, there is no activation-state or
/// rounds-budget check here -- AC Bonus is always on given class
/// ownership and level alone, so a non-Brawler character's ability
/// modifiers/combat baseline are never touched, and a Brawler's own value
/// is never conditional on any per-instance input beyond level.
pub(super) fn active_brawler_ac_bonus(input: &CharacterInput) -> Option<i16> {
    let brawler_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == BRAWLER_CLASS_ID)
        .map(|class_level| class_level.level)?;
    Some(brawler_ac_bonus(brawler_level))
}

/// PF1 Advanced Class Guide Brawler's Cunning: "if the brawler's
/// Intelligence score is less than 13, it counts as 13 for the purpose
/// of meeting the prerequisites of combat feats." Verified directly
/// against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|CombatFeatIntRequirement|max(13,INTSCORE)`. Flat,
/// unconditional from level 1 -- no `PREVARGTEQ`/`PRECLASS` gate on this
/// record at all, unlike Brawler's Strike below. Takes the raw
/// Intelligence SCORE (not the derived ability modifier), since the real
/// rule floors the score itself, not a modifier.
pub(super) fn brawler_cunning_effective_intelligence_score(intelligence_score: i16) -> i16 {
    effective_combat_feat_intelligence_score(BRAWLER_CUNNING_INTELLIGENCE_FLOOR, intelligence_score)
}

/// The floor Brawler's Cunning raises a low Intelligence score to.
pub(super) const BRAWLER_CUNNING_INTELLIGENCE_FLOOR: i16 = 13;

/// PF1 Advanced Class Guide Brawler's Strike progression tier:
/// `(level>=5)+(level>=9)+(level>=12)+(level>=17)`, verified directly
/// against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|BrawlersStrikeProgression|(BrawlerLVL>=5)+(BrawlerLVL>=9)+(BrawlerLVL>=12)+(BrawlerLVL>=17)`.
/// Returns `None` below level 5 -- the feature genuinely doesn't exist
/// yet at that point (the real rule text is granted at 5th level per
/// the corpus's own level-gated progression counter, not a formula that
/// merely evaluates to a zero magnitude), the same "not yet gained"
/// shape as Swashbuckler's own Charmed Life (granted at 2nd) and
/// Shaman's own Healer's Touch (gated to 8th, deliberately NOT the MVP
/// grounded for that class). Tier 1 (levels 5-8) grants "unarmed
/// strikes count as magic weapons for DR"; tier 2+ (level 9+) grants
/// "magic, cold iron, and silver" -- verified directly against the
/// record's own two real `DESC:` lines, each gated on
/// `PREVARGTEQ:BrawlersStrikeProgression,1`/`,2`. Tiers 3 (level 12) and
/// 4 (level 17) do not add a further DR-bypass DESC line; they instead
/// grant `Brawler's Strike Alignment Selection` ability-pool points (a
/// real chooser -- which alignment type to add -- this closure does not
/// model, named honestly in the deferred diagnostic below).
pub(super) fn brawler_strike_progression_tier(level: u8) -> Option<i16> {
    let tier = i16::from(level >= 5)
        + i16::from(level >= 9)
        + i16::from(level >= 12)
        + i16::from(level >= 17);
    if tier == 0 {
        None
    } else {
        Some(tier)
    }
}

/// Grounds Brawler's AC Bonus, Brawler's Cunning, and Brawler's Strike
/// as standalone explanation records, then pushes the new, narrower
/// diagnostic naming Brawler's remaining ungrounded features (v0.6 alpha
/// swarm, risks item 8, third APG/ACG closure, deepened 2026-07-26 to
/// add Cunning and Strike). Called from `compute_acg_class_chassis`'s
/// Brawler branch.
///
/// Unlike Skald/Bloodrager, Brawler has no spellcasting at all
/// (`ROLE:None`, no `SPELLSTAT` in the corpus) -- the permanent remaining
/// bucket here is Brawler's OTHER named features (Brawler's Flurry,
/// Knockout, Martial Flexibility, Awesome Blow, Improved Awesome Blow,
/// Martial Training, Bonus Feats, Close Weapon Mastery, Maneuver
/// Training, and Brawler's Strike's own Alignment Selection chooser),
/// not deferred spell math, so the diagnostic is named
/// `class_feature.acg.brawler.other_features_deferred.unsupported`
/// rather than `spellcasting_deferred`, mirroring the same
/// diagnostic-honesty discipline (retire the blanket "no named
/// class-feature computation... grounded anywhere" claim, replace with a
/// Brawler's Maneuver Training count (task #5, 2026-07-27):
/// 1/2/3/4/5 at levels 3/7/11/15/19.
///
/// **Set by TWO stacking corpus lines, not one.** The visible line
/// `(BrawlerLVL>2)+(BrawlerLVL>6)+(BrawlerLVL>10)+(BrawlerLVL>14)` tops
/// out at 4; a separate `BONUS:VAR|BrawlerManeuverTraining|1|
/// PREVARGT:BrawlerLVL,18` supplies the fifth. Reading only the first
/// gives 4 at levels 19-20 -- the same partial-read shape as the Sacred
/// Weapon divisor bug.
///
/// The corpus corroborates this directly: it carries the author's own
/// comment on a rejected single-line variant noting it "never got
/// BrawlerManeuverTraining up to 5".
pub(super) fn brawler_maneuver_training_count(level: u8) -> i16 {
    let level = i16::from(level);
    [2, 6, 10, 14, 18].iter().map(|gate| i16::from(level > *gate)).sum()
}

/// Knockout's stat bonus: `max(STR,DEX)` -- **bare tokens, so ability
/// MODIFIERS**, not scores.
///
/// The corpus disambiguates deliberately and this must not be
/// normalized: Brawler's Cunning writes `max(13,INTSCORE)`, an explicit
/// SCORE token, which is why `brawler_cunning_effective_intelligence_score`
/// correctly takes a score. Copying that idiom here would build a save
/// DC out of raw ability scores and be wildly wrong.
pub(super) fn brawler_knockout_stat_bonus(strength_modifier: i16, dexterity_modifier: i16) -> i16 {
    strength_modifier.max(dexterity_modifier)
}

/// Knockout's save DC: `(BrawlerLVL/2)+10+KnockoutStatBonus`.
pub(super) fn brawler_knockout_dc(level: u8, stat_bonus: i16) -> i16 {
    i16::from(level) / 2 + 10 + stat_bonus
}

/// Knockout's uses per day: `(BrawlerLVL+2)/6`. Granted at 4th level
/// (NOT 5th -- the class table says 4, and the formula yields exactly 1
/// there, so the corpus is internally consistent).
pub(super) fn brawler_knockout_uses_per_day(level: u8) -> i16 {
    (i16::from(level) + 2) / 6
}

/// Brawler's Flurry extra attacks: `min((level+6)/7,3)` -- 1/2/3.
/// Granted 2nd level.
///
/// **The `min(...,3)` cap is provably vacuous across the real level
/// range** and is transcribed only for fidelity to the corpus token:
/// `(20+6)/7` is already exactly 3, so the inner term never exceeds the
/// cap for any level 1-20. Recorded because a test cannot distinguish
/// the capped from the uncapped formula here -- removing `.min(3)`
/// changes no value a Brawler can actually reach, so its own coverage is
/// honest-but-inert, the same category as Alchemist's Gnome-only Bomb
/// term.
pub(super) fn brawler_flurry_extra_attacks(level: u8) -> i16 {
    ((i16::from(level) + 6) / 7).min(3)
}

/// Brawler's bonus combat feats: `(1+BrawlerLVL)/3` -- feats at levels
/// 2/5/8/11/14/17/20, seven by 20th.
///
/// Seven `-1` deductions against this pool exist in the corpus
/// (`PREVARGTEQ:BrawlerLvl,2/5/8/11/14/17/20`), each gated on a
/// `Brawler_CF_BonusFeatN` flag. Every one is provably vacuous here: the
/// only setter anywhere is `Brawler Archetype ~ Wild Child.MOD`, and
/// this repo's corpus ingests only the base `brawler.json` with no
/// archetypes. Summing them blind would understate the pool at every
/// level from 2 up.
pub(super) fn brawler_bonus_feat_count(level: u8) -> i16 {
    (1 + i16::from(level)) / 3
}

/// Martial Flexibility's uses per day: `max(1,level/2)+3`. Granted 1st.
///
/// Only the POOL grounds. The ability itself -- gaining the benefit of a
/// combat feat she does not possess -- is a chooser over the entire
/// combat-feat list, and under the ratified Skill Focus precedent that
/// half needs an explicit recorded choice rather than a silently seeded
/// canonical feat.
///
/// `Extra Martial Flexibility` adds `3` more
/// (`BONUS:VAR|BrawlerMartialFlexibilityTimes|3`, prose "three
/// additional times per day"). It routes through
/// [`non_stacking_resource_feat_bonus`], **not** the counting helper
/// every sibling uses: this is the one Extra-<resource> record in all
/// three books with no `STACK:`/`MULT:` token and no repeat clause.
pub(super) fn brawler_martial_flexibility_uses(level: u8, selected_feats: &[String]) -> i16 {
    (i16::from(level) / 2).max(1)
        + 3
        + non_stacking_resource_feat_bonus(
            selected_feats,
            EXTRA_MARTIAL_FLEXIBILITY_FEAT_KEY,
            EXTRA_MARTIAL_FLEXIBILITY_USES,
        )
}

/// The Medium-size Brawler unarmed strike damage die, as
/// `(dice count, die face)`.
///
/// Transcribed from the corpus's **live** per-size records --
/// `acg_abilities_class.lst` lines 966/976/986/996/1006/1016,
/// `Brawler Unarmed Damage LVL <N> (Medium)`, each carrying its own
/// `UDAM:` plus a corroborating
/// `BONUS:VAR|PrimaryAttackDamageDice|<count>` and
/// `BONUS:VAR|PrimaryAttackDamageSize|<face>` pair. It is deliberately
/// **not** read off the `#`-disabled combined `UDAM:` lines at 944-949
/// sitting directly above them in the same file: those are the retired
/// all-sizes-on-one-line form, exactly the `#`-prefixed duplicate trap.
/// (They happen to agree here, which is precisely why reading them would
/// have felt safe.)
///
/// Band selection is the corpus's own
/// `BONUS:VAR|BrawlerUnarmedDamageProgression|(min(5,BrawlerUnarmedDamageLVL/4))`
/// (line 940) with `BONUS:VAR|BrawlerUnarmedDamageLVL|BrawlerLVL` on the
/// same record -- integer division by 4, capped at 5. So: levels 1-3
/// 1d6, 4-7 1d8, 8-11 1d10, 12-15 2d6, 16-19 2d8, 20 2d10.
///
/// **This ladder is value-identical to `monk_unarmed_strike_damage_die`
/// at every level 1-20, and that is a real fact about PF1, not a
/// shortcut taken here.** Brawler is written to advance unarmed damage
/// on the monk's schedule, and the two corpus record sets agree band for
/// band. It is nonetheless kept as its own function reading its own
/// records, for two reasons: the sameness is a rules coincidence that
/// either class could break independently (an archetype or errata moving
/// one ladder must not silently move the other), and the two are
/// separately sourced -- Monk's from the CRB, Brawler's from
/// `acg_abilities_class.lst`. Sharing the function would make one class's
/// number depend on the other class's corpus, which is the kind of
/// coupling this file's identifier-collision discipline exists to
/// prevent. An earlier draft of this comment asserted the ladders
/// DIFFERED at level 12; that was wrong, and checking it against
/// `monk_unarmed_strike_damage_die` is what caught it.
pub(super) fn brawler_unarmed_strike_damage_die(level: u8) -> (i16, i16) {
    match (i16::from(level) / 4).min(5) {
        0 => (1, 6),
        1 => (1, 8),
        2 => (1, 10),
        3 => (2, 6),
        4 => (2, 8),
        _ => (2, 10),
    }
}

/// Close Weapon Mastery's grant level, per `acg_classes.lst:101`
/// (`5  ABILITY:Brawler Class Feature|AUTOMATIC|Brawler ~ Close Weapon
/// Mastery`).
pub(super) const BRAWLER_CLOSE_WEAPON_MASTERY_LEVEL: u8 = 5;

/// The "4 levels lower" offset named verbatim in Close Weapon Mastery's
/// own corpus `DESC:` (`acg_abilities_class.lst:1028`).
pub(super) const BRAWLER_CLOSE_WEAPON_MASTERY_LEVEL_OFFSET: u8 = 4;

/// Close Weapon Mastery's substituted base damage die, as
/// `(dice count, die face)`, or `None` below its 5th-level grant.
///
/// The corpus record (`KEY:Brawler ~ Close Weapon Mastery`,
/// `acg_abilities_class.lst:1028`) carries **no BONUS token at all** --
/// its rule lives entirely in `DESC:`: "When wielding a close weapon,
/// she uses the unarmed strike damage of a brawler 4 levels lower
/// instead of the base damage for that weapon."
///
/// That is a real, computable magnitude rather than a zero-magnitude
/// resolution, so this grounds an actual die rather than taking the
/// grant-only identity route Awesome Blow takes below.
///
/// **Independently corroborated by the record's own two `.MOD` lines**
/// (`acg_abilities_class.lst:1030-1031`), which spell out concrete
/// Medium-size answers: `DESC:...treat its base damage as 1d6...
/// |PRESIZEEQ:M|PREVAREQ:BrawlerUnarmedDamageProgression,1` and the
/// matching `1d8` at progression `2`. Progression 1 is brawler levels
/// 4-7 and progression 2 is 8-11; this function returns 1d6 across 5-7
/// and 1d8 across 8-11, so the derived ladder and the corpus's own
/// hardcoded answers agree without either being fitted to the other.
pub(super) fn brawler_close_weapon_mastery_die(level: u8) -> Option<(i16, i16)> {
    if level < BRAWLER_CLOSE_WEAPON_MASTERY_LEVEL {
        return None;
    }
    Some(brawler_unarmed_strike_damage_die(
        level - BRAWLER_CLOSE_WEAPON_MASTERY_LEVEL_OFFSET,
    ))
}

/// Awesome Blow's grant level, per `acg_classes.lst:102`.
pub(super) const BRAWLER_AWESOME_BLOW_LEVEL: u8 = 16;

/// Improved Awesome Blow's grant level, per `acg_classes.lst:103`.
pub(super) const BRAWLER_IMPROVED_AWESOME_BLOW_LEVEL: u8 = 20;

/// Grounds Brawler's six remaining real features (task #5, 2026-07-27),
/// each at its own verified grant level.
pub(super) fn ground_brawler_remaining_features(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let flexibility = brawler_martial_flexibility_uses(level, &input.chosen.selected_feats);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.brawler.martial_flexibility_uses_per_day".to_owned(),
        value: flexibility,
        detail: format!(
            "Brawler level {level} Martial Flexibility: usable {flexibility} times per day \
             (max(1, level/2) + 3 + Extra Martial Flexibility feat ({:+})). That feat is the \
             one Extra-<resource> record in the Core/APG/ACG feat files with no STACK:/MULT: \
             token and no repeat clause, so a second copy adds nothing. Only the pool grounds \
             -- the ability gains the benefit of a combat feat she does not possess, a chooser \
             over the whole combat-feat list, which needs an explicit recorded choice rather \
             than a seeded canonical feat",
            non_stacking_resource_feat_bonus(
                &input.chosen.selected_feats,
                EXTRA_MARTIAL_FLEXIBILITY_FEAT_KEY,
                EXTRA_MARTIAL_FLEXIBILITY_USES
            )
        ),
    });

    for (id, label) in [
        ("class_feature.acg.brawler.martial_training.fighter_level_equivalence", "fighter"),
        ("class_feature.acg.brawler.martial_training.monk_level_equivalence", "monk"),
        ("class_feature.acg.brawler.martial_training.monk_feat_qualify", "monk feat-qualifying"),
    ] {
        explanations.push(ComputationExplanation {
            id: id.to_owned(),
            value: i16::from(level),
            detail: format!(
                "Brawler level {level} Martial Training: brawler levels count as {label} level \
                 {level} for the purpose of qualifying for feats. Grounds the level-equivalence \
                 FACT; the feat_prereqs wiring stays deferred, the same treatment already ruled \
                 for Swashbuckler's identical `FighterWeaponQualifyLVL` fact. Note Alchemist's \
                 identically-named Martial Training is a genuine no-op with zero corpus tokens \
                 -- same name, different class, opposite verdict"
            ),
        });
    }

    if level >= BRAWLER_BONUS_FEAT_LEVEL {
        let feats = brawler_bonus_feat_count(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.bonus_feat_count".to_owned(),
            value: feats,
            detail: format!(
                "Brawler level {level} bonus combat feats: {feats} ((1 + level)/3, granting one \
                 at levels 2, 5, 8, 11, 14, 17, and 20). Seven `-1` deductions against this pool \
                 exist in the corpus but every one is gated on a Brawler archetype flag whose \
                 only setter is Wild Child, and this repo ingests no Brawler archetype -- \
                 provably vacuous. Only the count grounds; which feats are chosen is not modelled"
            ),
        });

        let extra_attacks = brawler_flurry_extra_attacks(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.flurry_extra_attacks".to_owned(),
            value: extra_attacks,
            detail: format!(
                "Brawler level {level} Brawler's Flurry: {extra_attacks} extra attack(s) \
                 (min((level + 6)/7, 3), capped at 3). This engine computes a melee attack total \
                 only for its own fixed single-weapon posture, so the extra attacks ground as a \
                 standalone count"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.flurry_attack_penalty".to_owned(),
            value: BRAWLER_FLURRY_ATTACK_PENALTY,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   Unlike Cavalier's Challenge penalty this carries a real
                //   `BONUS:VAR|BrawlersFlurryAttackPenalty|-2` token rather than being
                //   DESC-sourced.
                "Brawler level {level} Brawler's Flurry attack penalty: \
                 {BRAWLER_FLURRY_ATTACK_PENALTY} on each attack while flurrying. Grounded standalone \
                 rather than applied to the attack total, because flurrying is an action the \
                 character takes rather than a persistent state this engine records"
            ),
        });
    }

    if level >= BRAWLER_MANEUVER_TRAINING_LEVEL {
        let count = brawler_maneuver_training_count(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.maneuver_training_count".to_owned(),
            value: count,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `1|PREVARGT:BrawlerLVL,18`
                "Brawler level {level} Maneuver Training: {count} maneuver(s) trained (1/2/3/4/5 at \
                 levels 3/7/11/15/19). The fifth comes from a SECOND corpus line stacking on the \
                 visible `(L>2)+(L>6)+(L>10)+(L>14)` one, which alone tops out at 4"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.maneuver_training.bull_rush_bonus".to_owned(),
            value: count,
            detail: format!(
                "Brawler level {level} Maneuver Training on Bull Rush: +{count} to that combat \
                 maneuver. Bull Rush is the one canonical maneuver grounded here, narrowed the \
                 same way Hunter's Animal Focus narrowed to Bull; the other nine (Dirty Trick, \
                 Disarm, Drag, Grapple, Overrun, Reposition, Steal, Sunder, Trip) are deferred. \
                 As the FIRST pick it takes the undegraded count -- later picks degrade by -1 \
                 each, which is not modelled and must not be assumed away if multiple picks are \
                 ever added"
            ),
        });
    }

    if level >= BRAWLER_KNOCKOUT_LEVEL {
        let stat_bonus = brawler_knockout_stat_bonus(
            ability_modifier(input.chosen.ability_scores.strength),
            ability_modifier(input.chosen.ability_scores.dexterity),
        );
        let uses = brawler_knockout_uses_per_day(level);
        let dc = brawler_knockout_dc(level, stat_bonus);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.knockout_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Brawler level {level} Knockout: usable {uses} time(s) per day ((level + 2)/6). \
                 Granted at 4th level, where the formula yields exactly 1"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.knockout_dc".to_owned(),
            value: dc,
            detail: format!(
                "Brawler level {level} Knockout save DC: {dc} (level/2 + 10 + the higher of the \
                 Strength and Dexterity MODIFIERS, {stat_bonus:+}). The corpus writes \
                 `max(STR,DEX)` with bare tokens -- modifiers -- deliberately distinct from \
                 Brawler's Cunning's explicit `INTSCORE`; building this DC from raw scores would \
                 be wildly wrong. The effect itself (the target falls unconscious on a failed \
                 Fortitude save) is opponent-directed and stays deferred; the DC and the pool \
                 are self-scoped and ground, the same split already accepted for Alchemist's \
                 Bomb DC"
            ),
        });
    }

    ground_brawler_close_weapon_mastery_and_awesome_blow(level, explanations);
}

/// Grounds Brawler's last three named class features (task #91):
/// Close Weapon Mastery, Awesome Blow, and Improved Awesome Blow.
///
/// These were the entire remaining content of Brawler's claim-blocking
/// `other_features_deferred` diagnostic. They split into two shapes, and
/// the split is the whole point:
///
/// * **Close Weapon Mastery has a real magnitude.** Its corpus record
///   carries no BONUS token, but its `DESC:` names a computable die
///   ("the unarmed strike damage of a brawler 4 levels lower"), and the
///   Brawler unarmed-damage ladder it indexes into is itself real corpus
///   data. So it grounds an actual number -- see
///   `brawler_close_weapon_mastery_die`.
///
/// * **Awesome Blow and Improved Awesome Blow have none.** Both records
///   (`acg_abilities_class.lst:899` and `:900`) are `KEY` + `CATEGORY` +
///   `TYPE` + `DESC` + `SOURCEPAGE` and nothing else -- verified field by
///   field, not by a filtered grep. Their whole benefit is a resolution
///   at the table. They are therefore grounded as **bounded grant-only
///   identity records** (`value: 0`) quoting the real corpus DESC, the
///   same idiom Sorcerer's Arcane Apotheosis and Rogue's Master Strike
///   already use in this file.
///
/// They are deliberately **not** routed through
/// `description_completion::feat_description_completion`, even though
/// they are exactly the zero-magnitude shape that module was built for.
/// That module certifies a FEAT's text reaches the player via the Feats
/// tab, and it is checked against `input.chosen.selected_feats`. These
/// are CLASS features: there is no class-feature description surface in
/// the shipped app at all (the Actions tab renders hardcoded display
/// names from `characterProgression.ts`, with no description field and
/// no corpus text behind it), so claiming their text reaches the player
/// would be precisely the unearned `success: true` that module exists to
/// prevent. The distinction is already recorded at Arcane Apotheosis.
///
/// Note the near-miss: an `Improved Awesome Blow` row DOES exist in the
/// ACG **feat** catalog (`rules_tables/acg/feat_data/combat.rs`), with a
/// real description. It is a different record -- the feat, whose own
/// prerequisite string points AT `Brawler ~ Awesome Blow` -- and routing
/// the class feature through it would ground the wrong thing.
pub(super) fn ground_brawler_close_weapon_mastery_and_awesome_blow(
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if let Some((dice_count, die_face)) = brawler_close_weapon_mastery_die(level) {
        let source_level = level - BRAWLER_CLOSE_WEAPON_MASTERY_LEVEL_OFFSET;
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.close_weapon_mastery_base_damage_die".to_owned(),
            value: die_face,
            detail: format!(
                "Brawler level {level} Close Weapon Mastery (granted at level \
                 {BRAWLER_CLOSE_WEAPON_MASTERY_LEVEL}): while wielding a close weapon she may \
                 use the Medium-size Brawler unarmed strike damage of a brawler \
                 {BRAWLER_CLOSE_WEAPON_MASTERY_LEVEL_OFFSET} levels lower -- level \
                 {source_level}, i.e. {dice_count}d{die_face} -- instead of that weapon's base \
                 damage. This record grounds the die FACE ({die_face}); the die COUNT is its own \
                 facet below, the same two-facet split Monk's unarmed strike die already uses. \
                 The corpus record carries no BONUS token, so the magnitude comes from its DESC \
                 read together with the live `Brawler Unarmed Damage LVL <N> (Medium)` ladder, \
                 and is independently corroborated by the record's own two .MOD lines. Grounds \
                 the substituted die only: this codebase computes no close-weapon attack, so \
                 nothing consumes it yet, and the \"must be declared before the attack roll\" \
                 timing clause is a table-side resolution with no magnitude"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.close_weapon_mastery_base_damage_die_count".to_owned(),
            value: dice_count,
            detail: format!(
                "Brawler level {level} Close Weapon Mastery die count: {dice_count} \
                 (full die {dice_count}d{die_face}, read from the level-{source_level} band of \
                 the Brawler unarmed damage ladder). Carried as its own facet because a single \
                 `value` cannot express `{dice_count}d{die_face}` -- dropping the count would \
                 silently report {die_face} damage where the real answer is \
                 {dice_count}d{die_face}"
            ),
        });
    }

    if level >= BRAWLER_AWESOME_BLOW_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.awesome_blow_grant".to_owned(),
            value: 0,
            detail: format!(
                "Brawler level {level} Awesome Blow, granted at level \
                 {BRAWLER_AWESOME_BLOW_LEVEL} (corpus KEY:Brawler ~ Awesome Blow): \"The brawler \
                 can as a standard action perform an awesome blow combat maneuver against a \
                 corporeal creature of her size or smaller. If the combat maneuver check \
                 succeeds, the opponent takes damage as if the brawler hit it with the close \
                 weapon she is wielding or an unarmed strike, it is knocked flying 10 feet in a \
                 direction of the brawler's choice, and it falls prone. The brawler can only push \
                 the opponent in a straight line, and the opponent can't move closer to the \
                 brawler than the square it started in. If an obstacle prevents the completion of \
                 the opponent's move, the opponent and the obstacle each take 1d6 points of \
                 damage, and the opponent is knocked prone in the space adjacent to the obstacle. \
                 (Unlike the Awesome Blow monster feat, the brawler can be of any size to use \
                 this ability.)\" This is a bounded grant-only identity record (value 0, \
                 non-fabricated): the record's complete token list is KEY, CATEGORY, TYPE, DESC \
                 and SOURCEPAGE -- it carries no BONUS, no DEFINE and no ADD, so there is no \
                 magnitude to compute, now or ever. The 10-foot push and the 1d6 obstacle damage \
                 are fixed rules constants inside the resolution, not per-character quantities \
                 this engine derives, and no combat-maneuver resolution engine exists here to \
                 apply them against"
            ),
        });
    }

    if level >= BRAWLER_IMPROVED_AWESOME_BLOW_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.brawler.improved_awesome_blow_grant".to_owned(),
            value: 0,
            detail: format!(
                "Brawler level {level} Improved Awesome Blow, granted at level \
                 {BRAWLER_IMPROVED_AWESOME_BLOW_LEVEL} (corpus \
                 KEY:Brawler ~ Improved Awesome Blow): \"The brawler can use her awesome blow \
                 ability as an attack rather than as a standard action. She may use it on \
                 creatures of any size. If the maneuver roll is a natural 20, the brawler can \
                 immediately attempt to confirm the critical by rolling another combat maneuver \
                 check with all the same modifiers as the one just rolled; if the confirmation \
                 roll is successful, the attack deals double damage, and the damage from hitting \
                 an obstacle (if any) is also doubled.\" This is a bounded grant-only identity \
                 record (value 0, non-fabricated) on the same footing as Awesome Blow above: the \
                 record's complete token list is KEY, CATEGORY, TYPE, DESC and SOURCEPAGE, with \
                 no BONUS/DEFINE/ADD anywhere. Its entire content is an action-economy upgrade \
                 and a critical-confirmation rule -- both resolutions, neither a magnitude. This \
                 is the class feature, NOT the separately-catalogued `Improved Awesome Blow` ACG \
                 feat that shares its display name and merely names this record as a prerequisite"
            ),
        });
    }
}

/// narrower one naming only what's genuinely still missing).
pub(super) fn ground_brawler_ac_bonus_and_defer_the_rest(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    ground_brawler_remaining_features(input, level, explanations);

    let ac_bonus = brawler_ac_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.brawler.ac_bonus".to_owned(),
        value: ac_bonus,
        detail: format!(
            "Brawler level {level} AC Bonus: dodge bonus to Armor Class while wearing light or \
             no armor, (level>3) + (level>8) + (level>12) + (level>17) = {ac_bonus}. The \
             \"not wearing Medium/Heavy armor\" precondition is provably vacuous in this \
             codebase (no Medium- or Heavy-armor item id exists anywhere for any class to \
             equip, and the shared combat-baseline posture requires Chain Shirt, itself light \
             armor, EquippedActive unconditionally), so this value is grounded unconditionally \
             on class ownership and level alone -- see apply_brawler_ac_bonus_to_combat_baseline \
             for its integration into the shared Armor Class total"
        ),
    });
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.brawler.helpless_or_immobilized_not_modeled".to_owned(),
        message: "PF1 Brawler AC Bonus is lost while helpless or immobilized: this codebase has \
                   no transient combat-state representation for either condition (the same gap \
                   category as Barbarian Rage's own post-rage Fatigue), so the bonus above is \
                   always computed as though neither condition applies. Named honestly rather \
                   than silently modeled or silently dropped"
            .to_owned(),
        claim_blocking: false,
    });

    let effective_intelligence =
        brawler_cunning_effective_intelligence_score(input.chosen.ability_scores.intelligence);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.brawler.cunning_effective_intelligence_for_combat_feats".to_owned(),
        value: effective_intelligence,
        detail: format!(
            "Brawler's Cunning: for the purpose of meeting combat-feat prerequisites, an \
             Intelligence score below 13 counts as 13. Real Intelligence score \
             {} -> effective {effective_intelligence} for combat-feat prerequisite checks. \
             Grounds only the flat effective-score fact; this codebase's feat_prereqs modules \
             do not check ability scores as a prerequisite type, so no actual feat-prerequisite \
             resolution is computed against it",
            input.chosen.ability_scores.intelligence
        ),
    });

    match brawler_strike_progression_tier(level) {
        None => {
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.brawler.strike_not_yet_gained".to_owned(),
                value: 0,
                detail: format!(
                    "Brawler level {level} has not yet gained Brawler's Strike (granted \
                     starting at level 5): a genuinely valid PF1 posture for a 1st-4th-level \
                     Brawler, not a gap"
                ),
            });
        }
        Some(tier) => {
            let dr_bypass = if tier == 1 {
                "magic"
            } else {
                "magic, cold iron, and silver"
            };
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.brawler.strike_dr_bypass".to_owned(),
                value: tier,
                detail: format!(
                    "Brawler level {level} Brawler's Strike (progression tier {tier}): unarmed \
                     strikes are treated as {dr_bypass} weapons for the purpose of overcoming \
                     damage reduction. Grounds only the flat DR-bypass fact; this codebase \
                     computes no damage-reduction resolution to apply it against"
                ),
            });
            if tier >= 3 {
                diagnostics.push(ComputationDiagnostic {
                    id: "class_feature.acg.brawler.strike_alignment_selection.unmodeled"
                        .to_owned(),
                    message: "Brawler's Strike Alignment Selection (the chooser unlocked at \
                         progression tier 3+, level 12) is not modeled: which alignment \
                         (lawful/chaotic/good/evil) the DR-bypass gains is a real chooser this \
                         codebase does not implement. This does not block the otherwise-valid \
                         magic/cold-iron/silver DR-bypass fact grounded above"
                        .to_owned(),
                    claim_blocking: false,
                });
            }
        }
    }

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.brawler.other_features_deferred.unsupported".to_owned(),
        message: format!(
            "{BRAWLER_CLASS_ID} now grounds every named feature on its corpus class table: the \
             base-attack-bonus/base-save chassis pillar, its class-skill list, AC Bonus, \
             Brawler's Cunning, Brawler's Strike, Brawler's Flurry, Knockout, Martial \
             Flexibility, Martial Training, Bonus Feats, Maneuver Training, and -- newly, task \
             #91 -- Close Weapon Mastery's substituted damage die plus Awesome Blow and Improved \
             Awesome Blow as bounded grant-only identity records. This diagnostic is therefore no \
             longer claim-blocking. It is retained, rather than deleted, to carry the honest \
             remainder: what stays deferred is EXECUTION, not magnitude. Specifically -- no \
             combat-maneuver resolution engine exists here, so Awesome Blow's push and Maneuver \
             Training's and Knockout's opponent-directed effects compute against no target; \
             Martial Flexibility grounds its pool but not the borrow-a-combat-feat chooser; \
             Brawler's Strike's DR bypass grounds as a flat fact with no damage-reduction \
             resolution to apply it to; Close Weapon Mastery's die is grounded but no \
             close-weapon attack consumes it; and Maneuver Training is narrowed to the canonical \
             Bull Rush pick with the other nine maneuvers deferred. Each of those is a missing \
             consumer for a correctly-derived number, which under this repo's standalone-fact \
             grounding bar does not block the number itself. Awesome Blow and Improved Awesome \
             Blow carry no corpus BONUS/DEFINE/ADD token of any kind -- verified field by field \
             on `acg_abilities_class.lst:899` and `:900` -- so they have no magnitude that could \
             ever be computed, and are grounded by quoting their real rulebook text rather than \
             by fabricating a number. They are NOT claimed to be text-complete-to-the-player: no \
             class-feature description surface exists in the shipped app, so that stronger claim \
             is deliberately not made. This message previously listed Flurry, Knockout, Martial \
             Flexibility, Martial Training, Bonus Feats and Maneuver Training as ungrounded -- \
             all six are in fact grounded (task #5) -- and then listed Awesome Blow, Improved \
             Awesome Blow and Close Weapon Mastery, all three of which are now grounded too"
        ),
        claim_blocking: false,
    });
}

/// Applies Brawler's AC Bonus dodge bonus to `base_armor_class` when
/// `input` is a Brawler (v0.6 alpha swarm, risks item 8, third APG/ACG
/// closure). Unlike Barbarian/Skald/Bloodrager's own AC penalties (which
/// are applied inline in `compute_combat_baseline` via an
/// `if active_<x>_bonus(...).is_some() { PENALTY } else { 0 }` shape,
/// since those are activation-gated and either 0 or a single fixed
/// value), Brawler's own bonus is level-dependent and always-on given
/// class ownership, so this is a small helper rather than an inline
/// ternary, called directly from `compute_combat_baseline`.
pub(super) fn apply_brawler_ac_bonus_to_combat_baseline(input: &CharacterInput) -> i16 {
    active_brawler_ac_bonus(input).unwrap_or(0)
}

/// Fighter's own level within `input.chosen.class_levels`, whether single-class
/// or part of a supported Fighter+Wizard multiclass mix (SD-21 E7.30).
///
/// Unlike `supported_fighter_level` (which requires a single-element slice and
/// stays the gate for the base-attack-bonus / base-save chassis pillar and for
/// `compute_combat_baseline` / `compute_selected_skill_modifiers`, both still
/// Fighter-single-class-only per Epic 6's remaining scope), this reconciles
/// Fighter's own per-class feature explanations (Bravery, bonus-feat
/// progression) to keep firing once another class joins the mix, using
/// Fighter's own sub-level rather than requiring the whole mix to be
/// single-class. Returns `None` for a multiclass mix that is not itself a
/// supported combination (mirroring `is_supported_multiclass_mix`), so
/// Fighter's feature explanations never surface ahead of that mix's own base
/// chassis becoming genuinely supported -- and returns `None` when Fighter is
/// absent from the mix entirely, so introducing another class never makes
/// Fighter's features appear for a build that isn't actually part-Fighter.
pub(crate) fn fighter_level_in_mix(input: &CharacterInput) -> Option<u8> {
    if let Some(level) = supported_fighter_level(input) {
        return Some(level);
    }
    if !is_supported_multiclass_mix(input) {
        return None;
    }
    input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == FIGHTER_CLASS_ID)
        .map(|class_level| class_level.level)
}

/// Make the bounded Fighter milestone class features for this slice explicit rather
/// than leaving them incidental: the level-2 bonus-feat progression seam and the
/// level-3 armor-training seam.
///
/// This adds no general feat-effect or prerequisite engine. The level-2 bonus-feat
/// seam names the chosen selection only and contributes no computed mechanical value.
/// The level-3 armor-training seam names the concrete armor-check-penalty reduction
/// and maximum-Dexterity increase that the bounded selected-skill and armor-class
/// outputs already apply, so the derived-output change is legible instead of folklore.
///
/// SD-21 E7.30: gates on `fighter_level_in_mix` (not `supported_fighter_level`
/// directly) so these grants keep firing, using Fighter's own sub-level, once a
/// supported second class (Wizard) joins the mix -- reconciling Fighter's
/// feature integration with Epic 7's multiclass dispatch instead of silently
/// dropping Fighter's already-earned features the moment the mix stops being
/// single-class.
pub(super) fn explain_fighter_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = fighter_level_in_mix(input) else {
        return;
    };

    // SD28-C4.8, class two: Fighter's own `FighterBravery` slot is named as
    // replaced by several real, ingested APG archetypes (Archer, Free Hand
    // Fighter, Mobile Fighter, Phalanx Soldier, Polearm Master, Roughrider,
    // Savage Warrior, and others -- confirmed against the real catalog rows,
    // not assumed from one). Bravery previously computed unconditionally for
    // every Fighter; a genuinely selected archetype that claims this slot
    // now supersedes it, the same disposition Alchemist's Poison Resistance
    // already established for `archetype_claims_slot`'s first consumer.
    if let Some(archetype_name) =
        archetype_resolver::archetype_claiming_slot(input, "Fighter", "FighterBravery")
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.bravery".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level {level} Bravery: superseded by the selected {archetype_name} \
                 archetype, which replaces this base-class slot -- the base Will-vs-fear \
                 progression does not apply. {archetype_name}'s own replacement feature (if \
                 any) is not separately computed here"
            ),
        });
    } else {
        let bravery_bonus = fighter_bravery_bonus(level);
        if bravery_bonus > 0 {
            explanations.push(ComputationExplanation {
                id: "class_feature.fighter.bravery".to_owned(),
                value: bravery_bonus,
                detail: format!(
                    "Fighter level {FIGHTER_BRAVERY_LEVEL} Bravery (cr_abilities_class.lst Fighter; \
                     +1 at level {FIGHTER_BRAVERY_LEVEL} and another +1 every \
                     {FIGHTER_BRAVERY_RANK_LEVEL_STRIDE} Fighter levels thereafter): grants \
                     +{bravery_bonus} to Will saves against fear. This is a flat, non-fabricated \
                     bonus magnitude only — no fear-condition or Will-save-resolution engine exists \
                     anywhere in this codebase, so this bonus is never folded into the unconditional \
                     Will save total"
                ),
            });
        }
    }

    if level >= 2
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_2_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 2 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. This slice grounds the bonus-feat slot, not a \
                     general feat-effect or prerequisite engine, so it contributes no computed \
                     mechanical value (+0)"
            ),
        });
    }

    if level >= 4
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_4_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_4_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 4 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_4_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. This slice grounds the bonus-feat slot, not a \
                     general feat-effect or prerequisite engine, so it contributes no computed \
                     mechanical value (+0)"
            ),
        });
    }

    if level >= 6
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_6_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_6_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 6 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_6_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. This slice grounds the bonus-feat slot, not a \
                     general feat-effect or prerequisite engine, so it contributes no computed \
                     mechanical value (+0)"
            ),
        });
    }

    if level >= 8
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_8_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_8_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 8 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_8_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. This slice grounds the bonus-feat slot, not a \
                     general feat-effect or prerequisite engine, so it contributes no computed \
                     mechanical value (+0)"
            ),
        });
    }

    if level >= 10
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_10_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_10_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 10 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_10_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. The canonical Greater Weapon Focus \
                     selection's prerequisites (Weapon Focus (longsword) and fighter level 8) are \
                     honestly met by the canonical loadout. This slice grounds the bonus-feat \
                     slot, not a general feat-effect or prerequisite engine, so it contributes no \
                     computed mechanical value (+0)"
            ),
        });
    }

    if level >= 12
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_12_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_12_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 12 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_12_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. The canonical Weapon Specialization \
                     selection's prerequisites (fighter level 4 and Weapon Focus with the chosen \
                     weapon) are honestly met by the canonical loadout. This slice grounds the \
                     bonus-feat slot, not a general feat-effect or prerequisite engine, so it \
                     contributes no computed mechanical value (+0)"
            ),
        });
    }

    if level >= 14
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_14_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_14_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 14 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_14_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. The canonical Greater Weapon \
                     Specialization selection's prerequisites (fighter level 12, Weapon Focus \
                     and Weapon Specialization with the chosen weapon) are honestly met by the \
                     canonical loadout. This slice grounds the bonus-feat slot, not a general \
                     feat-effect or prerequisite engine, so it contributes no computed \
                     mechanical value (+0)"
            ),
        });
    }

    if level >= 16
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_16_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_16_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 16 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_16_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. The canonical Critical Focus selection's \
                     prerequisite (base attack bonus +9) is honestly met by the canonical \
                     loadout. This slice grounds the bonus-feat slot, not a general feat-effect \
                     or prerequisite engine, so it contributes no computed mechanical value (+0)"
            ),
        });
    }

    if level >= 18
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_18_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_18_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 18 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_18_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. The canonical Staggering Critical \
                     selection's prerequisites (Critical Focus and base attack bonus +13) are \
                     honestly met by the canonical loadout: Critical Focus is the level-16 \
                     fighter bonus feat and the level-18 base attack bonus is +18. This slice \
                     grounds the bonus-feat slot, not a general feat-effect or prerequisite \
                     engine, so it contributes no computed mechanical value (+0)"
            ),
        });
    }

    if level >= 20
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_20_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_20_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 20 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_20_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. The canonical Critical Mastery selection's \
                     prerequisites (two other critical feats) are honestly met by the canonical \
                     loadout: Improved Critical, Critical Focus, and Staggering Critical are all \
                     already-selected fighter bonus feats. This slice grounds the bonus-feat \
                     slot, not a general feat-effect or prerequisite engine, so it contributes no \
                     computed mechanical value (+0)"
            ),
        });
    }

    let armor_training = fighter_armor_training(level);
    if armor_training.rank == 4 {
        let reduced_penalty = effective_chain_shirt_armor_check_penalty(level);
        let raised_max_dex = CHAIN_SHIRT_MAX_DEX + armor_training.max_dex_increase;
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.armor_training".to_owned(),
            value: i16::from(armor_training.rank),
            detail: format!(
                "Fighter level {FIGHTER_ARMOR_TRAINING_4_LEVEL} Armor Training 4 (armor training, \
                 cr_abilities_class.lst Fighter): further reduces the worn Chain Shirt armor-check \
                 penalty by {ARMOR_TRAINING_4_ARMOR_CHECK_REDUCTION} cumulative (from \
                 {CHAIN_SHIRT_ARMOR_CHECK_PENALTY:+} to {reduced_penalty:+}, already capped at 0 by \
                 Armor Training 2, so no further reduction is visible), which raises the \
                 armor-check-penalty-affected selected skill totals (Climb, Swim) by no additional \
                 amount on this fixture, and raises the maximum Dexterity bonus by \
                 {ARMOR_TRAINING_4_MAX_DEX_INCREASE} cumulative (from {CHAIN_SHIRT_MAX_DEX} to \
                 {raised_max_dex}); on the deterministic +2 Dexterity contribution, this changes \
                 no derived armor-class value on this fixture"
            ),
        });
    } else if armor_training.rank == 3 {
        let reduced_penalty = effective_chain_shirt_armor_check_penalty(level);
        let raised_max_dex = CHAIN_SHIRT_MAX_DEX + armor_training.max_dex_increase;
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.armor_training".to_owned(),
            value: i16::from(armor_training.rank),
            detail: format!(
                "Fighter level {FIGHTER_ARMOR_TRAINING_3_LEVEL} Armor Training 3 (armor training, \
                 cr_abilities_class.lst Fighter): further reduces the worn Chain Shirt armor-check \
                 penalty by {ARMOR_TRAINING_3_ARMOR_CHECK_REDUCTION} cumulative (from \
                 {CHAIN_SHIRT_ARMOR_CHECK_PENALTY:+} to {reduced_penalty:+}, already capped at 0 by \
                 Armor Training 2, so no further reduction is visible), which raises the \
                 armor-check-penalty-affected selected skill totals (Climb, Swim) by no additional \
                 amount on this fixture, and raises the maximum Dexterity bonus by \
                 {ARMOR_TRAINING_3_MAX_DEX_INCREASE} cumulative (from {CHAIN_SHIRT_MAX_DEX} to \
                 {raised_max_dex}); on the deterministic +2 Dexterity contribution, this changes \
                 no derived armor-class value on this fixture"
            ),
        });
    } else if armor_training.rank == 2 {
        let reduced_penalty = effective_chain_shirt_armor_check_penalty(level);
        let raised_max_dex = CHAIN_SHIRT_MAX_DEX + armor_training.max_dex_increase;
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.armor_training".to_owned(),
            value: i16::from(armor_training.rank),
            detail: format!(
                "Fighter level {FIGHTER_ARMOR_TRAINING_2_LEVEL} Armor Training 2 (armor training, \
                 cr_abilities_class.lst Fighter): further reduces the worn Chain Shirt armor-check \
                 penalty by {ARMOR_TRAINING_2_ARMOR_CHECK_REDUCTION} cumulative (from \
                 {CHAIN_SHIRT_ARMOR_CHECK_PENALTY:+} to {reduced_penalty:+}), which raises the \
                 armor-check-penalty-affected selected skill totals (Climb, Swim) by the same \
                 amount, and raises the maximum Dexterity bonus by \
                 {ARMOR_TRAINING_2_MAX_DEX_INCREASE} cumulative (from {CHAIN_SHIRT_MAX_DEX} to \
                 {raised_max_dex}); on the deterministic +2 Dexterity contribution, this changes \
                 no derived armor-class value on this fixture"
            ),
        });
    } else if armor_training.rank == 1 {
        let reduced_penalty = effective_chain_shirt_armor_check_penalty(level);
        let raised_max_dex = CHAIN_SHIRT_MAX_DEX + armor_training.max_dex_increase;
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.armor_training".to_owned(),
            value: i16::from(armor_training.rank),
            detail: format!(
                "Fighter level {FIGHTER_ARMOR_TRAINING_1_LEVEL} Armor Training 1 (armor training, \
                 cr_abilities_class.lst Fighter): reduces the worn Chain Shirt armor-check penalty by \
                 {ARMOR_TRAINING_1_ARMOR_CHECK_REDUCTION} (from {CHAIN_SHIRT_ARMOR_CHECK_PENALTY:+} to \
                 {reduced_penalty:+}) and raises the maximum Dexterity bonus by \
                 {ARMOR_TRAINING_1_MAX_DEX_INCREASE} (from {CHAIN_SHIRT_MAX_DEX} to {raised_max_dex})"
            ),
        });
    }

    // `AT-34-E3-001` (mechanism 3 continuation, cycle 9): the four blocks
    // below used to check `choice_selection(...) == Some(<one hardcoded
    // canonical group>)` per tier. Generalized to
    // `weapon_training_group_name_for_selection(...)`, which accepts any of
    // the 14 real PF1 weapon-training groups (`WEAPON_TRAINING_GROUPS`'s
    // own doc comment states why this is the real rule, not a relaxation).
    // `fighter_weapon_training_attack_bonus` itself is UNCHANGED (still
    // folds into the baseline total only for Heavy Blades, since only that
    // group covers the deterministic Longsword) -- these explanation
    // records are a separate, additive surface, exactly like the
    // pre-existing tier 2-4 "explanation-only" idiom this cycle widens.
    let rank = fighter_weapon_training_rank(level);
    if rank > 0 {
        let rank_level = if rank >= 4 {
            FIGHTER_WEAPON_TRAINING_1_LEVEL + 3 * FIGHTER_WEAPON_TRAINING_RANK_LEVEL_STRIDE
        } else if rank >= 3 {
            FIGHTER_WEAPON_TRAINING_1_LEVEL + 2 * FIGHTER_WEAPON_TRAINING_RANK_LEVEL_STRIDE
        } else if rank >= 2 {
            FIGHTER_WEAPON_TRAINING_1_LEVEL + FIGHTER_WEAPON_TRAINING_RANK_LEVEL_STRIDE
        } else {
            FIGHTER_WEAPON_TRAINING_1_LEVEL
        };

        // Tier 1 (level 5): the first chosen weapon group. Folds into the
        // baseline melee attack bonus only for Heavy Blades (the
        // deterministic Longsword's own group); any other of the 14 groups
        // is a real, non-fabricated +rank grant that this seam surfaces as
        // explanation-only, since no other weapon is in the deterministic
        // loadout for it to apply to.
        if let Some(selection) = choice_selection(input, FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID)
            && let Some(group_name) = weapon_training_group_name_for_selection(selection) {
                let first_group_bonus = rank;
                let detail = if selection == HEAVY_BLADES_GROUP_SELECTION {
                    format!(
                        "Fighter level {rank_level} Weapon Training {rank} (weapon training, \
                         cr_abilities_class.lst Fighter; rank = 1 + (level - 5) / 4): the first \
                         chosen weapon group \
                         ({FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID} -> {selection}) grants \
                         +{first_group_bonus} to attack rolls with weapons of that group, which \
                         the deterministic Longsword falls under; this +{first_group_bonus} is \
                         already folded into the baseline melee attack bonus. Weapon Training \
                         also grants +{first_group_bonus} to damage rolls with weapons of that \
                         group, but no damage total is computed anywhere in this codebase for \
                         any Fighter level, so the damage-roll half stays explicitly unproven \
                         rather than silently omitted"
                    )
                } else {
                    format!(
                        "Fighter level {rank_level} Weapon Training {rank} (weapon training, \
                         cr_abilities_class.lst Fighter; rank = 1 + (level - 5) / 4): the first \
                         chosen weapon group \
                         ({FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID} -> {selection}) grants \
                         +{first_group_bonus} to attack and damage rolls with weapons of that \
                         group. No {group_name} weapon is part of the deterministic Longsword \
                         loadout, so this seam is explanation-only: the +{first_group_bonus} is \
                         not folded into any computed total, and the baseline melee attack bonus \
                         is computed independently of this tier's own group choice"
                    )
                };
                explanations.push(ComputationExplanation {
                    id: "class_feature.fighter.weapon_training".to_owned(),
                    value: first_group_bonus,
                    detail,
                });
            }

        // Tiers 2-4 (levels 9/13/17): each later-chosen group's bonus sits
        // one point lower than the tier before it, regardless of which of
        // the 14 groups was picked -- always explanation-only, since the
        // deterministic loadout carries only one weapon (a Longsword,
        // Heavy Blades, tier 1's own group).
        const LATER_TIERS: [(u8, i16, &str, &str); 3] = [
            (
                2,
                1,
                FIGHTER_WEAPON_TRAINING_GROUP_2_CHOICE_ID,
                "class_feature.fighter.weapon_training_group_2",
            ),
            (
                3,
                2,
                FIGHTER_WEAPON_TRAINING_GROUP_3_CHOICE_ID,
                "class_feature.fighter.weapon_training_group_3",
            ),
            (
                4,
                3,
                FIGHTER_WEAPON_TRAINING_GROUP_4_CHOICE_ID,
                "class_feature.fighter.weapon_training_group_4",
            ),
        ];
        for (tier, rank_offset, choice_id, explanation_id) in LATER_TIERS {
            if rank < i16::from(tier) {
                continue;
            }
            let Some(selection) = choice_selection(input, choice_id) else {
                continue;
            };
            let Some(group_name) = weapon_training_group_name_for_selection(selection) else {
                continue;
            };
            let later_group_bonus = rank - rank_offset;
            explanations.push(ComputationExplanation {
                id: explanation_id.to_owned(),
                value: later_group_bonus,
                detail: format!(
                    "Fighter level {rank_level} Weapon Training {rank} also grants a tier-{tier} \
                     chosen weapon group ({choice_id} -> {selection}) +{later_group_bonus} to \
                     attack and damage rolls with weapons of that group. No {group_name} weapon \
                     is part of the deterministic Longsword loadout, so this seam is \
                     explanation-only: the +{later_group_bonus} is not folded into any computed \
                     total, and the baseline melee attack bonus uses only the first-group (Heavy \
                     Blades) rank"
                ),
            });
        }
    }

    // Armor Mastery (level 19, SD18 widening): a bounded flat-magnitude
    // record only, mirroring exactly how the Barbarian's own Damage
    // Reduction class feature was grounded (class_feature.barbarian.
    // damage_reduction). No damage-resolution engine and no incoming-damage
    // total exists anywhere in this codebase to apply it, and no
    // worn-armor-or-shield condition check is computed, so this grounds no
    // actual damage reduction.
    if level >= FIGHTER_ARMOR_MASTERY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.armor_mastery".to_owned(),
            value: ARMOR_MASTERY_DAMAGE_REDUCTION,
            detail: format!(
                "Fighter level {FIGHTER_ARMOR_MASTERY_LEVEL} Armor Mastery (cr_abilities_class.lst \
                 Fighter; verified against d20pfsrd.com/classes/core-classes/fighter/ and \
                 aonprd.com/ClassDisplay.aspx?ItemName=Fighter, byte-for-byte agreement: \"a \
                 fighter gains DR {ARMOR_MASTERY_DAMAGE_REDUCTION}/-- whenever he is wearing \
                 armor or using a shield\"): a bounded flat-magnitude record only (value \
                 {ARMOR_MASTERY_DAMAGE_REDUCTION}, non-fabricated), mirroring exactly how the \
                 Barbarian's own Damage Reduction class feature was grounded -- no \
                 damage-resolution engine and no incoming-damage total exists anywhere in this \
                 codebase to apply it, and no worn-armor-or-shield condition check is computed, \
                 so this grounds no actual damage reduction"
            ),
        });
    }

    // Weapon Mastery (level 20, SD18 widening, the Fighter capstone): a
    // bounded grant-only identity/magnitude record only, mirroring exactly
    // how the Fighter's own Armor Mastery class feature was grounded
    // (class_feature.fighter.armor_mastery). No critical-hit-confirmation
    // engine, no damage-multiplier-application engine, and no
    // disarm-resolution engine exists anywhere in this codebase to apply
    // it, so this grounds no actual automatic-critical-confirmation, no
    // actual damage-multiplier change, and no actual disarm immunity.
    if level >= FIGHTER_WEAPON_MASTERY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.weapon_mastery".to_owned(),
            value: WEAPON_MASTERY_CRITICAL_MULTIPLIER_INCREASE,
            detail: format!(
                "Fighter level {FIGHTER_WEAPON_MASTERY_LEVEL} Weapon Mastery \
                 (cr_abilities_class.lst Fighter; verified against \
                 d20pfsrd.com/classes/core-classes/fighter/ and \
                 aonprd.com/ClassDisplay.aspx?ItemName=Fighter, byte-for-byte agreement: \"a \
                 fighter chooses one weapon, such as the longsword, greataxe, or longbow. Any \
                 attacks made with that weapon automatically confirm all critical threats and \
                 have their damage multiplier increased by {WEAPON_MASTERY_CRITICAL_MULTIPLIER_INCREASE} \
                 (x2 becomes x3, for example). In addition, he cannot be disarmed while wielding \
                 a weapon of this type\"): a bounded grant-only identity/magnitude record only \
                 (critical-multiplier-increase value {WEAPON_MASTERY_CRITICAL_MULTIPLIER_INCREASE}, \
                 non-fabricated), mirroring exactly how the Fighter's own Armor Mastery class \
                 feature was grounded -- no critical-hit-confirmation engine, no \
                 damage-multiplier-application engine, and no disarm-resolution engine exists \
                 anywhere in this codebase to apply it, so this grounds no actual \
                 automatic-critical-confirmation, no actual damage-multiplier change, and no \
                 actual disarm immunity"
            ),
        });
    }
}

/// Ground the Fighter level-1 hit-point milestone as a standalone explanation
/// record: level-1 hit points = 10 (the maximized d10 Fighter hit die at 1st
/// character level, PF1 Core Rulebook) + the Constitution modifier already
/// computed from the raw chosen score by [`compute_ability_modifiers`].
///
/// Gated the same way the other Fighter explanation seams gate (the bounded
/// [`supported_fighter_level`] recognition), narrowed to level 1 because only
/// the level-1 hit-point value is grounded. The record is deliberately wired
/// into no view-model total and no derived combat/defense output. Still
/// unproven and named in the record detail: the favored-class +1 hp /
/// +1 skill-rank choice (no input surface exists for it), hit points at
/// levels 2+ (no average/rolled hit-die policy is grounded), and Toughness /
/// feat hit-point interplay.
///
/// SD13-E5 update: the favored-class bonus CHOICE itself (which of the two
/// legal options, +1 hp or +1 skill rank, was picked) is now recognized as a
/// standalone record by [`explain_fighter_favored_class_bonus_choice`]. That
/// record's own +1 magnitude is never wired into this hit-point total (nor
/// into any selected-skill-rank total) — the sentence above stays accurate
/// for this record specifically, which still carries no favored-class
/// contribution of its own.
pub(super) fn explain_fighter_level1_hit_points(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if supported_fighter_level(input) != Some(1) {
        return;
    }

    let constitution_modifier = ability_modifiers.constitution;
    // v0.6 alpha swarm, task #22 (2026-07-27): Toughness is the ONE
    // orphan feat_effects producer with a real computed total to land
    // on, and this record's own doc text already named "Toughness / feat
    // hit-point interplay" as its documented gap. Reads effective feats
    // so a class-granted Toughness would count too.
    let toughness_hit_points =
        crate::rules_core::feat_effects::hp_bonus_from_feats(&effective_character_feats(input));
    let hit_points =
        FIGHTER_LEVEL_1_MAX_HIT_DIE_HIT_POINTS + constitution_modifier + toughness_hit_points;
    explanations.push(ComputationExplanation {
        id: "class_chassis.fighter.level_1_hit_points".to_owned(),
        value: hit_points,
        detail: format!(
            "Fighter level-1 hit points: maximized d10 Fighter hit die at 1st level \
             ({FIGHTER_LEVEL_1_MAX_HIT_DIE_HIT_POINTS}) + Constitution modifier \
             ({constitution_modifier:+}) = {hit_points}. This is a standalone grounded \
             record wired into no view-model total. Still unproven and out of scope: the \
             favored-class +1 hp / +1 skill-rank choice (no input surface exists for it) and \
             hit points at levels 2+ (no average/rolled hit-die policy is grounded). Toughness \
             now contributes its real +{toughness_hit_points} here"
        ),
    });
}

/// Ground the Fighter level-1 favored-class bonus CHOICE as a standalone
/// recognition record, mirroring the already-landed Sorcerer bloodline choice
/// / Cleric domain choice / Druid nature-bond choice / Monk bonus-feat choice
/// recognition idiom: recognize which of the two PF1 Core Rulebook Favored
/// Class rule options (`bonus:hp` or `bonus:skill_rank`) was selected, and
/// name the rule's own genuinely flat +1 magnitude (verified against the
/// Archives of Nethys primary source: the bonus is always exactly +1
/// regardless of which option is chosen, so this is not a fabricated number).
///
/// This is recognition of the choice slot only. It never applies the +1 to
/// the level-1 hit-point total grounded by [`explain_fighter_level1_hit_points`]
/// (`class_chassis.fighter.level_1_hit_points`) nor to any selected-skill-rank
/// total — doing so would require wiring into the integrated hit-point /
/// skill-rank computation, which stays out of scope for this slice. A
/// selection present but naming neither legal option is acknowledged without
/// claiming a resolved hp/skill-rank identity, mirroring the Monk bonus-feat
/// choice's "present but unrecognized" branch. No record is emitted at all
/// when the choice slot is absent, so no favored-class input is fabricated
/// for a fixture that never selected one.
pub(super) fn explain_fighter_favored_class_bonus_choice(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if supported_fighter_level(input) != Some(1) {
        return;
    }

    let Some(selection) = choice_selection(input, FAVORED_CLASS_BONUS_CHOICE_ID) else {
        return;
    };

    let (value, detail) = if selection == FAVORED_CLASS_BONUS_HP_SELECTION {
        (
            1,
            format!(
                "Favored Class bonus choice ({FAVORED_CLASS_BONUS_CHOICE_ID} -> {selection}): \
                 PF1 Core Rulebook pg. 31 grants a character +1 hit point or +1 skill rank for \
                 each level taken in his favored class; a Human's favored class is Any, which \
                 trivially includes Fighter, so this level-1 Fighter class level qualifies. This \
                 selection chooses the +1 hit point option. This is a flat, non-fabricated bonus \
                 magnitude only (+1) — it is standalone and not applied to the level-1 hit-point \
                 total (`class_chassis.fighter.level_1_hit_points`), since that would require \
                 wiring into the integrated hit-point computation, never attempted in this \
                 codebase"
            ),
        )
    } else if selection == FAVORED_CLASS_BONUS_SKILL_RANK_SELECTION {
        (
            1,
            format!(
                "Favored Class bonus choice ({FAVORED_CLASS_BONUS_CHOICE_ID} -> {selection}): \
                 PF1 Core Rulebook pg. 31 grants a character +1 hit point or +1 skill rank for \
                 each level taken in his favored class; a Human's favored class is Any, which \
                 trivially includes Fighter, so this level-1 Fighter class level qualifies. This \
                 selection chooses the +1 skill rank option. This is a flat, non-fabricated bonus \
                 magnitude only (+1) — it is standalone and not applied to any selected-skill-rank \
                 total, since that would require wiring into a general class-skill-rank \
                 allocation engine, never attempted in this codebase"
            ),
        )
    } else {
        (
            0,
            format!(
                "Favored Class bonus choice slot is present ({FAVORED_CLASS_BONUS_CHOICE_ID} -> \
                 {selection}), but only the PF1 Core Rulebook's two legal options \
                 ({FAVORED_CLASS_BONUS_HP_SELECTION} or \
                 {FAVORED_CLASS_BONUS_SKILL_RANK_SELECTION}) are recognized on this bounded seam; \
                 no hp/skill-rank identity is resolved and no mechanical value is fabricated (+0)"
            ),
        )
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.fighter.favored_class_bonus_choice".to_owned(),
        value,
        detail,
    });
}

/// The canonical Human Fighter feat-choice selections this slice preserves on the
/// deterministic level-1 through level-10 seam, as `(choice_set_id,
/// canonical_selection_id)` pairs. Any named slot present but deviating from its
/// canonical selection is claim-blocked. A slot absent for the chosen level (e.g.
/// the level-2 bonus feat at level 1) is not fabricated. This same machinery
/// validates the level-5 and level-9 weapon-training-group choices, since each is
/// structurally identical to a bonus-feat slot (a named choice-set that must match
/// one canonical selection).
pub(super) const CANONICAL_FIGHTER_FEAT_CHOICES: [(&str, &str); 17] = [
    (
        LEVEL_1_CHARACTER_FEAT_CHOICE_ID,
        POWER_ATTACK_FEAT_SELECTION,
    ),
    (HUMAN_BONUS_FEAT_CHOICE_ID, DODGE_FEAT_ID),
    (
        FIGHTER_BONUS_FEAT_CHOICE_ID,
        WEAPON_FOCUS_LONGSWORD_SELECTION,
    ),
    (
        FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID,
        TOUGHNESS_FEAT_SELECTION,
    ),
    (
        FIGHTER_LEVEL_4_BONUS_FEAT_CHOICE_ID,
        CLEAVE_FEAT_SELECTION,
    ),
    (
        FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID,
        HEAVY_BLADES_GROUP_SELECTION,
    ),
    (
        FIGHTER_LEVEL_6_BONUS_FEAT_CHOICE_ID,
        COMBAT_REFLEXES_FEAT_SELECTION,
    ),
    (
        FIGHTER_LEVEL_8_BONUS_FEAT_CHOICE_ID,
        IMPROVED_CRITICAL_FEAT_SELECTION,
    ),
    (
        FIGHTER_WEAPON_TRAINING_GROUP_2_CHOICE_ID,
        BOWS_GROUP_SELECTION,
    ),
    (
        FIGHTER_LEVEL_10_BONUS_FEAT_CHOICE_ID,
        GREATER_WEAPON_FOCUS_FEAT_SELECTION,
    ),
    (
        FIGHTER_LEVEL_12_BONUS_FEAT_CHOICE_ID,
        WEAPON_SPECIALIZATION_FEAT_SELECTION,
    ),
    (
        FIGHTER_WEAPON_TRAINING_GROUP_3_CHOICE_ID,
        POLEARMS_GROUP_SELECTION,
    ),
    (
        FIGHTER_LEVEL_14_BONUS_FEAT_CHOICE_ID,
        GREATER_WEAPON_SPECIALIZATION_FEAT_SELECTION,
    ),
    (
        FIGHTER_LEVEL_16_BONUS_FEAT_CHOICE_ID,
        CRITICAL_FOCUS_FEAT_SELECTION,
    ),
    (
        FIGHTER_WEAPON_TRAINING_GROUP_4_CHOICE_ID,
        HAMMERS_GROUP_SELECTION,
    ),
    (
        FIGHTER_LEVEL_18_BONUS_FEAT_CHOICE_ID,
        STAGGERING_CRITICAL_FEAT_SELECTION,
    ),
    (
        FIGHTER_LEVEL_20_BONUS_FEAT_CHOICE_ID,
        CRITICAL_MASTERY_FEAT_SELECTION,
    ),
];

/// Claim-block non-canonical feat-choice mutations on the deterministic Fighter
/// levels 1-20 seam, while preserving the accepted canonical selections exactly.
///
/// This is deliberately not a general feat legality or prerequisite engine. It only knows
/// the exact accepted deterministic feat-choice selections on the bounded Fighter
/// seam. When one of those named choice slots is present but deviates from its canonical
/// selection, it emits a claim-blocking diagnostic that names the offending choice identity
/// and states plainly that alternative feat/prerequisite legality is outside this bounded
/// proof without a general engine — instead of letting the non-canonical build ride through
/// as a fabricated computed success.
///
/// v0.6 alpha swarm (real gap found via frontend's ceiling/limit sweep): this used to
/// return early for any non-Human Fighter, on the documented assumption that
/// `unmet_combat_posture_conditions` already claim-blocks every other case upstream. That
/// assumption was wrong -- `unmet_combat_posture_conditions` only checks the level-1
/// `FIGHTER_BONUS_FEAT_CHOICE_ID` slot; `CANONICAL_FIGHTER_FEAT_CHOICES` covers 17 slots
/// (levels 2/4/6/8/10/12/14/16/18/20 bonus feats plus 4 weapon training groups), so a
/// non-Human Fighter submitting a non-canonical selection for any of the other 14+ slots
/// (reachable today via `level_up_character`'s caller-supplied `additional_choices`, no
/// canonical-value validation anywhere else) previously reached `Computed` silently
/// unchecked. Confirmed empirically (an Elf Fighter level 2 with a wrong
/// `choice:fighter_bonus_feat_2` selection produced zero claim-blocking diagnostics) before
/// removing the race gate. Fighter's own bonus-feat/weapon-training choices are Fighter
/// class features, not Human-specific -- the one race-coupled entry in the table
/// (`HUMAN_BONUS_FEAT_CHOICE_ID`) is self-limiting: that choice slot is never seeded for a
/// non-Human character in the first place, so the loop's existing "slot absent, skip"
/// handling (`let Some(selection) = ... else { continue; }`, no change needed) already
/// covers it correctly for every race.
///
/// It runs only for a supported single-class Fighter (levels 1-20); any other posture
/// is already claim-blocked upstream and is left untouched here. It grounds no alternative
/// feat effect and does not touch the read-only canonical Human ability-bonus target.
pub(super) fn validate_fighter_feat_choice_legality(
    input: &CharacterInput,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // v0.6 alpha swarm (real gap, second one found in this same function):
    // `supported_fighter_level` is single-class-only (matches `[class_level]`
    // exactly), so this used to also skip every Fighter+X multiclass mix
    // entirely -- regardless of race, even Human -- since a multiclass
    // Fighter never satisfies that single-class match. Confirmed empirically
    // (a Human Fighter1/Rogue3 with a wrong level-1 bonus-feat choice
    // produced zero claim-blocking diagnostics) before switching to
    // `fighter_level_in_mix`, the multiclass-aware equivalent already used
    // elsewhere in this file for the same reconciliation (e.g.
    // `wizard_level_in_mix`'s identical precedent). The loop body itself
    // never used the specific level value, only the presence/absence gate,
    // so no other change was needed.
    if fighter_level_in_mix(input).is_none() {
        return;
    }

    for (choice_set_id, canonical_selection) in CANONICAL_FIGHTER_FEAT_CHOICES {
        let Some(selection) = choice_selection(input, choice_set_id) else {
            // The slot is absent for this level; do not fabricate a required choice.
            continue;
        };
        if selection != canonical_selection {
            diagnostics.push(ComputationDiagnostic {
                id: format!("feat_choice.non_canonical.{choice_set_id}"),
                message: format!(
                    "feat-choice slot {choice_set_id} on the deterministic Fighter levels \
                     1-{MAX_SUPPORTED_FIGHTER_LEVEL} seam must be the canonical {canonical_selection}; \
                     chosen selection {selection} is a non-canonical feat choice. This bounded slice \
                     preserves only the accepted canonical Fighter feat-choice path and grounds \
                     no general feat-effect or prerequisite engine, so alternative feat/prerequisite \
                     legality is outside this proof and the non-canonical build is claim-blocked \
                     rather than computed as a legal build"
                ),
                claim_blocking: true,
            });
        }
    }
}

/// v0.6 alpha swarm: frontend's ceiling/limit sweep flagged (but did not
/// verify) that `validate_fighter_feat_choice_legality`'s own doc comment
/// claim -- "any other posture is already claim-blocked upstream" for
/// non-Human Fighters -- might not actually hold. Confirmed empirically it
/// didn't: `unmet_combat_posture_conditions` only checks the level-1
/// `FIGHTER_BONUS_FEAT_CHOICE_ID` slot, leaving the other 14+ entries in
/// `CANONICAL_FIGHTER_FEAT_CHOICES` (levels 2-20 bonus feats, weapon
/// training groups) completely unchecked for any non-Human Fighter.
#[cfg(test)]
mod fighter_feat_choice_legality_race_gate_tests {
    use super::compute_pilot_base_chassis;
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// The real gap this fix closes: before removing the Human-only race
    /// gate, this exact scenario produced ZERO claim-blocking diagnostics
    /// (empirically confirmed before writing the fix, not assumed) --
    /// silently accepting a non-canonical feat choice for a non-Human
    /// Fighter at a level the doc comment claimed was already covered.
    #[test]
    fn elf_fighter_level2_non_canonical_bonus_feat_is_now_claim_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.race_id = "race:elf".to_owned();
        input.chosen.class_levels[0].level = 2;
        // Non-canonical: real canonical is feat:toughness; select feat:cleave instead.
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:fighter_bonus_feat_2".to_owned(),
            selection_id: "feat:cleave".to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);

        assert!(
            computation
                .diagnostics
                .iter()
                .any(|d| d.id == "feat_choice.non_canonical.choice:fighter_bonus_feat_2"
                    && d.claim_blocking),
            "a non-Human Fighter's non-canonical level-2 bonus feat choice must now be \
             claim-blocked, matching the same treatment a Human Fighter already gets: {:?}",
            computation.diagnostics
        );
    }

    /// Positive control: a non-Human Fighter with the CANONICAL selection
    /// must NOT be claim-blocked -- this fix widens the check, it doesn't
    /// newly reject correct builds.
    #[test]
    fn elf_fighter_level2_canonical_bonus_feat_is_not_claim_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.race_id = "race:elf".to_owned();
        input.chosen.class_levels[0].level = 2;
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:fighter_bonus_feat_2".to_owned(),
            selection_id: "feat:toughness".to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);

        assert!(
            !computation
                .diagnostics
                .iter()
                .any(|d| d.id.starts_with("feat_choice.non_canonical")),
            "the canonical selection must never be claim-blocked, for any race: {:?}",
            computation.diagnostics
        );
    }
}

/// v0.6 alpha swarm: a second, independent gap found in the same function
/// while confirming the race-gate fix's coverage was actually complete --
/// `validate_fighter_feat_choice_legality` used `supported_fighter_level`
/// (single-class-only), so it also skipped every Fighter+X multiclass mix
/// entirely, regardless of race, even Human. Confirmed empirically (a Human
/// Fighter1/Rogue3 with a wrong level-1 bonus-feat choice produced zero
/// claim-blocking diagnostics) before switching to `fighter_level_in_mix`.
#[cfg(test)]
mod fighter_feat_choice_legality_multiclass_tests {
    use super::{compute_pilot_base_chassis, CharacterClassLevel, FIGHTER_CLASS_ID, ROGUE_CLASS_ID};
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    #[test]
    fn human_fighter1_rogue3_non_canonical_level1_bonus_feat_is_now_claim_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
            CharacterClassLevel { class_id: ROGUE_CLASS_ID.to_owned(), level: 3 },
        ];
        // Overwrite the canonical FIGHTER_BONUS_FEAT_CHOICE_ID selection with a
        // wrong one.
        input.chosen.selected_choices.retain(|c| c.choice_set_id != "choice:fighter_bonus_feat");
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:fighter_bonus_feat".to_owned(),
            selection_id: "feat:cleave".to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);

        assert!(
            computation
                .diagnostics
                .iter()
                .any(|d| d.id == "feat_choice.non_canonical.choice:fighter_bonus_feat"
                    && d.claim_blocking),
            "a multiclass Fighter's non-canonical bonus feat choice must now be claim-blocked, \
             matching single-class Fighter's existing treatment: {:?}",
            computation.diagnostics
        );
    }

    /// Positive control: the canonical selection in a multiclass mix must
    /// not be blocked, and the mix must still genuinely reach Computed
    /// (proving this fix didn't newly break the multiclass dispatch task 4
    /// already proved works).
    #[test]
    fn human_fighter1_rogue3_canonical_bonus_feat_still_reaches_computed() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
            CharacterClassLevel { class_id: ROGUE_CLASS_ID.to_owned(), level: 3 },
        ];

        let computation = compute_pilot_base_chassis(&input);

        assert!(
            !computation
                .diagnostics
                .iter()
                .any(|d| d.id.starts_with("feat_choice.non_canonical") && d.claim_blocking),
            "the canonical selection must never be claim-blocked in a multiclass mix: {:?}",
            computation.diagnostics
        );
    }
}

/// SD28-C4.8, class two: `archetype_resolver::archetype_claiming_slot`'s second
/// real-compute-output consumer, on a structurally different (non-resource-pool)
/// class than Alchemist. Reachability proof per `§43`: a character who genuinely
/// selects one of the real APG Fighter archetypes that names `FighterBravery` in
/// its own `replaces` list must see the base Bravery explanation change in the
/// actual compute output.
#[cfg(test)]
mod fighter_archetype_slot_reachability_tests {
    use super::{build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, FIGHTER_CLASS_ID};
    use crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID;
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_fighter_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level }];
        input
    }

    fn bravery_explanation(
        receipt: &crate::rules_core::pilot_compute::PilotHeadlessReceipt,
    ) -> &crate::rules_core::pilot_compute::ComputationExplanation {
        receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.fighter.bravery")
            .expect("Bravery must always ground an explanation, archetype or not")
    }

    /// The base case, unaffected: a level-2 Fighter with NO archetype
    /// selected grounds the real +1 Bravery.
    #[test]
    fn a_bare_fighter_grounds_the_real_bravery_progression() {
        let input = human_fighter_input(2);
        let receipt = build_pilot_headless_receipt(&input);
        let explanation = bravery_explanation(&receipt);
        assert_eq!(explanation.value, 1, "level 2 Bravery is +1: {explanation:?}");
        assert!(
            !explanation.detail.contains("superseded"),
            "a bare Fighter's own explanation must not mention an archetype supersession: \
             {explanation:?}"
        );
    }

    /// The reachability proof: the SAME level-2 Fighter, with Archer
    /// genuinely selected via the real `ARCHETYPE_CHOICE_ID` choice-set,
    /// must see the base progression superseded in the ACTUAL compute
    /// output -- not a resolver-only unit test, the real end-to-end path a
    /// player's own selection would take.
    #[test]
    fn an_archer_fighter_supersedes_the_base_bravery_in_real_compute_output() {
        let mut input = human_fighter_input(2);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Fighter Archetype ~ Archer".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        let explanation = bravery_explanation(&receipt);
        assert_eq!(
            explanation.value, 0,
            "Archer supersedes the base progression -- the base +1 must not appear: \
             {explanation:?}"
        );
        assert!(
            explanation.detail.contains("Archer"),
            "the explanation must name the actual superseding archetype, not a generic message: \
             {explanation:?}"
        );
        assert!(
            explanation.detail.contains("superseded"),
            "the explanation must say plainly that the base progression does not apply: \
             {explanation:?}"
        );
    }

    /// The negative-adjacent proof: selecting a DIFFERENT Fighter archetype
    /// that does NOT touch Bravery (Crossbowman, whose `replaces` list
    /// omits `FighterBravery`) must leave the base progression grounding
    /// exactly as if no archetype were selected -- proves the wiring is
    /// scoped to the real claimed slot, not "any archetype selected turns
    /// this off."
    #[test]
    fn an_unrelated_fighter_archetype_leaves_bravery_grounding_unchanged() {
        let mut input = human_fighter_input(2);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Fighter Archetype ~ Crossbowman".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        let explanation = bravery_explanation(&receipt);
        assert_eq!(
            explanation.value, 1,
            "Crossbowman does not touch Bravery -- the real +1 must still ground: {explanation:?}"
        );
    }
}

/// v0.6 alpha swarm, task #5 (Brawler's six remaining real features,
/// 2026-07-27).
#[cfg(test)]
mod brawler_remaining_feature_tests {
    use super::{build_pilot_headless_receipt, CharacterClassLevel, CharacterInput,
        BRAWLER_CLASS_ID, BRAWLER_CLOSE_WEAPON_MASTERY_LEVEL};
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn brawler(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: BRAWLER_CLASS_ID.to_owned(), level }];
        input
    }

    fn value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    /// Maneuver Training is set by TWO stacking corpus lines: the
    /// `(L>2)+(L>6)+(L>10)+(L>14)` line tops out at 4, and a separate
    /// `1|PREVARGT:BrawlerLVL,18` supplies the 5th. Reading only the
    /// first gives 4 at levels 19-20 instead of 5.
    ///
    /// The corpus itself corroborates this: it carries the author's
    /// comment on a rejected single-line variant saying it "never got
    /// BrawlerManeuverTraining up to 5".
    #[test]
    fn maneuver_training_count_reaches_five_via_its_second_corpus_line() {
        for (level, want) in [
            (1u8, 0i16), (2, 0), (3, 1), (6, 1), (7, 2), (10, 2), (11, 3),
            (14, 3), (15, 4), (18, 4), (19, 5), (20, 5),
        ] {
            assert_eq!(super::brawler_maneuver_training_count(level), want, "level {level}");
        }
    }

    /// The Brawler unarmed-damage ladder, pinned against the live
    /// `Brawler Unarmed Damage LVL <N> (Medium)` records
    /// (`acg_abilities_class.lst` 966/976/986/996/1006/1016), banded by
    /// the corpus's own `min(5,BrawlerLVL/4)`.
    ///
    /// The second half of this test records, rather than hides, that the
    /// ladder currently agrees with Monk's at every level. That is a true
    /// PF1 fact (Brawler advances unarmed damage on the monk's
    /// schedule), so asserting a difference here would be asserting a
    /// fiction. What the assertion pins instead is that the agreement is
    /// EXACT and deliberate -- if either corpus ladder is ever edited
    /// apart from the other, this fails and forces the reader to decide
    /// which one moved rather than letting one class silently inherit the
    /// other's number.
    #[test]
    fn brawler_unarmed_damage_ladder_matches_its_own_corpus_records() {
        for (level, want) in [
            (1u8, (1i16, 6i16)), (3, (1, 6)),
            (4, (1, 8)), (7, (1, 8)),
            (8, (1, 10)), (11, (1, 10)),
            (12, (2, 6)), (15, (2, 6)),
            (16, (2, 8)), (19, (2, 8)),
            (20, (2, 10)),
        ] {
            assert_eq!(
                super::brawler_unarmed_strike_damage_die(level),
                want,
                "brawler level {level}"
            );
        }

        for level in 1..=20u8 {
            let (brawler_count, brawler_face) = super::brawler_unarmed_strike_damage_die(level);
            let (monk_face, monk_count, _) = super::monk_unarmed_strike_damage_die(level);
            assert_eq!(
                (brawler_count, brawler_face),
                (monk_count, monk_face),
                "level {level}: Brawler and Monk unarmed ladders are expected to agree exactly. \
                 If this fails, one corpus ladder moved -- do NOT make either class read the \
                 other's table to restore agreement"
            );
        }
    }

    /// Close Weapon Mastery reads the die of a brawler FOUR LEVELS
    /// LOWER, and is absent before its 5th-level grant.
    ///
    /// The two assertions at levels 5-7 and 8-11 are exactly the two
    /// answers the corpus hardcodes in Close Weapon Mastery's own `.MOD`
    /// lines (1d6 at `BrawlerUnarmedDamageProgression,1`; 1d8 at `,2`),
    /// so this pins the derived ladder against an independent corpus
    /// statement rather than against itself.
    #[test]
    fn close_weapon_mastery_reads_the_die_four_levels_lower() {
        for level in 1..BRAWLER_CLOSE_WEAPON_MASTERY_LEVEL {
            assert_eq!(
                super::brawler_close_weapon_mastery_die(level),
                None,
                "Close Weapon Mastery must not exist at level {level}"
            );
        }
        for (level, want) in [
            (5u8, (1i16, 6i16)), (7, (1, 6)),
            (8, (1, 8)), (11, (1, 8)),
            (12, (1, 10)),
            (16, (2, 6)),
            (20, (2, 8)),
        ] {
            assert_eq!(
                super::brawler_close_weapon_mastery_die(level),
                Some(want),
                "brawler level {level}"
            );
        }

        assert_eq!(
            value(&brawler(4), "class_feature.acg.brawler.close_weapon_mastery_base_damage_die"),
            None,
        );
        assert_eq!(
            value(&brawler(5), "class_feature.acg.brawler.close_weapon_mastery_base_damage_die"),
            Some(6),
        );
        assert_eq!(
            value(
                &brawler(20),
                "class_feature.acg.brawler.close_weapon_mastery_base_damage_die",
            ),
            Some(8),
        );
        // The count facet is load-bearing: without it a reader sees "8"
        // and cannot tell 1d8 from 2d8.
        assert_eq!(
            value(
                &brawler(20),
                "class_feature.acg.brawler.close_weapon_mastery_base_damage_die_count",
            ),
            Some(2),
        );
    }

    /// Awesome Blow (16th) and Improved Awesome Blow (20th) are
    /// zero-magnitude corpus records -- KEY/CATEGORY/TYPE/DESC/SOURCEPAGE
    /// and nothing else. They ground as bounded grant-only identity
    /// records at their own gates, value 0, and must not appear early.
    #[test]
    fn awesome_blow_records_ground_only_at_their_corpus_gates() {
        let awesome = "class_feature.acg.brawler.awesome_blow_grant";
        let improved = "class_feature.acg.brawler.improved_awesome_blow_grant";

        assert_eq!(value(&brawler(15), awesome), None);
        assert_eq!(value(&brawler(16), awesome), Some(0));
        assert_eq!(value(&brawler(20), awesome), Some(0));

        assert_eq!(value(&brawler(19), improved), None);
        assert_eq!(value(&brawler(20), improved), Some(0));

        // The identity record must carry the real corpus text, not a
        // placeholder -- that quoted text is the entire justification for
        // grounding something whose value is 0.
        let receipt = build_pilot_headless_receipt(&brawler(20));
        let detail = &receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == awesome)
            .expect("Awesome Blow grounds at 20")
            .detail;
        assert!(
            detail.contains("knocked flying 10 feet")
                && detail.contains("KEY:Brawler ~ Awesome Blow"),
            "the identity record must quote the real corpus DESC and cite its namespaced key: \
             {detail}"
        );
    }

    /// Knockout's stat bonus is `max(STR,DEX)` -- BARE tokens, i.e.
    /// ability MODIFIERS, unlike Brawler's Cunning's explicit
    /// `INTSCORE`. The corpus disambiguates deliberately, and a DC built
    /// from scores instead of modifiers would be wildly wrong.
    #[test]
    fn knockout_uses_ability_modifiers_not_scores_unlike_brawlers_cunning() {
        // Fixture STR 16 (+3), DEX 14 (+2) -> max modifier 3.
        assert_eq!(super::brawler_knockout_stat_bonus(3, 2), 3);
        assert_eq!(super::brawler_knockout_stat_bonus(-1, 4), 4);
        // Level 4 DC: 4/2 + 10 + 3 = 15. A score-based read would give 26.
        assert_eq!(super::brawler_knockout_dc(4, 3), 15);
        assert_eq!(super::brawler_knockout_dc(20, 5), 25);
        // Cunning still floors a SCORE at 13 -- the two must not converge.
        assert_eq!(super::brawler_cunning_effective_intelligence_score(10), 13);
    }

    /// Knockout is granted at 4th, not 5th, and `(L+2)/6` yields exactly
    /// 1 there -- the corpus is internally consistent.
    #[test]
    fn knockout_uses_per_day_matches_its_fourth_level_grant() {
        for (level, want) in [(4u8, 1i16), (9, 1), (10, 2), (16, 3), (20, 3)] {
            assert_eq!(super::brawler_knockout_uses_per_day(level), want, "level {level}");
        }
        assert_eq!(value(&brawler(1), "class_feature.acg.brawler.knockout_dc"), None);
        assert_eq!(value(&brawler(4), "class_feature.acg.brawler.knockout_dc"), Some(15));
    }

    /// Flurry's extra attacks, and its self-applied attack penalty --
    /// a real corpus token, unlike Cavalier's DESC-only one.
    ///
    /// Note the corpus `min(...,3)` cap is NOT meaningfully covered
    /// here, and cannot be: `(20+6)/7` is already 3, so the cap never
    /// binds for any reachable level. Asserting the values is the most
    /// this test can honestly do.
    #[test]
    fn brawlers_flurry_extra_attacks_cap_at_three() {
        for (level, want) in [(2u8, 1i16), (7, 1), (8, 2), (14, 2), (15, 3), (20, 3)] {
            assert_eq!(super::brawler_flurry_extra_attacks(level), want, "level {level}");
        }
        assert_eq!(
            value(&brawler(2), "class_feature.acg.brawler.flurry_attack_penalty"),
            Some(-2)
        );
        assert_eq!(value(&brawler(1), "class_feature.acg.brawler.flurry_extra_attacks"), None);
    }

    /// Bonus Feats: `(1+level)/3`. Seven archetype `-1` deductions exist
    /// in the corpus but are provably vacuous here -- their only setter
    /// is the Wild Child archetype, and this repo ingests no Brawler
    /// archetype at all.
    #[test]
    fn bonus_feat_count_ignores_the_seven_vacuous_archetype_deductions() {
        for (level, want) in [(1u8, 0i16), (2, 1), (4, 1), (5, 2), (8, 3), (11, 4), (20, 7)] {
            assert_eq!(super::brawler_bonus_feat_count(level), want, "level {level}");
        }
    }

    /// Martial Flexibility's uses/day pool, granted 1st.
    #[test]
    fn martial_flexibility_pool_grounds_from_first_level() {
        for (level, want) in [(1u8, 4i16), (2, 4), (4, 5), (10, 8), (20, 13)] {
            assert_eq!(super::brawler_martial_flexibility_uses(level, &[]), want, "level {level}");
        }
        assert_eq!(
            value(&brawler(1), "class_feature.acg.brawler.martial_flexibility_uses_per_day"),
            Some(4)
        );
    }

    /// Martial Training grounds three level-equivalence facts. Brawler's
    /// feature shares a NAME with Alchemist's, which is a genuine no-op
    /// with zero tokens -- same name, different class, opposite verdict.
    #[test]
    fn martial_training_grounds_its_three_level_equivalence_facts() {
        for id in [
            "class_feature.acg.brawler.martial_training.fighter_level_equivalence",
            "class_feature.acg.brawler.martial_training.monk_level_equivalence",
            "class_feature.acg.brawler.martial_training.monk_feat_qualify",
        ] {
            assert_eq!(value(&brawler(7), id), Some(7), "{id}");
        }
    }

    /// Nothing leaks onto a non-Brawler.
    #[test]
    fn none_of_the_six_grounds_for_a_non_brawler() {
        let fighter = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        for frag in ["knockout", "flurry", "martial_flexibility", "martial_training",
            "maneuver_training", "bonus_feat_count"] {
            assert!(
                !build_pilot_headless_receipt(&fighter)
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.contains(&format!("brawler.{frag}"))),
                "a Fighter must not ground brawler.{frag}"
            );
        }
    }
}

