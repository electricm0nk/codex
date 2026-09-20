#[allow(unused_imports)]
pub(crate) use super::*;

// ---------------------------------------------------------------------------------------------
// SD-34 wave 43 (`decisions.md §22`'s 12-unit "small-precedented-new-compute" remainder):
// Duelist, Shadowdancer, Assassin, Loremaster. All four are real prestige classes registered
// in `prestige_class_entry_gate::is_registered` (confirmed directly against
// `tests/fixtures/rules_core/prestige-class-entry-requirements.json`), but none is a
// `ClassId`-family enum member anywhere in this file, so none can reach `compute_class_chassis`'s
// per-class dispatch chain (Skald/Bloodrager/.../Swashbuckler/... above, or the CRB
// `table_class_id` chain Paladin/Cleric use). Every function below is therefore called
// unconditionally from `compute_pilot_base_chassis` itself, keyed on the raw `class_id` string,
// the identical shape `ground_paladin_detect_evil` (immediately above) already established.
//
// Classifier reachability was checked directly (not assumed) for every one of the 12 ids below,
// the same way wave 42's own receipt did: `class_feature_exact_suffix_grounded`'s 3-segment
// `<owner>.<feature_slug>.<descriptor>` shape already recognizes an id whose second-to-last dot
// segment equals the corpus record's own feature slug, gated only on `owner` (`"duelist"`,
// `"shadowdancer"`, `"assassin"`, `"loremaster"`) matching the record's `group` text -- already
// PROVEN live for all four owners by their own pre-existing `text-complete` siblings (e.g.
// `class_feature.duelist.corpus_record.deflect_arrows`,
// `class_feature.assassin.weapon_and_armor_proficiency`,
// `class_feature.shadowdancer.weapon_and_armor_proficiency`; Loremaster has no roster/proficiency
// sibling yet, but its OWN `owner` resolution needs nothing more than the corpus's own
// `"Loremaster"` group text, which `class_feature_owner` derives generically from
// `facts.class_books`/`facts.corpus_class_names`, not from any per-class registration list). No
// `CLASS_FEATURE_ID_KNOWN_SYNONYMS` table entry and no `canonical_seeds_for()` match arm are
// needed for any of the 12 -- `src/bin/v06_work_inventory.rs` carries zero diff this cycle.
// ---------------------------------------------------------------------------------------------

/// PF1 Core Rulebook Duelist Canny Defense (`cr_abilities_class.lst:2987`,
/// `KEY:Duelist ~ Canny Defense`): `DEFINE:CannyDefenseLVL|0` /
/// `BONUS:VAR|CannyDefenseLVL|DuelistLVL` / `BONUS:COMBAT|AC|
/// max(0,min(INT,CannyDefenseLVL))|TYPE=Dodge|PREMULT:...` -- a dodge bonus
/// to AC while wearing light or no armor and wielding a melee weapon, equal
/// to the LOWER of the duelist's Intelligence bonus and her duelist level,
/// floored at 0 (a penalty Intelligence never turns this into a penalty).
/// Granted from class level 1 (`Duelist_CFP_Level,1`, matching Precise
/// Strike's own grant gate). `None` below level 1.
pub(super) fn duelist_canny_defense_dodge_bonus(level: u8, intelligence_modifier: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(0.max(intelligence_modifier.min(i16::from(level))))
}

/// PF1 Core Rulebook Duelist Improved Reaction (`cr_abilities_class.lst:2988`,
/// `KEY:Duelist ~ Improved Reaction`): `DEFINE:ImprovedReaction|0` /
/// `BONUS:VAR|ImprovedReaction|floor((DuelistLVL+4)/6)*2` -- a flat bonus
/// on initiative checks. Granted from class level 2 (`Duelist_CFP_Level,2`).
/// `None` below level 2 (the formula is genuinely 0 at level 1 anyway --
/// `floor(5/6)*2 = 0` -- but the level gate is the honest reason, not the
/// arithmetic coincidence).
pub(super) fn duelist_improved_reaction_initiative_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    let level = i16::from(level);
    Some(((level + 4) / 6) * 2)
}

/// PF1 Core Rulebook Duelist Precise Strike (`cr_abilities_class.lst:2991`,
/// `KEY:Duelist ~ Precise Strike`): `DEFINE:PreciseStrikeDamage|0` /
/// `BONUS:VAR|PreciseStrikeDamage|DuelistLVL` -- bonus weapon damage with a
/// light or one-handed piercing weapon, equal to duelist level. The
/// identical shape as ACG Swashbuckler's own `swashbuckler_precise_strike_
/// damage` (this file, above), grounded standalone for the same reason:
/// this engine computes no weapon-damage total to layer it onto. Granted
/// from class level 1. `None` below level 1.
pub(super) fn duelist_precise_strike_damage_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// PF1 Core Rulebook Duelist Elaborate Defense (`cr_abilities_class.lst:2997`,
/// `KEY:Duelist ~ Elaborate Defense`): `DEFINE:ElaborateParryLVL|0`
/// `DEFINE:ElaborateDefense|0` / `BONUS:VAR|ElaborateParryLVL|DuelistLVL`
/// then `BONUS:VAR|ElaborateDefense|ElaborateParryLVL/3` -- an additional
/// dodge bonus to AC while fighting defensively or using total defense,
/// equal to duelist level / 3. Granted from class level 7
/// (`Duelist_CFP_Level,7`). `None` below level 7.
pub(super) fn duelist_elaborate_defense_dodge_bonus(level: u8) -> Option<i16> {
    if level < 7 {
        return None;
    }
    Some(i16::from(level) / 3)
}

/// Grounds Duelist's four genuinely new-compute class features
/// (`decisions.md §22`'s 12-unit remainder). Unconditional on chassis
/// support -- Duelist has no `ClassId` enum entry, so this is called
/// directly from `compute_pilot_base_chassis`, mirroring
/// `ground_paladin_detect_evil`'s own placement and reasoning.
pub(super) fn ground_duelist_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == DUELIST_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(dodge) =
        duelist_canny_defense_dodge_bonus(level, ability_modifiers.intelligence)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.duelist.canny_defense.dodge_bonus".to_owned(),
            value: dodge,
            detail: format!(
                "Duelist level {level} Canny Defense: a +{dodge} dodge bonus to Armor Class \
                 while wearing light or no armor and wielding a melee weapon (corpus \
                 `max(0,min(INT,CannyDefenseLVL))`, this character's Intelligence modifier \
                 {int_mod:+} against duelist level {level}, floored at 0). Grounds the \
                 magnitude only: no armor-class total exists anywhere in this engine for it to \
                 layer onto, and the flat-footed/no-shield/melee-weapon preconditions are not \
                 modelled",
                int_mod = ability_modifiers.intelligence
            ),
        });
    }

    if let Some(initiative) = duelist_improved_reaction_initiative_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.duelist.improved_reaction.initiative_bonus".to_owned(),
            value: initiative,
            detail: format!(
                "Duelist level {level} Improved Reaction: a +{initiative} bonus on initiative \
                 checks (corpus `floor((DuelistLVL+4)/6)*2`), stacking with Improved Initiative. \
                 No initiative total exists anywhere in this engine, so this grounds standalone -- \
                 the same shape as Inquisitor's Cunning Initiative"
            ),
        });
    }

    if let Some(damage) = duelist_precise_strike_damage_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.duelist.precise_strike.damage_bonus".to_owned(),
            value: damage,
            detail: format!(
                "Duelist level {level} Precise Strike: +{damage} bonus damage with a light or \
                 one-handed piercing melee weapon (corpus `PreciseStrikeDamage = DuelistLVL`), \
                 the identical shape ACG Swashbuckler's own Precise Strike deed already grounds. \
                 Grounds standalone: no weapon-damage total exists anywhere in this engine"
            ),
        });
    }

    if let Some(dodge) = duelist_elaborate_defense_dodge_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.duelist.elaborate_defense.dodge_bonus".to_owned(),
            value: dodge,
            detail: format!(
                "Duelist level {level} Elaborate Defense: an additional +{dodge} dodge bonus to \
                 Armor Class while fighting defensively or using total defense (corpus \
                 `ElaborateParryLVL/3`, `ElaborateParryLVL = DuelistLVL`). Grounds standalone: \
                 this engine models no fighting-defensively/total-defense combat action for it \
                 to modify"
            ),
        });
    }
}

/// PF1 Core Rulebook Shadowdancer Shadow Illusion (`cr_abilities_class.lst:3069`,
/// `KEY:Shadowdancer ~ Shadow Illusion`): `DEFINE:ShadowIllusionLVL|0` /
/// `SPELLS:Class|TIMES=1|CASTERLEVEL=ShadowIllusionLVL|Silent Image,11+CHA`
/// / `BONUS:VAR|ShadowIllusionLVL|ShadowdancerLVL` -- a spell-like ability
/// (functions as Silent Image), caster level equal to shadowdancer level.
/// **Real corpus discrepancy, resolved deliberately** (the same discipline
/// Warpriest's own Channel Energy DC precedent already established): the
/// record's own DESC prose claims "once per day for every two shadowdancer
/// levels", but the record's own computed `SPELLS:` token is a literal
/// `TIMES=1` -- a flat, unconditional 1/day, not a formula. The literal
/// token is what this cycle transcribes, per this bundle's own
/// authoritative-token-over-DESC-prose ruling. Granted from class level 3
/// (`Shadowdancer_CFP_Level,3`). `None` below level 3.
pub(super) fn shadowdancer_shadow_illusion_caster_level(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(i16::from(level))
}

/// See [`shadowdancer_shadow_illusion_caster_level`]'s own doc comment for
/// why this is a literal `1`, not a level-derived formula.
pub(super) const SHADOWDANCER_SHADOW_ILLUSION_USES_PER_DAY: i16 = 1;

/// PF1 Core Rulebook Shadowdancer Shadow Call (`cr_abilities_class.lst:3071`,
/// `KEY:Shadowdancer ~ Shadow Call`): `DEFINE:ShadowCallLvl|0`
/// `DEFINE:ShadowCallTimes|0` / `BONUS:VAR|ShadowCallLvl|ShadowDancerLVL`
/// -- a spell-like ability (functions as Shadow Conjuration), caster level
/// equal to shadowdancer level. Granted from class level 4
/// (`Shadowdancer_CFP_Level,4`, the same grant gate as Shadow Jump). `None`
/// below level 4.
pub(super) fn shadowdancer_shadow_call_caster_level(level: u8) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(i16::from(level))
}

/// See [`shadowdancer_shadow_call_caster_level`]'s own doc comment.
/// `BONUS:VAR|ShadowCallTimes|ShadowDancerLVL/2` -- uses per day. `None`
/// below level 4, the same grant gate.
pub(super) fn shadowdancer_shadow_call_uses_per_day(level: u8) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// PF1 Core Rulebook Shadowdancer Shadow Jump (`cr_abilities_class.lst:3072`,
/// `KEY:Shadowdancer ~ Shadow Jump`): `DEFINE:ShadowJump|0`
/// `DEFINE:ShadowJumpProgression|0` / four separate, cumulative
/// `BONUS:VAR|ShadowJump|<N>|PREVARGTEQ:ShadowdancerLVL,<T>` tokens -- `20`
/// at level 4, another `20` at level 6, `40` at level 8, `80` at level 10.
/// Verified directly against the same additive-`BONUS:VAR` idiom this
/// codebase's own `alchemist_poison_resistance_bonus` already established
/// for a multi-threshold same-variable chain (each higher threshold's
/// contribution ADDS to every lower threshold already met, expressed as a
/// nested if/else returning the cumulative total): `20` (level 4-5), `40`
/// (level 6-7, `20+20`), `80` (level 8-9, `20+20+40`), `160` (level 10+,
/// `20+20+40+80`). **Real corpus discrepancy, resolved deliberately**: the
/// record's own DESC prose describes a doubling progression of `40`/`80`/
/// `160`/`320` feet (exactly double the literal token sum at every tier) --
/// the same "authoritative computed token, not DESC prose" discrepancy
/// Warpriest's Channel Energy DC precedent already resolved one way, so this
/// transcribes the literal token sum, not the DESC narrative. Granted from
/// class level 4 (`Shadowdancer_CFP_Level,4`). `None` below level 4.
pub(super) fn shadowdancer_shadow_jump_daily_distance_feet(level: u8) -> Option<i16> {
    if level < 4 {
        None
    } else if level < 6 {
        Some(20)
    } else if level < 8 {
        Some(40)
    } else if level < 10 {
        Some(80)
    } else {
        Some(160)
    }
}

/// PF1 Core Rulebook Shadowdancer Summon Shadow (`cr_abilities_class.lst:3070`,
/// `KEY:Shadowdancer ~ Summon Shadow`): `DEFINE:ShadowCompanionLVL|0` /
/// `BONUS:VAR|ShadowCompanionLVL|ShadowdancerLVL` -- a flat
/// level-equivalence fact (the summoned shadow companion's own effective
/// level, used elsewhere in the record's own DESC for its hit-point and
/// base-attack/save derivation, none of which this engine models). The
/// identical "ground the level-equivalence fact, not the companion's own
/// stat block" shape as ACG Swashbuckler's own `fighter_level_equivalence_
/// for_feats`. Granted from class level 3 (`Shadowdancer_CFP_Level,3`, the
/// same grant gate as Shadow Illusion). `None` below level 3.
pub(super) fn shadowdancer_summon_shadow_companion_level(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(i16::from(level))
}

/// Grounds Shadowdancer's four genuinely new-compute class features
/// (`decisions.md §22`'s 12-unit remainder). Unconditional on chassis
/// support, the same placement/reasoning as `ground_duelist_class_
/// features` above.
pub(super) fn ground_shadowdancer_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == SHADOWDANCER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(caster_level) = shadowdancer_shadow_illusion_caster_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.shadowdancer.shadow_illusion.caster_level".to_owned(),
            value: caster_level,
            detail: format!(
                "Shadowdancer level {level} Shadow Illusion: spell-like ability (functions as \
                 Silent Image), caster level {caster_level} (corpus `ShadowIllusionLVL = \
                 ShadowdancerLVL`). Grounds the caster-level fact only -- the SLA triple idiom \
                 already established by `ground_summoner_slice_a_features`: no illusion effect, \
                 spell DC, or Charisma-based save is modelled"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.shadowdancer.shadow_illusion.uses_per_day".to_owned(),
            value: SHADOWDANCER_SHADOW_ILLUSION_USES_PER_DAY,
            detail: format!(
                "Shadowdancer level {level} Shadow Illusion uses per day: \
                 {SHADOWDANCER_SHADOW_ILLUSION_USES_PER_DAY} (corpus `SPELLS:Class|TIMES=1|...` -- \
                 a literal, unconditional daily use, NOT the `floor(level/2)` the record's own \
                 DESC prose describes; see this fact's own compute function doc comment for the \
                 authoritative-token-over-DESC-prose ruling)"
            ),
        });
    }

    if let Some(caster_level) = shadowdancer_shadow_call_caster_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.shadowdancer.shadow_call.caster_level".to_owned(),
            value: caster_level,
            detail: format!(
                "Shadowdancer level {level} Shadow Call: spell-like ability (functions as Shadow \
                 Conjuration), caster level {caster_level} (corpus `ShadowCallLvl = \
                 ShadowDancerLVL`). Grounds the caster-level fact only, the same SLA-triple idiom \
                 as Shadow Illusion above"
            ),
        });
        if let Some(uses) = shadowdancer_shadow_call_uses_per_day(level) {
            explanations.push(ComputationExplanation {
                id: "class_feature.shadowdancer.shadow_call.uses_per_day".to_owned(),
                value: uses,
                detail: format!(
                    "Shadowdancer level {level} Shadow Call uses per day: {uses} (corpus \
                     `ShadowCallTimes = ShadowDancerLVL/2`). Grounds the per-day budget only"
                ),
            });
        }
    }

    if let Some(distance) = shadowdancer_shadow_jump_daily_distance_feet(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.shadowdancer.shadow_jump.daily_distance_feet".to_owned(),
            value: distance,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|ShadowJump|<N>|PREVARGTEQ:ShadowdancerLVL,<T>` tokens summed; see
                //   this fact's own compute function doc comment for the real discrepancy against
                //   the record's own DESC-prose doubling narrative, resolved by transcribing the
                //   literal token
                "Shadowdancer level {level} Shadow Jump: {distance} feet of dimension-door-like \
                 travel per day between areas of dim light or darker (corpus's own four cumulative). \
                 Grounds the daily budget only: no per-jump accounting or the dim-light precondition \
                 is modelled"
            ),
        });
    }

    if let Some(companion_level) = shadowdancer_summon_shadow_companion_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.shadowdancer.summon_shadow.companion_level".to_owned(),
            value: companion_level,
            detail: format!(
                "Shadowdancer level {level} Summon Shadow: the summoned shadow companion's own \
                 effective level, {companion_level} (corpus `ShadowCompanionLVL = \
                 ShadowdancerLVL`). Grounds the level-equivalence fact only, the same shape ACG \
                 Swashbuckler's own fighter-level-equivalence-for-feats fact uses: no companion \
                 stat block (hit points, base attack, base saves) is derived from it here"
            ),
        });
    }
}

/// PF1 Core Rulebook Assassin Save against Poisons (`cr_abilities_class.lst:2945`,
/// `KEY:Assassin ~ Save against Poisons`): `DEFINE:AssassinPoisonSaveBonus|0`
/// / `BONUS:VAR|AssassinPoisonSaveBonus|AssassinLVL/2` -- a flat bonus on
/// saving throws against poison. The formula string `AssassinLVL/2` is
/// already a literal in this repo's own test fixtures
/// (`class_feature_grant_consumer.rs:2392`). Granted from class level 2
/// (`Assassin_CFP_Level,2`). `None` below level 2.
pub(super) fn assassin_save_against_poisons_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// PF1 Core Rulebook Assassin Death Attack (`cr_abilities_class.lst:2947`,
/// `KEY:Assassin ~ Death Attack`): `BONUS:VAR|DeathAttackDC,DeathAttackDuration|
/// AssassinLVL` -- adds Assassin level to BOTH shared variables the
/// generic `Death Attack` record (`cr_abilities_class.lst:2869`,
/// `BONUS:VAR|DeathAttackDC|10+INT`) already seeds. Since PCGen `BONUS:VAR`
/// tokens on the same variable stack additively, the real save DC is the
/// SUM of both contributions: `10 + INT + AssassinLVL`, matching the
/// record's own DESC exactly ("DC 10 + the assassin's class level + the
/// assassin's Int modifier"). Granted from class level 1
/// (`Assassin_CFP_Level,1`). `None` below level 1.
pub(super) fn assassin_death_attack_save_dc(level: u8, intelligence_modifier: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 + i16::from(level) + intelligence_modifier)
}

/// See [`assassin_death_attack_save_dc`]'s own doc comment. `DeathAttackDuration
/// = AssassinLVL` -- the "+ 1 round per level of the assassin" half of the
/// paralysis duration ("1d6 rounds plus 1 round per level"); the `1d6` base
/// is dice notation this engine does not model, the same "grounds the
/// level-derived bonus, defers the dice" treatment ACG Swashbuckler's own
/// deed damage formulas already use. Granted from class level 1. `None`
/// below level 1.
pub(super) fn assassin_death_attack_duration_bonus_rounds(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// Grounds Assassin's two genuinely new-compute class features
/// (`decisions.md §22`'s 12-unit remainder). Unconditional on chassis
/// support, the same placement/reasoning as `ground_duelist_class_
/// features` above.
pub(super) fn ground_assassin_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ASSASSIN_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = assassin_save_against_poisons_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.assassin.save_against_poisons.save_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Assassin level {level} Save against Poisons: a +{bonus} bonus on saving throws \
                 against poison (corpus `AssassinPoisonSaveBonus = AssassinLVL/2`). Grounds \
                 standalone: no saving-throw total this engine computes is poison-specific"
            ),
        });
    }

    let int_mod = ability_modifiers.intelligence;
    if let Some(dc) = assassin_death_attack_save_dc(level, int_mod) {
        explanations.push(ComputationExplanation {
            id: "class_feature.assassin.death_attack.save_dc".to_owned(),
            value: dc,
            detail: format!(
                "Assassin level {level} Death Attack: Fortitude save DC {dc} (10 + assassin \
                 level {level} + Intelligence modifier {int_mod:+}, the sum of the generic Death \
                 Attack record's own `10+INT` plus this class's own `+AssassinLVL` contribution \
                 to the same shared `DeathAttackDC` variable). Grounds the DC only: the sneak- \
                 attack/3-round-study precondition and the kill-vs-paralysis choice are not \
                 modelled"
            ),
        });
    }
    if let Some(duration) = assassin_death_attack_duration_bonus_rounds(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.assassin.death_attack.duration_bonus_rounds".to_owned(),
            value: duration,
            detail: format!(
                "Assassin level {level} Death Attack paralysis duration: 1d6 + {duration} rounds \
                 (corpus `DeathAttackDuration = AssassinLVL`). Grounds the level-derived bonus \
                 rounds only; the 1d6 base is dice notation this engine does not model, the same \
                 dice-deferred treatment ACG Swashbuckler's own deed damage formulas already use"
            ),
        });
    }
}

/// PF1 Core Rulebook Loremaster Lore (`cr_abilities_class.lst:3020`,
/// `KEY:Loremaster ~ Lore`): `BONUS:SKILL|TYPE=Knowledge|LoreMasterLVL/2`
/// -- a flat bonus on all Knowledge skill checks, equal to half loremaster
/// level. Granted from class level 2 (`Loremaster_CFP_Level,2`), per the
/// record's own DESC ("At 2nd level..."), matching the class table's own
/// grant list. `None` below level 2.
pub(super) fn loremaster_lore_knowledge_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// PF1 Core Rulebook Loremaster Secret Lore (`cr_abilities_class.lst:3017`,
/// `KEY:Loremaster ~ Secret Lore`): `DEFINE:LoremasterSecretsLVL|0`
/// `DEFINE:LoremasterSecretCount|0` / `BONUS:VAR|LoremasterSecretCount|
/// (LoreMasterLVL+1)/2` -- the SIZE of the loremaster secrets pool (one
/// secret at 1st level and every two levels after). Grounds the pool SIZE
/// only, the same "quantity is a real fact, the choice made with it is a
/// different question" shape `eidolon_evolution_pool` already established
/// for Summoner: which of the ten Loremaster Secrets table entries is
/// chosen at each slot is not modelled here, and neither is
/// `LoremasterSecretsLVL` (`LoreMasterLVL+INT`, which secrets are
/// selectable) -- both are the SPENDING question, not the pool-size fact
/// this cycle grounds. Granted from class level 1 (`Loremaster_CFP_Level,1`).
/// `None` below level 1.
pub(super) fn loremaster_secret_lore_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 1) / 2)
}

/// Grounds Loremaster's two genuinely new-compute class features
/// (`decisions.md §22`'s 12-unit remainder). Unconditional on chassis
/// support, the same placement/reasoning as `ground_duelist_class_
/// features` above.
pub(super) fn ground_loremaster_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == LOREMASTER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = loremaster_lore_knowledge_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.loremaster.lore.knowledge_bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SKILL|TYPE=Knowledge|LoreMasterLVL/2`
                "Loremaster level {level} Lore: a +{bonus} bonus on all Knowledge skill checks, \
                 usable untrained. Grounds standalone: this engine computes no Knowledge-skill \
                 total, the same treatment Inquisitor's Monster Lore already uses"
            ),
        });
    }

    if let Some(pool) = loremaster_secret_lore_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.loremaster.secret_lore.pool_size".to_owned(),
            value: pool,
            detail: format!(
                "Loremaster level {level} Secret Lore: a pool of {pool} loremaster secrets \
                 (corpus `LoremasterSecretCount = (LoreMasterLVL+1)/2`). Grounds the pool SIZE \
                 only -- which secret is chosen from the Loremaster Secrets table at each slot, \
                 and the separate `LoremasterSecretsLVL` selectability gate, are the spending \
                 question, not modelled here, mirroring Summoner's own eidolon evolution pool"
            ),
        });
    }
}

/// Grounds Pathfinder Delver's three PaDFE (Pathfinder Delver Favored
/// Enemy) records -- `decisions.md §22`, Piece 2 item 3. Unconditional on
/// chassis support: Pathfinder Delver has no `ClassId` enum entry, so this
/// is called directly from `compute_pilot_base_chassis`, mirroring
/// `ground_duelist_class_features`'s own placement and reasoning above.
pub(super) fn ground_pathfinder_delver_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == PATHFINDER_DELVER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = pathfinder_delver_padfe_bonus(level) {
        for (feature_slug, creature_type) in [
            ("padfe_construct", "constructs"),
            ("padfe_ooze", "oozes"),
            ("padfe_undead", "undead creatures"),
        ] {
            explanations.push(ComputationExplanation {
                id: format!(
                    "class_feature.adventurers_guide.pathfinder_delver.{feature_slug}.bonus"
                ),
                value: bonus,
                detail: format!(
                    "Pathfinder Delver level {level} Guardbreaker: a +{bonus} bonus on Bluff, \
                     Knowledge, Perception, Sense Motive, and Survival checks made against \
                     {creature_type}, and the same bonus on weapon attack and damage rolls \
                     against them (corpus `Favored{{Construct,Ooze,Undead}} = TrapSenseBonus`, \
                     `TrapSenseBonus = RogueTrapSenseLVL/3`, `RogueTrapSenseLVL = PaDLVL+1` for \
                     a Pathfinder-Delver-only character). Grounds only the flat bonus \
                     magnitude; it modifies no actual skill, attack, or damage total, and this \
                     bonus never applies if the character already has Ranger's own real \
                     Favored Enemy of that type (this engine does not model that override \
                     precondition)"
                ),
            });
        }

        // SD-34 wave 46 (`decisions.md §22`'s WAVE 46 UPDATE): Guardbreaker's
        // OWN record (`ag_abilities_class.lst:382` -- `KEY:Pathfinder Delver
        // ~ Guardbreaker`, `BONUS:VAR|FavoredConstruct,FavoredOoze,
        // FavoredUndead|TrapSenseBonus`) is a distinct corpus unit from the
        // three PaDFE sub-records above (which only reference the SAME
        // `TrapSenseBonus` variable via their own `Favored<Type>` `ASPECT`).
        // Same magnitude, same level gate, its own explanation id.
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_delver.guardbreaker.bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|FavoredConstruct,FavoredOoze,FavoredUndead|TrapSenseBonus`,
                //   `ag_abilities_class.lst:382`
                "Pathfinder Delver level {level} Guardbreaker: sets `FavoredConstruct`, \
                 `FavoredOoze`, and `FavoredUndead` to `TrapSenseBonus` ({bonus}) -- the same \
                 magnitude the three PaDFE sub-records above ground, on the granting record itself"
            ),
        });
    }

    if let Some(bonus) = pathfinder_delver_master_explorer_skill_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_delver.master_explorer.skill_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Pathfinder Delver level {level} Master Explorer: a +{bonus} bonus on Disable \
                 Device and Perception checks (corpus `PaDSkillBonus = max(1,CL/2)`, granted \
                 level 1, `ag_classes.lst:285`). Grounds standalone: this engine computes no \
                 Disable-Device/Perception-skill total"
            ),
        });
    }

    if let Some(times) = pathfinder_delver_thrilling_escape_uses_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_delver.thrilling_escape.uses_per_day"
                .to_owned(),
            value: times,
            detail: format!(
                "Pathfinder Delver level {level} Thrilling Escape: usable {times}/day (corpus \
                 `PaDEscapeTimes`, a cumulative `+1` at levels 3, 7, and 9, `ag_classes.lst:\
                 287,291,292`). Grounds standalone: this engine tracks no per-day use budget \
                 for it"
            ),
        });
    }

    if let Some(initiative) = pathfinder_delver_vigilant_combatant_initiative_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_delver.vigilant_combatant.\
                 initiative_bonus"
                .to_owned(),
            value: initiative,
            detail: format!(
                "Pathfinder Delver level {level} Vigilant Combatant: a +{initiative} bonus on \
                 initiative checks (corpus `PaDInitiative = CL/2`, granted level 4, \
                 `ag_classes.lst:288`). No initiative total exists anywhere in this engine, so \
                 this grounds standalone -- the same shape as Duelist's own Improved Reaction"
            ),
        });
    }

    if let Some(times) = pathfinder_delver_fortunate_soul_uses_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_delver.fortunate_soul.uses_per_day"
                .to_owned(),
            value: times,
            detail: format!(
                "Pathfinder Delver level {level} Fortunate Soul: usable {times}/day (corpus \
                 `PaDFortunateTimes`, a cumulative `+1` at levels 6 and 10, `ag_classes.lst:\
                 290,293`). Grounds standalone: this engine tracks no per-day use budget for it"
            ),
        });
    }

    if let Some(caster_level) = pathfinder_delver_true_seeing_caster_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_delver.true_seeing.caster_level"
                .to_owned(),
            value: caster_level,
            detail: format!(
                "Pathfinder Delver level {level} True Seeing: 1/day spell-like ability \
                 (functions as true seeing), caster level {caster_level} (corpus `SPELLS:\
                 Pathfinder Delver|TIMES=1|CASTERLEVEL=PaDLvl|True Seeing,...`, `PaDLvl = CL`, \
                 granted level 9, `ag_classes.lst:292`). Grounds the caster level and the \
                 literal `1`/day use count only; no spell-effect total exists anywhere in this \
                 engine for it to layer onto"
            ),
        });
    }
}

/// Pathfinder Delver Master Explorer (`ag_abilities_class.lst:379`,
/// `KEY:Pathfinder Delver ~ Master Explorer`): `DEFINE:PaDSkillBonus|0`,
/// consumed by `BONUS:SKILL|Disable Device,Perception|PaDSkillBonus`. The
/// formula itself -- `BONUS:VAR|PaDSkillBonus|max(1,CL/2)` -- lives on the
/// class's own level-1 grant row (`ag_classes.lst:285`), the SAME
/// cross-file "class-table BONUS:VAR sets the variable the class_feature
/// record's own DEFINE only defaults to 0" idiom `pathfinder_delver_padfe_
/// bonus` above already establishes for this class (`TrapSenseBonus`).
/// Granted from class level 1. `None` below level 1.
pub(super) fn pathfinder_delver_master_explorer_skill_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let level = i16::from(level);
    Some(std::cmp::max(1, level / 2))
}

