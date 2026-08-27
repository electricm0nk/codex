//! SD-19 spell-id resolver.
//!
//! Resolves a `CharacterInput.spells_selected[].spell_id` to its real
//! PCGen corpus record and (when available) the foundation slice's
//! canonical Paizo-table-cell reference.
//!
//! Note on identity: unlike equipment records, spell records in
//! `cr_spells.lst` carry no `KEY:` token — a spell's identity is its
//! `name` field (confirmed against the real corpus; see
//! `rules_tables::crb::spell_list`'s doc comment). So "spell_id" here
//! means the spell's corpus `name`, matched exactly; no normalization is
//! needed since PF1 spell names are unique across the strict-school
//! partition.

use crate::pcgen_import::lst_parser::spell::LstSpellRecord;
use crate::pcgen_import::source_content_payload::SourceContentPayload;
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::{
    acg, adventurers_guide, advanced_race_guide, apg, bestiary, bestiary_4, bestiary_6,
    book_of_the_damned_volume_1, book_of_the_damned_volume_2, crb, horror_adventures,
    inner_sea_faiths, inner_sea_gods, inner_sea_intrigue, inner_sea_magic, inner_sea_races,
    inner_sea_temples, inner_sea_world_guide, monster_codex, mythic_adventures,
    occult_adventures, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic,
    ultimate_magic_wordsofpower, ultimate_wilderness,
};
use crate::rules_core::source_content::{SourceContentKind, SourcePackageContent};

