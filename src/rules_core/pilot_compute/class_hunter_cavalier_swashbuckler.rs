#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm, risks item 8 (fourth APG/ACG closure): ACG Hunter, a
/// Druid+Ranger hybrid whose 1st-level Animal Companion is, per the PCGen
/// corpus's own DESC text, mechanically identical to Druid's own: "the
/// hunter's effective druid level is equal to her hunter level." Unlike
/// Druid's own Nature Bond (a genuine choice between an animal companion
/// and a domain), Hunter's Animal Companion is unconditional -- every
/// Hunter gets one automatically at 1st level, per the corpus text
/// ("At 1st level, a hunter forms a bond with an animal companion",
/// stated as fact, never framed as one of several options) -- so unlike
/// every choice-gated or activation-gated mechanic built this session,
/// Hunter's companion needs neither a `selected_choices` nor a
/// `class_ability_activations` entry at all; it is grounded purely on
/// class ownership and level, mirroring Brawler's own "always on" shape
/// even more directly than Druid's (which needed a bond-type choice
/// check). The species choice the corpus also names ("any of the
/// animals on the druid list") is handled the same way Druid's own was:
/// this codebase models no species-selection input at all, so Wolf is
/// assumed as the canonical species (the same "smallest defensible
/// slice" choice, not a new reachability gap -- Druid's own build never
/// asked the character to pick a species either).
pub(super) const HUNTER_CLASS_ID: &str = "class:hunter";

/// PF1 Advanced Class Guide Wild Empathy: "adds ... Hunter level +
/// Charisma modifier + any other bonuses" -- verified directly against
/// `acg_abilities_class.lst`'s own `BONUS:VAR|HunterWildEmpatyBonus|
/// CHA+HunterLVL` (deepening 2026-07-26, task #2). A flat, unconditional
/// check-modifier fact (not an activated ability with any duration or
/// budget, unlike Animal Focus below) -- grounds as a standalone
/// explanation record, no live consumer needed, the same corrected bar
/// Inquisitor's Monster Lore/Cunning Initiative/Track and Skald's Bardic
/// Knowledge/Damage Reduction already established. No `class_ability_
/// activations`/`selected_choices` entry is needed at all -- it is a
/// passive check modifier, not an activated ability.
/// `ClassAbilityActivation.ability_id` for Hunter Animal Focus -- unlike
/// Wild Empathy, this IS a real activated ability ("swift action...
/// usable %1 minutes per day", `BONUS:VAR|HunterAnimalFocusMinutes|
/// HunterLVL`), the same activation-gating pattern Judgment/Rage/Mutagen
/// already use, with a genuinely enforced per-day minutes budget.
pub(super) const HUNTER_ANIMAL_FOCUS_ABILITY_ID: &str = "animal_focus";

/// The choice set for which Animal Focus type is currently active. 13
/// real options exist in the corpus (`KEY:Hunter Animal Focus ~ ...`).
///
/// **SD-32 T12 Epic 8 row 18 cycle 17: widened from Bull-only (the prior
/// canonical-narrowing choice) to all 11 magnitude-bearing options plus
/// Mouse's boolean evasion posture and No Ability's text-only posture --
/// the full 13, a generic pass rather than a per-object one (`§17`).**
/// Every magnitude-bearing option (`HUNTER_ANIMAL_FOCUS_TIERED_OPTIONS`
/// below) shares the SAME real corpus shape Bull already proved: a base
/// `BONUS:VAR|HunterAnimalFocus<Name><Suffix>|<N>` plus two more additive
/// `PREVARGTEQ:HunterAnimalFocusLVL,8`/`,15`-gated increments -- verified
/// directly against each option's own `data/corpus/advanced_class_guide/
/// class_feature/hunter_animal_focus/<name>.json`. Mouse (`Get evasion...
/// and improved evasion|PREVARGTEQ:HunterAnimalFocusLVL,12`) and No
/// Ability (`Get nothing`, no `BONUS:VAR` at all) are genuinely different
/// shapes -- a boolean posture fact and a text-only fact respectively --
/// handled as their own small branches rather than forced into the
/// tiered-magnitude table.
pub(super) const HUNTER_ANIMAL_FOCUS_CHOICE_ID: &str = "choice:hunter_animal_focus";

// Named only by test fixtures below (production code matches selection_id
// strings generically against `HUNTER_ANIMAL_FOCUS_TIERED_OPTIONS`, never by
// this name) -- `#[cfg(test)]` here, not an `#[allow(dead_code)]`, is the
// real fix: it is genuinely test-only, not a false-positive lint.
#[cfg(test)]
pub(super) const HUNTER_ANIMAL_FOCUS_BULL_SELECTION_ID: &str = "animal_focus:bull";

/// One magnitude-bearing Animal Focus option's real corpus shape: a base
/// bonus plus two more additive tiers at Hunter level 8 and 15
/// (`PREVARGTEQ:HunterAnimalFocusLVL,8`/`,15`, `HunterAnimalFocusLVL`
/// itself unconditionally `HunterLVL` -- same chain Bull's own doc cites).
pub(super) struct HunterAnimalFocusTieredOption {
    /// The `choice:hunter_animal_focus` selection id, `"animal_focus:<slug>"`.
    selection_id: &'static str,
    /// Human-readable label used only in explanation/diagnostic text.
    label: &'static str,
    /// What the magnitude is a bonus TO, for the explanation text (e.g.
    /// `"enhancement bonus to Strength"`, `"competence bonus to Perception"`,
    /// `"feet of darkvision range"`).
    benefit: &'static str,
    base: i16,
    per_level_8: i16,
    per_level_15: i16,
}

/// SD-32 T12 Epic 8 row 18 cycle 17: the 11 magnitude-bearing Animal Focus
/// options (of the 13 total; Mouse and No Ability are handled separately,
/// see `HUNTER_ANIMAL_FOCUS_CHOICE_ID`'s own doc). Each row verified
/// directly against its own corpus record:
/// - Bull: `STAT|STR|...|TYPE=Enhancement`, `2/2/2` (cycle predates this
///   table; re-verified unchanged).
/// - Bear: `STAT|CON|...|TYPE=Enhancement`, `2/2/2`.
/// - Tiger: `STAT|DEX|...|TYPE=Enhancement`, `2/2/2` -- named in Bull's own
///   prior doc as "structurally identical," now built.
/// - Falcon: `SKILL|Perception|...|TYPE=Competence`, `4/2/2`.
/// - Frog: `SITUATION|Acrobatics=When Jumping` + `SKILL|Swim|...
///   |TYPE=Competence`, `4/2/2` (one magnitude, two skills -- the corpus's
///   own single `HunterAnimalFocusFrogBonus` target feeds both BONUS rows).
/// - Monkey: `SKILL|Climb|...|TYPE=Competence`, `4/2/2`.
/// - Owl: `SKILL|Stealth|...|TYPE=Competence`, `4/2/2`.
/// - Snake: no `BONUS:` target beyond the bare `VAR` itself (a flat
///   attack-of-opportunity/AC bonus with no integrated total anywhere in
///   this codebase, the same "standalone fact" bar Wild Empathy already
///   established), `2/2/2`.
/// - Stag: `MOVEADD|TYPE.Walk|...`, `5/5/5`.
/// - Wolf: `VAR|ScentRange|HunterAnimalFocusWolfScent` (scent range in
///   feet), `10/10/10`.
/// - Bat: `VAR|DarkvisionRange|HunterAnimalFocusBatSight|TYPE=Base`
///   (darkvision range in feet), `60/30/0` -- the real corpus record has
///   NO third `PREVARGTEQ:...,15` numeric tier on the sight-range `VAR`
///   itself; level 15's own benefit is the SEPARATE boolean "blindsense to
///   10 feet" fact, handled in `ground_or_block_hunter_animal_focus`'s own
///   Bat branch rather than folded into this numeric table.
pub(super) const HUNTER_ANIMAL_FOCUS_TIERED_OPTIONS: &[HunterAnimalFocusTieredOption] = &[
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:bull",
        label: "Bull",
        benefit: "enhancement bonus to Strength",
        base: 2,
        per_level_8: 2,
        per_level_15: 2,
    },
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:bear",
        label: "Bear",
        benefit: "enhancement bonus to Constitution",
        base: 2,
        per_level_8: 2,
        per_level_15: 2,
    },
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:tiger",
        label: "Tiger",
        benefit: "enhancement bonus to Dexterity",
        base: 2,
        per_level_8: 2,
        per_level_15: 2,
    },
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:falcon",
        label: "Falcon",
        benefit: "competence bonus to Perception",
        base: 4,
        per_level_8: 2,
        per_level_15: 2,
    },
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:frog",
        label: "Frog",
        benefit: "competence bonus to Swim and to Acrobatics when jumping",
        base: 4,
        per_level_8: 2,
        per_level_15: 2,
    },
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:monkey",
        label: "Monkey",
        benefit: "competence bonus to Climb",
        base: 4,
        per_level_8: 2,
        per_level_15: 2,
    },
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:owl",
        label: "Owl",
        benefit: "competence bonus to Stealth",
        base: 4,
        per_level_8: 2,
        per_level_15: 2,
    },
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:snake",
        label: "Snake",
        benefit: "bonus to attacks of opportunity and to AC against attacks of opportunity",
        base: 2,
        per_level_8: 2,
        per_level_15: 2,
    },
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:stag",
        label: "Stag",
        benefit: "foot enhancement bonus to base land speed",
        base: 5,
        per_level_8: 5,
        per_level_15: 5,
    },
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:wolf",
        label: "Wolf",
        benefit: "feet of scent range",
        base: 10,
        per_level_8: 10,
        per_level_15: 10,
    },
    HunterAnimalFocusTieredOption {
        selection_id: "animal_focus:bat",
        label: "Bat",
        benefit: "feet of darkvision range",
        base: 60,
        per_level_8: 30,
        per_level_15: 0,
    },
];

pub(super) const HUNTER_ANIMAL_FOCUS_MOUSE_SELECTION_ID: &str = "animal_focus:mouse";

pub(super) const HUNTER_ANIMAL_FOCUS_NO_ABILITY_SELECTION_ID: &str = "animal_focus:no_ability";

/// v0.6 alpha swarm, risks item 8 (Cavalier Mount closure, first APG
/// class-specific closure): APG Cavalier's 1st-level Mount is, per the
/// PF1 Core Rulebook, "the same mechanic as a druid's animal companion,
/// using the cavalier's level as his effective druid level" -- verified
/// directly against `apg_abilities_class.lst`. Unconditional on class
/// ownership and level alone (no choice/activation gating needed,
/// mirroring Hunter's own Animal Companion shape), Horse assumed as the
/// canonical species for this codebase's Medium-only Human fixture. This
/// is the FIRST widening of `has_supported_class_chassis` to any APG
/// class -- `ApgClassId::from_class_id_str` mirrors `AcgClassId`'s own
/// structure exactly, so the same exact-match gate-widening discipline
/// applies unchanged.
pub(super) const CAVALIER_CLASS_ID: &str = "class:cavalier";

/// `ClassAbilityActivation.ability_id` for Cavalier's Challenge.
pub(super) const CAVALIER_CHALLENGE_ABILITY_ID: &str = "challenge";

/// Cavalier gains Expert Trainer at 4th level (APG class table).
pub(super) const CAVALIER_EXPERT_TRAINER_LEVEL: u8 = 4;

/// The choice set for which Order a Cavalier swears at 1st level.
pub(super) const CAVALIER_ORDER_CHOICE_ID: &str = "choice:cavalier_order";

/// Challenge's self-applied Armor Class penalty while a challenge is
/// active.
///
/// **Evidentiary caveat, deliberately recorded rather than glossed**:
/// unlike Bloodrage's own `-2`, which has a real
/// `BONUS:VAR|BloodrageACPenalty|-2` token, Challenge's penalty exists
/// ONLY in the record's `DESC:` prose ("The cavalier takes a -2 penalty
/// to his Armor Class, except against attacks made by the target of his
/// challenge"). Verified directly: `KEY:Cavalier ~ Challenge` carries no
/// `BONUS:COMBAT` token of any kind. The mechanic matches Bloodrage's
/// shape, but the corpus evidence is the weaker Panache-style path, and
/// the two should not be conflated.
///
/// The "except against the target of his challenge" exception is NOT
/// modelled -- that would need the same opponent-tracking this engine
/// lacks -- so the penalty is applied unconditionally while challenging,
/// which is the strictly more conservative reading.
pub(super) const CAVALIER_CHALLENGE_ARMOR_CLASS_PENALTY: i16 = -2;

/// Cavalier's Challenge uses per day: `(CavalierLVL+2)/3`, verified
/// directly against `apg_abilities_class.lst`'s own
/// `BONUS:VAR|CavalierChallengeTimes|(CavalierLVL+2)/3`.
pub fn cavalier_challenge_uses_per_day(level: u8) -> i16 {
    (i16::from(level) + 2) / 3
}

/// Cavalier's bonus COMBAT feat count: `CavalierLVL/6` (1 at 6th, 2 at
/// 12th, 3 at 18th), verified against
/// `BONUS:ABILITYPOOL|Cavalier Feat|CavalierLVL/6`.
///
/// Three `.MOD` records each subtract 1 from this pool
/// (`TYPE.CavalierCavaliersBonusFeat6/12/18`), but every one is gated on
/// a specific archetype, and this repo ingests no Cavalier archetype at
/// all -- provably vacuous here, the same check that cleared Alchemist's
/// Gnome-only and Ultimate-Magic-gated Bomb terms.
pub fn cavalier_bonus_combat_feat_count(level: u8) -> i16 {
    i16::from(level) / 6
}

/// Cavalier's teamwork feat count: Tactician at 1st, Greater Tactician
/// at 9th, and Master Tactician at 17th each add
/// `BONUS:ABILITYPOOL|Tactician Teamwork Feat|1`, so the count is 1/2/3.
///
/// Only the COUNT grounds. Each Tactician tier also grants the chosen
/// feat to allies within 30 feet for a few rounds, which is ally-scoped
/// and stays deferred, consistent with Skald's Raging Song.
pub(super) fn cavalier_teamwork_feat_count(level: u8) -> i16 {
    let level = i16::from(level);
    let mut count = 0;
    if level >= 1 {
        count += 1;
    }
    if level >= 9 {
        count += 1;
    }
    if level >= 17 {
        count += 1;
    }
    count
}

/// Expert Trainer's Handle Animal bonus when handling a mount:
/// `CavalierLVL/2`, granted at 4th level.
///
/// **Evidentiary caveat**: this magnitude lives only in the record's
/// `DESC:` substitution parameter (`|CavalierLVL/2`) -- there is no
/// `BONUS:SKILL` token at all. Same weaker path as Panache and
/// Challenge's AC penalty. Cross-checked against the published rule text
/// ("a +1/2 his cavalier level bonus whenever he uses Handle Animal on
/// an animal that serves as a mount"), which agrees.
pub(super) fn cavalier_expert_trainer_bonus(level: u8) -> i16 {
    i16::from(level) / 2
}

/// Order of the Sword's own order bonus: a competence bonus on Sense
/// Motive checks made to oppose a Bluff check, equal to `1/2 cavalier
/// level (minimum +1)`.
///
/// Grounds despite being an opposed check, per the ruling already made
/// for Oracle's Deaf: a flat modifier applying to the character's OWN
/// roll clears the bar; what does not is a bonus needing a persistent
/// tracked relationship with a specific opponent.
///
/// **Evidentiary caveat**: DESC-sourced, like Expert Trainer.
pub(super) fn cavalier_order_of_the_sword_sense_motive_bonus(level: u8) -> i16 {
    (i16::from(level) / 2).max(1)
}

/// Order of the Dragon's own order bonus: a competence bonus on Survival
/// checks made to provide food/water for allies or protect them from harsh
/// weather, equal to `1/2 cavalier level (minimum +1)` --
/// `apg_abilities_class.lst:243`'s own `DESC:...|max(1,CavalierLVL/2)`
/// substitution argument. The identical formula shape to Order of the
/// Sword's Sense Motive bonus above (byte-for-byte the same
/// `max(1,CavalierLVL/2)` expression), grounds for the identical reason: a
/// flat modifier applying to the character's own roll, DESC-sourced like
/// Order of the Sword's own bonus.
///
/// **Evidentiary caveat**: this record's OTHER magnitude,
/// `OrderChallengeBonus|CavalierLVL/4` (a circumstance bonus on melee
/// attack rolls against the character's OWN challenge target), is NOT
/// grounded here -- it is opponent-conditioned, the same reason the other
/// four un-grounded orders' challenge riders stay deferred. Aid Allies'
/// own ally-scoped bonus (a separate corpus record) likewise stays
/// deferred.
pub(super) fn cavalier_order_of_the_dragon_survival_bonus(level: u8) -> i16 {
    (i16::from(level) / 2).max(1)
}

/// Cavalier's Challenge Armor Class penalty when a challenge is actively
/// declared (task #6, 2026-07-27). `None` for every non-Cavalier or
/// not-currently-challenging character, so this is class-ownership-gated
/// by construction, mirroring `active_bloodrager_bloodrage_bonus`.
///
/// Deliberately NOT budget-checked against
/// `cavalier_challenge_uses_per_day`: a challenge lasts until the target
/// dies or the encounter ends, so "rounds consumed today" has no
/// meaning here the way it does for Rage or Bardic Performance. Naming
/// that rather than inventing an enforcement the rules do not have.
pub(super) fn active_cavalier_challenge_armor_class_penalty(input: &CharacterInput) -> Option<i16> {
    input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == CAVALIER_CLASS_ID)?;
    let activation = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == CAVALIER_CHALLENGE_ABILITY_ID)?;
    (activation.active_state == ActiveState::EquippedActive)
        .then_some(CAVALIER_CHALLENGE_ARMOR_CLASS_PENALTY)
}

/// v0.6 alpha swarm, risks item 8 (Swashbuckler full-build closure, 9th
/// ACG/APG class-specific closure): APG Swashbuckler, verified directly
/// against `acg_classes.lst`'s own confirmed non-caster status (no
/// `SPELLSTAT` token at all) -- zero spellcasting scope, the same
/// cheapest structural shape as Slayer. This closure was mis-scoped as
/// "harder" in the second comparative pass (lumped in with Investigator/
/// Shaman/Summoner/Witch's own real subsystem gaps) -- corrected in the
/// third comparative pass after checking the real corpus directly rather
/// than trusting the prior label. See
/// `docs/release/v0.6/third-full-class-build-comparative-scoping.md` for
/// the full corpus verification and scope record.
pub(super) const SWASHBUCKLER_CLASS_ID: &str = "class:swashbuckler";

/// `ClassAbilityActivation.ability_id` for Charmed Life.
pub(super) const SWASHBUCKLER_CHARMED_LIFE_ABILITY_ID: &str = "charmed_life";

/// Charmed Life is granted starting at 2nd level (verified via a real
/// web search, since `acg_abilities_class.lst`'s own Charmed Life record
/// carries no `PRELEVEL`/level-gate token at all -- the corpus's
/// `BONUS:VAR|SwashbucklerCharmedLifeTimes|((SwashbucklerLVL-2)/4)+3`
/// formula alone doesn't reveal when the feature first applies): "at 2nd
/// level... three times per day... at 6th level and every 4 levels
/// thereafter, the number... increases by one." This resolves the open
/// verification question from the scoping doc (whether the formula's
/// negative-operand behavior at level 1 matters) by making it moot --
/// level 1 never evaluates this formula at all, the feature simply isn't
/// granted yet, mirroring `ORACLE_KNOWN_SPELLS_SUPPORTED_MAX_LEVEL`'s own
/// "bound the scope, don't guess at the edge" discipline.
pub(super) const SWASHBUCKLER_CHARMED_LIFE_MIN_LEVEL: u8 = 2;

/// v0.6 alpha swarm, risks item 8 (Cavalier Mount closure): whether
/// `input` is a single-class Cavalier at a level within
/// `apg::class_chassis_resolve`'s declared ceiling for Cavalier -- the
/// FIRST widening of this gate to any APG class, mirroring the four ACG
/// exact-match gates exactly (`== Some(ApgClassId::Cavalier)`, not a
/// broad `.is_some()` that would admit any of the 6 APG classes).
pub(super) fn is_supported_cavalier_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if ApgClassId::from_class_id_str(&class_level.class_id) != Some(ApgClassId::Cavalier) {
        return false;
    }
    apg::class_chassis_resolve(ApgClassId::Cavalier, class_level.level, RuleSetId::Apg).is_some()
}

/// v0.6 alpha swarm, risks item 8 (fourth APG/ACG closure): whether
/// `input` is a single-class Hunter at a level within
/// `acg::class_chassis_resolve`'s declared ceiling for Hunter -- mirrors
/// the other three exact-match gates exactly.
pub(super) fn is_supported_hunter_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if AcgClassId::from_class_id_str(&class_level.class_id) != Some(AcgClassId::Hunter) {
        return false;
    }
    acg::class_chassis_resolve(AcgClassId::Hunter, class_level.level, RuleSetId::Acg).is_some()
}

/// v0.6 alpha swarm, risks item 8 (Swashbuckler full-build closure):
/// whether `input` is a single-class Swashbuckler at a level within
/// `acg::class_chassis_resolve`'s declared ceiling for Swashbuckler --
/// mirrors the other seven ACG/APG exact-match gates exactly.
pub(super) fn is_supported_swashbuckler_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if AcgClassId::from_class_id_str(&class_level.class_id) != Some(AcgClassId::Swashbuckler) {
        return false;
    }
    acg::class_chassis_resolve(AcgClassId::Swashbuckler, class_level.level, RuleSetId::Acg)
        .is_some()
}

