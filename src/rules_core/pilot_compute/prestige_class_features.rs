#[allow(unused_imports)]
pub(crate) use super::*;

/// SD-34 wave 47 (`decisions.md §22`'s WAVE 47 UPDATE): same "registered
/// prestige class, no `ClassId`-family enum entry, no chassis dispatch
/// reaches it" gap as the six wave-46 classes above. Matches `tests/
/// fixtures/rules_core/prestige-class-entry-requirements.json`'s own entry
/// exactly (`class:divine_scion`, source book `inner_sea_magic`).
pub(super) const DIVINE_SCION_CLASS_ID: &str = "class:divine_scion";

/// SD-34 wave 47 CORRECTION (`decisions.md §22`): the real oracle
/// (`ism_classes.lst:103`, `BONUS:ABILITYPOOL|Opposition Alignment|1`) and
/// `ism_abilities_class.lst:35`'s own `# Opposition Alignment choices`
/// section header both confirm this is a genuine one-of-four
/// mutually-exclusive `ABILITYPOOL` selection -- the SAME "real pool
/// selection this engine must gate on, not assume" shape
/// `SORCERER_BLOODLINE_CHOICE_ID` / `CLERIC_DOMAIN_CHOICE_ID` already
/// establish, not the "single-owner, unconditional" shape this wave's own
/// first draft wrongly applied to all four alignment DR records at once.
pub(super) const DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID: &str =
    "choice:divine_scion_opposition_alignment";
/// SD-34 wave 47 CORRECTION (`decisions.md §22`): the real oracle
/// (`ism_classes.lst:104`, `BONUS:ABILITYPOOL|Domain Specialization|1`) and
/// `ism_abilities_class.lst:47`'s own `# Domain Specialization choices`
/// section header both confirm this is a genuine one-of-35
/// mutually-exclusive `ABILITYPOOL` selection -- same correction and same
/// precedent as `DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID` immediately
/// above.
pub(super) const DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID: &str =
    "choice:divine_scion_domain_specialization";
/// SD-34 wave 48 (`decisions.md §22`'s WAVE 48 UPDATE): Twilight Talon and
/// Golden Legionnaire's own class ids, needed by `ground_twilight_talon_
/// class_features` / `ground_golden_legionnaire_class_features` below --
/// the same "real prestige class, registered in `prestige_class_entry_gate`
/// (source book `adventurers_guide`), no `ClassId`-family enum entry, no
/// chassis dispatch reaches it" gap as the wave-46/47 classes above.
/// Matches `tests/fixtures/rules_core/prestige-class-entry-requirements.
/// json`'s own entries exactly.
pub(super) const TWILIGHT_TALON_CLASS_ID: &str = "class:twilight_talon";

/// PF1 Pathfinder Society's Adventurer's Guide Pathfinder Delver's
/// Guardbreaker feature (`ag_abilities_class.lst:382`, `KEY:Pathfinder
/// Delver ~ Guardbreaker`): `BONUS:VAR|FavoredConstruct,FavoredOoze,
/// FavoredUndead|TrapSenseBonus`, granted from class level 3.
///
/// **Real audit correction, resolved by direct corpus read (`decisions.md
/// §22`, Piece 2 item 3):** the audit that scoped this wave claimed the
/// real owner of `PaDFE Construct`/`PaDFE Ooze`/`PaDFE Undead` is Ranger,
/// reachable through Ranger's own open-ended `choice:ranger_favored_enemy`
/// recognizer. Direct read of `ag_abilities_class.lst:382/390-392` and the
/// records those tokens chain into disproves that: each `PaDFE <Type>`
/// record's own `%1` substitution is `Favored<Type>`, a `DEFINE`d variable
/// set ONLY by Guardbreaker's `BONUS:VAR|FavoredConstruct,FavoredOoze,
/// FavoredUndead|TrapSenseBonus` -- a Pathfinder-Delver-only class feature,
/// gated `!PREABILITY:...Favored Enemy (<Type>)` (only applies when the
/// character does NOT already have Ranger's own real Favored Enemy of that
/// type). There is no `RangerLVL` or `RangerFavoredEnemy*` variable
/// anywhere in this record's own token closure -- Ranger's favored-enemy
/// table is a different, unrelated mechanism this wave leaves untouched.
///
/// `TrapSenseBonus` itself resolves through Pathfinder Delver's own level-2
/// grant (`ag_classes.lst:286`, `ABILITY:...|Rogue ~ Trap Sense` +
/// `BONUS:VAR|RogueTrapSenseLVL|CL+1`, `CL` = Pathfinder Delver level here)
/// feeding `Rogue ~ Trap Sense`'s own `BONUS:VAR|TrapSenseBonus|
/// RogueTrapSenseLVL/3` (`cr_abilities_class.lst:1618`) -- so for a
/// Pathfinder-Delver-only character (no separate Rogue levels contributing
/// to the same shared `RogueTrapSenseLVL` variable), `TrapSenseBonus =
/// (PaDLVL+1)/3`. Granted from class level 3 (Guardbreaker's own grant
/// gate, `ag_classes.lst:287`); `None` below level 3 (the level-2
/// `RogueTrapSenseLVL` term is already active by level 3, so no separate
/// level-2 threshold check is needed here).
pub(super) fn pathfinder_delver_padfe_bonus(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    let rogue_trap_sense_lvl = i16::from(level) + 1;
    Some(rogue_trap_sense_lvl / 3)
}

/// Ultimate Psionics Phrenic Slayer's 31 Favored Enemy creature-type
/// sub-records (`up_abilities_class.lst`, `KEY:Phrenic Slayer Favored Enemy
/// ~ <Type>`, one file per type under `data/corpus/ultimate_psionics/
/// class_feature/phrenic_slayer_favored_enemy/`): `(slug, display name)`,
/// verified directly against the real corpus `data.key`/`data.name` fields
/// for all 31 files (every `name` is byte-identical to the `key`'s own " ~ "
/// suffix). Each sub-record carries no own `DEFINE`/`BONUS` token -- only a
/// `%1` DESC substitution and an `ASPECT:Ability Benefit|+%1|
/// SlayerFavoredEnemy` referencing the SAME shared variable
/// `ground_phrenic_slayer_class_features`'s base-record explanation grounds.
/// Shared with `v06_work_inventory.rs`'s
/// `probe_phrenic_slayer_favored_enemy_wiring` so the id list is a single
/// source of truth, never duplicated.
pub const PHRENIC_SLAYER_FAVORED_ENEMY_MEMBERS: &[(&str, &str)] = &[
    ("aberration", "Aberration"),
    ("animal", "Animal"),
    ("construct", "Construct"),
    ("dragon", "Dragon"),
    ("fey", "Fey"),
    ("humanoid_aquatic", "Humanoid (Aquatic)"),
    ("humanoid_dwarf", "Humanoid (Dwarf)"),
    ("humanoid_elf", "Humanoid (Elf)"),
    ("humanoid_giant", "Humanoid (Giant)"),
    ("humanoid_gnoll", "Humanoid (Gnoll)"),
    ("humanoid_gnome", "Humanoid (Gnome)"),
    ("humanoid_goblinoid", "Humanoid (Goblinoid)"),
    ("humanoid_halfling", "Humanoid (Halfling)"),
    ("humanoid_human", "Humanoid (Human)"),
    ("humanoid_orc", "Humanoid (Orc)"),
    ("humanoid_reptilian", "Humanoid (Reptilian)"),
    ("magical_beast", "Magical Beast"),
    ("monstrous_humanoid", "Monstrous Humanoid"),
    ("ooze", "Ooze"),
    ("outsider_air", "Outsider (Air)"),
    ("outsider_chaotic", "Outsider (Chaotic)"),
    ("outsider_earth", "Outsider (Earth)"),
    ("outsider_evil", "Outsider (Evil)"),
    ("outsider_fire", "Outsider (Fire)"),
    ("outsider_good", "Outsider (Good)"),
    ("outsider_lawful", "Outsider (Lawful)"),
    ("outsider_native", "Outsider (Native)"),
    ("outsider_water", "Outsider (Water)"),
    ("plant", "Plant"),
    ("undead", "Undead"),
    ("vermin", "Vermin"),
];

/// Ultimate Psionics Phrenic Slayer Favored Enemy
/// (`up_abilities_class.lst:1326`, `KEY:Phrenic Slayer ~ Favored Enemy`):
/// `DEFINE:SlayerFavoredEnemy|0` / `BONUS:VAR|SlayerFavoredEnemy|
/// 2*floor((2+PhrenicSlayerLVL)/3)` -- a flat bonus on attack, damage, and
/// skill checks against the chosen favored-enemy creature type, shared
/// identically by every one of
/// [`PHRENIC_SLAYER_FAVORED_ENEMY_MEMBERS`]'s 31 creature-type sub-records
/// (verified directly: `SlayerFavoredEnemy` is defined ONCE, on this base
/// record; every sub-record's own `ASPECT` references it by name, with no
/// own `DEFINE`/`BONUS` token). Granted from class level 1
/// (`up_classes.lst:935`, `1  ABILITY:Phrenic Slayer Class Feature|
/// AUTOMATIC|Phrenic Slayer ~ Favored Enemy`); `PhrenicSlayerLVL = CL`
/// (`up_classes.lst:932`, the class's own raw level, no prime-stat
/// resolution needed). `None` below level 1 (never reachable in practice --
/// named for the same honesty `duelist_precise_strike_damage_bonus`'s own
/// level-1 gate states).
pub(super) fn phrenic_slayer_favored_enemy_bonus(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let level = i16::from(level);
    Some(2 * ((2 + level) / 3))
}

/// Divine Scion Domain Specialization's own pool-choice base record
/// (`ism_abilities_class.lst:30`, `KEY:Divine Scion ~ Domain
/// Specialization`): `DEFINE:DomainSpecBonus|0` / `BONUS:ABILITYPOOL|Domain
/// Specialization|1` -- the SIZE of the domain-specialization choice pool,
/// literally and unconditionally 1 (a divine scion specializes in exactly
/// one domain granted by her deity), the same "grounds the pool SIZE only"
/// shape `loremaster_secret_lore_pool_size` already established. Granted
/// automatically at class level 3 (`ism_classes.lst:104`). `None` below
/// level 3.
pub(super) fn divine_scion_domain_specialization_pool_size(level: u8) -> Option<i16> {
    if level < 3 {
        return None;
    }
    Some(1)
}

/// Divine Scion Divine Wrath (`ism_abilities_class.lst:31`, `KEY:Divine
/// Scion ~ Divine Wrath`): `DEFINE:DivineWrathBonus|0` /
/// `BONUS:VAR|DivineWrathBonus|1` -- a flat +1 damage per die against
/// creatures matching the divine scion's own opposition alignment,
/// granted automatically at class level 4 (`ism_classes.lst:105`). True
/// Scion's own further `BONUS:VAR|DivineWrathBonus|1` increment (raising
/// this to +2) is NOT modelled here -- True Scion Charisma/Wisdom are this
/// class's own two remaining, mutually-exclusive open sm5 units, left
/// unattempted this cycle (a real `ABILITYPOOL|True Scion|1` choice this
/// engine does not yet track a selection for). `None` below level 4.
pub(super) fn divine_scion_divine_wrath_bonus(level: u8) -> Option<i16> {
    if level < 4 {
        return None;
    }
    Some(1)
}

/// Divine Scion Deific Defense (`ism_abilities_class.lst:32`, `KEY:Divine
/// Scion ~ Deific Defense`): `DEFINE:DeificDefenseBonus|0` /
/// `BONUS:VAR|DeificDefenseBonus|2` -- a flat DR 2, bypassed by attacks
/// with the divine scion's own opposition alignment subtype, granted
/// automatically at class level 7 (`ism_classes.lst:106`). True Scion's
/// own further `BONUS:VAR|DeificDefenseBonus|3` increment (raising this to
/// DR 5) is NOT modelled here, the same True-Scion boundary `divine_
/// scion_divine_wrath_bonus` above already documents. `None` below level
/// 7.
pub(super) fn divine_scion_deific_defense_bonus(level: u8) -> Option<i16> {
    if level < 7 {
        return None;
    }
    Some(2)
}

/// Divine Scion's four Opposition Alignment records (Chaotic/Evil/Good/
/// Lawful, `ism_abilities_class.lst:37-40`): each carries a single `DR`
/// token restating the SAME `DeificDefenseBonus` magnitude `divine_scion_
/// deific_defense_bonus` already grounds, e.g. `DR:DeificDefenseBonus/
/// evil|PREABILITY:1,CATEGORY=Special Ability,Divine Scion ~ Deific
/// Defense` -- gated on already having Deific Defense, itself
/// auto-granted at the identical class level 7. Reuses that value
/// directly rather than re-deriving it; each record's own separate
/// prose-only "+1 bonus on caster level checks to overcome spell
/// resistance" clause carries no `BONUS` token anywhere in the corpus and
/// is not modelled. `None` below level 7. The MAGNITUDE is the same
/// regardless of which of the four is the character's own opposition
/// alignment (this is the pure per-alignment formula; WHICH of the four
/// records actually fires for a given character is a separate,
/// `DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID`-gated question resolved by
/// `ground_divine_scion_class_features` below, corrected this cycle -- see
/// that function's own doc comment).
pub(super) fn divine_scion_opposition_alignment_dr(level: u8) -> Option<i16> {
    divine_scion_deific_defense_bonus(level)
}

/// Divine Scion Weapon and Armor Proficiency (`ism_abilities_class.lst:29`,
/// `KEY:Divine Scion ~ Weapon and Armor Proficiency`): `BONUS:VAR|
/// GreatWeapFocusQualify,WeapSpecQualify,GreatWeapSpecQualify|1` -- a flat
/// internal qualifying flag (waives the Fighter-level prerequisite for
/// Greater Weapon Focus/Weapon Specialization/Greater Weapon
/// Specialization), granted automatically at class level 1
/// (`ism_classes.lst:103`). Grounds the flag magnitude only: this engine
/// applies no feat-prerequisite waiver logic for it to feed into. `None`
/// below level 1.
pub(super) fn divine_scion_weapon_and_armor_proficiency_qualify_flag(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(1)
}

/// Divine Scion Domain Specialization's own 35 per-domain sub-records
/// (`ism_abilities_class.lst:49-83`), each `SPELLS:Innate|
/// TIMES=<1|3|ATWILL>|CASTERLEVEL=TL|<spell name>,<DC>` -- the per-domain
/// spell-like ability's uses-per-day literal, or `None` for the five
/// domains whose spell is `TIMES=ATWILL` (Chaos/Evil/Good/Law/Magic, each
/// a constant `Detect <Alignment/Magic>` effect) -- the same "`ATWILL` has
/// no discrete per-day count to ground" boundary `paladin_detect_evil`
/// already established. Verified against the real, non-ingested PCGen
/// oracle (`ism_abilities_class.lst`) for all 35 domains, not just the
/// ingested corpus JSON. Caster level for every domain is uniformly `TL`
/// (total character level, via `total_character_level` above) -- no
/// per-domain difference there, only the uses-per-day
/// literal (or its absence) varies by domain. Each domain's own secondary
/// skill/save/AC/CMD/concentration bonus (always a flat, alignment-typed
/// constant) is NOT modelled -- the same "ground the SLA triple, don't
/// model the effect" split `ground_shadowdancer_class_features` already
/// established; here even the "triple" narrows to caster level (+
/// uses-per-day when numeric), since the DC itself is never grounded for
/// any SPELLS-shaped record in this engine. This table lists every
/// domain's own formula so any one of them can be resolved once selected
/// (`ground_divine_scion_class_features` gates WHICH domain actually
/// applies on `DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID`, corrected
/// this cycle -- a real character only ever has ONE of these 35 active,
/// the `ism_abilities_class.lst:47` `# Domain Specialization choices`
/// section header's own words).
pub(super) const DIVINE_SCION_DOMAIN_SPECIALIZATION_USES_PER_DAY: &[(&str, &str, Option<i16>)] = &[
    ("air", "Air", Some(1)),
    ("animal", "Animal", Some(1)),
    ("artifice", "Artifice", Some(3)),
    ("chaos", "Chaos", None),
    ("charm", "Charm", Some(3)),
    ("community", "Community", Some(1)),
    ("darkness", "Darkness", Some(3)),
    ("death", "Death", Some(3)),
    ("destruction", "Destruction", Some(3)),
    ("earth", "Earth", Some(3)),
    ("evil", "Evil", None),
    ("fire", "Fire", Some(1)),
    ("glory", "Glory", Some(1)),
    ("good", "Good", None),
    ("healing", "Healing", Some(1)),
    ("knowledge", "Knowledge", Some(3)),
    ("law", "Law", None),
    ("liberation", "Liberation", Some(3)),
    ("luck", "Luck", Some(3)),
    ("madness", "Madness", Some(3)),
    ("magic", "Magic", None),
    ("nobility", "Nobility", Some(1)),
    ("plant", "Plant", Some(1)),
    ("protection", "Protection", Some(1)),
    ("repose", "Repose", Some(3)),
    ("rune", "Rune", Some(3)),
    ("scalykind", "Scalykind", Some(1)),
    ("strength", "Strength", Some(3)),
    ("sun", "Sun", Some(1)),
    ("travel", "Travel", Some(3)),
    ("trickery", "Trickery", Some(1)),
    ("void", "Void", Some(1)),
    ("war", "War", Some(3)),
    ("water", "Water", Some(1)),
    ("weather", "Weather", Some(1)),
];

