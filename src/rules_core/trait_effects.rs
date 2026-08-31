//! Character trait/drawback selection & effects engine (AT-34-E4-002,
//! bucket M closure, `ultimate_campaign`'s `trait_content` kind --
//! corpus-wide, the same evidence shape is 1,665 of 49,438 units).
//!
//! Before this file, no character trait/drawback selection capability
//! existed anywhere in this crate: `CharacterInput` carried no field for
//! it, and the one existing PF1 "Trait" machinery in the repo,
//! `trait_pool.rs`, is a *different* mechanic entirely -- it indexes
//! `Trait.RaceTrait.<X>` records so an Adopted-Race selector can offer
//! them as options, and its own module doc states plainly "Nothing is
//! computed... this loader only indexes them." Confirmed by a whole-tree
//! grep for `selected_traits|character_traits|CharacterTrait\b` returning
//! zero matches before this cycle
//! (`AT-34-E4-002_cycle_receipt_3.md`, folded into `782584b4b3`).
//!
//! ## Scope this cycle deliberately covers, and how it was derived
//!
//! Every one of `ultimate_campaign`'s 59 `trait_content` bucket-M records'
//! real corpus JSON (`data/corpus/ultimate_campaign/trait_generic/*.json`)
//! was read directly and its `BONUS` `raw_tokens` classified by shape.
//! **31 of the 59** carry exactly one `BONUS` token, of the form
//! `SKILL|<Name>[,<Name>...]|<integer>[|TYPE=Trait]` -- a fixed named skill
//! (or comma-separated list of skills, each granted the identical flat
//! integer), with no `%LIST` player-chosen-target placeholder and no
//! ability-difference formula. Those 31 are [`FLAT_SKILL_TRAIT_BONUSES`]
//! below, transcribed directly from each record's own corpus JSON, not
//! guessed -- re-derive with:
//! ```text
//! python3 -c "import json,os; ..." # see the cycle receipt for the exact
//! filter (single BONUS token, SKILL|, no %LIST, integer third field).
//! ```
//!
//! This mirrors `feat_effects.rs`'s own first-slice precedent exactly
//! ("land one real feat, prove the pattern, then widen") -- landing the
//! single cleanest, most generalizable shape end-to-end rather than a
//! half-wired attempt at all 59.
//!
//! ## Second slice: fixed-choice `BONUS:SKILL|%LIST` traits
//!
//! A second cycle widened the spine to cover the `%LIST` player-chosen-
//! target shape -- but only where the corpus's own `CHOOSE:SKILL` token
//! enumerates a **fixed, closed list of concrete named skills** (e.g.
//! `trait_criminal`: `CHOOSE:SKILL|Disable Device|Intimidate|Sleight of
//! Hand`). The character's choice is recorded the same general way
//! `archetype_resolver.rs` already records an archetype pick -- a
//! [`SelectedChoice`] with `choice_set_id` = [`trait_skill_choice_id`] of
//! the trait's own `trait_id`, and `selection_id` = the chosen `skill:`
//! wire id -- not a new parallel selection mechanism. See
//! [`SKILL_CHOICE_TRAIT_BONUSES`] for the exact 5-record table and
//! [`skill_choice_bonuses_from_traits`] for the compute path. An untrusted
//! or stale `selection_id` outside the trait's own `skill_options` is
//! never honored (same "omit rather than fabricate" posture as the flat
//! table).
//!
//! ## Third slice: open-subtype-family `BONUS:SKILL|%LIST` traits
//!
//! **Correction of the prior cycle's own doc comment** (retro-logged): the
//! prior cycle's receipt characterized the 4 remaining `%LIST` traits
//! whose `CHOOSE:SKILL` names a `TYPE=<Family>` subtype family
//! (`TYPE=Craft`, `TYPE=Perform`, `TYPE=Profession`) as needing "a genuinely
//! open-ended text-entry chooser... a materially different UI/input shape"
//! than the closed-list case, and named it out of scope. That is not
//! actually true of this app: `skill_allocation.rs` already carries a
//! closed, corpus-derived enumeration of every `Craft`/`Perform`/
//! `Profession` subtype this crate recognizes (`CRAFT_SKILL_IDS` (23),
//! `PERFORM_SKILL_IDS` (9), `PROFESSION_SKILL_IDS` (31), transcribed from
//! `data/corpus/core_rulebook/skill/*.json`) and already treats
//! `TYPE=<Family>` as exactly that closed universe for its own
//! class-skill-wildcard expansion ([`skill_allocation::skill_family_
//! member_ids`]). A second, independent open-text chooser for the same
//! three families would silently disagree with what this crate itself
//! already considers a legal skill. So the union of the relevant
//! families' member ids IS this crate's own closed list for these traits
//! too -- the identical [`SelectedChoice`]-backed closed-list mechanism
//! [`SKILL_CHOICE_TRAIT_BONUSES`] already established, just with the
//! option list computed from [`skill_allocation::skill_family_member_
//! ids`] instead of hand-transcribed per trait. See
//! [`FAMILY_CHOICE_TRAIT_BONUSES`] for the 4-record table and
//! [`family_choice_bonuses_from_traits`] for the compute path.
//!
//! ## Fourth slice: flat `BONUS:SAVE` traits
//!
//! A fourth cycle widened the spine past `BONUS:SKILL` entirely, into a
//! **different pillar** -- total saving throws. `ultimate_campaign`'s two
//! `BONUS:SAVE` trait records (`trait_life_of_toil`: `SAVE|Fortitude|1`,
//! `trait_indomitable_faith`: `SAVE|Will|1`) are both a single flat token
//! with no `%LIST` choice, the exact shape `feat_effects::save_bonuses_
//! from_feats` already grounds for Great Fortitude/Iron Will/Lightning
//! Reflexes -- reusing that same real consumer,
//! `pilot_compute::compute_total_saves`, rather than inventing a second
//! save-bonus pathway. See [`SAVE_TRAIT_BONUSES`] for the 2-record table
//! and [`save_bonuses_from_traits`] for the compute path; the classifier-
//! facing grounding check is [`save_trait_magnitude_is_grounded_for_
//! corpus_key`].
//!
//! ## What this module deliberately does NOT cover
//!
//! - **3 records** carry an ability-score-difference formula magnitude
//!   (`max(INT,CHA)-CHA` etc, e.g. `trait_bruising_intellect`) -- no
//!   formula evaluator exists in this crate for that shape.
//! - **The remaining 13 `trait_content` records** mix `BONUS:VAR`,
//!   `BONUS:SITUATION`, `BONUS:ABILITYPOOL`, `BONUS:COMBAT`, and
//!   `BONUS:CONCENTRATION` tokens -- different pillars entirely (combat
//!   maneuvers, concentration checks, a bonus trait-slot pool, and
//!   engine-internal variables with no consuming pillar), out of this
//!   module's scope.
//! - **All 30 `ability_content` units** (`ultimate_campaign`'s
//!   Drawback/Retraining sub-mechanics) -- house-rule bookkeeping and
//!   GM-adjudicated narrative penalties with no clean formulaic trigger,
//!   per the prior cycle's own direct reading of that corpus.
//!
//! Widening past these four slices is future work, gated on either a
//! formula evaluator for ability-score-difference magnitudes, or genuinely
//! separate per-pillar compute paths for the mixed `BONUS:VAR/SITUATION/
//! ABILITYPOOL/COMBAT/CONCENTRATION` records -- neither exists yet, and
//! building either as a rushed half-measure here would risk the same "8
//! closures where measurement found 1" failure this bundle's own doctrine
//! warns against.

use std::collections::BTreeMap;

use crate::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};

/// One flat, named-skill `BONUS:SKILL` trait -- see the module doc
/// comment's "Scope this cycle deliberately covers" section for exactly
/// which shape qualifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraitSkillBonus {
    /// The wire id `CharacterInput.chosen.selected_traits` carries for
    /// this trait -- `"trait:" + the corpus filename slug` (e.g.
    /// `data/corpus/ultimate_campaign/trait_generic/trait_acrobat.json`
    /// -> `"trait:trait_acrobat"`), the same flat compound-string idiom
    /// `selected_feats`' `"feat:weapon_focus"` uses.
    pub trait_id: &'static str,
    /// The record's own corpus `KEY` token (e.g. `"Trait ~ Acrobat"`) --
    /// what `v06_work_inventory.rs`'s classifier carries as `unit.key`,
    /// used by [`flat_skill_trait_magnitude_is_grounded_for_corpus_key`]
    /// to look this record up without re-deriving the wire id.
    pub corpus_key: &'static str,
    /// The trait's display name, transcribed from the corpus record's own
    /// `name` field.
    pub name: &'static str,
    /// The `skill:` wire id(s) this trait's flat bonus applies to --
    /// normalized the same way `skill_allocation.rs`'s own
    /// `normalize_skill_display_name` would (lowercase, parens stripped,
    /// separators collapsed to `_`), transcribed by hand from the
    /// record's own `BONUS:SKILL|<Name>[,<Name>]|...` token, not
    /// re-derived at runtime.
    pub skills: &'static [&'static str],
    /// The flat integer bonus, applied identically to every skill in
    /// `skills` (PF1 "trait bonus" -- most are `+1`; `trait_principled`'s
    /// `-2` Bluff penalty is a real, corpus-cited negative trait bonus,
    /// not a sign error).
    pub bonus: i8,
    /// The trait's own corpus `description` field, verbatim -- what a
    /// desktop trait picker shows the player, sourced from the same
    /// record this bonus is transcribed from (never invented UI copy).
    pub description: &'static str,
}

