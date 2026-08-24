//! Slayer (Advanced Class Guide) split out of `pilot_compute.rs`
//! (SD31-E4-F1-005), a pure code-move with unchanged behaviour. `use super::*;`
//! gives this submodule the same visibility into `pilot_compute`'s private items
//! its functions relied on before the move. `ground_or_block_slayer_class_features`,
//! `ground_slayer_weapon_and_armor_proficiency` and
//! `ground_slayer_remaining_named_features` are called from `mod.rs`'s class
//! dispatcher; `slayer_track_bonus` and `slayer_trap_sense_bonus` are also read from
//! `mod.rs` (a shared cross-class trapfinding/tracking summary); the rest are
//! Slayer-internal helpers, read by this file's own siblings and by `mod.rs`'s own
//! inline `#[cfg(test)]` module that exercises them through `super::<name>`.

use super::*;

/// PF1 Advanced Class Guide Slayer Sneak Attack: dice count
/// `SlayerLVL/3`, verified directly against `acg_abilities_class.lst`'s
/// own `BONUS:VAR|SneakAttackDice|SlayerSneakAttackLVL/3`.
pub(super) fn slayer_sneak_attack_dice(level: u8) -> i16 {
    i16::from(level) / 3
}

/// PF1 Advanced Class Guide Slayer Trap Sense: `max(1, SlayerLVL/3)`,
/// verified directly against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|TrapSenseBonus|max(1,SlayerTrapSenseLVL/3)`. Grounded as a
/// standalone flat record with no further total-integration, mirroring
/// Barbarian's own `class_feature.barbarian.trap_sense` and Rogue's own
/// `class_feature.rogue.trap_sense` exactly -- this codebase has no
/// "trap AC/save" pillar for either of those closures to integrate into
/// either, an already-established idiom.
pub(super) fn slayer_trap_sense_bonus(level: u8) -> i16 {
    (i16::from(level) / 3).max(1)
}

/// PF1 Advanced Class Guide Slayer Trapfinding: `SlayerLVL/2`, verified
/// directly against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|SlayerTrapfindingBonus|SlayerTrapfindingLVL/2`. A bonus on
/// Perception (to locate traps) and Disable Device -- neither tracked by
/// `compute_selected_skill_modifiers` (which only tracks Climb/
/// Intimidate/Swim), so this grounds as a standalone flat record.
pub(super) fn slayer_trapfinding_bonus(level: u8) -> i16 {
    i16::from(level) / 2
}

/// PF1 Advanced Class Guide Slayer Track: `max(SlayerLVL/2, 1)`,
/// verified directly against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|SlayerTrackBonus|max(SlayerTrackLVL/2,1)`. A bonus on
/// Survival (to follow tracks) -- also not among the three tracked
/// skills, so this grounds as a standalone flat record too.
pub(super) fn slayer_track_bonus(level: u8) -> i16 {
    (i16::from(level) / 2).max(1)
}

/// Grounds Slayer's class features for `level` (v0.6 alpha swarm, risks
/// item 8, Slayer full-build closure). Called from
/// `compute_acg_class_chassis`'s Slayer branch, gated only on Slayer
/// class-ownership. All four sub-features are flat, always-on class
/// features (not activation-gated, not choice-gated) -- grounds each as
/// its own standalone explanation record, then pushes the narrowed
/// `other_features_deferred` diagnostic naming Studied Target
/// (opponent-dependent) and Slayer Talents (a chooser-list) as the
/// genuinely still-missing pieces.
/// Slayer's talent count: `SlayerTalentLVL/2`, where
/// `SlayerTalentLVL = SlayerLVL` -- one talent at 2nd level, ten by
/// 20th.
///
/// Ten `-1` deductions against this pool exist in the corpus, each gated
/// on a `Slayer_CF_TalentN` flag. Every setter is a Slayer ARCHETYPE
/// `.MOD` record (Bounty Hunter, Cleaner, Cutthroat, and others), and
/// this repo ingests only the base `slayer.json` -- provably vacuous,
/// the same check that cleared Brawler's seven and Cavalier's three.
pub(super) fn slayer_talent_count(level: u8) -> i16 {
    i16::from(level) / 2
}

