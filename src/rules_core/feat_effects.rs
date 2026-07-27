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
        if !feats.iter().any(|feat| feat == granted) {
            feats.push((*granted).to_owned());
        }
    }
    feats
}

/// The real PF1 Core Rulebook feat catalog's key for Toughness
/// (`rules_tables::crb::feat_data::general`'s own
/// `FeatTableEntry { key: "Toughness", ... }` record) -- NOT a synthetic
/// `"feat:toughness"` id, and verified against the real selection pipeline
/// rather than assumed: `FeatCatalogEntryDto.key` (`feat_catalog.rs`) passes
/// the catalog key through verbatim with no transformation, and the
/// frontend's `handleAddFeat`/`handleAddFeatLevelUp` (`CharacterSheet.tsx`)
/// send `entry.key` directly as `addFeatSelection`'s `featId`, which
/// `apply_add_feat_selection` (`pf1_adapter.rs`) pushes onto
/// `selected_feats` unmodified. This exact string, not the `"feat:<name>"`
/// convention the three hardcoded fixed-loadout feats use (a separate,
/// bespoke id space that never runs through the catalog at all), is what a
/// real user's `selected_feats` entry actually contains after picking
/// Toughness through the live Feat picker.
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
    if selected_feats.iter().any(|feat| feat == TOUGHNESS_FEAT_KEY) {
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
        fortitude: if selected_feats.iter().any(|feat| feat == GREAT_FORTITUDE_FEAT_KEY) {
            SAVE_FEAT_BONUS
        } else {
            0
        },
        reflex: if selected_feats.iter().any(|feat| feat == LIGHTNING_REFLEXES_FEAT_KEY) {
            SAVE_FEAT_BONUS
        } else {
            0
        },
        will: if selected_feats.iter().any(|feat| feat == IRON_WILL_FEAT_KEY) {
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
    let has = |key: &str| selected_feats.iter().any(|feat| feat == key);

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

/// Every grounded standalone skill-feat fact for the feats actually present in
/// `selected_feats`, in the stable corpus order of `STANDALONE_TWO_SKILL_FACTS`.
/// Returns an empty vec (not fabricated facts) when none of the grounding feats
/// is selected. Keyed on the exact catalog `key` string, so a longer feat whose
/// name merely begins with a grounded key (e.g. "Acrobatic Steps") never
/// matches.
pub fn standalone_skill_facts_from_feats(selected_feats: &[String]) -> Vec<StandaloneSkillFeatFact> {
    STANDALONE_TWO_SKILL_FACTS
        .iter()
        .filter(|fact| selected_feats.iter().any(|feat| feat == fact.feat_key))
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
    if !selected_feats.iter().any(|feat| feat == SKILL_FOCUS_FEAT_KEY) {
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
    if selected_feats.iter().any(|feat| feat == IMPROVED_INITIATIVE_FEAT_KEY) {
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
    if selected_feats.iter().any(|feat| feat == ENDURANCE_FEAT_KEY) {
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
    if !selected_feats.iter().any(|feat| feat == SPELL_FOCUS_FEAT_KEY) {
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

    if selected_feats.iter().any(|feat| feat == GREATER_SPELL_FOCUS_FEAT_KEY) {
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

/// Weapon Focus / Greater Weapon Focus, recognised across **both** id spaces
/// this codebase uses for them.
///
/// Unlike Skill Focus and Spell Focus, Weapon Focus already existed here before
/// this module: the fixed creation-time loadout records it in `selected_feats`
/// under the **synthetic** id `feat:weapon_focus` (never the catalog key), and
/// expresses its target as a **compound selection** inside the Fighter
/// bonus-feat choice slot (`choice:fighter_bonus_feat ->
/// feat:weapon_focus:weapon:longsword`). Recognising only the catalog key and a
/// clean new choice set would leave the real shipped Fighter loadout grounding
/// nothing -- the same "misses exactly the characters it matters most for"
/// failure as Ranger's automatic Endurance. So presence is accepted from either
/// id, and targets from either source, deduplicated by weapon.
const WEAPON_FOCUS_FEAT_KEY: &str = "Weapon Focus";
const WEAPON_FOCUS_SYNTHETIC_FEAT_ID: &str = "feat:weapon_focus";
const GREATER_WEAPON_FOCUS_FEAT_KEY: &str = "Greater Weapon Focus";
const GREATER_WEAPON_FOCUS_SYNTHETIC_FEAT_ID: &str = "feat:greater_weapon_focus";
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
/// recognising both the catalog and synthetic feat ids and both the new and
/// pre-existing target representations (see [`WEAPON_FOCUS_FEAT_KEY`]).
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
    let holds = |catalog_key: &str, synthetic_id: &str| {
        selected_feats.iter().any(|feat| feat == catalog_key || feat == synthetic_id)
    };

    let mut facts: Vec<WeaponFocusFact> = Vec::new();

    if holds(WEAPON_FOCUS_FEAT_KEY, WEAPON_FOCUS_SYNTHETIC_FEAT_ID) {
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

    if holds(GREATER_WEAPON_FOCUS_FEAT_KEY, GREATER_WEAPON_FOCUS_SYNTHETIC_FEAT_ID) {
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
        .filter(|bonus| selected_feats.iter().any(|feat| feat == bonus.feat_key))
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
    if !selected_feats.iter().any(|feat| feat == STUNNING_FIST_FEAT_KEY) {
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
    let has = |key: &str| selected_feats.iter().any(|feat| feat == key);
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
    let picks = selected_feats.iter().filter(|feat| *feat == FLEET_FEAT_KEY).count();
    i16::try_from(picks).unwrap_or(i16::MAX).saturating_mul(FLEET_BASE_SPEED_FEET)
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

    #[test]
    fn does_not_match_the_old_synthetic_feat_toughness_convention() {
        // The three hardcoded fixed-loadout feats use a "feat:<name>"
        // convention (a bespoke id space, never catalog-derived); a real
        // Toughness pick never produces that shape. Confirms this engine
        // keys on the real catalog string, not the unrelated convention.
        let selected_feats = vec!["feat:toughness".to_owned()];
        assert_eq!(hp_bonus_from_feats(&selected_feats), 0);
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
        // The whole reason for unioning both id spaces: the real shipped Fighter
        // fixture carries the SYNTHETIC feat id in selected_feats and expresses
        // its target as a compound selection inside choice:fighter_bonus_feat.
        // Keying only on the catalog key would ground nothing for it.
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