/// Grounds Divine Scion's 43 magnitude-bearing class features --
/// `decisions.md §22`'s WAVE 47 UPDATE, sub-mechanism-5's "registered
/// prestige class, magnitude-only" remainder. Unconditional on chassis
/// support (no `ClassId`-family enum entry for this class, source book
/// `inner_sea_magic`), same placement as `ground_phrenic_slayer_class_
/// features` above.
///
/// **CORRECTION, same cycle (recovered from a stalled prior run and
/// verified against the real oracle before this function was committed):**
/// the first draft of this function ground all four Opposition Alignment
/// DR records and all 35 Domain Specialization per-domain records
/// unconditionally, for every Divine Scion character simultaneously. Both
/// are real `ABILITYPOOL`-gated one-of-N choices, not single-owner
/// unconditional grants -- `ism_abilities_class.lst:35`'s own `# Opposition
/// Alignment choices` and `:47`'s own `# Domain Specialization choices`
/// section headers say so directly, and `ism_classes.lst:103`/`:104` grant
/// `BONUS:ABILITYPOOL|Opposition Alignment|1` / `BONUS:ABILITYPOOL|Domain
/// Specialization|1` (pool SIZE 1: exactly one of each is ever active on a
/// real character), the exact shape `SORCERER_BLOODLINE_CHOICE_ID` /
/// `CLERIC_DOMAIN_CHOICE_ID` / `choice_selection` already gate everywhere
/// else in this file. Fixed here: the per-alignment DR block and the
/// per-domain block below now each gate on their own
/// `DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID` /
/// `DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID` selection, exactly one
/// member surfaces per real character, and the corpus-wide reachability
/// proof (`v06_work_inventory.rs`'s `probe_divine_scion_wiring`) sweeps
/// every one of the 4 + 35 candidate selections in turn (the same
/// `probe_cleric_domain_generic_member_wiring` idiom) rather than reading
/// a single fixture's own accidental silence as "always grounded."
///
/// **True Scion Charisma/Wisdom (this class's own remaining two sm5
/// units) are NOT attempted this cycle**: they are a real
/// `ABILITYPOOL|True Scion|1` mutually-exclusive choice between an
/// ability-score bump to Charisma or Wisdom (each also re-stating the
/// SAME `DomainSpecBonus`/`DivineWrathBonus`/`DeificDefenseBonus`
/// increments already excluded above) -- left named for a future wave
/// rather than guessed at.
pub(super) fn ground_divine_scion_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == DIVINE_SCION_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };

    if let Some(pool) = divine_scion_domain_specialization_pool_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_magic.divine_scion.domain_specialization.pool_size"
                .to_owned(),
            value: pool,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:ABILITYPOOL|Domain Specialization|1`
                "Divine Scion level {level} Domain Specialization: a pool of {pool}. Grounds the \
                 pool SIZE only -- which domain is specialized in is not modelled"
            ),
        });
    }

    if let Some(bonus) = divine_scion_divine_wrath_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_magic.divine_scion.divine_wrath.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Divine Scion level {level} Divine Wrath: +{bonus} damage per die against \
                 creatures matching the divine scion's own opposition alignment (corpus \
                 `DivineWrathBonus = 1`). Grounds the magnitude only: no damaging-spell total \
                 exists anywhere in this engine for it to layer onto; True Scion's own further \
                 increment is not modelled (see this function's own doc comment)"
            ),
        });
    }

    if let Some(bonus) = divine_scion_deific_defense_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_magic.divine_scion.deific_defense.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Divine Scion level {level} Deific Defense: DR {bonus}/(opposition alignment) \
                 (corpus `DeificDefenseBonus = 2`). Grounds the magnitude only: no damage- \
                 reduction total exists anywhere in this engine for it to layer onto; True \
                 Scion's own further increment is not modelled"
            ),
        });

        // `divine_scion_opposition_alignment_dr` is a pure re-export of
        // `divine_scion_deific_defense_bonus` (see its own doc comment) --
        // called explicitly here (rather than reusing `bonus` directly) so
        // the function has a real call site to pure-formula test against.
        //
        // CORRECTION (see this function's own doc comment): only the ONE
        // alignment the character actually recorded via
        // `DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID` surfaces -- a real
        // divine scion has exactly one opposition alignment, never all
        // four at once.
        if let Some(dr) = divine_scion_opposition_alignment_dr(level)
            && let Some(selection) =
                choice_selection(input, DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID)
        {
            for (slug, display) in [
                ("chaotic", "Chaotic"),
                ("evil", "Evil"),
                ("good", "Good"),
                ("lawful", "Lawful"),
            ] {
                if selection != format!("alignment:{slug}") {
                    continue;
                }
                explanations.push(ComputationExplanation {
                    id: format!(
                        "class_feature.inner_sea_magic.divine_scion.{slug}_opposition_\
                         alignment.dr"
                    ),
                    value: dr,
                    detail: format!(
                        "Divine Scion level {level} {display} Opposition Alignment: DR {dr} \
                         bypassed by {display_lower} creatures (corpus \
                         `DR:DeificDefenseBonus/{slug}`, restating the same \
                         `DeificDefenseBonus` magnitude Deific Defense already grounds; \
                         recorded selection {DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID} -> \
                         {selection}). Grounds the magnitude only; the record's own separate \
                         +1 caster-level-check bonus carries no `BONUS` token anywhere and is \
                         not modelled",
                        display_lower = display.to_lowercase()
                    ),
                });
            }
        }
    }

    if let Some(flag) = divine_scion_weapon_and_armor_proficiency_qualify_flag(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.inner_sea_magic.divine_scion.weapon_and_armor_proficiency.\
                 qualify_flag"
                .to_owned(),
            value: flag,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|GreatWeapFocusQualify,WeapSpecQualify,GreatWeapSpecQualify|1`
                "Divine Scion level {level} Weapon and Armor Proficiency: a qualifying flag waiving \
                 the Fighter-level prerequisite for Greater Weapon Focus/Weapon \
                 Specialization/Greater Weapon Specialization. Grounds the flag magnitude only: this \
                 engine applies no feat-prerequisite waiver logic for it to feed into"
            ),
        });
    }

    // CORRECTION (see this function's own doc comment): only the ONE
    // domain the character actually recorded via
    // `DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID` surfaces -- a real
    // divine scion specializes in exactly one domain, never all 35 at
    // once.
    if level >= 3
        && let Some(selection) =
            choice_selection(input, DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID)
        && let Some((slug, display, uses_per_day)) = DIVINE_SCION_DOMAIN_SPECIALIZATION_USES_PER_DAY
            .iter()
            .find(|(slug, _, _)| selection == format!("domain:{slug}"))
    {
        let caster_level = total_character_level(input);
        explanations.push(ComputationExplanation {
            id: format!(
                "class_feature.inner_sea_magic.divine_scion.{slug}_specialization.\
                 caster_level"
            ),
            value: caster_level,
            detail: format!(
                "Divine Scion {display} Specialization: spell-like ability, caster \
                 level {caster_level} (corpus `SPELLS:Innate|...|CASTERLEVEL=TL|...`, \
                 `TL` = this character's total level across every class, \
                 {caster_level}; recorded selection \
                 {DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID} -> {selection}). Grounds \
                 the caster-level fact only -- the SLA triple idiom already established \
                 by `ground_summoner_slice_a_features`: no spell effect, save DC, or \
                 secondary skill/save/AC/CMD/concentration bonus is modelled"
            ),
        });

        if let Some(times) = uses_per_day {
            explanations.push(ComputationExplanation {
                id: format!(
                    "class_feature.inner_sea_magic.divine_scion.{slug}_specialization.\
                     uses_per_day"
                ),
                value: *times,
                detail: format!(
                    "Divine Scion {display} Specialization uses per day: {times} \
                     (corpus `SPELLS:Innate|TIMES={times}|...`). Grounds the per-day \
                     budget only"
                ),
            });
        }
    }
}

/// SD-34 AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_
/// held_by_engine` mechanism, cycle 7). Proves the widened precondition on
/// `compute_pilot_base_chassis`'s generic class_feature grant roster call
/// (this module, just above `compute_class_chassis`'s prestige-entry-gate
/// branch): a prestige-class-only character now gets the SAME generic
/// roster every other modelled class already gets, even though
/// `compute_class_chassis` genuinely produces no BAB/save chassis for any
/// CRB prestige class (asserted directly, as this suite's own premise, not
/// assumed).
#[cfg(test)]
mod prestige_class_feature_generic_grant_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, ASSASSIN_CLASS_ID,
        SHADOWDANCER_CLASS_ID,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture must load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn explanation(input: &CharacterInput, id: &str) -> Option<(i16, String)> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .into_iter()
            .find(|e| e.id == id)
            .map(|e| (e.value, e.detail))
    }

    #[test]
    fn assassin_class_features_ground_even_though_no_bab_save_chassis_exists() {
        let input = character(ASSASSIN_CLASS_ID, 8);
        assert!(
            explanation(&input, "class_chassis.base_attack_bonus").is_none(),
            "this test's own premise: a CRB prestige class's chassis is genuinely \
             unsupported by compute_class_chassis today"
        );
        let (value, detail) =
            explanation(&input, "class_feature.assassin.corpus_record.hidden_weapons")
                .expect("a level-8 Assassin must be granted Hidden Weapons (real, level-4 grant)");
        assert_eq!(value, 4);
        assert!(
            detail.contains("Hidden Weapons") && detail.contains("granted from class level 4"),
            "must name the real feature and its real granted-at level: {detail}"
        );
        let (death_value, _) =
            explanation(&input, "class_feature.assassin.corpus_record.true_death")
                .expect("a level-8 Assassin must also be granted True Death (level-4 grant)");
        assert_eq!(death_value, 4);
    }

    #[test]
    fn shadowdancer_class_features_ground_even_though_no_bab_save_chassis_exists() {
        let input = character(SHADOWDANCER_CLASS_ID, 8);
        assert!(explanation(&input, "class_chassis.base_attack_bonus").is_none());
        let (value, detail) =
            explanation(&input, "class_feature.shadowdancer.corpus_record.darkvision")
                .expect("a level-8 Shadowdancer must be granted Darkvision (real, level-2 grant)");
        assert_eq!(value, 2);
        assert!(
            detail.contains("Darkvision") && detail.contains("granted from class level 2"),
            "must name the real feature and its real granted-at level: {detail}"
        );
    }

    #[test]
    fn duelist_deflect_arrows_grounds_even_though_no_bab_save_chassis_exists() {
        let input = character("class:duelist", 9);
        assert!(explanation(&input, "class_chassis.base_attack_bonus").is_none());
        let (value, detail) =
            explanation(&input, "class_feature.duelist.corpus_record.deflect_arrows")
                .expect("a level-9 Duelist must be granted Deflect Arrows (real, level-9 grant)");
        assert_eq!(value, 9);
        assert!(detail.contains("Deflect Arrows"), "must name the real feature: {detail}");
    }

    #[test]
    fn an_unrecognized_class_id_still_grounds_nothing_from_the_widened_gate() {
        // Sanity: the widened gate is keyed on the real prestige-class-entry
        // registry, not merely "chassis_supported happened to be false" --
        // an arbitrary unrecognized class id must still ground nothing.
        let input = character("class:not_a_real_class", 5);
        assert!(explanation(&input, "class_chassis.base_attack_bonus").is_none());
        let has_roster_id = build_pilot_headless_receipt(&input)
            .computation
            .explanations
            .iter()
            .any(|e| e.id.contains(".not_a_real_class."));
        assert!(!has_roster_id, "an unrecognized class id must never gain the generic roster");
    }
}

/// SD-32 Epic 3 (`epic-3-class-reachability`, AT-32-E3-001): proves the
/// prestige-class entry-requirement gate really runs through the real
/// `compute_pilot_base_chassis` → `compute_class_chassis` call site (not a
/// direct unit call on `prestige_class_entry_gate` alone) for a real
/// corpus-derived prestige class, `class:arcane_archer`.
#[cfg(test)]
mod prestige_class_entry_gate_wiring_tests {
    use super::{compute_pilot_base_chassis, PilotBaseChassisComputation};
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn arcane_archer_input(feats: Vec<String>) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly: {:?}", result.diagnostics);
        let mut input = result
            .character_input
            .expect("valid fixture should produce a character input record");
        input.chosen.class_levels[0].class_id = "class:arcane_archer".to_owned();
        input.chosen.class_levels[0].level = 1;
        input.chosen.selected_feats = feats;
        input
    }

    fn diagnostic<'a>(
        computation: &'a PilotBaseChassisComputation,
        id: &str,
    ) -> Option<&'a str> {
        computation
            .diagnostics
            .iter()
            .find(|d| d.id == id)
            .map(|d| d.message.as_str())
    }

    /// RED, confirmed manually before this wiring landed (SD-31 wave 27's
    /// own investigation note above `compute_generic_table_chassis`): the
    /// `else { None }` arm produced no diagnostic at all for a
    /// `class:arcane_archer` single-class input -- entry requirements were
    /// simply never asked about. GREEN below.
    #[test]
    fn prestige_class_dispatch_still_leaves_chassis_unsupported() {
        let input = arcane_archer_input(vec![]);
        let computation = compute_pilot_base_chassis(&input);
        assert_eq!(computation.base_attack_bonus, 0);
        assert!(
            diagnostic(&computation, "class_chassis.unsupported").is_some(),
            "the generic unsupported-chassis diagnostic must still fire: {:?}",
            computation.diagnostics
        );
    }

    #[test]
    fn missing_feats_surfaces_the_unmet_entry_gate_diagnostic() {
        let input = arcane_archer_input(vec![]);
        let computation = compute_pilot_base_chassis(&input);
        let message = diagnostic(&computation, "class_chassis.prestige_entry_gate.unmet")
            .unwrap_or_else(|| panic!("expected unmet entry-gate diagnostic: {:?}", computation.diagnostics));
        assert!(message.contains("Arcane Archer"));
        assert!(diagnostic(&computation, "class_chassis.prestige_entry_gate.met").is_none());
    }

    #[test]
    fn real_feats_surface_the_met_entry_gate_diagnostic() {
        let input = arcane_archer_input(vec![
            "Point-Blank Shot".to_owned(),
            "Precise Shot".to_owned(),
            "Weapon Focus (Longbow)".to_owned(),
        ]);
        let computation = compute_pilot_base_chassis(&input);
        let message = diagnostic(&computation, "class_chassis.prestige_entry_gate.met")
            .unwrap_or_else(|| panic!("expected met entry-gate diagnostic: {:?}", computation.diagnostics));
        assert!(message.contains("Arcane Archer"));
        assert!(diagnostic(&computation, "class_chassis.prestige_entry_gate.unmet").is_none());
    }
}

