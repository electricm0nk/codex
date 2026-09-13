//! v0.6 alpha swarm feat catalog browser — Tauri command adapter over the
//! full feat table store across every ingested rule book
//! (`rules_tables::feats_all::all_feat_tables`): 690 real corpus records,
//! 185 CRB + 172 APG + 129 ACG + 187 ARG + 17 PU.
//!
//! Mirrors `equipment_catalog.rs`'s own command/pure-fn split and
//! unfiltered/filtered command pair exactly — this is a standalone catalog
//! view of every real feat record the engine knows about, for the
//! frontend's Feat picker.
//!
//! **This was CRB-only until the APG/ACG ingest.** The other books'
//! feat tables did not exist anywhere in the engine, so a player building
//! an APG or ACG class could not take a single feat from that class's own
//! book. Reading the aggregate rather than `crb::feats::feat_tables()`
//! directly is what puts those 505 feats in front of a player; every DTO
//! now names its `source` book, the same way the spell catalog already
//! does. Advanced Race Guide and Pathfinder Unchained joined the aggregate
//! after that widening and reach the picker through the same path, with no
//! change needed here beyond the record type the aggregate now hands out
//! (`feats_all::FeatCatalogRecord` — see that module's own doc comment for
//! why ARG's and PU's tables could not honestly be folded into
//! `crb::feats::FeatTableEntry`).

use serde::{Deserialize, Serialize};

use codex::rules_core::feat_effects;
use codex::rules_core::feat_prereqs::{
    evaluate_catalog_feat_prerequisites, FeatPrerequisiteReport, PrereqFacts,
};
use codex::rules_core::rules_tables::feats_all::all_feat_tables;
use codex::rules_core::rules_tables::RuleSetId;

/// One feat's prerequisite verdict for the character the picker is open
/// for. Absent (`None`) when the catalog is served with no character
/// context at all -- `list_feats` / `list_feat_catalog`, which the Tester
/// Workbench and the release-checks surfaces read.
///
/// The frontend greys a row out when `eligible` is false and shows
/// `unavailableReason` on it. `unverified` is shown as a note on rows that
/// stay selectable, so a player is told what could not be checked instead
/// of being quietly allowed or quietly denied.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatEligibilityDto {
    pub eligible: bool,
    /// One line, already joined, for the greyed-out row. `None` exactly
    /// when `eligible` is true.
    pub unavailable_reason: Option<String>,
    /// The prerequisites this character satisfies.
    pub met: Vec<String>,
    /// The prerequisites this character does not satisfy. Empty when
    /// `eligible`.
    pub unmet: Vec<String>,
    /// Prerequisites the engine could not evaluate. These never block --
    /// see `feat_prereqs::converted_gate`'s three-outcome design.
    pub unverified: Vec<String>,
    /// How many top-level prerequisite terms the record's converted gate carries. `0`
    /// means the feat genuinely has no prerequisites.
    pub prerequisite_count: usize,
}

