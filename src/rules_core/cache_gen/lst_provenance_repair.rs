//! Repairs a shipped equipment record's `source` citation when the record
//! carries a `web_second_source` kind for values that actually came from the
//! pinned PCGen corpus (SD-31 `epic-6-ingest-lanes` F5).
//!
//! ## The defect this exists for
//!
//! `rules_tables::apg::equipment_data`'s own module doc comment states the
//! sourcing methodology plainly: every APG equipment record's `key`,
//! `cost_gp` and `weight` were **generated from the real PCGen corpus**
//! (`COST:`, `WT:`, `OUTPUTNAME:`, `KEY:` tokens in `apg_equip_general.lst` /
//! `apg_equip_arms_armor.lst` / `apg_equip_magic_items.lst`). Only the
//! `description` came from a web second source, because APG's equipment
//! `.lst` files carry **zero** `DESC:` tokens — equipment flavour text lives
//! in the printed book's prose, not the LST data.
//!
//! The shipped `data/corpus/advanced_players_guide/equipment/*.json` records
//! nevertheless stamp `source: { kind: "web_second_source", url:
//! "https://legacy.aonprd.com/..." }` for the **whole record**. That
//! misattributes corpus-derived identity and magnitudes to a web page, and it
//! has a measurable consequence: `corpus_literal_sweep`'s population is
//! `source.kind == "lst_token"` records only, so none of these records is
//! ever byte-compared against the corpus at all, and their units can never
//! leave the `static` + `ingested-magnitude` rung (`held`) for
//! `literal-verified` (`done`).
//!
//! ## What this module does, and the bar it holds itself to
//!
//! It only ever **narrows** a citation from "some web page" to "this exact
//! corpus row", and only when the corpus itself proves the record's own
//! shipped magnitudes. For each `web_second_source` record it:
//!
//! 1. resolves the record's identity (`data.key`, else `data.name`) to a real
//!    corpus row with [`equipment_gap::find_citation`] — the same resolver
//!    `cache_gen::equipment_gap` and `cache_gen::hand_authored_equipment`
//!    already use, not a fork;
//! 2. builds that row's **token closure** with
//!    `corpus_literal_sweep::token_closure` over `wiring_class::
//!    build_mod_index` — again the sweep's own functions, so the predicate
//!    here is the gate's predicate rather than a weaker sibling of it;
//! 3. requires every typed magnitude the record actually claims (`cost_gp`,
//!    and `weight_lbs` **or** its APG/ACG/Bestiary-era spelling `weight`) to
//!    be numerically present in that closure under `COST:` / `WT:`;
//! 4. only then rewrites `source` to an `lst_token` citation
//!    (path/sha256/line/record_key) and moves the original web citation, in
//!    full, to an additive `description_source` key.
//!
//! A record that fails **any** of those steps is left exactly as it was and
//! reported by name. Nothing is fabricated and no field is invented: the
//! whole change is a citation getting more specific.
//!
//! **`weight` is checked here even though `corpus_literal_sweep` reads only
//! `weight_lbs`.** APG/ACG/Bestiary-era records spell that field `weight`
//! (`enrich_equipment_raw_tokens`'s own module doc comment records the
//! divergence and the data loss it once caused), which means the sweep's
//! typed-field check silently skips it for exactly the records this module
//! moves into the sweep's population. Upgrading a record whose weight the
//! gate will never look at would be manufacturing an unfalsifiable pass, so
//! this module checks that field itself, at the point it creates the claim.
//!
//! ## Why raw `serde_json::Value`, never a typed round-trip
//!
//! Same reason `enrich_equipment_raw_tokens` documents at length: these
//! records carry per-book fields (`weight`, `equip_type`, `plus`) that
//! `shape_b_v1::EquipmentCacheData` does not know about, and deserializing
//! into the typed struct silently **drops** them on re-serialize (a real,
//! caught-in-review data loss on that tool's first version). Operating on
//! `Value` and touching only `source` / `description_source` means every
//! other field survives by construction.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::rules_core::cache_gen::equipment_gap::{
    disabled_identity_column, find_citation, sha256_file,
};
use crate::rules_core::corpus_literal_sweep::token_closure;
use crate::rules_core::wiring_class::build_mod_index;

