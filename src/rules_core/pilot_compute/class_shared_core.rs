#[allow(unused_imports)]
pub(crate) use super::*;

/// Result of the GE-06 pilot deterministic compute surface, accumulating the
/// base chassis, baseline combat, and total-save outputs proven across slices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotBaseChassisComputation {
    pub ability_modifiers: AbilityModifiers,
    /// Class/base attack bonus only. Zero when the chassis is unsupported.
    pub base_attack_bonus: i16,
    /// Class/base save bonuses only (no ability modifiers added to these).
    pub base_saves: BaseSaves,
    /// Baseline melee attack bonus for the deterministic Longsword loadout. Zero
    /// when the required deterministic combat posture is absent or unsupported.
    pub baseline_melee_attack_bonus: i16,
    /// Baseline armor class for the deterministic Chain Shirt / Dodge / no-shield
    /// posture. Zero when that posture is absent or unsupported.
    pub baseline_armor_class: i16,
    /// Total saving throws (Fighter base save + relevant ability modifier). Zero
    /// when the Fighter level-1 chassis is absent or unsupported.
    pub total_saves: BaseSaves,
    /// Selected deterministic Climb / Intimidate / Swim skill modifiers. All zero
    /// when the deterministic selected-skill or Chain Shirt posture is absent or
    /// widened beyond this slice.
    pub selected_skill_modifiers: SelectedSkillModifiers,
    pub explanations: Vec<ComputationExplanation>,
    pub diagnostics: Vec<ComputationDiagnostic>,
    /// SD-35 AT-35-E2-002: the "Rules and features" lines -- every held sheet rule's line
    /// (`sheet_rule::render_sheet`), one per held, printed rule. Empty from
    /// `compute_pilot_base_chassis` itself, which has no `data/sheet_rules/` package in hand;
    /// filled by [`PilotBaseChassisComputation::with_sheet_rules`] once a caller loads one
    /// (the desktop, once per process).
    pub sheet_lines: Vec<crate::rules_core::sheet_rule::SheetLine>,
}

impl PilotBaseChassisComputation {
    /// Attach the sheet lines for this computation: the held set is the seed (the character's
    /// own selections plus the class-feature records this computation grounded, plus
    /// `extra_race_traits` from the caller's race resolver) closed over the package's grants.
    pub fn with_sheet_rules(
        mut self,
        input: &CharacterInput,
        package: &crate::rules_core::sheet_rule::SheetRulePackage,
        extra_race_traits: &[String],
    ) -> Self {
        use crate::rules_core::sheet_rule::{render_sheet, CharacterFacts, HeldSeed};
        let mut seed = HeldSeed::from_character(input, &self);
        seed.race_traits.extend(extra_race_traits.iter().cloned());
        let facts = CharacterFacts::from_character(input, &self);
        self.sheet_lines = render_sheet(package, &seed, &facts);
        self
    }
}

/// Ability modifiers derived from chosen ability scores via `floor(score/2) - 5`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AbilityModifiers {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

/// Base save bonuses from the grounded class chassis row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BaseSaves {
    pub fortitude: i16,
    pub reflex: i16,
    pub will: i16,
}

/// Selected deterministic skill modifiers bounded to the GE-06 pilot slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectedSkillModifiers {
    pub climb: i16,
    pub intimidate: i16,
    pub swim: i16,
}

/// A machine-checkable record explaining why a single computed value exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputationExplanation {
    /// Stable id (e.g. `ability_modifier.strength`, `class_chassis.base_attack_bonus`).
    pub id: String,
    /// The computed value this record explains.
    pub value: i16,
    /// Human-auditable detail referencing the source input and formula.
    pub detail: String,
}

/// A diagnostic that blocks downstream claims when an input is unsupported here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputationDiagnostic {
    /// Stable id for the diagnostic subject (e.g. `class_chassis.unsupported`).
    pub id: String,
    pub message: String,
    pub claim_blocking: bool,
}

pub(super) const HYBRID_BASELINE_LEVEL: u8 = 1;

// PF1 Core Rulebook Archery combat style, 2nd-level bonus feat list.
pub(super) const FAR_SHOT_FEAT_SELECTION: &str = "feat:far_shot";

pub(super) const POINT_BLANK_SHOT_FEAT_SELECTION: &str = "feat:point_blank_shot";

pub(super) const PRECISE_SHOT_FEAT_SELECTION: &str = "feat:precise_shot";

pub(super) const RAPID_SHOT_FEAT_SELECTION: &str = "feat:rapid_shot";

// PF1 Core Rulebook Two-Weapon Combat style, 2nd-level bonus feat list.
pub(super) const DOUBLE_SLICE_FEAT_SELECTION: &str = "feat:double_slice";

pub(super) const IMPROVED_SHIELD_BASH_FEAT_SELECTION: &str = "feat:improved_shield_bash";

pub(super) const QUICK_DRAW_FEAT_SELECTION: &str = "feat:quick_draw";

pub(super) const TWO_WEAPON_FIGHTING_FEAT_SELECTION: &str = "feat:two_weapon_fighting";

pub(super) const SHOT_ON_THE_RUN_FEAT_SELECTION: &str = "feat:shot_on_the_run";

// PF1 Core Rulebook Two-Weapon Combat style, 10th-level bonus feat list.
pub(super) const GREATER_TWO_WEAPON_FIGHTING_FEAT_SELECTION: &str = "feat:greater_two_weapon_fighting";

pub(super) const TWO_WEAPON_REND_FEAT_SELECTION: &str = "feat:two_weapon_rend";

// PF1 Core Rulebook Archery combat style, 6th-level bonus feat list.
pub(super) const IMPROVED_PRECISE_SHOT_FEAT_SELECTION: &str = "feat:improved_precise_shot";

pub(super) const MANYSHOT_FEAT_SELECTION: &str = "feat:manyshot";

// PF1 Core Rulebook Two-Weapon Combat style, 6th-level bonus feat list.
pub(super) const IMPROVED_TWO_WEAPON_FIGHTING_FEAT_SELECTION: &str = "feat:improved_two_weapon_fighting";

pub(super) const TWO_WEAPON_DEFENSE_FEAT_SELECTION: &str = "feat:two_weapon_defense";

pub(super) const ARCANE_BLOODLINE_SELECTION_ID: &str = "bloodline:arcane";

pub(super) const DRACONIC_BLOODLINE_SELECTION_ID: &str = "bloodline:draconic";

pub(super) const ARCANE_BOND_FAMILIAR_SELECTION_ID: &str = "bond:familiar";

pub(super) const ARCANE_BOND_BONDED_OBJECT_SELECTION_ID: &str = "bond:bonded_object";

/// PF1 Core Rulebook Arcane Bond: "Once per day, your bonded item allows
/// you to cast any one of your spells known" -- a flat, non-level-scaled
/// daily budget (unlike Rage/Bardic Performance's rising rounds-per-day).
pub(super) const ARCANE_BOND_USES_PER_DAY: i16 = 1;

/// PF1 Core Rulebook level at which the Arcane bloodline's bonus spells
/// begin (verified via web search aggregation against the standard
/// Arcane bloodline bonus-spell table: identify at 3rd, invisibility at
/// 5th, dispel magic at 7th, etc.). Below this level, "no bonus spells or
/// bonus feats are implemented" is a correct level-gate absence, not a
/// gap; at or above it, real bonus-spell/bonus-feat grants exist in PF1
/// and stay genuinely unimplemented here, so the diagnostic must keep
/// blocking at 3rd level and above even once Arcane Bond itself is
/// recognized.
pub(super) const ARCANE_BLOODLINE_BONUS_LEVEL: u8 = 3;

/// The Arcane bloodline's nine bonus spells, as `(sorcerer level granted,
/// sorcerer spell level, spell name)`.
///
/// Transcribed verbatim from the PF1 Core Rulebook corpus record
/// `KEY:Arcane Bloodline ~ Bonus Spells` (cr_abilities_class.lst), whose nine
/// `SPELLKNOWN:CLASS|Sorcerer=<spell level>|<spell>|PREVAREQ:Sorcerer_CF_BloodlineSpell<n>,0|
/// PREVARGTEQ:BloodlineCasterLVL,<grant level>` tokens supply all three columns
/// directly. `BloodlineCasterLVL` is `SorcererLVL`
/// (`BONUS:VAR|BloodlineCasterLVL|SorcererLVL|TYPE=Base`), so the gate levels are
/// plain sorcerer levels.
///
/// Cross-checked, not merely restated: every entry's spell level is asserted
/// against this codebase's own separately-ingested
/// `sorcerer_spell_list::SORCERER_SPELL_LIST` by
/// `every_arcane_bonus_spell_resolves_on_the_real_sorcerer_spell_list`, so a
/// transcription slip in either table fails the build.
///
/// These are Arcane-bloodline-specific. Bloodrager also has an "Arcane"
/// bloodline and ten of the Sorcerer bloodline names recur there, but those are
/// entirely distinct corpus records under a different `KEY:` namespace and none
/// of their values are used here.
pub(super) const ARCANE_BLOODLINE_BONUS_SPELLS: &[(u8, u8, &str)] = &[
    (3, 1, "Identify"),
    (5, 2, "Invisibility"),
    (7, 3, "Dispel Magic"),
    (9, 4, "Dimension Door"),
    (11, 5, "Overland Flight"),
    (13, 6, "True Seeing"),
    (15, 7, "Teleport (Greater)"),
    (17, 8, "Power Word Stun"),
    (19, 9, "Wish"),
];

/// The eight feats a bloodline-feat slot may be spent on for the Arcane
/// bloodline, transcribed from the corpus record `Arcane Bloodline ~ Feat
/// Tracker` (cr_abilities_class.lst), whose
/// `BONUS:VAR|Sorcerer_BloodlineFeat_<X>|1` tokens are exactly this set.
///
/// Only the COUNT of slots is grounded as a magnitude; which feat fills a slot
/// is a player choice this seam deliberately does not model, the ratified
/// treatment already used for Fighter's, Cavalier's, and Brawler's own bonus
/// feats ("Only the count grounds; which feats are chosen is not modelled").
/// This list exists so the count's explanation can name the real eligible set
/// rather than gesturing at an unnamed one.
pub(super) const ARCANE_BLOODLINE_ELIGIBLE_BONUS_FEATS: &[&str] = &[
    "Combat Casting",
    "Improved Counterspell",
    "Improved Initiative",
    "Iron Will",
    "Scribe Scroll",
    "Skill Focus (Knowledge [arcana])",
    "Spell Focus",
    "Still Spell",
];

/// Sorcerer level at which the first bloodline bonus feat is granted — the
/// level `(BloodlineFeatProgression - 1) / 6` first reaches 1, and the first
/// level at which any of this progression's player sub-choices arises.
pub(super) const ARCANE_BLOODLINE_FIRST_BONUS_FEAT_LEVEL: u8 = 7;

/// Sorcerer level at which the Arcane bloodline's 3rd-level power, Metamagic
/// Adept, is granted (corpus `KEY:Arcane Bloodline ~ Metamagic Adept`,
/// `PREVARGTEQ:Sorcerer_Arcane_BloodlineProgressionLVL,3`).
pub(super) const ARCANE_BLOODLINE_METAMAGIC_ADEPT_LEVEL: u8 = 3;

/// Sorcerer level at which the Arcane bloodline's 9th-level power, New Arcana,
/// is granted (corpus `KEY:Arcane Bloodline ~ New Arcana`,
/// `PREVARGTEQ:Sorcerer_Arcane_BloodlineProgressionLVL,9`).
pub(super) const ARCANE_BLOODLINE_NEW_ARCANA_LEVEL: u8 = 9;

/// Sorcerer level at which the Arcane bloodline's 15th-level power, School
/// Power, is granted (corpus `KEY:Arcane Bloodline ~ School Power Choice`,
/// `PREVARGTEQ:Sorcerer_Arcane_BloodlineProgressionLVL,15`).
pub(super) const ARCANE_BLOODLINE_SCHOOL_POWER_LEVEL: u8 = 15;

/// Sorcerer level at which the Arcane bloodline's capstone, Arcane Apotheosis,
/// is granted (corpus `KEY:Arcane Bloodline ~ Arcane Apotheosis`,
/// `PREVARGTEQ:Sorcerer_Arcane_BloodlineProgressionLVL,20`).
pub(super) const ARCANE_BLOODLINE_ARCANE_APOTHEOSIS_LEVEL: u8 = 20;

/// Order of the Sword is the one canonical Order this closure grounds
/// (task #6, 2026-07-27). Chosen over the other five because its own
/// bonus is flat and self-scoped; five of the six orders' challenge
/// riders are opponent- or ally-conditioned and stay deferred.
pub(super) const ORDER_OF_THE_SWORD_SELECTION: &str = "order:sword";

/// SD-34 wave 44 (`decisions.md §22`, Piece 2 item 2): a second Order this
/// closure grounds, for the same reason Order of the Sword qualified --
/// verified directly against `apg_abilities_class.lst:243`'s own `KEY:Order
/// of the Dragon` record: its Survival-check bonus
/// (`max(1,CavalierLVL/2)`, DESC-sourced) is a flat, self-scoped magnitude,
/// exactly like Order of the Sword's Sense Motive bonus. The SAME record
/// also carries `OrderChallengeBonus|CavalierLVL/4` (a melee attack bonus
/// against the character's own challenge target) and Aid Allies'
/// `3+(CavalierLVL-2)/6` (an ally-scoped bonus, its own separate corpus
/// record) -- both stay deferred, same reason the other four orders'
/// challenge riders do: opponent- or ally-conditioned, not this
/// character's own unconditional roll.
pub(super) const ORDER_OF_THE_DRAGON_SELECTION: &str = "order:dragon";

pub(super) const FERAL_MUTAGEN_DISCOVERY_SELECTION: &str = "discovery:feral_mutagen";

/// Feral Mutagen's Medium-size natural-attack damage dice, corpus-verbatim.
pub(super) const FERAL_MUTAGEN_CLAW_DAMAGE_DIE: i16 = 6;

pub(super) const FERAL_MUTAGEN_BITE_DAMAGE_DIE: i16 = 8;

/// Feral Mutagen's competence bonus on Intimidate while mutated.
pub(super) const FERAL_MUTAGEN_INTIMIDATE_BONUS: i16 = 2;

/// The one canonical extract this codebase seeds into an Alchemist's and
/// an Investigator's formula book at creation time -- the shared
/// `alchemist_spell_list` records it at extract level 1
/// (`("Cure Light Wounds", 1)`), which is the only extract level either
/// class can access at class level 1, so a single id works across the
/// whole 1-20 sweep for both. Mirrors `WIZARD_STARTER_SPELL_ID`'s own
/// bootstrap shape exactly.
///
/// `allow(dead_code)`: the production consumer is `compose_character_input`
/// in `apps/desktop/src-tauri`, a SEPARATE cargo workspace that mirrors
/// its own copy of every seed constant (see `WIZARD_STARTER_SPELL_ID`'s
/// own mirrored copy there), so nothing in this crate's non-test build
/// reads it. Declared here anyway because this is where the corpus
/// verification for the value belongs.
#[allow(dead_code)]
pub(crate) const CANONICAL_EXTRACT_SPELL_ID: &str = "Cure Light Wounds";

/// Destruction Blessing is the one canonical Blessing this closure
/// grounds -- see this const's own containing doc comment above for why.
pub(super) const DESTRUCTION_BLESSING_SELECTION: &str = "blessing:destruction";

/// Strength Blessing is the second canonical Blessing this partial
/// re-scope grounds (deepening 2026-07-26, task #9). Chosen over the
/// other 18 because its minor power, Strength Surge, is the only one
/// that is BOTH a flat-magnitude self-buff AND natively self-targeted
/// ("As a swift action you can focus your own strength" -- verified
/// directly against `acg_abilities_class.lst`'s own
/// `KEY:Strength Blessing ~ Strength Surge` DESC), so unlike Destructive
/// Attacks it needs no "self-application only" narrowing at all.
pub(super) const STRENGTH_BLESSING_SELECTION: &str = "blessing:strength";

/// v0.6 alpha swarm, risks item 8 (Slayer full-build closure, seventh
/// ACG/APG class-specific closure): APG/ACG Slayer, verified directly
/// against `acg_classes.lst`'s own confirmed non-caster status (no
/// `SPELLSTAT` token at all) and `acg_abilities_class.lst`'s own
/// `KEY:Slayer ~ Sneak Attack`/`Trap Sense`/`Trapfinding`/`Track`/`Class
/// Skills` records. Zero spellcasting scope, unlike every prior closure
/// this session except Cavalier/Brawler/Hunter. Grounds four real, flat
/// formulas as standalone explanation records with no further total-
/// integration -- an already-established idiom, not a new compromise:
/// Barbarian's own `class_feature.barbarian.trap_sense` and Rogue's own
/// `class_feature.rogue.trap_sense` are both already grounded the same
/// way. Studied Target (this class's own real name for its marquee
/// feature -- there is no separate "Quarry" record) stays confirmed
/// opponent-dependent, the same "no target-creature representation
/// exists anywhere in this codebase" wall that already excluded it from
/// the single-ability scan. Also fixes a real, independently-confirmed
/// bug, the THIRD class needing this exact widening: Slayer's own
/// class-skill list genuinely includes Climb/Intimidate/Swim (like
/// Warpriest, unlike Wizard/Arcanist), so
/// `selected_skill_class_skill_bonus_applies` needed real widening
/// again. See `docs/release/v0.6/second-full-class-build-comparative-scoping.md`
/// for the full corpus verification and scope record.
pub(super) const SLAYER_CLASS_ID: &str = "class:slayer";

/// The choice set naming which Slayer Talent was taken. Talents are a
/// chooser whose entire value IS the choice, so this follows the
/// ratified no-silent-seeding design: ground a talent only when it is
/// explicitly recorded, never seed a canonical one.
pub(super) const SLAYER_TALENT_CHOICE_ID: &str = "choice:slayer_talent";

/// Foil Scrutiny is the one canonical talent this closure grounds,
/// narrowed the same way Order of the Sword and Animal Focus's Bull
/// were. Chosen because its `BONUS:SKILL|Bluff,Disguise|2` is the
/// cleanest flat, self-scoped magnitude among the 41 talent records --
/// most of the rest grant other abilities by reference (Combat Style,
/// Combat Trick, Weapon Training) rather than carrying a magnitude.
pub(super) const SLAYER_TALENT_FOIL_SCRUTINY_SELECTION: &str = "talent:foil_scrutiny";

/// Foil Scrutiny's bonus on Bluff and Disguise checks made to avoid
/// notice.
pub(super) const SLAYER_FOIL_SCRUTINY_BONUS: i16 = 2;

/// `Empower Spell` (namespaced as `metamagic:empower_spell` -- see this
/// const's own value and `arcanist_metamagic_knowledge_feat_name`'s doc
/// comment for why) is the canonical, proven example this closure's own
/// tests exercise -- but unlike Destruction Blessing's own hardcoded
/// single-value recognition (where every OTHER Blessing type genuinely
/// lacks any built minor power to check), `ground_or_block_arcanist_metamagic_knowledge`
/// deliberately validates WHATEVER real metamagic feat the choice names,
/// not only this one literal. This is a conscious difference, not an
/// oversight: `feat_prereqs::metamagic::evaluate_metamagic_feat_prerequisites`
/// already validates every real CRB Metamagic-category feat with equal
/// confidence (bounded to catalog membership either way, no per-feat
/// distinction in what's "more built" for one feat over another), so
/// artificially restricting recognition to only `Empower Spell` would be
/// a narrower, less honest claim than what the reused module already
/// proves for the whole category.
///
/// **Real bug found and fixed (2026-07-25), risks-and-open-questions.md
/// item 23-family regression**: this value used to be the bare literal
/// `"Empower Spell"` (zero colons), which the compute engine itself
/// accepted fine but `saved_character::local_store::validate_character_input`
/// genuinely rejects at save time -- every `selected_choices` entry's
/// `selection_id` must carry at least one colon to round-trip through the
/// fixture grammar. Frontend caught this via real live-testing (a fresh
/// Human Arcanist 1 hit a raw save-time error, not a graceful `Blocked`
/// diagnostic) before shipping the `CLASS_OPTIONS` entry. A naive rename
/// to any colon-satisfying string would have silently broken
/// `evaluate_metamagic_feat_prerequisites`'s own real catalog lookup
/// (which matches directly against the literal feat name) -- caught
/// before it shipped. The real fix is the translation layer below:
/// `arcanist_metamagic_knowledge_feat_name` strips the `metamagic:`
/// namespace and reconstructs the literal Title Case feat name generically
/// (every real CRB Metamagic feat is a simple "Word Spell" phrase), so
/// this constant is now the namespaced seed value, translated back to
/// `"Empower Spell"` before ever reaching the feat catalog.
//
// Named only by test fixtures (production code reconstructs the feat name
// generically via `arcanist_metamagic_knowledge_feat_name`'s slug transform,
// never by this literal) -- `#[cfg(test)]`, the real fix, not `#[allow]`.
#[cfg(test)]
pub(super) const EMPOWER_SPELL_METAMAGIC_SELECTION: &str = "metamagic:empower_spell";

/// Life Mystery is the one canonical Mystery this closure grounds -- its
/// own Healing Hands revelation is the cleanest flat, self-scoped value
/// of the 10 real Mystery types checked.
pub(super) const LIFE_MYSTERY_SELECTION: &str = "mystery:life";

/// Clouded Vision is the one canonical Curse this closure grounds -- a
/// genuinely self-contained, flat, no-target-creature restriction-plus-
/// benefit pair, unlike Deaf/Haunted/Lame/Wasting.
pub(super) const CLOUDED_VISION_CURSE_SELECTION: &str = "curse:clouded_vision";

/// The four additional Mysteries this deepening recognizes, alongside
/// the already-shipped `LIFE_MYSTERY_SELECTION`.
pub(super) const LORE_MYSTERY_SELECTION: &str = "mystery:lore";

pub(super) const NATURE_MYSTERY_SELECTION: &str = "mystery:nature";

pub(super) const BONE_MYSTERY_SELECTION: &str = "mystery:bone";

pub(super) const FLAME_MYSTERY_SELECTION: &str = "mystery:flame";

/// SD31-E4-F2-001: Battle Mystery, the seventh Mystery this deepening
/// grounds and the first wired through the new
/// `archetype_resolver::chooser_option_selected` primitive rather than the
/// hand-rolled `oracle_level_with_revelation` shape the other six use --
/// picked as this cycle's representative pool (mirrors the standing
/// "ground one representative option per pool honestly" ruling) because
/// its own tier-1 Battlecry revelation is entirely flat, self-scoped
/// magnitudes, no dice roll, matching the same shape every already-
/// grounded revelation above requires.
pub(super) const BATTLE_MYSTERY_SELECTION: &str = "mystery:battle";

/// SD31-E4-F2-002: the 4 remaining Mysteries `SD31-E4-F2-001`'s own
/// followup named (`kanban.md` "Wave 10 integration status" /
/// `OPEN-ISSUES.md` row 168) -- Stone, Waves, Wind, Heavens -- wired
/// through the same `archetype_resolver::chooser_option_selected`
/// primitive Battle Mystery proved. Each grounds one representative
/// tier-1 revelation with a genuinely flat, self-scoped, non-target-
/// dependent magnitude, matching the standing "ground one representative
/// option per pool honestly" ruling.
pub(super) const STONE_MYSTERY_SELECTION: &str = "mystery:stone";

pub(super) const WAVES_MYSTERY_SELECTION: &str = "mystery:waves";

pub(super) const WIND_MYSTERY_SELECTION: &str = "mystery:wind";

pub(super) const HEAVENS_MYSTERY_SELECTION: &str = "mystery:heavens";

/// The three additional Curse types this deepening grounds (2026-07-26,
/// task #10). An Oracle selects exactly ONE curse, so these are mutually
/// exclusive with each other and with Clouded Vision -- no revelation-
/// budget question arises here, unlike the Mystery revelations.
pub(super) const LAME_CURSE_SELECTION: &str = "curse:lame";

pub(super) const WASTING_CURSE_SELECTION: &str = "curse:wasting";

pub(super) const DEAF_CURSE_SELECTION: &str = "curse:deaf";

