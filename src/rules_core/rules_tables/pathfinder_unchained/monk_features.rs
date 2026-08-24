//! Pathfinder Unchained — **Unchained Monk** class features and chassis,
//! one hand-modelled pure function per magnitude.
//!
//! # Why hand-modelled and not interpreted
//!
//! `decisions.md §24.1` forbids a general `BONUS:`/`DEFINE:`/`PREREQ:`
//! formula interpreter and mandates the shape the 27 already-shipped
//! classes use: a small pure function whose formula is verified byte-exact
//! against the corpus row, plus a test. Every function below names the exact
//! source token it transcribes and the file and line it came from. Nothing
//! here parses a formula at runtime.
//!
//! # This is a REPLACEMENT for the Core Rulebook Monk, and both coexist
//!
//! Pathfinder Unchained declares **zero `CLASS:` objects**. `Monk ~
//! Unchained Class` is a `CATEGORY:CLASS` selection ability
//! (`pu_abilities_class.lst:115`) layered over Core Rulebook's real
//! `CLASS:Monk` (`cr_classes.lst:147`, `MAXLEVEL:20`). A campaign chooses
//! one or the other; a character never has both.
//!
//! **How the two are kept distinct in this repo:**
//!
//! 1. The CRB Monk's tables and functions are **not touched**.
//!    `rules_tables::crb::class_tables`'s `ClassId::Monk` row (d8,
//!    three-quarter BAB, three good saves) and `pilot_compute.rs`'s
//!    `monk_*` family stay exactly as they are. This module adds files;
//!    it edits none.
//! 2. Everything here lives under a separate module path
//!    (`rules_tables::pathfinder_unchained::monk_features`) with its own
//!    `UnchainedMonkFeature` type and its own `Unchained Monk ~ …` keys, so
//!    no name in either namespace can shadow the other.
//! 3. The chassis genuinely differs, and
//!    `unchained_monk_chassis_differs_from_the_crb_monk` asserts the
//!    difference against the shipped CRB table, so a future edit that
//!    collapses one into the other fails loudly:
//!
//! | | CRB Monk | Unchained Monk |
//! |---|---|---|
//! | hit die | `d8` (a documented operator override of the corpus's `HD:10`) | `d10` ([`HIT_DIE`], `TEMPLATE:Monk ~ Unchained HD` → `HITDIE:10\|CLASS=Monk`) |
//! | base attack | three-quarter | **full** ([`base_attack_bonus`], `TYPE=Base.REPLACE`) |
//! | good saves | Fort, Ref, Will | Fort, Ref — **Will is poor** ([`will_save`]) |
//! | flurry | 4 attacks by level 15 | up to **6** attacks ([`flurry_attack_count`]) |
//!
//! The chassis tokens, all on `pu_abilities_class.lst:115`:
//! - `BONUS:COMBAT|BASEAB|classlevel("Monk","APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE|PREVAREQ:UseAlternateBABProgression,0`
//! - `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("Monk","APPLIEDAS=NONEPIC")/2+2|PREVAREQ:UseAlternateSaveProgression,0`
//! - `BONUS:SAVE|BASE.Will|classlevel("Monk","APPLIEDAS=NONEPIC")/3|PREVAREQ:UseAlternateSaveProgression,0`
//!
//! The only edit made transcribing those three is substituting the literal
//! `level` for the `classlevel("Monk", …)` call; the arithmetic tail is
//! byte-identical.
//!
//! # Two magnitudes come from shared Core Rulebook internal records
//!
//! PU's AC Bonus row grants `ABILITY:Internal|AUTOMATIC|Monk AC Tracker`
//! and its Ki Pool row grants `ABILITY:Internal|AUTOMATIC|Ki Pool Tracker`
//! — both defined in `core_rulebook/cr_abilities_class.lst` (lines 1090 and
//! 1175/1179). PU sets the level variables those trackers read
//! (`MonkACLVL`, `KiPoolLVL`) and modifies nothing else, so the Unchained
//! Monk's AC bonus and ki pool are genuinely the CRB formulas driven by an
//! Unchained Monk level. That cross-file read is stated here rather than
//! hidden, and [`armor_class_bonus_from_level`] / [`ki_points`] cite both
//! halves.
//!
//! # What is deliberately NOT modelled here
//!
//! - **Unarmed strike damage.** `Unchained Monk ~ Unarmed Strike`
//!   (`:464`) grants the shared `ABILITY:Internal|AUTOMATIC|Monk ~ Unarmed
//!   Damage` record — the same one CRB's Monk grants — and adds no token of
//!   its own. Pathfinder Unchained writes no `MonkUnarmedDamage` or `UDAM`
//!   token anywhere, so it overrides nothing about that record and the
//!   progression is literally the Core Rulebook's.
//!   `pilot_compute.rs`'s `monk_unarmed_strike_damage_die` already states it,
//!   and restating it here would create a second, competing source of truth
//!   for one fact, which this repo has been burned by before.
//!
//!   **Corrected 2026-08-01: "not modelled here" was being read as "reaches
//!   the sheet".** It did not. Those rows
//!   (`class_chassis.monk.unarmed_strike_damage_die` and its `_die_count`
//!   sibling) are pushed only by `explain_monk_level1_chassis`, which returns
//!   early unless the character holds Core Rulebook `class:monk`, so an
//!   Unchained Monk got the roster row naming Unarmed Strike and no number at
//!   any level from 1 to 20. `pilot_compute.rs`'s
//!   `ground_unchained_monk_unarmed_strike_damage` now calls the existing
//!   function from the Unchained path — still one ladder, now reached from
//!   both classes. **This module is still the wrong home for it**, which is
//!   why the fix landed there and not here: the fact is a Core Rulebook
//!   record's, not Pathfinder Unchained's.
//! - **Stunning Fist's save DC and uses per day.** PU's row (`:463`)
//!   carries exactly one token, `BONUS:VAR|StunningFistMonkLVL|MonkLVL`
//!   ([`stunning_fist_monk_level`]). The DC (`10+(TL/2)+WIS`) and uses
//!   (`MonkLVL+floor((TL-MonkLVL)/4)`) live on the shared CRB *feat*
//!   (`cr_feats.lst:171`) and are already grounded in
//!   `rules_core::feat_effects`. Feat-lane content stays in the feat lane —
//!   the same boundary `warpriest_fervor_uses_per_day` draws for Extra
//!   Channel.
//! - **The 31 Ki Powers, 10 Style Strikes and the Unchained Monk bonus-feat
//!   list.** These are the *options* the pools below are spent on. The
//!   ingestion cycle deliberately did not write them (they need their own
//!   content-kind directory). [`ki_powers_known`] and
//!   [`style_strikes_known`] return real pool sizes; they do not imply a
//!   catalogue exists.
//! - **Archetype suppression.** Lines 479-487 carry nine
//!   `BONUS:VAR|Pool_Unchained_Ki_Power|-1` `.MOD` rows, each gated on a
//!   `Monk_CF_KiPowersN` archetype flag, and every progression row is gated
//!   on a `PREVAREQ:Monk_CF_<Feature>,0`. No archetype engine exists in this
//!   repo, so these functions model the unsuppressed progression and say so
//!   rather than pretending to apply flags nothing sets.

//! # Why the CRB comparison is pinned against the corpus, not against
//! `crb::class_tables`
//!
//! This module is reached two ways: through the library crate, and through
//! `src/bin/gen_book_cache.rs`, which pulls
//! `rules_tables/pathfinder_unchained/mod.rs` in with `#[path]` (a
//! workaround from a cycle that could not touch `rules_tables/mod.rs`;
//! that binary is outside this cycle's write scope, so the include stays).
//! Under `#[path]` there is no `crate::rules_core::…` to import, so a
//! direct `use` of `crb::class_tables` would break that binary's build.
//! `unchained_monk_chassis_differs_from_the_crb_monk` therefore reads
//! `data/corpus/core_rulebook/class/monk.json` — PCGen's own statement of
//! the CRB Monk chassis — which works in both contexts and pins the rules
//! fact rather than one in-repo transcription of it.

