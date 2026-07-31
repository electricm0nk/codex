//! Pathfinder Unchained — **Unchained Summoner** class features, hand-modelled
//! per `decisions.md §24.1`.
//!
//! **No formula interpreter.** Same discipline as
//! [`super::rogue_features`]: every function is hand-written, its arithmetic
//! read byte-for-byte off one named corpus row, and pinned by a test.
//!
//! Source file for every unqualified citation:
//! `pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_abilities_class.lst`
//! (sha256 `2becbb0524bd4c367cc1273434c20bcd8e42e3e08abd372f99b005e69e8c4725`).
//! Cross-book citations name their own file. Lines are 1-based.
//!
//! # How the Unchained Summoner stays distinct from the APG Summoner
//!
//! Two mechanisms, both declared by the corpus rather than designed here.
//!
//! **1. One single-slot selection pool**, as for the rogue.
//! `advanced_players_guide/apg_abilities_class.lst:741` puts the APG summoner in
//! it (`KEY:Summoner ~ Standard Class  TYPE:Summoner Class Selection  COST:1`);
//! `:117` of this book puts this one in the same pool
//! (`KEY:Summoner ~ Unchained Class  TYPE:Summoner Class Selection.SummonerAlternative
//! COST:1`); `apg_abilities_class.lst:739` seeds it with
//! `BONUS:VAR|Pool_Summoner_Class_Selection|1|TYPE=Base`.
//!
//! **2. A `StandardSummoner` flag that swaps the entire spell list.** This is
//! the part worth being precise about, because it is easy to get backwards.
//! `apg_abilities_class.lst:739` carries:
//!
//! ```text
//! BONUS:VAR|StandardSummoner|1|TYPE=Base|!PREABILITY:1,CATEGORY=Class,TYPE.Summoner Class Selection
//! ```
//!
//! — read: *`StandardSummoner` is 1 **unless** the character holds any `Summoner
//! Class Selection` ability.* Taking `Summoner ~ Unchained Class` (which is
//! exactly such an ability) drives it to 0. Every APG, Ultimate Magic, Ultimate
//! Combat and Mythic Adventures summoner spell-list row is gated
//! `|PREVAREQ:StandardSummoner,1`, so all of them switch off together; the
//! Unchained rows (`:269..275`, `:278..283`) carry no such gate and hang off the
//! Unchained ability's own `.MOD`, so only they remain. **It is a replacement,
//! not an addition** — verified rather than assumed: the file contains no
//! `.CLEAR` for the summoner list, so the flag is the whole mechanism.
//!
//! That flag is the summoner's exact analogue of the ARG replace-flag protocol
//! `decisions.md §26` records — a data relationship already stated in the
//! corpus, transcribed rather than invented.
//!
//! **The spell list content itself is not in this module.** `:269..275` declare
//! 202 spells (12 / 35 / 39 / 39 / 27 / 23 / 27 at levels 0-6). Transcribing
//! them belongs in a `summoner_spell_list.rs` slice alongside the established
//! `rules_tables::{crb,apg,acg}::*_spell_list` modules, not inside a
//! class-feature module — and it should land knowing that **46 of the 202 name
//! spells no ingested book defines** (they come from Ultimate Magic and
//! Ultimate Combat, both unregistered). That gap is a number here rather than a
//! silence; see this cycle's report.
//!
//! **What differs mechanically** (verified against
//! `advanced_players_guide/apg_abilities_class.lst`, not assumed):
//!
//! | | APG Summoner | Unchained Summoner |
//! |---|---|---|
//! | Eidolon evolution pool | `3+...` (`:813`), 3 at 1st | `1+...` (`:746`), 1 at 1st — [`eidolon_evolution_pool`] |
//! | Summon Monster uses | `CHA+3` (`:795`) | `max(CHA,0)+3` (`:733`) — never below 3 |
//! | Summon Monster var names | `SummonMonsterLVL` / `SummonMonsterTimes` | `SummonerSummonMonsterLVL` / `SummonerSummonMonsterTimes` — separate vars, no shadowing |
//! | Aspect | granted 10th–17th only (`:756` adds `PREVARLT:Summoner_CFP_Level,18`) | granted from 10th with no upper bound (`:261`) |
//! | Gate at 19th | granted (`:761`) | **not granted** — PU declares no `Gate` row (see [`UnchainedSummonerFeature::min_level`]) |
//!
//! In this codebase the separation is also structural: APG summoner content
//! lives under `rules_tables::apg` and answers to `RuleSetId::Apg`; this module
//! lives under `rules_tables::pathfinder_unchained` and answers to
//! `RuleSetId::Pu`; every key here is `Unchained Summoner ~ ...` where APG's are
//! `Summoner ~ ...`. Nothing here overwrites or shadows an APG table.
//!
//! # Gating convention
//!
//! Identical to [`super::rogue_features`]: each function returns `None` unless
//! the level is in `1..=MAX_SUPPORTED_LEVEL` **and** at or above the granting
//! row's `PREVARGTEQ:Summoner_CFP_Level,N`.
//!
//! `Summoner_CFP_Level` is not a PU concept, and two APG rows set it to the same
//! thing: `apg_abilities_globalvar.lst:9`
//! (`BONUS:VAR|Summoner_CFP_Level|SummonerLVL|TYPE=Base`) and
//! `apg_abilities_class.lst:739`
//! (`BONUS:VAR|Summoner_CFP_Level|classlevel("Summoner")`). Both are the class
//! level, 1:1, so every `Summoner_CFP_Level` below is just "summoner level".