/// Why one record was left alone. Every variant is reported by name rather
/// than counted, so a run that repaired nothing says which records refused
/// and why — the "0 reclaimed means structurally full, not clean" lesson
/// applied to this tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// `data.key`/`data.name` matched no row in the book's corpus directory.
    UnresolvedCitation(String),
    /// The resolved row's identity column opens with `#` — a row PCGen's own
    /// maintainers disabled. Never cited as live content.
    DisabledRow(String),
    /// A typed magnitude the record claims is not numerically present in the
    /// resolved row's token closure. THIS is the check that makes the
    /// upgrade meaningful; a hit here is a real disagreement between the
    /// shipped record and the pinned corpus and must be investigated, never
    /// waved through.
    MagnitudeDisagreement { record: String, detail: String },
    /// The cited row is itself a `.COPY=` declaration. Resolving the base
    /// row it copies is `corpus_literal_sweep`'s own job and this module
    /// does not fork that rule, so such a record is refused rather than
    /// upgraded on a closure that may be missing inherited tokens.
    CopyRowNotResolved(String),
    /// The record carries neither `data.key` nor `data.name` — no identity
    /// to resolve.
    NoIdentity(String),
}

impl Refusal {
    pub fn describe(&self) -> String {
        match self {
            Refusal::UnresolvedCitation(r) => format!("{r}: no corpus row matches this record's identity"),
            Refusal::DisabledRow(r) => format!("{r}: resolved row's identity column is #-disabled"),
            Refusal::MagnitudeDisagreement { record, detail } => {
                format!("{record}: shipped magnitude disagrees with the corpus row -- {detail}")
            }
            Refusal::CopyRowNotResolved(r) => format!("{r}: cited row is a .COPY= declaration; base row not folded in"),
            Refusal::NoIdentity(r) => format!("{r}: record carries neither data.key nor data.name"),
        }
    }
}

#[derive(Debug, Default)]
pub struct RepairReport {
    /// Record paths whose `source` was narrowed to a real `lst_token`
    /// citation.
    pub upgraded: Vec<String>,
    /// Records already carrying an `lst_*` citation — untouched, not an
    /// error.
    pub already_cited: usize,
    /// Records read in total.
    pub records_seen: usize,
    /// Records left alone, each with the reason.
    pub refused: Vec<Refusal>,
}

/// Every numeric value the closure states under `<lst_key>:`. Mirrors
/// `corpus_literal_sweep`'s own (private) `closure_numeric_values` — a
/// `.MOD` override can legitimately state the key more than once, so all
/// candidates are returned and any one matching is a match.
fn closure_numeric_values(closure: &BTreeSet<String>, lst_key: &str) -> Vec<f64> {
    let prefix = format!("{lst_key}:");
    closure
        .iter()
        .filter_map(|t| t.strip_prefix(prefix.as_str()))
        .filter_map(|v| v.parse::<f64>().ok())
        .collect()
}

/// `true` when `shipped` is absent (nothing claimed, nothing to prove) or
/// numerically equal to some `<lst_key>:` value in the closure.
fn magnitude_is_corpus_backed(shipped: Option<f64>, closure: &BTreeSet<String>, lst_key: &str) -> bool {
    let Some(shipped) = shipped else { return true };
    closure_numeric_values(closure, lst_key)
        .into_iter()
        .any(|candidate| (candidate - shipped).abs() < 1e-9)
}

/// The identity a `.COPY=<name>` row's first column names as its base.
/// `None` for a plain row. Same literal PCGen syntax split
/// `enrich_equipment_raw_tokens` and `gen_equipment_gap_tables` use.
fn is_copy_row(line: &str) -> bool {
    line.split('\t').next().unwrap_or("").contains(".COPY=")
}

