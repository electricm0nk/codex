//! SD-36 Epic F1 rule-gap investigation (`epic-f-class-completion.md`, Review log; receipt
//! at `docs/release/SD-36-consolidation/artifacts/epic-f/stage4/rule-gap-receipt.md`) --
//! `_report.json`'s `rules_written` (71,862) read 369 more rules than
//! `rules_core::corpus_loader::load_sheet_rules` actually loads into the live package
//! (71,493). A script over the tracked `data/sheet_rules/` (the same directory this repo's
//! converter writes and the live loader reads) found every one of the 369 missing rules is a
//! same-id collision: `SheetRulePackage::insert_rule`'s `BTreeMap<RuleId, SheetRule>` silently
//! keeps only the LAST write for a given id, and 306 ids each had 2-4 separate `SheetRule`
//! entries competing for it -- every single one under a `#naturalN` suffix
//! (`crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs`'s `NATURALATTACKS` arm).
//!
//! Root cause: that arm used to suffix each emitted line `natural{i}` where `i` is the
//! enumerate position WITHIN one `v.split('|')` call -- reset to 0 every time the arm runs.
//! A record whose source carries the `NATURALATTACKS:` tag more than once on the same row
//! (real shape: `bestiary:monster:chimera`'s `b1_races.lst:69` carries it twice, once per
//! draconic head, each a single-entry occurrence) hits the arm twice, both occurrences
//! minting `#natural0` -- one head's bite rule silently overwrites the other's in the loaded
//! package, and the shadowed one never reaches a sheet (a chimera character/NPC prints only
//! one dragon-head bite, not both). This is the "content the converter produced never
//! reaches a sheet" shape `docs/governance/no-stub-mvp-doctrine.md` calls a live defect, not
//! a benign report/loader mismatch.
//!
//! The fix: the arm now suffixes with `acc.lines.len()`, the record-global running line
//! count every OTHER multi-emit arm in this match already uses for exactly this reason
//! (`weapon{}` / `bonus{}` / `spell{}_`) -- unique per record no matter how many times the
//! token occurs. These are the live-corpus, real-oracle tests: `convert_record` over the
//! pinned tree (never writing `data/sheet_rules/` to disk -- this checkout's TRACKED package
//! is deliberately left un-regenerated this stage; `corpus_loader.rs`'s own tests pin its
//! current, pre-fix duplicate count separately).
//!
//! **n=2 units and why:** `bestiary:monster:chimera` (`b1_races.lst:69`) and
//! `bestiary:monster:demon_glabrezu` (`b1_races.lst:95`) are two of the 306 real ids the scan
//! found colliding -- Chimera exercises the 2-way collision (two dragon-head bites), Glabrezu
//! the 3-way collision (bite + 2 claw + 2 pincer, three SEPARATE `NATURALATTACKS:` tab
//! fields on one line) so both group sizes the scan found (2 and 3) are covered by a real
//! record, not a synthetic one.

use std::collections::HashSet;
use std::sync::OnceLock;

use codex_ingest::pcgen_import::sheet_rule::closure::{corpus_root, Closure, PinnedTree};
use codex_ingest::pcgen_import::sheet_rule::convert::{convert_record, Converted};
use codex_ingest::pcgen_import::sheet_rule::ctx::CorpusIndex;
use codex_ingest::pcgen_import::sheet_rule::{build_index, load_population};

fn repo() -> std::path::PathBuf {
    codex_ingest::repo_root()
}

struct Shared {
    tree: PinnedTree,
    index: CorpusIndex,
    closures: Vec<Closure>,
}

fn shared() -> &'static Shared {
    static S: OnceLock<Shared> = OnceLock::new();
    S.get_or_init(|| {
        let tree = PinnedTree::load(&corpus_root()).expect("pinned corpus checkout present (scripts/fetch-pcgen-oracle.sh)");
        let records = load_population(&repo(), &tree).expect("docs/work-inventory.json and data/corpus readable");
        let (index, closures) = build_index(&tree, records);
        Shared { tree, index, closures }
    })
}

fn convert_unit(id: &str) -> Converted {
    let s = shared();
    let pos = s.index.records.iter().position(|r| r.id == id).unwrap_or_else(|| panic!("unit {id} is in docs/work-inventory.json"));
    convert_record(&s.tree, &s.index, &s.index.records[pos], &s.closures[pos])
}

/// F1.9 rule-gap fix: every `SheetRule` id a real record converts to is unique WITHIN that
/// record, so nothing downstream of `convert_record` -- `SheetRulePackage::insert_rule`'s
/// `BTreeMap` foremost -- can silently shadow one against another.
fn assert_all_ids_unique(unit: &str, converted: &Converted) {
    assert!(converted.refusals.is_empty(), "{unit} refusals: {:?}", converted.refusals);
    let mut seen: HashSet<&str> = HashSet::new();
    let mut dupes: Vec<&str> = Vec::new();
    for rule in &converted.rules {
        if !seen.insert(rule.id.as_str()) {
            dupes.push(rule.id.as_str());
        }
    }
    assert!(
        dupes.is_empty(),
        "{unit}: convert_record emitted colliding ids {dupes:?} -- one of them silently \
         shadows another in SheetRulePackage's BTreeMap and never reaches a live sheet: {:#?}",
        converted.rules.iter().map(|r| &r.id).collect::<Vec<_>>()
    );
}

#[test]
fn chimera_two_head_bites_no_longer_collide_on_natural0() {
    let chimera = convert_unit("bestiary:monster:chimera");
    assert_all_ids_unique("chimera", &chimera);
    let natural0_ids: Vec<&str> =
        chimera.rules.iter().map(|r| r.id.as_str()).filter(|id| id.ends_with("#natural0") || id.contains("#natural0#")).collect();
    // Both dragon-head bites must survive under DISTINCT `#naturalN` ids -- not one shared
    // `#natural0` where only the last conversion wins.
    let bite_labels: Vec<&str> = chimera.rules.iter().filter(|r| r.label.to_ascii_lowercase().contains("bite")).map(|r| r.label.as_str()).collect();
    assert_eq!(bite_labels.len(), 2, "chimera must convert both dragon-head and lion-head Bite lines, not one shadowing the other: {bite_labels:?}");
    assert!(natural0_ids.len() <= 1, "at most one line may legitimately hold the bare `#natural0` suffix once ids are unique: {natural0_ids:?}");
}

#[test]
fn glabrezu_three_way_collision_no_longer_collides_on_natural0() {
    let glabrezu = convert_unit("bestiary:monster:demon_glabrezu");
    assert_all_ids_unique("demon_glabrezu", &glabrezu);
    let natural_attack_labels: Vec<&str> = glabrezu
        .rules
        .iter()
        .filter(|r| ["bite", "claw", "pincer"].iter().any(|kw| r.label.to_ascii_lowercase().contains(kw)))
        .map(|r| r.label.as_str())
        .collect();
    assert_eq!(
        natural_attack_labels.len(),
        3,
        "demon_glabrezu must convert its Bite, 2 Claw and 2 Pincer natural-attack lines as three \
         SEPARATE rules, not collapsed onto one shared #natural0 id: {natural_attack_labels:?}"
    );
}
