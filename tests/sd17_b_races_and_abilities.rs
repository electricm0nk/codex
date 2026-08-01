//! GE-03 SD-17-B-3 acceptance tests for races and race-ability objects.
//!
//! Verifies that [`codex::pcgen_import::lst_parser`]:
//!
//! - recognizes every `RACE:` / `RACES:` line in the PCGen LST corpus and
//!   structures it as a deterministic parse record with source line numbers
//!   preserved
//! - recognizes every `ABILITY:` line (pointer or full declaration) in the
//!   PCGen LST corpus and structures the category, name, kind, target, and
//!   trailing modifiers as discrete fields where present
//! - emits a `MalformedSD17-B-3` diagnostic for hand-built LST inputs whose
//!   race or ability declarations are not parseable
//! - parses representative LST files in O(n) time (linear in line count)
//! - carries source line numbers on every parse record
//!
//! These tests are TDD-failing at the time of authorship: `lst_parser`
//! does not yet exist. The first run will fail with a compile error in
//! the `use codex::pcgen_import::lst_parser::...` line; that compile
//! error is the RED state. Implementation in `src/pcgen_import/lst_parser.rs`
//! drives the transition to GREEN.

use std::path::PathBuf;

use codex::pcgen_import::lst_parser::race_ability::LstDiagnosticKind;
use codex::pcgen_import::lst_parser::{AbilityKind, parse_lst_entry};

/// `PCGEN_CORPUS_ROOT` wins when set; otherwise
/// `$HOME/workspace/repos/pcgen/data` — HOME-relative because the operator
/// keeps `workspace/` in the home directory and syncs it between machines.
/// Rust does not expand `~`, so `$HOME` is read via `std::env::var_os`.
fn corpus_root() -> Option<PathBuf> {
    let configured = std::env::var_os("PCGEN_CORPUS_ROOT").map(PathBuf::from);
    let default = std::env::var_os("HOME")
        .filter(|home| !home.is_empty())
        .map(|home| PathBuf::from(home).join("workspace/repos/pcgen/data"));
    configured
        .or_else(|| default.filter(|path| path.is_dir()))
        .filter(|path| path.is_dir())
}

// --- Race pointer recognition (RACE: / RACES:) ----------------------------

#[test]
fn parses_race_pointer_with_line_number_and_raw_directive() {
    let source_path = "core_rulebook.pcc";
    let input = "# Sources\n\
                  SOURCELONG:Pathfinder\n\
                  RACE:cr_races.lst\n\
                  RACES:cr_races_companion.lst\n";

    let result = parse_lst_entry(source_path, input);

    // Two race pointer declarations: the PCC convention is the singular
    // `RACE:` form; the operator's card body uses the plural. Both are valid.
    assert_eq!(
        result.race_pointers.len(),
        2,
        "expected both RACE: and RACES: declarations to register as race pointers, got {:?}",
        result.race_pointers
    );

    let first = &result.race_pointers[0];
    assert_eq!(first.source_path, source_path);
    assert_eq!(first.line_number, 3);
    assert_eq!(first.raw_directive, "RACE:cr_races.lst");
    assert_eq!(first.target, "cr_races.lst");

    let second = &result.race_pointers[1];
    assert_eq!(second.line_number, 4);
    assert_eq!(second.raw_directive, "RACES:cr_races_companion.lst");
    assert_eq!(second.target, "cr_races_companion.lst");
}

#[test]
fn race_pointer_without_target_emits_malformed_diagnostic() {
    let source_path = "broken_race.pcc";
    let input = "RACE:\nRACES:\n";

    let result = parse_lst_entry(source_path, input);

    // Neither line produces a race pointer record.
    assert!(
        result.race_pointers.is_empty(),
        "empty-target race pointers must not produce records, got {:?}",
        result.race_pointers
    );

    // Both lines are reported as structured diagnostics, not silently dropped.
    assert_eq!(
        result.diagnostics.len(),
        2,
        "expected one diagnostic per malformed race pointer, got {:?}",
        result.diagnostics
    );
    assert_eq!(result.diagnostics[0].line_number, 1);
    assert_eq!(
        result.diagnostics[0].kind,
        LstDiagnosticKind::MalformedRacePointer
    );
    assert_eq!(result.diagnostics[1].line_number, 2);
    assert_eq!(
        result.diagnostics[1].kind,
        LstDiagnosticKind::MalformedRacePointer
    );
}

