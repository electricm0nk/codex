#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm, risks item 8 (Alchemist Mutagen closure, second APG
/// class-specific closure): APG Alchemist's Mutagen, verified directly
/// against `apg_abilities_class.lst:75`. Unlike every prior activation-
/// gated closure this session (Barbarian/Skald/Bloodrager Rage-shaped
/// mechanics, which affect a FIXED set of stats), Mutagen genuinely
/// requires a choice: "you select one physical ability score -- either
/// Strength, Dexterity, or Constitution" -- and the mental-score PENALTY
/// target depends on that choice (Str->Int, Dex->Wis, Con->Cha, per the
/// corpus DESC). This combines the choice-recognition pattern (Sorcerer's
/// Arcane Bond / Cleric's domain choice) with the activation-gating
/// pattern (Barbarian's Rage) for the first time -- both independently
/// proven, combined here, confirmed by the lead not to need a full
/// adversarial review. Confirmed (grepping `CharacterSheet.tsx` directly
/// before scoping) that this lands in the same headless-only
/// reachability bucket as Sorcerer/Cleric/Druid's own choice-gated
/// mechanics: no generic choice-picker exists anywhere in the creation/
/// level-up UI for an arbitrary choice set like this one.
pub(super) const ALCHEMIST_CLASS_ID: &str = "class:alchemist";

/// `ClassAbilityActivation.ability_id` for Alchemist Mutagen. Unlike
/// Rage-shaped mechanics, Mutagen has no rounds-per-day budget to
/// validate (the corpus text has no daily-use cap, only a "one dose at a
/// time" brewing-supply constraint this codebase's activation-state
/// schema doesn't need to model to ground the ability-score/AC values
/// honestly).
pub(super) const ALCHEMIST_MUTAGEN_ABILITY_ID: &str = "mutagen";

/// The choice set and selection prefix for which physical ability score
/// Mutagen enhances -- reuses `ABILITY_SELECTION_PREFIX` (`"ability:"`),
/// the same idiom Human's own ability-bonus choice already uses, rather
/// than inventing a new naming scheme.
pub(super) const ALCHEMIST_MUTAGEN_STAT_CHOICE_ID: &str = "choice:alchemist_mutagen_stat";

/// PF1 Advanced Player's Guide Mutagen: "+4 alchemical bonus to the
/// selected ability score" (an ability-SCORE bonus, halved onto the
/// ability-MODIFIER exactly like every other Rage-shaped bonus this
/// session grounds -- an even score bonus always halves exactly).
pub(super) const ALCHEMIST_MUTAGEN_STAT_BONUS: i16 = 4;

/// PF1 Advanced Player's Guide Mutagen: "a -2 penalty to one of your
/// mental ability scores" -- an ability-SCORE penalty, halved onto the
/// ability-MODIFIER the same way the bonus above is.
pub(super) const ALCHEMIST_MUTAGEN_STAT_PENALTY: i16 = -2;

/// PF1 Advanced Player's Guide Mutagen: "a +2 natural armor bonus."
pub(super) const ALCHEMIST_MUTAGEN_NATURAL_ARMOR_BONUS: i16 = 2;

/// PF1 Advanced Player's Guide Alchemist Poison Resistance's level gates
/// (deepening 2026-07-26, task #4), re-derived directly against
/// `apg_abilities_class.lst`'s own `AlchemistPoisonLVL` tier tokens
/// (`PREVARGTEQ:AlchemistLVL,2/5/8/10`) rather than assumed from
/// Investigator's own identical-shaped feature: None below level 2,
/// +2 (2-4), +4 (5-7), +6 (8-9), full immunity at 10+. Kept as
/// Alchemist's own separate constants/functions rather than reusing
/// Investigator's directly, the same "parallel copy over cross-class-
/// function-reuse" discipline Skald's own spellcasting closure and
/// Inquisitor's own Track already used.
pub(super) const ALCHEMIST_POISON_RESISTANCE_LEVEL: u8 = 2;

pub(super) const ALCHEMIST_POISON_RESISTANCE_TWO_LEVEL: u8 = 5;

pub(super) const ALCHEMIST_POISON_RESISTANCE_THREE_LEVEL: u8 = 8;

pub(super) const ALCHEMIST_POISON_IMMUNITY_LEVEL: u8 = 10;

/// Alchemist's Discovery chooser, and the ONE canonical Discovery this
/// codebase grounds out of the corpus's 35 (v0.6 alpha swarm, Alchemist
/// spellcasting-shaped closure) -- the same canonical-narrowing shape
/// Cleric's Good domain, Wizard's Evocation school, Oracle's Mystery and
/// Arcanist's Metamagic Knowledge already established: one corpus-verified
/// option grounded for real, the other 34 named as honestly deferred.
///
/// The grant cadence is `level/2` -- read directly off
/// `apg_abilities_class.lst`'s own `KEY:Alchemist ~ Discovery` record
/// (`BONUS:ABILITYPOOL|Alchemist Discovery|AlchemistDiscoveryLVL/2`,
/// with `BONUS:VAR|AlchemistDiscoveryLVL|AlchemistLVL`), so the first
/// Discovery lands at alchemist level 2, not level 1.
///
/// Feral Mutagen is the canonical pick because it is the only Discovery
/// whose corpus record carries real, self-contained numeric magnitudes
/// that attach to an already-grounded feature of this class: it extends
/// Mutagen (already wired here, choice- and activation-gated) rather than
/// requiring a subsystem this engine lacks. Verified directly against
/// `KEY:Discovery ~ Feral Mutagen`: "he gains two claw attacks and a bite
/// attack... The claw attacks deal 1d6 points of damage (1d4 if the
/// alchemist is Small) and the bite attack deals 1d8 points of damage
/// (1d6 if the alchemist is Small). While the mutagen is in effect, the
/// alchemist gains a +2 competence bonus on Intimidate skill checks"
/// (`TEMPBONUS:PC|SKILL|Intimidate|2`).
pub(super) const ALCHEMIST_DISCOVERY_CHOICE_ID: &str = "choice:alchemist_discovery";

/// First alchemist level at which any Discovery is granted (`level/2`).
pub(super) const ALCHEMIST_DISCOVERY_GRANT_LEVEL: u8 = 2;

/// v0.6 alpha swarm, risks item 8 (Investigator full-build closure, 10th
/// ACG/APG class-specific closure): APG Investigator, verified directly
/// against `acg_classes.lst`'s own `SPELLSTAT:INT`/`MEMORIZE:YES`/
/// `SPELLBOOK:YES` tokens -- a PREPARED caster like Wizard/Arcanist/
/// Warpriest, not the harder spontaneous shape Oracle needed. This
/// closure is a smaller no-spellcasting MVP: Investigator's own
/// `SPELLLIST:1|Alchemist` reuses the Alchemist formula list, and no
/// Alchemist spell-list mapping exists anywhere in this codebase yet
/// (confirmed directly) -- building one is a genuinely new data-
/// ingestion cost, deferred to its own dedicated follow-on slice
/// (mirroring the Skald spellcasting split). This slice grounds only
/// Trapfinding, Trap Sense, and Inspiration's flat pool-size fact.
///
/// Investigator's own real class-skill list is a genuine 2-of-3 PARTIAL
/// match (Climb and Intimidate present, Swim absent) -- the first
/// partial match on the whole roster, forcing the class-skill-bonus
/// helper to split from one shared scalar into three independent per-
/// skill functions (see `selected_skill_climb_is_class_skill`'s own doc
/// comment). See `docs/release/v0.6/investigator-acg-full-build-scoping.md`
/// for the full corpus verification and scope record.
pub(super) const INVESTIGATOR_CLASS_ID: &str = "class:investigator";

/// Studied Defense is an Investigator TALENT gated
/// `PREVARGTEQ:InvestigatorTalentLVL,9`. That variable IS properly set
/// (`BONUS:VAR|InvestigatorTalentLVL|InvestigatorLVL`) -- checked
/// tree-wide rather than assumed by analogy to Swashbuckler's deed gate,
/// which superficially resembles it but genuinely is never set.
pub(super) const INVESTIGATOR_STUDIED_DEFENSE_LEVEL: u8 = 9;

/// Studied Defense redirects Studied Combat's SAME insight bonus from
/// attack rolls to Armor Class. One magnitude, two destinations, chosen
/// by the player -- so it takes an explicit recorded choice under the
/// ratified Skill Focus precedent, never a canonical default.
pub(super) const INVESTIGATOR_STUDIED_DEFENSE_CHOICE_ID: &str = "choice:investigator_studied_defense";

pub(super) const INVESTIGATOR_STUDIED_DEFENSE_SELECTION: &str = "studied_defense:armor_class";

/// PF1 Advanced Class Guide level gate of the Investigator's FIRST talent:
/// "At 3rd level, and every two levels thereafter, an investigator gains
/// an investigator talent" -- genuinely DIFFERENT cadence from Rogue's own
/// 2/4/6/8..., verified independently (not assumed by analogy). Investigator
/// draws Resiliency from its own explicit 40-record whitelist of
/// `KEY:Investigator ~ Rogue Talent ~ *` records (`acg_abilities_class.lst`)
/// -- a genuinely separate corpus record from Rogue's own copy, with its
/// own level variable (`BONUS:VAR|ResiliencyHitPoints|InvestigatorLVL`, not
/// `RogueTalentLVL`). Only Resiliency is recognized here (task #58); the
/// other 39 whitelist entries and every other Investigator Talent stay
/// named-but-unproven, the same open-ended-chooser posture Rogue's own
/// talent slots carry for every OTHER talent besides Resiliency.
pub(super) const INVESTIGATOR_TALENT_GRANT_LEVEL: u8 = 3;

pub(super) const INVESTIGATOR_TALENT_CHOICE_ID: &str = "choice:investigator_talent";

/// v0.6 alpha swarm, risks item 8 (Alchemist Mutagen closure): whether
/// `input` is a single-class Alchemist at a level within
/// `apg::class_chassis_resolve`'s declared ceiling for Alchemist --
/// mirrors `is_supported_cavalier_single_class` exactly, including the
/// same exact-match discipline (`== Some(ApgClassId::Alchemist)`, not a
/// broad `.is_some()`).
pub(super) fn is_supported_alchemist_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if ApgClassId::from_class_id_str(&class_level.class_id) != Some(ApgClassId::Alchemist) {
        return false;
    }
    apg::class_chassis_resolve(ApgClassId::Alchemist, class_level.level, RuleSetId::Apg).is_some()
}

/// v0.6 alpha swarm, risks item 8 (Investigator full-build closure):
/// whether `input` is a single-class Investigator at a level within
/// `acg::class_chassis_resolve`'s declared ceiling for Investigator --
/// mirrors the other eight ACG/APG exact-match gates exactly.
pub(super) fn is_supported_investigator_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if AcgClassId::from_class_id_str(&class_level.class_id) != Some(AcgClassId::Investigator) {
        return false;
    }
    acg::class_chassis_resolve(AcgClassId::Investigator, class_level.level, RuleSetId::Acg)
        .is_some()
}

/// Alchemist Mutagen's duration: `level * 10` minutes (v0.6 alpha swarm,
/// risks item 8, Alchemist Mutagen closure), verified against the corpus
/// formula `AlchemistMutagenDuration = AlchemistLVL*10`. Informational
/// only -- this codebase tracks no elapsed-time state, the same "named
/// but not simulated" shape every other timed value this session grounds
/// uses (e.g. Barbarian Rage's own rounds-consumed-today budget, which is
/// validated but never causes time to actually pass).
pub(super) fn alchemist_mutagen_duration_minutes(level: u8) -> i16 {
    i16::from(level) * 10
}

/// Mutagen's mental-ability-score penalty target for a given chosen
/// physical ability (v0.6 alpha swarm, risks item 8, Alchemist Mutagen
/// closure), verified verbatim against the corpus DESC text: "If the
/// mutagen enhances your Strength, it applies a penalty to your
/// Intelligence. If it enhances your Dexterity, it applies a penalty to
/// your Wisdom. If it enhances your Constitution, it applies a penalty
/// to your Charisma." Returns `None` for any ability outside the three
/// valid Mutagen targets (Strength/Dexterity/Constitution), so an
/// unrecognized or non-physical selection is never silently mapped to a
/// fabricated penalty target.
pub(super) fn alchemist_mutagen_mental_penalty_target(physical_ability: &str) -> Option<&'static str> {
    match physical_ability {
        "strength" => Some("intelligence"),
        "dexterity" => Some("wisdom"),
        "constitution" => Some("charisma"),
        _ => None,
    }
}

/// Whether `input` is an Alchemist validly, actively mutated right now
/// with a recognized stat choice, and if so, the chosen physical ability
/// and its corresponding mental-penalty target (v0.6 alpha swarm, risks
/// item 8, Alchemist Mutagen closure). Class-ownership-gated by
/// construction, mirroring every other `active_<class>_<ability>_bonus`
/// query function this session: only returns `Some` when `class_levels`
/// actually contains Alchemist. An activation present but not
/// `ActiveState::EquippedActive`, or one active but missing/naming an
/// unrecognized `choice:alchemist_mutagen_stat` selection, is treated the
/// same as "not mutated" here -- pushes no diagnostic itself
/// (`ground_or_block_alchemist_mutagen` is the single place that pushes
/// the posture-violation claim-blocking diagnostic and the informational
/// recognition records).
pub(super) fn active_alchemist_mutagen_bonus(
    input: &CharacterInput,
) -> Option<(u8, &str, &'static str)> {
    let alchemist_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == ALCHEMIST_CLASS_ID)
        .map(|class_level| class_level.level)?;

    let activation = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == ALCHEMIST_MUTAGEN_ABILITY_ID)?;

    if activation.active_state != ActiveState::EquippedActive {
        return None;
    }

    let selection = choice_selection(input, ALCHEMIST_MUTAGEN_STAT_CHOICE_ID)?;
    let physical_ability = selection.strip_prefix(ABILITY_SELECTION_PREFIX).unwrap_or(selection);
    let mental_ability = alchemist_mutagen_mental_penalty_target(physical_ability)?;

    Some((alchemist_level, physical_ability, mental_ability))
}

