//! SD-20 Epic 4 — skill-rank allocation engine. Scope draft §1.4.
//!
//! First work-unit (per the loop instruction's Step 2 per-epic order):
//! **class-skill handling**. `allocate_skill_ranks` determines, for each
//! of the character's user-allocated skill ranks
//! (`CharacterInput.chosen.skill_allocations` — the existing, already-wired
//! type; see below), whether that skill is a class skill for the
//! character's class(es), and computes each allocated skill's total
//! modifier. A later Epic-4 cycle adds max-rank-cap handling on top of
//! this module (`SkillTotals.untrained_use` is present at the type level,
//! per `technical-design.md` §2.3, and is populated starting with the
//! third work-unit below; this cycle does not yet enforce a max-rank-cap
//! diagnostic).
//!
//! Second work-unit (this cycle): **cross-class-penalty handling**. See
//! the "PF1 cross-class rule" section below for the exact rule (confirmed
//! against the SRD, not guessed) and what this cycle does and does not
//! cover.
//!
//! ## Deviation from `technical-design.md` §2.3's illustrative seam
//!
//! `technical-design.md` §2.3 illustrates
//! `allocate_skill_ranks(input, allocation: &SkillAllocation, rules_tables:
//! &RulesTables) -> SkillTotals`. Two of those three parameters don't
//! survive contact with the real repo, mirroring the doc-note precedent
//! `pilot_compute_corpus.rs` already set ("`PilotReceipt` in the doctrine
//! doc's illustrative code does not exist in this repo"):
//!
//! - `allocation: &SkillAllocation` (a `BTreeMap<SkillId, u8>` wrapper) is
//!   redundant with the already-landed
//!   `CharacterInput.chosen.skill_allocations: Vec<character_input::SkillAllocation>`
//!   (`skill_id: String, ranks: u8` per record) — the character's own
//!   ranks-per-skill choices are already on the wire type. Introducing a
//!   second, differently-shaped carrier for the same information would
//!   duplicate the single wire contract every SD-20 Epic-1 cycle to date
//!   has kept singular (`classify_character_input(&CharacterInput)`,
//!   `to_pilot_receipt(&CorpusPilotReceipt)` both take the composed input
//!   type directly, no echo parameter).
//! - `rules_tables: &RulesTables` has no defined type anywhere in this
//!   repo as of this cycle (confirmed by grep: zero hits outside
//!   `technical-design.md`'s own illustrative code). It is referenced
//!   identically across Epic 2/3/4/5's seam signatures, so it is shared
//!   infrastructure no single epic's file-touch partition (own parent
//!   module only) can safely invent unilaterally without risking a
//!   shape collision with a concurrent sibling stream — this exact
//!   reasoning is independently recorded by the Epic-3 sibling cycle's
//!   blocked-cycle log entry (`~/workspace/SD-20-rules-engine-completeness-progress.md`,
//!   cycle-2026-07-17T1920).
//!
//! ## Class-skill data source
//!
//! `src/rules_core/rules_tables/crb/` carries no class-skill-list table
//! today — only `class_tables.rs`'s per-class-per-level base-attack-bonus
//! and base-save rows (`ClassTableRow`), confirmed by grep across the
//! whole `rules_tables` tree before writing this cycle's RED test.
//! Extending that table store is out of SD-20's authority (SD-19 owns the
//! table store's shape; see the loop instruction's hard stops, "SD-20
//! cannot extend it autonomously") and `rules_tables/crb/` sits outside
//! this epic's file-touch partition regardless.
//!
//! Unlike Epic 3's feat catalog (verified absent *everywhere* in the
//! repo, informal or otherwise — see the Epic-3 blocked-cycle log entry
//! above), a class-skill fact already exists, grounded and shipped, in
//! `pilot_compute.rs`'s deterministic bounded posture: the comment block
//! above `compute_selected_skill_modifiers` cites `cr_abilities_class.lst:2835`
//! for "Fighter class skills include Climb, Intimidate, Swim" (also
//! `cr_skills.lst:10/42/102` for each skill's own `TYPE=ClassSkill`
//! entry). This module reuses those same three already-cited skill
//! identities — not a duplicate derivation, the same underlying evidence
//! — scoped identically (Fighter only, these three skills only). It does
//! not import `pilot_compute.rs`'s private constants (they're not `pub`);
//! it composes with the chassis's own already-computed, public
//! `AbilityModifiers` (via `compute_pilot_base_chassis`) for the ability
//! side of the math, rather than re-deriving ability modifiers.
//!
//! No skill or class outside this bounded, cited posture is ever claimed
//! as a class skill, and no skill outside the bounded ability-key mapping
//! below is ever given a fabricated `ability_modifier` — such allocations
//! are simply absent from `SkillTotals.totals` rather than populated with
//! an invented number. Widening this posture (more classes, more skills)
//! is future Epic-4 cycle territory, gated on either a genuine SD-19
//! table-store extension or the operator authorizing one.
//!
//! This cycle widens the bounded ability-key mapping by exactly one
//! skill, `skill:diplomacy` (Charisma-keyed), to make the cross-class path
//! exercisable at all against the grounded Fighter posture: Diplomacy is
//! confirmed *not* a member of Fighter's class-skill list (see the
//! "PF1 cross-class rule" section below for the citation), so it is a
//! genuine, cited cross-class example for a Fighter build — not an
//! invented one.
//!
//! ## PF1 core rule reused as-is
//!
//! A skill that is a class skill for the character and has at least 1
//! rank invested gets a flat +3 trained bonus. This is a system-wide PF1
//! rule (identical for every class and every skill), not per-class table
//! data — the same status this codebase already grants the ability-score
//! modifier formula (`floor(score / 2) - 5`, computed with no corpus
//! citation anywhere in `pilot_compute.rs`).
//!
//! ## PF1 cross-class rule (confirmed, not guessed)
//!
//! Pathfinder 1st Edition removed D&D 3.5's "cross-class skills cost 2
//! skill points per rank" rule (confirmed via a Paizo rules-forum thread
//! discussing precisely this point of common confusion, and independently
//! via the Roll20 PF1 compendium's "Acquiring Skills" page: a skill point
//! always buys exactly 1 rank, regardless of class-skill status). The two
//! real mechanical differences a cross-class skill has in PF1 are:
//!
//! 1. It never gets the class skill's flat +3 trained bonus (already
//!    landed in the class-skill-handling cycle — the existing
//!    `class_skill_bonus` computation already yields `0` for any skill
//!    not in `class_skills`, so no change was needed there).
//! 2. Its maximum investable rank is *half* a class skill's cap, rounded
//!    up: `ceil((character level + 1) / 2)`, versus a class skill's
//!    `character level + 3`, per `scope-draft.md` §1.4's explicit formula.
//!    This module enforces only the cross-class half-cap this cycle — the
//!    class-skill cap (`character level + 3`) and *diagnostic* surfacing
//!    of cap violations for either category are explicitly the later
//!    "max-rank-cap handling" work-unit (see the loop instruction's Step
//!    2 per-epic order). This cycle silently reports the true, legal
//!    effective rank total for a cross-class skill — never the raw
//!    over-allocated number, and never a diagnostic (that's out of this
//!    cycle's scope).
//!
//! `SkillId` "skill:diplomacy"'s cross-class status for Fighter is cited
//! from `cr_abilities_class.lst:2835` ("Fighter Core Class Skills ...
//! CSKILL:Climb|TYPE=Craft|Handle Animal|Intimidate|Knowledge
//! (Dungeoneering)|Knowledge (Engineering)|TYPE=Profession|Ride|Survival|
//! Swim" — no Diplomacy) and its Charisma key from `cr_skills.lst:35`
//! (`Diplomacy ... KEYSTAT:CHA`).
//!
//! Third work-unit (this cycle): **untrained-use handling**.
//!
//! ## PF1 untrained-use rule (confirmed, not guessed)
//!
//! Most PF1 skills may be attempted with zero ranks invested, at the
//! character's raw ability-modifier value (no ranks, no trained bonus —
//! this was already this module's behavior for any recognized skill with
//! `ranks == 0`, prior to this cycle, simply because `class_skill_bonus`
//! is `0` below 1 rank). A bounded set of skills are "Trained Only" per
//! the Core Rulebook's skill summary table (Disable Device, Handle
//! Animal, all Knowledge subtypes, Linguistics, Profession, Sleight of
//! Hand, Spellcraft, Use Magic Device): a character with zero ranks
//! invested in one of those skills cannot attempt the check at all — no
//! total modifier of any kind, fabricated or otherwise, may be reported
//! for it.
//!
//! This cycle widens the bounded ability-key mapping by exactly one
//! skill, `skill:disable_device` (Dexterity-keyed), to make the
//! trained-only path exercisable at all: cited from `cr_skills.lst:36`
//! (`Disable Device ... KEYSTAT:DEX ... USEUNTRAINED:NO`) — the same
//! corpus file this module's earlier cycles already cite for every other
//! recognized skill's key-ability mapping. Disable Device is not a member
//! of Fighter's grounded class-skill list either (see the citation
//! above), so a ranked Disable Device allocation on a Fighter build also
//! exercises the already-landed cross-class path; that is incidental to
//! this cycle, not a new rule.
//!
//! `SkillTotals.untrained_use` (present at the type level since the
//! class-skill-handling cycle, always empty until now) is populated by
//! this cycle: for every recognized, allocated skill whose *final*
//! effective rank count is `0` (which, after the trained-only exclusion
//! above, can only be a skill that is genuinely usable untrained), the
//! map records that skill's raw ability-modifier value — the same number
//! already present in `SkillTotal.total_modifier` for that entry, since
//! ranks and class-skill bonus are both `0` at that point. This is a
//! deliberately narrow, non-fabricating scope: `untrained_use` mirrors
//! only what `totals` already grounds; it does not enumerate every skill
//! in the bounded universe regardless of whether the character allocated
//! it (no such enumeration exists anywhere in this module).
//!
//! Fourth work-unit (this cycle): **max-rank-cap handling**. Closes
//! Epic 4.
//!
//! ## PF1 max-rank-cap rule (confirmed, not guessed)
//!
//! Per `scope-draft.md` §1.4 / `epic-breakdown.md` criterion 9: "class
//! skills max at character level + 3, cross-class skills max at
//! (character level + 1) / 2 rounded up. Cap violations produce
//! diagnostics, not fabricated totals."
//!
//! Checked against what the two prior work-units already do, to confirm
//! this is genuinely distinct rather than already covered:
//!
//! - The class-skill cap (`character level + 3`) was **never enforced**
//!   before this cycle — the class-skill-handling cycle's `(ranks,
//!   class_skill_bonus) = (allocation.ranks, bonus)` branch passes the
//!   raw allocated `ranks` straight through, uncapped, and the
//!   cross-class-penalty cycle did not touch that branch.
//! - The cross-class half-cap (`ceil((character level + 1) / 2)`) *was*
//!   already enforced by the cross-class-penalty cycle
//!   (`allocation.ranks.min(cross_class_cap)`), but silently — no
//!   diagnostic was ever produced when that clip actually fired on a
//!   genuine over-allocation. Criterion 9's diagnostic requirement is not
//!   yet satisfied for that cap either.
//!
//! This cycle therefore does two things: (1) enforces the previously
//! unenforced class-skill cap, and (2) adds a `ComputationDiagnostic` to
//! the new `SkillTotals.diagnostics` field whenever either cap actually
//! clips a raw allocation — for both categories, not just the newly
//! enforced one. In both cases the reported `SkillTotal.ranks` remains
//! the real, legal, capped number (never the raw over-allocated one, and
//! never silently uncapped) — the diagnostic is additive information, not
//! a change to the already-correct capped total.

