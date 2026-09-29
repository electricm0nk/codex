#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm, risks item 8 (Oracle full-build closure, 8th ACG/APG
/// class-specific closure): APG Oracle, verified directly against
/// `apg_classes.lst`'s own `SPELLSTAT:CHA`/`MEMORIZE:NO` tokens -- a
/// spontaneous caster like Sorcerer/Bard, NOT a prepared caster like
/// Cleric/Wizard/Arcanist/Warpriest, so this closure mirrors
/// `unmet_sorcerer_known_spell_conditions`/`sorcerer_spells_known_table`/
/// `ground_sorcerer_known_spells`'s own shape, not Wizard's/Arcanist's/
/// Warpriest's own simpler prepared-spellbook shape. `SPELLLIST:2|Cleric|
/// Oracle` reuses Cleric's own spell-list content directly
/// (`cleric_spell_list::cleric_spell_level`) for the shared portion; the
/// Oracle-specific bonus-spell-list portion stays explicitly out of
/// scope, named honestly. See
/// `docs/release/v0.6/oracle-apg-full-build-scoping.md` for the full
/// corpus verification and scope record.
pub(super) const ORACLE_CLASS_ID: &str = "class:oracle";

/// Mirrors `WARPRIEST_SPELLBOOK_SUPPORTED_MAX_LEVEL` exactly, including
/// its v0.6 widening: this table originally verified Oracle's own known-
/// spells table for levels 1-3 only. The full 1-20 table is now
/// transcribed from the corpus (see `oracle_spells_known_table`), so no
/// legal class level is refused for lack of a verified row any more.
///
/// Note this only ever governed Oracle's known-spell posture. Oracle
/// stays claim-blocked at every level on its Mystery/Curse/other-features
/// diagnostics, which are entirely separate from this ceiling.
pub(super) const ORACLE_KNOWN_SPELLS_SUPPORTED_MAX_LEVEL: u8 = 20;

/// The choice set for which Mystery an Oracle selects at 1st level
/// (mirrors `WARPRIEST_BLESSING_CHOICE_ID`'s own single choice-set shape).
pub(super) const ORACLE_MYSTERY_CHOICE_ID: &str = "choice:oracle_mystery";

/// The choice set for which Curse an Oracle selects at 1st level.
pub(super) const ORACLE_CURSE_CHOICE_ID: &str = "choice:oracle_curse";

/// PF1 Advanced Player's Guide Life Mystery's Healing Hands revelation:
/// "+4 bonus on Heal checks" -- verified directly against
/// `apg_abilities_class.lst`'s own `BONUS:SKILL|Heal|4` tag. Flat,
/// unconditional once the Mystery is chosen (not activation-gated).
pub(super) const ORACLE_HEALING_HANDS_HEAL_BONUS: i16 = 4;

/// PF1 Advanced Player's Guide Clouded Vision Curse: "you cannot see
/// anything beyond 30 feet" -- verified directly against
/// `apg_abilities_class.lst`'s own `BONUS:VAR|OracleCloudedVisionRange|30`
/// tag. Flat, unconditional once the Curse is chosen.
pub(super) const ORACLE_CLOUDED_VISION_RANGE_FEET: i16 = 30;

/// The choice set naming WHICH revelation an Oracle actually took
/// (deepening 2026-07-26, task #10).
///
/// Revelations are a BUDGETED pick -- `OracleMysteryLevel` grants exactly
/// 1 at level 1, then +1 each at Oracle 3/7/11/15/19 -- so unlike
/// Warpriest's Blessing minor powers (which are automatic once the
/// blessing is chosen), gating a revelation on its Mystery alone would
/// assert a capability the character may never have bought. A level-1
/// Lore Oracle legally holds ONE revelation, so grounding both Sidestep
/// Secret and Lore Keeper off `mystery:lore` would describe an illegal
/// character.
///
/// This closure therefore requires an explicit
/// `choice:oracle_revelation -> revelation:<slug>` alongside the Mystery
/// pick, and grounds nothing when it is absent -- the same
/// no-silent-seeding design already ratified for
/// `choice:skill_focus_target`, reused here rather than reinvented.
///
/// **Known, deliberate inconsistency**: Life Mystery's own Healing Hands
/// (shipped in the original Oracle closure) still grounds on the Mystery
/// pick alone and is deliberately left that way -- retrofitting
/// already-shipped behavior to this pattern is a separate consistency
/// pass, not part of this deepening.
pub(super) const ORACLE_REVELATION_CHOICE_ID: &str = "choice:oracle_revelation";

/// The six Tier-1 revelations this deepening grounds.
pub(super) const ORACLE_CHANNEL_REVELATION: &str = "revelation:channel";

pub(super) const ORACLE_SIDESTEP_SECRET_REVELATION: &str = "revelation:sidestep_secret";

pub(super) const ORACLE_NATURES_WHISPERS_REVELATION: &str = "revelation:natures_whispers";

pub(super) const ORACLE_LORE_KEEPER_REVELATION: &str = "revelation:lore_keeper";

pub(super) const ORACLE_NEAR_DEATH_REVELATION: &str = "revelation:near_death";

pub(super) const ORACLE_CINDER_DANCE_REVELATION: &str = "revelation:cinder_dance";

/// Life Mystery's Channel: `BONUS:VAR|OracleChannelDieSize|6`.
pub(super) const ORACLE_CHANNEL_DIE_SIZE: i16 = 6;

/// Bone Mystery's Near Death: base `BONUS:VAR|OracleNearDeathSaveBonus|2`
/// plus a second `|2|PRECLASS:1,Oracle=11`, so +2 below Oracle 11 and +4
/// from 11 on.
pub(super) const ORACLE_NEAR_DEATH_BASE_SAVE_BONUS: i16 = 2;

pub(super) const ORACLE_NEAR_DEATH_UPGRADE_LEVEL: u8 = 11;

/// Flame Mystery's Cinder Dance: `BONUS:MOVEADD|TYPE.WALK|10`.
pub(super) const ORACLE_CINDER_DANCE_SPEED_BONUS: i16 = 10;

/// Lore Mystery's Lore Keeper names exactly 10 Knowledge skills in its
/// own `BONUS:SKILL|...` token.
pub(super) const ORACLE_LORE_KEEPER_KNOWLEDGE_SKILL_COUNT: i16 = 10;

/// The full, real, corpus-declared Mystery pool `chooser_option_selected`
/// validates every Mystery selection against -- every `KEY:<X> Mystery`
/// row `advanced_players_guide/apg_abilities_class.lst` declares, grep-
/// verified: `grep -oP '(?<=KEY:)[A-Za-z]+ Mystery\b'
/// apg_abilities_class.lst | sort -u` -> exactly these 10, matching
/// `SD31-E3-F1-001-clearance-table.json`'s own `mysteries_named_in_book:
/// 10`. Transcribed verbatim, never generated or inferred -- the exact
/// guard this primitive's own doc comment names.
pub(super) const ORACLE_MYSTERY_POOL: &[&str] = &[
    "mystery:battle",
    "mystery:bone",
    "mystery:flame",
    "mystery:heavens",
    "mystery:life",
    "mystery:lore",
    "mystery:nature",
    "mystery:stone",
    "mystery:waves",
    "mystery:wind",
];

/// Battle Mystery's own tier-1 revelation this cycle grounds: PF1
/// Advanced Player's Guide, "Battlecry" -- `BONUS:VAR|OracleBattlecryBonus|1`
/// (+2 from `PRECLASS:1,Oracle=10`), `BONUS:VAR|OracleBattlecryDuration|CHA`,
/// `BONUS:VAR|OracleBattlecryTimes|1+classlevel("Oracle")/5`. All three are
/// flat formulas -- no dice roll, no target-creature-state dependency --
/// the same shape Sidestep Secret/Nature's Whispers/Lore Keeper already
/// ground.
pub(super) const ORACLE_BATTLECRY_REVELATION: &str = "revelation:battlecry";

/// Battle Mystery's real, corpus-declared revelation pool this Mystery's
/// own choice draws from -- every `KEY:Battle Mystery ~ <X>` row in
/// `apg_abilities_class.lst`, grep-verified (`grep -oP
/// '(?<=KEY:Battle Mystery ~ )[^\t]+' apg_abilities_class.lst`), 11 real
/// entries. Scoped to Battle Mystery alone (not a flat cross-Mystery
/// revelation list) -- a revelation name from a DIFFERENT Mystery must
/// never validate here, matching this program's own "a shared NAME is not
/// a duplicate" discipline (`decisions.md §10`).
pub(super) const ORACLE_BATTLE_MYSTERY_REVELATION_POOL: &[&str] = &[
    "revelation:battlecry",
    "revelation:battlefield_clarity",
    "revelation:combat_healer",
    "revelation:iron_skin",
    "revelation:maneuver_mastery",
    "revelation:resiliency",
    "revelation:skill_at_arms",
    "revelation:surprising_charge",
    "revelation:war_sight",
    "revelation:weapon_mastery",
    "revelation:final_revelation",
];

/// Battlecry's flat morale bonus: `1`, upgraded to `2` at Oracle level 10
/// (`BONUS:VAR|OracleBattlecryBonus|1|PRECLASS:1,Oracle=10`).
pub(super) const ORACLE_BATTLECRY_BASE_BONUS: i16 = 1;

pub(super) const ORACLE_BATTLECRY_UPGRADED_BONUS: i16 = 2;

pub(super) const ORACLE_BATTLECRY_UPGRADE_LEVEL: u8 = 10;

/// Battlecry's uses-per-day divisor: `1+classlevel("Oracle")/5`.
pub(super) const ORACLE_BATTLECRY_TIMES_PER_DAY_LEVEL_DIVISOR: i16 = 5;

/// Stone Mystery's real, corpus-declared revelation pool -- every
/// `KEY:Stone Mystery ~ <X>` row in `apg_abilities_class.lst`, grep-
/// verified (`grep -oP '(?<=KEY:Stone Mystery ~ )[^\t]+'
/// apg_abilities_class.lst`), 11 real entries, transcribed verbatim.
pub(super) const ORACLE_STONE_MYSTERY_REVELATION_POOL: &[&str] = &[
    "revelation:acid_skin",
    "revelation:clobbering_strike",
    "revelation:crystal_sight",
    "revelation:earth_glide",
    "revelation:mighty_pebble",
    "revelation:rock_throwing",
    "revelation:shard_explosion",
    "revelation:steelbreaker_skin",
    "revelation:stone_stability",
    "revelation:touch_of_acid",
    "revelation:final_revelation",
];

/// Waves Mystery's real, corpus-declared revelation pool, same grep
/// discipline as Stone's, 11 real entries.
pub(super) const ORACLE_WAVES_MYSTERY_REVELATION_POOL: &[&str] = &[
    "revelation:blizzard",
    "revelation:fluid_nature",
    "revelation:fluid_travel",
    "revelation:freezing_spell",
    "revelation:ice_armor",
    "revelation:icy_skin",
    "revelation:punitive_transformation",
    "revelation:water_form",
    "revelation:water_sight",
    "revelation:wintry_touch",
    "revelation:final_revelation",
];

/// Wind Mystery's real, corpus-declared revelation pool, 10 real entries.
pub(super) const ORACLE_WIND_MYSTERY_REVELATION_POOL: &[&str] = &[
    "revelation:air_barrier",
    "revelation:gaseous_form",
    "revelation:invisibility",
    "revelation:lightning_breath",
    "revelation:spark_skin",
    "revelation:touch_of_electricity",
    "revelation:wortex_spells",
    "revelation:wind_sight",
    "revelation:wings_of_air",
    "revelation:final_revelation",
];

/// Heavens Mystery's real, corpus-declared revelation pool, 11 real
/// entries.
pub(super) const ORACLE_HEAVENS_MYSTERY_REVELATION_POOL: &[&str] = &[
    "revelation:awesome_display",
    "revelation:coat_of_many_stars",
    "revelation:dweller_in_darkness",
    "revelation:guiding_star",
    "revelation:interstellar_void",
    "revelation:lure_of_the_heavens",
    "revelation:mantle_of_moonlight",
    "revelation:moonlight_bridge",
    "revelation:spray_of_shooting_stars",
    "revelation:star_chart",
    "revelation:final_revelation",
];

/// Stone Mystery's Steelbreaker Skin, this cycle's representative for
/// Stone: `apg_abilities_class.lst`'s own record grants
/// `BONUS:VAR|OracleSteelbreakerSkinDamage|classlevel("Oracle")` and
/// `BONUS:VAR|OracleSteelbreakerSkinDuration|classlevel("Oracle")` -- a
/// standard-action, once-per-day self-activation (not a target-creature
/// or environment condition), so it clears Decision 7 REFINED's
/// universal-vs-conditional bar the same way Life's Channel (also
/// activation-gated) already does.
pub(super) const ORACLE_STEELBREAKER_SKIN_REVELATION: &str = "revelation:steelbreaker_skin";

/// Waves Mystery's Icy Skin, this cycle's representative for Waves:
/// `BONUS:VAR|ColdResistanceBonus|5+OracleColdResistanceBonus|TYPE=Resistance`
/// plus two PRECLASS-gated additions to `OracleColdResistanceBonus`
/// (`|5|PRECLASS:1,Oracle=5`, `|10|PRECLASS:1,Oracle=11`) -- the same
/// additive two-tier shape Bone Mystery's Near Death already grounds
/// (`ORACLE_NEAR_DEATH_BASE_SAVE_BONUS`/`ORACLE_NEAR_DEATH_UPGRADE_LEVEL`),
/// always-on cold resistance, no target-creature dependency.
pub(super) const ORACLE_ICY_SKIN_REVELATION: &str = "revelation:icy_skin";

/// Wind Mystery's Spark Skin, this cycle's representative for Wind:
/// identical formula shape to Icy Skin
/// (`BONUS:VAR|ElectricityResistanceBonus|5+OracleElectricityResistanceBonus`,
/// same two PRECLASS-gated `+5`/`+10` steps at Oracle 5/11), electricity
/// resistance instead of cold.
pub(super) const ORACLE_SPARK_SKIN_REVELATION: &str = "revelation:spark_skin";

/// Heavens Mystery's Coat of Many Stars, this cycle's representative for
/// Heavens: `BONUS:VAR|OracleCoatofManyStarsACBonus|
/// 4+2*max(0,floor((classlevel("Oracle")-3)/4))` and
/// `BONUS:VAR|OracleCoatofManyStarsDuration|classlevel("Oracle")` --
/// self-cast armor bonus, standard-action activation gated the same way
/// Steelbreaker Skin is, no target-creature or environment dependency.
pub(super) const ORACLE_COAT_OF_MANY_STARS_REVELATION: &str = "revelation:coat_of_many_stars";

/// Icy Skin / Spark Skin's shared elemental-resistance formula: a flat
/// `5` base, `+5` more at Oracle level `ORACLE_ELEMENTAL_RESISTANCE_TIER_TWO_LEVEL`,
/// `+10` more (cumulative, not replacing) at Oracle level
/// `ORACLE_ELEMENTAL_RESISTANCE_TIER_THREE_LEVEL` -- re-derived directly
/// from both records' identical `BONUS:VAR` chains, not assumed shared
/// because the names match.
pub(super) const ORACLE_ELEMENTAL_RESISTANCE_BASE: i16 = 5;

pub(super) const ORACLE_ELEMENTAL_RESISTANCE_TIER_TWO_LEVEL: u8 = 5;

pub(super) const ORACLE_ELEMENTAL_RESISTANCE_TIER_TWO_BONUS: i16 = 5;

pub(super) const ORACLE_ELEMENTAL_RESISTANCE_TIER_THREE_LEVEL: u8 = 11;

pub(super) const ORACLE_ELEMENTAL_RESISTANCE_TIER_THREE_BONUS: i16 = 10;

/// Coat of Many Stars' AC-bonus step size and the level at which stepping
/// begins (`4 + 2*max(0,floor((level-3)/4))`).
pub(super) const ORACLE_COAT_OF_MANY_STARS_BASE_AC_BONUS: i16 = 4;

pub(super) const ORACLE_COAT_OF_MANY_STARS_AC_STEP: i16 = 2;

pub(super) const ORACLE_COAT_OF_MANY_STARS_STEP_START_LEVEL: i16 = 3;

pub(super) const ORACLE_COAT_OF_MANY_STARS_STEP_INTERVAL: i16 = 4;

/// Deaf Curse's `BONUS:SITUATION|Perception=Opposed|-4`. Grounds per the
/// lead's explicit ruling (2026-07-27): a flat modifier applying to your
/// OWN roll clears the corrected bar even when the check is opposed --
/// the Studied-Target line is about needing a persistent tracked
/// relationship with a specific opponent, not about opposition per se.
pub(super) const ORACLE_DEAF_OPPOSED_PERCEPTION_PENALTY: i16 = -4;

/// Curse-level thresholds at which every Oracle curse steps up
/// (`PREVARGTEQ:OracleCurseLVL,5` / `,10` / `,15` across the `.MOD`
/// records). Only the first two carry numeric magnitudes in the curses
/// this deepening grounds; the 15 tier's content is non-numeric
/// (tremorsense, immunities) and stays deferred.
pub(super) const ORACLE_CURSE_TIER_TWO_LEVEL: i16 = 5;

pub(super) const ORACLE_CURSE_TIER_THREE_LEVEL: i16 = 10;

/// Wasting Curse's `BONUS:SKILL|STAT.CHA|-4` -- a penalty on ALL
/// Charisma-based skill checks.
pub(super) const ORACLE_WASTING_CHARISMA_SKILL_PENALTY: i16 = -4;

/// Wasting Curse's `BONUS:SKILL|Intimidate|4`, which exists solely to
/// cancel the Charisma-skill penalty on Intimidate. See
/// `oracle_wasting_intimidate_net_effect` for why this is NOT a `+4`.
pub(super) const ORACLE_WASTING_INTIMIDATE_OFFSET: i16 = 4;