/// PF1 Advanced Player's Guide Alchemist Bomb damage dice (deepening
/// 2026-07-26, task #4): a base 1d6 plus `(level-1)/2` additional d6 --
/// verified directly against `apg_abilities_class.lst`'s own
/// `BONUS:VAR|AlchemistBombAdditionalDice|(AlchemistBombLVL-1)/2`
/// (`AlchemistBombDiceSize|6` confirms the die size). **Verified the full
/// 1-20 progression carefully, not just the base formula shape** (the
/// same rigor the Sacred Weapon dice-count bug demanded): the corpus also
/// carries 10 conditional `-1` override branches at levels 3/5/7/9/11/
/// 13/15/17/19, each gated behind `PREVAREQ:Alchemist_CF_BombDamage<N>,1`
/// -- but `Alchemist_CF_BombDamage<N>` is `DEFINE`d to `0` for the base
/// Alchemist class (`apg_abilities_globalvar.lst`) and is ONLY ever set
/// to `1` by two specific Ultimate Magic archetype feature grants
/// (Psychonaut, Reanimator, `um_abilities_class.lst`), entirely
/// out-of-scope splatbook/archetype content this codebase's own
/// single-book, base-class-only build never reaches. These 10 branches
/// are therefore provably vacuous for every character this engine
/// represents, the same "provably vacuous precondition" shape Brawler's
/// own light-armor check and Dodge's own AC bonus already established --
/// the base formula is the real, complete answer for this closure's
/// scope.
pub fn alchemist_bomb_damage_dice(level: u8) -> i16 {
    1 + (i16::from(level) - 1) / 2
}

/// PF1 Advanced Player's Guide Alchemist Bomb damage bonus: a flat
/// Intelligence modifier added to bomb damage (deepening 2026-07-26,
/// task #4), verified directly against `apg_abilities_class.lst`'s own
/// `BONUS:VAR|AlchemistBombDamageBonus|INT`.
pub fn alchemist_bomb_damage_bonus(intelligence_modifier: i16) -> i16 {
    intelligence_modifier
}

/// PF1 Advanced Player's Guide Alchemist Bomb save DC: `10 + (level/2) +
/// Intelligence modifier` (deepening 2026-07-26, task #4), verified
/// directly against `apg_abilities_class.lst`'s own
/// `BONUS:VAR|AlchemistBombDC|10+(AlchemistBombLVL/2)+INT`, the same
/// flat-DC standalone shape Blessing's/Mutagen's own DC-style facts use.
pub fn alchemist_bomb_dc(level: u8, intelligence_modifier: i16) -> i16 {
    10 + i16::from(level) / 2 + intelligence_modifier
}

/// PF1 Advanced Player's Guide Alchemist Bomb uses per day: `level +
/// Intelligence modifier` (deepening 2026-07-26, task #4), verified
/// directly against `apg_abilities_class.lst`'s own
/// `BONUS:VAR|AlchemistBombTimes|AlchemistBombLVL+INT`. A second
/// contribution exists in the corpus (`BONUS:VAR|AlchemistBombTimes|
/// BonusBombCount/2`), but `BonusBombCount` (`DEFINE:BonusBombCount|0`,
/// `apg_abilities_race.lst`) only ever increments via a Gnome-specific
/// Favored Class Bonus CHOICE this codebase's own fixed Human posture
/// never reaches (no favored-class-bonus selection mechanism exists
/// anywhere in this engine) -- provably `0` for every character this
/// engine represents, the same "provably vacuous" shape the Bomb damage
/// dice's own override branches already established above.
///
/// A THIRD contribution to the same variable is real and now applied:
/// the `Extra Bombs` feat's own `BONUS:VAR|AlchemistBombTimes|2` (see
/// [`EXTRA_BOMBS_FEAT_KEY`]). Folded into the formula rather than added
/// at the display site, matching [`barbarian_rage_rounds_per_day`]'s
/// ratified reasoning: any future budget check reads the widened pool
/// by construction.
pub(super) fn alchemist_bomb_uses_per_day(
    level: u8,
    intelligence_modifier: i16,
    selected_feats: &[String],
) -> i16 {
    i16::from(level)
        + intelligence_modifier
        + extra_resource_feat_bonus(selected_feats, EXTRA_BOMBS_FEAT_KEY, EXTRA_BOMBS_USES)
}

/// PF1 Advanced Player's Guide Alchemist Poison Resistance's numeric
/// bonus tier (deepening 2026-07-26, task #4), re-derived directly
/// against `apg_abilities_class.lst`'s own `AlchemistPoisonLVL`
/// tier-gating tokens: `None` below level 2 (not yet gained), `Some(2)`
/// from level 2, `Some(4)` from level 5, `Some(6)` from level 8 --
/// identical shape to Investigator's own Poison Resistance, confirmed
/// independently rather than assumed. Kept as Alchemist's own separate
/// function per the established parallel-copy discipline.
pub(super) fn alchemist_poison_resistance_bonus(level: u8) -> Option<i16> {
    if level < ALCHEMIST_POISON_RESISTANCE_LEVEL {
        None
    } else if level < ALCHEMIST_POISON_RESISTANCE_TWO_LEVEL {
        Some(2)
    } else if level < ALCHEMIST_POISON_RESISTANCE_THREE_LEVEL {
        Some(4)
    } else {
        Some(6)
    }
}

/// Whether `level` has reached full poison immunity (deepening
/// 2026-07-26, task #4), verified directly against the corpus's own
/// immunity gate at `AlchemistPoisonLVL,4` (resolving to Alchemist level
/// 10) -- identical shape to Investigator's own
/// `investigator_is_poison_immune`, kept as Alchemist's own separate copy.
pub(super) fn alchemist_is_poison_immune(level: u8) -> bool {
    level >= ALCHEMIST_POISON_IMMUNITY_LEVEL
}

/// Grounds Alchemist's Bomb (damage dice/bonus, save DC, uses-per-day)
/// and Poison Resistance as standalone explanation records (deepening
/// 2026-07-26, task #4) -- both are flat, unconditional, always-on
/// facts (no choice or activation gate, unlike Mutagen), so this never
/// claim-blocks. Called unconditionally from `compute_apg_class_chassis`'s
/// Alchemist branch, independent of Mutagen's own state.
///
/// SD28-C4.8: the Poison Resistance branch below is the first real
/// consumer of `archetype_resolver::archetype_claims_slot`
/// (`decisions.md §59`/`§60`). Before this, the base APG progression
/// computed unconditionally for every Alchemist regardless of archetype
/// selection -- the exact "provably vacuous... this repo ingests no
/// Alchemist archetype" shape `§59` catalogued for six other classes, now
/// closed here for real rather than left vacuous, because Alchemist
/// archetypes ARE ingested (`§51`'s ACG/ARG tables).
pub(super) fn ground_alchemist_bomb_and_poison_resistance(
    input: &CharacterInput,
    level: u8,
    intelligence_modifier: i16,
    selected_feats: &[String],
    explanations: &mut Vec<ComputationExplanation>,
) {
    let bomb_damage_dice = alchemist_bomb_damage_dice(level);
    let bomb_damage_bonus = alchemist_bomb_damage_bonus(intelligence_modifier);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.alchemist.bomb_damage".to_owned(),
        value: bomb_damage_dice,
        detail: format!(
            "Alchemist level {level} Bomb damage: {bomb_damage_dice}d6 (1 + (level-1)/2 = \
             {bomb_damage_dice}) + {bomb_damage_bonus:+} Intelligence modifier. This is a \
             weapon-like damage magnitude against any target, not conditioned on an \
             opponent-tracking interaction (unlike Studied Combat/Strike), so it grounds \
             standalone the same way Sacred Weapon's dice count and the Wolf companion's bite \
             damage already do"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.alchemist.bomb_dc".to_owned(),
        value: alchemist_bomb_dc(level, intelligence_modifier),
        detail: format!(
            "Alchemist level {level} Bomb save DC: 10 + (level/2) + Intelligence modifier \
             ({intelligence_modifier:+}) = {}",
            alchemist_bomb_dc(level, intelligence_modifier)
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.alchemist.bomb_uses_per_day".to_owned(),
        value: alchemist_bomb_uses_per_day(level, intelligence_modifier, selected_feats),
        detail: format!(
            "Alchemist level {level} Bomb uses per day: level + Intelligence modifier \
             ({intelligence_modifier:+}) + Extra Bombs feat ({:+}) = {}. A Gnome-specific \
             Favored Class Bonus can add further uses in the real PF1 rules, but this codebase \
             models no favored-class-bonus selection mechanism at all, so that term is provably \
             0 here",
            extra_resource_feat_bonus(selected_feats, EXTRA_BOMBS_FEAT_KEY, EXTRA_BOMBS_USES),
            alchemist_bomb_uses_per_day(level, intelligence_modifier, selected_feats)
        ),
    });

    if let Some(archetype_name) =
        archetype_resolver::archetype_claiming_slot(input, "Alchemist", "AlchemistPoisonResistance")
    {
        // SD28-C4.8: the base APG Poison Resistance progression no longer
        // grounds unconditionally once a real, selected archetype has
        // claimed this slot (e.g. ARG's Plague Bringer, whose own
        // `replaces` list names this slot alongside its poison-resistance
        // tiers and Mutagen -- confirmed against the real catalog row,
        // `archetype_resolver.rs`'s own test). This does not compute the
        // archetype's OWN replacement mechanic (that is a separate,
        // not-yet-modelled feature this table's `grants` list names but
        // does not itself ground), so the value stays 0 and the message
        // says plainly that the base progression is superseded, not that
        // it fires -- the same "name the gap, do not fabricate a number
        // for it" discipline this codebase applies everywhere else.
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.alchemist.poison_resistance_bonus".to_owned(),
            value: 0,
            detail: format!(
                "Alchemist level {level} Poison Resistance: superseded by the selected {archetype_name} \
                 archetype, which replaces this base-class slot -- the base APG progression \
                 does not apply. {archetype_name}'s own replacement feature (if any) is not \
                 separately computed here"
            ),
        });
    } else {
    match alchemist_poison_resistance_bonus(level) {
        None => {
            explanations.push(ComputationExplanation {
                id: "class_feature.apg.alchemist.poison_resistance_bonus".to_owned(),
                value: 0,
                detail: format!(
                    "Alchemist level {level} Poison Resistance: correctly absent below level 2 \
                     by PF1 Advanced Player's Guide level gate; the at-grant magnitude is named \
                     but not computed"
                ),
            });
        }
        Some(bonus) if alchemist_is_poison_immune(level) => {
            explanations.push(ComputationExplanation {
                id: "class_feature.apg.alchemist.poison_resistance_bonus".to_owned(),
                value: 0,
                detail: format!(
                    "Alchemist level {level} is fully immune to poison (PF1 Advanced Player's \
                     Guide, granted at level 10): a qualitatively different fact from the \
                     numeric resistance bonus (which topped out at +{bonus} at level 8-9), not \
                     a fourth scaling tier, so no numeric bonus value applies here"
                ),
            });
        }
        Some(bonus) => {
            explanations.push(ComputationExplanation {
                id: "class_feature.apg.alchemist.poison_resistance_bonus".to_owned(),
                value: bonus,
                detail: format!(
                    "Alchemist level {level} Poison Resistance: a +{bonus} bonus on all saving \
                     throws against poison (2 at level 2, 4 at level 5, 6 at level 8, immune at \
                     level 10). No poison-save total exists anywhere in this codebase, so this \
                     grounds only the flat bonus value, identical shape to Investigator's own \
                     Poison Resistance"
                ),
            });
        }
    }
    }
}

/// PF1 Advanced Player's Guide Alchemist "Extracts Prepared" table
/// (deepening 2026-07-26, task #4), for extract levels 1st through 6th,
/// indexed 0=1st..5=6th -- identical to Investigator's own table (both
/// verified directly against d20pfsrd.com's own separate Alchemist page:
/// byte-for-byte the same progression), kept as Alchemist's own separate
/// copy per the established parallel-copy discipline. `apg_classes.lst`
/// has no per-level `CAST:`/`KNOWN:` rows for Alchemist at all -- the
/// same external-source caveat Investigator/Hunter/Arcanist/Warpriest
/// already had.
pub(super) fn alchemist_base_extracts_per_day(level: u8) -> [Option<i16>; 6] {
    match level {
        1 => [Some(1), None, None, None, None, None],
        2 => [Some(2), None, None, None, None, None],
        3 => [Some(3), None, None, None, None, None],
        4 => [Some(3), Some(1), None, None, None, None],
        5 => [Some(4), Some(2), None, None, None, None],
        6 => [Some(4), Some(3), None, None, None, None],
        7 => [Some(4), Some(3), Some(1), None, None, None],
        8 => [Some(4), Some(4), Some(2), None, None, None],
        9 => [Some(5), Some(4), Some(3), None, None, None],
        10 => [Some(5), Some(4), Some(3), Some(1), None, None],
        11 => [Some(5), Some(4), Some(4), Some(2), None, None],
        12 => [Some(5), Some(5), Some(4), Some(3), None, None],
        13 => [Some(5), Some(5), Some(4), Some(3), Some(1), None],
        14 => [Some(5), Some(5), Some(4), Some(4), Some(2), None],
        15 => [Some(5), Some(5), Some(5), Some(4), Some(3), None],
        16 => [Some(5), Some(5), Some(5), Some(4), Some(3), Some(1)],
        17 => [Some(5), Some(5), Some(5), Some(4), Some(4), Some(2)],
        18 => [Some(5), Some(5), Some(5), Some(5), Some(4), Some(3)],
        19 => [Some(5), Some(5), Some(5), Some(5), Some(5), Some(4)],
        _ => [Some(5), Some(5), Some(5), Some(5), Some(5), Some(5)],
    }
}

/// PF1 Advanced Player's Guide Alchemist extract save DC: `10 + extract
/// level + Intelligence modifier` (deepening 2026-07-26, task #4),
/// identical shape to Investigator's own extract save DC, kept as
/// Alchemist's own separate copy.
pub(super) fn alchemist_extract_save_dc(extract_level: u8, intelligence_modifier: i16) -> i16 {
    10 + i16::from(extract_level) + intelligence_modifier
}