/// SD-34 wave 43 (`decisions.md §22`'s 12-unit "small-precedented-new-compute"
/// remainder): Duelist ×4, Shadowdancer ×4, Assassin ×2, Loremaster ×2. Each
/// pure formula is unit-tested directly first (proving the arithmetic,
/// including edge cases the fixture below cannot exercise -- a negative or
/// high Intelligence modifier for Canny Defense, a range of level bands for
/// Shadow Jump), then a reachability test proves the SAME formula's
/// explanation id actually surfaces through the real pipeline end to end,
/// the identical two-layer discipline `paladin_detect_evil_and_cleric_aura_
/// tests` above already established.
#[cfg(test)]
mod wave43_prestige_class_new_compute_tests {
    use super::{
        assassin_death_attack_duration_bonus_rounds, assassin_death_attack_save_dc,
        assassin_save_against_poisons_bonus, build_pilot_headless_receipt,
        duelist_canny_defense_dodge_bonus, duelist_elaborate_defense_dodge_bonus,
        duelist_improved_reaction_initiative_bonus, duelist_precise_strike_damage_bonus,
        loremaster_lore_knowledge_bonus, loremaster_secret_lore_pool_size,
        shadowdancer_shadow_call_caster_level, shadowdancer_shadow_call_uses_per_day,
        shadowdancer_shadow_illusion_caster_level, shadowdancer_shadow_jump_daily_distance_feet,
        shadowdancer_summon_shadow_companion_level, CharacterClassLevel, CharacterInput,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// This fixture's own Human ability-bonus choice targets Strength, not
    /// Intelligence (`choice:human_ability_bonus:ability:strength`), so the
    /// character's Intelligence stays the raw chosen `10` -> modifier `0`.
    /// Used below to confirm Canny Defense's and Death Attack's own real
    /// pipeline values, independent of the direct formula tests (which
    /// exercise non-zero Intelligence modifiers the fixture cannot).
    const FIXTURE_INTELLIGENCE_MODIFIER: i16 = 0;

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture must load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn explanation_value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    const ALL_FIFTEEN_EXPLANATION_IDS: &[&str] = &[
        "class_feature.duelist.canny_defense.dodge_bonus",
        "class_feature.duelist.improved_reaction.initiative_bonus",
        "class_feature.duelist.precise_strike.damage_bonus",
        "class_feature.duelist.elaborate_defense.dodge_bonus",
        "class_feature.shadowdancer.shadow_illusion.caster_level",
        "class_feature.shadowdancer.shadow_illusion.uses_per_day",
        "class_feature.shadowdancer.shadow_call.caster_level",
        "class_feature.shadowdancer.shadow_call.uses_per_day",
        "class_feature.shadowdancer.shadow_jump.daily_distance_feet",
        "class_feature.shadowdancer.summon_shadow.companion_level",
        "class_feature.assassin.save_against_poisons.save_bonus",
        "class_feature.assassin.death_attack.save_dc",
        "class_feature.assassin.death_attack.duration_bonus_rounds",
        "class_feature.loremaster.lore.knowledge_bonus",
        "class_feature.loremaster.secret_lore.pool_size",
    ];

    #[test]
    fn duelist_formulas_match_the_corpus_tokens() {
        // Canny Defense: max(0, min(INT, level)), None below level 1.
        assert_eq!(duelist_canny_defense_dodge_bonus(0, 5), None);
        assert_eq!(duelist_canny_defense_dodge_bonus(1, 5), Some(1));
        assert_eq!(duelist_canny_defense_dodge_bonus(5, 2), Some(2));
        assert_eq!(
            duelist_canny_defense_dodge_bonus(5, -3),
            Some(0),
            "a negative Intelligence modifier must floor at 0, never go negative"
        );

        // Improved Reaction: floor((level+4)/6)*2, None below level 2.
        assert_eq!(duelist_improved_reaction_initiative_bonus(1), None);
        assert_eq!(duelist_improved_reaction_initiative_bonus(2), Some(2));
        assert_eq!(duelist_improved_reaction_initiative_bonus(7), Some(2));
        assert_eq!(duelist_improved_reaction_initiative_bonus(8), Some(4));

        // Precise Strike: level, None below level 1.
        assert_eq!(duelist_precise_strike_damage_bonus(0), None);
        assert_eq!(duelist_precise_strike_damage_bonus(1), Some(1));
        assert_eq!(duelist_precise_strike_damage_bonus(20), Some(20));

        // Elaborate Defense: level/3, None below level 7.
        assert_eq!(duelist_elaborate_defense_dodge_bonus(6), None);
        assert_eq!(duelist_elaborate_defense_dodge_bonus(7), Some(2));
        assert_eq!(duelist_elaborate_defense_dodge_bonus(9), Some(3));
    }

    #[test]
    fn shadowdancer_formulas_match_the_corpus_tokens() {
        // Shadow Illusion caster level: raw level, None below level 3.
        assert_eq!(shadowdancer_shadow_illusion_caster_level(2), None);
        assert_eq!(shadowdancer_shadow_illusion_caster_level(3), Some(3));
        assert_eq!(shadowdancer_shadow_illusion_caster_level(20), Some(20));

        // Shadow Call: raw level / level/2, None below level 4.
        assert_eq!(shadowdancer_shadow_call_caster_level(3), None);
        assert_eq!(shadowdancer_shadow_call_caster_level(4), Some(4));
        assert_eq!(shadowdancer_shadow_call_uses_per_day(4), Some(2));
        assert_eq!(shadowdancer_shadow_call_uses_per_day(5), Some(2));
        assert_eq!(shadowdancer_shadow_call_uses_per_day(6), Some(3));

        // Shadow Jump: cumulative banded lookup, None below level 4.
        assert_eq!(shadowdancer_shadow_jump_daily_distance_feet(3), None);
        assert_eq!(shadowdancer_shadow_jump_daily_distance_feet(4), Some(20));
        assert_eq!(shadowdancer_shadow_jump_daily_distance_feet(5), Some(20));
        assert_eq!(shadowdancer_shadow_jump_daily_distance_feet(6), Some(40));
        assert_eq!(shadowdancer_shadow_jump_daily_distance_feet(7), Some(40));
        assert_eq!(shadowdancer_shadow_jump_daily_distance_feet(8), Some(80));
        assert_eq!(shadowdancer_shadow_jump_daily_distance_feet(9), Some(80));
        assert_eq!(shadowdancer_shadow_jump_daily_distance_feet(10), Some(160));
        assert_eq!(shadowdancer_shadow_jump_daily_distance_feet(20), Some(160));

        // Summon Shadow: raw level, None below level 3.
        assert_eq!(shadowdancer_summon_shadow_companion_level(2), None);
        assert_eq!(shadowdancer_summon_shadow_companion_level(3), Some(3));
    }

    #[test]
    fn assassin_formulas_match_the_corpus_tokens() {
        // Save against Poisons: level/2, None below level 2.
        assert_eq!(assassin_save_against_poisons_bonus(1), None);
        assert_eq!(assassin_save_against_poisons_bonus(2), Some(1));
        assert_eq!(assassin_save_against_poisons_bonus(3), Some(1));
        assert_eq!(assassin_save_against_poisons_bonus(4), Some(2));

        // Death Attack DC: 10 + level + INT modifier, None below level 1.
        assert_eq!(assassin_death_attack_save_dc(0, 0), None);
        assert_eq!(assassin_death_attack_save_dc(1, 0), Some(11));
        assert_eq!(assassin_death_attack_save_dc(1, 3), Some(14));
        assert_eq!(assassin_death_attack_save_dc(10, 2), Some(22));

        // Death Attack duration: level, None below level 1.
        assert_eq!(assassin_death_attack_duration_bonus_rounds(0), None);
        assert_eq!(assassin_death_attack_duration_bonus_rounds(1), Some(1));
        assert_eq!(assassin_death_attack_duration_bonus_rounds(10), Some(10));
    }

    #[test]
    fn loremaster_formulas_match_the_corpus_tokens() {
        // Lore: level/2, None below level 2.
        assert_eq!(loremaster_lore_knowledge_bonus(1), None);
        assert_eq!(loremaster_lore_knowledge_bonus(2), Some(1));
        assert_eq!(loremaster_lore_knowledge_bonus(3), Some(1));
        assert_eq!(loremaster_lore_knowledge_bonus(4), Some(2));

        // Secret Lore pool size: (level+1)/2, None below level 1.
        assert_eq!(loremaster_secret_lore_pool_size(0), None);
        assert_eq!(loremaster_secret_lore_pool_size(1), Some(1));
        assert_eq!(loremaster_secret_lore_pool_size(3), Some(2));
        assert_eq!(loremaster_secret_lore_pool_size(9), Some(5));
    }

    #[test]
    fn duelist_class_features_reach_the_real_pipeline() {
        let level1 = character("class:duelist", 1);
        assert_eq!(
            explanation_value(&level1, "class_feature.duelist.canny_defense.dodge_bonus"),
            Some(0.max(FIXTURE_INTELLIGENCE_MODIFIER.min(1))),
            "level 1 Canny Defense must ground at this fixture's real Intelligence modifier"
        );
        assert_eq!(
            explanation_value(&level1, "class_feature.duelist.precise_strike.damage_bonus"),
            Some(1)
        );
        assert_eq!(
            explanation_value(&level1, "class_feature.duelist.improved_reaction.initiative_bonus"),
            None,
            "Improved Reaction is not granted until level 2"
        );
        assert_eq!(
            explanation_value(&level1, "class_feature.duelist.elaborate_defense.dodge_bonus"),
            None,
            "Elaborate Defense is not granted until level 7"
        );

        let level7 = character("class:duelist", 7);
        assert_eq!(
            explanation_value(&level7, "class_feature.duelist.precise_strike.damage_bonus"),
            Some(7)
        );
        assert_eq!(
            explanation_value(&level7, "class_feature.duelist.improved_reaction.initiative_bonus"),
            Some(2)
        );
        assert_eq!(
            explanation_value(&level7, "class_feature.duelist.elaborate_defense.dodge_bonus"),
            Some(2)
        );
    }

    #[test]
    fn shadowdancer_class_features_reach_the_real_pipeline() {
        let level3 = character("class:shadowdancer", 3);
        assert_eq!(
            explanation_value(&level3, "class_feature.shadowdancer.shadow_illusion.caster_level"),
            Some(3)
        );
        assert_eq!(
            explanation_value(&level3, "class_feature.shadowdancer.shadow_illusion.uses_per_day"),
            Some(1)
        );
        assert_eq!(
            explanation_value(&level3, "class_feature.shadowdancer.summon_shadow.companion_level"),
            Some(3)
        );
        assert_eq!(
            explanation_value(&level3, "class_feature.shadowdancer.shadow_call.caster_level"),
            None,
            "Shadow Call is not granted until level 4"
        );
        assert_eq!(
            explanation_value(&level3, "class_feature.shadowdancer.shadow_jump.daily_distance_feet"),
            None,
            "Shadow Jump is not granted until level 4"
        );

        let level10 = character("class:shadowdancer", 10);
        assert_eq!(
            explanation_value(&level10, "class_feature.shadowdancer.shadow_call.caster_level"),
            Some(10)
        );
        assert_eq!(
            explanation_value(&level10, "class_feature.shadowdancer.shadow_call.uses_per_day"),
            Some(5)
        );
        assert_eq!(
            explanation_value(
                &level10,
                "class_feature.shadowdancer.shadow_jump.daily_distance_feet"
            ),
            Some(160)
        );
    }

    #[test]
    fn assassin_class_features_reach_the_real_pipeline() {
        let level1 = character("class:assassin", 1);
        assert_eq!(
            explanation_value(&level1, "class_feature.assassin.death_attack.save_dc"),
            Some(10 + 1 + FIXTURE_INTELLIGENCE_MODIFIER)
        );
        assert_eq!(
            explanation_value(&level1, "class_feature.assassin.death_attack.duration_bonus_rounds"),
            Some(1)
        );
        assert_eq!(
            explanation_value(&level1, "class_feature.assassin.save_against_poisons.save_bonus"),
            None,
            "Save against Poisons is not granted until level 2"
        );

        let level2 = character("class:assassin", 2);
        assert_eq!(
            explanation_value(&level2, "class_feature.assassin.save_against_poisons.save_bonus"),
            Some(1)
        );
    }

    #[test]
    fn loremaster_class_features_reach_the_real_pipeline() {
        let level1 = character("class:loremaster", 1);
        assert_eq!(
            explanation_value(&level1, "class_feature.loremaster.secret_lore.pool_size"),
            Some(1)
        );
        assert_eq!(
            explanation_value(&level1, "class_feature.loremaster.lore.knowledge_bonus"),
            None,
            "Lore is not granted until level 2"
        );

        let level2 = character("class:loremaster", 2);
        assert_eq!(
            explanation_value(&level2, "class_feature.loremaster.lore.knowledge_bonus"),
            Some(1)
        );
        assert_eq!(
            explanation_value(&level2, "class_feature.loremaster.secret_lore.pool_size"),
            Some(1)
        );
    }

    /// Negative control: none of the 15 explanation ids these four classes
    /// push may leak onto an unrelated class, nor onto any of the OTHER
    /// three prestige classes -- the same discipline `paladin_detect_evil_
    /// and_cleric_aura_tests::neither_record_leaks_onto_an_unrelated_class`
    /// already established for the two-unit wave 42 cycle, widened to all
    /// four classes and all fifteen ids here.
    #[test]
    fn none_of_the_fifteen_ids_leak_onto_an_unrelated_or_sibling_prestige_class() {
        let fighter = character("class:fighter", 10);
        for id in ALL_FIFTEEN_EXPLANATION_IDS {
            assert_eq!(
                explanation_value(&fighter, id),
                None,
                "a Fighter must not gain any of these fifteen prestige-class records: {id}"
            );
        }

        // Each class only ever pushes its OWN records -- spot-checked cross-class,
        // at a level high enough that every one of the OTHER three classes' own
        // records would have fired had the dispatch been mis-keyed.
        let duelist = character("class:duelist", 10);
        for id in ALL_FIFTEEN_EXPLANATION_IDS {
            if id.starts_with("class_feature.duelist.") {
                continue;
            }
            assert_eq!(explanation_value(&duelist, id), None, "a Duelist must not gain: {id}");
        }

        let shadowdancer = character("class:shadowdancer", 10);
        for id in ALL_FIFTEEN_EXPLANATION_IDS {
            if id.starts_with("class_feature.shadowdancer.") {
                continue;
            }
            assert_eq!(
                explanation_value(&shadowdancer, id),
                None,
                "a Shadowdancer must not gain: {id}"
            );
        }

        let assassin = character("class:assassin", 10);
        for id in ALL_FIFTEEN_EXPLANATION_IDS {
            if id.starts_with("class_feature.assassin.") {
                continue;
            }
            assert_eq!(explanation_value(&assassin, id), None, "an Assassin must not gain: {id}");
        }

        let loremaster = character("class:loremaster", 10);
        for id in ALL_FIFTEEN_EXPLANATION_IDS {
            if id.starts_with("class_feature.loremaster.") {
                continue;
            }
            assert_eq!(
                explanation_value(&loremaster, id),
                None,
                "a Loremaster must not gain: {id}"
            );
        }
    }
}

/// SD-34 wave 44 (`decisions.md §22`, Piece 2 item 3): Pathfinder Delver's
/// PaDFE Construct/Ooze/Undead, the same two-layer discipline
/// (`wave43_prestige_class_new_compute_tests` above) -- the pure formula
/// first, including edge cases the fixture cannot exercise, then a
/// reachability test proving the SAME formula's three explanation ids
/// actually surface through the real pipeline end to end.
#[cfg(test)]
mod wave44_pathfinder_delver_padfe_tests {
    use super::{build_pilot_headless_receipt, pathfinder_delver_padfe_bonus,
        CharacterClassLevel, CharacterInput};
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture must load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn explanation_value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    const ALL_THREE_EXPLANATION_IDS: &[&str] = &[
        "class_feature.adventurers_guide.pathfinder_delver.padfe_construct.bonus",
        "class_feature.adventurers_guide.pathfinder_delver.padfe_ooze.bonus",
        "class_feature.adventurers_guide.pathfinder_delver.padfe_undead.bonus",
    ];

    #[test]
    fn padfe_bonus_formula_matches_the_corpus_tokens() {
        // Guardbreaker is granted from level 3; TrapSenseBonus =
        // RogueTrapSenseLVL/3, RogueTrapSenseLVL = PaDLVL+1 (Pathfinder
        // Delver's own level-2 grant line, `ag_classes.lst:286`).
        assert_eq!(pathfinder_delver_padfe_bonus(1), None);
        assert_eq!(pathfinder_delver_padfe_bonus(2), None, "Guardbreaker itself grants at level 3");
        assert_eq!(pathfinder_delver_padfe_bonus(3), Some(1), "(3+1)/3 = 1");
        assert_eq!(pathfinder_delver_padfe_bonus(5), Some(2), "(5+1)/3 = 2");
        assert_eq!(pathfinder_delver_padfe_bonus(8), Some(3), "(8+1)/3 = 3");
        assert_eq!(pathfinder_delver_padfe_bonus(20), Some(7), "(20+1)/3 = 7");
    }

    #[test]
    fn all_three_padfe_ids_reach_the_real_pipeline() {
        let below_gate = character("class:pathfinder_delver", 2);
        for id in ALL_THREE_EXPLANATION_IDS {
            assert_eq!(
                explanation_value(&below_gate, id),
                None,
                "below Guardbreaker's own level-3 grant gate, {id} must not appear"
            );
        }

        let above_gate = character("class:pathfinder_delver", 5);
        for id in ALL_THREE_EXPLANATION_IDS {
            assert_eq!(
                explanation_value(&above_gate, id),
                Some(2),
                "level 5 Pathfinder Delver: (5+1)/3 = 2, {id}"
            );
        }
    }

