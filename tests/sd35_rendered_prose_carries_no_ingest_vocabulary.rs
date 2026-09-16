//! The prose this engine WRITES never prints PCGen ingest vocabulary.
//!
//! SD-35 Epic 6, `AT-35-E6-003-SWEEP` cycle 10.
//!
//! `ComputationExplanation.detail` (`src/rules_core/pilot_compute/mod.rs`) is
//! carried to the desktop crate as `ExplanationDto.detail` and rendered on the
//! Character Hub sheet; `ComputationDiagnostic.message` and
//! `SupportStateRow.next_required_uplift` reach a reader the same way. The
//! sheet rule (`decisions.md` §1) says a sheet line is a final number, dice in
//! final form, or the rule's words — never ingest vocabulary. A token named
//! mid-sentence inside one of those strings is therefore a sheet-rule defect,
//! and `scripts/pcgen_residue_gate.py` counts it as a live PCGen read.
//!
//! Three automatic citation frames were built for this mechanism across
//! `AT-35-E6-003-SWEEP` cycles 4–6 and two were **removed** for producing
//! ungrammatical sheet prose, so the remainder was cleared by hand, row by row,
//! in `artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle10_prose_citation_handwork.py`.
//! This test is the standing gate that keeps it cleared: it re-derives the
//! count from the source files themselves rather than from a recorded figure.
//!
//! # What is scanned, and what is not
//!
//! * **Comment lines are skipped.** Operator ruling B14 (`decisions.md` §17):
//!   a comment does not execute, and provenance recording where a converted
//!   number came from is kept. The hand table moves tokens *into* comments.
//! * **`#[cfg(test)]` regions are skipped.** They are compiled out of the
//!   shipping library. Whether the residue gate itself should skip them is an
//!   open operator ruling (`progress.md`, `## Open blockers`); this test does
//!   not presume that ruling for the *gate*, it simply declines to call a test
//!   assertion "rendered prose", which it is not.
//! * **The Pathfinder Unchained `DESC:` transcriptions are gone, not skipped.** They used to be
//!   exempted here as verbatim corpus text held so `render_pcgen_desc_tokens` could substitute
//!   `%N` into them. SD-35 `AT-35-E6-003` deleted them and the renderer call with them:
//!   `pilot_compute::resolved_prose` renders the CONVERTED prose out of `data/sheet_rules/`
//!   instead, and `sd27_pu_class_feature_descriptions_carry_the_characters_numbers` proves the
//!   two render byte-identical words. This file has no exemption left.

const SCANNED: &[&str] = &[
    "src/rules_core/pilot_compute/mod.rs",
    "src/rules_core/pilot_compute/class_slayer.rs",
    "src/rules_core/pilot_compute/class_ultimate_combat.rs",
    "src/rules_core/derived_evaluator_fixture_check.rs",
    "src/rules_core/support_state_matrix.rs",
    // Cycle 11. The shipped content tables are prose this engine writes to a
    // sheet just as much as an explanation string is: `description` is printed
    // verbatim and `description_variables` supplies the words `%1`/`%2` are
    // replaced with (`derived_evaluator_fixture_check::monster_ability_save_dc`
    // indexes it positionally; `apps/desktop/src-tauri/src/companion_catalog.rs`
    // serves it). A `%CHOICE` left in that array prints the ingest token where
    // the player's chosen option belongs.
    "src/rules_core/rules_tables/bestiary/monster_data.rs",
    "src/rules_core/rules_tables/bestiary_3/monster_data.rs",
    "src/rules_core/rules_tables/inner_sea_world_guide/monster_data.rs",
    // Cycle 12. A shipped feat catalog's `FeatEffectBonus.qualifiers` is read
    // by `damage_total::constant_damage_bonus` to produce a number the sheet
    // prints, and the sheet line for Weapon Focus has to say "your chosen
    // weapon" — the rule's words — not the ingest format's `%LIST`. These four
    // files are every shipped feat table that carried a selection stand-in;
    // `pcgen_import::feat_effect_selections` holds the verbatim chains.
    "src/rules_core/rules_tables/crb/feat_data/combat.rs",
    "src/rules_core/rules_tables/crb/feat_data/general.rs",
    "src/rules_core/rules_tables/acg/feat_data/combat.rs",
    "src/rules_core/rules_tables/advanced_race_guide/feat_data/general.rs",
    // Cycle 13. `CompanionRecord::external_ability_refs` is a list of ability
    // NAMES, and `apps/desktop/src-tauri/src/companion_catalog.rs` serves it to
    // the player as one. Three CRB creature rows carried the ingest guard the
    // corpus appended to the grant as a fourth entry of that list, so the token
    // was on screen. `pcgen_import::companion_pcgen_guards` holds the verbatim
    // pre-conversion arrays.
    "src/rules_core/rules_tables/crb/companion_data.rs",
];

/// A `\b` word boundary immediately before byte `at`, exactly as the residue
/// gate's `\bBONUS:` means it: a Rust identifier such as `ARMOR_BONUS: i16`, or
/// a format spec such as `{MONK_GRAPPLE_BONUS:+}`, is NOT an ingest token, and
/// neither is `TEMPBONUS:`.
fn on_word_boundary(line: &str, at: usize) -> bool {
    if at == 0 {
        return true;
    }
    let prev = line.as_bytes()[at - 1];
    !(prev.is_ascii_alphanumeric() || prev == b'_')
}

/// The residue gate's own token-syntax surface (`scripts/pcgen_residue_gate.py`,
/// `TOKEN_SYNTAX_PATTERNS`), re-expressed. `PRE[A-Z]+:` is matched by rule
/// rather than by literal; `%CHOICE` and `%LIST` carry no `\b` in the gate.
fn ingest_vocabulary_in(line: &str) -> Option<String> {
    for literal in ["BONUS:", "DEFINE:", "DESC:", "SAB:", "TYPE="] {
        for (i, _) in line.match_indices(literal) {
            if on_word_boundary(line, i) {
                return Some(literal.to_owned());
            }
        }
    }
    for literal in ["%CHOICE", "%LIST"] {
        if line.contains(literal) {
            return Some(literal.to_owned());
        }
    }
    let bytes = line.as_bytes();
    for (i, _) in line.match_indices("PRE") {
        if !on_word_boundary(line, i) {
            continue;
        }
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
/// same conservative walk the cycle-5 census uses.
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
fn rendered_prose_carries_no_pcgen_ingest_vocabulary() {
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
            if let Some(hit) = ingest_vocabulary_in(line) {
                findings.push(format!(
                    "{rel}:{}: `{hit}` in rendered prose -- {}",
                    idx + 1,
                    line.trim()
                ));
            }
        }
    }

    assert!(
        findings.is_empty(),
        "{} line(s) of prose this engine writes still print PCGen ingest \
         vocabulary on a player's sheet (sheet rule, decisions.md §1). Demote each \
         into a `//` provenance comment beside the value and say the same thing in \
         the rule's own words:\n{}",
        findings.len(),
        findings.join("\n")
    );
}
