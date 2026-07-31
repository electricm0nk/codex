//! PF1 creature size, modelled only as far as carrying capacity needs it
//! (v0.6 alpha swarm).
//!
//! # Why this module exists
//!
//! `encumbrance.rs` computed every character's carrying capacity as though
//! it were Medium, because creature size was not modelled anywhere in this
//! crate. PF1 scales capacity by size, so Gnome and Halfling characters
//! (both Small, and both curated playable races) were handed thresholds
//! `4/3` of their true values -- and with them the wrong load tier, the
//! wrong max-Dex cap, and the wrong armor check penalty.
//!
//! This module is the single owner of "what size is this creature, and what
//! does that multiply", so `encumbrance.rs` does not have to invent a
//! second, unowned source of truth for it (the concern its own doc comment
//! raised when it deliberately stopped short of fixing this).
//!
//! # Scope
//!
//! Per `docs/governance/no-stub-mvp-doctrine.md`, this module carries only
//! the size facts that a real consumer applies today:
//!
//!  - **carrying capacity** (`load_capacity_ratio`), and
//!  - **PF1's Table 8-1 size modifiers** (`armor_class_size_modifier`,
//!    `special_size_modifier`, `stealth_size_modifier`), added by SD-27 for
//!    the defect recorded at `docs/release/SD-27-.../decisions.md §28`: no
//!    race had a size modifier to AC, touch AC, CMB or CMD, so a Small
//!    character's sheet showed a Medium character's arithmetic.
//!
//! PF1 also scales weapon damage dice and Fly checks by size. Neither is
//! applied anywhere, and this module deliberately offers no helper that
//! would make them look available -- they are named as deferred rather than
//! half-built.
//!
//! # Sources
//!
//! Three facts meet here, from three different places -- none is inferred:
//!
//!  - **Which size a creature is**: the race's `~ Size` racial-default
//!    trait row's `TEMPLATE:SIZE_<code>`, falling back to the chassis'
//!    `FACT:BaseSize|<code>`. Owned by `race_resolver::RACE_SIZES` /
//!    `race_resolver::race_size_for_race_token`, which is the authority --
//!    **not** `rules_tables::crb::race_tables::race_size`, which knew only
//!    the 7 hardcoded CRB races.
//!  - **What that size multiplies capacity by**: `SIZEMULT:<code>|<value>`
//!    in the PCGen Pathfinder game mode's
//!    `/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`
//!    -- the same file `encumbrance.rs`'s `LOAD:` table comes from.
//!    Transcribed in `SizeCategory::load_capacity_ratio` below.
//!  - **What that size modifies on the sheet**: PF1 Core Rulebook Table 8-1,
//!    "Size Modifiers". Transcribed in full below. This one is *not* read
//!    out of the corpus: PCGen encodes it as `BONUS:COMBAT|AC|...` /
//!    `BONUS:COMBAT|TOHIT|...` rows spread across the game mode's size
//!    adjustment definitions, and this repo has no ingested game-mode
//!    surface at all. Per `decisions.md §24` it is therefore hand-modelled
//!    as a small pure function with the published table pinned by tests,
//!    which is the same treatment every class feature in this repo gets.

/// A PF1 creature size category.
///
/// The variant set and the single-letter codes are PCGen's own
/// (`load.lst`'s `SIZEMULT:F|D|T|S|L|H|G|C` rows plus the unmultiplied
/// Medium baseline), which is also the code used by each race record's
/// `FACT:BaseSize|<code>` token.
///
/// Only `Small` and `Medium` are reachable from this crate's seven curated
/// playable races today (Gnome and Halfling are Small; Human, Dwarf, Elf,
/// Half-Elf and Half-Orc are Medium). The remaining variants are carried
/// because `load_capacity_ratio` transcribes `load.lst`'s `SIZEMULT:`
/// table **in full**: a partial transcription is precisely the shape of
/// error that put a wrong Strength-15 threshold into this codebase, and it
/// would leave the next person to need Large or Tiny guessing at a value
/// that is sitting right there in the source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SizeCategory {
    Fine,
    Diminutive,
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
    Colossal,
}

