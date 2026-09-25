#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm, risks item 8 (Inquisitor Judgment closure, third
/// APG class-specific closure): APG Inquisitor's Judgment, verified
/// directly against `apg_abilities_class.lst`'s own `Inquisitor ~
/// Judgment`/`Judgment (Sacred)`/`Judgment (Profane)`/`Judgment / Justice`
/// entries. Combines the activation-gating pattern (Barbarian/Skald/
/// Bloodrager Rage-shaped mechanics) with the choice-recognition pattern
/// (Cleric's domain choice / Alchemist's mutagen-stat choice) -- the same
/// combination Alchemist's Mutagen already proved, not a new architecture.
/// Justice was the first canonical judgment type this closure grounded;
/// Protection, Purity, and Smiting were added in a later deepening pass
/// (2026-07-26, task #3) after re-verifying the original scoping doc's
/// "opponent/effect-dependent" exclusion of all 7 remaining types against
/// the raw corpus and finding it wrong for these three -- each is a flat,
/// unconditional self-buff with a real live consumer (see each
/// `INQUISITOR_JUDGMENT_*_SELECTION_ID` constant's own doc comment).
///
/// **All 9 real judgment types are now grounded (task #47, 2026-07-28,
/// Sacred/Profane Judgment widening).** The remaining 5 (Destruction,
/// Healing, Piercing, Resiliency, Resistance) were previously deferred
/// under the theory that each "needs a total this codebase has nowhere
/// to compute" -- but that was the exact over-strict bar task #18 already
/// corrected for Monster Lore/Cunning Initiative/Track, and Smiting's own
/// closure had already proven the fix: a live numeric total to layer onto
/// was never actually required, only a genuinely verified magnitude,
/// grounded as a standalone explanation record when no total exists.
/// Re-checked directly against `apg_abilities_class.lst`: every one of
/// the 5 carries a real `BONUS:VAR` formula off `InquisitorLVL` (no
/// opponent- or target-dependence) --
/// `KEY:Sacred Judgment ~ Destruction`/`KEY:Profane Judgment ~
/// Destruction` (`1+InqJudgeDestructionLVL/3`, weapon damage rolls),
/// `KEY:Sacred Judgment ~ Piercing`/`KEY:Profane Judgment ~ Piercing`
/// (`1+InqJudgePiercingLVL/3`, concentration/CL-vs-SR checks),
/// `KEY:Judgment ~ Healing` (`1+InqJudgeHealingLVL/3`, fast healing),
/// `KEY:Judgment ~ Resiliency` (`1+InqJudgeResiliencyLVL/5`, DR), and
/// `KEY:Judgment ~ Resistance` (`2*(1+floor(InqJudgeResistanceLVL/3))`,
/// energy resistance) -- so each grounds the same "standalone fact, no
/// live consumer" way Smiting already does, just with a real numeric
/// value instead of Smiting's flat boolean one. `Sacred Judgment`/
/// `Profane Judgment` are the two alignment-gated variants of the SAME
/// base Judgment ability (`!PREALIGN:LE,NE,CE` / `!PREALIGN:LG,NG,CG`),
/// not a second, additive ability -- every non-true-neutral inquisitor
/// legally has access to exactly one of the two, and (per Justice's own
/// established precedent) the numeric magnitude is identical either way,
/// only the flavor name (sacred vs. profane) differs, so this closure
/// does not attempt to pick the flavor correctly per-alignment for these
/// five either.
pub(super) const INQUISITOR_CLASS_ID: &str = "class:inquisitor";

/// `ClassAbilityActivation.ability_id` for Inquisitor Judgment. Unlike
/// Mutagen, Judgment DOES have a real per-day use budget (see
/// `inquisitor_judgment_uses_per_day`), genuinely enforced via
/// `activation.rounds_consumed_today` -- mirroring Rage's/Bloodrage's own
/// enforced rounds-per-day budget exactly (an earlier draft of this
/// closure incorrectly claimed this budget was informational-only; caught
/// in review 2026-07-25 against the actual Rage/Bloodrage code, which
/// does enforce theirs, and fixed before commit).
pub(super) const INQUISITOR_JUDGMENT_ABILITY_ID: &str = "judgment";

/// The choice set for which judgment type is currently pronounced. A
/// swift action can change this mid-combat per the corpus text, but this
/// codebase computes one deterministic snapshot, not a turn sequence, so
/// only the currently-selected type matters here -- the same "one
/// snapshot, not a turn-by-turn simulation" scope every activation-gated
/// closure this session already assumes.
pub(super) const INQUISITOR_JUDGMENT_CHOICE_ID: &str = "choice:inquisitor_judgment";

/// Task #64: the Inquisitor's own domain-choice seam, established this cycle
/// following the exact CLERIC_DOMAIN_CHOICE_ID pattern (PF1 Advanced Player's Guide
/// Domain: "An inquisitor can select one domain from among those belonging to her
/// deity," the same deity-restricted choice shape Cleric already has and this
/// codebase already leaves unmodeled -- the choice is surfaced as given, not
/// deity-validated). Verified independently against two primary sources
/// (d20pfsrd's own Inquisitor class page, quoted verbatim) before writing any code,
/// correcting a stale, INCORRECT claim this codebase's own
/// `push_inquisitor_other_features_deferred_diagnostic` doc comment previously made
/// ("no domain power exists for Inquisitor per the corpus"): the real PF1 rule is
/// the OPPOSITE of that claim. d20pfsrd's Inquisitor "Domain (or Inquisition)"
/// feature reads, verbatim: "An inquisitor does not gain the bonus spells listed
/// for each domain, nor does she gain bonus spell slots" (spells: NO) and "Each
/// domain grants a number of domain powers, depending on the level of the
/// inquisitor" (powers: YES) -- i.e. an Inquisitor's domain grants ONLY that
/// domain's powers, never its spells, the exact mirror image of what the
/// now-corrected comment claimed.
pub(super) const INQUISITOR_DOMAIN_CHOICE_ID: &str = "choice:inquisitor_domain";

/// PF1 Advanced Player's Guide Judgment (Justice): "granting a +X sacred
/// [or profane] bonus on all attack rolls" -- picked as the one canonical
/// judgment type this closure grounds because it is the only one of the 8
/// whose bonus is a pure numeric attack-roll bonus with no new state this
/// codebase would need to introduce (Destruction/damage rolls has no
/// damage-roll total anywhere in this codebase to layer onto; Healing/
/// Resiliency/Resistance/Protection need fast-healing, damage-reduction,
/// or energy-resistance state that doesn't exist yet; Piercing/Purity/
/// Smiting are opponent- or effect-type-dependent). Justice's own bonus
/// number is identical whether the inquisitor's alignment grants the
/// Sacred or Profane variant (`1+InqJudgeJusticeLVL/5` either way, per the
/// corpus's own two DESC blocks), so no alignment branching is needed for
/// the VALUE -- only the flavor name differs, which this closure does not
/// attempt to pick correctly per-alignment (see
/// `inquisitor_justice_judgment_attack_bonus`'s own doc comment).
pub(super) const INQUISITOR_JUDGMENT_JUSTICE_SELECTION_ID: &str = "judgment:justice";

/// PF1 Advanced Player's Guide Judgment (Protection): "+X sacred [or
/// profane] bonus to Armor Class" -- verified directly against
/// `apg_abilities_class.lst`'s own `KEY:Sacred Judgment ~ Protection`
/// DESC/BONUS:VAR records (`1+InqJudgeProtectionLVL/5`, identical shape
/// to Justice's own formula). A genuine scoping correction (2026-07-26):
/// this closure's original doc had grouped Protection with the
/// opponent/effect-dependent judgments, but it is actually a flat,
/// unconditional self-buff with a real live consumer already computed in
/// this codebase (`baseline_armor_class` in `compute_combat_baseline`,
/// the same total Dodge/Brawler's AC Bonus/Alchemist's Mutagen already
/// layer onto) -- so it grounds cleanly, the same shape as Justice.
pub(super) const INQUISITOR_JUDGMENT_PROTECTION_SELECTION_ID: &str = "judgment:protection";

/// PF1 Advanced Player's Guide Judgment (Purity): "+X sacred [or profane]
/// bonus on all saving throws" -- verified directly against
/// `apg_abilities_class.lst`'s own `KEY:Sacred Judgment ~ Purity`
/// DESC/BONUS:VAR records (`1+InqJudgePurityLVL/5`, identical shape to
/// Justice's/Protection's own formula). The same scoping correction as
/// Protection above: a flat, unconditional bonus to all three saves,
/// landing on the real `total_saves` total (`compute_total_saves`, the
/// same total `feat_save_bonuses`/Cleric's Touch of Good/the Rage family
/// already layer onto).
pub(super) const INQUISITOR_JUDGMENT_PURITY_SELECTION_ID: &str = "judgment:purity";

/// PF1 Advanced Player's Guide Judgment (Smiting): "the inquisitor's
/// weapons count as magic for the purposes of bypassing damage
/// reduction" -- verified directly against `apg_abilities_class.lst`'s
/// own `KEY:Judgment ~ Smiting` DESC/BONUS:VAR records
/// (`BONUS:VAR|InqJudgeSmitingLVL|InquisitorLVL`, no division/scaling at
/// all -- a flat, level-independent boolean fact, not a numeric bonus).
/// No numeric total exists anywhere in this codebase to layer this onto,
/// so it grounds as a standalone explanation record only, mirroring
/// `brawler_strike_progression_tier`'s own DR-bypass fact (tier 1: "her
/// weapon attacks count as magic for the purposes of overcoming DR") --
/// the closest existing precedent for this exact shape.
pub(super) const INQUISITOR_JUDGMENT_SMITING_SELECTION_ID: &str = "judgment:smiting";

/// PF1 Advanced Player's Guide Judgment (Destruction): "gaining a +X
/// sacred [or profane] bonus on all weapon damage rolls" -- verified
/// directly against `apg_abilities_class.lst`'s own `KEY:Sacred Judgment
/// ~ Destruction`/`KEY:Profane Judgment ~ Destruction` DESC/BONUS:VAR
/// records (`1+InqJudgeDestructionLVL/3`, identical either flavor). No
/// weapon-damage-roll total exists anywhere in this codebase to layer
/// this onto, so it grounds as a standalone explanation record, the same
/// shape as Smiting (task #47, 2026-07-28).
pub(super) const INQUISITOR_JUDGMENT_DESTRUCTION_SELECTION_ID: &str = "judgment:destruction";

/// PF1 Advanced Player's Guide Judgment (Healing): "gaining fast healing
/// %1" -- verified directly against `apg_abilities_class.lst`'s own
/// `KEY:Judgment ~ Healing` DESC/BONUS:VAR record
/// (`1+InqJudgeHealingLVL/3`, a base type with no Sacred/Profane split).
/// No fast-healing / round-tick hit-point state exists anywhere in this
/// codebase, so it grounds as a standalone explanation record, the same
/// shape as Smiting (task #47, 2026-07-28).
pub(super) const INQUISITOR_JUDGMENT_HEALING_SELECTION_ID: &str = "judgment:healing";

/// PF1 Advanced Player's Guide Judgment (Piercing): "a +X sacred [or
/// profane] bonus on concentration checks and caster level checks made
/// to overcome a target's spell resistance" -- verified directly against
/// `apg_abilities_class.lst`'s own `KEY:Sacred Judgment ~ Piercing`/
/// `KEY:Profane Judgment ~ Piercing` DESC/BONUS:VAR records
/// (`1+InqJudgePiercingLVL/3`, identical either flavor). No concentration-
/// check or caster-level-vs-SR-check total exists anywhere in this
/// codebase, so it grounds as a standalone explanation record, the same
/// shape as Smiting (task #47, 2026-07-28).
pub(super) const INQUISITOR_JUDGMENT_PIERCING_SELECTION_ID: &str = "judgment:piercing";

/// PF1 Advanced Player's Guide Judgment (Resiliency): "granting DR %1/
/// magic" (or, from level 10 on, DR against the alignment type opposite
/// the inquisitor's own) -- verified directly against
/// `apg_abilities_class.lst`'s own `KEY:Judgment ~ Resiliency` DESC/
/// BONUS:VAR record (`1+InqJudgeResiliencyLVL/5`, a base type with no
/// Sacred/Profane split). This is damage reduction the inquisitor
/// RECEIVES, a defensive facet distinct from Smiting's own DR-bypass-on-
/// attack -- no DR-received total exists anywhere in this codebase, so
/// it grounds as a standalone explanation record, the same shape as
/// Smiting (task #47, 2026-07-28). This grounds only the flat numeric DR
/// value; the level-10 bypass-type switch (magic vs. opposed alignment)
/// is named in the detail text but not separately modeled, mirroring how
/// Justice/Protection/Purity don't model which of Sacred/Profane an
/// inquisitor's own alignment grants.
pub(super) const INQUISITOR_JUDGMENT_RESILIENCY_SELECTION_ID: &str = "judgment:resiliency";

/// PF1 Advanced Player's Guide Judgment (Resistance): "%1 points of
/// energy resistance against one energy type ... chosen when the
/// judgment is declared" -- verified directly against
/// `apg_abilities_class.lst`'s own `KEY:Judgment ~ Resistance` DESC/
/// BONUS:VAR record (`2*(1+floor(InqJudgeResistanceLVL/3))`, a base type
/// with no Sacred/Profane split). The chosen energy TYPE does not affect
/// the numeric magnitude (mirrors Justice/Protection/Purity not modeling
/// which of Sacred/Profane an inquisitor's own alignment grants), and no
/// energy-resistance total exists anywhere in this codebase, so it
/// grounds as a standalone explanation record, the same shape as Smiting
/// (task #47, 2026-07-28).
pub(super) const INQUISITOR_JUDGMENT_RESISTANCE_SELECTION_ID: &str = "judgment:resistance";

/// Inquisitor's own known-spell posture (task #47, 2026-07-28): verified
/// directly against `apg_classes.lst`'s own `CLASS:Inquisitor` record --
/// `SPELLSTAT:WIS MEMORIZE:NO`, the same spontaneous-divine posture as
/// Sorcerer/Bard/Oracle, so this closure mirrors
/// `unmet_oracle_known_spell_conditions`/`oracle_spells_known_table`/
/// `ground_oracle_known_spells`'s own shape, not Wizard's/Arcanist's/
/// Warpriest's own simpler prepared-spellbook shape. Unlike Oracle
/// (`SPELLLIST:2|Cleric|Oracle` reuses Cleric's own list) and Hunter
/// (reuses Ranger's), the real `CLASS:Inquisitor` record carries no
/// `SPELLLIST:` token at all -- its spell list is independently tagged
/// per-spell (`CLASSES:...Inquisitor=N` across `apg_spells.lst`/
/// `acg_spells.lst`), so `rules_tables::apg::inquisitor_spell_list` had
/// to be built fresh rather than reusing an existing module (see that
/// module's own doc comment for the corpus derivation: 219 real spells,
/// levels 0-6, split 15/38/43/44/35/24/20).
///
/// Unlike Oracle's own bounded 1-3 MVP (`ORACLE_KNOWN_SPELLS_SUPPORTED_MAX_LEVEL`),
/// this table transcribes the FULL real level 1-20 `KNOWN:` row directly
/// off `apg_classes.lst`'s own `CLASS:Inquisitor` per-level block (each
/// row's `N+InquisitorKnownHack` term, with the corpus's own
/// `DEFINE:InquisitorKnownHack|0` default -- no variant-rule override
/// applied) -- there was no reason to arbitrarily bound the scope when
/// the complete verified table was directly available.
pub(super) const INQUISITOR_KNOWN_SPELLS_MAX_LEVEL: u8 = 20;

/// v0.6 alpha swarm, risks item 8 (Warpriest full-build closure, sixth
/// ACG/APG class-specific closure): APG/ACG Warpriest, verified directly
/// against `acg_classes.lst`'s own `SPELLLIST:1|Cleric`/Level-progression
/// `CAST:` rows and `acg_abilities_class.lst`'s own `KEY:Warpriest ~
/// Blessings`/`Sacred Weapon`/`Class Skills` records. Reuses Cleric's own
/// spell-list content and prepared-casting shape (no `MEMORIZE:NO`), but
/// -- confirmed via two independent checks, the same discipline the
/// Arcanist closure's own correction established -- Warpriest's real
/// per-level spells-per-day table matches Cleric's exactly at levels 1-2
/// but genuinely diverges from level 3 on (Cleric grants a 2nd-level slot
/// at level 3; Warpriest not until level 4, and at a lower count), so
/// this needs its own real, independently-verified table, not a
/// byte-identical reuse. Blessings (Warpriest's own domain-equivalent, a
/// choice of 2 from ~20 types matching Cleric's own domain list) gets the
/// same "pick ONE canonical, self-scoped-only option" narrowing Cleric's
/// own domain closure used (Good domain / Touch of Good) -- Destruction
/// Blessing's own Minor power (Destructive Attacks: touch an ally, grant
/// a flat morale bonus) is structurally identical to Touch of Good, so
/// it is the one Blessing this closure grounds. Also fixes a real,
/// independently-confirmed bug (not just a verification): Warpriest's own
/// class-skill list genuinely includes Climb/Intimidate/Swim (unlike
/// Wizard/Arcanist, whose lists include none of the three), so
/// `selected_skill_class_skill_bonus_applies` needed real widening, not
/// just a "verify it already answers correctly" check -- the mirror-image
/// of the original Wizard class-skill-modifier bug (false positive there,
/// false negative here). See
/// `docs/release/v0.6/warpriest-acg-full-build-scoping.md` for the full
/// corpus verification and scope record.
pub(super) const WARPRIEST_CLASS_ID: &str = "class:warpriest";