/// Grounds Cavalier's 1st-level Mount (v0.6 alpha swarm, risks item 8,
/// Cavalier Mount closure, first APG class-specific closure) by reusing
/// the "Animal Companion Base Statistics" shared progression math via
/// the new parallel Horse-specific helpers -- the PF1 Core Rulebook
/// confirms this is the same mechanic as Druid's/Hunter's own companion
/// ("This mount functions as a druid's animal companion, using the
/// cavalier's level as his effective druid level"), just a different
/// species. Called from `compute_apg_class_chassis`'s Cavalier branch,
/// unconditional on Cavalier class ownership and level alone: unlike
/// Druid's own choice-gated Nature Bond, every Cavalier gets a Mount
/// automatically at 1st level (the corpus states this as fact, never as
/// one of several options), the same "always on, no gate" shape Hunter's
/// own Animal Companion already established. Horse is assumed as the
/// canonical species for a Medium cavalier (this codebase's only race;
/// Camel is the other option, not built this slice; Small-cavalier
/// Pony/Wolf options don't apply to a Human fixture) -- no species-
/// selection input is modeled for any companion-granting class in this
/// codebase, mirroring Wolf's own precedent exactly.
///
/// Cavalier has no `SPELLSTAT` at all (confirmed directly against
/// `apg_classes.lst:42`) -- a pure martial class like Brawler, so the
/// remaining bucket here is Cavalier's OTHER named features
/// (Challenge, Order, Tactician, Cavalier's Charge, and the rest), not
/// deferred spell math -- named via the narrower
/// `class_feature.apg.cavalier.other_features_deferred.unsupported`
/// diagnostic, replacing the generic `class_feature.apg.cavalier
/// .unsupported` diagnostic for Cavalier specifically, mirroring
/// Brawler's own diagnostic-honesty fix exactly. That bucket used to be
/// described here as "permanent" and unconditionally claim-blocking; as
/// of Path A canonical narrowing (2026-07-29) it stops claim-blocking
/// once one of the two canonical Orders this codebase grounds is
/// genuinely recorded, and still claim-blocks in every other posture.
/// **SD-34 wave 44 (`decisions.md §22`, Piece 2 item 2) widened this from
/// one Order to two**: Order of the Dragon's own Survival bonus joins
/// Order of the Sword's Sense Motive bonus below.
/// Grounds Cavalier's named class features (task #6, 2026-07-27):
/// Challenge's uses-per-day pool and self-applied Armor Class penalty,
/// Expert Trainer, the two feat counts, Order of the Sword's own Sense
/// Motive bonus when that Order is recorded, and Order of the Dragon's
/// own Survival bonus when that Order is recorded instead.
///
/// Four of these six magnitudes are DESC-sourced rather than carried on a
/// `BONUS:` token (Challenge's AC penalty, Expert Trainer, Order of the
/// Sword's bonus, and Order of the Dragon's bonus). That is the weaker
/// Panache-shaped evidentiary path, named in each record's own detail
/// text rather than presented as token-verified.
///
/// Returns whether either canonical Order this codebase grounds (Order of
/// the Sword or Order of the Dragon) is genuinely recorded. The caller
/// uses that to decide whether the `other_features_deferred` diagnostic
/// still claim-blocks -- see `ground_cavalier_mount_and_defer_the_rest`
/// (Path A canonical narrowing, 2026-07-29; widened to two Orders, SD-34
/// wave 44).
pub(super) fn ground_cavalier_named_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> bool {
    let challenge_uses = cavalier_challenge_uses_per_day(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.cavalier.challenge_uses_per_day".to_owned(),
        value: challenge_uses,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:VAR|CavalierChallengeTimes|(CavalierLVL+2)/3`
            "Cavalier level {level} Challenge uses per day: (level + 2)/3 = {challenge_uses}. A flat \
             daily pool -- no per-use consumption is tracked. Challenge's own +{level} extra melee \
             damage against the challenge TARGET is deliberately not grounded: it needs a persistent \
             tracked relationship with a specific opponent, the same line already drawn for Slayer's \
             Studied Target and Investigator's Studied Combat"
        ),
    });

    if active_cavalier_challenge_armor_class_penalty(input).is_some() {
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.cavalier.challenge_damage_bonus".to_owned(),
            value: i16::from(level),
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|CavalierChallengeLVL|CavalierLVL`
                "Cavalier level {level} Challenge: +{level} extra damage on attacks made AGAINST THE \
                 TARGET of the challenge. Grounds standalone -- the formula reads only the \
                 cavalier's own level, nothing about the opponent. Note this scope is the INVERSE of \
                 the challenge's Armor Class penalty, which applies against everyone EXCEPT that \
                 target; the two must not be conflated"
            ),
        });
    }

    match active_cavalier_challenge_armor_class_penalty(input) {
        Some(penalty) => explanations.push(ComputationExplanation {
            id: "class_feature.apg.cavalier.challenge_armor_class_penalty".to_owned(),
            value: penalty,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:COMBAT` token at all, unlike Bloodrage's real token
                "Cavalier level {level} is actively challenging, taking a {penalty} penalty to Armor \
                 Class. INTEGRATED into the real `defense.baseline_armor_class` total, the same path \
                 Bloodrage's own -2 already uses. Two honest caveats: the magnitude is DESC-sourced \
                 (the Challenge record carries no), and the rule's \"except against attacks made by \
                 the target of his challenge\" exception is NOT modelled, so the penalty applies \
                 unconditionally here -- the strictly more conservative reading"
            ),
        }),
        None => explanations.push(ComputationExplanation {
            id: "class_feature.apg.cavalier.challenge_not_active".to_owned(),
            value: 0,
            detail: format!(
                "Cavalier level {level} is not currently challenging (no active \
                 class_ability_activations entry for \"{CAVALIER_CHALLENGE_ABILITY_ID}\"): a \
                 genuinely valid posture, so no Armor Class penalty is applied"
            ),
        }),
    }

    if level >= CAVALIER_EXPERT_TRAINER_LEVEL {
        let bonus = cavalier_expert_trainer_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.cavalier.expert_trainer_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Cavalier level {level} Expert Trainer: a +{bonus} bonus (level/2) on Handle \
                 Animal checks made on an animal serving as a mount. Handle Animal is not among \
                 the three skills this engine computes, so this grounds standalone, the same \
                 shape as Bard's Bardic Knowledge. Taken from the record's own \
                 description rather than from a skill-bonus magnitude, which it does not carry, \
                 and cross-checked against the published rule text, which agrees"
            ),
        });
    }

    let bonus_feats = cavalier_bonus_combat_feat_count(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.cavalier.bonus_combat_feat_count".to_owned(),
        value: bonus_feats,
        detail: format!(
            "Cavalier level {level} bonus combat feats: level/6 = {bonus_feats} (1 at 6th, 2 at \
             12th, 3 at 18th). Three `.MOD` records each subtract 1 from this pool, but every \
             one is gated on a specific Cavalier archetype and this repo ingests no Cavalier \
             archetype at all -- provably vacuous here. Only the COUNT is grounded; which feats \
             are chosen is not"
        ),
    });

    let teamwork_feats = cavalier_teamwork_feat_count(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.cavalier.teamwork_feat_count".to_owned(),
        value: teamwork_feats,
        detail: format!(
            "Cavalier level {level} teamwork feats from Tactician: {teamwork_feats} (Tactician at \
             1st, Greater Tactician at 9th, Master Tactician at 17th, each adding one). Only the \
             count grounds -- each tier also GRANTS the chosen feat to allies within 30 feet, \
             which is ally-scoped and stays deferred, consistent with Skald's Raging Song"
        ),
    });

    // SD-32 T12 Epic 8 row 18 cycle 7: the same "select ONE named group, inherit every one of its
    // real corpus members" generic pass cycles 5/6 wired for Sorcerer/Cleric/Bloodrager's own
    // Bloodline/Domain pools -- Cavalier's "Order of the X" family is the SAME shape (see
    // `real_pool_group_for_selection_slug`'s own doc, "THIRD real corpus naming/ownership
    // shape"), purely additive alongside the hand-modelled Order of the Sword branch below (a
    // DIFFERENT id prefix -- `class_feature.apg.cavalier.order.generic.*` vs
    // `...order_of_the_sword.*` -- proven non-colliding by
    // `cavalier_generic_order_pass_does_not_collide_with_the_hand_modelled_order_of_the_sword`).
    push_generic_pool_group_selection_magnitude(
        input,
        level,
        ability_modifiers,
        CAVALIER_ORDER_CHOICE_ID,
        "Cavalier",
        "Order",
        "order:",
        "class_feature.apg.cavalier.order.generic",
        1,
        explanations,
    );

    // SD-32 T12 Epic 8 row 18 cycle 15: the sibling generic pass for the DIFFERENT corpus shape
    // cycle 14's own §16 finding named -- Order of the Beast's two members carry no BONUS:VAR at
    // all but DO carry a real %N-substituted DESC formula, a shape the resolver above
    // (bonus_vars-only) can never reach. No hand-modelled Order of the Beast function exists, so
    // there is no exclusion list here (unlike Warpriest's own Destruction/Strength).
    push_generic_pool_group_selection_description_magnitude(
        input,
        level,
        ability_modifiers,
        CAVALIER_ORDER_CHOICE_ID,
        "Cavalier",
        "Order",
        "order:",
        "class_feature.apg.cavalier.order_description.generic",
        1,
        &[],
        explanations,
    );

    let order_selected = input
        .chosen
        .selected_choices
        .iter()
        .any(|c| c.choice_set_id == CAVALIER_ORDER_CHOICE_ID
            && c.selection_id == ORDER_OF_THE_SWORD_SELECTION);
    if order_selected {
        let sense_motive = cavalier_order_of_the_sword_sense_motive_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.cavalier.order_of_the_sword.sense_motive_bonus".to_owned(),
            value: sense_motive,
            detail: format!(
                "Cavalier level {level} with the Order of the Sword gains a +{sense_motive} \
                 competence bonus (1/2 level, minimum +1) on Sense Motive checks made to oppose \
                 a Bluff check. Grounds despite being an opposed check, per the ruling already \
                 made for Oracle's Deaf: the magnitude is fixed and applies to this character's \
                 own roll. Sense Motive is not among the three skills this engine computes, so \
                 this grounds standalone. DESC-sourced. The Order's own challenge rider (attack \
                 rolls while mounted) and its By My Honor save bonus are not grounded"
            ),
        });
    }

    // SD-34 wave 44 (`decisions.md §22`, Piece 2 item 2): Order of the
    // Dragon, the second Order this closure grounds -- see
    // `cavalier_order_of_the_dragon_survival_bonus`'s own doc comment for
    // the corpus verification.
    let dragon_order_selected = input
        .chosen
        .selected_choices
        .iter()
        .any(|c| c.choice_set_id == CAVALIER_ORDER_CHOICE_ID
            && c.selection_id == ORDER_OF_THE_DRAGON_SELECTION);
    if dragon_order_selected {
        let survival_bonus = cavalier_order_of_the_dragon_survival_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.cavalier.order_of_the_dragon.survival_bonus".to_owned(),
            value: survival_bonus,
            detail: format!(
                "Cavalier level {level} with the Order of the Dragon gains a +{survival_bonus} \
                 bonus (1/2 level, minimum +1) on Survival checks made to provide food and \
                 water for allies or to protect them from harsh weather. DESC-sourced \
                 (`apg_abilities_class.lst:243`'s own `max(1,CavalierLVL/2)` substitution \
                 argument), the identical shape to Order of the Sword's Sense Motive bonus \
                 above. Grounds standalone: a flat modifier applying to the character's own \
                 roll. The Order's own OrderChallengeBonus (a circumstance bonus on melee \
                 attack rolls against this character's challenge target, opponent-conditioned) \
                 and Aid Allies' own ally-scoped bonus (a separate corpus record) are not \
                 grounded"
            ),
        });
    }

    if !order_selected && !dragon_order_selected {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.apg.cavalier.order_powers.unsupported".to_owned(),
            message: "Cavalier remains blocked on its Order burden: no recognized Order of the \
                 Sword or Order of the Dragon choice is present (these are the two canonical \
                 Orders grounded in this codebase; the other four -- Cockatrice, Lion, Shield, \
                 Star -- carry challenge riders that are opponent- or ally-conditioned), so no \
                 Order support is claimed"
                .to_owned(),
            claim_blocking: true,
        });
    }
    order_selected || dragon_order_selected
}

pub(super) fn ground_cavalier_mount_and_defer_the_rest(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    ground_selected_companion_or_default(
        input,
        "class_chassis.cavalier.mount",
        "Cavalier",
        level,
        ground_horse_companion_stat_block,
        explanations,
    );
    ground_horse_companion_link_vacuous("class_feature.cavalier.mount", "cavalier", explanations);
    let order_of_the_sword_recorded = ground_cavalier_named_features(
        input,
        level,
        ability_modifiers,
        explanations,
        diagnostics,
    );
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.cavalier.mount.advancement_absent".to_owned(),
        message: "Cavalier Mount advancement is grounded for every column that has a consumer \
                   in this engine -- Hit Dice across all twenty master levels (2 HD at 1st \
                   through 16 HD at 20th; apg_companionmods.lst:75-93 grants them at exactly \
                   the same fourteen levels as the Core Rulebook animal companion's own block, \
                   and the cavalier's effective druid level is simply his cavalier level), and \
                   with them base attack bonus, all three base saves and hit points, plus the \
                   natural-armor and Strength advances the armor-class and attack/damage \
                   records consume. Deliberately NOT grounded, because nothing in this codebase \
                   consumes them: the Dexterity half of the stat advance, bonus tricks, the \
                   Mount's skill ranks and feats, the player-chosen Companion Stat Increase at \
                   master levels 4/9/14/20, the optional species size advance offered from \
                   master level 4 for a Horse, and the named abilities Evasion (3rd), Devotion \
                   (6th), Multiattack (9th) and Improved Evasion (16th). Light Armor \
                   Proficiency and \"combat trained\"/no-armor-check-penalty-on-Ride grants \
                   likewise stay ungrounded: this codebase computes no Ride check and no \
                   armor-proficiency-gated-benefit engine at all"
            .to_owned(),
        claim_blocking: false,
    });
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.apg.cavalier.other_features_deferred.unsupported".to_owned(),
        message: format!(
            "{CAVALIER_CLASS_ID} remains blocked beyond its base-attack-bonus/base-save chassis \
             pillar, the Mount, its class-skill list, Challenge's uses-per-day, self-applied \
             Armor Class penalty and its own +level damage bonus, Expert Trainer, the \
             bonus-combat-feat and teamwork-feat counts, and Order of the Sword's own Sense \
             Motive bonus (or Order of the Dragon's own Survival bonus, whichever Order is \
             recorded): Banner and Greater Banner, the charge family (Cavalier's Charge, \
             Mighty Charge, Supreme Charge), Demanding Challenge, the four non-Sword-or-Dragon \
             Orders (Cockatrice, Lion, Shield, Star) and every order's challenge rider \
             (including Sword's and Dragon's own riders), Order of the Sword's own By My Honor, \
             and the Tactician family's own grant facet (Tactician/Greater Tactician/Master \
             Tactician each also confer the chosen teamwork feat on allies within 30 feet -- \
             only the tier COUNT grounds) remain ungrounded. The charge family and every \
             challenge rider are blocked on real \
             missing engine state -- a charge action and a persistent opponent relationship -- \
             not on transcription effort; no class-feature execution is fabricated in this \
             bounded chassis baseline. This message previously claimed Challenge's +level \
             damage was ungrounded while listing Challenge's other two facets as grounded: \
             `challenge_damage_bonus` is shipped (task #6), so the message contradicted itself \
             about a single feature's own facets. {}",
            cavalier_deferred_remainder_posture(order_of_the_sword_recorded)
        ),
        claim_blocking: !order_of_the_sword_recorded,
    });
}

/// The one sentence that differs between this diagnostic's claim-blocking
/// and non-claim-blocking forms (Path A canonical narrowing, 2026-07-29).
///
/// With a canonical Order genuinely recorded, Cavalier's remaining gap is
/// the same shape Arcanist's own `exploits_deferred` already carries once
/// Metamagic Knowledge is recognized: a real, named, still-unbuilt
/// remainder that no longer blocks the claim, because the class's
/// corpus-verified canonical chooser options ARE grounded and nothing in
/// the computed output depends on the deferred rest. Without it, the
/// original claim-blocking posture is preserved byte-for-byte in behavior
/// -- see `apg_canonical_choice_path_a_tests`, which pins both halves.
///
/// **SD-34 wave 44:** the caller passes a single bool (Sword OR Dragon
/// recorded), not which specific Order -- so this text deliberately names
/// BOTH Orders generically rather than asserting a specific one was
/// picked (asserting "Order of the Sword" unconditionally would be false
/// prose for a character who recorded Order of the Dragon instead, the
/// exact fabrication shape this bundle's own doctrine forbids).
pub(super) fn cavalier_deferred_remainder_posture(canonical_order_recorded: bool) -> &'static str {
    if canonical_order_recorded {
        "This character HAS recorded one of the two canonical Orders this codebase grounds \
         (Order of the Sword, whose own Sense Motive bonus is computed above, or Order of the \
         Dragon, whose own Survival bonus is computed above), so this remainder is named but \
         no longer claim-blocking -- the same canonical-narrowing posture Arcanist's own \
         exploits_deferred and Cleric's own domain seam already ship. Every item listed above \
         genuinely remains ungrounded; none of them is silently fabricated."
    } else {
        "No canonical Order is recorded, so this remainder stays claim-blocking."
    }
}

/// Swashbuckler Weapon Mastery's critical-multiplier increase: +1
/// (x2 becomes x3, and so on), at 20th level.
///
/// The parent record `KEY:Swashbuckler ~ Swashbuckler Weapon Mastery`
/// (`acg_abilities_class.lst:2044`) carries **no numeric token at all**;
/// it only grants two Internal helper records, and the magnitude lives
/// on those: `TEMPBONUS:EQ|Weapon,Melee,Light,Piercing|WEAPON|
/// CRITMULTADD|1|TYPE=NonStackingCrit` and its OneHanded twin
/// (`:2047` and `:2048`). Reading only the parent would have concluded
/// this feature is zero-magnitude, which is exactly the
/// read-the-whole-record trap.
pub(super) const SWASHBUCKLER_WEAPON_MASTERY_CRIT_MULTIPLIER_INCREASE: i16 = 1;

/// Dizzying Defense's improved fighting-defensively dodge bonus (+4) and
/// reduced attack penalty (-2), at 15th level.
///
/// Both numbers exist **only in the record's DESC prose**
/// (`acg_abilities_class.lst:2091`); the record carries no BONUS or
/// DEFINE token. Transcribed verbatim from "the dodge bonus to AC gained
/// from that action increases to +4, and the penalty to attack rolls is
/// reduced to -2".
pub(super) const SWASHBUCKLER_DIZZYING_DEFENSE_DODGE_BONUS: i16 = 4;

/// See `SWASHBUCKLER_DIZZYING_DEFENSE_DODGE_BONUS`.
pub(super) const SWASHBUCKLER_DIZZYING_DEFENSE_ATTACK_PENALTY: i16 = -2;

/// Swashbuckler Weapon Mastery's grant level, per `acg_classes.lst:359`.
pub(super) const SWASHBUCKLER_WEAPON_MASTERY_LEVEL: u8 = 20;

/// Swashbuckler's bonus combat feats: `SwashbucklerLVL/4`.
///
/// The corpus supplies this as five separate `.MOD` increments on
/// `Swashbuckler ~ Bonus Feats` (`acg_abilities_class.lst:2051-2055`),
/// each `BONUS:VAR|Pool_SwashbucklerBonusFeat|1` gated
/// `PREVARGTEQ:SwashbucklerLVL,<4|8|12|16|20>`. Summing those five gates
/// is exactly integer `level/4` across levels 1-20, so this is the
/// faithful closed form rather than a simplification.
///
/// The parent record itself carries **no** increment -- only
/// `DEFINE:Pool_SwashbucklerBonusFeat|0` and the fighter-level
/// equivalence `BONUS:VAR|FighterWeaponQualifyLVL|SwashbucklerLVL`
/// already grounded separately. Reading the parent alone yields 0 feats
/// at every level.
///
/// Each `.MOD` also carries a `PREVAREQ:Swashbuckler_CF_BonusFeatN,0`
/// suppression flag. Every setter of those flags is a Swashbuckler
/// ARCHETYPE record (Daring Infiltrator, Mysterious Avenger), and this
/// repo ingests only the base `swashbuckler.json`, so all five are
/// provably vacuous here -- the same archetype-deduction check already
/// applied to Brawler's seven and Slayer's ten.
pub(super) fn swashbuckler_bonus_feat_count(level: u8) -> i16 {
    i16::from(level) / 4
}

/// The Swashbuckler deeds whose corpus records carry **no numeric token
/// of any kind**, as `(deed tier, display name, corpus DESC excerpt)`.
///
/// Each was verified field by field on its own record rather than by a
/// filtered grep: every one is `KEY` + `SORTKEY` + `CATEGORY` + `TYPE` +
/// `PREVARGTEQ` + `DESC` + `SOURCEPAGE` + `ASPECT`, with no `BONUS`, no
/// `DEFINE`, and no `TEMPBONUS`. Their whole benefit is a resolution at
/// the table, so they ground as bounded grant-only identity records --
/// the Arcane Apotheosis idiom -- rather than being reported as
/// unbuilt engine work that could never close.
///
/// **The key namespace is `Swashbuckler ~ <Name>`, not
/// `Swashbuckler Deed ~ <Name>`.** `Swashbuckler Deed` appears in the
/// corpus only as a TYPE segment; a grep for `KEY:Swashbuckler Deed ~`
/// returns zero and would suggest these records do not exist.
///
/// Deliberately excluded from this table, because they are NOT
/// zero-magnitude and are grounded with real numbers separately:
/// Swashbuckler Initiative (+2 initiative) and Dizzying Defense
/// (+4 dodge / -2 attack).
///
/// Also note `KEY:Swashbuckler ~ Daring` and
/// `KEY:Swashbuckler ~ Martial Training` in the APG are **Rogue
/// archetype talents** sharing this key prefix -- not ACG Swashbuckler
/// class features, and correctly absent here.
pub(super) const SWASHBUCKLER_ZERO_MAGNITUDE_DEEDS: &[(u8, &str, &str)] = &[
    (
        1,
        "Opportune Parry and Riposte",
        "when an opponent makes a melee attack against the swashbuckler, she can spend 1 panache \
         point and expend a use of an attack of opportunity to attempt to parry that attack ... \
         If her result is greater than the attacking creature's result, the creature's attack \
         automatically misses",
    ),
    (
        3,
        "Kip-Up",
        "while the swashbuckler has at least 1 panache point, she can kip-up from prone as a move \
         action without provoking an attack of opportunity. She can kip-up as a swift action \
         instead by spending 1 panache point",
    ),
    (
        3,
        "Menacing Swordplay",
        "when a swashbuckler hits an opponent with a light or one-handed piercing melee weapon, \
         she can choose to use Intimidate to demoralize that opponent as a swift action instead \
         of a standard action",
    ),
    (
        7,
        "Swashbuckler's Grace",
        "while the swashbuckler has at least 1 panache point, she takes no penalty for moving at \
         full speed when she uses Acrobatics to attempt to move through a threatened area or an \
         enemy's space",
    ),
    (
        7,
        "Superior Feint",
        "a swashbuckler with at least 1 panache point can, as a standard action, purposefully \
         miss a creature she could make a melee attack against with a wielded light or one-handed \
         piercing weapon. When she does, the creature is denied its Dexterity bonus to AC until \
         the start of the swashbuckler's next turn",
    ),
    (
        7,
        "Targeted Strike",
        "as a full-round action the swashbuckler can spend 1 panache point to make an attack with \
         a single light or one-handed piercing melee weapon that cripples part of a foe's body \
         ... Arms: drops one carried item. Head: confused for 1 round. Legs: knocked prone. \
         Torso or Wings: staggered for one round",
    ),
    (
        11,
        "Subtle Blade",
        "while a swashbuckler has at least 1 panache point, she is immune to disarm, steal, and \
         sunder combat maneuvers made against a light or one-handed piercing melee weapon she is \
         wielding",
    ),
    (
        11,
        "Evasive",
        "while a swashbuckler has at least 1 panache point, she gains the benefits of the \
         evasion, uncanny dodge, and improved uncanny dodge rogue class features. She uses her \
         swashbuckler level as her rogue level for improved uncanny dodge",
    ),
    (
        15,
        "Perfect Thrust",
        "while the swashbuckler has at least 1 panache point, she can as a full-round action make \
         a perfect thrust ... she makes the attack against the target's touch AC, and ignores all \
         damage reduction",
    ),
    (
        15,
        "Swashbuckler's Edge",
        "while the swashbuckler has at least 1 panache point, she can take 10 on any Acrobatics, \
         Climb, Escape Artist, Fly, Ride, or Swim check, even while distracted or in immediate \
         danger",
    ),
    (
        19,
        "Cheat Death",
        "whenever the swashbuckler is reduced to 0 or fewer hit points, she can spend all of her \
         remaining panache points (minimum 1) to instead be reduced to 1 hit point",
    ),
];

/// Derring-Do's uses per day: `BONUS:VAR|DerringDoTimes|DEX` -- the
/// Dexterity MODIFIER, not the score. Tier 1.
pub(super) fn swashbuckler_derring_do_uses(dexterity_modifier: i16) -> i16 {
    dexterity_modifier
}

/// Dodging Panache's dodge bonus to Armor Class:
/// `BONUS:VAR|DodgingPanacheBonus|CHA`. Tier 1.
///
/// Reported verbatim even when negative: a low-Charisma swashbuckler
/// genuinely gets a negative value from this formula, and the honest
/// reading is to say so rather than silently clamp to 0.
pub(super) fn swashbuckler_dodging_panache_bonus(charisma_modifier: i16) -> i16 {
    charisma_modifier
}

/// Precise Strike's bonus weapon damage: `SwashbucklerDeedsLVL`, doubled
/// on a critical hit (`2*SwashbucklerDeedsLVL`). Tier 3. The
/// weapon-damage idiom already used by Bomb and Sacred Weapon.
pub(super) fn swashbuckler_precise_strike_damage(level: u8) -> i16 {
    i16::from(level)
}

/// Bleeding Wound's bleed damage: `BONUS:VAR|BleedingWoundDamage|DEX`.
/// Tier 11.
pub(super) fn swashbuckler_bleeding_wound_damage(dexterity_modifier: i16) -> i16 {
    dexterity_modifier
}

/// The save DC shared byte-identically by Deadly Stab and Stunning Stab:
/// `SwashbucklerDeedsLVL/2 + 10 + DEX`. Both are tier 19, and both use
/// this one formula -- two records, one mechanism.
pub(super) fn swashbuckler_stab_save_dc(level: u8, dexterity_modifier: i16) -> i16 {
    i16::from(level) / 2 + 10 + dexterity_modifier
}

/// Grounds the six Swashbuckler deeds that carry real magnitudes
/// (task #14, 2026-07-27), each at its own corpus tier. The other 15
/// deeds carry no `BONUS`/`DEFINE` token of any kind and are genuine
/// no-ops in the Nature Training sense, not a transcription backlog.
pub(super) fn ground_swashbuckler_deeds(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let dexterity = ability_modifiers.dexterity;
    let charisma = ability_modifiers.charisma;

    if swashbuckler_deed_tier_reached(level, 1) {
        let uses = swashbuckler_derring_do_uses(dexterity);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.swashbuckler.derring_do_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Swashbuckler level {level} Derring-Do (deed tier 1): usable {uses} times per \
                 day, equal to the Dexterity modifier ({dexterity:+}). A flat daily pool -- the \
                 deed's own \"roll d6 and add it, rerolling on a 6\" resolution is not modelled"
            ),
        });
        let dodge = swashbuckler_dodging_panache_bonus(charisma);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.swashbuckler.dodging_panache_dodge_bonus".to_owned(),
            value: dodge,
            detail: format!(
                "Swashbuckler level {level} Dodging Panache (deed tier 1): a {dodge:+} dodge \
                 bonus to Armor Class against one attack, equal to the Charisma modifier \
                 ({charisma:+}). Grounds standalone rather than integrating into the armor-class \
                 total: it applies against a single triggering attack the engine cannot \
                 identify, unlike Nature's Whispers which is always on. Reported verbatim even \
                 when negative, which is what the corpus formula genuinely yields at low \
                 Charisma"
            ),
        });
    }

    if swashbuckler_deed_tier_reached(level, 3) {
        let damage = swashbuckler_precise_strike_damage(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.swashbuckler.precise_strike_damage".to_owned(),
            value: damage,
            detail: format!(
                "Swashbuckler level {level} Precise Strike (deed tier 3): +{damage} bonus damage \
                 with a light or one-handed piercing melee weapon, equal to swashbuckler level. \
                 The same weapon-damage idiom as Alchemist's Bomb and Warpriest's Sacred Weapon, \
                 grounded standalone because this engine computes no weapon-damage total"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.swashbuckler.precise_strike_critical_damage".to_owned(),
            value: 2 * damage,
            detail: format!(
                "Swashbuckler level {level} Precise Strike on a critical hit: {} bonus damage \
                 (2 x level), its own separate corpus variable rather than a derived doubling",
                2 * damage
            ),
        });
    }

    if swashbuckler_deed_tier_reached(level, 11) {
        let bleed = swashbuckler_bleeding_wound_damage(dexterity);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.swashbuckler.bleeding_wound_damage".to_owned(),
            value: bleed,
            detail: format!(
                "Swashbuckler level {level} Bleeding Wound (deed tier 11): {bleed} bleed damage \
                 per round, equal to the Dexterity modifier ({dexterity:+}). This engine models \
                 no round-tick damage state, so the magnitude grounds standalone"
            ),
        });
    }

    if swashbuckler_deed_tier_reached(level, 19) {
        let dc = swashbuckler_stab_save_dc(level, dexterity);
        for (id, name) in [
            ("class_feature.acg.swashbuckler.deadly_stab_dc", "Deadly Stab"),
            ("class_feature.acg.swashbuckler.stunning_stab_dc", "Stunning Stab"),
        ] {
            explanations.push(ComputationExplanation {
                id: id.to_owned(),
                value: dc,
                detail: format!(
                    "Swashbuckler level {level} {name} (deed tier 19): Fortitude save DC {dc} \
                     (level/2 + 10 + Dexterity modifier {dexterity:+}). Deadly Stab and Stunning \
                     Stab carry byte-identical DC formulas in the corpus -- two records, one \
                     mechanism"
                ),
            });
        }
    }

    ground_swashbuckler_remaining_deeds_and_mastery(level, explanations);
}

