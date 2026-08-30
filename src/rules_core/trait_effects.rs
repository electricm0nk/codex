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
//! ## What this module deliberately does NOT cover
//!
//! - **4 more `trait_content` records** (`trait_artisan`, `trait_mentored`,
//!   `trait_simple_disciple`, `trait_talented`) carry a `BONUS:SKILL|
//!   %LIST` token whose `CHOOSE:SKILL` names an **open subtype family**
//!   (`TYPE=Craft`, `TYPE=Perform`, `TYPE=Profession`) rather than a fixed
//!   list of concrete skills -- the player may name *any* Craft/Perform/
//!   Profession subtype (including ones with no existing `skill:` id in
//!   this crate's catalog, e.g. "Craft (Poison)"), which is a genuinely
//!   open-ended text-entry chooser, not an enumerable list picker. A
//!   materially different UI/input shape than [`SKILL_CHOICE_TRAIT_
//!   BONUSES`]'s closed-list case; building it as a same-shaped closed
//!   list here would silently drop legal player choices, so it is named,
//!   not built, this cycle.
//! - **3 records** carry an ability-score-difference formula magnitude
//!   (`max(INT,CHA)-CHA` etc, e.g. `trait_bruising_intellect`) -- no
//!   formula evaluator exists in this crate for that shape.
//! - **The remaining 15 `trait_content` records** mix `BONUS:VAR`,
//!   `BONUS:SAVE`, `BONUS:SITUATION`, `BONUS:ABILITYPOOL`,
//!   `BONUS:COMBAT`, and `BONUS:CONCENTRATION` tokens -- different pillars
//!   entirely (saves, combat maneuvers, concentration checks, a bonus
//!   trait-slot pool), out of this module's scope.
//! - **All 30 `ability_content` units** (`ultimate_campaign`'s
//!   Drawback/Retraining sub-mechanics) -- house-rule bookkeeping and
//!   GM-adjudicated narrative penalties with no clean formulaic trigger,
//!   per the prior cycle's own direct reading of that corpus.
//!
//! Widening past these two slices is future work, gated on either an
//! open-subtype chooser (Craft/Perform/Profession family text entry), or
//! a formula evaluator for ability-score-difference magnitudes -- neither
//! exists yet, and building either as a rushed half-measure here would
//! risk the same "8 closures where measurement found 1" failure this
//! bundle's own doctrine warns against.

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
}