/// Pathfinder Delver Thrilling Escape (`ag_abilities_class.lst:381`,
/// `KEY:Pathfinder Delver ~ Thrilling Escape`): `DEFINE:PaDEscapeTimes|0`.
/// `ag_classes.lst` grants a cumulative `BONUS:VAR|PaDEscapeTimes|1` three
/// separate times -- level 3 (`line 287`, alongside Guardbreaker), level 7
/// (`line 291`), and level 9 (`line 292`, alongside True Seeing) -- so the
/// real running total is 1 (levels 3-6), 2 (levels 7-8), 3 (level 9+).
/// `None` below level 3 (never granted).
pub(super) fn pathfinder_delver_thrilling_escape_uses_per_day(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    let mut times = 1;
    if level >= 7 {
        times += 1;
    }
    if level >= 9 {
        times += 1;
    }
    Some(times)
}

/// Pathfinder Delver Vigilant Combatant (`ag_abilities_class.lst:384`,
/// `KEY:Pathfinder Delver ~ Vigilant Combatant`): `DEFINE:PaDInitiative|0`,
/// consumed by `BONUS:COMBAT|Initiative|PaDInitiative`. The formula --
/// `BONUS:VAR|PaDInitiative|CL/2` -- lives on the class's own level-4 grant
/// row (`ag_classes.lst:288`), the same cross-file idiom as Master Explorer
/// above. Granted from class level 4. `None` below level 4.
pub(super) fn pathfinder_delver_vigilant_combatant_initiative_bonus(level: u8) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// Pathfinder Delver Fortunate Soul (`ag_abilities_class.lst:386`,
/// `KEY:Pathfinder Delver ~ Fortunate Soul`): `DEFINE:PaDFortunateTimes|0`.
/// `ag_classes.lst` grants a cumulative `BONUS:VAR|PaDFortunateTimes|1`
/// twice -- level 6 (`line 290`) and level 10 (`line 293`, alongside Nick
/// of Time) -- so the real running total is 1 (levels 6-9), 2 (level 10).
/// `None` below level 6 (never granted).
pub(super) fn pathfinder_delver_fortunate_soul_uses_per_day(level: u8) -> Option<i16> {
    if level < 6 {
        return None;
    }
    let mut times = 1;
    if level >= 10 {
        times += 1;
    }
    Some(times)
}

/// Pathfinder Delver True Seeing (`ag_abilities_class.lst:387`,
/// `KEY:Pathfinder Delver ~ True Seeing`): `SPELLS:Pathfinder Delver|\
/// TIMES=1|CASTERLEVEL=PaDLvl|True Seeing,16+max(INT,WIS,CHA)`, granted at
/// class level 9 (`ag_classes.lst:292`). `PaDLvl = CL` (the class's own
/// raw level, `ag_classes.lst:279`, `BONUS:VAR|PaDLVL|CL`) -- the same
/// "raw class level" caster-level idiom Shadowdancer's own Shadow
/// Illusion/Shadow Call already established. `None` below level 9.
pub(super) fn pathfinder_delver_true_seeing_caster_level(level: u8) -> Option<i16> {
    if level < 9 {
        return None;
    }
    Some(i16::from(level))
}

/// Grounds Phrenic Slayer's Favored Enemy record (the base fact plus all 31
/// creature-type sub-records) -- `decisions.md §22`'s WAVE 45 UPDATE, the
/// first closed slice of sub-mechanism-5's "registered prestige class,
/// magnitude-only" remainder. Phrenic Slayer has no `ClassId`-family enum
/// entry (source book `ultimate_psionics`, so `modelled_class_books()`'s
/// CRB-only prestige loop never registers it either) -- unconditional on
/// chassis support, called directly from `compute_pilot_base_chassis`,
/// mirroring `ground_pathfinder_delver_class_features`'s own placement and
/// reasoning above. This class's four remaining magnitude-bearing features
/// (Brain Nausea, Lucid Buffer, Power Resistance, Rebound Attack) all key off
/// `PhrenicSlayerPrimeStat` (which of several possible parent classes'
/// manifesting ability the character entered through), a genuinely separate
/// modelling question left out of this cycle's scope -- see the WAVE 45
/// UPDATE entry for the named remainder.
pub(super) fn ground_phrenic_slayer_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == PHRENIC_SLAYER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    let Some(bonus) = phrenic_slayer_favored_enemy_bonus(level) else {
        return;
    };

    explanations.push(ComputationExplanation {
        id: "class_feature.ultimate_psionics.phrenic_slayer.favored_enemy.bonus".to_owned(),
        value: bonus,
        detail: format!(
            "Phrenic Slayer level {level} Favored Enemy: a +{bonus} bonus on attack, damage, and \
             skill checks against the chosen favored-enemy creature type (corpus \
             `SlayerFavoredEnemy = 2*floor((2+PhrenicSlayerLVL)/3)`). Grounds the shared \
             magnitude fact only; the choice of WHICH creature type is not modelled"
        ),
    });

    for (slug, creature_type) in PHRENIC_SLAYER_FAVORED_ENEMY_MEMBERS {
        explanations.push(ComputationExplanation {
            id: format!(
                "class_feature.ultimate_psionics.phrenic_slayer.favored_enemy_{slug}.bonus"
            ),
            value: bonus,
            detail: format!(
                "Phrenic Slayer level {level} Favored Enemy ({creature_type}): a +{bonus} bonus \
                 on attack, damage, and skill checks against {creature_type} (corpus `%1` = \
                 `SlayerFavoredEnemy`, the identical shared magnitude the base Favored Enemy \
                 record above already grounds -- this sub-record's own `ASPECT` references the \
                 SAME variable, and defines no `DEFINE`/`BONUS` token of its own)"
            ),
        });
    }
}

/// Argent Dramaturge Argent Performance (`ag_abilities_class.lst:24`,
/// `KEY:Argent Dramaturge ~ Argent Performance`): `DEFINE:
/// ArgentPerformanceRounds|0` / `BONUS:VAR|ArgentPerformanceRounds|
/// ArgentDramaturgeLVL*2` -- rounds of bardic-performance-style use per day,
/// the same "level*2" idiom Bard's own bardic performance rounds already use.
/// Granted from class level 1 (no `PREVARGTEQ` gate on this token). `None`
/// below level 1.
pub(super) fn argent_dramaturge_argent_performance_rounds(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) * 2)
}

/// Argent Dramaturge Argent Performance's save DC (same record):
/// `DEFINE:ArgentPerformanceDC|0` / `BONUS:VAR|ArgentPerformanceDC|
/// 10+ArgentDramaturgeLVL+CHA` -- the classic "10 + level factor + ability
/// modifier" save-DC idiom (`warpriest_channel_energy_dc`'s own shape),
/// `CHA` here being the Charisma MODIFIER (this codebase's established
/// convention for a bare ability abbreviation inside a `BONUS:VAR` DC
/// formula, matching `warpriest_channel_energy_dc`'s own Wisdom-modifier
/// parameter). Granted from class level 1. `None` below level 1.
pub(super) fn argent_dramaturge_argent_performance_dc(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 + i16::from(level) + charisma_modifier)
}

/// Argent Dramaturge Dramaturgical Flourish (`ag_abilities_class.lst:25`,
/// `KEY:Argent Dramaturge ~ Dramaturgical Flourish`): `BONUS:ABILITYPOOL|
/// Dramaturgical Flourish Choice|ArgentDramaturgeLVL/2` -- the SIZE of the
/// dramaturgical-flourish choice pool (one flourish at 2nd level and every
/// two levels after), the same "grounds the pool SIZE only" shape
/// `loremaster_secret_lore_pool_size` already established; which flourish
/// is chosen from the list is not modelled. `None` below level 1 (the
/// formula itself yields 0 below level 2, no separate gate needed).
pub(super) fn argent_dramaturge_dramaturgical_flourish_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// Grounds Argent Dramaturge's two magnitude-bearing class features --
/// `decisions.md §22`'s WAVE 46 UPDATE, sub-mechanism-5's "registered
/// prestige class, magnitude-only" remainder. Unconditional on chassis
/// support (no `ClassId`-family enum entry for this class), same placement
/// as `ground_phrenic_slayer_class_features` above.
pub(super) fn ground_argent_dramaturge_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ARGENT_DRAMATURGE_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(rounds) = argent_dramaturge_argent_performance_rounds(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.argent_dramaturge.argent_performance.rounds"
                .to_owned(),
            value: rounds,
            detail: format!(
                "Argent Dramaturge level {level} Argent Performance: usable {rounds} rounds \
                 per day (corpus `ArgentPerformanceRounds = ArgentDramaturgeLVL*2`, shared with \
                 bardic performance rounds). Grounds standalone: this engine tracks no shared \
                 bardic-performance-rounds pool for it to add into"
            ),
        });
    }

    if let Some(dc) =
        argent_dramaturge_argent_performance_dc(level, ability_modifiers.charisma)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.argent_dramaturge.argent_performance.dc"
                .to_owned(),
            value: dc,
            detail: format!(
                "Argent Dramaturge level {level} Argent Performance save DC {dc} (corpus \
                 `ArgentPerformanceDC = 10+ArgentDramaturgeLVL+CHA`, this character's Charisma \
                 modifier {cha_mod:+}). Grounds the DC magnitude only; no save is actually \
                 rolled by this engine",
                cha_mod = ability_modifiers.charisma
            ),
        });
    }

    if let Some(pool) = argent_dramaturge_dramaturgical_flourish_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.argent_dramaturge.dramaturgical_flourish.\
                 pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                "Argent Dramaturge level {level} Dramaturgical Flourish: a pool of {pool} \
                 dramaturgical flourishes (corpus `ArgentDramaturgeLVL/2`). Grounds the pool \
                 SIZE only -- which flourish is chosen is not modelled"
            ),
        });
    }
}

/// Horizon Walker Favored Terrain (`apg_abilities_class.lst:1295`,
/// `KEY:Horizon Walker ~ Favored Terrain`): `DEFINE:
/// HorizonWalkerFavoredTerrainLVL|0` / `BONUS:VAR|
/// HorizonWalkerFavoredTerrainLVL|HorizonWalkerLVL` / `BONUS:VAR|
/// FavoredTerrainPool|(2*(HorizonWalkerFavoredTerrainLVL+1))/3` -- the SIZE
/// of the favored-terrain pool granted alongside the ranger-style favored
/// terrain chooser (`VISIBLE:NO`, an internal bookkeeping record; no
/// `PREVARGTEQ` gate, active from level 1). Grounds the pool SIZE only,
/// the same shape `loremaster_secret_lore_pool_size` already established.
/// `None` below level 1.
pub(super) fn horizon_walker_favored_terrain_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let hw_level = i16::from(level);
    Some((2 * (hw_level + 1)) / 3)
}

/// Horizon Walker Terrain Mastery (`apg_abilities_class.lst:1313`,
/// `KEY:Horizon Walker ~ Terrain Mastery`): `BONUS:ABILITYPOOL|Terrain
/// Mastery Selection|HorizonWalkerLVL/2` -- the SIZE of the terrain-mastery
/// choice pool (one terrain mastered at 2nd level and every two levels
/// after). No `PREVARGTEQ` gate; the formula itself yields 0 below level 2.
/// `None` below level 1.
pub(super) fn horizon_walker_terrain_mastery_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// Horizon Walker Terrain Dominance (`apg_abilities_class.lst:1337`,
/// `KEY:Horizon Walker ~ Terrain Dominance`): `BONUS:ABILITYPOOL|Terrain
/// Dominance Selection|HorizonWalkerLVL/3` -- the SIZE of the
/// terrain-dominance choice pool (one dominance at 3rd level and every
/// three levels after). No `PREVARGTEQ` gate; the formula itself yields 0
/// below level 3. `None` below level 1.
pub(super) fn horizon_walker_terrain_dominance_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 3)
}

/// Grounds Horizon Walker's three pool-size class features --
/// `decisions.md §22`'s WAVE 46 UPDATE. Unconditional on chassis support
/// (no `ClassId`-family enum entry for this class), same placement as
/// `ground_phrenic_slayer_class_features` above.
pub(super) fn ground_horizon_walker_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == HORIZON_WALKER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(pool) = horizon_walker_favored_terrain_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.horizon_walker.favored_terrain.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                "Horizon Walker level {level} Favored Terrain: a pool of {pool} (corpus \
                 `FavoredTerrainPool = (2*(HorizonWalkerFavoredTerrainLVL+1))/3`, \
                 `HorizonWalkerFavoredTerrainLVL = HorizonWalkerLVL`). Grounds the pool SIZE \
                 only -- which terrain is chosen is not modelled"
            ),
        });
    }

    if let Some(pool) = horizon_walker_terrain_mastery_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.horizon_walker.terrain_mastery.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                "Horizon Walker level {level} Terrain Mastery: a pool of {pool} (corpus \
                 `HorizonWalkerLVL/2`). Grounds the pool SIZE only -- which terrain is mastered \
                 is not modelled"
            ),
        });
    }

    if let Some(pool) = horizon_walker_terrain_dominance_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.horizon_walker.terrain_dominance.\
                 pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                "Horizon Walker level {level} Terrain Dominance: a pool of {pool} (corpus \
                 `HorizonWalkerLVL/3`). Grounds the pool SIZE only -- which terrain gains \
                 dominance is not modelled"
            ),
        });
    }
}

/// Nature Warden Companion Bond (`apg_abilities_class.lst:1421`,
/// `KEY:Nature Warden ~ Companion Bond`): `DEFINE:CompanionBondLVL|0`,
/// with the formula `BONUS:VAR|CompanionBondLVL|NatureWardenLVL` living on
/// the class's own level-1 grant row (`apg_classes.lst:460`), the same
/// cross-file "class-table BONUS:VAR sets the variable" idiom
/// `pathfinder_delver_padfe_bonus` established for Pathfinder Delver.
/// Grounds the raw level-tracking magnitude (nature warden levels stack
/// with animal-companion-granting class levels for the companion's own
/// progression, a companion-progression fact this engine does not model
/// beyond the magnitude itself). Granted from class level 1. `None` below
/// level 1.
pub(super) fn nature_warden_companion_bond_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// Nature Warden Survivalist (`apg_abilities_class.lst:1429`,
/// `KEY:Nature Warden ~ Survivalist`): `BONUS:VAR|SurvivalistLVL|
/// NatureWardenLVL`, in the record's own tokens (no external class-table
/// lookup needed, unlike Companion Bond above). A raw level-tracking
/// magnitude; the qualitative "no penalty for improvised weapons"/
/// "masterwork treatment" effects it gates are not modelled beyond the
/// magnitude itself. Granted from class level 1. `None` below level 1.
pub(super) fn nature_warden_survivalist_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// Grounds Nature Warden's two magnitude-bearing class features --
/// `decisions.md §22`'s WAVE 46 UPDATE. Unconditional on chassis support
/// (no `ClassId`-family enum entry for this class), same placement as
/// `ground_phrenic_slayer_class_features` above. Woodforging (this class's
/// third open sm5 unit) carries no `DEFINE`/`BONUS` token anywhere in the
/// corpus (`wiring_class: "display"`, `display:no_magnitude_token`) --
/// left named, not attempted, this cycle.
pub(super) fn ground_nature_warden_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == NATURE_WARDEN_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bond_level) = nature_warden_companion_bond_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.nature_warden.companion_bond.level"
                .to_owned(),
            value: bond_level,
            detail: format!(
                "Nature Warden level {level} Companion Bond: `CompanionBondLVL` = {bond_level} \
                 (corpus `CompanionBondLVL = NatureWardenLVL`). Grounds the raw level-tracking \
                 magnitude only; this engine does not model the animal-companion \
                 level-stacking effect it feeds"
            ),
        });
    }

    if let Some(survivalist_level) = nature_warden_survivalist_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.nature_warden.survivalist.level".to_owned(),
            value: survivalist_level,
            detail: format!(
                "Nature Warden level {level} Survivalist: `SurvivalistLVL` = \
                 {survivalist_level} (corpus `SurvivalistLVL = NatureWardenLVL`). Grounds the \
                 raw level-tracking magnitude only; the qualitative improvised-weapon/\
                 masterwork effects it gates are not modelled"
            ),
        });
    }
}

/// Rage Prophet Rage Prophet Mystery (`apg_abilities_class.lst:1440`,
/// `KEY:Rage Prophet ~ Rage Prophet Mystery`): `BONUS:VAR|
/// RageProphetMysteryLVL|RageProphetLVL`, in the record's own tokens. A raw
/// level-tracking magnitude; which extra spirit-guide spell is learned at
/// each even level is not modelled. Granted from class level 1. `None`
/// below level 1.
pub(super) fn rage_prophet_mystery_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// Rage Prophet Ragecaster (`apg_abilities_class.lst:1443`,
/// `KEY:Rage Prophet ~ Ragecaster`): `BONUS:VAR|RagecasterLVL|
/// RageProphetLVL`, in the record's own tokens. A raw level-tracking
/// magnitude; the moment-of-clarity caster-level boost and Constitution-
/// to-DC effects it gates are not modelled. Granted from class level 1.
/// `None` below level 1.
pub(super) fn rage_prophet_ragecaster_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// Grounds Rage Prophet's two magnitude-bearing class features --
/// `decisions.md §22`'s WAVE 46 UPDATE. Unconditional on chassis support
/// (no `ClassId`-family enum entry for this class), same placement as
/// `ground_phrenic_slayer_class_features` above. Spirit Warrior (this
/// class's third open sm5 unit) carries no `DEFINE`/`BONUS` token anywhere
/// in the corpus (`wiring_class: "display"`) -- left named, not attempted,
/// this cycle.
pub(super) fn ground_rage_prophet_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == RAGE_PROPHET_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(mystery_level) = rage_prophet_mystery_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.rage_prophet.rage_prophet_mystery.level"
                .to_owned(),
            value: mystery_level,
            detail: format!(
                "Rage Prophet level {level} Rage Prophet Mystery: `RageProphetMysteryLVL` = \
                 {mystery_level} (corpus `RageProphetMysteryLVL = RageProphetLVL`). Grounds the \
                 raw level-tracking magnitude only; which spirit-guide spell is learned is not \
                 modelled"
            ),
        });
    }

    if let Some(ragecaster_level) = rage_prophet_ragecaster_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.rage_prophet.ragecaster.level".to_owned(),
            value: ragecaster_level,
            detail: format!(
                "Rage Prophet level {level} Ragecaster: `RagecasterLVL` = {ragecaster_level} \
                 (corpus `RagecasterLVL = RageProphetLVL`). Grounds the raw level-tracking \
                 magnitude only; the moment-of-clarity caster-level boost and Constitution-to-DC \
                 effects it gates are not modelled"
            ),
        });
    }
}

/// Holy Vindicator Stigmata (`apg_abilities_class.lst:1278`,
/// `KEY:Holy Vindicator ~ Stigmata`): `DEFINE:StigmataLVL|0` /
/// `BONUS:VAR|StigmataLVL|floor(HolyVindicatorLVL/2)` -- a sacred/profane
/// bonus (on the vindicator's own choice of attack, damage, AC, caster
/// level checks, or saves) equal to half class level, the same "half
/// level, floored" idiom `duelist_elaborate_defense_dodge_bonus` already
/// established. Granted from class level 1 (no `PREVARGTEQ` gate). `None`
/// below level 1.
pub(super) fn holy_vindicator_stigmata_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// Grounds Holy Vindicator's one magnitude-bearing class feature --
/// `decisions.md §22`'s WAVE 46 UPDATE. Unconditional on chassis support
/// (no `ClassId`-family enum entry for this class), same placement as
/// `ground_phrenic_slayer_class_features` above. Channel Smite (this
/// class's other open sm5 unit) is a bonus-feat grant with no magnitude
/// token at all (`wiring_class: "display"`) -- left named, not attempted,
/// this cycle.
pub(super) fn ground_holy_vindicator_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == HOLY_VINDICATOR_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = holy_vindicator_stigmata_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.holy_vindicator.stigmata.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Holy Vindicator level {level} Stigmata: a +{bonus} sacred or profane bonus \
                 (corpus `StigmataLVL = floor(HolyVindicatorLVL/2)`). Grounds the magnitude \
                 only: this engine computes no attack/damage/AC/caster-level/save total for it \
                 to layer onto, and the player's own choice of which total it applies to is not \
                 modelled"
            ),
        });
    }
}

/// Stalwart Defender AC Bonus (`apg_abilities_class.lst:1451`,
/// `KEY:Stalwart Defender ~ AC Bonus`): `DEFINE:StalwartDefenderDodgeACBonus|0`
/// / `BONUS:VAR|StalwartDefenderDodgeACBonus|
/// 1+(StalwartDefenderLVL>=4)+(StalwartDefenderLVL>=7)+(StalwartDefenderLVL>=10)`
/// -- a step table transcribed literally (each `(SDL>=N)` term is 0 or 1):
/// +1 at level 1, +2 at level 4, +3 at level 7, +4 at level 10. Granted
/// from class level 1. `None` below level 1.
pub(super) fn stalwart_defender_ac_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let mut bonus = 1;
    if level >= 4 {
        bonus += 1;
    }
    if level >= 7 {
        bonus += 1;
    }
    if level >= 10 {
        bonus += 1;
    }
    Some(bonus)
}

/// Stalwart Defender Damage Reduction (`apg_abilities_class.lst:1470`,
/// `KEY:Stalwart Defender ~ Damage Reduction`): `DEFINE:DamageReductionLVL|0`
/// / `BONUS:VAR|DamageReductionLVL|
/// (StalwartDefenderLVL>4)+(StalwartDefenderLVL>6)+(StalwartDefenderLVL>6)+
/// (StalwartDefenderLVL>9)+(StalwartDefenderLVL>9)` -- transcribed literally
/// (the `>6` and `>9` terms each appear twice in the corpus token, i.e. DR
/// increases by 2 at levels 7 and 10, not 1): DR 0 below level 5, DR 1
/// (levels 5-6), DR 3 (levels 7-9), DR 5 (level 10+). `None` below level 1
/// (DR 0 is not worth grounding as a fact).
pub(super) fn stalwart_defender_damage_reduction(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let mut dr = 0;
    if level > 4 {
        dr += 1;
    }
    if level > 6 {
        dr += 2;
    }
    if level > 9 {
        dr += 2;
    }
    Some(dr)
}

/// Stalwart Defender Defensive Powers (`apg_abilities_class.lst:1453`,
/// `KEY:Stalwart Defender ~ Defensive Powers`): `DEFINE:DefensivePowerLVL|0`
/// / `BONUS:ABILITYPOOL|Defensive Stance Power|DefensivePowerLVL` /
/// `BONUS:VAR|DefensivePowerLVL|StalwartDefenderLVL/2` -- the SIZE of the
/// defensive-power choice pool (one power at 2nd level and every two
/// levels after), the same "grounds the pool SIZE only" shape
/// `loremaster_secret_lore_pool_size` already established. `None` below
/// level 1 (the formula itself yields 0 below level 2).
pub(super) fn stalwart_defender_defensive_powers_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// Stalwart Defender Defensive Stance (`apg_abilities_class.lst:1452`,
/// `KEY:Stalwart Defender ~ Defensive Stance`): `DEFINE:
/// DefensiveStanceDuration|0` / `BONUS:VAR|DefensiveStanceDuration|4+CON` /
/// `BONUS:VAR|DefensiveStanceDuration|(StalwartDefenderLVL-1)*2` -- rounds
/// per day the stance can be maintained: `4 + Constitution modifier` at
/// level 1, plus 2 more rounds per level thereafter. `CON` here is the
/// Constitution MODIFIER (this codebase's established convention, matching
/// `warpriest_channel_energy_dc`'s own ability-modifier parameter). Granted
/// from class level 1. `None` below level 1.
pub(super) fn stalwart_defender_defensive_stance_duration_rounds(
    level: u8,
    constitution_modifier: i16,
) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(4 + constitution_modifier + (i16::from(level) - 1) * 2)
}

