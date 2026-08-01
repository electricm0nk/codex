//! Feat-effects engine (v0.6 alpha swarm item 17, un-deferred by operator
//! directive 2026-07-24: "easy enough, you need to create a feats engine").
//!
//! Before this file, no feat except the three hardcoded into the fixed
//! creation-time loadout (Power Attack, Dodge, Weapon Focus) had any
//! computed mechanical effect anywhere in this crate -- QA verified
//! concretely (not reasoned abstractly): a Fighter with Toughness added to
//! `selected_feats` stayed at HP 12 instead of the correct 15, with zero
//! explanations mentioning it (`risks-and-open-questions.md` item 17).
//! Every other selectable feat in the 185-record CRB catalog was purely
//! recorded data with zero computed consequence.
//!
//! Deliberately scoped to exactly one feat for the first slice, not a
//! general resolver for all 185 (operator directive: land one real feat,
//! prove the pattern, then widen). [`hp_bonus_from_feats`] is shaped to
//! generalize without rework -- future feats extend the same "scan
//! `selected_feats`, sum real effects" idiom, one `if`/match arm at a time,
//! the same way this crate's other additive computations
//! (`skill_allocation.rs`'s class-skill lists, `durability.rs`'s per-class
//! hit-die table) started with one recognized case and grew.
//!
//! Widened (v0.6 alpha swarm, 2026-07-24) to the three CRB save-boosting
//! feats -- Great Fortitude, Iron Will, Lightning Reflexes -- proving the
//! pattern extends past a single feat. Deliberately NOT widened to Skill
//! Focus in the same pass: its own catalog record's `effect` qualifiers
//! (`["SKILL", "%LIST", "3", "TYPE=SkillFocus"]`) carry a `%LIST`
//! placeholder, PCGen's own marker for a player-chosen target -- unlike the
//! three save feats (each targets one fixed, named save) or Toughness (no
//! target at all), which skill Skill Focus boosts is a per-character choice
//! `selected_feats` has no slot to record (the same shape of gap Weapon
//! Focus's own `"feat:weapon_focus:weapon:longsword"` compound id works
//! around for the one hardcoded fixed-loadout case, but no general
//! chosen-target mechanism exists for a catalog-picked feat). Left for a
//! future slice, not guessed at here.

use crate::rules_core::character_input::SelectedChoice;
use crate::rules_core::feat_identity;

/// The complete set of feats a character actually possesses: everything the
/// player chose (`selected_feats`) plus everything their class granted
/// automatically.
///
/// **Why this exists.** Every producer in this module keys on the real catalog
/// string in `selected_feats`, which is correct for player-chosen feats but
/// silently misses feats a class hands out for free. Two such grants exist in
/// this codebase today, and both hit producers already shipped here:
/// - **Ranger gains Endurance automatically at 3rd level** ("A ranger gains
///   Endurance as a bonus feat", `RANGER_ENDURANCE_LEVEL`) -- so
///   [`endurance_check_bonus_from_feats`] would return `0` for every Ranger who
///   genuinely has the feat.
/// - **Monk gains Improved Unarmed Strike and Stunning Fist automatically at
///   1st level** ("even if he does not meet the prerequisites") -- so
///   [`stunning_fist_facts_from_feats`] would return `None` for the very
///   characters the feat matters most for.
///
/// Neither grant is represented as data anywhere today; both live only as prose
/// inside explanation strings, which is why the gap was invisible. Callers pass
/// the granted keys in explicitly: this module stays a dependency-free leaf and
/// never needs to know which class grants what at which level -- that knowledge
/// stays in the compute layer that already owns the level gates.
///
/// **Ordering and duplicate semantics, both load-bearing.** `selected_feats` is
/// passed through *verbatim*, duplicates included, because
/// [`base_speed_bonus_from_feats`] counts repeated Fleet picks as a real
/// cumulative bonus -- deduplicating the whole list would silently halve a
/// genuine +10 base speed to +5. Granted feats are appended in the order given,
/// and each is added only if not already present, so a Ranger who also picked
/// Endurance from the catalog does not end up holding it twice.
pub fn effective_feats(selected_feats: &[String], class_granted_feats: &[&str]) -> Vec<String> {
    let mut feats = selected_feats.to_vec();
    for granted in class_granted_feats {
        if !feat_identity::holds(&feats, granted) {
            feats.push((*granted).to_owned());
        }
    }
    feats
}

/// The real PF1 Core Rulebook feat catalog's key for Toughness
/// (`rules_tables::crb::feat_data::general`'s own
/// `FeatTableEntry { key: "Toughness", ... }` record), verified against the
/// real selection pipeline rather than assumed: `FeatCatalogEntryDto.key`
/// (`feat_catalog.rs`) passes the catalog key through verbatim with no
/// transformation, and the frontend's `handleAddFeat`/`handleAddFeatLevelUp`
/// (`CharacterSheet.tsx`) send `entry.key` directly as `addFeatSelection`'s
/// `featId`, which `apply_add_feat_selection` (`pf1_adapter.rs`) pushes onto
/// `selected_feats` unmodified. This is exactly what a real user's
/// `selected_feats` entry contains after picking Toughness through the live
/// Feat picker.
///
/// **The `feat:toughness` spelling names the same feat, and is matched too.**
/// This comment used to insist the `"feat:<name>"` convention was "a separate,
/// bespoke id space that never runs through the catalog at all" -- it is not:
/// `compose_character_input` seeds that shape, and it reaches
/// `add_feat_selection` directly. Producers here match through
/// [`crate::rules_core::feat_identity`]'s fold, so which spelling a character
/// happens to carry never decides whether the feat works.
const TOUGHNESS_FEAT_KEY: &str = "Toughness";

/// The real PF1 Core Rulebook Toughness feat benefit: a flat +3 hit points
/// ("Benefit: You gain +3 hit points.") -- not level-scaled, unlike the
/// D&D 3.5 predecessor version of this feat. Verified against the feat's
/// own catalog record (`feat_data/general.rs`): `FeatEffectBonus`'s
/// `qualifiers` cite `"HP"`/`"CURRENTMAX"`/`"max(3,TL)"`, i.e. the PCGen
/// corpus source itself already encodes this feat as a flat +3 (the
/// `max(3,TL)` qualifier is PCGen's own templating for a level-scaling
/// variant that never actually engages at the levels this crate computes;
/// `TL` -- total feat-granting levels -- never exceeds 1 for any class/
/// level this crate reaches `Computed` for today).
const TOUGHNESS_HP_BONUS: i16 = 3;

/// Sums the real, computed hit-point bonus from every feat effect this
/// engine currently grounds, across a character's `selected_feats`. Callers
/// add this to whatever base max-HP total they already have (e.g.
/// `durability::compute_max_hp`'s result) -- this function only contributes
/// the feat-driven delta, it is not itself a full max-HP computation.
/// Returns `0` (not a fabricated value) when no grounded feat is present,
/// or for any feat this engine does not yet ground.
pub fn hp_bonus_from_feats(selected_feats: &[String]) -> i16 {
    let mut bonus = 0;
    if feat_identity::holds(selected_feats, TOUGHNESS_FEAT_KEY) {
        bonus += TOUGHNESS_HP_BONUS;
    }
    bonus
}

/// The real PF1 Core Rulebook catalog keys for the three save-boosting
/// feats this engine grounds, verified against `feat_data/general.rs`
/// exactly the same way `TOUGHNESS_FEAT_KEY` was (`FeatTableEntry.key`,
/// passed through the real selection pipeline unmodified).
const GREAT_FORTITUDE_FEAT_KEY: &str = "Great Fortitude";
const IRON_WILL_FEAT_KEY: &str = "Iron Will";
const LIGHTNING_REFLEXES_FEAT_KEY: &str = "Lightning Reflexes";

/// The real PF1 Core Rulebook benefit shared by all three: a flat +2 to
/// exactly one named save. Verified against each feat's own catalog
/// record: Great Fortitude carries `["SAVE", "Fortitude", "2"]`, Iron Will
/// `["SAVE", "Will", "2"]`, Lightning Reflexes `["SAVE", "Reflex", "2"]` --
/// same magnitude, different target save, no level-scaling or other
/// qualifier on any of the three.
const SAVE_FEAT_BONUS: i16 = 2;

/// One character's feat-derived bonus to each of the three saves. Callers
/// add each field to whatever base total they already have for that save
/// (e.g. `compute_total_saves`'s own Fortitude/Reflex/Will totals) -- this
/// struct only carries the feat-driven delta, not a full save computation.
/// Deliberately not `pilot_compute::BaseSaves` (a distinct, local type) so
/// this module stays a dependency-free leaf, importing nothing from
/// `pilot_compute.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SaveBonusesFromFeats {
    pub fortitude: i16,
    pub reflex: i16,
    pub will: i16,
}

/// Sums the real, computed save bonus from every grounded save-boosting
/// feat, across a character's `selected_feats`. Each of the three fields
/// is `0` (not fabricated) when its feat is absent, or when
/// `selected_feats` carries a feat this engine does not yet ground.
pub fn save_bonuses_from_feats(selected_feats: &[String]) -> SaveBonusesFromFeats {
    SaveBonusesFromFeats {
        fortitude: if feat_identity::holds(selected_feats, GREAT_FORTITUDE_FEAT_KEY) {
            SAVE_FEAT_BONUS
        } else {
            0
        },
        reflex: if feat_identity::holds(selected_feats, LIGHTNING_REFLEXES_FEAT_KEY) {
            SAVE_FEAT_BONUS
        } else {
            0
        },
        will: if feat_identity::holds(selected_feats, IRON_WILL_FEAT_KEY) {
            SAVE_FEAT_BONUS
        } else {
            0
        },
    }
}

/// The real PF1 Core Rulebook catalog keys for the three skill-boosting
/// feats this engine grounds, verified against `feat_data/general.rs`
/// (`Athletic`/`Persuasive`) and `feat_data/combat.rs`
/// (`Intimidating Prowess`) exactly the same way `TOUGHNESS_FEAT_KEY` was
/// (`FeatTableEntry.key`, passed through the real selection pipeline
/// unmodified).
const ATHLETIC_FEAT_KEY: &str = "Athletic";
const PERSUASIVE_FEAT_KEY: &str = "Persuasive";
const INTIMIDATING_PROWESS_FEAT_KEY: &str = "Intimidating Prowess";

/// The real PF1 Core Rulebook benefit shared by Athletic and Persuasive: a
/// flat +2 to each of two named skills ("You get a +2 bonus on Climb and
/// Swim checks" / "...on Diplomacy and Intimidate checks"). Verified against
/// each feat's own catalog record: Athletic carries
/// `["SKILL","Climb","if(skillinfo(\"TOTALRANK\",\"Climb\")>=10,4,2)"]` (and
/// the same on Swim), Persuasive the same shape on Diplomacy/Intimidate. The
/// corpus's `if(TOTALRANK>=10,4,2)` conditional grants +4 only at 10+ ranks in
/// that skill; the deterministic selected-skill posture pins skill rank at 1
/// (`pilot_compute::SELECTED_SKILL_RANK`, and ranks never exceed character
/// level), so `TOTALRANK>=10` is provably false for every character this crate
/// reaches `Computed` for -- the value is +2, not a fabricated simplification,
/// exactly the way `TOUGHNESS_HP_BONUS`'s own `max(3,TL)` resolves to a flat 3
/// at supported levels. The +4 tier is deferred until the engine computes a
/// character with 10+ ranks (well beyond current coverage).
const TWO_SKILL_FEAT_BONUS: i16 = 2;

/// One character's feat-derived bonus to each of the three skills the engine
/// computes today (`pilot_compute::SelectedSkillModifiers`'s own
/// Climb/Intimidate/Swim). Callers add each field to whatever base total they
/// already have for that skill -- this struct only carries the feat-driven
/// delta, not a full skill-modifier computation. Deliberately shaped to those
/// exact three skills (not a general per-skill map) so every field is a value
/// the engine actually consumes -- no unwired half-effects. Persuasive's real
/// +2 Diplomacy half is intentionally absent: Diplomacy is not a computed
/// skill, so grounding it here would be a bonus with no live consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkillBonusesFromFeats {
    pub climb: i16,
    pub intimidate: i16,
    pub swim: i16,
}

/// Sums the real, computed skill bonus from every grounded skill-boosting
/// feat, across a character's `selected_feats`. Each field is `0` (not
/// fabricated) when no feat targeting that skill is present.
///
/// `strength_modifier` is the character's real Strength ability modifier,
/// consumed only by Intimidating Prowess (real CRB benefit: "Add your Strength
/// modifier to Intimidate skill checks in addition to your Charisma modifier",
/// corpus `["SKILL","Intimidate","STR"]`). It is threaded as a plain scalar,
/// not as a `pilot_compute::AbilityModifiers`, so this module stays the same
/// dependency-free leaf `save_bonuses_from_feats` is -- importing nothing from
/// `pilot_compute.rs`; the caller already holds `ability_modifiers.strength`.
/// Applied verbatim (no floor at 0): the corpus token adds the raw modifier,
/// so a negative Strength modifier would reduce Intimidate, and clamping it
/// would fabricate a value the corpus does not specify. Its Str 13 prerequisite
/// makes a negative modifier a non-realistic posture regardless.
pub fn skill_bonuses_from_feats(
    selected_feats: &[String],
    strength_modifier: i16,
) -> SkillBonusesFromFeats {
    let has = |key: &str| feat_identity::holds(selected_feats, key);

    let athletic = has(ATHLETIC_FEAT_KEY);
    let persuasive_intimidate = if has(PERSUASIVE_FEAT_KEY) { TWO_SKILL_FEAT_BONUS } else { 0 };
    let intimidating_prowess_intimidate =
        if has(INTIMIDATING_PROWESS_FEAT_KEY) { strength_modifier } else { 0 };

    SkillBonusesFromFeats {
        climb: if athletic { TWO_SKILL_FEAT_BONUS } else { 0 },
        intimidate: persuasive_intimidate + intimidating_prowess_intimidate,
        swim: if athletic { TWO_SKILL_FEAT_BONUS } else { 0 },
    }
}

/// One real, corpus-verified skill bonus a feat grants to a skill the engine
/// does **not** compute a total for today, grounded as a standalone fact
/// rather than deferred -- mirroring the codebase's established
/// standalone-explanation-record idiom (Track's Survival bonus, Bardic
/// Knowledge, Slayer's/Inquisitor's flat class-feature magnitudes: a real,
/// verified value recorded as its own fact, with integration into a running
/// total treated as a bonus, not a requirement). `feat_effects.rs` (a
/// dependency-free leaf) owns only the verified magnitude data; a caller in the
/// compute layer turns each fact into a `ComputationExplanation` record,
/// explicitly labeled as not wired into any skill total, exactly the way
/// `skill_bonuses_from_feats`'s three computed skills are consumed by
/// `compute_selected_skill_modifiers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StandaloneSkillFeatFact {
    /// The real catalog `key` string, keyed on exactly (no substring/prefix
    /// match), the same way every other feat in this module is.
    pub feat_key: &'static str,
    /// The skill the feat boosts, verbatim from the corpus `BONUS:SKILL|<skill>`
    /// token (`feat_data/general.rs`).
    pub skill_name: &'static str,
    /// The flat bonus magnitude. `TWO_SKILL_FEAT_BONUS` (+2) for every fact
    /// here: each source feat's corpus token is
    /// `if(skillinfo("TOTALRANK",<skill>)>=10,4,2)`, whose +4 tier requires 10+
    /// ranks in the skill -- unreachable for any character this engine
    /// represents, exactly the provable-floor reasoning `TWO_SKILL_FEAT_BONUS`
    /// already documents. The +4 tier is deferred, not fabricated here.
    pub bonus: i16,
}

/// Every standalone two-skill General-feat fact this engine grounds, in the
/// corpus source order of `feat_data/general.rs` (auditable against that file
/// top-to-bottom). Covers the boosted skill(s) of each two-skill feat that the
/// engine does **not** already compute a total for: the eight feats whose
/// *both* skills are uncomputed (Acrobatic, Alertness, Animal Affinity,
/// Deceitful, Deft Hands, Magical Aptitude, Self-Sufficient, Stealthy) plus
/// Persuasive's Diplomacy half (its Intimidate half is a computed skill,
/// grounded by `skill_bonuses_from_feats`). Athletic is absent entirely: both
/// its skills (Climb, Swim) are computed, so it has no standalone remainder.
const STANDALONE_TWO_SKILL_FACTS: &[StandaloneSkillFeatFact] = &[
    StandaloneSkillFeatFact { feat_key: "Acrobatic", skill_name: "Acrobatics", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Acrobatic", skill_name: "Fly", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Alertness", skill_name: "Perception", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Alertness", skill_name: "Sense Motive", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Animal Affinity", skill_name: "Handle Animal", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Animal Affinity", skill_name: "Ride", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Deceitful", skill_name: "Bluff", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Deceitful", skill_name: "Disguise", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Deft Hands", skill_name: "Disable Device", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Deft Hands", skill_name: "Sleight of Hand", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Magical Aptitude", skill_name: "Spellcraft", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Magical Aptitude", skill_name: "Use Magic Device", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Persuasive", skill_name: "Diplomacy", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Self-Sufficient", skill_name: "Heal", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Self-Sufficient", skill_name: "Survival", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Stealthy", skill_name: "Escape Artist", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Stealthy", skill_name: "Stealth", bonus: TWO_SKILL_FEAT_BONUS },
];

/// The Advanced Player's Guide's own standalone skill-feat facts, in the corpus
/// source order of `apg_feats.lst`. Kept as a second table rather than appended
/// to [`STANDALONE_TWO_SKILL_FACTS`] so that table's "two-skill General feat"
/// audit story against `cr_feats.lst` stays exactly true; these are a different
/// book and a different shape (one is a single-skill feat, the other targets two
/// whole skill *categories*).
///
/// **The APG carries exactly two `BONUS:SKILL` feats** -- verified by scanning
/// `apg_feats.lst` for the token, not assumed -- and the ACG carries none at
/// all, so this table plus [`STANDALONE_TWO_SKILL_FACTS`] and the three
/// already-computed skills of [`skill_bonuses_from_feats`] close `BONUS:SKILL`
/// across all three books. (A third APG feat, Sharp Senses, boosts Perception
/// through a `BONUS:VAR` rather than a `BONUS:SKILL` token and needs its own
/// replacement-semantics handling; see
/// [`sharp_senses_perception_bonus_from_feats`].)
///
/// Neither feat's skills are among the three
/// `compute_selected_skill_modifiers` computes (Climb/Intimidate/Swim), so both
/// ground standalone, exactly as the CRB table above does.
///
/// **Breadth of Experience targets categories, not named skills.** Its token is
/// `BONUS:SKILL|TYPE.Knowledge,TYPE.Profession|2` -- PCGen's `TYPE.` prefix means
/// "every skill of this type", and the `BENEFIT:` prose says so outright ("+2
/// bonus on all Knowledge and Profession skill checks"). The two `skill_name`
/// values below transcribe that prose rather than naming any one skill, because
/// naming one would be a fabrication and dropping the feat would discard a real
/// verified magnitude. Its second token
/// (`BONUS:VAR|UseProfessionUntrained,UseKnowledgeUntrained|1|TYPE=Boolean`,
/// letting those skills be used untrained) is a boolean capability with no
/// numeric magnitude, already complete under the standing text-only ruling, and
/// is deliberately not grounded as a number here.
///
/// Both feats' prerequisites (Breadth of Experience's
/// `PRERACE:1,RACESUBTYPE=Dwarf,RACESUBTYPE=Elf,RACESUBTYPE=Gnome` plus its
/// `PRETEXT:100+ years old.`, and Master Alchemist's
/// `PRESKILL:1,Craft (Alchemy)=5`) are NOT asserted here -- prerequisite
/// validation is `feat_prereqs`' job, the same split already documented for
/// Master Craftsman, and both magnitudes are a flat, unconditional `+2` under
/// any reading.
const STANDALONE_APG_SKILL_FACTS: &[StandaloneSkillFeatFact] = &[
    StandaloneSkillFeatFact { feat_key: "Breadth of Experience", skill_name: "all Knowledge skills", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Breadth of Experience", skill_name: "all Profession skills", bonus: TWO_SKILL_FEAT_BONUS },
    StandaloneSkillFeatFact { feat_key: "Master Alchemist", skill_name: "Craft (Alchemy)", bonus: TWO_SKILL_FEAT_BONUS },
];

/// Every grounded standalone skill-feat fact for the feats actually present in
/// `selected_feats`, in the stable corpus order of `STANDALONE_TWO_SKILL_FACTS`
/// followed by [`STANDALONE_APG_SKILL_FACTS`].
/// Returns an empty vec (not fabricated facts) when none of the grounding feats
/// is selected. Keyed on the exact catalog `key` string, so a longer feat whose
/// name merely begins with a grounded key (e.g. "Acrobatic Steps") never
/// matches.
///
/// No two facts across both tables name the same skill, which is load-bearing:
/// the consumer derives its explanation id by slugifying `skill_name`, so a
/// repeated skill would emit two records under one id. Sharp Senses' Perception
/// bonus is deliberately kept out of these tables for exactly that reason -- it
/// would collide with Alertness' Perception fact -- and grounds under its own id
/// instead.
pub fn standalone_skill_facts_from_feats(selected_feats: &[String]) -> Vec<StandaloneSkillFeatFact> {
    STANDALONE_TWO_SKILL_FACTS
        .iter()
        .chain(STANDALONE_APG_SKILL_FACTS)
        .filter(|fact| feat_identity::holds(selected_feats, fact.feat_key))
        .copied()
        .collect()
}

/// Skill Focus's real catalog key (`feat_data/general.rs`), and the Mechanism-B
/// contract for recording its player-chosen skill target: a
/// `SelectedChoice { choice_set_id: "choice:skill_focus_target",
/// selection_id: "skill:<Skill Name>" }` on `chosen.selected_choices`. Chosen
/// over a compound `selected_feats` key ("Skill Focus:skill:X") deliberately:
/// every real `selected_feats` reader (`feat_prereqs`, `pilot_compute_corpus`,
/// this module) matches the plain catalog key by exact equality, so a compound
/// string would silently break any prerequisite lookup requiring plain
/// "Skill Focus". Keeping the target in `selected_choices` leaves `selected_feats`
/// untouched. The `"skill:"` prefix mirrors the codebase's existing
/// `selection_id` conventions (`"school:evocation"`, `"feat:dodge"`); skill
/// names carry spaces but never colons, so a single prefix strip recovers the
/// full name.
const SKILL_FOCUS_FEAT_KEY: &str = "Skill Focus";
const SKILL_FOCUS_TARGET_CHOICE_SET: &str = "choice:skill_focus_target";
const SKILL_FOCUS_SKILL_SELECTION_PREFIX: &str = "skill:";

/// Skill Focus's real PF1 Core Rulebook benefit: a flat +3 to one chosen skill
/// ("+3 bonus on all checks with that skill; if you have 10 or more ranks..., it
/// increases to +6"). Verified against the feat's own catalog record
/// (`["SKILL","%LIST","3","TYPE=SkillFocus"]`). Provably +3 for every character
/// this engine represents (skill ranks never reach 10), the same floor
/// reasoning `TWO_SKILL_FEAT_BONUS` documents; the +6 tier is deferred, not
/// fabricated.
const SKILL_FOCUS_BONUS: i16 = 3;

/// One grounded Skill Focus fact: the real +3 applied to a specific,
/// player-chosen skill. Unlike [`StandaloneSkillFeatFact`] (whose `skill_name`
/// is a compile-time `&'static str` from a fixed table), the target here is a
/// runtime player pick, so `skill_name` is an owned `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillFocusFact {
    /// The chosen skill, verbatim from the `"skill:<name>"` selection.
    pub skill_name: String,
    /// Always [`SKILL_FOCUS_BONUS`] (+3) at levels this engine represents.
    pub bonus: i16,
}

/// Grounds Skill Focus's real +3 as a standalone fact for each explicitly
/// chosen skill target -- but **only** when Skill Focus is actually in
/// `selected_feats` AND a matching `choice:skill_focus_target -> skill:<name>`
/// is present in `selected_choices`. Grounds nothing (never a fabricated
/// canonical skill) when the feat is present without a target choice, when a
/// target choice is orphaned without the feat, or when a choice is malformed /
/// from a different choice set.
///
/// **One fact per distinct skill, in first-seen input order.** Skill Focus is
/// `STACK:NO MULT:YES` in the corpus (verified directly against `cr_feats.lst`):
/// it may be taken more than once, but only for a *different* skill each time --
/// two instances naming the same skill do not stack, so the second grounds
/// nothing rather than a second +3 record. Skill comparison is
/// case-insensitive on purpose: the consumer derives its explanation id by
/// lowercasing this name (`feat.skill_focus_bonus.<skill>`), so two case
/// variants of one skill would otherwise emit two records under a single id --
/// precisely the duplicate this dedup exists to prevent. The first spelling
/// seen is the one kept verbatim.
pub fn skill_focus_facts_from_choices(
    selected_feats: &[String],
    selected_choices: &[SelectedChoice],
) -> Vec<SkillFocusFact> {
    if !feat_identity::holds(selected_feats, SKILL_FOCUS_FEAT_KEY) {
        return Vec::new();
    }
    let mut facts: Vec<SkillFocusFact> = Vec::new();
    for skill_name in selected_choices
        .iter()
        .filter(|choice| choice.choice_set_id == SKILL_FOCUS_TARGET_CHOICE_SET)
        .filter_map(|choice| choice.selection_id.strip_prefix(SKILL_FOCUS_SKILL_SELECTION_PREFIX))
        .filter(|skill_name| !skill_name.is_empty())
    {
        let already_grounded = facts
            .iter()
            .any(|fact| fact.skill_name.to_lowercase() == skill_name.to_lowercase());
        if !already_grounded {
            facts.push(SkillFocusFact {
                skill_name: skill_name.to_owned(),
                bonus: SKILL_FOCUS_BONUS,
            });
        }
    }
    facts
}

/// Improved Initiative's real catalog key (`feat_data/combat.rs`), verified the
/// same way every other key in this module is.
const IMPROVED_INITIATIVE_FEAT_KEY: &str = "Improved Initiative";

/// Improved Initiative's real PF1 Core Rulebook benefit: a flat +4 on initiative
/// checks. Verified against BOTH the catalog record's own effect qualifiers
/// (`["COMBAT", "INITIATIVE", "4"]`) and the raw PCGen corpus's authoritative
/// `BENEFIT:` prose ("You get a +4 bonus on initiative checks") -- the two agree
/// exactly. No level scaling, no chooser, no prerequisite, and no `STACK:YES`/
/// `MULT:YES` (not repeatable), making this the single least-qualified numeric
/// record in the 185-feat CRB catalog.
const IMPROVED_INITIATIVE_BONUS: i16 = 4;

/// Improved Initiative's real, computed initiative bonus for a character's
/// `selected_feats`. Returns `0` (not a fabricated value) when the feat is
/// absent. Grounds as a standalone fact: this codebase computes no integrated
/// initiative total to layer onto, mirroring
/// `inquisitor_cunning_initiative_bonus`'s own already-grounded standalone
/// initiative record.
pub fn initiative_bonus_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, IMPROVED_INITIATIVE_FEAT_KEY) {
        IMPROVED_INITIATIVE_BONUS
    } else {
        0
    }
}

/// Endurance's real catalog key (`feat_data/general.rs`).
const ENDURANCE_FEAT_KEY: &str = "Endurance";

/// Endurance's real PF1 Core Rulebook benefit: a flat +4 on a specific, named
/// set of endurance-related checks and saves. Verified against the catalog
/// record's effect qualifiers (`["VAR", "Feat_Endurance_SaveBonus", "4",
/// "TYPE=Base"]`) and the raw corpus's `BENEFIT:` prose, which enumerates the
/// exact scope: Swim checks to resist nonlethal damage from exhaustion;
/// Constitution checks to continue running, to avoid nonlethal damage from a
/// forced march, to hold your breath, and to avoid nonlethal damage from
/// starvation or thirst; and Fortitude saves to avoid nonlethal damage from hot
/// or cold environments and to resist damage from suffocation.
///
/// Grounded as a standalone fact rather than folded into any computed total,
/// deliberately: the scope is a heterogeneous list of situational checks, none
/// of which this codebase computes, and the bonus explicitly does NOT apply to
/// Fortitude saves generally -- so layering it onto `compute_total_saves`'s
/// Fortitude total the way `save_bonuses_from_feats` layers Great Fortitude's
/// would be a real overstatement, not a simplification. This mirrors the
/// already-grounded Poison Resistance idiom (a verified magnitude against one
/// named hazard category, recorded as its own fact). The feat's separate
/// "sleep in light or medium armor without becoming fatigued" clause carries no
/// numeric token and is not grounded here.
const ENDURANCE_CHECK_BONUS: i16 = 4;

/// Endurance's real, computed bonus for a character's `selected_feats`. Returns
/// `0` (not a fabricated value) when the feat is absent. Not repeatable (no
/// `STACK:YES`/`MULT:YES` in the corpus), so a duplicated string stays +4.
pub fn endurance_check_bonus_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, ENDURANCE_FEAT_KEY) {
        ENDURANCE_CHECK_BONUS
    } else {
        0
    }
}

/// Spell Focus / Greater Spell Focus catalog keys, and the Mechanism-B contract
/// for recording their player-chosen school targets:
/// `choice:spell_focus_target -> school:<name>` and
/// `choice:greater_spell_focus_target -> school:<name>`. The `"school:"` prefix
/// matches the convention already shipped for Wizard specialization
/// (`choice:wizard_school_specialization -> school:evocation`).
const SPELL_FOCUS_FEAT_KEY: &str = "Spell Focus";
const GREATER_SPELL_FOCUS_FEAT_KEY: &str = "Greater Spell Focus";
const SPELL_FOCUS_TARGET_CHOICE_SET: &str = "choice:spell_focus_target";
const GREATER_SPELL_FOCUS_TARGET_CHOICE_SET: &str = "choice:greater_spell_focus_target";
const SPELL_FOCUS_SCHOOL_SELECTION_PREFIX: &str = "school:";

