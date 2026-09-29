#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm, risks item 8 (Witch full-build closure, 11th
/// ACG/APG class-specific closure): APG Witch, verified directly against
/// `apg_classes.lst`'s own `SPELLSTAT:INT` token (no `SPELLLIST:` reuse
/// token at all -- a fresh, own-list caster, deferred entirely this
/// slice, mirroring Skald's own spellcasting split). This closure was
/// re-scoped after a third comparative pass corrected the standing "all
/// three remaining classes blocked on an unbuilt Familiar/Eidolon"
/// verdict: Witch's own Ward hex (`KEY:Witch Hex ~ Ward`) is a flat,
/// self-scoped deflection/resistance bonus that does NOT require the
/// unbuilt Familiar subsystem at all -- confirmed its own gate
/// (`PREVARGTEQ:WitchHexAbilityLVL,1`, where `WitchHexAbilityLVL =
/// WitchLVL` unconditionally) is genuinely immediate at level 1, not a
/// delayed grant. See
/// `docs/release/v0.6/shaman-summoner-witch-comparative-scoping.md` for
/// the full corpus verification and scope record.
pub(super) const WITCH_CLASS_ID: &str = "class:witch";

/// The choice set for which Hex a Witch selects at 1st level (mirrors
/// `ORACLE_MYSTERY_CHOICE_ID`'s own single-selection shape).
pub(super) const WITCH_HEX_CHOICE_ID: &str = "choice:witch_hex";

/// Cauldron's `BONUS:SKILL|Craft (Alchemy)|4|TYPE=Insight`.
pub(super) const WITCH_CAULDRON_CRAFT_ALCHEMY_BONUS: i16 = 4;

/// Flight's `BONUS:SKILL|Swim|4|TYPE=Racial`.
pub(super) const WITCH_FLIGHT_SWIM_BONUS: i16 = 4;

/// v0.6 alpha swarm, risks item 8 (Witch full-build closure): whether
/// `input` is a single-class Witch at a level within
/// `apg::class_chassis_resolve`'s declared ceiling for Witch -- mirrors
/// `is_supported_oracle_single_class`/the other APG exact-match gates
/// exactly.
pub(super) fn is_supported_witch_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if ApgClassId::from_class_id_str(&class_level.class_id) != Some(ApgClassId::Witch) {
        return false;
    }
    apg::class_chassis_resolve(ApgClassId::Witch, class_level.level, RuleSetId::Apg).is_some()
}

/// v0.6 alpha swarm (Summoner chassis-recognition closure, 2026-07-29):
/// whether `input` is a single-class Summoner at a level within
/// `apg::class_chassis_resolve`'s declared ceiling for Summoner --
/// mirrors `is_supported_witch_single_class`/the other APG exact-match
/// gates exactly, including the same exact-match discipline
/// (`== Some(ApgClassId::Summoner)`, not a broad `.is_some()`).
///
/// Summoner was the last of the six APG classes with no arm in
/// `has_supported_class_chassis`, and its gap had the OPPOSITE shape to
/// Monk's. `compute_class_chassis`'s APG branch already resolved Summoner
/// correctly -- which is exactly why Summoner never emitted
/// `class_chassis.unsupported` at any level, unlike Monk. The chassis was
/// computed and then discarded: every consumer gated on this predicate
/// (`unmet_combat_posture_conditions`, `compute_total_saves`,
/// `unmet_selected_skill_posture_conditions`) refused a class whose
/// base attack bonus and base saves were already sitting there. So the
/// three downstream diagnostics Summoner shared with Monk had a wholly
/// different cause, and reading Monk's root cause across to Summoner
/// without checking would have been wrong.
pub(super) fn is_supported_summoner_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if ApgClassId::from_class_id_str(&class_level.class_id) != Some(ApgClassId::Summoner) {
        return false;
    }
    apg::class_chassis_resolve(ApgClassId::Summoner, class_level.level, RuleSetId::Apg).is_some()
}

/// PF1 Advanced Player's Guide Witch Ward hex: a flat deflection-AC/
/// saving-throw-resistance bonus, verified directly against
/// `apg_abilities_class.lst`'s own `BONUS:VAR|WitchWardBonus|2` (base),
/// `+1` at level 8 (`PREVARGTEQ:WitchHexAbilityLVL,8`), `+1` at level 16
/// (`PREVARGTEQ:WitchHexAbilityLVL,16`) -- `WitchHexAbilityLVL` resolves
/// to `WitchLVL` directly, confirmed unconditional (no delayed-grant gate
/// the way Shaman's Healer's Touch has).
pub fn witch_ward_bonus(level: u8) -> i16 {
    let mut bonus = 2;
    if level >= 8 {
        bonus += 1;
    }
    if level >= 16 {
        bonus += 1;
    }
    bonus
}

/// PF1 Advanced Player's Guide Summoner.
pub(super) const SUMMONER_CLASS_ID: &str = "class:summoner";

/// The Eidolon's own hit die (`HD:10` on `CLASS:Eidolon`,
/// `apg_classes.lst`) -- notably larger than the Wolf companion's.
pub(super) const EIDOLON_HIT_DIE: u8 = 10;

/// The Eidolon base race's racial natural-armor bonus
/// (`BONUS:VAR|AC_Natural_Armor|2`, `apg_races_companion.lst`), which the
/// level-driven progression stacks on top of.
pub(super) const EIDOLON_RACIAL_NATURAL_ARMOR: i16 = 2;

/// The Eidolon base race's own land speed (`MOVE:Walk,20`).
pub(super) const EIDOLON_BASE_WALK_SPEED: i16 = 20;

/// Each `Evolution ~ Legs` grants `BONUS:MOVEADD|TYPE.Walk|10`. The
/// canonical Quadruped is granted Legs TWICE, automatically.
pub(super) const EIDOLON_SPEED_PER_LEGS_EVOLUTION: i16 = 10;

/// The Bite evolution's damage die (`NATURALATTACKS:1 Bite,...,*1,1d6`).
/// Form-restricted: its `TYPE:` carries
/// `EvolutionQuadruped.EvolutionSerpentine`, so a Biped Eidolon does NOT
/// get it -- do not generalize this to the other base forms.
pub(super) const EIDOLON_BITE_DAMAGE_DIE: i16 = 6;

/// The choice slot recording WHICH evolution the Eidolon actually bought
/// out of `eidolon_evolution_pool`'s points (v0.6 alpha swarm, Summoner
/// Eidolon evolution canonical-narrowing closure, 2026-07-29).
///
/// Summoner's evolution menu is a point-buy economy over 104 real
/// `KEY:Evolution ~ *` records, and modelling all of it was deliberately
/// deferred as a product decision. This slot is the canonical-narrowing
/// closure of that decision, and is the same recognized-choice shape
/// Cleric's domain, Wizard's school, Sorcerer's bloodline, Oracle's
/// Mystery and Arcanist's Metamagic Knowledge Exploit already ratified:
/// ONE corpus-verified member of the menu is genuinely built, an
/// unrecognized or absent selection keeps the original claim-blocking
/// diagnostic, and the rest of the menu is named honestly in a
/// non-blocking note.
///
/// Deliberately NOT a numbered slot family in the
/// `BARBARIAN_RAGE_POWER_SLOTS` / `choice:rogue_talent_N` shape. Those
/// menus grant a fixed number of picks at fixed levels, so a slot per
/// pick is the right model. Evolutions are bought against a POINT pool
/// with per-evolution costs of 1-4, so "how many picks exist" is not a
/// function of level at all -- inventing N slots would misrepresent the
/// economy's own shape.
pub(super) const SUMMONER_EIDOLON_EVOLUTION_CHOICE_ID: &str = "choice:summoner_eidolon_evolution";

/// The total count of `KEY:Evolution ~ *` records in
/// `apg_abilities_companion.lst`, enumerated directly.
pub(super) const EIDOLON_EVOLUTION_RECORD_COUNT: usize = 104;

/// The Eidolon's evolution-point pool for a Summoner of `level`,
/// transcribed from `apg_abilities_companion.lst`'s own
/// `BONUS:VAR|EidolonEvolution|3+(SummonerLVL>=2)+...` -- a base 3 plus
/// one point at most levels and TWO at 4/9/14/19.
///
/// Two corpus terms are deliberately excluded, both verified vacuous
/// here: `mastervar("EidolonEvolution")` mirrors this same value from
/// the master onto the companion (computing both would double-count),
/// and `EidolonFavoredClassBonusEvolutionPointsEveryFour/4` requires a
/// favored-class-bonus selection mechanism this engine does not model at
/// all -- the same provably-vacuous shape as Alchemist's Gnome-only
/// `BonusBombCount`.
///
/// This is the pool SIZE only. Spending is not modelled, which is why
/// Summoner stays claim-blocked.
pub(super) fn eidolon_evolution_pool(level: u8) -> i16 {
    let level = i16::from(level);
    let at_least = |n: i16| i16::from(level >= n);
    3 + at_least(2)
        + at_least(3)
        + 2 * at_least(4)
        + at_least(5)
        + at_least(6)
        + at_least(7)
        + at_least(8)
        + 2 * at_least(9)
        + at_least(10)
        + at_least(11)
        + at_least(12)
        + at_least(13)
        + 2 * at_least(14)
        + at_least(15)
        + at_least(16)
        + at_least(17)
        + at_least(18)
        + 2 * at_least(19)
        + at_least(20)
}

/// The Eidolon's level-driven natural-armor bonus: `+2` at each of eight
/// master-level gates (2/5/7/10/12/15/17/20), verified against
/// `BONUS:VAR|EidolonNaturalArmorBonus|if(MasterLevel>=2,2,0)+...`.
/// Stacks on top of the racial `EIDOLON_RACIAL_NATURAL_ARMOR`.
pub(super) fn eidolon_natural_armor_bonus(level: u8) -> i16 {
    let level = i16::from(level);
    [2, 5, 7, 10, 12, 15, 17, 20]
        .iter()
        .map(|gate| 2 * i16::from(level >= *gate))
        .sum()
}

/// The Eidolon's maximum number of natural attacks:
/// `3+(MasterLevel>=4)+(>=9)+(>=14)+(>=19)` -> 3/4/5/6/7.
pub(super) fn eidolon_max_natural_attacks(level: u8) -> i16 {
    let level = i16::from(level);
    3 + [4, 9, 14, 19].iter().map(|gate| i16::from(level >= *gate)).sum::<i16>()
}

/// The Eidolon's base attack bonus: FULL, not three-quarters
/// (`BONUS:COMBAT|BASEAB|classlevel(...)` on `CLASS:Eidolon`).
///
/// This is the single sharpest reason `ground_wolf_companion_stat_block`
/// could not be reused: the Wolf is a 3/4-BAB, fixed-2-HD creature whose
/// own helper carries `debug_assert_eq!(companion_level, 1)`, while an
/// Eidolon needs full BAB across all 20 levels by construction. Right
/// idiom, wrong function -- the same category as Shaman's Spirit Animal
/// turning out to be a Familiar rather than an Animal Companion.
pub(super) fn eidolon_base_attack_bonus(level: u8) -> i16 {
    i16::from(level)
}

/// Slice A grant levels, read directly off the `.MOD` grant lines attached to
/// `Summoner ~ Standard Class` in `apg_abilities_class.lst` (each carrying a
/// real `PREVARGTEQ:Summoner_CFP_Level,<N>`), NOT off the features' own
/// `KEY:Summoner ~ <Feature>` records, which carry no `PRE` gate at all.
///
/// The distinction matters: every one of these formulas evaluates to a
/// plausible non-zero at level 1, so building them ungated would ship real
/// numbers for features the character does not yet have -- the same failure
/// shape as a formula that evaluates fine for an ability that does not exist.
/// `Summoner_CFP_Level` is a live proxy for the real class level
/// (`BONUS:VAR|Summoner_CFP_Level|classlevel("Summoner")`), not a dead
/// variable.
pub(super) const SUMMONER_SUMMON_MONSTER_LEVEL: u8 = 1;

pub(super) const SUMMONER_BOND_SENSES_LEVEL: u8 = 2;

pub(super) const SUMMONER_MAKERS_CALL_LEVEL: u8 = 6;

pub(super) const SUMMONER_MERGE_FORMS_LEVEL: u8 = 16;

pub(super) const SUMMONER_TWIN_EIDOLON_LEVEL: u8 = 20;

/// Bond Senses: `BONUS:VAR|BondSensesRounds|classlevel("Summoner")` --
/// rounds per day the summoner may share the eidolon's senses.
pub fn summoner_bond_senses_rounds_per_day(level: u8) -> i16 {
    i16::from(level)
}

/// Maker's Call: `BONUS:VAR|MakersCallTimes|((classlevel("Summoner")-2)/4)`
/// -- 1/day at 6th, +1 every four levels after (6->1, 10->2, 14->3, 18->4).
/// The formula is self-gating (it is 0 below 6th), and its first non-zero
/// value lands exactly on the independently-confirmed grant level.
pub fn summoner_makers_call_uses_per_day(level: u8) -> i16 {
    (i16::from(level) - 2) / 4
}

/// Merge Forms: `BONUS:VAR|MergeFormsRounds|classlevel("Summoner")`.
pub fn summoner_merge_forms_rounds_per_day(level: u8) -> i16 {
    i16::from(level)
}

/// Twin Eidolon: `BONUS:VAR|TwinEidolonMinutes|classlevel("Summoner")`.
pub fn summoner_twin_eidolon_minutes_per_day(level: u8) -> i16 {
    i16::from(level)
}

/// Summon Monster's duration: `BONUS:VAR|SummonMonsterDuration|
/// classlevel("Summoner")`, in **minutes**. The unit is not in the token --
/// it comes from the record's own `DESC`, which states it outright: "the
/// creatures remain for %3 **minutes** (instead of %3 rounds)", where `%3`
/// is this variable. Worth pinning, because the summoned-creature default
/// really is rounds and this ability's whole point is upgrading that unit.
pub fn summoner_summon_monster_duration_minutes(level: u8) -> i16 {
    i16::from(level)
}

/// Summon Monster's uses per day: `BONUS:VAR|SummonMonsterTimes|CHA+3`.
/// Applied verbatim with no floor at 0 -- the corpus adds the raw modifier,
/// the same treatment every other ability-derived quantity in this file
/// gets. A Charisma penalty genuinely reduces the count.
pub(super) fn summoner_summon_monster_uses_per_day(charisma_modifier: i16) -> i16 {
    charisma_modifier + 3
}

/// Summon Monster's accessible spell level: `BONUS:VAR|SummonMonsterLVL|
/// min(9,(classlevel("Summoner")+1)/2)` -- I at 1st through IX at 17th, and
/// the `min(9, ...)` genuinely binds, since `(20+1)/2` is 10 at the capstone.
pub(super) fn summoner_summon_monster_spell_level(level: u8) -> i16 {
    ((i16::from(level) + 1) / 2).min(9)
}

/// Grounds Summoner's five flat, self-scoped class-feature pools (Slice A).
///
/// Each is the Fervor/Panache/Challenge-uses shape this codebase has grounded
/// repeatedly: a verified, level-derived magnitude whose *effect* is not
/// modelled but whose *quantity* is a real fact. None of them reads any
/// property of the eidolon's stat block, which is why they are separable from
/// the evolution economy that keeps Summoner blocked.
///
/// Summon Monster splits the same way Bomb does -- its duration, uses per day
/// and accessible spell level are grounded; the actual summoning (creature
/// selection, stat blocks, the eidolon-unsummoned precondition) is not
/// modelled anywhere and is deliberately left ungrounded rather than faked.
///
/// Deliberately does NOT touch Aspect/Greater Aspect: those divert points out
/// of the eidolon's own evolution pool, making them a chooser over a shared
/// resource rather than an independent fact.
pub(super) fn ground_summoner_slice_a_features(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if level >= SUMMONER_SUMMON_MONSTER_LEVEL {
        let charisma_modifier = ability_modifier(input.chosen.ability_scores.charisma);
        let duration = summoner_summon_monster_duration_minutes(level);
        let uses = summoner_summon_monster_uses_per_day(charisma_modifier);
        let spell_level = summoner_summon_monster_spell_level(level);

        explanations.push(ComputationExplanation {
            id: "class_feature.apg.summoner.summon_monster_duration_minutes".to_owned(),
            value: duration,
            detail: format!(
                "Summoner level {level} Summon Monster spell-like ability: summoned creatures \
                 remain {duration} minutes (corpus `SummonMonsterDuration = \
                 classlevel(\"Summoner\")`; the record's own DESC states the unit outright -- \
                 \"remain for {duration} minutes (instead of {duration} rounds)\"). Grounds the \
                 duration only: no summoning, creature selection, or stat block is modelled \
                 anywhere in this codebase, and none is fabricated here"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.summoner.summon_monster_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Summoner Summon Monster uses per day: {uses} (corpus `SummonMonsterTimes = \
                 CHA+3`, with this character's Charisma modifier {charisma_modifier:+} applied \
                 verbatim and not floored). Grounds the per-day budget only; the ability's real \
                 precondition (usable only while the eidolon is not summoned) is not modelled"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.summoner.summon_monster_spell_level".to_owned(),
            value: spell_level,
            detail: format!(
                "Summoner level {level} Summon Monster accessible spell level: {spell_level} \
                 (corpus `min(9,(classlevel(\"Summoner\")+1)/2)` -- summon monster I at 1st \
                 through IX at 17th; the min genuinely binds, since (20+1)/2 would otherwise \
                 reach 10 at the capstone). Grounds which summon monster spell level is \
                 reachable, not its contents"
            ),
        });
    }

    if level >= SUMMONER_BOND_SENSES_LEVEL {
        let rounds = summoner_bond_senses_rounds_per_day(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.summoner.bond_senses_rounds_per_day".to_owned(),
            value: rounds,
            detail: format!(
                "Summoner level {level} Bond Senses: {rounds} rounds per day of shared eidolon \
                 senses (corpus `BondSensesRounds = classlevel(\"Summoner\")`, granted at \
                 summoner level {SUMMONER_BOND_SENSES_LEVEL}). Grounds the daily round budget \
                 only -- no sense-sharing, perception, or action-economy engine exists here"
            ),
        });
    }

    if level >= SUMMONER_MAKERS_CALL_LEVEL {
        let uses = summoner_makers_call_uses_per_day(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.summoner.makers_call_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Summoner level {level} Maker's Call: {uses} uses per day (corpus \
                 `MakersCallTimes = (classlevel(\"Summoner\")-2)/4`, granted at summoner level \
                 {SUMMONER_MAKERS_CALL_LEVEL} -- 1/day at 6th, one more every four levels \
                 after). Grounds the per-day budget only; the teleport-the-eidolon-to-your-side \
                 effect is not modelled"
            ),
        });
    }

    if level >= SUMMONER_MERGE_FORMS_LEVEL {
        let rounds = summoner_merge_forms_rounds_per_day(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.summoner.merge_forms_rounds_per_day".to_owned(),
            value: rounds,
            detail: format!(
                "Summoner level {level} Merge Forms: {rounds} rounds per day merged with the \
                 eidolon (corpus `MergeFormsRounds = classlevel(\"Summoner\")`, granted at \
                 summoner level {SUMMONER_MERGE_FORMS_LEVEL}). Grounds the daily round budget \
                 only -- the merged-form state itself (shared hit points, suppressed actions) \
                 is not modelled"
            ),
        });
    }

    if level >= SUMMONER_TWIN_EIDOLON_LEVEL {
        let minutes = summoner_twin_eidolon_minutes_per_day(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.summoner.twin_eidolon_minutes_per_day".to_owned(),
            value: minutes,
            detail: format!(
                "Summoner level {level} Twin Eidolon: {minutes} minutes per day assuming the \
                 eidolon's shape (corpus `TwinEidolonMinutes = classlevel(\"Summoner\")`, \
                 granted at the summoner level {SUMMONER_TWIN_EIDOLON_LEVEL} capstone). Grounds \
                 the daily minute budget only -- copying the eidolon's evolutions and ability \
                 scores would require the evolution economy this class stays blocked on"
            ),
        });
    }
}

/// Grounds the canonical Quadruped Eidolon's corpus-derived stat block
/// and its evolution-point pool (task #17, 2026-07-27), then
/// claim-blocks on the unspent evolutions.
///
/// Quadruped is the canonical base form because it has the smallest
/// distinct-evolution-type surface of the three (Bite + Legs, versus
/// Biped's three types and Serpentine's five), not because of any
/// resemblance to the Wolf companion.
///
/// Every base-form evolution is granted `ABILITY:Eidolon Evolution|
/// AUTOMATIC|...`, so this whole stat block is determined with ZERO
/// player evolution choices -- nothing here is a silently-seeded pick.
/// The `COST:2` on `Evolution ~ Legs` does NOT draw against the pool for
/// these automatic grants; charging it would spend 4 points from a
/// 3-point level-1 pool and yield an impossible negative.
pub(super) fn ground_summoner_eidolon(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let pool = eidolon_evolution_pool(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.evolution_pool".to_owned(),
        value: pool,
        detail: format!(
            "Summoner level {level} Eidolon evolution pool: {pool} evolution points. Grounds the \
             pool SIZE only -- which evolutions are bought is not modelled, the same split \
             already used for Warpriest's Fervor pool and Cavalier's Challenge uses per day. The \
             canonical Quadruped's own evolutions (Bite, Legs, Legs) are granted AUTOMATIC by \
             the corpus and draw nothing from this pool"
        ),
    });

    let bab = eidolon_base_attack_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.base_attack_bonus".to_owned(),
        value: bab,
        detail: format!(
            "Quadruped Eidolon base attack bonus: +{bab}, FULL progression (equal to the \
             summoner's own level), not the three-quarters an animal companion gets. The Eidolon \
             also uses a d{EIDOLON_HIT_DIE} hit die and takes its hit dice from the summoner's \
             level, so it is a genuinely different creature chassis from this codebase's Wolf \
             companion rather than a reuse of it"
        ),
    });

    let natural_armor = eidolon_natural_armor_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.natural_armor_bonus".to_owned(),
        value: natural_armor,
        detail: format!(
            "Quadruped Eidolon level-driven natural armor bonus: +{natural_armor} (+2 at each of \
             summoner levels 2, 5, 7, 10, 12, 15, 17, and 20). The Eidolon base race carries a \
             further racial +{EIDOLON_RACIAL_NATURAL_ARMOR} natural armor that stacks on top, so \
             the creature's total natural armor is +{}. Grounded standalone: this engine computes \
             no armor-class total for a companion creature",
            natural_armor + EIDOLON_RACIAL_NATURAL_ARMOR
        ),
    });

    let max_attacks = eidolon_max_natural_attacks(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.max_natural_attacks".to_owned(),
        value: max_attacks,
        detail: format!(
            "Quadruped Eidolon maximum natural attacks: {max_attacks} (3, rising by one at \
             summoner levels 4, 9, 14, and 19). A ceiling on how many natural attacks its \
             evolutions may grant, not a count of attacks it currently has"
        ),
    });

    let speed = EIDOLON_BASE_WALK_SPEED
        + EIDOLON_SPEED_PER_LEGS_EVOLUTION * QUADRUPED_LEGS_EVOLUTIONS;
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.base_land_speed".to_owned(),
        value: speed,
        detail: format!(
            "Quadruped Eidolon base land speed: {speed} feet -- the Eidolon race's own \
             {EIDOLON_BASE_WALK_SPEED} feet plus {EIDOLON_SPEED_PER_LEGS_EVOLUTION} feet for \
             each of the {QUADRUPED_LEGS_EVOLUTIONS} Legs evolutions Quadruped is granted \
             automatically, matching the published Quadruped speed"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.bite_damage_die".to_owned(),
        value: EIDOLON_BITE_DAMAGE_DIE,
        detail: format!(
            "Quadruped Eidolon Bite: 1d{EIDOLON_BITE_DAMAGE_DIE}, a primary natural attack. The \
             Bite evolution is form-restricted (its corpus TYPE names Quadruped and Serpentine \
             only), so a Biped Eidolon does not get it -- this magnitude must not be generalized \
             to the other base forms"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.quadruped_ability_bonuses".to_owned(),
        value: QUADRUPED_STRENGTH_BONUS,
        detail: format!(
            "Quadruped Eidolon ability-score bonuses: +{QUADRUPED_STRENGTH_BONUS} Strength and \
             +{QUADRUPED_DEXTERITY_BONUS} Dexterity from the base form, on top of the Eidolon \
             race's own +2 Constitution and -4 Intelligence. The bonuses are grounded rather \
             than absolute scores, because the creature's starting scores are not stated as \
             corpus constants and inventing them would be fabrication. Good saves are Fortitude \
             and Reflex for THIS form specifically -- Biped gets Fortitude and Will, Serpentine \
             Will and Reflex, so the pattern must not be hardcoded"
        ),
    });

    ground_or_block_summoner_eidolon_evolutions(input, level, pool, explanations, diagnostics);
}

