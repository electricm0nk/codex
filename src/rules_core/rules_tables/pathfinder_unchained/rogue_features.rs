//! Pathfinder Unchained — **Unchained Rogue** class features, hand-modelled
//! per `decisions.md §24.1`.
//!
//! **No formula interpreter.** Every function below is a hand-written pure
//! function whose arithmetic was read byte-for-byte off one named corpus row
//! and pinned by a test, exactly as the 27 already-shipped classes were built
//! (`warpriest_fervor_uses_per_day`, `slayer_sneak_attack_dice`, ...). A wrong
//! formula here is a failing test, not a plausible number nobody checks.
//!
//! Source file for every citation in this module:
//! `pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_abilities_class.lst`
//! (sha256 `2becbb0524bd4c367cc1273434c20bcd8e42e3e08abd372f99b005e69e8c4725`,
//! the same digest `data/corpus/pathfinder_unchained/class_feature/
//! rogue_unchained_class/*.json` records). Cross-book citations name their own
//! file. Line numbers below are 1-based into that file.
//!
//! # How the Unchained Rogue stays distinct from the CRB Rogue
//!
//! They are two selections in **one single-slot PCGen ability pool**, so a
//! character has exactly one of them — the mutual exclusion is declared by the
//! corpus, not invented here (the same posture `decisions.md §26` records for
//! the ARG replace-flag protocol):
//!
//! - `core_rulebook/cr_abilitycategories.lst:255` declares
//!   `ABILITYCATEGORY:Rogue Class Selection ... POOL:Pool_Rogue_Class_Selection`.
//! - `core_rulebook/cr_abilities_class.lst:131` puts the CRB rogue in that pool
//!   as `KEY:Empty Selection ~ Standard Rogue  TYPE:Rogue Class Selection`.
//! - `pu_abilities_class.lst:116` puts this one in the same pool as
//!   `KEY:Rogue ~ Unchained Class  TYPE:Rogue Class Selection.AltRogueChoice
//!   COST:1`, and `:108` (`CATEGORY=Class|Rogue.MOD`) seeds the pool with
//!   `BONUS:VAR|Pool_Rogue_Class_Selection|1|TYPE=Base` — one point, one pick.
//!
//! (`:127` carries a byte-identical seed token but is **commented out** — the
//! line begins `#Rogue`. Cited here only so a future reader who greps for the
//! token and finds two hits knows which one is live.)
//!
//! In this codebase that separation is structural rather than conventional:
//! CRB rogue content lives under `rules_tables::crb` and answers to
//! `RuleSetId::Crb`; this module lives under `rules_tables::pathfinder_unchained`
//! and answers to `RuleSetId::Pu`; and every key here is prefixed
//! `Unchained Rogue ~ ...` where the CRB keys are `Rogue ~ ...`. Nothing in this
//! file overwrites, shadows, or mutates a CRB table.
//!
//! **What actually differs mechanically** (verified against
//! `core_rulebook/cr_abilities_class.lst`, not assumed):
//!
//! | | CRB Rogue | Unchained Rogue |
//! |---|---|---|
//! | Sneak Attack | `(RogueSneakAttackLVL+1)/2` (`:1615`) | same formula, own row (`:589`) |
//! | Rogue Talents | `RogueTalentLVL/2` (`:1616`) | same formula, own row (`:587`) |
//! | Trapfinding | `max(TrapfindingLVL/2,1)` (`:1617`) | same formula, own row (`:590`) |
//! | Master Strike DC | `10+(MasterStrikeLVL/2)+INT` (`:1619`) | same formula, own row (`:586`) |
//! | level/3 defensive scaler | **Trap Sense** (`:1618`) — Reflex + dodge AC vs traps | **Danger Sense** (`:582`) — *also* Perception vs surprise (`:446`) |
//! | Finesse Training | absent | `(RogueLvl+5)/8` Dex-to-damage picks (`:585`) |
//! | Debilitating Injury | absent | `:583` |
//! | Rogue's Edge | absent | `RogueLVL/5` skill unlocks (`:588`) |
//!
//! Four formulas are numerically identical between the two rogues. That is a
//! corpus fact, and it is recorded here rather than deduplicated: they are
//! separate rows on separate records, and a future errata to one must not
//! silently move the other.
//!
//! # Gating convention
//!
//! PCGen defines these `VAR`s at every class level; the *ability* that carries
//! each `BONUS:` is what a level gate withholds. Each function here therefore
//! returns `None` unless the level is in `1..=MAX_SUPPORTED_LEVEL` **and** at or
//! above the granting row's `PREVARGTEQ:Rogue_CFP_Level,N` (see
//! [`UnchainedRogueFeature::min_level`]). Returning a live number for a feature
//! the character does not have would be exactly the silently-wrong answer
//! `decisions.md §24.1` exists to prevent.
//!
//! `MAX_SUPPORTED_LEVEL` is the **base** class's cap: PU declares no `CLASS:`
//! record of its own (the whole book declares zero), so the chassis is
//! `core_rulebook/cr_classes.lst:237`'s `CLASS:Rogue ... HD:8 MAXLEVEL:20`, and
//! `data/corpus/pathfinder_unchained/class/rogue_unchained_class.json` leaves
//! every chassis field `null` because PU overrides none of them.

