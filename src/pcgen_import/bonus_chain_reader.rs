//! Tool-side readers for the ingest cache records' *bonus-chain* array.
//!
//! SD-35 `AT-35-E6-002` cycle 5, under `decisions.md` §11 ("no PCGen in live
//! code") and `technical-design.md` §0 (the boundary is **by path**:
//! `src/pcgen_import/**` may read PCGen, `src/rules_core/**` may not).
//!
//! Sibling of [`race_trait_tokens`](crate::pcgen_import::race_trait_tokens),
//! which reads a typed record's *token* array. A corpus record carries the
//! `.lst` row's `BONUS:` clauses in a **second** array — `raw_bonus_chains` —
//! and cycle 4 found that array still being walked in eleven live
//! `src/rules_core/` files and in the desktop crate, invisible to cycles 1-3
//! because the residue gate's pattern list named only the token array
//! (`correction 1789082356496-at-35-e6-002-65186a`). This module is the
//! converter-side home for every one of those walks.
//!
//! What the live side gets back is **narrowed, already-classified values** —
//! `Vec<i32>` of magnitudes, a `u8` pick count, typed adjustment rows — not
//! the chains under another name. After this module no live module names a
//! qualifier position, a chain keyword (`VAR`, `STAT`, `ABILITYPOOL`,
//! `TYPE=Boolean`), or the ingest field itself.
//!
//! Every function here is a **reading**, not an interpretation
//! (`decisions.md §24`): it transcribes what one row states and yields
//! nothing where the row states nothing, never a guess. The doc comments that
//! justified each reading moved here with it, verbatim.
//!
//! KEPT for Starfinder, like the rest of `src/pcgen_import/`.

use crate::pcgen_import::ingest_payload::{RaceTraitCacheData, RawBonusChain};

/// A record deserialized from a corpus file that still carries the `BONUS:`
/// chains of the `.lst` row it was ingested from.
///
/// Implemented here rather than beside the structs so the field read stays on
/// the tool side of `technical-design.md` §0's path boundary.
pub trait IngestBonusChains {
    /// The row's chains, in file order.
    fn ingest_bonus_chains(&self) -> &[RawBonusChain];
}

impl IngestBonusChains for RaceTraitCacheData {
    fn ingest_bonus_chains(&self) -> &[RawBonusChain] {
        &self.raw_bonus_chains
    }
}

/// One fixed ability-score adjustment a row declares, transcribed verbatim.
///
/// `codes` is the comma-separated ability-code list exactly as the row spells
/// it (Goblin's `STR,CHA`), `magnitude` its unparsed text. Both are `Option`
/// because a malformed row states neither, and the caller — which knows the
/// row's key and can name it — is the one that must decide whether that is an
/// error. Parsing and code-to-ability mapping are game semantics and stay on
/// the live side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityAdjustment {
    pub codes: Option<String>,
    pub magnitude: Option<String>,
}

/// One same-row variable contribution a chain states.
///
/// `amount` is `None` when the contribution cannot be finished from this row
/// alone — a conditional chain carrying a trailing prerequisite qualifier, or
/// an amount that is not a bare integer. `None` is "declared but
/// unresolvable", which is a different fact from "never mentioned" (a name
/// that never appears here at all).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarContribution {
    pub name: String,
    pub amount: Option<i64>,
}

/// Everything a live module needs from one row's bonus chains, read once.
///
/// Built at resolution time and stored, so nothing downstream holds the
/// ingest array. Every field is a transcription of what the row states.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DeclaredBonuses {
    /// Every integer that appears as a bare numeric qualifier, in source
    /// order, deduplicated.
    ///
    /// This is a *reading*, not an interpretation: it does not decide what the
    /// number bonuses, does not sum anything, and does not resolve variables.
    /// A chain that names a variable instead of stating a number contributes
    /// nothing; its companion chain stating the number contributes that.
    pub magnitudes: Vec<i32>,
    /// The same reading, over the chains that actually state a game quantity —
    /// chains that only write an internal state flag are discarded first.
    pub magnitudes_excluding_flags: Vec<i32>,
    /// True when the row declares at least one chain and *every* one of them
    /// only writes an internal state flag, so the row states no quantity of
    /// its own at all.
    pub only_internal_flags: bool,
    /// The fixed ability-score adjustments the row declares, in source order.
    pub ability_adjustments: Vec<AbilityAdjustment>,
    /// How many freely-distributed ability picks the row grants.
    pub ability_pool_picks: u8,
    /// Every same-row variable contribution, one entry per named variable per
    /// chain, in source order.
    pub var_contributions: Vec<VarContribution>,
}

/// [`DeclaredBonuses`] for a typed cache record.
pub fn declared_bonuses<T: IngestBonusChains>(data: &T) -> DeclaredBonuses {
    declared_bonuses_from_chains(data.ingest_bonus_chains())
}

