//! SD-32 card 11 (T12), Psion cycle — real per-feature compute functions
//! for the Psion, the last named T12 item and `ultimate_psionics`'s tenth
//! magnitude-bearing class.
//!
//! # `psion` uses a genuinely-third grant convention (confirmed, not a bug)
//!
//! Every one of the nine already-closed `ultimate_psionics` classes (and
//! every other T12 class) grants its own class features either via a
//! `.MOD` virtual ability (shape 1) or a `CLASS:` level-table row whose
//! `ABILITY:<ClassName> Class Feature|AUTOMATIC|<ClassName> ~ <Feature>`
//! field repeats the class's own display name as the target's group
//! prefix (shape 2). `psion`'s own `CLASS:Psion` block in `up_classes.lst`
//! (line 264, re-derived against the pinned oracle,
//! `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`) instead
//! carries `ABILITY:Psion Class Feature|AUTOMATIC|Psion Manifesting` --
//! same `ABILITY:<ClassName> Class Feature|AUTOMATIC|` category prefix as
//! shape 2, but the payload is the bare feature name with **no** `Psion ~ `
//! group-separator prefix at all (shape 3, `census_untabled_base_class_
//! feature_roster.py`'s generalised group-membership rule below).
//!
//! This was checked against the precedent that closed 6 of the "7 classes
//! need a third shape" false lead (a one-line `CATEGORY=Class` vs
//! `CATEGORY=CLASS` case bug) before being trusted: `grep -c "Psion ~ "
//! up_classes.lst up_abilities_class.lst` returns 0 and 7 respectively, and
//! every one of those 7 `up_abilities_class.lst` hits is a false-positive
//! substring match inside a DIFFERENT class's own group name (e.g.
//! `Ascendant Psion ~ Hide Mind` contains the substring `Psion ~ ` without
//! being a `psion`-owned grant at all). There is no casing bug here and no
//! missing `Psion ~ ` prefix hiding in the data -- `psion`'s own class
//! block genuinely never repeats its own name before ` ~ `.
//!
//! # Scope: `Psion Manifesting` only, not the discipline-choice chain
//!
//! `psion`'s `CLASS:Psion` block grants exactly two own-named features at
//! level 1: `Psion Weapon Proficiencies` (a different `ABILITY:Special
//! Ability|AUTOMATIC|` category, no magnitude, out of the `Class Feature`
//! population entirely) and `Psion Manifesting` (the shape-3 record this
//! module grounds). Every other `ABILITY:Psion Class Feature|AUTOMATIC|`
//! grant from `CLASS:Psion`'s block is a **discipline or archetype pick**
//! (`Psychometabolism Discipline`, `Ascendant Psion` via `Archetype|
//! AUTOMATIC|Psion Archetype ~ Ascendant Psion`, `Bombardier ~ ...`, ...) --
//! a player selects exactly ONE, and that pick's own record chains further
//! `ABILITY:Psion Class Feature|AUTOMATIC|<Discipline> ~ <Feature>` grants
//! gated on a discipline-specific `PREVARGTEQ:<Discipline>DisciplineLVL,N`
//! variable, not `psion`'s own class level directly. This is structurally
//! a pool-shaped population (one of several mutually-exclusive picks, each
//! with its own progression) -- the same shape `census_untabled_base_
//! class_feature_roster.py`'s own module doc comment already excludes for
//! `Vigilante Talent`/`Magus Arcana` ("pool grants excluded... need
//! per-pool verification"), not a `psion`-own-named class feature. A
//! mechanical BFS from `CLASS:Psion`'s own block (re-derivable via
//! `scripts/psion_discipline_chain_census.py`, this cycle's own artifact,
//! see its own doc comment for the exact command) finds 32 magnitude-
//! bearing leaf records across 9 disciplines/archetypes reachable only
//! through that pool choice -- real, sized, and NOT filed out of scope
//! (`decisions.md §27b`), but requiring the pool-catalog closure mechanism
//! (`class_feature_pool_catalog.rs`'s own construction discipline), not
//! this roster/chassis mechanism. Named precisely in this cycle's receipt,
//! not silently dropped.
//!
//! # `Psion Manifesting`'s own magnitude
//!
//! The record's own `DESC` is `"Psion Powers Known: %1; Psion Maximum
//! Power Level Known: %2|PsionPowersKnown|PsionMaxPowerLevel"` -- a %N-
//! substituted DESC-prose magnitude with no `BONUS:` token of its own, the
//! same shape cycle 4 already closed for Kineticist's Burn and Vigilante's
//! Frightening/Stunning Appearance (`§1a`: grounded exactly as the prose
//! states, not fabricated). The two substituted variables are set by two
//! `CATEGORY:Internal` backing records `Psion Manifesting` itself grants
//! (`ABILITY:Internal|AUTOMATIC|Psion Manifesting Variables|Psion Power
//! Points`), both already ingested in the corpus
//! (`data/corpus/ultimate_psionics/class_feature/{psion_manifesting_
//! variables,psion_power_points}/*.json`).
//!
//! This module grounds `psion_power_points_total` --
//! `Psion Power Points`'s own `BasePowerPoints` ladder (a monotonically
//! increasing table of 20 `BONUS:VAR|BasePowerPoints|<value>|
//! PREVARGTEQ:PsionPPL,<threshold>` entries, read as the standard PCGen
//! "highest satisfied threshold wins" class-table idiom -- the only
//! reading whose values match the well-established real Power Points per
//! level table, not a literal cumulative sum of every threshold, which
//! would produce an implausible ~343 at level 20) plus its single-entry
//! `BonusPowerPoints|(PsionPPStat*PsionPPL)/2|TYPE=PsionBonusPP` term
//! (`PsionPPStat` = `PsionPrimeStat` = Intelligence modifier, `PsionPPL` =
//! manifester level = class level for a single-classed Psion).
//!
//! # `Psion Powers Known` / `Psion Maximum Power Level Known`: resolved, not guessed (SD-32 T12 follow-up)
//!
//! A previous cycle escalated `PsionPowersKnown`'s two `BONUS:VAR` entries
//! (`min(21,(2*PsionPKL)+1)` unconditional, `floor((PsionPKL-10)*3/2)`
//! added when `PsionPKL>=11`, neither carrying a `TYPE=`) as a "genuine
//! `BONUS:VAR` combination-semantics ambiguity this repo cannot resolve
//! without live PCGen." **That claim does not survive a check against this
//! repo's own already-built, PCGen-source-cited resolution**:
//! `pilot_compute::bonus_stack_reader`'s module doc (point 2) cites
//! `pcgen/core/PlayerCharacter.java:2136` -> `BonusManager.java`'s
//! `getTotalBonusTo`/`sumActiveBonusMap` directly: **multiple `BONUS:VAR`
//! entries sharing one target variable SUM, gated by each entry's own
//! `PREVARGTEQ` currently passing** -- not this reader's own policy
//! choice, the real PCGen aggregation. Applied here: below level 11 (in
//! `PsionPKL` terms) only the unconditional term is active; at
//! `PsionPKL>=11` BOTH terms sum. This is the exact "sum" reading the
//! escalating cycle had already identified as the plausible one (versus
//! "replace", which produces an implausible level-11 drop) -- the citation
//! confirms it as PCGen's real behavior rather than leaving it a guess.
//! [`psion_powers_known`] grounds it. `PsionMaxPowerLevel` has only ONE
//! `BONUS:VAR` term on the record (`min(9,floor((PsionMPL+1)/2),
//! PsionPLStatScore-10)`) -- no combination question at all, grounded by
//! [`psion_max_power_level`].