/// `MAXLEVEL:20` on the base `CLASS:Rogue` record
/// (`core_rulebook/cr_classes.lst:237`). PU overrides no chassis field for the
/// rogue, so the variant inherits this cap unchanged.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

/// `BONUS:VAR|SneakAttackDieSize|6|TYPE=Base` on the shared
/// `KEY:Sneak Attack` record (`core_rulebook/cr_abilities_class.lst:2868`),
/// which `pu_abilities_class.lst:602` re-`TYPE`s into
/// `Unchained Rogue Class Feature` rather than redefining. d6, unchanged by PU.
pub const SNEAK_ATTACK_DIE_SIZE: u8 = 6;

/// The 15 `Unchained Rogue ~ ...` records this book declares — the same 15 that
/// `data/corpus/pathfinder_unchained/class_feature/rogue_unchained_class/`
/// holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnchainedRogueFeature {
    ArmorProficiency,
    DangerSense,
    DebilitatingInjury,
    Evasion,
    FinesseTraining,
    ImprovedUncannyDodge,
    MasterStrike,
    RogueTalents,
    RoguesEdge,
    Skills,
    SneakAttack,
    Trapfinding,
    UncannyDodge,
    UncannyDodgeTracker,
    WeaponProficiency,
}

impl UnchainedRogueFeature {
    /// Source order of the declaring rows (`pu_abilities_class.lst:579..594`).
    pub const ALL: &'static [UnchainedRogueFeature] = &[
        UnchainedRogueFeature::Skills,
        UnchainedRogueFeature::WeaponProficiency,
        UnchainedRogueFeature::ArmorProficiency,
        UnchainedRogueFeature::DangerSense,
        UnchainedRogueFeature::DebilitatingInjury,
        UnchainedRogueFeature::Evasion,
        UnchainedRogueFeature::FinesseTraining,
        UnchainedRogueFeature::MasterStrike,
        UnchainedRogueFeature::RogueTalents,
        UnchainedRogueFeature::RoguesEdge,
        UnchainedRogueFeature::SneakAttack,
        UnchainedRogueFeature::Trapfinding,
        UnchainedRogueFeature::UncannyDodge,
        UnchainedRogueFeature::ImprovedUncannyDodge,
        UnchainedRogueFeature::UncannyDodgeTracker,
    ];

    /// The corpus `KEY:` token, verbatim. Deliberately `Unchained Rogue ~ ...`
    /// so it can never collide with a CRB `Rogue ~ ...` key.
    pub fn key(self) -> &'static str {
        match self {
            UnchainedRogueFeature::ArmorProficiency => "Unchained Rogue ~ Armor Proficiency",
            UnchainedRogueFeature::DangerSense => "Unchained Rogue ~ Danger Sense",
            UnchainedRogueFeature::DebilitatingInjury => "Unchained Rogue ~ Debilitating Injury",
            UnchainedRogueFeature::Evasion => "Unchained Rogue ~ Evasion",
            UnchainedRogueFeature::FinesseTraining => "Unchained Rogue ~ Finesse Training",
            UnchainedRogueFeature::ImprovedUncannyDodge => "Unchained Rogue ~ Improved Uncanny Dodge",
            UnchainedRogueFeature::MasterStrike => "Unchained Rogue ~ Master Strike",
            UnchainedRogueFeature::RogueTalents => "Unchained Rogue ~ Rogue Talents",
            UnchainedRogueFeature::RoguesEdge => "Unchained Rogue ~ Rogues Edge",
            UnchainedRogueFeature::Skills => "Unchained Rogue ~ Skills",
            UnchainedRogueFeature::SneakAttack => "Unchained Rogue ~ Sneak Attack",
            UnchainedRogueFeature::Trapfinding => "Unchained Rogue ~ Trapfinding",
            UnchainedRogueFeature::UncannyDodge => "Unchained Rogue ~ Uncanny Dodge",
            UnchainedRogueFeature::UncannyDodgeTracker => "Unchained Rogue ~ Uncanny Dodge Tracker",
            UnchainedRogueFeature::WeaponProficiency => "Unchained Rogue ~ Weapon Proficiency",
        }
    }

    /// The corpus row's display name (first column). Note `Rogue's Edge` keeps
    /// its apostrophe in the name while the `KEY:` spells it `Rogues Edge` —
    /// that asymmetry is in the corpus (`:588`), not introduced here.
    pub fn name(self) -> &'static str {
        match self {
            UnchainedRogueFeature::ArmorProficiency => "Armor Proficiency",
            UnchainedRogueFeature::DangerSense => "Danger Sense",
            UnchainedRogueFeature::DebilitatingInjury => "Debilitating Injury",
            UnchainedRogueFeature::Evasion => "Evasion",
            UnchainedRogueFeature::FinesseTraining => "Finesse Training",
            UnchainedRogueFeature::ImprovedUncannyDodge => "Improved Uncanny Dodge",
            UnchainedRogueFeature::MasterStrike => "Master Strike",
            UnchainedRogueFeature::RogueTalents => "Rogue Talents",
            UnchainedRogueFeature::RoguesEdge => "Rogue's Edge",
            UnchainedRogueFeature::Skills => "Skills",
            UnchainedRogueFeature::SneakAttack => "Sneak Attack",
            UnchainedRogueFeature::Trapfinding => "Trapfinding",
            UnchainedRogueFeature::UncannyDodge => "Uncanny Dodge",
            UnchainedRogueFeature::UncannyDodgeTracker => "Unchained Rogue ~ Uncanny Dodge Tracker",
            UnchainedRogueFeature::WeaponProficiency => "Weapon Proficiency",
        }
    }

    /// 1-based line of the row that *declares* this feature in
    /// `pu_abilities_class.lst`.
    pub fn declaring_line(self) -> u32 {
        match self {
            UnchainedRogueFeature::Skills => 579,
            UnchainedRogueFeature::WeaponProficiency => 580,
            UnchainedRogueFeature::ArmorProficiency => 581,
            UnchainedRogueFeature::DangerSense => 582,
            UnchainedRogueFeature::DebilitatingInjury => 583,
            UnchainedRogueFeature::Evasion => 584,
            UnchainedRogueFeature::FinesseTraining => 585,
            UnchainedRogueFeature::MasterStrike => 586,
            UnchainedRogueFeature::RogueTalents => 587,
            UnchainedRogueFeature::RoguesEdge => 588,
            UnchainedRogueFeature::SneakAttack => 589,
            UnchainedRogueFeature::Trapfinding => 590,
            UnchainedRogueFeature::UncannyDodge => 591,
            UnchainedRogueFeature::ImprovedUncannyDodge => 592,
            UnchainedRogueFeature::UncannyDodgeTracker => 594,
        }
    }

    /// The `PREVARGTEQ:Rogue_CFP_Level,N` on this feature's progression row
    /// (`pu_abilities_class.lst:217..229`), i.e. the class level it is first
    /// granted at.
    ///
    /// `None` for `UncannyDodge` and `ImprovedUncannyDodge`: **no progression
    /// row grants either one.** That is a real corpus fact, not an ingestion
    /// gap — the 13 grant rows at `:217..229` never name them. What actually
    /// turns them on is `UncannyDodgeTracker` (`:594`), whose
    /// `BONUS:VAR|UncannyDodgeLVL|1|PREVARGTEQ:RogueLVL,4` and `,8` clauses
    /// drive the shared `Uncanny Dodge ~ Base` machinery. See
    /// [`uncanny_dodge_tracker_steps`].
    pub fn min_level(self) -> Option<u8> {
        match self {
            UnchainedRogueFeature::ArmorProficiency
            | UnchainedRogueFeature::FinesseTraining
            | UnchainedRogueFeature::Skills
            | UnchainedRogueFeature::SneakAttack
            | UnchainedRogueFeature::Trapfinding
            | UnchainedRogueFeature::WeaponProficiency => Some(1),
            UnchainedRogueFeature::Evasion | UnchainedRogueFeature::RogueTalents => Some(2),
            UnchainedRogueFeature::DangerSense => Some(3),
            UnchainedRogueFeature::DebilitatingInjury | UnchainedRogueFeature::UncannyDodgeTracker => Some(4),
            UnchainedRogueFeature::RoguesEdge => Some(5),
            UnchainedRogueFeature::MasterStrike => Some(20),
            UnchainedRogueFeature::UncannyDodge | UnchainedRogueFeature::ImprovedUncannyDodge => None,
        }
    }

    /// The row's `SOURCEPAGE:` token, verbatim, or `None` where the row carries
    /// none. Only 3 of the 15 rows cite a page; the other 12 genuinely have no
    /// `SOURCEPAGE:` token. No page is invented for them.
    ///
    /// (This file carries zero `SOURCEPAGE:p.xx` placeholders — the pathology
    /// `decisions.md §27.2` found pervasive in the race rows — so a token here
    /// is either a real page or absent, with no third case.)
    pub fn source_page(self) -> Option<&'static str> {
        match self {
            UnchainedRogueFeature::FinesseTraining | UnchainedRogueFeature::Trapfinding => Some("p.20"),
            UnchainedRogueFeature::MasterStrike => Some("p.24"),
            _ => None,
        }
    }

    /// Whether a rogue of `level` has this feature. `false` for every level
    /// when [`min_level`](Self::min_level) is `None`.
    pub fn is_granted_at(self, level: u8) -> bool {
        match self.min_level() {
            Some(min) => (1..=MAX_SUPPORTED_LEVEL).contains(&level) && level >= min,
            None => false,
        }
    }
}

