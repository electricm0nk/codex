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
/// its 17 features carry prose and no arithmetic token at all.
///
/// **Three of those 12 state no number even in prose, and get no function
/// here.** Naming them is the point, so a later cycle cannot mistake the gap
/// for an oversight:
///
/// * **Cantrips** (`:730`) — "a number of cantrips ... as noted on Table 1-5".
///   The number is on a table this book's row does not carry, and the
///   Unchained Summoner spell list is not transcribed at all, so there is
///   nothing to read.
/// * **Transposition** (`:737`) — spends a
///   [`makers_call_uses_per_day`] use to swap places instead of teleporting
///   the eidolon. It changes what a Maker's Call does, not how many the
///   summoner has, so it adds no magnitude of its own.
/// * **Life Bond** (`:740`) — "as long as the eidolon has 1 or more hit
///   points"; damage "transferred 1 point at a time". Both `1`s are the
///   mechanic's granularity, not a quantity a player tracks; there is no
///   number to display.
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

    /// [`GREATER_ASPECT_POINTS`] from Greater Aspect's 18th level, `None`
    /// below it.
    ///
    /// The same number [`divertible_evolution_points`] reports from 18th, but
    /// keyed to the Greater Aspect record rather than to the Aspect one, so
    /// the feature a player reads under that name has a magnitude of its own
    /// instead of silently raising a sibling row's.
    pub fn greater_aspect_divertible_evolution_points(level: u8) -> Option<u8> {
        if !UnchainedSummonerFeature::GreaterAspect.is_granted_at(level) {
            return None;
        }
        Some(GREATER_ASPECT_POINTS)
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

    /// The shield bonus to Armor Class — and the identical circumstance bonus
    /// on saving throws — **the summoner himself** has: `+2` from
    /// [`UnchainedSummonerFeature::ShieldAlly`]'s 4th level, rising to `+4`
    /// from [`UnchainedSummonerFeature::GreaterShieldAlly`]'s 12th.
    ///
    /// `:735` gives the summoner `+2` while within his eidolon's reach;
    /// `:739` says the same bonuses "increase to +4" when the ally in
    /// question is the summoner. So the two rows are one progression from the
    /// summoner's point of view, and this is it.
    ///
    /// Conditional on the eidolon being adjacent and not grappled, helpless,
    /// paralyzed, stunned or unconscious — none of which this engine tracks —
    /// so this is a standalone magnitude and is deliberately not folded into
    /// the character's resting Armor Class.
    pub fn shield_ally_self_bonus(level: u8) -> Option<i16> {
        if !UnchainedSummonerFeature::ShieldAlly.is_granted_at(level) {
            return None;
        }
        if UnchainedSummonerFeature::GreaterShieldAlly.is_granted_at(level) {
            Some(GREATER_SHIELD_ALLY_SELF_BONUS)
        } else {
            Some(SHIELD_ALLY_BONUS)
        }
    }

    /// The shield bonus to Armor Class — and the identical circumstance bonus
    /// on saving throws — Greater Shield Ally extends to **allies other than
    /// the summoner**: `+2`, from 12th.
    ///
    /// This is the half of `:739` that is genuinely new at 12th level: before
    /// it, Shield Ally protects the summoner alone. `None` below 12.
    pub fn greater_shield_ally_bonus_to_allies(level: u8) -> Option<i16> {
        if !UnchainedSummonerFeature::GreaterShieldAlly.is_granted_at(level) {
            return None;
        }
        Some(SHIELD_ALLY_BONUS)
    }

    /// Merge Forms rounds per day, from `:741`: "The summoner can use this
    /// ability for a number of rounds per day equal to his summoner level."
    ///
    /// Same shape as [`bond_senses_rounds_per_day`] and, like it, carries no
    /// `BONUS:VAR|` — PCGen does not compute this number either. `None` below
    /// 16th.
    pub fn merge_forms_rounds_per_day(level: u8) -> Option<u8> {
        active_level(UnchainedSummonerFeature::MergeForms, level)
    }

    /// Twin Eidolon **minutes** per day, from `:743`: "The summoner can keep
    /// this form for a number of minutes per day equal to his summoner
    /// level."
    ///
    /// Minutes, not rounds — the unit is the row's own and is not silently
    /// converted, because the two sibling durations in this module
    /// ([`bond_senses_rounds_per_day`], [`merge_forms_rounds_per_day`]) are
    /// rounds and a unit slip between them would be invisible. The same
    /// sentence adds that the duration "must be spent in 1-minute
    /// increments", which is why converting would also be wrong in substance.
    /// `None` below 20th.
    pub fn twin_eidolon_minutes_per_day(level: u8) -> Option<u8> {
        active_level(UnchainedSummonerFeature::TwinEidolon, level)
    }

    /// The distance an eidolon may stray from its summoner and stay at full
    /// strength: `100` feet, from `:732`.
    ///
    /// Verbatim: "the eidolon and the summoner must remain within 100 feet of
    /// one another for the eidolon to remain at full strength."
    ///
    /// Flat — the row states no level scaling — so this takes a level only to
    /// gate on Life Link's 1st-level grant. The three degradation bands the
    /// same sentence goes on to state are
    /// [`LIFE_LINK_HALF_STRENGTH_RANGE_FEET`],
    /// [`LIFE_LINK_QUARTER_STRENGTH_RANGE_FEET`] and
    /// [`LIFE_LINK_BANISHMENT_RANGE_FEET`]; they are constants rather than a
    /// second function because a range band is not a per-level magnitude.
    pub fn life_link_full_strength_range_feet(level: u8) -> Option<i16> {
        active_level(UnchainedSummonerFeature::LifeLink, level).map(|_| 100)
    }

    /// Beyond [`life_link_full_strength_range_feet`] but closer than this,
    /// the eidolon's current and maximum hit points are halved (`:732`:
    /// "reduced by 50%").
    pub const LIFE_LINK_HALF_STRENGTH_RANGE_FEET: i32 = 1_000;

    /// Beyond [`LIFE_LINK_HALF_STRENGTH_RANGE_FEET`] but closer than this,
    /// the reduction is 75% (`:732`).
    pub const LIFE_LINK_QUARTER_STRENGTH_RANGE_FEET: i32 = 10_000;

    /// Past [`LIFE_LINK_QUARTER_STRENGTH_RANGE_FEET`] the eidolon "is
    /// immediately returned to its home plane" (`:732`). Same number,
    /// named for the third band so the receipt can state all three without
    /// re-using a constant under a misleading name.
    pub const LIFE_LINK_BANISHMENT_RANGE_FEET: i32 = LIFE_LINK_QUARTER_STRENGTH_RANGE_FEET;
}