use std::collections::BTreeMap;

use crate::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SkillAllocation,
};
use crate::rules_core::pilot_compute::{
    compute_pilot_base_chassis, AbilityModifiers, ComputationDiagnostic,
};

/// Plain-string skill identity, matching
/// `character_input::SkillAllocation.skill_id`'s existing convention —
/// class/item/spell identities are all `String` throughout this crate;
/// no typed `SkillId` enum exists, and none is introduced here.
pub type SkillId = String;

/// `character_input::CharacterClassLevel.class_id` value that hits the
/// grounded Fighter class-skill posture below. Matches
/// `pilot_compute.rs`'s own `FIGHTER_CLASS_ID` value (verified by
/// reading its source; that constant itself is not `pub`, so it is not
/// imported, only matched by value).
const FIGHTER_CLASS_ID: &str = "class:fighter";

/// **AT-34-E3-003 (bucket M, `skill_content_table_holds_record_magnitude_
/// not_yet_computed`, `decisions.md §2a`): Fighter's REAL, FULL class-skill
/// list, superseding the old 3-skill bounded slice.** Every entry is
/// Fighter's own literal `CSKILL:` token, read directly from
/// `rules_tables::crb::class_skill_tables::CLASS_SKILL_LISTS`'s
/// `"class:fighter"` row -- a table this repo already built and verified
/// byte-for-byte against `cr_abilities_class.lst:2835`
/// (`class_skill_lists_match_their_own_corpus_records`), not re-derived or
/// re-typed here. Falls back to an empty slice (never a fabricated one) if
/// that row is ever renamed or removed -- a real closed-list lookup, never
/// a name pattern.
fn full_fighter_class_skills() -> &'static [&'static str] {
    crate::rules_core::rules_tables::crb::class_skill_tables::CLASS_SKILL_LISTS
        .iter()
        .find(|list| list.owner_id == FIGHTER_CLASS_ID)
        .map(|list| list.skills)
        .unwrap_or(&[])
}

/// v0.6 alpha swarm: Wizard and Rogue both now reach `Computed` for real
/// characters (this file's own recognition was still Fighter-only). Matches
/// `pilot_compute.rs`'s `ROGUE_CLASS_ID`/`WIZARD_CLASS_ID` values (verified
/// by reading their source; not imported, only matched by value, same
/// convention as `FIGHTER_CLASS_ID` above).
const ROGUE_CLASS_ID: &str = "class:rogue";
const WIZARD_CLASS_ID: &str = "class:wizard";

/// The bounded, cited Rogue class-skill posture -- verified against the
/// real PCGen corpus, `cr_abilities_class.lst:2838` ("Rogue Core Class
/// Skills ... CSKILL:Acrobatics|Appraise|Bluff|Climb|TYPE=Craft|Diplomacy|
/// Disable Device|Disguise|Escape Artist|Intimidate|Knowledge
/// (Dungeoneering)|Knowledge (Local)|Linguistics|Perception|TYPE=Perform|
/// TYPE=Profession|Sense Motive|Sleight of Hand|Stealth|Swim|Use Magic
/// Device"): every one of this module's five bounded, recognized skills
/// (Climb, Intimidate, Swim, Diplomacy, Disable Device) is confirmed a
/// genuine Rogue class skill, so Rogue's grounded contribution here is
/// the module's full bounded skill universe.
const GROUNDED_ROGUE_CLASS_SKILLS: &[&str] = &[
    "skill:climb",
    "skill:intimidate",
    "skill:swim",
    "skill:diplomacy",
    "skill:disable_device",
];

/// **AT-34-E3-003: Wizard's REAL, FULL class-skill list**, superseding the
/// old always-empty 5-skill-intersection slice. Verified against the real
/// PCGen corpus record itself --
/// `data/corpus/core_rulebook/class_feature/wizard/class_skills.json`
/// (`cr_abilities_class.lst:2565`), whose own `ABILITY:` token reads
/// `"Class Skill|AUTOMATIC|Appraise|Craft|Fly|Knowledge|Linguistics|
/// Profession|Spellcraft"` and whose `DESC:` states the two bare-family
/// entries explicitly: "Knowledge (all) (Int)" and "Craft (Int)" /
/// "Profession (Wis)" with no subtype named, i.e. every subskill of that
/// family. `CLASS_SKILL_LISTS` (`rules_tables::crb::class_skill_tables`)
/// does not carry a Wizard row (Wizard is not one of its 9 transcribed
/// classes), so this list is transcribed here directly from the same
/// corpus file, in the same `TYPE=<Family>` wildcard convention
/// [`is_full_class_skill`] already expands for Fighter.
const FULL_WIZARD_CLASS_SKILLS: &[&str] = &[
    "Appraise",
    "TYPE=Craft",
    "Fly",
    "TYPE=Knowledge",
    "Linguistics",
    "TYPE=Profession",
    "Spellcraft",
];