/// The 31-of-59 `ultimate_campaign` `trait_content` records whose corpus
/// `BONUS` token is a flat, named-skill `SKILL` bonus. See the module doc
/// comment for the exact filter and what is deliberately excluded.
pub static FLAT_SKILL_TRAIT_BONUSES: &[TraitSkillBonus] = &[
    TraitSkillBonus { trait_id: "trait:trait_acrobat", corpus_key: "Trait ~ Acrobat", name: "Acrobat", skills: &["skill:acrobatics"], bonus: 1, description: "Having trained from a young age, you're capable of amazing feats of daring. You gain a +1 bonus on Acrobatics checks, and you take only a -2 penalty instead of the normal -5 penalty when using the Climb skill to attempt an accelerated climb." },
    TraitSkillBonus { trait_id: "trait:trait_bastard", corpus_key: "Trait ~ Bastard", name: "Bastard", skills: &["skill:sense_motive"], bonus: 1, description: "You were born out of wedlock. You have always been an outsider in society, and in your own family. This perspective has sharpened your insight. You gain a +1 trait bonus on Sense Motive checks, and Sense Motive is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_beast_bond", corpus_key: "Trait ~ Beast Bond", name: "Beast Bond", skills: &["skill:handle_animal", "skill:ride"], bonus: 1, description: "You share a close bond with animals. You gain a +1 bonus on Handle Animal checks and Ride checks. One of these skills (your choice) is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_brewmaster", corpus_key: "Trait ~ Brewmaster", name: "Brewmaster", skills: &["skill:profession_brewer", "skill:craft_alchemy"], bonus: 1, description: "Your family brought the secrets of dwarven brewing to non-dwarven lands. Though this has given you skill in the brewer's craft, it's also earned you distrust among dwarven purists. You gain a +1 trait bonus on Profession (brewer) and Craft (alchemy) checks, but you take a -1 penalty on Diplomacy checks made to change the attitude of dwarves who know that your family has shared brewing secrets with non-dwarves." },
    TraitSkillBonus { trait_id: "trait:trait_caretaker", corpus_key: "Trait ~ Caretaker", name: "Caretaker", skills: &["skill:heal"], bonus: 1, description: "As the child of an herbalist or an assistant in a temple infirmary, you often had to assist in tending to the sick and wounded. You gain a +1 trait bonus on Heal checks, and Heal is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_child_of_the_temple", corpus_key: "Trait ~ Child of the Temple", name: "Child of the Temple", skills: &["skill:knowledge_nobility", "skill:knowledge_religion"], bonus: 1, description: "You have long served at a temple in a city, where you picked up on many of the nobility's customs in addition to spending much time in the temple libraries studying your faith. You gain a +1 trait bonus on Knowledge (nobility) and Knowledge (religion) checks, and one of these skills (your choice) is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_civilized", corpus_key: "Trait ~ Civilized", name: "Civilized", skills: &["skill:knowledge_nobility", "skill:knowledge_local"], bonus: 1, description: "You are well versed in the local laws, customs, and politics. You gain a +1 trait bonus on Knowledge (nobility) checks and Knowledge (local) checks. Knowledge (local) is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_devotee_of_the_green", corpus_key: "Trait ~ Devotee of the Green", name: "Devotee of the Green", skills: &["skill:knowledge_geography", "skill:knowledge_nature"], bonus: 1, description: "Your faith in the natural world or one of the gods of nature makes it easy for you to pick up on related concepts. You gain a +1 trait bonus on Knowledge (geography) and Knowledge (nature) checks, and one of these skills (your choice) is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_ease_of_faith", corpus_key: "Trait ~ Ease of Faith", name: "Ease of Faith", skills: &["skill:diplomacy"], bonus: 1, description: "Your mentor, the person who invested your faith in you from an early age, took steps to ensure you understood that what powers your divine magic is no different from that which powers the magic of other religions. This philosophy makes it easier for you to interact with others who may not share your views. You gain a +1 bonus on Diplomacy checks, and Diplomacy is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_friend_in_every_town", corpus_key: "Trait ~ Friend in Every Town", name: "Friend in Every Town", skills: &["skill:knowledge_local", "skill:diplomacy"], bonus: 1, description: "You have no problem making friends and learning information from them wherever you go. You gain a +1 trait bonus on all Knowledge (local) checks and Diplomacy checks. One of these skills (your choice) is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_frontier_forged", corpus_key: "Trait ~ Frontier-Forged", name: "Frontier-Forged (Any Frontier Area)", skills: &["skill:perception"], bonus: 1, description: "A hard life on the edge of civilization has made you resourceful but has also given you a streak of self-preservation bordering on paranoia. You receive a +1 trait bonus on Perception checks and a +1 bonus on Survival checks made to get along in the wild." },
    TraitSkillBonus { trait_id: "trait:trait_imposing_scion", corpus_key: "Trait ~ Imposing Scion", name: "Imposing Scion", skills: &["skill:intimidate"], bonus: 1, description: "The reputation of your bloodline grants you a degree of fearful respect. You gain a +1 trait bonus on Intimidate checks. If your bloodline is keyed to a specific creature type, your trait bonus increases to +2 when interacting with creatures of that type." },
    TraitSkillBonus { trait_id: "trait:trait_nature_s_mimic", corpus_key: "Trait ~ Nature's Mimic", name: "Nature's Mimic", skills: &["skill:knowledge_nature"], bonus: 1, description: "Your knowledge of nature informs your fighting style, and that style gives you insight into related aspects of the natural world. You gain a +1 trait bonus on all Knowledge (nature) checks, and Knowledge (nature) is always a class skill for you. You can make Knowledge (nature) checks pertaining to animals that correspond to your style feats untrained." },
    TraitSkillBonus { trait_id: "trait:trait_omen", corpus_key: "Trait ~ Omen", name: "Omen", skills: &["skill:intimidate"], bonus: 1, description: "You are the harbinger of some future event. Whether this event bodes good or ill, you exude an ominous presence." },
    TraitSkillBonus { trait_id: "trait:trait_orphaned", corpus_key: "Trait ~ Orphaned", name: "Orphaned", skills: &["skill:survival"], bonus: 1, description: "You grew up separated from your birth parents, and had to learn to watch out for yourself. You gain a +1 trait bonus on Survival checks, and Survival is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_outcast_s_intuition", corpus_key: "Trait ~ Outcast's Intuition", name: "Outcast's Intuition", skills: &["skill:sense_motive"], bonus: 1, description: "You are able to sense the motives of others and use that sense to bolster your magic against dispelling. You gain a +1 trait bonus on Sense Motive checks, and Sense Motive is always a class skill for you. In addition, your caster level is treated as 1 level higher for the purposes of any attempts to dispel your magic." },
    TraitSkillBonus { trait_id: "trait:trait_perfectionist_s_brew", corpus_key: "Trait ~ Perfectionist's Brew", name: "Perfectionist's Brew", skills: &["skill:craft_alchemy"], bonus: 2, description: "You know that potion recipes should be followed with exact precision." },
    TraitSkillBonus { trait_id: "trait:trait_principled", corpus_key: "Trait ~ Principled", name: "Principled", skills: &["skill:bluff"], bonus: -2, description: "You hold yourself to a strict code of behavior that guides all of your decisions and actions. You take a -2 penalty on Bluff checks and gain a +2 trait bonus on saving throws against charm, compulsion, and emotion effects." },
    TraitSkillBonus { trait_id: "trait:trait_reckless", corpus_key: "Trait ~ Reckless", name: "Reckless", skills: &["skill:acrobatics"], bonus: 1, description: "You have a tendency for rash behavior, often disregarding your own safety as you move across the battlefield. You gain a +1 bonus on Acrobatics checks, and Acrobatics is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_reluctant_apprentice", corpus_key: "Trait ~ Reluctant Apprentice", name: "Reluctant Apprentice", skills: &["skill:knowledge_arcana"], bonus: 1, description: "[Untrained view Not Implemented] Your early training grants you knowledge of the arcane. You gain a +1 trait bonus on Knowledge (arcana) checks, and are considered trained in that skill even if you have no ranks in it." },
    TraitSkillBonus { trait_id: "trait:trait_savage", corpus_key: "Trait ~ Savage", name: "Savage", skills: &["skill:knowledge_nature"], bonus: 1, description: "You were born and raised in untamed lands far from civilization. You learned to survive in the elements among brutal humanoids and beasts. You gain a +1 trait bonus on Knowledge (nature) checks and a +1 trait bonus on Survival checks to get along in the wild. Knowledge (nature) is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_scholar_of_the_great_beyond", corpus_key: "Trait ~ Scholar of the Great Beyond", name: "Scholar of the Great Beyond", skills: &["skill:knowledge_history", "skill:knowledge_planes"], bonus: 1, description: "Your greatest interests as a child did not lie with current events or the mundane- you have always felt out of place, as if you were born in the wrong era. You take to philosophical discussions of the Great Beyond and of historical events with ease. You gain a +1 trait bonus on Knowledge (history) and Knowledge (planes) checks, and one of these skills (your choice) is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_sea_souled", corpus_key: "Trait ~ Sea-Souled", name: "Sea-Souled (Coastline or Island)", skills: &["skill:swim"], bonus: 1, description: "You are at home at sea. You receive a +1 trait bonus on Swim checks, and you can always take 10 while Swimming." },
    TraitSkillBonus { trait_id: "trait:trait_seeker", corpus_key: "Trait ~ Seeker", name: "Seeker", skills: &["skill:perception"], bonus: 1, description: "You are always on the lookout for reward and danger. You gain a +1 trait bonus on Perception checks, and Perception is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_truth_s_agent", corpus_key: "Trait ~ Truth's Agent", name: "Truth's Agent", skills: &["skill:knowledge_local"], bonus: 1, description: "You are skilled at weeding out information. You gain a +1 trait bonus on all Diplomacy checks made to gather information and all Knowledge (local) checks. Knowledge (local) is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_twinned_presence", corpus_key: "Trait ~ Twinned Presence", name: "Twinned Presence", skills: &["skill:intimidate"], bonus: 1, description: "Your eidolon-and your connection to it-makes others ill at ease. You gain a +1 trait bonus on Intimidate checks, and Intimidate is always a class skill for you. In addition, if your eidolon is summoned and within 30 feet, and its size exceeds your own, use its size modifier on any Intimidate checks you make." },
    TraitSkillBonus { trait_id: "trait:trait_unblemished_barrel", corpus_key: "Trait ~ Unblemished Barrel", name: "Unblemished Barrel", skills: &["skill:craft_alchemy", "skill:craft_weapons"], bonus: 1, description: "Your skill with firearms speeds your repair of such weapons. You gain a +1 trait bonus on Craft (alchemy) and Craft (weapons) checks, and it takes you only 30 minutes to remove the broken condition from a firearm." },
    TraitSkillBonus { trait_id: "trait:trait_unintentional_linguist", corpus_key: "Trait ~ Unintentional Linguist", name: "Unintentional Linguist", skills: &["skill:linguistics"], bonus: 1, description: "You can speak with outsiders. You gain a +1 trait bonus on all Linguistics checks, and you begin play knowing one of the following languages (in addition to those granted by your race and Intelligence modifier): Abyssal, Aquan, Celestial, Ignan, Infernal, Protean, or Terran." },
    TraitSkillBonus { trait_id: "trait:trait_unpredictable", corpus_key: "Trait ~ Unpredictable", name: "Unpredictable", skills: &["skill:bluff"], bonus: 1, description: "Your actions often seem random and chaotic to others, but there is a method to your madness. You gain a +1 trait bonus on Bluff checks, and Bluff is always a class skill for you." },
    TraitSkillBonus { trait_id: "trait:trait_vigilant_battler", corpus_key: "Trait ~ Vigilant Battler", name: "Vigilant Battler", skills: &["skill:sense_motive"], bonus: 1, description: "You are hard to fool and harder to trick with a feint. You gain a +1 trait bonus on all Sense Motive checks. This trait bonus increases to +2 when used to counter a feint in combat." },
    TraitSkillBonus { trait_id: "trait:trait_weathered_emissary", corpus_key: "Trait ~ Weathered Emissary", name: "Weathered Emissary", skills: &["skill:linguistics"], bonus: 1, description: "Your travels in the wildlands gave you insight into how to survive their dangers and communicate with their inhabitants. You gain a +1 trait bonus on all Linguistics and Survival checks, and Linguistics is always a class skill for you." },
];

