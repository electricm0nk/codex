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
//! # The render-and-refuse gate is the whole safety property
//!
//! Reused verbatim from `class_feature_pool_catalog.rs`: a pool member's
//! `description` is the raw, unresolved `DESC:` string (this ingest path
//! never splits the argument tail out the way `companion_chassis`'
//! transcriber does for owned ability rows — `data.description` already
//! carries the `|`-joined argument list verbatim, confirmed against real
//! records such as `ultimate_wilderness/companion/sneak.json`'s `"...+%1
//! competence bonus...|MasterLevel/2"`). `render_pcgen_desc` resolves it with
//! no character to resolve against; any unresolved `%N` or leaked PCGen
//! syntax refuses the record rather than serving broken text — the same
//! Decision-7 disposition `class_feature_pool_catalog.rs` uses, for the same
//! reason (a record that still needs a computation is not `text-complete`,
//! and this catalog performs none).
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

use codex::rules_core::pcgen_desc::{leaked_pcgen_syntax, render_pcgen_desc};
use codex::rules_core::rules_tables::companion_chassis;

use crate::reference_library_catalog::mechanical_summary;

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
    /// Rendered through `render_pcgen_desc`, with every unsubstituted `%N`
    /// argument refused rather than served (see the module doc's
    /// "render-and-refuse" section). Never empty or the PI-redaction marker
    /// — those never reach this struct at all.
    pub description: String,
    /// `true` when `description` is a rendered mechanical-token summary (a
    /// `.COPY=` template/variant row's `TEMPLATE`/`KIT`/`ASPECT` tokens,
    /// SD-32 row 20) rather than real authored prose — the same honesty
    /// distinction `reference_library_catalog.rs`'s own tier-3 carries, so a
    /// caller can tell a rendered token dump from a genuine sentence.
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
            // (`"declared"` -- a full `KEY:`/`DESC:` record in its own
            // right, the shared reference-library shape this module's doc
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
            // reinvented (`mechanical_summary`).
            let origin = data["origin"].as_str();
            if origin == Some("copy") {
                let Some(name) = data["name"].as_str() else { continue };
                let Some(summary) = mechanical_summary(data) else { continue };
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
                    is_mechanical_summary: true,
                });
                continue;
            }
            if origin != Some("declared") {
                continue;
            }
            let Some(name) = data["name"].as_str() else { continue };
            let Some(raw_desc) = data["description"].as_str() else { continue };
            if !is_real_description_value(raw_desc) {
                continue;
            }
            let rendered = render_pcgen_desc(raw_desc);
            // The render-and-refuse gate: an unresolved `%N` means a real
            // computation this catalog cannot perform is still missing from
            // the sentence -- refused rather than served broken.
            if !rendered.dropped_args.is_empty() {
                continue;
            }
            if leaked_pcgen_syntax(&rendered.text).is_some() {
                continue;
            }
            let group = key.split(" ~ ").next().unwrap_or(key).to_string();
            let Some(slug) = file.file_stem().map(|s| s.to_string_lossy().into_owned()) else {
                continue;
            };
            out.push(RawPoolEntry {
                corpus_book: book.corpus_book.to_string(),
                corpus_key: key.to_string(),
                pool_group: group,
                slug,
                name: name.trim_end_matches('*').trim().to_string(),
                description: rendered.text,
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
        assert!(
            found.description.contains("TEMPLATE") && found.description.contains("Fiendish"),
            "expected the mechanical summary to name the real TEMPLATE token, got: {}",
            found.description
        );
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
        assert!(
            found.description.contains("ASPECT"),
            "expected the mechanical summary to name the real ASPECT token, got: {}",
            found.description
        );
    }

    /// A record with an unresolvable `%N` (e.g. `sneak.json`'s `"...|
    /// MasterLevel/2"` tail) is refused, not served with a dropped digit.
    #[test]
    fn a_pool_row_with_an_unresolvable_formula_is_refused() {
        let repo = repo_root();
        let path = repo.join("data/corpus/ultimate_wilderness/companion/sneak.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_raw_pool_entries(&repo);
        assert!(
            !entries.iter().any(|e| e.corpus_book == "ultimate_wilderness" && e.slug == "sneak"),
            "a record whose description needs MasterLevel resolved must not be served"
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

    /// Mutation-proves-RED per the universal requirement: the render-and-
    /// refuse gate is live, not vacuous.
    #[test]
    fn render_and_refuse_gate_is_provably_live() {
        let clean = render_pcgen_desc("you move at full speed");
        assert!(clean.dropped_args.is_empty());
        let broken = render_pcgen_desc("you gain a +%1 bonus|SomeUnresolvedVar");
        assert!(!broken.dropped_args.is_empty(), "the gate must see this as unresolved");
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