/// Wire-form book codes for [`spell_catalog_rows`]. These are the same
/// short codes the desktop Spell Catalog already put on the wire, kept
/// verbatim so this consolidation changes no payload the frontend reads.
pub const SPELL_BOOK_CRB: &str = "CRB";
pub const SPELL_BOOK_APG: &str = "APG";
pub const SPELL_BOOK_ACG: &str = "ACG";
pub const SPELL_BOOK_ARG: &str = "ARG";
pub const SPELL_BOOK_UI: &str = "UI";
/// SD31-E6-F2-002: Ultimate Magic, the sixth book -- the first widening of
/// this catalog beyond the five books `spell_resolver.rs`'s own module doc
/// comment named as the reachable set before this cycle.
pub const SPELL_BOOK_UM: &str = "UM";
/// SD31-E6-F2-003: Occult Adventures, the seventh book -- the largest of
/// the 19-book remainder `SD31-E6-F2-002` named (`OPEN-ISSUES.md` row 57).
pub const SPELL_BOOK_OA: &str = "OA";
/// SD31-E6-F2-004: Ultimate Combat, the eighth book -- the largest
/// remaining not-started `spell` book after Occult Adventures, re-derived
/// fresh off `docs/work-inventory.json` at this cycle's own tip.
pub const SPELL_BOOK_UC: &str = "UC";
/// SD31-E6-F10-001: Inner Sea Gods, the ninth book -- the largest
/// `engine-does-not-hold` `spell` book (96 units) with BOTH a compiled `RuleSetId`
/// (`RuleSetId::Isg`, from SD-29's monster lane) AND a real, dedicated
/// `isg_spells.lst` corpus file, re-derived fresh off
/// `docs/work-inventory.json` at this cycle's own tip. (`bestiary`'s 109 and
/// `bestiary_4`'s 56 are larger by unit count but have NO dedicated spell
/// `.lst` file of their own -- their residual is monster-intrinsic
/// spell-like-ability data, a different shape, not a book-chaining gap.)
pub const SPELL_BOOK_ISG: &str = "ISG";
/// SD-31 wave-19 (`ultimate_wilderness` lane): Ultimate Wilderness, the
/// tenth book -- 61 base spell declarations (`uw_spells.lst`), the whole of
/// this book's `spell`-kind `engine-does-not-hold` population. See
/// `src/bin/ingest_ultimate_wilderness_spells.rs` for the ingest path.
pub const SPELL_BOOK_UW: &str = "UW";
/// SD-31 wave-24 (`bestiary_6` book-auditor lane): Bestiary 6, the eleventh
/// book -- both of its 2 base spell declarations (`b6_spells.lst`), the
/// whole of this book's `spell`-kind `engine-does-not-hold` population. See
/// `rules_tables::bestiary_6::spell_list`'s doc comment for the two rows'
/// verbatim reprint inside Ultimate Wilderness's own `uw_spells.lst`.
pub const SPELL_BOOK_B6: &str = "B6";
/// SD-31 wave-29 (`lane5-book-onboard` lane): Adventurer's Guide, the
/// twelfth book -- 45 of 49 base spell declarations ship (`ag_spells.lst`,
/// 4 PI-dropped on `NAMEISPI:YES`), this book's FIRST compiled `RuleSetId`
/// of any kind (`RuleSetId::AdventurersGuide`). See
/// `src/bin/ingest_adventurers_guide_spells.rs` for the ingest path.
pub const SPELL_BOOK_AG: &str = "AG";
/// SD-32 Gate 0 book-onboarding precondition (`gate-0-book-onboarding-
/// precondition`, AT-32-G0-003): Inner Sea Faiths, the thirteenth book --
/// this book's FIRST compiled `RuleSetId` of any kind. See
/// `src/bin/ingest_inner_sea_setting_spells.rs` for the ingest path.
pub const SPELL_BOOK_ISF: &str = "ISF";
/// SD-32 Gate 0 book-onboarding precondition (`gate-0-book-onboarding-
/// precondition`, AT-32-G0-003): Inner Sea Magic, the fourteenth book --
/// this book's FIRST compiled `RuleSetId` of any kind (its 218
/// `class_feature` units were already ingested corpus-wide but were
/// unreachable through the book-level gate before this variant existed).
/// See `src/bin/ingest_inner_sea_setting_spells.rs` for the ingest path.
pub const SPELL_BOOK_ISM: &str = "ISM";
/// SD-32 Gate 0 book-onboarding precondition (`gate-0-book-onboarding-
/// precondition`, AT-32-G0-003): Inner Sea Temples, the fifteenth book --
/// this book's FIRST compiled `RuleSetId` of any kind. See
/// `src/bin/ingest_inner_sea_setting_spells.rs` for the ingest path.
pub const SPELL_BOOK_ISTEM: &str = "ISTEM";
/// SD-32 card 11 (T9 onboarding, `decisions.md §19` sign-off): Horror
/// Adventures, the sixteenth book -- its second compiled record family
/// (`RuleSetId::Ha` already exists for `companion`/`monster`/
/// `monster_ability`). See `src/bin/ingest_spells.rs`'s `BOOKS` entry.
pub const SPELL_BOOK_HA: &str = "HA";
/// SD-32 row 20 (`decisions.md §17`/`§27b`): the 43-family reach gap row 19
/// cycle 4 discovered but did not close. Every one of these tables was
/// ALREADY generated by `src/bin/ingest_spells.rs`'s config-driven `BOOKS`
/// list -- the gap was pure chaining, never new ingest work. Bestiary 1's
/// content lives in the `bestiary` module (this book's `spell` corpus
/// records are custom spell-like-ability variants transcribed from
/// `core_essentials/ce_spells.lst`, the same shared-library-host shape
/// `decisions.md §9`/`cache_gen::equipment_gap::book_routing` already
/// document for this book's equipment rows -- confirmed real: every one of
/// `data/corpus/bestiary/spell/*.json`'s 111 records carries
/// `source.path == "pathfinder/paizo/roleplaying_game/core_essentials/
/// ce_spells.lst"`), so it is chained here under the wire code the census
/// already expects for the `beastiary1` family.
pub const SPELL_BOOK_B1: &str = "B1";
pub const SPELL_BOOK_B4: &str = "B4";
pub const SPELL_BOOK_BOTD1: &str = "BOTD1";
pub const SPELL_BOOK_BOTD2: &str = "BOTD2";
pub const SPELL_BOOK_ISI: &str = "ISI";
pub const SPELL_BOOK_ISR: &str = "ISR";
pub const SPELL_BOOK_ISWG: &str = "ISWG";
pub const SPELL_BOOK_MC: &str = "MC";
pub const SPELL_BOOK_MYTHIC: &str = "MYTHIC";
pub const SPELL_BOOK_UE: &str = "UE";
pub const SPELL_BOOK_UMWP: &str = "UMWP";

