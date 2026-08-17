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
//! **Bestiary 1 update (SD-25 criterion 7.N item 4):** the "not covered
//! by this test file" gap noted above through SD-24 is now closed.
//! `rules_tables::beastiary1::equipment_tables` exists as of this cycle
//! (see that module's own doc comment for full sourcing methodology);
//! `beastiary1_equipment_is_fully_record_ingested_with_full_description_coverage`
//! below is this book's own coverage assertion, mirroring the pattern the
//! other three books already use. RED -> GREEN evidence for this
//! addition: before `rules_tables::beastiary1::equipment_tables` existed,
//! a test referencing it did not compile (`error[E0433]: failed to
//! resolve: could not find `equipment_tables` in `beastiary1``); adding
//! the module (4 real records, hand-transcribed from
//! `b1_equip_general.lst` + `b1_equip_arms_armor.lst` +
//! `b1_equip_magic_items.lst`, one field web-sourced per the cycle
//! receipt) turned it GREEN.

use codex::rules_core::rules_tables::{acg, apg, beastiary1, crb};

/// CRB's equipment corpus (`cr_equip_arms_armor.lst` + `cr_equip_general.lst`
/// \+ `cr_equip_magic_items.lst` + `cr_equipmods.lst`) is **fully record-ingested**
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

/// The criterion 6.1 "zero weight/description coverage" canary is now
/// retired: CRB, APG, and ACG have all gained real `weight`/`description`
/// coverage across three concurrent SD-24 criteria 6.3/6.4 cycles
/// (`crb-field-completion-cycle`, `apg-field-completion-cycle`,
/// `acg-field-completion-cycle`). Each book's own dedicated test below
/// (`equipment_table_entry_weight_and_description_field_coverage_for_crb`,
/// `apg_equipment_gained_weight_field_and_is_fully_record_ingested`,
/// `equipment_table_entry_weight_and_description_field_coverage_for_acg`)
/// asserts its real, book-specific ceiling instead.

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
/// \+ `apg_equip_magic_items.lst`) is **fully record-ingested** as of the
/// criterion 6.2/6.3/6.4 cycle -- 338 real, active records (corrected
/// from the criterion 6.1 audit's originally-documented 341: each of the
/// three corpus files carries exactly one `SOURCELONG:` header line the
/// audit's grep-based count double-counted as a record; see
/// `rules_tables::apg::equipment_data`'s module doc comment). `weight` is
/// real per-row (319/338 -- the corpus's own `WT:` token, `None` for the
/// 19 records with no `WT:` token at all). `description` reached 331/338
/// via SD-25 criterion 7.N's `apg-description` web second-source pass
/// (register A16 / SD-24 Open Blocker #2 -- the real APG equipment
/// corpus itself still has no `DESC:` token on any row, confirmed by
/// direct inspection; every non-`None` value was identity-matched and
/// sourced from `legacy.aonprd.com`/`aonprd.com`/`d20pfsrd.com`, per that
/// cycle's receipt). The remaining 7 are honest, undispatched gaps (see
/// `equipment_data`'s doc comment), not a corpus ceiling any longer.
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
        report.has_description, 331,
        "SD-25 criterion 7.N's web second-source pass (register A16 / SD-24 Open Blocker #2) \
         identity-matched and sourced 331/338 descriptions from legacy.aonprd.com/aonprd.com/ \
         d20pfsrd.com -- if this regresses, the sourced descriptions were lost; if it exceeds \
         331 without a matching receipt update, verify the new value is genuinely sourced, not \
         fabricated"
    );
}

/// ACG's equipment corpus (`acg_equip.lst`'s 221 `TYPE:`-disambiguated
/// General/Arms-Armor/Magic-Items records + `acg_equipmods.lst`'s 48
/// `KEY:`-bearing modifier records = 269 total) is now **fully
/// record-ingested** (SD-24 criteria 6.2-6.4, `acg-field-completion-cycle`)
/// -- widened from criterion 6.1's original 221-only scope (which did not
/// count `acg_equipmods.lst` at all, unlike CRB's own four-category
/// treatment; see `progress.md`'s `## DISCOVERED`).
#[test]
fn acg_equipment_is_fully_record_ingested() {
    let report = acg::equipment_tables::field_coverage_report();
    assert_eq!(
        report.records_expected,
        221 + 48,
        "documented real ACG equipment corpus count (acg_equip.lst 221 + acg_equipmods.lst 48)"
    );
    assert_eq!(
        report.total_records, report.records_expected,
        "ACG equipment record coverage should be 100% (fully ingested this cycle)"
    );
}