impl SizeCategory {
    /// This size's carrying-capacity multiplier, as an exact
    /// `(numerator, denominator)` rational.
    ///
    /// Transcribed verbatim from
    /// `/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`
    /// lines 1-8:
    ///
    /// ```text
    /// SIZEMULT:F|0.125   SIZEMULT:D|0.25   SIZEMULT:T|0.5   SIZEMULT:S|0.75
    /// SIZEMULT:L|2       SIZEMULT:H|4      SIZEMULT:G|8     SIZEMULT:C|16
    /// ```
    ///
    /// `Medium` has no `SIZEMULT:` row in `load.lst` at all. That is not an
    /// omission to be guessed around: the `LOAD:` column is *itself*
    /// expressed at Medium size (`LOAD:10|100` is exactly PF1's published
    /// Medium Strength-10 heavy maximum), so Medium is the unmultiplied
    /// baseline and its ratio is `1/1`.
    ///
    /// # Why a rational and not an `f64`
    ///
    /// So the arithmetic in `encumbrance::carrying_capacity_thresholds` can
    /// stay in exact integers. PF1 truncates load thresholds to whole
    /// pounds, and `f64` division makes truncation a coin flip at exact
    /// boundaries: `300.0 * (2.0 / 3.0)` is `199.999...`, which truncates
    /// to 199 where the correct answer is 200. Integer `(value * num * 2)
    /// / (den * 3)` cannot drift.
    pub fn load_capacity_ratio(self) -> (i64, i64) {
        match self {
            SizeCategory::Fine => (1, 8),          // SIZEMULT:F|0.125
            SizeCategory::Diminutive => (1, 4),    // SIZEMULT:D|0.25
            SizeCategory::Tiny => (1, 2),          // SIZEMULT:T|0.5
            SizeCategory::Small => (3, 4),         // SIZEMULT:S|0.75
            SizeCategory::Medium => (1, 1),        // no SIZEMULT row: the baseline
            SizeCategory::Large => (2, 1),         // SIZEMULT:L|2
            SizeCategory::Huge => (4, 1),          // SIZEMULT:H|4
            SizeCategory::Gargantuan => (8, 1),    // SIZEMULT:G|8
            SizeCategory::Colossal => (16, 1),     // SIZEMULT:C|16
        }
    }

    /// This size's modifier to **Armor Class** -- and therefore to touch AC
    /// and flat-footed AC, both of which are the same Armor Class with
    /// contributors removed rather than separate statistics -- and to
    /// attack rolls.
    ///
    /// PF1 Core Rulebook Table 8-1, "Size Modifiers", column *Size Modifier*:
    ///
    /// ```text
    /// Fine +8   Diminutive +4   Tiny +2   Small +1   Medium +0
    /// Large -1  Huge -2         Gargantuan -4        Colossal -8
    /// ```
    ///
    /// # This is a size modifier, not a bonus of some other type
    ///
    /// It occupies its own PF1 modifier type: it never stacks with another
    /// size modifier, it is not a dodge bonus (so it is *not* lost when the
    /// creature is denied its Dexterity), and it is not an armor, natural
    /// armor, deflection, or untyped bonus. A creature has exactly one size,
    /// so exactly one of these applies and the non-stacking rule is
    /// satisfied structurally rather than by a max() over a list.
    ///
    /// The engine applies this to **Armor Class, touch Armor Class and attack
    /// rolls**, on both compute paths
    /// (`pilot_compute::compute_combat_baseline` and
    /// `pilot_compute_corpus::compute_combat_baseline_from_corpus`). The
    /// attack-roll half was a stated gap here until SD-27 closed it; the note
    /// is updated rather than deleted so the record of what was open, and when
    /// it shut, survives.
    ///
    /// Flat-footed Armor Class also takes it, and is still computed in the
    /// desktop view rather than the engine -- a genuinely remaining gap, named
    /// here for the same reason the attack-roll one was.
    pub fn armor_class_size_modifier(self) -> i16 {
        match self {
            SizeCategory::Fine => 8,
            SizeCategory::Diminutive => 4,
            SizeCategory::Tiny => 2,
            SizeCategory::Small => 1,
            SizeCategory::Medium => 0,
            SizeCategory::Large => -1,
            SizeCategory::Huge => -2,
            SizeCategory::Gargantuan => -4,
            SizeCategory::Colossal => -8,
        }
    }