/// Grounds the thirteen Swashbuckler deeds that had no coverage, plus
/// Swashbuckler Weapon Mastery and the bonus-feat pool (task #91).
///
/// Together these were the entire content of Swashbuckler's
/// claim-blocking `other_features_deferred` diagnostic. The split is
/// evidentiary, and the two numeric exceptions are the reason the
/// zero-magnitude table could not simply be applied to all thirteen:
///
/// * **Eleven deeds carry no numeric token at all** -- see
///   `SWASHBUCKLER_ZERO_MAGNITUDE_DEEDS`. Bounded grant-only identity
///   records, value 0.
///
/// * **Swashbuckler Initiative carries a real `TEMPBONUS`** of +2 on
///   initiative checks.
///
/// * **Dizzying Defense carries real numbers in DESC prose only**
///   (+4 dodge, -2 attack while fighting defensively).
///
/// None of the eleven is claimed to be text-complete-to-the-player. The
/// shipped app has no class-feature description surface, so quoting the
/// corpus text here grounds the GRANT and its rulebook basis, not its
/// delivery to a reader.
pub(super) fn ground_swashbuckler_remaining_deeds_and_mastery(
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    for (tier, display_name, description) in SWASHBUCKLER_ZERO_MAGNITUDE_DEEDS {
        if !swashbuckler_deed_tier_reached(level, *tier) {
            continue;
        }
        explanations.push(ComputationExplanation {
            id: format!(
                "class_feature.acg.swashbuckler.deed.{}_grant",
                class_feature_id_slug(display_name)
            ),
            value: 0,
            detail: format!(
                "Swashbuckler level {level} {display_name} (deed tier {tier}, corpus \
                 KEY:Swashbuckler ~ {display_name}): \"{description}\" This is a bounded \
                 grant-only identity record (value 0, non-fabricated): the record carries no \
                 BONUS, no DEFINE and no TEMPBONUS -- verified field by field -- so it has no \
                 magnitude to compute, now or ever. Its benefit is a resolution the player \
                 invokes at the table. The panache costs named in the text are spends against \
                 the Panache pool already grounded separately, not quantities derived here"
            ),
        });
    }

    if swashbuckler_deed_tier_reached(level, 3) {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.swashbuckler.deed.swashbuckler_initiative_bonus".to_owned(),
            value: SWASHBUCKLER_INITIATIVE_BONUS,
            detail: format!(
                "Swashbuckler level {level} Swashbuckler Initiative (deed tier 3): a \
                 +{SWASHBUCKLER_INITIATIVE_BONUS} bonus on initiative checks while she has at \
                 least 1 panache point. Verified against the record's own \
                 TEMPBONUS:PC|COMBAT|INITIATIVE|2. The token is a TEMPBONUS rather than a BONUS \
                 precisely because the benefit is conditional on holding panache, so this grounds \
                 as a standalone record and is deliberately NOT folded into any initiative total \
                 -- doing so would assert the bonus applies unconditionally. The deed's second \
                 clause (drawing a weapon as part of the initiative check, given Quick Draw) is a \
                 resolution with no magnitude"
            ),
        });
    }

    if swashbuckler_deed_tier_reached(level, 15) {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.swashbuckler.deed.dizzying_defense_dodge_bonus".to_owned(),
            value: SWASHBUCKLER_DIZZYING_DEFENSE_DODGE_BONUS,
            detail: format!(
                "Swashbuckler level {level} Dizzying Defense (deed tier 15): fighting \
                 defensively becomes a swift action for 1 panache point, and the dodge bonus to \
                 AC from that action rises to \
                 +{SWASHBUCKLER_DIZZYING_DEFENSE_DODGE_BONUS} while the attack penalty is reduced \
                 to {SWASHBUCKLER_DIZZYING_DEFENSE_ATTACK_PENALTY}. Both magnitudes are \
                 transcribed from the record's DESC prose, which is the only place they exist -- \
                 the record carries no BONUS or DEFINE token. Grounded standalone: this codebase \
                 models no fighting-defensively combat action for either number to modify, so \
                 neither is applied to an armour-class or attack total"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.swashbuckler.deed.dizzying_defense_attack_penalty".to_owned(),
            value: SWASHBUCKLER_DIZZYING_DEFENSE_ATTACK_PENALTY,
            detail: format!(
                "Swashbuckler level {level} Dizzying Defense attack penalty: \
                 {SWASHBUCKLER_DIZZYING_DEFENSE_ATTACK_PENALTY} while fighting defensively in \
                 this manner, replacing the normal -4. Carried as its own facet because a single \
                 `value` cannot express a paired bonus-and-penalty, and because the penalty is \
                 the half a reader is most likely to assume rather than check"
            ),
        });
    }

    if level >= SWASHBUCKLER_WEAPON_MASTERY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.swashbuckler.weapon_mastery_critical_multiplier_increase"
                .to_owned(),
            value: SWASHBUCKLER_WEAPON_MASTERY_CRIT_MULTIPLIER_INCREASE,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   The magnitude is NOT on the parent record, which carries no numeric token at
                //   all -- it lives on the two Internal helper records the parent grants, as
                //   TEMPBONUS:EQ|...|WEAPON|CRITMULTADD|1|TYPE=NonStackingCrit.
                "Swashbuckler level {level} Swashbuckler Weapon Mastery (granted at level \
                 {SWASHBUCKLER_WEAPON_MASTERY_LEVEL}): critical threats with a light or one-handed \
                 piercing melee weapon are automatically confirmed, and such weapons' critical \
                 multipliers increase by {SWASHBUCKLER_WEAPON_MASTERY_CRIT_MULTIPLIER_INCREASE} (x2 \
                 becomes x3, and so on). Reading only the parent would have wrongly concluded this \
                 feature is zero-magnitude. Grounds the increment only: this codebase computes no \
                 critical multiplier for a wielded weapon, and the automatic-confirmation clause is \
                 a resolution with no magnitude"
            ),
        });
    }

    let bonus_feats = swashbuckler_bonus_feat_count(level);
    if bonus_feats > 0 {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.swashbuckler.bonus_feat_count".to_owned(),
            value: bonus_feats,
            detail: format!(
                "Swashbuckler level {level} has {bonus_feats} bonus combat feat slot(s) \
                 (Pool_SwashbucklerBonusFeat), gained at 4th level and every four levels \
                 thereafter -- level/4. The corpus supplies this as five separate .MOD \
                 increments on the Bonus Feats parent, each +1 gated at levels 4/8/12/16/20; the \
                 parent record itself carries no increment, so reading it alone yields 0 at every \
                 level. The five archetype suppression flags on those .MODs are provably vacuous \
                 here (every setter is a Swashbuckler archetype record and only the base class is \
                 ingested). Grounds the SLOT COUNT only -- which combat feat fills each slot is a \
                 chooser over the whole combat-feat list and is never seeded, the same discipline \
                 applied to Brawler's Martial Flexibility. The separate fighter-level-equivalence \
                 facet these slots qualify against is grounded in its own record"
            ),
        });
    }
}

/// same mistake caught and reverted during the Warpriest closure.
/// Grounds Swashbuckler's class features for `level` (v0.6 alpha swarm,
/// risks item 8, Swashbuckler full-build closure, 9th ACG/APG class-
/// specific closure). Called from `compute_acg_class_chassis`'s
/// Swashbuckler branch, gated only on Swashbuckler class-ownership.
/// Panache's max and Nimble's dodge bonus are flat, always-on facts
/// (grounded unconditionally, standalone); Charmed Life is activation-
/// gated with a real per-day budget once granted (2nd level+), mirroring
/// Barbarian's Rage / Inquisitor's Judgment two-check budget-enforcement
/// shape -- but grounded within this one function rather than a separate
/// `active_*_bonus` helper, since (like Warpriest's own Destructive
/// Attacks) nothing else in this codebase consumes a save-total to
/// integrate it into; a separate helper would be genuine dead code, the
pub(super) fn ground_or_block_swashbuckler_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let finesse_effective_intelligence = effective_combat_feat_intelligence_score(
        input.chosen.ability_scores.charisma,
        input.chosen.ability_scores.intelligence,
    );
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.swashbuckler.finesse_effective_intelligence_for_combat_feats"
            .to_owned(),
        value: finesse_effective_intelligence,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:VAR|CombatFeatIntRequirement|max(CHASCORE,INTSCORE)`
            "Swashbuckler Finesse: for the purpose of meeting combat-feat prerequisites, the \
             swashbuckler uses her Charisma score in place of Intelligence when it is higher. Real \
             Charisma {} / Intelligence {} -> effective {finesse_effective_intelligence}. This \
             writes the same corpus variable as Brawler's Cunning, which floors Intelligence at the \
             constant 13 instead -- genuine shared-idiom reuse, with the operand differing per \
             class. Grounds only the flat effective-score fact: this codebase's feat_prereqs modules \
             do not check ability scores as a prerequisite type, so no feat-prerequisite resolution \
             runs against it. Finesse's OTHER half (gaining Weapon Finesse's benefits with light or \
             one-handed piercing weapons) carries no corpus token and needs real attack mechanics, \
             so it stays deferred",
            input.chosen.ability_scores.charisma, input.chosen.ability_scores.intelligence
        ),
    });

    let weapon_training = swashbuckler_weapon_training_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.swashbuckler.weapon_training_bonus".to_owned(),
        value: weapon_training,
        detail: format!(
            "Swashbuckler level {level} Weapon Training: +{weapon_training} on attack and damage \
             rolls with light and one-handed piercing melee weapons ((level - 1)/4 -- +1 at 5th, \
             +2 at 9th, +3 at 13th, +4 at 17th, and genuinely 0 below 5th). Grounds as a \
             standalone magnitude: this engine computes a melee attack total only for its own \
             fixed Longsword posture, which is not a light or piercing weapon, so there is no \
             matching total to layer this onto"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.swashbuckler.fighter_level_equivalence_for_feats".to_owned(),
        value: i16::from(level),
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:VAR|FighterWeaponQualifyLVL|SwashbucklerLVL`
            "Swashbuckler level {level} counts as fighter level {level} for the purpose of \
             qualifying for feats. Grounds the level-equivalence fact only; the feat_prereqs wiring \
             stays deferred, the same treatment already ruled for Brawler's Martial Training"
        ),
    });

    ground_swashbuckler_deeds(level, ability_modifiers, explanations);

    let panache_max =
        swashbuckler_panache_max(ability_modifiers.charisma, &input.chosen.selected_feats);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.swashbuckler.panache_max".to_owned(),
        value: panache_max,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   The base term is confirmed by the corpus's own `Swashbuckler ~ Panache.MOD` record,
            //   which carries a literal BONUS:VAR|PanachePointsBase|max(CHA,1) routed into
            //   Panache_Cap by the Internal Panache Tracker -- the base ability record itself only
            //   DEFINEs these to 0 (see swashbuckler_panache_max's own doc comment).
            "Swashbuckler level {level} Panache: max(1, Charisma modifier ({})) + Extra Panache feat \
             ({:+}) = {panache_max} points at the start of each day. This grounds only the flat \
             daily maximum; spending/regaining Panache on Deeds is not modeled",
            ability_modifiers.charisma,
            extra_resource_feat_bonus(
                &input.chosen.selected_feats,
                EXTRA_PANACHE_FEAT_KEY,
                EXTRA_PANACHE_POINTS
            )
        ),
    });

    let nimble_dodge_bonus = swashbuckler_nimble_dodge_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.swashbuckler.nimble_dodge_bonus".to_owned(),
        value: nimble_dodge_bonus,
        detail: format!(
            "Swashbuckler level {level} Nimble: a +{nimble_dodge_bonus} dodge bonus to AC while \
             wearing light or no armor ((level+1)/4 = {nimble_dodge_bonus}). Grounds only the \
             flat magnitude -- task #88 correction: `defense.baseline_armor_class` IS a real \
             integrated AC total this codebase computes (the same total Brawler's own AC Bonus \
             already integrates into), this magnitude is simply not wired into it yet; grounded \
             as a standalone flat record"
        ),
    });

    match swashbuckler_charmed_life_uses_per_day(level) {
        None => {
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.swashbuckler.charmed_life_not_yet_gained".to_owned(),
                value: 0,
                detail: format!(
                    "Swashbuckler level {level} has not yet gained Charmed Life (granted \
                     starting at level {SWASHBUCKLER_CHARMED_LIFE_MIN_LEVEL}): a genuinely \
                     valid PF1 posture for a 1st-level Swashbuckler, not a gap"
                ),
            });
        }
        Some(uses_per_day) => {
            let activation = input
                .chosen
                .class_ability_activations
                .iter()
                .find(|activation| activation.ability_id == SWASHBUCKLER_CHARMED_LIFE_ABILITY_ID);
            match activation {
                None => {
                    explanations.push(ComputationExplanation {
                        id: "class_feature.acg.swashbuckler.charmed_life_not_active".to_owned(),
                        value: 0,
                        detail: format!(
                            "Swashbuckler level {level} is not currently using Charmed Life (no \
                             class_ability_activations entry for \
                             \"{SWASHBUCKLER_CHARMED_LIFE_ABILITY_ID}\"): a genuinely valid PF1 \
                             posture -- not every save is one where a Swashbuckler chooses to \
                             spend a use -- so no bonus is claimed. Charmed Life's own uses-per-\
                             day budget ({uses_per_day}) is grounded regardless"
                        ),
                    });
                }
                Some(activation) => {
                    if let Some(uses_consumed_today) = activation.rounds_consumed_today
                        && i32::from(uses_consumed_today) > i32::from(uses_per_day)
                    {
                        diagnostics.push(ComputationDiagnostic {
                            id: "class_feature.acg.swashbuckler.charmed_life_uses_exceeded"
                                .to_owned(),
                            message: format!(
                                "Swashbuckler level {level} Charmed Life activation claims \
                                 {uses_consumed_today} uses consumed today, exceeding the \
                                 grounded uses-per-day budget of {uses_per_day} \
                                 (((level-2)/4)+3): a genuine posture violation, so no save \
                                 bonus is claimed for this input"
                            ),
                            claim_blocking: true,
                        });
                    } else if activation.active_state == ActiveState::EquippedActive {
                        explanations.push(ComputationExplanation {
                            id: "class_feature.acg.swashbuckler.charmed_life_active_bonus"
                                .to_owned(),
                            value: ability_modifiers.charisma,
                            detail: format!(
                                "Swashbuckler level {level} is actively spending a use of \
                                 Charmed Life as an immediate action before a saving throw, \
                                 adding her Charisma modifier ({}) to the result. This codebase \
                                 computes no per-roll save resolution to integrate this into \
                                 (unlike the flat base Fortitude/Reflex/Will totals, which have \
                                 no 'immediate action before this specific roll' concept) -- \
                                 grounded as a standalone record naming the magnitude only",
                                ability_modifiers.charisma
                            ),
                        });
                    } else {
                        explanations.push(ComputationExplanation {
                            id: "class_feature.acg.swashbuckler.charmed_life_not_active"
                                .to_owned(),
                            value: 0,
                            detail: format!(
                                "Swashbuckler level {level} has a Charmed Life activation entry \
                                 but it is not active for this snapshot: a genuinely valid PF1 \
                                 posture, so no bonus is claimed. Charmed Life's own uses-per-\
                                 day budget ({uses_per_day}) is grounded regardless"
                            ),
                        });
                    }
                }
            }
        }
    }

    push_swashbuckler_other_features_deferred_diagnostic(diagnostics);
}