/// SD-24 criteria 6.3/6.4 (this cycle, ACG scope): ACG's `EquipmentTableEntry`
/// gained real `weight_lbs`/`description` fields. Unlike CRB (whose corpus
/// has no `DESC:` token convention for most equipment), ACG's `SPROP:`
/// token is the sourcing basis for `description` (see `equipment_tables.rs`'s
/// own doc comment) and covers the large majority of records, so coverage
/// here is high but still honestly partial where the corpus itself has no
/// `WT:`/`SPROP:` token (e.g. every `acg_equipmods.lst` record has no
/// `WT:` token at all).
#[test]
fn equipment_table_entry_weight_and_description_field_coverage_for_acg() {
    let report = acg::equipment_tables::field_coverage_report();
    assert!(
        report.has_weight > 0 && report.has_weight < report.total_records,
        "ACG weight coverage should be real and partial (not fabricated to 100%, not still 0)"
    );
    assert!(
        report.has_description > 0 && report.has_description < report.total_records,
        "ACG description coverage should be real and partial (not fabricated to 100%, not still 0)"
    );
}

/// Bestiary 1's equipment corpus (`b1_equip_general.lst` (1) +
/// `b1_equip_arms_armor.lst` (2) + `b1_equip_magic_items.lst` (1) = 4
/// total) is **fully record-ingested** as of SD-25 criterion 7.N item 4
/// -- a plain scope gap (no module existed at all), not a corpus
/// ceiling, so 4/4 is the honest ceiling for `total_records` itself.
/// `weight_lbs` is 4/4 (every real record carries a `WT:` token,
/// including literal `0` values). `cost_gp` is **3/4, corrected
/// `SD31-E6-F5-004`** (`OPEN-ISSUES.md` row 91's typed-field cross-check):
/// `Poison (Black Smear)` (`b1_equip_general.lst:7`) carries no `COST:`
/// token at all in the pinned oracle -- the original `Some(0.0)` was a
/// transcription error (a stated `0` price is not the same fact as no
/// price being stated), not a genuine gap this test should paper over by
/// asserting a count the corpus does not support. `description` is 4/4:
/// 3 records source from the corpus's own `SPROP:` token (register A10,
/// same convention `acg::equipment_data` established, and the SAME
/// `SPROP:` token that recovers `Poison (Black Smear)`'s own
/// description even without a `COST:`), and the 4th (`Rag Armor (Dark
/// Creeper)`, which has neither `DESC:` nor `SPROP:`) was closed via an
/// identity-matched web second-source pass -- see this cycle's receipt
/// for the cited URLs.
#[test]
fn beastiary1_equipment_is_fully_record_ingested_with_full_description_coverage() {
    let report = beastiary1::equipment_tables::field_coverage_report();
    assert_eq!(
        report.records_expected, 4,
        "real Bestiary 1 equipment corpus count (1 general + 2 arms_armor + 1 magic_items)"
    );
    assert_eq!(
        report.total_records, report.records_expected,
        "Bestiary 1 equipment record coverage should be 100% (closed this cycle)"
    );
    assert_eq!(
        report.has_cost, 3,
        "3 of 4 real records carry a COST: token -- Poison (Black Smear) genuinely does not \
         (b1_equip_general.lst:7 has no COST: token in the pinned oracle at all); asserting 4 \
         here would re-introduce the fabricated-price defect SD31-E6-F5-004 fixed"
    );
    assert_eq!(report.has_weight, 4, "every real record carries a WT: token");
    assert_eq!(
        report.has_description, 4,
        "every real record has a description -- 3 from SPROP:, 1 (Rag Armor (Dark Creeper)) \
         from a cited web second-source pass (see this cycle's receipt) -- if this regresses, \
         a fabricated or dropped description slipped through"
    );
}

