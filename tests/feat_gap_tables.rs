//! The feat gap lane: every corpus `feat` record that belongs to an
//! ALREADY-COMPILED book whose hand-authored per-book feat table does not
//! hold it.
//!
//! `docs/work-inventory.json` classifies a `feat` unit `engine-does-not-hold`
//! (`evidence: "feat_key_absent_from_catalog"`) when the book has a compiled
//! rule set but `feats_all::all_feat_tables()` holds no record matching the
//! corpus record's `KEY:` **or** its display name. Closing that population
//! needs no new `RuleSetId` and no new player surface — the desktop feat
//! catalog already renders every record `all_feat_tables()` yields.
//!
//! These tests assert against the joined catalog, never against the
//! generator's stdout, so a regenerated table that dropped rows fails here.

use codex::rules_core::rules_tables::feats_all::{all_feat_tables, hand_authored_feat_tables};
use codex::rules_core::rules_tables::RuleSetId;
use std::collections::BTreeSet;

/// `RuleSetId` is deliberately not `Ord`, so every set here is keyed on its
/// `Debug` name rather than on the value — inventing an ordering for a shared
/// type to key one test's map would be the wrong direction of change.
fn rs(rule_set: RuleSetId) -> String {
    format!("{rule_set:?}")
}

/// Every `(rule_set, key)` the joined catalog holds.
fn catalog_keys() -> BTreeSet<(String, String)> {
    all_feat_tables()
        .iter()
        .flat_map(|book| book.entries.iter().map(move |e| (rs(book.rule_set), e.key.to_string())))
        .collect()
}

/// Every `(rule_set, name)` the joined catalog holds. `v06_work_inventory`'s
/// own `feat_keys` check accepts a match on EITHER the corpus key or the
/// display name, so a claim that a unit is now ingested must be tested the
/// same way.
fn catalog_names() -> BTreeSet<(String, String)> {
    all_feat_tables()
        .iter()
        .flat_map(|book| book.entries.iter().map(move |e| (rs(book.rule_set), e.name.to_string())))
        .collect()
}

fn holds(rule_set: RuleSetId, corpus_key: &str) -> bool {
    catalog_keys().contains(&(rs(rule_set), corpus_key.to_string()))
        || catalog_names().contains(&(rs(rule_set), corpus_key.to_string()))
}

/// One representative from each of the seven books this lane touches, taken
/// verbatim from `docs/work-inventory.json`'s `engine-does-not-hold` feat population
/// (`corpus_key` field) before this lane landed. Pinned by name rather than
/// by count so a regeneration that silently drops a book fails loudly.
///
/// `core_essentials` DOES have its own rule set (`RuleSetId::Ce`, added for
/// companion/familiar content and now compiled into `COMPILED_RULE_SETS`),
/// and `classify()`'s feat arm resolves a unit's `engine_book` from its
/// `source_book`, not its reporting `book` — so a `ce_feats.lst` record
/// files under `RuleSetId::Ce`, never `RuleSetId::Crb`, regardless of which
/// real-world book Decision 9's re-attribution says the record's *content*
/// belongs to (`bestiary`, in this case — see `CORE_ESSENTIALS_FEAT_GAP_ROWS`'s
/// own doc comment). `cr_feats.lst` is CRB's own file and stays filed under
/// `RuleSetId::Crb` — "Leadership Score" is genuinely from there, not from
/// `ce_feats.lst`.
const REPRESENTATIVES: &[(RuleSetId, &str)] = &[
    (RuleSetId::Crb, "Leadership Score"),
    (RuleSetId::Ce, "Craft Construct"),
    (RuleSetId::Ce, "Empower Spell-Like Ability ~ Ability"),
    (RuleSetId::Arg, "Angelic Flesh ~ Brazen"),
    (RuleSetId::Uc, "Gundarme Bonus Feat"),
    (RuleSetId::Ui, "Legendary Influence"),
    (RuleSetId::Um, "Extra Word"),
    (RuleSetId::Um, "Skill Focus Intimidate"),
    (RuleSetId::Upsi, "Feral Combat Training"),
    (RuleSetId::Uw, "Extended Animal Focus"),
    // Five books already compiled for another kind (race_trait/monster) that
    // never had a feat table at all (`SD31-E6-F8-002`) -- `feat_key_absent_
    // from_catalog` for every one of their feat units, not `no_compiled_
    // rule_set_for_book`, because `COMPILED_RULE_SETS` already names them.
    (RuleSetId::Ha, "Absorb Spirit"),
    (RuleSetId::Isr, "Alien Mindpaths"),
    (RuleSetId::Oa, "Alter Binary Mindscape"),
    (RuleSetId::Iswg, "Altitude Affinity"),
    (RuleSetId::MonsterCodex, "Angelbane Strike"),
    // `SD31-E6-F8-003` -- two more books already compiled into
    // `COMPILED_RULE_SETS` for another kind (`Isi`: familiars + abilities;
    // `Botd2`: monsters) that never had a feat table of their own. Neither
    // record carries `NAMEISPI:YES` (re-derived by direct read of
    // `isi_feats.lst`/`botd2_feats.lst`), unlike `inner_sea_world_guide`'s
    // remaining 6 `engine-does-not-hold` feats, which all do and correctly stay
    // undeliverable (SD-30 contract 53.5).
    (RuleSetId::Isi, "Convincing Persona"),
    (RuleSetId::Botd2, "Demonic Obedience"),
];