/// One ingested spell record, normalized across every book's own
/// `spell_list` table.
///
/// **Why this type exists.** Each ingested book declares its *own*
/// `SpellListEntry` and its *own* `Pf1SchoolId` enum (`crb`, `apg`, `acg`,
/// `advanced_race_guide`, `ultimate_intrigue` and `ultimate_magic` each
/// define both), so there is no single Rust type spanning them and every
/// consumer that wanted "all ingested spells" had to chain them by hand. Two
/// consumers did exactly that and drifted apart: the desktop
/// `spell_catalog::build_spell_catalog` chained **five** books, while
/// `v06_work_inventory::gather_engine_facts` inserted **three**
/// (`core_rulebook`, `advanced_players_guide`, `advanced_class_guide`) —
/// so every ARG and UI spell already shipping in the catalog was reported
/// `engine-does-not-hold` by the work inventory. That is precisely the
/// SD-28-E15 defect `equipment_resolver::equipment_catalog_rows` was built
/// to close for equipment, reproduced on the spell family; this type
/// closes it the same way, by leaving no second list to diverge.
///
/// **SD31-E6-F2-002 widened the chain to six books**, adding Ultimate Magic
/// -- the structural finding wave 3's spell lane made (the catalog chains
/// only the books this doc comment names, so every OTHER spell-bearing
/// book's units are structurally `engine-does-not-hold` no matter how much ingest
/// work runs against them) closed for one real book rather than only
/// analyzed. See `src/bin/ingest_ultimate_magic_spells.rs` for the ingest
/// path and `docs/release/SD-31-corpus-closure-grind/progress.md`'s
/// `SD31-E6-F2-002` receipt for the remaining books this cycle did not
/// reach.
///
/// **SD31-E6-F2-003 widened the chain to seven books**, adding Occult
/// Adventures (145 base spell declarations; see
/// `src/bin/ingest_occult_adventures_spells.rs` for the ingest path and its
/// own doc comment for the 328-unit `mod_only` class-widening residue this
/// cycle deliberately did not ingest).
///
/// **SD31-E6-F2-004 widened the chain to eight books**, adding Ultimate
/// Combat (146 base spell declarations; see
/// `src/bin/ingest_ultimate_combat_spells.rs` for the ingest path and its
/// own doc comment for the 2-unit `mod_only` residue this cycle deliberately
/// did not ingest).
#[derive(Debug, Clone, PartialEq)]
pub struct SpellCatalogRow {
    /// One of the `SPELL_BOOK_*` codes above.
    pub book: &'static str,
    /// The record's corpus identity — its `KEY:` token when the row
    /// carries one, else its display name.
    pub key: &'static str,
    /// The book table's `Pf1SchoolId` variant name verbatim (e.g.
    /// `"Abjuration"`). `None` only where the book's own table types the
    /// field optionally *and* the corpus row carries no `SCHOOL:` token
    /// (APG, UM). Never fabricated.
    pub school: Option<String>,
    /// Minimum spell level across the corpus record's `CLASSES:`/`DOMAINS:`
    /// tag(s). `None` only where the book's own table types it optionally
    /// *and* the corpus row carries neither token (APG, UM).
    pub level: Option<u8>,
    /// The record's `DESC:` text exactly as the book's table stores it —
    /// still carrying PCGen `%N`/`|` syntax. Rendering for a player is the
    /// caller's job (the desktop catalog runs `render_pcgen_desc`); this
    /// registry deliberately does not pre-render, so a non-player consumer
    /// (the work inventory) sees the corpus text unaltered.
    pub description: Option<&'static str>,
}

