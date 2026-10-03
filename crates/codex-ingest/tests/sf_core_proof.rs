//! Starfinder 1e proof generation (SD-37 E3.4 Core Rulebook, E3.5 all 8 in-scope books;
//! `decisions.md §4`).
//!
//! The converter was tuned on one book (E3.4), then went wide in one batch (E3.5). These tests
//! pin the SF path end to end on the pinned oracle:
//!
//! 1. the pinned tree reads exactly the `.lst` files the converted books' `.pcc` include, so an
//!    excluded book (`core/_society`, SSRGG, LPJ) cannot be read (`decisions.md §6`);
//! 2. the population is the SF work inventory's units of the converted books (E0.3), every one
//!    joined to its own source row;
//! 3. the overloaded PCGen fields route by the SF mapping table (`decisions.md §8`, E3.3):
//!    `BONUS:HP|ALTHP` is Stamina, `BONUS:HP|CURRENTMAX` is Hit Points, a `BONUS:HP` token no
//!    table term claims is a named degradation, and `BONUS:COMBAT|AC` splits into EAC and KAC by
//!    the game mode's own `ACTYPE:EAC` / `ACTYPE:KAC` rows;
//! 4. the generated package and corpus on disk agree with the inventory and print no Starfinder
//!    product-identity term (`pi_screening::classify_field_sf`, E0.2's term set).
//!
//! They skip, loudly, when the oracle checkout is absent, like every corpus test in this crate.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use codex::rules_core::game_system::GameSystem;
use codex::rules_core::pi_screening::classify_field_sf;
use codex::rules_core::shape_b_v1::License;
use codex_ingest::pcgen_import::sheet_rule::closure::{self, PinnedTree};
use codex_ingest::pcgen_import::sheet_rule::{convert_one_for, load_population};
use codex_ingest::pcgen_import::system_books::{BOOK_PCCS, CONVERTED_BOOKS, EXCLUDED_BOOK_PCCS, resolve_converted_book_includes};

const SF_INVENTORY: &str = "docs/work-inventory.starfinder-1e.json";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn oracle_present() -> bool {
    let root = closure::corpus_root().join("starfinder");
    if root.is_dir() {
        true
    } else {
        eprintln!("skipped: no {} (run scripts/fetch-pcgen-oracle.sh)", root.display());
        false
    }
}

fn sf_tree() -> PinnedTree {
    PinnedTree::load_for(GameSystem::Starfinder1e, &closure::corpus_root()).expect("the Starfinder tree loads")
}

/// The converted books' ids as the population names them: the last segment of each book dir.
fn converted_book_ids() -> BTreeSet<String> {
    CONVERTED_BOOKS
        .books(GameSystem::Starfinder1e)
        .iter()
        .map(|b| b.dir.rsplit('/').next().unwrap().to_string())
        .collect()
}

/// `(id, book dir relative to starfinder/)` of every inventory unit of a converted book.
fn inventory_units_of_converted_books() -> Vec<(String, String)> {
    let text = std::fs::read_to_string(repo_root().join(SF_INVENTORY)).expect("SF inventory");
    let inv: serde_json::Value = serde_json::from_str(&text).expect("SF inventory parses");
    let dirs: BTreeSet<String> = CONVERTED_BOOKS
        .books(GameSystem::Starfinder1e)
        .iter()
        .map(|b| b.dir.trim_start_matches("starfinder/").to_string())
        .collect();
    inv["units"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|u| dirs.contains(u["book"].as_str().unwrap()))
        .map(|u| (u["id"].as_str().unwrap().to_string(), u["book"].as_str().unwrap().to_string()))
        .collect()
}

fn targets(rules: &serde_json::Value) -> Vec<String> {
    rules
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|r| r.get("target"))
        .map(|t| match t {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        })
        .collect()
}

fn converted(id: &str) -> (serde_json::Value, BTreeMap<String, Vec<String>>) {
    let (c, _rows) = convert_one_for(&repo_root(), GameSystem::Starfinder1e, id).unwrap_or_else(|e| panic!("{id}: {e}"));
    (serde_json::to_value(&c.rules).unwrap(), c.defects.clone())
}