/// CRB's spell list (`cr_spells.lst`) carries 664 of 664 real,
/// level-and-school-bearing spell records -- criterion 6.1's original
/// "675 real / 96.6%" figure was a measurement error (see
/// `SpellFieldCoverage::records_expected`'s doc comment for the
/// correction), not a genuine record-coverage gap; this cycle's re-audit
/// found CRB spell record coverage was already 100%. SD-24 criterion 6.5
/// additionally replaced every present record's truncated first-sentence
/// `description` with the fullest text the real corpus provides. SD31
/// decisions.md §15 (2026-08-17) added the 12 `.COPY=` racial
/// spell-like-ability variant records this book owns (the 13th belongs to
/// `advanced_race_guide`) as their own distinct entries rather than
/// merging them into their parent -- the count grew from 652 to 664 for
/// that reason, not a new coverage gap.
#[test]
fn crb_spell_list_is_fully_record_complete_with_full_text_coverage() {
    let report = crb::spell_list::spell_coverage_report();
    assert_eq!(
        report.records_expected, 664,
        "real, level-and-school-bearing cr_spells.lst record count, including the 12 \
         `.COPY=` racial SLA variants ingested under decisions.md §15"
    );
    assert_eq!(report.total_records, 664, "current CRB spell list ingest count");
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
/// `rules_tables::apg::spell_list`'s module doc comment). 284 of 297
/// records now carry full SRD/PRD text (`full_text_verified`), raised
/// from criterion 6.5's original 261 by SD-25 criterion 7.N's
/// "apg-spell-text" pass -- 13 recovered from an ingest miss on a
/// same-line-concatenated `.MOD` pair the corpus itself already carried
/// (`Fiery Body`/`Fester (Mass)`, `Transmute Potion to Poison`/
/// `Transmogrify`, plus the `Summon Monster I`-`IX` family's own
/// same-line double-`DESC:` full paragraph), 3 from a same-book
/// `PRESPELL`-fallback extension (`Threefold Aspect`'s 3 sub-forms), and
/// 7 from a `d20pfsrd.com`/`legacy.aonprd.com` web second-source pass
/// (see `rules_tables::apg::spell_list`'s module doc comment for the
/// full per-record sourcing and the rejected edition-cousin false match).
/// The remaining 13/297 gap WAS the documented cross-book `.COPY=`
/// variant scope boundary plus the corpus-typo `Wall of Thorms`. **That
/// boundary is resolved (SD-27, 2026-07-31)**: every `.COPY=` variant
/// whose base spell lives in CRB's `cr_spells.lst` now inherits that
/// base's school, level and description, so coverage is 297/297
/// descriptions and 296/297 full text. The one holdout is
/// `Threefold Aspect`, whose text is a same-book `PRESPELL` fallback
/// rather than sourced SRD prose. Twelve of the thirteen were reaching
/// `list_spell_catalog` as a key and three nulls; see
/// `tests/sd27_apg_delta_spell_rows_resolve_against_their_base.rs`.
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
    assert_eq!(
        apg_report.has_description, 297,
        "297/297 records have a sourced description (was 285 until SD-27's cross-book \
         `.COPY=` resolution; 281 before SD-25 criterion 7.N's apg-spell-text pass)"
    );
    assert_eq!(
        apg_report.full_text_verified, 296,
        "296/297 records carry full SRD/PRD text (284 until SD-27's cross-book `.COPY=` \
         resolution; 261 before SD-25 criterion 7.N's apg-spell-text pass -- if this now \
         regresses, that pass's ingest-bug fixes and web second-source sourcing were lost; \
         if it exceeds 296 without a matching receipt update, verify the new value is \
         genuinely sourced, not fabricated)"
    );
}

/// ACG's spell list (`acg_spells.lst`) is now **fully record-complete with
/// full-text coverage** (SD-24 criteria 6.2/6.5, `acg-field-completion-cycle`).
/// Criterion 6.1's original "145" figure double-counted the file's own
/// `SOURCELONG:` header line as a spell (see `spell_list.rs`'s doc comment
/// for the correction, the same measurement-error shape CRB's own "675"
/// figure had); the real, level-and-school-bearing count is 144. Unlike
/// CRB, ACG's base (non-`.MOD`) spell record already carries the full
/// corpus text, so `full_text_verified` reaches 144/144 without a
/// second-pass `.MOD`-record lookup.
#[test]
fn acg_spell_list_is_fully_record_complete_with_full_text_coverage() {
    let acg_report = acg::spell_list::spell_coverage_report();
    assert_eq!(
        acg_report.records_expected, 144,
        "real, level-and-school-bearing acg_spells.lst record count (corrected from the \
         original 145 measurement-error figure)"
    );
    assert_eq!(acg_report.total_records, 144, "current ACG spell list ingest count");
    assert_eq!(
        acg_report.records_expected, acg_report.total_records,
        "ACG spell record coverage should be 100% (fully ingested this cycle)"
    );
    assert_eq!(
        acg_report.has_description, acg_report.total_records,
        "every present SpellListEntry has a non-empty description"
    );
    assert_eq!(
        acg_report.full_text_verified, acg_report.total_records,
        "every present ACG spell should carry the corpus's fullest available text"
    );
}