/// `MAXLEVEL:20` on the base `CLASS:Summoner` record
/// (`advanced_players_guide/apg_classes.lst:139`, the same record
/// `rules_tables::apg::class_summoner` transcribes). PU overrides no chassis
/// field for the summoner — `data/corpus/pathfinder_unchained/class/
/// summoner_unchained_class.json` leaves `hit_die`, `bab` and all three saves
/// `null` — so the variant inherits d8 / three-quarter BAB / good Will
/// unchanged, and this cap with them.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

/// The 17 `Unchained Summoner ~ ...` records this book declares — the same 17
/// that `data/corpus/pathfinder_unchained/class_feature/
/// summoner_unchained_class/` holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnchainedSummonerFeature {
    Aspect,
    BondSenses,
    Cantrips,
    Eidolon,
    GreaterAspect,
    GreaterShieldAlly,
    LifeBond,
    LifeLink,
    MakersCall,
    MergeForms,
    ShieldAlly,
    Skills,
    Spells,
    SummonMonster,
    Transposition,
    TwinEidolon,
    WeaponAndArmorProficiency,
}

impl UnchainedSummonerFeature {
    /// Source order of the declaring rows (`:727..743`).
    pub const ALL: &'static [UnchainedSummonerFeature] = &[
        UnchainedSummonerFeature::Skills,
        UnchainedSummonerFeature::WeaponAndArmorProficiency,
        UnchainedSummonerFeature::Spells,
        UnchainedSummonerFeature::Cantrips,
        UnchainedSummonerFeature::Eidolon,
        UnchainedSummonerFeature::LifeLink,
        UnchainedSummonerFeature::SummonMonster,
        UnchainedSummonerFeature::BondSenses,
        UnchainedSummonerFeature::ShieldAlly,
        UnchainedSummonerFeature::MakersCall,
        UnchainedSummonerFeature::Transposition,
        UnchainedSummonerFeature::Aspect,
        UnchainedSummonerFeature::GreaterShieldAlly,
        UnchainedSummonerFeature::LifeBond,
        UnchainedSummonerFeature::MergeForms,
        UnchainedSummonerFeature::GreaterAspect,
        UnchainedSummonerFeature::TwinEidolon,
    ];

    /// The corpus `KEY:` token, verbatim. Deliberately `Unchained Summoner ~ ...`
    /// so it can never collide with an APG `Summoner ~ ...` key.
    pub fn key(self) -> &'static str {
        match self {
            UnchainedSummonerFeature::Aspect => "Unchained Summoner ~ Aspect",
            UnchainedSummonerFeature::BondSenses => "Unchained Summoner ~ Bond Senses",
            UnchainedSummonerFeature::Cantrips => "Unchained Summoner ~ Cantrips",
            UnchainedSummonerFeature::Eidolon => "Unchained Summoner ~ Eidolon",
            UnchainedSummonerFeature::GreaterAspect => "Unchained Summoner ~ Greater Aspect",
            UnchainedSummonerFeature::GreaterShieldAlly => "Unchained Summoner ~ Greater Shield Ally",
            UnchainedSummonerFeature::LifeBond => "Unchained Summoner ~ Life Bond",
            UnchainedSummonerFeature::LifeLink => "Unchained Summoner ~ Life Link",
            UnchainedSummonerFeature::MakersCall => "Unchained Summoner ~ Maker's Call",
            UnchainedSummonerFeature::MergeForms => "Unchained Summoner ~ Merge Forms",
            UnchainedSummonerFeature::ShieldAlly => "Unchained Summoner ~ Shield Ally",
            UnchainedSummonerFeature::Skills => "Unchained Summoner ~ Skills",
            UnchainedSummonerFeature::Spells => "Unchained Summoner ~ Spells",
            UnchainedSummonerFeature::SummonMonster => "Unchained Summoner ~ Summon Monster",
            UnchainedSummonerFeature::Transposition => "Unchained Summoner ~ Transposition",
            UnchainedSummonerFeature::TwinEidolon => "Unchained Summoner ~ Twin Eidolon",
            UnchainedSummonerFeature::WeaponAndArmorProficiency => "Unchained Summoner ~ Weapon and Armor Proficiency",
        }
    }

    /// The corpus row's display name (first column). Note `SummonMonster`'s row
    /// is named `Summon Monster I` even though the ability scales to IX — the
    /// row overrides its own display via nine `ASPECT:NAME|Summon Monster <n>`
    /// clauses keyed on `SummonerSummonMonsterLVL`. The raw first column is
    /// reproduced here; [`summon_monster_spell_level`] is what actually says
    /// which one a character has.
    pub fn name(self) -> &'static str {
        match self {
            UnchainedSummonerFeature::Aspect => "Aspect",
            UnchainedSummonerFeature::BondSenses => "Bond Senses",
            UnchainedSummonerFeature::Cantrips => "Cantrips",
            UnchainedSummonerFeature::Eidolon => "Eidolon",
            UnchainedSummonerFeature::GreaterAspect => "Greater Aspect",
            UnchainedSummonerFeature::GreaterShieldAlly => "Greater Shield Ally",
            UnchainedSummonerFeature::LifeBond => "Life Bond",
            UnchainedSummonerFeature::LifeLink => "Life Link",
            UnchainedSummonerFeature::MakersCall => "Maker's Call",
            UnchainedSummonerFeature::MergeForms => "Merge Forms",
            UnchainedSummonerFeature::ShieldAlly => "Shield Ally",
            UnchainedSummonerFeature::Skills => "Skills",
            UnchainedSummonerFeature::Spells => "Spells",
            UnchainedSummonerFeature::SummonMonster => "Summon Monster I",
            UnchainedSummonerFeature::Transposition => "Transposition",
            UnchainedSummonerFeature::TwinEidolon => "Twin Eidolon",
            UnchainedSummonerFeature::WeaponAndArmorProficiency => "Weapon and Armor Proficiency",
        }
    }

    /// 1-based line of the row that declares this feature.
    pub fn declaring_line(self) -> u32 {
        match self {
            UnchainedSummonerFeature::Skills => 727,
            UnchainedSummonerFeature::WeaponAndArmorProficiency => 728,
            UnchainedSummonerFeature::Spells => 729,
            UnchainedSummonerFeature::Cantrips => 730,
            UnchainedSummonerFeature::Eidolon => 731,
            UnchainedSummonerFeature::LifeLink => 732,
            UnchainedSummonerFeature::SummonMonster => 733,
            UnchainedSummonerFeature::BondSenses => 734,
            UnchainedSummonerFeature::ShieldAlly => 735,
            UnchainedSummonerFeature::MakersCall => 736,
            UnchainedSummonerFeature::Transposition => 737,
            UnchainedSummonerFeature::Aspect => 738,
            UnchainedSummonerFeature::GreaterShieldAlly => 739,
            UnchainedSummonerFeature::LifeBond => 740,
            UnchainedSummonerFeature::MergeForms => 741,
            UnchainedSummonerFeature::GreaterAspect => 742,
            UnchainedSummonerFeature::TwinEidolon => 743,
        }
    }

    /// The `PREVARGTEQ:Summoner_CFP_Level,N` on this feature's progression row
    /// (`:250..266`).
    ///
    /// Unlike the rogue's, **every** declared summoner feature is granted: all
    /// 17 rows appear in the 17 progression rows, so this never returns `None`
    /// and is deliberately not an `Option`.
    ///
    /// The asymmetry worth knowing is the other direction — APG's progression
    /// has an **18th** entry, `Summoner ~ Gate` at 19th
    /// (`advanced_players_guide/apg_abilities_class.lst:761`), and PU declares
    /// no counterpart. The Unchained Summon Monster row's own `DESC:` promises
    /// "At 19th level, this ability can be used as gate or summon monster IX",
    /// so the corpus under-implements its own prose. Recorded, not patched:
    /// inventing a `Gate` row PCGen does not have is exactly the fabrication
    /// `AGENTS.md` forbids.
    pub fn min_level(self) -> u8 {
        match self {
            UnchainedSummonerFeature::Skills
            | UnchainedSummonerFeature::WeaponAndArmorProficiency
            | UnchainedSummonerFeature::Spells
            | UnchainedSummonerFeature::Cantrips
            | UnchainedSummonerFeature::Eidolon
            | UnchainedSummonerFeature::LifeLink
            | UnchainedSummonerFeature::SummonMonster => 1,
            UnchainedSummonerFeature::BondSenses => 2,
            UnchainedSummonerFeature::ShieldAlly => 4,
            UnchainedSummonerFeature::MakersCall => 6,
            UnchainedSummonerFeature::Transposition => 8,
            UnchainedSummonerFeature::Aspect => 10,
            UnchainedSummonerFeature::GreaterShieldAlly => 12,
            UnchainedSummonerFeature::LifeBond => 14,
            UnchainedSummonerFeature::MergeForms => 16,
            UnchainedSummonerFeature::GreaterAspect => 18,
            UnchainedSummonerFeature::TwinEidolon => 20,
        }
    }

    /// The row's `SOURCEPAGE:` token, verbatim. Every one of the 17 summoner
    /// rows carries a real page — a markedly better citation rate than the
    /// rogue's 3-of-15 — and none is the `p.xx` placeholder `decisions.md §27.2`
    /// found pervasive elsewhere.
    pub fn source_page(self) -> &'static str {
        match self {
            UnchainedSummonerFeature::Skills
            | UnchainedSummonerFeature::WeaponAndArmorProficiency
            | UnchainedSummonerFeature::Spells
            | UnchainedSummonerFeature::Cantrips
            | UnchainedSummonerFeature::Eidolon => "p.25",
            UnchainedSummonerFeature::LifeLink | UnchainedSummonerFeature::SummonMonster => "p.26",
            UnchainedSummonerFeature::BondSenses
            | UnchainedSummonerFeature::ShieldAlly
            | UnchainedSummonerFeature::MakersCall
            | UnchainedSummonerFeature::Transposition
            | UnchainedSummonerFeature::Aspect
            | UnchainedSummonerFeature::GreaterShieldAlly
            | UnchainedSummonerFeature::LifeBond
            | UnchainedSummonerFeature::MergeForms => "p.27",
            UnchainedSummonerFeature::GreaterAspect | UnchainedSummonerFeature::TwinEidolon => "p.28",
        }
    }

    /// Whether a summoner of `level` has this feature.
    pub fn is_granted_at(self, level: u8) -> bool {
        (1..=MAX_SUPPORTED_LEVEL).contains(&level) && level >= self.min_level()
    }
}