/// Mirrors `WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL`/
/// `ARCANIST_SPELLBOOK_SUPPORTED_MAX_LEVEL` exactly, including their v0.6
/// widening: this table originally verified Warpriest's own spells-per-day
/// table for levels 1-3 only. The full 1-20 table is now transcribed from
/// the corpus (see `warpriest_base_spells_per_day`), so no legal class
/// level is refused for lack of a verified row any more.
pub(super) const WARPRIEST_SPELLBOOK_SUPPORTED_MAX_LEVEL: u8 = 20;

/// The choice set for which 2 Blessings a Warpriest selects at 1st level
/// (mirrors `CLERIC_DOMAIN_CHOICE_ID`'s own shape exactly -- a Vec of
/// selections under one choice-set id, not a single value).
pub(super) const WARPRIEST_BLESSING_CHOICE_ID: &str = "choice:warpriest_blessing";

/// `ClassAbilityActivation.ability_id` for Destruction Blessing's own
/// Destructive Attacks minor power.
pub(super) const WARPRIEST_DESTRUCTIVE_ATTACKS_ABILITY_ID: &str = "destructive_attacks";

/// `ClassAbilityActivation.ability_id` for Strength Blessing's own
/// Strength Surge minor power.
pub(super) const WARPRIEST_STRENGTH_SURGE_ABILITY_ID: &str = "strength_surge";

/// Warpriest level at which Fervor is granted (`acg_classes.lst`'s own
/// per-level grant row: `2 ABILITY:Warpriest Class Feature|AUTOMATIC|
/// Warpriest ~ Fervor`).
pub(super) const WARPRIEST_FERVOR_LEVEL: u8 = 2;

/// Warpriest level at which Channel Energy is granted (`acg_classes.lst`:
/// `4 ABILITY:...|Warpriest ~ Channel Energy`).
pub(super) const WARPRIEST_CHANNEL_ENERGY_LEVEL: u8 = 4;

/// Warpriest level at which Sacred Armor is granted (`acg_classes.lst`:
/// `7 ABILITY:...|Warpriest ~ Sacred Armor`).
pub(super) const WARPRIEST_SACRED_ARMOR_LEVEL: u8 = 7;

/// Fervor uses consumed by one Channel Energy, verified directly against
/// `acg_abilities_class.lst`'s own `KEY:Warpriest ~ Channel Energy` DESC
/// ("Using this ability is a standard action that expends two uses of his
/// fervor ability") and restated on both display records ("This consumes
/// 2 uses of your Fervor Ability").
pub(super) const WARPRIEST_CHANNEL_ENERGY_FERVOR_COST: i16 = 2;

/// The one canonical spell this codebase seeds into a Warpriest's
/// spellbook at creation time. `Light` is a real 0-level entry of
/// `SPELL_LIST` (`Pf1SchoolId::Evocation`, level 0) and a real Cleric
/// orison -- Warpriest casts from `SPELLLIST:1|Cleric`, so it is genuinely
/// on this class's own list. Deliberately the SAME id
/// `WIZARD_STARTER_SPELL_ID`/`ARCANIST_STARTER_SPELL_ID` already use, for
/// the same bootstrap reason.
/// `allow(dead_code)` for the same reason as `CANONICAL_EXTRACT_SPELL_ID`:
/// the production consumer lives in the separate `apps/desktop/src-tauri`
/// workspace, which mirrors its own copy.
#[allow(dead_code)]
pub(crate) const WARPRIEST_STARTER_SPELL_ID: &str = "Light";

/// v0.6 alpha swarm, risks item 8 (Inquisitor Judgment closure): whether
/// `input` is a single-class Inquisitor at a level within
/// `apg::class_chassis_resolve`'s declared ceiling for Inquisitor --
/// mirrors `is_supported_cavalier_single_class`/
/// `is_supported_alchemist_single_class` exactly, including the same
/// exact-match discipline (`== Some(ApgClassId::Inquisitor)`, not a broad
/// `.is_some()`).
pub(super) fn is_supported_inquisitor_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if ApgClassId::from_class_id_str(&class_level.class_id) != Some(ApgClassId::Inquisitor) {
        return false;
    }
    apg::class_chassis_resolve(ApgClassId::Inquisitor, class_level.level, RuleSetId::Apg).is_some()
}

/// v0.6 alpha swarm, risks item 8 (Warpriest full-build closure): whether
/// `input` is a single-class Warpriest at a level within
/// `acg::class_chassis_resolve`'s declared ceiling for Warpriest --
/// mirrors the other five ACG/APG exact-match gates exactly.
pub(super) fn is_supported_warpriest_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if AcgClassId::from_class_id_str(&class_level.class_id) != Some(AcgClassId::Warpriest) {
        return false;
    }
    acg::class_chassis_resolve(AcgClassId::Warpriest, class_level.level, RuleSetId::Acg).is_some()
}

/// PF1 Advanced Player's Guide Judgment: "an inquisitor can use this
/// ability 1 + 1 for every three inquisitor levels beyond 1st". Verified
/// directly against `apg_abilities_class.lst`'s own `BONUS:VAR|
/// InquisitorJudgmentTimes|1+(InquisitorLVL-1)/3`. Informational only --
/// see `INQUISITOR_JUDGMENT_ABILITY_ID`'s own doc comment for why this
/// codebase never enforces a per-day-use budget.
pub(super) fn inquisitor_judgment_uses_per_day(level: u8) -> i16 {
    1 + (i16::from(level) - 1) / 3
}

/// PF1 Advanced Player's Guide Judgment / Justice: "granting a +1 sacred
/// [or profane] bonus on all attack rolls" at 1st level, scaling per
/// `1+InqJudgeJusticeLVL/5` (verified directly against
/// `apg_abilities_class.lst`'s own two Judgment/Justice DESC blocks,
/// Sacred and Profane, which share this exact formula).
pub(super) fn inquisitor_justice_judgment_attack_bonus(level: u8) -> i16 {
    1 + i16::from(level) / 5
}

/// PF1 Advanced Player's Guide Judgment / Protection: "+X sacred [or
/// profane] bonus to Armor Class" -- identical shape/formula to Justice's
/// own attack-bonus formula, verified against
/// `apg_abilities_class.lst`'s own `1+InqJudgeProtectionLVL/5`.
pub(super) fn inquisitor_protection_judgment_ac_bonus(level: u8) -> i16 {
    1 + i16::from(level) / 5
}

/// PF1 Advanced Player's Guide Judgment / Purity: "+X sacred [or profane]
/// bonus on all saving throws" -- identical shape/formula to Justice's/
/// Protection's own formulas, verified against
/// `apg_abilities_class.lst`'s own `1+InqJudgePurityLVL/5`.
pub(super) fn inquisitor_purity_judgment_save_bonus(level: u8) -> i16 {
    1 + i16::from(level) / 5
}

/// PF1 Advanced Player's Guide Judgment / Destruction: "+X sacred [or
/// profane] bonus on all weapon damage rolls" -- verified against
/// `apg_abilities_class.lst`'s own `1+InqJudgeDestructionLVL/3` (task #47,
/// 2026-07-28).
pub(super) fn inquisitor_destruction_judgment_damage_bonus(level: u8) -> i16 {
    1 + i16::from(level) / 3
}

/// PF1 Advanced Player's Guide Judgment / Healing: fast healing X --
/// verified against `apg_abilities_class.lst`'s own
/// `1+InqJudgeHealingLVL/3` (task #47, 2026-07-28).
pub(super) fn inquisitor_healing_judgment_fast_healing(level: u8) -> i16 {
    1 + i16::from(level) / 3
}

/// PF1 Advanced Player's Guide Judgment / Piercing: "+X sacred [or
/// profane] bonus on concentration checks and caster level checks made
/// to overcome a target's spell resistance" -- verified against
/// `apg_abilities_class.lst`'s own `1+InqJudgePiercingLVL/3` (task #47,
/// 2026-07-28).
pub(super) fn inquisitor_piercing_judgment_bonus(level: u8) -> i16 {
    1 + i16::from(level) / 3
}

/// PF1 Advanced Player's Guide Judgment / Resiliency: damage reduction X
/// -- verified against `apg_abilities_class.lst`'s own
/// `1+InqJudgeResiliencyLVL/5` (task #47, 2026-07-28).
pub(super) fn inquisitor_resiliency_judgment_dr(level: u8) -> i16 {
    1 + i16::from(level) / 5
}

/// PF1 Advanced Player's Guide Judgment / Resistance: energy resistance X
/// -- verified against `apg_abilities_class.lst`'s own
/// `2*(1+floor(InqJudgeResistanceLVL/3))` (task #47, 2026-07-28).
pub(super) fn inquisitor_resistance_judgment_energy_resistance(level: u8) -> i16 {
    2 * (1 + i16::from(level) / 3)
}

/// Whether `input` is an Inquisitor actively, validly pronouncing the
/// Justice judgment (v0.6 alpha swarm, risks item 8, Inquisitor Judgment
/// closure) -- mirrors `active_touch_of_good_bonus`'s exact shape
/// (class ownership, then a recognized choice, then an active
/// class_ability_activations entry), the closest existing precedent for
/// a self-applied attack-roll bonus gated on both a choice and an
/// activation.
pub(super) fn active_inquisitor_justice_judgment_bonus(input: &CharacterInput) -> Option<(u8, i16)> {
    let inquisitor_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == INQUISITOR_CLASS_ID)
        .map(|class_level| class_level.level)?;

    let judgment_selection = choice_selection(input, INQUISITOR_JUDGMENT_CHOICE_ID);
    if judgment_selection != Some(INQUISITOR_JUDGMENT_JUSTICE_SELECTION_ID) {
        return None;
    }

    let activation = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == INQUISITOR_JUDGMENT_ABILITY_ID)?;
    if activation.active_state != ActiveState::EquippedActive {
        return None;
    }

    if let Some(uses_consumed_today) = activation.rounds_consumed_today {
        let uses_per_day = inquisitor_judgment_uses_per_day(inquisitor_level);
        if i32::from(uses_consumed_today) > i32::from(uses_per_day) {
            return None;
        }
    }

    Some((inquisitor_level, inquisitor_justice_judgment_attack_bonus(inquisitor_level)))
}

/// Whether `input` is an Inquisitor actively, validly pronouncing the
/// Protection judgment -- mirrors `active_inquisitor_justice_judgment_bonus`'s
/// exact shape, differing only in the recognized selection id and the
/// applied formula (Armor Class, not attack rolls).
pub(super) fn active_inquisitor_protection_judgment_bonus(input: &CharacterInput) -> Option<(u8, i16)> {
    let inquisitor_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == INQUISITOR_CLASS_ID)
        .map(|class_level| class_level.level)?;

    let judgment_selection = choice_selection(input, INQUISITOR_JUDGMENT_CHOICE_ID);
    if judgment_selection != Some(INQUISITOR_JUDGMENT_PROTECTION_SELECTION_ID) {
        return None;
    }

    let activation = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == INQUISITOR_JUDGMENT_ABILITY_ID)?;
    if activation.active_state != ActiveState::EquippedActive {
        return None;
    }

    if let Some(uses_consumed_today) = activation.rounds_consumed_today {
        let uses_per_day = inquisitor_judgment_uses_per_day(inquisitor_level);
        if i32::from(uses_consumed_today) > i32::from(uses_per_day) {
            return None;
        }
    }

    Some((inquisitor_level, inquisitor_protection_judgment_ac_bonus(inquisitor_level)))
}

/// Whether `input` is an Inquisitor actively, validly pronouncing the
/// Purity judgment -- mirrors `active_inquisitor_justice_judgment_bonus`'s
/// exact shape, differing only in the recognized selection id and the
/// applied formula (all saving throws, not attack rolls).
pub(super) fn active_inquisitor_purity_judgment_bonus(input: &CharacterInput) -> Option<(u8, i16)> {
    let inquisitor_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == INQUISITOR_CLASS_ID)
        .map(|class_level| class_level.level)?;

    let judgment_selection = choice_selection(input, INQUISITOR_JUDGMENT_CHOICE_ID);
    if judgment_selection != Some(INQUISITOR_JUDGMENT_PURITY_SELECTION_ID) {
        return None;
    }

    let activation = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == INQUISITOR_JUDGMENT_ABILITY_ID)?;
    if activation.active_state != ActiveState::EquippedActive {
        return None;
    }

    if let Some(uses_consumed_today) = activation.rounds_consumed_today {
        let uses_per_day = inquisitor_judgment_uses_per_day(inquisitor_level);
        if i32::from(uses_consumed_today) > i32::from(uses_per_day) {
            return None;
        }
    }

    Some((inquisitor_level, inquisitor_purity_judgment_save_bonus(inquisitor_level)))
}

/// PF1 Advanced Player's Guide Monster Lore: "adds her Wisdom modifier
/// on Knowledge skill checks in addition to her Intelligence modifier,
/// when making skill checks to identify the abilities and weaknesses of
/// creatures" -- verified directly against `apg_abilities_class.lst`'s
/// own `BONUS:VAR|MonsterLoreBonus|WIS` (a flat competence-style bonus
/// equal to the raw Wisdom modifier, no formula beyond that). Grounded
/// as a standalone explanation record only (task #18, 2026-07-26,
/// correcting an earlier over-strict "needs a live consumer" exclusion):
/// this codebase's own established precedent (Bard's Bardic Knowledge,
/// Slayer's Track/Trapfinding, Barbarian's Damage Reduction all already
/// ground a standalone flat fact with zero live consumer) shows a
/// consumer was never actually required, only a genuinely verified
/// magnitude. No Knowledge-skill total exists anywhere in this codebase,
/// so this grounds only the flat bonus value.
pub fn inquisitor_monster_lore_bonus(wisdom_modifier: i16) -> i16 {
    wisdom_modifier
}

/// PF1 Advanced Player's Guide Cunning Initiative: "adds her Wisdom
/// modifier on initiative checks, in addition to her Dexterity modifier"
/// -- verified directly against `apg_abilities_class.lst`'s own
/// `BONUS:COMBAT|INITIATIVE|WIS`. Grounded as a standalone explanation
/// record only (task #18, 2026-07-26), the same corrected shape as
/// Monster Lore above: no Initiative total exists anywhere in this
/// codebase (confirmed directly), so this grounds only the flat bonus
/// value.
pub fn inquisitor_cunning_initiative_bonus(wisdom_modifier: i16) -> i16 {
    wisdom_modifier
}

/// PF1 Advanced Player's Guide Track: "adds half her level on Survival
/// skill checks made to follow or identify tracks" -- verified directly
/// against `apg_abilities_class.lst`'s own `BONUS:VAR|TrackLVL|
/// InquisitorLVL` plus the shared cross-class `TrackBonus|
/// max(TrackLVL/2,1)` definition (the real PF1 Track feature Ranger/
/// Slayer also share; confirmed identical to Slayer's own
/// `slayer_track_bonus` formula, kept as a separate Inquisitor-named
/// copy rather than calling Slayer's function directly, the same
/// "parallel copy over cross-class-function-reuse" discipline Skald's
/// own spellcasting closure used for its base-spells-per-day table).
/// Grounded as a standalone explanation record only (task #18,
/// 2026-07-26): Survival is not among the three tracked skills either,
/// so this grounds as a standalone flat record, the same shape Slayer's
/// own Track uses.
pub(super) fn inquisitor_track_bonus(level: u8) -> i16 {
    (i16::from(level) / 2).max(1)
}

/// PF1 Advanced Player's Guide Bane: "This ability lasts for %1 rounds
/// per day" -- verified directly against `apg_abilities_class.lst`'s own
/// `KEY:Inquisitor ~ Bane` record: `BONUS:VAR|InquisitorBanePool|
/// InquisitorLVL`, a flat rounds/day pool with no division or scaling at
/// all (task #47, 2026-07-28). The same "pool size only" MVP shape as
/// Swashbuckler's own Panache (`swashbuckler_panache_max`) and Warpriest's
/// own Fervor uses/day: this grounds only the flat daily pool size --
/// which creature type (and subtype, for humanoid/outsider) is imbued,
/// and how the rounds are spent/tracked across a day, is not modeled.
pub fn inquisitor_bane_pool_rounds(level: u8) -> i16 {
    i16::from(level)
}