fn map_eligibility_dto(report: &FeatPrerequisiteReport) -> FeatEligibilityDto {
    FeatEligibilityDto {
        eligible: report.is_eligible,
        unavailable_reason: report.unavailable_reason(),
        met: report.met.clone(),
        unmet: report.unmet.iter().map(|failed| failed.reason.clone()).collect(),
        unverified: report.unverified.iter().map(|warning| warning.message.clone()).collect(),
        prerequisite_count: report.prerequisite_token_count,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatCatalogEntryDto {
    pub key: String,
    /// The `FeatCategory` variant name verbatim (e.g. "Combat").
    pub category: String,
    pub name: String,
    pub description: Option<String>,
    /// Which rule book this record came from — the `RuleSetId` variant
    /// name verbatim, i.e. `"Crb"`, `"Apg"`, `"Acg"`, `"Arg"` or `"Pu"`.
    /// Read off the `BookFeatTable` the entry belongs to, never inferred
    /// from the key.
    ///
    /// It is also what distinguishes the catalog's one cross-book
    /// duplicate key: `Endurance` is served twice, once from CRB and once
    /// from PU, which re-lists that feat under its Wound Threshold rules
    /// (pinned by `feats_all`'s
    /// `cross_book_key_collisions_are_exactly_the_known_set`).
    ///
    /// A player picking a feat needs to know which book it is from, the
    /// same reason `SpellCatalogEntryDto` carries `book`.
    pub source: String,
    /// `"Weapon"`, `"Skill"` or `"SpellSchool"` for a feat whose target the
    /// engine consumes; `None` for every other feat.
    ///
    /// This is deliberately narrower than the corpus: many more feats carry a
    /// `CHOOSE:` token, but a target recorded against a feat no producer
    /// reads would render in the picker and change nothing computed. Only
    /// the feats in `feat_effects::CHOOSER_FEAT_CONTRACTS` are marked, so a
    /// prompt shown to a player always leads to real arithmetic.
    pub chooser_target_kind: Option<String>,
    /// This feat's prerequisite verdict for the character the picker is
    /// open for, or `None` when the catalog was requested with no
    /// character (`list_feats` / `list_feat_catalog`).
    ///
    /// `#[serde(skip_serializing_if)]` so the character-less callers send
    /// no key at all rather than a literal `null` a frontend `!== undefined`
    /// check would wave through as "checked, and fine".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibility: Option<FeatEligibilityDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatCatalogResponse {
    pub entries: Vec<FeatCatalogEntryDto>,
}

/// The `data/corpus/<dir>/` directory each compiled feat book is ingested from.
///
/// Exhaustive on purpose, and it panics on a variant nobody mapped, for the same reason
/// `equipment_catalog::corpus_book_dir` and `spell_catalog::corpus_book_dir` do: a silently
/// unmapped book would serve no words at all for every one of its feats, and a screen showing
/// nothing looks exactly like a book with nothing to show. The mapping is the same one
/// `src/bin/v06_work_inventory.rs::corpus_dir_for` carries — the inventory's own book identity,
/// not a second opinion about it.
fn corpus_book_dir(rule_set: RuleSetId) -> &'static str {
    match rule_set {
        RuleSetId::Crb => "core_rulebook",
        RuleSetId::Apg => "advanced_players_guide",
        RuleSetId::Acg => "advanced_class_guide",
        // The one id not spelled like its directory.
        RuleSetId::Bestiary1 => "bestiary",
        RuleSetId::Arg => "advanced_race_guide",
        RuleSetId::Pu => "pathfinder_unchained",
        RuleSetId::Uca => "ultimate_campaign",
        RuleSetId::Ui => "ultimate_intrigue",
        RuleSetId::Ue => "ultimate_equipment",
        RuleSetId::Uw => "ultimate_wilderness",
        RuleSetId::Uc => "ultimate_combat",
        RuleSetId::Um => "ultimate_magic",
        RuleSetId::Upsi => "ultimate_psionics",
        RuleSetId::BonusBestiary => "bonus_bestiary",
        RuleSetId::MonsterCodex => "monster_codex",
        RuleSetId::Isr => "inner_sea_races",
        RuleSetId::Ha => "horror_adventures",
        RuleSetId::Botd1 => "book_of_the_damned_volume_1",
        RuleSetId::Botd2 => "book_of_the_damned_volume_2",
        RuleSetId::Iswg => "inner_sea_world_guide",
        RuleSetId::Ce => "core_essentials",
        RuleSetId::Isc => "inner_sea_combat",
        RuleSetId::Isi => "inner_sea_intrigue",
        RuleSetId::B5 => "bestiary_5",
        RuleSetId::B6 => "bestiary_6",
        RuleSetId::B2 => "bestiary_2",
        RuleSetId::B3 => "bestiary_3",
        RuleSetId::B4 => "bestiary_4",
        RuleSetId::Isb => "inner_sea_bestiary",
        RuleSetId::Isg => "inner_sea_gods",
        RuleSetId::Oa => "occult_adventures",
        RuleSetId::Mythic => "mythic_adventures",
        RuleSetId::AdventurersGuide => "adventurers_guide",
        RuleSetId::InnerSeaFaiths => "inner_sea_faiths",
        RuleSetId::InnerSeaMagic => "inner_sea_magic",
        RuleSetId::InnerSeaTaverns => "inner_sea_taverns",
        RuleSetId::InnerSeaTemples => "inner_sea_temples",
    }
}

/// The words this catalog serves for one feat row: **the converted record's own**, never the
/// compiled table's stored string re-parsed at run time.
///
/// SD-35 `decisions.md §11` — nothing on the live side reads the ingest format. Until
/// `AT-35-E6-003` cycle 8 this module handed the compiled table's stored description to a
/// run-time rewriter, because a minority of those strings still carried the source format's
/// positional markers and literal-percent escape. That rewriter *is* the ingest-format reader
/// the ruling removes, and the substitution it performed already happens at ingest
/// (`src/pcgen_import/sheet_rule/`):
/// `sheet_rule_catalog::catalog_description` renders the converted record with no character in
/// hand — a final number where the term is settled, the rule's own words where it is not
/// (`decisions.md §1`'s three printed forms).
///
/// A row the converted package holds under no rule keeps the compiled table's stored string,
/// **but only when that string is already the record's plain words**: one still carrying a `%`
/// marker is refused rather than shown half-rendered, because the rewriter that used to clean
/// it up is exactly what left the live side. Refusing whole is the disposition the rest of this
/// crate takes — never a partial sentence. This is `equipment_catalog::row_description`'s shape,
/// applied to the feat tables, with one difference stated deliberately: the refusal predicate is
/// [`leaked_pcgen_syntax`](codex::pcgen_import::pcgen_desc::leaked_pcgen_syntax) rather than a bare
/// "contains a `%`". That function is a **refusal**, not a reader — it answers "does this string
/// still show the ingest format", writes nothing and parses nothing — and it is the predicate the
/// rest of this crate already sweeps every served description with. It matters here because the
/// feat tables carry real English percentages ("reduce ... by 20%", "a 50% chance"), which a bare
/// `%` test would throw away as if they were unresolved markers, and `leaked_pcgen_syntax`
/// correctly exempts a digit-preceded sign while still refusing a positional marker, a keyword
/// marker and a bare `%`.
/// The refusal applies to **both** sources, not only the fallback. One converted rule,
/// corpus-wide, still renders text this sweep reads as a gap — Mythic Adventures' `Prophetic
/// Visionary` states its chance as a scaling term followed by a literal percent SIGN, the
/// converter resolves the term to the rule's words, and the sign is left against a letter
/// ("...increases by a rules variable%."). That is a **converter** finding, not a live-side one,
/// and the fix belongs on the converter side (`decisions.md §11`); until it lands, that rendering
/// is refused here rather than exempted from the crate-wide sweep
/// (`equipment_catalog::no_catalog_serves_a_description_carrying_raw_pcgen_syntax`), because a
/// gate with a name on a list in it is not a gate. Refusing it costs that row nothing today —
/// its compiled-table string is already the record's plain words, so the fallback serves them
/// and the population ratchet below is unchanged by the refusal. That is luck, not design: a
/// row whose stored string were *also* unclean would serve nothing, which is the honest outcome
/// and the reason the finding is reported rather than buried.
fn row_description(rule_set: RuleSetId, key: &str, table_text: Option<&str>) -> Option<String> {
    let clean = |text: &str| {
        codex::rules_core::pilot_compute::resolved_prose::leaked_markup(text).is_none()
    };
    if let Some(text) =
        crate::converted_prose::description_for(corpus_book_dir(rule_set), "feat", key)
    {
        if clean(&text) {
            return Some(text);
        }
    }
    table_text.filter(|text| clean(text)).map(str::to_owned)
}

fn map_catalog_entry(
    entry: &codex::rules_core::rules_tables::feats_all::FeatCatalogRecord,
    rule_set: RuleSetId,
    source: &str,
    eligibility: Option<FeatEligibilityDto>,
) -> FeatCatalogEntryDto {
    FeatCatalogEntryDto {
        eligibility,
        key: entry.key.to_string(),
        // Already the source book's own `FeatCategory` variant name
        // verbatim — the aggregate projects it there, over an exhaustive
        // per-book match, because the five books do not share one category
        // enum. Same wire strings as the previous `format!("{:?}", ..)`,
        // pinned by `feats_all`'s
        // `category_names_match_the_debug_form_of_every_variant`.
        category: entry.category.to_string(),
        name: entry.name.to_string(),
        // The converted record's own words, from [`row_description`]. The Add
        // Feat picker folds this straight into its `detail` line
        // (`itemPickerFilter::mapFeatCatalogEntries`), so whatever this field
        // holds is what a player reads — which is why it is resolved once, in
        // one place, and never re-decided here. `None` where neither the
        // converted package nor a clean stored string states anything: a real
        // and documented gap, never a fabricated stand-in.
        description: row_description(rule_set, entry.key, entry.description),
        source: source.to_string(),
        chooser_target_kind: feat_effects::chooser_contract_for_feat(entry.key)
            .map(|contract| format!("{:?}", contract.target_kind)),
    }
}

/// Build the full catalog response across every ingested book, in book
/// order (CRB, APG, ACG, ARG, PU). A thin, testable wrapper behind the
/// Tauri command below — mirrors
/// `equipment_catalog::build_equipment_catalog`.
pub fn build_feat_catalog() -> FeatCatalogResponse {
    build_feat_catalog_for(None)
}

/// Finds one feat's own already-verified description by an EXACT name
/// match — never fuzzy. Built for `class_feature_feat_bridge.rs`
/// (SD31-W29-CLASSFEATURE-FEATBRIDGE-001, THE-BOX §2.1 F2): a `class_feature`
/// record whose entire content is a grant of an already-separately-modelled
/// `feat` (`ABILITY:FEAT|AUTOMATIC|<name>` / `ABILITY:FEAT|VIRTUAL|<name>`)
/// carries no local description of its own, but the target feat's real
/// rulebook text already exists here, already rendered, already leak-
/// checked. This is that lookup, kept deliberately narrow: **exact string
/// equality only.** This codebase's own standing lesson is that a shared
/// name never implies a shared thing — a fuzzy matcher belongs, if built at
/// all, in the caller, verified per-match against the owning record, never
/// silently folded into this function.
///
/// Calls [`row_description`] — the one resolution [`map_catalog_entry`] uses — rather than
/// re-deriving a second one, so this function can never promise cleaner or different text than
/// the picker itself would ship for the same record. The match is on the **name**, but the text
/// is resolved from the matched row's own **key** and book, so the answer always comes from the
/// owning record rather than from whatever else shares the name. When a name matches multiple
/// books' feats (the catalog's own documented "Endurance" collision), the first exact match in
/// `all_feat_tables()`'s own book order (CRB, APG, ACG, ARG, PU, …) wins — the SAME determinism
/// `build_feat_catalog`'s caller already relies on for that collision, applied here rather than
/// invented fresh.
///
/// Returns `None` when no book's feat carries this exact name, and when the matched record
/// states no words in the converted package and has no clean stored string either — never a
/// guessed or partial description.
pub fn feat_description_by_exact_name(name: &str) -> Option<String> {
    for book in all_feat_tables() {
        for entry in book.entries {
            if entry.name != name {
                continue;
            }
            if let Some(text) = row_description(book.rule_set, entry.key, entry.description) {
                return Some(text);
            }
        }
    }
    None
}

fn build_feat_catalog_for(facts: Option<&PrereqFacts>) -> FeatCatalogResponse {
    let mut entries = Vec::new();
    for book in all_feat_tables() {
        let source = format!("{:?}", book.rule_set);
        entries.extend(book.entries.iter().map(|entry| {
            let eligibility = facts.map(|facts| {
                map_eligibility_dto(&evaluate_catalog_feat_prerequisites(
                    entry,
                    book.rule_set,
                    facts,
                ))
            });
            map_catalog_entry(entry, book.rule_set, &source, eligibility)
        }));
    }
    FeatCatalogResponse { entries }
}

#[tauri::command]
pub fn list_feat_catalog() -> FeatCatalogResponse {
    build_feat_catalog()
}

/// Filter criteria for `list_feats`. Every field is optional and
/// `None`/empty matches everything — mirrors
/// `equipment_catalog::EquipmentCatalogFilter` exactly.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatCatalogFilter {
    /// Case-insensitive substring match against `name`.
    pub name_contains: Option<String>,
    /// Exact match against the source book's `FeatCategory` variant name
    /// verbatim (e.g. "Combat"), as projected onto
    /// `FeatCatalogEntryDto::category`. PU contributes three category
    /// names no other book has — "Alignment", "CombatStamina",
    /// "WoundThreshold" — because its corpus groups feats by `###Block:`
    /// marker rather than by `TYPE:` facet.
    pub category: Option<String>,
    /// Exact match against the `RuleSetId` variant name verbatim (`"Crb"`,
    /// `"Apg"`, `"Acg"`, `"Arg"`, `"Pu"`), as projected onto
    /// `FeatCatalogEntryDto::source`. `None` spans every book.
    ///
    /// `#[serde(default)]` because callers that predate the APG/ACG ingest
    /// send a filter payload with no `source` key at all; that must mean
    /// "every book", not a deserialization error.
    #[serde(default)]
    pub source: Option<String>,
}