/// One ingested `class_feature` record for the Unchained Monk.
///
/// The roster is exactly the 18 records under
/// `data/corpus/pathfinder_unchained/class_feature/monk_unchained_class/`.
/// Unlike the Barbarian's, every one of them is granted by the single
/// `Monk ~ Unchained Class.MOD` progression block
/// (`pu_abilities_class.lst:154-171`), so there is no Full/Ex-Class split
/// and no ungranted record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnchainedMonkFeature {
    /// PCGen `KEY:` token.
    pub key: &'static str,
    /// PCGen display name (the row's first column).
    pub name: &'static str,
    /// Grant level from `PREVARGTEQ:Monk_CFP_Level,N`.
    pub min_level: u8,
    /// 1-based line in `pu_abilities_class.lst`.
    pub corpus_line: u32,
}

/// `MAXLEVEL:20` on the base `CLASS:Monk` record
/// (`core_rulebook/cr_classes.lst:147`). PU adds no levels.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

/// `d10` — `pu_templates.lst:5`, `Monk ~ Unchained HD  HITDIE:10|CLASS=Monk`,
/// applied by `pu_abilities_class.lst:115`'s
/// `TEMPLATE:Monk ~ Unchained HD`.
///
/// This is the one place the Unchained Monk's hit die and the CRB Monk's
/// diverge *in this repo for a reason unrelated to the books*: CRB's row is
/// a documented operator override of its own corpus `HD:10` (see
/// `crb::class_tables`' `hit_die` doc comment, risks item 91). PU's `d10` is
/// a plain transcription of a different token in a different file and is
/// **not** subject to that override, which is why it is stated here rather
/// than reused.
pub const HIT_DIE: u8 = 10;

/// `pu_abilities_class.lst:154` — `PREVARGTEQ:Monk_CFP_Level,1`.
pub const WEAPON_AND_ARMOR_PROFICIENCY_LEVEL: u8 = 1;
/// `pu_abilities_class.lst:155` — `PREVARGTEQ:Monk_CFP_Level,1`.
pub const AC_BONUS_LEVEL: u8 = 1;
/// `pu_abilities_class.lst:156` — `PREVARGTEQ:Monk_CFP_Level,1`.
pub const BONUS_FEAT_LEVEL: u8 = 1;
/// `pu_abilities_class.lst:157` — `PREVARGTEQ:Monk_CFP_Level,1`.
pub const FLURRY_OF_BLOWS_LEVEL: u8 = 1;
/// `pu_abilities_class.lst:158` — `PREVARGTEQ:Monk_CFP_Level,1`.
pub const STUNNING_FIST_LEVEL: u8 = 1;
/// `pu_abilities_class.lst:159` — `PREVARGTEQ:Monk_CFP_Level,1`.
pub const UNARMED_STRIKE_LEVEL: u8 = 1;
/// `pu_abilities_class.lst:160` — `PREVARGTEQ:Monk_CFP_Level,2`.
pub const EVASION_LEVEL: u8 = 2;
/// `pu_abilities_class.lst:161` — `PREVARGTEQ:Monk_CFP_Level,3`.
pub const FAST_MOVEMENT_LEVEL: u8 = 3;
/// `pu_abilities_class.lst:162` — `PREVARGTEQ:Monk_CFP_Level,3`.
pub const KI_POOL_LEVEL: u8 = 3;
/// `pu_abilities_class.lst:163` — `PREVARGTEQ:Monk_CFP_Level,4`.
pub const KI_POWERS_LEVEL: u8 = 4;
/// `pu_abilities_class.lst:164` — `PREVARGTEQ:Monk_CFP_Level,4`.
pub const STILL_MIND_LEVEL: u8 = 4;
/// `pu_abilities_class.lst:165` — `PREVARGTEQ:Monk_CFP_Level,5`.
pub const PURITY_OF_BODY_LEVEL: u8 = 5;
/// `pu_abilities_class.lst:166` — `PREVARGTEQ:Monk_CFP_Level,5`.
pub const STYLE_STRIKE_LEVEL: u8 = 5;
/// `pu_abilities_class.lst:167` — `PREVARGTEQ:Monk_CFP_Level,9`.
pub const IMPROVED_EVASION_LEVEL: u8 = 9;
/// `pu_abilities_class.lst:168` — `PREVARGTEQ:Monk_CFP_Level,13`.
pub const TONGUE_OF_THE_SUN_AND_MOON_LEVEL: u8 = 13;
/// `pu_abilities_class.lst:169` — `PREVARGTEQ:Monk_CFP_Level,17`.
pub const TIMELESS_BODY_LEVEL: u8 = 17;
/// `pu_abilities_class.lst:170` — `PREVARGTEQ:Monk_CFP_Level,19`.
pub const FLAWLESS_MIND_LEVEL: u8 = 19;
/// `pu_abilities_class.lst:171` — `PREVARGTEQ:Monk_CFP_Level,20`.
pub const PERFECT_SELF_LEVEL: u8 = 20;

/// The cap on the level component of the monk's AC bonus:
/// `min((MonkACLVL)/4,5)` (`core_rulebook/cr_abilities_class.lst:1090`).
pub const AC_BONUS_LEVEL_COMPONENT_CAP: i16 = 5;

/// Still Mind's bonus on saving throws against enchantment spells and
/// effects.
///
/// **Read off the row's own `DESC:` prose**, not a `BONUS:` token
/// (`pu_abilities_class.lst:469`: "At 4th level, a monk gains a +2 bonus on
/// saving throws against enchantment spells and effects"). PCGen carries no
/// mechanical token for it because the condition — "against enchantment" —
/// is not expressible as an unconditional save bonus, so the feature is
/// descriptive-only in PCGen too. Labelled explicitly so nobody mistakes a
/// prose-derived constant for a transcribed token.
pub const STILL_MIND_SAVE_BONUS: i16 = 2;

/// Perfect Self's damage reduction, `10/chaotic`
/// (`pu_abilities_class.lst:476` `DESC:`). Prose-derived for the same
/// reason as [`STILL_MIND_SAVE_BONUS`]: the `/chaotic` bypass condition has
/// no PCGen token on this row. The bypass type is deliberately not encoded
/// as data here — this repo has no damage-type bypass engine, and inventing
/// one field for one feature would be worse than naming the gap.
pub const PERFECT_SELF_DAMAGE_REDUCTION: i16 = 10;

