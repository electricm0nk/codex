//! Companion ability reference-pool catalog (SD-32 row 19 cycle 3).
//!
//! # The shape this closes
//!
//! `companion_chassis::COMPANION_BOOKS` (`companion_catalog.rs`'s own data
//! source) serves a companion `Ability` row attached to the creature row that
//! owns it, mirroring how `monster_ability` is served under its monster. That
//! is the right model for an ability a specific creature grants, but PCGen's
//! `*_abilities_companion.lst` files also carry a second shape row 19 cycle 2
//! named and refused to hand-list: a `" ~ "`-group-qualified record with
//! `owners: []` — no creature row of its own book claims it, because it is
//! not a creature's ability at all. It is a **shared reference-library entry**
//! a player picks from a POOL the archetype/trick/evolution system grants
//! (`Animal Trick ~ Aid`, `Aberrant Companion ~ Aberrant Sight`, `Companion
//! Archetype ~ ...`, an Ultimate Wilderness Eidolon evolution, ...) — the
//! exact "member of a referenced pool" shape `class_feature_pool_catalog.rs`
//! already built for `class_feature` (SD31-W22-POOLMEMBER-001), generalized
//! here to `companion` per `decisions.md §17` ("the generic ingest already
//! exists... stop treating every object as a snowflake") and `§27b`
//! ("EVERYTHING. No carve-outs survive... novelty of shape... is NONE of
//! them"). Built once, generically, rather than as 434 individually-named
//! exceptions (row 19 cycle 2's own refusal, `§17a`/`§1a`).
//!
//! # What counts as a pool member here
//!
//! A `companion/*.json` record with an empty `data.owners` array AND
//! `data.origin == "declared"`. Neither condition alone is enough:
//! `owners: []` alone would ALSO admit a creature stat-block record (a
//! `gen_book_cache`-written entry carries no `owners` field at all, so
//! `.as_array().is_none_or(..)` reads it as vacuously empty) — `origin`
//! structurally excludes those too, because that field only exists on the
//! flat `scripts/ingest_companion.py`-shaped records this catalog reads
//! (confirmed: `inner_sea_combat/companion/companion_griffon.json` carries
//! no `origin` field at all, so `data["origin"].as_str() != Some("declared")`
//! excludes it before this catalog ever looks at its `owners`). `origin`'s
//! two other real values, `"mod_only"`/`"copy"`, are what the SAME check
//! refuses on the other side — a PCGen `.MOD`/`.COPY=` delta row, which this
//! catalog has no second record to resolve a delta against (see the
//! render-and-refuse section below for the confirmed real example). A
//! `" ~ "` group qualifier in the key is common (most
//! pool members are archetype/trick/evolution-scoped) but not required —
//! two real Advanced Player's Guide records (`Companion Bonus Skill`,
//! `Eidolon Bonus Skill`) are genuine, ungrouped, clean-rendering standalone
//! content with no group prefix at all, and are served as singleton pools
//! (their own key is their own group) rather than excluded on a syntax
//! technicality. `companion_catalog.rs`'s `KNOWN_UNTRANSCRIBED_COMPANION_
//! RECORDS` still names the delta rows individually, deliberately, because
//! there is no second record here to resolve a delta against.
//!
//! # Where a pool member's words come from (SD-35 `AT-35-E6-003` cycle 3)
//!
//! A pool member's description is the CONVERTED record's own prose, read from
//! `data/sheet_rules/<book>/companion/<slug>.json` through
//! [`catalog_description`]. Until this cycle it was the ingest format's own
//! unresolved description string, resolved here at run time and refused when a
//! term this catalog could not compute was still standing. Both halves of that
//! moved: the substitution now happens once, at ingest, in
//! `src/pcgen_import/sheet_rule/`, and a term no catalog screen can settle
//! prints **the rule's words** instead of being refused
//! (`decisions.md` §1 form 3 — a final number, dice in final form, or the
//! rule's words). Ultimate Wilderness's `Pilferer ~ Sneak` is the worked
//! example: it used to be refused outright because its bonus needs the
//! master's level, and it now reads as a sentence with that term named.
//!
//! The refusal that remains is the honest one: a record whose converted rule
//! states no descriptive prose at all has nothing to serve, and
//! [`catalog_description`] answers `None` for it. Never a partial or
//! fabricated sentence — the same disposition, asked of our own schema.
//!
//! # PI screening
//!
//! Already discharged upstream by `scripts/ingest_companion.py` before a
//! record is ever written to `data/corpus/` (the same trust boundary
//! `class_feature_pool_catalog.rs` documents for its own ingest path). This
//! module reads only that already-screened output and re-runs no PI check of
//! its own.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use codex::rules_core::corpus_loader::live_sheet_rules;
use codex::rules_core::rules_tables::companion_chassis;
use codex::rules_core::sheet_rule::SheetRulePackage;
use codex::rules_core::sheet_rule_catalog::{
    catalog_description, catalog_description_or_fields, DescriptionTier,
};