#[cfg(test)]
mod tests {
    use super::prose_derived;
    use super::*;

    /// The ingested `description` for one Unchained Summoner record, read off
    /// disk. Every prose-derived reading in this module is checked against
    /// this rather than against a copy of the sentence kept here — a pin that
    /// quotes itself pins nothing.
    fn description_of(key: &str) -> String {
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("data/corpus/pathfinder_unchained/class_feature/summoner_unchained_class");
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("ingested Summoner corpus dir {dir:?} must exist: {e}"));
        for entry in entries {
            let path = entry.expect("readable dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable corpus record");
            let value: serde_json::Value =
                serde_json::from_str(&text).expect("corpus record is valid JSON");
            if value["data"]["key"] == key {
                return value["data"]["description"]
                    .as_str()
                    .unwrap_or_else(|| panic!("{key} must carry a rendered description"))
                    .to_owned();
            }
        }
        panic!("no ingested record with KEY:{key}");
    }

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

        // Keyed to the Greater Aspect record rather than the Aspect one.
        for level in 0..18 {
            assert_eq!(
                prose_derived::greater_aspect_divertible_evolution_points(level),
                None,
                "level {level}"
            );
        }
        for level in 18..=MAX_SUPPORTED_LEVEL {
            assert_eq!(
                prose_derived::greater_aspect_divertible_evolution_points(level),
                Some(6),
                "level {level}"
            );
        }
        assert_eq!(prose_derived::greater_aspect_divertible_evolution_points(21), None);
    }

    #[test]
    fn shield_ally_bonuses_are_the_stated_two_and_four() {
        assert_eq!(prose_derived::SHIELD_ALLY_BONUS, 2);
        assert_eq!(prose_derived::GREATER_SHIELD_ALLY_SELF_BONUS, 4);
    }

    /// From the summoner's own point of view `Shield Ally` (`:735`) and
    /// `Greater Shield Ally` (`:739`) are one progression: `+2` at 4th, `+4`
    /// at 12th. Before this, both rows computed nothing.
    #[test]
    fn shield_ally_self_bonus_steps_from_two_to_four_at_twelfth() {
        for level in [0u8, 1, 2, 3] {
            assert_eq!(prose_derived::shield_ally_self_bonus(level), None, "level {level}");
        }
        for level in 4..12 {
            assert_eq!(prose_derived::shield_ally_self_bonus(level), Some(2), "level {level}");
        }
        for level in 12..=MAX_SUPPORTED_LEVEL {
            assert_eq!(prose_derived::shield_ally_self_bonus(level), Some(4), "level {level}");
        }
        assert_eq!(prose_derived::shield_ally_self_bonus(21), None);

        // The genuinely new half of :739 — allies other than the summoner.
        for level in 0..12 {
            assert_eq!(prose_derived::greater_shield_ally_bonus_to_allies(level), None, "level {level}");
        }
        for level in 12..=MAX_SUPPORTED_LEVEL {
            assert_eq!(prose_derived::greater_shield_ally_bonus_to_allies(level), Some(2), "level {level}");
        }
    }

    /// `Merge Forms` (`:741`) is rounds per day; `Twin Eidolon` (`:743`) is
    /// **minutes** per day. Both equal the summoner's level, and the differing
    /// unit is the whole reason they are two functions.
    #[test]
    fn merge_forms_is_rounds_and_twin_eidolon_is_minutes_both_equal_to_level() {
        for level in 0..16 {
            assert_eq!(prose_derived::merge_forms_rounds_per_day(level), None, "level {level}");
        }
        for level in 16..=MAX_SUPPORTED_LEVEL {
            assert_eq!(prose_derived::merge_forms_rounds_per_day(level), Some(level), "level {level}");
        }
        for level in 0..20 {
            assert_eq!(prose_derived::twin_eidolon_minutes_per_day(level), None, "level {level}");
        }
        assert_eq!(prose_derived::twin_eidolon_minutes_per_day(20), Some(20));
        assert_eq!(prose_derived::twin_eidolon_minutes_per_day(21), None);
    }

    /// Life Link's leash: 100 feet at full strength, then two degradation
    /// bands and banishment.
    #[test]
    fn life_link_leash_is_the_hundred_feet_its_prose_states() {
        assert_eq!(prose_derived::life_link_full_strength_range_feet(0), None);
        for level in 1..=MAX_SUPPORTED_LEVEL {
            assert_eq!(prose_derived::life_link_full_strength_range_feet(level), Some(100), "level {level}");
        }
        assert_eq!(prose_derived::life_link_full_strength_range_feet(21), None);
        assert_eq!(prose_derived::LIFE_LINK_HALF_STRENGTH_RANGE_FEET, 1_000);
        assert_eq!(prose_derived::LIFE_LINK_QUARTER_STRENGTH_RANGE_FEET, 10_000);
        assert_eq!(prose_derived::LIFE_LINK_BANISHMENT_RANGE_FEET, 10_000);
    }

    /// Every prose-derived reading above is a sentence, so every sentence is
    /// re-read off the ingested corpus record. A corpus edit that changes the
    /// wording fails here instead of silently invalidating a number.
    #[test]
    fn prose_derived_readings_still_match_the_ingested_corpus_prose() {
        for (key, sentence) in [
            (
                "Unchained Summoner ~ Shield Ally",
                "the summoner gains a +2 shield bonus to his Armor Class and a +2 circumstance \
                 bonus on his saving throws",
            ),
            (
                "Unchained Summoner ~ Greater Shield Ally",
                "the ally gains a +2 shield bonus to its Armor Class and a +2 circumstance bonus \
                 on its saving throws. If this ally is the summoner, these bonuses increase to +4",
            ),
            (
                "Unchained Summoner ~ Merge Forms",
                "The summoner can use this ability for a number of rounds per day equal to his \
                 summoner level",
            ),
            (
                "Unchained Summoner ~ Twin Eidolon",
                "The summoner can keep this form for a number of minutes per day equal to his \
                 summoner level",
            ),
            (
                "Unchained Summoner ~ Life Link",
                "the eidolon and the summoner must remain within 100 feet of one another for the \
                 eidolon to remain at full strength",
            ),
            (
                "Unchained Summoner ~ Bond Senses",
                "He can use this ability a number of rounds per day equal to his summoner level",
            ),
            (
                "Unchained Summoner ~ Maker's Call",
                "The summoner can use this ability once per day at 6th level, plus one additional \
                 time per day for every four levels beyond 6th",
            ),
            (
                "Unchained Summoner ~ Aspect",
                "a summoner can divert up to 2 points from his eidolon's evolution pool",
            ),
            (
                "Unchained Summoner ~ Greater Aspect",
                "the maximum number of evolution points the summoner can divert increases to 6",
            ),
        ] {
            let description = description_of(key);
            assert!(
                description.contains(sentence),
                "{key} prose changed; the reading derived from it must be re-derived.\n\
                 expected to contain: {sentence}\ncorpus says: {description}"
            );
        }
    }

    /// The three Unchained Summoner features that state no number even in
    /// prose. Pinning the *reason* keeps a later cycle from inventing one —
    /// and makes a corpus that grows a number fail loudly.
    #[test]
    fn cantrips_transposition_and_life_bond_state_no_displayable_number() {
        assert!(
            description_of("Unchained Summoner ~ Cantrips").contains("as noted on Table 1-5"),
            "Cantrips' count still lives on a table this row does not carry"
        );
        assert!(
            description_of("Unchained Summoner ~ Transposition")
                .contains("a summoner can use his maker's call ability to swap locations"),
            "Transposition still spends a Maker's Call use rather than adding a magnitude"
        );
        let life_bond = description_of("Unchained Summoner ~ Life Bond");
        assert!(
            life_bond.contains("As long as the eidolon has 1 or more hit points")
                && life_bond.contains("transferred 1 point at a time"),
            "Life Bond's only numbers are still the mechanic's granularity, not a quantity"
        );
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