/// Slayer's Studied Target insight bonus: `SlayerLVL/5 + 1` on attack
/// and damage rolls (and several skills) against the studied target.
///
/// Grounds standalone despite being target-conditioned. The formula
/// reads only the slayer's own level -- nothing about the opponent --
/// and the decisive precedent is this class's OWN Sneak Attack dice,
/// already grounded standalone though it is flanking-conditional.
/// (Lead ruling, risks item 52, reversing the earlier
/// opponent-dependent-defers line.)
pub(super) fn slayer_studied_target_bonus(level: u8) -> i16 {
    i16::from(level) / 5 + 1
}

/// How many targets a Slayer may have studied at once:
/// `(SlayerLVL>0)+(SlayerLVL>6)` -- one from 1st level, two from 7th.
pub(super) fn slayer_studied_target_count(level: u8) -> i16 {
    let level = i16::from(level);
    [0, 6].iter().map(|gate| i16::from(level > *gate)).sum()
}

/// Stalker's grant level, per `acg_classes.lst:337`.
const SLAYER_STALKER_LEVEL: u8 = 7;

/// Swift Tracker's grant level, per `acg_classes.lst:338`.
const SLAYER_SWIFT_TRACKER_LEVEL: u8 = 11;

/// Slayer's Advance's grant level, per `acg_classes.lst:339`.
const SLAYER_ADVANCE_LEVEL: u8 = 13;

/// Quarry's grant level, per `acg_classes.lst:340`.
///
/// Note this is **14**, not the 11th level a reader who assumed parity
/// with Ranger's Quarry might expect -- Slayer has its own table.
const SLAYER_QUARRY_LEVEL: u8 = 14;

/// Improved Quarry's grant level, per `acg_classes.lst:341`.
const SLAYER_IMPROVED_QUARRY_LEVEL: u8 = 19;

/// Master Slayer's grant level, per `acg_classes.lst:342`.
const SLAYER_MASTER_SLAYER_LEVEL: u8 = 20;

/// Quarry's insight bonus on attack rolls against the quarry: +2.
///
/// **Sourced from the record's `DESC:` prose, not a `BONUS:` token.**
/// `KEY:Slayer ~ Quarry Output` (`acg_abilities_class.lst:1800`) carries
/// no BONUS or DEFINE at all; its whole rule is text. The value is
/// transcribed verbatim from "you receive a +2 insight bonus on attack
/// rolls made against your quarry".
const SLAYER_QUARRY_ATTACK_BONUS: i16 = 2;

/// Improved Quarry's insight bonus on attack rolls: +4, superseding
/// Quarry's +2. Same evidentiary path -- DESC prose on
/// `KEY:Slayer ~ Improved Quarry` (`acg_abilities_class.lst:1789`),
/// which likewise carries no BONUS token.
const SLAYER_IMPROVED_QUARRY_ATTACK_BONUS: i16 = 4;

/// Stalker's bonus on Disguise, Intimidate and Stealth checks against a
/// studied opponent: `SlayerLVL/5 + 1`.
///
/// Verified against `acg_abilities_class.lst:1793`'s own
/// `BONUS:VAR|SlayerStalkerBonus|SlayerStalkerLVL/5+1`, chained through
/// `BONUS:VAR|SlayerStalkerLVL|SlayerStudiedTargetLVL` and Studied
/// Target's `BONUS:VAR|SlayerStudiedTargetLVL|SlayerLVL`. Following that
/// two-hop chain matters: reading `SlayerStalkerLVL` as an independent
/// variable would leave it at its `DEFINE:...|0` default and yield a
/// flat +1 at every level.
///
/// Deliberately shares no code with `slayer_studied_target_bonus`
/// despite computing the same expression today. They are two separate
/// corpus records whose agreement is incidental, and Stalker applies to
/// a different skill set (Disguise/Intimidate/Stealth vs attack, damage
/// and a different five skills).
pub(super) fn slayer_stalker_bonus(level: u8) -> i16 {
    i16::from(level) / 5 + 1
}