/// One reference-pool member's real corpus row, with a description proven to
/// render with nothing missing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanionPoolAbilityDto {
    /// The canonical `<book>:companion:<slug>` identity — `<slug>` is the
    /// corpus record's own on-disk file stem, not a second slug formula, so
    /// this can never drift from the file `every_served_key_matches_a_
    /// corpus_record_file` checks against.
    pub key: String,
    /// The corpus `data.key` field VERBATIM (`"Animal Trick ~ Aid"`, not the
    /// `key` field's slugged wire form). `reach_gate.rs`'s `corpus_record_
    /// keys` reads this exact raw field as its ingested-record identity for
    /// EVERY companion record, owned or not — `scripts/ingest_companion.py`
    /// writes this shape's `data.key` as the raw PCGen `KEY:` token rather
    /// than a slugged `<book>:companion:<slug>` identity (unlike the
    /// `gen_book_cache`-written creature/owned-ability records, whose raw
    /// name happens to slug back to itself). Carried as its own field, not
    /// reconstructed, so a caller matching the reach denominator does not
    /// have to reverse a slug formula that was never applied here.
    pub corpus_key: String,
    /// The corpus `KEY:` token's `" ~ "`-split group prefix (e.g. `"Animal
    /// Trick"`, `"Aberrant Companion"`) — the pool this record is a member
    /// of.
    pub pool_group: String,
    pub name: String,
    /// The converted record's own words ([`catalog_description`]) — one final
    /// number, dice in final form, or the rule's words for a term no catalog
    /// screen can settle. Never empty or the PI-redaction marker: a record that
    /// states no descriptive prose is refused before it reaches this struct.
    pub description: String,
    /// `true` when `description` came from the converted rule's stat-block
    /// lines or its typed fields rather than the record's own authored prose (a
    /// `.COPY=` template/variant row, SD-32 row 20) — the same honesty
    /// distinction `reference_library_catalog.rs` carries, so a caller can
    /// tell a rendered stat line from a genuine sentence.
    pub is_mechanical_summary: bool,
}

/// One book's pool, grouped by the `" ~ "` prefix every member shares.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanionPoolGroupDto {
    /// The book's wire code — same vocabulary `CompanionCatalogEntryDto::book`
    /// uses (`"UW"`, `"UM"`, `"ARG"`, `"BOTD1"`, ...).
    pub book: String,
    pub pool_group: String,
    pub abilities: Vec<CompanionPoolAbilityDto>,
}

/// Repo root, from this crate's compile-time manifest dir rather than the
/// process's cwd — the same derivation `reach_gate.rs`/`corpus_ingest_
/// diagnostic.rs` use, and for the same reason (a test's cwd is not
/// guaranteed to be the repo root).
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// The converted package's id for a corpus `companion` record: `<book>:companion:
/// <slug of the record's own key>`.
///
/// The slug is taken from the record's `data.key` (`"Animal Trick ~ Aid"` ->
/// `animal_trick_aid`), NOT from its on-disk file stem (`aid`) — the converter
/// mints a record's id from its key, and for this ingest path the two differ for
/// every `" ~ "`-grouped row. [`CompanionPoolAbilityDto::key`] keeps the file-stem
/// identity `reach_gate.rs` joins on; this is only how the same record's converted
/// words are found.
///
/// `beastiary` is the corpus directory's own historical misspelling; the converter
/// writes that book under `bestiary`, so the one rename is applied here rather than
/// leaving 25 records unresolvable.
fn converted_id(corpus_book: &str, corpus_key: &str) -> String {
    let book = if corpus_book == "beastiary" { "bestiary" } else { corpus_book };
    format!("{book}:companion:{}", codex::rules_core::sheet_rule::slug(corpus_key))
}

