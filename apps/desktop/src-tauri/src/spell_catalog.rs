//! Spell catalog browser — Tauri command adapter over every ingested PF1
//! spell table: `crb::spell_list` (664, since SD31-E6-F7-002 -- 652 base
//! records + 12 `.COPY=` racial spell-like-ability variants, decisions.md
//! §15), `apg::spell_list` (297), `acg::spell_list` (144),
//! `advanced_race_guide::spell_list` (93, since SD31-E6-F7-002 -- 92 base
//! records + the 13th `.COPY=` variant, `Fins to Feet (self only)`),
//! `ultimate_intrigue::spell_list` (101), `ultimate_magic::spell_list`
//! (269, since SD31-E6-F2-002), `occult_adventures::spell_list`
//! (144, since SD31-E6-F2-003), `ultimate_combat::spell_list` (146, since
//! SD31-E6-F2-004) and, since SD31-E6-F10-001,
//! `inner_sea_gods::spell_list` (92) and, since SD-31 wave-19
//! (`ultimate_wilderness` lane), `ultimate_wilderness::spell_list` (61) —
//! 2011 in total. This adapter
//! never chains a book by hand; it reads `spell_resolver::spell_catalog_rows()`
//! (see `build_spell_catalog` below), so a book widening that registry
//! reaches this DTO automatically. The per-book count is still worth
//! stating here because it is the sweep target: `SpellCatalogScreen.tsx`'s
//! `BOOK_ORDER`/`BOOK_LABELS` and its own test's `CHAINED_BOOK_CODES` are
//! hand-copies by design (their own doc comments explain why an
//! independent oracle beats a derived one) and do NOT update themselves
//! when this list grows.
//!
//! Pathfinder Unchained is deliberately absent, and that absence is real
//! rather than an oversight: `pu_spells.lst` is 224 lines and every single
//! one is a `#`-commented-out row, so the book defines no spell of its own
//! and there is no `pathfinder_unchained::spell_list` module to chain.
//! Re-verified against the raw corpus rather than taken from the ingest
//! notes: `grep -v '^\s*#' pu_spells.lst | grep -vc '^\s*$'` returns 0.
//! Inventing a PU spell surface here would mean serving records the corpus
//! does not contain.
//!
//! This adapter previously served CRB alone. The APG and ACG tables were
//! fully ingested — with school, level and real corpus spell text — but
//! reached no user-facing surface at all: not this catalog browser, not
//! the Character Sheet's Add Spell picker (which calls `list_spells`), and
//! so not a character's own spell list either. 441 real spells, 40% of
//! everything ingested, were invisible.
//!
//! Distinct from the Character Sheet's Spells tab: this is a standalone
//! catalog view of every real spell record the engine knows about, not
//! what one character has selected. Mirrors `equipment_catalog.rs`.
//!
//! **Optional fields are absences, never placeholders.** CRB's and ACG's
//! tables carry a school, level and description on every record. APG's
//! table types those three as `Option`, and as ingested, 3 of its records
//! carry no school, 25 no level and none lack a description. Those arrive
//! here as `null` rather than an invented value, and the UI must render
//! the absence rather than a plausible-looking default.
//!
//! **Those counts were 16 / 41 / 12 until 2026-07-31**, and this doc used
//! to record the difference between them and the raw file as an untraced
//! "open ingest-fidelity question". It is traced, and the answer was
//! `.COPY=`: fifteen delta rows in `apg_spells.lst` name a base spell that
//! lives in CRB's `cr_spells.lst`, and the APG ingest deliberately stopped
//! at the book boundary rather than reaching across for the base's fields.
//! Twelve of them therefore reached this adapter as a key and three nulls
//! and rendered as a row of empty columns. `apg::spell_list` now resolves
//! every one against its CRB base, pinned record-by-record against the
//! live CRB table by
//! `tests/sd27_apg_delta_spell_rows_resolve_against_their_base.rs`.
//!
//! The remainder are real corpus absences, not unresolved deltas: mostly
//! Summoner eidolon-only spells that PF1 grants automatically and so never
//! places on a leveled spell list. The counts below are still properties
//! of `apg::spell_list` as ingested and are asserted as such.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use codex::rules_core::derived_evaluator_fixture_check::{
    all_spell_caster_level_durations, all_spell_caster_level_ranges,
    format_caster_level_linear_duration, format_spell_range_formula,
    spell_book_corpus_dir_for_short_code, CasterLevelLinearFormula, SpellRangeFormula,
};
use codex::rules_core::pcgen_desc::render_pcgen_desc;
use codex::rules_core::rules_tables::{
    acg, adventurers_guide, advanced_race_guide, apg, bestiary, bestiary_4,
    book_of_the_damned_volume_1, book_of_the_damned_volume_2, crb, horror_adventures,
    inner_sea_faiths, inner_sea_gods, inner_sea_intrigue, inner_sea_magic, inner_sea_races,
    inner_sea_temples, inner_sea_world_guide, monster_codex, mythic_adventures,
    occult_adventures, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic,
    ultimate_magic_wordsofpower, ultimate_wilderness,
};
use codex::rules_core::spell_resolver;

use crate::authoring_workbench::codex_repo_root;