/// Looks up one [`FLAT_SKILL_TRAIT_BONUSES`] entry by its wire `trait_id`.
fn find_by_trait_id(trait_id: &str) -> Option<&'static TraitSkillBonus> {
    FLAT_SKILL_TRAIT_BONUSES.iter().find(|entry| entry.trait_id == trait_id)
}

/// The real, computed skill bonus contribution of every trait in
/// `selected_traits` that this module recognizes (one of
/// [`FLAT_SKILL_TRAIT_BONUSES`]) -- keyed by `skill:` wire id, summed
/// across every selected trait that targets that skill (a character who
/// somehow selected two traits granting the same skill gets both; PF1
/// core rules do not forbid two traits sharing a skill target). A trait
/// id this module does not recognize (outside the 31-record flat slice,
/// or simply not a real trait id) contributes nothing -- never a
/// fabricated bonus, the same "omit rather than fabricate" discipline
/// `skill_allocation.rs`'s own bounded posture already follows.
pub fn skill_bonuses_from_traits(selected_traits: &[String]) -> BTreeMap<String, i8> {
    let mut totals: BTreeMap<String, i8> = BTreeMap::new();
    for trait_id in selected_traits {
        let Some(entry) = find_by_trait_id(trait_id) else {
            continue;
        };
        for skill_id in entry.skills {
            let slot = totals.entry((*skill_id).to_owned()).or_insert(0);
            *slot = slot.saturating_add(entry.bonus);
        }
    }
    totals
}

/// **AT-34-E4-002's classifier-facing entry point**, the same shape
/// `skill_allocation::skill_bonus_is_grounded_for_display_name`
/// established for `AT-34-E3-003`'s bucket-M skill closure: takes a
/// corpus trait record's own `KEY` token (exactly what
/// `v06_work_inventory.rs`'s `Kind::Trait` classifier carries as
/// `unit.key`), ACTUALLY BUILDS a minimal fixture character who selected
/// exactly that trait with the target skill allocated at 1 rank, runs it
/// through the real [`crate::rules_core::skill_allocation::
/// allocate_skill_ranks`] engine, and returns the genuine, computed
/// `misc_modifier` this module's engine produces for that skill -- never
/// an assumption that it "should" equal the transcribed `bonus`, an
/// executed check that it DOES. Returns `None` for any corpus key outside
/// the 31-record flat slice (nothing to fixture-check), or in the
/// unreachable case the fixture-executed value ever disagreed with the
/// transcribed table (a real defect, not silently promoted).
pub fn flat_skill_trait_magnitude_is_grounded_for_corpus_key(corpus_key: &str) -> Option<i8> {
    let entry = FLAT_SKILL_TRAIT_BONUSES
        .iter()
        .find(|entry| entry.corpus_key == corpus_key)?;
    let target_skill = entry.skills.first()?;

    let input = CharacterInput {
        case_id: None,
        source_package_id: "at_34_e4_002_fixture".to_owned(),
        chosen: ChosenCharacterState {
            race_id: "race:human".to_owned(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:fighter".to_owned(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: vec![crate::rules_core::character_input::SkillAllocation {
                skill_id: (*target_skill).to_owned(),
                ranks: 1,
            }],
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            selected_traits: vec![entry.trait_id.to_owned()],
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    };

    let totals = crate::rules_core::skill_allocation::allocate_skill_ranks(&input);
    let computed = totals.totals.get(*target_skill).map(|total| total.misc_modifier)?;

    if computed == entry.bonus {
        Some(computed)
    } else {
        // The engine genuinely disagreed with the transcribed table --
        // this is a real defect (a transcription error or an engine
        // regression), never something to paper over by returning the
        // table's own value instead of the fixture's.
        None
    }
}

