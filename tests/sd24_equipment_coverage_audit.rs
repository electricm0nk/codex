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

/// `EquipmentTableEntry` (ACG) still has no `weight` field populated and
/// no `description` field populated at all -- not "populated for some
/// rows and empty for others," but structurally absent-of-content in the
/// only book left with zero coverage in both fields. `has_cost` is real
/// (computed from `cost_gp.is_some()`), never fabricated.
///
/// **CRB and APG are no longer part of this canary**, as of two SD-24
/// criteria 6.3/6.4 cycles landing concurrently: CRB's own copy of the
/// type gained both fields, populated to the corpus's honest ceiling
/// (see `equipment_table_entry_weight_and_description_field_coverage_for_crb`
/// below and `tests/sd24_equipment_field_completion.rs` for the exact
/// per-field counts). APG's own copy gained real `weight: Option<f64>`
/// and `description: Option<&'static str>` fields -- `weight` is
/// populated for 319/338 real records (see
/// `apg_equipment_gained_weight_field_and_is_fully_record_ingested`
/// below); `description` remains `None` for all 338 (the real APG
/// equipment corpus carries zero `DESC:` tokens on any equipment row --
/// a genuine corpus limitation, not a parsing gap).
#[test]
fn equipment_table_entry_has_zero_weight_and_description_field_coverage_in_acg() {
    macro_rules! assert_zero_weight_and_description {
        ($report:expr) => {
            let report = $report;
            assert_eq!(
                report.has_weight, 0,
                "EquipmentTableEntry has no weight field populated in this book today -- if \
                 this now fails, the book gained weight coverage and criterion 6.3 should \
                 already be underway for it"
            );
            assert_eq!(
                report.has_description, 0,
                "EquipmentTableEntry has no description field populated in this book today -- \
                 if this now fails, the book gained description coverage and criterion 6.4 \
                 should already be underway for it"
            );
            assert!(
                report.total_records > 0,
                "every book's equipment table should have at least one real record"
            );
        };
    }
    assert_zero_weight_and_description!(acg::equipment_tables::field_coverage_report());
}

/// SD-24 criteria 6.3/6.4 (this cycle, CRB-only file-touch scope): CRB's
/// `EquipmentTableEntry` gained real `weight_lbs`/`description` fields,
/// populated to the honest ceiling the real corpus supports (never
/// fabricated -- a `None` here is a genuine corpus `WT:`/`DESC:`-token
/// absence). See `tests/sd24_equipment_field_completion.rs` for the exact
/// counts this canary intentionally does not repeat.
#[test]
fn equipment_table_entry_weight_and_description_field_coverage_for_crb() {
    let report = crb::equipment_tables::field_coverage_report();
    assert!(
        report.has_weight > 0 && report.has_weight < report.total_records,
        "CRB weight coverage should be real and partial (not fabricated to 100%, not still 0)"
    );
    assert!(
        report.has_description > 0 && report.has_description < report.total_records,
        "CRB description coverage should be real and partial (not fabricated to 100%, not still 0)"
    );
}