/// The choice set naming which familiar species a spellcaster bonded
/// with (task #11 Tier 0, 2026-07-27). Shared by Witch and Shaman: both
/// corpus records delegate to the same `Standard Familiar List`, so this
/// closure follows the corpus rather than RAW's spirit-linked reading
/// (lead ruling), and one implementation closes the familiar slot for
/// both classes.
pub(super) const FAMILIAR_CHOICE_ID: &str = "choice:familiar_species";

pub(super) const FAMILIAR_TOAD_SELECTION: &str = "familiar:toad";

/// Ward is the one canonical Hex this closure grounds -- the cleanest of
/// the ~19 base hexes checked: a flat, self-scoped deflection/resistance
/// bonus with no opponent/save-DC interaction (most other hexes --
/// Evil Eye, Misfortune, Slumber, Cackle, Fortune -- are opponent/ally-
/// targeted, the same "opponent-dependent" wall Slayer's Studied Target
/// already hit).
pub(super) const WARD_HEX_SELECTION: &str = "hex:ward";

/// The two hexes carrying a magnitude distinct from the shared hex save
/// DC (task #11, 2026-07-27). Every other one of the 53 hex records --
/// minor, major and grand -- carries only `WitchHexDC_<Name>|WitchHexDC`,
/// a per-hex ALIAS of one shared variable, so they are facets of a
/// single DC mechanism rather than 27 separate magnitudes.
pub(super) const CAULDRON_HEX_SELECTION: &str = "hex:cauldron";

pub(super) const FLIGHT_HEX_SELECTION: &str = "hex:flight";

/// Life Spirit is the one canonical Spirit this closure grounds -- its
/// own immediately-available Channel ability is the cleanest flat,
/// self-scoped value of the 10 real Spirit types checked (the same 10
/// primary spirits as Oracle's own 10 Mysteries).
pub(super) const LIFE_SPIRIT_SELECTION: &str = "spirit:life";

/// The other nine primary Spirits, each grounded through its own
/// immediately-available (ungated) base ability -- see
/// `ground_shaman_spirit_base_ability`. Verified uniform across the
/// corpus: every one of the ten `KEY:Shaman Spirit ~ <Name>` records
/// grants exactly four abilities, of which exactly ONE carries no
/// `PREVARGTEQ` gate. The other three are gated at
/// `ShamanSpiritGreater` (`PRECLASS:1,Shaman=8`), `ShamanSpiritTrue`
/// (`PRECLASS:1,Shaman=16`), and `Shaman Manifestation` (the capstone),
/// all of which stay deferred.
pub(super) const BATTLE_SPIRIT_SELECTION: &str = "spirit:battle";

pub(super) const BONES_SPIRIT_SELECTION: &str = "spirit:bones";

pub(super) const FLAME_SPIRIT_SELECTION: &str = "spirit:flame";

pub(super) const HEAVENS_SPIRIT_SELECTION: &str = "spirit:heavens";

pub(super) const LORE_SPIRIT_SELECTION: &str = "spirit:lore";

pub(super) const NATURE_SPIRIT_SELECTION: &str = "spirit:nature";

pub(super) const STONE_SPIRIT_SELECTION: &str = "spirit:stone";

pub(super) const WAVES_SPIRIT_SELECTION: &str = "spirit:waves";

pub(super) const WIND_SPIRIT_SELECTION: &str = "spirit:wind";

// SD13-E5 Fascinate flat DC base. PF1 Core Rulebook Fascinate Will save DC is
// 10 + 1/2 bard level + Charisma modifier; only the fixed base term is a named
// constant, since the level and Charisma terms are already grounded elsewhere.
pub(super) const FASCINATE_DC_BASE: i16 = 10;

pub(super) const EVOCATION_SCHOOL_SELECTION: &str = "school:evocation";

pub(super) const TRANSMUTATION_SCHOOL_SELECTION: &str = "school:transmutation";

// Task #66 Abjuration slice: a second canonical deterministic school
// selection, alongside the pre-existing Evocation triple above. A wizard
// specializes in exactly one school, so this and
// `wizard_has_canonical_specialization_selections` are mutually exclusive
// on any single input. Necromancy/Transmutation remain legal opposed
// schools regardless of which school is the specialty (PF1's only
// opposition restriction is "not your own specialty school and not
// Divination"), so the same two opposed-school constants are reused rather
// than duplicated.
pub(super) const ABJURATION_SCHOOL_SELECTION: &str = "school:abjuration";

// `AT-34-E3-001` (mechanism 2 continuation, cycle 7): a fourth canonical
// deterministic school selection, alongside Evocation, Abjuration, and
// Transmutation above. Conjuration/Divination remain legal opposed schools
// for a Conjuration specialist (PF1's only opposition restriction is "not
// your own specialty school and not Divination" -- so a Conjuration
// specialist may not oppose Divination, but nothing bars using it for a
// DIFFERENT specialist's own two opposed slots); this reuses the same
// idiom as the Transmutation slice, picking two already-existing
// non-Conjuration, non-Divination selection constants.
pub(super) const CONJURATION_SCHOOL_SELECTION: &str = "school:conjuration";

// `AT-34-E3-001` (mechanism 2 continuation, cycle 8): the "no specialization"
// arm, distinct in shape from every specialist selection above. A wizard who
// declines to specialize gains the Universal School's own two powers
// (`cr_abilities_class.lst`'s `KEY:Universal School ~ *` records) instead of
// a specialist school's powers, and PF1's own rule is that a universalist
// "need not select an opposition school" -- unlike every specialist arm
// above, whose canonical fixture always carries exactly two opposed-school
// selections, the canonical universalist fixture carries ZERO.
pub(super) const UNIVERSAL_SCHOOL_SELECTION: &str = "school:universal";

/// SD31-E4-F2-003: the real, corpus-declared Core Rulebook Rage Power pool
/// `chooser_option_selected` validates every Rage Power selection against --
/// every `KEY:Rage Power ~ <X>` row `core_rulebook/cr_abilities_class.lst`
/// declares, grep-verified: `grep -oP '(?<=KEY:Rage Power ~ )[^\t]+'
/// cr_abilities_class.lst | sort -u` -> exactly these 28. Scoped to the
/// Core Rulebook's own base Rage Power list only -- the wider ~60-record
/// corpus-wide family (ACG/APG/UC/UW/AG/HA/UI/player-companion additions,
/// e.g. Skald's/Bloodrager's own copies) stays named-but-unproven, matching
/// the standing "ground one representative pool, transcribed verbatim"
/// ruling this program has applied to every other option-pool closure
/// (`ORACLE_MYSTERY_POOL`, `ORACLE_BATTLE_MYSTERY_REVELATION_POOL`, ...).
/// Transcribed verbatim from the pinned oracle, never generated or
/// inferred.
///
/// **Bare slugs, no prefix** -- `src/bin/v06_work_inventory.rs`'s own
/// `CLASS_FEATURE_POOLS` registry declares `("Rage Power", "barbarian",
/// "choice:barbarian_rage_power", "")`: an EMPTY namespace column, whose own
/// doc comment states "An empty namespace means the consumer is open-ended
/// (it echoes whatever raw string it is given)". A `"rage_power:"`-prefixed
/// id would silently mismatch every real selection this engine's own
/// `--class-feature-probe` diagnostic and the production `class_feature`
/// classify() path both generate (`class_feature_engine_join_slug(member)`,
/// no namespace prepended) -- checked directly against that file rather
/// than assumed, after an earlier draft of this pool used a
/// `"rage_power:"` prefix and the probe reported `no_consumer_delta`
/// instead of `wired` as a direct result.
pub(super) const CORE_RULEBOOK_RAGE_POWER_POOL: &[&str] = &[
    "animal_fury",
    "clear_mind",
    "fearless_rage",
    "guarded_stance",
    "increased_damage_reduction",
    "internal_fortitude",
    "intimidating_glare",
    "knockback",
    "low_light_vision",
    "mighty_swing",
    "moment_of_clarity",
    "night_vision",
    "no_escape",
    "powerful_blow",
    "quick_reflexes",
    "raging_climber",
    "raging_leaper",
    "raging_swimmer",
    "renewed_vigor",
    "rolling_dodge",
    "roused_anger",
    "scent",
    "strength_surge",
    "superstition",
    "surprise_accuracy",
    "swift_foot",
    "terrifying_howl",
    "unexpected_strike",
];

/// Superstition is this cycle's representative Rage Power (mirrors the
/// standing "ground one representative option per pool honestly" ruling
/// Battle Mystery/Ward Hex/Life Spirit each already applied). Its own
/// `KEY:Rage Power ~ Superstition`, `BONUS:VAR|SuperstitionSaveBonus|
/// 2+RagePowersLVL/4` token carries NO `PREVARGTEQ:Raging,1`/
/// `PREVAREQ:Raging,1` gate anywhere on that line (checked directly against
/// the full raw row, field by field -- unlike Raging Climber/Raging
/// Swimmer's or Witch's Ward hex's own explicit Raging-state gates), so
/// this closure follows the corpus token literally: the bonus grounds once
/// the power is selected, not only while actively raging. `RagePowersLVL`
/// is `BONUS:VAR|RagePowersLVL|BarbarianLVL` on the base Barbarian's own
/// `Rage Powers` internal record (`cr_abilities_class.lst`), so the formula
/// is `2 + BarbarianLVL/4` for the base class this closure scopes to.
pub(super) const SUPERSTITION_RAGE_POWER_SELECTION: &str = "superstition";

/// The base value Superstition's own `DEFINE:SuperstitionSaveBonus|0`
/// formula adds to, before the level term.
pub(super) const SUPERSTITION_SAVE_BONUS_BASE: i16 = 2;

/// `RagePowersLVL / 4`'s divisor, straight from the corpus formula.
pub(super) const SUPERSTITION_SAVE_BONUS_LEVEL_DIVISOR: i16 = 4;

/// The real, corpus-declared Pathfinder Unchained Rage Power pool -- every
/// `KEY:Unchained Rage Power ~ <X>` row `pathfinder_unchained/pu_abilities_
/// class.lst` declares, transcribed verbatim (`grep -oP '(?<=KEY:Unchained
/// Rage Power ~ )[^\t]+' pu_abilities_class.lst | sort -u` -> exactly these
/// 54; matches `SD31-E4-F2-003`'s own "the 54 Unchained Rage Powers" figure).
/// A DIFFERENT, non-overlapping 54-member pool from
/// [`CORE_RULEBOOK_RAGE_POWER_POOL`]'s 28 -- Unchained Barbarian's own book
/// republishes its Rage Powers under this book's own `Unchained Rage Power`
/// category rather than reusing the base class's `Rage Power` one (each row's
/// own `SERVESAS:ABILITY=Special Ability|Rage Power ~ <X>` token names the
/// base-class ability it stands in for, never claims to BE it). Slugged by
/// the identical `class_feature_engine_join_slug` transform
/// `v06_work_inventory.rs`'s probe and `classify()` both use (alphanumeric
/// lowercased, apostrophes swallowed, everything else one underscore).
pub(super) const UNCHAINED_RAGE_POWER_POOL: &[&str] = &[
    "accurate_stance",
    "animal_fury",
    "auspicious_mark",
    "bleeding_blow",
    "calm_stance",
    "clear_mind",
    "crippling_blow",
    "deadly_accuracy",
    "eater_of_magic",
    "elemental_stance",
    "energy_absorption",
    "energy_resistance",
    "fearless_rage",
    "flesh_wound",
    "ground_breaker",
    "ground_breaker_greater",
    "guarded_stance",
    "increased_damage_reduction",
    "inspire_ferocity",
    "internal_fortitude",
    "intimidating_glare",
    "knockback",
    "knockdown_stance",
    "lethal_accuracy",
    "low_light_vision",
    "mighty_swing",
    "night_vision",
    "no_escape",
    "perfect_clarity",
    "powerful_stance",
    "protect_vitals",
    "quick_reflexes",
    "raging_climber",
    "raging_leaper",
    "raging_swimmer",
    "reckless_stance",
    "reflexive_dodge",
    "regenerative_stance",
    "renewed_vigor",
    "renewed_vitality",
    "roused_anger",
    "scent",
    "sharpened_accuracy",
    "shove_aside",
    "shove_aside_greater",
    "smasher",
    "sprint",
    "strength_stance",
    "superstition",
    "swift_foot",
    "taunting_stance",
    "terrifying_howl",
    "unexpected_strike",
    "witch_hunter",
];

/// Unchained Barbarian's own representative Rage Power selection, the same
/// real name as the base class's ([`SUPERSTITION_RAGE_POWER_SELECTION`]) --
/// both are the identical PF1 Superstition ability, just declared under two
/// separate corpus categories (`Rage Power` vs `Unchained Rage Power`).
pub(super) const UNCHAINED_SUPERSTITION_RAGE_POWER_SELECTION: &str = "superstition";

/// The character's full feat set as every `feat_effects` producer should
/// see it: everything the player chose, plus everything their classes
/// granted automatically (task #20, 2026-07-27).
///
/// Chosen feats pass through VERBATIM including duplicates, because
/// repeatable `STACK:YES MULT:YES` feats are counted by occurrence -- a
/// whole-list dedup here would silently halve a real bonus.
pub(super) fn effective_character_feats(input: &CharacterInput) -> Vec<String> {
    crate::rules_core::feat_effects::effective_feats(
        &input.chosen.selected_feats,
        &class_granted_feats(input),
    )
}

pub(super) const CATCH_OFF_GUARD_FEAT_SELECTION: &str = "feat:catch_off_guard";

pub(super) const DEFLECT_ARROWS_FEAT_SELECTION: &str = "feat:deflect_arrows";

pub(super) const IMPROVED_GRAPPLE_FEAT_SELECTION: &str = "feat:improved_grapple";

pub(super) const SCORPION_STYLE_FEAT_SELECTION: &str = "feat:scorpion_style";

pub(super) const THROW_ANYTHING_FEAT_SELECTION: &str = "feat:throw_anything";

/// `AT-34-E3-001` owner-matched cycle 6: Assassin's and Shadowdancer's own
/// class ids, needed only by `ground_class_weapon_and_armor_proficiency`'s
/// per-class dispatch below -- neither prestige class has any chassis
/// wired anywhere else in this file (`§13559`'s own dispatched-and-
/// documented finding: neither is a registered `ClassId`-family enum
/// member, so no chassis dispatch can reach them), and this record's own
/// `has_class` check reads only `CharacterClassLevel.class_id`, a flat
/// `String` field with no enum-membership precondition.
pub(super) const ASSASSIN_CLASS_ID: &str = "class:assassin";

pub(super) const SHADOWDANCER_CLASS_ID: &str = "class:shadowdancer";

/// SD-34 wave 43 (`decisions.md §22`'s 12-unit "small-precedented-new-
/// compute" remainder): Duelist's and Loremaster's own class ids, needed by
/// `ground_duelist_class_features`/`ground_loremaster_class_features` below
/// -- neither is a registered `ClassId`-family enum member either (the same
/// "real prestige class, no chassis dispatch reaches it" gap the comment
/// above already names for Assassin/Shadowdancer).
pub(super) const DUELIST_CLASS_ID: &str = "class:duelist";

pub(super) const LOREMASTER_CLASS_ID: &str = "class:loremaster";

/// SD-34 wave 44 (`decisions.md §22`, Piece 2 item 3): Pathfinder Delver's
/// own class id, needed by `ground_pathfinder_delver_class_features` below
/// -- the same "real prestige class, no `ClassId`-family enum entry, no
/// chassis dispatch reaches it" gap as Duelist/Loremaster/Assassin/
/// Shadowdancer above (confirmed directly: zero hits for
/// `"Pathfinder Delver"`/`"PathfinderDelver"` anywhere in this file before
/// this wave).
pub(super) const PATHFINDER_DELVER_CLASS_ID: &str = "class:pathfinder_delver";

/// SD-34 wave 45 (`decisions.md §22`'s WAVE 45 UPDATE, sub-mechanism-5's
/// "registered prestige class, magnitude-only" remainder): Phrenic Slayer's
/// own class id, needed by `ground_phrenic_slayer_class_features` below --
/// the same "real prestige class, registered in `prestige_class_entry_gate`
/// (source book `ultimate_psionics`, not `core_rulebook`), no `ClassId`-
/// family enum entry, no chassis dispatch reaches it" gap as Pathfinder
/// Delver above.
pub(super) const PHRENIC_SLAYER_CLASS_ID: &str = "class:phrenic_slayer";

/// SD-34 wave 46 (`decisions.md §22`'s WAVE 46 UPDATE): the class ids
/// `ground_pathfinder_delver_class_features`'s six-unit extension and the
/// five new `ground_<class>_class_features` functions below need -- the
/// SAME "real prestige class, registered in `prestige_class_entry_gate`,
/// no `ClassId`-family enum entry, no chassis dispatch reaches it" gap as
/// Pathfinder Delver/Phrenic Slayer above. Every one matches its own entry
/// in `tests/fixtures/rules_core/prestige-class-entry-requirements.json`
/// exactly (`class:<slug>`).
pub(super) const ARGENT_DRAMATURGE_CLASS_ID: &str = "class:argent_dramaturge";

pub(super) const HORIZON_WALKER_CLASS_ID: &str = "class:horizon_walker";

pub(super) const NATURE_WARDEN_CLASS_ID: &str = "class:nature_warden";

pub(super) const RAGE_PROPHET_CLASS_ID: &str = "class:rage_prophet";

pub(super) const HOLY_VINDICATOR_CLASS_ID: &str = "class:holy_vindicator";

pub(super) const STALWART_DEFENDER_CLASS_ID: &str = "class:stalwart_defender";

pub(super) const GOLDEN_LEGIONNAIRE_CLASS_ID: &str = "class:golden_legionnaire";

/// SD-34 wave 48: Twilight Talon's Enhanced Tattoo grants one spell-like
/// ability per tier reached (2nd/4th/6th/8th/10th level) -- each tier a
/// genuine `ABILITYPOOL`-gated one-of-two choice, verified directly against
/// the real, non-ingested PCGen oracle (`ag_abilities_class.lst:542`'s own
/// `BONUS:ABILITYPOOL|Twilight Talon Tattoo Level <N>|1|
/// PREVARGTEQ:TwilightTalonLVL,<N>` tokens) -- the same "real pool
/// selection this engine must gate on, not assume" shape
/// `DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID` established (wave 47's
/// own correction). Five separate choice axes, one per tier, since a
/// character accumulates a NEW tattoo at each tier reached over their
/// career rather than picking once for the whole class (unlike Divine
/// Scion's single Domain Specialization pick).
pub(super) const TWILIGHT_TALON_TATTOO_LEVEL_2_CHOICE_ID: &str = "choice:twilight_talon_tattoo_level_2";

pub(super) const TWILIGHT_TALON_TATTOO_LEVEL_4_CHOICE_ID: &str = "choice:twilight_talon_tattoo_level_4";

pub(super) const TWILIGHT_TALON_TATTOO_LEVEL_6_CHOICE_ID: &str = "choice:twilight_talon_tattoo_level_6";

pub(super) const TWILIGHT_TALON_TATTOO_LEVEL_8_CHOICE_ID: &str = "choice:twilight_talon_tattoo_level_8";

pub(super) const TWILIGHT_TALON_TATTOO_LEVEL_10_CHOICE_ID: &str = "choice:twilight_talon_tattoo_level_10";

/// SD-34 wave 49 (`decisions.md §22`'s WAVE 49 UPDATE): 33 more registered
/// prestige classes in the same "no `ClassId` enum entry, magnitude-only"
/// remainder as every class above -- one raw string const per class,
/// matched against `CharacterClassLevel::class_id` exactly like every
/// other class in this family. Matches `tests/fixtures/rules_core/
/// prestige-class-entry-requirements.json`'s own entries exactly.
pub(super) const CYPHERMAGE_CLASS_ID: &str = "class:cyphermage";

pub(super) const PSYCHIC_FIST_CLASS_ID: &str = "class:psychic_fist";

pub(super) const ASAVIR_CLASS_ID: &str = "class:asavir";

pub(super) const METAMORPH_CLASS_ID: &str = "class:metamorph";

pub(super) const WAR_MIND_CLASS_ID: &str = "class:war_mind";

pub(super) const HELLKNIGHT_CLASS_ID: &str = "class:hellknight";

pub(super) const ADAPTIVE_WARRIOR_CLASS_ID: &str = "class:adaptive_warrior";

pub(super) const SANGUINE_ANGEL_CLASS_ID: &str = "class:sanguine_angel";

pub(super) const BODY_SNATCHER_CLASS_ID: &str = "class:body_snatcher";

pub(super) const STEEL_FALCON_CLASS_ID: &str = "class:steel_falcon";

pub(super) const LANTERN_BEARER_CLASS_ID: &str = "class:lantern_bearer";

pub(super) const STORM_KINDLER_CLASS_ID: &str = "class:storm_kindler";

pub(super) const WESTCROWN_DEVIL_CLASS_ID: &str = "class:westcrown_devil";

pub(super) const PYROKINETICIST_CLASS_ID: &str = "class:pyrokineticist";

pub(super) const ASPIS_AGENT_CLASS_ID: &str = "class:aspis_agent";

pub(super) const GRAY_CORSAIR_CLASS_ID: &str = "class:gray_corsair";

pub(super) const PATHFINDER_SAVANT_CLASS_ID: &str = "class:pathfinder_savant";

pub(super) const RIVETHUN_EMISSARY_CLASS_ID: &str = "class:rivethun_emissary";

pub(super) const STUDENT_OF_WAR_CLASS_ID: &str = "class:student_of_war";

pub(super) const DIABOLIST_CLASS_ID: &str = "class:diabolist";

pub(super) const LION_BLADE_CLASS_ID: &str = "class:lion_blade";

pub(super) const BELLFLOWER_TILLER_CLASS_ID: &str = "class:bellflower_tiller";

pub(super) const HELLKNIGHT_SIGNIFER_CLASS_ID: &str = "class:hellknight_signifer";

pub(super) const MYSTIC_ARCHER_CLASS_ID: &str = "class:mystic_archer";

pub(super) const MAMMOTH_RIDER_CLASS_ID: &str = "class:mammoth_rider";

pub(super) const DEMONIAC_CLASS_ID: &str = "class:demoniac";

pub(super) const MASTER_CHYMIST_CLASS_ID: &str = "class:master_chymist";

pub(super) const ENCHANTING_COURTESAN_CLASS_ID: &str = "class:enchanting_courtesan";

pub(super) const DARK_TEMPEST_CLASS_ID: &str = "class:dark_tempest";

pub(super) const BATTLE_HERALD_CLASS_ID: &str = "class:battle_herald";

pub(super) const MASTER_SPY_CLASS_ID: &str = "class:master_spy";

pub(super) const EVANGELIST_CLASS_ID: &str = "class:evangelist";

pub(super) const ULFEN_GUARD_CLASS_ID: &str = "class:ulfen_guard";

pub const GOOD_DOMAIN_SELECTION: &str = "domain:good";

pub(super) const HEALING_DOMAIN_SELECTION: &str = "domain:healing";

// SD-31 wave 25 (OPERATOR-RULINGS-2026-08-21.md section 20): two further
// domains grounded for real via `domain_power`'s interpreted magnitude
// formula, both self-application-safe (a beneficial touch power, the same
// shape Good's own Touch of Good already grounds) -- see
// `domain_power::DOMAIN_POWER_CATALOG`'s own doc comment for why Evil/
// Darkness/Madness (the same formula shape, but an enemy-facing debuff) stay
// unground.
pub const WAR_DOMAIN_SELECTION: &str = "domain:war";

pub const STRENGTH_DOMAIN_SELECTION: &str = "domain:strength";

// SD-31 wave 26 (OPERATOR-RULINGS-2026-08-21.md section 20, "PROVE BEFORE YOU
// EXTEND" satisfied first against Good/Healing before either was added -- see
// `domain_power`'s own `fixture_check_tests` module doc): two more domains,
// both scanned corpus-wide for the same self-application-safe shape (a
// beneficial effect on a touched/self target, never an enemy-facing debuff)
// Good/War/Strength already establish.
pub const DESTRUCTION_DOMAIN_SELECTION: &str = "domain:destruction";

pub const GLORY_DOMAIN_SELECTION: &str = "domain:glory";