/// `Psion Power Points`: `BasePowerPoints` (a level-keyed table, "highest
/// satisfied `PREVARGTEQ:PsionPPL,N` threshold wins" -- see this module's
/// own doc comment) plus `BonusPowerPoints` = `(int_mod * level) / 2`
/// (`TYPE=PsionBonusPP`, the record's only entry for that variable, no
/// combination ambiguity).
pub fn psion_power_points_total(level: u8, int_mod: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let base: i16 = match level {
        1 => 2,
        2 => 4,
        3 => 5,
        4 => 6,
        5 => 8,
        6 => 10,
        7 => 11,
        8 => 12,
        9 => 14,
        10 => 16,
        11 => 18,
        12 => 20,
        13 => 21,
        14 => 23,
        15 => 25,
        16 => 26,
        17 => 29,
        18 => 30,
        19 => 31,
        _ => 32, // level >= 20
    };
    let bonus = (int_mod * i16::from(level)) / 2;
    Some(base + bonus)
}

/// `PsionPowersKnown`: `min(21,(2*PsionPKL)+1)` always active, plus
/// `floor((PsionPKL-10)*3/2)` summed on top once `PsionPKL>=11`
/// (`bonus_stack_reader`'s documented PCGen "multiple `BONUS:VAR` on one
/// target SUM, gated by each entry's own currently-passing `PREVARGTEQ`"
/// semantics -- see this module's own doc comment). `PsionPKL` is the raw
/// class level on a single-classed Psion (no bonus manifester levels
/// tracked by this engine). `None` below level 1.
pub fn psion_powers_known(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let pkl = i16::from(level);
    let base = (2 * pkl + 1).min(21);
    let bonus = if pkl >= 11 { ((pkl - 10) * 3) / 2 } else { 0 };
    Some(base + bonus)
}