/// The still-deferred remainder, named identically in both the blocking
/// and non-blocking branches so the two cannot drift apart.
///
/// Enumerated from a full `KEY:Summoner ~` / `KEY:Eidolon ~` corpus
/// sweep. Bond Senses, Maker's Call, Merge Forms, Twin Eidolon and the
/// Summon Monster spell-like ability are deliberately NOT in this list --
/// all five are genuinely grounded (task #35).
pub(super) const SUMMONER_REMAINING_DEFERRED_FEATURES: &str =
    "the other two base forms (Biped, Serpentine), Aspect and Greater Aspect, Life Link, Life \
     Bond, Shield Ally and Greater Shield Ally, Transposition, Gate, the Eidolon's own Link, \
     Share Spells and Skills records, and Summoner's own spontaneous Charisma spellcasting \
     including its Cantrips";

/// Grounds or claim-blocks the Eidolon's evolution spending (v0.6 alpha
/// swarm, Summoner Eidolon evolution canonical-narrowing closure,
/// 2026-07-29) -- the last blocker on the last unclosed class of the
/// 27-class roster.
///
/// A recognized `choice:summoner_eidolon_evolution` naming
/// `evolution:improved_natural_armor` spends a real point out of the
/// already-grounded pool, grounds its `+2` into the Eidolon's
/// natural-armor TOTAL, and replaces
/// the claim-blocking `evolutions_deferred` diagnostic with a
/// non-blocking note naming the other 103 evolutions and the still-unspent
/// points. An absent or unrecognized selection keeps the original
/// claim-blocking diagnostic exactly as it was.
///
/// **Why unspent points are honestly non-blocking once one is spent.**
/// The original blocker's real complaint was that "which evolutions a
/// given Eidolon actually bought is unknown" -- the engine had no way to
/// record a purchase at all. Once a purchase is recorded and validated,
/// leftover points are a legal PF1 character state, not an unmodelled
/// mechanic, and they are reported explicitly rather than hidden. This is
/// the same line this codebase already drew for Barbarian's ten rage-power
/// slots and Rogue's talent slots: both classes reach Computed while
/// recording WHICH pick was made without applying most picks' effects, and
/// both name that chooser-enforcement burden as still open. Nothing here
/// weakens the blocker to force green -- an unseeded Summoner is still
/// Blocked, and a bogus evolution still blocks.
pub(super) fn ground_or_block_summoner_eidolon_evolutions(
    input: &CharacterInput,
    level: u8,
    pool: i16,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(selection) = choice_selection(input, SUMMONER_EIDOLON_EVOLUTION_CHOICE_ID) else {
        push_summoner_evolutions_deferred_diagnostic(diagnostics);
        return;
    };

    if selection != IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.summoner.eidolon.evolution_unrecognized".to_owned(),
            message: format!(
                "The Eidolon's evolution choice names '{selection}' via \
                 {SUMMONER_EIDOLON_EVOLUTION_CHOICE_ID}, but the only evolution this engine \
                 genuinely builds is \
                 '{IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION}'. The other \
                 {} of the corpus's {EIDOLON_EVOLUTION_RECORD_COUNT} \
                 `KEY:Evolution ~ *` records are named but not modelled, so no purchase is \
                 fabricated for this selection",
                EIDOLON_EVOLUTION_RECORD_COUNT - 1
            ),
            claim_blocking: true,
        });
        push_summoner_evolutions_deferred_diagnostic(diagnostics);
        return;
    }

    // Both of this purchase's legality conditions are satisfied by
    // construction for a SINGLE instance, and are asserted rather than
    // branched on, because a branch that can never be taken is dead
    // scaffolding dressed as a safety check:
    //
    //   * Affordability. `eidolon_evolution_pool` is 3 at level 1 and
    //     only rises, so a cost-1 evolution is affordable at every level
    //     in the sweep. A cost-4 evolution would NOT have been -- that is
    //     precisely why the canonical pick is a cheap one.
    //   * The record's own `PREVARLTEQ:EvoImpNatArmCount,MasterLevel/5`.
    //     For the first instance the check is `0 <= level/5`, true at
    //     every level. A SECOND instance would require level 5.
    //
    // These assertions are the tripwire for whoever widens this slice:
    // add a costlier evolution, or a second instance of this one, and the
    // condition stops holding by construction and must become a real
    // runtime branch with a real diagnostic. Same idiom as the Wolf
    // companion helper's own `debug_assert_eq!(companion_level, 1)`.
    let instances_taken: i16 = 1;
    debug_assert!(
        IMPROVED_NATURAL_ARMOR_COST <= pool,
        "level {level}: Improved Natural Armor costs {IMPROVED_NATURAL_ARMOR_COST} but the \
         evolution pool is only {pool}"
    );
    debug_assert!(
        instances_taken - 1 <= i16::from(level) / IMPROVED_NATURAL_ARMOR_LEVELS_PER_EXTRA,
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   PREVARLTEQ:EvoImpNatArmCount,MasterLevel/5
        "level {level}: {instances_taken} instances of Improved Natural Armor exceeds the \
         corpus cap of one per five master levels"
    );

    let spent = IMPROVED_NATURAL_ARMOR_COST * instances_taken;
    let unspent = pool - spent;

    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.improved_natural_armor".to_owned(),
        value: IMPROVED_NATURAL_ARMOR_BONUS,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:VAR|AC_Natural_Armor|2|TYPE=Base.STACK` on `KEY:Evolution ~ Improved Natural
            //   Armor`, corroborated by its own DESC
            //   `PREVARLTEQ:EvoImpNatArmCount,MasterLevel/5` -- at most one instance per
            //   {IMPROVED_NATURAL_ARMOR_LEVELS_PER_EXTRA} summoner levels
            "Quadruped Eidolon Improved Natural Armor evolution: +{IMPROVED_NATURAL_ARMOR_BONUS} \
             natural armor. Bought for {IMPROVED_NATURAL_ARMOR_COST} evolution point -- the record \
             carries no COST field, which in this corpus means the default of 1, not a missing \
             value. Its `TYPE:EvolutionChoice` carries no base-form restriction, so it is legal on \
             the canonical Quadruped. This slice buys exactly ONE instance, which satisfies the \
             record's real prerequisite at every level by construction; a second instance would \
             require summoner level {IMPROVED_NATURAL_ARMOR_LEVELS_PER_EXTRA} and is not bought \
             here"
        ),
    });

    let total = EIDOLON_RACIAL_NATURAL_ARMOR
        + eidolon_natural_armor_bonus(level)
        + IMPROVED_NATURAL_ARMOR_BONUS;
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.total_natural_armor".to_owned(),
        value: total,
        detail: format!(
            "Quadruped Eidolon total natural armor: +{total} -- the Eidolon race's racial \
             +{EIDOLON_RACIAL_NATURAL_ARMOR}, plus +{} from the level-driven progression, plus \
             +{IMPROVED_NATURAL_ARMOR_BONUS} from the purchased Improved Natural Armor \
             evolution, which the corpus marks `STACK:YES` so it genuinely adds rather than \
             overlapping. This is the record the evolution's magnitude lands ON: the purchase \
             changes a total this engine already computed, rather than sitting beside it",
            eidolon_natural_armor_bonus(level)
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.evolution_points_spent".to_owned(),
        value: spent,
        detail: format!(
            "Quadruped Eidolon evolution points spent: {spent} of the level {level} pool's \
             {pool}, on Improved Natural Armor. The point-buy economy is genuinely honoured for \
             this purchase -- the cost is drawn from the real pool rather than the evolution \
             being granted for free"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.apg.summoner.eidolon.evolution_points_unspent".to_owned(),
        value: unspent,
        detail: format!(
            "Quadruped Eidolon evolution points still unspent: {unspent} of {pool}. Reported \
             rather than hidden: leaving points unspent is a legal PF1 character state, and \
             spending them would require the other {} evolution records this slice does not \
             model. The Quadruped's own automatic Bite and two Legs evolutions are granted \
             AUTOMATIC by the corpus and draw nothing from this pool",
            EIDOLON_EVOLUTION_RECORD_COUNT - 1
        ),
    });

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.apg.summoner.eidolon.evolutions_deferred.unsupported".to_owned(),
        message: format!(
            "{SUMMONER_CLASS_ID} grounds the canonical Quadruped Eidolon's stat block, its \
             evolution-point pool, and ONE genuinely built evolution purchase (Improved Natural \
             Armor, cost {IMPROVED_NATURAL_ARMOR_COST} drawn from that pool, its corpus \
             prerequisite satisfied, magnitude landed \
             on the Eidolon's natural-armor total). Still deferred, and named rather than \
             hidden: the other {} of the corpus's {EIDOLON_EVOLUTION_RECORD_COUNT} \
             `KEY:Evolution ~ *` records (costs 1-4, with their own base-form and level \
             prerequisites), and this Eidolon's {unspent} remaining unspent points, which cannot \
             be allocated without them. Also still ungrounded: {SUMMONER_REMAINING_DEFERRED_FEATURES}. \
             This is a canonical narrowing in the ratified shape -- one corpus-verified member \
             of a large menu genuinely built, the rest named honestly -- not a claim that the \
             evolution economy is modelled. No evolution spending is fabricated",
            EIDOLON_EVOLUTION_RECORD_COUNT - 1
        ),
        claim_blocking: false,
    });
}

/// Pushes the ORIGINAL, claim-blocking `evolutions_deferred` diagnostic.
/// Called only from `ground_or_block_summoner_eidolon_evolutions`'s
/// not-recognized branches -- once an evolution IS recognized and bought,
/// a different, non-blocking version of this same diagnostic id is pushed
/// inline there instead. Mirrors
/// `push_arcanist_exploits_deferred_diagnostic` exactly.
pub(super) fn push_summoner_evolutions_deferred_diagnostic(diagnostics: &mut Vec<ComputationDiagnostic>) {
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.apg.summoner.eidolon.evolutions_deferred.unsupported".to_owned(),
        message: format!(
            "{SUMMONER_CLASS_ID} remains blocked on its Eidolon's unspent evolution points: this \
             slice grounds the canonical Quadruped's fixed stat block and the pool SIZE, but no \
             evolution purchase is recorded, and the full evolution point-buy economy \
             ({EIDOLON_EVOLUTION_RECORD_COUNT} real evolution records with costs 1-4 and their \
             own base-form and level prerequisites) is not modelled, so which evolutions a given \
             Eidolon actually bought is unknown. A recognized \
             {SUMMONER_EIDOLON_EVOLUTION_CHOICE_ID} selection naming \
             '{IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION}' clears this blocker. Still genuinely \
             ungrounded alongside it, per a full `KEY:Summoner ~` / `KEY:Eidolon ~` corpus \
             enumeration: {SUMMONER_REMAINING_DEFERRED_FEATURES}. Bond Senses, Maker's Call, \
             Merge Forms, Twin Eidolon and the Summon Monster spell-like ability's \
             duration/uses/accessible-spell-level are NOT among them -- all five are grounded \
             (task #35). No evolution spending or class-feature execution is fabricated here"
        ),
        claim_blocking: true,
    });
}

/// claim-blocking `hex_powers.unsupported` diagnostic.
/// Grounds or claim-blocks Witch's Hex choice (v0.6 alpha swarm, risks
/// item 8, Witch full-build closure, 11th ACG/APG class-specific
/// closure). A recognized `choice:witch_hex` selection naming
/// `hex:ward` grounds the flat deflection/resistance bonus and replaces
/// the claim-blocking hex-powers diagnostic with a non-blocking note
/// naming the other ~18 hexes as still deferred, mirroring Oracle's own
/// Mystery/Curse three-branch dispatch shape (and Warpriest's Blessing
/// Resolves the character's bonded familiar species from an explicit
/// recorded choice (task #11 Tier 0, 2026-07-27).
///
/// Class-ownership-gated to Witch and Shaman, the two classes whose
/// corpus records set `FamiliarMasterLVL` and delegate to the shared
/// `Standard Familiar List`. Requires an explicit pick and seeds
/// nothing: a familiar chooser's entire value IS which species was
/// bonded, the same no-silent-seeding line ratified for Skill Focus.
///
/// Toad is the one canonical species recognised, chosen because its
/// master benefit is the only one landing on max hit points -- the most
/// load-bearing total in the app -- making this a genuine integration
/// rather than another standalone record.
pub(super) fn bonded_familiar_species(input: &CharacterInput) -> Option<FamiliarSpecies> {
    let owns_familiar_class = input.chosen.class_levels.iter().any(|class_level| {
        class_level.class_id == WITCH_CLASS_ID
            || class_level.class_id == SHAMAN_CLASS_ID
            || class_level.class_id == ARCANIST_CLASS_ID
    });
    if !owns_familiar_class {
        return None;
    }
    input
        .chosen
        .selected_choices
        .iter()
        .any(|c| {
            c.choice_set_id == FAMILIAR_CHOICE_ID && c.selection_id == FAMILIAR_TOAD_SELECTION
        })
        .then_some(FamiliarSpecies::Toad)
}

/// The familiar's flat hit-point contribution for this character, ready
/// to layer onto a real max-HP total at its consumer.
pub fn character_familiar_hp_bonus(input: &CharacterInput) -> i16 {
    crate::rules_core::durability::familiar_master_hp_bonus(bonded_familiar_species(input))
}

