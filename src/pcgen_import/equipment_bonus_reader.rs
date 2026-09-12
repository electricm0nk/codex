//! Tool-side readers for an equipment record's *bonus-type* qualifier.
//!
//! SD-35 `AT-35-E6-003-SWEEP` cycle 14, under `decisions.md` §11 ("no PCGen in
//! live code") and `technical-design.md` §0 (the boundary is **by path**:
//! `src/pcgen_import/**` may read PCGen, `src/rules_core/**` may not).
//!
//! Sibling of [`bonus_chain_reader`](crate::pcgen_import::bonus_chain_reader),
//! which reads a *race-trait* cache record's `raw_bonus_chains` array. This
//! module reads the same ingest concept off the **equipment** side, where the
//! chains arrive as [`BonusToken`]s on an
//! [`EquipmentRecord`](crate::pcgen_import::lst_parser::equipment::EquipmentRecord)
//! rather than as `RawBonusChain`s on a cache payload.
//!
//! # What this module is for, and what it is not
//!
//! It exists because two live `src/rules_core/equipment_effects/` functions
//! were classifying a bonus chain by comparing a qualifier to an ingest string
//! they held themselves:
//!
//! ```text
//! arms_armor.rs:142   && !qualifiers.iter().any(|q| q == "TYPE=Circumstance");
//! equipmods.rs:231    if qualifiers.len() >= 4 && qualifiers[3].eq_ignore_ascii_case("TYPE=Enhancement")
//! ```
//!
//! Each is a live module holding the ingest format's vocabulary, which is the
//! thing §11 forbids. What moved here is the **grammar**, not just the literal:
//! after this module neither call site names a qualifier position, the `TYPE=`
//! keyword, or a bonus-type spelling. Each gets back a `bool` that answers a
//! PF1 rules question — *is this a circumstance bonus?*, *is this an
//! enhancement bonus on a to-hit/damage roll?* — and the reasoning about the
//! ingest format's grammar lives on this side of the path boundary.
//!
//! Both functions are **readings**, not interpretations (`decisions.md` §24):
//! each transcribes the predicate its caller applied before the move, exactly,
//! including the two respects in which the two predicates differ from one
//! another and must keep differing. `bonus_type_qualifiers_are_unchanged_by_
//! this_module` in the tests below pins that against the live corpus rather
//! than against a hand-copied expectation.
//!
//! KEPT for Starfinder, like the rest of `src/pcgen_import/`.

use crate::pcgen_import::lst_parser::equipment::BonusToken;

/// PCGen's spelling of the PF1 *circumstance* bonus type, as a whole qualifier
/// segment.
///
/// Real corpus instance, and as of this cycle the only one carrying it on a
/// `COMBAT|AC` chain: `advanced_race_guide:equipment:sea_knife`'s
/// `BONUS:COMBAT|AC|-2|TYPE=Circumstance`.
const CIRCUMSTANCE_BONUS_TYPE: &str = "TYPE=Circumstance";

/// PCGen's spelling of the PF1 *enhancement* bonus type, as a whole qualifier
/// segment. Matched case-insensitively: the `ultimate_psionics`
/// dissonance-modifier family (`up_equipmods.lst:141-142`) spells it
/// `TYPE=ENHANCEMENT`.
const ENHANCEMENT_BONUS_TYPE: &str = "TYPE=Enhancement";

/// The qualifier position PCGen puts a weapon roll chain's bonus type in:
/// `BONUS:WEAPON|<TOHIT|DAMAGE|...>|<n>|TYPE=Enhancement` — subject, targets,
/// magnitude, **type**.
const ROLL_BONUS_TYPE_POSITION: usize = 3;

/// Whether this chain declares the PF1 *circumstance* bonus type anywhere in
/// its qualifiers.
///
/// A circumstance bonus is, by PF1's own rules definition, conditional on a
/// specific in-game situation the item's holder must be in — never a standing
/// contribution. The caller (`arms_armor::armor_class_bonus_from_bonus_chains`)
/// uses that to decline an otherwise-unconditional `COMBAT|AC` match; the
/// reason it must, and the pinned-oracle disagreement that forced it in SD-33
/// remediation wave 4 (`AT-33-E5-003`), stayed at the call site with the rule
/// it justifies.
///
/// Scans **every** qualifier, not a fixed position, and compares
/// case-sensitively. Both are the predicate as it stood before the move, and
/// both differ deliberately from [`roll_bonus_carries_enhancement_type`]:
/// widening either one here would silently change which corpus records match.
pub fn declares_circumstance_bonus_type(chain: &BonusToken) -> bool {
    qualifiers_declare_circumstance_bonus_type(&chain.qualifiers)
}