// SD-34 wave 37 lane A (bucket D's "domain-vs-class_feature dual-
// representation" mechanism gap, `wave36_laneC_creature_type_collision_
// disposition_cycle_receipt.md` next-cycle plan item 5): Undead Subdomain's
// Death's Kiss, the first APG SUBDOMAIN (not a base CRB domain) ever added to
// this catalog. Confirmed a real, legal Cleric/Inquisitor subdomain by direct
// corpus read (`data/corpus/advanced_players_guide/domain/undead_subdomain.json`'s
// own `PREMULT` gate: `[PREDOMAIN:1,Undead Subdomain],[PREVARLT:DeathDomain,1]`
// -- selectable once Death Domain is available, the same substitution shape
// every APG subdomain uses). Unlike every domain this catalog already grounds,
// Death's Kiss's own `DESC` formula slot is NOT a flat combat/skill/save
// bonus -- it is the power's EFFECT DURATION in rounds ("touched creatures are
// treated as undead ... for %1 rounds"), so `DOMAIN_POWER_CATALOG`'s shared
// "a +{magnitude} {label} {duration}" sentence would misrepresent a round
// count as a game bonus. `DomainPowerSpec::grounds_self_application` (added
// alongside this entry) is `false` here, so this spec's `magnitude_formula`
// is transcribed and byte/parse-verified by this module's own
// `fixture_check_tests` but never surfaced as a "self_application"/
// "not_active" explanation -- only its real, honestly-labeled uses-per-day
// (the shared, corpus-proven `3+WIS` chain, see below) is ever computed and
// reported, exactly the same discipline Good's own doc comment uses for
// declining to model granting a bonus to another creature: refuse to claim
// what would be a plausible-looking but wrong number, rather than fabricate
// one.
pub const UNDEAD_SUBDOMAIN_SELECTION: &str = "domain:undead_subdomain";

// SD-34 wave 38 lane A (wave 37 lane A's own next-cycle plan item 1): the
// second APG SUBDOMAIN this catalog grounds. Confirmed a real, legal
// Cleric/Inquisitor subdomain by direct corpus read
// (`data/corpus/advanced_players_guide/domain/construct_subdomain.json`'s
// own `PREMULT` gate: `[PREDOMAIN:1,Construct Subdomain],
// [PREVARLT:ArtificeDomain,1]` -- selectable once Artifice Domain is
// available, the same substitution shape Undead Subdomain uses; Inquisitor
// legality confirmed via `inquisitor_domains.json`'s own
// `DEFINE:InquisitorDomainConstructSubdomain|0` token). Unlike Undead
// Subdomain, whose own `DESC` formula slot is a real bonus-shaped number
// (merely the WRONG shape, an effect duration, to be a self-application
// bonus), Construct Subdomain's own granted power (Animate Servant) has no
// bonus-shaped effect at all -- its formula slot IS the power's
// uses-per-day count, genuinely different from the shared `3+WIS` chain
// (`domain_power::DomainPowerSpec::uses_per_day_formula`, added alongside
// this entry).
pub const CONSTRUCT_SUBDOMAIN_SELECTION: &str = "domain:construct_subdomain";

/// v0.6 alpha swarm, risks item 8 (Cleric Good domain closure, generalized
/// task #64 to every real base class whose own domain-choice class feature
/// genuinely grants the chosen domain's POWERS -- see
/// `active_touch_of_good_bonus`): `ability_id` for the Good domain's
/// granted power, Touch of Good. Shared across every such class rather
/// than named per class, because it is the exact same PF1 Core Rulebook
/// granted power regardless of which class's domain choice unlocked it.
/// Grounds SELF-application only -- PF1 Core Rulebook Good Domain reads
/// "touch a creature, granting IT a sacred bonus," and this codebase has
/// no target-creature entity anywhere, so the only honest reuse of the
/// `ClassAbilityActivation` schema (which has no `target` field) is the
/// character touching herself. This is Touch of Good's RAW-secondary use
/// case (buffing an ally is the primary one), named explicitly wherever
/// this ability is explained -- narrows by TARGET, not by facet, unlike
/// Inspire Courage's attack-only narrowing.
///
/// Task #64 checked all four other classes with any DomainLVL-shaped
/// class feature before wiring anything, independently verifying each
/// against fresh primary sources rather than trusting an existing
/// blocker-message claim (one of which turned out to be wrong -- see
/// below):
/// - **Inquisitor: included.** PF1 Advanced Player's Guide Domain (or
///   Inquisition): "An inquisitor can select one domain from among those
///   belonging to her deity" (the same deity-restricted, alignment-gated
///   choice shape Cleric already has and this codebase already leaves
///   unmodeled) and grants ONLY that domain's powers -- "Each domain
///   grants a number of domain powers, depending on the level of the
///   inquisitor" -- explicitly NOT its bonus spells: "An inquisitor does
///   not gain the bonus spells listed for each domain, nor does she gain
///   bonus spell slots" (verbatim, d20pfsrd's Inquisitor class page).
///   This CORRECTS a stale, wrong claim this codebase's own
///   `push_inquisitor_other_features_deferred_diagnostic` doc comment
///   previously made ("no domain power exists for Inquisitor per the
///   corpus") -- the real rule is the opposite: powers yes, spells no.
/// - **Druid: excluded.** Nature's Bond's domain option (chosen in place
///   of an animal companion) genuinely does grant the chosen domain's
///   powers, but PF1 Core Rulebook Nature Bond hard-restricts that choice
///   to "one of the following cleric domains: Air, Animal, Earth, Fire,
///   Plant, Water, or Weather" (verified independently against two
///   primary sources: d20pfsrd's own Druid class page and the Archives of
///   Nethys `DruidDomains` reference, both confirmed byte-for-byte
///   consistent on this list) -- Good is never among them, so no real
///   Druid can ever hold Good-domain Touch of Good, and wiring it here
///   would fabricate an illegal domain/class combination.
/// - **Hunter and Paladin: excluded.** Both were confirmed (multiple
///   times this session, independently of this task) to have only
///   archetype-only DomainLVL-setting paths (Divine Hunter, Temple
///   Champion, Sacred Servant) -- no real base-class version of either
///   ever sets DomainLVL at all, so neither is wired here.
pub(super) const TOUCH_OF_GOOD_ABILITY_ID: &str = "touch_of_good";

/// SD-31 wave 25: War domain's Battle Rage self-application activation id,
/// the `domain_power::DOMAIN_POWER_CATALOG` sibling of `TOUCH_OF_GOOD_ABILITY_ID`.
pub(super) const BATTLE_RAGE_ABILITY_ID: &str = "battle_rage";

/// SD-31 wave 25: Strength domain's Strength Surge self-application
/// activation id, the `domain_power::DOMAIN_POWER_CATALOG` sibling of
/// `TOUCH_OF_GOOD_ABILITY_ID`.
pub(super) const STRENGTH_SURGE_ABILITY_ID: &str = "strength_surge";

/// SD-31 wave 26: Destruction domain's Destructive Smite self-application
/// activation id, the `domain_power::DOMAIN_POWER_CATALOG` sibling of
/// `TOUCH_OF_GOOD_ABILITY_ID`.
pub(super) const DESTRUCTIVE_SMITE_ABILITY_ID: &str = "destructive_smite";

/// SD-31 wave 26: Glory domain's Touch of Glory self-application activation
/// id, the `domain_power::DOMAIN_POWER_CATALOG` sibling of
/// `TOUCH_OF_GOOD_ABILITY_ID`.
pub(super) const TOUCH_OF_GLORY_ABILITY_ID: &str = "touch_of_glory";

/// SD-34 wave 37 lane A: Undead Subdomain's Death's Kiss self-application
/// activation id, the `domain_power::DOMAIN_POWER_CATALOG` sibling of
/// `TOUCH_OF_GOOD_ABILITY_ID` -- kept even though `grounds_self_application`
/// is `false` for this spec (no activation state is ever read for it in
/// production), so a future cycle that DOES honestly ground its duration
/// effect has a stable, already-reserved id to activate against.
pub(super) const DEATH_S_KISS_ABILITY_ID: &str = "death_s_kiss";

/// SD-34 wave 38 lane A: Construct Subdomain's Animate Servant
/// self-application activation id, the `domain_power::DOMAIN_POWER_CATALOG`
/// sibling of `TOUCH_OF_GOOD_ABILITY_ID` -- kept even though
/// `grounds_self_application` is `false` for this spec (no activation state
/// is ever read for it in production; its real effect is casting animate
/// objects, a spell-like ability with no self-application buff state to
/// activate), for the same reservation reason `DEATH_S_KISS_ABILITY_ID`
/// documents.
pub(super) const ANIMATE_SERVANT_ABILITY_ID: &str = "animate_servant";

// Task #64 checked whether a Druid's Nature Bond domain option should also be wired
// to the shared Good-domain Touch of Good granted power, the same way Cleric and
// Inquisitor are (see TOUCH_OF_GOOD_ABILITY_ID's own doc comment) -- and confirmed,
// independently against two primary sources (d20pfsrd's own Druid class page and the
// Archives of Nethys DruidDomains reference), that Nature Bond's domain option is
// hard-restricted to Air, Animal, Earth, Fire, Plant, Water, or Weather (PF1 Core
// Rulebook: "granting the druid one of the following cleric domains: Air, Animal,
// Earth, Fire, Plant, Water, or Weather"). The Good domain is never a legal Nature
// Bond selection, so no DRUID_DOMAIN_CHOICE_ID seam or Druid wiring was added here --
// doing so would fabricate a domain/power combination no real PF1 Druid can ever have.

/// v0.6 alpha swarm, risks item 8 (Druid animal companion closure): Wolf's
/// real PF1 Core Rulebook base statistics, verified against three
/// independent sources -- d20pfsrd's Animal Companions page, the Archives
/// of Nethys aonprd.com mirror's own Wolf companion page, and the PCGen
/// corpus (`cr_races_companion.lst`, citing Core Rulebook p.56 directly).
/// Ability scores agree across all three (Str 13, Dex 15, Con 15, Int 2,
/// Wis 12, Cha 6). Natural armor and Trip disagreed between d20pfsrd (+1,
/// no Trip until 4th-level advancement) and the other two (+2, Trip
/// present from 1st level) -- resolved in favor of the 2-of-3 majority,
/// the aonprd.com/corpus pair, the latter backed by a real page citation.
/// Wolf chosen as the canonical companion species: the simplest stat line
/// (one natural attack, no special movement modes), same "smallest
/// defensible single case" discipline as every other class's own fixed
/// canonical choice this session (Barbarian's Longsword, Sorcerer's
/// Arcane bloodline, etc.).
pub(super) const WOLF_COMPANION_STRENGTH_SCORE: i16 = 13;

pub(super) const WOLF_COMPANION_CONSTITUTION_SCORE: i16 = 15;

/// The Wolf companion race's own BASE natural armor
/// (`BONUS:VAR|AC_Natural_Armor|2|TYPE=Base`,
/// `core_rulebook/cr_races_companion.lst:32`). The companion class's own
/// level-scaling natural-armor bonus stacks on top of this -- see
/// `animal_companion_natural_armor_bonus`.
pub(super) const WOLF_COMPANION_NATURAL_ARMOR: i16 = 2;

pub(super) const WOLF_COMPANION_HIT_DIE_SIZE: u8 = 8;

pub(super) const HORSE_COMPANION_CONSTITUTION_SCORE: i16 = 15;

pub(super) const HORSE_COMPANION_NATURAL_ARMOR: i16 = 4;

pub(super) const HORSE_COMPANION_HIT_DIE_SIZE: u8 = 8;

/// The Wolf companion's Hit Dice at `companion_level` (the owning
/// character's own class level -- see `ground_wolf_companion_stat_block`
/// for why master level and companion level coincide for every class this
/// engine grounds).
///
/// Reads the one universal corpus progression,
/// `ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL`; see that constant for the
/// full derivation and corpus citations. At master level 1 this is 2 HD
/// (`MONSTERCLASS:Companion:2` -- a companion starts at 2 HD even for a
/// 1st-level druid), rising to 16 HD at master level 20.
pub(super) fn wolf_companion_hit_dice(companion_level: u8) -> u8 {
    ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL[animal_companion_table_index(companion_level)]
}

/// Grounds the Wolf companion's Link and Share Spells abilities as
/// vacuous-correction records (v0.6 alpha swarm, risks item 8, extracted
/// alongside `ground_wolf_companion_stat_block` for the same reuse
/// reason): both are provably vacuous under this codebase's model (Link's
/// Handle Animal skill-check exemption can never matter since this
/// codebase never computes a Handle Animal check; Share Spells' retarget-
/// a-cast-spell benefit can never trigger since this codebase has no
/// spell-casting-resolution engine anywhere, for any class), not merely
/// unmodeled -- the same "provably zero, not merely unmodeled" shape
/// Sorcerer's Arcane Bond spell-casting benefit needed.
pub(super) fn ground_wolf_companion_link_and_share_spells_vacuous(
    id_prefix: &str,
    owner_class_label: &str,
    explanations: &mut Vec<ComputationExplanation>,
) {
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.link_vacuous"),
        value: 0,
        detail: format!(
            "Wolf companion's Link ability (PF1 Core Rulebook: \"A {owner_class_label} can \
             handle her animal companion as a free action, or push it as a move action, even if \
             she doesn't have any ranks in the Handle Animal skill\") is vacuous under this \
             bounded seam: this codebase computes exactly three selected skills (Climb, \
             Intimidate, Swim), never Handle Animal, so the skill-check exemption Link grants \
             can never matter here regardless of build. This record documents that correction \
             only; it carries no mechanical value (+0)"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.share_spells_vacuous"),
        value: 0,
        detail: format!(
            "Wolf companion's Share Spells ability (PF1 Core Rulebook: \"The {owner_class_label} \
             may cast a spell with a target of 'You' on her animal companion... instead of on \
             herself\") is vacuous under this bounded seam: this codebase has no spell-casting-\
             resolution engine anywhere, for any class -- no spell is ever actually cast, so \
             retargeting one is never triggerable here regardless of build (the same structural \
             gap that made Sorcerer's Arcane Bond spell-casting benefit provably vacuous). This \
             record documents that correction only; it carries no mechanical value (+0)"
        ),
    });
}

/// Horse companion's Hit Dice at `companion_level` (v0.6 alpha swarm,
/// risks item 8, Cavalier Mount closure) -- a parallel, deliberately NOT
/// shared copy of `wolf_companion_hit_dice`'s own identical logic (the
/// PF1 Core Rulebook "Animal Companion Base Statistics" HD progression is
/// universal to every companion species), kept separate rather than
/// reusing the Wolf-named function directly so neither this function's
/// name nor its own future evolution ever risks touching Druid's/
/// Hunter's already-shipped Wolf call sites.
///
/// The corpus confirms the progression really is shared: `CavalierMountLVL`
/// (`advanced_players_guide/apg_companionmods.lst:75-93`) grants its Hit
/// Dice at exactly the same fourteen master levels as the CRB animal
/// companion's own `AnimalCompanionLVL` block, so both species names read
/// the single `ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL` table.
pub(super) fn horse_companion_hit_dice(companion_level: u8) -> u8 {
    ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL[animal_companion_table_index(companion_level)]
}

/// Grounds the Horse companion's Link ability as a vacuous-correction
/// record (v0.6 alpha swarm, risks item 8, Cavalier Mount closure) --
/// mirrors `ground_wolf_companion_link_and_share_spells_vacuous`'s own
/// Link reasoning exactly (this codebase computes no Handle Animal
/// check, so Link's skill-check exemption can never matter here).
/// Deliberately does NOT ground a Share-Spells vacuous-correction record
/// at all: the PF1 Core Rulebook explicitly states "a cavalier's mount
/// does not gain the share spells special ability" -- the Mount never
/// has this ability in the first place, so grounding a "correction" for
/// an ability it doesn't even possess would misrepresent what's being
/// corrected, unlike Wolf's Share Spells (which Druids/Hunters' own
/// companions DO have, just provably vacuous under this codebase's
/// model).
pub(super) fn ground_horse_companion_link_vacuous(
    id_prefix: &str,
    owner_class_label: &str,
    explanations: &mut Vec<ComputationExplanation>,
) {
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.link_vacuous"),
        value: 0,
        detail: format!(
            "Horse Mount's Link ability (PF1 Core Rulebook: \"A {owner_class_label} can handle \
             her animal companion as a free action, or push it as a move action, even if she \
             doesn't have any ranks in the Handle Animal skill\") is vacuous under this bounded \
             seam: this codebase computes exactly three selected skills (Climb, Intimidate, \
             Swim), never Handle Animal, so the skill-check exemption Link grants can never \
             matter here regardless of build. This record documents that correction only; it \
             carries no mechanical value (+0). Unlike Druid's/Hunter's own companion, a \
             cavalier's Mount does not gain the Share Spells special ability at all (per the \
             PF1 Core Rulebook's own Mount description), so no Share-Spells vacuous-correction \
             record is grounded here -- there is nothing to correct for an ability the Mount \
             never has"
        ),
    });
}

// Grounded Human pilot race seam identities. These name the already-accepted
// deterministic Human selections; this slice makes their pressure explicit but
// grounds no non-Human race semantics and no broader Human racial trait burden.
pub(super) const HUMAN_RACE_ID: &str = "race:human";

pub(super) const HUMAN_ABILITY_BONUS_CHOICE_ID: &str = "choice:human_ability_bonus";

pub(super) const HUMAN_BONUS_FEAT_CHOICE_ID: &str = "choice:human_bonus_feat";

pub(super) const ABILITY_SELECTION_PREFIX: &str = "ability:";

/// PF1 Core Rulebook Standard Human flat racial ability-bonus magnitude (the
/// player-chosen "+2 to one ability score of your choice" trait). Unlike
/// Dwarf/Half-Elf/etc., whose fixtures encode the convention "the chosen score
/// already reflects the fixed racial adjustment," Human's deterministic pilot
/// fixture and the real PCGen oracle both confirm the opposite convention for
/// Human: the chosen score is the PRE-bonus base, and the +2 must be applied
/// at compute time, before ability modifiers are derived. See
/// `apply_human_ability_bonus`.
pub(super) const HUMAN_ABILITY_BONUS_MAGNITUDE: i16 = 2;

// SD13-E6-F3a Human racial trait bundle (size, speed, senses, extra skill ranks).
// These name the remaining Human racial trait burden explicitly, classified
// against PF1 Core Rulebook Standard Human racial traits (source evidence only,
// not oracle-checked parity):
//   cr_races.lst race:human SIZE:MEDIUM        -> Medium size category
//   cr_races.lst race:human GAIT:WALK|30       -> 30 ft base land speed
//   cr_races.lst race:human                   -> no special senses (PCGen races
//                                                in the CRB only carry the SENSE
//                                                tag when a sense bonus exists;
//                                                Human has none for Standard Human)
//   cr_races.lst race:human BONUS:SKILL|...   -> 4 extra skill points at 1st
//                                                level and 1 extra skill rank
//                                                per level thereafter
//
// This constant set deliberately names the entire PF1 Standard Human racial
// trait surface — every line a Player's Handbook Human racial entry lists —
// so the explanation records can name each dimension explicitly instead of
// leaving it an incidental side-effect or a folklore claim.
//
// None of these ground a computed mechanical contribution to the existing
// NumericOutputs in this slice. They explain Human identity only; the chassis
// totals remain controlled by the bounded deterministic posture.
pub(super) const HUMAN_SIZE_CATEGORY: &str = "Medium";

pub(super) const HUMAN_BASE_SPEED_FEET: i16 = 30;

pub(super) const HUMAN_EXTRA_SKILL_POINTS_AT_LEVEL_1: u8 = 4;

pub(super) const HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL: u8 = 1;

// Grounded deterministic combat-baseline contributors and posture identities.
pub(crate) const LONGSWORD_ITEM_ID: &str = "item:longsword";

pub(super) const CHAIN_SHIRT_ITEM_ID: &str = "item:chain_shirt";

pub(super) const SHIELD_ITEM_ID: &str = "item:shield";

pub(crate) const POWER_ATTACK_ITEM_ID: &str = "power_attack";

pub(crate) const DODGE_FEAT_ID: &str = "feat:dodge";

pub(crate) const WEAPON_FOCUS_FEAT_ID: &str = "feat:weapon_focus";

pub(crate) const WEAPON_FOCUS_LONGSWORD_SELECTION: &str = "feat:weapon_focus:weapon:longsword";

// SD13-E5-F9 canonical Human Fighter feat-choice seam. These name the exact accepted
// deterministic feat-choice selections on the level-1/2/3 seam. This slice preserves
// these selections and claim-blocks any deviation of the named slots; it grounds no
// general feat-effect or prerequisite engine and no alternative feat legality.
pub(super) const LEVEL_1_CHARACTER_FEAT_CHOICE_ID: &str = "choice:level_1_character_feat";

pub(super) const POWER_ATTACK_FEAT_SELECTION: &str = "feat:power_attack";

pub(super) const TOUGHNESS_FEAT_SELECTION: &str = "feat:toughness";

// Grounded numeric contributors (source evidence only; not oracle-checked parity):
//   cr_equip_arms_armor.lst:40  Chain Shirt -> BONUS:COMBAT|AC|4|TYPE=Armor, MAXDEX:4
//   cr_feats.lst:53             Dodge       -> BONUS:COMBAT|AC|1|TYPE=Dodge
//   cr_feats.lst:184            Weapon Focus-> +1 to-hit with the selected weapon
pub(crate) const ARMOR_CLASS_BASE: i16 = 10;

pub(super) const CHAIN_SHIRT_ARMOR_BONUS: i16 = 4;

pub(super) const CHAIN_SHIRT_MAX_DEX: i16 = 4;

pub(crate) const DODGE_AC_BONUS: i16 = 1;

pub(crate) const WEAPON_FOCUS_TO_HIT_BONUS: i16 = 1;

/// Raised when a character's `race_id` resolves to no ingested race, so its
/// creature size -- and therefore its size modifier to Armor Class -- is
/// unknown.
///
/// Claim-blocking, deliberately, and for the same reason
/// `contract::encumbrance_size_for_race` already blocks on the identical
/// condition: an Armor Class computed at an assumed size is not real data, and
/// this repo's recorded failure mode is wrong numbers that survived because
/// nothing failed loudly. All 18 creatable races resolve, so no character a
/// player can actually build reaches this.
pub(crate) const UNKNOWN_RACE_ARMOR_CLASS_SIZE_DIAGNOSTIC_ID: &str =
    "defense.size_modifier.unknown_race";

/// Every size-derived term the combat baseline needs, resolved **once** per
/// computation so a single unknown race raises exactly one diagnostic rather
/// than one per consumer.
///
/// The two magnitudes are different PF1 columns, not one value reused: Table
/// 8-1's *Size Modifier* (Armor Class, and attack rolls, which take the
/// identical value) and the *special size modifier* the CMB/CMD text calls for,
/// which runs in the opposite direction. See `size.rs`, which transcribes both
/// against the published table and pins them independently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CombatSizeModifiers {
    /// PF1 Table 8-1's size modifier: applies to Armor Class (and therefore
    /// touch AC) **and** to attack rolls.
    pub(crate) armor_class_and_attack: i16,
    /// The special size modifier: applies to CMB and CMD.
    pub(crate) special: i16,
    /// Human-readable size name for the explanation strings, or `"unknown"`.
    pub(crate) label: &'static str,
    /// The resolved category itself, or `None` for an unresolvable race.
    /// Needed because CMB's ability-modifier term is size-*dependent*, not just
    /// size-*modified*: Tiny and smaller creatures substitute Dexterity for
    /// Strength, which no scalar modifier can express.
    pub(crate) category: Option<SizeCategory>,
}