/// `true` for a real, servable description value — reproduced from `class_
/// feature_pool_catalog.rs`'s own copy, this crate's disjoint-file-touch
/// convention rather than a shared dependency for a three-line predicate.
fn is_real_description_value(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !matches!(lower.as_str(), ".clear" | ".clearall" | "[redacted pi]")
}

/// One flat pool-member row, read from one corpus book's `companion/`
/// directory. Kept separate from [`CompanionPoolAbilityDto`] (which carries
/// only the wire code, not the corpus book id) so callers that need to join
/// against `companion_chassis` books (by `corpus_book`) do not have to
/// reverse a wire-code lookup.
#[derive(Debug, Clone)]
struct RawPoolEntry {
    corpus_book: String,
    corpus_key: String,
    pool_group: String,
    slug: String,
    name: String,
    description: String,
    is_mechanical_summary: bool,
}

/// Reads every `" ~ "`-qualified, `owners: []` companion record across every
/// book `companion_chassis::COMPANION_BOOKS` registers, applying the
/// render-and-refuse safety gate to each. Deliberately walks the SAME book
/// set the transcribed table covers (not every `data/corpus/*/companion/`
/// directory on disk) — a companion book this catalog has not registered at
/// all is a different, `KNOWN_UNTRANSCRIBED_COMPANION_RECORDS`-adjacent gap,
/// not one this generic pass silently annexes.
fn load_raw_pool_entries(repo_root: &Path) -> Vec<RawPoolEntry> {
    let mut out = Vec::new();
    let package: Option<&'static SheetRulePackage> = live_sheet_rules();
    for book in companion_chassis::COMPANION_BOOKS {
        let dir = repo_root.join("data/corpus").join(book.corpus_book).join("companion");
        let Ok(read_dir) = std::fs::read_dir(&dir) else { continue };
        let mut files: Vec<PathBuf> = read_dir
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
            .collect();
        files.sort();
        for file in files {
            let Ok(text) = std::fs::read_to_string(&file) else { continue };
            let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
            let data = &doc["data"];
            let Some(key) = data["key"].as_str() else { continue };
            let owners_empty = data["owners"].as_array().is_none_or(|a| a.is_empty());
            if !owners_empty {
                // Owned by a creature row of this book -- already served by
                // `companion_catalog.rs` under that creature. Not this
                // catalog's record to duplicate.
                continue;
            }
            // `origin` distinguishes a genuine standalone pool row
            // (`"declared"` -- a record stated in full in its own right, the
            // shared reference-library shape this module's doc
            // comment names) from a delta row that states only a CHANGE on
            // some other record (`"mod_only"` / `"copy"` -- PCGen `.MOD`/
            // `.COPY=` rows). A `mod_only` row's `description` can render
            // perfectly clean PCGen syntax while still being a meaningless
            // fragment without the base row it modifies (confirmed: real
            // record `beastiary/companion/universal_monster_rule_fast_
            // healing.json`, `origin: "mod_only"`, description "Works only
            // in gusty and windy areas." -- a dangling conditional clause,
            // not a sentence) -- structurally excluded, unchanged.
            //
            // SD-32 row 20: `"copy"` (a `.COPY=` row) is admitted separately
            // below, NOT folded into the `"declared"` description path,
            // because it is a genuinely different shape from `mod_only`: a
            // `.COPY=` template/variant row does not carry a dangling
            // fragment of some other record's prose at all -- re-derived
            // corpus-wide (`data/corpus/*/companion/*.json`, `origin ==
            // "copy"`), all 25 real `.COPY=` companion records carry
            // `description: null` and instead carry real, self-contained
            // mechanical tokens (`TEMPLATE`/`KIT` for a creature-template
            // application header like `Cat (Fiendish)`, or `ASPECT` for an
            // ability variant like `Pooka ~ Change Shape`'s "2 of the
            // following forms: cat, goat, rabbit ..."). This is the exact
            // tier-3 shape `reference_library_catalog.rs` already built for
            // the twelve reference-library kinds, reused here rather than
            // reinvented.
            //
            // SD-35 `AT-35-E6-003`: those facts are now read from the
            // CONVERTED record's stat-block lines and typed fields
            // (`catalog_description_or_fields`), not from the ingest format's
            // token rows at run time -- the same swap
            // `reference_library_catalog.rs` made for its own twelve kinds.
            let origin = data["origin"].as_str();
            if origin == Some("copy") {
                let Some(name) = data["name"].as_str() else { continue };
                let Some(package) = package else { continue };
                let Some(rule) = package.rule(&converted_id(book.corpus_book, key)) else {
                    continue;
                };
                let Some(resolved) = catalog_description_or_fields(package, rule) else { continue };
                let summary = resolved.text;
                if !is_real_description_value(&summary) {
                    continue;
                }
                let mechanical = resolved.tier != DescriptionTier::Prose;
                let group = key.split(" ~ ").next().unwrap_or(key).to_string();
                let Some(slug) = file.file_stem().map(|s| s.to_string_lossy().into_owned())
                else {
                    continue;
                };
                out.push(RawPoolEntry {
                    corpus_book: book.corpus_book.to_string(),
                    corpus_key: key.to_string(),
                    pool_group: group,
                    slug,
                    name: name.trim_end_matches('*').trim().to_string(),
                    description: summary,
                    is_mechanical_summary: mechanical,
                });
                continue;
            }
            if origin != Some("declared") {
                continue;
            }
            let Some(name) = data["name"].as_str() else { continue };
            let group = key.split(" ~ ").next().unwrap_or(key).to_string();
            let Some(slug) = file.file_stem().map(|s| s.to_string_lossy().into_owned()) else {
                continue;
            };
            // The converted record's own words. The substitution this call site
            // used to perform at run time already happened at ingest
            // (`src/pcgen_import/sheet_rule/`), and a term no catalog screen can
            // settle prints the rule's words rather than a number nobody
            // computed (`decisions.md` §1 form 3, `sheet_rule_catalog`).
            let Some(package) = package else { continue };
            let Some(rule) = package.rule(&converted_id(book.corpus_book, key)) else { continue };
            // The refuse gate, restated over the converted package: a record that
            // states no descriptive prose at all has nothing to serve. It is the
            // same disposition as before -- refused, never served a partial or
            // fabricated sentence -- asked of our own schema rather than of the
            // ingest format's argument tail.
            let Some(description) = catalog_description(package, rule) else { continue };
            if !is_real_description_value(&description) {
                continue;
            }
            out.push(RawPoolEntry {
                corpus_book: book.corpus_book.to_string(),
                corpus_key: key.to_string(),
                pool_group: group,
                slug,
                name: name.trim_end_matches('*').trim().to_string(),
                description,
                is_mechanical_summary: false,
            });
        }
    }
    out
}