/// Grounds the familiar's master benefit (task #11 Tier 0, 2026-07-27).
///
/// The design pass's own first-look prediction was that a familiar would
/// need a Tier-2 creature-stat-block pillar, on the reasoning that a
/// familiar has its own stat block so its properties are genuine inputs.
/// For the part that reaches the character sheet that is false: the
/// master benefit is a flat magnitude on the MASTER that reads nothing
/// from the familiar creature at all. The creature's own stat block is
/// genuinely Tier-2-shaped, and stays deferred with the Eidolon MVP
/// recorded as its template.
pub(super) fn ground_familiar_master_benefit(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(species) = bonded_familiar_species(input) else {
        return;
    };
    let hp_bonus = crate::rules_core::durability::familiar_master_hp_bonus(Some(species));
    explanations.push(ComputationExplanation {
        id: "class_feature.familiar.master_hit_point_bonus".to_owned(),
        value: hp_bonus,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:HP|CURRENTMAX|FamiliarGrantedBonus_3`
            "Bonded Toad familiar: its master gains +{hp_bonus} maximum hit points. INTEGRATED into \
             the real computed max-HP total rather than grounded as a standalone record -- it is \
             layered on as a per-character add-on at the same consumer that already applies \
             Toughness's feat bonus, not folded into the class hit-die table. The familiar's own \
             creature stat block (its hit dice, saves, skills and Intelligence, all scaling off \
             master level {level}) is deferred. The corpus states the benefit applies only while \
             master and familiar are within a mile of each other; the cancelling setters for that \
             condition live in books this repo does not ingest, so within this scope the bonus is \
             unconditional"
        ),
    });
}

/// The Witch's hex save DC: `10 + WitchHexStat + (WitchHexAbilityLVL/2)`,
/// where the corpus sets `WitchHexStat = INT` and
/// `WitchHexAbilityLVL = WitchLVL` (task #11, 2026-07-27).
///
/// **One formula covers every hex.** 51 of the corpus's 53 base hex
/// records carry only `BONUS:VAR|WitchHexDC_<Name>|WitchHexDC`, a
/// per-hex alias of this single variable. That makes them facets of one
/// DC mechanism, not 51 magnitudes, which is why grounding the DC once
/// covers nearly the whole hex list rather than needing a canonical pick.
///
/// **Corpus-count correction (canonical-narrowing pass).** This doc and
/// every sibling message used to say "all 27 hex records -- 14 minor, 8
/// major, 5 grand". That is the Advanced Player's Guide's own set alone
/// (14+8+5=27), presented as if it were the whole corpus. Re-derived
/// across every `.lst` in the corpus, the real base-class hex list is
/// **53** records -- 29 minor, 16 major, 8 grand -- spanning APG,
/// Ultimate Magic, Ultimate Wilderness and Monster Codex. That total
/// already excludes 6 archetype-locked records (5 gated on `Witch
/// Archetype ~ Mountain Witch`, plus Bouda's Eye, granted only inside
/// `Witch Archetype ~ Bouda`) and `Witch Hex ~ Hair`, which shares the
/// KEY prefix but is `CATEGORY:Natural Attack` -- the auto-granted
/// natural-weapon sub-record of the real Prehensile Hair hex, not a
/// selectable hex.
///
/// **The "100% magnitude-bearing" claim is false.** A prior scoping pass
/// recorded Witch's hexes as the only 100%-magnitude-bearing list on the
/// roster. Re-checked directly: 51 of 53 carry at least one `BONUS:`
/// token, so the real ratio is 51/53 (~96.2%). `Witch Hex ~ City Sight`
/// and `Witch Hex ~ Summer's Heat` (both
/// `ultimate_wilderness/uw_abilities_class.lst`) carry no `BONUS:` token
/// at all -- no `WitchHexDC_<Name>` alias, no Ability Focus hook, `DESC:`
/// only. They are the two hexes this shared DC genuinely does NOT cover.
///
/// The corpus also carries `+2` variants gated on the
/// `Ability Focus (Witch Hex)` feat and per-hex Ability Focus feats.
/// Those are feat-conditional rather than class-derived and are not
/// folded in here; a character who takes one would need that feat
/// recognised, which this closure does not do.
pub(super) fn witch_hex_save_dc(level: u8, intelligence_modifier: i16) -> i16 {
    10 + intelligence_modifier + i16::from(level) / 2
}

/// Whether this Witch took the named hex. Class-ownership-gated, and
/// requires an explicit recorded pick -- a hex chooser's entire value IS
/// which hex was taken, the same no-silent-seeding line ratified for
/// Skill Focus.
pub(super) fn witch_has_hex(input: &CharacterInput, hex: &str) -> bool {
    input
        .chosen
        .class_levels
        .iter()
        .any(|class_level| class_level.class_id == WITCH_CLASS_ID)
        && input
            .chosen
            .selected_choices
            .iter()
            .any(|c| c.choice_set_id == WITCH_HEX_CHOICE_ID && c.selection_id == hex)
}

/// The Flight hex's contribution to its Witch's computed Swim total.
///
/// Flight is the standout of the 53 hexes: `BONUS:SKILL|Swim|4` lands on
/// a skill this engine actually computes, so it integrates into the real
/// total rather than grounding as another standalone record. Returns 0
/// for every non-Witch and every Witch who did not take it.
pub(super) fn witch_flight_hex_swim_bonus(input: &CharacterInput) -> i16 {
    if witch_has_hex(input, FLIGHT_HEX_SELECTION) {
        WITCH_FLIGHT_SWIM_BONUS
    } else {
        0
    }
}

/// shape before that). An unrecognized or missing choice keeps a
/// The APG Witch class table's BASE spells-per-day row, one entry per
/// spell level 0-9 (`None` for an inaccessible "—" column; index 0 is
/// cantrips). Transcribed from `apg_classes.lst`'s own 20 `CAST:` rows,
/// which are **byte-identical to Cleric's across all 20 levels** --
/// verified by direct diff, including the level-11 row where Shaman
/// alone diverges (risks item 56).
pub(super) fn witch_base_spells_per_day_table(level: u8) -> [Option<i16>; 10] {
    // Identical progression to Cleric's own corpus rows; see
    // `cleric_base_spells_per_day_table` for the row-by-row citation.
    cleric_base_spells_per_day_table(level)
}

/// Witch's highest accessible spell level, derived from the slot table
/// itself so the ceiling and the budget cannot drift apart (same
/// single-source-of-truth shape as `shaman_spell_level_access`).
pub(super) fn witch_spell_level_access(level: u8) -> i16 {
    witch_base_spells_per_day_table(level)
        .iter()
        .rposition(Option::is_some)
        .map_or(0, |index| index as i16)
}

/// The real per-day slot budget per spell level 0-9 (base + Intelligence
/// bonus spells). Cantrips take no bonus, matching every other prepared
/// caster here.
pub(super) fn witch_total_spells_per_day(level: u8, intelligence_modifier: i16) -> [Option<i16>; 10] {
    let base = witch_base_spells_per_day_table(level);
    let mut total = [None; 10];
    for (spell_level, base_count) in base.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let bonus = if spell_level == 0 {
            0
        } else {
            ability_bonus_spells(intelligence_modifier, spell_level as i16)
        };
        total[spell_level] = Some(base_count + bonus);
    }
    total
}

/// Whether a Witch spell level must be backed by a spell stored in the
/// familiar before it can be prepared.
///
/// **This is the token that makes Witch a third caster shape**, not a
/// Cleric variant. Compare the real class lines:
///
/// ```text
/// Cleric  SPELLSTAT:WIS  KNOWNSPELLS:LEVEL=0|LEVEL=1|...|LEVEL=9
/// Wizard  SPELLSTAT:INT  SPELLBOOK:YES
/// Witch   SPELLSTAT:INT  KNOWNSPELLS:LEVEL=0
/// ```
///
/// Cleric/Druid/Shaman automatically know every level and prepare
/// straight off the class list. Wizard is spellbook-gated. Witch
/// automatically knows **only cantrips** -- everything above level 0
/// lives in her familiar, exactly as PF1 RAW describes. So level 0 needs
/// no stored record and every higher level does.
pub(super) fn witch_spell_level_requires_familiar_storage(spell_level: u8) -> bool {
    spell_level > 0
}

/// Return the list of unmet conditions for Witch's real prepared-spell
/// posture.
///
/// Structurally this is Alchemist's/Investigator's two-step
/// `Known`-backs-`Prepared` shape rather than Cleric's one-step, because
/// the Witch genuinely has a spell STORE: her familiar. `Known` models
/// spells stored in the familiar; `Prepared` models today's preparation
/// drawn from it. The one real deviation from the Alchemist shape is
/// `witch_spell_level_requires_familiar_storage`: cantrips are
/// automatically known and need no stored record.
///
/// **No familiar stat block is needed for any of this**, and that is the
/// load-bearing insight rather than a convenience: this repo's Wizard
/// "spellbook" was never a modeled object either -- it is just
/// `spells_selected` filtered by `AcquisitionMode`. So "the familiar
/// stores the spells" (which is what `KEY:Witch ~ Familiar`'s own DESC
/// says) is representable with exactly the machinery already present,
/// and the Familiar subsystem's Tier 2 is NOT a prerequisite. Witch is a
/// hybrid of two shapes this codebase already has -- Cleric-shaped at
/// level 0, Wizard-shaped at levels 1-9 -- not a third mechanism.
/// Design: scout's `witch-spellcasting-shape-design.md`; ruling: team
/// lead, `risks-and-open-questions.md` item 57.
///
/// Deliberately NOT modeled here (and still named in the deferred
/// diagnostic): losing access when the familiar is dead or absent, and
/// the familiar's own creature stat block (the Familiar subsystem's
/// Tier 2). This grounds which spells a Witch may prepare and how many;
/// it does not simulate the familiar as an object that can go missing.
pub(super) fn unmet_witch_prepared_spell_conditions(
    input: &CharacterInput,
    witch_level: u8,
    intelligence_modifier: i16,
) -> Vec<String> {
    let mut unmet = Vec::new();

    let witch_spells = |mode: AcquisitionMode| -> Vec<&str> {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(|s| s.source_class_id == WITCH_CLASS_ID && s.acquisition_mode == mode)
            .map(|s| s.spell_id.as_str())
            .collect()
    };
    let stored = witch_spells(AcquisitionMode::Known);
    let prepared = witch_spells(AcquisitionMode::Prepared);

    let access_ceiling = witch_spell_level_access(witch_level);
    let total_per_day = witch_total_spells_per_day(witch_level, intelligence_modifier);

    for spell_id in &stored {
        if witch_spell_list::witch_spell_level(spell_id).is_none() {
            unmet.push(format!(
                "spell '{spell_id}' stored in the familiar is not on the real PF1 witch spell \
                 list"
            ));
        }
    }

    let mut consumed_per_level: [i16; 10] = [0; 10];
    for spell_id in &prepared {
        let Some(spell_level) = witch_spell_list::witch_spell_level(spell_id) else {
            unmet.push(format!(
                "prepared spell '{spell_id}' is not on the real PF1 witch spell list"
            ));
            continue;
        };
        if spell_level > 0 && i16::from(spell_level) > access_ceiling {
            unmet.push(format!(
                "prepared spell '{spell_id}' targets spell level {spell_level}, not yet \
                 accessible at witch level {witch_level} (access ceiling {access_ceiling})"
            ));
            continue;
        }
        if witch_spell_level_requires_familiar_storage(spell_level)
            && !stored.contains(spell_id)
        {
            unmet.push(format!(
                "prepared spell '{spell_id}' (level {spell_level}) is not stored in the \
                 witch's familiar -- only cantrips are known automatically"
            ));
            continue;
        }
        consumed_per_level[usize::from(spell_level)] += 1;
    }

    for (spell_level, consumed) in consumed_per_level.iter().enumerate() {
        if *consumed == 0 {
            continue;
        }
        let total_slots = total_per_day[spell_level].unwrap_or(0);
        if *consumed > total_slots {
            unmet.push(format!(
                "spell level {spell_level} over-prepared: {consumed} spells prepared but only \
                 {total_slots} slots available (base + Intelligence bonus)"
            ));
        }
    }

    unmet
}

/// Ground Witch's real prepared-spell posture once
/// `unmet_witch_prepared_spell_conditions` reports an empty unmet list.
pub(super) fn ground_witch_prepared_spells(
    input: &CharacterInput,
    witch_level: u8,
    intelligence_modifier: i16,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == WITCH_CLASS_ID && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();
    let stored_count = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == WITCH_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known
        })
        .count();

    explanations.push(ComputationExplanation {
        id: "class_spell.apg.witch.familiar_stored_spells".to_owned(),
        value: stored_count as i16,
        detail: format!(
            "Witch level {witch_level} spells stored in her familiar ({stored_count} spells). \
             MODELING CHOICE, stated rather than implied: this engine represents the familiar's \
             stored spells as `AcquisitionMode::Known` entries in `spells_selected`. The corpus \
             does NOT encode that mapping -- it is the same representation this codebase already \
             uses for a Wizard's spellbook and an Alchemist's formula book, neither of which is \
             a modeled object either. What IS corpus-encoded is the requirement itself: unlike \
             Cleric/Druid/Shaman, whose class lines carry the full \
             `KNOWNSPELLS:LEVEL=0|...|LEVEL=9` ladder and who therefore know every level \
             automatically, the Witch's own line carries `KNOWNSPELLS:LEVEL=0` alone, so only \
             cantrips are known automatically and every higher-level spell must be recorded \
             first -- which `KEY:Witch ~ Familiar`'s own DESC attributes to the familiar. This \
             grounds the store's contents; it does not model the familiar being lost or slain, \
             and no familiar creature stat block is required for any of it"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_spell.apg.witch.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Witch level {witch_level} daily preparation selection ({} spells, \
             AcquisitionMode::Prepared): {}. Each prepared spell is verified against the real \
             PF1 witch spell list (`witch_spell_list::WITCH_SPELL_LIST`, 326 records across the \
             ingested books), the witch's own spell-level access ceiling, the per-level slot \
             budget (base + Intelligence bonus), and -- for every spell above cantrip level -- \
             that it is actually stored in her familiar. This grounds the prepared-spell \
             selection for real; it computes no spell save DC resolution against a target and \
             no casting execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let total_per_day = witch_total_spells_per_day(witch_level, intelligence_modifier);
    for (spell_level, total) in total_per_day.iter().enumerate() {
        let Some(total) = total else {
            continue;
        };
        explanations.push(ComputationExplanation {
            id: format!("class_spell.apg.witch.total_spells_per_day.spell_level_{spell_level}"),
            value: *total,
            detail: format!(
                "Witch level {witch_level} total spells per day at spell level {spell_level}: \
                 {total} (base table count + Intelligence bonus; cantrips take no bonus). The \
                 witch's own `CAST:` rows are byte-identical to Cleric's across all 20 levels"
            ),
        });
    }
}

pub(super) fn ground_or_block_witch_class_features(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    ground_familiar_master_benefit(input, level, explanations);

    let intelligence_modifier = ability_modifier(input.chosen.ability_scores.intelligence);
    let unmet_spells =
        unmet_witch_prepared_spell_conditions(input, level, intelligence_modifier);
    if unmet_spells.is_empty() {
        ground_witch_prepared_spells(input, level, intelligence_modifier, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.apg.witch.prepared_spells.unsupported".to_owned(),
            message: format!(
                "{WITCH_CLASS_ID} prepared-spell posture is not satisfied: {}",
                unmet_spells.join("; ")
            ),
            claim_blocking: true,
        });
    }

    let hex_dc = witch_hex_save_dc(level, ability_modifier(input.chosen.ability_scores.intelligence));
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.witch.hex_save_dc".to_owned(),
        value: hex_dc,
        detail: format!(
            "Witch level {level} hex save DC: 10 + Intelligence modifier + level/2 = {hex_dc}. \
             ONE formula covers every hex -- 51 of the corpus's 53 base records carry \
             only a per-hex alias of this single shared variable, so they are facets of one DC \
             mechanism rather than 51 separate magnitudes. Corpus-count correction: this said \
             \"all 27 records (14 minor, 8 major, 5 grand)\", which is the APG's own set alone \
             presented as the whole corpus; re-derived across every `.lst`, the base-class list \
             is 53 (29 minor, 16 major, 8 grand) across APG, Ultimate Magic, Ultimate \
             Wilderness and Monster Codex. `City Sight` and `Summer's Heat` are the two \
             carrying no DC alias at all, so this formula covers 51 of the 53, not every one. \
             The corpus's +2 Ability Focus variants are feat-conditional rather than \
             class-derived and are not folded in"
        ),
    });

    if witch_has_hex(input, CAULDRON_HEX_SELECTION) {
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.witch.cauldron_hex.craft_alchemy_bonus".to_owned(),
            value: WITCH_CAULDRON_CRAFT_ALCHEMY_BONUS,
            detail: format!(
                "Witch level {level} took the Cauldron hex: a \
                 +{WITCH_CAULDRON_CRAFT_ALCHEMY_BONUS} insight bonus on Craft (Alchemy) checks. \
                 Craft is not among the three skills this engine computes, so this grounds \
                 standalone. Cauldron and Flight are the only two hexes carrying a magnitude \
                 distinct from the shared hex DC"
            ),
        });
    }

    if witch_has_hex(input, FLIGHT_HEX_SELECTION) {
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.witch.flight_hex.swim_bonus".to_owned(),
            value: WITCH_FLIGHT_SWIM_BONUS,
            detail: format!(
                "Witch level {level} took the Flight hex: a +{WITCH_FLIGHT_SWIM_BONUS} bonus on \
                 Swim checks. INTEGRATED into the real `skill.selected_modifier.swim` total -- \
                 Swim is one of the three skills this engine actually computes, making Flight \
                 the only one of the 53 hexes whose magnitude reaches a live total. The hex's \
                 own later-level flight benefits (feather fall, levitate, fly) are spell-effect \
                 wrappers with no independent magnitude and are not grounded"
            ),
        });
    }

    let hex_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == WITCH_HEX_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();

    // The canonical-narrowing posture is satisfied by any of the three
    // hexes whose own magnitude this codebase genuinely grounds. Keep
    // this list in step with the three grounding branches above -- an
    // ungrounded hex id must NOT satisfy it (pinned by
    // `an_ungrounded_witch_hex_selection_still_claim_blocks`).
    let hex_recognized = hex_selections.contains(&WARD_HEX_SELECTION)
        || hex_selections.contains(&CAULDRON_HEX_SELECTION)
        || hex_selections.contains(&FLIGHT_HEX_SELECTION);

    if hex_selections.contains(&WARD_HEX_SELECTION) {
        let ward_bonus = witch_ward_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.witch.ward_hex.deflection_and_resistance_bonus".to_owned(),
            value: ward_bonus,
            detail: format!(
                "Witch level {level} with the Ward hex grants a warded creature a \
                 +{ward_bonus} deflection bonus to AC and a +{ward_bonus} resistance bonus on \
                 saving throws (base 2, +1 at level 8, +1 at level 16). Grounds only the flat \
                 magnitude -- task #88 correction: `defense.baseline_armor_class` and the total \
                 saves ARE real integrated totals this codebase computes (the same totals \
                 Barbarian Rage's penalty and Oracle's Nature's Whispers bonus already \
                 integrate into), this magnitude is simply not wired into either one yet; \
                 self-application (the witch may ward herself) is the only case named, warding \
                 another creature is not modeled"
            ),
        });
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.witch.hex_powers_beyond_ward.unmodeled".to_owned(),
            message: "Witch Hex content beyond Ward remains unmodeled: the other 50 base hexes \
                 (Evil Eye, Misfortune, Slumber, Cackle, Fortune, Healing, and the rest, plus \
                 the separate Major Hex/Grand Hex tiers) are not implemented, beyond Cauldron's \
                 and Flight's own grounded magnitudes and the shared save DC. Corpus-count \
                 correction: this said \"the other ~18 base hexes\", an APG-only figure; the \
                 real base-class list is 53 records across four books. This does not \
                 block an otherwise-valid Ward posture"
                .to_owned(),
            claim_blocking: false,
        });
    } else if hex_selections.contains(&CAULDRON_HEX_SELECTION)
        || hex_selections.contains(&FLIGHT_HEX_SELECTION)
    {
        // Staleness fix (canonical-narrowing pass): this branch did not
        // exist, so `hex_powers.unsupported` -- whose own message names
        // all THREE grounded hexes -- still fired for a Witch who took
        // Cauldron or Flight, telling her "no recognized hex choice is
        // present" while the engine was simultaneously grounding her
        // hex's real magnitude above. Ward keeps its own dedicated note
        // only because its message enumerates the specific hexes Ward
        // leaves behind; the substance of the two notes is the same.
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.witch.hex_powers_beyond_base.unmodeled".to_owned(),
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   Cauldron's Craft (Alchemy) insight bonus and Flight's Swim bonus are the two
            //   non-Ward magnitudes this codebase grounds; the other 50 of the corpus's 53 base
            //   hexes (29 `Witch Hex`, 16 `Witch Major Hex`, 8 `Witch Grand Hex`) carry no modeled
            //   magnitude beyond that shared DC -- which itself misses `City Sight` and `Summer's
            //   Heat`, the two records carrying no `BONUS:` token at all -- and Flight's own
            //   later-level feather fall/levitate/fly benefits are spell-effect wrappers with no
            //   independent magnitude.
            message: "Only the selected hex's own grounded magnitude is modeled, alongside the \
                 shared hex save DC that covers 51 of the 53 records. This does not block an \
                 otherwise-valid hex posture"
                .to_owned(),
            claim_blocking: false,
        });
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.witch.hex_powers.unsupported".to_owned(),
            message: "Witch remains blocked on its Hex powers burden: no recognized hex choice \
                 is present. Three of the corpus's 53 base hexes have their own grounded \
                 magnitude (Ward's deflection/resistance, Cauldron's Craft (alchemy) bonus, \
                 Flight's Swim bonus) and the shared hex save DC grounds for 51 of the 53, but \
                 each requires an explicit recorded choice; nothing is seeded here. This \
                 message previously said \"only Ward's own deflection/resistance bonus is \
                 genuinely grounded\", which stopped being true once Cauldron and Flight landed \
                 (task #76), and counted the hex list as 27 -- the APG's own set alone, \
                 corrected to the full-corpus 53 by the canonical-narrowing pass"
                .to_owned(),
            claim_blocking: true,
        });
    }

    push_witch_other_features_deferred_diagnostic(diagnostics, hex_recognized);
}

/// Pushes the narrower diagnostic replacing
/// `class_feature.apg.witch.unsupported` for Witch specifically (v0.6
/// alpha swarm, risks item 8, Witch full-build closure): names ONLY the
/// genuinely still-missing pieces.
///
/// **Canonical-narrowing pass**: this used to be pushed unconditionally
/// claim-blocking, which meant no Witch could ever reach `Computed` no
/// matter what she chose. It now carries the SAME id and the SAME named
/// gaps in both branches -- only `claim_blocking` differs -- exactly the
/// shape Arcanist's own `exploits_deferred` established when it closed
/// (`ground_or_block_arcanist_metamagic_knowledge`). The distinction is
/// deliberate and narrow: once one corpus-verified hex is genuinely
/// grounded, what remains is *breadth* (24 more hexes, Patron Spells,
/// the familiar's own creature stat block) rather than a correctness
/// hole -- every number the receipt reports for the recorded posture is
/// right. With no recognized hex at all, the class's single defining
/// chooser is unanswered, and that IS claim-blocking.
///
/// Neither branch drops a named gap. `hex_recognized` is true only for
/// the three hexes whose own magnitude is grounded, never for an
/// arbitrary selection string.
pub(super) fn push_witch_other_features_deferred_diagnostic(
    diagnostics: &mut Vec<ComputationDiagnostic>,
    hex_recognized: bool,
) {
    let posture = if hex_recognized {
        "and the recognized hex's own grounded magnitude. Genuinely still deferred, and NOT \
         claim-blocking now that the class's defining chooser is answered by a corpus-verified \
         pick"
    } else {
        "and the Ward, Cauldron and Flight hexes' own magnitudes (none of which is recognized \
         here). Claim-blocking, because no recognized hex choice is present at all. Also still \
         deferred"
    };
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.apg.witch.other_features_deferred.unsupported".to_owned(),
        message: format!(
            "{WITCH_CLASS_ID} is grounded for its base-attack-bonus/base-save chassis \
             pillar, its class-skill list (Intimidate only -- a genuine partial match), its \
             own-list spellcasting with daily preparation and prepared-spell validation, \
             Familiar Touch Spells, the bonded familiar's own master hit-point benefit, the \
             single shared hex save DC covering 51 of its 53 hex records, {posture}: Patron \
             Spells, the bonded familiar's own creature stat block, and the other 50 of the \
             corpus's 53 base hexes (29 `Witch Hex`, 16 `Witch Major Hex`, 8 `Witch Grand Hex`) \
             remain ungrounded \
             anywhere in this codebase; no class-feature or spell execution is fabricated in \
             this bounded chassis baseline. This message previously listed own-list \
             spellcasting (\"Cantrips, Patron Spells -- no `SPELLLIST:` reuse token, a \
             genuinely new data-ingestion cost\") and \"the Familiar and Familiar Touch Spells \
             (an unbuilt subsystem)\" as wholly ungrounded: both were true when written and are \
             false now (tasks #11/#23/#33), with only Patron Spells and the familiar creature's \
             own stat block genuinely remaining (task #76)"
        ),
        claim_blocking: !hex_recognized,
    });
}

/// PF1 Advanced Class Guide Arcane Reservoir: "Each day, when preparing
/// spells, your arcane reservoir fills... gaining a number of points
/// equal to [3 + 1/2 arcanist level]." Verified directly against
/// `acg_abilities_class.lst`'s own `BONUS:VAR|ArcanistReservoirSize|
/// 3+ArcanistLVL/2`. A flat, choice-free daily resource -- unlike every
/// activation-gated closure this session, this needs no
/// `class_ability_activations` entry at all, mirroring Cleric's own
/// `channel_energy_dice`/`channel_energy_uses_per_day`'s "flat, no gate"
/// shape.
pub(super) fn arcanist_reservoir_daily_fill(level: u8) -> i16 {
    3 + i16::from(level) / 2
}

/// PF1 Advanced Class Guide Arcane Reservoir: "The arcane reservoir can
/// hold a maximum of [3 + arcanist level] points." Verified directly
/// against `acg_abilities_class.lst`'s own `BONUS:VAR|
/// MaxArcanistReservoirSize|3+ArcanistLVL`.
///
/// The `Extra Reservoir` feat adds `3` to **this** variable and only
/// this one, which settles a genuine ambiguity in its own prose: "You
/// gain three more points in your arcane reservoir, AND the maximum
/// number of points ... increases by that amount" reads like both the
/// daily fill and the cap rise. The corpus separates them cleanly --
/// `ArcanistReservoirSize` (fill) and `MaxArcanistReservoirSize` (cap)
/// are two distinct variables on the same `Arcanist ~ Arcane Reservoir`
/// record, matching this engine's own two functions one-for-one -- and
/// `acg_feats.lst`'s Extra Reservoir record names only the cap. So
/// [`arcanist_reservoir_daily_fill`] deliberately does NOT take the
/// feat; pinned by `extra_reservoir_raises_only_the_cap_never_the_
/// daily_fill`.
pub(super) fn arcanist_reservoir_max(level: u8, selected_feats: &[String]) -> i16 {
    3 + i16::from(level)
        + extra_resource_feat_bonus(selected_feats, EXTRA_RESERVOIR_FEAT_KEY, EXTRA_RESERVOIR_POINTS)
}

/// PF1 Advanced Class Guide Arcanist "Spells Prepared" table, base counts
/// before any Intelligence bonus spells, one entry per spell level 0
/// (cantrip) through 9 (`None` for an inaccessible "--" column).
///
/// Unlike Wizard's/Warpriest's own tables, the Arcanist's per-level counts
/// are NOT literal `CAST:` rows in the corpus. Its `CLASS:Arcanist` level-
/// progression line is a single `REPEATLEVEL` row of variable references
/// (`acg_classes.lst:31`:
/// `1:REPEATLEVEL:1 CAST:ArcanistPreparedLVL_0,...,ArcanistPreparedLVL_9`),
/// and those variables are defined by `BONUS:VAR` formulas on the
/// `KEY:Arcanist ~ Spells Prepared` record at
/// `data/pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_abilities_class.lst`
/// line 77. This table is those formulas evaluated at each level:
///
/// - `ArcanistPreparedLVL_0 = 4+(L>=2)+(L>=4)+(L>=6)+(L>=8)+(L>=10)`
/// - `ArcanistPreparedLVL_1 = 2+(L>=3)+(L>=5)+(L>=7)`
/// - `ArcanistPreparedLVL_2 = (L>=4)+(L>=5)+(L>=7)+(L>=9)+(L>=11)`
/// - `ArcanistPreparedLVL_3 = (L>=6)+(L>=7)+(L>=9)+(L>=11)`
/// - `ArcanistPreparedLVL_4 = (L>=8)+(L>=9)+(L>=11)+(L>=13)`
/// - `ArcanistPreparedLVL_5 = (L>=10)+(L>=11)+(L>=13)+(L>=15)`
/// - `ArcanistPreparedLVL_6 = (L>=12)+(L>=13)+(L>=15)`
/// - `ArcanistPreparedLVL_7 = (L>=14)+(L>=15)+(L>=17)`
/// - `ArcanistPreparedLVL_8 = (L>=16)+(L>=17)+(L>=19)`
/// - `ArcanistPreparedLVL_9 = (L>=18)+(L>=19)+(L>=20)`
///
/// where each `(L>=N)` term is 1 when true and 0 when false, and `L` is
/// `ArcanistCastingLVL` -- defined on the same record as
/// `ArcanistLVL+var("BL=Arcanist")`, i.e. the plain class level for a
/// single-class character with no caster-level bonus. A formula result of
/// 0 means the spell level is not yet accessible, so it maps to `None`.
///
/// Deliberately NOT read off the same record's `ArcanistCastLVL_*`
/// variables, which are a different quantity despite the near-identical
/// name: those are the DESC display helpers for spell slots per day, are
/// gated on `INTSCORE` and fold the Intelligence bonus spells inline,
/// whereas this table is the Intelligence-independent base that
/// `ability_bonus_spells` is added to separately by the caller.
///
/// The levels 1-3 rows are byte-for-byte the ones this table already
/// shipped with, previously cross-checked against legacy.aonprd.com's own
/// printed "Table: Arcanist Spells Prepared" (`4/2/-/-`, `5/2/-/-`,
/// `5/3/-/-`). Genuinely DIFFERENT from `wizard_base_spells_per_day`
/// (level 1 Wizard is `3/1/-/-`; Arcanist's 2nd-level spells first become
/// accessible at Arcanist level 4, not Wizard's level 3).
///
/// Returns an all-`None` row outside the legal 1-20 class-level range.
pub(super) fn arcanist_base_spells_per_day(level: u8) -> [Option<i16>; 10] {
    match level {
        1 => [Some(4), Some(2), None, None, None, None, None, None, None, None],
        2 => [Some(5), Some(2), None, None, None, None, None, None, None, None],
        3 => [Some(5), Some(3), None, None, None, None, None, None, None, None],
        4 => [Some(6), Some(3), Some(1), None, None, None, None, None, None, None],
        5 => [Some(6), Some(4), Some(2), None, None, None, None, None, None, None],
        6 => [Some(7), Some(4), Some(2), Some(1), None, None, None, None, None, None],
        7 => [Some(7), Some(5), Some(3), Some(2), None, None, None, None, None, None],
        8 => [Some(8), Some(5), Some(3), Some(2), Some(1), None, None, None, None, None],
        9 => [Some(8), Some(5), Some(4), Some(3), Some(2), None, None, None, None, None],
        10 => [Some(9), Some(5), Some(4), Some(3), Some(2), Some(1), None, None, None, None],
        11 => [Some(9), Some(5), Some(5), Some(4), Some(3), Some(2), None, None, None, None],
        12 => [Some(9), Some(5), Some(5), Some(4), Some(3), Some(2), Some(1), None, None, None],
        13 => [Some(9), Some(5), Some(5), Some(4), Some(4), Some(3), Some(2), None, None, None],
        14 => [Some(9), Some(5), Some(5), Some(4), Some(4), Some(3), Some(2), Some(1), None, None],
        15 => [Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(2), None, None],
        16 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(2), Some(1), None,
        ],
        17 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None,
        ],
        18 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2),
            Some(1),
        ],
        19 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(3),
            Some(2),
        ],
        20 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(3),
            Some(3),
        ],
        _ => [None, None, None, None, None, None, None, None, None, None],
    }
}