/// Grounds Inquisitor's remaining flat, standalone named-feature facts
/// (Monster Lore, Cunning Initiative, Track, Bane) that had incorrectly
/// stayed deferred under an earlier "needs a live consumer" bar --
/// corrected (task #18, 2026-07-26; Bane added task #47, 2026-07-28)
/// after re-checking this codebase's own already-shipped precedent.
/// Called unconditionally alongside Judgment's own grounding, from the
/// Inquisitor branch of `compute_apg_class_chassis`; never claim-blocks.
pub(super) fn ground_inquisitor_flat_named_facts(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let wisdom_modifier = ability_modifier(input.chosen.ability_scores.wisdom);

    let monster_lore_bonus = inquisitor_monster_lore_bonus(wisdom_modifier);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.inquisitor.monster_lore_bonus".to_owned(),
        value: monster_lore_bonus,
        detail: format!(
            "Inquisitor level {level} Monster Lore: adds her Wisdom modifier \
             ({monster_lore_bonus:+}) on Knowledge skill checks made to identify the abilities \
             and weaknesses of creatures, in addition to her Intelligence modifier. No \
             Knowledge-skill total exists anywhere in this codebase, so this grounds only the \
             flat bonus value -- not a full Knowledge-check resolution engine"
        ),
    });

    let cunning_initiative_bonus = inquisitor_cunning_initiative_bonus(wisdom_modifier);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.inquisitor.cunning_initiative_bonus".to_owned(),
        value: cunning_initiative_bonus,
        detail: format!(
            "Inquisitor level {level} Cunning Initiative: adds her Wisdom modifier \
             ({cunning_initiative_bonus:+}) on initiative checks, in addition to her Dexterity \
             modifier. No Initiative total exists anywhere in this codebase, so this grounds \
             only the flat bonus value"
        ),
    });

    let track_bonus = inquisitor_track_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.inquisitor.track_bonus".to_owned(),
        value: track_bonus,
        detail: format!(
            "Inquisitor level {level} Track: a +{track_bonus} bonus on Survival checks made to \
             follow or identify tracks (max(level/2, 1) = {track_bonus}). Survival is not among \
             the three tracked skills either, so this grounds as a standalone flat record"
        ),
    });

    let bane_pool_rounds = inquisitor_bane_pool_rounds(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.apg.inquisitor.bane_pool_rounds".to_owned(),
        value: bane_pool_rounds,
        detail: format!(
            "Inquisitor level {level} Bane: a {bane_pool_rounds}-round-per-day pool \
             (InquisitorLVL, no scaling) during which one wielded weapon can be imbued with the \
             bane weapon special ability against a chosen creature type (and subtype, for \
             humanoid or outsider). This grounds only the flat daily pool size; which creature \
             type is imbued and how rounds are spent/tracked across a day is not modeled"
        ),
    });
}

/// PF1 Advanced Player's Guide Stern Gaze: "+X morale bonus on all
/// Intimidate and Sense Motive checks" -- verified directly against
/// `apg_abilities_class.lst`'s own `BONUS:VAR|SternGazeBonus|
/// max(1,InquisitorLVL/2)` formula. A real, unconditional level-1 class
/// feature (see `apg_classes.lst`'s own level-feature table: granted
/// automatically at 1st level, no choice or activation gate unlike
/// Judgment) -- so this stacks with any Judgment state, not gated on it.
pub(super) fn inquisitor_stern_gaze_intimidate_bonus(level: u8) -> i16 {
    1.max(i16::from(level) / 2)
}

/// Whether `input` is an Inquisitor at all (Stern Gaze is unconditional
/// once the class is present, unlike Judgment) -- only the Intimidate
/// half of Stern Gaze's real two-skill bonus is grounded here; Sense
/// Motive is not a skill this codebase computes anywhere, the same
/// "ground only the computed half" discipline `feat_effects::
/// skill_bonuses_from_feats` already applied to Persuasive's own
/// Diplomacy half.
pub(super) fn active_inquisitor_stern_gaze_bonus(input: &CharacterInput) -> Option<i16> {
    let inquisitor_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == INQUISITOR_CLASS_ID)
        .map(|class_level| class_level.level)?;

    Some(inquisitor_stern_gaze_intimidate_bonus(inquisitor_level))
}

/// Grounds or claim-blocks Inquisitor's Judgment execution engine for
/// `inquisitor_level` (v0.6 alpha swarm, risks item 8, Inquisitor
/// Judgment closure). Called from `compute_apg_class_chassis`'s
/// Inquisitor branch, gated only on Inquisitor class-ownership.
///
/// A character who simply isn't currently pronouncing judgment (no
/// `class_ability_activations` entry for `INQUISITOR_JUDGMENT_ABILITY_ID`,
/// or one present but `active_state != EquippedActive`) is a genuinely
/// valid PF1 posture -- not every Inquisitor is always judging -- so this
/// grounds a real "not judging" recognition record rather than claim-
/// blocking, mirroring "not mutated"/"not raging" from every other
/// activation-gated closure this session. An activation that IS active
/// but names no recognized `choice:inquisitor_judgment` selection (i.e.
/// none of Justice/Protection/Purity/Smiting, or one of the remaining 5
/// judgment types this closure does not ground) is a genuine posture
/// violation -- pronouncing judgment always requires choosing a type
/// first per the corpus's own sequencing -- and claim-blocks, mirroring
/// Alchemist's own "active but no recognized stat choice" shape. An
/// activation whose `rounds_consumed_today` exceeds the
/// grounded uses-per-day budget also claim-blocks, mirroring Rage's/
/// Bloodrage's own genuinely-enforced over-budget check exactly (caught
/// in review 2026-07-25: an earlier version of this closure's own doc
/// comment incorrectly claimed the budget was informational-only, citing
/// Rage/Bloodrage as precedent for that claim -- but both of those
/// actually DO enforce their budget with a real over-budget diagnostic,
/// so this closure was missing the same check, not deliberately scoping
/// it out).
pub(super) fn ground_or_block_inquisitor_judgment(
    input: &CharacterInput,
    inquisitor_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(activation) = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == INQUISITOR_JUDGMENT_ABILITY_ID)
    else {
        explanations.push(ComputationExplanation {
            id: "class_feature.apg.inquisitor.judgment_execution.not_judging".to_owned(),
            value: 0,
            detail: format!(
                "Inquisitor level {inquisitor_level} is not currently pronouncing judgment (no \
                 class_ability_activations entry for \
                 \"{INQUISITOR_JUDGMENT_ABILITY_ID}\"): a genuinely valid PF1 posture, so no \
                 attack-roll bonus is claimed. This grounds the Judgment execution engine's \
                 \"inactive\" branch only; being actively judging is grounded separately below \
                 when an active activation with the recognized Justice choice is present"
            ),
        });
        push_inquisitor_other_features_deferred_diagnostic(input, diagnostics);
        return;
    };

    let uses_per_day = inquisitor_judgment_uses_per_day(inquisitor_level);
    if let Some(uses_consumed_today) = activation.rounds_consumed_today
        && i32::from(uses_consumed_today) > i32::from(uses_per_day) {
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.apg.inquisitor.judgment_execution.uses_exceeded".to_owned(),
                message: format!(
                    "Inquisitor level {inquisitor_level} Judgment activation claims \
                     {uses_consumed_today} uses consumed today, exceeding the grounded \
                     uses-per-day budget of {uses_per_day} (1 + (level-1)/3): a genuine posture \
                     violation, so no attack-roll bonus is claimed for this input"
                ),
                claim_blocking: true,
            });
            push_inquisitor_other_features_deferred_diagnostic(input, diagnostics);
            return;
        }

    match activation.active_state {
        ActiveState::EquippedActive => {
            let judgment_selection = choice_selection(input, INQUISITOR_JUDGMENT_CHOICE_ID);
            let uses_consumed_today = activation.rounds_consumed_today.unwrap_or(0);

            if judgment_selection == Some(INQUISITOR_JUDGMENT_JUSTICE_SELECTION_ID) {
                let attack_bonus = inquisitor_justice_judgment_attack_bonus(inquisitor_level);
                explanations.push(ComputationExplanation {
                    id: "class_feature.apg.inquisitor.judgment_execution.active".to_owned(),
                    value: attack_bonus,
                    detail: format!(
                        "Inquisitor level {inquisitor_level} is actively pronouncing the Justice \
                         judgment, within the grounded uses-per-day budget ({uses_per_day} uses; \
                         {uses_consumed_today} consumed today), granting a +{attack_bonus} sacred \
                         (or profane) bonus on all attack rolls. The bonus is applied to her own \
                         baseline melee attack bonus only (see compute_combat_baseline) -- this \
                         codebase does not model which of Sacred/Profane Judgment an inquisitor's \
                         own alignment grants, since the numeric bonus is identical either way, only \
                         the flavor name differs"
                    ),
                });
            } else if judgment_selection == Some(INQUISITOR_JUDGMENT_PROTECTION_SELECTION_ID) {
                let ac_bonus = inquisitor_protection_judgment_ac_bonus(inquisitor_level);
                explanations.push(ComputationExplanation {
                    id: "class_feature.apg.inquisitor.judgment_execution.active".to_owned(),
                    value: ac_bonus,
                    detail: format!(
                        "Inquisitor level {inquisitor_level} is actively pronouncing the \
                         Protection judgment, within the grounded uses-per-day budget \
                         ({uses_per_day} uses; {uses_consumed_today} consumed today), granting a \
                         +{ac_bonus} sacred (or profane) bonus to Armor Class. The bonus is \
                         applied to her own baseline armor class only (see \
                         compute_combat_baseline) -- this codebase does not model which of \
                         Sacred/Profane Judgment an inquisitor's own alignment grants, since the \
                         numeric bonus is identical either way, only the flavor name differs"
                    ),
                });
            } else if judgment_selection == Some(INQUISITOR_JUDGMENT_PURITY_SELECTION_ID) {
                let save_bonus = inquisitor_purity_judgment_save_bonus(inquisitor_level);
                explanations.push(ComputationExplanation {
                    id: "class_feature.apg.inquisitor.judgment_execution.active".to_owned(),
                    value: save_bonus,
                    detail: format!(
                        "Inquisitor level {inquisitor_level} is actively pronouncing the Purity \
                         judgment, within the grounded uses-per-day budget ({uses_per_day} uses; \
                         {uses_consumed_today} consumed today), granting a +{save_bonus} sacred \
                         (or profane) bonus on all saving throws. The bonus is applied to her own \
                         total saves only (see compute_total_saves) -- this codebase does not \
                         model which of Sacred/Profane Judgment an inquisitor's own alignment \
                         grants, since the numeric bonus is identical either way, only the \
                         flavor name differs"
                    ),
                });
            } else if judgment_selection == Some(INQUISITOR_JUDGMENT_SMITING_SELECTION_ID) {
                explanations.push(ComputationExplanation {
                    id: "class_feature.apg.inquisitor.judgment_execution.active".to_owned(),
                    value: 0,
                    detail: format!(
                        "Inquisitor level {inquisitor_level} is actively pronouncing the Smiting \
                         judgment, within the grounded uses-per-day budget ({uses_per_day} uses; \
                         {uses_consumed_today} consumed today): her weapons count as magic for \
                         the purposes of bypassing damage reduction -- a flat, level-independent \
                         fact with no numeric total in this codebase to layer it onto, so it is \
                         grounded as a standalone fact record only, the same shape Brawler's \
                         Strike DR-bypass fact already established"
                    ),
                });
            } else if judgment_selection == Some(INQUISITOR_JUDGMENT_DESTRUCTION_SELECTION_ID) {
                let damage_bonus = inquisitor_destruction_judgment_damage_bonus(inquisitor_level);
                explanations.push(ComputationExplanation {
                    id: "class_feature.apg.inquisitor.judgment_execution.active".to_owned(),
                    value: damage_bonus,
                    detail: format!(
                        "Inquisitor level {inquisitor_level} is actively pronouncing the \
                         Destruction judgment, within the grounded uses-per-day budget \
                         ({uses_per_day} uses; {uses_consumed_today} consumed today), granting a \
                         +{damage_bonus} sacred (or profane) bonus on all weapon damage rolls. No \
                         weapon-damage-roll total exists anywhere in this codebase to layer this \
                         onto, so it is grounded as a standalone fact record only, the same shape \
                         Smiting already established"
                    ),
                });
            } else if judgment_selection == Some(INQUISITOR_JUDGMENT_HEALING_SELECTION_ID) {
                let fast_healing = inquisitor_healing_judgment_fast_healing(inquisitor_level);
                explanations.push(ComputationExplanation {
                    id: "class_feature.apg.inquisitor.judgment_execution.active".to_owned(),
                    value: fast_healing,
                    detail: format!(
                        "Inquisitor level {inquisitor_level} is actively pronouncing the Healing \
                         judgment, within the grounded uses-per-day budget ({uses_per_day} uses; \
                         {uses_consumed_today} consumed today), granting fast healing \
                         {fast_healing} (the inquisitor heals {fast_healing} point of damage each \
                         round while alive and the judgment lasts). No round-tick hit-point state \
                         exists anywhere in this codebase to layer this onto, so it is grounded as \
                         a standalone fact record only, the same shape Smiting already established"
                    ),
                });
            } else if judgment_selection == Some(INQUISITOR_JUDGMENT_PIERCING_SELECTION_ID) {
                let piercing_bonus = inquisitor_piercing_judgment_bonus(inquisitor_level);
                explanations.push(ComputationExplanation {
                    id: "class_feature.apg.inquisitor.judgment_execution.active".to_owned(),
                    value: piercing_bonus,
                    detail: format!(
                        "Inquisitor level {inquisitor_level} is actively pronouncing the Piercing \
                         judgment, within the grounded uses-per-day budget ({uses_per_day} uses; \
                         {uses_consumed_today} consumed today), granting a +{piercing_bonus} \
                         sacred (or profane) bonus on concentration checks and caster level \
                         checks made to overcome a target's spell resistance. No concentration- \
                         check or caster-level-vs-SR-check total exists anywhere in this codebase \
                         to layer this onto, so it is grounded as a standalone fact record only, \
                         the same shape Smiting already established"
                    ),
                });
            } else if judgment_selection == Some(INQUISITOR_JUDGMENT_RESILIENCY_SELECTION_ID) {
                let dr = inquisitor_resiliency_judgment_dr(inquisitor_level);
                explanations.push(ComputationExplanation {
                    id: "class_feature.apg.inquisitor.judgment_execution.active".to_owned(),
                    value: dr,
                    detail: format!(
                        "Inquisitor level {inquisitor_level} is actively pronouncing the \
                         Resiliency judgment, within the grounded uses-per-day budget \
                         ({uses_per_day} uses; {uses_consumed_today} consumed today), granting \
                         damage reduction {dr}/magic (or, from level 10 on, DR against the \
                         alignment type opposite the inquisitor's own -- this closure grounds \
                         only the flat numeric DR value, not the bypass-type switch). No DR- \
                         received total exists anywhere in this codebase to layer this onto (a \
                         defensive facet distinct from Smiting's own DR-bypass-on-attack), so it \
                         is grounded as a standalone fact record only, the same shape Smiting \
                         already established"
                    ),
                });
            } else if judgment_selection == Some(INQUISITOR_JUDGMENT_RESISTANCE_SELECTION_ID) {
                let energy_resistance =
                    inquisitor_resistance_judgment_energy_resistance(inquisitor_level);
                explanations.push(ComputationExplanation {
                    id: "class_feature.apg.inquisitor.judgment_execution.active".to_owned(),
                    value: energy_resistance,
                    detail: format!(
                        "Inquisitor level {inquisitor_level} is actively pronouncing the \
                         Resistance judgment, within the grounded uses-per-day budget \
                         ({uses_per_day} uses; {uses_consumed_today} consumed today), granting \
                         {energy_resistance} points of energy resistance against one energy type \
                         (acid, cold, electricity, fire, or sonic) chosen when the judgment is \
                         declared -- the chosen type does not affect the numeric magnitude \
                         (mirrors Justice/Protection/Purity not modeling which of Sacred/Profane \
                         an inquisitor's own alignment grants). No energy-resistance total exists \
                         anywhere in this codebase to layer this onto, so it is grounded as a \
                         standalone fact record only, the same shape Smiting already established"
                    ),
                });
            } else {
                diagnostics.push(ComputationDiagnostic {
                    id: "class_feature.apg.inquisitor.judgment_execution.judgment_choice_missing"
                        .to_owned(),
                    message: format!(
                        "Inquisitor level {inquisitor_level} claims an active Judgment \
                         ({INQUISITOR_JUDGMENT_ABILITY_ID}) but has no recognized \
                         {INQUISITOR_JUDGMENT_CHOICE_ID} selection naming one of the 9 real \
                         judgment types (Justice, Protection, Purity, Smiting, Destruction, \
                         Healing, Piercing, Resiliency, Resistance -- got \
                         {judgment_selection:?}): pronouncing judgment always requires choosing a \
                         type first per the PF1 Advanced Player's Guide's own sequencing, and \
                         those nine are the complete real judgment-type roster this closure \
                         grounds (see each INQUISITOR_JUDGMENT_*_SELECTION_ID's own doc comment), \
                         so an active judgment naming any other (unrecognized) value -- or none \
                         -- is a genuine posture violation, not a silently passing one -- no bonus \
                         is claimed for this input"
                    ),
                    claim_blocking: true,
                });
                push_inquisitor_other_features_deferred_diagnostic(input, diagnostics);
                return;
            }

            explanations.push(ComputationExplanation {
                id: "class_feature.apg.inquisitor.judgment_execution.uses_per_day".to_owned(),
                value: uses_per_day,
                detail: format!(
                    "Inquisitor level {inquisitor_level} Judgment uses per day: \
                     1 + (level-1)/3 = {uses_per_day}. Genuinely enforced -- an activation whose \
                     rounds_consumed_today exceeds this budget claim-blocks (see the \
                     uses_exceeded check above), mirroring Rage's/Bloodrage's own enforced \
                     budget exactly"
                ),
            });
        }
        ActiveState::SelectedInactive | ActiveState::Absent => {
            explanations.push(ComputationExplanation {
                id: "class_feature.apg.inquisitor.judgment_execution.not_judging".to_owned(),
                value: 0,
                detail: format!(
                    "Inquisitor level {inquisitor_level} has a \
                     \"{INQUISITOR_JUDGMENT_ABILITY_ID}\" activation entry but it is not active \
                     for this snapshot: a genuinely valid PF1 posture, so no attack-roll bonus \
                     is claimed"
                ),
            });
        }
    }

    push_inquisitor_other_features_deferred_diagnostic(input, diagnostics);
}

