#[allow(unused_imports)]
pub(crate) use super::*;

/// The highest master level the PF1 animal-companion progression is
/// defined for (`MAXLEVEL:20` on `CLASS:Companion`,
/// `core_rulebook/cr_classes_companion.lst:6`).
pub(super) const MAX_ANIMAL_COMPANION_MASTER_LEVEL: u8 = 20;

/// The PF1 Core Rulebook "Animal Companion Base Statistics" Hit Dice
/// column, indexed by `master_level - 1`.
///
/// Transcribed from the PCGen corpus, not from memory, from two encodings
/// that must be combined:
///
/// * every CRB companion race carries `MONSTERCLASS:Companion:2`
///   (`core_rulebook/cr_races_companion.lst` -- all 38 companion races,
///   Wolf at line 32 and Horse at line 21), so a companion starts at 2 HD
///   even for a 1st-level master; and
/// * `core_rulebook/cr_companionmods.lst:11-24` grants exactly one further
///   Hit Die (`HD:1`) at master levels 2, 4, 5, 6, 8, 9, 10, 12, 13, 14,
///   16, 17, 18 and 20 -- and at no other level.
///
/// Every companion progression variable in the entire corpus repeats that
/// same 14-level list -- `SpecialMountLVL`
/// (`core_rulebook/cr_companionmods.lst:34-47`), `CavalierMountLVL`
/// (`advanced_players_guide/apg_companionmods.lst:75-93`),
/// `BeastRiderLvl`, `FeatheredCompanionLvl`, `EmpyrealCompanion`, and all
/// twenty Packmaster `AnimalCompanionLVL{A..T}` variants -- so this is one
/// universal table, which is exactly why Druid, Hunter and Cavalier can
/// all share it.
///
/// The `CLASS:Companion` formulas that consume it
/// (`core_rulebook/cr_classes_companion.lst:6`) key off this HD count, not
/// off the master's level: `BASEAB = HD*3/4`,
/// `BASE.Fortitude/BASE.Reflex = HD/2+2`, `BASE.Will = HD/3`, all floor
/// division.
pub(super) const ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL: [u8; MAX_ANIMAL_COMPANION_MASTER_LEVEL as usize] =
    [2, 3, 3, 4, 5, 6, 6, 7, 8, 9, 9, 10, 11, 12, 12, 13, 14, 15, 15, 16];

/// Clamps a master level onto the real table's domain (1-20).
///
/// This is the replacement for the old
/// `debug_assert_eq!(companion_level, 1, "only companion level 1 is
/// grounded this slice")`. That guard had the wrong shape twice over: it
/// rejected every level the progression actually defines, and because
/// `debug_assert` compiles out of release builds it let a release build
/// silently hand a 20th-level master's companion a 1st-level companion's
/// Hit Dice. The guard is kept, not deleted -- a caller passing a level
/// outside 1-20 is still a bug and still trips in test/debug builds -- but
/// the domain it enforces is now the progression's real one, and release
/// builds saturate onto the nearest real row instead of returning a
/// number from the wrong end of the table.
pub(super) fn animal_companion_table_index(master_level: u8) -> usize {
    debug_assert!(
        (1..=MAX_ANIMAL_COMPANION_MASTER_LEVEL).contains(&master_level),
        "animal companion master level {master_level} is outside the corpus progression's own 1-{MAX_ANIMAL_COMPANION_MASTER_LEVEL} domain"
    );
    usize::from(master_level.clamp(1, MAX_ANIMAL_COMPANION_MASTER_LEVEL)) - 1
}

/// The companion class's own level-scaling natural-armor bonus, which
/// STACKS on top of the companion race's own base natural armor:
/// `BONUS:COMBAT|AC|2*floor(MasterLevel/3)|TYPE=NaturalArmor.STACK`
/// (`core_rulebook/cr_abilities_companion.lst:59`, the shared
/// `Animal Companion ~ AC Bonus` record that BOTH
/// `Base Companion ~ Animal Companion` and `Base Companion ~ Special
/// Mount` pull in). Independently confirmed by the APG Cavalier Mount
/// block, which spells the same progression out level by level as
/// `BONUS:VAR|AC_Natural_Armor|2|TYPE=Base.STACK` at master levels 3, 6,
/// 9, 12, 15 and 18 (`apg_companionmods.lst:76,79,82,85,88,91`).
///
/// This has a live consumer -- the companion's own `.armor_class`
/// explanation record -- so widening Hit Dice without widening this would
/// have shipped an understated Armor Class from master level 3 upward.
pub fn animal_companion_natural_armor_bonus(master_level: u8) -> i16 {
    let clamped = master_level.clamp(1, MAX_ANIMAL_COMPANION_MASTER_LEVEL);
    2 * (i16::from(clamped) / 3)
}