/// Grounds Stalwart Defender's four magnitude-bearing class features --
/// `decisions.md §22`'s WAVE 46 UPDATE. Unconditional on chassis support
/// (no `ClassId`-family enum entry for this class), same placement as
/// `ground_phrenic_slayer_class_features` above. This class's two other
/// open sm5 units are NOT attempted this cycle: Increased Damage
/// Reduction is a `Defensive Stance Power` pool MEMBER (`BONUS:VAR|
/// DamageReductionLVL|1`, selectable up to twice) whose own magnitude
/// depends on a real recorded pool selection this engine does not track
/// for this class's pool; Renewed Defense heals `%1d8 + %2` (`CL/2`,
/// `CON`) -- dice notation this engine's `formula_interpreter.rs` does not
/// parse, the same "grounds the level-derived factor only, never the
/// die roll" boundary as Assassin's Death Attack, left unattempted here
/// rather than guessed at.
pub(super) fn ground_stalwart_defender_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == STALWART_DEFENDER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = stalwart_defender_ac_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.stalwart_defender.ac_bonus.dodge_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Stalwart Defender level {level} AC Bonus: a +{bonus} dodge bonus to Armor \
                 Class (corpus `StalwartDefenderDodgeACBonus = \
                 1+(SDL>=4)+(SDL>=7)+(SDL>=10)`). Grounds the magnitude only: no armor-class \
                 total exists anywhere in this engine for it to layer onto"
            ),
        });
    }

    if let Some(dr) = stalwart_defender_damage_reduction(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.stalwart_defender.damage_reduction.value"
                .to_owned(),
            value: dr,
            detail: format!(
                "Stalwart Defender level {level} Damage Reduction {dr}/- (corpus \
                 `DamageReductionLVL = (SDL>4)+(SDL>6)+(SDL>6)+(SDL>9)+(SDL>9)`, transcribed \
                 literally). Grounds the magnitude only: no damage-reduction total exists \
                 anywhere in this engine for it to layer onto"
            ),
        });
    }

    if let Some(pool) = stalwart_defender_defensive_powers_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.stalwart_defender.defensive_powers.\
                 pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                "Stalwart Defender level {level} Defensive Powers: a pool of {pool} (corpus \
                 `DefensivePowerLVL = StalwartDefenderLVL/2`). Grounds the pool SIZE only -- \
                 which defensive power is chosen is not modelled"
            ),
        });
    }

    if let Some(rounds) = stalwart_defender_defensive_stance_duration_rounds(
        level,
        ability_modifiers.constitution,
    ) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.stalwart_defender.defensive_stance.\
                 duration_rounds"
                .to_owned(),
            value: rounds,
            detail: format!(
                "Stalwart Defender level {level} Defensive Stance: usable {rounds} rounds per \
                 day (corpus `DefensiveStanceDuration = 4+CON+(SDL-1)*2`, this character's \
                 Constitution modifier {con_mod:+}). Grounds the per-day round budget only; \
                 this engine tracks no active-stance state or its combat bonuses",
                con_mod = ability_modifiers.constitution
            ),
        });
    }
}

/// Total character level ("`TL`" in PCGen formula syntax) -- the sum of
/// every class level this character has taken, not just the prestige
/// class currently being swept. The same "total character level" global
/// `explain_undine_formula_race_trait` already established as a supported
/// `formula_interpreter` variable; extracted here as its own pure helper
/// since Divine Scion's own Domain Specialization sub-records key their
/// caster level directly on `TL`, not on `DivineScionLVL`.
pub(super) fn total_character_level(input: &CharacterInput) -> i16 {
    let total: i64 = input.chosen.class_levels.iter().map(|c| i64::from(c.level)).sum();
    i16::try_from(total).unwrap_or(i16::MAX)
}

/// Twilight Talon Sneak Attack (`ag_abilities_class.lst:541`, `KEY:Twilight
/// Talon ~ Sneak Attack`): `BONUS:VAR|SneakAttackDice|(TwilightTalonLVL+2)/3`
/// -- the classic sneak-attack-dice-by-level idiom this file already grounds
/// repeatedly for other classes (`slayer_sneak_attack_dice`,
/// Duelist/Assassin's own precedents). No `PREVARGTEQ` gate on the token
/// itself. `None` below level 1.
pub(super) fn twilight_talon_sneak_attack_dice(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 2) / 3)
}

/// Twilight Talon Enhanced Tattoo's own save DC (`ag_abilities_class.
/// lst:542`, `KEY:Twilight Talon ~ Enhanced Tattoo`): `DEFINE:
/// EnhancedTattooDC|0` / `BONUS:VAR|EnhancedTattooDC|
/// 10+TwilightTalonLVL/2+CHA` -- the classic "10 + level factor + ability
/// modifier" save-DC idiom (`warpriest_channel_energy_dc`'s own shape,
/// `argent_dramaturge_argent_performance_dc`'s own precedent within this
/// same book), `CHA` being the Charisma MODIFIER (this codebase's
/// established convention). No `PREVARGTEQ` gate on the token itself.
/// `None` below level 1.
pub(super) fn twilight_talon_enhanced_tattoo_save_dc(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 + i16::from(level) / 2 + charisma_modifier)
}

/// Twilight Talon's own 5 Enhanced Tattoo tiers, each a real one-of-two
/// `ABILITYPOOL` choice -- verified directly against the real, non-ingested
/// PCGen oracle (`ag_abilities_class.lst:548-557`): the tier's own minimum
/// class level, its own choice-set id, and its two candidate (slug,
/// display name) members. Every tattoo record's own `SPELLS:Enhanced
/// Tattoo|CASTERLEVEL=TL|<spell name>[,EnhancedTattooDC]` token grounds
/// only the caster-level fact (`TL` = total character level) -- the same
/// "ground the SLA triple, don't model the effect or its own save DC total"
/// split `ground_divine_scion_class_features`'s own domain block already
/// established; no `TIMES=` parameter appears on any of these 10 tokens
/// (PCGen's own implicit 1/day default), so no separate uses-per-day fact
/// is grounded here -- there is no literal token to cite for it.
///
/// Type alias per clippy's own `type_complexity` lint (this bundle's
/// zero-warning ceiling): (tier min level, choice-set id, tier members).
pub(super) type TwilightTalonTattooTier = (u8, &'static str, &'static [(&'static str, &'static str)]);

pub(super) const TWILIGHT_TALON_TATTOO_TIERS: &[TwilightTalonTattooTier] = &[
    (
        2,
        TWILIGHT_TALON_TATTOO_LEVEL_2_CHOICE_ID,
        &[
            ("disguise_self", "Disguise Self"),
            ("undetectable_alignment", "Undetectable Alignment"),
        ],
    ),
    (
        4,
        TWILIGHT_TALON_TATTOO_LEVEL_4_CHOICE_ID,
        &[("alter_self", "Alter Self"), ("invisibility", "Invisibility")],
    ),
    (
        6,
        TWILIGHT_TALON_TATTOO_LEVEL_6_CHOICE_ID,
        &[("glibness", "Glibness"), ("secret_page", "Secret Page")],
    ),
    (
        8,
        TWILIGHT_TALON_TATTOO_LEVEL_8_CHOICE_ID,
        &[("modify_memory", "Modify Memory"), ("zone_of_silence", "Zone of Silence")],
    ),
    (
        10,
        TWILIGHT_TALON_TATTOO_LEVEL_10_CHOICE_ID,
        &[("mislead", "Mislead"), ("seeming", "Seeming")],
    ),
];

/// Grounds Twilight Talon's 12 magnitude-bearing class features --
/// `decisions.md §22`'s WAVE 48 UPDATE, sub-mechanism-5's "registered
/// prestige class, magnitude-only" remainder. Unconditional on chassis
/// support (no `ClassId`-family enum entry for this class, source book
/// `adventurers_guide`), same placement as `ground_divine_scion_class_
/// features` above. The remaining 5 sm5 units this class carries (Many
/// Hats, Eye for Detail, Dead Drop, Resourceful Agent, Unassuming
/// Presence) carry no `BONUS`/`DEFINE` token at all (pure prose class
/// features) and are NOT attempted -- the same "no magnitude token to
/// ground" boundary this whole sub-mechanism-5 series already applies
/// elsewhere (e.g. Stalwart Defender's own Renewed Defense, wave 46).
pub(super) fn ground_twilight_talon_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == TWILIGHT_TALON_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(dice) = twilight_talon_sneak_attack_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.twilight_talon.sneak_attack.dice".to_owned(),
            value: dice,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|SneakAttackDice|(TwilightTalonLVL+2)/3`
                "Twilight Talon level {level} Sneak Attack: +{dice}d6. Grounds standalone: this \
                 engine tracks no shared sneak-attack-dice total for it to add into"
            ),
        });
    }

    if let Some(dc) =
        twilight_talon_enhanced_tattoo_save_dc(level, ability_modifiers.charisma)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.twilight_talon.enhanced_tattoo.save_dc"
                .to_owned(),
            value: dc,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|EnhancedTattooDC|10+TwilightTalonLVL/2+CHA`,
                "Twilight Talon level {level} Enhanced Tattoo save DC {dc} (corpus ` this \
                 character's Charisma modifier {cha_mod:+}). Grounds the DC magnitude only; no save \
                 is actually rolled by this engine, and no per-spell-like-ability total exists \
                 anywhere for it to feed into",
                cha_mod = ability_modifiers.charisma
            ),
        });
    }

    // Each of the 5 tiers is a real one-of-two `ABILITYPOOL` choice (see
    // `TWILIGHT_TALON_TATTOO_TIERS`'s own doc comment) -- only the ONE
    // tattoo the character actually recorded at each tier reached
    // surfaces, never both candidates simultaneously, the same
    // choice-gating discipline `ground_divine_scion_class_features`'s own
    // correction established.
    let caster_level = total_character_level(input);
    for &(tier_level, choice_id, members) in TWILIGHT_TALON_TATTOO_TIERS {
        if level < tier_level {
            continue;
        }
        let Some(selection) = choice_selection(input, choice_id) else {
            continue;
        };
        let Some(&(slug, display)) =
            members.iter().find(|(slug, _)| selection == format!("tattoo:{slug}"))
        else {
            continue;
        };
        explanations.push(ComputationExplanation {
            id: format!("class_feature.adventurers_guide.twilight_talon.{slug}.caster_level"),
            value: caster_level,
            detail: format!(
                "Twilight Talon level {level} Enhanced Tattoo (tier {tier_level}), {display}: \
                 spell-like ability, caster level {caster_level} (corpus `SPELLS:Enhanced \
                 Tattoo|CASTERLEVEL=TL|{display},...`, `TL` = this character's total level \
                 across every class, {caster_level}; recorded selection {choice_id} -> \
                 {selection}). Grounds the caster-level fact only -- no spell effect or save \
                 DC total is modelled beyond the standalone Enhanced Tattoo DC above"
            ),
        });
    }
}

/// Golden Legionnaire Allied Retribution (`ag_abilities_class.lst:136`
/// area, `KEY:Golden Legionnaire ~ Allied Retribution`): `DEFINE:
/// AlliedRetributionBonus|0` / `BONUS:VAR|AlliedRetributionBonus|
/// 1+(GoldenLegionnaireLVL>=7)` -- a flat +1, stepping to +2 at level 7.
/// No `PREVARGTEQ` gate on the token itself. `None` below level 1.
pub(super) fn golden_legionnaire_allied_retribution_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level >= 7))
}

/// Golden Legionnaire Authoritative Command (same record family):
/// `DEFINE:AuthoritativeCommandBonus|0` / `BONUS:VAR|
/// AuthoritativeCommandBonus|1+(GoldenLegionnaireLVL>=6)` -- a flat +1,
/// stepping to +2 at level 6. `None` below level 1.
pub(super) fn golden_legionnaire_authoritative_command_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level >= 6))
}

/// Golden Legionnaire Improved Aid (same record family): `DEFINE:
/// LegionImprovedAid|0` / `BONUS:VAR|LegionImprovedAid|
/// 1+(GoldenLegionnaireLVL>=9)` -- a flat +1, stepping to +2 at level 9.
/// `None` below level 1.
pub(super) fn golden_legionnaire_improved_aid_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level >= 9))
}

/// Golden Legionnaire United Defense (same record family): `DEFINE:
/// UnitedDefenseBonus|0` / `BONUS:VAR|UnitedDefenseBonus|
/// 1+(GoldenLegionnaireLVL>=6)+(GoldenLegionnaireLVL>=10)` -- a flat +1,
/// stepping to +2 at level 6 and +3 at level 10. `None` below level 1.
pub(super) fn golden_legionnaire_united_defense_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level >= 6) + i16::from(level >= 10))
}

/// Grounds Golden Legionnaire's 4 magnitude-bearing class features --
/// `decisions.md §22`'s WAVE 48 UPDATE, sub-mechanism-5's "registered
/// prestige class, magnitude-only" remainder. Unconditional on chassis
/// support (no `ClassId`-family enum entry for this class, source book
/// `adventurers_guide`), same placement as `ground_twilight_talon_class_
/// features` above. The remaining 12 sm5 units this class carries
/// (Bodyguard, Combat Feat, Defy Danger, Guardian of Liberty, Hold the
/// Line, In Harm's Way, Intercept, Legion Feats, Preemptive Strike,
/// Retaliate, Stand Still, Swift Aid) are either pure prose (no `BONUS`/
/// `DEFINE` token), automatic single-feat grants with no magnitude, or (for
/// Legion Feats/Combat Feat) a bonus-feat `ABILITYPOOL` this cycle chose
/// not to model a bare pool-of-feats magnitude for -- left named for a
/// future wave rather than guessed at.
pub(super) fn ground_golden_legionnaire_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == GOLDEN_LEGIONNAIRE_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = golden_legionnaire_allied_retribution_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.golden_legionnaire.allied_retribution.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|AlliedRetributionBonus|1+(GoldenLegionnaireLVL>=7)`
                "Golden Legionnaire level {level} Allied Retribution: +{bonus}. Grounds the \
                 magnitude only: no shared allied-retribution total exists anywhere in this engine \
                 for it to layer onto"
            ),
        });
    }

    if let Some(bonus) = golden_legionnaire_authoritative_command_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.golden_legionnaire.authoritative_command.\
                 bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|AuthoritativeCommandBonus|1+(GoldenLegionnaireLVL>=6)`
                "Golden Legionnaire level {level} Authoritative Command: +{bonus}. Grounds the \
                 magnitude only: no shared authoritative-command total exists anywhere in this \
                 engine for it to layer onto"
            ),
        });
    }

    if let Some(bonus) = golden_legionnaire_improved_aid_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.golden_legionnaire.improved_aid.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|LegionImprovedAid|1+(GoldenLegionnaireLVL>=9)`
                "Golden Legionnaire level {level} Improved Aid: +{bonus}. Grounds the magnitude \
                 only: no shared aid-another total exists anywhere in this engine for it to layer \
                 onto"
            ),
        });
    }

    if let Some(bonus) = golden_legionnaire_united_defense_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.golden_legionnaire.united_defense.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|UnitedDefenseBonus|1+(GoldenLegionnaireLVL>=6)+(GoldenLegionnaireLVL>=10)`
                "Golden Legionnaire level {level} United Defense: +{bonus}. Grounds the magnitude \
                 only: no shared united-defense total exists anywhere in this engine for it to layer \
                 onto"
            ),
        });
    }
}

// SD-34 wave 49 (`decisions.md §22`'s WAVE 49 UPDATE): 33 more registered
// prestige classes in sub-mechanism-5's "registered prestige class,
// magnitude-only" remainder, each grounded exactly like the classes
// above -- one `ground_<class>_class_features` function per class, called
// unconditionally from `compute_pilot_base_chassis`, matched on the raw
// `class_id` string. Every formula is transcribed directly from its own
// ingested corpus record (`data/corpus/<book>/class_feature/<slug>/...`),
// independently cross-checked against the real, non-ingested PCGen oracle
// where the corpus record's own token needed an external class-table
// cross-reference (cited per function). This wave prioritizes breadth: a
// short doc comment per function citing its own literal token, not the
// long prose established in earlier waves.

// ---- Cyphermage (inner_sea_magic / adventurers_guide) ----

/// `ism_abilities_class.lst`, `KEY:Cyphermage ~ Analyze Scroll`:
/// `BONUS:VAR|AnalyzeScrollBonus|CyphermageLVL`.
pub(super) fn cyphermage_analyze_scroll_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `ag_abilities_class.lst`, `KEY:Cyphermage ~ Cypher Lore`:
/// `BONUS:ABILITYPOOL|Cypher Lore Choice|min(9,CyphermageLVL)` (the
/// `adventurers_guide` printing's own capped pool size).
pub(super) fn cyphermage_cypher_lore_pool_size_capped(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level).min(9))
}

/// `ism_abilities_class.lst`, `KEY:Cyphermage ~ Cypher Lore`:
/// `BONUS:ABILITYPOOL|Cypher Lore|CyphermageLVL` (the `inner_sea_magic`
/// printing's own uncapped pool size -- a real corpus discrepancy between
/// two printings of the same feature, transcribed literally per-printing,
/// not reconciled).
pub(super) fn cyphermage_cypher_lore_pool_size_uncapped(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn ground_cyphermage_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == CYPHERMAGE_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = cyphermage_analyze_scroll_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_magic.cyphermage.analyze_scroll.bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|AnalyzeScrollBonus|CyphermageLVL`
                "Cyphermage level {level} Analyze Scroll: +{bonus} on Use Magic Device checks to \
                 activate scrolls"
            ),
        });
    }
    if let Some(pool) = cyphermage_cypher_lore_pool_size_capped(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.cyphermage.cypher_lore.pool_size".to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Cypher Lore Choice|min(9,CyphermageLVL)`
                "Cyphermage level {level} Cypher Lore (adventurers_guide printing): a pool of \
                 {pool}. Grounds the pool SIZE only"
            ),
        });
    }
    if let Some(pool) = cyphermage_cypher_lore_pool_size_uncapped(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_magic.cyphermage.cypher_lore.pool_size".to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Cypher Lore|CyphermageLVL`, no cap on this printing
                "Cyphermage level {level} Cypher Lore (inner_sea_magic printing): a pool of {pool}. \
                 Grounds the pool SIZE only"
            ),
        });
    }
}

// ---- Psychic Fist (ultimate_psionics) ----

/// `up_abilities_class.lst`, `KEY:Psychic Fist ~ Infused Body`:
/// `BONUS:VAR|InfusedBody|floor(PsychicFistLVL/3)`.
pub(super) fn psychic_fist_infused_body_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 3)
}

/// `up_abilities_class.lst`, `KEY:Psychic Fist ~ Ki Power`:
/// `BONUS:VAR|KiPower|PsychicFistLVL/2`.
pub(super) fn psychic_fist_ki_power_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// `up_abilities_class.lst`, `KEY:Psychic Fist ~ Mesmerizing Glow`:
/// `BONUS:VAR|MesmerizingGlowTargets|PsychicFistLVL/2` -- the record's own
/// second token (`MesmerizingGlowDC|14+PsychicFistPrimeStat`) needs
/// `PsychicFistPrimeStat`, a cross-class "which parent psionic class fed
/// the prime manifesting stat" resolution this engine does not yet model
/// (the same shape this bundle already excludes for Phrenic Slayer's own
/// remainder) -- not grounded here.
pub(super) fn psychic_fist_mesmerizing_glow_targets(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

pub(super) fn ground_psychic_fist_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == PSYCHIC_FIST_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = psychic_fist_infused_body_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.psychic_fist.infused_body.bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|InfusedBody|floor(PsychicFistLVL/3)`
                "Psychic Fist level {level} Infused Body: +{bonus} natural armor"
            ),
        });
    }
    if let Some(bonus) = psychic_fist_ki_power_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.psychic_fist.ki_power.bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|KiPower|PsychicFistLVL/2`
                "Psychic Fist level {level} Ki Power: {bonus} extra ki points"
            ),
        });
    }
    if let Some(targets) = psychic_fist_mesmerizing_glow_targets(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.psychic_fist.mesmerizing_glow.targets"
                .to_owned(),
            value: targets,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|MesmerizingGlowTargets|PsychicFistLVL/2`
                "Psychic Fist level {level} Mesmerizing Glow: affects {targets} targets. Grounds the \
                 target-count magnitude only; the ability's own save DC needs `PsychicFistPrimeStat` \
                 (a cross-class parent-entry resolution this engine does not model) and is not \
                 grounded"
            ),
        });
    }
}

// ---- Asavir (adventurers_guide) ----

/// `ag_abilities_class.lst`, `KEY:Asavir ~ Camaraderie`:
/// `BONUS:VAR|AsavirCamaraderieBonus|1` (+1 at level 5, +1 at level 9).
/// Granted from level 1 (`ag_classes.lst` level-1 row).
pub(super) fn asavir_camaraderie_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level >= 5) + i16::from(level >= 9))
}

/// `ag_abilities_class.lst`, `KEY:Asavir ~ Djinni's Blessing`:
/// `BONUS:VAR|AsavirDjinnisBlessingBonus|10` (+10 more at level 8). Granted
/// at level 4 (`ag_classes.lst` level-4 row).
pub(super) fn asavir_djinnis_blessing_bonus(level: u8) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(10 + 10 * i16::from(level >= 8))
}

/// `ag_abilities_class.lst`, `KEY:Asavir ~ Efreeti's Blessing` (mount
/// variant): `BONUS:VAR|FireResistanceBonus|5|TYPE=Resistance`. Granted at
/// level 8 (`ag_classes.lst` level-8 row).
pub(super) fn asavir_efreeti_blessing_fire_resistance(level: u8) -> Option<i16> {
    if level < 8 {
        return None;
    }
    Some(5)
}

/// `ag_abilities_class.lst`, `KEY:Asavir ~ Equine Bond`:
/// `BONUS:VAR|AsavirAnimalCompanionLVL|AsavirLVL+2` (also restated onto
/// `AnimalCompanionMasterLVL`). Granted from level 1.
pub(super) fn asavir_equine_bond_companion_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) + 2)
}

/// `ag_abilities_class.lst`, `KEY:Asavir ~ Janni's Blessing` (base and
/// mount variants both carry the identical token):
/// `BONUS:SAVE|ALL|1|TYPE=Luck`. Granted at level 10 (`ag_classes.lst`
/// level-10 row).
pub(super) fn asavir_jannis_blessing_luck_save(level: u8) -> Option<i16> {
    if level < 10 {
        return None;
    }
    Some(1)
}

/// `ag_abilities_class.lst`, `KEY:Asavir ~ Marid's Blessing` (mount
/// variant): `BONUS:SAVE|Reflex|2|TYPE=Racial`. Granted at level 6
/// (`ag_classes.lst` level-6 row).
pub(super) fn asavir_marids_blessing_reflex_save(level: u8) -> Option<i16> {
    if level < 6 {
        return None;
    }
    Some(2)
}

/// `ag_abilities_class.lst`, `KEY:Asavir ~ Shaitan's Blessing`:
/// `BONUS:VAR|AsavirShaitansBlessingBonus|2` (+2 more at level 9). Granted
/// at level 2 (`ag_classes.lst` level-2 row).
pub(super) fn asavir_shaitans_blessing_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(2 + 2 * i16::from(level >= 9))
}

/// `ag_abilities_class.lst`, `KEY:Asavir ~ Thunderous Charge`:
/// `BONUS:VAR|AsavirThunderousChargeBonus|5` (+5 more at level 6, +10 more
/// at level 10). Granted at level 2.
pub(super) fn asavir_thunderous_charge_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(5 + 5 * i16::from(level >= 6) + 10 * i16::from(level >= 10))
}

pub(super) fn ground_asavir_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ASAVIR_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = asavir_camaraderie_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.asavir.camaraderie.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Asavir level {level} Camaraderie: +{bonus} (corpus `AsavirCamaraderieBonus`, \
                 stepping at levels 5 and 9)"
            ),
        });
    }
    if let Some(bonus) = asavir_djinnis_blessing_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.asavir.djinnis_blessing.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Asavir level {level} Djinni's Blessing: +{bonus} (corpus \
                 `AsavirDjinnisBlessingBonus`, stepping at level 8)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.asavir.djinnis_blessing_mount.move_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:MOVEADD|TYPE.Walk|MASTERVAR(\"AsavirDjinnisBlessingBonus\")`, restating
                //   the same bonus above
                "Asavir level {level} Djinni's Blessing (mount): +{bonus} ft. mounted movement"
            ),
        });
    }
    if let Some(fire_resist) = asavir_efreeti_blessing_fire_resistance(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.asavir.efreeti_blessing_mount.fire_resistance"
                .to_owned(),
            value: fire_resist,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|FireResistanceBonus|5|TYPE=Resistance`
                "Asavir level {level} Efreeti's Blessing (mount): fire resistance {fire_resist}"
            ),
        });
    }
    if let Some(companion_level) = asavir_equine_bond_companion_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.asavir.equine_bond.companion_level".to_owned(),
            value: companion_level,
            detail: format!(
                "Asavir level {level} Equine Bond: animal companion effective level \
                 {companion_level} (corpus `AsavirAnimalCompanionLVL = AsavirLVL+2`)"
            ),
        });
    }
    if let Some(save) = asavir_jannis_blessing_luck_save(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.asavir.jannis_blessing.luck_save".to_owned(),
            value: save,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SAVE|ALL|1|TYPE=Luck`
                "Asavir level {level} Janni's Blessing: +{save} luck bonus on all saves"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.asavir.jannis_blessing_mount.luck_save"
                .to_owned(),
            value: save,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SAVE|ALL|1|TYPE=Luck`, restating the same base record
                "Asavir level {level} Janni's Blessing (mount): +{save} luck bonus on all saves"
            ),
        });
    }
    if let Some(save) = asavir_marids_blessing_reflex_save(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.asavir.marids_blessing_mount.reflex_save"
                .to_owned(),
            value: save,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SAVE|Reflex|2|TYPE=Racial`
                "Asavir level {level} Marid's Blessing (mount): +{save} racial bonus on Reflex \
                 saves"
            ),
        });
    }
    if let Some(bonus) = asavir_shaitans_blessing_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.asavir.shaitans_blessing.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Asavir level {level} Shaitan's Blessing: +{bonus} (corpus \
                 `AsavirShaitansBlessingBonus`, stepping at level 9)"
            ),
        });
    }
    if let Some(bonus) = asavir_thunderous_charge_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.asavir.thunderous_charge.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Asavir level {level} Thunderous Charge: +{bonus} damage (corpus \
                 `AsavirThunderousChargeBonus`, stepping at levels 6 and 10)"
            ),
        });
    }
}

// ---- Metamorph (ultimate_psionics) ----

/// `up_abilities_class.lst`, `KEY:Metamorph ~ Alter Metamorphosis`:
/// `BONUS:VAR|AlterMetamorphosisLVL|MetamorphLVL`.
pub(super) fn metamorph_alter_metamorphosis_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst`, `KEY:Metamorph ~ Free Shift`:
/// `BONUS:VAR|FreeShiftTimes|(MetamorphLVL/2)`.
pub(super) fn metamorph_free_shift_times(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// `up_abilities_class.lst`, `KEY:Metamorph ~ Natural Shifter`:
/// `BONUS:VAR|NaturalShifter|floor((MetamorphLVL+4)/5)`.
pub(super) fn metamorph_natural_shifter_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 4) / 5)
}

pub(super) fn ground_metamorph_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == METAMORPH_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(value) = metamorph_alter_metamorphosis_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.metamorph.alter_metamorphosis.level"
                .to_owned(),
            value,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|AlterMetamorphosisLVL|MetamorphLVL`
                "Metamorph level {level} Alter Metamorphosis: `AlterMetamorphosisLVL` = {value}"
            ),
        });
    }
    if let Some(times) = metamorph_free_shift_times(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.metamorph.free_shift.times".to_owned(),
            value: times,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|FreeShiftTimes|(MetamorphLVL/2)`
                "Metamorph level {level} Free Shift: {times} times per day"
            ),
        });
    }
    if let Some(bonus) = metamorph_natural_shifter_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.metamorph.natural_shifter.bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|NaturalShifter|floor((MetamorphLVL+4)/5)`
                "Metamorph level {level} Natural Shifter: +{bonus} caster level on Metamorphosis \
                 spells"
            ),
        });
    }
}