/// Spell Focus's real benefit: `+1` to the saving-throw DC of spells from the
/// chosen school (`BONUS:DC|SCHOOL.%LIST|1|TYPE=SpellFocus`, `BENEFIT:` "Add +1
/// to the Difficulty Class for all saving throws against spells from the school
/// of magic you select"). Token and prose agree exactly.
const SPELL_FOCUS_DC_BONUS: i16 = 1;

/// Greater Spell Focus's real **total** with Spell Focus: `+2`.
///
/// This one needs care, because the corpus token and the printed rule look like
/// they disagree. Greater Spell Focus carries
/// `BONUS:DC|SCHOOL.%LIST|2|TYPE=SpellFocus` -- a bare `2` -- while its
/// `BENEFIT:` prose reads "Add **+1** to the Difficulty Class ... This bonus
/// stacks with the bonus from Spell Focus." Both are correct and describe the
/// same outcome: PCGen takes the *highest* of same-typed bonuses, and both feats
/// emit `TYPE=SpellFocus`, so holding Spell Focus (1) and Greater Spell Focus
/// (2) resolves to `max(1, 2) = 2` -- which is exactly RAW's +1 stacked on +1.
///
/// The token is therefore a **total, not an increment**. Summing the two tokens
/// would assert `+3`, a specific, checkable, wrong DC. This is the same class of
/// trap as Extra Channel's `ABILITYPOOL|1` (deferred for contradicting its own
/// "two additional times per day" prose) and is why every magnitude in this
/// module is checked against `BENEFIT:` prose, not the `BONUS:` token alone.
const GREATER_SPELL_FOCUS_DC_BONUS: i16 = 2;

/// One grounded Spell Focus fact: the real saving-throw DC bonus applied to one
/// specific, player-chosen school of magic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellFocusFact {
    /// The chosen school, verbatim from the `"school:<name>"` selection.
    pub school_name: String,
    /// [`SPELL_FOCUS_DC_BONUS`] (+1), raised to
    /// [`GREATER_SPELL_FOCUS_DC_BONUS`] (+2) when Greater Spell Focus also
    /// names this school.
    pub dc_bonus: i16,
}

/// Every `school:<name>` target named by a given choice set, in input order,
/// skipping malformed and empty selections.
fn chosen_schools<'a>(selected_choices: &'a [SelectedChoice], choice_set_id: &str) -> Vec<&'a str> {
    selected_choices
        .iter()
        .filter(|choice| choice.choice_set_id == choice_set_id)
        .filter_map(|choice| choice.selection_id.strip_prefix(SPELL_FOCUS_SCHOOL_SELECTION_PREFIX))
        .filter(|school| !school.is_empty())
        .collect()
}

/// Grounds Spell Focus's real DC bonus per explicitly chosen school -- but
/// **only** when Spell Focus is actually in `selected_feats` AND a matching
/// `choice:spell_focus_target -> school:<name>` is present. Grounds nothing
/// (never a fabricated canonical school) when either half is missing, exactly
/// the no-silent-seeding contract ratified for Skill Focus: the entire value of
/// a Focus feat IS which target the player chose.
///
/// One fact per distinct school, first-seen order, compared case-insensitively
/// (both feats are `STACK:NO MULT:YES` -- repeatable across different schools,
/// never stacking on the same one -- and the consumer lowercases the school name
/// to build its explanation id, so case variants must not emit two records under
/// one id).
///
/// Greater Spell Focus **raises an existing school's fact to `+2`** rather than
/// adding a second record; see [`GREATER_SPELL_FOCUS_DC_BONUS`] for why that is
/// a total rather than an increment. It deliberately grounds **nothing** for a
/// school the character has no base Spell Focus in: that build cannot legally
/// exist (`PREABILITY:1,CATEGORY=FEAT,Spell Focus`, and
/// `CHOOSE:SCHOOLS|ABILITY=FEAT[Spell Focus]` restricts the target to a school
/// already focused), and its magnitude in isolation is genuinely ambiguous
/// between RAW's `+1` and the token's `2` -- so no checkable number is asserted
/// for an impossible character. This differs from
/// [`difficult_terrain_feet_from_feats`], which does honour Acrobatic Steps
/// without its Nimble Moves prerequisite: there the magnitude is unambiguous
/// either way, so reporting it is safe and prerequisite validation stays
/// `feat_prereqs`' job. The distinction is ambiguity, not prerequisites.
pub fn spell_focus_facts_from_choices(
    selected_feats: &[String],
    selected_choices: &[SelectedChoice],
) -> Vec<SpellFocusFact> {
    if !feat_identity::holds(selected_feats, SPELL_FOCUS_FEAT_KEY) {
        return Vec::new();
    }

    let mut facts: Vec<SpellFocusFact> = Vec::new();
    for school in chosen_schools(selected_choices, SPELL_FOCUS_TARGET_CHOICE_SET) {
        let already_grounded = facts
            .iter()
            .any(|fact| fact.school_name.to_lowercase() == school.to_lowercase());
        if !already_grounded {
            facts.push(SpellFocusFact {
                school_name: school.to_owned(),
                dc_bonus: SPELL_FOCUS_DC_BONUS,
            });
        }
    }

    if feat_identity::holds(selected_feats, GREATER_SPELL_FOCUS_FEAT_KEY) {
        for school in chosen_schools(selected_choices, GREATER_SPELL_FOCUS_TARGET_CHOICE_SET) {
            if let Some(fact) = facts
                .iter_mut()
                .find(|fact| fact.school_name.to_lowercase() == school.to_lowercase())
            {
                fact.dc_bonus = GREATER_SPELL_FOCUS_DC_BONUS;
            }
        }
    }

    facts
}

/// Weapon Focus / Greater Weapon Focus.
///
/// Unlike Skill Focus and Spell Focus, Weapon Focus already existed here before
/// this module: the fixed creation-time loadout records it in `selected_feats`
/// under the engine token `feat:weapon_focus` rather than the catalog key, and
/// expresses its target as a **compound selection** inside the Fighter
/// bonus-feat choice slot (`choice:fighter_bonus_feat ->
/// feat:weapon_focus:weapon:longsword`).
///
/// **Presence needs no special handling for that.** Every producer here matches
/// through [`crate::rules_core::feat_identity`]'s shared fold, which already
/// treats the catalog key, the engine token and the compound form as one feat --
/// this pair used to carry hand-listed `*_SYNTHETIC_FEAT_ID` constants for
/// exactly that job, and they were deleted as redundant once the fold landed.
/// Only the **target** still needs the extra lookup: it lives in the Fighter
/// bonus-feat choice slot rather than this feat's own choice set, so both
/// sources are read and deduplicated by weapon. Reading only the new choice set
/// would leave the real shipped Fighter loadout grounding nothing -- the same
/// "misses exactly the characters it matters most for" failure as Ranger's
/// automatic Endurance.
const WEAPON_FOCUS_FEAT_KEY: &str = "Weapon Focus";
const GREATER_WEAPON_FOCUS_FEAT_KEY: &str = "Greater Weapon Focus";
const WEAPON_FOCUS_TARGET_CHOICE_SET: &str = "choice:weapon_focus_target";
const GREATER_WEAPON_FOCUS_TARGET_CHOICE_SET: &str = "choice:greater_weapon_focus_target";
const FIGHTER_BONUS_FEAT_CHOICE_SET: &str = "choice:fighter_bonus_feat";
const WEAPON_SELECTION_PREFIX: &str = "weapon:";
const LEGACY_WEAPON_FOCUS_COMPOUND_PREFIX: &str = "feat:weapon_focus:weapon:";

/// Weapon Focus's and Greater Weapon Focus's real attack bonuses: `+1` **each**,
/// genuinely cumulative to `+2`.
///
/// Both feats' `BONUS:` tokens name a *variable* rather than a number
/// (`BONUS:WEAPONPROF=%LIST|TOHIT|WeaponFocusToHit` and `...|
/// GreaterWeaponFocusToHit`), so the magnitude had to be resolved rather than
/// read off. Both variables are defined to `0` on the feats themselves and
/// unconditionally raised to `1` by a single global `CATEGORY=Internal|
/// Default.MOD` record carrying `BONUS:VAR|WeaponFocusToHit|1|TYPE=Base` and
/// `BONUS:VAR|GreaterWeaponFocusToHit|1|TYPE=Base`, with no `PRE` gating.
/// Both `BENEFIT:` texts agree ("+1 bonus on ... attack rolls", and Greater's
/// "This bonus stacks with other bonuses on attack rolls, including those from
/// Weapon Focus").
///
/// **This is the opposite encoding from Spell Focus, and the difference is a
/// real trap.** Spell Focus and Greater Spell Focus share one bonus *type*
/// (`TYPE=SpellFocus`), so PCGen takes the highest and Greater's token `2` is a
/// TOTAL. Weapon Focus and Greater Weapon Focus write to two *separate
/// variables*, each worth 1, applied by two separate tokens -- so they genuinely
/// ADD. Reusing the Spell Focus take-highest shape here would yield
/// `max(1, 1) = 1` and understate the real bonus by a full point. Two feat
/// families that look identical on the surface encode stacking oppositely.
const WEAPON_FOCUS_ATTACK_BONUS: i16 = 1;
const GREATER_WEAPON_FOCUS_ATTACK_BONUS: i16 = 1;

/// One grounded Weapon Focus fact: the real attack-roll bonus applied to one
/// specific, player-chosen weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponFocusFact {
    /// The chosen weapon, verbatim from whichever selection named it.
    pub weapon_name: String,
    /// `+1` from either feat alone, `+2` when both name this weapon.
    pub attack_bonus: i16,
}

/// Every `<prefix><name>` target named by a given choice set, in input order,
/// skipping malformed and empty selections.
fn chosen_targets<'a>(
    selected_choices: &'a [SelectedChoice],
    choice_set_id: &str,
    selection_prefix: &str,
) -> Vec<&'a str> {
    selected_choices
        .iter()
        .filter(|choice| choice.choice_set_id == choice_set_id)
        .filter_map(|choice| choice.selection_id.strip_prefix(selection_prefix))
        .filter(|name| !name.is_empty())
        .collect()
}

/// First-seen order, case-insensitive deduplication -- matching the consumer's
/// lowercased explanation id, so case variants never emit two records under one
/// id.
fn dedup_targets<'a>(names: Vec<&'a str>) -> Vec<&'a str> {
    let mut kept: Vec<&'a str> = Vec::new();
    for name in names {
        if !kept.iter().any(|seen| seen.to_lowercase() == name.to_lowercase()) {
            kept.push(name);
        }
    }
    kept
}

/// Grounds Weapon Focus's real attack bonus per explicitly chosen weapon,
/// recognising both the new and pre-existing target representations (see
/// [`WEAPON_FOCUS_FEAT_KEY`]). The feat's own id shapes are absorbed by the
/// shared identity fold, not enumerated here.
///
/// Grounds nothing when the feat is held with no target, or a target is orphaned
/// with no feat -- the same no-silent-seeding contract as Skill Focus and Spell
/// Focus. One fact per distinct weapon, first-seen order, compared
/// case-insensitively; neither feat stacks on a single weapon (Weapon Focus is
/// `MULT:YES` with no `STACK` token, which PCGen treats as no-stack, and Greater
/// Weapon Focus is explicitly `STACK:NO`).
///
/// Greater Weapon Focus **adds** its own `+1` to a weapon already focused, and
/// grounds its own unambiguous `+1` even for a weapon with no base Weapon Focus.
/// That last part deliberately differs from [`spell_focus_facts_from_choices`],
/// which grounds nothing for a Greater-without-base school. The ratified
/// distinguishing axis is **ambiguity, not prerequisites**: Greater Weapon
/// Focus's magnitude is its own separate variable worth exactly 1 under any
/// reading, so reporting it asserts nothing uncertain, whereas Greater Spell
/// Focus's value in isolation was genuinely ambiguous between RAW's `+1` and its
/// token's `2`.
///
/// This function only reads `selected_feats`/`selected_choices`; it does not
/// touch the fixed-loadout posture gate that separately requires the exact
/// `feat:weapon_focus:weapon:longsword` selection.
pub fn weapon_focus_facts_from_choices(
    selected_feats: &[String],
    selected_choices: &[SelectedChoice],
) -> Vec<WeaponFocusFact> {
    let holds = |catalog_key: &str| feat_identity::holds(selected_feats, catalog_key);

    let mut facts: Vec<WeaponFocusFact> = Vec::new();

    if holds(WEAPON_FOCUS_FEAT_KEY) {
        let mut targets =
            chosen_targets(selected_choices, WEAPON_FOCUS_TARGET_CHOICE_SET, WEAPON_SELECTION_PREFIX);
        targets.extend(chosen_targets(
            selected_choices,
            FIGHTER_BONUS_FEAT_CHOICE_SET,
            LEGACY_WEAPON_FOCUS_COMPOUND_PREFIX,
        ));
        for weapon in dedup_targets(targets) {
            facts.push(WeaponFocusFact {
                weapon_name: weapon.to_owned(),
                attack_bonus: WEAPON_FOCUS_ATTACK_BONUS,
            });
        }
    }

    if holds(GREATER_WEAPON_FOCUS_FEAT_KEY) {
        let targets = chosen_targets(
            selected_choices,
            GREATER_WEAPON_FOCUS_TARGET_CHOICE_SET,
            WEAPON_SELECTION_PREFIX,
        );
        for weapon in dedup_targets(targets) {
            match facts
                .iter_mut()
                .find(|fact| fact.weapon_name.to_lowercase() == weapon.to_lowercase())
            {
                Some(fact) => fact.attack_bonus += GREATER_WEAPON_FOCUS_ATTACK_BONUS,
                None => facts.push(WeaponFocusFact {
                    weapon_name: weapon.to_owned(),
                    attack_bonus: GREATER_WEAPON_FOCUS_ATTACK_BONUS,
                }),
            }
        }
    }

    facts
}

/// Weapon Specialization's and Greater Weapon Specialization's catalog keys and
/// their Mechanism-B target contracts, following the same shape as the two
/// Focus feats above.
///
/// Neither carries a legacy compound-id form the way Weapon Focus does: the
/// shipped Fighter fixture is level 1 and cannot hold either feat (see the
/// qualify gates below), so there is no pre-existing loadout to stay compatible
/// with and no reason to invent a second accepted representation.
const WEAPON_SPECIALIZATION_FEAT_KEY: &str = "Weapon Specialization";
const GREATER_WEAPON_SPECIALIZATION_FEAT_KEY: &str = "Greater Weapon Specialization";
const WEAPON_SPECIALIZATION_TARGET_CHOICE_SET: &str = "choice:weapon_specialization_target";
const GREATER_WEAPON_SPECIALIZATION_TARGET_CHOICE_SET: &str =
    "choice:greater_weapon_specialization_target";

/// Both Specialization feats' real bonuses: `+2` **each**, genuinely cumulative
/// to `+4`, and applied to **damage rolls, not attack rolls**.
///
/// This is the single most important fact about this family and the easiest to
/// get wrong. Both tokens are `BONUS:WEAPONPROF=%LIST|DAMAGE|2` -- the payload
/// slot reads `DAMAGE` where the two Focus feats read `TOHIT`. Grouping all
/// four "weapon feats" as attack-roll contributors, which their shared naming
/// invites, would silently inflate every attack total by up to `+4`.
///
/// Unlike Weapon Focus, the magnitude here is a literal `2` in the token rather
/// than a variable needing a trace, and both `BENEFIT:` texts agree ("+2 bonus
/// on all damage rolls"). Greater's benefit text states the stacking outright:
/// "This bonus to damage stacks with other damage roll bonuses, including any
/// you gain from Weapon Specialization." Its `STACK:NO` constrains taking the
/// *same feat* twice for one weapon, not its interaction with the base feat --
/// the same distinction already resolved for Greater Weapon Focus.
const WEAPON_SPECIALIZATION_DAMAGE_BONUS: i16 = 2;
const GREATER_WEAPON_SPECIALIZATION_DAMAGE_BONUS: i16 = 2;

/// One grounded Specialization fact: the real damage-roll bonus applied to one
/// specific, player-chosen weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponSpecializationFact {
    /// The chosen weapon, verbatim from whichever selection named it.
    pub weapon_name: String,
    /// `+2` from either feat alone, `+4` when both name this weapon.
    pub damage_bonus: i16,
}

/// Grounds the Specialization family's real damage bonus per explicitly chosen
/// weapon, mirroring [`weapon_focus_facts_from_choices`] exactly -- including
/// grounding Greater alone, on the same ratified ambiguity-not-prerequisites
/// axis (Greater's `+2` is unambiguous in isolation).
///
/// **Prerequisites are deliberately not enforced here.** Corpus gates these on
/// Fighter levels via internal qualify variables (`PREVARGTEQ:WeapSpecQualify`
/// / `GreatWeapSpecQualify`, set by hidden `FighterWeaponQualify` abilities at
/// Fighter 4 and 12), plus feat chains through Weapon Focus. This producer
/// grounds what is held and explicitly chosen, exactly as the Focus producer
/// does; prerequisite validity is a separate concern from magnitude, and
/// splitting that responsibility differently here would make two sibling
/// producers behave inconsistently for no stated reason.
pub fn weapon_specialization_facts_from_choices(
    selected_feats: &[String],
    selected_choices: &[SelectedChoice],
) -> Vec<WeaponSpecializationFact> {
    let holds = |catalog_key: &str| feat_identity::holds(selected_feats, catalog_key);

    let mut facts: Vec<WeaponSpecializationFact> = Vec::new();

    if holds(WEAPON_SPECIALIZATION_FEAT_KEY) {
        let targets = chosen_targets(
            selected_choices,
            WEAPON_SPECIALIZATION_TARGET_CHOICE_SET,
            WEAPON_SELECTION_PREFIX,
        );
        for weapon in dedup_targets(targets) {
            facts.push(WeaponSpecializationFact {
                weapon_name: weapon.to_owned(),
                damage_bonus: WEAPON_SPECIALIZATION_DAMAGE_BONUS,
            });
        }
    }

    if holds(GREATER_WEAPON_SPECIALIZATION_FEAT_KEY) {
        let targets = chosen_targets(
            selected_choices,
            GREATER_WEAPON_SPECIALIZATION_TARGET_CHOICE_SET,
            WEAPON_SELECTION_PREFIX,
        );
        for weapon in dedup_targets(targets) {
            match facts
                .iter_mut()
                .find(|fact| fact.weapon_name.to_lowercase() == weapon.to_lowercase())
            {
                Some(fact) => fact.damage_bonus += GREATER_WEAPON_SPECIALIZATION_DAMAGE_BONUS,
                None => facts.push(WeaponSpecializationFact {
                    weapon_name: weapon.to_owned(),
                    damage_bonus: GREATER_WEAPON_SPECIALIZATION_DAMAGE_BONUS,
                }),
            }
        }
    }

    facts
}

/// Improved Critical's catalog key and its Mechanism-B target contract.
const IMPROVED_CRITICAL_FEAT_KEY: &str = "Improved Critical";
const IMPROVED_CRITICAL_TARGET_CHOICE_SET: &str = "choice:improved_critical_target";

/// Every weapon explicitly named by an Improved Critical selection, deduplicated
/// case-insensitively, in first-seen order.
///
/// **This returns targets rather than a magnitude, and the shape is the point.**
/// Improved Critical's token is `BONUS:WEAPONPROF=%LIST|CRITRANGEDOUBLE|1|
/// TYPE=NonStackingCrit` -- a *multiplier on the weapon's own threat range*,
/// not a flat bonus. There is no single number to hand back: doubling means
/// something different for every weapon (a Longsword's 19-20 becomes 17-20, a
/// Battleaxe's 20 becomes 19-20), so only a consumer holding the weapon's stat
/// block can apply it. Returning a `+1`-shaped fact here to match its siblings
/// would be inventing a magnitude the corpus does not state.
///
/// `TYPE=NonStackingCrit` means it never compounds with another threat-range
/// effect of the same type, so holding it is boolean per weapon -- hence a plain
/// target list with no count.
pub fn improved_critical_targets_from_choices(
    selected_feats: &[String],
    selected_choices: &[SelectedChoice],
) -> Vec<String> {
    let holds = selected_feats
        .iter()
        .any(|feat| feat_identity::same(feat, IMPROVED_CRITICAL_FEAT_KEY));
    if !holds {
        return Vec::new();
    }

    let targets = chosen_targets(
        selected_choices,
        IMPROVED_CRITICAL_TARGET_CHOICE_SET,
        WEAPON_SELECTION_PREFIX,
    );
    dedup_targets(targets).into_iter().map(str::to_owned).collect()
}

/// The three CRB feats that GRANT weapon proficiency, and the two
/// Mechanism-B target contracts the chooser pair need.
///
/// **These do not carry a `BONUS:` token at all, and that is the point.**
/// A survey that looks for `BONUS:WEAPONPROF` finds the five weapon feats
/// above and concludes the proficiency-granting feats are unimplementable
/// -- but `BONUS:WEAPONPROF=<group>|TOHIT|n` means "apply n to attacks with
/// that proficiency group", it never grants a proficiency. The grant token
/// is `AUTO:WEAPONPROF`, which is why all three of these records read
/// `effect: None` in this crate's own feat catalog and therefore computed
/// nothing whatsoever before this producer. Their magnitude is not a bonus
/// -- it is the REMOVAL of PF1's -4 nonproficiency attack penalty, which
/// this engine already computes
/// (`pilot_compute::WEAPON_NONPROFICIENCY_ATTACK_PENALTY`, read from the
/// game mode's own `WEAPONNONPROFPENALTY:-4`).
const SIMPLE_WEAPON_PROFICIENCY_FEAT_KEY: &str = "Simple Weapon Proficiency";
const MARTIAL_WEAPON_PROFICIENCY_FEAT_KEY: &str = "Martial Weapon Proficiency";
const EXOTIC_WEAPON_PROFICIENCY_FEAT_KEY: &str = "Exotic Weapon Proficiency";
const MARTIAL_WEAPON_PROFICIENCY_TARGET_CHOICE_SET: &str =
    "choice:martial_weapon_proficiency_target";
const EXOTIC_WEAPON_PROFICIENCY_TARGET_CHOICE_SET: &str =
    "choice:exotic_weapon_proficiency_target";

/// Every weapon proficiency a character's feats grant, in the two shapes
/// the corpus actually uses.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WeaponProficiencyGrantsFromFeats {
    /// Simple Weapon Proficiency grants the whole Simple TIER, not one
    /// weapon: its corpus record carries no `CHOOSE:` at all, only
    /// `ABILITY:Internal|AUTOMATIC|Weapon Prof ~ Simple` -- the same
    /// indirection `weapon_tables`' own doc comment documents for class
    /// grants, whose target carries `AUTO:WEAPONPROF|TYPE=Simple`. Modelling
    /// it as a per-weapon choice would understate it by the entire rest of
    /// the Simple list.
    pub grants_simple_tier: bool,
    /// The individually named weapons granted by Martial Weapon Proficiency
    /// and Exotic Weapon Proficiency, verbatim from each `weapon:<name>`
    /// selection, deduplicated case-insensitively in first-seen order.
    ///
    /// Both are `MULT:YES` (repeatable for a different weapon each time)
    /// with no `STACK:` token -- and stacking would be meaningless anyway,
    /// since proficiency is boolean. The two feats' grants are pooled into
    /// one list because `AUTO:WEAPONPROF|%LIST` is byte-identical on both:
    /// which feat granted a proficiency changes nothing about its effect.
    pub named_weapons: Vec<String>,
}

/// Grounds the real proficiency grants a character's feats confer.
///
/// Grants nothing for a chooser feat held with no recorded target -- the
/// same no-silent-seeding contract as every other chooser feat here. That
/// matters more than usual for this pair: seeding a default weapon would
/// silently erase a -4 penalty from a real attack total.
///
/// **Prerequisites are deliberately not enforced**, matching every sibling
/// producer in this module: Exotic Weapon Proficiency's `PRETOTALAB:1` and
/// Martial Weapon Proficiency's `!PREABILITY:...Output` guard are
/// `feat_prereqs`' responsibility.
///
/// **The chosen weapon's own tier is deliberately not checked either, and
/// that is a considered call rather than an omission.** Martial Weapon
/// Proficiency's `CHOOSE:WEAPONPROFICIENCY|!PC[TYPE=Martial]` and Exotic's
/// `ANY[TYPE=Exotic]` restrict what the PICKER offers, and rejecting an
/// off-tier target here would break a legitimate, extremely common PF1
/// build: Exotic Weapon Proficiency (bastard sword) is the canonical way to
/// wield a bastard sword one-handed, but `weapon_tables` resolves the
/// Bastard Sword -- a genuinely dual-tier weapon -- to its Martial tier
/// (see `the_two_dual_tier_weapons_resolve_to_their_martial_tier`). A tier
/// gate would therefore reject a correct choice while the current shape
/// only accepts an incorrect one that the picker never offers in the first
/// place. Over-refusal is the worse failure, and the consumer joins the
/// name to a real weapon and is where any such check would have to live
/// anyway -- this module imports no weapon table, by design.
pub fn weapon_proficiency_grants_from_feats(
    selected_feats: &[String],
    selected_choices: &[SelectedChoice],
) -> WeaponProficiencyGrantsFromFeats {
    let holds = |key: &str| feat_identity::holds(selected_feats, key);

    let mut named: Vec<&str> = Vec::new();
    if holds(MARTIAL_WEAPON_PROFICIENCY_FEAT_KEY) {
        named.extend(chosen_targets(
            selected_choices,
            MARTIAL_WEAPON_PROFICIENCY_TARGET_CHOICE_SET,
            WEAPON_SELECTION_PREFIX,
        ));
    }
    if holds(EXOTIC_WEAPON_PROFICIENCY_FEAT_KEY) {
        named.extend(chosen_targets(
            selected_choices,
            EXOTIC_WEAPON_PROFICIENCY_TARGET_CHOICE_SET,
            WEAPON_SELECTION_PREFIX,
        ));
    }

    WeaponProficiencyGrantsFromFeats {
        grants_simple_tier: holds(SIMPLE_WEAPON_PROFICIENCY_FEAT_KEY),
        named_weapons: dedup_targets(named).into_iter().map(str::to_owned).collect(),
    }
}

/// Weapon Finesse's catalog key. It takes no target: unlike the five
/// `%LIST` weapon feats, its corpus token names the `Finesseable` weapon
/// TYPE directly (`BONUS:COMBAT|TOHIT.Finesseable|
/// ((max(STR,DEX)-STR)+SHIELDACCHECK)|TYPE=NotRanged`), so it applies to
/// every finesseable weapon the character wields at once and has no
/// `CHOOSE:` token and no chooser contract.
const WEAPON_FINESSE_FEAT_KEY: &str = "Weapon Finesse";

/// Whether the character holds Weapon Finesse.
///
/// **Returns a boolean, not a magnitude, and the shape is the point.** The
/// corpus token's payload is `((max(STR,DEX)-STR)+SHIELDACCHECK)` -- an
/// expression over two ability scores and the worn shield's armor check
/// penalty, none of which this dependency-free leaf module knows. There is
/// no single number to hand back, the same way Improved Critical's
/// threat-range doubling has none. The consumer, which holds the ability
/// modifiers and the weapon's own `Finesseable` facet, is the only place
/// that can evaluate it.
///
/// Not repeatable (no `MULT:`/`STACK:` token), so holding it twice is
/// still one effect.
pub fn holds_weapon_finesse(selected_feats: &[String]) -> bool {
    feat_identity::holds(selected_feats, WEAPON_FINESSE_FEAT_KEY)
}

/// Master Craftsman's catalog key and its Mechanism-B target contract:
/// `choice:master_craftsman_target -> skill:<name>`. Fourth use of the chooser
/// pattern proven by Skill Focus, Spell Focus and Weapon Focus.
const MASTER_CRAFTSMAN_FEAT_KEY: &str = "Master Craftsman";
const MASTER_CRAFTSMAN_TARGET_CHOICE_SET: &str = "choice:master_craftsman_target";
const MASTER_CRAFTSMAN_SKILL_SELECTION_PREFIX: &str = "skill:";

/// Master Craftsman's real benefit: a flat `+2` on one chosen Craft or
/// Profession skill. Token (`BONUS:SKILL|LIST|2`) and `BENEFIT:` prose ("You
/// receive a +2 bonus on your chosen Craft or Profession skill") agree exactly
/// -- no level scaling, no conditional tier.
const MASTER_CRAFTSMAN_SKILL_BONUS: i16 = 2;

/// One grounded Master Craftsman fact: the real `+2` on a specific,
/// player-chosen Craft or Profession skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasterCraftsmanFact {
    /// The chosen skill, verbatim from the `"skill:<name>"` selection.
    pub skill_name: String,
    /// Always [`MASTER_CRAFTSMAN_SKILL_BONUS`] (+2).
    pub bonus: i16,
}