/// The character's creature size, resolved into the two PF1 size-modifier
/// columns the combat baseline consumes.
///
/// When the race does not resolve, both modifiers are 0 **and** a
/// claim-blocking diagnostic is pushed, so the assumption is visible rather
/// than laundered into a plausible-looking total.
///
/// # Why the size comes from `race_resolver` and not `rules_tables::crb`
///
/// `race_resolver::race_size_for_race_token` is the authority per
/// `decisions.md §25.5`: it covers all 18 in-scope races and reads each one's
/// `~ Size` racial-default trait `TEMPLATE:SIZE_<code>`, which is *not* always
/// the chassis' `FACT:BaseSize` (Aasimar and Tiefling carry `FACT:BaseSize|S`
/// and are Medium creatures). `rules_tables::crb::race_tables::race_size_for_race_id`
/// knew only the 7 hardcoded CRB races and returned `None` for all 11 Bestiary 1
/// ones -- using it here would have silently left Goblin, Kobold and
/// Svirfneblin on Medium arithmetic, which is the very defect being fixed.
pub(super) fn combat_size_modifiers(
    input: &CharacterInput,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> CombatSizeModifiers {
    match race_size_for_race_token(&input.chosen.race_id) {
        Some(size) => CombatSizeModifiers {
            armor_class_and_attack: size.armor_class_size_modifier(),
            special: size.special_size_modifier(),
            label: size_label(size),
            category: Some(size),
        },
        None => {
            diagnostics.push(ComputationDiagnostic {
                id: UNKNOWN_RACE_ARMOR_CLASS_SIZE_DIAGNOSTIC_ID.to_owned(),
                message: format!(
                    "race {:?} resolves to no ingested race, so its creature size is unknown; \
                     the size modifiers to Armor Class, touch AC, attack rolls, CMB and CMD \
                     could not be applied and the combat totals below are not real data",
                    input.chosen.race_id
                ),
                claim_blocking: true,
            });
            CombatSizeModifiers {
                armor_class_and_attack: 0,
                special: 0,
                label: "unknown",
                category: None,
            }
        }
    }
}

/// PF1's Combat Maneuver Defense base, as the Core Rulebook prints it:
/// `CMD = 10 + BAB + Str modifier + Dex modifier + special size modifier`.
///
/// Deliberately its own constant rather than a reuse of `ARMOR_CLASS_BASE`.
/// Both happen to be 10, but they are two separate published formulas; tying
/// them together would encode "these are the same fact", which is exactly the
/// unstated assumption `size.rs` refuses to make between its own two columns.
pub(crate) const COMBAT_MANEUVER_DEFENSE_BASE: i16 = 10;

/// PF1's Combat Maneuver Bonus: `BAB + Strength modifier + special size
/// modifier`.
///
/// # The Tiny-or-smaller substitution is real and is applied
///
/// The Core Rulebook's CMB entry states that creatures of size Tiny or smaller
/// use their **Dexterity** modifier in place of Strength. No currently
/// creatable race is Tiny or smaller (the 18 in-scope races are 13 Medium and 5
/// Small), so this branch is unreachable from the UI today -- it is written
/// anyway, and pinned by a unit test, because the alternative is a function
/// that is silently wrong the moment a Tiny race is ingested. That is the same
/// reasoning `size.rs::load_capacity_ratio` gives for transcribing all nine
/// `SIZEMULT:` rows rather than the two that are reachable.
///
/// # Named boundary, not silently omitted
///
/// Real PF1 CMB also takes the Improved/Greater maneuver feats, a size-changing
/// effect, and any circumstance bonus. None of those is modelled anywhere in
/// this engine, so none is summed here; the explanation string states the terms
/// it actually contains rather than implying completeness.
pub(crate) fn combat_maneuver_bonus(
    base_attack_bonus: i16,
    strength_modifier: i16,
    dexterity_modifier: i16,
    size: Option<SizeCategory>,
    special_size_modifier: i16,
) -> i16 {
    // `None` is an unresolvable race, which the caller has already raised a
    // claim-blocking diagnostic for. Strength is used so the shape of the
    // formula is unchanged; the number is not claimed to be real either way.
    let ability_modifier = match size {
        Some(SizeCategory::Fine | SizeCategory::Diminutive | SizeCategory::Tiny) => {
            dexterity_modifier
        }
        _ => strength_modifier,
    };
    base_attack_bonus + ability_modifier + special_size_modifier
}

/// PF1's Combat Maneuver Defense: `10 + BAB + Str modifier + Dex modifier +
/// special size modifier`, exactly as the Core Rulebook prints the formula.
///
/// # Two boundaries, both stated rather than guessed
///
/// 1. **The AC bonuses CMD also receives are not folded in.** The CRB adds that
///    circumstance, deflection, dodge, insight, luck, morale, profane and
///    sacred bonuses to Armor Class apply to CMD as well. This engine grounds
///    several of those (Dodge's +1, Brawler's AC Bonus, Inquisitor's Protection
///    judgment). They are deliberately **not** summed here: the defect this
///    cycle closes is the missing *size* term, and folding in a second,
///    unmeasured correction at the same time would make it impossible to say
///    which change moved a number. The explanation string names the terms it
///    contains, so nothing is implied that is not computed.
/// 2. **The Dexterity modifier is the raw ability modifier**, not the
///    armor-capped contribution touch AC uses. That is what the published
///    formula prints ("Dex modifier"), and it is what this repo's UI already
///    computed, so this cycle does not move it. Whether an armor's `MAXDEX`
///    limits CMD is a separate published question this codebase has no
///    authority on yet; it is named here rather than silently decided.
pub(crate) fn combat_maneuver_defense(
    base_attack_bonus: i16,
    strength_modifier: i16,
    dexterity_modifier: i16,
    special_size_modifier: i16,
) -> i16 {
    COMBAT_MANEUVER_DEFENSE_BASE
        + base_attack_bonus
        + strength_modifier
        + dexterity_modifier
        + special_size_modifier
}

/// PF1 touch Armor Class: the character's **own** Armor Class total with the
/// contributors a touch attack ignores removed.
///
/// Derived by subtraction, not recomputed from scratch, and that is the whole
/// point. Touch AC is not an independent statistic -- it is the same Armor
/// Class, minus armor, shield and natural armor. Computing it separately is
/// precisely how the shipped sheet ended up displaying `AC 19` beside
/// `TOUCH 14` with a 4-point armor bonus, a self-contradiction that no amount
/// of correct-looking arithmetic in the touch formula would have caught.
/// Expressed this way, the two cannot disagree.
///
/// `excluded` is the caller's own sum of its armor + shield + natural-armor
/// terms. Each call site knows which of its terms are which; this function
/// deliberately does not try to re-derive that from a total it cannot see
/// inside.
pub(crate) fn touch_armor_class(armor_class: i16, excluded: i16) -> i16 {
    armor_class - excluded
}

/// PF1 flat-footed Armor Class: the character's **own** Armor Class total with
/// the contributors a flat-footed creature is denied removed.
///
/// Same subtract-from-the-real-total shape as [`touch_armor_class`], and for
/// the same reason. Until SD-27 (`decisions.md §28`) this statistic did not
/// exist in this engine at all — it was a THIRD compute twin living in
/// `apps/desktop/src/characterHub/CharacterSheet.tsx` as
/// `ac - Math.max(0, dexMod)` (introduced by `f5117103`, 2026-07-11). That
/// formula is missing half the rule, so a Tiefling who took Dodge read
/// `AC 20 / flat-footed 17` on screen where PF1's answer is 16.
///
/// # The two quantities, each named rather than inferred
///
/// * `dexterity_contribution_to_armor_class` — the caller's whole
///   Dexterity-derived contribution, **after** any armor `MAXDEX` cap, and
///   including any ability that substitutes another score *for* Dexterity
///   (Oracle Nature's Whispers). Only a positive contribution is removed: a
///   Dexterity *penalty* is not a bonus, and PF1 denies the bonus, so a
///   clumsy character keeps their penalty while flat-footed. That `max(0)` is
///   the one piece of the old React line that was already right, and it is
///   kept here so both compute twins share one copy of it.
/// * `dodge_typed_bonuses` — every dodge-typed term the caller summed into
///   `armor_class`. PF1, *Bonus Types*: "A dodge bonus improves Armor Class
///   resulting from physical skill at avoiding blows. Any situation that
///   denies you your Dexterity bonus to Armor Class also denies you dodge
///   bonuses." Each call site knows which of its own terms are dodge-typed;
///   this function deliberately does not try to re-derive that from a total it
///   cannot see inside.
///
/// Every other modifier type stays: armor, shield, natural armor, deflection,
/// the size modifier, sacred/profane bonuses, and every penalty (a penalty is
/// never dropped by being caught unprepared).
pub(crate) fn flat_footed_armor_class(
    armor_class: i16,
    dexterity_contribution_to_armor_class: i16,
    dodge_typed_bonuses: i16,
) -> i16 {
    armor_class - dexterity_contribution_to_armor_class.max(0) - dodge_typed_bonuses
}

/// The PF1 size-category name, for explanation strings.
pub(super) fn size_label(size: SizeCategory) -> &'static str {
    match size {
        SizeCategory::Fine => "Fine",
        SizeCategory::Diminutive => "Diminutive",
        SizeCategory::Tiny => "Tiny",
        SizeCategory::Small => "Small",
        SizeCategory::Medium => "Medium",
        SizeCategory::Large => "Large",
        SizeCategory::Huge => "Huge",
        SizeCategory::Gargantuan => "Gargantuan",
        SizeCategory::Colossal => "Colossal",
    }
}

// Grounded selected-skill contributors (source evidence only; not oracle-checked):
//   cr_skills.lst:10   Climb      -> KEYSTAT:STR, ACHECK:YES, BONUS:SKILL|Climb|3|TYPE=ClassSkill
//   cr_skills.lst:42   Intimidate -> KEYSTAT:CHA (no ACHECK), BONUS:SKILL|Intimidate|3|TYPE=ClassSkill
//   cr_skills.lst:102  Swim       -> KEYSTAT:STR, ACHECK:YES, BONUS:SKILL|Swim|3|TYPE=ClassSkill
//   cr_abilities_class.lst:2835   Fighter class skills include Climb, Intimidate, Swim
//   cr_equip_arms_armor.lst:40    Chain Shirt -> ACCHECK:-2
pub(crate) const CLIMB_SKILL_ID: &str = "skill:climb";

pub(crate) const INTIMIDATE_SKILL_ID: &str = "skill:intimidate";

pub(crate) const SWIM_SKILL_ID: &str = "skill:swim";

pub(crate) const SELECTED_SKILL_RANK: u8 = 1;

pub(crate) const CLASS_SKILL_BONUS: i16 = 3;

pub(super) const CHAIN_SHIRT_ARMOR_CHECK_PENALTY: i16 = -2;

pub(super) const CLEAVE_FEAT_SELECTION: &str = "feat:cleave";

pub(super) const COMBAT_REFLEXES_FEAT_SELECTION: &str = "feat:combat_reflexes";

pub(super) const IMPROVED_CRITICAL_FEAT_SELECTION: &str = "feat:improved_critical";

pub(super) const GREATER_WEAPON_FOCUS_FEAT_SELECTION: &str = "feat:greater_weapon_focus";

pub(super) const WEAPON_SPECIALIZATION_FEAT_SELECTION: &str = "feat:weapon_specialization";

pub(super) const GREATER_WEAPON_SPECIALIZATION_FEAT_SELECTION: &str = "feat:greater_weapon_specialization";

pub(super) const CRITICAL_FOCUS_FEAT_SELECTION: &str = "feat:critical_focus";

pub(super) const STAGGERING_CRITICAL_FEAT_SELECTION: &str = "feat:staggering_critical";

pub(super) const CRITICAL_MASTERY_FEAT_SELECTION: &str = "feat:critical_mastery";

pub(super) const HEAVY_BLADES_GROUP_SELECTION: &str = "group:heavy_blades";

pub(super) const BOWS_GROUP_SELECTION: &str = "group:bows";

pub(super) const POLEARMS_GROUP_SELECTION: &str = "group:polearms";

pub(super) const HAMMERS_GROUP_SELECTION: &str = "group:hammers";

// `AT-34-E3-001` (mechanism 3 continuation, cycle 9): the full, closed
// enumeration of the Fighter's 14 canonical weapon-training groups
// (cr_abilities_class.lst's own "Weapon Training <tier> <group>" corpus
// records, confirmed directly against `data/corpus/core_rulebook/
// class_feature/weapon_training_*/`, never assumed). PF1's own Weapon
// Training rule grants the bonus to WHICHEVER group a Fighter selects at
// each tier -- the bonus magnitude (rank, rank-1, rank-2, rank-3 for tiers
// 1-4) depends only on the tier and character level, never on which of
// the 14 groups was picked. Unlike a Fighter's open-ended bonus-feat
// choice (deliberately left un-generalized -- any feat is legal, an
// unbounded set), the weapon-training group choice is a closed,
// enumerable, rulebook-fixed set of exactly 14 values, the same shape
// `FAVORED_ENEMY_TYPES` (cycle 3) and the wizard opposed-school constants
// already generalize over. Widening the 4 canonical-only checks below to
// accept any of these 14 selections is therefore not a relaxation of the
// bounded-fixture discipline -- it is applying the SAME real, unconditional
// PF1 rule this file already encodes, to every legal input instead of one
// hardcoded one. `"Monk"` is included for rule-completeness (a real
// Fighter may train the monk weapon group) even though its own corpus
// record resolves through a different code path before ever reaching this
// check (its `class` field is `"Monk"`, not `null`), so including it here
// changes no observed unit's verdict.
pub(super) const WEAPON_TRAINING_GROUPS: [(&str, &str); 14] = [
    ("Axes", "group:axes"),
    ("Blades Heavy", HEAVY_BLADES_GROUP_SELECTION),
    ("Blades Light", "group:blades_light"),
    ("Bows", BOWS_GROUP_SELECTION),
    ("Close", "group:close"),
    ("Crossbows", "group:crossbows"),
    ("Double", "group:double"),
    ("Flails", "group:flails"),
    ("Hammers", HAMMERS_GROUP_SELECTION),
    ("Monk", "group:monk"),
    ("Natural", "group:natural"),
    ("Pole Arms", POLEARMS_GROUP_SELECTION),
    ("Spears", "group:spears"),
    ("Thrown", "group:thrown"),
];

/// Return the corpus group-name suffix (e.g. `"Axes"`, matching
/// `"Weapon Training <tier> Axes"`) for a `choice:fighter_weapon_training_
/// group*` selection literal, or `None` when the selection is absent or is
/// not one of the 14 canonical PF1 weapon-training groups.
pub(super) fn weapon_training_group_name_for_selection(selection: &str) -> Option<&'static str> {
    WEAPON_TRAINING_GROUPS
        .iter()
        .find(|(_, sel)| *sel == selection)
        .map(|(name, _)| *name)
}

pub(super) const ARMOR_TRAINING_1_ARMOR_CHECK_REDUCTION: i16 = 1;

pub(super) const ARMOR_TRAINING_1_MAX_DEX_INCREASE: i16 = 1;

pub(super) const ARMOR_TRAINING_2_ARMOR_CHECK_REDUCTION: i16 = 2;

pub(super) const ARMOR_TRAINING_2_MAX_DEX_INCREASE: i16 = 2;

pub(super) const ARMOR_TRAINING_3_ARMOR_CHECK_REDUCTION: i16 = 3;

pub(super) const ARMOR_TRAINING_3_MAX_DEX_INCREASE: i16 = 3;

pub(super) const ARMOR_TRAINING_4_ARMOR_CHECK_REDUCTION: i16 = 4;

pub(super) const ARMOR_TRAINING_4_MAX_DEX_INCREASE: i16 = 4;

pub(super) const ARMOR_MASTERY_DAMAGE_REDUCTION: i16 = 5;

pub(super) const WEAPON_MASTERY_CRITICAL_MULTIPLIER_INCREASE: i16 = 1;

/// Simple integrated status for the GE-06 pilot headless receipt: whether the
/// path produced computed evidence or is blocked. This distinguishes evidence
/// from a blocker posture; it is not an oracle-checked parity verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadlessReceiptStatus {
    /// The integrated deterministic path produced computed evidence with no
    /// claim-blocking diagnostics.
    Computed,
    /// The integrated deterministic path is blocked; at least one claim-blocking
    /// diagnostic is present and no success state is fabricated.
    Blocked,
}

/// One bounded, library-first, headless receipt for the accepted deterministic
/// GE-06 pilot path. It preserves case and source-package identity, a simple
/// computed/blocked status, and the full underlying computation (already-grounded
/// outputs, explanations, and diagnostics) for later parity or UI consumers.
///
/// This is headless computed evidence only; it must not be relabeled as
/// oracle-checked parity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotHeadlessReceipt {
    /// Case identity carried from the loaded input (absent when the input names none).
    pub case_id: Option<String>,
    /// Source package identity carried from the loaded input.
    pub source_package_id: String,
    /// Whether the integrated path produced evidence or is blocked.
    pub status: HeadlessReceiptStatus,
    /// The underlying pilot computation, preserving the already-grounded outputs,
    /// explanation records, and claim-blocking diagnostics unchanged.
    pub computation: PilotBaseChassisComputation,
}

/// Build the GE-06 pilot headless receipt from a loaded character input.
///
/// This runs the existing deterministic compute surface and wraps it in one
/// bounded receipt shape, deriving the integrated status from the computation's
/// claim-blocking diagnostics: any claim-blocking diagnostic blocks the path,
/// otherwise the path is computed. It adds no new computed value, fabricates no
/// success state, and discards none of the existing explanations or diagnostics.
pub fn build_pilot_headless_receipt(input: &CharacterInput) -> PilotHeadlessReceipt {
    let computation = compute_pilot_base_chassis(input);

    let status = if computation.diagnostics.iter().any(|d| d.claim_blocking) {
        HeadlessReceiptStatus::Blocked
    } else {
        HeadlessReceiptStatus::Computed
    };

    PilotHeadlessReceipt {
        case_id: input.case_id.clone(),
        source_package_id: input.source_package_id.clone(),
        status,
        computation,
    }
}