/// Every ingested book's spell rows, in the same book order the desktop
/// catalog adapter chains them (CRB, APG, ACG, ARG, UI) and, within a book,
/// in that book's own table order.
///
/// Adding a sixth book here widens the desktop catalog **and** the work
/// inventory's `spell_levels` map in the same edit — there is no second
/// place to remember.
pub fn spell_catalog_rows() -> &'static [SpellCatalogRow] {
    static ROWS: std::sync::OnceLock<Vec<SpellCatalogRow>> = std::sync::OnceLock::new();
    ROWS.get_or_init(|| {
        let crb_rows = crb::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_CRB,
            key: entry.key,
            school: Some(format!("{:?}", entry.school)),
            level: Some(entry.level),
            description: Some(entry.description),
        });
        let apg_rows = apg::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_APG,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let acg_rows = acg::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_ACG,
            key: entry.key,
            school: Some(format!("{:?}", entry.school)),
            level: Some(entry.level),
            description: Some(entry.description),
        });
        let arg_rows =
            advanced_race_guide::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_ARG,
                key: entry.key,
                school: Some(format!("{:?}", entry.school)),
                level: Some(entry.level),
                description: Some(entry.description),
            });
        let ui_rows =
            ultimate_intrigue::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_UI,
                key: entry.key,
                school: Some(format!("{:?}", entry.school)),
                level: Some(entry.level),
                description: Some(entry.description),
            });
        let um_rows = ultimate_magic::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_UM,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let oa_rows = occult_adventures::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_OA,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let uc_rows = ultimate_combat::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_UC,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let isg_rows = inner_sea_gods::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_ISG,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let uw_rows = ultimate_wilderness::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_UW,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let b6_rows = bestiary_6::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_B6,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let ag_rows =
            adventurers_guide::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_AG,
                key: entry.key,
                school: entry.school.map(|school| format!("{school:?}")),
                level: entry.level,
                description: entry.description,
            });
        let isf_rows =
            inner_sea_faiths::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_ISF,
                key: entry.key,
                school: entry.school.map(|school| format!("{school:?}")),
                level: entry.level,
                description: entry.description,
            });
        let ism_rows =
            inner_sea_magic::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_ISM,
                key: entry.key,
                school: entry.school.map(|school| format!("{school:?}")),
                level: entry.level,
                description: entry.description,
            });
        let ha_rows = horror_adventures::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_HA,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let istem_rows =
            inner_sea_temples::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_ISTEM,
                key: entry.key,
                school: entry.school.map(|school| format!("{school:?}")),
                level: entry.level,
                description: entry.description,
            });
        let b1_rows = bestiary::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_B1,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let b4_rows = bestiary_4::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_B4,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let botd1_rows =
            book_of_the_damned_volume_1::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_BOTD1,
                key: entry.key,
                school: entry.school.map(|school| format!("{school:?}")),
                level: entry.level,
                description: entry.description,
            });
        let botd2_rows =
            book_of_the_damned_volume_2::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_BOTD2,
                key: entry.key,
                school: entry.school.map(|school| format!("{school:?}")),
                level: entry.level,
                description: entry.description,
            });
        let isi_rows =
            inner_sea_intrigue::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_ISI,
                key: entry.key,
                school: entry.school.map(|school| format!("{school:?}")),
                level: entry.level,
                description: entry.description,
            });
        let isr_rows = inner_sea_races::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_ISR,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let iswg_rows =
            inner_sea_world_guide::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_ISWG,
                key: entry.key,
                school: entry.school.map(|school| format!("{school:?}")),
                level: entry.level,
                description: entry.description,
            });
        let mc_rows = monster_codex::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_MC,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let mythic_rows =
            mythic_adventures::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_MYTHIC,
                key: entry.key,
                school: entry.school.map(|school| format!("{school:?}")),
                level: entry.level,
                description: entry.description,
            });
        let ue_rows = ultimate_equipment::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_UE,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let umwp_rows =
            ultimate_magic_wordsofpower::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_UMWP,
                key: entry.key,
                school: entry.school.map(|school| format!("{school:?}")),
                level: entry.level,
                description: entry.description,
            });
        let chained: Vec<SpellCatalogRow> = crb_rows
            .chain(apg_rows)
            .chain(acg_rows)
            .chain(arg_rows)
            .chain(ui_rows)
            .chain(um_rows)
            .chain(oa_rows)
            .chain(uc_rows)
            .chain(isg_rows)
            .chain(uw_rows)
            .chain(b6_rows)
            .chain(ag_rows)
            .chain(isf_rows)
            .chain(ism_rows)
            .chain(istem_rows)
            .chain(ha_rows)
            .chain(b1_rows)
            .chain(b4_rows)
            .chain(botd1_rows)
            .chain(botd2_rows)
            .chain(isi_rows)
            .chain(isr_rows)
            .chain(iswg_rows)
            .chain(mc_rows)
            .chain(mythic_rows)
            .chain(ue_rows)
            .chain(umwp_rows)
            .collect();
        // SD-31 wave-24 (integration cycle, W24-INTEGRATE): a later-chained
        // book can genuinely reprint an earlier book's spell verbatim (e.g.
        // Bestiary 6's two Scalykind-subdomain spells are also printed,
        // word-for-word, inside Ultimate Wilderness's own `uw_spells.lst` --
        // see `rules_tables::bestiary_6::spell_list`'s doc comment). Serving
        // the same `key` twice broke `no_key_is_served_twice_so_a_selection_
        // resolves_unambiguously` (apps/desktop/src-tauri's own product
        // invariant: the catalog browser and picker key off spell name
        // alone, with no book qualifier in the selection path), so this is
        // the SAME general policy `ultimate_combat::spell_list` already
        // applies by omission for `Share Language (Communal)` (that book's
        // own ingest simply never included the thinner duplicate) --
        // generalized here as a resolver-level rule instead of a per-ingest
        // hand omission, so it also protects every future book widening.
        // First-chained wins (book declaration order above), which for
        // every existing collision keeps the earlier-registered book's row.
        // This is a general, book-agnostic dedup -- it does not name
        // Bestiary 6 or any other book -- and it is a no-op for every book
        // pair that does not collide (confirmed: the pre-B6 catalog carried
        // zero cross-book key collisions, so this dedup changes nothing for
        // the ten books that were already chained here).
        let mut seen = std::collections::HashSet::new();
        chained.into_iter().filter(|row| seen.insert(row.key)).collect()
    })
}