/// PF1 core rule: flat trained bonus for any class skill with at least 1
/// rank invested. A system-wide constant, not per-class/per-book table
/// data — see the module doc comment's closing section.
const TRAINED_CLASS_SKILL_BONUS: i8 = 3;

/// **AT-34-E3-003: the module's own already-stated, previously-unimplemented
/// "Trained Only" roster**, per the module doc comment's "PF1 untrained-use
/// rule" section: "Disable Device, Handle Animal, all Knowledge subtypes,
/// Linguistics, Profession, Sleight of Hand, Spellcraft, Use Magic Device" --
/// the Core Rulebook's own skill-summary table (`cr_skills.lst`'s
/// `USEUNTRAINED:NO` token, confirmed present on exactly these skills'
/// records and absent from every other skill's). Only `skill:disable_device`
/// was actually enforced before this cycle; the rest of this doc-comment's
/// own list is implemented now, not widened past what it already named.
const TRAINED_ONLY_SKILLS: &[&str] = &[
    "skill:disable_device",
    "skill:handle_animal",
    "skill:linguistics",
    "skill:sleight_of_hand",
    "skill:spellcraft",
    "skill:use_magic_device",
];

/// Every core_rulebook `Knowledge (<subtype>)` skill's `skill:` wire id
/// (`skillIdFor`'s convention, mirrored in
/// `apps/desktop/src/characterHub/skillsModel.ts`), transcribed from
/// `data/corpus/core_rulebook/skill/*.json`'s own 10 `Knowledge (...)`
/// records. Used both as `TRAINED_ONLY_SKILLS`'s "all Knowledge subtypes"
/// member test and as [`is_full_class_skill`]'s `TYPE=Knowledge` wildcard
/// expansion (Wizard's own class-skill grant).
const KNOWLEDGE_SKILL_IDS: &[&str] = &[
    "skill:knowledge_arcana",
    "skill:knowledge_dungeoneering",
    "skill:knowledge_engineering",
    "skill:knowledge_geography",
    "skill:knowledge_history",
    "skill:knowledge_local",
    "skill:knowledge_nature",
    "skill:knowledge_nobility",
    "skill:knowledge_planes",
    "skill:knowledge_religion",
];

/// Every core_rulebook `Craft (<subtype>)` skill's wire id, transcribed
/// from `data/corpus/core_rulebook/skill/*.json`'s 23 `Craft (...)`
/// records (excluding the separate `Craft (Untrained)` catch-all record,
/// a different PCGen shape with no `BONUS:SKILL` class-skill token at
/// all). Used by [`is_full_class_skill`]'s `TYPE=Craft` wildcard
/// expansion (Fighter, Rogue, and Wizard all grant this family).
const CRAFT_SKILL_IDS: &[&str] = &[
    "skill:craft_alchemy",
    "skill:craft_armor",
    "skill:craft_baskets",
    "skill:craft_blacksmithing",
    "skill:craft_books",
    "skill:craft_bows",
    "skill:craft_calligraphy",
    "skill:craft_carpentry",
    "skill:craft_cloth",
    "skill:craft_clothing",
    "skill:craft_gemcutting",
    "skill:craft_glass",
    "skill:craft_jewelry",
    "skill:craft_leather",
    "skill:craft_locks",
    "skill:craft_paintings",
    "skill:craft_pottery",
    "skill:craft_sculptures",
    "skill:craft_ships",
    "skill:craft_shoes",
    "skill:craft_stonemasonry",
    "skill:craft_traps",
    "skill:craft_weapons",
];

/// Every core_rulebook `Perform (<subtype>)` skill's wire id, transcribed
/// from `data/corpus/core_rulebook/skill/*.json`'s 9 `Perform (...)`
/// records (excluding `Perform (Untrained)`, same shape exclusion as
/// [`CRAFT_SKILL_IDS`]). Used by [`is_full_class_skill`]'s `TYPE=Perform`
/// wildcard expansion (Rogue's own class-skill grant).
const PERFORM_SKILL_IDS: &[&str] = &[
    "skill:perform_act",
    "skill:perform_comedy",
    "skill:perform_dance",
    "skill:perform_keyboard_instruments",
    "skill:perform_oratory",
    "skill:perform_percussion_instruments",
    "skill:perform_sing",
    "skill:perform_string_instruments",
    "skill:perform_wind_instruments",
];

/// Every core_rulebook `Profession (<subtype>)` skill's wire id,
/// transcribed from `data/corpus/core_rulebook/skill/*.json`'s 31
/// `Profession (...)` records (excluding `Profession (Untrained)`, same
/// shape exclusion as [`CRAFT_SKILL_IDS`]). Used both as
/// `TRAINED_ONLY_SKILLS`'s "Profession" member test and as
/// [`is_full_class_skill`]'s `TYPE=Profession` wildcard expansion
/// (Fighter, Rogue, and Wizard all grant this family).
const PROFESSION_SKILL_IDS: &[&str] = &[
    "skill:profession_architect",
    "skill:profession_baker",
    "skill:profession_barrister",
    "skill:profession_brewer",
    "skill:profession_butcher",
    "skill:profession_clerk",
    "skill:profession_cook",
    "skill:profession_courtesan",
    "skill:profession_driver",
    "skill:profession_engineer",
    "skill:profession_farmer",
    "skill:profession_fisherman",
    "skill:profession_gambler",
    "skill:profession_gardener",
    "skill:profession_herbalist",
    "skill:profession_innkeeper",
    "skill:profession_librarian",
    "skill:profession_merchant",
    "skill:profession_midwife",
    "skill:profession_miller",
    "skill:profession_miner",
    "skill:profession_porter",
    "skill:profession_sailor",
    "skill:profession_scribe",
    "skill:profession_shepherd",
    "skill:profession_soldier",
    "skill:profession_soothsayer",
    "skill:profession_stable_master",
    "skill:profession_tanner",
    "skill:profession_trapper",
    "skill:profession_woodcutter",
];

/// Whether `skill_id` matches the literal, un-prefixed corpus `CSKILL:`
/// entry `name` once normalized to the `skill:` wire convention
/// (`apps/desktop/src/characterHub/skillsModel.ts`'s `skillIdFor`, mirrored
/// here byte-for-byte: lowercase, strip parens, collapse any run of
/// non-alphanumeric characters to one underscore, trim leading/trailing
/// underscores).
fn normalize_skill_display_name(name: &str) -> SkillId {
    let mut normalized = String::with_capacity(name.len() + 6);
    normalized.push_str("skill:");
    let mut pending_sep = false;
    for ch in name.chars() {
        if ch == '(' || ch == ')' {
            continue;
        }
        if ch.is_ascii_alphanumeric() {
            if pending_sep && normalized.len() > "skill:".len() {
                normalized.push('_');
            }
            normalized.push(ch.to_ascii_lowercase());
            pending_sep = false;
        } else {
            pending_sep = true;
        }
    }
    normalized
}

fn normalized_skill_id_matches(name: &str, skill_id: &str) -> bool {
    normalize_skill_display_name(name) == skill_id
}

/// The family member-id list a `TYPE=<Family>` corpus wildcard token
/// expands to, or `None` for a family this module does not (yet) carry an
/// enumerated roster for.
///
/// `pub(crate)` (AT-34-E4-002, third slice): `trait_effects.rs`'s
/// `FAMILY_CHOICE_TRAIT_BONUSES` reuses this exact same corpus-backed
/// enumeration as the closed, legal option set for a trait whose
/// `CHOOSE:SKILL` token names a `TYPE=<Family>` subtype family
/// (`trait_artisan`, `trait_mentored`, `trait_simple_disciple`,
/// `trait_talented`) rather than a fixed list of concrete skill names --
/// this module already treats `TYPE=Craft`/`TYPE=Perform`/`TYPE=Profession`
/// as closed, corpus-enumerated universes for class-skill-wildcard
/// purposes, so a second, independent "open text entry" chooser for the
/// same three families would silently disagree with what this module
/// itself already considers legal.
pub(crate) fn skill_family_member_ids(family: &str) -> Option<&'static [&'static str]> {
    match family {
        "Craft" => Some(CRAFT_SKILL_IDS),
        "Knowledge" => Some(KNOWLEDGE_SKILL_IDS),
        "Perform" => Some(PERFORM_SKILL_IDS),
        "Profession" => Some(PROFESSION_SKILL_IDS),
        _ => None,
    }
}

