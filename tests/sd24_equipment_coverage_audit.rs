//! SD-24 Epic 6 criterion 6.1 — Equipment/armor/spell coverage audit
//! (read-only). Full PF1 core rules (CRB) + APG + ACG corpus, for the
//! `cost` / `weight` / `description` / `full spell text` fields per
//! `epic-breakdown.md`'s Epic 6 purpose statement and
//! `technical-design.md §4.1`.
//!
//! This is the standing regression test behind
//! `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_6/equipment-coverage-matrix.md`:
//! it makes the audit's numeric claims (record coverage vs. the real
//! PCGen corpus, field presence) executable and checked on every run,
//! rather than a one-off hand count that could silently drift from the
//! code, mirroring criteria 4.1-4.3's own `coverage_report()` audit
//! pattern.
//!
//! RED -> GREEN evidence (recorded in this cycle's receipt): before
//! `rules_tables::{crb,apg,acg}::equipment_tables::field_coverage_report`
//! and `rules_tables::{crb,apg,acg}::spell_list::spell_coverage_report`
//! existed, this file did not compile (RED -- no such items). Adding
//! those small, real (every field computed from the live table, or a
//! documented corpus record count derived from the real PCGen LST files
//! under `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/`)
//! coverage-report APIs turned it GREEN.
//!
//! **Bestiary 1 is not covered by this test file.** No
//! `rules_tables::beastiary1::equipment_tables` module exists at all (the
//! `beastiary1` book module only carries monster stat blocks, per its own
//! `mod.rs` doc comment) -- there is no live code to call. This is
//! documented as a finding in the coverage-matrix artifact and this
//! cycle's receipt, not asserted here, per the same "no file to touch"
//! posture criterion 4.2's audit used for APG's missing `level_up`
//! modules.

use codex::rules_core::rules_tables::{acg, apg, crb};

/// CRB's equipment corpus (`cr_equip_arms_armor.lst` + `cr_equip_general.lst`
/// + `cr_equip_magic_items.lst` + `cr_equipmods.lst`) is **fully record-ingested**
/// -- each per-category module's own doc comment already documents an
/// exact post-SD-17-merge-fix record count (310 + 453 + 1556 + 658 =
/// 2977), matching this test's independently-computed total. The
/// `equipment_tables.rs` module-level doc comment's "one representative
/// item per category" framing is therefore stale prose predating that
/// full ingest -- a real gap, but a documentation gap, not a data gap
/// (see this cycle's `## DISCOVERED` entry).
#[test]
fn crb_equipment_is_fully_record_ingested() {
    let report = crb::equipment_tables::field_coverage_report();
    assert_eq!(
        report.records_expected, 2977,
        "documented post-merge-fix CRB equipment record count (310+453+1556+658)"
    );
    assert_eq!(
        report.total_records, report.records_expected,
        "CRB equipment record coverage should be 100% (already fully ingested)"
    );
}

/// The audit's headline finding for criterion 6.1: `EquipmentTableEntry`
/// (CRB, APG, and ACG all share the same shape) has no `weight` field and
/// no `description` field at all -- not "populated for some rows and
/// empty for others," but structurally absent from the type. `has_cost`
/// is real (computed from `cost_gp.is_some()`), never fabricated.
#[test]
fn equipment_table_entry_has_zero_weight_and_description_field_coverage_in_every_book() {
    macro_rules! assert_zero_weight_and_description {
        ($report:expr) => {
            let report = $report;
            assert_eq!(
                report.has_weight, 0,
                "EquipmentTableEntry has no weight field in any book today -- if this now \
                 fails, a book gained a weight field and criterion 6.3 should already be \
                 underway"
            );
            assert_eq!(
                report.has_description, 0,
                "EquipmentTableEntry has no description field in any book today -- if this \
                 now fails, a book gained a description field and criterion 6.4 should \
                 already be underway"
            );
            assert!(
                report.total_records > 0,
                "every book's equipment table should have at least one real record"
            );
        };
    }
    assert_zero_weight_and_description!(crb::equipment_tables::field_coverage_report());
    assert_zero_weight_and_description!(apg::equipment_tables::field_coverage_report());
    assert_zero_weight_and_description!(acg::equipment_tables::field_coverage_report());
}