/// The companion class's own level-scaling Strength/Dexterity bonus:
/// `BONUS:STAT|STR,DEX|floor(MasterLevel/3)`
/// (`core_rulebook/cr_abilities_companion.lst:60`), independently
/// confirmed by the APG Cavalier Mount block's own inline
/// `BONUS:STAT|STR,DEX|1` at the same six master levels.
///
/// Only the Strength half currently has a consumer here (the companion's
/// attack bonus and its natural attack's damage bonus). The Dexterity
/// half is deliberately NOT applied anywhere: this codebase grounds no
/// Dexterity contribution to the companion's Armor Class in the first
/// place (the `.armor_class` record says so in its own detail text), so
/// applying it would build an unwired field. Named here, deferred in the
/// companion's own advancement diagnostic.
///
/// Distinct from the separate player-chosen `Companion Stat Increase`
/// (`BONUS:ABILITYPOOL|Companion Stat Increase|1` at master levels 4, 9,
/// 14 and 20, `apg_companionmods.lst:77,82,87,93`), which is a chooser
/// input with no canonical default and stays deferred.
pub fn animal_companion_stat_bonus(master_level: u8) -> i16 {
    let clamped = master_level.clamp(1, MAX_ANIMAL_COMPANION_MASTER_LEVEL);
    i16::from(clamped) / 3
}

/// A companion's hit points at `hit_dice` Hit Dice of size `hit_die_size`:
/// the maximized first Hit Die plus the average for every Hit Die after
/// it, each plus the companion's own Constitution modifier -- this
/// codebase's own established HP idiom (`durability.rs`'s
/// `compute_max_hp`), now applied across the companion's real Hit Dice
/// count rather than the hardcoded two it was written against.
pub(super) fn animal_companion_hit_points(hit_dice: u8, hit_die_size: u8, constitution_modifier: i16) -> i16 {
    let maximized_first = i16::from(hit_die_size) + constitution_modifier;
    let average_subsequent =
        crate::rules_core::durability::average_hit_die_value(hit_die_size) + constitution_modifier;
    maximized_first + i16::from(hit_dice.saturating_sub(1)) * average_subsequent
}

/// v0.6 alpha swarm, risks item 8 (Cavalier Mount closure, first APG
/// class-specific closure): Horse's real PF1 Core Rulebook base
/// statistics, verified against two independent primary sources --
/// aonprd.com's own Horse companion page and d20pfsrd's Animal
/// Companions page -- plus the PCGen corpus (`cr_races_companion.lst`,
/// `bestiary/b1_races.lst`) as tiebreaker, mirroring the exact Wolf
/// natural-armor/Trip resolution methodology. Ability scores agree
/// completely across both sources (Str 16, Dex 13, Con 15, Int 2, Wis
/// 12, Cha 6). Natural armor disagreed (aonprd +4, d20pfsrd +1) --
/// resolved in favor of aonprd, backed directly by the PCGen corpus's
/// own `BONUS:VAR|AC_Natural_Armor|4|TYPE=Base`
/// (`cr_races_companion.lst:21`). Speed also disagreed (aonprd 50 ft.,
/// d20pfsrd 60 ft.) -- resolved in favor of aonprd, backed by the
/// corpus's own base Horse race `MOVE:Walk,50` (`bestiary/b1_races.lst:235`);
/// speed itself is not grounded anywhere in this codebase (no movement/
/// mobility pillar exists), named here only for the verification record.
/// Horse is the Cavalier's Mount for a Medium cavalier (this codebase's
/// only race), per "A Medium cavalier can select a camel or a horse" --
/// Horse chosen as the canonical species over Camel, the same
/// "smallest defensible single case" discipline as every other class's
/// own fixed canonical choice (Wolf for Druid/Hunter, Longsword for
/// Barbarian, etc.).
pub(super) const HORSE_COMPANION_STRENGTH_SCORE: i16 = 16;