/// Pushes the new, narrower diagnostic replacing
/// `class_feature.acg.swashbuckler.unsupported` for Swashbuckler
/// specifically (v0.6 alpha swarm, risks item 8, Swashbuckler full-build
/// closure): named ONLY the genuinely still-missing pieces. Pushed
/// unconditionally regardless of Charmed Life's own active state,
/// mirroring Warpriest's/Slayer's own diagnostic-honesty pattern.
pub(super) fn push_swashbuckler_other_features_deferred_diagnostic(
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.swashbuckler.other_features_deferred.unsupported".to_owned(),
        message: format!(
            "{SWASHBUCKLER_CLASS_ID} now grounds every named feature on its corpus class table: \
             the base-attack-bonus/base-save chassis pillar, its class-skill list, Panache's flat \
             daily maximum, Nimble's dodge bonus, Charmed Life, Swashbuckler Finesse, \
             Swashbuckler Weapon Training, all NINETEEN Swashbuckler Deed records, and -- newly, \
             task #91 -- Swashbuckler Weapon Mastery's critical-multiplier increase and the \
             bonus-feat slot count (`Pool_SwashbucklerBonusFeat`, level/4). This diagnostic is \
             therefore no longer claim-blocking; it is retained to carry the honest remainder. \
             Of the thirteen deeds closed by task #91, eleven carry no numeric corpus token of \
             any kind -- verified field by field, not by a filtered grep -- and are grounded as \
             bounded grant-only identity records quoting their real rulebook text: Opportune \
             Parry and Riposte (1st), Kip-Up and Menacing Swordplay (3rd), Superior Feint, \
             Swashbuckler's Grace and Targeted Strike (7th), Evasive and Subtle Blade (11th), \
             Perfect Thrust and Swashbuckler's Edge (15th), and Cheat Death (19th). The other \
             two are genuinely numeric and are grounded with real values: Swashbuckler \
             Initiative's +2 (a TEMPBONUS, hence conditional on holding panache) and Dizzying \
             Defense's +4 dodge / -2 attack (DESC prose only). What stays deferred is EXECUTION, \
             not magnitude: no combat action, attack roll, critical multiplier, armour-class \
             total or initiative total exists here for these numbers to feed, and the eleven \
             zero-magnitude deeds have no number that could ever be computed. Those eleven are \
             NOT claimed to be text-complete-to-the-player -- no class-feature description \
             surface exists in the shipped app, so the stronger claim is deliberately not made. \
             The bonus-feat slot count grounds without seeding which combat feat fills each slot. \
             This message previously described Deeds as \"named but not built\" and Swashbuckler \
             Finesse as having \"no hook\" (both false since task #14); a later revision named \
             only four remaining items, understating the gap (task #76); the thirteen-deed list \
             it then carried is now closed"
        ),
        claim_blocking: false,
    });
}

/// Swashbuckler Weapon Training's attack/damage bonus with light and
/// one-handed piercing melee weapons: `(level-1)/4` -- +1 at 5th, +2 at
/// 9th, +3 at 13th, +4 at 17th, and genuinely 0 below 5th. Verified
/// directly against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|SwashbucklerWeaponTrainingBonus|(SwashbucklerWeaponTrainingLVL-1)/4`,
/// where `SwashbucklerWeaponTrainingLVL` is set from `SwashbucklerLVL`.
pub(super) fn swashbuckler_weapon_training_bonus(level: u8) -> i16 {
    (i16::from(level) - 1) / 4
}

/// The six Summon Nature's Ally spells (I-VI) Hunter automatically knows,
/// per `acg_classes.lst`'s own `CLASS:Hunter` record: `KNOWNSPELLS:Summon
/// Nature's Ally I|II|III|IV|V|VI`. Real PF1 rule text: "In addition, a
/// hunter adds the following spells to her list of spells known at the
/// indicated levels, as normal spells of that level" -- an unconditional,
/// automatic grant on top of the Hunter Spells Known table's own per-level
/// count, not a budgeted choice the way the rest of a Hunter's known
/// spells are. Every entry's level matches the union spell list's own
/// resolution for that name (`hunter_spell_list::hunter_spell_level`),
/// verified identical on both source lists, so this array is not a second,
/// independently-drifting source of truth for those six levels.
pub(super) const HUNTER_AUTOMATIC_KNOWN_SPELLS: [(&str, u8); 6] = [
    ("Summon Nature's Ally I", 1),
    ("Summon Nature's Ally II", 2),
    ("Summon Nature's Ally III", 3),
    ("Summon Nature's Ally IV", 4),
    ("Summon Nature's Ally V", 5),
    ("Summon Nature's Ally VI", 6),
];

/// The PF1 Advanced Class Guide Hunter Spells Known table's row, one entry
/// per spell level 0-6 (index 0 is orisons; `None` for an inaccessible
/// "--" column). A literal table lookup transcribed directly from
/// `acg_classes.lst`'s own `CLASS:Hunter` record's `KNOWN:` column of its
/// "Level progression" block (levels 1-20 all present -- Hunter casts from
/// 1st level, unlike Bloodrager's own level-4-gated posture). Mirrors
/// `oracle_spells_known_table`'s/`sorcerer_spells_known_table`'s own
/// shape: the cap on distinct spells KNOWN (permanent), not a per-day
/// consumable resource. This is the cap on FREELY CHOSEN known spells
/// only -- the six automatic Summon Nature's Ally grants above are on top
/// of these counts, not counted against them (see
/// `unmet_hunter_known_spell_conditions`'s own doc comment).
///
/// This bounded slice grounds the KNOWN column only, mirroring Oracle's
/// own narrower shape exactly (`ground_oracle_known_spells` grounds no
/// per-day slot totals either) -- the corpus's own CAST column (spells
/// per day) stays a separately named, still-ungrounded burden.
pub(super) fn hunter_spells_known_table(level: u8) -> [Option<i16>; 7] {
    match level {
        1 => [Some(4), Some(3), None, None, None, None, None],
        2 => [Some(5), Some(4), None, None, None, None, None],
        3 => [Some(6), Some(5), None, None, None, None, None],
        4 => [Some(6), Some(5), Some(3), None, None, None, None],
        5 => [Some(6), Some(5), Some(4), None, None, None, None],
        6 => [Some(6), Some(5), Some(5), None, None, None, None],
        7 => [Some(6), Some(6), Some(5), Some(3), None, None, None],
        8 => [Some(6), Some(6), Some(5), Some(4), None, None, None],
        9 => [Some(6), Some(6), Some(5), Some(5), None, None, None],
        10 => [Some(6), Some(6), Some(6), Some(5), Some(3), None, None],
        11 => [Some(6), Some(7), Some(6), Some(5), Some(4), None, None],
        12 => [Some(6), Some(7), Some(6), Some(5), Some(5), None, None],
        13 => [Some(6), Some(7), Some(6), Some(6), Some(5), Some(3), None],
        14 => [Some(6), Some(7), Some(7), Some(6), Some(5), Some(4), None],
        15 => [Some(6), Some(7), Some(7), Some(6), Some(5), Some(5), None],
        16 => [Some(6), Some(7), Some(7), Some(6), Some(6), Some(5), Some(3)],
        17 => [Some(6), Some(7), Some(7), Some(7), Some(6), Some(5), Some(4)],
        18 => [Some(6), Some(7), Some(7), Some(7), Some(6), Some(5), Some(5)],
        19 => [Some(6), Some(7), Some(7), Some(7), Some(6), Some(6), Some(5)],
        20 => [Some(6), Some(7), Some(7), Some(7), Some(7), Some(6), Some(6)],
        _ => [None, None, None, None, None, None, None],
    }
}

/// Hunter's base spells-per-day table -- the corpus's own `CAST:`
/// column, transcribed level by level from `acg_classes.lst:144-163`.
///
/// Indexed by spell level 1-6; index 0 is spell level 1. Hunter's
/// maximum spell level is 6, so this is a `[Option<i16>; 6]` where
/// Skald's equivalent is `[_; 4]`.
///
/// **The leading `0` in every `CAST:` row is deliberately dropped, not
/// stored.** That column is spell level 0 -- Orisons -- and PCGen writes
/// `0` there to mean *unlimited*, not *none*. Storing it as a per-day
/// count of zero would report a Hunter who can cast no orisons at all,
/// which is the same trap Skald's Cantrips column carries.
///
/// Distinct from `hunter_spells_known_table` (the `KNOWN:` column):
/// spells KNOWN is a permanent cap on distinct spells, spells PER DAY is
/// the consumable slot budget. The corpus's own comment above the table
/// warns that the KNOWN column is deliberately inflated by 1 to absorb
/// the automatic Summon Nature's Ally grants; the CAST column carries no
/// such adjustment, so the two must not be derived from one another.
pub(super) fn hunter_base_spells_per_day_table(level: u8) -> [Option<i16>; 6] {
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
        20 => [Some(5), Some(5), Some(5), Some(5), Some(5), Some(5)],
        _ => [None, None, None, None, None, None],
    }
}

/// Hunter's highest castable spell level at `level`, derived from the
/// same `CAST:` rows: the count of populated spell-level columns.
///
/// Derived from the table rather than written as a second literal ladder
/// so the two can never disagree -- the access ceiling IS "how many
/// columns have a slot", by construction.
pub(super) fn hunter_spell_level_access(level: u8) -> i16 {
    hunter_base_spells_per_day_table(level)
        .iter()
        .filter(|slots| slots.is_some())
        .count() as i16
}

/// Nature Training's grant level, per `acg_classes.lst:125`.
pub(super) const HUNTER_NATURE_TRAINING_LEVEL: u8 = 1;

/// Grounds Hunter's spellcasting chassis -- spell-level access, the
/// per-day slot budget, and the spell save DC -- plus Nature Training
/// (task #91).
///
/// These were the substance of Hunter's claim-blocking diagnostic. The
/// shape deliberately mirrors `ground_or_block_skald_spellcasting`'s
/// flat-records half: all three are real regardless of whether the
/// known-spell posture validates, because a Hunter with an invalid
/// known-spell list still has a real access ladder, a real slot budget
/// and a real DC formula. The known-spell posture keeps its own separate
/// claim-blocking diagnostic, which is untouched here.
///
/// **Hunter's SPELLSTAT is WISDOM** (`acg_classes.lst:114`), unlike
/// Skald's Charisma -- and Hunter is a `FACT:SpellType|Divine` caster
/// where Skald is Arcane. Copying Skald's Charisma term would produce a
/// wrong DC for every Hunter.
pub(super) fn ground_hunter_spellcasting_chassis_and_nature_training(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let access_ceiling = hunter_spell_level_access(level);
    explanations.push(ComputationExplanation {
        id: "class_chassis.hunter.spontaneous.spell_level_access".to_owned(),
        value: access_ceiling,
        detail: format!(
            "Hunter spell-level access at hunter level {level}: {access_ceiling}. Derived from \
             the count of populated columns in the corpus's own CAST: row rather than written as \
             a second literal ladder, so the ceiling and the slot table can never disagree. \
             Hunter's maximum spell level is 6, not 4 or 9. This grounds the access ladder only: \
             no slot consumption, no casting execution, and no spells-known posture is decided \
             by this record"
        ),
    });

    let base_spells_per_day = hunter_base_spells_per_day_table(level);
    for (index, base_count) in base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = index + 1;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.hunter.spontaneous.base_spells_per_day.spell_level_{spell_level}"
            ),
            value: *base_count,
            detail: format!(
                "Hunter base spells per day at hunter level {level}, spell level {spell_level}: \
                 {base_count}, transcribed from the corpus's own CAST: column \
                 (acg_classes.lst:144-163). This is the consumable slot budget, a different \
                 table from the spells-KNOWN cap already grounded separately -- the corpus's own \
                 comment warns that the KNOWN column is inflated by 1 to absorb the automatic \
                 Summon Nature's Ally grants while CAST carries no such adjustment, so the two \
                 must never be derived from each other. Grounds the BASE count only: bonus slots \
                 from a high Wisdom are not computed, and the leading CAST: 0 for orisons means \
                 UNLIMITED rather than none, so no zero-slot orison record is emitted"
            ),
        });
    }

    let wisdom_modifier = ability_modifier_for(ability_modifiers, "wisdom");
    for spell_level in 1..=access_ceiling {
        let spell_save_dc = 10 + spell_level + wisdom_modifier;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.hunter.spontaneous.spell_save_dc.spell_level_{spell_level}"
            ),
            value: spell_save_dc,
            detail: format!(
                "Hunter spell save DC at hunter level {level}, spell level {spell_level}: \
                 10 + {spell_level} + Wisdom modifier {wisdom_modifier} = {spell_save_dc}. The \
                 stat is WISDOM, per the class record's own SPELLSTAT:WIS -- Hunter is a divine \
                 caster (FACT:SpellType|Divine), and reusing the Charisma term from Skald's \
                 arcane equivalent would give every Hunter a wrong DC. This grounds the base DC \
                 formula only: no saving-throw resolution, no target, and no feat DC modifiers \
                 are computed"
            ),
        });
    }

    if level >= HUNTER_NATURE_TRAINING_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.hunter.nature_training_grant".to_owned(),
            value: 0,
            detail: format!(
                "Hunter level {level} Nature Training, granted at level \
                 {HUNTER_NATURE_TRAINING_LEVEL} (corpus KEY:Hunter ~ Nature Training): the \
                 hunter counts as a druid or ranger for feats, traits, and options that modify \
                 or improve an animal companion. This is a bounded grant-only identity record \
                 (value 0, non-fabricated): the record carries no BONUS, no DEFINE and no ADD -- \
                 it is a pure qualification flag with no magnitude to compute, now or ever. \
                 Grounds the qualification fact itself; this codebase resolves no \
                 druid-or-ranger-gated companion feat or trait for it to unlock, and the corpus \
                 author's own DESC carries a literal \"[Not implemented]\" marker recording that \
                 PCGen does not resolve it either"
            ),
        });
    }
}

/// Return the list of unmet conditions for Hunter's real known-spell
/// posture, mirroring `unmet_oracle_known_spell_conditions`'s own shape
/// exactly, substituting Hunter's own 1-20 table and
/// `hunter_spell_list::hunter_spell_level` (the union of the Druid and
/// Ranger general lists, take-the-lower on a level conflict, already
/// bounded to Hunter's own 6th-level ceiling) for the per-spell-id level
/// lookup. An empty list means the posture is fully valid; zero known
/// spells is always valid, same reasoning as Sorcerer's/Oracle's own
/// posture -- this check only inspects `AcquisitionMode::Known` freely
/// chosen selections. The six automatic Summon Nature's Ally grants are
/// never subject to this cap check at all: they are unconditional on
/// class ownership and level alone (see `ground_hunter_known_spells`),
/// exactly mirroring how Wild Empathy and Animal Focus are ungated by
/// `spells_selected`.
pub(super) fn unmet_hunter_known_spell_conditions(input: &CharacterInput, hunter_level: u8) -> Vec<String> {
    let mut unmet = Vec::new();

    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| s.source_class_id == HUNTER_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();

    let known_table = hunter_spells_known_table(hunter_level);

    let mut known_per_level: [i16; 7] = [0; 7];
    for spell_id in &known {
        let Some(spell_level) = hunter_spell_list::hunter_spell_level(spell_id) else {
            unmet.push(format!(
                "known spell '{spell_id}' is not on the real PF1 Hunter spell list (the union of \
                 the Druid and Ranger general lists, bounded to Hunter's own 6th-level ceiling)"
            ));
            continue;
        };
        if usize::from(spell_level) >= known_table.len() {
            unmet.push(format!(
                "known spell '{spell_id}' targets spell level {spell_level}, not yet accessible \
                 at hunter level {hunter_level}"
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
                 slots available on the Hunter Spells Known table"
            ));
        }
    }

    unmet
}

/// Ground the real known-spell posture once
/// `unmet_hunter_known_spell_conditions` reports an empty unmet list,
/// mirroring `ground_oracle_known_spells`'s own shape exactly, plus a
/// second, unconditional record for the six automatic Summon Nature's
/// Ally spells (v0.6 alpha swarm, task #44).
pub(super) fn ground_hunter_known_spells(
    input: &CharacterInput,
    hunter_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| s.source_class_id == HUNTER_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.acg.hunter.known_spells".to_owned(),
        value: known.len() as i16,
        detail: format!(
            "Hunter level {hunter_level} freely chosen known-spell selection ({} spells, \
             AcquisitionMode::Known): {}. Each known spell is verified against the real PF1 \
             Hunter spell list (`hunter_spell_list::hunter_spell_level`, the union of the Druid \
             and Ranger general lists, take-the-lower on a level conflict, bounded to Hunter's \
             own 6th-level ceiling) and the Hunter Spells Known table's own per-level cap for \
             levels 1-20. Real PF1 Hunter rules have no daily preparation step at all (Hunter is \
             a spontaneous, not prepared, caster) -- a hunter's known spells are permanent once \
             learned, cast spontaneously. This grounds the freely chosen known-spell selection \
             for real; it computes no spell save DC resolution against a target and no casting \
             execution",
            known.len(),
            known.join(", ")
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_spell.acg.hunter.automatic_summon_natures_ally_known_spells".to_owned(),
        value: HUNTER_AUTOMATIC_KNOWN_SPELLS.len() as i16,
        detail: format!(
            "Hunter level {hunter_level} automatically knows all six Summon Nature's Ally \
             spells (I-VI, spell levels 1-6) per `acg_classes.lst`'s own \
             `KNOWNSPELLS:Summon Nature's Ally I|II|III|IV|V|VI` token: {}. This grant is \
             unconditional on class ownership alone (no `spells_selected` entry is required, \
             the same \"always on\" shape as Wild Empathy) and adds to the Hunter Spells Known \
             table's own per-level counts above rather than consuming a slot from them -- real \
             PF1 rule text: \"a hunter adds the following spells to her list of spells known ... \
             as normal spells of that level\"",
            HUNTER_AUTOMATIC_KNOWN_SPELLS
                .iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>()
                .join(", ")
        ),
    });
}

/// Top-level dispatch for Hunter's known-spell posture (v0.6 alpha swarm,
/// task #44): grounds the real posture once
/// `unmet_hunter_known_spell_conditions` reports an empty unmet list,
/// otherwise pushes the narrower `class_spell.acg.hunter.known_spells
/// .unsupported` claim-blocking diagnostic, mirroring
/// `ground_or_block_oracle_class_features`'s own known-spell dispatch
/// shape exactly.
pub(super) fn ground_or_block_hunter_known_spells(
    input: &CharacterInput,
    hunter_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let known_spell_unmet = unmet_hunter_known_spell_conditions(input, hunter_level);
    if known_spell_unmet.is_empty() {
        ground_hunter_known_spells(input, hunter_level, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.acg.hunter.known_spells.unsupported".to_owned(),
            message: format!(
                "Hunter remains blocked on its known-spell posture burden: {}",
                known_spell_unmet.join("; ")
            ),
            claim_blocking: true,
        });
    }
}