/// Parses an Alchemist extract's `spell_id` into its real Alchemist-
/// specific extract level (deepening 2026-07-26, task #4), by looking it
/// up directly in `alchemist_spell_list::ALCHEMIST_SPELL_LIST` -- the
/// same shared list Investigator's own `parse_investigator_extract_id`
/// already consumes (Alchemist IS the list's own namesake class, so this
/// is direct reuse of the underlying data, not a parallel copy of it).
/// No arcane-school mechanic exists for Alchemist (confirmed: no
/// "School" record anywhere in its own `KEY:Alchemist ~ ...` list), so
/// every prepared extract costs exactly 1 slot.
pub(super) fn parse_alchemist_extract_id(spell_id: &str) -> Option<u8> {
    alchemist_spell_list::alchemist_spell_level(spell_id)
}

/// Return the list of unmet conditions for Alchemist's prepared-extract
/// posture (deepening 2026-07-26, task #4). Mirrors
/// `unmet_investigator_extract_conditions`'s exact shape (itself mirroring
/// `unmet_arcanist_spellbook_conditions`): an empty list means the
/// posture is fully supported.
pub(super) fn unmet_alchemist_extract_conditions(
    input: &CharacterInput,
    level: u8,
    intelligence_modifier: i16,
) -> Vec<String> {
    let mut unmet = Vec::new();

    let alchemist_spells = |mode: AcquisitionMode| {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(move |s| s.source_class_id == ALCHEMIST_CLASS_ID && s.acquisition_mode == mode)
    };
    let recorded: Vec<&str> = alchemist_spells(AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();
    let prepared: Vec<&str> = alchemist_spells(AcquisitionMode::Prepared)
        .map(|s| s.spell_id.as_str())
        .collect();

    if recorded.is_empty() {
        unmet.push(
            "no alchemist extracts recorded in the formula book (AcquisitionMode::Known)"
                .to_owned(),
        );
    }
    if prepared.is_empty() {
        unmet.push("no alchemist extracts prepared today (AcquisitionMode::Prepared)".to_owned());
    }

    for spell_id in &prepared {
        if !recorded.contains(spell_id) {
            unmet.push(format!(
                "prepared extract '{spell_id}' is not recorded in the formula book"
            ));
        }
    }

    let base_extracts_per_day = alchemist_base_extracts_per_day(level);
    for (index, base_count) in base_extracts_per_day.iter().enumerate() {
        let extract_level = (index + 1) as u8;
        let Some(base_count) = base_count else {
            if prepared
                .iter()
                .filter_map(|id| parse_alchemist_extract_id(id))
                .any(|l| l == extract_level)
            {
                unmet.push(format!(
                    "a prepared extract targets extract level {extract_level}, not yet \
                     accessible at alchemist level {level}"
                ));
            }
            continue;
        };
        let int_bonus = ability_bonus_spells(intelligence_modifier, i16::from(extract_level));
        let total_slots = base_count + int_bonus;
        let consumed: i16 = prepared
            .iter()
            .filter_map(|id| parse_alchemist_extract_id(id))
            .filter(|l| *l == extract_level)
            .count() as i16;
        if consumed > total_slots {
            unmet.push(format!(
                "extract level {extract_level} over-prepared: {consumed} extracts prepared but \
                 only {total_slots} slots available (base {base_count} + Intelligence bonus \
                 {int_bonus})"
            ));
        }
    }

    unmet
}

/// Ground the real prepared-extract / daily-preparation state once
/// `unmet_alchemist_extract_conditions` reports an empty unmet list
/// (deepening 2026-07-26, task #4). Mirrors
/// `ground_investigator_prepared_extracts`'s exact shape.
pub(super) fn ground_alchemist_prepared_extracts(
    input: &CharacterInput,
    level: u8,
    intelligence_modifier: i16,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let alchemist_spells = |mode: AcquisitionMode| {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(move |s| s.source_class_id == ALCHEMIST_CLASS_ID && s.acquisition_mode == mode)
    };
    let recorded: Vec<&str> = alchemist_spells(AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();
    let prepared: Vec<&str> = alchemist_spells(AcquisitionMode::Prepared)
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.apg.alchemist.formula_book_contents".to_owned(),
        value: recorded.len() as i16,
        detail: format!(
            "Alchemist level {level} recorded formula book contents ({} extracts, \
             AcquisitionMode::Known): {}. This grounds which extracts are recorded as real, \
             chosen input; it does not verify against any corpus that a named extract genuinely \
             exists or genuinely belongs to the level its own identifier claims",
            recorded.len(),
            recorded.join(", ")
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_spell.apg.alchemist.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Alchemist level {level} daily preparation selection ({} extracts, \
             AcquisitionMode::Prepared, each already verified recorded in the formula book \
             above): {}. Every prepared extract is drawn from the recorded formula book, \
             consuming its extract level's slot budget (one slot each -- Alchemist has no \
             opposed-school double-cost rule). It computes no extract-drinking execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let base_extracts_per_day = alchemist_base_extracts_per_day(level);
    for (index, base_count) in base_extracts_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let extract_level = (index + 1) as u8;
        let int_bonus = ability_bonus_spells(intelligence_modifier, i16::from(extract_level));
        let total = base_count + int_bonus;

        explanations.push(ComputationExplanation {
            id: format!(
                "class_spell.apg.alchemist.base_extracts_per_day.extract_level_{extract_level}"
            ),
            value: *base_count,
            detail: format!(
                "Alchemist level {level} base extracts per day at extract level \
                 {extract_level}: {base_count}, read directly from the real Extracts Prepared \
                 table (verified via d20pfsrd.com's own Alchemist page, byte-for-byte identical \
                 to Investigator's own table)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!(
                "class_spell.apg.alchemist.intelligence_bonus_extracts_per_day.extract_level_{extract_level}"
            ),
            value: int_bonus,
            detail: format!(
                "Alchemist level {level} Intelligence bonus extracts per day at extract level \
                 {extract_level}: {int_bonus} from Intelligence modifier {intelligence_modifier} \
                 (PF1 Core Rulebook Table: Ability Modifiers and Bonus Spells)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!(
                "class_spell.apg.alchemist.total_extracts_per_day.extract_level_{extract_level}"
            ),
            value: total,
            detail: format!(
                "Alchemist level {level} total extracts per day at extract level \
                 {extract_level}: base {base_count} + Intelligence bonus {int_bonus} = {total} \
                 (no specialist bonus slot -- Alchemist has no arcane school)"
            ),
        });

        let save_dc = alchemist_extract_save_dc(extract_level, intelligence_modifier);
        explanations.push(ComputationExplanation {
            id: format!("class_spell.apg.alchemist.extract_save_dc.extract_level_{extract_level}"),
            value: save_dc,
            detail: format!(
                "Alchemist level {level} extract level {extract_level} save DC: \
                 10 + {extract_level} + Intelligence modifier ({intelligence_modifier}) = \
                 {save_dc}"
            ),
        });
    }
}

/// Grounds or claim-blocks Alchemist's Mutagen execution engine for
/// `alchemist_level` (v0.6 alpha swarm, risks item 8, Alchemist Mutagen
/// closure). Called from `compute_apg_class_chassis`'s Alchemist branch,
/// gated only on Alchemist class-ownership.
///
/// A character who simply hasn't brewed/isn't currently mutated (no
/// `class_ability_activations` entry for `ALCHEMIST_MUTAGEN_ABILITY_ID`,
/// or one present but `active_state != EquippedActive`) is a genuinely
/// valid PF1 posture -- not every Alchemist is always mutated -- so this
/// grounds a real "not mutated" recognition record rather than
/// claim-blocking, mirroring "not raging"/"not singing"/"not
/// bloodraging" from the Rage-shaped classes. An activation that IS
/// active but has no recognized `choice:alchemist_mutagen_stat` selection
/// is a genuine posture violation (claiming to be mutated without saying
/// which stat is enhanced is inconsistent input, since brewing always
/// requires choosing a stat first per the corpus's own sequencing) and
/// claim-blocks, mirroring Sorcerer's own "recognized bloodline but no
/// bond choice given" shape.
pub(super) fn ground_or_block_alchemist_mutagen(
    input: &CharacterInput,
    alchemist_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(activation) = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == ALCHEMIST_MUTAGEN_ABILITY_ID)
    else {
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.alchemist.mutagen_execution.not_mutated".to_owned(),
            value: 0,
            detail: format!(
                "Alchemist level {alchemist_level} is not currently mutated (no \
                 class_ability_activations entry for \
                 \"{ALCHEMIST_MUTAGEN_ABILITY_ID}\"): a genuinely valid PF1 posture, so no \
                 ability-score bonus/penalty or natural armor bonus is claimed. This grounds \
                 the Mutagen execution engine's \"inactive\" branch only; being mutated is \
                 grounded separately below when an active activation with a recognized stat \
                 choice is present"
            ),
        });
        push_alchemist_other_features_deferred_diagnostic(input, alchemist_level, diagnostics);
        return;
    };

    match activation.active_state {
        ActiveState::EquippedActive => {
            let selection = choice_selection(input, ALCHEMIST_MUTAGEN_STAT_CHOICE_ID);
            let recognized = selection.and_then(|selection| {
                let physical_ability =
                    selection.strip_prefix(ABILITY_SELECTION_PREFIX).unwrap_or(selection);
                alchemist_mutagen_mental_penalty_target(physical_ability)
                    .map(|mental_ability| (physical_ability, mental_ability))
            });

            let Some((physical_ability, mental_ability)) = recognized else {
                diagnostics.push(ComputationDiagnostic {
                    id: "class_feature.apg.alchemist.mutagen_execution.stat_choice_missing"
                        .to_owned(),
                    message: format!(
                        "Alchemist level {alchemist_level} claims an active Mutagen \
                         ({ALCHEMIST_MUTAGEN_ABILITY_ID}) but has no recognized \
                         {ALCHEMIST_MUTAGEN_STAT_CHOICE_ID} selection naming Strength, \
                         Dexterity, or Constitution (got {selection:?}): brewing a mutagen \
                         always requires choosing a physical ability score first per the PF1 \
                         Advanced Player's Guide's own sequencing, so an active mutagen with no \
                         recognized stat choice is a genuine posture violation, not a silently \
                         passing one -- no ability-score bonus/penalty or natural armor bonus is \
                         claimed for this input"
                    ),
                    claim_blocking: true,
                });
                push_alchemist_other_features_deferred_diagnostic(input, alchemist_level, diagnostics);
                return;
            };

            let duration_minutes = alchemist_mutagen_duration_minutes(alchemist_level);
            explanations.push(ComputationExplanation {
                id: "class_feature.apg.alchemist.mutagen_execution.active".to_owned(),
                value: 0,
                detail: format!(
                    "Alchemist level {alchemist_level} is actively mutated, enhancing \
                     {physical_ability} (+{ALCHEMIST_MUTAGEN_STAT_BONUS} alchemical bonus, \
                     ability score) at the cost of {mental_ability} \
                     ({ALCHEMIST_MUTAGEN_STAT_PENALTY:+} penalty, ability score), plus a \
                     {ALCHEMIST_MUTAGEN_NATURAL_ARMOR_BONUS:+} natural armor bonus. The \
                     ability-score bonus/penalty are applied to the integrated ability \
                     modifiers, and the natural armor bonus to baseline Armor Class -- see \
                     apply_alchemist_mutagen_ability_bonuses and compute_combat_baseline"
                ),
            });
            explanations.push(ComputationExplanation {
                id: "class_feature.apg.alchemist.mutagen_execution.duration_minutes".to_owned(),
                value: duration_minutes,
                detail: format!(
                    "Alchemist level {alchemist_level} Mutagen duration: level * 10 = \
                     {duration_minutes} minutes. Informational only -- this codebase tracks no \
                     elapsed-time state, so this names the real duration value without \
                     simulating its passage"
                ),
            });
        }
        ActiveState::SelectedInactive | ActiveState::Absent => {
            explanations.push(ComputationExplanation {
                id: "class_feature.apg.alchemist.mutagen_execution.not_mutated".to_owned(),
                value: 0,
                detail: format!(
                    "Alchemist level {alchemist_level} has a \
                     \"{ALCHEMIST_MUTAGEN_ABILITY_ID}\" activation entry but it is not active \
                     for this snapshot: a genuinely valid PF1 posture (a dose may be brewed but \
                     not yet drunk), so no ability-score bonus/penalty or natural armor bonus is \
                     claimed"
                ),
            });
        }
    }

    push_alchemist_other_features_deferred_diagnostic(input, alchemist_level, diagnostics);
}

