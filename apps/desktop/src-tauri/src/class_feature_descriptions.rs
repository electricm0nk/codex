//! Real corpus-description surface for `class_feature` records
//! (SD31-D7-PROSE-003).
//!
//! # Why this module exists
//!
//! `ClassFeatureRow.detail` (`characterHub/classFeaturesModel.ts`) renders
//! `ExplanationDto.detail` -- the rules engine's own COMPUTED derivation
//! text, cited from `pilot_compute.rs`'s explanations. That is real and
//! correct for what it is, but it is not the record's own rulebook text: a feature
//! whose engine derivation is a bare `+2` magnitude with no accompanying
//! prose carries no rulebook description anywhere on the character sheet.
//! `SD31-D7-PROSE-001`/`002` built the equivalent surface for `race_trait`
//! (`race_trait_picker.rs`) and `monster_ability`
//! (`monster_catalog::serve_ability_description`); `class_feature` was
//! deliberately left for its own cycle because, unlike those two kinds,
//! nothing in this engine already carries a class feature's corpus
//! description onto the wire -- this module is that missing render path,
//! built to the same shape.
//!
//! # Source of truth: the record's identity from the corpus cache, its words
//! from the converted package
//!
//! `src/pcgen_import/cache_gen/class_feature.rs` (SD31-E5-F1-001) transcribes
//! every in-scope `class_feature` unit's real corpus row into
//! `data/corpus/<book>/class_feature/<class-slug>/<feature-slug>.json`, and
//! this module reads that cache for the four plain identity fields it needs
//! (`key`, `name`, `class`, and the book directory the record sits in) --
//! exactly the way `corpus_full.rs` reads `data/corpus/<book>/equipment/`, via
//! `authoring_workbench::codex_repo_root()`.
//!
//! **The description no longer comes from there.** SD-35 `decisions.md §11`:
//! nothing on the live side reads the ingest format's own description string,
//! its argument tail, or its placeholders. The substitution this module used to
//! perform at run time already happened at ingest
//! (`src/pcgen_import/sheet_rule/`); the served text is now the converted
//! record's own words, read through
//! [`codex::rules_core::sheet_rule_catalog::catalog_description`] -- one final
//! number, dice in final form, or the rule's words for a term no catalog screen
//! can settle (`decisions.md §1`). The join is the record's own converted id,
//! [`converted_id`].
//!
//! **Same packaging caveat as `corpus_full.rs`**: this works for a source
//! checkout and any deployment that sets `CODEX_REPO_ROOT` to a location
//! carrying `data/corpus/`; bundling `data/corpus/` into a packaged Tauri
//! installer is separate, already-tracked follow-on work, not assumed here.
//! `data/sheet_rules/` is loaded by `corpus_loader::live_sheet_rules`, which
//! resolves it from the library crate's own manifest directory.
//!
//! # The join: `(class_slug, feature_slug)`, not a stored crosswalk
//!
//! `ExplanationDto.id` (e.g. `class_feature.rogue.sneak_attack`) carries no
//! corpus citation -- the engine's own id is a computed-record identity, not
//! a corpus key. `Kind::ClassFeature`'s own `classify()` arm
//! (`v06_work_inventory.rs`) already trusts exactly this join to decide
//! `grounded`: `id.contains(".{owner}.") && id.ends_with(&feature_slug)`,
//! where `owner` is a class name the engine models and `feature_slug` is
//! `slug()` of the corpus feature name. This module computes the SAME two
//! slugs (`slug()`, transcribed byte-for-byte from
//! `v06_work_inventory.rs::slug` -- reproduced locally rather than shared,
//! per this package's disjoint-file-touch convention every `cache_gen::*`
//! module and `corpus_full.rs` already follow) from the cache record's own
//! `data.class`/`data.name`, and the frontend (`classFeaturesModel.ts`)
//! performs the identical suffix/contains match against the real
//! `explanation.id` it already has. Reusing an already-adjudicated matching
//! rule rather than inventing a second one is deliberate: the frontend's own
//! join can never be more permissive than the join the board's own doneness
//! measurement already trusts.
//!
//! # PI screening
//!
//! Already discharged upstream: `cache_gen::class_feature::generate` runs
//! both SD-30 contracts (`§52.3` blacklist sweep, `§53.5` declared-PI reader)
//! on NAME and DESCRIPTION before a record is ever written to
//! `data/corpus/`, and a `NAMEISPI:YES` row is not written at all. This
//! module reads only already-screened output; it re-runs no PI check of its
//! own, the same trust boundary `corpus_full.rs` and every other
//! `data/corpus/`-reading module in this crate already holds.
//!
//! # The leak guard is gone, because there is nothing left to leak
//!
//! This module used to render the ingest format's description at run time and
//! then check the output for surviving syntax (`leaked_pcgen_syntax`), skipping
//! a record whose render mis-split. Both the render and the check are gone: the
//! converted package carries no token, no formula string, no argument tail and
//! no positional placeholder, which `sheet_rule_convert -- --check` and the
//! package-wide source-marker grep over `data/sheet_rules/` in
//! `workflow-instruction.md §6` prove for the whole package rather than per
//! record. The two refusals that remain are stated over our own schema:
//! a record with no converted rule, and a converted rule that states no
//! descriptive prose at all ([`catalog_description`]'s `None` arm).

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Serialize;
use serde_json::Value;

