//! SD-27 — the price a newly reachable equipment row is charged must be the
//! price its own corpus record carries.
//!
//! **What this closes.** The desktop "Attach Modifier" picker is served by
//! `build_equipment_catalog()` (all six ingested books) while
//! `attach_equipment_modifier_at_root` validated against
//! `crb::equipment_tables()` alone. 105 of the 763 offered `Equipmods` rows
//! (ACG 48, ARG 15, PU 42) were therefore offered and then refused —
//! *"'Material ~ Whipwood' is not a recognized equipment catalog item"* —
//! the dead affordance `docs/governance/no-stub-mvp-doctrine.md` forbids.
//!
//! Widening *recognition* alone would have been worse than the refusal: 20
//! of those 105 rows carry a real, non-zero flat price (ACG's 4,500 gp
//! `Amorphous`, ARG's 500 gp `Whipwood`, …), and a CRB-only cost path would
//! have attached every one of them for **free**. Recognition and price were
//! widened together, and this suite is the independent check that the price
//! half is actually right.
//!
//! **Why it reads the corpus rather than the tables.** The unit tests beside
//! `equipment_resolver.rs` assert the resolver agrees with the compiled
//! per-book tables. That would still pass if a table itself had drifted from
//! the book it was generated from. This suite goes to `data/corpus/<book>/`
//! and compares against each record's own raw `COST:` **token** — the actual
//! PCGen LST value, the same field `encumbrance.rs` was already fixed to read
//! off a resolved record rather than from CRB-only tables.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::equipment_resolver::equipment_catalog_row_by_key;

/// The three books whose equipment rows the attach/purchase paths could not
/// recognize before this change. CRB is deliberately excluded: its 316
/// duplicate keys mean a corpus record cannot be matched 1:1 to the single
/// table row a key lookup returns, so a per-record comparison there would be
/// asserting against an ambiguity rather than against the corpus.
const NEWLY_REACHABLE_BOOKS: &[(&str, &str)] = &[
    ("ACG", "advanced_class_guide"),
    ("ARG", "advanced_race_guide"),
    ("PU", "pathfinder_unchained"),
];