/// E3.4 proved the method on the Core Rulebook alone; E3.5 goes wide in one batch
/// (`decisions.md §4`): the converted books are every registered (licence: include) book.
#[test]
fn the_converted_books_are_every_registered_book_and_never_an_excluded_book() {
    let dirs: Vec<&str> = CONVERTED_BOOKS.books(GameSystem::Starfinder1e).iter().map(|b| b.dir).collect();
    let registered: Vec<&str> = BOOK_PCCS.books(GameSystem::Starfinder1e).iter().map(|b| b.dir).collect();
    assert_eq!(dirs, registered, "every licence-include book is converted");
    assert_eq!(
        dirs,
        [
            "starfinder/paizo/core",
            "starfinder/paizo/armory",
            "starfinder/paizo/character_operations_manual",
            "starfinder/paizo/pact_worlds",
            "starfinder/paizo/near_space",
            "starfinder/paizo/alien_archive",
            "starfinder/paizo/alien_archive_2",
            "starfinder/paizo/alien_archive_3",
        ]
    );
    for ex in EXCLUDED_BOOK_PCCS.books(GameSystem::Starfinder1e) {
        assert!(!dirs.contains(&ex.book.dir), "{} is excluded", ex.book.dir);
    }
    assert!(CONVERTED_BOOKS.books(GameSystem::Pathfinder1e).is_empty(), "Pathfinder is read by directory");
}

#[test]
fn the_starfinder_tree_reads_exactly_the_converted_books_includes() {
    if !oracle_present() {
        return;
    }
    let tree = sf_tree();
    assert_eq!(tree.system, GameSystem::Starfinder1e);
    assert_eq!(tree.book_paths.keys().cloned().collect::<BTreeSet<_>>(), converted_book_ids());
    let root = closure::corpus_root();
    let includes = resolve_converted_book_includes(GameSystem::Starfinder1e, &root).expect("includes resolve");
    let want: BTreeSet<String> = includes
        .lst_files
        .iter()
        .map(|f| f.path.strip_prefix(&root).unwrap().to_string_lossy().replace('\\', "/"))
        .collect();
    let got: BTreeSet<String> = tree.files.iter().map(|f| f.rel_path.clone()).collect();
    assert_eq!(got, want, "the tree is the include list, nothing more and nothing less");
    let society: Vec<&String> = got.iter().filter(|p| p.contains("/_society/") || p.contains("starfinder_society_rules") || p.contains("lpj_design")).collect();
    assert!(society.is_empty(), "an excluded book was read: {society:?}");
}

#[test]
fn the_starfinder_population_is_the_inventory_units_of_the_converted_books() {
    if !oracle_present() {
        return;
    }
    let tree = sf_tree();
    let records = load_population(&repo_root(), &tree).expect("population loads");
    let want = inventory_units_of_converted_books();
    // E0.3's in-scope total over the 8 books (artifacts/epic_0/E0.3_cycle_receipt.md), and its
    // per-book split (Python over docs/work-inventory.starfinder-1e.json `units[].book`).
    assert_eq!(want.len(), 8582, "E0.3 counted 8,582 units in the 8 in-scope books");
    let mut per_book: BTreeMap<&str, usize> = BTreeMap::new();
    for (_, book) in &want {
        *per_book.entry(book.as_str()).or_default() += 1;
    }
    let expected: BTreeMap<&str, usize> = [
        ("paizo/core", 3105),
        ("paizo/armory", 2922),
        ("paizo/character_operations_manual", 1000),
        ("paizo/pact_worlds", 428),
        ("paizo/near_space", 351),
        ("paizo/alien_archive", 300),
        ("paizo/alien_archive_2", 298),
        ("paizo/alien_archive_3", 178),
    ]
    .into_iter()
    .collect();
    assert_eq!(per_book, expected, "E0.3's per-book unit counts");
    assert_eq!(records.len(), want.len());
    let ids: BTreeSet<&str> = records.iter().map(|r| r.id.as_str()).collect();
    for (id, _) in &want {
        assert!(ids.contains(id.as_str()), "{id} is not in the population");
    }
    let books: BTreeSet<&str> = records.iter().map(|r| r.book.as_str()).collect();
    assert_eq!(books.iter().map(|s| s.to_string()).collect::<BTreeSet<_>>(), converted_book_ids());
    let unjoined: Vec<&str> = records.iter().filter(|r| r.rel_path.is_empty()).map(|r| r.id.as_str()).take(10).collect();
    assert!(unjoined.is_empty(), "units with no source row: {unjoined:?}");
}