/// Shared gate: `Some(level)` only when `level` is a legal class level at which
/// `feature` is granted.
fn active_level(feature: UnchainedSummonerFeature, level: u8) -> Option<u8> {
    feature.is_granted_at(level).then_some(level)
}

/// `CSKILL:` on `Unchained Summoner ~ Skills` (`:727`), verbatim and in source
/// order. `TYPE=` entries are PCGen skill-*type* selectors, preserved as written
/// rather than expanded.
///
/// Note `TYPE=Knowledge` — the summoner gets *every* Knowledge skill, where the
/// Unchained Rogue's row names only two. That contrast is why both lists are
/// transcribed verbatim instead of normalised into one representation.
pub fn class_skills() -> &'static [&'static str] {
    &[
        "TYPE=Craft",
        "Fly",
        "Handle Animal",
        "TYPE=Knowledge",
        "Linguistics",
        "TYPE=Profession",
        "Ride",
        "Spellcraft",
        "Use Magic Device",
    ]
}

/// The eidolon's effective companion level:
/// `BONUS:VAR|EidolonCompanionLVL|SummonerLVL` on `:731`. 1:1 with class level.
///
/// The APG summoner's `:796` carries a byte-identical token, so this one number
/// is genuinely the same in both versions — recorded rather than deduplicated,
/// for the same reason the rogue's four shared formulas are.
pub fn eidolon_companion_level(level: u8) -> Option<u8> {
    active_level(UnchainedSummonerFeature::Eidolon, level)
}