/// One fixed-choice `BONUS:SKILL|%LIST` trait -- see the module doc
/// comment's "Second slice" section for the exact filter and what stays
/// out of scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraitSkillChoiceBonus {
    /// The wire id `CharacterInput.chosen.selected_traits` carries for
    /// this trait -- same `"trait:" + corpus filename slug` idiom as
    /// [`TraitSkillBonus::trait_id`].
    pub trait_id: &'static str,
    /// The record's own corpus `KEY` token, as transcribed from
    /// `data.key`.
    pub corpus_key: &'static str,
    /// The trait's display name, transcribed from the corpus record's own
    /// `name` field.
    pub name: &'static str,
    /// Every `skill:` wire id the corpus's own `CHOOSE:SKILL` token
    /// enumerates, normalized the same way `skill_allocation.rs`'s own
    /// `normalize_skill_display_name` would. A character's recorded
    /// choice outside this list is never honored -- see
    /// [`skill_choice_bonuses_from_traits`].
    pub skill_options: &'static [&'static str],
    /// The flat integer bonus applied to whichever `skill_options` entry
    /// the character actually chose. Transcribed from the record's own
    /// `BONUS:SKILL|%LIST|<n>|...` token (or, where the corpus token
    /// omits the magnitude entirely, from that same record's own
    /// description text, which states it in prose -- `trait_harvester`
    /// and `trait_simple_disciple` both do this; never invented).
    pub bonus: i8,
    /// The trait's own corpus `description` field, verbatim.
    pub description: &'static str,
}

/// The 5-of-59 `ultimate_campaign` `trait_content` records whose corpus
/// `BONUS` token is a `%LIST` player choice constrained to a fixed,
/// closed list of concrete named skills (never an open `TYPE=<Family>`
/// subtype chooser -- see the module doc comment for what that excludes).
pub static SKILL_CHOICE_TRAIT_BONUSES: &[TraitSkillChoiceBonus] = &[
    TraitSkillChoiceBonus {
        trait_id: "trait:trait_criminal",
        corpus_key: "Trait ~ Criminal",
        name: "Criminal",
        skill_options: &["skill:disable_device", "skill:intimidate", "skill:sleight_of_hand"],
        bonus: 1,
        description: "You spent your early life robbing and stealing to get by. Select one of the following skills: Disable Device, Intimidate, or Sleight of Hand. You gain a +1 trait bonus on that skill, and it is always a class skill for you.",
    },
    TraitSkillChoiceBonus {
        trait_id: "trait:trait_fiend_blood",
        corpus_key: "Trait ~ Fiend Blood",
        name: "Fiend Blood",
        skill_options: &["skill:bluff", "skill:intimidate", "skill:knowledge_planes"],
        bonus: 1,
        description: "The blood of fiends taints your line, manifesting physically, though it may be barely noticeable. Choose one of the following skills: Bluff, Intimidate, or Knowledge (planes). You gain a +1 trait bonus on checks with that skill, and it is always a class skill for you.",
    },
    TraitSkillChoiceBonus {
        trait_id: "trait:trait_harvester",
        corpus_key: "Trait ~ Harvester",
        name: "Harvester",
        skill_options: &["skill:profession_tanner", "skill:profession_trapper"],
        bonus: 1,
        description: "You were trained to harvest all parts of an animal with care and precision. You gain a +1 trait bonus on Profession (tanner) or Profession (trapper) checks, and you may make these checks as if you were trained in the skill even if you have no ranks. Additionally, you do not risk poisoning yourself whenever you handle or apply poison taken from a venomous creature.",
    },
    TraitSkillChoiceBonus {
        trait_id: "trait:trait_influence",
        corpus_key: "Trait ~ Influence",
        name: "Influence",
        skill_options: &["skill:diplomacy", "skill:intimidate", "skill:sense_motive"],
        bonus: 1,
        description: "Your position in society grants you special insight into others, and special consideration or outright awe from others.Choose one of the following skills: Diplomacy, Intimidate, or Sense Motive. You gain a +1 trait bonus on that skill, and it is always a class skill for you.",
    },
    TraitSkillChoiceBonus {
        trait_id: "trait:trait_style_sage",
        corpus_key: "Trait ~ Style Sage",
        name: "Style Sage",
        skill_options: &["skill:knowledge_history", "skill:knowledge_local"],
        bonus: 1,
        description: "You have a passion for history and news concerning monastic disciplines. You gain a +1 trait bonus on checks with your choice of either Knowledge (local) or Knowledge (history), and the one you choose is always a class skill for you. In addition, you gain a +1 trait bonus on Diplomacy checks made to gather information about any person with levels in monk.",
    },
];

/// Looks up one [`SKILL_CHOICE_TRAIT_BONUSES`] entry by its wire
/// `trait_id`.
fn find_choice_by_trait_id(trait_id: &str) -> Option<&'static TraitSkillChoiceBonus> {
    SKILL_CHOICE_TRAIT_BONUSES.iter().find(|entry| entry.trait_id == trait_id)
}

/// The `choice_set_id` a character's chosen skill for one
/// [`SKILL_CHOICE_TRAIT_BONUSES`] trait is recorded under -- one
/// `SelectedChoice` per trait, the same "one choice-set id per
/// independent decision" convention `archetype_resolver::ARCHETYPE_
/// CHOICE_ID` already establishes, scoped per-trait (not a single shared
/// id) so a character who somehow selected two `%LIST` traits records
/// each one's choice independently rather than colliding on one slot.
pub fn trait_skill_choice_id(trait_id: &str) -> String {
    format!("trait_choice:{trait_id}")
}

/// The real, computed skill bonus contribution of every
/// [`SKILL_CHOICE_TRAIT_BONUSES`] trait in `selected_traits` **that also
/// carries a genuine, corpus-legal recorded choice** in
/// `selected_choices` -- a trait selected without its matching choice
/// recorded yet (character-creation-in-progress) or with a `selection_id`
/// outside that trait's own `skill_options` (untrusted/stale data)
/// contributes nothing, never a fabricated or first-guessed default.
/// Mirrors [`skill_bonuses_from_traits`]'s "omit rather than fabricate"
/// discipline exactly; the two are summed together by
/// `skill_allocation::allocate_skill_ranks`, never double-applied against
/// each other because a trait id can only ever appear in one of the two
/// tables (enforced by [`no_trait_id_appears_in_both_tables`]).
pub fn skill_choice_bonuses_from_traits(
    selected_traits: &[String],
    selected_choices: &[SelectedChoice],
) -> BTreeMap<String, i8> {
    let mut totals: BTreeMap<String, i8> = BTreeMap::new();
    for trait_id in selected_traits {
        let Some(entry) = find_choice_by_trait_id(trait_id) else {
            continue;
        };
        let choice_set_id = trait_skill_choice_id(trait_id);
        let Some(chosen) = selected_choices
            .iter()
            .find(|choice| choice.choice_set_id == choice_set_id)
        else {
            continue;
        };
        if !entry.skill_options.contains(&chosen.selection_id.as_str()) {
            // An untrusted or stale selection outside this trait's own
            // corpus-declared options -- never honored, same discipline
            // `archetype_resolver::chooser_option_selected` already
            // established for a pool choice.
            continue;
        }
        let slot = totals.entry(chosen.selection_id.clone()).or_insert(0);
        *slot = slot.saturating_add(entry.bonus);
    }
    totals
}

/// **AT-34-E4-002's classifier-facing entry point for the choice-based
/// slice**, the same shape as [`flat_skill_trait_magnitude_is_grounded_
/// for_corpus_key`]: takes a corpus trait record's own `KEY` token, ACTUALLY
/// BUILDS a minimal fixture character who selected exactly that trait
/// *and* recorded a choice for its first-listed `skill_options` entry,
/// runs it through the real [`crate::rules_core::skill_allocation::
/// allocate_skill_ranks`] engine, and returns the genuine, computed
/// `misc_modifier` -- never an assumed value. Returns `None` for any
/// corpus key outside the 5-record choice slice, or in the unreachable
/// case the fixture-executed value ever disagreed with the transcribed
/// table.
pub fn skill_choice_trait_magnitude_is_grounded_for_corpus_key(corpus_key: &str) -> Option<i8> {
    let entry = SKILL_CHOICE_TRAIT_BONUSES
        .iter()
        .find(|entry| entry.corpus_key == corpus_key)?;
    let chosen_skill = entry.skill_options.first()?;

    let input = CharacterInput {
        case_id: None,
        source_package_id: "at_34_e4_002_fixture".to_owned(),
        chosen: ChosenCharacterState {
            race_id: "race:human".to_owned(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:fighter".to_owned(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: vec![crate::rules_core::character_input::SkillAllocation {
                skill_id: (*chosen_skill).to_owned(),
                ranks: 1,
            }],
            equipment_selections: Vec::new(),
            selected_choices: vec![SelectedChoice {
                choice_set_id: trait_skill_choice_id(entry.trait_id),
                selection_id: (*chosen_skill).to_owned(),
            }],
            selected_traits: vec![entry.trait_id.to_owned()],
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    };

    let totals = crate::rules_core::skill_allocation::allocate_skill_ranks(&input);
    let computed = totals.totals.get(*chosen_skill).map(|total| total.misc_modifier)?;

    if computed == entry.bonus {
        Some(computed)
    } else {
        None
    }
}