/// Shared gate: `Some(level)` only when `level` is a legal class level at which
/// `feature` is granted.
fn active_level(feature: UnchainedRogueFeature, level: u8) -> Option<u8> {
    feature.is_granted_at(level).then_some(level)
}

/// `CSKILL:` on `Unchained Rogue ~ Skills` (`:579`), verbatim and in source
/// order. `TYPE=` entries are PCGen skill-*type* selectors (every Craft, every
/// Perform, every Profession), preserved as written rather than expanded — the
/// expansion would need the skill corpus this module does not own.
///
/// Identical to the CRB rogue's list? **No** — checked, not assumed: this row
/// is its own `CSKILL:` on its own record, and the Unchained Rogue keeps
/// Knowledge (Dungeoneering) and Knowledge (Local) only, exactly as printed.
pub fn class_skills() -> &'static [&'static str] {
    &[
        "Acrobatics",
        "Appraise",
        "Bluff",
        "Climb",
        "TYPE=Craft",
        "Diplomacy",
        "Disable Device",
        "Disguise",
        "Escape Artist",
        "Intimidate",
        "Knowledge (Dungeoneering)",
        "Knowledge (Local)",
        "Linguistics",
        "Perception",
        "TYPE=Perform",
        "TYPE=Profession",
        "Sense Motive",
        "Sleight of Hand",
        "Stealth",
        "Swim",
        "Use Magic Device",
    ]
}

