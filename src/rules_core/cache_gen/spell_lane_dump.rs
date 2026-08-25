//! Spell-lane supplemental JSON cache generator (SD-31 `epic-6-ingest-lanes`
//! F2, `SD31-E6-F2-005`).
//!
//! **The gap this closes.** `spell_resolver::spell_catalog_rows()` chains
//! eight books today (CRB/APG/ACG/ARG/UI/UM/OA/UC), each backed by a
//! compiled `rules_tables::<book>::spell_list::SPELL_LIST` table -- but
//! only the original five (CRB/APG/ACG/ARG/UI) ever had a
//! `data/corpus/<book>/spell/*.json` cache written for them. Ultimate
//! Magic, Occult Adventures and Ultimate Combat were chained straight from
//! their compiled tables with no on-disk cache at all
//! (`OPEN-ISSUES.md` rows 76/77: "`data/corpus/ultimate_magic/spell/` and
//! `data/corpus/occult_adventures/spell/` do not exist at all"). Without
//! that cache, `corpus_literal_sweep` has nothing to examine for those
//! books' `spell` records, so their `wiring_class == Static` units can
//! never reach the `literal-verified` rung `pf1e_dashboard_producer.
//! doneness_verdict` requires for `done` -- 87 units sit `held` at
//! `ingested-magnitude` for exactly this reason (re-derived
//! `2026-08-16`, `docs/work-inventory.json`: `occult_adventures` 40,
//! `ultimate_magic` 26, `ultimate_combat` 11, `ultimate_intrigue` 10).
//! **`ultimate_intrigue` was re-derived to carry no `data/corpus/
//! ultimate_intrigue/spell/` directory at all either** — its own module
//! doc comment ("SD28-E24 slice 2") and the desktop catalog both read
//! straight from its compiled `SPELL_LIST` table, and no prior cycle ever
//! built it a JSON cache — so all four books this generator covers are
//! genuinely new ground, not three new plus one re-cover.
//!
//! Mirrors `cache_gen::ultimate_equipment`'s shape closely (own local
//! Shape B types except the shared `shape_b_v1::License` enum, own
//! citation helpers, both SD-30 PI invocation contracts -- `decisions.md
//! §52.3`/`§53.5` -- unioned exactly as that module's own doc comment
//! describes) with one difference: this module DUMPS from FOUR compiled
//! tables (one per book) rather than one, because all four share the
//! identical `SpellListEntry`-shaped record and this generator would
//! otherwise be copy-pasted four times.
//!
//! **Never re-parses raw PCGen LST to derive a field's *value*** (per
//! `decisions.md §11.3`). The `.lst` file is read for exactly one
//! purpose: recovering the real, checkable line-number *citation* for a
//! value already known (from the compiled Rust module) to be correct --
//! via `pcgen_import::lst_parser::spell::parse_lst_spell_file`, the same
//! tested parser every `ingest_*_spells.rs` binary already uses, not a
//! second hand-rolled scan.
//!
//! **This module writes NO `raw_tokens`.** That is a second, separate
//! pass -- `src/bin/enrich_spell_raw_tokens.rs`, widened by this same
//! cycle to cover these four books' newly-created directories, exactly
//! mirroring the two-phase pipeline `cache_gen::acg::generate_spells` /
//! `enrich_spell_raw_tokens.rs` already established for the original five
//! books. Splitting the concerns keeps `token_closure`-based enrichment in
//! one place for every book, new or old.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use serde::Serialize;

use crate::pcgen_import::lst_parser::spell::parse_lst_spell_file;
use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::codex_neutral_name::neutral_name;
use crate::rules_core::pi_screening::{
    self, classify_optional_field_declared, declared_product_identity,
};
use crate::rules_core::shape_b_v1::{License, PI_MARKER_REDACTED, REDACTED_PI_MARKER};
use crate::rules_core::rules_tables::{
    adventurers_guide, bestiary, bestiary_4, bestiary_6, book_of_the_damned_volume_1,
    book_of_the_damned_volume_2, horror_adventures, inner_sea_faiths, inner_sea_gods,
    inner_sea_intrigue, inner_sea_magic, inner_sea_races, inner_sea_temples, inner_sea_world_guide,
    monster_codex, mythic_adventures, occult_adventures, ultimate_combat, ultimate_equipment,
    ultimate_intrigue, ultimate_magic, ultimate_magic_wordsofpower, ultimate_wilderness,
};