#[test]
fn every_gap_lane_representative_is_now_in_the_joined_catalog() {
    let missing: Vec<&(RuleSetId, &str)> =
        REPRESENTATIVES.iter().filter(|(rs, key)| !holds(*rs, key)).collect();
    assert!(
        missing.is_empty(),
        "these previously `engine-does-not-hold` corpus feat records are still absent \
         from `all_feat_tables()`: {missing:?}"
    );
}

/// The full `ce_feats.lst` population (`SD31-E6-F8-001`) — 15 records,
/// verbatim from `docs/work-inventory.json`'s `feat` units whose
/// `source_file == "ce_feats.lst"`. All 15 were `engine-does-not-hold`
/// (`feat_key_absent_from_catalog`) despite `feat_gap_tables.rs` shipping
/// hand-authored rows for every one of them, because the shipped rows were
/// filed under `RuleSetId::Crb` — the wrong host. `classify()`'s feat arm
/// resolves `engine_book_for(&unit.source_book)`, and `source_book` for a
/// `core_essentials`-directory record is `"core_essentials"`, which resolves
/// directly to `RuleSetId::Ce` (own_engine_book, never the CRB
/// shared-library-host fallback) — so these records were never reachable
/// through `RuleSetId::Crb`'s catalog at all, regardless of Decision 9's
/// separate re-attribution of the reporting `book` field to `bestiary`.
const CE_FEATS_LST_POPULATION: &[&str] = &[
    "Ability Focus",
    "Awesome Blow",
    "Craft Construct",
    "Empower Spell-Like Ability ~ Ability",
    "Empower Spell-Like Ability ~ Spell",
    "Flyby Attack",
    "Hover",
    "Improved Natural Armor",
    "Improved Natural Attack",
    "Multiattack",
    "Multiweapon Fighting",
    "Quicken Spell-Like Ability ~ Ability",
    "Quicken Spell-Like Ability ~ Spell",
    "Snatch",
    "Wingover",
];

#[test]
fn every_ce_feats_lst_record_resolves_under_rule_set_ce() {
    let missing: Vec<&&str> =
        CE_FEATS_LST_POPULATION.iter().filter(|key| !holds(RuleSetId::Ce, key)).collect();
    assert!(
        missing.is_empty(),
        "these ce_feats.lst records are still absent from RuleSetId::Ce's \
         joined catalog: {missing:?}"
    );
}

/// The same 15 records must NOT resolve under `RuleSetId::Crb` — a
/// record filed under the wrong host is exactly the defect this lane fixed,
/// and re-filing it under Crb would silently reopen it (Crb's catalog would
/// "hold" a key no `core_rulebook`-book unit ever carries).
#[test]
fn no_ce_feats_lst_record_is_filed_under_crb() {
    let wrongly_filed: Vec<&&str> =
        CE_FEATS_LST_POPULATION.iter().filter(|key| holds(RuleSetId::Crb, key)).collect();
    assert!(
        wrongly_filed.is_empty(),
        "these ce_feats.lst records are filed under RuleSetId::Crb, the wrong \
         host: {wrongly_filed:?}"
    );
}

#[test]
fn the_gap_rows_are_exactly_the_joined_catalog_minus_the_hand_authored_one() {
    let hand: BTreeSet<(String, String)> = hand_authored_feat_tables()
        .iter()
        .flat_map(|book| book.entries.iter().map(move |e| (rs(book.rule_set), e.key.to_string())))
        .collect();
    let joined = catalog_keys();
    assert!(
        hand.is_subset(&joined),
        "joining the gap rows must never remove a hand-authored record"
    );
    let added = joined.len() - hand.len();
    // `SD31-E6-F2-007` -- +358 with Mythic Adventures' own first-ever gap
    // lane (`ma_feats.lst`'s non-`.MOD` declarations, all 358 with distinct
    // keys, verbatim from `gen_feat_gap_tables`'s own stdout at the pinned
    // oracle). `SD31-W10-INTEGRATE-001` -- -159 excluding `VISIBLE:EXPORT`
    // display-plumbing twins (PCGen's own duplicate-for-PDF-export rows,
    // never independently selectable): 358 -> 199.
    assert_eq!(
        added, 649,
        "the gap lane is 649 rows: 540 as of `a50b7da04` (83 from the \
         original 7-book lane `SD31-E6-F8-001` + 242 from the five books \
         `SD31-E6-F8-002` added (horror_adventures 61 + inner_sea_races 50 \
         + occult_adventures 68 + inner_sea_world_guide 31 + monster_codex \
         32) + 199 from Mythic Adventures (`SD31-E6-F2-007`, 358 raw minus \
         159 `VISIBLE:EXPORT` twins excluded by `SD31-W10-INTEGRATE-001`) + \
         7 from two more already-compiled books `SD31-E6-F8-003` added \
         (inner_sea_intrigue 6 + book_of_the_damned_volume_2 1) + 9 from \
         `inner_sea_taverns`, AT-32-G0-003) + 109 from SD-32 T9 onboarding \
         (card 11, `decisions.md §19` PI sign-off): inner_sea_combat 23 \
         (24 raw minus 1 `NAMEISPI:YES` record dropped) + inner_sea_gods 86 \
         (deity-name prerequisites redacted in place by the generator's \
         existing blacklist screen, not dropped -- neither book's population \
         is the `.MOD`/`VISIBLE:EXPORT` continuation shape found blocking \
         `horror_adventures`/`mythic_adventures`, whose own `engine-does-not-hold` \
         feat counts stay at this same 61/199 pin because that population is \
         not real feat content -- see this cycle's own receipt), verbatim \
         from `gen_feat_gap_tables`'s own stdout at the pinned oracle"
    );
}