/// `BONUS:HP|CURRENTMAX|6*SoldierLVL` is Hit Points and `BONUS:HP|ALTHP|7*SoldierLVL` is
/// Stamina (`sf-mapping-table.v1.json` rows `hit_points` / `stamina`); the Pathfinder reading
/// put both into `Hp`.
#[test]
fn class_hit_point_pools_route_by_the_sf_mapping_table() {
    if !oracle_present() {
        return;
    }
    let (rules, _) = converted("core:class:soldier");
    let t = targets(&rules);
    assert_eq!(t.iter().filter(|x| *x == "Hp").count(), 1, "one Hit Points line: {t:?}");
    assert_eq!(t.iter().filter(|x| *x == "Stamina").count(), 1, "one Stamina line: {t:?}");
    // The coefficient each pool carries is the oracle's: 6 per Soldier level to Hit Points, 7 to
    // Stamina (a swapped reading, M1, prints 7 and 6).
    let value_of = |target: &str| -> String {
        rules.as_array().unwrap().iter().find(|r| r.get("target").and_then(|t| t.as_str()) == Some(target)).map(|r| r["value"].to_string()).unwrap()
    };
    assert_eq!(value_of("Hp"), r#"{"Number":{"Mul":[{"Const":6},{"ClassLevel":"soldier"}]}}"#);
    assert_eq!(value_of("Stamina"), r#"{"Number":{"Mul":[{"Const":7},{"ClassLevel":"soldier"}]}}"#);
    let (rules, _) = converted("core:feat:toughness");
    let t = targets(&rules);
    assert!(t.contains(&"Stamina".to_string()), "Toughness feeds Stamina (BONUS:HP|ALTHP|TL): {t:?}");
    assert!(!t.contains(&"Hp".to_string()), "Toughness does not feed Hit Points: {t:?}");
}

/// A `BONUS:HP` token no table term claims is never guessed into a pool (`SD-d`): the table
/// names `+1 Hit Point` and the drone's pools as refusals.
#[test]
fn a_hit_point_token_no_table_term_claims_is_a_named_degradation() {
    if !oracle_present() {
        return;
    }
    for id in ["core:ability:1_hit_point", "core:class:drone"] {
        let (rules, _) = converted(id);
        let t = targets(&rules);
        assert!(!t.iter().any(|x| x == "Hp" || x == "Stamina"), "{id} must not feed a pool: {t:?}");
        let (c, _) = convert_one_for(&repo_root(), GameSystem::Starfinder1e, id).unwrap();
        assert!(
            c.degradations.iter().any(|d| d.contains("no Starfinder mapping-table term")),
            "{id}: the degradation names the missing term: {:?}",
            c.degradations
        );
    }
}

/// `BONUS:COMBAT|AC|n|TYPE=EAC_Armor` feeds EAC only, `TYPE=KAC_Armor` KAC only (the game
/// mode's `ACTYPE:EAC ... REMOVE:KAC|KAC_Armor|...` rows); never the Pathfinder `Ac`.
#[test]
fn armor_class_bonuses_split_into_eac_and_kac() {
    if !oracle_present() {
        return;
    }
    let (rules, _) = converted("core:equipment:second_skin");
    let t = targets(&rules);
    assert_eq!(t.iter().filter(|x| *x == "Eac").count(), 1, "{t:?}");
    assert_eq!(t.iter().filter(|x| *x == "Kac").count(), 1, "{t:?}");
    assert!(!t.contains(&"Ac".to_string()), "no Pathfinder AC line: {t:?}");
}

fn package_dir() -> PathBuf {
    repo_root().join("data/starfinder-1e/sheet_rules")
}

fn corpus_dir() -> PathBuf {
    repo_root().join("data/starfinder-1e/corpus")
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap_or_else(|e| panic!("{}: {e}", d.display())).flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "json") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// The package on disk: its report counts every converted-book unit once, and every refusal
/// carries a named token type.
#[test]
fn the_package_report_counts_every_inventory_unit_and_names_every_refusal() {
    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(package_dir().join("_report.json")).expect("_report.json is generated")).unwrap();
    let want = inventory_units_of_converted_books().len() as u64;
    assert_eq!(report["records"].as_u64(), Some(want));
    let converted = report["converted"].as_u64().unwrap();
    let refused = report["refused"].as_u64().unwrap();
    assert_eq!(converted + refused, want);
    let refused_file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(package_dir().join("_refused.json")).expect("_refused.json")).unwrap();
    let entries = refused_file["entries"].as_array().unwrap();
    assert_eq!(entries.len() as u64, refused);
    for e in entries {
        let tts = e["token_types"].as_array().unwrap();
        assert!(!tts.is_empty() && tts.iter().all(|t| !t.as_str().unwrap().is_empty()), "unnamed refusal: {e}");
    }
}