/// Grounds Hunter's 1st-level Animal Companion (v0.6 alpha swarm, risks
/// item 8, fourth APG/ACG closure) by reusing the exact Wolf stat-block
/// math Druid's own closure already verified and shipped -- the corpus
/// text confirms this is the same mechanic ("the hunter's effective
/// druid level is equal to her hunter level"), not merely a similar one.
/// Called from `compute_acg_class_chassis`'s Hunter branch, unconditional
/// on Hunter class ownership and level alone (no `selected_choices` or
/// `class_ability_activations` entry is required -- see `HUNTER_CLASS_ID`'s
/// own doc comment for why this is unconditional, unlike Druid's own
/// Nature-Bond-choice-gated version).
///
/// **Correction (v0.6 alpha swarm, task #44):** the known-spell posture
/// named below as still-missing is now real. `SPELLLIST:2|Druid|Ranger`
/// on Hunter's own `CLASS:Hunter` record means Hunter's castable list is
/// the UNION of the Druid and Ranger general lists (not restricted to
/// Summon Nature's Ally spells the way an earlier version of this comment
/// claimed -- `KNOWNSPELLS:Summon Nature's Ally I|II|III|IV|V|VI` only
/// grants those six spells automatically, on top of the union list, it
/// does not narrow the list itself). See `hunter_spell_list.rs` and
/// `ground_or_block_hunter_known_spells` for the real grounding: the
/// union spell list, the take-the-lower-level ruling for the 27 spells
/// whose level conflicts between the two source lists, the Hunter Spells
/// Known table (levels 1-20), and the six automatic Summon Nature's Ally
/// grants. Hunter's own remaining named features (Animal Focus, Nature
/// Training, Precise Companion) and its per-day CAST-table slot totals,
/// spell save DCs, and casting execution stay ungrounded -- stays claim-
/// blocked via the
/// `class_feature.acg.hunter.other_features_deferred.unsupported`
/// diagnostic (renamed from `spellcasting_deferred` now that the
/// known-spell posture is genuinely validated), replacing the generic
/// `class_feature.acg.hunter.unsupported` diagnostic for Hunter
/// specifically, mirroring Skald's and Bloodrager's own
/// diagnostic-honesty fix.
/// Hunter Wild Empathy's flat check-modifier magnitude: Hunter level +
/// Charisma modifier (deepening 2026-07-26, task #2), verified directly
/// against `acg_abilities_class.lst`'s own `CHA+HunterLVL` formula.
pub(super) fn hunter_wild_empathy_bonus(level: u8, charisma_modifier: i16) -> i16 {
    charisma_modifier + i16::from(level)
}

/// Hunter Animal Focus's genuinely enforced per-day minutes budget:
/// equal to Hunter level (deepening 2026-07-26, task #2), verified
/// directly against `acg_abilities_class.lst`'s own
/// `BONUS:VAR|HunterAnimalFocusMinutes|HunterLVL`.
///
/// The `Extended Animal Focus` feat contributes `max(1, WIS)` to the
/// SAME variable (`acg_feats.lst`:
/// `BONUS:VAR|HunterAnimalFocusMinutes|max(1,WIS)`, prose "Add your
/// Wisdom modifier [minimum 1] to the number of minutes per day that
/// you can use your animal focus ability" -- token and prose agree
/// including the floor). Folded into the formula rather than applied at
/// a display site because this budget's only consumer is the enforced
/// over-budget claim-block: widening it anywhere else would show a
/// number the enforcement did not honour.
///
/// Presence-based, not counted: the record carries no `STACK:`, no
/// `MULT:`, and its BENEFIT has no repeat clause.
pub(super) fn hunter_animal_focus_uses_per_day(
    level: u8,
    wisdom_modifier: i16,
    selected_feats: &[String],
) -> i16 {
    i16::from(level)
        + non_stacking_resource_feat_bonus(
            selected_feats,
            EXTENDED_ANIMAL_FOCUS_FEAT_KEY,
            wisdom_modifier.max(1),
        )
}

/// The shared tiered-magnitude shape every `HUNTER_ANIMAL_FOCUS_TIERED_OPTIONS` row uses: a base
/// value, plus its own `per_level_8` increment once `level >= 8`, plus its own `per_level_15`
/// increment once `level >= 15` -- the exact three-`BONUS:VAR` stacking shape each option's own
/// corpus record carries (`HunterAnimalFocusLVL` itself always `HunterLVL` unconditionally, so
/// this module's own `level` parameter, already the Hunter class level, is exactly that value).
pub(super) fn hunter_animal_focus_tiered_bonus(option: &HunterAnimalFocusTieredOption, level: u8) -> i16 {
    let mut bonus = option.base;
    if level >= 8 {
        bonus += option.per_level_8;
    }
    if level >= 15 {
        bonus += option.per_level_15;
    }
    bonus
}

/// Grounds Hunter's Wild Empathy as a standalone explanation record
/// (deepening 2026-07-26, task #2, correcting an earlier over-strict
/// "needs a live consumer" exclusion -- see Inquisitor's own task #18 and
/// Skald's own task #7 for the same correction applied first): no "wild
/// empathy check" total exists anywhere in this codebase, so this
/// grounds only the flat bonus value. Unconditional on class ownership
/// and level alone; never claim-blocks.
pub(super) fn ground_hunter_wild_empathy(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let wild_empathy_bonus = hunter_wild_empathy_bonus(level, ability_modifiers.charisma);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.hunter.wild_empathy_bonus".to_owned(),
        value: wild_empathy_bonus,
        detail: format!(
            "Hunter level {level} Wild Empathy: a +{wild_empathy_bonus} bonus (Hunter level \
             {level} + Charisma modifier {:+}) added to a 1d20 roll to improve an animal's \
             attitude. No wild-empathy-check total exists anywhere in this codebase, so this \
             grounds only the flat bonus value",
            ability_modifiers.charisma
        ),
    });
}

/// Grounds or claim-blocks Hunter's Animal Focus execution engine for
/// `level` (deepening 2026-07-26, task #2; widened SD-32 T12 Epic 8 row
/// 18 cycle 17 from Bull-only to all 13 real corpus options -- a generic
/// pass over `HUNTER_ANIMAL_FOCUS_TIERED_OPTIONS` plus Mouse's and No
/// Ability's own small branches, per `§17`, not 13 near-duplicate
/// functions). Mirrors `ground_or_block_inquisitor_judgment`'s exact
/// three-branch shape (activation-gating + choice-recognition): a
/// character not currently focused (no `class_ability_activations` entry,
/// or one present but not `EquippedActive`) is a genuinely valid PF1
/// posture -- grounds a real "not focused" recognition record, no
/// claim-block. An active focus naming no recognized
/// `choice:hunter_animal_focus` selection is a genuine posture violation
/// and claim-blocks. An active focus naming any of the 13 real options
/// grounds its own real magnitude (or boolean/text-only posture fact for
/// Mouse/No Ability). The per-day minutes budget is genuinely enforced
/// against `activation.rounds_consumed_today`, mirroring Rage's/
/// Judgment's own enforced budget exactly, unchanged from before this
/// cycle.
pub(super) fn ground_or_block_hunter_animal_focus(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(activation) = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == HUNTER_ANIMAL_FOCUS_ABILITY_ID)
    else {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.hunter.animal_focus_execution.not_focused".to_owned(),
            value: 0,
            detail: format!(
                "Hunter level {level} is not currently using Animal Focus (no \
                 class_ability_activations entry for \
                 \"{HUNTER_ANIMAL_FOCUS_ABILITY_ID}\"): a genuinely valid PF1 posture, so no \
                 focus bonus is claimed"
            ),
        });
        return;
    };

    let uses_per_day = hunter_animal_focus_uses_per_day(
        level,
        ability_modifier(input.chosen.ability_scores.wisdom),
        &input.chosen.selected_feats,
    );
    if let Some(minutes_consumed_today) = activation.rounds_consumed_today
        && i32::from(minutes_consumed_today) > i32::from(uses_per_day) {
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.acg.hunter.animal_focus_execution.uses_exceeded".to_owned(),
                message: format!(
                    "Hunter level {level} Animal Focus activation claims \
                     {minutes_consumed_today} minutes consumed today, exceeding the grounded \
                     per-day budget of {uses_per_day} minutes (Hunter level, plus max(1, Wisdom \
                     modifier) if the Extended Animal Focus feat is held): a genuine posture \
                     violation, so no focus bonus is claimed for this input"
                ),
                claim_blocking: true,
            });
            return;
        }

    match activation.active_state {
        ActiveState::EquippedActive => {
            let focus_selection = choice_selection(input, HUNTER_ANIMAL_FOCUS_CHOICE_ID);
            let minutes_consumed_today = activation.rounds_consumed_today.unwrap_or(0);

            let recognized_tiered =
                focus_selection.and_then(|s| HUNTER_ANIMAL_FOCUS_TIERED_OPTIONS.iter().find(|o| o.selection_id == s));

            if let Some(option) = recognized_tiered {
                let bonus = hunter_animal_focus_tiered_bonus(option, level);
                explanations.push(ComputationExplanation {
                    id: "class_feature.acg.hunter.animal_focus_execution.active".to_owned(),
                    value: bonus,
                    detail: format!(
                        "Hunter level {level} is actively using the {} Animal Focus, within the \
                         grounded per-day budget ({uses_per_day} minutes; {minutes_consumed_today} \
                         consumed today), granting +{bonus} {}. This is a standalone fact -- it is \
                         not applied to any integrated ability modifier, skill total, speed total, \
                         or vision-range total",
                        option.label, option.benefit
                    ),
                });
                if option.selection_id == "animal_focus:bat" && level >= 15 {
                    explanations.push(ComputationExplanation {
                        id: "class_feature.acg.hunter.animal_focus_execution.bat_blindsense"
                            .to_owned(),
                        value: 10,
                        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                        //   DESC: \"and blindsense to 10 feet|PREVARGTEQ:HunterAnimalFocusLVL,15\"
                        detail: "Hunter level 15+ Bat Animal Focus also grants blindsense to a range \
                                  of 10 feet -- a separate boolean fact from the darkvision range \
                                  magnitude above (real corpus), not folded into it"
                            .to_owned(),
                    });
                }
            } else if focus_selection == Some(HUNTER_ANIMAL_FOCUS_MOUSE_SELECTION_ID) {
                let improved = level >= 12;
                explanations.push(ComputationExplanation {
                    id: "class_feature.acg.hunter.animal_focus_execution.active".to_owned(),
                    value: i16::from(improved),
                    detail: format!(
                        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                        //   DESC: \"Get evasion ... and improved
                        //   evasion|PREVARGTEQ:HunterAnimalFocusLVL,12\"
                        "Hunter level {level} is actively using the Mouse Animal Focus, within the \
                         grounded per-day budget ({uses_per_day} minutes; {minutes_consumed_today} \
                         consumed today), granting evasion{}. A boolean posture fact (real corpus) \
                         -- no magnitude exists for this option at all, and this \
                         codebase has no integrated evasion/improved-evasion total to apply it to",
                        if improved { " and improved evasion (level 12+)" } else { "" }
                    ),
                });
            } else if focus_selection == Some(HUNTER_ANIMAL_FOCUS_NO_ABILITY_SELECTION_ID) {
                explanations.push(ComputationExplanation {
                    id: "class_feature.acg.hunter.animal_focus_execution.active".to_owned(),
                    value: 0,
                    detail: format!(
                        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                        //   DESC: \"Ability not being used\"
                        "Hunter level {level} is actively using Animal Focus but has chosen \"No \
                         Ability\" (real corpus), within the grounded per-day budget ({uses_per_day} \
                         minutes; {minutes_consumed_today} consumed today): a genuinely valid, \
                         text-only posture carrying no magnitude at all -- no focus bonus is \
                         claimed"
                    ),
                });
            } else {
                diagnostics.push(ComputationDiagnostic {
                    id: "class_feature.acg.hunter.animal_focus_execution.focus_choice_missing"
                        .to_owned(),
                    message: format!(
                        "Hunter level {level} claims an active Animal Focus \
                         ({HUNTER_ANIMAL_FOCUS_ABILITY_ID}) but has no recognized \
                         {HUNTER_ANIMAL_FOCUS_CHOICE_ID} selection naming one of the 13 real \
                         corpus options (got {focus_selection:?}): taking on an animal focus \
                         always requires choosing a type first per the corpus's own sequencing, \
                         so an active focus naming an unrecognized type -- or none -- is a \
                         genuine posture violation, not a silently passing one -- no focus \
                         bonus is claimed for this input"
                    ),
                    claim_blocking: true,
                });
                return;
            }

            explanations.push(ComputationExplanation {
                id: "class_feature.acg.hunter.animal_focus_execution.uses_per_day".to_owned(),
                value: uses_per_day,
                detail: format!(
                    "Hunter level {level} Animal Focus minutes per day: equal to Hunter level = \
                     {uses_per_day}. Genuinely enforced -- an activation whose \
                     rounds_consumed_today exceeds this budget claim-blocks (see the \
                     uses_exceeded check above), mirroring Judgment's/Rage's own enforced budget \
                     exactly"
                ),
            });
        }
        ActiveState::SelectedInactive | ActiveState::Absent => {
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.hunter.animal_focus_execution.not_focused".to_owned(),
                value: 0,
                detail: format!(
                    "Hunter level {level} has a \"{HUNTER_ANIMAL_FOCUS_ABILITY_ID}\" activation \
                     entry but it is not active for this snapshot: a genuinely valid PF1 \
                     posture, so no focus bonus is claimed"
                ),
            });
        }
    }
}

/// Grounds Hunter's remaining flat-magnitude class features (task #85),
/// each verified against its own `KEY:Hunter ~ ...` corpus record.
///
/// **Second Animal Focus and Master Hunter share ONE record.** Both carry
/// the identical token `BONUS:ABILITYPOOL|Hunter Animal Focus|1`, i.e.
/// each is a +1 increment to the same simultaneous-focus pool rather than
/// a distinct magnitude -- the same "two features, one shared counter"
/// shape as Bloodrager's Uncanny Dodge / Improved Uncanny Dodge tier.
/// Modelling them as two separate records would imply two independent
/// mechanics where the corpus has one.
///
/// This counter is deliberately NOT the same thing as
/// `animal_focus_execution.uses_per_day`, which is the MINUTES-per-day
/// budget (equal to Hunter level). These increment how many different
/// animal aspects may be active at once; that one bounds how long. Same
/// feature family, unrelated quantities.
#[cfg(test)]
mod hunter_remaining_features_tests {
    use super::ground_hunter_remaining_features;

    /// Exercises the pure grounding function directly rather than through
    /// `build_pilot_headless_receipt`.
    ///
    /// **Deliberate, and worth the explanation:** these are pure
    /// `level -> records` magnitudes, so calling
    /// `ground_hunter_remaining_features` directly tests exactly what this
    /// slice grounds and nothing else, without dragging the whole receipt
    /// pipeline into a class-feature magnitude assertion.
    ///
    /// Historical note: this indirection was originally forced, not
    /// merely preferred. Hunter's animal-companion slice used to carry a
    /// `debug_assert_eq!(companion_level, 1, "only companion level 1 is
    /// grounded this slice")`, which made every above-level-1 Hunter
    /// receipt panic under `cfg(test)`. That bound is gone -- the
    /// companion's Hit Dice now come from the real corpus progression at
    /// every master level 1-20 -- so the full-receipt path would work
    /// here now. The direct call is kept because it is still the sharper
    /// test.
    fn value(level: u8, id: &str) -> Option<i16> {
        let mut explanations = Vec::new();
        ground_hunter_remaining_features(level, &mut explanations);
        explanations.iter().find(|e| e.id == id).map(|e| e.value)
    }

    /// Each feature lands exactly at its own corpus gate, and its absence
    /// below that gate is itself grounded rather than omitted (task #85).
    #[test]
    fn each_feature_appears_exactly_at_its_corpus_gate() {
        for (id, gate, at_gate) in [
            ("class_feature.acg.hunter.track_survival_bonus", 2u8, 1i16),
            ("class_feature.acg.hunter.precise_companion_bonus_feat_count", 2, 1),
            ("class_feature.acg.hunter.bonus_tricks", 7, 1),
        ] {
            assert_eq!(value(gate - 1, id), Some(0), "{id} absence must be grounded");
            assert_eq!(value(gate, id), Some(at_gate), "{id} at its own gate");
        }
    }

    /// Track is half level, rounded down, and keeps tracking level.
    #[test]
    fn track_is_half_level_rounded_down() {
        let id = "class_feature.acg.hunter.track_survival_bonus";
        assert_eq!(value(2, id), Some(1));
        assert_eq!(value(3, id), Some(1), "3/2 truncates to 1");
        assert_eq!(value(20, id), Some(10));
    }

    /// `floor((HunterLVL-1)/6)` -- first trick at 7th, then 13th and 19th,
    /// exactly matching the record's own DESC.
    #[test]
    fn bonus_tricks_step_at_seven_thirteen_and_nineteen() {
        let id = "class_feature.acg.hunter.bonus_tricks";
        assert_eq!(value(6, id), Some(0));
        assert_eq!(value(7, id), Some(1));
        assert_eq!(value(12, id), Some(1));
        assert_eq!(value(13, id), Some(2));
        assert_eq!(value(19, id), Some(3));
        assert_eq!(value(20, id), Some(3), "no fourth trick before 25th");
    }

    /// Second Animal Focus (8th) and Master Hunter (20th) are two corpus
    /// features incrementing ONE shared pool by the identical token, so
    /// they share a single counter rather than earning separate records.
    #[test]
    fn the_simultaneous_focus_counter_steps_twice() {
        let id = "class_feature.acg.hunter.simultaneous_animal_focus_count";
        assert_eq!(value(1, id), Some(1), "one aspect at a time by default");
        assert_eq!(value(7, id), Some(1));
        assert_eq!(value(8, id), Some(2), "Second Animal Focus at 8th");
        assert_eq!(value(19, id), Some(2));
        assert_eq!(value(20, id), Some(3), "Master Hunter at 20th");
    }

    /// The simultaneous-aspect counter must not be confused with the
    /// minutes-per-day budget: they are different quantities that happen
    /// to belong to the same feature family.
    #[test]
    fn the_focus_counter_is_not_the_minutes_per_day_budget() {
        // The minutes-per-day budget is grounded elsewhere (equal to
        // Hunter level, so 20 at level 20). This counter is 3. Asserting
        // the value directly keeps the distinction explicit without
        // pulling the activation path into a pure-function test.
        assert_eq!(
            value(20, "class_feature.acg.hunter.simultaneous_animal_focus_count"),
            Some(3),
            "3 aspects at once is a different quantity from the 20 minutes/day budget"
        );
    }
}

pub(super) const HUNTER_TRACK_LEVEL: u8 = 2;

pub(super) const HUNTER_PRECISE_COMPANION_LEVEL: u8 = 2;

pub(super) const HUNTER_BONUS_TRICKS_LEVEL: u8 = 7;

pub(super) const HUNTER_SECOND_ANIMAL_FOCUS_LEVEL: u8 = 8;

pub(super) const HUNTER_MASTER_HUNTER_LEVEL: u8 = 20;

pub(super) fn ground_hunter_remaining_features(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    let gated = |gate: u8, granted: i16| if level >= gate { granted } else { 0 };

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.hunter.track_survival_bonus".to_owned(),
        value: gated(HUNTER_TRACK_LEVEL, i16::from(level) / 2),
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:SKILL|Survival|HunterLVL/2`, a 2nd-level class feature
            "Hunter Track at hunter level {level}: adds half her level ({}) to Survival checks made \
             to FOLLOW TRACKS specifically. Grounds the magnitude only -- this codebase computes no \
             follow-tracks-specific Survival check, so the bonus is named and not applied to the \
             general Survival total, the same treatment Slayer's and Inquisitor's own Track records \
             already use",
            i16::from(level) / 2
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.hunter.bonus_tricks".to_owned(),
        value: gated(HUNTER_BONUS_TRICKS_LEVEL, (i16::from(level) - 1) / 6),
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:VAR|HunterBonusTricks|floor((HunterLVL-1)/6)`
            "Hunter Bonus Tricks at hunter level {level}: {} bonus trick(s) known by her animal \
             companion, from the corpus's own count of one per six hunter levels after the \
             first. The formula and the printed description agree exactly -- first trick at \
             7th, then 13th and 19th. Grounds the COUNT; which tricks were chosen is a chooser \
             this codebase does not model, and the companion's own trick list is not built",
            (i16::from(level) - 1) / 6
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.hunter.precise_companion_bonus_feat_count".to_owned(),
        value: gated(HUNTER_PRECISE_COMPANION_LEVEL, 1),
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `BONUS:ABILITYPOOL|Hunter Precise Companion Feat|1`, a 2nd-level class feature
            "Hunter Precise Companion at hunter level {level}: one bonus feat, chosen as either \
             Precise Shot or Outflank, with the prerequisites explicitly waived. Grounds the SLOT \
             COUNT only: which of the two was taken is a real chooser this codebase does not model, \
             and neither feat's own mechanics are built. If Outflank is chosen the corpus also \
             grants it to the animal companion -- ally-scoped, and likewise not modelled"
        ),
    });

    let simultaneous_foci = 1
        + i16::from(level >= HUNTER_SECOND_ANIMAL_FOCUS_LEVEL)
        + i16::from(level >= HUNTER_MASTER_HUNTER_LEVEL);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.hunter.simultaneous_animal_focus_count".to_owned(),
        value: simultaneous_foci,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   One counter, TWO corpus features incrementing it by the identical token
            //   `BONUS:ABILITYPOOL|Hunter Animal Focus|1` -- Second Animal Focus at 8th and Master
            //   Hunter at 20th -- so this is one shared pool rather than two independent mechanics,
            //   the same shape as Bloodrager's Uncanny Dodge/Improved Uncanny Dodge tier counter.
            "Hunter simultaneous Animal Focus aspects at hunter level {level}: {simultaneous_foci}. \
             Distinct from `animal_focus_execution.uses_per_day`, which is the minutes-per-day \
             budget: this bounds how MANY aspects may be active at once, that one bounds how LONG. \
             Which aspects are chosen stays the existing activation-gated chooser's business; Master \
             Hunter's own always-move-at-full-speed-while-tracking rider is not modelled (no \
             movement-during-Survival engine exists)"
        ),
    });
}