/// APG's equipment corpus (`apg_equip_general.lst` + `apg_equip_arms_armor.lst`
/// + `apg_equip_magic_items.lst`) is **fully record-ingested** as of the
/// criterion 6.2/6.3/6.4 cycle -- 338 real, active records (corrected
/// from the criterion 6.1 audit's originally-documented 341: each of the
/// three corpus files carries exactly one `SOURCELONG:` header line the
/// audit's grep-based count double-counted as a record; see
/// `rules_tables::apg::equipment_data`'s module doc comment). `weight` is
/// real per-row (319/338 -- the corpus's own `WT:` token, `None` for the
/// 19 records with no `WT:` token at all). `description` is `None` for
/// every record -- the real APG equipment corpus has no `DESC:` token on
/// any equipment row (confirmed by direct inspection), a genuine corpus
/// limitation criterion 6.4 cannot close from this corpus alone.
#[test]
fn apg_equipment_gained_weight_field_and_is_fully_record_ingested() {
    let report = apg::equipment_tables::field_coverage_report();
    assert_eq!(
        report.records_expected, 338,
        "corrected real APG equipment corpus count (93+75+170 active, non-.MOD, \
         non-SOURCELONG-header records)"
    );
    assert_eq!(
        report.total_records, report.records_expected,
        "APG equipment record coverage should be 100% (fully ingested as of criterion 6.2)"
    );
    assert_eq!(report.has_weight, 319, "319/338 real records carry a WT: token");
    assert_eq!(
        report.has_description, 0,
        "the real APG equipment corpus has no DESC: token on any equipment row -- if this \
         now fails, a description source has been found and criterion 6.4 should already be \
         underway"
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

/// CRB's spell list (`cr_spells.lst`) carries 652 of 652 real,
/// level-and-school-bearing spell records -- criterion 6.1's original
/// "675 real / 96.6%" figure was a measurement error (see
/// `SpellFieldCoverage::records_expected`'s doc comment for the
/// correction), not a genuine record-coverage gap; this cycle's re-audit
/// found CRB spell record coverage was already 100%. SD-24 criterion 6.5
/// (this cycle) additionally replaced every present record's truncated
/// first-sentence `description` with the fullest text the real corpus
/// provides -- `full_text_verified` is now 652/652, not the pre-cycle 0.
#[test]
fn crb_spell_list_is_fully_record_complete_with_full_text_coverage() {
    let report = crb::spell_list::spell_coverage_report();
    assert_eq!(
        report.records_expected, 652,
        "real, level-and-school-bearing cr_spells.lst record count (corrected from the \
         original 675 measurement-error figure)"
    );
    assert_eq!(report.total_records, 652, "current CRB spell list ingest count");
    assert_eq!(
        report.records_expected, report.total_records,
        "CRB spell record coverage should be 100% (already fully ingested, criterion 6.1's \
         original gap claim was a counting error)"
    );
    assert_eq!(
        report.has_description, report.total_records,
        "every present SpellListEntry has a non-empty description"
    );
    assert_eq!(
        report.full_text_verified, report.total_records,
        "every present spell should now carry the corpus's fullest available text -- if this \
         now regresses, criterion 6.5's full-text ingestion was lost"
    );
}

/// APG's spell list (`apg_spells.lst`) is **fully record-ingested** as of
/// the criterion 6.2/6.5 cycle -- 297 real, deduplicated-by-name records
/// (corrected from the criterion 6.1 audit's originally-documented 298:
/// the real corpus has one genuine duplicate `Resounding Blow` base
/// record the audit's dedup methodology missed; see
/// `rules_tables::apg::spell_list`'s module doc comment). 261 of 297
/// records carry full SRD/PRD text sourced from a matching `<Name>.MOD`
/// corpus record (`full_text_verified`), a real majority -- criterion
/// 6.5 is not 100% closed for APG (41 records have no `SCHOOL:`/
/// `CLASSES:` token at all -- mostly Summoner eidolon spells with no
/// leveled spell-list entry in the real rules -- and not every present
/// record has a matching `.MOD` full-text record either), but this is
/// real, substantial, sourced progress, not a bootstrap sample.
#[test]
fn apg_spell_list_is_fully_record_ingested_with_majority_full_text_coverage() {
    let apg_report = apg::spell_list::spell_coverage_report();
    assert_eq!(
        apg_report.records_expected, 297,
        "corrected real apg_spells.lst deduplicated-by-name active record count"
    );
    assert_eq!(
        apg_report.total_records, apg_report.records_expected,
        "APG spell list record coverage should be 100% (fully ingested as of criterion 6.2)"
    );
    assert_eq!(apg_report.has_description, 281, "281/297 records have a sourced description");
    assert_eq!(
        apg_report.full_text_verified, 261,
        "261/297 records carry full SRD/PRD text sourced from a matching .MOD record -- if \
         this now fails, criterion 6.5's APG full-text ingest has regressed"
    );
}

/// ACG's spell list (`acg_spells.lst`, 145 real active records) is still
/// at its SD-22 bootstrap sample (4 records) -- a large record-coverage
/// gap, same shape APG's equipment/spell tables had before this cycle.
#[test]
fn acg_spell_list_is_bootstrap_only_far_below_full_corpus() {
    let acg_report = acg::spell_list::spell_coverage_report();
    assert_eq!(acg_report.total_records, 4, "ACG spell list bootstrap sample");
    assert_eq!(acg_report.records_expected, 145, "documented real acg_spells.lst active record count");
    assert_eq!(acg_report.full_text_verified, 0);
}
