//! SD-36 Epic F3b2b (1): which formula identifiers the pinned oracle can read as something other
//! than a data variable.
//!
//! PCGen resolves a formula term in three steps (`VariableProcessor.lookupVariable`,
//! `code/src/java/pcgen/core/VariableProcessor.java:532-561` in the pinned checkout):
//!
//! 1. a data variable (`pc.hasVariable`, i.e. some loaded row `DEFINE`s it);
//! 2. an internal term (`VariableProcessorPC.getInternalVariable` ->
//!    `EvaluatorFactory.getTermEvaluator`): one case-sensitive regex anchored at the start of the
//!    term, built from every `TermEvaluatorBuilderPCVar` / `TermEvaluatorBuilderEQVar` pattern
//!    plus the loaded stat keys (`EvaluatorFactory.java:54` `"^("`, `:78` `Pattern.compile` with
//!    no flags, `:123-133`: an entire-term builder must match the whole term, the others match a
//!    prefix);
//! 3. an output (export) token (`VariableProcessor.getExportVariable` ->
//!    `ExportHandler.replaceToken`, `ExportHandler.java:1576-1645`): the text before the first
//!    `.` or `,` looked up exactly in `TOKEN_MAP`, whose names are all upper case.
//!
//! When none of the three answers, the term's text is parsed as a number and a failure is
//! "just zero" (`VariableProcessor.java:394-402`: `valFloat = 0.0f` ... `// Don't care, as it's
//! just zero`). So a name that no row of the loaded tree declares and that is not a built-in term
//! evaluates as 0 in the oracle.
//!
//! [`undeclared_reads_as_zero`] and [`may_be_builtin_term`] are the conservative side of that test: `true` for every identifier
//! step 2 or step 3 could answer. It is `false` only when the identifier's first `.`/`,` segment
//! carries a lower-case letter (so no upper-case export token and no entire-term builder can
//! match it) AND it starts with none of the prefix-matching builders' literal prefixes. A `true`
//! never reads as 0: the converter keeps such a name a closure defect.

/// The literal prefixes of the prefix-matching (not entire-term) builders of the pinned
/// `TermEvaluatorBuilderPCVar` (`BL`, `CL`, `CL;BEFORELEVEL`, `CLASS`, `CLASSLEVEL`, `COUNT[...`,
/// `EQTYPE`, `HASDEITY:`, `HASFEAT:`, `MODEQUIP`, `MOVE[`, `PC.HEIGHT`, `PC.WEIGHT`, `PC.SIZE`,
/// `SKILLRANK`, `SKILLTOTAL`, `VARDEFINED:`, `WEIGHT.`), plus the Pathfinder stat keys
/// (`makeStatBuilder`, a prefix builder). `CL` covers `CL;`, `CLASS` and `CLASSLEVEL`; `PC.`
/// covers the three `PC.` builders.
const PREFIX_BUILDERS: [&str; 19] = [
    "BL", "CL", "COUNT[", "EQTYPE", "HASDEITY:", "HASFEAT:", "MODEQUIP", "MOVE[", "PC.", "SKILLRANK", "SKILLTOTAL", "VARDEFINED:", "WEIGHT.", "STR", "DEX", "CON",
    "INT", "WIS", "CHA",
];

/// `true` when `name`, declared by no row of the pinned tree, provably evaluates as 0 in the
/// oracle: it is one plain identifier (`[A-Za-z_][A-Za-z0-9_]*`; anything else is a formula the
/// converter's lexer did not split, e.g. `RagePowersLVL%2`, and is not judged here) and not
/// [`may_be_builtin_term`].
pub fn undeclared_reads_as_zero(name: &str) -> bool {
    let name = name.trim();
    let mut chars = name.chars();
    let plain = chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') && chars.all(|c| c.is_ascii_alphanumeric() || c == '_');
    plain && !may_be_builtin_term(name)
}

/// `true` when the pinned oracle could read `name` as a built-in term or an output token (steps 2
/// and 3 above); `false` only when it provably cannot.
pub fn may_be_builtin_term(name: &str) -> bool {
    let name = name.trim();
    let first = name.split(['.', ',']).next().unwrap_or(name);
    if !first.chars().any(|c| c.is_ascii_lowercase()) {
        return true;
    }
    PREFIX_BUILDERS.iter().any(|p| name.starts_with(p))
}

#[cfg(test)]
mod tests {
    use super::{may_be_builtin_term, undeclared_reads_as_zero};

    #[test]
    fn only_a_plain_non_builtin_identifier_reads_as_zero() {
        assert!(undeclared_reads_as_zero("SecretLore"));
        assert!(!undeclared_reads_as_zero("RagePowersLVL%2"), "an unsplit formula is not judged");
        assert!(!undeclared_reads_as_zero("CRITMULT"));
        assert!(!undeclared_reads_as_zero(""));
    }

    #[test]
    fn a_mixed_case_data_name_is_not_a_builtin_term() {
        for n in ["CasterLevel_Highest", "SecretLore", "MetaforgedLVL", "MysticTheurgeLVL", "PaDTrueSeeingLvl", "NotAllowed", "IsProfane"] {
            assert!(!may_be_builtin_term(n), "{n}");
        }
    }

    #[test]
    fn an_upper_case_or_prefix_matched_name_may_be_builtin() {
        for n in ["CASTERLEVEL", "CRITMULT", "SHIELDACCHECK", "SR", "DAMAGE.Melee", "BASE.Fortitude", "CLevel", "STRBonus", "COUNT[FEATS]", "PC.HEIGHT", "LIST"] {
            assert!(may_be_builtin_term(n), "{n}");
        }
    }
}