use codex::rules_core::corpus_loader::live_sheet_rules;
use codex::rules_core::sheet_rule::SheetRulePackage;
use codex::rules_core::sheet_rule_catalog::catalog_description;

use crate::authoring_workbench::codex_repo_root;

/// One class feature's real corpus description, joined to the engine
/// explanation ids that may claim it. See the module doc comment's "The
/// join" section for exactly how `classSlug`/`featureSlug` are used.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClassFeatureDescriptionDto {
    /// The corpus book directory this record was read from
    /// (`data/corpus/<book>/class_feature/...`).
    pub book: String,
    /// `slug()` of the record's `data.class` value (`"Rogue"` -> `"rogue"`),
    /// the same token an `ExplanationDto.id` carries for a class the engine
    /// models.
    pub class_slug: String,
    /// `slug()` of the record's `data.name` value, matched against the
    /// TRAILING segment(s) of an `ExplanationDto.id` via `endsWith` -- never
    /// equality, because some ids carry an extra `corpus_record` segment
    /// (`classFeaturesModel.ts`'s own `RECORD_FAMILY_SEGMENTS`).
    pub feature_slug: String,
    /// The corpus `KEY:` token verbatim (`"Rogue ~ Sneak Attack"`).
    pub key: String,
    pub name: String,
    /// The converted record's own words ([`catalog_description`]) -- one final
    /// number, dice in final form, or the rule's words for a term no catalog
    /// screen can settle. Never `null` and never empty: a record whose
    /// converted rule states no descriptive prose is not emitted at all.
    pub description: String,
    /// `None` for every record this module itself emits. `Some(<exact feat
    /// name>)` is `class_feature_feat_bridge.rs`'s own addition (T4-L9,
    /// `epic-2-cause-closure`): that module's records carry a synthetic
    /// pool-group `class_slug` (e.g. `"golden_legionnaire"`) rather than a
    /// real class token, so the class-held reachability join
    /// (`unmatchedClassFeatureDescriptions`'s `heldTokens.has(d.classSlug)`)
    /// can never match them by construction. This field names the ALREADY-
    /// VERIFIED feat the record's sole content grants (the exact string
    /// `feat_description_by_exact_name` matched on), so the frontend can
    /// gate reachability on the character HOLDING THAT FEAT instead --
    /// `feat_identity::holds`'s own fold, reproduced client-side by
    /// `featsTabModel.ts::normalizeFeatIdentity` (see that module's own doc
    /// comment on why the two folds must stay identical). Left as a plain
    /// `String`, not the class feature's own `name`/`feature_slug`: some
    /// bridge records' class-feature name differs from the feat name they
    /// grant, and only the token `feat_description_by_exact_name` itself
    /// matched on can be trusted as the held-feat identity.
    pub granted_feat: Option<String>,
}