/// Recursively collects every `*.json` under `dir` except `LICENSE.json`
/// (a per-book licence manifest, not a content record).
fn record_paths(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json")
                && path.file_name().and_then(|f| f.to_str()) != Some("LICENSE.json")
            {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// Decides one already-parsed record. Returns the new `source` value plus
/// the web citation to move to `description_source`, or the refusal.
///
/// Split out from [`repair_book`] so the decision — which is the whole of
/// this module's judgement — is unit-testable without a corpus tree on disk.
#[allow(clippy::too_many_arguments)]
fn decide(
    record_path: &str,
    data: &Value,
    web_source: &Value,
    rel_lst_path: &str,
    line: u32,
    sha256: &str,
    row: &str,
    identities: &BTreeSet<String>,
    mod_index: &BTreeMap<String, Vec<String>>,
) -> Result<(Value, Value), Refusal> {
    if is_copy_row(row) {
        return Err(Refusal::CopyRowNotResolved(record_path.to_string()));
    }
    let closure = token_closure(row, identities, mod_index, None);

    let cost = data.get("cost_gp").and_then(Value::as_f64);
    if !magnitude_is_corpus_backed(cost, &closure, "COST") {
        return Err(Refusal::MagnitudeDisagreement {
            record: record_path.to_string(),
            detail: format!(
                "cost_gp={:?} is not stated by any COST: token at {rel_lst_path}:{line}",
                cost.expect("magnitude_is_corpus_backed only refuses a Some value")
            ),
        });
    }
    // `weight_lbs` is the schema spelling; `weight` is the APG/ACG/Bestiary-era
    // spelling the same value ships under. Whichever the record uses is the
    // one that has to be corpus-backed.
    let weight = data
        .get("weight_lbs")
        .and_then(Value::as_f64)
        .or_else(|| data.get("weight").and_then(Value::as_f64));
    if !magnitude_is_corpus_backed(weight, &closure, "WT") {
        return Err(Refusal::MagnitudeDisagreement {
            record: record_path.to_string(),
            detail: format!(
                "weight={:?} is not stated by any WT: token at {rel_lst_path}:{line}",
                weight.expect("magnitude_is_corpus_backed only refuses a Some value")
            ),
        });
    }

    let record_key = data
        .get("key")
        .and_then(Value::as_str)
        .or_else(|| data.get("name").and_then(Value::as_str))
        .unwrap_or_default()
        .to_string();

    Ok((
        json!({
            "kind": "lst_token",
            "path": rel_lst_path,
            "sha256": sha256,
            "line": line,
            "record_key": record_key,
        }),
        web_source.clone(),
    ))
}

/// Repairs every `web_second_source` equipment record under `records_dir`.
///
/// * `corpus_root` — a pinned PCGen `data/` checkout
///   (`scripts/pcgen-oracle-pin.env`; resolved by the caller, never a
///   literal path in this module).
/// * `book_rel_dir` — the book's directory relative to `corpus_root`, e.g.
///   `pathfinder/paizo/roleplaying_game/advanced_players_guide`. The written
///   `source.path` is `<book_rel_dir>/<file>`, which is exactly the shape
///   `corpus_literal_sweep` resolves and whose immediate parent directory
///   name it attributes the book from.
/// * `records_dir` — `data/corpus/<book>/equipment`.
/// * `write` — `false` for a check run: every record is read and decided
///   exactly as it would be for a real run (the SAME predicate, not a second
///   implementation of it); only the final `std::fs::write` is skipped.
pub fn repair_book(
    corpus_root: &Path,
    book_rel_dir: &str,
    records_dir: &Path,
    write: bool,
) -> std::io::Result<RepairReport> {
    let mut report = RepairReport::default();
    let book_dir = corpus_root.join(book_rel_dir);
    let book_paths: BTreeMap<String, PathBuf> =
        [(book_rel_dir.to_string(), book_dir.clone())].into_iter().collect();
    // `build_mod_index` keys on `(book, identity)`; this module works one
    // book at a time, so narrow it to the identity half the sweep's own
    // `token_closure` expects.
    let mod_index: BTreeMap<String, Vec<String>> = build_mod_index(&book_paths)
        .into_iter()
        .map(|((_, identity), rows)| (identity, rows))
        .collect();

    let mut line_cache: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
    let mut sha_cache: BTreeMap<PathBuf, String> = BTreeMap::new();

    for path in record_paths(records_dir) {
        report.records_seen += 1;
        let record_path = path.display().to_string();
        let text = std::fs::read_to_string(&path)?;
        let mut root: Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if root.get("source").and_then(|s| s.get("kind")).and_then(Value::as_str)
            != Some("web_second_source")
        {
            report.already_cited += 1;
            continue;
        }
        let data = root.get("data").cloned().unwrap_or(Value::Null);
        let key = data.get("key").and_then(Value::as_str).unwrap_or_default().to_string();
        let name = data.get("name").and_then(Value::as_str).unwrap_or_default().to_string();
        if key.is_empty() && name.is_empty() {
            report.refused.push(Refusal::NoIdentity(record_path));
            continue;
        }
        let identity_key = if key.is_empty() { name.clone() } else { key.clone() };
        let identity_name = if name.is_empty() { key.clone() } else { name.clone() };

        // `find_citation` returns the path RELATIVE to `book_dir`, and it
        // may be nested one or more directories down inside the book (its own
        // recursive fallback). Both facts matter below: the absolute path is
        // needed to read/digest the file, and the relative path is what the
        // written citation states.
        let Some((rel_lst, line)) = find_citation(&book_dir, &identity_key, &identity_name) else {
            report.refused.push(Refusal::UnresolvedCitation(record_path));
            continue;
        };
        let abs_lst = book_dir.join(&rel_lst);
        if disabled_identity_column(&abs_lst, line) {
            report.refused.push(Refusal::DisabledRow(record_path));
            continue;
        }
        let lines = match line_cache.get(&abs_lst) {
            Some(l) => l,
            None => {
                let l: Vec<String> =
                    std::fs::read_to_string(&abs_lst)?.lines().map(str::to_string).collect();
                line_cache.entry(abs_lst.clone()).or_insert(l)
            }
        };
        let Some(row) = lines.get(line.saturating_sub(1) as usize).cloned() else {
            report.refused.push(Refusal::UnresolvedCitation(record_path));
            continue;
        };
        let sha = match sha_cache.get(&abs_lst) {
            Some(s) => s.clone(),
            None => {
                let s = sha256_file(&abs_lst)?;
                sha_cache.insert(abs_lst.clone(), s.clone());
                s
            }
        };
        let rel_lst_path = format!("{book_rel_dir}/{}", rel_lst.display());

        let identities: BTreeSet<String> =
            [key.clone(), name.clone()].into_iter().filter(|s| !s.is_empty()).collect();
        let web_source = root.get("source").cloned().unwrap_or(Value::Null);

        match decide(
            &record_path,
            &data,
            &web_source,
            &rel_lst_path,
            line,
            &sha,
            &row,
            &identities,
            &mod_index,
        ) {
            Err(refusal) => report.refused.push(refusal),
            Ok((new_source, description_source)) => {
                root["source"] = new_source;
                if let Value::Object(map) = &mut root {
                    map.insert("description_source".to_string(), description_source);
                }
                if write {
                    let mut out = serde_json::to_string_pretty(&root)
                        .expect("a Value re-serializes; it was just parsed from JSON");
                    out.push('\n');
                    std::fs::write(&path, out)?;
                }
                report.upgraded.push(record_path);
            }
        }
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::corpus_literal_sweep::tab_tokens;

    fn row_index(rows: &[&str]) -> BTreeMap<String, Vec<String>> {
        let mut index: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for row in rows {
            let head = row.split('\t').next().unwrap_or("");
            if let Some(at) = head.find(".MOD") {
                index
                    .entry(crate::rules_core::wiring_class::mod_base_name(&head[..at]))
                    .or_default()
                    .push((*row).to_string());
            }
        }
        index
    }

    fn identities(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|s| (*s).to_string()).collect()
    }

    const ABACUS_ROW: &str = "Abacus\tTYPE:Goods.Tools\tCOST:2\tWT:2\tSOURCEPAGE:p.182";

    fn web(url: &str) -> Value {
        json!({
            "kind": "web_second_source",
            "url": url,
            "fetched_at": "2026-07-22T08:37:22-04:00",
            "identity_match_basis": "name+cost",
        })
    }

    /// The core case: a record whose shipped cost and weight are both stated
    /// by the resolved corpus row gets a real `lst_token` citation, and the
    /// web citation it replaces is preserved verbatim under
    /// `description_source` rather than discarded.
    #[test]
    fn a_corpus_backed_record_is_narrowed_to_an_lst_token_citation() {
        let data = json!({"key": "Abacus", "name": "Abacus", "cost_gp": 2.0, "weight": 2.0});
        let (source, desc_source) = decide(
            "r.json",
            &data,
            &web("https://legacy.aonprd.com/x"),
            "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_equip_general.lst",
            73,
            "abc123",
            ABACUS_ROW,
            &identities(&["Abacus"]),
            &BTreeMap::new(),
        )
        .expect("a corpus-backed record must be upgraded");
        assert_eq!(source["kind"], "lst_token");
        assert_eq!(source["line"], 73);
        assert_eq!(source["sha256"], "abc123");
        assert_eq!(source["record_key"], "Abacus");
        assert_eq!(
            source["path"],
            "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_equip_general.lst"
        );
        // The web citation is moved, never dropped.
        assert_eq!(desc_source["kind"], "web_second_source");
        assert_eq!(desc_source["url"], "https://legacy.aonprd.com/x");
    }

    /// The check that makes the upgrade mean something: a shipped cost the
    /// corpus row does not state is a refusal, not a silently-narrowed
    /// citation. Without this the whole tool would be a gate that cannot
    /// fail.
    #[test]
    fn a_cost_the_corpus_row_does_not_state_refuses() {
        let data = json!({"key": "Abacus", "name": "Abacus", "cost_gp": 99.0, "weight": 2.0});
        let err = decide(
            "r.json",
            &data,
            &web("https://x"),
            "a/apg_equip_general.lst",
            73,
            "abc",
            ABACUS_ROW,
            &identities(&["Abacus"]),
            &BTreeMap::new(),
        )
        .expect_err("a cost the row does not state must refuse");
        assert!(matches!(err, Refusal::MagnitudeDisagreement { .. }), "{err:?}");
        assert!(err.describe().contains("COST:"), "{}", err.describe());
    }

    /// `weight` (the APG/ACG/Bestiary spelling) is checked even though
    /// `corpus_literal_sweep` reads only `weight_lbs` — otherwise this tool
    /// would move records into the sweep's population whose weight the gate
    /// then never looks at.
    #[test]
    fn a_weight_the_corpus_row_does_not_state_refuses_under_either_field_name() {
        for field in ["weight", "weight_lbs"] {
            let mut data = json!({"key": "Abacus", "name": "Abacus", "cost_gp": 2.0});
            data[field] = json!(37.0);
            let err = decide(
                "r.json",
                &data,
                &web("https://x"),
                "a/apg_equip_general.lst",
                73,
                "abc",
                ABACUS_ROW,
                &identities(&["Abacus"]),
                &BTreeMap::new(),
            )
            .expect_err("a weight the row does not state must refuse");
            assert!(matches!(err, Refusal::MagnitudeDisagreement { .. }), "{field}: {err:?}");
            assert!(err.describe().contains("WT:"), "{field}: {}", err.describe());
        }
    }

    /// A record that claims no magnitude at all has nothing to disprove; the
    /// citation still narrows. (Real population: APG's `Bomb` and
    /// `Formula Book` template rows carry no `COST:`/`WT:` token.)
    #[test]
    fn a_record_claiming_no_magnitude_is_upgraded_on_identity_alone() {
        let data = json!({"key": "Bomb", "name": "Bomb", "cost_gp": null, "weight": null});
        decide(
            "r.json",
            &data,
            &web("https://x"),
            "a/apg_equip_arms_armor.lst",
            5,
            "abc",
            "Bomb\tTYPE:Weapon.Exotic",
            &identities(&["Bomb"]),
            &BTreeMap::new(),
        )
        .expect("a record with no claimed magnitude has nothing to disprove");
    }

    /// A `.MOD` row's override counts: the closure is the sweep's closure,
    /// not just the base row's tokens.
    #[test]
    fn a_mod_row_override_satisfies_the_magnitude_check() {
        let rows = ["Abacus\tTYPE:Goods.Tools\tCOST:2\tWT:2", "Abacus.MOD\tCOST:5"];
        let data = json!({"key": "Abacus", "name": "Abacus", "cost_gp": 5.0, "weight": 2.0});
        decide(
            "r.json",
            &data,
            &web("https://x"),
            "a/apg_equip_general.lst",
            1,
            "abc",
            rows[0],
            &identities(&["Abacus"]),
            &row_index(&rows),
        )
        .expect("a .MOD override is part of the closure the sweep will compare against");
    }

    /// A `.COPY=` row's inherited tokens are `corpus_literal_sweep`'s own
    /// resolution rule; this module refuses rather than forking it.
    #[test]
    fn a_copy_row_is_refused_rather_than_upgraded_on_a_partial_closure() {
        let data = json!({"key": "Bastard's Sting", "name": "Bastard's Sting", "cost_gp": 1.0});
        let err = decide(
            "r.json",
            &data,
            &web("https://x"),
            "a/apg_equip_arms_armor.lst",
            447,
            "abc",
            "Bastard's Sting.COPY=Dagger\tCOST:1",
            &identities(&["Bastard's Sting"]),
            &BTreeMap::new(),
        )
        .expect_err(".COPY= rows are refused");
        assert!(matches!(err, Refusal::CopyRowNotResolved(_)), "{err:?}");
    }

    /// The token this module writes must be the one `tab_tokens` reads back
    /// out of the row — the shared parse, not a second one.
    #[test]
    fn the_closure_this_module_builds_is_the_sweeps_own_tab_token_split() {
        let closure = token_closure(ABACUS_ROW, &identities(&["Abacus"]), &BTreeMap::new(), None);
        assert!(closure.contains("COST:2"));
        assert!(closure.contains("WT:2"));
        assert_eq!(closure.len(), tab_tokens(ABACUS_ROW).len());
    }
}