/// Narrows the full catalog to the entries matching `filter`. A thin,
/// testable wrapper behind the `list_feats` Tauri command below — mirrors
/// `equipment_catalog::filter_equipment_catalog`.
pub fn filter_feat_catalog(filter: &FeatCatalogFilter) -> FeatCatalogResponse {
    filter_feat_catalog_for(filter, None)
}

/// `filter_feat_catalog`, with each surviving record's real prerequisite
/// verdict for `facts` attached. Backs the `list_feats_for_character`
/// command.
///
/// **This is what closes the "no feat prerequisite enforcement anywhere"
/// defect at the UI boundary.** A Fighter 1 asking for the catalog gets
/// Improved Two-Weapon Fighting back with `eligible: false` and the reason,
/// so the picker can grey the row *and say why* -- rather than offering it,
/// accepting it, and silently producing an illegal character. Every record
/// surviving `filter` is still returned: an unavailable feat must be
/// visible and explained, never removed from the list.
pub fn filter_feat_catalog_with_eligibility(
    filter: &FeatCatalogFilter,
    facts: &PrereqFacts,
) -> FeatCatalogResponse {
    filter_feat_catalog_for(filter, Some(facts))
}

fn filter_feat_catalog_for(
    filter: &FeatCatalogFilter,
    facts: Option<&PrereqFacts>,
) -> FeatCatalogResponse {
    let name_needle = filter
        .name_contains
        .as_ref()
        .filter(|needle| !needle.is_empty())
        .map(|needle| needle.to_lowercase());

    let entries = build_feat_catalog_for(facts)
        .entries
        .into_iter()
        .filter(|entry| match &name_needle {
            Some(needle) => entry.name.to_lowercase().contains(needle.as_str()),
            None => true,
        })
        .filter(|entry| match &filter.category {
            Some(category) => &entry.category == category,
            None => true,
        })
        .filter(|entry| match &filter.source {
            Some(source) => &entry.source == source,
            None => true,
        })
        .collect();

    FeatCatalogResponse { entries }
}