/// One open-subtype-family `BONUS:SKILL|%LIST` trait -- see the module
/// doc comment's "Third slice" section for the exact filter and the
/// correction of the prior cycle's own "text entry" characterization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraitSkillFamilyChoiceBonus {
    /// The wire id `CharacterInput.chosen.selected_traits` carries for
    /// this trait -- same `"trait:" + corpus filename slug` idiom as
    /// [`TraitSkillBonus::trait_id`].
    pub trait_id: &'static str,
    /// The record's own corpus `KEY` token, as transcribed from
    /// `data.key`.
    pub corpus_key: &'static str,
    /// The trait's display name, transcribed from the corpus record's own
    /// `name` field.
    pub name: &'static str,
    /// Every `TYPE=<Family>` name the corpus's own `CHOOSE:SKILL` token
    /// enumerates (e.g. `&["Craft"]`, or `&["Craft", "Perform",
    /// "Profession"]` for `trait_mentored`'s three-family choice) -- each
    /// resolved at call time via [`skill_allocation::skill_family_member_
    /// ids`], never hand-duplicated here.
    pub skill_families: &'static [&'static str],
    /// The flat integer bonus applied to whichever resolved skill id the
    /// character actually chose. Transcribed from the record's own
    /// `BONUS:SKILL|%LIST|<n>|...` token (or, where the token omits the
    /// magnitude, from that record's own description text -- never
    /// invented).
    pub bonus: i8,
    /// The trait's own corpus `description` field, verbatim.
    pub description: &'static str,
}

/// The 4-of-59 `ultimate_campaign` `trait_content` records whose corpus
/// `BONUS` token is a `%LIST` player choice constrained to an open
/// `TYPE=<Family>` subtype family rather than a fixed list of concrete
/// named skills. Every family named here already has a closed,
/// corpus-derived member roster in `skill_allocation.rs`
/// (`CRAFT_SKILL_IDS`/`PERFORM_SKILL_IDS`/`PROFESSION_SKILL_IDS`), so this
/// is a closed-list choice, not an open text field -- see the module doc
/// comment's "Third slice" section.
pub static FAMILY_CHOICE_TRAIT_BONUSES: &[TraitSkillFamilyChoiceBonus] = &[
    TraitSkillFamilyChoiceBonus {
        trait_id: "trait:trait_artisan",
        corpus_key: "Trait ~ Artisan",
        name: "Artisan",
        skill_families: &["Craft"],
        bonus: 2,
        description: "You spent time working under artisans, or your parents were artisans who were particularly skilled at their trade. You gain a +2 trait bonus on a single Craft skill (your choice).",
    },
    TraitSkillFamilyChoiceBonus {
        trait_id: "trait:trait_mentored",
        corpus_key: "Trait ~ Mentored",
        name: "Mentored",
        skill_families: &["Craft", "Perform", "Profession"],
        bonus: 1,
        description: "A tutor or private instructor guided you in learning your art, profession, or trade, and through your education, you became capable of teaching and guiding others. Choose a single Craft, Perform, or Profession skill. You gain a +1 trait bonus on checks with that skill. You also gain a +1 trait bonus when you aid another's skill check with any skill.",
    },
    TraitSkillFamilyChoiceBonus {
        trait_id: "trait:trait_simple_disciple",
        corpus_key: "Trait ~ Simple Disciple",
        name: "Simple Disciple",
        skill_families: &["Craft", "Profession"],
        bonus: 1,
        description: "You picked up skill in a craft or a profession during your time at a monastery. You gain a +1 trait bonus on checks with a Profession or Craft skill of your choice.",
    },
    TraitSkillFamilyChoiceBonus {
        trait_id: "trait:trait_talented",
        corpus_key: "Trait ~ Talented",
        name: "Talented",
        skill_families: &["Perform"],
        bonus: 1,
        description: "You are a virtuoso musician, actor, or storyteller. You gain a +1 trait bonus on checks with a single Perform skill (your choice), and all Perform skills are always class skills for you.",
    },
];

/// Looks up one [`FAMILY_CHOICE_TRAIT_BONUSES`] entry by its wire
/// `trait_id`.
fn find_family_choice_by_trait_id(trait_id: &str) -> Option<&'static TraitSkillFamilyChoiceBonus> {
    FAMILY_CHOICE_TRAIT_BONUSES.iter().find(|entry| entry.trait_id == trait_id)
}

/// The full, deduplicated union of every `skill:` id belonging to any of
/// `entry.skill_families`, in family-then-member order -- resolved live
/// via [`skill_allocation::skill_family_member_ids`], the same closed
/// roster that module's own `TYPE=<Family>` class-skill-wildcard
/// expansion already uses, never a second hand-duplicated list. A family
/// name this crate does not (yet) carry a roster for (`skill_family_
/// member_ids` returns `None`) contributes no options -- omit rather than
/// fabricate, same discipline as every other entry point in this module.
pub fn family_choice_skill_options(
    entry: &TraitSkillFamilyChoiceBonus,
) -> Vec<&'static str> {
    let mut options: Vec<&'static str> = Vec::new();
    for family in entry.skill_families {
        let Some(members) = crate::rules_core::skill_allocation::skill_family_member_ids(family)
        else {
            continue;
        };
        for skill_id in members {
            if !options.contains(skill_id) {
                options.push(skill_id);
            }
        }
    }
    options
}

/// The real, computed skill bonus contribution of every
/// [`FAMILY_CHOICE_TRAIT_BONUSES`] trait in `selected_traits` **that also
/// carries a genuine, corpus-legal recorded choice** in
/// `selected_choices` -- identical "omit rather than fabricate" discipline
/// as [`skill_choice_bonuses_from_traits`], just checking membership
/// against [`family_choice_skill_options`]'s resolved union instead of a
/// hand-transcribed literal list.
pub fn family_choice_bonuses_from_traits(
    selected_traits: &[String],
    selected_choices: &[SelectedChoice],
) -> BTreeMap<String, i8> {
    let mut totals: BTreeMap<String, i8> = BTreeMap::new();
    for trait_id in selected_traits {
        let Some(entry) = find_family_choice_by_trait_id(trait_id) else {
            continue;
        };
        let choice_set_id = trait_skill_choice_id(trait_id);
        let Some(chosen) = selected_choices
            .iter()
            .find(|choice| choice.choice_set_id == choice_set_id)
        else {
            continue;
        };
        let options = family_choice_skill_options(entry);
        if !options.contains(&chosen.selection_id.as_str()) {
            // An untrusted or stale selection outside this trait's own
            // resolved family union -- never honored.
            continue;
        }
        let slot = totals.entry(chosen.selection_id.clone()).or_insert(0);
        *slot = slot.saturating_add(entry.bonus);
    }
    totals
}

/// **AT-34-E4-002's classifier-facing entry point for the third,
/// family-choice slice**, the same shape as [`skill_choice_trait_
/// magnitude_is_grounded_for_corpus_key`]: takes a corpus trait record's
/// own `KEY` token, ACTUALLY BUILDS a minimal fixture character who
/// selected exactly that trait *and* recorded a choice for its resolved
/// family union's first entry, runs it through the real
/// [`crate::rules_core::skill_allocation::allocate_skill_ranks`] engine,
/// and returns the genuine, computed `misc_modifier` -- never an assumed
/// value. Returns `None` for any corpus key outside the 4-record
/// family-choice slice, or in the unreachable case the fixture-executed
/// value ever disagreed with the transcribed table.
pub fn family_choice_trait_magnitude_is_grounded_for_corpus_key(corpus_key: &str) -> Option<i8> {
    let entry = FAMILY_CHOICE_TRAIT_BONUSES
        .iter()
        .find(|entry| entry.corpus_key == corpus_key)?;
    let options = family_choice_skill_options(entry);
    let chosen_skill = *options.first()?;

    let input = CharacterInput {
        case_id: None,
        source_package_id: "at_34_e4_002_fixture".to_owned(),
        chosen: ChosenCharacterState {
            race_id: "race:human".to_owned(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:fighter".to_owned(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: vec![crate::rules_core::character_input::SkillAllocation {
                skill_id: chosen_skill.to_owned(),
                ranks: 1,
            }],
            equipment_selections: Vec::new(),
            selected_choices: vec![SelectedChoice {
                choice_set_id: trait_skill_choice_id(entry.trait_id),
                selection_id: chosen_skill.to_owned(),
            }],
            selected_traits: vec![entry.trait_id.to_owned()],
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    };

    let totals = crate::rules_core::skill_allocation::allocate_skill_ranks(&input);
    let computed = totals.totals.get(chosen_skill).map(|total| total.misc_modifier)?;

    if computed == entry.bonus {
        Some(computed)
    } else {
        None
    }
}