/// Return the list of unmet conditions for this grounding's bounded
/// prepared-spellbook posture. An empty list means the posture is fully
/// supported: an Arcanist at a supported level, with at least one spell
/// recorded (`AcquisitionMode::Known`) and at least one prepared today
/// (`AcquisitionMode::Prepared`), every prepared spell already recorded,
/// and no spell level's prepared count exceeding that level's total slot
/// budget (base table count + the Intelligence bonus). Mirrors
/// `unmet_wizard_spellbook_conditions`'s own shape MINUS the school-
/// specialization check and the opposed-school slot-cost multiplier --
/// Arcanist has no arcane-school mechanic at all (verified directly
/// against its own `KEY:Arcanist ~ ...` list: no "School" record exists),
/// so every prepared spell costs exactly 1 slot, never 2.
pub(super) fn unmet_arcanist_spellbook_conditions(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Vec<String> {
    let mut unmet = Vec::new();

    if level > ARCANIST_SPELLBOOK_SUPPORTED_MAX_LEVEL {
        unmet.push(format!(
            "prepared spellbook grounding is only supported for arcanist levels \
             1-{ARCANIST_SPELLBOOK_SUPPORTED_MAX_LEVEL}, got {level}"
        ));
        return unmet;
    }

    let arcanist_spells = |mode: AcquisitionMode| {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(move |s| s.source_class_id == ARCANIST_CLASS_ID && s.acquisition_mode == mode)
    };
    let recorded: Vec<&str> = arcanist_spells(AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();
    let prepared: Vec<&str> = arcanist_spells(AcquisitionMode::Prepared)
        .map(|s| s.spell_id.as_str())
        .collect();

    if recorded.is_empty() {
        unmet.push(
            "no arcanist spells recorded in the spellbook (AcquisitionMode::Known)".to_owned(),
        );
    }
    if prepared.is_empty() {
        unmet.push("no arcanist spells prepared today (AcquisitionMode::Prepared)".to_owned());
    }

    for spell_id in &prepared {
        if !recorded.contains(spell_id) {
            unmet.push(format!(
                "prepared spell '{spell_id}' is not recorded in the spellbook"
            ));
        }
    }

    // Same up-front per-class resolution as `unmet_wizard_spellbook_conditions`
    // — see `resolve_prepared_spell_level`. Arcanist's corpus-stated
    // `SPELLLIST:1|Wizard` means it resolves against Wizard's list.
    let mut prepared_levels: Vec<u8> = Vec::new();
    for spell_id in &prepared {
        match resolve_prepared_spell_level(ARCANIST_CLASS_ID, spell_id) {
            PreparedSpellLevel::Known(spell_level) => prepared_levels.push(spell_level),
            PreparedSpellLevel::Unknown(reason) => unmet.push(reason),
        }
    }

    let base_spells_per_day = arcanist_base_spells_per_day(level);
    for (spell_level, base_count) in base_spells_per_day.iter().enumerate() {
        let spell_level = spell_level as u8;
        let Some(base_count) = base_count else {
            if prepared_levels.contains(&spell_level) {
                unmet.push(format!(
                    "a prepared spell targets spell level {spell_level}, not yet accessible at \
                     arcanist level {level}"
                ));
            }
            continue;
        };
        let int_bonus =
            ability_bonus_spells(ability_modifiers.intelligence, i16::from(spell_level));
        let total_slots = base_count + int_bonus;
        let consumed: i16 = prepared_levels.iter().filter(|l| **l == spell_level).count() as i16;
        if consumed > total_slots {
            unmet.push(format!(
                "spell level {spell_level} over-prepared: {consumed} spells prepared but only \
                 {total_slots} slots available (base {base_count} + Intelligence bonus \
                 {int_bonus})"
            ));
        }
    }

    unmet
}

/// Ground the real prepared-spellbook / daily-preparation state once
/// `unmet_arcanist_spellbook_conditions` reports an empty unmet list:
/// the recorded spellbook contents, the daily preparation selection, and
/// the base/Intelligence-bonus/total spells-per-day counts per accessible
/// spell level. Mirrors `ground_wizard_prepared_spellbook`'s own shape
/// minus the specialist-bonus-slot term (Arcanist has none).
pub(super) fn ground_arcanist_prepared_spellbook(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let arcanist_spells = |mode: AcquisitionMode| {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(move |s| s.source_class_id == ARCANIST_CLASS_ID && s.acquisition_mode == mode)
    };
    let recorded: Vec<&str> = arcanist_spells(AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();
    let prepared: Vec<&str> = arcanist_spells(AcquisitionMode::Prepared)
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.acg.arcanist.spellbook_contents".to_owned(),
        value: recorded.len() as i16,
        detail: format!(
            "Arcanist level {level} recorded spellbook contents ({} spells, \
             AcquisitionMode::Known): {}. This grounds which spells are recorded as real, \
             chosen input; it does not verify against any corpus that a named spell genuinely \
             exists or genuinely belongs to the level its own identifier claims",
            recorded.len(),
            recorded.join(", ")
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_spell.acg.arcanist.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Arcanist level {level} daily preparation selection ({} spells, \
             AcquisitionMode::Prepared, each already verified recorded in the spellbook above): \
             {}. This grounds the prepared-vs-known distinction for real: every prepared spell \
             is drawn from the recorded spellbook, consuming its spell level's slot budget (one \
             slot each -- Arcanist has no opposed-school double-cost rule, unlike Wizard). It \
             computes no spell save DC and no casting execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let base_spells_per_day = arcanist_base_spells_per_day(level);
    for (spell_level, base_count) in base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = spell_level as u8;
        let int_bonus =
            ability_bonus_spells(ability_modifiers.intelligence, i16::from(spell_level));
        let total = base_count + int_bonus;

        explanations.push(ComputationExplanation {
            id: format!("class_spell.acg.arcanist.base_spells_per_day.spell_level_{spell_level}"),
            value: *base_count,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR formulas, hand-evaluated, and legacy.aonprd.com's own printed table
                //   -- both agree
                "Arcanist level {level} base spells per day at spell level {spell_level}: \
                 {base_count}, read directly from the PF1 Advanced Class Guide Arcanist class \
                 table's spells-prepared row (verified against the raw corpus)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!(
                "class_spell.acg.arcanist.intelligence_bonus_spells_per_day.spell_level_{spell_level}"
            ),
            value: int_bonus,
            detail: format!(
                "Arcanist level {level} Intelligence bonus spells per day at spell level \
                 {spell_level}: {int_bonus} from Intelligence modifier \
                 {} (PF1 Core Rulebook Table: Ability Modifiers and Bonus Spells)",
                ability_modifiers.intelligence
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!("class_spell.acg.arcanist.total_spells_per_day.spell_level_{spell_level}"),
            value: total,
            detail: format!(
                "Arcanist level {level} total spells per day at spell level {spell_level}: base \
                 {base_count} + Intelligence bonus {int_bonus} = {total} (no specialist bonus \
                 slot -- Arcanist has no arcane school)"
            ),
        });
    }
}

/// Grounds Arcanist's class features for `level` (v0.6 alpha swarm, risks
/// item 8, Arcanist full-build closure). Called from
/// `compute_acg_class_chassis`'s Arcanist branch, gated only on Arcanist
/// class-ownership. Grounds the Familiar Exploit (task #56) via the
/// shared, class-agnostic `ground_familiar_master_benefit` -- `KEY:
/// Arcanist Exploit ~ Familiar` carries the identical `BONUS:VAR|
/// FamiliarMasterLVL|ArcanistLVL` token as Witch's and Shaman's own
/// familiar grants, with no Arcane Reservoir cost and no PRE gate, so
/// this is a class-eligibility extension of already-shipped machinery,
/// not a new mechanism. Also grounds the Arcane Reservoir (flat,
/// unconditional -- no choice or activation gate) and the real
/// prepared-spellbook posture (conditional on
/// `unmet_arcanist_spellbook_conditions`), then grounds or blocks
/// Metamagic Knowledge (v0.6 alpha swarm, Metamagic Knowledge Exploit
/// closure): a recognized `choice:
/// arcanist_metamagic_knowledge` selection naming `Empower Spell`
/// (validated for real via `feat_prereqs::metamagic`, genuine reuse, not
/// hand-rolled) clears the claim-blocking exploits diagnostic in favor
/// of a non-blocking note naming the other 44 Exploits as still
/// deferred; an unrecognized/missing choice keeps the original
/// claim-blocking `exploits_deferred` diagnostic unchanged.
pub(super) fn ground_or_block_arcanist_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    ground_familiar_master_benefit(input, level, explanations);

    let reservoir_max = arcanist_reservoir_max(level, &input.chosen.selected_feats);
    let reservoir_daily_fill = arcanist_reservoir_daily_fill(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.arcanist.arcane_reservoir_max".to_owned(),
        value: reservoir_max,
        detail: format!(
            "Arcanist level {level} Arcane Reservoir maximum: 3 + level + Extra Reservoir feat \
             ({:+}) = {reservoir_max}. A flat, choice-free daily resource pool -- no \
             class_ability_activations entry is needed to ground this value, unlike every \
             activation-gated closure this session. Extra Reservoir raises this cap only: the \
             corpus gives the fill and the cap two separate variables and the feat names just \
             MaxArcanistReservoirSize",
            extra_resource_feat_bonus(
                &input.chosen.selected_feats,
                EXTRA_RESERVOIR_FEAT_KEY,
                EXTRA_RESERVOIR_POINTS
            )
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.arcanist.arcane_reservoir_daily_fill".to_owned(),
        value: reservoir_daily_fill,
        detail: format!(
            "Arcanist level {level} Arcane Reservoir daily fill: 3 + level/2 = \
             {reservoir_daily_fill}. This grounds only the flat daily-fill count; it does not \
             track intraday consumption from casting Exploits or Consume Spells"
        ),
    });

    let spellbook_unmet = unmet_arcanist_spellbook_conditions(input, level, ability_modifiers);
    if spellbook_unmet.is_empty() {
        ground_arcanist_prepared_spellbook(input, level, ability_modifiers, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.acg.arcanist.prepared_spellbook.unsupported".to_owned(),
            message: format!(
                "Arcanist remains blocked on its prepared spellbook / spells prepared / spell \
                 slot posture burden: {}",
                spellbook_unmet.join("; ")
            ),
            claim_blocking: true,
        });
    }

    ground_or_block_arcanist_metamagic_knowledge(input, explanations, diagnostics);
}

/// Grounds or claim-blocks Arcanist's Metamagic Knowledge Exploit (v0.6
/// alpha swarm, risks item 8, Arcanist Metamagic Knowledge Exploit
/// closure). A recognized `choice:arcanist_metamagic_knowledge`
/// selection naming e.g. `metamagic:empower_spell` -- translated to the
/// real feat name via `arcanist_metamagic_knowledge_feat_name`, then
/// validated for real via
/// `feat_prereqs::metamagic::evaluate_metamagic_feat_prerequisites`
/// (genuine reuse of an already-built, already-tested module from an
/// earlier, unrelated SD-20 Epic 3 cycle, not hand-rolled validation) --
/// grounds the real feat grant (its own catalog description text, via
/// `resolve_metamagic_feat_effect`) and replaces the claim-blocking
/// `exploits_deferred` diagnostic with a non-blocking note naming the
/// other 44 Exploits, Greater Exploits, Consume Spells, and Magical
/// Supremacy as still deferred (Familiar is no longer among them --
/// task #56 wired it into the shared, class-agnostic
/// `ground_familiar_master_benefit`). An unrecognized or missing choice keeps
/// the original claim-blocking `exploits_deferred` diagnostic
/// unchanged, mirroring the exact "recognized choice clears the
/// blocker, unrecognized stays blocked" shape Cleric's domain and
/// Warpriest's Blessing already established.
pub(super) fn ground_or_block_arcanist_metamagic_knowledge(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let selection = choice_selection(input, ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID);

    let Some(selection) = selection else {
        push_arcanist_exploits_deferred_diagnostic(diagnostics);
        return;
    };

    let Some(feat_name) = arcanist_metamagic_knowledge_feat_name(selection) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.acg.arcanist.metamagic_knowledge.feat_ineligible".to_owned(),
            message: format!(
                "Arcanist's Metamagic Knowledge Exploit names '{selection}' via \
                 {ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID}, but this is not a recognized \
                 `metamagic:<feat_slug>`-namespaced selection (e.g. `metamagic:empower_spell`)"
            ),
            claim_blocking: true,
        });
        push_arcanist_exploits_deferred_diagnostic(diagnostics);
        return;
    };

    let evaluation = evaluate_metamagic_feat_prerequisites(&feat_name);
    if !evaluation.is_eligible {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.acg.arcanist.metamagic_knowledge.feat_ineligible".to_owned(),
            message: format!(
                "Arcanist's Metamagic Knowledge Exploit names '{selection}' (translated to \
                 '{feat_name}') via {ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID}, but this is not a \
                 recognized, eligible Metamagic feat: {}",
                evaluation.failing_prerequisites.join("; ")
            ),
            claim_blocking: true,
        });
        push_arcanist_exploits_deferred_diagnostic(diagnostics);
        return;
    }

    let effect = resolve_metamagic_feat_effect(&feat_name)
        .expect("evaluate_metamagic_feat_prerequisites already confirmed eligibility");
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.arcanist.metamagic_knowledge.feat_granted".to_owned(),
        value: 0,
        detail: format!(
            "Arcanist's Metamagic Knowledge Exploit grants '{}' as a bonus feat (selection \
             '{selection}' translated to the real feat name, validated via \
             feat_prereqs::metamagic against the real CRB Metamagic feat catalog): {}",
            effect.feat_id, effect.description
        ),
    });
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.arcanist.exploits_deferred.unsupported".to_owned(),
        message: format!(
            "{ARCANIST_CLASS_ID} remains blocked beyond its base-attack-bonus/base-save chassis \
             pillar, Arcane Reservoir, prepared spellbook, and Metamagic Knowledge: the other \
             44 Arcanist Exploits (a chooser-list of real mechanical variety, named but not \
             built), Greater Arcanist Exploits, Consume Spells (a real formula, Charisma-\
             modifier-gated resource conversion), and Magical Supremacy (a capstone ability) \
             remain ungrounded anywhere in this codebase; no class-feature execution is \
             fabricated in this bounded chassis baseline"
        ),
        claim_blocking: false,
    });
}

/// Pushes the ORIGINAL, unconditional diagnostic replacing
/// `class_feature.acg.arcanist.unsupported` for Arcanist specifically
/// (v0.6 alpha swarm, risks item 8, Arcanist full-build closure): named
/// ONLY the genuinely still-missing pieces (Arcanist Exploits / Greater
/// Arcanist Exploits -- a chooser-list, real mechanical variety --
/// Consume Spells, and Magical Supremacy). Called only from
/// `ground_or_block_arcanist_metamagic_knowledge`'s own not-recognized
/// branches -- once Metamagic Knowledge IS recognized, a different,
/// non-blocking version of this same diagnostic id is pushed inline
/// there instead (see that function's own doc comment).
pub(super) fn push_arcanist_exploits_deferred_diagnostic(diagnostics: &mut Vec<ComputationDiagnostic>) {
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.arcanist.exploits_deferred.unsupported".to_owned(),
        message: format!(
            "{ARCANIST_CLASS_ID} remains blocked beyond its base-attack-bonus/base-save chassis \
             pillar, Arcane Reservoir, and prepared spellbook: Arcanist Exploits and Greater \
             Arcanist Exploits (a chooser-list of real mechanical variety, named but not built), \
             Consume Spells (a real formula, Charisma-modifier-gated resource conversion), and \
             Magical Supremacy (a capstone ability) remain ungrounded anywhere in this codebase; \
             no class-feature execution is fabricated in this bounded chassis baseline"
        ),
        claim_blocking: true,
    });
}

/// PF1 Advanced Class Guide Warpriest "Spells" table, base counts before
/// any Wisdom bonus spells, one entry per spell level 0 (cantrip) through
/// 9 (`None` for an inaccessible "--" column).
///
/// Transcribed literally from the PCGen corpus's own `CLASS:Warpriest`
/// level-progression `CAST:` rows at
/// `data/pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst`
/// lines 391-410 (level 1 on line 391 through level 20 on line 410) -- a
/// literal table lookup, not a derived formula.
///
/// Warpriest is a 6-level caster: spell levels 7-9 stay `None` at every
/// class level, matching both the corpus rows (no row has more than 7
/// columns) and the class's own
/// `KNOWNSPELLS:LEVEL=0|...|LEVEL=6` token on `acg_classes.lst:368`.
///
/// Genuinely DIFFERENT from `cleric_base_spells_per_day_table` starting at
/// level 3 (Cleric grants a 2nd-level spell slot at level 3; Warpriest
/// does not, first unlocking 2nd-level spells at level 4) even though the
/// spell-list CONTENT and casting SHAPE (prepared, no known-spells cap,
/// `SPELLLIST:1|Cleric`) are genuinely shared -- see
/// `WARPRIEST_CLASS_ID`'s own doc comment for the full verification
/// record.
///
/// Returns an all-`None` row outside the legal 1-20 class-level range.
pub(super) fn warpriest_base_spells_per_day(level: u8) -> [Option<i16>; 10] {
    match level {
        1 => [Some(3), Some(1), None, None, None, None, None, None, None, None],
        2 => [Some(4), Some(2), None, None, None, None, None, None, None, None],
        3 => [Some(4), Some(3), None, None, None, None, None, None, None, None],
        4 => [Some(4), Some(3), Some(1), None, None, None, None, None, None, None],
        5 => [Some(4), Some(4), Some(2), None, None, None, None, None, None, None],
        6 => [Some(5), Some(4), Some(3), None, None, None, None, None, None, None],
        7 => [Some(5), Some(4), Some(3), Some(1), None, None, None, None, None, None],
        8 => [Some(5), Some(4), Some(4), Some(2), None, None, None, None, None, None],
        9 => [Some(5), Some(5), Some(4), Some(3), None, None, None, None, None, None],
        10 => [Some(5), Some(5), Some(4), Some(3), Some(1), None, None, None, None, None],
        11 => [Some(5), Some(5), Some(4), Some(4), Some(2), None, None, None, None, None],
        12 => [Some(5), Some(5), Some(5), Some(4), Some(3), None, None, None, None, None],
        13 => [Some(5), Some(5), Some(5), Some(4), Some(3), Some(1), None, None, None, None],
        14 => [Some(5), Some(5), Some(5), Some(4), Some(4), Some(2), None, None, None, None],
        15 => [Some(5), Some(5), Some(5), Some(5), Some(4), Some(3), None, None, None, None],
        16 => [Some(5), Some(5), Some(5), Some(5), Some(4), Some(3), Some(1), None, None, None],
        17 => [Some(5), Some(5), Some(5), Some(5), Some(4), Some(4), Some(2), None, None, None],
        18 => [Some(5), Some(5), Some(5), Some(5), Some(5), Some(4), Some(3), None, None, None],
        19 => [Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(4), None, None, None],
        20 => [Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), None, None, None],
        _ => [None, None, None, None, None, None, None, None, None, None],
    }
}