/// Every string a Starfinder rule file carries -- labels, prose, fact values, the names a grant
/// or a bonus refers to, weapon-set members -- screened against E0.2's term set
/// (`classify_field_sf`: the Pathfinder list plus `SF_PI_TERMS`). Only identifiers are exempt:
/// a rule id (`<book>:<kind>:<slug>`, E0.3's unit id, never printed) and the provenance block.
#[test]
fn the_package_prints_no_product_identity_term() {
    fn is_rule_id(s: &str) -> bool {
        let parts: Vec<&str> = s.split(':').collect();
        parts.len() == 3 && parts.iter().all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'#'))
    }
    fn strings(v: &serde_json::Value, key: &str, out: &mut Vec<(String, String)>) {
        match v {
            serde_json::Value::String(s) if !is_rule_id(s) => out.push((key.to_string(), s.clone())),
            serde_json::Value::Array(a) => a.iter().for_each(|x| strings(x, key, out)),
            serde_json::Value::Object(o) => {
                for (k, x) in o {
                    if k != "provenance" {
                        strings(x, k, out);
                    }
                }
            }
            _ => {}
        }
    }
    let mut hits = Vec::new();
    let mut scanned = 0usize;
    for p in json_files(&package_dir()) {
        if p.file_name().is_some_and(|n| n.to_string_lossy().starts_with('_')) {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap();
        let mut out = Vec::new();
        strings(&v, "", &mut out);
        for (k, s) in out {
            scanned += 1;
            if classify_field_sf(&k, &s).0 == License::PiRedacted {
                hits.push(format!("{}: {k}={s}", p.strip_prefix(package_dir()).unwrap().display()));
            }
        }
    }
    assert!(scanned > 0, "the package prints strings");
    assert!(hits.is_empty(), "{} printed strings carry a product-identity term, first: {:?}", hits.len(), &hits[..hits.len().min(10)]);
}

/// The corpus: one record per converted-book unit, each licence-stamped, every unredacted name
/// clean under the SF screen, and a per-book LICENSE.json whose count is the file count.
#[test]
fn the_corpus_holds_one_screened_record_per_unit() {
    let want = inventory_units_of_converted_books();
    let mut by_book: BTreeMap<String, usize> = BTreeMap::new();
    let mut ids: BTreeSet<String> = BTreeSet::new();
    for p in json_files(&corpus_dir()) {
        if p.file_name().is_some_and(|n| n == "LICENSE.json") {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap();
        let book = p.strip_prefix(corpus_dir()).unwrap().components().next().unwrap().as_os_str().to_string_lossy().to_string();
        *by_book.entry(book).or_default() += 1;
        ids.insert(v["unit_id"].as_str().unwrap_or_else(|| panic!("{}: unit_id", p.display())).to_string());
        let license = v["license"].as_str().unwrap();
        assert!(license == "OGL" || license == "PI-REDACTED", "{}: licence {license}", p.display());
        let name = v["data"]["name"].as_str().unwrap();
        let pi: Vec<&str> = v["pi_field"].as_str().unwrap_or("").split(',').filter(|s| !s.is_empty()).collect();
        if !pi.contains(&"name") {
            assert_eq!(classify_field_sf("name", name).0, License::Ogl, "{}: unredacted PI name {name}", p.display());
        }
        assert_eq!(license == "PI-REDACTED", !pi.is_empty(), "{}: licence and pi_field agree", p.display());
    }
    let want_ids: BTreeSet<String> = want.iter().map(|(id, _)| id.clone()).collect();
    assert_eq!(ids, want_ids, "one corpus record per unit");
    for (book, n) in &by_book {
        let lic: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(corpus_dir().join(book).join("LICENSE.json")).expect("LICENSE.json")).unwrap();
        assert_eq!(lic["records_processed"].as_u64(), Some(*n as u64), "{book}: LICENSE.json counts the files");
    }
}

/// The stat-block rows (`prose` family `StatBlock`) a converted record prints: `(label, text)`.
fn stat_rows(rules: &serde_json::Value) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for r in rules.as_array().unwrap() {
        for p in r.get("prose").and_then(|p| p.as_array()).into_iter().flatten() {
            if let Some(label) = p["family"]["StatBlock"].as_str() {
                let text: String = p["pieces"].as_array().unwrap().iter().filter_map(|x| x["Text"].as_str()).collect();
                out.push((label.to_string(), text));
            }
        }
    }
    out
}

