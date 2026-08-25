//! The corpus equipment gap lane: every `equipment`/`equipment_modifier`
//! record belonging to an already-compiled book that no hand-authored
//! per-book table holds.
//!
//! These tests pin the properties that make the lane real rather than
//! decorative: the rows reach `equipment_catalog_rows()` (the one function
//! both the desktop catalog and the headless cost resolver read), they add a
//! book the catalog could not serve at all before, they never fabricate a
//! price, and — the non-regression that matters most — every key the
//! hand-authored tables already answered still resolves to the identical
//! hand-authored row at the identical price.

use codex::rules_core::equipment_resolver::{
    equipment_catalog_row_by_key, equipment_catalog_rows, hand_authored_equipment_rows,
    EQUIPMENT_BOOK_UW,
};
use codex::rules_core::rules_tables::equipment_gap_tables::equipment_gap_rows;

/// The generator's own per-book output, re-derived here from the table rather
/// than transcribed from its stdout. Each figure is that book's
/// `not-ingested` `equipment` + `equipment_modifier` unit count in
/// `docs/work-inventory.json` at the time this lane landed.
const EXPECTED_PER_BOOK: &[(&str, usize)] = &[
    // `SD31-E6-F10-002`, `decisions.md §9`: 3 of CRB's original 335 gap
    // rows (`Rock (Small)`, `Rock (Medium)`, `Poison (Violet Venom)`)
    // moved to `"B1"` -- both source `.lst` files (under the shared
    // `core_essentials` library) carry an uncommented `SOURCELONG:Bestiary`
    // header, so the re-attribution rule Decision 9 already applies to
    // `monster_ability`/`race_trait`/`race` reaches these 3 equipment rows
    // too. 332 + 3 = 335, the total is unchanged.
    ("CRB", 332),
    ("B1", 3),
    ("APG", 37),
    ("ACG", 50),
    ("ARG", 15),
    ("UC", 20),
    ("UI", 7),
    // `SD31-E6-F10-003`: 65 -> 64. Extending this generator's own
    // `declared_pi_at` check over the full compiled table (built for the 8
    // new books below) caught a genuine, pre-existing PI leak this cycle
    // did not introduce: `ultimate_equipment:"Elysian Shield"` declares
    // `NAMEISPI:YES` in the real corpus and was shipping unscreened here.
    // SD-32 `decisions.md §24`: 64 -> 65. The declared-PI row above is no
    // longer excluded whole -- it is INCLUDED under a Codex-generated
    // neutral name (`name_pi_citation` carries its real citation).
    ("UE", 65),
    ("UPSI", 113),
    ("UW", 127),
    // `SD31-E6-F10-003`: 8 further already-compiled books extended into the
    // gap lane, same "no hand-authored table" shape as `UW` above. Each
    // figure is that book's `not-ingested` `equipment` + `equipment_
    // modifier` population net of its own declared-PI exclusions (12
    // corpus-wide this cycle; `inner_sea_races` -1, `inner_sea_world_guide`
    // -7, `bestiary_4` -3, `ultimate_equipment` -1 above) and, for `B2`/
    // `B3`, net of 1 bare PFS organized-play legality OVERLAY row each
    // (`is_non_record_line`'s `PFSNotLegal` extension): `bestiary_2/_pfs/
    // pfs_b2_equip_arms_armor.lst`'s bare `Maul of the Titans` row (no
    // `KEY:` of its own) and `bestiary_3/_pfs/pfs_b3_equip_arms_armor.lst`'s
    // bare `Ranged Cannon` row were each shipping as a spurious SECOND
    // catalog entry citing the SAME book's real, differently-`KEY:`-ed
    // declaration (`Elysian Maul of the Titans`, `Ranged Cannon ~
    // Clockwork Goliath`) -- a genuine `record_key`/cited-line mismatch
    // `tests/v06_corpus_trap_report.rs`'s
    // `ingested_record_keys_match_their_cited_line` caught.
    ("OA", 119),
    ("HA", 117),
    // SD-32 `decisions.md §24`: 71 -> 72 (`inner_sea_races`), 46 -> 53
    // (`inner_sea_world_guide`, +7), 5 -> 8 (`bestiary_4`, +3) -- each
    // book's declared-PI/blacklist exclusions are no longer excluded
    // whole; they are INCLUDED under a Codex-generated neutral name.
    ("ISR", 72),
    ("ISWG", 53),
    ("MC", 49),
    ("B2", 7),
    ("B3", 8),
    ("B4", 8),
    // `SD31-E6-F10-004`: 5 further already-compiled books, the ones
    // `SD31-E6-F10-003` deliberately left out of the batch above because
    // their real corpus text hit `screen_generated_table`'s whole-file
    // blacklist hard stop (`OPEN-ISSUES.md` row 186). Reachable now that a
    // per-record `blacklist_hit` pre-filter excludes/redacts only the
    // individual offending rows -- the whole-file hard stop is unchanged
    // and still runs over the finished table (0 hits, confirmed). Each
    // figure is that book's row count after both the declared-PI reader
    // (`ISG` -2, `MYTHIC` -4, no hits for the other 3) and the new
    // blacklist screen (9 name/key exclusions corpus-wide across the 5) net
    // out.
    // SD-32 `decisions.md §24`: each book's declared-PI/blacklist name
    // exclusions are no longer excluded whole -- they are INCLUDED under a
    // Codex-generated neutral name (`name_pi_citation` carries the real
    // citation). 125 -> 150 (`inner_sea_gods`, +25), 252 -> 255
    // (`mythic_adventures`, +3), 65 -> 72 (`inner_sea_combat`, +7),
    // 34 -> 42 (`inner_sea_intrigue`, +8), 5 -> 6
    // (`book_of_the_damned_volume_2`, +1).
    ("ISG", 150),
    ("MYTHIC", 255),
    ("ISC", 72),
    ("ISI", 42),
    ("BOTD2", 6),
    // SD-32 T9 onboarding (card 11), `decisions.md §19` PI sign-off: two
    // more already-compiled books extended into the gap lane. Both figures
    // are that book's `not-ingested` equipment population, re-derived
    // directly against `fresh_inventory.json`, net of the generator's own
    // declared-PI/blacklist screens (0 hits for either book -- confirmed
    // via this generator's own stdout at the pinned oracle).
    ("ISTEM", 43),
    // SD-32 T9 residual (`decisions.md §20`): 6 -> 68. `cache_gen::equipment_
    // gap`'s `book_routing` had no arm for `"ISM"` (nor `"ISTEM"`/`"AG"`
    // above/below) at all -- the config table generated these rows but the
    // cache writer's `let Some(..) = book_routing(book) else { continue }`
    // silently dropped every one before it ever reached `data/corpus/`.
    // Fixed in `cache_gen::equipment_gap::book_routing`. Separately,
    // `ism_equipmods.lst` was deliberately left out of this book's citation
    // files on a stale "zero not-ingested equipment units for that file"
    // claim; re-derived against the pinned oracle, 62 `not-ingested`
    // `equipment_modifier` units cite it. Both fixed together: 6 + 62 = 68.
    ("ISM", 68),
    // SD-32 T9 residual: `adventurers_guide` had no `BOOK_INPUT` entry at
    // all before this cycle -- the single largest un-covered `equipment`
    // population (115 `not-ingested` units, re-derived against the pinned
    // oracle). 97 of the 115 resolve to a real citation and clear PI
    // screening; the remainder are unresolved citations or PI exclusions,
    // both reported by the generator's own stdout, not fabricated to close
    // the gap.
    // SD-32 `decisions.md §24`: 97 -> 115 (+18). The PI exclusions above
    // are no longer excluded whole -- they are INCLUDED under a
    // Codex-generated neutral name.
    // SD-32 T9 onboarding (card 11) group C, `decisions.md §20` residual:
    // `ag_equipmods.lst` was simply absent from `adventurers_guide`'s
    // `BOOK_INPUTS` `files` list -- the book's one real `equipment_modifier`
    // object ("Medium Grey Maiden Plate" ~ "Special Ability ~ Agile Maiden
    // ~ Armor") was never read at all. Added back in: 115 -> 116 (+1).
    ("AG", 116),
    // SD-32 T9 residual: `ultimate_magic` (`EQUIPMENT_BOOK_UM`, already
    // routed in the compiled catalog) had no `BOOK_INPUT` entry either, but
    // its real residual (19 `not-ingested` equipment units) turns out to be
    // status `unknown`/`ingested-magnitude` in `docs/work-inventory.json`,
    // not `status == "not-ingested"` -- this generator's own selection
    // predicate (see its module doc comment) only covers the latter, so 0
    // rows land here. The config entry is added (harmless, additive) and
    // the real residual is named as a next-cycle item rather than widening
    // this generator's predicate untested in the same cycle.
    ("UM", 0),
    // SD-32 `sd32-beginner-box-ingest` (`decisions.md §27b`): `beginner_box`
    // had no `BOOK_INPUT` entry at all -- an inherited "will not be brought
    // in" carve-out `§27b` overturns. 19 = the book's whole `equipment`-kind
    // population (`docs/work-inventory.json`), all 19 resolve to a real
    // citation and clear PI screening except one (`bbox_equip_magic_items.
    // lst:16`, a declared/blacklisted name INCLUDED under a Codex-generated
    // neutral identity per `decisions.md §24`, not excluded).
    ("BB", 19),
];