/// Return the list of unmet conditions for this grounding's bounded
/// prepared-spellbook posture. Mirrors `unmet_arcanist_spellbook_conditions`'s
/// own shape exactly (Warpriest has no opposed-school/specialization
/// mechanic either -- it is a divine caster with a domain-equivalent
/// Blessing choice, not an arcane school).
pub(super) fn unmet_warpriest_spellbook_conditions(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Vec<String> {
    let mut unmet = Vec::new();

    if level > WARPRIEST_SPELLBOOK_SUPPORTED_MAX_LEVEL {
        unmet.push(format!(
            "prepared spellbook grounding is only supported for warpriest levels \
             1-{WARPRIEST_SPELLBOOK_SUPPORTED_MAX_LEVEL}, got {level}"
        ));
        return unmet;
    }

    let warpriest_spells = |mode: AcquisitionMode| {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(move |s| s.source_class_id == WARPRIEST_CLASS_ID && s.acquisition_mode == mode)
    };
    let recorded: Vec<&str> = warpriest_spells(AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();
    let prepared: Vec<&str> = warpriest_spells(AcquisitionMode::Prepared)
        .map(|s| s.spell_id.as_str())
        .collect();

    if recorded.is_empty() {
        unmet.push(
            "no warpriest spells recorded in the spellbook (AcquisitionMode::Known)".to_owned(),
        );
    }
    if prepared.is_empty() {
        unmet.push("no warpriest spells prepared today (AcquisitionMode::Prepared)".to_owned());
    }

    for spell_id in &prepared {
        if !recorded.contains(spell_id) {
            unmet.push(format!(
                "prepared spell '{spell_id}' is not recorded in the spellbook"
            ));
        }
    }

    // Same up-front per-class resolution as `unmet_wizard_spellbook_conditions`
    // — see `resolve_prepared_spell_level`. Warpriest's corpus-stated
    // `SPELLLIST:1|Cleric` means it resolves against Cleric's list.
    let mut prepared_levels: Vec<u8> = Vec::new();
    for spell_id in &prepared {
        match resolve_prepared_spell_level(WARPRIEST_CLASS_ID, spell_id) {
            PreparedSpellLevel::Known(spell_level) => prepared_levels.push(spell_level),
            PreparedSpellLevel::Unknown(reason) => unmet.push(reason),
        }
    }

    let base_spells_per_day = warpriest_base_spells_per_day(level);
    for (spell_level, base_count) in base_spells_per_day.iter().enumerate() {
        let spell_level = spell_level as u8;
        let Some(base_count) = base_count else {
            if prepared_levels.contains(&spell_level) {
                unmet.push(format!(
                    "a prepared spell targets spell level {spell_level}, not yet accessible at \
                     warpriest level {level}"
                ));
            }
            continue;
        };
        let wisdom_bonus =
            ability_bonus_spells(ability_modifiers.wisdom, i16::from(spell_level));
        let total_slots = base_count + wisdom_bonus;
        let consumed: i16 = prepared_levels.iter().filter(|l| **l == spell_level).count() as i16;
        if consumed > total_slots {
            unmet.push(format!(
                "spell level {spell_level} over-prepared: {consumed} spells prepared but only \
                 {total_slots} slots available (base {base_count} + Wisdom bonus {wisdom_bonus})"
            ));
        }
    }

    unmet
}

/// Ground the real prepared-spellbook / daily-preparation state once
/// `unmet_warpriest_spellbook_conditions` reports an empty unmet list.
/// Mirrors `ground_arcanist_prepared_spellbook`'s own shape exactly,
/// substituting Wisdom for Intelligence as the casting ability.
pub(super) fn ground_warpriest_prepared_spellbook(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let warpriest_spells = |mode: AcquisitionMode| {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(move |s| s.source_class_id == WARPRIEST_CLASS_ID && s.acquisition_mode == mode)
    };
    let recorded: Vec<&str> = warpriest_spells(AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();
    let prepared: Vec<&str> = warpriest_spells(AcquisitionMode::Prepared)
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.acg.warpriest.spellbook_contents".to_owned(),
        value: recorded.len() as i16,
        detail: format!(
            "Warpriest level {level} recorded spellbook contents ({} spells, \
             AcquisitionMode::Known): {}. This grounds which spells are recorded as real, \
             chosen input; it does not verify against any corpus that a named spell genuinely \
             exists or genuinely belongs to the level its own identifier claims",
            recorded.len(),
            recorded.join(", ")
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_spell.acg.warpriest.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Warpriest level {level} daily preparation selection ({} spells, \
             AcquisitionMode::Prepared, each already verified recorded in the spellbook above): \
             {}. This grounds the prepared-vs-known distinction for real: every prepared spell \
             is drawn from the recorded spellbook, consuming its spell level's slot budget (one \
             slot each). It computes no spell save DC and no casting execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let base_spells_per_day = warpriest_base_spells_per_day(level);
    for (spell_level, base_count) in base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = spell_level as u8;
        let wisdom_bonus =
            ability_bonus_spells(ability_modifiers.wisdom, i16::from(spell_level));
        let total = base_count + wisdom_bonus;

        explanations.push(ComputationExplanation {
            id: format!("class_spell.acg.warpriest.base_spells_per_day.spell_level_{spell_level}"),
            value: *base_count,
            detail: format!(
                "Warpriest level {level} base spells per day at spell level {spell_level}: \
                 {base_count}, read directly from the PF1 Advanced Class Guide Warpriest class \
                 table's own real per-level CAST rows (acg_classes.lst) -- a literal table \
                 lookup, not a derived formula"
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!(
                "class_spell.acg.warpriest.wisdom_bonus_spells_per_day.spell_level_{spell_level}"
            ),
            value: wisdom_bonus,
            detail: format!(
                "Warpriest level {level} Wisdom bonus spells per day at spell level \
                 {spell_level}: {wisdom_bonus} from Wisdom modifier \
                 {} (PF1 Core Rulebook Table: Ability Modifiers and Bonus Spells)",
                ability_modifiers.wisdom
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!("class_spell.acg.warpriest.total_spells_per_day.spell_level_{spell_level}"),
            value: total,
            detail: format!(
                "Warpriest level {level} total spells per day at spell level {spell_level}: base \
                 {base_count} + Wisdom bonus {wisdom_bonus} = {total}"
            ),
        });
    }
}

/// Destruction Blessing's own Minor power, Destructive Attacks: "+[max(1,
/// level/2)] morale bonus on weapon damage rolls" (PF1 Advanced Class
/// Guide), verified directly against `acg_abilities_class.lst`'s own
/// `KEY:Destruction Blessing ~ Destructive Attacks` DESC and
/// `max(1,WarpriestLVL/2)` formula. Byte-identical shape to
/// `cleric_touch_of_good_bonus`.
pub(super) fn warpriest_destructive_attacks_bonus(level: u8) -> i16 {
    (i16::from(level) / 2).max(1)
}

/// Warpriest's Blessings uses-per-day: `(level/2)+3` (PF1 Advanced Class
/// Guide), verified directly against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|WarpriestBlessingUses|(WarpriestBlessingLVL/2)+3`.
pub(super) fn warpriest_blessing_uses_per_day(level: u8) -> i16 {
    i16::from(level) / 2 + 3
}

/// Warpriest's Blessings save DC: `(level/2)+10+Wisdom modifier` (PF1
/// Advanced Class Guide), verified directly against
/// `acg_abilities_class.lst`'s own
/// `BONUS:VAR|WarpriestBlessingDC|(WarpriestBlessingLVL/2)+10+WIS`.
pub(super) fn warpriest_blessing_dc(level: u8, wisdom_modifier: i16) -> i16 {
    i16::from(level) / 2 + 10 + wisdom_modifier
}

/// Sacred Weapon's base-damage-die upgrade for a Medium weapon (this
/// codebase's own Longsword fixture), verified directly against
/// `acg_abilities_class.lst`'s own size-branched `BONUS:VAR` formulas
/// (`PREBASESIZEEQ:M` branch): dice size `if(LVL<5,6,if(LVL<10,8,
/// if(LVL<15,10,if(LVL<20,6,8))))`, dice count `1+min(1,LVL/15)`.
/// Returns `(dice_count, dice_size)`. At every level within this
/// grounding's own bounded 1-3 range, this evaluates to a flat `1d6` --
/// genuinely near-zero value for a Longsword (whose own native base
/// damage, 1d8, is already better), named honestly rather than
/// suppressed, mirroring Brawler's own "AC Bonus is genuinely +0 at
/// level 1" precedent.
///
/// **Real bug found and fixed (v0.6 alpha swarm, 2026-07-26, scout's
/// formula-audit pass, lead independently confirmed)**: `dice_count`
/// previously used `LVL/20`, the `PREBASESIZELT:M` (smaller-than-Medium)
/// branch's own denominator, transcribed by mistake into this function's
/// EQ:M (Medium) branch -- a sibling-branch mix-up, not a fabricated
/// value; the doc comment above also incorrectly cited `/20` as if
/// corpus-verified, so the error was internally self-consistent and
/// wouldn't have been caught by a doc-vs-code check alone. Corrected to
/// the real EQ:M formula, `1+min(1,LVL/15)`, verified directly against
/// the corpus record. Real effect: levels 15-19 previously computed
/// `dice_count = 1` (wrong; `15/20` through `19/20` all floor to 0) but
/// should be `2` (`15/15 = 1`, `min(1,1) = 1`, `+1 = 2`) -- this
/// grounding's own bounded 1-3 level range never exercised the wrong
/// branch, but the function itself was wrong at every level 15+.
pub(super) fn warpriest_sacred_weapon_base_dice(level: u8) -> (i16, i16) {
    let level = i16::from(level);
    let dice_size = if level < 5 {
        6
    } else if level < 10 {
        8
    } else if level < 15 {
        10
    } else if level < 20 {
        6
    } else {
        8
    };
    let dice_count = 1 + 1.min(level / 15);
    (dice_count, dice_size)
}

/// Warpriest Fervor's uses-per-day pool (deepening 2026-07-26, task #9),
/// re-derived directly against `acg_abilities_class.lst`'s own
/// `KEY:Warpriest ~ Fervor` record: `BONUS:VAR|WarpriestFervorUses|
/// WarpriestLVL/2+WIS`. `None` below level 2, the record's own grant
/// level (`acg_classes.lst`'s per-level row `2 ABILITY:...|Warpriest ~
/// Fervor`), mirroring Alchemist's/Investigator's own Poison Resistance
/// level-gate shape.
///
/// The corpus also carries `Warpriest Extra Channel`
/// (`BONUS:VAR|WarpriestFervorUses|2`), a feat-sourced second
/// contribution -- deliberately excluded here, not overlooked: it is a
/// feat effect, not a class-feature magnitude, and lives in featmate's
/// own `feat_effects.rs` lane rather than this class-dispatch one.
pub(super) fn warpriest_fervor_uses_per_day(level: u8, wisdom_modifier: i16) -> Option<i16> {
    if level < WARPRIEST_FERVOR_LEVEL {
        return None;
    }
    Some(i16::from(level) / 2 + wisdom_modifier)
}

/// Warpriest Fervor's swift-action self-heal magnitude in d6 (deepening
/// 2026-07-26, task #9), re-derived directly against the same
/// `KEY:Warpriest ~ Fervor` record: `BONUS:VAR|WarpriestFervorDice|
/// 1+max(0,min(20,WarpriestLVL)-2)/3`. Note the `/3` sits OUTSIDE the
/// `max(0,...)` here -- the opposite nesting from Sacred Armor's own
/// enhancement formula below, transcribed per-record rather than carried
/// across by assumption (the Sacred Weapon sibling-branch lesson).
/// Yields 1d6 at level 2, +1d6 per 3 levels after: 1/2/3/4/5/6/7 at
/// levels 2/5/8/11/14/17/20.
pub(super) fn warpriest_fervor_heal_dice(level: u8) -> Option<i16> {
    if level < WARPRIEST_FERVOR_LEVEL {
        return None;
    }
    Some(1 + 0.max(20.min(i16::from(level)) - 2) / 3)
}

/// Warpriest Channel Energy's save DC (deepening 2026-07-26, task #9),
/// re-derived directly against `acg_abilities_class.lst`'s own
/// `KEY:Warpriest ~ Channel Energy` record:
/// `BONUS:VAR|WarpriestChannelEnergyDC|10+WarpriestLVL/2+WIS`. `None`
/// below level 4, the record's own grant level.
///
/// **Real corpus discrepancy, resolved deliberately**: the separate
/// `KEY:Warpriest ~ Channel Positive Energy`/`Channel Negative Energy`
/// display records carry `10+(TL/2)+CHA` in their own DESC substitution
/// args -- a different stat (Charisma, the Cleric idiom) and a different
/// level term (total level). This transcribes the `BONUS:VAR` on the
/// real Channel Energy record itself, which is the authoritative
/// computed token and matches Warpriest's own WIS-keyed casting stat;
/// the display records' CHA is a copy-paste artifact of the Cleric
/// original, not Warpriest's real rule.
pub(super) fn warpriest_channel_energy_dc(level: u8, wisdom_modifier: i16) -> Option<i16> {
    if level < WARPRIEST_CHANNEL_ENERGY_LEVEL {
        return None;
    }
    Some(10 + i16::from(level) / 2 + wisdom_modifier)
}

/// How many times per day a Warpriest can Channel Energy: the Fervor pool
/// divided by the two Fervor uses one channel expends. `None` below level
/// 4, Channel Energy's own grant level.
///
/// This is a derived count, not a transcribed table: the corpus gives
/// Channel Energy no pool of its own at all (`BONUS:ABILITYPOOL|Warpriest
/// Channel Energy|1` grants the *ability*, once, not daily uses), and its
/// own DESC states the cost -- "expends two uses of his fervor ability".
/// So the honest daily count is exactly this quotient, and it moves with
/// Wisdom because Fervor's own pool does.
pub(super) fn warpriest_channel_energy_uses_per_day(level: u8, wisdom_modifier: i16) -> Option<i16> {
    if level < WARPRIEST_CHANNEL_ENERGY_LEVEL {
        return None;
    }
    let fervor_uses = warpriest_fervor_uses_per_day(level, wisdom_modifier)?;
    Some(fervor_uses / WARPRIEST_CHANNEL_ENERGY_FERVOR_COST)
}

/// Warpriest Sacred Armor's armor enhancement bonus (deepening
/// 2026-07-26, task #9), re-derived directly against
/// `acg_abilities_class.lst`'s own `KEY:Warpriest ~ Sacred Armor`
/// record: `BONUS:VAR|WarpriestSacredArmorEnhancement|
/// 1+max(0,(min(20,WarpriestLVL)-7)/3)`. Here the `/3` sits INSIDE the
/// `max(0,...)`, unlike Fervor's own dice formula above. Yields +1 at
/// level 7, +1 per 3 levels after, capping at +5 at level 19-20 --
/// matching the record's own DESC ceiling ("to a maximum of +5").
pub(super) fn warpriest_sacred_armor_enhancement(level: u8) -> Option<i16> {
    if level < WARPRIEST_SACRED_ARMOR_LEVEL {
        return None;
    }
    Some(1 + 0.max((20.min(i16::from(level)) - 7) / 3))
}

/// Warpriest Sacred Armor's uses-per-day pool, in 1-minute increments
/// (deepening 2026-07-26, task #9), re-derived directly against the same
/// record: `BONUS:VAR|WarpriestSacredArmorUses|WarpriestLVL`.
pub(super) fn warpriest_sacred_armor_uses_per_day(level: u8) -> Option<i16> {
    if level < WARPRIEST_SACRED_ARMOR_LEVEL {
        return None;
    }
    Some(i16::from(level))
}

/// Strength Blessing's own Minor power, Strength Surge: "an enhancement
/// bonus equal to [max(1, level/2)] on melee attack rolls, combat
/// maneuver checks that rely on Strength, Strength-based skills, and
/// Strength checks for 1 round" (deepening 2026-07-26, task #9),
/// verified directly against `acg_abilities_class.lst`'s own
/// `KEY:Strength Blessing ~ Strength Surge` DESC and its
/// `max(1,WarpriestLVL/2)` substitution arg -- byte-identical to
/// Destructive Attacks' own formula, and granted at the identical gate
/// (`PREVARGTEQ:WarpriestBlessingLVL,WarpriestMinorBlessingGrantedLVL`).
pub(super) fn warpriest_strength_surge_bonus(level: u8) -> i16 {
    (i16::from(level) / 2).max(1)
}

/// Grounds Warpriest's class features for `level` (v0.6 alpha swarm,
/// risks item 8, Warpriest full-build closure). Called from
/// `compute_acg_class_chassis`'s Warpriest branch, gated only on
/// Warpriest class-ownership. Grounds Blessings' flat uses-per-day/DC
/// (unconditional) and Sacred Weapon's flat base-damage-die (unconditional
/// -- both are always-on class features, not activation-gated), the
/// Destructive Attacks self-application closure (conditional on a
/// recognized Destruction Blessing choice plus activation, mirroring
/// `ground_or_block_cleric_domain`'s own three-branch shape), and the
/// real prepared-spellbook posture (conditional on
/// `unmet_warpriest_spellbook_conditions`), then pushes the narrowed
/// `other_features_deferred` diagnostic regardless of any of the above.
pub(super) fn ground_or_block_warpriest_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let blessing_uses_per_day = warpriest_blessing_uses_per_day(level);
    let blessing_dc = warpriest_blessing_dc(level, ability_modifiers.wisdom);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.warpriest.blessing_uses_per_day".to_owned(),
        value: blessing_uses_per_day,
        detail: format!(
            "Warpriest level {level} Blessings uses per day: level/2 + 3 = \
             {blessing_uses_per_day}. Genuinely enforced by nothing in this codebase yet -- \
             flat count only, no per-use consumption tracked here"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.warpriest.blessing_dc".to_owned(),
        value: blessing_dc,
        detail: format!(
            "Warpriest level {level} Blessings save DC: level/2 + 10 + Wisdom modifier \
             ({}) = {blessing_dc}",
            ability_modifiers.wisdom
        ),
    });

    let (sacred_weapon_dice_count, sacred_weapon_dice_size) =
        warpriest_sacred_weapon_base_dice(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.warpriest.sacred_weapon_base_damage_die".to_owned(),
        value: sacred_weapon_dice_size,
        detail: format!(
            "Warpriest level {level} Sacred Weapon base damage die for a Medium weapon (this \
             codebase's own Longsword fixture): {sacred_weapon_dice_count}d{sacred_weapon_dice_size}. \
             Genuinely near-zero value at this level -- the Longsword's own native base damage \
             (1d8) is already better -- named honestly rather than suppressed. The active \
             weapon-enhancement mechanic (a swift-action enhancement bonus plus a menu of \
             weapon special abilities) is not grounded"
        ),
    });

    ground_warpriest_fervor_channel_and_sacred_armor(level, ability_modifiers, explanations);

    // SD-32 T12 Epic 8 row 18 cycle 5: the generic "select ONE blessing,
    // inherit every one of its real corpus powers" pass, covering the
    // other 36 real Warpriest blessing groups (111 real records total,
    // `census_class_feature_pool_group_names.py`) beyond Destruction and
    // Strength this file already hand-models by name below -- purely
    // additive.
    // SD-36 Epic E PC4-1: recorded across BOTH generic passes below so `blessing_recognized`
    // (used for the `blessing_powers.unsupported` / `blessing_minor_major_powers.unmodeled`
    // diagnostic choice just below, and for `push_warpriest_other_features_deferred_diagnostic`)
    // reflects whether resolution for the player's ACTUAL selection genuinely succeeded, never
    // just whether the selection happens to be Destruction or Strength by name. Per-member
    // resolution inside both resolvers can silently `continue` (an unrecognised slug, a formula
    // the interpreter refuses, an empty header chain); bracketing `explanations.len()` around
    // both calls is a direct success probe -- a real explanation was pushed for THIS character's
    // selection, or it was not -- rather than a second, drifting copy of either resolver's own
    // membership logic.
    let blessing_generic_grounding_start = explanations.len();
    push_generic_pool_group_selection_magnitude(
        input,
        level,
        ability_modifiers,
        WARPRIEST_BLESSING_CHOICE_ID,
        "Warpriest",
        "Blessing",
        "blessing:",
        "class_feature.acg.warpriest.blessing.generic",
        1,
        explanations,
    );

    // SD-32 T12 Epic 8 row 18 cycle 15: the sibling generic pass for the DIFFERENT corpus shape
    // cycle 14's own §16 finding named -- 6 real Blessing groups (Earth, Trickery, Rune,
    // Protection, Repose, Knowledge) carry no BONUS:VAR at all but DO carry a real
    // %N-substituted DESC formula, a shape the resolver above (bonus_vars-only) can never reach.
    // Destruction and Strength are excluded (`already_hand_modelled_keys`) -- both already
    // grounded above by name, with activation-state gating this generic pass cannot reproduce.
    push_generic_pool_group_selection_description_magnitude(
        input,
        level,
        ability_modifiers,
        WARPRIEST_BLESSING_CHOICE_ID,
        "Warpriest",
        "Blessing",
        "blessing:",
        "class_feature.acg.warpriest.blessing_description.generic",
        1,
        &[
            "Destruction Blessing ~ Destructive Attacks",
            "Strength Blessing ~ Strength Surge",
        ],
        explanations,
    );
    // SD-36 Epic E PC4-1: true only if one of the two generic passes above actually pushed a
    // real explanation for the player's own selection -- never a nominal "is this slug in a
    // known list" check, which is exactly what let a Rune/Earth/etc. Blessing choice carry a
    // genuine grounded magnitude AND a `claim_blocking: true` "not supported" diagnostic at the
    // same time (SD-35 code review PC4-1, retro `1789643677178-sd36-epic-e-30e143`).
    let blessing_generically_grounded = explanations.len() > blessing_generic_grounding_start;

    let blessing_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == WARPRIEST_BLESSING_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();
    if blessing_selections.contains(&STRENGTH_BLESSING_SELECTION) {
        ground_warpriest_strength_surge(input, level, explanations);
    }
    if blessing_selections.contains(&DESTRUCTION_BLESSING_SELECTION) {
        let destructive_attacks_bonus = warpriest_destructive_attacks_bonus(level);
        let activation = input
            .chosen
            .class_ability_activations
            .iter()
            .find(|activation| {
                activation.ability_id == WARPRIEST_DESTRUCTIVE_ATTACKS_ABILITY_ID
            });
        match activation.map(|activation| activation.active_state) {
            Some(ActiveState::EquippedActive) => {
                explanations.push(ComputationExplanation {
                    id: "class_feature.acg.warpriest.destruction_blessing.destructive_attacks_self_application"
                        .to_owned(),
                    value: destructive_attacks_bonus,
                    detail: format!(
                        "Warpriest level {level} is actively using Destructive Attacks on \
                         HERSELF, SELF-APPLICATION ONLY (PF1 Advanced Class Guide Destruction \
                         Blessing: touch an ally, granting it a +{destructive_attacks_bonus} \
                         morale bonus on weapon damage rolls for 1 minute). Grounds only the \
                         flat magnitude -- this headless engine computes no weapon-damage total \
                         anywhere (unlike attack bonus/AC/saves/skills, which do have real \
                         integrated totals), so there is no existing pillar to layer this bonus \
                         onto; granting it to ANOTHER creature is also not modeled, mirroring \
                         Cleric's own Touch of Good self-application-only precedent"
                    ),
                });
            }
            _ => {
                explanations.push(ComputationExplanation {
                    id: "class_feature.acg.warpriest.destruction_blessing.destructive_attacks_not_active"
                        .to_owned(),
                    value: 0,
                    detail: format!(
                        "Warpriest level {level} is not currently using Destructive Attacks (no \
                         active class_ability_activations entry for \
                         \"{WARPRIEST_DESTRUCTIVE_ATTACKS_ABILITY_ID}\"): a genuinely valid PF1 \
                         posture -- not every Destruction-Blessing Warpriest is using this \
                         limited-use power at every moment -- so no morale bonus is claimed"
                    ),
                });
            }
        }
    }
    let blessing_hand_modeled = blessing_selections.contains(&DESTRUCTION_BLESSING_SELECTION)
        || blessing_selections.contains(&STRENGTH_BLESSING_SELECTION);
    let blessing_recognized = blessing_hand_modeled || blessing_generically_grounded;
    if blessing_recognized {
        let unmodeled_detail = if blessing_hand_modeled {
            "Warpriest Blessing minor/major power content beyond Destruction's own \
                 Destructive Attacks and Strength's own Strength Surge remains unmodeled: which \
                 specific power the OTHER chosen Blessing type grants (minor at 1st level, major \
                 at a later level) is not implemented for any of the other 18 Blessing types. \
                 About 15 of those remaining powers are touch-weapon \"Strike\" or summon \
                 \"Companion\" abilities, which need a weapon-enhancement activation surface and \
                 a summon subsystem this engine lacks entirely -- a genuine architecture gap, not \
                 a transcription backlog. This does not block an otherwise-valid \
                 Destruction-Blessing or Strength-Blessing posture"
                .to_owned()
        } else {
            "This chosen Blessing type's own minor/major power content beyond the single \
                 generically-resolved magnitude above (SD-36 Epic E PC4-1) remains unmodeled: \
                 the generic pass grounds one real per-member formula or bonus value, never \
                 the type's full minor-then-major power progression. This does not block an \
                 otherwise-valid posture for this Blessing choice"
                .to_owned()
        };
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.acg.warpriest.blessing_minor_major_powers.unmodeled".to_owned(),
            message: unmodeled_detail,
            claim_blocking: false,
        });
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.acg.warpriest.blessing_powers.unsupported".to_owned(),
            message: "Warpriest remains blocked on its Blessing powers burden: no recognized \
                 Destruction or Strength Blessing choice is present, and no OTHER Blessing \
                 selection's own member content resolved generically either. Of the corpus's 33 \
                 Blessing types -- each carrying its own minor AND major power, 66 powers in \
                 all -- exactly two powers are hand-modeled in this codebase (Destruction's \
                 Destructive Attacks and Strength's Strength Surge, both minor), plus whatever \
                 the generic pool-group resolvers can reach per corpus record, so no Warpriest \
                 Blessing-power support is claimed for this selection"
                .to_owned(),
            claim_blocking: true,
        });
    }

    let spellbook_unmet = unmet_warpriest_spellbook_conditions(input, level, ability_modifiers);
    if spellbook_unmet.is_empty() {
        ground_warpriest_prepared_spellbook(input, level, ability_modifiers, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.acg.warpriest.prepared_spellbook.unsupported".to_owned(),
            message: format!(
                "Warpriest remains blocked on its prepared spellbook / spells prepared / spell \
                 slot posture burden: {}",
                spellbook_unmet.join("; ")
            ),
            claim_blocking: true,
        });
    }

    ground_warpriest_focus_weapon(input, level, explanations);

    push_warpriest_other_features_deferred_diagnostic(blessing_recognized, diagnostics);
}