/// Sneak Attack dice, in `d`[`SNEAK_ATTACK_DIE_SIZE`].
///
/// Two rows, both quoted verbatim:
/// - `:589` `BONUS:VAR|RogueSneakAttackLVL|RogueLVL` — the Unchained Rogue's own
///   row sets the driving var to the class level, 1:1.
/// - `core_rulebook/cr_abilities_class.lst:2868`
///   `BONUS:VAR|SneakAttackDice|(RogueSneakAttackLVL+1)/2` on the shared
///   `KEY:Sneak Attack` record that `:589`'s
///   `ABILITY:Unchained Rogue Class Feature|AUTOMATIC|Sneak Attack` grants.
///
/// Substituting the first into the second gives `(level+1)/2`, PCGen integer
/// division. 1d6 at 1st, +1d6 every odd level, 10d6 at 20th.
///
/// The arithmetic is widened to `i16` before dividing, matching the established
/// `slayer_sneak_attack_dice` convention. That is deliberate on both counts: it
/// keeps the expression's literal `(x + 1) / 2` shape identical to the corpus
/// token — which `clippy::manual_div_ceil` would otherwise rewrite into
/// `div_ceil(2)` and destroy the correspondence — and it matches the return type
/// the other 27 classes' feature functions already use.
pub fn sneak_attack_dice(level: u8) -> Option<i16> {
    active_level(UnchainedRogueFeature::SneakAttack, level).map(|level| (i16::from(level) + 1) / 2)
}

/// Trapfinding bonus: `BONUS:VAR|TrapfindingBonus|max(TrapfindingLVL/2,1)` with
/// `BONUS:VAR|TrapfindingLVL|RogueLVL`, both on `:590`.
///
/// Applied by that same row as `BONUS:SKILL|Disable Device|TrapfindingBonus` and
/// `BONUS:SITUATION|Perception=Trapfinding|TrapfindingBonus`. The `max(...,1)`
/// is why a 1st-level rogue gets +1 rather than +0.
pub fn trapfinding_bonus(level: u8) -> Option<i16> {
    active_level(UnchainedRogueFeature::Trapfinding, level).map(|level| core::cmp::max(i16::from(level) / 2, 1))
}

/// Danger Sense bonus: `BONUS:VAR|TrapSenseBonus|RogueTrapSenseLVL/3` with
/// `BONUS:VAR|RogueTrapSenseLVL|RogueLVL`, both on `:582`.
///
/// The bonus lands on three things, per the shared `Danger Sense` ability at
/// `:446`: Reflex saves to avoid traps, dodge AC against traps, and — the part
/// CRB's Trap Sense does *not* have — `BONUS:SITUATION|Perception=Avoid Surprise`.
/// First granted at 3rd (`:225`), so +1 at 3rd through +6 at 20th.
pub fn danger_sense_bonus(level: u8) -> Option<i16> {
    active_level(UnchainedRogueFeature::DangerSense, level).map(|level| i16::from(level) / 3)
}