/// The 18 ingested Unchained Monk `class_feature` records, in
/// `pu_abilities_class.lst` line order.
const FEATURES: &[UnchainedMonkFeature] = &[
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Weapon and Armor Proficiency",
        name: "Weapon and Armor Proficiency",
        min_level: WEAPON_AND_ARMOR_PROFICIENCY_LEVEL,
        corpus_line: 459,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ AC Bonus",
        name: "AC Bonus",
        min_level: AC_BONUS_LEVEL,
        corpus_line: 460,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Bonus Feat",
        name: "Bonus Feat",
        min_level: BONUS_FEAT_LEVEL,
        corpus_line: 461,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Flurry of Blows",
        name: "Flurry of Blows",
        min_level: FLURRY_OF_BLOWS_LEVEL,
        corpus_line: 462,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Stunning Fist",
        name: "Stunning Fist",
        min_level: STUNNING_FIST_LEVEL,
        corpus_line: 463,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Unarmed Strike",
        name: "Unarmed Strike",
        min_level: UNARMED_STRIKE_LEVEL,
        corpus_line: 464,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Evasion",
        name: "Evasion",
        min_level: EVASION_LEVEL,
        corpus_line: 465,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Fast Movement",
        name: "Fast Movement",
        min_level: FAST_MOVEMENT_LEVEL,
        corpus_line: 466,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Ki Pool",
        name: "Ki Pool",
        min_level: KI_POOL_LEVEL,
        corpus_line: 467,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Ki Powers",
        name: "Ki Powers",
        min_level: KI_POWERS_LEVEL,
        corpus_line: 468,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Still Mind",
        name: "Still Mind",
        min_level: STILL_MIND_LEVEL,
        corpus_line: 469,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Purity of Body",
        name: "Purity of Body",
        min_level: PURITY_OF_BODY_LEVEL,
        corpus_line: 470,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Style Strike",
        name: "Style Strike",
        min_level: STYLE_STRIKE_LEVEL,
        corpus_line: 471,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Improved Evasion",
        name: "Improved Evasion",
        min_level: IMPROVED_EVASION_LEVEL,
        corpus_line: 472,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Tongue of the Sun and Moon",
        name: "Tongue of the Sun and Moon",
        min_level: TONGUE_OF_THE_SUN_AND_MOON_LEVEL,
        corpus_line: 473,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Timeless Body",
        name: "Timeless Body",
        min_level: TIMELESS_BODY_LEVEL,
        corpus_line: 474,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Flawless Mind",
        name: "Flawless Mind",
        min_level: FLAWLESS_MIND_LEVEL,
        corpus_line: 475,
    },
    UnchainedMonkFeature {
        key: "Unchained Monk ~ Perfect Self",
        name: "Perfect Self",
        min_level: PERFECT_SELF_LEVEL,
        corpus_line: 476,
    },
];

/// The full ingested feature roster.
pub fn features() -> &'static [UnchainedMonkFeature] {
    FEATURES
}

/// Looks a feature up by its PCGen `KEY:`.
pub fn feature(key: &str) -> Option<&'static UnchainedMonkFeature> {
    FEATURES.iter().find(|f| f.key == key)
}

/// Full base attack bonus — `level`, replacing the base Monk column
/// entirely (`TYPE=Base.REPLACE`).
///
/// `pu_abilities_class.lst:115` —
/// `BONUS:COMBAT|BASEAB|classlevel("Monk","APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE`.
pub fn base_attack_bonus(level: u8) -> i16 {
    i16::from(level)
}

/// Good Fortitude save — `level/2+2`.
///
/// `pu_abilities_class.lst:115` —
/// `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("Monk","APPLIEDAS=NONEPIC")/2+2`.
/// Fortitude and Reflex share that one token, which is why they share a
/// formula here.
pub fn fort_save(level: u8) -> i16 {
    i16::from(level) / 2 + 2
}

/// Good Reflex save — `level/2+2`, from the same token as [`fort_save`].
pub fn ref_save(level: u8) -> i16 {
    i16::from(level) / 2 + 2
}

/// **Poor** Will save — `level/3`. This is the Unchained Monk's signature
/// chassis concession and the sharpest divergence from the CRB Monk, which
/// has a good Will save.
///
/// `pu_abilities_class.lst:115` —
/// `BONUS:SAVE|BASE.Will|classlevel("Monk","APPLIEDAS=NONEPIC")/3`.
pub fn will_save(level: u8) -> i16 {
    i16::from(level) / 3
}

/// The level component of the monk's AC and CMD bonus:
/// `min(level / 4, 5)`.
///
/// Two tokens:
/// - `pu_abilities_class.lst:460` — `BONUS:VAR|MonkACLVL|MonkLVL|TYPE=Level`
/// - `core_rulebook/cr_abilities_class.lst:1090` (`Monk AC Tracker`, the
///   shared internal record PU's row grants) —
///   `BONUS:VAR|MonkACBonus|min((MonkACLVL)/4,5)|TYPE=level`
///
/// The same row's `DESC:` states the identical rule in prose ("+1 bonus to
/// AC and CMD at 4th level … up to a maximum of +5 at 20th level"), and
/// `ac_bonus_level_component_agrees_with_the_rows_own_prose` pins the two
/// statements together.
pub fn armor_class_bonus_from_level(level: u8) -> i16 {
    (i16::from(level) / 4).min(AC_BONUS_LEVEL_COMPONENT_CAP)
}

/// The monk's full AC and CMD bonus when unarmored and unencumbered:
/// [`armor_class_bonus_from_level`] plus `max(Wisdom modifier, 0)`.
///
/// `core_rulebook/cr_abilities_class.lst:1090` —
/// `BONUS:VAR|MonkACStatBonus|max(WIS,0)|TYPE=Class|PREVARGTEQ:IsMonk,1`,
/// with `pu_abilities_class.lst:115`'s `BONUS:VAR|IsMonk|1|TYPE=Boolean`
/// satisfying that gate for the Unchained Monk.
///
/// The `max(…,0)` is load-bearing: a negative Wisdom modifier does **not**
/// reduce the monk's AC. The armor/shield/encumbrance condition on the whole
/// bonus (`PREVAREQ:MonkRestricted,0`) is a caller concern and is not
/// silently applied here.
pub fn armor_class_bonus(level: u8, wisdom_modifier: i16) -> i16 {
    armor_class_bonus_from_level(level) + wisdom_modifier.max(0)
}

/// Bonus feats available: `1 + max((level + 2) / 4, 0)`.
///
/// `pu_abilities_class.lst:461` —
/// `BONUS:ABILITYPOOL|Unchained Monk Bonus Feat|1+max((MonkBonusFeatLVL+2)/4,0)`
/// with the same row's `BONUS:VAR|MonkBonusFeatLVL|MonkLVL`.
///
/// Reproduces the published "1st level, 2nd level, and every 4 levels
/// thereafter" ladder: 1 at 1st, 2 at 2nd, then 3/4/5/6 at 6th/10th/14th/
/// 18th. `None` below [`BONUS_FEAT_LEVEL`].
pub fn bonus_feats_known(level: u8) -> Option<i16> {
    if level < BONUS_FEAT_LEVEL {
        return None;
    }
    Some(1 + ((i16::from(level) + 2) / 4).max(0))
}

/// How many attacks a flurry of blows yields, as a function of **total base
/// attack bonus** (not level):
/// `2 + (bab>=6) + 2*(bab>=11) + (bab>=16)`.
///
/// `pu_abilities_class.lst:495` —
/// `BONUS:VAR|FlurryAttacks|2+(Total_BAB>=6)+if(Total_BAB>=11,2,0)+(Total_BAB>=16)`
/// with `:494`'s `BONUS:VAR|Total_BAB|BAB`. PCGen's comparison operators
/// yield 1/0, which is what makes the sum a count.
///
/// **The argument is BAB, not level, and that is the corpus's choice.** For
/// a single-classed Unchained Monk the two coincide, because
/// [`base_attack_bonus`] is full — but for a multiclass character they do
/// not, and substituting level would quietly change what is measured. Use
/// [`flurry_attack_count_at_monk_level`] for the single-classed case.
///
/// The corpus also carries a second variable, `:492`'s
/// `BONUS:VAR|FlurryExtraAttacks|2+(Total_BAB>=11)+(Total_BAB>=6)+(Total_BAB>=11)+(Total_BAB>=16)`,
/// which is written differently (the `>=11` term appears twice instead of
/// being doubled) but is numerically identical at every BAB.
/// `flurry_extra_attacks_is_numerically_identical_to_flurry_attacks` checks
/// that rather than taking it on faith.
pub fn flurry_attack_count(total_base_attack_bonus: i16) -> i16 {
    let mut count = 2;
    if total_base_attack_bonus >= 6 {
        count += 1;
    }
    if total_base_attack_bonus >= 11 {
        count += 2;
    }
    if total_base_attack_bonus >= 16 {
        count += 1;
    }
    count
}

/// [`flurry_attack_count`] for a single-classed Unchained Monk, whose BAB
/// equals his level because [`base_attack_bonus`] is full and
/// `TYPE=Base.REPLACE`.
pub fn flurry_attack_count_at_monk_level(level: u8) -> i16 {
    flurry_attack_count(base_attack_bonus(level))
}