pub fn spell_id_resolve<'a>(
    spell_id: &str,
    rule_set: RuleSetId,
    corpus: &SourcePackageContent<'a>,
) -> Option<(&'a LstSpellRecord, Option<TableCellRef>)> {
    for record in corpus.records_by_kind(SourceContentKind::Spell) {
        if let SourceContentPayload::Spell(spell) = record.payload
            && spell.name == spell_id
        {
            let table_cell = SPELL_LIST
                .iter()
                .find(|entry| entry.key == spell_id)
                .map(|_| TableCellRef {
                    rule_set,
                    table: "spell_list".to_string(),
                    row_key: spell_id.to_string(),
                    column_key: String::new(),
                });
            return Some((spell, table_cell));
        }
    }
    None
}

#[cfg(test)]
mod spell_catalog_rows_tests {
    use super::*;
    use crate::rules_core::codex_neutral_name;

    /// SD31-E6-F10-001: Inner Sea Gods is the ninth book chained into
    /// `spell_catalog_rows()`. A real, non-empty row set proves the chain
    /// actually wired the new book in, not merely that the constant
    /// compiles.
    #[test]
    fn inner_sea_gods_is_chained_into_the_catalog() {
        let isg_rows: Vec<&SpellCatalogRow> =
            spell_catalog_rows().iter().filter(|row| row.book == SPELL_BOOK_ISG).collect();
        assert!(!isg_rows.is_empty(), "expected at least one Inner Sea Gods spell row");
        assert!(
            isg_rows.iter().any(|row| row.key == "Blade Snare"),
            "expected the real corpus row \"Blade Snare\" (isg_spells.lst) among Inner Sea \
             Gods rows: {isg_rows:?}"
        );
    }

    /// SD-31 wave-29 (`lane5-book-onboard` lane): Adventurer's Guide is the
    /// twelfth book chained into `spell_catalog_rows()`, and this book's
    /// FIRST compiled rule set of any kind. A real, non-empty row set
    /// proves the chain actually wired the new book in, not merely that
    /// the constant compiles.
    #[test]
    fn adventurers_guide_is_chained_into_the_catalog() {
        let ag_rows: Vec<&SpellCatalogRow> =
            spell_catalog_rows().iter().filter(|row| row.book == SPELL_BOOK_AG).collect();
        assert!(!ag_rows.is_empty(), "expected at least one Adventurer's Guide spell row");
        assert!(
            ag_rows.iter().any(|row| row.key == "Bone Flense"),
            "expected the real corpus row \"Bone Flense\" (ag_spells.lst) among Adventurer's \
             Guide rows: {ag_rows:?}"
        );
    }