/// Pushes the new, narrower diagnostic replacing
/// `class_feature.apg.alchemist.unsupported` for Alchemist specifically
/// (v0.6 alpha swarm, risks item 8, Alchemist Mutagen closure): named
/// ONLY the genuinely still-missing pieces.
///
/// **Updated (deepening 2026-07-26, task #4)**: renamed from
/// `push_alchemist_spellcasting_deferred_diagnostic`
/// (`class_feature.apg.alchemist.spellcasting_deferred.unsupported`) now
/// that Bomb, Poison Resistance, and prepared-extract spellcasting are
/// all genuinely wired -- mirrors Investigator's own diagnostic-honesty
/// fix (`other_features_deferred` replacing `spellcasting_deferred`
/// once spellcasting stopped being entirely ungrounded). Spellcasting
/// now has its own separate
/// `class_spell.apg.alchemist.prepared_extracts.unsupported` diagnostic
/// (mirroring `class_spell.acg.investigator.prepared_extracts.unsupported`).
/// Pushed from every branch of `ground_or_block_alchemist_mutagen`
/// regardless of Mutagen's own active state, mirroring Skald's/
/// Bloodrager's own diagnostic-honesty fix.
///
/// **Canonical narrowing (v0.6 alpha swarm, Alchemist spellcasting-shaped
/// closure).** This used to claim-block unconditionally. It now mirrors
/// `ground_or_block_arcanist_metamagic_knowledge`'s established shape: a
/// recognized Discovery choice grounds one real, corpus-verified option
/// out of the 35 (`ground_alchemist_feral_mutagen_discovery`) and
/// downgrades this to a NON-blocking note naming the other 34 plus the
/// rest of the honest remainder; no recognized Discovery keeps it
/// claim-blocking, unchanged.
pub(super) fn push_alchemist_other_features_deferred_diagnostic(
    input: &CharacterInput,
    level: u8,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let discovery_recognized = choice_selection(input, ALCHEMIST_DISCOVERY_CHOICE_ID)
        == Some(FERAL_MUTAGEN_DISCOVERY_SELECTION);
    let discovery_granted_yet = level >= ALCHEMIST_DISCOVERY_GRANT_LEVEL;

    let remainder = "the other 34 Discoveries (a chooser-list of real mechanical variety, 35 \
         shared `KEY:Discovery ~ ...` records in all), Grand Discovery, Swift Alchemy, Swift \
         Poisoning, Instant Alchemy, Persistent Mutagen, Poison Use, the Brew Potion and Throw \
         Anything bonus-feat grants, and Mutagen's own second and third stat-bonus tiers \
         (`SecondMutagenStatBonus`/`ThirdMutagenStatBonus` -- only the flat first-tier +4/-2 is \
         grounded) remain ungrounded anywhere in this codebase; no class-feature or spell \
         execution is fabricated in this bounded chassis baseline. This message previously \
         asserted \"this APG class has no class-skill list\": Alchemist's own `KEY:Alchemist ~ \
         Class Skills` corpus record exists -- it simply contains none of the three skills this \
         codebase tracks, so the resulting zero bonus is correct for a different reason than \
         the one claimed (task #76)";

    let grounded = "its base-attack-bonus/base-save chassis pillar, Mutagen's first stat-bonus \
         tier, Bomb, Poison Resistance, Formulae's own formula-book contents, and (subject to \
         its own real prepared-extract validation) spellcasting";

    let message = if discovery_recognized && discovery_granted_yet {
        format!(
            "{ALCHEMIST_CLASS_ID} has {grounded}, plus one real, corpus-verified Discovery \
             chosen from its own 35-record chooser (Feral Mutagen: its claw/bite damage dice \
             and its +{FERAL_MUTAGEN_INTIMIDATE_BONUS} competence bonus on Intimidate while \
             mutated) -- the canonical narrowing this codebase applies to every large class \
             chooser (Cleric's Good domain, Wizard's Evocation school, Oracle's Mystery, \
             Arcanist's Metamagic Knowledge). What is deferred, honestly and non-blockingly: \
             {remainder}"
        )
    } else if discovery_recognized {
        format!(
            "{ALCHEMIST_CLASS_ID} has {grounded}. Its Discovery slot carries this codebase's \
             one grounded Discovery choice (Feral Mutagen), which is correctly inert at level \
             {level}: the corpus pool is `AlchemistDiscoveryLVL/2`, so the first Discovery is \
             not granted until alchemist level {ALCHEMIST_DISCOVERY_GRANT_LEVEL} and there is \
             no Discovery gap to block on at this level at all. What is deferred, honestly and \
             non-blockingly: {remainder}"
        )
    } else {
        format!(
            "{ALCHEMIST_CLASS_ID} remains blocked beyond {grounded}: no recognized Discovery \
             choice is present, and {remainder}"
        )
    };

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.apg.alchemist.other_features_deferred.unsupported".to_owned(),
        message,
        claim_blocking: !discovery_recognized,
    });
}

/// Grounds Alchemist's one canonical Discovery, Feral Mutagen (v0.6 alpha
/// swarm, Alchemist spellcasting-shaped closure), gated on a recognized
/// `choice:alchemist_discovery` selection plus the class's own
/// `AlchemistDiscoveryLVL/2` grant gate.
///
/// Every value is transcribed verbatim from `apg_abilities_class.lst`'s
/// own `KEY:Discovery ~ Feral Mutagen` record -- see
/// `FERAL_MUTAGEN_DISCOVERY_SELECTION`'s doc comment for the quoted DESC
/// and the `TEMPBONUS:PC|SKILL|Intimidate|2` token.
///
/// The damage dice ground unconditionally once the Discovery is granted,
/// the same shape Warpriest's Sacred Weapon base damage die already uses:
/// they are fixed properties of the Discovery at Medium size, not
/// activation-dependent quantities. What the mutagen's active state gates
/// is whether the attacks and the Intimidate bonus are *currently* held,
/// which is recorded separately -- mirroring Destructive Attacks' and
/// Strength Surge's own active/not-active split exactly.
///
/// Not integrated into a total, and honest about it: this codebase has no
/// natural-attack routine for the claws/bite to enter, and the +2
/// Intimidate is a `TEMPBONUS` that only exists while a mutagen is in
/// effect, which `compute_selected_skill_modifiers` does not model. Both
/// ground as standalone flat magnitudes under the same bar Bard's Bardic
/// Knowledge and Slayer's Track already ship under.
pub(super) fn ground_alchemist_feral_mutagen_discovery(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if level < ALCHEMIST_DISCOVERY_GRANT_LEVEL
        || choice_selection(input, ALCHEMIST_DISCOVERY_CHOICE_ID)
            != Some(FERAL_MUTAGEN_DISCOVERY_SELECTION)
    {
        return;
    }

    explanations.push(ComputationExplanation {
        id: "class_feature.apg.alchemist.discovery.feral_mutagen_claw_damage_die".to_owned(),
        value: FERAL_MUTAGEN_CLAW_DAMAGE_DIE,
        detail: format!(
            "Alchemist level {level} selected the Feral Mutagen discovery: whenever he imbibes \
             a mutagen he gains TWO claw attacks dealing \
             1d{FERAL_MUTAGEN_CLAW_DAMAGE_DIE} each (1d4 if Small -- this codebase's own \
             fixture race is Medium). Primary attacks made at his full base attack bonus. This \
             engine has no natural-attack routine and computes no weapon-damage total anywhere, \
             so the damage die is grounded as a standalone magnitude rather than folded into a \
             total that does not exist"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.alchemist.discovery.feral_mutagen_bite_damage_die".to_owned(),
        value: FERAL_MUTAGEN_BITE_DAMAGE_DIE,
        detail: format!(
            "Alchemist level {level} Feral Mutagen bite attack: \
             1d{FERAL_MUTAGEN_BITE_DAMAGE_DIE} (1d6 if Small). Same standalone-magnitude \
             posture as the claws above"
        ),
    });

    let mutated = active_alchemist_mutagen_bonus(input).is_some();
    if mutated {
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.alchemist.discovery.feral_mutagen_intimidate_bonus".to_owned(),
            value: FERAL_MUTAGEN_INTIMIDATE_BONUS,
            detail: format!(
                "Alchemist level {level} is currently mutated, so Feral Mutagen's \
                 +{FERAL_MUTAGEN_INTIMIDATE_BONUS} competence bonus on Intimidate checks is \
                 genuinely held (`TEMPBONUS:PC|SKILL|Intimidate|2`). Grounded as a standalone \
                 magnitude, NOT folded into this engine's real Intimidate total: it is a \
                 TEMPBONUS that exists only for the mutagen's duration, and \
                 compute_selected_skill_modifiers models no duration-scoped skill bonuses at \
                 all, so adding it to the total would claim a time-scoping this codebase does \
                 not enforce"
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.alchemist.discovery.feral_mutagen_not_mutated".to_owned(),
            value: 0,
            detail: format!(
                "Alchemist level {level} holds the Feral Mutagen discovery but is not currently \
                 mutated: a genuinely valid PF1 posture (the discovery only takes effect \
                 \"whenever the alchemist imbibes a mutagen\"), so no claw/bite attacks and no \
                 +{FERAL_MUTAGEN_INTIMIDATE_BONUS} Intimidate bonus are claimed as currently \
                 held. The dice above name what the discovery grants, not what is active now"
            ),
        });
    }
}

/// Applies Alchemist Mutagen's ability-score bonus/penalty to `base` when
/// a valid, active, in-choice Mutagen activation is present for this
/// character (v0.6 alpha swarm, risks item 8, Alchemist Mutagen closure).
/// Mirrors `apply_rage_ability_bonuses`'s own pattern (layering onto
/// `compute_ability_modifiers`'s output rather than baking into it), but
/// unlike every Rage-shaped bonus, the TARGET ability is chosen input,
/// not fixed -- `assign_modifier`/`ability_modifier_for` (the same
/// generic-by-name helpers `compute_ability_modifiers` itself uses) apply
/// the bonus/penalty to whichever ability `active_alchemist_mutagen_bonus`
/// names, rather than a hardcoded field access.
pub(super) fn apply_alchemist_mutagen_ability_bonuses(
    base: AbilityModifiers,
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) -> AbilityModifiers {
    let Some((alchemist_level, physical_ability, mental_ability)) =
        active_alchemist_mutagen_bonus(input)
    else {
        return base;
    };

    let physical_modifier_bonus = ALCHEMIST_MUTAGEN_STAT_BONUS / 2;
    let mental_modifier_penalty = ALCHEMIST_MUTAGEN_STAT_PENALTY / 2;
    let mut modifiers = base;
    let new_physical_modifier =
        ability_modifier_for(&modifiers, physical_ability) + physical_modifier_bonus;
    assign_modifier(&mut modifiers, physical_ability, new_physical_modifier);
    let new_mental_modifier =
        ability_modifier_for(&modifiers, mental_ability) + mental_modifier_penalty;
    assign_modifier(&mut modifiers, mental_ability, new_mental_modifier);

    explanations.push(ComputationExplanation {
        id: "ability_modifier.alchemist.mutagen_bonus_applied".to_owned(),
        value: physical_modifier_bonus,
        detail: format!(
            "Alchemist level {alchemist_level} Mutagen applied to ability modifiers: \
             +{ALCHEMIST_MUTAGEN_STAT_BONUS} {physical_ability} alchemical bonus (ability score) \
             is +{physical_modifier_bonus} {physical_ability} modifier, and \
             {ALCHEMIST_MUTAGEN_STAT_PENALTY:+} {mental_ability} penalty (ability score) is \
             {mental_modifier_penalty:+} {mental_ability} modifier (an even score bonus/penalty \
             always halves exactly onto the floored modifier). Applied only while actively, \
             validly mutated with a recognized stat choice; the \
             {ALCHEMIST_MUTAGEN_NATURAL_ARMOR_BONUS:+} natural armor bonus is layered onto \
             compute_combat_baseline separately"
        ),
    });
    modifiers
}

/// Applies Alchemist Mutagen's natural armor bonus to `base_armor_class`
/// when `input` is an Alchemist actively, validly mutated with a
/// recognized stat choice (v0.6 alpha swarm, risks item 8, Alchemist
/// Mutagen closure). Mirrors Barbarian's/Skald's/Bloodrager's own inline
/// `if active_<x>_bonus(...).is_some() { BONUS } else { 0 }` shape at
/// `compute_combat_baseline`'s own call site.
pub(super) fn apply_alchemist_mutagen_ac_bonus_to_combat_baseline(input: &CharacterInput) -> i16 {
    if active_alchemist_mutagen_bonus(input).is_some() {
        ALCHEMIST_MUTAGEN_NATURAL_ARMOR_BONUS
    } else {
        0
    }
}

/// PF1 Advanced Class Guide Investigator Trapfinding: `max(InvestigatorLVL/2,1)`,
/// verified directly against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|InvestigatorTrapfindingBonus|max(InvestigatorLVL/2,1)`. **Has**
/// the floor -- do not confuse with Investigator's own Trap Sense, which
/// does not (see `investigator_trap_sense_bonus`'s own doc comment for
/// the real swapped-floors hazard between this class and Slayer). A
/// bonus on Perception (to locate traps) and Disable Device -- neither
/// tracked by `compute_selected_skill_modifiers`, so this grounds as a
/// standalone flat record, the same idiom as Slayer's own Trapfinding.
pub(super) fn investigator_trapfinding_bonus(level: u8) -> i16 {
    (i16::from(level) / 2).max(1)
}

/// PF1 Advanced Class Guide Investigator Trap Sense: `InvestigatorLVL/3`,
/// verified directly against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|TrapSenseBonus|InvestigatorLVL/3`. **Does NOT** have a
/// `max(1,...)` floor -- genuinely different from Slayer's own Trap
/// Sense (`max(1,SlayerTrapSenseLVL/3)`, `slayer_trap_sense_bonus`
/// above), even though both records share the same `TrapSenseBonus`
/// `BONUS:VAR` name. The floors are effectively swapped between the two
/// classes' two features (Investigator's Trapfinding has the floor,
/// Trap Sense doesn't; Slayer's Trap Sense has the floor, Trapfinding
/// doesn't) -- verified each formula against its own class's own
/// record directly, not copied from the other, per the real
/// implementation hazard the scoping doc flagged.
pub(super) fn investigator_trap_sense_bonus(level: u8) -> i16 {
    i16::from(level) / 3
}

/// PF1 Advanced Class Guide Investigator Inspiration pool size:
/// `max(1,InvestigatorLVL/2+INT)`, verified directly against
/// `acg_abilities_class.lst`'s own
/// `BONUS:VAR|InvestigatorInspirationPoolBonus|max(1,InvestigatorLVL/2+INT)`
/// (die `1d6`, verified via the same record's own
/// `InvestigatorInspirationDice|1`/`InvestigatorInspirationDieSize|6`
/// tags). Grounds only the flat, choice-free daily pool SIZE, the same
/// "pool size only" MVP shape as Swashbuckler's own Panache -- the
/// pool's USE (spend one use as a free action to add 1d6 to a skill/
/// ability check, two uses for an attack roll or saving throw, or the
/// free Knowledge/Linguistics/Spellcraft interaction) ties into per-roll
/// resolution this codebase has no surface for, and stays deferred.
///
/// The `Extra Inspiration` feat contributes `3` to the same variable
/// (`BONUS:VAR|InvestigatorInspirationPoolBonus|3`). PCGen sums
/// independent `BONUS:VAR` contributions to one variable, so the feat's
/// `3` lands OUTSIDE the class record's own `max(1, ...)` clamp, not
/// inside it -- the clamp belongs to the class formula alone.
pub(super) fn investigator_inspiration_pool_size(
    level: u8,
    intelligence_modifier: i16,
    selected_feats: &[String],
) -> i16 {
    (i16::from(level) / 2 + intelligence_modifier).max(1)
        + extra_resource_feat_bonus(
            selected_feats,
            EXTRA_INSPIRATION_FEAT_KEY,
            EXTRA_INSPIRATION_USES,
        )
}