    #[test]
    fn padfe_ids_never_leak_onto_an_unrelated_class() {
        let fighter = character("class:fighter", 10);
        for id in ALL_THREE_EXPLANATION_IDS {
            assert_eq!(
                explanation_value(&fighter, id),
                None,
                "a Fighter must not gain Pathfinder Delver's PaDFE records: {id}"
            );
        }

        // A real Ranger (not Pathfinder Delver) must not gain these either --
        // this wave's own audit correction found Ranger's favored-enemy
        // mechanism is NOT the real owner of these three records.
        let ranger = character("class:ranger", 10);
        for id in ALL_THREE_EXPLANATION_IDS {
            assert_eq!(
                explanation_value(&ranger, id),
                None,
                "a Ranger must not gain Pathfinder Delver's own PaDFE records: {id}"
            );
        }
    }
}

/// SD-34 wave 45 (`decisions.md §22`'s WAVE 45 UPDATE): Phrenic Slayer's
/// Favored Enemy record (base + 31 creature-type sub-records), the same
/// two-layer discipline (`wave43_prestige_class_new_compute_tests`,
/// `wave44_pathfinder_delver_padfe_tests` above) -- the pure formula first,
/// including edge cases the fixture cannot exercise, then a reachability
/// test proving all 32 explanation ids actually surface through the real
/// pipeline end to end.
#[cfg(test)]
mod wave45_phrenic_slayer_favored_enemy_tests {
    use super::{
        build_pilot_headless_receipt, phrenic_slayer_favored_enemy_bonus, CharacterClassLevel,
        CharacterInput, PHRENIC_SLAYER_FAVORED_ENEMY_MEMBERS,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture must load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn explanation_value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    const BASE_ID: &str = "class_feature.ultimate_psionics.phrenic_slayer.favored_enemy.bonus";

    fn all_thirty_two_ids() -> Vec<String> {
        let mut ids = vec![BASE_ID.to_owned()];
        for (slug, _) in PHRENIC_SLAYER_FAVORED_ENEMY_MEMBERS {
            ids.push(format!(
                "class_feature.ultimate_psionics.phrenic_slayer.favored_enemy_{slug}.bonus"
            ));
        }
        ids
    }

    #[test]
    fn member_list_has_exactly_thirty_one_creature_types_matching_the_real_corpus() {
        // Verified directly against `data/corpus/ultimate_psionics/
        // class_feature/phrenic_slayer_favored_enemy/*.json` -- 31 files,
        // one per creature type (`ls | wc -l` re-counted directly after an
        // initial miscount of 30 during this cycle's own investigation).
        assert_eq!(PHRENIC_SLAYER_FAVORED_ENEMY_MEMBERS.len(), 31);
        assert_eq!(all_thirty_two_ids().len(), 32, "base record + 31 creature types");
    }

    #[test]
    fn favored_enemy_bonus_formula_matches_the_corpus_token() {
        // `SlayerFavoredEnemy = 2*floor((2+PhrenicSlayerLVL)/3)`, granted
        // from level 1 (`up_classes.lst:935`).
        assert_eq!(phrenic_slayer_favored_enemy_bonus(1), Some(2), "2*floor(3/3) = 2");
        assert_eq!(phrenic_slayer_favored_enemy_bonus(2), Some(2), "2*floor(4/3) = 2");
        assert_eq!(phrenic_slayer_favored_enemy_bonus(3), Some(2), "2*floor(5/3) = 2");
        assert_eq!(phrenic_slayer_favored_enemy_bonus(4), Some(4), "2*floor(6/3) = 4");
        assert_eq!(phrenic_slayer_favored_enemy_bonus(6), Some(4), "2*floor(8/3) = 4");
        assert_eq!(phrenic_slayer_favored_enemy_bonus(7), Some(6), "2*floor(9/3) = 6");
        assert_eq!(phrenic_slayer_favored_enemy_bonus(9), Some(6), "2*floor(11/3) = 6");
        assert_eq!(phrenic_slayer_favored_enemy_bonus(10), Some(8), "2*floor(12/3) = 8");
    }

    #[test]
    fn all_thirty_two_ids_reach_the_real_pipeline_at_level_one() {
        let level_1 = character("class:phrenic_slayer", 1);
        for id in all_thirty_two_ids() {
            assert_eq!(
                explanation_value(&level_1, &id),
                Some(2),
                "level 1 Phrenic Slayer: 2*floor(3/3) = 2, {id}"
            );
        }
    }

    #[test]
    fn all_thirty_two_ids_reach_the_real_pipeline_at_a_higher_level() {
        let level_10 = character("class:phrenic_slayer", 10);
        for id in all_thirty_two_ids() {
            assert_eq!(
                explanation_value(&level_10, &id),
                Some(8),
                "level 10 Phrenic Slayer: 2*floor(12/3) = 8, {id}"
            );
        }
    }

    #[test]
    fn ids_never_leak_onto_an_unrelated_class() {
        let fighter = character("class:fighter", 10);
        for id in all_thirty_two_ids() {
            assert_eq!(
                explanation_value(&fighter, &id),
                None,
                "a Fighter must not gain Phrenic Slayer's Favored Enemy records: {id}"
            );
        }

        // A real Ranger (not Phrenic Slayer) must not gain these either --
        // this is a genuinely separate mechanism from Ranger's own favored
        // enemy, not a reroute of it.
        let ranger = character("class:ranger", 10);
        for id in all_thirty_two_ids() {
            assert_eq!(
                explanation_value(&ranger, &id),
                None,
                "a Ranger must not gain Phrenic Slayer's own Favored Enemy records: {id}"
            );
        }
    }
}

/// SD-34 wave 46 (`decisions.md §22`'s WAVE 46 UPDATE): six more units on
/// Pathfinder Delver's own remainder (extending `ground_pathfinder_delver_
/// class_features`) plus five new prestige classes (Argent Dramaturge,
/// Horizon Walker, Nature Warden, Rage Prophet, Holy Vindicator, Stalwart
/// Defender) -- the same two-layer discipline every prior wave's own test
/// module in this file established: the pure formula first (including edge
/// cases the fixture cannot exercise), then a reachability test proving
/// every explanation id actually surfaces through the real
/// `build_pilot_headless_receipt` pipeline end to end, plus a negative
/// control proving none of them leak onto an unrelated class.
#[cfg(test)]
mod wave46_registered_prestige_magnitude_formulas_tests {
    use super::{
        argent_dramaturge_argent_performance_dc, argent_dramaturge_argent_performance_rounds,
        argent_dramaturge_dramaturgical_flourish_pool_size, build_pilot_headless_receipt,
        holy_vindicator_stigmata_bonus, horizon_walker_favored_terrain_pool_size,
        horizon_walker_terrain_dominance_pool_size, horizon_walker_terrain_mastery_pool_size,
        nature_warden_companion_bond_level, nature_warden_survivalist_level,
        pathfinder_delver_fortunate_soul_uses_per_day, pathfinder_delver_master_explorer_skill_bonus,
        pathfinder_delver_thrilling_escape_uses_per_day, pathfinder_delver_true_seeing_caster_level,
        pathfinder_delver_vigilant_combatant_initiative_bonus, rage_prophet_mystery_level,
        rage_prophet_ragecaster_level, stalwart_defender_ac_bonus,
        stalwart_defender_damage_reduction, stalwart_defender_defensive_powers_pool_size,
        stalwart_defender_defensive_stance_duration_rounds, CharacterClassLevel, CharacterInput,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// This fixture's own ability scores: STR 16 (+2 human bonus, not
    /// relevant here), DEX 14, CON 14, INT 10, WIS 12, CHA 8 -- the Human
    /// ability-bonus choice targets Strength, so Constitution and Charisma
    /// stay exactly as rolled. Modifiers: CON 14 -> +2, CHA 8 -> -1. Used to
    /// confirm Argent Dramaturge's DC and Stalwart Defender's Defensive
    /// Stance duration against the real pipeline, independent of the direct
    /// formula tests (which exercise other modifier values the fixture
    /// cannot).
    const FIXTURE_CONSTITUTION_MODIFIER: i16 = 2;
    const FIXTURE_CHARISMA_MODIFIER: i16 = -1;

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture must load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn explanation_value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    const ALL_TWENTY_EXPLANATION_IDS: &[&str] = &[
        "class_feature.adventurers_guide.pathfinder_delver.guardbreaker.bonus",
        "class_feature.adventurers_guide.pathfinder_delver.master_explorer.skill_bonus",
        "class_feature.adventurers_guide.pathfinder_delver.thrilling_escape.uses_per_day",
        "class_feature.adventurers_guide.pathfinder_delver.vigilant_combatant.initiative_bonus",
        "class_feature.adventurers_guide.pathfinder_delver.fortunate_soul.uses_per_day",
        "class_feature.adventurers_guide.pathfinder_delver.true_seeing.caster_level",
        "class_feature.adventurers_guide.argent_dramaturge.argent_performance.rounds",
        "class_feature.adventurers_guide.argent_dramaturge.argent_performance.dc",
        "class_feature.adventurers_guide.argent_dramaturge.dramaturgical_flourish.pool_size",
        "class_feature.advanced_players_guide.horizon_walker.favored_terrain.pool_size",
        "class_feature.advanced_players_guide.horizon_walker.terrain_mastery.pool_size",
        "class_feature.advanced_players_guide.horizon_walker.terrain_dominance.pool_size",
        "class_feature.advanced_players_guide.nature_warden.companion_bond.level",
        "class_feature.advanced_players_guide.nature_warden.survivalist.level",
        "class_feature.advanced_players_guide.rage_prophet.rage_prophet_mystery.level",
        "class_feature.advanced_players_guide.rage_prophet.ragecaster.level",
        "class_feature.advanced_players_guide.holy_vindicator.stigmata.bonus",
        "class_feature.advanced_players_guide.stalwart_defender.ac_bonus.dodge_bonus",
        "class_feature.advanced_players_guide.stalwart_defender.damage_reduction.value",
        "class_feature.advanced_players_guide.stalwart_defender.defensive_stance.duration_rounds",
    ];

    // ---- Pathfinder Delver's six-unit extension ----

    #[test]
    fn master_explorer_skill_bonus_matches_the_corpus_token() {
        // `PaDSkillBonus = max(1,CL/2)`, granted level 1 (`ag_classes.lst:285`).
        assert_eq!(pathfinder_delver_master_explorer_skill_bonus(1), Some(1));
        assert_eq!(pathfinder_delver_master_explorer_skill_bonus(2), Some(1));
        assert_eq!(pathfinder_delver_master_explorer_skill_bonus(5), Some(2));
        assert_eq!(pathfinder_delver_master_explorer_skill_bonus(10), Some(5));
    }

    #[test]
    fn thrilling_escape_uses_per_day_matches_the_corpus_token() {
        // Cumulative `PaDEscapeTimes`: +1 at level 3, 7, 9.
        assert_eq!(pathfinder_delver_thrilling_escape_uses_per_day(2), None);
        assert_eq!(pathfinder_delver_thrilling_escape_uses_per_day(3), Some(1));
        assert_eq!(pathfinder_delver_thrilling_escape_uses_per_day(6), Some(1));
        assert_eq!(pathfinder_delver_thrilling_escape_uses_per_day(7), Some(2));
        assert_eq!(pathfinder_delver_thrilling_escape_uses_per_day(8), Some(2));
        assert_eq!(pathfinder_delver_thrilling_escape_uses_per_day(9), Some(3));
        assert_eq!(pathfinder_delver_thrilling_escape_uses_per_day(10), Some(3));
    }

    #[test]
    fn vigilant_combatant_initiative_bonus_matches_the_corpus_token() {
        // `PaDInitiative = CL/2`, granted level 4 (`ag_classes.lst:288`).
        assert_eq!(pathfinder_delver_vigilant_combatant_initiative_bonus(3), None);
        assert_eq!(pathfinder_delver_vigilant_combatant_initiative_bonus(4), Some(2));
        assert_eq!(pathfinder_delver_vigilant_combatant_initiative_bonus(5), Some(2));
        assert_eq!(pathfinder_delver_vigilant_combatant_initiative_bonus(10), Some(5));
    }

    #[test]
    fn fortunate_soul_uses_per_day_matches_the_corpus_token() {
        // Cumulative `PaDFortunateTimes`: +1 at level 6, 10.
        assert_eq!(pathfinder_delver_fortunate_soul_uses_per_day(5), None);
        assert_eq!(pathfinder_delver_fortunate_soul_uses_per_day(6), Some(1));
        assert_eq!(pathfinder_delver_fortunate_soul_uses_per_day(9), Some(1));
        assert_eq!(pathfinder_delver_fortunate_soul_uses_per_day(10), Some(2));
    }

    #[test]
    fn true_seeing_caster_level_matches_the_corpus_token() {
        // Granted level 9 (`ag_classes.lst:292`); `PaDLvl = CL`.
        assert_eq!(pathfinder_delver_true_seeing_caster_level(8), None);
        assert_eq!(pathfinder_delver_true_seeing_caster_level(9), Some(9));
        assert_eq!(pathfinder_delver_true_seeing_caster_level(10), Some(10));
    }

    // ---- Argent Dramaturge ----

    #[test]
    fn argent_performance_rounds_matches_the_corpus_token() {
        // `ArgentPerformanceRounds = ArgentDramaturgeLVL*2`.
        assert_eq!(argent_dramaturge_argent_performance_rounds(1), Some(2));
        assert_eq!(argent_dramaturge_argent_performance_rounds(5), Some(10));
        assert_eq!(argent_dramaturge_argent_performance_rounds(10), Some(20));
    }

    #[test]
    fn argent_performance_dc_matches_the_corpus_token() {
        // `ArgentPerformanceDC = 10+ArgentDramaturgeLVL+CHA`.
        assert_eq!(argent_dramaturge_argent_performance_dc(1, 3), Some(14));
        assert_eq!(argent_dramaturge_argent_performance_dc(1, -1), Some(10));
        assert_eq!(argent_dramaturge_argent_performance_dc(10, 0), Some(20));
    }

    #[test]
    fn dramaturgical_flourish_pool_size_matches_the_corpus_token() {
        // `ArgentDramaturgeLVL/2`.
        assert_eq!(argent_dramaturge_dramaturgical_flourish_pool_size(1), Some(0));
        assert_eq!(argent_dramaturge_dramaturgical_flourish_pool_size(5), Some(2));
        assert_eq!(argent_dramaturge_dramaturgical_flourish_pool_size(10), Some(5));
    }

    // ---- Horizon Walker ----

    #[test]
    fn favored_terrain_pool_size_matches_the_corpus_token() {
        // `FavoredTerrainPool = (2*(HorizonWalkerFavoredTerrainLVL+1))/3`,
        // `HorizonWalkerFavoredTerrainLVL = HorizonWalkerLVL`.
        assert_eq!(horizon_walker_favored_terrain_pool_size(1), Some(1));
        assert_eq!(horizon_walker_favored_terrain_pool_size(5), Some(4));
        assert_eq!(horizon_walker_favored_terrain_pool_size(10), Some(7));
    }

    #[test]
    fn terrain_mastery_pool_size_matches_the_corpus_token() {
        // `HorizonWalkerLVL/2`.
        assert_eq!(horizon_walker_terrain_mastery_pool_size(1), Some(0));
        assert_eq!(horizon_walker_terrain_mastery_pool_size(5), Some(2));
        assert_eq!(horizon_walker_terrain_mastery_pool_size(10), Some(5));
    }

    #[test]
    fn terrain_dominance_pool_size_matches_the_corpus_token() {
        // `HorizonWalkerLVL/3`.
        assert_eq!(horizon_walker_terrain_dominance_pool_size(1), Some(0));
        assert_eq!(horizon_walker_terrain_dominance_pool_size(5), Some(1));
        assert_eq!(horizon_walker_terrain_dominance_pool_size(10), Some(3));
    }

    // ---- Nature Warden ----

    #[test]
    fn companion_bond_and_survivalist_level_are_the_raw_class_level() {
        assert_eq!(nature_warden_companion_bond_level(1), Some(1));
        assert_eq!(nature_warden_companion_bond_level(10), Some(10));
        assert_eq!(nature_warden_survivalist_level(1), Some(1));
        assert_eq!(nature_warden_survivalist_level(10), Some(10));
    }

    // ---- Rage Prophet ----

    #[test]
    fn mystery_and_ragecaster_level_are_the_raw_class_level() {
        assert_eq!(rage_prophet_mystery_level(1), Some(1));
        assert_eq!(rage_prophet_mystery_level(10), Some(10));
        assert_eq!(rage_prophet_ragecaster_level(1), Some(1));
        assert_eq!(rage_prophet_ragecaster_level(10), Some(10));
    }

    // ---- Holy Vindicator ----

    #[test]
    fn stigmata_bonus_matches_the_corpus_token() {
        // `StigmataLVL = floor(HolyVindicatorLVL/2)`.
        assert_eq!(holy_vindicator_stigmata_bonus(1), Some(0));
        assert_eq!(holy_vindicator_stigmata_bonus(5), Some(2));
        assert_eq!(holy_vindicator_stigmata_bonus(10), Some(5));
    }

    // ---- Stalwart Defender ----

    #[test]
    fn ac_bonus_matches_the_corpus_token() {
        // `StalwartDefenderDodgeACBonus = 1+(SDL>=4)+(SDL>=7)+(SDL>=10)`.
        assert_eq!(stalwart_defender_ac_bonus(1), Some(1));
        assert_eq!(stalwart_defender_ac_bonus(3), Some(1));
        assert_eq!(stalwart_defender_ac_bonus(4), Some(2));
        assert_eq!(stalwart_defender_ac_bonus(7), Some(3));
        assert_eq!(stalwart_defender_ac_bonus(10), Some(4));
    }

    #[test]
    fn damage_reduction_matches_the_corpus_token() {
        // `DamageReductionLVL = (SDL>4)+(SDL>6)+(SDL>6)+(SDL>9)+(SDL>9)`,
        // transcribed literally (the `>6`/`>9` terms are each counted twice).
        assert_eq!(stalwart_defender_damage_reduction(4), Some(0));
        assert_eq!(stalwart_defender_damage_reduction(5), Some(1));
        assert_eq!(stalwart_defender_damage_reduction(6), Some(1));
        assert_eq!(stalwart_defender_damage_reduction(7), Some(3));
        assert_eq!(stalwart_defender_damage_reduction(9), Some(3));
        assert_eq!(stalwart_defender_damage_reduction(10), Some(5));
    }

    #[test]
    fn defensive_powers_pool_size_matches_the_corpus_token() {
        // `DefensivePowerLVL = StalwartDefenderLVL/2`.
        assert_eq!(stalwart_defender_defensive_powers_pool_size(1), Some(0));
        assert_eq!(stalwart_defender_defensive_powers_pool_size(5), Some(2));
        assert_eq!(stalwart_defender_defensive_powers_pool_size(10), Some(5));
    }

    #[test]
    fn defensive_stance_duration_matches_the_corpus_token() {
        // `DefensiveStanceDuration = 4+CON+(SDL-1)*2`.
        assert_eq!(stalwart_defender_defensive_stance_duration_rounds(1, 2), Some(6));
        assert_eq!(stalwart_defender_defensive_stance_duration_rounds(1, -1), Some(3));
        assert_eq!(stalwart_defender_defensive_stance_duration_rounds(5, 2), Some(14));
        assert_eq!(stalwart_defender_defensive_stance_duration_rounds(10, 2), Some(24));
    }

    // ---- Reachability: all twenty ids surface through the real pipeline ----

    #[test]
    fn pathfinder_delver_ids_reach_the_real_pipeline_at_level_ten() {
        let character = character("class:pathfinder_delver", 10);
        let expected: &[(&str, i16)] = &[
            // `TrapSenseBonus = RogueTrapSenseLVL/3`, `RogueTrapSenseLVL = PaDLVL+1`:
            // at level 10, (10+1)/3 = 3.
            ("class_feature.adventurers_guide.pathfinder_delver.guardbreaker.bonus", 3),
            ("class_feature.adventurers_guide.pathfinder_delver.master_explorer.skill_bonus", 5),
            (
                "class_feature.adventurers_guide.pathfinder_delver.thrilling_escape.uses_per_day",
                3,
            ),
            (
                "class_feature.adventurers_guide.pathfinder_delver.vigilant_combatant.\
                 initiative_bonus",
                5,
            ),
            ("class_feature.adventurers_guide.pathfinder_delver.fortunate_soul.uses_per_day", 2),
            ("class_feature.adventurers_guide.pathfinder_delver.true_seeing.caster_level", 10),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&character, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn argent_dramaturge_ids_reach_the_real_pipeline_at_level_five() {
        let character = character("class:argent_dramaturge", 5);
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.adventurers_guide.argent_dramaturge.argent_performance.rounds"
            ),
            Some(10)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.adventurers_guide.argent_dramaturge.argent_performance.dc"
            ),
            Some(10 + 5 + FIXTURE_CHARISMA_MODIFIER)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.adventurers_guide.argent_dramaturge.dramaturgical_flourish.\
                 pool_size"
            ),
            Some(2)
        );
    }