#[test]
fn the_gap_lane_carries_one_row_per_previously_not_ingested_unit() {
    let total: usize = equipment_gap_rows().count();
    let expected: usize = EXPECTED_PER_BOOK.iter().map(|(_, n)| *n).sum();
    assert_eq!(
        total, expected,
        "gap row count moved; regenerate with `cargo run --bin gen_equipment_gap_tables` and \
         re-derive the per-book figures from docs/work-inventory.json before changing them"
    );
    assert_eq!(
        total, 1973,
        "1973 = 1954 + 19 (SD-32 `sd32-beginner-box-ingest`, `decisions.md §27b`: `beginner_box` \
         had no BOOK_INPUT entry at all -- see EXPECTED_PER_BOOK's `BB` entry above). \
         1954 = 1953 + 1 (SD-32 T9 onboarding card 11 group C, `decisions.md §20` residual: \
         `ag_equipmods.lst` was absent from adventurers_guide's BOOK_INPUTS, so its one real \
         equipment_modifier object was never read; AG 115 -> 116, see EXPECTED_PER_BOOK above). \
         1953 = 1879 + 74 (SD-32 `decisions.md §24`: a declared-PI or blacklisted-name row is \
         no longer excluded from this table whole -- it is INCLUDED under a Codex-generated \
         neutral name/key, `name_pi_citation` carrying its real citation forward so `cache_gen::\
         equipment_gap` can still resolve it; 65 declared + 9 blacklist across 10 books: UE +1, \
         ISR +1, ISWG +7, B4 +3, ISG +25, MYTHIC +3, ISC +7, ISI +8, BOTD2 +1, AG +18). \
         1879 = 1720 + 159 new (SD-32 T9 residual, `decisions.md §20`: ISM's routing fix + \
         its recovered ism_equipmods.lst citations raise ISM 6 -> 68 (+62), the new AG book \
         adds 97, the new UM book adds 0). 1720 = 1671 + 49 (2 books, SD-32 T9 onboarding \
         card 11: inner_sea_temples 43 + inner_sea_magic 6) = 1190 + 481 new (5 books, \
         `SD31-E6-F10-004`) - 35 declared-PI exclusions - 9 blacklist name/key exclusions \
         (both net of redactions, which keep the record)"
    );
}