/// Grounds or names-but-defers Inquisitor's Domain class feature's granted power
/// (task #64, generalizing the pre-existing Cleric-only closure -- see
/// `active_touch_of_good_bonus` and `TOUCH_OF_GOOD_ABILITY_ID`'s own doc comment
/// for the corpus verification backing this, which corrected a previously-wrong
/// claim that Inquisitor domains grant no powers at all). PF1 Advanced Player's
/// Guide Domain: an inquisitor selects one domain from among those belonging to
/// her deity and gains ONLY that domain's powers, never its bonus spells. Called
/// unconditionally alongside Judgment's own grounding, from the Inquisitor branch
/// of `compute_apg_class_chassis`, mirroring the Cleric domain-power closure's
/// exact shape (self-application-only, activation-gated) -- the catch-all
/// diagnostic below preserves the same claim-blocking posture for every domain
/// not in `domain_power::DOMAIN_POWER_CATALOG`, exactly the "narrow the blocker,
/// don't remove it" discipline this whole file uses.
///
/// SD-31 wave 25 (OPERATOR-RULINGS-2026-08-21.md section 20): widened from
/// Good-only to every `domain_power::DOMAIN_POWER_CATALOG` entry -- War's Battle
/// Rage and Strength's Strength Surge, both confirmed legal base-class Inquisitor
/// domains directly against the corpus (`advanced_players_guide/class_feature/
/// inquisitor/inquisitor_domains.json` carries `DEFINE:InquisitorDomainWar|0` and
/// `DEFINE:InquisitorDomainStrength|0` alongside Good's own), not just reused from
/// Cleric's list -- via the SAME interpreted-formula path, generalized rather than
/// duplicated per domain. SD-31 wave 26 widened the catalog again (Destruction's
/// Destructive Smite, Glory's Touch of Glory) -- same corpus file confirms
/// `DEFINE:InquisitorDomainDestruction|0` and `DEFINE:InquisitorDomainGlory|0`
/// alongside the rest, so both are real base-class Inquisitor domains too; no
/// change was needed to this function's own logic, only to the shared catalog it
/// already reads generically.
pub(super) fn ground_or_block_inquisitor_domain_power(
    input: &CharacterInput,
    inquisitor_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(spec) = inquisitor_canonical_domain_recorded(input) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.inquisitor.domain_powers.unsupported".to_owned(),
            message: "Inquisitor remains blocked on its Domain class feature's granted-power \
                 burden: domain selection and the granted powers of any domain other than Good, \
                 War, Strength, Destruction, Glory, Undead Subdomain, or Construct Subdomain \
                 (whose own granted powers are grounded separately when actually chosen, one \
                 domain at a time) are not implemented anywhere in this codebase, so no \
                 Inquisitor domain-power support is claimed. Unlike Cleric, an \
                 inquisitor's domain never grants bonus spells (PF1 Advanced Player's Guide \
                 Domain: 'An inquisitor does not gain the bonus spells listed for each domain, \
                 nor does she gain bonus spell slots'), so there is no separate domain-spell \
                 burden to name here"
                .to_owned(),
            claim_blocking: true,
        });
        return;
    };

    // SD-34 wave 37 lane A: mirrors Cleric's own `grounds_self_application`
    // guard (see `explain_cleric_level1_spell_baseline`'s own comment) --
    // `false` for a catalog entry whose corpus formula is an effect DURATION
    // rather than a flat bonus (Undead Subdomain's Death's Kiss), so this
    // block never emits a "+{magnitude}" sentence that would misrepresent a
    // round count as a game bonus. Unchanged for every pre-existing entry.
    if spec.grounds_self_application {
        let magnitude = domain_power_magnitude(spec, inquisitor_level, &AbilityModifiers::default());
        let activation = input
            .chosen
            .class_ability_activations
            .iter()
            .find(|activation| activation.ability_id == spec.ability_id);
        match activation.map(|activation| activation.active_state) {
            Some(ActiveState::EquippedActive) => {
                explanations.push(ComputationExplanation {
                    id: domain_power_explanation_id(spec, "self_application"),
                    value: magnitude,
                    detail: format!(
                        "Inquisitor level {inquisitor_level}, whose Domain class feature selected \
                         {domain} (PF1 Advanced Player's Guide Domain: an inquisitor gains ONLY that \
                         domain's powers, never its bonus spells), is actively using {power} on \
                         HERSELF, SELF-APPLICATION ONLY: a +{magnitude} {label} {duration}. \
                         {power_specific}Granting this bonus to ANOTHER creature is NOT modeled: no \
                         target-creature entity exists anywhere in this codebase.",
                        domain = spec.domain_display_name,
                        power = spec.granted_power_name,
                        label = spec.magnitude_label,
                        duration = spec.effect_duration_phrase,
                        power_specific = if spec.selection_id == GOOD_DOMAIN_SELECTION {
                            "The +{magnitude} bonus is applied to her own baseline melee attack \
                             bonus, selected-skill modifiers, and total saves (see \
                             compute_combat_baseline, compute_selected_skill_modifiers, \
                             compute_total_saves); the ability-check facet has no separate \
                             integrated total in this codebase and stays a flat, unintegrated \
                             magnitude. "
                        } else {
                            "This grounds only the flat magnitude; it is not integrated into any \
                             other computed total (melee damage, combat maneuver, or ability-check \
                             totals) anywhere in this codebase. "
                        },
                    ),
                });
            }
            _ => {
                explanations.push(ComputationExplanation {
                    id: domain_power_explanation_id(spec, "not_active"),
                    value: 0,
                    detail: format!(
                        "Inquisitor level {inquisitor_level} is not currently using {power} (no \
                         active class_ability_activations entry for \"{ability_id}\"): a genuinely \
                         valid PF1 posture -- not every {domain}-domain Inquisitor is using this \
                         limited-use power at every moment -- so no {label} is claimed",
                        power = spec.granted_power_name,
                        ability_id = spec.ability_id,
                        domain = spec.domain_display_name,
                        label = spec.magnitude_label,
                    ),
                });
            }
        }
    }

    let wisdom_modifier = ability_modifier(input.chosen.ability_scores.wisdom);
    // SD-31 wave 25 (OPERATOR-RULINGS-2026-08-21.md section 20): interprets
    // the corpus's own shared `3+WIS` formula (`domain_power::DOMAIN_POWER_TIMES_FORMULA`)
    // rather than a hand-written `(3 + wisdom_modifier).max(0)` closed form.
    // SD-34 wave 38 lane A: `domain_power_uses_per_day_for` reads `spec.
    // uses_per_day_formula` when a spec carries one (Animate Servant), else
    // falls back to the exact same `3+WIS` value `domain_power_uses_per_day`
    // computes -- every pre-existing entry's own value is unchanged.
    let uses_per_day = domain_power_uses_per_day_for(
        spec,
        inquisitor_level,
        &AbilityModifiers { wisdom: wisdom_modifier, ..AbilityModifiers::default() },
    );
    let uses_per_day_detail = if let Some(formula) = spec.uses_per_day_formula {
        format!(
            "Inquisitor {domain} domain granted power {power} uses per day (PF1 Advanced \
             Player's Guide {domain}): this power's OWN corpus formula, {formula} (not the \
             shared 3 + Wisdom modifier chain every other catalogued domain power uses), \
             evaluated at Inquisitor level {inquisitor_level} and floored at 0. This is \
             {uses_per_day}. This grounds only the flat daily use count; it performs no \
             per-use consumption tracking",
            domain = spec.domain_display_name,
            power = spec.granted_power_name,
        )
    } else {
        format!(
            "Inquisitor {domain} domain granted power {power} uses per day (PF1 Core Rulebook \
             Domains): 3 + Wisdom modifier, floored at 0. At Wisdom modifier {wisdom_modifier} \
             this is max(3 + {wisdom_modifier}, 0) = {uses_per_day}. This grounds only the flat \
             daily use count; it performs no per-use consumption tracking",
            domain = spec.domain_display_name,
            power = spec.granted_power_name,
        )
    };
    explanations.push(ComputationExplanation {
        id: domain_power_explanation_id(spec, "uses_per_day"),
        value: uses_per_day,
        detail: uses_per_day_detail,
    });

    // SD-34 wave 37 lane A: the real corpus `DEFINE` token concatenates a
    // multi-word domain display name with no space (`Undead Subdomain` ->
    // `InquisitorDomainUndeadSubdomain`, confirmed by direct corpus read of
    // `inquisitor_domains.json`) -- every pre-existing catalog entry's own
    // `domain_display_name` happens to be a single word, so this is the
    // first spec where blindly interpolating a space into the token name
    // would cite a token that does not exist.
    let define_token_domain = spec.domain_display_name.replace(' ', "");
    let grounded_facets = if spec.grounds_self_application {
        "bonus magnitude, self-application state, and uses-per-day count"
    } else {
        // Death's Kiss shape: its own corpus formula is an effect DURATION,
        // not a flat bonus -- only its honestly-computed uses-per-day count
        // is grounded (see `grounds_self_application`'s own doc comment).
        "uses-per-day count only (its own corpus formula is an effect \
         duration, not a flat bonus, so no bonus magnitude is claimed)"
    };
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.inquisitor.domain_powers.unsupported".to_owned(),
        message: format!(
            "Inquisitor's remaining Domain class-feature granted-power burden is named but no \
             longer claim-blocking: the {domain} domain's {power} ({grounded_facets}) IS \
             grounded, and the corpus confirms {domain} is a real base-class Inquisitor domain \
             (`advanced_players_guide/class_feature/inquisitor/inquisitor_domains.json` \
             declares an `InquisitorDomain{define_token_domain}` variable of its own). So a \
             {domain}-domain \
             inquisitor's domain power is genuinely computed rather than deferred. Every domain \
             not in `domain_power::DOMAIN_POWER_CATALOG` (Good, War, Strength, Destruction, \
             Glory, Undead Subdomain, Construct Subdomain) remains entirely unproven and is \
             still not claimed anywhere -- selecting one keeps this diagnostic claim-blocking \
             (Path A canonical narrowing, 2026-07-29, widened SD-31 wave 25, widened again \
             SD-31 wave 26, widened again SD-34 wave 37, widened again SD-34 wave 38, \
             mirroring Cleric's own domain-power seam)",
            domain = spec.domain_display_name,
            power = spec.granted_power_name,
        ),
        claim_blocking: false,
    });
}

/// The one canonical `domain_power::DOMAIN_POWER_CATALOG` entry this input
/// records for its Inquisitor Domain class feature, if exactly one such
/// recognized domain is selected and no other -- recognized or not --
/// selection rides alongside it (Path A canonical narrowing, 2026-07-29;
/// widened from Good-only to the whole catalog SD-31 wave 25).
///
/// The "and no other" half is load-bearing rather than defensive: PF1
/// gives an inquisitor exactly ONE domain (PF1 Advanced Player's Guide
/// Domain), so a second selection is already an illegal posture, and
/// letting it ride the first domain's own narrowing to `Computed` would
/// claim support for a domain whose granted power is grounded nowhere.
/// This is Cleric's own `unrecognized_other_domain_chosen` guard, applied
/// to the class that was missing it.
pub(super) fn inquisitor_canonical_domain_recorded(input: &CharacterInput) -> Option<&'static DomainPowerSpec> {
    let domain_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == INQUISITOR_DOMAIN_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();

    if domain_selections.len() != 1 {
        return None;
    }
    resolve_domain_power(domain_selections[0])
}

/// Pushes the new, narrower diagnostic replacing
/// `class_feature.apg.inquisitor.unsupported` for Inquisitor specifically
/// (v0.6 alpha swarm, risks item 8, Inquisitor Judgment closure, widened
/// 2026-07-26 to name Protection/Purity/Smiting and Stern Gaze as grounded
/// rather than missing; task #64 corrected this comment's own prior WRONG
/// claim that no Inquisitor domain power exists -- see
/// `ground_or_block_inquisitor_domain_power`'s own doc comment for the
/// corpus correction; widened again task #47, 2026-07-28, to name the
/// remaining 5 judgment types, Bane, and known-spell posture as grounded
/// too): named ONLY the genuinely still-missing pieces.
/// Inquisitor's domain BONUS SPELLS specifically (verified directly
/// against d20pfsrd's Inquisitor class page: "An inquisitor does not gain
/// the bonus spells listed for each domain, nor does she gain bonus spell
/// slots") are folded into the spellcasting bucket below rather than named
/// separately, since there genuinely is no domain-spell burden for
/// Inquisitor to name (unlike Cleric). The domain POWERS burden (Good's
/// Touch of Good now grounded, every other domain still unproven) is named
/// by `ground_or_block_inquisitor_domain_power`'s own separate diagnostic
/// instead, so it is deliberately NOT duplicated in this message. Orisons
/// are folded into the spellcasting bucket, not named separately -- the
/// same "shares the general spellcasting mechanism, not independently
/// implemented" reasoning already applied to Arcanist's Cantrips and
/// Warpriest's own Orisons. Second/Third Judgment and Greater Bane are
/// named but not separately grounded: both are DESC-only multipliers on
/// already-grounded mechanics (Second/Third Judgment let an inquisitor
/// pronounce more than one judgment type at once; Greater Bane widens
/// Bane's own weapon-enhancement bonus) with no independent primary-
/// source-backed numeric magnitude of their own to verify. Pushed
/// unconditionally regardless of Judgment's own active state, mirroring
/// Alchemist's/Skald's own diagnostic-honesty fix.
pub(super) fn push_inquisitor_other_features_deferred_diagnostic(
    input: &CharacterInput,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // Path A canonical narrowing (2026-07-29): once the one canonical
    // Inquisitor domain this codebase grounds is genuinely recorded, this
    // remainder is named but no longer claim-blocking -- Arcanist's own
    // `exploits_deferred` shape exactly. With no domain recorded (or an
    // unrecognized one alongside Good) the original claim-blocking posture
    // is preserved unchanged, and `ground_or_block_inquisitor_domain_power`
    // independently claim-blocks that same case, so this narrowing can
    // never be the only thing standing between an unproven domain and
    // `Computed`.
    let canonical_domain_recorded = inquisitor_canonical_domain_recorded(input).is_some();
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.apg.inquisitor.other_features_deferred.unsupported".to_owned(),
        message: format!(
            "{INQUISITOR_CLASS_ID} remains blocked beyond its base-attack-bonus/base-save \
             chassis pillar, all 9 Judgment types (Justice, Protection, Purity, Smiting, \
             Destruction, Healing, Piercing, Resiliency, Resistance), Stern Gaze's Intimidate \
             half, Monster Lore, Cunning Initiative, Track, Bane's pool size, the Good domain's \
             Touch of Good, and its own known-spell posture (a real, independently-verified \
             219-spell list, levels 0-6 -- the real CLASS:Inquisitor record carries no \
             SPELLLIST token, unlike Hunter/Oracle, so no existing list module could be \
             reused): this APG class has no class-skill list, no domain power for any domain \
             but Good (named separately as its own domain-powers burden), Orisons are folded \
             into the general known-spell posture above rather than named separately, and no \
             other named class feature (Detect Alignment, Discern Lies, Exploit Weakness, Solo \
             Tactics, Stalwart, Stern Gaze's Sense Motive half, Second/Third Judgment, Greater \
             Bane -- the latter two are DESC-only multipliers on already-grounded mechanics \
             with no independent numeric magnitude of their own) grounded anywhere in this \
             codebase yet; no class-feature or spell execution is fabricated in this bounded \
             chassis baseline. {}",
            if canonical_domain_recorded {
                "This character HAS recorded the one canonical Inquisitor domain this codebase \
                 grounds (Good, whose Touch of Good is computed above), so this remainder is \
                 named but no longer claim-blocking -- the same canonical-narrowing posture \
                 Arcanist's own exploits_deferred and Cleric's own Good-domain seam already \
                 ship. Every item listed above genuinely remains ungrounded; none of them is \
                 silently fabricated."
            } else {
                "No canonical domain is recorded, so this remainder stays claim-blocking."
            }
        ),
        claim_blocking: !canonical_domain_recorded,
    });
}

