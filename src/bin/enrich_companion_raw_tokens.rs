//! `companion`'s counterpart to `enrich_monster_ability_raw_tokens.rs`/
//! `enrich_spell_raw_tokens.rs`/`enrich_equipment_raw_tokens.rs`: populates
//! `data.raw_tokens` on the shipped `companion` corpus JSON records that
//! already carry a `source.kind == "lst_token"` citation but no `raw_tokens`
//! array — the exact gap `corpus_literal_sweep`'s own population rule
//! (`source.kind == "lst_token"` AND `data.raw_tokens` present) leaves a
//! record sitting outside its coverage entirely, at `held` forever, for want
//! of the one field this tool adds.
//!
//! `SD31-E6-F7-001` traced this end to end before writing this file, the same
//! way `SD31-E6-F9-001` traced `monster_ability`'s identical shape: of the
//! kind's 99 `static`+`grounded` (`held`) units, **0 of 99** carried
//! `raw_tokens` (`python3` scan over every `data/corpus/*/companion/*.json`
//! record named by `docs/work-inventory.json`'s own `static`+`grounded`
//! filter). A `static`-classified `companion` unit's `wiring_class` is
//! computed independently, straight off the raw PCGen `.lst` row
//! (`wiring_class::classify`); its `status` is computed independently again,
//! off `companion_chassis::COMPANION_BOOKS`'s own table membership. Neither
//! axis reads `data/corpus/**/*.json` at all. What DOES read it is
//! `corpus_literal_sweep`, whose `--json-out` report is the ONLY thing that
//! can promote a `Static` unit's status to `literal-verified`
//! (`v06_work_inventory::apply_done_rung_stamps`), which is the ONLY status
//! that reaches `done` for a `static` unit
//! (`pf1e_dashboard_producer.doneness_verdict`). A `static` companion record
//! already `grounded` moves to `done` through this single, narrow gate —
//! never by touching `companion_chassis.rs`'s own tables, never by widening
//! the classifier.
//!
//! **Byte-for-byte, not reconstructed.** Reuses `corpus_literal_sweep`'s OWN
//! `tab_tokens`/`token_closure` functions — the exact code the verifier
//! itself runs — so the tokens this tool writes and the tokens the sweep
//! later re-derives from the same citation are computed by one function, not
//! two independently-written ones that could drift apart.
//!
//! Book-agnostic by construction: walks every `data/corpus/*/companion/`
//! directory that exists on disk rather than naming a fixed book list — a
//! `companion` unit's `status` is decided by `companion_chassis::
//! COMPANION_BOOKS`'s table *membership*, not by a fixed enumerable book
//! list the way `spell`'s catalog chain is, so there is no narrower correct
//! scope to name here. This ALSO reaches `companion` records this cycle's
//! own render-readiness check found are NOT yet registered in
//! `COMPANION_BOOKS` (e.g. `beastiary`'s directory spelling vs. the
//! `bestiary` label `docs/work-inventory.json` carries for the same unit,
//! `OPEN-ISSUES.md` row 73) — enriching `raw_tokens` on disk is independent
//! of chassis-table registration, so this tool's own population is simply
//! "every already-shipped `companion` JSON record with an `lst_token`
//! citation and no `raw_tokens` yet," the same scope-derivation
//! `enrich_monster_ability_raw_tokens.rs` used.
//!
//! **PI-safety checked before writing a single byte, per the standing
//! dispatch mandate.** `grep -rl "DESCISPI:YES\|NAMEISPI:YES"` over every
//! `*_races_companion.lst`/`*_abilities_companion.lst`/`ce_*familiar*.lst`
//! file belonging to every book currently registered in `COMPANION_BOOKS`
//! found **zero** hits (re-derived this cycle, not transcribed from the
//! epic-breakdown's "17 registered companion books carry zero declared-PI
//! source tokens" claim — `SD31-E6-F9-001` found that same claim's
//! `monster_ability` counterpart was wrong for `bestiary_4`, so this cycle
//! did not trust it unchecked). `declared_pi_shipping_audit` is re-run
//! after writing, same as every precedent enrichment tool.
//!
//! **R8-04 consolidation:** the file walk, citation resolution, and
//! PI-screen-then-write sequence now live in
//! `codex::rules_core::cache_gen::enrich_raw_tokens_shared` (shared with
//! `enrich_monster_raw_tokens.rs`/`enrich_monster_ability_raw_tokens.rs`/
//! `enrich_spell_raw_tokens.rs` — see that module's doc comment for which
//! axes are configurable and why). This file supplies only this kind's own
//! `CONFIG` and the `data/corpus` book-directory walk.

