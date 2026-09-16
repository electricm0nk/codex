//! The `class_feature` -> `feat` cross-reference bridge (SD31-W29-
//! CLASSFEATURE-FEATBRIDGE-001, wave 29, THE-BOX §2.1 F2).
//!
//! # What this closes
//!
//! `THE-BOX.md` §2.1 F2 names a distinct, provable `class_feature` shape: a
//! record whose entire content is a grant of an already-separately-modelled
//! `feat` -- "the class feature IS a feat." A real example is
//! `adventurers_guide:class_feature:golden_legionnaire_swift_aid`, whose
//! converted rule states no descriptive prose of its own at all: the record's
//! whole job is the grant, not the content. A player reading this class feature
//! is owed the real Swift Aid feat text, which already exists in the converted
//! package. This module is that bridge: it finds every class feature whose sole
//! content is one feat grant and serves the granted feat's own words in the
//! same [`ClassFeatureDescriptionDto`] shape `class_feature_descriptions.rs`
//! already emits -- the frontend's existing merge
//! (`+ list_class_feature_feat_bridge_descriptions`, `CharacterSheet.tsx`)
//! reaches the player through the SAME render path, no second UI surface.
//!
//! # Kept deliberately narrow -- one grant, nothing else, stated over our own schema
//!
//! SD-35 `AT-35-E6-003` cycle 4 rewrote this module's population rule. It used
//! to read the corpus record's verbatim token array and match the ingest format's
//! own automatic-feat-grant token shape, refusing a record that carried a second
//! grant entry, any other engine-effect token, or a placeholder target.
//! `decisions.md §11` rules that array off the live side entirely -- and the
//! relation it was being read for **is already in the converted package, in the
//! other direction**: the granted feat's own rule carries a `granted_by` entry
//! naming the class-feature rule that hands it out. Inverting that index gives
//! this module its population with no token read, no matcher, and no name
//! lookup ([`feat_grants_by_class_feature`]).
//!
//! The three refusals survive, restated over the schema:
//!
//! 1. **Exactly one granted feat.** A class-feature rule that grants two or
//!    more feats cannot be honestly described by any one feat's text -- refused,
//!    not partially served. (Was: a second grant token.)
//! 2. **No other effect of its own.** A rule with a non-empty `grants` list or
//!    an `offers` choice does more than hand over the feat -- a player-chosen
//!    weapon, a uses-per-day counter -- and serving the bare feat's text would
//!    silently drop it. Refused. (Was: a `CHOOSE`/`BONUS`/`DEFINE` token
//!    alongside the grant.)
//! 3. **The granted rule must state descriptive prose of its own.** A feat with
//!    nothing to lend lends nothing. (Was: the feat catalog's exact-name lookup
//!    answering `None`.)
//!
//! `granted_feat` is the granted rule's own `label` -- the feat's exact name,
//! read off the record rather than parsed out of a token value.
//!
//! # Never double-serves a record `class_feature_descriptions.rs` already covers
//!
//! The two populations are disjoint **by construction**, and since cycle 4 they
//! are disjoint on one predicate asked in one place: this module admits a class
//! feature only when `catalog_description` answers `None` for its converted
//! rule, and `class_feature_descriptions.rs` admits it only when the same call
//! answers `Some`. Before cycle 4 the split was "has a real local description"
//! here versus "has one" there -- the same intent, but read off the ingest
//! format and evaluated twice.
//!
//! # PI screening and the join -- same trust boundaries as `class_feature_descriptions.rs`
//!
//! Reads only the already-PI-screened `cache_gen::class_feature` cache
//! (`§52.3`/`§53.5`, discharged upstream, re-runs no check of its own) for the
//! four identity fields, and the converted package for the words. The leak guard
//! is gone for the same reason it is gone there: the converted package carries
//! no token to leak, proven package-wide by `sheet_rule_convert -- --check` and
//! `workflow-instruction.md §6`'s `data/sheet_rules/` grep rather than per
//! record. The `(class_slug, feature_slug)` join is the identical algorithm
//! `class_feature_descriptions.rs` uses (`slug`, reproduced locally here too) --
//! this module names the CLASS FEATURE's own identity in the served DTO, never
//! the matched feat's, so `classFeaturesModel.ts`'s existing
//! `matchesCorpusFeature` join needs no changes to pick these records up.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde_json::Value;