/// Grounds Master Craftsman's real `+2` per explicitly chosen Craft/Profession
/// skill -- only when the feat is in `selected_feats` AND a matching
/// `choice:master_craftsman_target -> skill:<name>` is present. Grounds nothing
/// (never a fabricated canonical skill) when either half is missing, the same
/// no-silent-seeding contract the other three chooser feats carry.
///
/// One fact per distinct skill, first-seen order, compared case-insensitively:
/// the feat is `STACK:NO MULT:YES` (repeatable across different Craft/Profession
/// skills, never stacking on one), and the consumer lowercases the skill name to
/// build its explanation id.
///
/// **Two parts of this feat are deliberately not grounded here.**
/// 1. Its second corpus token, `BONUS:VAR|MasterCraftsmanRanks|
///    var("SKILLRANK=%LIST")`, makes ranks in the chosen skill stand in for
///    caster level when crafting magic items. This codebase models no item
///    creation at all (all eight `ItemCreation` feats carry no numeric token)
///    and tracks no Craft/Profession ranks, so there is nothing to substitute
///    into -- naming it rather than inventing a value.
/// 2. Its `PRESKILL:1,TYPE.Craft=5,TYPE.Profession=5` prerequisite (5 ranks in
///    the chosen skill) is not enforced here. Prerequisite validation is
///    `feat_prereqs`' job, the same split already documented for Acrobatic
///    Steps, and the `+2` magnitude is unambiguous regardless -- so reporting it
///    asserts nothing uncertain. Worth knowing that the deterministic skill
///    posture pins ranks at 1 (`pilot_compute::SELECTED_SKILL_RANK`), so no
///    character this engine currently composes could legally hold this feat;
///    that is a gap in what the engine can represent, not a reason to withhold a
///    verified number.
pub fn master_craftsman_facts_from_choices(
    selected_feats: &[String],
    selected_choices: &[SelectedChoice],
) -> Vec<MasterCraftsmanFact> {
    if !feat_identity::holds(selected_feats, MASTER_CRAFTSMAN_FEAT_KEY) {
        return Vec::new();
    }
    let targets = chosen_targets(
        selected_choices,
        MASTER_CRAFTSMAN_TARGET_CHOICE_SET,
        MASTER_CRAFTSMAN_SKILL_SELECTION_PREFIX,
    );
    dedup_targets(targets)
        .into_iter()
        .map(|skill_name| MasterCraftsmanFact {
            skill_name: skill_name.to_owned(),
            bonus: MASTER_CRAFTSMAN_SKILL_BONUS,
        })
        .collect()
}

/// One real, corpus-verified combat-maneuver bonus granted by one feat.
///
/// The PF1 Core Rulebook carries twelve of these in two uniform families:
/// six `Improved <maneuver>` feats (corpus `BONUS:VAR|CMB_X,CMD_X|2` -- a single
/// token granting BOTH an offensive and a defensive +2) and six
/// `Greater <maneuver>` feats (corpus `BONUS:VAR|CMB_X|2` -- offensive only).
/// Every one of the twelve carries the identical `+2` magnitude.
///
/// Grounded as standalone facts: this codebase computes no Combat Maneuver Bonus
/// or Combat Maneuver Defense total to layer onto. The precedent is direct and
/// double -- `MONK_IMPROVED_GRAPPLE_BONUS` already grounds exactly this `+2` from
/// exactly this corpus token (for Improved Grapple, reached through Monk's own
/// bonus-feat choice slot), and `DWARF_STABILITY_CMD_BONUS` already grounds a
/// flat racial CMD-against-a-named-maneuver bonus the same way.
///
/// These pass the opponent-dependency bar in both halves: the `CMB` half is a
/// bonus to the character's *own* maneuver check, conditioned on nothing; the
/// `CMD` half is conditioned on the opponent's *action type* ("whenever an
/// opponent tries to trip you"), which is a static defensive property of the
/// character rather than a fact about a specific opponent the engine would have
/// to evaluate -- exactly what Dwarf Stability already grounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatManeuverFeatBonus {
    /// The real catalog `key` string, matched on exactly.
    pub feat_key: &'static str,
    /// The maneuver the bonus applies to, as it appears in the corpus variable
    /// name (`CMB_BullRush` -> "Bull Rush").
    pub maneuver: &'static str,
    /// Bonus to the character's own combat maneuver check. `+2` for all twelve.
    pub cmb_bonus: i16,
    /// Bonus to Combat Maneuver Defense against that maneuver. `+2` for the six
    /// `Improved` feats, `0` for the six `Greater` feats, whose corpus token
    /// genuinely carries no `CMD_` term.
    pub cmd_bonus: i16,
}

/// The flat magnitude shared by all twelve combat-maneuver feats, verified
/// against each feat's own corpus record and its `BENEFIT:` prose ("+2 bonus on
/// checks made to ... a foe", "+2 bonus to your Combat Maneuver Defense").
const COMBAT_MANEUVER_FEAT_BONUS: i16 = 2;

/// Every combat-maneuver feat this engine grounds, in the stable corpus source
/// order of `feat_data/combat.rs` (auditable against that file top-to-bottom --
/// all six `Greater` records precede all six `Improved` records there).
///
/// Improved Grapple is deliberately included even though Monk's bonus-feat path
/// already grounds it: that path keys on a synthetic `feat:improved_grapple` id
/// in `selected_choices`, whereas this producer keys on the real catalog string
/// in `selected_feats`. A non-Monk who picks Improved Grapple from the real feat
/// catalog gets nothing today -- this closes that gap. The two paths read
/// different fields, so neither shadows nor double-counts the other.
const COMBAT_MANEUVER_FEAT_BONUSES: &[CombatManeuverFeatBonus] = &[
    CombatManeuverFeatBonus { feat_key: "Greater Bull Rush", maneuver: "Bull Rush", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: 0 },
    CombatManeuverFeatBonus { feat_key: "Greater Disarm", maneuver: "Disarm", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: 0 },
    CombatManeuverFeatBonus { feat_key: "Greater Grapple", maneuver: "Grapple", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: 0 },
    CombatManeuverFeatBonus { feat_key: "Greater Overrun", maneuver: "Overrun", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: 0 },
    CombatManeuverFeatBonus { feat_key: "Greater Sunder", maneuver: "Sunder", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: 0 },
    CombatManeuverFeatBonus { feat_key: "Greater Trip", maneuver: "Trip", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: 0 },
    CombatManeuverFeatBonus { feat_key: "Improved Bull Rush", maneuver: "Bull Rush", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: COMBAT_MANEUVER_FEAT_BONUS },
    CombatManeuverFeatBonus { feat_key: "Improved Disarm", maneuver: "Disarm", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: COMBAT_MANEUVER_FEAT_BONUS },
    CombatManeuverFeatBonus { feat_key: "Improved Grapple", maneuver: "Grapple", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: COMBAT_MANEUVER_FEAT_BONUS },
    CombatManeuverFeatBonus { feat_key: "Improved Overrun", maneuver: "Overrun", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: COMBAT_MANEUVER_FEAT_BONUS },
    CombatManeuverFeatBonus { feat_key: "Improved Sunder", maneuver: "Sunder", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: COMBAT_MANEUVER_FEAT_BONUS },
    CombatManeuverFeatBonus { feat_key: "Improved Trip", maneuver: "Trip", cmb_bonus: COMBAT_MANEUVER_FEAT_BONUS, cmd_bonus: COMBAT_MANEUVER_FEAT_BONUS },
];

/// Every grounded combat-maneuver bonus for the feats actually present in
/// `selected_feats`, in the stable corpus order of
/// [`COMBAT_MANEUVER_FEAT_BONUSES`]. Returns an empty vec (not fabricated
/// bonuses) when none is selected. Keyed on the exact catalog `key` string, so a
/// longer feat whose name merely begins with a grounded key never matches.
///
/// An `Improved` and its matching `Greater` feat ground as two separate facts
/// rather than one superseding the other: every Greater feat's `BENEFIT:` prose
/// states explicitly that its bonus "stacks with the bonus granted by
/// Improved <maneuver>". No maneuver feat is repeatable (none carries
/// `STACK:YES`/`MULT:YES`), so a duplicated string still grounds one fact.
pub fn combat_maneuver_bonuses_from_feats(
    selected_feats: &[String],
) -> Vec<CombatManeuverFeatBonus> {
    COMBAT_MANEUVER_FEAT_BONUSES
        .iter()
        .filter(|bonus| feat_identity::holds(selected_feats, bonus.feat_key))
        .copied()
        .collect()
}

/// Stunning Fist's real catalog key (`feat_data/combat.rs`).
const STUNNING_FIST_FEAT_KEY: &str = "Stunning Fist";

/// Stunning Fist's two real, computed magnitudes: the Fortitude save DC a
/// damaged foe must beat, and how many times per day the character may attempt
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StunningFistFacts {
    /// `10 + (total level / 2) + Wisdom modifier`.
    pub save_dc: i16,
    /// `monk level + floor((total level - monk level) / 4)`.
    pub uses_per_day: i16,
}

/// Grounds Stunning Fist's real save DC and uses-per-day when the feat is in
/// `selected_feats`; `None` (never fabricated values) when it is absent.
///
/// Both formulas are transcribed verbatim from the feat's own corpus record:
/// `BONUS:VAR|StunningFistDC|10+(TL/2)+WIS` and
/// `BONUS:VAR|StunningFistAttack|MonkLVL+floor((TL-MonkLVL)/4)`. The
/// uses-per-day shape is the class-aware version of the `BENEFIT:` prose's
/// "once per day for every four levels you have attained (but see Special)" --
/// the Special clause is precisely the monk exception the `MonkLVL` term
/// encodes, so a monk gets one use per monk level while every other level
/// contributes at the quarter rate.
///
/// **Why this grounds despite the absence of a save-resolution engine.** Monk's
/// own remaining-feats slice deferred Stunning Fist citing "no DC/save engine,"
/// but that objection does not survive the corrected standalone-grounding bar:
/// `monk_scorpion_style_dc` already grounds a save DC from the byte-identical
/// formula `10+(TL/2)+WIS` under the same missing engine. A DC is a property of
/// the character, not of any opponent -- what is missing is only the resolution
/// of a save against it, which is exactly the "integration is a bonus, not a
/// requirement" case the corrected bar covers.
///
/// `total_level` and `monk_level` are threaded as plain scalars, and
/// `wisdom_modifier` applied verbatim with no floor at 0 (clamping would
/// fabricate a value the corpus does not specify), keeping this module the same
/// dependency-free leaf the rest of the file is. The level subtraction saturates
/// so an inconsistent caller cannot panic.
pub fn stunning_fist_facts_from_feats(
    selected_feats: &[String],
    total_level: u8,
    monk_level: u8,
    wisdom_modifier: i16,
) -> Option<StunningFistFacts> {
    if !feat_identity::holds(selected_feats, STUNNING_FIST_FEAT_KEY) {
        return None;
    }
    let non_monk_levels = total_level.saturating_sub(monk_level);
    Some(StunningFistFacts {
        save_dc: 10 + i16::from(total_level) / 2 + wisdom_modifier,
        uses_per_day: i16::from(monk_level) + i16::from(non_monk_levels) / 4,
    })
}

/// The real catalog keys for the three movement feats this engine grounds
/// (`feat_data/general.rs`).
const NIMBLE_MOVES_FEAT_KEY: &str = "Nimble Moves";
const ACROBATIC_STEPS_FEAT_KEY: &str = "Acrobatic Steps";
const FLEET_FEAT_KEY: &str = "Fleet";

/// Feet of difficult terrain each feat lets the character cross as if it were
/// normal terrain, each round. Verified against both feats' corpus records
/// (`BONUS:VAR|Feat_NimbleMoves_Squares|5` and `|15` -- note both write the same
/// variable, PCGen's own way of expressing that they add together) and against
/// Acrobatic Steps' `BENEFIT:` prose, which states the combined total outright:
/// "The effects of this feat stack with those provided by Nimble Moves (allowing
/// you to move normally through a total of 20 feet of difficult terrain each
/// round)."
const NIMBLE_MOVES_DIFFICULT_TERRAIN_FEET: i16 = 5;
const ACROBATIC_STEPS_DIFFICULT_TERRAIN_FEET: i16 = 15;

/// Fleet's real base-speed increase: `BONUS:MOVEADD|TYPE.Walk|5`, with
/// `BENEFIT:` prose "While you are wearing light or no armor, your base speed
/// increases by 5 feet. You lose the benefits of this feat if you carry a medium
/// or heavy load." The magnitude is unconditional; the armor/encumbrance
/// qualifier is a posture condition for the consumer to state on the grounded
/// record, not a reason to withhold a verified number.
const FLEET_BASE_SPEED_FEET: i16 = 5;

/// Total feet of difficult terrain a character's feats let them cross as normal
/// terrain each round. Nimble Moves and Acrobatic Steps genuinely stack (see
/// [`ACROBATIC_STEPS_DIFFICULT_TERRAIN_FEET`]); neither is repeatable, so each
/// contributes at most once. Returns `0` when neither is selected.
///
/// Acrobatic Steps' real prerequisite is Nimble Moves, but prerequisite
/// validation is `feat_prereqs`'s responsibility, not this module's -- this
/// producer reports the corpus magnitude of whatever is actually selected rather
/// than silently re-deriving legality.
pub fn difficult_terrain_feet_from_feats(selected_feats: &[String]) -> i16 {
    let has = |key: &str| feat_identity::holds(selected_feats, key);
    let mut feet = 0;
    if has(NIMBLE_MOVES_FEAT_KEY) {
        feet += NIMBLE_MOVES_DIFFICULT_TERRAIN_FEET;
    }
    if has(ACROBATIC_STEPS_FEAT_KEY) {
        feet += ACROBATIC_STEPS_DIFFICULT_TERRAIN_FEET;
    }
    feet
}

/// Feet of base-speed increase from Fleet. Unlike every other feat this module
/// grounds, Fleet carries `STACK:YES MULT:YES` in the corpus -- it is genuinely
/// repeatable, and PF1 characters may take it more than once for a cumulative
/// increase. Occurrences are therefore **counted**, not merely detected: two
/// Fleet picks are a real +10 feet, and collapsing them to +5 would understate a
/// verified magnitude just as surely as fabricating one would overstate it.
/// Returns `0` when Fleet is absent.
pub fn base_speed_bonus_from_feats(selected_feats: &[String]) -> i16 {
    let picks = feat_identity::count(selected_feats, FLEET_FEAT_KEY);
    i16::try_from(picks).unwrap_or(i16::MAX).saturating_mul(FLEET_BASE_SPEED_FEET)
}

/// The real APG/ACG catalog keys for the four passive-bonus feats grounded
/// below, verified against the ingested catalog records
/// (`rules_tables::apg::feat_data::general` / `rules_tables::acg::feat_data::
/// general`) AND against the raw corpus lines they came from
/// (`apg_feats.lst` 21/41/172/187, `acg_feats.lst` 155). Each appears exactly
/// once in its book, with no `.MOD` record and no `#`-disabled duplicate.
const SHARP_SENSES_FEAT_KEY: &str = "Sharp Senses";
const STEEL_SOUL_FEAT_KEY: &str = "Steel Soul";
const DEEPSIGHT_FEAT_KEY: &str = "Deepsight";
const STEADFAST_PERSONALITY_FEAT_KEY: &str = "Steadfast Personality";

/// Sharp Senses' real **resulting** Perception bonus: `+4`, not the `+2` its
/// token names.
///
/// This is the Spell-Focus total-vs-increment trap in its other direction, and
/// reading the token alone gets it wrong. The corpus token is
/// `BONUS:VAR|KeenSensesBonus|2` -- an *increment* to the same variable the
/// racial keen-senses trait already sets to `2` (this engine already recognises
/// that racial token: `BONUS:SKILL|Perception|KeenSensesBonus|TYPE=Racial`,
/// `BONUS:VAR|KeenSensesBonus|2`). Two plus two is four, and the `BENEFIT:`
/// prose states the result outright: "You receive a +4 racial bonus on
/// Perception skill checks. **This replaces the normal bonus from the keen
/// senses racial trait.**"
///
/// So `+4` is the total a holder ends up with, and it does **not** stack on top
/// of the racial `+2` -- it supersedes it. The feat's own
/// `PREABILITY:1,CATEGORY=Special Ability,TYPE.KeenSenses` prerequisite
/// guarantees the racial trait is present, so `+4` is exact for every character
/// who can legally hold this feat, not a best case.
const SHARP_SENSES_PERCEPTION_BONUS: i16 = 4;

/// Sharp Senses' real, computed Perception bonus. Returns `0` (not a fabricated
/// value) when the feat is absent. Not repeatable (no `STACK:YES`/`MULT:YES`),
/// so a duplicated string stays `+4`.
///
/// Grounded under its own record rather than as a
/// [`StandaloneSkillFeatFact`] deliberately: Alertness already grounds a
/// Perception fact, and both would slugify to one explanation id. Keeping this
/// separate also gives the replacement semantics somewhere to be stated, which a
/// bare table row has no room for.
pub fn sharp_senses_perception_bonus_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, SHARP_SENSES_FEAT_KEY) {
        SHARP_SENSES_PERCEPTION_BONUS
    } else {
        0
    }
}

/// Steel Soul's real **resulting** bonus on saves against spells and
/// spell-like abilities: `+4`, not the `+2` its token names -- the identical
/// increment-not-total shape as [`SHARP_SENSES_PERCEPTION_BONUS`], and verified
/// the same way.
///
/// Token: `BONUS:VAR|SaveBonus_vs_Spells|2|TYPE=Racial`, incrementing the very
/// variable the dwarf Hardy racial trait already sets to `2` (again already
/// recognised by this engine). `BENEFIT:` prose: "You receive a +4 racial bonus
/// on saving throws against spells and spell-like abilities. **This replaces the
/// normal bonus from the dwarf's hardy racial trait.**" Its
/// `PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Hardy` and
/// `PRERACE:1,RACESUBTYPE=Dwarf` prerequisites guarantee the racial base, so `+4`
/// is exact rather than a ceiling. Replaces, does not stack.
///
/// **Deliberately NOT layered onto any of `compute_total_saves`' three totals.**
/// The bonus applies only against spells and spell-like abilities; those totals
/// are the general, unconditional saves. Adding it there would overstate every
/// save against a non-magical effect -- the same overstatement
/// [`ENDURANCE_CHECK_BONUS`] refuses for its own named-hazard scope. It grounds
/// standalone instead, which is precisely what this engine already does for the
/// dwarf Hardy racial bonus this feat supersedes: a scope condition on a
/// *category of effect* is a static property of the character, not a fact about
/// a particular opponent, so it clears the standalone-grounding bar.
///
/// The feat's remaining token (`BONUS:VAR|DwarfHardyAspect|1|TYPE=Boolean`,
/// with its paired `DEFINE:DwarfHardyAspect|0`) is a boolean display marker
/// carrying no magnitude, and is not grounded as a number.
const STEEL_SOUL_SAVE_VS_SPELLS_BONUS: i16 = 4;

/// Steel Soul's real, computed bonus on saves against spells and spell-like
/// abilities. Returns `0` (not a fabricated value) when the feat is absent. Not
/// repeatable, so a duplicated string stays `+4`.
pub fn steel_soul_save_vs_spells_bonus_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, STEEL_SOUL_FEAT_KEY) {
        STEEL_SOUL_SAVE_VS_SPELLS_BONUS
    } else {
        0
    }
}

/// Deepsight's real darkvision increase: `+60` feet.
///
/// Token (`BONUS:VISION|Darkvision|60`) and `BENEFIT:` prose ("Your darkvision
/// has a range of 120 feet") agree once the prerequisite is read: the feat
/// requires `PREVISION:1,Darkvision=60`, so the holder's base is exactly 60 and
/// `60 + 60 = 120`. The token's `60` is an increment; the prose's `120` is the
/// resulting total. Both numbers are real, and the value grounded here is the
/// increment the token states, with the resulting 120 left to the consumer's
/// record to explain -- this module never sees the character's race.
///
/// **Grounded standalone because this engine models no vision numerically at
/// all.** Racial darkvision exists here only as prose inside a race trait's
/// `detail` string (`race_tables.rs`: "Darkvision 60 ft (cr_races.lst
/// race:dwarf SENSE:Darkvision (60 ft))"), never as a number, so there is no
/// vision total to layer onto and no way to compute the 120 from data. The
/// magnitude itself is unconditional -- no activation, no per-day budget, no
/// opponent dependency -- so it clears the standalone bar cleanly; what is
/// missing is a vision model, not a fact about this feat.
const DEEPSIGHT_DARKVISION_FEET: i16 = 60;

/// Deepsight's real, computed darkvision increase in feet. Returns `0` (not a
/// fabricated value) when the feat is absent. Not repeatable, so a duplicated
/// string stays `+60`.
pub fn deepsight_darkvision_bonus_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, DEEPSIGHT_FEAT_KEY) {
        DEEPSIGHT_DARKVISION_FEET
    } else {
        0
    }
}

/// Steadfast Personality's real net change to Will saves against mind-affecting
/// effects, for a character with the given Charisma and Wisdom **modifiers**:
/// `CHA - max(WIS, 0)`.
///
/// Both of the feat's corpus tokens are needed to get this right, and each is
/// meaningless alone:
/// - `BONUS:SAVE|Will|CHA-WIS` adds `CHA - WIS`. A Will save already includes
///   `+WIS`, so this nets to "use Charisma in place of Wisdom" -- exactly the
///   `BENEFIT:` prose's "Add your Charisma modifier instead of your Wisdom bonus
///   on Will saves against mind-affecting effects."
/// - `BONUS:SAVE|Will|WIS|PREVARLT:WIS,0` adds `WIS` back, but **only when the
///   Wisdom modifier is negative**. Combined, a negative-Wisdom character gets
///   `CHA - WIS + WIS = CHA`, i.e. keeps the penalty *and* gains Charisma --
///   again exactly the prose: "If you have a Wisdom penalty, you must apply both
///   your Wisdom penalty and your Charisma modifier."
///
/// Collapsing those two cases gives `CHA - max(WIS, 0)`. Reading only the first
/// token would understate a negative-Wisdom character by the full Wisdom
/// penalty; reading only the second would be nonsense. (The `PREVARLT:WIS,0`
/// gate is read as testing the Wisdom *modifier*, not the score: a score below
/// zero is impossible, which would make the token dead code, whereas the prose
/// explicitly describes the penalty case as live.)
///
/// Returns `None` when the feat is absent. Returns `Some(0)` -- a real,
/// meaningful answer, not an absence -- when Charisma and Wisdom happen to be
/// equal: the character genuinely holds the feat and it genuinely nets to zero
/// for them, which is worth reporting rather than silently suppressing.
///
/// **Deliberately NOT layered onto `compute_total_saves`' Will total**, even
/// though this engine *does* integrate the structurally identical Oracle
/// Sidestep Secret (Charisma for Dexterity on Reflex saves). The difference is
/// decisive: Sidestep Secret is unconditional and always on, so it belongs in
/// the total, whereas this feat applies **only against mind-affecting effects**.
/// The Will total here is the general, unconditional Will save, and folding a
/// mind-affecting-only substitution into it would report a specific, checkable,
/// wrong number for every other Will save the character makes. The condition is
/// a scope condition on a category of effect -- a static property of the
/// character rather than a fact about a particular opponent -- so it grounds
/// standalone rather than being deferred.
///
/// Both modifiers are threaded as plain scalars, keeping this module the same
/// dependency-free leaf the rest of the file is, and neither is clamped beyond
/// the `max(WIS, 0)` the corpus itself specifies.
pub fn steadfast_personality_will_bonus_from_feats(
    selected_feats: &[String],
    charisma_modifier: i16,
    wisdom_modifier: i16,
) -> Option<i16> {
    if !feat_identity::holds(selected_feats, STEADFAST_PERSONALITY_FEAT_KEY) {
        return None;
    }
    Some(charisma_modifier - wisdom_modifier.max(0))
}

/// What kind of thing a chooser feat's target names. Decides which picker a
/// caller should offer, and is the only rules knowledge a UI needs in order
/// to prompt correctly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChooserTargetKind {
    /// A specific weapon, e.g. `weapon:Longsword`.
    Weapon,
    /// A specific skill, e.g. `skill:Perception`.
    Skill,
    /// A school of magic, e.g. `school:Evocation`.
    SpellSchool,
}

/// One chooser feat's complete Mechanism-B contract: how to recognise the
/// feat, which choice set records its target, and what that target names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChooserFeatContract {
    /// The catalog display key, e.g. `"Weapon Focus"`.
    ///
    /// One key is enough to recognise the feat in every shape.
    /// [`chooser_contract_for_feat`] matches through
    /// [`crate::rules_core::feat_identity`]'s fold, so the engine-token form
    /// (`"feat:weapon_focus"`) and the compound target-carrying form
    /// (`"feat:weapon_focus:weapon:longsword"`) both resolve here without this
    /// table naming them. It used to carry a per-feat `synthetic_feat_id`
    /// alongside this one; that field was removed rather than kept, because a
    /// contract listing only some feats' token forms is precisely the drift the
    /// shared fold exists to end.
    pub feat_key: &'static str,
    /// The choice set that records this feat's target.
    pub choice_set_id: &'static str,
    /// The prefix every selection in that choice set carries.
    pub selection_prefix: &'static str,
    pub target_kind: ChooserTargetKind,
}

/// Every feat whose target this engine actually consumes.
///
/// **Deliberately not the corpus-wide set of `CHOOSE:` feats.** Many more
/// corpus feats carry a `CHOOSE:` token, but a target recorded for a feat no
/// producer reads would be decorative -- it would render in a UI and change
/// nothing. This table is exactly the feats with a live producer above, so
/// anything a caller records against it reaches real arithmetic.
///
/// Every field references the same private constant its producer uses rather
/// than repeating the literal, so a contract cannot drift from the code that
/// honours it. `chooser_contracts_cover_every_target_choice_set` guards the
/// remaining gap -- adding a tenth chooser feat without extending this table.
pub const CHOOSER_FEAT_CONTRACTS: &[ChooserFeatContract] = &[
    ChooserFeatContract {
        feat_key: SKILL_FOCUS_FEAT_KEY,
        choice_set_id: SKILL_FOCUS_TARGET_CHOICE_SET,
        selection_prefix: SKILL_FOCUS_SKILL_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::Skill,
    },
    ChooserFeatContract {
        feat_key: MASTER_CRAFTSMAN_FEAT_KEY,
        choice_set_id: MASTER_CRAFTSMAN_TARGET_CHOICE_SET,
        selection_prefix: MASTER_CRAFTSMAN_SKILL_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::Skill,
    },
    ChooserFeatContract {
        feat_key: SPELL_FOCUS_FEAT_KEY,
        choice_set_id: SPELL_FOCUS_TARGET_CHOICE_SET,
        selection_prefix: SPELL_FOCUS_SCHOOL_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::SpellSchool,
    },
    ChooserFeatContract {
        feat_key: GREATER_SPELL_FOCUS_FEAT_KEY,
        choice_set_id: GREATER_SPELL_FOCUS_TARGET_CHOICE_SET,
        selection_prefix: SPELL_FOCUS_SCHOOL_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::SpellSchool,
    },
    ChooserFeatContract {
        feat_key: WEAPON_FOCUS_FEAT_KEY,
        choice_set_id: WEAPON_FOCUS_TARGET_CHOICE_SET,
        selection_prefix: WEAPON_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::Weapon,
    },
    ChooserFeatContract {
        feat_key: GREATER_WEAPON_FOCUS_FEAT_KEY,
        choice_set_id: GREATER_WEAPON_FOCUS_TARGET_CHOICE_SET,
        selection_prefix: WEAPON_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::Weapon,
    },
    ChooserFeatContract {
        feat_key: WEAPON_SPECIALIZATION_FEAT_KEY,
        choice_set_id: WEAPON_SPECIALIZATION_TARGET_CHOICE_SET,
        selection_prefix: WEAPON_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::Weapon,
    },
    ChooserFeatContract {
        feat_key: GREATER_WEAPON_SPECIALIZATION_FEAT_KEY,
        choice_set_id: GREATER_WEAPON_SPECIALIZATION_TARGET_CHOICE_SET,
        selection_prefix: WEAPON_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::Weapon,
    },
    ChooserFeatContract {
        feat_key: IMPROVED_CRITICAL_FEAT_KEY,
        choice_set_id: IMPROVED_CRITICAL_TARGET_CHOICE_SET,
        selection_prefix: WEAPON_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::Weapon,
    },
    // The two proficiency-granting chooser feats. Simple Weapon
    // Proficiency is deliberately absent: it grants a whole tier and
    // carries no `CHOOSE:` token, so offering a weapon picker for it would
    // invite a target the producer correctly ignores.
    ChooserFeatContract {
        feat_key: MARTIAL_WEAPON_PROFICIENCY_FEAT_KEY,
        choice_set_id: MARTIAL_WEAPON_PROFICIENCY_TARGET_CHOICE_SET,
        selection_prefix: WEAPON_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::Weapon,
    },
    ChooserFeatContract {
        feat_key: EXOTIC_WEAPON_PROFICIENCY_FEAT_KEY,
        choice_set_id: EXOTIC_WEAPON_PROFICIENCY_TARGET_CHOICE_SET,
        selection_prefix: WEAPON_SELECTION_PREFIX,
        target_kind: ChooserTargetKind::Weapon,
    },
];

/// The chooser contract for a feat named by either its catalog key or its
/// engine-token form, or `None` for a feat that takes no target.
pub fn chooser_contract_for_feat(feat_id: &str) -> Option<&'static ChooserFeatContract> {
    CHOOSER_FEAT_CONTRACTS.iter().find(|contract| {
        feat_identity::same(contract.feat_key, feat_id)
    })
}

/// One feat's recorded targets, resolved against the contracts above.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChosenFeatTargets {
    /// Verbatim as it appeared in `selected_feats`.
    pub feat_id: String,
    pub target_kind: ChooserTargetKind,
    /// The targets recorded for this feat, prefix stripped, deduplicated
    /// case-insensitively in first-seen order. Empty when the feat is held
    /// with no target recorded -- which is a real, reportable state, not an
    /// error, and callers must render it as "no target chosen" rather than
    /// inventing one.
    pub targets: Vec<String>,
}

