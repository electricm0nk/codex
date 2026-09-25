// SD-36 Epic A: `write_settled_spell_artifact` used to live in `codex` behind
// `#[cfg(test)]`, calling `crate::pcgen_import::spell_formula_settle::build`
// directly -- a live read of the converter that only worked because it never
// shipped (B15: a `#[cfg(test)]` region is not live code). Both consumers
// (`ScratchRangeRoot`, `ScratchDurationRoot`) moved to this crate along with
// their tests, so the helper moved here too rather than staying in `codex`
// unconditionally reading a crate that no longer exists there.
fn write_settled_spell_artifact(root: &std::path::Path) {
    let package = codex::rules_core::record_vars::RecordVarPackage {
        spell_formulas: codex_ingest::pcgen_import::spell_formula_settle::build(root),
        ..Default::default()
    };
    let out = root.join(codex::rules_core::record_vars::RECORD_VARS_PATH);
    std::fs::create_dir_all(out.parent().unwrap()).unwrap();
    std::fs::write(&out, serde_json::to_string(&package).unwrap()).unwrap();
}

// -- split from `spell_range_seam_tests` in src/rules_core/derived_evaluator_fixture_check.rs (pcgen-touching items only) --
mod spell_range_seam_tests {
    use codex::rules_core::derived_evaluator_fixture_check::*;
    use std::path::PathBuf;

    #[test]
    fn a_wrong_expected_base_ft_makes_run_spell_range_bar_check_report_a_failure() {
        let real = spell_range_formula("Close").unwrap();
        let wrong_expected_base_ft = 999;
        assert_ne!(
            real.base_ft, wrong_expected_base_ft,
            "a corrupted expected value must genuinely disagree with the real formula"
        );

        let scratch = ScratchRangeRoot::new(
            "wrong_base_ft",
            wrong_expected_base_ft,
            real.rate_ft,
            real.per_levels,
        );
        let report = run_spell_range_bar_check(&scratch.root);

        assert!(
            report.cleared.is_empty(),
            "a fixture asserting a wrong expected base_ft must never clear the bar, got {:?}",
            report.cleared
        );
        assert_eq!(
            report.failures.len(),
            1,
            "the one corrupted fixture must be reported as a failure, got {:?}",
            report.failures
        );
        assert!(
            report.failures.contains_key("scratch:spell:scratch_close_spell"),
            "failures: {:?}",
            report.failures
        );
    }

    #[test]
    fn a_correct_expected_base_ft_clears_run_spell_range_bar_check() {
        let real = spell_range_formula("Close").unwrap();
        let scratch =
            ScratchRangeRoot::new("correct_base_ft", real.base_ft, real.rate_ft, real.per_levels);
        let report = run_spell_range_bar_check(&scratch.root);

        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:spell:scratch_close_spell"));
    }

    /// A synthetic `repo_root` carrying exactly one spell corpus record
    /// (`RANGE:Close`) plus one fixture file whose `spell_range_entries`
    /// row is the caller's to corrupt -- lets a test drive the REAL
    /// `run_spell_range_bar_check(&root)` end to end without touching the
    /// committed fixture (which a concurrent cycle may also be reading).
    /// Same `std::env::temp_dir()` + pid-suffixed scratch-dir pattern as
    /// `wiring_class.rs`'s `ScratchBook`.
    struct ScratchRangeRoot {
        root: PathBuf,
    }

    impl ScratchRangeRoot {
        fn new(name: &str, expected_base_ft: i32, expected_rate_ft: i32, expected_per_levels: i32) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_spell_range_mutation_proof_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let spell_dir = root.join("data/corpus/core_rulebook/spell");
            std::fs::create_dir_all(&spell_dir).unwrap();
            std::fs::write(
                spell_dir.join("scratch_close_spell.json"),
                codex_ingest::pcgen_import::ingest_payload::ingest_record_json("scratch_close_spell", &[("RANGE", "Close")]),
            )
            .unwrap();
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(
                    r#"{{"spell_range_entries":[{{
                        "unit_id":"scratch:spell:scratch_close_spell",
                        "book":"core_rulebook",
                        "record_key":"scratch_close_spell",
                        "upstream_lst":"scratch.lst",
                        "upstream_lst_sha256":"0",
                        "upstream_line":1,
                        "corpus_field":"RANGE:Close",
                        "expected":{{"base_ft":{expected_base_ft},"rate_ft":{expected_rate_ft},"per_levels":{expected_per_levels}}}
                    }}]}}"#
                ),
            )
            .unwrap();
            super::write_settled_spell_artifact(&root);
            ScratchRangeRoot { root }
        }
    }

    impl Drop for ScratchRangeRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }


}