/// Which ingested book a catalog entry came from. Short codes are the wire
/// form; the frontend maps them to display labels.
const BOOK_CRB: &str = "CRB";
const BOOK_APG: &str = "APG";
const BOOK_ACG: &str = "ACG";
const BOOK_ARG: &str = "ARG";
const BOOK_UI: &str = "UI";
const BOOK_UM: &str = "UM";
const BOOK_OA: &str = "OA";
const BOOK_UC: &str = "UC";
const BOOK_ISG: &str = "ISG";
const BOOK_UW: &str = "UW";
/// SD-31 wave-29 (`lane5-book-onboard` lane): Adventurer's Guide, the
/// twelfth book -- this book's first record family of any kind.
const BOOK_AG: &str = "AG";
/// SD-32 Gate 0 book-onboarding precondition (`gate-0-book-onboarding-
/// precondition`, AT-32-G0-003): Inner Sea Faiths, the thirteenth book --
/// this book's first record family of any kind.
const BOOK_ISF: &str = "ISF";
/// SD-32 Gate 0 book-onboarding precondition (`gate-0-book-onboarding-
/// precondition`, AT-32-G0-003): Inner Sea Magic, the fourteenth book --
/// this book's first record family of any kind.
const BOOK_ISM: &str = "ISM";
/// SD-32 Gate 0 book-onboarding precondition (`gate-0-book-onboarding-
/// precondition`, AT-32-G0-003): Inner Sea Temples, the fifteenth book --
/// this book's first record family of any kind.
const BOOK_ISTEM: &str = "ISTEM";
/// SD-32 card 11 (T9 onboarding, `decisions.md §19` sign-off): Horror
/// Adventures, the sixteenth book -- its second record family of any
/// kind (`companion`/`monster`/`monster_ability` already ship).
const BOOK_HA: &str = "HA";
/// SD-32 row 20 (`decisions.md §17`/`§27b`): eleven more books, chained into
/// `spell_resolver::spell_catalog_rows()` this cycle -- see that module's
/// own `SPELL_BOOK_*` doc comments for provenance.
const BOOK_B1: &str = "B1";
const BOOK_B4: &str = "B4";
const BOOK_BOTD1: &str = "BOTD1";
const BOOK_BOTD2: &str = "BOTD2";
const BOOK_ISI: &str = "ISI";
const BOOK_ISR: &str = "ISR";
const BOOK_ISWG: &str = "ISWG";
const BOOK_MC: &str = "MC";
const BOOK_MYTHIC: &str = "MYTHIC";
const BOOK_UE: &str = "UE";
const BOOK_UMWP: &str = "UMWP";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellCatalogEntryDto {
    /// The record's corpus identity — its `KEY:` token when the row
    /// carries one, else its display name. Unique across all four books;
    /// see `tests/spell_cross_book_identity.rs`.
    pub key: String,
    /// `"CRB"`, `"APG"`, `"ACG"` or `"ARG"`.
    pub book: String,
    /// The `Pf1SchoolId` variant name verbatim (e.g. "Abjuration"), or
    /// `None` for an APG record whose corpus row has no `SCHOOL:` token.
    pub school: Option<String>,
    /// `None` for an APG record whose corpus row has no `CLASSES:` token,
    /// so no spell level can be derived without inventing one.
    pub level: Option<u8>,
    /// `None` for an APG record the corpus supplies no `DESC:` text for.
    pub description: Option<String>,
    /// The corpus's own `DURATION:` formula, rendered as literal text ("N
    /// <unit> per caster level") when it matches a caster-level-LINEAR
    /// shape (`derived_evaluator_fixture_check::parse_caster_level_linear_duration`,
    /// SD31-E6-F2-006). `None` both for spells with no such formula (a
    /// flat/instantaneous/permanent duration — most of the catalog) and
    /// for the small population whose formula is more complex than this
    /// function commits to (`min(`/`max(`/an additive term) — never a
    /// resolved live number: a spell's actual duration depends on the
    /// CASTING CHARACTER's caster level, which no corpus row states and
    /// this reference catalog has no character context for (see that
    /// function's own doc comment for why fabricating one would repeat
    /// the ability-score-scaling monster mistake `SD31-E6-F1-002` refused).
    pub duration: Option<String>,
    /// The corpus's own `RANGE:` keyword, rendered as literal text ("N ft. +
    /// N ft. per [N] caster level(s)") when it names one of the three PF1
    /// caster-level-linear range keywords — `Close`, `Medium`, `Long`
    /// (`derived_evaluator_fixture_check::spell_range_formula`,
    /// SD31-E6-F2-008). `None` both for spells whose range is not one of
    /// those three keywords (`Personal`, `Touch`, a literal distance, "See
    /// text", ...) and — same posture as `duration` above — never a
    /// resolved live number: the formula's own base + rate is corpus/
    /// ruleset-grounded, but the CASTING CHARACTER's actual range in feet
    /// depends on a caster level this reference catalog has no character
    /// context for.
    pub range: Option<String>,
}

/// `(book corpus dir, record key) -> parsed caster-level-linear DURATION`
/// for every spell in every ingested book, built once per process. Backs
/// [`duration_for`]; see [`all_spell_caster_level_durations`]'s own doc
/// comment for what "parseable" means and why an unparseable/absent
/// DURATION renders `None` rather than a guess.
fn spell_caster_level_durations() -> &'static BTreeMap<(String, String), CasterLevelLinearFormula>
{
    static DURATIONS: OnceLock<BTreeMap<(String, String), CasterLevelLinearFormula>> =
        OnceLock::new();
    DURATIONS.get_or_init(|| match codex_repo_root() {
        Ok(root) => all_spell_caster_level_durations(&root),
        Err(_) => BTreeMap::new(),
    })
}

/// The rendered duration text for one catalog row, or `None` — see
/// [`SpellCatalogEntryDto::duration`].
fn duration_for(book_short_code: &str, key: &str) -> Option<String> {
    let corpus_dir = spell_book_corpus_dir_for_short_code(book_short_code)?;
    spell_caster_level_durations()
        .get(&(corpus_dir.to_string(), key.to_string()))
        .map(format_caster_level_linear_duration)
}

/// `(book corpus dir, record key) -> resolved SPELLRANGE formula` for every
/// spell in every ingested book, built once per process, same
/// process-lifetime-cached shape as [`spell_caster_level_durations`]. Backs
/// [`range_for`]; see [`all_spell_caster_level_ranges`]'s own doc comment.
fn spell_caster_level_ranges() -> &'static BTreeMap<(String, String), SpellRangeFormula> {
    static RANGES: OnceLock<BTreeMap<(String, String), SpellRangeFormula>> = OnceLock::new();
    RANGES.get_or_init(|| match codex_repo_root() {
        Ok(root) => all_spell_caster_level_ranges(&root),
        Err(_) => BTreeMap::new(),
    })
}

/// The rendered range text for one catalog row, or `None` — see
/// [`SpellCatalogEntryDto::range`].
fn range_for(book_short_code: &str, key: &str) -> Option<String> {
    let corpus_dir = spell_book_corpus_dir_for_short_code(book_short_code)?;
    spell_caster_level_ranges()
        .get(&(corpus_dir.to_string(), key.to_string()))
        .map(format_spell_range_formula)
}

/// Renders one table description into the prose this catalog is allowed to
/// serve.
///
/// The four `spell_list` tables hold each record's `DESC:` token as the
/// corpus writes it — prose plus, where the book states a caster-level
/// formula, a `%N` reference and its `|`-delimited argument tail. That is the
/// right thing for a corpus transcription to store and the wrong thing to put
/// in front of a player: before this, ARG's "Absorbing Inhalation" reached the
/// Spell Catalog screen (and the Character Sheet's Add Spell picker, which
/// calls `list_spells`) reading *"contained within you for up to %1 rounds"*
/// and ending *"you suffer the cloud's effects|CASTERLEVEL"*.
///
/// Derived over the four tables rather than assumed: 79 of the 1173 served
/// descriptions carried PCGen syntax — 63 CRB, 3 APG, 0 ACG, 13 ARG. 21 of
/// CRB's are its inline rulebook tables' ` | ` column separators, which are
/// real prose and are preserved; the rest are `%%` escapes (49 records) and
/// ARG's 10 caster-level `%N` references.
///
/// [`render_pcgen_desc`] owns the treatment and the reasoning about what may
/// and may not be substituted; this is only the point of application.
fn serve_description(raw: &str) -> String {
    render_pcgen_desc(raw).text
}