/// Resolves the recorded targets for every chooser feat the character holds.
///
/// Returns one entry per distinct chooser feat, in the order the feats first
/// appear in `selected_feats`. Feats that take no target are absent entirely;
/// a chooser feat with no recorded target is present with an empty
/// `targets`, because "you have Weapon Focus but never said in what" is
/// exactly the state a sheet needs to show.
///
/// **Duplicate picks are reported per feat, not per pick.** `selected_feats`
/// is an append-only list with no deduplication, so a character can hold
/// Weapon Focus twice; but nothing in the data model links the first pick to
/// the first target. Returning one entry per feat carrying every target it
/// names reports exactly what was recorded. Emitting one entry per pick would
/// require pairing pick N with target N, which the data does not support and
/// which would silently fabricate an association.
pub fn chosen_feat_targets(
    selected_feats: &[String],
    selected_choices: &[SelectedChoice],
) -> Vec<ChosenFeatTargets> {
    let mut resolved: Vec<ChosenFeatTargets> = Vec::new();

    for feat_id in selected_feats {
        let Some(contract) = chooser_contract_for_feat(feat_id) else {
            continue;
        };
        if resolved.iter().any(|entry| entry.feat_id == *feat_id) {
            continue;
        }

        let mut targets =
            chosen_targets(selected_choices, contract.choice_set_id, contract.selection_prefix);
        // Weapon Focus alone also accepts the pre-existing compound form
        // recorded through the Fighter bonus-feat slot; the producer accepts
        // it, so the sheet must show it rather than report "no target" for a
        // character whose target is plainly recorded.
        if contract.feat_key == WEAPON_FOCUS_FEAT_KEY {
            targets.extend(chosen_targets(
                selected_choices,
                FIGHTER_BONUS_FEAT_CHOICE_SET,
                LEGACY_WEAPON_FOCUS_COMPOUND_PREFIX,
            ));
        }

        resolved.push(ChosenFeatTargets {
            feat_id: feat_id.clone(),
            target_kind: contract.target_kind,
            targets: dedup_targets(targets).into_iter().map(str::to_owned).collect(),
        });
    }

    resolved
}

// ---------------------------------------------------------------------------
// SD-27 (`decisions.md` §24/§28, 2026-07-31): the Advanced Race Guide's and
// Pathfinder Unchained's feats that carry a real, unconditional standing
// magnitude.
//
// **The classification that decided this set, derived by command, not by
// taste.** Parsing `arg_feats.lst`'s 187 `CATEGORY:FEAT` records and
// `pu_feats.lst`'s 17 for `BONUS:` tokens and for the inline `PRE*`/`!PRE*`
// qualifiers a PCGen `BONUS:` may carry splits them three ways:
//
// | | ARG | PU |
// |---|---|---|
// | no `BONUS:` token at all -- prose-only, situational or action-granting | 133 | 12 |
// | every `BONUS:` token carries its own inline `PRE*` condition | 5 | 0 |
// | at least one unconditional `BONUS:` token | **49** | **5** |
//
// A record's row-level `PRE*` tokens are deliberately NOT counted as a
// condition on the bonus: those gate *taking* the feat (`feat_prereqs`' job,
// the split this module already documents for Master Craftsman), whereas a
// `PRE*` appended to a `BONUS:` token gates the bonus itself.
//
// **That 49/5 is an upper bound on "should be wired", and the tables below are
// deliberately smaller.** An unconditional `BONUS:` token is not automatically
// a standing number a sheet should show:
//
// * `BONUS:ABILITYPOOL|<pool>|1` (11 ARG feats, 2 PU feats) grants a *further
//   choice*, not a magnitude -- Angelic Flesh's metal, Catfolk Exemplar's
//   aspect, PU's Signature Skill. The number `1` counts choices, not anything
//   on a character sheet.
// * `BONUS:VAR|<internal>|N` (38 ARG tokens) mostly increments a PCGen
//   bookkeeping variable that only means something to another PCGen record --
//   spell-like-ability uses per day (`RacialSLA_*_Times`), racial luck
//   budgets (`Halfling_AdaptableLuck_Times`), fly-maneuverability tiers
//   (`Maneuverability`). This engine models no spell-like abilities, no
//   per-day racial budgets and no maneuverability, so those numbers have
//   nothing to land on and grounding them would state a magnitude out of the
//   only context that gives it meaning.
// * Opponent-dependent bonuses are excluded by this module's own standing bar:
//   Great Hatred's extra +1 applies only against "targets of your hatred
//   racial trait", which is a fact about the creature being attacked, not
//   about the character. See [`STEEL_SOUL_SAVE_VS_SPELLS_BONUS`]'s doc comment
//   for the category-of-effect / particular-opponent line this follows.
//
// **Two of those exclusions were wrong and are withdrawn here (2026-08-01),
// re-derived by command rather than by re-reading the paragraph above.**
//
// * `BONUS:SITUATION|<skill>=<circumstance>|N` was deferred on the grounds that
//   "the corpus itself classifies these as situational". It does -- and this
//   engine **already models that exact classification**: `pilot_compute`'s
//   Dwarf Stonecunning and Dwarf Greed records ground
//   `BONUS:SITUATION|Perception=...|2` and `BONUS:SITUATION|Appraise=...|2` as
//   flat situational-bonus-magnitude records carrying their circumstance in
//   their own text. Being situational was never the bar; having nowhere to land
//   was. These land exactly where the dwarf's do. See
//   [`ARG_SITUATIONAL_SKILL_FACTS`]. The count was also wrong: 3 tokens across
//   **2** feats (Echoes of Stone twice, Carrion Feeder once), not 3 feats.
// * "`BONUS:VAR` increments a variable with a base this engine does not model"
//   is true of the halfling, suli and fetchling budgets -- and false of
//   `DefiantLuckTimes` and `ImprovisationBonus`, whose `DEFINE:` sits on the
//   ARG *feat* record itself, making the total wholly feat-determined with no
//   unmodelled base underneath. Those two are now grounded; see
//   [`DEFIANT_LUCK_FEAT_KEYS`] and [`IMPROVISATION_FEAT_KEYS`]. A third
//   misfile: Stretched Wings was deferred as a manoeuvrability feat, but its
//   `BONUS:VAR|Maneuverability|1` is only half the record -- its other token is
//   `BONUS:MOVEADD|TYPE.Fly|40`, the same movement shape Aquatic Ancestry is
//   already grounded on. See [`stretched_wings_fly_speed_bonus_from_feats`].
//
// What is left -- and what the tables below ground -- is every ARG and PU feat
// whose unconditional token names a dimension this engine either computes a
// total for (Armor Class) or already grounds standalone facts for (skills,
// combat maneuvers, energy resistance, movement, attack rolls, save
// categories, resource pools, per-day pools, vision). Each is hand-modelled per
// §24: no interpreter, one corpus-verified formula per feat, each pinned by its
// own test.
//
// **The ledger, as of 2026-08-01: 24 of ARG's 49 move a number, 25 do not.**
// Derived, not declared: `tests/sd27_arg_and_pu_feat_effects.rs`'s
// `twenty_four_of_args_forty_nine_unconditionally_bonused_feats_now_move_a_number`
// runs the real pipeline once per catalog key and fails if the count drifts in
// either direction. The 25 still deferred, by the engine dimension that does not
// exist for them:
//
// | dimension that does not exist | feats |
// |---|---|
// | a chooser pool, not a magnitude (`BONUS:ABILITYPOOL`) | Elven Spirit, Mother's Gift, Catfolk Exemplar, Blood Drinker, Diverse Palate, Greater Brand, Umbral Scion, Orc Weapon Expertise, Multitalented Mastery, Heavenly Radiance |
// | spell-like abilities and their uses/day (`RacialSLA_*`) | Drow Nobility, Greater Drow Nobility, Improved Drow Nobility, Magical Tail, Shadow Ghost, Heavenly Radiance |
// | a racial per-day budget whose base is the racial trait, not the feat | Adaptive Fortune, Fortunate One, Extra Elemental Assault |
//
// **The per-day-budget row above was wrong about two of its three feats, and
// is corrected here (2026-08-01, operator ruling).** It conflated "this engine
// computes no *total* for uses per day" — still true, and nothing below builds
// one — with "this engine cannot state the number", which is false. The
// operator:
//
// > You do not need a full blown engine for things like uses per day. You just
// > need the ability to calculate the value that is displayed in the
// > description or elsewhere in the UI. […] These are all just display values.
//
// The base is not missing: `Halfling ~ Adaptable Luck` states it on its own row
// as `DEFINE:Halfling_AdaptableLuck_Times|0` plus
// `BONUS:VAR|Halfling_AdaptableLuck_Times|3`, which this engine already reads
// as a same-row constant, and the trait's own `DESC:` substitutes it as `%1`.
// **Adaptive Fortune and Fortunate One therefore now move a rendered number**
// — see [`FeatDisplayValueDeltas`] and
// `tests/sd27_resolved_racial_trait_display_values.rs`, which pins a halfling's
// description reading "Three times per day" with neither feat, "4 times per
// day" with Fortunate One, and "5 times per day … a +4 luck bonus" with both.
// **Extra Elemental Assault stays deferred for a different and real reason**:
// its `Suli_ElementalAssault_Duration` belongs to the Suli, a race outside
// SD-27's 18 in-scope set, so no ingested description exists to put the number
// into. The same correction reaches Great Hatred, which the
// opponent-dependent row below still correctly bars from any attack-roll
// *total* while its stated magnitude now renders in `Gnome ~ Hatred`'s
// sentence.
// | fly manoeuvrability tiers | Angel Wings |
// | animal-companion / special-mount effective levels | Beast Rider |
// | glide ratio while falling (and the corpus disagrees with itself: two unconditional `BONUS:VAR\|GlidingBaseSpeed` tokens, `5` and `25`) | Draconic Glide |
// | weapon-size wielding penalties (`PCSIZE`) and natural-weapon damage dice (`DAMAGESIZE`) | Goblin Gunslinger, Blood Beak |
// | opponent-dependent bonuses, excluded by this module's standing bar | Great Hatred |
//
// **The table above has 25 rows but only 24 distinct feats, and omits a 25th
// (2026-08-01).** Heavenly Radiance is listed twice — under both the chooser
// pool and the spell-like-ability rows, which is true of it and double-counts
// it here — while **Bestow Luck** appears in no row at all. The headline "25 do
// not" is therefore right only because the duplicate offsets the omission.
// Bestow Luck's absence from the moving set is already explained and pinned by
// `tests/sd27_arg_and_pu_feat_effects.rs`: alone it is not a legal character
// and has no Defiant Luck ability for its extra use to attach to, so it moves a
// number only in the company its own prerequisite requires. It belongs in this
// table under a reason of its own, not in the silence between rows.
//
// This is the identical mentions-vs-distinct defect `decisions.md §27.1`
// recorded for the replace-flag counts, in a table written after that
// correction was published. Verified by set difference over
// `rules_tables::advanced_race_guide::feats` rather than by re-reading the
// table: 49 unconditionally bonused, 24 moving, 24 distinct deferred, and
// `Bestow Luck` in neither set.
// ---------------------------------------------------------------------------

/// One ARG feat's real, corpus-verified flat bonus to a named skill.
///
/// Kept as its own record type rather than reusing [`StandaloneSkillFeatFact`]
/// for one decisive reason: that type's consumer derives its explanation id
/// from the *skill name alone*, which is why its own doc comment requires that
/// no two facts across its tables name the same skill. ARG breaks that
/// property immediately and unavoidably -- Seen and Unseen grants `+2` Stealth
/// while Angelic Flesh imposes `-2` Stealth, and CRB's Stealthy already owns
/// `feat.standalone_skill_bonus.stealth`. Three different feats, one skill.
/// This record therefore carries the feat key into the id, so any number of
/// feats may touch one skill without colliding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgSkillFeatFact {
    /// The real ARG catalog `key` string
    /// (`rules_tables::advanced_race_guide::feats`), matched through
    /// [`crate::rules_core::feat_identity`]'s fold.
    pub feat_key: &'static str,
    /// The skill named by the corpus `BONUS:SKILL|<skill>` token, verbatim.
    pub skill_name: &'static str,
    /// The flat magnitude. Signed: Angelic Flesh's is a real `-2` penalty, and
    /// dropping it because it is unwelcome would misreport the character just
    /// as surely as inventing a bonus would.
    pub bonus: i16,
}

/// Every ARG feat whose corpus `BONUS:SKILL` token carries a plain integer and
/// no inline `PRE*`, in `arg_feats.lst` source order.
///
/// Per-feat corpus evidence, each verified against both the `BONUS:` token and
/// the record's own `BENEFIT:` prose:
///
/// * **Brewmaster** -- `BONUS:SKILL|Craft (Alchemy),Profession (Brewer)|1`.
///   **The token and the prose disagree, and the prose is followed here.** The
///   record's own `BENEFIT:` reads "You gain a +2 bonus on Craft (alchemy) and
///   Profession (brewer) checks", which is also the published ARG text. Unlike
///   Sharp Senses, no racial base makes up the difference: the feat's only
///   prerequisites are `PREFACT:1,TEMPLATES,IsDwarf=true` and
///   `PRESKILL:2,Craft (Alchemy)=1,Profession (Brewer)=1`, neither of which
///   grants a skill bonus, so `1` is a PCGen transcription defect rather than
///   an increment. Recorded as `2` with the disagreement stated rather than
///   silently taking either number.
/// * **Sure and Fleet** -- `BONUS:SKILL|Acrobatics,Climb|2|TYPE=Racial`;
///   `BENEFIT:` "You gain a +2 racial bonus on Acrobatics and Climb checks."
///   Token and prose agree exactly. **It does not stack with the halfling
///   Sure-Footed racial trait, and cannot collide with it**: the feat requires
///   `PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Fleet Of Foot`, and
///   Fleet of Foot is precisely the alternate racial trait that *replaces*
///   Sure-Footed. The feat gives back what the alternate trait took away, so a
///   legal holder never has both.
/// * **Seen and Unseen** -- `BONUS:SKILL|Stealth|2`; prose agrees. Its other
///   two clauses (+2 on saves against scrying/divination, and a -4 penalty on
///   others' Survival checks to track the holder) carry no `BONUS:` token at
///   all and are not grounded as numbers here.
/// * **Angelic Flesh** -- `BONUS:SKILL|Disguise,Stealth|-2`; prose "You take a
///   -2 penalty on Disguise and Stealth checks". Its companion
///   `BONUS:ABILITYPOOL|Angelic Flesh Option|1` grants the metal sub-choice and
///   is a chooser, not a magnitude.
/// * **Scavenger's Eye** -- `BONUS:SKILL|Appraise|2`; prose agrees. Its further
///   "+2 on the Appraise check to find the most valuable item in a hoard" is a
///   situational second bonus with no token of its own and is not grounded.
///
/// None of the five carries `MULT:YES`/`STACK:YES` (checked against the corpus
/// records, not assumed), so each grounds once however many times it appears.
const ARG_SKILL_FEAT_FACTS: &[ArgSkillFeatFact] = &[
    ArgSkillFeatFact { feat_key: "Brewmaster", skill_name: "Craft (Alchemy)", bonus: 2 },
    ArgSkillFeatFact { feat_key: "Brewmaster", skill_name: "Profession (Brewer)", bonus: 2 },
    ArgSkillFeatFact { feat_key: "Sure and Fleet", skill_name: "Acrobatics", bonus: 2 },
    ArgSkillFeatFact { feat_key: "Sure and Fleet", skill_name: "Climb", bonus: 2 },
    ArgSkillFeatFact { feat_key: "Seen and Unseen", skill_name: "Stealth", bonus: 2 },
    ArgSkillFeatFact { feat_key: "Angelic Flesh", skill_name: "Disguise", bonus: -2 },
    ArgSkillFeatFact { feat_key: "Angelic Flesh", skill_name: "Stealth", bonus: -2 },
    ArgSkillFeatFact { feat_key: "Scavenger's Eye", skill_name: "Appraise", bonus: 2 },
];

/// Every grounded ARG skill fact for the feats actually present, in the stable
/// corpus order of [`ARG_SKILL_FEAT_FACTS`]. Empty (never fabricated facts)
/// when none is held.
pub fn arg_skill_facts_from_feats(selected_feats: &[String]) -> Vec<ArgSkillFeatFact> {
    ARG_SKILL_FEAT_FACTS
        .iter()
        .filter(|fact| feat_identity::holds(selected_feats, fact.feat_key))
        .copied()
        .collect()
}

/// Sure and Fleet's real `+2` racial Climb bonus, for the one computed skill
/// total this engine keeps that any ARG feat touches.
///
/// Returned separately from [`arg_skill_facts_from_feats`] because Climb is
/// genuinely different from the other seven facts: `compute_selected_skill_modifiers`
/// computes a Climb total, so this value is *added to a number a player reads*
/// rather than grounded as a standalone remark. The fact table still lists
/// Climb so the feat's full corpus content stays auditable in one place; the
/// consumer labels that record as integrated rather than standalone.
pub fn arg_computed_climb_bonus_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, "Sure and Fleet") {
        2
    } else {
        0
    }
}

/// What a character's feats add to the named PCGen variables that appear as
/// **numbers inside racial-trait descriptions**.
///
/// # Why this exists, and why it is not a resource subsystem
///
/// Three ARG feats were deferred as needing "a racial per-day budget whose base
/// is the racial trait, not the feat" or as "opponent-dependent". The operator
/// overruled the first reason on 2026-08-01:
///
/// > You do not need a full blown engine for things like uses per day. You just
/// > need the ability to calculate the value that is displayed in the
/// > description or elsewhere in the UI. […] These are all just display values.
///
/// The base *is* stated by the racial trait row, as a same-row
/// `DEFINE:` + literal `BONUS:VAR:` pair this engine already reads — Adaptable
/// Luck's `DEFINE:Halfling_AdaptableLuck_Times|0` plus
/// `BONUS:VAR|Halfling_AdaptableLuck_Times|3` is the number three, written
/// across two tokens. Adding a feat's `+1` to it is arithmetic, and the result
/// is a word in a sentence. Nothing here tracks expenditure, holds a pool, or
/// knows what "using" an ability means.
///
/// # Membership, and why it is only three feats
///
/// Every ARG feat carrying an unconditional `BONUS:VAR` was re-read for this.
/// A feat earns a field here only when its variable is actually *shown to a
/// player* — that is, when some record whose description this engine renders
/// references it through a `%N`. Measured over the whole shipped corpus rather
/// than assumed, only two rendered records carry such a variable
/// (`Gnome ~ Hatred` and `Halfling ~ Adaptable Luck`), and exactly three feats
/// feed them. The others are named in this module's deferral ledger with the
/// reason each is still short.
///
/// Deliberately **not** included, and why, per record:
///
/// * **Draconic Glide** (`GlidingBaseSpeed`) and **Shadow Ghost**
///   (`ShadowWalkTimes`) resolve completely — both are a same-row `DEFINE:0`
///   plus literal `BONUS:VAR` tokens — but their `%1` sits in a `BENEFIT:`
///   token, and no served table in this repo carries `BENEFIT:`. Grounding
///   them would move no number on any screen, so they are reported rather than
///   written as code nothing reaches.
/// * **Extra Elemental Assault** (`Suli_ElementalAssault_Duration`),
///   **Magical Tail** (`KitsuneTails`) and **Improved Drow Nobility** /
///   **Heavenly Radiance** (`RacialSLA_*_Times`) resolve arithmetically too,
///   but every record whose description would show them belongs to a race
///   outside SD-27's 18 in-scope set (Suli, Kitsune) or to a spell-like-ability
///   record kind this book has not ingested. There is no description to put the
///   number into yet.
/// * **Beast Rider** (`SpecialMountLVL`) genuinely cannot be resolved:
///   `min(TL,max(PaladinLVL+2,AnimalCompanionLVL+2,CavalierMountLVL+2))` needs
///   an animal-companion / special-mount effective level, and this engine
///   computes none of the three.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FeatDisplayValueDeltas {
    /// Added to `Gnome_Hatred_AttackBonus`, the variable `Gnome ~ Hatred`'s
    /// own description substitutes into *"Gnomes receive a +%1 bonus on attack
    /// rolls…"*.
    ///
    /// **Great Hatred**, `arg_feats.lst:42` —
    /// `BONUS:VAR|Gnome_Hatred_AttackBonus|1`, `BENEFIT:` "You gain an
    /// additional +1 bonus on melee and thrown weapon attacks against targets
    /// of your hatred racial trait." Token and prose agree.
    ///
    /// This module's standing bar on opponent-dependent bonuses still holds and
    /// is not being relaxed: Great Hatred is **not** added to any attack-roll
    /// total, because whether it applies depends on the creature being
    /// attacked. What changes is the *sentence that states the magnitude*,
    /// which is a fact about the character and always true of them.
    pub gnome_hatred_attack_bonus: i16,
    /// Added to `Halfling_AdaptableLuck_Times` — the uses-per-day count
    /// `Halfling ~ Adaptable Luck`'s description renders, and the variable its
    /// own `ASPECT:CheckType|Uses per Day` labels.
    ///
    /// * **Fortunate One**, `arg_feats.lst:89` —
    ///   `BONUS:VAR|Halfling_AdaptableLuck_Times|1`.
    /// * **Adaptive Fortune**, `arg_feats.lst:84` —
    ///   `BONUS:VAR|Halfling_AdaptableLuck_Times|1`.
    ///
    /// Both stack in PCGen and in the published text ("Increase the number of
    /// times per day … by 1"), and Adaptive Fortune's own
    /// `PREABILITY:1,CATEGORY=FEAT,Fortunate One` means a legal holder of the
    /// second always holds the first — so `+2` is the only combined value a
    /// legal character reaches, never `+1` from Adaptive Fortune alone.
    pub halfling_adaptable_luck_times: i16,
    /// Added to `Halfling_AdaptableLuck_Bonus`, the luck bonus the same
    /// description renders twice (`+%1` full, `+%2` after the roll, where the
    /// second argument is the corpus's `Halfling_AdaptableLuck_Bonus-1`).
    ///
    /// **Adaptive Fortune**, `arg_feats.lst:84` —
    /// `BONUS:VAR|Halfling_AdaptableLuck_Bonus|2`; `BENEFIT:` "…increase the
    /// luck bonus for each type of use by 2." Fortunate One does **not** touch
    /// this variable, which is the whole difference between the two feats.
    pub halfling_adaptable_luck_bonus: i16,
}

impl FeatDisplayValueDeltas {
    /// True when no held feat moves any display variable — the ordinary case,
    /// and the one where a description must render exactly its racial base.
    pub fn is_zero(self) -> bool {
        self == Self::default()
    }
}

/// Resolves [`FeatDisplayValueDeltas`] for one character's held feats.
///
/// Every magnitude is the corpus `BONUS:VAR` token's own integer, cited on the
/// field it lands in. Nothing is summed across a variable this engine does not
/// render, and nothing is invented for a feat whose variable has no description
/// to appear in.
pub fn display_value_deltas_from_feats(selected_feats: &[String]) -> FeatDisplayValueDeltas {
    let mut deltas = FeatDisplayValueDeltas::default();

    if feat_identity::holds(selected_feats, "Great Hatred") {
        deltas.gnome_hatred_attack_bonus += 1;
    }
    if feat_identity::holds(selected_feats, "Fortunate One") {
        deltas.halfling_adaptable_luck_times += 1;
    }
    if feat_identity::holds(selected_feats, "Adaptive Fortune") {
        deltas.halfling_adaptable_luck_times += 1;
        deltas.halfling_adaptable_luck_bonus += 2;
    }

    deltas
}

/// One ARG feat's real bonus to Combat Maneuver Defense against a named
/// maneuver.
///
/// Separate from [`CombatManeuverFeatBonus`] rather than folded into it: that
/// type pairs a CMB half with a CMD half and its consumer keys ids on the
/// maneuver name alone, so Feline Grace's CMD-vs-trip bonus would collide with
/// Improved Trip's on any character holding both. These two ARG feats grant a
/// CMD bonus and no CMB bonus at all, so there is no CMB half to carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgManeuverDefenseFact {
    pub feat_key: &'static str,
    /// The maneuver as it appears in the corpus variable name (`CMD_BullRush`
    /// -> "Bull Rush").
    pub maneuver: &'static str,
    pub cmd_bonus: i16,
}

/// The flat CMD magnitude both ARG maneuver-defense feats share, verified
/// against each record's `BONUS:VAR|CMD_*|2` token and its `BENEFIT:` prose.
const ARG_MANEUVER_DEFENSE_BONUS: i16 = 2;

/// ARG's two Combat-Maneuver-Defense feats, in `arg_feats.lst` source order.
///
/// * **Feline Grace** -- `BONUS:VAR|CMD_BullRush,CMD_Grapple,CMD_Overrun,CMD_Reposition,CMD_Trip|2`;
///   `BENEFIT:` "You gain a +2 bonus to your CMD against bull rush, grapple,
///   overrun, repositioning, and trip combat maneuvers." Five maneuvers, one
///   token, prose and token in exact agreement.
/// * **Tree Hanger** -- `BONUS:VAR|CMD_Trip|2`; `BENEFIT:` "You gain a +2 bonus
///   to your CMD against all trip attacks."
///
/// **These pass the opponent-dependency bar**, for exactly the reason
/// [`COMBAT_MANEUVER_FEAT_BONUSES`] already records for the CRB `Improved`
/// feats: the condition is the opponent's *action type* ("against trip
/// attacks"), a static defensive property of the character, not a fact about a
/// particular opponent.
///
/// **Deliberately NOT added to the `defense.combat_maneuver_defense` total.**
/// That total is the general CMD, applying to every maneuver; folding in a
/// bonus that only applies to five of them would report a specific, checkable,
/// wrong number against a disarm or a sunder. Same reasoning
/// [`STEEL_SOUL_SAVE_VS_SPELLS_BONUS`] applies to the save totals.
const ARG_MANEUVER_DEFENSE_FACTS: &[ArgManeuverDefenseFact] = &[
    ArgManeuverDefenseFact { feat_key: "Feline Grace", maneuver: "Bull Rush", cmd_bonus: ARG_MANEUVER_DEFENSE_BONUS },
    ArgManeuverDefenseFact { feat_key: "Feline Grace", maneuver: "Grapple", cmd_bonus: ARG_MANEUVER_DEFENSE_BONUS },
    ArgManeuverDefenseFact { feat_key: "Feline Grace", maneuver: "Overrun", cmd_bonus: ARG_MANEUVER_DEFENSE_BONUS },
    ArgManeuverDefenseFact { feat_key: "Feline Grace", maneuver: "Reposition", cmd_bonus: ARG_MANEUVER_DEFENSE_BONUS },
    ArgManeuverDefenseFact { feat_key: "Feline Grace", maneuver: "Trip", cmd_bonus: ARG_MANEUVER_DEFENSE_BONUS },
    ArgManeuverDefenseFact { feat_key: "Tree Hanger", maneuver: "Trip", cmd_bonus: ARG_MANEUVER_DEFENSE_BONUS },
];

/// Every grounded ARG maneuver-defense fact for the feats actually present, in
/// the stable corpus order of [`ARG_MANEUVER_DEFENSE_FACTS`].
pub fn arg_maneuver_defense_facts_from_feats(
    selected_feats: &[String],
) -> Vec<ArgManeuverDefenseFact> {
    ARG_MANEUVER_DEFENSE_FACTS
        .iter()
        .filter(|fact| feat_identity::holds(selected_feats, fact.feat_key))
        .copied()
        .collect()
}

/// One ARG feat's real energy resistance grant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgEnergyResistanceFact {
    pub feat_key: &'static str,
    /// The energy type named by the corpus `BONUS:VAR|<Energy>ResistanceBonus`
    /// variable, spelled as the `BENEFIT:` prose spells it.
    pub energy_type: &'static str,
    pub amount: i16,
}

/// The flat resistance magnitude every ARG resistance feat shares.
const ARG_ENERGY_RESISTANCE_AMOUNT: i16 = 5;

/// ARG's five unconditional energy-resistance grants, in `arg_feats.lst`
/// source order.
///
/// The four `Expanded Fiendish Resistance (<Energy>)` records are the cleanest
/// cases in the whole book: one token each
/// (`BONUS:VAR|<Energy>ResistanceBonus|5|TYPE=Resistance`), one prose sentence
/// each ("You gain resistance 5 to <Energy>."), token and prose identical, no
/// prerequisite touching the magnitude, no `MULT:YES`.
///
/// **Flame Heart's `5` is not a `10`.** Its record carries the fire-resistance
/// token *twice* -- `BONUS:VAR|FireResistanceBonus|5|TYPE=Resistance` and a
/// bare `BONUS:VAR|FireResistanceBonus|5` -- which naively summed would read as
/// resistance 10. Its `BENEFIT:` prose says "You gain fire resistance 5", so
/// the duplicate is PCGen expressing one typed and one untyped copy of the same
/// grant, not two grants. The prose value is grounded.
///
/// **Armor of the Pit is deliberately absent from this table** even though it
/// carries `BONUS:VAR|ColdResistanceBonus,ElectricityResistanceBonus,FireResistanceBonus|5|TYPE=Resistance`
/// with no inline `PRE*`. Its prose reads "You gain a +2 natural armor bonus.
/// If you have the scaled skin racial trait, you instead gain resistance 5 to
/// **two of** the following energy types **that you don't have resistance to
/// already**: cold, electricity, and fire." The token names all three
/// unconditionally, the rule grants two of three chosen by the player, and
/// which two is a choice this engine records nowhere. Grounding three would
/// overstate it, grounding a guessed two would fabricate the choice. Its
/// natural-armor half -- the branch that applies to the character who has *not*
/// taken Scaled Skin, which is what its token's own
/// `!PREABILITY:...Scaled Skin...` says -- is real, unambiguous, and is wired
/// into Armor Class instead; see
/// [`armor_of_the_pit_natural_armor_bonus_from_feats`].
const ARG_ENERGY_RESISTANCE_FACTS: &[ArgEnergyResistanceFact] = &[
    ArgEnergyResistanceFact { feat_key: "Flame Heart", energy_type: "fire", amount: ARG_ENERGY_RESISTANCE_AMOUNT },
    ArgEnergyResistanceFact { feat_key: "Expanded Fiendish Resistance (Acid)", energy_type: "acid", amount: ARG_ENERGY_RESISTANCE_AMOUNT },
    ArgEnergyResistanceFact { feat_key: "Expanded Fiendish Resistance (Cold)", energy_type: "cold", amount: ARG_ENERGY_RESISTANCE_AMOUNT },
    ArgEnergyResistanceFact { feat_key: "Expanded Fiendish Resistance (Electricity)", energy_type: "electricity", amount: ARG_ENERGY_RESISTANCE_AMOUNT },
    ArgEnergyResistanceFact { feat_key: "Expanded Fiendish Resistance (Fire)", energy_type: "fire", amount: ARG_ENERGY_RESISTANCE_AMOUNT },
];