/// Grounds the Wolf companion's standalone stat block as explanation
/// records under `id_prefix` (v0.6 alpha swarm, risks item 8, fourth
/// APG/ACG closure): extracted from Druid's own original inline
/// implementation so Hunter's own animal companion (whose corpus text
/// reads "the hunter's effective druid level is equal to her hunter
/// level" -- mechanically identical to Druid's own progression, not a
/// new mechanic) can reuse the exact same, already-3-source-verified math
/// rather than re-deriving or copy-pasting it. `owner_class_label` is
/// used only in explanation prose (e.g. "Druid" or "Hunter"); the values
/// themselves depend only on `companion_level` (the owning character's
/// own level, matching `wolf_companion_hit_dice`'s own parameter shape).
/// Callers are responsible for their own class-ownership/level/choice
/// gating -- this function only grounds the stat block itself.
pub(super) fn ground_wolf_companion_stat_block(
    id_prefix: &str,
    owner_class_label: &str,
    companion_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let companion_hd = wolf_companion_hit_dice(companion_level);
    let companion_hd_value = i16::from(companion_hd);
    let companion_base_attack_bonus = companion_hd_value * 3 / 4;
    let companion_fort_ref_save = companion_hd_value / 2 + 2;
    let companion_will_save = companion_hd_value / 3;
    // The companion class's own level-scaling Strength bonus stacks on the
    // race's base score (corpus: `BONUS:STAT|STR,DEX|floor(MasterLevel/3)`).
    // Zero at master levels 1 and 2, so every previously-shipped level-1
    // value below is byte-for-byte unchanged by this widening.
    let strength_bonus = animal_companion_stat_bonus(companion_level);
    let strength_score = WOLF_COMPANION_STRENGTH_SCORE + strength_bonus;
    let natural_armor = WOLF_COMPANION_NATURAL_ARMOR
        + animal_companion_natural_armor_bonus(companion_level);
    let companion_armor_class = 10 + natural_armor;
    let strength_modifier = ability_modifier(strength_score);
    let constitution_modifier = ability_modifier(WOLF_COMPANION_CONSTITUTION_SCORE);
    let companion_attack_bonus = companion_base_attack_bonus + strength_modifier;
    // Primary natural attack: 1.5x Strength modifier, floored (PF1 Core
    // Rulebook natural-attack damage rule), added to the base 1d6 bite
    // die (the die itself is not a flat number and is named, not rolled).
    let companion_bite_damage_bonus = (strength_modifier * 3) / 2;
    // Maximized first Hit Die plus average for every Hit Die after it
    // (this codebase's own established HP idiom, `durability.rs`'s
    // `compute_max_hp`), each plus the companion's own Constitution
    // modifier.
    let companion_hp = animal_companion_hit_points(
        companion_hd,
        WOLF_COMPANION_HIT_DIE_SIZE,
        constitution_modifier,
    );

    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.wolf_stat_block"),
        value: 0,
        detail: format!(
            "{owner_class_label} level {companion_level} animal companion, Wolf (the canonical \
             PF1 Core Rulebook companion species this bounded seam grounds): a wholly separate \
             creature with its own combat statistics -- none of the values below are ever \
             applied to the {owner_class_label}'s own integrated totals. Base ability scores \
             (verified against d20pfsrd, the Archives of Nethys aonprd.com mirror, and the \
             PCGen corpus cr_races_companion.lst, Core Rulebook p.56): Str \
             {WOLF_COMPANION_STRENGTH_SCORE}, Con {WOLF_COMPANION_CONSTITUTION_SCORE}. This is a \
             bounded recognition record only (+0); the companion's own flat stat values are \
             grounded separately as standalone explanation records below"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_attack_bonus"),
        value: companion_attack_bonus,
        detail: format!(
            "Wolf companion base attack bonus at {companion_hd} HD (PF1 Core Rulebook Animal \
             Companion Base Statistics: HD*3/4 = {companion_base_attack_bonus}) + Strength \
             modifier ({strength_modifier:+}, Str {strength_score} = base \
             {WOLF_COMPANION_STRENGTH_SCORE} + {strength_bonus} from the companion class's own \
             floor(master level/3) Strength/Dexterity advance) = {companion_attack_bonus}. \
             Standalone record; the companion is a separate creature, not integrated into the \
             {owner_class_label}'s own combat totals"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.fortitude"),
        value: companion_fort_ref_save,
        detail: format!(
            "Wolf companion base Fortitude save at companion level {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/2+2 = \
             {companion_fort_ref_save}). Standalone record; not the {owner_class_label}'s own \
             save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.reflex"),
        value: companion_fort_ref_save,
        detail: format!(
            "Wolf companion base Reflex save at companion level {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/2+2 = \
             {companion_fort_ref_save}). Standalone record; not the {owner_class_label}'s own \
             save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.will"),
        value: companion_will_save,
        detail: format!(
            "Wolf companion base Will save at companion level {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/3 = {companion_will_save}). \
             Standalone record; not the {owner_class_label}'s own save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.armor_class"),
        value: companion_armor_class,
        detail: format!(
            "Wolf companion armor class: base 10 + natural armor (+{natural_armor} = the Wolf \
             race's own base +{WOLF_COMPANION_NATURAL_ARMOR} plus \
             +{} from the companion class's own 2*floor(master level/3) natural-armor advance) \
             = {companion_armor_class}. The base figure was verified against 2 of 3 sources \
             (aonprd.com's own Wolf companion page and the PCGen corpus, which disagreed with \
             d20pfsrd's +1 figure; resolved in favor of the majority, the corpus citing Core \
             Rulebook p.56 directly). Standalone record; Dexterity's own contribution to the \
             companion's AC is not grounded, so the Dexterity half of the same advance is \
             deliberately not applied either",
            animal_companion_natural_armor_bonus(companion_level)
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.bite_attack"),
        value: companion_bite_damage_bonus,
        detail: format!(
            "Wolf companion bite attack: 1d6 (a real die, named not rolled) + \
             {companion_bite_damage_bonus:+} (1.5x Strength modifier ({strength_modifier:+}), \
             floored, PF1 Core Rulebook's primary-natural-attack damage rule), plus the Trip \
             special attack (verified present at companion level 1 via aonprd.com and the PCGen \
             corpus, which disagreed with d20pfsrd's 4th-level-advancement framing). No \
             attack-roll or damage-roll resolution engine exists in this codebase, so this \
             grounds the flat damage bonus only, not an actual attack or damage outcome"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.hit_points"),
        value: companion_hp,
        detail: format!(
            "Wolf companion hit points at {companion_hd} HD (d{WOLF_COMPANION_HIT_DIE_SIZE}): \
             maximized first Hit Die plus average for each of the remaining {} (this codebase's \
             own established HP idiom, durability.rs's compute_max_hp), each plus the \
             companion's Constitution modifier ({constitution_modifier:+}, Con \
             {WOLF_COMPANION_CONSTITUTION_SCORE}) = {companion_hp}",
            companion_hd.saturating_sub(1)
        ),
    });
}

/// Grounds the Horse companion's standalone stat block as explanation
/// records under `id_prefix` (v0.6 alpha swarm, risks item 8, Cavalier
/// Mount closure, first APG class-specific closure) -- a parallel,
/// deliberately NOT genericized copy of `ground_wolf_companion_stat_block`'s
/// own structure, using Horse's own verified constants instead of Wolf's.
/// Kept as a separate function (per the lead's own review) rather than
/// genericizing the shared Wolf function, so this closure carries zero
/// risk to Druid's/Hunter's already-shipped, already-tested Wolf output.
pub(super) fn ground_horse_companion_stat_block(
    id_prefix: &str,
    owner_class_label: &str,
    companion_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let companion_hd = horse_companion_hit_dice(companion_level);
    let companion_hd_value = i16::from(companion_hd);
    let companion_base_attack_bonus = companion_hd_value * 3 / 4;
    let companion_fort_ref_save = companion_hd_value / 2 + 2;
    let companion_will_save = companion_hd_value / 3;
    // Same corpus advance as the Wolf block's own -- the Cavalier Mount
    // companionmod block spells it out level by level rather than as a
    // formula, but resolves to the identical numbers.
    let strength_bonus = animal_companion_stat_bonus(companion_level);
    let strength_score = HORSE_COMPANION_STRENGTH_SCORE + strength_bonus;
    let natural_armor = HORSE_COMPANION_NATURAL_ARMOR
        + animal_companion_natural_armor_bonus(companion_level);
    let companion_armor_class = 10 + natural_armor;
    let strength_modifier = ability_modifier(strength_score);
    let constitution_modifier = ability_modifier(HORSE_COMPANION_CONSTITUTION_SCORE);
    let companion_attack_bonus = companion_base_attack_bonus + strength_modifier;
    // Primary natural attack: 1.5x Strength modifier, floored (PF1 Core
    // Rulebook natural-attack damage rule), added to the base 1d6 hoof
    // die (the die itself is not a flat number and is named, not
    // rolled). Hooves, not bite, are grounded as the primary attack: the
    // PCGen corpus's own base (non-companion) Horse race entry
    // (bestiary/b1_races.lst:235) has ONLY a hoof natural attack, no
    // bite at all, confirming hooves are the Horse's fundamental attack
    // -- the companion-specific bite (1d4, per aonprd.com) is a
    // secondary attack layered on top, not separately grounded this
    // slice (mirrors the Wolf record's own single-attack-only scope; no
    // primary/secondary natural-attack Strength-multiplier distinction
    // is modeled anywhere in this codebase).
    let companion_hoof_damage_bonus = (strength_modifier * 3) / 2;
    let companion_hp = animal_companion_hit_points(
        companion_hd,
        HORSE_COMPANION_HIT_DIE_SIZE,
        constitution_modifier,
    );

    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.horse_stat_block"),
        value: 0,
        detail: format!(
            "{owner_class_label} level {companion_level} Mount, Horse (the canonical PF1 Core \
             Rulebook Medium-cavalier companion species this bounded seam grounds; Camel is the \
             other Medium option but is not built this slice -- no species-selection input is \
             modeled for either, mirroring Wolf's own \"assumed, not chosen\" precedent): a \
             wholly separate creature with its own combat statistics -- none of the values \
             below are ever applied to the {owner_class_label}'s own integrated totals. Base \
             ability scores (verified against aonprd.com's own Horse companion page and \
             d20pfsrd's Animal Companions page, both agreeing completely; the PCGen corpus \
             cr_races_companion.lst backs the natural-armor/speed values below): Str \
             {HORSE_COMPANION_STRENGTH_SCORE}, Con {HORSE_COMPANION_CONSTITUTION_SCORE}. This is \
             a bounded recognition record only (+0); the companion's own flat stat values are \
             grounded separately as standalone explanation records below"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_attack_bonus"),
        value: companion_attack_bonus,
        detail: format!(
            "Horse companion base attack bonus at {companion_hd} HD (PF1 Core Rulebook Animal \
             Companion Base Statistics: HD*3/4 = {companion_base_attack_bonus}) + Strength \
             modifier ({strength_modifier:+}, Str {strength_score} = base \
             {HORSE_COMPANION_STRENGTH_SCORE} + {strength_bonus} from the companion class's own \
             floor(master level/3) Strength/Dexterity advance) = {companion_attack_bonus}. \
             Standalone record; the companion is a separate creature, not integrated into the \
             {owner_class_label}'s own combat totals"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.fortitude"),
        value: companion_fort_ref_save,
        detail: format!(
            "Horse companion base Fortitude save at companion level {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/2+2 = \
             {companion_fort_ref_save}). Standalone record; not the {owner_class_label}'s own \
             save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.reflex"),
        value: companion_fort_ref_save,
        detail: format!(
            "Horse companion base Reflex save at companion level {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/2+2 = \
             {companion_fort_ref_save}). Standalone record; not the {owner_class_label}'s own \
             save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.will"),
        value: companion_will_save,
        detail: format!(
            "Horse companion base Will save at companion level {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/3 = {companion_will_save}). \
             Standalone record; not the {owner_class_label}'s own save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.armor_class"),
        value: companion_armor_class,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   The base figure was verified against aonprd.com's own Horse companion page, backed
            //   directly by the PCGen corpus's BONUS:VAR|AC_Natural_Armor|4|TYPE=Base
            //   (cr_races_companion.lst:21) -- d20pfsrd's own +1 figure disagreed and was rejected
            //   as the uncorroborated minority reading, the same resolution methodology as Wolf's
            //   own natural-armor question.
            "Horse companion armor class: base 10 + natural armor (+{natural_armor} = the Horse \
             race's own base +{HORSE_COMPANION_NATURAL_ARMOR} plus +{} from the companion class's \
             own 2*floor(master level/3) natural-armor advance) = {companion_armor_class}. \
             Standalone record; Dexterity's own contribution to the companion's AC is not grounded, \
             so the Dexterity half of the same advance is deliberately not applied either",
            animal_companion_natural_armor_bonus(companion_level)
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.hoof_attack"),
        value: companion_hoof_damage_bonus,
        detail: format!(
            "Horse companion hoof attack: 1d6 (a real die, named not rolled) + \
             {companion_hoof_damage_bonus:+} (1.5x Strength modifier ({strength_modifier:+}), \
             floored, PF1 Core Rulebook's primary-natural-attack damage rule). Hooves are \
             grounded as the primary natural attack, not the companion's own secondary bite \
             (1d4, per aonprd.com) -- the PCGen corpus's base, non-companion Horse race entry \
             (bestiary/b1_races.lst:235) has only a hoof attack, no bite at all, confirming \
             hooves as the Horse's fundamental natural weapon. No attack-roll or damage-roll \
             resolution engine exists in this codebase, so this grounds the flat damage bonus \
             only, not an actual attack or damage outcome, and no primary/secondary natural-\
             attack Strength-multiplier distinction is modeled (mirroring the Wolf record's own \
             single-attack-only scope)"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.hit_points"),
        value: companion_hp,
        detail: format!(
            "Horse companion hit points at {companion_hd} HD (d{HORSE_COMPANION_HIT_DIE_SIZE}): \
             maximized first Hit Die plus average for each of the remaining {} (this codebase's \
             own established HP idiom, durability.rs's compute_max_hp), each plus the \
             companion's Constitution modifier ({constitution_modifier:+}, Con \
             {HORSE_COMPANION_CONSTITUTION_SCORE}) = {companion_hp}",
            companion_hd.saturating_sub(1)
        ),
    });
}

#[cfg(test)]
mod animal_companion_progression_tests {
    use super::{
        ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL, MAX_ANIMAL_COMPANION_MASTER_LEVEL,
        animal_companion_natural_armor_bonus, animal_companion_stat_bonus,
        horse_companion_hit_dice, wolf_companion_hit_dice,
    };

    /// The real PF1 Core Rulebook "Animal Companion Base Statistics" Hit
    /// Dice column, transcribed from the PCGen corpus rather than from
    /// memory.
    ///
    /// Derivation, from two independent corpus encodings that agree:
    ///
    /// 1. Every CRB companion race carries `MONSTERCLASS:Companion:2`
    ///    (`core_rulebook/cr_races_companion.lst`, all 38 companion races;
    ///    Wolf at line 32, Horse at line 21) -- a 2 HD floor even for a
    ///    1st-level master.
    /// 2. `core_rulebook/cr_companionmods.lst:11-24` grants exactly one
    ///    additional Hit Die (`HD:1`) at master levels 2, 4, 5, 6, 8, 9,
    ///    10, 12, 13, 14, 16, 17, 18 and 20 -- and nowhere else.
    ///
    /// The identical 14-level list is repeated by every other companion
    /// progression variable in the whole corpus (`CavalierMountLVL` at
    /// `advanced_players_guide/apg_companionmods.lst:75-93`,
    /// `SpecialMountLVL` at `core_rulebook/cr_companionmods.lst:34-47`,
    /// plus `BeastRiderLvl`, `FeatheredCompanionLvl`, `EmpyrealCompanion`
    /// and all twenty Packmaster `AnimalCompanionLVL{A..T}` variants), so
    /// this is one universal progression, not a per-class one.
    const CORPUS_HIT_DICE_BY_MASTER_LEVEL: [u8; 20] =
        [2, 3, 3, 4, 5, 6, 6, 7, 8, 9, 9, 10, 11, 12, 12, 13, 14, 15, 15, 16];

    #[test]
    fn the_hit_dice_table_matches_the_corpus_at_every_master_level() {
        assert_eq!(
            ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL, CORPUS_HIT_DICE_BY_MASTER_LEVEL,
            "the shipped Hit Dice table must be the corpus's own progression"
        );
    }

    /// Both species-named lookups must return the one universal
    /// progression at every master level 1-20 -- the whole point of this
    /// widening. Previously both hardcoded 2 and `debug_assert`ed that the
    /// master level was 1.
    #[test]
    fn both_species_lookups_return_the_corpus_progression_at_every_master_level() {
        for (index, &expected) in CORPUS_HIT_DICE_BY_MASTER_LEVEL.iter().enumerate() {
            let master_level = u8::try_from(index + 1).expect("1..=20 fits in u8");
            assert_eq!(
                wolf_companion_hit_dice(master_level),
                expected,
                "Wolf companion Hit Dice at master level {master_level}"
            );
            assert_eq!(
                horse_companion_hit_dice(master_level),
                expected,
                "Horse companion Hit Dice at master level {master_level}"
            );
        }
    }

    /// The progression is monotonic and never advances more than one Hit
    /// Die per master level -- a cheap structural guard against a
    /// transcription typo that happens to keep the endpoints right.
    #[test]
    fn the_hit_dice_progression_never_regresses_and_spans_two_to_sixteen() {
        assert_eq!(
            ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL[0], 2,
            "a 1st-level master's companion has 2 HD"
        );
        assert_eq!(
            ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL[19], 16,
            "a 20th-level master's companion has 16 HD"
        );
        for window in ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL.windows(2) {
            let [earlier, later] = window else { unreachable!("windows(2) yields pairs") };
            assert!(later >= earlier, "Hit Dice must never regress: {earlier} -> {later}");
            assert!(later - earlier <= 1, "Hit Dice advance at most one per master level");
        }
    }

    /// `BONUS:COMBAT|AC|2*floor(MasterLevel/3)|TYPE=NaturalArmor.STACK`
    /// (`core_rulebook/cr_abilities_companion.lst:59`, the shared
    /// `Animal Companion ~ AC Bonus` record reached by BOTH
    /// `Base Companion ~ Animal Companion` and
    /// `Base Companion ~ Special Mount`), independently confirmed by the
    /// Cavalier Mount block's own inline `BONUS:VAR|AC_Natural_Armor|2`
    /// at master levels 3, 6, 9, 12, 15 and 18
    /// (`advanced_players_guide/apg_companionmods.lst:76,79,82,85,88,91`).
    #[test]
    fn the_natural_armor_bonus_matches_the_corpus_formula() {
        for (master_level, expected) in [
            (1u8, 0i16),
            (2, 0),
            (3, 2),
            (5, 2),
            (6, 4),
            (9, 6),
            (12, 8),
            (15, 10),
            (18, 12),
            (20, 12),
        ] {
            assert_eq!(
                animal_companion_natural_armor_bonus(master_level),
                expected,
                "natural armor bonus at master level {master_level}: 2*floor({master_level}/3)"
            );
        }
    }

    /// `BONUS:STAT|STR,DEX|floor(MasterLevel/3)`
    /// (`core_rulebook/cr_abilities_companion.lst:60`), independently
    /// confirmed by the Cavalier Mount block's own inline
    /// `BONUS:STAT|STR,DEX|1` at the same six master levels.
    #[test]
    fn the_strength_and_dexterity_bonus_matches_the_corpus_formula() {
        for (master_level, expected) in [
            (1u8, 0i16),
            (2, 0),
            (3, 1),
            (5, 1),
            (6, 2),
            (9, 3),
            (12, 4),
            (15, 5),
            (18, 6),
            (20, 6),
        ] {
            assert_eq!(
                animal_companion_stat_bonus(master_level),
                expected,
                "Str/Dex bonus at master level {master_level}: floor({master_level}/3)"
            );
        }
    }

    /// The table covers exactly the domain the corpus defines: master
    /// levels 1-20 (`MAXLEVEL:20` on `CLASS:Companion`).
    #[test]
    fn the_table_covers_exactly_the_corpus_domain() {
        assert_eq!(ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL.len(), 20);
        assert_eq!(MAX_ANIMAL_COMPANION_MASTER_LEVEL, 20);
    }

    /// The old `debug_assert_eq!(companion_level, 1, "only companion level
    /// 1 is grounded this slice")` was REPLACED, not deleted: a master
    /// level outside the progression's own 1-20 domain is still a caller
    /// bug and still trips in debug/test builds. What changed is the
    /// domain it enforces -- the progression's real one instead of a
    /// single level.
    #[test]
    #[should_panic(expected = "outside the corpus progression's own 1-20 domain")]
    fn a_master_level_below_the_domain_still_trips_the_guard() {
        let _ = wolf_companion_hit_dice(0);
    }

    /// The same guard, at the top end, through the other species name --
    /// so neither lookup can quietly lose it.
    #[test]
    #[should_panic(expected = "outside the corpus progression's own 1-20 domain")]
    fn a_master_level_above_the_domain_still_trips_the_guard() {
        let _ = horse_companion_hit_dice(MAX_ANIMAL_COMPANION_MASTER_LEVEL + 1);
    }

    /// In a RELEASE build the `debug_assert` above compiles out, so the
    /// clamp underneath it is what actually protects the index. It
    /// saturates onto the nearest real row. This is the half of the fix
    /// that matters most: the old guard's release behaviour was to hand a
    /// 20th-level master's companion a 1st-level companion's Hit Dice,
    /// which is a wrong number shipping to users rather than a loud
    /// failure.
    ///
    /// Exercised through the two unguarded scaling helpers, which share
    /// the identical clamp and can be called at out-of-domain levels
    /// without tripping the debug guard.
    #[test]
    fn out_of_domain_master_levels_clamp_onto_the_real_table() {
        assert_eq!(animal_companion_natural_armor_bonus(0), 0, "below 1st clamps to the 1st-level row");
        assert_eq!(animal_companion_stat_bonus(0), 0, "below 1st clamps to the 1st-level row");
        assert_eq!(
            animal_companion_natural_armor_bonus(MAX_ANIMAL_COMPANION_MASTER_LEVEL + 1),
            animal_companion_natural_armor_bonus(MAX_ANIMAL_COMPANION_MASTER_LEVEL),
            "past 20th clamps onto the 20th-level row rather than running away"
        );
        assert_eq!(
            animal_companion_stat_bonus(MAX_ANIMAL_COMPANION_MASTER_LEVEL + 1),
            animal_companion_stat_bonus(MAX_ANIMAL_COMPANION_MASTER_LEVEL),
            "past 20th clamps onto the 20th-level row rather than running away"
        );
    }
}

/// End-to-end pins on the companion stat blocks the three consumer
/// classes actually emit, at the low, middle and top of the progression.
/// Separate from the pure-table tests above so a regression names itself:
/// a broken table fails there, a broken stat-block assembly fails here.
#[cfg(test)]
mod animal_companion_stat_block_tests {
    use super::{ComputationExplanation, ground_horse_companion_stat_block, ground_wolf_companion_stat_block};

    fn value_of(explanations: &[ComputationExplanation], id: &str) -> i16 {
        explanations
            .iter()
            .find(|explanation| explanation.id == id)
            .unwrap_or_else(|| panic!("expected a `{id}` record; got {:?}", explanations.iter().map(|e| &e.id).collect::<Vec<_>>()))
            .value
    }

    /// Druid and Hunter share the Wolf stat block, and both use their own
    /// class level as the companion's master level unshifted (Druid via
    /// `CompanionMasterLVL_Druid|DruidLVL`,
    /// `core_rulebook/cr_abilities_class.lst:772`; Hunter via
    /// "the hunter's effective druid level is equal to her hunter level",
    /// `advanced_class_guide/acg_abilities_class.lst:1171`), so one set of
    /// expected values covers both -- asserted for both owners rather than
    /// assumed.
    ///
    /// Wolf base Str 13, Con 15 (+2), natural armor +2, d8 Hit Dice.
    #[test]
    fn the_wolf_stat_block_is_right_at_the_bottom_middle_and_top_of_the_progression() {
        // (master level, HD, base attack bonus incl. Str, Fort/Ref, Will, AC, bite damage, HP)
        for (master_level, hit_dice, attack, fort_ref, will, armor_class, bite, hit_points) in [
            (1u8, 2u8, 2i16, 3i16, 0i16, 12i16, 1i16, 17i16),
            (7, 6, 6, 5, 2, 16, 3, 45),
            (20, 16, 16, 10, 5, 24, 6, 115),
        ] {
            assert_eq!(
                super::wolf_companion_hit_dice(master_level),
                hit_dice,
                "Wolf Hit Dice at master level {master_level}"
            );
            for owner in ["Druid", "Hunter"] {
                let mut explanations = Vec::new();
                ground_wolf_companion_stat_block("companion", owner, master_level, &mut explanations);
                let at = |id: &str| value_of(&explanations, id);
                assert_eq!(at("companion.base_attack_bonus"), attack, "{owner} L{master_level} attack");
                assert_eq!(at("companion.base_save.fortitude"), fort_ref, "{owner} L{master_level} Fort");
                assert_eq!(at("companion.base_save.reflex"), fort_ref, "{owner} L{master_level} Ref");
                assert_eq!(at("companion.base_save.will"), will, "{owner} L{master_level} Will");
                assert_eq!(at("companion.armor_class"), armor_class, "{owner} L{master_level} AC");
                assert_eq!(at("companion.bite_attack"), bite, "{owner} L{master_level} bite damage");
                assert_eq!(at("companion.hit_points"), hit_points, "{owner} L{master_level} HP");
            }
        }
    }

    /// Cavalier's Mount, whose effective druid level is likewise his own
    /// cavalier level (`BONUS:VAR|CompanionMasterLVL_Cavalier|CavalierLVL`,
    /// `advanced_players_guide/apg_abilities_class.lst:194`).
    ///
    /// Horse base Str 16, Con 15 (+2), natural armor +4, d8 Hit Dice --
    /// so the Hit Dice, saves and hit points match the Wolf's exactly,
    /// while the Strength- and natural-armor-derived numbers do not.
    #[test]
    fn the_horse_mount_stat_block_is_right_at_the_bottom_middle_and_top_of_the_progression() {
        for (master_level, hit_dice, attack, fort_ref, will, armor_class, hoof, hit_points) in [
            (1u8, 2u8, 4i16, 3i16, 0i16, 14i16, 4i16, 17i16),
            (7, 6, 8, 5, 2, 18, 6, 45),
            (20, 16, 18, 10, 5, 26, 9, 115),
        ] {
            assert_eq!(
                super::horse_companion_hit_dice(master_level),
                hit_dice,
                "Horse Hit Dice at master level {master_level}"
            );
            let mut explanations = Vec::new();
            ground_horse_companion_stat_block("mount", "Cavalier", master_level, &mut explanations);
            let at = |id: &str| value_of(&explanations, id);
            assert_eq!(at("mount.base_attack_bonus"), attack, "Cavalier L{master_level} attack");
            assert_eq!(at("mount.base_save.fortitude"), fort_ref, "Cavalier L{master_level} Fort");
            assert_eq!(at("mount.base_save.reflex"), fort_ref, "Cavalier L{master_level} Ref");
            assert_eq!(at("mount.base_save.will"), will, "Cavalier L{master_level} Will");
            assert_eq!(at("mount.armor_class"), armor_class, "Cavalier L{master_level} AC");
            assert_eq!(at("mount.hoof_attack"), hoof, "Cavalier L{master_level} hoof damage");
            assert_eq!(at("mount.hit_points"), hit_points, "Cavalier L{master_level} HP");
        }
    }

    /// The level-1 outputs must be byte-for-byte what shipped before this
    /// widening -- `floor(1/3)` is 0 for both the natural-armor and the
    /// Strength advance, so no previously-verified value moved.
    #[test]
    fn level_one_output_is_unchanged_by_the_widening() {
        let mut wolf = Vec::new();
        ground_wolf_companion_stat_block("companion", "Druid", 1, &mut wolf);
        assert_eq!(value_of(&wolf, "companion.armor_class"), 12, "Wolf base natural armor +2");
        assert_eq!(value_of(&wolf, "companion.base_attack_bonus"), 2, "Wolf: HD*3/4 = 1, Str 13 = +1");

        let mut horse = Vec::new();
        ground_horse_companion_stat_block("mount", "Cavalier", 1, &mut horse);
        assert_eq!(value_of(&horse, "mount.armor_class"), 14, "Horse base natural armor +4");
        assert_eq!(value_of(&horse, "mount.base_attack_bonus"), 4, "Horse: HD*3/4 = 1, Str 16 = +3");
    }
}