/// Deaf Curse's `BONUS:SKILL|Perception|3|PREVARGTEQ:OracleCurseLVL,5`.
pub(super) const ORACLE_DEAF_PERCEPTION_BONUS: i16 = 3;

/// Deaf Curse's base `BONUS:VAR|OracleDeafInitPenalty|-4`, walked back
/// toward zero by two `+2` `.MOD` steps -- see
/// `oracle_deaf_initiative_penalty`.
pub(super) const ORACLE_DEAF_BASE_INITIATIVE_PENALTY: i16 = -4;

pub(super) const ORACLE_DEAF_INITIATIVE_PENALTY_STEP: i16 = 2;

/// Lame Curse's speed penalty branches: `BONUS:VAR|OracleLameEffect|10|
/// PREMOVE:1,Walk=30` and `|5|!PREMOVE:1,Walk=30`.
pub(super) const ORACLE_LAME_PENALTY_AT_THIRTY_FEET: i16 = 10;

pub(super) const ORACLE_LAME_PENALTY_BELOW_THIRTY_FEET: i16 = 5;

/// The base land speed at which Lame's larger penalty branch applies
/// (`PREMOVE:1,Walk=30`).
pub(super) const ORACLE_LAME_FULL_SPEED_FEET: i16 = 30;

/// v0.6 alpha swarm, risks item 8 (Oracle full-build closure): whether
/// `input` is a single-class Oracle at a level within
/// `apg::class_chassis_resolve`'s declared ceiling for Oracle -- mirrors
/// `is_supported_cavalier_single_class`/`is_supported_alchemist_single_class`/
/// `is_supported_inquisitor_single_class` exactly, including the same
/// exact-match discipline (`== Some(ApgClassId::Oracle)`, not a broad
/// `.is_some()`).
pub(super) fn is_supported_oracle_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if ApgClassId::from_class_id_str(&class_level.class_id) != Some(ApgClassId::Oracle) {
        return false;
    }
    apg::class_chassis_resolve(ApgClassId::Oracle, class_level.level, RuleSetId::Apg).is_some()
}

/// The PF1 Advanced Player's Guide Oracle Spells Known table's row, one
/// entry per spell level 0 (orisons) through 9 (`None` for an
/// inaccessible "--" column). This is the cap on distinct spells KNOWN
/// (permanent), the same shape as `sorcerer_spells_known_table` -- Oracle
/// is a spontaneous caster (`MEMORIZE:NO`), not a prepared caster like
/// Cleric/Wizard/Arcanist/Warpriest.
///
/// Transcribed literally from the PCGen corpus's own `CLASS:Oracle`
/// level-progression `KNOWN:` rows at
/// `data/pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst`
/// lines 115-134 (level 1 on line 115 through level 20 on line 134) -- a
/// literal table lookup, not a derived formula. Read off the `KNOWN:`
/// column, deliberately not the `CAST:` column on those same lines:
/// `CAST:` is Oracle's spells PER DAY, a different quantity, and it
/// additionally carries a leading `0` sentinel in its spell-level-0
/// column because a spontaneous caster's orisons are cast at will rather
/// than from a daily count.
///
/// These rows are byte-identical to `sorcerer_spells_known_table`'s. That
/// is not an assumed reuse: both files' raw rows were read and compared
/// directly, and PF1 genuinely gives Oracle and Sorcerer the same spells-
/// known progression. Each class keeps its own table so a future
/// divergence in either book cannot silently propagate to the other.
///
/// Returns an all-`None` row outside the legal 1-20 class-level range.
pub(super) fn oracle_spells_known_table(level: u8) -> [Option<i16>; 10] {
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

/// Return the list of unmet conditions for Oracle's real known-spell
/// posture, mirroring `unmet_sorcerer_known_spell_conditions`'s own shape
/// exactly, substituting Oracle's own bounded 1-3 table and reusing
/// `cleric_spell_list::cleric_spell_level` directly for the per-spell-id
/// level lookup (`SPELLLIST:2|Cleric|Oracle` means the Cleric portion of
/// Oracle's spell list is genuinely shared; the Oracle-specific bonus-
/// spell-list portion stays explicitly out of scope, so a spell that is
/// only on the Oracle-specific list is treated the same as any other
/// off-list spell -- named honestly, not silently accepted). An empty
/// list means the posture is fully valid; zero known spells is always
/// valid, same reasoning as Sorcerer's own posture.
pub(super) fn unmet_oracle_known_spell_conditions(input: &CharacterInput, oracle_level: u8) -> Vec<String> {
    let mut unmet = Vec::new();

    if oracle_level > ORACLE_KNOWN_SPELLS_SUPPORTED_MAX_LEVEL {
        unmet.push(format!(
            "known-spell grounding is only supported for oracle levels \
             1-{ORACLE_KNOWN_SPELLS_SUPPORTED_MAX_LEVEL}, got {oracle_level}"
        ));
        return unmet;
    }

    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| s.source_class_id == ORACLE_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();

    let known_table = oracle_spells_known_table(oracle_level);

    let mut known_per_level: [i16; 10] = [0; 10];
    for spell_id in &known {
        let Some(spell_level) = cleric_spell_list::cleric_spell_level(spell_id) else {
            unmet.push(format!(
                "known spell '{spell_id}' is not on the real PF1 cleric spell list \
                 (Oracle's own bonus spell-list portion is explicitly out of scope)"
            ));
            continue;
        };
        if usize::from(spell_level) >= known_table.len() {
            unmet.push(format!(
                "known spell '{spell_id}' targets spell level {spell_level}, not yet accessible \
                 at oracle level {oracle_level}"
            ));
            continue;
        }
        known_per_level[usize::from(spell_level)] += 1;
    }

    for (index, count) in known_per_level.iter().enumerate() {
        if *count == 0 {
            continue;
        }
        let cap = known_table[index].unwrap_or(0);
        if *count > cap {
            unmet.push(format!(
                "spell level {index} over-known: {count} distinct spells known but only {cap} \
                 slots available on the Oracle Spells Known table"
            ));
        }
    }

    unmet
}

/// Ground the real known-spell posture once
/// `unmet_oracle_known_spell_conditions` reports an empty unmet list,
/// mirroring `ground_sorcerer_known_spells`'s own shape exactly.
pub(super) fn ground_oracle_known_spells(
    input: &CharacterInput,
    oracle_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| s.source_class_id == ORACLE_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.apg.oracle.known_spells".to_owned(),
        value: known.len() as i16,
        detail: format!(
            "Oracle level {oracle_level} known-spell selection ({} spells, \
             AcquisitionMode::Known): {}. Each known spell is verified against the real PF1 \
             cleric spell list (`cleric_spell_list::cleric_spell_level`, all ingested books, the \
             shared portion of `SPELLLIST:2|Cleric|Oracle`) and the Oracle Spells Known \
             table's own per-level cap for levels 1-3. Real PF1 Oracle rules have no daily \
             preparation step at all (unlike Cleric/Wizard/Arcanist/Warpriest) -- an oracle's \
             known spells are permanent once learned, cast spontaneously. This grounds the \
             known-spell selection for real; it computes no spell save DC resolution against a \
             target and no casting execution",
            known.len(),
            known.join(", ")
        ),
    });
}