/// Every grounded ARG energy-resistance fact for the feats actually present, in
/// the stable corpus order of [`ARG_ENERGY_RESISTANCE_FACTS`].
///
/// Two feats naming the same energy type (Flame Heart and Expanded Fiendish
/// Resistance (Fire)) return two facts, not a sum: PF1 energy resistance from
/// two sources does not add, the larger applies, and both are `5`. The consumer
/// keys its ids on the feat as well as the energy type so both remain visible
/// and neither is silently dropped.
pub fn arg_energy_resistance_facts_from_feats(
    selected_feats: &[String],
) -> Vec<ArgEnergyResistanceFact> {
    ARG_ENERGY_RESISTANCE_FACTS
        .iter()
        .filter(|fact| feat_identity::holds(selected_feats, fact.feat_key))
        .copied()
        .collect()
}

/// Armor of the Pit's real `+2` natural armor bonus to Armor Class.
///
/// Token: `BONUS:VAR|AC_Natural_Armor|2|TYPE=Base|!PREABILITY:1,CATEGORY=Special Ability,Scaled Skin C ~ Tiefling,Scaled Skin E ~ Tiefling,Scaled Skin F ~ Tiefling`.
/// `BENEFIT:` "You gain a +2 natural armor bonus. If you have the scaled skin
/// racial trait, you instead gain resistance 5 to two of the following energy
/// types..."
///
/// **The inline `!PRE*` here is a static character property, not a situation.**
/// This is the one place the "an inline `PRE*` means situational" heuristic
/// gets the wrong answer: the negated prerequisite asks whether this character
/// took the Scaled Skin alternate racial trait -- a persisted creation-time
/// decision this engine already reads -- so the branch is fully decidable, and
/// the `+2` is unconditional for every character on the other side of it.
/// `has_scaled_skin` is threaded in as a plain bool so this module stays the
/// dependency-free leaf the rest of the file is; the caller owns the
/// alternate-racial-trait lookup.
const ARMOR_OF_THE_PIT_NATURAL_ARMOR_BONUS: i16 = 2;

/// Armor of the Pit's real, computed natural armor bonus. `0` (not a
/// fabricated value) when the feat is absent, and `0` when the character took
/// Scaled Skin -- for whom the corpus token's own negated prerequisite, and the
/// prose's "you *instead* gain", both withhold it.
///
/// Not repeatable (no `MULT:YES`/`STACK:YES` in the record), so a duplicated
/// string still yields `+2`.
pub fn armor_of_the_pit_natural_armor_bonus_from_feats(
    selected_feats: &[String],
    has_scaled_skin: bool,
) -> i16 {
    if has_scaled_skin || !feat_identity::holds(selected_feats, "Armor of the Pit") {
        return 0;
    }
    ARMOR_OF_THE_PIT_NATURAL_ARMOR_BONUS
}

/// Flame Heart's real `+1` caster level for fire-descriptor spells.
///
/// Token: `BONUS:CASTERLEVEL|DESCRIPTOR.Fire|1`. `BENEFIT:` "When casting
/// spells with the fire descriptor or throwing alchemist bombs that deal fire
/// damage, treat your caster level or alchemist level as if you were 1 level
/// higher." Token and prose agree.
///
/// **Deliberately NOT added to any `*.effective_caster_level` record.** Those
/// are the character's general caster level, applying to every spell; this
/// applies only to spells carrying the fire descriptor, and this engine's spell
/// records carry no descriptor. Grounding it standalone states the real bonus
/// without overstating any computed caster level -- the same scope-condition
/// treatment [`STEEL_SOUL_SAVE_VS_SPELLS_BONUS`] receives.
pub fn flame_heart_fire_caster_level_bonus_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, "Flame Heart") {
        1
    } else {
        0
    }
}

/// The two ARG human feats that raise saves against effects with the *emotion*
/// descriptor, and the `+1` each contributes.
///
/// Both write the same corpus variable -- Fearless Curiosity's
/// `BONUS:VAR|FearlessCuriosityBonus|1` and Intimidating Confidence's identical
/// token -- which is PCGen's own way of saying they add together, exactly the
/// shape [`NIMBLE_MOVES_DIFFICULT_TERRAIN_FEET`] already documents for Nimble
/// Moves and Acrobatic Steps. Each record's `BENEFIT:` prose prints the running
/// total through a `%1` substitution token bound to that variable ("You gain a +%1
/// bonus on saving throws against effects with the emotion descriptor" /
/// "Your bonus ... increases to +%1"), so the corpus itself never states a
/// literal: `+1` alone, `+2` together. Intimidating Confidence's own
/// `PREABILITY:1,CATEGORY=FEAT,Fearless Curiosity` prerequisite means the `+2`
/// case is the only way the second feat can appear.
const EMOTION_SAVE_FEAT_KEYS: &[&str] = &["Fearless Curiosity", "Intimidating Confidence"];

/// One feat's contribution to the emotion-descriptor save bonus.
const EMOTION_SAVE_BONUS_PER_FEAT: i16 = 1;

/// The character's real, computed bonus on saving throws against effects with
/// the emotion descriptor. `0` when neither feat is held.
///
/// **Deliberately NOT added to `defense.total_save.will` or any other save
/// total**: it applies only against emotion-descriptor effects, while those
/// totals are the general saves. Same line [`steel_soul_save_vs_spells_bonus_from_feats`]
/// draws, and drawn for the same reason.
///
/// Neither feat is repeatable, so a duplicated string does not inflate the sum.
pub fn emotion_save_bonus_from_feats(selected_feats: &[String]) -> i16 {
    EMOTION_SAVE_FEAT_KEYS
        .iter()
        .filter(|key| feat_identity::holds(selected_feats, key))
        .count() as i16
        * EMOTION_SAVE_BONUS_PER_FEAT
}

/// Aquatic Ancestry's real `+10`-foot swim speed increase.
///
/// Token: `BONUS:MOVEADD|TYPE.Swim|10`. `BENEFIT:` "You gain the amphibious
/// special quality. Your swim speed increases by +10 feet." Token and prose
/// agree exactly.
///
/// Grounds standalone for the same reason [`FLEET_BASE_SPEED_FEET`] does: this
/// engine computes no movement total of any kind, so there is nothing to layer
/// onto. The amphibious quality is a capability with no magnitude and is not
/// grounded as a number.
pub fn aquatic_ancestry_swim_speed_bonus_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, "Aquatic Ancestry") {
        10
    } else {
        0
    }
}

/// Gnome Weapon Focus's real `+1` attack bonus with gnome weapons.
///
/// Token: `BONUS:WEAPONPROF=TYPE.Gnome|TOHIT|1`. `BENEFIT:` "You gain a +1
/// bonus on attack rolls with gnome weapons (weapons with "gnome" in the
/// title)." Token and prose agree.
///
/// **Deliberately NOT added to any attack total.** The bonus is scoped to a
/// weapon *type*, and this engine's per-weapon attack totals carry no gnome
/// facet to test -- unlike Weapon Focus, whose target is a specific weapon the
/// player records through `choice:weapon_focus_target`. Adding it to the
/// baseline melee attack bonus would overstate every swing with a longsword.
/// Grounds standalone with its scope stated.
pub fn gnome_weapon_focus_attack_bonus_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, "Gnome Weapon Focus") {
        1
    } else {
        0
    }
}

/// One ARG feat's real bonus on a named skill *in a named circumstance*.
///
/// `BONUS:SITUATION|<skill>=<circumstance>|N` is PCGen's own token for a bonus
/// that applies only in a stated circumstance, and this engine **already models
/// exactly that shape**: `pilot_compute`'s
/// `DWARF_STONECUNNING_PERCEPTION_SITUATIONAL_BONUS` and
/// `DWARF_GREED_APPRAISE_SITUATIONAL_BONUS` ground the Core Rulebook dwarf's
/// `BONUS:SITUATION|Perception=to notice unusual stonework|2|TYPE=Racial` and
/// `BONUS:SITUATION|Appraise=to assess nonmagical metals or gemstones|2|TYPE=Racial`
/// as flat situational-bonus-magnitude records. The circumstance is carried in
/// the record's own text rather than being folded into a total, so the number a
/// player reads is never overstated for the ordinary check.
///
/// **This corrects an earlier classification.** This module's SD-27 header
/// deferred `BONUS:SITUATION` on the grounds that "the corpus itself classifies
/// these as situational" -- true, and not a reason to withhold the magnitude,
/// because a situational bonus is precisely what the dwarf records already
/// state. Re-derived by command, ARG carries **3 such tokens across 2 feats**
/// (not 3 feats): Echoes of Stone has two, Carrion Feeder one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgSituationalSkillFact {
    pub feat_key: &'static str,
    /// The skill named by the corpus token's `<skill>=` half, verbatim.
    pub skill_name: &'static str,
    /// The circumstance named by the token's `=<circumstance>` half, restated
    /// in the record's own `BENEFIT:` wording so the sheet reads as English
    /// rather than as a PCGen enum.
    pub circumstance: &'static str,
    pub bonus: i16,
}

/// ARG's three `BONUS:SITUATION` tokens, in `arg_feats.lst` source order.
///
/// * **Echoes of Stone** (`arg_feats.lst:380`, oread) --
///   `BONUS:SITUATION|Perception=Underground|4` and
///   `BONUS:SITUATION|Survival=Avoid Becoming Lost in Caverns and Rocky Areas|4`.
///   `BENEFIT:` "You gain a +4 racial bonus on Perception checks underground,
///   and on Survival checks to avoid becoming lost in caverns and rocky areas."
///   Two tokens, two prose clauses, magnitudes identical.
/// * **Carrion Feeder** (`arg_feats.lst:405`, tengu) --
///   `BONUS:SITUATION|Survival=Find Food|2`; `BENEFIT:` "... You receive a +2
///   bonus on Survival skill checks to find food for yourself (and only
///   yourself)." The record's *other* prose clause (a +2 racial bonus on saves
///   against disease and ingested poison) carries **no `BONUS:` token at all**
///   and is deliberately not grounded as a number here -- the same line
///   [`ARG_SKILL_FEAT_FACTS`] already draws for Seen and Unseen's untokened
///   scrying-save and tracking clauses.
///
/// Neither record carries `MULT:YES`/`STACK:YES` (checked, not assumed), so
/// each grounds once however many times its string appears. Neither skill is
/// among the three `compute_selected_skill_modifiers` computes
/// (Climb/Intimidate/Swim), so there is no total either could collide with.
const ARG_SITUATIONAL_SKILL_FACTS: &[ArgSituationalSkillFact] = &[
    ArgSituationalSkillFact {
        feat_key: "Echoes of Stone",
        skill_name: "Perception",
        circumstance: "underground",
        bonus: 4,
    },
    ArgSituationalSkillFact {
        feat_key: "Echoes of Stone",
        skill_name: "Survival",
        circumstance: "to avoid becoming lost in caverns and rocky areas",
        bonus: 4,
    },
    ArgSituationalSkillFact {
        feat_key: "Carrion Feeder",
        skill_name: "Survival",
        circumstance: "to find food for yourself (and only yourself)",
        bonus: 2,
    },
];

/// Every grounded ARG situational-skill fact for the feats actually present, in
/// the stable corpus order of [`ARG_SITUATIONAL_SKILL_FACTS`].
pub fn arg_situational_skill_facts_from_feats(
    selected_feats: &[String],
) -> Vec<ArgSituationalSkillFact> {
    ARG_SITUATIONAL_SKILL_FACTS
        .iter()
        .filter(|fact| feat_identity::holds(selected_feats, fact.feat_key))
        .copied()
        .collect()
}

/// The two ARG human feats that raise skill checks for skills the character has
/// **no ranks in**, and the `+2` each contributes.
///
/// Both write one corpus variable, `ImprovisationBonus`, whose `DEFINE:` sits
/// on the Improvisation record itself (`DEFINE:ImprovisationBonus|0`), so the
/// running total is fully determined by which of the two feats is held -- no
/// racial or class base is being incremented out of view. Verified by command
/// that these are the *only* two writers of `ImprovisationBonus` anywhere in
/// the PCGen Pathfinder data set (`arg_abilities_class.lst`'s Feral Gnasher
/// writes the differently-named `FeralGnasherWickedImprovisationBonus`).
///
/// * **Improvisation** (`arg_feats.lst:111`) -- `BONUS:VAR|ImprovisationBonus|2`;
///   `BENEFIT:` "You gain a +%1 bonus on all skill checks for skills you have
///   no ranks in.", the `%1` bound to that same variable, so the corpus states
///   no literal and the magnitude is read off the token.
/// * **Improved Improvisation** (`arg_feats.lst:110`) -- an identical
///   `BONUS:VAR|ImprovisationBonus|2`; `BENEFIT:` "... The bonus on all skill
///   checks for skills you have no ranks in increases." Its own
///   `PREABILITY:2,CATEGORY=FEAT,Fast Learner,Improvisation` requires both
///   named feats, so Improvisation is always held alongside it and `+4` is the
///   only way the second feat can appear.
///
/// **The scope is proved by the corpus, not inferred from the prose.**
/// Improvisation carries 26 companion tokens of the form
/// `BONUS:SKILL|<skill>|ImprovisationBonus|!PRESKILL:1,<skill>=1` -- one per
/// skill, each gated on *not* having a rank in that skill. That is PCGen
/// spelling out "applies to a skill exactly when the character is untrained in
/// it", which is why those 26 are conditioned tokens and this variable is not.
///
/// Two further tokens on these records are deliberately **not** magnitudes:
/// Improvisation's `BONUS:VAR|UseUntrainedSkills|1` is a capability (use
/// trained-only skills untrained) with no number, and Improved Improvisation's
/// `BONUS:VAR|ACCHECK|ArmorCheckPenalty/2` halves a *nonproficiency* penalty --
/// this engine models no armor or weapon proficiency state to be non-proficient
/// against, so there is nothing for it to halve.
const IMPROVISATION_FEAT_KEYS: &[&str] = &["Improvisation", "Improved Improvisation"];

/// One Improvisation feat's contribution (`BONUS:VAR|ImprovisationBonus|2`).
const IMPROVISATION_BONUS_PER_FEAT: i16 = 2;

/// The character's real, computed bonus on skill checks for skills they have no
/// ranks in: `+2` with Improvisation, `+4` with Improved Improvisation on top.
/// `0` when neither is held.
///
/// **Deliberately NOT added to `skill.selected_modifier.{climb,intimidate,swim}`.**
/// Not as a scope judgement but because the rule excludes them: this engine's
/// deterministic selected-skill posture pins all three at rank 1, and this bonus
/// applies only to skills with *no* ranks. Adding it there would contradict the
/// feat's own text.
///
/// Neither feat is repeatable (no `MULT:YES`/`STACK:YES` on either record), so a
/// duplicated string does not inflate the sum.
pub fn improvisation_untrained_skill_bonus_from_feats(selected_feats: &[String]) -> i16 {
    IMPROVISATION_FEAT_KEYS
        .iter()
        .filter(|key| feat_identity::holds(selected_feats, key))
        .count() as i16
        * IMPROVISATION_BONUS_PER_FEAT
}

/// Stretched Wings' real `+40`-foot fly speed increase.
///
/// Token: `BONUS:MOVEADD|TYPE.Fly|40` (`arg_feats.lst:173`). `BENEFIT:` "Your
/// strix racial fly speed increases to 60 feet (average)."
///
/// **Token and prose agree exactly, and the base that reconciles them is fixed
/// by the feat's own prerequisite.** Stretched Wings requires
/// `PREABILITY:1,CATEGORY=Special Ability,Strix ~ Wing-Clipped`, and the
/// wing-clipped strix's fly speed is stated in the corpus as a literal:
/// `arg_abilities_race.lst:1157` `KEY:Wing-Clipped ~ Strix ~ Flight` carries
/// `MOVE:Fly,20` and `DESC:Wing-clipped strix have a fly speed of 20 feet
/// (poor).` So `20 + 40 = 60`, exactly what the prose says. This is the same
/// shape [`DEEPSIGHT_DARKVISION_FEET`] relies on: a prerequisite that pins the
/// base makes the increment's resulting total exact rather than assumed.
///
/// Grounds standalone for the same reason [`aquatic_ancestry_swim_speed_bonus_from_feats`]
/// does: this engine computes no movement total of any kind.
///
/// **Its companion `BONUS:VAR|Maneuverability|1` is deliberately not grounded.**
/// That token moves a manoeuvrability *tier* on PCGen's own integer scale --
/// the wing-clipped flight record sets `BONUS:VAR|Maneuverability|2|TYPE=Base`
/// for "poor" and this feat's `+1` is what makes the prose's "(average)" -- and
/// this engine has no manoeuvrability dimension for a tier index to mean
/// anything in. The feat's second clause ("You ignore the wing-clipped trait's
/// Fly check requirement to fly upward") is a capability with no magnitude.
pub fn stretched_wings_fly_speed_bonus_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, "Stretched Wings") {
        40
    } else {
        0
    }
}

/// The two ARG human feats that make up the Defiant Luck daily budget, and the
/// one use each contributes.
///
/// Unlike ARG's other per-day variables -- halfling `Halfling_AdaptableLuck_Times`,
/// suli `Suli_ElementalAssault_Duration`, fetchling `ShadowWalkTimes` -- this
/// one has **no base outside the feats**: `DEFINE:DefiantLuckTimes|0` sits on
/// the Defiant Luck record itself (`arg_feats.lst:104`), so the total is
/// entirely determined by which of these two feats is held. That is what makes
/// it groundable while the increment-only ones are not.
///
/// * **Defiant Luck** -- `DEFINE:DefiantLuckTimes|0` plus
///   `BONUS:VAR|DefiantLuckTimes|1`; `BENEFIT:` "%1/day, after you roll a
///   natural 1 on a saving throw or a critical hit is confirmed against you,
///   you can either reroll that saving throw, or force the creature ... to
///   reroll the critical confirmation roll.", the `%1` bound to that variable.
///   The corpus states no literal; `1` is read off the token.
/// * **Bestow Luck** -- `BONUS:VAR|DefiantLuckTimes|1`; `BENEFIT:` "You gain an
///   extra use per day of your Defiant Luck ability." Its own
///   `PREABILITY:2,CATEGORY=FEAT,Defiant Luck,Inexplicable Luck` requires *both*
///   named feats, so Defiant Luck is always held alongside it.
///
/// **Scope note, recorded rather than glossed:** one uningested book
/// (`player_companion/blood_of_the_beast/botb_abilities.lst`) also writes
/// `BONUS:VAR|DefiantLuckTimes|2`. Nothing from that book exists in this
/// engine's catalogs, so the sum here is complete for everything a character can
/// currently hold; when that book lands, its writer joins this function or the
/// number becomes wrong.
const DEFIANT_LUCK_FEAT_KEYS: &[&str] = &["Defiant Luck", "Bestow Luck"];

/// The character's real Defiant Luck uses per day, or `None` (never a
/// fabricated zero) when Defiant Luck itself is absent -- without it there is no
/// such ability at all, exactly as [`stamina_pool_facts_from_feats`] returns
/// `None` without Combat Stamina.
///
/// Neither feat is repeatable, so a duplicated string does not inflate the
/// budget.
pub fn defiant_luck_uses_per_day_from_feats(selected_feats: &[String]) -> Option<i16> {
    if !feat_identity::holds(selected_feats, "Defiant Luck") {
        return None;
    }
    Some(
        DEFIANT_LUCK_FEAT_KEYS
            .iter()
            .filter(|key| feat_identity::holds(selected_feats, key))
            .count() as i16,
    )
}

/// Fiend Sight's real darkvision range, in feet.
///
/// The record (`arg_feats.lst:419`) states this **absolutely, twice**:
/// `VISION:Darkvision (120')` and `BENEFIT:` "You gain low-light vision and your
/// darkvision improves to 120 ft." Its `PREVISION:1,Darkvision=60` prerequisite
/// fixes the holder's base at 60 feet, so the `120` is a doubling of a known
/// base rather than a free-floating claim -- the identical reconciliation
/// [`DEEPSIGHT_DARKVISION_FEET`] performs for APG's Deepsight, which grounds the
/// same 60-to-120 step as a `BONUS:VISION|Darkvision|60` increment.
///
/// **Reported as the resulting range, not as an increment**, because that is
/// what this record's own tokens say; Deepsight's are an increment and are
/// reported as one. The two records therefore carry different ids and different
/// semantics, each matching its own corpus token.
///
/// **The feat's own `BONUS:VAR|FiendSightTier|1` is not the magnitude.** It is a
/// pick counter (`DEFINE:FiendSightTier|0`, capped at two by
/// `PREVARLT:FiendSightTier,2`) whose only consumer is the record's own
/// `ABILITY:Special Ability|AUTOMATIC|See in Darkness|PREVAREQ:FiendSightTier,2`
/// -- a second pick grants the *see in darkness* universal monster ability, a
/// capability with no number, and does not extend the range any further (the
/// `VISION:` token stays `120'`). So the range is 120 at one pick and at two.
const FIEND_SIGHT_DARKVISION_FEET: i16 = 120;

/// The character's real darkvision range in feet once Fiend Sight is held; `0`
/// when it is not.
pub fn fiend_sight_darkvision_feet_from_feats(selected_feats: &[String]) -> i16 {
    if feat_identity::holds(selected_feats, "Fiend Sight") {
        FIEND_SIGHT_DARKVISION_FEET
    } else {
        0
    }
}

/// Pathfinder Unchained's stamina pools, the book's one genuinely numeric feat
/// mechanic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaminaPoolFacts {
    /// `base attack bonus + Constitution modifier`, plus `3` per Extra Stamina
    /// pick.
    pub primary: i16,
    /// The Constitution modifier, when Push the Limits is held; `None` when it
    /// is not.
    pub secondary: Option<i16>,
    /// How many times Extra Stamina was taken, after the corpus's own cap.
    pub extra_stamina_picks: u8,
}

/// Extra Stamina's per-pick magnitude (`BONUS:VAR|StaminaPool|3`).
const EXTRA_STAMINA_POINTS_PER_PICK: i16 = 3;

/// The corpus's own cap on Extra Stamina, transcribed from the record's
/// `!PREABILITY:3,CATEGORY=FEAT,Extra Stamina` token -- PCGen's way of writing
/// "you may not already hold three of these" -- and confirmed by its
/// `BENEFIT:` "Special: You can select this feat up to three times."
///
/// The cap is applied rather than trusted: a saved character carrying four
/// copies (a legal-looking string list this engine does not validate) would
/// otherwise be handed a pool three points too large.
const EXTRA_STAMINA_MAX_PICKS: usize = 3;

/// The character's real, computed stamina pools. `None` (never a fabricated
/// zero-pool) when Combat Stamina is absent -- without it there is no stamina
/// pool at all, and both other feats require it
/// (`PREABILITY:1,CATEGORY=FEAT,Combat Stamina`).
///
/// Formulas, transcribed verbatim from the three corpus records:
/// * **Combat Stamina** -- `BONUS:VAR|StaminaPool|BAB+CON`; `BENEFIT:` "You
///   gain a stamina pool." The published PU rule and the token agree that the
///   pool is base attack bonus plus Constitution modifier.
/// * **Extra Stamina** -- `BONUS:VAR|StaminaPool|3`, `STACK:YES MULT:YES`;
///   "Your stamina pool increases by 3 points ... up to three times." Counted
///   by occurrence through [`crate::rules_core::feat_identity::count`], the
///   same way [`base_speed_bonus_from_feats`] counts Fleet, and capped at
///   [`EXTRA_STAMINA_MAX_PICKS`].
/// * **Push the Limits** -- `BONUS:VAR|SecondaryStaminaPool|CON`; "You gain a
///   secondary stamina pool with a number of stamina points equal to your
///   Constitution modifier." A separate pool, spendable only under conditions
///   this engine does not model, so it is reported separately rather than added
///   into the primary total.
///
/// `base_attack_bonus` and `constitution_modifier` are threaded as plain
/// scalars, keeping this module the dependency-free leaf it is, and neither is
/// clamped: a negative Constitution modifier genuinely shrinks the pool, and
/// the corpus specifies no floor.
pub fn stamina_pool_facts_from_feats(
    selected_feats: &[String],
    base_attack_bonus: i16,
    constitution_modifier: i16,
) -> Option<StaminaPoolFacts> {
    if !feat_identity::holds(selected_feats, "Combat Stamina") {
        return None;
    }
    let extra_picks =
        feat_identity::count(selected_feats, "Extra Stamina").min(EXTRA_STAMINA_MAX_PICKS);
    Some(StaminaPoolFacts {
        primary: base_attack_bonus
            + constitution_modifier
            + extra_picks as i16 * EXTRA_STAMINA_POINTS_PER_PICK,
        secondary: feat_identity::holds(selected_feats, "Push the Limits")
            .then_some(constitution_modifier),
        extra_stamina_picks: extra_picks as u8,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grants_no_bonus_when_toughness_is_not_selected() {
        let selected_feats = vec!["feat:power_attack".to_owned(), "Cleave".to_owned()];
        assert_eq!(hp_bonus_from_feats(&selected_feats), 0);
    }

    #[test]
    fn grants_no_bonus_for_an_empty_feat_list() {
        assert_eq!(hp_bonus_from_feats(&[]), 0);
    }

    #[test]
    fn grants_the_real_flat_plus_three_when_toughness_is_selected() {
        let selected_feats = vec!["feat:power_attack".to_owned(), "Toughness".to_owned()];
        assert_eq!(hp_bonus_from_feats(&selected_feats), 3);
    }

    /// **Deliberate behaviour change**, replacing a test that asserted
    /// `hp_bonus_from_feats(["feat:toughness"]) == 0`.
    ///
    /// That test's stated premise -- "a real Toughness pick never produces
    /// that shape" -- was simply untrue. `feat:<snake_case>` is not a bespoke
    /// id space confined to the fixed loadout: `compose_character_input` seeds
    /// it, and `add_feat_selection` is called with it directly by this repo's
    /// own adapter tests. Treating the two shapes as different feats is what
    /// left a player who picked Dodge from the catalog with no armor class at
    /// all. Both shapes name Toughness, so both ground its real +3.
    #[test]
    fn grounds_toughness_from_the_engine_token_shape_too() {
        assert_eq!(hp_bonus_from_feats(&["feat:toughness".to_owned()]), 3);
    }

    /// The fold must not turn a *different* feat into Toughness.
    #[test]
    fn grounds_nothing_for_a_feat_that_merely_resembles_toughness() {
        assert_eq!(hp_bonus_from_feats(&["feat:toughness_of_stone".to_owned()]), 0);
        assert_eq!(hp_bonus_from_feats(&["Improved Toughness".to_owned()]), 0);
    }

    #[test]
    fn is_not_double_counted_if_the_same_feat_string_somehow_appears_twice() {
        // Not a realistic selected_feats shape (nothing in this crate
        // pushes a duplicate), but the flat scan-and-match shape naturally
        // stays a single +3 rather than +3-per-occurrence, worth pinning.
        let selected_feats = vec!["Toughness".to_owned(), "Toughness".to_owned()];
        assert_eq!(hp_bonus_from_feats(&selected_feats), 3);
    }
}

#[cfg(test)]
mod save_bonuses_from_feats_tests {
    use super::*;

    #[test]
    fn grants_no_bonus_when_no_save_feat_is_selected() {
        let selected_feats = vec!["Toughness".to_owned(), "Cleave".to_owned()];
        assert_eq!(save_bonuses_from_feats(&selected_feats), SaveBonusesFromFeats::default());
    }

    #[test]
    fn grants_no_bonus_for_an_empty_feat_list() {
        assert_eq!(save_bonuses_from_feats(&[]), SaveBonusesFromFeats::default());
    }

    #[test]
    fn great_fortitude_grants_the_real_flat_plus_two_to_fortitude_only() {
        let selected_feats = vec!["Great Fortitude".to_owned()];
        assert_eq!(
            save_bonuses_from_feats(&selected_feats),
            SaveBonusesFromFeats { fortitude: 2, reflex: 0, will: 0 }
        );
    }

    #[test]
    fn lightning_reflexes_grants_the_real_flat_plus_two_to_reflex_only() {
        let selected_feats = vec!["Lightning Reflexes".to_owned()];
        assert_eq!(
            save_bonuses_from_feats(&selected_feats),
            SaveBonusesFromFeats { fortitude: 0, reflex: 2, will: 0 }
        );
    }

    #[test]
    fn iron_will_grants_the_real_flat_plus_two_to_will_only() {
        let selected_feats = vec!["Iron Will".to_owned()];
        assert_eq!(
            save_bonuses_from_feats(&selected_feats),
            SaveBonusesFromFeats { fortitude: 0, reflex: 0, will: 2 }
        );
    }

    #[test]
    fn all_three_stack_independently_when_all_selected() {
        let selected_feats =
            vec!["Great Fortitude".to_owned(), "Lightning Reflexes".to_owned(), "Iron Will".to_owned()];
        assert_eq!(
            save_bonuses_from_feats(&selected_feats),
            SaveBonusesFromFeats { fortitude: 2, reflex: 2, will: 2 }
        );
    }

    #[test]
    fn does_not_match_the_improved_variant_feats() {
        // "Improved Great Fortitude" etc. are distinct, real CRB feats with
        // no `effect` at all in the catalog (see feat_data/general.rs) --
        // confirms this engine matches the exact base-feat string, not a
        // substring/prefix.
        let selected_feats = vec![
            "Improved Great Fortitude".to_owned(),
            "Improved Iron Will".to_owned(),
            "Improved Lightning Reflexes".to_owned(),
        ];
        assert_eq!(save_bonuses_from_feats(&selected_feats), SaveBonusesFromFeats::default());
    }
}

#[cfg(test)]
mod skill_bonuses_from_feats_tests {
    use super::*;

    // A realistic positive Strength modifier for the Intimidating Prowess
    // cases (its Str 13 prerequisite guarantees a non-negative modifier for
    // any character that legally holds the feat). Only Intimidating Prowess
    // reads this argument; the other two feats ignore it entirely.
    const STR_MOD: i16 = 3;

    #[test]
    fn grants_no_bonus_when_no_skill_feat_is_selected() {
        let selected_feats = vec!["Toughness".to_owned(), "Great Fortitude".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats::default()
        );
    }

    #[test]
    fn grants_no_bonus_for_an_empty_feat_list() {
        assert_eq!(skill_bonuses_from_feats(&[], STR_MOD), SkillBonusesFromFeats::default());
    }

    #[test]
    fn athletic_grants_the_real_flat_plus_two_to_climb_and_swim_only() {
        let selected_feats = vec!["Athletic".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats { climb: 2, intimidate: 0, swim: 2 }
        );
    }

    #[test]
    fn persuasive_grants_the_real_flat_plus_two_to_intimidate_only() {
        // Persuasive's real CRB effect also grants +2 Diplomacy, but Diplomacy
        // is not one of the three skills the engine computes today, so this
        // engine grounds only the Intimidate half -- the half that has a live
        // consumer. The Diplomacy half is deferred, not fabricated.
        let selected_feats = vec!["Persuasive".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats { climb: 0, intimidate: 2, swim: 0 }
        );
    }

    #[test]
    fn intimidating_prowess_adds_the_strength_modifier_to_intimidate_only() {
        // Real CRB benefit: "Add your Strength modifier to Intimidate skill
        // checks in addition to your Charisma modifier." Corpus token
        // `BONUS:SKILL|Intimidate|STR` -- the raw Strength modifier, not a
        // flat constant, which is why this function takes a strength_modifier
        // argument at all.
        let selected_feats = vec!["Intimidating Prowess".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats { climb: 0, intimidate: STR_MOD, swim: 0 }
        );
    }

    #[test]
    fn persuasive_and_intimidating_prowess_stack_on_intimidate() {
        // Different real sources (Persuasive's flat +2 vs. Intimidating
        // Prowess's Strength-modifier add), so PF1 stacks them on Intimidate.
        let selected_feats = vec!["Persuasive".to_owned(), "Intimidating Prowess".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats { climb: 0, intimidate: 2 + STR_MOD, swim: 0 }
        );
    }

    #[test]
    fn all_three_stack_independently_when_all_selected() {
        let selected_feats = vec![
            "Athletic".to_owned(),
            "Persuasive".to_owned(),
            "Intimidating Prowess".to_owned(),
        ];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats { climb: 2, intimidate: 2 + STR_MOD, swim: 2 }
        );
    }

    #[test]
    fn intimidating_prowess_faithfully_adds_a_negative_strength_modifier() {
        // The corpus token `BONUS:SKILL|Intimidate|STR` is the raw modifier
        // with no floor, so a negative Strength modifier reduces Intimidate.
        // Not a realistic posture for a feat gated on Str 13, but pinning the
        // verbatim-corpus behavior rather than silently clamping to 0 (which
        // would fabricate a value the corpus does not specify).
        let selected_feats = vec!["Intimidating Prowess".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, -1),
            SkillBonusesFromFeats { climb: 0, intimidate: -1, swim: 0 }
        );
    }

    #[test]
    fn does_not_match_a_substring_or_prefix_of_a_feat_key() {
        // Confirms the exact-string match, not a prefix/substring one: a
        // synthetic "Athletic Steps" (which begins with the grounded "Athletic"
        // key) must not trigger Athletic's bonus, and "Acrobatic" (a real,
        // distinct CRB feat this engine does not ground) must not either.
        let selected_feats = vec!["Athletic Steps".to_owned(), "Acrobatic".to_owned()];
        assert_eq!(
            skill_bonuses_from_feats(&selected_feats, STR_MOD),
            SkillBonusesFromFeats::default()
        );
    }
}