/// [`declares_circumstance_bonus_type`] over a bare qualifier slice — the shape
/// [`bonus_chain_qualifiers`](crate::pcgen_import::ingest_record::bonus_chain_qualifiers)
/// hands back when a corpus record is read as JSON rather than parsed from an
/// `.lst` row.
pub fn qualifiers_declare_circumstance_bonus_type<S: AsRef<str>>(qualifiers: &[S]) -> bool {
    qualifiers.iter().any(|q| q.as_ref() == CIRCUMSTANCE_BONUS_TYPE)
}

/// Whether this chain carries the PF1 *enhancement* bonus type **in a weapon
/// roll chain's bonus-type position**.
///
/// Deliberately positional, and deliberately not a scan: requiring the type at
/// position 3 of a `WEAPON|<targets>|<magnitude>|<type>` chain is what excludes
/// the `BONUS:WEAPON|WIELDCATEGORY|...` chains and the bare untyped
/// `BONUS:WEAPON|TOHIT|<n>` Wield-Size chains, which are real but are not a
/// magic enhancement bonus. The caller
/// (`equipmods::weapon_enhancement_bonus`) documents why that exclusion is
/// correct; this function is only the reading of it.
///
/// Compared case-insensitively — SD-33 remediation wave 6
/// (`AT-33-E5-last39-skill-combat`): the `ultimate_psionics`
/// dissonance-modifier family carries the same shape with `TYPE=ENHANCEMENT`,
/// which the prior exact-string match never matched. No real corpus record's
/// `TYPE=` qualifier for this shape is observed in any casing other than those
/// two, and case-insensitive comparison of a **whole segment** cannot turn an
/// unrelated qualifier into a false match.
pub fn roll_bonus_carries_enhancement_type(chain: &BonusToken) -> bool {
    qualifiers_carry_enhancement_roll_type(&chain.qualifiers)
}

