//! `companion_chassis::GRANT_TOKEN_ONLY_DISPATCH_ROWS`'s own claim, checked
//! against the live corpus and `docs/work-inventory.json` (SD-36 Epic A E1
//! eviction: this test used only `codex::rules_core`'s public API plus
//! `codex_ingest::pcgen_import::ingest_record`, so it moved verbatim out of
//! `src/rules_core/rules_tables/companion_chassis.rs`'s `#[cfg(test)] mod
//! tests` rather than being fixture-ized).

use serde_json::Value;
use std::collections::BTreeMap;

use codex_ingest::pcgen_import::ingest_record;

/// This cycle's own build: proves `codex::rules_core::rules_tables::companion_chassis::GRANT_TOKEN_ONLY_DISPATCH_ROWS`'
/// own claim, per record, against the live corpus AND the live
/// `docs/work-inventory.json` -- never merely asserted in a doc
/// comment. For each of the 12 named keys: (1) the corpus shape is
/// genuinely zero-content (no `TYPE`/`DESC`/`BONUS` token, `ABILITY:`
/// present), and (2) EVERY `ABILITY:` token's target key is a real
/// `core_rulebook` companion row whose live work-inventory status is
/// already `grounded`, `text-complete`, or `literal-verified` --
/// i.e. this row's only job is to fan out to content the engine
/// ALREADY holds, not to a dead pointer or an unheld row. RED if the
/// corpus ever adds real content to one of these 12 keys, or if any
/// target's engine status ever regresses out of the held set (exactly
/// when `decisions.md §2`'s "cleared by revisiting the stated
/// condition" fires).
#[test]
fn grant_token_only_rows_dispatch_to_already_held_content() {
    let repo_root = codex_ingest::repo_root();
    let inventory_text = std::fs::read_to_string(repo_root.join("docs/work-inventory.json"))
        .expect("docs/work-inventory.json is readable");
    let inventory: Value = serde_json::from_str(&inventory_text)
        .expect("docs/work-inventory.json is valid JSON");
    let units = inventory["units"].as_array().expect("units is an array");
    let mut status_by_key: BTreeMap<&str, &str> = BTreeMap::new();
    for u in units {
        if u["book"].as_str() == Some("core_rulebook") && u["kind"].as_str() == Some("companion")
            && let (Some(k), Some(s)) = (u["corpus_key"].as_str(), u["status"].as_str()) {
                status_by_key.insert(k, s);
            }
    }
    // `oracle-agree`/`oracle-unverifiable` (`decisions.md §19`) are
    // REFINEMENTS of `literal-verified`/`fixture-verified`, never a new
    // tier -- the unit already met `literal-verified`'s bar before the
    // oracle looked at it (or, for `oracle-unverifiable`, before the
    // oracle found it had no surface to check). Both are held content,
    // same as `58b4f837cc` taught the doneness table.
    // `sheet-complete` (SD-35 AT-35-E2-003, `decisions.md §1`): the
    // record's `SheetRule` renders for a probe character -- held content
    // under the sheet rule, the terminal state above `engine-does-not-hold`.
    const HELD_STATUSES: [&str; 6] = [
        "grounded",
        "text-complete",
        "literal-verified",
        "oracle-agree",
        "oracle-unverifiable",
        "sheet-complete",
    ];

    let companion_dir = repo_root.join("data/corpus/core_rulebook/companion");
    let mut companion_docs: Vec<Value> = Vec::new();
    for entry in std::fs::read_dir(&companion_dir)
        .unwrap_or_else(|e| panic!("{}: {e}", companion_dir.display()))
    {
        let path = entry.expect("readable dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("readable json file");
        let doc: Value = serde_json::from_str(&text).expect("valid json");
        companion_docs.push(doc);
    }
    let find_by_key = |key: &str| -> &Value {
        companion_docs
            .iter()
            .find(|d| d["data"]["key"].as_str() == Some(key))
            .unwrap_or_else(|| panic!("{key}: no corpus record found under {}", companion_dir.display()))
    };

    assert_eq!(codex::rules_core::rules_tables::companion_chassis::GRANT_TOKEN_ONLY_DISPATCH_ROWS.len(), 12);
    for (key, _reason) in codex::rules_core::rules_tables::companion_chassis::GRANT_TOKEN_ONLY_DISPATCH_ROWS {
        let doc = find_by_key(key);
        let has_modelled_token = ingest_record::token_keys(doc)
            .into_iter()
            .any(|k| matches!(k, "TYPE" | "DESC" | "BONUS"));
        assert!(
            !has_modelled_token,
            "{key}: expected zero-content (ABILITY grant only), but a modelled token is \
             present -- this row may now carry real content and no longer belong here"
        );
        let ability_targets: Vec<&str> = ingest_record::token_values(doc, "ABILITY")
            .into_iter()
            .map(|value| {
                // `Companion Class Feature|AUTOMATIC|<target key>|<optional PRE conditions>`
                value.split('|').nth(2).unwrap_or_else(|| {
                    panic!("{key}: ABILITY token has no target key segment: {value}")
                })
            })
            .collect();
        assert!(!ability_targets.is_empty(), "{key}: expected at least one ABILITY: token");
        for target in ability_targets {
            let status = status_by_key.get(target).unwrap_or_else(|| {
                panic!(
                    "{key}: ABILITY: target {target:?} is not a core_rulebook companion unit \
                     in docs/work-inventory.json at all"
                )
            });
            assert!(
                HELD_STATUSES.contains(status),
                "{key}: ABILITY: target {target:?} has status {status:?}, not one of \
                 {HELD_STATUSES:?} -- this dispatch row would be routing to unheld content"
            );
        }
    }
}