/// PF1 Advanced Class Guide Investigator Poison Resistance's numeric
/// bonus tier (deepening 2026-07-26, task #8), verified directly against
/// `acg_abilities_class.lst`'s own internal `InvestigatorPoisonLVL`
/// tier-gating tokens rather than assumed from a general "poison
/// resistance progression" pattern: `None` below level 2 (not yet
/// gained), `Some(2)` from level 2, `Some(4)` from level 5, `Some(6)`
/// from level 8. **Correction to the scoping doc's own claim** ("+2/+4/
/// +6/+8 scaling, immunity at 20th"): the raw corpus has no level-20
/// tier and no +8 step at all -- the real progression tops out at +6
/// (level 8-9), then converts to full immunity at level 10 (see
/// `investigator_is_poison_immune`), not a fourth numeric tier. Poison
/// Lore and Poison Resistance both begin at 2nd level per the corpus's
/// own `PREVARGTEQ:InvestigatorLVL,2` gate on the first tier.
pub(super) fn investigator_poison_resistance_bonus(level: u8) -> Option<i16> {
    if level < 2 {
        None
    } else if level < 5 {
        Some(2)
    } else if level < 8 {
        Some(4)
    } else {
        Some(6)
    }
}

/// Whether `level` has reached full poison immunity (deepening
/// 2026-07-26, task #8), verified directly against the corpus's own
/// `DESC:You are completely immune to poison.|PREVAREQ:
/// InvestigatorPoisonLVL,4` gate, which resolves to Investigator level
/// 10 (the fourth and final tier step). A qualitatively different fact
/// from the numeric resistance bonus above, not a fourth scaling step.
pub(super) fn investigator_is_poison_immune(level: u8) -> bool {
    level >= 10
}

/// PF1 Advanced Class Guide Investigator Alchemy: "you gain a
/// competence bonus equal to [Investigator level] on the [Craft
/// (alchemy)] skill check" -- verified directly against
/// `acg_abilities_class.lst`'s own `BONUS:VAR|
/// InvestigatorAlchemyCreationBonus|InvestigatorLVL` (deepening
/// 2026-07-26, task #8). A flat competence bonus with no computed
/// total (Craft (alchemy) is not among the skills this codebase
/// computes), the same shape as Bard's own Bardic Knowledge.
pub(super) fn investigator_alchemy_creation_bonus(level: u8) -> i16 {
    i16::from(level)
}

/// PF1 Advanced Class Guide Investigator "Extracts Prepared" table
/// (deepening 2026-07-26, task #8), for extract levels 1st through 6th,
/// indexed 0=1st..5=6th (Investigator has no 0th-level extracts, the
/// same "no cantrip slot" shape Skald's own base-spells-per-day table
/// uses). `acg_classes.lst` has no per-level `CAST:`/`KNOWN:` rows for
/// Investigator at all -- an external-source table, verified via THREE
/// independent sources before committing: aonprd.com's own Investigator
/// page, d20pfsrd.com's own Investigator page, and d20pfsrd.com's own
/// separate Alchemist page (confirming a general claim found while
/// searching -- "Investigators get just as many extracts per day as an
/// alchemist" -- with the real numbers, not just the claim). All three
/// agree byte-for-byte. Unlike Wizard/Arcanist, this closure grounds the
/// FULL 1-20 range, not bounded to level 3 -- that bound was an idiom
/// inherited from the original GE-06 pilot slice, not a genuine
/// verification limit (confirmed directly in review), and Investigator's
/// own base chassis already supports all 20 levels with real, fully-
/// verified data in hand for every one of them.
pub(super) fn investigator_base_extracts_per_day(level: u8) -> [Option<i16>; 6] {
    match level {
        1 => [Some(1), None, None, None, None, None],
        2 => [Some(2), None, None, None, None, None],
        3 => [Some(3), None, None, None, None, None],
        4 => [Some(3), Some(1), None, None, None, None],
        5 => [Some(4), Some(2), None, None, None, None],
        6 => [Some(4), Some(3), None, None, None, None],
        7 => [Some(4), Some(3), Some(1), None, None, None],
        8 => [Some(4), Some(4), Some(2), None, None, None],
        9 => [Some(5), Some(4), Some(3), None, None, None],
        10 => [Some(5), Some(4), Some(3), Some(1), None, None],
        11 => [Some(5), Some(4), Some(4), Some(2), None, None],
        12 => [Some(5), Some(5), Some(4), Some(3), None, None],
        13 => [Some(5), Some(5), Some(4), Some(3), Some(1), None],
        14 => [Some(5), Some(5), Some(4), Some(4), Some(2), None],
        15 => [Some(5), Some(5), Some(5), Some(4), Some(3), None],
        16 => [Some(5), Some(5), Some(5), Some(4), Some(3), Some(1)],
        17 => [Some(5), Some(5), Some(5), Some(4), Some(4), Some(2)],
        18 => [Some(5), Some(5), Some(5), Some(5), Some(4), Some(3)],
        19 => [Some(5), Some(5), Some(5), Some(5), Some(5), Some(4)],
        _ => [Some(5), Some(5), Some(5), Some(5), Some(5), Some(5)],
    }
}

/// PF1 Advanced Class Guide Investigator extract save DC: `10 + extract
/// level + Intelligence modifier` (deepening 2026-07-26, task #8),
/// verified directly against `acg_abilities_class.lst`'s own Alchemy
/// DESC text ("The saving throw DC for an investigator's extract is
/// equal to 10 + the extract's level + the investigator's Intelligence
/// modifier"), mirroring Arcanist's/Warpriest's own spell-save-DC
/// formula shape exactly (same `10 + spell level + casting ability
/// modifier` structure every prepared/spontaneous caster in this
/// codebase already uses).
pub(super) fn investigator_extract_save_dc(extract_level: u8, intelligence_modifier: i16) -> i16 {
    10 + i16::from(extract_level) + intelligence_modifier
}

/// Parses an Investigator extract's `spell_id` into its real Alchemist-
/// specific extract level (deepening 2026-07-26, task #8), by looking it
/// up directly in `alchemist_spell_list::ALCHEMIST_SPELL_LIST` --
/// Investigator's own `SPELLLIST:1|Alchemist` corpus token confirms this
/// is the real, shared list, not a separate one. Unlike
/// `parse_wizard_spellbook_spell_id`, this resolves no school at all:
/// Investigator has no arcane-school mechanic (confirmed: no "School"
/// record anywhere in its own `KEY:Investigator ~ ...` list), so every
/// prepared extract costs exactly 1 slot, the same simpler shape
/// Arcanist's own parser already established relative to Wizard's.
pub(super) fn parse_investigator_extract_id(spell_id: &str) -> Option<u8> {
    alchemist_spell_list::alchemist_spell_level(spell_id)
}

