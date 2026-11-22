//! The live side never names an ingest *qualifier* in executable code.
//!
//! SD-35 Epic 6, `AT-35-E6-003-SWEEP` cycle 14.
//!
//! Sibling of `sd35_rendered_prose_carries_no_ingest_vocabulary.rs`, and a
//! different concern from it. That gate is about the strings this engine
//! *prints*: an ingest token named mid-sentence inside prose a player reads is
//! a sheet-rule defect (`decisions.md` §1). This gate is about the strings this
//! engine *compares against*: a live module that writes
//!
//! ```text
//! qualifiers.iter().any(|q| q == "TYPE=Circumstance")
//! ```
//!
//! is not printing a token, but it is doing the thing `decisions.md` §11
//! forbids outright — holding the ingest format's vocabulary on the live side
//! and reading a PCGen token with it. `technical-design.md` §0 draws that
//! boundary **by path**: `src/pcgen_import/**` may know the ingest grammar,
//! `src/rules_core/**` and `apps/desktop/**` may not.
//!
//! The remedy is the one `src/pcgen_import/bonus_chain_reader.rs` states in its
//! own module doc: the converter hands the live side a **narrowed,
//! already-classified value** — a `bool`, a typed enum, a parsed kin name — and
//! after it "no live module names a qualifier position, a chain keyword ... or
//! the ingest field itself". Moving the literal is not the point; moving the
//! *grammar ownership* is, and the live call site must end up asking a
//! semantic question rather than performing a string match it had to know the
//! ingest format to write.
//!
//! # What is scanned, and what is not
//!
//! * **Comment lines are skipped.** Operator ruling B14 (`decisions.md` §17): a
//!   comment does not execute, and provenance recording which ingest token a
//!   converted value came from is deliberately kept. Every relocation this
//!   gate forced left its justification behind as a `//` comment naming the
//!   token, exactly as B14 intends.
//! * **`#[cfg(test)]` regions are skipped.** They are compiled out of the
//!   shipping library. Whether `scripts/pcgen_residue_gate.py` itself should
//!   skip them is an open operator ruling (`progress.md`, `## Open blockers`);
//!   this gate does not presume that ruling for the gate, it simply declines to
//!   call a test assertion "live code", which it is not.
//!
//! # Why a per-file list rather than a whole-tree scan
//!
//! The whole-tree scan already exists and is the authority:
//! `python3 scripts/pcgen_residue_gate.py --check`. It reports 370 code hits in
//! 48 live files at the tree this gate was written on, 351 of them inside
//! `#[cfg(test)]` modules. A Rust test asserting zero across the whole tree
//! would be red on arrival and stay red until an operator ruling lands, which
//! makes it a wish rather than a gate. This list is instead the **ratchet**:
//! every cycle that clears a file adds it here, and `cargo test` then keeps it
//! clear without waiting on the python gate to be run by hand.

/// Live files whose executable lines have been cleared of ingest qualifier
/// vocabulary. Append-only: a file is added by the cycle that clears it.
const SCANNED: &[&str] = &[
    // Cycle 14. `armor_class_bonus_from_bonus_chains` compared a parsed
    // qualifier slice to `TYPE=Circumstance`, and the weapon enhancement match
    // compared position 3 to `TYPE=Enhancement`. Both classifications now come
    // back as a `bool` from `pcgen_import::equipment_bonus_reader`.
    "src/rules_core/equipment_effects/arms_armor.rs",
    "src/rules_core/equipment_effects/equipmods.rs",
    // Cycle 14. Both files held an ingest prefix as a live `const` and did the
    // `strip_prefix` / `starts_with` themselves. The prefixes and the parse
    // both moved into `pcgen_import::race_trait_tokens`, which already owned
    // the rest of this record kind's grammar.
    "src/rules_core/skinwalker_change_shape.rs",
    "src/rules_core/race_resolver.rs",
    // Cycle 15. The Alternate Racial Traits picker read the mutual-exclusion
    // guard by walking `raw_tokens` itself, keyed on `ABILITY` / `PREMULT` /
    // `PREABILITY` / `!PREFACT` and stripping `PREVAREQ:` by hand -- four
    // spellings of one relation, all four of them ingest grammar held inside
    // the desktop crate. The whole reading is now
    // `pcgen_import::race_trait_tokens::exclusion_guard_flags` and its two
    // sibling findings readers, which hand back already-derived flag names.
    // This is the only `apps/` file the residue gate ever listed.
    "apps/desktop/src-tauri/src/race_trait_picker.rs",
];