#[cfg(test)]
mod standalone_skill_facts_from_feats_tests {
    use super::*;

    fn fact(feat_key: &'static str, skill_name: &'static str) -> StandaloneSkillFeatFact {
        StandaloneSkillFeatFact { feat_key, skill_name, bonus: 2 }
    }

    #[test]
    fn grounds_nothing_for_an_empty_feat_list() {
        assert!(standalone_skill_facts_from_feats(&[]).is_empty());
    }

    #[test]
    fn grounds_nothing_when_no_grounded_feat_is_selected() {
        let selected_feats = vec!["Toughness".to_owned(), "Great Fortitude".to_owned()];
        assert!(standalone_skill_facts_from_feats(&selected_feats).is_empty());
    }

    #[test]
    fn acrobatic_grounds_its_two_uncomputed_skills() {
        let selected_feats = vec!["Acrobatic".to_owned()];
        assert_eq!(
            standalone_skill_facts_from_feats(&selected_feats),
            vec![fact("Acrobatic", "Acrobatics"), fact("Acrobatic", "Fly")]
        );
    }

    #[test]
    fn deceitful_grounds_bluff_and_disguise() {
        let selected_feats = vec!["Deceitful".to_owned()];
        assert_eq!(
            standalone_skill_facts_from_feats(&selected_feats),
            vec![fact("Deceitful", "Bluff"), fact("Deceitful", "Disguise")]
        );
    }

    #[test]
    fn persuasive_grounds_only_its_diplomacy_half_here_not_intimidate() {
        // Persuasive's Intimidate half is a *computed* skill, grounded by
        // `skill_bonuses_from_feats`. Only its Diplomacy half (an uncomputed
        // skill) is a standalone fact -- the two functions partition
        // Persuasive's real +2/+2 effect cleanly, with no skill double-counted.
        let selected_feats = vec!["Persuasive".to_owned()];
        assert_eq!(
            standalone_skill_facts_from_feats(&selected_feats),
            vec![fact("Persuasive", "Diplomacy")]
        );
    }

    #[test]
    fn athletic_grounds_no_standalone_fact_because_both_its_skills_are_computed() {
        // Athletic's Climb and Swim are both computed skills, fully grounded by
        // `skill_bonuses_from_feats`; it contributes nothing here.
        let selected_feats = vec!["Athletic".to_owned()];
        assert!(standalone_skill_facts_from_feats(&selected_feats).is_empty());
    }

    #[test]
    fn every_grounded_feat_produces_its_exact_verified_facts_in_corpus_order() {
        let selected_feats = vec![
            "Acrobatic".to_owned(),
            "Alertness".to_owned(),
            "Animal Affinity".to_owned(),
            "Deceitful".to_owned(),
            "Deft Hands".to_owned(),
            "Magical Aptitude".to_owned(),
            "Persuasive".to_owned(),
            "Self-Sufficient".to_owned(),
            "Stealthy".to_owned(),
        ];
        assert_eq!(
            standalone_skill_facts_from_feats(&selected_feats),
            vec![
                fact("Acrobatic", "Acrobatics"),
                fact("Acrobatic", "Fly"),
                fact("Alertness", "Perception"),
                fact("Alertness", "Sense Motive"),
                fact("Animal Affinity", "Handle Animal"),
                fact("Animal Affinity", "Ride"),
                fact("Deceitful", "Bluff"),
                fact("Deceitful", "Disguise"),
                fact("Deft Hands", "Disable Device"),
                fact("Deft Hands", "Sleight of Hand"),
                fact("Magical Aptitude", "Spellcraft"),
                fact("Magical Aptitude", "Use Magic Device"),
                fact("Persuasive", "Diplomacy"),
                fact("Self-Sufficient", "Heal"),
                fact("Self-Sufficient", "Survival"),
                fact("Stealthy", "Escape Artist"),
                fact("Stealthy", "Stealth"),
            ]
        );
    }

    #[test]
    fn does_not_match_a_substring_or_prefix_of_a_feat_key() {
        // "Acrobatic Steps" is a real, distinct CRB feat that begins with the
        // grounded "Acrobatic" key; it must not trigger Acrobatic's facts.
        let selected_feats = vec!["Acrobatic Steps".to_owned()];
        assert!(standalone_skill_facts_from_feats(&selected_feats).is_empty());
    }
}

#[cfg(test)]
mod skill_focus_facts_from_choices_tests {
    use super::*;

    fn choice(set: &str, sel: &str) -> SelectedChoice {
        SelectedChoice { choice_set_id: set.to_owned(), selection_id: sel.to_owned() }
    }

    fn target(skill: &str) -> SelectedChoice {
        choice("choice:skill_focus_target", &format!("skill:{skill}"))
    }

    fn fact(skill: &str) -> SkillFocusFact {
        SkillFocusFact { skill_name: skill.to_owned(), bonus: 3 }
    }

    #[test]
    fn grounds_nothing_for_empty_inputs() {
        assert!(skill_focus_facts_from_choices(&[], &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_when_skill_focus_is_selected_but_no_target_choice_is_present() {
        // The load-bearing no-fabrication case: Skill Focus with no explicit
        // target choice grounds NOTHING (no silently-seeded canonical skill).
        let selected_feats = vec!["Skill Focus".to_owned()];
        assert!(skill_focus_facts_from_choices(&selected_feats, &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_for_an_orphan_target_choice_without_the_feat() {
        // A target choice with no Skill Focus in selected_feats is an orphan --
        // ground nothing rather than fabricate a bonus for an unpicked feat.
        let choices = vec![target("Stealth")];
        assert!(skill_focus_facts_from_choices(&[], &choices).is_empty());
    }

    #[test]
    fn grounds_the_chosen_skill_when_feat_and_explicit_choice_are_both_present() {
        let selected_feats = vec!["Skill Focus".to_owned()];
        let choices = vec![target("Stealth")];
        assert_eq!(
            skill_focus_facts_from_choices(&selected_feats, &choices),
            vec![fact("Stealth")]
        );
    }

    #[test]
    fn grounds_a_multi_word_skill_name_verbatim() {
        // Skill names carry spaces ("Sleight of Hand") but never colons, so the
        // "skill:" prefix strip recovers the full name intact.
        let selected_feats = vec!["Skill Focus".to_owned()];
        let choices = vec![target("Sleight of Hand")];
        assert_eq!(
            skill_focus_facts_from_choices(&selected_feats, &choices),
            vec![fact("Sleight of Hand")]
        );
    }

    #[test]
    fn grounds_one_fact_per_target_when_skill_focus_is_taken_more_than_once() {
        // Skill Focus is legally repeatable (once per skill); each explicit
        // target choice grounds its own +3 fact, in input order.
        let selected_feats = vec!["Skill Focus".to_owned()];
        let choices = vec![target("Stealth"), target("Perception")];
        assert_eq!(
            skill_focus_facts_from_choices(&selected_feats, &choices),
            vec![fact("Stealth"), fact("Perception")]
        );
    }

    #[test]
    fn ignores_a_choice_from_a_different_choice_set() {
        // A Wizard school-specialization choice is not a Skill Focus target.
        let selected_feats = vec!["Skill Focus".to_owned()];
        let choices =
            vec![choice("choice:wizard_school_specialization", "school:evocation")];
        assert!(skill_focus_facts_from_choices(&selected_feats, &choices).is_empty());
    }

    #[test]
    fn ignores_a_target_choice_whose_selection_lacks_the_skill_prefix() {
        // A selection_id that isn't a "skill:<name>" is not a valid skill
        // target -- don't fabricate a fact from a malformed/wrong-kind value.
        let selected_feats = vec!["Skill Focus".to_owned()];
        let choices = vec![
            choice("choice:skill_focus_target", "Stealth"),
            choice("choice:skill_focus_target", "school:evocation"),
        ];
        assert!(skill_focus_facts_from_choices(&selected_feats, &choices).is_empty());
    }

    #[test]
    fn ignores_an_empty_skill_name_after_the_prefix() {
        let selected_feats = vec!["Skill Focus".to_owned()];
        let choices = vec![choice("choice:skill_focus_target", "skill:")];
        assert!(skill_focus_facts_from_choices(&selected_feats, &choices).is_empty());
    }

    #[test]
    fn grounds_only_one_fact_when_the_same_skill_is_targeted_twice() {
        // Skill Focus is STACK:NO MULT:YES in the corpus (verified directly
        // against cr_feats.lst): repeatable across DIFFERENT skills, but two
        // instances naming the same skill do not stack. Grounding two records
        // would also collide on one explanation id downstream, since the
        // consumer derives that id from the skill name.
        let selected_feats = vec!["Skill Focus".to_owned()];
        let choices = vec![target("Stealth"), target("Stealth")];
        assert_eq!(
            skill_focus_facts_from_choices(&selected_feats, &choices),
            vec![fact("Stealth")]
        );
    }

    #[test]
    fn treats_case_variants_of_one_skill_as_the_same_target() {
        // The consumer lowercases the skill name to build its explanation id
        // ("feat.skill_focus_bonus.stealth"), so "Stealth" and "stealth" would
        // collide on that id -- exactly the duplicate-record symptom this dedup
        // exists to prevent. The first spelling seen is the one kept.
        let selected_feats = vec!["Skill Focus".to_owned()];
        let choices = vec![target("Stealth"), target("stealth")];
        assert_eq!(
            skill_focus_facts_from_choices(&selected_feats, &choices),
            vec![fact("Stealth")]
        );
    }

    #[test]
    fn dedup_keeps_every_distinct_skill_and_preserves_input_order() {
        // Guards the opposite failure: dedup must not over-collapse genuinely
        // different targets, which is the whole point of MULT:YES.
        let selected_feats = vec!["Skill Focus".to_owned()];
        let choices = vec![
            target("Perception"),
            target("Stealth"),
            target("Perception"),
            target("Sleight of Hand"),
        ];
        assert_eq!(
            skill_focus_facts_from_choices(&selected_feats, &choices),
            vec![fact("Perception"), fact("Stealth"), fact("Sleight of Hand")]
        );
    }
}

#[cfg(test)]
mod effective_feats_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|key| (*key).to_owned()).collect()
    }

    #[test]
    fn is_empty_when_nothing_is_selected_or_granted() {
        assert!(effective_feats(&[], &[]).is_empty());
    }

    #[test]
    fn passes_selected_feats_through_unchanged_when_nothing_is_granted() {
        let selected = feats(&["Toughness", "Improved Initiative"]);
        assert_eq!(effective_feats(&selected, &[]), selected);
    }

    #[test]
    fn appends_a_class_granted_feat_the_player_did_not_choose() {
        // The whole point: a Ranger of level 3+ has Endurance automatically and a
        // Monk has Stunning Fist automatically, neither via selected_feats.
        let selected = feats(&["Toughness"]);
        assert_eq!(
            effective_feats(&selected, &["Endurance"]),
            feats(&["Toughness", "Endurance"])
        );
    }

    #[test]
    fn does_not_duplicate_a_granted_feat_the_player_also_chose() {
        // A Ranger who separately picked Endurance from the catalog must not end
        // up holding it twice.
        let selected = feats(&["Endurance", "Toughness"]);
        assert_eq!(
            effective_feats(&selected, &["Endurance"]),
            feats(&["Endurance", "Toughness"])
        );
    }

    #[test]
    fn preserves_repeated_selections_of_a_genuinely_repeatable_feat() {
        // Load-bearing: Fleet is STACK:YES MULT:YES and base_speed_bonus_from_feats
        // COUNTS occurrences, so this must not collapse duplicates in
        // selected_feats. Naively deduping the whole list would silently halve a
        // real +10 base speed to +5.
        let selected = feats(&["Fleet", "Fleet"]);
        assert_eq!(effective_feats(&selected, &["Endurance"]), feats(&["Fleet", "Fleet", "Endurance"]));
        assert_eq!(base_speed_bonus_from_feats(&effective_feats(&selected, &[])), 10);
    }

    #[test]
    fn appends_several_granted_feats_in_the_order_given() {
        // Monk grants both Improved Unarmed Strike and Stunning Fist at 1st level.
        let selected = feats(&["Dodge"]);
        assert_eq!(
            effective_feats(&selected, &["Improved Unarmed Strike", "Stunning Fist"]),
            feats(&["Dodge", "Improved Unarmed Strike", "Stunning Fist"])
        );
    }

    #[test]
    fn a_granted_feat_actually_reaches_the_producers_that_key_on_it() {
        // End-to-end proof this closes the real defect: a Ranger who never chose
        // Endurance still gets its verified +4, and a Monk who never chose
        // Stunning Fist still gets its real DC/uses.
        let ranger = effective_feats(&[], &["Endurance"]);
        assert_eq!(endurance_check_bonus_from_feats(&ranger), 4);

        let monk = effective_feats(&[], &["Stunning Fist"]);
        assert_eq!(
            stunning_fist_facts_from_feats(&monk, 1, 1, 2),
            Some(StunningFistFacts { save_dc: 12, uses_per_day: 1 })
        );
    }
}

#[cfg(test)]
mod initiative_bonus_from_feats_tests {
    use super::*;

    #[test]
    fn grants_no_bonus_for_an_empty_feat_list() {
        assert_eq!(initiative_bonus_from_feats(&[]), 0);
    }

    #[test]
    fn grants_no_bonus_when_improved_initiative_is_not_selected() {
        let selected_feats = vec!["Toughness".to_owned(), "Dodge".to_owned()];
        assert_eq!(initiative_bonus_from_feats(&selected_feats), 0);
    }

    #[test]
    fn grants_the_real_flat_plus_four_when_improved_initiative_is_selected() {
        let selected_feats = vec!["Toughness".to_owned(), "Improved Initiative".to_owned()];
        assert_eq!(initiative_bonus_from_feats(&selected_feats), 4);
    }

    #[test]
    fn does_not_match_a_partial_feat_name() {
        // Exact catalog-key equality, the same discipline every other producer
        // in this module uses -- no substring/prefix matching.
        let selected_feats = vec!["Initiative".to_owned(), "Improved".to_owned()];
        assert_eq!(initiative_bonus_from_feats(&selected_feats), 0);
    }

    #[test]
    fn is_not_double_counted_if_the_same_feat_string_appears_twice() {
        // Improved Initiative carries no STACK:YES/MULT:YES in the corpus --
        // unlike Fleet, it is not repeatable, so a duplicated string stays +4.
        let selected_feats =
            vec!["Improved Initiative".to_owned(), "Improved Initiative".to_owned()];
        assert_eq!(initiative_bonus_from_feats(&selected_feats), 4);
    }
}

#[cfg(test)]
mod endurance_check_bonus_from_feats_tests {
    use super::*;

    #[test]
    fn grants_no_bonus_for_an_empty_feat_list() {
        assert_eq!(endurance_check_bonus_from_feats(&[]), 0);
    }

    #[test]
    fn grants_no_bonus_when_endurance_is_not_selected() {
        let selected_feats = vec!["Diehard".to_owned(), "Toughness".to_owned()];
        assert_eq!(endurance_check_bonus_from_feats(&selected_feats), 0);
    }

    #[test]
    fn grants_the_real_flat_plus_four_when_endurance_is_selected() {
        let selected_feats = vec!["Endurance".to_owned()];
        assert_eq!(endurance_check_bonus_from_feats(&selected_feats), 4);
    }

    #[test]
    fn is_not_double_counted_if_the_same_feat_string_appears_twice() {
        // Endurance carries no STACK:YES/MULT:YES in the corpus -- not repeatable.
        let selected_feats = vec!["Endurance".to_owned(), "Endurance".to_owned()];
        assert_eq!(endurance_check_bonus_from_feats(&selected_feats), 4);
    }
}

#[cfg(test)]
mod combat_maneuver_bonuses_from_feats_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|key| (*key).to_owned()).collect()
    }

    #[test]
    fn grounds_nothing_for_an_empty_feat_list() {
        assert!(combat_maneuver_bonuses_from_feats(&[]).is_empty());
    }

    #[test]
    fn grounds_nothing_when_no_maneuver_feat_is_selected() {
        let selected = feats(&["Toughness", "Improved Initiative"]);
        assert!(combat_maneuver_bonuses_from_feats(&selected).is_empty());
    }

    #[test]
    fn an_improved_feat_grounds_both_the_offensive_and_defensive_halves() {
        // Corpus: BONUS:VAR|CMB_Trip,CMD_Trip|2 -- one token, two dimensions.
        let selected = feats(&["Improved Trip"]);
        assert_eq!(
            combat_maneuver_bonuses_from_feats(&selected),
            vec![CombatManeuverFeatBonus {
                feat_key: "Improved Trip",
                maneuver: "Trip",
                cmb_bonus: 2,
                cmd_bonus: 2,
            }]
        );
    }

    #[test]
    fn a_greater_feat_grounds_the_offensive_half_only() {
        // Corpus: BONUS:VAR|CMB_Trip|2 -- no CMD term, unlike the Improved version.
        let selected = feats(&["Greater Trip"]);
        assert_eq!(
            combat_maneuver_bonuses_from_feats(&selected),
            vec![CombatManeuverFeatBonus {
                feat_key: "Greater Trip",
                maneuver: "Trip",
                cmb_bonus: 2,
                cmd_bonus: 0,
            }]
        );
    }

    #[test]
    fn the_improved_and_greater_versions_ground_as_two_separate_stacking_facts() {
        // The corpus BENEFIT for every Greater maneuver feat says explicitly that
        // its bonus "stacks with the bonus granted by Improved <maneuver>", so
        // both ground rather than one superseding the other.
        let selected = feats(&["Improved Trip", "Greater Trip"]);
        let grounded = combat_maneuver_bonuses_from_feats(&selected);
        assert_eq!(grounded.len(), 2);
        assert_eq!(grounded.iter().map(|b| b.cmb_bonus).sum::<i16>(), 4);
        assert_eq!(grounded.iter().map(|b| b.cmd_bonus).sum::<i16>(), 2);
    }

    #[test]
    fn grounds_all_twelve_maneuver_feats_in_corpus_order() {
        let selected = feats(&[
            "Improved Trip",
            "Greater Trip",
            "Improved Bull Rush",
            "Greater Bull Rush",
            "Improved Disarm",
            "Greater Disarm",
            "Improved Grapple",
            "Greater Grapple",
            "Improved Overrun",
            "Greater Overrun",
            "Improved Sunder",
            "Greater Sunder",
        ]);
        let grounded = combat_maneuver_bonuses_from_feats(&selected);
        assert_eq!(grounded.len(), 12);
        // Emitted in the stable corpus source order of feat_data/combat.rs
        // (all six Greater records precede all six Improved records there),
        // NOT in the caller's selection order.
        assert_eq!(
            grounded.iter().map(|b| b.feat_key).collect::<Vec<_>>(),
            vec![
                "Greater Bull Rush",
                "Greater Disarm",
                "Greater Grapple",
                "Greater Overrun",
                "Greater Sunder",
                "Greater Trip",
                "Improved Bull Rush",
                "Improved Disarm",
                "Improved Grapple",
                "Improved Overrun",
                "Improved Sunder",
                "Improved Trip",
            ]
        );
        // Every one of the twelve carries the same verified +2 offensive magnitude.
        assert!(grounded.iter().all(|b| b.cmb_bonus == 2));
        // Exactly the six Improved records carry the defensive half.
        assert_eq!(grounded.iter().filter(|b| b.cmd_bonus == 2).count(), 6);
    }

    #[test]
    fn does_not_match_a_substring_or_prefix_of_a_feat_key() {
        // "Improved Trip" must not be matched by a longer or shorter string.
        let selected = feats(&["Trip", "Improved", "Improved Tripping"]);
        assert!(combat_maneuver_bonuses_from_feats(&selected).is_empty());
    }

    #[test]
    fn is_not_double_counted_if_the_same_feat_string_appears_twice() {
        // No maneuver feat carries STACK:YES/MULT:YES -- none is repeatable.
        let selected = feats(&["Improved Grapple", "Improved Grapple"]);
        assert_eq!(combat_maneuver_bonuses_from_feats(&selected).len(), 1);
    }
}

#[cfg(test)]
mod stunning_fist_facts_from_feats_tests {
    use super::*;

    #[test]
    fn grounds_nothing_when_the_feat_is_absent() {
        assert_eq!(stunning_fist_facts_from_feats(&[], 1, 1, 2), None);
    }

    #[test]
    fn grounds_a_level_one_monks_real_dc_and_uses_per_day() {
        // DC = 10 + (TL/2) + WIS = 10 + 0 + 2 = 12.
        // Uses = MonkLVL + floor((TL - MonkLVL)/4) = 1 + 0 = 1.
        let selected = vec!["Stunning Fist".to_owned()];
        assert_eq!(
            stunning_fist_facts_from_feats(&selected, 1, 1, 2),
            Some(StunningFistFacts { save_dc: 12, uses_per_day: 1 })
        );
    }

    #[test]
    fn a_non_monk_gets_the_real_once_per_four_levels_rate() {
        // A Fighter (monk_level 0) at level 8 with WIS +1:
        // DC = 10 + 4 + 1 = 15. Uses = 0 + floor(8/4) = 2, matching the corpus
        // BENEFIT's "once per day for every four levels you have attained".
        let selected = vec!["Stunning Fist".to_owned()];
        assert_eq!(
            stunning_fist_facts_from_feats(&selected, 8, 0, 1),
            Some(StunningFistFacts { save_dc: 15, uses_per_day: 2 })
        );
    }

    #[test]
    fn a_multiclass_monk_adds_monk_levels_to_the_quarter_rate_of_the_rest() {
        // Monk 5 / other 3 (TL 8), WIS +0: DC = 10 + 4 + 0 = 14.
        // Uses = 5 + floor(3/4) = 5 + 0 = 5.
        let selected = vec!["Stunning Fist".to_owned()];
        assert_eq!(
            stunning_fist_facts_from_feats(&selected, 8, 5, 0),
            Some(StunningFistFacts { save_dc: 14, uses_per_day: 5 })
        );
    }

    #[test]
    fn applies_a_negative_wisdom_modifier_faithfully_to_the_dc() {
        // The corpus formula adds WIS verbatim; clamping would fabricate a value
        // the corpus does not specify, the same reasoning Intimidating Prowess
        // already documents for a negative Strength modifier.
        let selected = vec!["Stunning Fist".to_owned()];
        assert_eq!(
            stunning_fist_facts_from_feats(&selected, 1, 1, -1),
            Some(StunningFistFacts { save_dc: 9, uses_per_day: 1 })
        );
    }

    #[test]
    fn does_not_underflow_when_monk_level_exceeds_total_level() {
        // Not a reachable posture (monk levels are a subset of total levels), but
        // the subtraction must not panic on an inconsistent caller.
        let selected = vec!["Stunning Fist".to_owned()];
        assert_eq!(
            stunning_fist_facts_from_feats(&selected, 1, 5, 0),
            Some(StunningFistFacts { save_dc: 10, uses_per_day: 5 })
        );
    }
}

#[cfg(test)]
mod movement_feat_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|key| (*key).to_owned()).collect()
    }

    #[test]
    fn grants_no_difficult_terrain_movement_for_an_empty_feat_list() {
        assert_eq!(difficult_terrain_feet_from_feats(&[]), 0);
    }

    #[test]
    fn nimble_moves_alone_grants_its_real_five_feet() {
        assert_eq!(difficult_terrain_feet_from_feats(&feats(&["Nimble Moves"])), 5);
    }

    #[test]
    fn acrobatic_steps_alone_grants_its_real_fifteen_feet() {
        // Acrobatic Steps' real prerequisite is Nimble Moves, but prerequisite
        // validation belongs to feat_prereqs, not this module -- this producer
        // reports the corpus magnitude of whatever is actually selected.
        assert_eq!(difficult_terrain_feet_from_feats(&feats(&["Acrobatic Steps"])), 15);
    }

    #[test]
    fn nimble_moves_and_acrobatic_steps_stack_to_the_corpus_stated_twenty_feet() {
        // The corpus BENEFIT for Acrobatic Steps states the total explicitly:
        // "allowing you to move normally through a total of 20 feet".
        let selected = feats(&["Nimble Moves", "Acrobatic Steps"]);
        assert_eq!(difficult_terrain_feet_from_feats(&selected), 20);
    }

    #[test]
    fn grants_no_speed_bonus_when_fleet_is_absent() {
        assert_eq!(base_speed_bonus_from_feats(&feats(&["Nimble Moves"])), 0);
    }

    #[test]
    fn fleet_grants_its_real_five_feet_of_base_speed() {
        assert_eq!(base_speed_bonus_from_feats(&feats(&["Fleet"])), 5);
    }

    #[test]
    fn fleet_stacks_with_itself_because_the_corpus_marks_it_repeatable() {
        // Fleet carries STACK:YES MULT:YES -- unlike every other feat this module
        // grounds, it is genuinely repeatable, so occurrences are COUNTED rather
        // than merely detected.
        let selected = feats(&["Fleet", "Fleet", "Fleet"]);
        assert_eq!(base_speed_bonus_from_feats(&selected), 15);
    }
}