// ---- War Mind (ultimate_psionics) ----

/// `up_abilities_class.lst`, `KEY:War Mind ~ Chain of Defensive Posture`:
/// `BONUS:VAR|DefensiveChain|2*floor((WarMindLVL+4)/6)`.
pub(super) fn war_mind_chain_of_defensive_posture_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 * ((i16::from(level) + 4) / 6))
}

/// `up_abilities_class.lst`, `KEY:War Mind ~ Chain of Personal
/// Superiority`: `BONUS:VAR|SuperiorityChain|2*floor((WarMindLVL+5)/6)`.
pub(super) fn war_mind_chain_of_personal_superiority_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 * ((i16::from(level) + 5) / 6))
}

/// `up_abilities_class.lst`, `KEY:War Mind ~ Enduring Body`:
/// `BONUS:VAR|EnduringBody|floor(WarMindLVL/3)`.
pub(super) fn war_mind_enduring_body_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 3)
}

pub(super) fn ground_war_mind_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == WAR_MIND_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = war_mind_chain_of_defensive_posture_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.war_mind.chain_of_defensive_posture.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|DefensiveChain|2*floor((WarMindLVL+4)/6)`
                "War Mind level {level} Chain of Defensive Posture: +{bonus} natural armor"
            ),
        });
    }
    if let Some(bonus) = war_mind_chain_of_personal_superiority_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.war_mind.chain_of_personal_superiority.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|SuperiorityChain|2*floor((WarMindLVL+5)/6)`
                "War Mind level {level} Chain of Personal Superiority: +{bonus}"
            ),
        });
    }
    if let Some(bonus) = war_mind_enduring_body_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.war_mind.enduring_body.bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|EnduringBody|floor(WarMindLVL/3)`
                "War Mind level {level} Enduring Body: +{bonus} hit points per Hit Die"
            ),
        });
    }
}

// ---- Hellknight (adventurers_guide / inner_sea_world_guide) ----

/// `ag_abilities_class.lst`, `KEY:Detect Chaos ~ HK`: `SPELLS:Hellknight|
/// TIMES=ATWILL|CASTERLEVEL=TL|Detect Chaos,11+CHA`. Granted at level 2
/// (`ag_classes.lst` level-2 row).
pub(super) fn hellknight_detect_chaos_dc(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(11 + charisma_modifier)
}

/// `ag_abilities_class.lst`, `KEY:Discern Lies ~ HK`: `SPELLS:Hellknight|
/// TIMES=3+CHA|CASTERLEVEL=TL|Discern Lies,14+CHA`. Granted at level 2.
pub(super) fn hellknight_discern_lies_uses_per_day(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(3 + charisma_modifier)
}

pub(super) fn hellknight_discern_lies_dc(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(14 + charisma_modifier)
}

/// `ag_abilities_class.lst`, `KEY:Smite Chaos ~ HK`: `DEFINE:
/// HKSmiteTimes|0`, with the real formula on `ag_classes.lst`'s own
/// level-1 row: `BONUS:VAR|HKSmiteTimes|(CL+2)/3` -- `CL` here is this
/// class's own raw level (the same "bare `CL` on a class's own table row
/// means that class's own level, not total character level" idiom already
/// confirmed for `PaDLVL|CL`/`TwilightTalonLVL|CL`/`GoldenLegionnaireLVL|
/// CL`, all already shipped in this file treating it as the raw class
/// level, not `total_character_level`). The record's own DESC substitution
/// list (`HKSmiteTimes|max(CHA,0)|HellknightLvl|max(CHA,0)`) names three
/// further magnitudes literally: the attack-roll bonus (`max(CHA,0)`), the
/// damage bonus (`HellknightLvl`, the class's own raw level -- not
/// Charisma, transcribed literally per this bundle's authoritative-token-
/// over-prose discipline), and the deflection AC bonus (`max(CHA,0)`
/// again). Granted at level 1.
pub(super) fn hellknight_smite_chaos_uses_per_day(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 2) / 3)
}

pub(super) fn hellknight_smite_chaos_attack_bonus(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(charisma_modifier.max(0))
}

pub(super) fn hellknight_smite_chaos_damage_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `ag_abilities_class.lst`/`ag_classes.lst`, `KEY:Hellknight Armor ~ HK`:
/// `DEFINE:HellknightArmorLVL|0` (set to `CL`, this class's own raw level,
/// on the class's own level-2 table row -- same "bare `CL`" idiom as
/// `hellknight_smite_chaos_uses_per_day` above) / `BONUS:VAR|
/// HellknightArmorBonus|floor((HellknightArmorLVL+1)/3)`. Granted at
/// level 2. Both the `adventurers_guide` and `inner_sea_world_guide`
/// printings carry the identical record.
pub(super) fn hellknight_armor_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some((i16::from(level) + 1) / 3)
}

pub(super) fn ground_hellknight_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == HELLKNIGHT_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };
    let charisma = ability_modifiers.charisma;

    if let Some(dc) = hellknight_detect_chaos_dc(level, charisma) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.hellknight.detect_chaos.dc".to_owned(),
            value: dc,
            detail: format!(
                "Hellknight level {level} Detect Chaos, at will, DC {dc} (corpus \
                 `SPELLS:Hellknight|TIMES=ATWILL|...|Detect Chaos,11+CHA`)"
            ),
        });
    }
    if let Some(uses) = hellknight_discern_lies_uses_per_day(level, charisma) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.hellknight.discern_lies.uses_per_day"
                .to_owned(),
            value: uses,
            detail: format!(
                "Hellknight level {level} Discern Lies: {uses} times per day (corpus \
                 `SPELLS:Hellknight|TIMES=3+CHA|...`)"
            ),
        });
    }
    if let Some(dc) = hellknight_discern_lies_dc(level, charisma) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.hellknight.discern_lies.dc".to_owned(),
            value: dc,
            detail: format!(
                "Hellknight level {level} Discern Lies, DC {dc} (corpus `...Discern \
                 Lies,14+CHA`)"
            ),
        });
    }
    if let Some(uses) = hellknight_smite_chaos_uses_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.hellknight.smite_chaos.uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|HKSmiteTimes|(CL+2)/3`,
                "Hellknight level {level} Smite Chaos: {uses} times per day (corpus ` `CL` = this \
                 class's own level {level})"
            ),
        });
    }
    if let Some(bonus) = hellknight_smite_chaos_attack_bonus(level, charisma) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.hellknight.smite_chaos.attack_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Hellknight level {level} Smite Chaos: +{bonus} attack roll against the smited \
                 target (corpus DESC substitution `max(CHA,0)`)"
            ),
        });
    }
    if let Some(bonus) = hellknight_smite_chaos_damage_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.hellknight.smite_chaos.damage_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Hellknight level {level} Smite Chaos: +{bonus} damage against the smited \
                 target (corpus DESC substitution `HellknightLvl`, the class's own raw level, \
                 transcribed literally rather than assumed to be Charisma)"
            ),
        });
    }
    if let Some(bonus) = hellknight_smite_chaos_attack_bonus(level, charisma) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.hellknight.smite_chaos.deflection_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Hellknight level {level} Smite Chaos: +{bonus} deflection bonus to AC against \
                 the smited target's attacks (corpus DESC substitution `max(CHA,0)`)"
            ),
        });
    }
    if let Some(bonus) = hellknight_armor_bonus(level) {
        for book in ["adventurers_guide", "inner_sea_world_guide"] {
            explanations.push(ComputationExplanation {
                id: format!("class_feature.{book}.hellknight.hellknight_armor.bonus"),
                value: bonus,
                detail: format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   BONUS:VAR|HellknightArmorBonus|floor((HellknightArmorLVL+1)/3)`,
                    "Hellknight level {level} Hellknight Armor: reduces armor check penalty and \
                     raises max Dexterity bonus by {bonus} while wearing Hellknight armor (corpus ` \
                     `HellknightArmorLVL` = this class's own level {level})"
                ),
            });
            explanations.push(ComputationExplanation {
                id: format!("class_feature.{book}.hellknight.hellknight_armor_benefits.bonus"),
                value: bonus,
                detail: format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   `BONUS:MISC|MAXDEX,ACCHECK|HellknightArmorBonus`
                    "Hellknight level {level} Hellknight Armor Benefits: restates the same +{bonus} \
                     armor-check/max-Dex bonus above"
                ),
            });
        }
    }
}

// ---- Adaptive Warrior (ultimate_psionics) ----

/// `up_abilities_class.lst`, `KEY:Adaptive Warrior ~ Combine Fighting
/// Styles`: `BONUS:VAR|CombineFightingStylesTimes|4+INT`.
pub(super) fn adaptive_warrior_combine_fighting_styles_times(
    level: u8,
    intelligence_modifier: i16,
) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(4 + intelligence_modifier)
}

/// `up_abilities_class.lst`, `KEY:Adaptive Warrior ~ Counter Fighting
/// Style`: `BONUS:VAR|CounterFightingStyleBonus|max(1,AdaptiveWarriorLVL/2)`.
pub(super) fn adaptive_warrior_counter_fighting_style_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) / 2).max(1))
}

/// `up_abilities_class.lst`, `KEY:Adaptive Warrior ~ Examine Technique`:
/// `BONUS:VAR|ExamineTechniqueTargets|AdaptiveWarriorLVL`.
pub(super) fn adaptive_warrior_examine_technique_targets(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst`, `KEY:Adaptive Warrior ~ Extended
/// Examination`: `BONUS:VAR|ExtendedExaminationBonus|AdaptiveWarriorLVL`.
pub(super) fn adaptive_warrior_extended_examination_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst`, `KEY:Adaptive Warrior ~ Mimic Skill`:
/// `BONUS:VAR|MimicSkillRanks|AdaptiveWarriorLVL`.
pub(super) fn adaptive_warrior_mimic_skill_ranks(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn ground_adaptive_warrior_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ADAPTIVE_WARRIOR_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(times) =
        adaptive_warrior_combine_fighting_styles_times(level, ability_modifiers.intelligence)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.adaptive_warrior.combine_fighting_styles.\
                 times_per_day"
                .to_owned(),
            value: times,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|CombineFightingStylesTimes|4+INT`
                "Adaptive Warrior level {level} Combine Fighting Styles: {times} times per day"
            ),
        });
    }
    if let Some(bonus) = adaptive_warrior_counter_fighting_style_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.adaptive_warrior.counter_fighting_style.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|CounterFightingStyleBonus|max(1,AdaptiveWarriorLVL/2)`
                "Adaptive Warrior level {level} Counter Fighting Style: +{bonus}"
            ),
        });
    }
    if let Some(targets) = adaptive_warrior_examine_technique_targets(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.adaptive_warrior.examine_technique.targets"
                .to_owned(),
            value: targets,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|ExamineTechniqueTargets|AdaptiveWarriorLVL`
                "Adaptive Warrior level {level} Examine Technique: {targets}"
            ),
        });
    }
    if let Some(bonus) = adaptive_warrior_extended_examination_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.adaptive_warrior.extended_examination.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|ExtendedExaminationBonus|AdaptiveWarriorLVL`
                "Adaptive Warrior level {level} Extended Examination: +{bonus}"
            ),
        });
    }
    if let Some(ranks) = adaptive_warrior_mimic_skill_ranks(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.adaptive_warrior.mimic_skill.ranks".to_owned(),
            value: ranks,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|MimicSkillRanks|AdaptiveWarriorLVL`
                "Adaptive Warrior level {level} Mimic Skill: {ranks} ranks"
            ),
        });
    }
}

// ---- Sanguine Angel (adventurers_guide) ----

/// `ag_abilities_class.lst`, `KEY:Sanguine Angel ~ Armored Angel`:
/// `BONUS:VAR|ArmorTrainingLVL|SanguineAngelLVL|PREEQUIP:1,Gray Maiden
/// Plate%`. Grounds the level magnitude only; this engine does not track
/// which armor a character has equipped, so the `PREEQUIP` gate itself is
/// not modelled. Granted from level 1 (no `PREVARGTEQ` on the token
/// itself).
pub(super) fn sanguine_angel_armored_angel_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `ag_abilities_class.lst`, `KEY:Sanguine Angel ~ Mystique of Ardad
/// Lili`: `SPELLS:Innate|CASTERLEVEL=TL|Dominate Person,10+TL/2+CHA` --
/// the classic "ground the SLA triple" idiom (caster level and save DC),
/// `TL` = total character level.
pub(super) fn sanguine_angel_mystique_dc(total_level: i16, charisma_modifier: i16) -> i16 {
    10 + total_level / 2 + charisma_modifier
}

pub(super) fn ground_sanguine_angel_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == SANGUINE_ANGEL_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(value) = sanguine_angel_armored_angel_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.sanguine_angel.armored_angel.level".to_owned(),
            value,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|ArmorTrainingLVL|SanguineAngelLVL|...`
                "Sanguine Angel level {level} Armored Angel: `ArmorTrainingLVL` = {value}. Grounds \
                 the magnitude only; this engine does not track whether Gray Maiden Plate is \
                 equipped"
            ),
        });
    }
    if level >= 1 {
        let total_level = total_character_level(input);
        let dc = sanguine_angel_mystique_dc(total_level, ability_modifiers.charisma);
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.sanguine_angel.mystique_of_ardad_lili.caster_\
                 level"
                .to_owned(),
            value: total_level,
            detail: format!(
                "Sanguine Angel Mystique of Ardad Lili: Dominate Person spell-like ability, \
                 caster level {total_level} (corpus `SPELLS:Innate|CASTERLEVEL=TL|...`)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.sanguine_angel.mystique_of_ardad_lili.dc"
                .to_owned(),
            value: dc,
            detail: format!(
                "Sanguine Angel Mystique of Ardad Lili: Dominate Person save DC {dc} (corpus \
                 `Dominate Person,10+TL/2+CHA`)"
            ),
        });
    }
}

// ---- Body Snatcher (ultimate_psionics) ----

/// `up_abilities_class.lst`, `KEY:Body Snatcher ~ Body Thief`:
/// `BONUS:CASTERLEVEL|SPELL.Mind Switch|BodySnatcherLVL`.
pub(super) fn body_snatcher_body_thief_caster_level_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst`, `KEY:Body Snatcher ~ Death Is Only the
/// Beginning`: `BONUS:CASTERLEVEL|SPELL.Mind Switch (True)|BodySnatcherLVL`.
pub(super) fn body_snatcher_death_is_only_the_beginning_caster_level_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst`, `KEY:Body Snatcher ~ Melding Exchange`:
/// `BONUS:VAR|MeldingExchangeBonus|2*BodySnatcherLVL`.
pub(super) fn body_snatcher_melding_exchange_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 * i16::from(level))
}

pub(super) fn ground_body_snatcher_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == BODY_SNATCHER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = body_snatcher_body_thief_caster_level_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.body_snatcher.body_thief.caster_level_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:CASTERLEVEL|SPELL.Mind Switch|BodySnatcherLVL`
                "Body Snatcher level {level} Body Thief: +{bonus} caster level on Mind Switch. \
                 Grounds the magnitude only: no Mind Switch total exists anywhere in this engine for \
                 it to layer onto"
            ),
        });
    }
    if let Some(bonus) = body_snatcher_death_is_only_the_beginning_caster_level_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.body_snatcher.death_is_only_the_beginning.\
                 caster_level_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:CASTERLEVEL|SPELL.Mind Switch (True)|BodySnatcherLVL`
                "Body Snatcher level {level} Death Is Only the Beginning: +{bonus} caster level on \
                 Mind Switch (True)"
            ),
        });
    }
    if let Some(bonus) = body_snatcher_melding_exchange_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.body_snatcher.melding_exchange.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|MeldingExchangeBonus|2*BodySnatcherLVL`
                "Body Snatcher level {level} Melding Exchange: +{bonus}"
            ),
        });
    }
}

// ---- Steel Falcon (adventurers_guide) ----

/// `ag_abilities_class.lst`, `KEY:Steel Falcon ~ Chainbreaker`:
/// `BONUS:SITUATION|Escape Artist=Escape manacles or ropes|10`.
pub(super) fn steel_falcon_chainbreaker_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10)
}

/// `ag_abilities_class.lst`, `KEY:Steel Falcon ~ Enemy of Slavers`:
/// `BONUS:VAR|EnemyOfSlavers|2*(1+(SteelFalconLVL>=5)+(SteelFalconLVL>=9))`.
pub(super) fn steel_falcon_enemy_of_slavers_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 * (1 + i16::from(level >= 5) + i16::from(level >= 9)))
}

/// `ag_abilities_class.lst`, `KEY:Steel Falcon ~ Sailor and Survivalist`:
/// `BONUS:SKILL|Profession (sailor)|SteelFalconLVL` (and the identical
/// value restated for Survival checks made to get along in the wild or
/// navigate).
pub(super) fn steel_falcon_sailor_and_survivalist_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn ground_steel_falcon_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == STEEL_FALCON_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = steel_falcon_chainbreaker_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.steel_falcon.chainbreaker.bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SITUATION|Escape Artist=...|10`
                "Steel Falcon level {level} Chainbreaker: +{bonus} to escape manacles or ropes"
            ),
        });
    }
    if let Some(bonus) = steel_falcon_enemy_of_slavers_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.steel_falcon.enemy_of_slavers.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Steel Falcon level {level} Enemy of Slavers: +{bonus} (corpus \
                 `EnemyOfSlavers|2*(1+(SteelFalconLVL>=5)+(SteelFalconLVL>=9))`)"
            ),
        });
    }
    if let Some(bonus) = steel_falcon_sailor_and_survivalist_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.steel_falcon.sailor_and_survivalist.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SKILL|Profession (sailor)|SteelFalconLVL`
                "Steel Falcon level {level} Sailor and Survivalist: +{bonus} on Profession (sailor) \
                 and relevant Survival checks"
            ),
        });
    }
    if level >= 1 {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.steel_falcon.talmandor_s_blessing.acrobatics_\
                 bonus"
                .to_owned(),
            value: 10,
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:SITUATION|Acrobatics=When Jumping|10`
            detail: "Steel Falcon Talmandor's Blessing: +10 on Acrobatics checks made when jumping"
                .to_owned(),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.steel_falcon.talmandor_s_blessing.perception_\
                 bonus"
                .to_owned(),
            value: 4,
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:SKILL|Perception|4`
            detail: "Steel Falcon Talmandor's Blessing: +4 on Perception checks"
                .to_owned(),
        });
    }
}

// ---- Lantern Bearer (adventurers_guide) ----

/// `ag_abilities_class.lst`, `KEY:Lantern Bearer ~ Favored Enemy`:
/// `BONUS:ABILITYPOOL|Lantern Bearer Favored Enemy|1+(LanternBearerLVL>=8)`.
pub(super) fn lantern_bearer_favored_enemy_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level >= 8))
}

/// Same record's second pool: `BONUS:ABILITYPOOL|Lantern Bearer Favored
/// Enemy Bonus|1|PREVARGTEQ:LanternBearerLVL,8`.
pub(super) fn lantern_bearer_favored_enemy_bonus_pool_size(level: u8) -> Option<i16> {
    if level < 8 {
        return None;
    }
    Some(1)
}

/// `ag_abilities_class.lst`, `KEY:Lantern Bearer ~ Proven Weapon
/// Familiarity`: `BONUS:WEAPONPROF=<group>|DAMAGE,TOHIT|1` (identical +1
/// across every listed weapon group).
pub(super) fn lantern_bearer_proven_weapon_familiarity_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1)
}

/// `ag_abilities_class.lst`, `KEY:Lantern Bearer ~ Superior
/// Discernment`: `BONUS:ABILITYPOOL|Lantern Bearer Discernment|1`.
pub(super) fn lantern_bearer_superior_discernment_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1)
}

pub(super) fn ground_lantern_bearer_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == LANTERN_BEARER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(pool) = lantern_bearer_favored_enemy_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.lantern_bearer.favored_enemy.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Lantern Bearer Favored Enemy|1+(LanternBearerLVL>=8)`
                "Lantern Bearer level {level} Favored Enemy: a pool of {pool}. Grounds the pool SIZE \
                 only"
            ),
        });
    }
    if let Some(pool) = lantern_bearer_favored_enemy_bonus_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.lantern_bearer.favored_enemy.bonus_pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Lantern Bearer Favored Enemy
                //   Bonus|1|PREVARGTEQ:LanternBearerLVL,8`
                "Lantern Bearer level {level} Favored Enemy Bonus: a pool of {pool}"
            ),
        });
    }
    if let Some(bonus) = lantern_bearer_proven_weapon_familiarity_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.lantern_bearer.proven_weapon_familiarity.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:WEAPONPROF=...|DAMAGE,TOHIT|1`
                "Lantern Bearer level {level} Proven Weapon Familiarity: +{bonus} damage and attack \
                 with longbow, shortbow, longsword, short sword, and elven weapons"
            ),
        });
    }
    if let Some(pool) = lantern_bearer_superior_discernment_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.lantern_bearer.superior_discernment.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Lantern Bearer Discernment|1`
                "Lantern Bearer level {level} Superior Discernment: a pool of {pool}"
            ),
        });
    }
}

// ---- Storm Kindler (adventurers_guide) ----

/// `ag_abilities_class.lst`, `KEY:Storm Kindler ~ Aura of Calm`:
/// `BONUS:VAR|StormKindlerAuraRadius|5` (+5 more at levels 5, 7, 9).
pub(super) fn storm_kindler_aura_of_calm_radius(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(5 + 5 * i16::from(level >= 5) + 5 * i16::from(level >= 7) + 5 * i16::from(level >= 9))
}

/// Same record's second magnitude: `BONUS:VAR|StormKindlerAuraBonus|2`
/// (+2 more at level 7).
pub(super) fn storm_kindler_aura_of_calm_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 + 2 * i16::from(level >= 7))
}

/// `ag_abilities_class.lst`, `KEY:Storm Kindler ~ Oceanic Spirit`:
/// `BONUS:VAR|OceanicSpiritBonus|5` (+5 more at level 5, +10 more at
/// level 9).
pub(super) fn storm_kindler_oceanic_spirit_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(5 + 5 * i16::from(level >= 5) + 10 * i16::from(level >= 9))
}

/// `ag_abilities_class.lst`, `KEY:Storm Kindler ~ Storm Shape`:
/// `BONUS:VAR|StormShapeHeight|10*(1+min(5,StormKindlerLVL/2))`.
pub(super) fn storm_kindler_storm_shape_height(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 * (1 + (i16::from(level) / 2).min(5)))
}

/// `ag_abilities_class.lst`, `KEY:Storm Kindler ~ Weather's Fury`:
/// `BONUS:VAR|StormKindlerFury|3+(StormKindlerLVL>=8)+(StormKindlerLVL>=10)`.
pub(super) fn storm_kindler_weathers_fury_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(3 + i16::from(level >= 8) + i16::from(level >= 10))
}

pub(super) fn ground_storm_kindler_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == STORM_KINDLER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(radius) = storm_kindler_aura_of_calm_radius(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.storm_kindler.aura_of_calm.radius".to_owned(),
            value: radius,
            detail: format!(
                "Storm Kindler level {level} Aura of Calm: {radius} ft. radius (corpus \
                 `StormKindlerAuraRadius`, stepping at levels 5, 7, 9)"
            ),
        });
    }
    if let Some(bonus) = storm_kindler_aura_of_calm_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.storm_kindler.aura_of_calm.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Storm Kindler level {level} Aura of Calm: +{bonus} save bonus within the aura \
                 (corpus `StormKindlerAuraBonus`, stepping at level 7)"
            ),
        });
    }
    if let Some(bonus) = storm_kindler_oceanic_spirit_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.storm_kindler.oceanic_spirit.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Storm Kindler level {level} Oceanic Spirit: +{bonus} electricity/sonic \
                 resistance and Fly/Swim (corpus `OceanicSpiritBonus`, stepping at levels 5, 9)"
            ),
        });
    }
    if let Some(height) = storm_kindler_storm_shape_height(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.storm_kindler.storm_shape.height".to_owned(),
            value: height,
            detail: format!(
                "Storm Kindler level {level} Storm Shape: {height} ft. tall (corpus \
                 `StormShapeHeight|10*(1+min(5,StormKindlerLVL/2))`)"
            ),
        });
    }
    if let Some(bonus) = storm_kindler_weathers_fury_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.storm_kindler.weathers_fury.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Storm Kindler level {level} Weather's Fury: +{bonus} damage per die (corpus \
                 `StormKindlerFury|3+(StormKindlerLVL>=8)+(StormKindlerLVL>=10)`)"
            ),
        });
    }
}

// ---- Westcrown Devil (adventurers_guide) ----

/// `ag_abilities_class.lst`, `KEY:Westcrown Devil ~ Council's Secret`:
/// `BONUS:ABILITYPOOL|Rogue Talent|WestcrownDevilLVL/2`.
pub(super) fn westcrown_devil_council_secret_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// `ag_abilities_class.lst`, `KEY:Westcrown Devil ~ Founders' Favor`:
/// `BONUS:VAR|FoundersFavorPool|WestcrownDevilLVL+max(INT,WIS,CHA)`.
pub(super) fn westcrown_devil_founders_favor_pool(level: u8, max_mental_modifier: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) + max_mental_modifier)
}

/// Same record's second magnitude: `BONUS:VAR|FoundersFavorDC|
/// 10+WestcrownDevilLVL/2+max(INT,WIS,CHA)`.
pub(super) fn westcrown_devil_founders_favor_dc(level: u8, max_mental_modifier: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 + i16::from(level) / 2 + max_mental_modifier)
}

/// `ag_abilities_class.lst`, `KEY:Westcrown Devil ~ Sneak Attack`:
/// `BONUS:VAR|SneakAttackDice|WestcrownDevilLVL/3`.
pub(super) fn westcrown_devil_sneak_attack_dice(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 3)
}

pub(super) fn ground_westcrown_devil_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == WESTCROWN_DEVIL_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };
    let max_mental = ability_modifiers
        .intelligence
        .max(ability_modifiers.wisdom)
        .max(ability_modifiers.charisma);

    if let Some(pool) = westcrown_devil_council_secret_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.westcrown_devil.council_s_secret.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Rogue Talent|WestcrownDevilLVL/2`
                "Westcrown Devil level {level} Council's Secret: a pool of {pool} rogue talents"
            ),
        });
    }
    if let Some(pool) = westcrown_devil_founders_favor_pool(level, max_mental) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.westcrown_devil.founders_favor.pool".to_owned(),
            value: pool,
            detail: format!(
                "Westcrown Devil level {level} Founders' Favor: a pool of {pool} (corpus \
                 `FoundersFavorPool|WestcrownDevilLVL+max(INT,WIS,CHA)`)"
            ),
        });
    }
    if let Some(dc) = westcrown_devil_founders_favor_dc(level, max_mental) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.westcrown_devil.founders_favor.dc".to_owned(),
            value: dc,
            detail: format!(
                "Westcrown Devil level {level} Founders' Favor DC {dc} (corpus \
                 `FoundersFavorDC|10+WestcrownDevilLVL/2+max(INT,WIS,CHA)`)"
            ),
        });
    }
    if let Some(dice) = westcrown_devil_sneak_attack_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.westcrown_devil.sneak_attack.dice".to_owned(),
            value: dice,
            detail: format!(
                "Westcrown Devil level {level} Sneak Attack: +{dice}d6 (corpus \
                 `SneakAttackDice|WestcrownDevilLVL/3`)"
            ),
        });
    }
}

// ---- Pyrokineticist (ultimate_psionics) ----

pub(super) fn pyrokineticist_bolt_of_fire_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst`, `KEY:Pyrokineticist ~ Fire Adaptation`:
/// `BONUS:VAR|FireAdaptation|4*floor((PyrokineticistLVL+3)/5)`.
pub(super) fn pyrokineticist_fire_adaptation_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(4 * ((i16::from(level) + 3) / 5))
}