/// Rogue talents known: `BONUS:ABILITYPOOL|Unchained Rogue Talent|RogueTalentLVL/2`
/// with `BONUS:VAR|RogueTalentLVL|RogueLVL`, both on `:587`.
///
/// 1 at 2nd, 10 at 20th. The **selectable talents themselves are not modelled
/// here** — `decisions.md §24`'s hand-modelling ruling covers class features,
/// and the 27 Unchained Rogue Talents plus 16 Advanced Rogue Talents are
/// options a feature offers, filed as their own content-kind. This function
/// answers "how many picks", never "which".
pub fn rogue_talents_known(level: u8) -> Option<u8> {
    active_level(UnchainedRogueFeature::RogueTalents, level).map(|level| level / 2)
}

/// Finesse Training Dex-to-damage weapon choices:
/// `BONUS:ABILITYPOOL|Unchained Rogue Finesse Damage Choice|RogueFinesseTrainingLVL`
/// with `BONUS:VAR|RogueFinesseTrainingLVL|(RogueLvl+5)/8`, both on `:585`.
/// (The corpus spells the same variable `RogueLvl` here and `RogueLVL`
/// elsewhere; PCGen variable names are case-insensitive. Transcribed as the
/// same quantity, which is what it is.)
///
/// `(level+5)/8` yields **0 at 1st and 2nd**, 1 from 3rd, 2 from 11th, 3 from
/// 19th — matching the row's own `DESC:` ("starting at 3rd level ... a second
/// weapon at 11th level and a third at 19th"). `Some(0)` at 1st–2nd is correct
/// and is not a missing answer: the feature *is* granted at 1st (`:218`), where
/// it confers `ABILITY:FEAT|AUTOMATIC|Weapon Finesse` and no weapon choice yet.
pub fn finesse_training_weapon_choices(level: u8) -> Option<u8> {
    active_level(UnchainedRogueFeature::FinesseTraining, level).map(|level| (level + 5) / 8)
}

/// Rogue's Edge skill unlocks:
/// `BONUS:ABILITYPOOL|Skill Unlock Choice|RoguesEdgeLVL` with
/// `BONUS:VAR|RoguesEdgeLVL|RogueLVL/5`, both on `:588`. 1 at 5th, 4 at 20th.
///
/// This is the one Unchained Rogue feature whose **prose** could not be
/// honestly rendered — its `DESC:` is three mutually exclusive fragments gated
/// on `RoguesEdgeLVL`, so the corpus record carries `description: null`. The
/// *number*, though, comes off a single unambiguous `BONUS:VAR|` on the same
/// row, and is modelled here.
pub fn rogues_edge_skill_unlocks(level: u8) -> Option<u8> {
    active_level(UnchainedRogueFeature::RoguesEdge, level).map(|level| level / 5)
}

/// Master Strike save DC: `BONUS:VAR|MasterStrikeDC|10+(MasterStrikeLVL/2)+INT`
/// with `BONUS:VAR|MasterStrikeLVL|RogueLVL`, both on `:586`.
///
/// `INT` in a PCGen formula is the **modifier**, not the score — cross-checked
/// against `:647`, where a 0-level spell-like ability's DC is written `10+INT`
/// and a 1st-level one `11+INT` (`:646`).
///
/// Granted only at 20th (`:229`), so this returns `None` below that: a
/// "Master Strike DC" for a 7th-level rogue is not a smaller number, it is not
/// a thing.
pub fn master_strike_dc(level: u8, int_modifier: i16) -> Option<i16> {
    active_level(UnchainedRogueFeature::MasterStrike, level).map(|level| 10 + i16::from(level) / 2 + int_modifier)
}

/// The flanking level Uncanny Dodge protects against:
/// `BONUS:VAR|UncannyDodgeFlankingLevel|RogueLVL|TYPE=EachClass.REPLACE` — i.e.
/// only a rogue of `level + 4` or higher can flank this rogue.
///
/// Read off `:594` (the Tracker), **not** `:591` (`Unchained Rogue ~ Uncanny
/// Dodge`), even though both rows carry a byte-identical token. `:591` is never
/// granted by any progression row, so its copy is dead; `:594`'s copy carries
/// the live gate `PREVARGTEQ:RogueLVL,4|PREVAREQ:Rogue_CF_UncannyDodge,0` and is
/// what a real character actually gets.
pub fn uncanny_dodge_flanking_level(level: u8) -> Option<u8> {
    active_level(UnchainedRogueFeature::UncannyDodgeTracker, level)
}