use codex::rules_core::corpus_loader::live_sheet_rules;
use codex::rules_core::sheet_rule::{Granter, RuleId, SheetRulePackage};
use codex::rules_core::sheet_rule_catalog::catalog_description;

use crate::authoring_workbench::codex_repo_root;
use crate::class_feature_descriptions::{converted_id, ClassFeatureDescriptionDto};

/// Reproduced from `class_feature_descriptions.rs::slug` -- see this
/// module's own doc comment for why a local copy, not a shared import.
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

/// Reproduced from `class_feature_descriptions.rs::is_real_description_value`.
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

/// `class_feature` rule id -> the `feat` rules it hands out, inverted out of
/// every feat rule's own `granted_by` list.
///
/// The converted package already carries a granter index internally, keyed the
/// other way and used by the held-rule fixpoint; it is not part of the public
/// schema surface, so this module builds the one relation it needs -- one pass
/// over the package's feat rules -- rather than widening `SheetRulePackage`'s
/// API for one consumer. The `when` condition on each grant is deliberately NOT
/// filtered: a level-gated or variable-gated grant is still a grant, and the
/// token shape this replaced accepted exactly those too (a trailing gate
/// qualifier was stripped and the record served).
fn feat_grants_by_class_feature(package: &SheetRulePackage) -> BTreeMap<RuleId, Vec<RuleId>> {
    let mut out: BTreeMap<RuleId, Vec<RuleId>> = BTreeMap::new();
    for feat in package.rules_of_kind("feat") {
        for grant in &feat.granted_by {
            let Granter::Rule(granter) = &grant.by else { continue };
            if !granter.contains(":class_feature:") {
                continue;
            }
            let entry = out.entry(granter.clone()).or_default();
            if !entry.contains(&feat.id) {
                entry.push(feat.id.clone());
            }
        }
    }
    out
}

/// The single feat this class-feature rule's whole content grants, or `None`
/// when one of the module doc comment's three refusals applies.
fn sole_granted_feat<'a>(
    package: &'a SheetRulePackage,
    index: &BTreeMap<RuleId, Vec<RuleId>>,
    class_feature_id: &str,
) -> Option<&'a codex::rules_core::sheet_rule::SheetRule> {
    let granted = index.get(class_feature_id)?;
    // Refusal 1: more than one granted feat is more than one feat's text.
    let [only] = granted.as_slice() else { return None };
    package.rule(only)
}

/// Reads every `class_feature` cache record under `<repo_root>/data/corpus/
/// */class_feature/**/*.json` for its identity, and the converted package for
/// everything else: a record survives only when its own rule states no
/// descriptive prose, carries no other effect, and grants exactly one feat that
/// does state prose (module doc comment's three refusals).
fn load_class_feature_feat_bridge_descriptions(repo_root: &Path) -> Vec<ClassFeatureDescriptionDto> {
    let corpus_root = repo_root.join("data/corpus");
    let mut out = Vec::new();
    let Some(package) = live_sheet_rules() else { return out };
    let index = feat_grants_by_class_feature(package);
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

            let id = converted_id(&book, key);
            let Some(rule) = package.rule(&id) else { continue };

            // Disjoint from `class_feature_descriptions.rs`, on the one
            // predicate both modules now ask: a rule that states its own prose
            // is that module's population, not this one's.
            if catalog_description(package, rule).is_some() {
                continue;
            }
            // Refusal 2: a rule that does anything else of its own is not
            // "sole content is one feat grant".
            if !rule.grants.is_empty() || rule.offers.is_some() {
                continue;
            }
            let Some(feat) = sole_granted_feat(package, &index, &id) else { continue };
            // Refusal 3: a feat with no words of its own has none to lend.
            let Some(description) = catalog_description(package, feat) else { continue };
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
                // T4-L9 (`decisions.md §13`): this module's `class_slug` is a
                // synthetic pool-group name, never a real class token (module
                // doc comment), so the class-held join can never match these
                // records. The granted rule's own `label` is the feat's exact
                // name -- carried here so the frontend can gate reachability on
                // the character holding THAT feat instead (see
                // `ClassFeatureDescriptionDto::granted_feat`'s own doc comment).
                granted_feat: Some(feat.label.clone()),
            });
        }
    }
    out
}