/// Grounds Warpriest's own Focus Weapon class feature: "At 1st level, a
/// warpriest receives Weapon Focus as a bonus feat (he can choose any
/// weapon, not just his deity's favored weapon)" -- verified directly
/// against `acg_abilities_class.lst`'s own `KEY:Warpriest ~ Focus Weapon`
/// record, whose only token besides the DESC is
/// `BONUS:ABILITYPOOL|Warpriest Focus Weapon Choice|1`.
///
/// This grant needs no new engine at all: Weapon Focus's own +1 attack
/// bonus is already computed and already integrated into this codebase's
/// real per-weapon attack totals (`feat.standalone.weapon_focus.<weapon>`,
/// via `feat_effects::weapon_focus_facts_from_choices`). So the honest
/// grounding is to resolve the grant against that live seam and report
/// which weapon it actually landed on -- never to restate the bonus as a
/// second, parallel number. When the character has not recorded the feat
/// on the fields that seam reads, this records that absence rather than
/// claiming a grant that produces nothing, mirroring Monk's own
/// bonus-feat unmet-precondition branch.
pub(super) fn ground_warpriest_focus_weapon(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let facts = crate::rules_core::feat_effects::weapon_focus_facts_from_choices(
        &input.chosen.selected_feats,
        &input.chosen.selected_choices,
    );
    if facts.is_empty() {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.focus_weapon.not_recorded".to_owned(),
            value: 0,
            detail: format!(
                "Warpriest level {level} Focus Weapon grants Weapon Focus as a bonus feat at \
                 1st level, but this character records no resolvable Weapon Focus target: \
                 `selected_feats` carries no Weapon Focus entry, or no weapon-target choice \
                 names the weapon it applies to. The grant is real and its mechanics are \
                 already built (Weapon Focus's +1 flows into this engine's per-weapon attack \
                 totals whenever both are recorded), so this is an unmet precondition on the \
                 input, not a missing engine -- no attack bonus is claimed for it here"
            ),
        });
        return;
    }
    let weapons: Vec<&str> = facts.iter().map(|f| f.weapon_name.as_str()).collect();
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.warpriest.focus_weapon.bonus_feat_granted".to_owned(),
        value: facts.len() as i16,
        detail: format!(
            "Warpriest level {level} Focus Weapon is genuinely realized: this character holds \
             Weapon Focus, resolved against {} weapon target(s) ({}), and the class grant \
             places no restriction on which weapon (\"he can choose any weapon, not just his \
             deity's favored weapon\"). No separate magnitude is recorded here on purpose -- \
             Weapon Focus's own +1 attack bonus is ALREADY integrated into this engine's real \
             per-weapon attack totals (`feat.standalone.weapon_focus.*`), and restating it \
             would double-count a number the totals already carry",
            facts.len(),
            weapons.join(", ")
        ),
    });
}

/// Grounds Warpriest's Fervor, Channel Energy DC, and Sacred Armor as
/// standalone explanation records (deepening 2026-07-26, task #9). All
/// five magnitudes are flat, always-on class-feature facts once their
/// own level gate is met, so this never claim-blocks -- the same
/// unconditional shape `ground_alchemist_bomb_and_poison_resistance`
/// already established. Below each gate, the feature is named with an
/// honest "correctly absent" record rather than silently omitted,
/// mirroring Alchemist's own Poison Resistance precedent.
///
/// None of the five is integrated into a computed total: this engine has
/// no healing/hit-point-recovery total, no channel-resolution pillar, and
/// no player armor-class enhancement pillar. They ground standalone under
/// the corrected canonical bar (a genuine, verifiable magnitude, honestly
/// labelled as not-integrated), the same bar Bard's Bardic Knowledge and
/// Slayer's Track already ship under.
pub(super) fn ground_warpriest_fervor_channel_and_sacred_armor(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let wisdom = ability_modifiers.wisdom;
    match warpriest_fervor_uses_per_day(level, wisdom) {
        Some(uses) => explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.fervor_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Warpriest level {level} Fervor uses per day: level/2 + Wisdom modifier \
                 ({wisdom:+}) = {uses}. A flat daily pool -- no per-use consumption is tracked \
                 here, the same shape as Blessings' own uses-per-day. The Extra Channel feat \
                 adds 2 more in the real PF1 rules; that is a feat effect, not a class-feature \
                 magnitude, and is not folded in here"
            ),
        }),
        None => explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.fervor_uses_per_day".to_owned(),
            value: 0,
            detail: format!(
                "Warpriest level {level} Fervor: correctly absent below level \
                 {WARPRIEST_FERVOR_LEVEL} by PF1 Advanced Class Guide level gate; the at-grant \
                 magnitude is named but not computed"
            ),
        }),
    }
    if let Some(dice) = warpriest_fervor_heal_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.fervor_heal_dice".to_owned(),
            value: dice,
            detail: format!(
                "Warpriest level {level} Fervor heal/harm magnitude: {dice}d6 \
                 (1 + max(0, min(20, level) - 2)/3 = {dice}) -- 1d6 at level 2, +1d6 per 3 \
                 levels after. Channel Energy deals or heals this same amount (its own record \
                 defers to Fervor's dice rather than defining its own). This engine computes no \
                 healing or hit-point-recovery total anywhere, so this grounds as a standalone \
                 flat magnitude only"
            ),
        });
    }
    // Channel Energy's own daily count. The corpus does NOT give Channel
    // Energy a pool of its own: `KEY:Warpriest ~ Channel Energy` says
    // "Using this ability is a standard action that expends two uses of
    // his fervor ability", and both display records restate it ("This
    // consumes 2 uses of your Fervor Ability"). So the real number of
    // channels available per day is the already-grounded Fervor pool
    // divided by that cost -- a derived count off a corpus-verified rule,
    // not a second transcribed table. Retires the standing claim that
    // "the Channel Positive/Negative Energy dice and uses ... remain
    // ungrounded": the DICE half of that claim was already stale when
    // written (both display records' `1+max(0,min(20,WarpriestLVL)-2)/3`
    // is byte-identical to `warpriest_fervor_heal_dice`, grounded above),
    // and this record closes the USES half.
    match warpriest_channel_energy_uses_per_day(level, wisdom) {
        Some(uses) => explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.channel_energy_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Warpriest level {level} Channel Energy uses per day: the Fervor pool \
                 ({}) divided by the {WARPRIEST_CHANNEL_ENERGY_FERVOR_COST} Fervor uses one \
                 channel expends = {uses}. Channel Energy has no daily pool of its own in the \
                 corpus -- it spends Fervor's. Flat count only: no per-use consumption is \
                 tracked here, and a Warpriest who spends Fervor on its other uses has \
                 correspondingly fewer channels, which this record does not simulate",
                warpriest_fervor_uses_per_day(level, wisdom).unwrap_or(0)
            ),
        }),
        None => explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.channel_energy_uses_per_day".to_owned(),
            value: 0,
            detail: format!(
                "Warpriest level {level} Channel Energy: correctly absent below level \
                 {WARPRIEST_CHANNEL_ENERGY_LEVEL} by PF1 Advanced Class Guide level gate; the \
                 at-grant magnitude is named but not computed"
            ),
        }),
    }
    if let Some(dice) = warpriest_fervor_heal_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.channel_energy_dice".to_owned(),
            value: dice,
            detail: format!(
                "Warpriest level {level} Channel Energy dice: {dice}d6, the SAME magnitude as \
                 Fervor's own heal/harm dice -- the Channel Positive Energy and Channel \
                 Negative Energy records both carry `1+max(0,min(20,WarpriestLVL)-2)/3` \
                 verbatim, byte-identical to the Fervor formula, and the Channel Energy record \
                 itself defers to it (\"The amount of damage dealt or healed is equal to the \
                 amount listed in the fervor ability\"). Recorded under its own id so the \
                 feature is visibly grounded rather than only implied by Fervor's record. \
                 Which of the two (positive or negative) a given Warpriest channels is \
                 alignment- and deity-driven; this codebase models neither, so no \
                 positive/negative determination is claimed"
            ),
        });
    }
    if let Some(dc) = warpriest_channel_energy_dc(level, wisdom) {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.channel_energy_dc".to_owned(),
            value: dc,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   Keyed on Wisdom, Warpriest's own casting stat, per the authoritative BONUS:VAR
                //   on the Channel Energy record itself; the class's separate positive/negative
                //   display records carry a Charisma-keyed variant inherited from the Cleric
                //   original, which is not Warpriest's real rule.
                "Warpriest level {level} Channel Energy save DC: 10 + level/2 + Wisdom modifier \
                 ({wisdom:+}) = {dc}. The resource routing (a channel expends two Fervor uses) is \
                 not modelled -- only the DC and the shared Fervor dice are grounded"
            ),
        });
    }
    match warpriest_sacred_armor_enhancement(level) {
        Some(enhancement) => {
            let uses = warpriest_sacred_armor_uses_per_day(level)
                .expect("Sacred Armor uses share the enhancement's own level gate");
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.warpriest.sacred_armor_enhancement".to_owned(),
                value: enhancement,
                detail: format!(
                    "Warpriest level {level} Sacred Armor enhancement bonus: \
                     +{enhancement} (1 + max(0, (min(20, level) - 7)/3)) -- +1 at level 7, +1 \
                     per 3 levels after, capping at +5. Grounds only the flat magnitude -- task \
                     #88 correction: `defense.baseline_armor_class` IS a real integrated AC \
                     total this codebase computes (the same total Brawler's own AC Bonus \
                     already integrates into), this magnitude is simply not wired into it yet, \
                     and the swift-action activation and the menu of armor special abilities it \
                     can buy instead are not modelled either, so this grounds as a standalone \
                     flat magnitude"
                ),
            });
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.warpriest.sacred_armor_uses_per_day".to_owned(),
                value: uses,
                detail: format!(
                    "Warpriest level {level} Sacred Armor uses per day: level = {uses} minutes, \
                     spendable in 1-minute increments. A flat daily pool -- no per-use \
                     consumption is tracked here"
                ),
            });
        }
        None => explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.sacred_armor_enhancement".to_owned(),
            value: 0,
            detail: format!(
                "Warpriest level {level} Sacred Armor: correctly absent below level \
                 {WARPRIEST_SACRED_ARMOR_LEVEL} by PF1 Advanced Class Guide level gate; the \
                 at-grant magnitude is named but not computed"
            ),
        }),
    }
}

/// Grounds Strength Blessing's own Strength Surge minor power (deepening
/// 2026-07-26, task #9), gated on a recognized Strength Blessing choice
/// plus a real activation -- the same three-branch shape
/// `ground_or_block_warpriest_class_features` already uses for
/// Destructive Attacks.
///
/// Unlike Destructive Attacks, Strength Surge needs no
/// "self-application only" narrowing: the corpus DESC targets the
/// Warpriest natively ("you can focus your own strength"). The bonus is
/// still grounded standalone rather than folded into this engine's real
/// attack-bonus total, because it is a 1-round, swift-action buy against
/// the Blessings daily pool and this closure tracks no per-use
/// consumption -- integrating it into a total without also enforcing that
/// budget would overstate what is actually verified, the same line
/// Charmed Life's own two-spot budget enforcement already drew.
pub(super) fn ground_warpriest_strength_surge(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let bonus = warpriest_strength_surge_bonus(level);
    let active = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == WARPRIEST_STRENGTH_SURGE_ABILITY_ID)
        .map(|activation| activation.active_state)
        == Some(ActiveState::EquippedActive);
    if active {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.strength_blessing.strength_surge_active".to_owned(),
            value: bonus,
            detail: format!(
                "Warpriest level {level} is actively using Strength Surge on herself (PF1 \
                 Advanced Class Guide Strength Blessing: as a swift action, focus your own \
                 strength, gaining a +{bonus} enhancement bonus on melee attack rolls, \
                 Strength-based combat maneuver checks, Strength-based skills, and Strength \
                 checks for 1 round). Natively self-targeted, so no self-application narrowing \
                 is needed. Grounds only the flat magnitude: it is a 1-round buy against the \
                 Blessings daily pool, and this closure tracks no per-use consumption, so \
                 folding it into this engine's attack-bonus total would claim an enforcement \
                 that does not exist"
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.warpriest.strength_blessing.strength_surge_not_active"
                .to_owned(),
            value: 0,
            detail: format!(
                "Warpriest level {level} is not currently using Strength Surge (no active \
                 class_ability_activations entry for \"{WARPRIEST_STRENGTH_SURGE_ABILITY_ID}\"): \
                 a genuinely valid PF1 posture -- a 1-round swift-action power is not up at \
                 every moment -- so no enhancement bonus is claimed"
            ),
        });
    }
}

/// Pushes the new, narrower diagnostic replacing
/// `class_feature.acg.warpriest.unsupported` for Warpriest specifically
/// (v0.6 alpha swarm, risks item 8, Warpriest full-build closure): named
/// ONLY the genuinely still-missing pieces.
///
/// **Canonical narrowing (v0.6 alpha swarm, Warpriest spellcasting-shaped
/// closure).** This used to claim-block unconditionally, which meant a
/// Warpriest who HAD made this codebase's one grounded Blessing choice
/// was reported identically to one who had made none -- the same
/// flattening `ground_or_block_arcanist_metamagic_knowledge` already
/// corrected for Arcanist. It now mirrors that shape exactly: a
/// recognized Blessing (Destruction or Strength) grounds a real
/// class-feature option and downgrades this to a NON-blocking note naming
/// the honest remainder; no recognized Blessing keeps it claim-blocking,
/// unchanged.
///
/// Two clauses were retired here as genuinely stale rather than
/// suppressed. **Focus Weapon** is now grounded for real
/// (`ground_warpriest_focus_weapon`) against the already-integrated
/// Weapon Focus attack-bonus seam. **The Channel Positive/Negative Energy
/// dice and uses** are now grounded too: the dice half was stale when the
/// clause was written (both display records carry Fervor's own
/// `1+max(0,min(20,WarpriestLVL)-2)/3` verbatim), and the uses half is
/// closed by `warpriest_channel_energy_uses_per_day`.
pub(super) fn push_warpriest_other_features_deferred_diagnostic(
    blessing_recognized: bool,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let remainder = "31 of the corpus's 33 Blessing types (only Destruction's own Destructive \
         Attacks and Strength's own Strength Surge are grounded -- 2 of 66 powers, since each \
         of the 33 types carries its own minor AND major power), Sacred Weapon's active \
         weapon-enhancement mechanic, Aspect of War, Spontaneous Casting, Aura, and Bonus \
         Languages remain ungrounded anywhere in this codebase. Sacred Weapon's active \
         enhancement and a large share of the remaining Blessing powers are blocked on the same \
         real, missing architecture -- a weapon-enhancement activation surface and a summon \
         subsystem -- not on transcription effort; no class-feature execution is fabricated in \
         this bounded chassis baseline. Two clauses this message used to carry are retired as \
         stale, not suppressed: Focus Weapon is now genuinely grounded against the already-\
         integrated Weapon Focus attack-bonus seam, and the Channel Positive/Negative Energy \
         dice and uses are now grounded (the dice were already Fervor's own formula verbatim; \
         the uses are the Fervor pool divided by the 2 uses a channel expends). This message \
         also previously said \"18 of the 20 Blessing types\": the real corpus count is 33 \
         types, verified by enumerating `KEY:<X> Blessing ~ ...` directly (task #76)";

    let grounded = "its base-attack-bonus/base-save chassis pillar, Blessings' flat \
         uses-per-day/DC, Sacred Weapon's base damage die, Fervor's pool and heal dice, Channel \
         Energy's save DC, dice and uses per day, Sacred Armor's enhancement and pool, Focus \
         Weapon's bonus-feat grant, its class-skill list, Orisons, and prepared spellbook";

    let message = if blessing_recognized {
        format!(
            "{WARPRIEST_CLASS_ID} has {grounded}, plus one real, corpus-verified Blessing type \
             chosen from its own chooser -- the canonical narrowing this codebase applies to \
             every large class chooser (Cleric's Good domain, Wizard's Evocation school, \
             Oracle's Mystery, Arcanist's Metamagic Knowledge). What is deferred, honestly and \
             non-blockingly: {remainder}"
        )
    } else {
        format!("{WARPRIEST_CLASS_ID} remains blocked beyond {grounded}: {remainder}")
    };

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.warpriest.other_features_deferred.unsupported".to_owned(),
        message,
        claim_blocking: !blessing_recognized,
    });
}


/// PF1 Advanced Class Guide Swashbuckler Panache: "a swashbuckler gains a
/// number of panache points equal to her Charisma modifier (minimum 1)."
/// **Sourced from the ability's own DESC text, not a literal `BONUS:VAR`
/// token** -- confirmed directly that the base Swashbuckler's own
/// `KEY:Swashbuckler ~ Panache` record in `acg_abilities_class.lst`
/// defines no `Panache_Cap`/`PanachePoints` formula at all (only the
/// Inspired Blade archetype variant and the Extra Panache feat set/adjust
/// these variables). Cross-validated against the Inspired Blade
/// archetype's own explicit `BONUS:VAR|Panache_Cap|MAX(1,CHA)+MAX(1,INT)`
/// -- its own DESC says "unlike other swashbucklers... this ability
/// alters the panache class feature," and the archetype's formula adding
/// an Intelligence term ON TOP of a base `MAX(1,CHA)` confirms the base
/// class's real formula is exactly `MAX(1,CHA)`, not a guess.
///
/// **Evidentiary upgrade (2026-07-29).** The hedge above -- that this
/// formula alone among its siblings rested on inference rather than a
/// literal token -- can now be retired. A literal token does exist; it
/// simply is not on the base record, which is why reading only
/// `KEY:Swashbuckler ~ Panache` missed it. `acg_abilities_class.lst`
/// line 1976 carries `CATEGORY=Special Ability|Swashbuckler ~
/// Panache.MOD  BONUS:VAR|PanachePointsBase|max(CHA,1)|TYPE=Base`, and
/// an `CATEGORY=Internal|Panache Tracker.MOD` block above it (lines
/// 1965-1975) `DEFINE`s `PanachePointsBase`/`Panache_Cap`/
/// `PanachePoints` to `0` and then routes
/// `BONUS:VAR|Panache_Cap|PanachePointsBase`. That is precisely the
/// "`DEFINE`s to 0 while the real value arrives from an unconditional
/// `BONUS:VAR` on a different record" shape this file has been bitten by
/// before (`WeaponFocusToHit`). The inferred value and the literal token
/// agree exactly at `max(CHA,1)`, so no magnitude changes -- only the
/// confidence label does.
///
/// `Extra Panache` adds `2` to `Panache_Cap` (and the same `2` to
/// `PanachePoints`, the start-of-day count this engine does not
/// separately compute); its prose "your maximum panache increases by
/// two" matches the token exactly.
pub(super) fn swashbuckler_panache_max(charisma_modifier: i16, selected_feats: &[String]) -> i16 {
    charisma_modifier.max(1)
        + extra_resource_feat_bonus(selected_feats, EXTRA_PANACHE_FEAT_KEY, EXTRA_PANACHE_POINTS)
}

/// PF1 Advanced Class Guide Swashbuckler Charmed Life uses per day:
/// granted starting at 2nd level (see `SWASHBUCKLER_CHARMED_LIFE_MIN_LEVEL`'s
/// own doc comment for the real-rule verification), `((SwashbucklerLVL-2)/4)+3`
/// from there, verified directly against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|SwashbucklerCharmedLifeTimes|((SwashbucklerLVL-2)/4)+3`.
/// Returns `None` below the grant level (the feature genuinely doesn't
/// exist yet, not a zero-use edge case).
pub(super) fn swashbuckler_charmed_life_uses_per_day(level: u8) -> Option<i16> {
    if level < SWASHBUCKLER_CHARMED_LIFE_MIN_LEVEL {
        return None;
    }
    Some(((i16::from(level) - 2) / 4) + 3)
}

/// PF1 Advanced Class Guide Swashbuckler Nimble: `(SwashbucklerLVL+1)/4`
/// dodge bonus to AC while wearing light or no armor, verified directly
/// against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|SwashbucklerDodgeBonus|(SwashbucklerLVL+1)/4`. Grounds as a
/// standalone flat record -- this codebase computes no player AC total
/// anywhere (the same `risks-and-open-questions.md` item 1 architecture
/// gap that already excluded Trapfinding/Track/Destructive Attacks from
/// any total-integration).
pub(super) fn swashbuckler_nimble_dodge_bonus(level: u8) -> i16 {
    (i16::from(level) + 1) / 4
}

/// Whether a Swashbuckler of `level` has reached deed `tier`.
///
/// **Deliberate, ruled deviation from the corpus's literal gate**
/// (task #14, 2026-07-27; lead ruling recorded as
/// `risks-and-open-questions.md` item 50).
///
/// Every deed tier gates on `PREVARGTEQ:SwashbucklerDeedQualifyLVL,N`,
/// but that variable is `DEFINE:...|0` and has exactly ONE setter in the
/// entire PCGen tree: `BONUS:VAR|SwashbucklerDeedQualifyLVL|MagusLVL|
/// TYPE=Base`, on the Magus archetype that borrows deeds. It is never
/// set from `SwashbucklerLVL`. Implemented literally, a pure
/// single-class Swashbuckler would have ZERO deeds at every level.
///
/// This substitutes `SwashbucklerLVL` as the level source. That is a
/// transcription fix rather than a fabrication, and narrowly so: the
/// corpus itself already states the correct driver in a sibling variable
/// on the same records -- `BONUS:VAR|SwashbucklerDeedsLVL|SwashbucklerLVL`,
/// which every deed FORMULA correctly uses. Only the wiring between two
/// corpus-stated facts is missing. The tier thresholds (1/3/7/11/15/19)
/// are read directly from the corpus's own gates and match RAW.
///
/// A future case where the corpus does NOT already encode the right
/// answer elsewhere would need its own ruling; it cannot appeal to this
/// one.
pub(super) fn swashbuckler_deed_tier_reached(level: u8, tier: u8) -> bool {
    level >= tier
}

/// Swashbuckler Initiative's flat initiative bonus: +2 while she has at
/// least 1 panache point.
///
/// Verified against `acg_abilities_class.lst:2075`'s own
/// `TEMPBONUS:PC|COMBAT|INITIATIVE|2`. Note the token is a **TEMPBONUS**,
/// not a `BONUS` -- PCGen does not apply it unconditionally, which
/// matches the rule's own "as long as she has at least 1 panache point"
/// condition. That is why this grounds as a standalone record and is not
/// folded into any initiative total.
pub(super) const SWASHBUCKLER_INITIATIVE_BONUS: i16 = 2;