/// Reproduced from `v06_work_inventory.rs::slug` -- see this module's own doc
/// comment on why it is a local copy rather than a shared import.
fn slug(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut last_underscore = true;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            last_underscore = false;
        } else if !last_underscore {
            out.push('_');
            last_underscore = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    out
}

/// Reproduced from `v06_work_inventory.rs::is_real_description_value` --
/// same three refusals (empty, `.CLEAR`/`.CLEARALL`, the PI-redaction
/// marker), same reproduced-locally convention as `slug` above.
fn is_real_description_value(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !matches!(lower.as_str(), ".clear" | ".clearall" | "[redacted pi]")
}

fn walk_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk_json_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

/// The converted rule id for a corpus `class_feature` record: the record's own
/// `data.key` slugged, under its book. `beastiary` is the corpus directory's
/// historical spelling of the book the converter writes as `bestiary` -- the
/// same one-line fold `companion_pool_catalog::converted_id` carries.
pub(crate) fn converted_id(corpus_book: &str, corpus_key: &str) -> String {
    let book = if corpus_book == "beastiary" { "bestiary" } else { corpus_book };
    format!("{book}:class_feature:{}", codex::rules_core::sheet_rule::slug(corpus_key))
}

/// Reads every `class_feature` cache record under `<repo_root>/data/corpus/
/// */class_feature/**/*.json`, keeping only the ones with a resolvable owning
/// class (`data.class`) whose converted rule states descriptive prose -- a
/// pool-member record with no `~`-split owner has no class to join against and
/// is correctly absent from this catalog, exactly as it is absent from
/// `Kind::ClassFeature`'s `class_feature_owner` matching in
/// `v06_work_inventory.rs`.
fn load_class_feature_descriptions(repo_root: &Path) -> Vec<ClassFeatureDescriptionDto> {
    let corpus_root = repo_root.join("data/corpus");
    let mut out = Vec::new();
    let package: Option<&'static SheetRulePackage> = live_sheet_rules();
    let Ok(books) = std::fs::read_dir(&corpus_root) else { return out };
    let mut book_dirs: Vec<_> = books.flatten().collect();
    book_dirs.sort_by_key(|e| e.file_name());
    for book_entry in book_dirs {
        let book_dir = book_entry.path();
        if !book_dir.is_dir() {
            continue;
        }
        let book = book_entry.file_name().to_string_lossy().to_string();
        let cf_dir = book_dir.join("class_feature");
        if !cf_dir.is_dir() {
            continue;
        }
        let mut files = Vec::new();
        walk_json_files(&cf_dir, &mut files);
        for file in files {
            let Ok(text) = std::fs::read_to_string(&file) else { continue };
            let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
            let data = &doc["data"];
            let (Some(key), Some(name), Some(class)) =
                (data["key"].as_str(), data["name"].as_str(), data["class"].as_str())
            else {
                continue;
            };
            // The converted record's own words. Everything this call site used
            // to do at run time -- read the ingest format's description string,
            // split its argument tail, substitute its positional placeholders,
            // then check the result for surviving syntax -- happened at ingest
            // instead (`src/pcgen_import/sheet_rule/`, `decisions.md §11`).
            let Some(package) = package else { continue };
            let Some(rule) = package.rule(&converted_id(&book, key)) else { continue };
            // The refuse gate, restated over the converted package: a record
            // whose rule states no descriptive prose at all has nothing to
            // serve. Same disposition as before -- refused, never a partial or
            // fabricated sentence -- asked of our own schema.
            let Some(description) = catalog_description(package, rule) else { continue };
            if !is_real_description_value(&description) {
                continue;
            }
            out.push(ClassFeatureDescriptionDto {
                book: book.clone(),
                class_slug: slug(class),
                feature_slug: slug(name),
                key: key.to_string(),
                name: name.to_string(),
                description,
                granted_feat: None,
            });
        }
    }
    out
}

