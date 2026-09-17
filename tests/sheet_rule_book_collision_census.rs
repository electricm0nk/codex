//! SD-36 Epic E engine-P1-3 (partial mitigation): `SheetRulePackage::find(kind, slug)` resolves
//! a name shared by two non-core-rulebook books via an alphabetical book-id tie-break with no
//! per-character book context (`src/rules_core/sheet_rule.rs`'s `find()`, consumed by every
//! `held_set()` seed lookup). The FULL fix -- threading the source book through `HeldSeed`,
//! `ChosenCharacterState` (`selected_feats`/`equipment_selections`/etc. are bare compound-string
//! slugs today, corpus-wide, with no book component) and migrating the saved-character schema --
//! is a multi-cycle schema migration across `src/rules_core/character_input.rs` and every fixture
//! that builds a `ChosenCharacterState`, not a bounded fix for one review cycle; it is escalated
//! (see the epic-e receipt's "NEEDS HUMAN RULING" note) rather than attempted here.
//!
//! The fix_sketch's stated minimum bar is met instead: "treat a (kind,slug) collision with
//! differing value/bonus_type across non-core books as a data defect to SURFACE... rather than
//! silently picking one." This test is that surface. It re-derives, from the live
//! `data/sheet_rules` package, every (kind, slug) pair `held_set()` can reach through `find()`
//! (feat/trait/equipment/spell/skill/race_trait/class_feature) that collides across 2+ NON-core
//! books with a materially different `value` -- exactly the shape `find()`'s alphabetical
//! tie-break silently mis-resolves today -- and asserts the set exactly matches a reviewed,
//! checked-in list. A NEW collision (a future book landing new content that happens to share an
//! existing name) fails this test loudly, naming the pair, instead of resolving silently forever.
//!
//! Regenerate the list with:
//! ```text
//! python3 -c "
//! import json,glob,collections
//! KINDS = {'feat','trait','equipment','spell','skill','race_trait','class_feature'}
//! groups = collections.defaultdict(dict)
//! for f in glob.glob('data/sheet_rules/*/*/*.json'):
//!     d = json.load(open(f))
//!     if not d: continue
//!     r = d[0]
//!     parts = r['id'].split(':')
//!     if len(parts) != 3: continue
//!     book, kind, slug = parts
//!     if kind not in KINDS or slug == 'default': continue
//!     groups[(kind, slug)][book] = json.dumps(r.get('value'), sort_keys=True)
//! for (kind, slug), books in sorted(groups.items()):
//!     non_core = {b: v for b, v in books.items() if b != 'core_rulebook'}
//!     if len(non_core) >= 2 and len(set(non_core.values())) > 1:
//!         print(f'(\"{kind}\", \"{slug}\"),')
//! "
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/// Reviewed as of this cycle (SD-36 Epic E). Each pair is a REAL corpus name collision across
/// 2+ non-core books whose converted `value` differs materially -- confirmed examples include
/// `equipment:dogslicer` (1d4 in one book, 1d6 in two others), `class_feature:sabre_fighting`
/// (bare Text in one book, a real computed formula in another), and `feat:scholar` (a flat +2 in
/// one book, a skill-rank-scaled formula in another). Adding a new pair here is a conscious
/// acknowledgment that `find()` will keep silently picking one book for it until engine-P1-3's
/// full fix lands; it is not an invitation to grow this list casually.
const KNOWN_UNRESOLVED_BOOK_COLLISIONS: &[(&str, &str)] = &[
    ("class_feature", "bloodrager_bloodline_tracker"),
    ("class_feature", "cyphermage_cypher_lore"),
    ("class_feature", "rogue_talent_nimble_climber"),
    ("class_feature", "sabre_fighting"),
    ("class_feature", "tempest_druid_sodden_shore_sense"),
    ("class_feature", "verdant_bloodline"),
    ("equipment", "aklys"),
    ("equipment", "binding_contract"),
    ("equipment", "cat_s_eye_crown"),
    ("equipment", "celestial_shield"),
    ("equipment", "corset_of_the_vishkanya"),
    ("equipment", "dawnflower_sash"),
    ("equipment", "dogslicer"),
    ("equipment", "elysian_shield"),
    ("equipment", "halo_of_inner_calm"),
    ("equipment", "horsechopper"),
    ("equipment", "madu_leather"),
    ("equipment", "madu_steel"),
    ("equipment", "mammoth_hide"),
    ("equipment", "ring_of_rat_fangs"),
    ("equipment", "ring_of_the_sophisticate"),
    ("equipment", "slippers_of_the_triton"),
    ("equipment", "stonemist_cloak"),
    ("feat", "deepsight"),
    ("feat", "distance_thrower"),
    ("feat", "improved_stonecunning"),
    ("feat", "monastic_legacy"),
    ("feat", "perfect_style"),
    ("feat", "scholar"),
    ("feat", "sociable"),
    ("feat", "voice_of_the_sibyl"),
    ("feat", "warrior_priest"),
];