/// Returns the all-book feat catalog narrowed by `filter` — see
/// `FeatCatalogFilter`'s own doc comment for the supported fields.
#[tauri::command]
pub fn list_feats(filter: FeatCatalogFilter) -> FeatCatalogResponse {
    filter_feat_catalog(&filter)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The number of feat rows that showed words at `AT-35-E6-003` cycle 8, corpus-wide. A
    /// **floor**: raise it deliberately when a cycle gains rows, never lower it to match a
    /// regression. Re-derive with
    /// `cargo test --locked converted_feat_prose_population -- --nocapture`.
    const FEAT_PROSE_FLOOR: usize = 2162;

    /// The same floor, per book, so a gain in one book cannot hide a loss in another — the
    /// exact hole a single corpus-wide total leaves open. Books serving nothing at the pin are
    /// listed at 0 rather than omitted, so the list is the whole roster and not a selection.
    const FEAT_PROSE_FLOOR_BY_BOOK: &[(&str, usize)] = &[
        ("Acg", 129),
        ("Apg", 171),
        ("Arg", 225),
        ("Botd2", 1),
        ("Ce", 15),
        ("Crb", 177),
        ("Ha", 61),
        ("InnerSeaTaverns", 9),
        ("Isc", 23),
        ("Isg", 86),
        ("Isi", 6),
        ("Isr", 50),
        ("Iswg", 31),
        ("MonsterCodex", 32),
        ("Mythic", 164),
        ("Oa", 68),
        ("Pu", 17),
        ("Uc", 262),
        ("Uca", 23),
        ("Ui", 107),
        ("Um", 147),
        ("Upsi", 222),
        ("Uw", 136),
    ];

    /// The converted words for APG's `Extra Hex`, asserted from two directions — the picker's
    /// own row and `feat_description_by_exact_name`'s answer for the same record — so the two
    /// call sites can never quietly disagree. Longer than the compiled table's stored string,
    /// which held only the first sentence.
    const EXTRA_HEX_CONVERTED_WORDS: &str = "You have learned the secrets of a new hex.\nYou gain \
        one additional hex. You must meet all of the prerequisites for this hex. Special - You \
        can gain Extra Hex multiple times.";

    /// The words the Feat picker serves, counted over the **whole live catalog**, per book.
    ///
    /// # What replaced what, and why
    ///
    /// Until `AT-35-E6-003` cycle 8 this slot held
    /// `feat_descriptions_are_rendered_and_otherwise_byte_identical`, which pinned the run-time
    /// rewriter's output against the compiled table's stored string — 187 rewritten rows named
    /// one by one. That test existed to prove the rewriter changed exactly the rows it had to
    /// and nothing else. The rewriter is gone (`decisions.md §11`), so the claim it pinned no
    /// longer exists to be pinned, and a test asserting the served text equals the stored text
    /// would now be asserting the opposite of what this module does.
    ///
    /// What replaces it is the claim that actually matters to a player: **how many feats show
    /// words, and does any of them show the ingest format.** A ratchet over the live corpus, per
    /// `decisions.md §4` — never a fixture with a hand-derived value. The per-book floors are a
    /// floor, not an identity: a one-for-one swap inside a book's total would read as no change
    /// here, which is why the leak sweep below runs over every served row rather than a sample.
    #[test]
    fn converted_feat_prose_population() {
        use codex::pcgen_import::pcgen_desc::leaked_pcgen_syntax;
        use std::collections::BTreeMap;

        let catalog = build_feat_catalog();
        let mut served_by_book: BTreeMap<&str, usize> = BTreeMap::new();
        let mut total_rows = 0usize;
        let mut with_description = 0usize;
        let mut leaks: Vec<(String, String, &'static str)> = Vec::new();
        for entry in &catalog.entries {
            total_rows += 1;
            let Some(text) = entry.description.as_deref() else { continue };
            with_description += 1;
            *served_by_book.entry(entry.source.as_str()).or_default() += 1;
            if let Some(reason) = leaked_pcgen_syntax(text) {
                leaks.push((entry.key.clone(), entry.source.clone(), reason));
            }
        }
        for (key, source, reason) in &leaks {
            println!("leak {source} {key}: {reason}");
        }
        // Zero, and it stays zero without an exemption list: [`row_description`] refuses a
        // leaking string from EITHER source, so a converter rendering artifact costs one row's
        // words rather than putting the ingest format on a player's screen. The one row that
        // costs today (Mythic Adventures' `Prophetic Visionary`) is named in `row_description`'s
        // own doc comment and in `AT-35-E6-003`'s cycle-8 receipt as a converter finding.
        assert!(
            leaks.is_empty(),
            "a served feat description shows the ingest format: {leaks:?} -- re-derive with \
             `cargo test --locked converted_feat_prose_population -- --nocapture`"
        );
        println!("feat rows={total_rows} with_description={with_description}");
        println!("by book: {served_by_book:?}");

        assert_eq!(total_rows, 2227, "the served catalog is the whole aggregate, unchanged");
        assert!(
            with_description >= FEAT_PROSE_FLOOR,
            "served feat descriptions fell to {with_description}, below the pinned floor of \
             {FEAT_PROSE_FLOOR} -- re-derive with `cargo test -p codex-desktop \
             converted_feat_prose_population -- --nocapture` and raise the floor deliberately, \
             never lower it to match a regression"
        );
        for (book, floor) in FEAT_PROSE_FLOOR_BY_BOOK {
            let served = served_by_book.get(book).copied().unwrap_or(0);
            assert!(
                served >= *floor,
                "{book} serves {served} feat descriptions, below its pinned floor of {floor}"
            );
        }
    }

    /// A feat whose converted record states its words serves **those**, and the resolution is
    /// the same one `feat_description_by_exact_name` hands `class_feature_feat_bridge.rs`.
    #[test]
    fn a_feat_serves_its_converted_words_and_the_bridge_agrees() {
        let catalog = build_feat_catalog();
        let wings = catalog
            .entries
            .iter()
            .find(|e| e.key == "Angel Wings" && e.source == "Arg")
            .expect("'Angel Wings' (Arg) must be offered by the picker");
        let served = wings.description.as_deref().expect("Angel Wings states words");
        assert_eq!(
            crate::converted_prose::description_for("advanced_race_guide", "feat", "Angel Wings")
                .as_deref(),
            Some(served),
            "the picker must serve the converted record's own words"
        );
        assert_eq!(
            feat_description_by_exact_name("Angel Wings").as_deref(),
            Some(served),
            "the class-feature bridge must not promise different text than the picker ships"
        );
    }

    /// The refusal is live: a stored string still carrying an unresolved marker is not served
    /// half-rendered when the converted package holds nothing for the row.
    #[test]
    fn an_unresolved_stored_string_is_refused_rather_than_half_rendered() {
        // A key the package holds under no rule, so the fallback is the only path.
        assert_eq!(
            row_description(RuleSetId::Crb, "Not A Real Feat At All", Some("%1 times per day")),
            None,
            "a stored string carrying an unresolved marker must be refused whole"
        );
        assert_eq!(
            row_description(RuleSetId::Crb, "Not A Real Feat At All", Some("plain english")),
            Some("plain english".to_owned()),
            "a stored string that is already the record's plain words is served"
        );
        assert_eq!(
            row_description(RuleSetId::Crb, "Not A Real Feat At All", Some("a 20% chance")),
            Some("a 20% chance".to_owned()),
            "a real English percentage is not an unresolved marker and must not be thrown away"
        );
        assert_eq!(row_description(RuleSetId::Crb, "Not A Real Feat At All", None), None);
    }

    /// Every corpus gap row reaches the served catalog, under its own book's
    /// wire `source`.
    ///
    /// This is the claim the count assertions cannot make on their own: a
    /// total moving by 83 proves 83 rows arrived somewhere, not that *these*
    /// 83 arrived, nor that they arrived attributed to the right book. Asserted
    /// per row against `build_feat_catalog()` — the same function
    /// `list_feat_catalog` and the sheet's Add Feat picker call — so a row
    /// that the projection dropped or misfiled fails here by name.
    #[test]
    fn catalog_serves_every_corpus_gap_row() {
        use codex::rules_core::rules_tables::feat_gap_tables::feat_gap_rows_for;
        use codex::rules_core::rules_tables::feats_all::hand_authored_feat_tables;

        let response = build_feat_catalog();
        let mut total = 0usize;
        for book in hand_authored_feat_tables() {
            let source = format!("{:?}", book.rule_set);
            let served: std::collections::BTreeSet<&str> = response
                .entries
                .iter()
                .filter(|e| e.source == source)
                .map(|e| e.key.as_str())
                .collect();
            for row in feat_gap_rows_for(book.rule_set) {
                total += 1;
                assert!(
                    served.contains(row.key),
                    "gap row '{}' is missing from the served catalog under source {source}",
                    row.key
                );
            }
        }
        assert_eq!(total, 649, "the feat gap lane is 649 rows (SD31-E6-F8-001's 83 + SD31-E6-F8-002's 242 + SD31-E6-F2-007's 199 Mythic Adventures rows -- SD31-W10-INTEGRATE-001 excluded 159 VISIBLE:EXPORT display-plumbing twins from the original 358 -- + SD31-E6-F8-003's 7 + SD-32 Gate 0 book-onboarding precondition's 9 inner_sea_taverns rows + SD-32 T9 onboarding's (card 11) 109: inner_sea_combat 23 + inner_sea_gods 86)");
    }

    #[test]
    fn catalog_spans_every_ingested_book_with_their_real_counts() {
        let response = build_feat_catalog();
        assert_eq!(
            response.entries.len(),
            2227,
            "1578 hand-authored (185 CRB + 172 APG + 129 ACG + 187 ARG + 17 PU + 23 UCA \
             + 104 UI + 135 UW + 261 UC + 144 UM + 221 UPsi + 0 Ce + 0 Ha + 0 Isr + 0 Oa \
             + 0 Iswg + 0 MonsterCodex + 0 Mythic + 0 Isi + 0 Botd2 + 0 InnerSeaTaverns \
             + 0 Isc + 0 Isg) \
             + 649 corpus gap rows \
             (SD31-E6-F8-001's original 83 + SD31-E6-F8-002's 242: 61 Ha, 50 Isr, 68 Oa, \
             31 Iswg, 32 MonsterCodex + SD31-E6-F2-007's 199 Mythic Adventures rows -- \
             SD31-W10-INTEGRATE-001 excluded 159 VISIBLE:EXPORT display-plumbing twins \
             from the original 358 -- + SD31-E6-F8-003's 7: inner_sea_intrigue 6, \
             book_of_the_damned_volume_2 1 + SD-32 Gate 0 book-onboarding \
             precondition's 9 inner_sea_taverns rows, this book's FIRST \
             compiled rule set of any kind + SD-32 T9 onboarding's (card 11) 109: \
             inner_sea_combat 23, inner_sea_gods 86, `decisions.md §19` PI sign-off). \
             Each per-source count below is that book's hand-authored figure \
             plus its gap rows; the rows themselves are asserted by key in \
             `catalog_serves_every_corpus_gap_row`."
        );

        let by_source =
            |source: &str| response.entries.iter().filter(|e| e.source == source).count();
        // `<hand-authored> + <corpus gap rows>` per book. `core_essentials`
        // (`Ce`) is its own real rule set (`SD31-E6-F8-001`): the shared
        // library has no hand-authored feat table of its own, but its 15
        // `ce_feats.lst` records are served as their own `source: "Ce"`
        // entries, not folded into CRB's `Crb` source -- `classify()`'s feat
        // arm resolves a `core_essentials`-directory record's engine book
        // straight to `Ce` via `source_book`, never through CRB.
        assert_eq!(by_source("Crb"), 186, "185 + 1");
        assert_eq!(by_source("Apg"), 172, "172 + 0");
        assert_eq!(by_source("Acg"), 129, "129 + 0");
        assert_eq!(by_source("Arg"), 235, "187 + 48");
        assert_eq!(by_source("Pu"), 17, "17 + 0");
        assert_eq!(by_source("Uca"), 23, "23 + 0");
        assert_eq!(by_source("Ui"), 107, "104 + 3");
        assert_eq!(by_source("Uw"), 136, "135 + 1");
        assert_eq!(by_source("Uc"), 263, "261 + 2");
        assert_eq!(by_source("Um"), 156, "144 + 12");
        assert_eq!(by_source("Ce"), 15, "0 + 15");
        assert_eq!(by_source("Upsi"), 222, "221 + 1");
        // `SD31-E6-F8-002` -- five more books already compiled for another
        // kind (race_trait and/or monster) that had no feat table at all
        // before this cycle; every one of their served entries is a gap row.
        assert_eq!(by_source("Ha"), 61, "0 + 61");
        assert_eq!(by_source("Isr"), 50, "0 + 50");
        assert_eq!(by_source("Oa"), 68, "0 + 68");
        assert_eq!(by_source("Iswg"), 31, "0 + 31");
        assert_eq!(by_source("MonsterCodex"), 32, "0 + 32");
        // `SD31-E6-F2-007` -- Mythic Adventures' first compiled rule set of
        // any kind; every served entry is a gap row (`ma_feats.lst`'s 358
        // non-`.MOD` declarations).
        assert_eq!(by_source("Mythic"), 199, "0 + 199 (SD31-W10-INTEGRATE-001: 358 - 159 VISIBLE:EXPORT twins)");
        // `SD31-E6-F8-003` -- two more books already compiled for another
        // kind that had no feat table at all before this cycle; every one
        // of their served entries is a gap row.
        assert_eq!(by_source("Isi"), 6, "0 + 6");
        assert_eq!(by_source("Botd2"), 1, "0 + 1");
        // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
        // onboarding-precondition`, AT-32-G0-003) -- Inner Sea Taverns'
        // first compiled rule set of any kind; every served entry is a gap
        // row (`istav_feats.lst`'s 9 non-`.MOD` declarations).
        assert_eq!(by_source("InnerSeaTaverns"), 9, "0 + 9");
        // SD-32 T9 onboarding (card 11), `decisions.md §19` PI sign-off --
        // already-compiled books with no feat table of their own, same
        // shape as `Ha`/`Isr`/`Oa`/`Iswg`/`MonsterCodex` above; every
        // served entry is a gap row.
        assert_eq!(by_source("Isc"), 23, "0 + 23");
        assert_eq!(by_source("Isg"), 86, "0 + 86");

        let counts = |category: &str| {
            response.entries.iter().filter(|e| e.category == category).count()
        };
        // CRB 50 + APG 69 + ACG 62 + ARG 132 + PU 2 + UI 52 + UW 77 + UC 63
        // + UM 100 + UPsi 21, and so on per category.
        // + 22 corpus gap rows + 124 SD31-E6-F8-002 gap rows (Ha 19, Isr 6,
        // Oa 62, Iswg 24, MonsterCodex 13).
        // +2 with `SD31-E6-F2-007`'s 358 Mythic Adventures rows joined on:
        // two of them (`TYPE:General`) fold to this facet.
        // +7 with `SD31-E6-F8-003`'s two more gap-lane books joined on: all
        // 7 (inner_sea_intrigue 6 + book_of_the_damned_volume_2 1) carry
        // `TYPE:General`.
        // + 7 with SD-32 Gate 0 book-onboarding precondition's 9
        // inner_sea_taverns rows joined on (7 of the 9 carry `TYPE:General`).
        // + 65 with SD-32 T9 onboarding's (card 11) 109 new rows joined on
        // (isc_abilities_feat.lst 1 + isg_abilities_feat.lst 64 -- re-derived
        // directly against the generated `feat_gap_tables.rs`).
        assert_eq!(counts("General"), 854);
        // CRB + APG + ACG + ARG 52 + UI 46 + UW 41 + UC 182 + UM 3 + UPsi 9,
        // and so on.
        // + 2 corpus gap rows + 65 SD31-E6-F8-002 gap rows (Ha 24, Isr 18,
        // Iswg 7, MonsterCodex 16).
        // + 1 with SD-32 Gate 0 book-onboarding precondition's 9
        // inner_sea_taverns rows joined on (`Implacable`, `TYPE:Combat`).
        // + 42 with SD-32 T9 onboarding's (card 11) 109 new rows joined on
        // (isc_abilities_feat.lst 20 + isg_abilities_feat.lst 22).
        assert_eq!(counts("Combat"), 692);
        // Brand-new category, present ONLY as SD-32 T9 onboarding gap rows
        // (card 11): `isc_abilities_feat.lst`'s 2 `TYPE:Monstrous Mount`
        // records (`Monstrous Mount`, `Monstrous Mount Mastery`).
        assert_eq!(counts("Monstrous Mount"), 2, "inner_sea_combat's own TYPE:Monstrous Mount feats");
        // + UW 1 + UM 2 + UPsi 3.
        // + 1 corpus gap row (`Craft Construct`, via core_essentials).
        assert_eq!(counts("ItemCreation"), 15);
        // + UI 4 + UW 2 + UM 9.
        // + 7 SD31-E6-F8-002 gap rows (Oa 6, Ha 1).
        // +1 with `SD31-E6-F2-007`'s 358 Mythic Adventures rows joined on
        // (`Ascendant Spell`, `TYPE:Metamagic`).
        assert_eq!(counts("Metamagic"), 59);
        // + UI 2 + UW 3 + UC 7 + UM 1.
        // + 29 SD31-E6-F8-002 gap rows (Isr 26, MonsterCodex 3).
        // + 1 with SD-32 Gate 0 book-onboarding precondition's 9
        // inner_sea_taverns rows joined on (`Drinking Buddy`, `TYPE:Teamwork`).
        assert_eq!(counts("Teamwork"), 53);
        assert_eq!(counts("Panache"), 4);
        // PU's three `###Block:`-derived categories; no other book has them.
        assert_eq!(counts("Alignment"), 9);
        assert_eq!(counts("CombatStamina"), 3);
        assert_eq!(counts("WoundThreshold"), 3);
        // UCA's single corpus-derived category -- all 23 records are
        // `TYPE:Story`.
        // + 4 SD31-E6-F8-002 gap rows (horror_adventures, its own
        // `TYPE:Story` feats).
        assert_eq!(counts("Story"), 27);
        // `SD31-E6-F8-002` -- two brand-new categories, present ONLY as gap
        // rows (`category` is the corpus `TYPE:` token's first dot-segment
        // verbatim, never remapped to an existing book's enum spelling, so
        // `Item Creation` (with a space) is a distinct string from the
        // hand-authored tables' own `ItemCreation`).
        assert_eq!(counts("Monster"), 12, "horror_adventures' own TYPE:Monster feats");
        assert_eq!(counts("Item Creation"), 1, "horror_adventures, one TYPE:Item Creation feat");
        // UW's own new category -- Companion/animal-focused feats. No
        // other book carries this facet.
        assert_eq!(counts("Animal"), 11);
        // UC's own new categories -- Gunslinger Grit, and its own bare
        // `Critical`/`Style`/`Called Shot` facets, distinct from the
        // `Combat.*` sub-facets that fold to `Combat`. `UcPanache` never
        // appears (0 records; see `ultimate_combat::feat_tables`'s own
        // doc comment). UC + UM both carry a bare `Critical` facet
        // (1 + 3 = 4).
        assert_eq!(counts("Grit"), 7);
        assert_eq!(counts("CalledShot"), 2);
        assert_eq!(counts("Critical"), 4);
        assert_eq!(counts("Style"), 1);
        // UM's own new categories -- Bard `Masterpiece` performance feats
        // and Wizard `Discovery`-as-feat records. No other book carries
        // either facet.
        assert_eq!(counts("Masterpiece"), 15);
        assert_eq!(counts("Discovery"), 11);
        // UPsi's own new categories -- `Psionic` (this book's dominant
        // facet) and `Metapsionic` (its metamagic equivalent). No other
        // book carries either facet.
        assert_eq!(counts("Psionic"), 153);
        assert_eq!(counts("Metapsionic"), 35);
        // Mythic Adventures' own new categories (`SD31-E6-F2-007`) --
        // `category` is the corpus `TYPE:` token's first dot-segment
        // verbatim, same rule as `SD31-E6-F8-002`'s gap-row facets below.
        // `Mythic` is the book's dominant facet (160 of 199, after
        // SD31-W10-INTEGRATE-001 excluded 159 VISIBLE:EXPORT display-
        // plumbing twins from the raw 358 -- every excluded twin was itself
        // `TYPE:Mythic`, so the whole reduction lands on this one facet);
        // `Mythic Racial Heritage` is its 34 racial mythic-boost feats,
        // unaffected (no twin was `TYPE:Mythic Racial Heritage`); the other
        // three sit at 1 each (`SpecialAttack`: `Marked for Glory Output`;
        // `Metamagic`: `Ascendant Spell`, counted above; `Familiar Class
        // Feature`: a single familiar-boosting mythic feat).
        assert_eq!(counts("Mythic"), 160);
        assert_eq!(counts("Mythic Racial Heritage"), 34);
        assert_eq!(counts("SpecialAttack"), 1);
        assert_eq!(counts("Familiar Class Feature"), 1);

        // The corpus gap rows' own facets. Unlike every category above,
        // these are the corpus `TYPE:` token's first dot-segment verbatim
        // rather than a per-book `FeatCategory` variant name — a gap row has
        // no per-book table to take a variant from (see `feat_gap_tables`'
        // module doc). 58 rows across 16 facets; the other 25 gap rows fall
        // into `General`/`Combat`/`ItemCreation` above.
        assert_eq!(counts("AngelicFleshOption"), 4);
        assert_eq!(counts("BloodDrinkerType"), 12);
        assert_eq!(counts("CatfolkExemplarOption"), 3);
        assert_eq!(counts("DragonShamanBonus"), 1);
        assert_eq!(counts("Extraordinary"), 2);
        assert_eq!(counts("HeavenlyRadianceOption"), 5);
        // `TYPE:Internal`, not the `CATEGORY:Internal` bookkeeping shape the
        // enumerator excludes — these are real records carrying an Internal
        // type facet, and `v06_work_inventory` counts them as units for the
        // same reason.
        assert_eq!(counts("Internal"), 2);
        assert_eq!(counts("Kobold Scale Color"), 5);
        assert_eq!(counts("MultitalentedMasteryBonus"), 2);
        assert_eq!(counts("OrcWeaponExpertise"), 6);
        assert_eq!(counts("SaurianShamanBonus"), 1);
        assert_eq!(counts("SharkShamanBonus"), 1);
        assert_eq!(counts("Special"), 1);
        assert_eq!(counts("SpecialQuality"), 4);
        assert_eq!(counts("Supernatural"), 6);
        assert_eq!(counts("Umbral Scion Spell"), 3);

        let categorised: usize = [
            "General",
            "Combat",
            "ItemCreation",
            "Metamagic",
            "Teamwork",
            "Panache",
            "Alignment",
            "CombatStamina",
            "WoundThreshold",
            "Story",
            "Animal",
            "Grit",
            "CalledShot",
            "Critical",
            "Style",
            "Masterpiece",
            "Discovery",
            "Psionic",
            "Metapsionic",
            // Mythic Adventures' own new categories (`SD31-E6-F2-007`).
            "Mythic",
            "Mythic Racial Heritage",
            "SpecialAttack",
            "Familiar Class Feature",
            // corpus gap-row facets
            "AngelicFleshOption",
            "BloodDrinkerType",
            "CatfolkExemplarOption",
            "DragonShamanBonus",
            "Extraordinary",
            "HeavenlyRadianceOption",
            "Internal",
            "Kobold Scale Color",
            "MultitalentedMasteryBonus",
            "OrcWeaponExpertise",
            "SaurianShamanBonus",
            "SharkShamanBonus",
            "Special",
            "SpecialQuality",
            "Supernatural",
            "Umbral Scion Spell",
            // `SD31-E6-F8-002`'s five-book gap lane -- two brand-new
            // categories, present only as gap rows.
            "Monster",
            "Item Creation",
            // SD-32 T9 onboarding (card 11) -- brand-new category, present
            // only as `inner_sea_combat`'s own gap rows.
            "Monstrous Mount",
        ]
        .iter()
        .map(|category| counts(category))
        .sum();
        assert_eq!(
            categorised,
            response.entries.len(),
            "every served record must carry one of the categories asserted above"
        );
    }

    /// The point of widening the aggregate to ARG and PU: a player opening
    /// the Feat picker can now see and select those books' real feats,
    /// with their real corpus descriptions.
    #[test]
    fn real_arg_and_pu_feats_reach_the_picker_with_their_descriptions() {
        let response = build_feat_catalog();
        let find = |key: &str, source: &str| {
            response
                .entries
                .iter()
                .find(|e| e.key == key && e.source == source)
                .unwrap_or_else(|| panic!("'{key}' ({source}) must be offered by the picker"))
        };

        let wings = find("Angel Wings", "Arg");
        assert_eq!(wings.category, "General");
        // The converted record's own words, and the gain this swap made visible: until
        // `AT-35-E6-003` cycle 8 this row served only the compiled table's first sentence,
        // because that is all the stored string held. The converter joins the record's
        // descriptive fields, so the player now reads the feat's actual benefit as well.
        assert_eq!(
            wings.description.as_deref(),
            Some(
                "Feathered wings sprout from your back.\nYou gain a pair of gleaming feathered \
                 wings that grant a fly speed of 30 feet (average maneuverability) if wearing \
                 light armor or unencumbered, or 20 feet (poor maneuverability) with a medium or \
                 heavy load or medium or heavy armor. Fly is a class skill for you."
            )
        );

        let champion = find("Champion of Tyranny", "Pu");
        assert_eq!(champion.category, "Alignment");
        assert_eq!(
            champion.description.as_deref(),
            Some("You must beat down the masses to have true order.")
        );
    }

    /// The catalog's one cross-book duplicate key. Both rows are served,
    /// because dropping PU's would make this response disagree with that
    /// book's own table about how many feats it has, and `source` plus
    /// `category` are what tell a player which listing they are looking
    /// at. The description is deliberately asserted *equal*: PU re-lists
    /// the Core Rulebook feat rather than defining a new one, so identical
    /// text here is the corpus being reported faithfully, not one record
    /// shadowing the other.
    #[test]
    fn both_endurance_listings_are_served_and_are_distinguishable() {
        let response = build_feat_catalog();
        let endurance: Vec<_> = response.entries.iter().filter(|e| e.key == "Endurance").collect();
        // `SD31-E6-F2-007` -- a third listing now exists, Mythic Adventures'
        // own mythic upgrade. Its prerequisite is the Core Rulebook feat of
        // the same name, which is the mechanical proof this is a real
        // variant rather than a coincidental name clash --
        // `feats_all::tests::cross_book_key_collisions_are_exactly_the_known_set`.
        assert_eq!(endurance.len(), 3, "CRB lists Endurance, PU re-lists it, Mythic upgrades it");

        let sources: Vec<&str> = endurance.iter().map(|e| e.source.as_str()).collect();
        assert_eq!(sources, vec!["Crb", "Pu", "Mythic"]);
        assert_eq!(endurance[0].category, "General");
        assert_eq!(endurance[1].category, "WoundThreshold");
        assert_eq!(endurance[2].category, "Mythic");
        // Deliberately **not** asserted equal any more, and the change is the point. Until
        // `AT-35-E6-003` cycle 8 both rows served the same stored string, so the two listings
        // read identically. The converted package holds one record per book: Core Rulebook's
        // Endurance and Pathfinder Unchained's Wound Threshold re-listing state different
        // benefits, and each row now serves its own book's. Both are real text; neither shadows
        // the other, which is what this test has always been for.
        assert!(endurance[0].description.is_some(), "CRB's Endurance states its own words");
        assert!(endurance[1].description.is_some(), "PU's re-listing states its own words");
        assert_ne!(
            endurance[0].description, endurance[1].description,
            "each book's listing must serve its OWN record's words, not a shared string"
        );
        assert!(
            endurance[2].description.is_some(),
            "the Mythic listing states its own words too"
        );
    }

    /// PU's block-derived categories are real filter values, not labels
    /// that render and select nothing.
    #[test]
    fn filter_feat_catalog_narrows_to_a_pu_only_category() {
        let response = filter_feat_catalog(&FeatCatalogFilter {
            name_contains: None,
            category: Some("Alignment".to_owned()),
            source: None,
        });

        assert_eq!(response.entries.len(), 9, "the 9 'Champion of ...' feats");
        for entry in &response.entries {
            assert_eq!(entry.source, "Pu");
            assert!(entry.name.starts_with("Champion of "), "{:?}", entry.name);
        }
    }

    /// The point of the whole ingest: a player opening the Feat picker can
    /// now see and select real APG and ACG feats, with their real
    /// descriptions, not just CRB's 185.
    #[test]
    fn real_apg_and_acg_feats_reach_the_picker_with_their_descriptions() {
        let response = build_feat_catalog();
        let find = |key: &str| {
            response
                .entries
                .iter()
                .find(|e| e.key == key)
                .unwrap_or_else(|| panic!("'{key}' must be offered by the picker"))
        };

        let extra_hex = find("Extra Hex");
        assert_eq!(extra_hex.source, "Apg");
        assert_eq!(extra_hex.category, "General");
        assert_eq!(extra_hex.description.as_deref(), Some(EXTRA_HEX_CONVERTED_WORDS));

        let allied = find("Allied Spellcaster");
        assert_eq!(allied.source, "Apg");
        assert_eq!(allied.category, "Teamwork");

        let extra_panache = find("Extra Panache");
        assert_eq!(extra_panache.source, "Acg");
        assert_eq!(extra_panache.category, "Panache");
        assert_eq!(
            extra_panache.description.as_deref(),
            Some(
                "You have more panache than the ordinary swashbuckler.\nYou gain two more \
                 panache points at the start of each day, and your maximum panache increases by \
                 two.\n Special: If you have levels in the swashbuckler class, you can take this \
                 feat multiple times. Its effects stack."
            )
        );

        // A CRB feat is still there and still tagged CRB.
        assert_eq!(find("Power Attack").source, "Crb");
    }

    #[test]
    fn filter_feat_catalog_narrows_to_one_book() {
        let response = filter_feat_catalog(&FeatCatalogFilter {
            name_contains: None,
            category: None,
            source: Some("Acg".to_owned()),
        });
        assert_eq!(response.entries.len(), 129);
        for entry in &response.entries {
            assert_eq!(entry.source, "Acg");
        }
    }

    #[test]
    fn every_entry_has_a_non_empty_key_and_name() {
        let response = build_feat_catalog();
        for entry in &response.entries {
            assert!(!entry.key.is_empty());
            assert!(!entry.name.is_empty());
        }
    }

    /// The proof case for `class_feature_feat_bridge.rs`: a real, exact
    /// name lookup returns the SAME text `map_catalog_entry` would render
    /// for that record — proving reuse, not reinvention.
    #[test]
    fn feat_description_by_exact_name_finds_a_real_feat() {
        let extra_hex = feat_description_by_exact_name("Extra Hex");
        let catalog_entry = build_feat_catalog()
            .entries
            .into_iter()
            .find(|e| e.name == "Extra Hex")
            .expect("Extra Hex must be in the catalog");
        assert_eq!(extra_hex, catalog_entry.description);
        assert_eq!(extra_hex.as_deref(), Some(EXTRA_HEX_CONVERTED_WORDS));
    }

    /// No fuzzy matching, ever — a name one character off must not resolve.
    #[test]
    fn feat_description_by_exact_name_refuses_a_near_miss() {
        assert_eq!(feat_description_by_exact_name("Extra Hexes"), None);
        assert_eq!(feat_description_by_exact_name("extra hex"), None);
        assert_eq!(feat_description_by_exact_name(""), None);
        assert_eq!(feat_description_by_exact_name("Not A Real Feat Name At All"), None);
    }

    /// The multi-book collision precedent (`both_endurance_listings_are_
    /// served_and_are_distinguishable`, above): CRB's own `Endurance` wins,
    /// deterministically, the same book-order tie-break `all_feat_tables()`
    /// already fixes.
    #[test]
    fn feat_description_by_exact_name_is_deterministic_on_a_cross_book_collision() {
        let endurance = feat_description_by_exact_name("Endurance");
        let crb_entry = build_feat_catalog()
            .entries
            .into_iter()
            .find(|e| e.name == "Endurance" && e.source == "Crb")
            .expect("CRB's Endurance must be in the catalog");
        assert_eq!(endurance, crb_entry.description);
    }

    #[test]
    fn filter_feat_catalog_with_no_filter_fields_returns_the_full_catalog() {
        let response = filter_feat_catalog(&FeatCatalogFilter::default());
        assert_eq!(response.entries.len(), build_feat_catalog().entries.len());
    }

    #[test]
    fn filter_feat_catalog_matches_name_contains_case_insensitively() {
        let response = filter_feat_catalog(&FeatCatalogFilter {
            name_contains: Some("dodge".to_owned()),
            category: None,
            source: None,
        });

        assert!(!response.entries.is_empty(), "the real CRB corpus has a Dodge feat");
        assert!(response.entries.len() < build_feat_catalog().entries.len());
        for entry in &response.entries {
            assert!(entry.name.to_lowercase().contains("dodge"), "{:?}", entry.name);
        }
    }

    #[test]
    fn filter_feat_catalog_matches_category_exactly() {
        let response = filter_feat_catalog(&FeatCatalogFilter {
            name_contains: None,
            category: Some("Metamagic".to_owned()),
            source: None,
        });

        // 17 CRB + 19 APG + 4 UI + 2 UW + 9 UM; ACG, ARG, PU and UCA have
        // no Metamagic feat records. + 7 corpus gap rows (SD31-E6-F8-002:
        // occult_adventures 6, horror_adventures 1). +1 with
        // `SD31-E6-F2-007`'s Mythic `Ascendant Spell` (`TYPE:Metamagic`).
        assert_eq!(response.entries.len(), 59);
        for entry in &response.entries {
            assert_eq!(entry.category, "Metamagic");
        }
    }

    #[test]
    fn filter_feat_catalog_combines_name_and_category_filters() {
        let response = filter_feat_catalog(&FeatCatalogFilter {
            name_contains: Some("spell".to_owned()),
            category: Some("Metamagic".to_owned()),
            source: None,
        });

        assert!(
            !response.entries.is_empty(),
            "the real CRB corpus has known metamagic feats with 'spell' in the name (e.g. Still Spell)"
        );
        for entry in &response.entries {
            assert_eq!(entry.category, "Metamagic");
            assert!(entry.name.to_lowercase().contains("spell"));
        }
    }
}

/// The real corpus weapon list, for the "which weapon?" step of adding a
/// chooser feat.
///
/// Sourced from `rules_tables::crb::weapon_tables::WEAPON_TABLE` -- the same
/// 106 ingested records the per-weapon attack/damage/threat-range totals are
/// computed from. Deliberately NOT the arms-and-armor equipment catalog:
/// that mixes armor and shields in, and offering "Chain Shirt" as a Weapon
/// Focus target would let a player record a choice no producer can honour.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponTargetDto {
    /// The weapon's corpus key, which is what a chooser feat's target names.
    pub key: String,
    /// e.g. `"1d8 · threat 19-20/x2"` -- enough to tell similar weapons apart
    /// in the picker without opening anything.
    pub detail: String,
}

