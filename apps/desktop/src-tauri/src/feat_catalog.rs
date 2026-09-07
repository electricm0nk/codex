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
/// Reuses the identical render treatment [`map_catalog_entry`] applies —
/// `render_pcgen_desc` then a `dropped_args`/`leaked_pcgen_syntax` double
/// refusal — rather than re-deriving it, so this function can never promise
/// cleaner text than the picker itself would ship for the same record. When
/// a name matches multiple books' feats (the catalog's own documented
/// "Endurance" collision), the first exact match in `all_feat_tables()`'s
/// own book order (CRB, APG, ACG, ARG, PU, …) wins — the SAME determinism
/// `build_feat_catalog`'s caller already relies on for that collision,
/// applied here rather than invented fresh.
///
/// Returns `None` when no book's feat carries this exact name, when the
/// matched record has no `DESC:` token of its own, or when its render would
/// itself be refused by the picker (an unresolved `%N`/raw PCGen leak) —
/// never a guessed or partial description.
pub fn feat_description_by_exact_name(name: &str) -> Option<String> {
    for book in all_feat_tables() {
        for entry in book.entries {
            if entry.name != name {
                continue;
            }
            let Some(raw) = entry.description else { continue };
            let rendered = codex::rules_core::pcgen_desc::render_pcgen_desc(raw);
            if !rendered.dropped_args.is_empty() {
                continue;
            }
            if codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&rendered.text).is_some() {
                continue;
            }
            return Some(rendered.text);
        }
    }
    None
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

        // 9 of the original 690 carry no DESC: token; UCA's 23 records and
        // UI's 104 records all carry a joined description (DESC + BENEFIT,
        // or DESC + the deferral diagnostic for UCA's 2 corrupted records),
        // so all 127 add to `with_description` rather than the no-DESC:
        // bucket.
        // +65 with the 83 corpus gap rows joined on (2026-08-11): 18 of them
        // carry neither a `DESC:` nor a `BENEFIT:` token in the corpus and
        // are served with no description rather than a fabricated one, which
        // is why this moves by 65 and not by 83.
        // +242 with `SD31-E6-F8-002`'s five-book gap lane: all 242 carry a
        // real `DESC:` and/or `BENEFIT:` token, so `with_description` moves
        // by exactly 242, not less.
        // +323 with `SD31-E6-F2-007`'s 358 Mythic Adventures rows joined on
        // (2026-08-17): 35 of them carry neither a `DESC:` nor a `BENEFIT:`
        // token in the corpus and are served with no description rather
        // than a fabricated one.
        // +7 with `SD31-E6-F8-003`'s two more gap-lane books joined on: all
        // 7 (inner_sea_intrigue 6 + book_of_the_damned_volume_2 1) carry a
        // real `DESC:` token, so `with_description` moves by exactly 7.
        // +9 with SD-32 Gate 0 book-onboarding precondition's inner_sea_taverns
        // rows joined on: all 9 carry a real `DESC:` (joined with `BENEFIT:`),
        // so `with_description` moves by exactly 9.
        // +109 with SD-32 T9 onboarding's (card 11) inner_sea_combat (23) and
        // inner_sea_gods (86) rows joined on: all 109 carry a real `DESC:`
        // token in the corpus (re-derived directly against the generated
        // `feat_gap_tables.rs`: zero `description: None` in either book's
        // static array), so `with_description` moves by exactly 109.
        assert_eq!(with_description, 2161, "66 of the 2227 records carry no served description -- 13 hand-authored plus the 18 corpus gap rows plus 35 Mythic gap rows whose records carry neither DESC: nor BENEFIT: (SD31-W10-INTEGRATE-001: 199 Mythic gap rows, not 358 -- 159 VISIBLE:EXPORT display-plumbing twins excluded, none of which lacked a description)");
        // 17 of the original 690 + UCA's `Battlefield Healer` + 10 UI
        // records: 5 carry a literal `%%` escape (`Eye for Ingredients`,
        // `Planar Wanderer`, `Structural Strike`, `Subtle Enchantments`,
        // `Superior Scryer`) and 5 carry an unsubstituted `%N` argument
        // reference plus a raw non-whitespace-bounded `|` (`Brilliant
        // Planner`, `Conceal Spell`, `Feign Curse`, `Nerve-Racking
        // Negotiator`, `Street Sweep`), joined verbatim into
        // `feats_all::map_uca_entry`/`map_ui_entry`'s description exactly
        // as the corpus spells it -- `render_pcgen_desc` correctly rewrites
        // each for the player, the same treatment CRB's own leaking rows
        // already get.
        changed.sort_unstable();
        // 185 hand-authored + 5 corpus gap rows whose own `DESC:`/`BENEFIT:`
        // text carries PCGen syntax (`Empower Spell-Like Ability ~ Ability`
        // and `~ Spell` and `Hover` carry a literal `%%` escape,
        // `Mother's Gift ~ Uncanny Resistance` an unsubstituted `%1` plus a
        // raw `|TL+6` tail, `Feral Combat Training` a raw `|`). Every one is
        // rewritten by `render_pcgen_desc` and confirmed leak-free by this
        // same test's per-record assertion above — a raw-corpus-shape count,
        // not a new player-visible leak.
        // `SD31-E6-F2-007` -- +2 with Mythic Adventures' 358 gap rows joined
        // on: two of them carry an unsubstituted PCGen syntax leak in their
        // own `DESC:`/`BENEFIT:` text, rewritten by `render_pcgen_desc`
        // exactly like every other book's leaking rows.
        assert_eq!(raw_leaks, 187, "the raw tables' own leak count (SD31-W10-INTEGRATE-001: 188 - 1, one of the two excluded VISIBLE:EXPORT Mythic twins carried a raw leak of its own -- the other's `changed` membership came from the two-field DESC/BENEFIT join differing from either raw field alone, not from a leak marker)");
        // +1 with SD-32 Gate 0 book-onboarding precondition's inner_sea_taverns
        // rows joined on: one record's own DESC/BENEFIT join differs from
        // either raw field alone (the same shape as the Mythic twin above),
        // not from a new raw leak (`raw_leaks` stays 187).
        // +2 with SD-32 T9 onboarding's (card 11) 109 new rows joined on
        // (inner_sea_combat 23 + inner_sea_gods 86): re-derived from this
        // same test's own RED-run assertion output against the pinned
        // oracle -- `raw_leaks` stays 187 (no new raw PCGen-syntax leak),
        // so both are rendering-only rewrites, not new leaks.
        // `AT-34-E3-003` bucket-U cycle 2 (2026-08-28, `36db23a053`,
        // `decisions.md §17`): 201 -> 187, -14. `render_pcgen_desc` had no
        // exemption for a bare `%` immediately preceded by a DIGIT ("75%
        // chance...") -- unlike `leaked_pcgen_syntax`'s own correct
        // digit-preceded exemption -- so it silently dropped the literal
        // percent sign, making `served` differ from a `raw` string that was
        // already clean, and wrongly counting the record as "rewritten".
        // The fix mirrors that exemption in the render path (confirmed by
        // this commit's own new unit test,
        // `pcgen_desc::tests::a_digit_preceded_percent_sign_is_a_literal_
        // sign_not_a_drop`). The 14 that dropped out of `changed`, named
        // exactly (re-derived from this test's own RED-run diff against the
        // exact-name pin below, not guessed): `Arcane Armor Training`,
        // `Empower Spell-Like Ability ~ Ability`, `~ Spell`, `Greater
        // Mesmerizing Feint`, `Hover`, `Lingering Spell-Like Ability`,
        // `Messenger Of Fate`, `Mirror Kin`, `Phantom Fortification`,
        // `Protector of the People`, `Reject Poison`, `Seeping Darkness`,
        // `Spirit Sense`, `Tavern Regular` -- every one a feat whose corpus
        // `DESC:`/`BENEFIT:` text carries a literal `N%` and nothing else
        // `render_pcgen_desc` needed to rewrite, so it now renders
        // byte-identical to its already-clean raw text.
        // `raw_leaks` above is untouched: none of these 14 was ever a real
        // leak (`leaked_pcgen_syntax` already exempted digit-preceded `%`),
        // only the render path was wrongly disagreeing with it.
        assert_eq!(changed.len(), 187, "exactly the leaking/rewritten records (AT-34-E3-003 bucket-U cycle 2, `36db23a053`: 201 - 14, the digit-preceded bare-`%` render-path defect fix)");
        // 28 pre-existing (CRB/UCA/UI) + 35 UW + 74 UC + 14 UM + 34 new
        // UPsi records. Every served description for all 185 is
        // confirmed leak-free by this same test's per-record
        // `leaked_pcgen_syntax(served) == None` assertion above -- this
        // is a raw-corpus-shape count (`&nl;` entity escapes, `%N`/raw
        // `|` tails, all correctly rewritten by `render_pcgen_desc`),
        // not a new player-visible leak.
        assert_eq!(
            changed,
            vec![
                "Access Psionic Talent",
                "Adder Strike",
                "Advanced Archer Path",
                "Advanced Ascetic Path",
                "Advanced Assassin Path",
                "Advanced Brawling Path",
                "Advanced Dervish Path",
                "Advanced Feral Path",
                "Advanced Infiltrator Path",
                "Advanced Interceptor Path",
                "Advanced Mind Knight Path",
                "Advanced Survivor Path",
                "Advanced Weaponmaster Path",
                "Amateur Gunslinger",
                "Ambush Awareness",
                "Animal Call",
                "Aquatic Combatant",
                "Arcane Strike",
                "Battle Cry",
                "Battlefield Healer",
                "Beast Hunter",
                "Beastmaster Style",
                "Befuddling Strike",
                "Binding Throw",
                "Bludgeoner",
                "Brilliant Planner",
                "Close-Quarters Thrower",
                "Clustered Shots",
                "Combat Style Master",
                "Conceal Spell",
                "Cover Tracks",
                "Crashing Wave Buffet",
                "Dazing Fist",
                "Death or Glory",
                "Deceptive Exchange",
                "Deep Diver",
                "Defensive Weapon Training",
                "Detect Expertise",
                "Dimensional Dervish",
                "Discovery (Arcane Builder)",
                "Discovery (Split Slot)",
                "Djinni Style",
                "Domain Strike",
                "Dragon Ferocity",
                "Dragon Roar",
                "Dragon Style",
                "Draining Strike",
                "Earth Child Binder",
                "Earth Child Style",
                "Earth Child Topple",
                "Eidolon Mount",
                "Empower Power",
                "Energized Wild Shape",
                "Expert Cartographer",
                "Extended Bane",
                "Extra Grit",
                "Eye for Ingredients",
                "Faerie's Strike",
                "False Trail",
                "Fear's Reach",
                "Feign Curse",
                "Feral Combat Training",
                "Field Repair",
                "Final Embrace",
                "Frightful Shape",
                "Gnome Trickster",
                "Grasping Strike",
                "Greater Beast Hunter",
                "Greater Hunter's Bond",
                "Greater Intuitive Shot",
                "Greater Spring Attack",
                "Greater Whip Mastery",
                "Gruesome Slaughter",
                "Gunsmithing",
                "Harmonic Resonance",
                "Harmonic Sage",
                "Haunted Gnome Shroud",
                "Hawkeye",
                "Hex Strike",
                "Hide Worker",
                "Horse Master",
                "Impact Critical Shot",
                "Improved Beast Hunter",
                "Improved Charging Hurler",
                "Improved Hunter's Bond",
                "Improved Monster Lore",
                "Improved Snap Shot",
                "Improved Spring Attack",
                "Indomitable Mountain Peak",
                "Instant Judgment",
                "Intuitive Shot",
                "Learn Ranger Trap",
                "Life Lure",
                "Master Siege Engineer",
                "Menacing Bane",
                "Merciful Bane",
                // SD-32 T9 onboarding (card 11): inner_sea_gods's own
                // `Messenger Of Fate` DESC/BENEFIT join differs from either
                // raw field alone (the same rendering-only shape as the
                // other gap-row entries in this list, not a new leak).
                "Modified Blast",
                "Monastic Legacy",
                "Monkey Moves",
                "Monkey Shine",
                "Monkey Style",
                "Moonlight Stalker Feint",
                "Moonlight Stalker Master",
                "Mother's Gift ~ Uncanny Resistance",
                "Mutated Shape",
                "Nerve-Racking Negotiator",
                "Net Adept",
                "Net and Trident",
                "Nightmare Weaver",
                "One Eye Open",
                "Open Door",
                "Out of the Sun",
                "Pack Attack",
                "Painful Anchor",
                "Paralyzing Strike",
                "Passing Trick",
                "Photosynthetic Healing",
                "Pinpoint Poisoner",
                "Piranha Strike",
                "Planar Wanderer",
                "Prone Slinger",
                "Prophetic Visionary",
                "Psionic Bull Rush",
                "Psionic Disarm",
                "Psionic Overrun",
                "Psionic Shield Bash",
                "Psionic Sunder",
                "Psionic Talent",
                "Psionic Trip",
                "Quick Bull Rush",
                "Quick Dirty Trick",
                "Quick Drag",
                "Quick Reposition",
                "Quick Steal",
                "Radiant Charge",
                "Raging Concentration",
                "Rapid Draw",
                "Rebuffing Reduction",
                "Recovered Rage",
                // SD-32 T9 onboarding (card 11): inner_sea_gods's own
                // `Reject Poison` DESC/BENEFIT join differs from either raw
                // field alone, same rendering-only shape as the entry above.
                "Remote Bomb",
                "Resilient Eidolon",
                "Revelation Strike",
                "Reward of Life",
                "Ricochet",
                "River Raider",
                "School Strike",
                "Scion of the Land",
                "Selective Power",
                "Shapeshifting Hunter",
                "Siege Engineer",
                "Siege Gunner",
                "Skilled Driver",
                "Slayer's Knack",
                "Sling Flail",
                "Snake Sidewind",
                "Snake Style",
                "Snap Shot",
                "Sorcerous Strike",
                "Spinning Throw",
                "Stage Combatant",
                "Staggering Fist",
                "Street Sweep",
                "Structural Strike",
                "Stunning Fist",
                "Style Feat Wildcard",
                "Subtle Enchantments",
                "Superior Scryer",
                "Sword and Pistol",
                "Thrill of the Hunt",
                "Tiger Style",
                "Toughened Suit",
                "Tree Leaper",
                "Tribal Hunter",
                "Twinned Feint",
                "Two-Handed Thrower",
                "Unfettered Familiar",
                "Unwilling Participant",
                "Urban Tracking",
                "Verdant Spell",
                "Versatile Channeler",
                "Vigilant Charger",
                "Whip Mastery",
                "Wild Growth Hex",
                "Wilding",
                "Winter's Strike",
                "Wood Crafter",
                "Wounded Paw Gambit",
            ]
        );
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
        // `SD31-E6-F2-007` -- a third listing now exists, Mythic Adventures'
        // own mythic upgrade (its `PREABILITY:...,CATEGORY=FEAT,Endurance`
        // prerequisite is the mechanical proof this is a real variant, not
        // a coincidental name clash --
        // `feats_all::tests::cross_book_key_collisions_are_exactly_the_known_set`).
        assert_eq!(endurance.len(), 3, "CRB lists Endurance, PU re-lists it, Mythic upgrades it");

        let sources: Vec<&str> = endurance.iter().map(|e| e.source.as_str()).collect();
        assert_eq!(sources, vec!["Crb", "Pu", "Mythic"]);
        assert_eq!(endurance[0].category, "General");
        assert_eq!(endurance[1].category, "WoundThreshold");
        assert_eq!(endurance[2].category, "Mythic");
        assert_eq!(
            endurance[0].description, endurance[1].description,
            "both corpus rows carry the same DESC: text"
        );
        assert!(endurance[0].description.is_some(), "and it is real text, not a shared absence");
        assert!(
            endurance[2].description.is_some(),
            "the Mythic listing has its own real DESC: text too"
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
        assert_eq!(extra_hex.as_deref(), Some("You have learned the secrets of a new hex."));
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