/// [`DeclaredBonuses`] for a chain list held directly.
pub fn declared_bonuses_from_chains(chains: &[RawBonusChain]) -> DeclaredBonuses {
    DeclaredBonuses {
        magnitudes: bare_magnitudes(chains.iter()),
        magnitudes_excluding_flags: bare_magnitudes(
            chains.iter().filter(|chain| !is_internal_flag_chain(chain)),
        ),
        only_internal_flags: !chains.is_empty() && chains.iter().all(is_internal_flag_chain),
        ability_adjustments: ability_adjustments(chains),
        ability_pool_picks: ability_pool_picks(chains),
        var_contributions: var_contributions(chains),
    }
}

fn bare_magnitudes<'a>(chains: impl Iterator<Item = &'a RawBonusChain>) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    for chain in chains {
        for qualifier in &chain.qualifiers {
            if let Ok(value) = qualifier.parse::<i32>()
                && !out.contains(&value)
            {
                out.push(value);
            }
        }
    }
    out
}

/// Reads `BONUS:STAT|<codes>|<magnitude>` chains only. `<codes>` is
/// comma-separated and frequently names more than one ability — Goblin's
/// `BONUS:STAT|STR,CHA|-2` grants **both** — so the whole list is handed back
/// for the caller to credit code by code.
fn ability_adjustments(chains: &[RawBonusChain]) -> Vec<AbilityAdjustment> {
    chains
        .iter()
        .filter(|chain| chain.qualifiers.first().map(String::as_str) == Some("STAT"))
        .map(|chain| AbilityAdjustment {
            codes: chain.qualifiers.get(1).cloned(),
            magnitude: chain.qualifiers.get(2).cloned(),
        })
        .collect()
}

/// The *number of picks* a row grants is machine-readable
/// (`BONUS:ABILITYPOOL|Ability Bonus|1`); the *magnitude per pick* appears only
/// in the row's own display name, which is the live side's business to read.
fn ability_pool_picks(chains: &[RawBonusChain]) -> u8 {
    chains
        .iter()
        .filter(|chain| {
            chain.qualifiers.first().map(String::as_str) == Some("ABILITYPOOL")
                && chain.qualifiers.get(1).map(String::as_str) == Some("Ability Bonus")
        })
        .map(|chain| chain.qualifiers.get(2).and_then(|n| n.parse::<u8>().ok()).unwrap_or(0))
        .sum()
}

/// A variable stops resolving the instant any contribution stops being a
/// same-row literal — a conditional chain carrying a trailing prerequisite
/// qualifier, or an amount naming another variable. It is then reported as
/// `None` rather than guessed.
fn var_contributions(chains: &[RawBonusChain]) -> Vec<VarContribution> {
    let mut out: Vec<VarContribution> = Vec::new();
    for chain in chains {
        let quals = &chain.qualifiers;
        if !quals.first().is_some_and(|q| q.eq_ignore_ascii_case("VAR")) {
            continue;
        }
        let (Some(names), Some(amount)) = (quals.get(1), quals.get(2)) else { continue };
        let conditional = quals[3.min(quals.len())..]
            .iter()
            .any(|q| q.starts_with("PRE") || q.starts_with("!PRE"));
        let amount = if conditional { None } else { amount.trim().parse::<i64>().ok() };
        for name in names.split(',') {
            out.push(VarContribution { name: name.trim().to_owned(), amount });
        }
    }
    out
}

/// True when a variable name is one PCGen uses as an internal state flag
/// rather than as a quantity.
///
/// Deliberately narrow. `Bonus`-suffixed names, `…_Times` (uses per day),
/// `AC_Natural_Armor` and the rest are real quantities that a looser rule
/// would silently erase, which would be a worse defect than the one this
/// closes.
fn variable_name_is_flag_shaped(name: &str) -> bool {
    let name = name.trim();
    if name.is_empty() {
        return false;
    }
    if name.ends_with("Flag") || name.ends_with("ExoticUse") {
        return true;
    }
    ["Has", "Is"].iter().any(|prefix| {
        name.strip_prefix(prefix)
            .and_then(|rest| rest.chars().next())
            .is_some_and(|next| next.is_ascii_uppercase())
    })
}