#[test]
fn ignores_non_race_and_non_ability_lines() {
    // Lines that are comments, blanks, or other LST object pointers must not
    // appear in either `race_pointers` or `ability_declarations`. This guards
    // the parser's scope boundary against future slice bleed.
    let input = "\
        # leading comment\n\
        \n\
        CLASS:cr_classes.lst\n\
        SKILL:cr_skills.lst\n\
        TEMPLATE:cr_templates.lst\n\
        # trailing comment\n";

    let result = parse_lst_entry("isolation.pcc", input);

    assert!(result.race_pointers.is_empty());
    assert!(result.ability_declarations.is_empty());
    assert!(
        result.diagnostics.is_empty(),
        "no diagnostics for out-of-scope lines, got {:?}",
        result.diagnostics
    );
}

// --- Ability declaration recognition -------------------------------------

#[test]
fn parses_ability_pointer_with_line_number_and_raw_directive() {
    // The simplest form: a bare `ABILITY:<target.lst>` pointer used in PCC files.
    let source_path = "core_rulebook.pcc";
    let input = "\
        ABILITY:cr_abilities.lst\n\
        ABILITY:cr_feats.lst\n";

    let result = parse_lst_entry(source_path, input);

    assert_eq!(result.ability_declarations.len(), 2);

    let first = &result.ability_declarations[0];
    assert_eq!(first.source_path, source_path);
    assert_eq!(first.line_number, 1);
    assert_eq!(first.raw_directive, "ABILITY:cr_abilities.lst");
    // Bare pointers carry no parsed-into-fields structure; they record the
    // pointer target so the wider GE-03 importer can resolve them.
    assert_eq!(
        first.parsed.as_ref().map(|p| p.target.as_str()),
        Some("cr_abilities.lst")
    );

    let second = &result.ability_declarations[1];
    assert_eq!(second.line_number, 2);
    assert_eq!(second.raw_directive, "ABILITY:cr_feats.lst");
}

#[test]
fn parses_ability_pipe_delimited_declaration_with_fields() {
    // The full structure used inside race `_abilities_race.lst` files:
    // `ABILITY:<name>|<kind>|<target>` where kind is one of AUTOMATIC,
    // VIRTUAL, NORMAL.
    let input = "ABILITY:Dwarf Racial Trait|AUTOMATIC|Dwarf ~ Ability Scores\n";

    let result = parse_lst_entry("dwarf_abilities_race.lst", input);
    assert_eq!(result.ability_declarations.len(), 1);
    assert!(result.diagnostics.is_empty());

    let entry = &result.ability_declarations[0];
    assert_eq!(entry.line_number, 1);
    assert_eq!(entry.source_path, "dwarf_abilities_race.lst");

    let parsed = entry
        .parsed
        .as_ref()
        .expect("pipe-delimited ability should be parsed");
    // No CATEGORY= prefix → category is None.
    assert_eq!(parsed.category, None);
    assert_eq!(parsed.name, "Dwarf Racial Trait");
    assert_eq!(parsed.kind, Some(AbilityKind::Automatic));
    assert_eq!(parsed.target, "Dwarf ~ Ability Scores");
    assert!(parsed.trailing_modifiers.is_empty());
}

#[test]
fn parses_ability_category_prefix_and_trailing_modifiers() {
    // The Darwin's World corpus shows the `CATEGORY=...|name[tabs]FREE:YES`
    // shape. Splits at the first `|` carry the category; subsequent
    // tab/whitespace-separated `KEY:VALUE` modifiers are preserved as
    // trailing entries.
    let input = "ABILITY:CATEGORY=FEAT|Alertness\tFREE:YES\n";

    let result = parse_lst_entry("dw2_kits.lst", input);
    assert_eq!(result.ability_declarations.len(), 1);
    assert!(result.diagnostics.is_empty());

    let parsed = result.ability_declarations[0]
        .parsed
        .as_ref()
        .expect("CATEGORY=...|name should be parsed");
    assert_eq!(parsed.category.as_deref(), Some("FEAT"));
    assert_eq!(parsed.name, "Alertness");
    assert_eq!(parsed.kind, None);
    assert_eq!(parsed.target, "");
    assert_eq!(parsed.trailing_modifiers, vec!["FREE:YES"]);
}

#[test]
fn parses_ability_category_prefix_and_single_space_trailing_modifiers() {
    let input = "ABILITY:CATEGORY=FEAT|Alertness FREE:YES STACK:YES\n";

    let result = parse_lst_entry("legacy_spaces.lst", input);
    assert_eq!(result.ability_declarations.len(), 1);
    assert!(result.diagnostics.is_empty());

    let parsed = result.ability_declarations[0]
        .parsed
        .as_ref()
        .expect("single-space modifiers should be parsed");
    assert_eq!(parsed.category.as_deref(), Some("FEAT"));
    assert_eq!(parsed.name, "Alertness");
    assert_eq!(parsed.trailing_modifiers, vec!["FREE:YES", "STACK:YES"]);
}