struct CorpusRecord {
    key: String,
    /// The record's own raw `COST:` token value, verbatim. `None` when the
    /// LST line carries no `COST:` token at all.
    cost_token: Option<String>,
    path: PathBuf,
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = fs::read_dir(&current) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if path.is_dir() {
                if name != "_parity" {
                    stack.push(path);
                }
            } else if name != "LICENSE.json"
                && path.extension().and_then(|e| e.to_str()) == Some("json")
            {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn load_records(book_dir: &str) -> Vec<CorpusRecord> {
    let dir = Path::new("data/corpus").join(book_dir).join("equipment");
    assert!(dir.is_dir(), "real corpus directory must exist: {}", dir.display());

    let mut records = Vec::new();
    for path in json_files(&dir) {
        let text = fs::read_to_string(&path).expect("corpus file must be readable");
        let value: serde_json::Value =
            serde_json::from_str(&text).expect("corpus file must be valid JSON");
        let Some(data) = value.get("data") else { continue };
        let Some(key) = data.get("key").and_then(serde_json::Value::as_str) else { continue };

        let cost_token = data
            .get("raw_tokens")
            .and_then(serde_json::Value::as_array)
            .and_then(|tokens| {
                tokens
                    .iter()
                    .find(|token| {
                        token.get("key").and_then(serde_json::Value::as_str) == Some("COST")
                    })
                    .and_then(|token| token.get("value").and_then(serde_json::Value::as_str))
                    .map(str::to_string)
            });

        records.push(CorpusRecord { key: key.to_string(), cost_token, path });
    }
    assert!(!records.is_empty(), "{book_dir} must contribute real corpus records");
    records
}

/// **The core proof.** For every equipment record in every newly reachable
/// book, the price the engine will charge is exactly the record's own
/// `COST:` token — and where the token is a PCGen formula rather than a
/// number, the engine reports *no known price* rather than inventing one.
#[test]
fn every_newly_reachable_record_is_priced_by_its_own_corpus_cost_token() {
    let mut checked_numeric = 0usize;
    let mut checked_formula = 0usize;
    let mut checked_absent = 0usize;

    for (book_code, book_dir) in NEWLY_REACHABLE_BOOKS {
        for record in load_records(book_dir) {
            let Some(row) = equipment_catalog_row_by_key(&record.key) else {
                panic!(
                    "{book_code} corpus record {:?} ({}) is not recognized by the engine at all \
                     — that is the dead affordance this change exists to remove",
                    record.key,
                    record.path.display()
                );
            };
            // Guard against a same-key row from another book answering: the
            // comparison below is only meaningful against this book's row.
            assert_eq!(
                row.book, *book_code,
                "{:?} resolved to book {} instead of {book_code}",
                record.key, row.book
            );

            match record.cost_token.as_deref() {
                Some(token) => match token.parse::<f64>() {
                    Ok(corpus_cost) => {
                        assert_eq!(
                            row.cost_gp,
                            Some(corpus_cost),
                            "{book_code} {:?} charges {:?} but its corpus record carries COST:{} \
                             ({})",
                            record.key,
                            row.cost_gp,
                            token,
                            record.path.display()
                        );
                        checked_numeric += 1;
                    }
                    Err(_) => {
                        assert_eq!(
                            row.cost_gp,
                            None,
                            "{book_code} {:?} carries the PCGen formula COST:{} which this engine \
                             does not evaluate, so it must report no known price rather than a \
                             fabricated flat number ({})",
                            record.key,
                            token,
                            record.path.display()
                        );
                        checked_formula += 1;
                    }
                },
                None => {
                    assert_eq!(
                        row.cost_gp,
                        None,
                        "{book_code} {:?} has no COST: token in the corpus at all, so any price \
                         at all would be fabricated ({})",
                        record.key,
                        record.path.display()
                    );
                    checked_absent += 1;
                }
            }
        }
    }

    // Derived by running this suite, never assumed. Pinned so the coverage
    // itself cannot silently shrink to zero and leave the assertions above
    // passing vacuously.
    //
    // The single formula record is ARG's `Material ~ Darkleaf Cloth ~ Item`
    // (`COST:WT*375`, a weight-dependent price this engine does not
    // evaluate).
    //
    // RAISED `SD31-W4-INTEGRATE-001`, 2026-08-16 (found already red at the
    // merged wave-4 tip, before ANY of this cycle's own edits touched
    // ACG/ARG/PU): `SD31-E6-F5-002`'s `equipment_gap` cache-gen added real,
    // oracle-cited ACG/ARG equipmod records this test's book-directory walk
    // now also covers -- every added record's own per-record assertion in
    // the loop above already passed (a numeric COST: token prices exactly,
    // an absent COST: token prices as None), so this is real coverage
    // growth, not a defect: checked_numeric 425 -> 433 (+8, the same ACG
    // Amorphous/Burdenless/Exclusionary/Prehensile/Restful/Sneaky/Spiteful/
    // Trackless rows already reconciled in equipment_resolver.rs and
    // character_hub.rs), checked_absent 85 -> 140 (+55, ACG/ARG records
    // with no COST: token, priced correctly as None).
    // RAISED again `SD31-E6-F6-001`, 2026-08-16: `gen_equipment_gap_tables.
    // rs` gained `.COPY=` inheritance -- 14 of the 140 previously-`checked_
    // absent` ACG/ARG/PU records (a `.COPY=` row with no `COST:` of its own)
    // now inherit a real, corpus-true `cost_gp` from their base record
    // (resolved by the identical `KEY:`-or-bare-name identity a `.COPY=`
    // reference itself resolves against, verified one record deep, never
    // fabricated). checked_numeric 433 -> 447 (+14), checked_absent
    // 140 -> 126 (-14), total unchanged (574 = 447+1+126) -- the SAME 574
    // records, 14 reclassified from "no known price" to "a real price",
    // never a population change.
    assert_eq!(
        (checked_numeric, checked_formula, checked_absent),
        (447, 1, 126),
        "records priced by a numeric COST: token, by an unevaluated formula, and with no COST: \
         token at all"
    );
    // RAISED `SD31-W4-INTEGRATE-001`, 2026-08-16, same reconciliation as the
    // tuple assertion above: 511 -> 574 (433 + 1 + 140), covering
    // `equipment_gap_tables`'s real ACG/ARG additions on top of the
    // original ACG 269 + ARG 200 + PU 42 population. Every record in all
    // three books is still covered, so none can be skipped by a
    // silently-failing file walk.
    assert_eq!(checked_numeric + checked_formula + checked_absent, 574);
}

/// The two rows named in the on-screen defect report, stated explicitly so
/// the regression is legible without reading the loop above.
#[test]
fn the_two_rows_the_ui_refused_on_screen_now_resolve_at_their_corpus_prices() {
    // ARG: refused as unrecognized; `arg_equipmods.lst` line 38 carries
    // COST:500. Attaching this for free was the specific silent-mispricing
    // outcome a recognition-only fix would have produced.
    let whipwood = equipment_catalog_row_by_key("Material ~ Whipwood")
        .expect("'Material ~ Whipwood' must be recognized");
    assert_eq!(whipwood.book, "ARG");
    assert_eq!(whipwood.cost_gp, Some(500.0));

    // PU: refused as unrecognized; `pu_equipmods.lst` carries no COST: token
    // on any of its 42 rows, so "no known price" is the corpus truth here,
    // not a lookup failure.
    let attunement = equipment_catalog_row_by_key("ABP ~ +3 Attunement ~ Armor")
        .expect("'ABP ~ +3 Attunement ~ Armor' must be recognized");
    assert_eq!(attunement.book, "PU");
    assert_eq!(attunement.cost_gp, None);

    // Control: the CRB row that was already failing correctly, on price
    // rather than on recognition, is untouched — 1,000 gp = the 100,000 cp
    // the UI reported.
    let mithril = equipment_catalog_row_by_key("Material ~ Mithril ~ Armor / Light")
        .expect("the CRB control must still resolve");
    assert_eq!(mithril.book, "CRB");
    assert_eq!(mithril.cost_gp, Some(1000.0));
}

/// Every newly reachable book's corpus keys must be unique within that book,
/// which is what makes the per-record comparison above a 1:1 check rather
/// than a first-match coincidence. (CRB, excluded above, is where the
/// duplicates live.)
#[test]
fn the_newly_reachable_books_have_no_duplicate_corpus_keys() {
    for (book_code, book_dir) in NEWLY_REACHABLE_BOOKS {
        let mut seen: BTreeMap<String, usize> = BTreeMap::new();
        for record in load_records(book_dir) {
            *seen.entry(record.key).or_default() += 1;
        }
        let dupes: Vec<&String> =
            seen.iter().filter(|(_, count)| **count > 1).map(|(key, _)| key).collect();
        assert!(dupes.is_empty(), "{book_code} has duplicate corpus keys: {dupes:?}");
    }
}
