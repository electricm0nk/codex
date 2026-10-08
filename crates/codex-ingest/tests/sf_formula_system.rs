//! Starfinder 1e formula-system reader (SD-37 E3.2): `MODIFY:` / `MODIFYOTHER:` tokens, the
//! `VARIABLE:` declarations (`GLOBAL:`/`LOCAL:`/`CHANNEL:`), the `DATACONTROL:` functions and
//! dynamic scopes, the `DYNAMIC:` objects and the `DATATABLE:` tables.
//!
//! `docs/release/SD-37-starfinder-1e/epic-breakdown.md` E3.2: every SF `MODIFY*` token (CUI F-9:
//! 1,954 over all 10 PCC data trees) is mapped or refused by name. These tests read the pinned
//! oracle through `closure::corpus_root()` and skip, loudly, when the checkout is absent, like
//! every other corpus test in this crate.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use codex::rules_core::game_system::GameSystem;
use codex_ingest::pcgen_import::sheet_rule::closure;
use codex_ingest::pcgen_import::sheet_rule::formula_system::{
    Disposition, FsExpr, ModifyKind, SheetUse, census_json, read_formula_system, render_census,
};
use codex_ingest::pcgen_import::system_books::{BOOK_PCCS, EXCLUDED_BOOK_PCCS};

/// CUI F-9: tab fields starting `MODIFY:`/`MODIFYOTHER:` on non-comment lines of every SF `.lst`.
const SF_MODIFY_TOKENS_ALL_TREES: usize = 1954;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus_root() -> Option<PathBuf> {
    let root = closure::corpus_root();
    if root.join("starfinder").is_dir() {
        Some(root)
    } else {
        eprintln!("skipped: no starfinder/ under {} (run scripts/fetch-pcgen-oracle.sh)", root.display());
        None
    }
}

/// An implementation independent of the reader: walk every `.lst` under `dir`, count the tab
/// fields that start with `MODIFY:` or `MODIFYOTHER:` on lines that do not start with `#`.
fn naive_count(dir: &Path) -> usize {
    let mut n = 0;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "lst") {
                let text = String::from_utf8_lossy(&std::fs::read(&p).unwrap()).into_owned();
                for line in text.lines().filter(|l| !l.starts_with('#')) {
                    n += line.split('\t').filter(|f| f.starts_with("MODIFY:") || f.starts_with("MODIFYOTHER:")).count();
                }
            }
        }
    }
    n
}

#[test]
fn every_in_scope_modify_token_is_mapped_or_refused_by_name() {
    let Some(root) = corpus_root() else { return };
    let reading = read_formula_system(GameSystem::Starfinder1e, &root).expect("SF formula system reads");
    let mapped = reading.tokens.iter().filter(|t| matches!(t.disposition, Disposition::Mapped { .. })).count();
    let refused: Vec<_> = reading
        .tokens
        .iter()
        .filter_map(|t| match &t.disposition {
            Disposition::Refused { reason } => Some((t.file.clone(), t.line, reason.clone())),
            _ => None,
        })
        .collect();
    for (file, line, reason) in &refused {
        assert!(!reason.trim().is_empty(), "{file}:{line} refused without a name");
        eprintln!("refused {file}:{line}: {reason}");
    }
    assert_eq!(mapped + refused.len(), reading.tokens.len(), "mapped + refused = tokens");
    // The excluded books carry no MODIFY* token, so the in-scope subset is all 1,954 (the
    // all-trees figure is asserted against the independent walk below).
    assert_eq!(reading.tokens.len(), SF_MODIFY_TOKENS_ALL_TREES);
    assert_eq!(refused.len(), 0, "every SF MODIFY* token in the 8 in-scope books maps");
}