// -- split from `spell_seam_tests` in src/rules_core/derived_evaluator_fixture_check.rs (pcgen-touching items only) --
mod spell_seam_tests {
    use codex::rules_core::derived_evaluator_fixture_check::*;
    use std::path::PathBuf;

    #[test]
    fn a_wrong_expected_per_level_makes_run_spell_bar_check_report_a_failure() {
        let real = parse_caster_level_linear_duration("(CASTERLEVEL*10) minutes [D]").unwrap();
        let wrong_expected_per_level = 99;
        assert_ne!(
            real.per_level, wrong_expected_per_level,
            "a corrupted expected value must genuinely disagree with the real parse"
        );

        let scratch =
            ScratchDurationRoot::new("wrong_per_level", wrong_expected_per_level, &real.unit);
        let report = run_spell_bar_check(&scratch.root);

        assert!(
            report.cleared.is_empty(),
            "a fixture asserting a wrong expected per_level must never clear the bar, got {:?}",
            report.cleared
        );
        assert_eq!(
            report.failures.len(),
            1,
            "the one corrupted fixture must be reported as a failure, got {:?}",
            report.failures
        );
        assert!(
            report.failures.contains_key("scratch:spell:scratch_duration_spell"),
            "failures: {:?}",
            report.failures
        );
    }

    #[test]
    fn a_wrong_expected_unit_makes_run_spell_bar_check_report_a_failure() {
        // The scratch corpus row always states "(CASTERLEVEL*10) minutes [D]"
        // (`ScratchDurationRoot::new`) -- get its real parse so `per_level`
        // is correct and only `unit` is corrupted, isolating this test from
        // the sibling per_level-wrong test above.
        let real = parse_caster_level_linear_duration("(CASTERLEVEL*10) minutes [D]").unwrap();
        assert_ne!(real.unit, "rounds", "a corrupted expected unit must genuinely disagree");

        let scratch = ScratchDurationRoot::new("wrong_unit", real.per_level, "rounds");
        let report = run_spell_bar_check(&scratch.root);

        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }

    #[test]
    fn a_correct_expected_duration_clears_run_spell_bar_check() {
        let real = parse_caster_level_linear_duration("(CASTERLEVEL*10) minutes [D]").unwrap();
        let scratch = ScratchDurationRoot::new("correct", real.per_level, &real.unit);
        let report = run_spell_bar_check(&scratch.root);

        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:spell:scratch_duration_spell"));
    }

    /// Duration-seam sibling of `spell_range_seam_tests::ScratchRangeRoot`:
    /// a synthetic `repo_root` carrying one `DURATION:(CASTERLEVEL*10)
    /// minutes [D]` spell record plus a fixture whose `spell_entries` row
    /// the caller corrupts, so a test can drive the REAL
    /// `run_spell_bar_check(&root)` rather than asserting the parser's
    /// output against a hand-typed wrong number in isolation.
    struct ScratchDurationRoot {
        root: PathBuf,
    }

    impl ScratchDurationRoot {
        fn new(name: &str, expected_per_level: i32, expected_unit: &str) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_spell_duration_mutation_proof_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let spell_dir = root.join("data/corpus/core_rulebook/spell");
            std::fs::create_dir_all(&spell_dir).unwrap();
            std::fs::write(
                spell_dir.join("scratch_duration_spell.json"),
                codex_ingest::pcgen_import::ingest_payload::ingest_record_json("scratch_duration_spell", &[("DURATION", "(CASTERLEVEL*10) minutes [D]")]),
            )
            .unwrap();
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(
                    r#"{{"spell_entries":[{{
                        "unit_id":"scratch:spell:scratch_duration_spell",
                        "book":"core_rulebook",
                        "record_key":"scratch_duration_spell",
                        "upstream_lst":"scratch.lst",
                        "upstream_lst_sha256":"0",
                        "upstream_line":1,
                        "corpus_field":"DURATION:(CASTERLEVEL*10) minutes [D]",
                        "expected":{{"per_level":{expected_per_level},"unit":{expected_unit:?}}}
                    }}]}}"#
                ),
            )
            .unwrap();
            super::write_settled_spell_artifact(&root);
            ScratchDurationRoot { root }
        }
    }

    impl Drop for ScratchDurationRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }


}