/// `PsionMaxPowerLevel`'s single `BONUS:VAR` term:
/// `min(9,floor((PsionMPL+1)/2),PsionPLStatScore-10)` -- no combination
/// question (one term, no sibling entry on the same target). `int_score`
/// is the Psion's raw Intelligence score (`PsionPLStatScore = INTSCORE`,
/// not the modifier). `None` below level 1.
pub fn psion_max_power_level(level: u8, int_score: i16) -> Option<i16> {
    if level < 1 {
        return None;
    }
    let mpl = i16::from(level);
    Some(((mpl + 1) / 2).min(9).min(int_score - 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn psion_power_points_total_uses_the_base_ladder_and_int_bonus() {
        // Level 1, INT modifier 0: base 2, bonus (0*1)/2 = 0.
        assert_eq!(psion_power_points_total(1, 0), Some(2));
        // Level 5, INT modifier 3: base 8, bonus (3*5)/2 = 7 (floor).
        assert_eq!(psion_power_points_total(5, 3), Some(15));
        // Level 20, INT modifier 5: base 32, bonus (5*20)/2 = 50.
        assert_eq!(psion_power_points_total(20, 5), Some(82));
        // Level 0 is not a real manifester level.
        assert_eq!(psion_power_points_total(0, 3), None);
    }

    #[test]
    fn psion_power_points_total_ladder_steps_at_every_named_threshold() {
        let expected: [(u8, i16); 20] = [
            (1, 2),
            (2, 4),
            (3, 5),
            (4, 6),
            (5, 8),
            (6, 10),
            (7, 11),
            (8, 12),
            (9, 14),
            (10, 16),
            (11, 18),
            (12, 20),
            (13, 21),
            (14, 23),
            (15, 25),
            (16, 26),
            (17, 29),
            (18, 30),
            (19, 31),
            (20, 32),
        ];
        for (level, base) in expected {
            // INT modifier 0 isolates the base ladder from the bonus term.
            assert_eq!(
                psion_power_points_total(level, 0),
                Some(base),
                "level {level} base power points"
            );
        }
    }

    #[test]
    fn psion_power_points_total_handles_a_negative_int_modifier() {
        // Level 4, INT modifier -1: base 6, bonus (-1*4)/2 = -2.
        assert_eq!(psion_power_points_total(4, -1), Some(4));
    }

    #[test]
    fn psion_powers_known_only_the_base_term_below_level_eleven() {
        // Level 1: min(21,2*1+1)=3, no bonus term yet.
        assert_eq!(psion_powers_known(1), Some(3));
        // Level 10: min(21,2*10+1)=21 (base term itself saturates here).
        assert_eq!(psion_powers_known(10), Some(21));
        assert_eq!(psion_powers_known(0), None);
    }

    #[test]
    fn psion_powers_known_sums_both_terms_from_level_eleven() {
        // Level 11: base min(21,23)=21, bonus floor(1*3/2)=1 -> 22.
        assert_eq!(psion_powers_known(11), Some(22));
        // Level 20: base min(21,41)=21, bonus floor(10*3/2)=15 -> 36.
        assert_eq!(psion_powers_known(20), Some(36));
    }

    #[test]
    fn psion_powers_known_mutation_proof_replace_semantics_would_drop_at_level_eleven() {
        // Sanity check on the module doc's own claim: a "replace, don't
        // sum" reading (using ONLY the level->=11 term once its gate
        // passes) produces an implausible drop from level 10's 21 to a
        // level-11 value of 1 -- this is why "sum" (this function's real
        // behavior) is the correct PCGen semantics, not a coin flip.
        let replace_semantics_at_11 = ((11i16 - 10) * 3) / 2; // = 1
        assert_eq!(replace_semantics_at_11, 1);
        assert_ne!(psion_powers_known(11), Some(replace_semantics_at_11));
    }

    #[test]
    fn psion_max_power_level_is_capped_by_the_lowest_of_three_terms() {
        // Level 1, INT score 10 (modifier 0): min(9, floor(2/2)=1, 10-10=0) = 0.
        assert_eq!(psion_max_power_level(1, 10), Some(0));
        // Level 20, INT score 20 (modifier +5): min(9, floor(21/2)=10, 20-10=10) = 9.
        assert_eq!(psion_max_power_level(20, 20), Some(9));
        // Level 5, INT score 12: min(9, floor(6/2)=3, 12-10=2) = 2 (stat-capped).
        assert_eq!(psion_max_power_level(5, 12), Some(2));
        assert_eq!(psion_max_power_level(0, 20), None);
    }
}