/// Slayer's Advance uses per day: `1 + (SlayerLVL > 16)`.
///
/// Verified against `acg_abilities_class.lst:1791`'s own
/// `BONUS:VAR|SlayersAdvanceTimes|1+(SlayerLVL>16)`. PCGen evaluates the
/// comparison to 1/0, so this is 1/day from its 13th-level grant and
/// 2/day from 17th -- **not** a level/N progression.
pub(super) fn slayer_advance_uses_per_day(level: u8) -> i16 {
    1 + i16::from(i16::from(level) > 16)
}

/// Master Slayer's save DC: `10 + SlayerLVL/2 + INT modifier`.
///
/// Verified against `acg_abilities_class.lst:1794`'s own
/// `BONUS:VAR|MasterSlayerDC|10+(MasterSlayerLVL/2)+INT`, with
/// `BONUS:VAR|MasterSlayerLVL|SlayerLVL`.
///
/// **The stat is INT**, which is worth stating out loud: most save DCs
/// in this file key off the class's casting stat, and Slayer has none.
/// The bare `INT` token is a modifier, not a score -- the same
/// bare-token/`SCORE`-token distinction that governs Brawler's Knockout
/// DC versus Brawler's Cunning.
pub(super) fn slayer_master_slayer_dc(level: u8, intelligence_modifier: i16) -> i16 {
    10 + i16::from(level) / 2 + intelligence_modifier
}