    #[test]
    fn horizon_walker_ids_reach_the_real_pipeline_at_level_five() {
        let character = character("class:horizon_walker", 5);
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.horizon_walker.favored_terrain.pool_size"
            ),
            Some(4)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.horizon_walker.terrain_mastery.pool_size"
            ),
            Some(2)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.horizon_walker.terrain_dominance.pool_size"
            ),
            Some(1)
        );
    }

    #[test]
    fn nature_warden_ids_reach_the_real_pipeline_at_level_five() {
        let character = character("class:nature_warden", 5);
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.nature_warden.companion_bond.level"
            ),
            Some(5)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.nature_warden.survivalist.level"
            ),
            Some(5)
        );
    }

    #[test]
    fn rage_prophet_ids_reach_the_real_pipeline_at_level_five() {
        let character = character("class:rage_prophet", 5);
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.rage_prophet.rage_prophet_mystery.level"
            ),
            Some(5)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.rage_prophet.ragecaster.level"
            ),
            Some(5)
        );
    }

    #[test]
    fn holy_vindicator_id_reaches_the_real_pipeline_at_level_five() {
        let character = character("class:holy_vindicator", 5);
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.holy_vindicator.stigmata.bonus"
            ),
            Some(2)
        );
    }

    #[test]
    fn stalwart_defender_ids_reach_the_real_pipeline_at_level_ten() {
        let character = character("class:stalwart_defender", 10);
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.stalwart_defender.ac_bonus.dodge_bonus"
            ),
            Some(4)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.stalwart_defender.damage_reduction.value"
            ),
            Some(5)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.advanced_players_guide.stalwart_defender.defensive_stance.\
                 duration_rounds"
            ),
            Some(4 + FIXTURE_CONSTITUTION_MODIFIER + 9 * 2)
        );
    }

    #[test]
    fn none_of_the_twenty_ids_leak_onto_an_unrelated_class() {
        let fighter = character("class:fighter", 10);
        for id in ALL_TWENTY_EXPLANATION_IDS {
            assert_eq!(
                explanation_value(&fighter, id),
                None,
                "a Fighter must not gain any of this wave's 20 new-class-feature ids: {id}"
            );
        }
    }
}

/// SD-34 wave 47 (`decisions.md §22`'s WAVE 47 UPDATE): Divine Scion's 43
/// magnitude-bearing class features -- the same two-layer discipline every
/// prior wave's own test module in this file established: the pure
/// formula first (including edge cases the fixture cannot exercise), then
/// a reachability test proving every explanation id actually surfaces
/// through the real `build_pilot_headless_receipt` pipeline end to end,
/// plus a negative control proving none of them leak onto an unrelated
/// class.
#[cfg(test)]
mod wave47_divine_scion_class_features_tests {
    use super::{
        build_pilot_headless_receipt, divine_scion_deific_defense_bonus,
        divine_scion_divine_wrath_bonus, divine_scion_domain_specialization_pool_size,
        divine_scion_opposition_alignment_dr,
        divine_scion_weapon_and_armor_proficiency_qualify_flag, total_character_level,
        CharacterClassLevel, CharacterInput, DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID,
        DIVINE_SCION_DOMAIN_SPECIALIZATION_USES_PER_DAY,
        DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID,
    };
    use crate::rules_core::character_input::{SelectedChoice, load_character_input_fixture};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture must load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    /// Same as `character`, but with a recorded Domain Specialization
    /// and/or Opposition Alignment choice -- the CORRECTION this cycle
    /// made necessary: neither the 35 per-domain records nor the 4
    /// per-alignment DR records surface at all without one (see
    /// `ground_divine_scion_class_features`'s own doc comment).
    fn character_with_choices(
        level: u8,
        domain: Option<&str>,
        alignment: Option<&str>,
    ) -> CharacterInput {
        let mut input = character("class:divine_scion", level);
        if let Some(domain) = domain {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID.to_owned(),
                selection_id: format!("domain:{domain}"),
            });
        }
        if let Some(alignment) = alignment {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID.to_owned(),
                selection_id: format!("alignment:{alignment}"),
            });
        }
        input
    }

    fn explanation_value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    const ALL_STATIC_EXPLANATION_IDS: &[&str] = &[
        "class_feature.inner_sea_magic.divine_scion.domain_specialization.pool_size",
        "class_feature.inner_sea_magic.divine_scion.divine_wrath.bonus",
        "class_feature.inner_sea_magic.divine_scion.deific_defense.bonus",
        "class_feature.inner_sea_magic.divine_scion.weapon_and_armor_proficiency.qualify_flag",
        "class_feature.inner_sea_magic.divine_scion.chaotic_opposition_alignment.dr",
        "class_feature.inner_sea_magic.divine_scion.evil_opposition_alignment.dr",
        "class_feature.inner_sea_magic.divine_scion.good_opposition_alignment.dr",
        "class_feature.inner_sea_magic.divine_scion.lawful_opposition_alignment.dr",
        "class_feature.inner_sea_magic.divine_scion.fire_specialization.caster_level",
        "class_feature.inner_sea_magic.divine_scion.fire_specialization.uses_per_day",
        "class_feature.inner_sea_magic.divine_scion.chaos_specialization.caster_level",
    ];

    // ---- Pure formula ----

    #[test]
    fn domain_specialization_pool_size_matches_the_corpus_token() {
        // `BONUS:ABILITYPOOL|Domain Specialization|1`, granted level 3
        // (`ism_classes.lst:104`) -- literally, unconditionally 1.
        assert_eq!(divine_scion_domain_specialization_pool_size(2), None);
        assert_eq!(divine_scion_domain_specialization_pool_size(3), Some(1));
        assert_eq!(divine_scion_domain_specialization_pool_size(10), Some(1));
    }

    #[test]
    fn divine_wrath_bonus_matches_the_corpus_token() {
        // `DivineWrathBonus = 1`, granted level 4 (`ism_classes.lst:105`).
        assert_eq!(divine_scion_divine_wrath_bonus(3), None);
        assert_eq!(divine_scion_divine_wrath_bonus(4), Some(1));
        assert_eq!(divine_scion_divine_wrath_bonus(10), Some(1));
    }

    #[test]
    fn deific_defense_and_opposition_alignment_dr_match_the_corpus_token() {
        // `DeificDefenseBonus = 2`, granted level 7 (`ism_classes.lst:106`);
        // the four Opposition Alignment DR records restate the identical
        // magnitude.
        assert_eq!(divine_scion_deific_defense_bonus(6), None);
        assert_eq!(divine_scion_deific_defense_bonus(7), Some(2));
        assert_eq!(divine_scion_deific_defense_bonus(10), Some(2));
        assert_eq!(divine_scion_opposition_alignment_dr(6), None);
        assert_eq!(divine_scion_opposition_alignment_dr(7), Some(2));
        assert_eq!(divine_scion_opposition_alignment_dr(10), Some(2));
    }

    #[test]
    fn weapon_and_armor_proficiency_qualify_flag_matches_the_corpus_token() {
        // `BONUS:VAR|GreatWeapFocusQualify,WeapSpecQualify,
        // GreatWeapSpecQualify|1`, granted level 1 (`ism_classes.lst:103`).
        assert_eq!(divine_scion_weapon_and_armor_proficiency_qualify_flag(0), None);
        assert_eq!(divine_scion_weapon_and_armor_proficiency_qualify_flag(1), Some(1));
        assert_eq!(divine_scion_weapon_and_armor_proficiency_qualify_flag(10), Some(1));
    }

    #[test]
    fn total_character_level_sums_every_class_not_just_the_swept_one() {
        let mut input = character("class:divine_scion", 3);
        // A multiclassed character: 3 levels of Divine Scion plus 7 of an
        // earlier class -- `TL` must be the SUM (10), not the swept
        // class's own level (3) alone.
        input.chosen.class_levels.push(CharacterClassLevel {
            class_id: "class:cleric".to_owned(),
            level: 7,
        });
        assert_eq!(total_character_level(&input), 10);
    }

    #[test]
    fn domain_specialization_uses_per_day_table_matches_the_real_oracle() {
        // Verified directly against `ism_abilities_class.lst`'s own 35
        // `SPELLS:Innate|TIMES=<1|3|ATWILL>|...` tokens (lines 49-83), not
        // just the ingested corpus JSON.
        assert_eq!(DIVINE_SCION_DOMAIN_SPECIALIZATION_USES_PER_DAY.len(), 35);
        let atwill: Vec<&str> = DIVINE_SCION_DOMAIN_SPECIALIZATION_USES_PER_DAY
            .iter()
            .filter(|(_, _, times)| times.is_none())
            .map(|(slug, _, _)| *slug)
            .collect();
        assert_eq!(atwill, vec!["chaos", "evil", "good", "law", "magic"]);
        let times_of = |slug: &str| {
            DIVINE_SCION_DOMAIN_SPECIALIZATION_USES_PER_DAY
                .iter()
                .find(|(s, _, _)| *s == slug)
                .and_then(|(_, _, times)| *times)
        };
        assert_eq!(times_of("air"), Some(1));
        assert_eq!(times_of("artifice"), Some(3));
        assert_eq!(times_of("scalykind"), Some(1));
        assert_eq!(times_of("war"), Some(3));
    }

    // ---- Reachability: the real pipeline, at level 10 (every gate open) ----

    #[test]
    fn divine_scion_unconditional_static_ids_reach_the_real_pipeline_at_level_ten() {
        // No Domain Specialization/Opposition Alignment choice recorded at all
        // -- these four facts are unconditional single-owner grants, so they
        // still surface (see `ground_divine_scion_class_features`'s own doc
        // comment for why the OTHER 39 do not).
        let character = character("class:divine_scion", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.inner_sea_magic.divine_scion.domain_specialization.pool_size", 1),
            ("class_feature.inner_sea_magic.divine_scion.divine_wrath.bonus", 1),
            ("class_feature.inner_sea_magic.divine_scion.deific_defense.bonus", 2),
            (
                "class_feature.inner_sea_magic.divine_scion.weapon_and_armor_proficiency.\
                 qualify_flag",
                1,
            ),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&character, id), Some(*value), "{id}");
        }
    }

    /// CORRECTION (this cycle): none of the 4 Opposition Alignment DR
    /// records or the 35 Domain Specialization per-domain records surface
    /// for a character with no recorded choice -- the first draft of this
    /// wave wrongly grounded all of them unconditionally. This is the RED
    /// this cycle's own fix turns GREEN.
    #[test]
    fn divine_scion_choice_gated_ids_are_absent_with_no_recorded_selection() {
        let character = character("class:divine_scion", 10);
        for id in [
            "class_feature.inner_sea_magic.divine_scion.chaotic_opposition_alignment.dr",
            "class_feature.inner_sea_magic.divine_scion.evil_opposition_alignment.dr",
            "class_feature.inner_sea_magic.divine_scion.good_opposition_alignment.dr",
            "class_feature.inner_sea_magic.divine_scion.lawful_opposition_alignment.dr",
            "class_feature.inner_sea_magic.divine_scion.fire_specialization.caster_level",
            "class_feature.inner_sea_magic.divine_scion.fire_specialization.uses_per_day",
            "class_feature.inner_sea_magic.divine_scion.chaos_specialization.caster_level",
        ] {
            assert_eq!(
                explanation_value(&character, id),
                None,
                "{id}: a real divine scion has exactly one domain/alignment, never all of them \
                 and never none of them silently defaulted"
            );
        }
    }

    #[test]
    fn divine_scion_opposition_alignment_dr_surfaces_only_the_recorded_alignment() {
        let evil = character_with_choices(10, None, Some("evil"));
        assert_eq!(
            explanation_value(
                &evil,
                "class_feature.inner_sea_magic.divine_scion.evil_opposition_alignment.dr"
            ),
            Some(2)
        );
        for other in [
            "class_feature.inner_sea_magic.divine_scion.chaotic_opposition_alignment.dr",
            "class_feature.inner_sea_magic.divine_scion.good_opposition_alignment.dr",
            "class_feature.inner_sea_magic.divine_scion.lawful_opposition_alignment.dr",
        ] {
            assert_eq!(
                explanation_value(&evil, other),
                None,
                "{other}: recording Evil must not also surface the other three"
            );
        }
    }

    #[test]
    fn divine_scion_domain_caster_level_is_total_character_level_not_class_level() {
        // Single-classed sweep: `TL` == the swept class's own level, 10.
        let character = character_with_choices(10, Some("fire"), None);
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.inner_sea_magic.divine_scion.fire_specialization.caster_level"
            ),
            Some(10)
        );
        // Recording Fire must not also surface Chaos's own caster-level fact.
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.inner_sea_magic.divine_scion.chaos_specialization.caster_level"
            ),
            None
        );
    }

    #[test]
    fn divine_scion_uses_per_day_is_present_for_numeric_times_and_absent_for_atwill() {
        let fire = character_with_choices(10, Some("fire"), None);
        assert_eq!(
            explanation_value(
                &fire,
                "class_feature.inner_sea_magic.divine_scion.fire_specialization.uses_per_day"
            ),
            Some(1)
        );
        let artifice = character_with_choices(10, Some("artifice"), None);
        assert_eq!(
            explanation_value(
                &artifice,
                "class_feature.inner_sea_magic.divine_scion.artifice_specialization.\
                 uses_per_day"
            ),
            Some(3)
        );
        let chaos = character_with_choices(10, Some("chaos"), None);
        assert_eq!(
            explanation_value(
                &chaos,
                "class_feature.inner_sea_magic.divine_scion.chaos_specialization.uses_per_day"
            ),
            None,
            "ATWILL domains ground no uses_per_day fact, the same boundary \
             `paladin_detect_evil` already established"
        );
        // Chaos's OWN caster-level fact still grounds even though it has no
        // uses-per-day fact.
        assert_eq!(
            explanation_value(
                &chaos,
                "class_feature.inner_sea_magic.divine_scion.chaos_specialization.caster_level"
            ),
            Some(10)
        );
    }

    #[test]
    fn divine_scion_ids_are_absent_below_their_own_level_gates() {
        // Level 2: below every gate (3/4/7) except the level-1 proficiency flag.
        // Choices recorded regardless, to isolate the level gate from the
        // choice gate this cycle's correction added.
        let character = character_with_choices(2, Some("fire"), Some("evil"));
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.inner_sea_magic.divine_scion.weapon_and_armor_proficiency.\
                 qualify_flag"
            ),
            Some(1)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.inner_sea_magic.divine_scion.domain_specialization.pool_size"
            ),
            None
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.inner_sea_magic.divine_scion.fire_specialization.caster_level"
            ),
            None
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.inner_sea_magic.divine_scion.divine_wrath.bonus"
            ),
            None
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.inner_sea_magic.divine_scion.deific_defense.bonus"
            ),
            None
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.inner_sea_magic.divine_scion.evil_opposition_alignment.dr"
            ),
            None,
            "level 2 is below Deific Defense's own level-7 gate, regardless of the \
             recorded alignment choice"
        );
    }

    #[test]
    fn none_of_the_static_ids_leak_onto_an_unrelated_class() {
        let mut fighter = character("class:fighter", 10);
        fighter.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID.to_owned(),
            selection_id: "domain:fire".to_owned(),
        });
        fighter.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID.to_owned(),
            selection_id: "alignment:evil".to_owned(),
        });
        for id in ALL_STATIC_EXPLANATION_IDS {
            assert_eq!(
                explanation_value(&fighter, id),
                None,
                "a Fighter must not gain any of Divine Scion's new class-feature ids: {id}, \
                 even with Divine Scion's own choices recorded"
            );
        }
    }
}

