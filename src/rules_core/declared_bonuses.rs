//! The **settled** result of reading one ingested row's bonus chains.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 14, under `decisions.md` §11 ("no PCGen in
//! live code") and `technical-design.md` §0 (the boundary is **by path**:
//! `src/pcgen_import/**` may read PCGen, `src/rules_core/**` may not).
//!
//! # Why these three structs live here and not beside the reader
//!
//! Every field below is a **narrowed, already-classified value** — a
//! `Vec<i32>` of magnitudes, a `u8` pick count, a typed adjustment row. None
//! of them names a qualifier position, a chain keyword (`VAR`, `STAT`,
//! `ABILITYPOOL`, `TYPE=Boolean`), or an ingest field. They were already the
//! *answer* rather than the *grammar* when
//! [`crate::pcgen_import::bonus_chain_reader`] defined them in SD-35
//! `AT-35-E6-002` cycle 5 — but because they were defined **inside**
//! `src/pcgen_import/`, a live module could not hold one without writing
//! `pcgen_import` in its own source, which ruling B16 (`decisions.md` §19)
//! correctly counts as a read of the converter.
//!
//! So the types moved to the live side and the **reader stayed on the
//! converter side**: `bonus_chain_reader::declared_bonuses` still does every
//! bit of the chain walking and still re-exports these names for the
//! converter's own call sites. Not one line of reading logic changed, and the
//! values are field-for-field identical — this is where the answer is
//! *declared*, never where it is *derived*.
//!
//! This is exactly the shape cycle 13 used for
//! [`CorpusEquipmentRecord`](crate::rules_core::equipment_record::CorpusEquipmentRecord):
//! the settled type is live, the function that fills it is not.

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

/// One bonus a chain states against a **named target** — `BONUS:SAVE|Will|2`,
/// `BONUS:SKILL|Swim|4`.
///
/// `keyword` is the chain's own leading qualifier (`"SAVE"`, `"SKILL"`, …) and
/// `target` the thing it names, both verbatim. `magnitude` is `Some` only when
/// the row states a plain integer: a formula, a variable or an upstream
/// `%LIST` placeholder is out of reach by `decisions.md §24` and lands as
/// `None` rather than being guessed at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetBonus {
    pub keyword: String,
    pub target: String,
    pub magnitude: Option<i32>,
}

/// Everything a live module needs from one row's bonus chains, read once.
///
/// Built at ingest-boundary time and stored, so nothing downstream holds the
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
    /// Every bonus the row states against a named target, in source order —
    /// the `BONUS:SAVE|<save>|<n>` / `BONUS:SKILL|<skill>|<n>` shape and its
    /// siblings. Added by SD-35 `AT-35-E6-003-RULED` cycle 14 so a caller can
    /// ask *what does this row bonus, and by how much?* without walking the
    /// ingest chain array to ask it.
    pub target_bonuses: Vec<TargetBonus>,
}