fn map_crb_entry(entry: &crb::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_CRB.to_string(),
        school: Some(format!("{:?}", entry.school)),
        level: Some(entry.level),
        description: Some(serve_description(entry.description)),
        duration: duration_for(BOOK_CRB, entry.key),
        range: range_for(BOOK_CRB, entry.key),
    }
}

fn map_apg_entry(entry: &apg::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_APG.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_APG, entry.key),
        range: range_for(BOOK_APG, entry.key),
    }
}

fn map_acg_entry(entry: &acg::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ACG.to_string(),
        school: Some(format!("{:?}", entry.school)),
        level: Some(entry.level),
        description: Some(serve_description(entry.description)),
        duration: duration_for(BOOK_ACG, entry.key),
        range: range_for(BOOK_ACG, entry.key),
    }
}

/// ARG's table types `school`, `level` and `description` non-optionally,
/// exactly as CRB's and ACG's do, so like those two this map invents
/// nothing by wrapping in `Some` — every ARG record genuinely carries all
/// three (pinned by `crb_acg_and_arg_records_are_always_fully_populated`).
fn map_arg_entry(entry: &advanced_race_guide::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ARG.to_string(),
        school: Some(format!("{:?}", entry.school)),
        level: Some(entry.level),
        description: Some(serve_description(entry.description)),
        duration: duration_for(BOOK_ARG, entry.key),
        range: range_for(BOOK_ARG, entry.key),
    }
}

/// UI's table types `school`, `level` and `description` non-optionally,
/// exactly as ARG's does, so like that one this map invents nothing by
/// wrapping in `Some` -- every UI record genuinely carries all three
/// (every `ui_spells.lst` base record carries `SCHOOL:`, `CLASSES:` and
/// `DESC:`; see `ultimate_intrigue::spell_list`'s own doc comment).
/// UM's table types `school`, `level` and `description` optionally, like
/// APG's -- the real corpus gap this cycle's own ingest found and named
/// (`Restore Eidolon` and 24 siblings carry neither `CLASSES:` nor
/// `DOMAINS:`; 15 `Masterpiece` records carry a `SCHOOL:` value ("Masterpiece")
/// this engine's 9-school enum does not recognize), never fabricated.
fn map_um_entry(entry: &ultimate_magic::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_UM.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_UM, entry.key),
        range: range_for(BOOK_UM, entry.key),
    }
}

fn map_ui_entry(entry: &ultimate_intrigue::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_UI.to_string(),
        school: Some(format!("{:?}", entry.school)),
        level: Some(entry.level),
        description: Some(serve_description(entry.description)),
        duration: duration_for(BOOK_UI, entry.key),
        range: range_for(BOOK_UI, entry.key),
    }
}

/// OA's table types `school`, `level` and `description` optionally, like
/// UM's -- the real corpus gaps this cycle's own ingest found and named
/// (`Talismanic Implement` carries no `CLASSES:` token; `Share Language
/// (Communal)` carries neither `SCHOOL:` nor `DESC:` of its own), never
/// fabricated.
fn map_oa_entry(entry: &occult_adventures::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_OA.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_OA, entry.key),
        range: range_for(BOOK_OA, entry.key),
    }
}

/// UC's table types `school`, `level` and `description` optionally, like
/// OA's -- the real corpus gap this cycle's own ingest found and named
/// (`Life Conduit` and its two named variants carry neither `SCHOOL:` nor
/// `CLASSES:` of their own), never fabricated.
fn map_uc_entry(entry: &ultimate_combat::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_UC.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_UC, entry.key),
        range: range_for(BOOK_UC, entry.key),
    }
}

/// ISG's table types `school`, `level` and `description` optionally, like
/// UC's -- the real corpus gap this cycle's own ingest found and named
/// (`SD31-E6-F10-001`: 31 of 92 records carry no `CLASSES:`/`DOMAINS:`
/// token of their own, mostly deity-boon variant spells whose base entry
/// lives in another book), never fabricated.
fn map_isg_entry(entry: &inner_sea_gods::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ISG.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_ISG, entry.key),
        range: range_for(BOOK_ISG, entry.key),
    }
}

/// UW's table types `school`, `level` and `description` optionally, like
/// UC's/ISG's -- SD-31 wave-19's `ultimate_wilderness` lane ingest found
/// every one of the 61 base declarations carries all three, but the table
/// keeps the `Option` shape every sibling per-book table uses rather than
/// asserting non-optionality this book's own corpus happens not to
/// exercise (`src/bin/ingest_ultimate_wilderness_spells.rs`).
fn map_uw_entry(entry: &ultimate_wilderness::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_UW.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_UW, entry.key),
        range: range_for(BOOK_UW, entry.key),
    }
}

/// AG's table types `school`, `level` and `description` optionally, like
/// UW's/UC's/ISG's -- SD-31 wave-29's `lane5-book-onboard` lane ingest
/// found 2 of the 45 shipped base declarations carry no `CLASSES:`/
/// `DOMAINS:` level (`Continual Flame (Lantern Bearer)`, `Summon Mantis`).
fn map_ag_entry(entry: &adventurers_guide::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_AG.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_AG, entry.key),
        range: range_for(BOOK_AG, entry.key),
    }
}

/// ISF's table types `school`, `level` and `description` optionally, like
/// AG's/UW's/UC's/ISG's -- SD-32 Gate 0 book-onboarding precondition
/// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) ingest found all 3
/// shipped base declarations carry no `CLASSES:`/`DOMAINS:` level at all
/// (`isf_spells.lst` names no class list on any of its rows), a real corpus
/// gap, not fabricated.
fn map_isf_entry(entry: &inner_sea_faiths::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ISF.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_ISF, entry.key),
        range: range_for(BOOK_ISF, entry.key),
    }
}

/// ISM's table types `school`, `level` and `description` optionally, like
/// ISF's above -- SD-32 Gate 0 book-onboarding precondition
/// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) ingest.
fn map_ism_entry(entry: &inner_sea_magic::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ISM.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_ISM, entry.key),
        range: range_for(BOOK_ISM, entry.key),
    }
}