/// Builds every book's pool groups, keyed by wire code via `wire_code_of` so
/// callers (`companion_catalog.rs`) stay the single source of the corpus-book
/// → wire-code map rather than this module keeping a second copy.
pub fn build_companion_pool_groups(
    repo_root: &Path,
    wire_code_of: impl Fn(&str) -> &'static str,
) -> Vec<CompanionPoolGroupDto> {
    let raw = load_raw_pool_entries(repo_root);
    let mut groups: std::collections::BTreeMap<(String, String), Vec<CompanionPoolAbilityDto>> =
        std::collections::BTreeMap::new();
    for entry in raw {
        let wire = wire_code_of(&entry.corpus_book).to_string();
        let key = format!("{}:companion:{}", entry.corpus_book, entry.slug);
        groups.entry((wire, entry.pool_group.clone())).or_default().push(CompanionPoolAbilityDto {
            key,
            corpus_key: entry.corpus_key,
            pool_group: entry.pool_group,
            name: entry.name,
            description: entry.description,
            is_mechanical_summary: entry.is_mechanical_summary,
        });
    }
    groups
        .into_iter()
        .map(|((book, pool_group), mut abilities)| {
            abilities.sort_by(|a, b| a.key.cmp(&b.key));
            CompanionPoolGroupDto { book, pool_group, abilities }
        })
        .collect()
}