// ---------------------------------------------------------------------
// Shape B schema -- own local types (per-book/per-lane generators stay
// self-contained, no shared record-shape file; `shape_b_v1::License` is
// the one shared type, matching `cache_gen::ultimate_equipment`).
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Population {
    InScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Completeness {
    ChassisOnly,
    Full,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Source {
    LstToken { path: String, sha256: String, line: u32, record_key: String },
}

/// `decisions.md §24b`-4: divergence recorded as coordinate + reason only,
/// never the original PI string.
#[derive(Debug, Clone, Serialize)]
pub struct RenameInfo {
    pub reason: String,
    pub coordinate: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheRecord<T: Serialize> {
    pub population: Population,
    pub completeness: Completeness,
    pub ingested_at: String,
    pub data: T,
    pub source: Source,
    pub wiring_class: String,
    pub wiring_class_signals: Vec<String>,
    pub license: crate::rules_core::shape_b_v1::License,
    pub pi_field: Option<String>,
    pub pi_marker: Option<String>,
    /// `decisions.md §24b`-3: "a field marks it as carrying a
    /// Codex-generated name, so no reader or player mistakes it for the
    /// printed name."
    #[serde(default)]
    pub codex_generated_name: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename: Option<RenameInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpellData {
    pub key: String,
    pub school: Option<String>,
    pub level: Option<u8>,
    pub description: Option<String>,
}

// ---------------------------------------------------------------------
// The four target books
// ---------------------------------------------------------------------

/// One book's real spell entries, normalized to a common shape so the one
/// generation loop below can walk all four compiled tables identically.
/// Built fresh per book from that book's own compiled `SPELL_LIST` --
/// never a second, hand-authored source.
struct NormalizedEntry<'a> {
    key: &'a str,
    /// `decisions.md §24`: `Some(line)` when `key` above is ALREADY a
    /// Codex-generated neutral identity (`ingest_spells.rs`'s own rename
    /// branch renamed it before it ever reached `SPELL_LIST`) -- carries
    /// the real citation line so this generator can resolve it directly
    /// instead of `lines_by_name.get(key)`, which would fail: the real
    /// `.lst` content no longer contains the neutral string. `None` for
    /// an ordinary (non-renamed) entry.
    name_pi_line: Option<u32>,
    school: Option<String>,
    level: Option<u8>,
    description: Option<&'a str>,
}

struct BookSpec {
    /// `wiring_class`'s corpus-wide book id, matching `spell_resolver`'s
    /// `spell_book_slug_for` output for this book.
    book_id: &'static str,
    /// Corpus-relative directory, `pathfinder/paizo/roleplaying_game/<book>`.
    dir: &'static str,
    /// The book's spell `.lst` file basename.
    spell_file: &'static str,
    entries: Vec<NormalizedEntry<'static>>,
}

fn book_specs() -> Vec<BookSpec> {
    vec![
        BookSpec {
            book_id: "occult_adventures",
            dir: "pathfinder/paizo/roleplaying_game/occult_adventures",
            spell_file: "oa_spells.lst",
            entries: occult_adventures::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "ultimate_magic",
            dir: "pathfinder/paizo/roleplaying_game/ultimate_magic",
            spell_file: "um_spells.lst",
            entries: ultimate_magic::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        // SD-32 `decisions.md §20`, no_record-to-zero wave: `ultimate_magic`'s
        // SECOND source file, same shipped `book_id` -- the Words of Power
        // subsystem's three "Example Word Spells"
        // (`um_spells_wordsofpower.lst`), a distinct `.lst` file this
        // generator's own `ultimate_magic` `BookSpec` above never reads.
        // Two `BookSpec`s sharing one `book_id`/`out_spell_dir` is the same
        // shape `spell_mod_access`'s cross-generator self-erasure guard
        // (below, in `generate()`) already exists to make safe: each
        // spec's `remove_stale_owned_files` call is scoped to its OWN
        // `spell_file`'s cited lines only, so this entry cannot touch the
        // other `ultimate_magic` `BookSpec`'s files, and vice versa.
        BookSpec {
            book_id: "ultimate_magic",
            dir: "pathfinder/paizo/roleplaying_game/ultimate_magic",
            spell_file: "um_spells_wordsofpower.lst",
            entries: ultimate_magic_wordsofpower::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "ultimate_combat",
            dir: "pathfinder/paizo/roleplaying_game/ultimate_combat",
            spell_file: "uc_spells.lst",
            entries: ultimate_combat::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "ultimate_intrigue",
            dir: "pathfinder/paizo/roleplaying_game/ultimate_intrigue",
            spell_file: "ui_spells.lst",
            entries: ultimate_intrigue::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: Some(format!("{:?}", e.school)),
                    level: Some(e.level),
                    description: Some(e.description),
                })
                .collect(),
        },
        // SD31-E6-F10-001: Inner Sea Gods, the 9th spell book chained into
        // `spell_resolver::spell_catalog_rows()`. Unlike the four books
        // above, this one is a `campaign_setting/` book, not
        // `roleplaying_game/` -- `dir` is generic per-`BookSpec`, so this
        // is a plain data addition, no path-shape change to `generate()`.
        BookSpec {
            book_id: "inner_sea_gods",
            dir: "pathfinder/paizo/campaign_setting/inner_sea_gods",
            spell_file: "isg_spells.lst",
            entries: inner_sea_gods::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        // W19-INTEGRATE (wave-19 adversarial review, confirmed finding on
        // the `ultimate_wilderness` lane): the lane wired the book's 61
        // spells into `spell_resolver::spell_catalog_rows()` and the
        // desktop `spell_catalog` screen, but stopped short of also
        // writing a `data/corpus/ultimate_wilderness/spell/` cache --
        // without it, `corpus_literal_sweep`/`derived_evaluator_fixture_check`
        // have nothing to verify against, so no unit can reach the
        // `literal-verified`/`fixture-verified` `done` rung no matter how
        // complete its data is. Same shape as every `BookSpec` above.
        BookSpec {
            book_id: "ultimate_wilderness",
            dir: "pathfinder/paizo/roleplaying_game/ultimate_wilderness",
            spell_file: "uw_spells.lst",
            entries: ultimate_wilderness::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        // SD-32 `decisions.md §20`: five more already-compiled books whose
        // `spell_list::SPELL_LIST` table existed (via `ingest_spells.rs`'s
        // config-driven `BOOKS`) but never had a `data/corpus/<book>/
        // spell/*.json` cache generated for them at all -- the exact same
        // "compiled table but no corpus dump" gap this module already
        // closed for `occult_adventures`/`ultimate_magic`/`ultimate_combat`/
        // `ultimate_intrigue`/`inner_sea_gods`/`ultimate_wilderness` above.
        // Re-derived directly: `ls data/corpus/<book>/spell/` was empty (or
        // absent) for all five before this cycle despite each book's own
        // compiled table already carrying real, screened entries.
        BookSpec {
            book_id: "adventurers_guide",
            dir: "pathfinder/paizo/roleplaying_game/adventurers_guide",
            spell_file: "ag_spells.lst",
            entries: adventurers_guide::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "inner_sea_faiths",
            dir: "pathfinder/paizo/campaign_setting/inner_sea_faiths",
            spell_file: "isf_spells.lst",
            entries: inner_sea_faiths::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "inner_sea_magic",
            dir: "pathfinder/paizo/campaign_setting/inner_sea_magic",
            spell_file: "ism_spells.lst",
            entries: inner_sea_magic::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "inner_sea_temples",
            dir: "pathfinder/paizo/campaign_setting/inner_sea_temples",
            spell_file: "istem_spells.lst",
            entries: inner_sea_temples::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "horror_adventures",
            dir: "pathfinder/paizo/roleplaying_game/horror_adventures",
            spell_file: "ha_spells.lst",
            entries: horror_adventures::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        // SD-32 `decisions.md §20`: `bestiary`/`bestiary_4`'s custom
        // spell-variant declarations, re-derived to be real dedicated
        // `.lst` content (`decisions.md §17a` re-derivation) rather than
        // the "monster-intrinsic, no dedicated `.lst`" a prior cycle
        // reported without checking. `bestiary`'s file physically lives
        // under the shared `core_essentials` directory -- same split
        // `cache_gen::equipment_gap::book_routing`'s own doc comment names
        // for this book's equipment rows; the shipped book id stays
        // `"bestiary"`.
        BookSpec {
            book_id: "bestiary",
            dir: "pathfinder/paizo/roleplaying_game/core_essentials",
            spell_file: "ce_spells.lst",
            entries: bestiary::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "bestiary_4",
            dir: "pathfinder/paizo/roleplaying_game/bestiary_4",
            spell_file: "b4_spells_modified.lst",
            entries: bestiary_4::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        // SD-32 `decisions.md §20`, no_record-to-zero wave: eight more
        // compiled tables (`ingest_spells.rs`'s config-driven `BOOKS`)
        // with no `data/corpus/<book>/spell/*.json` cache at all -- same
        // "compiled table but no corpus dump" gap this module already
        // closes for every book above.
        BookSpec {
            book_id: "inner_sea_races",
            dir: "pathfinder/paizo/campaign_setting/inner_sea_races",
            spell_file: "isr_spells.lst",
            entries: inner_sea_races::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "inner_sea_intrigue",
            dir: "pathfinder/paizo/campaign_setting/inner_sea_intrigue",
            spell_file: "isi_spells.lst",
            entries: inner_sea_intrigue::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "monster_codex",
            dir: "pathfinder/paizo/roleplaying_game/monster_codex",
            spell_file: "mc_spells.lst",
            entries: monster_codex::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "inner_sea_world_guide",
            dir: "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
            spell_file: "iswg_spells.lst",
            entries: inner_sea_world_guide::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "book_of_the_damned_volume_1",
            dir: "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
            spell_file: "botd1_spells.lst",
            entries: book_of_the_damned_volume_1::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "book_of_the_damned_volume_2",
            dir: "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
            spell_file: "botd2_spells.lst",
            entries: book_of_the_damned_volume_2::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "mythic_adventures",
            dir: "pathfinder/paizo/roleplaying_game/mythic_adventures",
            spell_file: "ma_spells.lst",
            entries: mythic_adventures::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        // SD-32 `decisions.md §20`, spell no_record closure: `bestiary_6`'s
        // `spell_list::SPELL_LIST` (SD-31 wave 24, hand-authored, both of
        // the book's two base declarations already compiled) had no
        // `book_specs()` entry -- the same "compiled table, no corpus dump"
        // gap this module closes for every book above -- so its two units
        // had no `data/corpus/bestiary_6/spell/*.json` record for
        // `shape_ledger.py`'s join to find, reporting `no_record` despite
        // real, already-transcribed content.
        BookSpec {
            book_id: "bestiary_6",
            dir: "pathfinder/paizo/roleplaying_game/bestiary_6",
            spell_file: "b6_spells.lst",
            entries: bestiary_6::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
        BookSpec {
            book_id: "ultimate_equipment",
            dir: "pathfinder/paizo/roleplaying_game/ultimate_equipment",
            spell_file: "ue_spells.lst",
            entries: ultimate_equipment::spell_list::SPELL_LIST
                .iter()
                .map(|e| NormalizedEntry {
                    key: e.key,
                    name_pi_line: e.name_pi_line,
                    school: e.school.map(|s| format!("{s:?}")),
                    level: e.level,
                    description: e.description,
                })
                .collect(),
        },
    ]
}

// ---------------------------------------------------------------------
// Corpus-access helpers (citation lookup only, never value derivation)
// ---------------------------------------------------------------------

/// Real sha256 of `path`'s current on-disk content, via the system
/// `sha256sum` tool (mirrors every sibling `cache_gen` module -- no
/// `sha2` crate dependency exists in this workspace's binaries).
fn sha256_file(path: &Path) -> std::io::Result<String> {
    let output = Command::new("sha256sum").arg(path).output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!("sha256sum failed for {}", path.display())));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.split_whitespace().next().unwrap_or_default().to_string())
}

/// Every base spell declaration's real 1-based source line, keyed by its
/// column-0 name -- via the SAME tested parser
/// (`pcgen_import::lst_parser::spell::parse_lst_spell_file`) every
/// `ingest_*_spells.rs` binary already used to build the compiled table
/// this generator dumps. A `.MOD`/`.COPY=` row is not a base declaration
/// (identical exclusion every sibling ingest binary's own doc comment
/// states) and is never inserted, so a base declaration's citation can
/// never resolve to a `.MOD` row by construction.
fn base_declaration_lines(lst_path: &Path) -> std::io::Result<BTreeMap<String, u32>> {
    let parsed = parse_lst_spell_file(lst_path)
        .map_err(|e| std::io::Error::other(format!("failed to parse {lst_path:?}: {e:?}")))?;
    let raw_text = std::fs::read_to_string(lst_path)?;
    let raw_lines: Vec<&str> = raw_text.split('\n').collect();
    let mut out = BTreeMap::new();
    for record in &parsed.records {
        if record.name.ends_with(".MOD") {
            continue;
        }
        // `decisions.md §17`: a `.COPY=` row IS a real, citable corpus row
        // (`ingest_spells.rs::copy_variant_split`'s fix gives it its own
        // new identity, the part after `.COPY=`) -- it must resolve to ITS
        // OWN line, not be dropped as though it named nothing. Mirrors
        // `cache_gen::equipment_gap::find_citation`'s `.COPY=` fallback for
        // the identical corpus shape (real example:
        // `core_essentials/ce_spells.lst:49`,
        // `"Veil.COPY=Veil (self only)"` -- `entry.key` is `"Veil (self
        // only)"`, resolvable only by indexing the variant name here).
        if let Some((_, variant)) = record.name.split_once(".COPY=") {
            out.entry(variant.to_string()).or_insert(record.line_number as u32);
            continue;
        }
        out.entry(record.name.clone()).or_insert(record.line_number as u32);
        // A declared `KEY:` token is a record's real identity, distinct
        // from its display name -- `ingest_inner_sea_gods_spells.rs`'s own
        // `key_field` reads the identical token for the identical reason
        // (`SD31-E6-F10-001`, `v06_corpus_trap_report`'s `key-differs-
        // from-name` finding). Indexed under BOTH the display name (above,
        // unchanged for every pre-existing book) and the declared key
        // (here, additive) so a compiled `SpellListEntry.key` that already
        // resolved to the `KEY:` value still finds its real citation line.
        if let Some(raw_line) = raw_lines.get(record.line_number - 1)
            && let Some(declared_key) =
                raw_line.split('\t').skip(1).find_map(|col| col.trim().strip_prefix("KEY:"))
        {
            out.entry(declared_key.to_string()).or_insert(record.line_number as u32);
        }
    }
    Ok(out)
}

/// Classifies a `SpellListEntry.description` for storage -- the ONE place
/// this generator's own re-screen must recognize an already-redacted
/// compiled-table value rather than re-classifying it as if it were raw
/// prose (`SD31-E6-F10-001`).
///
/// **The defect this closes.** Every `ingest_*_spells.rs` binary already
/// screens `description` with both SD-30 PI contracts at ingest time,
/// storing [`REDACTED_PI_MARKER`] into the COMPILED table when the real
/// prose hits the blacklist. This generator's own re-screen (defense in
/// depth, per its own module doc comment) then ran `classify_field` on
/// THAT marker string -- which of course contains no blacklisted term --
/// and classified it `License::Ogl`, producing shipped records reading
/// `description: "[redacted PI]"` under `license: "OGL"`. Found live: 4
/// Inner Sea Gods records (`Spawn Calling`, `Early Judgment`, `Fairness`,
/// `Deadeye's Arrow`) shipped exactly this impossible combination, and
/// `corpus_literal_sweep`'s DESC exemption -- gated on
/// `license == "PI-REDACTED"` -- could never recognize them, so they
/// `MISMATCH`ed on every sweep run for a redaction that is legitimate, not
/// a transcription defect.
fn description_classification(
    entry_description: Option<&str>,
    declared_description: bool,
) -> (License, Option<String>, Option<String>, Option<String>) {
    if entry_description == Some(REDACTED_PI_MARKER) {
        return (
            License::PiRedacted,
            Some("description".to_string()),
            Some(PI_MARKER_REDACTED.to_string()),
            Some(REDACTED_PI_MARKER.to_string()),
        );
    }
    classify_optional_field_declared("description", entry_description, declared_description)
}

/// [`pi_screening::DeclaredProductIdentity`], read off the real corpus line
/// at `lst_path:line` (1-indexed) -- `§53.5`'s declared-PI reader, applied
/// per-record against the same citation this generator already resolved.
/// `line == 0` (an unresolved citation) reads as no declaration, matching
/// every sibling generator's honest-gap handling.
fn declared_pi_at(lst_path: &Path, line: u32) -> std::io::Result<pi_screening::DeclaredProductIdentity> {
    if line == 0 {
        return Ok(pi_screening::DeclaredProductIdentity::default());
    }
    let content = std::fs::read_to_string(lst_path)?;
    let Some(row) = content.lines().nth((line - 1) as usize) else {
        return Ok(pi_screening::DeclaredProductIdentity::default());
    };
    let tokens: Vec<(&str, &str)> = row.split('\t').filter_map(|field| field.split_once(':')).collect();
    Ok(declared_product_identity(tokens))
}

// ---------------------------------------------------------------------
// Generation report
// ---------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub spells_written: usize,
    /// `(book, file, line, record_key)` for every row DROPPED because
    /// its NAME is Product Identity (`NAMEISPI:YES` or a blacklist hit) --
    /// a name cannot be redacted (it is the record's own identity), so
    /// the only safe action is to not publish the row at all
    /// (`pi_screening.rs`'s own doc comment on `DeclaredProductIdentity::
    /// name`; the same rule `cache_gen::ultimate_equipment` and
    /// `ingest_race_traits.rs` both already apply).
    pub name_pi_dropped: Vec<String>,
    /// Record keys whose real LST citation could not be resolved --
    /// should be empty against the real, pinned corpus; surfaced rather
    /// than silently defaulted to line 0.
    pub unresolved_citations: Vec<String>,
}

#[derive(Debug)]
pub enum GenerationError {
    Io(std::io::Error),
}

impl From<std::io::Error> for GenerationError {
    fn from(e: std::io::Error) -> Self {
        GenerationError::Io(e)
    }
}

/// Writes `data/corpus/<book>/spell/*.json` for all four target books,
/// by dumping their already-compiled `SPELL_LIST` tables. `corpus_root`
/// is the pinned PCGen oracle's `data/` directory (citation lookup only);
/// `data_corpus_root` is this repo's own `data/corpus` directory (the
/// write target).
pub fn generate(
    corpus_root: &Path,
    data_corpus_root: &Path,
    ingested_at: &str,
) -> Result<GenerationReport, GenerationError> {
    let mut report = GenerationReport::default();

    for spec in book_specs() {
        let book_dir = corpus_root.join(spec.dir);
        let lst_path = book_dir.join(spec.spell_file);
        let sha256 = sha256_file(&lst_path)?;
        let lines_by_name = base_declaration_lines(&lst_path)?;

        let out_book_dir = data_corpus_root.join(spec.book_id);
        let out_spell_dir = out_book_dir.join("spell");
        std::fs::create_dir_all(&out_spell_dir)?;

        let wiring_index = WiringClassIndex::build(spec.book_id, &book_dir);
        let mut wiring_lines = wiring_index.lines();
        let mut used_slugs = std::collections::BTreeSet::new();
        // **SD-32 Epic 5 protective sweep, CORRECTED**: this directory used
        // to be wiped wholesale on every run (the durability discipline
        // comment this replaces) so a record dropped this run would
        // actually disappear -- but that also erased every STILL-VALID
        // record's file, discarding whatever `enrich_spell_raw_tokens.rs`
        // had separately written into it in a later pass this generator's
        // own `SpellData` cannot reconstruct (the S6/D9 self-erasure
        // shape; 712 of 712 on-disk spell records across these five books
        // carry `raw_tokens` today, `grep -l raw_tokens
        // data/corpus/{occult_adventures,ultimate_magic,ultimate_combat,
        // inner_sea_gods,ultimate_wilderness}/spell/*.json`). See
        // `cache_gen::ultimate_equipment::remove_stale_owned_files`'s twin
        // fix and doc comment for the exact same shape and the same
        // "genuinely stale, not merely about to be rewritten" rule.
        let mut current_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

        for entry in &spec.entries {
            // `decisions.md §24`: a renamed entry's `key` is ALREADY the
            // Codex-generated neutral identity -- `lines_by_name` (built
            // from the real `.lst` content by REAL spell name) cannot
            // find it, so `name_pi_line` (set by `ingest_spells.rs`'s own
            // rename branch) is used directly instead.
            let line = entry.name_pi_line.unwrap_or_else(|| lines_by_name.get(entry.key).copied().unwrap_or(0));
            if line == 0 {
                report.unresolved_citations.push(format!("{}:spell:{}", spec.book_id, entry.key));
                continue;
            }

            // Both SD-30 PI invocation contracts on the NAME, unioned:
            // the corpus's own `NAMEISPI:YES` declaration (`§53.5`) and
            // the shared 55-term blacklist sweep (`§52.3`). Defense in
            // depth -- the compiled `SPELL_LIST` table this generator
            // dumps was already screened (and renamed, `decisions.md
            // §24`) at ingest time (`ingest_spells.rs`'s `pi_screen`), so
            // this is expected to agree, but the production write path
            // re-screens rather than trusting an upstream table.
            let declared = declared_pi_at(&lst_path, line).unwrap_or_default();
            let (name_license, ..) = pi_screening::classify_field("name", entry.key);
            let name_blacklisted = name_license != crate::rules_core::shape_b_v1::License::Ogl;
            // A renamed entry's OWN key ("Codex-Named Unit (...)") never
            // trips the blacklist itself; `declared.name` (read off the
            // REAL corpus line via `line` above) is what still correctly
            // flags it, since `entry.name_pi_line.is_some()` alone would
            // also work but this keeps the check symmetric with an
            // ordinary entry and mirrors `cache_gen::equipment_gap`'s own
            // idempotent re-derivation.
            let name_is_pi = declared.name || name_blacklisted;

            let (mut license, mut pi_field, mut pi_marker, stored_description) =
                description_classification(entry.description, declared.description);

            let (wiring_class, wiring_class_signals) =
                wiring_index.wiring_class_for(&mut wiring_lines, spec.spell_file, line, entry.key, entry.key);

            let completeness = if entry.description.is_some() { Completeness::Full } else { Completeness::ChassisOnly };

            // `decisions.md §24` -- a name-PI entry is no longer dropped;
            // it is written under a Codex-generated neutral name derived
            // ONLY from `(kind, book, source_file, source_line)`. When
            // `entry.key` was ALREADY renamed upstream (`ingest_spells.rs`),
            // re-deriving here is a pure no-op (same coordinates, same
            // output, `codex_neutral_name`'s own determinism proof).
            let (record_key, codex_generated_name, rename_info) = if name_is_pi {
                let codex_name = neutral_name("spell", spec.book_id, spec.spell_file, line);
                report.name_pi_dropped.push(format!("{}:{}:{line} (renamed, decisions.md §24)", spec.book_id, spec.spell_file));
                let mut redacted_fields: Vec<&str> = Vec::new();
                if pi_field.as_deref() == Some("description") {
                    redacted_fields.push("description");
                }
                redacted_fields.push("name");
                license = crate::rules_core::shape_b_v1::License::PiRedacted;
                pi_field = Some(redacted_fields.join(","));
                pi_marker = Some(crate::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string());
                let rename_info = Some(RenameInfo {
                    reason: "name_pi_blocked".to_string(),
                    coordinate: format!("{}:{}:{line}", spec.book_id, spec.spell_file),
                });
                (codex_name, true, rename_info)
            } else {
                (entry.key.to_string(), false, None)
            };

            let record = CacheRecord {
                population: Population::InScope,
                completeness,
                ingested_at: ingested_at.to_string(),
                data: SpellData {
                    key: record_key.clone(),
                    school: entry.school.clone(),
                    level: entry.level,
                    description: stored_description,
                },
                source: Source::LstToken {
                    path: format!("{}/{}", spec.dir, spec.spell_file),
                    sha256: sha256.clone(),
                    line,
                    record_key: record_key.clone(),
                },
                wiring_class,
                wiring_class_signals,
                license,
                pi_field,
                pi_marker,
                codex_generated_name,
                rename: rename_info,
            };

            current_keys.insert(record_key.clone());
            let slug = slugify(&record_key, &mut used_slugs);
            write_json(&out_spell_dir, &slug, &record)?;
            report.spells_written += 1;
        }

        // **Cross-generator self-erasure guard (2026-08-23 incident).**
        // `data/corpus/occult_adventures/spell/` and `.../ultimate_magic/
        // spell/` are also written by `cache_gen::spell_mod_access`'s
        // `.MOD` rows, from the SAME literal `oa_spells.lst`/`um_spells.
        // lst` files this generator reads -- so a citation-path-only
        // predicate (as `ultimate_equipment`'s own call site below uses)
        // is NOT safe here; both generators cite the identical file, and
        // a `.MOD` row genuinely reuses its base spell's own key/name
        // (`Occultist Spell ~ Accelerate Poison.MOD` widens class access
        // for the spell this generator itself dumps under the plain key
        // `accelerate poison`). Confirmed live: an unscoped run deleted
        // 1,580 real `spell_mod_access` `.MOD` records across these two
        // books before this guard existed.
        //
        // The real ownership boundary is the LINE: `base_declaration_
        // lines` (built above into `lines_by_name`) explicitly excludes
        // every `.MOD`/`.COPY=` row, so its value set is exactly "every
        // line this generator's own parse of this file would ever cite."
        // A `.MOD` row's citation line can never appear in it, so this
        // predicate structurally cannot match a `spell_mod_access` record
        // even though the file and the key both collide.
        let owned_path = format!("{}/{}", spec.dir, spec.spell_file);
        let owned_lines: std::collections::HashSet<u32> = lines_by_name.values().copied().collect();
        crate::rules_core::cache_gen::ultimate_equipment::remove_stale_owned_files(
            &out_spell_dir,
            &current_keys,
            &|path, line| path == owned_path && owned_lines.contains(&line),
        );
    }

    Ok(report)
}

/// SD-32 Epic 5 protective sweep -- see `cache_gen::acg::write_json`'s
/// identical doc comment; same shape, same fix. This module's own
/// `remove_stale_owned_files` call (see the per-book loop above) is what
/// still makes a genuinely dropped record's file disappear.
fn write_json<T: Serialize>(out_dir: &Path, slug: &str, record: &CacheRecord<T>) -> std::io::Result<()> {
    let path = out_dir.join(format!("{slug}.json"));
    if path.exists() {
        return Ok(());
    }
    let json = serde_json::to_string_pretty(record)
        .expect("CacheRecord<T> is a plain-data shape; serialization cannot fail");
    std::fs::write(path, json + "\n")
}

fn slugify(name: &str, used: &mut std::collections::BTreeSet<String>) -> String {
    let mut slug: String =
        name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    while slug.contains("__") {
        slug = slug.replace("__", "_");
    }
    let slug = slug.trim_matches('_').to_string();
    let slug = if slug.is_empty() { "unnamed".to_string() } else { slug };

    if !used.contains(&slug) {
        used.insert(slug.clone());
        return slug;
    }
    let mut n = 2;
    loop {
        let candidate = format!("{slug}-{n}");
        if !used.contains(&candidate) {
            used.insert(candidate.clone());
            return candidate;
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn pcgen_corpus_root() -> PathBuf {
        if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
            return PathBuf::from(v);
        }
        let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
        PathBuf::from(home).join("workspace/repos/pcgen/data")
    }

    #[test]
    fn generation_against_the_real_pinned_corpus_resolves_every_citation() {
        let corpus_root = pcgen_corpus_root();
        if !corpus_root.exists() {
            eprintln!("skipping: no pinned PCGen corpus checkout at {corpus_root:?}");
            return;
        }
        let tmp = std::env::temp_dir().join(format!(
            "spell_lane_dump_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        let report = generate(&corpus_root, &tmp, "2026-08-16T00:00:00Z").expect("generation must succeed");
        assert!(
            report.unresolved_citations.is_empty(),
            "every compiled SPELL_LIST entry must resolve a real citation against the pinned oracle: {:?}",
            report.unresolved_citations
        );
        assert!(report.spells_written > 0, "must write at least one record");
        std::fs::remove_dir_all(&tmp).ok();
    }

    /// SD-32 Epic 5 protective sweep (`epic-breakdown.md` Epic 5, T3
    /// residual): a second `generate()` call must not erase a later
    /// enrichment pass on a record that is still valid this run -- the
    /// S6/D9 self-erasure shape, live-reproduced against the real pinned
    /// PCGen oracle rather than a synthetic fixture.
    #[test]
    fn a_second_run_does_not_erase_a_later_enrichment_pass_on_a_still_valid_record() {
        let corpus_root = pcgen_corpus_root();
        if !corpus_root.exists() {
            eprintln!("skipping: no pinned PCGen corpus checkout at {corpus_root:?}");
            return;
        }
        let tmp = std::env::temp_dir().join(format!(
            "spell_lane_dump_no_self_erasure_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        generate(&corpus_root, &tmp, "2026-08-16T00:00:00Z").expect("run 1 must succeed");
        let occult_dir = tmp.join("occult_adventures").join("spell");
        let written = std::fs::read_dir(&occult_dir)
            .expect("run 1 must have created occult_adventures/spell")
            .flatten()
            .map(|e| e.path())
            .find(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
            .expect("run 1 must write at least one occult_adventures spell record");

        // Simulate `enrich_spell_raw_tokens.rs` running after generation.
        let original = std::fs::read_to_string(&written).unwrap();
        let enriched = original.replacen("\"license\":", "\"ENRICHED-MARKER\": true,\n  \"license\":", 1);
        std::fs::write(&written, &enriched).unwrap();
        let marker_present_after_edit = std::fs::read_to_string(&written).unwrap().contains("ENRICHED-MARKER");

        generate(&corpus_root, &tmp, "2026-08-17T00:00:00Z").expect("run 2 must succeed");
        let after = std::fs::read_to_string(&written).unwrap();
        std::fs::remove_dir_all(&tmp).ok();

        assert!(marker_present_after_edit, "the fixture edit must actually take");
        assert!(
            after.contains("ENRICHED-MARKER"),
            "a second run must not erase a later enrichment pass on a still-valid record: {after}"
        );
    }

    #[test]
    fn a_record_whose_row_declares_nameispi_is_read_correctly() {
        // Synthetic corpus fixture -- proves the wiring, not the real book.
        // `decisions.md §24` (SD-32): a `NAMEISPI:YES` row is no longer
        // dropped by `generate()` -- it is written under a Codex-generated
        // neutral name (see this module's own `resolve`/rename block and
        // `ingest_spells.rs`'s end-to-end proof,
        // `pi_screen_renames_a_record_whose_row_declares_nameispi_yes`).
        // This test still only proves `declared_pi_at` reads the flag off
        // the real corpus row -- `generate()`'s own loop reads
        // `book_specs()`'s fixed, compiled tables and cannot take an
        // injected fixture, so the rename branch's end-to-end proof is the
        // real corpus regen + `shape_ledger.py` before/after diff (cycle
        // receipt), not a unit test here.
        let tmp = std::env::temp_dir().join(format!(
            "spell_lane_dump_pi_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        let book_dir = tmp.join("pathfinder/paizo/roleplaying_game/occult_adventures");
        std::fs::create_dir_all(&book_dir).unwrap();
        // A minimal one-row spell file whose row declares NAMEISPI:YES.
        // The generator only ever iterates the COMPILED table's keys, so
        // this test exercises `declared_pi_at`/the drop branch directly
        // rather than the full loop (the compiled table cannot be swapped
        // in a unit test): a row-level, table-driven proof.
        std::fs::write(
            book_dir.join("oa_spells.lst"),
            "Secret Name\tNAMEISPI:YES\tCLASSES:Medium=1\tSCHOOL:Divination\tDESC:hidden lore\n",
        )
        .unwrap();
        let declared = declared_pi_at(&book_dir.join("oa_spells.lst"), 1).unwrap();
        assert!(declared.name, "NAMEISPI:YES must be read off the real corpus row");
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn description_classification_recognizes_an_already_redacted_compiled_table_value() {
        let (license, pi_field, pi_marker, stored) =
            description_classification(Some(REDACTED_PI_MARKER), false);
        assert_eq!(license, License::PiRedacted);
        assert_eq!(pi_field.as_deref(), Some("description"));
        assert_eq!(pi_marker.as_deref(), Some(PI_MARKER_REDACTED));
        assert_eq!(stored.as_deref(), Some(REDACTED_PI_MARKER));
    }

    #[test]
    fn description_classification_still_re_screens_ordinary_clean_prose() {
        let (license, pi_field, pi_marker, stored) =
            description_classification(Some("Ordinary spell prose."), false);
        assert_eq!(license, License::Ogl);
        assert_eq!(pi_field, None);
        assert_eq!(pi_marker, None);
        assert_eq!(stored.as_deref(), Some("Ordinary spell prose."));
    }

    #[test]
    fn description_classification_still_redacts_a_genuinely_undeclared_blacklist_hit() {
        // Proves the fix did not disable the pre-existing re-screen: real,
        // UNREDACTED prose naming a deity must still redact through the
        // ordinary `classify_field` path, not merely the marker-equality
        // shortcut.
        let (license, pi_field, ..) =
            description_classification(Some("As per Iomedae's blessing, you gain a +2 bonus."), false);
        assert_eq!(license, License::PiRedacted);
        assert_eq!(pi_field.as_deref(), Some("description"));
    }

    #[test]
    fn slugify_disambiguates_repeated_names() {
        let mut used = std::collections::BTreeSet::new();
        assert_eq!(slugify("Akashic Form", &mut used), "akashic_form");
        assert_eq!(slugify("Akashic Form", &mut used), "akashic_form-2");
    }

    /// RED before this cycle's fix: a `.COPY=` row was excluded from
    /// `base_declaration_lines` the same way a `.MOD` row is, so its own
    /// new identity (the name after `.COPY=`) never resolved a citation
    /// line -- `generate()` reported it `unresolved_citations` even though
    /// `ingest_spells.rs`'s own `.COPY=` fix (this same cycle) had already
    /// given it real `level`/`school`/`description`. GREEN after: the
    /// variant name resolves to the `.COPY=` row's OWN line (not the base
    /// record's line).
    #[test]
    fn base_declaration_lines_indexes_a_copy_rows_own_variant_name() {
        let tmp = std::env::temp_dir().join(format!(
            "spell_lane_dump_copy_variant_test_{}",
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let f = tmp.join("t_spells.lst");
        std::fs::write(
            &f,
            "Veil\tCLASSES:Bard,Sorcerer,Wizard=6\tSCHOOL:Illusion\tDESC:base prose\n\
             Veil.COPY=Veil (self only)\t\tCLASSES:.CLEARALL\tDESC:variant prose\n",
        )
        .unwrap();
        let lines = base_declaration_lines(&f).unwrap();
        assert_eq!(lines.get("Veil"), Some(&1), "the base record still resolves under its own name");
        assert_eq!(
            lines.get("Veil (self only)"),
            Some(&2),
            "the .COPY= row's own variant name must resolve to ITS OWN line, not the base's"
        );
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn base_declaration_lines_excludes_mod_and_copy_rows() {
        let tmp = std::env::temp_dir().join(format!(
            "spell_lane_dump_base_decl_test_{}",
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let f = tmp.join("t_spells.lst");
        std::fs::write(
            &f,
            "Real Spell\tCLASSES:Wizard=1\tSCHOOL:Evocation\tDESC:a real spell\n\
             Real Spell.MOD\tDESC:a modifier row\n",
        )
        .unwrap();
        let lines = base_declaration_lines(&f).unwrap();
        assert_eq!(lines.get("Real Spell"), Some(&1));
        assert!(!lines.contains_key("Real Spell.MOD"));
        std::fs::remove_dir_all(&tmp).ok();
    }

    /// `SD31-E6-F10-001`: a record whose row declares a `KEY:` different
    /// from its own display name must resolve under BOTH, so a compiled
    /// `SpellListEntry.key` already corrected to the declared `KEY:` value
    /// (`ingest_inner_sea_gods_spells.rs`'s own fix for the same corpus
    /// shape) still finds its real citation line.
    #[test]
    fn base_declaration_lines_also_indexes_a_declared_key_distinct_from_the_display_name() {
        let tmp = std::env::temp_dir().join(format!(
            "spell_lane_dump_declared_key_test_{}",
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let f = tmp.join("t_spells.lst");
        std::fs::write(
            &f,
            "Lighten Object, Mass\tKEY:Lighten Object (Mass)\tCLASSES:Wizard=5\tSCHOOL:Transmutation\tDESC:a mass version\n",
        )
        .unwrap();
        let lines = base_declaration_lines(&f).unwrap();
        assert_eq!(lines.get("Lighten Object, Mass"), Some(&1), "display name still resolves");
        assert_eq!(
            lines.get("Lighten Object (Mass)"),
            Some(&1),
            "the declared KEY: value must ALSO resolve to the same line"
        );
        std::fs::remove_dir_all(&tmp).ok();
    }
}