pub(super) fn ground_hunter_animal_companion_and_defer_the_rest(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    ground_selected_companion_or_default(
        input,
        "class_chassis.hunter.animal_companion",
        "Hunter",
        level,
        ground_wolf_companion_stat_block,
        explanations,
    );
    ground_wolf_companion_link_and_share_spells_vacuous(
        "class_feature.hunter.animal_companion",
        "hunter",
        explanations,
    );
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.hunter.animal_companion.advancement_absent".to_owned(),
        message: "Hunter animal companion advancement is grounded for every column that has a \
                   consumer in this engine -- Hit Dice across all twenty master levels (2 HD at \
                   1st through 16 HD at 20th; the hunter's effective druid level is equal to \
                   her hunter level, acg_abilities_class.lst:1171, so she reads the Core \
                   Rulebook companion progression unshifted), and with them base attack bonus, \
                   all three base saves and hit points, plus the natural-armor and Strength \
                   advances the armor-class and attack/damage records consume. Deliberately NOT \
                   grounded, because nothing in this codebase consumes them: the Dexterity half \
                   of the stat advance, bonus tricks, the companion's skill ranks and feats, \
                   the player-chosen Companion Stat Increase at master levels 4/9/14/20, the \
                   optional species size advance offered from master level 7 for a Wolf, the \
                   named abilities Evasion (3rd), Devotion (6th), Multiattack (9th), Spell \
                   Resistance (15th) and Improved Evasion (16th), and the hunter-specific \
                   skirmisher tricks her companion may use (no trick engine exists)"
            .to_owned(),
        claim_blocking: false,
    });
    ground_hunter_wild_empathy(level, ability_modifiers, explanations);
    ground_hunter_remaining_features(level, explanations);
    ground_or_block_hunter_animal_focus(input, level, explanations, diagnostics);
    ground_or_block_hunter_known_spells(input, level, explanations, diagnostics);
    ground_hunter_spellcasting_chassis_and_nature_training(level, ability_modifiers, explanations);
    // v0.6 alpha swarm, task #44: the known-spell posture is now genuinely
    // validated (the union of the Druid and Ranger general spell lists,
    // take-the-lower on a level conflict, the Hunter Spells Known table,
    // and the six automatic Summon Nature's Ally grants -- see
    // `ground_or_block_hunter_known_spells`), so this diagnostic is
    // renamed from `spellcasting_deferred` to `other_features_deferred`,
    // mirroring Skald's/Bloodrager's/Brawler's own diagnostic-honesty
    // rename exactly once their own gating feature stopped being
    // genuinely deferred.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.hunter.other_features_deferred.unsupported".to_owned(),
        message: format!(
            "{HUNTER_CLASS_ID} now grounds every named feature on its corpus class table: the \
             base-attack-bonus/base-save chassis pillar, its class-skill list, Animal Companion, \
             Wild Empathy, Track, Bonus Tricks, Precise Companion, the Bull Animal Focus, the \
             known-spell posture (task #44 -- the union of the Druid and Ranger general spell \
             lists, take-the-lower on the 27 spells whose level conflicts between those two \
             lists, the Hunter Spells Known table for levels 1-20, and the six automatic Summon \
             Nature's Ally grants), and -- newly, task #91 -- the spell-level access ladder, the \
             per-day CAST-table slot totals, the Wisdom-based spell save DC, and Nature Training. \
             This diagnostic is therefore no longer claim-blocking; it is retained to carry the \
             honest remainder. What stays deferred: (1) CASTING EXECUTION -- slot consumption, \
             spell tracking, and casting itself -- exactly as it stays deferred for every other \
             caster in this codebase including Skald, Sorcerer and Wizard; the access ladder, \
             per-day budget and save DC are grounded as flat records, and no engine spends them. \
             Bonus slots from a high Wisdom are likewise not computed. (2) The Animal Focus \
             family is covered at 1 of its 13 corpus records (Bull), narrowed the same way \
             Oracle's Mystery and Cavalier's Order of the Sword are. That is a catalog gap in a \
             chooser list, not a missing magnitude, and it does NOT block: a Hunter who records \
             an unrecognised focus is still caught by the separate, still-claim-blocking \
             `animal_focus_execution.focus_choice_missing` record, which fires on absence of a \
             recognisable choice rather than on incompleteness of the catalog. The twelve \
             unmodelled options are Bat, Bear, Falcon, Frog, Monkey, Mouse, Owl, Snake, Stag, \
             Tiger, Wolf, and the \"No Ability\" sentinel -- note WOLF, which an earlier revision \
             of this message omitted while still claiming a count of twelve. (3) Nature Training \
             grounds as a qualification flag with no magnitude; no druid-or-ranger-gated \
             companion feat resolution exists for it to unlock. This message previously claimed \
             this class had \"no class-skill list\" and \"no Precise Companion\" -- both were \
             already grounded when it said so, and the claim was stale"
        ),
        claim_blocking: false,
    });
}