/// Grounds Oracle's Mystery choice (v0.6 alpha swarm, risks item 8, Oracle
/// full-build closure): recognizes `choice:oracle_mystery -> mystery:life`
/// and, if present, grounds Life Mystery's own Healing Hands revelation
/// (a flat +4 Heal check bonus, unconditional once chosen -- not
/// activation-gated, unlike Destructive Attacks/Touch of Good). Heal isn't
/// among the three skills `compute_selected_skill_modifiers` tracks, so
/// this grounds as a standalone flat record, the same idiom as Slayer's
/// own Trapfinding/Track. Mirrors Warpriest's own Blessing three-branch
/// dispatch shape (`ground_or_block_warpriest_class_features`'s own
/// blessing_selections handling).
pub(super) fn ground_or_block_oracle_mystery(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let mystery_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == ORACLE_MYSTERY_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();

    if mystery_selections.contains(&LIFE_MYSTERY_SELECTION) {
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.life_mystery.healing_hands".to_owned(),
            value: ORACLE_HEALING_HANDS_HEAL_BONUS,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SKILL|Heal|4` tag
                "Oracle with the Life Mystery gets Healing Hands: a \
                 +{ORACLE_HEALING_HANDS_HEAL_BONUS} bonus on Heal checks (PF1 Advanced Player's \
                 Guide, verified directly against `apg_abilities_class.lst`'s own). Grounds only the \
                 flat magnitude -- Heal isn't among the three skills \
                 `compute_selected_skill_modifiers` tracks, so no total integration exists; the \
                 revelation's own \"treat two people at once\" clause is not modeled"
            ),
        });
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.oracle.mystery_revelations_beyond_life.unmodeled".to_owned(),
            message: "Oracle Mystery revelation content beyond the grounded Tier-1 set remains \
                 unmodeled. Grounded: Life's Healing Hands and Channel, Lore's Sidestep Secret \
                 and Lore Keeper, Nature's Nature's Whispers, Bone's Near Death, Flame's Cinder \
                 Dance, Battle's Battlecry, Stone's Steelbreaker Skin, Waves' Icy Skin, Wind's \
                 Spark Skin, and Heavens' Coat of Many Stars -- each (except the already-shipped \
                 Healing Hands) requiring an explicit `choice:oracle_revelation` pick, since \
                 revelations are a budgeted selection rather than an automatic grant. Every \
                 other revelation across all 10 Mysteries remains unimplemented, including the \
                 ally-dependent, environment-dependent, and opponent-state-dependent ones (which \
                 need engine state this codebase does not have), the summon-subsystem ones, and \
                 the revelation records that carry no numeric token at all -- a genuine no-op, \
                 not a transcription backlog. This does not block an otherwise-valid Mystery \
                 posture"
                .to_owned(),
            claim_blocking: false,
        });
    } else if !mystery_selections.iter().any(|selection| {
        matches!(
            *selection,
            LORE_MYSTERY_SELECTION
                | NATURE_MYSTERY_SELECTION
                | BONE_MYSTERY_SELECTION
                | FLAME_MYSTERY_SELECTION
                | BATTLE_MYSTERY_SELECTION
                | STONE_MYSTERY_SELECTION
                | WAVES_MYSTERY_SELECTION
                | WIND_MYSTERY_SELECTION
                | HEAVENS_MYSTERY_SELECTION
        )
    }) {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.oracle.mystery_powers.unsupported".to_owned(),
            message: "Oracle remains blocked on its Mystery powers burden: no recognized Mystery \
                 choice is present (this codebase now recognizes all 10 real Mysteries -- Life, \
                 Lore, Nature, Bone, Flame, Battle, Stone, Waves, Wind, Heavens -- each with at \
                 least one grounded revelation), so no Oracle Mystery-power support is claimed"
                .to_owned(),
            claim_blocking: true,
        });
    }
}

/// Whether this character is an Oracle who took `mystery` AND explicitly
/// recorded `revelation` (deepening 2026-07-26, task #10). Returns the
/// Oracle level when both hold. Class-ownership-gated, so a spoofed
/// choice on a non-Oracle can never ground anything.
///
/// Requiring the explicit revelation pick is the whole point of the
/// gate -- see `ORACLE_REVELATION_CHOICE_ID` for why a Mystery-only gate
/// would describe an illegal character.
pub(super) fn oracle_level_with_revelation(
    input: &CharacterInput,
    mystery: &str,
    revelation: &str,
) -> Option<u8> {
    let oracle_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ORACLE_CLASS_ID)
        .map(|class_level| class_level.level)?;
    let has = |choice_set_id: &str, selection: &str| {
        input
            .chosen
            .selected_choices
            .iter()
            .any(|c| c.choice_set_id == choice_set_id && c.selection_id == selection)
    };
    if !has(ORACLE_MYSTERY_CHOICE_ID, mystery) {
        return None;
    }
    if !has(ORACLE_REVELATION_CHOICE_ID, revelation) {
        return None;
    }
    Some(oracle_level)
}

/// Same question as `oracle_level_with_revelation` (did this Oracle take
/// `BATTLE_MYSTERY_SELECTION` AND explicitly record
/// `ORACLE_BATTLECRY_REVELATION`?), but composed through the new
/// `archetype_resolver::chooser_option_selected` primitive
/// (SD31-E4-F2-001) instead of the bare `.contains()` idiom above --
/// this is the primitive's first production caller, proven reachable via
/// `build_pilot_headless_receipt` in this module's own test suite. Kept as
/// a separate function rather than a rewrite of `oracle_level_with_revelation`
/// itself: the six existing revelations stay on their proven, already-
/// reviewed path unchanged, and this new path can be reviewed on its own
/// against the primitive independently, matching this card's own
/// "land the primitive before any pool uses it, so the mechanism can be
/// reviewed independently of its first consumers" instruction.
pub(super) fn oracle_level_with_battlecry_revelation(input: &CharacterInput) -> Option<u8> {
    let oracle_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ORACLE_CLASS_ID)
        .map(|class_level| class_level.level)?;
    if !archetype_resolver::chooser_option_selected(
        input,
        ORACLE_MYSTERY_CHOICE_ID,
        BATTLE_MYSTERY_SELECTION,
        ORACLE_MYSTERY_POOL,
    ) {
        return None;
    }
    if !archetype_resolver::chooser_option_selected(
        input,
        ORACLE_REVELATION_CHOICE_ID,
        ORACLE_BATTLECRY_REVELATION,
        ORACLE_BATTLE_MYSTERY_REVELATION_POOL,
    ) {
        return None;
    }
    Some(oracle_level)
}

/// Battlecry's flat morale bonus: `ORACLE_BATTLECRY_BASE_BONUS` (+1),
/// stepping to `ORACLE_BATTLECRY_UPGRADED_BONUS` (+2) at Oracle level
/// `ORACLE_BATTLECRY_UPGRADE_LEVEL` (10) -- `BONUS:VAR|OracleBattlecryBonus|1`
/// plus `|1|PRECLASS:1,Oracle=10` (the two flat additions sum to +2, they
/// do not replace one another).
pub fn oracle_battlecry_bonus(oracle_level: u8) -> i16 {
    if oracle_level >= ORACLE_BATTLECRY_UPGRADE_LEVEL {
        ORACLE_BATTLECRY_UPGRADED_BONUS
    } else {
        ORACLE_BATTLECRY_BASE_BONUS
    }
}

/// Battlecry's uses-per-day: `1+classlevel("Oracle")/5`, integer division.
pub(super) fn oracle_battlecry_times_per_day(oracle_level: u8) -> i16 {
    1 + i16::from(oracle_level) / ORACLE_BATTLECRY_TIMES_PER_DAY_LEVEL_DIVISOR
}

/// SD31-E4-F2-002: Stone Mystery's Steelbreaker Skin, composed through
/// `chooser_option_selected` exactly as `oracle_level_with_battlecry_revelation`
/// composes Battle Mystery's Battlecry -- see that function's own doc
/// comment for why this stays a separate function per-Mystery rather than
/// a generalized one.
pub(super) fn oracle_level_with_steelbreaker_skin_revelation(input: &CharacterInput) -> Option<u8> {
    let oracle_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ORACLE_CLASS_ID)
        .map(|class_level| class_level.level)?;
    if !archetype_resolver::chooser_option_selected(
        input,
        ORACLE_MYSTERY_CHOICE_ID,
        STONE_MYSTERY_SELECTION,
        ORACLE_MYSTERY_POOL,
    ) {
        return None;
    }
    if !archetype_resolver::chooser_option_selected(
        input,
        ORACLE_REVELATION_CHOICE_ID,
        ORACLE_STEELBREAKER_SKIN_REVELATION,
        ORACLE_STONE_MYSTERY_REVELATION_POOL,
    ) {
        return None;
    }
    Some(oracle_level)
}

/// SD31-E4-F2-002: Waves Mystery's Icy Skin, same composition shape as
/// Steelbreaker Skin above.
pub(super) fn oracle_level_with_icy_skin_revelation(input: &CharacterInput) -> Option<u8> {
    let oracle_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ORACLE_CLASS_ID)
        .map(|class_level| class_level.level)?;
    if !archetype_resolver::chooser_option_selected(
        input,
        ORACLE_MYSTERY_CHOICE_ID,
        WAVES_MYSTERY_SELECTION,
        ORACLE_MYSTERY_POOL,
    ) {
        return None;
    }
    if !archetype_resolver::chooser_option_selected(
        input,
        ORACLE_REVELATION_CHOICE_ID,
        ORACLE_ICY_SKIN_REVELATION,
        ORACLE_WAVES_MYSTERY_REVELATION_POOL,
    ) {
        return None;
    }
    Some(oracle_level)
}

/// SD31-E4-F2-002: Wind Mystery's Spark Skin, same composition shape as
/// Steelbreaker Skin above.
pub(super) fn oracle_level_with_spark_skin_revelation(input: &CharacterInput) -> Option<u8> {
    let oracle_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ORACLE_CLASS_ID)
        .map(|class_level| class_level.level)?;
    if !archetype_resolver::chooser_option_selected(
        input,
        ORACLE_MYSTERY_CHOICE_ID,
        WIND_MYSTERY_SELECTION,
        ORACLE_MYSTERY_POOL,
    ) {
        return None;
    }
    if !archetype_resolver::chooser_option_selected(
        input,
        ORACLE_REVELATION_CHOICE_ID,
        ORACLE_SPARK_SKIN_REVELATION,
        ORACLE_WIND_MYSTERY_REVELATION_POOL,
    ) {
        return None;
    }
    Some(oracle_level)
}

/// SD31-E4-F2-002: Heavens Mystery's Coat of Many Stars, same composition
/// shape as Steelbreaker Skin above.
pub(super) fn oracle_level_with_coat_of_many_stars_revelation(input: &CharacterInput) -> Option<u8> {
    let oracle_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ORACLE_CLASS_ID)
        .map(|class_level| class_level.level)?;
    if !archetype_resolver::chooser_option_selected(
        input,
        ORACLE_MYSTERY_CHOICE_ID,
        HEAVENS_MYSTERY_SELECTION,
        ORACLE_MYSTERY_POOL,
    ) {
        return None;
    }
    if !archetype_resolver::chooser_option_selected(
        input,
        ORACLE_REVELATION_CHOICE_ID,
        ORACLE_COAT_OF_MANY_STARS_REVELATION,
        ORACLE_HEAVENS_MYSTERY_REVELATION_POOL,
    ) {
        return None;
    }
    Some(oracle_level)
}

/// Steelbreaker Skin's flat per-use magnitudes: both damage prevented and
/// duration equal the Oracle's class level
/// (`BONUS:VAR|OracleSteelbreakerSkinDamage|classlevel("Oracle")`,
/// `...Duration|classlevel("Oracle")`).
pub(super) fn oracle_steelbreaker_skin_damage(oracle_level: u8) -> i16 {
    i16::from(oracle_level)
}

pub(super) fn oracle_steelbreaker_skin_duration_minutes(oracle_level: u8) -> i16 {
    i16::from(oracle_level)
}

/// The elemental-resistance formula shared by Icy Skin and Spark Skin: a
/// flat base, `+TIER_TWO_BONUS` more from `TIER_TWO_LEVEL`, and
/// `+TIER_THREE_BONUS` more (additive, not replacing) from
/// `TIER_THREE_LEVEL` -- the same additive two-tier shape Bone Mystery's
/// Near Death already grounds.
pub(super) fn oracle_elemental_resistance(oracle_level: u8) -> i16 {
    let mut resistance = ORACLE_ELEMENTAL_RESISTANCE_BASE;
    if oracle_level >= ORACLE_ELEMENTAL_RESISTANCE_TIER_TWO_LEVEL {
        resistance += ORACLE_ELEMENTAL_RESISTANCE_TIER_TWO_BONUS;
    }
    if oracle_level >= ORACLE_ELEMENTAL_RESISTANCE_TIER_THREE_LEVEL {
        resistance += ORACLE_ELEMENTAL_RESISTANCE_TIER_THREE_BONUS;
    }
    resistance
}

/// Coat of Many Stars' AC bonus: `4 + 2*max(0,floor((level-3)/4))`.
/// `div_euclid` gives floor division for a positive divisor even when the
/// dividend (`level - 3`) is negative, matching PCGen's own `floor()`.
pub(super) fn oracle_coat_of_many_stars_ac_bonus(oracle_level: u8) -> i16 {
    let steps = (i16::from(oracle_level) - ORACLE_COAT_OF_MANY_STARS_STEP_START_LEVEL)
        .div_euclid(ORACLE_COAT_OF_MANY_STARS_STEP_INTERVAL)
        .max(0);
    ORACLE_COAT_OF_MANY_STARS_BASE_AC_BONUS + ORACLE_COAT_OF_MANY_STARS_AC_STEP * steps
}

/// Coat of Many Stars' duration: `classlevel("Oracle")` hours/day.
pub(super) fn oracle_coat_of_many_stars_duration_hours(oracle_level: u8) -> i16 {
    i16::from(oracle_level)
}

/// The stat-substitution idiom shared by Sidestep Secret
/// (`max(CHA,DEX)-DEX`), Nature's Whispers (`max(DEX,CHA)-DEX`), and Lore
/// Keeper (`max(CHA,INT)-INT`): the DELTA that raises a value already
/// computed from `base_modifier` up to `substitute_modifier` when the
/// substitute is higher, and 0 otherwise.
///
/// Returned as a delta rather than a replacement precisely because every
/// consumer here already adds the base modifier itself -- adding this on
/// top yields `max(substitute, base)` exactly, with no double-count.
/// Genuinely 0 whenever the substitute stat is not higher, which is an
/// honest and common outcome, not a failure.
pub(super) fn oracle_stat_substitution_delta(substitute_modifier: i16, base_modifier: i16) -> i16 {
    substitute_modifier.max(base_modifier) - base_modifier
}

/// Life Mystery's Channel uses per day: `BONUS:VAR|OracleChannelTimes|
/// 1+CHA`. Note this is Oracle's OWN formula and is deliberately kept
/// separate from `shaman_channel_uses_per_day` per the established
/// parallel-copy discipline, even though the corpus declares
/// `SERVESAS:ABILITY=...|Cleric ~ Channel Energy`.
pub(super) fn oracle_channel_uses_per_day(charisma_modifier: i16) -> i16 {
    1 + charisma_modifier
}

/// Life Mystery's Channel dice: `BONUS:VAR|OracleChannelDice|
/// (OracleChannelLVL+1)/2`, where `OracleChannelLVL = classlevel("Oracle")`.
pub(super) fn oracle_channel_dice(oracle_level: u8) -> i16 {
    (i16::from(oracle_level) + 1) / 2
}

/// Life Mystery's Channel save DC: `BONUS:VAR|OracleChannelDC|
/// 10+(OracleChannelLVL/2)+CHA`.
///
/// The `Improved Channel` feat contributes `+2` to this same variable,
/// and this is the ONLY channel DC in this engine it can reach (v0.6
/// alpha swarm, `BONUS:VAR` triage slice, 2026-07-29). The feat's own
/// `cr_feats.lst` record names five variables --
/// `ClericChannelPositiveEnergyDC`, `PaladinChannelPositiveEnergyDC`,
/// `ClericChannelNegativeEnergyDC`, `PowerOverUndeadCommandDC`,
/// `PowerOverUndeadTurnDC` -- and this engine computes a total for none
/// of them: Cleric grounds `channel_energy_dice` and
/// `channel_energy_uses_per_day` but no DC at all, Paladin grounds
/// `channel_positive_energy_dice` but no DC. Read against the Core file
/// alone the feat is a pure no-op here, which is exactly why it had been
/// skipped. `apg_feats.lst`'s own `CATEGORY=FEAT|Improved Channel.MOD`
/// adds `BONUS:VAR|OracleChannelDC|2` (and `UndeadServitudeDC|2`, which
/// has no consumer here) -- the same "the real reach lives on a `.MOD`
/// in a different book" shape that hid Extra Performance's Skald half.
///
/// Presence-based, not counted: Improved Channel carries no `STACK:`,
/// no `MULT:` and no repeat clause. Magnitude `2` agrees between token
/// and `BENEFIT:` prose ("Add 2 to the DC of saving throws made to
/// resist the effects of your channel energy ability").
pub(super) fn oracle_channel_dc(oracle_level: u8, charisma_modifier: i16, selected_feats: &[String]) -> i16 {
    10 + i16::from(oracle_level) / 2
        + charisma_modifier
        + non_stacking_resource_feat_bonus(
            selected_feats,
            IMPROVED_CHANNEL_FEAT_KEY,
            IMPROVED_CHANNEL_DC_BONUS,
        )
}

/// Bone Mystery's Near Death insight bonus on saves against disease,
/// mind-affecting effects, and poison: `+2`, rising to `+4` from Oracle
/// level 11 (the corpus stacks a second `BONUS:VAR|
/// OracleNearDeathSaveBonus|2|PRECLASS:1,Oracle=11` on the base one).
///
/// Deliberately NOT folded into `compute_total_saves`: it applies only
/// against three specific effect categories, and this engine models no
/// per-category save facet, so adding it to the flat save totals would
/// overstate it as an unconditional bonus.
pub(super) fn oracle_near_death_save_bonus(oracle_level: u8) -> i16 {
    if oracle_level >= ORACLE_NEAR_DEATH_UPGRADE_LEVEL {
        ORACLE_NEAR_DEATH_BASE_SAVE_BONUS * 2
    } else {
        ORACLE_NEAR_DEATH_BASE_SAVE_BONUS
    }
}

/// Lore Mystery's Sidestep Secret Reflex delta, ready to layer into
/// `compute_total_saves` (deepening 2026-07-26, task #10, integration
/// ratified by the lead): `BONUS:SAVE|Reflex|max(CHA,DEX)-DEX`.
///
/// Always-on with no activation and no per-day budget, so unlike
/// Warpriest's Strength Surge there is no enforcement to fake by
/// integrating it -- it belongs in the real total. `None` for every
/// non-Oracle or Oracle who did not take this revelation.
pub(super) fn active_oracle_sidestep_secret_reflex_bonus(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
) -> Option<i16> {
    oracle_level_with_revelation(
        input,
        LORE_MYSTERY_SELECTION,
        ORACLE_SIDESTEP_SECRET_REVELATION,
    )?;
    Some(oracle_stat_substitution_delta(
        ability_modifiers.charisma,
        ability_modifiers.dexterity,
    ))
}

/// Nature Mystery's Nature's Whispers Armor Class delta, ready to layer
/// into `compute_combat_baseline` (deepening 2026-07-26, task #10,
/// integration ratified by the lead):
/// `BONUS:COMBAT|AC|(max(DEX,CHA)-DEX)|Type=Ability`. Same always-on,
/// no-budget shape as Sidestep Secret.
pub(super) fn active_oracle_natures_whispers_ac_bonus(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
) -> Option<i16> {
    oracle_level_with_revelation(
        input,
        NATURE_MYSTERY_SELECTION,
        ORACLE_NATURES_WHISPERS_REVELATION,
    )?;
    Some(oracle_stat_substitution_delta(
        ability_modifiers.charisma,
        ability_modifiers.dexterity,
    ))
}

/// Oracle's curse level: `OracleCurseLVL = OracleLVL+((TL-OracleLVL)/2)`
/// (deepening 2026-07-26, task #10), verified directly against
/// `apg_abilities_globalvar.lst`. For a single-class Oracle this
/// collapses to the Oracle level, but it is a HALF progression for a
/// multiclass Oracle -- the non-Oracle half of total level contributes at
/// half rate. Deliberately computed from both levels rather than
/// hardcoded to the Oracle level, since this engine does support
/// multiclass characters.
pub(super) fn oracle_curse_level(oracle_level: u8, total_character_level: u8) -> i16 {
    let oracle_level = i16::from(oracle_level);
    let total = i16::from(total_character_level);
    oracle_level + (total - oracle_level) / 2
}

/// Lame Curse's base-land-speed reduction (deepening 2026-07-26, task
/// #10), verified directly against `apg_abilities_class.lst`'s own
/// `BONUS:VAR|OracleLameEffect|10|PREMOVE:1,Walk=30` /
/// `|5|!PREMOVE:1,Walk=30` pair, applied via
/// `BONUS:MOVEADD|TYPE.Walk|-OracleLameEffect`. Returned as a positive
/// magnitude; it is a reduction.
pub(super) fn oracle_lame_speed_penalty(base_land_speed_feet: i16) -> i16 {
    if base_land_speed_feet >= ORACLE_LAME_FULL_SPEED_FEET {
        ORACLE_LAME_PENALTY_AT_THIRTY_FEET
    } else {
        ORACLE_LAME_PENALTY_BELOW_THIRTY_FEET
    }
}

/// Deaf Curse's initiative penalty (deepening 2026-07-26, task #10),
/// re-derived directly against the base record's own
/// `BONUS:VAR|OracleDeafInitPenalty|-4` plus the two `.MOD` records'
/// `|2|PREVARGTEQ:OracleCurseLVL,5` and `|2|PREVARGTEQ:OracleCurseLVL,10`
/// steps, which ADD toward zero rather than deepening the penalty:
/// **-4 at curse level 1-4, -2 at 5-9, 0 at 10+**. The level-15 `.MOD`
/// record's own DESC ("do not receive any initiative penalty for being
/// deaf") independently confirms the penalty has reached 0 by then.
pub(super) fn oracle_deaf_initiative_penalty(curse_level: i16) -> i16 {
    let mut penalty = ORACLE_DEAF_BASE_INITIATIVE_PENALTY;
    if curse_level >= ORACLE_CURSE_TIER_TWO_LEVEL {
        penalty += ORACLE_DEAF_INITIATIVE_PENALTY_STEP;
    }
    if curse_level >= ORACLE_CURSE_TIER_THREE_LEVEL {
        penalty += ORACLE_DEAF_INITIATIVE_PENALTY_STEP;
    }
    penalty
}

/// Deaf Curse's Perception competence bonus (deepening 2026-07-26, task
/// #10): `BONUS:SKILL|Perception|3|PREVARGTEQ:OracleCurseLVL,5`. `None`
/// below curse level 5 -- the bonus genuinely does not exist yet, rather
/// than being zero.
pub(super) fn oracle_deaf_perception_bonus(curse_level: i16) -> Option<i16> {
    (curse_level >= ORACLE_CURSE_TIER_TWO_LEVEL).then_some(ORACLE_DEAF_PERCEPTION_BONUS)
}

/// Wasting Curse's REAL net effect on Intimidate (deepening 2026-07-26,
/// task #10). The corpus carries two tokens: `BONUS:SKILL|STAT.CHA|-4`
/// (every Charisma-based skill) and `BONUS:SKILL|Intimidate|4`.
/// Intimidate IS Charisma-based, so the `+4` exists purely to cancel the
/// `-4` -- matching the published rule text exactly ("a -4 penalty on all
/// Charisma-based skill checks EXCEPT Intimidate"). The net is therefore
/// **zero, not +4**.
///
/// This matters more here than anywhere else in this closure: Intimidate
/// is one of the three skills `compute_selected_skill_modifiers` actually
/// computes, so reading the `+4` token in isolation would land a wrong
/// number on a live total. Because the net is zero, this closure
/// deliberately wires NOTHING into that total and grounds the
/// cancellation as an explicit explanation record instead.
pub(super) fn oracle_wasting_intimidate_net_effect() -> i16 {
    ORACLE_WASTING_CHARISMA_SKILL_PENALTY + ORACLE_WASTING_INTIMIDATE_OFFSET
}

/// Grounds the six Tier-1 Mystery revelations (deepening 2026-07-26,
/// task #10), each gated on BOTH its Mystery and an explicit
/// `choice:oracle_revelation` pick -- see `ORACLE_REVELATION_CHOICE_ID`
/// for why the Mystery alone is not enough. Grounds nothing at all when
/// no recognized revelation is recorded, and never claim-blocks on its
/// own (Oracle's remaining block comes from
/// `push_oracle_other_features_deferred_diagnostic`, which as of Path A
/// canonical narrowing (2026-07-29) stops claim-blocking once a
/// power-grounding Mystery AND a grounded Curse are both recorded).
///
/// Two of the six also layer into REAL computed totals rather than
/// grounding standalone -- Sidestep Secret into the Reflex save and
/// Nature's Whispers into baseline Armor Class -- via
/// `active_oracle_sidestep_secret_reflex_bonus` /
/// `active_oracle_natures_whispers_ac_bonus` at those totals' own sites.
/// The records pushed here explain the magnitude; the totals apply it.
pub(super) fn ground_oracle_tier_one_revelations(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let ability_modifiers = AbilityModifiers {
        strength: ability_modifier(input.chosen.ability_scores.strength),
        dexterity: ability_modifier(input.chosen.ability_scores.dexterity),
        constitution: ability_modifier(input.chosen.ability_scores.constitution),
        intelligence: ability_modifier(input.chosen.ability_scores.intelligence),
        wisdom: ability_modifier(input.chosen.ability_scores.wisdom),
        charisma: ability_modifier(input.chosen.ability_scores.charisma),
    };
    let ability_modifiers = &ability_modifiers;
    let charisma = ability_modifiers.charisma;

    if let Some(oracle_level) =
        oracle_level_with_revelation(input, LIFE_MYSTERY_SELECTION, ORACLE_CHANNEL_REVELATION)
    {
        let uses = oracle_channel_uses_per_day(charisma);
        let dice = oracle_channel_dice(oracle_level);
        let dc = oracle_channel_dc(oracle_level, charisma, &input.chosen.selected_feats);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.life_mystery.channel_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Oracle level {oracle_level} Life Mystery Channel uses per day: 1 + Charisma \
                 modifier ({charisma:+}) = {uses}. A flat daily pool -- no per-use consumption is \
                 tracked here. The corpus declares this ability `SERVESAS` Cleric's own Channel \
                 Energy, but the formula is transcribed from Oracle's own record rather than \
                 shared with Shaman's structurally-identical Channel, per the established \
                 parallel-copy discipline"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.life_mystery.channel_dice".to_owned(),
            value: dice,
            detail: format!(
                "Oracle level {oracle_level} Life Mystery Channel heals \
                 {dice}d{ORACLE_CHANNEL_DIE_SIZE} ((level + 1)/2 = {dice} dice, die size \
                 {ORACLE_CHANNEL_DIE_SIZE}). This engine computes no healing total anywhere, so \
                 this grounds as a standalone magnitude"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.life_mystery.channel_dc".to_owned(),
            value: dc,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   apg_feats.lst's own `CATEGORY=FEAT|Improved Channel.MOD`, which adds
                //   `BONUS:VAR|OracleChannelDC|2`
                "Oracle level {oracle_level} Life Mystery Channel save DC: 10 + level/2 + \
                 Charisma modifier ({charisma:+}) + Improved Channel feat ({:+}) = {dc}. That \
                 feat's own Core Rulebook record names only Cleric/Paladin channel DC variables \
                 this engine computes no total for; its reach here comes from the Advanced \
                 Player's Guide's own modification of that feat, which adds +2 to the Oracle's \
                 channel DC",
                non_stacking_resource_feat_bonus(
                    &input.chosen.selected_feats,
                    IMPROVED_CHANNEL_FEAT_KEY,
                    IMPROVED_CHANNEL_DC_BONUS
                )
            ),
        });
    }

    if let Some(oracle_level) = oracle_level_with_revelation(
        input,
        LORE_MYSTERY_SELECTION,
        ORACLE_SIDESTEP_SECRET_REVELATION,
    ) {
        let delta = oracle_stat_substitution_delta(charisma, ability_modifiers.dexterity);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.lore_mystery.sidestep_secret_reflex_bonus".to_owned(),
            value: delta,
            detail: format!(
                "Oracle level {oracle_level} Lore Mystery Sidestep Secret adds \
                 max(Charisma, Dexterity) - Dexterity = max({charisma:+}, {:+}) - {:+} = \
                 {delta:+} to the Reflex save, letting Charisma stand in for Dexterity when it \
                 is higher. This is genuinely {delta:+} for this character. Unlike most Oracle \
                 facts this one is INTEGRATED: it is folded into the real \
                 `defense.total_save.reflex` total, since it is always on with no activation and \
                 no per-day budget to fake",
                ability_modifiers.dexterity, ability_modifiers.dexterity
            ),
        });
    }

    if let Some(oracle_level) = oracle_level_with_revelation(
        input,
        NATURE_MYSTERY_SELECTION,
        ORACLE_NATURES_WHISPERS_REVELATION,
    ) {
        let delta = oracle_stat_substitution_delta(charisma, ability_modifiers.dexterity);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.nature_mystery.natures_whispers_ac_bonus".to_owned(),
            value: delta,
            detail: format!(
                "Oracle level {oracle_level} Nature Mystery Nature's Whispers adds \
                 max(Dexterity, Charisma) - Dexterity = {delta:+} to Armor Class, letting \
                 Charisma stand in for Dexterity when it is higher. INTEGRATED into the real \
                 `defense.baseline_armor_class` total (the same total Inquisitor's Protection \
                 judgment already lands on). The corpus applies it as an untyped ability bonus \
                 on top of the Dexterity contribution, so where the worn armor's maximum \
                 Dexterity bonus caps that contribution the two interact exactly as the raw \
                 tokens do. The revelation's parallel Combat Maneuver Defense half is not \
                 integrated -- this engine computes no Combat Maneuver Defense total"
            ),
        });
    }

    if let Some(oracle_level) =
        oracle_level_with_revelation(input, LORE_MYSTERY_SELECTION, ORACLE_LORE_KEEPER_REVELATION)
    {
        let delta = oracle_stat_substitution_delta(charisma, ability_modifiers.intelligence);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.lore_mystery.lore_keeper_knowledge_bonus".to_owned(),
            value: delta,
            detail: format!(
                "Oracle level {oracle_level} Lore Mystery Lore Keeper adds \
                 max(Charisma, Intelligence) - Intelligence = {delta:+} to all \
                 {ORACLE_LORE_KEEPER_KNOWLEDGE_SKILL_COUNT} Knowledge skills named in its own \
                 corpus token, letting Charisma stand in for Intelligence when it is higher. No \
                 Knowledge skill is among the three skills this engine computes, so this grounds \
                 as a standalone flat magnitude, the same shape as Bard's Bardic Knowledge"
            ),
        });
    }

    if let Some(oracle_level) =
        oracle_level_with_revelation(input, BONE_MYSTERY_SELECTION, ORACLE_NEAR_DEATH_REVELATION)
    {
        let bonus = oracle_near_death_save_bonus(oracle_level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.bone_mystery.near_death_save_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Oracle level {oracle_level} Bone Mystery Near Death grants a +{bonus} insight \
                 bonus on saving throws against disease, mind-affecting effects, and poison \
                 (+2, rising to +4 from Oracle level {ORACLE_NEAR_DEATH_UPGRADE_LEVEL}). \
                 Deliberately NOT folded into the real save totals: it applies only against \
                 those three effect categories, and this engine models no per-category save \
                 facet, so adding it to the flat totals would overstate it as unconditional"
            ),
        });
    }

    if let Some(oracle_level) =
        oracle_level_with_revelation(input, FLAME_MYSTERY_SELECTION, ORACLE_CINDER_DANCE_REVELATION)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.flame_mystery.cinder_dance_speed_bonus".to_owned(),
            value: ORACLE_CINDER_DANCE_SPEED_BONUS,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:MOVEADD|TYPE.WALK|10`
                "Oracle level {oracle_level} Flame Mystery Cinder Dance increases base land speed by \
                 {ORACLE_CINDER_DANCE_SPEED_BONUS} feet, always on with no level gate -- the exact \
                 mirror of the Lame Curse's own reduction. This engine computes no movement total, \
                 so it grounds standalone. The corpus record excludes the Oracle's Lame curse \
                 outright, making this revelation mutually exclusive with the Lame \
                 Curse; the bonus feats it also grants at Oracle 5 and 10 (Nimble Moves, Acrobatic \
                 Steps) carry no magnitude and are not modelled"
            ),
        });
    }

    // SD31-E4-F2-001: Battle Mystery's Battlecry, the seventh revelation this
    // deepening grounds and the first wired through
    // `archetype_resolver::chooser_option_selected` rather than the
    // hand-rolled `oracle_level_with_revelation` shape above -- see
    // `oracle_level_with_battlecry_revelation`'s own doc comment for why it
    // is a separate function rather than a rewrite of the shared helper.
    if let Some(oracle_level) = oracle_level_with_battlecry_revelation(input) {
        let bonus = oracle_battlecry_bonus(oracle_level);
        // `BONUS:VAR|OracleBattlecryDuration|CHA` -- PCGen's bare stat
        // abbreviation (`CHA`) is the ability MODIFIER, not the score
        // (`CHASCORE`); see the game system's own stat definition,
        // `STATNAME:Charisma ABB:CHA STATMOD:floor(SCORE/2)-5`, and the
        // sibling `OracleRevelationDC|10+classlevel("Oracle")/2+CHA` token
        // in the same corpus record block, which is unambiguously the
        // Cha-modifier-based revelation DC rule. `charisma` here is
        // already the modifier (bound above from `ability_modifiers`).
        let duration_rounds = charisma;
        let times_per_day = oracle_battlecry_times_per_day(oracle_level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.battle_mystery.battlecry_bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   -- `BONUS:VAR|OracleBattlecryBonus|1` plus `|1|PRECLASS:1,Oracle=10`
                "Oracle level {oracle_level} Battle Mystery Battlecry grants allies within 100 feet \
                 a +{bonus} morale bonus on attack rolls, skill checks, and saving throws (+1, \
                 rising to +2 from Oracle level {ORACLE_BATTLECRY_UPGRADE_LEVEL}). This engine \
                 computes no ally entity anywhere, so the bonus grounds as a standalone flat \
                 magnitude, the same 'grant-only identity record, no execution engine' idiom used \
                 throughout this session"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.battle_mystery.battlecry_duration_rounds".to_owned(),
            value: duration_rounds,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|OracleBattlecryDuration|CHA`; PCGen's bare `CHA` token is the
                //   ability modifier, not the score -- confirmed against the game system's own
                //   `STATMOD:floor(SCORE/2)-5` definition and the sibling revelation-DC token in
                //   the same corpus record
                "Oracle level {oracle_level} Battle Mystery Battlecry's morale bonus lasts a number \
                 of rounds equal to the Oracle's Charisma MODIFIER. At a Charisma modifier of \
                 {duration_rounds:+} this is {duration_rounds} rounds. This engine tracks no \
                 round-by-round duration state, so this grounds as a standalone flat magnitude"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.battle_mystery.battlecry_uses_per_day".to_owned(),
            value: times_per_day,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|OracleBattlecryTimes|1+classlevel(\"Oracle\")/5`
                "Oracle level {oracle_level} Battle Mystery Battlecry uses per day: 1 + Oracle level \
                 / {ORACLE_BATTLECRY_TIMES_PER_DAY_LEVEL_DIVISOR}. At Oracle level {oracle_level} \
                 this is 1 + {oracle_level} / {ORACLE_BATTLECRY_TIMES_PER_DAY_LEVEL_DIVISOR} = \
                 {times_per_day}. This grounds only the flat daily use count; it performs no per-use \
                 consumption tracking"
            ),
        });
    }

    // SD31-E4-F2-002: the 4 remaining Mysteries wired through
    // `chooser_option_selected`, same shape as Battle Mystery above.
    if let Some(oracle_level) = oracle_level_with_steelbreaker_skin_revelation(input) {
        let damage = oracle_steelbreaker_skin_damage(oracle_level);
        let duration = oracle_steelbreaker_skin_duration_minutes(oracle_level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.stone_mystery.steelbreaker_skin_damage".to_owned(),
            value: damage,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|OracleSteelbreakerSkinDamage|classlevel(\"Oracle\")`
                "Oracle level {oracle_level} Stone Mystery Steelbreaker Skin: as a standard action, \
                 any weapon striking you takes {damage} points of damage. Usable once per day; not \
                 folded into any weapon-damage total this engine computes, since no weapon-vs-Oracle \
                 interaction exists here. From Oracle level 15 the damage also ignores up to 10 \
                 points of hardness -- not modelled, no hardness facet exists in this engine"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.stone_mystery.steelbreaker_skin_duration_minutes"
                .to_owned(),
            value: duration,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|OracleSteelbreakerSkinDuration|classlevel(\"Oracle\")`
                "Oracle level {oracle_level} Stone Mystery Steelbreaker Skin lasts {duration} \
                 minutes per use. This engine tracks no round-by-round or minute-by-minute duration \
                 state, so this grounds as a standalone flat magnitude"
            ),
        });
    }

    if let Some(oracle_level) = oracle_level_with_icy_skin_revelation(input) {
        let resistance = oracle_elemental_resistance(oracle_level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.waves_mystery.icy_skin_cold_resistance".to_owned(),
            value: resistance,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `ABILITY:...Immunity to Cold|PRECLASS:1,Oracle=17`
                "Oracle level {oracle_level} Waves Mystery Icy Skin grants cold resistance \
                 {resistance} (`5 + OracleColdResistanceBonus`, always on, stepping to \
                 +{ORACLE_ELEMENTAL_RESISTANCE_TIER_TWO_BONUS} more at Oracle level \
                 {ORACLE_ELEMENTAL_RESISTANCE_TIER_TWO_LEVEL} and \
                 +{ORACLE_ELEMENTAL_RESISTANCE_TIER_THREE_BONUS} more (additive, not replacing) at \
                 Oracle level {ORACLE_ELEMENTAL_RESISTANCE_TIER_THREE_LEVEL}). This engine computes \
                 no resistance/DR total, so this grounds as a standalone flat magnitude. From Oracle \
                 level 17 the corpus record replaces resistance with cold IMMUNITY; this engine does \
                 not model that qualitative shift, so this magnitude understates a level-17+ \
                 character and is honest about doing so rather than fabricating an immunity sentinel \
                 value"
            ),
        });
    }

    if let Some(oracle_level) = oracle_level_with_spark_skin_revelation(input) {
        let resistance = oracle_elemental_resistance(oracle_level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.wind_mystery.spark_skin_electricity_resistance"
                .to_owned(),
            value: resistance,
            detail: format!(
                "Oracle level {oracle_level} Wind Mystery Spark Skin grants electricity \
                 resistance {resistance} -- the identical formula shape and step values as \
                 Waves Mystery's Icy Skin (`ORACLE_ELEMENTAL_RESISTANCE_TIER_TWO_LEVEL`/\
                 `_TIER_THREE_LEVEL`), electricity instead of cold. This engine computes no \
                 resistance/DR total, so this grounds as a standalone flat magnitude, with the \
                 same honest level-17+ immunity understatement Icy Skin's own detail names"
            ),
        });
    }

    if let Some(oracle_level) = oracle_level_with_coat_of_many_stars_revelation(input) {
        let ac_bonus = oracle_coat_of_many_stars_ac_bonus(oracle_level);
        let duration_hours = oracle_coat_of_many_stars_duration_hours(oracle_level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.heavens_mystery.coat_of_many_stars_ac_bonus".to_owned(),
            value: ac_bonus,
            detail: format!(
                "Oracle level {oracle_level} Heavens Mystery Coat of Many Stars grants a \
                 +{ac_bonus} armor bonus to AC (`{ORACLE_COAT_OF_MANY_STARS_BASE_AC_BONUS} + \
                 {ORACLE_COAT_OF_MANY_STARS_AC_STEP}*max(0,floor((Oracle level - \
                 {ORACLE_COAT_OF_MANY_STARS_STEP_START_LEVEL})/{ORACLE_COAT_OF_MANY_STARS_STEP_INTERVAL}))`), \
                 usable {duration_hours} hour(s) per day (not necessarily consecutive). Not \
                 folded into the real `defense.baseline_armor_class` total: unlike Nature's \
                 Whispers, this bonus is activation-gated (a conjured coat you don a standard \
                 action to summon), not always on, and integrating an activation-gated bonus \
                 into a flat total would overstate an Oracle who has not activated it. From \
                 Oracle level 13 the coat also grants DR 5/slashing -- not modelled, no DR \
                 facet exists in this engine"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.heavens_mystery.coat_of_many_stars_duration_hours"
                .to_owned(),
            value: duration_hours,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|OracleCoatofManyStarsDuration|classlevel(\"Oracle\")`
                "Oracle level {oracle_level} Heavens Mystery Coat of Many Stars is usable \
                 {duration_hours} hour(s) per day, equal to the Oracle's class level; the duration \
                 does not need to be consecutive. This engine tracks no hour-by-hour duration state, \
                 so this grounds as a standalone flat magnitude"
            ),
        });
    }
}

