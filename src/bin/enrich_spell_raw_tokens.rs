//! `spell`'s counterpart to `enrich_monster_ability_raw_tokens.rs`/
//! `enrich_companion_raw_tokens.rs`/`enrich_equipment_raw_tokens.rs`:
//! populates `data.raw_tokens` on the shipped `spell` corpus JSON records
//! that already carry a `source.kind == "lst_token"` citation but no
//! `raw_tokens` array — the exact gap `corpus_literal_sweep`'s own
//! population rule (`source.kind == "lst_token"` AND `data.raw_tokens`
//! present) leaves a record sitting outside its coverage entirely, at
//! `held` forever, for want of the one field this tool adds.
//!
//! **Byte-for-byte, not reconstructed.** Reuses `corpus_literal_sweep`'s OWN
//! `tab_tokens`/`token_closure` functions — the exact code the verifier
//! itself runs — so the tokens this tool writes and the tokens the sweep
//! later re-derives from the same citation are computed by one function, not
//! two independently-written ones that could drift apart. See
//! `enrich_equipment_raw_tokens.rs`'s own doc comment for why a typed
//! re-parse-and-reserialize approach is deliberately avoided.
//!
//! **R8-04 consolidation:** the file walk, citation resolution, and
//! PI-screen-then-write sequence now live in
//! `codex::rules_core::cache_gen::enrich_raw_tokens_shared` (shared with
//! `enrich_companion_raw_tokens.rs`/`enrich_monster_raw_tokens.rs`/
//! `enrich_monster_ability_raw_tokens.rs` — see that module's doc comment
//! for which axes are configurable and why). This bin is the one that sets
//! `remove_file_on_name_pi: false` -- a `NAMEISPI:YES` row is left on disk
//! with `raw_tokens` simply never written, rather than deleted -- and
//! supplies its own broader `screen_raw_token` (covers `BENEFIT`/`SPECIAL`
//! fields too, and treats any non-`Ogl` license as blacklisted).
//!
//! The corpus book directories `spell_resolver::spell_catalog_rows()`
//! models (`spell_book_slug_for` in `v06_work_inventory.rs`) — the only
//! books whose `static` spell units this enrichment can ever move to `done`.
//! Widened 5 -> 8 by `SD31-E6-F2-005`: `ultimate_magic`/`occult_adventures`/
//! `ultimate_combat` now have a `data/corpus/<book>/spell/*.json` cache
//! (`cache_gen::spell_lane_dump`, same cycle) for this tool to enrich.
//! Widened 8 -> 9 by `SD31-E6-F10-001`: `inner_sea_gods` joins the same
//! `cache_gen::spell_lane_dump` cache (a `campaign_setting/` book, not
//! `roleplaying_game/` -- no special-casing needed here, since every path
//! this tool follows is read from the JSON record's own `source.path`
//! field, never assembled from a hardcoded `roleplaying_game/<book>`
//! prefix).
const TARGET_BOOKS: &[&str] = &[
    "core_rulebook",
    "advanced_players_guide",
    "advanced_class_guide",
    "advanced_race_guide",
    "ultimate_intrigue",
    "ultimate_magic",
    "occult_adventures",
    "ultimate_combat",
    "inner_sea_gods",
    // Widened 9 -> 10 (W19-INTEGRATE): `ultimate_wilderness` joined
    // `cache_gen::spell_lane_dump`'s cache this wave (wave-19
    // `ultimate_wilderness` lane + integration-cycle follow-up).
    "ultimate_wilderness",
];

use std::path::PathBuf;

use codex::rules_core::cache_gen::enrich_raw_tokens_shared::{
    self as shared, EnrichConfig, Outcome,
};
use codex::rules_core::pi_screening;
use codex::rules_core::shape_b_v1::{License, REDACTED_PI_MARKER};

const KIND_SUBDIR: &str = "spell";

/// `enrich_spell_raw_tokens`'s own PI screen: broader than the shared
/// [`shared::screen_field_value`] used by companion/monster/monster_ability
/// -- also treats `BENEFIT`/`SPECIAL` as description-shaped fields, and
/// blacklists any non-`Ogl` [`License`] rather than only `PiRedacted`.
fn screen_raw_token(key: &str, value: &str, declared_description: bool) -> (String, bool) {
    let key_upper = key.to_ascii_uppercase();
    let is_desc_field = key_upper == "DESC" || key_upper == "BENEFIT" || key_upper == "SPECIAL";
    let (blacklist_license, ..) = pi_screening::classify_field(key, value);
    let blacklisted = blacklist_license != License::Ogl;
    let redact = blacklisted || (declared_description && is_desc_field);
    let stored_value = if redact { REDACTED_PI_MARKER.to_string() } else { value.to_string() };
    (stored_value, redact)
}