const HELD_KINDS: &[&str] = &["feat", "trait", "equipment", "spell", "skill", "race_trait", "class_feature"];

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// (kind, slug) -> book -> the record's `value` field, serialized for equality comparison.
fn collect_values() -> BTreeMap<(String, String), BTreeMap<String, String>> {
    let root = repo().join("data/sheet_rules");
    let mut groups: BTreeMap<(String, String), BTreeMap<String, String>> = BTreeMap::new();
    let mut book_dirs: Vec<PathBuf> = std::fs::read_dir(&root).expect("data/sheet_rules present").flatten().map(|e| e.path()).filter(|p| p.is_dir() && !p.file_name().is_some_and(|n| n.to_string_lossy().starts_with('_'))).collect();
    book_dirs.sort();
    for book_dir in book_dirs {
        let book = book_dir.file_name().unwrap().to_string_lossy().to_string();
        let Ok(kind_dirs) = std::fs::read_dir(&book_dir) else { continue };
        for kind_entry in kind_dirs.flatten() {
            let kind_path = kind_entry.path();
            if !kind_path.is_dir() {
                continue;
            }
            let kind = kind_path.file_name().unwrap().to_string_lossy().to_string();
            if !HELD_KINDS.contains(&kind.as_str()) {
                continue;
            }
            let Ok(files) = std::fs::read_dir(&kind_path) else { continue };
            for f in files.flatten() {
                let path = f.path();
                if path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }
                let Ok(text) = std::fs::read_to_string(&path) else { continue };
                let Ok(rules) = serde_json::from_str::<Vec<serde_json::Value>>(&text) else { continue };
                let Some(first) = rules.first() else { continue };
                let Some(id) = first.get("id").and_then(|v| v.as_str()) else { continue };
                let parts: Vec<&str> = id.splitn(3, ':').collect();
                if parts.len() != 3 {
                    continue;
                }
                let slug = parts[2].to_string();
                if slug == "default" {
                    continue;
                }
                let value_json = serde_json::to_string(&first.get("value")).unwrap_or_default();
                groups.entry((kind.clone(), slug)).or_default().insert(book.clone(), value_json);
            }
        }
    }
    groups
}

#[test]
fn book_collisions_with_a_different_converted_value_are_all_on_the_reviewed_list() {
    let groups = collect_values();
    let mut found: BTreeSet<(String, String)> = BTreeSet::new();
    for ((kind, slug), books) in &groups {
        let non_core: BTreeMap<&String, &String> = books.iter().filter(|(b, _)| b.as_str() != "core_rulebook").collect();
        if non_core.len() < 2 {
            continue;
        }
        let distinct: BTreeSet<&&String> = non_core.values().collect();
        if distinct.len() > 1 {
            found.insert((kind.clone(), slug.clone()));
        }
    }
    let known: BTreeSet<(String, String)> = KNOWN_UNRESOLVED_BOOK_COLLISIONS.iter().map(|(k, s)| (k.to_string(), s.to_string())).collect();
    let new: Vec<&(String, String)> = found.difference(&known).collect();
    let gone: Vec<&(String, String)> = known.difference(&found).collect();
    assert!(
        new.is_empty(),
        "NEW (kind,slug) collision(s) across non-core books with a differing converted value -- \
         `find()`'s alphabetical tie-break (engine-P1-3, sheet_rule.rs) will silently pick one book \
         for a real character holding this name; review, then add to KNOWN_UNRESOLVED_BOOK_COLLISIONS: {new:?}"
    );
    assert!(
        gone.is_empty(),
        "a previously-reviewed collision no longer reproduces -- remove it from \
         KNOWN_UNRESOLVED_BOOK_COLLISIONS (a book's content changed, or the converter's own fix \
         for a related finding incidentally resolved it): {gone:?}"
    );
}