/// v0.6 alpha swarm, risks item 8 (Hunter deepening, 2026-07-26, task
/// #2): Wild Empathy (a flat, unconditional check-modifier fact) and
/// Animal Focus (Bull) (activation-gated, mirroring Judgment's own
/// three-branch shape) are grounded alongside the already-shipped Animal
/// Companion. Hunter still never reaches `Computed` (spellcasting/Nature
/// Training/the other 12 Animal Focus options stay deferred), so this
/// mirrors `bloodrager_dispatch_widening_safety_tests`'s shape (stays
/// Blocked throughout).
#[cfg(test)]
mod hunter_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, ActiveState, AcquisitionMode, CharacterClassLevel,
        CharacterInput, HeadlessReceiptStatus, HUNTER_ANIMAL_FOCUS_ABILITY_ID,
        HUNTER_ANIMAL_FOCUS_CHOICE_ID, HUNTER_ANIMAL_FOCUS_BULL_SELECTION_ID, HUNTER_CLASS_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_hunter_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: HUNTER_CLASS_ID.to_owned(), level }];
        input
    }

    /// Wild Empathy is unconditional the moment Hunter levels are present
    /// -- no choice, no activation gate. Fixture: CHA 8 -> -1 modifier.
    /// Level 1: -1 + 1 = 0. (This test stays at level 1 because that is
    /// the fixture's own level, not because of any companion limitation:
    /// Hunter's Animal Companion math is now grounded at every master
    /// level 1-20 from the real corpus progression, so the end-to-end
    /// pipeline no longer panics above level 1. Wild Empathy's own
    /// level-scaling is proven separately below via a direct,
    /// pipeline-free unit test.)
    #[test]
    fn single_class_hunter_gets_the_unconditional_wild_empathy_bonus() {
        let input = human_hunter_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        let wild_empathy = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.hunter.wild_empathy_bonus")
            .expect("expected Wild Empathy grounded at level 1");
        assert_eq!(wild_empathy.value, 0, "level 1 Wild Empathy: {:?}", wild_empathy);
    }

    /// Wild Empathy's own real level-scaling (Hunter level + Charisma
    /// modifier), proven directly rather than through the level-1-only
    /// end-to-end pipeline (see the test above's own doc comment for why).
    #[test]
    fn hunter_wild_empathy_bonus_matches_the_real_cha_plus_level_formula() {
        for (level, charisma_modifier, expected) in [(1, -1, 0), (5, -1, 4), (10, 2, 12), (20, 0, 20)] {
            assert_eq!(
                super::hunter_wild_empathy_bonus(level, charisma_modifier),
                expected,
                "level {level}, CHA mod {charisma_modifier}"
            );
        }
    }

    /// A Hunter not using Animal Focus (no activation entry at all) is a
    /// genuinely valid PF1 posture, producing an honest "not focused"
    /// recognition record rather than a diagnostic.
    ///
    /// **Task #91 flips the status assertion** -- with Hunter's
    /// spellcasting chassis and Nature Training grounded the class
    /// computes. The posture fact this test protects is unchanged.
    #[test]
    fn single_class_hunter_not_focused_computes_with_an_honest_not_focused_record() {
        let input = human_hunter_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "declining to use Animal Focus is a valid posture and must not block: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.hunter.animal_focus_execution.not_focused"),
            "expected the honest not-focused recognition record: {:?}",
            receipt.computation.explanations
        );
    }

    /// A single-class Hunter actively using the Bull Animal Focus applies
    /// the real STR enhancement bonus as a standalone fact (level 1: +2;
    /// see the test above's own doc comment for why the full pipeline is
    /// level-1-bounded -- the formula's own level scaling is proven
    /// separately below).
    #[test]
    fn single_class_hunter_actively_focused_on_bull_applies_the_real_bonus() {
        let mut input = human_hunter_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: HUNTER_ANIMAL_FOCUS_CHOICE_ID.to_owned(),
            selection_id: HUNTER_ANIMAL_FOCUS_BULL_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: HUNTER_ANIMAL_FOCUS_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        let active = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.hunter.animal_focus_execution.active")
            .expect("expected Bull Animal Focus grounded at level 1");
        assert_eq!(active.value, 2, "level 1 Bull bonus: {:?}", active);
    }

    /// SD-32 T12 Epic 8 row 18 cycle 17: every one of the 11 magnitude-bearing
    /// Animal Focus options resolves to its own real level-scaled magnitude,
    /// proven directly against `HUNTER_ANIMAL_FOCUS_TIERED_OPTIONS`'s own
    /// per-option base/tier fields (each independently verified against its
    /// own `data/corpus/advanced_class_guide/class_feature/hunter_animal_focus/
    /// <name>.json`, see that table's own doc). Tiger mirrors Bull exactly
    /// (2/2/2); Falcon/Frog/Monkey/Owl share the 4/2/2 shape; Stag is 5/5/5;
    /// Wolf is 10/10/10; Bat is 60/30/0 (its own level-15 benefit is the
    /// separate blindsense boolean, proven in its own test below).
    #[test]
    fn every_tiered_animal_focus_option_matches_its_own_real_level_gates() {
        let expectations: &[(&str, &[(u8, i16)])] = &[
            ("animal_focus:bull", &[(1, 2), (7, 2), (8, 4), (14, 4), (15, 6), (20, 6)]),
            ("animal_focus:bear", &[(1, 2), (7, 2), (8, 4), (14, 4), (15, 6), (20, 6)]),
            ("animal_focus:tiger", &[(1, 2), (7, 2), (8, 4), (14, 4), (15, 6), (20, 6)]),
            ("animal_focus:falcon", &[(1, 4), (7, 4), (8, 6), (14, 6), (15, 8), (20, 8)]),
            ("animal_focus:frog", &[(1, 4), (7, 4), (8, 6), (14, 6), (15, 8), (20, 8)]),
            ("animal_focus:monkey", &[(1, 4), (7, 4), (8, 6), (14, 6), (15, 8), (20, 8)]),
            ("animal_focus:owl", &[(1, 4), (7, 4), (8, 6), (14, 6), (15, 8), (20, 8)]),
            ("animal_focus:snake", &[(1, 2), (7, 2), (8, 4), (14, 4), (15, 6), (20, 6)]),
            ("animal_focus:stag", &[(1, 5), (7, 5), (8, 10), (14, 10), (15, 15), (20, 15)]),
            ("animal_focus:wolf", &[(1, 10), (7, 10), (8, 20), (14, 20), (15, 30), (20, 30)]),
            ("animal_focus:bat", &[(1, 60), (7, 60), (8, 90), (14, 90), (15, 90), (20, 90)]),
        ];
        for (selection_id, per_level) in expectations {
            let option = super::HUNTER_ANIMAL_FOCUS_TIERED_OPTIONS
                .iter()
                .find(|o| &o.selection_id == selection_id)
                .unwrap_or_else(|| panic!("missing tiered option {selection_id}"));
            for (level, expected) in *per_level {
                assert_eq!(
                    super::hunter_animal_focus_tiered_bonus(option, *level),
                    *expected,
                    "{selection_id} at level {level}"
                );
            }
        }
    }

    /// A single-class Hunter actively using the Tiger Animal Focus (of the
    /// 12 options grounded this cycle, alongside Bull) applies its own real
    /// DEX enhancement bonus through the full end-to-end pipeline, proving
    /// the widened active-branch dispatch, not only the standalone formula.
    #[test]
    fn single_class_hunter_actively_focused_on_tiger_applies_the_real_bonus() {
        let mut input = human_hunter_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: HUNTER_ANIMAL_FOCUS_CHOICE_ID.to_owned(),
            selection_id: "animal_focus:tiger".to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: HUNTER_ANIMAL_FOCUS_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        let active = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.hunter.animal_focus_execution.active")
            .expect("expected Tiger Animal Focus grounded at level 1");
        assert_eq!(active.value, 2, "level 1 Tiger bonus: {:?}", active);
    }

    /// Bat Animal Focus's own extra level-15 blindsense fact is a SEPARATE
    /// boolean explanation from its darkvision-range magnitude -- proven
    /// present at level 15 and absent below it.
    #[test]
    fn bat_animal_focus_blindsense_appears_only_at_level_15_and_above() {
        for (level, expect_blindsense) in [(1u8, false), (14, false), (15, true), (20, true)] {
            let mut input = human_hunter_input(level);
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: HUNTER_ANIMAL_FOCUS_CHOICE_ID.to_owned(),
                selection_id: "animal_focus:bat".to_owned(),
            });
            input.chosen.class_ability_activations.push(ClassAbilityActivation {
                ability_id: HUNTER_ANIMAL_FOCUS_ABILITY_ID.to_owned(),
                active_state: ActiveState::EquippedActive,
                rounds_consumed_today: None,
            });

            let receipt = build_pilot_headless_receipt(&input);
            let has_blindsense = receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.hunter.animal_focus_execution.bat_blindsense");
            assert_eq!(
                has_blindsense, expect_blindsense,
                "level {level} Bat blindsense presence"
            );
        }
    }

    /// Mouse Animal Focus is a boolean evasion/improved-evasion posture
    /// fact, not a `BONUS:VAR` magnitude -- proven at level 1 (evasion
    /// only) and level 12+ (improved evasion too).
    #[test]
    fn mouse_animal_focus_grounds_evasion_and_improved_evasion_at_its_own_gate() {
        for (level, expect_improved) in [(1u8, false), (11, false), (12, true), (20, true)] {
            let mut input = human_hunter_input(level);
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: HUNTER_ANIMAL_FOCUS_CHOICE_ID.to_owned(),
                selection_id: "animal_focus:mouse".to_owned(),
            });
            input.chosen.class_ability_activations.push(ClassAbilityActivation {
                ability_id: HUNTER_ANIMAL_FOCUS_ABILITY_ID.to_owned(),
                active_state: ActiveState::EquippedActive,
                rounds_consumed_today: None,
            });

            let receipt = build_pilot_headless_receipt(&input);
            let active = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.hunter.animal_focus_execution.active")
                .expect("expected Mouse Animal Focus grounded");
            assert_eq!(
                active.value,
                i16::from(expect_improved),
                "level {level} Mouse improved-evasion flag: {:?}",
                active
            );
        }
    }

    /// No Ability is a genuinely valid, text-only choice -- grounds cleanly
    /// with zero magnitude, never claim-blocks.
    #[test]
    fn no_ability_animal_focus_grounds_cleanly_with_zero_magnitude() {
        let mut input = human_hunter_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: HUNTER_ANIMAL_FOCUS_CHOICE_ID.to_owned(),
            selection_id: "animal_focus:no_ability".to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: HUNTER_ANIMAL_FOCUS_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);
        assert_ne!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "No Ability must never claim-block: {:?}",
            receipt.computation.diagnostics
        );
        let active = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.hunter.animal_focus_execution.active")
            .expect("expected No Ability grounded");
        assert_eq!(active.value, 0, "No Ability must carry zero magnitude: {:?}", active);
    }

    /// An active Animal Focus naming no recognized choice at all (all 13
    /// real corpus options are grounded as of cycle 17, so this now proves
    /// the "no selection made" posture specifically) is a genuine posture
    /// violation and must claim-block -- never silently passed.
    #[test]
    fn single_class_hunter_active_focus_without_a_recognized_bull_choice_stays_blocked() {
        let mut input = human_hunter_input(1);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: HUNTER_ANIMAL_FOCUS_ABILITY_ID.to_owned(),
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
                    == "class_feature.acg.hunter.animal_focus_execution.focus_choice_missing"
                    && d.claim_blocking),
            "expected the missing-focus-choice claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// ACG `Extended Animal Focus` widens that same genuinely-enforced
    /// budget (v0.6 alpha swarm, `BONUS:VAR` triage slice, 2026-07-29).
    ///
    /// `acg_feats.lst`: `BONUS:VAR|HunterAnimalFocusMinutes|max(1,WIS)`,
    /// prose "Add your Wisdom modifier [minimum 1] to the number of
    /// minutes per day that you can use your animal focus ability" --
    /// token and prose agree exactly, including the minimum-1 floor. It
    /// writes to the SAME variable the class record's own
    /// `BONUS:VAR|HunterAnimalFocusMinutes|HunterLVL` sets, which
    /// `hunter_animal_focus_uses_per_day` already computes.
    ///
    /// The pin is behavioural, not cosmetic: the fixture's Wisdom 12
    /// (+1) makes the level-1 budget 1 + 1 = 2 minutes, so the exact
    /// activation the test below proves is claim-BLOCKING without the
    /// feat must become valid with it. No `STACK:`/`MULT:` on the
    /// record and no repeat clause in its BENEFIT, so it is
    /// presence-based.
    #[test]
    fn extended_animal_focus_widens_the_enforced_hunter_minutes_budget() {
        let mut input = human_hunter_input(1);
        input.chosen.selected_feats.push("Extended Animal Focus".to_owned());
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: HUNTER_ANIMAL_FOCUS_CHOICE_ID.to_owned(),
            selection_id: HUNTER_ANIMAL_FOCUS_BULL_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: HUNTER_ANIMAL_FOCUS_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(2),
        });

        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.hunter.animal_focus_execution.uses_exceeded"),
            "2 minutes is within the Extended-Animal-Focus-widened budget (level 1 + Wisdom \
             modifier 1 = 2); the enforced budget must see the feat: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The magnitude itself, across levels and Wisdom modifiers,
    /// including the corpus's own `max(1, WIS)` floor at zero and
    /// negative Wisdom.
    #[test]
    fn extended_animal_focus_adds_the_wisdom_modifier_with_a_minimum_of_one() {
        let feat = vec!["Extended Animal Focus".to_owned()];
        for (level, wisdom, want) in [
            (1u8, 1i16, 2i16),
            (1, 4, 5),
            (1, 0, 2),
            (1, -2, 2),
            (5, 3, 8),
            (20, 5, 25),
        ] {
            assert_eq!(
                super::hunter_animal_focus_uses_per_day(level, wisdom, &feat),
                want,
                "level {level}, Wisdom modifier {wisdom}"
            );
            assert_eq!(
                super::hunter_animal_focus_uses_per_day(level, wisdom, &[]),
                i16::from(level),
                "without the feat, level {level} must stay at the bare class budget"
            );
        }
    }

    /// An Animal Focus activation that exceeds the grounded per-day
    /// minutes budget is a genuine posture violation and must claim-block.
    ///
    /// Level 1 uses per day: 1 minute.
    #[test]
    fn single_class_hunter_over_budget_animal_focus_stays_blocked_and_applies_no_bonus() {
        let mut input = human_hunter_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: HUNTER_ANIMAL_FOCUS_CHOICE_ID.to_owned(),
            selection_id: HUNTER_ANIMAL_FOCUS_BULL_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: HUNTER_ANIMAL_FOCUS_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(2),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.hunter.animal_focus_execution.uses_exceeded"
                    && d.claim_blocking),
            "expected the uses-exceeded claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.hunter.animal_focus_execution.active"),
            "no Bull bonus should be applied for an over-budget posture: {:?}",
            receipt.computation.explanations
        );
    }

    /// A bare level-1 Hunter (no freely chosen known spells) still
    /// automatically knows all six Summon Nature's Ally spells -- that
    /// grant is unconditional on class ownership alone, mirroring Wild
    /// Empathy's own "always on" shape -- and zero freely chosen known
    /// spells is itself a valid posture (mirrors Oracle's/Sorcerer's own
    /// reasoning), so the known_spells diagnostic never fires (v0.6 alpha
    /// swarm, task #44).
    #[test]
    fn single_class_hunter_bare_gets_the_six_automatic_summon_natures_ally_spells() {
        let input = human_hunter_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.hunter.known_spells.unsupported"),
            "zero freely chosen known spells is itself a valid posture: {:?}",
            receipt.computation.diagnostics
        );
        let automatic = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.hunter.automatic_summon_natures_ally_known_spells")
            .expect("expected the automatic Summon Nature's Ally grant to be grounded");
        assert_eq!(automatic.value, 6, "all six Summon Nature's Ally spells: {:?}", automatic);
        assert!(
            automatic.detail.contains("Summon Nature's Ally I")
                && automatic.detail.contains("Summon Nature's Ally VI"),
            "detail should name the spells: {:?}",
            automatic
        );
        let freely_chosen = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.hunter.known_spells")
            .expect("expected the freely-chosen known-spell count to be grounded");
        assert_eq!(freely_chosen.value, 0, "no spells were freely chosen: {:?}", freely_chosen);
    }

    /// Hunter's per-day CAST table is a DIFFERENT table from its
    /// spells-known table and must never be derived from it.
    ///
    /// The corpus's own comment above the class block warns that the
    /// KNOWN column is deliberately inflated by 1 to absorb the automatic
    /// Summon Nature's Ally grants, while CAST carries no such
    /// adjustment. Level 1 makes the divergence visible: 1 slot per day
    /// against 3 freely-chosen 1st-level spells known.
    ///
    /// The access ceiling is derived from the same table by counting
    /// populated columns, so the two can never disagree; Hunter reaches
    /// spell level 6, not 4 or 9.
    #[test]
    fn hunter_per_day_cast_table_is_distinct_from_its_known_table_and_caps_at_six() {
        for (level, want) in [
            (1u8, [Some(1i16), None, None, None, None, None]),
            (4, [Some(3), Some(1), None, None, None, None]),
            (10, [Some(5), Some(4), Some(3), Some(1), None, None]),
            (16, [Some(5), Some(5), Some(5), Some(4), Some(3), Some(1)]),
            (20, [Some(5), Some(5), Some(5), Some(5), Some(5), Some(5)]),
        ] {
            assert_eq!(
                super::hunter_base_spells_per_day_table(level),
                want,
                "hunter level {level}"
            );
        }

        for (level, want) in [(1u8, 1i16), (3, 1), (4, 2), (7, 3), (10, 4), (13, 5), (16, 6), (20, 6)] {
            assert_eq!(super::hunter_spell_level_access(level), want, "level {level}");
        }

        // The two tables genuinely differ at level 1: 1 slot/day, but 3
        // freely-chosen 1st-level spells known.
        assert_eq!(super::hunter_base_spells_per_day_table(1)[0], Some(1));
        assert_eq!(super::hunter_spells_known_table(1)[1], Some(3));

        let receipt = build_pilot_headless_receipt(&human_hunter_input(1));
        let value = |id: &str| {
            receipt.computation.explanations.iter().find(|e| e.id == id).map(|e| e.value)
        };
        assert_eq!(
            value("class_chassis.hunter.spontaneous.base_spells_per_day.spell_level_1"),
            Some(1),
        );
        // Orisons are UNLIMITED (corpus CAST: 0), so no zero-slot record
        // may be emitted for spell level 0.
        assert_eq!(
            value("class_chassis.hunter.spontaneous.base_spells_per_day.spell_level_0"),
            None,
            "a spell-level-0 record would report the CAST: 0 sentinel as 'no orisons'"
        );
    }

    /// Hunter's spell save DC keys off WISDOM (SPELLSTAT:WIS, a divine
    /// caster), not the Charisma its ACG sibling Skald uses. The fixture
    /// gives a nonzero Wisdom modifier so a copy-paste of Skald's
    /// Charisma term would produce a different number and fail here.
    #[test]
    fn hunter_spell_save_dc_uses_wisdom_not_charisma() {
        let input = human_hunter_input(5);
        let receipt = build_pilot_headless_receipt(&input);

        let wisdom_modifier = super::ability_modifier(input.chosen.ability_scores.wisdom);
        let charisma_modifier = super::ability_modifier(input.chosen.ability_scores.charisma);
        assert_ne!(
            wisdom_modifier, charisma_modifier,
            "this test is only meaningful when the two modifiers differ in the fixture"
        );

        for spell_level in 1..=2i16 {
            let dc = receipt
                .computation
                .explanations
                .iter()
                .find(|e| {
                    e.id
                        == format!(
                            "class_chassis.hunter.spontaneous.spell_save_dc.spell_level_{spell_level}"
                        )
                })
                .unwrap_or_else(|| panic!("spell level {spell_level} DC must ground"))
                .value;
            assert_eq!(dc, 10 + spell_level + wisdom_modifier, "spell level {spell_level}");
        }
    }

    /// Nature Training is a pure qualification flag: a real corpus record
    /// carrying no BONUS, DEFINE or ADD. It grounds as a zero-magnitude
    /// identity record from 1st level.
    #[test]
    fn hunter_nature_training_grounds_as_a_zero_magnitude_qualification_flag() {
        let receipt = build_pilot_headless_receipt(&human_hunter_input(1));
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.hunter.nature_training_grant")
            .expect("Nature Training grounds from level 1");
        assert_eq!(record.value, 0);
        assert!(
            record.detail.contains("KEY:Hunter ~ Nature Training"),
            "must cite its namespaced corpus key: {}",
            record.detail
        );
    }

    /// A real, on-list spell recorded as a Hunter known spell grounds the
    /// freely-chosen known-spell count for real (v0.6 alpha swarm, task
    /// #44). `Cure Light Wounds` is Druid 1st-level, so it must be
    /// accepted at Hunter level 1.
    #[test]
    fn single_class_hunter_with_a_real_known_spell_grounds_it() {
        let mut input = human_hunter_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: HUNTER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.hunter.known_spells.unsupported"),
            "the known_spells diagnostic must not fire once a real known spell is recorded: {:?}",
            receipt.computation.diagnostics
        );
        let known = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.hunter.known_spells")
            .expect("known-spell count must be grounded");
        assert_eq!(known.value, 1, "Hunter level 1 with one freely chosen known spell: {:?}", known);
    }

    /// `Cure Light Wounds` is Druid 1st-level / Ranger 2nd-level -- a real
    /// conflicting entry (v0.6 alpha swarm, task #44). The take-the-lower
    /// ruling must resolve it to 1st level: a level-1 Hunter recording it
    /// grounds cleanly (only 1st-level slots are accessible at Hunter
    /// level 1), proving the ruling end to end. If the ruling instead took
    /// the HIGHER level (2nd), this same input would incorrectly
    /// claim-block as "not yet accessible at hunter level 1", since a
    /// Hunter Spells Known table has no 2nd-level column until level 4.
    #[test]
    fn a_conflicting_level_spell_resolves_per_the_take_the_lower_ruling() {
        assert_eq!(
            super::druid_spell_list::druid_spell_level("Cure Light Wounds"),
            Some(1),
            "Cure Light Wounds must be Druid 1st-level"
        );
        assert_eq!(
            super::ranger_spell_list::ranger_spell_level("Cure Light Wounds"),
            Some(2),
            "Cure Light Wounds must be Ranger 2nd-level"
        );
        assert_eq!(
            super::hunter_spell_list::hunter_spell_level("Cure Light Wounds"),
            Some(1),
            "take-the-lower must resolve Cure Light Wounds to 1st level"
        );

        let mut input = human_hunter_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: HUNTER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.hunter.known_spells.unsupported"),
            "a level-1 Hunter recording Cure Light Wounds must ground cleanly under the \
             take-the-lower ruling (it would claim-block as inaccessible under take-the-higher): \
             {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A Hunter known spell not on the union of the Druid/Ranger general
    /// lists (or above Hunter's own 6th-level ceiling) is a genuine
    /// posture violation and must claim-block (v0.6 alpha swarm, task
    /// #44).
    #[test]
    fn single_class_hunter_with_an_off_list_known_spell_stays_blocked_on_known_spells() {
        let mut input = human_hunter_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Definitely Not A Real Spell".to_owned(),
            source_class_id: HUNTER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.hunter.known_spells.unsupported"
                    && d.claim_blocking),
            "expected the known_spells claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A 9th-level Druid/Ranger-general-list spell (level above Hunter's
    /// own 6th-level ceiling) is excluded the same way as an off-list
    /// spell -- `hunter_spell_level` returns `None` above the ceiling, so
    /// no separate range check is needed anywhere in the grounding itself
    /// (v0.6 alpha swarm, task #44).
    #[test]
    fn single_class_hunter_with_a_spell_above_the_level_6_ceiling_stays_blocked() {
        assert_eq!(
            super::druid_spell_list::druid_spell_level("Summon Nature's Ally IX"),
            Some(9),
            "Summon Nature's Ally IX must be a real Druid 9th-level spell"
        );
        assert_eq!(
            super::hunter_spell_list::hunter_spell_level("Summon Nature's Ally IX"),
            None,
            "Summon Nature's Ally IX must be excluded above Hunter's own 6th-level ceiling"
        );

        let mut input = human_hunter_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Summon Nature's Ally IX".to_owned(),
            source_class_id: HUNTER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.hunter.known_spells.unsupported"
                    && d.claim_blocking),
            "expected the known_spells claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A Hunter recording more freely chosen known spells at a level than
    /// the Hunter Spells Known table allows for that level is a genuine
    /// posture violation and must claim-block (v0.6 alpha swarm, task
    /// #44). Level 1 cap for 1st-level spells is 3; record a 4th distinct
    /// 1st-level spell.
    #[test]
    fn single_class_hunter_over_known_at_a_spell_level_stays_blocked_on_known_spells() {
        let mut input = human_hunter_input(1);
        for spell_id in ["Ant Haul", "Charm Animal", "Calm Animals", "Bristle"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: HUNTER_CLASS_ID.to_owned(),
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
                .any(|d| d.id == "class_spell.acg.hunter.known_spells.unsupported"
                    && d.claim_blocking
                    && d.message.contains("over-known")),
            "expected the over-known known_spells claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// `hunter_spells_known_table`'s own real level-scaling (byte-exact
    /// transcription from `acg_classes.lst`'s Hunter "Level progression"
    /// block, verified at a spread of levels including the 4th/7th/10th/
    /// 13th/16th-level column-widening rows), proven directly rather than
    /// through the level-1-only end-to-end pipeline (see
    /// `human_hunter_input`'s own doc comment for why: Hunter's Animal
    /// Companion math is only verified at companion level 1 in this
    /// codebase, so the full pipeline can only be exercised at Hunter
    /// level 1).
    #[test]
    fn hunter_spells_known_table_matches_the_real_acg_level_progression() {
        for (level, expected) in [
            (1, [Some(4), Some(3), None, None, None, None, None]),
            (4, [Some(6), Some(5), Some(3), None, None, None, None]),
            (7, [Some(6), Some(6), Some(5), Some(3), None, None, None]),
            (10, [Some(6), Some(6), Some(6), Some(5), Some(3), None, None]),
            (13, [Some(6), Some(7), Some(6), Some(6), Some(5), Some(3), None]),
            (16, [Some(6), Some(7), Some(7), Some(6), Some(6), Some(5), Some(3)]),
            (20, [Some(6), Some(7), Some(7), Some(7), Some(7), Some(6), Some(6)]),
        ] {
            assert_eq!(super::hunter_spells_known_table(level), expected, "level {level}");
        }
    }
}

/// v0.6 alpha swarm, risks item 8 (Swashbuckler full-build closure, 9th
/// ACG/APG class-specific closure): tests Panache/Nimble's unconditional
/// flat grounding and Charmed Life's level-gated, activation-gated,
/// budget-enforced dispatch, mirroring the established dispatch-widening
/// test module shape.
#[cfg(test)]
mod swashbuckler_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, ActiveState, CharacterClassLevel, CharacterInput,
        HeadlessReceiptStatus, FIGHTER_CLASS_ID, SWASHBUCKLER_CHARMED_LIFE_ABILITY_ID,
        SWASHBUCKLER_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, ClassAbilityActivation};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_swashbuckler_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SWASHBUCKLER_CLASS_ID.to_owned(), level }];
        input
    }

    /// A single-class Human Swashbuckler at level 1 stays `Blocked` on
    /// the new, narrower `other_features_deferred` diagnostic alone
    /// (never the retired generic one), with Panache's daily max and
    /// Nimble's dodge bonus grounded unconditionally, and Charmed Life
    /// correctly recognized as not-yet-gained at level 1 (real PF1 rule:
    /// granted starting level 2).
    ///
    /// Fixture Charisma 8 (-1 modifier): Panache max(1,-1)=1. Level 1
    /// Nimble: (1+1)/4=0.
    ///
    /// **Task #91 flips the status assertion.** The thirteen deeds,
    /// Swashbuckler Weapon Mastery and the bonus-feat slot count that
    /// this diagnostic's blocking claim named are now grounded, so the
    /// record is demoted rather than deleted and Swashbuckler computes.
    #[test]
    fn single_class_swashbuckler_level1_computes_with_panache_and_nimble_grounded_and_charmed_life_not_yet_gained()
    {
        let input = human_swashbuckler_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Swashbuckler grounds every named corpus feature and must now compute: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.swashbuckler.unsupported"),
            "the retired generic diagnostic must never appear for Swashbuckler: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.swashbuckler.other_features_deferred.unsupported"
                    && !d.claim_blocking),
            "the remainder record must survive as a NON-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );

        let panache = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.swashbuckler.panache_max")
            .expect("Panache max must ground unconditionally");
        assert_eq!(panache.value, 1, "fixture Charisma 8 (-1 modifier): max(1,-1)=1: {:?}", panache);

        let nimble = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.swashbuckler.nimble_dodge_bonus")
            .expect("Nimble dodge bonus must ground unconditionally");
        assert_eq!(nimble.value, 0, "Swashbuckler level 1 Nimble: (1+1)/4=0: {:?}", nimble);

        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.swashbuckler.charmed_life_not_yet_gained"),
            "expected the honest not-yet-gained recognition record at level 1: {:?}",
            receipt.computation.explanations
        );
    }

    /// All nineteen corpus Deed records are now covered, each appearing
    /// exactly at its own corpus tier gate (1/3/7/11/15/19) and never
    /// before it.
    ///
    /// The count is asserted against the corpus's real total rather than
    /// a running tally, so adding a deed record without gating it, or
    /// double-emitting one, fails here.
    #[test]
    fn all_nineteen_deeds_are_covered_each_at_its_own_corpus_tier() {
        // (explanation id fragment, first level it may appear)
        let deeds: &[(&str, u8)] = &[
            // The six already built before task #91.
            ("derring_do_uses_per_day", 1),
            ("dodging_panache_dodge_bonus", 1),
            ("precise_strike_damage", 3),
            ("bleeding_wound_damage", 11),
            ("deadly_stab_dc", 19),
            ("stunning_stab_dc", 19),
            // The thirteen closed by task #91.
            ("deed.opportune_parry_and_riposte_grant", 1),
            ("deed.kip_up_grant", 3),
            ("deed.menacing_swordplay_grant", 3),
            ("deed.swashbuckler_initiative_bonus", 3),
            ("deed.swashbucklers_grace_grant", 7),
            ("deed.superior_feint_grant", 7),
            ("deed.targeted_strike_grant", 7),
            ("deed.subtle_blade_grant", 11),
            ("deed.evasive_grant", 11),
            ("deed.dizzying_defense_dodge_bonus", 15),
            ("deed.perfect_thrust_grant", 15),
            ("deed.swashbucklers_edge_grant", 15),
            ("deed.cheat_death_grant", 19),
        ];
        assert_eq!(
            deeds.len(),
            19,
            "the corpus carries exactly nineteen Swashbuckler Deed records"
        );

        for (fragment, gate) in deeds {
            let id = format!("class_feature.acg.swashbuckler.{fragment}");
            if *gate > 1 {
                let below = build_pilot_headless_receipt(&human_swashbuckler_input(gate - 1));
                assert!(
                    !below.computation.explanations.iter().any(|e| e.id == id),
                    "{id} must not exist at level {}, below its tier-{gate} gate",
                    gate - 1
                );
            }
            let at_gate = build_pilot_headless_receipt(&human_swashbuckler_input(*gate));
            assert!(
                at_gate.computation.explanations.iter().any(|e| e.id == id),
                "{id} must ground at its tier-{gate} gate"
            );
            let capstone = build_pilot_headless_receipt(&human_swashbuckler_input(20));
            assert_eq!(
                capstone.computation.explanations.iter().filter(|e| e.id == id).count(),
                1,
                "{id} must appear exactly once at level 20, never double-emitted"
            );
        }
    }

    /// The two deeds among the thirteen that are NOT zero-magnitude
    /// carry their real corpus values.
    ///
    /// Swashbuckler Initiative's +2 comes from a `TEMPBONUS`, and
    /// Dizzying Defense's +4/-2 exist only in DESC prose -- two
    /// different evidentiary paths, both distinct from the eleven
    /// genuinely tokenless deeds, and both easy to mis-file as
    /// zero-magnitude.
    #[test]
    fn the_two_numeric_deeds_carry_real_values_not_zero() {
        let receipt = build_pilot_headless_receipt(&human_swashbuckler_input(15));
        let value = |id: &str| {
            receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .map(|e| e.value)
        };

        assert_eq!(
            value("class_feature.acg.swashbuckler.deed.swashbuckler_initiative_bonus"),
            Some(2),
        );
        assert_eq!(
            value("class_feature.acg.swashbuckler.deed.dizzying_defense_dodge_bonus"),
            Some(4),
        );
        // The penalty is a real negative, not an absolute value.
        assert_eq!(
            value("class_feature.acg.swashbuckler.deed.dizzying_defense_attack_penalty"),
            Some(-2),
        );

        // ...while a representative tokenless deed is genuinely 0.
        assert_eq!(
            value("class_feature.acg.swashbuckler.deed.kip_up_grant"),
            Some(0),
        );
    }

    /// Swashbuckler Weapon Mastery's magnitude is NOT on its parent
    /// record -- it lives on the two Internal helper records the parent
    /// grants (`CRITMULTADD|1`). Reading only the parent would conclude
    /// this feature is zero-magnitude, so the assertion is specifically
    /// that it is 1 and not 0.
    #[test]
    fn weapon_mastery_reads_its_magnitude_off_the_helper_records_not_the_parent() {
        let id = "class_feature.acg.swashbuckler.weapon_mastery_critical_multiplier_increase";
        let below = build_pilot_headless_receipt(&human_swashbuckler_input(19));
        assert!(!below.computation.explanations.iter().any(|e| e.id == id));

        let at_20 = build_pilot_headless_receipt(&human_swashbuckler_input(20));
        let mastery = at_20
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .expect("Weapon Mastery grounds at 20");
        assert_eq!(
            mastery.value, 1,
            "the +1 critical multiplier lives on the helper records; 0 would mean only the \
             parent was read: {mastery:?}"
        );
    }

    /// The bonus-feat pool is `level/4`, supplied by five separate .MOD
    /// increments. The parent record carries none, so reading it alone
    /// yields 0 forever -- hence the assertion that level 20 is 5.
    #[test]
    fn swashbuckler_bonus_feat_pool_sums_its_five_mod_increments() {
        for (level, want) in [
            (1u8, 0i16), (3, 0), (4, 1), (7, 1), (8, 2),
            (11, 2), (12, 3), (15, 3), (16, 4), (19, 4), (20, 5),
        ] {
            assert_eq!(super::swashbuckler_bonus_feat_count(level), want, "level {level}");
        }

        let id = "class_feature.acg.swashbuckler.bonus_feat_count";
        let at_3 = build_pilot_headless_receipt(&human_swashbuckler_input(3));
        assert!(
            !at_3.computation.explanations.iter().any(|e| e.id == id),
            "no slot exists before 4th level, so no record should claim one"
        );
        let at_20 = build_pilot_headless_receipt(&human_swashbuckler_input(20));
        assert_eq!(
            at_20.computation.explanations.iter().find(|e| e.id == id).map(|e| e.value),
            Some(5),
        );
    }

    /// task #88 correction: Nimble's own detail string used to claim "this
    /// codebase computes no player AC total anywhere to integrate this
    /// into" -- false: `is_supported_swashbuckler_single_class` is part of
    /// `has_supported_class_chassis`, so a GE-06-posture Swashbuckler
    /// reaches the same `defense.baseline_armor_class` pillar every other
    /// supported class does (Brawler's own AC Bonus already integrates
    /// into it). Nimble's magnitude simply isn't wired into it. Level 4
    /// gives a real nonzero bonus ((4+1)/4=1), so the "not folded in"
    /// assertion is meaningful.
    #[test]
    fn swashbuckler_nimble_detail_no_longer_falsely_claims_no_ac_total_exists() {
        let receipt = build_pilot_headless_receipt(&human_swashbuckler_input(4));

        let nimble = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.swashbuckler.nimble_dodge_bonus")
            .expect("Nimble dodge bonus must ground unconditionally");
        assert_eq!(nimble.value, 1, "Swashbuckler level 4 Nimble: (4+1)/4=1: {:?}", nimble);
        assert!(
            !nimble.detail.contains("computes no player AC total"),
            "the corrected detail must not repeat the false no-total-exists claim: {:?}",
            nimble
        );
        assert!(
            nimble.detail.contains("defense.baseline_armor_class"),
            "the corrected detail must name the real AC total it isn't wired into: {:?}",
            nimble
        );

        let baseline_ac = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .expect(
                "a GE-06-posture Swashbuckler is a supported class chassis, so baseline AC must \
                 be real, not absent",
            );
        assert!(
            !baseline_ac.detail.contains("Nimble"),
            "Nimble's +1 must NOT be folded into baseline AC yet -- the corrected claim says \
             'not wired in', not 'wired in': {:?}",
            baseline_ac
        );
    }

    /// A single-class Human Swashbuckler at level 2 (Charmed Life's real
    /// grant level) who is not currently spending a use is a genuinely
    /// valid PF1 posture -- grounds the honest "not active" record, with
    /// the real uses-per-day budget grounded regardless.
    ///
    /// Level 2 Charmed Life uses/day: ((2-2)/4)+3 = 3.
    #[test]
    fn single_class_swashbuckler_level2_not_using_charmed_life_grounds_the_honest_not_active_record()
    {
        let input = human_swashbuckler_input(2);
        let receipt = build_pilot_headless_receipt(&input);

        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.swashbuckler.charmed_life_not_active")
            .expect("expected the honest not-active recognition record");
        assert_eq!(record.value, 0, "{:?}", record);
    }

    /// A single-class Human Swashbuckler at level 2, actively spending a
    /// use of Charmed Life within budget, grounds the real Charisma
    /// magnitude as a standalone record.
    ///
    /// Fixture Charisma 8 (-1 modifier).
    #[test]
    fn single_class_swashbuckler_level2_actively_using_charmed_life_in_budget_grounds_the_real_bonus()
    {
        let mut input = human_swashbuckler_input(2);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: SWASHBUCKLER_CHARMED_LIFE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(2),
        });

        let receipt = build_pilot_headless_receipt(&input);

        let bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.swashbuckler.charmed_life_active_bonus")
            .expect("the active Charmed Life explanation must be grounded");
        assert_eq!(bonus.value, -1, "fixture Charisma 8 (-1 modifier): {:?}", bonus);
    }

    /// A single-class Human Swashbuckler at level 2 claiming more Charmed
    /// Life uses today than the real budget allows is a genuine posture
    /// violation and must claim-block, mirroring Rage's/Judgment's own
    /// genuinely-enforced over-budget check exactly.
    #[test]
    fn single_class_swashbuckler_level2_over_budget_charmed_life_stays_blocked_and_applies_no_bonus()
    {
        let mut input = human_swashbuckler_input(2);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: SWASHBUCKLER_CHARMED_LIFE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(4),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.swashbuckler.charmed_life_uses_exceeded"
                    && d.claim_blocking),
            "expected the uses_exceeded claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.swashbuckler.charmed_life_active_bonus"),
            "no bonus should apply once the budget is exceeded: {:?}",
            receipt.computation.explanations
        );
    }

    /// A non-Swashbuckler character carrying a spoofed Charmed Life
    /// activation must have it silently ignored -- the class-ownership
    /// gate is by construction, not a bolt-on rejection. Also proves
    /// Fighter's own golden path is unaffected.
    #[test]
    fn non_swashbuckler_characters_spoofed_charmed_life_entry_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: SWASHBUCKLER_CHARMED_LIFE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Swashbuckler entry: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.acg.swashbuckler.")),
            "a non-Swashbuckler character must never ground any Swashbuckler explanation: {:?}",
            receipt.computation.explanations
        );
    }
}