use std::fs;
use std::path::PathBuf;

use codex::rules_core::cache_gen::enrich_raw_tokens_shared::{
    self as shared, EnrichConfig, Outcome,
};

const KIND_SUBDIR: &str = "companion";

const CONFIG: EnrichConfig = EnrichConfig {
    book_dir_of: shared::book_dir_of_strict,
    identity_fields: &["key", "name", "corpus_key"],
    screen: shared::screen_field_value,
    mark_redacted_root: false,
    remove_file_on_name_pi: true,
};

fn main() {
    let data_root = shared::pcgen_data_root();
    let corpus_root = PathBuf::from("data/corpus");

    let mut total_enriched = 0u32;
    let mut total_no_citation = 0u32;
    let mut total_already = 0u32;
    let mut total_dropped = 0u32;
    let mut misses: Vec<String> = Vec::new();
    let mut dropped: Vec<String> = Vec::new();
    let mut mod_index_cache = std::collections::BTreeMap::new();

    let Ok(book_entries) = fs::read_dir(&corpus_root) else {
        eprintln!("enrich_companion_raw_tokens: no {corpus_root:?} directory found");
        return;
    };
    let mut book_dirs: Vec<PathBuf> = book_entries.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
    book_dirs.sort();

    for book_dir in &book_dirs {
        let book_name = book_dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let files = shared::find_kind_json_files(book_dir, KIND_SUBDIR);
        if files.is_empty() {
            continue;
        }
        let mut book_enriched = 0u32;
        for file in &files {
            match shared::enrich_one(file, &data_root, &mut mod_index_cache, &CONFIG) {
                Outcome::Enriched { .. } => {
                    total_enriched += 1;
                    book_enriched += 1;
                }
                Outcome::NoLstCitation => total_no_citation += 1,
                Outcome::AlreadyEnriched => total_already += 1,
                Outcome::CitationMiss(msg) => misses.push(format!("{}: {}", file.display(), msg)),
                Outcome::DroppedPi(msg) => {
                    total_dropped += 1;
                    dropped.push(msg);
                }
                Outcome::NameIsProductIdentity => {
                    unreachable!("CONFIG.remove_file_on_name_pi is true; NameIsProductIdentity is never produced")
                }
            }
        }
        eprintln!("{book_name}: {} companion files scanned, {book_enriched} enriched", files.len());
    }

    eprintln!(
        "\nenrich_companion_raw_tokens: {total_enriched} enriched, {total_no_citation} no-LST-citation (untouched), {total_already} already-enriched, {total_dropped} dropped for NAMEISPI, {} citation misses",
        misses.len()
    );
    if !dropped.is_empty() {
        eprintln!("\nDropped for NAMEISPI:YES (not fabricated, not shipped):");
        for msg in &dropped {
            eprintln!("  {msg}");
        }
    }
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
    use serde_json::Value;
    use std::collections::BTreeSet;
    use std::path::Path;

    /// A throwaway `PCGEN_CORPUS_ROOT`-shaped book directory plus a
    /// throwaway `data/corpus`-shaped companion JSON, both under
    /// `std::env::temp_dir()` and cleaned up on drop — same pattern
    /// `enrich_monster_ability_raw_tokens.rs`'s own `Scratch` fixture uses.
    struct Scratch {
        data_root: PathBuf,
        corpus_root: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let base = std::env::temp_dir()
                .join(format!("codex_enrich_companion_raw_tokens_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let data_root = base.join("pcgen_data");
            let corpus_root = base.join("data_corpus");
            fs::create_dir_all(data_root.join("pathfinder/paizo/roleplaying_game/x_book")).unwrap();
            fs::create_dir_all(corpus_root.join("x_book/companion")).unwrap();
            Scratch { data_root, corpus_root }
        }

        fn write_lst(&self, contents: &str) {
            fs::write(
                self.data_root.join("pathfinder/paizo/roleplaying_game/x_book/x_abilities_companion.lst"),
                contents,
            )
            .unwrap();
        }

        fn write_json(&self, name: &str, contents: &str) -> PathBuf {
            let path = self.corpus_root.join("x_book/companion").join(name);
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
        assert_eq!(
            shared::split_token_field("TYPE:SpecialQuality.Supernatural"),
            Some(("TYPE", "SpecialQuality.Supernatural"))
        );
        assert_eq!(
            shared::split_token_field("DESC:It shares: a mental link."),
            Some(("DESC", "It shares: a mental link."))
        );
    }

    #[test]
    fn split_token_field_every_result_reconstructs_the_original_field() {
        for field in ["TYPE:SpecialQuality.Supernatural", "CATEGORY:Special Ability", "BONUS:STAT|CON|2"] {
            let (key, value) = shared::split_token_field(field).unwrap();
            assert_eq!(format!("{key}:{value}"), field);
        }
    }

    #[test]
    fn split_token_field_refuses_a_field_with_no_colon() {
        assert_eq!(shared::split_token_field("NoColonAtAll"), None);
    }

    // ----- enrich_one: the real end-to-end path against a throwaway corpus -----

    /// Mirrors the real `eidolon` record traced this cycle
    /// (`advanced_players_guide/companion/eidolon.json`): a `static`,
    /// `grounded` unit whose JSON carries a valid `lst_token` citation but no
    /// `raw_tokens` — the exact shape that keeps every affected `companion`
    /// record in this corpus at `held` today.
    #[test]
    fn enrich_one_adds_the_full_token_closure_byte_for_byte() {
        let scratch = Scratch::new("basic");
        scratch.write_lst("Eidolon\tKEY:Eidolon\tCATEGORY:Special Ability\tTYPE:CompanionAdvancement\tBONUS:STAT|CON|2\n");
        let json_path = scratch.write_json(
            "eidolon.json",
            r#"{"data":{"key":"advanced_players_guide:companion:eidolon","corpus_key":"Eidolon","name":"Eidolon"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_companion.lst","line":1,"record_key":"Eidolon"}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::Enriched { .. }));

        let written: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().expect("raw_tokens array present");
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        let expected: BTreeSet<String> = [
            "KEY:Eidolon",
            "CATEGORY:Special Ability",
            "TYPE:CompanionAdvancement",
            "BONUS:STAT|CON|2",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        assert_eq!(joined, expected);
    }

    #[test]
    fn enrich_one_includes_a_mod_rows_tokens_in_the_closure() {
        let scratch = Scratch::new("modrow");
        scratch.write_lst(
            "Eidolon\tKEY:Eidolon\tTYPE:CompanionAdvancement\nEidolon.MOD\tDESC:Updated text.\n",
        );
        let json_path = scratch.write_json(
            "eidolon.json",
            r#"{"data":{"key":"advanced_players_guide:companion:eidolon","corpus_key":"Eidolon"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_companion.lst","line":1,"record_key":"Eidolon"}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::Enriched { .. }));

        let written: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().unwrap();
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        assert!(
            joined.contains("DESC:Updated text."),
            "the .MOD row's own DESC token must appear in the enriched raw_tokens, not just the base row's"
        );
    }

    /// MUTATION PROOF for the NAMEISPI drop path -- byte-identical shape to
    /// `enrich_monster_raw_tokens.rs`'s own proof against a real Demon Lord
    /// row (SD-30 `§50.3`: a name cannot be redacted, only dropped). This is
    /// the production write path the module doc comment previously only
    /// ASSERTED was safe via an author-time grep; this test proves the call
    /// is actually wired.
    #[test]
    fn enrich_one_drops_a_record_whose_base_row_declares_nameispi() {
        let scratch = Scratch::new("nameispi");
        scratch.write_lst("Ghlaunder\tKEY:Ghlaunder (Companion)\tNAMEISPI:YES\tCATEGORY:Special Ability\n");
        let json_path = scratch.write_json(
            "ghlaunder.json",
            r#"{"data":{"name":"Ghlaunder (Companion)","key":"x_book:companion:ghlaunder"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_companion.lst","line":1,"record_key":"Ghlaunder (Companion)"}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::DroppedPi(_)), "expected a drop, not an enrich");
        assert!(!json_path.exists(), "a NAMEISPI:YES record must be removed from disk, never shipped");
    }

    /// Same proof, but the declaration arrives via a `.MOD` row rather than
    /// the base row -- `declared_product_identity` must read the WHOLE
    /// closure, not just the cited line, exactly as the monster enricher's
    /// own sibling test proves for its own kind.
    #[test]
    fn enrich_one_drops_a_record_whose_mod_row_declares_nameispi() {
        let scratch = Scratch::new("nameispi_mod");
        scratch.write_lst(
            "Ghlaunder\tKEY:Ghlaunder (Companion)\tCATEGORY:Special Ability\nGhlaunder (Companion).MOD\tNAMEISPI:YES\n",
        );
        let json_path = scratch.write_json(
            "ghlaunder.json",
            r#"{"data":{"name":"Ghlaunder (Companion)","key":"x_book:companion:ghlaunder"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_companion.lst","line":1,"record_key":"Ghlaunder (Companion)"}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::DroppedPi(_)));
        assert!(!json_path.exists());
    }

    /// MUTATION PROOF for the blacklist redaction path: a closure token
    /// whose value contains a `PI_BLACKLIST_TERMS` hit is redacted in the
    /// WRITTEN `raw_tokens`, not shipped as prose.
    #[test]
    fn enrich_one_redacts_a_blacklist_term_hit_anywhere_in_the_closure() {
        let scratch = Scratch::new("blacklist");
        scratch.write_lst("Herald\tKEY:Herald\tDESC:A herald blessed by Iomedae herself.\tCATEGORY:Special Ability\n");
        let json_path = scratch.write_json(
            "herald.json",
            r#"{"data":{"name":"Herald","key":"x_book:companion:herald"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_companion.lst","line":1,"record_key":"Herald"}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::Enriched { .. }));

        let written: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().unwrap();
        let desc = tokens.iter().find(|t| t["key"] == "DESC").unwrap();
        assert_eq!(desc["value"], codex::rules_core::shape_b_v1::REDACTED_PI_MARKER, "the deity-name hit must not ship verbatim");
    }

    #[test]
    fn enrich_one_leaves_an_already_enriched_record_untouched() {
        let scratch = Scratch::new("already");
        scratch.write_lst("Eidolon\tTYPE:CompanionAdvancement\n");
        let json_path = scratch.write_json(
            "eidolon.json",
            r#"{"data":{"key":"K","raw_tokens":[{"key":"TYPE","value":"CompanionAdvancement"}]},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_companion.lst","line":1}}"#,
        );
        let before = fs::read_to_string(&json_path).unwrap();
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::AlreadyEnriched));
        assert_eq!(fs::read_to_string(&json_path).unwrap(), before, "already-enriched records must not be rewritten");
    }

    #[test]
    fn enrich_one_reports_a_citation_miss_rather_than_inventing_tokens_for_a_missing_line() {
        let scratch = Scratch::new("miss");
        scratch.write_lst("Eidolon\tTYPE:CompanionAdvancement\n");
        let json_path = scratch.write_json(
            "ghost.json",
            r#"{"data":{"key":"Ghost Companion"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_companion.lst","line":99}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::CitationMiss(_)));
        let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        assert!(after["data"].get("raw_tokens").is_none());
    }

    #[test]
    fn enrich_one_skips_a_non_lst_token_source_without_error() {
        let scratch = Scratch::new("nonlst");
        let json_path = scratch.write_json(
            "web.json",
            r#"{"data":{"key":"Web Second Source Companion"},"source":{"kind":"web_second_source"}}"#,
        );
        let outcome = enrich(&json_path, &scratch.data_root);
        assert!(matches!(outcome, Outcome::NoLstCitation));
    }

    #[test]
    fn find_companion_json_files_finds_flat_files() {
        let scratch = Scratch::new("flatscan");
        scratch.write_json("a.json", "{}");
        scratch.write_json("b.json", "{}");
        let found = shared::find_kind_json_files(&scratch.corpus_root.join("x_book"), KIND_SUBDIR);
        assert_eq!(found.len(), 2);
    }
}