/// Same record's second magnitude: `BONUS:VAR|FireResistanceBonus|
/// 10*floor((PyrokineticistLVL+3)/5)`.
pub(super) fn pyrokineticist_fire_adaptation_resistance(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 * ((i16::from(level) + 3) / 5))
}

/// `up_abilities_class.lst`, `KEY:Pyrokineticist ~ Hand Afire`:
/// `BONUS:VAR|HandAfire|2*floor((PyrokineticistLVL+4)/6)`.
pub(super) fn pyrokineticist_hand_afire_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 * ((i16::from(level) + 4) / 6))
}

/// `up_abilities_class.lst`, `KEY:Pyrokineticist ~ Leech Heat`:
/// `BONUS:VAR|LeechHeat|2*floor((PyrokineticistLVL-3)/3)`. Negative below
/// level 4 -- `None` there.
pub(super) fn pyrokineticist_leech_heat_bonus(level: u8) -> Option<i16> {
    let value = 2 * ((i16::from(level) - 3) / 3);
    if value <= 0 {
        return None;
    }
    Some(value)
}

/// `up_abilities_class.lst`, `KEY:Pyrokineticist ~ Manipulate Blaze`:
/// `BONUS:VAR|ManipulateBlazeRange|(25+5*floor(PyrokineticistLVL/2))`.
pub(super) fn pyrokineticist_manipulate_blaze_range(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(25 + 5 * (i16::from(level) / 2))
}

/// `up_abilities_class.lst`, `KEY:Pyrokineticist ~ Nimbus`:
/// `BONUS:VAR|NimbusDuration|PyrokineticistLVL`.
pub(super) fn pyrokineticist_nimbus_duration(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// Same record's magnitude: `BONUS:VAR|Nimbus|2*floor((PyrokineticistLVL-2)/3)`.
/// Negative below level 3 -- `None` there. (`NimbusTimes` -- a shared
/// variable also incremented by Fire Soul's own token -- is not grounded
/// here: this record's own contribution cannot be cleanly separated from
/// Fire Soul's without double-counting.)
pub(super) fn pyrokineticist_nimbus_bonus(level: u8) -> Option<i16> {
    let value = 2 * ((i16::from(level) - 2) / 3);
    if value <= 0 {
        return None;
    }
    Some(value)
}

pub(super) fn pyrokineticist_penetrating_fire_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// `up_abilities_class.lst`, `KEY:Pyrokineticist ~ Weapon Afire`:
/// `BONUS:VAR|WeaponAfire|2*floor((PyrokineticistLVL)/4)`.
pub(super) fn pyrokineticist_weapon_afire_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 * (i16::from(level) / 4))
}

pub(super) fn ground_pyrokineticist_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == PYROKINETICIST_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = pyrokineticist_bolt_of_fire_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.pyrokineticist.bolt_of_fire.bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|BoltFire|PyrokineticistLVL`
                "Pyrokineticist level {level} Bolt of Fire: +{bonus} fire damage"
            ),
        });
    }
    if let Some(bonus) = pyrokineticist_fire_adaptation_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.pyrokineticist.fire_adaptation.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Pyrokineticist level {level} Fire Adaptation: +{bonus} (corpus \
                 `FireAdaptation|4*floor((PyrokineticistLVL+3)/5)`)"
            ),
        });
    }
    if let Some(resist) = pyrokineticist_fire_adaptation_resistance(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.pyrokineticist.fire_adaptation.fire_resistance"
                .to_owned(),
            value: resist,
            detail: format!(
                "Pyrokineticist level {level} Fire Adaptation: fire resistance {resist} \
                 (corpus `FireResistanceBonus|10*floor((PyrokineticistLVL+3)/5)`)"
            ),
        });
    }
    if let Some(bonus) = pyrokineticist_hand_afire_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.pyrokineticist.hand_afire.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Pyrokineticist level {level} Hand Afire: +{bonus} fire damage (corpus \
                 `HandAfire|2*floor((PyrokineticistLVL+4)/6)`)"
            ),
        });
    }
    if let Some(bonus) = pyrokineticist_leech_heat_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.pyrokineticist.leech_heat.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Pyrokineticist level {level} Leech Heat: +{bonus} (corpus \
                 `LeechHeat|2*floor((PyrokineticistLVL-3)/3)`)"
            ),
        });
    }
    if let Some(range) = pyrokineticist_manipulate_blaze_range(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.pyrokineticist.manipulate_blaze.range"
                .to_owned(),
            value: range,
            detail: format!(
                "Pyrokineticist level {level} Manipulate Blaze: range {range} ft. (corpus \
                 `ManipulateBlazeRange|(25+5*floor(PyrokineticistLVL/2))`)"
            ),
        });
    }
    if let Some(duration) = pyrokineticist_nimbus_duration(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.pyrokineticist.nimbus.duration_rounds"
                .to_owned(),
            value: duration,
            detail: format!(
                "Pyrokineticist level {level} Nimbus: lasts {duration} rounds (corpus \
                 `NimbusDuration|PyrokineticistLVL`)"
            ),
        });
    }
    if let Some(bonus) = pyrokineticist_nimbus_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.pyrokineticist.nimbus.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Pyrokineticist level {level} Nimbus: +{bonus} fire damage per die (corpus \
                 `Nimbus|2*floor((PyrokineticistLVL-2)/3)`)"
            ),
        });
    }
    if let Some(bonus) = pyrokineticist_penetrating_fire_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.pyrokineticist.penetrating_fire.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Pyrokineticist level {level} Penetrating Fire: +{bonus} (corpus \
                 `PenetratingFire|PyrokineticistLVL`)"
            ),
        });
    }
    if let Some(bonus) = pyrokineticist_weapon_afire_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.pyrokineticist.weapon_afire.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Pyrokineticist level {level} Weapon Afire: +{bonus} fire damage (corpus \
                 `WeaponAfire|2*floor((PyrokineticistLVL)/4)`)"
            ),
        });
    }
}

// ---- Aspis Agent (adventurers_guide) ----

pub(super) fn aspis_agent_agency_secrets_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

pub(super) fn aspis_agent_sneak_attack_dice(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level >= 9))
}

pub(super) fn aspis_agent_trap_sense_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level) / 3)
}

pub(super) fn aspis_agent_trapfinding_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) / 2).max(1))
}

pub(super) fn ground_aspis_agent_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ASPIS_AGENT_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(pool) = aspis_agent_agency_secrets_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.aspis_agent.agency_secrets.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                "Aspis Agent level {level} Agency Secrets: a pool of {pool} (corpus \
                 `AspisAgencySecretsCount|AspisAgentLVL/2`)"
            ),
        });
    }
    if let Some(dice) = aspis_agent_sneak_attack_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.aspis_agent.sneak_attack.dice".to_owned(),
            value: dice,
            detail: format!(
                "Aspis Agent level {level} Sneak Attack: +{dice}d6 (corpus `SneakAttackDice|1` \
                 plus 1 more at level 9)"
            ),
        });
    }
    if let Some(bonus) = aspis_agent_trap_sense_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.aspis_agent.trap_sense.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Aspis Agent level {level} Trap Sense: +{bonus} (corpus \
                 `TrapSenseBonus|1+AspisAgentTrapSenseLVL/3`)"
            ),
        });
    }
    if let Some(bonus) = aspis_agent_trapfinding_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.aspis_agent.trapfinding.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Aspis Agent level {level} Trapfinding: +{bonus} (corpus \
                 `TrapfindingBonus|max(TrapfindingLVL/2,1)`)"
            ),
        });
    }
}

// ---- Gray Corsair (adventurers_guide) ----

pub(super) fn gray_corsair_favored_port_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2)
}

pub(super) fn gray_corsair_slaver_slayer_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 * (1 + i16::from(level >= 6) + i16::from(level >= 9)))
}

pub(super) fn ground_gray_corsair_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == GRAY_CORSAIR_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = gray_corsair_favored_port_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.gray_corsair.favored_port.bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SITUATION|...=In favored port|+2`
                "Gray Corsair level {level} Favored Port: +{bonus} on \
                 Bluff/Diplomacy/Intimidate/Knowledge (local) in a favored port"
            ),
        });
    }
    if let Some(bonus) = gray_corsair_slaver_slayer_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.gray_corsair.slaver_slayer.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Gray Corsair level {level} Slaver Slayer: +{bonus} (corpus \
                 `SlaverSlayerBonus|2*(1+(GrayCorsairLVL>=6)+(GrayCorsairLVL>=9))`)"
            ),
        });
    }
}

// ---- Pathfinder Savant (adventurers_guide) ----

/// `ag_classes.lst`'s own level-1 row: `BONUS:VAR|PaSSkillBonus|
/// max(1,CL/2)`, applied on the record's own `BONUS:SKILL|Knowledge
/// (Arcana),Spellcraft,Use Magic Device|PaSSkillBonus` token -- the same
/// "external class-table sets the DEFINE'd variable" cross-file idiom
/// `nature_warden_companion_bond_level` already established. `CL` here is
/// this class's own raw level -- the same "bare `CL` on a class's own
/// table row means that class's own level, not total character level"
/// idiom already confirmed for `PaDLVL|CL`/`TwilightTalonLVL|CL`/
/// `GoldenLegionnaireLVL|CL`, all already shipped in this file treating
/// it as the raw class level.
pub(super) fn pathfinder_savant_master_scholar_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) / 2).max(1))
}

/// `ag_classes.lst`'s own level-2 row: `BONUS:VAR|PaSEsotericSpells|
/// CL-1`, feeding the record's own `BONUS:ABILITYPOOL|Esoteric Magic
/// Spell|PaSEsotericSpells` pool.
pub(super) fn pathfinder_savant_esoteric_magic_pool_size(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(i16::from(level) - 1)
}

/// `ag_classes.lst`'s own level-4 row: `BONUS:VAR|PaSIdentifyTimes|
/// CL/2`, feeding `SPELLS:...|TIMES=PaSIdentifyTimes|...|Identify`.
pub(super) fn pathfinder_savant_quick_identification_times(level: u8) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// `ag_classes.lst`'s own level-5 row: `BONUS:VAR|PaSSaveBonus|CL`.
/// Grounds the magnitude only: the ability's own record carries no
/// mechanical `BONUS:SAVE` token, only a DESC/ASPECT display substitution.
pub(super) fn pathfinder_savant_sigil_master_save_bonus(level: u8) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some(i16::from(level))
}

/// `ag_classes.lst`'s own level-6 row: `BONUS:VAR|PaSAnalyzeDuration|CL`
/// (this class's own raw level again); the record's own `SPELLS:Pathfinder
/// Savant|TIMES=PaSLvl|...|CASTERLEVEL=PaSLvl|Analyze Dweomer` token uses
/// `PaSLvl` directly -- also this class's own raw level (`DEFINE:PaSLVL|0`
/// / `BONUS:VAR|PaSLVL|CL` on the class's own base row).
pub(super) fn pathfinder_savant_analyze_dweomer_times_and_caster_level(level: u8) -> Option<i16> {
    if level < 6 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn ground_pathfinder_savant_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == PATHFINDER_SAVANT_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = pathfinder_savant_master_scholar_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_savant.master_scholar.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Pathfinder Savant level {level} Master Scholar: +{bonus} on Knowledge \
                 (arcana)/Spellcraft/Use Magic Device (corpus `PaSSkillBonus|max(1,CL/2)`)"
            ),
        });
    }
    if let Some(pool) = pathfinder_savant_esoteric_magic_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_savant.esoteric_magic.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                "Pathfinder Savant level {level} Esoteric Magic: a pool of {pool} (corpus \
                 `PaSEsotericSpells|CL-1`)"
            ),
        });
    }
    if let Some(times) = pathfinder_savant_quick_identification_times(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_savant.quick_identification.times_\
                 per_day"
                .to_owned(),
            value: times,
            detail: format!(
                "Pathfinder Savant level {level} Quick Identification: {times} times per day \
                 (corpus `PaSIdentifyTimes|CL/2`)"
            ),
        });
    }
    if let Some(bonus) = pathfinder_savant_sigil_master_save_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_savant.sigil_master.save_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Pathfinder Savant level {level} Sigil Master: +{bonus} on saves against \
                 writing-based magical traps (corpus `PaSSaveBonus|CL`)"
            ),
        });
    }
    if let Some(value) = pathfinder_savant_analyze_dweomer_times_and_caster_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.pathfinder_savant.analyze_dweomer.times_per_\
                 day"
                .to_owned(),
            value,
            detail: format!(
                "Pathfinder Savant level {level} Analyze Dweomer: {value} rounds per day, \
                 caster level {value} (corpus `SPELLS:Pathfinder Savant|TIMES=PaSLvl|...|\
                 CASTERLEVEL=PaSLvl|Analyze Dweomer`, `PaSLvl` = this class's own level)"
            ),
        });
    }
}

// ---- Rivethun Emissary (adventurers_guide) ----

pub(super) fn rivethun_emissary_enhanced_spirit_animal_ep(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 + i16::from(level >= 5))
}

pub(super) fn rivethun_emissary_sixth_sense_uses(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn rivethun_emissary_spirit_animal_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn rivethun_emissary_spirit_bond_hex_dc(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

pub(super) fn rivethun_emissary_spirit_bond_hex_ability_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn ground_rivethun_emissary_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == RIVETHUN_EMISSARY_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(ep) = rivethun_emissary_enhanced_spirit_animal_ep(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.rivethun_emissary.enhanced_spirit_animal.\
                 evolution_points"
                .to_owned(),
            value: ep,
            detail: format!(
                "Rivethun Emissary level {level} Enhanced Spirit Animal: {ep} evolution points \
                 (corpus `FamiliarEP|2` plus 1 more at level 5)"
            ),
        });
    }
    if level >= 1 {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.rivethun_emissary.parley.uses_per_day"
                .to_owned(),
            value: 3,
            detail: "Rivethun Emissary Parley: 3 times per day (corpus `SPELLS:Innate|\
                     TIMES=3|...`)"
                .to_owned(),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.rivethun_emissary.parley.dc".to_owned(),
            value: 12 + ability_modifiers.charisma,
            detail: format!(
                "Rivethun Emissary Parley: Rivethun Calm Spirit save DC {} (corpus \
                 `Rivethun Calm Spirit,12+CHA`)",
                12 + ability_modifiers.charisma
            ),
        });
    }
    if let Some(uses) = rivethun_emissary_sixth_sense_uses(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.rivethun_emissary.sixth_sense.uses_per_day"
                .to_owned(),
            value: uses,
            detail: format!(
                "Rivethun Emissary level {level} Sixth Sense: {uses} times per day (corpus \
                 `SPELLS:Innate|TIMES=RivethunEmissaryLVL|...`)"
            ),
        });
    }
    if let Some(value) = rivethun_emissary_spirit_animal_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.rivethun_emissary.spirit_animal.level"
                .to_owned(),
            value,
            detail: format!(
                "Rivethun Emissary level {level} Spirit Animal: `ShamanSpiritLVL` = \
                 `FamiliarMasterLVL` = {value} (corpus `ShamanSpiritLVL|RivethunEmissaryLVL`)"
            ),
        });
    }
    if let Some(dc) = rivethun_emissary_spirit_bond_hex_dc(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.rivethun_emissary.spirit_bond.hex_dc"
                .to_owned(),
            value: dc,
            detail: format!(
                "Rivethun Emissary level {level} Spirit Bond hex DC bonus {dc} (corpus \
                 `ShamanHexDC|RivethunEmissaryLVL/2`)"
            ),
        });
    }
    if let Some(value) = rivethun_emissary_spirit_bond_hex_ability_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.rivethun_emissary.spirit_bond.hex_ability_\
                 level"
                .to_owned(),
            value,
            detail: format!(
                "Rivethun Emissary level {level} Spirit Bond: `ShamanHexAbilityLVL` = {value} \
                 (corpus `ShamanHexAbilityLVL|RivethunEmissaryLVL`)"
            ),
        });
    }
}

// ---- Student of War (adventurers_guide) ----

pub(super) fn student_of_war_additional_skill_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 1) / 2)
}

/// `ag_abilities_class.lst`, `KEY:Student of War ~ Mind Over Metal`:
/// `BONUS:COMBAT|AC|INT-DEX|...|PREMULT:2,[no armor/shield],[INT>DEX]`.
/// Grounds the magnitude only when Intelligence exceeds Dexterity (the
/// condition the corpus record itself gates on); this engine does not
/// track whether armor or a shield is currently equipped.
pub(super) fn student_of_war_mind_over_metal_ac_bonus(
    intelligence_modifier: i16,
    dexterity_modifier: i16,
) -> Option<i16> {
    if intelligence_modifier <= dexterity_modifier {
        return None;
    }
    Some(intelligence_modifier - dexterity_modifier)
}

pub(super) fn ground_student_of_war_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == STUDENT_OF_WAR_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(pool) = student_of_war_additional_skill_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.student_of_war.additional_skill.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Student of War Skill|(SoWLvl+1)/2`
                "Student of War level {level} Additional Skill: a pool of {pool}"
            ),
        });
    }
    if let Some(bonus) = student_of_war_mind_over_metal_ac_bonus(
        ability_modifiers.intelligence,
        ability_modifiers.dexterity,
    ) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.student_of_war.mind_over_metal.ac_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:COMBAT|AC|INT-DEX|...`, this character's Intelligence modifier exceeds
                //   Dexterity
                "Student of War level {level} Mind Over Metal: +{bonus} AC. Grounds the magnitude \
                 only; this engine does not track whether armor or a shield is equipped, which the \
                 corpus record itself also requires"
            ),
        });
    }
}

// ---- Diabolist (book_of_the_damned_volume_1) ----

pub(super) fn diabolist_channel_hellfire_times(level: u8, charisma_modifier: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(charisma_modifier.max(1))
}

pub(super) fn diabolist_infernal_transport_times(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2)
}

/// `botd1_classes.lst`'s own level-1 row: `BONUS:VAR|DiabolistDamnedLVL|
/// CL` -- this class's own raw level, the same "bare `CL` on a class's own
/// table row means that class's own level, not total character level"
/// idiom already confirmed for `PaDLVL|CL`/`TwilightTalonLVL|CL`/
/// `GoldenLegionnaireLVL|CL`, all already shipped in this file treating it
/// as the raw class level.
pub(super) fn diabolist_damned_dc(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 + i16::from(level))
}

/// Same level-1 row: `BONUS:VAR|DiabolistInfernalCharismaLVL|CL`.
pub(super) fn diabolist_infernal_charisma_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 * ((i16::from(level) + 2) / 3))
}

/// `botd1_classes.lst`'s own level-3 row: `BONUS:VAR|DiabolistHeresyLVL|
/// CL`.
pub(super) fn diabolist_heresy_bonus(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(2 * ((i16::from(level) + 3) / 6))
}

/// `botd1_classes.lst`'s own level-8 row: `BONUS:VAR|
/// DiabolistHellfireRayLVL|CL`.
pub(super) fn diabolist_hellfire_ray_caster_level(level: u8) -> Option<i16> {
    if level < 8 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn ground_diabolist_class_features(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == DIABOLIST_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };
    let charisma = ability_modifiers.charisma;

    if let Some(times) = diabolist_channel_hellfire_times(level, charisma) {
        explanations.push(ComputationExplanation {
            id: "class_feature.book_of_the_damned_volume_1.diabolist.channel_hellfire.times_\
                 per_day"
                .to_owned(),
            value: times,
            detail: format!(
                "Diabolist level {level} Channel Hellfire: {times} times per day (corpus \
                 `DiabolistChannelHellfireTimes|max(1,CHA)`)"
            ),
        });
    }
    if let Some(times) = diabolist_infernal_transport_times(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.book_of_the_damned_volume_1.diabolist.infernal_transport.times_\
                 per_day"
                .to_owned(),
            value: times,
            detail: format!(
                "Diabolist level {level} Infernal Transport: {times} times per day (corpus \
                 `DiabolistInfernalTransportTimes|2`)"
            ),
        });
    }
    if let Some(dc) = diabolist_damned_dc(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.book_of_the_damned_volume_1.diabolist.damned.dc".to_owned(),
            value: dc,
            detail: format!(
                "Diabolist level {level} Damned, DC {dc} (corpus `DiabolistDamnedDC|\
                 10+DiabolistDamnedLVL`, `DiabolistDamnedLVL` = this class's own level \
                 {level})"
            ),
        });
    }
    if let Some(bonus) = diabolist_infernal_charisma_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.book_of_the_damned_volume_1.diabolist.infernal_charisma.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Diabolist level {level} Infernal Charisma: +{bonus} Charisma-based check \
                 bonus (corpus `DiabolistInfernalCharismaBonus|\
                 floor((DiabolistInfernalCharismaLVL+2)/3)*2`)"
            ),
        });
    }
    if let Some(bonus) = diabolist_heresy_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.book_of_the_damned_volume_1.diabolist.heresy.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Diabolist level {level} Heresy: +{bonus} caster level check bonus (corpus \
                 `DiabolistHeresyBonus|floor((DiabolistHeresyLVL+3)/6)*2`)"
            ),
        });
    }
    if let Some(caster_level) = diabolist_hellfire_ray_caster_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.book_of_the_damned_volume_1.diabolist.hellfire_ray.caster_level"
                .to_owned(),
            value: caster_level,
            detail: format!(
                "Diabolist level {level} Hellfire Ray: 2 times per day, caster level \
                 {caster_level} (corpus `SPELLS:Class|TIMES=2|CASTERLEVEL=\
                 DiabolistHellFireRayLVL|Hellfire Ray,16+CHA`)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.book_of_the_damned_volume_1.diabolist.hellfire_ray.dc".to_owned(),
            value: 16 + charisma,
            detail: format!(
                "Diabolist level {level} Hellfire Ray, DC {} (corpus `Hellfire Ray,16+CHA`)",
                16 + charisma
            ),
        });
    }
}

// ---- Lion Blade (inner_sea_intrigue) ----

pub(super) fn lion_blade_expeditious_advance_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10)
}

pub(super) fn lion_blade_silent_soul_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10)
}

pub(super) fn lion_blade_sneak_attack_dice(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level >= 2) + i16::from(level >= 6) + i16::from(level >= 10))
}

pub(super) fn ground_lion_blade_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == LION_BLADE_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = lion_blade_expeditious_advance_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_intrigue.lion_blade.expeditious_advance.speed_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:MOVEADD|TYPE=Walk|10|...`
                "Lion Blade level {level} Expeditious Advance: +{bonus} ft. movement speed, while \
                 lightly loaded and not wearing medium or heavy armor -- this engine does not track \
                 encumbrance or armor weight, so the gate is not itself modelled"
            ),
        });
    }
    if let Some(bonus) = lion_blade_silent_soul_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_intrigue.lion_blade.silent_soul.stealth_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SKILL|Stealth|10|TYPE=Circumstance`
                "Lion Blade level {level} Silent Soul: +{bonus} Stealth"
            ),
        });
    }
    if let Some(dice) = lion_blade_sneak_attack_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_intrigue.lion_blade.sneak_attack.dice".to_owned(),
            value: dice,
            detail: format!(
                "Lion Blade level {level} Sneak Attack: +{dice}d6 (corpus `SneakAttackDice|1` \
                 at levels 2, 6, 10)"
            ),
        });
    }
}

// ---- Bellflower Tiller (adventurers_guide) ----

pub(super) fn bellflower_tiller_bellflower_crop_range(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(30 + 30 * i16::from(level >= 7))
}

pub(super) fn bellflower_tiller_crop_guardian_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn bellflower_tiller_sneak_attack_dice(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 3)
}

pub(super) fn bellflower_tiller_swift_sower_speed(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(10 + 10 * i16::from(level >= 6))
}

pub(super) fn bellflower_tiller_teamwork_feat_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level >= 6) + i16::from(level >= 10))
}

pub(super) fn ground_bellflower_tiller_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == BELLFLOWER_TILLER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(range) = bellflower_tiller_bellflower_crop_range(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.bellflower_tiller.bellflower_crop.range"
                .to_owned(),
            value: range,
            detail: format!(
                "Bellflower Tiller level {level} Bellflower Crop: range {range} ft. (corpus \
                 `BellflowerCropRange|30` plus 30 more at level 7)"
            ),
        });
    }
    if let Some(bonus) = bellflower_tiller_crop_guardian_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.bellflower_tiller.crop_guardian.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Bellflower Tiller level {level} Crop Guardian: `CropVigilanceBonusLVL` = \
                 {bonus} (corpus `CropVigilanceBonusLVL|BellflowerTillerLVL`)"
            ),
        });
    }
    if let Some(dice) = bellflower_tiller_sneak_attack_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.bellflower_tiller.sneak_attack.dice".to_owned(),
            value: dice,
            detail: format!(
                "Bellflower Tiller level {level} Sneak Attack: +{dice}d6 (corpus \
                 `SneakAttackDice|BellflowerTillerLVL/3`)"
            ),
        });
    }
    if let Some(speed) = bellflower_tiller_swift_sower_speed(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.bellflower_tiller.swift_sower.speed_bonus"
                .to_owned(),
            value: speed,
            detail: format!(
                "Bellflower Tiller level {level} Swift Sower: +{speed} ft. movement (corpus \
                 `SwiftSowerSpeed|10` plus 10 more at level 6)"
            ),
        });
    }
    if let Some(pool) = bellflower_tiller_teamwork_feat_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.bellflower_tiller.teamwork_feat.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Teamwork
                //   Feat|1+(BellflowerTillerLVL>=6)+(BellflowerTillerLVL>=10)`
                "Bellflower Tiller level {level} Teamwork Feat: a pool of {pool}"
            ),
        });
    }
}

// ---- Hellknight Signifer (adventurers_guide) ----

pub(super) fn hellknight_signifer_assiduous_gaze_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level >= 7))
}

pub(super) fn hellknight_signifer_signifer_mask_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2)
}

pub(super) fn hellknight_signifer_infernal_resilience_dr(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(5)
}