/// The PF1 Advanced Player's Guide Inquisitor Spells Known table's row,
/// one entry per spell level 0-6 (index 0 is orisons; `None` for an
/// inaccessible "--" column). A literal table lookup transcribed
/// directly from `apg_classes.lst`'s own `CLASS:Inquisitor` per-level
/// `KNOWN:` rows (task #47, 2026-07-28), levels 1-20 -- see
/// `INQUISITOR_KNOWN_SPELLS_MAX_LEVEL`'s own doc comment for why this is
/// the complete real table rather than an artificially bounded MVP slice.
/// This is the cap on distinct spells KNOWN (permanent), the same shape
/// as `oracle_spells_known_table`/`sorcerer_spells_known_table` --
/// Inquisitor is a spontaneous caster (`MEMORIZE:NO`), not a prepared
/// caster like Cleric/Wizard/Arcanist/Warpriest.
pub(super) fn inquisitor_spells_known_table(level: u8) -> [Option<i16>; 7] {
    match level {
        1 => [Some(4), Some(2), None, None, None, None, None],
        2 => [Some(5), Some(3), None, None, None, None, None],
        3 => [Some(6), Some(4), None, None, None, None, None],
        4 => [Some(6), Some(4), Some(2), None, None, None, None],
        5 => [Some(6), Some(4), Some(3), None, None, None, None],
        6 => [Some(6), Some(4), Some(4), None, None, None, None],
        7 => [Some(6), Some(5), Some(4), Some(2), None, None, None],
        8 => [Some(6), Some(5), Some(4), Some(3), None, None, None],
        9 => [Some(6), Some(5), Some(4), Some(4), None, None, None],
        10 => [Some(6), Some(5), Some(5), Some(4), Some(2), None, None],
        11 => [Some(6), Some(6), Some(5), Some(4), Some(3), None, None],
        12 => [Some(6), Some(6), Some(5), Some(4), Some(4), None, None],
        13 => [Some(6), Some(6), Some(5), Some(5), Some(4), Some(2), None],
        14 => [Some(6), Some(6), Some(6), Some(5), Some(4), Some(3), None],
        15 => [Some(6), Some(6), Some(6), Some(5), Some(4), Some(4), None],
        16 => [Some(6), Some(6), Some(6), Some(5), Some(5), Some(4), Some(2)],
        17 => [Some(6), Some(6), Some(6), Some(6), Some(5), Some(4), Some(3)],
        18 => [Some(6), Some(6), Some(6), Some(6), Some(5), Some(4), Some(4)],
        19 => [Some(6), Some(6), Some(6), Some(6), Some(5), Some(5), Some(4)],
        20 => [Some(6), Some(6), Some(6), Some(6), Some(6), Some(5), Some(5)],
        _ => [None, None, None, None, None, None, None],
    }
}

/// Return the list of unmet conditions for Inquisitor's real known-spell
/// posture (task #47, 2026-07-28), mirroring
/// `unmet_oracle_known_spell_conditions`'s own shape exactly, substituting
/// Inquisitor's own full 1-20 table and
/// `inquisitor_spell_list::inquisitor_spell_level` for the per-spell-id
/// level lookup (the real `CLASS:Inquisitor` record carries no
/// `SPELLLIST:` token, so this is Inquisitor's own independently-tagged
/// list, not a reused one). An empty list means the posture is fully
/// valid; zero known spells is always valid, same reasoning as
/// Sorcerer's/Oracle's own posture.
pub(super) fn unmet_inquisitor_known_spell_conditions(input: &CharacterInput, inquisitor_level: u8) -> Vec<String> {
    let mut unmet = Vec::new();

    if inquisitor_level > INQUISITOR_KNOWN_SPELLS_MAX_LEVEL {
        unmet.push(format!(
            "known-spell grounding is only supported for inquisitor levels \
             1-{INQUISITOR_KNOWN_SPELLS_MAX_LEVEL}, got {inquisitor_level}"
        ));
        return unmet;
    }

    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == INQUISITOR_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    let known_table = inquisitor_spells_known_table(inquisitor_level);

    let mut known_per_level: [i16; 7] = [0; 7];
    for spell_id in &known {
        let Some(spell_level) = inquisitor_spell_list::inquisitor_spell_level(spell_id) else {
            unmet.push(format!(
                "known spell '{spell_id}' is not on the real PF1 inquisitor spell list"
            ));
            continue;
        };
        if usize::from(spell_level) >= known_table.len() {
            unmet.push(format!(
                "known spell '{spell_id}' targets spell level {spell_level}, not yet accessible \
                 at inquisitor level {inquisitor_level}"
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
                 slots available on the Inquisitor Spells Known table"
            ));
        }
    }

    unmet
}

/// Ground the real known-spell posture once
/// `unmet_inquisitor_known_spell_conditions` reports an empty unmet list
/// (task #47, 2026-07-28), mirroring `ground_oracle_known_spells`'s own
/// shape exactly.
pub(super) fn ground_inquisitor_known_spells(
    input: &CharacterInput,
    inquisitor_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == INQUISITOR_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.apg.inquisitor.known_spells".to_owned(),
        value: known.len() as i16,
        detail: format!(
            "Inquisitor level {inquisitor_level} known-spell selection ({} spells, \
             AcquisitionMode::Known): {}. Each known spell is verified against the real PF1 \
             inquisitor spell list (`inquisitor_spell_list::inquisitor_spell_level`, all \
             ingested books, a fresh independent parse -- the real CLASS:Inquisitor record \
             carries no SPELLLIST token, unlike Hunter/Oracle, so no existing list module could \
             be reused) and the Inquisitor Spells Known table's own per-level cap. Real PF1 \
             Inquisitor rules have no daily preparation step at all (unlike Cleric/Wizard/\
             Arcanist/Warpriest) -- an inquisitor's known spells are permanent once learned, \
             cast spontaneously. This grounds the known-spell selection for real; it computes no \
             spell save DC resolution against a target and no casting execution",
            known.len(),
            known.join(", ")
        ),
    });
}