/// `BONUS:VAR|UncannyDodgeLVL|1|PREVARGTEQ:RogueLVL,4` plus
/// `BONUS:VAR|UncannyDodgeLVL|1|PREVARGTEQ:RogueLVL,8`, both on `:594`.
///
/// Two independent `+1`s, so the value is 1 from 4th (Uncanny Dodge) and 2 from
/// 8th (Improved Uncanny Dodge). This is the mechanism that grants the two
/// features `min_level()` reports as `None` — see that method's note.
pub fn uncanny_dodge_tracker_steps(level: u8) -> Option<u8> {
    active_level(UnchainedRogueFeature::UncannyDodgeTracker, level)
        .map(|level| u8::from(level >= 4) + u8::from(level >= 8))
}

/// Numbers stated only in a row's English `DESC:`, never in a `BONUS:`/`DEFINE:`
/// token.
///
/// Kept in its own module so the distinction is visible at every call site:
/// everything outside `prose_derived` was transcribed from a machine-readable
/// formula token; everything inside it was read out of a sentence. Each item
/// quotes the exact sentence it came from, and a test pins that sentence against
/// the ingested corpus record so a corpus edit cannot silently invalidate the
/// reading.
pub mod prose_derived {
    use super::{UnchainedRogueFeature, active_level};

    /// Debilitating Injury's flat penalty against *everyone*, from `:583`:
    /// "the target becomes bewildered, taking a -2 penalty to AC".
    pub const GENERAL_PENALTY: i16 = -2;

    /// Debilitating Injury's total penalty against the rogue's own attacks.
    ///
    /// From `:583`, verbatim: "The target takes an additional -2 penalty to AC
    /// against all attacks made by the rogue. At 10th level and 16th level, the
    /// penalty to AC against attacks made by the rogue increases by -2 (to a
    /// total maximum of -8)."
    ///
    /// `-4` from 4th, `-6` from 10th, `-8` from 16th. The sentence is ambiguous
    /// about whether "increases by -2" applies to the additional penalty or the
    /// combined one — but **both readings converge** on the same three numbers,
    /// and the stated cap of `-8` confirms them, so nothing is being guessed
    /// between competing answers.
    ///
    /// The row states the identical escalation for the Disoriented option's
    /// attack-roll penalty; this function covers both.
    pub fn penalty_vs_the_rogue(level: u8) -> Option<i16> {
        active_level(UnchainedRogueFeature::DebilitatingInjury, level)
            .map(|level| -4 - 2 * (i16::from(level >= 10) + i16::from(level >= 16)))
    }
}

#[cfg(test)]
mod tests {
    use super::prose_derived;
    use super::*;