/// E3.5: shapes the seven other books carry that the Core Rulebook does not. In core the race
/// formula-system tokens are commented out (`scr_races.lst:25`, `#GRANT:MOVEMENT|Walk ...`); in
/// Alien Archive and Pact Worlds they are live, and COM/Near Space feats gate on `PREATT`,
/// `PREHANDS` and `PREREACH` (PCGen `PreAttackTester`: base attack bonus >= n; `PreHandsTester`,
/// `PreReachTester`: the character's hands / reach >= n).
#[test]
fn the_wide_books_prerequisites_and_race_body_rows_convert_without_degrading() {
    if !oracle_present() {
        return;
    }
    let degradations = |id: &str| -> BTreeSet<String> {
        let (c, _) = convert_one_for(&repo_root(), GameSystem::Starfinder1e, id).unwrap_or_else(|e| panic!("{id}: {e}"));
        c.degradations.clone()
    };
    // PREATT:10 is a base-attack-bonus comparison, a gate the sheet can evaluate.
    let (rules, _) = converted("character_operations_manual:feat:dispelling_strike");
    assert!(degradations("character_operations_manual:feat:dispelling_strike").is_empty());
    let text = rules.to_string();
    assert!(text.contains(r#"{"Compare":{"lhs":"BaseAttack","op":"Gte","rhs":{"Const":10}}}"#), "PREATT:10 -> BAB >= 10: {text}");
    // PREHANDS:4 / PREREACH:10 print as words (the sheet holds no hands or reach total).
    let (rules, _) = converted("character_operations_manual:feat:double_draw");
    assert!(degradations("character_operations_manual:feat:double_draw").is_empty());
    assert!(rules.to_string().contains("at least 4 hands"), "{rules}");
    let (rules, _) = converted("character_operations_manual:feat:shelter_ally");
    assert!(degradations("character_operations_manual:feat:shelter_ally").is_empty());
    assert!(rules.to_string().contains("reach of at least 10 ft."), "{rules}");
    // `MODIFY:RaceType_Humanoid|SET|True` prints the body plan; `MODIFY:Face|SET|10,10` prints
    // the space the race occupies.
    let (rules, _) = converted("alien_archive:race:haan");
    assert!(degradations("alien_archive:race:haan").is_empty(), "{:?}", degradations("alien_archive:race:haan"));
    let rows = stat_rows(&rules);
    assert!(rows.contains(&("Body plan".to_string(), "humanoid".to_string())), "{rows:?}");
    assert!(rows.contains(&("Space".to_string(), "10 ft.".to_string())), "{rows:?}");
    assert!(rows.contains(&("Reach".to_string(), "10".to_string())), "{rows:?}");
}

/// E3.5: three defects the seven other books exposed that the Core Rulebook does not carry.
///
/// 1. Two oracle rows glue a token onto the previous one with a space instead of a tab
///    (`saa_abilities.lst:102`, `...|TYPE=Base BONUS:VAR|BlindsenseRange|30|...`;
///    `scom_spells.lst:13`, `SOURCEPAGE: pg. 134 DESC:...`). Each glued token is its own token:
///    no bonus type carries source syntax, and the spell prints its description.
/// 2. A fact whose NAME is product identity (`FACT:SkyfireCenturion|True`) is withheld, as a
///    product-identity fact value already is.
/// 3. A prerequisite naming a record the PI screen renamed (`Driftborn`, renamed in the corpus)
///    resolves through the row's own name to that record, so neither the name nor a dangling
///    reference prints.
#[test]
fn glued_tokens_split_and_product_identity_names_never_print() {
    if !oracle_present() {
        return;
    }
    // The whole conversion (rules and var-table contributions), not only the rule file.
    let (c, _) = convert_one_for(&repo_root(), GameSystem::Starfinder1e, "alien_archive:ability:formian_default_formian_senses").unwrap();
    let text = format!("{:?} {:?}", c.rules, c.var_contribs);
    assert!(!text.contains("BONUS:"), "source syntax in the conversion: {text}");
    assert_eq!(c.var_contribs.len(), 2, "darkvision 60 and blindsense 30: {:?}", c.var_contribs);
    let (rules, _) = converted("character_operations_manual:spell:delay_countermeasures");
    let desc: String = rules[0]["prose"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|p| p["family"] == "Desc")
        .flat_map(|p| p["pieces"].as_array().unwrap().iter().filter_map(|x| x["Text"].as_str()).map(str::to_string).collect::<Vec<_>>())
        .collect();
    assert!(desc.contains("Countermeasures on the target computer are suppressed"), "{rules}");
    let (rules, _) = converted("pact_worlds:ability:envoy_archetype_skyfire_centurion");
    assert!(!rules.to_string().contains("SkyfireCenturion"), "{rules}");
    let (rules, _) = converted("character_operations_manual:feat:multifaceted_nature");
    let text = rules.to_string();
    assert!(text.contains(r#""Rule":"character_operations_manual:ability:gnome_driftborn""#), "{text}");
    assert!(!text.contains("Driftborn") && !text.contains("MissingRule"), "{text}");
}