/// SD-34 wave 48 (`decisions.md §22`'s WAVE 48 UPDATE): Twilight Talon and
/// Golden Legionnaire's own magnitude-bearing class features. Same overall
/// shape as `wave47_divine_scion_class_features_tests` immediately above --
/// pure-formula tests for every new function, plus real-pipeline
/// reachability tests proving the choice-gated tattoo records surface
/// exactly one member per tier and nothing else.
#[cfg(test)]
mod wave48_registered_prestige_magnitude_formulas_tests {
    use super::{
        build_pilot_headless_receipt, golden_legionnaire_allied_retribution_bonus,
        golden_legionnaire_authoritative_command_bonus, golden_legionnaire_improved_aid_bonus,
        golden_legionnaire_united_defense_bonus, twilight_talon_enhanced_tattoo_save_dc,
        twilight_talon_sneak_attack_dice, CharacterClassLevel, CharacterInput,
        TWILIGHT_TALON_TATTOO_LEVEL_2_CHOICE_ID, TWILIGHT_TALON_TATTOO_LEVEL_4_CHOICE_ID,
    };
    use crate::rules_core::character_input::{SelectedChoice, load_character_input_fixture};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// This fixture's own Charisma modifier -- CHA 8 -> -1, the same
    /// documented constant `wave46_registered_prestige_magnitude_formulas_
    /// tests`'s own `FIXTURE_CHARISMA_MODIFIER` already establishes for the
    /// identical shared fixture.
    const FIXTURE_CHARISMA_MODIFIER: i16 = -1;

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture must load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn twilight_talon_with_tattoos(level: u8, picks: &[(&str, &str)]) -> CharacterInput {
        let mut input = character("class:twilight_talon", level);
        for (choice_set_id, selection_id) in picks {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: (*choice_set_id).to_owned(),
                selection_id: (*selection_id).to_owned(),
            });
        }
        input
    }

    fn explanation_value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    // ---- Pure formula ----

    #[test]
    fn twilight_talon_sneak_attack_dice_matches_the_corpus_token() {
        // `(TwilightTalonLVL+2)/3`, integer division, no `PREVARGTEQ` gate.
        assert_eq!(twilight_talon_sneak_attack_dice(0), None);
        assert_eq!(twilight_talon_sneak_attack_dice(1), Some(1));
        assert_eq!(twilight_talon_sneak_attack_dice(3), Some(1));
        assert_eq!(twilight_talon_sneak_attack_dice(4), Some(2));
        assert_eq!(twilight_talon_sneak_attack_dice(10), Some(4));
    }

    #[test]
    fn twilight_talon_enhanced_tattoo_save_dc_matches_the_corpus_token() {
        // `10+TwilightTalonLVL/2+CHA`, no `PREVARGTEQ` gate.
        assert_eq!(twilight_talon_enhanced_tattoo_save_dc(0, 3), None);
        assert_eq!(twilight_talon_enhanced_tattoo_save_dc(2, 3), Some(14));
        assert_eq!(twilight_talon_enhanced_tattoo_save_dc(10, 3), Some(18));
        assert_eq!(twilight_talon_enhanced_tattoo_save_dc(10, -1), Some(14));
    }

    #[test]
    fn golden_legionnaire_step_bonuses_match_the_corpus_tokens() {
        // `1+(GoldenLegionnaireLVL>=7)`.
        assert_eq!(golden_legionnaire_allied_retribution_bonus(0), None);
        assert_eq!(golden_legionnaire_allied_retribution_bonus(1), Some(1));
        assert_eq!(golden_legionnaire_allied_retribution_bonus(6), Some(1));
        assert_eq!(golden_legionnaire_allied_retribution_bonus(7), Some(2));
        // `1+(GoldenLegionnaireLVL>=6)`.
        assert_eq!(golden_legionnaire_authoritative_command_bonus(5), Some(1));
        assert_eq!(golden_legionnaire_authoritative_command_bonus(6), Some(2));
        // `1+(GoldenLegionnaireLVL>=9)`.
        assert_eq!(golden_legionnaire_improved_aid_bonus(8), Some(1));
        assert_eq!(golden_legionnaire_improved_aid_bonus(9), Some(2));
        // `1+(GoldenLegionnaireLVL>=6)+(GoldenLegionnaireLVL>=10)`.
        assert_eq!(golden_legionnaire_united_defense_bonus(5), Some(1));
        assert_eq!(golden_legionnaire_united_defense_bonus(6), Some(2));
        assert_eq!(golden_legionnaire_united_defense_bonus(10), Some(3));
    }

    // ---- Reachability: the real pipeline ----

    #[test]
    fn twilight_talon_unconditional_ids_reach_the_real_pipeline_at_level_ten() {
        let character = character("class:twilight_talon", 10);
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.adventurers_guide.twilight_talon.sneak_attack.dice"
            ),
            Some(4)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.adventurers_guide.twilight_talon.enhanced_tattoo.save_dc"
            ),
            Some(15 + FIXTURE_CHARISMA_MODIFIER)
        );
    }

    /// CORRECTION-shaped RED-for-the-right-reason: none of the 10 choice-
    /// gated tattoo caster-level facts surface for a character with no
    /// recorded tier selections -- the same discipline wave 47's own
    /// correction established for Divine Scion.
    #[test]
    fn twilight_talon_tattoo_ids_are_absent_with_no_recorded_selection() {
        let character = character("class:twilight_talon", 10);
        for id in [
            "class_feature.adventurers_guide.twilight_talon.disguise_self.caster_level",
            "class_feature.adventurers_guide.twilight_talon.alter_self.caster_level",
            "class_feature.adventurers_guide.twilight_talon.glibness.caster_level",
            "class_feature.adventurers_guide.twilight_talon.modify_memory.caster_level",
            "class_feature.adventurers_guide.twilight_talon.mislead.caster_level",
        ] {
            assert_eq!(
                explanation_value(&character, id),
                None,
                "{id}: a real twilight talon has no tattoo at all until a tier selection is \
                 recorded, never all candidates defaulted"
            );
        }
    }

    #[test]
    fn twilight_talon_tattoo_surfaces_only_the_recorded_member_per_tier() {
        let character = twilight_talon_with_tattoos(
            10,
            &[
                (TWILIGHT_TALON_TATTOO_LEVEL_2_CHOICE_ID, "tattoo:disguise_self"),
                (TWILIGHT_TALON_TATTOO_LEVEL_4_CHOICE_ID, "tattoo:invisibility"),
            ],
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.adventurers_guide.twilight_talon.disguise_self.caster_level"
            ),
            Some(10)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.adventurers_guide.twilight_talon.invisibility.caster_level"
            ),
            Some(10)
        );
        // Recording Disguise Self at tier 2 must not also surface tier 2's
        // OWN sibling (Undetectable Alignment), and recording Invisibility
        // at tier 4 must not surface tier 4's own sibling (Alter Self).
        for absent in [
            "class_feature.adventurers_guide.twilight_talon.undetectable_alignment.\
             caster_level",
            "class_feature.adventurers_guide.twilight_talon.alter_self.caster_level",
        ] {
            assert_eq!(explanation_value(&character, absent), None, "{absent}");
        }
        // Tiers 6/8/10 were never recorded at all -- absent too.
        for absent in [
            "class_feature.adventurers_guide.twilight_talon.glibness.caster_level",
            "class_feature.adventurers_guide.twilight_talon.modify_memory.caster_level",
            "class_feature.adventurers_guide.twilight_talon.mislead.caster_level",
        ] {
            assert_eq!(explanation_value(&character, absent), None, "{absent}");
        }
    }

    #[test]
    fn twilight_talon_tattoo_ids_are_absent_below_their_own_tier_gate() {
        // Level 3: tier 2 is open, tiers 4/6/8/10 are not -- even though a
        // (premature) tier-4 selection is recorded, it must not surface.
        let character = twilight_talon_with_tattoos(
            3,
            &[
                (TWILIGHT_TALON_TATTOO_LEVEL_2_CHOICE_ID, "tattoo:disguise_self"),
                (TWILIGHT_TALON_TATTOO_LEVEL_4_CHOICE_ID, "tattoo:alter_self"),
            ],
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.adventurers_guide.twilight_talon.disguise_self.caster_level"
            ),
            Some(3)
        );
        assert_eq!(
            explanation_value(
                &character,
                "class_feature.adventurers_guide.twilight_talon.alter_self.caster_level"
            ),
            None,
            "tier 4 is gated below level 4, regardless of the recorded selection"
        );
    }

    #[test]
    fn golden_legionnaire_unconditional_ids_reach_the_real_pipeline_at_level_ten() {
        let character = character("class:golden_legionnaire", 10);
        let expected: &[(&str, i16)] = &[
            (
                "class_feature.adventurers_guide.golden_legionnaire.allied_retribution.bonus",
                2,
            ),
            (
                "class_feature.adventurers_guide.golden_legionnaire.authoritative_command.\
                 bonus",
                2,
            ),
            ("class_feature.adventurers_guide.golden_legionnaire.improved_aid.bonus", 2),
            ("class_feature.adventurers_guide.golden_legionnaire.united_defense.bonus", 3),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&character, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn none_of_the_wave48_ids_leak_onto_an_unrelated_class() {
        let mut fighter = character("class:fighter", 10);
        fighter.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: TWILIGHT_TALON_TATTOO_LEVEL_2_CHOICE_ID.to_owned(),
            selection_id: "tattoo:disguise_self".to_owned(),
        });
        for id in [
            "class_feature.adventurers_guide.twilight_talon.sneak_attack.dice",
            "class_feature.adventurers_guide.twilight_talon.enhanced_tattoo.save_dc",
            "class_feature.adventurers_guide.twilight_talon.disguise_self.caster_level",
            "class_feature.adventurers_guide.golden_legionnaire.allied_retribution.bonus",
            "class_feature.adventurers_guide.golden_legionnaire.authoritative_command.bonus",
            "class_feature.adventurers_guide.golden_legionnaire.improved_aid.bonus",
            "class_feature.adventurers_guide.golden_legionnaire.united_defense.bonus",
        ] {
            assert_eq!(
                explanation_value(&fighter, id),
                None,
                "a Fighter must not gain any of Twilight Talon/Golden Legionnaire's new \
                 class-feature ids: {id}"
            );
        }
    }
}