/// One flat `BONUS:SAVE` trait -- see the module doc comment's "Fourth
/// slice" section for the exact filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraitSaveBonus {
    /// The wire id `CharacterInput.chosen.selected_traits` carries for
    /// this trait -- same `"trait:" + corpus filename slug` idiom as
    /// [`TraitSkillBonus::trait_id`].
    pub trait_id: &'static str,
    /// The record's own corpus `KEY` token, as transcribed from
    /// `data.key`.
    pub corpus_key: &'static str,
    /// The trait's display name, transcribed from the corpus record's own
    /// `name` field.
    pub name: &'static str,
    /// Which of the three saves this trait's `BONUS:SAVE` token targets
    /// -- exactly `"Fortitude"`, `"Reflex"`, or `"Will"`, transcribed
    /// verbatim from the token's own second field.
    pub save: &'static str,
    /// The flat integer bonus, transcribed from the record's own
    /// `BONUS:SAVE|<Save>|<n>|...` token.
    pub bonus: i8,
    /// The trait's own corpus `description` field, verbatim.
    pub description: &'static str,
}

/// The 2-of-59 `ultimate_campaign` `trait_content` records whose corpus
/// `BONUS` token is a single, flat, named-save `SAVE` bonus with no
/// `%LIST` choice -- see the module doc comment's "Fourth slice" section.
pub static SAVE_TRAIT_BONUSES: &[TraitSaveBonus] = &[
    TraitSaveBonus {
        trait_id: "trait:trait_life_of_toil",
        corpus_key: "Trait ~ Life of Toil",
        name: "Life of Toil",
        save: "Fortitude",
        bonus: 1,
        description: "You have lived a physically taxing life, working long hours for a master or to support a trade. Hard physical labor has toughened your body and mind. You gain a +1 trait bonus on Fortitude saves.",
    },
    TraitSaveBonus {
        trait_id: "trait:trait_indomitable_faith",
        corpus_key: "Trait ~ Indomitable Faith",
        name: "Indomitable Faith",
        save: "Will",
        bonus: 1,
        description: "You were born in a region where your faith was not popular, but you still have never abandoned it. Your constant struggle to maintain your own faith has bolstered your drive. You gain a +1 trait bonus on Will saves.",
    },
];

/// Looks up one [`SAVE_TRAIT_BONUSES`] entry by its wire `trait_id`.
fn find_save_by_trait_id(trait_id: &str) -> Option<&'static TraitSaveBonus> {
    SAVE_TRAIT_BONUSES.iter().find(|entry| entry.trait_id == trait_id)
}

/// Three-save total, the same shape `feat_effects::SaveBonusesFromFeats`
/// already established for feats -- summed by
/// `pilot_compute::compute_total_saves` alongside every other save-bonus
/// source (ability modifier, feats, class/race features).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SaveBonusesFromTraits {
    pub fortitude: i16,
    pub reflex: i16,
    pub will: i16,
}

/// Sums the real, computed save bonus from every [`SAVE_TRAIT_BONUSES`]
/// trait in `selected_traits`. A trait id this module does not recognize
/// (outside the 2-record flat-save slice) contributes nothing -- never a
/// fabricated bonus, the same "omit rather than fabricate" discipline
/// every other entry point in this module follows. A character who
/// somehow selected both traits gets both (PF1 core rules do not forbid
/// stacking two traits that target different saves, and these two do:
/// Fortitude and Will).
pub fn save_bonuses_from_traits(selected_traits: &[String]) -> SaveBonusesFromTraits {
    let mut total = SaveBonusesFromTraits::default();
    for trait_id in selected_traits {
        let Some(entry) = find_save_by_trait_id(trait_id) else {
            continue;
        };
        match entry.save {
            "Fortitude" => total.fortitude = total.fortitude.saturating_add(entry.bonus as i16),
            "Reflex" => total.reflex = total.reflex.saturating_add(entry.bonus as i16),
            "Will" => total.will = total.will.saturating_add(entry.bonus as i16),
            // Unreachable for the two transcribed entries above; omitted
            // rather than fabricated if this table ever grows a malformed
            // entry.
            _ => {}
        }
    }
    total
}

