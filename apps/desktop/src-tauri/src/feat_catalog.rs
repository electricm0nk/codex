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
use codex::rules_core::feat_prereqs::pre_tokens::CharacterPrereqFacts;
use codex::rules_core::feat_prereqs::{evaluate_catalog_feat_prerequisites, FeatPrerequisiteReport};
use codex::rules_core::rules_tables::feats_all::all_feat_tables;

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
    /// see `feat_prereqs::pre_tokens`' three-outcome design.
    pub unverified: Vec<String>,
    /// How many `PRE`-family tokens the corpus record carries. `0` means
    /// the feat genuinely has no prerequisites.
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

fn map_catalog_entry(
    entry: &codex::rules_core::rules_tables::feats_all::FeatCatalogRecord,
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
        // Rendered, not copied. The Add Feat picker folds this straight
        // into its `detail` line (`itemPickerFilter::mapFeatCatalogEntries`),
        // so the raw corpus `DESC:` token reached the player verbatim.
        //
        // 17 of the 681 served descriptions carried PCGen syntax; partitioned
        // by their first-detected leak that is 13 an unsubstituted `%N`
        // ("%1 times per day"), 3 an undecoded `&nl;`, and ACG's
        // `Twinned Feint` a trailing
        // `|!PREABILITY:1,CATEGORY=FEAT,Improved Feint`. The remaining 664 are
        // returned byte-identical — `render_pcgen_desc` rewrites nothing in
        // prose that carries no PCGen syntax, which
        // `feat_descriptions_are_rendered_and_otherwise_byte_identical` pins.
        // `render_pcgen_desc` owns the treatment; this module does not
        // re-decide it.
        description: entry
            .description
            .map(|d| codex::rules_core::pcgen_desc::render_pcgen_desc(d).text),
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

fn build_feat_catalog_for(facts: Option<&CharacterPrereqFacts>) -> FeatCatalogResponse {
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
            map_catalog_entry(entry, &source, eligibility)
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
    facts: &CharacterPrereqFacts,
) -> FeatCatalogResponse {
    filter_feat_catalog_for(filter, Some(facts))
}

fn filter_feat_catalog_for(
    filter: &FeatCatalogFilter,
    facts: Option<&CharacterPrereqFacts>,
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

    /// The rendering `map_catalog_entry` now applies, pinned on both sides.
    ///
    /// The Add Feat picker was showing raw PCGen `DESC:` tokens. This asserts
    /// two things at once: the leaking records really are rewritten, and the
    /// rewrite touches **nothing else** — a renderer that reflowed all 681
    /// descriptions would pass a leak check while quietly changing every feat
    /// a player reads.
    #[test]
    fn feat_descriptions_are_rendered_and_otherwise_byte_identical() {
        use codex::rules_core::pcgen_desc::leaked_pcgen_syntax;
        use codex::rules_core::rules_tables::feats_all::all_feat_tables;
        use std::collections::BTreeMap;

        let raw_by_key: BTreeMap<(String, &str), &str> = all_feat_tables()
            .iter()
            .flat_map(|table| {
                table.entries.iter().filter_map(move |entry| {
                    entry
                        .description
                        .map(|d| ((format!("{:?}", table.rule_set), entry.key), d))
                })
            })
            .collect();

        let mut with_description = 0usize;
        let mut changed: Vec<&str> = Vec::new();
        let mut raw_leaks = 0usize;
        let catalog = build_feat_catalog();
        for entry in &catalog.entries {
            let Some(served) = entry.description.as_deref() else { continue };
            with_description += 1;
            let raw = raw_by_key[&(entry.source.clone(), entry.key.as_str())];
            if leaked_pcgen_syntax(raw).is_some() {
                raw_leaks += 1;
            }
            if served != raw {
                changed.push(entry.key.as_str());
            }
            assert_eq!(
                leaked_pcgen_syntax(served),
                None,
                "served feat {:?} still leaks",
                entry.key
            );
        }

        // 9 of the original 690 carry no DESC: token; UCA's 23 records all
        // carry a joined description (DESC + BENEFIT, or DESC + the
        // deferral diagnostic for the 3 corrupted records), so all 23 add
        // to `with_description` rather than the no-DESC: bucket.
        assert_eq!(with_description, 704, "9 of the 713 records carry no DESC: token");
        // 17 of the original 690 + UCA's `Battlefield Healer`, whose real
        // `.MOD BENEFIT:` row carries a literal `%%` escape ("reduce the
        // damage taken by 50%% for the purposes of..."), joined verbatim
        // into `feats_all::map_uca_entry`'s description exactly as the
        // corpus spells it -- `render_pcgen_desc` correctly collapses it to
        // one `%` for the player, the same treatment CRB's own `%%`-bearing
        // rows already get.
        assert_eq!(raw_leaks, 18, "the raw tables' own leak count, unchanged by this mapper");
        assert_eq!(changed.len(), 18, "exactly the leaking records are rewritten");
        assert_eq!(
            changed,
            vec![
                "Arcane Strike",
                "Stunning Fist",
                "Unfettered Familiar",
                "Battle Cry",
                "Befuddling Strike",
                "Dazing Fist",
                "Draining Strike",
                "Faerie's Strike",
                "Grasping Strike",
                "Gruesome Slaughter",
                "Paralyzing Strike",
                "Raging Concentration",
                "Recovered Rage",
                "Staggering Fist",
                "Twinned Feint",
                "Winter's Strike",
                "Wounded Paw Gambit",
                "Battlefield Healer",
            ]
        );
    }

    #[test]
    fn catalog_spans_every_ingested_book_with_their_real_counts() {
        let response = build_feat_catalog();
        assert_eq!(
            response.entries.len(),
            713,
            "185 CRB + 172 APG + 129 ACG + 187 ARG + 17 PU + 23 UCA"
        );

        let by_source =
            |source: &str| response.entries.iter().filter(|e| e.source == source).count();
        assert_eq!(by_source("Crb"), 185);
        assert_eq!(by_source("Apg"), 172);
        assert_eq!(by_source("Acg"), 129);
        assert_eq!(by_source("Arg"), 187);
        assert_eq!(by_source("Pu"), 17);
        assert_eq!(by_source("Uca"), 23);

        let counts = |category: &str| {
            response.entries.iter().filter(|e| e.category == category).count()
        };
        // CRB 50 + APG 69 + ACG 62 + ARG 132 + PU 2, and so on per category.
        assert_eq!(counts("General"), 315);
        assert_eq!(counts("Combat"), 302);
        assert_eq!(counts("ItemCreation"), 8);
        assert_eq!(counts("Metamagic"), 36);
        assert_eq!(counts("Teamwork"), 10);
        assert_eq!(counts("Panache"), 4);
        // PU's three `###Block:`-derived categories; no other book has them.
        assert_eq!(counts("Alignment"), 9);
        assert_eq!(counts("CombatStamina"), 3);
        assert_eq!(counts("WoundThreshold"), 3);
        // UCA's single corpus-derived category -- all 23 records are
        // `TYPE:Story`.
        assert_eq!(counts("Story"), 23);

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
        assert_eq!(
            wings.description.as_deref(),
            Some("Feathered wings sprout from your back.")
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
        assert_eq!(endurance.len(), 2, "CRB lists Endurance and PU re-lists it");

        let sources: Vec<&str> = endurance.iter().map(|e| e.source.as_str()).collect();
        assert_eq!(sources, vec!["Crb", "Pu"]);
        assert_eq!(endurance[0].category, "General");
        assert_eq!(endurance[1].category, "WoundThreshold");
        assert_eq!(
            endurance[0].description, endurance[1].description,
            "both corpus rows carry the same DESC: text"
        );
        assert!(endurance[0].description.is_some(), "and it is real text, not a shared absence");
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
        assert_eq!(
            extra_hex.description.as_deref(),
            Some("You have learned the secrets of a new hex.")
        );

        let allied = find("Allied Spellcaster");
        assert_eq!(allied.source, "Apg");
        assert_eq!(allied.category, "Teamwork");

        let extra_panache = find("Extra Panache");
        assert_eq!(extra_panache.source, "Acg");
        assert_eq!(extra_panache.category, "Panache");
        assert_eq!(
            extra_panache.description.as_deref(),
            Some("You have more panache than the ordinary swashbuckler.")
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

        // 17 CRB + 19 APG; ACG, ARG and PU have no Metamagic feat records.
        assert_eq!(response.entries.len(), 36);
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
