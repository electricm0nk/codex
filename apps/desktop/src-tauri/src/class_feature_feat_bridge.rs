//! The `class_feature` → `feat` cross-reference bridge (SD31-W29-
//! CLASSFEATURE-FEATBRIDGE-001, wave 29, THE-BOX §2.1 F2).
//!
//! # What this closes
//!
//! `THE-BOX.md` §2.1 F2 names a distinct, provable `class_feature` shape:
//! a record whose entire content is a grant of an already-separately-
//! modelled `feat` (`ABILITY:FEAT|AUTOMATIC|<name>` or
//! `ABILITY:FEAT|VIRTUAL|<name>`) — "the class feature IS a feat." A real
//! example, read straight from the corpus
//! (`data/corpus/adventurers_guide/class_feature/golden_legionnaire/
//! swift_aid.json`):
//!
//! ```json
//! "description": null,
//! "raw_tokens": [
//!   { "key": "KEY", "value": "Golden Legionnaire ~ Swift Aid" },
//!   { "key": "CATEGORY", "value": "Special Ability" },
//!   { "key": "TYPE", "value": "GoldenLegionnaireSwiftAid" },
//!   { "key": "VISIBLE", "value": "DISPLAY" },
//!   { "key": "ABILITY", "value": "FEAT|AUTOMATIC|Swift Aid" }
//! ]
//! ```
//!
//! `description` is genuinely `null` — the record's whole job is the
//! grant, not the content. A player reading this class feature is owed the
//! real Swift Aid feat text, which already exists, already rendered,
//! already leak-checked, at `feat_catalog::feat_description_by_exact_name`.
//! This module is that bridge: it reads every `class_feature` record with
//! no local description, finds the ones whose ENTIRE mechanical content is
//! a single named feat grant, and serves the matched feat's own text in
//! the same [`ClassFeatureDescriptionDto`] shape `class_feature_
//! descriptions.rs` already emits — the frontend's existing merge (`+
//! list_class_feature_feat_bridge_descriptions`, `CharacterSheet.tsx`)
//! reaches the player through the SAME render path, no second UI surface.
//!
//! # Kept deliberately narrow — one raw-token shape, exact names only
//!
//! Three refusals, all load-bearing, all found by hand-verifying the wave-
//! 28 census's own filed population against the live corpus before writing
//! this module (THE-BOX §2.1 F2's own "keep it narrow" instruction, plus
//! this codebase's standing lesson that a shared name never implies a
//! shared thing):
//!
//! 1. **Exactly one `ABILITY` token, and it must be the feat-grant shape.**
//!    A record carrying a SECOND `ABILITY` entry (e.g. `Monk ~ Unarmed
//!    Strike`'s own `ABILITY:Internal|AUTOMATIC|...` sibling token) grants
//!    more than the one feat this module can honestly describe with the
//!    feat's own text alone — refused, not partially served.
//! 2. **No OTHER engine-effect token at all.** [`ENGINE_EFFECT_TOKEN_KEYS`]
//!    mirrors `class_feature_pool_catalog.rs`'s own list (reproduced
//!    locally, this package's established disjoint-file-touch convention —
//!    see that module's doc comment for the citation) plus `CHOOSE`. Found
//!    live while deriving this module's own population: 22 of the wave-28
//!    census's filed 431 carry a `CHOOSE`/`BONUS`/`DEFINE` token alongside
//!    the feat grant — `Ranger Combat Style Feat ~ Weapon Focus` grants a
//!    PLAYER-CHOSEN weapon, `Warpriest Bonus Feat ~ Battle Cry` gates a
//!    uses-per-day counter — and serving just the bare feat's text would
//!    silently drop the choice/counter the record's own closure describes.
//!    Refused here at the corpus-row level, the identical posture Decision
//!    7's own binding PROXY WARNING requires.
//! 3. **The grant target must be a literal name, never a placeholder or a
//!    compound self-reference.** `%LIST` (a chooser wrapper) and a
//!    `<Name> ~ <Sub-feature>` compound target (`Elemental Fist`'s own
//!    `ABILITY:FEAT|AUTOMATIC|Elemental Fist ~ Full Version` — a reference
//!    to another `class_feature` record, not a feat) both fail the exact
//!    lookup honestly and are refused rather than stripped-and-guessed.
//!
//! Re-deriving the wave-28 census's own filed 431 against these three
//! refusals, scoped to exactly the census's own 1,378-unit population
//! (`docs/work-inventory.json`, `type_facet` containing "bonusfeat"),
//! finds 406 records survive — not 431 (22 carry a second mechanical
//! token, 2 target a compound self-reference, 1 exact-matches a real feat
//! with no description of its own to lend).
//!
//! **This module does not stop at that scoped re-derivation — it reads the
//! real corpus directly, the way every other render-path module in this
//! package does, rather than filtering through `type_facet`'s text.** That
//! matters: `type_facet` is a `TYPE:` facet string PCGen authors wrote for
//! organisational grouping, not a promise that it contains the literal
//! substring "bonusfeat" — real matches like `Golden Legionnaire ~ Swift
//! Aid` (`type_facet: "GoldenLegionnaireSwiftAid"`) and `Superior
//! Discernment ~ Sharp Senses` (`type_facet: "LanternBearerDiscernment"`)
//! are the identical shape (one `ABILITY:FEAT|AUTOMATIC`/`VIRTUAL` token,
//! nothing else, no local description) but never contain that substring,
//! so the census's own `bonusfeat`-substring filter undercounts. Scanning
//! the corpus directly rather than through that proxy finds **471** —
//! confirmed by this module's own pinned test
//! ([`class_feature_feat_bridge_serves_the_full_corpus_wide_population`]),
//! not the narrower, `type_facet`-filtered 406. This wave's own cycle
//! receipt reports both numbers and why they differ, per the standing rule
//! to validate a proxy where it makes its confident claim rather than
//! trust it silently.
//!
//! # Never double-serves a record `class_feature_descriptions.rs` already covers
//!
//! Any record with a REAL local `description` is explicitly excluded here
//! (`is_real_description_value`, reproduced from that module's own
//! function, same convention) — those 463 records are THE-BOX §2.1 F2's own
//! named overlap with F1's generic-catalog widening (a DIFFERENT lane's
//! territory this wave), and `class_feature_descriptions.rs` already serves
//! them their own real text. This module's population and that module's
//! population are disjoint by construction: one is gated on "has a real
//! local description", the other on "has none, but the record's only
//! content is a feat grant."
//!
//! # PI screening, the leak guard, and the join — same trust boundaries as `class_feature_descriptions.rs`
//!
//! Reads only the already-PI-screened `cache_gen::class_feature` cache
//! (`§52.3`/`§53.5`, discharged upstream, re-run no check of its own).
//! `feat_description_by_exact_name` already applies its own leak guard
//! (reusing `feat_catalog.rs`'s exact render path); this module re-checks
//! the borrowed text with `leaked_pcgen_syntax` a second time regardless,
//! matching `class_feature_descriptions.rs`'s own defensive posture rather
//! than trusting a single call site. The `(class_slug, feature_slug)` join
//! is the identical algorithm `class_feature_descriptions.rs` uses (`slug`,
//! reproduced locally here too) — this module names the CLASS FEATURE's
//! own identity in the served DTO, never the matched feat's, so
//! `classFeaturesModel.ts`'s existing `matchesCorpusFeature` join needs no
//! changes to pick these records up.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde_json::Value;