pub(super) fn ground_hellknight_signifer_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == HELLKNIGHT_SIGNIFER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(pool) = hellknight_signifer_assiduous_gaze_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.hellknight_signifer.assiduous_gaze.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Assiduous Gaze Ability|1+(HellknightSigniferLVL>=7)`
                "Hellknight Signifer level {level} Assiduous Gaze: a pool of {pool}"
            ),
        });
    }
    if let Some(bonus) = hellknight_signifer_signifer_mask_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.hellknight_signifer.signifer_mask.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SKILL|Sense Motive|2|TYPE=Competence|PREEQUIP:1,Signifer Mask%`
                "Hellknight Signifer level {level} Signifer Mask: +{bonus} Sense Motive while \
                 wearing the mask. Grounds the magnitude only; this engine does not track whether \
                 the mask is equipped"
            ),
        });
    }
    if let Some(dr) = hellknight_signifer_infernal_resilience_dr(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.hellknight_signifer.infernal_resilience.dr"
                .to_owned(),
            value: dr,
            detail: format!(
                "Hellknight Signifer level {level} Infernal Resilience: DR {dr}/chaotic \
                 (corpus `DR:5/Chaotic`)"
            ),
        });
    }
}

// ---- Mystic Archer (ultimate_psionics) ----

/// `up_classes.lst`'s own level-3 row grants Heightened Senses;
/// `up_abilities_class.lst`'s own token: `BONUS:VAR|HeightenedSensesRange|
/// 5*(MysticArcherLVL-2)`.
pub(super) fn mystic_archer_heightened_senses_range(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(5 * (i16::from(level) - 2))
}

/// Granted at level 2 (`up_classes.lst`'s own level-2 row). `30 +
/// HeightenedSensesRange` once Heightened Senses (level 3+) applies.
pub(super) fn mystic_archer_blindsense_range(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(30 + mystic_archer_heightened_senses_range(level).unwrap_or(0))
}

/// Granted at level 6.
pub(super) fn mystic_archer_blindsight_range(level: u8) -> Option<i16> {
    if level < 6 {
        return None;
    }
    Some(30 + mystic_archer_heightened_senses_range(level).unwrap_or(0))
}

/// Granted at level 4.
pub(super) fn mystic_archer_tremorsense_range(level: u8) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(30 + mystic_archer_heightened_senses_range(level).unwrap_or(0))
}

/// Granted at level 1: `BONUS:VAR|InevitableStrikeTimes|
/// (MysticArcherLVL+1)/2`.
pub(super) fn mystic_archer_inevitable_strike_uses(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 1) / 2)
}

/// Granted at level 2: `BONUS:VAR|RangedSneakAttack|
/// floor((MysticArcherLVL+1)/3)`.
pub(super) fn mystic_archer_ranged_sneak_attack_dice(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some((i16::from(level) + 1) / 3)
}

/// Granted at level 7: `SPELLS:Innate|TIMES=1|...|Pierce the Veils`.
pub(super) fn mystic_archer_unhindered_vision_uses(level: u8) -> Option<i16> {
    if level < 7 {
        return None;
    }
    Some(1)
}

pub(super) fn ground_mystic_archer_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == MYSTIC_ARCHER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(range) = mystic_archer_heightened_senses_range(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.mystic_archer.heightened_senses.range"
                .to_owned(),
            value: range,
            detail: format!(
                "Mystic Archer level {level} Heightened Senses: {range} ft. (corpus \
                 `HeightenedSensesRange|5*(MysticArcherLVL-2)`)"
            ),
        });
    }
    if let Some(range) = mystic_archer_blindsense_range(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.mystic_archer.blindsense.range".to_owned(),
            value: range,
            detail: format!(
                "Mystic Archer level {level} Blindsense: {range} ft. (corpus \
                 `BlindsenseRange|30`, plus `HeightenedSensesRange` once granted)"
            ),
        });
    }
    if let Some(range) = mystic_archer_blindsight_range(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.mystic_archer.blindsight.range".to_owned(),
            value: range,
            detail: format!(
                "Mystic Archer level {level} Blindsight: {range} ft. (corpus \
                 `BlindsightRange|30`, plus `HeightenedSensesRange`)"
            ),
        });
    }
    if let Some(range) = mystic_archer_tremorsense_range(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.mystic_archer.tremorsense.range".to_owned(),
            value: range,
            detail: format!(
                "Mystic Archer level {level} Tremorsense: {range} ft. (corpus \
                 `TremorsenseRange|30`, plus `HeightenedSensesRange`)"
            ),
        });
    }
    if let Some(uses) = mystic_archer_inevitable_strike_uses(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.mystic_archer.inevitable_strike.uses_per_day"
                .to_owned(),
            value: uses,
            detail: format!(
                "Mystic Archer level {level} Inevitable Strike: {uses} times per day (corpus \
                 `InevitableStrikeTimes|(MysticArcherLVL+1)/2`)"
            ),
        });
    }
    if let Some(dice) = mystic_archer_ranged_sneak_attack_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.mystic_archer.ranged_sneak_attack.dice"
                .to_owned(),
            value: dice,
            detail: format!(
                "Mystic Archer level {level} Ranged Sneak Attack: +{dice}d6 within 30 ft. \
                 (corpus `RangedSneakAttack|floor((MysticArcherLVL+1)/3)`)"
            ),
        });
    }
    if let Some(uses) = mystic_archer_unhindered_vision_uses(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.mystic_archer.unhindered_vision.uses_per_day"
                .to_owned(),
            value: uses,
            detail: format!(
                "Mystic Archer level {level} Unhindered Vision: {uses} time per 10 minutes \
                 (corpus `SPELLS:Innate|TIMES=1|...|Pierce the Veils`)"
            ),
        });
    }
}

// ---- Mammoth Rider (adventurers_guide) ----

pub(super) fn mammoth_rider_born_survivor_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 2) / 4)
}

pub(super) fn mammoth_rider_flag(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1)
}

pub(super) fn mammoth_rider_steed_companion_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn ground_mammoth_rider_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == MAMMOTH_RIDER_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(pool) = mammoth_rider_born_survivor_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.mammoth_rider.born_survivor.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Mammoth Rider Born Survivor Feat|(MammothRiderLvl+2)/4`
                "Mammoth Rider level {level} Born Survivor: a pool of {pool}"
            ),
        });
    }
    if let Some(flag) = mammoth_rider_flag(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.mammoth_rider.gigantic_steed.qualify_flag"
                .to_owned(),
            value: flag,
            detail: format!(
                "Mammoth Rider level {level} Gigantic Steed: a qualifying flag (corpus \
                 `GrantGiganticSteed|1`)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.mammoth_rider.rugged_steed.qualify_flag"
                .to_owned(),
            value: flag,
            detail: format!(
                "Mammoth Rider level {level} Rugged Steed: a qualifying flag (corpus \
                 `GrantRuggedSteed|1`)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.mammoth_rider.steeds_reach.qualify_flag"
                .to_owned(),
            value: flag,
            detail: format!(
                "Mammoth Rider level {level} Steed's Reach: a qualifying flag (corpus \
                 `GrantSteedsReach|1`)"
            ),
        });
    }
    if let Some(companion_level) = mammoth_rider_steed_companion_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.mammoth_rider.steed.companion_level".to_owned(),
            value: companion_level,
            detail: format!(
                "Mammoth Rider level {level} Steed: animal companion effective level \
                 {companion_level} (corpus `CompanionBondLVL|MammothRiderLVL`, restated onto \
                 `AnimalCompanionMasterLVL`)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.adventurers_guide.mammoth_rider.wild_coercion.level".to_owned(),
            value: companion_level,
            detail: format!(
                "Mammoth Rider level {level} Wild Coercion: `WildEmpathyLVL` = \
                 {companion_level} (corpus `WildEmpathyLVL|MammothRiderLVL`)"
            ),
        });
    }
}

// ---- Demoniac (book_of_the_damned_volume_2) ----

pub(super) fn demoniac_summon_demon_i_caster_level(level: u8, total_level: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(total_level - 1)
}

pub(super) fn demoniac_summon_demon_ii_caster_level(level: u8, total_level: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(total_level - 1)
}

pub(super) fn ground_demoniac_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == DEMONIAC_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };
    let total_level = total_character_level(input);

    if let Some(caster_level) = demoniac_summon_demon_i_caster_level(level, total_level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.book_of_the_damned_volume_2.demoniac.summon_demon_i.caster_level"
                .to_owned(),
            value: caster_level,
            detail: format!(
                "Demoniac level {level} Summon Demon I: once per day, caster level \
                 {caster_level} (corpus `SPELLS:Demoniac|TIMES=1|CASTERLEVEL=TL-1|Summon \
                 Monster VI (Demoniac)`)"
            ),
        });
    }
    if let Some(caster_level) = demoniac_summon_demon_ii_caster_level(level, total_level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.book_of_the_damned_volume_2.demoniac.summon_demon_ii.caster_\
                 level"
                .to_owned(),
            value: caster_level,
            detail: format!(
                "Demoniac level {level} Summon Demon II: once per day, caster level \
                 {caster_level} (corpus `SPELLS:Demoniac|TIMES=1|CASTERLEVEL=TL-1|Summon \
                 Monster VIII (Demoniac)`)"
            ),
        });
    }
}

// ---- Master Chymist (advanced_players_guide) ----

pub(super) fn master_chymist_advanced_mutagen_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

pub(super) fn master_chymist_bomb_thrower_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn master_chymist_brutality_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 + 2 * i16::from(level >= 7) + 2 * i16::from(level >= 9))
}

pub(super) fn master_chymist_extracts_per_day_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn master_chymist_mutate_times(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(2 + i16::from(level >= 5) + i16::from(level >= 8) + i16::from(level >= 10))
}

pub(super) fn ground_master_chymist_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == MASTER_CHYMIST_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(pool) = master_chymist_advanced_mutagen_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.master_chymist.advanced_mutagen.pool_\
                 size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Master Chymist Advanced
                //   Mutagen|MasterChymist_AdvancedMutagen_LVL/2`
                "Master Chymist level {level} Advanced Mutagen: a pool of {pool}"
            ),
        });
    }
    if let Some(value) = master_chymist_bomb_thrower_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.master_chymist.bomb_thrower.level"
                .to_owned(),
            value,
            detail: format!(
                "Master Chymist level {level} Bomb-Thrower: `AlchemistBombLVL` = {value} \
                 (corpus `AlchemistBombLVL|MasterChymistLVL`)"
            ),
        });
    }
    if let Some(bonus) = master_chymist_brutality_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.master_chymist.brutality.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Master Chymist level {level} Brutality: +{bonus} (corpus \
                 `MasterChymist_Brutality_Bonus|2`, stepping at levels 7 and 9)"
            ),
        });
    }
    if let Some(value) = master_chymist_extracts_per_day_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.master_chymist.extracts_per_day.level"
                .to_owned(),
            value,
            detail: format!(
                "Master Chymist level {level} Extracts per Day: `AlchemistAlchemyLVL` = \
                 {value} (corpus `AlchemistAlchemyLVL|MasterChymistLVL`)"
            ),
        });
    }
    if let Some(times) = master_chymist_mutate_times(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.master_chymist.mutate.times_per_day"
                .to_owned(),
            value: times,
            detail: format!(
                "Master Chymist level {level} Mutate: {times} times per day (corpus \
                 `MasterChymist_Mutate_Times|2`, stepping at levels 5, 8, 10)"
            ),
        });
    }
}

// ---- Enchanting Courtesan (inner_sea_intrigue) ----

pub(super) fn enchanting_courtesan_hidden_spell_count(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1 + i16::from(level >= 6) + i16::from(level >= 9))
}

pub(super) fn enchanting_courtesan_seductive_intuition_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level) / 2)
}

pub(super) fn ground_enchanting_courtesan_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ENCHANTING_COURTESAN_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(count) = enchanting_courtesan_hidden_spell_count(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_intrigue.enchanting_courtesan.hidden_spell.count"
                .to_owned(),
            value: count,
            detail: format!(
                "Enchanting Courtesan level {level} Hidden Spell: {count} times per day \
                 (corpus `EnchantingCourtesanHiddenSpell|1`, stepping at levels 6 and 9)"
            ),
        });
    }
    if let Some(bonus) = enchanting_courtesan_seductive_intuition_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_intrigue.enchanting_courtesan.seductive_intuition.\
                 bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SKILL|...|EnchantingCourtesanLVL/2`
                "Enchanting Courtesan level {level} Seductive Intuition: +{bonus} on \
                 Bluff/Diplomacy/Sense Motive/Sleight of Hand"
            ),
        });
    }
}

// ---- Dark Tempest (ultimate_psionics) ----

/// Granted at level 5 (`up_classes.lst`'s own level-5 row): `BONUS:
/// ABILITYPOOL|Dark Tempest Blade Skill|floor((DarkTempestLVL-2)/3)`.
pub(super) fn dark_tempest_blade_skills_pool_size(level: u8) -> Option<i16> {
    if level < 5 {
        return None;
    }
    Some((i16::from(level) - 2) / 3)
}

/// Granted at level 1: `BONUS:VAR|BladeSkillPrereqLVL,SoulknifeFeatPrereqLVL|
/// DarkTempestLVL`.
pub(super) fn dark_tempest_diverse_training_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// Granted at level 2: `ADD:ABILITY|...|Dark Tempest ~ Expanded Power
/// List`, `BONUS:ABILITYPOOL|Dark Tempest Expanded Power List Class|1`.
pub(super) fn dark_tempest_expanded_power_list_pool_size(level: u8) -> Option<i16> {
    if level < 2 {
        return None;
    }
    Some(1)
}

/// Granted at level 3: `BONUS:VAR|PowerStrikePowerLevel|
/// floor(DarkTempestLVL/3)`.
pub(super) fn dark_tempest_power_strike_power_level(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(i16::from(level) / 3)
}

/// Granted at level 1: `BONUS:VAR|PsychicStrikeDice|
/// floor((DarkTempestLVL+2)/3)` -- this class's own additive contribution
/// to the shared Soulknife Psychic Strike dice progression.
pub(super) fn dark_tempest_psychic_strike_dice(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 2) / 3)
}

pub(super) fn ground_dark_tempest_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == DARK_TEMPEST_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(pool) = dark_tempest_blade_skills_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.dark_tempest.blade_skills.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Dark Tempest Blade Skill|floor((DarkTempestLVL-2)/3)`
                "Dark Tempest level {level} Blade Skills: a pool of {pool}"
            ),
        });
    }
    if let Some(value) = dark_tempest_diverse_training_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.dark_tempest.diverse_training.level"
                .to_owned(),
            value,
            detail: format!(
                "Dark Tempest level {level} Diverse Training: `BladeSkillPrereqLVL` = \
                 `SoulknifeFeatPrereqLVL` = {value} (corpus \
                 `BladeSkillPrereqLVL,SoulknifeFeatPrereqLVL|DarkTempestLVL`)"
            ),
        });
    }
    if let Some(pool) = dark_tempest_expanded_power_list_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.dark_tempest.expanded_power_list.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Dark Tempest Expanded Power List Class|1`
                "Dark Tempest level {level} Expanded Power List: a pool of {pool}"
            ),
        });
    }
    if let Some(power_level) = dark_tempest_power_strike_power_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.dark_tempest.power_strike.power_level"
                .to_owned(),
            value: power_level,
            detail: format!(
                "Dark Tempest level {level} Power Strike: `PowerStrikePowerLevel` = \
                 {power_level} (corpus `PowerStrikePowerLevel|floor(DarkTempestLVL/3)`)"
            ),
        });
    }
    if let Some(dice) = dark_tempest_psychic_strike_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_psionics.dark_tempest.psychic_strike.dice".to_owned(),
            value: dice,
            detail: format!(
                "Dark Tempest level {level} Psychic Strike: +{dice}d6 (corpus \
                 `PsychicStrikeDice|floor((DarkTempestLVL+2)/3)`, this class's own additive \
                 contribution to the shared Soulknife Psychic Strike progression)"
            ),
        });
    }
}

// ---- Battle Herald (advanced_players_guide) ----

pub(super) fn battle_herald_inspiring_command_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn battle_herald_teamwork_feat_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1)
}

pub(super) fn ground_battle_herald_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == BATTLE_HERALD_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(value) = battle_herald_inspiring_command_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.battle_herald.inspiring_command.level"
                .to_owned(),
            value,
            detail: format!(
                "Battle Herald level {level} Inspiring Command: `InspiringCommandLVL` = \
                 {value} (corpus `InspiringCommandLVL|BattleHeraldLVL`)"
            ),
        });
    }
    if let Some(pool) = battle_herald_teamwork_feat_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.battle_herald.teamwork_feat.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Teamwork Feat|1`
                "Battle Herald level {level} Teamwork Feat: a pool of {pool}"
            ),
        });
    }
}

// ---- Master Spy (advanced_players_guide) ----

pub(super) fn master_spy_art_of_deception_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

pub(super) fn master_spy_slippery_mind_times(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1)
}

pub(super) fn master_spy_sneak_attack_dice(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(((i16::from(level) - 1) / 3) + 1)
}

pub(super) fn ground_master_spy_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == MASTER_SPY_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(bonus) = master_spy_art_of_deception_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.master_spy.art_of_deception.bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Master Spy level {level} Art of Deception: +{bonus} on Bluff/Disguise/Sense \
                 Motive (corpus `MasterSpyArtOfDeceptionBonus|MasterSpyLVL`)"
            ),
        });
    }
    if let Some(times) = master_spy_slippery_mind_times(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.master_spy.slippery_mind.times".to_owned(),
            value: times,
            detail: format!(
                "Master Spy level {level} Slippery Mind: {times} extra save re-roll (corpus \
                 `SlipperyMindTimes|1`)"
            ),
        });
    }
    if let Some(dice) = master_spy_sneak_attack_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.advanced_players_guide.master_spy.sneak_attack.dice".to_owned(),
            value: dice,
            detail: format!(
                "Master Spy level {level} Sneak Attack: +{dice}d6 (corpus \
                 `SneakAttackDice|((MasterSpyLVL-1)/3)+1`)"
            ),
        });
    }
}

// ---- Evangelist (ultimate_combat) ----

pub(super) fn evangelist_single_minded_domain_count_delta(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(-1)
}

pub(super) fn ground_evangelist_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == EVANGELIST_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(delta) = evangelist_single_minded_domain_count_delta(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.ultimate_combat.evangelist.single_minded.domain_count_delta"
                .to_owned(),
            value: delta,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|ClericDomainCount|-1`, the Evangelist gives up their second domain
                //   for full Bardic-Performance-style abilities
                "Evangelist level {level} Single-Minded: `ClericDomainCount` {delta}"
            ),
        });
    }
}

// ---- Ulfen Guard (inner_sea_combat) ----

pub(super) fn ulfen_guard_dedication_pool_size(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some((i16::from(level) + 1) / 2)
}

pub(super) fn ground_ulfen_guard_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ULFEN_GUARD_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(pool) = ulfen_guard_dedication_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_combat.ulfen_guard.guard_dedications.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|Pool_GuardDedication|(Ulfen_GuardLVL+1)/2`
                "Ulfen Guard level {level} Guard Dedications: a pool of {pool}"
            ),
        });
    }
}

/// The storage-only namespace prefix `pf1_adapter.rs`'s Path A seed must use
/// to satisfy `validate_character_input`'s colon requirement. Stripped here,
/// never re-added anywhere else -- the engine-facing value is always the
/// bare corpus slug (`CORE_RULEBOOK_RAGE_POWER_POOL`'s own shape).
pub(super) const RAGE_POWER_SEED_NAMESPACE: &str = "rage_power:";

pub(super) fn rage_power_selection_denamespaced(raw: &str) -> &str {
    raw.strip_prefix(RAGE_POWER_SEED_NAMESPACE).unwrap_or(raw)
}

/// The four "Extra <resource>" General feats (task #19, 2026-07-27).
/// Each is prerequisite-gated on already having the base resource
/// (`PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage` and friends) and
/// adds a flat amount to a per-day total this engine already computes --
/// no new mechanism, no new choice gate.
///
/// Magnitudes verified twice and in agreement: this repo's own
/// `feat_data/general.rs` catalog entries and the raw PCGen
/// `cr_feats.lst` records (`BONUS:VAR|RageDuration|6`,
/// `BONUS:VAR|BardicPerformanceDuration|6`, `BONUS:VAR|KiPoints|2`,
/// `BONUS:VAR|LayOnHandsTimes|2`).
pub(super) const EXTRA_RAGE_FEAT_KEY: &str = "Extra Rage";

pub(super) const EXTRA_PERFORMANCE_FEAT_KEY: &str = "Extra Performance";

pub(super) const EXTRA_KI_FEAT_KEY: &str = "Extra Ki";

pub(super) const EXTRA_LAY_ON_HANDS_FEAT_KEY: &str = "Extra Lay On Hands";

/// Extra Rage and Extra Performance both grant 6 additional rounds/day.
pub(super) const EXTRA_ROUNDS_PER_DAY: i16 = 6;

/// Extra Ki grants 2 ki points; Extra Lay On Hands grants 2 uses/day.
pub(super) const EXTRA_POINTS_PER_DAY: i16 = 2;

/// The five APG/ACG "Extra <resource>" feats whose pool this engine
/// already computes (v0.6 alpha swarm, `BONUS:VAR`/`BONUS:DC`/
/// `BONUS:ABILITYPOOL` triage slice, 2026-07-29), extending the four
/// Core feats above rather than duplicating them.
///
/// Every magnitude is the raw `BONUS:VAR` token and the record's own
/// `BENEFIT:` prose **in agreement**, checked one record at a time --
/// the disagreement that deferred Extra Channel (`ABILITYPOOL|1` vs
/// "two additional times per day") does not occur in any of these:
/// - `Extra Bombs` (`apg_feats.lst`): `BONUS:VAR|AlchemistBombTimes|2`,
///   "You can throw two additional bombs per day."
/// - `Extra Inspiration` (`acg_feats.lst`):
///   `BONUS:VAR|InvestigatorInspirationPoolBonus|3`, "three extra
///   use[s] per day of inspiration". Its second token,
///   `BONUS:VAR|InspirationTimes|3`, is deliberately NOT grounded:
///   `InspirationTimes` is a different pool, the one `Amateur
///   Investigator` conjures for a character with no Investigator level
///   (`DEFINE:InspirationTimes|0` plus `BONUS:VAR|InspirationTimes|INT`
///   on that feat's own record). This engine computes no total for it,
///   and grounding it would mean inventing one.
/// - `Extra Martial Flexibility` (`acg_feats.lst`):
///   `BONUS:VAR|BrawlerMartialFlexibilityTimes|3`, "three additional
///   times per day."
/// - `Extra Panache` (`acg_feats.lst`): `BONUS:VAR|Panache_Cap|2`,
///   "your maximum panache increases by two."
/// - `Extra Reservoir` (`acg_feats.lst`):
///   `BONUS:VAR|MaxArcanistReservoirSize|3`, "the maximum number of
///   points in your arcane reservoir increases by that amount."
pub(super) const EXTRA_BOMBS_FEAT_KEY: &str = "Extra Bombs";

pub(super) const EXTRA_INSPIRATION_FEAT_KEY: &str = "Extra Inspiration";

pub(super) const EXTRA_MARTIAL_FLEXIBILITY_FEAT_KEY: &str = "Extra Martial Flexibility";

pub(super) const EXTRA_PANACHE_FEAT_KEY: &str = "Extra Panache";

pub(super) const EXTRA_RESERVOIR_FEAT_KEY: &str = "Extra Reservoir";

/// Extra Bombs and Extra Panache each grant 2; Extra Inspiration,
/// Extra Martial Flexibility and Extra Reservoir each grant 3.
pub(super) const EXTRA_BOMBS_USES: i16 = 2;

pub(super) const EXTRA_PANACHE_POINTS: i16 = 2;

pub(super) const EXTRA_INSPIRATION_USES: i16 = 3;

pub(super) const EXTRA_MARTIAL_FLEXIBILITY_USES: i16 = 3;

pub(super) const EXTRA_RESERVOIR_POINTS: i16 = 3;

/// `Improved Channel` (`cr_feats.lst`), the one `BONUS:VAR` feat in
/// this slice that lands on a DC rather than a resource pool. See
/// [`oracle_channel_dc`] for why Oracle's Life Mystery Channel is the
/// only total in this engine it can reach.
pub(super) const IMPROVED_CHANNEL_FEAT_KEY: &str = "Improved Channel";

pub(super) const IMPROVED_CHANNEL_DC_BONUS: i16 = 2;

/// `Extended Animal Focus` (`acg_feats.lst`). Its magnitude is not a
/// constant -- it is `max(1, WIS)` -- so only the key lives here; see
/// [`hunter_animal_focus_uses_per_day`].
pub(super) const EXTENDED_ANIMAL_FOCUS_FEAT_KEY: &str = "Extended Animal Focus";

/// `bonus` if `feat_key` is held **at all**, ignoring duplicates.
///
/// The deliberate counterpart to [`extra_resource_feat_bonus`], which
/// COUNTS. `Extra Martial Flexibility` is the single Extra-<resource>
/// record in all three books carrying no `STACK:`, no `MULT:` and no
/// `CHOOSE:` token, and whose `BENEFIT:` has no "Special: you can take
/// this feat multiple times" clause -- it is genuinely not repeatable.
/// Every one of the other eight is `STACK:YES MULT:YES` with an
/// explicit repeat clause. Routing this one through the counting helper
/// would hand a Brawler +9 uses per day for a build the rules forbid,
/// which is exactly the `STACK:`/`MULT:`-invisible-to-a-filtered-grep
/// failure that already cost this family one correction pass.
pub(super) fn non_stacking_resource_feat_bonus(selected_feats: &[String], feat_key: &str, bonus: i16) -> i16 {
    if feat_identity::holds(selected_feats, feat_key) { bonus } else { 0 }
}