/// The attack penalty on the `index`-th flurry attack (1-based), relative to
/// the monk's highest attack bonus. `None` when that attack does not exist
/// at the given `attack_count`.
///
/// Transcribed from the `FAB_n` ladder, `pu_abilities_class.lst:497-502`:
///
/// | token | line | penalty |
/// |---|---|---|
/// | `BONUS:VAR\|FAB_1\|FAB` | 497 | `0` |
/// | `BONUS:VAR\|FAB_2\|FAB` | 498 | `0` |
/// | `BONUS:VAR\|FAB_3\|FAB+if(FlurryAttacks==3,-5,0)` | 499 | `-5` only when the flurry has exactly 3 attacks |
/// | `BONUS:VAR\|FAB_4\|FAB-5` | 500 | `-5` |
/// | `BONUS:VAR\|FAB_5\|FAB-10` | 501 | `-10` |
/// | `BONUS:VAR\|FAB_6\|FAB-15` | 502 | `-15` |
///
/// **`BONUS:VAR|FAB_7|FAB` (line 503) is deliberately not modelled.**
/// [`flurry_attack_count`] tops out at 6, so slot 7 is unreachable;
/// transcribing its zero offset would hand a monk a seventh attack at full
/// attack bonus — a plausible-looking number with no rule behind it. Index 7
/// returns `None` and this note is the record of why.
///
/// Reproduces the published Unchained Monk flurry line exactly: `+6/+6/+1`
/// at BAB 6, `+11/+11/+11/+6/+1` at BAB 11, `+16/+16/+16/+11/+6/+1` at
/// BAB 16.
pub fn flurry_iterative_attack_penalty(index: u8, attack_count: i16) -> Option<i16> {
    if index == 0 || i16::from(index) > attack_count {
        return None;
    }
    match index {
        1 | 2 => Some(0),
        3 => Some(if attack_count == 3 { -5 } else { 0 }),
        4 => Some(-5),
        5 => Some(-10),
        6 => Some(-15),
        _ => None,
    }
}

/// Fast Movement's enhancement bonus to land speed: `10 * (level / 3)` feet.
///
/// `pu_abilities_class.lst:466` —
/// `BONUS:VAR|MonkFastMovementBonus|10*floor(MonkFastMovementLVL/3)` with
/// the same row's `BONUS:VAR|MonkFastMovementLVL|MonkLVL`. `None` below
/// [`FAST_MOVEMENT_LEVEL`].
///
/// This token is byte-identical to the CRB Monk's own, so the two classes
/// genuinely share this magnitude. It is restated here rather than shared in
/// code because `pilot_compute.rs`'s `monk_fast_movement_bonus_feet` is
/// private to that module and this table must not reach into it; if the two
/// books ever diverge, this is the PU statement.
///
/// The corpus applies it only when unencumbered and wearing no armor
/// (`PREVAREQ:ENCUMBERANCE,0,var("COUNT[EQTYPE.ARMOR.EQUIPPED]"),0`); that
/// condition is a caller concern and is not silently applied here.
pub fn fast_movement_bonus_feet(level: u8) -> Option<i16> {
    if level < FAST_MOVEMENT_LEVEL {
        return None;
    }
    Some(10 * (i16::from(level) / 3))
}

/// Ki pool size: `level / 2 + ki-stat modifier`.
///
/// Three tokens:
/// - `pu_abilities_class.lst:467` — `BONUS:VAR|KiPoolLVL|MonkLVL`
/// - `core_rulebook/cr_abilities_class.lst:1175` (`Ki Pool Tracker`, the
///   shared internal record PU's row grants) —
///   `BONUS:VAR|KiPoints|KiPoolLVL/2`
/// - `core_rulebook/cr_abilities_class.lst:1179`
///   (`Ki Stat Choice ~ Wisdom`) — `BONUS:VAR|KiPoints|WIS`
///
/// The row's own `DESC:` states the same rule ("equal to 1/2 his monk level
/// + his Wisdom modifier"). `None` below [`KI_POOL_LEVEL`].
///
/// The argument is named `ki_stat_modifier`, not `wisdom_modifier`, because
/// the corpus makes it a choice: `Ki Pool Tracker` also carries
/// `Ki Stat Choice ~ Charisma` and `~ Intelligence` variants behind
/// `BONUS:ABILITYPOOL|Ki Pool Stat Choice|1|PREVARGTEQ:KiPool,2`. For a
/// plain Unchained Monk that choice is Wisdom.
pub fn ki_points(level: u8, ki_stat_modifier: i16) -> Option<i16> {
    if level < KI_POOL_LEVEL {
        return None;
    }
    Some(i16::from(level) / 2 + ki_stat_modifier)
}

/// Ki powers known: `(level - 2) / 2`.
///
/// `pu_abilities_class.lst:468` —
/// `BONUS:VAR|Pool_Unchained_Ki_Power|(MonkLVL-2)/2`. `None` below
/// [`KI_POWERS_LEVEL`], which also keeps the subtraction off the negative
/// branch. Reproduces "4th level and every 2 levels thereafter": 1 at 4th
/// rising to 9 at 20th.
pub fn ki_powers_known(level: u8) -> Option<i16> {
    if level < KI_POWERS_LEVEL {
        return None;
    }
    Some((i16::from(level) - 2) / 2)
}

/// Style strikes known: `(level - 1) / 4`.
///
/// `pu_abilities_class.lst:471` —
/// `BONUS:VAR|Pool_Unchained_Style_Strike|(MonkLVL-1)/4`. `None` below
/// [`STYLE_STRIKE_LEVEL`]. Reproduces "one at 5th, an additional one at 9th
/// and every 4 levels thereafter": 1/2/3/4 at 5th/9th/13th/17th, and it does
/// **not** grow again at 20th.
pub fn style_strikes_known(level: u8) -> Option<i16> {
    if level < STYLE_STRIKE_LEVEL {
        return None;
    }
    Some((i16::from(level) - 1) / 4)
}

/// [`STILL_MIND_SAVE_BONUS`], or `None` below [`STILL_MIND_LEVEL`]. Applies
/// only against enchantment spells and effects.
pub fn still_mind_save_bonus(level: u8) -> Option<i16> {
    if level < STILL_MIND_LEVEL {
        return None;
    }
    Some(STILL_MIND_SAVE_BONUS)
}

/// [`PERFECT_SELF_DAMAGE_REDUCTION`] (`10/chaotic`), or `None` below
/// [`PERFECT_SELF_LEVEL`].
///
/// Perfect Self's other two clauses — the permanent Outsider type change and
/// the ki-regaining "perfect calm" state — carry no numeric magnitude and
/// need engines this repo does not have. They are named here and left
/// unmodelled rather than approximated.
pub fn perfect_self_damage_reduction(level: u8) -> Option<i16> {
    if level < PERFECT_SELF_LEVEL {
        return None;
    }
    Some(PERFECT_SELF_DAMAGE_REDUCTION)
}

/// `StunningFistMonkLVL` — the monk's own level, the single token PU's
/// Stunning Fist row authors (`pu_abilities_class.lst:463`,
/// `BONUS:VAR|StunningFistMonkLVL|MonkLVL`). `None` below
/// [`STUNNING_FIST_LEVEL`].
///
/// This is *not* the uses per day and *not* the save DC — see the module
/// doc comment for where those live and why they are not restated here.
pub fn stunning_fist_monk_level(level: u8) -> Option<i16> {
    if level < STUNNING_FIST_LEVEL {
        return None;
    }
    Some(i16::from(level))
}