const CONFIG: EnrichConfig = EnrichConfig {
    book_dir_of: shared::book_dir_of_strict,
    identity_fields: &["key", "name"],
    screen: screen_raw_token,
    mark_redacted_root: false,
    remove_file_on_name_pi: false,
};

fn main() {
    let data_root = shared::pcgen_data_root();
    let corpus_root = PathBuf::from("data/corpus");

    let mut total_enriched = 0u32;
    let mut total_no_citation = 0u32;
    let mut total_already = 0u32;
    let mut total_name_pi_blocked = 0u32;
    let mut misses: Vec<String> = Vec::new();
    let mut mod_index_cache = std::collections::BTreeMap::new();

    for book in TARGET_BOOKS {
        let book_dir = corpus_root.join(book);
        if !book_dir.is_dir() {
            continue;
        }
        let files = shared::find_kind_json_files(&book_dir, KIND_SUBDIR);
        let mut book_enriched = 0u32;
        for file in &files {
            match shared::enrich_one(file, &data_root, &mut mod_index_cache, &CONFIG) {
                Outcome::Enriched { .. } => {
                    total_enriched += 1;
                    book_enriched += 1;
                }
                Outcome::NoLstCitation => total_no_citation += 1,
                Outcome::AlreadyEnriched => total_already += 1,
                Outcome::NameIsProductIdentity => total_name_pi_blocked += 1,
                Outcome::CitationMiss(msg) => misses.push(format!("{}: {}", file.display(), msg)),
                Outcome::DroppedPi(_) => {
                    unreachable!("CONFIG.remove_file_on_name_pi is false; DroppedPi is never produced")
                }
            }
        }
        eprintln!("{book}: {} spell files scanned, {book_enriched} enriched", files.len());
    }

    eprintln!(
        "\nenrich_spell_raw_tokens: {total_enriched} enriched, {total_no_citation} no-LST-citation (untouched), {total_already} already-enriched, {total_name_pi_blocked} name-PI-blocked, {} citation misses",
        misses.len()
    );
    if !misses.is_empty() {
        eprintln!("\nCitation misses (not enriched, real gaps to investigate):");
        for miss in &misses {
            eprintln!("  {miss}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::corpus_literal_sweep::token_closure;
    use serde_json::Value;
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;

    /// A throwaway `PCGEN_CORPUS_ROOT`-shaped book directory plus a
    /// throwaway `data/corpus`-shaped spell JSON, both under
    /// `std::env::temp_dir()` and cleaned up on drop -- the same pattern
    /// `v06_work_inventory.rs`'s own `ScratchBook`/`ScratchSpellBook` test
    /// fixtures use, not a new one invented here.
    struct Scratch {
        data_root: PathBuf,
        corpus_root: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let base = std::env::temp_dir()
                .join(format!("codex_enrich_spell_raw_tokens_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let data_root = base.join("pcgen_data");
            let corpus_root = base.join("data_corpus");
            fs::create_dir_all(data_root.join("pathfinder/paizo/roleplaying_game/x_book")).unwrap();
            fs::create_dir_all(corpus_root.join("x_book/spell")).unwrap();
            Scratch { data_root, corpus_root }
        }

        fn write_lst(&self, contents: &str) {
            fs::write(
                self.data_root.join("pathfinder/paizo/roleplaying_game/x_book/x_spells.lst"),
                contents,
            )
            .unwrap();
        }

        fn write_json(&self, name: &str, contents: &str) -> PathBuf {
            let path = self.corpus_root.join("x_book/spell").join(name);
            fs::write(&path, contents).unwrap();
            path
        }

        /// A record nested one level under `spell/`, the `core_rulebook`
        /// corpus shape (`spell/level_0/…json`) a single-level `read_dir`
        /// silently misses entirely.
        fn write_json_nested(&self, subdir: &str, name: &str, contents: &str) -> PathBuf {
            let dir = self.corpus_root.join("x_book/spell").join(subdir);
            fs::create_dir_all(&dir).unwrap();
            let path = dir.join(name);
            fs::write(&path, contents).unwrap();
            path
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.data_root.parent().unwrap());
        }
    }

    fn enrich(path: &Path, data_root: &Path) -> Outcome {
        let mut cache = std::collections::BTreeMap::new();
        shared::enrich_one(path, data_root, &mut cache, &CONFIG)
    }

    // ----- split_token_field: the round-trip the whole tool depends on -----

    #[test]
    fn split_token_field_splits_on_the_first_colon_only() {
        assert_eq!(shared::split_token_field("COST:150"), Some(("COST", "150")));
        assert_eq!(
            shared::split_token_field("DESC:You place a curse: it triggers later."),
            Some(("DESC", "You place a curse: it triggers later."))
        );
    }

    #[test]
    fn split_token_field_every_result_reconstructs_the_original_field() {
        for field in ["SCHOOL:Transmutation", "CLASSES:Wizard=1|Sorcerer=1", "COMPS:V, S, M"] {
            let (key, value) = shared::split_token_field(field).unwrap();
            assert_eq!(format!("{key}:{value}"), field);
        }
    }

    #[test]
    fn split_token_field_refuses_a_field_with_no_colon() {
        assert_eq!(shared::split_token_field("NoColonAtAll"), None);
    }

    // ----- find_kind_json_files: the recursive-scan fix, pinned -----

    /// Caught live against `core_rulebook`'s real corpus before this test
    /// existed: a single-level `read_dir` reported "0 spell files scanned"
    /// for a book whose `spell/` directory nests one subdirectory per spell
    /// level and therefore contains zero files directly in `spell/` itself.
    #[test]
    fn find_spell_json_files_walks_into_level_subdirectories() {
        let scratch = Scratch::new("nested_scan");
        scratch.write_json_nested("level_0", "acid_splash.json", "{}");
        scratch.write_json_nested("level_4", "curse_of_burning_sleep.json", "{}");
        let found = shared::find_kind_json_files(&scratch.corpus_root.join("x_book"), KIND_SUBDIR);
        let names: BTreeSet<String> = found
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            names,
            ["acid_splash.json".to_string(), "curse_of_burning_sleep.json".to_string()]
                .into_iter()
                .collect(),
            "both nested files must be found, not just files directly under spell/"
        );
    }

    // ----- enrich_one: the real end-to-end path against a throwaway corpus -----

    #[test]
    fn enrich_one_adds_the_full_token_closure_byte_for_byte() {
        let scratch = Scratch::new("basic");
        scratch.write_lst("Blade Lash\tSCHOOL:Transmutation\tCLASSES:Bloodrager=1\tDESC:Elongated blade.\n");
        let json_path = scratch.write_json(
            "blade_lash.json",
            r#"{"data":{"key":"Blade Lash"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        // Signature test note: `enrich_one`'s `data_root` parameter is the raw
        // PCGen checkout root, matching `pcgen_data_root()`'s real return
        // value -- the scratch fixture keeps the same two-root split so this
        // test cannot pass by accident of the two roots coinciding.
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::Enriched { .. }));

        let written: Value =
            serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().expect("raw_tokens array present");
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        let expected: BTreeSet<String> = [
            "SCHOOL:Transmutation",
            "CLASSES:Bloodrager=1",
            "DESC:Elongated blade.",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        assert_eq!(joined, expected);

        // The round-trip this whole tool exists to satisfy: re-deriving the
        // closure independently (as `corpus_literal_sweep` itself would) and
        // checking every written token is a member of it.
        let base_row = fs::read_to_string(
            scratch.data_root.join("pathfinder/paizo/roleplaying_game/x_book/x_spells.lst"),
        )
        .unwrap();
        let mod_index =
            shared::mod_index_for_book(&scratch.data_root, "pathfinder/paizo/roleplaying_game/x_book");
        let closure = token_closure(
            base_row.split('\n').next().unwrap(),
            &["Blade Lash".to_string()].into_iter().collect(),
            &mod_index,
            None,
        );
        for field in &joined {
            assert!(closure.contains(field), "{field} must be a member of the independently-derived closure");
        }
    }

    #[test]
    fn enrich_one_includes_a_mod_rows_tokens_in_the_closure() {
        let scratch = Scratch::new("modrow");
        scratch.write_lst(
            "Blade Lash\tSCHOOL:Transmutation\tCLASSES:Bloodrager=1\nBlade Lash.MOD\tDESC:Use like a whip.\n",
        );
        let json_path = scratch.write_json(
            "blade_lash.json",
            r#"{"data":{"key":"Blade Lash"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::Enriched { .. }));

        let written: Value =
            serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().unwrap();
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        assert!(
            joined.contains("DESC:Use like a whip."),
            "the .MOD row's own DESC token must appear in the enriched raw_tokens, not just the base row's"
        );
    }

    #[test]
    fn enrich_one_leaves_an_already_enriched_record_untouched() {
        let scratch = Scratch::new("already");
        scratch.write_lst("Blade Lash\tSCHOOL:Transmutation\n");
        let json_path = scratch.write_json(
            "blade_lash.json",
            r#"{"data":{"key":"Blade Lash","raw_tokens":[{"key":"SCHOOL","value":"Transmutation"}]},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let before = fs::read_to_string(&json_path).unwrap();
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::AlreadyEnriched));
        assert_eq!(fs::read_to_string(&json_path).unwrap(), before, "already-enriched records must not be rewritten");
    }

    #[test]
    fn enrich_one_reports_a_citation_miss_rather_than_inventing_tokens_for_a_missing_line() {
        let scratch = Scratch::new("miss");
        scratch.write_lst("Blade Lash\tSCHOOL:Transmutation\n");
        let json_path = scratch.write_json(
            "ghost.json",
            r#"{"data":{"key":"Ghost Spell"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":99}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::CitationMiss(_)));
        assert!(!json_path.exists() || {
            let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
            after["data"].get("raw_tokens").is_none()
        });
    }

    // ----- PI screen on raw_tokens: the SD-30 hard-stop, both contracts -----

    #[test]
    fn enrich_one_hard_stops_when_the_row_declares_nameispi_yes() {
        let scratch = Scratch::new("name_pi");
        scratch.write_lst("Secret Rite\tNAMEISPI:YES\tSCHOOL:Transmutation\tDESC:A rite.\n");
        let json_path = scratch.write_json(
            "secret_rite.json",
            r#"{"data":{"key":"Secret Rite"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::NameIsProductIdentity));
        let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        assert!(
            after["data"].get("raw_tokens").is_none(),
            "a NAMEISPI:YES row must never get raw_tokens written"
        );
    }

    #[test]
    fn enrich_one_redacts_a_raw_token_whose_value_hits_the_blacklist() {
        let scratch = Scratch::new("blacklist_hit");
        scratch.write_lst(
            "Iomedae's Blessing\tSCHOOL:Evocation\tDESC:A blessing granted by Iomedae herself.\n",
        );
        let json_path = scratch.write_json(
            "iomedaes_blessing.json",
            r#"{"data":{"key":"Iomedae's Blessing"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::Enriched { .. }));
        let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = after["data"]["raw_tokens"].as_array().unwrap();
        let desc_value = tokens
            .iter()
            .find(|t| t["key"].as_str() == Some("DESC"))
            .and_then(|t| t["value"].as_str())
            .unwrap();
        assert_eq!(
            desc_value, "[redacted PI]",
            "a raw_tokens value carrying a blacklisted term must be redacted, not shipped verbatim"
        );
        let school_value = tokens
            .iter()
            .find(|t| t["key"].as_str() == Some("SCHOOL"))
            .and_then(|t| t["value"].as_str())
            .unwrap();
        assert_eq!(school_value, "Evocation", "a clean field must ship unredacted");
    }

    #[test]
    fn enrich_one_redacts_a_desc_field_when_the_row_declares_descispi_yes() {
        let scratch = Scratch::new("desc_pi");
        scratch.write_lst("Hidden Ward\tDESCISPI:YES\tSCHOOL:Abjuration\tDESC:A ward of great secrecy.\n");
        let json_path = scratch.write_json(
            "hidden_ward.json",
            r#"{"data":{"key":"Hidden Ward"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::Enriched { .. }));
        let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = after["data"]["raw_tokens"].as_array().unwrap();
        let desc_value = tokens
            .iter()
            .find(|t| t["key"].as_str() == Some("DESC"))
            .and_then(|t| t["value"].as_str())
            .unwrap();
        assert_eq!(desc_value, "[redacted PI]", "a DESCISPI:YES-declared row's DESC token must be redacted");
    }

    #[test]
    fn enrich_one_skips_a_non_lst_token_source_without_error() {
        let scratch = Scratch::new("nonlst");
        let json_path = scratch.write_json(
            "web.json",
            r#"{"data":{"key":"Web Second Source Spell"},"source":{"kind":"web_second_source"}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::NoLstCitation));
    }
}