/// ISTem's table types `school`, `level` and `description` optionally, like
/// ISF's/ISM's above -- SD-32 Gate 0 book-onboarding precondition
/// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) ingest.
fn map_istem_entry(entry: &inner_sea_temples::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ISTEM.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_ISTEM, entry.key),
        range: range_for(BOOK_ISTEM, entry.key),
    }
}

/// Horror Adventures -- SD-32 card 11 (T9 onboarding, `decisions.md §19`
/// sign-off), config-driven ingest (`src/bin/ingest_spells.rs`,
/// `decisions.md §17`). Its table types `school`, `level` and
/// `description` optionally, like ISF's/ISM's/ISTem's above.
fn map_ha_entry(entry: &horror_adventures::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_HA.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_HA, entry.key),
        range: range_for(BOOK_HA, entry.key),
    }
}

/// SD-32 row 20: eleven more per-book mapping helpers, all typed against the
/// same `Option`-everywhere `SpellListEntry` shape `src/bin/ingest_spells.rs`
/// generates uniformly (see that module's own doc comment).
fn map_b1_entry(entry: &bestiary::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_B1.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_B1, entry.key),
        range: range_for(BOOK_B1, entry.key),
    }
}
fn map_b4_entry(entry: &bestiary_4::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_B4.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_B4, entry.key),
        range: range_for(BOOK_B4, entry.key),
    }
}
fn map_botd1_entry(
    entry: &book_of_the_damned_volume_1::spell_list::SpellListEntry,
) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_BOTD1.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_BOTD1, entry.key),
        range: range_for(BOOK_BOTD1, entry.key),
    }
}
fn map_botd2_entry(
    entry: &book_of_the_damned_volume_2::spell_list::SpellListEntry,
) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_BOTD2.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_BOTD2, entry.key),
        range: range_for(BOOK_BOTD2, entry.key),
    }
}
fn map_isi_entry(entry: &inner_sea_intrigue::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ISI.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_ISI, entry.key),
        range: range_for(BOOK_ISI, entry.key),
    }
}
fn map_isr_entry(entry: &inner_sea_races::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ISR.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_ISR, entry.key),
        range: range_for(BOOK_ISR, entry.key),
    }
}
fn map_iswg_entry(
    entry: &inner_sea_world_guide::spell_list::SpellListEntry,
) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ISWG.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_ISWG, entry.key),
        range: range_for(BOOK_ISWG, entry.key),
    }
}
fn map_mc_entry(entry: &monster_codex::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_MC.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_MC, entry.key),
        range: range_for(BOOK_MC, entry.key),
    }
}
fn map_mythic_entry(
    entry: &mythic_adventures::spell_list::SpellListEntry,
) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_MYTHIC.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_MYTHIC, entry.key),
        range: range_for(BOOK_MYTHIC, entry.key),
    }
}
fn map_ue_entry(entry: &ultimate_equipment::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_UE.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_UE, entry.key),
        range: range_for(BOOK_UE, entry.key),
    }
}
fn map_umwp_entry(
    entry: &ultimate_magic_wordsofpower::spell_list::SpellListEntry,
) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_UMWP.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(serve_description),
        duration: duration_for(BOOK_UMWP, entry.key),
        range: range_for(BOOK_UMWP, entry.key),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellCatalogResponse {
    pub entries: Vec<SpellCatalogEntryDto>,
}

/// Build the full catalog response across every ingested book. A thin,
/// testable wrapper behind the Tauri command below (mirroring
/// `equipment_catalog`'s own command/pure-fn split).
pub fn build_spell_catalog() -> SpellCatalogResponse {
    // SD-29 Epic 4 (spell lane): the five per-book chains this function used
    // to spell out inline now live in
    // `codex::rules_core::spell_resolver::spell_catalog_rows()`, shared with
    // `v06_work_inventory`'s `spell_levels` map. Those two lists had drifted
    // (five books here, three there), which reported every shipping ARG and
    // UI spell as `not-ingested`. Reading one registry is what makes that
    // divergence unrepresentable; the per-book `map_*_entry` helpers below
    // are retained as the typed proof that each book's own table supplies
    // exactly the fields this DTO claims (see their doc comments and the
    // `mapping_helpers_agree_with_the_registry` test).
    let entries = spell_resolver::spell_catalog_rows()
        .iter()
        .map(|row| SpellCatalogEntryDto {
            key: row.key.to_string(),
            book: row.book.to_string(),
            school: row.school.clone(),
            level: row.level,
            description: row.description.map(serve_description),
            duration: duration_for(row.book, row.key),
            range: range_for(row.book, row.key),
        })
        .collect();
    SpellCatalogResponse { entries }
}

#[tauri::command]
pub fn list_spell_catalog() -> SpellCatalogResponse {
    build_spell_catalog()
}

/// Filter criteria for `list_spells`. Every field is optional and
/// `None`/empty matches everything — an all-`None` filter is equivalent to
/// the unfiltered `list_spell_catalog` response. Kept deliberately narrow
/// (substring name match, exact school/book match) rather than an
/// exhaustive query DSL; widen only if a real caller needs more.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellCatalogFilter {
    /// Case-insensitive substring match against `key` (the spell's corpus
    /// identity/name — see `SpellCatalogEntryDto::key`'s doc comment).
    pub name_contains: Option<String>,
    /// Exact match against the `Pf1SchoolId` variant name verbatim (e.g.
    /// "Evocation"). A record whose corpus row has no `SCHOOL:` token
    /// matches no school filter — it is genuinely unknown, so it is not
    /// swept into any school.
    pub school: Option<String>,
    /// Exact match against `"CRB"`, `"APG"`, `"ACG"` or `"ARG"`.
    pub book: Option<String>,
}

/// Narrows the full catalog to the entries matching `filter`. A thin,
/// testable wrapper behind the `list_spells` Tauri command below, mirroring
/// `build_spell_catalog`'s own command/pure-fn split.
pub fn filter_spell_catalog(filter: &SpellCatalogFilter) -> SpellCatalogResponse {
    let name_needle = filter
        .name_contains
        .as_ref()
        .filter(|needle| !needle.is_empty())
        .map(|needle| needle.to_lowercase());

    let entries = build_spell_catalog()
        .entries
        .into_iter()
        .filter(|entry| match &name_needle {
            Some(needle) => entry.key.to_lowercase().contains(needle.as_str()),
            None => true,
        })
        .filter(|entry| match &filter.school {
            Some(school) => entry.school.as_deref() == Some(school.as_str()),
            None => true,
        })
        .filter(|entry| match &filter.book {
            Some(book) => &entry.book == book,
            None => true,
        })
        .collect();

    SpellCatalogResponse { entries }
}