/// Numbers stated only in a row's English `DESC:`, never in a
/// `BONUS:`/`DEFINE:` token.
///
/// Kept in its own module for the same reason as
/// [`super::rogue_features::prose_derived`] and
/// [`super::summoner_features::prose_derived`]: so no call site can mistake a
/// sentence for a formula. Each item quotes the sentence it came from, and
/// `prose_derived_functions_still_match_the_corpus_prose` re-reads those
/// sentences off the ingested corpus records.
///
/// **What is deliberately absent.** Purity of Body (`:470`, "immunity to all
/// diseases") and Tongue of the Sun and Moon (`:473`, "understand and speak
/// with any living creature") state no number at all, in prose or in token.
/// They get no function here, because there is nothing to compute and a
/// fabricated `1` would be a magnitude the book never wrote.
pub mod prose_derived {
    use super::{EVASION_LEVEL, FLAWLESS_MIND_LEVEL, IMPROVED_EVASION_LEVEL, TIMELESS_BODY_LEVEL};

    /// The percentage of damage an Unchained Monk takes on a **successful**
    /// Reflex save against an attack that normally deals half damage on a
    /// save: `0`.
    ///
    /// From `pu_abilities_class.lst:465`, verbatim: "If a monk succeeds at a
    /// Reflex saving throw against an attack that normally deals half damage
    /// on a successful save, he instead takes no damage."
    ///
    /// "No damage" is the number: 0% where the default is 50%. That row
    /// carries no `BONUS:`/`DEFINE:` token, so the percentage is read out of
    /// the sentence.
    ///
    /// The row's two conditions — light armor or none, and not helpless —
    /// are real gates this engine cannot evaluate, so they are stated in the
    /// receipt rather than folded into this number.
    pub const EVASION_DAMAGE_PERCENT_ON_A_SUCCESSFUL_REFLEX_SAVE: i16 = 0;

    /// The percentage of damage an Unchained Monk takes on a **failed**
    /// Reflex save once Improved Evasion is online: `50`.
    ///
    /// From `pu_abilities_class.lst:472`, verbatim: "He still takes no damage
    /// on successful Ref lex saving throws against attacks, but henceforth he
    /// takes only half damage on failed saves." (The broken "Ref lex" is the
    /// corpus's own line-break artefact; it is quoted, not corrected.)
    pub const IMPROVED_EVASION_DAMAGE_PERCENT_ON_A_FAILED_REFLEX_SAVE: i16 = 50;

    /// [`EVASION_DAMAGE_PERCENT_ON_A_SUCCESSFUL_REFLEX_SAVE`] from
    /// [`EVASION_LEVEL`], `None` below it.
    pub fn evasion_damage_percent_on_a_successful_reflex_save(level: u8) -> Option<i16> {
        if level < EVASION_LEVEL {
            return None;
        }
        Some(EVASION_DAMAGE_PERCENT_ON_A_SUCCESSFUL_REFLEX_SAVE)
    }

    /// [`IMPROVED_EVASION_DAMAGE_PERCENT_ON_A_FAILED_REFLEX_SAVE`] from
    /// [`IMPROVED_EVASION_LEVEL`], `None` below it.
    pub fn improved_evasion_damage_percent_on_a_failed_reflex_save(level: u8) -> Option<i16> {
        if level < IMPROVED_EVASION_LEVEL {
            return None;
        }
        Some(IMPROVED_EVASION_DAMAGE_PERCENT_ON_A_FAILED_REFLEX_SAVE)
    }

    /// How many d20s an Unchained Monk of [`FLAWLESS_MIND_LEVEL`] rolls for a
    /// Will save, keeping the better: `2`.
    ///
    /// From `pu_abilities_class.lst:475`, verbatim: "Whenever he attempts a
    /// Will save, he can roll twice and take the better result."
    ///
    /// "Roll twice" is the number. The row's second clause — a fresh save at
    /// the end of each hour against effects lasting longer than an hour —
    /// is a retry interval, carried in the receipt's text; it is not a
    /// second magnitude and gets no constant of its own.
    pub fn flawless_mind_will_save_rolls(level: u8) -> Option<i16> {
        if level < FLAWLESS_MIND_LEVEL {
            return None;
        }
        Some(2)
    }

    /// The ability-score penalty an Unchained Monk of [`TIMELESS_BODY_LEVEL`]
    /// takes for aging: `0`.
    ///
    /// From `pu_abilities_class.lst:474`, verbatim: "a monk no longer takes
    /// penalties to his ability scores for aging and cannot be magically
    /// aged."
    ///
    /// A genuine zero, not a filler one: the row's numeric content is that
    /// the aging penalty becomes nothing. The same sentence's carve-outs —
    /// penalties already taken remain, age *bonuses* still accrue, and the
    /// monk still dies of old age — are stated in the receipt, because this
    /// number is the penalty and not a claim of immortality.
    pub fn timeless_body_aging_ability_penalty(level: u8) -> Option<i16> {
        if level < TIMELESS_BODY_LEVEL {
            return None;
        }
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn corpus_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("data/corpus/pathfinder_unchained/class_feature/monk_unchained_class")
    }