#[test]
fn every_books_gap_row_count_is_pinned() {
    for (code, expected) in EXPECTED_PER_BOOK {
        let actual = equipment_gap_rows().filter(|row| row.book == *code).count();
        assert_eq!(actual, *expected, "gap row count for book {code}");
    }
}

#[test]
fn every_gap_row_reaches_the_shared_catalog() {
    let catalog = equipment_catalog_rows();
    assert_eq!(
        catalog.len(),
        hand_authored_equipment_rows().len() + equipment_gap_rows().count(),
        "the catalog must be exactly the hand tables plus the gap rows — no row dropped, \
         none double-counted"
    );
    // Not a length check alone: a sample of real identities must be findable.
    for row in equipment_gap_rows() {
        assert!(
            catalog.iter().any(|c| c.key == row.key && c.book == row.book),
            "gap row {:?} ({}) never reached equipment_catalog_rows()",
            row.key,
            row.book
        );
    }
}

#[test]
fn ultimate_wilderness_reaches_the_catalog_only_through_the_gap_lane() {
    assert!(
        !hand_authored_equipment_rows().iter().any(|r| r.book == EQUIPMENT_BOOK_UW),
        "UW has no hand-authored equipment table; if one lands, this lane's UW rows are \
         duplicates and must be regenerated"
    );
    let uw = equipment_catalog_rows().iter().filter(|r| r.book == EQUIPMENT_BOOK_UW).count();
    assert_eq!(uw, 127, "every UW catalog row comes from the gap lane");
}