/// The eidolon's evolution pool.
///
/// `:746`, verbatim:
/// `BONUS:VAR|EidolonEvolution|1+(EidolonCompanionLVL>=2)+(EidolonCompanionLVL>=3)
/// +(EidolonCompanionLVL>=5)+(EidolonCompanionLVL>=6)+(EidolonCompanionLVL>=7)
/// +(EidolonCompanionLVL>=9)+(EidolonCompanionLVL>=10)+(EidolonCompanionLVL>=11)
/// +(EidolonCompanionLVL>=13)+(EidolonCompanionLVL>=14)+(EidolonCompanionLVL>=15)
/// +(EidolonCompanionLVL>=17)+(EidolonCompanionLVL>=18)+(EidolonCompanionLVL>=19)`
///
/// A base of 1 plus 14 independent threshold indicators — no operator this
/// module has to interpret, just a sum of booleans, which is why
/// [`EIDOLON_EVOLUTION_THRESHOLDS`] can hold the thresholds as literal data.
/// Yields 1, 2, 3, 3, 4, 5, 6, 6, 7, 8, 9, 9, 10, 11, 12, 12, 13, 14, 15, 15 —
/// the pool pauses on every 4th, 8th, 12th, 16th and 20th level.
///
/// **This is the sharpest divergence from the APG summoner.**
/// `advanced_players_guide/apg_abilities_class.lst:813` is the same shape with a
/// base of **3** and `if(...,2,0)` double steps, so the chained eidolon starts
/// three times as customisable. Getting these two confused would silently
/// mis-build every eidolon; they are separate functions in separate modules on
/// separate `RuleSetId`s precisely so they cannot be.
pub fn eidolon_evolution_pool(level: u8) -> Option<u8> {
    active_level(UnchainedSummonerFeature::Eidolon, level).map(|level| {
        1 + EIDOLON_EVOLUTION_THRESHOLDS
            .iter()
            .filter(|threshold| level >= **threshold)
            .count() as u8
    })
}