/// Grounds Oracle's Curse choice (v0.6 alpha swarm, risks item 8, Oracle
/// full-build closure): recognizes
/// `choice:oracle_curse -> curse:clouded_vision` and, if present, grounds
/// Clouded Vision's own flat 30-foot vision-range cap (unconditional once
/// chosen). Mirrors `ground_or_block_oracle_mystery`'s own three-branch
/// dispatch shape.
pub(super) fn ground_or_block_oracle_curse(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let curse_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == ORACLE_CURSE_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();

    let total_character_level: u8 =
        input.chosen.class_levels.iter().map(|c| c.level).sum();
    let curse_level = oracle_curse_level(level, total_character_level);

    if curse_selections.contains(&LAME_CURSE_SELECTION) {
        ground_oracle_lame_curse(input, curse_level, explanations, diagnostics);
    }
    if curse_selections.contains(&WASTING_CURSE_SELECTION) {
        ground_oracle_wasting_curse(curse_level, explanations, diagnostics);
    }
    if curse_selections.contains(&DEAF_CURSE_SELECTION) {
        ground_oracle_deaf_curse(curse_level, explanations, diagnostics);
    }

    if curse_selections.contains(&CLOUDED_VISION_CURSE_SELECTION) {
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.clouded_vision_curse.vision_range_cap".to_owned(),
            value: ORACLE_CLOUDED_VISION_RANGE_FEET,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|OracleCloudedVisionRange|30` tag
                "Oracle with the Clouded Vision Curse cannot see beyond \
                 {ORACLE_CLOUDED_VISION_RANGE_FEET} feet (PF1 Advanced Player's Guide, verified \
                 directly against `apg_abilities_class.lst`'s own), but sees as if under darkvision \
                 within that range. Grounds only the flat vision-range magnitude -- this headless \
                 engine computes no vision/darkvision total anywhere to integrate this \
                 restriction-plus-benefit pair into"
            ),
        });
    }

    let recognized_curse = curse_selections.iter().any(|selection| {
        matches!(
            *selection,
            CLOUDED_VISION_CURSE_SELECTION
                | LAME_CURSE_SELECTION
                | WASTING_CURSE_SELECTION
                | DEAF_CURSE_SELECTION
        )
    });
    if recognized_curse {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.oracle.curses_beyond_grounded_set.unmodeled".to_owned(),
            message: "Oracle Curse content beyond Clouded Vision, Lame, Wasting, and Deaf remains \
                 unmodeled: Haunted (which carries no numeric token of any kind in the corpus -- \
                 a genuine no-op, not a transcription gap) and Tongues (an 8-language chooser \
                 whose only token is an ability-pool count) are not implemented, and the \
                 non-numeric halves of the grounded curses (Lame's encumbrance/fatigue \
                 immunities, Wasting's sickened/disease/nauseated immunities, Deaf's scent and \
                 tremorsense) are named but not modelled. This does not block an otherwise-valid \
                 curse posture"
                .to_owned(),
            claim_blocking: false,
        });
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.oracle.curse_powers.unsupported".to_owned(),
            message: "Oracle remains blocked on its Curse powers burden: no recognized Curse \
                 choice is present (only Clouded Vision, Lame, Wasting, and Deaf are genuinely \
                 grounded in this codebase), so no Oracle Curse-power support is claimed"
                .to_owned(),
            claim_blocking: true,
        });
    }
}

/// Grounds the Lame Curse (deepening 2026-07-26, task #10): a flat base-
/// land-speed reduction whose magnitude depends on the character's own
/// race speed. This engine computes no movement total anywhere, so the
/// reduction grounds as a standalone magnitude alongside the resulting
/// speed, both derived from the authoritative CRB race table rather than
/// assumed to be 30 feet.
pub(super) fn ground_oracle_lame_curse(
    input: &CharacterInput,
    curse_level: i16,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(base_speed) = base_land_speed_feet(&input.chosen.race_id) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.oracle.lame_curse.unknown_race_speed".to_owned(),
            message: format!(
                "Lame Curse's speed reduction depends on the character's own base land speed \
                 (10 feet at a 30-foot base, 5 feet below that), but race \
                 \"{}\" has no grounded Speed trait in this codebase's race table, so no \
                 magnitude is claimed",
                input.chosen.race_id
            ),
            claim_blocking: true,
        });
        return;
    };
    let penalty = oracle_lame_speed_penalty(base_speed);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.oracle.lame_curse.base_land_speed_penalty".to_owned(),
        value: penalty,
        detail: format!(
            "Oracle with the Lame Curse has a permanently wounded leg, reducing base land speed \
             by {penalty} feet (PF1 Advanced Player's Guide: 10 feet at a 30-foot base speed, \
             5 feet for a slower race). This character's base land speed is {base_speed} feet, \
             read from the CRB race table's own Speed trait, so the reduction is {penalty} feet, \
             leaving {} feet. Grounds the flat magnitude only -- this engine computes no \
             movement total to apply it to. The curse's own \"speed is never reduced by \
             encumbrance\" clause (and, at curse level {ORACLE_CURSE_TIER_THREE_LEVEL}+, by \
             armor) plus the fatigue immunities are non-numeric and are not modelled; this \
             character's curse level is {curse_level}",
            base_speed - penalty
        ),
    });
}

/// Grounds the Wasting Curse (deepening 2026-07-26, task #10). Its
/// Charisma-skill penalty is real and flat, but its headline trap is the
/// Intimidate cancellation -- see `oracle_wasting_intimidate_net_effect`.
pub(super) fn ground_oracle_wasting_curse(
    curse_level: i16,
    explanations: &mut Vec<ComputationExplanation>,
    _diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.oracle.wasting_curse.charisma_skill_penalty".to_owned(),
        value: ORACLE_WASTING_CHARISMA_SKILL_PENALTY,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:SKILL|STAT.CHA|-4`
            "Oracle with the Wasting Curse takes a {ORACLE_WASTING_CHARISMA_SKILL_PENALTY} penalty \
             on all Charisma-based skill checks (PF1 Advanced Player's Guide, verified directly \
             against `apg_abilities_class.lst`'s own), with Intimidate explicitly excepted. Of the \
             three skills this engine actually computes, only Intimidate is Charisma-based -- and it \
             is the excepted one -- so this penalty lands on no computed total and grounds \
             standalone"
        ),
    });
    let net = oracle_wasting_intimidate_net_effect();
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.oracle.wasting_curse.intimidate_net_effect".to_owned(),
        value: net,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   The corpus carries two tokens: `BONUS:SKILL|STAT.CHA|-4` (which hits Intimidate, a
            //   Charisma-based skill) and `BONUS:SKILL|Intimidate|4` (which exists solely to cancel
            //   it), matching the published rule \"a -4 penalty on all Charisma-based skill checks
            //   except Intimidate\".
            "Oracle with the Wasting Curse has a NET {net} effect on Intimidate, not \
             +{ORACLE_WASTING_INTIMIDATE_OFFSET}. Grounded explicitly because Intimidate IS one of \
             this engine's three computed skill totals: reading the +4 token alone would land a \
             wrong number on a live total, so this closure deliberately wires nothing into that \
             total. This character's curse level is {curse_level}; the sickened/disease/nauseated \
             immunities at curse levels 5/10/15 are non-numeric and are not modelled"
        ),
    });
}