    /// This size's **special size modifier**, which is what CMB and CMD use
    /// in place of the ordinary size modifier.
    ///
    /// PF1 Core Rulebook, Combat Maneuver Bonus / Combat Maneuver Defense:
    /// both are computed with the *special size modifier*, which runs in the
    /// opposite direction to the Armor Class one -- being bigger helps you
    /// grapple, and being smaller helps you dodge arrows.
    ///
    /// ```text
    /// Fine -8   Diminutive -4   Tiny -2   Small -1   Medium +0
    /// Large +1  Huge +2         Gargantuan +4        Colossal +8
    /// ```
    ///
    /// # Why this is its own function and not `-armor_class_size_modifier()`
    ///
    /// For the nine PF1 sizes the two tables happen to be exact negations,
    /// so deriving one from the other would compile and pass. It is written
    /// out anyway because the identity is a coincidence of the published
    /// numbers, not a rule -- Table 8-1 and the CMB/CMD text are two
    /// separate statements in the book. Deriving it would encode "these are
    /// the same fact negated", which is the kind of unstated assumption this
    /// repo has repeatedly been burned by. The test below pins both
    /// independently against the published values.
    ///
    /// Applied since SD-27 by `pilot_compute::combat_maneuver_bonus` and
    /// `combat_maneuver_defense`, which both engine paths call -- before that
    /// nothing called this function at all, and CMB/CMD were computed in React
    /// with no size term.
    pub fn special_size_modifier(self) -> i16 {
        match self {
            SizeCategory::Fine => -8,
            SizeCategory::Diminutive => -4,
            SizeCategory::Tiny => -2,
            SizeCategory::Small => -1,
            SizeCategory::Medium => 0,
            SizeCategory::Large => 1,
            SizeCategory::Huge => 2,
            SizeCategory::Gargantuan => 4,
            SizeCategory::Colossal => 8,
        }
    }

    /// This size's racial modifier to **Stealth** checks.
    ///
    /// PF1 Core Rulebook Table 8-1, column *Size Modifier to Stealth*. Note
    /// that this scales by 4 per size *category*, which is a different
    /// progression from [`armor_class_size_modifier`](Self::armor_class_size_modifier)
    /// -- it is not four times that column (Fine is +16, not +32):
    ///
    /// ```text
    /// Fine +16  Diminutive +12  Tiny +8   Small +4   Medium +0
    /// Large -4  Huge -8         Gargantuan -12       Colossal -16
    /// ```
    ///
    /// **Not applied by any consumer yet.** It is transcribed here with its
    /// siblings because splitting one published table across two cycles is
    /// exactly how the partial-transcription errors this module's own header
    /// warns about get made, and because `skill_allocation`'s Stealth total
    /// is the next thing that will need it.
    pub fn stealth_size_modifier(self) -> i16 {
        match self {
            SizeCategory::Fine => 16,
            SizeCategory::Diminutive => 12,
            SizeCategory::Tiny => 8,
            SizeCategory::Small => 4,
            SizeCategory::Medium => 0,
            SizeCategory::Large => -4,
            SizeCategory::Huge => -8,
            SizeCategory::Gargantuan => -12,
            SizeCategory::Colossal => -16,
        }
    }