pub(super) fn ground_or_block_slayer_class_features(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let talent_count = slayer_talent_count(level);
    if talent_count > 0 {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.slayer.talent_count".to_owned(),
            value: talent_count,
            detail: format!(
                "Slayer level {level} has {talent_count} Slayer Talent(s) (level/2, first at 2nd \
                 level). Grounds the COUNT only -- which talents were taken is a chooser over 41 \
                 records, and a talent chooser's entire value is which talent was picked, so \
                 none is seeded. Ten archetype-gated deductions against this pool are provably \
                 vacuous here: every setter is a Slayer archetype record and this repo ingests \
                 only the base class"
            ),
        });

        if input.chosen.selected_choices.iter().any(|c| {
            c.choice_set_id == SLAYER_TALENT_CHOICE_ID
                && c.selection_id == SLAYER_TALENT_FOIL_SCRUTINY_SELECTION
        }) {
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.slayer.talent.foil_scrutiny_bonus".to_owned(),
                value: SLAYER_FOIL_SCRUTINY_BONUS,
                detail: format!(
                    "Slayer level {level} took the Foil Scrutiny talent: a \
                     +{SLAYER_FOIL_SCRUTINY_BONUS} bonus on Bluff and Disguise checks made to \
                     avoid notice. The one canonical talent grounded here, narrowed the same way \
                     Order of the Sword was; it requires an explicit recorded pick and is never \
                     seeded. Neither Bluff nor Disguise is among the three skills this engine \
                     computes, so this grounds standalone"
                ),
            });
        }

        // SD-32 T12 Epic 8 cycle 3: generic pass over the OTHER 45 real
        // corpus `Slayer Talent ~ *` records (46 total, real group name
        // confirmed by direct corpus grep -- unlike several other
        // `CLASS_FEATURE_POOLS`-registered pools, "Slayer Talent" is the
        // real group prefix) this file has never hand-modelled by name.
        // Purely additive alongside Foil Scrutiny above -- a talent this
        // resolver cannot ground (a dice-notation magnitude, a
        // multi-terminal record, or one carrying no BONUS/DEFINE token at
        // all) contributes nothing here, exactly as it contributed
        // nothing before.
        push_generic_pool_choice_magnitude(
            input,
            level,
            &ability_modifiers_from_scores(&input.chosen.ability_scores),
            SLAYER_TALENT_CHOICE_ID,
            "Slayer Talent",
            "talent:",
            "class_feature.acg.slayer.talent.generic",
            2,
            explanations,
        );
    }

    let studied_bonus = slayer_studied_target_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.slayer.studied_target_bonus".to_owned(),
        value: studied_bonus,
        detail: format!(
            "Slayer level {level} Studied Target: a +{studied_bonus} insight bonus (level/5 + 1) \
             on attack and damage rolls against a target he has studied, and on Bluff, \
             Knowledge, Perception, Sense Motive, and Survival checks about it. Grounds \
             standalone: the formula reads only the slayer's own level, nothing about the \
             opponent, and this class's own Sneak Attack dice is already grounded the same way \
             despite being flanking-conditional. Which creature is studied is not modelled"
        ),
    });
    let studied_count = slayer_studied_target_count(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.slayer.studied_target_count".to_owned(),
        value: studied_count,
        detail: format!(
            "Slayer level {level} may have {studied_count} target(s) studied at once \
             ((level>0)+(level>6) -- one from 1st level, a second from 7th). A flat capacity \
             fact; no target identity or per-target state is tracked"
        ),
    });

    let sneak_attack_dice = slayer_sneak_attack_dice(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.slayer.sneak_attack_dice".to_owned(),
        value: sneak_attack_dice,
        detail: format!(
            "Slayer level {level} Sneak Attack dice: level/3 = {sneak_attack_dice}d6. This \
             codebase computes no sneak-attack-damage total to layer this onto; the flat dice \
             count is grounded as a standalone record only"
        ),
    });

    let trap_sense_bonus = slayer_trap_sense_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.slayer.trap_sense_bonus".to_owned(),
        value: trap_sense_bonus,
        detail: format!(
            "Slayer level {level} Trap Sense: a +{trap_sense_bonus} bonus on Reflex saves made \
             to avoid traps and a +{trap_sense_bonus} dodge bonus to AC against attacks made by \
             traps (max(1, level/3) = {trap_sense_bonus}). This codebase has no trap-specific \
             AC/save pillar; grounded as a standalone flat record, mirroring Barbarian's/Rogue's \
             own Trap Sense precedent exactly"
        ),
    });

    let trapfinding_bonus = slayer_trapfinding_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.slayer.trapfinding_bonus".to_owned(),
        value: trapfinding_bonus,
        detail: format!(
            "Slayer level {level} Trapfinding: a +{trapfinding_bonus} bonus on Perception \
             checks made to locate traps and Disable Device checks (level/2 = \
             {trapfinding_bonus}). Neither Perception nor Disable Device is among the three \
             skills compute_selected_skill_modifiers tracks (Climb/Intimidate/Swim), so this \
             grounds as a standalone flat record"
        ),
    });

    let track_bonus = slayer_track_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.slayer.track_bonus".to_owned(),
        value: track_bonus,
        detail: format!(
            "Slayer level {level} Track: a +{track_bonus} bonus on Survival checks made to \
             follow tracks (max(level/2, 1) = {track_bonus}). Survival is not among the three \
             tracked skills either, so this grounds as a standalone flat record"
        ),
    });

    ground_slayer_weapon_and_armor_proficiency(input, explanations);

    ground_slayer_remaining_named_features(input, level, explanations);

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.slayer.other_features_deferred.unsupported".to_owned(),
        message: format!(
            "{SLAYER_CLASS_ID} now grounds every named feature on its corpus class table: the \
             base-attack-bonus/base-save chassis pillar, its class-skill list, Sneak Attack dice, \
             Trap Sense, Trapfinding, Track, Studied Target's bonus and count, the talent count \
             with its canonical Foil Scrutiny pick, -- task #91 -- Stalker, Swift Tracker, \
             Slayer's Advance, Quarry, Improved Quarry and Master Slayer, and -- SD31-E4-F1-001, \
             the class's last previously-unwired slot -- Weapon and Armor Proficiency, now \
             wired through the real `archetype_claiming_slot_entry` supersession primitive \
             against the 3 Slayer archetypes (Bounty Hunter, Deliverer, Stygian Slayer) this \
             same cycle added to the ACG archetype-swap catalog. This \
             diagnostic is therefore no longer claim-blocking; it is retained to carry the \
             honest remainder. What stays deferred: (1) APPLICATION, not magnitude -- Studied \
             Target's, Stalker's and Quarry's bonuses all only matter against a studied or \
             quarried opponent, and no target-creature representation exists here, so the \
             numbers are derived correctly but nothing consumes them; likewise Master Slayer's \
             DC has no saving-throw resolution to be rolled against, and Sneak Attack's dice no \
             damage total. Under this repo's standalone-fact grounding bar a missing consumer \
             does not block a correctly-derived number. (2) The Slayer Talent family is covered \
             at 1 of its 41 real corpus records (Foil Scrutiny), narrowed the same way Oracle's \
             Mystery and Cavalier's Order of the Sword are: the canonical pick must be recorded \
             explicitly and is never seeded, and the 40 unmodelled talents are a catalog gap, \
             not a defect in the talent count. A Slayer who records some OTHER talent still \
             computes -- that talent simply contributes no magnitude. (3) Swift Tracker, Quarry \
             and Improved Quarry carry no BONUS or DEFINE token anywhere in the corpus; their \
             numbers, where they have any, live only in DESC prose and are transcribed as such \
             above. This message previously named Studied Target and Slayer Talents as the two \
             things remaining, and named nothing else -- both were already grounded (tasks \
             #13/#58) -- and a later revision correctly listed the seven features above, all of \
             which are now grounded"
        ),
        claim_blocking: false,
    });
}