/// Grounds the Unchained Summoner's named features
/// (`rules_tables::pathfinder_unchained::summoner_features`).
pub(super) fn ground_unchained_summoner_class_features(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if let Some(companion_level) = summoner_features::eidolon_companion_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.eidolon_companion_level".to_owned(),
            value: i16::from(companion_level),
            detail: format!(
                "Unchained Summoner level {level} Eidolon: effective companion level \
                 {companion_level} (1:1 with class level)"
            ),
        });
    }
    if let Some(pool) = summoner_features::eidolon_evolution_pool(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.eidolon_evolution_pool".to_owned(),
            value: i16::from(pool),
            detail: format!(
                "Unchained Summoner level {level} Eidolon: an evolution pool of {pool} (a base of \
                 1 plus one point at each of 14 level thresholds). This is the SHARPEST \
                 divergence from the Advanced Player's Guide Summoner, whose pool starts at 3 and \
                 takes double steps -- the two are separate functions in separate modules on \
                 separate rule sets precisely so they cannot be confused"
            ),
        });
    }
    if let Some(spell_level) = summoner_features::summon_monster_spell_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.summon_monster_spell_level".to_owned(),
            value: spell_level,
            detail: format!(
                "Unchained Summoner level {level} Summon Monster: casts summon monster \
                 {spell_level} as a spell-like ability (min((level + 1) / 2, 9))"
            ),
        });
    }
    if let Some(uses) =
        summoner_features::summon_monster_uses_per_day(level, ability_modifiers.charisma)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.summon_monster_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Unchained Summoner level {level} Summon Monster: {uses} uses per day \
                 (max(Charisma modifier {}, 0) + 3). The max(...,0) is a genuine Unchained change \
                 -- the Advanced Player's Guide Summoner writes plain CHA + 3, so a negative \
                 Charisma modifier reduces a chained summoner below 3 uses and cannot reduce an \
                 unchained one",
                ability_modifiers.charisma
            ),
        });
    }
    if let Some(marker) = summoner_features::unchained_summoner_marker(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.unchained_summoner_marker".to_owned(),
            value: i16::from(marker),
            detail:
                "The UnchainedSummoner flag is set. In the corpus this is the switch that swaps \
                 the entire summoner spell list: every Advanced Player's Guide / Ultimate Magic / \
                 Ultimate Combat / Mythic Adventures summoner spell row is gated on \
                 StandardSummoner, which holding this class drives to 0, and only the Unchained \
                 rows remain. It is grounded here so the replacement is checkable rather than \
                 merely documented"
                    .to_owned(),
        });
    }
    // Nine Unchained Summoner records whose whole numeric content is their
    // own English DESC:. Twelve of this class's seventeen features carry no
    // arithmetic token at all -- it leans on prose harder than any of the
    // other three -- so this is where most of the book's "computes nothing"
    // set lived. Every sentence these come from is quoted in
    // `summoner_features::prose_derived` and re-read off the ingested corpus
    // record by that module's tests.
    if let Some(rounds) = summoner_features::prose_derived::bond_senses_rounds_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.bond_senses_rounds_per_day".to_owned(),
            value: i16::from(rounds),
            detail: format!(
                "Unchained Summoner level {level} Bond Senses: {rounds} rounds per day of sharing \
                 the eidolon's senses (equal to summoner level), as a standard action, at \
                 unlimited range so long as both are on the same plane"
            ),
        });
    }
    if let Some(bonus) = summoner_features::prose_derived::shield_ally_self_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.shield_ally_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Unchained Summoner level {level} Shield Ally: a +{bonus} shield bonus to Armor \
                 Class and a +{bonus} circumstance bonus on saving throws while within the \
                 eidolon's reach -- +2 from level 4, rising to +4 at level 12 when Greater Shield \
                 Ally names the summoner himself. Conditional on the eidolon being in reach and \
                 not grappled, helpless, paralyzed, stunned or unconscious, none of which this \
                 engine tracks, so it is NOT folded into the character's resting Armor Class"
            ),
        });
    }
    if let Some(uses) = summoner_features::prose_derived::makers_call_uses_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.makers_call_uses_per_day".to_owned(),
            value: i16::from(uses),
            detail: format!(
                "Unchained Summoner level {level} Maker's Call: {uses} uses per day \
                 (1 + (level - 6) / 4) of calling the eidolon to the summoner's side as dimension \
                 door at the summoner's caster level"
            ),
        });
    }
    if let Some(points) = summoner_features::prose_derived::divertible_evolution_points(level) {
        // Aspect grants the ability at 10 and Greater Aspect raises the same
        // ceiling at 18, so the two records share one magnitude and are named
        // together rather than double-counted.
        if points > 0 {
            explanations.push(ComputationExplanation {
                id: "class_feature.pu.unchained_summoner.aspect_evolution_points_divertible"
                    .to_owned(),
                value: i16::from(points),
                detail: format!(
                    "Unchained Summoner level {level} Aspect: up to {points} points may be \
                     diverted from the eidolon's evolution pool to the summoner himself -- 2 from \
                     level 10, raised to 6 by Greater Aspect at level 18, which additionally \
                     changes the exchange rate so the eidolon loses 1 pool point per 2 diverted \
                     (or fraction thereof) rather than 1 per 1. That second rule is a separate \
                     rule and is deliberately NOT folded into this number. The evolution \
                     catalogue these points are spent on is not ingested"
                ),
            });
        }
    }
    if let Some(points) =
        summoner_features::prose_derived::greater_aspect_divertible_evolution_points(level)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.greater_aspect_evolution_points_divertible"
                .to_owned(),
            value: i16::from(points),
            detail: format!(
                "Unchained Summoner level {level} Greater Aspect: the diversion ceiling rises to \
                 {points} points, and the exchange rate improves -- the eidolon loses 1 pool \
                 point for every 2 diverted (or fraction thereof) instead of 1 for each. The \
                 ceiling is this number; the exchange rate is a second, separate rule and is not \
                 folded into it"
            ),
        });
    }
    if let Some(bonus) = summoner_features::prose_derived::greater_shield_ally_bonus_to_allies(level)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.greater_shield_ally_bonus_to_allies"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Unchained Summoner level {level} Greater Shield Ally: allies other than the \
                 summoner within the eidolon's reach gain a +{bonus} shield bonus to Armor Class \
                 and a +{bonus} circumstance bonus on saving throws. This is the half of the \
                 feature that is genuinely new at level 12 -- the summoner's own bonus is the \
                 Shield Ally row above, which this raises to +4"
            ),
        });
    }
    if let Some(rounds) = summoner_features::prose_derived::merge_forms_rounds_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.merge_forms_rounds_per_day".to_owned(),
            value: i16::from(rounds),
            detail: format!(
                "Unchained Summoner level {level} Merge Forms: {rounds} rounds per day merged \
                 into the eidolon (equal to summoner level), untargetable while merged. If the \
                 eidolon is sent home mid-merge the summoner takes 4d6 damage and is stunned for \
                 1 round -- stated, not applied, because this engine resolves no damage"
            ),
        });
    }
    if let Some(minutes) = summoner_features::prose_derived::twin_eidolon_minutes_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.twin_eidolon_minutes_per_day".to_owned(),
            value: i16::from(minutes),
            detail: format!(
                "Unchained Summoner level {level} Twin Eidolon: {minutes} MINUTES per day in the \
                 eidolon's shape (equal to summoner level), spent in 1-minute increments. Minutes \
                 rather than rounds is the row's own unit and is not converted -- its two sibling \
                 durations here, Bond Senses and Merge Forms, are rounds"
            ),
        });
    }
    if let Some(feet) = summoner_features::prose_derived::life_link_full_strength_range_feet(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_summoner.life_link_full_strength_range_feet".to_owned(),
            value: feet,
            detail: format!(
                "Unchained Summoner level {level} Life Link: the eidolon stays at full strength \
                 within {feet} feet of the summoner; beyond that and within {} feet its current \
                 and maximum hit points are halved, beyond that and within {} feet they are cut \
                 by three quarters, and past {} feet it returns to its home plane. The summoner \
                 may also sacrifice hit points one-for-one to prevent damage that would send it \
                 home",
                summoner_features::prose_derived::LIFE_LINK_HALF_STRENGTH_RANGE_FEET,
                summoner_features::prose_derived::LIFE_LINK_QUARTER_STRENGTH_RANGE_FEET,
                summoner_features::prose_derived::LIFE_LINK_BANISHMENT_RANGE_FEET,
            ),
        });
    }
    let class_skills = summoner_features::class_skills();
    explanations.push(ComputationExplanation {
        id: "class_feature.pu.unchained_summoner.class_skill_count".to_owned(),
        value: class_skills.len() as i16,
        detail: format!(
            "Unchained Summoner class skills, as the book's own class-skill list states them \
             ({} entries): {}. This class gets every Knowledge skill, where the Unchained \
             Rogue's list names only two",
            class_skills.len(),
            render_class_skill_list(class_skills)
        ),
    });

    push_deferred_class_features(
        "class_feature.pu.unchained_summoner.other_features_deferred.unsupported",
            "class:unchained_summoner grounds every Unchained Summoner magnitude this book states \
             as a formula token: the chassis (borrowed unchanged from the Advanced Player's Guide \
             Summoner, which the corpus record confirms it does not override -- the one of the \
             four Unchained classes whose base class is not the Core Rulebook), the eidolon's \
             companion level and evolution pool, the Summon Monster spell level and uses per day, \
             the spell-list swap flag, and the class-skill list -- plus nine whose only numbers \
             are in their own prose: Life Link's range bands, Bond Senses' rounds per day, Shield \
             Ally's and Greater Shield Ally's bonuses, Maker's Call's uses per day, Aspect's and \
             Greater Aspect's diversion ceilings, Merge Forms' rounds per day and Twin Eidolon's \
             minutes per day. This diagnostic is NOT claim-blocking; it carries the honest \
             remainder. THE LARGEST MISSING PIECE IS \
             SPELLCASTING: this class has its own 202-spell list (12/35/39/39/27/23/27 at levels \
             0-6) declared in the book, and none of it is transcribed, so an Unchained Summoner \
             computes with no spells known, no spells per day and no spell DCs. That is a real \
             gap and is stated rather than filled with the Advanced Player's Guide list, which \
             would be the WRONG list -- the corpus explicitly switches it off. Note also that 46 \
             of those 202 spells are defined only in Ultimate Magic and Ultimate Combat, neither \
             of which is an ingested book, so the list cannot be completed here even in \
             principle. Also missing: the 13 eidolon subtypes are named slots with no contents \
             ingested; the evolution catalogue the pool is spent on is not ingested; and three \
             features state no number even in prose, so nothing is computed for them and nothing \
             is invented -- CANTRIPS, whose count lives on Table 1-5 rather than on this row and \
             whose spell list is the untranscribed one above; TRANSPOSITION, which spends a \
             Maker's Call use to swap places rather than adding a quantity of its own; and LIFE \
             BOND, whose only numbers ('1 or more hit points', damage 'transferred 1 point at a \
             time') are the mechanic's granularity and not a quantity a player tracks. The nine \
             prose-derived magnitudes above are magnitudes and not applications: none of them is \
             folded into a resting total, because every one is gated on the eidolon's position or \
             condition, which this engine does not model"
            .to_owned(),
        explanations,
        diagnostics,
    );
}

/// v0.6 alpha swarm, risks item 8 (Witch full-build closure, 11th
/// ACG/APG class-specific closure): tests the Ward hex choice dispatch,
/// mirroring the established dispatch-widening test module shape.
#[cfg(test)]
mod witch_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, HeadlessReceiptStatus,
        CAULDRON_HEX_SELECTION, FIGHTER_CLASS_ID, FLIGHT_HEX_SELECTION, WARD_HEX_SELECTION,
        WITCH_CLASS_ID, WITCH_HEX_CHOICE_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, AcquisitionMode, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn witch_spell(spell_id: &str, mode: AcquisitionMode) -> SpellSelection {
        SpellSelection {
            spell_id: spell_id.to_owned(),
            source_class_id: WITCH_CLASS_ID.to_owned(),
            acquisition_mode: mode,
        }
    }

    /// The defining Witch mechanic: a spell above cantrip level must be
    /// stored in her familiar before it can be prepared. This is what
    /// separates her from Cleric/Druid/Shaman, whose class lines carry
    /// the full `KNOWNSPELLS:LEVEL=0|...|LEVEL=9` ladder while hers
    /// carries `KNOWNSPELLS:LEVEL=0` alone.
    #[test]
    fn a_levelled_spell_not_stored_in_the_familiar_cannot_be_prepared() {
        let mut input = human_witch_input(3);
        // Prepared but never stored.
        input.chosen.spells_selected.push(witch_spell("Ray of Enfeeblement", AcquisitionMode::Prepared));

        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.witch.prepared_spells.unsupported"
                    && d.claim_blocking
                    && d.message.contains("not stored in the witch's familiar")),
            "preparing an unstored levelled spell must claim-block: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The same spell, once stored, prepares cleanly -- proving the gate
    /// is the storage requirement itself and not something incidental.
    #[test]
    fn the_same_spell_prepares_once_it_is_stored_in_the_familiar() {
        let mut input = human_witch_input(3);
        input.chosen.spells_selected.push(witch_spell("Ray of Enfeeblement", AcquisitionMode::Known));
        input.chosen.spells_selected.push(witch_spell("Ray of Enfeeblement", AcquisitionMode::Prepared));

        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.witch.prepared_spells.unsupported"),
            "a stored spell must prepare cleanly: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_spell.apg.witch.familiar_stored_spells"),
            "the familiar's store must be grounded"
        );
    }

    /// Cantrips are the documented exception: `KNOWNSPELLS:LEVEL=0` means
    /// level-0 spells are known automatically and need no stored record.
    #[test]
    fn a_cantrip_needs_no_familiar_storage() {
        let mut input = human_witch_input(1);
        input.chosen.spells_selected.push(witch_spell("Guidance", AcquisitionMode::Prepared));

        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.witch.prepared_spells.unsupported"),
            "a cantrip must prepare without being stored: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A spell stored in the familiar must still be a real witch spell.
    #[test]
    fn a_non_witch_spell_cannot_be_stored_in_the_familiar() {
        let mut input = human_witch_input(3);
        input.chosen.spells_selected.push(witch_spell("Magic Missile", AcquisitionMode::Known));

        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.witch.prepared_spells.unsupported"
                    && d.message.contains("is not on the real PF1 witch spell list")),
            "an off-list spell must not be storable: {:?}",
            receipt.computation.diagnostics
        );
    }

    fn human_witch_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: WITCH_CLASS_ID.to_owned(), level }];
        input
    }

    /// A bare single-class Human Witch (no Hex choice) stays `Blocked`
    /// on both the hex_powers diagnostic and other_features_deferred,
    /// never the retired generic diagnostic.
    #[test]
    fn single_class_witch_bare_stays_blocked_on_hex_powers_and_other_features() {
        let input = human_witch_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Witch must stay Blocked without a recognized Hex choice: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.witch.unsupported"),
            "the retired generic diagnostic must never appear for Witch: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.witch.hex_powers.unsupported"
                    && d.claim_blocking),
            "expected the hex_powers claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.witch.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Human Witch with the Ward hex recognized grounds
    /// the flat deflection/resistance bonus for real and clears the
    /// hex_powers diagnostic in favor of the non-blocking "other hexes"
    /// note -- stays `Blocked` on other_features_deferred regardless.
    ///
    /// Level 1 Ward bonus: base 2 (no level-8/16 bump yet).
    #[test]
    fn single_class_witch_with_ward_hex_grounds_the_real_bonus() {
        let mut input = human_witch_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WITCH_HEX_CHOICE_ID.to_owned(),
            selection_id: WARD_HEX_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.witch.hex_powers.unsupported"),
            "hex_powers must not fire once Ward is recognized: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.witch.hex_powers_beyond_ward.unmodeled"
                    && !d.claim_blocking),
            "expected the non-blocking other-hexes note: {:?}",
            receipt.computation.diagnostics
        );
        let ward = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.witch.ward_hex.deflection_and_resistance_bonus")
            .expect("Ward's bonus must be grounded once recognized");
        assert_eq!(ward.value, 2, "Witch level 1 Ward bonus: base 2: {:?}", ward);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Ward is one of the three corpus-grounded hexes, so the canonical-narrowing \
             posture is satisfied and other_features_deferred reports non-blocking: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// **Canonical narrowing, Witch's own (task: chooser-shaped power
    /// lists).** Flight is the canonical hex this codebase seeds, chosen
    /// over Ward and Cauldron because its magnitude is the only one of
    /// the 27 that lands on a total this engine actually computes:
    /// `BONUS:SKILL|Swim|4|TYPE=Racial`
    /// (`advanced_players_guide/apg_abilities_class.lst:892`,
    /// `KEY:Witch Hex ~ Flight`) flows into `skill.selected_modifier.swim`.
    /// Ward's deflection/resistance is not wired into the AC/save totals,
    /// and Cauldron's is a Craft (Alchemy) bonus on a skill this engine
    /// does not compute -- both ground standalone.
    ///
    /// This also pins the staleness fix: `hex_powers.unsupported` used to
    /// be retired by Ward ALONE, so a Witch who took Flight -- a hex whose
    /// magnitude is genuinely grounded and genuinely integrated -- was
    /// still told "no recognized hex choice is present", which the
    /// diagnostic's own message contradicted by naming all three.
    #[test]
    fn single_class_witch_with_the_canonical_flight_hex_reaches_computed() {
        let mut input = human_witch_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WITCH_HEX_CHOICE_ID.to_owned(),
            selection_id: FLIGHT_HEX_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.witch.hex_powers.unsupported"),
            "hex_powers must not fire once the grounded Flight hex is recognized: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.witch.hex_powers_beyond_base.unmodeled"
                    && !d.claim_blocking),
            "expected the non-blocking other-hexes note for a non-Ward grounded hex: {:?}",
            receipt.computation.diagnostics
        );
        let flight = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.witch.flight_hex.swim_bonus")
            .expect("Flight's Swim bonus must be grounded once recognized");
        assert_eq!(
            flight.value, 4,
            "corpus `BONUS:SKILL|Swim|4|TYPE=Racial`: {:?}",
            flight
        );
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Witch with the canonical Flight hex must reach Computed: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Cauldron is the third grounded hex and must retire the blocker on
    /// the same footing as Ward and Flight -- its magnitude is real
    /// (`+4` insight on Craft (Alchemy)), it simply lands on a skill this
    /// engine does not compute, which is a coverage fact about the skill
    /// list, not a reason to call the hex unrecognized.
    #[test]
    fn single_class_witch_with_the_cauldron_hex_also_retires_the_blocker() {
        let mut input = human_witch_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WITCH_HEX_CHOICE_ID.to_owned(),
            selection_id: CAULDRON_HEX_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Cauldron is a grounded hex and must satisfy the canonical-narrowing posture: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A hex the corpus really has but this codebase has NOT grounded
    /// (Slumber carries no magnitude beyond the shared hex DC) must still
    /// claim-block. Canonical narrowing grounds one choice; it does not
    /// silently accept every string.
    #[test]
    fn an_ungrounded_witch_hex_selection_still_claim_blocks() {
        let mut input = human_witch_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WITCH_HEX_CHOICE_ID.to_owned(),
            selection_id: "hex:slumber".to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.witch.hex_powers.unsupported"
                    && d.claim_blocking),
            "an ungrounded hex must still claim-block: {:?}",
            receipt.computation.diagnostics
        );
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "an ungrounded hex must leave the Witch Blocked: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The canonical Flight posture must hold at every PF1 level, not
    /// just level 1 -- the sweep `v06_class_state_dump` reports on.
    #[test]
    fn witch_with_the_canonical_flight_hex_stays_computed_at_every_level() {
        for level in 1..=20u8 {
            let mut input = human_witch_input(level);
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: WITCH_HEX_CHOICE_ID.to_owned(),
                selection_id: FLIGHT_HEX_SELECTION.to_owned(),
            });

            let receipt = build_pilot_headless_receipt(&input);

            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Computed,
                "Witch level {level} with the canonical Flight hex must be Computed: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// task #88 correction: the Ward hex's own detail string used to claim
    /// "this headless engine computes no player AC/save total to integrate
    /// this into" -- false even at the time it was written (2026-07-25's
    /// Skald/Bloodrager closures had already wired Rage-shaped AC/save
    /// bonuses into these exact totals, a day before the Witch closure
    /// landed). `is_supported_witch_single_class` is part of
    /// `has_supported_class_chassis`, so a GE-06-posture Witch reaches the
    /// same `defense.baseline_armor_class` and `defense.total_save.*`
    /// pillars every other supported class does -- Ward's own magnitude
    /// simply isn't wired into either. This proves both halves: the totals
    /// really are computed (not fabricated as "nonexistent"), and Ward's
    /// bonus really is absent from them (the corrected detail's "not wired
    /// in yet" framing, not the old "no total exists" framing, is the
    /// accurate one).
    #[test]
    fn witch_ward_hex_detail_no_longer_falsely_claims_no_ac_save_total_exists() {
        let mut input = human_witch_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WITCH_HEX_CHOICE_ID.to_owned(),
            selection_id: WARD_HEX_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        let ward = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.witch.ward_hex.deflection_and_resistance_bonus")
            .expect("Ward's bonus must be grounded once recognized");
        assert!(
            !ward.detail.contains("computes no player AC/save total"),
            "the corrected detail must not repeat the false no-total-exists claim: {:?}",
            ward
        );
        assert!(
            ward.detail.contains("defense.baseline_armor_class"),
            "the corrected detail must name the real AC total it isn't wired into: {:?}",
            ward
        );

        let baseline_ac = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .expect(
                "a GE-06-posture Witch is a supported class chassis, so baseline AC must be \
                 real, not absent",
            );
        assert!(
            !baseline_ac.detail.contains("Ward"),
            "Ward's +2 must NOT be folded into baseline AC yet -- the corrected claim says \
             'not wired in', not 'wired in': {:?}",
            baseline_ac
        );

        let total_will_save = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.total_save.will")
            .expect("total saves must also be real for a supported Witch, not absent");
        assert!(
            !total_will_save.detail.contains("Ward"),
            "Ward's resistance bonus must NOT be folded into total saves yet: {:?}",
            total_will_save
        );
    }

    /// Ward's bonus progression at higher levels, verified against the
    /// PCGen corpus formula directly (not merely trusting the level-1
    /// base case above): +1 at level 8, +1 more at level 16.
    #[test]
    fn witch_ward_hex_bonus_progression_matches_the_corpus_formula_at_higher_levels() {
        for (level, expected_bonus) in [(1, 2), (7, 2), (8, 3), (15, 3), (16, 4), (20, 4)] {
            let mut input = human_witch_input(level);
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: WITCH_HEX_CHOICE_ID.to_owned(),
                selection_id: WARD_HEX_SELECTION.to_owned(),
            });

            let receipt = build_pilot_headless_receipt(&input);

            let ward = receipt
                .computation
                .explanations
                .iter()
                .find(|e| {
                    e.id == "class_feature.apg.witch.ward_hex.deflection_and_resistance_bonus"
                })
                .expect("Ward's bonus must be grounded");
            assert_eq!(ward.value, expected_bonus, "Witch level {level} Ward bonus: {:?}", ward);
        }
    }

    /// A non-Witch character carrying a spoofed Ward hex choice must
    /// have it silently ignored -- the class-ownership gate is by
    /// construction, not a bolt-on rejection. Also proves Fighter's own
    /// golden path is unaffected.
    #[test]
    fn non_witch_characters_spoofed_ward_hex_choice_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WITCH_HEX_CHOICE_ID.to_owned(),
            selection_id: WARD_HEX_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Witch choice: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.apg.witch.")),
            "a non-Witch character must never ground any Witch explanation: {:?}",
            receipt.computation.explanations
        );
    }
}