/// SD-34 wave 49 (`decisions.md §22`'s WAVE 49 UPDATE): 33 more registered
/// prestige classes' magnitude-only remainder. Same two-layer discipline as
/// every prior wave's own test module in this file: pure-formula edge-case
/// tests for the trickiest formulas (floor/negative-clamped values, the
/// ability-modifier gate on Student of War's Mind Over Metal), then one
/// reachability test per class proving every new explanation id actually
/// surfaces through the real `build_pilot_headless_receipt` pipeline at a
/// representative level, plus one negative control proving none of the new
/// ids leak onto an unrelated class (Fighter). Given this wave's breadth
/// (33 classes), reachability tests double as the primary formula check --
/// their expected values were independently derived (not copied from the
/// production code) before being transcribed here.
#[cfg(test)]
mod wave49_registered_prestige_magnitude_formulas_tests {
    use super::{
        build_pilot_headless_receipt, dark_tempest_blade_skills_pool_size,
        mystic_archer_heightened_senses_range, pyrokineticist_leech_heat_bonus,
        pyrokineticist_nimbus_bonus, student_of_war_mind_over_metal_ac_bonus,
        CharacterClassLevel, CharacterInput,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// This fixture's own ability modifiers (matches every prior wave's own
    /// documented constants for the identical shared fixture): STR +3
    /// (human bonus, not used here), DEX +2, CON +2, INT +0, WIS +1,
    /// CHA -1.
    const FIXTURE_CHARISMA_MODIFIER: i16 = -1;

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture must load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn explanation_value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    // ---- Pure-formula edge cases ----

    #[test]
    fn pyrokineticist_leech_heat_is_none_below_the_level_it_turns_positive() {
        assert_eq!(pyrokineticist_leech_heat_bonus(3), None);
        assert_eq!(pyrokineticist_leech_heat_bonus(4), None);
        assert_eq!(pyrokineticist_leech_heat_bonus(6), Some(2));
        assert_eq!(pyrokineticist_leech_heat_bonus(10), Some(4));
    }

    #[test]
    fn pyrokineticist_nimbus_bonus_is_none_below_the_level_it_turns_positive() {
        assert_eq!(pyrokineticist_nimbus_bonus(2), None);
        assert_eq!(pyrokineticist_nimbus_bonus(4), None);
        assert_eq!(pyrokineticist_nimbus_bonus(5), Some(2));
        assert_eq!(pyrokineticist_nimbus_bonus(10), Some(4));
    }

    #[test]
    fn dark_tempest_blade_skills_pool_size_is_none_below_level_five() {
        assert_eq!(dark_tempest_blade_skills_pool_size(4), None);
        assert_eq!(dark_tempest_blade_skills_pool_size(5), Some(1));
        assert_eq!(dark_tempest_blade_skills_pool_size(10), Some(2));
    }

    #[test]
    fn mystic_archer_heightened_senses_range_is_none_below_level_three() {
        assert_eq!(mystic_archer_heightened_senses_range(2), None);
        assert_eq!(mystic_archer_heightened_senses_range(3), Some(5));
        assert_eq!(mystic_archer_heightened_senses_range(7), Some(25));
    }

    #[test]
    fn student_of_war_mind_over_metal_requires_int_above_dex() {
        assert_eq!(student_of_war_mind_over_metal_ac_bonus(0, 2), None);
        assert_eq!(student_of_war_mind_over_metal_ac_bonus(2, 2), None);
        assert_eq!(student_of_war_mind_over_metal_ac_bonus(4, 2), Some(2));
    }

    // ---- Reachability: one test per class, at a representative level ----

    #[test]
    fn cyphermage_ids_reach_the_real_pipeline() {
        let c = character("class:cyphermage", 10);
        assert_eq!(
            explanation_value(&c, "class_feature.inner_sea_magic.cyphermage.analyze_scroll.bonus"),
            Some(10)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.adventurers_guide.cyphermage.cypher_lore.pool_size"
            ),
            Some(9)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.inner_sea_magic.cyphermage.cypher_lore.pool_size"
            ),
            Some(10)
        );
    }

    #[test]
    fn psychic_fist_ids_reach_the_real_pipeline() {
        let c = character("class:psychic_fist", 10);
        assert_eq!(
            explanation_value(&c, "class_feature.ultimate_psionics.psychic_fist.infused_body.bonus"),
            Some(3)
        );
        assert_eq!(
            explanation_value(&c, "class_feature.ultimate_psionics.psychic_fist.ki_power.bonus"),
            Some(5)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.ultimate_psionics.psychic_fist.mesmerizing_glow.targets"
            ),
            Some(5)
        );
    }

    #[test]
    fn asavir_ids_reach_the_real_pipeline() {
        let c = character("class:asavir", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.adventurers_guide.asavir.camaraderie.bonus", 3),
            ("class_feature.adventurers_guide.asavir.djinnis_blessing.bonus", 20),
            ("class_feature.adventurers_guide.asavir.djinnis_blessing_mount.move_bonus", 20),
            (
                "class_feature.adventurers_guide.asavir.efreeti_blessing_mount.fire_resistance",
                5,
            ),
            ("class_feature.adventurers_guide.asavir.equine_bond.companion_level", 12),
            ("class_feature.adventurers_guide.asavir.jannis_blessing.luck_save", 1),
            ("class_feature.adventurers_guide.asavir.jannis_blessing_mount.luck_save", 1),
            ("class_feature.adventurers_guide.asavir.marids_blessing_mount.reflex_save", 2),
            ("class_feature.adventurers_guide.asavir.shaitans_blessing.bonus", 4),
            ("class_feature.adventurers_guide.asavir.thunderous_charge.bonus", 20),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn metamorph_ids_reach_the_real_pipeline() {
        let c = character("class:metamorph", 10);
        assert_eq!(
            explanation_value(&c, "class_feature.ultimate_psionics.metamorph.alter_metamorphosis.level"),
            Some(10)
        );
        assert_eq!(
            explanation_value(&c, "class_feature.ultimate_psionics.metamorph.free_shift.times"),
            Some(5)
        );
        assert_eq!(
            explanation_value(&c, "class_feature.ultimate_psionics.metamorph.natural_shifter.bonus"),
            Some(2)
        );
    }

    #[test]
    fn war_mind_ids_reach_the_real_pipeline() {
        let c = character("class:war_mind", 10);
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.ultimate_psionics.war_mind.chain_of_defensive_posture.bonus"
            ),
            Some(4)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.ultimate_psionics.war_mind.chain_of_personal_superiority.bonus"
            ),
            Some(4)
        );
        assert_eq!(
            explanation_value(&c, "class_feature.ultimate_psionics.war_mind.enduring_body.bonus"),
            Some(3)
        );
    }

    #[test]
    fn hellknight_ids_reach_the_real_pipeline() {
        let c = character("class:hellknight", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.adventurers_guide.hellknight.detect_chaos.dc", 10),
            ("class_feature.adventurers_guide.hellknight.discern_lies.uses_per_day", 2),
            ("class_feature.adventurers_guide.hellknight.discern_lies.dc", 13),
            ("class_feature.adventurers_guide.hellknight.smite_chaos.uses_per_day", 4),
            ("class_feature.adventurers_guide.hellknight.smite_chaos.attack_bonus", 0),
            ("class_feature.adventurers_guide.hellknight.smite_chaos.damage_bonus", 10),
            ("class_feature.adventurers_guide.hellknight.smite_chaos.deflection_bonus", 0),
            ("class_feature.adventurers_guide.hellknight.hellknight_armor.bonus", 3),
            ("class_feature.inner_sea_world_guide.hellknight.hellknight_armor.bonus", 3),
            ("class_feature.adventurers_guide.hellknight.hellknight_armor_benefits.bonus", 3),
            (
                "class_feature.inner_sea_world_guide.hellknight.hellknight_armor_benefits.bonus",
                3,
            ),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn adaptive_warrior_ids_reach_the_real_pipeline() {
        let c = character("class:adaptive_warrior", 10);
        let expected: &[(&str, i16)] = &[
            (
                "class_feature.ultimate_psionics.adaptive_warrior.combine_fighting_styles.\
                 times_per_day",
                4,
            ),
            (
                "class_feature.ultimate_psionics.adaptive_warrior.counter_fighting_style.bonus",
                5,
            ),
            ("class_feature.ultimate_psionics.adaptive_warrior.examine_technique.targets", 10),
            (
                "class_feature.ultimate_psionics.adaptive_warrior.extended_examination.bonus",
                10,
            ),
            ("class_feature.ultimate_psionics.adaptive_warrior.mimic_skill.ranks", 10),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn sanguine_angel_ids_reach_the_real_pipeline() {
        let c = character("class:sanguine_angel", 10);
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.adventurers_guide.sanguine_angel.armored_angel.level"
            ),
            Some(10)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.adventurers_guide.sanguine_angel.mystique_of_ardad_lili.caster_\
                 level"
            ),
            Some(10)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.adventurers_guide.sanguine_angel.mystique_of_ardad_lili.dc"
            ),
            Some(10 + 10 / 2 + FIXTURE_CHARISMA_MODIFIER)
        );
    }

    #[test]
    fn body_snatcher_ids_reach_the_real_pipeline() {
        let c = character("class:body_snatcher", 10);
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.ultimate_psionics.body_snatcher.body_thief.caster_level_bonus"
            ),
            Some(10)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.ultimate_psionics.body_snatcher.death_is_only_the_beginning.\
                 caster_level_bonus"
            ),
            Some(10)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.ultimate_psionics.body_snatcher.melding_exchange.bonus"
            ),
            Some(20)
        );
    }

    #[test]
    fn steel_falcon_ids_reach_the_real_pipeline() {
        let c = character("class:steel_falcon", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.adventurers_guide.steel_falcon.chainbreaker.bonus", 10),
            ("class_feature.adventurers_guide.steel_falcon.enemy_of_slavers.bonus", 6),
            ("class_feature.adventurers_guide.steel_falcon.sailor_and_survivalist.bonus", 10),
            (
                "class_feature.adventurers_guide.steel_falcon.talmandor_s_blessing.acrobatics_\
                 bonus",
                10,
            ),
            (
                "class_feature.adventurers_guide.steel_falcon.talmandor_s_blessing.perception_\
                 bonus",
                4,
            ),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn lantern_bearer_ids_reach_the_real_pipeline() {
        let c = character("class:lantern_bearer", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.adventurers_guide.lantern_bearer.favored_enemy.pool_size", 2),
            ("class_feature.adventurers_guide.lantern_bearer.favored_enemy.bonus_pool_size", 1),
            (
                "class_feature.adventurers_guide.lantern_bearer.proven_weapon_familiarity.bonus",
                1,
            ),
            (
                "class_feature.adventurers_guide.lantern_bearer.superior_discernment.pool_size",
                1,
            ),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn storm_kindler_ids_reach_the_real_pipeline() {
        let c = character("class:storm_kindler", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.adventurers_guide.storm_kindler.aura_of_calm.radius", 20),
            ("class_feature.adventurers_guide.storm_kindler.aura_of_calm.bonus", 4),
            ("class_feature.adventurers_guide.storm_kindler.oceanic_spirit.bonus", 20),
            ("class_feature.adventurers_guide.storm_kindler.storm_shape.height", 60),
            ("class_feature.adventurers_guide.storm_kindler.weathers_fury.bonus", 5),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn westcrown_devil_ids_reach_the_real_pipeline() {
        let c = character("class:westcrown_devil", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.adventurers_guide.westcrown_devil.council_s_secret.pool_size", 5),
            ("class_feature.adventurers_guide.westcrown_devil.founders_favor.pool", 11),
            ("class_feature.adventurers_guide.westcrown_devil.founders_favor.dc", 16),
            ("class_feature.adventurers_guide.westcrown_devil.sneak_attack.dice", 3),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn pyrokineticist_ids_reach_the_real_pipeline() {
        let c = character("class:pyrokineticist", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.ultimate_psionics.pyrokineticist.bolt_of_fire.bonus", 10),
            ("class_feature.ultimate_psionics.pyrokineticist.fire_adaptation.bonus", 8),
            (
                "class_feature.ultimate_psionics.pyrokineticist.fire_adaptation.fire_resistance",
                20,
            ),
            ("class_feature.ultimate_psionics.pyrokineticist.hand_afire.bonus", 4),
            ("class_feature.ultimate_psionics.pyrokineticist.leech_heat.bonus", 4),
            ("class_feature.ultimate_psionics.pyrokineticist.manipulate_blaze.range", 50),
            ("class_feature.ultimate_psionics.pyrokineticist.nimbus.duration_rounds", 10),
            ("class_feature.ultimate_psionics.pyrokineticist.nimbus.bonus", 4),
            ("class_feature.ultimate_psionics.pyrokineticist.penetrating_fire.bonus", 10),
            ("class_feature.ultimate_psionics.pyrokineticist.weapon_afire.bonus", 4),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn aspis_agent_ids_reach_the_real_pipeline() {
        let c = character("class:aspis_agent", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.adventurers_guide.aspis_agent.agency_secrets.pool_size", 5),
            ("class_feature.adventurers_guide.aspis_agent.sneak_attack.dice", 2),
            ("class_feature.adventurers_guide.aspis_agent.trap_sense.bonus", 4),
            ("class_feature.adventurers_guide.aspis_agent.trapfinding.bonus", 5),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn gray_corsair_ids_reach_the_real_pipeline() {
        let c = character("class:gray_corsair", 10);
        assert_eq!(
            explanation_value(&c, "class_feature.adventurers_guide.gray_corsair.favored_port.bonus"),
            Some(2)
        );
        assert_eq!(
            explanation_value(&c, "class_feature.adventurers_guide.gray_corsair.slaver_slayer.bonus"),
            Some(6)
        );
    }

    #[test]
    fn pathfinder_savant_ids_reach_the_real_pipeline() {
        let c = character("class:pathfinder_savant", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.adventurers_guide.pathfinder_savant.master_scholar.bonus", 5),
            ("class_feature.adventurers_guide.pathfinder_savant.esoteric_magic.pool_size", 9),
            (
                "class_feature.adventurers_guide.pathfinder_savant.quick_identification.times_\
                 per_day",
                5,
            ),
            ("class_feature.adventurers_guide.pathfinder_savant.sigil_master.save_bonus", 10),
            (
                "class_feature.adventurers_guide.pathfinder_savant.analyze_dweomer.times_per_\
                 day",
                10,
            ),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn rivethun_emissary_ids_reach_the_real_pipeline() {
        let c = character("class:rivethun_emissary", 10);
        let expected: &[(&str, i16)] = &[
            (
                "class_feature.adventurers_guide.rivethun_emissary.enhanced_spirit_animal.\
                 evolution_points",
                3,
            ),
            ("class_feature.adventurers_guide.rivethun_emissary.parley.uses_per_day", 3),
            (
                "class_feature.adventurers_guide.rivethun_emissary.parley.dc",
                12 + FIXTURE_CHARISMA_MODIFIER,
            ),
            ("class_feature.adventurers_guide.rivethun_emissary.sixth_sense.uses_per_day", 10),
            ("class_feature.adventurers_guide.rivethun_emissary.spirit_animal.level", 10),
            ("class_feature.adventurers_guide.rivethun_emissary.spirit_bond.hex_dc", 5),
            (
                "class_feature.adventurers_guide.rivethun_emissary.spirit_bond.hex_ability_\
                 level",
                10,
            ),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn student_of_war_additional_skill_reaches_the_real_pipeline() {
        let c = character("class:student_of_war", 10);
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.adventurers_guide.student_of_war.additional_skill.pool_size"
            ),
            Some(5)
        );
        // Mind Over Metal does not fire on this fixture: its own Intelligence
        // modifier (+0) does not exceed Dexterity (+2).
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.adventurers_guide.student_of_war.mind_over_metal.ac_bonus"
            ),
            None
        );
    }

    #[test]
    fn diabolist_ids_reach_the_real_pipeline() {
        let c = character("class:diabolist", 10);
        let expected: &[(&str, i16)] = &[
            (
                "class_feature.book_of_the_damned_volume_1.diabolist.channel_hellfire.times_\
                 per_day",
                1,
            ),
            (
                "class_feature.book_of_the_damned_volume_1.diabolist.infernal_transport.times_\
                 per_day",
                2,
            ),
            ("class_feature.book_of_the_damned_volume_1.diabolist.damned.dc", 20),
            ("class_feature.book_of_the_damned_volume_1.diabolist.infernal_charisma.bonus", 8),
            ("class_feature.book_of_the_damned_volume_1.diabolist.heresy.bonus", 4),
            (
                "class_feature.book_of_the_damned_volume_1.diabolist.hellfire_ray.caster_level",
                10,
            ),
            ("class_feature.book_of_the_damned_volume_1.diabolist.hellfire_ray.dc", 15),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn lion_blade_ids_reach_the_real_pipeline() {
        let c = character("class:lion_blade", 10);
        let expected: &[(&str, i16)] = &[
            (
                "class_feature.inner_sea_intrigue.lion_blade.expeditious_advance.speed_bonus",
                10,
            ),
            ("class_feature.inner_sea_intrigue.lion_blade.silent_soul.stealth_bonus", 10),
            ("class_feature.inner_sea_intrigue.lion_blade.sneak_attack.dice", 3),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn bellflower_tiller_ids_reach_the_real_pipeline() {
        let c = character("class:bellflower_tiller", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.adventurers_guide.bellflower_tiller.bellflower_crop.range", 60),
            ("class_feature.adventurers_guide.bellflower_tiller.crop_guardian.bonus", 10),
            ("class_feature.adventurers_guide.bellflower_tiller.sneak_attack.dice", 3),
            ("class_feature.adventurers_guide.bellflower_tiller.swift_sower.speed_bonus", 20),
            ("class_feature.adventurers_guide.bellflower_tiller.teamwork_feat.pool_size", 3),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn hellknight_signifer_ids_reach_the_real_pipeline() {
        let c = character("class:hellknight_signifer", 10);
        let expected: &[(&str, i16)] = &[
            (
                "class_feature.adventurers_guide.hellknight_signifer.assiduous_gaze.pool_size",
                2,
            ),
            ("class_feature.adventurers_guide.hellknight_signifer.signifer_mask.bonus", 2),
            (
                "class_feature.adventurers_guide.hellknight_signifer.infernal_resilience.dr",
                5,
            ),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn mystic_archer_ids_reach_the_real_pipeline() {
        let c = character("class:mystic_archer", 7);
        let expected: &[(&str, i16)] = &[
            ("class_feature.ultimate_psionics.mystic_archer.heightened_senses.range", 25),
            ("class_feature.ultimate_psionics.mystic_archer.blindsense.range", 55),
            ("class_feature.ultimate_psionics.mystic_archer.blindsight.range", 55),
            ("class_feature.ultimate_psionics.mystic_archer.tremorsense.range", 55),
            ("class_feature.ultimate_psionics.mystic_archer.inevitable_strike.uses_per_day", 4),
            ("class_feature.ultimate_psionics.mystic_archer.ranged_sneak_attack.dice", 2),
            ("class_feature.ultimate_psionics.mystic_archer.unhindered_vision.uses_per_day", 1),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn mammoth_rider_ids_reach_the_real_pipeline() {
        let c = character("class:mammoth_rider", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.adventurers_guide.mammoth_rider.born_survivor.pool_size", 3),
            ("class_feature.adventurers_guide.mammoth_rider.gigantic_steed.qualify_flag", 1),
            ("class_feature.adventurers_guide.mammoth_rider.rugged_steed.qualify_flag", 1),
            ("class_feature.adventurers_guide.mammoth_rider.steeds_reach.qualify_flag", 1),
            ("class_feature.adventurers_guide.mammoth_rider.steed.companion_level", 10),
            ("class_feature.adventurers_guide.mammoth_rider.wild_coercion.level", 10),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn demoniac_ids_reach_the_real_pipeline() {
        let c = character("class:demoniac", 10);
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.book_of_the_damned_volume_2.demoniac.summon_demon_i.caster_level"
            ),
            Some(9)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.book_of_the_damned_volume_2.demoniac.summon_demon_ii.caster_\
                 level"
            ),
            Some(9)
        );
    }

    #[test]
    fn master_chymist_ids_reach_the_real_pipeline() {
        let c = character("class:master_chymist", 10);
        let expected: &[(&str, i16)] = &[
            (
                "class_feature.advanced_players_guide.master_chymist.advanced_mutagen.pool_\
                 size",
                5,
            ),
            ("class_feature.advanced_players_guide.master_chymist.bomb_thrower.level", 10),
            ("class_feature.advanced_players_guide.master_chymist.brutality.bonus", 6),
            (
                "class_feature.advanced_players_guide.master_chymist.extracts_per_day.level",
                10,
            ),
            ("class_feature.advanced_players_guide.master_chymist.mutate.times_per_day", 5),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn enchanting_courtesan_ids_reach_the_real_pipeline() {
        let c = character("class:enchanting_courtesan", 10);
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.inner_sea_intrigue.enchanting_courtesan.hidden_spell.count"
            ),
            Some(3)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.inner_sea_intrigue.enchanting_courtesan.seductive_intuition.\
                 bonus"
            ),
            Some(5)
        );
    }

    #[test]
    fn dark_tempest_ids_reach_the_real_pipeline() {
        let c = character("class:dark_tempest", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.ultimate_psionics.dark_tempest.blade_skills.pool_size", 2),
            ("class_feature.ultimate_psionics.dark_tempest.diverse_training.level", 10),
            ("class_feature.ultimate_psionics.dark_tempest.expanded_power_list.pool_size", 1),
            ("class_feature.ultimate_psionics.dark_tempest.power_strike.power_level", 3),
            ("class_feature.ultimate_psionics.dark_tempest.psychic_strike.dice", 4),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn battle_herald_ids_reach_the_real_pipeline() {
        let c = character("class:battle_herald", 10);
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.advanced_players_guide.battle_herald.inspiring_command.level"
            ),
            Some(10)
        );
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.advanced_players_guide.battle_herald.teamwork_feat.pool_size"
            ),
            Some(1)
        );
    }

    #[test]
    fn master_spy_ids_reach_the_real_pipeline() {
        let c = character("class:master_spy", 10);
        let expected: &[(&str, i16)] = &[
            ("class_feature.advanced_players_guide.master_spy.art_of_deception.bonus", 10),
            ("class_feature.advanced_players_guide.master_spy.slippery_mind.times", 1),
            ("class_feature.advanced_players_guide.master_spy.sneak_attack.dice", 4),
        ];
        for (id, value) in expected {
            assert_eq!(explanation_value(&c, id), Some(*value), "{id}");
        }
    }

    #[test]
    fn evangelist_id_reaches_the_real_pipeline() {
        let c = character("class:evangelist", 10);
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.ultimate_combat.evangelist.single_minded.domain_count_delta"
            ),
            Some(-1)
        );
    }

    #[test]
    fn ulfen_guard_id_reaches_the_real_pipeline() {
        let c = character("class:ulfen_guard", 10);
        assert_eq!(
            explanation_value(
                &c,
                "class_feature.inner_sea_combat.ulfen_guard.guard_dedications.pool_size"
            ),
            Some(5)
        );
    }

    // ---- Negative control: none of this wave's ids leak onto Fighter ----

    const ALL_WAVE_49_EXPLANATION_IDS: &[&str] = &[
        "class_feature.inner_sea_magic.cyphermage.analyze_scroll.bonus",
        "class_feature.adventurers_guide.cyphermage.cypher_lore.pool_size",
        "class_feature.inner_sea_magic.cyphermage.cypher_lore.pool_size",
        "class_feature.ultimate_psionics.psychic_fist.infused_body.bonus",
        "class_feature.ultimate_psionics.psychic_fist.ki_power.bonus",
        "class_feature.ultimate_psionics.psychic_fist.mesmerizing_glow.targets",
        "class_feature.adventurers_guide.asavir.camaraderie.bonus",
        "class_feature.adventurers_guide.asavir.djinnis_blessing.bonus",
        "class_feature.adventurers_guide.asavir.djinnis_blessing_mount.move_bonus",
        "class_feature.adventurers_guide.asavir.efreeti_blessing_mount.fire_resistance",
        "class_feature.adventurers_guide.asavir.equine_bond.companion_level",
        "class_feature.adventurers_guide.asavir.jannis_blessing.luck_save",
        "class_feature.adventurers_guide.asavir.jannis_blessing_mount.luck_save",
        "class_feature.adventurers_guide.asavir.marids_blessing_mount.reflex_save",
        "class_feature.adventurers_guide.asavir.shaitans_blessing.bonus",
        "class_feature.adventurers_guide.asavir.thunderous_charge.bonus",
        "class_feature.ultimate_psionics.metamorph.alter_metamorphosis.level",
        "class_feature.ultimate_psionics.metamorph.free_shift.times",
        "class_feature.ultimate_psionics.metamorph.natural_shifter.bonus",
        "class_feature.ultimate_psionics.war_mind.chain_of_defensive_posture.bonus",
        "class_feature.ultimate_psionics.war_mind.chain_of_personal_superiority.bonus",
        "class_feature.ultimate_psionics.war_mind.enduring_body.bonus",
        "class_feature.adventurers_guide.hellknight.detect_chaos.dc",
        "class_feature.adventurers_guide.hellknight.discern_lies.uses_per_day",
        "class_feature.adventurers_guide.hellknight.discern_lies.dc",
        "class_feature.adventurers_guide.hellknight.smite_chaos.uses_per_day",
        "class_feature.adventurers_guide.hellknight.smite_chaos.attack_bonus",
        "class_feature.adventurers_guide.hellknight.smite_chaos.damage_bonus",
        "class_feature.adventurers_guide.hellknight.smite_chaos.deflection_bonus",
        "class_feature.adventurers_guide.hellknight.hellknight_armor.bonus",
        "class_feature.inner_sea_world_guide.hellknight.hellknight_armor.bonus",
        "class_feature.adventurers_guide.hellknight.hellknight_armor_benefits.bonus",
        "class_feature.inner_sea_world_guide.hellknight.hellknight_armor_benefits.bonus",
        "class_feature.ultimate_psionics.adaptive_warrior.combine_fighting_styles.times_per_day",
        "class_feature.ultimate_psionics.adaptive_warrior.counter_fighting_style.bonus",
        "class_feature.ultimate_psionics.adaptive_warrior.examine_technique.targets",
        "class_feature.ultimate_psionics.adaptive_warrior.extended_examination.bonus",
        "class_feature.ultimate_psionics.adaptive_warrior.mimic_skill.ranks",
        "class_feature.adventurers_guide.sanguine_angel.armored_angel.level",
        "class_feature.adventurers_guide.sanguine_angel.mystique_of_ardad_lili.caster_level",
        "class_feature.adventurers_guide.sanguine_angel.mystique_of_ardad_lili.dc",
        "class_feature.ultimate_psionics.body_snatcher.body_thief.caster_level_bonus",
        "class_feature.ultimate_psionics.body_snatcher.death_is_only_the_beginning.caster_\
         level_bonus",
        "class_feature.ultimate_psionics.body_snatcher.melding_exchange.bonus",
        "class_feature.adventurers_guide.steel_falcon.chainbreaker.bonus",
        "class_feature.adventurers_guide.steel_falcon.enemy_of_slavers.bonus",
        "class_feature.adventurers_guide.steel_falcon.sailor_and_survivalist.bonus",
        "class_feature.adventurers_guide.steel_falcon.talmandor_s_blessing.acrobatics_bonus",
        "class_feature.adventurers_guide.steel_falcon.talmandor_s_blessing.perception_bonus",
        "class_feature.adventurers_guide.lantern_bearer.favored_enemy.pool_size",
        "class_feature.adventurers_guide.lantern_bearer.favored_enemy.bonus_pool_size",
        "class_feature.adventurers_guide.lantern_bearer.proven_weapon_familiarity.bonus",
        "class_feature.adventurers_guide.lantern_bearer.superior_discernment.pool_size",
        "class_feature.adventurers_guide.storm_kindler.aura_of_calm.radius",
        "class_feature.adventurers_guide.storm_kindler.aura_of_calm.bonus",
        "class_feature.adventurers_guide.storm_kindler.oceanic_spirit.bonus",
        "class_feature.adventurers_guide.storm_kindler.storm_shape.height",
        "class_feature.adventurers_guide.storm_kindler.weathers_fury.bonus",
        "class_feature.adventurers_guide.westcrown_devil.council_s_secret.pool_size",
        "class_feature.adventurers_guide.westcrown_devil.founders_favor.pool",
        "class_feature.adventurers_guide.westcrown_devil.founders_favor.dc",
        "class_feature.adventurers_guide.westcrown_devil.sneak_attack.dice",
        "class_feature.ultimate_psionics.pyrokineticist.bolt_of_fire.bonus",
        "class_feature.ultimate_psionics.pyrokineticist.fire_adaptation.bonus",
        "class_feature.ultimate_psionics.pyrokineticist.fire_adaptation.fire_resistance",
        "class_feature.ultimate_psionics.pyrokineticist.hand_afire.bonus",
        "class_feature.ultimate_psionics.pyrokineticist.leech_heat.bonus",
        "class_feature.ultimate_psionics.pyrokineticist.manipulate_blaze.range",
        "class_feature.ultimate_psionics.pyrokineticist.nimbus.duration_rounds",
        "class_feature.ultimate_psionics.pyrokineticist.nimbus.bonus",
        "class_feature.ultimate_psionics.pyrokineticist.penetrating_fire.bonus",
        "class_feature.ultimate_psionics.pyrokineticist.weapon_afire.bonus",
        "class_feature.adventurers_guide.aspis_agent.agency_secrets.pool_size",
        "class_feature.adventurers_guide.aspis_agent.sneak_attack.dice",
        "class_feature.adventurers_guide.aspis_agent.trap_sense.bonus",
        "class_feature.adventurers_guide.aspis_agent.trapfinding.bonus",
        "class_feature.adventurers_guide.gray_corsair.favored_port.bonus",
        "class_feature.adventurers_guide.gray_corsair.slaver_slayer.bonus",
        "class_feature.adventurers_guide.pathfinder_savant.master_scholar.bonus",
        "class_feature.adventurers_guide.pathfinder_savant.esoteric_magic.pool_size",
        "class_feature.adventurers_guide.pathfinder_savant.quick_identification.times_per_day",
        "class_feature.adventurers_guide.pathfinder_savant.sigil_master.save_bonus",
        "class_feature.adventurers_guide.pathfinder_savant.analyze_dweomer.times_per_day",
        "class_feature.adventurers_guide.rivethun_emissary.enhanced_spirit_animal.evolution_\
         points",
        "class_feature.adventurers_guide.rivethun_emissary.parley.uses_per_day",
        "class_feature.adventurers_guide.rivethun_emissary.parley.dc",
        "class_feature.adventurers_guide.rivethun_emissary.sixth_sense.uses_per_day",
        "class_feature.adventurers_guide.rivethun_emissary.spirit_animal.level",
        "class_feature.adventurers_guide.rivethun_emissary.spirit_bond.hex_dc",
        "class_feature.adventurers_guide.rivethun_emissary.spirit_bond.hex_ability_level",
        "class_feature.adventurers_guide.student_of_war.additional_skill.pool_size",
        "class_feature.adventurers_guide.student_of_war.mind_over_metal.ac_bonus",
        "class_feature.book_of_the_damned_volume_1.diabolist.channel_hellfire.times_per_day",
        "class_feature.book_of_the_damned_volume_1.diabolist.infernal_transport.times_per_day",
        "class_feature.book_of_the_damned_volume_1.diabolist.damned.dc",
        "class_feature.book_of_the_damned_volume_1.diabolist.infernal_charisma.bonus",
        "class_feature.book_of_the_damned_volume_1.diabolist.heresy.bonus",
        "class_feature.book_of_the_damned_volume_1.diabolist.hellfire_ray.caster_level",
        "class_feature.book_of_the_damned_volume_1.diabolist.hellfire_ray.dc",
        "class_feature.inner_sea_intrigue.lion_blade.expeditious_advance.speed_bonus",
        "class_feature.inner_sea_intrigue.lion_blade.silent_soul.stealth_bonus",
        "class_feature.inner_sea_intrigue.lion_blade.sneak_attack.dice",
        "class_feature.adventurers_guide.bellflower_tiller.bellflower_crop.range",
        "class_feature.adventurers_guide.bellflower_tiller.crop_guardian.bonus",
        "class_feature.adventurers_guide.bellflower_tiller.sneak_attack.dice",
        "class_feature.adventurers_guide.bellflower_tiller.swift_sower.speed_bonus",
        "class_feature.adventurers_guide.bellflower_tiller.teamwork_feat.pool_size",
        "class_feature.adventurers_guide.hellknight_signifer.assiduous_gaze.pool_size",
        "class_feature.adventurers_guide.hellknight_signifer.signifer_mask.bonus",
        "class_feature.adventurers_guide.hellknight_signifer.infernal_resilience.dr",
        "class_feature.ultimate_psionics.mystic_archer.heightened_senses.range",
        "class_feature.ultimate_psionics.mystic_archer.blindsense.range",
        "class_feature.ultimate_psionics.mystic_archer.blindsight.range",
        "class_feature.ultimate_psionics.mystic_archer.tremorsense.range",
        "class_feature.ultimate_psionics.mystic_archer.inevitable_strike.uses_per_day",
        "class_feature.ultimate_psionics.mystic_archer.ranged_sneak_attack.dice",
        "class_feature.ultimate_psionics.mystic_archer.unhindered_vision.uses_per_day",
        "class_feature.adventurers_guide.mammoth_rider.born_survivor.pool_size",
        "class_feature.adventurers_guide.mammoth_rider.gigantic_steed.qualify_flag",
        "class_feature.adventurers_guide.mammoth_rider.rugged_steed.qualify_flag",
        "class_feature.adventurers_guide.mammoth_rider.steeds_reach.qualify_flag",
        "class_feature.adventurers_guide.mammoth_rider.steed.companion_level",
        "class_feature.adventurers_guide.mammoth_rider.wild_coercion.level",
        "class_feature.book_of_the_damned_volume_2.demoniac.summon_demon_i.caster_level",
        "class_feature.book_of_the_damned_volume_2.demoniac.summon_demon_ii.caster_level",
        "class_feature.advanced_players_guide.master_chymist.advanced_mutagen.pool_size",
        "class_feature.advanced_players_guide.master_chymist.bomb_thrower.level",
        "class_feature.advanced_players_guide.master_chymist.brutality.bonus",
        "class_feature.advanced_players_guide.master_chymist.extracts_per_day.level",
        "class_feature.advanced_players_guide.master_chymist.mutate.times_per_day",
        "class_feature.inner_sea_intrigue.enchanting_courtesan.hidden_spell.count",
        "class_feature.inner_sea_intrigue.enchanting_courtesan.seductive_intuition.bonus",
        "class_feature.ultimate_psionics.dark_tempest.blade_skills.pool_size",
        "class_feature.ultimate_psionics.dark_tempest.diverse_training.level",
        "class_feature.ultimate_psionics.dark_tempest.expanded_power_list.pool_size",
        "class_feature.ultimate_psionics.dark_tempest.power_strike.power_level",
        "class_feature.ultimate_psionics.dark_tempest.psychic_strike.dice",
        "class_feature.advanced_players_guide.battle_herald.inspiring_command.level",
        "class_feature.advanced_players_guide.battle_herald.teamwork_feat.pool_size",
        "class_feature.advanced_players_guide.master_spy.art_of_deception.bonus",
        "class_feature.advanced_players_guide.master_spy.slippery_mind.times",
        "class_feature.advanced_players_guide.master_spy.sneak_attack.dice",
        "class_feature.ultimate_combat.evangelist.single_minded.domain_count_delta",
        "class_feature.inner_sea_combat.ulfen_guard.guard_dedications.pool_size",
    ];

    #[test]
    fn none_of_wave_49s_ids_leak_onto_an_unrelated_class() {
        let fighter = character("class:fighter", 10);
        for id in ALL_WAVE_49_EXPLANATION_IDS {
            assert_eq!(
                explanation_value(&fighter, id),
                None,
                "a Fighter must not gain any of wave 49's new class-feature ids: {id}"
            );
        }
    }
}