/// v0.6 alpha swarm, risks item 8 (Inquisitor Judgment closure, third APG
/// class-specific closure, 2026-07-25): Inquisitor's Judgment combines the
/// activation-gating pattern with the choice-recognition pattern, the
/// same shape as Alchemist's Mutagen, mirroring its own test module
/// exactly. Like Cavalier/Alchemist, a valid Inquisitor posture never
/// reaches full `Computed` this slice (spellcasting/other named features
/// stay permanently deferred behind the narrowed diagnostic).
#[cfg(test)]
mod inquisitor_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, ActiveState, ANIMATE_SERVANT_ABILITY_ID, CharacterClassLevel,
        CharacterInput, CONSTRUCT_SUBDOMAIN_SELECTION, DEATH_S_KISS_ABILITY_ID,
        HeadlessReceiptStatus, FIGHTER_CLASS_ID, GLORY_DOMAIN_SELECTION, GOOD_DOMAIN_SELECTION,
        INQUISITOR_CLASS_ID, INQUISITOR_DOMAIN_CHOICE_ID, INQUISITOR_JUDGMENT_ABILITY_ID,
        INQUISITOR_JUDGMENT_CHOICE_ID, INQUISITOR_JUDGMENT_DESTRUCTION_SELECTION_ID,
        INQUISITOR_JUDGMENT_HEALING_SELECTION_ID, INQUISITOR_JUDGMENT_JUSTICE_SELECTION_ID,
        INQUISITOR_JUDGMENT_PIERCING_SELECTION_ID, INQUISITOR_JUDGMENT_PROTECTION_SELECTION_ID,
        INQUISITOR_JUDGMENT_PURITY_SELECTION_ID, INQUISITOR_JUDGMENT_RESILIENCY_SELECTION_ID,
        INQUISITOR_JUDGMENT_RESISTANCE_SELECTION_ID, INQUISITOR_JUDGMENT_SMITING_SELECTION_ID,
        TOUCH_OF_GLORY_ABILITY_ID, TOUCH_OF_GOOD_ABILITY_ID, UNDEAD_SUBDOMAIN_SELECTION,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation, SelectedChoice,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_inquisitor_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: INQUISITOR_CLASS_ID.to_owned(), level }];
        input
    }

    /// A single-class Human Inquisitor who is not currently pronouncing
    /// judgment (no `class_ability_activations` entry at all) is a
    /// genuinely valid PF1 posture -- stays `Blocked` on the new, narrower
    /// `other_features_deferred` diagnostic alone (never the retired
    /// generic one), with the honest "not judging" recognition record
    /// grounded.
    #[test]
    fn single_class_inquisitor_not_judging_stays_blocked_on_deferred_features_only() {
        let input = human_inquisitor_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Inquisitor must stay Blocked on other-features alone: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.apg.inquisitor.judgment_execution.not_judging"),
            "expected the honest not-judging recognition record: {:?}",
            receipt.computation.explanations
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.inquisitor.unsupported"),
            "the retired generic diagnostic must never appear for Inquisitor: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.inquisitor.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Human Inquisitor actively, validly pronouncing the
    /// Justice judgment applies the real attack-roll bonus to the
    /// integrated melee attack bonus -- but still stays `Blocked`
    /// (other_features_deferred), mirroring Alchemist's own "real bonus
    /// applied, still Blocked overall" shape.
    ///
    /// Level 1 Justice bonus: 1 + 1/5 = 1.
    #[test]
    fn single_class_inquisitor_actively_judging_justice_with_recognized_choice_applies_real_bonus()
    {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_JUSTICE_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Inquisitor stays Blocked on other features even while actively, validly judging: \
             {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.inquisitor.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the other_features_deferred diagnostic even while judging: {:?}",
            receipt.computation.diagnostics
        );

        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        // Base attack bonus (Inquisitor level 1: 0) + Strength modifier
        // (fixture base 16 + Human +2 -> +4) + Weapon Focus (+1) +
        // Justice judgment bonus (+1) - 4 nonproficiency = 2.
        //
        // Corrected 6 -> 2 (risks item #89, tasks #80+#86, 2026-07-29).
        // Inquisitor's real corpus grant
        // (`apg_abilities_globalvar.lst:340`) is the Simple tier plus
        // `AUTO:WEAPONPROF|Crossbow (Hand)|Longbow|Crossbow (Repeating
        // Heavy)|Crossbow (Repeating Light)|Shortbow` and
        // `AUTO:WEAPONPROF|DEITYWEAPONS` -- no Longsword, and no deity is
        // modelled, so this baseline's Longsword owes the -4. The
        // Judgment bonus itself is unaffected, which is exactly what this
        // test still guards.
        assert_eq!(
            melee_attack_bonus.value, 2,
            "Justice judgment's attack-roll bonus must be applied: {:?}",
            melee_attack_bonus
        );
    }

    /// An Inquisitor claiming an active Judgment but with no recognized
    /// `choice:inquisitor_judgment` selection naming Justice is a genuine
    /// posture violation and must claim-block -- never silently passed,
    /// mirroring Alchemist's own "active but no recognized stat choice"
    /// shape.
    #[test]
    fn single_class_inquisitor_active_judgment_without_a_recognized_justice_choice_stays_blocked() {
        let mut input = human_inquisitor_input(1);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
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
                    == "class_feature.apg.inquisitor.judgment_execution.judgment_choice_missing"
                    && d.claim_blocking),
            "expected the missing-judgment-choice claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );

        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        // No Justice judgment bonus applied for an active-but-unrecognized-
        // choice posture: 0 (BAB) + 4 (STR) + 1 (Weapon Focus)
        // - 4 (nonproficiency, see the Justice case above) = 1.
        assert_eq!(
            melee_attack_bonus.value, 1,
            "no Judgment bonus is applied for an active-but-unrecognized-choice posture: {:?}",
            melee_attack_bonus
        );
    }

    /// A Judgment activation that exceeds the grounded uses-per-day
    /// budget is a genuine posture violation and must claim-block -- never
    /// silently capped, mirroring Rage's/Bloodrage's own over-budget check
    /// exactly (caught in review 2026-07-25: an earlier draft of this
    /// closure was missing this check entirely).
    ///
    /// Level 1 uses per day: 1 + (1-1)/3 = 1.
    #[test]
    fn single_class_inquisitor_over_budget_judgment_stays_blocked_and_applies_no_bonus() {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_JUSTICE_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
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
                .any(|d| d.id == "class_feature.apg.inquisitor.judgment_execution.uses_exceeded"
                    && d.claim_blocking),
            "expected the uses-exceeded claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );

        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        // No Justice judgment bonus applied for an over-budget posture:
        // 0 (BAB) + 4 (STR) + 1 (Weapon Focus)
        // - 4 (nonproficiency, see the Justice case above) = 1.
        assert_eq!(
            melee_attack_bonus.value, 1,
            "no Judgment bonus is applied for an over-budget posture: {:?}",
            melee_attack_bonus
        );
    }

    /// A single-class Human Inquisitor actively, validly pronouncing the
    /// Protection judgment applies the real Armor Class bonus to the
    /// integrated baseline armor class -- but still stays `Blocked`
    /// (other_features_deferred), mirroring Justice's own "real bonus
    /// applied, still Blocked overall" shape (2026-07-26 deepening,
    /// task #3: a genuine scoping correction, not a new architecture).
    ///
    /// Level 1 Protection bonus: 1 + 1/5 = 1.
    #[test]
    fn single_class_inquisitor_actively_judging_protection_with_recognized_choice_applies_real_bonus()
    {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_PROTECTION_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Inquisitor stays Blocked on other features even while actively, validly judging: \
             {:?}",
            receipt.computation.diagnostics
        );

        let armor_class = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .expect("baseline armor class must be grounded");
        // Base 10 + Chain Shirt (+4) + DEX contribution (fixture DEX 14,
        // mod +2, within MAXDEX 4) + Dodge (+1) + Protection judgment (+1) = 18.
        assert_eq!(
            armor_class.value, 18,
            "Protection judgment's Armor Class bonus must be applied: {:?}",
            armor_class
        );

        // Justice's own attack-bonus consumer must NOT leak a bonus for a
        // Protection-judging Inquisitor -- proves the four `active_*`
        // helpers are mutually exclusive by construction.
        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        // 0 (BAB) + 4 (STR) + 1 (Weapon Focus) - 4 (nonproficiency, see
        // the Justice case above) = 1. Protection contributes nothing to
        // attack, which is what this test guards.
        assert_eq!(
            melee_attack_bonus.value, 1,
            "Protection judgment must not leak an attack-roll bonus: {:?}",
            melee_attack_bonus
        );
    }

    /// A single-class Human Inquisitor actively, validly pronouncing the
    /// Purity judgment applies the real bonus to all three total saves --
    /// but still stays `Blocked` (other_features_deferred), same shape as
    /// Justice/Protection.
    ///
    /// Level 1 Purity bonus: 1 + 1/5 = 1.
    #[test]
    fn single_class_inquisitor_actively_judging_purity_with_recognized_choice_applies_real_bonus() {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_PURITY_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

        let fortitude = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.total_save.fortitude")
            .expect("total Fortitude save must be grounded");
        let reflex = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.total_save.reflex")
            .expect("total Reflex save must be grounded");
        let will = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.total_save.will")
            .expect("total Will save must be grounded");

        // Inquisitor level 1 base saves: Fortitude/Will +2, Reflex +0
        // (good Fort/Will, poor Reflex). Fixture: CON 14 (+2), DEX 14
        // (+2), WIS 12 (+1). Purity adds +1 to all three.
        assert_eq!(fortitude.value, 2 + 2 + 1, "Purity's +1 must apply to Fortitude: {fortitude:?}");
        assert_eq!(reflex.value, 2 + 1, "Purity's +1 must apply to Reflex: {reflex:?}");
        assert_eq!(will.value, 2 + 1 + 1, "Purity's +1 must apply to Will: {will:?}");
    }

    /// A single-class Human Inquisitor actively, validly pronouncing the
    /// Smiting judgment grounds the real DR-bypass fact as a standalone
    /// explanation record (no numeric total exists to layer it onto) --
    /// still stays `Blocked` (other_features_deferred).
    #[test]
    fn single_class_inquisitor_actively_judging_smiting_with_recognized_choice_grounds_the_dr_bypass_fact()
    {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_SMITING_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        let active_explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.inquisitor.judgment_execution.active")
            .expect("Smiting's active explanation must be grounded");
        assert!(
            active_explanation.detail.contains("magic") && active_explanation.detail.contains("Smiting"),
            "expected the Smiting DR-bypass fact to be named: {active_explanation:?}"
        );
    }

    /// A single-class Human Inquisitor actively, validly pronouncing the
    /// Destruction judgment grounds the real weapon-damage-roll bonus as a
    /// standalone explanation record (no numeric total exists to layer it
    /// onto) -- still stays `Blocked` (other_features_deferred). Task #47,
    /// 2026-07-28. Level 1 Destruction bonus: 1 + 1/3 = 1.
    #[test]
    fn single_class_inquisitor_actively_judging_destruction_with_recognized_choice_applies_real_bonus()
    {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_DESTRUCTION_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        let active_explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.inquisitor.judgment_execution.active")
            .expect("Destruction's active explanation must be grounded");
        assert_eq!(
            active_explanation.value, 1,
            "Destruction judgment's damage-roll bonus must be applied: {active_explanation:?}"
        );
        assert!(
            active_explanation.detail.contains("Destruction")
                && active_explanation.detail.contains("weapon damage rolls"),
            "expected the Destruction damage-roll bonus to be named: {active_explanation:?}"
        );
    }

    /// A single-class Human Inquisitor actively, validly pronouncing the
    /// Healing judgment grounds the real fast-healing magnitude as a
    /// standalone explanation record (no round-tick hit-point state exists
    /// to layer it onto) -- still stays `Blocked` (other_features_deferred).
    /// Task #47, 2026-07-28. Level 1 Healing fast healing: 1 + 1/3 = 1.
    #[test]
    fn single_class_inquisitor_actively_judging_healing_with_recognized_choice_applies_real_bonus() {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_HEALING_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        let active_explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.inquisitor.judgment_execution.active")
            .expect("Healing's active explanation must be grounded");
        assert_eq!(
            active_explanation.value, 1,
            "Healing judgment's fast-healing magnitude must be applied: {active_explanation:?}"
        );
        assert!(
            active_explanation.detail.contains("Healing") && active_explanation.detail.contains("fast healing"),
            "expected the Healing fast-healing fact to be named: {active_explanation:?}"
        );
    }

    /// A single-class Human Inquisitor actively, validly pronouncing the
    /// Piercing judgment grounds the real concentration/CL-vs-SR bonus as
    /// a standalone explanation record -- still stays `Blocked`
    /// (other_features_deferred). Task #47, 2026-07-28. Level 1 Piercing
    /// bonus: 1 + 1/3 = 1.
    #[test]
    fn single_class_inquisitor_actively_judging_piercing_with_recognized_choice_applies_real_bonus() {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_PIERCING_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        let active_explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.inquisitor.judgment_execution.active")
            .expect("Piercing's active explanation must be grounded");
        assert_eq!(
            active_explanation.value, 1,
            "Piercing judgment's bonus must be applied: {active_explanation:?}"
        );
        assert!(
            active_explanation.detail.contains("Piercing")
                && active_explanation.detail.contains("concentration"),
            "expected the Piercing bonus to be named: {active_explanation:?}"
        );
    }

    /// A single-class Human Inquisitor actively, validly pronouncing the
    /// Resiliency judgment grounds the real DR magnitude as a standalone
    /// explanation record (a defensive DR-received facet distinct from
    /// Smiting's own DR-bypass-on-attack) -- still stays `Blocked`
    /// (other_features_deferred). Task #47, 2026-07-28. Level 1 Resiliency
    /// DR: 1 + 1/5 = 1.
    #[test]
    fn single_class_inquisitor_actively_judging_resiliency_with_recognized_choice_applies_real_bonus()
    {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_RESILIENCY_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        let active_explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.inquisitor.judgment_execution.active")
            .expect("Resiliency's active explanation must be grounded");
        assert_eq!(
            active_explanation.value, 1,
            "Resiliency judgment's DR magnitude must be applied: {active_explanation:?}"
        );
        assert!(
            active_explanation.detail.contains("Resiliency")
                && active_explanation.detail.contains("damage reduction"),
            "expected the Resiliency DR fact to be named: {active_explanation:?}"
        );
    }

    /// A single-class Human Inquisitor actively, validly pronouncing the
    /// Resistance judgment grounds the real energy-resistance magnitude as
    /// a standalone explanation record -- still stays `Blocked`
    /// (other_features_deferred). Task #47, 2026-07-28. Level 1 Resistance:
    /// 2*(1+floor(1/3)) = 2.
    #[test]
    fn single_class_inquisitor_actively_judging_resistance_with_recognized_choice_applies_real_bonus()
    {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_RESISTANCE_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        let active_explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.inquisitor.judgment_execution.active")
            .expect("Resistance's active explanation must be grounded");
        assert_eq!(
            active_explanation.value, 2,
            "Resistance judgment's energy-resistance magnitude must be applied: {active_explanation:?}"
        );
        assert!(
            active_explanation.detail.contains("Resistance")
                && active_explanation.detail.contains("energy resistance"),
            "expected the Resistance fact to be named: {active_explanation:?}"
        );
    }

    /// An Inquisitor claiming an active Judgment but naming an
    /// unrecognized judgment selection id is a genuine posture violation,
    /// distinct from the "no choice at all" case already covered above --
    /// both must claim-block via the same diagnostic. All 9 real judgment
    /// types are now built (task #47, 2026-07-28), so this uses a
    /// definitely-bogus id rather than a real-but-formerly-unbuilt one
    /// (Destruction, this test's own id before the widening, now resolves
    /// to a real applied bonus -- see
    /// `single_class_inquisitor_actively_judging_destruction_with_recognized_choice_applies_real_bonus`).
    #[test]
    fn single_class_inquisitor_active_judgment_naming_an_unrecognized_type_stays_blocked() {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: "judgment:nonexistent".to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
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
                    == "class_feature.apg.inquisitor.judgment_execution.judgment_choice_missing"
                    && d.claim_blocking),
            "expected the missing-judgment-choice claim-blocking diagnostic even for a named but \
             unbuilt judgment type: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Stern Gaze's Intimidate morale bonus is unconditional the moment
    /// Inquisitor levels are present -- no choice, no activation gate,
    /// unlike Judgment -- and stacks independently of whatever Judgment
    /// state (if any) is active.
    ///
    /// Level 1 Stern Gaze bonus: max(1, 1/2) = 1.
    #[test]
    fn single_class_inquisitor_gets_the_unconditional_stern_gaze_intimidate_bonus() {
        let input = human_inquisitor_input(1);
        let computation = super::compute_pilot_base_chassis(&input);

        let intimidate = computation
            .explanations
            .iter()
            .find(|e| e.id == "skill.selected_modifier.intimidate")
            .expect("selected Intimidate modifier must be grounded");
        assert!(
            intimidate.detail.contains("Stern Gaze"),
            "expected Stern Gaze to be named in the Intimidate explanation: {intimidate:?}"
        );
    }

    /// Monster Lore, Cunning Initiative, Track (task #18, 2026-07-26), and
    /// Bane (task #47, 2026-07-28) are grounded as standalone flat
    /// records, unconditional the moment Inquisitor levels are present --
    /// no choice, no activation gate, same shape as Stern Gaze, and
    /// independent of Judgment's own state. Fixture: WIS 12 -> +1
    /// modifier.
    #[test]
    fn single_class_inquisitor_gets_the_unconditional_flat_named_facts() {
        let input = human_inquisitor_input(1);
        let computation = super::compute_pilot_base_chassis(&input);

        let monster_lore = computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.inquisitor.monster_lore_bonus")
            .expect("Monster Lore must be grounded");
        assert_eq!(monster_lore.value, 1, "WIS modifier +1: {monster_lore:?}");

        let cunning_initiative = computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.inquisitor.cunning_initiative_bonus")
            .expect("Cunning Initiative must be grounded");
        assert_eq!(cunning_initiative.value, 1, "WIS modifier +1: {cunning_initiative:?}");

        let track = computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.inquisitor.track_bonus")
            .expect("Track must be grounded");
        assert_eq!(track.value, 1, "max(1/2, 1) = 1: {track:?}");

        let bane = computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.apg.inquisitor.bane_pool_rounds")
            .expect("Bane must be grounded");
        assert_eq!(bane.value, 1, "level 1 Bane pool: InquisitorLVL = 1: {bane:?}");
    }

    /// Bane's real level-scaling (a flat InquisitorLVL rounds/day pool, no
    /// division) matches the corpus's own formula at every level.
    #[test]
    fn inquisitor_bane_pool_rounds_matches_the_real_flat_level_formula() {
        for level in [1, 2, 5, 10, 20] {
            assert_eq!(
                super::inquisitor_bane_pool_rounds(level),
                i16::from(level),
                "level {level} Bane pool rounds"
            );
        }
    }

    /// Track's own real level-scaling (max(level/2, 1)) matches Slayer's
    /// identically-shaped formula at every level, proven directly rather
    /// than assumed from the shared corpus formula alone.
    #[test]
    fn inquisitor_track_bonus_matches_the_real_max_level_half_one_formula() {
        for (level, expected) in [(1, 1), (2, 1), (3, 1), (4, 2), (10, 5), (20, 10)] {
            assert_eq!(
                super::inquisitor_track_bonus(level),
                expected,
                "level {level} Track bonus"
            );
        }
    }

    /// These three flat facts never claim-block, regardless of Judgment's
    /// own state -- proven against a not-judging posture (the cheapest
    /// posture to construct) to isolate them from Judgment's own gating.
    #[test]
    fn flat_named_facts_never_claim_block_even_while_not_judging() {
        let input = human_inquisitor_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        for id in [
            "class_feature.apg.inquisitor.monster_lore_bonus",
            "class_feature.apg.inquisitor.cunning_initiative_bonus",
            "class_feature.apg.inquisitor.track_bonus",
        ] {
            assert!(
                receipt.computation.explanations.iter().any(|e| e.id == id),
                "expected {id} to be grounded even while not judging: {:?}",
                receipt.computation.explanations
            );
        }
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("monster_lore")
                    || d.id.contains("cunning_initiative")
                    || d.id.contains("track_bonus")),
            "the three flat facts must never claim-block: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A non-Inquisitor character carrying a spoofed `"judgment"`
    /// activation entry (plus a spoofed Justice choice) must have it
    /// silently ignored, not applied -- the class-ownership gate is by
    /// construction (`active_inquisitor_justice_judgment_bonus` only ever
    /// reads `class_ability_activations`/`selected_choices` after
    /// confirming `class_levels` contains Inquisitor), not a bolt-on
    /// rejection. Also proves Fighter's own golden path is unaffected.
    #[test]
    fn non_inquisitor_characters_spoofed_judgment_activation_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_JUDGMENT_CHOICE_ID.to_owned(),
            selection_id: INQUISITOR_JUDGMENT_JUSTICE_SELECTION_ID.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: INQUISITOR_JUDGMENT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Inquisitor judgment entry: \
             {:?}",
            receipt.computation.diagnostics
        );

        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        assert_eq!(
            melee_attack_bonus.value, 6,
            "a non-Inquisitor character's spoofed judgment entry must never apply a bonus: {:?}",
            melee_attack_bonus
        );
    }

    /// Task #64: an Inquisitor whose Domain class feature selected Good and who is
    /// actively, validly using Touch of Good genuinely gets the sacred bonus applied to
    /// her real integrated melee attack bonus -- proving `active_touch_of_good_bonus`'s
    /// generalization beyond Cleric-only is wired for real, not just a claim.
    ///
    /// **Updated 2026-07-29 (Path A canonical narrowing).** This test used to assert
    /// Inquisitor stays `Blocked` here, because `other_features_deferred` fired
    /// unconditionally. That was a statement about the diagnostic's wiring, not about
    /// what the engine could honestly compute: with the one canonical, corpus-verified
    /// Inquisitor domain recorded, every claim this engine makes about her IS computed,
    /// so the deferred remainder is now named without blocking (Arcanist's own
    /// `exploits_deferred` shape). The bare-Inquisitor case is unchanged and still
    /// asserted by `inquisitor_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one`
    /// immediately above.
    #[test]
    fn single_class_inquisitor_with_good_domain_touch_of_good_active_applies_real_bonus() {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_DOMAIN_CHOICE_ID.to_owned(),
            selection_id: GOOD_DOMAIN_SELECTION.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: TOUCH_OF_GOOD_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "with the canonical Good domain recorded, Inquisitor's remaining gaps are named \
             without blocking: {:?}",
            receipt.computation.diagnostics
        );

        let self_application = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.good_touch_of_good_self_application")
            .expect("Touch of Good self-application must be grounded");
        assert_eq!(
            self_application.value, 1,
            "Inquisitor level 1 Touch of Good bonus: max(1/2, 1) = 1: {self_application:?}"
        );

        let uses_per_day = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.good_touch_of_good_uses_per_day")
            .expect("Touch of Good uses per day must be grounded");
        assert_eq!(
            uses_per_day.value, 4,
            "3 + WIS modifier (+1 from the fixture's WIS 12) = 4: {uses_per_day:?}"
        );

        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        assert!(
            melee_attack_bonus.detail.contains("Good domain Touch of Good sacred bonus"),
            "expected Touch of Good to be named in the melee attack bonus explanation: \
             {melee_attack_bonus:?}"
        );
    }

    /// SD-31 wave 26: an Inquisitor who selected Glory (newly widened past
    /// Good/War/Strength) and is actively using Touch of Glory reaches
    /// `Computed` through the SAME generic catalog loop
    /// `ground_or_block_inquisitor_domain_power` already used for Good/War/
    /// Strength -- this test needed no change to that function, only to
    /// the shared catalog it reads.
    #[test]
    fn single_class_inquisitor_with_glory_domain_touch_of_glory_active_applies_real_bonus() {
        let mut input = human_inquisitor_input(9);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_DOMAIN_CHOICE_ID.to_owned(),
            selection_id: GLORY_DOMAIN_SELECTION.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: TOUCH_OF_GLORY_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "with the Glory domain recorded, Inquisitor's remaining gaps are named without \
             blocking: {:?}",
            receipt.computation.diagnostics
        );

        let self_application = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.glory_touch_of_glory_self_application")
            .expect("Touch of Glory self-application must be grounded");
        assert_eq!(
            self_application.value, 9,
            "Inquisitor level 9 Touch of Glory bonus: the bare level, unhalved = 9: \
             {self_application:?}"
        );
    }

    /// SD-34 wave 37 lane A: an Inquisitor who selected Undead Subdomain (a
    /// real, legal Inquisitor domain -- `advanced_players_guide/class_feature/
    /// inquisitor/inquisitor_domains.json` carries
    /// `DEFINE:InquisitorDomainUndeadSubdomain|0`, confirmed by direct corpus
    /// read) reaches `Computed` through the SAME generic catalog loop, but
    /// -- unlike every domain above -- Death's Kiss's own `grounds_self_
    /// application: false` means NO "self_application"/"not_active" bonus
    /// explanation is ever emitted for it, even with an activation entry
    /// present: only its real uses-per-day is grounded.
    #[test]
    fn single_class_inquisitor_with_undead_subdomain_grounds_uses_per_day_only() {
        let mut input = human_inquisitor_input(4);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_DOMAIN_CHOICE_ID.to_owned(),
            selection_id: UNDEAD_SUBDOMAIN_SELECTION.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: DEATH_S_KISS_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "with Undead Subdomain recorded, Inquisitor's remaining gaps are named without \
             blocking: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.domain.undead_subdomain_death_s_kiss_self_application"
                    || e.id == "class_feature.domain.undead_subdomain_death_s_kiss_not_active"),
            "Death's Kiss must never surface a self_application/not_active bonus explanation \
             for Inquisitor either: {:?}",
            receipt.computation.explanations
        );
        let uses_per_day = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.undead_subdomain_death_s_kiss_uses_per_day")
            .expect("Death's Kiss uses-per-day explanation must be grounded");
        assert_eq!(
            uses_per_day.value, 4,
            "3 + WIS modifier (+1 from the fixture's WIS 12) = 4: {uses_per_day:?}"
        );
    }

    /// SD-34 wave 38 lane A: an Inquisitor who selected Construct Subdomain
    /// (a real, legal Inquisitor domain --
    /// `advanced_players_guide/class_feature/inquisitor/inquisitor_domains.json`
    /// carries `DEFINE:InquisitorDomainConstructSubdomain|0`, confirmed by
    /// direct corpus read) reaches `Computed` through the SAME generic
    /// catalog loop as Undead Subdomain, but its uses-per-day comes from
    /// its OWN `uses_per_day_formula` override (`DomainArtificeLVL/4-1`),
    /// NOT the shared `3+WIS` chain -- proven the same way the Cleric-side
    /// test above proves it, by using a class level where the two formulas
    /// disagree.
    #[test]
    fn single_class_inquisitor_with_construct_subdomain_grounds_its_own_uses_per_day_formula() {
        let mut input = human_inquisitor_input(12);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_DOMAIN_CHOICE_ID.to_owned(),
            selection_id: CONSTRUCT_SUBDOMAIN_SELECTION.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: ANIMATE_SERVANT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "with Construct Subdomain recorded, Inquisitor's remaining gaps are named without \
             blocking: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.domain.construct_subdomain_animate_servant_self_application"
                    || e.id == "class_feature.domain.construct_subdomain_animate_servant_not_active"),
            "Animate Servant must never surface a self_application/not_active bonus \
             explanation for Inquisitor either: {:?}",
            receipt.computation.explanations
        );
        let uses_per_day = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.construct_subdomain_animate_servant_uses_per_day")
            .expect("Animate Servant uses-per-day explanation must be grounded");
        // DomainArtificeLVL/4-1 at Inquisitor level 12: 12/4-1 = 2. The
        // fixture's own Wisdom modifier is +1, so the shared 3+WIS formula
        // would give 4 -- a DIFFERENT value, proving the override, not the
        // shared formula, genuinely computed this number.
        assert_eq!(
            uses_per_day.value, 2,
            "DomainArtificeLVL/4-1 at level 12 = 2, NOT the shared 3+WIS value of 4: \
             {uses_per_day:?}"
        );
    }

    /// An Inquisitor with Good domain selected but NOT currently using Touch of Good (no
    /// active `class_ability_activations` entry) is a genuinely valid posture too: the
    /// "not active" record grounds with a zero value, and no bonus is applied anywhere.
    #[test]
    fn single_class_inquisitor_with_good_domain_not_using_touch_of_good_applies_no_bonus() {
        let mut input = human_inquisitor_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INQUISITOR_DOMAIN_CHOICE_ID.to_owned(),
            selection_id: GOOD_DOMAIN_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        let not_active = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.good_touch_of_good_not_active")
            .expect("Touch of Good not-active record must be grounded");
        assert_eq!(not_active.value, 0, "{not_active:?}");

        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.domain.good_touch_of_good_self_application"),
            "no self-application record may exist without an active activation: {:?}",
            receipt.computation.explanations
        );

        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        assert!(
            !melee_attack_bonus.detail.contains("Good domain Touch of Good sacred bonus (+1"),
            "an inactive Touch of Good must not apply a nonzero bonus: {melee_attack_bonus:?}"
        );
    }

    /// An Inquisitor with no domain selected at all stays claim-blocked on the domain
    /// powers burden, and no Touch of Good record is fabricated -- the catch-all
    /// preserves the honest blocked posture exactly as before this task's slice.
    #[test]
    fn single_class_inquisitor_without_good_domain_selected_stays_blocked_on_domain_powers() {
        let input = human_inquisitor_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.inquisitor.domain_powers.unsupported"
                    && d.claim_blocking),
            "expected the domain-powers blocker to fire with no domain chosen: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.domain.good_touch_of_good")),
            "no Touch of Good record may be fabricated without a Good domain selection: {:?}",
            receipt.computation.explanations
        );
    }

    /// Task #64 negative control: a Druid character carrying a spoofed
    /// `choice:inquisitor_domain -> domain:good` selection AND a spoofed active
    /// `"touch_of_good"` activation must never receive the bonus -- `DRUID_CLASS_ID` is
    /// simply not one of the two class ids `active_touch_of_good_bonus` recognizes (task
    /// #64 confirmed, against two independent primary sources, that Nature Bond's real
    /// domain option can never legally be Good in the first place -- see
    /// `TOUCH_OF_GOOD_ABILITY_ID`'s own doc comment), so this proves the exclusion holds
    /// even under a maximally-adversarial spoofed input, not merely "nothing wires it."
    #[test]
    fn druid_characters_can_never_receive_touch_of_good_even_when_spoofed() {
        let result = load_character_input_fixture(
            "case_id=pf1-crb-human-druid-touch-of-good-spoof-negative-control\n\
             source_package_id=pf1.core_rulebook\n\
             race_id=race:human\n\
             class_level=class:druid:5\n\
             ability=strength:10\n\
             ability=dexterity:12\n\
             ability=constitution:13\n\
             ability=intelligence:8\n\
             ability=wisdom:17\n\
             ability=charisma:12\n\
             choice=choice:inquisitor_domain:domain:good\n",
        );
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: TOUCH_OF_GOOD_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        assert_eq!(super::active_touch_of_good_bonus(&input), None);
    }
}