/// Grounds Slayer's Weapon and Armor Proficiency (corpus `KEY:Slayer ~
/// Weapon and Armor Proficiency`) with the real archetype-supersession
/// `if let`/`else` shape SD31-E4-F1's acceptance names, using
/// `archetype_resolver::archetype_claiming_slot_entry` -- the first ACG
/// consumer of that primitive (Alchemist/Fighter, APG/CRB, were the
/// first two consumers overall).
///
/// **Zero-magnitude, grant-only record, by design.** The corpus row
/// carries no `BONUS:` token, only `ABILITY:...AUTOMATIC` proficiency
/// grants -- confirmed against the ingested corpus JSON
/// (`data/corpus/advanced_class_guide/class_feature/slayer/
/// weapon_and_armor_proficiency.json`, `wiring_class: "display"`,
/// `wiring_class_signals: ["display:no_magnitude_token"]`). Matches
/// Decision 7's prose done-bar (`decisions.md §7`) exactly: prose only,
/// nothing to compute, and the description is populated here and
/// rendered on the character sheet's Class Features section
/// (`classFeaturesModel.ts`'s generic `class_feature.` prefix pickup) --
/// the same "grant-only identity record" idiom this file already uses
/// for Sorcerer's Arcane Apotheosis and Rogue's Master Strike.
///
/// **The weapon half's real mechanical consequence is grounded
/// elsewhere, not duplicated here.** `weapon_tables::
/// class_weapon_proficiency("class:slayer")` already carries Slayer's
/// Simple+Martial weapon tiers and is read by
/// `character_is_proficient_with` to decide the real -4
/// nonproficiency-attack-penalty on every equipped weapon -- this
/// record is the class-features-tab DISPLAY grounding, a different
/// concern from that combat-math grounding.
///
/// **No armor-nonproficiency-penalty mechanic exists anywhere in this
/// engine, verified rather than assumed.** The PF1 game system's own
/// machine-readable `system/gameModes/Pathfinder/miscinfo.lst` carries
/// exactly one `NONPROF` token, `WEAPONNONPROFPENALTY:-4` -- no armor
/// equivalent. Building one (an armor-proficiency table plus an
/// armor-check-penalty consumer, mirroring the weapon lane) is real,
/// bounded, out-of-territory-this-cycle follow-on work (this file's own
/// `pilot_compute.rs`/`archetype_resolver.rs` territory does not
/// include a new armor table module), named rather than silently
/// skipped -- see `OPEN-ISSUES.md`.
pub(super) fn ground_slayer_weapon_and_armor_proficiency(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    const SLAYER_PROFICIENCY_SLOT_IDS: [&str; 3] =
        ["WeaponProficiencies", "ArmorProficiencies", "Proficiencies"];

    let claimed = SLAYER_PROFICIENCY_SLOT_IDS
        .iter()
        .find_map(|slot| archetype_resolver::archetype_claiming_slot_entry(input, "Slayer", slot));

    if let Some(entry) = claimed {
        // Supersession branch: a real, selected Slayer archetype claims one
        // of the three proficiency-shaped slot ids this book's Slayer rows
        // declare (Bounty Hunter's own FACT-set names the split
        // WeaponProficiencies+ArmorProficiencies pair; Deliverer/Stygian
        // Slayer's own PREMULT clause names the generic Proficiencies
        // fact instead -- a real, verified corpus inconsistency between
        // the two shapes, not smoothed over). The base ACG progression
        // does not apply; the archetype's OWN "~ Weapon and Armor
        // Proficiency" sub-feature text is read directly off its real
        // catalog `grants` entry, never re-typed by hand a second time.
        let own_grant = entry
            .grants
            .iter()
            .find(|g| g.grants_feature_key.ends_with("~ Weapon and Armor Proficiency"));
        let detail = match own_grant.and_then(|g| g.description) {
            Some(text) => format!(
                "Slayer Weapon and Armor Proficiency: superseded by the selected {} archetype \
                 (corpus KEY:{}), which replaces this base-class slot. {}'s own text: \"{text}\"",
                entry.archetype_name, entry.key, entry.archetype_name
            ),
            None => format!(
                "Slayer Weapon and Armor Proficiency: superseded by the selected {} archetype \
                 (corpus KEY:{}), which replaces this base-class slot. The base ACG progression \
                 does not apply; {}'s own replacement proficiency text is not resolved in this \
                 catalog entry",
                entry.archetype_name, entry.key, entry.archetype_name
            ),
        };
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.slayer.weapon_and_armor_proficiency".to_owned(),
            value: 0,
            detail,
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.slayer.weapon_and_armor_proficiency".to_owned(),
            value: 0,
            detail: "Slayer Weapon and Armor Proficiency (corpus KEY:Slayer ~ Weapon and Armor \
                 Proficiency): \"A slayer is proficient with all simple and martial weapons, as \
                 well as with light armor, medium armor, and shields (except tower shields).\" \
                 This is a bounded grant-only identity record (value 0, non-fabricated): the \
                 record's only tokens are ABILITY:...AUTOMATIC proficiency grants, no BONUS: \
                 magnitude anywhere. The weapon half's real mechanical consequence -- avoiding \
                 the -4 nonproficiency attack penalty -- is already grounded separately by \
                 `weapon_tables::class_weapon_proficiency(\"class:slayer\")`, which this record \
                 does not duplicate. No armor-nonproficiency-penalty mechanic exists anywhere in \
                 this engine (the game system's own miscinfo.lst carries only \
                 WEAPONNONPROFPENALTY:-4, no armor equivalent), so the armor half has nothing \
                 further to compute here"
                .to_owned(),
        });
    }
}