/// APG's equipment corpus (`apg_equip_general.lst` + `apg_equip_arms_armor.lst`
/// + `apg_equip_magic_items.lst`, 94 + 76 + 171 = 341 real active records)
/// is still at its SD-22 Epic 3 bootstrap sample (3 records, one per
/// category) -- a genuine, large record-coverage gap for criterion 6.2's
/// remediation backlog.
#[test]
fn apg_equipment_is_bootstrap_only_far_below_full_corpus() {
    let report = apg::equipment_tables::field_coverage_report();
    assert_eq!(report.total_records, 3, "APG equipment bootstrap sample (one per category)");
    assert_eq!(
        report.records_expected, 341,
        "documented real APG equipment corpus count (94+76+171 active, non-.MOD records)"
    );
}

/// ACG's equipment corpus (single `acg_equip.lst`, 221 real active records)
/// is likewise still at its SD-22 Epic 4 bootstrap sample (3 records).
#[test]
fn acg_equipment_is_bootstrap_only_far_below_full_corpus() {
    let report = acg::equipment_tables::field_coverage_report();
    assert_eq!(report.total_records, 3, "ACG equipment bootstrap sample (one per category)");
    assert_eq!(
        report.records_expected, 221,
        "documented real ACG equipment corpus count (acg_equip.lst active records)"
    );
}

/// CRB's spell list (`cr_spells.lst`) carries 652 of 675 real active
/// records -- a real, if modest, record-coverage gap. Every present
/// record's `description` field is populated (real per-`SpellListEntry`
/// text), but it is always the corpus's short `DESC:` summary line, never
/// the full SRD/PRD spell text (which for many spells lives in a
/// separate `.MOD` record's own, longer `DESC:` token, e.g. `Alarm.MOD`
/// vs. base `Alarm`) -- `full_text_verified` is 0 for every present
/// record today, the exact gap criterion 6.5 exists to close.
#[test]
fn crb_spell_list_has_a_record_gap_and_zero_full_text_coverage() {
    let report = crb::spell_list::spell_coverage_report();
    assert_eq!(
        report.records_expected, 675,
        "documented real cr_spells.lst active (non-.MOD, non-comment) record count"
    );
    assert_eq!(report.total_records, 652, "current CRB spell list ingest count");
    assert_eq!(
        report.has_description, report.total_records,
        "every present SpellListEntry has a non-empty description (the short summary)"
    );
    assert_eq!(
        report.full_text_verified, 0,
        "no present spell has full SRD/PRD text ingested yet -- if this now fails, criterion \
         6.5 has already landed full text for at least one spell and this canary is stale"
    );
}

/// APG's (`apg_spells.lst`, 300 real active records) and ACG's
/// (`acg_spells.lst`, 145 real active records) spell lists are both still
/// at their SD-22 bootstrap sample (5 records each) -- large
/// record-coverage gaps, same shape as their equipment tables above.
#[test]
fn apg_and_acg_spell_lists_are_bootstrap_only_far_below_full_corpus() {
    let apg_report = apg::spell_list::spell_coverage_report();
    assert_eq!(apg_report.total_records, 4, "APG spell list bootstrap sample");
    assert_eq!(apg_report.records_expected, 298, "documented real apg_spells.lst active record count");
    assert_eq!(apg_report.full_text_verified, 0);

    let acg_report = acg::spell_list::spell_coverage_report();
    assert_eq!(acg_report.total_records, 4, "ACG spell list bootstrap sample");
    assert_eq!(acg_report.records_expected, 145, "documented real acg_spells.lst active record count");
    assert_eq!(acg_report.full_text_verified, 0);
}