/// [`roll_bonus_carries_enhancement_type`] over a bare qualifier slice.
pub fn qualifiers_carry_enhancement_roll_type<S: AsRef<str>>(qualifiers: &[S]) -> bool {
    qualifiers
        .get(ROLL_BONUS_TYPE_POSITION)
        .is_some_and(|q| q.as_ref().eq_ignore_ascii_case(ENHANCEMENT_BONUS_TYPE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ingest_record::bonus_chain_qualifiers;
    use std::path::{Path, PathBuf};

    fn repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// Every equipment record in the shipped corpus, as its bonus-chain
    /// qualifier lists. Reads `data/corpus/**` directly — the live corpus
    /// directory, not a fixture with a hand-derived value.
    fn corpus_equipment_chains() -> Vec<(String, Vec<Vec<String>>)> {
        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else { return };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "json") {
                    out.push(path);
                }
            }
        }

        let corpus = repo().join("data/corpus");
        let mut books: Vec<PathBuf> = std::fs::read_dir(&corpus)
            .expect("data/corpus must exist")
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        books.sort();

        let mut out = Vec::new();
        for book in books {
            let Ok(kinds) = std::fs::read_dir(&book) else { continue };
            let mut kind_dirs: Vec<PathBuf> = kinds
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.is_dir()
                        && p.file_name()
                            .and_then(|n| n.to_str())
                            .is_some_and(|n| n.starts_with("equipment"))
                })
                .collect();
            kind_dirs.sort();
            for kind in kind_dirs {
                let mut files = Vec::new();
                walk(&kind, &mut files);
                files.sort();
                for file in files {
                    let Ok(text) = std::fs::read_to_string(&file) else { continue };
                    let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
                    let chains: Vec<Vec<String>> = bonus_chain_qualifiers(&doc)
                        .into_iter()
                        .map(|c| c.into_iter().map(str::to_owned).collect())
                        .collect();
                    if chains.is_empty() {
                        continue;
                    }
                    let rel = file.strip_prefix(repo()).unwrap_or(&file).display().to_string();
                    out.push((rel, chains));
                }
            }
        }
        out
    }

    /// The whole point of the move: the classification must be **identical**
    /// to the string comparison it replaced, on every real record, not merely
    /// on the ones somebody thought to write down.
    ///
    /// The "before" side here is the literal predicate as it stood in
    /// `arms_armor.rs:142` and `equipmods.rs:231` at `bdae51f9f6`, spelled out
    /// inline. That is deliberate: a round-trip proved against a *paraphrase*
    /// of the old predicate proves nothing, so the old code is transcribed
    /// rather than referenced.
    #[test]
    fn bonus_type_qualifiers_are_unchanged_by_this_module() {
        let mut disagreements = Vec::new();
        let mut circumstance = 0usize;
        let mut enhancement = 0usize;
        let mut records = 0usize;
        let mut chains_seen = 0usize;

        for (rel, chains) in corpus_equipment_chains() {
            records += 1;
            for qualifiers in &chains {
                chains_seen += 1;

                // Verbatim `arms_armor.rs:142` before the move.
                let before_circumstance = qualifiers.iter().any(|q| q == "TYPE=Circumstance");
                // Verbatim `equipmods.rs:231` before the move.
                let before_enhancement =
                    qualifiers.len() >= 4 && qualifiers[3].eq_ignore_ascii_case("TYPE=Enhancement");

                let after_circumstance = qualifiers_declare_circumstance_bonus_type(qualifiers);
                let after_enhancement = qualifiers_carry_enhancement_roll_type(qualifiers);

                if before_circumstance != after_circumstance || before_enhancement != after_enhancement {
                    disagreements.push(format!("{rel}: {qualifiers:?}"));
                }
                if after_circumstance {
                    circumstance += 1;
                }
                if after_enhancement {
                    enhancement += 1;
                }
            }
        }

        assert!(
            disagreements.is_empty(),
            "{} corpus bonus chain(s) classify differently after the move:\n{}",
            disagreements.len(),
            disagreements.join("\n")
        );
        assert!(records > 0, "no equipment record carried a bonus chain -- the walk found nothing");
        assert!(chains_seen > 0, "no bonus chains were examined");
        // Not pinned to an exact figure: a new book legitimately moves both.
        // What is pinned is that each population is non-empty, so a walk that
        // silently stopped finding records cannot pass this test by agreeing
        // with itself about nothing.
        assert!(
            circumstance > 0,
            "no corpus chain declares TYPE=Circumstance -- the walk is reading nothing"
        );
        assert!(
            enhancement > 0,
            "no corpus chain carries TYPE=Enhancement in the roll bonus-type position -- \
             the walk is reading nothing"
        );
    }

    /// The two predicates differ in two respects, and both differences are
    /// load-bearing. This pins them so a future tidy-up cannot quietly
    /// converge them.
    #[test]
    fn the_two_predicates_keep_their_deliberate_differences() {
        // Circumstance scans every position; enhancement does not.
        let anywhere = vec!["COMBAT", "AC", "-2", "X", "TYPE=Circumstance"];
        assert!(qualifiers_declare_circumstance_bonus_type(&anywhere));
        let late_enhancement = vec!["WEAPON", "TOHIT", "1", "X", "TYPE=Enhancement"];
        assert!(
            !qualifiers_carry_enhancement_roll_type(&late_enhancement),
            "enhancement is positional: position 3 only"
        );

        // Enhancement is case-insensitive; circumstance is not.
        let shouted = vec!["WEAPON", "DAMAGE,TOHIT", "2", "TYPE=ENHANCEMENT"];
        assert!(qualifiers_carry_enhancement_roll_type(&shouted));
        let shouted_circumstance = vec!["COMBAT", "AC", "-2", "TYPE=CIRCUMSTANCE"];
        assert!(
            !qualifiers_declare_circumstance_bonus_type(&shouted_circumstance),
            "circumstance is compared case-sensitively, as it was before the move"
        );

        // A short chain is not an enhancement chain.
        let short = vec!["WEAPON", "TOHIT", "1"];
        assert!(!qualifiers_carry_enhancement_roll_type(&short));
    }

    /// The `BonusToken` entry points agree with the slice entry points they
    /// delegate to, so the live side and the corpus gate cannot drift apart.
    #[test]
    fn the_token_and_slice_entry_points_agree() {
        let token = BonusToken {
            line_number: 1,
            raw_bonus: "BONUS:WEAPON|TOHIT|1|TYPE=Enhancement".to_string(),
            qualifiers: vec![
                "WEAPON".to_string(),
                "TOHIT".to_string(),
                "1".to_string(),
                "TYPE=Enhancement".to_string(),
            ],
        };
        assert_eq!(
            roll_bonus_carries_enhancement_type(&token),
            qualifiers_carry_enhancement_roll_type(&token.qualifiers)
        );
        assert_eq!(
            declares_circumstance_bonus_type(&token),
            qualifiers_declare_circumstance_bonus_type(&token.qualifiers)
        );
        assert!(roll_bonus_carries_enhancement_type(&token));
        assert!(!declares_circumstance_bonus_type(&token));
    }
}