    /// PI screening at ingest time must have actually run, not merely
    /// compiled: `inner_sea_gods`'s 4 deity-name-blacklisted records must
    /// never appear in the shipped catalog under their real, PI-carrying
    /// names. `decisions.md §24` (SD-32): these records are no longer
    /// dropped whole -- they ship under a Codex-generated neutral identity
    /// (`ingest_spells.rs`'s `pi_screen`) instead, so this test asserts
    /// the ORIGINAL name never ships (the binding claim), not that the
    /// record is absent. The 4 real names are deliberately NOT written
    /// here (`decisions.md §24b`-2: "the PI original appears nowhere...
    /// not in a test") -- checked by class (the marker prefix) and by
    /// count instead.
    #[test]
    fn inner_sea_gods_never_ships_a_deity_possessive_name_unrenamed() {
        let isg_rows: Vec<_> =
            spell_catalog_rows().iter().filter(|row| row.book == SPELL_BOOK_ISG).cloned().collect();
        let renamed = isg_rows
            .iter()
            .filter(|row| row.key.starts_with(codex_neutral_name::NAME_PREFIX))
            .count();
        assert_eq!(
            renamed, 4,
            "expected exactly 4 inner_sea_gods records under a Codex-generated neutral \
             identity (the deity-possessive names): {:?}",
            isg_rows.iter().map(|r| r.key).collect::<Vec<_>>()
        );
    }

    /// SD31-E6-F2-002: Ultimate Magic is the sixth book chained into
    /// `spell_catalog_rows()`, the widening this cycle's own dispatch names
    /// as its primary deliverable. A real, non-empty row set proves the
    /// chain actually wired the new book in, not merely that the constant
    /// compiles.
    #[test]
    fn ultimate_magic_is_chained_into_the_catalog() {
        let um_rows: Vec<&SpellCatalogRow> =
            spell_catalog_rows().iter().filter(|row| row.book == SPELL_BOOK_UM).collect();
        assert!(!um_rows.is_empty(), "expected at least one Ultimate Magic spell row");
        assert!(
            um_rows.iter().any(|row| row.key == "Acidic Spray"),
            "expected the real corpus record 'Acidic Spray' among Ultimate Magic's rows"
        );
    }

    /// A record this cycle's own ingest confirmed carries neither `CLASSES:`
    /// nor `DOMAINS:` (`Restore Eidolon`) must ship with `level: None`, never
    /// a fabricated level -- the no-stub-mvp doctrine applied to this
    /// specific, named corpus gap.
    #[test]
    fn a_um_record_with_no_classes_or_domains_token_carries_no_level() {
        let row = spell_catalog_rows()
            .iter()
            .find(|row| row.book == SPELL_BOOK_UM && row.key == "Restore Eidolon")
            .expect("Restore Eidolon must be present in the UM catalog");
        assert_eq!(row.level, None);
    }

    /// SD31-E6-F2-003: Occult Adventures is the seventh book chained into
    /// `spell_catalog_rows()`.
    #[test]
    fn occult_adventures_is_chained_into_the_catalog() {
        let oa_rows: Vec<&SpellCatalogRow> =
            spell_catalog_rows().iter().filter(|row| row.book == SPELL_BOOK_OA).collect();
        assert!(!oa_rows.is_empty(), "expected at least one Occult Adventures spell row");
        assert!(
            oa_rows.iter().any(|row| row.key == "Akashic Form"),
            "expected the real corpus record 'Akashic Form' among Occult Adventures's rows"
        );
    }

    /// A record this cycle's own ingest confirmed carries neither `CLASSES:`
    /// nor `DOMAINS:` (`Talismanic Implement`) must ship with `level: None`,
    /// never a fabricated level.
    #[test]
    fn an_oa_record_with_no_classes_token_carries_no_level() {
        let row = spell_catalog_rows()
            .iter()
            .find(|row| row.book == SPELL_BOOK_OA && row.key == "Talismanic Implement")
            .expect("Talismanic Implement must be present in the OA catalog");
        assert_eq!(row.level, None);
    }

    /// SD31-E6-F2-004: Ultimate Combat is the eighth book chained into
    /// `spell_catalog_rows()`.
    #[test]
    fn ultimate_combat_is_chained_into_the_catalog() {
        let uc_rows: Vec<&SpellCatalogRow> =
            spell_catalog_rows().iter().filter(|row| row.book == SPELL_BOOK_UC).collect();
        assert!(!uc_rows.is_empty(), "expected at least one Ultimate Combat spell row");
        assert!(
            uc_rows.iter().any(|row| row.key == "Ablative Barrier"),
            "expected the real corpus record 'Ablative Barrier' among Ultimate Combat's rows"
        );
    }