/// Compute the GE-06 pilot base chassis from a loaded character input.
pub fn compute_pilot_base_chassis(input: &CharacterInput) -> PilotBaseChassisComputation {
    let mut explanations = Vec::new();
    let mut diagnostics = Vec::new();

    let ability_scores_for_modifiers = apply_human_ability_bonus(input, &mut explanations);
    let base_ability_modifiers =
        compute_ability_modifiers(&ability_scores_for_modifiers, &mut explanations);
    // v0.6 alpha swarm, risks item 8: Barbarian Rage's Strength/Constitution
    // morale bonus layers onto the base modifiers here (mirrors
    // `compute_total_saves`'s existing feat-bonus layering pattern), so
    // every downstream consumer of `ability_modifiers` (combat baseline,
    // total saves, selected-skill modifiers) sees the boosted values while
    // actively raging. The rage-rounds-per-day budget check itself uses the
    // separate `base_ability_modifiers` (pre-rage Constitution), since PF1's
    // rounds-per-day is derived from the character's normal Constitution,
    // not a value that depends on already being raged.
    let ability_modifiers_after_rage =
        apply_rage_ability_bonuses(base_ability_modifiers, input, &mut explanations);
    // v0.6 alpha swarm, risks item 8 (first APG/ACG closure): Skald Inspired
    // Rage's Strength/Constitution morale bonus layers on immediately after
    // Barbarian's, the identical shape -- class-ownership-gated by
    // `active_skald_inspired_rage_bonus` construction, so a non-Skald
    // character (which includes every Barbarian) sees no change here.
    let ability_modifiers_after_inspired_rage = apply_skald_inspired_rage_ability_bonuses(
        ability_modifiers_after_rage,
        input,
        &mut explanations,
    );
    // v0.6 alpha swarm, risks item 8 (second APG/ACG closure): Bloodrager
    // Bloodrage's Strength/Constitution morale bonus layers on next, the
    // identical shape -- class-ownership-gated by
    // `active_bloodrager_bloodrage_bonus` construction, so a non-Bloodrager
    // character sees no change here.
    let ability_modifiers_after_bloodrage = apply_bloodrager_bloodrage_ability_bonuses(
        ability_modifiers_after_inspired_rage,
        input,
        &mut explanations,
    );
    // v0.6 alpha swarm, risks item 8 (Alchemist Mutagen closure): Mutagen's
    // ability-score bonus/penalty layers on last, the identical
    // "apply on top of the chain" shape -- class-ownership-gated by
    // `active_alchemist_mutagen_bonus` construction, so a non-Alchemist
    // character sees no change here.
    let ability_modifiers = apply_alchemist_mutagen_ability_bonuses(
        ability_modifiers_after_bloodrage,
        input,
        &mut explanations,
    );

    let computed_chassis =
        compute_class_chassis(input, &ability_modifiers, &mut explanations, &mut diagnostics);
    // SD-27 (`decisions.md` §24/§28, 2026-07-31): whether the chassis really
    // produced a base attack bonus, kept separately from the `0` the fallback
    // below substitutes. PU's Combat Stamina pool is `BAB + CON`, and a `0`
    // that means "not computed" would silently ship a stamina pool short by the
    // character's whole base attack bonus.
    let chassis_supported = computed_chassis.is_some();
    let (base_attack_bonus, base_saves) = computed_chassis
            .unwrap_or_else(|| {
            diagnostics.push(ComputationDiagnostic {
                id: "class_chassis.unsupported".to_owned(),
                message: format!(
                    "base class chassis is only supported for a single-class {FIGHTER_CLASS_ID} \
                     at levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL} or a single-class \
                     {WIZARD_CLASS_ID} at levels 1-{MAX_SUPPORTED_WIZARD_LEVEL}; chosen class \
                     levels {:?} do not provide it (either a multiclass mix, deferred to Epic 7, \
                     or a class this per-class dispatch does not yet recognize), so no chassis \
                     values were computed",
                    input.chosen.class_levels
                ),
                claim_blocking: true,
            });
            (0, BaseSaves::default())
        });

    let (baseline_melee_attack_bonus, baseline_armor_class) = compute_combat_baseline(
        input,
        &ability_modifiers,
        base_attack_bonus,
        &mut explanations,
        &mut diagnostics,
    );

    // Stages 2-3 (task #72): additive per-weapon totals, independent of the
    // GE-06 posture gate above -- an unsupported baseline posture must not
    // suppress a weapon's own real attack total.
    ground_per_weapon_combat_totals(
        input,
        &ability_modifiers,
        base_attack_bonus,
        &mut explanations,
        &mut diagnostics,
    );

    let total_saves = compute_total_saves(
        input,
        &ability_modifiers,
        &base_saves,
        &mut explanations,
        &mut diagnostics,
    );

    let selected_skill_modifiers = compute_selected_skill_modifiers(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    // v0.6 alpha swarm: standalone feat-derived skill facts (turnkey
    // consumer wiring for `feat_effects::standalone_skill_facts_from_feats`/
    // `skill_focus_facts_from_choices`, both already built and tested).
    // Unconditional on class ownership/posture -- these are general feat
    // effects, not class-specific, so they ground for every character
    // regardless of chassis support.
    ground_standalone_feat_skill_facts(input, &mut explanations);
    ground_orphan_feat_facts(
        input,
        base_attack_bonus,
        chassis_supported,
        &ability_modifiers,
        &mut explanations,
    );
    ground_orphan_trait_facts(input, &mut explanations);

    explain_fighter_class_features(input, &mut explanations);

    explain_fighter_level1_hit_points(input, &ability_modifiers, &mut explanations);

    explain_fighter_favored_class_bonus_choice(input, &mut explanations);
    explain_other_classes_favored_class_bonus_choice(input, &mut explanations);

    explain_hybrid_level1_chassis(input, &mut explanations);
    explain_barbarian_level1_chassis(
        input,
        &base_ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );
    explain_monk_level1_chassis(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );
    explain_rogue_level1_chassis(input, &ability_modifiers, &mut explanations);
    explain_base_class_weapon_and_armor_proficiency(input, &mut explanations);


    // SD13-E3/E4/E5 Paladin-only decomposition: split the F6 hybrid class-feature
    // and spell-burden blockers into per-burden diagnostics so the chassis
    // burden is separable from the partial-caster spell burden on the runtime
    // path, widened by SD13-E5 to the level-2 lay on hands / divine grace
    // grant. This is an extension, never a downgrade, of the F6 surface.
    explain_paladin_level1_chassis_and_spell_burden_separation(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    // `decisions.md §22`'s "FURTHER UPDATE, 2026-09-04": unlike the burden
    // separation above, unconditional on race and single-class status --
    // see `ground_paladin_detect_evil`'s own doc comment.
    ground_paladin_detect_evil(input, &mut explanations);

    // SD-34 wave 43 (`decisions.md §22`'s 12-unit "small-precedented-new-
    // compute" remainder, closing the last 4 of the 15-unit new-chassis
    // list bar Wizard's Arcane Bond): four prestige classes with NO
    // `ClassId`-family enum entry at all, so -- exactly like
    // `ground_paladin_detect_evil` immediately above -- these run
    // unconditional on chassis support, keyed on the raw `class_id` string
    // rather than any enum dispatch. See each function's own doc comment
    // for its corpus citation.
    ground_duelist_class_features(input, &ability_modifiers, &mut explanations);
    ground_shadowdancer_class_features(input, &mut explanations);
    ground_assassin_class_features(input, &ability_modifiers, &mut explanations);
    ground_loremaster_class_features(input, &mut explanations);

    // SD-34 wave 44 (`decisions.md §22`, Piece 2 item 3): a fifth prestige
    // class in this same "no `ClassId` enum entry" family -- see
    // `ground_pathfinder_delver_class_features`'s own doc comment for the
    // real-owner audit correction (Pathfinder Delver's own Guardbreaker
    // feature, not Ranger's favored-enemy chooser).
    ground_pathfinder_delver_class_features(input, &mut explanations);

    // SD-34 wave 45 (`decisions.md §22`'s WAVE 45 UPDATE): Phrenic Slayer's
    // Favored Enemy record, the first closed slice of sub-mechanism-5's
    // "registered prestige class, magnitude-only" remainder -- same
    // unconditional placement as the five prestige-class functions above.
    ground_phrenic_slayer_class_features(input, &mut explanations);

    // SD-34 wave 46 (`decisions.md §22`'s WAVE 46 UPDATE): five more
    // prestige classes in the same "registered, no `ClassId` enum entry,
    // magnitude-only" remainder -- same unconditional placement as the
    // Pathfinder Delver/Phrenic Slayer functions above.
    ground_argent_dramaturge_class_features(input, &ability_modifiers, &mut explanations);
    ground_horizon_walker_class_features(input, &mut explanations);
    ground_nature_warden_class_features(input, &mut explanations);
    ground_rage_prophet_class_features(input, &mut explanations);
    ground_holy_vindicator_class_features(input, &mut explanations);
    ground_stalwart_defender_class_features(input, &ability_modifiers, &mut explanations);
    ground_divine_scion_class_features(input, &mut explanations);

    // SD-34 wave 48 (`decisions.md §22`'s WAVE 48 UPDATE): two more
    // prestige classes in the same "registered, no `ClassId` enum entry,
    // magnitude-only" remainder -- same unconditional placement as the
    // wave 46/47 functions above.
    ground_twilight_talon_class_features(input, &ability_modifiers, &mut explanations);
    ground_golden_legionnaire_class_features(input, &mut explanations);

    // SD-34 wave 49 (`decisions.md §22`'s WAVE 49 UPDATE): 33 more prestige
    // classes in the same "registered, no `ClassId` enum entry,
    // magnitude-only" remainder -- same unconditional placement as the
    // wave 43-48 functions above.
    ground_cyphermage_class_features(input, &mut explanations);
    ground_psychic_fist_class_features(input, &mut explanations);
    ground_asavir_class_features(input, &mut explanations);
    ground_metamorph_class_features(input, &mut explanations);
    ground_war_mind_class_features(input, &mut explanations);
    ground_hellknight_class_features(input, &ability_modifiers, &mut explanations);
    ground_adaptive_warrior_class_features(input, &ability_modifiers, &mut explanations);
    ground_sanguine_angel_class_features(input, &ability_modifiers, &mut explanations);
    ground_body_snatcher_class_features(input, &mut explanations);
    ground_steel_falcon_class_features(input, &mut explanations);
    ground_lantern_bearer_class_features(input, &mut explanations);
    ground_storm_kindler_class_features(input, &mut explanations);
    ground_westcrown_devil_class_features(input, &ability_modifiers, &mut explanations);
    ground_pyrokineticist_class_features(input, &mut explanations);
    ground_aspis_agent_class_features(input, &mut explanations);
    ground_gray_corsair_class_features(input, &mut explanations);
    ground_pathfinder_savant_class_features(input, &mut explanations);
    ground_rivethun_emissary_class_features(input, &ability_modifiers, &mut explanations);
    ground_student_of_war_class_features(input, &ability_modifiers, &mut explanations);
    ground_diabolist_class_features(input, &ability_modifiers, &mut explanations);
    ground_lion_blade_class_features(input, &mut explanations);
    ground_bellflower_tiller_class_features(input, &mut explanations);
    ground_hellknight_signifer_class_features(input, &mut explanations);
    ground_mystic_archer_class_features(input, &mut explanations);
    ground_mammoth_rider_class_features(input, &mut explanations);
    ground_demoniac_class_features(input, &mut explanations);
    ground_master_chymist_class_features(input, &mut explanations);
    ground_enchanting_courtesan_class_features(input, &mut explanations);
    ground_dark_tempest_class_features(input, &mut explanations);
    ground_battle_herald_class_features(input, &mut explanations);
    ground_master_spy_class_features(input, &mut explanations);
    ground_evangelist_class_features(input, &mut explanations);
    ground_ulfen_guard_class_features(input, &mut explanations);

    // SD13-E3 Ranger-only decomposition: split the F6 Ranger non-spell
    // class-feature blocker into three named pillars, and ground Track and
    // combat style for real (Track as a bounded flat numeric value, combat
    // style as a level-gate absence record). This is an extension, never a
    // downgrade, of the F6 surface, mirroring the Paladin decomposition
    // immediately above.
    explain_ranger_level1_chassis_and_class_feature_separation(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_sorcerer_level1_spell_baseline(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    // task #61, 2026-07-28: Draconic Bloodline Dragon Resistances' two flat
    // magnitudes (natural armor bonus, energy resistance bonus). The natural armor
    // bonus is ALSO wired into compute_combat_baseline's shared Armor Class total via
    // apply_sorcerer_draconic_dragon_resistances_ac_bonus_to_combat_baseline, called
    // separately from within that function.
    ground_sorcerer_draconic_bloodline_dragon_resistances(input, &mut explanations, &mut diagnostics);

    explain_wizard_level1_prepared_spell_baseline(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_cleric_level1_spell_baseline(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_druid_level1_spell_baseline(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_bard_level1_spell_baseline(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    // v0.6 Receipt-to-Sheet slice 1, item 4: the caster level for every
    // casting class in the mix. Deliberately unconditional on chassis support
    // and on spell-posture validity -- the caster level is a fact about class
    // and level alone. Paladin and Ranger are excluded here because they
    // already ground the identical arithmetic under their own
    // `partial_caster.effective_caster_level` ids; see
    // `CASTER_LEVEL_RULES`' doc comment.
    ground_caster_level_records(input, &mut explanations);

    explain_human_pilot_race_seam(input, &ability_modifiers, &mut explanations, &mut diagnostics);

    explain_human_trait_bundle(input, &mut explanations, &mut diagnostics);

    explain_dwarf_race_seam(input, &mut explanations, &mut diagnostics);

    explain_elf_race_seam(input, &mut explanations, &mut diagnostics);

    explain_gnome_race_seam(input, &mut explanations, &mut diagnostics);

    explain_half_elf_race_seam(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_half_orc_race_seam(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_halfling_race_seam(input, &mut explanations, &mut diagnostics);
    explain_size_only_race_trait_bundle(input, &mut explanations);
    explain_rougarou_flat_override_race_trait(input, &mut explanations);
    explain_gillman_flat_override_race_trait(input, &mut explanations);
    explain_vanara_flat_override_race_trait(input, &mut explanations);
    explain_samsaran_flat_override_race_trait(input, &mut explanations);
    explain_nagaji_flat_override_race_trait(input, &ability_modifiers, &mut explanations);
    explain_undine_formula_race_trait(input, &ability_modifiers, &mut explanations);

    explain_selected_alternate_racial_traits(input, &mut explanations, &mut diagnostics);

    validate_fighter_feat_choice_legality(input, &mut diagnostics);

    // SD31-W23-CLASSFEATURE-001: the generic GRANT-fact class_feature roster,
    // deliberately called LAST -- after every per-class `explain_*_class_
    // features`/`explain_*_level1_*` call above, most of which run OUTSIDE
    // `compute_class_chassis` (e.g. `explain_fighter_class_features` at this
    // function's own line ~8071, which pushes the real
    // `class_feature.fighter.bravery` explanation well after
    // `compute_class_chassis` already returned). An EARLIER call site inside
    // `compute_class_chassis` itself was tried first and found live-broken:
    // `sd20_contract_level_up_preview.rs::
    // compute_level_up_preview_carries_real_fighter_level_2_grants` failed,
    // because at that earlier point `explain_fighter_class_features` had not
    // run yet, so this module's own already-computed-slug collision guard
    // (see `class_feature_grant_consumer`'s doc comment, section on the
    // Bravery collision) saw an empty set and could not suppress the
    // colliding roster id. Calling this LAST, after every real explanation
    // this function ever pushes, is what makes that guard actually work.
    // Emitted only when `chassis_supported` (this function's own line ~8004)
    // is true -- the brief's "supported chassis" precondition -- and only
    // for the single-class case; see `class_feature_grant_consumer`'s doc
    // comment for the full done-bar this satisfies and the anti-fabrication-
    // gate/open-ended-choice-pool exclusions applied before anything is
    // pushed. Pathfinder Unchained classes already push their own hand-
    // curated roster from inside `compute_pu_class_chassis` (a DIFFERENT id
    // namespace, `class_feature.pu.*`); this call is a no-op for them.
    //
    // SD-34 AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_
    // not_held_by_engine` mechanism, cycle 7): `chassis_supported` ALONE
    // used to gate this whole call, which meant a prestige-class-only
    // character (Assassin, Shadowdancer, Duelist, ...) never got a single
    // one of their own real, corpus-described class features, because
    // `compute_class_chassis`'s own `prestige_class_entry_gate` branch,
    // above, deliberately returns `None` for the BAB/save chassis
    // regardless of whether the class id is real -- "chassis magnitude
    // still unsupported" is not the same claim as "this class does not
    // exist". `prestige_class_entry_gate::is_registered` answers the
    // narrower, correct question this call site actually needs: is
    // `class_level.class_id` a real class id the engine's own census
    // recognizes, whether or not its numeric chassis is built yet. Widening
    // to `||` rather than replacing `chassis_supported` outright preserves
    // every existing modelled class's behaviour unchanged (chassis_supported
    // is still sufficient on its own), and adds coverage only for the
    // registry's own named prestige classes -- never for an arbitrary
    // unrecognized class id, which still grounds nothing here (see
    // `prestige_class_feature_generic_grant_tests::
    // an_unrecognized_class_id_still_grounds_nothing_from_the_widened_gate`).
    if let [class_level] = input.chosen.class_levels.as_slice()
        && (chassis_supported
            || prestige_class_entry_gate::is_registered(&class_level.class_id))
    {
        class_feature_grant_consumer::push_generic_class_feature_grant_records(
            &class_level.class_id,
            class_level.level,
            &ability_modifiers,
            &mut explanations,
        );
    }

    PilotBaseChassisComputation {
        ability_modifiers,
        base_attack_bonus,
        base_saves,
        baseline_melee_attack_bonus,
        baseline_armor_class,
        total_saves,
        selected_skill_modifiers,
        explanations,
        diagnostics,
        sheet_lines: Vec::new(),
    }
}

/// Apply the Human ability-bonus choice's PF1 Core Rulebook +2 racial adjustment
/// to the targeted ability BEFORE ability modifiers are derived, and record the
/// real arithmetic (base score, racial bonus, adjusted score) as an audit trail.
///
/// Gated strictly on `race:human` plus a resolved `choice:human_ability_bonus`
/// selection. Every other race's chosen ability scores pass through completely
/// unchanged here: Dwarf, Elf, Gnome, Half-Elf, Half-Orc, and Halfling fixtures
/// all rely on the opposite, already-documented convention (see
/// `explain_dwarf_race_seam` and `explain_half_elf_race_seam`) that the chosen
/// score already bakes in their fixed/chosen racial adjustment, so applying any
/// further arithmetic to their scores here would double-count the bonus. Human's
/// own deterministic pilot fixture and the real PCGen oracle both confirm Human
/// is the one race whose chosen score is the PRE-bonus base — see the CG-03
/// follow-up receipt for the oracle evidence.
/// `pub` (rather than private) so this crate's own `contract.rs` (task 5's
/// encumbrance wiring) and the downstream `apps/desktop/src-tauri` crate's
/// `character_hub.rs` (item 2's durability wiring -- a separate crate,
/// where `pub(crate)` would not reach) can both derive the character's
/// real *effective* ability scores (post-racial-bonus) without re-deriving
/// this same Human-bonus arithmetic a second time, or falling back to the
/// pre-bonus raw scores, which would be wrong for a Human whose chosen
/// ability-bonus target is the relevant ability.
pub fn apply_human_ability_bonus(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) -> AbilityScores {
    let scores = input.chosen.ability_scores.clone();

    if input.chosen.race_id != HUMAN_RACE_ID {
        return scores;
    }

    let Some(selection) = choice_selection(input, HUMAN_ABILITY_BONUS_CHOICE_ID) else {
        return scores;
    };

    let ability = selection
        .strip_prefix(ABILITY_SELECTION_PREFIX)
        .unwrap_or(selection)
        .to_owned();

    let Some(base_score) = ability_score_for(&scores, &ability) else {
        // An unrecognized ability target names nothing to adjust; leave scores
        // untouched rather than fabricating an adjustment.
        return scores;
    };

    let adjusted_score = base_score + HUMAN_ABILITY_BONUS_MAGNITUDE;
    let scores = with_ability_score(scores, &ability, adjusted_score);

    explanations.push(ComputationExplanation {
        id: "race.human.ability_bonus_applied".to_owned(),
        value: adjusted_score,
        detail: format!(
            "Human ability-bonus selection ({HUMAN_ABILITY_BONUS_CHOICE_ID} -> {selection}) applies \
             the PF1 Core Rulebook Standard Human +2 racial bonus to {ability} BEFORE ability \
             modifiers are derived: base chosen score {base_score} + \
             {HUMAN_ABILITY_BONUS_MAGNITUDE} racial = {adjusted_score}. This differs from \
             Dwarf/Half-Elf/etc., whose fixtures pre-bake their own racial adjustment into the \
             chosen score itself (no arithmetic performed on those seams); Human's chosen score \
             is confirmed PRE-bonus by both the deterministic pilot fixture and the real PCGen \
             oracle output."
        ),
    });

    scores
}

/// Look up the chosen score for a named ability. Unknown ability names yield
/// `None` rather than fabricating a value.
pub(super) fn ability_score_for(scores: &AbilityScores, ability: &str) -> Option<i16> {
    match ability {
        "strength" => Some(scores.strength),
        "dexterity" => Some(scores.dexterity),
        "constitution" => Some(scores.constitution),
        "intelligence" => Some(scores.intelligence),
        "wisdom" => Some(scores.wisdom),
        "charisma" => Some(scores.charisma),
        _ => None,
    }
}

/// Return a copy of `scores` with the named ability's score replaced. Unknown
/// ability names return `scores` unchanged rather than fabricating a field.
pub(super) fn with_ability_score(mut scores: AbilityScores, ability: &str, new_score: i16) -> AbilityScores {
    match ability {
        "strength" => scores.strength = new_score,
        "dexterity" => scores.dexterity = new_score,
        "constitution" => scores.constitution = new_score,
        "intelligence" => scores.intelligence = new_score,
        "wisdom" => scores.wisdom = new_score,
        "charisma" => scores.charisma = new_score,
        _ => {}
    }
    scores
}

pub(super) fn compute_ability_modifiers(
    scores: &AbilityScores,
    explanations: &mut Vec<ComputationExplanation>,
) -> AbilityModifiers {
    let abilities = [
        ("strength", scores.strength),
        ("dexterity", scores.dexterity),
        ("constitution", scores.constitution),
        ("intelligence", scores.intelligence),
        ("wisdom", scores.wisdom),
        ("charisma", scores.charisma),
    ];

    let mut modifiers = AbilityModifiers::default();
    for (ability, score) in abilities {
        let modifier = ability_modifier(score);
        explanations.push(ComputationExplanation {
            id: format!("ability_modifier.{ability}"),
            value: modifier,
            detail: format!(
                "{ability} ability modifier from chosen score {score}: floor({score} / 2) - 5 = {modifier}"
            ),
        });
        assign_modifier(&mut modifiers, ability, modifier);
    }

    modifiers
}

/// Applies Barbarian Rage's Strength/Constitution morale bonus to `base`
/// when a valid, active, in-budget Rage activation is present for this
/// character (v0.6 alpha swarm, risks item 8). Mirrors `compute_total_saves`'s
/// existing pattern of layering `feat_effects::save_bonuses_from_feats`'s
/// separately-computed result onto a base total, rather than baking this
/// into `compute_ability_modifiers` itself -- `compute_ability_modifiers` is
/// called for every class, every request, and stays completely untouched.
/// Class-ownership-gated by construction: `active_barbarian_rage_bonus` only
/// returns `Some` when `class_levels` actually contains Barbarian, so a
/// non-Barbarian character's stray `class_ability_activations` entry for
/// `BARBARIAN_RAGE_ABILITY_ID` is never read at all.
///
/// The Strength/Constitution ability-SCORE bonus (+4/+6/+8, always even)
/// becomes exactly half that as an ability-MODIFIER bonus (+2/+3/+4):
/// modifier = floor((score - 10) / 2), and adding an even N to score adds
/// exactly N/2 to the floored modifier regardless of the original score's
/// parity.
pub(super) fn apply_rage_ability_bonuses(
    base: AbilityModifiers,
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) -> AbilityModifiers {
    let Some((barbarian_level, strength_bonus, constitution_bonus, will_save_bonus, rage_source_feature)) =
        active_barbarian_rage_bonus(input, &base)
    else {
        return base;
    };

    let strength_modifier_bonus = strength_bonus / 2;
    let constitution_modifier_bonus = constitution_bonus / 2;
    let boosted = AbilityModifiers {
        strength: base.strength + strength_modifier_bonus,
        constitution: base.constitution + constitution_modifier_bonus,
        ..base
    };
    explanations.push(ComputationExplanation {
        id: "ability_modifier.barbarian.rage_bonus_applied".to_owned(),
        value: strength_modifier_bonus,
        detail: format!(
            "Barbarian level {barbarian_level} {rage_source_feature} applied to ability \
             modifiers: +{strength_bonus} Strength / +{constitution_bonus} Constitution morale \
             bonus (ability score) is +{strength_modifier_bonus} Strength modifier / \
             +{constitution_modifier_bonus} Constitution modifier (an even score bonus always \
             halves exactly onto the floored modifier). Applied only while actively, validly \
             raging; the Will-save bonus (+{will_save_bonus}) is layered onto compute_total_saves \
             separately, and the {BARBARIAN_RAGE_ARMOR_CLASS_PENALTY} Armor Class penalty onto \
             compute_combat_baseline separately"
        ),
    });
    boosted
}

/// Pathfinder ability modifier: `floor(score / 2) - 5`. `div_euclid` gives true
/// floor division so negative scores would round down rather than toward zero.
/// `pub` (rather than private) so `apps/desktop/src-tauri`'s
/// `character_hub.rs` durability commands (v0.6 alpha swarm, item 2, a
/// separate downstream crate -- `pub(crate)` would not reach it) can derive
/// the real Constitution modifier from an effective ability score without
/// re-deriving this same formula a second time.
pub fn ability_modifier(score: i16) -> i16 {
    score.div_euclid(2) - 5
}

/// Builds a full [`AbilityModifiers`] straight from the character's raw
/// [`AbilityScores`], without pushing any `ComputationExplanation` --
/// `compute_ability_modifiers` (above) does that plus the six baseline
/// explanation records, but several call sites (SD-32 T12 Epic 8's generic
/// pool-choice magnitude resolver) need only the plain struct to seed a
/// formula evaluator's variable environment, not a second copy of the six
/// already-pushed baseline explanations.
pub(super) fn ability_modifiers_from_scores(scores: &AbilityScores) -> AbilityModifiers {
    AbilityModifiers {
        strength: ability_modifier(scores.strength),
        dexterity: ability_modifier(scores.dexterity),
        constitution: ability_modifier(scores.constitution),
        intelligence: ability_modifier(scores.intelligence),
        wisdom: ability_modifier(scores.wisdom),
        charisma: ability_modifier(scores.charisma),
    }
}

pub(super) fn assign_modifier(modifiers: &mut AbilityModifiers, ability: &str, modifier: i16) {
    match ability {
        "strength" => modifiers.strength = modifier,
        "dexterity" => modifiers.dexterity = modifier,
        "constitution" => modifiers.constitution = modifier,
        "intelligence" => modifiers.intelligence = modifier,
        "wisdom" => modifiers.wisdom = modifier,
        "charisma" => modifiers.charisma = modifier,
        _ => unreachable!("ability set is fixed and fully matched"),
    }
}


pub(super) const DWARF_RACE_ID: &str = "race:dwarf";

/// Each PF1 Core Dwarf standard racial trait's own replace-flag, verbatim from
/// the `!PREFACT:1,ABILITIES,<flag>=True` gate the corpus row declares
/// (`core_essentials/races/dwarf/dwarf_abilities_race.lst`; the ingested
/// records' `suppressed_by_flag`). `decisions.md §26`: a standard trait applies
/// **iff** no selected alternate has set its flag, so these are the exact
/// strings that decide whether each record below is emitted.
/// `tests/sd27_alternate_racial_trait_reachability.rs` pins every one of them
/// against the on-disk corpus, so a renamed flag is a failing test rather than
/// a record that silently stops swapping.
pub(super) const DWARF_REPLACE_VISION_FLAG: &str = "Dwarf_ReplaceVision";

pub(super) const DWARF_REPLACE_STONECUNNING_FLAG: &str = "Dwarf_ReplaceStonecunning";

pub(super) const DWARF_REPLACE_GREED_FLAG: &str = "Dwarf_ReplaceGreed";

pub(super) const DWARF_REPLACE_HARDY_FLAG: &str = "Dwarf_ReplaceHardy";

pub(super) const DWARF_REPLACE_STABILITY_FLAG: &str = "Dwarf_ReplaceStability";

pub(super) const DWARF_REPLACE_DEFENSIVE_TRAINING_FLAG: &str = "Dwarf_ReplaceDefensiveTraining";

/// ARG's `Dwarf ~ Minesight` (`arg_abilities_race.lst:39`, ARG p.12): the one
/// Dwarf alternate that replaces a standard trait this engine grounds a
/// *number* for, and replaces it with a different number —
/// `VISION:Darkvision (90)` against the CRB row's `Darkvision (60)`.
pub(super) const DWARF_MINESIGHT_TRAIT_KEY: &str = "Dwarf ~ Minesight";

pub(super) const DWARF_MINESIGHT_DARKVISION_FEET: i16 = 90;

pub(super) const DWARF_SIZE_CATEGORY: &str = "Medium";

pub(super) const DWARF_BASE_SPEED_FEET: i16 = 20;

pub(super) const DWARF_DARKVISION_FEET: i16 = 60;

pub(super) const DWARF_CON_ADJUSTMENT: i16 = 2;

pub(super) const DWARF_CHA_ADJUSTMENT: i16 = -2;

/// v0.6 alpha swarm (QA-found systemic parity gap, same shape as Elf's
/// missing +2 Intelligence): verified against the real PCGen corpus,
/// `core_essentials/races/dwarf/dwarf_abilities_race.lst:18`'s "Dwarf
/// Racial Default" ability-score row (not an alternate) --
/// `BONUS:STAT|CON,WIS|2|TYPE=Racial`, `BONUS:STAT|CHA|-2|TYPE=Racial`: +2
/// Constitution, +2 Wisdom, -2 Charisma. This engine only grounded CON/CHA;
/// Wisdom was missing entirely, not even flagged as an alternate.
pub(super) const DWARF_WIS_ADJUSTMENT: i16 = 2;

/// PF1 Core Rulebook Dwarf Stonecunning flat Perception situational-bonus
/// magnitude (verified against `dwarf_abilities_race.lst:27`'s
/// `BONUS:SITUATION|Perception=to notice unusual stonework|2|TYPE=Racial`
/// and `dwarf_skills.lst:6`'s `Perception.MOD SITUATION:to notice unusual
/// stonework`). Distinct from the separate Greed (Appraise) racial trait.
pub(super) const DWARF_STONECUNNING_PERCEPTION_BONUS: i16 = 2;

/// PF1 Core Rulebook Dwarf Greed flat Appraise situational-bonus magnitude
/// (verified against `dwarf_abilities_race.lst:23`'s
/// `BONUS:SITUATION|Appraise=to assess nonmagical metals or gemstones|2|TYPE=Racial`
/// and `dwarf_skills.lst:5`'s `Appraise.MOD SITUATION:to assess nonmagical
/// metals or gemstones`). Distinct from the already-grounded Stonecunning
/// (Perception) racial trait.
pub(super) const DWARF_GREED_APPRAISE_BONUS: i16 = 2;

/// PF1 Core Rulebook Dwarf Hardy flat racial saving-throw-bonus magnitude
/// against poison, spells, and spell-like abilities (verified against
/// `dwarf_abilities_race.lst:25`'s
/// `BONUS:VAR|SaveBonus_vs_Poison|2|TYPE=Racial` and
/// `BONUS:VAR|SaveBonus_vs_Spells|2|TYPE=Racial`). Both save categories
/// share the same flat +2 magnitude, mirroring the already-grounded Elf
/// Elven Immunities enchantment-save-bonus idiom.
pub(super) const DWARF_HARDY_SAVE_BONUS: i16 = 2;

/// PF1 Core Rulebook Dwarf Stability flat racial Combat Maneuver Defense
/// bonus magnitude against bull rush and trip attempts made while the Dwarf
/// is standing on the ground (verified against
/// `dwarf_abilities_race.lst:26`'s
/// `BONUS:VAR|CMD_BullRush,CMD_Trip|4|TYPE=Racial`). Both maneuver
/// categories share the same flat +4 magnitude, mirroring the already-
/// grounded Dwarf Hardy two-save-category flat-bonus idiom exactly.
pub(super) const DWARF_STABILITY_CMD_BONUS: i16 = 4;

/// PF1 Core Rulebook Dwarf Defensive Training flat racial dodge-bonus-to-AC
/// magnitude against monsters of the giant subtype (verified against
/// `dwarf_abilities_race.lst:22`'s
/// `BONUS:VAR|RacialDefensiveTrainingBonus|4`). A single flat magnitude
/// applied to a single named derived-stat target, mirroring the already-
/// grounded Dwarf Stability flat-bonus idiom exactly.
pub(super) const DWARF_DEFENSIVE_TRAINING_DODGE_BONUS: i16 = 4;


pub(super) const ELF_RACE_ID: &str = "race:elf";

pub(super) const ELF_SIZE_CATEGORY: &str = "Medium";

pub(super) const ELF_BASE_SPEED_FEET: i16 = 30;

pub(super) const ELF_DEX_ADJUSTMENT: i16 = 2;

pub(super) const ELF_CON_ADJUSTMENT: i16 = -2;

/// v0.6 alpha swarm (QA-found real PCGen-parity gap): this record's own
/// detail text used to call +2 Intelligence "the alternate PF1 +2
/// Intelligence Elf variant... out of scope" -- checked against the real
/// PCGen corpus (not memory) and that framing was wrong. `cr_races.lst`'s
/// `Elf.MOD` is only a source-page citation; the real base race data lives
/// in a separate PCC pack, `elf_abilities_race.lst:18` (
/// `core_essentials/races/elf/elf_abilities_race.lst`), whose ability-score
/// row is explicitly typed `TYPE:...Elf Racial Default...` (not an
/// alternate/optional variant) and carries `BONUS:STAT|DEX,INT|2|TYPE=Racial`
/// alongside `BONUS:STAT|CON|-2|TYPE=Racial` -- +2 Intelligence is the
/// CRB-standard default, not an alternate. Matters now specifically because
/// Elf Wizard is a real reachable combination (v0.6 alpha swarm) and
/// Intelligence drives a Wizard's spell save DC and bonus spells, so
/// omitting it silently shorted an Elf Wizard's casting stat by 2.
pub(super) const ELF_INT_ADJUSTMENT: i16 = 2;


pub(super) const GNOME_RACE_ID: &str = "race:gnome";

pub(super) const GNOME_SIZE_CATEGORY: &str = "Small";

pub(super) const GNOME_BASE_SPEED_FEET: i16 = 20;

pub(super) const GNOME_CON_ADJUSTMENT: i16 = 2;

pub(super) const GNOME_STR_ADJUSTMENT: i16 = -2;

/// v0.6 alpha swarm (QA-found systemic parity gap, same shape as Elf's
/// missing +2 Intelligence): verified against the real PCGen corpus,
/// `core_essentials/races/gnome/gnome_abilities_race.lst:18`'s "Gnome
/// Racial Default" ability-score row (not an alternate) --
/// `BONUS:STAT|CON,CHA|2|TYPE=Racial`, `BONUS:STAT|STR|-2|TYPE=Racial`: +2
/// Constitution, +2 Charisma, -2 Strength. This engine only grounded
/// CON/STR; Charisma was missing entirely, not even flagged as an
/// alternate.
pub(super) const GNOME_CHA_ADJUSTMENT: i16 = 2;


pub(super) const HALF_ELF_RACE_ID: &str = "race:half-elf";

pub(super) const HALF_ELF_SIZE_CATEGORY: &str = "Medium";

pub(super) const HALF_ELF_BASE_SPEED_FEET: i16 = 30;

pub(super) const HALF_ELF_ABILITY_BONUS_CHOICE_ID: &str = "choice:half_elf_ability_bonus";

/// ARG's `Half-Elf ~ Dual Minded` (`arg_abilities_race.lst:158`, ARG p.42).
///
/// The one alternate racial trait across all 153 whose declared bonus lands on
/// a **saving throw** this engine totals: its whole mechanical body is
/// `BONUS:SAVE|Will|2`, with no situational qualifier and no PCGen variable,
/// against `compute_total_saves`' real `total_saves.will`. The only other ten
/// that reach a computed total at all are skill bonuses, handled by
/// [`ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES`].
///
/// Derived by scanning every ARG alternate's declared bonus chains
/// (`pcgen_import::ingest_record::bonus_chain_qualifiers`) against the
/// engine's computed-total surface, not asserted —
/// `tests/sd27_alternate_racial_trait_reachability.rs` re-runs that scan and
/// pins the whole set of eleven.
///
/// **It stacks, unlike the ten skill bonuses.** The corpus chain carries no
/// `TYPE=` token at all, so this is an untyped bonus and PF1's
/// same-type-does-not-stack rule does not apply to it; it is added, not
/// maximised.
///
/// The standard trait it replaces is `Half-Elf ~ Adaptability` (a Skill Focus
/// bonus feat), which this engine grounds no record for, so this is a pure
/// addition rather than a swap of one number for another.
/// Every alternate racial trait that declares a plain-integer `BONUS:SAVE` on
/// a saving throw `compute_total_saves` actually totals, as
/// `(trait key, owning race id, Fortitude, Reflex, Will)`.
///
/// # This list is a measurement, not a selection
///
/// The sibling of [`ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES`], and derived the
/// same way: `tests/sd27_alternate_racial_trait_reachability.rs` rescans every
/// alternate's declared bonus chains
/// (`pcgen_import::ingest_record::bonus_chain_qualifiers`) against the
/// engine's computed-total surface
/// and fails, naming the trait, if this table and the corpus disagree in
/// either direction.
///
/// **This used to be a single hardcoded constant** for ARG's
/// `Half-Elf ~ Dual Minded`, documented as "the one alternate across all 153
/// whose declared bonus lands on a saving throw". That was true of ARG and
/// stopped being true the moment SD-29's race-trait lane added books: Inner Sea
/// Races' `Dwarf ~ Unstoppable` and Horror Adventures' `Half-Elf ~ Mismatched`
/// both land here, and with the constant in place both were offered in the
/// picker, persisted on the character, and moved no number on the sheet — the
/// browse-only stub class `decisions.md §44.2` describes. Round 3 replaced the
/// constant with this table (`decisions.md §47`).
///
/// # Stacking
///
/// The contributions are **summed**, and the pinned invariant that makes that
/// correct is that no race contributes twice to the same save — asserted by
/// `no_race_contributes_two_alternate_trait_bonuses_to_one_save` below, so a
/// future book that breaks it fails here rather than silently choosing between
/// summing a typed pair (wrong) and maximising a penalty away (also wrong).
/// `Half-Elf ~ Mismatched`'s **negative** entry is why maximising is not the
/// default the skill table uses: a -2 penalty must apply, never be discarded
/// as "not the highest".
pub(super) const ALTERNATE_TRAIT_SAVE_BONUSES: &[(&str, &str, i16, i16, i16)] = &[
    // (trait key, race id, Fortitude, Reflex, Will)
    //
    // ARG p.42, `arg_abilities_race.lst:158`, `BONUS:SAVE|Will|2`. Untyped in
    // the corpus (no `TYPE=` token at all), so PF1's same-type rule does not
    // reach it. Replaces `Half-Elf ~ Adaptability`, a bonus feat this engine
    // grounds no record for, so it is a pure addition rather than a swap.
    ("Half-Elf ~ Dual Minded", "race:half-elf", 0, 0, 2),
    // Inner Sea Races, `BONUS:SAVE|Fortitude|1|TYPE=Racial`.
    ("Dwarf ~ Unstoppable", "race:dwarf", 1, 0, 0),
    // Horror Adventures, `BONUS:SAVE|Reflex|-2`. The only negative magnitude
    // in either alternate-trait bonus table. Its chain also carries
    // `BONUS:COMBAT|INITIATIVE|4|TYPE=Racial`, which this engine totals no
    // initiative for and therefore deliberately does not model here.
    ("Half-Elf ~ Mismatched", "race:half-elf", 0, -2, 0),
    // SD-31-E6-F4-003 (ARG's own Strix chassis batch, 2026-08-16), wired
    // SD31-W9-INTEGRATE-001 (`arg_abilities_race.lst:1149-1153`): both
    // `BONUS:SAVE|<Save>|1|TYPE=Racial`, untyped, landing on two DIFFERENT
    // saves from Strix's own skill-bonus pair below.
    ("Strix ~ Nimble", "race:strix", 0, 1, 0),
    ("Strix ~ Tough", "race:strix", 1, 0, 0),
];

pub(super) const HALF_ORC_SIZE_CATEGORY: &str = "Medium";

pub(super) const HALF_ORC_BASE_SPEED_FEET: i16 = 30;

pub(super) const HALF_ORC_DARKVISION_FEET: i16 = 60;

pub(super) const HALF_ORC_ABILITY_BONUS_CHOICE_ID: &str = "choice:half_orc_ability_bonus";

pub(super) const HALF_ORC_INTIMIDATING_BONUS: i16 = 2;


pub(super) const HALFLING_RACE_ID: &str = "race:halfling";

pub(super) const HALFLING_SIZE_CATEGORY: &str = "Small";

pub(super) const HALFLING_BASE_SPEED_FEET: i16 = 20;

pub(super) const HALFLING_DEX_ADJUSTMENT: i16 = 2;

pub(super) const HALFLING_STR_ADJUSTMENT: i16 = -2;

/// v0.6 alpha swarm (QA-found systemic parity gap, same shape as Elf's
/// missing +2 Intelligence): verified against the real PCGen corpus,
/// `core_essentials/races/halfling/halfling_abilities_race.lst:18`'s
/// "Halfling Racial Default" ability-score row (not an alternate) --
/// `BONUS:STAT|DEX,CHA|2|TYPE=Racial`, `BONUS:STAT|STR|-2|TYPE=Racial`: +2
/// Dexterity, +2 Charisma, -2 Strength. This engine only grounded DEX/STR;
/// Charisma was missing entirely, not even flagged as an alternate.
pub(super) const HALFLING_CHA_ADJUSTMENT: i16 = 2;

pub(super) const HALFLING_FEARLESS_SAVE_VS_FEAR_BONUS: i16 = 2;

pub(super) const HALFLING_LUCK_ALL_SAVES_BONUS: i16 = 1;


/// SD31-W25-RACETRAIT-001 — the "race-trait flat-override compute seam" the
/// Bestiary 6 ledger named as missing (`artifacts/BESTIARY-6-LEDGER.md`,
/// `Rougarou ~ Speed`/`~ Vision`/`~ Natural Weapon` rows) and `OPEN-ISSUES.md`
/// row 353 scoped as a cross-book lever: `race_ids_with_a_magnitude_consumer`'s
/// three existing seams (CRB's 7 `explain_*_race_seam` functions, the two
/// `ALTERNATE_TRAIT_*` tables, `SIZE_ONLY_RACE_TRAIT_BUNDLE`) cover no flat
/// speed/vision/natural-weapon override for a non-CRB race, so a `computed`
/// wiring-class record stating one (`MOVE:Walk,30`, `VISION:Low-Light
/// Vision`, a fixed `DAMAGESIZE` natural attack) stayed `ingested-magnitude`
/// forever, however reachable.
///
/// Three races below, each hand-written (unlike `SIZE_ONLY_RACE_TRAIT_BUNDLE`,
/// their shapes genuinely differ — Gillman also has a Swim speed, Rougarou
/// also has a natural weapon, Vanara also has a Climb speed), mirroring the
/// exact idiom `explain_dwarf_race_seam` already uses for Dwarf's own base
/// speed and Darkvision: the transcribed value IS the fact reported, grounded
/// standalone, never folded into a movement/vision total this engine does not
/// have (no such total exists anywhere in this codebase, verified before
/// writing this — `grep -rn "base_speed\|land_speed" src/rules_core` outside
/// this new block returns nothing, and neither `PilotBaseChassisComputation`
/// nor `SelectedSkillModifiers` carries a speed or vision field).
///
/// Each race's racial-DEFAULT record and, where the corpus offers one, its
/// selectable ALTERNATE override are both handled — the alternate check reuses
/// [`replaced_by_alternate_trait`], the same generic flag lookup Dwarf's
/// Minesight branch above already calls, over `race_resolver.rs`'s own
/// `ALTERNATE_TRAIT_REPLACE_FLAGS` table (not re-implemented, not touched).
pub(super) const ROUGAROU_RACE_ID: &str = "race:rougarou";

pub(super) const ROUGAROU_BASE_SPEED_FEET: i16 = 30;

pub(super) const ROUGAROU_BITE_DAMAGE_DIE: i16 = 4;

pub(super) const GILLMAN_RACE_ID: &str = "race:gillman";

pub(super) const GILLMAN_BASE_SPEED_FEET: i16 = 30;

pub(super) const GILLMAN_SWIM_SPEED_FEET: i16 = 30;

pub(super) const GILLMAN_REPLACE_SPEED_FLAG: &str = "Gillman_ReplaceSpeed";

pub(super) const GILLMAN_THROWBACK_TRAIT_KEY: &str = "Gillman ~ Throwback";

pub(super) const THROWBACK_GILLMAN_SPEED_FEET: i16 = 30;

pub(super) const VANARA_RACE_ID: &str = "race:vanara";

pub(super) const VANARA_BASE_SPEED_FEET: i16 = 30;

pub(super) const VANARA_CLIMB_SPEED_FEET: i16 = 20;

pub(super) const VANARA_REPLACE_SPEED_FLAG: &str = "Vanara_ReplaceSpeed";

/// SD31-W27-RACETRAIT-001 — Samsaran (Advanced Race Guide), extending the
/// flat-override compute seam per `race_ids_with_a_magnitude_consumer`'s own
/// doc comment. Samsaran is single-book (`samsaran_abilities_race.lst`, no
/// cross-book alternate traits found: `grep -rl '"Samsaran ~' data/corpus`
/// returns exactly the 9 records below), so the entire reachable
/// `computed`-wiring-class population is accounted for here, mirroring
/// Rougarou's full-coverage precedent exactly (`wave 26 OPEN-ISSUES.md` row
/// 365 records the opposite, partial-coverage shape as the GAMED finding
/// this cycle avoids).
///
/// Covered here (all four not-`done` `computed` records at time of writing):
/// `~ Speed` (`samsaran_abilities_race.lst:17`, `BONUS:VAR|MOVEBASE|30`),
/// `~ Vision` (`:18`, `BONUS:VAR|HasRacialVision|1`, low-light vision, a
/// binary trait not a distance magnitude), `~ Lifebound` (`:19`, `BONUS:
/// VAR|SaveBonus_vs_DeathEffects,SaveBonus_vs_NegativeEnergy,
/// FortSave_vs_NegativeLevels|2|TYPE=Racial` — a save bonus CONDITIONAL on
/// death effects / negative energy / negative levels, one of Decision 7
/// REFINED's own named condition families), and `~ Shards of the Past`
/// (`:21`, `BONUS:SKILL|LIST|2|TYPE=Racial` — a player-chosen pair of
/// skills this engine models no chooser for, exactly the `%LIST` shape
/// `ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES`'s own doc comment already
/// declines to guess at).
///
/// `~ Ability Scores` is NOT re-covered here — it already grounds through
/// [`probe_race_creation_roster`]'s generic ability-adjustment chassis
/// consumer (`race_creation_chassis`), confirmed via `docs/work-inventory.
/// json` showing it `grounded` before this seam existed. `~ Languages`,
/// `~ Samsaran Magic`, `~ Size`, `~ Type` are zero-magnitude prose already
/// `text-complete` under Decision 7. `~ Mystic Past Life` and `~
/// Mountaineer` are genuinely NOT ingested (no JSON record exists under
/// `data/corpus/**/race_trait/samsaran/` for either — a book-ingestion gap,
/// not a compute-seam gap) and are unaffected by this seam either way,
/// since [`probe_race_trait_corpus`] only observes records the race-corpus
/// load actually finds.
///
/// No save-bonus-vs-condition-type or arbitrary-skill-bonus consuming total
/// exists anywhere in this codebase (`grep -rn
/// "SaveBonus_vs_DeathEffects\|SaveBonus_vs_NegativeEnergy" src/rules_core`
/// outside this block returns nothing; `SelectedSkillModifiers` carries
/// only Climb/Intimidate/Swim, confirmed above this file's own struct
/// definition), so Lifebound and Shards of the Past are grounded as
/// standalone recognition values, the same posture Nagaji's `~ Resistant`
/// and `~ Serpent's Sense` take immediately below.
pub(super) const SAMSARAN_RACE_ID: &str = "race:samsaran";

pub(super) const SAMSARAN_BASE_SPEED_FEET: i16 = 30;

pub(super) const SAMSARAN_LIFEBOUND_SAVE_BONUS: i16 = 2;

pub(super) const SAMSARAN_SHARDS_OF_THE_PAST_SKILL_BONUS: i16 = 2;

/// SD31-W27-RACETRAIT-001 — Nagaji (Advanced Race Guide), extending the
/// flat-override compute seam alongside Samsaran above. Single-book
/// (`nagaji_abilities_race.lst`, `grep -rl '"Nagaji ~' data/corpus` returns
/// exactly the 10 records below, plus `Nagaji ~ Serpent Affinity` — an
/// Inner Sea Races alternate with NO JSON record under
/// `data/corpus/**/race_trait/nagaji/` at all, a book-ingestion gap, not a
/// compute-seam gap, unaffected either way since `probe_race_trait_corpus`
/// only observes records the race-corpus load actually finds).
///
/// Covered here (all six not-`done` `computed` records at time of writing):
/// `~ Speed` (`:17`, `BONUS:VAR|MOVEBASE|30`), `~ Vision` (`:18`,
/// `BONUS:VAR|HasRacialVision|1`, binary), `~ Armored Scales` (`:19`,
/// `BONUS:VAR|AC_Natural_Armor|1|TYPE=Base` — a UNIVERSAL, unconditional
/// natural armor bonus. A real natural-armor consuming total DOES exist in
/// this codebase (`FeatDerivedPillarContributions::natural_armor_bonus`,
/// summed into both the armor-class total and touch-AC exclusion, and
/// already accepting contributions from three other sources — Alchemist
/// Mutagen, Sorcerer Draconic Bloodline, ARG Armor of the Pit); this race
/// trait is not wired into it (that is a real, separate follow-on, logged
/// to `OPEN-ISSUES.md`), so it is grounded here as a standalone recognition
/// value rather than folded into a total it does not yet feed — corrected
/// during the wave-27 integration cycle after adversarial review found the
/// original comment's "no consuming total exists anywhere" claim false),
/// `~ Resistant` (`:20`, `BONUS:VAR|
/// SaveBonus_vs_MindAffecting,SaveBonus_vs_Poison|2|TYPE=Racial` —
/// CONDITIONAL, an effect-type subset per Decision 7 REFINED), `~
/// Serpent's Sense` (`:21`, `BONUS:SITUATION|Handle Animal=against
/// reptiles|2` and `BONUS:SKILL|Perception|2|TYPE=Racial` — Handle Animal
/// and Perception are both named in `ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES`'s
/// own doc comment as skills "this codebase computes no total for"), and `~
/// Hypnotic Gaze` (`arg_abilities_race.lst:985`, DESC-stated formula, no
/// structured `BONUS:`/`DEFINE:` token: "the DC of this effect is equal to
/// 11 + the nagaji's Charisma modifier" and "caster level equal to the
/// nagaji's Hit Dice" — a REAL, hand-modelled two-value computation below,
/// not a recognition record, mirroring the existing `alchemist_extract_
/// save_dc`/`witch_hex_save_dc` DC-function idiom this file already uses
/// throughout for class features).
///
/// **`~ Hypnotic Gaze` is an ALTERNATE trait, not a default one — fixed
/// during the wave-27 integration cycle.** `nagaji_hypnotic_gaze.json` sets
/// `is_racial_default: false` and `sets_replace_flags:
/// ["Nagaji_ReplaceSerpentsSense"]`; `race_resolver.rs`'s
/// `ALTERNATE_TRAIT_REPLACE_FLAGS` table registers `("Nagaji ~ Hypnotic
/// Gaze", &["Nagaji_ReplaceSerpentsSense"])`. It REPLACES `~ Serpent's
/// Sense`, the same shape Gillman's Throwback and Vanara's Tree Stranger
/// use for Speed above — selecting it should suppress Serpent's Sense and
/// emit Hypnotic Gaze instead; leaving both unselected should keep
/// Serpent's Sense as the default and emit no Hypnotic Gaze record at all.
/// The original version of this function emitted Hypnotic Gaze
/// unconditionally for every nagaji regardless of selection, alongside
/// Serpent's Sense (the trait it is supposed to replace) — gated below via
/// `replaced_by_alternate_trait`, the exact function Gillman/Vanara already
/// use for this shape.
///
/// `~ Ability Scores` already grounds through the generic ability-adjustment
/// chassis consumer (`probe_race_creation_roster`), same as Samsaran.
/// `~ Languages`, `~ Size`, `~ Type` are zero-magnitude prose already
/// `text-complete` under Decision 7.
pub(super) const NAGAJI_RACE_ID: &str = "race:nagaji";

pub(super) const NAGAJI_BASE_SPEED_FEET: i16 = 30;

pub(super) const NAGAJI_ARMORED_SCALES_NATURAL_ARMOR: i16 = 1;

pub(super) const NAGAJI_RESISTANT_SAVE_BONUS: i16 = 2;

pub(super) const NAGAJI_SERPENTS_SENSE_SKILL_BONUS: i16 = 2;

pub(super) const NAGAJI_REPLACE_SERPENTS_SENSE_FLAG: &str = "Nagaji_ReplaceSerpentsSense";

/// PF1 SLA save DC base: `11 + the nagaji's Charisma modifier`, transcribed
/// verbatim from `nagaji_hypnotic_gaze.json`'s own `description` field
/// (`arg_abilities_race.lst:985`'s `DESC:` token) — a real formula stated in
/// the corpus's own prose, not guessed at.
pub(super) const NAGAJI_HYPNOTIC_GAZE_DC_BASE: i16 = 11;

/// `11 + the nagaji's Charisma modifier`, the exact formula
/// `nagaji_hypnotic_gaze.json`'s `description` states.
pub(super) fn nagaji_hypnotic_gaze_dc(charisma_modifier: i16) -> i16 {
    NAGAJI_HYPNOTIC_GAZE_DC_BASE + charisma_modifier
}

// -------------------------------------------------------------------------------------------
// SD31-W26-RACETRAIT-001 — the race-trait FORMULA compute seam, folded into SD-33 per
// `docs/release/SD-31-corpus-closure-grind/artifacts/OPEN-ISSUES.md` row 365's remediation
// path (a): the seam + fixtures are real, correct, reusable work (reviewer-confirmed); only
// the race-level `FORMULA_RACE_TRAIT_RACES` doneness-credit const from the original branch was
// the gaming vector, and it is deliberately NOT ported here — this seam is wired unconditionally
// (below, next to the other race seam calls) but `race_ids_with_a_magnitude_consumer` is left
// untouched, banking 0 board-credit units per the operator's own instruction.
// -------------------------------------------------------------------------------------------
//
// The flat-override seam above handles literal, unconditional magnitudes (`MOVE:Walk,30`, a
// fixed `DAMAGESIZE`). `formula_interpreter` (`OPERATOR-RULINGS-2026-08-21.md` §20) built a real
// evaluator for the PCGen `BONUS:VAR`/`DEFINE` arithmetic grammar; this block is a consumer of
// it: Undine's three selectable alternate racial traits (replacing the racial-default Spell-Like
// Ability trait) each state a real formula over total character level (`TL`) and one ability
// modifier.
//
// **The formula text is the single source of truth for BOTH the compute path and its own fixture
// gate**: [`UNDINE_RACE_TRAIT_FORMULAS`] is read by [`explain_undine_formula_race_trait`] below
// AND by `oracle_validation::race_trait_formula_bar_check` (a separate crate
// module, oracle-side since SD-35 `AT-35-E6-001`) — a transcription regression in this table therefore fails the SAME gate whether it
// corrupts the value a player sees or the value the bar check verifies.
//
// Re-verified against this repo's own independently re-ingested corpus records
// (`data/corpus/advanced_race_guide/race_trait/undine/undine_{acid_breath,nereid_fascination,
// ooze_breath}.json`'s declared bonus chains, read by
// `pcgen_import::ingest_record::bonus_chain_qualifiers`) at fold time — all nine formula strings match the
// transcription below byte-for-byte, including Ooze Breath's genuinely-as-written
// `min(floor((TL+1/2)),5)` (not `(TL+1)/2`), confirming the branch's transcription was faithful
// upstream arithmetic, not a typo.
pub(super) const UNDINE_RACE_ID: &str = "race:undine";

pub(super) const UNDINE_ACID_BREATH_TRAIT_KEY: &str = "Undine ~ Acid Breath";

pub(super) const UNDINE_NEREID_FASCINATION_TRAIT_KEY: &str = "Undine ~ Nereid Fascination";

pub(super) const UNDINE_OOZE_BREATH_TRAIT_KEY: &str = "Undine ~ Ooze Breath";

/// `(unit_id, formula_field_name, raw_formula)` — the exact `BONUS:VAR` formula text transcribed
/// verbatim from the pinned oracle, never hand-duplicated elsewhere in this file. See the section
/// doc above for why this table is the shared source of truth for both compute and gate.
pub const UNDINE_RACE_TRAIT_FORMULAS: &[(&str, &str, &str)] = &[
    ("advanced_race_guide:race_trait:undine_acid_breath", "Undine_AcidBreath_Times", "1"),
    (
        "advanced_race_guide:race_trait:undine_acid_breath",
        "Undine_AcidBreath_Dice",
        "min(floor((TL+1)/2),5)",
    ),
    ("advanced_race_guide:race_trait:undine_acid_breath", "Undine_AcidBreath_DC", "10+(TL/2)+CON"),
    (
        "advanced_race_guide:race_trait:undine_nereid_fascination",
        "Undine_NereidFascination_Times",
        "1",
    ),
    (
        "advanced_race_guide:race_trait:undine_nereid_fascination",
        "Undine_NereidFascination_Duration",
        "max((TL/2),1)",
    ),
    (
        "advanced_race_guide:race_trait:undine_nereid_fascination",
        "Undine_NereidFascination_DC",
        "10+(TL/2)+CHA",
    ),
    ("advanced_race_guide:race_trait:undine_ooze_breath", "Undine_OozeBreath_Times", "1"),
    (
        "advanced_race_guide:race_trait:undine_ooze_breath",
        "Undine_OozeBreath_Dice",
        "min(floor((TL+1/2)),5)",
    ),
    ("advanced_race_guide:race_trait:undine_ooze_breath", "Undine_OozeBreath_DC", "10+(TL/2)+CON"),
];

/// The ability-modifier slot [`CharacterFacts::ability_mods`] uses, so this file can seed the
/// two modifiers Undine's alternate racial traits actually reference without duplicating the
/// sheet evaluator's own ordering.
pub(super) fn ability_index_of(a: crate::rules_core::sheet_rule::Ability) -> usize {
    use crate::rules_core::sheet_rule::Ability;
    match a {
        Ability::Str => 0,
        Ability::Dex => 1,
        Ability::Con => 2,
        Ability::Int => 3,
        Ability::Wis => 4,
        Ability::Cha => 5,
    }
}

/// One Undine alternate-racial-trait field's arithmetic, as a CONVERTED
/// [`Expr`](crate::rules_core::sheet_rule::Expr) over total character level and one ability
/// modifier (SD-35 `AT-35-E6-001`, `decisions.md` §11 -- the live side holds converted
/// arithmetic, never an ingest-format formula string).
///
/// Each arm is the conversion of the corresponding row of
/// [`UNDINE_RACE_TRAIT_FORMULAS`], which the
/// converter/oracle side still carries verbatim and which
/// `derived_evaluator_fixture_check`'s race_trait_formula bar still checks both halves of.
/// Panics on a field name this file does not build -- every field this function is ever called
/// with is a literal named directly below, so a mismatch is a coding error here, never a
/// runtime condition.
pub(super) fn undine_expr(field: &str) -> crate::rules_core::sheet_rule::Expr {
    use crate::rules_core::sheet_rule::{Ability, Expr};
    // `10 + (TL/2) + <ability modifier>` -- the save DC shape all three alternates share.
    let dc = |a: Ability| {
        Expr::sum(vec![
            Expr::Const(10),
            Expr::div(Expr::Level, Expr::Const(2)),
            Expr::AbilityMod(a),
        ])
    };
    match field {
        "Undine_AcidBreath_Times"
        | "Undine_NereidFascination_Times"
        | "Undine_OozeBreath_Times" => Expr::Const(1),
        // `min(floor((TL+1)/2),5)`
        "Undine_AcidBreath_Dice" => Expr::min(
            Expr::Floor(Box::new(Expr::div(
                Expr::sum(vec![Expr::Level, Expr::Const(1)]),
                Expr::Const(2),
            ))),
            Expr::Const(5),
        ),
        "Undine_AcidBreath_DC" | "Undine_OozeBreath_DC" => dc(Ability::Con),
        // `max((TL/2),1)`
        "Undine_NereidFascination_Duration" => {
            Expr::max(Expr::div(Expr::Level, Expr::Const(2)), Expr::Const(1))
        }
        "Undine_NereidFascination_DC" => dc(Ability::Cha),
        // `min(floor((TL+1/2)),5)` -- genuinely `TL + one half`, not `(TL+1)/2`; see the
        // section doc above `UNDINE_RACE_TRAIT_FORMULAS` for why that asymmetry is faithful.
        "Undine_OozeBreath_Dice" => Expr::min(
            Expr::Floor(Box::new(Expr::sum(vec![
                Expr::Level,
                Expr::div(Expr::Const(1), Expr::Const(2)),
            ]))),
            Expr::Const(5),
        ),
        other => panic!("undine_expr carries no converted arithmetic for field {other:?}"),
    }
}

/// SD13-E6-F3a Human racial trait bundle explanation seam.
///
/// Surfaces each remaining PF1 Standard Human racial trait dimension (size,
/// speed, senses, extra skill ranks) as an explicit `ComputationExplanation`
/// record so the trait bundle is legible on the runtime path rather than left
/// as an incidental side-effect or a folklore claim. Three of the four
/// dimensions carry the grounded PF1 source value as a recognition record;
/// the senses dimension carries a bounded "no special senses" classification
/// because PF1 Standard Human grants no special sense bonus.
///
/// This function:
///   - runs only when `race_id == race:human`; non-Human races stay on the
///     bounded diagnostics the `explain_race_seam` dispatcher emits (the
///     Half-Elf bounded diagnostic or the `race.semantics.unverified`
///     catch-all),
///   - adds no new computed mechanical contribution; each record carries the
///     grounded source value as recognition and contributes nothing to the
///     chassis totals, selected-skill modifiers, combat baseline, or AC,
///   - replaces the previous "Human size, speed, senses, extra skill ranks
///     remain unverified" non-claim-blocking note from
///     `race.human.bounded_semantics` with explicit per-dimension records,
///   - is bounded to the deterministic Human Fighter level-1/2/3 pilot
///     posture implicitly via the caller; it deliberately grounds no other
///     Human racial variant (alternate Human racial traits, variant Humans,
///     half-Humans), no other race, and no PF1 alternate ruleset.
pub(super) fn explain_human_trait_bundle(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    _diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // ----- size -----
    // Recognition record only; carries the grounded Human size category name
    // as the recognition value so the explanation reads as the humanoid
    // identity rather than fabricating a numeric contribution.
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Human racial trait bundle — size: PF1 Standard Human is {HUMAN_SIZE_CATEGORY} size \
             (cr_races.lst race:human SIZE:MEDIUM). This is a bounded recognition record naming \
             the Human size category on the deterministic pilot seam; it contributes no numeric \
             effect to attack rolls, AC, skill checks, ability checks, or any other computed \
             value, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- speed -----
    // Recognition record for the 30 ft base land speed. The bounded
    // selected-skill and combat baselines never consult base speed, so this
    // record is identity-only — no computed speed-derived value is fabricated.
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.speed".to_owned(),
        value: HUMAN_BASE_SPEED_FEET,
        detail: format!(
            "Human racial trait bundle — speed: PF1 Standard Human has a base land speed of \
             {HUMAN_BASE_SPEED_FEET} ft (cr_races.lst race:human GAIT:WALK|{HUMAN_BASE_SPEED_FEET}). \
             This is a grounded recognition value carrying the human base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    // Bounded "no special senses" classification. PF1 Standard Human grants
    // no special senses (darkvision, low-light, scent, etc.), so this
    // dimension is classified explicitly as no-effect rather than a silent
    // omission or a fabricated sense bonus.
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Human racial trait bundle — senses: PF1 Standard Human grants no special senses \
             (cr_races.lst race:human carries no SENSE tag for Standard Human; darkvision, \
             low-light vision, scent, and other sense bonuses are absent). This is a bounded \
             no-effect classification record on the deterministic pilot seam; it carries no \
             fabricated sense bonus and contributes no computed value (+0)"
            .to_owned(),
    });

    // ----- extra skill ranks -----
    // Recognition record for the extra-skill-ranks Human trait. PF1 Standard
    // Human grants 4 extra skill points at 1st level and 1 extra skill rank
    // per additional level thereafter; this slice surfaces both numbers as a
    // recognition record and explicitly does not propagate them through the
    // bounded selected-skill modifier computation (which controls the
    // deterministic Climb / Intimidate / Swim rank-1 posture only).
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.extra_skill_ranks".to_owned(),
        value: i16::from(HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL),
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:SKILL|...
            "Human racial trait bundle — extra skill ranks: PF1 Standard Human gains \
             {HUMAN_EXTRA_SKILL_POINTS_AT_LEVEL_1} extra skill points at 1st level and \
             {HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL} extra skill rank per additional level thereafter \
             (cr_races.lst race:human). The recognition value \
             ({HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL:+}) carries the per-additional-level extra-rank \
             identity on the deterministic pilot seam; this slice does not propagate these extra \
             skill points/rank through the bounded Climb/Intimidate/Swim rank-1 selected \
             skill-modifier computation, so the bounded fighter-posture skill totals remain grounded \
             by the canonical rank-1 posture rather than by the unbounded Human extra skill-rank \
             rule"
        ),
    });
}

/// The choice-set id under which a chosen ARG alternate racial trait is
/// persisted on `ChosenCharacterState::selected_choices`.
///
/// Reuses the engine's existing general choice channel rather than adding a
/// field to `ChosenCharacterState`: a `SelectedChoice` already round-trips
/// through `SavedCharacterStore`'s serializer, through `clone_character`, and
/// through `apply_level_up`, so a persisted alternate racial trait survives
/// save/load/clone/level-up with no schema change. The selection id is the
/// corpus record key verbatim (`"Dwarf ~ Saltbeard"`) — the same string
/// `RaceCorpus::resolve` and `race_trait_picker`'s command surface take, so
/// the picker round-trips it unchanged.
///
/// The set is repeatable: a character may hold several alternates at once, so
/// readers must scan every matching entry rather than use `choice_selection`,
/// which returns only the first.
pub const RACE_ALTERNATE_TRAIT_CHOICE_ID: &str = "choice:race_alternate_trait";

/// The choice-set id under which a real character-creation-time companion/
/// mount species pick is persisted on `ChosenCharacterState::selected_
/// choices` — SD-32 T12 `epic-10-reference-library-residual-reach` row 20
/// cycle 7's own new dispatch point, closing the wiring gap cycle 6 named
/// (`companion_base_stat_table.rs`'s own module doc: "`ground_companion_
/// stat_block` has zero live callers anywhere in the crate"). The
/// selection id is a `companion_base_stat_table::companion_base_stat_
/// table` slug verbatim (`"gulper_plant"`, `"allosaurus"`, ...); see
/// `ground_selected_companion_or_default` below for the read side and
/// `apps/desktop/src-tauri/src/pf1_adapter.rs`'s `compose_character_input`
/// for where a real `CreateCharacterRequest.companion_species` field
/// writes it. Uses the same general choice channel `RACE_ALTERNATE_TRAIT_
/// CHOICE_ID`'s own doc comment names, for the same reason: zero schema
/// change needed for this to survive save/load/clone/level-up.
pub const COMPANION_SPECIES_CHOICE_ID: &str = "choice:companion_species";

/// Grounds the companion/mount stat block for whichever species a
/// character-creation request selected via `COMPANION_SPECIES_CHOICE_ID`,
/// when that species has a verified row in `companion_base_stat_table::
/// companion_base_stat_table` — the real character-creation-time dispatch
/// point `companion_base_stat_table.rs`'s own cycle-6 module doc named as
/// missing. When no choice was made (an omitted `companion_species`
/// field), OR the requested slug has no verified row yet (a typo, or a
/// species this engine has not hand-authored — 201 of 213 `RACETYPE:
/// Companion` corpus records as of cycle 6), falls back to `default_
/// ground`, the class's own existing single-species hardcoded function
/// (`ground_wolf_companion_stat_block`/`ground_horse_companion_stat_
/// block`) — so an omitted or unrecognized choice never regresses any of
/// the 61 classes below its already-`Computed` behavior, and an
/// unverified species is never fabricated, only silently defaulted to the
/// species this engine already has real, sourced data for.
pub(super) fn ground_selected_companion_or_default(
    input: &CharacterInput,
    id_prefix: &str,
    owner_class_label: &str,
    companion_level: u8,
    default_ground: fn(&str, &str, u8, &mut Vec<ComputationExplanation>),
    explanations: &mut Vec<ComputationExplanation>,
) {
    // No real player selection present (the overwhelming majority of
    // characters, including every existing test that predates this
    // dispatch point): use `default_ground`, the class's own prior
    // hand-authored function, UNCHANGED -- not merely equivalent output,
    // the literal same code path, so this is byte-for-byte non-regressive
    // even where the generic table (`companion_base_stat_table.rs`)
    // happens to already carry `default_species_slug`'s own row (it does,
    // for both "wolf" and "horse" -- cycle 5's own reproduction proof) but
    // grounds a narrower record set than the hand-authored function does
    // (no per-species natural-attack record, e.g. Wolf's own `bite_
    // attack`/Horse's own `hoof_attack` -- see companion_base_stat_table.
    // rs's own module doc, "grounds only the fields with a live downstream
    // reader"). Only an ACTUAL selection (including one that happens to
    // name the same default species) takes the generic, narrower path.
    let Some(species_slug) = choice_selection(input, COMPANION_SPECIES_CHOICE_ID) else {
        default_ground(id_prefix, owner_class_label, companion_level, explanations);
        return;
    };
    let display_name = companion_base_stat_table::companion_display_name(species_slug);
    let grounded = companion_base_stat_table::ground_companion_stat_block(
        species_slug,
        id_prefix,
        owner_class_label,
        &display_name,
        companion_level,
        explanations,
    );
    if !grounded {
        default_ground(id_prefix, owner_class_label, companion_level, explanations);
    }
}

/// The namespace every alternate-racial-trait selection id carries.
///
/// **Not decoration.** `SavedCharacterStore`'s serializer rejects any
/// `SelectedChoice::selection_id` with fewer than two colon-segments, because
/// its line grammar splits `choice=<set>:<selection>` on colons — so a bare
/// corpus key (`"Dwarf ~ Minesight"`) cannot be persisted at all. Prefixing it
/// gives the same `feat:` / `school:` / `bond:` shape every other selection id
/// in this engine already has, and the key survives verbatim after the first
/// colon (the corpus keys contain no colon of their own, verified across all
/// 156 ingested records by
/// `tests/sd27_alternate_racial_trait_reachability.rs`).
pub const RACE_ALTERNATE_TRAIT_SELECTION_PREFIX: &str = "race_trait:";

/// Wraps a corpus alternate-racial-trait key into the persisted selection id.
pub fn race_alternate_trait_selection_id(trait_key: &str) -> String {
    format!("{RACE_ALTERNATE_TRAIT_SELECTION_PREFIX}{trait_key}")
}

/// Every alternate racial trait key this character has taken, in selection
/// order, deduplicated, with the storage namespace stripped back off.
///
/// A selection recorded without the prefix is read as-is rather than dropped:
/// the engine's job here is to see every choice that was made, and an
/// unrecognized key is reported by `explain_selected_alternate_racial_traits`
/// rather than silently skipped.
pub(crate) fn selected_alternate_trait_keys(input: &CharacterInput) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for choice in &input.chosen.selected_choices {
        if choice.choice_set_id != RACE_ALTERNATE_TRAIT_CHOICE_ID {
            continue;
        }
        let key = choice
            .selection_id
            .strip_prefix(RACE_ALTERNATE_TRAIT_SELECTION_PREFIX)
            .unwrap_or(choice.selection_id.as_str())
            .to_owned();
        if !out.contains(&key) {
            out.push(key);
        }
    }
    out
}

/// The alternate racial trait key ARG's Armor of the Pit feat tests for, as
/// the shipped 153-record table spells it
/// (`race_resolver.rs`: `("Tiefling ~ Scaled Skin", &["Tiefling_ReplaceFiendishResistance"])`,
/// matching `data/corpus/advanced_race_guide/race_trait/tiefling/tiefling_scaled_skin.json`'s
/// own `key`).
///
/// The feat's own corpus token names PCGen's three per-energy sub-abilities
/// (`Scaled Skin C ~ Tiefling`, `... E ...`, `... F ...`) rather than the
/// parent trait. Those three are the cold/electricity/fire variants a Scaled
/// Skin holder ends up with, and this engine's alternate-trait picker offers
/// only the parent — so the parent is the exact, and only, decidable form of
/// the same question here. Naming the three sub-keys instead would test for
/// strings no character in this codebase can ever carry, which is how a gate
/// silently never fires.
pub(super) const TIEFLING_SCALED_SKIN_TRAIT_KEY: &str = "Tiefling ~ Scaled Skin";

/// Whether this character took the Scaled Skin alternate racial trait — the
/// single fact ARG's Armor of the Pit needs in order to decide which of its two
/// mutually exclusive halves applies.
pub(super) fn character_has_tiefling_scaled_skin(input: &CharacterInput) -> bool {
    selected_alternate_trait_keys(input)
        .iter()
        .any(|key| key == TIEFLING_SCALED_SKIN_TRAIT_KEY)
}

/// Whether one of this character's chosen alternate racial traits fires the
/// named `<Race>_Replace<Trait>` flag — i.e. whether the standard trait whose
/// `!PREFACT:1,ABILITIES,<flag>=True` gate names it has been replaced.
///
/// This is `decisions.md §26`'s protocol applied to the engine's own
/// hand-modelled records. Every standard racial trait this file grounds is now
/// gated on its own corpus-declared flag, so a swap the player made in the
/// picker really does stop the standard trait's effect from applying.
///
/// The flag lookup is `race_resolver::alternate_traits_fire_flag`, a pure
/// function over a table pinned against the on-disk corpus — this file may not
/// read the filesystem (see `RACE_SIZES`' own doc comment for the same
/// constraint and the same resolution).
pub(super) fn replaced_by_alternate_trait(input: &CharacterInput, flag: &str) -> bool {
    crate::rules_core::race_resolver::alternate_traits_fire_flag(
        &selected_alternate_trait_keys(input),
        flag,
    )
}

/// Every ARG alternate racial trait that declares a plain-integer
/// `BONUS:SKILL` on one of the three skills `compute_selected_skill_modifiers`
/// actually totals, as `(trait key, owning race id, Climb, Intimidate, Swim)`.
///
/// # This list is a measurement, not a selection
///
/// `tests/sd27_alternate_racial_trait_reachability.rs` re-derives it by
/// scanning all 153 alternates' declared bonus chains
/// (`pcgen_import::ingest_record::bonus_chain_qualifiers`) against the engine's own
/// computed-total surface, and fails if the two disagree in either direction.
/// It is short because the *engine* is narrow, not because the content is:
/// every other alternate's declared number is situational
/// (`BONUS:SITUATION|Perception=to notice flying creatures|2`), aimed at a
/// skill this codebase computes no total for (Perception, Fly, Linguistics,
/// Profession, Handle Animal), or formula-valued (`TL/2`,
/// `1+Global_LuckBonus`) and so out of reach under `decisions.md §24`'s
/// no-interpreter ruling. Widen the engine's totals and this table grows —
/// the test will say by how much and name the traits.
///
/// # Why the values are read out of multi-skill chains
///
/// A chain names every skill it covers: `Goblin ~ Tree Runner` is
/// `BONUS:SKILL|Acrobatics,Climb|4|TYPE=Racial`, and Acrobatics has no total
/// here, so only the Climb half lands. `Gnome ~ Explorer`'s
/// `Climb,%LIST` likewise contributes its Climb half; the `%LIST` half is a
/// player-chosen skill this engine models no chooser for, and is deliberately
/// not guessed at.
pub(super) const ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES: &[(&str, &str, i16, i16, i16)] = &[
    // (trait key, race id, Climb, Intimidate, Swim)
    ("Elf ~ Spirit of the Waters", "race:elf", 0, 0, 4),
    ("Gnome ~ Explorer", "race:gnome", 2, 0, 0),
    ("Goblin ~ Tree Runner", "race:goblin", 4, 0, 0),
    // Inner Sea Races (SD-29 race-trait lane round 2). Round 2 landed the
    // records without these rows, so all three were offered and moved nothing
    // until round 3 (`decisions.md §47`).
    ("Gnome ~ Intrepid Settler", "race:gnome", 2, 0, 2),
    ("Half-Elf ~ Sea Legs", "race:half-elf", 0, 0, 2),
    ("Hobgoblin ~ Authoritative", "race:hobgoblin", 0, 2, 0),
    ("Half-Elf ~ Water Child", "race:half-elf", 0, 0, 4),
    ("Half-Orc ~ Forest Walker", "race:half-orc", 2, 0, 0),
    ("Half-Orc ~ Rock Climber", "race:half-orc", 1, 0, 0),
    ("Hobgoblin ~ Bandy-Legged", "race:hobgoblin", 2, 0, 0),
    ("Hobgoblin ~ Fearsome", "race:hobgoblin", 0, 4, 0),
    ("Human ~ Heart of the Mountain", "race:human", 2, 0, 0),
    ("Human ~ Heart of the Sea", "race:human", 0, 0, 2),
    // SD-31 Epic 1-F2 (2026-08-15). `BONUS:SKILL|Diplomacy,Intimidate|2|
    // TYPE=Racial` -- Diplomacy has no total here, so only Intimidate lands.
    ("Grippli ~ Princely", "race:grippli", 0, 2, 0),
    // SD-31-E6-F4-003 (ARG's own Strix chassis batch, 2026-08-16), wired
    // SD31-W9-INTEGRATE-001 (`arg_abilities_race.lst:1149-1153`):
    // `BONUS:SKILL|Intimidate|2` and `BONUS:SKILL|Bluff,Diplomacy,Climb|2` --
    // only Intimidate and Climb are tracked totals here, Bluff/Diplomacy
    // are not.
    ("Strix ~ Frightening", "race:strix", 0, 2, 0),
    ("Strix ~ Wing-Clipped", "race:strix", 2, 0, 0),
    // SD-33 Epic 6's fold of SD31-E6-F4-005's lost wave-11 Skinwalker
    // heritage lane (2026-08-26). Of the 9 kins' `~ Animal-Minded`
    // replacement rows (all `TraitRole::Alternate`, independently
    // selectable), exactly these 2 land a `BONUS:SKILL` on one of this
    // table's three tracked skills:
    // `skinwalker_abilities_race_subrace.lst`'s Werebear-Kin row is
    // `BONUS:SKILL|Climb|2|TYPE=Racial` (plus a `BONUS:VAR|WildEmpathy|2`
    // this table does not track) and Wereshark-Kin's is
    // `BONUS:SKILL|Swim|2|TYPE=Racial`. The other 7 kins land on
    // Fly/Perception/Stealth/Survival/Perception-at-night, none of which
    // this table computes a total for, so they contribute nothing here --
    // same "engine is narrow, not the content" shape this const's own doc
    // comment already states for ARG.
    ("Werebear-Kin ~ Animal-Minded", "race:skinwalker", 2, 0, 0),
    ("Wereshark-Kin ~ Animal-Minded", "race:skinwalker", 0, 0, 2),
];

/// The Climb / Intimidate / Swim racial bonus this character's chosen
/// alternate racial traits contribute.
///
/// **The highest applies, not the sum.** All ten corpus chains carry
/// `TYPE=Racial`, and PF1's stacking rule for two same-typed named bonuses is
/// that only the largest counts. This matters for a real pair a player can
/// legally hold: `Half-Orc ~ Forest Walker` (+2 Climb, replaces vision) and
/// `Half-Orc ~ Rock Climber` (+1 Climb, replaces Intimidating) fire different
/// flags, so ARG's own `PREMULT` guard does not exclude them from each other —
/// a Half-Orc may take both, and gets +2, not +3.
///
/// Race-gated by construction: a trait key is matched only against its owning
/// race, so a selection copied onto another race contributes nothing.
pub(super) fn alternate_trait_selected_skill_bonuses(input: &CharacterInput) -> SelectedSkillModifiers {
    let selected = selected_alternate_trait_keys(input);
    let mut best = SelectedSkillModifiers::default();
    for (key, race_id, climb, intimidate, swim) in ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES {
        if input.chosen.race_id != *race_id || !selected.iter().any(|chosen| chosen == key) {
            continue;
        }
        best.climb = best.climb.max(*climb);
        best.intimidate = best.intimidate.max(*intimidate);
        best.swim = best.swim.max(*swim);
    }
    best
}

/// Return the selection id chosen for the named choice set, if present.
pub(crate) fn choice_selection<'a>(input: &'a CharacterInput, choice_set_id: &str) -> Option<&'a str> {
    input
        .chosen
        .selected_choices
        .iter()
        .find(|c| c.choice_set_id == choice_set_id)
        .map(|c| c.selection_id.as_str())
}

/// Return a human-readable display name (e.g. "Knowledge (arcana)") when the given
/// selection names a specific Knowledge skill (a "knowledge:<skill>"-shaped token).
/// Returns `None` for any selection that is not itself shaped that way. This recognizes
/// the whole Knowledge skill family rather than a restricted enum list, because the PF1
/// Core Rulebook Arcane bloodline's own class-skill grant text reads "Knowledge (any
/// one)" — any Knowledge skill is legal, not just Knowledge (arcana).
pub(super) fn knowledge_skill_display_name(selection: &str) -> Option<String> {
    selection
        .strip_prefix("knowledge:")
        .filter(|skill| !skill.is_empty())
        .map(|skill| format!("Knowledge ({skill})"))
}

/// Look up the already-computed modifier for a named ability. Unknown ability names
/// contribute nothing rather than fabricating a value.
pub(super) fn ability_modifier_for(modifiers: &AbilityModifiers, ability: &str) -> i16 {
    match ability {
        "strength" => modifiers.strength,
        "dexterity" => modifiers.dexterity,
        "constitution" => modifiers.constitution,
        "intelligence" => modifiers.intelligence,
        "wisdom" => modifiers.wisdom,
        "charisma" => modifiers.charisma,
        _ => 0,
    }
}

/// The effective Chain Shirt armor-check penalty at a Fighter level, after any
/// armor-training reduction. Capped at 0 so the reduction never turns the penalty
/// into a bonus.
pub(super) fn effective_chain_shirt_armor_check_penalty(level: u8) -> i16 {
    (CHAIN_SHIRT_ARMOR_CHECK_PENALTY + fighter_armor_training(level).armor_check_reduction).min(0)
}

/// SD-21 Epic 6 (E6.25) / Epic 7 (E7.28); widened v0.6 alpha swarm task 4:
/// dispatch the base-attack-bonus / base-save chassis pillar to the supported
/// single class, to `compute_multiclass_base_chassis` for a supported
/// length-2+ multiclass mix, or return `None` when the input is a single
/// class this dispatch does not yet recognize, or a multiclass mix
/// containing an unrecognized class. Each recognized class's own
/// `compute_<class>_chassis` function (or the shared table-driven
/// `compute_generic_table_chassis` path) still independently checks its own
/// level range and pushes `class_chassis.unsupported` itself when out of
/// range, so this dispatch only needs to route by `class_id` / mix shape.
pub(crate) fn has_supported_class_chassis(input: &CharacterInput) -> bool {
    supported_fighter_level(input).is_some()
        || supported_wizard_level(input).is_some()
        || is_supported_multiclass_mix(input)
        || is_supported_generic_single_class(input)
        || is_supported_skald_single_class(input)
        || is_supported_bloodrager_single_class(input)
        || is_supported_brawler_single_class(input)
        || is_supported_hunter_single_class(input)
        || is_supported_cavalier_single_class(input)
        || is_supported_alchemist_single_class(input)
        || is_supported_inquisitor_single_class(input)
        || is_supported_oracle_single_class(input)
        || is_supported_arcanist_single_class(input)
        || is_supported_warpriest_single_class(input)
        || is_supported_slayer_single_class(input)
        || is_supported_swashbuckler_single_class(input)
        || is_supported_investigator_single_class(input)
        || is_supported_witch_single_class(input)
        || is_supported_shaman_single_class(input)
        || is_supported_summoner_single_class(input)
        // SD-27 (2026-07-31): all four Pathfinder Unchained classes at once
        // -- see `is_supported_pu_single_class` for why this one gate is
        // broad where the sixteen APG/ACG gates above are per-class.
        || is_supported_pu_single_class(input)
        // SD-31 wave 20 (chassis-coverage lane): Ultimate Combat's three
        // classes (Gunslinger, Ninja, Samurai) have dispatched through
        // `compute_class_chassis` to `compute_uc_class_chassis` since
        // `SD31-E4-F1-002`/`-005`, but this gate -- which several OTHER
        // downstream pillars (`compute_total_saves`, `compute_combat_baseline`,
        // `compute_selected_skill_modifiers`) check independently of
        // `compute_class_chassis` itself -- never grew a matching arm. The
        // chassis numbers were real and correct; the receipt still never
        // reached `Computed` because those siblings' own
        // `claim_blocking: true` "unsupported" diagnostics fired anyway. See
        // `ultimate_combat_chassis_gate_tests` below `supported_class_chassis_description`.
        || is_supported_uc_single_class(input)
        // SD-34 wave 33 lane C (`class_modelled_but_no_observed_delta_on_
        // the_rendered_snapshot`): the SAME recurring gap the UC comment
        // above already names -- `untabled_base_class_chassis::resolve`
        // (SD-32 card 11, the 20-class real-base-class registry: Aegis,
        // Antipaladin, Cryptic, Dread, Kineticist, Magus, Marksman, Medium,
        // Mesmerist, Occultist, Psion, Psychic, Psychic Warrior, Shifter,
        // Soulknife, Spiritualist, Tactician, Vigilante, Vitalist, Wilder)
        // and `crb_untabled_class_chassis::resolve` (SD-34's own seven CRB
        // NPC/`Ex-*` classes: Adept, Aristocrat, Commoner, Expert, Warrior,
        // Ex-Barbarian, Ex-Paladin) have dispatched real BAB/base-save
        // chassis rows through `compute_class_chassis` since their own
        // cycles, but this shared gate -- which `compute_total_saves`,
        // `compute_combat_baseline`, and `compute_selected_skill_modifiers`
        // each check independently of `compute_class_chassis` itself --
        // never grew a matching arm, so all 27 of those classes' receipts
        // still never reached `Computed` despite a real, correct chassis.
        // Deliberately NOT extended to `prestige_class_entry_gate`: that
        // registry's own module doc comment states it "still returns no
        // chassis magnitude" by design (`class_chassis.unsupported` stays
        // claim-blocking for prestige classes on purpose, since no BAB/save
        // row exists to fold into a total save or combat baseline -- adding
        // it here would fabricate zeroes as if they were real numbers).
        || is_supported_untabled_base_class_single_class(input)
        || is_supported_crb_untabled_class_single_class(input)
}

/// A single-class character at a level `untabled_base_class_chassis::
/// resolve` carries a real corpus-derived BAB/base-save row for -- one of
/// the 20 real base classes (Aegis, Antipaladin, Cryptic, Dread,
/// Kineticist, Magus, Marksman, Medium, Mesmerist, Occultist, Psion,
/// Psychic, Psychic Warrior, Shifter, Soulknife, Spiritualist, Tactician,
/// Vigilante, Vitalist, Wilder) that had no `compute_class_chassis`
/// dispatch arm until SD-32 card 11. Named and shaped like
/// `is_supported_uc_single_class` rather than folded inline, for the same
/// reason: a future widening has one obvious place to grow.
pub(super) fn is_supported_untabled_base_class_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    untabled_base_class_chassis::resolve(&class_level.class_id, class_level.level).is_some()
}

/// A single-class character at a level `crb_untabled_class_chassis::
/// resolve` carries a real corpus-derived BAB/base-save row for -- one of
/// CRB's five NPC classes or two `Ex-*` variant states (Adept, Aristocrat,
/// Commoner, Expert, Warrior, Ex-Barbarian, Ex-Paladin), registered by
/// SD-34 `AT-34-E3-001`. Named and shaped like `is_supported_uc_single_class`
/// for the same reason as its sibling immediately above.
pub(super) fn is_supported_crb_untabled_class_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    crb_untabled_class_chassis::resolve(&class_level.class_id, class_level.level).is_some()
}

/// A single-class Ultimate Combat character (Gunslinger, Ninja, or
/// Samurai) at a level `rules_tables::ultimate_combat::class_chassis_resolve`
/// carries a row for. Named and shaped like `is_supported_pu_single_class`
/// rather than folded inline, so a future widening (a fourth UC class) has
/// one obvious place to grow.
pub(super) fn is_supported_uc_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    let Some(class_id) = UcClassId::from_class_id_str(&class_level.class_id) else {
        return false;
    };
    uc::class_chassis_resolve(class_id, class_level.level, RuleSetId::Uc).is_some()
}