/// The 14 `EidolonCompanionLVL>=N` thresholds on `:746`, in the order the token
/// writes them. Public so a test — or an auditor — can compare them against the
/// live corpus row without re-deriving the arithmetic.
pub const EIDOLON_EVOLUTION_THRESHOLDS: [u8; 14] = [2, 3, 5, 6, 7, 9, 10, 11, 13, 14, 15, 17, 18, 19];

/// The 13 eidolon subtypes PU declares, from the 13
/// `BONUS:VAR|EidolonSubtype_<name>|1|PREVAREQ:Summoner_CF_EidolonSubtype_<name>,1`
/// rows at `:747..759`, in source order.
///
/// These are the subtype *slots the corpus opens*, not the subtypes' contents —
/// each one's evolutions and base form live in rows this cycle does not model.
/// Listing the names is transcription; claiming the eidolons are playable from
/// this list would not be.
pub fn eidolon_subtypes() -> &'static [&'static str] {
    &[
        "Agathion",
        "Angel",
        "Archon",
        "Azata",
        "Daemon",
        "Demon",
        "Devil",
        "Div",
        "Elemental",
        "Fey",
        "Inevitable",
        "Protean",
        "Psychopomp",
    ]
}

/// Which `summon monster` spell the SLA casts:
/// `BONUS:VAR|SummonerSummonMonsterLVL|min((Summoner_CFP_Level+1)/2,9)` on
/// `:733`, with `Summoner_CFP_Level = SummonerLVL`
/// (`advanced_players_guide/apg_abilities_globalvar.lst:9`).
///
/// I at 1st, II at 3rd, ..., IX from 17th — matching the row's own `DESC:` ("to
/// a maximum of summon monster IX at 17th level") and the nine
/// `ASPECT:NAME|Summon Monster <n>` clauses on the same row.
///
/// Widened to `i16` before dividing for the same reason as
/// [`super::rogue_features::sneak_attack_dice`]: it preserves the literal
/// `(x + 1) / 2` shape of the corpus token that `clippy::manual_div_ceil` would
/// otherwise rewrite.
pub fn summon_monster_spell_level(level: u8) -> Option<i16> {
    active_level(UnchainedSummonerFeature::SummonMonster, level)
        .map(|level| core::cmp::min((i16::from(level) + 1) / 2, 9))
}

/// Uses per day of the Summon Monster SLA:
/// `BONUS:VAR|SummonerSummonMonsterTimes|max(CHA,0)+3` on `:733`.
///
/// `CHA` in a PCGen formula is the **modifier**, not the score. The `max(...,0)`
/// is a genuine Unchained change: the APG summoner's `:795` writes plain
/// `CHA+3`, so a negative Charisma modifier reduces a chained summoner below 3
/// uses and cannot reduce an unchained one. That single token is why this
/// function exists separately rather than delegating to the APG one.
pub fn summon_monster_uses_per_day(level: u8, cha_modifier: i16) -> Option<i16> {
    active_level(UnchainedSummonerFeature::SummonMonster, level).map(|_| core::cmp::max(cha_modifier, 0) + 3)
}

/// The `UnchainedSummoner` marker: `DEFINE:UnchainedSummoner|0` plus
/// `BONUS:VAR|UnchainedSummoner|1` on the `Unchained Summoner ~ Spells` row
/// (`:729`).
///
/// It is not a game quantity — it is the flag the rest of the corpus reads to
/// tell which spell list is live, and it is modelled so the swap described in
/// this module's header is checkable rather than merely documented. `Some(1)`
/// from 1st level, `None` outside the legal band.
pub fn unchained_summoner_marker(level: u8) -> Option<u8> {
    active_level(UnchainedSummonerFeature::Spells, level).map(|_| 1)
}

/// Numbers stated only in a row's English `DESC:`, never in a `BONUS:`/`DEFINE:`
/// token.
///
/// Kept in its own module for the same reason as
/// [`super::rogue_features::prose_derived`]: so no call site can mistake a
/// sentence for a formula. Each item quotes the sentence it came from.
///
/// The Unchained Summoner leans on this far harder than the rogue does — 12 of
/// its 17 features carry prose and no arithmetic token at all. Where the prose
/// states no number (Life Link, Transposition, Life Bond, Merge Forms, Twin
/// Eidolon, Cantrips, and the two proficiency/skill rows) there is nothing to
/// model and nothing is invented.
pub mod prose_derived {
    use super::{UnchainedSummonerFeature, active_level};

    /// Bond Senses rounds per day, from `:734`: "He can use this ability a
    /// number of rounds per day equal to his summoner level."
    ///
    /// Unambiguous, but genuinely prose — the row carries no `BONUS:VAR|` for
    /// it, so PCGen itself does not compute this number.
    pub fn bond_senses_rounds_per_day(level: u8) -> Option<u8> {
        active_level(UnchainedSummonerFeature::BondSenses, level)
    }