/// Whether `skill_id` is a class skill under `raw_list` (a `CLASS_SKILL_
/// LISTS`-shaped literal entry list: bare skill names and/or `TYPE=<Family>`
/// wildcard tokens, exactly as transcribed from the corpus's own `CSKILL:`/
/// `ABILITY:` token). A `TYPE=<Family>` entry matches every id in that
/// family's member list ([`skill_family_member_ids`]); a bare name matches
/// only after the same normalization the corpus-verification tests already
/// use ([`normalized_skill_id_matches`]). Returns `false` (never a
/// fabricated match) for a family this module has no member list for.
fn is_full_class_skill(raw_list: &[&str], skill_id: &str) -> bool {
    raw_list.iter().any(|entry| {
        if let Some(family) = entry.strip_prefix("TYPE=") {
            skill_family_member_ids(family).is_some_and(|members| members.contains(&skill_id))
        } else {
            normalized_skill_id_matches(entry, skill_id)
        }
    })
}

/// [`ComputationDiagnostic::id`] for a class skill's raw allocated ranks
/// exceeding its `character level + 3` cap. See the module doc comment's
/// "PF1 max-rank-cap rule" section.
const CLASS_SKILL_MAX_RANK_EXCEEDED_ID: &str = "skill_allocation.class_skill_max_rank_exceeded";

/// [`ComputationDiagnostic::id`] for a cross-class skill's raw allocated
/// ranks exceeding its `ceil((character level + 1) / 2)` cap. See the
/// module doc comment's "PF1 max-rank-cap rule" section.
const CROSS_CLASS_MAX_RANK_EXCEEDED_ID: &str = "skill_allocation.cross_class_max_rank_exceeded";

/// Output of [`allocate_skill_ranks`]. Per `technical-design.md` §2.3.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SkillTotals {
    /// One entry per skill the character actually allocated ranks to
    /// *and* that this module's bounded, cited posture recognizes (see
    /// the module doc comment). Skills outside that bounded posture are
    /// omitted here rather than populated with a fabricated modifier.
    pub totals: BTreeMap<SkillId, SkillTotal>,
    /// The character's full class-skill set (union across all of the
    /// character's classes that this module's bounded posture
    /// recognizes), independent of whether ranks were actually
    /// allocated to each member. Sorted for determinism.
    pub class_skills: Vec<SkillId>,
    /// `true` when at least one of the character's allocated skills was a
    /// recognized cross-class skill (not in `class_skills`), meaning PF1's
    /// cross-class half-cap (`ceil((character level + 1) / 2)`, see the
    /// module doc comment's "PF1 cross-class rule" section) was applied to
    /// that skill's effective `ranks` in `totals`. `false` when no
    /// allocated skill fell into that bounded, recognized cross-class
    /// universe.
    pub cross_class_penalty_applied: bool,
    /// One entry per recognized, allocated skill that was actually used
    /// untrained (i.e. its final effective `ranks` in `totals` is `0`),
    /// mapping to that skill's raw ability-modifier value — see the
    /// module doc comment's "PF1 untrained-use rule" section. A
    /// trained-only skill (see [`TRAINED_ONLY_SKILLS`]) with zero ranks
    /// never appears here, or in `totals`, at all: it cannot be attempted
    /// untrained, so no total of any kind is reported for it.
    pub untrained_use: BTreeMap<SkillId, i8>,
    /// One [`ComputationDiagnostic`] per recognized, allocated skill whose
    /// raw allocated ranks exceeded its legal PF1 max-rank cap (class
    /// skill: `character level + 3`; cross-class: `ceil((character
    /// level + 1) / 2)`) — see the module doc comment's "PF1
    /// max-rank-cap rule" section. Every such diagnostic has
    /// `claim_blocking: false`: the corresponding `SkillTotal.ranks` in
    /// `totals` already carries the real, legal, capped number, never
    /// the raw over-allocated one and never a fabricated uncapped one.
    /// Empty when no allocated skill in this module's bounded,
    /// recognized universe exceeded its cap.
    pub diagnostics: Vec<ComputationDiagnostic>,
}

/// One skill's computed total, per `technical-design.md` §2.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkillTotal {
    pub ranks: u8,
    pub ability_modifier: i8,
    pub class_skill_bonus: i8,
    /// **AT-34-E4-002**: now carries the real, computed trait bonus for
    /// this skill (`trait_effects::skill_bonuses_from_traits`), when the
    /// character selected a recognized flat-`BONUS:SKILL` trait targeting
    /// it. Armor-check-penalty and other non-trait `misc_modifier`
    /// contributions remain Epic 5's territory and are not yet composed
    /// in; `0` for any character with no trait bonus on this skill,
    /// exactly the prior behavior.
    pub misc_modifier: i8,
    pub total_modifier: i8,
}

/// The bounded, cited ability-key mapping for the skills this module
/// currently recognizes at all (see the module doc comment). Returns
/// `None` for any skill id outside that bounded universe — callers must
/// not fabricate a modifier in that case.
fn skill_key_ability_modifier(skill_id: &str, ability_modifiers: &AbilityModifiers) -> Option<i16> {
    let AbilityModifiers { strength, dexterity, constitution: _, intelligence, wisdom, charisma } =
        *ability_modifiers;
    // **AT-34-E3-003: every one of the Core Rulebook's 35 skill categories,
    // not the original 5-skill bounded slice.** Every `KEYSTAT:` below is
    // read directly from `data/corpus/core_rulebook/skill/*.json`'s own
    // `KEYSTAT` token (each record independently confirmed against the
    // live corpus, not carried over from any other source) -- the same
    // `skill_ability_key_matches_the_live_corpus_for_every_skill` test
    // below re-derives this table from that corpus at test time so a
    // future corpus edit that moved a `KEYSTAT:` would fail loudly here,
    // never silently disagree with the character sheet.
    if let Some(family) = skill_id.strip_prefix("skill:craft_") {
        return (!family.is_empty()).then_some(intelligence);
    }
    if let Some(family) = skill_id.strip_prefix("skill:knowledge_") {
        return (!family.is_empty()).then_some(intelligence);
    }
    if let Some(family) = skill_id.strip_prefix("skill:perform_") {
        return (!family.is_empty()).then_some(charisma);
    }
    if let Some(family) = skill_id.strip_prefix("skill:profession_") {
        return (!family.is_empty()).then_some(wisdom);
    }
    match skill_id {
        "skill:acrobatics" | "skill:disable_device" | "skill:escape_artist" | "skill:fly"
        | "skill:ride" | "skill:sleight_of_hand" | "skill:stealth" => Some(dexterity),
        "skill:appraise" | "skill:linguistics" | "skill:spellcraft" => Some(intelligence),
        "skill:bluff" | "skill:diplomacy" | "skill:disguise" | "skill:handle_animal"
        | "skill:intimidate" | "skill:use_magic_device" => Some(charisma),
        "skill:climb" | "skill:swim" => Some(strength),
        "skill:heal" | "skill:perception" | "skill:sense_motive" | "skill:survival" => {
            Some(wisdom)
        }
        _ => None,
    }
}

/// Whether `skill_id` is one of this module's bounded, cited "Trained
/// Only" skills. See [`TRAINED_ONLY_SKILLS`] and the module doc comment's
/// "PF1 untrained-use rule" section. The doc comment's roster names two
/// whole families ("all Knowledge subtypes", "Profession") rather than
/// enumerating every subtype id twice in this file -- family membership is
/// checked the same way [`is_full_class_skill`] checks a `TYPE=` wildcard,
/// against the same [`KNOWLEDGE_SKILL_IDS`]/[`PROFESSION_SKILL_IDS`]
/// rosters, never a separate, driftable copy.
fn is_trained_only_skill(skill_id: &str) -> bool {
    TRAINED_ONLY_SKILLS.contains(&skill_id)
        || KNOWLEDGE_SKILL_IDS.contains(&skill_id)
        || PROFESSION_SKILL_IDS.contains(&skill_id)
}