/// Prose listing of every chassis `has_supported_class_chassis` accepts,
/// kept directly beneath it for exactly one reason (task #82 fix,
/// 2026-07-28): every widening of the OR-chain above has reliably updated
/// this function's own doc comment (see the "SD-21 E6.26 widened...",
/// "SD-21 E6b.1 widened..." comments this file carries at each downstream
/// call site) while the user-facing diagnostic prose in
/// `defense.total_save.unsupported` and `skill.selected_modifier.unsupported`
/// (both places) kept hardcoding a stale "Fighter levels 1-N or Wizard
/// levels 1-N" framing three widenings after this gate grew past those two
/// classes -- misleading for any of the other 17 classes this gate now
/// recognizes. Both diagnostics interpolate this function instead of
/// hardcoding their own copy, so there is exactly one place left to update
/// the NEXT time `has_supported_class_chassis` widens; keep this list in
/// the same order and wording family as the OR-chain above so a reviewer
/// can diff them side by side.
pub(crate) fn supported_class_chassis_description() -> String {
    format!(
        "{FIGHTER_CLASS_ID} levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL}, {WIZARD_CLASS_ID} levels \
         1-{MAX_SUPPORTED_WIZARD_LEVEL}, a supported multiclass mix, a supported generic \
         single class, or a supported single-class Skald, Bloodrager, Brawler, Hunter, \
         Cavalier, Alchemist, Inquisitor, Oracle, Arcanist, Warpriest, Slayer, Swashbuckler, \
         Investigator, Witch, Shaman, or Summoner chassis, or a supported \
         single-class Unchained Barbarian, Unchained Monk, Unchained Rogue, or \
         Unchained Summoner chassis (Pathfinder Unchained), or a supported \
         single-class Gunslinger, Ninja, or Samurai chassis (Ultimate Combat), or a \
         supported single-class untabled real base class (Aegis, Antipaladin, \
         Cryptic, Dread, Kineticist, Magus, Marksman, Medium, Mesmerist, Occultist, \
         Psion, Psychic, Psychic Warrior, Shifter, Soulknife, Spiritualist, \
         Tactician, Vigilante, Vitalist, or Wilder) or CRB NPC/Ex-* class (Adept, \
         Aristocrat, Commoner, Expert, Warrior, Ex-Barbarian, or Ex-Paladin) chassis"
    )
}