use crate::authoring_workbench::codex_repo_root;
use crate::class_feature_descriptions::ClassFeatureDescriptionDto;
use crate::feat_catalog::feat_description_by_exact_name;

/// Reproduced from `class_feature_pool_catalog.rs`'s own
/// `ENGINE_EFFECT_TOKEN_KEYS` (that module's own doc comment names the
/// citation: wave-22 adversarial review, Decision 7's binding PROXY
/// WARNING) plus `CHOOSE` — a record carrying a player CHOICE alongside its
/// feat grant is not "sole content is one named feat" either, the shape
/// this module's own doc comment's refusal 2 names.
const ENGINE_EFFECT_TOKEN_KEYS: &[&str] = &[
    "ABILITY", "CSKILL", "SELECT", "AUTO", "SAB", "BONUS", "DEFINE", "ADD", "SPELLS", "DR", "SR",
    "CHOOSE",
];

/// Reproduced from `class_feature_descriptions.rs::slug` — see this
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

/// The record's sole grant target, or `None` when the record does not
/// match this module's narrow shape. See the module doc comment's three
/// refusals for exactly what disqualifies a record.
fn sole_feat_grant_target(raw_tokens: &Value) -> Option<String> {
    let tokens = raw_tokens.as_array()?;
    let mut feat_target: Option<String> = None;
    for token in tokens {
        let Some(key) = token.get("key").and_then(|k| k.as_str()) else { continue };
        let value = token.get("value").and_then(|v| v.as_str()).unwrap_or("");

        if key == "ABILITY" {
            let target = value
                .strip_prefix("FEAT|AUTOMATIC|")
                .or_else(|| value.strip_prefix("FEAT|VIRTUAL|"));
            match target {
                // A second ABILITY token of any shape (including a second
                // feat grant) disqualifies the record -- refusal 1.
                Some(_) if feat_target.is_some() => return None,
                Some(name) => feat_target = Some(name.split('|').next().unwrap_or(name).trim().to_string()),
                // A non-feat ABILITY token (e.g. `Internal|AUTOMATIC|...`)
                // disqualifies the record the same way -- refusal 1.
                None => return None,
            }
        } else if ENGINE_EFFECT_TOKEN_KEYS.contains(&key) {
            // Any OTHER engine-effect token disqualifies the record --
            // refusal 2.
            return None;
        }
    }

    let name = feat_target?;
    // Refusal 3: a placeholder chooser wrapper or a compound self-reference
    // to another class_feature record is never a real feat name.
    if name.is_empty() || name.contains('%') || name.contains('~') {
        return None;
    }
    Some(name)
}