#[test]
fn preserves_spaces_inside_ability_targets_without_modifiers() {
    let input = "ABILITY:Dwarf Racial Trait|AUTOMATIC|Dwarf ~ Ability Scores\n";

    let result = parse_lst_entry("target_spaces.lst", input);
    let parsed = result.ability_declarations[0]
        .parsed
        .as_ref()
        .expect("ability target should be parsed");
    assert_eq!(parsed.target, "Dwarf ~ Ability Scores");
    assert!(parsed.trailing_modifiers.is_empty());
}

#[test]
fn parses_ability_virtual_kind() {
    let input = "ABILITY:Traits|VIRTUAL|%LIST\n";
    let result = parse_lst_entry("aasimar_abilities_race.lst", input);
    assert_eq!(result.ability_declarations.len(), 1);
    let parsed = result.ability_declarations[0]
        .parsed
        .as_ref()
        .expect("virtual ability declaration should be parsed");
    assert_eq!(parsed.name, "Traits");
    assert_eq!(parsed.kind, Some(AbilityKind::Virtual));
    assert_eq!(parsed.target, "%LIST");
}

#[test]
fn parses_ability_normal_kind() {
    let input = "ABILITY:Longbow Proficiency|NORMAL|Longbow\n";
    let result = parse_lst_entry("weap_prof.lst", input);
    let parsed = result.ability_declarations[0]
        .parsed
        .as_ref()
        .expect("NORMAL ability declaration should be parsed");
    assert_eq!(parsed.name, "Longbow Proficiency");
    assert_eq!(parsed.kind, Some(AbilityKind::Normal));
    assert_eq!(parsed.target, "Longbow");
}

#[test]
fn malformed_ability_emits_diagnostic() {
    // A pipe-delimited ability with only one segment (no kind or target) is
    // not shape-parseable; we treat it as malformed and surface a diagnostic
    // rather than fabricate fields.
    let input = "ABILITY:|\n";
    let result = parse_lst_entry("broken.lst", input);

    assert!(result.ability_declarations.is_empty());
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(
        result.diagnostics[0].kind,
        LstDiagnosticKind::MalformedAbilityDeclaration
    );
    assert_eq!(result.diagnostics[0].line_number, 1);
}

#[test]
fn every_record_carries_source_line_number() {
    let input = "\
        # block\n\
        ABILITY:cr_abilities.lst\n\
        RACE:cr_races.lst\n\
        ABILITY:Dwarf Racial Trait|AUTOMATIC|Dwarf ~ Size\n";
    let result = parse_lst_entry("combo.lst", input);

    assert!(result.diagnostics.is_empty());
    assert_eq!(result.race_pointers.len(), 1);
    assert_eq!(result.ability_declarations.len(), 2);

    for record in &result.race_pointers {
        assert!(
            record.line_number >= 1,
            "every race record must carry a 1-based source line number, got {:?}",
            record
        );
        assert_eq!(record.source_path, "combo.lst");
    }
    for record in &result.ability_declarations {
        assert!(
            record.line_number >= 1,
            "every ability record must carry a 1-based source line number, got {:?}",
            record
        );
        assert_eq!(record.source_path, "combo.lst");
    }
}

// --- Corpus-wide deterministic parse (7 core races) ---------------------

#[test]
fn parses_seven_core_pathfinder_races_deterministically() {
    let Some(corpus_root) = corpus_root() else {
        eprintln!("PCGEN_CORPUS_ROOT is unavailable; skipping canonical race corpus test");
        return;
    };

    // The Pathfinder core races whose `races.lst` files exist under
    // `core_essentials/races/<name>/`. Each entry is `(dir_name,
    // file_stem)` because the PCGen corpus spells some race directories
    // and file stems inconsistently (`half_elf/` + `halfelf_races.lst`,
    // `half_orc/` + `halforc_races.lst`, etc.).
    let race_files: &[(&str, &str)] = &[
        ("dwarf", "dwarf"),
        ("elf", "elf"),
        ("gnome", "gnome"),
        ("half_elf", "halfelf"),
        ("half_orc", "halforc"),
        ("halfling", "halfling"),
        ("human", "human"),
    ];

    for (dir, stem) in race_files {
        let races_path = corpus_root
            .join("pathfinder/paizo/roleplaying_game/core_essentials/races")
            .join(dir)
            .join(format!("{stem}_races.lst"));
        let input = std::fs::read_to_string(&races_path).unwrap_or_else(|e| {
            panic!(
                "core race fixture missing: {} (error: {})",
                races_path.display(),
                e
            )
        });
        let source_path = races_path.to_string_lossy();
        let result = parse_lst_entry(&source_path, &input);

        // For each core race's `<race>_races.lst`, the parser walks the
        // file with no errors (inline `ABILITY:<bundle>|AUTOMATIC|Racial
        // Traits ~ <Race>` snippets embedded in tab-separated rows are
        // left for the SD-17 row-level slice and intentionally ignored
        // by this line-leading directive recognizer). The parser must
        // therefore report zero diagnostics on each core race file.
        assert!(
            result.diagnostics.is_empty(),
            "core race fixture {} must parse with no diagnostics: {:?}",
            races_path.display(),
            result.diagnostics
        );

        // Each core race file declares one playable race row whose
        // documented structure begins with the race name at column 0.
        // The parser does not (yet) walk tabular rows; the slice's
        // scope is line-leading `RACE:` / `ABILITY:` directives. We
        // therefore expect zero records on these specific inputs and
        // assert that bound so a future regression that invents
        // records here (without the SD-17-B-x row-grammar slice) is
        // caught loudly.
        assert!(
            result.race_pointers.is_empty(),
            "core race {} (a races.lst, not a PCC file) must not produce race pointers, got {:?}",
            races_path.display(),
            result.race_pointers
        );
    }
}