/// Grounds the Deaf Curse (deepening 2026-07-26, task #10): an initiative
/// penalty that walks back toward zero as curse level rises, plus a flat
/// Perception competence bonus from curse level 5.
pub(super) fn ground_oracle_deaf_curse(
    curse_level: i16,
    explanations: &mut Vec<ComputationExplanation>,
    _diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let initiative_penalty = oracle_deaf_initiative_penalty(curse_level);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.oracle.deaf_curse.initiative_penalty".to_owned(),
        value: initiative_penalty,
        detail: format!(
            "Oracle with the Deaf Curse takes a {initiative_penalty} penalty on initiative \
             checks at curse level {curse_level} (PF1 Advanced Player's Guide: -4 at curse \
             levels 1-4, -2 at 5-9, and 0 from 10 on, as the two `.MOD` records each add +2 back \
             toward zero). This engine computes no initiative total anywhere, so this grounds as \
             a standalone flat magnitude, the same shape as Inquisitor's own Cunning Initiative"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.oracle.deaf_curse.opposed_perception_penalty".to_owned(),
        value: ORACLE_DEAF_OPPOSED_PERCEPTION_PENALTY,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:SITUATION|Perception=Opposed|-4`
            "Oracle with the Deaf Curse takes a {ORACLE_DEAF_OPPOSED_PERCEPTION_PENALTY} penalty on \
             OPPOSED Perception checks, at every curse level. Grounds despite being situational: the \
             magnitude is fixed and applies to this character's own roll, needing nothing known \
             about the opposing creature -- unlike a Studied-Target-style bonus, which requires a \
             persistent tracked relationship with a specific opponent that this engine models \
             nowhere. Perception is not among the three skills this engine computes, so this grounds \
             standalone"
        ),
    });
    match oracle_deaf_perception_bonus(curse_level) {
        Some(bonus) => explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.deaf_curse.perception_bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:SKILL|Perception|3|PREVARGTEQ:OracleCurseLVL,5`
                "Oracle with the Deaf Curse gains a +{bonus} competence bonus on Perception checks \
                 that do not rely on hearing, from curse level {ORACLE_CURSE_TIER_TWO_LEVEL}. \
                 Perception is not among the three skills this engine computes, so this grounds \
                 standalone. The curse's own automatic failure on hearing-based Perception checks is \
                 a qualitative rule, not a magnitude, and is not modelled"
            ),
        }),
        None => explanations.push(ComputationExplanation {
            id: "class_feature.apg.oracle.deaf_curse.perception_bonus".to_owned(),
            value: 0,
            detail: format!(
                "Oracle with the Deaf Curse has no Perception bonus at curse level \
                 {curse_level}: correctly absent below curse level \
                 {ORACLE_CURSE_TIER_TWO_LEVEL} by the PF1 Advanced Player's Guide level gate; \
                 the at-grant magnitude is named but not computed"
            ),
        }),
    }
}

/// Top-level Oracle class-feature dispatch (v0.6 alpha swarm, risks item
/// 8, Oracle full-build closure, 8th ACG/APG class-specific closure):
/// grounds the real known-spell posture, the Mystery choice, and the
/// Curse choice, then always pushes the narrower
/// `other_features_deferred` diagnostic naming the genuinely still-
/// missing pieces.
///
/// **Corrected 2026-07-29 (Path A canonical narrowing).** This comment
/// used to assert Oracle "permanently stays claim-blocked (no MVP
/// narrowing was found for Cure Wounds/Inflict Wounds/Tongues this
/// slice)". Cure Wounds was never the narrowing Oracle needed: the class
/// is chooser-shaped, and the narrowing is the canonical Mystery/Curse
/// pair, exactly as for Arcanist's Metamagic Knowledge and Cleric's Good
/// domain. `push_oracle_other_features_deferred_diagnostic` now stops
/// claim-blocking once BOTH a power-grounding Mystery and a grounded
/// Curse are recorded, and keeps blocking in every other posture.
pub(super) fn ground_or_block_oracle_class_features(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let known_spell_unmet = unmet_oracle_known_spell_conditions(input, level);
    if known_spell_unmet.is_empty() {
        ground_oracle_known_spells(input, level, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.apg.oracle.known_spells.unsupported".to_owned(),
            message: format!(
                "Oracle remains blocked on its known-spell posture burden: {}",
                known_spell_unmet.join("; ")
            ),
            claim_blocking: true,
        });
    }

    ground_or_block_oracle_mystery(input, explanations, diagnostics);

    // SD-32 T12 Epic 8 row 18 cycle 5: `push_generic_pool_group_selection_
    // magnitude` was tried here and DELIBERATELY WITHDRAWN, not merely
    // "not attempted". Unlike Domain/Bloodline (whose powers are ALL
    // automatically granted once the group is chosen -- confirmed by this
    // file's own existing Cleric/Sorcerer wiring, which never requires a
    // second budgeted pick), Oracle Mystery members are mostly
    // REVELATIONS -- a genuinely budgeted PF1 sub-choice this codebase
    // already, correctly, gates on an explicit `ORACLE_REVELATION_CHOICE_ID`
    // pick (see `oracle_level_with_revelation`'s own doc comment). A
    // group-selection-grants-everything pass is WRONG for this pool: it
    // was caught live by this file's own regression suite
    // (`oracle_dispatch_widening_safety_tests::
    // a_mystery_pick_alone_grounds_no_tier_one_revelation`, which failed
    // when this was wired -- a bare `mystery:lore` pick started grounding
    // Sidestep Secret with no revelation pick recorded at all). Reverted
    // rather than weakening that test (`decisions.md §1a`). Mystery
    // therefore stays wired ONLY through the pre-existing 10-mystery hand
    // model above; the other 11 real mystery groups are a genuine,
    // named "cannot be wired without a budgeted-choice-aware mechanism"
    // gap, not a silently skipped one.
    ground_oracle_tier_one_revelations(input, explanations);
    ground_or_block_oracle_curse(input, level, explanations, diagnostics);

    // The Cinder Dance revelation's own corpus record carries
    // `!PREABILITY:1,CATEGORY=Special Ability,Oracle ~ Lame`, so a
    // character holding both it and the Lame Curse is genuinely illegal
    // in PF1 -- named honestly rather than silently computing the two
    // opposing speed magnitudes as if they cancelled.
    let has_choice = |choice_set_id: &str, selection: &str| {
        input
            .chosen
            .selected_choices
            .iter()
            .any(|c| c.choice_set_id == choice_set_id && c.selection_id == selection)
    };
    if has_choice(ORACLE_CURSE_CHOICE_ID, LAME_CURSE_SELECTION)
        && has_choice(ORACLE_REVELATION_CHOICE_ID, ORACLE_CINDER_DANCE_REVELATION)
    {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.oracle.cinder_dance_lame_mutually_exclusive".to_owned(),
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `!PREABILITY:1,CATEGORY=Special Ability,Oracle ~ Lame`
            message: "This character holds both the Lame Curse and the Cinder Dance revelation, \
                 which the corpus makes mutually exclusive (Cinder Dance carries). Both speed \
                 magnitudes are reported as the records themselves state them; they are NOT netted \
                 against each other, because this combination cannot legally exist rather than \
                 resolving to some combined speed"
                .to_owned(),
            claim_blocking: true,
        });
    }

    push_oracle_other_features_deferred_diagnostic(input, diagnostics);
}

/// Whether this Oracle's Mystery selection both (a) names only Mysteries
/// this codebase recognizes and (b) actually GROUNDS a power for the one
/// it names (Path A canonical narrowing, 2026-07-29).
///
/// (b) is the part that matters and the part a naive "is it in the
/// recognized list?" check would get wrong. Life is the only Mystery whose
/// grounded power (Healing Hands, `BONUS:SKILL|Heal|4`) follows from the
/// Mystery choice alone; Lore, Nature, Bone and Flame each ground only
/// through an explicitly recorded `choice:oracle_revelation` pick, because
/// revelations are a budgeted selection rather than an automatic grant
/// (see `ORACLE_REVELATION_CHOICE_ID`). Treating a bare `mystery:lore`
/// with no revelation as "recognized" would let a character whose Mystery
/// contributes literally nothing reach `Computed`.
pub(super) fn oracle_mystery_grounds_a_power(input: &CharacterInput) -> bool {
    let mystery_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == ORACLE_MYSTERY_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();

    let all_recognized = mystery_selections.iter().all(|selection| {
        matches!(
            *selection,
            LIFE_MYSTERY_SELECTION
                | LORE_MYSTERY_SELECTION
                | NATURE_MYSTERY_SELECTION
                | BONE_MYSTERY_SELECTION
                | FLAME_MYSTERY_SELECTION
                | BATTLE_MYSTERY_SELECTION
                | STONE_MYSTERY_SELECTION
                | WAVES_MYSTERY_SELECTION
                | WIND_MYSTERY_SELECTION
                | HEAVENS_MYSTERY_SELECTION
        )
    });
    if !all_recognized {
        return false;
    }

    if mystery_selections.contains(&LIFE_MYSTERY_SELECTION) {
        return true;
    }
    let revelation_grounds = [
        (LORE_MYSTERY_SELECTION, ORACLE_SIDESTEP_SECRET_REVELATION),
        (LORE_MYSTERY_SELECTION, ORACLE_LORE_KEEPER_REVELATION),
        (NATURE_MYSTERY_SELECTION, ORACLE_NATURES_WHISPERS_REVELATION),
        (BONE_MYSTERY_SELECTION, ORACLE_NEAR_DEATH_REVELATION),
        (FLAME_MYSTERY_SELECTION, ORACLE_CINDER_DANCE_REVELATION),
    ]
    .iter()
    .any(|(mystery, revelation)| oracle_level_with_revelation(input, mystery, revelation).is_some());
    // SD31-E4-F2-001/002: Battle/Stone/Waves/Wind/Heavens Mysteries checked
    // separately -- each is grounded through the `chooser_option_selected`
    // primitive (a dedicated `oracle_level_with_<X>_revelation` function per
    // Mystery), not the `revelation_grounds` array's shared
    // `oracle_level_with_revelation` helper.
    revelation_grounds
        || oracle_level_with_battlecry_revelation(input).is_some()
        || oracle_level_with_steelbreaker_skin_revelation(input).is_some()
        || oracle_level_with_icy_skin_revelation(input).is_some()
        || oracle_level_with_spark_skin_revelation(input).is_some()
        || oracle_level_with_coat_of_many_stars_revelation(input).is_some()
}

/// Whether this Oracle's Curse selection names at least one of the four
/// genuinely grounded Curses and nothing this codebase does not recognize
/// (Path A canonical narrowing, 2026-07-29).
///
/// Unlike Mysteries, all four grounded Curses follow from the Curse choice
/// alone -- there is no second budgeted pick between choosing a Curse and
/// receiving its effect -- so recognition and grounding coincide here.
/// A PF1 oracle has exactly ONE curse, so a second selection is already an
/// illegal posture; requiring every selection to be recognized keeps an
/// ungrounded one (Haunted, Tongues, Blackened, ...) from riding a
/// recognized one to `Computed`.
pub(super) fn oracle_curse_grounds_a_power(input: &CharacterInput) -> bool {
    let curse_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == ORACLE_CURSE_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();

    let recognized = |selection: &str| {
        matches!(
            selection,
            CLOUDED_VISION_CURSE_SELECTION
                | LAME_CURSE_SELECTION
                | WASTING_CURSE_SELECTION
                | DEAF_CURSE_SELECTION
        )
    };

    !curse_selections.is_empty() && curse_selections.iter().all(|s| recognized(s))
}

/// Pushes the new, narrower diagnostic replacing
/// `class_feature.apg.oracle.unsupported` for Oracle specifically (v0.6
/// alpha swarm, risks item 8, Oracle full-build closure): named ONLY the
/// genuinely still-missing pieces. Pushed unconditionally regardless of
/// the known-spell/Mystery/Curse postures' own states, mirroring
/// Warpriest's/Arcanist's own diagnostic-honesty pattern.
///
/// **Superseded in one direction (Path A canonical narrowing,
/// 2026-07-29).** This diagnostic's own doc comment and
/// `ground_or_block_oracle_class_features`'s used to record that Oracle
/// "permanently stays claim-blocked (no MVP narrowing was found for Cure
/// Wounds/Inflict Wounds/Tongues this slice)". That was a true statement
/// about a *slice*, not about the class: the MVP narrowing Oracle needed
/// was never Cure Wounds -- it was the same canonical-chooser narrowing
/// Arcanist's Metamagic Knowledge and Cleric's Good domain already ship.
/// An Oracle who has genuinely recorded BOTH a Mystery that grounds a real
/// power AND a grounded Curse has every claim this engine makes about her
/// actually computed, so this remainder is named without blocking. Every
/// other posture -- no Mystery, no Curse, or one this codebase grounds
/// nothing for -- keeps the original claim-blocking behavior exactly, and
/// the separate `mystery_powers`/`curse_powers` diagnostics claim-block
/// those cases independently.
pub(super) fn push_oracle_other_features_deferred_diagnostic(
    input: &CharacterInput,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let canonical_choices_ground_powers =
        oracle_mystery_grounds_a_power(input) && oracle_curse_grounds_a_power(input);
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.apg.oracle.other_features_deferred.unsupported".to_owned(),
        message: format!(
            "{ORACLE_CLASS_ID} remains blocked beyond its base-attack-bonus/base-save chassis \
             pillar, its known-spell posture, Life Mystery's own Healing Hands, and the Clouded \
             Vision, Lame, Wasting, and Deaf Curses, and at least one revelation each from the \
             Bone, Flame, Life, Lore, Nature and Battle Mysteries (Battle's own Battlecry \
             grounded SD31-E4-F2-001, wired through the new chooser_option_selected \
             primitive): Haunted (a genuine no-op -- zero \
             numeric tokens in the corpus), Tongues (an 8-language chooser with no magnitude), \
             Orisons (the `CAST:0,3` at-will 0-level set -- the known-spell grounding covers \
             spell levels 1-3 only), the four Mysteries with NO grounded revelation at all \
             (Heavens, Stone, Waves, Wind) plus every ungrounded revelation within \
             the six that do, Oracle's own Mystery-granted bonus spell-list \
             portion of `SPELLLIST:2|Cleric|Oracle`, and spontaneous Cure Wounds/Inflict Wounds \
             conversion (mirrors Cleric's own unmodeled spontaneous-conversion gap) remain \
             ungrounded anywhere in this codebase; no class-feature or spell execution is \
             fabricated in this bounded chassis baseline. Orisons and the four \
             no-revelation Mysteries were previously covered only by the vague phrase \
             \"the remaining Mystery revelations\" (task #76). {}",
            if canonical_choices_ground_powers {
                "This character HAS recorded both a Mystery that grounds a real power and a \
                 grounded Curse, so this remainder is named but no longer claim-blocking -- the \
                 same canonical-narrowing posture Arcanist's own exploits_deferred and Cleric's \
                 own Good-domain seam already ship. Every item listed above genuinely remains \
                 ungrounded; none of them is silently fabricated."
            } else {
                "No Mystery-and-Curse pair that grounds real powers is recorded, so this \
                 remainder stays claim-blocking."
            }
        ),
        claim_blocking: !canonical_choices_ground_powers,
    });
}