/// v0.6 alpha swarm, task #17 (Summoner bounded Eidolon MVP,
/// 2026-07-27): the canonical Quadruped Eidolon's corpus-derived stat
/// block, plus the evolution-point pool whose spending stays deferred.
#[cfg(test)]
mod summoner_eidolon_tests {
    use super::{build_pilot_headless_receipt, CharacterClassLevel, CharacterInput,
        SUMMONER_CLASS_ID};
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn summoner(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SUMMONER_CLASS_ID.to_owned(), level }];
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

    /// The evolution-point pool, re-derived level by level from the
    /// corpus formula's own `>=` terms rather than copied from the
    /// published table.
    #[test]
    fn evolution_pool_matches_the_corpus_formula_at_every_level() {
        let expected = [
            3, 4, 5, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 25, 26,
        ];
        for (index, want) in expected.iter().enumerate() {
            let level = (index + 1) as u8;
            assert_eq!(
                super::eidolon_evolution_pool(level),
                *want,
                "summoner level {level} evolution points"
            );
        }
    }

    /// Natural armor steps +2 at eight specific master levels.
    #[test]
    fn eidolon_natural_armor_steps_at_its_eight_corpus_gates() {
        for (level, want) in [
            (1u8, 0i16),
            (2, 2),
            (4, 2),
            (5, 4),
            (7, 6),
            (10, 8),
            (12, 10),
            (15, 12),
            (17, 14),
            (20, 16),
        ] {
            assert_eq!(
                super::eidolon_natural_armor_bonus(level),
                want,
                "level {level} natural armor bonus (on top of the racial +2)"
            );
        }
    }

    /// Max natural attacks: 3, then +1 at 4/9/14/19.
    #[test]
    fn eidolon_max_natural_attacks_steps_at_four_nine_fourteen_nineteen() {
        for (level, want) in
            [(1u8, 3i16), (3, 3), (4, 4), (8, 4), (9, 5), (13, 5), (14, 6), (18, 6), (19, 7), (20, 7)]
        {
            assert_eq!(super::eidolon_max_natural_attacks(level), want, "level {level}");
        }
    }

    /// The Eidolon has FULL base attack bonus (`classlevel`), unlike the
    /// Wolf companion's 3/4 -- the single most important reason the Wolf
    /// function could not be reused.
    #[test]
    fn eidolon_base_attack_bonus_is_full_not_three_quarters() {
        for level in [1u8, 5, 11, 20] {
            assert_eq!(
                super::eidolon_base_attack_bonus(level),
                i16::from(level),
                "level {level}: full BAB"
            );
        }
    }

    /// A level-1 Quadruped Eidolon's whole stat block grounds, and the
    /// pool is reported without pretending any of it is spent.
    #[test]
    fn a_level_one_quadruped_eidolon_grounds_its_real_stat_block() {
        let input = summoner(1);
        for (id, want) in [
            ("class_feature.apg.summoner.eidolon.evolution_pool", 3),
            ("class_feature.apg.summoner.eidolon.base_attack_bonus", 1),
            ("class_feature.apg.summoner.eidolon.natural_armor_bonus", 0),
            ("class_feature.apg.summoner.eidolon.max_natural_attacks", 3),
            ("class_feature.apg.summoner.eidolon.base_land_speed", 40),
            ("class_feature.apg.summoner.eidolon.bite_damage_die", 6),
        ] {
            assert_eq!(value(&input, id), Some(want), "{id}");
        }
    }

    /// Summoner stays Blocked on unspent evolutions -- the MVP grounds
    /// the pool SIZE, never its expenditure.
    #[test]
    fn summoner_stays_blocked_on_unspent_evolutions_at_every_level() {
        for level in [1u8, 10, 20] {
            let receipt = build_pilot_headless_receipt(&summoner(level));
            assert_eq!(
                receipt.status,
                super::HeadlessReceiptStatus::Blocked,
                "level {level} Summoner must stay Blocked"
            );
            assert!(
                receipt.computation.diagnostics.iter().any(|d| {
                    d.id == "class_feature.apg.summoner.eidolon.evolutions_deferred.unsupported"
                        && d.claim_blocking
                }),
                "level {level} must carry the claim-blocking evolutions_deferred diagnostic: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// The Eidolon's records never leak onto a non-Summoner.
    #[test]
    fn eidolon_records_never_ground_for_a_non_summoner() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let fighter = result.character_input.expect("valid fixture");
        assert!(
            !build_pilot_headless_receipt(&fighter)
                .computation
                .explanations
                .iter()
                .any(|e| e.id.contains("eidolon")),
            "a Fighter must ground no Eidolon record"
        );
    }

    // ---- Slice A: the five flat, self-scoped Summoner class features ----
    // Every gate and formula below was re-derived directly from
    // `apg_abilities_class.lst` rather than taken from the scoping doc.

    /// Bond Senses: `BONUS:VAR|BondSensesRounds|classlevel("Summoner")`,
    /// gated `PREVARGTEQ:Summoner_CFP_Level,2`.
    #[test]
    fn bond_senses_grounds_rounds_per_day_from_its_gate_onward() {
        let id = "class_feature.apg.summoner.bond_senses_rounds_per_day";
        assert_eq!(value(&summoner(1), id), None, "absent below the level-2 gate");
        assert_eq!(value(&summoner(2), id), Some(2));
        assert_eq!(value(&summoner(20), id), Some(20));
    }

    /// Maker's Call: `BONUS:VAR|MakersCallTimes|((classlevel("Summoner")-2)/4)`,
    /// gated at level 6 -- 1/day at 6th, +1 every 4 levels after.
    #[test]
    fn makers_call_uses_per_day_match_the_corpus_formula() {
        let id = "class_feature.apg.summoner.makers_call_uses_per_day";
        assert_eq!(value(&summoner(5), id), None, "absent below the level-6 gate");
        for (level, want) in [(6u8, 1i16), (9, 1), (10, 2), (14, 3), (18, 4), (20, 4)] {
            assert_eq!(value(&summoner(level), id), Some(want), "summoner level {level}");
        }
    }

    /// Merge Forms: `BONUS:VAR|MergeFormsRounds|classlevel("Summoner")`,
    /// gated at level 16.
    #[test]
    fn merge_forms_grounds_rounds_per_day_from_level_sixteen() {
        let id = "class_feature.apg.summoner.merge_forms_rounds_per_day";
        assert_eq!(value(&summoner(15), id), None, "absent below the level-16 gate");
        assert_eq!(value(&summoner(16), id), Some(16));
        assert_eq!(value(&summoner(20), id), Some(20));
    }

    /// Twin Eidolon: `BONUS:VAR|TwinEidolonMinutes|classlevel("Summoner")`,
    /// gated at the level-20 capstone.
    #[test]
    fn twin_eidolon_grounds_minutes_per_day_only_at_level_twenty() {
        let id = "class_feature.apg.summoner.twin_eidolon_minutes_per_day";
        assert_eq!(value(&summoner(19), id), None, "absent below the level-20 gate");
        assert_eq!(value(&summoner(20), id), Some(20));
    }

    /// Summon Monster's three grounded facets, all from level 1:
    /// duration `classlevel`, uses `CHA+3`, spell level
    /// `min(9,(classlevel+1)/2)`. The fixture's Charisma 8 is a -1
    /// modifier, so uses = -1 + 3 = 2 at every level.
    #[test]
    fn summon_monster_grounds_duration_uses_and_spell_level() {
        let duration = "class_feature.apg.summoner.summon_monster_duration_minutes";
        let uses = "class_feature.apg.summoner.summon_monster_uses_per_day";
        let spell_level = "class_feature.apg.summoner.summon_monster_spell_level";

        assert_eq!(value(&summoner(1), duration), Some(1));
        assert_eq!(value(&summoner(1), uses), Some(2));
        assert_eq!(value(&summoner(1), spell_level), Some(1));

        assert_eq!(value(&summoner(11), duration), Some(11));
        assert_eq!(value(&summoner(11), spell_level), Some(6));
    }

    /// The spell-level term is `min(9, ...)` -- it genuinely caps rather
    /// than running to 10 at the level-20 capstone.
    #[test]
    fn summon_monster_spell_level_caps_at_nine() {
        let id = "class_feature.apg.summoner.summon_monster_spell_level";
        assert_eq!(value(&summoner(17), id), Some(9));
        assert_eq!(value(&summoner(20), id), Some(9), "(20+1)/2 = 10, capped to 9");
    }

    /// Slice A grounds real magnitudes but must NOT unblock Summoner --
    /// the unspent-evolutions blocker is untouched and still claim-blocking.
    #[test]
    fn slice_a_leaves_summoner_blocked_on_unspent_evolutions() {
        for level in [2u8, 6, 16, 20] {
            let receipt = build_pilot_headless_receipt(&summoner(level));
            assert_eq!(
                receipt.status,
                super::HeadlessReceiptStatus::Blocked,
                "level {level} Summoner must stay Blocked"
            );
        }
    }
}

/// v0.6 alpha swarm (Summoner Eidolon evolution canonical-narrowing
/// closure, 2026-07-29): a recognized `choice:summoner_eidolon_evolution`
/// naming the canonical Improved Natural Armor evolution spends a real
/// point out of the already-grounded pool, grounds the evolution's real
/// +2 natural armor into the Eidolon's own natural-armor total, and
/// turns the `evolutions_deferred` blocker non-blocking -- the exact
/// shape Arcanist's Metamagic Knowledge Exploit and Cleric's domain
/// already ratified.
#[cfg(test)]
mod summoner_eidolon_evolution_choice_tests {
    use super::{build_pilot_headless_receipt, CharacterClassLevel, CharacterInput,
        HeadlessReceiptStatus, IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION,
        SUMMONER_CLASS_ID, SUMMONER_EIDOLON_EVOLUTION_CHOICE_ID};
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    const EVOLUTIONS_DEFERRED: &str =
        "class_feature.apg.summoner.eidolon.evolutions_deferred.unsupported";

    fn summoner(level: u8) -> CharacterInput {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SUMMONER_CLASS_ID.to_owned(), level }];
        input
    }

    fn summoner_with(level: u8, selection: &str) -> CharacterInput {
        let mut input = summoner(level);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SUMMONER_EIDOLON_EVOLUTION_CHOICE_ID.to_owned(),
            selection_id: selection.to_owned(),
        });
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

    /// **The milestone test.** A Summoner whose Eidolon has actually
    /// spent a point on the canonical Improved Natural Armor evolution
    /// reaches `Computed` at every one of the 20 levels -- the last
    /// blocker on the whole 27-class roster's hardest class.
    #[test]
    fn summoner_with_a_recognized_eidolon_evolution_reaches_computed_at_every_level() {
        for level in 1u8..=20 {
            let receipt = build_pilot_headless_receipt(&summoner_with(
                level,
                IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION,
            ));
            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Computed,
                "summoner level {level} with a spent evolution must reach Computed: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// The deferral diagnostic survives -- it is narrowed to non-blocking,
    /// never deleted. The remaining unspent points and the other 103
    /// evolutions must still be named honestly.
    #[test]
    fn the_evolutions_deferred_diagnostic_is_narrowed_not_removed() {
        let receipt = build_pilot_headless_receipt(&summoner_with(
            20,
            IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION,
        ));
        let deferred = receipt
            .computation
            .diagnostics
            .iter()
            .find(|d| d.id == EVOLUTIONS_DEFERRED)
            .expect("the deferral diagnostic must still be present, just non-blocking");
        assert!(
            !deferred.claim_blocking,
            "a recognized evolution narrows the blocker: {deferred:?}"
        );
        assert!(
            deferred.message.contains("103"),
            "the narrowed message must name the other 103 evolutions: {}",
            deferred.message
        );
    }

    /// Improved Natural Armor's real corpus magnitude
    /// (`BONUS:VAR|AC_Natural_Armor|2|TYPE=Base.STACK`) lands on the
    /// Eidolon's natural-armor total rather than sitting beside it.
    #[test]
    fn improved_natural_armor_grounds_its_corpus_magnitude_and_the_new_total() {
        let bonus = "class_feature.apg.summoner.eidolon.improved_natural_armor";
        let total = "class_feature.apg.summoner.eidolon.total_natural_armor";

        // Level 1: racial +2, level-driven +0, evolution +2 => +4.
        let level_one = summoner_with(1, IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION);
        assert_eq!(value(&level_one, bonus), Some(2), "the evolution's own +2");
        assert_eq!(value(&level_one, total), Some(4), "2 racial + 0 level + 2 evolution");

        // Level 20: racial +2, level-driven +16, evolution +2 => +20.
        let capstone = summoner_with(20, IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION);
        assert_eq!(value(&capstone, total), Some(20), "2 racial + 16 level + 2 evolution");
    }

    /// The point-buy economy is genuinely honoured: one point is spent
    /// out of the level's real pool and the remainder is reported, never
    /// silently ignored.
    #[test]
    fn the_spent_point_is_drawn_from_the_real_pool() {
        let spent = "class_feature.apg.summoner.eidolon.evolution_points_spent";
        let unspent = "class_feature.apg.summoner.eidolon.evolution_points_unspent";

        // Level 1 pool is 3: one spent, two left.
        let level_one = summoner_with(1, IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION);
        assert_eq!(value(&level_one, spent), Some(1));
        assert_eq!(value(&level_one, unspent), Some(2));

        // The pool itself is unchanged by the purchase.
        assert_eq!(
            value(&level_one, "class_feature.apg.summoner.eidolon.evolution_pool"),
            Some(3)
        );
    }

    /// An unrecognized selection must NOT unblock the class -- it keeps
    /// the original claim-blocking deferral and adds its own blocker,
    /// mirroring Arcanist's `feat_ineligible` branch.
    #[test]
    fn an_unrecognized_evolution_selection_stays_blocked() {
        let receipt =
            build_pilot_headless_receipt(&summoner_with(5, "evolution:large"));
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "an unrecognized evolution must not fabricate a purchase: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == EVOLUTIONS_DEFERRED && d.claim_blocking),
            "the original blocker must survive an unrecognized selection: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// With no choice recorded at all, Summoner is exactly as blocked as
    /// it was before this closure -- no silent canonical seeding inside
    /// the engine.
    #[test]
    fn a_summoner_with_no_recorded_evolution_choice_stays_blocked() {
        for level in [1u8, 10, 20] {
            let receipt = build_pilot_headless_receipt(&summoner(level));
            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Blocked,
                "level {level} without a choice must stay Blocked"
            );
            assert!(
                receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id == EVOLUTIONS_DEFERRED && d.claim_blocking),
                "level {level}: the original blocker must be untouched"
            );
        }
    }

    /// A spoofed evolution choice on a non-Summoner grounds nothing and
    /// leaves that character's own golden path alone.
    #[test]
    fn a_spoofed_evolution_choice_on_a_non_summoner_is_ignored() {
        let mut fighter = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        fighter.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SUMMONER_EIDOLON_EVOLUTION_CHOICE_ID.to_owned(),
            selection_id: IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&fighter);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's golden path must be unaffected: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt.computation.explanations.iter().any(|e| e.id.contains("eidolon")),
            "no eidolon record may ground for a Fighter"
        );
    }
}

/// v0.6 alpha swarm (Monk/Summoner chassis-recognition closure,
/// 2026-07-29): the two classes whose chassis pillars were computable but
/// unreachable, for two DIFFERENT reasons that happened to surface the
/// same downstream trio.
///
/// Monk: `table_class_id` never mapped `class:monk`, so
/// `is_supported_generic_single_class` -- and therefore
/// `has_supported_class_chassis` -- rejected Monk outright, even though
/// `class_tables()` has always carried a complete corpus-backed Monk row
/// (3/4 BAB, all three saves good, levels 1-20). Monk's chassis
/// computation itself was never the gap; only the string->`ClassId`
/// mapping was.
///
/// Summoner: the opposite shape. `compute_class_chassis`'s APG branch
/// already resolved Summoner correctly (which is why Summoner never
/// emitted `class_chassis.unsupported` at all), but no
/// `is_supported_summoner_single_class` arm existed in
/// `has_supported_class_chassis`, so every gate keyed off that predicate
/// -- combat baseline, total saves, selected skill modifiers -- refused a
/// class whose chassis was sitting right there, already computed.
///
/// Both expected value sets below are re-derived from the corpus
/// formulas, not copied from a published table:
///   Monk, `cr_classes.lst:147` (`CLASS:Monk ... MAXLEVEL:20`), 3/4 BAB
///   `(level*3)/4` and good saves `level/2+2` on all three.
///   Summoner, `apg_classes.lst:139`,
///   `BONUS:COMBAT|BASEAB|classlevel*3/4`, `BONUS:SAVE|BASE.Will|classlevel/2+2`,
///   `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel/3`, `MAXLEVEL:20`.
#[cfg(test)]
mod monk_and_summoner_chassis_recognition_tests {
    use super::{
        build_pilot_headless_receipt, has_supported_class_chassis, table_class_id,
        CharacterClassLevel, CharacterInput, MONK_CLASS_ID, SUMMONER_CLASS_ID,
    };
    use crate::rules_core::character_input::load_character_input_fixture;
    use crate::rules_core::rules_tables::crb::class_tables::ClassId;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn single_class(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn explanation(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    fn blocking_ids(input: &CharacterInput) -> Vec<String> {
        let mut ids: Vec<String> = build_pilot_headless_receipt(input)
            .computation
            .diagnostics
            .into_iter()
            .filter(|d| d.claim_blocking)
            .map(|d| d.id)
            .collect();
        ids.sort();
        ids
    }

    /// The single omission that blocked Monk: the string mapping itself.
    #[test]
    fn table_class_id_maps_monk() {
        assert_eq!(table_class_id(MONK_CLASS_ID), Some(ClassId::Monk));
    }

    /// Both classes must now pass the gate every downstream pillar keys off.
    #[test]
    fn both_classes_pass_the_chassis_gate_at_every_level_one_through_twenty() {
        for level in 1..=20u8 {
            assert!(
                has_supported_class_chassis(&single_class(MONK_CLASS_ID, level)),
                "monk level {level} must be a supported chassis"
            );
            assert!(
                has_supported_class_chassis(&single_class(SUMMONER_CLASS_ID, level)),
                "summoner level {level} must be a supported chassis"
            );
        }
    }

    /// Monk's real chassis numbers, re-derived from the corpus formulas at
    /// every one of the 20 levels -- not merely "present".
    #[test]
    fn monk_chassis_values_match_the_corpus_formulas_at_every_level() {
        for level in 1..=20u8 {
            let input = single_class(MONK_CLASS_ID, level);
            let want_bab = (level as i16 * 3) / 4;
            let want_good_save = level as i16 / 2 + 2;

            assert_eq!(
                explanation(&input, "class_chassis.base_attack_bonus"),
                Some(want_bab),
                "monk level {level} base attack bonus (3/4 BAB)"
            );
            for save in ["fortitude", "reflex", "will"] {
                assert_eq!(
                    explanation(&input, &format!("class_chassis.base_save.{save}")),
                    Some(want_good_save),
                    "monk level {level} base {save} save (Monk has all three good)"
                );
            }
        }
    }

    /// Summoner's real chassis numbers: same 3/4 BAB as Monk, but only Will
    /// is good -- Fortitude and Reflex are poor (`level/3`). Asserting the
    /// poor saves explicitly is what proves the two classes did not get
    /// collapsed onto one shared progression by this widening.
    #[test]
    fn summoner_chassis_values_match_the_corpus_formulas_at_every_level() {
        for level in 1..=20u8 {
            let input = single_class(SUMMONER_CLASS_ID, level);
            let want_bab = (level as i16 * 3) / 4;
            let want_poor_save = level as i16 / 3;
            let want_good_save = level as i16 / 2 + 2;

            assert_eq!(
                explanation(&input, "class_chassis.base_attack_bonus"),
                Some(want_bab),
                "summoner level {level} base attack bonus (3/4 BAB)"
            );
            assert_eq!(
                explanation(&input, "class_chassis.base_save.fortitude"),
                Some(want_poor_save),
                "summoner level {level} base Fortitude save (poor)"
            );
            assert_eq!(
                explanation(&input, "class_chassis.base_save.reflex"),
                Some(want_poor_save),
                "summoner level {level} base Reflex save (poor)"
            );
            assert_eq!(
                explanation(&input, "class_chassis.base_save.will"),
                Some(want_good_save),
                "summoner level {level} base Will save (good)"
            );
        }
    }

    /// The four chassis-integration diagnostics must be GONE for both
    /// classes at every level -- this is the actual deliverable, stated as
    /// an absence rather than inferred from the value assertions above.
    #[test]
    fn the_four_chassis_integration_blockers_are_gone_for_both_classes() {
        for class_id in [MONK_CLASS_ID, SUMMONER_CLASS_ID] {
            for level in 1..=20u8 {
                let ids = blocking_ids(&single_class(class_id, level));
                for gone in [
                    "class_chassis.unsupported",
                    "combat.baseline_unsupported",
                    "defense.total_save.unsupported",
                    "skill.selected_modifier.unsupported",
                ] {
                    assert!(
                        !ids.contains(&gone.to_owned()),
                        "{class_id} level {level} must no longer emit {gone}: {ids:?}"
                    );
                }
            }
        }
    }

    /// What genuinely remains, pinned exactly so it cannot quietly grow.
    ///
    /// Neither class reaches `Computed`, and this test refuses to pretend
    /// otherwise. Each retains exactly ONE real, non-chassis feature gap:
    /// Monk its bonus-feat grant (no `choice:monk_bonus_feat` is seeded in
    /// this posture), Summoner its Eidolon evolution-point spending. Both
    /// are genuine unbuilt/unchosen surfaces, not recognition gaps.
    #[test]
    fn each_class_retains_exactly_one_real_non_chassis_blocker() {
        for (class_id, want) in [
            (MONK_CLASS_ID, "class_feature.monk.bounded_progression.bonus_feat.unsupported"),
            (
                SUMMONER_CLASS_ID,
                "class_feature.apg.summoner.eidolon.evolutions_deferred.unsupported",
            ),
        ] {
            for level in 1..=20u8 {
                assert_eq!(
                    blocking_ids(&single_class(class_id, level)),
                    vec![want.to_owned()],
                    "{class_id} level {level} must retain exactly its one real feature gap"
                );
            }
        }
    }
}

