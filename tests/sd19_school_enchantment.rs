//! SD-19 §2.4 spell-school card: Enchantment.
//!
//! Proves every real-corpus Enchantment spell in `cr_spells.lst` (60
//! records, independently confirmed via `grep -c "SCHOOL:Enchantment"
//! cr_spells.lst`, and the "Aid" record independently confirmed via
//! `grep -n "^Aid\b" cr_spells.lst` before this test was written, per the
//! loop instruction's Step 4 corpus-existence check) is (a) resolvable via
//! `spell_id_resolve`, and (b) present in
//! `CorpusPilotReceipt.corpus_derived.school_coverage[Enchantment].spells`
//! after a call to `compute_pilot_with_corpus`.
//!
//! Reads the real PCGen corpus directly (the per-cycle `CORPUS_ROOT`
//! pattern from `decisions.md` §6.6, distinct from the slice-ship
//! `sd19_seam_shapes_correctness` test's checked-in fixtures) rather than
//! hand-copying 60 fixture files. Skips with a documented `eprintln!` when
//! `CORPUS_ROOT` is unset, matching SD-17-B-4/B-5's skip semantics.

use std::path::PathBuf;

use codex::pcgen_import::ir_converter::convert_spell_record;
use codex::pcgen_import::lst_parser::spell::parse_lst_spell_file;
use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    SpellSelection,
};
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::crb::spell_list::Pf1SchoolId;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};
use codex::rules_core::spell_resolver::spell_id_resolve;
use codex::rules_core::support_state_matrix::{
    EvidenceTier, MatrixSubjectType, SupportState, seeded_current_truth,
};

fn corpus_root() -> Option<PathBuf> {
    match std::env::var("CORPUS_ROOT") {
        Ok(value) => {
            let path = PathBuf::from(value);
            if path.is_dir() { Some(path) } else { None }
        }
        Err(_) => None,
    }
}

fn cr_spells_path(root: &std::path::Path) -> PathBuf {
    root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst")
}

fn base_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd19_school_enchantment".to_string()),
        source_package_id: "sd19_school_enchantment".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "wizard".to_string(),
                level: 10,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn enchantment_school_spell_count_matches_real_corpus() {
    let Some(root) = corpus_root() else {
        eprintln!(
            "CORPUS_ROOT not set or not a directory; skipping (set \
             CORPUS_ROOT=$HOME/workspace/repos/pcgen/data to enable)"
        );
        return;
    };
    let cr_spells = cr_spells_path(&root);
    if !cr_spells.is_file() {
        eprintln!("canonical cr_spells.lst not present at {}; skipping", cr_spells.display());
        return;
    }

    let spell_file = parse_lst_spell_file(&cr_spells).expect("cr_spells.lst must parse");
    let enchantment: Vec<_> = spell_file
        .records
        .iter()
        .filter(|r| r.school.as_deref() == Some("Enchantment"))
        .collect();

    // Pinned to the real corpus's own count, independently grepped before
    // this test was written (see module doc comment).
    assert_eq!(
        enchantment.len(),
        60,
        "expected 60 SCHOOL:Enchantment records in cr_spells.lst"
    );
    assert!(
        enchantment.iter().any(|r| r.name == "Aid"),
        "expected 'Aid' among the resolved Enchantment records"
    );
}

#[test]
fn every_enchantment_spell_resolves_and_reaches_school_coverage() {
    let Some(root) = corpus_root() else {
        eprintln!(
            "CORPUS_ROOT not set or not a directory; skipping (set \
             CORPUS_ROOT=$HOME/workspace/repos/pcgen/data to enable)"
        );
        return;
    };
    let cr_spells = cr_spells_path(&root);
    if !cr_spells.is_file() {
        eprintln!("canonical cr_spells.lst not present at {}; skipping", cr_spells.display());
        return;
    }

    let spell_file = parse_lst_spell_file(&cr_spells).expect("cr_spells.lst must parse");
    let enchantment_records: Vec<_> = spell_file
        .records
        .iter()
        .filter(|r| r.school.as_deref() == Some("Enchantment"))
        .collect();
    assert!(
        !enchantment_records.is_empty(),
        "corpus-existence check must find Enchantment records"
    );

    let source_ref = SourceRef {
        lst_file: cr_spells.display().to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd19_school_enchantment", source_ref);
    for record in &enchantment_records {
        corpus.push(convert_spell_record(record));
    }

    // (a) every Enchantment spell resolves via spell_id_resolve.
    for record in &enchantment_records {
        assert!(
            spell_id_resolve(&record.name, RuleSetId::Crb, &corpus).is_some(),
            "expected spell_id_resolve to resolve Enchantment spell '{}'",
            record.name
        );
    }

    // (b) every Enchantment spell is present in
    // corpus_derived.school_coverage[Enchantment].spells after a call to
    // compute_pilot_with_corpus. (c) each selection is tagged with a real
    // acquisition_mode on the input side.
    let mut input = base_input();
    for record in &enchantment_records {
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: record.name.clone(),
            source_class_id: "wizard".to_string(),
            acquisition_mode: AcquisitionMode::Known,
        });
    }

    let receipt = compute_pilot_with_corpus(&input, &corpus);
    let coverage = receipt
        .corpus_derived
        .school_coverage
        .get(&Pf1SchoolId::Enchantment)
        .expect("expected an Enchantment entry in corpus_derived.school_coverage");

    let mut expected: Vec<String> = enchantment_records.iter().map(|r| r.name.clone()).collect();
    expected.sort();
    assert_eq!(
        coverage.spells, expected,
        "expected school_coverage[Enchantment].spells to carry every real-corpus Enchantment spell"
    );

    // (d) the row's grounding reaches the foundation slice's table store
    // through the bootstrap table cell.
    assert!(
        coverage.table_cell.is_some(),
        "expected school_coverage[Enchantment].table_cell to resolve through the foundation slice's table store"
    );
}

#[test]
fn enchantment_matrix_row_reflects_the_grounded_reachability_proof() {
    let matrix = seeded_current_truth();
    let row = matrix
        .rows
        .iter()
        .find(|r| r.subject_type == MatrixSubjectType::School(Pf1SchoolId::Enchantment))
        .expect("expected a School(Enchantment) row in the seeded matrix");

    assert_eq!(row.support_state, SupportState::Supported);
    assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    assert!(
        row.grounding_ref.contains("sd19_school_enchantment"),
        "expected the row's grounding_ref to cite this cycle's proof test, got: {}",
        row.grounding_ref
    );
    assert!(
        !row.blocker_or_lossiness_note.is_empty(),
        "expected a non-empty blocker_or_lossiness_note on a Partial row"
    );
}