/// Return the list of unmet conditions for Investigator's prepared-
/// extract posture (deepening 2026-07-26, task #8). Mirrors
/// `unmet_arcanist_spellbook_conditions`'s exact shape: an empty list
/// means the posture is fully supported (an Investigator with at least
/// one extract recorded (`AcquisitionMode::Known`) and at least one
/// prepared today (`AcquisitionMode::Prepared`), every prepared extract
/// already recorded, and no extract level's prepared count exceeding
/// that level's total slot budget). No arcane-school mechanic exists for
/// Investigator (same as Arcanist), so every prepared extract costs
/// exactly 1 slot, never 2.
pub(super) fn unmet_investigator_extract_conditions(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Vec<String> {
    let mut unmet = Vec::new();

    let investigator_spells = |mode: AcquisitionMode| {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(move |s| s.source_class_id == INVESTIGATOR_CLASS_ID && s.acquisition_mode == mode)
    };
    let recorded: Vec<&str> = investigator_spells(AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();
    let prepared: Vec<&str> = investigator_spells(AcquisitionMode::Prepared)
        .map(|s| s.spell_id.as_str())
        .collect();

    if recorded.is_empty() {
        unmet.push(
            "no investigator extracts recorded in the formula book (AcquisitionMode::Known)"
                .to_owned(),
        );
    }
    if prepared.is_empty() {
        unmet.push("no investigator extracts prepared today (AcquisitionMode::Prepared)".to_owned());
    }

    for spell_id in &prepared {
        if !recorded.contains(spell_id) {
            unmet.push(format!(
                "prepared extract '{spell_id}' is not recorded in the formula book"
            ));
        }
    }

    let base_extracts_per_day = investigator_base_extracts_per_day(level);
    for (index, base_count) in base_extracts_per_day.iter().enumerate() {
        let extract_level = (index + 1) as u8;
        let Some(base_count) = base_count else {
            if prepared
                .iter()
                .filter_map(|id| parse_investigator_extract_id(id))
                .any(|l| l == extract_level)
            {
                unmet.push(format!(
                    "a prepared extract targets extract level {extract_level}, not yet \
                     accessible at investigator level {level}"
                ));
            }
            continue;
        };
        let int_bonus = ability_bonus_spells(ability_modifiers.intelligence, i16::from(extract_level));
        let total_slots = base_count + int_bonus;
        let consumed: i16 = prepared
            .iter()
            .filter_map(|id| parse_investigator_extract_id(id))
            .filter(|l| *l == extract_level)
            .count() as i16;
        if consumed > total_slots {
            unmet.push(format!(
                "extract level {extract_level} over-prepared: {consumed} extracts prepared but \
                 only {total_slots} slots available (base {base_count} + Intelligence bonus \
                 {int_bonus})"
            ));
        }
    }

    unmet
}

/// Ground the real prepared-extract / daily-preparation state once
/// `unmet_investigator_extract_conditions` reports an empty unmet list
/// (deepening 2026-07-26, task #8): the recorded formula book contents,
/// the daily preparation selection, the base/Intelligence-bonus/total
/// extracts-per-day counts per accessible extract level, and the extract
/// save DC. Mirrors `ground_arcanist_prepared_spellbook`'s own shape.
pub(super) fn ground_investigator_prepared_extracts(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let investigator_spells = |mode: AcquisitionMode| {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(move |s| s.source_class_id == INVESTIGATOR_CLASS_ID && s.acquisition_mode == mode)
    };
    let recorded: Vec<&str> = investigator_spells(AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();
    let prepared: Vec<&str> = investigator_spells(AcquisitionMode::Prepared)
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.acg.investigator.formula_book_contents".to_owned(),
        value: recorded.len() as i16,
        detail: format!(
            "Investigator level {level} recorded formula book contents ({} extracts, \
             AcquisitionMode::Known): {}. This grounds which extracts are recorded as real, \
             chosen input; it does not verify against any corpus that a named extract genuinely \
             exists or genuinely belongs to the level its own identifier claims",
            recorded.len(),
            recorded.join(", ")
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_spell.acg.investigator.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Investigator level {level} daily preparation selection ({} extracts, \
             AcquisitionMode::Prepared, each already verified recorded in the formula book \
             above): {}. Every prepared extract is drawn from the recorded formula book, \
             consuming its extract level's slot budget (one slot each -- Investigator has no \
             opposed-school double-cost rule). It computes no extract-drinking execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let base_extracts_per_day = investigator_base_extracts_per_day(level);
    for (index, base_count) in base_extracts_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let extract_level = (index + 1) as u8;
        let int_bonus = ability_bonus_spells(ability_modifiers.intelligence, i16::from(extract_level));
        let total = base_count + int_bonus;

        explanations.push(ComputationExplanation {
            id: format!(
                "class_spell.acg.investigator.base_extracts_per_day.extract_level_{extract_level}"
            ),
            value: *base_count,
            detail: format!(
                "Investigator level {level} base extracts per day at extract level \
                 {extract_level}: {base_count}, read directly from the real Extracts Prepared \
                 table (verified via aonprd.com, d20pfsrd.com's own Investigator page, and \
                 d20pfsrd.com's own Alchemist page -- all three agree byte-for-byte)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!(
                "class_spell.acg.investigator.intelligence_bonus_extracts_per_day.extract_level_{extract_level}"
            ),
            value: int_bonus,
            detail: format!(
                "Investigator level {level} Intelligence bonus extracts per day at extract \
                 level {extract_level}: {int_bonus} from Intelligence modifier {} (PF1 Core \
                 Rulebook Table: Ability Modifiers and Bonus Spells)",
                ability_modifiers.intelligence
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!(
                "class_spell.acg.investigator.total_extracts_per_day.extract_level_{extract_level}"
            ),
            value: total,
            detail: format!(
                "Investigator level {level} total extracts per day at extract level \
                 {extract_level}: base {base_count} + Intelligence bonus {int_bonus} = {total} \
                 (no specialist bonus slot -- Investigator has no arcane school)"
            ),
        });

        let save_dc = investigator_extract_save_dc(extract_level, ability_modifiers.intelligence);
        explanations.push(ComputationExplanation {
            id: format!("class_spell.acg.investigator.extract_save_dc.extract_level_{extract_level}"),
            value: save_dc,
            detail: format!(
                "Investigator level {level} extract level {extract_level} save DC: \
                 10 + {extract_level} + Intelligence modifier ({}) = {save_dc}",
                ability_modifiers.intelligence
            ),
        });
    }
}

/// Grounds Investigator's class features for `level` (v0.6 alpha swarm,
/// risks item 8, Investigator full-build closure, 10th ACG/APG class-
/// specific closure, no-spellcasting MVP). Called from
/// `compute_acg_class_chassis`'s Investigator branch, gated only on
/// Investigator class-ownership. All three sub-features are flat,
/// always-on class features (not activation-gated, not choice-gated) --
/// grounds each as its own standalone explanation record, then pushes
/// the narrowed `other_features_deferred` diagnostic naming spellcasting
/// (deferred to its own follow-on slice pending an Alchemist formula
/// spell list), Inspiration's use, and Investigator Talents (a chooser-
/// list) as the genuinely still-missing pieces.
/// Investigator's Studied Combat insight bonus: `InvestigatorLVL/2`, on
/// attack and damage rolls against the studied creature. Same
/// own-level-only shape as Slayer's Studied Target.
pub(super) fn investigator_studied_combat_bonus(level: u8) -> i16 {
    i16::from(level) / 2
}

/// Studied Combat's duration in rounds: `max(1, INT)`.
///
/// Grounded as a FACT but explicitly NOT enforced -- this engine has no
/// round clock, the same honest gap already named for Bloodrage's
/// post-rage fatigue.
pub(super) fn investigator_studied_combat_duration(intelligence_modifier: i16) -> i16 {
    intelligence_modifier.max(1)
}

/// Studied Strike's damage dice: `min(9, (InvestigatorLVL-2)/2)` d6.
/// Genuinely 0 below 4th level, reaching 9d6 at 20th.
///
/// **The `clamp(0, 9)` is inert across the whole real level range** and
/// is kept only for fidelity to the corpus token. `(20-2)/2` is exactly
/// 9, so the upper bound never binds; and Rust's truncating division
/// makes `(1-2)/2` evaluate to 0 rather than -1, so the lower bound
/// never binds either. Removing the clamp changes no value an
/// Investigator can reach, which means a test cannot distinguish the
/// clamped from the unclamped form.
///
/// This is the SECOND instance of this shape in as many closures --
/// Brawler's Flurry `min(...,3)` cap is the same -- so it is worth
/// recognising as a pattern: PF1 formulas routinely carry defensive
/// bounds that the class's own 1-20 range never reaches. Transcribe
/// them, but do not mistake a test that exercises the bounded values for
/// coverage OF the bound.
pub(super) fn investigator_studied_strike_dice(level: u8) -> i16 {
    let raw = (i16::from(level) - 2) / 2;
    raw.clamp(0, 9)
}

pub(super) fn ground_or_block_investigator_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let studied_combat = investigator_studied_combat_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.investigator.studied_combat_bonus".to_owned(),
        value: studied_combat,
        detail: format!(
            "Investigator level {level} Studied Combat: a +{studied_combat} insight bonus \
             (level/2) on attack and damage rolls against the creature he is studying. Grounds \
             standalone -- the formula reads only the investigator's own level, nothing about \
             the opponent, the same basis on which Slayer's Sneak Attack dice already grounds \
             despite being flanking-conditional. Which creature is studied is not modelled"
        ),
    });
    let duration = investigator_studied_combat_duration(ability_modifiers.intelligence);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.investigator.studied_combat_duration".to_owned(),
        value: duration,
        detail: format!(
            "Investigator level {level} Studied Combat lasts {duration} round(s) (max(1, \
             Intelligence modifier {})). This duration is grounded as a fact but is NOT \
             enforced: this engine has no round clock, so nothing expires it -- named honestly \
             rather than silently modelled, the same idiom already used for Bloodrage's \
             post-rage fatigue",
            ability_modifiers.intelligence
        ),
    });

    let strike_dice = investigator_studied_strike_dice(level);
    if strike_dice > 0 {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.investigator.studied_strike_dice".to_owned(),
            value: strike_dice,
            detail: format!(
                "Investigator level {level} Studied Strike: {strike_dice}d6 extra damage \
                 (min(9, (level - 2)/2), so it genuinely does not exist below 4th level and caps \
                 at 9d6). A weapon-damage magnitude, the same idiom as Alchemist's Bomb and \
                 Swashbuckler's Precise Strike"
            ),
        });
    }

    if level >= INVESTIGATOR_STUDIED_DEFENSE_LEVEL
        && input.chosen.selected_choices.iter().any(|c| {
            c.choice_set_id == INVESTIGATOR_STUDIED_DEFENSE_CHOICE_ID
                && c.selection_id == INVESTIGATOR_STUDIED_DEFENSE_SELECTION
        })
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.investigator.studied_defense_ac_bonus".to_owned(),
            value: studied_combat,
            detail: format!(
                "Investigator level {level} Studied Defense: the SAME +{studied_combat} insight \
                 bonus is applied to Armor Class against the studied target instead of to attack \
                 rolls against it. One magnitude, two destinations, chosen by the player -- so it \
                 requires an explicit recorded choice and is never seeded, the same \
                 no-silent-seeding design ratified for Skill Focus. The damage half of Studied \
                 Combat is unaffected either way. Gated at investigator level \
                 {INVESTIGATOR_STUDIED_DEFENSE_LEVEL} by the talent's own prerequisite, which \
                 requires nine investigator talent levels"
            ),
        });
    }

    // Resiliency (task #58, v0.6 alpha swarm): Investigator's OWN separate
    // copy of the Rogue Talent, drawn from its explicit 40-record
    // `KEY:Investigator ~ Rogue Talent ~ *` whitelist. Genuinely different
    // corpus record from Rogue's own copy -- `BONUS:VAR|ResiliencyHitPoints|
    // InvestigatorLVL`, not `RogueTalentLVL` -- gated on Investigator's own
    // talent chooser and its own real 3/5/7/9... grant cadence, not Rogue's.
    if level >= INVESTIGATOR_TALENT_GRANT_LEVEL
        && choice_selection(input, INVESTIGATOR_TALENT_CHOICE_ID) == Some(RESILIENCY_TALENT_SELECTION)
    {
        let temp_hp = crate::rules_core::durability::investigator_resiliency_temp_hp(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.investigator.resiliency_temp_hp".to_owned(),
            value: temp_hp,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   , `BONUS:VAR|ResiliencyHitPoints|InvestigatorLVL`, a separate corpus record
                //   from Rogue's own copy, no PRE gate
                "Investigator level {level} selected the Resiliency talent (from her own Rogue \
                 Talent whitelist): once per day, when brought below 0 hit points, she can gain \
                 {temp_hp} temporary hit points (equal to her investigator level) as an immediate \
                 action, lasting 1 minute. This grounds the magnitude only: the once-per-day budget \
                 and the \"brought below 0 hit points\" trigger are named but not enforced -- no \
                 once-per-day activation tracker and no HP-threshold trigger engine exists anywhere \
                 in this codebase (the same honest-gap idiom already used for Skald's Raging Song \
                 rounds-per-day). No temporary-hit-point total exists anywhere in this codebase or \
                 its downstream apps/desktop/src-tauri character_hub.rs consumer (which tracks only \
                 max_hp/current_hp), so this grounds as a standalone flat magnitude rather than a \
                 false integration claim, the same honest-gap idiom already used for Witch's Ward \
                 hex"
            ),
        });
    }

    let trapfinding_bonus = investigator_trapfinding_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.investigator.trapfinding_bonus".to_owned(),
        value: trapfinding_bonus,
        detail: format!(
            "Investigator level {level} Trapfinding: a +{trapfinding_bonus} bonus on Perception \
             checks made to locate traps and Disable Device checks (max(level/2, 1) = \
             {trapfinding_bonus}). Neither Perception nor Disable Device is among the three \
             skills compute_selected_skill_modifiers tracks (Climb/Intimidate/Swim), so this \
             grounds as a standalone flat record"
        ),
    });

    let trap_sense_bonus = investigator_trap_sense_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.investigator.trap_sense_bonus".to_owned(),
        value: trap_sense_bonus,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR name
            "Investigator level {level} Trap Sense: a +{trap_sense_bonus} bonus on Reflex saves made \
             to avoid traps and a +{trap_sense_bonus} dodge bonus to AC against attacks made by \
             traps (level/3 = {trap_sense_bonus}, no floor -- genuinely different from Slayer's own \
             floored Trap Sense despite sharing the same corpus). This codebase has no trap-specific \
             AC/save pillar; grounded as a standalone flat record"
        ),
    });

    let inspiration_pool_size = investigator_inspiration_pool_size(
        level,
        ability_modifiers.intelligence,
        &input.chosen.selected_feats,
    );
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.investigator.inspiration_pool_size".to_owned(),
        value: inspiration_pool_size,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   The feat's 3 lands outside the max(1, ...) clamp because the corpus applies it as a
            //   separate BONUS:VAR contribution to the same variable, not as a term inside the
            //   class formula.
            "Investigator level {level} Inspiration pool size: max(1, level/2 + Intelligence \
             modifier ({})) + Extra Inspiration feat ({:+}) = {inspiration_pool_size} points at the \
             start of each day, each worth 1d6 when spent. Grounds only the flat daily maximum -- \
             spending a use on a skill/ability/attack/save roll, and the free \
             Knowledge/Linguistics/Spellcraft interaction, are not modeled (no per-roll resolution \
             surface exists in this codebase)",
            ability_modifiers.intelligence,
            extra_resource_feat_bonus(
                &input.chosen.selected_feats,
                EXTRA_INSPIRATION_FEAT_KEY,
                EXTRA_INSPIRATION_USES
            )
        ),
    });

    match investigator_poison_resistance_bonus(level) {
        None => {
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.investigator.poison_resistance_bonus".to_owned(),
                value: 0,
                detail: format!(
                    "Investigator level {level} Poison Resistance: correctly absent below level \
                     2 by PF1 Advanced Class Guide level gate; the at-grant magnitude is named \
                     but not computed"
                ),
            });
        }
        Some(bonus) if investigator_is_poison_immune(level) => {
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.investigator.poison_resistance_bonus".to_owned(),
                value: 0,
                detail: format!(
                    "Investigator level {level} is fully immune to poison (PF1 Advanced Class \
                     Guide, granted at level 10): a qualitatively different fact from the \
                     numeric resistance bonus (which topped out at +{bonus} at level 8-9), not a \
                     fourth scaling tier, so no numeric bonus value applies here"
                ),
            });
        }
        Some(bonus) => {
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.investigator.poison_resistance_bonus".to_owned(),
                value: bonus,
                detail: format!(
                    "Investigator level {level} Poison Resistance: a +{bonus} bonus on all \
                     saving throws against poison (2 at level 2, 4 at level 5, 6 at level 8, \
                     immune at level 10). No poison-save total exists anywhere in this \
                     codebase, so this grounds only the flat bonus value"
                ),
            });
        }
    }

    let alchemy_bonus = investigator_alchemy_creation_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.investigator.alchemy_bonus".to_owned(),
        value: alchemy_bonus,
        detail: format!(
            "Investigator level {level} Alchemy: a +{alchemy_bonus} competence bonus on Craft \
             (alchemy) checks to create mundane alchemical items (equal to Investigator level). \
             No Craft (alchemy) total exists anywhere in this codebase, so this grounds only \
             the flat bonus value, mirroring Bard's own Bardic Knowledge"
        ),
    });

    let extract_unmet = unmet_investigator_extract_conditions(input, level, ability_modifiers);
    if extract_unmet.is_empty() {
        ground_investigator_prepared_extracts(input, level, ability_modifiers, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.acg.investigator.prepared_extracts.unsupported".to_owned(),
            message: format!(
                "Investigator remains blocked on its prepared extract / daily preparation / \
                 extract slot posture burden: {}",
                extract_unmet.join("; ")
            ),
            claim_blocking: true,
        });
    }

    push_investigator_other_features_deferred_diagnostic(input, level, diagnostics);
}

/// Pushes the new, narrower diagnostic replacing
/// `class_feature.acg.investigator.unsupported` for Investigator
/// specifically (v0.6 alpha swarm, risks item 8, Investigator full-
/// build closure): named ONLY the genuinely still-missing pieces.
///
/// **Updated (deepening 2026-07-26, task #8)**: this diagnostic no
/// longer claims spellcasting is entirely ungrounded -- prepared extract
/// validation is now real (`unmet_investigator_extract_conditions`/
/// `ground_investigator_prepared_extracts`, reusing the shared
/// `alchemist_spell_list` module), gated behind its own separate
/// `class_spell.acg.investigator.prepared_extracts.unsupported`
/// diagnostic (mirroring Arcanist's own
/// `class_spell.acg.arcanist.prepared_spellbook.unsupported`). Pushed
/// exactly once from the top-level Investigator branch, independent of
/// the extract posture's own state.
/// **Canonical narrowing (v0.6 alpha swarm, Investigator
/// spellcasting-shaped closure).** This used to claim-block
/// unconditionally, which flattened two genuinely different characters
/// into one report: an Investigator who HAD filled her talent slot with
/// this codebase's one grounded talent (Resiliency, task #58) read
/// identically to one who had chosen nothing. It now mirrors
/// `ground_or_block_arcanist_metamagic_knowledge`'s established shape --
/// a recognized talent choice grounds a real class-feature option and
/// downgrades this to a NON-blocking note naming the honest remainder; no
/// recognized choice keeps it claim-blocking, unchanged.
pub(super) fn push_investigator_other_features_deferred_diagnostic(
    input: &CharacterInput,
    level: u8,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let talent_recognized =
        choice_selection(input, INVESTIGATOR_TALENT_CHOICE_ID) == Some(RESILIENCY_TALENT_SELECTION);
    let talent_granted_yet = level >= INVESTIGATOR_TALENT_GRANT_LEVEL;

    let remainder = "Inspiration's actual spend (a free/two-use action on skill/ability/attack/\
         save rolls, plus the free Knowledge/Linguistics/Spellcraft interaction), the remainder \
         of Investigator Talents (a chooser-list of real mechanical variety including the large \
         Rogue Talent and Discovery sub-lists, Resiliency alone excepted), Keen Recollection, \
         Poison Lore, Swift Alchemy, and True Inspiration remain ungrounded anywhere in this \
         codebase; no class-feature or spell execution is fabricated in this bounded chassis \
         baseline. Studied Combat and Studied Strike are NOT among them: their bonus, duration, \
         Studied Defense AC bonus and strike dice are all grounded, and this message previously \
         deferred them \"pending an opponent-tracking pillar, ruled consistently with Slayer's \
         own Studied Target\" -- a ruling that no longer matches either class's shipped state. \
         As with Slayer, what remains unmodelled for both is the APPLICATION against a specific \
         studied opponent, not the magnitudes";

    let grounded = "its base-attack-bonus/base-save chassis pillar, its class-skill list, \
         Trapfinding, Trap Sense, Inspiration's flat pool-size fact, Poison Resistance, \
         Alchemy, Studied Combat/Studied Strike/Studied Defense, and (subject to its own real \
         prepared-extract validation) spellcasting";

    let message = if talent_recognized && talent_granted_yet {
        format!(
            "{INVESTIGATOR_CLASS_ID} has {grounded}, plus Resiliency's own temporary-hit-point \
             magnitude, recognized through her own separate Rogue Talent whitelist choice slot \
             (task #58) -- the canonical narrowing this codebase applies to every large class \
             chooser (Cleric's Good domain, Wizard's Evocation school, Oracle's Mystery, \
             Arcanist's Metamagic Knowledge): one corpus-verified option grounded for real, the \
             other 39 whitelist entries named rather than fabricated. What is deferred, \
             honestly and non-blockingly: {remainder}"
        )
    } else if talent_recognized {
        format!(
            "{INVESTIGATOR_CLASS_ID} has {grounded}. Her talent slot carries this codebase's \
             one grounded talent choice (Resiliency), which is correctly inert at level \
             {level}: Investigator's first talent is not granted until level \
             {INVESTIGATOR_TALENT_GRANT_LEVEL}, so there is no talent gap to block on at this \
             level at all -- the recorded choice simply takes effect when the slot opens. What \
             is deferred, honestly and non-blockingly: {remainder}"
        )
    } else {
        format!(
            "{INVESTIGATOR_CLASS_ID} remains blocked beyond {grounded}: no recognized \
             Investigator Talent choice is present, and {remainder}"
        )
    };

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.investigator.other_features_deferred.unsupported".to_owned(),
        message,
        claim_blocking: !talent_recognized,
    });
}