#[test]
fn widening_leaves_every_hand_authored_key_resolving_to_its_original_row() {
    // The gap rows are chained LAST precisely so first-match key lookup is
    // unchanged. Proven, not asserted: for every hand-authored key, the row
    // the shared resolver returns is a hand-authored one with the same price.
    for hand in hand_authored_equipment_rows() {
        let resolved = equipment_catalog_row_by_key(hand.key)
            .unwrap_or_else(|| panic!("hand-authored key {:?} no longer resolves", hand.key));
        // Compared by VALUE, not by pointer: `equipment_catalog_rows()` owns a
        // copied `Vec`, so every row it hands back is a distinct allocation
        // from the hand table's own — a first draft used `std::ptr::eq` and
        // failed on `Arrow (Base)` for that reason alone, not because the row
        // was wrong.
        let original = hand_authored_equipment_rows()
            .iter()
            .find(|h| h.key == hand.key)
            .expect("just iterated it");
        assert_eq!(
            resolved, original,
            "key {:?} now resolves to a different row than its first hand-authored one",
            hand.key
        );
    }
}

#[test]
fn no_gap_row_fabricates_an_identity_or_a_price() {
    for row in equipment_gap_rows() {
        assert!(!row.key.is_empty(), "empty key");
        assert!(!row.name.is_empty(), "empty name on {:?}", row.key);
        assert!(!row.book.is_empty(), "empty book on {:?}", row.key);
        assert!(
            matches!(row.category, "General" | "ArmsArmor" | "MagicItems" | "Equipmods"),
            "unknown category {:?} on {:?}",
            row.category,
            row.key
        );
        if let Some(cost) = row.cost_gp {
            assert!(cost.is_finite() && cost >= 0.0, "nonsense cost on {:?}", row.key);
        }
        if let Some(weight) = row.weight_lbs {
            assert!(weight.is_finite() && weight >= 0.0, "nonsense weight on {:?}", row.key);
        }
        if let Some(desc) = row.description {
            assert!(!desc.trim().is_empty(), "empty description on {:?}", row.key);
        }
    }
}

#[test]
fn a_real_priced_row_and_a_real_honestly_unpriced_row_are_both_present() {
    // A lane that emitted `None` for every price would pass every check above
    // while having ingested nothing usable.
    let priced = equipment_gap_rows().filter(|r| r.cost_gp.is_some()).count();
    assert!(priced > 0, "not one gap row carries a real corpus COST: token");
    let unpriced = equipment_gap_rows().filter(|r| r.cost_gp.is_none()).count();
    assert!(
        unpriced > 0,
        "every row priced is itself suspicious — PCGen formula costs must stay None, not be \
         evaluated into a fabricated flat number"
    );
}