/// The character's total level across every class (PF1's "character
/// level" for skill-rank-cap purposes — the sum, not any single class's
/// level).
fn character_level(input: &CharacterInput) -> u16 {
    input
        .chosen
        .class_levels
        .iter()
        .map(|class_level| class_level.level as u16)
        .sum()
}

/// PF1's cross-class maximum rank cap: half a class skill's cap, rounded
/// up — `ceil((character level + 1) / 2)`, per `scope-draft.md` §1.4's
/// explicit formula. See the module doc comment's "PF1 cross-class rule"
/// section for what this is and is not (no diagnostic surfacing here;
/// that's the later max-rank-cap-handling work-unit).
fn cross_class_max_ranks(character_level: u16) -> u8 {
    // ceil((character_level + 1) / 2) == (character_level + 2) / 2 under
    // integer (floor) division.
    ((character_level + 2) / 2) as u8
}

/// PF1's class-skill maximum rank cap: `character level + 3`, per
/// `scope-draft.md` §1.4's explicit formula. See the module doc comment's
/// "PF1 max-rank-cap rule" section.
fn class_skill_max_ranks(character_level: u16) -> u8 {
    (character_level + 3) as u8
}

/// Expands a `CLASS_SKILL_LISTS`-shaped raw entry list (bare names and/or
/// `TYPE=<Family>` wildcard tokens) into the concrete `skill:` ids it
/// grants, deduplicated. A `TYPE=<Family>` entry expands to every id in
/// [`skill_family_member_ids`]'s roster for that family (never a guessed
/// enumeration -- families this module has no roster for contribute
/// nothing, same as an unrecognized bare name would); a bare name expands
/// to its own normalized id ([`normalize_skill_display_name`]).
fn expand_raw_class_skill_list(raw_list: &[&str]) -> Vec<SkillId> {
    let mut expanded = Vec::new();
    for entry in raw_list {
        if let Some(family) = entry.strip_prefix("TYPE=") {
            if let Some(members) = skill_family_member_ids(family) {
                for member in members {
                    if !expanded.iter().any(|existing: &SkillId| existing == member) {
                        expanded.push((*member).to_string());
                    }
                }
            }
            continue;
        }
        let id = normalize_skill_display_name(entry);
        if !expanded.contains(&id) {
            expanded.push(id);
        }
    }
    expanded
}

/// The character's class-skill set: the union, across every class the
/// character has levels in, of that class's grounded class-skill
/// posture.
///
/// **AT-34-E3-003 (`decisions.md §2a`): Fighter and Wizard now contribute
/// their REAL, FULL class-skill lists** ([`full_fighter_class_skills`] /
/// [`FULL_WIZARD_CLASS_SKILLS`], expanded via
/// [`expand_raw_class_skill_list`]) rather than the old 3-skill/always-empty
/// bounded slices -- see those constants' own doc comments for the corpus
/// citation each entry traces to. **Rogue is deliberately left at its
/// existing bounded 5-skill posture** in this cycle: `pilot_compute_corpus
/// ::has_supported_class_chassis` (the gate `resolve_unified_pilot_snapshot`
/// checks before this module's output ever reaches a real character sheet)
/// recognizes only Fighter and Wizard chassis, so widening Rogue's list
/// here would add a class-skill claim this module could not yet prove
/// reaches a player -- a real widening, staged for whenever Rogue joins
/// that supported-chassis set. Every other class still contributes
/// nothing, same bounded-caution philosophy as before.
fn class_skill_set(input: &CharacterInput) -> Vec<SkillId> {
    let mut class_skills: Vec<SkillId> = Vec::new();
    for class_level in &input.chosen.class_levels {
        let grounded: Vec<SkillId> = match class_level.class_id.as_str() {
            FIGHTER_CLASS_ID => expand_raw_class_skill_list(full_fighter_class_skills()),
            ROGUE_CLASS_ID => {
                GROUNDED_ROGUE_CLASS_SKILLS.iter().map(|s| (*s).to_string()).collect()
            }
            WIZARD_CLASS_ID => expand_raw_class_skill_list(FULL_WIZARD_CLASS_SKILLS),
            _ => Vec::new(),
        };
        for skill_id in grounded {
            if !class_skills.contains(&skill_id) {
                class_skills.push(skill_id);
            }
        }
    }
    class_skills.sort();
    class_skills
}

/// Whether the character has at least one class with a *grounded*
/// class-skill posture (Fighter, Rogue, or Wizard) -- see
/// `class_skill_set`'s own doc comment for what "grounded" means for each.
/// Only then do we have real PF1 evidence of whether a given skill is
/// cross-class rather than simply unknown; a build with no grounded
/// class-skill posture at all gets no cross-class treatment, same
/// bounded-caution philosophy `class_skill_set` already follows.
fn has_grounded_class_skill_posture(input: &CharacterInput) -> bool {
    input.chosen.class_levels.iter().any(|class_level| {
        matches!(
            class_level.class_id.as_str(),
            FIGHTER_CLASS_ID | ROGUE_CLASS_ID | WIZARD_CLASS_ID
        )
    })
}