/// v0.6 alpha swarm, risks item 8 (Alchemist Mutagen closure, second APG
/// class-specific closure, 2026-07-25): the first mechanic this session
/// combines the choice-recognition pattern (Sorcerer's Arcane Bond /
/// Cleric's domain choice) with the activation-gating pattern
/// (Barbarian's Rage) -- both independently proven, combined here for
/// the first time. Like every other choice-gated closure, Alchemist
/// still never reaches `Computed` (spellcasting/other features stay
/// deferred), so this mirrors `bloodrager_dispatch_widening_safety_tests`'s
/// shape (stays Blocked throughout) rather than Barbarian's own
/// (which reaches Computed).
#[cfg(test)]
mod alchemist_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, ActiveState, CharacterClassLevel,
        CharacterInput, HeadlessReceiptStatus, ALCHEMIST_CLASS_ID, ALCHEMIST_MUTAGEN_ABILITY_ID,
        ALCHEMIST_MUTAGEN_STAT_CHOICE_ID, FIGHTER_CLASS_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_alchemist_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: ALCHEMIST_CLASS_ID.to_owned(), level }];
        input
    }

    /// A single-class Human Alchemist actively, validly mutated with a
    /// recognized Strength choice applies the real ability-modifier
    /// bonus/penalty and natural armor bonus to the integrated totals --
    /// but still stays `Blocked` (other_features_deferred), mirroring
    /// every other Rage-shaped closure that never reaches full Computed.
    ///
    /// Base fixture is Strength 16 (+4 with the fixture's chosen Human
    /// +2 floating Strength bonus applied) -> Mutagen's +4 Strength
    /// alchemical bonus (ability score) is +2 ability modifier -> +6.
    /// Intelligence 10 (+0) -> Mutagen's -2 penalty (ability score) is -1
    /// ability modifier -> -1.
    #[test]
    fn single_class_alchemist_actively_mutated_with_strength_choice_applies_real_bonuses() {
        let mut input = human_alchemist_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_MUTAGEN_STAT_CHOICE_ID.to_owned(),
            selection_id: "ability:strength".to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: ALCHEMIST_MUTAGEN_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Alchemist stays Blocked on other-features even while actively, validly mutated: \
             {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.alchemist.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the other_features_deferred diagnostic even while mutated: {:?}",
            receipt.computation.diagnostics
        );

        assert_eq!(receipt.computation.ability_modifiers.strength, 6);
        assert_eq!(receipt.computation.ability_modifiers.intelligence, -1);

        let armor_class = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .expect("baseline Armor Class must be grounded");
        // Base AC (10 + Chain Shirt 4 + DEX +2 + Dodge 1 = 17) + Mutagen
        // natural armor bonus (2) = 19.
        assert_eq!(
            armor_class.value, 19,
            "Mutagen's natural armor bonus must be applied: {:?}",
            armor_class
        );
    }

    /// A single-class Human Alchemist actively, validly mutated with a
    /// recognized Constitution choice applies the bonus/penalty to the
    /// correct, DIFFERENT pair of abilities (Con/Cha, not Str/Int) --
    /// proving the choice genuinely drives which fields are touched, not
    /// a hardcoded pair.
    ///
    /// Constitution 14 (+2) -> +2 ability modifier -> +4. Charisma 8
    /// (-1) -> -1 ability modifier -> -2.
    #[test]
    fn single_class_alchemist_actively_mutated_with_constitution_choice_applies_the_correct_pair() {
        let mut input = human_alchemist_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_MUTAGEN_STAT_CHOICE_ID.to_owned(),
            selection_id: "ability:constitution".to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: ALCHEMIST_MUTAGEN_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.computation.ability_modifiers.constitution, 4);
        assert_eq!(receipt.computation.ability_modifiers.charisma, -2);
        // The Strength/Intelligence pair from the other test must be
        // untouched here -- proves the choice is genuinely read, not a
        // hardcoded Str/Int pair applied regardless of selection.
        assert_eq!(receipt.computation.ability_modifiers.strength, 4);
        assert_eq!(receipt.computation.ability_modifiers.intelligence, 0);
    }

    /// An Alchemist claiming an active Mutagen but with no recognized
    /// `choice:alchemist_mutagen_stat` selection is a genuine posture
    /// violation and must claim-block -- never silently passed, mirroring
    /// Sorcerer's own "recognized bloodline but no bond choice given"
    /// shape.
    #[test]
    fn single_class_alchemist_active_mutagen_without_a_recognized_stat_choice_stays_blocked() {
        let mut input = human_alchemist_input(1);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: ALCHEMIST_MUTAGEN_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.apg.alchemist.mutagen_execution.stat_choice_missing"
                    && d.claim_blocking),
            "expected the missing-stat-choice claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert_eq!(
            receipt.computation.ability_modifiers.strength, 4,
            "no Mutagen bonus is applied for an active-but-unrecognized-choice posture: {:?}",
            receipt.computation.ability_modifiers
        );
    }

    /// A non-Alchemist character carrying a spoofed `"mutagen"`
    /// activation entry (plus a spoofed stat choice) must have it
    /// silently ignored, not applied -- the class-ownership gate is by
    /// construction (`active_alchemist_mutagen_bonus` only ever reads
    /// `class_ability_activations`/`selected_choices` after confirming
    /// `class_levels` contains Alchemist), not a bolt-on rejection. Also
    /// proves Fighter's own golden path is unaffected.
    #[test]
    fn non_alchemist_characters_spoofed_mutagen_activation_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_MUTAGEN_STAT_CHOICE_ID.to_owned(),
            selection_id: "ability:strength".to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: ALCHEMIST_MUTAGEN_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Alchemist mutagen entry: \
             {:?}",
            receipt.computation.diagnostics
        );
        assert_eq!(
            receipt.computation.ability_modifiers.strength, 4,
            "a non-Alchemist character's spoofed mutagen entry must never apply a bonus: {:?}",
            receipt.computation.ability_modifiers
        );
    }

    /// Bomb (damage dice/bonus, save DC, uses-per-day) and Poison
    /// Resistance are unconditional the moment Alchemist levels are
    /// present -- no choice, no activation gate, unlike Mutagen
    /// (deepening 2026-07-26, task #4). Fixture: Intelligence 10 -> +0
    /// modifier. Level 1: Bomb damage 1d6+0 (1+(1-1)/2=1), DC
    /// 10+0+0=10, uses/day 1+0=1. Poison Resistance honestly absent
    /// below level 2.
    #[test]
    fn single_class_alchemist_gets_the_unconditional_bomb_and_poison_resistance_facts() {
        let input = human_alchemist_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        let bomb_damage = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.alchemist.bomb_damage")
            .expect("Bomb damage must be grounded");
        assert_eq!(bomb_damage.value, 1, "level 1 Bomb damage dice: 1: {:?}", bomb_damage);

        let bomb_dc = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.alchemist.bomb_dc")
            .expect("Bomb DC must be grounded");
        assert_eq!(bomb_dc.value, 10, "level 1 Bomb DC: 10+0+0=10: {:?}", bomb_dc);

        let bomb_uses = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.alchemist.bomb_uses_per_day")
            .expect("Bomb uses per day must be grounded");
        assert_eq!(bomb_uses.value, 1, "level 1 Bomb uses/day: 1+0=1: {:?}", bomb_uses);

        let poison_resistance = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.alchemist.poison_resistance_bonus")
            .expect("Poison Resistance must ground unconditionally, even when absent");
        assert_eq!(
            poison_resistance.value, 0,
            "level 1 Poison Resistance is honestly absent below the real level-2 gate: {:?}",
            poison_resistance
        );
    }

    /// Bomb damage dice's real level-scaling progression, proven at every
    /// real level gate (1 + (level-1)/2), and Poison Resistance's own
    /// tier progression (+2/+4/+6, immune at 10), both re-derived
    /// independently rather than assumed from Investigator's own numbers.
    #[test]
    fn alchemist_bomb_damage_and_poison_resistance_match_the_real_corpus_progression() {
        for (level, expected_dice, expected_poison) in
            [(1, 1, 0), (3, 2, 2), (5, 3, 4), (8, 4, 6), (9, 5, 6), (10, 5, 0), (20, 10, 0)]
        {
            let input = human_alchemist_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let bomb_damage = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.apg.alchemist.bomb_damage")
                .unwrap_or_else(|| panic!("expected Bomb damage grounded at level {level}"));
            assert_eq!(bomb_damage.value, expected_dice, "level {level} Bomb damage dice: {:?}", bomb_damage);

            let poison_resistance = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.apg.alchemist.poison_resistance_bonus")
                .unwrap_or_else(|| panic!("expected Poison Resistance grounded at level {level}"));
            assert_eq!(
                poison_resistance.value, expected_poison,
                "level {level} Poison Resistance: {:?}",
                poison_resistance
            );
        }
    }

    /// A single-class Alchemist with a real, valid prepared-extract
    /// posture (a recorded and prepared extract from the shared
    /// Alchemist formula list) grounds the extract posture for real, but
    /// stays `Blocked` overall on `other_features_deferred` alone
    /// (deepening 2026-07-26, task #4) -- mirroring Investigator's own
    /// "posture grounds for real, stays Blocked on the rest" shape.
    ///
    /// Level 1 base 1st-level extracts: 1 (fixture Intelligence 10, +0
    /// modifier, so no Intelligence bonus extracts).
    #[test]
    fn single_class_alchemist_with_a_real_prepared_extract_grounds_the_posture_and_stays_blocked_on_other_features_only()
    {
        let mut input = human_alchemist_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: ALCHEMIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: ALCHEMIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Alchemist stays Blocked on other_features_deferred alone, even with a real, valid \
             prepared-extract posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.alchemist.prepared_extracts.unsupported"),
            "the prepared_extracts diagnostic must not fire once a real, valid posture is \
             recorded: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.alchemist.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected other_features_deferred even with a valid extract posture: {:?}",
            receipt.computation.diagnostics
        );

        let base_first_level = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.apg.alchemist.base_extracts_per_day.extract_level_1")
            .expect("base 1st-level extracts per day must be grounded");
        assert_eq!(base_first_level.value, 1, "level 1 base 1st-level extracts: 1: {:?}", base_first_level);

        let save_dc = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.apg.alchemist.extract_save_dc.extract_level_1")
            .expect("extract level 1 save DC must be grounded");
        assert_eq!(save_dc.value, 11, "10 + 1 + 0 = 11: {:?}", save_dc);
    }

    /// An Alchemist with no extracts recorded/prepared at all is a
    /// genuine, honest "hasn't started casting yet" posture -- claim-
    /// blocks on the dedicated prepared_extracts diagnostic, mirroring
    /// Investigator's/Arcanist's own "must have a real formula book" bar.
    #[test]
    fn single_class_alchemist_with_zero_extracts_stays_blocked_on_the_prepared_extracts_diagnostic()
    {
        let input = human_alchemist_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.alchemist.prepared_extracts.unsupported"
                    && d.claim_blocking),
            "expected the prepared_extracts diagnostic for a bare Alchemist with no extracts: \
             {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A prepared extract that was never recorded in the formula book is
    /// a genuine posture violation and must claim-block.
    #[test]
    fn single_class_alchemist_with_an_unrecorded_prepared_extract_stays_blocked() {
        let mut input = human_alchemist_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: ALCHEMIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.alchemist.prepared_extracts.unsupported"
                    && d.claim_blocking),
            "expected the prepared_extracts diagnostic for an unrecorded prepared extract: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Over-preparing a spell level beyond its real total slot budget is
    /// a genuine posture violation. Level 1 has exactly 1 first-level
    /// extract slot; preparing 2 different 1st-level extracts overflows it.
    #[test]
    fn single_class_alchemist_over_prepared_at_an_extract_level_stays_blocked() {
        let mut input = human_alchemist_input(1);
        for spell_id in ["Cure Light Wounds", "Shield"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: ALCHEMIST_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: ALCHEMIST_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Prepared,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.alchemist.prepared_extracts.unsupported"
                    && d.claim_blocking),
            "expected the prepared_extracts diagnostic for an over-prepared extract level: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A non-Alchemist character carrying spoofed Alchemist spell
    /// selections must have them silently ignored, not applied. Also
    /// proves Fighter's own golden path is unaffected.
    #[test]
    fn non_alchemist_characters_spoofed_extracts_are_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: ALCHEMIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: ALCHEMIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Alchemist extract entry: \
             {:?}",
            receipt.computation.diagnostics
        );
    }
}