#[test]
fn an_independent_walk_agrees_on_the_token_counts() {
    let Some(root) = corpus_root() else { return };
    let reading = read_formula_system(GameSystem::Starfinder1e, &root).expect("SF formula system reads");
    assert_eq!(naive_count(&root.join("starfinder")), SF_MODIFY_TOKENS_ALL_TREES, "CUI F-9, all trees");
    let excluded: usize =
        EXCLUDED_BOOK_PCCS.books(GameSystem::Starfinder1e).iter().map(|e| naive_count(&root.join(e.book.dir))).sum();
    assert_eq!(excluded, 0, "no MODIFY* token lies in an excluded book");
    // Per file, the reader's count equals a naive count of that file.
    let mut per_file: BTreeMap<&str, usize> = BTreeMap::new();
    for t in &reading.tokens {
        *per_file.entry(t.file.as_str()).or_default() += 1;
    }
    for f in &reading.files_read {
        let text = String::from_utf8_lossy(&std::fs::read(root.join(f)).unwrap()).into_owned();
        let naive: usize = text
            .lines()
            .filter(|l| !l.starts_with('#'))
            .map(|l| l.split('\t').filter(|x| x.starts_with("MODIFY:") || x.starts_with("MODIFYOTHER:")).count())
            .sum();
        assert_eq!(per_file.get(f.as_str()).copied().unwrap_or(0), naive, "{f}");
    }
    // E3.1's 123 in-scope `.lst` under `starfinder/`, plus the shared `_universal/races.lst` the
    // Core Rulebook includes (`RACE:*/_universal/races.lst`; E3.1 receipt, "Figures").
    let under_sf = reading.files_read.iter().filter(|f| f.starts_with("starfinder/")).count();
    assert_eq!(under_sf, 123, "E3.1's in-scope .lst set");
    let outside: Vec<_> = reading.files_read.iter().filter(|f| !f.starts_with("starfinder/")).collect();
    assert_eq!(outside, vec!["_universal/races.lst"]);
    for book in BOOK_PCCS.books(GameSystem::Starfinder1e) {
        assert!(reading.files_read.iter().any(|f| f.starts_with(book.dir)), "{} read", book.dir);
    }
}

#[test]
fn declarations_channel_function_dynamic_scopes_and_datatables_are_read() {
    let Some(root) = corpus_root() else { return };
    let r = read_formula_system(GameSystem::Starfinder1e, &root).expect("SF formula system reads");
    let d = &r.declarations;
    // 23 GLOBAL + 28 LOCAL (scr__variables 50, saa__variables 1).
    assert_eq!(d.variables.len(), 51);
    assert_eq!(d.variables.iter().filter(|v| v.scope == "PC").count(), 23);
    let face = d.variables.iter().find(|v| v.name == "Face").unwrap();
    assert_eq!(face.format, "ORDEREDPAIR");
    let slot = d.variables.iter().find(|v| v.name == "UpgradeSlot").unwrap();
    assert_eq!((slot.scope.as_str(), slot.format.as_str()), ("PC.EQUIPMENT", "NUMBER"), "format defaults to NUMBER");
    assert_eq!(d.channels.len(), 1);
    assert_eq!((d.channels[0].scope.as_str(), d.channels[0].name.as_str()), ("PC.STAT", "StatScore"));
    assert_eq!(d.functions.iter().map(|f| f.name.as_str()).collect::<Vec<_>>(), vec!["d20Mod"]);
    assert_eq!(d.dynamic_scopes, vec!["MOVEMENT".to_string(), "VISION".to_string()]);
    assert_eq!(d.dynamic_objects.len(), 8);
    assert_eq!(d.datatables.len(), 2);
    let cost = &d.datatables[0];
    assert_eq!(cost.name, "Item Level Cost");
    assert_eq!(cost.columns, vec!["ItemLevel", "Price"]);
    assert_eq!(cost.rows.len(), 20);
    assert_eq!(cost.rows[0], vec!["1", "120"]);
    let enc = &d.datatables[1];
    assert_eq!(enc.name, "Encumbrance");
    assert_eq!(enc.columns.len(), 4);
    assert_eq!(enc.rows.len(), 30);
}

