//! SD-17 Slice A acceptance tests for PCC include-graph resolution.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use codex::pcgen_import::include_resolver::{
    IncludeDiagnosticKind, resolve_pcc_includes, resolve_pcc_includes_from,
};

const CORE_RULEBOOK_MINIMAL: &str = include_str!("fixtures/pcc/core_rulebook_minimal.pcc");

struct TestCorpus {
    root: PathBuf,
}

impl TestCorpus {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "codex_sd17_include_graph_{}_{}_{}",
            name,
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&root).expect("create test corpus root");
        Self { root }
    }

    fn write(&self, relative_path: &str, contents: &str) {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create fixture parent dir");
        }
        fs::write(path, contents).expect("write fixture file");
    }

    fn touch(&self, relative_path: &str) {
        self.write(relative_path, "# fixture\n");
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn path(&self, relative_path: &str) -> PathBuf {
        self.root.join(relative_path)
    }
}

impl Drop for TestCorpus {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[test]
fn resolves_linear_chain_to_deterministic_pcc_and_lst_order() {
    let corpus = TestCorpus::new("linear");
    corpus.write(
        "entry.pcc",
        "PCC:chapters/first.pcc\nCLASS:root_classes.lst\n",
    );
    corpus.write(
        "chapters/first.pcc",
        "PCC:../second.pcc\nRACE:first_races.lst\n",
    );
    corpus.write("second.pcc", "SPELL:second_spells.lst\n");
    corpus.touch("root_classes.lst");
    corpus.touch("chapters/first_races.lst");
    corpus.touch("second_spells.lst");

    let resolution = resolve_pcc_includes(&corpus.path("entry.pcc")).expect("resolve succeeds");

    assert_eq!(
        resolution
            .pcc_files
            .iter()
            .map(|file| rel(corpus.root(), &file.path))
            .collect::<Vec<_>>(),
        vec!["entry.pcc", "chapters/first.pcc", "second.pcc"]
    );
    assert_eq!(resolution.pcc_edges.len(), 2);
    assert_eq!(
        resolution
            .lst_files
            .iter()
            .map(|file| rel(corpus.root(), &file.path))
            .collect::<Vec<_>>(),
        vec![
            "second_spells.lst",
            "chapters/first_races.lst",
            "root_classes.lst"
        ]
    );
    assert!(resolution.diagnostics.is_empty());
}

#[test]
fn resolves_diamond_includes_once_while_preserving_all_directed_edges() {
    let corpus = TestCorpus::new("diamond");
    corpus.write("entry.pcc", "PCC:left.pcc\nPCC:right.pcc\n");
    corpus.write("left.pcc", "PCC:shared/leaf.pcc\nCLASS:left.lst\n");
    corpus.write("right.pcc", "PCC:@/shared/leaf.pcc\nCLASS:right.lst\n");
    corpus.write("shared/leaf.pcc", "CLASS:shared.lst\n");
    corpus.touch("left.lst");
    corpus.touch("right.lst");
    corpus.touch("shared/shared.lst");

    let resolution = resolve_pcc_includes(&corpus.path("entry.pcc")).expect("resolve succeeds");

    assert_eq!(
        resolution
            .pcc_files
            .iter()
            .map(|file| rel(corpus.root(), &file.path))
            .collect::<Vec<_>>(),
        vec!["entry.pcc", "left.pcc", "shared/leaf.pcc", "right.pcc"]
    );
    assert_eq!(resolution.pcc_edges.len(), 4);
    assert_eq!(
        rel(corpus.root(), &resolution.pcc_edges[3].source_path),
        "right.pcc"
    );
    assert_eq!(
        rel(corpus.root(), &resolution.pcc_edges[3].target_path),
        "shared/leaf.pcc"
    );
    assert_eq!(
        resolution
            .lst_files
            .iter()
            .map(|file| rel(corpus.root(), &file.path))
            .collect::<Vec<_>>(),
        vec!["shared/shared.lst", "left.lst", "right.lst"]
    );
    assert!(resolution.diagnostics.is_empty());
}

#[test]
fn reports_include_cycles_without_recursing_forever() {
    let corpus = TestCorpus::new("cycle");
    corpus.write("entry.pcc", "PCC:a.pcc\n");
    corpus.write("a.pcc", "PCC:b.pcc\n");
    corpus.write("b.pcc", "PCC:entry.pcc\n");

    let resolution = resolve_pcc_includes(&corpus.path("entry.pcc")).expect("resolve succeeds");

    assert_eq!(resolution.pcc_files.len(), 3);
    assert_eq!(resolution.pcc_edges.len(), 3);
    assert_eq!(resolution.diagnostics.len(), 1);
    let diagnostic = &resolution.diagnostics[0];
    assert_eq!(diagnostic.kind, IncludeDiagnosticKind::CycleDetected);
    assert_eq!(rel(corpus.root(), &diagnostic.source_path), "b.pcc");
    assert_eq!(diagnostic.line_number, Some(1));
    assert!(
        diagnostic
            .message
            .contains("entry.pcc -> a.pcc -> b.pcc -> entry.pcc"),
        "cycle path should be diagnostic evidence: {}",
        diagnostic.message
    );
}

#[test]
fn reports_missing_include_targets_without_panicking() {
    let corpus = TestCorpus::new("missing");
    corpus.write("entry.pcc", "PCC:missing/nope.pcc\nCLASS:survives.lst\n");
    corpus.touch("survives.lst");

    let resolution = resolve_pcc_includes(&corpus.path("entry.pcc")).expect("resolve succeeds");

    assert_eq!(resolution.pcc_files.len(), 1);
    assert_eq!(resolution.pcc_edges.len(), 1);
    assert_eq!(resolution.lst_files.len(), 1);
    assert_eq!(
        rel(corpus.root(), &resolution.pcc_edges[0].target_path),
        "missing/nope.pcc"
    );
    assert_eq!(resolution.diagnostics.len(), 1);
    let diagnostic = &resolution.diagnostics[0];
    assert_eq!(diagnostic.kind, IncludeDiagnosticKind::MissingTarget);
    assert_eq!(rel(corpus.root(), &diagnostic.source_path), "entry.pcc");
    assert_eq!(diagnostic.line_number, Some(1));
    assert!(
        diagnostic.message.contains("missing/nope.pcc"),
        "missing target should be diagnostic evidence: {}",
        diagnostic.message
    );
}

#[test]
fn resolves_core_rulebook_minimal_fixture_to_flat_lst_list_and_parser_diagnostics() {
    let corpus = TestCorpus::new("core-rulebook-minimal");
    corpus.write(
        "pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc",
        CORE_RULEBOOK_MINIMAL,
    );
    corpus.write(
        "homebrew/conversion_support/conversion_support.pcc",
        "ABILITY:cs_abilities.lst\n",
    );
    corpus.touch("homebrew/conversion_support/cs_abilities.lst");
    corpus.write(
        "pathfinder/paizo/roleplaying_game/core_essentials/_core_essentials.pcc",
        concat!(
            "DATACONTROL:*/_universal/datacontrols_no_align.lst\n",
            "RACE:*/_universal/races.lst\n",
            "SKILL:ce_skills.lst\n",
            "SPELL:ce_spells.lst\n",
        ),
    );
    corpus.touch("_universal/datacontrols_no_align.lst");
    corpus.touch("_universal/races.lst");
    corpus.touch("pathfinder/paizo/roleplaying_game/core_essentials/ce_skills.lst");
    corpus.touch("pathfinder/paizo/roleplaying_game/core_essentials/ce_spells.lst");
    for ancestry in [
        "dwarf", "elf", "gnome", "half_elf", "half_orc", "halfling", "human",
    ] {
        corpus.write(
            &format!(
                "pathfinder/paizo/roleplaying_game/core_essentials/races/{ancestry}/_race.pcc"
            ),
            "# no LST refs in this minimal fixture\n",
        );
    }
    corpus.touch("pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst");
    corpus.touch("pathfinder/paizo/roleplaying_game/core_rulebook/cr_races.lst");
    corpus.touch("pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst");

    let resolution = resolve_pcc_includes_from(
        corpus.root(),
        &corpus.path("pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc"),
    )
    .expect("fixture resolves");

    assert_eq!(resolution.pcc_files.len(), 10);
    assert_eq!(resolution.pcc_edges.len(), 9);
    assert_eq!(resolution.lst_files.len(), 8);
    assert_eq!(
        resolution
            .lst_files
            .iter()
            .map(|file| rel(corpus.root(), &file.path))
            .collect::<Vec<_>>(),
        vec![
            "homebrew/conversion_support/cs_abilities.lst",
            "_universal/datacontrols_no_align.lst",
            "_universal/races.lst",
            "pathfinder/paizo/roleplaying_game/core_essentials/ce_skills.lst",
            "pathfinder/paizo/roleplaying_game/core_essentials/ce_spells.lst",
            "pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst",
            "pathfinder/paizo/roleplaying_game/core_rulebook/cr_races.lst",
            "pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst",
        ]
    );
    assert_eq!(resolution.diagnostics.len(), 1);
    assert_eq!(
        resolution.diagnostics[0].kind,
        IncludeDiagnosticKind::MalformedInclude
    );
    assert_eq!(resolution.diagnostics[0].line_number, Some(22));
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn resolves_real_core_rulebook_pcc_from_local_pcgen_corpus() {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source =
        corpus_root.join("pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc");

    let resolution =
        resolve_pcc_includes_from(&corpus_root, &source).expect("real corpus resolves");

    assert_eq!(resolution.pcc_files.len(), 10);
    assert_eq!(resolution.pcc_edges.len(), 9);
    assert_eq!(resolution.lst_files.len(), 141);
    assert!(
        resolution.diagnostics.is_empty(),
        "real core_rulebook.pcc should have no include diagnostics: {:?}",
        resolution.diagnostics
    );
}