/// SD28-C4.8: `archetype_resolver::archetype_claims_slot`'s first
/// real-compute-output consumer (`decisions.md §59`/`§60`). Reachability
/// proof per `§43`: a character who genuinely selects ARG's Plague
/// Bringer archetype must see the base Poison Resistance explanation
/// change in the actual compute output, not merely in a unit test over
/// the resolver alone.
#[cfg(test)]
mod alchemist_archetype_slot_reachability_tests {
    use super::{build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, ALCHEMIST_CLASS_ID};
    use crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID;
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_alchemist_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: ALCHEMIST_CLASS_ID.to_owned(), level }];
        input
    }

    fn poison_resistance_explanation(
        receipt: &crate::rules_core::pilot_compute::PilotHeadlessReceipt,
    ) -> &crate::rules_core::pilot_compute::ComputationExplanation {
        receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.alchemist.poison_resistance_bonus")
            .expect("Poison Resistance must always ground an explanation, archetype or not")
    }

    /// The base case, unaffected: a level-5 Alchemist with NO archetype
    /// selected grounds the real APG progression (+4 at level 5).
    #[test]
    fn a_bare_alchemist_grounds_the_real_poison_resistance_progression() {
        let input = human_alchemist_input(5);
        let receipt = build_pilot_headless_receipt(&input);
        let explanation = poison_resistance_explanation(&receipt);
        assert_eq!(explanation.value, 4, "level 5 Poison Resistance is +4: {explanation:?}");
        assert!(
            !explanation.detail.contains("superseded"),
            "a bare Alchemist's own explanation must not mention an archetype supersession: \
             {explanation:?}"
        );
    }

    /// The reachability proof: the SAME level-5 Alchemist, with Plague
    /// Bringer genuinely selected via the real `ARCHETYPE_CHOICE_ID`
    /// choice-set, must see the base progression superseded in the
    /// ACTUAL compute output -- not a resolver-only unit test, the real
    /// end-to-end path a player's own selection would take.
    #[test]
    fn a_plague_bringer_alchemist_supersedes_the_base_poison_resistance_in_real_compute_output() {
        let mut input = human_alchemist_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Alchemist Archetype ~ Plague Bringer".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        let explanation = poison_resistance_explanation(&receipt);
        assert_eq!(
            explanation.value, 0,
            "Plague Bringer supersedes the base progression -- the base +4 must not appear: \
             {explanation:?}"
        );
        assert!(
            explanation.detail.contains("Plague Bringer"),
            "the explanation must name the actual superseding archetype, not a generic message: \
             {explanation:?}"
        );
        assert!(
            explanation.detail.contains("superseded"),
            "the explanation must say plainly that the base progression does not apply: \
             {explanation:?}"
        );
    }

    /// The negative-adjacent proof: selecting a DIFFERENT Alchemist
    /// archetype that does NOT touch Poison Resistance (Bramble Brewer,
    /// which only replaces the Discovery slot) must leave the base
    /// progression grounding exactly as if no archetype were selected --
    /// proves the wiring is scoped to the real claimed slot, not "any
    /// archetype selected turns this off."
    #[test]
    fn an_unrelated_alchemist_archetype_leaves_poison_resistance_grounding_unchanged() {
        let mut input = human_alchemist_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Alchemist Archetype ~ Bramble Brewer".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        let explanation = poison_resistance_explanation(&receipt);
        assert_eq!(
            explanation.value, 4,
            "Bramble Brewer does not touch Poison Resistance -- the real +4 must still ground: \
             {explanation:?}"
        );
    }
}

/// v0.6 alpha swarm, risks item 8 (Investigator full-build closure, 10th
/// ACG/APG class-specific closure, no-spellcasting MVP): tests the three
/// flat class-feature formulas directly, mirroring Slayer's own
/// dispatch-widening test module shape (no choice/activation gates for
/// any of them).
#[cfg(test)]
mod investigator_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, CharacterClassLevel, CharacterInput,
        HeadlessReceiptStatus, FIGHTER_CLASS_ID, INVESTIGATOR_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, SpellSelection};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_investigator_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: INVESTIGATOR_CLASS_ID.to_owned(), level }];
        input
    }

    /// A single-class Human Investigator stays `Blocked` on the new,
    /// narrower `other_features_deferred` diagnostic alone (never the
    /// retired generic one), with all three flat sub-feature formulas
    /// grounded unconditionally -- Investigator has no choice or
    /// activation gate for any of them in this MVP.
    ///
    /// Level 1: Trapfinding max(1/2,1)=1, Trap Sense 1/3=0 (no floor),
    /// Inspiration pool size max(1, 1/2 + INT(0))=max(1,0)=1 (fixture
    /// Intelligence 10, +0 modifier -- the fixture's Human ability bonus
    /// choice applies to Strength, not Intelligence).
    #[test]
    fn single_class_investigator_stays_blocked_on_other_features_only_with_all_three_flat_formulas_grounded()
    {
        let input = human_investigator_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Investigator must stay Blocked on other-features-deferred alone: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.investigator.unsupported"),
            "the retired generic diagnostic must never appear for Investigator: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.acg.investigator.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );

        let trapfinding = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.investigator.trapfinding_bonus")
            .expect("Trapfinding must ground unconditionally");
        assert_eq!(trapfinding.value, 1, "Investigator level 1 Trapfinding: max(0,1)=1: {:?}", trapfinding);

        let trap_sense = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.investigator.trap_sense_bonus")
            .expect("Trap Sense must ground unconditionally");
        assert_eq!(
            trap_sense.value, 0,
            "Investigator level 1 Trap Sense: level/3=0, no floor (genuinely different from \
             Slayer's own floored formula): {:?}",
            trap_sense
        );

        let inspiration = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.investigator.inspiration_pool_size")
            .expect("Inspiration pool size must ground unconditionally");
        assert_eq!(
            inspiration.value, 1,
            "Investigator level 1 Inspiration pool size: max(1, 0 + 0)=1 (fixture Intelligence \
             10, +0 modifier): {:?}",
            inspiration
        );

        let poison_resistance = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.investigator.poison_resistance_bonus")
            .expect("Poison Resistance must ground unconditionally, even when absent");
        assert_eq!(
            poison_resistance.value, 0,
            "level 1 Poison Resistance is honestly absent below the real level-2 gate: {:?}",
            poison_resistance
        );

        let alchemy = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.investigator.alchemy_bonus")
            .expect("Alchemy must ground unconditionally");
        assert_eq!(alchemy.value, 1, "level 1 Alchemy: equal to Investigator level = 1: {:?}", alchemy);
    }

    /// Investigator's Trapfinding/Trap Sense/Inspiration progression at a
    /// higher level, verified against the PCGen corpus formulas directly
    /// (not merely trusting the level-1 zero/floor case above): level 6
    /// Trapfinding max(3,1)=3, Trap Sense 6/3=2 (still no floor to
    /// distinguish from a floored value at this level), Inspiration pool
    /// size max(1, 3 + 0)=3 (fixture Intelligence 10, +0 modifier).
    #[test]
    fn investigator_flat_formulas_match_the_corpus_at_a_higher_level() {
        let input = human_investigator_input(6);
        let receipt = build_pilot_headless_receipt(&input);

        let trapfinding = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.investigator.trapfinding_bonus")
            .expect("Trapfinding must ground unconditionally");
        assert_eq!(trapfinding.value, 3, "level 6 Trapfinding: max(3,1)=3: {:?}", trapfinding);

        let trap_sense = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.investigator.trap_sense_bonus")
            .expect("Trap Sense must ground unconditionally");
        assert_eq!(trap_sense.value, 2, "level 6 Trap Sense: 6/3=2: {:?}", trap_sense);

        let inspiration = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.investigator.inspiration_pool_size")
            .expect("Inspiration pool size must ground unconditionally");
        assert_eq!(
            inspiration.value, 3,
            "level 6 Inspiration pool size: max(1, 3 + 0)=3 (fixture Intelligence 10, +0 \
             modifier): {:?}",
            inspiration
        );

        let alchemy = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.investigator.alchemy_bonus")
            .expect("Alchemy must ground unconditionally");
        assert_eq!(alchemy.value, 6, "level 6 Alchemy: equal to Investigator level = 6: {:?}", alchemy);
    }

    /// Poison Resistance's real tier progression (deepening 2026-07-26,
    /// task #8): honestly absent below level 2, +2/+4/+6 at levels 2/5/8,
    /// then converts to full immunity (value 0, a qualitatively different
    /// fact, not a fourth numeric tier) at level 10 -- a genuine
    /// correction to the scoping doc's own "+2/+4/+6/+8, immune at 20th"
    /// claim, verified directly against the raw corpus tier-gating tokens.
    #[test]
    fn investigator_poison_resistance_matches_the_real_corpus_tiers_not_the_scoping_docs_claim() {
        for (level, expected_value, expected_substring) in [
            (1, 0, "correctly absent"),
            (2, 2, "a +2 bonus"),
            (4, 2, "a +2 bonus"),
            (5, 4, "a +4 bonus"),
            (7, 4, "a +4 bonus"),
            (8, 6, "a +6 bonus"),
            (9, 6, "a +6 bonus"),
            (10, 0, "fully immune"),
            (15, 0, "fully immune"),
        ] {
            let input = human_investigator_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let poison_resistance = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.investigator.poison_resistance_bonus")
                .unwrap_or_else(|| panic!("expected Poison Resistance grounded at level {level}"));
            assert_eq!(
                poison_resistance.value, expected_value,
                "level {level} Poison Resistance: {:?}",
                poison_resistance
            );
            assert!(
                poison_resistance.detail.contains(expected_substring),
                "level {level} detail should mention '{expected_substring}': {}",
                poison_resistance.detail
            );
        }
    }

    /// A non-Investigator character carrying no Investigator entries at
    /// all must never ground any Investigator explanation. Also proves
    /// Fighter's own golden path is unaffected.
    #[test]
    fn non_investigator_characters_never_ground_investigator_explanations() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.acg.investigator.")),
            "a non-Investigator character must never ground any Investigator explanation: {:?}",
            receipt.computation.explanations
        );
    }

    /// A single-class Investigator with a real, valid prepared-extract
    /// posture (a recorded and prepared extract from the real, shared
    /// Alchemist formula list) grounds the extract posture for real, but
    /// stays `Blocked` overall on `other_features_deferred` alone
    /// (deepening 2026-07-26, task #8) -- mirroring Arcanist's own
    /// "spellbook grounds for real, stays Blocked on the rest" shape.
    ///
    /// Level 1 base 1st-level extracts: 1 (fixture Intelligence 10, +0
    /// modifier, so no Intelligence bonus extracts).
    #[test]
    fn single_class_investigator_with_a_real_prepared_extract_grounds_the_posture_and_stays_blocked_on_other_features_only()
    {
        let mut input = human_investigator_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Investigator stays Blocked on other_features_deferred alone, even with a real, \
             valid prepared-extract posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.investigator.prepared_extracts.unsupported"),
            "the prepared_extracts diagnostic must not fire once a real, valid posture is \
             recorded: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.investigator.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected other_features_deferred even with a valid extract posture: {:?}",
            receipt.computation.diagnostics
        );

        let base_first_level = receipt
            .computation
            .explanations
            .iter()
            .find(|e| {
                e.id == "class_spell.acg.investigator.base_extracts_per_day.extract_level_1"
            })
            .expect("base 1st-level extracts per day must be grounded");
        assert_eq!(base_first_level.value, 1, "level 1 base 1st-level extracts: 1: {:?}", base_first_level);

        let save_dc = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.investigator.extract_save_dc.extract_level_1")
            .expect("extract level 1 save DC must be grounded");
        assert_eq!(save_dc.value, 11, "10 + 1 + 0 = 11: {:?}", save_dc);
    }

    /// An Investigator with no extracts recorded/prepared at all is a
    /// genuine, honest "hasn't started casting yet" posture -- claim-
    /// blocks on the dedicated prepared_extracts diagnostic, mirroring
    /// Arcanist's own "must have a real spellbook" bar.
    #[test]
    fn single_class_investigator_with_zero_extracts_stays_blocked_on_the_prepared_extracts_diagnostic()
    {
        let input = human_investigator_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.investigator.prepared_extracts.unsupported"
                    && d.claim_blocking),
            "expected the prepared_extracts diagnostic for a bare Investigator with no extracts: \
             {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A prepared extract that was never recorded in the formula book is
    /// a genuine posture violation and must claim-block.
    #[test]
    fn single_class_investigator_with_an_unrecorded_prepared_extract_stays_blocked() {
        let mut input = human_investigator_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.investigator.prepared_extracts.unsupported"
                    && d.claim_blocking),
            "expected the prepared_extracts diagnostic for an unrecorded prepared extract: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// An extract prepared at a spell level not yet accessible at the
    /// character's own Investigator level is a genuine posture violation.
    /// Level 1 Investigator has no 2nd-level extract access at all
    /// (base_extracts_per_day returns None for extract level 2).
    #[test]
    fn single_class_investigator_with_an_inaccessible_extract_level_stays_blocked() {
        let mut input = human_investigator_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Aid".to_owned(),
            source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Aid".to_owned(),
            source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.investigator.prepared_extracts.unsupported"
                    && d.claim_blocking),
            "expected the prepared_extracts diagnostic for an inaccessible extract level: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Over-preparing a spell level beyond its real total slot budget is
    /// a genuine posture violation. Level 1 has exactly 1 first-level
    /// extract slot; preparing 2 different 1st-level extracts overflows it.
    #[test]
    fn single_class_investigator_over_prepared_at_an_extract_level_stays_blocked() {
        let mut input = human_investigator_input(1);
        for spell_id in ["Cure Light Wounds", "Shield"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Prepared,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.investigator.prepared_extracts.unsupported"
                    && d.claim_blocking),
            "expected the prepared_extracts diagnostic for an over-prepared extract level: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A non-Investigator character carrying spoofed Investigator spell
    /// selections must have them silently ignored, not applied -- the
    /// class-ownership gate is by construction. Also proves Fighter's own
    /// golden path is unaffected.
    #[test]
    fn non_investigator_characters_spoofed_extracts_are_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Investigator extract \
             entry: {:?}",
            receipt.computation.diagnostics
        );
    }
}