    /// Maker's Call uses per day, from `:736`: "The summoner can use this
    /// ability once per day at 6th level, plus one additional time per day for
    /// every four levels beyond 6th."
    ///
    /// `1 + (level - 6) / 4`: 1 at 6th, 2 at 10th, 3 at 14th, 4 at 18th, and
    /// still 4 at 20th. Also carries no `BONUS:VAR|`.
    pub fn makers_call_uses_per_day(level: u8) -> Option<u8> {
        active_level(UnchainedSummonerFeature::MakersCall, level).map(|level| 1 + (level - 6) / 4)
    }

    /// Aspect evolution points, from `:738`: "a summoner can divert up to 2
    /// points from his eidolon's evolution pool to add evolutions to himself."
    pub const ASPECT_POINTS: u8 = 2;

    /// Greater Aspect evolution points, from `:742`: "the maximum number of
    /// evolution points the summoner can divert increases to 6."
    pub const GREATER_ASPECT_POINTS: u8 = 6;

    /// Evolution points a summoner may divert to himself at `level`: 0 below
    /// 10th, [`ASPECT_POINTS`] from 10th, [`GREATER_ASPECT_POINTS`] from 18th.
    ///
    /// Greater Aspect also changes the *exchange rate* — `:742`: "the eidolon
    /// loses 1 point from its evolution pool for every 2 points (or fraction
    /// thereof) diverted" — which is a second, separate rule and is **not**
    /// folded into this number.
    pub fn divertible_evolution_points(level: u8) -> Option<u8> {
        if !(1..=super::MAX_SUPPORTED_LEVEL).contains(&level) {
            return None;
        }
        if UnchainedSummonerFeature::GreaterAspect.is_granted_at(level) {
            Some(GREATER_ASPECT_POINTS)
        } else if UnchainedSummonerFeature::Aspect.is_granted_at(level) {
            Some(ASPECT_POINTS)
        } else {
            Some(0)
        }
    }

    /// Shield Ally, from `:735`: "the summoner gains a +2 shield bonus to his
    /// Armor Class and a +2 circumstance bonus on his saving throws."
    pub const SHIELD_ALLY_BONUS: i16 = 2;

    /// Greater Shield Ally's bonus **to the summoner himself**, from `:739`:
    /// "the ally gains a +2 shield bonus ... If this ally is the summoner,
    /// these bonuses increase to +4."
    ///
    /// Allies get [`SHIELD_ALLY_BONUS`]; only the summoner gets this.
    pub const GREATER_SHIELD_ALLY_SELF_BONUS: i16 = 4;
}

#[cfg(test)]
mod tests {
    use super::prose_derived;
    use super::*;