/// v0.6 alpha swarm, risks item 8 (Slayer full-build closure): whether
/// `input` is a single-class Slayer at a level within
/// `acg::class_chassis_resolve`'s declared ceiling for Slayer -- mirrors
/// the other six ACG/APG exact-match gates exactly.
pub(super) fn is_supported_slayer_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if AcgClassId::from_class_id_str(&class_level.class_id) != Some(AcgClassId::Slayer) {
        return false;
    }
    acg::class_chassis_resolve(AcgClassId::Slayer, class_level.level, RuleSetId::Acg).is_some()
}

/// SD-27 (Pathfinder Unchained class wiring, 2026-07-31): whether `input`
/// is a single-class Unchained Barbarian / Monk / Rogue / Summoner at a
/// level within `pu::class_chassis::class_chassis_resolve`'s declared
/// ceiling.
///
/// **Deliberately a broad `from_class_id_str(...).is_some()` where the ten
/// ACG and six APG gates above are each an exact single-variant match.**
/// That difference is intentional and is the opposite of a widening
/// mistake, so it is stated rather than left to be noticed: those books
/// were wired one class at a time, so a broad check would have silently
/// admitted classes whose features nothing grounded. Pathfinder Unchained
/// declares exactly four classes and this change wires **all four** at
/// once -- each one gets its own `ground_unchained_*_class_features` call
/// in `compute_pu_class_chassis`, so there is no fifth PU class for a
/// broad check to let through. `PuClassId::ALL.len() == 4` is asserted in
/// `rules_tables::pathfinder_unchained::class_chassis`'s own tests, and
/// `pu_gate_admits_exactly_the_four_unchained_classes` below re-checks the
/// admitted set from this side of the seam, so adding a fifth variant
/// without wiring it fails loudly instead of leaking through here.
pub(super) fn is_supported_pu_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    let Some(pu_class_id) = PuClassId::from_class_id_str(&class_level.class_id) else {
        return false;
    };
    pu_class_chassis::class_chassis_resolve(pu_class_id, class_level.level, RuleSetId::Pu).is_some()
}