/// True when this chain only writes an internal PCGen state flag and therefore
/// declares no game quantity at all.
///
/// Authoritative signal first: PCGen tags the chain itself `TYPE=Boolean`.
/// Three served rows carry it — `Drow ~ Light Blindness`
/// (`UMR_LightBlindness_SpecificDesc`), `Merfolk ~ Legless` (`CantBeTripped`)
/// and `Svirfneblin ~ Svirfneblin Magic` (`RacialSLA_Nondetection_Constant`) —
/// as do four ARG alternates. The vision traits are *not* tagged, hence the
/// name conventions in [`variable_name_is_flag_shaped`] as the second signal.
///
/// A chain naming several variables counts as a flag only when *every* name is
/// flag-shaped, so a mixed chain keeps its magnitude. Non-variable chains are
/// never flags: `Svirfneblin ~ Svirfneblin Magic`'s companion
/// `BONUS:DC|SCHOOL.Illusion|1` is a real +1 and survives.
fn is_internal_flag_chain(chain: &RawBonusChain) -> bool {
    if chain.qualifiers.first().map(String::as_str) != Some("VAR") {
        return false;
    }
    if chain.qualifiers.iter().any(|qualifier| qualifier == "TYPE=Boolean") {
        return true;
    }
    let Some(names) = chain.qualifiers.get(1) else {
        return false;
    };
    let mut named = names.split(',').filter(|name| !name.trim().is_empty()).peekable();
    named.peek().is_some() && named.all(variable_name_is_flag_shaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain(quals: &[&str]) -> RawBonusChain {
        RawBonusChain { qualifiers: quals.iter().map(|q| (*q).to_owned()).collect() }
    }

    #[test]
    fn magnitudes_are_read_in_source_order_and_deduplicated() {
        let chains = [chain(&["STAT", "STR,CHA", "-2"]), chain(&["STAT", "DEX", "2"]), chain(&["VAR", "X", "2"])];
        assert_eq!(declared_bonuses_from_chains(&chains).magnitudes, vec![-2, 2]);
    }

    #[test]
    fn a_chain_that_only_names_a_variable_states_no_magnitude() {
        let chains = [chain(&["SITUATION", "Perception=underground", "Dwarf_StoneCunning_SkillBonus"])];
        assert!(declared_bonuses_from_chains(&chains).magnitudes.is_empty());
    }

    #[test]
    fn ability_adjustments_keep_the_whole_code_list_unparsed() {
        let read = declared_bonuses_from_chains(&[chain(&["STAT", "STR,CHA", "-2"]), chain(&["DC", "SCHOOL.Illusion", "1"])]);
        assert_eq!(
            read.ability_adjustments,
            vec![AbilityAdjustment { codes: Some("STR,CHA".to_owned()), magnitude: Some("-2".to_owned()) }]
        );
    }

    #[test]
    fn a_malformed_ability_chain_reports_absence_rather_than_a_guess() {
        let read = declared_bonuses_from_chains(&[chain(&["STAT"])]);
        assert_eq!(read.ability_adjustments, vec![AbilityAdjustment { codes: None, magnitude: None }]);
    }

    #[test]
    fn ability_pool_picks_sum_only_the_ability_bonus_pool() {
        let chains =
            [chain(&["ABILITYPOOL", "Ability Bonus", "1"]), chain(&["ABILITYPOOL", "Feat Pool", "1"])];
        assert_eq!(declared_bonuses_from_chains(&chains).ability_pool_picks, 1);
    }

    #[test]
    fn a_conditional_variable_contribution_is_unresolvable_not_zero() {
        let plain = declared_bonuses_from_chains(&[chain(&["VAR", "A,B", " 2 "])]);
        assert_eq!(
            plain.var_contributions,
            vec![
                VarContribution { name: "A".to_owned(), amount: Some(2) },
                VarContribution { name: "B".to_owned(), amount: Some(2) },
            ]
        );
        let conditional = declared_bonuses_from_chains(&[chain(&["VAR", "A", "2", "PRECLASS:1,Fighter=1"])]);
        assert_eq!(conditional.var_contributions, vec![VarContribution { name: "A".to_owned(), amount: None }]);
    }

    #[test]
    fn an_internal_flag_chain_states_no_quantity_but_a_mixed_chain_keeps_its_magnitude() {
        let flag = [chain(&["VAR", "HasRacialVision", "1"])];
        let read = declared_bonuses_from_chains(&flag);
        assert!(read.only_internal_flags);
        assert_eq!(read.magnitudes, vec![1]);
        assert!(read.magnitudes_excluding_flags.is_empty());

        let tagged = [chain(&["VAR", "CantBeTripped", "1", "TYPE=Boolean"])];
        assert!(declared_bonuses_from_chains(&tagged).only_internal_flags);

        let mixed = [chain(&["VAR", "HasRacialVision,KeenSensesBonus", "2"])];
        let read = declared_bonuses_from_chains(&mixed);
        assert!(!read.only_internal_flags);
        assert_eq!(read.magnitudes_excluding_flags, vec![2]);
    }

    #[test]
    fn a_row_with_no_chains_declares_no_flags_rather_than_all_of_them() {
        let read = declared_bonuses_from_chains(&[]);
        assert!(!read.only_internal_flags);
        assert_eq!(read, DeclaredBonuses::default());
    }

    #[test]
    fn flag_shaping_is_narrow_enough_to_leave_real_quantities_alone() {
        for flag in ["HasRacialVision", "IsAquatic", "SomeThingFlag", "BastardSwordExoticUse"] {
            assert!(variable_name_is_flag_shaped(flag), "{flag}");
        }
        for quantity in ["Hasted_Bonus", "Island_Bonus", "KeenSensesBonus", "Orc_OrcFerocity_Times", "AC_Natural_Armor", ""] {
            assert!(!variable_name_is_flag_shaped(quantity), "{quantity}");
        }
    }
}