    #[test]
    fn every_declared_feature_is_enumerated_exactly_once() {
        assert_eq!(UnchainedSummonerFeature::ALL.len(), 17, "the corpus declares 17 Unchained Summoner ~ rows");
        let mut keys: Vec<&str> = UnchainedSummonerFeature::ALL.iter().map(|f| f.key()).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), 17, "no duplicate keys");
    }

    #[test]
    fn every_key_is_namespaced_away_from_the_apg_summoner() {
        for feature in UnchainedSummonerFeature::ALL {
            assert!(
                feature.key().starts_with("Unchained Summoner ~ "),
                "{} must not be able to collide with an APG `Summoner ~ ...` key",
                feature.key()
            );
        }
    }

    #[test]
    fn all_is_in_declaring_source_order() {
        let lines: Vec<u32> = UnchainedSummonerFeature::ALL.iter().map(|f| f.declaring_line()).collect();
        let mut sorted = lines.clone();
        sorted.sort_unstable();
        assert_eq!(lines, sorted, "ALL should list features in pu_abilities_class.lst order");
    }

    #[test]
    fn min_levels_match_the_seventeen_progression_rows() {
        // pu_abilities_class.lst:250..266 — 17 rows, transcribed as pairs.
        let expected: &[(UnchainedSummonerFeature, u8)] = &[
            (UnchainedSummonerFeature::Skills, 1),
            (UnchainedSummonerFeature::WeaponAndArmorProficiency, 1),
            (UnchainedSummonerFeature::Spells, 1),
            (UnchainedSummonerFeature::Cantrips, 1),
            (UnchainedSummonerFeature::Eidolon, 1),
            (UnchainedSummonerFeature::LifeLink, 1),
            (UnchainedSummonerFeature::SummonMonster, 1),
            (UnchainedSummonerFeature::BondSenses, 2),
            (UnchainedSummonerFeature::ShieldAlly, 4),
            (UnchainedSummonerFeature::MakersCall, 6),
            (UnchainedSummonerFeature::Transposition, 8),
            (UnchainedSummonerFeature::Aspect, 10),
            (UnchainedSummonerFeature::GreaterShieldAlly, 12),
            (UnchainedSummonerFeature::LifeBond, 14),
            (UnchainedSummonerFeature::MergeForms, 16),
            (UnchainedSummonerFeature::GreaterAspect, 18),
            (UnchainedSummonerFeature::TwinEidolon, 20),
        ];
        assert_eq!(expected.len(), UnchainedSummonerFeature::ALL.len(), "17 features, 17 grant rows");
        for (feature, min) in expected {
            assert_eq!(feature.min_level(), *min, "{}", feature.key());
        }
    }

    #[test]
    fn aspect_has_no_upper_bound_unlike_the_apg_summoner() {
        // APG :756 adds PREVARLT:Summoner_CFP_Level,18; PU :261 does not.
        for level in 10..=MAX_SUPPORTED_LEVEL {
            assert!(UnchainedSummonerFeature::Aspect.is_granted_at(level), "Aspect must survive past 17th at {level}");
        }
    }

    #[test]
    fn every_row_cites_a_real_page() {
        for feature in UnchainedSummonerFeature::ALL {
            let page = feature.source_page();
            assert!(page.starts_with("p."), "{}: {page}", feature.key());
            assert_ne!(page, "p.xx", "decisions.md §27.2 placeholder must never be transcribed as a real page");
        }
        let pages: Vec<&str> = UnchainedSummonerFeature::ALL.iter().map(|f| f.source_page()).collect();
        assert_eq!(pages.iter().filter(|p| ***p == *"p.25").count(), 5);
        assert_eq!(pages.iter().filter(|p| ***p == *"p.26").count(), 2);
        assert_eq!(pages.iter().filter(|p| ***p == *"p.27").count(), 8);
        assert_eq!(pages.iter().filter(|p| ***p == *"p.28").count(), 2);
    }

    #[test]
    fn class_skill_list_is_the_verbatim_cskill_row() {
        let skills = class_skills();
        assert_eq!(skills.len(), 9, "CSKILL: on :727 has 9 pipe-separated entries");
        assert_eq!(skills[0], "TYPE=Craft");
        assert_eq!(skills[8], "Use Magic Device");
        assert!(skills.contains(&"TYPE=Knowledge"), "the summoner gets every Knowledge, not a named subset");
        assert!(skills.contains(&"Fly"));
        assert!(skills.contains(&"Handle Animal"));
    }

    // ---- formula pins -----------------------------------------------------

    #[test]
    fn eidolon_companion_level_tracks_class_level_one_to_one() {
        for level in 1..=MAX_SUPPORTED_LEVEL {
            assert_eq!(eidolon_companion_level(level), Some(level), "level {level}");
        }
    }

    #[test]
    fn eidolon_evolution_pool_matches_the_indicator_sum() {
        // Derived from the :746 token, not from memory: 1 + one point at each
        // of the 14 thresholds. Pauses on 4th, 8th, 12th, 16th, 20th.
        let expected = [1u8, 2, 3, 3, 4, 5, 6, 6, 7, 8, 9, 9, 10, 11, 12, 12, 13, 14, 15, 15];
        for (idx, want) in expected.iter().enumerate() {
            let level = idx as u8 + 1;
            assert_eq!(eidolon_evolution_pool(level), Some(*want), "level {level}");
        }
    }

    #[test]
    fn eidolon_evolution_thresholds_are_the_fourteen_from_the_token() {
        assert_eq!(EIDOLON_EVOLUTION_THRESHOLDS.len(), 14);
        // Every level that is NOT a threshold is a level the pool does not grow.
        for level in 2..=MAX_SUPPORTED_LEVEL {
            let grew = eidolon_evolution_pool(level) != eidolon_evolution_pool(level - 1);
            assert_eq!(grew, EIDOLON_EVOLUTION_THRESHOLDS.contains(&level), "level {level}");
        }
    }

    #[test]
    fn unchained_eidolon_pool_starts_at_one_where_the_apg_one_starts_at_three() {
        // apg_abilities_class.lst:813 writes `3+...`; this book's :746 writes `1+...`.
        assert_eq!(eidolon_evolution_pool(1), Some(1), "the whole point of the unchained eidolon");
        assert_ne!(eidolon_evolution_pool(1), Some(3), "must never resolve to the APG summoner's base");
    }

    #[test]
    fn eidolon_subtypes_are_the_thirteen_declared_rows() {
        let subtypes = eidolon_subtypes();
        assert_eq!(subtypes.len(), 13, ":747..759 is 13 rows");
        let mut sorted = subtypes.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 13, "no duplicates");
        assert_eq!(subtypes[0], "Agathion");
        assert_eq!(subtypes[12], "Psychopomp");
    }

    #[test]
    fn summon_monster_spell_level_climbs_every_other_level_and_caps_at_nine() {
        let expected = [1i16, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 9, 9];
        for (idx, want) in expected.iter().enumerate() {
            let level = idx as u8 + 1;
            assert_eq!(summon_monster_spell_level(level), Some(*want), "level {level}");
        }
        assert_eq!(summon_monster_spell_level(17), Some(9), "the DESC's stated maximum, reached at 17th");
    }

    #[test]
    fn summon_monster_uses_never_drop_below_three() {
        assert_eq!(summon_monster_uses_per_day(1, 4), Some(7));
        assert_eq!(summon_monster_uses_per_day(1, 0), Some(3));
        assert_eq!(
            summon_monster_uses_per_day(1, -3),
            Some(3),
            "max(CHA,0)+3 — the APG summoner's plain CHA+3 would give 0 here"
        );
        assert_eq!(summon_monster_uses_per_day(20, -3), Some(3));
    }

    #[test]
    fn the_unchained_marker_is_live_from_first_level() {
        assert_eq!(unchained_summoner_marker(1), Some(1));
        assert_eq!(unchained_summoner_marker(20), Some(1));
        assert_eq!(unchained_summoner_marker(0), None);
        assert_eq!(unchained_summoner_marker(21), None);
    }

    // ---- prose-derived ----------------------------------------------------

    #[test]
    fn bond_senses_rounds_equal_class_level_from_second() {
        assert_eq!(prose_derived::bond_senses_rounds_per_day(1), None, "granted at 2nd (:257)");
        assert_eq!(prose_derived::bond_senses_rounds_per_day(2), Some(2));
        assert_eq!(prose_derived::bond_senses_rounds_per_day(20), Some(20));
    }

    #[test]
    fn makers_call_gains_a_use_every_four_levels_after_sixth() {
        assert_eq!(prose_derived::makers_call_uses_per_day(5), None, "granted at 6th (:259)");
        assert_eq!(prose_derived::makers_call_uses_per_day(6), Some(1));
        assert_eq!(prose_derived::makers_call_uses_per_day(9), Some(1));
        assert_eq!(prose_derived::makers_call_uses_per_day(10), Some(2));
        assert_eq!(prose_derived::makers_call_uses_per_day(14), Some(3));
        assert_eq!(prose_derived::makers_call_uses_per_day(18), Some(4));
        assert_eq!(prose_derived::makers_call_uses_per_day(20), Some(4));
    }

    #[test]
    fn divertible_evolution_points_step_at_ten_and_eighteen() {
        assert_eq!(prose_derived::divertible_evolution_points(9), Some(0));
        assert_eq!(prose_derived::divertible_evolution_points(10), Some(2));
        assert_eq!(prose_derived::divertible_evolution_points(17), Some(2));
        assert_eq!(prose_derived::divertible_evolution_points(18), Some(6));
        assert_eq!(prose_derived::divertible_evolution_points(20), Some(6));
        assert_eq!(prose_derived::divertible_evolution_points(0), None);
        assert_eq!(prose_derived::divertible_evolution_points(21), None);
    }

    #[test]
    fn shield_ally_bonuses_are_the_stated_two_and_four() {
        assert_eq!(prose_derived::SHIELD_ALLY_BONUS, 2);
        assert_eq!(prose_derived::GREATER_SHIELD_ALLY_SELF_BONUS, 4);
    }

    // ---- refusal ----------------------------------------------------------

    #[test]
    fn no_formula_answers_outside_the_legal_level_band() {
        for level in [0u8, 21, 30, 255] {
            assert_eq!(eidolon_companion_level(level), None, "level {level}");
            assert_eq!(eidolon_evolution_pool(level), None, "level {level}");
            assert_eq!(summon_monster_spell_level(level), None, "level {level}");
            assert_eq!(summon_monster_uses_per_day(level, 3), None, "level {level}");
            assert_eq!(unchained_summoner_marker(level), None, "level {level}");
            assert_eq!(prose_derived::bond_senses_rounds_per_day(level), None, "level {level}");
            assert_eq!(prose_derived::makers_call_uses_per_day(level), None, "level {level}");
            assert_eq!(prose_derived::divertible_evolution_points(level), None, "level {level}");
        }
    }

    #[test]
    fn every_formula_is_defined_across_the_whole_granted_band() {
        for level in 1..=MAX_SUPPORTED_LEVEL {
            assert!(eidolon_evolution_pool(level).is_some(), "Eidolon is a 1st-level feature; level {level}");
            assert!(summon_monster_spell_level(level).is_some(), "level {level}");
            assert_eq!(
                prose_derived::bond_senses_rounds_per_day(level).is_some(),
                UnchainedSummonerFeature::BondSenses.is_granted_at(level)
            );
            assert_eq!(
                prose_derived::makers_call_uses_per_day(level).is_some(),
                UnchainedSummonerFeature::MakersCall.is_granted_at(level)
            );
        }
    }
}