/// Task #47 (2026-07-28): tests Inquisitor's own spontaneous known-spell
/// posture, mirroring `oracle_dispatch_widening_safety_tests`'s own shape
/// exactly (`unmet_inquisitor_known_spell_conditions`/
/// `inquisitor_spells_known_table`/`ground_inquisitor_known_spells` mirror
/// Oracle's `unmet_oracle_known_spell_conditions`/
/// `oracle_spells_known_table`/`ground_oracle_known_spells`).
#[cfg(test)]
mod inquisitor_known_spell_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, CharacterClassLevel, CharacterInput,
        HeadlessReceiptStatus, INQUISITOR_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, SpellSelection};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_inquisitor_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: INQUISITOR_CLASS_ID.to_owned(), level }];
        input
    }

    /// A bare single-class Human Inquisitor (no known spells) does not
    /// trip the known_spells diagnostic -- zero known spells is itself a
    /// valid posture, mirroring Sorcerer's/Oracle's own reasoning.
    #[test]
    fn single_class_inquisitor_bare_does_not_trip_known_spells_diagnostic() {
        let input = human_inquisitor_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.inquisitor.known_spells.unsupported"),
            "zero known spells is itself a valid posture, mirroring Sorcerer's/Oracle's own \
             reasoning: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Human Inquisitor with a real known spell (verified
    /// against the real, independently re-derived Inquisitor spell list)
    /// grounds the known-spell posture for real and clears the
    /// known_spells diagnostic, but stays `Blocked` on
    /// other_features_deferred regardless. "Bane" (the spell, not the
    /// class feature of the same name) is a real level-1 Inquisitor spell.
    #[test]
    fn single_class_inquisitor_with_a_real_known_spell_grounds_it_and_stays_blocked_elsewhere() {
        let mut input = human_inquisitor_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Bane".to_owned(),
            source_class_id: INQUISITOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.inquisitor.known_spells.unsupported"),
            "the known_spells diagnostic must not fire once a real known spell is recorded: {:?}",
            receipt.computation.diagnostics
        );
        let known = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.apg.inquisitor.known_spells")
            .expect("known-spell count must be grounded");
        assert_eq!(known.value, 1, "Inquisitor level 1 with one known spell: {:?}", known);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Inquisitor still stays Blocked on other_features_deferred: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// An Inquisitor known spell not on the real, independently re-derived
    /// Inquisitor spell list is a genuine posture violation and must
    /// claim-block via known_spells.
    #[test]
    fn single_class_inquisitor_with_an_off_list_known_spell_stays_blocked_on_known_spells() {
        let mut input = human_inquisitor_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Definitely Not A Real Spell".to_owned(),
            source_class_id: INQUISITOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.apg.inquisitor.known_spells.unsupported"
                    && d.claim_blocking),
            "expected the known_spells claim-blocking diagnostic for an off-list spell: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// An Inquisitor over-known at a spell level (more distinct known
    /// spells at that level than the real Spells Known table allows) is a
    /// genuine posture violation and must claim-block via known_spells.
    /// Level 1 caps at 4 orisons (level 0) / 2 first-level spells; five
    /// distinct level-1 spells exceeds that cap.
    #[test]
    fn single_class_inquisitor_over_known_at_a_spell_level_stays_blocked_on_known_spells() {
        let mut input = human_inquisitor_input(1);
        for spell_id in ["Bane", "Bless", "Cause Fear", "Command", "Divine Favor"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: INQUISITOR_CLASS_ID.to_owned(),
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
                .any(|d| d.id == "class_spell.apg.inquisitor.known_spells.unsupported"
                    && d.claim_blocking
                    && d.message.contains("over-known")),
            "expected the over-known claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }
}

/// v0.6 alpha swarm, risks item 8 (Warpriest full-build closure, sixth
/// ACG/APG class-specific closure): tests the real prepared-spellbook
/// grounding and the Destruction Blessing self-application closure
/// directly, mirroring the Arcanist/Cleric dispatch-widening test
/// modules' own shape.
#[cfg(test)]
mod warpriest_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, ActiveState, CharacterClassLevel,
        CharacterInput, HeadlessReceiptStatus, DESTRUCTION_BLESSING_SELECTION, FIGHTER_CLASS_ID,
        STRENGTH_BLESSING_SELECTION, WARPRIEST_BLESSING_CHOICE_ID,
        WARPRIEST_CHANNEL_ENERGY_LEVEL, WARPRIEST_CLASS_ID,
        WARPRIEST_DESTRUCTIVE_ATTACKS_ABILITY_ID, WARPRIEST_FERVOR_LEVEL,
        WARPRIEST_SACRED_ARMOR_LEVEL, WARPRIEST_STRENGTH_SURGE_ABILITY_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_warpriest_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: WARPRIEST_CLASS_ID.to_owned(), level }];
        input
    }

    /// A single-class Human Warpriest who has not chosen the Destruction
    /// Blessing is a genuinely valid PF1 posture only for the OTHER
    /// blessing types (unbuilt), so the blessing-powers diagnostic
    /// claim-blocks -- stays `Blocked`, never the retired generic
    /// diagnostic, with the Blessings uses-per-day/DC and Sacred Weapon
    /// base die grounded regardless (unconditional class features).
    #[test]
    fn single_class_warpriest_without_destruction_blessing_stays_blocked_on_blessing_powers() {
        let input = human_warpriest_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Warpriest must stay Blocked without a recognized Destruction Blessing choice: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.unsupported"),
            "the retired generic diagnostic must never appear for Warpriest: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.blessing_powers.unsupported"
                    && d.claim_blocking),
            "expected the blessing_powers claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );

        let uses = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.warpriest.blessing_uses_per_day")
            .expect("Blessings uses per day must ground unconditionally");
        assert_eq!(uses.value, 3, "Warpriest level 1 Blessing uses: 1/2 + 3 = 3: {:?}", uses);

        let dc = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.warpriest.blessing_dc")
            .expect("Blessings DC must ground unconditionally");
        // Fixture Wisdom 12 (+1 modifier): 1/2 + 10 + 1 = 11.
        assert_eq!(dc.value, 11, "Warpriest level 1 Blessing DC: 0 + 10 + 1 = 11: {:?}", dc);

        let sacred_weapon = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.warpriest.sacred_weapon_base_damage_die")
            .expect("Sacred Weapon base damage die must ground unconditionally");
        assert_eq!(
            sacred_weapon.value, 6,
            "Warpriest level 1 Sacred Weapon base die: 1d6: {:?}",
            sacred_weapon
        );
    }

    /// A single-class Human Warpriest with the Destruction Blessing
    /// chosen but not currently using Destructive Attacks is a genuinely
    /// valid posture -- stays `Blocked` on its prepared spellbook alone
    /// (never blessing_powers, never the retired generic diagnostic),
    /// with the honest "not active" recognition record grounded.
    ///
    /// **Updated (v0.6 alpha swarm, Warpriest spellcasting-shaped
    /// closure)**: this used to assert `other_features_deferred` was
    /// claim-blocking here. A recognized Blessing now downgrades it to a
    /// non-blocking note (the canonical-narrowing shape Arcanist's own
    /// Metamagic Knowledge established), so the assertion is inverted
    /// rather than dropped -- the diagnostic must still be PRESENT,
    /// naming the honest remainder, just not claim-blocking. What keeps
    /// this input `Blocked` is now the empty spellbook, which this test
    /// pins explicitly so the reason cannot drift silently.
    #[test]
    fn single_class_warpriest_with_destruction_blessing_not_active_stays_blocked_on_other_features_only()
    {
        let mut input = human_warpriest_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WARPRIEST_BLESSING_CHOICE_ID.to_owned(),
            selection_id: DESTRUCTION_BLESSING_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Warpriest must stay Blocked on other-features/spellbook alone: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.blessing_powers.unsupported"),
            "blessing_powers must not fire once Destruction is recognized: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.other_features_deferred.unsupported"
                    && !d.claim_blocking),
            "expected the other_features_deferred diagnostic, present but NON-blocking once a \
             Blessing is recognized: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.warpriest.prepared_spellbook.unsupported"
                    && d.claim_blocking),
            "the empty spellbook is what keeps this input Blocked: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id
                    == "class_feature.acg.warpriest.destruction_blessing.destructive_attacks_not_active"),
            "expected the honest not-active recognition record: {:?}",
            receipt.computation.explanations
        );
    }

    /// A single-class Human Warpriest actively, validly using Destructive
    /// Attacks (Destruction Blessing recognized) grounds the real morale
    /// bonus as a standalone explanation record -- this codebase computes
    /// no weapon-damage total anywhere to layer it onto.
    ///
    /// Level 1 bonus: max(1, 1/2) = 1.
    #[test]
    fn single_class_warpriest_actively_using_destructive_attacks_grounds_the_real_bonus() {
        let mut input = human_warpriest_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WARPRIEST_BLESSING_CHOICE_ID.to_owned(),
            selection_id: DESTRUCTION_BLESSING_SELECTION.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: WARPRIEST_DESTRUCTIVE_ATTACKS_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        let bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| {
                e.id == "class_feature.acg.warpriest.destruction_blessing.destructive_attacks_self_application"
            })
            .expect("the active Destructive Attacks explanation must be grounded");
        assert_eq!(bonus.value, 1, "Warpriest level 1 Destructive Attacks bonus: max(1,0)=1: {:?}", bonus);
    }

    /// SD-36 Epic E PC4-1 (SD-35 code review): a Warpriest who chose Earth Blessing -- neither
    /// hand-modeled Blessing -- gets a real generically-grounded magnitude from
    /// `push_generic_pool_group_selection_description_magnitude` (Earth's own "Armor of Earth"
    /// member, proven separately by
    /// `warpriest_generic_blessing_description_pass_grounds_a_zero_bonus_var_blessing` at the
    /// same level 8). Before this fix, `blessing_recognized` only ever checked for the literal
    /// Destruction/Strength selection ids, so this exact character got BOTH a genuine grounded
    /// explanation AND the `blessing_powers.unsupported` claim-blocking "no Blessing-power
    /// support is claimed" diagnostic at the same time -- a real contradiction on one receipt.
    /// The fix must resolve the contradiction: recognized (the narrower `unmodeled` note,
    /// non-blocking), never `unsupported`.
    #[test]
    fn warpriest_with_a_generically_grounded_blessing_is_recognized_not_unsupported() {
        let mut input = human_warpriest_input(8);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WARPRIEST_BLESSING_CHOICE_ID.to_owned(),
            selection_id: "blessing:earth".to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.acg.warpriest.blessing_description.generic.earth")),
            "Earth Blessing's own member must still ground generically: {:?}",
            receipt.computation.explanations
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.blessing_powers.unsupported"),
            "a generically-grounded Blessing must never ALSO claim no Blessing-power support: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.blessing_minor_major_powers.unmodeled"
                    && !d.claim_blocking),
            "expected the non-blocking unmodeled note once a Blessing is recognized generically: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Human Warpriest with a real recorded and prepared
    /// spell, plus the Destruction Blessing recognized, grounds the
    /// spellbook for real -- and, as of the v0.6 alpha swarm's
    /// Warpriest spellcasting-shaped closure, now reaches `Computed`:
    /// those are exactly the two seeds `compose_character_input` applies
    /// at creation, so this is the real shipped creation posture. It was
    /// previously `Blocked` on `other_features_deferred`, which a
    /// recognized Blessing now downgrades to a non-blocking note (the
    /// canonical-narrowing shape Arcanist's Metamagic Knowledge
    /// established). The assertion is inverted rather than dropped -- the
    /// diagnostic must still be present, naming the honest remainder.
    ///
    /// Fixture Wisdom 12 (+1 modifier, no bonus spells at spell level 0,
    /// but a real bonus at level 1: (1-1)/4+1 = 1). Level 1 base:
    /// cantrips 3, 1st-level 1 (verified against the raw corpus CAST
    /// rows).
    #[test]
    fn single_class_warpriest_with_a_real_prepared_spell_grounds_the_spellbook_and_stays_blocked_only_on_other_features()
    {
        let mut input = human_warpriest_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WARPRIEST_BLESSING_CHOICE_ID.to_owned(),
            selection_id: DESTRUCTION_BLESSING_SELECTION.to_owned(),
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: WARPRIEST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: WARPRIEST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.warpriest.prepared_spellbook.unsupported"),
            "the prepared_spellbook diagnostic must not fire once a real, valid posture is \
             recorded: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.other_features_deferred.unsupported"
                    && !d.claim_blocking),
            "expected the other_features_deferred diagnostic, present but NON-blocking, even \
             with a valid spellbook: {:?}",
            receipt.computation.diagnostics
        );
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "this is exactly the creation posture compose_character_input seeds, so it must \
             reach Computed: {:?}",
            receipt.computation.diagnostics
        );

        let base_cantrips = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.warpriest.base_spells_per_day.spell_level_0")
            .expect("base cantrips per day must be grounded");
        assert_eq!(base_cantrips.value, 3, "Warpriest level 1 base cantrips: 3: {:?}", base_cantrips);

        let base_first_level = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.warpriest.base_spells_per_day.spell_level_1")
            .expect("base 1st-level spells per day must be grounded");
        assert_eq!(
            base_first_level.value, 1,
            "Warpriest level 1 base 1st-level spells: 1: {:?}",
            base_first_level
        );
    }

    /// The v0.6 spellcasting widening must reach Warpriest end to end, not
    /// merely as a table lookup: a level-20 Warpriest with the same valid
    /// posture grounds the real top-of-table row rather than tripping the
    /// old bounded 1-3 ceiling.
    ///
    /// `acg_classes.lst:410` -- `20 CAST:5,5,5,5,5,5,5`. Warpriest is a
    /// 6-level caster, so spell levels 7-9 must ground no record at all even
    /// at class level 20.
    #[test]
    fn warpriest_level20_grounds_the_full_widened_corpus_spells_per_day_row() {
        let mut input = human_warpriest_input(20);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WARPRIEST_BLESSING_CHOICE_ID.to_owned(),
            selection_id: DESTRUCTION_BLESSING_SELECTION.to_owned(),
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: WARPRIEST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: WARPRIEST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.warpriest.prepared_spellbook.unsupported"),
            "the prepared_spellbook ceiling must not fire at level 20 after the widening: {:?}",
            receipt.computation.diagnostics
        );

        for spell_level in 0..=6i16 {
            let id =
                format!("class_spell.acg.warpriest.base_spells_per_day.spell_level_{spell_level}");
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("Warpriest level 20 must ground {id}"));
            assert_eq!(
                record.value, 5,
                "Warpriest level 20 base spells per day is 5 at every accessible spell level 0-6 \
                 (acg_classes.lst:410): {record:?}"
            );
        }
        for spell_level in 7..=9i16 {
            let id =
                format!("class_spell.acg.warpriest.base_spells_per_day.spell_level_{spell_level}");
            assert!(
                !receipt.computation.explanations.iter().any(|e| e.id == id),
                "Warpriest is a 6-level caster: spell level {spell_level} must ground no \
                 spells-per-day record even at class level 20"
            );
        }
    }

    /// A non-Warpriest character carrying spoofed Warpriest choice/
    /// activation/spell entries must have them silently ignored -- the
    /// class-ownership gate is by construction, not a bolt-on rejection.
    /// Also proves Fighter's own golden path is unaffected.
    #[test]
    fn non_warpriest_characters_spoofed_warpriest_entries_are_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WARPRIEST_BLESSING_CHOICE_ID.to_owned(),
            selection_id: DESTRUCTION_BLESSING_SELECTION.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: WARPRIEST_DESTRUCTIVE_ATTACKS_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by stray Warpriest entries: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.acg.warpriest.")),
            "a non-Warpriest character must never ground any Warpriest explanation: {:?}",
            receipt.computation.explanations
        );
    }

    /// Real bug found and fixed (v0.6 alpha swarm, 2026-07-26, scout's
    /// formula-audit pass, lead independently confirmed): `dice_count`
    /// previously used the `PREBASESIZELT:M` branch's own `/20`
    /// denominator by mistake instead of the applicable `PREBASESIZEEQ:M`
    /// branch's `/15` -- silently wrong for any level 15+ Warpriest
    /// (never caught by this closure's own bounded 1-3 level scope,
    /// which never diverges between the two denominators). Tests the
    /// pure formula directly across the full level range, including the
    /// specific 15-19 band the bug affected, since `dice_count` is only
    /// ever embedded in the explanation's own prose text (never a
    /// separate `.value` field any existing test could have caught).
    #[test]
    fn warpriest_sacred_weapon_dice_count_matches_the_real_eq_m_corpus_branch_at_every_level() {
        for (level, expected_dice_count, expected_dice_size) in [
            (1, 1, 6),
            (4, 1, 6),
            (5, 1, 8),
            (9, 1, 8),
            (10, 1, 10),
            (14, 1, 10),
            (15, 2, 6),
            (17, 2, 6),
            (19, 2, 6),
            (20, 2, 8),
        ] {
            let (dice_count, dice_size) = super::warpriest_sacred_weapon_base_dice(level);
            assert_eq!(
                dice_count, expected_dice_count,
                "level {level} Sacred Weapon dice count: 1+min(1,level/15) = \
                 {expected_dice_count}"
            );
            assert_eq!(
                dice_size, expected_dice_size,
                "level {level} Sacred Weapon dice size: {expected_dice_size}"
            );
        }
    }

    /// Fervor's own two magnitudes across the full 1-20 level range,
    /// re-derived independently from the PF1 Advanced Class Guide's own
    /// prose ("1d6 at 2nd level, plus 1d6 for every three warpriest
    /// levels beyond 2nd") rather than replayed from the corpus formula
    /// this code transcribes -- the Sacred Weapon lesson applied: a
    /// formula and its own restatement agreeing proves nothing.
    #[test]
    fn warpriest_fervor_magnitudes_match_the_real_corpus_record_at_every_level() {
        for level in 1..WARPRIEST_FERVOR_LEVEL {
            assert_eq!(
                super::warpriest_fervor_uses_per_day(level, 1),
                None,
                "level {level} is below Fervor's own grant level, so no pool exists"
            );
            assert_eq!(super::warpriest_fervor_heal_dice(level), None);
        }
        // (level, expected heal dice) -- every step boundary plus both
        // sides of each, independently derived from the published
        // progression, not from the formula.
        for (level, expected_dice) in [
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 2),
            (7, 2),
            (8, 3),
            (10, 3),
            (11, 4),
            (13, 4),
            (14, 5),
            (16, 5),
            (17, 6),
            (19, 6),
            (20, 7),
        ] {
            assert_eq!(
                super::warpriest_fervor_heal_dice(level),
                Some(expected_dice),
                "level {level} Fervor heal dice: {expected_dice}d6"
            );
        }
        for (level, wisdom, expected_uses) in
            [(2, 1, 2), (2, 0, 1), (5, 1, 3), (5, 3, 5), (20, 1, 11), (20, -1, 9)]
        {
            assert_eq!(
                super::warpriest_fervor_uses_per_day(level, wisdom),
                Some(expected_uses),
                "level {level} Fervor uses at Wisdom modifier {wisdom:+}: level/2 + WIS"
            );
        }
    }

    /// Sacred Armor's enhancement bonus across the full level range,
    /// independently derived from the published progression (+1 at 7th,
    /// +1 per three levels after, maximum +5) rather than from the
    /// corpus formula. Note the formula's `max(0, .../3)` nests the
    /// division INSIDE the max, the opposite of Fervor's own dice
    /// formula -- this test pins the real per-level values so a future
    /// transcription cannot quietly swap the two nestings.
    #[test]
    fn warpriest_sacred_armor_magnitudes_match_the_real_corpus_record_at_every_level() {
        for level in 1..WARPRIEST_SACRED_ARMOR_LEVEL {
            assert_eq!(
                super::warpriest_sacred_armor_enhancement(level),
                None,
                "level {level} is below Sacred Armor's own grant level"
            );
            assert_eq!(super::warpriest_sacred_armor_uses_per_day(level), None);
        }
        for (level, expected_enhancement) in [
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 2),
            (12, 2),
            (13, 3),
            (15, 3),
            (16, 4),
            (18, 4),
            (19, 5),
            (20, 5),
        ] {
            assert_eq!(
                super::warpriest_sacred_armor_enhancement(level),
                Some(expected_enhancement),
                "level {level} Sacred Armor enhancement: +{expected_enhancement}"
            );
            assert_eq!(
                super::warpriest_sacred_armor_uses_per_day(level),
                Some(i16::from(level)),
                "level {level} Sacred Armor uses per day equal the warpriest level"
            );
        }
        assert_eq!(
            super::warpriest_sacred_armor_enhancement(20),
            Some(5),
            "the record's own DESC caps the enhancement at +5; the formula must not exceed it"
        );
    }

    /// Channel Energy's save DC is keyed on Wisdom (the authoritative
    /// `BONUS:VAR` on the Channel Energy record itself), not the
    /// Charisma the class's separate positive/negative display records
    /// inherited from the Cleric original.
    #[test]
    fn warpriest_channel_energy_dc_matches_the_real_corpus_record_at_every_level() {
        for level in 1..WARPRIEST_CHANNEL_ENERGY_LEVEL {
            assert_eq!(
                super::warpriest_channel_energy_dc(level, 1),
                None,
                "level {level} is below Channel Energy's own grant level"
            );
        }
        for (level, wisdom, expected_dc) in
            [(4, 1, 13), (4, 0, 12), (5, 1, 13), (6, 1, 14), (20, 1, 21), (20, 5, 25)]
        {
            assert_eq!(
                super::warpriest_channel_energy_dc(level, wisdom),
                Some(expected_dc),
                "level {level} Channel Energy DC at Wisdom modifier {wisdom:+}: 10 + level/2 + WIS"
            );
        }
    }

    /// Strength Surge's magnitude is byte-identical to Destructive
    /// Attacks' own `max(1,WarpriestLVL/2)`, including the minimum-1
    /// floor at level 1.
    #[test]
    fn warpriest_strength_surge_bonus_matches_destructive_attacks_at_every_level() {
        for level in 1..=20u8 {
            assert_eq!(
                super::warpriest_strength_surge_bonus(level),
                super::warpriest_destructive_attacks_bonus(level),
                "level {level}: both minor powers share the same corpus formula"
            );
        }
        for (level, expected) in [(1, 1), (2, 1), (3, 1), (4, 2), (20, 10)] {
            assert_eq!(
                super::warpriest_strength_surge_bonus(level),
                expected,
                "level {level} Strength Surge: max(1, level/2) = {expected}"
            );
        }
    }

    /// End-to-end through the real pipeline: a level-1 Warpriest is
    /// below every one of the three new features' grant levels, so each
    /// is named with an honest "correctly absent" record rather than
    /// silently omitted, and none of them claim-blocks.
    #[test]
    fn level_1_warpriest_names_fervor_and_sacred_armor_as_correctly_absent() {
        let receipt = build_pilot_headless_receipt(&human_warpriest_input(1));

        for id in [
            "class_feature.acg.warpriest.fervor_uses_per_day",
            "class_feature.acg.warpriest.sacred_armor_enhancement",
        ] {
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("{id} must be named even below its grant level"));
            assert_eq!(record.value, 0, "{id} must claim no magnitude below its gate: {record:?}");
            assert!(
                record.detail.contains("correctly absent"),
                "{id} must say why it is absent: {record:?}"
            );
        }
        for id in [
            "class_feature.acg.warpriest.fervor_heal_dice",
            "class_feature.acg.warpriest.channel_energy_dc",
            "class_feature.acg.warpriest.sacred_armor_uses_per_day",
        ] {
            assert!(
                !receipt.computation.explanations.iter().any(|e| e.id == id),
                "{id} must not be grounded at all below its own grant level"
            );
        }
    }

    /// End-to-end through the real pipeline at level 7, the first level
    /// at which all three new features are simultaneously live. Fixture
    /// Wisdom is 12 (+1 modifier).
    #[test]
    fn level_7_warpriest_grounds_fervor_channel_energy_and_sacred_armor() {
        let receipt = build_pilot_headless_receipt(&human_warpriest_input(7));

        for (id, expected) in [
            // 7/2 + 1 = 4
            ("class_feature.acg.warpriest.fervor_uses_per_day", 4),
            // 1 + max(0, 7-2)/3 = 1 + 1 = 2
            ("class_feature.acg.warpriest.fervor_heal_dice", 2),
            // 10 + 7/2 + 1 = 14
            ("class_feature.acg.warpriest.channel_energy_dc", 14),
            // 1 + max(0, (7-7)/3) = 1
            ("class_feature.acg.warpriest.sacred_armor_enhancement", 1),
            ("class_feature.acg.warpriest.sacred_armor_uses_per_day", 7),
        ] {
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("{id} must ground at level 7"));
            assert_eq!(record.value, expected, "{id} at level 7: {record:?}");
        }

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("fervor")
                    || d.id.contains("channel_energy")
                    || d.id.contains("sacred_armor")),
            "none of the three new features may introduce a diagnostic of its own: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.other_features_deferred.unsupported"
                    && (d.message.contains("Fervor,")
                        || d.message.contains("Channel Energy,")
                        || d.message.contains("Sacred Armor,")))
            ,
            "the deferred diagnostic must no longer list the three features this slice grounded: \
             {:?}",
            receipt.computation.diagnostics
        );
    }

    /// task #88 correction: Sacred Armor's own detail string used to claim
    /// "this engine computes no player armor-class total that an armor
    /// enhancement bonus could layer onto" -- false: `is_supported_warpriest_single_class`
    /// is part of `has_supported_class_chassis`, so a GE-06-posture Warpriest
    /// reaches the same `defense.baseline_armor_class` pillar every other
    /// supported class does (Brawler's own AC Bonus already integrates into
    /// it). Sacred Armor's magnitude simply isn't wired into it. Proves both
    /// halves: the total really is computed, and Sacred Armor's bonus really
    /// is absent from it.
    #[test]
    fn warpriest_sacred_armor_detail_no_longer_falsely_claims_no_ac_total_exists() {
        let receipt = build_pilot_headless_receipt(&human_warpriest_input(7));

        let sacred_armor = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.warpriest.sacred_armor_enhancement")
            .expect("Sacred Armor enhancement must ground at level 7");
        assert!(
            !sacred_armor.detail.contains("computes no player"),
            "the corrected detail must not repeat the false no-total-exists claim: {:?}",
            sacred_armor
        );
        assert!(
            sacred_armor.detail.contains("defense.baseline_armor_class"),
            "the corrected detail must name the real AC total it isn't wired into: {:?}",
            sacred_armor
        );

        let baseline_ac = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .expect(
                "a GE-06-posture Warpriest is a supported class chassis, so baseline AC must be \
                 real, not absent",
            );
        assert!(
            !baseline_ac.detail.contains("Sacred Armor"),
            "Sacred Armor's +1 must NOT be folded into baseline AC yet -- the corrected claim \
             says 'not wired in', not 'wired in': {:?}",
            baseline_ac
        );
    }

    /// A Warpriest whose ONLY recognized Blessing is Strength must no
    /// longer hit the blessing-powers claim-blocking diagnostic -- the
    /// gate now admits either canonical Blessing, not Destruction alone.
    #[test]
    fn single_class_warpriest_with_strength_blessing_alone_clears_the_blessing_powers_block() {
        let mut input = human_warpriest_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WARPRIEST_BLESSING_CHOICE_ID.to_owned(),
            selection_id: STRENGTH_BLESSING_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.blessing_powers.unsupported"),
            "a recognized Strength Blessing must clear the blessing-powers block: {:?}",
            receipt.computation.diagnostics
        );
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| {
                e.id == "class_feature.acg.warpriest.strength_blessing.strength_surge_not_active"
            })
            .expect("the honest not-active recognition record must ground");
        assert_eq!(record.value, 0, "no bonus may be claimed while inactive: {record:?}");
    }

    /// A Warpriest actively using Strength Surge grounds the real flat
    /// magnitude. Level 4 is used deliberately: `max(1, 4/2) = 2`
    /// distinguishes the real formula from the minimum-1 floor that
    /// level 1 alone would not.
    #[test]
    fn single_class_warpriest_actively_using_strength_surge_grounds_the_real_bonus() {
        let mut input = human_warpriest_input(4);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WARPRIEST_BLESSING_CHOICE_ID.to_owned(),
            selection_id: STRENGTH_BLESSING_SELECTION.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: WARPRIEST_STRENGTH_SURGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.warpriest.strength_blessing.strength_surge_active")
            .expect("an active Strength Surge must ground its real bonus");
        assert_eq!(record.value, 2, "level 4 Strength Surge: max(1, 4/2) = 2: {record:?}");
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id
                    == "class_feature.acg.warpriest.strength_blessing.strength_surge_not_active"),
            "the active and not-active records are mutually exclusive: {:?}",
            receipt.computation.explanations
        );
    }

    /// A non-Warpriest carrying a spoofed Strength Blessing choice and
    /// activation must ground nothing -- the same dispatch-safety proof
    /// the Destruction Blessing entries already carry.
    #[test]
    fn non_warpriest_characters_spoofed_strength_surge_entries_are_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WARPRIEST_BLESSING_CHOICE_ID.to_owned(),
            selection_id: STRENGTH_BLESSING_SELECTION.to_owned(),
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: WARPRIEST_STRENGTH_SURGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.acg.warpriest.")),
            "a non-Warpriest must never ground any Warpriest explanation: {:?}",
            receipt.computation.explanations
        );
    }
}