    /// Parses a race record's `FACT:BaseSize|<code>` payload.
    ///
    /// Codes are PCGen's own single letters. `None` for anything else,
    /// rather than a guessed default -- a caller that cannot determine a
    /// creature's size must decide for itself what to do about that, and
    /// say so, instead of silently receiving Medium.
    pub fn from_base_size_code(code: &str) -> Option<SizeCategory> {
        match code.trim() {
            "F" => Some(SizeCategory::Fine),
            "D" => Some(SizeCategory::Diminutive),
            "T" => Some(SizeCategory::Tiny),
            "S" => Some(SizeCategory::Small),
            "M" => Some(SizeCategory::Medium),
            "L" => Some(SizeCategory::Large),
            "H" => Some(SizeCategory::Huge),
            "G" => Some(SizeCategory::Gargantuan),
            "C" => Some(SizeCategory::Colossal),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every `SIZEMULT:` row of `load.lst`, as the decimal the file
    /// actually prints, cross-checked against the exact rational this
    /// module stores. Guards the transcription in the same direction the
    /// file itself reads, so a swapped numerator/denominator cannot pass.
    #[test]
    fn load_capacity_ratios_evaluate_to_the_decimals_printed_in_load_lst() {
        let expected = [
            (SizeCategory::Fine, 0.125),
            (SizeCategory::Diminutive, 0.25),
            (SizeCategory::Tiny, 0.5),
            (SizeCategory::Small, 0.75),
            (SizeCategory::Medium, 1.0),
            (SizeCategory::Large, 2.0),
            (SizeCategory::Huge, 4.0),
            (SizeCategory::Gargantuan, 8.0),
            (SizeCategory::Colossal, 16.0),
        ];
        for (size, decimal) in expected {
            let (numerator, denominator) = size.load_capacity_ratio();
            assert_eq!(
                numerator as f64 / denominator as f64,
                decimal,
                "{size:?} must equal load.lst's own SIZEMULT decimal"
            );
            assert!(denominator > 0, "{size:?} must not carry a zero/negative denominator");
        }
    }

    /// PF1 Core Rulebook Table 8-1 transcribed a second time, as one literal
    /// row per size, and checked against all three accessors at once. Written
    /// as a whole-table literal rather than three spot checks so a swapped
    /// pair of rows cannot pass, and so the two columns that are exact
    /// negations of each other are each pinned against the *book* rather than
    /// against each other.
    #[test]
    fn size_modifiers_match_table_8_1_row_for_row() {
        // (size, AC/attack modifier, special size modifier, Stealth modifier)
        let table_8_1 = [
            (SizeCategory::Fine, 8, -8, 16),
            (SizeCategory::Diminutive, 4, -4, 12),
            (SizeCategory::Tiny, 2, -2, 8),
            (SizeCategory::Small, 1, -1, 4),
            (SizeCategory::Medium, 0, 0, 0),
            (SizeCategory::Large, -1, 1, -4),
            (SizeCategory::Huge, -2, 2, -8),
            (SizeCategory::Gargantuan, -4, 4, -12),
            (SizeCategory::Colossal, -8, 8, -16),
        ];
        for (size, armor_class, special, stealth) in table_8_1 {
            assert_eq!(
                size.armor_class_size_modifier(),
                armor_class,
                "{size:?} Armor Class size modifier must match Table 8-1"
            );
            assert_eq!(
                size.special_size_modifier(),
                special,
                "{size:?} special size modifier (CMB/CMD) must match the published value"
            );
            assert_eq!(
                size.stealth_size_modifier(),
                stealth,
                "{size:?} Stealth size modifier must match Table 8-1"
            );
        }
    }

    /// Medium is PF1's unmodified baseline, and every value this repo ships
    /// today was computed as though every creature were Medium. If Medium
    /// ever stops being zero on all three columns, every previously-correct
    /// Medium character's sheet silently changes -- so it gets its own
    /// named guard rather than living only inside the table above.
    #[test]
    fn medium_is_zero_on_every_size_modifier_column() {
        assert_eq!(SizeCategory::Medium.armor_class_size_modifier(), 0);
        assert_eq!(SizeCategory::Medium.special_size_modifier(), 0);
        assert_eq!(SizeCategory::Medium.stealth_size_modifier(), 0);
    }

    /// The two directions are opposite, and that is the whole point of the
    /// "special" size modifier: a Small creature is harder to hit (+1 AC) and
    /// worse at combat maneuvers (-1 CMB/CMD). Pinning the relationship
    /// catches a copy-paste that gave CMB the Armor Class column's sign.
    #[test]
    fn the_special_size_modifier_runs_opposite_to_the_armor_class_one() {
        for size in [
            SizeCategory::Fine,
            SizeCategory::Diminutive,
            SizeCategory::Tiny,
            SizeCategory::Small,
            SizeCategory::Medium,
            SizeCategory::Large,
            SizeCategory::Huge,
            SizeCategory::Gargantuan,
            SizeCategory::Colossal,
        ] {
            assert_eq!(
                size.special_size_modifier(),
                -size.armor_class_size_modifier(),
                "{size:?}'s special size modifier must be the opposite of its AC one"
            );
        }
    }

    #[test]
    fn base_size_codes_round_trip_the_corpus_token_payloads() {
        // The two codes the seven playable races actually carry.
        assert_eq!(SizeCategory::from_base_size_code("M"), Some(SizeCategory::Medium));
        assert_eq!(SizeCategory::from_base_size_code("S"), Some(SizeCategory::Small));
        // Full code set is parseable, matching load_capacity_ratio's coverage.
        assert_eq!(SizeCategory::from_base_size_code("F"), Some(SizeCategory::Fine));
        assert_eq!(SizeCategory::from_base_size_code("C"), Some(SizeCategory::Colossal));
    }

    /// An unrecognized payload must not silently become Medium -- that is
    /// exactly the failure mode this module was created to remove.
    #[test]
    fn an_unknown_base_size_code_is_none_rather_than_a_defaulted_medium() {
        assert_eq!(SizeCategory::from_base_size_code("MEDIUM"), None);
        assert_eq!(SizeCategory::from_base_size_code(""), None);
        assert_eq!(SizeCategory::from_base_size_code("X"), None);
    }
}