#[cfg(test)]
mod spell_focus_facts_from_choices_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|key| (*key).to_owned()).collect()
    }

    fn choice(choice_set_id: &str, selection_id: &str) -> SelectedChoice {
        SelectedChoice {
            choice_set_id: choice_set_id.to_owned(),
            selection_id: selection_id.to_owned(),
        }
    }

    fn base(school: &str) -> SelectedChoice {
        choice("choice:spell_focus_target", &format!("school:{school}"))
    }

    fn greater(school: &str) -> SelectedChoice {
        choice("choice:greater_spell_focus_target", &format!("school:{school}"))
    }

    fn fact(school: &str, dc_bonus: i16) -> SpellFocusFact {
        SpellFocusFact { school_name: school.to_owned(), dc_bonus }
    }

    #[test]
    fn grounds_nothing_for_empty_inputs() {
        assert!(spell_focus_facts_from_choices(&[], &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_when_the_feat_is_selected_but_no_target_choice_is_present() {
        // The same no-silent-seeding rule Skill Focus established: Spell Focus's
        // entire value IS which school was chosen, so never fabricate one.
        let selected = feats(&["Spell Focus"]);
        assert!(spell_focus_facts_from_choices(&selected, &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_for_an_orphan_target_choice_without_the_feat() {
        assert!(spell_focus_facts_from_choices(&[], &[base("evocation")]).is_empty());
    }

    #[test]
    fn grounds_the_real_plus_one_for_an_explicitly_chosen_school() {
        let selected = feats(&["Spell Focus"]);
        assert_eq!(
            spell_focus_facts_from_choices(&selected, &[base("evocation")]),
            vec![fact("evocation", 1)]
        );
    }

    #[test]
    fn grounds_one_fact_per_distinct_school_in_input_order() {
        // Spell Focus is MULT:YES -- repeatable across different schools.
        let selected = feats(&["Spell Focus"]);
        assert_eq!(
            spell_focus_facts_from_choices(&selected, &[base("evocation"), base("necromancy")]),
            vec![fact("evocation", 1), fact("necromancy", 1)]
        );
    }

    #[test]
    fn grounds_only_one_fact_when_the_same_school_is_targeted_twice() {
        // STACK:NO, exactly like Skill Focus.
        let selected = feats(&["Spell Focus"]);
        assert_eq!(
            spell_focus_facts_from_choices(&selected, &[base("evocation"), base("evocation")]),
            vec![fact("evocation", 1)]
        );
    }

    #[test]
    fn greater_spell_focus_raises_the_same_school_to_two_rather_than_adding_a_second_fact() {
        // THE load-bearing case. Greater Spell Focus's corpus token is `2`, but
        // its BENEFIT prose reads "+1 ... This bonus stacks with the bonus from
        // Spell Focus" -- i.e. the token is the TOTAL of the same-TYPE=SpellFocus
        // stack (PCGen takes the highest of same-typed bonuses), not an increment.
        // Summing the two tokens would assert +3, which is wrong.
        let selected = feats(&["Spell Focus", "Greater Spell Focus"]);
        assert_eq!(
            spell_focus_facts_from_choices(&selected, &[base("evocation"), greater("evocation")]),
            vec![fact("evocation", 2)]
        );
    }

    #[test]
    fn greater_spell_focus_grounds_nothing_for_a_school_without_the_base_feats_focus() {
        // Greater Spell Focus's own corpus record requires it
        // (PREABILITY:1,CATEGORY=FEAT,Spell Focus, and
        // CHOOSE:SCHOOLS|ABILITY=FEAT[Spell Focus] restricts its target to a
        // school you already have Spell Focus in), so this combination cannot
        // legally exist. Its magnitude is also genuinely ambiguous in isolation
        // -- RAW's "+1" vs the token's "2" -- so ground nothing rather than
        // assert a checkable number for an impossible build.
        let selected = feats(&["Spell Focus", "Greater Spell Focus"]);
        assert_eq!(
            spell_focus_facts_from_choices(
                &selected,
                &[base("evocation"), greater("necromancy")]
            ),
            vec![fact("evocation", 1)]
        );
    }

    #[test]
    fn upgrades_only_the_greater_targeted_school_leaving_others_at_one() {
        let selected = feats(&["Spell Focus", "Greater Spell Focus"]);
        assert_eq!(
            spell_focus_facts_from_choices(
                &selected,
                &[base("evocation"), base("necromancy"), greater("evocation")]
            ),
            vec![fact("evocation", 2), fact("necromancy", 1)]
        );
    }

    #[test]
    fn a_greater_target_without_the_greater_feat_does_not_upgrade_anything() {
        let selected = feats(&["Spell Focus"]);
        assert_eq!(
            spell_focus_facts_from_choices(&selected, &[base("evocation"), greater("evocation")]),
            vec![fact("evocation", 1)]
        );
    }

    #[test]
    fn treats_case_variants_of_one_school_as_the_same_target() {
        // Same reasoning as Skill Focus: the consumer lowercases the school name
        // to build its explanation id, so case variants must not emit two records.
        let selected = feats(&["Spell Focus"]);
        assert_eq!(
            spell_focus_facts_from_choices(&selected, &[base("Evocation"), base("evocation")]),
            vec![fact("Evocation", 1)]
        );
    }

    #[test]
    fn ignores_a_choice_from_a_different_choice_set_or_with_a_wrong_prefix() {
        let selected = feats(&["Spell Focus"]);
        let choices = vec![
            choice("choice:wizard_school_specialization", "school:evocation"),
            choice("choice:spell_focus_target", "evocation"),
            choice("choice:spell_focus_target", "school:"),
        ];
        assert!(spell_focus_facts_from_choices(&selected, &choices).is_empty());
    }
}

#[cfg(test)]
mod weapon_focus_facts_from_choices_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|key| (*key).to_owned()).collect()
    }

    fn choice(choice_set_id: &str, selection_id: &str) -> SelectedChoice {
        SelectedChoice {
            choice_set_id: choice_set_id.to_owned(),
            selection_id: selection_id.to_owned(),
        }
    }

    fn target(weapon: &str) -> SelectedChoice {
        choice("choice:weapon_focus_target", &format!("weapon:{weapon}"))
    }

    fn greater_target(weapon: &str) -> SelectedChoice {
        choice("choice:greater_weapon_focus_target", &format!("weapon:{weapon}"))
    }

    /// The pre-existing fixed-loadout representation: the target rides inside the
    /// Fighter bonus-feat choice as a compound selection id.
    fn legacy(weapon: &str) -> SelectedChoice {
        choice("choice:fighter_bonus_feat", &format!("feat:weapon_focus:weapon:{weapon}"))
    }

    fn fact(weapon: &str, attack_bonus: i16) -> WeaponFocusFact {
        WeaponFocusFact { weapon_name: weapon.to_owned(), attack_bonus }
    }

    #[test]
    fn grounds_nothing_for_empty_inputs() {
        assert!(weapon_focus_facts_from_choices(&[], &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_when_the_feat_is_held_but_no_target_is_chosen() {
        let selected = feats(&["Weapon Focus"]);
        assert!(weapon_focus_facts_from_choices(&selected, &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_for_an_orphan_target_without_the_feat() {
        assert!(weapon_focus_facts_from_choices(&[], &[target("longsword")]).is_empty());
        assert!(weapon_focus_facts_from_choices(&[], &[legacy("longsword")]).is_empty());
    }

    #[test]
    fn grounds_the_real_plus_one_from_the_catalog_key_and_new_choice_set() {
        let selected = feats(&["Weapon Focus"]);
        assert_eq!(
            weapon_focus_facts_from_choices(&selected, &[target("greataxe")]),
            vec![fact("greataxe", 1)]
        );
    }

    #[test]
    fn grounds_the_shipped_fighter_loadout_via_the_synthetic_id_and_legacy_compound() {
        // The real shipped Fighter fixture carries the ENGINE TOKEN in
        // selected_feats and expresses its target as a compound selection
        // inside choice:fighter_bonus_feat. Presence resolves through the
        // shared identity fold (which is why no per-feat token constant is
        // listed any more); the compound target still needs its own lookup.
        let selected = feats(&["feat:weapon_focus"]);
        assert_eq!(
            weapon_focus_facts_from_choices(&selected, &[legacy("longsword")]),
            vec![fact("longsword", 1)]
        );
    }

    #[test]
    fn the_legacy_compound_is_parsed_generically_not_hardcoded_to_longsword() {
        let selected = feats(&["feat:weapon_focus"]);
        assert_eq!(
            weapon_focus_facts_from_choices(&selected, &[legacy("warhammer")]),
            vec![fact("warhammer", 1)]
        );
    }

    #[test]
    fn both_representations_of_one_weapon_ground_a_single_fact() {
        // A character described both ways must not be double-counted to +2.
        let selected = feats(&["Weapon Focus", "feat:weapon_focus"]);
        assert_eq!(
            weapon_focus_facts_from_choices(&selected, &[target("longsword"), legacy("longsword")]),
            vec![fact("longsword", 1)]
        );
    }

    #[test]
    fn greater_weapon_focus_adds_a_second_plus_one_rather_than_capping_at_a_total() {
        // THE case that differs from Spell Focus. Weapon Focus and Greater Weapon
        // Focus write to two SEPARATE corpus variables (WeaponFocusToHit and
        // GreaterWeaponFocusToHit), each unconditionally 1, so they genuinely ADD
        // to +2. Reusing Spell Focus's take-highest shape here would yield
        // max(1,1) = 1 and understate the real bonus by 1.
        let selected = feats(&["Weapon Focus", "Greater Weapon Focus"]);
        assert_eq!(
            weapon_focus_facts_from_choices(
                &selected,
                &[target("longsword"), greater_target("longsword")]
            ),
            vec![fact("longsword", 2)]
        );
    }

    #[test]
    fn greater_weapon_focus_alone_grounds_its_own_unambiguous_plus_one() {
        // Deliberately UNLIKE Greater Spell Focus, which grounds nothing without
        // its base feat. The ratified distinguishing axis is ambiguity, not
        // prerequisites: Greater Weapon Focus's own magnitude is its own variable
        // worth exactly 1 under any reading, so reporting it asserts nothing
        // uncertain. Greater Spell Focus's was genuinely ambiguous in isolation.
        let selected = feats(&["Greater Weapon Focus"]);
        assert_eq!(
            weapon_focus_facts_from_choices(&selected, &[greater_target("rapier")]),
            vec![fact("rapier", 1)]
        );
    }

    #[test]
    fn grounds_one_fact_per_distinct_weapon_in_first_seen_order() {
        let selected = feats(&["Weapon Focus", "Greater Weapon Focus"]);
        assert_eq!(
            weapon_focus_facts_from_choices(
                &selected,
                &[target("longsword"), target("dagger"), greater_target("longsword")]
            ),
            vec![fact("longsword", 2), fact("dagger", 1)]
        );
    }

    #[test]
    fn the_same_weapon_targeted_twice_does_not_stack() {
        // Weapon Focus carries MULT:YES with no STACK token (PCGen defaults to
        // no-stack), so it is repeatable across weapons but never on one weapon.
        let selected = feats(&["Weapon Focus"]);
        assert_eq!(
            weapon_focus_facts_from_choices(&selected, &[target("longsword"), target("longsword")]),
            vec![fact("longsword", 1)]
        );
    }

    #[test]
    fn treats_case_variants_of_one_weapon_as_the_same_target() {
        let selected = feats(&["Weapon Focus"]);
        assert_eq!(
            weapon_focus_facts_from_choices(&selected, &[target("Longsword"), target("longsword")]),
            vec![fact("Longsword", 1)]
        );
    }

    #[test]
    fn ignores_unrelated_choices_and_malformed_selections() {
        let selected = feats(&["Weapon Focus"]);
        let choices = vec![
            // A Fighter bonus-feat slot holding something that is not Weapon Focus.
            choice("choice:fighter_bonus_feat", "feat:power_attack"),
            choice("choice:wizard_school_specialization", "school:evocation"),
            choice("choice:weapon_focus_target", "longsword"),
            choice("choice:weapon_focus_target", "weapon:"),
        ];
        assert!(weapon_focus_facts_from_choices(&selected, &choices).is_empty());
    }
}

#[cfg(test)]
mod chooser_feat_contract_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|key| (*key).to_owned()).collect()
    }

    fn choice(choice_set_id: &str, selection_id: &str) -> SelectedChoice {
        SelectedChoice {
            choice_set_id: choice_set_id.to_owned(),
            selection_id: selection_id.to_owned(),
        }
    }

    /// The drift guard this table exists to make possible.
    ///
    /// Every `*_TARGET_CHOICE_SET` constant in this file names a chooser
    /// feat whose target a producer reads. If one is ever added without a
    /// matching contract, a UI built on this table would silently offer no
    /// picker for it -- the feat would look targetless rather than
    /// unsupported. Scanning our own source is unusual, but the alternative
    /// is a hand-maintained count that decays exactly the way every other
    /// hand-maintained inventory in this repo has.
    #[test]
    fn chooser_contracts_cover_every_target_choice_set() {
        // Scan only the production region. Past the first `#[cfg(test)]`
        // the file contains this very filter string, so scanning the whole
        // file makes the test match its own source and fail on a constant
        // that does not exist.
        let full = include_str!("feat_effects.rs");
        let source = full.split("\n#[cfg(test)]").next().expect("source is non-empty");
        let declared: Vec<&str> = source
            .lines()
            .filter(|line| line.contains("_TARGET_CHOICE_SET: &str"))
            .filter_map(|line| line.split('"').nth(1))
            .collect();
        // Multi-line constants put the literal on the following line, so
        // pick those up too rather than silently undercounting.
        let continued: Vec<&str> = source
            .lines()
            .zip(source.lines().skip(1))
            .filter(|(decl, _)| {
                decl.contains("_TARGET_CHOICE_SET: &str") && !decl.contains('"')
            })
            .filter_map(|(_, value)| value.split('"').nth(1))
            .collect();

        let mut all: Vec<&str> = declared.into_iter().chain(continued).collect();
        all.sort_unstable();
        all.dedup();
        assert!(
            all.len() >= 9,
            "expected at least the 9 known chooser choice sets, found {all:?}"
        );

        for choice_set in all {
            assert!(
                CHOOSER_FEAT_CONTRACTS
                    .iter()
                    .any(|contract| contract.choice_set_id == choice_set),
                "{choice_set} has no entry in CHOOSER_FEAT_CONTRACTS -- a chooser feat was \
                 added without a contract, so nothing can offer a picker for it"
            );
        }
    }

    /// Every contract must resolve from every shape a real `selected_feats`
    /// entry can carry -- not just the shapes some table happened to list.
    /// The token and compound forms are derived from the key here rather than
    /// declared, so a newly added chooser feat is covered automatically.
    #[test]
    fn every_contract_is_reachable_from_every_real_selection_shape() {
        for contract in CHOOSER_FEAT_CONTRACTS {
            let token = format!(
                "feat:{}",
                contract.feat_key.to_lowercase().replace(['\'', '-'], "").replace(' ', "_")
            );
            let compound = format!("{token}:weapon:longsword");
            for shape in [contract.feat_key.to_owned(), token.clone(), compound] {
                assert_eq!(
                    chooser_contract_for_feat(&shape),
                    Some(contract),
                    "{} must resolve from the shape {shape:?}",
                    contract.feat_key
                );
            }
        }
    }

    #[test]
    fn a_feat_that_takes_no_target_has_no_contract() {
        assert_eq!(chooser_contract_for_feat("Toughness"), None);
        assert_eq!(chooser_contract_for_feat("feat:power_attack"), None);
    }

    #[test]
    fn resolves_a_recorded_target_with_its_kind() {
        let resolved = chosen_feat_targets(
            &feats(&["Weapon Focus"]),
            &[choice("choice:weapon_focus_target", "weapon:Longsword")],
        );
        assert_eq!(
            resolved,
            vec![ChosenFeatTargets {
                feat_id: "Weapon Focus".to_owned(),
                target_kind: ChooserTargetKind::Weapon,
                targets: vec!["Longsword".to_owned()],
            }]
        );
    }

    /// The state the sheet most needs to distinguish: the feat is held, but
    /// no target was ever recorded. It must be reported, not omitted, and
    /// must not be filled in with a default.
    #[test]
    fn a_chooser_feat_with_no_recorded_target_is_reported_with_an_empty_list() {
        let resolved = chosen_feat_targets(&feats(&["Skill Focus"]), &[]);
        assert_eq!(resolved.len(), 1, "the feat must still be reported");
        assert_eq!(resolved[0].target_kind, ChooserTargetKind::Skill);
        assert!(resolved[0].targets.is_empty(), "nothing may be seeded: {resolved:?}");
    }

    #[test]
    fn feats_that_take_no_target_are_absent_entirely() {
        assert!(chosen_feat_targets(&feats(&["Toughness", "Dodge"]), &[]).is_empty());
    }

    /// Two picks of one MULT:YES feat report both targets under one entry.
    /// Pairing pick N with target N is not recorded anywhere, so it is not
    /// invented here.
    #[test]
    fn a_feat_taken_twice_reports_both_targets_once() {
        let resolved = chosen_feat_targets(
            &feats(&["Weapon Focus", "Weapon Focus"]),
            &[
                choice("choice:weapon_focus_target", "weapon:Longsword"),
                choice("choice:weapon_focus_target", "weapon:Rapier"),
            ],
        );
        assert_eq!(resolved.len(), 1, "one entry per feat, not per pick: {resolved:?}");
        assert_eq!(resolved[0].targets, vec!["Longsword".to_owned(), "Rapier".to_owned()]);
    }

    /// The shipped deterministic fixture records Weapon Focus's target
    /// through the Fighter bonus-feat slot, not the clean choice set. The
    /// producer honours that form, so this must too -- otherwise the one
    /// character that actually has a target would display none.
    #[test]
    fn the_legacy_compound_form_still_resolves_a_target() {
        let resolved = chosen_feat_targets(
            &feats(&["feat:weapon_focus"]),
            &[choice("choice:fighter_bonus_feat", "feat:weapon_focus:weapon:longsword")],
        );
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].targets, vec!["longsword".to_owned()]);
    }

    #[test]
    fn each_target_kind_resolves_for_a_representative_feat() {
        let cases = [
            ("Skill Focus", "choice:skill_focus_target", "skill:Perception", ChooserTargetKind::Skill),
            (
                "Spell Focus",
                "choice:spell_focus_target",
                "school:Evocation",
                ChooserTargetKind::SpellSchool,
            ),
            (
                "Improved Critical",
                "choice:improved_critical_target",
                "weapon:Rapier",
                ChooserTargetKind::Weapon,
            ),
        ];
        for (feat, set, selection, kind) in cases {
            let resolved = chosen_feat_targets(&feats(&[feat]), &[choice(set, selection)]);
            assert_eq!(resolved.len(), 1, "{feat} must resolve");
            assert_eq!(resolved[0].target_kind, kind, "{feat} target kind");
            assert_eq!(resolved[0].targets.len(), 1, "{feat} must carry its target");
        }
    }

    /// A target recorded against the wrong choice set must not be picked up
    /// by a different feat -- the same cross-contamination guard the
    /// Specialization producer carries.
    #[test]
    fn a_target_from_another_feats_choice_set_is_not_read() {
        let resolved = chosen_feat_targets(
            &feats(&["Weapon Specialization"]),
            &[choice("choice:weapon_focus_target", "weapon:Longsword")],
        );
        assert_eq!(resolved.len(), 1);
        assert!(
            resolved[0].targets.is_empty(),
            "Weapon Focus's target is not Weapon Specialization's: {resolved:?}"
        );
    }
}

#[cfg(test)]
mod weapon_specialization_facts_from_choices_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|key| (*key).to_owned()).collect()
    }

    fn choice(choice_set_id: &str, selection_id: &str) -> SelectedChoice {
        SelectedChoice {
            choice_set_id: choice_set_id.to_owned(),
            selection_id: selection_id.to_owned(),
        }
    }

    fn target(weapon: &str) -> SelectedChoice {
        choice("choice:weapon_specialization_target", &format!("weapon:{weapon}"))
    }

    fn greater_target(weapon: &str) -> SelectedChoice {
        choice("choice:greater_weapon_specialization_target", &format!("weapon:{weapon}"))
    }

    fn fact(weapon: &str, damage_bonus: i16) -> WeaponSpecializationFact {
        WeaponSpecializationFact { weapon_name: weapon.to_owned(), damage_bonus }
    }

    #[test]
    fn grounds_nothing_for_empty_inputs() {
        assert!(weapon_specialization_facts_from_choices(&[], &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_when_the_feat_is_held_but_no_target_is_chosen() {
        let selected = feats(&["Weapon Specialization"]);
        assert!(weapon_specialization_facts_from_choices(&selected, &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_for_an_orphaned_target_with_no_feat() {
        assert!(weapon_specialization_facts_from_choices(&[], &[target("longsword")]).is_empty());
    }

    #[test]
    fn grounds_the_real_damage_bonus_for_a_chosen_weapon() {
        let selected = feats(&["Weapon Specialization"]);
        assert_eq!(
            weapon_specialization_facts_from_choices(&selected, &[target("Longsword")]),
            vec![fact("Longsword", 2)]
        );
    }

    #[test]
    fn accepts_the_synthetic_feat_id_as_well_as_the_catalog_key() {
        let selected = feats(&["feat:weapon_specialization"]);
        assert_eq!(
            weapon_specialization_facts_from_choices(&selected, &[target("Rapier")]),
            vec![fact("Rapier", 2)]
        );
    }

    /// The corpus fact this family turns on: Greater ADDS to base rather than
    /// replacing or taking-highest, per its own BENEFIT text.
    #[test]
    fn greater_stacks_additively_with_base_on_the_same_weapon() {
        let selected = feats(&["Weapon Specialization", "Greater Weapon Specialization"]);
        assert_eq!(
            weapon_specialization_facts_from_choices(
                &selected,
                &[target("Longsword"), greater_target("Longsword")]
            ),
            vec![fact("Longsword", 4)],
            "+2 and +2 must total +4, not take-highest +2"
        );
    }

    #[test]
    fn greater_grounds_alone_on_its_own_unambiguous_bonus() {
        let selected = feats(&["Greater Weapon Specialization"]);
        assert_eq!(
            weapon_specialization_facts_from_choices(&selected, &[greater_target("Rapier")]),
            vec![fact("Rapier", 2)]
        );
    }

    #[test]
    fn deduplicates_repeated_targets_case_insensitively() {
        let selected = feats(&["Weapon Specialization"]);
        assert_eq!(
            weapon_specialization_facts_from_choices(
                &selected,
                &[target("Longsword"), target("longsword")]
            ),
            vec![fact("Longsword", 2)],
            "one weapon must not ground twice under one id"
        );
    }

    /// Guards the mistake this whole family invites: a Focus target must never
    /// be read as a Specialization target, and vice versa. If the choice-set
    /// constants were ever crossed, the two feats' magnitudes would land on
    /// each other's roll type.
    #[test]
    fn does_not_read_focus_targets_or_malformed_selections() {
        let selected = feats(&["Weapon Specialization"]);
        let choices = vec![
            choice("choice:weapon_focus_target", "weapon:longsword"),
            choice("choice:greater_weapon_focus_target", "weapon:longsword"),
            choice("choice:weapon_specialization_target", "longsword"),
            choice("choice:weapon_specialization_target", "weapon:"),
        ];
        assert!(weapon_specialization_facts_from_choices(&selected, &choices).is_empty());
    }
}

#[cfg(test)]
mod improved_critical_targets_from_choices_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|key| (*key).to_owned()).collect()
    }

    fn choice(choice_set_id: &str, selection_id: &str) -> SelectedChoice {
        SelectedChoice {
            choice_set_id: choice_set_id.to_owned(),
            selection_id: selection_id.to_owned(),
        }
    }

    fn target(weapon: &str) -> SelectedChoice {
        choice("choice:improved_critical_target", &format!("weapon:{weapon}"))
    }

    #[test]
    fn grounds_nothing_for_empty_inputs() {
        assert!(improved_critical_targets_from_choices(&[], &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_when_the_feat_is_held_but_no_target_is_chosen() {
        let selected = feats(&["Improved Critical"]);
        assert!(improved_critical_targets_from_choices(&selected, &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_for_an_orphaned_target_with_no_feat() {
        assert!(improved_critical_targets_from_choices(&[], &[target("longsword")]).is_empty());
    }

    #[test]
    fn returns_the_chosen_weapon_for_either_feat_id() {
        assert_eq!(
            improved_critical_targets_from_choices(
                &feats(&["Improved Critical"]),
                &[target("Longsword")]
            ),
            vec!["Longsword".to_owned()]
        );
        assert_eq!(
            improved_critical_targets_from_choices(
                &feats(&["feat:improved_critical"]),
                &[target("Rapier")]
            ),
            vec!["Rapier".to_owned()]
        );
    }

    #[test]
    fn deduplicates_repeated_targets_case_insensitively() {
        assert_eq!(
            improved_critical_targets_from_choices(
                &feats(&["Improved Critical"]),
                &[target("Longsword"), target("longsword")]
            ),
            vec!["Longsword".to_owned()],
            "TYPE=NonStackingCrit -- holding it twice for one weapon is still once"
        );
    }

    #[test]
    fn ignores_unrelated_choices_and_malformed_selections() {
        let selected = feats(&["Improved Critical"]);
        let choices = vec![
            choice("choice:weapon_focus_target", "weapon:longsword"),
            choice("choice:improved_critical_target", "longsword"),
            choice("choice:improved_critical_target", "weapon:"),
        ];
        assert!(improved_critical_targets_from_choices(&selected, &choices).is_empty());
    }
}

#[cfg(test)]
mod weapon_proficiency_grants_from_feats_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|key| (*key).to_owned()).collect()
    }

    fn choice(choice_set_id: &str, selection_id: &str) -> SelectedChoice {
        SelectedChoice {
            choice_set_id: choice_set_id.to_owned(),
            selection_id: selection_id.to_owned(),
        }
    }

    #[test]
    fn grants_nothing_for_a_character_with_no_proficiency_feats() {
        let grants = weapon_proficiency_grants_from_feats(&feats(&["Toughness"]), &[]);
        assert_eq!(grants, WeaponProficiencyGrantsFromFeats::default());
    }

    /// Simple Weapon Proficiency grants a TIER, so it needs no target and
    /// must not be modelled as one weapon.
    #[test]
    fn simple_weapon_proficiency_grants_the_whole_simple_tier_with_no_target() {
        let grants =
            weapon_proficiency_grants_from_feats(&feats(&["Simple Weapon Proficiency"]), &[]);
        assert!(grants.grants_simple_tier);
        assert!(
            grants.named_weapons.is_empty(),
            "a tier grant must not fabricate a named weapon: {grants:?}"
        );
    }

    #[test]
    fn martial_weapon_proficiency_grants_exactly_its_recorded_target() {
        let grants = weapon_proficiency_grants_from_feats(
            &feats(&["Martial Weapon Proficiency"]),
            &[choice("choice:martial_weapon_proficiency_target", "weapon:Longsword")],
        );
        assert!(!grants.grants_simple_tier, "the martial feat grants no tier");
        assert_eq!(grants.named_weapons, vec!["Longsword".to_owned()]);
    }

    #[test]
    fn exotic_weapon_proficiency_grants_exactly_its_recorded_target() {
        let grants = weapon_proficiency_grants_from_feats(
            &feats(&["Exotic Weapon Proficiency"]),
            &[choice("choice:exotic_weapon_proficiency_target", "weapon:Spiked Chain")],
        );
        assert_eq!(grants.named_weapons, vec!["Spiked Chain".to_owned()]);
    }

    /// No silent seeding: this one matters more than usual, because a
    /// fabricated target would erase a real -4 penalty from an attack total.
    #[test]
    fn a_proficiency_feat_with_no_recorded_target_grants_nothing() {
        let grants = weapon_proficiency_grants_from_feats(
            &feats(&["Martial Weapon Proficiency", "Exotic Weapon Proficiency"]),
            &[],
        );
        assert!(grants.named_weapons.is_empty(), "{grants:?}");
    }

    /// And the mirror: a recorded target with no feat behind it grants
    /// nothing either.
    #[test]
    fn an_orphaned_target_with_no_feat_grants_nothing() {
        let grants = weapon_proficiency_grants_from_feats(
            &[],
            &[choice("choice:martial_weapon_proficiency_target", "weapon:Longsword")],
        );
        assert!(grants.named_weapons.is_empty(), "{grants:?}");
    }

    /// `MULT:YES` on both feats: repeated picks name different weapons and
    /// all of them must be granted.
    #[test]
    fn both_feats_are_repeatable_and_their_grants_pool_together() {
        let grants = weapon_proficiency_grants_from_feats(
            &feats(&["Martial Weapon Proficiency", "Exotic Weapon Proficiency"]),
            &[
                choice("choice:martial_weapon_proficiency_target", "weapon:Longsword"),
                choice("choice:martial_weapon_proficiency_target", "weapon:Greatsword"),
                choice("choice:exotic_weapon_proficiency_target", "weapon:Whip"),
            ],
        );
        assert_eq!(
            grants.named_weapons,
            vec!["Longsword".to_owned(), "Greatsword".to_owned(), "Whip".to_owned()]
        );
    }

    /// Proficiency is boolean, so a duplicated target is still one grant.
    #[test]
    fn a_duplicated_target_grants_once() {
        let grants = weapon_proficiency_grants_from_feats(
            &feats(&["Martial Weapon Proficiency"]),
            &[
                choice("choice:martial_weapon_proficiency_target", "weapon:Longsword"),
                choice("choice:martial_weapon_proficiency_target", "weapon:longsword"),
            ],
        );
        assert_eq!(grants.named_weapons, vec!["Longsword".to_owned()]);
    }

    #[test]
    fn ignores_unrelated_choice_sets_and_malformed_selections() {
        let grants = weapon_proficiency_grants_from_feats(
            &feats(&["Martial Weapon Proficiency"]),
            &[
                choice("choice:weapon_focus_target", "weapon:Greatsword"),
                choice("choice:martial_weapon_proficiency_target", "Longsword"),
                choice("choice:martial_weapon_proficiency_target", "weapon:"),
            ],
        );
        assert!(grants.named_weapons.is_empty(), "{grants:?}");
    }

    #[test]
    fn weapon_finesse_is_recognised_only_when_held() {
        assert!(!holds_weapon_finesse(&[]));
        assert!(!holds_weapon_finesse(&feats(&["Weapon Focus"])));
        assert!(holds_weapon_finesse(&feats(&["Weapon Finesse"])));
    }

    /// Weapon Finesse takes no target, so it must not appear in the chooser
    /// table -- a picker offered for it would record a target nothing reads.
    #[test]
    fn weapon_finesse_has_no_chooser_contract() {
        assert_eq!(chooser_contract_for_feat("Weapon Finesse"), None);
    }

    /// Simple Weapon Proficiency likewise takes no target.
    #[test]
    fn simple_weapon_proficiency_has_no_chooser_contract() {
        assert_eq!(chooser_contract_for_feat("Simple Weapon Proficiency"), None);
        assert!(chooser_contract_for_feat("Martial Weapon Proficiency").is_some());
        assert!(chooser_contract_for_feat("Exotic Weapon Proficiency").is_some());
    }
}

#[cfg(test)]
mod master_craftsman_facts_from_choices_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|key| (*key).to_owned()).collect()
    }

    fn choice(choice_set_id: &str, selection_id: &str) -> SelectedChoice {
        SelectedChoice {
            choice_set_id: choice_set_id.to_owned(),
            selection_id: selection_id.to_owned(),
        }
    }

    fn target(skill: &str) -> SelectedChoice {
        choice("choice:master_craftsman_target", &format!("skill:{skill}"))
    }

    fn fact(skill: &str) -> MasterCraftsmanFact {
        MasterCraftsmanFact { skill_name: skill.to_owned(), bonus: 2 }
    }

    #[test]
    fn grounds_nothing_for_empty_inputs() {
        assert!(master_craftsman_facts_from_choices(&[], &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_when_the_feat_is_held_but_no_target_is_chosen() {
        // Same no-silent-seeding contract as every other chooser feat here: the
        // value of Master Craftsman IS which Craft/Profession skill was picked.
        let selected = feats(&["Master Craftsman"]);
        assert!(master_craftsman_facts_from_choices(&selected, &[]).is_empty());
    }

    #[test]
    fn grounds_nothing_for_an_orphan_target_without_the_feat() {
        assert!(master_craftsman_facts_from_choices(&[], &[target("Craft (armor)")]).is_empty());
    }

    #[test]
    fn grounds_the_real_flat_plus_two_for_an_explicitly_chosen_skill() {
        let selected = feats(&["Master Craftsman"]);
        assert_eq!(
            master_craftsman_facts_from_choices(&selected, &[target("Craft (armor)")]),
            vec![fact("Craft (armor)")]
        );
    }

    #[test]
    fn preserves_a_parenthesised_skill_name_verbatim() {
        // Craft and Profession skills carry parenthetical specialisations and can
        // contain spaces; they never contain colons, so the single "skill:"
        // prefix strip recovers the whole name intact.
        let selected = feats(&["Master Craftsman"]);
        assert_eq!(
            master_craftsman_facts_from_choices(&selected, &[target("Profession (siege engineer)")]),
            vec![fact("Profession (siege engineer)")]
        );
    }

    #[test]
    fn grounds_one_fact_per_distinct_skill_in_input_order() {
        // MULT:YES -- repeatable across different Craft/Profession skills.
        let selected = feats(&["Master Craftsman"]);
        assert_eq!(
            master_craftsman_facts_from_choices(
                &selected,
                &[target("Craft (armor)"), target("Profession (sailor)")]
            ),
            vec![fact("Craft (armor)"), fact("Profession (sailor)")]
        );
    }

    #[test]
    fn grounds_only_one_fact_when_the_same_skill_is_chosen_twice() {
        // STACK:NO -- never stacks on one skill, same contract as Skill Focus.
        let selected = feats(&["Master Craftsman"]);
        assert_eq!(
            master_craftsman_facts_from_choices(
                &selected,
                &[target("Craft (armor)"), target("Craft (armor)")]
            ),
            vec![fact("Craft (armor)")]
        );
    }

    #[test]
    fn treats_case_variants_of_one_skill_as_the_same_target() {
        let selected = feats(&["Master Craftsman"]);
        assert_eq!(
            master_craftsman_facts_from_choices(
                &selected,
                &[target("Craft (armor)"), target("craft (armor)")]
            ),
            vec![fact("Craft (armor)")]
        );
    }

    #[test]
    fn ignores_a_choice_from_a_different_choice_set_or_with_a_wrong_prefix() {
        let selected = feats(&["Master Craftsman"]);
        let choices = vec![
            choice("choice:skill_focus_target", "skill:Craft (armor)"),
            choice("choice:master_craftsman_target", "Craft (armor)"),
            choice("choice:master_craftsman_target", "skill:"),
        ];
        assert!(master_craftsman_facts_from_choices(&selected, &choices).is_empty());
    }
}

/// The APG/ACG passive-bonus widening (2026-07-29): the four feats whose flat
/// modifier lands on a dimension this engine computes no total for, plus the
/// two APG `BONUS:SKILL` feats folded into the standalone skill table.
#[cfg(test)]
mod apg_acg_passive_bonus_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|k| (*k).to_owned()).collect()
    }

    fn skill_names(selected: &[String]) -> Vec<&'static str> {
        standalone_skill_facts_from_feats(selected).iter().map(|f| f.skill_name).collect()
    }

    #[test]
    fn master_alchemist_grounds_its_single_craft_alchemy_skill() {
        // Corpus: BONUS:SKILL|Craft (Alchemy)|2, BENEFIT "+2 bonus on Craft
        // (alchemy) checks". A single-skill feat, unlike every CRB entry in
        // the table it joins.
        let facts = standalone_skill_facts_from_feats(&feats(&["Master Alchemist"]));
        assert_eq!(facts.len(), 1, "{facts:?}");
        assert_eq!(facts[0].skill_name, "Craft (Alchemy)");
        assert_eq!(facts[0].bonus, 2);
    }

    #[test]
    fn breadth_of_experience_grounds_both_skill_categories_it_names() {
        // Corpus token targets TYPE.Knowledge and TYPE.Profession -- whole
        // categories, not named skills -- so both are transcribed from the
        // BENEFIT prose rather than one skill being invented.
        assert_eq!(
            skill_names(&feats(&["Breadth of Experience"])),
            vec!["all Knowledge skills", "all Profession skills"]
        );
    }

    #[test]
    fn the_apg_skill_feats_do_not_disturb_the_crb_table() {
        // The CRB two-skill feats still ground exactly as before, and the two
        // tables compose rather than shadow one another.
        assert_eq!(skill_names(&feats(&["Acrobatic"])), vec!["Acrobatics", "Fly"]);
        assert_eq!(
            skill_names(&feats(&["Acrobatic", "Master Alchemist"])),
            vec!["Acrobatics", "Fly", "Craft (Alchemy)"],
            "CRB facts keep their corpus order and precede the APG ones"
        );
    }

    #[test]
    fn no_two_standalone_skill_facts_share_a_skill_name() {
        // Load-bearing: the consumer derives its explanation id by slugifying
        // skill_name, so a repeated skill would emit two records under one id.
        // This is exactly why Sharp Senses' Perception bonus is NOT in these
        // tables -- it would collide with Alertness'.
        let mut names: Vec<&str> = STANDALONE_TWO_SKILL_FACTS
            .iter()
            .chain(STANDALONE_APG_SKILL_FACTS)
            .map(|f| f.skill_name)
            .collect();
        let before = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), before, "duplicate skill name across the standalone tables");
    }

    #[test]
    fn sharp_senses_grounds_the_plus_four_result_not_its_plus_two_token() {
        // The token is BONUS:VAR|KeenSensesBonus|2, an increment to the racial
        // variable already worth 2; BENEFIT prose states the +4 result.
        assert_eq!(sharp_senses_perception_bonus_from_feats(&feats(&["Sharp Senses"])), 4);
    }

    #[test]
    fn sharp_senses_grounds_nothing_when_absent_and_never_stacks_with_itself() {
        assert_eq!(sharp_senses_perception_bonus_from_feats(&[]), 0);
        assert_eq!(sharp_senses_perception_bonus_from_feats(&feats(&["Alertness"])), 0);
        assert_eq!(
            sharp_senses_perception_bonus_from_feats(&feats(&["Sharp Senses", "Sharp Senses"])),
            4,
            "not STACK:YES/MULT:YES in the corpus"
        );
    }

    #[test]
    fn steel_soul_grounds_the_plus_four_result_not_its_plus_two_token() {
        assert_eq!(steel_soul_save_vs_spells_bonus_from_feats(&feats(&["Steel Soul"])), 4);
        assert_eq!(steel_soul_save_vs_spells_bonus_from_feats(&[]), 0);
        assert_eq!(
            steel_soul_save_vs_spells_bonus_from_feats(&feats(&["Steel Soul", "Steel Soul"])),
            4
        );
    }

    #[test]
    fn deepsight_grounds_the_sixty_foot_increment_its_token_states() {
        assert_eq!(deepsight_darkvision_bonus_from_feats(&feats(&["Deepsight"])), 60);
        assert_eq!(deepsight_darkvision_bonus_from_feats(&[]), 0);
    }

    #[test]
    fn steadfast_personality_is_absent_without_the_feat() {
        assert_eq!(steadfast_personality_will_bonus_from_feats(&[], 4, 1), None);
        assert_eq!(steadfast_personality_will_bonus_from_feats(&feats(&["Iron Will"]), 4, 1), None);
    }

    #[test]
    fn steadfast_personality_swaps_charisma_in_for_a_positive_wisdom_modifier() {
        // BONUS:SAVE|Will|CHA-WIS alone: +4 CHA replacing +1 WIS is a net +3.
        let selected = feats(&["Steadfast Personality"]);
        assert_eq!(steadfast_personality_will_bonus_from_feats(&selected, 4, 1), Some(3));
    }

    #[test]
    fn steadfast_personality_keeps_a_wisdom_penalty_and_adds_charisma_on_top() {
        // The second token (BONUS:SAVE|Will|WIS|PREVARLT:WIS,0) fires only for
        // a negative Wisdom modifier, so the penalty is KEPT rather than
        // replaced: the net delta is the full Charisma modifier. Reading only
        // the first token would give CHA - WIS = 4 - -2 = 6, overstating the
        // bonus by cancelling a penalty the prose says must still apply.
        let selected = feats(&["Steadfast Personality"]);
        assert_eq!(steadfast_personality_will_bonus_from_feats(&selected, 4, -2), Some(4));
    }

    #[test]
    fn steadfast_personality_reports_a_real_zero_rather_than_an_absence() {
        // Equal Charisma and Wisdom genuinely nets to zero; the character still
        // holds the feat, so Some(0) is the honest answer, not None.
        let selected = feats(&["Steadfast Personality"]);
        assert_eq!(steadfast_personality_will_bonus_from_feats(&selected, 2, 2), Some(0));
    }

    #[test]
    fn steadfast_personality_can_be_a_net_penalty_and_is_not_clamped() {
        // A low-Charisma, high-Wisdom character genuinely loses ground by
        // swapping; clamping at 0 would fabricate a value the corpus formula
        // does not specify.
        let selected = feats(&["Steadfast Personality"]);
        assert_eq!(steadfast_personality_will_bonus_from_feats(&selected, -1, 3), Some(-4));
    }
}