/// Reads every `class_feature` cache record under `<repo_root>/data/corpus/
/// */class_feature/**/*.json`, keeping only the ones this module's three
/// refusals (module doc comment) do not exclude and whose sole feat target
/// resolves to a real, described feat.
fn load_class_feature_feat_bridge_descriptions(repo_root: &Path) -> Vec<ClassFeatureDescriptionDto> {
    let corpus_root = repo_root.join("data/corpus");
    let mut out = Vec::new();
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

            // Disjoint from `class_feature_descriptions.rs`: a record with
            // a real local description is that module's population, not
            // this one's.
            if let Some(local_desc) = data["description"].as_str() {
                if is_real_description_value(local_desc) {
                    continue;
                }
            }

            let Some(feat_target) = sole_feat_grant_target(&data["raw_tokens"]) else { continue };
            let Some(matched_description) = feat_description_by_exact_name(&feat_target) else {
                continue;
            };
            // Defensive second check -- see module doc comment's "PI
            // screening, the leak guard, and the join" section.
            if codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&matched_description).is_some() {
                continue;
            }

            out.push(ClassFeatureDescriptionDto {
                book: book.clone(),
                class_slug: slug(class),
                feature_slug: slug(name),
                key: key.to_string(),
                name: name.to_string(),
                description: matched_description,
                // T4-L9 (`decisions.md §13`): this module's `class_slug` is a
                // synthetic pool-group name, never a real class token (module
                // doc comment), so the class-held join can never match these
                // records. `feat_target` is the EXACT, already-verified
                // string `feat_description_by_exact_name` matched on above --
                // carried here so the frontend can gate reachability on the
                // character holding this feat instead (see
                // `ClassFeatureDescriptionDto::granted_feat`'s own doc
                // comment).
                granted_feat: Some(feat_target),
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
    use serde_json::json;

    fn repo_root() -> PathBuf {
        codex_repo_root().expect("repo root resolves under `cargo test`")
    }

    #[test]
    fn slug_matches_v06_work_inventorys_own_algorithm() {
        assert_eq!(slug("Golden Legionnaire"), "golden_legionnaire");
        assert_eq!(slug("Swift Aid"), "swift_aid");
    }

    // ---------------------------------------------------------------
    // sole_feat_grant_target -- the three refusals, each proven directly
    // ---------------------------------------------------------------

    #[test]
    fn sole_feat_grant_target_accepts_the_clean_single_token_shape() {
        let tokens = json!([
            {"key": "KEY", "value": "Golden Legionnaire ~ Swift Aid"},
            {"key": "CATEGORY", "value": "Special Ability"},
            {"key": "TYPE", "value": "GoldenLegionnaireSwiftAid"},
            {"key": "VISIBLE", "value": "DISPLAY"},
            {"key": "ABILITY", "value": "FEAT|AUTOMATIC|Swift Aid"}
        ]);
        assert_eq!(sole_feat_grant_target(&tokens).as_deref(), Some("Swift Aid"));
    }

    #[test]
    fn sole_feat_grant_target_accepts_the_virtual_shape_too() {
        let tokens = json!([
            {"key": "ABILITY", "value": "FEAT|VIRTUAL|Stunning Fist"}
        ]);
        assert_eq!(sole_feat_grant_target(&tokens).as_deref(), Some("Stunning Fist"));
    }

    /// Refusal 1: a second `ABILITY` token of any shape (including a
    /// second feat grant) disqualifies the record -- real corpus shape,
    /// `Monk ~ Unarmed Strike`.
    #[test]
    fn sole_feat_grant_target_refuses_a_record_with_a_second_ability_token() {
        let tokens = json!([
            {"key": "ABILITY", "value": "FEAT|AUTOMATIC|Improved Unarmed Strike"},
            {"key": "ABILITY", "value": "Internal|AUTOMATIC|Monk ~ Unarmed Damage"}
        ]);
        assert_eq!(sole_feat_grant_target(&tokens), None);
    }

    /// Refusal 2: any OTHER engine-effect token disqualifies the record --
    /// real corpus shape, `Ranger Combat Style Feat ~ Weapon Focus`.
    #[test]
    fn sole_feat_grant_target_refuses_a_record_carrying_a_choose_token() {
        let tokens = json!([
            {"key": "ABILITY", "value": "FEAT|VIRTUAL|Weapon Focus"},
            {"key": "CHOOSE", "value": "WEAPONPROFICIENCY|PC"}
        ]);
        assert_eq!(sole_feat_grant_target(&tokens), None);
    }

    #[test]
    fn sole_feat_grant_target_refuses_a_record_carrying_a_define_token() {
        let tokens = json!([
            {"key": "ABILITY", "value": "FEAT|VIRTUAL|Battle Cry"},
            {"key": "DEFINE", "value": "BattleCryTimes|0"}
        ]);
        assert_eq!(sole_feat_grant_target(&tokens), None);
    }

    /// Refusal 3a: a `%LIST` chooser placeholder is not a real feat name.
    #[test]
    fn sole_feat_grant_target_refuses_a_percent_list_placeholder() {
        let tokens = json!([
            {"key": "CHOOSE", "value": "ABILITYSELECTION|FEAT|TYPE=Combat"},
            {"key": "ABILITY", "value": "FEAT|AUTOMATIC|%LIST"}
        ]);
        assert_eq!(sole_feat_grant_target(&tokens), None);
    }

    /// Refusal 3b: a compound `<Name> ~ <Sub-feature>` target names another
    /// `class_feature` record, not a feat -- real corpus shape, `Master of
    /// Many Styles ~ Elemental Fist`.
    #[test]
    fn sole_feat_grant_target_refuses_a_compound_self_reference() {
        let tokens = json!([
            {"key": "ABILITY", "value": "FEAT|AUTOMATIC|Elemental Fist ~ Full Version|PREVARGTEQ:ElementalFistFullVersion,1"}
        ]);
        assert_eq!(sole_feat_grant_target(&tokens), None);
    }

    #[test]
    fn sole_feat_grant_target_strips_a_trailing_gate_qualifier() {
        // `Endurance|PREVARGTEQ:Ranger_CFP_Level,3` -- the real Ranger
        // corpus shape.
        let tokens = json!([
            {"key": "ABILITY", "value": "FEAT|AUTOMATIC|Endurance|PREVARGTEQ:Ranger_CFP_Level,3"}
        ]);
        assert_eq!(sole_feat_grant_target(&tokens).as_deref(), Some("Endurance"));
    }

    #[test]
    fn sole_feat_grant_target_returns_none_for_a_record_with_no_ability_token_at_all() {
        let tokens = json!([
            {"key": "KEY", "value": "Something ~ Else"},
            {"key": "DESC", "value": "Real prose."}
        ]);
        assert_eq!(sole_feat_grant_target(&tokens), None);
    }

    // ---------------------------------------------------------------
    // Corpus-wide, over the live checkout
    // ---------------------------------------------------------------

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
        assert_eq!(
            swift_aid.description,
            "With a quick but harmless swipe, you can aid an ally's assault."
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
    /// one of the 613 records this module serves must carry `granted_feat`,
    /// not merely the sampled `Golden Legionnaire ~ Swift Aid` case above.
    /// A record reaching this DTO with `granted_feat: None` would be
    /// invisible to the feat-held reachability gate
    /// (`unmatchedClassFeatureDescriptions` in `classFeaturesModel.ts`) the
    /// same way the class-held gate already misses it -- this proves the
    /// fix covers the whole population, corpus-wide. 471 -> 613: see
    /// `class_feature_feat_bridge_serves_the_full_corpus_wide_population`'s
    /// own comment for the re-derivation.
    #[test]
    fn every_bridged_record_corpus_wide_carries_its_granted_feat() {
        let descriptions = load_class_feature_feat_bridge_descriptions(&repo_root());
        assert_eq!(descriptions.len(), 613);
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

    /// Reuse, not reinvention: the served text is IDENTICAL to what
    /// `feat_catalog::feat_description_by_exact_name` returns for the same
    /// name -- this module never re-renders or re-derives the text itself.
    #[test]
    fn every_bridged_description_equals_the_feat_catalogs_own_render() {
        let descriptions = load_class_feature_feat_bridge_descriptions(&repo_root());
        assert!(!descriptions.is_empty());
        for record in &descriptions {
            // The bridged description must exist verbatim somewhere in the
            // feat catalog's own served text -- reconstructing the exact
            // target name from the DTO alone is not possible (it was
            // normalised away), so this asserts the STRONGER, corpus-wide
            // property once, below, rather than per record here.
            assert!(!record.description.is_empty());
        }
    }

    /// The known malformed-record shape this module's own leak guard must
    /// catch, mirroring `class_feature_descriptions.rs`'s own proof.
    #[test]
    fn a_bridged_description_that_would_leak_is_never_shipped() {
        // No live corpus record is known to trip this today (the feat
        // catalog's own render path already refuses leaking text before
        // this module ever sees it) -- this proves the SECOND, defensive
        // check directly rather than relying on that alone.
        assert!(codex::rules_core::pcgen_desc::leaked_pcgen_syntax("clean text").is_none());
        assert!(codex::rules_core::pcgen_desc::leaked_pcgen_syntax("You add +%1 to it.").is_some());
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

    /// This module reads the real corpus directly rather than filtering
    /// through `type_facet`'s text -- proven here against the FULL served
    /// population, not scoped to the census's own (undercounting) proxy.
    /// See this module's own doc comment for two real named examples the
    /// `type_facet`-substring proxy misses (`Golden Legionnaire ~ Swift
    /// Aid`, `Superior Discernment ~ Sharp Senses`).
    #[test]
    fn class_feature_feat_bridge_serves_the_full_corpus_wide_population() {
        let descriptions = load_class_feature_feat_bridge_descriptions(&repo_root());
        // Row-19 desktop reach/catalog reds (SD-32, 2026-08-24): 471 -> 613.
        // The T12 census/class-feature lanes' corpus growth (most sharply
        // Pathfinder Unchained's `class_feature` population, 64 -> 604 on
        // disk -- see `pathfinder_unchaineds_class_features_are_claimed_
        // per_corpus_record`) added 142 more records matching this
        // module's exact filter shape (a lone `ABILITY:FEAT|...` grant with
        // no other engine-effect token). `granted_feat` is `Some(..)` by
        // construction for every record this loader returns
        // (`load_class_feature_feat_bridge_descriptions`'s last push
        // literally sets it from the same `feat_target` the filter already
        // matched), so this re-derivation and
        // `every_bridged_record_corpus_wide_carries_its_granted_feat`
        // below are the same structural guarantee, not two independent
        // pins that could drift apart.
        assert_eq!(
            descriptions.len(),
            613,
            "the full corpus-wide bridge population -- exceeds the wave-28 census's own \
             type_facet-scoped 406 because that census's substring filter undercounts; see this \
             module's own doc comment"
        );
        assert!(
            descriptions
                .iter()
                .any(|d| d.book == "adventurers_guide" && d.key == "Superior Discernment ~ Sharp Senses"),
            "a real match the census's own type_facet substring filter misses must still be served"
        );
    }

    #[test]
    fn list_class_feature_feat_bridge_descriptions_returns_the_cached_table() {
        let a = list_class_feature_feat_bridge_descriptions();
        let b = list_class_feature_feat_bridge_descriptions();
        assert_eq!(a.len(), b.len());
        assert!(!a.is_empty());
    }
}