    /// A record this cycle's own ingest confirmed carries neither `CLASSES:`
    /// nor `DOMAINS:` (`Life Conduit`) must ship with `level: None`, never a
    /// fabricated level.
    #[test]
    fn a_uc_record_with_no_classes_token_carries_no_level() {
        let row = spell_catalog_rows()
            .iter()
            .find(|row| row.book == SPELL_BOOK_UC && row.key == "Life Conduit")
            .expect("Life Conduit must be present in the UC catalog");
        assert_eq!(row.level, None);
    }

    /// `Share Language (Communal)` is a genuine cross-book collision: OA's
    /// own ingest (`SD31-E6-F2-003`) already declared it as a real new OA
    /// spell, and UC's own `uc_spells.lst` independently declares the same
    /// bare name with no `SCHOOL:`/`CLASSES:` of its own -- UC's thinner
    /// duplicate must be skipped, keeping OA's fuller record as the single
    /// shipped entry for that key.
    #[test]
    fn share_language_communal_is_served_once_from_oa_not_duplicated_from_uc() {
        let matches: Vec<&SpellCatalogRow> =
            spell_catalog_rows().iter().filter(|row| row.key == "Share Language (Communal)").collect();
        assert_eq!(matches.len(), 1, "expected exactly one served row for this cross-book collision");
        assert_eq!(matches[0].book, SPELL_BOOK_OA, "OA's fuller record must be the one that ships");
    }

    /// SD-31 wave-24 (`bestiary_6` lane, corrected by the W24-INTEGRATE
    /// cycle): `bestiary_6::spell_list::SPELL_LIST` really does carry 2
    /// entries and both really do compile into `SPELL_BOOK_B6`-tagged rows
    /// upstream of the dedup pass -- the constant is not dead weight. But
    /// both of this book's rows are verbatim reprints of spells Ultimate
    /// Wilderness already ships (see `bestiary_6::spell_list`'s doc
    /// comment), so the cross-book dedup pass in `spell_catalog_rows()`
    /// (first-chained book wins; UW is chained before B6) suppresses both
    /// -- `SPELL_BOOK_B6` ships zero rows into the *served* catalog today.
    /// That is the correct, general outcome, not a bug: see
    /// `bestiary_6_reprints_are_served_once_from_uw_not_duplicated_from_b6`
    /// below for the invariant this test would otherwise have broken
    /// (`no_key_is_served_twice_so_a_selection_resolves_unambiguously`,
    /// apps/desktop/src-tauri/src/spell_catalog.rs). A future book that adds
    /// a spell Bestiary 6 alone prints would show up here as a non-empty,
    /// non-colliding `SPELL_BOOK_B6` row.
    #[test]
    fn bestiary_6_book_code_is_registered_but_contributes_no_served_rows_today() {
        let b6_rows: Vec<&SpellCatalogRow> =
            spell_catalog_rows().iter().filter(|row| row.book == SPELL_BOOK_B6).collect();
        assert_eq!(
            b6_rows.len(),
            0,
            "both of Bestiary 6's spell rows are verbatim UW reprints and must be suppressed \
             by the cross-book dedup pass, not served under their own book code: {b6_rows:?}"
        );
        assert_eq!(bestiary_6::spell_list::SPELL_LIST.len(), 2, "the book's own table is unchanged");
    }

    /// Bestiary 6's own two spells are ALSO reprinted verbatim inside
    /// Ultimate Wilderness's `uw_spells.lst` (`bestiary_6::spell_list`'s own
    /// doc comment) -- a genuine cross-book reprint, not the "thinner
    /// duplicate" shape `share_language_communal_...` above pins (both
    /// copies are fully populated). Decision 10's Supersession Register is
    /// proposed, not applied, so this test does not attempt that ruling;
    /// it only pins the interim, conservative default this integration
    /// cycle chose to protect the pre-existing `no_key_is_served_twice_...`
    /// product invariant: the earlier-chained book (UW, registered in wave
    /// 19) is the one that ships, and the later one (B6, wave 24) is
    /// suppressed as a duplicate rather than shipped twice.
    #[test]
    fn bestiary_6_reprints_are_served_once_from_uw_not_duplicated_from_b6() {
        for key in ["Animal Growth (Reptiles Only)", "Animal Shapes (Reptiles Only)"] {
            let books: Vec<&str> = spell_catalog_rows()
                .iter()
                .filter(|row| row.key == key)
                .map(|row| row.book)
                .collect();
            assert_eq!(books, vec![SPELL_BOOK_UW], "{key} must ship exactly once, from UW");
        }
    }
}
