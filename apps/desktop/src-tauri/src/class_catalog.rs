//! SD-19 class progression catalog browser — Tauri command adapter over the
//! full CRB class table store (`rules_tables::crb::class_tables`, every
//! class's level-1 through its `max_supported_level` ceiling: BAB and the
//! three base saves).
//!
//! Distinct from the Character Sheet: this is a standalone catalog view of
//! every real class-progression row the engine knows about, not what one
//! character has selected. Built to satisfy the operator's full
//! "UI-surfacing" bar for the SD-19 `class.*` matrix rows — literal display
//! of every level of every class, not just a per-character sample. Mirrors
//! `equipment_catalog.rs` / `spell_catalog.rs` exactly.

use serde::{Deserialize, Serialize};

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::crb::class_tables::class_tables;
use codex::rules_core::rules_tables::pathfinder_unchained::class_chassis::{
    self as pu_class_chassis, PuClassId,
};

use crate::authoring_workbench::codex_repo_root;
use crate::class_catalog_generic::generic_class_catalog_entries;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassCatalogEntryDto {
    /// The `ClassId` variant name verbatim (e.g. "Fighter").
    pub class_id: String,
    pub level: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassCatalogResponse {
    pub entries: Vec<ClassCatalogEntryDto>,
}

fn map_catalog_entry(
    row: &codex::rules_core::rules_tables::crb::class_tables::ClassTableRow,
) -> ClassCatalogEntryDto {
    ClassCatalogEntryDto {
        class_id: format!("{:?}", row.class_id),
        level: row.level,
        base_attack_bonus: row.base_attack_bonus,
        fort_save: row.fort_save,
        ref_save: row.ref_save,
        will_save: row.will_save,
    }
}

/// Build the full catalog response. A thin, testable wrapper behind the
/// Tauri command below (mirroring `equipment_catalog`'s and
/// `spell_catalog`'s own command/pure-fn split).
///
/// **SD-27 (2026-07-31) widened this past the CRB table** to include
/// Pathfinder Unchained's four classes, which a player can now select at
/// creation. They appear under their own display names ("Unchained
/// Barbarian", …) alongside — never instead of — the eleven CRB rows, which
/// is what the browser needs to show for a REPLACEMENT pair: a campaign
/// picks one of the two, and the catalog's job is to let a reader compare
/// them.
///
/// **The 16 APG/ACG classes are still absent from this catalog.** That is a
/// pre-existing gap, not one this change introduced, and it was deliberately
/// left alone: those books have their own `class_chassis_resolve` seams and
/// widening the catalog to them is a separate piece of work with its own
/// row-count expectations. The screen's caption is derived from the data
/// rather than hardcoded, so it states the true class count either way.
pub fn build_class_catalog() -> ClassCatalogResponse {
    let mut entries: Vec<ClassCatalogEntryDto> =
        class_tables().iter().map(map_catalog_entry).collect();

    for class_id in PuClassId::ALL {
        for level in 1..=class_id.max_supported_level() {
            let Some(row) = pu_class_chassis::class_chassis_resolve(class_id, level, RuleSetId::Pu)
            else {
                // Unreachable for `1..=max_supported_level()`, and pinned by
                // `pathfinder_unchained::class_chassis`'s own level-sweep
                // test. Skipping rather than unwrapping keeps a catalog
                // browser from being the thing that panics the app.
                continue;
            };
            entries.push(ClassCatalogEntryDto {
                class_id: class_id.display_name().to_owned(),
                level: row.level,
                base_attack_bonus: row.base_attack_bonus,
                fort_save: row.fort_save,
                ref_save: row.ref_save,
                will_save: row.will_save,
            });
        }
    }

    // SD-32 T12 Epic 10 row 20 cycle 4: widened with the 60 (of the 61 real
    // conventional PC classes row20-cycle3 found across 13 of the 17
    // `classes`-family gap books; see `class_catalog_generic.rs`'s own
    // module doc for the 61st, `Demoniac`, and why it does not resolve yet)
    // classes whose BAB/save progression is computed generically from their
    // own corpus `raw_tokens` rather than hand-authored, per `decisions.md
    // §17` ("stop treating every object as a snowflake"). A missing repo
    // root (packaged-app deployment without `data/corpus/` bundled, the
    // same caveat `class_feature_descriptions.rs` already documents) skips
    // this widening rather than panicking the whole catalog.
    if let Ok(repo_root) = codex_repo_root() {
        entries.extend(generic_class_catalog_entries(&repo_root));
    }

    ClassCatalogResponse { entries }
}

#[tauri::command]
pub fn list_class_catalog() -> ClassCatalogResponse {
    build_class_catalog()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_all_eleven_classes_and_expected_counts() {
        let response = build_class_catalog();
        // Task #49 (2026-07-28) widened Monk's own max_supported_level from
        // 12 to 20 (rules_tables::crb::class_tables.rs's CLASS_META row,
        // mirroring pilot_compute.rs's own MAX_SUPPORTED_MONK_LEVEL
        // widening to the full PF1 Core Rulebook capstone range), so the
        // total row count rose from 207 to 215 (207 - 12 + 20).
        //
        // Commit 72d83e75 ("widen Druid to level 20 -- 10th class fully
        // computing", 2026-07-29) then widened Druid's CLASS_META row from
        // 15 to 20 but did not update this test, leaving it red on
        // tranche/6. Corrected here: 215 - 15 + 20 = 220.
        //
        // SD-27 (2026-07-31) added Pathfinder Unchained's four classes at
        // 20 levels each: 220 + 80 = 300. The eleven CRB counts below are
        // asserted unchanged in the same test, so a PU row landing on a CRB
        // class instead of beside it fails here rather than passing quietly.
        //
        // SD-32 T12 Epic 10 row 20 cycle 4 added the 60 generically-computed
        // conventional-PC classes (`class_catalog_generic.rs`): 808 more
        // rows (`python3` sweep over their own `MAXLEVEL`/`Prestige`-default
        // per-class row counts, cited in that module's doc comment) ->
        // 300 + 808 = 1108.
        assert_eq!(response.entries.len(), 1108);

        let counts = |class_id: &str| {
            response
                .entries
                .iter()
                .filter(|e| e.class_id == class_id)
                .count()
        };
        assert_eq!(counts("Barbarian"), 20);
        assert_eq!(counts("Bard"), 20);
        assert_eq!(counts("Cleric"), 20);
        assert_eq!(counts("Druid"), 20);
        assert_eq!(counts("Fighter"), 20);
        assert_eq!(counts("Monk"), 20);
        assert_eq!(counts("Paladin"), 20);
        assert_eq!(counts("Ranger"), 20);
        assert_eq!(counts("Rogue"), 20);
        assert_eq!(counts("Sorcerer"), 20);
        assert_eq!(counts("Wizard"), 20);

        assert_eq!(counts("Unchained Barbarian"), 20);
        assert_eq!(counts("Unchained Monk"), 20);
        assert_eq!(counts("Unchained Rogue"), 20);
        assert_eq!(counts("Unchained Summoner"), 20);
    }

    /// A replacement pair must be two visibly distinct rows, never one
    /// overwriting the other. Checked on the axis a browser actually shows:
    /// the Unchained Monk's full base attack bonus and poor Will save
    /// against the CRB Monk's three-quarter progression and good Will.
    #[test]
    fn the_unchained_monk_is_a_separate_row_from_the_crb_monk_and_the_numbers_differ() {
        let response = build_class_catalog();
        let row = |class_id: &str, level: u8| {
            response
                .entries
                .iter()
                .find(|e| e.class_id == class_id && e.level == level)
                .unwrap_or_else(|| panic!("{class_id} level {level} must be in the catalog"))
                .clone()
        };
        for level in [1u8, 10, 20] {
            let crb = row("Monk", level);
            let pu = row("Unchained Monk", level);
            assert_eq!(pu.base_attack_bonus, i16::from(level));
            assert!(pu.base_attack_bonus > crb.base_attack_bonus);
            assert!(pu.will_save < crb.will_save);
        }
    }

    /// The other three pairs deliberately carry identical numbers -- their
    /// corpus records override no chassis field -- so they must still be
    /// two rows, distinguished by class name alone. Asserted so "identical
    /// numbers" is never mistaken for "one row got dropped".
    #[test]
    fn the_other_three_pairs_are_still_two_rows_each_with_matching_numbers() {
        let response = build_class_catalog();
        for (unchained, base) in [
            ("Unchained Barbarian", "Barbarian"),
            ("Unchained Rogue", "Rogue"),
            ("Unchained Summoner", "Summoner"),
        ] {
            let unchained_rows: Vec<_> =
                response.entries.iter().filter(|e| e.class_id == unchained).collect();
            assert_eq!(unchained_rows.len(), 20, "{unchained}");

            let base_rows: Vec<_> =
                response.entries.iter().filter(|e| e.class_id == base).collect();
            if base == "Summoner" {
                // The APG Summoner is not in this catalog at all yet (see
                // `build_class_catalog`'s note on the 16 absent APG/ACG
                // classes), so there is no base row to compare against --
                // recorded explicitly rather than letting the loop skip it
                // silently.
                assert_eq!(base_rows.len(), 0, "APG Summoner is not in this catalog yet");
                continue;
            }
            assert_eq!(base_rows.len(), 20, "{base}");
            for level in 1..=20u8 {
                let u = unchained_rows.iter().find(|e| e.level == level).expect("level");
                let b = base_rows.iter().find(|e| e.level == level).expect("level");
                assert_eq!(u.base_attack_bonus, b.base_attack_bonus, "{unchained} level {level}");
                assert_eq!(u.fort_save, b.fort_save, "{unchained} level {level}");
                assert_eq!(u.ref_save, b.ref_save, "{unchained} level {level}");
                assert_eq!(u.will_save, b.will_save, "{unchained} level {level}");
            }
        }
    }

    #[test]
    fn every_entry_has_a_non_empty_class_id_and_positive_level() {
        let response = build_class_catalog();
        for entry in &response.entries {
            assert!(!entry.class_id.is_empty());
            assert!(entry.level >= 1);
        }
    }
}