/// Convenience wrapper for production call sites, which always want the real
/// repo root.
pub fn load_companion_pool_groups(wire_code_of: impl Fn(&str) -> &'static str) -> Vec<CompanionPoolGroupDto> {
    build_companion_pool_groups(&repo_root(), wire_code_of)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The served population, pinned as a ratchet (SD-35 `AT-35-E6-003` cycle 3).
    ///
    /// Reading the converted package instead of resolving the ingest format's
    /// description at run time moved this catalog from **407** served
    /// `declared` rows to **459**, with **0** of the 407 lost — the before/after
    /// key sets were captured on either side of the swap and diffed, and the 52
    /// gained are rows whose bonus stands on a term no catalog screen can settle
    /// (a companion's breath weapon scaled by its master's level, a poison DC),
    /// which used to be refused outright and now read as the rule's words.
    ///
    /// The 25 `.COPY=` mechanical-summary rows are a separate, unchanged path.
    /// A drop below either floor is a served row that stopped reaching a player.
    #[test]
    fn the_served_pool_population_never_falls_below_its_recorded_floor() {
        let entries = load_raw_pool_entries(&repo_root());
        let declared = entries.iter().filter(|e| !e.is_mechanical_summary).count();
        let summary = entries.iter().filter(|e| e.is_mechanical_summary).count();
        println!("POOL_CENSUS total={} declared={declared} summary={summary}", entries.len());
        assert!(declared >= 459, "served declared pool rows fell to {declared}, below the recorded 459");
        assert!(summary >= 25, "served mechanical-summary rows fell to {summary}, below the recorded 25");
        for entry in &entries {
            assert!(
                !entry.description.is_empty(),
                "{} reached the wire with an empty description",
                entry.slug
            );
        }
    }

    #[test]
    fn is_real_description_value_refuses_empty_clear_and_the_pi_marker() {
        assert!(!is_real_description_value(""));
        assert!(!is_real_description_value("   "));
        assert!(!is_real_description_value(".CLEAR"));
        assert!(!is_real_description_value(".ClearAll"));
        assert!(!is_real_description_value("[REDACTED PI]"));
        assert!(is_real_description_value("you move at full speed"));
    }

    /// The exact real shape `ultimate_wilderness/companion/aid.json` carries:
    /// a `" ~ "`-qualified key, no owners, a clean prose description with no
    /// `%N` argument. Must be served.
    #[test]
    fn a_clean_orphan_pool_row_renders_and_serves() {
        let repo = repo_root();
        let path = repo.join("data/corpus/ultimate_wilderness/companion/aid.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_raw_pool_entries(&repo);
        let found = entries
            .iter()
            .find(|e| e.corpus_book == "ultimate_wilderness" && e.slug == "aid")
            .expect("Animal Trick ~ Aid must be served by the pool catalog");
        assert_eq!(found.pool_group, "Animal Trick");
        assert_eq!(found.name, "Aid");
        assert_eq!(found.corpus_key, "Animal Trick ~ Aid");
        assert!(!found.description.is_empty());
        assert!(!found.description.contains('%'), "an unresolved formula reached the description");
    }

    /// A `.MOD`-only delta row (`beastiary/companion/universal_monster_rule_
    /// fast_healing.json`, real corpus record) renders perfectly clean PCGen
    /// syntax (`"Works only in gusty and windy areas."`) while still being a
    /// meaningless fragment without the base record it modifies. Structural
    /// `origin == "declared"` gate must refuse it even though the render-
    /// and-refuse gate alone would not catch it -- this is the exact
    /// regression `every_served_key_matches_a_corpus_record_file` caught
    /// while building this module.
    #[test]
    fn a_mod_only_delta_row_is_refused_even_though_it_renders_clean() {
        let repo = repo_root();
        let path = repo.join("data/corpus/beastiary/companion/universal_monster_rule_fast_healing.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_raw_pool_entries(&repo);
        assert!(
            !entries
                .iter()
                .any(|e| e.corpus_book == "beastiary" && e.slug == "universal_monster_rule_fast_healing"),
            "a .MOD-only delta row must never be served as a standalone pool member"
        );
    }

    /// SD-32 row 20: a `.COPY=` creature-template row (`beastiary/companion/
    /// cat_fiendish.json`, real corpus record, `origin: "copy"`,
    /// `description: null`, `TEMPLATE`/`KIT` raw tokens) is admitted via the
    /// new tier-3 mechanical-summary path, distinct from `mod_only`'s
    /// structural refusal immediately above -- the two `origin` values are
    /// NOT treated the same way, on purpose.
    #[test]
    fn a_copy_template_row_is_served_as_a_mechanical_summary() {
        let repo = repo_root();
        let path = repo.join("data/corpus/beastiary/companion/cat_fiendish.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_raw_pool_entries(&repo);
        let found = entries
            .iter()
            .find(|e| e.corpus_book == "beastiary" && e.slug == "cat_fiendish")
            .expect("Cat (Fiendish) must be served via the .COPY= tier-3 admission");
        assert_eq!(found.corpus_key, "Cat (Fiendish)");
        assert!(found.is_mechanical_summary, "a .COPY= template row has no real prose to render");
        // SD-35 `AT-35-E6-003` cycle 12: the row's fact -- that it applies the Fiendish
        // Creature template -- is read from the converted package, where it lives on the far
        // end of the grant edge, and printed as words. It used to be the source token's own
        // head and value.
        assert_eq!(found.description, "Grants Fiendish Creature");
        assert!(!found.description.is_empty());
    }

    /// The same `.COPY=` admission for an ability-variant row carrying a real
    /// `ASPECT` token (`bestiary_4/companion/pooka_change_shape.json`) --
    /// proves the mechanism serves more than just creature-template headers.
    #[test]
    fn a_copy_ability_variant_row_is_served_from_its_aspect_token() {
        let repo = repo_root();
        let path = repo.join("data/corpus/bestiary_4/companion/pooka_change_shape.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_raw_pool_entries(&repo);
        let found = entries
            .iter()
            .find(|e| e.corpus_book == "bestiary_4" && e.slug == "pooka_change_shape")
            .expect("Pooka ~ Change Shape must be served via the .COPY= tier-3 admission");
        assert_eq!(found.pool_group, "Pooka");
        assert!(found.is_mechanical_summary);
        // The same swap: the variant's forms are the converted rule's own stat-block line, not
        // the source token's head and value.
        assert!(
            found.description.contains("cat, goat, rabbit"),
            "expected the converted record's own words, got: {}",
            found.description
        );
        assert!(
            !found.description.contains("ASPECT:"),
            "the source token head must not reach the screen: {}",
            found.description
        );
    }

    /// A row whose bonus stands on a term this catalog cannot settle
    /// (`ultimate_wilderness/companion/sneak.json`, `Pilferer ~ Sneak`, whose
    /// competence bonus is half the master's level) is served as **the rule's
    /// words**, never as a dropped digit and never as the characterless `0` the
    /// sheet evaluator would compute for it.
    ///
    /// SD-35 `AT-35-E6-003` cycle 3 deliberately moved this expectation. Until
    /// this cycle the record was refused outright, which `decisions.md` §1
    /// form 3 rules out: an unresolvable term prints the rule's words.
    #[test]
    fn a_pool_row_standing_on_an_unsettled_term_is_served_as_the_rules_words() {
        let repo = repo_root();
        let path = repo.join("data/corpus/ultimate_wilderness/companion/sneak.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_raw_pool_entries(&repo);
        let found = entries
            .iter()
            .find(|e| e.corpus_book == "ultimate_wilderness" && e.slug == "sneak")
            .expect("Pilferer ~ Sneak must be served with its own words");
        assert_eq!(found.corpus_key, "Pilferer ~ Sneak");
        assert!(
            found.description.contains("competence bonus"),
            "expected the record's own sentence, got: {}",
            found.description
        );
        assert!(
            !found.description.contains("+0 "),
            "the unsettled term reached the wire as a computed zero: {}",
            found.description
        );
    }

    /// A genuine, ungrouped, clean-rendering standalone record (`Companion
    /// Bonus Skill`, real Advanced Player's Guide corpus record — no `" ~ "`
    /// group prefix, `description: "Add +1 skill rank."`, no `%N`) is served
    /// as its own singleton pool rather than excluded for lacking a group
    /// syntax it was never going to have.
    #[test]
    fn an_ungrouped_clean_record_is_served_as_its_own_singleton_pool() {
        let repo = repo_root();
        let path = repo.join("data/corpus/advanced_players_guide/companion/companion_bonus_skill.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_raw_pool_entries(&repo);
        let found = entries
            .iter()
            .find(|e| e.corpus_book == "advanced_players_guide" && e.slug == "companion_bonus_skill")
            .expect("Companion Bonus Skill must be served by the pool catalog");
        assert_eq!(found.corpus_key, "Companion Bonus Skill");
        assert_eq!(found.pool_group, "Companion Bonus Skill", "an ungrouped key is its own group");
        assert_eq!(found.description, "Add +1 skill rank.");
    }

    /// A creature stat-block record (`gen_book_cache`-written, no `owners`
    /// field and no `origin` field at all) must never be admitted merely
    /// because a missing `owners` field reads as vacuously empty — the
    /// `origin == "declared"` gate is what actually protects this, and this
    /// test is the regression guard for it (the exact near-miss found while
    /// widening this module past the `" ~ "` requirement).
    #[test]
    fn a_creature_stat_block_record_is_never_admitted_as_a_pool_member() {
        let repo = repo_root();
        let path = repo.join("data/corpus/inner_sea_combat/companion/companion_griffon.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_raw_pool_entries(&repo);
        assert!(
            !entries.iter().any(|e| e.corpus_book == "inner_sea_combat" && e.slug == "companion_griffon"),
            "a creature stat-block record must never be served as a pool member"
        );
    }

    /// An owned ability (real creature owner, e.g. `Sea Krait ~ Poison`) is
    /// never duplicated here — it already reaches the wire under its
    /// creature via `companion_catalog.rs`.
    #[test]
    fn an_owned_ability_with_a_tilde_group_is_not_duplicated() {
        let repo = repo_root();
        let path = repo.join("data/corpus/ultimate_wilderness/companion/sea_krait_poison.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_raw_pool_entries(&repo);
        assert!(
            !entries.iter().any(|e| e.corpus_book == "ultimate_wilderness" && e.slug == "sea_krait_poison"),
            "an owned ability must not be re-served as a pool member"
        );
    }

    /// Mutation-proves-RED per the universal requirement: the refuse gate is
    /// live, not vacuous. Asked of the LIVE converted package rather than of a
    /// hand-written fixture (`decisions.md` §4): across every converted
    /// `companion` record in `data/sheet_rules/`, some state descriptive prose
    /// and some state none, so `catalog_description`'s `None` arm is reached and
    /// its `Some` arm is reached. A gate that answered the same way for every
    /// record would be the vacuous one this test exists to catch.
    #[test]
    fn the_refuse_gate_is_provably_live_over_the_converted_package() {
        let package = live_sheet_rules().expect(
            "data/sheet_rules/ must be present (cargo run --locked --bin sheet_rule_convert)",
        );
        let mut with_prose = 0usize;
        let mut without_prose = 0usize;
        for (id, rule) in &package.rules {
            if !id.contains(":companion:") {
                continue;
            }
            match catalog_description(package, rule) {
                Some(text) if is_real_description_value(&text) => with_prose += 1,
                _ => without_prose += 1,
            }
        }
        println!("COMPANION_RULES with_prose={with_prose} without_prose={without_prose}");
        assert!(with_prose > 0, "no converted companion record states any descriptive prose");
        assert!(without_prose > 0, "the refuse arm is never reached -- the gate is vacuous");
    }

    #[test]
    fn build_companion_pool_groups_groups_by_book_and_pool_prefix() {
        let repo = repo_root();
        let groups = build_companion_pool_groups(&repo, |corpus_book| {
            if corpus_book == "ultimate_wilderness" { "UW" } else { "OTHER" }
        });
        // Restrict to ultimate_wilderness's own "Animal Trick" group so this
        // assertion is not coupled to every other book's row count.
        let animal_trick = groups
            .iter()
            .find(|g| g.book == "UW" && g.pool_group == "Animal Trick")
            .expect("ultimate_wilderness must carry an Animal Trick pool group");
        assert!(animal_trick.abilities.iter().any(|a| a.key.ends_with(":aid")));
        for ability in &animal_trick.abilities {
            assert_eq!(ability.pool_group, "Animal Trick");
            assert!(!ability.description.is_empty());
        }
    }
}