/// Surface direct SD13-E3/E5 runtime evidence for the deterministic Human Rogue
/// level-1/level-2/level-3/level-4/level-5/level-6/level-7/level-8 chassis, mirroring the
/// Barbarian/Monk level-1 baseline pattern and the Fighter
/// `supported_fighter_level` / Paladin `supported_paladin_level`
/// level-range-gate idiom.
/// The SD13-E3 pillar-grounding slice grounds three of the four named
/// burdens directly (base-attack progression, base-save progression, and
/// sneak attack die count); the SD13-E5 trapfinding slice grounds the
/// fourth, Trapfinding, mirroring the grounded Ranger Track record, so no
/// named Rogue pillar burden remains claim-blocked; a later SD13-E5 slice
/// widens the level-1-only gate to level 2 (the PF1 Core Rulebook Rogue
/// class table's next milestone) and grounds Evasion as a bounded
/// identity/recognition record; a further SD13-E5 slice widens the gate to
/// level 3 and grounds Trap Sense (the class table's 3rd-level "Special"
/// entry) as a bounded flat-magnitude record; a further SD13-E5 slice widens
/// the gate to level 4 and grounds Uncanny Dodge (the class table's
/// 4th-level "Special" entry, verified independently against d20pfsrd and
/// legacy.aonprd.com — NOT the same level as Barbarian's own 2nd-level
/// Uncanny Dodge) as a bounded identity/recognition record; a further
/// SD13-E5 slice widens the gate to level 5 (verified independently against
/// d20pfsrd and legacy.aonprd.com: the class table's level-5 "Special"
/// column reads only "Sneak attack +3d6," no other new feature) and the
/// pre-existing sneak-attack die-count formula genuinely produces `3` (i.e.
/// `3d6`) at level 5, with Evasion, Trap Sense, and Uncanny Dodge all
/// staying granted, not re-derived; a further SD13-E5 slice widens the gate
/// to level 6 (verified independently against d20pfsrd and
/// legacy.aonprd.com: the class table's level-6 "Special" column reads
/// "Rogue talent, trap sense +2") — the pre-existing Trap Sense
/// flat-magnitude formula (`level / 3`, floor) genuinely rises to `2` at
/// level 6, the pre-existing sneak-attack die-count formula stays at `3`
/// (i.e. `3d6`, unchanged from level 5), and Trapfinding genuinely rises to
/// `3` via the same pre-existing formula, with Evasion and Uncanny Dodge
/// staying granted, not re-derived; the level-6 row's OTHER named entry, a
/// second Rogue Talent slot, is deliberately left named-but-unproven this
/// slice, mirroring the level-2/level-4 rogue-talent precedent; a further
/// SD13-E5 slice widens the gate to level 7 (verified independently against
/// d20pfsrd and legacy.aonprd.com: the class table's level-7 "Special"
/// column reads only "Sneak attack +4d6," no other new feature) — the
/// pre-existing sneak-attack die-count formula (`(level + 1) / 2`) genuinely
/// rises to `4` (i.e. `4d6`) at level 7, up from `3` at level 6, via the
/// same formula, not a new record; the pre-existing Trap Sense
/// flat-magnitude formula stays at `2` (unchanged from level 6, the next
/// rise is at 9th level); Trapfinding stays at `3` (`max(7 / 2, 1)`, an
/// integer-division coincidence with level 6); Evasion and Uncanny Dodge
/// both stay granted, not re-derived. A further SD13-E5 slice widens the
/// gate to level 8 (verified independently against d20pfsrd and
/// legacy.aonprd.com: the class table's level-8 "Special" column reads
/// "Improved uncanny dodge, rogue talent") — the pre-existing sneak-attack
/// die-count formula (`(level + 1) / 2`) stays at `4` (i.e. `4d6`, unchanged
/// from level 7, since the die count only rises at odd rogue levels); the
/// pre-existing Trap Sense flat-magnitude formula stays at `2` (unchanged
/// from level 7, the next rise is at 9th level); Trapfinding genuinely rises
/// to `4` (`max(8 / 2, 1)`, up from `3` at level 7, via the same formula);
/// Evasion and Uncanny Dodge both stay granted, not re-derived; Improved
/// Uncanny Dodge is newly granted and grounded as a bounded
/// identity/recognition record only, mirroring exactly how Barbarian's own
/// Improved Uncanny Dodge was grounded at barbarian level 5. The level-8
/// row's OTHER named entry, a third Rogue Talent slot, is deliberately left
/// named-but-unproven this slice, mirroring the level-2/level-4/level-6
/// rogue-talent precedent.
///
/// This deliberately does not compute a full Rogue class engine. It grounds,
/// at every supported level (1, 2, 3, 4, 5, 6, 7, and 8):
/// - base-attack progression (3/4 BAB, `level * 3 / 4`),
/// - base-save progression (good Reflex, poor Fortitude, poor Will),
/// - the sneak attack damage-die *count* only (`(level + 1) / 2`, i.e. `1`
///   at levels 1-2, `2` at levels 3-4, `3` at levels 5-6, and `4` at levels
///   7-8, `1d6`/`2d6`/`3d6`/`4d6`) — not damage-roll execution and not the
///   flanking / Dexterity-denial trigger-condition engine,
/// - the Trapfinding flat numeric bonus (`max(level / 2, 1)`, `+1` at levels
///   1-3, `+2` at levels 4-5, `+3` at levels 6-7, and `+4` at level 8) on
///   Perception checks to locate traps and on Disable Device checks, plus
///   the magic-trap-disarm statement — not a check-execution engine, no
///   trap DC resolution, and no magic-trap disarm engine,
/// - Evasion (a 2nd-level Rogue class feature): below level 2 it is grounded
///   as a correct PF1 Core Rulebook level-gate absence (value 0); at level 2
///   and above it is grounded as a bounded identity/recognition record only
///   (value 0, non-fabricated) naming the rule text (no damage on a
///   successful Reflex save against an effect that normally allows half
///   damage on a successful save; no benefit on a failed save) — mirroring
///   how Divine Grace and Bravery were grounded as flat rules-text records
///   without folding into an actual saving-throw-resolution or
///   damage-resolution engine, neither of which exists in this codebase,
/// - Trap Sense (a 3rd-level Rogue class feature, verified independently
///   against d20pfsrd and legacy.aonprd.com): below level 3 it is grounded as
///   a correct PF1 Core Rulebook level-gate absence (value 0); at level 3 and
///   above it is grounded as a bounded flat-magnitude record only
///   (`level / 3`, floor; `+1` at levels 3-5, genuinely rising to `+2` at
///   level 6) naming the rule text (a bonus on Reflex saves made to avoid
///   traps and an equal dodge bonus to AC against attacks made by traps) —
///   mirroring the Fighter Bravery / Paladin Divine Grace idiom: the
///   magnitude is never applied to any actual Reflex-save total or AC total,
///   since no saving-throw-resolution or armor-class-resolution engine
///   exists in this codebase, and no trap-detection or trap-triggering
///   engine exists to decide when it would apply, and
/// - Uncanny Dodge (a 4th-level Rogue class feature, verified independently
///   against d20pfsrd and legacy.aonprd.com): below level 4 it is grounded as
///   a correct PF1 Core Rulebook level-gate absence (value 0); at level 4 and
///   above it is grounded as a bounded identity/recognition record only
///   (value 0, non-fabricated) naming the rule text (cannot be caught
///   flat-footed; retains Dexterity bonus to AC even against an invisible
///   attacker; still loses it if immobilized) — mirroring exactly how
///   Barbarian's own Uncanny Dodge was grounded, without folding into any
///   actual flat-footed-state tracking, Armor Class computation, or
///   invisibility-detection engine, none of which exists in this codebase.
///   The level-4 row's OTHER named entry, a Rogue Talent (an open-ended
///   choice-list feature), is deliberately left named-but-unproven this
///   slice, mirroring the Monk level-2 bonus feat / Barbarian Rage Power
///   precedent, and
/// - Improved Uncanny Dodge (an 8th-level Rogue class feature, verified
///   independently against d20pfsrd and legacy.aonprd.com): below level 8 it
///   is grounded as a correct PF1 Core Rulebook level-gate absence (value 0);
///   at level 8 and above it is grounded as a bounded identity/recognition
///   record only (value 0, non-fabricated) naming the rule text (can no
///   longer be flanked; denies another rogue the ability to sneak attack by
///   flanking unless the attacker has at least four more rogue levels) —
///   mirroring exactly how Barbarian's own Improved Uncanny Dodge was
///   grounded, without folding into any actual flanking-resolution or
///   attacker-level-comparison engine, neither of which exists in this
///   codebase. The level-8 row's OTHER named entry, a third Rogue Talent
///   slot, is deliberately left named-but-unproven this slice, mirroring the
///   level-2/level-4/level-6 rogue-talent precedent.
///
/// It still grounds no rogue talent (a level-2+/level-4+/level-6+/level-8+
/// choice-list feature, and a genuinely open-ended talent tree — a
/// new-subsystem-shaped burden left named but unproven) and no level-9+
/// progression. These
/// `class_chassis.rogue.*` / `class_feature.rogue.evasion` /
/// `class_feature.rogue.trap_sense` / `class_feature.rogue.uncanny_dodge` /
/// `class_feature.rogue.improved_uncanny_dodge`
/// explanation records are standalone: they are not wired into
/// `compute_fighter_chassis`, `compute_total_saves`, or
/// `compute_combat_baseline`, so `defense.total_save.*` is still never
/// computed for Rogue here. It only:
/// - leaves one chassis-recognition explanation so the `class:rogue:N`
///   identity is acknowledged rather than an undocumented packet placeholder
///   (direct runtime evidence, carrying no fabricated mechanical value), and
/// - leaves nine grounded pillar explanations (base-attack, base-save
///   fortitude/reflex/will, sneak-attack die count, trapfinding, Evasion,
///   Trap Sense, Uncanny Dodge, Improved Uncanny Dodge).
///
/// The named Rogue claim-blocking diagnostic set is now empty; the four
/// generic chassis diagnostics (`class_chassis.unsupported`,
/// `combat.baseline_unsupported`, `defense.total_save.unsupported`,
/// `skill.selected_modifier.unsupported`) still claim-block this input
/// (including `tests/ge06_pilot_total_saves.rs::unsupported_chassis_blocks_total_saves`,
/// which keeps passing unmodified since no `defense.total_save.*` explanation
/// is ever computed here); this seam keeps that blocked posture but makes the
/// Rogue chassis identity and its grounded pillars legible on the runtime
/// path.
/// SD-32 Epic 1 (compute-library wiring, `epic-breakdown.md` F3): resolves one
/// `BONUS:VAR|<target_var>|<formula>` value straight off a real corpus `class_feature`
/// record's own token chain, through the SAME already-authorised, already-fixture-checked
/// mechanism `class_feature_grant_consumer::resolve_pcgen_var_chain` already proves
/// correct at `tests/fixtures/rules_core/derived-evaluator-fixtures.json`'s
/// `class_feature_description_entries` (operator ruling §20, `decisions.md §3`: "every
/// interpreted value clears `derived_evaluator_fixture_check`" -- this call site reuses
/// that already-cleared mechanism rather than adding a second, uncoordinated evaluator).
///
/// `None` when the chain does not fully resolve (record missing, formula shape refused,
/// or the target variable was never bound) -- never guessed, mirroring
/// `resolve_pcgen_var_chain`'s own "refuse rather than default" contract.
pub(super) fn resolve_class_feature_bonus_var(
    record_key: &str,
    class_level_var: &str,
    target_var: &str,
    level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Option<i16> {
    let record = class_feature_grant_consumer::class_feature_record_tokens().get(record_key)?;
    let vars = class_feature_grant_consumer::resolve_pcgen_var_chain(
        &record.bonus_vars,
        class_level_var,
        level,
        ability_modifiers,
    );
    vars.get(target_var).copied().and_then(|v| i16::try_from(v).ok())
}

/// SD-32 T12 Epic 8 (`epic-2-cause-closure`, row 18, "pool-shaped class
/// features"). `resolve_class_feature_bonus_var` above (SD-32 Epic 1) still
/// needs its caller to already know the record's OWN target variable name
/// (`"MasterHunterDC"`), which is fine for a handful of hand-picked named
/// grants but does not scale to a 1,913-group, ~5,927-numeric-magnitude
/// (cycle 3 `§17a` re-derivation, `scripts/census_class_feature_pool_population.py`
/// -- corrects cycle 2's own 5,981, which had not yet registered Witch
/// Hex's pre-existing `witch_hex_save_dc` coverage)
/// open-ended-choice-pool population (Alchemist Discovery, Witch Hex,
/// Oracle Curse/Mystery/Revelation, Shaman Spirit, Cavalier Order, Hunter
/// Animal Focus, Inquisitor Judgment, ...) one bespoke function per member
/// at a time.
///
/// This is the group-agnostic generalisation `decisions.md §17` requires:
/// given only a corpus `" ~ "`-qualified key, it finds the record's OWN
/// "terminal" `BONUS:VAR`/`DEFINE` target -- the one target name that is
/// never itself referenced inside another of the SAME record's formulas
/// (an intermediate hop like a `DomainXLVL`-shaped chain variable IS
/// referenced by its sibling and is therefore never picked as the
/// terminal) -- and resolves it through the SAME already-fixture-checked
/// `resolve_pcgen_var_chain` mechanism `resolve_class_feature_bonus_var`
/// uses, seeded with the real class level and the real six ability
/// modifiers. A record with zero or more than one terminal target refuses
/// (`None`) rather than guessing which one is "the" magnitude -- exactly
/// the four named shapes (flat, level-scaled, ability-modifier-only,
/// level+ability) resolve to a single terminal target; anything genuinely
/// novel (two independent magnitudes on one record, an unresolvable
/// `classlevel("OtherClass")`/dice-notation/multi-target formula) refuses
/// cleanly and is reported, never fabricated.
/// `pool_group` is used to look up the pool's own HEADER record
/// (`"<class> ~ <pool_group>"` -- e.g. `"Alchemist ~ Discovery"`,
/// `"Witch ~ Hex"`, real corpus keys distinct from any member's own
/// `"<pool_group> ~ <member>"` key), whose own `BONUS:VAR` chain very
/// often defines the pool-specific level variable individual members
/// scale on (`AlchemistDiscoveryLVL|AlchemistLVL`, confirmed live in
/// `advanced_players_guide/class_feature/alchemist/discovery.json`) --
/// without merging that header chain in, a member formula referencing
/// `AlchemistDiscoveryLVL` would refuse for want of a binding, even
/// though the corpus itself defines exactly what that variable is. A
/// header var never overrides a member's own same-named var (`or_insert`
/// only fills gaps), and a pool with no such header record simply merges
/// nothing, unchanged from before this widening.
/// Finds a pool's own HEADER record (`"<class> ~ <PoolHeaderName>"`) by a
/// singular/plural-normalized suffix match against `pool_group`, rather than
/// requiring the header's real corpus name to be byte-identical to the
/// `pool_group` string a call site passes -- `decisions.md §17a` (T12 cycle
/// 3): the real corpus HEADER for Slayer Talent is `"Slayer ~ Slayer
/// Talents"` (plural), while `"Slayer Talent"` (singular) is simultaneously
/// the CORRECT member-key prefix (`"Slayer Talent ~ <name>"` is the real
/// member shape) -- so the two names cannot be unified by changing the call
/// site's single string, and a hardcoded per-pool alias table is exactly the
/// "relabelled shape" anti-pattern `decisions.md §1a`/`§17` forbid. One
/// generic normalization instead: strip a single trailing `s` from both the
/// requested `pool_group` and each candidate header's own suffix (after its
/// `"<class> ~ "` prefix) before comparing, so `"Slayer Talent"` matches
/// `"Slayer Talents"` and an already-exact match (the common case --
/// `"Alchemist ~ Discovery"`, `"Witch ~ Hex"`) is unaffected either way.
/// `None` when no header record exists at all -- callers already treat that
/// as "this pool has no header chain to merge", unchanged from before this
/// widening.
///
/// SD-32 T12 Epic 8 row 18 cycle 11: this function used to return the FIRST
/// matching header record and stop -- correct while every pool had at most
/// one real header-shaped record, but WRONG once a pool has more than one
/// each contributing a DIFFERENT slice of the real chain. Confirmed live:
/// Cleric's own `"Cleric Domain ~ Air"` (a domain SPELL LIST record, class
/// `"Cleric"`, CATEGORY `"Internal"`, carrying a `SPELLLEVEL` token and ZERO
/// `BONUS:VAR` tokens) is a WHOLLY DIFFERENT record from the real power
/// header `"Air Domain"` (bare key, the one this function's own cycle-5
/// widening below already finds, carrying the actually useful
/// `DomainAirLVL`/`DomainAirDC`/`DomainAirTimes` chain) -- both real, both
/// legitimately named "the header" by different naming conventions, and a
/// single-match `Option` return can only ever surface one of them. This
/// function now MERGES every real header candidate's own `bonus_vars`
/// (never overwriting an already-bound target, same never-clobber policy
/// `merge_bonus_var_target_map_never_overwriting` already uses for the
/// cross-book merges) rather than returning the first hit, so a pool with
/// several real per-shape header records gets the union of all of them.
///
/// `registered_name` widening (cycle 11): a FOURTH real header shape, found
/// by tracing WHY Sorcerer Bloodline's terminal `BONUS:VAR` targets (e.g.
/// `Sorcerer_Aberrant_BloodlinePower1LVL`) sit unbound even though a
/// resolvable, wholly UNGATED definition exists in the corpus. Cycle 10
/// traced one such target to a row it believed was gated by a `PREABILITY:`
/// tag this module could not verify without the oracle's own `.java`
/// sources (its own receipt); the oracle's `.java` sources ARE readable from
/// its git objects without widening the sparse checkout (`git show
/// HEAD:code/src/java/plugin/pretokens/{parser/PreAbilityParser,
/// test/PreAbilityTester}.java`, confirmed live this cycle) -- but reading
/// them surfaces a DIFFERENT root cause than a PRE-gate this module cannot
/// evaluate: the record cycle 10 traced (`class_feature_bonus_vars_any_
/// record`'s bare-key `"Aberrant Bloodline"` match, contributed by
/// `advanced_class_guide`'s own Eldritch-Heritage-shaped fallback record,
/// `VAR|Sorcerer_Aberrant_BloodlinePower1LVL|1|!PREABILITY:...|!PREABILITY:
/// ...`) is NOT the real per-bloodline header PCGen's own class chooser
/// binds when a Sorcerer selects this bloodline normally. A SEPARATE,
/// wholly UNGATED corpus record this lookup never tried carries the real
/// formula: `"Sorcerer Bloodline ~ Aberrant"` (confirmed live,
/// `data/corpus/core_rulebook/class_feature/sorcerer_bloodline/
/// aberrant_bloodline.json`, `BONUS:VAR|Sorcerer_Aberrant_BloodlinePower1LVL
/// |Sorcerer_Aberrant_BloodlineLVL+BloodlinePower1LVLBonus`, no PRE-tag tail
/// at all) -- keyed `"<class> <registered_name> ~ <pool_group with its own
/// trailing \" <registered_name>\" stripped>"`. Confirmed the SAME shape,
/// not a one-bloodline coincidence, across every pool this row's census
/// already tracks that carries a `registered_name_for_tracker`: `"Cleric
/// Domain ~ Air"` (22 domains -- itself the SPELL LIST record above, not
/// the power header, which is exactly why merging rather than
/// first-match-wins matters here), `"Shaman Spirit ~ Battle"` (12 spirits),
/// `"Bloodrager Bloodline ~ ..."` (11), `"Cavalier Order ~ Order of the
/// ..."` (7, this one keeping the FULL unstripped `pool_group` as its own
/// suffix rather than a stripped one -- both shapes are tried below).
/// Warpriest Blessing carries ZERO such records (`grep` count 0) -- this
/// widening changes nothing for it, matching cycle 10's own finding that
/// Warpriest's gap is a genuinely different shape. `registered_name` is
/// `None` for the tracker-header call site below (unchanged -- that lookup
/// already targets a wholly different key, `"<registered_name> Tracker"`,
/// and never needs this widening) and for every caller that has never
/// passed a `registered_name_for_tracker` at all.
///
/// Returns the merged map directly (never a bare record reference) -- empty
/// when no header candidate matches at all, exactly equivalent to the old
/// `None` for every caller (`for (name, formula) in &merged { ... }` over an
/// empty map is a no-op, same as the old `if let Some(header) = ...`).
pub(super) fn pool_header_record_by_normalized_suffix(
    class: &str,
    pool_group: &str,
    registered_name: Option<&str>,
) -> crate::rules_core::record_vars::ConvertedChain {
    let table = class_feature_grant_consumer::class_feature_bonus_vars_any_record();
    let mut merged = crate::rules_core::record_vars::ConvertedChain::new();
    let mut merge_in = |header: &class_feature_grant_consumer::ClassFeatureRecordTokens| {
        class_feature_grant_consumer::merge_bonus_var_target_map_never_overwriting(
            &mut merged,
            header.bonus_vars.clone(),
        );
    };

    let exact_key = format!("{class} ~ {pool_group}");
    if let Some(header) = table.get(&exact_key) {
        merge_in(header);
    }

    if let Some(rn) = registered_name {
        // Widening 4 (cycle 11): try the unstripped shape (Cavalier Order's
        // own convention -- `pool_group` is already the header's own
        // suffix verbatim), then the trailing-`" <rn>"`-stripped shape
        // (Bloodline/Domain/Spirit's own convention).
        let unstripped_key = format!("{class} {rn} ~ {pool_group}");
        if let Some(header) = table
            .get(&unstripped_key)
            .filter(|header| header.class == class || header.class.is_empty())
        {
            merge_in(header);
        }
        if let Some(suffix) = pool_group.strip_suffix(&format!(" {rn}")) {
            let stripped_key = format!("{class} {rn} ~ {suffix}");
            if let Some(header) = table
                .get(&stripped_key)
                .filter(|header| header.class == class || header.class.is_empty())
            {
                merge_in(header);
            }
        }
    }

    let prefix = format!("{class} ~ ");
    let normalized_target = pool_group.trim_end_matches('s');
    if let Some(header) = table.iter().find_map(|(key, header)| {
        let suffix = key.strip_prefix(&prefix)?;
        (suffix.trim_end_matches('s') == normalized_target).then_some(header)
    }) {
        merge_in(header);
    }
    // SD-32 T12 Epic 8 row 18 cycle 5: a SECOND real corpus header shape,
    // confirmed by direct inspection of `data/corpus/core_rulebook/
    // class_feature/air/air.json` (key `"Air Domain"`, class `"Cleric"`,
    // real `BONUS:VAR|DomainAirDC|10+(DomainAirLVL/2)+CHA`) and
    // `.../aberrant_bloodline/aberrant_bloodline.json` (key
    // `"Aberrant Bloodline"`) -- unlike Alchemist Discovery/Witch Hex/
    // Slayer Talent's `"<class> ~ <pool word(s)>"` header key, the
    // Domain/Bloodline/Mystery/Blessing/Spirit "select ONE, inherit
    // everything" pool family keys its own header record with the BARE
    // group name (no `"<class> ~ "` prefix, no `" ~ "` separator at all)
    // -- the exact string `pool_group` already names (`"Air Domain"`,
    // `"Aberrant Bloodline"`). Class-checked before merging (never trusted
    // from the bare key alone) so a same-named group owned by a DIFFERENT
    // class can never be picked up by mistake.
    //
    // SD-32 T12 Epic 8 row 18 cycle 8: `header.class.is_empty()` also accepted -- confirmed live,
    // every real per-bloodline HEADER record in this corpus (Sorcerer's own "Marid Bloodline",
    // "Draconic Bloodline", ... -- all 53 groups' own headers, `class_feature_bonus_vars_any_
    // record`'s own doc) ingests with `class: null` (kept `""`, never fabricated) even after row
    // 21 restored their real `.MOD`-appended `BONUS:VAR` rows. A bare, un-namespaced header key
    // (`"Marid Bloodline"`, never `"<class> ~ Marid Bloodline"`) is already, by construction,
    // globally unique across this corpus -- no other class defines a same-named bare-key header --
    // so accepting an unowned one carries none of the cross-class-collision risk the `header.class
    // == class` check above exists to prevent; it merely lets a header this corpus never tagged
    // with its owner still contribute its own `BONUS:VAR` chain to the merge.
    if let Some(header) =
        table.get(pool_group).filter(|header| header.class == class || header.class.is_empty())
    {
        merge_in(header);
    }
    // SD-32 T12 Epic 8 row 18 cycle 13: a FOURTH real corpus header shape -- a bare (no `" ~ "`),
    // PLURAL class-independent record, confirmed live: `data/corpus/core_rulebook/class_feature/
    // domains/domains.json` (key exactly `"Domains"`, `data.class` genuinely absent -- real PCGen
    // never tags this record with a `CLASS:` token, it is granted generically via the Cleric
    // class's own `ABILITY:...|Domains` reference, `Cleric ~ Domains` `core_rulebook/class_
    // feature/cleric/domains.json`) carrying the REAL `BONUS:VAR|DomainPowerTimes|3+WIS` every
    // Domain member's own `Times` formula (`DomainAirTimes|DomainPowerTimes`, ...) chains through.
    // The exact bare-key clause above misses it (`"Domain"` singular != `"Domains"` plural); this
    // clause tolerates the same trailing-`s` PCGen already uses inconsistently between a pool's
    // registered name and its shared base record's key, the SAME normalization the `"<class> ~ "`
    // wildcard clause above already applies. Traced end to end for THIS one case before writing:
    // no other corpus record anywhere ever also targets `DomainPowerTimes` (confirmed by direct
    // grep), so merging this is a real, unambiguous corpus fact, not a guess. Class-checked
    // exactly like the exact-match clause above, so a same-named bare record genuinely owned by a
    // different class still cannot be picked up by mistake.
    let normalized_pool_group = pool_group.trim_end_matches('s');
    if let Some(header) = table
        .iter()
        .find_map(|(key, header)| {
            (!key.contains(" ~ ") && key.trim_end_matches('s') == normalized_pool_group)
                .then_some(header)
        })
        .filter(|header| header.class == class || header.class.is_empty())
    {
        merge_in(header);
    }
    // SD-32 T12 Epic 8 row 18 cycle 18 (`§27b`): a FIFTH real corpus header shape -- a `domain`-
    // kind record (never `class_feature`), keyed by the domain's own BARE name with no " Domain"
    // suffix (`"Cave"`, not `"Cave Domain"`) -- confirmed live,
    // `class_feature_grant_consumer::domain_kind_bonus_vars_any_record`'s own doc comment. Scoped
    // to `registered_name == Some("Domain")` (the only pool family this corpus shape belongs to;
    // Cavalier/Bloodline/Spirit groups never have a sibling `domain/` directory to collide with)
    // and to a `pool_group` that actually ends with the expected `" Domain"` suffix, so a group
    // this widening does not understand is never silently misrouted.
    if registered_name == Some("Domain")
        && let Some(bare) = pool_group.strip_suffix(" Domain")
            && let Some(vars) = class_feature_grant_consumer::domain_kind_bonus_vars_any_record().get(bare)
            {
                class_feature_grant_consumer::merge_bonus_var_target_map_never_overwriting(
                    &mut merged,
                    vars.clone(),
                );
            }
    // SD-32 T12 Epic 8 row 18 cycle 20 (`§27b`/`§17`): a SIXTH real corpus header shape, a
    // DIFFERENT directory from cycle 18's fifth (`domain/*.json`, bare-keyed) -- `class_feature/
    // domain_base/*.json`, keyed `"Domain Base ~ <bare>"` (e.g. `"Domain Base ~ Void"`,
    // confirmed live: `data/corpus/inner_sea_world_guide/class_feature/domain_base/void.json`,
    // `CATEGORY:Internal`, real `BONUS:VAR|DomainVoidLVL|DomainLVL`/`...DC`/`...Times` chain).
    // Its own `data.class` is the literal string `"Domain Base"` -- PCGen's own class-agnostic
    // marker for this record family, not a real playable class name (every one of its 40 real
    // members carries this exact string, never a PC class) -- so it is admitted the SAME way
    // `header.class.is_empty()` already is above: a corpus-tagged "not owned by any one class"
    // header can never collide with a real class-owned same-named record, so accepting it costs
    // nothing in cross-class-collision risk. Scoped to `registered_name == Some("Domain")`
    // exactly like cycle 18's fifth shape, for the same reason (no other pool family has a
    // sibling `domain_base/` directory). This closes `Void Domain` (whose only bonus_vars
    // member's sole terminal, `DomainVoidTimes`, chains through here) and corrects cycle 19's
    // own `§27b` exhaustive-verification claim that `Scalykind` carries "no BONUS:VAR-bearing
    // header exists anywhere in the corpus" -- `Domain Base ~ Scalykind` is real and was missed
    // (that verification checked `domain-kind` and bare `class_feature` shapes, never this
    // THIRD directory) -- filed as a retro correction, not silently.
    if registered_name == Some("Domain")
        && let Some(bare) = pool_group.strip_suffix(" Domain")
            && let Some(header) = table
                .get(&format!("Domain Base ~ {bare}"))
                .filter(|header| header.class == "Domain Base")
            {
                class_feature_grant_consumer::merge_bonus_var_target_map_never_overwriting(
                    &mut merged,
                    header.bonus_vars.clone(),
                );
            }
    // SD-32 T12 Epic 8 row 18 cycle 21 (`§27b`): a SEVENTH real corpus header shape -- not a new
    // FILE location like shapes 5/6, but a corpus-DECLARED relationship: a Wildblooded bloodline
    // variant's own PREABILITY token names a real, different, parent pool group its own header
    // vars are always genuinely bound through (see `wildblooded_variant_parent_pool_group`'s own
    // doc for the full derivation and why this is NOT cycle 17/19's unrelated-cross-bloodline
    // refusal shape). Recurses exactly once (a variant's own parent is never itself a variant,
    // confirmed live across all 20 real Wildblooded records) so no infinite loop is possible.
    if registered_name == Some("Bloodline")
        && let Some(parent) =
            class_feature_grant_consumer::wildblooded_variant_parent_pool_group().get(pool_group)
        {
            let parent_vars = pool_header_record_by_normalized_suffix(class, parent, registered_name);
            class_feature_grant_consumer::merge_bonus_var_target_map_never_overwriting(
                &mut merged,
                parent_vars,
            );
        }
    merged
}

/// `owning_class_override`: SD-32 T12 Epic 8 row 18 cycle 7. A real corpus quirk, confirmed live
/// (`data/corpus/ultimate_wilderness/class_feature/order_of_the_green/favored_terrain.json`,
/// key `"Order of the Green ~ Favored Terrain"`) -- some pool member records tag their own
/// `data.class` field SELF-REFERENTIALLY with the pool GROUP's own name (`"Order of the Green"`)
/// rather than the real owning PC class (`"Cavalier"`), even though the record's own formula text
/// references the real class's auto-var directly (`BONUS:VAR|CavalierFavoredTerrainLVL|
/// CavalierLVL`). Left at `record.class` (the original, unconditional behaviour, when this is
/// `None`), `class_level_variable_name` would derive the WRONG level variable
/// (`"OrderoftheGreenLVL"`, never bound) and the header lookup would filter on the wrong class
/// too -- silently refusing a record whose own chain is otherwise complete. A caller that has
/// ALREADY independently verified the real owning class (`push_generic_pool_group_selection_
/// magnitude`'s own `class` parameter, confirmed by `real_pool_group_for_selection_slug`'s
/// majority-tally-or-header-ownership-proof) may pass it here to override the member's own
/// possibly-unreliable self-tag. `None` (the flat-pool caller,
/// `push_generic_pool_choice_magnitude`, which has never needed this) preserves the exact
/// original behaviour, unchanged.
///
/// `registered_name_for_tracker`: SD-32 T12 Epic 8 row 18 cycle 8. The "select ONE group, inherit
/// every member" pool family (Bloodline, and PCGen's own equivalent convention elsewhere) very
/// often binds its own per-group header's chain vars (`Sorcerer_Marid_BloodlineLVL`, ...) onto a
/// SECOND, class-WIDE shared var (bare `BloodlineLVL`) via a real corpus record this codebase
/// calls a "Tracker" -- confirmed live, `"Bloodline Tracker"` (Sorcerer's own, a BARE key) and
/// `"Bloodrager ~ Bloodline Tracker"` (Bloodrager's own, the ordinary `"<class> ~ "`-prefixed
/// shape) -- a DIFFERENT record from the per-group header `pool_header_record_by_normalized_
/// suffix(owning_class, pool_group)` already merges. Passing `Some(registered_name)` (e.g.
/// `"Bloodline"`) merges this SECOND header too, via the SAME generic `pool_header_record_by_
/// normalized_suffix` lookup (parameterized by `"<registered_name> Tracker"`, not a hardcoded
/// per-class table) -- reusing the existing function rather than building a new one (`§17`).
/// `None` (every pre-existing caller) preserves the exact original single-header behaviour,
/// unchanged.
///
/// SD-32 T12 Epic 8 row 18 cycle 19: the per-group header, `"<RegisteredName> Tracker"` header,
/// class-wide `"<class> ~ <RegisteredName>"` bare base header, and the owning class's own
/// record-level `BONUS:VAR` chain, factored out of this function's own body into
/// [`pool_group_header_vars_merged`] so [`class_feature_grant_consumer::
/// resolved_description_for_formula_only_desc_argument`] -- the OTHER generic resolver, for a
/// record whose `bonus_vars` is EMPTY -- merges the SAME real corpus header chain this function
/// does, rather than an independently-maintained (and, until this cycle, incomplete) copy of it
/// (`§17`, generic passes not per-object work). This function's own behaviour is unchanged: the
/// extraction reproduces the exact same three-merge sequence, verified by the full unchanged
/// six-pool census re-run below.
pub(super) fn pool_group_header_vars_merged(
    owning_class: &str,
    pool_group: &str,
    registered_name_for_tracker: Option<&str>,
) -> crate::rules_core::record_vars::ConvertedChain {
    let mut combined_vars = crate::rules_core::record_vars::ConvertedChain::new();
    let header_vars = pool_header_record_by_normalized_suffix(
        owning_class,
        pool_group,
        registered_name_for_tracker,
    );
    for (name, var) in &header_vars {
        combined_vars.entry(name.clone()).or_insert_with(|| var.clone());
    }
    if let Some(registered_name) = registered_name_for_tracker {
        let tracker_name = format!("{registered_name} Tracker");
        let tracker_vars =
            pool_header_record_by_normalized_suffix(owning_class, &tracker_name, None);
        for (name, var) in &tracker_vars {
            combined_vars.entry(name.clone()).or_insert_with(|| var.clone());
        }
        // SD-32 T12 Epic 8 row 18 cycle 13: a THIRD real corpus shape for the class-wide shared
        // base of a "select ONE group, inherit every member" pool, distinct from both the
        // per-group header (`pool_header_record_by_normalized_suffix(owning_class, pool_group)`)
        // and the `"<RegisteredName> Tracker"` shape above. Confirmed live: `data/corpus/
        // advanced_class_guide/class_feature/shaman/spirit.json` (key `"Shaman ~ Spirit"`, class
        // `"Shaman"`, real `BONUS:VAR|ShamanSpiritLVL|ShamanLVL`) -- the EXACT `"<class> ~
        // <registered_name>"` shape (no `" Tracker"` suffix), never merged before cycle 13
        // because neither the per-group lookup (`pool_group` is the specific spirit's own name,
        // e.g. `"Bones Spirit"`, never the bare `"Spirit"` word) nor the Tracker lookup (which
        // only ever tries `"<rn> Tracker"`) reaches it. Traced end to end: every one of Shaman's
        // real per-spirit member formulas (e.g. `Bones Spirit ~ Shedding Form`'s
        // `ShamanSheddingFormRounds|ShamanSpiritLVL`) references this bare `ShamanSpiritLVL`
        // directly, with no competing corpus record anywhere else ever targeting that same name
        // (`every_corpus_bound_bonus_var_target()` unaffected -- this is a REAL, unambiguous
        // corpus-verified bind, not a guessed default), so merging it here closes real members
        // rather than fabricating one. Cavalier's own `"Cavalier ~ Order"` record matches this
        // same shape too (binds `OrderAbilityLVL`) but changes nothing there -- every real
        // Cavalier Order group's own members carry zero `BONUS:VAR` tokens at all
        // (`record.bonus_vars.is_empty()` returns `None` before `combined_vars` is even built),
        // confirmed unchanged by this cycle's own re-derived census. `None` for every class with
        // no such record (Sorcerer, Bloodrager, Cleric, Warpriest all lack a `"<class> ~
        // <registered_name>"` key -- confirmed by direct corpus lookup) preserves prior behaviour
        // exactly, unchanged. This is ALSO, per cycle 19, exactly the clause that reaches
        // Cleric Domain's own `"Domains"` bare plural record (`registered_name = "Domain"`,
        // `pool_header_record_by_normalized_suffix`'s own trailing-`s`-tolerant bare-key clause)
        // carrying `BONUS:VAR|DomainPowerTimes|3+WIS` -- the identifier `Mountain Domain ~
        // Foothold`'s own `%1` argument (`DomainMountainTimes|DomainPowerTimes`, itself only
        // reachable via the domain-kind header cycle 18 added) needs a SECOND hop through to
        // resolve, and which the desc-formula resolver's own pre-cycle-19 header merge (a single
        // `pool_header_record_by_normalized_suffix(class, pool_group, ...)` call, no tracker/base
        // hop) never reached -- confirmed live, this exact gap is what motivated factoring this
        // function out rather than duplicating a narrower, incomplete merge at the new call site.
        let base_vars =
            pool_header_record_by_normalized_suffix(owning_class, registered_name, None);
        for (name, var) in &base_vars {
            combined_vars.entry(name.clone()).or_insert_with(|| var.clone());
        }
    }
    // SD-32 T12 Epic 8 row 18 cycle 8: the owning CLASS's own record-level `BONUS:VAR` chain
    // (`class_record_bonus_vars`'s own doc -- Cleric's `DomainLVL`, real PCGen source
    // `cr_classes.lst`, binds HERE, never on any `class_feature` record) merged in too, always
    // tried (no gating flag -- a class with no such record simply merges nothing, exactly like an
    // absent per-group/tracker header above).
    if let Some(class_vars) = class_feature_grant_consumer::class_record_bonus_vars().get(owning_class) {
        for (name, var) in class_vars {
            combined_vars.entry(name.clone()).or_insert_with(|| var.clone());
        }
    }
    // SD-32 T12 Epic 8 row 18 cycle 22 (`§27b`/`§17a`): an EIGHTH real corpus header shape -- the
    // class's own `"<class> ~ Spells"` record, carrying the base spellcasting-stat `BONUS:VAR`
    // chain (`Sorcerer_Spells_StatBonus|CHA`, confirmed live: `data/corpus/core_rulebook/
    // class_feature/sorcerer/spells.json`). Traced end to end: Karmic/Seaborn/Warped Bloodline's
    // own real DC-formula members (`Fate's Retribution`/`Water Blast`/`Warp Touch`) all resolve
    // `10+(<Parent>BloodlinePower1LVL/2)+Sorcerer_Spells_StatBonus` -- every term but this LAST one
    // was already reachable through the SEVENTH shape's parent-header recursion (cycle 21); this
    // identifier alone was never merged anywhere, by ANY existing clause above (it lives on a
    // `"<class> ~ Spells"` record, a shape none of the per-group/Tracker/`"<class>
    // <registered_name>"` clauses reach), which is why `resolve_pcgen_var_chain`'s own corpus-wide
    // unbound-identifier default correctly REFUSED to fabricate a value for it (`Sorcerer_Spells_
    // StatBonus` IS bound elsewhere in the corpus -- `every_corpus_bound_bonus_var_target()` -- so
    // the 0-default fallback never applies; this was a genuine missing READ PATH, not a data gap).
    // The SAME missing var also blocked Sanguine Bloodline's `The Blood Is the Life` (`%1` desc-
    // formula argument `Sorcerer_Undead_BloodlinePower1Times`, itself chained through `Undead`'s
    // own `BloodlinePowerTimes|3+Sorcerer_Spells_StatBonus`) -- confirmed live by direct trace, not
    // assumed from the DC-formula fix alone. Two bloodlines override this base value locally on
    // their OWN member record (`Empyreal`: `WIS-CHA`, `Sage`: `INT-CHA`, confirmed live,
    // `data/corpus/ultimate_magic/class_feature/{empyreal,sage}_bloodline/bloodline_arcana.json`)
    // -- `combined_vars.entry(...).or_insert_with(...)` merged HERE, LAST and lowest-priority,
    // never overwrites either override, so both keep resolving through their own local bind
    // exactly as before this cycle. Unconditional and class-parameterized (no per-pool gating
    // flag), exactly like the class-record clause immediately above it: a class with no `"<class>
    // ~ Spells"` record (Cavalier, a non-caster) simply merges nothing, unchanged.
    if let Some(header) = class_feature_grant_consumer::class_feature_bonus_vars_any_record()
        .get(&format!("{owning_class} ~ Spells"))
    {
        for (name, var) in &header.bonus_vars {
            combined_vars.entry(name.clone()).or_insert_with(|| var.clone());
        }
    }
    combined_vars
}

/// SD-32 T12 Epic 8 row 18 cycle 20: the shared setup both
/// [`resolve_pool_member_sole_magnitude`] and [`resolve_pool_member_all_magnitudes`] build on --
/// looks the record up, merges in every real header source
/// ([`pool_group_header_vars_merged`]), and resolves the FULL PCGen var chain once. Returns the
/// record's own terminal target names (every `bonus_vars` key not itself referenced inside
/// ANOTHER `bonus_vars` formula on this same record -- the existing, unchanged
/// `is_referenced_elsewhere` filter, e.g. `Madness Domain`'s own `DomainMadnessLVL` is filtered
/// out because `DomainMadnessDC`'s formula names it, leaving `DomainMadnessDC`/
/// `DomainMadnessTimes`/`DomainMadnessAbilityTriggerLVL` as its three real terminals) together
/// with the resolved value map every one of those names is looked up in. `None` for a record with
/// no `bonus_vars` at all, or one whose target(s) never resolve through the chain -- both
/// preserved exactly, unchanged from the pre-cycle-20 single-target function this factors out of.
pub(super) fn pool_member_terminal_targets_and_resolved_vars(
    key: &str,
    pool_group: &str,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    owning_class_override: Option<&str>,
    registered_name_for_tracker: Option<&str>,
) -> Option<(Vec<String>, std::collections::BTreeMap<String, i64>)> {
    let record = class_feature_grant_consumer::class_feature_record_tokens_pre_gate_safe().get(key)?;
    if record.bonus_vars.is_empty() {
        return None;
    }
    let owning_class = owning_class_override.unwrap_or(&record.class);
    let is_referenced_elsewhere = |name: &str| {
        record
            .bonus_vars
            .iter()
            .any(|(other_name, var)| other_name != name && converted_var_names_identifier(var, name))
    };
    let terminals: Vec<String> = record
        .bonus_vars
        .keys()
        .filter(|name| !is_referenced_elsewhere(name))
        .cloned()
        .collect();
    if terminals.is_empty() {
        return None;
    }
    let mut combined_vars = record.bonus_vars.clone();
    let merged_header_vars =
        pool_group_header_vars_merged(owning_class, pool_group, registered_name_for_tracker);
    for (name, var) in &merged_header_vars {
        combined_vars.entry(name.clone()).or_insert_with(|| var.clone());
    }
    let class_level_var = class_feature_grant_consumer::class_level_variable_name(owning_class);
    let vars = class_feature_grant_consumer::resolve_pcgen_var_chain(
        &combined_vars,
        &class_level_var,
        level,
        ability_modifiers,
    );
    Some((terminals, vars))
}

/// SD-32 T12 Epic 8 row 18 cycle 20: genuinely resolves EVERY independent terminal target a pool
/// member record carries, rather than refusing once there is more than one. The real PCGen rule
/// this implements (established by reading `pcgen/core/PlayerCharacter.java:2136` and
/// `pcgen/core/BonusManager.java`'s `sumActiveBonusMap`, both already cited in
/// the converter-side bonus-stack reader's own module doc): a bonus target's value is the sum of every
/// ACTIVE contribution filed under THAT variable's own name -- summation only ever happens WITHIN
/// one target name. A record carrying several `BONUS:VAR` tokens with DIFFERENT target names
/// (`Forbidden Rites Domain ~ Madness Domain`'s real `DomainMadnessDC`/`DomainMadnessTimes`/
/// `DomainMadnessAbilityTriggerLVL`, confirmed live in `data/corpus/ultimate_magic/class_feature/
/// forbidden_rites_domain/madness.json`) is not one ambiguous quantity needing a guess between
/// three candidates -- it is THREE separate quantities, each already correctly and
/// unambiguously computed by [`class_feature_grant_consumer::resolve_pcgen_var_chain`]'s existing
/// full-chain evaluation (which was already computing every one of them correctly; only the
/// single-value REPORTING contract was discarding all but a guessed one). This function changes
/// nothing about HOW a value is computed -- it reports every terminal the shared resolver
/// ([`pool_member_terminal_targets_and_resolved_vars`]) already correctly derives, instead of
/// refusing. The refusal is preserved wherever it is still real: a record whose only terminal(s)
/// never resolve through the chain (an unbound external reference, an unrecognised formula
/// operator) is still silently absent from the returned `Vec`, exactly as `resolve_pool_member_
/// sole_magnitude` silently returns `None` for the same case -- "cannot resolve" is never
/// fabricated into a guessed 0 or omitted target here either. Callers push one
/// `ComputationExplanation` per returned `(target, value)` pair; the existing explanation `id`
/// scheme (`{id_prefix}.{group_slug}.{member_slug}.{target_slug}`,
/// `push_generic_pool_group_selection_magnitude`'s own format string) already keys on
/// `target_slug`, so multiple terminals from one member produce distinct ids with zero collision
/// risk, unchanged from before this cycle for every existing single-terminal member.
pub(crate) fn resolve_pool_member_all_magnitudes(
    key: &str,
    pool_group: &str,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    owning_class_override: Option<&str>,
    registered_name_for_tracker: Option<&str>,
) -> Vec<(String, i64)> {
    let Some((terminals, vars)) = pool_member_terminal_targets_and_resolved_vars(
        key,
        pool_group,
        level,
        ability_modifiers,
        owning_class_override,
        registered_name_for_tracker,
    ) else {
        return Vec::new();
    };
    terminals
        .into_iter()
        .filter_map(|target| vars.get(&target).copied().map(|value| (target, value)))
        .collect()
}

/// Whether `identifier` appears in `formula` as a whole token (not as a
/// substring of a longer identifier) -- splits on every character that is
/// not `[A-Za-z0-9_]`, PCGen formula syntax's own identifier-boundary set
/// (the same character class `formula_interpreter.rs`'s tokenizer treats
/// as an identifier body).
/// Whether a converted variable's source form referenced `identifier`. SD-35 `AT-35-E6-001`
/// cycle 4: this used to tokenize the source formula TEXT. The converter records each variable's
/// references by name at ingest (`ConvertedVar::refs`) precisely because the converted `Expr`
/// carries opaque ids, so the same question is now answered from the artifact.
pub(super) fn converted_var_names_identifier(
    var: &crate::rules_core::record_vars::ConvertedVar,
    identifier: &str,
) -> bool {
    var.refs.iter().any(|r| r == identifier)
}

/// Resolves a real, recorded `SelectedChoice::selection_id` (e.g.
/// `"discovery:feral_mutagen"`) back to the exact corpus `" ~ "`-qualified
/// key it names (e.g. `"Discovery ~ Feral Mutagen"`), group-agnostically:
/// scans every corpus `class_feature` record whose key starts with
/// `"<pool_group> ~ "`, folds each member's own display name through the
/// SAME `class_feature_id_slug` transform this codebase's own hand-picked
/// selection constants were built with (`FERAL_MUTAGEN_DISCOVERY_SELECTION
/// = "discovery:feral_mutagen"` <- `class_feature_id_slug("Feral
/// Mutagen")`), and returns the one whose `"<namespace><slug>"` matches
/// the recorded selection id exactly. `None` for an invented or
/// unrecognised selection id -- never guessed, mirroring
/// `chooser_option_selected`'s own "an invented id must never ground a
/// real magnitude" contract this module's existing Rage Power tests pin.
pub(crate) fn resolve_pool_selection_corpus_key(
    pool_group: &str,
    namespace: &str,
    selection_id: &str,
) -> Option<String> {
    let prefix = format!("{pool_group} ~ ");
    class_feature_grant_consumer::class_feature_record_tokens()
        .keys()
        .find(|key| {
            key.strip_prefix(&prefix)
                .is_some_and(|member_name| {
                    format!("{namespace}{}", class_feature_id_slug(member_name)) == selection_id
                })
        })
        .cloned()
}

/// Pushes one `ComputationExplanation` per REAL, recorded selection under
/// `choice_set_id` whose corpus pool member (`"<pool_group> ~ <member>"`)
/// carries a single resolvable terminal magnitude -- the generic pass
/// closing the residual tail of a large open-ended chooser (Discovery,
/// Hex, Curse, Mystery, Revelation, Spirit, Order, Animal Focus, Judgment,
/// Evolution, Blessing, Bloodline, Talent, ...) beyond whatever handful of
/// members this codebase already hand-models by name elsewhere in this
/// file. Purely ADDITIVE: an already hand-modelled selection's own
/// `ground_*` function still runs and still emits its own, differently-
/// named id (dice-notation magnitudes like Feral Mutagen's claw damage die
/// are refused here anyway, since `formula_interpreter.rs` does not parse
/// dice notation -- `resolve_pool_member_sole_magnitude` returns `None`
/// for them), so this function cannot collide with or double-count an
/// existing hand-picked explanation. A record that does not resolve is
/// silently skipped (never a fabricated value) -- the population that
/// still needs a bespoke function or an upstream data fix is reported by
/// the census script, not silently guessed at here.
// Each parameter is an independently-real input this generic pool-choice
// resolver needs (character input, level, ability modifiers, the choice
// set to read, plus the pool-record identity fields) -- bundling them into
// a struct would only add indirection for call sites that already pass
// them as named locals; out of this clippy-remediation cycle's scope.
#[allow(clippy::too_many_arguments)]
pub(super) fn push_generic_pool_choice_magnitude(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    choice_set_id: &str,
    pool_group: &str,
    namespace: &str,
    id_prefix: &str,
    min_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if level < min_level {
        // The pool itself is not yet granted at this level (e.g. Alchemist
        // Discovery's own `AlchemistDiscoveryLVL/2` gate does not open
        // until level 2) -- a recorded selection below that gate must
        // never ground a magnitude, mirroring every hand-picked `ground_*`
        // function's own level check.
        return;
    }
    let prefix = format!("{pool_group} ~ ");
    for selection_id in input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == choice_set_id)
        .map(|c| c.selection_id.as_str())
    {
        let Some(key) = resolve_pool_selection_corpus_key(pool_group, namespace, selection_id)
        else {
            continue;
        };
        // SD-32 T12 Epic 8 row 18 cycle 20: resolves EVERY independent terminal the member
        // carries (1 for the overwhelming majority, 2-3 for the newly-closed multi-terminal
        // shape) rather than refusing whenever there is more than one -- see
        // `resolve_pool_member_all_magnitudes`'s own doc for the PCGen citation. A single-
        // terminal member still produces exactly the one explanation it always did.
        let member_name = key.strip_prefix(&prefix).unwrap_or(&key);
        let member_slug = class_feature_id_slug(member_name);
        for (target, value) in
            resolve_pool_member_all_magnitudes(&key, pool_group, level, ability_modifiers, None, None)
        {
            let Ok(value) = i16::try_from(value) else { continue };
            let target_slug = class_feature_id_slug(&target);
            explanations.push(ComputationExplanation {
                id: format!("{id_prefix}.{member_slug}.{target_slug}"),
                value,
                detail: format!(
                    "{pool_group} member \"{member_name}\" (corpus key `{key}`, real level {level}): \
                     {target} = {value}. Resolved generically -- not a hand-picked, per-member \
                     function -- through resolve_pcgen_var_chain's real PCGen formula evaluator, \
                     seeded with this character's real class level and real ability modifiers \
                     (SD-32 T12 Epic 8, decisions.md §17 generic pool-choice magnitude resolver)."
                ),
            });
        }
    }
}