/// **AT-34-E4-002's classifier-facing entry point for the fourth,
/// flat-save slice.** Unlike the three skill-pillar checks above (which
/// call `skill_allocation::allocate_skill_ranks` directly), a save bonus
/// is only observable through the real consumer,
/// `pilot_compute::compute_total_saves` -- so this ACTUALLY BUILDS a
/// minimal Fighter level-1 fixture character twice (once with the trait
/// selected, once without) and diffs the real computed total for the
/// specific save this entry targets, the identical pattern
/// `save_boosting_feats_widen_total_saves_tests` already proved for
/// Great Fortitude/Iron Will/Lightning Reflexes. Returns `None` for any
/// corpus key outside the 2-record flat-save slice, or in the
/// unreachable case the fixture-executed delta ever disagreed with the
/// transcribed table.
pub fn save_trait_magnitude_is_grounded_for_corpus_key(corpus_key: &str) -> Option<i8> {
    let entry = SAVE_TRAIT_BONUSES.iter().find(|entry| entry.corpus_key == corpus_key)?;

    fn fixture_input() -> CharacterInput {
        CharacterInput {
            case_id: None,
            source_package_id: "at_34_e4_002_fixture".to_owned(),
            chosen: ChosenCharacterState {
                race_id: "race:human".to_owned(),
                class_levels: vec![CharacterClassLevel {
                    class_id: "class:fighter".to_owned(),
                    level: 1,
                }],
                ability_scores: AbilityScores {
                    strength: 10,
                    dexterity: 10,
                    constitution: 10,
                    intelligence: 10,
                    wisdom: 10,
                    charisma: 10,
                },
                selected_feats: Vec::new(),
                skill_allocations: Vec::new(),
                equipment_selections: Vec::new(),
                selected_choices: Vec::new(),
                selected_traits: Vec::new(),
                spells_selected: Vec::new(),
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    let baseline = crate::rules_core::pilot_compute::compute_pilot_base_chassis(&fixture_input());
    let mut with_trait_input = fixture_input();
    with_trait_input.chosen.selected_traits.push(entry.trait_id.to_owned());
    let with_trait = crate::rules_core::pilot_compute::compute_pilot_base_chassis(&with_trait_input);

    let (base_value, new_value) = match entry.save {
        "Fortitude" => (baseline.total_saves.fortitude, with_trait.total_saves.fortitude),
        "Reflex" => (baseline.total_saves.reflex, with_trait.total_saves.reflex),
        "Will" => (baseline.total_saves.will, with_trait.total_saves.will),
        _ => return None,
    };
    let computed_delta = (new_value - base_value) as i8;

    if computed_delta == entry.bonus {
        Some(computed_delta)
    } else {
        // The engine genuinely disagreed with the transcribed table --
        // this is a real defect, never something to paper over by
        // returning the table's own value instead of the fixture's.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every entry's `skills` list is non-empty and every skill id starts
    /// with `skill:` -- a transcription-shape sanity check, not a
    /// corpus-content check (that's `skill_bonuses_from_traits_matches_the_
    /// live_corpus_for_every_flat_entry` below).
    #[test]
    fn every_table_entry_has_at_least_one_skill_id() {
        for entry in FLAT_SKILL_TRAIT_BONUSES {
            assert!(
                !entry.skills.is_empty(),
                "{} has no target skill",
                entry.trait_id
            );
            for skill_id in entry.skills {
                assert!(
                    skill_id.starts_with("skill:"),
                    "{} has malformed skill id {skill_id}",
                    entry.trait_id
                );
            }
        }
    }

    /// The table carries exactly 31 entries -- re-derive independently
    /// with the corpus filter in the module doc comment if this ever
    /// needs to change; a silent shrink or growth here is a real defect.
    #[test]
    fn table_has_exactly_thirty_one_entries() {
        assert_eq!(FLAT_SKILL_TRAIT_BONUSES.len(), 31);
    }

    /// No two entries share a `trait_id` -- `find_by_trait_id` must
    /// resolve unambiguously.
    #[test]
    fn every_trait_id_is_unique() {
        let mut ids: Vec<&str> = FLAT_SKILL_TRAIT_BONUSES.iter().map(|e| e.trait_id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), FLAT_SKILL_TRAIT_BONUSES.len());
    }

    /// The core case: one selected, recognized trait contributes its real
    /// bonus to its one target skill.
    #[test]
    fn a_single_selected_trait_contributes_its_bonus() {
        let bonuses = skill_bonuses_from_traits(&["trait:trait_acrobat".to_string()]);
        assert_eq!(bonuses.get("skill:acrobatics"), Some(&1));
        assert_eq!(bonuses.len(), 1);
    }

    /// A trait naming two skills grants the identical bonus to both.
    #[test]
    fn a_two_skill_trait_grants_both_skills() {
        let bonuses = skill_bonuses_from_traits(&["trait:trait_beast_bond".to_string()]);
        assert_eq!(bonuses.get("skill:handle_animal"), Some(&1));
        assert_eq!(bonuses.get("skill:ride"), Some(&1));
        assert_eq!(bonuses.len(), 2);
    }

    /// A negative trait bonus (`trait_principled`'s -2 Bluff) is applied
    /// verbatim, never inverted or clamped to zero.
    #[test]
    fn a_negative_trait_bonus_is_applied_verbatim() {
        let bonuses = skill_bonuses_from_traits(&["trait:trait_principled".to_string()]);
        assert_eq!(bonuses.get("skill:bluff"), Some(&-2));
    }

    /// Two selected traits both stacking on the same skill sum, rather
    /// than the second silently overwriting the first (`trait_acrobat`
    /// and `trait_reckless` both grant +1 Acrobatics).
    #[test]
    fn two_traits_targeting_the_same_skill_sum() {
        let bonuses = skill_bonuses_from_traits(&[
            "trait:trait_acrobat".to_string(),
            "trait:trait_reckless".to_string(),
        ]);
        assert_eq!(bonuses.get("skill:acrobatics"), Some(&2));
    }

    /// An unrecognized trait id (outside the 31-record flat slice, or not
    /// a real trait at all) contributes nothing -- never a fabricated
    /// bonus.
    #[test]
    fn an_unrecognized_trait_id_contributes_nothing() {
        let bonuses = skill_bonuses_from_traits(&["trait:trait_artisan".to_string()]);
        assert!(bonuses.is_empty());
    }

    /// No selected traits at all is the default, empty case.
    #[test]
    fn no_selected_traits_contributes_nothing() {
        assert!(skill_bonuses_from_traits(&[]).is_empty());
    }

    /// **The classifier-facing entry point, executed, not asserted.**
    /// `flat_skill_trait_magnitude_is_grounded_for_corpus_key` must
    /// genuinely run the fixture and agree with the transcribed table for
    /// every one of the 31 entries -- this is what `v06_work_inventory.rs`
    /// promotes `trait_content` records to `grounded` on, so a silent
    /// disagreement here would silently under- or over-promote the
    /// corpus.
    #[test]
    fn every_flat_entry_is_genuinely_grounded_by_fixture_execution() {
        for entry in FLAT_SKILL_TRAIT_BONUSES {
            let grounded = flat_skill_trait_magnitude_is_grounded_for_corpus_key(entry.corpus_key);
            assert_eq!(
                grounded,
                Some(entry.bonus),
                "{} ({}) did not ground to its transcribed bonus via real fixture execution",
                entry.trait_id,
                entry.corpus_key
            );
        }
    }

    /// A corpus key outside the flat slice (one of the choice-based,
    /// open-subtype, or formula records, or any unrelated string) is
    /// honestly `None`, never a fabricated grounding.
    #[test]
    fn an_ungrounded_corpus_key_returns_none() {
        assert_eq!(
            flat_skill_trait_magnitude_is_grounded_for_corpus_key("Trait ~ Artisan"),
            None
        );
        assert_eq!(
            flat_skill_trait_magnitude_is_grounded_for_corpus_key("not a real trait key"),
            None
        );
    }

    // -- Second slice: fixed-choice `BONUS:SKILL|%LIST` traits --------

    /// Every entry has at least two `skill_options` (a one-option "choice"
    /// would not be a real choice) and every skill id is well-formed.
    #[test]
    fn every_choice_table_entry_has_at_least_two_skill_options() {
        for entry in SKILL_CHOICE_TRAIT_BONUSES {
            assert!(
                entry.skill_options.len() >= 2,
                "{} has fewer than two skill options",
                entry.trait_id
            );
            for skill_id in entry.skill_options {
                assert!(
                    skill_id.starts_with("skill:"),
                    "{} has malformed skill id {skill_id}",
                    entry.trait_id
                );
            }
        }
    }

    /// The choice table carries exactly 5 entries -- re-derive from the
    /// corpus filter in the module doc comment's "Second slice" section if
    /// this ever needs to change.
    #[test]
    fn choice_table_has_exactly_five_entries() {
        assert_eq!(SKILL_CHOICE_TRAIT_BONUSES.len(), 5);
    }

    /// No `trait_id` is shared between the flat table and the choice
    /// table -- the two compute paths must never both claim the same
    /// trait (that would double-apply the bonus once
    /// `skill_allocation.rs` sums both maps together).
    #[test]
    fn no_trait_id_appears_in_both_tables() {
        for choice_entry in SKILL_CHOICE_TRAIT_BONUSES {
            assert!(
                find_by_trait_id(choice_entry.trait_id).is_none(),
                "{} appears in both the flat and choice tables",
                choice_entry.trait_id
            );
        }
    }

    /// With no recorded choice yet, a selected `%LIST` trait contributes
    /// nothing -- never a first-guessed default skill.
    #[test]
    fn a_choice_trait_with_no_recorded_choice_contributes_nothing() {
        let bonuses = skill_choice_bonuses_from_traits(
            &["trait:trait_criminal".to_string()],
            &[],
        );
        assert!(bonuses.is_empty());
    }

    /// The core case: a selected `%LIST` trait with a genuine, in-list
    /// recorded choice contributes its bonus to exactly that skill.
    #[test]
    fn a_choice_trait_with_a_recorded_choice_contributes_to_that_skill() {
        let bonuses = skill_choice_bonuses_from_traits(
            &["trait:trait_criminal".to_string()],
            &[SelectedChoice {
                choice_set_id: trait_skill_choice_id("trait:trait_criminal"),
                selection_id: "skill:intimidate".to_string(),
            }],
        );
        assert_eq!(bonuses.get("skill:intimidate"), Some(&1));
        assert_eq!(bonuses.len(), 1);
    }

    /// A recorded choice outside the trait's own `skill_options` (stale
    /// or untrusted data) is never honored.
    #[test]
    fn a_choice_trait_with_an_out_of_list_choice_contributes_nothing() {
        let bonuses = skill_choice_bonuses_from_traits(
            &["trait:trait_criminal".to_string()],
            &[SelectedChoice {
                choice_set_id: trait_skill_choice_id("trait:trait_criminal"),
                selection_id: "skill:acrobatics".to_string(),
            }],
        );
        assert!(bonuses.is_empty());
    }

    /// A recorded choice under a DIFFERENT trait's choice-set id is not
    /// mistaken for this trait's own choice (per-trait scoping, not one
    /// shared slot).
    #[test]
    fn a_choice_recorded_under_a_different_traits_choice_set_is_ignored() {
        let bonuses = skill_choice_bonuses_from_traits(
            &["trait:trait_criminal".to_string()],
            &[SelectedChoice {
                choice_set_id: trait_skill_choice_id("trait:trait_influence"),
                selection_id: "skill:intimidate".to_string(),
            }],
        );
        assert!(bonuses.is_empty());
    }

    /// **The choice-slice classifier-facing entry point, executed, not
    /// asserted.** Must genuinely run the fixture and agree with the
    /// transcribed table for every one of the 5 entries.
    #[test]
    fn every_choice_entry_is_genuinely_grounded_by_fixture_execution() {
        for entry in SKILL_CHOICE_TRAIT_BONUSES {
            let grounded = skill_choice_trait_magnitude_is_grounded_for_corpus_key(entry.corpus_key);
            assert_eq!(
                grounded,
                Some(entry.bonus),
                "{} ({}) did not ground to its transcribed bonus via real fixture execution",
                entry.trait_id,
                entry.corpus_key
            );
        }
    }

    /// A corpus key outside the choice slice is honestly `None`.
    #[test]
    fn an_ungrounded_choice_corpus_key_returns_none() {
        assert_eq!(
            skill_choice_trait_magnitude_is_grounded_for_corpus_key("Trait ~ Acrobat"),
            None
        );
        assert_eq!(
            skill_choice_trait_magnitude_is_grounded_for_corpus_key("not a real trait key"),
            None
        );
    }

    // -- Third slice: FAMILY_CHOICE_TRAIT_BONUSES ---------------------------

    /// The family-choice table carries exactly 4 entries -- re-derive from
    /// the module doc comment's "Third slice" section if this ever needs
    /// to change.
    #[test]
    fn family_choice_table_has_exactly_four_entries() {
        assert_eq!(FAMILY_CHOICE_TRAIT_BONUSES.len(), 4);
    }

    /// No `trait_id` is shared across any two of the three tables -- each
    /// of the three compute paths must claim a disjoint trait set, or
    /// `skill_allocation.rs` would double-apply a bonus when it sums all
    /// three maps together.
    #[test]
    fn no_trait_id_appears_in_more_than_one_table() {
        for family_entry in FAMILY_CHOICE_TRAIT_BONUSES {
            assert!(
                find_by_trait_id(family_entry.trait_id).is_none(),
                "{} appears in both the flat and family-choice tables",
                family_entry.trait_id
            );
            assert!(
                find_choice_by_trait_id(family_entry.trait_id).is_none(),
                "{} appears in both the fixed-choice and family-choice tables",
                family_entry.trait_id
            );
        }
    }

    /// Every family-choice entry resolves to a non-empty, deduplicated
    /// option list -- a family this crate does not recognize would
    /// silently produce zero options, which this catches immediately.
    #[test]
    fn every_family_choice_entry_resolves_to_a_nonempty_deduplicated_option_list() {
        for entry in FAMILY_CHOICE_TRAIT_BONUSES {
            let options = family_choice_skill_options(entry);
            assert!(
                !options.is_empty(),
                "{} resolved to zero skill options -- check its skill_families against \
                 skill_allocation::skill_family_member_ids",
                entry.trait_id
            );
            let mut deduped = options.clone();
            deduped.sort_unstable();
            deduped.dedup();
            assert_eq!(
                deduped.len(),
                options.len(),
                "{} resolved to a non-deduplicated option list",
                entry.trait_id
            );
        }
    }

    /// `trait_mentored`'s three-family union carries a Craft, a Perform,
    /// and a Profession id -- not just the first family's members.
    #[test]
    fn mentored_option_list_unions_all_three_named_families() {
        let mentored = FAMILY_CHOICE_TRAIT_BONUSES
            .iter()
            .find(|e| e.trait_id == "trait:trait_mentored")
            .expect("trait_mentored must be in the family-choice table");
        let options = family_choice_skill_options(mentored);
        assert!(options.contains(&"skill:craft_alchemy"));
        assert!(options.contains(&"skill:perform_sing"));
        assert!(options.contains(&"skill:profession_scribe"));
    }

    /// With no recorded choice yet, a selected family-choice trait
    /// contributes nothing -- never a first-guessed default skill.
    #[test]
    fn a_family_choice_trait_with_no_recorded_choice_contributes_nothing() {
        let bonuses = family_choice_bonuses_from_traits(&["trait:trait_artisan".to_string()], &[]);
        assert!(bonuses.is_empty());
    }

    /// The core case: a selected family-choice trait with a genuine,
    /// in-union recorded choice contributes its bonus to exactly that
    /// skill.
    #[test]
    fn a_family_choice_trait_with_a_recorded_choice_contributes_to_that_skill() {
        let bonuses = family_choice_bonuses_from_traits(
            &["trait:trait_artisan".to_string()],
            &[SelectedChoice {
                choice_set_id: trait_skill_choice_id("trait:trait_artisan"),
                selection_id: "skill:craft_weapons".to_string(),
            }],
        );
        assert_eq!(bonuses.get("skill:craft_weapons"), Some(&2));
        assert_eq!(bonuses.len(), 1);
    }

    /// A recorded choice outside the trait's resolved family union (stale
    /// or untrusted data, or a skill from an un-named family) is never
    /// honored.
    #[test]
    fn a_family_choice_trait_with_an_out_of_union_choice_contributes_nothing() {
        let bonuses = family_choice_bonuses_from_traits(
            &["trait:trait_artisan".to_string()],
            &[SelectedChoice {
                choice_set_id: trait_skill_choice_id("trait:trait_artisan"),
                // Perform, not Craft -- trait_artisan only names TYPE=Craft.
                selection_id: "skill:perform_sing".to_string(),
            }],
        );
        assert!(bonuses.is_empty());
    }

    /// **The family-choice-slice classifier-facing entry point, executed,
    /// not asserted.** Must genuinely run the fixture and agree with the
    /// transcribed table for every one of the 4 entries.
    #[test]
    fn every_family_choice_entry_is_genuinely_grounded_by_fixture_execution() {
        for entry in FAMILY_CHOICE_TRAIT_BONUSES {
            let grounded = family_choice_trait_magnitude_is_grounded_for_corpus_key(entry.corpus_key);
            assert_eq!(
                grounded,
                Some(entry.bonus),
                "{} ({}) did not ground to its transcribed bonus via real fixture execution",
                entry.trait_id,
                entry.corpus_key
            );
        }
    }

    /// A corpus key outside the family-choice slice is honestly `None`.
    #[test]
    fn an_ungrounded_family_choice_corpus_key_returns_none() {
        assert_eq!(
            family_choice_trait_magnitude_is_grounded_for_corpus_key("Trait ~ Acrobat"),
            None
        );
        assert_eq!(
            family_choice_trait_magnitude_is_grounded_for_corpus_key("not a real trait key"),
            None
        );
    }

    /// Both `SAVE_TRAIT_BONUSES` entries target one of the three real save
    /// names, and neither collides with the other's save (a stacking bug
    /// would be invisible if both entries happened to target the same
    /// save).
    #[test]
    fn save_table_entries_target_distinct_real_saves() {
        let saves: Vec<&str> = SAVE_TRAIT_BONUSES.iter().map(|entry| entry.save).collect();
        for save in &saves {
            assert!(
                ["Fortitude", "Reflex", "Will"].contains(save),
                "{save} is not a real save name"
            );
        }
        let mut deduped = saves.clone();
        deduped.sort_unstable();
        deduped.dedup();
        assert_eq!(deduped.len(), saves.len(), "two entries target the same save");
    }

    /// No `trait_id` in `SAVE_TRAIT_BONUSES` collides with either of the
    /// two skill-pillar tables -- proves the three summed maps
    /// `skill_allocation::allocate_skill_ranks` sums, plus this module's
    /// own save path, never double-apply a single selected trait.
    #[test]
    fn no_save_trait_id_collides_with_a_skill_pillar_trait_id() {
        for entry in SAVE_TRAIT_BONUSES {
            assert!(
                find_by_trait_id(entry.trait_id).is_none(),
                "{} appears in both the save and flat-skill tables",
                entry.trait_id
            );
            assert!(
                find_choice_by_trait_id(entry.trait_id).is_none(),
                "{} appears in both the save and skill-choice tables",
                entry.trait_id
            );
            assert!(
                find_family_choice_by_trait_id(entry.trait_id).is_none(),
                "{} appears in both the save and family-choice tables",
                entry.trait_id
            );
        }
    }

    /// An unselected save trait contributes nothing.
    #[test]
    fn no_selected_save_traits_contribute_nothing() {
        assert_eq!(save_bonuses_from_traits(&[]), SaveBonusesFromTraits::default());
        assert_eq!(
            save_bonuses_from_traits(&["trait:trait_reckless".to_string()]),
            SaveBonusesFromTraits::default()
        );
    }

    /// The core case: selecting Life of Toil contributes +1 Fortitude and
    /// nothing else; selecting Indomitable Faith contributes +1 Will and
    /// nothing else; selecting both sums independently onto their own
    /// saves.
    #[test]
    fn selected_save_traits_contribute_to_exactly_their_own_save() {
        let life_of_toil = save_bonuses_from_traits(&["trait:trait_life_of_toil".to_string()]);
        assert_eq!(
            life_of_toil,
            SaveBonusesFromTraits { fortitude: 1, reflex: 0, will: 0 }
        );

        let indomitable_faith =
            save_bonuses_from_traits(&["trait:trait_indomitable_faith".to_string()]);
        assert_eq!(
            indomitable_faith,
            SaveBonusesFromTraits { fortitude: 0, reflex: 0, will: 1 }
        );

        let both = save_bonuses_from_traits(&[
            "trait:trait_life_of_toil".to_string(),
            "trait:trait_indomitable_faith".to_string(),
        ]);
        assert_eq!(both, SaveBonusesFromTraits { fortitude: 1, reflex: 0, will: 1 });
    }

    /// **The flat-save-slice classifier-facing entry point, executed, not
    /// asserted.** Must genuinely run the fixture through the real
    /// `pilot_compute::compute_total_saves` consumer and agree with the
    /// transcribed table for both entries.
    #[test]
    fn every_save_entry_is_genuinely_grounded_by_fixture_execution() {
        for entry in SAVE_TRAIT_BONUSES {
            let grounded = save_trait_magnitude_is_grounded_for_corpus_key(entry.corpus_key);
            assert_eq!(
                grounded,
                Some(entry.bonus),
                "{} ({}) did not ground to its transcribed bonus via real fixture execution",
                entry.trait_id,
                entry.corpus_key
            );
        }
    }

    /// A corpus key outside the flat-save slice is honestly `None`.
    #[test]
    fn an_ungrounded_save_corpus_key_returns_none() {
        assert_eq!(
            save_trait_magnitude_is_grounded_for_corpus_key("Trait ~ Acrobat"),
            None
        );
        assert_eq!(
            save_trait_magnitude_is_grounded_for_corpus_key("not a real trait key"),
            None
        );
    }
}