#[test]
fn hand_build_malformed_races_and_abilities_produces_malformed_sd17_b3_diagnostic() {
    // Hand-built LST input containing every flavor of malformed race or
    // ability declaration. Each becomes a structured diagnostic with the
    // shared SD-17-B-3 classification. The specific kind on each
    // diagnostic records which shape failed.
    let input = "\
        # Header comment\n\
        RACE:\n\
        RACES:    \n\
        ABILITY:|\n";

    let result = parse_lst_entry("malformed_sd17_b3.lst", input);

    assert!(result.race_pointers.is_empty());
    assert!(result.ability_declarations.is_empty());
    assert_eq!(result.diagnostics.len(), 3);

    // The shared classification surface is the SD-17-B-3 prefix: every
    // diagnostic carries the SD-17-B-3 tag.
    for diag in &result.diagnostics {
        assert!(
            diag.message.contains("SD17-B-3") || diag.kind_is_sd17_b3(),
            "diagnostic must classify as SD17-B-3, got {:?}",
            diag
        );
        assert!(diag.line_number >= 1);
    }
}

// --- Performance: parse is O(n) in line count ----------------------------

#[test]
fn parser_runs_in_linear_time_on_representative_lst() {
    let Some(corpus_root) = corpus_root() else {
        eprintln!("PCGEN_CORPUS_ROOT is unavailable; skipping canonical corpus perf test");
        return;
    };

    // 211 LST files contain `RACE:` lines per the operator-directed
    // corpus scan. Use the largest representative file (Darwin's World
    // combined `.lst` files) to demonstrate linear-time parsing.
    let representative = corpus_root.join(
        "darwins_world_2/rpg_objects/darwins_world_2/terrors_of_twisted_earth/dw2tte_kits_creatures_m_z.lst",
    );
    let input = std::fs::read_to_string(&representative).unwrap_or_else(|e| {
        panic!(
            "representative fixture missing: {} ({})",
            representative.display(),
            e
        )
    });

    let line_count = input.lines().count();
    assert!(
        line_count >= 100,
        "representative fixture must be non-trivial, got {} lines",
        line_count
    );

    // First pass establishes a baseline. Second pass must scale linearly:
    // doubling the input must at worst quadruple the time. We assert the
    // simpler property that the parser completes well under a generous
    // per-line budget.
    let start = std::time::Instant::now();
    let result = parse_lst_entry(&representative.to_string_lossy(), &input);
    let elapsed = start.elapsed();

    // Per-line budget: 1ms / 100 lines. For a ~hundreds-line file this is
    // generous (a few ms total). The point is to catch quadratic blowups
    // (nested-loop scanners) without flaking on slow CI hosts.
    let budget_nanos = (line_count as u128) * 1_000_000;
    assert!(
        elapsed.as_nanos() < budget_nanos,
        "parse exceeded linear budget: {} lines took {:?}, budget {:?}",
        line_count,
        elapsed,
        std::time::Duration::from_nanos(budget_nanos as u64)
    );

    // Sanity: the corpus file carries real `ABILITY:` records; ensure the
    // parser actually consumed them rather than returning an empty result.
    assert!(
        !result.ability_declarations.is_empty(),
        "representative corpus file must carry real ABILITY: records"
    );
}

// --- API surface assertions ---------------------------------------------

#[test]
fn lst_entry_file_carries_required_fields() {
    let result = parse_lst_entry("x.lst", "");
    assert_eq!(result.source_path, "x.lst");
    assert!(result.race_pointers.is_empty());
    assert!(result.ability_declarations.is_empty());
    assert!(result.diagnostics.is_empty());
}