/// v0.6 alpha swarm, task #6 (Cavalier, 2026-07-27): the five named
/// features this closure grounds, plus the decoy-variable guard.
#[cfg(test)]
mod cavalier_named_feature_tests {
    use super::{
        build_pilot_headless_receipt, ActiveState, CharacterClassLevel, CharacterInput,
        CAVALIER_CHALLENGE_ABILITY_ID, CAVALIER_CLASS_ID, CAVALIER_ORDER_CHOICE_ID,
        ORDER_OF_THE_DRAGON_SELECTION, ORDER_OF_THE_SWORD_SELECTION,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation, SelectedChoice,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn cavalier(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: CAVALIER_CLASS_ID.to_owned(), level }];
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

    /// Challenge's uses/day is `(level+2)/3`, re-derived across the full
    /// range from the corpus token rather than spot-checked.
    #[test]
    fn challenge_uses_per_day_matches_the_corpus_formula_at_every_level() {
        for (level, expected) in
            [(1u8, 1i16), (2, 1), (3, 1), (4, 2), (6, 2), (7, 3), (10, 4), (20, 7)]
        {
            assert_eq!(
                super::cavalier_challenge_uses_per_day(level),
                expected,
                "level {level}: (level + 2)/3"
            );
        }
    }

    /// The decoy guard. Two similarly-named variables exist in the
    /// corpus: the real `OrderChallengeBonus` (`CavalierLVL/4`, consumed
    /// by all six Orders) and `CavalierOrderChallengeBonus`
    /// (`(CavalierLVL+3)/4`), which is defined on the base Challenge
    /// record -- the natural first place to look -- and referenced
    /// NOWHERE. This closure grounds neither (every order challenge rider
    /// is opponent- or ally-conditioned), so the guard is that no
    /// Cavalier explanation ever carries the decoy's value shape.
    #[test]
    fn no_grounded_value_uses_the_never_consumed_decoy_formula() {
        // At level 5 the two formulas diverge: real = 5/4 = 1,
        // decoy = (5+3)/4 = 2. The pipeline check stays at level 1 for
        // the companion-boundary reason noted above.
        let receipt = build_pilot_headless_receipt(&cavalier(1));
        let decoy = (5 + 3) / 4;
        assert_eq!(decoy, 2, "the decoy really does differ from the real formula at level 5");
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.contains("order_challenge")),
            "no order-challenge-bonus record is grounded at all this closure: {:?}",
            receipt.computation.explanations
        );
    }

    /// Challenge's -2 AC penalty applies only while actively
    /// challenging, and lands on the REAL armor class total -- proven by
    /// differencing the same character with and without the activation.
    #[test]
    fn an_active_challenge_lowers_the_real_armor_class_total_by_two() {
        let idle = cavalier(1);
        let without = value(&idle, "defense.baseline_armor_class").expect("AC computed");

        let mut challenging = idle.clone();
        challenging.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: CAVALIER_CHALLENGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });
        let with = value(&challenging, "defense.baseline_armor_class").expect("AC computed");

        assert_eq!(with - without, -2, "an active challenge costs 2 AC");
        assert_eq!(
            value(&challenging, "class_feature.apg.cavalier.challenge_armor_class_penalty"),
            Some(-2)
        );
        assert_eq!(
            value(&idle, "class_feature.apg.cavalier.challenge_not_active"),
            Some(0),
            "an idle cavalier gets the honest not-active record instead"
        );
    }

    /// A non-Cavalier carrying a spoofed challenge activation must not
    /// lose armor class.
    #[test]
    fn a_spoofed_challenge_never_lowers_a_non_cavaliers_armor_class() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut fighter = result.character_input.expect("valid fixture");
        let clean = value(&fighter, "defense.baseline_armor_class");
        fighter.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: CAVALIER_CHALLENGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });
        assert_eq!(
            value(&fighter, "defense.baseline_armor_class"),
            clean,
            "a Fighter's AC is untouched by a spoofed Cavalier challenge"
        );
    }

    /// Expert Trainer is a 4th-level feature and must not ground before
    /// its own gate.
    ///
    /// Pipeline assertions stay at level 1 and higher levels are exercised
    /// through the pure formula. This used to be forced --
    /// `ground_horse_companion_stat_block` carried a `debug_assert` that
    /// only companion level 1 was grounded (Hunter hit the identical one)
    /// -- but that boundary is gone now that the companion's Hit Dice come
    /// from the real corpus progression at every master level 1-20. The
    /// split is kept because the pure formula is the sharper assertion for
    /// a per-level magnitude.
    #[test]
    fn expert_trainer_grounds_only_from_fourth_level() {
        assert_eq!(
            value(&cavalier(1), "class_feature.apg.cavalier.expert_trainer_bonus"),
            None,
            "a level-1 Cavalier has not gained Expert Trainer"
        );
        for (level, expected) in [(4u8, 2i16), (7, 3), (20, 10)] {
            assert_eq!(
                super::cavalier_expert_trainer_bonus(level),
                expected,
                "level {level} Expert Trainer: level/2"
            );
        }
    }

    /// Both feat counts, across their real step boundaries.
    #[test]
    fn the_two_feat_counts_step_at_their_real_levels() {
        for (level, expected) in [(1u8, 0i16), (5, 0), (6, 1), (11, 1), (12, 2), (18, 3), (20, 3)] {
            assert_eq!(
                super::cavalier_bonus_combat_feat_count(level),
                expected,
                "level {level} bonus combat feats: level/6"
            );
        }
        for (level, expected) in [(1u8, 1i16), (8, 1), (9, 2), (16, 2), (17, 3), (20, 3)] {
            assert_eq!(
                super::cavalier_teamwork_feat_count(level),
                expected,
                "level {level} teamwork feats: Tactician 1st / 9th / 17th"
            );
        }
    }

    /// Order of the Sword grounds only when explicitly chosen, and
    /// clears the order-powers block when it is.
    #[test]
    fn order_of_the_sword_grounds_only_when_explicitly_recorded() {
        let bare = build_pilot_headless_receipt(&cavalier(1));
        assert!(
            bare.computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.cavalier.order_powers.unsupported"
                    && d.claim_blocking),
            "a Cavalier with no recorded Order stays blocked on it: {:?}",
            bare.computation.diagnostics
        );

        let mut sworn = cavalier(1);
        sworn.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: CAVALIER_ORDER_CHOICE_ID.to_owned(),
            selection_id: ORDER_OF_THE_SWORD_SELECTION.to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&sworn);
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.cavalier.order_powers.unsupported"),
            "a recognized Order clears the block: {:?}",
            receipt.computation.diagnostics
        );
        // 1/2 level, minimum +1: the floor is what applies at level 1.
        assert_eq!(
            value(&sworn, "class_feature.apg.cavalier.order_of_the_sword.sense_motive_bonus"),
            Some(1)
        );
        for (level, expected) in [(1u8, 1i16), (2, 1), (4, 2), (11, 5), (20, 10)] {
            assert_eq!(
                super::cavalier_order_of_the_sword_sense_motive_bonus(level),
                expected,
                "level {level}: max(1, level/2)"
            );
        }
    }

    /// SD-34 wave 44 (`decisions.md §22`, Piece 2 item 2): Order of the
    /// Dragon, the second Order grounded here, mirroring the Order of the
    /// Sword proof exactly.
    #[test]
    fn order_of_the_dragon_grounds_only_when_explicitly_recorded() {
        let bare = build_pilot_headless_receipt(&cavalier(1));
        assert!(
            bare.computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.cavalier.order_powers.unsupported"
                    && d.claim_blocking),
            "a Cavalier with no recorded Order stays blocked on it: {:?}",
            bare.computation.diagnostics
        );

        let mut sworn = cavalier(1);
        sworn.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: CAVALIER_ORDER_CHOICE_ID.to_owned(),
            selection_id: ORDER_OF_THE_DRAGON_SELECTION.to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&sworn);
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.cavalier.order_powers.unsupported"),
            "a recognized Order clears the block: {:?}",
            receipt.computation.diagnostics
        );
        // 1/2 level, minimum +1: the floor is what applies at level 1.
        assert_eq!(
            value(&sworn, "class_feature.apg.cavalier.order_of_the_dragon.survival_bonus"),
            Some(1)
        );
        for (level, expected) in [(1u8, 1i16), (2, 1), (4, 2), (11, 5), (20, 10)] {
            assert_eq!(
                super::cavalier_order_of_the_dragon_survival_bonus(level),
                expected,
                "level {level}: max(1, level/2)"
            );
        }
        // Order of the Dragon's OTHER two magnitudes (the challenge-target-
        // conditioned melee bonus, and Aid Allies' own ally-scoped bonus)
        // must NOT ground -- only the flat Survival bonus does.
        assert_eq!(
            value(&sworn, "class_feature.apg.cavalier.order_of_the_dragon.challenge_bonus"),
            None,
            "the opponent-conditioned melee bonus must stay deferred, never fabricated"
        );
    }

    /// An unrecognized Order selection must clear neither block -- only
    /// the two canonically grounded Orders do.
    #[test]
    fn an_unrecognized_order_selection_keeps_the_order_powers_block() {
        let mut input = cavalier(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: CAVALIER_ORDER_CHOICE_ID.to_owned(),
            selection_id: "order:lion".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.cavalier.order_powers.unsupported"
                    && d.claim_blocking),
            "Order of the Lion is not one of the two canonically grounded Orders: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Cavalier's real CSKILL list includes all three tracked skills.
    #[test]
    fn cavalier_earns_the_class_skill_bonus_on_all_three_tracked_skills() {
        let receipt = build_pilot_headless_receipt(&cavalier(1));
        for skill in ["climb", "intimidate", "swim"] {
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == format!("skill.selected_modifier.{skill}"))
                .unwrap_or_else(|| panic!("{skill} must be computed"));
            assert!(
                record.detail.contains("class-skill bonus (+3)"),
                "{skill} is on Cavalier's real CSKILL list: {record:?}"
            );
        }
    }

    /// No Cavalier diagnostic may still assert the class has no
    /// class-skill list -- it has one, and that text was false.
    #[test]
    fn no_cavalier_diagnostic_claims_the_class_has_no_class_skill_list() {
        for diagnostic in &build_pilot_headless_receipt(&cavalier(1)).computation.diagnostics {
            assert!(
                !diagnostic.message.contains("no class-skill list"),
                "Cavalier DOES have a class-skill list: {diagnostic:?}"
            );
        }
    }
}

/// v0.6 alpha swarm, task #14 (Swashbuckler, 2026-07-27): Finesse's
/// prerequisite substitution, Weapon Training, and the Bonus Feats
/// level-equivalence fact.
#[cfg(test)]
mod swashbuckler_finesse_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, SWASHBUCKLER_CLASS_ID,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn swashbuckler(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SWASHBUCKLER_CLASS_ID.to_owned(), level }];
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

    /// Finesse takes `max(CHASCORE, INTSCORE)` -- the max of two real
    /// ability SCORES, unlike Brawler's Cunning which floors INT at the
    /// constant 13. The shared helper must handle both without either
    /// class borrowing the other's operand.
    #[test]
    fn finesse_substitutes_the_higher_of_charisma_and_intelligence_scores() {
        for (charisma, intelligence, expected) in
            [(18, 10, 18), (10, 18, 18), (7, 7, 7), (13, 13, 13), (20, 3, 20)]
        {
            assert_eq!(
                super::effective_combat_feat_intelligence_score(charisma, intelligence),
                expected,
                "max(CHA {charisma}, INT {intelligence})"
            );
        }
    }

    /// Brawler's own Cunning must be unchanged by the generalization --
    /// it floors at the constant 13, which is NOT what Finesse does.
    #[test]
    fn brawler_cunning_still_floors_intelligence_at_thirteen() {
        for (intelligence, expected) in [(7, 13), (12, 13), (13, 13), (18, 18)] {
            assert_eq!(
                super::brawler_cunning_effective_intelligence_score(intelligence),
                expected,
                "Brawler floors INT {intelligence} at 13"
            );
        }
    }

    /// Finesse grounds unconditionally from level 1. Fixture CHA 8,
    /// INT 10 -> max is 10.
    #[test]
    fn finesse_grounds_the_effective_score_from_first_level() {
        assert_eq!(
            value(
                &swashbuckler(1),
                "class_feature.acg.swashbuckler.finesse_effective_intelligence_for_combat_feats"
            ),
            Some(10),
            "fixture CHA 8 / INT 10 -> effective 10"
        );
    }

    /// Weapon Training is `(level-1)/4`: +1 at 5th, +2 at 9th, +3 at
    /// 13th, +4 at 17th, and genuinely 0 below 5th.
    #[test]
    fn weapon_training_bonus_matches_the_corpus_formula_at_every_level() {
        for (level, expected) in [
            (1u8, 0i16),
            (4, 0),
            (5, 1),
            (8, 1),
            (9, 2),
            (12, 2),
            (13, 3),
            (16, 3),
            (17, 4),
            (20, 4),
        ] {
            assert_eq!(
                super::swashbuckler_weapon_training_bonus(level),
                expected,
                "level {level}: (level - 1)/4"
            );
        }
    }

    /// Swashbuckler levels count as fighter levels for feat
    /// qualification -- the fact grounds, the prereq wiring stays
    /// deferred (the Martial Training ruling).
    #[test]
    fn bonus_feats_ground_the_fighter_level_equivalence_fact() {
        assert_eq!(
            value(
                &swashbuckler(1),
                "class_feature.acg.swashbuckler.fighter_level_equivalence_for_feats"
            ),
            Some(1)
        );
    }

    /// None of the three leaks onto a non-Swashbuckler.
    #[test]
    fn none_of_the_three_grounds_for_a_non_swashbuckler() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let fighter = result.character_input.expect("valid fixture");
        for id in [
            "class_feature.acg.swashbuckler.finesse_effective_intelligence_for_combat_feats",
            "class_feature.acg.swashbuckler.weapon_training_bonus",
            "class_feature.acg.swashbuckler.fighter_level_equivalence_for_feats",
        ] {
            assert_eq!(value(&fighter, id), None, "{id} must not ground for a Fighter");
        }
    }

    /// The six deeds carrying real magnitudes, each at its own corpus
    /// tier. Tier gating uses `SwashbucklerLVL` rather than the corpus's
    /// own `SwashbucklerDeedQualifyLVL`, which is never populated for
    /// this class -- see `swashbuckler_deed_tier_reached` for the full
    /// reasoning and the lead's ruling.
    #[test]
    fn deed_tiers_gate_on_swashbuckler_level_because_the_corpus_gate_is_unpopulated() {
        for (tier, first_level) in [(1u8, 1u8), (3, 3), (7, 7), (11, 11), (15, 15), (19, 19)] {
            assert!(
                !super::swashbuckler_deed_tier_reached(first_level - 1, tier)
                    || first_level == 1,
                "tier {tier} must not be reached below level {first_level}"
            );
            assert!(
                super::swashbuckler_deed_tier_reached(first_level, tier),
                "tier {tier} is reached at level {first_level}"
            );
            assert!(super::swashbuckler_deed_tier_reached(20, tier), "tier {tier} at level 20");
        }
    }

    /// Tier-1 deeds: Derring-Do's uses equal the Dexterity MODIFIER and
    /// Dodging Panache's dodge bonus equals the Charisma modifier. The
    /// fixture's Charisma 8 gives a genuinely negative -1, which is
    /// reported honestly rather than floored.
    #[test]
    fn tier_one_deeds_ground_their_real_ability_derived_magnitudes() {
        let receipt = build_pilot_headless_receipt(&swashbuckler(1));
        let find = |id: &str| {
            receipt.computation.explanations.iter().find(|e| e.id == id).map(|e| e.value)
        };
        // Fixture DEX 14 -> +2, CHA 8 -> -1.
        assert_eq!(find("class_feature.acg.swashbuckler.derring_do_uses_per_day"), Some(2));
        assert_eq!(find("class_feature.acg.swashbuckler.dodging_panache_dodge_bonus"), Some(-1));
    }

    /// Precise Strike is a weapon-damage magnitude equal to swashbuckler
    /// level, doubled on a critical -- the Bomb/Sacred-Weapon idiom.
    /// Tier 3, so absent at levels 1-2.
    #[test]
    fn precise_strike_grounds_from_tier_three_and_doubles_on_a_critical() {
        for level in [1u8, 2] {
            let receipt = build_pilot_headless_receipt(&swashbuckler(level));
            assert!(
                !receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.contains("precise_strike")),
                "level {level} is below Precise Strike's tier"
            );
        }
        for level in [3u8, 10, 20] {
            let receipt = build_pilot_headless_receipt(&swashbuckler(level));
            let find = |id: &str| {
                receipt.computation.explanations.iter().find(|e| e.id == id).map(|e| e.value)
            };
            assert_eq!(
                find("class_feature.acg.swashbuckler.precise_strike_damage"),
                Some(i16::from(level))
            );
            assert_eq!(
                find("class_feature.acg.swashbuckler.precise_strike_critical_damage"),
                Some(2 * i16::from(level))
            );
        }
    }

    /// The two tier-19 stab DCs share a byte-identical formula
    /// (`level/2 + 10 + DEX`), and Bleeding Wound arrives at tier 11.
    #[test]
    fn high_tier_deeds_ground_at_their_own_gates() {
        let mid = build_pilot_headless_receipt(&swashbuckler(11));
        assert_eq!(
            mid.computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.swashbuckler.bleeding_wound_damage")
                .map(|e| e.value),
            Some(2),
            "Bleeding Wound bleeds the Dexterity modifier"
        );
        assert!(
            !mid.computation.explanations.iter().any(|e| e.id.contains("stab")),
            "the stab deeds are tier 19, not 11"
        );

        let top = build_pilot_headless_receipt(&swashbuckler(19));
        // 19/2 + 10 + 2 = 9 + 12 = 21.
        for id in [
            "class_feature.acg.swashbuckler.deadly_stab_dc",
            "class_feature.acg.swashbuckler.stunning_stab_dc",
        ] {
            assert_eq!(
                top.computation.explanations.iter().find(|e| e.id == id).map(|e| e.value),
                Some(21),
                "{id}: level/2 + 10 + DEX"
            );
        }
        assert_eq!(super::swashbuckler_stab_save_dc(19, 2), 21);
        assert_eq!(super::swashbuckler_stab_save_dc(20, 0), 20);
    }

    /// No deed leaks onto a non-Swashbuckler.
    #[test]
    fn deeds_never_ground_for_a_non_swashbuckler() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let fighter = result.character_input.expect("valid fixture");
        let receipt = build_pilot_headless_receipt(&fighter);
        for fragment in ["derring_do", "dodging_panache", "precise_strike", "bleeding_wound", "stab"] {
            assert!(
                !receipt.computation.explanations.iter().any(|e| e.id.contains(fragment)),
                "a Fighter must not ground {fragment}"
            );
        }
    }
}