#[test]
fn the_stat_chain_maps_to_ability_compute_targets() {
    let Some(root) = corpus_root() else { return };
    let r = read_formula_system(GameSystem::Starfinder1e, &root).expect("SF formula system reads");
    let stats: Vec<_> = r.tokens.iter().filter(|t| t.file.ends_with("scr__stats.lst")).collect();
    assert_eq!(stats.len(), 24);
    let mut roles = BTreeMap::<String, usize>::new();
    for t in &stats {
        let Disposition::Mapped { role, sheet, value, .. } = &t.disposition else { panic!("{t:?}") };
        assert_eq!(*sheet, SheetUse::Compute, "{t:?}");
        *roles.entry(role.clone()).or_default() += 1;
        if t.var == "Score" {
            assert_eq!(
                *value,
                FsExpr::Call { name: "input".into(), args: vec![FsExpr::Str("STATSCORE".into())] },
                "the score is the stat channel's input"
            );
        }
        if t.var == "Mod" {
            assert_eq!(*value, FsExpr::Call { name: "d20Mod".into(), args: vec![FsExpr::Var("Score".into())] });
        }
    }
    assert_eq!(
        roles,
        BTreeMap::from([
            ("ability_modifier".to_string(), 6),
            ("ability_modifier_alias".to_string(), 6),
            ("ability_score".to_string(), 6),
            ("ability_score_alias".to_string(), 6),
        ])
    );
}

#[test]
fn modifyother_tokens_are_speed_on_a_declared_movement() {
    let Some(root) = corpus_root() else { return };
    let r = read_formula_system(GameSystem::Starfinder1e, &root).expect("SF formula system reads");
    let other: Vec<_> = r.tokens.iter().filter(|t| t.kind == ModifyKind::ModifyOther).collect();
    assert_eq!(other.len(), 186);
    let groupings: BTreeSet<_> = other.iter().map(|t| t.grouping.clone().unwrap()).collect();
    assert_eq!(groupings, ["Climb", "Fly", "Swim", "Walk"].into_iter().map(String::from).collect());
    for t in &other {
        assert_eq!(t.scope, "PC.MOVEMENT");
        let Disposition::Mapped { role, sheet, .. } = &t.disposition else { panic!("{t:?}") };
        assert_eq!((role.as_str(), *sheet), ("movement_speed", SheetUse::Compute));
    }
    let with_priority = r.tokens.iter().filter(|t| t.priority.is_some()).count();
    assert_eq!(with_priority, 22, "PRIORITY= associations read");
}

#[test]
fn variable_names_resolve_case_insensitively_like_pcgen() {
    // PCGen-Formula VariableManager keys variable definitions in a CaseInsensitiveMap, and
    // input("STATSCORE") reaches the channel declared `StatScore`.
    let Some(root) = corpus_root() else { return };
    let r = read_formula_system(GameSystem::Starfinder1e, &root).expect("SF formula system reads");
    let lower: Vec<_> = r.tokens.iter().filter(|t| t.var == "Race_reach").collect();
    assert_eq!(lower.len(), 1);
    let Disposition::Mapped { variable, role, sheet, .. } = &lower[0].disposition else { panic!("{:?}", lower[0]) };
    assert_eq!((variable.as_str(), role.as_str(), *sheet), ("Race_Reach", "reach", SheetUse::Print));
}

#[test]
fn print_roles_cover_equipment_and_race_facts() {
    let Some(root) = corpus_root() else { return };
    let r = read_formula_system(GameSystem::Starfinder1e, &root).expect("SF formula system reads");
    let mut by_role = BTreeMap::<(String, SheetUse), usize>::new();
    for t in &r.tokens {
        if let Disposition::Mapped { role, sheet, .. } = &t.disposition {
            *by_role.entry((role.clone(), *sheet)).or_default() += 1;
        }
    }
    let item_level = by_role.get(&("item_level".to_string(), SheetUse::Print)).copied();
    assert_eq!(item_level, Some(1273));
    let total: usize = by_role.values().sum();
    assert_eq!(total, SF_MODIFY_TOKENS_ALL_TREES);
}

#[test]
fn the_committed_census_is_fresh() {
    let Some(root) = corpus_root() else { return };
    let r = read_formula_system(GameSystem::Starfinder1e, &root).expect("SF formula system reads");
    let fresh = render_census(&census_json(&r));
    let path = repo_root().join("docs/release/SD-37-starfinder-1e/artifacts/epic_3/formula-system/sf-formula-census.json");
    let committed = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        committed == fresh,
        "{} is stale or absent: run `cargo run --locked -p codex-ingest --bin sf_formula_census`",
        path.display()
    );
}