    fn ingested_records() -> Vec<serde_json::Value> {
        let mut out = Vec::new();
        let dir = corpus_dir();
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("ingested Monk corpus dir {dir:?} must exist: {e}"));
        for entry in entries {
            let path = entry.expect("readable dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable corpus record");
            let value: serde_json::Value =
                serde_json::from_str(&text).expect("corpus record is valid JSON");
            out.push(value["data"].clone());
        }
        out
    }

    fn bonus_tokens(record: &serde_json::Value) -> Vec<String> {
        record["raw_bonus_chains"]
            .as_array()
            .expect("raw_bonus_chains is an array")
            .iter()
            .map(|chain| {
                let parts: Vec<String> = chain["qualifiers"]
                    .as_array()
                    .expect("qualifiers is an array")
                    .iter()
                    .map(|q| q.as_str().expect("qualifier is a string").to_owned())
                    .collect();
                format!("BONUS:{}", parts.join("|"))
            })
            .collect()
    }

    fn record_for(key: &str) -> serde_json::Value {
        ingested_records()
            .into_iter()
            .find(|r| r["key"] == key)
            .unwrap_or_else(|| panic!("no ingested record with KEY:{key}"))
    }

    fn assert_bonus_token(key: &str, token: &str) {
        let record = record_for(key);
        let tokens = bonus_tokens(&record);
        assert!(
            tokens.iter().any(|t| t == token),
            "{key} must carry the exact token {token:?}; it carries {tokens:?}"
        );
    }

    fn description_of(key: &str) -> String {
        record_for(key)["description"]
            .as_str()
            .unwrap_or_else(|| panic!("{key} must carry a rendered description"))
            .to_owned()
    }

    fn pcgen_root() -> PathBuf {
        PathBuf::from(
            std::env::var("PCGEN_CORPUS_ROOT")
                .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
        )
    }

    /// The roster must be exactly the ingested set — same size, same keys,
    /// same names, same grant levels.
    #[test]
    fn feature_roster_is_the_eighteen_ingested_records() {
        let ingested = ingested_records();
        assert_eq!(ingested.len(), 18, "the ingested Monk feature directory must hold 18 records");
        assert_eq!(features().len(), ingested.len());

        for feature in features() {
            let record = record_for(feature.key);
            assert_eq!(
                record["name"].as_str(),
                Some(feature.name),
                "{}: name disagrees with the corpus",
                feature.key
            );
            assert_eq!(
                record["min_level"].as_u64().map(|v| v as u8),
                Some(feature.min_level),
                "{}: min_level disagrees with the corpus",
                feature.key
            );
            assert_eq!(
                record["is_granted"].as_bool(),
                Some(true),
                "{}: every Unchained Monk feature is granted",
                feature.key
            );
        }
    }

    /// The chassis is the Unchained Monk's whole point, and it must not
    /// collapse into the CRB Monk's. Asserted against the ingested Core
    /// Rulebook `CLASS:Monk` record, so that a change to either book's
    /// chassis fails here rather than silently merging the two classes.
    #[test]
    fn unchained_monk_chassis_differs_from_the_crb_monk() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("data/corpus/core_rulebook/class/monk.json");
        let text = std::fs::read_to_string(&path).expect("CRB Monk record must be readable");
        let crb: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
        let crb = &crb["data"];
        assert_eq!(crb["class_id"].as_str(), Some("Monk"));

        // CRB Monk: three-quarter BAB. Unchained Monk: full, replacing it.
        assert_eq!(crb["bab"].as_str(), Some("level*3/4"));
        for level in 1..=MAX_SUPPORTED_LEVEL {
            let crb_three_quarter = (i16::from(level) * 3) / 4;
            assert_eq!(base_attack_bonus(level), i16::from(level), "level {level}");
            if level >= 4 {
                assert!(
                    base_attack_bonus(level) > crb_three_quarter,
                    "level {level}: the Unchained Monk must out-attack the CRB Monk"
                );
            }
        }

        // CRB Monk: three good saves. Unchained Monk: Will is poor.
        assert_eq!(crb["save_fort"].as_str(), Some("level/2+2"));
        assert_eq!(crb["save_ref"].as_str(), Some("level/2+2"));
        assert_eq!(crb["save_will"].as_str(), Some("level/2+2"));
        assert_eq!(fort_save(20), 12, "good progression: 20/2+2");
        assert_eq!(ref_save(20), 12);
        assert_eq!(will_save(20), 6, "poor progression: 20/3");
        assert_ne!(will_save(20), fort_save(20));

        // Hit die. NOTE: `rules_tables::crb::class_tables` deliberately
        // ships the CRB Monk at d8 against its own corpus `HD:10`, per the
        // operator's 2026-07-29 ruling (risks item 91). That override is a
        // repo decision about the CRB row and does not reach here: the
        // Unchained Monk's d10 comes from a different token in a different
        // file (`pu_templates.lst:5`). Asserted as the Unchained fact only;
        // the CRB override is not re-litigated by this module.
        assert_eq!(HIT_DIE, 10);
    }

    /// The chassis constants must equal what the ingestion cycle recorded on
    /// the class record — two independent transcriptions of one row.
    #[test]
    fn chassis_matches_the_ingested_class_record() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("data/corpus/pathfinder_unchained/class/monk_unchained_class.json");
        let text = std::fs::read_to_string(&path).expect("class record must be readable");
        let value: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
        let data = &value["data"];
        assert_eq!(data["base_class_key"].as_str(), Some("Monk"));
        assert_eq!(data["base_class_book"].as_str(), Some("core_rulebook"));
        assert_eq!(data["hit_die"].as_u64(), Some(u64::from(HIT_DIE)));
        assert_eq!(data["hit_die_template"].as_str(), Some("Monk ~ Unchained HD"));
        assert_eq!(data["bab"].as_str(), Some("level"));
        assert_eq!(data["bab_replaces_base"].as_bool(), Some(true));
        assert_eq!(data["save_fort"].as_str(), Some("level/2+2"));
        assert_eq!(data["save_ref"].as_str(), Some("level/2+2"));
        assert_eq!(data["save_will"].as_str(), Some("level/3"));
    }

    #[test]
    fn ac_bonus_rises_every_four_levels_and_caps_at_five() {
        assert_eq!(armor_class_bonus_from_level(1), 0);
        assert_eq!(armor_class_bonus_from_level(3), 0);
        assert_eq!(armor_class_bonus_from_level(4), 1);
        assert_eq!(armor_class_bonus_from_level(8), 2);
        assert_eq!(armor_class_bonus_from_level(12), 3);
        assert_eq!(armor_class_bonus_from_level(16), 4);
        assert_eq!(armor_class_bonus_from_level(20), 5);

        // The Wisdom half is floored at zero -- a negative modifier must not
        // lower the monk's AC.
        assert_eq!(armor_class_bonus(20, 5), 10);
        assert_eq!(armor_class_bonus(1, -2), 0);
        assert_eq!(armor_class_bonus(4, 0), 1);
    }

    /// The formula and the row's own prose are two independent statements of
    /// one rule; pin them so neither can drift alone.
    #[test]
    fn ac_bonus_level_component_agrees_with_the_rows_own_prose() {
        let desc = description_of("Unchained Monk ~ AC Bonus");
        assert!(
            desc.contains("a monk gains a +1 bonus to AC and CMD at 4th level"),
            "AC Bonus prose changed: {desc}"
        );
        assert!(
            desc.contains("up to a maximum of +5 at 20th level"),
            "AC Bonus prose changed: {desc}"
        );
        assert_eq!(armor_class_bonus_from_level(4), 1);
        assert_eq!(armor_class_bonus_from_level(20), AC_BONUS_LEVEL_COMPONENT_CAP);
        // The cap genuinely binds: without it, level 20 would read 5 anyway,
        // so assert it binds where it actually would -- nowhere below 24.
        assert_eq!(armor_class_bonus_from_level(24), AC_BONUS_LEVEL_COMPONENT_CAP);
    }

    #[test]
    fn bonus_feats_arrive_at_one_two_and_every_four_levels_thereafter() {
        let gain_levels: Vec<u8> = (1..=MAX_SUPPORTED_LEVEL)
            .filter(|&level| bonus_feats_known(level) != bonus_feats_known(level - 1))
            .collect();
        assert_eq!(gain_levels, vec![1, 2, 6, 10, 14, 18]);
        assert_eq!(bonus_feats_known(0), None);
        assert_eq!(bonus_feats_known(1), Some(1));
        assert_eq!(bonus_feats_known(2), Some(2));
        assert_eq!(bonus_feats_known(20), Some(6));
    }

    /// The flurry ladder must reproduce the published Unchained Monk attack
    /// line, band by band. This is the check that the boolean-sum formula was
    /// transcribed correctly rather than plausibly.
    #[test]
    fn flurry_reproduces_the_published_attack_line() {
        let line = |bab: i16| -> Vec<i16> {
            let count = flurry_attack_count(bab);
            (1..=count as u8)
                .map(|i| {
                    bab + flurry_iterative_attack_penalty(i, count)
                        .unwrap_or_else(|| panic!("attack {i} of {count} must exist"))
                })
                .collect()
        };
        assert_eq!(line(1), vec![1, 1]);
        assert_eq!(line(5), vec![5, 5]);
        assert_eq!(line(6), vec![6, 6, 1]);
        assert_eq!(line(10), vec![10, 10, 5]);
        assert_eq!(line(11), vec![11, 11, 11, 6, 1]);
        assert_eq!(line(15), vec![15, 15, 15, 10, 5]);
        assert_eq!(line(16), vec![16, 16, 16, 11, 6, 1]);
        assert_eq!(line(20), vec![20, 20, 20, 15, 10, 5]);

        assert_eq!(flurry_attack_count_at_monk_level(20), 6);
        assert_eq!(flurry_attack_count_at_monk_level(1), 2);
    }

    /// Slot 7 exists in the corpus as `BONUS:VAR|FAB_7|FAB` and is
    /// deliberately unmodelled. Pinned so that "returns None" is a decision
    /// with a test, not an oversight.
    #[test]
    fn flurry_refuses_attacks_beyond_the_count_including_the_unmodelled_seventh() {
        assert_eq!(flurry_iterative_attack_penalty(0, 6), None);
        assert_eq!(flurry_iterative_attack_penalty(3, 2), None);
        assert_eq!(flurry_iterative_attack_penalty(7, 6), None);
        // Even if a caller claimed a 7-attack flurry, slot 7 stays unmodelled.
        assert_eq!(flurry_iterative_attack_penalty(7, 7), None);
    }

    /// The corpus's two flurry-count variables are written differently and
    /// must be numerically identical; checked rather than assumed.
    #[test]
    fn flurry_extra_attacks_is_numerically_identical_to_flurry_attacks() {
        for bab in 0..=25 {
            let extra = 2
                + i16::from(bab >= 11)
                + i16::from(bab >= 6)
                + i16::from(bab >= 11)
                + i16::from(bab >= 16);
            assert_eq!(flurry_attack_count(bab), extra, "BAB {bab}");
        }
    }

    #[test]
    fn fast_movement_rises_ten_feet_every_three_levels() {
        assert_eq!(fast_movement_bonus_feet(2), None);
        assert_eq!(fast_movement_bonus_feet(3), Some(10));
        assert_eq!(fast_movement_bonus_feet(5), Some(10));
        assert_eq!(fast_movement_bonus_feet(6), Some(20));
        assert_eq!(fast_movement_bonus_feet(18), Some(60));
        assert_eq!(fast_movement_bonus_feet(20), Some(60));
    }

    #[test]
    fn ki_points_match_the_corpus_formula() {
        assert_eq!(ki_points(2, 3), None);
        assert_eq!(ki_points(3, 3), Some(4));
        assert_eq!(ki_points(4, 3), Some(5));
        assert_eq!(ki_points(20, 5), Some(15));
        // A negative ki-stat modifier genuinely shrinks the pool: the corpus
        // carries no max() on this token, unlike the AC bonus's.
        assert_eq!(ki_points(10, -1), Some(4));
    }

    #[test]
    fn ki_powers_arrive_at_fourth_and_every_two_levels() {
        assert_eq!(ki_powers_known(3), None);
        let gain_levels: Vec<u8> = (KI_POWERS_LEVEL..=MAX_SUPPORTED_LEVEL)
            .filter(|&level| ki_powers_known(level) != ki_powers_known(level - 1))
            .collect();
        assert_eq!(gain_levels, vec![4, 6, 8, 10, 12, 14, 16, 18, 20]);
        assert_eq!(ki_powers_known(4), Some(1));
        assert_eq!(ki_powers_known(20), Some(9));
    }

    #[test]
    fn style_strikes_arrive_at_fifth_ninth_and_every_four_levels() {
        assert_eq!(style_strikes_known(4), None);
        let gain_levels: Vec<u8> = (STYLE_STRIKE_LEVEL..=MAX_SUPPORTED_LEVEL)
            .filter(|&level| style_strikes_known(level) != style_strikes_known(level - 1))
            .collect();
        assert_eq!(gain_levels, vec![5, 9, 13, 17]);
        assert_eq!(style_strikes_known(20), Some(4));
    }

    #[test]
    fn level_gated_flat_magnitudes_appear_exactly_at_their_grant_level() {
        assert_eq!(still_mind_save_bonus(3), None);
        assert_eq!(still_mind_save_bonus(4), Some(2));
        assert_eq!(perfect_self_damage_reduction(19), None);
        assert_eq!(perfect_self_damage_reduction(20), Some(10));
        assert_eq!(stunning_fist_monk_level(0), None);
        assert_eq!(stunning_fist_monk_level(1), Some(1));
        assert_eq!(stunning_fist_monk_level(20), Some(20));
    }

    /// `Unchained Monk ~ Evasion` (`:465`) and `~ Improved Evasion` (`:472`)
    /// each carry a `DESC:` and **no** `BONUS:`/`DEFINE:` token. Before this
    /// they computed nothing; the numbers their prose states are 0% damage on
    /// a made Reflex save and 50% on a failed one.
    #[test]
    fn evasion_percentages_are_the_ones_the_two_rows_prose_states() {
        use prose_derived::{
            evasion_damage_percent_on_a_successful_reflex_save as made,
            improved_evasion_damage_percent_on_a_failed_reflex_save as failed,
        };
        for level in 0..EVASION_LEVEL {
            assert_eq!(made(level), None, "level {level}");
        }
        for level in EVASION_LEVEL..=MAX_SUPPORTED_LEVEL {
            assert_eq!(made(level), Some(0), "level {level}");
        }
        for level in 0..IMPROVED_EVASION_LEVEL {
            assert_eq!(failed(level), None, "level {level}");
        }
        for level in IMPROVED_EVASION_LEVEL..=MAX_SUPPORTED_LEVEL {
            assert_eq!(failed(level), Some(50), "level {level}");
        }

        for key in ["Unchained Monk ~ Evasion", "Unchained Monk ~ Improved Evasion"] {
            assert!(
                bonus_tokens(&record_for(key)).is_empty(),
                "{key} must still carry no BONUS: token -- if it gained one, the prose-derived \
                 reading is no longer the only source and must be revisited"
            );
        }
        assert!(
            description_of("Unchained Monk ~ Evasion").contains(
                "against an attack that normally deals half damage on a successful save, he \
                 instead takes no damage"
            ),
            "Evasion prose changed; the 0% reading must be re-derived"
        );
        assert!(
            description_of("Unchained Monk ~ Improved Evasion")
                .contains("henceforth he takes only half damage on failed saves"),
            "Improved Evasion prose changed; the 50% reading must be re-derived"
        );
    }

    /// `Unchained Monk ~ Flawless Mind` (`:475`) and `~ Timeless Body`
    /// (`:474`) — same shape, same fix.
    #[test]
    fn flawless_mind_and_timeless_body_state_the_numbers_their_prose_carries() {
        use prose_derived::{
            flawless_mind_will_save_rolls as rolls,
            timeless_body_aging_ability_penalty as aging_penalty,
        };
        for level in 0..FLAWLESS_MIND_LEVEL {
            assert_eq!(rolls(level), None, "level {level}");
        }
        assert_eq!(rolls(FLAWLESS_MIND_LEVEL), Some(2));
        assert_eq!(rolls(MAX_SUPPORTED_LEVEL), Some(2));

        for level in 0..TIMELESS_BODY_LEVEL {
            assert_eq!(aging_penalty(level), None, "level {level}");
        }
        for level in TIMELESS_BODY_LEVEL..=MAX_SUPPORTED_LEVEL {
            assert_eq!(aging_penalty(level), Some(0), "level {level}");
        }

        for key in ["Unchained Monk ~ Flawless Mind", "Unchained Monk ~ Timeless Body"] {
            assert!(
                bonus_tokens(&record_for(key)).is_empty(),
                "{key} must still carry no BONUS: token"
            );
        }
        assert!(
            description_of("Unchained Monk ~ Flawless Mind")
                .contains("Whenever he attempts a Will save, he can roll twice and take the better result"),
            "Flawless Mind prose changed; the two-rolls reading must be re-derived"
        );
        assert!(
            description_of("Unchained Monk ~ Timeless Body")
                .contains("a monk no longer takes penalties to his ability scores for aging"),
            "Timeless Body prose changed; the zero-aging-penalty reading must be re-derived"
        );
    }

    /// The two Unchained Monk features that genuinely state no number, in
    /// prose or in token. This pins the *reason* they compute nothing, so a
    /// later cycle cannot quietly invent a magnitude for them — and so that
    /// if the corpus ever grows one, this test fails and says so.
    #[test]
    fn purity_of_body_and_tongue_of_the_sun_and_moon_state_no_number_at_all() {
        for (key, grant_level_digits) in [
            ("Unchained Monk ~ Purity of Body", "5"),
            ("Unchained Monk ~ Tongue of the Sun and Moon", "13"),
        ] {
            assert!(
                bonus_tokens(&record_for(key)).is_empty(),
                "{key} carries no BONUS: token"
            );
            let description = description_of(key);
            let digits: String = description.chars().filter(|c| c.is_ascii_digit()).collect();
            assert_eq!(
                digits, grant_level_digits,
                "{key}'s only digits must still be its own grant level -- a new number in this \
                 prose means a magnitude is now derivable and must be modelled. \
                 Corpus says: {description}"
            );
        }
    }

    /// The two prose-derived constants must still match the prose the corpus
    /// actually carries. Prose-derived is allowed here (see each constant's
    /// doc comment) but it is not allowed to be unchecked.
    #[test]
    fn prose_derived_constants_still_match_the_corpus_prose() {
        assert!(
            description_of("Unchained Monk ~ Still Mind").contains(
                "a monk gains a +2 bonus on saving throws against enchantment spells and effects"
            ),
            "Still Mind prose changed"
        );
        assert!(
            description_of("Unchained Monk ~ Perfect Self")
                .contains("the monk gains damage reduction 10/chaotic"),
            "Perfect Self prose changed"
        );
    }

    /// §24 compliance for every magnitude whose token the ingested record
    /// actually carries.
    #[test]
    fn every_modelled_formula_is_byte_exact_against_the_ingested_corpus_record() {
        assert_bonus_token("Unchained Monk ~ AC Bonus", "BONUS:VAR|MonkACLVL|MonkLVL|TYPE=Level");
        assert_bonus_token(
            "Unchained Monk ~ Bonus Feat",
            "BONUS:ABILITYPOOL|Unchained Monk Bonus Feat|1+max((MonkBonusFeatLVL+2)/4,0)",
        );
        assert_bonus_token("Unchained Monk ~ Bonus Feat", "BONUS:VAR|MonkBonusFeatLVL|MonkLVL");
        assert_bonus_token(
            "Unchained Monk ~ Fast Movement",
            "BONUS:VAR|MonkFastMovementBonus|10*floor(MonkFastMovementLVL/3)",
        );
        assert_bonus_token(
            "Unchained Monk ~ Fast Movement",
            "BONUS:VAR|MonkFastMovementLVL|MonkLVL",
        );
        assert_bonus_token("Unchained Monk ~ Ki Pool", "BONUS:VAR|KiPoolLVL|MonkLVL");
        assert_bonus_token(
            "Unchained Monk ~ Ki Powers",
            "BONUS:VAR|Pool_Unchained_Ki_Power|(MonkLVL-2)/2",
        );
        assert_bonus_token(
            "Unchained Monk ~ Style Strike",
            "BONUS:VAR|Pool_Unchained_Style_Strike|(MonkLVL-1)/4",
        );
        assert_bonus_token(
            "Unchained Monk ~ Stunning Fist",
            "BONUS:VAR|StunningFistMonkLVL|MonkLVL",
        );

        // SD-32 T12 row 21 cycle 2: Flurry of Blows' formulas live on a
        // separate `.MOD` block (raw `.lst` lines 492-504) that
        // `ingest_pu_classes.rs` used to read only the base row and silently
        // drop -- the exact `.MOD`-appended-row-loss defect row 21 fixed for
        // the generic `class_feature.rs` path, found live in THIS book's own
        // generator too and fixed here (`raw_tokens_excluding_bonus`/
        // `raw_bonus_chains` now read the full `.MOD` closure, matching
        // `out_of_record_formulas_are_byte_exact_against_the_real_lst_rows`
        // below, which independently pins the same raw `.lst` tokens). No
        // longer an honest absence -- the real tokens now ship.
        for token in [
            "BONUS:VAR|FlurryExtraAttacks|2+(Total_BAB>=11)+(Total_BAB>=6)+(Total_BAB>=11)+(Total_BAB>=16)",
            "BONUS:VAR|Total_BAB|BAB",
            "BONUS:VAR|FlurryAttacks|2+(Total_BAB>=6)+if(Total_BAB>=11,2,0)+(Total_BAB>=16)",
            "BONUS:VAR|FAB_1|FAB",
            "BONUS:VAR|FAB_2|FAB",
            "BONUS:VAR|FAB_3|FAB+if(FlurryAttacks==3,-5,0)",
            "BONUS:VAR|FAB_4|FAB-5",
            "BONUS:VAR|FAB_5|FAB-10",
            "BONUS:VAR|FAB_6|FAB-15",
            "BONUS:VAR|FAB_7|FAB",
        ] {
            assert_bonus_token("Unchained Monk ~ Flurry of Blows", token);
        }
    }

    /// The magnitudes that live outside the ingested records — the flurry
    /// `.MOD` block, the chassis row, and the two shared Core Rulebook
    /// internal trackers — can only be pinned against the raw `.lst`. Opt-in
    /// via `PCGEN_CORPUS_ROOT`, but genuinely checked when it is set.
    #[test]
    #[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
    fn out_of_record_formulas_are_byte_exact_against_the_real_lst_rows() {
        let root = pcgen_root();

        let pu = std::fs::read_to_string(root.join(
            "pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_abilities_class.lst",
        ))
        .expect("pu_abilities_class.lst must be readable");
        let pu_lines: Vec<&str> = pu.lines().collect();

        // Chassis row.
        let chassis = pu_lines[114];
        assert!(chassis.contains("KEY:Monk ~ Unchained Class"), "line 115 moved: {chassis}");
        for token in [
            "BONUS:COMBAT|BASEAB|classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")|TYPE=Base.REPLACE",
            "BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")/2+2",
            "BONUS:SAVE|BASE.Will|classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")/3",
            "TEMPLATE:Monk ~ Unchained HD",
        ] {
            assert!(chassis.contains(token), "line 115 must carry {token:?}");
        }

        // Flurry `.MOD` block.
        for (index, token) in [
            (491_usize, "BONUS:VAR|FlurryExtraAttacks|2+(Total_BAB>=11)+(Total_BAB>=6)+(Total_BAB>=11)+(Total_BAB>=16)"),
            (493, "BONUS:VAR|Total_BAB|BAB"),
            (494, "BONUS:VAR|FlurryAttacks|2+(Total_BAB>=6)+if(Total_BAB>=11,2,0)+(Total_BAB>=16)"),
            (496, "BONUS:VAR|FAB_1|FAB"),
            (497, "BONUS:VAR|FAB_2|FAB"),
            (498, "BONUS:VAR|FAB_3|FAB+if(FlurryAttacks==3,-5,0)"),
            (499, "BONUS:VAR|FAB_4|FAB-5"),
            (500, "BONUS:VAR|FAB_5|FAB-10"),
            (501, "BONUS:VAR|FAB_6|FAB-15"),
            (502, "BONUS:VAR|FAB_7|FAB"),
        ] {
            let line = pu_lines[index];
            assert!(
                line.contains("Unchained Monk ~ Flurry of Blows.MOD"),
                "line {} is not a flurry .MOD row: {line}",
                index + 1
            );
            assert!(line.contains(token), "line {} must carry {token:?}", index + 1);
        }

        // Hit die template.
        let templates = std::fs::read_to_string(
            root.join("pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_templates.lst"),
        )
        .expect("pu_templates.lst must be readable");
        assert!(
            templates.lines().any(|l| l.starts_with("Monk ~ Unchained HD")
                && l.contains("HITDIE:10|CLASS=Monk")),
            "the Unchained HD template must state HITDIE:10"
        );

        // The two shared Core Rulebook internal trackers.
        let crb = std::fs::read_to_string(
            root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst"),
        )
        .expect("cr_abilities_class.lst must be readable");
        let crb_lines: Vec<&str> = crb.lines().collect();

        let ac_tracker = crb_lines[1089];
        assert!(ac_tracker.starts_with("Monk AC Tracker"), "line 1090 moved: {ac_tracker}");
        assert!(ac_tracker.contains("BONUS:VAR|MonkACBonus|min((MonkACLVL)/4,5)|TYPE=level"));
        assert!(ac_tracker.contains("BONUS:VAR|MonkACStatBonus|max(WIS,0)|TYPE=Class|PREVARGTEQ:IsMonk,1"));

        let ki_tracker = crb_lines[1174];
        assert!(ki_tracker.starts_with("Ki Pool Tracker"), "line 1175 moved: {ki_tracker}");
        assert!(ki_tracker.contains("BONUS:VAR|KiPoints|KiPoolLVL/2"));

        let ki_stat = crb_lines[1178];
        assert!(ki_stat.starts_with("Ki Stat Choice ~ Wisdom"), "line 1179 moved: {ki_stat}");
        assert!(ki_stat.contains("BONUS:VAR|KiPoints|WIS"));
    }
}