/// The ingest qualifier vocabulary, in the same shapes
/// `scripts/pcgen_residue_gate.py` counts.
///
/// `raw_tokens` is the ingest **field** rather than a qualifier, and is scanned
/// for the reason this file's header already gives: after the remedy "no live
/// module names a qualifier position, a chain keyword ... or the ingest field
/// itself". A live module that reaches into the token array has held the
/// grammar whether or not the line it wrote contains a colon --
/// `token.key == "PREMULT"` names a PCGen token and the `PRE<UPPER>+:` family
/// walk below cannot see it. Scanning the field the traversal must start from
/// closes that gap without guessing at token spellings.
const LITERAL_PATTERNS: &[&str] =
    &["TYPE=", "BONUS:", "DEFINE:", "DESC:", "SAB:", "%CHOICE", "%LIST", "CHOOSE:", "raw_tokens"];

/// The first ingest qualifier this line names, if any. `PRE<UPPER>+:` is
/// matched separately because it is a family, not a literal.
fn ingest_qualifier_in(line: &str) -> Option<String> {
    for literal in LITERAL_PATTERNS {
        if line.contains(literal) {
            return Some((*literal).to_owned());
        }
    }
    let bytes = line.as_bytes();
    for (i, _) in line.match_indices("PRE") {
        let mut j = i + 3;
        while j < bytes.len() && bytes[j].is_ascii_uppercase() {
            j += 1;
        }
        if j > i + 3 && j < bytes.len() && bytes[j] == b':' {
            return Some(line[i..=j].to_owned());
        }
    }
    None
}

/// 1-based line numbers inside a `#[cfg(test)]` item, by brace balance. The
/// same conservative walk the cycle-5 census and the cycle-10 prose gate use.
fn cfg_test_lines(lines: &[&str]) -> Vec<bool> {
    let mut inside = vec![false; lines.len()];
    let mut i = 0usize;
    while i < lines.len() {
        if lines[i].trim_start().starts_with("#[cfg(test)]") {
            let mut j = i;
            while j < lines.len() && !lines[j].contains('{') {
                j += 1;
            }
            if j == lines.len() {
                break;
            }
            let mut depth = 0i32;
            let start = j;
            while j < lines.len() {
                for ch in lines[j].chars() {
                    match ch {
                        '{' => depth += 1,
                        '}' => depth -= 1,
                        _ => {}
                    }
                }
                #[allow(clippy::needless_range_loop)]
                for k in start..=j {
                    inside[k] = true;
                }
                if depth <= 0 {
                    break;
                }
                j += 1;
            }
            i = j + 1;
            continue;
        }
        i += 1;
    }
    inside
}

#[test]
fn cleared_live_files_name_no_ingest_qualifier_in_executable_code() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut findings: Vec<String> = Vec::new();

    for rel in SCANNED {
        let path = repo.join(rel);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        let lines: Vec<&str> = text.lines().collect();
        let in_test = cfg_test_lines(&lines);

        for (idx, line) in lines.iter().enumerate() {
            if line.trim_start().starts_with("//") || in_test[idx] {
                continue;
            }
            if let Some(hit) = ingest_qualifier_in(line) {
                findings.push(format!("{rel}:{}: `{hit}` -- {}", idx + 1, line.trim()));
            }
        }
    }

    assert!(
        findings.is_empty(),
        "{} executable line(s) on the live side still name an ingest qualifier \
         (decisions.md §11; technical-design.md §0 draws the boundary by path). \
         Move the grammar into `src/pcgen_import/**` and have it hand back an \
         already-classified value -- a bool, a typed enum, a parsed name -- so \
         the live call site asks a semantic question instead of matching a \
         string it had to know the ingest format to write:\n{}",
        findings.len(),
        findings.join("\n")
    );
}

/// The ratchet only means something if it is actually holding files.
#[test]
fn the_cleared_list_is_not_empty_and_every_entry_exists() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!SCANNED.is_empty(), "the cleared-file list must never be emptied");
    for rel in SCANNED {
        assert!(repo.join(rel).is_file(), "{rel} is listed as cleared but does not exist");
    }
}