    #[test]
    fn every_declared_feature_is_enumerated_exactly_once() {
        assert_eq!(UnchainedRogueFeature::ALL.len(), 15, "the corpus declares 15 Unchained Rogue ~ rows");
        let mut keys: Vec<&str> = UnchainedRogueFeature::ALL.iter().map(|f| f.key()).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), 15, "no duplicate keys");
    }

    #[test]
    fn every_key_is_namespaced_away_from_the_crb_rogue() {
        for feature in UnchainedRogueFeature::ALL {
            assert!(
                feature.key().starts_with("Unchained Rogue ~ "),
                "{} must not be able to collide with a CRB `Rogue ~ ...` key",
                feature.key()
            );
        }
    }

    #[test]
    fn all_is_in_declaring_source_order() {
        let lines: Vec<u32> = UnchainedRogueFeature::ALL.iter().map(|f| f.declaring_line()).collect();
        let mut sorted = lines.clone();
        sorted.sort_unstable();
        assert_eq!(lines, sorted, "ALL should list features in pu_abilities_class.lst order");
    }

    #[test]
    fn min_levels_match_the_thirteen_progression_rows() {
        // pu_abilities_class.lst:217..229 — 13 rows, transcribed as pairs.
        let expected: &[(UnchainedRogueFeature, u8)] = &[
            (UnchainedRogueFeature::ArmorProficiency, 1),
            (UnchainedRogueFeature::FinesseTraining, 1),
            (UnchainedRogueFeature::Skills, 1),
            (UnchainedRogueFeature::SneakAttack, 1),
            (UnchainedRogueFeature::Trapfinding, 1),
            (UnchainedRogueFeature::WeaponProficiency, 1),
            (UnchainedRogueFeature::Evasion, 2),
            (UnchainedRogueFeature::RogueTalents, 2),
            (UnchainedRogueFeature::DangerSense, 3),
            (UnchainedRogueFeature::DebilitatingInjury, 4),
            (UnchainedRogueFeature::UncannyDodgeTracker, 4),
            (UnchainedRogueFeature::RoguesEdge, 5),
            (UnchainedRogueFeature::MasterStrike, 20),
        ];
        assert_eq!(expected.len(), 13, "the corpus carries 13 grant rows for the rogue");
        for (feature, min) in expected {
            assert_eq!(feature.min_level(), Some(*min), "{}", feature.key());
        }
    }

    #[test]
    fn the_two_ungranted_features_report_no_min_level() {
        // A real corpus fact: no :217..229 row names either one.
        assert_eq!(UnchainedRogueFeature::UncannyDodge.min_level(), None);
        assert_eq!(UnchainedRogueFeature::ImprovedUncannyDodge.min_level(), None);
        for level in 1..=MAX_SUPPORTED_LEVEL {
            assert!(!UnchainedRogueFeature::UncannyDodge.is_granted_at(level));
            assert!(!UnchainedRogueFeature::ImprovedUncannyDodge.is_granted_at(level));
        }
    }

    #[test]
    fn only_three_rows_cite_a_page_and_none_is_a_placeholder() {
        let cited: Vec<&str> = UnchainedRogueFeature::ALL.iter().filter_map(|f| f.source_page()).collect();
        assert_eq!(cited.len(), 3, "3 of 15 rows carry SOURCEPAGE:");
        for page in cited {
            assert_ne!(page, "p.xx", "decisions.md §27.2 placeholder must never be transcribed as a real page");
        }
    }

    #[test]
    fn class_skill_list_is_the_verbatim_cskill_row() {
        let skills = class_skills();
        assert_eq!(skills.len(), 21, "CSKILL: on :579 has 21 pipe-separated entries");
        assert_eq!(skills[0], "Acrobatics");
        assert_eq!(skills[20], "Use Magic Device");
        assert_eq!(
            skills.iter().filter(|s| s.starts_with("TYPE=")).count(),
            3,
            "TYPE=Craft, TYPE=Perform, TYPE=Profession are preserved unexpanded"
        );
        assert!(skills.contains(&"Knowledge (Dungeoneering)"));
        assert!(skills.contains(&"Knowledge (Local)"));
        assert!(!skills.contains(&"TYPE=Knowledge"), "the rogue gets two named Knowledges, not all of them");
    }

    // ---- formula pins -----------------------------------------------------

    #[test]
    fn sneak_attack_dice_is_level_plus_one_over_two() {
        // (RogueSneakAttackLVL+1)/2 with RogueSneakAttackLVL = RogueLVL.
        let expected = [1i16, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10];
        for (idx, want) in expected.iter().enumerate() {
            let level = idx as u8 + 1;
            assert_eq!(sneak_attack_dice(level), Some(*want), "level {level}");
        }
        assert_eq!(SNEAK_ATTACK_DIE_SIZE, 6);
    }

    #[test]
    fn trapfinding_bonus_is_half_level_floored_at_one() {
        assert_eq!(trapfinding_bonus(1), Some(1), "max(1/2,1) = 1, not 0");
        assert_eq!(trapfinding_bonus(2), Some(1));
        assert_eq!(trapfinding_bonus(3), Some(1));
        assert_eq!(trapfinding_bonus(4), Some(2));
        assert_eq!(trapfinding_bonus(20), Some(10));
    }

    #[test]
    fn danger_sense_bonus_is_a_third_of_level_from_third() {
        assert_eq!(danger_sense_bonus(2), None, "granted at 3rd (:225)");
        assert_eq!(danger_sense_bonus(3), Some(1));
        assert_eq!(danger_sense_bonus(5), Some(1));
        assert_eq!(danger_sense_bonus(6), Some(2));
        assert_eq!(danger_sense_bonus(18), Some(6));
        assert_eq!(danger_sense_bonus(20), Some(6));
    }

    #[test]
    fn rogue_talents_known_is_half_level_from_second() {
        assert_eq!(rogue_talents_known(1), None, "granted at 2nd (:224)");
        assert_eq!(rogue_talents_known(2), Some(1));
        assert_eq!(rogue_talents_known(3), Some(1));
        assert_eq!(rogue_talents_known(4), Some(2));
        assert_eq!(rogue_talents_known(20), Some(10));
    }

    #[test]
    fn finesse_training_choices_open_at_three_eleven_and_nineteen() {
        assert_eq!(finesse_training_weapon_choices(1), Some(0), "granted at 1st, but 0 picks: (1+5)/8 = 0");
        assert_eq!(finesse_training_weapon_choices(2), Some(0));
        assert_eq!(finesse_training_weapon_choices(3), Some(1));
        assert_eq!(finesse_training_weapon_choices(10), Some(1));
        assert_eq!(finesse_training_weapon_choices(11), Some(2));
        assert_eq!(finesse_training_weapon_choices(18), Some(2));
        assert_eq!(finesse_training_weapon_choices(19), Some(3));
        assert_eq!(finesse_training_weapon_choices(20), Some(3));
    }

    #[test]
    fn rogues_edge_unlocks_every_fifth_level() {
        assert_eq!(rogues_edge_skill_unlocks(4), None, "granted at 5th (:228)");
        assert_eq!(rogues_edge_skill_unlocks(5), Some(1));
        assert_eq!(rogues_edge_skill_unlocks(9), Some(1));
        assert_eq!(rogues_edge_skill_unlocks(10), Some(2));
        assert_eq!(rogues_edge_skill_unlocks(15), Some(3));
        assert_eq!(rogues_edge_skill_unlocks(20), Some(4));
    }

    #[test]
    fn master_strike_dc_exists_only_at_twentieth() {
        assert_eq!(master_strike_dc(19, 5), None, "granted at 20th (:229)");
        assert_eq!(master_strike_dc(20, 0), Some(20), "10 + 20/2 + 0");
        assert_eq!(master_strike_dc(20, 5), Some(25));
        assert_eq!(master_strike_dc(20, -1), Some(19), "a negative Int modifier is not clamped by the token");
    }

    #[test]
    fn uncanny_dodge_comes_online_at_four_and_eight() {
        assert_eq!(uncanny_dodge_tracker_steps(3), None);
        assert_eq!(uncanny_dodge_tracker_steps(4), Some(1));
        assert_eq!(uncanny_dodge_tracker_steps(7), Some(1));
        assert_eq!(uncanny_dodge_tracker_steps(8), Some(2));
        assert_eq!(uncanny_dodge_tracker_steps(20), Some(2));

        assert_eq!(uncanny_dodge_flanking_level(3), None);
        assert_eq!(uncanny_dodge_flanking_level(4), Some(4));
        assert_eq!(uncanny_dodge_flanking_level(20), Some(20));
    }

    #[test]
    fn debilitating_injury_penalty_escalates_to_the_stated_cap() {
        assert_eq!(prose_derived::GENERAL_PENALTY, -2);
        assert_eq!(prose_derived::penalty_vs_the_rogue(3), None, "granted at 4th (:226)");
        assert_eq!(prose_derived::penalty_vs_the_rogue(4), Some(-4));
        assert_eq!(prose_derived::penalty_vs_the_rogue(9), Some(-4));
        assert_eq!(prose_derived::penalty_vs_the_rogue(10), Some(-6));
        assert_eq!(prose_derived::penalty_vs_the_rogue(15), Some(-6));
        assert_eq!(prose_derived::penalty_vs_the_rogue(16), Some(-8));
        assert_eq!(
            prose_derived::penalty_vs_the_rogue(20),
            Some(-8),
            "the row states a total maximum of -8; nothing may exceed it"
        );
    }

    // ---- refusal ----------------------------------------------------------

    #[test]
    fn no_formula_answers_outside_the_legal_level_band() {
        for level in [0u8, 21, 30, 255] {
            assert_eq!(sneak_attack_dice(level), None, "level {level}");
            assert_eq!(trapfinding_bonus(level), None, "level {level}");
            assert_eq!(danger_sense_bonus(level), None, "level {level}");
            assert_eq!(rogue_talents_known(level), None, "level {level}");
            assert_eq!(finesse_training_weapon_choices(level), None, "level {level}");
            assert_eq!(rogues_edge_skill_unlocks(level), None, "level {level}");
            assert_eq!(master_strike_dc(level, 4), None, "level {level}");
            assert_eq!(uncanny_dodge_tracker_steps(level), None, "level {level}");
            assert_eq!(uncanny_dodge_flanking_level(level), None, "level {level}");
            assert_eq!(prose_derived::penalty_vs_the_rogue(level), None, "level {level}");
        }
    }

    #[test]
    fn every_formula_is_defined_across_the_whole_granted_band() {
        // No silent holes: if a feature is granted at a level, its number resolves.
        for level in 1..=MAX_SUPPORTED_LEVEL {
            assert_eq!(sneak_attack_dice(level).is_some(), UnchainedRogueFeature::SneakAttack.is_granted_at(level));
            assert_eq!(trapfinding_bonus(level).is_some(), UnchainedRogueFeature::Trapfinding.is_granted_at(level));
            assert_eq!(danger_sense_bonus(level).is_some(), UnchainedRogueFeature::DangerSense.is_granted_at(level));
            assert_eq!(rogue_talents_known(level).is_some(), UnchainedRogueFeature::RogueTalents.is_granted_at(level));
            assert_eq!(
                finesse_training_weapon_choices(level).is_some(),
                UnchainedRogueFeature::FinesseTraining.is_granted_at(level)
            );
            assert_eq!(rogues_edge_skill_unlocks(level).is_some(), UnchainedRogueFeature::RoguesEdge.is_granted_at(level));
            assert_eq!(master_strike_dc(level, 0).is_some(), UnchainedRogueFeature::MasterStrike.is_granted_at(level));
        }
    }
}