/// Computes per-skill rank totals for every skill the character both
/// allocated ranks to and that this module's bounded, cited posture
/// recognizes. Applies PF1's class-skill cap, cross-class half-cap, and
/// trained-only exclusion (see the module doc comment's "PF1 cross-class
/// rule", "PF1 untrained-use rule", and "PF1 max-rank-cap rule"
/// sections), surfacing a [`ComputationDiagnostic`] whenever either cap
/// actually clips a raw over-allocation. See the module doc comment for
/// what's deliberately not yet handled (non-Fighter class-skill
/// postures).
pub fn allocate_skill_ranks(input: &CharacterInput) -> SkillTotals {
    let chassis = compute_pilot_base_chassis(input);
    let class_skills = class_skill_set(input);
    let level = character_level(input);
    let cross_class_cap = cross_class_max_ranks(level);
    let class_cap = class_skill_max_ranks(level);
    // The cross-class half-cap is only knowable for a skill when the
    // character has at least one class with a *grounded* class-skill
    // posture (Fighter, Rogue, or Wizard as of the v0.6 alpha swarm --
    // see `has_grounded_class_skill_posture`'s own doc comment) -- only
    // then do we have real PF1 evidence that a given skill is cross-class
    // rather than simply unknown. A build with no grounded class-skill
    // posture at all gets no cross-class treatment, same bounded-caution
    // philosophy `class_skill_set` already follows.
    let has_grounded_class_skill_posture = has_grounded_class_skill_posture(input);

    // **AT-34-E4-002**: the character's real, computed trait skill bonuses
    // (the 31-of-59 `ultimate_campaign` flat `BONUS:SKILL` traits --
    // `trait_effects`'s own doc comment names the exact shape and what is
    // deliberately not yet covered). Empty for any character with no
    // `selected_traits` or none this module recognizes -- byte-identical
    // to pre-cycle behavior for every existing fixture.
    let mut trait_skill_bonuses =
        crate::rules_core::trait_effects::skill_bonuses_from_traits(&input.chosen.selected_traits);
    // Second slice: fixed-choice `%LIST` traits (`trait_effects`'s own
    // "Second slice" doc-comment section) -- summed into the same map, not
    // double-applied, because `no_trait_id_appears_in_both_tables` proves
    // no trait id is ever a member of both tables.
    for (skill_id, bonus) in crate::rules_core::trait_effects::skill_choice_bonuses_from_traits(
        &input.chosen.selected_traits,
        &input.chosen.selected_choices,
    ) {
        let slot = trait_skill_bonuses.entry(skill_id).or_insert(0);
        *slot = slot.saturating_add(bonus);
    }
    // Third slice (`AT-34-E4-002`): `%LIST` traits whose `CHOOSE:SKILL`
    // names an open `TYPE=<Family>` subtype family (Craft/Perform/
    // Profession) rather than a fixed list of concrete skills -- folded
    // into the same map, not double-applied, because a trait id can only
    // ever be a member of one of the three tables (enforced by
    // `no_trait_id_appears_in_more_than_one_table`).
    for (skill_id, bonus) in crate::rules_core::trait_effects::family_choice_bonuses_from_traits(
        &input.chosen.selected_traits,
        &input.chosen.selected_choices,
    ) {
        let slot = trait_skill_bonuses.entry(skill_id).or_insert(0);
        *slot = slot.saturating_add(bonus);
    }

    let mut totals = BTreeMap::new();
    let mut untrained_use = BTreeMap::new();
    let mut diagnostics = Vec::new();
    let mut cross_class_penalty_applied = false;
    for allocation in &input.chosen.skill_allocations {
        let Some(ability_mod) =
            skill_key_ability_modifier(&allocation.skill_id, &chassis.ability_modifiers)
        else {
            // Outside the bounded, cited skill universe: no known
            // ability-key mapping. Omit rather than fabricate.
            continue;
        };

        if is_trained_only_skill(&allocation.skill_id) && allocation.ranks == 0 {
            // PF1's untrained-use rule: a trained-only skill with zero
            // ranks invested cannot be attempted at all. See the module
            // doc comment's "PF1 untrained-use rule" section. Omit
            // entirely rather than reporting any total (fabricated or
            // otherwise).
            continue;
        }

        let is_class_skill = class_skills
            .iter()
            .any(|skill_id| skill_id == &allocation.skill_id);

        // Cross-class skills never carry the trained bonus (PF1 rule,
        // already true before this cycle for any non-class-skill) and, for
        // a build whose class-skill posture is actually grounded, are
        // additionally capped at PF1's cross-class half-cap — the ranks
        // actually usable by this character, not the raw allocation. See
        // the module doc comment's "PF1 cross-class rule" section.
        let (ranks, class_skill_bonus) = if is_class_skill {
            let bonus = if allocation.ranks >= 1 {
                TRAINED_CLASS_SKILL_BONUS
            } else {
                0
            };
            // PF1's class-skill max-rank cap (`character level + 3`, see
            // the module doc comment's "PF1 max-rank-cap rule" section):
            // the real, legal, capped ranks -- never the raw
            // over-allocated number -- with a diagnostic recording the
            // violation when the raw allocation actually exceeded it.
            if allocation.ranks > class_cap {
                diagnostics.push(ComputationDiagnostic {
                    id: CLASS_SKILL_MAX_RANK_EXCEEDED_ID.to_string(),
                    message: format!(
                        "{} allocated {} ranks exceeds the class-skill max-rank cap of {} at character level {}",
                        allocation.skill_id, allocation.ranks, class_cap, level
                    ),
                    claim_blocking: false,
                });
            }
            (allocation.ranks.min(class_cap), bonus)
        } else if has_grounded_class_skill_posture {
            cross_class_penalty_applied = true;
            // PF1's cross-class half-cap (see the module doc comment's
            // "PF1 cross-class rule" and "PF1 max-rank-cap rule"
            // sections): same capped-total behavior as before this
            // cycle, now paired with a diagnostic on genuine violation.
            if allocation.ranks > cross_class_cap {
                diagnostics.push(ComputationDiagnostic {
                    id: CROSS_CLASS_MAX_RANK_EXCEEDED_ID.to_string(),
                    message: format!(
                        "{} allocated {} ranks exceeds the cross-class max-rank cap of {} at character level {}",
                        allocation.skill_id, allocation.ranks, cross_class_cap, level
                    ),
                    claim_blocking: false,
                });
            }
            (allocation.ranks.min(cross_class_cap), 0)
        } else {
            (allocation.ranks, 0)
        };

        let ability_modifier = ability_mod as i8;
        let misc_modifier =
            trait_skill_bonuses.get(&allocation.skill_id).copied().unwrap_or(0);
        let total_modifier = ranks as i8 + ability_modifier + class_skill_bonus + misc_modifier;

        if ranks == 0 {
            // Genuinely usable untrained: the trained-only, zero-rank case
            // was already excluded above, so any zero-rank entry reaching
            // this point is a skill PF1 allows to be attempted without
            // ranks. Record its raw ability-modifier value. See the
            // module doc comment's "PF1 untrained-use rule" section.
            untrained_use.insert(allocation.skill_id.clone(), ability_modifier);
        }

        totals.insert(
            allocation.skill_id.clone(),
            SkillTotal {
                ranks,
                ability_modifier,
                class_skill_bonus,
                misc_modifier,
                total_modifier,
            },
        );
    }

    // **AT-34-E4-002**: a trait's flat skill bonus applies whether or not
    // the character has invested any ranks in the skill (PF1 traits are
    // not rank-gated the way class-skill/cross-class caps are) -- a
    // character who took Acrobat but has 0 Acrobatics ranks must still
    // see the +1, not silently lose it because no `skill_allocations`
    // entry exists for it yet. Mirrors the untrained-use rule immediately
    // above: a trained-only skill (`is_trained_only_skill`) with no ranks
    // still cannot be attempted at all, trait bonus or not, so it is
    // correctly omitted here too -- never a fabricated total for a skill
    // the character cannot use.
    for (skill_id, bonus) in &trait_skill_bonuses {
        if totals.contains_key(skill_id) {
            // Already handled by the allocation loop above (which already
            // folded this same `trait_skill_bonuses` value into its own
            // `misc_modifier`) -- do not double-count.
            continue;
        }
        if is_trained_only_skill(skill_id) {
            continue;
        }
        let Some(ability_mod) = skill_key_ability_modifier(skill_id, &chassis.ability_modifiers)
        else {
            // Outside the bounded, cited skill universe: no known
            // ability-key mapping. Omit rather than fabricate, same as
            // the allocation loop above.
            continue;
        };
        let ability_modifier = ability_mod as i8;
        let total_modifier = ability_modifier + bonus;
        untrained_use.insert(skill_id.clone(), ability_modifier);
        totals.insert(
            skill_id.clone(),
            SkillTotal {
                ranks: 0,
                ability_modifier,
                class_skill_bonus: 0,
                misc_modifier: *bonus,
                total_modifier,
            },
        );
    }

    SkillTotals {
        totals,
        class_skills,
        cross_class_penalty_applied,
        untrained_use,
        diagnostics,
    }
}

/// Whether `skill_id` is a class skill for `class_id` under this module's
/// class-skill lists (Fighter and Wizard now full, per [`class_skill_set`]'s
/// own doc comment; Rogue at its existing bounded 5-skill list). `false`
/// for any other class -- a real, checkable fact, never a fabricated one
/// for a class this module carries no data for.
pub fn is_class_skill_for(class_id: &str, skill_id: &str) -> bool {
    match class_id {
        FIGHTER_CLASS_ID => is_full_class_skill(full_fighter_class_skills(), skill_id),
        WIZARD_CLASS_ID => is_full_class_skill(FULL_WIZARD_CLASS_SKILLS, skill_id),
        ROGUE_CLASS_ID => GROUNDED_ROGUE_CLASS_SKILLS.contains(&skill_id),
        _ => false,
    }
}