/// Built once, cached for the process lifetime -- mirrors
/// `corpus_full::full_corpus_bundle()`'s own caching shape.
fn class_feature_descriptions() -> &'static Vec<ClassFeatureDescriptionDto> {
    static TABLE: OnceLock<Vec<ClassFeatureDescriptionDto>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let repo_root = codex_repo_root()
            .expect("codex repo root must resolve for class_feature description loading");
        load_class_feature_descriptions(&repo_root)
    })
}

#[tauri::command]
pub fn list_class_feature_descriptions() -> Vec<ClassFeatureDescriptionDto> {
    class_feature_descriptions().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        codex_repo_root().expect("repo root resolves under `cargo test`")
    }

    #[test]
    fn slug_matches_v06_work_inventorys_own_algorithm() {
        assert_eq!(slug("Sneak Attack"), "sneak_attack");
        assert_eq!(slug("Unchained Summoner"), "unchained_summoner");
        assert_eq!(slug("Rogue"), "rogue");
        assert_eq!(slug("  Trailing/Punct!! "), "trailing_punct");
    }

    #[test]
    fn is_real_description_value_refuses_empty_clear_and_the_pi_marker() {
        assert!(!is_real_description_value(""));
        assert!(!is_real_description_value("   "));
        assert!(!is_real_description_value(".CLEAR"));
        assert!(!is_real_description_value(".CLEARALL"));
        assert!(!is_real_description_value("[redacted PI]"));
        assert!(is_real_description_value("You gain a bonus."));
    }

    /// The real corpus loads and carries thousands of real, described class
    /// features -- proven against the live `data/corpus/` checkout, not a
    /// fixture.
    #[test]
    fn loads_thousands_of_real_described_class_features_from_the_live_corpus() {
        let descriptions = load_class_feature_descriptions(&repo_root());
        assert!(
            descriptions.len() > 1000,
            "expected thousands of real described class_feature records, got {}",
            descriptions.len()
        );
        // `Rogue ~ Sneak Attack` itself carries `description: null` in the
        // real corpus (its rules text lives on the class table, not a
        // per-feature description) -- not a fixture stand-in for that record,
        // `Aberrant Bloodline ~ Aberrant Form` genuinely does carry a real,
        // `%N`-free description (SD-31 wave 26: `Rogue ~ Trapfinding` USED
        // to be this test's example, but its description carries an
        // unresolved `%1` and is no longer served by this catalog -- see
        // `a_description_whose_percent_n_argument_has_no_character_context_
        // to_resolve_it_is_not_served`, below).
        //
        // **SD-32 card 11 (`epic-2-cause-closure`, T2a/T12 combined cycle)
        // correction:** `class_slug` used to assert `"aberrant_bloodline"`
        // -- the group-prefix category label, exactly T2a's own shape
        // ("`data.class` read from the wrong place"). `cache_gen::class_
        // feature::generate`'s pool-catalog fallback now resolves this
        // record's real owner (Aberrant is a Sorcerer bloodline; the
        // `Bloodline` -> `Sorcerer` pool entry), so the live corpus now
        // ships `data.class: "Sorcerer"` for this exact record -- which is
        // what lets `classSlug` actually join a real `class_feature.sorcerer.*`
        // `ExplanationDto.id` on the character sheet instead of a slug no
        // explanation id can ever match. This assertion is updated to the
        // now-correct value, not loosened.
        let aberrant_form = descriptions
            .iter()
            .find(|d| d.book == "core_rulebook" && d.key == "Aberrant Bloodline ~ Aberrant Form");
        let record = aberrant_form
            .expect("core_rulebook's real Aberrant Bloodline ~ Aberrant Form record must be in the catalog");
        assert_eq!(record.class_slug, "sorcerer");
        assert_eq!(record.feature_slug, "aberrant_form");
        assert!(record.description.starts_with("Your body becomes truly unnatural."));
        assert!(!record.description.contains('|'), "the pipe-arg tail must never leak into prose");
    }

    /// SD-35 `AT-35-E6-003` cycle 4 -- the same record, the opposite
    /// disposition, and the reason the disposition changed.
    ///
    /// `Rogue ~ Trapfinding`'s real rulebook sentence is "You add +%1 to
    /// Perception skill checks ...". When this module rendered the ingest
    /// format at run time, the `%1` had no character context to fill, the
    /// renderer dropped the placeholder AND the `+` that introduced it, and the
    /// honest response was to refuse the record entirely -- a player saw no
    /// Trapfinding text at all. The converter now carries that hole as a TYPED
    /// slot over the term it stands on, and `catalog_description` prints the
    /// term's words where no number can be settled (`decisions.md §1` form 3).
    /// So the record is served, with its own sentence, naming what the number
    /// depends on instead of silently losing it.
    #[test]
    fn the_record_that_used_to_be_refused_for_an_unfillable_number_now_serves_its_words() {
        let descriptions = load_class_feature_descriptions(&repo_root());
        let trapfinding = descriptions
            .iter()
            .find(|d| d.book == "core_rulebook" && d.key == "Rogue ~ Trapfinding")
            .expect("Rogue ~ Trapfinding must now be served: its slot prints the term's words");
        assert!(
            trapfinding.description.starts_with("You add +"),
            "the sentence must keep the sign the old render path dropped: {:?}",
            trapfinding.description
        );
        assert!(
            trapfinding.description.contains("to Perception skill checks made to locate traps"),
            "the rest of the real sentence must survive: {:?}",
            trapfinding.description
        );
        assert!(
            !carries_a_positional_placeholder(&trapfinding.description),
            "no `%N` may reach the screen: {:?}",
            trapfinding.description
        );
    }

    /// The same for the one record whose run-time render was known to MIS-SPLIT
    /// -- `Enhancement Savant Subschool ~ Perfection of Self` declared two
    /// pipe-separated arguments while its prose referenced only `%1`, so the
    /// renderer's segment-count heuristic left a literal `|` in the output and
    /// this module refused it. The converter splits the source row structurally
    /// rather than by counting segments, so the record now serves a clean
    /// sentence with both of its holes printed as words.
    #[test]
    fn the_record_whose_run_time_render_used_to_mis_split_now_serves_a_clean_sentence() {
        let descriptions = load_class_feature_descriptions(&repo_root());
        let record = descriptions
            .iter()
            .find(|d| d.key == "Enhancement Savant Subschool ~ Perfection of Self")
            .expect("the formerly-refused mis-splitting record must now be served");
        assert!(
            record.description.starts_with("As a swift action you can grant yourself a +"),
            "{:?}",
            record.description
        );
        assert!(record.description.ends_with("times per day."), "{:?}", record.description);
        assert!(!record.description.contains('|'), "{:?}", record.description);
        assert!(
            !carries_a_positional_placeholder(&record.description),
            "{:?}",
            record.description
        );
    }

    /// `%1`, `%2`, ... -- the source format's positional placeholder, the one
    /// thing that must never reach a player's eye. Deliberately NOT a bare `%`
    /// or a bare `|`: real rulebook prose uses both (`"increased by half
    /// (+50%)"`, a spell's damage table row), and a test that banned the
    /// characters rather than the construct would be measuring English, not
    /// ingest-format residue -- it flagged 173 clean sentences when it was
    /// first written this way (`AT-35-E6-003` cycle 4).
    fn carries_a_positional_placeholder(text: &str) -> bool {
        let bytes = text.as_bytes();
        bytes
            .iter()
            .enumerate()
            .any(|(i, b)| *b == b'%' && bytes.get(i + 1).is_some_and(u8::is_ascii_digit))
    }

    /// Corpus-wide, over the whole served population: no served description
    /// carries a positional placeholder. This replaces the two run-time guards
    /// this module used to run per record -- the dropped-argument check and the
    /// leaked-syntax check -- both of them properties of a render this module no
    /// longer performs. The rest of what they protected (no source-format marker
    /// anywhere in the served text) is proven for the WHOLE package, all 49,438
    /// units, by `workflow-instruction.md §6`'s source-marker grep over
    /// `data/sheet_rules/`; restating it per served row here would be a second,
    /// weaker copy of a gate that already runs every cycle.
    #[test]
    fn no_served_description_carries_a_positional_placeholder() {
        let descriptions = load_class_feature_descriptions(&repo_root());
        assert!(
            descriptions.len() > 1000,
            "no real descriptions were checked; the check proved nothing: {}",
            descriptions.len()
        );
        let mut offenders: Vec<&str> = Vec::new();
        for record in &descriptions {
            let text = record.description.as_str();
            if carries_a_positional_placeholder(text) {
                offenders.push(record.key.as_str());
            }
        }
        assert!(
            offenders.is_empty(),
            "{} served description(s) carry a `%N`: {:?}",
            offenders.len(),
            offenders.iter().take(8).collect::<Vec<_>>()
        );
    }

    /// Mutation-proves-RED per the universal requirement, asked of the LIVE
    /// converted package rather than a hand-written fixture (`decisions.md §4`):
    /// across every converted `class_feature` rule, some state descriptive
    /// prose and some state none, so `catalog_description`'s `Some` arm AND its
    /// `None` arm -- this module's whole refusal -- are both reached. A gate
    /// that answered the same way for every record would be the vacuous one
    /// this test exists to catch.
    #[test]
    fn the_refuse_gate_is_provably_live_over_the_converted_package() {
        let package = live_sheet_rules().expect(
            "data/sheet_rules/ must be present (cargo run --locked --bin sheet_rule_convert)",
        );
        let mut with_prose = 0usize;
        let mut without_prose = 0usize;
        for (id, rule) in &package.rules {
            if !id.contains(":class_feature:") {
                continue;
            }
            match catalog_description(package, rule) {
                Some(text) if is_real_description_value(&text) => with_prose += 1,
                _ => without_prose += 1,
            }
        }
        println!("CLASS_FEATURE_RULES with_prose={with_prose} without_prose={without_prose}");
        assert!(with_prose > 0, "no converted class_feature record states any descriptive prose");
        assert!(without_prose > 0, "the refuse arm is never reached -- the gate is vacuous");
    }

    /// The population ratchet. `AT-35-E6-003` cycle 4 measured the served
    /// population on both sides of the swap with a temporary census: **8,895
    /// before, 11,877 after**, 23 of the 8,895 lost -- every one of them a
    /// corpus record with no converted rule at all, named in that cycle's
    /// receipt. A floor, not an identity: a cycle that lost a row and gained a
    /// different one would pass it.
    #[test]
    fn the_served_population_never_falls_below_its_recorded_floor() {
        let descriptions = load_class_feature_descriptions(&repo_root());
        assert!(
            descriptions.len() >= 11_800,
            "the served class_feature description population fell below its recorded floor: {}",
            descriptions.len()
        );
        for record in &descriptions {
            assert!(
                !record.description.trim().is_empty(),
                "{:?}: a served row must never reach the wire with an empty description",
                record.key
            );
        }
    }

    #[test]
    fn list_class_feature_descriptions_returns_the_cached_table() {
        let a = list_class_feature_descriptions();
        let b = list_class_feature_descriptions();
        assert_eq!(a.len(), b.len());
        assert!(!a.is_empty());
    }
}