/// SD-27 (`decisions.md` §24/§28, 2026-07-31): the ARG and PU feats whose
/// unconditional corpus `BONUS:` token carries a real standing magnitude.
///
/// One test per grounded feat, each asserting the exact corpus-verified
/// formula and its absence baseline, per §24's "hand-modelled, corpus-verified,
/// one test each" ruling.
#[cfg(test)]
mod arg_and_pu_feat_effect_tests {
    use super::*;

    fn feats(keys: &[&str]) -> Vec<String> {
        keys.iter().map(|k| (*k).to_owned()).collect()
    }

    fn skill_pairs(selected: &[String]) -> Vec<(&'static str, &'static str, i16)> {
        arg_skill_facts_from_feats(selected)
            .iter()
            .map(|f| (f.feat_key, f.skill_name, f.bonus))
            .collect()
    }

    #[test]
    fn no_arg_or_pu_fact_grounds_for_a_character_holding_none_of_them() {
        let none = feats(&["Toughness", "Power Attack", "Dodge"]);
        assert!(arg_skill_facts_from_feats(&none).is_empty());
        assert!(arg_maneuver_defense_facts_from_feats(&none).is_empty());
        assert!(arg_energy_resistance_facts_from_feats(&none).is_empty());
        assert_eq!(arg_computed_climb_bonus_from_feats(&none), 0);
        assert_eq!(armor_of_the_pit_natural_armor_bonus_from_feats(&none, false), 0);
        assert_eq!(flame_heart_fire_caster_level_bonus_from_feats(&none), 0);
        assert_eq!(emotion_save_bonus_from_feats(&none), 0);
        assert_eq!(aquatic_ancestry_swim_speed_bonus_from_feats(&none), 0);
        assert_eq!(gnome_weapon_focus_attack_bonus_from_feats(&none), 0);
        assert_eq!(stamina_pool_facts_from_feats(&none, 5, 3), None);
    }

    /// Brewmaster: token says `1`, `BENEFIT:` prose says `+2`, and no
    /// prerequisite supplies the difference. The prose value is grounded and
    /// the disagreement is recorded in the table's doc comment, not hidden.
    #[test]
    fn brewmaster_grounds_the_prose_plus_two_on_both_of_its_named_skills() {
        assert_eq!(
            skill_pairs(&feats(&["Brewmaster"])),
            vec![
                ("Brewmaster", "Craft (Alchemy)", 2),
                ("Brewmaster", "Profession (Brewer)", 2),
            ]
        );
    }

    /// Sure and Fleet: `BONUS:SKILL|Acrobatics,Climb|2|TYPE=Racial`. Climb is
    /// the one computed total any ARG feat reaches, so it is returned twice --
    /// once as an auditable fact and once as the integrated value.
    #[test]
    fn sure_and_fleet_grounds_two_racial_skill_facts_and_the_computed_climb_bonus() {
        let held = feats(&["Sure and Fleet"]);
        assert_eq!(
            skill_pairs(&held),
            vec![("Sure and Fleet", "Acrobatics", 2), ("Sure and Fleet", "Climb", 2)]
        );
        assert_eq!(arg_computed_climb_bonus_from_feats(&held), 2);
    }

    /// Seen and Unseen (+2) and Angelic Flesh (-2) both name Stealth, and CRB's
    /// Stealthy already owns `feat.standalone_skill_bonus.stealth`. Three feats,
    /// one skill: exactly the collision that made this a separate record type.
    #[test]
    fn two_arg_feats_may_name_one_skill_with_opposite_signs() {
        assert_eq!(
            skill_pairs(&feats(&["Seen and Unseen", "Angelic Flesh"])),
            vec![
                ("Seen and Unseen", "Stealth", 2),
                ("Angelic Flesh", "Disguise", -2),
                ("Angelic Flesh", "Stealth", -2),
            ]
        );
    }

    #[test]
    fn scavengers_eye_grounds_its_flat_appraise_bonus() {
        assert_eq!(
            skill_pairs(&feats(&["Scavenger's Eye"])),
            vec![("Scavenger's Eye", "Appraise", 2)]
        );
    }

    /// Feline Grace's single token names five maneuvers; Tree Hanger's names
    /// one. Both are `+2` and both are CMD-only -- neither corpus token carries
    /// a `CMB_` term.
    #[test]
    fn feline_grace_and_tree_hanger_ground_cmd_only_maneuver_bonuses() {
        let feline: Vec<_> = arg_maneuver_defense_facts_from_feats(&feats(&["Feline Grace"]))
            .iter()
            .map(|f| (f.maneuver, f.cmd_bonus))
            .collect();
        assert_eq!(
            feline,
            vec![("Bull Rush", 2), ("Grapple", 2), ("Overrun", 2), ("Reposition", 2), ("Trip", 2)]
        );

        let tree = arg_maneuver_defense_facts_from_feats(&feats(&["Tree Hanger"]));
        assert_eq!(tree.len(), 1);
        assert_eq!((tree[0].maneuver, tree[0].cmd_bonus), ("Trip", 2));
    }

    /// The four `Expanded Fiendish Resistance` feats are independent: holding
    /// one must not ground another's energy type.
    #[test]
    fn each_expanded_fiendish_resistance_grounds_only_its_own_energy_type() {
        for (key, energy) in [
            ("Expanded Fiendish Resistance (Acid)", "acid"),
            ("Expanded Fiendish Resistance (Cold)", "cold"),
            ("Expanded Fiendish Resistance (Electricity)", "electricity"),
            ("Expanded Fiendish Resistance (Fire)", "fire"),
        ] {
            let grounded = arg_energy_resistance_facts_from_feats(&feats(&[key]));
            assert_eq!(grounded.len(), 1, "{key} must ground exactly one energy type");
            assert_eq!((grounded[0].energy_type, grounded[0].amount), (energy, 5), "{key}");
        }
    }

    /// Flame Heart's record carries the fire-resistance token twice. Summing
    /// them would read as resistance 10; its `BENEFIT:` prose says 5.
    #[test]
    fn flame_hearts_duplicated_corpus_token_still_grounds_resistance_five_not_ten() {
        let held = feats(&["Flame Heart"]);
        let grounded = arg_energy_resistance_facts_from_feats(&held);
        assert_eq!(grounded.len(), 1);
        assert_eq!((grounded[0].energy_type, grounded[0].amount), ("fire", 5));
        assert_eq!(flame_heart_fire_caster_level_bonus_from_feats(&held), 1);
    }

    /// Armor of the Pit's `!PREABILITY:...Scaled Skin...` is a decidable
    /// property of the character, not a situation: the `+2` applies to a
    /// tiefling who did not take Scaled Skin and is withheld from one who did,
    /// exactly as the prose's "you *instead* gain" says.
    #[test]
    fn armor_of_the_pit_grants_natural_armor_only_without_the_scaled_skin_trait() {
        let held = feats(&["Armor of the Pit"]);
        assert_eq!(armor_of_the_pit_natural_armor_bonus_from_feats(&held, false), 2);
        assert_eq!(armor_of_the_pit_natural_armor_bonus_from_feats(&held, true), 0);
    }

    /// Both emotion-save feats write the same corpus variable, so they add;
    /// the prose prints the running total through a `%1` substitution token rather
    /// than a literal, which is why neither number is read off a single record.
    #[test]
    fn the_two_emotion_save_feats_add_to_plus_two_together() {
        assert_eq!(emotion_save_bonus_from_feats(&feats(&["Fearless Curiosity"])), 1);
        assert_eq!(emotion_save_bonus_from_feats(&feats(&["Intimidating Confidence"])), 1);
        assert_eq!(
            emotion_save_bonus_from_feats(&feats(&["Fearless Curiosity", "Intimidating Confidence"])),
            2
        );
    }

    /// Neither emotion-save feat is repeatable, so a duplicated string must not
    /// inflate the sum the way a genuinely repeatable feat's would.
    #[test]
    fn a_duplicated_emotion_save_feat_does_not_stack() {
        assert_eq!(
            emotion_save_bonus_from_feats(&feats(&["Fearless Curiosity", "Fearless Curiosity"])),
            1
        );
    }

    #[test]
    fn aquatic_ancestry_and_gnome_weapon_focus_ground_their_flat_magnitudes() {
        assert_eq!(
            aquatic_ancestry_swim_speed_bonus_from_feats(&feats(&["Aquatic Ancestry"])),
            10
        );
        assert_eq!(
            gnome_weapon_focus_attack_bonus_from_feats(&feats(&["Gnome Weapon Focus"])),
            1
        );
    }

    /// Combat Stamina's pool is `BAB + CON`, verified against
    /// `BONUS:VAR|StaminaPool|BAB+CON`.
    #[test]
    fn combat_stamina_pool_is_base_attack_bonus_plus_constitution_modifier() {
        let facts = stamina_pool_facts_from_feats(&feats(&["Combat Stamina"]), 6, 2)
            .expect("Combat Stamina must ground a pool");
        assert_eq!(facts.primary, 8, "BAB +6 + CON +2");
        assert_eq!(facts.secondary, None, "no Push the Limits, no secondary pool");
        assert_eq!(facts.extra_stamina_picks, 0);
    }

    /// A negative Constitution modifier genuinely shrinks the pool; the corpus
    /// specifies no floor, so none is invented.
    #[test]
    fn a_negative_constitution_modifier_is_applied_verbatim_not_clamped() {
        let facts = stamina_pool_facts_from_feats(&feats(&["Combat Stamina"]), 1, -2)
            .expect("Combat Stamina must ground a pool");
        assert_eq!(facts.primary, -1, "BAB +1 + CON -2");
    }

    /// Extra Stamina is `STACK:YES MULT:YES` and its own
    /// `!PREABILITY:3,CATEGORY=FEAT,Extra Stamina` caps it at three picks.
    #[test]
    fn extra_stamina_adds_three_per_pick_and_stops_at_the_corpus_cap_of_three() {
        for (picks, expected_primary, expected_counted) in
            [(1usize, 9i16, 1u8), (2, 12, 2), (3, 15, 3), (4, 15, 3)]
        {
            let mut held = feats(&["Combat Stamina"]);
            held.extend(std::iter::repeat_n("Extra Stamina".to_owned(), picks));
            let facts = stamina_pool_facts_from_feats(&held, 4, 2)
                .expect("Combat Stamina must ground a pool");
            assert_eq!(
                facts.primary, expected_primary,
                "BAB +4 + CON +2 + 3*min({picks},3)"
            );
            assert_eq!(facts.extra_stamina_picks, expected_counted, "{picks} picks");
        }
    }

    /// Push the Limits' secondary pool equals the Constitution modifier and is
    /// reported separately: it is spendable only under conditions this engine
    /// does not model, so folding it into the primary total would overstate
    /// what the character can spend.
    #[test]
    fn push_the_limits_grounds_a_separate_secondary_pool_equal_to_constitution() {
        let facts =
            stamina_pool_facts_from_feats(&feats(&["Combat Stamina", "Push the Limits"]), 4, 3)
                .expect("Combat Stamina must ground a pool");
        assert_eq!(facts.primary, 7, "BAB +4 + CON +3, unchanged by the secondary pool");
        assert_eq!(facts.secondary, Some(3), "secondary pool = CON modifier");
    }

    /// Every stamina feat requires Combat Stamina
    /// (`PREABILITY:1,CATEGORY=FEAT,Combat Stamina`), so without it there is no
    /// pool at all -- `None`, never a fabricated zero.
    #[test]
    fn no_stamina_pool_grounds_without_combat_stamina() {
        assert_eq!(
            stamina_pool_facts_from_feats(&feats(&["Extra Stamina", "Push the Limits"]), 9, 4),
            None
        );
    }

    /// Every key these producers match must exist verbatim in the shipped
    /// catalog, or the wiring is keyed on a string no player can ever send.
    #[test]
    fn every_grounded_key_is_a_real_shipped_catalog_key() {
        use crate::rules_core::rules_tables::feats_all::all_feat_tables;
        let catalog: Vec<&str> = all_feat_tables()
            .iter()
            .flat_map(|t| t.entries.iter().map(|e| e.key))
            .collect();
        let grounded: Vec<&str> = ARG_SKILL_FEAT_FACTS
            .iter()
            .map(|f| f.feat_key)
            .chain(ARG_MANEUVER_DEFENSE_FACTS.iter().map(|f| f.feat_key))
            .chain(ARG_ENERGY_RESISTANCE_FACTS.iter().map(|f| f.feat_key))
            .chain(ARG_SITUATIONAL_SKILL_FACTS.iter().map(|f| f.feat_key))
            .chain(EMOTION_SAVE_FEAT_KEYS.iter().copied())
            .chain(IMPROVISATION_FEAT_KEYS.iter().copied())
            .chain(DEFIANT_LUCK_FEAT_KEYS.iter().copied())
            .chain([
                "Sure and Fleet",
                "Armor of the Pit",
                "Flame Heart",
                "Aquatic Ancestry",
                "Gnome Weapon Focus",
                "Stretched Wings",
                "Fiend Sight",
                "Combat Stamina",
                "Extra Stamina",
                "Push the Limits",
            ])
            .collect();
        for key in grounded {
            assert!(catalog.contains(&key), "{key} is not a shipped catalog key");
        }
    }

    // -----------------------------------------------------------------------
    // 2026-08-01: the eight ARG feats moved from "carries an unconditional
    // BONUS: token but moves nothing" to grounded. One test per feat, each
    // pinning the corpus-verified formula and its absence baseline.
    // -----------------------------------------------------------------------

    fn situational_triples(selected: &[String]) -> Vec<(&'static str, &'static str, i16)> {
        arg_situational_skill_facts_from_feats(selected)
            .iter()
            .map(|f| (f.feat_key, f.skill_name, f.bonus))
            .collect()
    }

    #[test]
    fn none_of_the_newly_grounded_feats_claims_anything_when_absent() {
        let none = feats(&["Toughness", "Power Attack", "Dodge"]);
        assert!(arg_situational_skill_facts_from_feats(&none).is_empty());
        assert_eq!(improvisation_untrained_skill_bonus_from_feats(&none), 0);
        assert_eq!(stretched_wings_fly_speed_bonus_from_feats(&none), 0);
        assert_eq!(defiant_luck_uses_per_day_from_feats(&none), None);
        assert_eq!(fiend_sight_darkvision_feet_from_feats(&none), 0);
    }

    /// Echoes of Stone: `BONUS:SITUATION|Perception=Underground|4` and
    /// `BONUS:SITUATION|Survival=Avoid Becoming Lost in Caverns and Rocky Areas|4`.
    /// Two tokens, two prose clauses, both `+4`.
    #[test]
    fn echoes_of_stone_grounds_both_of_its_four_point_situational_bonuses() {
        assert_eq!(
            situational_triples(&feats(&["Echoes of Stone"])),
            vec![
                ("Echoes of Stone", "Perception", 4),
                ("Echoes of Stone", "Survival", 4),
            ]
        );
    }

    /// Carrion Feeder: `BONUS:SITUATION|Survival=Find Food|2`. Its untokened
    /// save-versus-disease clause is deliberately not grounded, so exactly one
    /// fact appears -- a second would mean a number was invented from prose.
    #[test]
    fn carrion_feeder_grounds_only_its_tokened_survival_bonus() {
        assert_eq!(
            situational_triples(&feats(&["Carrion Feeder"])),
            vec![("Carrion Feeder", "Survival", 2)]
        );
    }

    /// Both feats name Survival; holding both must yield both facts rather
    /// than one collapsed or summed record. PF1 does not add two circumstance
    /// bonuses that never apply to the same check.
    #[test]
    fn two_feats_naming_survival_ground_two_distinct_facts_and_never_a_sum() {
        let held = feats(&["Echoes of Stone", "Carrion Feeder"]);
        let survival: Vec<i16> = arg_situational_skill_facts_from_feats(&held)
            .iter()
            .filter(|f| f.skill_name == "Survival")
            .map(|f| f.bonus)
            .collect();
        assert_eq!(survival, vec![4, 2]);
    }

    /// Improvisation is `+2`; Improved Improvisation writes the same
    /// `ImprovisationBonus` variable for `+4` together. Neither is `MULT:YES`,
    /// so a duplicated string must not inflate the total.
    #[test]
    fn improvisation_is_two_alone_and_four_with_its_improved_form() {
        assert_eq!(improvisation_untrained_skill_bonus_from_feats(&feats(&["Improvisation"])), 2);
        assert_eq!(
            improvisation_untrained_skill_bonus_from_feats(&feats(&[
                "Improvisation",
                "Improved Improvisation"
            ])),
            4
        );
        assert_eq!(
            improvisation_untrained_skill_bonus_from_feats(&feats(&[
                "Improvisation",
                "Improvisation"
            ])),
            2,
            "not repeatable: a duplicated string must not inflate the bonus"
        );
    }

    /// Stretched Wings: `BONUS:MOVEADD|TYPE.Fly|40`, reconciling with its
    /// prose's "increases to 60 feet" against the wing-clipped strix's corpus
    /// `MOVE:Fly,20` base that the feat's own prerequisite fixes.
    #[test]
    fn stretched_wings_grants_forty_feet_of_fly_speed() {
        assert_eq!(
            stretched_wings_fly_speed_bonus_from_feats(&feats(&["Stretched Wings"])),
            40
        );
    }

    /// Defiant Luck is 1/day; Bestow Luck adds a second use. The `DEFINE:` is
    /// on the Defiant Luck record itself, so the budget is wholly
    /// feat-determined.
    #[test]
    fn defiant_luck_is_one_use_per_day_and_two_with_bestow_luck() {
        assert_eq!(defiant_luck_uses_per_day_from_feats(&feats(&["Defiant Luck"])), Some(1));
        assert_eq!(
            defiant_luck_uses_per_day_from_feats(&feats(&["Defiant Luck", "Bestow Luck"])),
            Some(2)
        );
        assert_eq!(
            defiant_luck_uses_per_day_from_feats(&feats(&["Defiant Luck", "Defiant Luck"])),
            Some(1),
            "not repeatable: a duplicated string must not inflate the budget"
        );
    }

    /// Bestow Luck without Defiant Luck is not a legal character (its own
    /// `PREABILITY:2,...,Defiant Luck,Inexplicable Luck` requires both), and
    /// there is no ability for the extra use to attach to, so nothing is
    /// claimed at all rather than a bare `1`.
    #[test]
    fn bestow_luck_alone_claims_no_defiant_luck_budget() {
        assert_eq!(defiant_luck_uses_per_day_from_feats(&feats(&["Bestow Luck"])), None);
    }

    /// Fiend Sight states its range absolutely twice (`VISION:Darkvision (120')`
    /// and its `BENEFIT:` prose), over the 60-foot base its own
    /// `PREVISION:1,Darkvision=60` fixes. A second pick grants *see in
    /// darkness*, a capability, and does not extend the range further.
    #[test]
    fn fiend_sight_reports_a_hundred_and_twenty_foot_darkvision_at_one_pick_and_at_two() {
        assert_eq!(fiend_sight_darkvision_feet_from_feats(&feats(&["Fiend Sight"])), 120);
        assert_eq!(
            fiend_sight_darkvision_feet_from_feats(&feats(&["Fiend Sight", "Fiend Sight"])),
            120,
            "the second pick grants see in darkness, not more range"
        );
    }
}