/// **AT-34-E3-003's own fixture-execution instrument.** Given a skill and a
/// class this module recognizes, actually BUILDS a minimal level-1
/// character of that class with exactly 1 rank allocated to that skill,
/// runs it through the real [`allocate_skill_ranks`] engine, and returns
/// the genuine, computed `class_skill_bonus` this module's engine produces
/// -- never an assertion that it "should" be 3, an executed check that it
/// IS. Returns `None` for a class/skill pair [`is_class_skill_for`] does
/// not recognize (nothing to fixture-check). This is the same shape the
/// module doc comment's "PF1 core rule reused as-is" section already
/// documents as a fixed, system-wide constant
/// ([`TRAINED_CLASS_SKILL_BONUS`]) -- this function proves that constant is
/// genuinely reachable for `skill_id` through a real class, not merely
/// declared. Used by `v06_work_inventory.rs`'s `Kind::Skill` classifier
/// (`AT-34-E3-003`) to ground a corpus record's class-skill-bonus magnitude
/// only when this fixture actually executes and agrees, never on the
/// classifier's own say-so.
pub fn class_skill_bonus_is_grounded(class_id: &str, skill_id: &str) -> Option<i8> {
    if !is_class_skill_for(class_id, skill_id) {
        return None;
    }
    let input = CharacterInput {
        case_id: None,
        source_package_id: "at_34_e3_003_fixture".to_owned(),
        chosen: ChosenCharacterState {
            race_id: "race:human".to_owned(),
            class_levels: vec![CharacterClassLevel { class_id: class_id.to_owned(), level: 1 }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: vec![SkillAllocation { skill_id: skill_id.to_owned(), ranks: 1 }],
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            selected_traits: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    };
    allocate_skill_ranks(&input).totals.get(skill_id).map(|total| total.class_skill_bonus)
}

/// **AT-34-E3-003's classifier-facing entry point.** Takes a corpus skill
/// record's own display name (e.g. `"Acrobatics"`, `"Craft (Alchemy)"` --
/// exactly the string `v06_work_inventory.rs`'s `Kind::Skill` unit carries
/// as `unit.name`), normalizes it the SAME way [`class_skill_set`]'s own
/// `TYPE=` expansion does ([`normalize_skill_display_name`]), and checks
/// it against every class this module recognizes at all (Fighter, Rogue,
/// Wizard -- [`FIGHTER_CLASS_ID`], [`ROGUE_CLASS_ID`], [`WIZARD_CLASS_ID`]),
/// in that fixed order so the result is deterministic when more than one
/// recognized class shares the skill. Returns the first
/// [`class_skill_bonus_is_grounded`] hit -- a real, fixture-executed
/// `class_skill_bonus`, never an assumed `3` -- or `None` when no class
/// this module has real data for treats the skill as a class skill (most
/// often because the skill belongs only to a class this module's bounded
/// posture does not yet cover, e.g. Bard's `Perform`).
pub fn skill_bonus_is_grounded_for_display_name(display_name: &str) -> Option<i8> {
    let skill_id = normalize_skill_display_name(display_name);
    for class_id in [FIGHTER_CLASS_ID, ROGUE_CLASS_ID, WIZARD_CLASS_ID] {
        if let Some(bonus) = class_skill_bonus_is_grounded(class_id, &skill_id) {
            return Some(bonus);
        }
    }
    None
}

/// v0.6 alpha swarm: this module's class-skill recognition was still
/// Fighter-only even though Wizard and Rogue both now reach `Computed` for
/// real characters. Confirmed empirically before fixing: a level-1 Wizard
/// allocating 5 ranks to the cross-class skill Diplomacy (real cross-class
/// cap at level 1 is `ceil((1+1)/2) = 1`) got the raw, uncapped 5 ranks
/// back with `cross_class_penalty_applied: false` and no diagnostic --
/// PF1's cross-class rank cap was silently unenforced for any class this
/// module didn't recognize. Ground Rogue's real class-skill list (all five
/// of this module's bounded skills, per `cr_abilities_class.lst:2838`) and
/// Wizard's (a checked, genuinely empty intersection, per
/// `cr_abilities_class.lst:2565`) to close this for both.
#[cfg(test)]
mod wizard_and_rogue_class_skill_grounding_tests {
    use super::allocate_skill_ranks;
    use crate::rules_core::character_input::{
        AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SkillAllocation,
    };

    fn single_class_with_skill(class_id: &str, level: u8, skill_id: &str, ranks: u8) -> CharacterInput {
        CharacterInput {
            case_id: None,
            source_package_id: "test".to_owned(),
            chosen: ChosenCharacterState {
                race_id: "race:human".to_owned(),
                class_levels: vec![CharacterClassLevel { class_id: class_id.to_owned(), level }],
                ability_scores: AbilityScores {
                    strength: 10,
                    dexterity: 10,
                    constitution: 10,
                    intelligence: 16,
                    wisdom: 10,
                    charisma: 10,
                },
                selected_feats: Vec::new(),
                skill_allocations: vec![SkillAllocation {
                    skill_id: skill_id.to_owned(),
                    ranks,
                }],
                equipment_selections: Vec::new(),
                selected_choices: Vec::new(),
                selected_traits: Vec::new(),
                spells_selected: Vec::new(),
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    #[test]
    fn wizard_cross_class_over_allocation_is_now_capped_and_flagged() {
        // Real gap this test guards: before grounding Wizard, this exact
        // scenario returned the raw, uncapped 5 ranks with
        // cross_class_penalty_applied: false and no diagnostic.
        let input = single_class_with_skill("class:wizard", 1, "skill:diplomacy", 5);
        let totals = allocate_skill_ranks(&input);

        let diplomacy = totals
            .totals
            .get("skill:diplomacy")
            .expect("recognized skill must be present");
        assert_eq!(diplomacy.ranks, 1, "cross-class cap at level 1 is ceil((1+1)/2) = 1");
        assert_eq!(diplomacy.class_skill_bonus, 0, "Wizard has no class-skill bonus on Diplomacy");
        // AT-34-E3-003: Wizard's class-skill list is no longer the always-
        // empty 5-skill-intersection slice -- it now carries Wizard's REAL
        // full list (Appraise/Craft/Fly/Knowledge/Linguistics/Profession/
        // Spellcraft), so this must assert the real, still-true fact
        // (Diplomacy specifically is not on it) rather than the list being
        // empty overall.
        assert!(
            !totals.class_skills.is_empty(),
            "Wizard's grounded class-skill list now has real content (Appraise, Craft, Fly, \
             Knowledge, Linguistics, Profession, Spellcraft)"
        );
        assert!(
            !totals.class_skills.iter().any(|s| s == "skill:diplomacy"),
            "Diplomacy is still genuinely not a Wizard class skill"
        );
        assert!(totals.cross_class_penalty_applied);
        assert!(
            totals
                .diagnostics
                .iter()
                .any(|d| d.id == "skill_allocation.cross_class_max_rank_exceeded"),
            "the over-allocation must now be flagged: {:?}",
            totals.diagnostics
        );
    }

    #[test]
    fn wizard_within_cross_class_cap_is_not_flagged() {
        let input = single_class_with_skill("class:wizard", 1, "skill:diplomacy", 1);
        let totals = allocate_skill_ranks(&input);

        assert!(
            totals.diagnostics.is_empty(),
            "an in-budget allocation must not be flagged: {:?}",
            totals.diagnostics
        );
    }

    #[test]
    fn rogue_gets_the_class_skill_bonus_on_all_five_bounded_skills() {
        for skill_id in ["skill:climb", "skill:intimidate", "skill:swim", "skill:diplomacy", "skill:disable_device"]
        {
            let input = single_class_with_skill("class:rogue", 1, skill_id, 1);
            let totals = allocate_skill_ranks(&input);

            let total = totals
                .totals
                .get(skill_id)
                .unwrap_or_else(|| panic!("{skill_id} should be recognized"));
            assert_eq!(
                total.class_skill_bonus, 3,
                "{skill_id} is a real Rogue class skill and must get the +3 trained bonus"
            );
            assert!(
                totals.class_skills.iter().any(|s| s == skill_id),
                "{skill_id} must appear in Rogue's grounded class_skills list"
            );
        }
    }

    #[test]
    fn rogue_class_skill_over_allocation_uses_the_wider_class_cap_not_the_cross_class_cap() {
        // Class-skill cap at level 1 is level + 3 = 4; cross-class cap
        // would be ceil((1+1)/2) = 1. Allocate 4 ranks -- legal for a
        // class skill, would be flagged if Rogue's grounding were missing
        // and this fell through to the cross-class path instead.
        let input = single_class_with_skill("class:rogue", 1, "skill:climb", 4);
        let totals = allocate_skill_ranks(&input);

        let climb = totals.totals.get("skill:climb").expect("recognized skill must be present");
        assert_eq!(climb.ranks, 4, "4 ranks is within the class-skill cap (level + 3 = 4)");
        assert!(
            totals.diagnostics.is_empty(),
            "a legal class-skill allocation must not be flagged: {:?}",
            totals.diagnostics
        );
    }
}

/// AT-34-E3-003 (bucket M, `decisions.md §2a`): Fighter and Wizard's newly
/// FULL class-skill lists, including the `TYPE=Craft`/`TYPE=Knowledge`/
/// `TYPE=Profession` wildcard expansions -- one real, executed fixture per
/// claim, never an assertion the classifier trusts unverified.
#[cfg(test)]
mod at_34_e3_003_full_class_skill_list_tests {
    use super::{allocate_skill_ranks, class_skill_bonus_is_grounded, is_class_skill_for};
    use crate::rules_core::character_input::{
        AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SkillAllocation,
    };

    fn single_class_with_skill(class_id: &str, level: u8, skill_id: &str, ranks: u8) -> CharacterInput {
        CharacterInput {
            case_id: None,
            source_package_id: "test".to_owned(),
            chosen: ChosenCharacterState {
                race_id: "race:human".to_owned(),
                class_levels: vec![CharacterClassLevel { class_id: class_id.to_owned(), level }],
                ability_scores: AbilityScores {
                    strength: 10,
                    dexterity: 10,
                    constitution: 10,
                    intelligence: 16,
                    wisdom: 12,
                    charisma: 10,
                },
                selected_feats: Vec::new(),
                skill_allocations: vec![SkillAllocation { skill_id: skill_id.to_owned(), ranks }],
                equipment_selections: Vec::new(),
                selected_choices: Vec::new(),
                selected_traits: Vec::new(),
                spells_selected: Vec::new(),
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    #[test]
    fn fighter_gets_the_class_skill_bonus_on_a_directly_named_skill_the_old_slice_omitted() {
        // Real gap this test guards: Handle Animal is a real Fighter class
        // skill (`cr_abilities_class.lst:2835`) that the old 3-skill
        // bounded slice never recognized at all.
        let input = single_class_with_skill("class:fighter", 1, "skill:handle_animal", 1);
        let totals = allocate_skill_ranks(&input);
        let total = totals.totals.get("skill:handle_animal").expect("skill:handle_animal should be recognized");
        assert_eq!(total.class_skill_bonus, 3, "Handle Animal is a real Fighter class skill");
        assert!(totals.class_skills.iter().any(|s| s == "skill:handle_animal"));
    }

    #[test]
    fn fighter_type_craft_wildcard_grants_every_craft_subtype_the_class_skill_bonus() {
        for skill_id in ["skill:craft_alchemy", "skill:craft_weapons", "skill:craft_armor"] {
            let input = single_class_with_skill("class:fighter", 1, skill_id, 1);
            let totals = allocate_skill_ranks(&input);
            let total = totals.totals.get(skill_id).unwrap_or_else(|| panic!("{skill_id} should be recognized"));
            assert_eq!(
                total.class_skill_bonus, 3,
                "{skill_id} is granted by Fighter's TYPE=Craft wildcard"
            );
            assert_eq!(total.ability_modifier, 3, "{skill_id} is Intelligence-keyed (INT 16 -> +3)");
        }
    }

    #[test]
    fn fighter_type_profession_wildcard_grants_every_profession_subtype() {
        let input = single_class_with_skill("class:fighter", 1, "skill:profession_merchant", 1);
        let totals = allocate_skill_ranks(&input);
        let total = totals.totals.get("skill:profession_merchant").expect("recognized skill");
        assert_eq!(total.class_skill_bonus, 3, "granted by Fighter's TYPE=Profession wildcard");
    }

    #[test]
    fn wizard_type_knowledge_wildcard_grants_every_knowledge_subtype() {
        for skill_id in ["skill:knowledge_arcana", "skill:knowledge_religion"] {
            let input = single_class_with_skill("class:wizard", 1, skill_id, 1);
            let totals = allocate_skill_ranks(&input);
            let total = totals.totals.get(skill_id).unwrap_or_else(|| panic!("{skill_id} should be recognized"));
            assert_eq!(total.class_skill_bonus, 3, "{skill_id} is granted by Wizard's TYPE=Knowledge wildcard");
        }
    }

    #[test]
    fn wizard_type_craft_and_profession_wildcards_grant_every_subtype() {
        let input = single_class_with_skill("class:wizard", 1, "skill:craft_locks", 1);
        let totals = allocate_skill_ranks(&input);
        assert_eq!(totals.totals.get("skill:craft_locks").unwrap().class_skill_bonus, 3);

        let input = single_class_with_skill("class:wizard", 1, "skill:profession_scribe", 1);
        let totals = allocate_skill_ranks(&input);
        assert_eq!(totals.totals.get("skill:profession_scribe").unwrap().class_skill_bonus, 3);
    }

    #[test]
    fn trained_only_family_skills_cannot_be_attempted_at_zero_ranks() {
        // AT-34-E3-003 widens TRAINED_ONLY_SKILLS to the module doc
        // comment's own already-documented full roster: "all Knowledge
        // subtypes" and "Profession" (both families), plus Handle Animal,
        // Linguistics, Sleight of Hand, Spellcraft, Use Magic Device.
        for skill_id in [
            "skill:knowledge_arcana",
            "skill:profession_baker",
            "skill:handle_animal",
            "skill:linguistics",
            "skill:sleight_of_hand",
            "skill:spellcraft",
            "skill:use_magic_device",
        ] {
            let input = single_class_with_skill("class:wizard", 1, skill_id, 0);
            let totals = allocate_skill_ranks(&input);
            assert!(
                !totals.totals.contains_key(skill_id),
                "{skill_id} is trained-only and must not appear at 0 ranks"
            );
        }
    }

    #[test]
    fn a_non_trained_only_skill_still_reports_untrained_use_at_zero_ranks() {
        // Guards against `TRAINED_ONLY_SKILLS`'s widening accidentally
        // sweeping in a skill that should remain usable untrained --
        // Survival (Wisdom-keyed, not trained-only) still reports a raw
        // ability-modifier total at 0 ranks.
        let input = single_class_with_skill("class:wizard", 1, "skill:survival", 0);
        let totals = allocate_skill_ranks(&input);
        assert!(totals.untrained_use.contains_key("skill:survival"));
    }

    #[test]
    fn class_skill_bonus_is_grounded_actually_executes_the_engine_not_just_asserts() {
        // The classifier's own fixture-execution instrument (AT-34-E3-003):
        // a real class/skill pair returns the real, computed bonus.
        assert_eq!(
            class_skill_bonus_is_grounded("class:fighter", "skill:handle_animal"),
            Some(3)
        );
        assert_eq!(class_skill_bonus_is_grounded("class:wizard", "skill:knowledge_planes"), Some(3));
        // A real class this module recognizes, but a skill genuinely not
        // in its list, is honestly None -- never a fabricated 3.
        assert_eq!(class_skill_bonus_is_grounded("class:fighter", "skill:appraise"), None);
        // A class this module carries no data for at all is also honestly
        // None.
        assert_eq!(class_skill_bonus_is_grounded("class:cleric", "skill:heal"), None);
    }

    #[test]
    fn is_class_skill_for_matches_class_skill_bonus_is_grounded() {
        assert!(is_class_skill_for("class:fighter", "skill:handle_animal"));
        assert!(!is_class_skill_for("class:fighter", "skill:appraise"));
        assert!(!is_class_skill_for("class:cleric", "skill:heal"));
    }

    #[test]
    fn skill_bonus_is_grounded_for_display_name_normalizes_and_checks_every_recognized_class() {
        // `v06_work_inventory.rs`'s `Kind::Skill` classifier calls this with
        // the corpus record's own display name, exactly as it appears in
        // `cr_skills.lst` -- never a pre-normalized `skill:` id. "Handle
        // Animal" is a real Fighter class skill (see
        // `class_skill_bonus_is_grounded`'s own fighter/handle_animal
        // case), so the display-name entry point must reach the identical
        // real, fixture-executed answer.
        assert_eq!(
            super::skill_bonus_is_grounded_for_display_name("Handle Animal"),
            Some(3)
        );
        // A parenthesized subtype name normalizes the same way
        // `class_skill_set`'s own `TYPE=Knowledge` expansion does.
        assert_eq!(
            super::skill_bonus_is_grounded_for_display_name("Knowledge (Planes)"),
            Some(3)
        );
        // A skill no recognized class (Fighter/Rogue/Wizard) treats as a
        // class skill -- Perform is Bard-only, a class this module carries
        // no data for -- is honestly None, never a fabricated 3.
        assert_eq!(super::skill_bonus_is_grounded_for_display_name("Perform (Sing)"), None);
    }
}