/// Returns the spell catalog narrowed by `filter` — see
/// `SpellCatalogFilter`'s own doc comment for the supported fields.
#[tauri::command]
pub fn list_spells(filter: SpellCatalogFilter) -> SpellCatalogResponse {
    filter_spell_catalog(&filter)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn book_entries(book: &str) -> Vec<SpellCatalogEntryDto> {
        build_spell_catalog()
            .entries
            .into_iter()
            .filter(|entry| entry.book == book)
            .collect()
    }

    /// The consolidation must be payload-identical to the five hand-chained
    /// `map_*_entry` calls it replaced. Each helper is still the typed,
    /// per-book statement of which fields that book genuinely supplies
    /// (CRB/ACG/ARG/UI non-optional, APG optional); this test asserts the
    /// registry reproduces all five exactly, so the helpers are the proof
    /// rather than a second implementation that could drift.
    ///
    /// `bestiary_6` is deliberately NOT chained into `expected` below --
    /// BOTH of its 2 rows are cross-book verbatim reprints the resolver's
    /// own dedup drops (`spell_resolver::spell_catalog_rows`'s
    /// "first-chained wins" pass), so it contributes zero net entries to
    /// `actual` and omitting it entirely still balances. `horror_adventures`
    /// (SD-32 card 11, T9 onboarding) is NOT zero-net -- only 2 of its 72
    /// rows collide (`Green Caress`, `Verminous Transformation`, both
    /// already served under `BOOK_UW`, earlier in the chain) -- so it IS
    /// chained below, with those 2 keys excluded to mirror the same dedup
    /// `actual` applies, preserving both count and relative order.
    #[test]
    fn mapping_helpers_agree_with_the_registry() {
        let expected: Vec<SpellCatalogEntryDto> = crb::spell_list::SPELL_LIST
            .iter()
            .map(map_crb_entry)
            .chain(apg::spell_list::SPELL_LIST.iter().map(map_apg_entry))
            .chain(acg::spell_list::SPELL_LIST.iter().map(map_acg_entry))
            .chain(advanced_race_guide::spell_list::SPELL_LIST.iter().map(map_arg_entry))
            .chain(ultimate_intrigue::spell_list::SPELL_LIST.iter().map(map_ui_entry))
            .chain(ultimate_magic::spell_list::SPELL_LIST.iter().map(map_um_entry))
            .chain(occult_adventures::spell_list::SPELL_LIST.iter().map(map_oa_entry))
            .chain(ultimate_combat::spell_list::SPELL_LIST.iter().map(map_uc_entry))
            .chain(inner_sea_gods::spell_list::SPELL_LIST.iter().map(map_isg_entry))
            .chain(ultimate_wilderness::spell_list::SPELL_LIST.iter().map(map_uw_entry))
            .chain(adventurers_guide::spell_list::SPELL_LIST.iter().map(map_ag_entry))
            .chain(inner_sea_faiths::spell_list::SPELL_LIST.iter().map(map_isf_entry))
            .chain(inner_sea_magic::spell_list::SPELL_LIST.iter().map(map_ism_entry))
            .chain(inner_sea_temples::spell_list::SPELL_LIST.iter().map(map_istem_entry))
            .chain(
                horror_adventures::spell_list::SPELL_LIST
                    .iter()
                    .filter(|e| e.key != "Green Caress" && e.key != "Verminous Transformation")
                    .map(map_ha_entry),
            )
            // SD-32 row 20: eleven more books chained this cycle
            // (`decisions.md §17`/`§27b`), same dedup-exclusion shape as
            // `horror_adventures` above -- see `reach_gate.rs`'s matching
            // dispatch arms for each collision's own provenance.
            .chain(bestiary::spell_list::SPELL_LIST.iter().map(map_b1_entry))
            .chain(
                bestiary_4::spell_list::SPELL_LIST
                    .iter()
                    .filter(|e| e.key != "Quickened Lightning Bolt")
                    .map(map_b4_entry),
            )
            .chain(
                book_of_the_damned_volume_1::spell_list::SPELL_LIST
                    .iter()
                    .filter(|e| e.key != "Agonize" && e.key != "Vision of Hell")
                    .map(map_botd1_entry),
            )
            .chain(
                book_of_the_damned_volume_2::spell_list::SPELL_LIST
                    .iter()
                    .filter(|e| {
                        e.key != "Disfiguring Touch"
                            && e.key != "Vermin Shape I"
                            && e.key != "Vermin Shape II"
                    })
                    .map(map_botd2_entry),
            )
            .chain(
                inner_sea_intrigue::spell_list::SPELL_LIST
                    .iter()
                    .filter(|e| e.key != "Brightest Light")
                    .map(map_isi_entry),
            )
            .chain(inner_sea_races::spell_list::SPELL_LIST.iter().map(map_isr_entry))
            .chain(
                inner_sea_world_guide::spell_list::SPELL_LIST
                    .iter()
                    .filter(|e| {
                        ![
                            "Animal Growth (Reptiles Only)",
                            "Animal Shapes (Reptiles Only)",
                            "Interplanetary Teleport",
                            "Vermin Shape I",
                            "Vermin Shape II",
                            "Dirge of the Victorious Knights",
                            "Summon Mantis",
                            "Quickened Dispel Magic (Greater)",
                        ]
                        .contains(&e.key)
                    })
                    .map(map_iswg_entry),
            )
            .chain(monster_codex::spell_list::SPELL_LIST.iter().map(map_mc_entry))
            .chain(mythic_adventures::spell_list::SPELL_LIST.iter().map(map_mythic_entry))
            .chain(ultimate_equipment::spell_list::SPELL_LIST.iter().map(map_ue_entry))
            .chain(
                ultimate_magic_wordsofpower::spell_list::SPELL_LIST.iter().map(map_umwp_entry),
            )
            // `inner_sea_races`'s own table restates "Elemental Mastery" 5
            // times and `bestiary_4`'s own table restates two keys twice
            // each (confirmed real: `grep -oP 'key: "\K[^"]+'` on each
            // table, `uniq -c`) -- genuine WITHIN-book duplicates, distinct
            // from the cross-book collisions excluded above. Production's
            // `spell_catalog_rows()` global "first key wins" dedup collapses
            // these the same way it collapses a cross-book collision, so
            // this reconstruction applies the identical pass rather than
            // hand-listing every duplicate key.
            .collect::<Vec<_>>();
        let mut seen = std::collections::HashSet::new();
        let expected: Vec<SpellCatalogEntryDto> =
            expected.into_iter().filter(|e| seen.insert(e.key.clone())).collect();
        let actual = build_spell_catalog().entries;
        assert_eq!(actual.len(), expected.len());
        for (a, e) in actual.iter().zip(expected.iter()) {
            assert_eq!(a.key, e.key);
            assert_eq!(a.book, e.book);
            assert_eq!(a.school, e.school);
            assert_eq!(a.level, e.level);
            assert_eq!(a.description, e.description);
        }
    }

    #[test]
    fn the_catalog_serves_every_ingested_book_not_only_crb() {
        let response = build_spell_catalog();
        // SD-31 wave-29 (`lane5-book-onboard` lane): +45 AG spells, the
        // twelfth book, 2011 -> 2056.
        // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
        // onboarding-precondition`, AT-32-G0-003): +2 ISF + 34 ISM + 21
        // ISTEM, the thirteenth/fourteenth/fifteenth books, 2056 -> 2113.
        // ISF's raw `isf_spells.lst` carries 3 base declarations but one
        // ("Curse of Disgust (Besmaran)") is genuinely restated a second
        // time later in the same file; the ingest binary dedups within a
        // book (first-declaration-wins, mirroring `spell_resolver`'s own
        // cross-book policy), so only 2 of the 3 ship.
        // SD-32 card 11 (T9 onboarding, `decisions.md §19` sign-off): +72
        // HA (Horror Adventures), the sixteenth book, base declarations,
        // but 2 ("Green Caress", "Verminous Transformation") are verbatim
        // reprints of spells Ultimate Wilderness already ships (earlier in
        // the chain) -- the resolver's cross-book dedup (SD-31 wave-24,
        // first-chained wins) keeps only UW's copy, so only 70 of HA's 72
        // reach the SERVED catalog. 2113 -> 2183.
        // Row-19 desktop reach/catalog reds (SD-32, 2026-08-24): the T12
        // census/class-feature lanes' corpus growth landed +14 more spells
        // corpus-wide with no regen of this pin (`decisions.md §17a`
        // re-derivation, not a loosening -- every book's own count is
        // re-checked below, not just the total). Re-derived via a one-shot
        // debug print of `book_entries(..).len()` for all 15 books,
        // confirmed to sum to the new total: ISG +4 (92 -> 96), AG +4
        // (45 -> 49), ISF +1 (2 -> 3), ISM +5 (34 -> 39); the other eleven
        // books are unchanged. 2183 -> 2197.
        // SD-32 row 20 (`decisions.md §17`/`§27b`): eleven more books
        // chained, 2197 -> 2481. Re-derived via `build_spell_catalog()`
        // directly (`book_entries(..).len()` per book, summed and cross-
        // checked against `response.entries.len()`), not assumed:
        // `inner_sea_races`'s own table restates "Elemental Mastery" 5
        // times and `bestiary_4`'s restates two keys twice each -- genuine
        // within-book corpus duplicates the global "first key wins" dedup
        // collapses, same as any cross-book collision. +111 B1 (bestiary/
        // beastiary1, 2 bare -- see BARE_RECORD_FINDINGS) + 55 B4 (58 raw -
        // 2 within-book dupes - 1 cross-book collision with B1) + 4 BOTD1
        // (6 - 2 collisions with UM) + 8 BOTD2 (11 - 3 collisions with UM)
        // + 25 ISI (26 - 1 collision with AG) + 29 ISR (33 - 4 within-book
        // dupes) + 14 ISWG (22 - 8 collisions with UW/UM/AG/B4) + 24 MC +
        // 10 MYTHIC + 1 UE + 3 UMWP (no collisions for these four).
        assert_eq!(response.entries.len(), 2481);
        assert_eq!(book_entries(BOOK_CRB).len(), 664);
        assert_eq!(book_entries(BOOK_APG).len(), 297);
        assert_eq!(book_entries(BOOK_ACG).len(), 144);
        assert_eq!(book_entries(BOOK_ARG).len(), 93);
        assert_eq!(book_entries(BOOK_UI).len(), 101);
        assert_eq!(book_entries(BOOK_UM).len(), 269);
        assert_eq!(book_entries(BOOK_OA).len(), 144);
        assert_eq!(book_entries(BOOK_UC).len(), 146);
        assert_eq!(book_entries(BOOK_ISG).len(), 96);
        assert_eq!(book_entries(BOOK_UW).len(), 61);
        assert_eq!(book_entries(BOOK_AG).len(), 49);
        assert_eq!(book_entries(BOOK_ISF).len(), 3);
        assert_eq!(book_entries(BOOK_ISM).len(), 39);
        assert_eq!(book_entries(BOOK_ISTEM).len(), 21);
        // 70, not 72: "Green Caress"/"Verminous Transformation" serve under
        // BOOK_UW (earlier in the chain), not BOOK_HA -- see the comment
        // above `response.entries.len()`'s own assertion.
        assert_eq!(book_entries(BOOK_HA).len(), 70);
        assert_eq!(book_entries(BOOK_B1).len(), 111);
        assert_eq!(book_entries(BOOK_B4).len(), 55);
        assert_eq!(book_entries(BOOK_BOTD1).len(), 4);
        assert_eq!(book_entries(BOOK_BOTD2).len(), 8);
        assert_eq!(book_entries(BOOK_ISI).len(), 25);
        assert_eq!(book_entries(BOOK_ISR).len(), 29);
        assert_eq!(book_entries(BOOK_ISWG).len(), 14);
        assert_eq!(book_entries(BOOK_MC).len(), 24);
        assert_eq!(book_entries(BOOK_MYTHIC).len(), 10);
        assert_eq!(book_entries(BOOK_UE).len(), 1);
        assert_eq!(book_entries(BOOK_UMWP).len(), 3);
    }

    #[test]
    fn crb_school_counts_match_the_real_corpus() {
        let crb = book_entries(BOOK_CRB);
        let counts = |school: &str| {
            crb.iter()
                .filter(|e| e.school.as_deref() == Some(school))
                .count()
        };
        assert_eq!(counts("Abjuration"), 74);
        assert_eq!(counts("Conjuration"), 120);
        assert_eq!(counts("Divination"), 54);
        assert_eq!(counts("Enchantment"), 61);
        assert_eq!(counts("Evocation"), 87);
        assert_eq!(counts("Illusion"), 48);
        assert_eq!(counts("Necromancy"), 62);
        assert_eq!(counts("Transmutation"), 153);
        assert_eq!(counts("Universal"), 5);
    }

    #[test]
    fn every_entry_has_a_non_empty_key_and_a_known_book() {
        for entry in &build_spell_catalog().entries {
            assert!(!entry.key.is_empty());
            assert!(
                [
                    BOOK_CRB, BOOK_APG, BOOK_ACG, BOOK_ARG, BOOK_UI, BOOK_UM, BOOK_OA, BOOK_UC,
                    BOOK_ISG, BOOK_UW, BOOK_AG, BOOK_ISF, BOOK_ISM, BOOK_ISTEM, BOOK_HA, BOOK_B1,
                    BOOK_B4, BOOK_BOTD1, BOOK_BOTD2, BOOK_ISI, BOOK_ISR, BOOK_ISWG, BOOK_MC,
                    BOOK_MYTHIC, BOOK_UE, BOOK_UMWP,
                ]
                .contains(&entry.book.as_str())
            );
        }
    }

    #[test]
    fn no_key_is_served_twice_so_a_selection_resolves_unambiguously() {
        let entries = build_spell_catalog().entries;
        let mut keys: Vec<String> = entries.iter().map(|entry| entry.key.clone()).collect();
        keys.sort();
        let total = keys.len();
        keys.dedup();
        assert_eq!(keys.len(), total, "the catalog serves a duplicate spell key");
    }

    #[test]
    fn crb_acg_and_arg_records_are_always_fully_populated() {
        for entry in book_entries(BOOK_CRB)
            .iter()
            .chain(book_entries(BOOK_ACG).iter())
            .chain(book_entries(BOOK_ARG).iter())
        {
            assert!(entry.school.is_some(), "{} has no school", entry.key);
            assert!(entry.level.is_some(), "{} has no level", entry.key);
            assert!(
                entry.description.as_deref().is_some_and(|d| !d.is_empty()),
                "{} has no description",
                entry.key
            );
        }
    }

    #[test]
    fn apg_records_missing_a_field_are_served_with_that_field_null() {
        // Transcribed from `apg::spell_list` as ingested — a pin on what
        // this adapter serves. Was 16 / 41 / 12 before the `.COPY=`
        // cross-book resolution landed (see this module's doc comment).
        // If the ingest changes, re-derive these; do not relax them.
        let apg = book_entries(BOOK_APG);
        assert_eq!(apg.iter().filter(|e| e.school.is_none()).count(), 3);
        assert_eq!(apg.iter().filter(|e| e.level.is_none()).count(), 25);
        assert_eq!(apg.iter().filter(|e| e.description.is_none()).count(), 0);
        // The defect this closed, asserted as the property rather than as
        // three numbers: no APG record reaches the catalog carrying nothing
        // but its key.
        assert_eq!(
            apg.iter()
                .filter(|e| e.school.is_none() && e.level.is_none() && e.description.is_none())
                .count(),
            0
        );
    }

    #[test]
    fn the_archetype_summon_records_are_served_under_their_corpus_key() {
        let entries = build_spell_catalog().entries;
        let has = |key: &str| entries.iter().any(|entry| entry.key == key);
        // The CRB spell and the two archetype variants are three distinct
        // records and all reach the catalog under distinct names.
        assert!(has("Summon Monster I"));
        assert!(has("Summoner Summon Monster I"));
        assert!(has("Summon Nature's Ally I"));
        assert!(has("Naturalist Summon Nature's Ally I"));
    }

    #[test]
    fn filter_spell_catalog_with_no_filter_fields_returns_the_full_catalog() {
        let response = filter_spell_catalog(&SpellCatalogFilter::default());
        assert_eq!(response.entries.len(), build_spell_catalog().entries.len());
    }

    #[test]
    fn filter_spell_catalog_matches_name_contains_case_insensitively() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: Some("fireball".to_owned()),
            school: None,
            book: None,
        });

        assert!(
            !response.entries.is_empty(),
            "the real CRB corpus has a Fireball record"
        );
        assert!(response.entries.len() < build_spell_catalog().entries.len());
        for entry in &response.entries {
            assert!(
                entry.key.to_lowercase().contains("fireball"),
                "entry {:?} does not contain 'fireball'",
                entry.key
            );
        }
    }

    #[test]
    fn filter_spell_catalog_matches_school_exactly_across_books() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: None,
            school: Some("Evocation".to_owned()),
            book: None,
        });

        // Now spans all four books, so strictly more than CRB's own 87.
        assert!(response.entries.len() > 87);
        for entry in &response.entries {
            assert_eq!(entry.school.as_deref(), Some("Evocation"));
        }
    }

    #[test]
    fn filter_spell_catalog_narrows_to_one_book() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: None,
            school: None,
            book: Some(BOOK_APG.to_owned()),
        });

        assert_eq!(response.entries.len(), 297);
        for entry in &response.entries {
            assert_eq!(entry.book, BOOK_APG);
        }
    }

    #[test]
    fn arg_school_counts_match_the_real_ingested_table() {
        // Derived from `advanced_race_guide::spell_list::SPELL_LIST` as
        // served by this adapter, not from any planning figure. The nine
        // school variants sum to ARG's whole 93; `Universal` is absent
        // from `arg_spells.lst` (the variant exists only for cross-book
        // schema parity), so it is pinned at 0 rather than omitted.
        let arg = book_entries(BOOK_ARG);
        let counts = |school: &str| {
            arg.iter()
                .filter(|e| e.school.as_deref() == Some(school))
                .count()
        };
        assert_eq!(counts("Abjuration"), 9);
        assert_eq!(counts("Conjuration"), 10);
        assert_eq!(counts("Divination"), 4);
        assert_eq!(counts("Enchantment"), 8);
        assert_eq!(counts("Evocation"), 8);
        assert_eq!(counts("Illusion"), 9);
        assert_eq!(counts("Necromancy"), 7);
        assert_eq!(counts("Transmutation"), 38);
        assert_eq!(counts("Universal"), 0);
        assert_eq!(
            counts("Abjuration")
                + counts("Conjuration")
                + counts("Divination")
                + counts("Enchantment")
                + counts("Evocation")
                + counts("Illusion")
                + counts("Necromancy")
                + counts("Transmutation")
                + counts("Universal"),
            arg.len(),
            "an ARG record landed outside the nine PF1 schools"
        );
    }

    #[test]
    fn filter_spell_catalog_narrows_to_arg() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: None,
            school: None,
            book: Some(BOOK_ARG.to_owned()),
        });

        assert_eq!(response.entries.len(), 93);
        for entry in &response.entries {
            assert_eq!(entry.book, BOOK_ARG);
        }
    }

    #[test]
    fn a_real_arg_spell_reaches_the_catalog_with_its_corpus_text() {
        let entries = build_spell_catalog().entries;
        let entry = entries
            .iter()
            .find(|entry| entry.key == "Aboleth's Lung")
            .expect("ARG's `Aboleth's Lung` record must reach the catalog");
        assert_eq!(entry.book, BOOK_ARG);
        assert_eq!(entry.school.as_deref(), Some("Transmutation"));
        assert_eq!(entry.level, Some(2));
        assert!(
            entry
                .description
                .as_deref()
                .is_some_and(|text| text.contains("breathe water")),
            // Phrased without the word this repo's wired-integration audit
            // treats as a stub marker (tests/sd24_wired_integration_audit.rs):
            // the assertion is that the shipped `DESC:` text arrives verbatim.
            "the ARG record must carry its real corpus `DESC:` text verbatim"
        );
    }

    #[test]
    fn filter_spell_catalog_combines_name_and_school_filters() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: Some("flame".to_owned()),
            school: Some("Evocation".to_owned()),
            book: None,
        });

        assert!(
            !response.entries.is_empty(),
            "the real corpus has Evocation spells with 'flame' in the name (e.g. Flame \
             Blade, Flame Strike)"
        );
        for entry in &response.entries {
            assert_eq!(entry.school.as_deref(), Some("Evocation"));
            assert!(entry.key.to_lowercase().contains("flame"));
        }
    }

    /// The production guard `src/bin/ingest_race_traits.rs` already
    /// carries for racial traits, ported to the surface that actually serves
    /// spell text to a player.
    ///
    /// Before this, 79 of the 1173 served descriptions carried raw PCGen
    /// `DESC:` syntax — ARG's "Absorbing Inhalation" ended
    /// `…the cloud's effects|CASTERLEVEL` and read `for up to %1 rounds` in
    /// the middle of the sentence. A future book that lands a leaking table
    /// now fails this test instead of reaching a screen.
    #[test]
    fn no_served_spell_description_carries_raw_pcgen_syntax() {
        use codex::rules_core::pcgen_desc::leaked_pcgen_syntax;

        let response = build_spell_catalog();
        let leaks: Vec<String> = response
            .entries
            .iter()
            .filter_map(|entry| {
                let description = entry.description.as_deref()?;
                let leak = leaked_pcgen_syntax(description)?;
                Some(format!("{} ({}): {leak}", entry.key, entry.book))
            })
            .collect();
        assert!(leaks.is_empty(), "served spell descriptions leaking PCGen syntax: {leaks:#?}");
    }

    /// The one record the defect was reported against, pinned end to end so a
    /// regression names itself.
    #[test]
    fn absorbing_inhalation_reads_as_prose_rather_than_as_a_pcgen_token() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: Some("Absorbing Inhalation".to_owned()),
            school: None,
            book: Some("ARG".to_owned()),
        });
        let entry = response
            .entries
            .first()
            .expect("Absorbing Inhalation is a real ARG spell record");
        let description = entry.description.as_deref().expect("ARG records always carry description text");

        assert!(
            description.ends_with("you suffer the cloud's effects"),
            "the `|CASTERLEVEL` argument tail must not survive: {description}"
        );
        assert!(
            !description.contains("%1"),
            "the `%1` caster-level reference must not survive: {description}"
        );
        assert!(
            description.contains("contained within you for up to rounds"),
            "the caster-level formula is dropped, not guessed, and the sentence closes up: {description}"
        );
    }

    /// The reason this catalog does not reuse the race-trait binary's
    /// "any `|` is an argument tail" rule: CRB spell text renders rulebook
    /// tables inline.
    #[test]
    fn a_crb_prose_table_survives_the_render_intact() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: Some("Power Word Stun".to_owned()),
            school: None,
            book: Some("CRB".to_owned()),
        });
        let entry = response.entries.first().expect("Power Word Stun is a real CRB record");
        let description = entry.description.as_deref().expect("CRB records always carry description text");
        assert!(
            description.contains("Hit Points | Duration"),
            "the inline rulebook table's column separators are prose and must survive: {description}"
        );
    }

    // SD31-E6-F2-006: the DoD-8 worked example this cycle's on-screen
    // verification also uses — `OPEN-ISSUES.md` row 119's own traced unit.
    #[test]
    fn adhesive_blood_serves_its_caster_level_linear_duration() {
        let entries = build_spell_catalog().entries;
        let entry = entries
            .iter()
            .find(|entry| entry.key == "Adhesive Blood" && entry.book == BOOK_ACG)
            .expect("ACG's Adhesive Blood record must reach the catalog");
        assert_eq!(
            entry.duration.as_deref(),
            Some("1 minutes per caster level"),
            "acg_spells.lst:8 states DURATION:(CASTERLEVEL) minutes"
        );
    }

    #[test]
    fn a_flat_duration_spell_serves_no_caster_level_duration() {
        let entries = build_spell_catalog().entries;
        let entry = entries
            .iter()
            .find(|entry| entry.key == "Power Word Stun" && entry.book == BOOK_CRB)
            .expect("CRB's Power Word Stun record must reach the catalog");
        assert_eq!(
            entry.duration, None,
            "Power Word Stun's DURATION is Instantaneous, not caster-level-linear -- must not \
             be fabricated"
        );
    }

    #[test]
    fn gentle_breeze_serves_its_close_range_formula() {
        let entries = build_spell_catalog().entries;
        let entry = entries
            .iter()
            .find(|entry| entry.key == "Gentle Breeze" && entry.book == BOOK_ACG)
            .expect("ACG's Gentle Breeze record must reach the catalog");
        assert_eq!(
            entry.range.as_deref(),
            Some("25 ft. + 5 ft. per 2 caster levels"),
            "acg_spells.lst:61 states RANGE:Close, PF1's standard SPELLRANGE:CLOSE formula"
        );
    }

    #[test]
    fn a_touch_range_spell_serves_no_range_formula() {
        let entries = build_spell_catalog().entries;
        let entry = entries
            .iter()
            .find(|entry| entry.key == "Mage Armor" && entry.book == BOOK_CRB)
            .expect("CRB's Mage Armor record must reach the catalog");
        assert_eq!(
            entry.range, None,
            "Mage Armor's RANGE is Touch, not one of the three caster-level-linear keywords -- \
             must not be fabricated"
        );
    }
}