/// Whether `input` is a single class, other than Fighter/Wizard (which have
/// their own bespoke `supported_<class>_level` gates above), at a level
/// within that class's own `class_tables()`-declared ceiling -- the
/// table-driven single-class chassis path `compute_class_chassis` also
/// recognizes via `compute_generic_table_chassis` (v0.6 alpha swarm, task 4).
pub(super) fn is_supported_generic_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if class_level.class_id == FIGHTER_CLASS_ID || class_level.class_id == WIZARD_CLASS_ID {
        return false;
    }
    multiclass_class_level_supported(class_level)
}

/// A human-readable class label for explanation text (e.g. "Fighter",
/// "Fighter+Rogue" for a multiclass mix), derived from
/// `input.chosen.class_levels` rather than a hardcoded class name. Before
/// this fix, `compute_combat_baseline`/`compute_total_saves`'s explanation
/// text unconditionally said "Fighter", which was accurate while those
/// functions only ever ran for a supported Fighter chassis -- now that the
/// chassis dispatch also serves Wizard, Rogue, and multiclass mixes (task
/// 4), a hardcoded "Fighter" became misleading for any other class even
/// though the underlying numeric values were always correct (v0.6 alpha
/// swarm fix, flagged by QA).
pub(crate) fn class_summary_label(input: &CharacterInput) -> String {
    input
        .chosen
        .class_levels
        .iter()
        .map(|class_level| {
            let name = class_level
                .class_id
                .strip_prefix("class:")
                .unwrap_or(&class_level.class_id);
            let mut chars = name.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join("+")
}

/// A human-readable race label for explanation text (e.g. "Elf", "Half-Elf"),
/// derived from `race_id` rather than a hardcoded race name -- the same
/// "derive from the real input, don't hardcode" fix `class_summary_label`
/// already established for class labels (v0.6 alpha swarm item 18 widening,
/// 2026-07-24: `explain_wizard_level1_prepared_spell_baseline`'s explanation
/// text previously said "Human Wizard" unconditionally, accurate while that
/// function only ran for Human, now misleading once widened to any race).
/// Capitalizes each hyphen-separated segment independently ("half-elf" ->
/// "Half-Elf"), matching every curated `race:<name>` id this crate
/// recognizes.
pub(super) fn race_display_label(race_id: &str) -> String {
    let name = race_id.strip_prefix("race:").unwrap_or(race_id);
    name.split('-')
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join("-")
}

/// Maps a wire-level `class_id` string to `rules_tables::crb::class_tables`'s
/// `ClassId`, for Fighter, Wizard, and Rogue (v0.6 alpha swarm, task 4 --
/// widened from a Fighter/Wizard-only pair to add Rogue, the task's own
/// literal "Fighter/X" reproducer). Used to route both single-class and
/// multiclass base-chassis computation to the shared
/// `compute_generic_table_chassis` path for Rogue; Fighter keeps its own
/// bespoke `compute_fighter_chassis` (it additionally grounds Bravery/
/// bonus-feat named-feature explanations beyond the raw BAB/save numbers
/// this table alone carries) and Wizard keeps its own existing
/// `compute_wizard_chassis` (left untouched to avoid any behavior change to
/// already-tested code) -- `table_class_id` recognizing both lets the
/// multiclass-support range check below share one implementation instead of
/// several.
///
/// Deliberately NOT widened to all 11 core classes in one pass:
/// `class_tables()` (`rules_tables/crb/class_tables.rs`) carries real data
/// for all 11, but one of those classes (Monk) already has its
/// OWN deliberate standalone-only `class_chassis.<class>.*` chassis explanations elsewhere
/// in this file, each with an existing `tests/*.rs` file asserting that
/// class's own base-attack/base-save stay claim-blocked / not integrated
/// into the generic `class_chassis.base_attack_bonus` pillar (e.g.
/// `tests/sd13_rogue_level1_chassis_baseline.rs`'s own doc comment: "not
/// wired into compute_fighter_chassis, compute_total_saves, or
/// compute_combat_baseline"). Widening `table_class_id` to all 11 in one
/// step broke ~60 of those pre-existing negative-control assertions across
/// ~15 QA-owned test files in this task's own RED/GREEN loop -- confirmed
/// by running `cargo test --test '*' --no-fail-fast` before scoping back
/// down to a narrower allowlist. Rogue's own equivalent standalone tests
/// (`sd13_rogue_level1_chassis_baseline.rs`) still broke by that narrower
/// change and were flagged to `qa` directly rather than silently pushed;
/// widening the other classes is one class (and one coordinated test
/// update) at a time (`class-multiclass-breadth-scoping.md`, risks item 8).
///
/// **Ranger added (v0.6 alpha swarm, risks item 8, first slice, 2026-07-24)**
/// -- the one class of the remaining 8 with no self-imposed claim-blocking
/// diagnostic of its own (`explain_ranger_level1_chassis_and_class_feature_separation`'s
/// signature never took a `diagnostics` parameter, so it structurally
/// could not self-block), and the largest existing investment (full
/// level-20 chassis, Track, combat style, Favored Enemy/Terrain, Hunter's
/// Bond, and a complete partial-caster spell ladder already grounded as
/// standalone explanations). Widening only this one entry lets Ranger
/// reach a real `Computed` status via `compute_generic_table_chassis`
/// (already built, already used for Rogue) without touching any other
/// class's dispatch.
///
/// **Paladin added (v0.6 alpha swarm, risks item 8, second slice, 2026-07-25)**
/// -- same shape as Ranger: `explain_paladin_level1_chassis_and_spell_burden_separation`
/// had its own self-imposed diagnostic (`class_spell.paladin.partial_caster.unsupported`)
/// but pushed it AFTER the single-class-only Human-gate check, the exact
/// structural flaw the Ranger adversarial review found -- fixed here the
/// same way, moving the check (and now the real validation) to the top of
/// that function, unconditional on single-class/race, before this widening
/// landed, so a Paladin+X multiclass or non-Human Paladin cannot silently
/// bypass it via this widening the way an unfixed copy of the flaw would
/// have allowed.
/// `pub(crate)` (rather than private) so `durability.rs`'s max-HP
/// computation (v0.6 alpha swarm, item 2) can reuse the same class-id
/// recognition this module's chassis dispatch already established, rather
/// than re-declaring a second, independently-maintained copy.
pub(crate) fn table_class_id(class_id_str: &str) -> Option<ClassId> {
    if class_id_str == FIGHTER_CLASS_ID {
        Some(ClassId::Fighter)
    } else if class_id_str == WIZARD_CLASS_ID {
        Some(ClassId::Wizard)
    } else if class_id_str == ROGUE_CLASS_ID {
        Some(ClassId::Rogue)
    } else if class_id_str == RANGER_CLASS_ID {
        Some(ClassId::Ranger)
    } else if class_id_str == PALADIN_CLASS_ID {
        Some(ClassId::Paladin)
    } else if class_id_str == SORCERER_CLASS_ID {
        Some(ClassId::Sorcerer)
    } else if class_id_str == CLERIC_CLASS_ID {
        Some(ClassId::Cleric)
    } else if class_id_str == DRUID_CLASS_ID {
        Some(ClassId::Druid)
    } else if class_id_str == BARBARIAN_CLASS_ID {
        Some(ClassId::Barbarian)
    } else if class_id_str == BARD_CLASS_ID {
        Some(ClassId::Bard)
    } else if class_id_str == MONK_CLASS_ID {
        // v0.6 alpha swarm (Monk chassis-recognition closure, 2026-07-29).
        // This arm's absence was the single root cause of FOUR of Monk's
        // five claim-blocking diagnostics (`class_chassis.unsupported`,
        // `combat.baseline_unsupported`, `defense.total_save.unsupported`,
        // `skill.selected_modifier.unsupported`) at all 20 levels:
        // `class_tables()` has carried a complete corpus-backed Monk row
        // since the CRB table was built (`cr_classes.lst:147`, 3/4 BAB,
        // all three saves good, `MAXLEVEL:20`), and
        // `compute_generic_table_chassis` could always have read it -- but
        // with no string mapping, `is_supported_generic_single_class`
        // never reached the table at all, so the chassis pillar was
        // computable and simply unreachable. Monk was the ONLY one of the
        // eleven CRB classes missing from this mapping.
        Some(ClassId::Monk)
    } else {
        None
    }
}