/// Built once, cached for the process lifetime -- mirrors
/// `class_feature_descriptions::class_feature_descriptions()`'s own
/// caching shape.
fn class_feature_feat_bridge_descriptions() -> &'static Vec<ClassFeatureDescriptionDto> {
    static TABLE: OnceLock<Vec<ClassFeatureDescriptionDto>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let repo_root = codex_repo_root()
            .expect("codex repo root must resolve for class_feature feat-bridge loading");
        load_class_feature_feat_bridge_descriptions(&repo_root)
    })
}

#[tauri::command]
pub fn list_class_feature_feat_bridge_descriptions() -> Vec<ClassFeatureDescriptionDto> {
    class_feature_feat_bridge_descriptions().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        codex_repo_root().expect("repo root resolves under `cargo test`")
    }

    #[test]
    fn slug_matches_v06_work_inventorys_own_algorithm() {
        assert_eq!(slug("Golden Legionnaire"), "golden_legionnaire");
        assert_eq!(slug("Swift Aid"), "swift_aid");
    }

    // ---------------------------------------------------------------
    // The three refusals, each proven over the LIVE converted package
    // (`decisions.md §4`: a per-kind gate over the real corpus, never a
    // fixture with a hand-derived expected value).
    // ---------------------------------------------------------------

    /// Refusal 1 is live, not vacuous: the inverted index really does contain
    /// class features that grant more than one feat, so the `[only]` arm of
    /// `sole_granted_feat` is genuinely reached in both directions.
    #[test]
    fn the_multi_grant_refusal_is_reached_by_real_records() {
        let package = live_sheet_rules().expect("data/sheet_rules/ must be present");
        let index = feat_grants_by_class_feature(package);
        let one = index.values().filter(|v| v.len() == 1).count();
        let many = index.values().filter(|v| v.len() > 1).count();
        println!("FEAT_GRANT_INDEX class_features={} sole={one} multi={many}", index.len());
        assert!(one > 0, "no class feature grants exactly one feat -- the index is empty or wrong");
        assert!(many > 0, "refusal 1 is never reached -- the multi-grant arm is vacuous");
        for (cf, feats) in &index {
            if feats.len() > 1 {
                assert!(
                    sole_granted_feat(package, &index, cf).is_none(),
                    "{cf}: a multi-grant class feature must be refused"
                );
            }
        }
    }

    /// Refusal 2 is live: among the class features that grant exactly one feat
    /// and state no prose of their own, some carry an effect or a choice of
    /// their own and are refused, and some do not and are served.
    #[test]
    fn the_other_effect_refusal_is_reached_by_real_records() {
        let package = live_sheet_rules().expect("data/sheet_rules/ must be present");
        let index = feat_grants_by_class_feature(package);
        let mut effectful = 0usize;
        let mut clean = 0usize;
        for (cf, feats) in &index {
            if feats.len() != 1 {
                continue;
            }
            let Some(rule) = package.rule(cf) else { continue };
            if catalog_description(package, rule).is_some() {
                continue;
            }
            if !rule.grants.is_empty() || rule.offers.is_some() {
                effectful += 1;
            } else {
                clean += 1;
            }
        }
        println!("SOLE_GRANT_NO_PROSE clean={clean} effectful={effectful}");
        assert!(clean > 0, "nothing survives refusal 2 -- the module would serve nothing");
        assert!(effectful > 0, "refusal 2 is never reached -- the gate is vacuous");
    }

    /// Refusal 3 is live: some granted feats state descriptive prose and some
    /// state none, so `catalog_description`'s `Some` and `None` arms are both
    /// reached on the feat side too.
    #[test]
    fn the_granted_feat_must_state_prose_and_the_refusal_is_reached() {
        let package = live_sheet_rules().expect("data/sheet_rules/ must be present");
        let mut with_prose = 0usize;
        let mut without_prose = 0usize;
        for feat in package.rules_of_kind("feat") {
            match catalog_description(package, feat) {
                Some(text) if is_real_description_value(&text) => with_prose += 1,
                _ => without_prose += 1,
            }
        }
        println!("FEAT_RULES with_prose={with_prose} without_prose={without_prose}");
        assert!(with_prose > 0, "no converted feat states any descriptive prose");
        assert!(without_prose > 0, "the refuse arm is never reached -- the gate is vacuous");
    }

    /// The real corpus loads real bridged records -- proven against the
    /// live `data/corpus/` checkout, not a fixture.
    #[test]
    fn loads_real_bridged_descriptions_from_the_live_corpus() {
        let descriptions = load_class_feature_feat_bridge_descriptions(&repo_root());
        assert!(
            descriptions.len() > 100,
            "expected hundreds of real bridged class_feature records, got {}",
            descriptions.len()
        );
        let swift_aid = descriptions
            .iter()
            .find(|d| d.book == "adventurers_guide" && d.key == "Golden Legionnaire ~ Swift Aid")
            .expect("Golden Legionnaire ~ Swift Aid must be bridged");
        assert_eq!(swift_aid.class_slug, "golden_legionnaire");
        assert_eq!(swift_aid.feature_slug, "swift_aid");
        // Two families, in the sheet's own order: the feat's `Desc` line and
        // its `Benefit` line. The old by-name lookup served only the first --
        // the ingest format's own description token -- and the benefit sentence, which
        // is the half a player actually needs, reached nobody.
        assert_eq!(
            swift_aid.description,
            "With a quick but harmless swipe, you can aid an ally's assault.\nAs a swift \
             action, you can attempt the aid another action, granting your ally either a +1 \
             bonus on his next attack roll or a +1 bonus to his AC."
        );
        assert_eq!(
            swift_aid.granted_feat.as_deref(),
            Some("Swift Aid"),
            "T4-L9: the bridged record must carry the exact feat name it grants, so the \
             frontend can gate reachability on the character holding THAT feat rather than \
             the synthetic pool-group class_slug"
        );
    }

    /// T4-L9 (`decisions.md §13`) -- closed by class, not by instance: every
    /// record this module serves must carry `granted_feat`, not merely the
    /// sampled `Golden Legionnaire ~ Swift Aid` case above. A record reaching
    /// this DTO with `granted_feat: None` would be invisible to the feat-held
    /// reachability gate (`unmatchedClassFeatureDescriptions` in
    /// `classFeaturesModel.ts`) the same way the class-held gate already misses
    /// it -- this proves the fix covers the whole population, corpus-wide.
    #[test]
    fn every_bridged_record_corpus_wide_carries_its_granted_feat() {
        let descriptions = load_class_feature_feat_bridge_descriptions(&repo_root());
        assert!(!descriptions.is_empty());
        let missing: Vec<&str> = descriptions
            .iter()
            .filter(|d| d.granted_feat.is_none())
            .map(|d| d.key.as_str())
            .collect();
        assert!(
            missing.is_empty(),
            "{} of {} bridged records carry no granted_feat (e.g. {:?}) -- these would be \
             unreachable under the new feat-held gate exactly as they were under the old \
             class-held one",
            missing.len(),
            descriptions.len(),
            missing.iter().take(4).collect::<Vec<_>>()
        );
        for d in &descriptions {
            assert!(
                !d.granted_feat.as_deref().unwrap_or_default().trim().is_empty(),
                "{:?}: granted_feat must never be an empty string -- that would silently \
                 gate every character out of a record that genuinely reaches this DTO",
                d.key
            );
        }
    }

    /// Reuse, not reinvention: the served text is the granted feat's OWN
    /// converted words, byte for byte -- this module re-renders nothing. Asked
    /// of every served record rather than a sample: the DTO's `granted_feat` is
    /// the granted rule's `label`, so the text can be looked back up and
    /// compared.
    #[test]
    fn every_bridged_description_is_the_granted_rules_own_words() {
        let package = live_sheet_rules().expect("data/sheet_rules/ must be present");
        let index = feat_grants_by_class_feature(package);
        let descriptions = load_class_feature_feat_bridge_descriptions(&repo_root());
        assert!(!descriptions.is_empty());
        let mut compared = 0usize;
        for record in &descriptions {
            let id = converted_id(&record.book, &record.key);
            let feat = sole_granted_feat(package, &index, &id)
                .unwrap_or_else(|| panic!("{id}: served but no sole granted feat at re-derivation"));
            assert_eq!(feat.label, record.granted_feat.clone().unwrap_or_default());
            assert_eq!(
                catalog_description(package, feat).as_deref(),
                Some(record.description.as_str()),
                "{id}: served text is not the granted rule's own words"
            );
            compared += 1;
        }
        assert_eq!(compared, descriptions.len());
    }

    /// The population ratchet. `AT-35-E6-003` cycle 4 measured this module's
    /// served population on both sides of the token-read removal with a
    /// temporary census: **612 before, 709 after**, 4 of the 612 lost. Two of
    /// the four have no converted rule at all; the other two
    /// (`advanced_race_guide` / `adventurers_guide` `~ Elemental Fist`) grant
    /// `advanced_players_guide:feat:elemental_fist`, a selector record that
    /// states no words of its own -- refusal 3, correctly applied, where the
    /// old by-name lookup borrowed a differently-identified sibling's text.
    /// A floor, not an identity.
    #[test]
    fn the_served_bridge_population_never_falls_below_its_recorded_floor() {
        let descriptions = load_class_feature_feat_bridge_descriptions(&repo_root());
        assert!(
            descriptions.len() >= 700,
            "the bridged population fell below its recorded floor: {}",
            descriptions.len()
        );
        assert!(
            descriptions
                .iter()
                .any(|d| d.book == "adventurers_guide" && d.key == "Superior Discernment ~ Sharp Senses"),
            "a real match the wave-28 census's own type_facet substring filter missed must \
             still be served"
        );
        for d in &descriptions {
            assert!(!d.description.trim().is_empty());
            // `%1`, `%2`, ... -- never a bare `%`, which real prose uses
            // ("increased by half (+50%)"). See
            // `class_feature_descriptions`'s own note on why.
            let bytes = d.description.as_bytes();
            assert!(
                !bytes.iter().enumerate().any(|(i, b)| *b == b'%'
                    && bytes.get(i + 1).is_some_and(u8::is_ascii_digit)),
                "{:?}: {:?}",
                d.key,
                d.description
            );
        }
    }

    /// A record with a REAL local description is never double-served here
    /// -- that population belongs to `class_feature_descriptions.rs`
    /// alone.
    #[test]
    fn never_serves_a_record_that_already_has_a_real_local_description() {
        let bridged = load_class_feature_feat_bridge_descriptions(&repo_root());
        let bridged_keys: std::collections::BTreeSet<(&str, &str)> =
            bridged.iter().map(|d| (d.book.as_str(), d.key.as_str())).collect();

        let real = crate::class_feature_descriptions::list_class_feature_descriptions();
        assert!(!real.is_empty());
        for record in &real {
            assert!(
                !bridged_keys.contains(&(record.book.as_str(), record.key.as_str())),
                "{:?} ({}) is served by BOTH modules -- population is not disjoint",
                record.key,
                record.book
            );
        }
    }

    #[test]
    fn list_class_feature_feat_bridge_descriptions_returns_the_cached_table() {
        let a = list_class_feature_feat_bridge_descriptions();
        let b = list_class_feature_feat_bridge_descriptions();
        assert_eq!(a.len(), b.len());
        assert!(!a.is_empty());
    }
}