/// The single `engine-does-not-hold` feat unit this lane leaves standing, and why —
/// recorded as a named residual rather than folded into a count.
///
/// `uw_feats.lst:164` is
/// `CATEGORY=Special Ability|Samurai ~ Mount.MOD	TYPE:Mount`. A `.MOD` row is
/// an **overlay onto a record defined elsewhere**, not a new record:
/// `v06_work_inventory::enumerate_file` skips it on the normal path and
/// stashes it in `mod_targets` for corpus-wide resolution afterwards, which is
/// why it surfaces as a unit at all. Emitting a `RuleSetId::Uw` catalog row
/// for it would ship a second copy of a record whose base belongs to another
/// book — inventing a UW feat the corpus never declared — so this lane does
/// not, and the unit stays `engine-does-not-hold` with this test as its reason.
///
/// Closing it honestly means teaching the catalog to represent `.MOD` overlays
/// as modifications of their base record, which is a mechanism, not a table
/// row, and therefore outside a proven-path lane.
#[test]
fn the_one_engine_does_not_hold_feat_unit_this_lane_deliberately_does_not_close() {
    assert!(
        !holds(RuleSetId::Uw, "Samurai ~ Mount"),
        "if this record is now in the catalog, a `.MOD` overlay was ingested as \
         a first-class UW feat — check that it was a deliberate mechanism \
         change and not an accident, and update the gap-row count above"
    );
}

#[test]
fn no_gap_row_displaces_a_hand_authored_record_of_the_same_key() {
    // The hand-authored records are chained FIRST in every book's slice, so
    // a first-match key lookup keeps resolving to the hand-authored record.
    // This is the feat analogue of the equipment lane's stage-ordering
    // finding: appending to a first-match structure is only safe when the
    // original rows are exhausted first.
    for book in all_feat_tables() {
        let hand_len = hand_authored_feat_tables()
            .iter()
            .find(|b| b.rule_set == book.rule_set)
            .map(|b| b.entries.len())
            .unwrap_or(0);
        let hand_prefix = hand_authored_feat_tables()
            .iter()
            .find(|b| b.rule_set == book.rule_set)
            .map(|b| b.entries)
            .unwrap_or(&[]);
        assert!(
            book.entries.len() >= hand_len,
            "{:?} lost records when the gap rows were joined",
            book.rule_set
        );
        assert_eq!(
            &book.entries[..hand_len],
            hand_prefix,
            "{:?}'s hand-authored records must remain the slice's prefix, unchanged",
            book.rule_set
        );
    }
}

#[test]
fn every_gap_row_carries_a_real_category_and_never_an_empty_string() {
    for book in all_feat_tables() {
        for entry in book.entries {
            assert!(
                !entry.category.trim().is_empty(),
                "{:?}/{} has an empty category",
                book.rule_set,
                entry.key
            );
            assert!(
                !entry.name.trim().is_empty(),
                "{:?}/{} has an empty display name",
                book.rule_set,
                entry.key
            );
        }
    }
}

#[test]
fn no_gap_row_carries_an_empty_description_or_an_empty_prerequisite_list() {
    // `Some(&[])` and `Some("")` are the two fabricated-absence shapes every
    // per-book feat table's own doc comment forbids: a record either has the
    // token or it does not, and `None` is how "does not" is spelled.
    for book in all_feat_tables() {
        for entry in book.entries {
            if let Some(desc) = entry.description {
                assert!(
                    !desc.trim().is_empty(),
                    "{:?}/{} carries an empty description instead of None",
                    book.rule_set,
                    entry.key
                );
            }
            if let Some(pres) = entry.prerequisites {
                assert!(
                    !pres.is_empty(),
                    "{:?}/{} carries Some(&[]) instead of None",
                    book.rule_set,
                    entry.key
                );
            }
        }
    }
}