/// The "select ONE named group, then inherit every one of its members"
/// pool shape (Cleric Domain, Oracle Mystery, Warpriest Blessing, Shaman
/// Spirit, Sorcerer/Bloodrager Bloodline) -- the sibling of
/// [`push_generic_pool_choice_magnitude`]'s "select individual members
/// from one flat pool" shape (Discovery, Hex, Slayer Talent). `registered_name`
/// is the bare pool word (`"Domain"`, `"Mystery"`, `"Bloodline"`) a
/// selection's own slug is matched against; the real corpus GROUP a given
/// selection resolves to (`"Air Domain"`, `"Battle Mystery"`,
/// `"Draconic Bloodline"`) is found generically by
/// [`real_pool_group_for_selection_slug`] -- never a per-group lookup
/// table (`decisions.md §17`/`§1a`). Once the real group is known, EVERY
/// one of its own `"<group> ~ <member>"` corpus records is resolved
/// through the SAME [`resolve_pool_member_sole_magnitude`] used by the
/// flat-pool shape; a member this resolver cannot ground (multi-terminal,
/// dice notation, no BONUS/DEFINE token) is silently skipped, never
/// fabricated. Purely additive: an already hand-modelled selection (e.g.
/// Cleric's Good/War/Strength/Destruction/Glory/Healing domains, Oracle's
/// ten hand-picked mysteries, Shaman's ten hand-picked spirits) keeps
/// emitting its own, differently-named explanation id unchanged -- the
/// same "cannot collide, may legitimately overlap" contract cycle 4's own
/// Slayer Talent/Foil Scrutiny wiring already established.
// Same shape as `push_generic_pool_choice_magnitude` above, one parameter
// wider for the group-selection variant's own extra identity field --
// see that function's comment for why this stays unbundled.
#[allow(clippy::too_many_arguments)]
pub(super) fn push_generic_pool_group_selection_magnitude(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    choice_set_id: &str,
    class: &str,
    registered_name: &str,
    namespace: &str,
    id_prefix: &str,
    min_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if level < min_level {
        return;
    }
    for selection_id in input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == choice_set_id)
        .map(|c| c.selection_id.as_str())
    {
        let Some(slug) = selection_id.strip_prefix(namespace) else { continue };
        let Some(group) = real_pool_group_for_selection_slug(class, registered_name, slug) else {
            continue;
        };
        let prefix = format!("{group} ~ ");
        let group_slug = class_feature_id_slug(&group);
        for (key, _record) in class_feature_grant_consumer::class_feature_record_tokens_pre_gate_safe()
            .iter()
        {
            let Some(member_name) = key.strip_prefix(&prefix) else { continue };
            // SD-32 T12 Epic 8 row 18 cycle 20: every independent terminal (1-3), not a
            // sole-terminal refusal -- closes `Forbidden Rites Domain` and its Starsoul/
            // Celestial/Fey Bloodline siblings' genuine multi-terminal records. See
            // `resolve_pool_member_all_magnitudes`'s own doc.
            let member_slug = class_feature_id_slug(member_name);
            for (target, value) in resolve_pool_member_all_magnitudes(
                key,
                &group,
                level,
                ability_modifiers,
                Some(class),
                Some(registered_name),
            ) {
                let Ok(value) = i16::try_from(value) else { continue };
                let target_slug = class_feature_id_slug(&target);
                explanations.push(ComputationExplanation {
                    id: format!("{id_prefix}.{group_slug}.{member_slug}.{target_slug}"),
                    value,
                    detail: format!(
                        "{group} member \"{member_name}\" (corpus key `{key}`, real level {level}): \
                         {target} = {value}. Resolved generically -- not a hand-picked, per-member \
                         function -- through resolve_pcgen_var_chain's real PCGen formula evaluator, \
                         seeded with this character's real class level and real ability modifiers, \
                         after resolving the recorded {choice_set_id} -> {selection_id} selection to \
                         its real corpus group {group} (SD-32 T12 Epic 8, decisions.md §17 generic \
                         pool-group-selection magnitude resolver)."
                    ),
                });
            }
        }
    }
}

