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
//! # Deliberately minimal
//!
//! Per `docs/governance/no-stub-mvp-doctrine.md`, this is a size model
//! sized to its one real consumer -- carrying capacity -- not a general
//! size subsystem. PF1 also scales weapon damage dice, AC, attack rolls,
//! CMB/CMD, Fly checks, and Stealth by size; **none of that is applied
//! anywhere**, and this module deliberately offers no helper that would
//! make it look available. Each of those needs its own corpus verification
//! and each would change shipped numbers, so they are named as deferred
//! rather than half-built.
//!
//! # Sources
//!
//! Two different corpus facts meet here, and they come from two different
//! files -- neither is inferred:
//!
//!  - **Which size a creature is**: `FACT:BaseSize|<code>` on the creature's
//!    own race record. See `rules_tables::crb::race_tables::race_size`.
//!  - **What that size multiplies capacity by**: `SIZEMULT:<code>|<value>`
//!    in the PCGen Pathfinder game mode's
//!    `/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`
//!    -- the same file `encumbrance.rs`'s `LOAD:` table comes from.
//!    Transcribed in `SizeCategory::load_capacity_ratio` below.

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