pub fn build_weapon_target_list() -> Vec<WeaponTargetDto> {
    use codex::rules_core::rules_tables::crb::weapon_tables::{
        weapon_critical_threat_low, WEAPON_TABLE,
    };

    WEAPON_TABLE
        .iter()
        .map(|entry| WeaponTargetDto {
            key: entry.key.to_string(),
            detail: format!(
                "{} · threat {}-20/x{}",
                entry.damage_die,
                weapon_critical_threat_low(entry),
                entry.critical_multiplier
            ),
        })
        .collect()
}

#[tauri::command]
pub fn list_weapon_targets() -> Vec<WeaponTargetDto> {
    build_weapon_target_list()
}

#[cfg(test)]
mod weapon_target_tests {
    use super::*;

    #[test]
    fn the_weapon_target_list_is_the_real_ingested_table() {
        let targets = build_weapon_target_list();
        assert!(targets.len() > 100, "expected the full ingested table, got {}", targets.len());
        let longsword = targets
            .iter()
            .find(|t| t.key == "Longsword")
            .expect("Longsword must be offerable as a target");
        assert_eq!(longsword.detail, "1d8 · threat 19-20/x2");
    }

    /// Body armor must never appear -- a Weapon Focus target naming a Chain
    /// Shirt could be recorded and would then ground nothing.
    ///
    /// **Shields deliberately DO appear, and that is correct.** A shield
    /// bash is a real PF1 attack: the corpus gives `Shieldbash (Heavy
    /// Shield)` its own `1d4`/x2 record and Martial proficiency, so Weapon
    /// Focus (Heavy Steel Shield) is a legitimate build. An earlier version
    /// of this test asserted no key could contain "Shield" and failed --
    /// the data was right and the assertion was wrong.
    #[test]
    fn body_armor_is_not_offered_as_a_weapon_target_but_shields_are() {
        let targets = build_weapon_target_list();
        for armor in ["Chain Shirt", "Breastplate", "Full Plate", "Leather Armor"] {
            assert!(
                !targets.iter().any(|t| t.key.contains(armor)),
                "{armor} is not a weapon and must not be offerable as a target"
            );
        }
        assert!(
            targets.iter().any(|t| t.key.contains("Shieldbash")),
            "shield bash is a real weapon and must remain offerable"
        );
    }
}