/// v0.6 alpha swarm, risks item 8 (Oracle full-build closure, 8th ACG/APG
/// class-specific closure): tests the spontaneous known-spell posture and
/// the Mystery/Curse choice dispatch, mirroring the established dispatch-
/// widening test module shape.
#[cfg(test)]
mod oracle_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, CharacterClassLevel, CharacterInput,
        HeadlessReceiptStatus, BATTLE_MYSTERY_SELECTION, BONE_MYSTERY_SELECTION,
        CLOUDED_VISION_CURSE_SELECTION, DEAF_CURSE_SELECTION, FIGHTER_CLASS_ID,
        FLAME_MYSTERY_SELECTION, HEAVENS_MYSTERY_SELECTION, LAME_CURSE_SELECTION,
        LIFE_MYSTERY_SELECTION, LORE_MYSTERY_SELECTION, NATURE_MYSTERY_SELECTION,
        ORACLE_BATTLECRY_REVELATION, ORACLE_CHANNEL_REVELATION, ORACLE_CINDER_DANCE_REVELATION,
        ORACLE_CLASS_ID, ORACLE_COAT_OF_MANY_STARS_REVELATION, ORACLE_CURSE_CHOICE_ID,
        ORACLE_ICY_SKIN_REVELATION, ORACLE_LORE_KEEPER_REVELATION, ORACLE_MYSTERY_CHOICE_ID,
        ORACLE_NATURES_WHISPERS_REVELATION, ORACLE_NEAR_DEATH_REVELATION,
        ORACLE_REVELATION_CHOICE_ID, ORACLE_SIDESTEP_SECRET_REVELATION,
        ORACLE_SPARK_SKIN_REVELATION, ORACLE_STEELBREAKER_SKIN_REVELATION, STONE_MYSTERY_SELECTION,
        WASTING_CURSE_SELECTION, WAVES_MYSTERY_SELECTION, WIND_MYSTERY_SELECTION,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_oracle_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: ORACLE_CLASS_ID.to_owned(), level }];
        input
    }

    /// A bare single-class Human Oracle (no known spells, no Mystery, no
    /// Curse) stays `Blocked` on all three claim-blocking diagnostics at
    /// once -- zero known spells is itself a valid posture (mirrors
    /// Sorcerer's own "zero known spells is always valid" reasoning), so
    /// only the Mystery/Curse/other_features_deferred diagnostics fire,
    /// never the retired generic one.
    #[test]
    fn single_class_oracle_bare_stays_blocked_on_mystery_curse_and_other_features() {
        let input = human_oracle_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Oracle must stay Blocked without a recognized Mystery/Curse choice: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.unsupported"),
            "the retired generic diagnostic must never appear for Oracle: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.oracle.known_spells.unsupported"),
            "zero known spells is itself a valid posture, mirroring Sorcerer's own reasoning: \
             {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.mystery_powers.unsupported"
                    && d.claim_blocking),
            "expected the mystery_powers claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.curse_powers.unsupported"
                    && d.claim_blocking),
            "expected the curse_powers claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Human Oracle with a real known spell (verified
    /// against the real Cleric spell list) grounds the known-spell
    /// posture for real and clears the known_spells diagnostic, but stays
    /// `Blocked` on Mystery/Curse/other_features_deferred regardless.
    #[test]
    fn single_class_oracle_with_a_real_known_spell_grounds_it_and_stays_blocked_elsewhere() {
        let mut input = human_oracle_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ORACLE_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.oracle.known_spells.unsupported"),
            "the known_spells diagnostic must not fire once a real known spell is recorded: {:?}",
            receipt.computation.diagnostics
        );
        let known = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.apg.oracle.known_spells")
            .expect("known-spell count must be grounded");
        assert_eq!(known.value, 1, "Oracle level 1 with one known spell: {:?}", known);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Oracle still stays Blocked on Mystery/Curse/other_features_deferred: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The v0.6 spellcasting widening must reach Oracle end to end: a
    /// level-20 Oracle knowing a real 9th-level Cleric spell is a legal
    /// posture (`apg_classes.lst:134` -- `20 KNOWN:9,5,5,4,4,4,3,3,3,3`
    /// grants 3 known spells at spell level 9), where the old bounded 1-3
    /// ceiling rejected the whole posture outright.
    ///
    /// Oracle still stays `Blocked` at every level on its
    /// Mystery/Curse/other-features diagnostics -- those are entirely
    /// separate from this ceiling and are deliberately untouched here.
    #[test]
    fn oracle_level20_admits_a_real_ninth_level_known_spell_after_the_widening() {
        let mut input = human_oracle_input(20);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Miracle".to_owned(),
            source_class_id: ORACLE_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.oracle.known_spells.unsupported"),
            "a 9th-level Cleric spell known at Oracle level 20 is a legal posture after the \
             widening: {:?}",
            receipt.computation.diagnostics
        );
        let known = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.apg.oracle.known_spells")
            .expect("known-spell count must be grounded at level 20");
        assert_eq!(known.value, 1, "Oracle level 20 with one known spell: {known:?}");
    }

    /// The widened Oracle table must still be a real per-spell-level cap,
    /// not an "anything goes above level 3" hole: `apg_classes.lst:118` --
    /// `4 KNOWN:6,3,1` grants exactly one 2nd-level known spell at Oracle
    /// level 4, so a second one is a genuine over-known violation.
    #[test]
    fn oracle_level4_still_enforces_the_real_per_spell_level_known_cap() {
        let mut input = human_oracle_input(4);
        for spell_id in ["Aid", "Align Weapon"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: ORACLE_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt.computation.diagnostics.iter().any(|d| d.id
                == "class_spell.apg.oracle.known_spells.unsupported"
                && d.message.contains("over-known")),
            "two 2nd-level known spells exceed Oracle level 4's single 2nd-level slot \
             (apg_classes.lst:118): {:?}",
            receipt.computation.diagnostics
        );
    }

    /// An Oracle known spell not on the real Cleric spell list (the
    /// shared portion of `SPELLLIST:2|Cleric|Oracle`) is a genuine
    /// posture violation and must claim-block via known_spells.
    #[test]
    fn single_class_oracle_with_an_off_list_known_spell_stays_blocked_on_known_spells() {
        let mut input = human_oracle_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Definitely Not A Real Spell".to_owned(),
            source_class_id: ORACLE_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.oracle.known_spells.unsupported"
                    && d.claim_blocking),
            "expected the known_spells claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// An Oracle recording more known spells at a level than the Oracle
    /// Spells Known table allows for that level is a genuine posture
    /// violation and must claim-block via known_spells over-known.
    #[test]
    fn single_class_oracle_over_known_at_a_spell_level_stays_blocked_on_known_spells() {
        let mut input = human_oracle_input(1);
        // Level 1 cantrip cap is 4; record a 5th distinct cantrip.
        for spell_id in ["Light", "Detect Magic", "Read Magic", "Resistance", "Stabilize"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: ORACLE_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.oracle.known_spells.unsupported"
                    && d.claim_blocking
                    && d.message.contains("over-known")),
            "expected the over-known known_spells claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Human Oracle with the Life Mystery recognized
    /// grounds Healing Hands' flat +4 Heal bonus and clears the
    /// mystery_powers diagnostic in favor of the non-blocking "other
    /// Mysteries" note -- stays `Blocked` on Curse/other_features_deferred
    /// regardless.
    #[test]
    fn single_class_oracle_with_life_mystery_grounds_healing_hands() {
        let mut input = human_oracle_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
            selection_id: LIFE_MYSTERY_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.mystery_powers.unsupported"),
            "mystery_powers must not fire once Life is recognized: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.apg.oracle.mystery_revelations_beyond_life.unmodeled"
                    && !d.claim_blocking),
            "expected the non-blocking other-Mysteries note: {:?}",
            receipt.computation.diagnostics
        );
        let healing_hands = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.life_mystery.healing_hands")
            .expect("Healing Hands must be grounded once Life is recognized");
        assert_eq!(healing_hands.value, 4, "Healing Hands is a flat +4 Heal bonus: {:?}", healing_hands);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Oracle still stays Blocked on Curse/other_features_deferred: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Human Oracle with the Clouded Vision Curse
    /// recognized grounds the flat 30-foot vision-range cap and clears
    /// the curse_powers diagnostic in favor of the non-blocking "other
    /// Curses" note.
    #[test]
    fn single_class_oracle_with_clouded_vision_curse_grounds_the_vision_range_cap() {
        let mut input = human_oracle_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_CURSE_CHOICE_ID.to_owned(),
            selection_id: CLOUDED_VISION_CURSE_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.curse_powers.unsupported"),
            "curse_powers must not fire once Clouded Vision is recognized: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.curses_beyond_grounded_set.unmodeled"
                    && !d.claim_blocking),
            "expected the non-blocking other-Curses note: {:?}",
            receipt.computation.diagnostics
        );
        let vision_cap = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.clouded_vision_curse.vision_range_cap")
            .expect("Clouded Vision's vision-range cap must be grounded once recognized");
        assert_eq!(vision_cap.value, 30, "Clouded Vision caps vision at 30 feet: {:?}", vision_cap);
    }

    /// **Replaces `single_class_oracle_with_everything_recognized_still_stays_permanently_blocked`
    /// (Path A canonical narrowing, 2026-07-29).** That test asserted
    /// Oracle could never reach `Computed` because "no MVP narrowing was
    /// found for Cure Wounds/Inflict Wounds/Tongues this slice". The
    /// premise was the mistake, not the slice: Oracle is a chooser-shaped
    /// class, and its MVP narrowing is the canonical Mystery/Curse pair,
    /// the identical shape Arcanist's Metamagic Knowledge and Cleric's
    /// Good domain already ship. With a valid known-spell posture, the
    /// Life Mystery (whose Healing Hands is genuinely grounded) and the
    /// Clouded Vision Curse (whose 30-foot cap is genuinely grounded) all
    /// recorded, everything this engine claims about this character IS
    /// computed, so the deferred remainder is named without blocking.
    ///
    /// The negative half -- bare Oracle, and Oracle with an ungrounded
    /// Mystery or Curse -- is unchanged and asserted by
    /// `single_class_oracle_bare_stays_blocked_on_mystery_curse_and_other_features`
    /// and `apg_canonical_choice_path_a_tests`.
    #[test]
    fn single_class_oracle_with_everything_recognized_reaches_computed() {
        let mut input = human_oracle_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ORACLE_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
            selection_id: LIFE_MYSTERY_SELECTION.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_CURSE_CHOICE_ID.to_owned(),
            selection_id: CLOUDED_VISION_CURSE_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "with a grounded Mystery AND a grounded Curse recorded, Oracle's remaining gaps are \
             named without blocking: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.other_features_deferred.unsupported"
                    && !d.claim_blocking),
            "the deferred remainder must still be NAMED, just not claim-blocking: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A non-Oracle character carrying spoofed Oracle choice/spell
    /// entries must have them silently ignored -- the class-ownership
    /// gate is by construction, not a bolt-on rejection. Also proves
    /// Fighter's own golden path is unaffected.
    #[test]
    fn non_oracle_characters_spoofed_oracle_entries_are_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
            selection_id: LIFE_MYSTERY_SELECTION.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_CURSE_CHOICE_ID.to_owned(),
            selection_id: CLOUDED_VISION_CURSE_SELECTION.to_owned(),
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ORACLE_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by stray Oracle entries: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.apg.oracle.")
                    || e.id.starts_with("class_spell.apg.oracle.")),
            "a non-Oracle character must never ground any Oracle explanation: {:?}",
            receipt.computation.explanations
        );
    }

    fn oracle_with_curse(level: u8, curse: &str) -> CharacterInput {
        let mut input = human_oracle_input(level);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_CURSE_CHOICE_ID.to_owned(),
            selection_id: curse.to_owned(),
        });
        input
    }

    /// `OracleCurseLVL = OracleLVL + ((TL - OracleLVL)/2)` collapses to
    /// the Oracle level for a single-class Oracle but is a genuine HALF
    /// progression for a multiclass one. Pinned directly so a future
    /// simplification to "just the Oracle level" cannot pass silently.
    #[test]
    fn oracle_curse_level_is_a_half_progression_for_the_non_oracle_half_of_total_level() {
        for level in 1..=20u8 {
            assert_eq!(
                super::oracle_curse_level(level, level),
                i16::from(level),
                "single-class Oracle level {level}: curse level equals the Oracle level"
            );
        }
        // A 4th-level Oracle / 6th-level something-else: 4 + (10-4)/2 = 7.
        assert_eq!(super::oracle_curse_level(4, 10), 7);
        // Odd remainder truncates: 4 + (9-4)/2 = 4 + 2 = 6.
        assert_eq!(super::oracle_curse_level(4, 9), 6);
        assert_eq!(super::oracle_curse_level(1, 20), 10);
    }

    /// Deaf's initiative penalty walks BACK toward zero as curse level
    /// rises (-4 / -2 / 0), because both `.MOD` records add `+2`. Pinned
    /// across the full range: a naive reading that treated the steps as
    /// deepening the penalty would produce -4/-6/-8 and pass any
    /// single-level spot-check at level 1.
    #[test]
    fn oracle_deaf_initiative_penalty_walks_back_toward_zero_at_every_curse_level() {
        for curse_level in 1..=4 {
            assert_eq!(super::oracle_deaf_initiative_penalty(curse_level), -4);
        }
        for curse_level in 5..=9 {
            assert_eq!(super::oracle_deaf_initiative_penalty(curse_level), -2);
        }
        for curse_level in 10..=20 {
            assert_eq!(
                super::oracle_deaf_initiative_penalty(curse_level),
                0,
                "the level-15 .MOD record's own DESC confirms no initiative penalty remains"
            );
        }
        for curse_level in 1..=4 {
            assert_eq!(super::oracle_deaf_perception_bonus(curse_level), None);
        }
        for curse_level in 5..=20 {
            assert_eq!(super::oracle_deaf_perception_bonus(curse_level), Some(3));
        }
    }

    /// The headline trap: Wasting's net effect on Intimidate is ZERO,
    /// not `+4`. Intimidate is a live computed total here, so this is the
    /// one place in the closure where a misreading would ship a wrong
    /// number.
    #[test]
    fn oracle_wasting_curse_nets_to_zero_on_intimidate_not_plus_four() {
        assert_eq!(
            super::oracle_wasting_intimidate_net_effect(),
            0,
            "BONUS:SKILL|STAT.CHA|-4 and BONUS:SKILL|Intimidate|4 cancel on Intimidate"
        );

        let receipt = build_pilot_headless_receipt(&oracle_with_curse(1, WASTING_CURSE_SELECTION));
        let net = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.wasting_curse.intimidate_net_effect")
            .expect("the net-effect record must be grounded explicitly");
        assert_eq!(net.value, 0, "{net:?}");
        let penalty = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.wasting_curse.charisma_skill_penalty")
            .expect("the Charisma-skill penalty must be grounded");
        assert_eq!(penalty.value, -4, "{penalty:?}");
    }

    /// Lame's magnitude is read from the character's own race speed, not
    /// assumed to be 30 feet. The fixture race is Human (30 ft), so the
    /// penalty is the larger 10-foot branch.
    #[test]
    fn oracle_lame_curse_derives_its_penalty_from_the_characters_real_base_land_speed() {
        assert_eq!(super::oracle_lame_speed_penalty(30), 10);
        assert_eq!(super::oracle_lame_speed_penalty(20), 5);
        assert_eq!(super::base_land_speed_feet("race:human"), Some(30));
        assert_eq!(super::base_land_speed_feet("race:dwarf"), Some(20));
        assert_eq!(super::base_land_speed_feet("race:halfling"), Some(20));
        assert_eq!(super::base_land_speed_feet("race:nonexistent"), None);

        let receipt = build_pilot_headless_receipt(&oracle_with_curse(1, LAME_CURSE_SELECTION));
        let penalty = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.lame_curse.base_land_speed_penalty")
            .expect("Lame's speed penalty must be grounded");
        assert_eq!(penalty.value, 10, "Human base speed 30 -> 10-foot branch: {penalty:?}");
        assert!(
            penalty.detail.contains("20 feet"),
            "the record must name the resulting speed, not just the reduction: {penalty:?}"
        );
    }

    /// Each of the three new curses clears the claim-blocking
    /// `curse_powers.unsupported` diagnostic on its own, exactly as
    /// Clouded Vision already does -- an Oracle picks one curse, so no
    /// combination is required.
    #[test]
    fn each_newly_grounded_curse_alone_clears_the_curse_powers_block() {
        for curse in [LAME_CURSE_SELECTION, WASTING_CURSE_SELECTION, DEAF_CURSE_SELECTION] {
            let receipt = build_pilot_headless_receipt(&oracle_with_curse(1, curse));
            assert!(
                !receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id == "class_feature.apg.oracle.curse_powers.unsupported"),
                "{curse} must clear the curse-powers block: {:?}",
                receipt.computation.diagnostics
            );
            assert!(
                receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id
                        == "class_feature.apg.oracle.curses_beyond_grounded_set.unmodeled"
                        && !d.claim_blocking),
                "{curse} must still carry the non-blocking remainder note: {:?}",
                receipt.computation.diagnostics
            );
            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Blocked,
                "a Curse alone is not enough -- with no Mystery recorded, Oracle stays Blocked \
                 on mystery_powers: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// End-to-end at a curse level past both Deaf tiers: level 10 gives
    /// initiative penalty 0 and the +3 Perception bonus simultaneously.
    #[test]
    fn level_10_deaf_oracle_grounds_zero_initiative_penalty_and_the_perception_bonus() {
        let receipt = build_pilot_headless_receipt(&oracle_with_curse(10, DEAF_CURSE_SELECTION));

        let initiative = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.deaf_curse.initiative_penalty")
            .expect("Deaf's initiative penalty must be grounded");
        assert_eq!(initiative.value, 0, "curse level 10 has walked the penalty to 0: {initiative:?}");

        let perception = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.deaf_curse.perception_bonus")
            .expect("Deaf's Perception bonus must be grounded");
        assert_eq!(perception.value, 3, "{perception:?}");
    }

    /// A level-1 Deaf Oracle is below the Perception tier, so that half
    /// is named as correctly absent rather than silently omitted, while
    /// the initiative penalty is at its full -4.
    #[test]
    fn level_1_deaf_oracle_names_the_perception_bonus_as_correctly_absent() {
        let receipt = build_pilot_headless_receipt(&oracle_with_curse(1, DEAF_CURSE_SELECTION));

        let initiative = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.deaf_curse.initiative_penalty")
            .expect("Deaf's initiative penalty must be grounded");
        assert_eq!(initiative.value, -4, "{initiative:?}");

        let perception = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.deaf_curse.perception_bonus")
            .expect("the Perception half must still be named below its tier");
        assert_eq!(perception.value, 0, "{perception:?}");
        assert!(
            perception.detail.contains("correctly absent"),
            "it must say why it is absent: {perception:?}"
        );
    }

    /// None of the three new curses may leak onto a non-Oracle, and none
    /// may disturb the computed Intimidate total (Wasting's tokens cancel,
    /// so nothing is wired into it at all).
    #[test]
    fn newly_grounded_curses_never_leak_onto_a_non_oracle_character() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 }];
        for curse in [LAME_CURSE_SELECTION, WASTING_CURSE_SELECTION, DEAF_CURSE_SELECTION] {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: ORACLE_CURSE_CHOICE_ID.to_owned(),
                selection_id: curse.to_owned(),
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.apg.oracle.")),
            "a Fighter must never ground an Oracle curse: {:?}",
            receipt.computation.explanations
        );
    }

    /// Builds an Oracle who took `mystery` and explicitly recorded
    /// `revelation`, with Charisma raised to 18 (+4) so the
    /// stat-substitution revelations produce a non-trivial delta -- the
    /// stock fixture's Charisma 8 (-1) makes every substitution a
    /// genuine but uninformative 0.
    fn oracle_with_revelation(level: u8, mystery: &str, revelation: &str) -> CharacterInput {
        let mut input = human_oracle_input(level);
        input.chosen.ability_scores.charisma = 18;
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
            selection_id: mystery.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_REVELATION_CHOICE_ID.to_owned(),
            selection_id: revelation.to_owned(),
        });
        input
    }

    /// The load-bearing property of the ratified option-(B) design: a
    /// Mystery pick ALONE grounds no revelation. Revelations are a
    /// budgeted selection (1 at level 1), so grounding them off the
    /// Mystery would describe an illegal character.
    #[test]
    fn a_mystery_pick_alone_grounds_no_tier_one_revelation() {
        for (mystery, id_fragment) in [
            (LORE_MYSTERY_SELECTION, "sidestep_secret"),
            (LORE_MYSTERY_SELECTION, "lore_keeper"),
            (NATURE_MYSTERY_SELECTION, "natures_whispers"),
            (BONE_MYSTERY_SELECTION, "near_death"),
            (FLAME_MYSTERY_SELECTION, "cinder_dance"),
            (LIFE_MYSTERY_SELECTION, "channel_uses_per_day"),
        ] {
            let mut input = human_oracle_input(1);
            input.chosen.ability_scores.charisma = 18;
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
                selection_id: mystery.to_owned(),
            });
            let receipt = build_pilot_headless_receipt(&input);
            assert!(
                !receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.contains(id_fragment)),
                "{mystery} alone must not ground {id_fragment} without an explicit revelation \
                 pick: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// A revelation pick without its own Mystery grounds nothing either --
    /// the gate is a genuine conjunction, not an either/or.
    #[test]
    fn a_revelation_pick_without_its_mystery_grounds_nothing() {
        let input = oracle_with_revelation(
            1,
            BONE_MYSTERY_SELECTION,
            ORACLE_SIDESTEP_SECRET_REVELATION,
        );
        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.contains("sidestep_secret")),
            "Sidestep Secret needs the LORE mystery, not merely any mystery: {:?}",
            receipt.computation.explanations
        );
    }

    /// The stat-substitution idiom shared by Sidestep Secret, Nature's
    /// Whispers, and Lore Keeper. Returned as a DELTA on top of the base
    /// modifier, so that a consumer which already added the base ends up
    /// at exactly `max(substitute, base)` with no double-count.
    #[test]
    fn oracle_stat_substitution_is_a_delta_that_never_double_counts() {
        for (substitute, base) in
            [(4, 2), (2, 4), (0, 0), (-1, 2), (5, -1), (3, 3), (-2, -4), (-4, -2)]
        {
            let delta = super::oracle_stat_substitution_delta(substitute, base);
            assert_eq!(
                base + delta,
                substitute.max(base),
                "base {base} + delta {delta} must equal max({substitute}, {base})"
            );
            assert!(delta >= 0, "the substitution never lowers a value: {delta}");
        }
    }

    /// Sidestep Secret is INTEGRATED into the real Reflex total, not
    /// merely reported. Proven by differencing the same character with
    /// and without the revelation, so the assertion cannot pass on a
    /// standalone record alone.
    #[test]
    fn sidestep_secret_actually_raises_the_real_reflex_save_total() {
        let reflex_total = |input: &CharacterInput| {
            build_pilot_headless_receipt(input)
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "defense.total_save.reflex")
                .map(|e| e.value)
        };

        let mut baseline = human_oracle_input(1);
        baseline.chosen.ability_scores.charisma = 18;
        let without = reflex_total(&baseline).expect("Reflex total must be computed for an Oracle");

        let with = reflex_total(&oracle_with_revelation(
            1,
            LORE_MYSTERY_SELECTION,
            ORACLE_SIDESTEP_SECRET_REVELATION,
        ))
        .expect("Reflex total must still be computed");

        // Fixture DEX 14 (+2), Charisma raised to 18 (+4): delta = 4-2 = 2.
        assert_eq!(
            with - without,
            2,
            "Sidestep Secret must raise the REAL Reflex total by max(CHA,DEX)-DEX = 2"
        );
    }

    /// Nature's Whispers is likewise integrated into the real Armor Class
    /// total, differenced the same way.
    #[test]
    fn natures_whispers_actually_raises_the_real_armor_class_total() {
        let armor_class = |input: &CharacterInput| {
            build_pilot_headless_receipt(input)
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "defense.baseline_armor_class")
                .map(|e| e.value)
        };

        let mut baseline = human_oracle_input(1);
        baseline.chosen.ability_scores.charisma = 18;
        let without = armor_class(&baseline).expect("AC must be computed for an Oracle");

        let with = armor_class(&oracle_with_revelation(
            1,
            NATURE_MYSTERY_SELECTION,
            ORACLE_NATURES_WHISPERS_REVELATION,
        ))
        .expect("AC must still be computed");

        assert_eq!(
            with - without,
            2,
            "Nature's Whispers must raise the REAL armor class by max(DEX,CHA)-DEX = 2"
        );
    }

    /// Neither integrated revelation may move a total for a character who
    /// did not take it -- the gate is what keeps every other class's
    /// Reflex and AC totals untouched.
    #[test]
    fn integrated_revelations_never_move_totals_for_characters_without_them() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut fighter = result.character_input.expect("valid fixture");
        fighter.chosen.ability_scores.charisma = 18;
        let clean = build_pilot_headless_receipt(&fighter);
        let clean_reflex = clean
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.total_save.reflex")
            .map(|e| e.value);
        let clean_ac = clean
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .map(|e| e.value);

        // Same Fighter, now carrying spoofed Oracle mystery + revelation picks.
        for (choice_set, selection) in [
            (ORACLE_MYSTERY_CHOICE_ID, LORE_MYSTERY_SELECTION),
            (ORACLE_REVELATION_CHOICE_ID, ORACLE_SIDESTEP_SECRET_REVELATION),
            (ORACLE_MYSTERY_CHOICE_ID, NATURE_MYSTERY_SELECTION),
            (ORACLE_REVELATION_CHOICE_ID, ORACLE_NATURES_WHISPERS_REVELATION),
        ] {
            fighter.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: choice_set.to_owned(),
                selection_id: selection.to_owned(),
            });
        }
        let spoofed = build_pilot_headless_receipt(&fighter);

        assert_eq!(
            spoofed
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "defense.total_save.reflex")
                .map(|e| e.value),
            clean_reflex,
            "a Fighter's Reflex total must be untouched by spoofed Oracle revelation picks"
        );
        assert_eq!(
            spoofed
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "defense.baseline_armor_class")
                .map(|e| e.value),
            clean_ac,
            "a Fighter's armor class must be untouched by spoofed Oracle revelation picks"
        );
    }

    /// Life Mystery's Channel: all three magnitudes at once. Charisma 18
    /// (+4) at Oracle level 3 gives uses 1+4=5, dice (3+1)/2=2, DC
    /// 10+1+4=15.
    #[test]
    fn life_mystery_channel_grounds_uses_dice_and_dc() {
        let receipt = build_pilot_headless_receipt(&oracle_with_revelation(
            3,
            LIFE_MYSTERY_SELECTION,
            ORACLE_CHANNEL_REVELATION,
        ));
        for (id, expected) in [
            ("class_feature.apg.oracle.life_mystery.channel_uses_per_day", 5),
            ("class_feature.apg.oracle.life_mystery.channel_dice", 2),
            ("class_feature.apg.oracle.life_mystery.channel_dc", 15),
        ] {
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("{id} must ground"));
            assert_eq!(record.value, expected, "{id}: {record:?}");
        }
    }

    /// `Improved Channel` raises this DC by 2 -- and it is the ONLY
    /// channel DC in this engine the feat can reach.
    ///
    /// The feat is a Core Rulebook record naming five variables
    /// (`ClericChannelPositiveEnergyDC`, `PaladinChannelPositiveEnergyDC`,
    /// `ClericChannelNegativeEnergyDC`, `PowerOverUndeadCommandDC`,
    /// `PowerOverUndeadTurnDC`), every one of which this engine computes
    /// nothing for: Cleric grounds `channel_energy_dice` and
    /// `channel_energy_uses_per_day` but no DC, and Paladin grounds
    /// `channel_positive_energy_dice` but no DC. Read against
    /// `cr_feats.lst` alone the feat is therefore a pure no-op here.
    /// `apg_feats.lst`'s own `CATEGORY=FEAT|Improved Channel.MOD` adds
    /// `BONUS:VAR|OracleChannelDC|2`, and `OracleChannelDC` IS computed
    /// (`oracle_channel_dc`) -- the same `.MOD`-in-a-different-book shape
    /// that hid Extra Performance's Skald half.
    ///
    /// Magnitude `2` in both the token and the prose ("Add 2 to the DC of
    /// saving throws made to resist the effects of your channel energy
    /// ability"). No `STACK:`/`MULT:` and no repeat clause, so it is
    /// presence-based, not counted.
    ///
    /// Charisma 18 (+4) at Oracle level 3: base DC 10 + 1 + 4 = 15,
    /// raised to 17.
    #[test]
    fn improved_channel_raises_the_real_oracle_life_mystery_channel_dc_by_two() {
        let base = oracle_with_revelation(3, LIFE_MYSTERY_SELECTION, ORACLE_CHANNEL_REVELATION);
        let dc_of = |input: &CharacterInput| {
            build_pilot_headless_receipt(input)
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.apg.oracle.life_mystery.channel_dc")
                .map(|e| e.value)
        };
        assert_eq!(dc_of(&base), Some(15), "base Life Mystery channel DC");

        let mut with_feat = base.clone();
        with_feat.chosen.selected_feats.push("Improved Channel".to_owned());
        assert_eq!(dc_of(&with_feat), Some(17), "Improved Channel must add exactly 2");

        // Not MULT:YES: a second copy must add nothing more.
        let mut twice = with_feat.clone();
        twice.chosen.selected_feats.push("Improved Channel".to_owned());
        assert_eq!(dc_of(&twice), Some(17), "Improved Channel is not repeatable");
    }

    /// The negative half of the same triage. Improved Channel's five
    /// Core-Rulebook variables name Cleric and Paladin channel DCs this
    /// engine does not compute; the feat must not silently move the
    /// channel totals it DOES compute (dice and uses per day), for
    /// either class or for the Oracle.
    #[test]
    fn improved_channel_moves_no_channel_total_other_than_the_oracle_dc() {
        let base = oracle_with_revelation(3, LIFE_MYSTERY_SELECTION, ORACLE_CHANNEL_REVELATION);
        let mut with_feat = base.clone();
        with_feat.chosen.selected_feats.push("Improved Channel".to_owned());

        let value_of = |input: &CharacterInput, id: &str| {
            build_pilot_headless_receipt(input)
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .map(|e| e.value)
        };
        for id in [
            "class_feature.apg.oracle.life_mystery.channel_uses_per_day",
            "class_feature.apg.oracle.life_mystery.channel_dice",
        ] {
            assert_eq!(
                value_of(&with_feat, id),
                value_of(&base, id),
                "Improved Channel raises only the DC, not {id}"
            );
        }
    }

    /// Channel's dice and DC across the full level range, derived from
    /// the corpus formulas rather than spot-checked at one level.
    #[test]
    fn oracle_channel_dice_and_dc_match_the_corpus_at_every_level() {
        for (level, expected_dice) in
            [(1, 1), (2, 1), (3, 2), (4, 2), (5, 3), (10, 5), (19, 10), (20, 10)]
        {
            assert_eq!(
                super::oracle_channel_dice(level),
                expected_dice,
                "level {level} Channel dice: (level+1)/2"
            );
        }
        for (level, charisma, expected_dc) in
            [(1, 0, 10), (1, 4, 14), (2, 4, 15), (20, 4, 24), (20, -1, 19)]
        {
            assert_eq!(super::oracle_channel_dc(level, charisma, &[]), expected_dc);
        }
        assert_eq!(super::oracle_channel_uses_per_day(4), 5);
        assert_eq!(super::oracle_channel_uses_per_day(-1), 0);
    }

    /// Near Death steps from +2 to +4 at Oracle level 11 exactly, because
    /// the corpus stacks a second `+2` behind `PRECLASS:1,Oracle=11`.
    #[test]
    fn near_death_save_bonus_steps_up_at_oracle_level_eleven() {
        for level in 1..=10u8 {
            assert_eq!(super::oracle_near_death_save_bonus(level), 2, "level {level}");
        }
        for level in 11..=20u8 {
            assert_eq!(super::oracle_near_death_save_bonus(level), 4, "level {level}");
        }

        let receipt = build_pilot_headless_receipt(&oracle_with_revelation(
            11,
            BONE_MYSTERY_SELECTION,
            ORACLE_NEAR_DEATH_REVELATION,
        ));
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.bone_mystery.near_death_save_bonus")
            .expect("Near Death must ground");
        assert_eq!(record.value, 4, "{record:?}");
    }

    /// Near Death must NOT be folded into the flat save totals: it only
    /// applies against disease, mind-affecting effects, and poison, and
    /// this engine models no per-category save facet.
    #[test]
    fn near_death_does_not_inflate_the_flat_save_totals() {
        let saves = |input: &CharacterInput| {
            let receipt = build_pilot_headless_receipt(input);
            ["fortitude", "reflex", "will"]
                .iter()
                .filter_map(|s| {
                    receipt
                        .computation
                        .explanations
                        .iter()
                        .find(|e| e.id == format!("defense.total_save.{s}"))
                        .map(|e| e.value)
                })
                .collect::<Vec<_>>()
        };
        let mut baseline = human_oracle_input(11);
        baseline.chosen.ability_scores.charisma = 18;

        assert_eq!(
            saves(&oracle_with_revelation(
                11,
                BONE_MYSTERY_SELECTION,
                ORACLE_NEAR_DEATH_REVELATION
            )),
            saves(&baseline),
            "Near Death is category-restricted and must not move any flat save total"
        );
    }

    /// Lore Keeper and Cinder Dance, the two remaining standalone
    /// revelations.
    #[test]
    fn lore_keeper_and_cinder_dance_ground_their_real_magnitudes() {
        let lore = build_pilot_headless_receipt(&oracle_with_revelation(
            1,
            LORE_MYSTERY_SELECTION,
            ORACLE_LORE_KEEPER_REVELATION,
        ));
        let keeper = lore
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.lore_mystery.lore_keeper_knowledge_bonus")
            .expect("Lore Keeper must ground");
        // Fixture INT 10 (+0), Charisma raised to 18 (+4): delta = 4-0 = 4.
        assert_eq!(keeper.value, 4, "max(CHA,INT)-INT = 4: {keeper:?}");

        let flame = build_pilot_headless_receipt(&oracle_with_revelation(
            1,
            FLAME_MYSTERY_SELECTION,
            ORACLE_CINDER_DANCE_REVELATION,
        ));
        let cinder = flame
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.flame_mystery.cinder_dance_speed_bonus")
            .expect("Cinder Dance must ground");
        assert_eq!(cinder.value, 10, "flat +10 ft base land speed: {cinder:?}");
    }

    /// SD31-E4-F2-001: Battle Mystery's Battlecry, the first revelation
    /// grounded through `archetype_resolver::chooser_option_selected`
    /// rather than the shared `oracle_level_with_revelation` helper.
    /// Charisma raised to 18 (fixture default via `oracle_with_revelation`)
    /// -- ability MODIFIER floor((18-10)/2) = 4 -- so the duration
    /// magnitude (`BONUS:VAR|OracleBattlecryDuration|CHA`, PCGen's bare
    /// `CHA` = the modifier, not `CHASCORE`) is non-trivial and
    /// distinguishable from the score. At Oracle level 1 (below the
    /// level-10 upgrade): bonus 1, duration 4, uses per day 1 + 1/5 = 1.
    #[test]
    fn battlecry_grounds_its_bonus_duration_and_uses_per_day_at_level_one() {
        let receipt = build_pilot_headless_receipt(&oracle_with_revelation(
            1,
            BATTLE_MYSTERY_SELECTION,
            ORACLE_BATTLECRY_REVELATION,
        ));
        for (id, expected) in [
            ("class_feature.apg.oracle.battle_mystery.battlecry_bonus", 1),
            ("class_feature.apg.oracle.battle_mystery.battlecry_duration_rounds", 4),
            ("class_feature.apg.oracle.battle_mystery.battlecry_uses_per_day", 1),
        ] {
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("{id} must ground: {:?}", receipt.computation.diagnostics));
            assert_eq!(record.value, expected, "{id}: {record:?}");
        }
    }

    /// SD31-W10-INTEGRATE-001 fix for a CONFIRMED fabricated-magnitude
    /// finding: the original grounding read the raw Charisma SCORE where
    /// the corpus token means the MODIFIER, and the only prior test used
    /// the shared fixture's fixed Charisma 18, under which score and a
    /// stale-modifier bug could both plausibly produce a passing number.
    /// This test picks a DIFFERENT Charisma (14 -> modifier +2, not the
    /// fixture's 18 -> +4) and a different level, so the duration cannot
    /// be satisfied by any constant or by reading the score.
    #[test]
    fn battlecry_duration_tracks_charisma_modifier_not_score_at_a_different_charisma() {
        let mut input = human_oracle_input(10);
        input.chosen.ability_scores.charisma = 14;
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
            selection_id: BATTLE_MYSTERY_SELECTION.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_REVELATION_CHOICE_ID.to_owned(),
            selection_id: ORACLE_BATTLECRY_REVELATION.to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        let duration = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.battle_mystery.battlecry_duration_rounds")
            .expect("Battlecry duration must ground");
        assert_eq!(
            duration.value, 2,
            "Charisma 14 -> modifier +2 (floor((14-10)/2)); NOT the score 14: {duration:?}"
        );
    }

    /// The bonus steps from +1 to +2 at Oracle level 10
    /// (`PRECLASS:1,Oracle=10`); uses per day steps with level too
    /// (1 + 10/5 = 3).
    #[test]
    fn battlecry_bonus_and_uses_per_day_step_up_at_oracle_level_ten() {
        let receipt = build_pilot_headless_receipt(&oracle_with_revelation(
            10,
            BATTLE_MYSTERY_SELECTION,
            ORACLE_BATTLECRY_REVELATION,
        ));
        let bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.battle_mystery.battlecry_bonus")
            .expect("Battlecry bonus must ground");
        assert_eq!(bonus.value, 2, "upgraded +2 at Oracle level 10: {bonus:?}");
        let times = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.battle_mystery.battlecry_uses_per_day")
            .expect("Battlecry uses per day must ground");
        assert_eq!(times.value, 3, "1 + 10/5 = 3: {times:?}");
    }

    /// A Mystery pick alone (no revelation recorded) grounds nothing for
    /// Battle either -- the same budgeted-revelation shape every other
    /// non-Life Mystery already enforces, now proven for the primitive-
    /// composed path too.
    #[test]
    fn a_battle_mystery_pick_alone_grounds_no_battlecry() {
        let mut input = human_oracle_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
            selection_id: BATTLE_MYSTERY_SELECTION.to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.apg.oracle.battle_mystery.")),
            "a bare Mystery pick must ground no revelation: {:?}",
            receipt.computation.explanations
        );
    }

    /// A revelation name from a DIFFERENT Mystery's own real pool must
    /// never ground Battlecry -- proves `ORACLE_BATTLE_MYSTERY_REVELATION_POOL`
    /// genuinely scopes the primitive's guard to Battle Mystery alone, not
    /// "any revelation anywhere" (`decisions.md §10`'s "a shared NAME is
    /// not a duplicate" discipline, applied to the chooser primitive).
    #[test]
    fn a_revelation_from_a_different_mysterys_pool_does_not_ground_battlecry() {
        let receipt = build_pilot_headless_receipt(&oracle_with_revelation(
            1,
            BATTLE_MYSTERY_SELECTION,
            ORACLE_LORE_KEEPER_REVELATION,
        ));
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.apg.oracle.battle_mystery.")),
            "a cross-Mystery revelation id must never ground Battlecry: {:?}",
            receipt.computation.explanations
        );
    }

    /// A Fighter carrying spoofed Battle Mystery + Battlecry entries must
    /// never ground anything -- the same class-ownership gate every other
    /// Oracle grounding proves, now proven for the primitive-composed path.
    #[test]
    fn a_non_oracle_never_grounds_battlecry() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut fighter = result.character_input.expect("valid fixture");
        fighter.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
            selection_id: BATTLE_MYSTERY_SELECTION.to_owned(),
        });
        fighter.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_REVELATION_CHOICE_ID.to_owned(),
            selection_id: ORACLE_BATTLECRY_REVELATION.to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&fighter);
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.apg.oracle.")),
            "a Fighter must never ground an Oracle revelation via the primitive: {:?}",
            receipt.computation.explanations
        );
    }

    /// **The full-character reachability proof.** A Battle-Mystery Oracle
    /// with a grounded Curse recorded reaches `HeadlessReceiptStatus::Computed`
    /// through `build_pilot_headless_receipt` exactly the same way
    /// `single_class_oracle_with_everything_recognized_reaches_computed`
    /// already proves for Life Mystery -- proving the new primitive's first
    /// consumer is genuinely reachable end to end, not merely unit-tested
    /// in isolation (this card's own "reachability is proven through a
    /// headless pilot receipt, never a resolver unit test alone" bar).
    #[test]
    fn single_class_oracle_with_battle_mystery_and_a_curse_reaches_computed() {
        let mut input = human_oracle_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
            selection_id: BATTLE_MYSTERY_SELECTION.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_REVELATION_CHOICE_ID.to_owned(),
            selection_id: ORACLE_BATTLECRY_REVELATION.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_CURSE_CHOICE_ID.to_owned(),
            selection_id: CLOUDED_VISION_CURSE_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Battle-Mystery Oracle with a grounded Curse must reach Computed via the new \
             chooser_option_selected-composed path, exactly as Life Mystery already does: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.apg.oracle.battle_mystery.battlecry_bonus"),
            "the Computed receipt must actually carry the grounded Battlecry bonus: {:?}",
            receipt.computation.explanations
        );
    }

    /// SD31-E4-F2-002: Stone Mystery's Steelbreaker Skin, wired through
    /// the same `chooser_option_selected` composition as Battle Mystery's
    /// Battlecry. At Oracle level 5, damage and duration both equal the
    /// class level (5).
    #[test]
    fn steelbreaker_skin_grounds_damage_and_duration_at_class_level() {
        let receipt = build_pilot_headless_receipt(&oracle_with_revelation(
            5,
            STONE_MYSTERY_SELECTION,
            ORACLE_STEELBREAKER_SKIN_REVELATION,
        ));
        for (id, expected) in [
            ("class_feature.apg.oracle.stone_mystery.steelbreaker_skin_damage", 5),
            (
                "class_feature.apg.oracle.stone_mystery.steelbreaker_skin_duration_minutes",
                5,
            ),
        ] {
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("{id} must ground: {:?}", receipt.computation.diagnostics));
            assert_eq!(record.value, expected, "{id}: {record:?}");
        }
    }

    /// SD31-E4-F2-002: Waves Mystery's Icy Skin. Below Oracle level 5 the
    /// resistance is the flat base (5); at level 5-10 it steps to 10; at
    /// level 11+ it steps again (additive) to 20.
    #[test]
    fn icy_skin_grounds_cold_resistance_and_steps_with_level() {
        let below = build_pilot_headless_receipt(&oracle_with_revelation(
            1,
            WAVES_MYSTERY_SELECTION,
            ORACLE_ICY_SKIN_REVELATION,
        ));
        let tier_two = build_pilot_headless_receipt(&oracle_with_revelation(
            5,
            WAVES_MYSTERY_SELECTION,
            ORACLE_ICY_SKIN_REVELATION,
        ));
        let tier_three = build_pilot_headless_receipt(&oracle_with_revelation(
            11,
            WAVES_MYSTERY_SELECTION,
            ORACLE_ICY_SKIN_REVELATION,
        ));
        for (receipt, expected) in [(below, 5), (tier_two, 10), (tier_three, 20)] {
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.apg.oracle.waves_mystery.icy_skin_cold_resistance")
                .unwrap_or_else(|| panic!("Icy Skin must ground: {:?}", receipt.computation.diagnostics));
            assert_eq!(record.value, expected, "{record:?}");
        }
    }

    /// SD31-E4-F2-002: Wind Mystery's Spark Skin -- the identical formula
    /// shape as Icy Skin (electricity instead of cold), proven at the
    /// same tier-two level.
    #[test]
    fn spark_skin_grounds_electricity_resistance() {
        let receipt = build_pilot_headless_receipt(&oracle_with_revelation(
            5,
            WIND_MYSTERY_SELECTION,
            ORACLE_SPARK_SKIN_REVELATION,
        ));
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.wind_mystery.spark_skin_electricity_resistance")
            .unwrap_or_else(|| panic!("Spark Skin must ground: {:?}", receipt.computation.diagnostics));
        assert_eq!(record.value, 10, "Oracle level 5 -> tier-two resistance 10: {record:?}");
    }

    /// SD31-E4-F2-002: Heavens Mystery's Coat of Many Stars. At Oracle
    /// level 1 the AC bonus is the flat base (+4, since
    /// `max(0,floor((1-3)/4))` clamps to 0) and duration is 1 hour/day;
    /// at level 7 the AC bonus steps to +6 (`floor((7-3)/4)=1`).
    #[test]
    fn coat_of_many_stars_grounds_ac_bonus_and_duration_and_steps_with_level() {
        let level_one = build_pilot_headless_receipt(&oracle_with_revelation(
            1,
            HEAVENS_MYSTERY_SELECTION,
            ORACLE_COAT_OF_MANY_STARS_REVELATION,
        ));
        let ac_one = level_one
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.heavens_mystery.coat_of_many_stars_ac_bonus")
            .expect("Coat of Many Stars AC bonus must ground");
        assert_eq!(ac_one.value, 4, "level 1 -> base +4: {ac_one:?}");
        let level_seven = build_pilot_headless_receipt(&oracle_with_revelation(
            7,
            HEAVENS_MYSTERY_SELECTION,
            ORACLE_COAT_OF_MANY_STARS_REVELATION,
        ));
        let ac_seven = level_seven
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.oracle.heavens_mystery.coat_of_many_stars_ac_bonus")
            .expect("Coat of Many Stars AC bonus must ground");
        assert_eq!(ac_seven.value, 6, "level 7 -> +6 (floor((7-3)/4)=1): {ac_seven:?}");
    }

    /// A Mystery pick alone (no revelation recorded) grounds nothing for
    /// any of the 4 new Mysteries -- same budgeted-revelation shape every
    /// other non-Life Mystery enforces.
    #[test]
    fn a_new_mystery_pick_alone_grounds_no_revelation() {
        for mystery in [
            STONE_MYSTERY_SELECTION,
            WAVES_MYSTERY_SELECTION,
            WIND_MYSTERY_SELECTION,
            HEAVENS_MYSTERY_SELECTION,
        ] {
            let mut input = human_oracle_input(1);
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
                selection_id: mystery.to_owned(),
            });
            let receipt = build_pilot_headless_receipt(&input);
            assert!(
                !receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.contains("steelbreaker_skin")
                        || e.id.contains("icy_skin")
                        || e.id.contains("spark_skin")
                        || e.id.contains("coat_of_many_stars")),
                "a bare {mystery} pick must ground no revelation: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// A revelation name from a DIFFERENT Mystery's own pool must never
    /// ground Steelbreaker Skin -- proves
    /// `ORACLE_STONE_MYSTERY_REVELATION_POOL` scopes the primitive's guard
    /// to Stone Mystery alone.
    #[test]
    fn a_revelation_from_a_different_mysterys_pool_does_not_ground_steelbreaker_skin() {
        let receipt = build_pilot_headless_receipt(&oracle_with_revelation(
            5,
            STONE_MYSTERY_SELECTION,
            ORACLE_ICY_SKIN_REVELATION,
        ));
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.contains("steelbreaker_skin")),
            "a cross-Mystery revelation id must never ground Steelbreaker Skin: {:?}",
            receipt.computation.explanations
        );
    }

    /// **Full-character reachability, all 4 new Mysteries.** Each reaches
    /// `HeadlessReceiptStatus::Computed` through `build_pilot_headless_receipt`
    /// with a grounded Curse recorded, exactly as Battle Mystery already
    /// proves -- this card's own "reachability is proven through a
    /// headless pilot receipt, never a resolver unit test" bar, applied to
    /// all 4 remaining pools named in the mandate.
    #[test]
    fn each_new_mystery_reaches_computed_with_a_curse() {
        for (mystery, revelation, explanation_id) in [
            (
                STONE_MYSTERY_SELECTION,
                ORACLE_STEELBREAKER_SKIN_REVELATION,
                "class_feature.apg.oracle.stone_mystery.steelbreaker_skin_damage",
            ),
            (
                WAVES_MYSTERY_SELECTION,
                ORACLE_ICY_SKIN_REVELATION,
                "class_feature.apg.oracle.waves_mystery.icy_skin_cold_resistance",
            ),
            (
                WIND_MYSTERY_SELECTION,
                ORACLE_SPARK_SKIN_REVELATION,
                "class_feature.apg.oracle.wind_mystery.spark_skin_electricity_resistance",
            ),
            (
                HEAVENS_MYSTERY_SELECTION,
                ORACLE_COAT_OF_MANY_STARS_REVELATION,
                "class_feature.apg.oracle.heavens_mystery.coat_of_many_stars_ac_bonus",
            ),
        ] {
            let mut input = human_oracle_input(5);
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: ORACLE_MYSTERY_CHOICE_ID.to_owned(),
                selection_id: mystery.to_owned(),
            });
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: ORACLE_REVELATION_CHOICE_ID.to_owned(),
                selection_id: revelation.to_owned(),
            });
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: ORACLE_CURSE_CHOICE_ID.to_owned(),
                selection_id: CLOUDED_VISION_CURSE_SELECTION.to_owned(),
            });
            let receipt = build_pilot_headless_receipt(&input);
            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Computed,
                "{mystery} Oracle with a grounded Curse must reach Computed: {:?}",
                receipt.computation.diagnostics
            );
            assert!(
                receipt.computation.explanations.iter().any(|e| e.id == explanation_id),
                "the Computed receipt must carry {explanation_id}: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// Cinder Dance and the Lame Curse are mutually exclusive in the
    /// corpus. Holding both is an illegal character, so it is named as a
    /// claim-blocking diagnostic rather than silently netted to +0 feet.
    #[test]
    fn cinder_dance_with_the_lame_curse_is_named_as_mutually_exclusive() {
        let mut input = oracle_with_revelation(
            1,
            FLAME_MYSTERY_SELECTION,
            ORACLE_CINDER_DANCE_REVELATION,
        );
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ORACLE_CURSE_CHOICE_ID.to_owned(),
            selection_id: LAME_CURSE_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.apg.oracle.cinder_dance_lame_mutually_exclusive"
                    && d.claim_blocking),
            "the illegal combination must be named: {:?}",
            receipt.computation.diagnostics
        );
        // Both magnitudes stay as their own records; neither is netted away.
        assert_eq!(
            receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id
                    == "class_feature.apg.oracle.flame_mystery.cinder_dance_speed_bonus")
                .map(|e| e.value),
            Some(10)
        );
        assert_eq!(
            receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.apg.oracle.lame_curse.base_land_speed_penalty")
                .map(|e| e.value),
            Some(10)
        );
    }

    /// Deaf's opposed-Perception penalty grounds at every curse level per
    /// the lead's explicit ruling (2026-07-27): a flat modifier on your
    /// own roll clears the bar even when the check is opposed.
    #[test]
    fn deaf_curse_grounds_the_opposed_perception_penalty_at_every_level() {
        for level in [1u8, 5, 10, 15] {
            let receipt =
                build_pilot_headless_receipt(&oracle_with_curse(level, DEAF_CURSE_SELECTION));
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| {
                    e.id == "class_feature.apg.oracle.deaf_curse.opposed_perception_penalty"
                })
                .unwrap_or_else(|| panic!("opposed-Perception penalty must ground at level {level}"));
            assert_eq!(record.value, -4, "flat at every curse level: {record:?}");
        }
    }

    /// Grounding the whole Tier-1 revelation set does not by itself move
    /// Oracle to `Computed`: this input records four Mysteries and five
    /// revelations but NO Curse, and a PF1 oracle always has one, so the
    /// curse-powers burden is genuinely unmet.
    ///
    /// This test used to be named `..._stays_permanently_blocked...` and
    /// its comment claimed the closure "does not move Oracle toward
    /// Computed" as a matter of design. Path A canonical narrowing
    /// (2026-07-29) retired that claim -- adding a grounded Curse to this
    /// same input does reach `Computed`, per
    /// `single_class_oracle_with_everything_recognized_reaches_computed`.
    /// What this test actually proves, and still proves, is the narrower
    /// and more useful thing: revelations alone are not a substitute for
    /// the Curse half of the pair.
    #[test]
    fn oracle_stays_blocked_on_its_curse_burden_with_every_tier_one_revelation_recognized() {
        let mut input = human_oracle_input(11);
        input.chosen.ability_scores.charisma = 18;
        for (choice_set, selection) in [
            (ORACLE_MYSTERY_CHOICE_ID, LIFE_MYSTERY_SELECTION),
            (ORACLE_MYSTERY_CHOICE_ID, LORE_MYSTERY_SELECTION),
            (ORACLE_MYSTERY_CHOICE_ID, NATURE_MYSTERY_SELECTION),
            (ORACLE_MYSTERY_CHOICE_ID, BONE_MYSTERY_SELECTION),
            (ORACLE_REVELATION_CHOICE_ID, ORACLE_CHANNEL_REVELATION),
            (ORACLE_REVELATION_CHOICE_ID, ORACLE_SIDESTEP_SECRET_REVELATION),
            (ORACLE_REVELATION_CHOICE_ID, ORACLE_NATURES_WHISPERS_REVELATION),
            (ORACLE_REVELATION_CHOICE_ID, ORACLE_LORE_KEEPER_REVELATION),
            (ORACLE_REVELATION_CHOICE_ID, ORACLE_NEAR_DEATH_REVELATION),
        ] {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: choice_set.to_owned(),
                selection_id: selection.to_owned(),
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "revelations alone do not satisfy the Curse half of the pair: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.curse_powers.unsupported"
                    && d.claim_blocking),
            "the unmet burden must be named as the Curse one specifically: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.apg.oracle.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "the deferred diagnostic must still claim-block without a Curse: {:?}",
            receipt.computation.diagnostics
        );
    }
}