/// Grounds Slayer's last seven named class features (task #91): Stalker,
/// Swift Tracker, Slayer's Advance, Quarry, Quarry Output, Improved
/// Quarry and Master Slayer.
///
/// These were the entire content of Slayer's claim-blocking
/// `other_features_deferred` diagnostic. Three shapes appear:
///
/// * **Real `BONUS:VAR` magnitudes** -- Stalker, Slayer's Advance and
///   Master Slayer each carry a live corpus formula, transcribed in
///   `slayer_stalker_bonus`, `slayer_advance_uses_per_day` and
///   `slayer_master_slayer_dc`.
///
/// * **DESC-prose magnitudes** -- Quarry's +2 and Improved Quarry's +4
///   insight bonus on attack rolls exist only as rulebook text; the
///   records carry no BONUS token. Transcribed verbatim, and labelled in
///   the explanation as prose-sourced so a reader is never misled about
///   the evidentiary path.
///
/// * **Zero magnitude** -- Swift Tracker reduces the Survival penalty
///   for tracking while moving. Its record is DESC-only and the numbers
///   inside it (-5, -10, -20) are the *normal* penalties it waives, not
///   quantities derived from this character. It grounds as a bounded
///   grant-only identity record, the Arcane Apotheosis idiom.
///
/// **The supersession is real and is honoured.** The corpus models
/// Quarry as a hidden dispatcher (`KEY:Slayer ~ Quarry`, `VISIBLE:NO`,
/// no DESC, no numerics) that grants the visible `Slayer ~ Quarry
/// Output` record *only* when the character has nothing of TYPE
/// `SlayerImprovedQuarry` -- and `KEY:Slayer ~ Improved Quarry` carries
/// exactly that TYPE tag. So from 19th level Improved Quarry replaces
/// Quarry Output rather than stacking with it. Emitting both would
/// report +6 worth of insight bonus to a reader who summed them.
pub(super) fn ground_slayer_remaining_named_features(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if level >= SLAYER_STALKER_LEVEL {
        let stalker = slayer_stalker_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.slayer.stalker_bonus".to_owned(),
            value: stalker,
            detail: format!(
                "Slayer level {level} Stalker (granted at level {SLAYER_STALKER_LEVEL}): a \
                 +{stalker} bonus on Disguise, Intimidate and Stealth checks against his studied \
                 opponent (level/5 + 1). The corpus reaches this through a two-hop variable \
                 chain -- SlayerStalkerBonus = SlayerStalkerLVL/5+1, SlayerStalkerLVL = \
                 SlayerStudiedTargetLVL, SlayerStudiedTargetLVL = SlayerLVL -- and reading \
                 SlayerStalkerLVL as an independent variable would leave it at its DEFINE \
                 default of 0 and yield a flat +1 at every level. Grounds standalone: the \
                 formula reads only the slayer's own level. Intimidate IS one of the three \
                 skills this engine computes, but this bonus is scoped to a studied opponent and \
                 no target-creature representation exists, so it is deliberately NOT added to \
                 the Intimidate total -- adding it would overstate the general-case skill"
            ),
        });
    }

    if level >= SLAYER_SWIFT_TRACKER_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.slayer.swift_tracker_grant".to_owned(),
            value: 0,
            detail: format!(
                "Slayer level {level} Swift Tracker, granted at level \
                 {SLAYER_SWIFT_TRACKER_LEVEL} (corpus KEY:Slayer ~ Swift Tracker): \"You can \
                 move at your normal speed while using Survival to follow tracks without taking \
                 the normal -5 penalty. You take only a -10 penalty (instead of the normal -20) \
                 when moving at up to twice normal speed while tracking.\" This is a bounded \
                 grant-only identity record (value 0, non-fabricated): the record's complete \
                 token list is KEY, CATEGORY, TYPE, DESC and SOURCEPAGE, with no BONUS or DEFINE \
                 anywhere. The -5/-10/-20 inside the text are the NORMAL tracking penalties this \
                 feature waives or reduces, not quantities derived from this character, and this \
                 codebase models no movement-rate-versus-tracking penalty for them to modify. \
                 Distinct from KEY:Hunter ~ Swift Tracker, a separate record in a separate \
                 namespace"
            ),
        });
    }

    if level >= SLAYER_ADVANCE_LEVEL {
        let advance_uses = slayer_advance_uses_per_day(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.slayer.advance_uses_per_day".to_owned(),
            value: advance_uses,
            detail: format!(
                "Slayer level {level} Slayer's Advance (granted at level {SLAYER_ADVANCE_LEVEL}): \
                 usable {advance_uses} time(s) per day, moving up to twice his base speed as a \
                 move action. Corpus formula 1+(SlayerLVL>16) -- the comparison evaluates to \
                 1/0, so this is 1/day from the grant and 2/day from 17th, NOT a level/N \
                 progression. The -10 Stealth penalty for using Stealth as part of the move is a \
                 fixed rules constant inside the resolution, and this codebase computes no \
                 movement action to apply the doubled speed to"
            ),
        });
    }

    // Quarry and Improved Quarry are mutually exclusive in the corpus,
    // not cumulative -- see this function's doc comment.
    if level >= SLAYER_IMPROVED_QUARRY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.slayer.improved_quarry_attack_bonus".to_owned(),
            value: SLAYER_IMPROVED_QUARRY_ATTACK_BONUS,
            detail: format!(
                "Slayer level {level} Improved Quarry (granted at level \
                 {SLAYER_IMPROVED_QUARRY_LEVEL}): a +{SLAYER_IMPROVED_QUARRY_ATTACK_BONUS} \
                 insight bonus on attack rolls against his quarry, with all critical threats \
                 automatically confirmed, quarry designated as a free action, and take-20 on \
                 Survival to follow its tracks. This SUPERSEDES Quarry's \
                 +{SLAYER_QUARRY_ATTACK_BONUS} rather than stacking with it: the corpus suppresses \
                 the Quarry Output record whenever anything of TYPE SlayerImprovedQuarry is \
                 present, and Improved Quarry carries exactly that tag -- so this codebase emits \
                 one record or the other, never both. The magnitude is transcribed from the \
                 record's DESC prose, which is the only place it exists: the record carries no \
                 BONUS or DEFINE token. Grounds the bonus only -- no attack roll is computed \
                 against a designated quarry here"
            ),
        });
    } else if level >= SLAYER_QUARRY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.slayer.quarry_attack_bonus".to_owned(),
            value: SLAYER_QUARRY_ATTACK_BONUS,
            detail: format!(
                "Slayer level {level} Quarry (granted at level {SLAYER_QUARRY_LEVEL}): a \
                 +{SLAYER_QUARRY_ATTACK_BONUS} insight bonus on attack rolls against his quarry, \
                 with all critical threats automatically confirmed and take-10 on Survival to \
                 follow its tracks. The magnitude is transcribed from DESC prose -- the corpus \
                 models Quarry as a hidden VISIBLE:NO dispatcher record carrying no DESC and no \
                 numerics, which grants the visible `Slayer ~ Quarry Output` record that holds \
                 the actual text. Neither carries a BONUS token. Replaced entirely by Improved \
                 Quarry from level {SLAYER_IMPROVED_QUARRY_LEVEL}. Grounds the bonus only; the \
                 quarry target itself is not modelled"
            ),
        });
    }

    if level >= SLAYER_MASTER_SLAYER_LEVEL {
        let intelligence_modifier = ability_modifier(input.chosen.ability_scores.intelligence);
        let dc = slayer_master_slayer_dc(level, intelligence_modifier);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.slayer.master_slayer_dc".to_owned(),
            value: dc,
            detail: format!(
                "Slayer level {level} Master Slayer (granted at level \
                 {SLAYER_MASTER_SLAYER_LEVEL}) save DC: {dc} (10 + level/2 + the Intelligence \
                 MODIFIER, {intelligence_modifier:+}). The stat is INT, per the corpus's own \
                 `10+(MasterSlayerLVL/2)+INT` -- notable because Slayer has no casting stat, so \
                 there is no spellcasting ability to default to. The bare INT token is a \
                 modifier, not a score. Grounds the DC only: the effect it gates (kill, knock \
                 unconscious for 1d4 hours, or paralyze for 2d6 rounds on a failed Fortitude \
                 save) is opponent-directed, and this codebase resolves no saving throw against \
                 a target -- the same split already accepted for Brawler's Knockout DC"
            ),
        });
    }
}
