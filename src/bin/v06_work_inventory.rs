//! v0.6 alpha swarm: the durable, machine-derived **work inventory** — every
//! unit of work in the whole PF1 corpus, cross-referenced against what this
//! engine can actually prove it does.
//!
//! Run via `cargo run --bin v06_work_inventory`; emits JSON on stdout and
//! (unless `--stdout-only`) writes `docs/work-inventory.json`.
//!
//! **Why this exists.** Four of the corpus's books are ingested; the rest are
//! not. The operator needs a work inventory that (a) misses nothing and (b) is
//! monitorable during multi-day runs. Every hand-maintained artifact in this
//! project has drifted and then actively misled — a dashboard claimed 12
//! finished classes when 5 were true, a coverage matrix still read 1 wired
//! feature where the code had 6, shipped deferral strings claim engines do not
//! exist that do. So this inventory is **generated from the corpus and
//! cross-referenced against the engine**, never hand-maintained, in the same
//! discipline as `v06_class_state_dump` and `v06_content_state_dump` (read
//! both; this binary follows their conventions deliberately).
//!
//! **Two halves.**
//!
//! 1. *Corpus enumeration* (`enumerate_corpus`) walks every `.lst` file under
//!    `PCGEN_CORPUS_ROOT`'s `pathfinder/paizo/roleplaying_game/`, for **all**
//!    books including the ones no code has ever read, plus each extra root
//!    named in [`EXTRA_BOOK_DIRS`] (books that live outside `roleplaying_game`
//!    entirely: Dreamscarred Press's `ultimate_psionics` for SD-28, and the
//!    twelve Paizo `campaign_setting` books for SD-30). "What exists" is the
//!    completeness guarantee, so a
//!    book the engine knows nothing about still contributes real, named units
//!    — at `not-started` — rather than being silently skipped.
//! 2. *Engine cross-reference* (`EngineFacts`) asks the compiled tables and the
//!    real compute pipeline what is actually done, and assigns each unit a
//!    status **derived from the engine**, never from prose.
//!
//! **The corpus traps.** A naive enumerator produces confident garbage here.
//! Every rule in [`TRAP_RULES`] was validated against a figure the codebase
//! independently documents and defends before it was written down, and each is
//! asserted mechanically in `tests/v06_work_inventory.rs` so the trap is caught
//! once by a machine instead of rediscovered by hand on every new book.
//!
//! **What this binary will not do.** It never invents a unit and never invents
//! a status. A record it cannot classify is emitted as `unknown` with the
//! reason attached, because an honest unknown beats a confident wrong entry.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use codex::rules_core::character_input::{
    AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput, EquipmentSelection,
    SelectedChoice, SpellSelection, load_character_input_fixture,
};
use codex::rules_core::corpus_loader::{BookCorpusRoot, load_equipment_corpus, load_spell_corpus};
use codex::rules_core::race_resolver::{TraitRole, load_race_corpus};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::equipment_resolver;
use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus, PilotBaseChassisComputation, build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::acg::{self, AcgClassId};
use codex::rules_core::rules_tables::apg::{self, ApgClassId};
use codex::rules_core::rules_tables::beastiary1::{self, MonsterId};
use codex::rules_core::rules_tables::companion_chassis;
use codex::rules_core::rules_tables::monster_chassis;
use codex::rules_core::rules_tables::crb::{
    bard_spell_list as crb_bard_spell_list, class_tables::ClassId,
    cleric_spell_list as crb_cleric_spell_list, druid_spell_list as crb_druid_spell_list,
    equipment_tables as crb_equipment_tables, paladin_spell_list as crb_paladin_spell_list,
    race_tables::{RaceId, race_traits},
    ranger_spell_list as crb_ranger_spell_list, sorcerer_spell_list as crb_sorcerer_spell_list,
    spell_list as crb_spell_list, wizard_spell_list as crb_wizard_spell_list,
};
use codex::rules_core::rules_tables::feats_all::all_feat_tables;
use codex::rules_core::pcgen_desc::leaked_pcgen_syntax;
use codex::rules_core::pilot_view_model::{PilotSnapshot, PilotSpellbookViewModel, PilotViewModel};
use codex::rules_core::spell_resolver::{self, spell_id_resolve};
use codex::rules_core::spellbook::compute_spellbook_coverage;
use codex::rules_core::rules_tables::ultimate_campaign::feat_tables as uca_feat_tables;
use codex::rules_core::wiring_class::{self, MAGNITUDE_TOKENS};

/// The shared deterministic pilot input fixture, relative to the crate root.
/// Read at runtime rather than `include_str!`ed, exactly as
/// `v06_class_state_dump` and `v06_content_state_dump` do, and for the same
/// reason: a `src/` target should not bake a `tests/` asset into itself.
const FIXTURE_RELATIVE_PATH: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

/// Where the generated inventory lands, relative to the crate root.
const OUTPUT_RELATIVE_PATH: &str = "docs/work-inventory.json";

/// The corpus subtree every PF1 book lives under, relative to `PCGEN_CORPUS_ROOT`.
const BOOKS_RELATIVE: &str = "pathfinder/paizo/roleplaying_game";

/// Books in scope that live outside `roleplaying_game/`: SD-28's non-Paizo
/// Ultimate Psionics, and SD-30's twelve Paizo campaign_setting books
/// (Book of the Damned ×2, Inner Sea World Guide + nine Inner Sea modules).
/// Paths are relative to the corpus root; the book id is the directory
/// basename. Every entry must exist -- a missing one is a hard failure at
/// startup, never a silent skip.
const EXTRA_BOOK_DIRS: &[&str] = &[
    "pathfinder/dreamscarred_press/ultimate_psionics",
    "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
    "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
    "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
    "pathfinder/paizo/campaign_setting/inner_sea_combat",
    "pathfinder/paizo/campaign_setting/inner_sea_faiths",
    "pathfinder/paizo/campaign_setting/inner_sea_gods",
    "pathfinder/paizo/campaign_setting/inner_sea_magic",
    "pathfinder/paizo/campaign_setting/inner_sea_races",
    "pathfinder/paizo/campaign_setting/inner_sea_temples",
    "pathfinder/paizo/campaign_setting/inner_sea_taverns",
    "pathfinder/paizo/campaign_setting/inner_sea_bestiary",
    "pathfinder/paizo/campaign_setting/inner_sea_intrigue",
];

/// The levels the engine-fact sweeps evaluate. Level 1 is the creation posture
/// most operator questions are about; 5/10/15/20 reach the level-gated class
/// features (Monk's Diamond Body, Bard's Frightening Tune, Cleric's higher
/// channel dice) that a level-1-only sweep would report as ungrounded purely
/// because they are not yet available.
const SWEEP_LEVELS: &[u8] = &[1, 5, 10, 15, 20];

/// The class postures the feat-effect probe sweeps, and the reason for each.
/// Copied deliberately from `v06_content_state_dump::PROBE_CLASSES` — a feat
/// whose bonus is gated on a class resource (Extra Rage, Extra Ki, Extra
/// Panache) only shows up as a computed change on a character who *has* that
/// resource, so a single-class probe systematically undercounts.
const PROBE_CLASSES: &[&str] = &["fighter", "barbarian", "monk", "wizard", "swashbuckler"];

/// The levels the feat probe is evaluated at, matching
/// `v06_content_state_dump::PROBE_LEVELS`: 1 is the creation posture, 12 is
/// high enough that level-gated feat effects (Greater Weapon Focus needs
/// Fighter 8, Greater Weapon Specialization needs Fighter 12) are reachable.
const PROBE_LEVELS: &[u8] = &[1, 12];

/// Generic `selection_id` values the feat probe pairs with a feat's own derived
/// choice-set id, matching `v06_content_state_dump::PROBE_SELECTIONS`.
const PROBE_SELECTIONS: &[&str] = &[
    "weapon:Longsword",
    "skill:Climb",
    "school:evocation",
    "feat:dodge",
];

// ---------------------------------------------------------------------------
// Unit kinds
// ---------------------------------------------------------------------------

/// A kind of work unit. These are the six the operator named (class, race,
/// feat, spell, item, monster) split to the granularity the corpus itself
/// declares records at — a class *feature* and a race *trait* are separate
/// corpus records from their class/race, and each is separately a unit of
/// work, so collapsing them would hide most of the remaining work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Class,
    ClassFeature,
    Race,
    RaceTrait,
    Feat,
    Spell,
    Equipment,
    EquipmentModifier,
    Monster,
    /// SD28-E15 (2026-08-09): a monster's own natural attack/special
    /// quality/special attack/universal-monster-rule sub-record, DISTINCT
    /// from `Kind::Monster` (the top-level stat-block record itself).
    /// Introduced rather than folding this content into `Kind::Monster`
    /// specifically so `Kind::Monster`'s own count keeps meaning "stat
    /// blocks" -- see `refine_kind`'s own doc comment for the row-content
    /// rule that produces it, and `decisions.md §61` for the corpus
    /// evidence this variant exists to fix.
    MonsterAbility,
    Companion,
}

impl Kind {
    fn id(self) -> &'static str {
        match self {
            Kind::Class => "class",
            Kind::ClassFeature => "class_feature",
            Kind::Race => "race",
            Kind::RaceTrait => "race_trait",
            Kind::Feat => "feat",
            Kind::Spell => "spell",
            Kind::Equipment => "equipment",
            Kind::EquipmentModifier => "equipment_modifier",
            Kind::Monster => "monster",
            Kind::MonsterAbility => "monster_ability",
            Kind::Companion => "companion",
        }
    }

    /// Every kind, so `totals.by_kind` reports a real zero rather than
    /// omitting a kind a book happens not to carry.
    const ALL: &'static [Kind] = &[
        Kind::Class,
        Kind::ClassFeature,
        Kind::Race,
        Kind::RaceTrait,
        Kind::Feat,
        Kind::Spell,
        Kind::Equipment,
        Kind::EquipmentModifier,
        Kind::Monster,
        Kind::MonsterAbility,
        Kind::Companion,
    ];
}

/// Which unit kind a corpus `.lst` file declares, from its basename suffix.
///
/// `races*.lst` is deliberately ambiguous at the *file* level and resolved
/// per-record instead (see [`refine_kind`]): a Bestiary "race" record carrying
/// a `CR:` token is a monster, and the same file shape holds both.
///
/// Returns `None` for the many corpus files that declare something outside the
/// inventory's unit kinds (skills, languages, templates, deities, kits,
/// proficiencies, ability categories, data controls). Those are **recorded**
/// per book in `files_not_enumerated`, never silently dropped — a file whose
/// basename this function does not recognise shows up there by name, so a new
/// book introducing a new file shape is visible instead of invisible.
fn file_kind(basename: &str) -> Option<Kind> {
    // Order matters: `_abilities_class` and `_abilities_race` must be tested
    // before the bare `_abilities`, and `_equipmods` before `_equip`.
    if basename.contains("_abilities_class") {
        return Some(Kind::ClassFeature);
    }
    if basename.contains("_abilities_race") {
        // ...but a companion/familiar marker anywhere else in the basename
        // wins: `isi_abilities_race_companion.lst` and
        // `b4_abilities_race_ce_companion.lst` hold the racial abilities of
        // *companion creatures* (Clockwork Spy, Clockwork Familiar), not
        // racial traits of a player race. Without this narrowing the bare
        // `_abilities_race` substring claims them for `race_trait` before the
        // companion checks below are ever reached.
        if basename.contains("companion") || basename.contains("familiar") {
            return Some(Kind::Companion);
        }
        return Some(Kind::RaceTrait);
    }
    if basename.contains("_abilities_companion") || basename.contains("_abilities_familiar") {
        return Some(Kind::Companion);
    }
    if basename.contains("_races_companion") || basename.contains("_races_familiar") {
        return Some(Kind::Companion);
    }
    if basename.contains("_classes_companion") {
        return Some(Kind::Companion);
    }
    if basename.contains("_races") {
        return Some(Kind::Race);
    }
    if basename.contains("_classes") {
        return Some(Kind::Class);
    }
    if basename.contains("_feats") {
        return Some(Kind::Feat);
    }
    if basename.contains("_spells") {
        return Some(Kind::Spell);
    }
    if basename.contains("_equipmods") {
        return Some(Kind::EquipmentModifier);
    }
    if basename.contains("_equip") {
        return Some(Kind::Equipment);
    }
    None
}

// ---------------------------------------------------------------------------
// The corpus traps
// ---------------------------------------------------------------------------

/// One corpus trap: a record shape that looks like a unit and is not (or is a
/// unit that a naive rule would drop). Every entry is emitted into the JSON so
/// the consumer can see exactly which rules produced the counts, and every one
/// is pinned by a test in `tests/v06_work_inventory.rs`.
struct TrapRule {
    id: &'static str,
    description: &'static str,
}

const TRAP_RULES: &[TrapRule] = &[
    TrapRule {
        id: "comment_or_disabled",
        description:
            "A `#`-prefixed line. Some are file comments; many are DISABLED DUPLICATES of real \
             records that look completely real once the `#` is ignored. Never a unit.",
    },
    TrapRule {
        id: "directive_line",
        description:
            "A line whose first field is an ALL-CAPS `TOKEN:` directive (SOURCELONG, SOURCEPAGE, \
             CAMPAIGN, ...) rather than a record name. File metadata, never a unit.",
    },
    TrapRule {
        id: "mod_record",
        description:
            "A `<Name>.MOD` record MODIFIES an existing base record rather than declaring one. \
             Counting these inflated a feat estimate from 301 to 396. Not a unit — EXCEPT when \
             no base record for that name exists anywhere in the enumerated corpus, which is a \
             real declaration in disguise (see `mod_only_rescue`).",
    },
    TrapRule {
        id: "mod_only_rescue",
        description:
            "A `.MOD` record whose base name appears nowhere else in the corpus. Dropping these \
             blindly would report the Core Rulebook as having ZERO playable races: all seven are \
             declared as `.MOD` rows in `cr_races.lst` over bases that `core_rulebook.pcc` pulls \
             in from the shared `core_essentials` library. Emitted as a unit with \
             `origin: \"mod_only\"`.",
    },
    TrapRule {
        id: "copy_record",
        description:
            "A `<Base>.COPY=<Variant>` record declares `<Variant>` as a new named record derived \
             from `<Base>`. Emitted as a unit named `<Variant>` with `origin: \"copy\"` — the \
             equipment ingest counts these (they are distinct purchasable items; the rule \
             reproduces CRB's documented 310/453/1556 per-file figures exactly), while the CRB \
             spell ingest merges them into the base (which is what takes 674 raw spell rows down \
             to the documented 652). Tagging rather than hard-coding a per-kind choice lets a \
             consumer reproduce either count by filtering.",
    },
    TrapRule {
        id: "internal_namespace",
        description:
            "A record whose first field is `CATEGORY=Internal|...`, OR whose fields carry a \
             plain `CATEGORY:Internal` token anywhere (widened SD28-E15, 2026-08-09 -- the \
             directive-line-only check missed this second, normal-record shape, e.g. `Blue \
             Psion FC handler`). PCGen export-engine plumbing in a namespace no player ever \
             sees. Never a unit.",
    },
    TrapRule {
        id: "invisible_record",
        description:
            "A record carrying `VISIBLE:NO`. NOT dropped: `cr_equipmods.lst` alone holds 365 of \
             them and the engine's own equipment tables count them, so dropping them would \
             under-report that file by more than half. Emitted with `visible: false` so a \
             consumer that wants only player-visible records can filter, and the count \
             reconciles either way. (The 9 `VISIBLE:NO` feat helpers the CRB feat catalog \
             deliberately excludes are already excluded by `missing_classifying_token` — they \
             carry no `TYPE:` facet at all — so this rule does not reopen that hole.)",
    },
    TrapRule {
        id: "class_level_line",
        description:
            "In a `*_classes.lst`, only lines whose first field is `CLASS:<Name>` declare a \
             class; every following line is a per-level progression row for the class above it. \
             Counting all rows reports 368 CRB classes where 28 exist.",
    },
    TrapRule {
        id: "missing_classifying_token",
        description:
            "A feat row with no `TYPE:` facet, or a spell row with neither `SCHOOL:` nor \
             `CLASSES:`, is a sub-choice/`TEMPBONUS` helper rather than an independent record. \
             This rule is what makes CRB spells land on the documented 652 and CRB feats on 185.",
    },
    TrapRule {
        id: "duplicate_identity",
        description:
            "Two rows in one book+kind resolving to the same identity are ONE unit. Identity is \
             the record's `KEY:` token when present, else its display name — an \
             archetype-qualified `KEY:` differing from the display name (`KEY:Summoner Summon \
             Monster I` displayed as `Summon Monster I`) is a genuinely distinct record, and \
             keying on the display name collided 18 spells across books.",
    },
    TrapRule {
        id: "token_not_record",
        description:
            "One record can carry many tokens: a single feat holds 66 `BONUS:VAR` tokens. This \
             inventory counts RECORDS, never tokens; `magnitude_token_count` reports a record's \
             token count as a field rather than as extra rows.",
    },
    TrapRule {
        id: "book_scoped_count",
        description:
            "Every count in this document is emitted per book AND aggregated in `totals`, so a \
             per-book figure can never be read as corpus-wide. A '27 hexes' figure was APG-only \
             where the corpus holds 53 across four books.",
    },
    TrapRule {
        id: "race_favored_class_bonus_row",
        description:
            "SD28-E16 (2026-08-07). A row in a `_abilities_race.lst` file whose `TYPE:` field \
             carries a `FavoredClassBonus` dot-component. `file_kind` buckets the whole file as \
             `Kind::RaceTrait`, but a Favored Class Bonus row is a different mechanic (one row \
             per race x class) that can never appear in `race_trait_ids`. Counting it inflated \
             ARG's race_trait `not-ingested` figure by 291 units that no amount of ingestion \
             could ever close. See `decisions.md §35`.",
    },
    TrapRule {
        id: "race_choice_suboption_row",
        description:
            "SD28-E16 (2026-08-07). A `_abilities_race.lst` row carrying `CATEGORY:Choice`. This \
             is a `CHOOSE:` sub-option belonging to an already-counted parent trait (e.g. \
             `Elf ~ Elemental Resistance`'s Acid/Cold/Electricity/Fire choices), never itself an \
             independent racial trait -- every already-ingested ARG alternate trait carries \
             `CATEGORY:Special Ability`, none carry `CATEGORY:Choice`. Counting it double-counts \
             the parent. See `decisions.md §35`.",
    },
    TrapRule {
        id: "race_trait_class_level_adjustment_row",
        description:
            "SD28-E15 (2026-08-09). A `_abilities_race.lst` row whose `TYPE:` first segment \
             starts with `ClassLevelAdjustment` (e.g. Core Essentials' `+2 Charisma ~ Class \
             Level`). A real, level-based ability-score-adjustment record, but neither a racial \
             trait nor a monster ability -- excluded outright rather than forced into either \
             kind. See `decisions.md §61`.",
    },
];

// `MAGNITUDE_TOKENS` -- the tab-field prefixes that carry a real numeric
// magnitude -- is imported from `codex::rules_core::wiring_class` above.
// `wiring-class-determination.md` "Magnitude-bearing fields" is explicit
// that the determinator MUST NOT fork this list: a second copy here would
// drift from the one the generator itself uses to select magnitude fields,
// and the two would disagree about which records even have a magnitude.

// ---------------------------------------------------------------------------
// Corpus records
// ---------------------------------------------------------------------------

/// Where a record came from, so a status can always be traced back to a line.
#[derive(Debug, Clone)]
struct Provenance {
    file: String,
    line: usize,
}

/// How a record entered the inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Origin {
    /// A plain base declaration.
    Declared,
    /// The right-hand side of a `<Base>.COPY=<Variant>` row.
    Copy,
    /// A `.MOD` row rescued because no base declaration exists in the corpus.
    ModOnly,
}

impl Origin {
    fn id(self) -> &'static str {
        match self {
            Origin::Declared => "declared",
            Origin::Copy => "copy",
            Origin::ModOnly => "mod_only",
        }
    }
}

/// One enumerated corpus record, before the engine has been asked about it.
#[derive(Debug, Clone)]
struct CorpusUnit {
    /// The unit's TRUE book for reporting/attribution -- resolved off
    /// `core_essentials` to the real source book where provable
    /// (`resolve_true_book_for_core_essentials`, `SD31-ATTRIB-001`), never
    /// used for a physical file lookup: it may name a book whose own
    /// `book_paths` directory does not physically contain this row.
    book: String,
    /// The book directory `enumerate_book` actually WALKED to find this
    /// row -- i.e. what `book` always equaled before the `SD31-ATTRIB-001`
    /// re-attribution fix. This is the only field safe to pass to
    /// `token_closure_rows`/`CorpusLines::line`, which resolve a physical
    /// path via `book_paths[book_id]` and would silently fail to find the
    /// row (falling back to a weaker `wiring_class` verdict) if handed the
    /// re-attributed `book` instead.
    source_book: String,
    kind: Kind,
    /// The record's corpus identity: its `KEY:` token when present, else its
    /// display name.
    key: String,
    /// The record's display name (the first tab field, `.COPY=` resolved).
    name: String,
    origin: Origin,
    provenance: Provenance,
    /// How many [`MAGNITUDE_TOKENS`]-prefixed tab fields this record carries.
    magnitude_token_count: usize,
    /// The `TYPE:` facet string, when the record carries one.
    type_facet: Option<String>,
    /// `false` when the record carries `VISIBLE:NO`. See the
    /// `invisible_record` trap rule for why these are kept rather than cut.
    visible: bool,
}

/// One `.MOD` target `enumerate_file` stashed for `mod_only_rescue`:
/// `(kind, key, name, provenance, magnitude_token_count, resolved_book)`.
/// `resolved_book` is the true book when the row was found inside
/// `core_essentials/` and [`resolve_true_book_for_core_essentials`] resolved
/// it (`None` otherwise, including for every non-`core_essentials` book) --
/// carried through so `mod_only_rescue`'s own re-attribution stamps the same
/// book [`enumerate_file`] would have, rather than falling back to the
/// enumerating `BookMeta`'s id.
type ModTarget = (Kind, String, String, Provenance, usize, Option<&'static str>);

/// Per-book enumeration bookkeeping the JSON reports verbatim.
#[derive(Debug, Default)]
struct BookEnumeration {
    units: Vec<CorpusUnit>,
    trap_hits: BTreeMap<&'static str, usize>,
    files_enumerated: usize,
    files_not_enumerated: BTreeSet<String>,
    /// `.MOD` target names seen in this book, kept so `mod_only_rescue` can
    /// run after the whole corpus is known. See [`ModTarget`].
    mod_targets: Vec<ModTarget>,
}

/// Tokenise one `.lst` line into its tab fields with surrounding whitespace
/// preserved-but-trimmed per field, matching how PCGen itself reads them.
fn tab_fields(line: &str) -> Vec<&str> {
    line.trim_end_matches(['\n', '\r']).split('\t').collect()
}

// `mod_base_name` (resolving a `.MOD` row's base record name) lives in
// `codex::rules_core::wiring_class` -- imported below -- and is shared by
// this file's `mod_only_rescue` path and `wiring_class`'s own token-closure
// index, so the two always agree about which record a `.MOD` row belongs
// to (the same resolution `wiring-class-determination.py`'s `mod_index()`
// performs).

/// The value of the first field with the given `TOKEN:` prefix, if any.
fn token_value<'a>(fields: &[&'a str], prefix: &str) -> Option<&'a str> {
    fields
        .iter()
        .find(|f| f.starts_with(prefix))
        .map(|f| &f[prefix.len()..])
}

/// Whether any tab field starts with the given prefix.
fn has_token(fields: &[&str], prefix: &str) -> bool {
    fields.iter().any(|f| f.starts_with(prefix))
}

/// One `DESC:` field's value, trimmed, is a REAL description iff it is
/// non-empty and not one of the markers that mean "nothing here": `.CLEAR`/
/// `.CLEARALL` remove a prior row's description rather than stating one, and
/// `[redacted PI]` is the shipped PI-screen marker -- a player sees that
/// literal string, not the rulebook's prose, so it does not satisfy
/// Decision 7's condition 3 either.
fn is_real_description_value(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !matches!(lower.as_str(), ".clear" | ".clearall" | "[redacted pi]")
}

/// Whether a unit's full token closure (its base corpus row plus every
/// `.MOD` row targeting it — [`token_closure_rows`]'s own output, the SAME
/// closure `carries_prose_magnitude`'s `wc_reason` is derived from) carries
/// a real, non-empty `DESC:` value anywhere.
///
/// SD31-D7-PROSE-001 (Decision 7's condition 3): a record can be `known` to
/// the engine and carry zero `MAGNITUDE_TOKENS` fields while still having
/// nothing at all to show a player — no `DESC:` token anywhere, corpus
/// `description: null`. `magnitude_token_count == 0` alone answers
/// conditions 1/2 ("prose only", "nothing to compute"); this function is
/// condition 3's own check, over the identical closure the rest of this
/// file's classifier already builds.
fn closure_has_real_description(row_refs: &[Option<&str>]) -> bool {
    row_refs.iter().flatten().any(|line| {
        tab_fields(line)
            .iter()
            .filter_map(|f| f.strip_prefix("DESC:"))
            .any(is_real_description_value)
    })
}

/// SD31-D7-PROSE-002 (Decision 7's condition 3, `OPEN-ISSUES.md` row 70):
/// the SECOND source for "this unit's corpus record carries a real
/// description", alongside [`closure_has_real_description`]'s raw `.lst`
/// closure -- consults [`EngineFacts::corpus_json_descriptions`], joined on
/// the identical `(source_file, source_line)` coordinate every other rung in
/// this file already uses, plus the record's own key (see that field's doc
/// comment for why the coordinate alone is ambiguous). Recovers a `.COPY=`
/// record's description that was resolved by INHERITANCE at ingest time --
/// a fact the raw `.lst` text can never carry on its own, since the
/// inheritance resolution never touches the `.lst` file.
fn corpus_json_has_real_description(
    descriptions: &BTreeMap<(String, usize, String), String>,
    file: &str,
    line: usize,
    key: &str,
) -> bool {
    descriptions.contains_key(&(file.to_string(), line, key.to_string()))
}

/// CONFIRMED finding (integration-cycle adversarial review, `SD31-W6-
/// INTEGRATE-001`): 5 `equipment_modifier` units promoted by the
/// `corpus_json_descriptions` recovery rung ship the raw PCGen token
/// `%CHOICE` verbatim to the player -- the equipment render path
/// (`equipment_catalog::serve_description`), unlike the monster and
/// companion catalogs, carries no leak guard at all, and
/// `leaked_pcgen_syntax` itself only flagged `%` followed by a DIGIT until
/// this same finding's fix widened it to `%<UPPERCASE-KEYWORD>` too. Refuse
/// the description-completeness promotion for any unit whose recovered
/// `data.description` still carries an unresolved PCGen substitution --
/// Decision 7 condition 3 ("the prose is available to print... on the
/// character sheet") is not met by text a player would see with raw syntax
/// in it.
fn corpus_json_description_leaks_pcgen_syntax(
    descriptions: &BTreeMap<(String, usize, String), String>,
    file: &str,
    line: usize,
    key: &str,
) -> bool {
    descriptions
        .get(&(file.to_string(), line, key.to_string()))
        .is_some_and(|desc| leaked_pcgen_syntax(desc).is_some())
}

#[cfg(test)]
mod closure_has_real_description_tests {
    use super::*;

    /// The proof case: a real DESC: value on the base row is found.
    #[test]
    fn finds_a_real_desc_on_the_base_row() {
        let row = Some("Foo\tTYPE:General\tDESC:You do a thing.");
        assert!(closure_has_real_description(&[row]));
    }

    /// A `.MOD` continuation row can be where the real DESC: lives — the
    /// closure must be searched, not just the base row.
    #[test]
    fn finds_a_real_desc_on_a_mod_row_when_the_base_row_has_none() {
        let base = Some("Foo\tTYPE:General");
        let mod_row = Some("Foo.MOD\tDESC:You do a thing.");
        assert!(closure_has_real_description(&[base, mod_row]));
    }

    /// SD31-D7-PROSE-001's own failure mode, proven refused: no DESC: token
    /// anywhere in the closure.
    #[test]
    fn refuses_a_closure_with_no_desc_token_at_all() {
        let row = Some("Foo\tTYPE:General\tSOURCEPAGE:p.1");
        assert!(!closure_has_real_description(&[row]));
    }

    /// `DESC:.CLEAR` / `DESC:.CLEARALL` remove a prior description rather
    /// than stating one; must not count as real text.
    #[test]
    fn refuses_a_desc_clear_marker() {
        assert!(!closure_has_real_description(&[Some("Foo\tDESC:.CLEAR")]));
        assert!(!closure_has_real_description(&[Some("Foo\tDESC:.CLEARALL")]));
    }

    /// The shipped PI-redaction marker is not the rulebook's prose reaching
    /// the player — it must not satisfy condition 3 either.
    #[test]
    fn refuses_the_pi_redaction_marker() {
        assert!(!closure_has_real_description(&[Some("Foo\tDESC:[redacted PI]")]));
    }

    /// A DESC: field that is present but blank (whitespace only) is not
    /// real text.
    #[test]
    fn refuses_a_blank_desc_value() {
        assert!(!closure_has_real_description(&[Some("Foo\tDESC:   ")]));
    }

    /// A `None` row (no corpus line resolved at all, D0) contributes
    /// nothing and must not panic.
    #[test]
    fn a_missing_row_is_skipped_not_treated_as_a_hit() {
        assert!(!closure_has_real_description(&[None, None]));
    }
}

#[cfg(test)]
mod corpus_json_has_real_description_tests {
    use super::*;

    /// The proof case (OPEN-ISSUES row 70's own example): `core_rulebook:
    /// equipment:scale_mail`'s corpus JSON carries a real, ingest-time
    /// INHERITED description ("Scale mail is made up of dozens of small
    /// overlapping metal plates...") even though its own `.COPY=` `.lst`
    /// row carries no `DESC:` token at all -- `closure_has_real_description`
    /// alone would refuse this unit; this second source recovers it.
    #[test]
    fn finds_a_real_description_by_the_coordinate_and_key_join() {
        let mut descriptions = BTreeMap::new();
        descriptions.insert(
            ("cr_equip_arms_armor.lst".to_string(), 55, "Scale Mail".to_string()),
            "Scale mail is made up of dozens of small overlapping metal plates.".to_string(),
        );
        assert!(corpus_json_has_real_description(
            &descriptions,
            "cr_equip_arms_armor.lst",
            55,
            "Scale Mail",
        ));
    }

    /// A coordinate the map has nothing for (no `.lst`-sourced JSON record,
    /// or a `source.kind: "web_second_source"` record with no `path`/`line`
    /// at all, so nothing was ever inserted for it) is refused, not treated
    /// as a hit -- the honest "we found no second source" result.
    #[test]
    fn refuses_a_coordinate_the_map_has_no_entry_for() {
        let descriptions = BTreeMap::new();
        assert!(!corpus_json_has_real_description(
            &descriptions,
            "cr_equip_arms_armor.lst",
            55,
            "Scale Mail",
        ));
    }

    /// PROVE THE JOIN CAN FAIL, the ambiguous-coordinate case: two distinct
    /// records sharing one `.lst` line (`acg_equipmods.lst:41` is both
    /// `Flying` and `Special Ability ~ Flying ~ Melee`, a real corpus
    /// coordinate) must each be reachable ONLY by their own key -- the
    /// coordinate alone is not a safe join, which is exactly why `key` is a
    /// THIRD component of the tuple, not an afterthought.
    #[test]
    fn a_shared_lst_line_resolves_each_record_by_its_own_key_only() {
        let mut descriptions = BTreeMap::new();
        descriptions.insert(
            ("acg_equipmods.lst".to_string(), 41, "Flying".to_string()),
            "A flying special ability description.".to_string(),
        );
        assert!(corpus_json_has_real_description(
            &descriptions,
            "acg_equipmods.lst",
            41,
            "Flying",
        ));
        assert!(!corpus_json_has_real_description(
            &descriptions,
            "acg_equipmods.lst",
            41,
            "Special Ability ~ Flying ~ Melee",
        ));
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// The end-to-end proof, against the REAL on-disk corpus, not a
    /// synthetic fixture: `data/corpus/core_rulebook/equipment/scale_mail.json`
    /// (`OPEN-ISSUES.md` row 70's own named example) is a `source.kind:
    /// "lst_inherited_copy"` record whose `data.description` is real and
    /// whose own `.lst` row (`cr_equip_arms_armor.lst:55`, a `.COPY=`
    /// variant) carries no `DESC:` token at all. `load_corpus_json_descriptions`
    /// must recover it.
    #[test]
    fn the_real_corpus_loader_recovers_scale_mails_inherited_description() {
        let descriptions = load_corpus_json_descriptions(&repo_root());
        let hit = descriptions.get(&(
            "cr_equip_arms_armor.lst".to_string(),
            55,
            "Scale Mail".to_string(),
        ));
        assert!(
            hit.is_some_and(|d| d.contains("dozens of small overlapping metal plates")),
            "expected scale_mail's real inherited description, got {hit:?}"
        );
    }
}

#[cfg(test)]
mod corpus_json_description_leaks_pcgen_syntax_tests {
    use super::*;

    /// The proof case (CONFIRMED finding, `SD31-W6-INTEGRATE-001`): the
    /// real shipped shape of `ultimate_equipment:equipment_modifier:
    /// special_ability_defiant_armor`'s recovered description.
    #[test]
    fn catches_the_real_percent_choice_shape() {
        let mut descriptions = BTreeMap::new();
        descriptions.insert(
            ("ue_equipmods.lst".to_string(), 1541, "Special Ability ~ Defiant".to_string()),
            "+2 enhancement bonus and DR 2/- against %CHOICE".to_string(),
        );
        assert!(corpus_json_description_leaks_pcgen_syntax(
            &descriptions,
            "ue_equipmods.lst",
            1541,
            "Special Ability ~ Defiant",
        ));
    }

    /// A clean description (no leaked syntax) must not be flagged.
    #[test]
    fn does_not_flag_a_clean_description() {
        let mut descriptions = BTreeMap::new();
        descriptions.insert(
            ("cr_equip_arms_armor.lst".to_string(), 55, "Scale Mail".to_string()),
            "Scale mail is made up of dozens of small overlapping metal plates.".to_string(),
        );
        assert!(!corpus_json_description_leaks_pcgen_syntax(
            &descriptions,
            "cr_equip_arms_armor.lst",
            55,
            "Scale Mail",
        ));
    }

    /// A coordinate with no entry at all must not be flagged (nothing to
    /// leak if there is no recovered description).
    #[test]
    fn a_missing_coordinate_is_not_flagged() {
        let descriptions = BTreeMap::new();
        assert!(!corpus_json_description_leaks_pcgen_syntax(
            &descriptions,
            "ue_equipmods.lst",
            1541,
            "Special Ability ~ Defiant",
        ));
    }
}

#[cfg(test)]
mod equipment_verdict_rung_tests {
    use super::*;

    fn equipment_modifier_unit(file: &str, line: usize, key: &str) -> CorpusUnit {
        CorpusUnit {
            book: "ultimate_equipment".to_string(),
            source_book: "ultimate_equipment".to_string(),
            kind: Kind::EquipmentModifier,
            key: key.to_string(),
            name: key.to_string(),
            origin: Origin::Declared,
            provenance: Provenance { file: file.to_string(), line },
            magnitude_token_count: 0,
            type_facet: None,
            visible: true,
        }
    }

    /// PROVE THE RUNG CAN FAIL (CONFIRMED finding, `SD31-W6-INTEGRATE-001`):
    /// an `equipment_modifier` unit whose recovered corpus description
    /// carries a raw, unresolved `%CHOICE` must NOT read `text-complete` --
    /// the equipment render path has no leak guard of its own, so this
    /// promotion is the last chance to catch it before it reaches a
    /// player's screen verbatim.
    #[test]
    fn a_recovered_description_carrying_percent_choice_does_not_read_text_complete() {
        let mut facts = EngineFacts::default();
        facts
            .equipment_keys
            .entry("ultimate_equipment")
            .or_default()
            .insert("Special Ability ~ Defiant".to_string());
        facts.corpus_json_descriptions.insert(
            ("ue_equipmods.lst".to_string(), 1541, "Special Ability ~ Defiant".to_string()),
            "+2 enhancement bonus and DR 2/- against %CHOICE".to_string(),
        );
        let unit =
            equipment_modifier_unit("ue_equipmods.lst", 1541, "Special Ability ~ Defiant");
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "text-complete");
        // Falls through to the same "nothing safe to show a player" verdict
        // Decision 7's own condition-3 refusal uses for a genuinely
        // description-less record -- `unknown`, not a fabricated `done` or
        // `held` credit.
        assert_eq!(
            verdict.status, "unknown",
            "expected the leak refusal to fall through to unknown, got status {:?}", verdict.status
        );
    }

    /// The control case: a clean recovered description DOES read
    /// `text-complete`, proving the refusal above is scoped to the leak,
    /// not a blanket regression on the whole rung.
    #[test]
    fn a_clean_recovered_description_still_reads_text_complete() {
        let mut facts = EngineFacts::default();
        facts
            .equipment_keys
            .entry("ultimate_equipment")
            .or_default()
            .insert("Special Ability ~ Clean".to_string());
        facts.corpus_json_descriptions.insert(
            ("ue_equipmods.lst".to_string(), 1600, "Special Ability ~ Clean".to_string()),
            "+2 enhancement bonus and fire resistance 5.".to_string(),
        );
        let unit = equipment_modifier_unit("ue_equipmods.lst", 1600, "Special Ability ~ Clean");
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_eq!(verdict.status, "text-complete");
    }
}

/// `"Fast Movement"` -> `"fast_movement"`. The engine's own explanation-id
/// naming rule, applied to a corpus name so the two can be joined without a
/// hand-maintained mapping table (the same discipline
/// `v06_content_state_dump::derived_choice_set_id` uses, and the same honest
/// failure direction: a record whose engine id uses a different shape falls
/// out and reads as un-grounded rather than being fabricated as grounded).
fn slug(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut last_underscore = true;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            last_underscore = false;
        } else if !last_underscore {
            out.push('_');
            last_underscore = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    out
}

/// A stable 64-bit FNV-1a digest of one exact corpus key, rendered as sixteen
/// lowercase hex characters. Used only by [`unit_id`] to disambiguate two
/// distinct corpus keys whose [`slug`]s collide.
///
/// # Why FNV-1a and not [`std::collections::hash_map::DefaultHasher`]
///
/// `DefaultHasher`'s algorithm is explicitly documented as unspecified and
/// free to change between Rust releases. Hanging a field of this file's own
/// output on it would mean a toolchain upgrade silently rewrites every
/// disambiguated id — breaking, in the one place it is hardest to notice, the
/// byte-equality contract this whole function exists to protect. FNV-1a is a
/// fixed specification with fixed constants, so the digest is a function of
/// the key alone: same key, same sixteen characters, on any machine, under any
/// compiler, forever.
fn key_digest(key: &str) -> String {
    // FNV-1a, 64-bit: offset basis 0xcbf29ce484222325, prime 0x100000001b3.
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in key.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// One unit's `id` — the handle every downstream consumer keys on — built so
/// that it is **unique**, which before this function it was not.
///
/// # The defect this closes
///
/// `id` is `<book>:<kind>:<slug of the corpus key>`, but `duplicate_identity`
/// de-duplicates on the *exact* corpus key, and [`slug`] is deliberately lossy:
/// it collapses every run of non-alphanumerics to one `_`. So two genuinely
/// distinct records in one book+kind could — and did — land on one id.
/// `Path Skill Acrobatics` and `Path Skill ~ Acrobatics`, `MITHRAL_ITEM` and
/// `Mithral (Item)`, `Half-Elf ~ Drow Blooded` and `Half-Elf ~ Drow-Blooded`:
/// twenty-nine ids in the corpus-wide run carried two rows each, and
/// twenty-seven of those pairs disagreed about `wiring_class`, nineteen of them
/// as `computed` against `display`.
///
/// That is not a cosmetic flaw. Every consumer that indexes the inventory by
/// `id` — a `{u["id"]: u for u in units}` in Python, a `jq INDEX(.id)`, a
/// pandas `set_index` — keeps exactly one row per id and drops the other, and
/// *which* one it keeps is that consumer's own last-wins or first-wins detail.
/// Point two such indexes at a before snapshot and an after snapshot and the
/// dropped row can differ between them, manufacturing a `wiring_class`
/// transition that no code change caused. That is precisely the phantom
/// nineteen-unit `computed -> display` move recorded as a near-miss in
/// `docs/retro/events/wiring-classifier.jsonl`, and it was read there as
/// run-to-run nondeterminism in this generator. It is not: five consecutive
/// runs of one binary over one corpus are byte-identical. The generator was
/// deterministic and its output was ambiguous.
///
/// # The tie-break rule, and why it is the correct one
///
/// * **Suffix, never merge.** Two distinct corpus keys are two records. Making
///   them one unit would change a count, and this is a fix to an identifier,
///   not a ruling about content.
/// * **Every colliding unit is suffixed; none keeps the bare slug.** A rule
///   that let one member win the unsuffixed id would have to pick a winner, and
///   every available criterion — enumeration order, line number, lexical order
///   of the key — makes an id that moves when something *else* moves.
/// * **The suffix is a digest of the unit's own exact corpus key.** An ordinal
///   (`__1`, `__2`) would depend on the *set* of colliding siblings, so
///   ingesting one new row could renumber a unit that had not changed. A digest
///   of the key alone cannot: a unit's id is a function of its own identity.
/// * **`__` is an unambiguous delimiter.** [`slug`] collapses every run of
///   non-alphanumerics to a single `_`, so no slug can ever contain `__`, and no
///   suffixed id can be mistaken for an unsuffixed one.
/// * **Non-colliding units keep the id they have always had.** Only the rows
///   that were actually broken move, so this fix costs no downstream consumer a
///   re-pin it does not owe.
///
/// `collides` is the caller's answer to "does more than one unit in this
/// book+kind slug to this?", counted over the de-duplicated unit set.
fn unit_id(book: &str, kind: Kind, key: &str, collides: bool) -> String {
    let base = slug(key);
    if collides {
        format!("{book}:{}:{base}__{}", kind.id(), key_digest(key))
    } else {
        format!("{book}:{}:{base}", kind.id())
    }
}

/// SD28-E15 (2026-08-09): `_abilities_race.lst`'s own `TYPE:` first-segment
/// vocabulary that names a monster's own sub-ability rather than a player
/// racial trait -- PF1's own Bestiary terminology (`NaturalAttack`,
/// `SpecialAttack`, `SpecialQuality`, `Universal Monster Rule`), not this
/// tool's invention. Confirmed against two real files before ruling: Core
/// Essentials' `ce_abilities_race.lst` (776 `NaturalAttack` + 247
/// `SpecialQuality` + 104 `SpecialAttack` + 28 `Universal Monster Rule` =
/// 1,155 real, non-comment rows) and Bestiary 1's `b1_abilities_race.lst`
/// (273 `SpecialAttack` + 250 `SpecialQuality` + 1 `NaturalAttack` = 524,
/// in a book with zero playable races). See `decisions.md §61` for the
/// full survey, including the facets deliberately left unaddressed this
/// pass (a handful of monster-template-specific facets like
/// `HalfDragonType`/`UnicornRacialTrait` -- real content, likely also
/// monster-shaped, not reclassified here for lack of the same
/// cross-book-repeated evidence the four facets below have).
const MONSTER_ABILITY_TYPE_FACETS: &[&str] =
    &["NaturalAttack", "SpecialAttack", "SpecialQuality", "Universal Monster Rule"];

/// SD28-E15 (2026-08-09): `_abilities_race.lst`'s own `ClassLevelAdjustment*`
/// `TYPE:` facets (`ce_abilities_race.lst`'s own `+2/+4 Charisma ~ Class
/// Level`, `-2 Charisma ~ Class Level`) are level-based ability-score
/// adjustment records -- neither a racial trait nor a monster ability, and
/// forcing them into either kind would be the "convenient kind for an
/// ambiguous row" failure `§32` guards against. Excluded outright (this
/// tool's own established shape: named, counted, not silently kept as
/// `RaceTrait`), the same disposition the `race_favored_class_bonus_row`/
/// `race_choice_suboption_row` traps already established for other
/// `_abilities_race.lst` rows that are real content in the wrong bucket.
const RACE_TRAIT_EXCLUDED_TYPE_PREFIX: &str = "ClassLevelAdjustment";

/// SD31-E6-F9-001 (2026-08-16, `OPEN-ISSUES.md` row 34): a
/// `SpecialQuality`/`SpecialAttack`-first-segment `_abilities_race.lst` row is
/// STILL a player race+class mechanic row, not a monster ability, when it carries
/// either corpus-stated shape below. Neither is invented by this tool; both are
/// PCGen's own naming conventions for a Favored Class Bonus record, confirmed
/// **absent** (0 occurrences, `grep -c` over the raw `.lst`) from every file this
/// program currently ingests `monster_ability` from that is NOT one of these two --
/// `core_essentials/ce_abilities_race.lst`, every registered Bestiary
/// (`bestiary`/`bestiary_2`/`bestiary_3`/`bestiary_4`/`bestiary_5`/`bestiary_6`,
/// `bonus_bestiary`, `monster_codex`, `inner_sea_bestiary`), and
/// `inner_sea_gods`/`pathfinder_unchained`/`horror_adventures`/`book_of_the_damned_volume_1`/`_2`.
///
/// - The `TYPE:` **second** segment ends in the literal word `Choice`, e.g.
///   `SpecialQuality.ElfHunterCritialConfirmationChoice`
///   (`advanced_class_guide/acg_abilities_race.lst`) or
///   `SpecialQuality.ElfShamanHexRangeChoice`
///   (`acg_abilities_race.lst` and, re-derived this cycle, the not-yet-ingested
///   `player_companion/heroes_of_the_wild/hotw_abilities_race.lst`) -- PCGen's own
///   naming for a Favored Class Bonus sub-choice option table entry. No genuine
///   monster-ability `TYPE:` second segment observed anywhere in this corpus ends
///   in `Choice` (the confirmed vocabulary is `Extraordinary`/`Supernatural`/
///   `SpellLike`/`PermanencySpell`/`Immunity`/`Vision`/`Defensive`/`Communicate`/
///   `Special Attack`/`Special Ability`, plus per-book creature-template facets
///   like `OgrekinDisadvantageousDeformity`/`LycanthropeKind` -- none end in
///   `Choice` either).
/// - Any field carries the literal corpus string `Favored Class Bonus`, e.g.
///   `KEY:Favored Class Bonus Output ~ Shifter ~ Dwarf`
///   (`ultimate_wilderness/uw_abilities_race.lst`) -- PCGen's own
///   `Favored Class Bonus Output ~ <class> ~ <race>` KEY convention for the
///   auto-granted **display** row a Favored Class Bonus choice feeds (the row
///   with a bare, second-segment-less `TYPE:SpecialQuality` that the
///   `race_favored_class_bonus_row` trap below cannot catch, because that trap
///   reads only the row's OWN `TYPE:`, and this row's own `TYPE:` never carries a
///   `FavoredClassBonus` dot-component) -- or the `FavClassBonus`-suffixed
///   `DEFINE:`/`BONUS:VAR` variable-name convention every Favored Class Bonus row
///   in this corpus uses, e.g. `DEFINE:HalfOrcHunterFavClassBonus|0`.
///
/// Re-derived corpus-wide before this ruling, not assumed: of the 486 `monster_ability`
/// `not-ingested` units `OPEN-ISSUES.md` row 34 flagged across `advanced_class_guide`
/// (106) and `core_essentials` (380), this test moves **106 of 106** ACG units (100%
/// of that file's facet-matching content; `acg_abilities_race.lst` carries 0
/// `NaturalAttack`/`Universal Monster Rule` rows anywhere) and **0 of 380** CE units
/// (CE's facet-matching content is 100% genuine `Extraordinary`/`Supernatural`/
/// `SpellLike`/`Universal Monster Rule` vocabulary, matching the SD28-E15 precedent
/// this const's own doc comment already established) -- row 34's inclusion of CE as
/// also-misclassified does not survive a one-record-deep check and is corrected via
/// `retro.py correction` alongside this fix. A THIRD book this same audit newly
/// found misclassified, `ultimate_wilderness` (50 of its 52 `monster_ability` units;
/// the 2 real exceptions, "Plant Traits" and "Leshy Traits", are genuine
/// Bestiary-appendix creature-type traits with `TYPE:SpecialQuality.Extraordinary`
/// and neither test above matches), moves alongside ACG under the same two
/// corpus-stated tests -- proof this is a general row-content fix, not a
/// book-specific patch: no book name appears anywhere in the test itself.
fn is_player_favored_class_choice_row(fields: &[&str], type_second_segment: &str) -> bool {
    if type_second_segment.ends_with("Choice") {
        return true;
    }
    fields
        .iter()
        .any(|f| f.contains("Favored Class Bonus") || f.contains("FavClassBonus"))
}

/// A record's kind, refined from the file-level guess by what the record
/// itself says.
///
/// - A `*_races.lst` row carrying a `CR:` token is a monster: that is the
///   corpus's own discriminator (`cr_races.lst` carries zero `CR:` tokens
///   across its seven playable races; `b1_races.lst` carries 351).
/// - A `*_abilities_race.lst` row whose `TYPE:` first segment names a real
///   monster-ability shape (`MONSTER_ABILITY_TYPE_FACETS`) is a monster's
///   own sub-ability, not a racial trait -- `file_kind`'s own whole-file
///   `_abilities_race` -> `Kind::RaceTrait` guess was always a file-level
///   approximation; this is the row-content correction `§55`/`§56` already
///   proved necessary for `race_trait`'s own declared counts -- UNLESS the row
///   itself is a player race+class Favored Class Bonus row wearing a monster
///   facet's `TYPE:` first segment (`is_player_favored_class_choice_row`,
///   `OPEN-ISSUES.md` row 34).
fn refine_kind(file_kind: Kind, fields: &[&str]) -> Kind {
    match file_kind {
        Kind::Race if has_token(fields, "CR:") => Kind::Monster,
        Kind::RaceTrait => {
            let type_value = token_value(fields, "TYPE:").unwrap_or("");
            let mut type_segments = type_value.split('.');
            let type_first_segment = type_segments.next().unwrap_or("");
            let type_second_segment = type_segments.next().unwrap_or("");
            if MONSTER_ABILITY_TYPE_FACETS.contains(&type_first_segment)
                && !is_player_favored_class_choice_row(fields, type_second_segment)
            {
                Kind::MonsterAbility
            } else {
                Kind::RaceTrait
            }
        }
        other => other,
    }
}

#[cfg(test)]
mod refine_kind_monster_ability_tests {
    use super::*;

    /// `acg_abilities_race.lst:331` (`Kind::RaceTrait` file-level guess), verbatim
    /// field shape -- a Favored Class Bonus sub-choice option table entry wearing
    /// `TYPE:SpecialQuality`, one of `MONSTER_ABILITY_TYPE_FACETS`'s own facets.
    /// Before this fix `refine_kind` promoted this to `Kind::MonsterAbility`;
    /// `OPEN-ISSUES.md` row 34 is exactly this shape, 106 of 106 in this file.
    #[test]
    fn acg_elf_hunter_choice_row_stays_race_trait_not_monster_ability() {
        let fields = [
            "Longbow",
            "KEY:Elf Hunter Critical Confirmation Choice ~ Longbow",
            "CATEGORY:Special Ability",
            "TYPE:SpecialQuality.ElfHunterCritialConfirmationChoice",
            "VISIBLE:YES",
            "DEFINE:ElfHunterCritConfLongbowBonus|0",
            "DESC:Gain a bonus.",
            "BONUS:VAR|ElfHunterCritConfLongbowBonus|1",
        ];
        assert_eq!(refine_kind(Kind::RaceTrait, &fields), Kind::RaceTrait);
    }

    /// `uw_abilities_race.lst:235` -- the auto-granted "Output" display row a
    /// Favored Class Bonus choice feeds. Its OWN `TYPE:` is bare `SpecialQuality`
    /// (no `FavoredClassBonus` dot-component), so the pre-existing
    /// `race_favored_class_bonus_row` trap (which reads only the row's own
    /// `TYPE:`) cannot catch it -- this is the second, independent shape row 34's
    /// fix must also close.
    #[test]
    fn uw_favored_class_bonus_output_row_stays_race_trait_not_monster_ability() {
        let fields = [
            "Wild Empathy Bonus",
            "KEY:Favored Class Bonus Output ~ Shifter ~ Dwarf",
            "CATEGORY:Special Ability",
            "TYPE:SpecialQuality",
            "VISIBLE:EXPORT",
            "DESC:Add a bonus on wild empathy checks.|DwarfShifterEmpathyBonus/2",
        ];
        assert_eq!(refine_kind(Kind::RaceTrait, &fields), Kind::RaceTrait);
    }

    /// Same shape, caught via the `FavClassBonus`-suffixed variable-name
    /// convention rather than the literal `Favored Class Bonus` KEY text --
    /// `acg_abilities_race.lst:316`'s "Animal Companion Hit Points" row, one of
    /// the 10 ACG rows whose `TYPE:` is bare `SpecialQuality` (no `...Choice`
    /// second segment) and whose KEY does not literally contain the phrase
    /// `Favored Class Bonus`.
    #[test]
    fn acg_bare_specialquality_row_with_favclassbonus_variable_stays_race_trait() {
        let fields = [
            "Animal Companion Hit Points",
            "KEY:Half-orc ~ Hunter ~ Animal Companion Hit Points",
            "CATEGORY:Special Ability",
            "TYPE:SpecialQuality",
            "DESC:Your animal companion has extra hit points.|HalfOrcHunterFavClassBonus",
        ];
        assert_eq!(refine_kind(Kind::RaceTrait, &fields), Kind::RaceTrait);
    }

    /// `ce_abilities_race.lst:1739`, one of Core Essentials' 380 genuine
    /// monster/creature-type-trait rows this fix must NOT move -- confirmed by
    /// this cycle's own corpus-wide re-derivation to be legitimate
    /// `Extraordinary`/`Supernatural`/`SpellLike` vocabulary, not player content,
    /// contra `OPEN-ISSUES.md` row 34's inclusion of CE in its 486-unit claim.
    #[test]
    fn ce_aberration_traits_output_row_still_becomes_monster_ability() {
        let fields = [
            "Aberration Traits Output",
            "OUTPUTNAME:Aberration Traits",
            "CATEGORY:Special Ability",
            "TYPE:SpecialQuality.Extraordinary",
            "DESC:Aberrations breathe, eat, and sleep.",
        ];
        assert_eq!(refine_kind(Kind::RaceTrait, &fields), Kind::MonsterAbility);
    }

    /// `uw_abilities_race.lst:25`, one of `ultimate_wilderness`'s 2 genuine
    /// exceptions among its 52 `monster_ability` units -- a creature-TYPE trait
    /// (every Plant-type creature, not a player race), same `Extraordinary`
    /// vocabulary as the CE case above, must also still promote.
    #[test]
    fn uw_plant_traits_output_row_still_becomes_monster_ability() {
        let fields = [
            "Plant Traits",
            "KEY:Plant Traits Output (PC)",
            "CATEGORY:Special Ability",
            "TYPE:SpecialQuality.Extraordinary",
            "DESC:Plants breathe and eat, but do not sleep.",
        ];
        assert_eq!(refine_kind(Kind::RaceTrait, &fields), Kind::MonsterAbility);
    }

    /// A bare `NaturalAttack`/`Universal Monster Rule` first segment is
    /// unambiguous on its own (corpus-wide: 0 false positives, see this fix's own
    /// doc comment) and must never be gated by `is_player_favored_class_choice_row`
    /// even when a coincidental `Choice`-suffixed or `Favored Class Bonus`-bearing
    /// field is also present -- regression guard so a future edit cannot widen the
    /// gate onto these two self-evident facets.
    #[test]
    fn natural_attack_first_segment_is_never_gated_by_the_choice_test() {
        let fields = ["Bite 1 (Medium)", "CATEGORY:Special Ability", "TYPE:NaturalAttack"];
        assert_eq!(refine_kind(Kind::RaceTrait, &fields), Kind::MonsterAbility);
    }
}

/// True when a `Kind::RaceTrait`-guessed row's own `TYPE:` first segment
/// names the `ClassLevelAdjustment*` shape -- real content, but neither a
/// racial trait nor (unlike `MONSTER_ABILITY_TYPE_FACETS`) a monster
/// ability either. See `RACE_TRAIT_EXCLUDED_TYPE_PREFIX`'s own doc comment.
fn is_excluded_race_trait_row(file_kind: Kind, fields: &[&str]) -> bool {
    file_kind == Kind::RaceTrait
        && token_value(fields, "TYPE:")
            .and_then(|t| t.split('.').next())
            .map(|first| first.starts_with(RACE_TRAIT_EXCLUDED_TYPE_PREFIX))
            .unwrap_or(false)
}

/// Whether a record of this kind carries the token that proves it is an
/// independent record rather than a sub-choice helper. See the
/// `missing_classifying_token` trap rule.
fn has_classifying_token(kind: Kind, fields: &[&str]) -> bool {
    match kind {
        Kind::Feat => has_token(fields, "TYPE:"),
        Kind::Spell => has_token(fields, "SCHOOL:") || has_token(fields, "CLASSES:"),
        _ => true,
    }
}

/// `core_essentials/races/<slug>/` -> the true book, for the races whose
/// attribution is provable one record deep against an in-scope book's own
/// `.pcc` file (`OPEN-ISSUES.md` row 68; `decisions.md §25.2`).
///
/// Neither `core_essentials`'s own files carries a usable signal for this --
/// verified against `dwarf_races.lst` and every `_race.pcc`: no
/// `SOURCELONG`/`SOURCESHORT` token anywhere, only the placeholder
/// `SOURCEPAGE:p.xx` `decisions.md §26`/`§27` already found untrustworthy.
/// The provable signal lives in the INCLUDING book's own `.pcc`:
///
/// - `core_rulebook.pcc` and `advanced_race_guide.pcc`'s own `# Core
///   Races`/`# B1 races`/`# B2 races`/`# B3 races`/`# B4 races`/`#ISWG
///   races` comment sections (re-derived directly against the oracle,
///   2026-08-16: `grep -A40 'RACE:arg_races.lst' .../advanced_race_guide.pcc`
///   -- ARG reprints exactly 37 races across those six sections, matching
///   `arg_races.lst`'s own 37 `.MOD` lines, decisions.md §25.2's table
///   verified unchanged).
/// - `bestiary_4/_bestiary_4_for_players.pcc`'s own uncommented `# races`
///   section additionally declares `kasatha`, `trox`, `wyrwood`, `wyvaran`
///   -- Bestiary 4 natives ARG does not reprint.
/// - `bestiary_5/_bestiary_5_for_players.pcc` and
///   `bestiary_6/_bestiary_6 _for_players.pcc` each natively declare a
///   handful of their own races too, but most of those (`android`,
///   `ghoran`, `monkey_goblin`) are ALSO natively declared by
///   `inner_sea_bestiary`'s own `.pcc` -- two equally-real in-scope
///   candidates, which is not a resolved attribution. Only the members of
///   each book's own native set that no OTHER in-scope book's own `.pcc`
///   also natively declares are listed here: `skinwalker` (Bestiary 5) and
///   `rougarou` (Bestiary 6).
///
/// Left OUT on purpose, still `core_essentials` after this fix, because a
/// second candidate in-scope book exists or none does (verified by the same
/// `.pcc` grep, not guessed): `android`, `aquatic_elf`, `ghoran`, `lashunta`,
/// `monkey_goblin`, `syrinx`, `triaxian`, and (SD31-W5-INTEGRATE-001,
/// corrected per this wave's own adversarial review) `gathlain` --
/// declared by an identical uncommented `PCC:@...core_essentials\races\
/// gathlain\_race.pcc` line in BOTH `bestiary_4/_bestiary_4_for_players.pcc`
/// AND `ultimate_wilderness/_ultimate_wilderness.pcc`, and
/// `ultimate_wilderness` is itself an in-scope book (present in
/// `SOURCELONG_TO_BOOK`) -- by this doc comment's OWN "no other in-scope
/// book also natively declares it" test, gathlain does not qualify for
/// single-book attribution and must join the ambiguous set rather than
/// being asserted into `bestiary_4` alone. See
/// `core_essentials_ambiguous_races_stay_unattributed` for the fixture that
/// pins this list so a future book onboarding cannot silently narrow it.
const RACE_TRUE_BOOK: &[(&str, &str)] = &[
    // Core Rulebook -- `core_rulebook.pcc`'s own 7.
    ("dwarf", "core_rulebook"),
    ("elf", "core_rulebook"),
    ("gnome", "core_rulebook"),
    ("half_elf", "core_rulebook"),
    ("half_orc", "core_rulebook"),
    ("halfling", "core_rulebook"),
    ("human", "core_rulebook"),
    // Bestiary 1 -- ARG's `# B1 races` section, 11.
    ("aasimar", "bestiary"),
    ("drow", "bestiary"),
    ("duergar", "bestiary"),
    ("goblin", "bestiary"),
    ("hobgoblin", "bestiary"),
    ("kobold", "bestiary"),
    ("merfolk", "bestiary"),
    ("orc", "bestiary"),
    ("svirfneblin", "bestiary"),
    ("tengu", "bestiary"),
    ("tiefling", "bestiary"),
    // Bestiary 2 -- ARG's `# B2 races` section, 7.
    ("dhampir", "bestiary_2"),
    ("fetchling", "bestiary_2"),
    ("grippli", "bestiary_2"),
    ("ifrit", "bestiary_2"),
    ("oread", "bestiary_2"),
    ("sylph", "bestiary_2"),
    ("undine", "bestiary_2"),
    // Bestiary 3 -- ARG's `# B3 races` section, 5.
    ("catfolk", "bestiary_3"),
    ("ratfolk", "bestiary_3"),
    ("suli", "bestiary_3"),
    ("vanara", "bestiary_3"),
    ("vishkanya", "bestiary_3"),
    // Bestiary 4 -- ARG's `# B4 races` section (5) plus its own 5 more that
    // ARG does not reprint (`_bestiary_4_for_players.pcc`'s own uncommented
    // races section, 10 total).
    ("changeling", "bestiary_4"),
    ("kitsune", "bestiary_4"),
    ("nagaji", "bestiary_4"),
    ("samsaran", "bestiary_4"),
    ("wayang", "bestiary_4"),
    ("kasatha", "bestiary_4"),
    ("trox", "bestiary_4"),
    ("wyrwood", "bestiary_4"),
    ("wyvaran", "bestiary_4"),
    // Inner Sea World Guide -- ARG's `#ISWG races` section, 2.
    ("gillman", "inner_sea_world_guide"),
    ("strix", "inner_sea_world_guide"),
    // Bestiary 5 / Bestiary 6's own uniquely-native races (see doc comment).
    ("skinwalker", "bestiary_5"),
    ("rougarou", "bestiary_6"),
];

/// `SOURCELONG:<value>` (PCGen's own file-header source citation) -> the
/// corpus book directory whose own top-level files independently declare
/// the identical string. Cross-checked against every in-scope book's own
/// files (2026-08-16, `find_sourcelong` sweep over the pinned oracle), not
/// assumed from a book's name -- `ce_races_familiar_cr.lst` carries
/// `SOURCELONG:Bestiary` despite its own `_cr` filename suffix, which is
/// exactly why the filename is never trusted over the header.
const SOURCELONG_TO_BOOK: &[(&str, &str)] = &[
    ("Core Rulebook", "core_rulebook"),
    ("Bestiary", "bestiary"),
    ("Bestiary 2", "bestiary_2"),
    ("Bestiary 3", "bestiary_3"),
    ("Bestiary 4", "bestiary_4"),
    ("Bestiary 5", "bestiary_5"),
    ("Bestiary 6", "bestiary_6"),
    ("Advanced Player's Guide", "advanced_players_guide"),
    ("Advanced Race Guide", "advanced_race_guide"),
    ("Advanced Class Guide", "advanced_class_guide"),
    ("Ultimate Magic", "ultimate_magic"),
    ("Ultimate Combat", "ultimate_combat"),
    ("Ultimate Equipment", "ultimate_equipment"),
    ("Ultimate Intrigue", "ultimate_intrigue"),
    ("Ultimate Wilderness", "ultimate_wilderness"),
    ("Pathfinder Unchained", "pathfinder_unchained"),
    ("Monster Codex", "monster_codex"),
    ("Bonus Bestiary", "bonus_bestiary"),
    ("Inner Sea Races", "inner_sea_races"),
    ("Horror Adventures", "horror_adventures"),
    ("Inner Sea World Guide", "inner_sea_world_guide"),
    ("Inner Sea Combat", "inner_sea_combat"),
    ("Occult Adventures", "occult_adventures"),
    ("Inner Sea Bestiary", "inner_sea_bestiary"),
    ("Inner Sea Gods", "inner_sea_gods"),
];

/// The true book for a unit `enumerate_file` found inside `core_essentials/`,
/// or `None` when neither signal below resolves -- the honest "not yet
/// attributable" state, never a silent default back to `core_essentials`
/// (`decisions.md §25.2`/§25.3`, `race_resolver.rs`'s module doc,
/// `OPEN-ISSUES.md` row 68).
///
/// Two independent, per-record-provable signals, checked in order:
///
/// 1. **Per-race files** (`core_essentials/races/<slug>/...`): `slug` is the
///    directory component right after `races` in `path`, looked up in
///    [`RACE_TRUE_BOOK`].
/// 2. **Root-level shared files** (`core_essentials/ce_*.lst`): the file's
///    own header `SOURCELONG:` token (checked over the first 5 lines, where
///    every file that carries one in this corpus places it), looked up in
///    [`SOURCELONG_TO_BOOK`].
///
/// Callers only invoke this for `book == "core_essentials"`; every other
/// book's units are unaffected.
fn resolve_true_book_for_core_essentials(path: &Path, text: &str) -> Option<&'static str> {
    let components: Vec<String> =
        path.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
    let race_slug_book = components
        .iter()
        .position(|c| c == "races")
        .and_then(|races_at| components.get(races_at + 1))
        .and_then(|slug| RACE_TRUE_BOOK.iter().find(|(s, _)| s == slug))
        .map(|(_, book)| *book);
    if let Some(book) = race_slug_book {
        return Some(book);
    }
    text.lines().take(5).find_map(|line| {
        let value = line.split("SOURCELONG:").nth(1)?.split('\t').next().unwrap_or("").trim();
        SOURCELONG_TO_BOOK.iter().find(|(s, _)| *s == value).map(|(_, book)| *book)
    })
}

/// Enumerate one `.lst` file into `out`, recording every trap hit.
fn enumerate_file(path: &Path, book: &str, kind: Kind, text: &str, out: &mut BookEnumeration) {
    // `'static`, deliberately: only ever `Some` from `RACE_TRUE_BOOK` /
    // `SOURCELONG_TO_BOOK`, both `&'static str` tables, never derived from
    // `book`'s own shorter-lived reference -- so it can be stashed in
    // `mod_targets` (which outlives this call) without a lifetime escape.
    let resolved_book: Option<&'static str> =
        if book == "core_essentials" { resolve_true_book_for_core_essentials(path, text) } else { None };
    let effective_book: &str = resolved_book.unwrap_or(book);
    let rel = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();

    for (index, line) in text.lines().enumerate() {
        let line_number = index + 1;
        let fields = tab_fields(line);
        let Some(first) = fields.first() else { continue };
        let first = first.trim();
        if first.is_empty() {
            continue;
        }
        if first.starts_with('#') {
            *out.trap_hits.entry("comment_or_disabled").or_default() += 1;
            continue;
        }
        // An ALL-CAPS `TOKEN:` first field is file metadata, not a record --
        // except `CLASS:`, which is exactly how a class record declares itself.
        let is_directive = first
            .split_once(':')
            .map(|(head, _)| {
                !head.is_empty()
                    && head
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            })
            .unwrap_or(false);
        if is_directive && !first.starts_with("CLASS:") {
            *out.trap_hits.entry("directive_line").or_default() += 1;
            continue;
        }

        // SD28-E15 (2026-08-09): widened from `first.starts_with("CATEGORY=Internal|")`
        // alone, which only matched the directive-line shape. A normal
        // record row also carries `CATEGORY:Internal` as an ordinary field
        // (e.g. `Blue Psion FC handler`, `up_abilities_race.lst:382`:
        // `CATEGORY:Internal TYPE:SaveBonus VISIBLE:NO`), and that shape
        // slipped through this trap entirely -- confirmed by the same
        // `Internal`-category bookkeeping pattern `§51`'s archetype-swap
        // tables already excluded (`Armor Aptitude 7th Level`,
        // `Thoughtsinger ~ Wild Talent`), found independently a second
        // time via a different check.
        let is_internal_category = first.starts_with("CATEGORY=Internal|")
            || fields.iter().any(|f| f.trim() == "CATEGORY:Internal");
        if is_internal_category {
            *out.trap_hits.entry("internal_namespace").or_default() += 1;
            continue;
        }
        let visible = !fields.iter().any(|f| f.trim() == "VISIBLE:NO");
        if !visible {
            *out.trap_hits.entry("invisible_record").or_default() += 1;
        }

        // `.MOD` is resolved corpus-wide after enumeration; stash it.
        if let Some(mod_at) = first.find(".MOD") {
            let base = mod_base_name(&first[..mod_at]);
            *out.trap_hits.entry("mod_record").or_default() += 1;
            let magnitudes = MAGNITUDE_TOKENS
                .iter()
                .filter(|t| has_token(&fields, t))
                .count();
            out.mod_targets.push((
                refine_kind(kind, &fields),
                base.clone(),
                base,
                Provenance { file: rel.clone(), line: line_number },
                magnitudes,
                resolved_book,
            ));
            continue;
        }

        // A class file's non-`CLASS:` rows are per-level progression rows.
        if kind == Kind::Class && !first.starts_with("CLASS:") {
            *out.trap_hits.entry("class_level_line").or_default() += 1;
            continue;
        }

        // `_abilities_race.lst` is classified whole-file as `Kind::RaceTrait`
        // by `file_kind`, but it carries at least two other row shapes that
        // are real content in the wrong bucket, not racial traits:
        //
        // - A `TYPE:` field with a `FavoredClassBonus` dot-component is a
        //   Favored Class Bonus row (one per race x class), e.g.
        //   `TYPE:SpecialQuality.FavoredClassBonus.FavoredClassSorcerer`
        //   (`arg_abilities_race.lst`). `race_trait_ids` is keyed on
        //   `<race>.<trait-slug>` pairs and can never hold an FCB identity,
        //   so counting these as `race_trait` units reports them
        //   `not-ingested` forever regardless of ingestion effort.
        // - A `CATEGORY:Choice` row is a `CHOOSE:` sub-option belonging to an
        //   already-counted parent trait, e.g. `Elf ~ Elemental Resistance`
        //   (CATEGORY:Special Ability, already a unit) offers 4
        //   `CATEGORY:Choice` rows (Acid/Cold/Electricity/Fire) as its menu.
        //   Every one of the 156 already-ingested ARG alternate racial
        //   traits (`data/corpus/advanced_race_guide/race_trait/*/*.json`)
        //   carries `CATEGORY:Special Ability`, never `CATEGORY:Choice` --
        //   re-derived 2026-08-07 with a full scan of that corpus tree.
        //   Counting the sub-option as a second, independent unit
        //   double-counts the parent trait's own content.
        //
        // Neither is a unit for this inventory's purposes; both are recorded
        // as trap hits (never silently dropped) rather than reclassified
        // into a new `Kind`, since neither is itself the kind of standalone
        // content this inventory tracks elsewhere (`decisions.md §35`).
        if kind == Kind::RaceTrait {
            let is_fcb = fields
                .iter()
                .filter_map(|f| f.trim_start().strip_prefix("TYPE:"))
                .any(|value| value.split('.').any(|c| c == "FavoredClassBonus"));
            if is_fcb {
                *out.trap_hits.entry("race_favored_class_bonus_row").or_default() += 1;
                continue;
            }
            let is_choice_suboption = fields.iter().any(|f| f.trim() == "CATEGORY:Choice");
            if is_choice_suboption {
                *out.trap_hits.entry("race_choice_suboption_row").or_default() += 1;
                continue;
            }
        }

        let (display_name, origin) = if let Some((_, variant)) = first.split_once(".COPY=") {
            *out.trap_hits.entry("copy_record").or_default() += 1;
            (variant.to_string(), Origin::Copy)
        } else if let Some(rest) = first.strip_prefix("CLASS:") {
            (rest.to_string(), Origin::Declared)
        } else if let Some(rest) = first
            .strip_prefix("CATEGORY=")
            .and_then(|r| r.split_once('|'))
            .map(|(_, rest)| rest)
        {
            // A `CATEGORY=Special Ability|Foo` first field names `Foo`. Gated
            // on the `CATEGORY=` prefix rather than splitting on any `|`, so a
            // record whose display name legitimately contains a pipe is not
            // silently truncated.
            (rest.to_string(), Origin::Declared)
        } else {
            (first.to_string(), Origin::Declared)
        };

        if is_excluded_race_trait_row(kind, &fields) {
            *out.trap_hits.entry("race_trait_class_level_adjustment_row").or_default() += 1;
            continue;
        }

        let record_kind = refine_kind(kind, &fields);
        if !has_classifying_token(record_kind, &fields) {
            *out.trap_hits.entry("missing_classifying_token").or_default() += 1;
            continue;
        }

        let key = token_value(&fields, "KEY:")
            .map(|k| k.to_string())
            .unwrap_or_else(|| display_name.clone());

        out.units.push(CorpusUnit {
            book: effective_book.to_string(),
            source_book: book.to_string(),
            kind: record_kind,
            key,
            name: display_name,
            origin,
            provenance: Provenance { file: rel.clone(), line: line_number },
            magnitude_token_count: MAGNITUDE_TOKENS
                .iter()
                .filter(|t| has_token(&fields, t))
                .count(),
            type_facet: token_value(&fields, "TYPE:").map(|t| t.to_string()),
            visible,
        });
    }
    out.files_enumerated += 1;
}

/// Walk one book directory (recursively — `core_essentials` nests its per-race
/// files two levels deep) and enumerate every `.lst` it holds.
fn enumerate_book(book_dir: &Path, book: &str) -> BookEnumeration {
    let mut out = BookEnumeration::default();
    let mut stack = vec![book_dir.to_path_buf()];
    let mut files: Vec<PathBuf> = Vec::new();
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("lst") {
                files.push(path);
            }
        }
    }
    // Sorted so two runs over the same corpus enumerate in the same order and
    // the output is byte-identical (the idempotence contract).
    files.sort();

    for path in files {
        let basename = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        match file_kind(&basename) {
            Some(kind) => {
                if let Ok(text) = std::fs::read_to_string(&path) {
                    enumerate_file(&path, book, kind, &text, &mut out);
                } else {
                    out.files_not_enumerated.insert(basename);
                }
            }
            None => {
                out.files_not_enumerated.insert(basename);
            }
        }
    }
    out
}

// `build_mod_index`, `CorpusLines`, and `token_closure_rows` -- the shared
// GE-01 token-closure machinery -- live in `codex::rules_core::wiring_class`
// so `cache_gen::*`'s per-book generators can build the same closure a
// `.MOD` row belongs to without a second implementation. Only
// `mod_base_name` stays local: it is also the `mod_only_rescue` path's own
// base-name resolver, imported by name below.
use codex::rules_core::wiring_class::{
    CorpusLines, build_mod_index, mod_base_name, token_closure_rows,
};

// ---------------------------------------------------------------------------
// Book roster
// ---------------------------------------------------------------------------

/// One corpus book, with the scope the repository itself assigns it.
struct BookMeta {
    id: String,
    /// `in_scope` (a compiled rule set exists), `shared_library` (included by
    /// other books' `.pcc` rather than standing alone), `future_state` (a
    /// registered stub at `data/stubs/<id>.json`), `out_of_scope` (operator
    /// directive 2026-07-27), or `unregistered` (present in the corpus with
    /// no repo registration at all — reported, never hidden).
    scope: &'static str,
    rule_set: Option<RuleSetId>,
    /// Other book directories this book's `.pcc` files pull in, read from the
    /// real `PCC:` directives rather than assumed.
    pcc_includes: BTreeSet<String>,
}

/// Every rule set the engine compiles.
///
/// This list and `corpus_dir_for` together replace what used to be a
/// `match book_dir { ... _ => None }`. That wildcard was silent breakage: when
/// SD-27 added `RuleSetId::Arg` and `RuleSetId::Pu`, the *exhaustive*
/// `rule_set_id` below was forced by the compiler to grow two arms, but the
/// wildcard here absorbed both books without a word. The result was that
/// 2,269 ARG and 882 PU corpus units reported `no_compiled_rule_set_for_book`
/// -> `not-started` -> `scope: future_state` -> 0% proven, while the engine
/// was in fact shipping their feat tables, class chassis and reach gating.
/// Eleven days of delivered work read as untouched on the dashboard.
///
/// Adding a variant to `RuleSetId` now breaks `corpus_dir_for`'s match until
/// the corpus directory is declared, so the next book cannot go unmeasured
/// the same way.
const COMPILED_RULE_SETS: &[RuleSetId] = &[
    RuleSetId::Crb,
    RuleSetId::Apg,
    RuleSetId::Acg,
    RuleSetId::Bestiary1,
    RuleSetId::Arg,
    RuleSetId::Pu,
    RuleSetId::Uca,
    RuleSetId::Ui,
    RuleSetId::Ue,
    RuleSetId::Uw,
    RuleSetId::Uc,
    RuleSetId::Um,
    RuleSetId::Upsi,
    RuleSetId::BonusBestiary,
    RuleSetId::MonsterCodex,
    RuleSetId::Isr,
    RuleSetId::Ha,
    RuleSetId::Botd1,
    RuleSetId::Botd2,
    RuleSetId::Iswg,
    RuleSetId::Ce,
    RuleSetId::Isc,
    RuleSetId::Isi,
    RuleSetId::B5,
    RuleSetId::B6,
    RuleSetId::B2,
    // SD-29 Epic 5 extend, round 5 (monster lane).
    RuleSetId::B3,
    // SD-29 Epic 5 extend, round 6 (monster lane).
    RuleSetId::B4,
    // SD-29 Epic 5 extend, round 7 (monster lane). The engine id and the corpus
    // directory are spelled the same, unlike `bestiary` -> `bestiary_1`.
    RuleSetId::Isb,
    // SD-29 Epic 5 extend, round 9 (monster lane). Engine id and corpus
    // directory are spelled the same.
    RuleSetId::Isg,
    // SD31-E6-F2-003 -- this book's first compiled rule set of any kind.
    RuleSetId::Oa,
];

/// The corpus directory whose records a rule set is compiled from. Exhaustive
/// on purpose — see `COMPILED_RULE_SETS`.
fn corpus_dir_for(rule_set: RuleSetId) -> &'static str {
    match rule_set {
        RuleSetId::Crb => "core_rulebook",
        RuleSetId::Apg => "advanced_players_guide",
        RuleSetId::Acg => "advanced_class_guide",
        // The only id that is not spelled like its directory: the engine calls
        // it `bestiary_1`, the corpus directory is `bestiary`.
        RuleSetId::Bestiary1 => "bestiary",
        RuleSetId::Arg => "advanced_race_guide",
        RuleSetId::Pu => "pathfinder_unchained",
        RuleSetId::Uca => "ultimate_campaign",
        RuleSetId::Ui => "ultimate_intrigue",
        RuleSetId::Ue => "ultimate_equipment",
        RuleSetId::Uw => "ultimate_wilderness",
        RuleSetId::Uc => "ultimate_combat",
        RuleSetId::Um => "ultimate_magic",
        RuleSetId::Upsi => "ultimate_psionics",
        RuleSetId::BonusBestiary => "bonus_bestiary",
        RuleSetId::MonsterCodex => "monster_codex",
        RuleSetId::Isr => "inner_sea_races",
        RuleSetId::Ha => "horror_adventures",
        RuleSetId::Botd1 => "book_of_the_damned_volume_1",
        RuleSetId::Botd2 => "book_of_the_damned_volume_2",
        RuleSetId::Iswg => "inner_sea_world_guide",
        RuleSetId::Ce => "core_essentials",
        RuleSetId::Isc => "inner_sea_combat",
        RuleSetId::Isi => "inner_sea_intrigue",
        RuleSetId::B5 => "bestiary_5",
        RuleSetId::B6 => "bestiary_6",
        RuleSetId::B2 => "bestiary_2",
        RuleSetId::B3 => "bestiary_3",
        RuleSetId::B4 => "bestiary_4",
        RuleSetId::Isb => "inner_sea_bestiary",
        RuleSetId::Isg => "inner_sea_gods",
        RuleSetId::Oa => "occult_adventures",
    }
}

/// The engine's rule set for a corpus directory, or `None` when the engine has
/// not compiled that book at all — which is what makes the rest of the corpus
/// land at `not-started` honestly rather than by omission.
fn rule_set_for(book_dir: &str) -> Option<RuleSetId> {
    COMPILED_RULE_SETS
        .iter()
        .copied()
        .find(|&rs| corpus_dir_for(rs) == book_dir)
}

fn rule_set_id(rule_set: RuleSetId) -> &'static str {
    match rule_set {
        RuleSetId::Crb => "core_rulebook",
        RuleSetId::Apg => "advanced_players_guide",
        RuleSetId::Acg => "advanced_class_guide",
        RuleSetId::Bestiary1 => "bestiary_1",
        // SD-27. Unlike `bestiary` -> `bestiary_1`, these two engine ids are
        // spelled exactly like their corpus directories, so `engine_book_for`
        // joins them without a rename. They are listed here because
        // `all_feat_tables()` now yields their tables — without these arms the
        // whole root-crate bin set, this generator included, fails to compile.
        RuleSetId::Arg => "advanced_race_guide",
        RuleSetId::Pu => "pathfinder_unchained",
        RuleSetId::Uca => "ultimate_campaign",
        RuleSetId::Ui => "ultimate_intrigue",
        RuleSetId::Ue => "ultimate_equipment",
        RuleSetId::Uw => "ultimate_wilderness",
        RuleSetId::Uc => "ultimate_combat",
        RuleSetId::Um => "ultimate_magic",
        RuleSetId::Upsi => "ultimate_psionics",
        RuleSetId::BonusBestiary => "bonus_bestiary",
        RuleSetId::MonsterCodex => "monster_codex",
        RuleSetId::Isr => "inner_sea_races",
        RuleSetId::Ha => "horror_adventures",
        RuleSetId::Botd1 => "book_of_the_damned_volume_1",
        RuleSetId::Botd2 => "book_of_the_damned_volume_2",
        RuleSetId::Iswg => "inner_sea_world_guide",
        RuleSetId::Ce => "core_essentials",
        RuleSetId::Isc => "inner_sea_combat",
        RuleSetId::Isi => "inner_sea_intrigue",
        // Unlike `bestiary` -> `bestiary_1`, these three engine ids are spelled
        // exactly like their corpus directories.
        RuleSetId::B5 => "bestiary_5",
        RuleSetId::B6 => "bestiary_6",
        RuleSetId::B2 => "bestiary_2",
        RuleSetId::B3 => "bestiary_3",
        RuleSetId::B4 => "bestiary_4",
        RuleSetId::Isb => "inner_sea_bestiary",
        RuleSetId::Isg => "inner_sea_gods",
        RuleSetId::Oa => "occult_adventures",
    }
}

/// Translates one `equipment_resolver::EQUIPMENT_BOOK_*` short code to the
/// same book-directory slug `equipment_keys`'s map is keyed by elsewhere in
/// this file (`rule_set_id`'s own output). Panics on an unrecognized code
/// rather than silently dropping the book from the equipment classifier --
/// the failure mode this whole fix exists to replace. See
/// `equipment_book_slug_for_covers_every_catalog_book` for the guard that
/// exercises this against the resolver's own live output.
fn equipment_book_slug_for(short_code: &str) -> &'static str {
    match short_code {
        "CRB" => "core_rulebook",
        "APG" => "advanced_players_guide",
        "ACG" => "advanced_class_guide",
        "B1" => "bestiary_1",
        "ARG" => "advanced_race_guide",
        "PU" => "pathfinder_unchained",
        "UI" => "ultimate_intrigue",
        "UE" => "ultimate_equipment",
        "UM" => "ultimate_magic",
        "UPSI" => "ultimate_psionics",
        "UC" => "ultimate_combat",
        // SD-29 `epic-4-proven-equip-mod`: UW has no hand-authored equipment
        // table; all 127 of its catalog rows come from
        // `rules_tables::equipment_gap_tables`.
        "UW" => "ultimate_wilderness",
        other => panic!(
            "equipment_resolver::equipment_catalog_rows() now carries an unmapped book code \
             {other:?} -- add it to equipment_book_slug_for so the equipment classifier does \
             not silently drop the book (this is exactly the SD-28-E15 defect this function \
             replaces)"
        ),
    }
}

/// Translates one `spell_resolver::SPELL_BOOK_*` short code to the same
/// book-directory slug `spell_levels`'s map is keyed by elsewhere in this
/// file (`rule_set_id`'s own output). Panics on an unrecognized code rather
/// than silently dropping the book from the spell classifier -- the failure
/// mode this fix exists to replace, and the same guard shape
/// `equipment_book_slug_for` above already carries. See
/// `spell_book_slug_for_covers_every_catalog_book`.
fn spell_book_slug_for(short_code: &str) -> &'static str {
    match short_code {
        "CRB" => "core_rulebook",
        "APG" => "advanced_players_guide",
        "ACG" => "advanced_class_guide",
        "ARG" => "advanced_race_guide",
        "UI" => "ultimate_intrigue",
        "UM" => "ultimate_magic",
        "OA" => "occult_adventures",
        // SD31-E6-F2-004: UC joins `spell_resolver::spell_catalog_rows()` as
        // the catalog's 8th book. Same additive, single-line registration
        // the UM (SD31-E6-F2-002) and OA (SD31-E6-F2-003) spell-lane cycles
        // both made here before it -- this function is a closed-set lookup
        // table with its own dedicated test
        // (`spell_book_slug_for_covers_every_catalog_book`), not
        // attribution or measurement logic.
        "UC" => "ultimate_combat",
        other => panic!(
            "spell_resolver::spell_catalog_rows() now carries an unmapped book code {other:?} \
             -- add it to spell_book_slug_for so the spell classifier does not silently drop \
             the book (this is exactly the divergence this function replaces)"
        ),
    }
}

/// Which other book directories a book's `.pcc` files include. Read from the
/// real `PCC:` lines so the `core_essentials` relationship is *derived* — that
/// is what proves the seven CRB races, whose bases live in
/// `core_essentials/races/<race>/`, genuinely belong to the Core Rulebook.
fn pcc_includes(book_dir: &Path, known_books: &BTreeSet<String>) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let Ok(entries) = std::fs::read_dir(book_dir) else { return out };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("pcc") {
            continue;
        }
        let self_id = book_dir
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        for line in text.lines() {
            let Some(rest) = line.trim().strip_prefix("PCC:") else { continue };
            let normalised = rest.replace('\\', "/");
            for book in known_books {
                // A book's own `.pcc` naming its own subdirectory is not an
                // include of another book.
                if *book != self_id && normalised.contains(&format!("roleplaying_game/{book}/")) {
                    out.insert(book.clone());
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Engine facts
// ---------------------------------------------------------------------------

/// Everything the engine can prove, gathered once.
///
/// `Default` is derived so a test can state the ONE fact it is exercising and
/// leave every other field empty. Every field is a collection whose empty
/// value means "the engine proved nothing here", which is the correct and
/// conservative starting point: a defaulted `EngineFacts` grounds nothing.
#[derive(Default)]
struct EngineFacts {
    /// Feat keys whose presence genuinely changes a computed number, observed
    /// by running the real compute pipeline twice per feat.
    feat_effect_wired: BTreeSet<String>,
    /// SD28-E14-F2: equipment/equipment-modifier keys whose real on-disk
    /// corpus record was observed reaching
    /// `equipment_effects::compute_equipment_effects` and producing a real,
    /// non-`None` per-item stat effect. Populated by
    /// [`probe_equipment_effect_wiring`]. A lower bound in the same
    /// documented direction as `feat_effect_wired`: gated on a real
    /// on-disk JSON record existing under `data/corpus/<book>/equipment/`.
    ///
    /// `(engine_book, key)`, because the observation is book-scoped: the
    /// probe resolves each book's catalog keys against that book's own corpus
    /// alone, so a key shared by two books' reprints of *different* items
    /// grounds only the book whose record was actually read.
    equipment_effect_wired: BTreeSet<(String, String)>,
    /// SD-32 `ground-spell-units`: spell keys whose own corpus record was
    /// observed producing a real save-DC magnitude on
    /// `PilotSpellbookViewModel.spell_save_dc` — the field
    /// `pf1_adapter::resolve_unified_pilot_snapshot` puts on the snapshot and
    /// `CharacterSheet.tsx` renders as `DC {entry.dc}`. Populated by
    /// [`probe_spell_effect_wiring`] through
    /// [`spell_effect_wired_from_outcomes`], which admits only
    /// [`SpellProbeOutcome::Wired`].
    ///
    /// `(engine_book, key)` for the same book-scoping reason as
    /// `equipment_effect_wired`, and here it bites: every per-school resolver
    /// stamps `RuleSetId::Crb`, so a non-CRB book's same-named record is
    /// refused at the probe (`SpellProbeOutcome::ForeignBookTable`) and must
    /// not be re-admitted by a bare-key lookup here.
    ///
    /// A lower bound in the same documented direction as the other two probes:
    /// it can only reach a book with an on-disk `data/corpus/<book>/spell/`
    /// tree in [`OBSERVABLE_BOOK_DIRS`].
    spell_effect_wired: BTreeSet<(String, String)>,
    /// Every feat key the catalog holds, per book.
    feat_keys: BTreeMap<&'static str, BTreeSet<String>>,
    /// Every spell key the catalog holds, per book, with whether the engine
    /// resolved a numeric level for it.
    spell_levels: BTreeMap<&'static str, BTreeMap<String, bool>>,
    /// Every equipment key the catalog holds, per book.
    equipment_keys: BTreeMap<&'static str, BTreeSet<String>>,
    /// Every Bestiary 1 monster that resolves to a real stat block, by name.
    monster_names: BTreeSet<String>,
    /// Every chassis-book monster the engine holds, keyed by corpus book then
    /// by lowercase corpus key.
    ///
    /// Kept per book rather than merged into one set: the books' tables are
    /// independent, and a merged set would credit one book's stat block to
    /// another on a name collision -- the same book-gating `holds_key` already
    /// applies to races and race traits. Built by iterating
    /// `monster_chassis::MONSTER_BOOKS`, so registering a book there is what
    /// makes its units classify; nothing here names a book.
    chassis_monster_keys: BTreeMap<&'static str, BTreeSet<String>>,
    /// Every chassis-book `monster_ability` the engine holds, same shape. The
    /// key, never the display name: namespaced keys (`Seru ~ Poison`,
    /// `Caryatid Column ~ Immunity to Magic`) have leaves that are not unique.
    chassis_monster_ability_keys: BTreeMap<&'static str, BTreeSet<String>>,
    /// CONFIRMED finding subset of `chassis_monster_ability_keys`: the keys
    /// whose row declares a CHARACTER-SPECIFIC computed `DESC:` argument
    /// (`description_variables` non-empty, e.g. `13+Con`/`CONSCORE`/
    /// `BreathWeaponDC`) OR whose `description` text leaks a literal
    /// unresolved `%<digit>` even with no declared argument list. Both
    /// shapes render on the real player-facing screen with the number
    /// silently deleted (`serve_ability_description` renders with an EMPTY
    /// `PcgenDisplayValues`, and `render_pcgen_desc` drops any `%N` it
    /// cannot resolve) -- see the `monster_ability` rung's own call site.
    chassis_monster_ability_unresolved_desc_keys: BTreeMap<&'static str, BTreeSet<String>>,
    /// Every chassis-book `companion` record the engine holds, keyed by corpus
    /// book then by lowercase corpus key.
    ///
    /// One map for both of the kind's structural shapes (creature rows and
    /// ability rows), because `Kind::Companion` is one kind: `file_kind` types
    /// `*_races_companion.lst` and `*_abilities_companion.lst` alike, and a unit
    /// carries no field that distinguishes them. Built by iterating
    /// `companion_chassis::COMPANION_BOOKS`, so registering a book there is what
    /// makes its units classify; nothing here names a book.
    chassis_companion_keys: BTreeMap<&'static str, BTreeSet<String>>,
    /// Every class the engine models, by lowercase name, with its book.
    class_books: BTreeMap<String, &'static str>,
    /// Every modelled class the class consumer-delta probe OBSERVED producing
    /// a magnitude attributable to it alone on the snapshot the character
    /// sheet renders. A subset of `class_books`' keys, never a superset: see
    /// [`probe_class_name`].
    class_effect_wired: BTreeSet<String>,
    /// Every option-pool `class_feature` corpus key the class_feature
    /// consumer-delta probe OBSERVED moving a rendered fact attributably to
    /// itself, mapped to the book that models the pool's owning class.
    ///
    /// The book is carried, not just the key, for the reason
    /// `probe_equipment_effect_wiring`'s `Celestial Shield` discipline records
    /// and `SpellProbeOutcome::ForeignBookTable` enforces: a shared NAME is
    /// not a shared record. A key is only ever grounded for the book whose
    /// class the engine actually models.
    class_feature_effect_wired: BTreeMap<String, &'static str>,
    /// Every race the engine models, by lowercase name.
    race_names: BTreeSet<String>,
    /// Race trait identities the engine grounds, as `<race>.<trait slug>`.
    ///
    /// CRB's seven compiled races only. Kept as the FALLBACK rule beneath
    /// `reachable_race_traits`, never as the primary one — see
    /// [`probe_reachable_race_traits`] for why a probe pinned to this table
    /// under-reports the product by eleven races.
    race_trait_ids: BTreeSet<String>,
    /// Every race trait the app's loaded race corpus can APPLY to a player,
    /// by `(<lst basename>, <line>)` -> corpus book, and every record that
    /// load found at all. See [`probe_race_trait_corpus`].
    race_trait_probe: RaceTraitProbe,
    /// Explanation ids observed in a real receipt across the class sweep.
    explanation_ids: BTreeSet<String>,
    /// Diagnostics observed in the same sweep: id -> (message, claim_blocking).
    ///
    /// Both kinds are collected. A `claim_blocking: false` diagnostic is still
    /// the engine naming a real gap in its own words -- 37 of the 100
    /// diagnostics this engine constructs are non-blocking -- and quoting only
    /// the blocking ones would have made `deferred-with-reason` an empty
    /// status on a branch where all 27 classes reach `Computed`.
    diagnostics: BTreeMap<String, (String, bool)>,
    /// Every class name the CORPUS declares anywhere, lowercased. Used to tell
    /// "this feature belongs to a class the engine has not modelled yet" from
    /// "this feature belongs to no class at all".
    corpus_class_names: BTreeSet<String>,
    /// SD31-D7-PROSE-002 (Decision 7's condition 3, `OPEN-ISSUES.md` row 70):
    /// a second source for "this unit's corpus record carries a real
    /// description", alongside [`closure_has_real_description`]'s raw `.lst`
    /// closure. Keyed `(<lst basename>, <line>, <record key>)` -> the
    /// already-ingested `data/corpus/<book>/**/*.json`'s own `data.description`
    /// value. A `.COPY=` record's description is often resolved by
    /// INHERITANCE at ingest time -- a fact the raw `.lst` text alone can
    /// never carry, because the inheritance resolution never touches the
    /// `.lst` file. The record key joins alongside the coordinate because
    /// `(basename, line)` alone is ambiguous for 24 real corpus coordinates
    /// where two distinct records share one `.lst` line (e.g.
    /// `acg_equipmods.lst:41` is both `Flying` and `Special Ability ~ Flying
    /// ~ Melee`). Populated by [`load_corpus_json_descriptions`].
    corpus_json_descriptions: BTreeMap<(String, usize, String), String>,
}

impl EngineFacts {
    /// The engine book whose ingested race-trait corpus really holds this
    /// unit, joined on the record's own source coordinates.
    ///
    /// Race traits are the one kind whose `.lst` rows are routinely filed
    /// under a *different* book than the one that ingested them —
    /// `core_essentials/duergar_abilities_race.lst` is Bestiary 1's content
    /// living in the shared library — so name matching cannot attribute them
    /// and the source coordinate is the only identity that can.
    fn race_trait_engine_book(&self, unit: &CorpusUnit) -> Option<&'static str> {
        let coordinate = (unit.provenance.file.clone(), unit.provenance.line);
        engine_book_for_corpus_dir(self.race_trait_probe.reachable.get(&coordinate)?)
    }

    /// Whether this unit's record was found by the race-corpus load at all,
    /// applicable or not. `true` with [`Self::race_trait_engine_book`]
    /// returning `None` is the "ingested, loaded, inert" case.
    fn race_trait_was_loaded(&self, unit: &CorpusUnit) -> bool {
        self.race_trait_probe
            .loaded
            .contains(&(unit.provenance.file.clone(), unit.provenance.line))
    }

    /// This unit's description, rendered by the EXACT function
    /// `race_trait_picker::build_menu` calls to serve the real,
    /// player-facing Alternate Racial Traits screen
    /// (`list_alternate_racial_traits` Tauri command) — over the SAME
    /// `RaceCorpus` load this probe already performs. `None` when the record
    /// was not found by that load at all (see [`Self::race_trait_was_loaded`]
    /// for that case); `Some("")` is possible and is NOT a real description
    /// (the caller checks non-empty, same as everywhere else in this file).
    ///
    /// SD31-D7-PROSE-001 (Decision 7's condition 3): "the prose is available
    /// to print in the description on the character sheet" is not proven by
    /// a record merely being loaded -- it is proven by the SAME render path
    /// the shipped screen actually calls producing real text, which is
    /// exactly what this reuses rather than re-implementing.
    fn race_trait_rendered_description(&self, unit: &CorpusUnit) -> Option<&str> {
        let coordinate = (unit.provenance.file.clone(), unit.provenance.line);
        self.race_trait_probe.rendered.get(&coordinate).map(String::as_str)
    }

    /// Whether one book really holds this unit. Delegates to
    /// [`Self::holds_key`] for every kind whose identity is its name, and
    /// uses the source-coordinate join for race traits, which is the only
    /// kind for which the name is not enough.
    fn holds_unit(&self, book: &str, unit: &CorpusUnit) -> bool {
        if matches!(unit.kind, Kind::RaceTrait)
            && self.race_trait_engine_book(unit) == Some(book)
        {
            return true;
        }
        self.holds_key(book, &unit.kind, &unit.key, &unit.name)
    }

    /// Whether one book's own compiled table holds this unit's identity.
    /// Used to attribute a shared-library record to the book that really
    /// ingested it rather than to an arbitrary one of its hosts.
    fn holds_key(&self, book: &str, kind: &Kind, key: &str, name: &str) -> bool {
        let hit = |set: Option<&BTreeSet<String>>| {
            set.map(|s| s.contains(key) || s.contains(name)).unwrap_or(false)
        };
        // The chassis tables are indexed lowercase, because the corpus and the
        // inventory disagree on case for a handful of rows.
        let hit_lowercase = |set: Option<&BTreeSet<String>>| {
            set.map(|s| s.contains(&key.to_lowercase()) || s.contains(&name.to_lowercase()))
                .unwrap_or(false)
        };
        match kind {
            Kind::Feat => hit(self.feat_keys.get(book)),
            Kind::Equipment | Kind::EquipmentModifier => hit(self.equipment_keys.get(book)),
            Kind::Spell => self
                .spell_levels
                .get(book)
                .map(|t| t.contains_key(key) || t.contains_key(name))
                .unwrap_or(false),
            // Races, race traits and monsters each live in exactly one book's
            // module (`crb::race_tables`, `beastiary1`), so the book gate is
            // part of the fact -- without it a shared-library race would be
            // credited to whichever host happened to be tried first.
            // Bestiary 1 is served by TWO tables and both ground it: SD-22's
            // `beastiary1` (46 hand-modelled stat blocks, joined by display
            // name) and, since SD-29 Epic 5 round 8, the chassis holding the
            // book's other 284 rows (`decisions.md §58.3`). A UNION, not a
            // precedence: an early return on `monster_names` would report all
            // 284 chassis rows `not-ingested` while the registry held them, and
            // consulting only the chassis would demote the 46. Both halves have
            // to answer, because the book really is in both places.
            Kind::Monster => {
                (book == "bestiary_1" && self.monster_names.contains(&name.to_lowercase()))
                    || hit_lowercase(self.chassis_monster_keys.get(book))
            }
            Kind::MonsterAbility => hit_lowercase(self.chassis_monster_ability_keys.get(book)),
            Kind::Companion => hit_lowercase(self.chassis_companion_keys.get(book)),
            Kind::Race => book == "core_rulebook" && self.race_names.contains(&name.to_lowercase()),
            // The record's OWN race gates this, not "any modelled race" --
            // see `modelled_race_of_race_trait`.
            Kind::RaceTrait => {
                book == "core_rulebook"
                    && modelled_race_of_race_trait(key, &self.race_names).is_some_and(|race| {
                        self.race_trait_ids.contains(&format!("{race}.{}", slug(name)))
                    })
            }
            Kind::Class => self
                .class_books
                .get(&name.to_lowercase())
                .map(|b| *b == book)
                .unwrap_or(false),
            _ => false,
        }
    }

    /// Whether `book`/`key`/`name` names a `monster_ability` whose own row
    /// leaks an unresolved character-specific description argument to the
    /// player's screen -- see [`monster_ability_desc_leaks_unresolved_argument`].
    fn monster_ability_desc_leaks_unresolved_argument(&self, book: &str, key: &str, name: &str) -> bool {
        self.chassis_monster_ability_unresolved_desc_keys
            .get(book)
            .map(|s| s.contains(&key.to_lowercase()) || s.contains(&name.to_lowercase()))
            .unwrap_or(false)
    }
}

fn posture_input(fixture: &CharacterInput, class_name: &str, level: u8) -> CharacterInput {
    let mut input = fixture.clone();
    input.case_id = Some(format!("v06_work_inventory.{class_name}.level{level}"));
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: format!("class:{class_name}"),
        level,
    }];
    input
}

/// The class-conditional canonical seeds `compose_character_input`
/// (`apps/desktop/src-tauri/src/pf1_adapter.rs`) applies at creation time,
/// mirroring `v06_class_state_dump::canonical_seeds_for`. Without them the
/// sweep would see a bare skeleton rather than the posture the app actually
/// builds, and would under-report grounded class features for every class
/// whose chassis is gated behind a canonical default.
fn canonical_seeds_for(class_name: &str) -> (Vec<SelectedChoice>, Vec<SpellSelection>) {
    let choice = |set: &str, selection: &str| SelectedChoice {
        choice_set_id: set.to_owned(),
        selection_id: selection.to_owned(),
    };
    let spell = |class: &str, mode: AcquisitionMode| SpellSelection {
        spell_id: "Light".to_owned(),
        source_class_id: format!("class:{class}"),
        acquisition_mode: mode,
    };
    let extract = |class: &str, mode: AcquisitionMode| SpellSelection {
        spell_id: "Cure Light Wounds".to_owned(),
        source_class_id: format!("class:{class}"),
        acquisition_mode: mode,
    };

    match class_name {
        "wizard" => (
            vec![
                choice("choice:wizard_school_specialization", "school:evocation"),
                choice("choice:wizard_opposed_schools", "school:necromancy"),
                choice("choice:wizard_opposed_schools", "school:transmutation"),
            ],
            vec![
                spell("wizard", AcquisitionMode::Known),
                spell("wizard", AcquisitionMode::Prepared),
            ],
        ),
        "arcanist" => (
            vec![choice(
                "choice:arcanist_metamagic_knowledge",
                "metamagic:empower_spell",
            )],
            vec![
                spell("arcanist", AcquisitionMode::Known),
                spell("arcanist", AcquisitionMode::Prepared),
            ],
        ),
        "sorcerer" => (
            vec![
                choice("choice:sorcerer_bloodline", "bloodline:arcane"),
                choice("choice:sorcerer_arcane_bond", "bond:familiar"),
            ],
            Vec::new(),
        ),
        "cleric" => (vec![choice("choice:cleric_domain", "domain:good")], Vec::new()),
        "druid" => (
            vec![choice("choice:druid_nature_bond", "bond:animal_companion")],
            Vec::new(),
        ),
        "monk" => (vec![choice("choice:monk_bonus_feat", "feat:dodge")], Vec::new()),
        "witch" => (vec![choice("choice:witch_hex", "hex:flight")], Vec::new()),
        "shaman" => (vec![choice("choice:shaman_spirit", "spirit:life")], Vec::new()),
        "alchemist" => (
            vec![choice("choice:alchemist_discovery", "discovery:feral_mutagen")],
            vec![
                extract("alchemist", AcquisitionMode::Known),
                extract("alchemist", AcquisitionMode::Prepared),
            ],
        ),
        "investigator" => (
            vec![choice("choice:investigator_talent", "talent:resiliency")],
            vec![
                extract("investigator", AcquisitionMode::Known),
                extract("investigator", AcquisitionMode::Prepared),
            ],
        ),
        "warpriest" => (
            vec![choice("choice:warpriest_blessing", "blessing:destruction")],
            vec![
                spell("warpriest", AcquisitionMode::Known),
                spell("warpriest", AcquisitionMode::Prepared),
            ],
        ),
        "bloodrager" => (
            vec![choice("choice:bloodrager_bloodline", "bloodline:arcane")],
            Vec::new(),
        ),
        "summoner" => (
            vec![choice(
                "choice:summoner_eidolon_evolution",
                "evolution:improved_natural_armor",
            )],
            Vec::new(),
        ),
        "cavalier" => (vec![choice("choice:cavalier_order", "order:sword")], Vec::new()),
        "inquisitor" => (vec![choice("choice:inquisitor_domain", "domain:good")], Vec::new()),
        "oracle" => (
            vec![
                choice("choice:oracle_mystery", "mystery:life"),
                choice("choice:oracle_curse", "curse:clouded_vision"),
            ],
            Vec::new(),
        ),
        _ => (Vec::new(), Vec::new()),
    }
}

fn class_sweep_input(fixture: &CharacterInput, class_name: &str, level: u8) -> CharacterInput {
    let mut input = posture_input(fixture, class_name, level);
    let (choices, spells) = canonical_seeds_for(class_name);
    input.chosen.selected_choices.extend(choices);
    input.chosen.spells_selected.extend(spells);
    input
}

/// `"Greater Weapon Focus"` -> `"choice:greater_weapon_focus_target"`, the
/// engine's own choice-set naming rule (see
/// `v06_content_state_dump::derived_choice_set_id`).
fn derived_choice_set_id(feat_key: &str) -> String {
    let s: String = feat_key
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    format!("choice:{s}_target")
}

/// The computed facts a feat is allowed to move. `diagnostics` is deliberately
/// excluded: a feat whose prerequisites this character fails makes the engine
/// emit a diagnostic without computing anything, and counting that as "wired"
/// would inflate the number with exactly the feats that do nothing. Mirrors
/// `v06_content_state_dump::observable_facts`.
fn observable_facts(c: &PilotBaseChassisComputation) -> (Vec<(String, i16)>, [i16; 12]) {
    let explanations = c.explanations.iter().map(|e| (e.id.clone(), e.value)).collect();
    let numbers = [
        c.ability_modifiers.strength,
        c.ability_modifiers.dexterity,
        c.ability_modifiers.constitution,
        c.base_attack_bonus,
        c.base_saves.fortitude,
        c.base_saves.reflex,
        c.base_saves.will,
        c.baseline_melee_attack_bonus,
        c.baseline_armor_class,
        c.total_saves.fortitude,
        c.total_saves.reflex,
        c.total_saves.will,
    ];
    (explanations, numbers)
}

/// The feat probe's baseline posture: the fixture's three pre-granted feats
/// stripped, so the probe measures the feat rather than the fixture. Mirrors
/// `v06_content_state_dump::feat_probe_input` (including its reasoning: Dodge's
/// +1 AC and Weapon Focus' +1 attack are already applied on the shared fixture
/// and neither stacks with itself).
fn feat_probe_input(fixture: &CharacterInput, class_name: &str, level: u8) -> CharacterInput {
    let mut input = posture_input(fixture, class_name, level);
    input.chosen.selected_feats.retain(|f| !f.starts_with("feat:"));
    input.chosen.selected_choices.retain(|c| {
        !(c.choice_set_id.ends_with("_bonus_feat") || c.choice_set_id.ends_with("_character_feat"))
    });
    input
}

/// Every catalog feat key whose presence genuinely changes what this engine
/// computes. **A lower bound, and says so**: a feat whose effect needs a
/// posture outside the swept set reads as unwired here even if some code
/// somewhere mentions it. That is the honest direction to be wrong in for a
/// work inventory.
fn probe_feat_effect_wiring(fixture: &CharacterInput) -> BTreeSet<String> {
    let mut wired = BTreeSet::new();
    let keys: BTreeSet<&'static str> = all_feat_tables()
        .iter()
        .flat_map(|t| t.entries.iter().map(|e| e.key))
        .collect();

    for class_name in PROBE_CLASSES {
        for &level in PROBE_LEVELS {
            let base_input = feat_probe_input(fixture, class_name, level);
            let baseline = observable_facts(&compute_pilot_base_chassis(&base_input));
            for &key in &keys {
                if wired.contains(key) {
                    continue;
                }
                let choice_set = derived_choice_set_id(key);
                for variant in 0..=PROBE_SELECTIONS.len() {
                    let mut input = base_input.clone();
                    input.chosen.selected_feats.push(key.to_string());
                    if variant > 0 {
                        input.chosen.selected_choices.push(SelectedChoice {
                            choice_set_id: choice_set.clone(),
                            selection_id: PROBE_SELECTIONS[variant - 1].to_string(),
                        });
                    }
                    if observable_facts(&compute_pilot_base_chassis(&input)) != baseline {
                        wired.insert(key.to_string());
                        break;
                    }
                }
            }
        }
    }
    wired
}

/// Book roots the on-disk `data/corpus/<book>/` loader can currently reach
/// (`corpus_loader.rs`'s own doc comment: equipment content-kind first, now
/// joined by spell). This is exactly the set `data/corpus/` holds today --
/// re-derive with `ls data/corpus/` before trusting it if that changes.
const OBSERVABLE_BOOK_DIRS: &[&str] = &[
    "core_rulebook",
    "advanced_players_guide",
    "advanced_class_guide",
    "beastiary",
    "advanced_race_guide",
    "pathfinder_unchained",
    // SD-31 SD31-E6-F5-001: `gen_cache_ultimate_equipment` (+
    // `enrich_equipment_raw_tokens`) landed
    // `data/corpus/ultimate_equipment/equipment/*.json` this cycle --
    // previously the book had no corpus directory at all
    // (`OPEN-ISSUES.md` row 12), so `probe_equipment_effect_wiring`
    // never observed it and every UE-keyed equipment/equipment_modifier
    // unit stayed `ingested-magnitude`/`held` regardless of the real
    // catalog's `BONUS:STAT` content.
    "ultimate_equipment",
    // `SD31-E6-F5-003`: `gen_cache_equipment_gap` (wave 4) and
    // `gen_cache_hand_authored_equipment` (this cycle) both landed real
    // `data/corpus/<book>/equipment/*.json` content for these five books
    // -- none were ever added here, so `probe_equipment_effect_wiring`
    // has never observed any of them despite hundreds of real, cited
    // records now on disk (the identical `OPEN-ISSUES.md` row 12 shape
    // the `ultimate_equipment` entry above already names, found five
    // books later). Append-only per this wave's shared-file discipline.
    "ultimate_combat",
    "ultimate_intrigue",
    "ultimate_psionics",
    "ultimate_wilderness",
    "ultimate_magic",
];

fn book_corpus_roots(repo_root: &Path) -> Vec<PathBuf> {
    OBSERVABLE_BOOK_DIRS.iter().map(|b| repo_root.join("data/corpus").join(b)).collect()
}

/// Where the desktop app declares which books' race content it loads.
const RACE_CATALOG_SOURCE_RELATIVE: &str = "apps/desktop/src-tauri/src/race_catalog.rs";

/// Bestiary 1's `data/corpus/` directory is spelled `beastiary`, and
/// `corpus_dir_for` spells the same book `bestiary` — that one is the PCGen
/// SOURCE tree's directory, and the two names have simply never agreed.
///
/// `engine_book_for` keys on the source spelling, so a record whose
/// `book_id` came off disk needs the alias applied first or it resolves to no
/// engine book at all. `reach_gate::CORPUS_BOOK_IDS` records the same
/// divergence for the same reason (`("beastiary", "beastiary1")`).
///
/// Stated as a one-entry alias rather than papered over with a fuzzy match:
/// [`every_corpus_book_with_race_traits_resolves_to_an_engine_book`] proves
/// this is the only book that needs one.
const CORPUS_DIR_ALIASES: &[(&str, &str)] = &[("beastiary", "bestiary")];

/// [`engine_book_for`], for a `data/corpus/<dir>` directory name.
fn engine_book_for_corpus_dir(dir: &str) -> Option<&'static str> {
    let source_dir = CORPUS_DIR_ALIASES
        .iter()
        .find(|(corpus, _)| *corpus == dir)
        .map(|(_, source)| *source)
        .unwrap_or(dir);
    engine_book_for(source_dir)
}

/// The corpus books whose race content the shipped app really loads, read out
/// of the app's own `RACE_CORPUS_BOOKS` declaration.
///
/// Read rather than duplicated, and read from the *product's* source rather
/// than from a list in this file, because the claim this probe makes is about
/// the product. A hand-copied list here would keep reporting `grounded` for a
/// book the app had stopped loading, which is precisely the over-claim this
/// inventory exists to prevent. `tests/duergar_invisibility_sla_reaches_a_
/// player_via_monster_codex.rs` already parses the same declaration the same
/// way for the same reason.
///
/// An unreadable or unparseable declaration yields an EMPTY list, never a
/// guessed one: the probe then observes nothing, every race trait falls back
/// to the CRB-table rule below, and the inventory under-claims. Under-claiming
/// on a broken read is the safe direction.
fn app_race_corpus_books(repo_root: &Path) -> Vec<String> {
    let Ok(src) = std::fs::read_to_string(repo_root.join(RACE_CATALOG_SOURCE_RELATIVE)) else {
        return Vec::new();
    };
    let Some(decl) = src.split("pub(crate) const RACE_CORPUS_BOOKS: &[&str] =").nth(1) else {
        return Vec::new();
    };
    let Some(list) = decl.split(';').next() else {
        return Vec::new();
    };
    list.split('"').skip(1).step_by(2).map(str::to_owned).collect()
}

/// Every race-trait record the app's loaded race corpus can actually APPLY to
/// a player, keyed by the record's own source coordinates
/// (`(<lst basename>, <line>)`) and valued by the corpus book it came from.
///
/// # Why this exists (SD-29 `decisions.md §43.5`)
///
/// `race_trait_ids` below is built solely from `crb::race_traits()` — CRB's
/// seven compiled races. The **product** models eighteen, read off disk at
/// runtime by `race_resolver::load_race_corpus`, which is what
/// `race_trait_picker` and `list_alternate_racial_traits` serve. So every
/// ingested race trait belonging to a non-CRB race reported
/// `race_trait_race_not_modelled` **no matter how reachable it was** — ARG's
/// 156, Bestiary 1's 108 and Monster Codex's 5 among them, the last of which
/// `reach_gate` simultaneously carried a passing claim for and SD-29
/// photographed in the player's own picker. That is the doneness-instrument
/// hierarchy inverted: the narrower instrument was overruling the one that
/// executes the real path.
///
/// # What it will not do
///
/// It does not ground a record for being on disk. A record whose role is
/// [`TraitRole::Unclassified`] carries no readable gate and is never applied
/// by `RaceCorpus::resolve`, so it is deliberately absent here — `Oversized
/// Goblin` is the live instance, and it has a standing `OPEN_FINDINGS` entry
/// naming its remedy. Nor does it reach a trait whose race has no chassis in
/// any loaded book: `race_keys()` yields only races that have one, and
/// `resolve` returns `None` without one. Both exclusions keep this probe's
/// answer identical to what a player can actually obtain.
///
/// The join key is the source coordinate, never the name. A race trait's
/// display name is not unique corpus-wide — the name-coincidence defect
/// `modelled_race_of_race_trait` exists to close is the proof — whereas the
/// `.lst` file and line the ingest records verbatim is an identity.
fn probe_reachable_race_traits(repo_root: &Path) -> BTreeMap<(String, usize), String> {
    probe_race_trait_corpus(repo_root).reachable
}

/// What one load of the app's race corpus tells this generator.
#[derive(Debug, Default)]
struct RaceTraitProbe {
    /// Records the resolver can apply -> the corpus book each came from.
    reachable: BTreeMap<(String, usize), String>,
    /// EVERY record the load found, applicable or not. The difference between
    /// the two sets is the honest middle status: ingested, loaded, and still
    /// inert. Reporting those as "not ingested" would be a lie in the other
    /// direction, and reporting them as grounded would be the lie this whole
    /// generator exists to prevent.
    loaded: BTreeSet<(String, usize)>,
    /// Every loaded record's description, rendered by the exact function
    /// `race_trait_picker::build_menu` calls (`record.render_description`
    /// against `record.same_row_display_values()` -- the zero-feats,
    /// catalog-level rendering the menu itself uses with an empty feat
    /// list). SD31-D7-PROSE-001: the render path a real Tauri command
    /// already serves to the player, reused rather than re-implemented, so
    /// "renders on screen" is proven by construction, not asserted.
    rendered: BTreeMap<(String, usize), String>,
}

fn probe_race_trait_corpus(repo_root: &Path) -> RaceTraitProbe {
    let books = app_race_corpus_books(repo_root);
    let dirs: Vec<(String, PathBuf)> =
        books.into_iter().map(|b| (b.clone(), repo_root.join("data/corpus").join(b))).collect();
    let roots: Vec<BookCorpusRoot<'_>> = dirs
        .iter()
        .map(|(book, dir)| BookCorpusRoot { book_id: book.as_str(), dir: dir.as_path() })
        .collect();
    let corpus = load_race_corpus(&roots);

    let mut probe = RaceTraitProbe::default();
    // `race_keys()` yields only races that have a chassis record in some
    // loaded book. A trait whose race has none is left out of BOTH sets by
    // construction, which is right: `RaceCorpus::resolve` returns `None`
    // without a chassis, so no player can obtain it and no ingest of the
    // trait alone would change that. Monster Codex's six Ratfolk rows are
    // the live instance -- the pilot skipped writing them for exactly this
    // reason (SD-29 `decisions.md §43.4`).
    for race in corpus.race_keys() {
        for record in corpus.traits_for(race) {
            let Some(file) = Path::new(&record.source_path).file_name() else { continue };
            let coordinate = (file.to_string_lossy().into_owned(), record.source_line as usize);
            probe.loaded.insert(coordinate.clone());
            probe.rendered.insert(
                coordinate.clone(),
                record.render_description(&record.same_row_display_values()).text,
            );
            if record.role == TraitRole::Unclassified {
                continue;
            }
            probe.reachable.insert(coordinate, record.book_id.clone());
        }
    }
    probe
}

// SUPERSEDED 2026-08-13 (SD-32 `spell-consumer-delta-probe`), in its
// conclusion only. Everything below about the RETRACTED first attempt stands
// and is the reason this file keeps it: probing `school_coverage` observed
// resolution, not a magnitude, and promoted 1,067 of 1,067.
//
// Its closing conclusion -- "there is currently no wired spell-magnitude
// consumer to observe at all" -- was true when written and is now false.
// `epic-31-spell-wiring` (2026-08-07) wired `compute_spellbook_coverage` into
// `pf1_adapter::resolve_unified_pilot_snapshot`, which is the surface the
// desktop sheet reads; `contract::build_pilot_receipt` being uncalled, cited
// below as the blocker, is no longer the only route. `probe_spell_key` (above)
// observes that wired consumer. See its own block comment.
//
// SD28-E14-F1: **NOT implemented as a promoting probe.** An earlier version
// of this cycle probed `pilot_compute_corpus::compute_pilot_with_corpus`'s
// `school_coverage` map and promoted any spell that landed in it. Corrected
// after independent review (team-lead, 2026-08-06): `school_coverage` is
// populated purely by resolving the spell and reading its `school` string
// (`pilot_compute_corpus.rs:189-205`) -- no spell magnitude (level, DC,
// duration, ...) is read into any field a consumer produces. The predicate
// reduced to "this spell resolves against the on-disk corpus", which is a
// restatement of `ingested-magnitude`'s own existing evidence
// (`spell_list_entry_with_resolved_level`) through a compute call, not an
// observation of a delta. Confirmed by the 100% promotion rate (1,067 of
// 1,067) the wrong version produced -- a discriminating probe over real
// content does not do that; compare F2's 173-of-2,983 (5.8%).
//
// A genuine spell-magnitude consumer exists in this repo --
// `spellbook::compute_spellbook_coverage` reads each resolved spell's real
// `level` into `SpellEffect.level`, and from it computes `spell_save_dc`/
// `slots_total`/`slots_used`, wired into `contract::PilotReceipt.spellbook`
// (`contract.rs:397`) and from there into `sheet.spellbook.*` cells
// (`contract.rs:794-810`). But `contract::build_pilot_receipt` is never
// called by `apps/desktop/src-tauri/src/pf1_adapter.rs` or `character_hub.rs`
// (confirmed: `grep -rn build_pilot_receipt apps/desktop/src-tauri/src`
// returns nothing) -- it is exactly the "twin problem" `decisions.md §29.1`/
// `§29.2` already names: a real computation that never reaches the surface
// `pf1_adapter::resolve_unified_pilot_snapshot` gates on, i.e. not the twin
// the player reads. So there is currently no wired spell-magnitude consumer
// to observe at all, in either direction -- not a harness gap this cycle can
// close, an engine-wiring gap the next cycle that touches `contract.rs`/
// `pf1_adapter.rs` would have to close first. See
// `artifacts/e14-harness-widening.md` for the full finding; every targeted
// spell unit stays `ingested-magnitude`.

/// SD28-E14-F2: observes a real computed delta for an equipment (or
/// equipment-modifier) item -- the same `equipment_effects::compute_equipment_effects`
/// pipeline `pilot_compute_corpus::compute_pilot_with_corpus` (the twin the
/// player reads) already calls -- rather than the mere presence of an
/// equipment-table entry carrying a corpus magnitude
/// (`classify()`'s old `Kind::Equipment`/`Kind::EquipmentModifier` stop
/// point). An item that resolves against the real on-disk corpus but whose
/// record carries none of the mechanical tokens `compute_equipment_effects`
/// reads (armor/max-dex/spell-failure/armor-check-penalty/skill/ability/
/// weapon-enhancement) produces an all-`None` per-item effect and correctly
/// stays unwired -- see
/// `equipment_effect_probe_never_promotes_a_text_only_item_with_no_mechanical_tokens`.
///
/// **Book-scoped, and the result is keyed `(engine_book, key)`.** Each book's
/// catalog keys are resolved against **that book's own corpus, loaded alone**,
/// never against a merged corpus of every observable book. Two independent
/// reasons, both of them the same recorded defect this repo already fixed once
/// for `race_trait` (`modelled_race_of_race_trait`'s doc comment, SD-28 §56):
///
///   * A merged corpus lets a book with **no** corpus of its own ground on
///     another book's record purely because the two share a key. Widening the
///     key universe to the whole catalog surfaced this immediately: Ultimate
///     Equipment has no `data/corpus/ultimate_equipment` at all, yet six of
///     its units grounded off ARG/CRB rows. `Celestial Shield` is the proof
///     that a shared key is not a shared item -- ARG's
///     (`arg_equip_arms_armor.lst:22`) is a **heavy** shield, 13,170 gp,
///     `ACCHECK:0`, `SPELLFAILURE:0`; UE's (`ue_equip_arms_armor.lst:126`) is
///     a **light** shield, 4,020 gp, `ACCHECK:-1`, `SPELLFAILURE:5`, with a
///     `BONUS:COMBAT|AC` chain of its own. Reporting UE's unit as grounded on
///     ARG's numbers is the over-claim, not a hedge.
///   * Even between two books that both have a corpus, resolution order
///     decided which record answered. Scoping makes the attribution a rule
///     rather than a coincidence of iteration order.
///
/// A book with a catalog but no corpus directory therefore gets no probe
/// coverage at all and its units stay `ingested-magnitude`. That is the honest
/// result: nothing observed it, so nothing may claim it.
/// Every equipment key the probe asks the wiring question of.
///
/// **Derived from the engine catalog, never hand-listed.** `classify()`'s
/// `Kind::Equipment`/`Kind::EquipmentModifier` arm decides `known` from
/// `facts.equipment_keys`, which is built from
/// `equipment_resolver::equipment_catalog_rows()` (SD-28-E15 rebuilt it that
/// way for exactly this reason). The probe's key universe was four
/// hand-maintained `.extend()` calls over `crb`/`apg`/`acg`/`beastiary1`'s
/// compiled tables — so every key the catalog holds from any *other* source
/// was never examined at all, and its unit could only ever report
/// `equipment_table_entry_with_corpus_magnitude`. Two populations were
/// invisible to the probe this way:
///
///   * the four books' own **gap rows** — `equipment_gap_tables` supplies 335
///     `core_rulebook` records the hand-authored CRB table does not hold
///     (`CLOTH`, `LEATHER`, `MWORKW`, … — equipment *modifiers*, the largest
///     `in-progress` population on the board), plus APG/ACG/ARG rows;
///   * every book with a catalog but no entry in the four calls — ARG, PU,
///     UM, UC, UI, UE, UPSI, UW.
///
/// This is Decision 36's pattern (two lists of the same fact, never
/// reconciled) one function over from where SD-28-E15 already fixed it, and
/// [`the_probe_examines_every_key_the_engine_catalog_holds`] pins it closed.
///
/// **This widens what is ASKED, not what counts as an answer.** The bar is
/// still [`equipment_key_is_wired`], unchanged: the item must resolve against
/// the real on-disk corpus and produce at least one non-`None` mechanical
/// stat effect. A key from a book whose corpus is not loaded resolves to
/// nothing and stays unwired, which is the honest result rather than a
/// promotion.
fn probe_equipment_key_universe() -> BTreeSet<&'static str> {
    equipment_resolver::equipment_catalog_rows()
        .iter()
        .map(|row| row.key)
        .collect()
}

/// [`probe_equipment_key_universe`], partitioned by the engine book whose
/// catalog supplied each key.
fn probe_equipment_keys_by_book() -> BTreeMap<&'static str, BTreeSet<&'static str>> {
    let mut by_book: BTreeMap<&'static str, BTreeSet<&'static str>> = BTreeMap::new();
    for row in equipment_resolver::equipment_catalog_rows() {
        by_book.entry(equipment_book_slug_for(row.book)).or_default().insert(row.key);
    }
    by_book
}

fn probe_equipment_effect_wiring(repo_root: &Path) -> BTreeSet<(String, String)> {
    let mut wired = BTreeSet::new();
    let keys_by_book = probe_equipment_keys_by_book();

    for (dir_name, dir) in OBSERVABLE_BOOK_DIRS.iter().zip(book_corpus_roots(repo_root)) {
        let Some(engine_book) = engine_book_for_corpus_dir(dir_name) else {
            continue;
        };
        let Some(keys) = keys_by_book.get(engine_book) else {
            continue;
        };
        // One book's corpus, alone. See this function's doc comment.
        let roots = [BookCorpusRoot { book_id: engine_book, dir: &dir }];
        let corpus = load_equipment_corpus(&roots);
        if corpus.is_empty() {
            continue;
        }
        for &key in keys {
            if equipment_key_is_wired(key, &corpus) {
                wired.insert((engine_book.to_string(), key.to_string()));
            }
        }
    }
    wired
}

/// Whether equipping exactly this item, alone, against the real corpus
/// produces at least one non-`None` mechanical stat effect. Split out from
/// [`probe_equipment_effect_wiring`] as its own pure function so the
/// negative test can call it directly against a hand-built corpus, the same
/// shape `equipment_effects.rs`'s own tests already use.
fn equipment_key_is_wired(
    key: &str,
    corpus: &codex::rules_core::source_content::SourcePackageContent,
) -> bool {
    let selection =
        vec![EquipmentSelection {
            item_id: key.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }];
    let effects = compute_equipment_effects(&selection, corpus);
    let Some(item) = effects.per_item.first() else { return false };
    item.armor_class_bonus.is_some()
        || item.max_dex.is_some()
        || item.spell_failure.is_some()
        || item.armor_check_penalty.is_some()
        || item.skill_bonus.is_some()
        || item.ability_bonus.is_some()
        || item.weapon_enhancement_bonus.is_some()
}

// ---------------------------------------------------------------------------
// Spell consumer-delta probe
// ---------------------------------------------------------------------------
//
// Why this exists NOW when SD-28-E14-F1 recorded that it could not.
//
// That epic's finding (the long note further down this file, and
// `docs/release/SD-28-ultimate-book-content-ingestion/artifacts/e14-harness-widening.md`)
// was correct on the day it was written and is now STALE. It said:
// `spellbook::compute_spellbook_coverage` does read a real spell magnitude
// (`SpellEffect.level` -> `spell_save_dc`), but it was wired only into
// `contract::PilotReceipt`, and `contract::build_pilot_receipt` is called by
// no desktop command -- the "twin problem", a real computation nothing on
// screen reads.
//
// `epic-31-spell-wiring` (2026-08-07) closed exactly that gap.
// `pf1_adapter::resolve_unified_pilot_snapshot` now calls
// `compute_spellbook_coverage` itself and projects it through
// `PilotSpellbookViewModel::from_coverage` onto the `PilotSnapshot` the app
// renders (`character_hub::map_snapshot_dto` -> `PilotSnapshotDto.spellbook`
// -> `CharacterSheet.tsx` renders `spellbook.spellSaveDc`). Verify with:
//
//   grep -n 'PilotSpellbookViewModel::from_coverage' apps/desktop/src-tauri/src/pf1_adapter.rs
//   grep -n 'spellSaveDc' apps/desktop/src/characterHub/CharacterSheet.tsx
//
// So there IS now a wired consumer that reads a spell's own magnitude, and
// this probe observes it. It composes the same two engine calls the adapter
// composes, in the same order -- `compute_spellbook_coverage` then
// `PilotSpellbookViewModel::from_coverage` -- so what it measures is the value
// the sheet prints, not a private field beside it.
//
// What it does NOT do, stated plainly: it does not drive
// `resolve_unified_pilot_snapshot` itself. That function lives in
// `apps/desktop/src-tauri`, a separate cargo workspace this root-crate binary
// cannot call, and it emits a snapshot only for a build whose receipt is
// `Computed`. The probe therefore proves "this spell's own level produces the
// save-DC value the sheet's spellbook cell renders", not "this particular
// character build reaches Computed". That is the same boundary
// `probe_equipment_effect_wiring` already sits behind (it calls
// `compute_equipment_effects` directly, not the adapter).

/// Ability score the spell probe fixes on every casting ability, so the save
/// DC it observes has exactly one arithmetic explanation. 18 -> modifier +4
/// (`floor(18/2) - 5`).
const SPELL_PROBE_ABILITY_SCORE: i16 = 18;

/// [`SPELL_PROBE_ABILITY_SCORE`]'s PF1 ability modifier, stated as the probe's
/// oracle input rather than read back out of the engine -- the whole point of
/// the comparison below is that the two sides are derived independently.
const SPELL_PROBE_ABILITY_MODIFIER: i16 = 4;

/// The casting classes the probe will select a spell through, each paired with
/// that class's OWN per-class CRB spell-list accessor.
///
/// Not "any class". `spellbook::compute_spellbook_coverage` computes a save DC
/// for whatever `source_class_id` it is handed and never checks that the class
/// can cast the spell, so probing every spell as a Wizard would produce a real
/// number for a posture no player can build -- "a magnitude no player can
/// see", the failure this program has recorded three times. The probe asks
/// each class's own list first and selects the spell only through a class that
/// really has it.
///
/// These seven are exactly the ids `spellbook::casting_ability_for_class` maps
/// to a casting ability; a class it does not map yields no DC at all, so
/// probing through one could observe nothing.
const SPELL_PROBE_CASTING_CLASSES: &[(&str, fn(&str) -> Option<u8>)] = &[
    ("class:wizard", crb_wizard_spell_list::wizard_spell_level),
    ("class:cleric", crb_cleric_spell_list::cleric_spell_level),
    ("class:druid", crb_druid_spell_list::druid_spell_level),
    ("class:bard", crb_bard_spell_list::bard_spell_level),
    ("class:sorcerer", crb_sorcerer_spell_list::sorcerer_spell_level),
    ("class:paladin", crb_paladin_spell_list::paladin_spell_level),
    ("class:ranger", crb_ranger_spell_list::ranger_spell_level),
];

/// Why one spell key did or did not produce an observed consumer delta.
///
/// An enum rather than a `bool` because the ceiling report has to say what the
/// probe *cannot* reach and why, and a boolean can only say "no".
#[derive(Debug, Clone, PartialEq, Eq)]
enum SpellProbeOutcome {
    /// A real save-DC magnitude, attributable to this spell's own level,
    /// appeared on the very view model the character sheet renders.
    Wired { class_id: &'static str, level: u8, dc: u8 },
    /// No CRB casting class has this spell on its own list, so no player can
    /// put it in a spellbook and any DC it produced would be unreachable.
    NoCastingClassHasIt,
    /// The key is in this book's spell catalog but no record of that name
    /// exists in this book's own on-disk corpus.
    AbsentFromBookCorpus,
    /// The corpus record carries no `SCHOOL:` this engine recognizes, so
    /// `compute_spellbook_coverage` dispatches to no school function.
    SchoolNotRecognized,
    /// Resolved, school recognized, but no per-school table record exists for
    /// the key -- the spell is not in `crb::spell_list::SPELL_LIST` under that
    /// school, so no `SpellEffect` and so no level is produced.
    NoTableEffect,
    /// The magnitude came from a different book's table than the one whose
    /// unit would claim it. The `Celestial Shield` discipline
    /// (`probe_equipment_effect_wiring`'s doc comment) applied to spells: a
    /// shared NAME is not a shared record.
    ForeignBookTable,
    /// A `SpellEffect` was produced but the projection the sheet renders
    /// carried no save DC for the selecting class.
    NoSaveDcOnViewModel,
    /// A DC appeared but not the one this spell's own level explains. Never
    /// promoted: an unexplained number is not an observed magnitude.
    DcDisagreesWithOracle { observed: u8, oracle: i16 },
    /// The same character with this spell NOT selected already carried a save
    /// DC, so nothing about the observed DC is attributable to this spell.
    /// Never promoted -- this is the "delta" half of consumer-delta.
    BaselineAlreadyCarriesADc,
}

/// The first casting class whose own CRB spell list holds `key`.
fn probe_casting_class_for_spell(key: &str) -> Option<&'static str> {
    SPELL_PROBE_CASTING_CLASSES
        .iter()
        .find(|(_, level_of)| level_of(key).is_some())
        .map(|(class_id, _)| *class_id)
}

/// The probe's character posture: the shared fixture with every casting
/// ability pinned to [`SPELL_PROBE_ABILITY_SCORE`] and exactly the given spell
/// selection. `compute_spellbook_coverage` reads only `spells_selected` and
/// `ability_scores`, so nothing else about the fixture can influence what the
/// probe observes.
fn spell_probe_input(
    fixture: &CharacterInput,
    class_id: &str,
    spell_id: Option<&str>,
) -> CharacterInput {
    let mut input = fixture.clone();
    input.chosen.ability_scores.intelligence = SPELL_PROBE_ABILITY_SCORE;
    input.chosen.ability_scores.wisdom = SPELL_PROBE_ABILITY_SCORE;
    input.chosen.ability_scores.charisma = SPELL_PROBE_ABILITY_SCORE;
    input.chosen.spells_selected = match spell_id {
        Some(id) => vec![SpellSelection {
            spell_id: id.to_string(),
            source_class_id: class_id.to_string(),
            acquisition_mode: AcquisitionMode::Prepared,
        }],
        None => Vec::new(),
    };
    input
}

/// Whether selecting exactly this spell, alone, for a class that really has
/// it, against one book's own corpus, produces a save-DC magnitude explained
/// by that spell's own level on the surface the character sheet renders.
///
/// The spell-side sibling of [`equipment_key_is_wired`], and deliberately a
/// stricter bar than that one: equipment asks only "is some field non-`None`",
/// while this additionally requires the observed number to equal an
/// independently-stated oracle (`10 + level + modifier`). It has to be
/// stricter. A spell selection that merely resolves already has its own status
/// (`ingested-magnitude`), and SD-28-E14-F1's retracted first attempt failed
/// precisely by building a predicate that reduced to "this spell resolves"
/// -- it promoted 1,067 of 1,067.
fn probe_spell_key(
    fixture: &CharacterInput,
    key: &str,
    corpus: &codex::rules_core::source_content::SourcePackageContent,
    book_rule_set: RuleSetId,
) -> SpellProbeOutcome {
    let Some(class_id) = probe_casting_class_for_spell(key) else {
        return SpellProbeOutcome::NoCastingClassHasIt;
    };
    let Some((record, _)) = spell_id_resolve(key, book_rule_set, corpus) else {
        return SpellProbeOutcome::AbsentFromBookCorpus;
    };
    if record
        .school
        .as_deref()
        .and_then(crb_spell_list::Pf1SchoolId::from_corpus_str)
        .is_none()
    {
        return SpellProbeOutcome::SchoolNotRecognized;
    }

    // The delta's baseline: the same character, same corpus, this spell NOT
    // selected. If a DC is already there, the one observed below is not
    // attributable to this spell and must not be claimed for it.
    let baseline = compute_spellbook_coverage(&spell_probe_input(fixture, class_id, None), corpus);
    if PilotSpellbookViewModel::from_coverage(&baseline).is_some() {
        return SpellProbeOutcome::BaselineAlreadyCarriesADc;
    }

    let coverage =
        compute_spellbook_coverage(&spell_probe_input(fixture, class_id, Some(key)), corpus);
    let Some(prepared) = coverage.spells_prepared.first() else {
        return SpellProbeOutcome::NoTableEffect;
    };
    if prepared.effect.table_cell.rule_set != book_rule_set {
        return SpellProbeOutcome::ForeignBookTable;
    }
    let Some(view) = PilotSpellbookViewModel::from_coverage(&coverage) else {
        return SpellProbeOutcome::NoSaveDcOnViewModel;
    };
    let Some(observed) = view.spell_save_dc.iter().find(|entry| entry.class_id == class_id) else {
        return SpellProbeOutcome::NoSaveDcOnViewModel;
    };

    // The oracle: the DC the spell's OWN level explains, built from this
    // function's own two constants rather than read back out of the value
    // under test.
    let oracle = 10i16 + i16::from(prepared.effect.level) + SPELL_PROBE_ABILITY_MODIFIER;
    if i16::from(observed.dc) != oracle {
        return SpellProbeOutcome::DcDisagreesWithOracle { observed: observed.dc, oracle };
    }
    SpellProbeOutcome::Wired { class_id, level: prepared.effect.level, dc: observed.dc }
}

// There is deliberately no `spell_key_is_wired` bool wrapper beside
// [`probe_spell_key`], despite `equipment_key_is_wired` being the model for
// this probe. Nothing in this binary consults the spell probe's verdict yet
// (this cycle builds and proves the instrument; `classify()`'s `Kind::Spell`
// arm is untouched), so a wrapper would ship as dead code and spend a clippy
// warning against the recorded ceiling for nothing. `probe_spell_key` already
// carries strictly more information than a bool; the cycle that wires the
// verdict into `classify()` reads it directly.

/// Every spell key the engine catalog holds, partitioned by the engine book
/// that supplied it -- the spell-side sibling of
/// [`probe_equipment_keys_by_book`], derived from the same registry
/// (`spell_resolver::spell_catalog_rows`) `classify()`'s `Kind::Spell` arm
/// already decides `known` from, so the probe asks its question of exactly the
/// population the classifier judges.
fn probe_spell_keys_by_book() -> BTreeMap<&'static str, BTreeSet<&'static str>> {
    let mut by_book: BTreeMap<&'static str, BTreeSet<&'static str>> = BTreeMap::new();
    for row in spell_resolver::spell_catalog_rows() {
        by_book.entry(spell_book_slug_for(row.book)).or_default().insert(row.key);
    }
    by_book
}

/// The engine rule set whose [`rule_set_id`] is `engine_book`.
fn rule_set_for_engine_book(engine_book: &str) -> Option<RuleSetId> {
    COMPILED_RULE_SETS.iter().copied().find(|&rs| rule_set_id(rs) == engine_book)
}

/// Runs [`probe_spell_key`] over every catalog spell key of every observable
/// book, against that book's own corpus loaded ALONE -- the same book-scoping
/// discipline, and for the same recorded reason, as
/// [`probe_equipment_effect_wiring`].
///
/// Returns the full outcome per `(engine_book, key)` rather than only the
/// wired set, because the ceiling report needs the refusals and their reasons.
fn probe_spell_effect_wiring(
    fixture: &CharacterInput,
    repo_root: &Path,
) -> BTreeMap<(String, String), SpellProbeOutcome> {
    let mut outcomes = BTreeMap::new();
    let keys_by_book = probe_spell_keys_by_book();

    for (dir_name, dir) in OBSERVABLE_BOOK_DIRS.iter().zip(book_corpus_roots(repo_root)) {
        let Some(engine_book) = engine_book_for_corpus_dir(dir_name) else { continue };
        let Some(rule_set) = rule_set_for_engine_book(engine_book) else { continue };
        let Some(keys) = keys_by_book.get(engine_book) else { continue };
        // One book's corpus, alone. See this function's doc comment.
        let roots = [BookCorpusRoot { book_id: engine_book, dir: &dir }];
        let corpus = load_spell_corpus(&roots);
        for &key in keys {
            let outcome = probe_spell_key(fixture, key, &corpus, rule_set);
            outcomes.insert((engine_book.to_string(), key.to_string()), outcome);
        }
    }
    outcomes
}

/// The `(engine_book, key)` pairs [`classify`] may ground: exactly the probe's
/// [`SpellProbeOutcome::Wired`] verdicts, and nothing else.
///
/// A named function rather than an inline `filter` at the one call site so the
/// admission rule has somewhere to be tested against every refusal variant
/// (`only_wired_outcomes_enter_the_fact_set`). The match is exhaustive and
/// deliberately not a `_ =>` catch-all: a future outcome variant must be
/// classified as promoting or refusing by hand, not defaulted into either.
fn spell_effect_wired_from_outcomes(
    outcomes: &BTreeMap<(String, String), SpellProbeOutcome>,
) -> BTreeSet<(String, String)> {
    outcomes
        .iter()
        .filter(|(_, outcome)| match outcome {
            SpellProbeOutcome::Wired { .. } => true,
            SpellProbeOutcome::NoCastingClassHasIt
            | SpellProbeOutcome::AbsentFromBookCorpus
            | SpellProbeOutcome::SchoolNotRecognized
            | SpellProbeOutcome::NoTableEffect
            | SpellProbeOutcome::ForeignBookTable
            | SpellProbeOutcome::NoSaveDcOnViewModel
            | SpellProbeOutcome::DcDisagreesWithOracle { .. }
            | SpellProbeOutcome::BaselineAlreadyCarriesADc => false,
        })
        .map(|(pair, _)| pair.clone())
        .collect()
}

/// The probe's ceiling, printed by `--spell-probe`: how many catalog spell
/// keys it legitimately reaches and, for every one it does not, the reason it
/// refused. Grounding no unit, moving no number -- this is the instrument
/// reporting on itself.
fn spell_probe_ceiling_report(
    outcomes: &BTreeMap<(String, String), SpellProbeOutcome>,
) -> String {
    let mut per_book: BTreeMap<&str, BTreeMap<&'static str, usize>> = BTreeMap::new();
    let mut totals: BTreeMap<&'static str, usize> = BTreeMap::new();
    for ((book, _key), outcome) in outcomes {
        let label = match outcome {
            SpellProbeOutcome::Wired { .. } => "wired",
            SpellProbeOutcome::NoCastingClassHasIt => "no_casting_class_has_it",
            SpellProbeOutcome::AbsentFromBookCorpus => "absent_from_book_corpus",
            SpellProbeOutcome::SchoolNotRecognized => "school_not_recognized",
            SpellProbeOutcome::NoTableEffect => "no_table_effect",
            SpellProbeOutcome::ForeignBookTable => "foreign_book_table",
            SpellProbeOutcome::NoSaveDcOnViewModel => "no_save_dc_on_view_model",
            SpellProbeOutcome::DcDisagreesWithOracle { .. } => "dc_disagrees_with_oracle",
            SpellProbeOutcome::BaselineAlreadyCarriesADc => "baseline_already_carries_a_dc",
        };
        *per_book.entry(book.as_str()).or_default().entry(label).or_default() += 1;
        *totals.entry(label).or_default() += 1;
    }

    let mut out = String::new();
    out.push_str("spell consumer-delta probe -- ceiling report\n");
    out.push_str(&format!("keys examined: {}\n\n", outcomes.len()));
    for (book, counts) in &per_book {
        out.push_str(&format!("{book}\n"));
        for (label, n) in counts {
            out.push_str(&format!("  {label}: {n}\n"));
        }
    }
    out.push_str("\nTOTAL\n");
    for (label, n) in &totals {
        out.push_str(&format!("  {label}: {n}\n"));
    }
    out
}

// ---------------------------------------------------------------------------
// Class consumer-delta probe
// ---------------------------------------------------------------------------
//
// Why this exists, and what it is allowed to conclude.
//
// `classify()`'s `Kind::Class` arm grounded a class unit on ONE test: is this
// record's name a key of `class_books`, i.e. does some `ClassId`/`ApgClassId`/
// `AcgClassId` enum name it. That is a membership test on a Rust enum. It
// observes nothing about what the engine computes, and it would keep saying
// `grounded` if every class table in the crate were deleted and only the enum
// variants left behind. `class_modelled_and_swept_through_the_real_compute_pipeline`
// is the evidence string it emitted; the sweep in `engine_facts` really does
// run, but its result was unioned into `explanation_ids` for other kinds to
// consult and the class arm never read it back.
//
// This probe replaces that membership test with the same three-part bar the
// spell consumer-delta probe (`probe_spell_key`) established:
//
//   1. A real, creatable character of this class reaches
//      `HeadlessReceiptStatus::Computed` -- no claim-blocking diagnostic. This
//      is not a new bar invented here: it is the one this program already
//      ruled on and recorded, `docs/release/v0.6/risks-and-open-questions.md`
//      lines 208-210 ("'done' for any of the 24 classes must mean 'genuinely
//      reaches Computed'"), after a `done` claim was overstated and caught.
//   2. The magnitude reaches a consumer. The probe reads
//      `PilotViewModel::from_receipt(..).snapshot`, the projection
//      `PilotSnapshot::from_receipt` builds and the character sheet renders --
//      not a private field beside it. A `Blocked` receipt projects `None`.
//   3. The magnitude is attributable to selecting THIS class. A snapshot that
//      merely differs from the classless baseline only proves "having some
//      class computes something"; it cannot tell Fighter from Wizard. So the
//      probe additionally requires at least one explanation record whose id
//      names this class in its own dot-segment AND which no other modelled
//      class produces at that level. That is the `Celestial Shield` discipline
//      (`probe_equipment_effect_wiring`) in its class form: a shared row is
//      not this class's row.
//
// What it deliberately does NOT do: it does not drive
// `resolve_unified_pilot_snapshot`, which lives in `apps/desktop/src-tauri`, a
// separate cargo workspace this root-crate binary cannot call. Same boundary
// `probe_spell_key` and `probe_equipment_effect_wiring` already sit behind.
//
// The direction this probe moves the number is NOT assumed. It is strictly
// stricter than the membership test it replaces, so it can only confirm or
// demote the units that test grounded; it can promote nothing, because a class
// absent from `class_books` is a class the engine models nowhere and no delta
// can be observed for it. That is a finding about the corpus, not a weakness
// of the instrument, and `--class-probe` prints it rather than hiding it.

/// The levels the class probe evaluates. Identical to [`SWEEP_LEVELS`], and
/// deliberately the same postures `engine_facts`' existing class sweep already
/// walks, so the probe asks its question of exactly the population the
/// classifier judges.
const CLASS_PROBE_LEVELS: &[u8] = SWEEP_LEVELS;

/// Why one modelled class did or did not produce an observed consumer delta.
///
/// An enum rather than a `bool` for the same reason [`SpellProbeOutcome`] is
/// one: the ceiling report has to say what the probe *cannot* reach and why.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ClassProbeOutcome {
    /// A real creatable character of this class reached `Computed`, projected
    /// a snapshot that moved off the classless baseline, and carried at least
    /// one explanation record attributable to this class alone.
    Wired { level: u8, attributed_explanations: usize },
    /// The engine models no class of this name at all, so there is nothing to
    /// observe a delta against. Never promoted.
    NotModelledByEngine,
    /// The compute pipeline panicked at every probed level.
    PipelinePanicked,
    /// The pipeline ran but every probed level carried a claim-blocking
    /// diagnostic, so no level reaches `Computed` and no snapshot is projected.
    NeverReachesComputed,
    /// `Computed`, but `PilotViewModel::from_receipt` projected no snapshot --
    /// the consumer surface carried nothing.
    NoSnapshotProjected,
    /// A snapshot was projected but it is numerically identical to the same
    /// character with no class levels at all, so nothing on the rendered
    /// surface moved when this class was selected.
    NoSnapshotDeltaVsClasslessBaseline,
    /// The snapshot moved, but every explanation record naming this class is
    /// also produced by another modelled class, so the magnitude is not
    /// attributable to selecting this class. Never promoted.
    NoExplanationAttributedToThisClass,
}

/// The character posture the class probe measures: the shared fixture carrying
/// exactly this class at this level, with the same canonical creation-time
/// seeds `compose_character_input` applies. Reuses [`class_sweep_input`]
/// verbatim rather than forking a second posture builder, so the probe cannot
/// drift from the sweep whose population it judges.
fn class_probe_input(fixture: &CharacterInput, class_name: &str, level: u8) -> CharacterInput {
    class_sweep_input(fixture, class_name, level)
}

/// The delta's baseline: the same fixture with NO class levels at all. The
/// class-side equivalent of [`spell_probe_input`]'s `None` arm.
fn classless_probe_input(fixture: &CharacterInput) -> CharacterInput {
    let mut input = fixture.clone();
    input.case_id = Some("v06_work_inventory.classless_baseline".to_string());
    input.chosen.class_levels = Vec::new();
    input
}

/// The numbers a [`PilotSnapshot`] puts on the surface the character sheet
/// renders, flattened for equality comparison.
///
/// Every field here is one the sheet really prints. `ability_modifiers` is
/// deliberately included even though a class does not move it: leaving it out
/// would be choosing the comparison to favour a delta, and including a field
/// that never moves can only make the probe stricter, never looser.
fn class_snapshot_numbers(snapshot: &PilotSnapshot) -> Vec<i16> {
    vec![
        snapshot.ability_modifiers.strength,
        snapshot.ability_modifiers.dexterity,
        snapshot.ability_modifiers.constitution,
        snapshot.base_attack_bonus,
        snapshot.base_saves.fortitude,
        snapshot.base_saves.reflex,
        snapshot.base_saves.will,
        snapshot.combat.baseline_melee_attack_bonus,
        snapshot.defense.baseline_armor_class,
        snapshot.defense.total_save.fortitude,
        snapshot.defense.total_save.reflex,
        snapshot.defense.total_save.will,
        // `damage_reduction` is `Option`; absence and a real 0 are different
        // states and are encoded as different numbers rather than collapsed.
        snapshot.defense.damage_reduction.map_or(i16::MIN, |dr| dr),
    ]
}

/// True when `explanation_id` names `class_name` in one of its own
/// dot-separated segments.
///
/// Segment equality, never `contains`. `class_chassis.unchained_barbarian.x`
/// contains the substring `barbarian` while belonging to a different class
/// entirely, and a substring test would credit Barbarian with Unchained
/// Barbarian's magnitude -- the corpus-identifier scope collision this program
/// has already recorded.
fn explanation_names_class(explanation_id: &str, class_name: &str) -> bool {
    explanation_id.split('.').any(|segment| segment == class_name)
}

/// Every explanation id a modelled class produces at `level`, or `None` when
/// the pipeline panicked for it.
fn class_explanation_ids_at(
    fixture: &CharacterInput,
    class_name: &str,
    level: u8,
) -> Option<BTreeSet<String>> {
    let input = class_probe_input(fixture, class_name, level);
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        compute_pilot_base_chassis(&input)
    }));
    std::panic::set_hook(previous_hook);
    outcome.ok().map(|c| c.explanations.iter().map(|e| e.id.clone()).collect())
}

/// Whether selecting exactly this class, on a real creatable character,
/// produces a magnitude attributable to this class alone on the snapshot the
/// character sheet renders.
///
/// `modelled` carries every class the engine models, so the attribution half of
/// the delta is decided against the real engine rather than against an
/// assumption about how explanation ids are namespaced.
fn probe_class_name(
    fixture: &CharacterInput,
    class_name: &str,
    modelled: &BTreeSet<String>,
    baseline_numbers: Option<&Vec<i16>>,
) -> ClassProbeOutcome {
    if !modelled.contains(class_name) {
        return ClassProbeOutcome::NotModelledByEngine;
    }

    let mut any_level_ran = false;
    let mut any_level_computed = false;
    let mut any_snapshot = false;
    let mut any_snapshot_delta = false;

    for &level in CLASS_PROBE_LEVELS {
        let input = class_probe_input(fixture, class_name, level);
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            build_pilot_headless_receipt(&input)
        }));
        std::panic::set_hook(previous_hook);
        let Ok(receipt) = outcome else { continue };
        any_level_ran = true;
        if receipt.status != HeadlessReceiptStatus::Computed {
            continue;
        }
        any_level_computed = true;

        // The consumer surface, reached the way production reaches it.
        let view = PilotViewModel::from_receipt(&receipt);
        let Some(snapshot) = view.snapshot.as_ref() else { continue };
        any_snapshot = true;

        // Delta half one: the rendered numbers moved off the classless
        // baseline. A baseline that projects no snapshot at all is itself a
        // delta -- the same reasoning as the spell probe's "no DC at all".
        let numbers = class_snapshot_numbers(snapshot);
        if baseline_numbers.is_some_and(|b| *b == numbers) {
            continue;
        }
        any_snapshot_delta = true;

        // Delta half two: attribution. At least one explanation record naming
        // this class that NO other modelled class produces at this level.
        let mut others: BTreeSet<String> = BTreeSet::new();
        for other in modelled.iter().filter(|c| c.as_str() != class_name) {
            if let Some(ids) = class_explanation_ids_at(fixture, other, level) {
                others.extend(ids);
            }
        }
        let attributed = receipt
            .computation
            .explanations
            .iter()
            .filter(|e| explanation_names_class(&e.id, class_name) && !others.contains(&e.id))
            .count();
        if attributed > 0 {
            return ClassProbeOutcome::Wired { level, attributed_explanations: attributed };
        }
    }

    if !any_level_ran {
        ClassProbeOutcome::PipelinePanicked
    } else if !any_level_computed {
        ClassProbeOutcome::NeverReachesComputed
    } else if !any_snapshot {
        ClassProbeOutcome::NoSnapshotProjected
    } else if !any_snapshot_delta {
        ClassProbeOutcome::NoSnapshotDeltaVsClasslessBaseline
    } else {
        ClassProbeOutcome::NoExplanationAttributedToThisClass
    }
}

/// The classless baseline's rendered numbers, or `None` when it projects no
/// snapshot at all (which is itself the strongest possible baseline).
fn class_probe_baseline_numbers(fixture: &CharacterInput) -> Option<Vec<i16>> {
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let baseline = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        build_pilot_headless_receipt(&classless_probe_input(fixture))
    }));
    std::panic::set_hook(previous_hook);
    baseline
        .ok()
        .and_then(|r| PilotViewModel::from_receipt(&r).snapshot.as_ref().map(class_snapshot_numbers))
}

/// Runs [`probe_class_name`] over every class the engine models.
///
/// Returns the full outcome per class rather than only the wired set, because
/// the ceiling report needs the refusals and their reasons.
fn probe_class_effect_wiring(
    fixture: &CharacterInput,
    modelled: &BTreeSet<String>,
) -> BTreeMap<String, ClassProbeOutcome> {
    let baseline_numbers = class_probe_baseline_numbers(fixture);
    modelled
        .iter()
        .map(|class_name| {
            let outcome =
                probe_class_name(fixture, class_name, modelled, baseline_numbers.as_ref());
            (class_name.clone(), outcome)
        })
        .collect()
}

/// The class names [`classify`] may ground: exactly the probe's
/// [`ClassProbeOutcome::Wired`] verdicts, and nothing else.
///
/// The match is exhaustive and deliberately not a `_ =>` catch-all: a future
/// outcome variant must be classified as promoting or refusing by hand.
fn class_effect_wired_from_outcomes(
    outcomes: &BTreeMap<String, ClassProbeOutcome>,
) -> BTreeSet<String> {
    outcomes
        .iter()
        .filter(|(_, outcome)| match outcome {
            ClassProbeOutcome::Wired { .. } => true,
            ClassProbeOutcome::NotModelledByEngine
            | ClassProbeOutcome::PipelinePanicked
            | ClassProbeOutcome::NeverReachesComputed
            | ClassProbeOutcome::NoSnapshotProjected
            | ClassProbeOutcome::NoSnapshotDeltaVsClasslessBaseline
            | ClassProbeOutcome::NoExplanationAttributedToThisClass => false,
        })
        .map(|(name, _)| name.clone())
        .collect()
}

/// The probe's ceiling, printed by `--class-probe`: which modelled classes it
/// legitimately reaches and, for every one it does not, the reason it refused.
/// Grounding no unit, moving no number -- the instrument reporting on itself.
fn class_probe_ceiling_report(outcomes: &BTreeMap<String, ClassProbeOutcome>) -> String {
    let mut totals: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut out = String::new();
    out.push_str("class consumer-delta probe -- ceiling report\n");
    out.push_str(&format!("modelled classes examined: {}\n\n", outcomes.len()));
    for (name, outcome) in outcomes {
        let label = match outcome {
            ClassProbeOutcome::Wired { .. } => "wired",
            ClassProbeOutcome::NotModelledByEngine => "not_modelled_by_engine",
            ClassProbeOutcome::PipelinePanicked => "pipeline_panicked",
            ClassProbeOutcome::NeverReachesComputed => "never_reaches_computed",
            ClassProbeOutcome::NoSnapshotProjected => "no_snapshot_projected",
            ClassProbeOutcome::NoSnapshotDeltaVsClasslessBaseline => {
                "no_snapshot_delta_vs_classless_baseline"
            }
            ClassProbeOutcome::NoExplanationAttributedToThisClass => {
                "no_explanation_attributed_to_this_class"
            }
        };
        *totals.entry(label).or_default() += 1;
        match outcome {
            ClassProbeOutcome::Wired { level, attributed_explanations } => out.push_str(&format!(
                "  {name}: wired (level {level}, {attributed_explanations} attributed explanations)\n"
            )),
            _ => out.push_str(&format!("  {name}: {label}\n")),
        }
    }
    out.push_str("\nTOTAL\n");
    for (label, n) in &totals {
        out.push_str(&format!("  {label}: {n}\n"));
    }
    out
}

fn crb_class_name(class_id: ClassId) -> &'static str {
    match class_id {
        ClassId::Barbarian => "barbarian",
        ClassId::Bard => "bard",
        ClassId::Cleric => "cleric",
        ClassId::Druid => "druid",
        ClassId::Fighter => "fighter",
        ClassId::Monk => "monk",
        ClassId::Paladin => "paladin",
        ClassId::Ranger => "ranger",
        ClassId::Rogue => "rogue",
        ClassId::Sorcerer => "sorcerer",
        ClassId::Wizard => "wizard",
    }
}

/// The modelled race a `race_trait` record belongs to, or `None` when the
/// record names no race the engine models.
///
/// A race trait's corpus key names its own race in a `~`-qualifier ahead of
/// the trait name: `Blue ~ Keen Senses` -> `Blue`, and ARG's heritage form
/// `Saltbeard ~ Dwarf ~ Greed` -> `Dwarf`. Grounding must be keyed on THAT
/// race and never on "any race the engine models".
///
/// This is the fix for the name-coincidence defect
/// (`docs/release/corpus-work-channels.md` §9.3, SD-28 §56). `race_trait_ids`
/// is built solely from CRB's hardcoded `race_traits()` table, so the previous
/// rule -- pair the record's trait slug with *every* name in `race_names` and
/// ground on any hit -- let a non-CRB record reach `grounded` by coincidental
/// NAME match alone. Ultimate Psionics' `Blue ~ Keen Senses` scored off the
/// Elf's `elf.keen_senses`; `DuergarDSP ~ Hardy`, `DuergarDSP ~ Stability` and
/// `Forgeborn ~ Fearless` did the same. None of those four races is modelled at
/// all, so none of their traits is grounded by anything.
///
/// The TRAILING segment is the trait name, never the race, and is excluded
/// from the search — otherwise a trait whose name happens to equal a race name
/// would nominate itself. A key with no `~` separator names no race.
fn modelled_race_of_race_trait<'a>(
    key: &str,
    race_names: &'a BTreeSet<String>,
) -> Option<&'a String> {
    let segments: Vec<&str> = key.split(" ~ ").collect();
    segments[..segments.len().saturating_sub(1)].iter().find_map(|segment| {
        let segment = segment.trim().to_lowercase();
        race_names.iter().find(|race| **race == segment)
    })
}

fn race_name(race: RaceId) -> &'static str {
    match race {
        RaceId::Human => "human",
        RaceId::Dwarf => "dwarf",
        RaceId::Elf => "elf",
        RaceId::Gnome => "gnome",
        RaceId::HalfElf => "half-elf",
        RaceId::HalfOrc => "half-orc",
        RaceId::Halfling => "halfling",
    }
}

fn gather_engine_facts(
    fixture: &CharacterInput,
    corpus_class_names: BTreeSet<String>,
    repo_root: &Path,
) -> EngineFacts {
    let mut feat_keys: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();
    for table in all_feat_tables() {
        let book = rule_set_id(table.rule_set);
        let set = feat_keys.entry(book).or_default();
        for entry in table.entries {
            set.insert(entry.key.to_string());
        }
    }

    // SD-29 Epic 4 (spell lane): this map used to be three hand-maintained
    // `.insert()` calls (core_rulebook/apg/acg only) sitting beside
    // `spell_catalog::build_spell_catalog`, which already chained FIVE books
    // (adding ARG and UI). The two never being reconciled silently
    // misreported every already-shipping ARG and UI spell as
    // `not-ingested` -- Decision 36's pattern, the exact defect the
    // `equipment_keys` map immediately below this one was rebuilt to close
    // for equipment in SD-28-E15, reproduced one record family over.
    // Derived directly from `spell_resolver::spell_catalog_rows()` now, so
    // there is no second list left to diverge: adding a sixth book to the
    // registry populates this map automatically, and an unmapped
    // `SPELL_BOOK_*` code panics loudly here rather than silently vanishing
    // (see `spell_book_slug_for` and its own test below).
    let mut spell_levels: BTreeMap<&'static str, BTreeMap<String, bool>> = BTreeMap::new();
    for row in spell_resolver::spell_catalog_rows() {
        spell_levels
            .entry(spell_book_slug_for(row.book))
            .or_default()
            .insert(row.key.to_string(), row.level.is_some());
    }

    // SD-28-E15: this map used to be four hand-maintained `.insert()` calls
    // (core_rulebook/apg/acg/bestiary_1 only) sitting beside
    // `equipment_resolver::equipment_catalog_rows()`, which already chains
    // EIGHT books (adding arg/pu/ui/ue). The two never being reconciled
    // silently misreported ~1,650 already-landed UE/UI/PU units as
    // `not-ingested` -- Decision 36's pattern, at the largest scale this
    // program has found it. Derived directly from `equipment_catalog_rows()`
    // now, so there is no second list left to diverge: adding a ninth book
    // to the resolver populates this map automatically, and an unmapped
    // `EQUIPMENT_BOOK_*` code panics loudly here rather than silently
    // vanishing (see `equipment_book_slug_for` and its own test below).
    let mut equipment_keys: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();
    for row in equipment_resolver::equipment_catalog_rows() {
        equipment_keys
            .entry(equipment_book_slug_for(row.book))
            .or_default()
            .insert(row.key.to_string());
    }

    let monster_names: BTreeSet<String> = MonsterId::ALL
        .iter()
        .filter_map(|&id| beastiary1::monster_resolve(id, RuleSetId::Bestiary1))
        .map(|b| b.name.to_lowercase())
        .collect();

    // Registry-driven: every book in `monster_chassis::MONSTER_BOOKS` is
    // indexed here without being named. Adding a book to that registry is what
    // moves its `monster`/`monster_ability` units off `not-ingested`.
    let mut chassis_monster_keys: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();
    let mut chassis_monster_ability_keys: BTreeMap<&'static str, BTreeSet<String>> =
        BTreeMap::new();
    let mut chassis_monster_ability_unresolved_desc_keys: BTreeMap<&'static str, BTreeSet<String>> =
        BTreeMap::new();
    //
    // Keyed by the ENGINE book, translated from the registry's corpus
    // directory, exactly as `chassis_companion_keys` below is and for the same
    // reason: the `Kind::Monster` / `Kind::MonsterAbility` verdict arms have an
    // `engine_book` in hand (`rule_set_id`), never a corpus directory. For the
    // first nine registered monster books the two strings are identical, so a
    // raw `book.corpus_book` key worked by COINCIDENCE rather than by rule --
    // the same latent defect `decisions.md §54.3` records the companion lane
    // finding in its own copy of this loop. Bestiary 1 is where the coincidence
    // ends: its corpus directory is `beastiary`, its engine book is
    // `bestiary_1`, and an untranslated key would have reported all 607 of its
    // chassis records as `not-ingested` while the registry held them.
    for book in monster_chassis::MONSTER_BOOKS {
        let engine_book = engine_book_for_corpus_dir(book.corpus_book).unwrap_or_else(|| {
            panic!(
                "monster book {:?} is registered in MONSTER_BOOKS but resolves to no rule \
                 set; add it to CORPUS_DIR_ALIASES or register its RuleSetId",
                book.corpus_book
            )
        });
        chassis_monster_keys.insert(
            engine_book,
            book.monsters.iter().map(|m| m.key.to_lowercase()).collect(),
        );
        chassis_monster_ability_keys.insert(
            engine_book,
            book.monster_abilities.iter().map(|a| a.key.to_lowercase()).collect(),
        );
        chassis_monster_ability_unresolved_desc_keys.insert(
            engine_book,
            book
                .monster_abilities
                .iter()
                .filter(|a| monster_ability_desc_leaks_unresolved_argument(a))
                .map(|a| a.key.to_lowercase())
                .collect(),
        );
    }

    // Same registry discipline for `companion`: `companion_chassis::COMPANION_BOOKS`
    // is iterated, never enumerated here. Both structural shapes go into one set
    // per book because `Kind::Companion` is one kind (see
    // `EngineFacts::chassis_companion_keys`).
    //
    // Keyed by the ENGINE book, translated from the registry's corpus
    // directory, never by the corpus directory itself. The lookup at the
    // `Kind::Companion` verdict arm has an `engine_book` in hand
    // (`rule_set_id`), and for the first seven registered companion books the
    // two strings happened to be identical, so a raw `book.corpus_book` key
    // worked by coincidence rather than by rule. Bestiary 1 is where the
    // coincidence ends: its corpus directory is `beastiary`, its engine book is
    // `bestiary_1`, and an untranslated key would have reported all 59 of its
    // grounded records as `companion_content_has_no_engine_table` — the
    // silent-under-report shape `decisions.md §44` already paid for once.
    // `engine_book_for_corpus_dir` is the existing translation, not a new one.
    let mut chassis_companion_keys: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();
    for book in companion_chassis::COMPANION_BOOKS {
        let mut keys: BTreeSet<String> =
            book.companions.iter().map(|c| c.key.to_lowercase()).collect();
        keys.extend(book.companion_abilities.iter().map(|a| a.key.to_lowercase()));
        let engine_book = engine_book_for_corpus_dir(book.corpus_book).unwrap_or_else(|| {
            panic!(
                "companion book {:?} is registered in COMPANION_BOOKS but resolves to no rule \
                 set; add it to CORPUS_DIR_ALIASES or register its RuleSetId",
                book.corpus_book
            )
        });
        chassis_companion_keys.insert(engine_book, keys);
    }

    let class_books = modelled_class_books();

    let race_names: BTreeSet<String> =
        RaceId::ALL.iter().map(|&r| race_name(r).to_string()).collect();
    let race_trait_ids: BTreeSet<String> = race_traits()
        .iter()
        .map(|t| format!("{}.{}", race_name(t.race_id), slug(t.trait_name)))
        .collect();
    let race_trait_probe = probe_race_trait_corpus(repo_root);

    // The class consumer-delta probe, over exactly the classes the engine
    // models. Runs BEFORE the union sweep below because it asks a different
    // question of the same postures: not "what ids exist anywhere across all
    // classes" but "which magnitude is attributable to THIS class alone".
    let modelled_classes: BTreeSet<String> = class_books.keys().cloned().collect();
    let class_effect_wired =
        class_effect_wired_from_outcomes(&probe_class_effect_wiring(fixture, &modelled_classes));

    // Sweep every modelled class at every SWEEP_LEVELS level through the REAL
    // compute pipeline and union what it says. A panic is caught rather than
    // allowed to abort the inventory: a class that crashes the pipeline still
    // has real corpus units that need reporting.
    let mut explanation_ids = BTreeSet::new();
    let mut diagnostics: BTreeMap<String, (String, bool)> = BTreeMap::new();
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for class_name in class_books.keys() {
        for &level in SWEEP_LEVELS {
            let input = class_sweep_input(fixture, class_name, level);
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                compute_pilot_base_chassis(&input)
            }));
            let Ok(computation) = outcome else { continue };
            for e in &computation.explanations {
                explanation_ids.insert(e.id.clone());
            }
            for d in &computation.diagnostics {
                diagnostics
                    .entry(d.id.clone())
                    .or_insert_with(|| (d.message.clone(), d.claim_blocking));
            }
        }
    }
    std::panic::set_hook(previous_hook);

    EngineFacts {
        feat_effect_wired: probe_feat_effect_wiring(fixture),
        equipment_effect_wired: probe_equipment_effect_wiring(repo_root),
        spell_effect_wired: spell_effect_wired_from_outcomes(&probe_spell_effect_wiring(
            fixture, repo_root,
        )),
        feat_keys,
        spell_levels,
        equipment_keys,
        monster_names,
        chassis_monster_keys,
        chassis_monster_ability_keys,
        chassis_monster_ability_unresolved_desc_keys,
        chassis_companion_keys,
        class_books,
        class_effect_wired,
        // Filled by `main` after corpus enumeration: the probe's key
        // population and sibling map are corpus facts, not engine facts.
        class_feature_effect_wired: BTreeMap::new(),
        race_names,
        race_trait_ids,
        race_trait_probe,
        explanation_ids,
        diagnostics,
        corpus_class_names,
        corpus_json_descriptions: load_corpus_json_descriptions(repo_root),
    }
}

/// Populates [`EngineFacts::corpus_json_descriptions`] -- see that field's
/// doc comment for the join key and why it exists.
///
/// Walks `data/corpus/<book>/equipment/**/*.json` and
/// `data/corpus/<book>/spell/**/*.json` for every book in
/// [`OBSERVABLE_BOOK_DIRS`] (the set `OPEN-ISSUES.md` row 70 quantified: 134
/// `equipment_modifier` + 112 `equipment` + 1 `spell` recoverable units, all
/// under those two content kinds). A record whose `source` carries no
/// `path`/`line` (e.g. `source.kind: "web_second_source"`, sourced from a
/// URL rather than a `.lst` row) contributes nothing here -- there is no
/// `.lst` coordinate to join a `CorpusUnit` against, so admitting it by name
/// alone would risk crediting the wrong record, exactly the `Celestial
/// Shield` hazard this file's book-scoping discipline already guards
/// against elsewhere.
fn load_corpus_json_descriptions(repo_root: &Path) -> BTreeMap<(String, usize, String), String> {
    let mut out: BTreeMap<(String, usize, String), String> = BTreeMap::new();
    for book_dir in OBSERVABLE_BOOK_DIRS {
        for content_kind in ["equipment", "spell"] {
            let root = repo_root.join("data/corpus").join(book_dir).join(content_kind);
            if !root.is_dir() {
                continue;
            }
            let mut stack = vec![root];
            while let Some(dir) = stack.pop() {
                let Ok(entries) = std::fs::read_dir(&dir) else { continue };
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                        continue;
                    }
                    if path.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    let Ok(text) = std::fs::read_to_string(&path) else { continue };
                    let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else {
                        continue;
                    };
                    let Some(description) =
                        value.pointer("/data/description").and_then(|v| v.as_str())
                    else {
                        continue;
                    };
                    if !is_real_description_value(description) {
                        continue;
                    }
                    let Some(src_path) = value.pointer("/source/path").and_then(|v| v.as_str())
                    else {
                        continue;
                    };
                    let Some(line) = value.pointer("/source/line").and_then(|v| v.as_u64())
                    else {
                        continue;
                    };
                    // `record_key` (the SOURCE's own identity token) over
                    // `data.key` (the wire-facing key, sometimes cleaned up):
                    // `token_closure_rows`/`unit.key` both resolve against the
                    // corpus row's raw `KEY:`, and `record_key` is the JSON's
                    // own copy of exactly that token.
                    let Some(key) = value
                        .pointer("/source/record_key")
                        .and_then(|v| v.as_str())
                        .or_else(|| value.pointer("/data/key").and_then(|v| v.as_str()))
                    else {
                        continue;
                    };
                    let basename = Path::new(src_path)
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    out.insert(
                        (basename, line as usize, key.to_string()),
                        description.trim().to_string(),
                    );
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Status
// ---------------------------------------------------------------------------

/// The status vocabulary, with exactly what proves each one. Emitted into the
/// JSON so a consumer never has to guess what a status means.
const STATUS_VOCABULARY: &[(&str, &str)] = &[
    (
        "grounded",
        "A real computed magnitude was OBSERVED reaching a consumer: a feat whose presence \
         changes what compute_pilot_base_chassis returns; a class/race that reaches a real \
         receipt; a class feature whose explanation id appears in a real computation; a monster \
         that resolves to a real stat block through monster_resolve.",
    ),
    (
        "literal-verified",
        "A `static` unit whose shipped `data/corpus` record was byte-compared, this run, against \
         the upstream corpus literal it cites, by `corpus_literal_sweep --json-out`, and the WHOLE \
         sweep came back CLEAN. Strictly stronger than `ingested-magnitude`/`grounded`/ \
         `text-complete`, which it supersedes for a unit the sweep actually reached: only the \
         producer's `static`/`derived` doneness rung (operator directive 2026-08-13) maps this to \
         `done`. A unit the sweep did not reach, or a sweep that found any mismatch anywhere, \
         leaves every unit at its ordinary status -- this word is never assigned on trust. \
         Meaningful only on a `static`/`derived` unit; if a later reclassification leaves it on \
         any other class the producer reads it as `held`, and the next regen re-derives the \
         status from the class.",
    ),
    (
        "fixture-verified",
        "A `derived` unit whose engine evaluator was run, this run, over the real corpus record \
         through `compute_equipment_effects` and matched a pinned, independently-derived fixture \
         value exactly, by `derived_evaluator_fixture_check --json-out`. Strictly stronger than \
         `ingested-magnitude`/`grounded`/`text-complete`, which it supersedes for a unit the \
         fixture actually covers: only the producer's `static`/`derived` doneness rung (operator \
         directive 2026-08-13) maps this to `done`. Coverage is 94 of 2,879 held `derived` units \
         by the fixture's own design, not a sample -- a unit outside that coverage, or one the \
         check ran and failed, keeps its ordinary status and stays `held`. Meaningful only on a \
         `static`/`derived` unit; if a later reclassification leaves it on any other class the \
         producer reads it as `held`, and the next regen re-derives the status from the class.",
    ),
    (
        "ingested-magnitude",
        "The engine holds the record WITH its real numeric fields, but this generator observes no \
         consumer delta for this kind (spells, equipment). Strictly weaker than `grounded` and \
         deliberately a separate word: calling it grounded would be the same over-claim this \
         inventory exists to prevent.",
    ),
    (
        "text-complete",
        "The engine holds the record, the corpus record carries NO magnitude token at all (so \
         there is no number to compute), AND its token closure carries a real, non-empty, \
         non-.CLEAR/.CLEARALL, non-PI-redacted DESC: value (so there is real prose to show a \
         player). Per Decision 7 (`docs/release/SD-31-corpus-closure-grind/decisions.md`) all \
         three are required; the third was unchecked before SD31-D7-PROSE-001 and 634 units were \
         found `done` on the live board with a null corpus description as a result.",
    ),
    (
        "deferred-with-reason",
        "A claim-blocking diagnostic the engine itself emits names this unit. `reason` is that \
         diagnostic's message VERBATIM and `reason_id` is its id -- never re-narrated.",
    ),
    (
        "not-ingested",
        "The book IS ingested but the engine holds no record matching this unit's identity. A \
         real gap inside a started book, distinct from a book nobody has begun.",
    ),
    (
        "not-started",
        "The book has no compiled rule set at all. Nothing about this unit has been attempted.",
    ),
    (
        "unknown",
        "Could not be classified. `reason` says why. An honest unknown beats a confident wrong \
         entry.",
    ),
];

/// One unit's resolved status.
struct Verdict {
    status: &'static str,
    /// What proved it, as a short machine-readable token.
    evidence: String,
    /// Verbatim engine text (a claim-blocking diagnostic) or an unknown reason.
    reason: Option<String>,
    /// The book the ENGINE files this unit under, when it differs from the
    /// corpus directory the record physically lives in.
    engine_book: Option<String>,
}

/// A unit's engine book: the corpus directory, mapped to the engine's id.
/// `bestiary` on disk is `bestiary_1` in the engine.
fn engine_book_for(book: &str) -> Option<&'static str> {
    rule_set_for(book).map(rule_set_id)
}

/// The class a class-feature corpus record belongs to, derived from its
/// `<Group> ~ <Feature>` key. The group is the longest name in `classes` that
/// appears as a whole word at the start or end of it — `"Sorcerer Bloodline
/// Feat ~ X"` resolves to sorcerer, `"Domain Power ~ X"` to nothing. A record
/// whose group names no class falls out and is reported honestly rather than
/// guessed at.
fn class_feature_owner<'a, I: Iterator<Item = &'a String>>(key: &str, classes: I) -> Option<String> {
    let group = key.split(" ~ ").next().unwrap_or(key).to_lowercase();
    let mut best: Option<String> = None;
    for class in classes {
        let matches = group == *class
            || group.starts_with(&format!("{class} "))
            || group.ends_with(&format!(" {class}"));
        if matches && best.as_ref().map(|b| class.len() > b.len()).unwrap_or(true) {
            best = Some(class.clone());
        }
    }
    best
}

/// Resolve one corpus unit against the engine.
///
/// `carries_prose_magnitude` is the caller's own `wiring_class` verdict for
/// this same unit's token closure, narrowed to the two reasons
/// (`prose_expr`, `prose_formula_segment`) that mean "a real, non-guard,
/// non-cross-reference formula was found in prose" -- the %N-placeholder
/// pattern `99efb504` taught `wiring_class::determine_closure` but never
/// wired into this function's own, independent `text_only` signal. Before
/// that gap was closed here, a record like Zomok's Breath Weapon
/// (`DC %1...|CON+18`) could be `wiring_class: derived` (a real formula
/// exists) while simultaneously `status: text-complete` (`status_vocabulary`
/// promises that status ONLY when "the corpus record carries NO magnitude
/// token at all") -- a live contradiction of this file's own contract, and
/// the mechanism behind the classifier-quality/dashboard-score
/// anti-correlation the 2026-08-14 incentive-fix investigation traced to
/// this function. See that investigation's retro event for the corpus lines
/// (Rejuvenate Eidolon's `3d10+min(CASTERLEVEL,10)`, Telepathy Tap's
/// `10 + 1/2 your racial HD + your Charisma modifier`, Quarterstaff
/// (Hurricane)'s `.MOD` row `DC %1.|12+WIS`) that proved the pattern
/// genuine rather than a classifier false positive.
///
/// `has_real_description` (SD31-D7-PROSE-001, Decision 7's condition 3) is
/// whether this unit's own token closure carries a real, non-empty, non-
/// `.CLEAR`/`.CLEARALL`, non-PI-marker `DESC:` value anywhere -- computed by
/// the caller from the SAME `row_refs` closure `carries_prose_magnitude`'s
/// `wc_reason` is derived from, via [`closure_has_real_description`], so
/// this function never re-reads the corpus itself. Before this parameter
/// existed, `text_only` alone (zero `MAGNITUDE_TOKENS` fields and no prose
/// formula) was sufficient to grant `text-complete` -- Decision 7 requires
/// the description to be POPULATED from the real corpus row as a THIRD,
/// separate condition, and a corpus-wide re-derivation the day this
/// parameter was added found 634-1,060 units already `done` on the live
/// board with a `null` corpus `description` (`OPEN-ISSUES.md` row 71,
/// `progress.md`'s `SD31-D7-PROSE-001` receipt) -- a record merely existing
/// in a catalog is not the same fact as its prose reaching a player, and
/// Decision 7 says so explicitly.
///
/// `(engine_book, key)` pairs row 69's own hand-verification already
/// confirmed carry a FLAT (non-scaling) numeric printed only in prose within
/// `Kind::MonsterAbility` -- see the `monster_ability` rung's own call site
/// for the open interpretive question this is the conservative default for.
/// **A ONE-LINE change** (clear this list, once the operator's ruling
/// lands): reading (a) ("no numeric value at all") keeps it as-is; reading
/// (b) ("no character-specific scaling formula") clears it.
const MONSTER_ABILITY_FLAT_MAGNITUDE_PENDING_RULING: &[(&str, &str)] =
    &[("bestiary_2", "Devilfish ~ Water Dependency")];

fn monster_ability_flat_magnitude_pending_ruling(engine_book: &str, key: &str) -> bool {
    MONSTER_ABILITY_FLAT_MAGNITUDE_PENDING_RULING
        .iter()
        .any(|&(book, k)| book == engine_book && k == key)
}

/// CONFIRMED finding (integration-cycle adversarial review, `SD31-W6-
/// INTEGRATE-001`): 20 of the 947 `monster_ability` units the SD31-D7-
/// PROSE-002 rung promoted sit on corpus rows that declare a CHARACTER-
/// SPECIFIC computed `DESC:` argument -- `13+Con`, `CONSCORE`,
/// `BreathWeaponDC`, `SR`, `Mythic_Rank`, etc, not flat constants -- and
/// `monster_catalog::serve_ability_description` calls
/// `render_pcgen_desc` with an EMPTY `PcgenDisplayValues`, which silently
/// DROPS any `%N` it cannot resolve (popping the introducing `+`/`-` and
/// collapsing whitespace). The player sees "The psicrystal has power
/// resistance ." with the number deleted -- a Decision 7 condition-2 AND
/// condition-3 failure, the exact "green code gate over a hole on the
/// screen" shape the doctrine exists to prevent.
///
/// Two distinct shapes, both caught: (a) `description_variables` is
/// non-empty (17 of the 20) -- the row itself declares an argument list;
/// (b) `description_variables` is EMPTY but the raw `DESC:` text still
/// contains a bare `%<digit>` (15 of the 20, union with (a) = 20) -- a
/// malformed citation whose argument tail is missing entirely, so
/// `render_pcgen_desc`'s `dropped_args` (which only records a NAMED
/// argument) stays empty even though the digit is still silently dropped
/// from the rendered text. Checking `description_variables` directly,
/// rather than `render_pcgen_desc(desc).dropped_args`, is what catches
/// shape (b): `dropped_args` alone would miss it.
fn monster_ability_desc_leaks_unresolved_argument(record: &monster_chassis::MonsterAbilityRecord) -> bool {
    if !record.description_variables.is_empty() {
        return true;
    }
    match record.description {
        Some(desc) => desc.as_bytes().windows(2).any(|w| w[0] == b'%' && w[1].is_ascii_digit()),
        None => false,
    }
}

fn classify(
    unit: &CorpusUnit,
    facts: &EngineFacts,
    book_included_by: &BTreeSet<String>,
    carries_prose_magnitude: bool,
    has_real_description: bool,
) -> Verdict {
    // A book with no compiled rule set has had nothing attempted -- unless it
    // is the shared library other books pull in, in which case the record's
    // real home is whichever ingested book includes it. The host is chosen by
    // asking each candidate's own tables whether they hold this key, so the
    // attribution is OBSERVED rather than picked arbitrarily; when no candidate
    // holds it the record is left unattributed rather than assigned to a host
    // at random.
    // `unit.source_book`, deliberately, not `unit.book`: this resolves
    // which REAL engine consumer table serves this content (e.g.
    // `RuleSetId::Ce`'s `companion_chassis::COMPANION_BOOKS["core_essentials"]`
    // for companion/familiar content this engine has always served under
    // that id, regardless of which real-world book a given row's text
    // originates from). `unit.book` is the TRUE reporting attribution
    // (`SD31-ATTRIB-001`) and may now name a book with no such table at all
    // -- using it here silently downgraded 16 already-`grounded` companion
    // units to `not-ingested` (`companion_absent_from_bestiary_1_companion_tables`)
    // the first time this fix was measured, because Bestiary 1 genuinely
    // has no companion table of its own; the content was never anything but
    // `core_essentials`-served. `source_book` is always the book
    // `enumerate_book` actually walked, so this lookup is byte-identical to
    // this function's own pre-fix behaviour.
    let own_engine_book = engine_book_for(&unit.source_book);
    let engine_book = match own_engine_book {
        Some(b) => b.to_string(),
        None => {
            let hosts: Vec<&'static str> =
                book_included_by.iter().filter_map(|b| engine_book_for(b)).collect();
            if hosts.is_empty() {
                return Verdict {
                    status: "not-started",
                    evidence: "no_compiled_rule_set_for_book".to_string(),
                    reason: None,
                    engine_book: None,
                };
            }
            // A shared-library record is credited to a host only when that
            // host's OWN table is observed to hold it. Crediting it to an
            // arbitrary host would put thousands of un-ingested shared records
            // into some ingested book's reconciliation and make that book look
            // far further behind than it is.
            match hosts.iter().find(|b| facts.holds_unit(b, unit)) {
                Some(b) => b.to_string(),
                None => {
                    return Verdict {
                        status: "not-ingested",
                        evidence: "shared_library_record_held_by_no_ingested_host".to_string(),
                        reason: None,
                        engine_book: None,
                    };
                }
            }
        }
    };
    let engine_book_field = if own_engine_book.is_some() {
        None
    } else {
        Some(engine_book.clone())
    };

    // `magnitude_token_count` only counts `MAGNITUDE_TOKENS`-prefixed tab
    // fields (BONUS:, DEFINE:, ...). A record can carry zero of those and
    // still state a real, computable magnitude in prose -- see this
    // function's doc comment.
    let text_only = unit.magnitude_token_count == 0 && !carries_prose_magnitude;
    let not_ingested = |evidence: &str| Verdict {
        status: "not-ingested",
        evidence: evidence.to_string(),
        reason: None,
        engine_book: engine_book_field.clone(),
    };
    // Same verdict, for the registry-driven arms whose evidence token names the
    // book that answered and so cannot be a `&'static str`.
    let not_ingested_owned = |evidence: String| Verdict {
        status: "not-ingested",
        evidence,
        reason: None,
        engine_book: engine_book_field.clone(),
    };

    match unit.kind {
        Kind::Feat => {
            let known = facts
                .feat_keys
                .get(engine_book.as_str())
                .map(|s| s.contains(&unit.key) || s.contains(&unit.name))
                .unwrap_or(false);
            if !known {
                return not_ingested("feat_key_absent_from_catalog");
            }
            if facts.feat_effect_wired.contains(&unit.key)
                || facts.feat_effect_wired.contains(&unit.name)
            {
                return Verdict {
                    status: "grounded",
                    evidence: "feat_effect_probe_observed_computed_delta".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            // SD28-E13: the only book with a feat-scoped engine diagnostic
            // today. `ultimate_campaign::feat_tables::DEFERRED_WITH_REASON`
            // names the 2 UCA feats whose own corpus `.MOD BENEFIT:` row is
            // a confirmed upstream splice (see that module's own doc
            // comment) -- quoted verbatim here rather than re-narrated, the
            // same rule `Kind::ClassFeature`'s diagnostic lookup above
            // follows.
            if engine_book.as_str() == "ultimate_campaign"
                && let Some((_, diagnostic)) = uca_feat_tables::DEFERRED_WITH_REASON
                    .iter()
                    .find(|(key, _)| *key == unit.key || *key == unit.name)
            {
                return Verdict {
                    status: "deferred-with-reason",
                    evidence: "engine_diagnostic:ultimate_campaign::feat_tables::DEFERRED_WITH_REASON"
                        .to_string(),
                    reason: Some((*diagnostic).to_string()),
                    engine_book: engine_book_field,
                };
            }
            if text_only && has_real_description {
                return Verdict {
                    status: "text-complete",
                    evidence: "in_catalog_and_corpus_record_carries_no_magnitude_token".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            if text_only {
                // Decision 7's condition 3: zero magnitude AND no real DESC:
                // text anywhere in the token closure is nothing to compute
                // AND nothing to show a player -- not the completion the
                // ruling describes. `unknown` (not `not-ingested`, the
                // record IS in the catalog) so `doneness_verdict` reads it
                // `unmeasurable`, never `done` or `held`.
                return Verdict {
                    status: "unknown",
                    evidence: "text_only_but_corpus_record_carries_no_description_to_show_a_player"
                        .to_string(),
                    reason: Some(
                        "the feat is in the engine's catalog and its corpus record carries no \
                         magnitude token, but its token closure also carries no real DESC: text \
                         -- there is nothing to compute and nothing to show a player, so this is \
                         not the zero-magnitude completion Decision 7 describes"
                            .to_string(),
                    ),
                    engine_book: engine_book_field,
                };
            }
            Verdict {
                status: "unknown",
                evidence: "in_catalog_with_corpus_magnitude_but_no_observed_consumer".to_string(),
                reason: Some(format!(
                    "corpus record carries {} magnitude token(s){} and the feat IS in the \
                     engine's catalog, but the feat-effect probe observed no computed delta \
                     across the swept postures. That is the probe's documented lower-bound \
                     behaviour: the effect may need a posture, an opponent or a combat action \
                     this engine does not model. Reported as unknown rather than deferred \
                     because no engine diagnostic is scoped to a feat, so there is no engine \
                     text to quote",
                    unit.magnitude_token_count,
                    if carries_prose_magnitude {
                        " and a prose-embedded formula (wiring_class: derived)"
                    } else {
                        ""
                    }
                )),
                engine_book: engine_book_field,
            }
        }
        Kind::Spell => {
            let table = facts.spell_levels.get(engine_book.as_str());
            let level_known = table.and_then(|t| {
                t.get(&unit.key).copied().or_else(|| t.get(&unit.name).copied())
            });
            match level_known {
                None => not_ingested("spell_key_absent_from_spell_list"),
                // SD28-E14-F1 recorded that this arm could not promote,
                // because no wired consumer read a spell's magnitude. That
                // finding is SUPERSEDED, not overruled: `epic-31-spell-wiring`
                // (2026-08-07) wired `spellbook::compute_spellbook_coverage`
                // into `pf1_adapter::resolve_unified_pilot_snapshot`, so a
                // spell's own level now reaches a number the character sheet
                // prints. See the block comment above `probe_spell_key`.
                //
                // `(engine_book, key)`, never a bare key: the probe observed
                // this DC against ONE book's corpus record, and only that
                // book's unit may claim it -- the `Celestial Shield`
                // discipline the equipment arm below already follows. It is
                // load-bearing here, because every per-school resolver stamps
                // `RuleSetId::Crb`.
                Some(true) => {
                    let observed = |candidate: &str| {
                        facts
                            .spell_effect_wired
                            .contains(&(engine_book.clone(), candidate.to_string()))
                    };
                    if observed(&unit.key) || observed(&unit.name) {
                        return Verdict {
                            status: "grounded",
                            evidence: "spell_effect_probe_observed_computed_delta".to_string(),
                            reason: None,
                            engine_book: engine_book_field,
                        };
                    }
                    Verdict {
                        status: "ingested-magnitude",
                        evidence: "spell_list_entry_with_resolved_level".to_string(),
                        reason: None,
                        engine_book: engine_book_field,
                    }
                }
                // Decision 7's condition 3: the evidence token's own name
                // ("...with_description...") used to ASSERT a description
                // rather than checking one exists -- `has_real_description`
                // makes that check real.
                Some(false) if has_real_description => Verdict {
                    status: "text-complete",
                    evidence: "spell_list_entry_with_description_but_no_corpus_level".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                },
                Some(false) => Verdict {
                    status: "unknown",
                    evidence: "spell_list_entry_with_no_corpus_level_and_no_description".to_string(),
                    reason: Some(
                        "the spell resolves in the engine's spell list but this book's corpus \
                         record carries neither a resolved level nor any real DESC: text -- \
                         nothing to compute and nothing to show a player"
                            .to_string(),
                    ),
                    engine_book: engine_book_field,
                },
            }
        }
        Kind::Equipment | Kind::EquipmentModifier => {
            let known = facts
                .equipment_keys
                .get(engine_book.as_str())
                .map(|s| s.contains(&unit.key) || s.contains(&unit.name))
                .unwrap_or(false);
            if !known {
                return not_ingested("equipment_key_absent_from_equipment_tables");
            }
            // CONFIRMED finding (integration-cycle adversarial review,
            // `SD31-W6-INTEGRATE-001`): refuse this promotion when the
            // recovered corpus description still carries an unresolved
            // PCGen substitution (e.g. `%CHOICE`) -- see
            // `corpus_json_description_leaks_pcgen_syntax`'s own doc
            // comment. The equipment render path has no leak guard of its
            // own, so a promotion here is the last chance to catch it.
            if text_only
                && has_real_description
                && !corpus_json_description_leaks_pcgen_syntax(
                    &facts.corpus_json_descriptions,
                    &unit.provenance.file,
                    unit.provenance.line,
                    &unit.key,
                )
            {
                return Verdict {
                    status: "text-complete",
                    evidence: "in_equipment_tables_and_corpus_record_carries_no_magnitude_token"
                        .to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            // `(engine_book, key)`, never a bare key: the probe observed this
            // delta on ONE book's corpus record, and only that book's unit may
            // claim it. See `probe_equipment_effect_wiring`'s doc comment for
            // the `Celestial Shield` case that proves a shared key is not a
            // shared item. Consulted even for a text_only-but-undescribed
            // record (below), so a text-and-magnitude-free item the engine
            // somehow still observes a delta for is not demoted underneath
            // its own real evidence.
            let observed = |candidate: &str| {
                facts
                    .equipment_effect_wired
                    .contains(&(engine_book.clone(), candidate.to_string()))
            };
            if observed(&unit.key) || observed(&unit.name) {
                return Verdict {
                    status: "grounded",
                    evidence: "equipment_effect_probe_observed_computed_delta".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            if text_only {
                // Decision 7's condition 3, re-derived 2026-08-16: 634 units
                // of this exact shape (magnitude_token_count==0, corpus
                // `description: null`) were already `done` on the live board
                // -- `chassis_only`/`.COPY` equipment rows with no cost, no
                // weight and no DESC: token anywhere in their closure.
                return Verdict {
                    status: "unknown",
                    evidence: "text_only_but_corpus_record_carries_no_description_to_show_a_player"
                        .to_string(),
                    reason: Some(
                        "the item is in the engine's equipment tables and its corpus record \
                         carries no magnitude token, but its token closure also carries no real \
                         DESC: text -- there is nothing to compute and nothing to show a player, \
                         so this is not the zero-magnitude completion Decision 7 describes"
                            .to_string(),
                    ),
                    engine_book: engine_book_field,
                };
            }
            Verdict {
                status: "ingested-magnitude",
                evidence: "equipment_table_entry_with_corpus_magnitude".to_string(),
                reason: None,
                engine_book: engine_book_field,
            }
        }
        // Every registered chassis book, by the registry rather than by name.
        // The evidence token still carries the book, so a receipt reader sees
        // which table answered.
        Kind::Monster if facts.chassis_monster_keys.contains_key(engine_book.as_str()) => {
            if facts.holds_key(&engine_book, &unit.kind, &unit.key, &unit.name) {
                return Verdict {
                    status: "grounded",
                    evidence: format!(
                        "{engine_book}_monster_resolve_returned_a_real_stat_block"
                    ),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            not_ingested_owned(format!("monster_absent_from_{engine_book}_monsters"))
        }
        Kind::MonsterAbility
            if facts.chassis_monster_ability_keys.contains_key(engine_book.as_str()) =>
        {
            if facts.holds_key(&engine_book, &unit.kind, &unit.key, &unit.name) {
                // SD31-D7-PROSE-002 (Decision 7's done-bar, extending
                // SD31-D7-PROSE-001's `race_trait` rung): a text_only
                // ability the chassis table holds is `grounded` -- true,
                // but `grounded` is capped at `held` for `display`
                // wiring_class (`doneness_verdict`'s documented disagreement
                // signal), and for a record with no magnitude to disagree
                // about, that cap is wrong. `monster_chassis.rs`'s own
                // module doc: "Only ability rows WITH an owner are
                // registered" -- so any key this table holds is already
                // shown under some monster's catalog entry
                // (`monster_catalog::serve_ability_description` ->
                // `MonsterAbilityDto.description` ->
                // `MonsterCatalogScreen.tsx`'s `ability.description`
                // paragraph, unconditionally, for every registered ability).
                // No new render path; only the promotion, gated on the SAME
                // `has_real_description` closure check (over the SAME
                // `DESC:` token `MonsterAbilityRecord::description` is
                // itself parsed from) every other kind's rung already uses.
                //
                // `!flat_magnitude_pending_ruling`: Decision 7's own open
                // interpretive question (`OPEN-ISSUES.md` rows 69/87,
                // unresolved -- does a FLAT, non-scaling numeric printed
                // only in prose satisfy condition 2's "nothing to compute"?
                // Not this cycle's call). Conservative default while it is
                // open: refuse the one unit row 69's own hand-verified
                // sample already confirmed carries exactly that shape within
                // THIS kind (`bestiary_2:monster_ability:
                // devilfish_water_dependency`, "1 hour"/"2 hours" printed in
                // its `DESC:`) even though it is otherwise text_only with a
                // real description.
                // CONFIRMED finding (integration-cycle adversarial review,
                // `SD31-W6-INTEGRATE-001`): refuse promotion when the
                // record's own row would leak an unresolved
                // character-specific description argument to the player's
                // screen -- see `monster_ability_desc_leaks_unresolved_
                // argument`'s own doc comment for the two shapes this
                // catches and why "grounded"/`held` (never a fabricated
                // number) is the correct fallback.
                if text_only
                    && has_real_description
                    && !monster_ability_flat_magnitude_pending_ruling(&engine_book, &unit.key)
                    && !facts.monster_ability_desc_leaks_unresolved_argument(
                        &engine_book,
                        &unit.key,
                        &unit.name,
                    )
                {
                    return Verdict {
                        status: "text-complete",
                        evidence:
                            "monster_ability_held_and_corpus_record_carries_real_description"
                                .to_string(),
                        reason: None,
                        engine_book: engine_book_field,
                    };
                }
                return Verdict {
                    status: "grounded",
                    evidence: format!(
                        "{engine_book}_monster_ability_resolve_returned_a_real_record"
                    ),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            not_ingested_owned(format!(
                "monster_ability_absent_from_{engine_book}_monster_abilities"
            ))
        }
        Kind::Monster => {
            if facts.monster_names.contains(&unit.name.to_lowercase()) {
                return Verdict {
                    status: "grounded",
                    evidence: "monster_resolve_returned_a_real_stat_block".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            not_ingested("monster_absent_from_MonsterId_ALL")
        }
        Kind::Race => {
            if facts.race_names.contains(&unit.name.to_lowercase()) {
                return Verdict {
                    status: "grounded",
                    evidence: "race_modelled_by_RaceId_ALL_and_reachable_in_a_real_receipt"
                        .to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            not_ingested("race_absent_from_RaceId_ALL")
        }
        Kind::RaceTrait => {
            // PRIMARY: the race corpus the app really loads applies this
            // record to a player. This overrules the CRB-table rule below
            // rather than supplementing it, because it observes the path the
            // player uses -- see `probe_race_trait_corpus` and SD-29
            // `decisions.md §43.5`. Order matters: nothing the old rule
            // grounded can be demoted, because every record it grounds is
            // also in the loaded corpus.
            //
            // **The observation grounds on its own; it is not additionally
            // required to agree with the unit's own book.** This used to read
            // `== Some(engine_book.as_str())`, which was indistinguishable
            // from the rule above for every book whose `.lst` rows are filed
            // under itself -- and silently wrong for `core_essentials`, the
            // one book whose rows are routinely filed under a *different*
            // book (`race_trait_engine_book`'s own doc comment says exactly
            // that). While `core_essentials` had no compiled rule set the
            // shared-library path above resolved `engine_book` to the real
            // host and the equality held. SD-29's race-trait lane round 4 gave
            // the book a rule set of its own, for the 64 heritage records that
            // genuinely belong to it, and **155 Core Rulebook and Bestiary 1
            // standard racial traits stored in that directory instantly
            // dropped from `grounded` to
            // `race_trait_record_loaded_but_never_applies`** -- an evidence
            // token asserting the opposite of what the probe had just
            // observed. Nothing about those records changed; only the book
            // they are stored in gained an id.
            //
            // The probe's answer is the attribution, so it is reported as
            // such: a record whose observed book differs from its own is
            // credited to the observed one, exactly as a shared-library record
            // was before its host book was named. (`decisions.md §49.3`.)
            if let Some(observed) = facts.race_trait_engine_book(unit) {
                let engine_book_for_verdict = if own_engine_book == Some(observed) {
                    engine_book_field.clone()
                } else {
                    Some(observed.to_string())
                };
                // SD31-D7-PROSE-001 (Decision 7's done-bar, condition 3): a
                // zero-magnitude record the race corpus applies is `grounded`
                // -- true, but `grounded` is not a done-eligible status for
                // `display` wiring class (`doneness_verdict` caps it at
                // `held`, because for a MAGNITUDE-bearing record `grounded`
                // can mean "the classifier missed a real formula" -- see
                // that function's own doc comment). For a `text_only` record
                // there is no formula to have missed, so the only remaining
                // question is condition 3: does real prose reach the player?
                // Answered by calling the EXACT function
                // `race_trait_picker::build_menu` calls to serve the real
                // Alternate Racial Traits screen, over the SAME loaded
                // corpus -- never re-implemented, never asserted.
                // SD31-W5-INTEGRATE-001: `!rendered.trim().is_empty()` alone
                // accepted the PI-redaction placeholder `[redacted PI]` --
                // non-empty, but not real prose (a player sees the literal
                // marker string, not the rulebook's text). Reuse the SAME
                // refusal `closure_has_real_description` already applies to
                // every other text_only->text-complete branch in this file,
                // rather than a second, looser bar just for race_trait.
                if text_only
                    && let Some(rendered) = facts.race_trait_rendered_description(unit)
                    && is_real_description_value(rendered)
                {
                    return Verdict {
                        status: "text-complete",
                        evidence:
                            "race_trait_applied_by_the_race_corpus_and_rendered_with_real_text"
                                .to_string(),
                        reason: None,
                        engine_book: engine_book_for_verdict,
                    };
                }
                return Verdict {
                    status: "grounded",
                    evidence: "race_trait_applied_by_the_race_corpus_the_app_loads".to_string(),
                    reason: None,
                    engine_book: engine_book_for_verdict,
                };
            }
            // FALLBACK: CRB's seven compiled races. Still consulted, because
            // it is a real second opinion for the one book whose race traits
            // are also a compiled table.
            let crb_table_grounds = modelled_race_of_race_trait(&unit.key, &facts.race_names)
                .is_some_and(|race| {
                    facts.race_trait_ids.contains(&format!("{race}.{}", slug(&unit.name)))
                });
            if crb_table_grounds {
                return Verdict {
                    status: "grounded",
                    evidence: "race_trait_record_grounded_by_race_traits".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            // The honest middle: the record IS ingested and IS loaded, and
            // still no selection a player can make brings it in. Distinct
            // from "the engine holds no record matching this unit" and
            // reported as its own evidence rather than collapsed into it.
            if facts.race_trait_was_loaded(unit) {
                return not_ingested("race_trait_record_loaded_but_never_applies");
            }
            if modelled_race_of_race_trait(&unit.key, &facts.race_names).is_some() {
                return not_ingested("race_trait_absent_from_race_traits");
            }
            not_ingested("race_trait_race_not_modelled")
        }
        Kind::Class => {
            let name = unit.name.to_lowercase();
            // The engine must model a class of this name at all. Unchanged:
            // a name no class enum carries is a class nothing has ingested.
            if !facts.class_books.contains_key(&name) {
                return not_ingested("class_absent_from_ClassId_ALL_and_book_class_id_enums");
            }
            // PRIMARY, and the whole of the grounding decision: the class
            // consumer-delta probe OBSERVED this class put a magnitude
            // attributable to it alone on the snapshot the character sheet
            // renders. Strictly stricter than the membership test this
            // replaced -- see the probe's own section comment for why the
            // membership test could not tell a modelled class from a deleted
            // one, and why this change can only confirm or demote.
            if facts.class_effect_wired.contains(&name) {
                return Verdict {
                    status: "grounded",
                    evidence: "class_probe_observed_computed_delta_on_the_rendered_snapshot"
                        .to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            // The honest middle, and a genuinely different fact from "no class
            // enum names this": the engine DOES model a class of this name and
            // the probe still observed no magnitude a player can see for it.
            // Reported as its own evidence rather than collapsed into the
            // absence above.
            not_ingested("class_modelled_but_no_observed_delta_on_the_rendered_snapshot")
        }
        Kind::ClassFeature => {
            // The option-pool consumer-delta observation, asked FIRST because
            // the branches below cannot reach these records at all: a pool
            // member's group prefix names no class, so `class_feature_owner`
            // fails and the record lands `unknown` however wired it is. Only
            // the book whose class the engine models may claim the key --
            // `class_feature_effect_wired` carries that book precisely so a
            // second book's same-named record cannot ride this observation.
            if facts.class_feature_effect_wired.get(&unit.key) == Some(&unit.book.as_str()) {
                return Verdict {
                    status: "grounded",
                    evidence: "class_feature_probe_observed_a_delta_attributable_to_this_record"
                        .to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            let group = unit.key.split(" ~ ").next().unwrap_or(&unit.key);
            let Some(owner) = class_feature_owner(&unit.key, facts.class_books.keys()) else {
                // The group names no class this engine models. Before calling
                // it unclassifiable, ask whether the CORPUS declares a class by
                // that name anywhere: if it does, this is a feature of a class
                // nobody has ingested yet, which is a real `not-ingested` gap
                // rather than a mystery.
                if let Some(corpus_class) =
                    class_feature_owner(&unit.key, facts.corpus_class_names.iter())
                {
                    return Verdict {
                        status: "not-ingested",
                        evidence: format!(
                            "class_feature_of_unmodelled_corpus_class:{}",
                            slug(&corpus_class)
                        ),
                        reason: None,
                        engine_book: engine_book_field,
                    };
                }
                // SD28-E15: `text-complete` requires the engine to HOLD the
                // record (status_vocabulary's own definition), not merely
                // that the corpus record carries no magnitude token -- a
                // check first drafted here mirrored Kind::Feat's `text_only`
                // check without confirming that second precondition, and was
                // corrected before landing (decisions.md §40 amendment).
                // Direct search of rules_core and the desktop app found no
                // table, picker, or corpus JSON cache holding individual
                // option-pool records by name for any of the sampled
                // unowned groups (Rage Power, Discovery, ...) -- only a
                // handful of pools have a wired SLOT-COUNT mechanism (e.g.
                // barbarian_features::rage_powers_known), which counts how
                // many picks a character gets, never what any specific pick
                // is. Even pilot_compute.rs's own documented canonical
                // grounding example (`Discovery ~ Feral Mutagen`) does not
                // reach `grounded` through this code path. So for the
                // zero-magnitude subset sampled and confirmed here, the
                // engine genuinely holds none of these records -- a real,
                // correctly-reported gap, not text already served to a
                // player. Scoped deliberately to `text_only`: whether the
                // same "not held anywhere" finding generalizes to the
                // remaining magnitude>0 group-prefix units (856 distinct
                // pools, only a handful spot-checked) is a hypothesis, not
                // yet evidence at that scale -- left `unknown` below,
                // pending the per-group trace.
                if text_only {
                    return not_ingested("class_feature_option_pool_record_not_held_by_engine");
                }
                return Verdict {
                    status: "unknown",
                    evidence: "class_feature_group_names_no_class_at_all".to_string(),
                    reason: Some(format!(
                        "the record's `{group}` group prefix names neither a class this engine \
                         models nor any class the corpus declares (it is an option pool, an \
                         archetype, or a shared sub-choice set), so no explanation id can be \
                         derived for it without guessing"
                    )),
                    engine_book: engine_book_field,
                };
            };
            let feature = unit.key.split(" ~ ").nth(1).unwrap_or(&unit.name);
            let feature_slug = slug(feature);
            let grounded = facts
                .explanation_ids
                .iter()
                .any(|id| id.contains(&format!(".{owner}.")) && id.ends_with(&feature_slug));
            if grounded {
                return Verdict {
                    status: "grounded",
                    evidence: "explanation_id_observed_in_a_real_computation".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            // The engine's own diagnostics name the specific remaining gaps.
            // Quote one verbatim when it names this feature -- never re-narrate.
            let hit = facts.diagnostics.iter().find(|(id, _)| {
                id.contains(&format!(".{owner}.")) && id.contains(&feature_slug)
            });
            if let Some((id, (message, claim_blocking))) = hit {
                return Verdict {
                    status: "deferred-with-reason",
                    evidence: format!(
                        "engine_diagnostic:{id}:claim_blocking={claim_blocking}"
                    ),
                    reason: Some(message.clone()),
                    engine_book: engine_book_field,
                };
            }
            // SD28-E24 (decisions.md §42, completing §40's fix in its sibling
            // branch): a class-name substring match on `owner` is not a
            // holds-check. This branch used to grant `text-complete` to any
            // zero-magnitude record whose group prefix matched a modelled
            // class's name, without confirming any table or picker actually
            // serves that specific record -- the same defect §40 already
            // fixed a few lines above, in the "no owner" branch, for exactly
            // the same reason (`class_feature_option_pool_record_not_held_by_engine`).
            // Never fixed here until now. text_only alone is not sufficient;
            // absent a real holds-check (no generic class_feature catalog
            // exists anywhere in this engine, unlike feat/spell/equipment),
            // this branch can no longer manufacture `text-complete` either.
            if text_only {
                return not_ingested("class_feature_owner_matched_by_name_but_record_not_held_by_engine");
            }
            not_ingested("no_explanation_id_and_no_diagnostic_names_this_feature")
        }
        // SD-29 Epic 7 (companion lane). Registry-driven exactly as the two
        // monster arms above are: `companion_chassis::COMPANION_BOOKS` decides,
        // and the evidence token carries the book so a receipt reader sees which
        // table answered. A book with no registered companion table falls
        // through to the arm below, which keeps its original wording.
        Kind::Companion if facts.chassis_companion_keys.contains_key(engine_book.as_str()) => {
            if facts.holds_key(&engine_book, &unit.kind, &unit.key, &unit.name) {
                return Verdict {
                    status: "grounded",
                    evidence: format!("{engine_book}_companion_resolve_returned_a_real_record"),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            not_ingested_owned(format!("companion_absent_from_{engine_book}_companion_tables"))
        }
        Kind::Companion => not_ingested("companion_content_has_no_engine_table"),
        // SD28-E15 (2026-08-09): no engine table exists for monster
        // sub-abilities (natural attacks, special qualities/attacks,
        // universal monster rules) yet -- this kind is new precisely to
        // make that real, uningested population visible under its own
        // correct name, not to claim it is already reachable. Real content
        // in the wrong kind before this cycle; real content with no
        // engine table now, honestly reported as such.
        Kind::MonsterAbility => not_ingested("monster_ability_has_no_engine_table"),
    }
}

// ---------------------------------------------------------------------------
// Emission
// ---------------------------------------------------------------------------

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn q(s: &str) -> String {
    format!("\"{}\"", json_escape(s))
}

fn opt_q(s: &Option<String>) -> String {
    match s {
        Some(v) => q(v),
        None => "null".to_string(),
    }
}

fn real_now_iso8601() -> String {
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("system `date` must be available to stamp generated_at");
    String::from_utf8(output.stdout)
        .expect("date output is valid UTF-8")
        .trim()
        .to_string()
}

/// How many records the engine's own compiled tables hold, per (book, kind).
///
/// Emitted next to the corpus count as a `reconciliation` row so a mismatch
/// between "what the corpus declares" and "what the engine ingested" is a
/// visible number rather than something a reader has to notice. Every delta
/// this document reports is explained by a named trap rule.
fn engine_record_counts() -> BTreeMap<(&'static str, &'static str), usize> {
    use codex::rules_core::rules_tables::crb::equipment_tables::EquipmentCategory;

    let mut out: BTreeMap<(&'static str, &'static str), usize> = BTreeMap::new();
    for table in all_feat_tables() {
        out.insert((rule_set_id(table.rule_set), "feat"), table.entries.len());
    }
    out.insert(("core_rulebook", "spell"), crb_spell_list::SPELL_LIST.len());
    out.insert(
        ("advanced_players_guide", "spell"),
        apg::spell_list::SPELL_LIST.len(),
    );
    out.insert(("advanced_class_guide", "spell"), acg::spell_list::SPELL_LIST.len());

    let mut equipment_split = |book: &'static str, mods: usize, items: usize| {
        out.insert((book, "equipment"), items);
        out.insert((book, "equipment_modifier"), mods);
    };
    let crb = crb_equipment_tables::equipment_tables();
    let crb_mods = crb
        .iter()
        .filter(|e| e.category == EquipmentCategory::Equipmods)
        .count();
    equipment_split("core_rulebook", crb_mods, crb.len() - crb_mods);
    // Only the CRB table carries an `Equipmods` category at all: APG, ACG and
    // Bestiary 1 each declare their own `EquipmentCategory` enum with just
    // General/ArmsArmor/MagicItems. Their equipment-modifier record count is
    // therefore a real, engine-verified zero -- which is exactly what makes
    // the reconciliation row for their corpus `*_equipmods.lst` files show up
    // as an unstarted gap instead of silently vanishing.
    equipment_split(
        "advanced_players_guide",
        0,
        apg::equipment_tables::EQUIPMENT_TABLE.len(),
    );
    equipment_split(
        "advanced_class_guide",
        0,
        acg::equipment_tables::equipment_tables().len(),
    );
    equipment_split(
        "bestiary_1",
        0,
        beastiary1::equipment_tables::EQUIPMENT_TABLE.len(),
    );

    out.insert(("core_rulebook", "class"), ClassId::ALL.len());
    out.insert(("advanced_players_guide", "class"), ApgClassId::ALL.len());
    out.insert(("advanced_class_guide", "class"), AcgClassId::ALL.len());
    out.insert(("core_rulebook", "race"), RaceId::ALL.len());
    out.insert(("core_rulebook", "race_trait"), race_traits().len());
    out.insert(("bestiary_1", "monster"), MonsterId::ALL.len());
    out
}

/// One fully classified unit, ready to emit.
struct InventoryUnit {
    id: String,
    unit: CorpusUnit,
    verdict: Verdict,
    /// GE-01: what kind of evidence would prove this unit done, determined
    /// from the unit's token closure (`wiring_class::determine_closure`).
    /// Orthogonal to `verdict.status` -- never derived from it.
    wiring_class: wiring_class::WiringClass,
    wiring_class_reason: String,
    wiring_class_signals: BTreeSet<String>,
}

/// Reads the shared deterministic pilot input fixture, or exits with the
/// reason. Extracted from [`main`] so `--spell-probe` can run without also
/// requiring a `PCGEN_CORPUS_ROOT` checkout it does not read.
/// Parses `corpus_literal_sweep --json-out`'s report into the
/// `(book, source_file, source_line)` triples it verified.
///
/// Hand-rolled rather than pulling in a JSON parser for one file this binary
/// itself controls the shape of: `{"clean":<bool>,"records_examined":<n>,
/// "verified":[{"book":"...","source_file":"...","source_line":<n>},...]}`.
/// Returns an empty set on ANY read/parse failure or when `clean` is not
/// `true` -- a missing, stale, or dirty report must never be misread as
/// evidence. `clean:false` in particular is load-bearing: a sweep that found
/// a mismatch anywhere proves nothing about any individual record, so its
/// `verified` array (always empty on that branch, see `corpus_literal_sweep`)
/// is trusted precisely because this function refuses to trust anything else
/// in a non-clean report either.
fn load_sweep_verified(path: &Path) -> BTreeSet<(String, String, usize)> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return BTreeSet::new();
    };
    if !text.contains("\"clean\":true") {
        return BTreeSet::new();
    }
    let mut out = BTreeSet::new();
    let Some(list_start) = text.find("\"verified\":[") else {
        return BTreeSet::new();
    };
    let mut rest = &text[list_start + "\"verified\":[".len()..];
    while let Some(obj_start) = rest.find('{') {
        let Some(obj_end) = rest[obj_start..].find('}') else { break };
        let obj = &rest[obj_start..obj_start + obj_end];
        let book = json_field_str(obj, "book");
        let source_file = json_field_str(obj, "source_file");
        let source_line = json_field_usize(obj, "source_line");
        if let (Some(book), Some(source_file), Some(line)) = (book, source_file, source_line) {
            out.insert((book, source_file, line));
        }
        rest = &rest[obj_start + obj_end + 1..];
        if rest.trim_start().starts_with(']') {
            break;
        }
    }
    out
}

/// Parses `derived_evaluator_fixture_check --json-out`'s report into the
/// `unit_id`s it verified.
///
/// Shape: `{"fixtures_total":<n>,"cleared":<n>,"failed":<n>,
/// "not_ingested":<n>,"verified":["id1","id2",...]}`. Unlike the sweep's
/// report, there is no whole-report `clean` gate to check: this instrument's
/// coverage is deliberately partial by design (94 of 2,879 held `derived`
/// units), and a unit failing does not cast doubt on any other unit's
/// result the way one mismatched book does for the byte-equality sweep, so
/// the `verified` array alone -- built by `run_bar_check` from a per-unit
/// pass, never on trust -- is the whole of what this function needs.
fn load_derived_fixture_verified(path: &Path) -> BTreeSet<String> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return BTreeSet::new();
    };
    let mut out = BTreeSet::new();
    let Some(list_start) = text.find("\"verified\":[") else {
        return BTreeSet::new();
    };
    let mut rest = &text[list_start + "\"verified\":[".len()..];
    while let Some(quote_start) = rest.find('"') {
        let after_open = &rest[quote_start + 1..];
        let Some(quote_end) = after_open.find('"') else { break };
        let id = &after_open[..quote_end];
        out.insert(id.replace("\\\"", "\"").replace("\\\\", "\\"));
        rest = &after_open[quote_end + 1..];
        if rest.trim_start().starts_with(']') {
            break;
        }
    }
    out
}

/// Applies the `static`/`derived` done-rung stamps in place (operator
/// directive 2026-08-13, answering SD-32 decisions.md §2), extracted from
/// `main`'s two inline loops (launch-readiness remediation Step 4D) so the
/// invariant the dashboard producer's `held` mapping for unmapped
/// `(ambiguous, literal-/fixture-verified)` cells depends on -- that this
/// generator can NEVER emit those cells -- has a test next to the code that
/// makes it true, not just a comment claiming it.
///
/// `wiring_class == Static` items whose status is one of the three the
/// sweep's bar supersedes (`ingested-magnitude`/`grounded`/`text-complete`)
/// and whose own `(book, source_file, source_line)` triple is in
/// `sweep_verified` are upgraded to `literal-verified`. `wiring_class ==
/// Derived` items meeting the same status bar, joined on `id` against
/// `derived_fixture_verified`, are upgraded to `fixture-verified`. Every
/// other `wiring_class` -- `Display`, `Computed`, and, load-bearingly,
/// `Ambiguous` -- is left untouched by both loops regardless of whether its
/// `(book, file, line)`/`id` happens to appear in either verified set: the
/// `match` on `item.wiring_class` gates entry, so membership in a verified
/// set is necessary but never sufficient. A unit the sweep/fixture check did
/// not reach, or reached and failed, keeps its ordinary status and stays
/// `held`, same as before this rung existed.
fn apply_done_rung_stamps(
    inventory: &mut [InventoryUnit],
    sweep_verified: &BTreeSet<(String, String, usize)>,
    derived_fixture_verified: &BTreeSet<String>,
) {
    for item in inventory.iter_mut() {
        match item.wiring_class {
            wiring_class::WiringClass::Static => {
                // CONFIRMED cross-lane finding (`OPEN-ISSUES.md` row 104,
                // `SD31-E6-F7-001`): join on `source_book` (the PHYSICAL
                // book a `.lst` file lives under, matching
                // `corpus_literal_sweep`'s own `short_book_of` output), not
                // `book` (the re-attributed REPORTING field) -- the same
                // fix `engine_book_for`'s two call sites already apply.
                // Using `book` here silently strands every re-attributed
                // Static unit at `held` even when the sweep genuinely
                // verified its true citation.
                if matches!(
                    item.verdict.status,
                    "ingested-magnitude" | "grounded" | "text-complete"
                ) && sweep_verified.contains(&(
                    item.unit.source_book.clone(),
                    item.unit.provenance.file.clone(),
                    item.unit.provenance.line,
                )) {
                    item.verdict.status = "literal-verified";
                }
            }
            wiring_class::WiringClass::Derived => {
                if matches!(
                    item.verdict.status,
                    "ingested-magnitude" | "grounded" | "text-complete"
                ) && derived_fixture_verified.contains(&item.id)
                {
                    item.verdict.status = "fixture-verified";
                }
            }
            // Display, Computed, Ambiguous: never stamped, on purpose -- see
            // the function doc comment and
            // `ambiguous_display_computed_items_in_both_verified_sets_stay_unstamped`
            // below.
            wiring_class::WiringClass::Display
            | wiring_class::WiringClass::Computed
            | wiring_class::WiringClass::Ambiguous => {}
        }
    }
}

#[cfg(test)]
mod apply_done_rung_stamps_tests {
    use super::*;

    fn unit(id: &str, wc: wiring_class::WiringClass, status: &'static str, line: usize) -> InventoryUnit {
        InventoryUnit {
            id: id.to_string(),
            unit: CorpusUnit {
                book: "core_rulebook".to_string(),
                source_book: "core_rulebook".to_string(),
                kind: Kind::Feat,
                key: id.to_string(),
                name: id.to_string(),
                origin: Origin::Declared,
                provenance: Provenance { file: "test.lst".to_string(), line },
                magnitude_token_count: 0,
                type_facet: None,
                visible: true,
            },
            verdict: Verdict {
                status,
                evidence: "test".to_string(),
                reason: None,
                engine_book: None,
            },
            wiring_class: wc,
            wiring_class_reason: "test".to_string(),
            wiring_class_signals: BTreeSet::new(),
        }
    }

    /// The invariant the dashboard producer's `held` mapping for the
    /// unmapped `(ambiguous, literal-verified|fixture-verified)` cells
    /// (launch-readiness remediation Step 4D) depends on: this generator
    /// can NEVER emit those cells, because the stamping gate is
    /// `wiring_class`, not membership in either verified set. Proven by
    /// putting an Ambiguous item's own `(book, file, line)`/`id` in BOTH
    /// verified sets and confirming its status never moves -- same for
    /// Display and Computed, the two other non-`Static`/`Derived` classes.
    /// A Static control item in the same run IS stamped, proving the
    /// verified sets themselves are wired correctly (this isn't an
    /// empty-set false negative).
    #[test]
    fn ambiguous_display_computed_items_in_both_verified_sets_stay_unstamped() {
        let mut inventory = vec![
            unit("ambiguous_one", wiring_class::WiringClass::Ambiguous, "grounded", 1),
            unit("display_one", wiring_class::WiringClass::Display, "text-complete", 2),
            unit("computed_one", wiring_class::WiringClass::Computed, "grounded", 3),
            unit("static_control", wiring_class::WiringClass::Static, "grounded", 4),
        ];
        let sweep_verified: BTreeSet<(String, String, usize)> = inventory
            .iter()
            .map(|u| (u.unit.book.clone(), u.unit.provenance.file.clone(), u.unit.provenance.line))
            .collect();
        let derived_fixture_verified: BTreeSet<String> =
            inventory.iter().map(|u| u.id.clone()).collect();

        apply_done_rung_stamps(&mut inventory, &sweep_verified, &derived_fixture_verified);

        assert_eq!(inventory[0].verdict.status, "grounded", "Ambiguous must stay unstamped");
        assert_eq!(inventory[1].verdict.status, "text-complete", "Display must stay unstamped");
        assert_eq!(inventory[2].verdict.status, "grounded", "Computed must stay unstamped");
        assert_eq!(
            inventory[3].verdict.status, "literal-verified",
            "Static control must be stamped -- proves the verified sets are wired correctly"
        );
    }

    /// CONFIRMED cross-lane finding (`OPEN-ISSUES.md` row 104,
    /// `SD31-E6-F7-001`): `corpus_literal_sweep`'s own `short_book_of`
    /// resolves a re-attributed record's `(book, file, line)` triple using
    /// the PHYSICAL book a `.lst` file lives under (mirroring
    /// `CorpusUnit::source_book`'s documented contract), never the
    /// re-attributed REPORTING book (`CorpusUnit::book`) two sibling call
    /// sites (`engine_book_for`) already correctly join on. A unit whose
    /// `book` was re-attributed away from its `source_book` (the real
    /// `core_essentials`-housed `ce_*.lst` shape `SD31-ATTRIB-001` produces
    /// for `companion`/`monster_ability`/`race_trait`) must still stamp
    /// `literal-verified` when the sweep verifies its TRUE physical
    /// citation -- joining on the wrong field silently strands it at
    /// `held` forever, exactly the shape that blocked 34 real `companion`
    /// units this wave.
    #[test]
    fn static_stamp_joins_on_source_book_not_the_reattributed_reporting_book() {
        let mut reattributed = unit("companion_one", wiring_class::WiringClass::Static, "grounded", 9);
        reattributed.unit.book = "advanced_players_guide".to_string();
        reattributed.unit.source_book = "core_essentials".to_string();
        reattributed.unit.provenance.file = "ce_races_familiar_apg.lst".to_string();
        let mut inventory = vec![reattributed];

        // The sweep's own report keys on the PHYSICAL book, exactly as
        // `short_book_of` really produces it -- `source_book`, not `book`.
        let sweep_verified: BTreeSet<(String, String, usize)> =
            [("core_essentials".to_string(), "ce_races_familiar_apg.lst".to_string(), 9)].into_iter().collect();
        let derived_fixture_verified: BTreeSet<String> = BTreeSet::new();

        apply_done_rung_stamps(&mut inventory, &sweep_verified, &derived_fixture_verified);

        assert_eq!(
            inventory[0].verdict.status, "literal-verified",
            "a re-attributed Static unit must still stamp when the sweep verified its TRUE \
             physical (source_book, file, line) citation, not its reporting book"
        );
    }
}

/// The set of unit `id`s carrying a done-rung stamp (`literal-verified` or
/// `fixture-verified`) in a `work-inventory.json` document. Shared by the
/// regenerator's own stamp-loss guard (see [`stamp_loss`]) and its tests, so
/// the guard's notion of "stamped" can never drift from what it protects.
/// Returns an empty set on any parse failure -- an unreadable document proves
/// nothing about what it used to carry, and the guard below treats an empty
/// "previously stamped" set as "nothing to lose", never as an error, so a
/// malformed existing file cannot itself block a regeneration.
fn stamped_ids(inventory_json: &str) -> BTreeSet<String> {
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(inventory_json) else {
        return BTreeSet::new();
    };
    parsed["units"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|u| matches!(u["status"].as_str(), Some("literal-verified") | Some("fixture-verified")))
        .filter_map(|u| u["id"].as_str().map(|s| s.to_string()))
        .collect()
}

/// The stamp-loss guard's own decision (operator directive 2026-08-14,
/// hazard 1 of `SD-30-class-feature-archetype-bundle/state-goals-and-lessons.md`
/// §1.3): every unit id `existing_inventory_json` currently carries a
/// `literal-verified`/`fixture-verified` stamp for, that `incoming_stamped`
/// (the freshly computed run's own stamped set) does not reproduce. A plain
/// regen run with neither `CORPUS_LITERAL_SWEEP_REPORT` nor
/// `DERIVED_FIXTURE_CHECK_REPORT` set produces an empty `incoming_stamped`,
/// so every currently-stamped id comes back here -- exactly the silent-loss
/// hazard this function exists to make loud instead.
fn stamp_loss(existing_inventory_json: &str, incoming_stamped: &BTreeSet<String>) -> BTreeSet<String> {
    stamped_ids(existing_inventory_json).difference(incoming_stamped).cloned().collect()
}

fn json_field_str(obj: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\":\"");
    let start = obj.find(&needle)? + needle.len();
    let end = obj[start..].find('"')? + start;
    Some(obj[start..end].replace("\\\"", "\"").replace("\\\\", "\\"))
}

fn json_field_usize(obj: &str, key: &str) -> Option<usize> {
    let needle = format!("\"{key}\":");
    let start = obj.find(&needle)? + needle.len();
    let end = obj[start..].find(|c: char| !c.is_ascii_digit()).map(|e| start + e).unwrap_or(obj.len());
    obj[start..end].parse().ok()
}

fn load_probe_fixture(repo_root: &Path) -> CharacterInput {
    let fixture_path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let fixture_text = match std::fs::read_to_string(&fixture_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("could not read {}: {e}", fixture_path.display());
            std::process::exit(1);
        }
    };
    match load_character_input_fixture(&fixture_text).character_input {
        Some(fixture) => fixture,
        None => {
            eprintln!("fixture {} did not load", fixture_path.display());
            std::process::exit(1);
        }
    }
}

/// Every class this engine models, mapped to the book that models it. Shared
/// by `engine_facts` and the class_feature probe so the two can never disagree
/// about what "modelled" means.
fn modelled_class_books() -> BTreeMap<String, &'static str> {
    let mut class_books: BTreeMap<String, &'static str> = BTreeMap::new();
    for id in ClassId::ALL {
        class_books.insert(crb_class_name(*id).to_string(), "core_rulebook");
    }
    for id in ApgClassId::ALL {
        class_books.insert(id.name().to_string(), "advanced_players_guide");
    }
    for id in AcgClassId::ALL {
        class_books.insert(id.name().to_string(), "advanced_class_guide");
    }
    class_books
}

// ---------------------------------------------------------------------------
// class_feature consumer-delta probe
// ---------------------------------------------------------------------------
//
// Modelled on `probe_spell_key` above, and held to the same bar: a magnitude
// only counts when it is (a) observed on what a consumer actually renders and
// (b) *attributable to this record*, not to the mere fact that a slot was
// filled.
//
// Why class_feature needs a different shape from spell. `classify`'s
// `Kind::ClassFeature` arm grounds a feature when the engine's own real
// compute sweep emits an `explanation_id` for it. That sweep
// (`class_sweep_input`) builds a *base chassis*: the class's automatic
// features at each level, plus `canonical_seeds_for`'s defaults. A feature a
// player must PICK out of an option pool -- a rage power, a discovery, a rogue
// talent -- is never in that posture, so it can never emit an explanation
// there, and lands as `unknown` with `class_feature_group_names_no_class_at_all`.
//
// This probe asks the question that posture cannot: if a player DOES select
// this specific pool member, does any fact the sheet renders move?

/// The corpus group prefixes that name a real player-facing option pool, the
/// class that owns it, the engine choice slot that offers it, and the
/// `selection_id` NAMESPACE that slot's consumer recognizes.
///
/// The namespace matters and its omission is a probe defect, not a detail.
/// `choice_selection(input, "choice:cleric_domain")` is matched against ids
/// the engine writes as `domain:good`, never as bare `good`; a probe passing
/// the bare slug would be silently ignored by every namespaced consumer and
/// would then report `no_consumer_delta` for pools the engine genuinely does
/// compute per-record. An empty namespace means the consumer is open-ended
/// (it echoes whatever raw string it is given), which
/// `BARBARIAN_RAGE_POWER_SLOTS` documents for itself.
///
/// Both columns are asserted against the engine source by
/// `every_pool_names_a_choice_set_the_engine_source_declares` and
/// `every_namespaced_pool_uses_a_namespace_the_engine_source_writes`.
const CLASS_FEATURE_POOLS: &[(&str, &str, &str, &str)] = &[
    ("Rage Power", "barbarian", "choice:barbarian_rage_power", ""),
    ("Unchained Rage Power", "barbarian", "choice:barbarian_rage_power", ""),
    ("Discovery", "alchemist", "choice:alchemist_discovery", "discovery:"),
    ("Grand Discovery", "alchemist", "choice:alchemist_discovery", "discovery:"),
    ("Rogue Talent", "rogue", "choice:rogue_talent", "talent:"),
    ("Advanced Talents", "rogue", "choice:rogue_talent", "talent:"),
    ("Hex", "witch", "choice:witch_hex", "hex:"),
    ("Revelation", "oracle", "choice:oracle_revelation", "revelation:"),
    ("Mercy", "paladin", "choice:paladin_mercy", ""),
    ("Investigator Talent", "investigator", "choice:investigator_talent", "talent:"),
    ("Slayer Talent", "slayer", "choice:slayer_talent", "talent:"),
    ("Judgment", "inquisitor", "choice:inquisitor_judgment", "judgment:"),
    ("Inquisition", "inquisitor", "choice:inquisitor_domain", "domain:"),
    ("Blessing", "warpriest", "choice:warpriest_blessing", "blessing:"),
    ("Evolution", "summoner", "choice:summoner_eidolon_evolution", "evolution:"),
    ("Bloodline", "sorcerer", "choice:sorcerer_bloodline", "bloodline:"),
    ("Bloodrager Bloodline", "bloodrager", "choice:bloodrager_bloodline", "bloodline:"),
    ("Domain", "cleric", "choice:cleric_domain", "domain:"),
    ("Order", "cavalier", "choice:cavalier_order", "order:"),
    ("Mystery", "oracle", "choice:oracle_mystery", "mystery:"),
    ("Curse", "oracle", "choice:oracle_curse", "curse:"),
    ("Spirit", "shaman", "choice:shaman_spirit", "spirit:"),
    ("Animal Focus", "hunter", "choice:hunter_animal_focus", "animal_focus:"),
    ("Favored Enemy", "ranger", "choice:ranger_favored_enemy", "enemy:"),
    ("Favored Terrain", "ranger", "choice:ranger_favored_terrain", "terrain:"),
    ("Versatile Performance", "bard", "choice:bard_versatile_performance", ""),
    ("Arcane School", "wizard", "choice:wizard_school_specialization", "school:"),
    ("Focused Arcane School", "wizard", "choice:wizard_school_specialization", "school:"),
];

/// Every `class_feature` corpus key in the committed inventory, deduplicated.
/// The inventory is read rather than the PCGen corpus because the units this
/// probe is asked about are inventory units.
fn class_feature_keys_from_inventory(inventory_json: &str) -> Vec<String> {
    let parsed: serde_json::Value =
        serde_json::from_str(inventory_json).expect("work-inventory.json parses");
    let mut keys: BTreeSet<String> = BTreeSet::new();
    for unit in parsed["units"].as_array().into_iter().flatten() {
        if unit["kind"].as_str() != Some("class_feature") {
            continue;
        }
        if let Some(key) = unit["corpus_key"].as_str() {
            keys.insert(key.to_string());
        }
    }
    keys.into_iter().collect()
}

/// Group prefix -> every member name the corpus declares under it. The source
/// of each key's control member.
fn class_feature_siblings(keys: &[String]) -> BTreeMap<String, BTreeSet<String>> {
    let mut siblings: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for key in keys {
        let mut parts = key.split(" ~ ");
        let (Some(group), Some(member)) = (parts.next(), parts.next()) else { continue };
        siblings.entry(group.to_string()).or_default().insert(member.to_string());
    }
    siblings
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ClassFeatureProbeOutcome {
    /// Selecting this specific pool member moved a fact the sheet renders,
    /// AND a different real member of the same pool did not move it the same
    /// way. The delta is attributable to this record.
    Wired { moved: usize },
    /// The record's group prefix names no option pool this engine offers as a
    /// choice slot, so no player selection can reach it at all.
    NoChoiceSlotOffersIt,
    /// The pool's owning class is not modelled by this engine.
    OwnerClassNotModelled,
    /// The pool has only this one member in the corpus, so there is no sibling
    /// to control against. Never promoted: without a control, a delta cannot
    /// be told apart from "a slot got filled".
    NoSiblingToControlAgainst,
    /// The selection was accepted and changed nothing a consumer renders.
    NoConsumerDelta,
    /// A delta appeared, but a *different real member of the same pool*
    /// produced the identical delta -- the slot counts picks, it does not
    /// apply this record. Never promoted: this is the "attributable" half of
    /// consumer-delta, and it is the outcome `BARBARIAN_RAGE_POWER_SLOTS`'
    /// own "open-ended recognition (no power-list validation)" predicts.
    ///
    /// `shared` names the fact ids that moved identically for both members.
    /// Carried rather than discarded because these are exactly the units a
    /// probe WITHOUT a control would have promoted, and a reader deserves to
    /// see the number that was declined and what it was.
    DeltaNotAttributableToTheRecord { shared: Vec<String> },
}

/// The facts a class-feature selection is allowed to move: every explanation
/// the real compute pipeline emits, plus the twelve rendered numbers
/// `observable_facts` already pins. Sorted so two runs are comparable.
/// `None` when the pipeline panicked on this posture -- a panic is not a
/// delta, and must never be read as one.
fn class_feature_observable(input: &CharacterInput) -> Option<Vec<(String, i16)>> {
    let computation =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| compute_pilot_base_chassis(input)))
            .ok()?;
    let (explanations, numbers) = observable_facts(&computation);
    let mut facts = explanations;
    for (i, n) in numbers.iter().enumerate() {
        facts.push((format!("rendered_number.{i}"), *n));
    }
    facts.sort();
    Some(facts)
}

/// The whole verdict, isolated from any engine call so its three branches can
/// be pinned directly. `observed` is the character with THIS record selected;
/// `control` is the same character with a different real member of the same
/// pool selected instead.
fn classify_class_feature_delta(
    baseline: &[(String, i16)],
    observed: &[(String, i16)],
    control: &[(String, i16)],
) -> ClassFeatureProbeOutcome {
    if observed == baseline {
        return ClassFeatureProbeOutcome::NoConsumerDelta;
    }
    if observed == control {
        let shared = observed
            .iter()
            .filter(|f| !baseline.contains(f))
            .map(|(id, _)| id.clone())
            .collect();
        return ClassFeatureProbeOutcome::DeltaNotAttributableToTheRecord { shared };
    }
    let moved = observed.iter().filter(|f| !baseline.contains(f)).count();
    ClassFeatureProbeOutcome::Wired { moved }
}

/// The control member for `corpus_key`: any OTHER real member of the same
/// corpus group. Never a synthetic sentinel -- an invalid id the engine simply
/// rejects would make every open-ended slot look per-record.
fn class_feature_control_member<'a>(
    siblings: &'a BTreeMap<String, BTreeSet<String>>,
    group: &str,
    member: &str,
) -> Option<&'a str> {
    siblings.get(group)?.iter().map(String::as_str).find(|s| *s != member)
}

fn probe_class_feature_key(
    fixture: &CharacterInput,
    class_books: &BTreeMap<String, &'static str>,
    siblings: &BTreeMap<String, BTreeSet<String>>,
    corpus_key: &str,
) -> ClassFeatureProbeOutcome {
    let group = corpus_key.split(" ~ ").next().unwrap_or(corpus_key);
    let Some((_, owner, choice_set_id, namespace)) =
        CLASS_FEATURE_POOLS.iter().find(|(g, _, _, _)| *g == group)
    else {
        return ClassFeatureProbeOutcome::NoChoiceSlotOffersIt;
    };
    if !class_books.contains_key(*owner) {
        return ClassFeatureProbeOutcome::OwnerClassNotModelled;
    }
    let member = corpus_key.split(" ~ ").nth(1).unwrap_or(corpus_key);
    let Some(control_member) = class_feature_control_member(siblings, group, member) else {
        return ClassFeatureProbeOutcome::NoSiblingToControlAgainst;
    };

    let pick = |selection: &str| SelectedChoice {
        choice_set_id: (*choice_set_id).to_owned(),
        selection_id: format!("{namespace}{}", slug(selection)),
    };

    let mut verdict = ClassFeatureProbeOutcome::NoConsumerDelta;
    for &level in SWEEP_LEVELS {
        let mut base_input = class_sweep_input(fixture, owner, level);
        // `canonical_seeds_for` pre-fills several of these very slots
        // (`choice:cleric_domain -> domain:good`, `choice:witch_hex ->
        // hex:flight`, ...). Leaving a seed in place would make the BASELINE
        // already carry the pool's effect, so the record under test could
        // add nothing and every such pool would report `no_consumer_delta`
        // for the probe's own reason rather than the engine's. The slot under
        // test is emptied first, exactly as `probe_spell_key`'s baseline is
        // the same character with the spell NOT selected.
        base_input.chosen.selected_choices.retain(|c| c.choice_set_id != **choice_set_id);
        let Some(baseline) = class_feature_observable(&base_input) else { continue };

        let mut with_record = base_input.clone();
        with_record.chosen.selected_choices.push(pick(member));
        let Some(observed) = class_feature_observable(&with_record) else { continue };

        let mut with_control = base_input.clone();
        with_control.chosen.selected_choices.push(pick(control_member));
        let Some(control) = class_feature_observable(&with_control) else { continue };

        match classify_class_feature_delta(&baseline, &observed, &control) {
            wired @ ClassFeatureProbeOutcome::Wired { .. } => return wired,
            // A pick-counting slot at one level is the honest answer for the
            // whole key unless some other level genuinely applies the record.
            not_attributable @ ClassFeatureProbeOutcome::DeltaNotAttributableToTheRecord {
                ..
            } => {
                verdict = not_attributable;
            }
            _ => {}
        }
    }
    verdict
}

/// Runs the probe across every `class_feature` corpus key and keeps only the
/// `Wired` verdicts, each mapped to the book that models its pool's owning
/// class. The direct analogue of `spell_effect_wired_from_outcomes`.
fn probe_class_feature_effect_wiring(
    fixture: &CharacterInput,
    class_books: &BTreeMap<String, &'static str>,
    keys: &[String],
) -> BTreeMap<String, &'static str> {
    let siblings = class_feature_siblings(keys);
    let mut wired = BTreeMap::new();
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for key in keys {
        if !matches!(
            probe_class_feature_key(fixture, class_books, &siblings, key),
            ClassFeatureProbeOutcome::Wired { .. }
        ) {
            continue;
        }
        let group = key.split(" ~ ").next().unwrap_or(key);
        let Some((_, owner, _, _)) = CLASS_FEATURE_POOLS.iter().find(|(g, _, _, _)| *g == group)
        else {
            continue;
        };
        if let Some(book) = class_books.get(*owner) {
            wired.insert(key.clone(), *book);
        }
    }
    std::panic::set_hook(previous_hook);
    wired
}

/// The probe's ceiling, printed by `--class-feature-probe`. Grounds nothing
/// and moves no number on any board: this is the instrument reporting on
/// itself, exactly as `--spell-probe` does.
fn class_feature_probe_ceiling_report(
    outcomes: &BTreeMap<String, ClassFeatureProbeOutcome>,
) -> String {
    let mut totals: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut wired_keys: Vec<&str> = Vec::new();
    let mut declined_pools: BTreeMap<&str, usize> = BTreeMap::new();
    let mut declined_facts: BTreeMap<String, usize> = BTreeMap::new();
    for (key, outcome) in outcomes {
        let label = match outcome {
            ClassFeatureProbeOutcome::Wired { .. } => {
                wired_keys.push(key.as_str());
                "wired"
            }
            ClassFeatureProbeOutcome::NoChoiceSlotOffersIt => "no_choice_slot_offers_it",
            ClassFeatureProbeOutcome::OwnerClassNotModelled => "owner_class_not_modelled",
            ClassFeatureProbeOutcome::NoSiblingToControlAgainst => "no_sibling_to_control_against",
            ClassFeatureProbeOutcome::NoConsumerDelta => "no_consumer_delta",
            ClassFeatureProbeOutcome::DeltaNotAttributableToTheRecord { shared } => {
                *declined_pools
                    .entry(key.split(" ~ ").next().unwrap_or(key.as_str()))
                    .or_default() += 1;
                for id in shared {
                    *declined_facts.entry(id.clone()).or_default() += 1;
                }
                "delta_not_attributable_to_the_record"
            }
        };
        *totals.entry(label).or_default() += 1;
    }
    let mut out = String::new();
    out.push_str("class_feature consumer-delta probe -- ceiling report\n");
    out.push_str(&format!("keys examined: {}\n\n", outcomes.len()));
    for (label, n) in &totals {
        out.push_str(&format!("  {label:<40} {n}\n"));
    }
    out.push_str(&format!("\nwired keys: {}\n", wired_keys.len()));
    for key in wired_keys.iter().take(50) {
        out.push_str(&format!("  {key}\n"));
    }
    // The declined number, named. A probe without a control would have
    // promoted every one of these.
    out.push_str("\ndeclined as not-attributable, by pool:\n");
    let mut pools: Vec<_> = declined_pools.iter().collect();
    pools.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (pool, n) in pools {
        out.push_str(&format!("  {n:>5}  {pool}\n"));
    }
    out.push_str("\nthe facts those selections moved identically for both members:\n");
    let mut facts: Vec<_> = declined_facts.iter().collect();
    facts.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (id, n) in facts.iter().take(20) {
        out.push_str(&format!("  {n:>5}  {id}\n"));
    }
    out
}

fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let args: Vec<String> = std::env::args().collect();

    // The spell consumer-delta probe's own ceiling report. Reads only
    // `data/corpus/` and the engine's own tables, writes nothing, classifies
    // nothing, and moves no unit on any board -- it reports what the
    // instrument can and cannot reach. Deliberately an early return, before
    // the PCGen-corpus gate below, because it does not read that corpus.
    if args.iter().any(|a| a == "--spell-probe") {
        let fixture = load_probe_fixture(&repo_root);
        let outcomes = probe_spell_effect_wiring(&fixture, &repo_root);
        print!("{}", spell_probe_ceiling_report(&outcomes));
        return;
    }

    // The class consumer-delta probe's own ceiling report, on the same terms:
    // reads only the engine's own tables and the shared fixture, writes
    // nothing, classifies nothing, moves no unit.
    if args.iter().any(|a| a == "--class-probe") {
        let fixture = load_probe_fixture(&repo_root);
        let mut modelled: BTreeSet<String> = BTreeSet::new();
        for id in ClassId::ALL {
            modelled.insert(crb_class_name(*id).to_string());
        }
        for id in ApgClassId::ALL {
            modelled.insert(id.name().to_string());
        }
        for id in AcgClassId::ALL {
            modelled.insert(id.name().to_string());
        }
        let outcomes = probe_class_effect_wiring(&fixture, &modelled);
        print!("{}", class_probe_ceiling_report(&outcomes));
        return;
    }

    // The class_feature consumer-delta probe's own ceiling report. Same
    // contract as `--spell-probe`: writes nothing, classifies nothing, moves
    // no unit on any board. Its key population comes from the committed
    // inventory rather than the PCGen corpus, so it runs without a corpus
    // checkout -- the inventory is the artifact whose `class_feature` units
    // are the question.
    if args.iter().any(|a| a == "--class-feature-probe") {
        let fixture = load_probe_fixture(&repo_root);
        let class_books = modelled_class_books();
        let inventory = std::fs::read_to_string(repo_root.join(OUTPUT_RELATIVE_PATH))
            .expect("docs/work-inventory.json is readable");
        let keys = class_feature_keys_from_inventory(&inventory);
        let siblings = class_feature_siblings(&keys);
        let mut outcomes: BTreeMap<String, ClassFeatureProbeOutcome> = BTreeMap::new();
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        for key in &keys {
            let outcome = probe_class_feature_key(&fixture, &class_books, &siblings, key);
            outcomes.insert(key.clone(), outcome);
        }
        std::panic::set_hook(previous_hook);
        print!("{}", class_feature_probe_ceiling_report(&outcomes));
        return;
    }

    let summary_only = args.iter().any(|a| a == "--summary");
    // `--summary` never writes the file: a summary is not the artefact, and
    // overwriting the full inventory with one would be a silent data loss.
    let stdout_only = summary_only || args.iter().any(|a| a == "--stdout-only");

    // HOME-relative default: `workspace/` lives in the operator's home
    // directory and is synced across machines. `PCGEN_CORPUS_ROOT` still wins.
    let corpus_root = match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(configured) => PathBuf::from(configured),
        Err(_) => {
            let home = std::env::var("HOME")
                .expect("HOME must be set to locate the default PCGen corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    };
    let books_dir = corpus_root.join(BOOKS_RELATIVE);
    if !books_dir.is_dir() {
        eprintln!(
            "corpus not found at {} -- set PCGEN_CORPUS_ROOT to a PCGen data/ checkout",
            books_dir.display()
        );
        std::process::exit(1);
    }

    // The `static` done-rung evidence (operator directive 2026-08-13,
    // answering SD-32 decisions.md §2): `corpus_literal_sweep --json-out`'s
    // report of which shipped records it byte-compared clean, if a fresh one
    // has been generated. `CORPUS_LITERAL_SWEEP_REPORT` is opt-in and unset
    // by default, so an inventory generated without first running the sweep
    // carries no `literal-verified` units at all -- it never fabricates
    // evidence it was not handed. Only used for `wiring_class == Static`
    // below.
    let sweep_verified: BTreeSet<(String, String, usize)> =
        std::env::var("CORPUS_LITERAL_SWEEP_REPORT")
            .ok()
            .map(PathBuf::from)
            .map(|p| load_sweep_verified(&p))
            .unwrap_or_default();

    // The `derived` done-rung evidence, same operator directive:
    // `derived_evaluator_fixture_check --json-out`'s report of which
    // `unit_id`s the engine's evaluator actually matched against the pinned
    // fixture (coverage is 94 of 2,879 held `derived` units by the fixture's
    // own design -- see `tests/derived_evaluator_fixture_check.rs`'s module
    // doc; every unit not in this set keeps its ordinary status and stays
    // `held`). `DERIVED_FIXTURE_CHECK_REPORT` is opt-in and unset by
    // default, same reason as the static rung above. The report's
    // `verified` array carries `unit_id`s directly -- unlike the sweep's
    // triples, `Fixture::unit_id` is spelled identically to this
    // generator's own `InventoryUnit::id`, so the join is a direct set
    // membership test with no book/file/line reconstruction needed.
    let derived_fixture_verified: BTreeSet<String> = std::env::var("DERIVED_FIXTURE_CHECK_REPORT")
        .ok()
        .map(PathBuf::from)
        .map(|p| load_derived_fixture_verified(&p))
        .unwrap_or_default();

    let fixture = load_probe_fixture(&repo_root);

    // --- book roster, from the corpus itself ------------------------------
    // `book_paths` maps a book id (directory basename) to its real directory,
    // whether that directory lives under `roleplaying_game/` or is one of the
    // `EXTRA_BOOK_DIRS` roots elsewhere in the corpus (SD-28). Every later
    // lookup goes through this map so an extra book's real, different path is
    // never silently reconstructed as `books_dir.join(id)`.
    let mut book_paths: BTreeMap<String, PathBuf> = std::fs::read_dir(&books_dir)
        .expect("corpus books directory is readable")
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path()))
        .collect();
    for extra in EXTRA_BOOK_DIRS {
        let path = corpus_root.join(extra);
        if !path.is_dir() {
            eprintln!("extra book directory not found at {} -- EXTRA_BOOK_DIRS entry `{extra}` must exist", path.display());
            std::process::exit(1);
        }
        let id = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| panic!("EXTRA_BOOK_DIRS entry `{extra}` has no basename"));
        book_paths.insert(id, path);
    }
    let book_dirs: Vec<String> = book_paths.keys().cloned().collect();
    let known_books: BTreeSet<String> = book_paths.keys().cloned().collect();

    // Which books the repository has registered as future-state stubs, read
    // from `data/stubs/` rather than from a list in this file.
    let registered_stubs: BTreeSet<String> = std::fs::read_dir(repo_root.join("data/stubs"))
        .map(|entries| {
            entries
                .flatten()
                .filter_map(|e| {
                    e.file_name()
                        .to_string_lossy()
                        .strip_suffix(".json")
                        .map(|s| s.to_string())
                })
                .collect()
        })
        .unwrap_or_default();
    // Operator directive 2026-07-27: redundant to other tomes, will not be
    // brought in. Sourced here rather than inferred, and reported as its own
    // scope word so it can never be confused with "not yet done".
    let out_of_scope: BTreeSet<&str> = ["beginner_box", "core_essentials"].into_iter().collect();

    let mut books: Vec<BookMeta> = Vec::new();
    for id in &book_dirs {
        let dir = &book_paths[id];
        let includes = pcc_includes(dir, &known_books);
        let rule_set = rule_set_for(id);
        // A directory that no book's `.pcc` stands alone for and that other
        // books pull in is a shared library, not a book. Derived from the real
        // include graph, never assumed.
        //
        // `id == "core_essentials"` is checked BEFORE `rule_set.is_some()`
        // on purpose: `RuleSetId::Ce` exists for real companion/familiar
        // engine consumers (`rules_tables::companion_chassis`), so
        // `rule_set_for("core_essentials")` legitimately resolves to
        // `Some(RuleSetId::Ce)` -- which made this branch dead code before
        // this fix, silently reporting `core_essentials` as `"in_scope"`
        // (a real book) rather than the shared-library classification this
        // module's own contract requires (`decisions.md §25.2`/`§25.3`,
        // `race_resolver.rs`'s module doc, `OPEN-ISSUES.md` row 68). This
        // reorder changes only the REPORTED `scope` field; `RuleSetId::Ce`
        // itself, and every engine consumer that reads it, is untouched.
        let scope = if id == "core_essentials" {
            "shared_library"
        } else if rule_set.is_some() {
            "in_scope"
        } else if out_of_scope.contains(id.as_str()) {
            "out_of_scope"
        } else if registered_stubs.contains(id) || registered_stubs.contains(&format!("{id}_1")) {
            "future_state"
        } else {
            "unregistered"
        };
        books.push(BookMeta { id: id.clone(), scope, rule_set, pcc_includes: includes });
    }

    // Reverse the include graph: which books pull each directory in.
    let mut included_by: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for book in &books {
        for target in &book.pcc_includes {
            included_by.entry(target.clone()).or_default().insert(book.id.clone());
        }
    }

    // --- enumerate ---------------------------------------------------------
    let mut enumerations: BTreeMap<String, BookEnumeration> = BTreeMap::new();
    for book in &books {
        enumerations.insert(book.id.clone(), enumerate_book(&book_paths[&book.id], &book.id));
    }

    // `mod_only_rescue`: a `.MOD` row whose base name appears nowhere in the
    // enumerated corpus is a real declaration in disguise. Resolved AFTER the
    // whole corpus is enumerated, because the base very often lives in a
    // different book than the `.MOD` (the seven CRB races are exactly this).
    let mut declared: BTreeSet<(Kind, String)> = BTreeSet::new();
    for enumeration in enumerations.values() {
        for unit in &enumeration.units {
            declared.insert((unit.kind, unit.name.to_lowercase()));
            declared.insert((unit.kind, unit.key.to_lowercase()));
        }
    }
    for book in &books {
        let Some(enumeration) = enumerations.get_mut(&book.id) else { continue };
        let targets = std::mem::take(&mut enumeration.mod_targets);
        let mut rescued: BTreeSet<(Kind, String)> = BTreeSet::new();
        for (kind, key, name, provenance, magnitudes, resolved_book) in targets {
            if declared.contains(&(kind, name.to_lowercase())) {
                continue;
            }
            if !rescued.insert((kind, name.to_lowercase())) {
                *enumeration.trap_hits.entry("duplicate_identity").or_default() += 1;
                continue;
            }
            *enumeration.trap_hits.entry("mod_only_rescue").or_default() += 1;
            enumeration.units.push(CorpusUnit {
                book: resolved_book.map(str::to_string).unwrap_or_else(|| book.id.clone()),
                source_book: book.id.clone(),
                kind,
                key,
                name,
                origin: Origin::ModOnly,
                provenance,
                magnitude_token_count: magnitudes,
                type_facet: None,
                visible: true,
            });
        }
    }

    // `duplicate_identity`: two rows in one book+kind resolving to the same
    // identity are ONE unit. Applied per book+kind, keyed on the corpus
    // identity (KEY: when present, else display name) -- never on the display
    // name alone, which is what collided 18 archetype-qualified spells.
    for enumeration in enumerations.values_mut() {
        let mut seen: BTreeSet<(Kind, String)> = BTreeSet::new();
        let mut duplicates = 0usize;
        let units = std::mem::take(&mut enumeration.units);
        enumeration.units = units
            .into_iter()
            .filter(|u| {
                if seen.insert((u.kind, u.key.clone())) {
                    true
                } else {
                    duplicates += 1;
                    false
                }
            })
            .collect();
        if duplicates > 0 {
            *enumeration.trap_hits.entry("duplicate_identity").or_default() += duplicates;
        }
    }

    // --- engine ------------------------------------------------------------
    // Every class name the corpus declares anywhere, so a class feature of an
    // un-ingested class (Magus, Ninja, Samurai, ...) is reported as a real
    // `not-ingested` gap rather than as an unclassifiable mystery.
    let corpus_class_names: BTreeSet<String> = enumerations
        .values()
        .flat_map(|e| e.units.iter())
        .filter(|u| u.kind == Kind::Class)
        .map(|u| u.name.to_lowercase())
        .collect();
    let mut facts = gather_engine_facts(&fixture, corpus_class_names, &repo_root);

    // The class_feature consumer-delta probe, run over the keys the CORPUS
    // enumeration just produced rather than over the committed inventory --
    // generating this file from a previous copy of itself would make the
    // observation circular. Its sibling map (which other members share a
    // pool) comes from the same enumeration, so a key's control is always a
    // real alternative the corpus declares.
    {
        let class_feature_keys: Vec<String> = enumerations
            .values()
            .flat_map(|e| e.units.iter())
            .filter(|u| u.kind == Kind::ClassFeature)
            .map(|u| u.key.clone())
            .collect::<BTreeSet<String>>()
            .into_iter()
            .collect();
        facts.class_feature_effect_wired = probe_class_feature_effect_wiring(
            &fixture,
            &facts.class_books.clone(),
            &class_feature_keys,
        );
    }
    let facts = facts;

    // --- wiring_class (GE-01) -----------------------------------------------
    // Built once, corpus-wide: the token closure index and a raw-line cache
    // shared by every unit's determination.
    let mod_index = build_mod_index(&book_paths);
    let mut corpus_lines = CorpusLines::new(&book_paths);

    // --- id uniqueness ------------------------------------------------------
    // How many de-duplicated units in each book+kind slug to the same thing.
    // Counted BEFORE any id is minted, over the same unit set the classifier
    // walks, so `unit_id` can suffix exactly the ids that would otherwise
    // collide and leave every other id byte-identical to the one it has always
    // carried. See [`unit_id`] for why the collisions exist and what they broke.
    let mut slug_population: BTreeMap<(String, Kind, String), usize> = BTreeMap::new();
    for book in &books {
        let Some(enumeration) = enumerations.get(&book.id) else { continue };
        for unit in &enumeration.units {
            *slug_population
                .entry((book.id.clone(), unit.kind, slug(&unit.key)))
                .or_default() += 1;
        }
    }

    // --- classify ----------------------------------------------------------
    let empty: BTreeSet<String> = BTreeSet::new();
    let mut inventory: Vec<InventoryUnit> = Vec::new();
    for book in &books {
        let Some(enumeration) = enumerations.get(&book.id) else { continue };
        let hosts = included_by.get(&book.id).unwrap_or(&empty);
        for unit in &enumeration.units {
            // `source_book`, deliberately, not `unit.book`: this resolves a
            // PHYSICAL file path (`book_paths[book_id]`), and `unit.book`
            // may now name a book that does not physically contain this row
            // (`SD31-ATTRIB-001`'s re-attribution). `source_book` is always
            // the directory `enumerate_book` actually walked to find it.
            let rows = token_closure_rows(
                &mut corpus_lines,
                &mod_index,
                &unit.source_book,
                &unit.provenance.file,
                unit.provenance.line,
                &unit.name,
                &unit.key,
            );
            let row_refs: Vec<Option<&str>> = rows.iter().map(|r| r.as_deref()).collect();
            let (wc_class, wc_reason, wc_signals) = wiring_class::determine_closure(&row_refs);
            // See `classify`'s doc comment: these two reasons are the only
            // ones that mean "a real formula was found in prose", as
            // opposed to a literal-magnitude-token or bonus/pre-guard
            // signal `magnitude_token_count` already covers on its own.
            let carries_prose_magnitude =
                matches!(wc_reason.as_str(), "prose_expr" | "prose_formula_segment");
            let has_real_description = closure_has_real_description(&row_refs)
                || corpus_json_has_real_description(
                    &facts.corpus_json_descriptions,
                    &unit.provenance.file,
                    unit.provenance.line,
                    &unit.key,
                );
            let verdict = classify(unit, &facts, hosts, carries_prose_magnitude, has_real_description);
            let collides = slug_population
                .get(&(book.id.clone(), unit.kind, slug(&unit.key)))
                .is_some_and(|n| *n > 1);
            inventory.push(InventoryUnit {
                id: unit_id(&book.id, unit.kind, &unit.key, collides),
                unit: unit.clone(),
                verdict,
                wiring_class: wc_class,
                wiring_class_reason: wc_reason,
                wiring_class_signals: wc_signals,
            });
        }
    }
    // Deterministic order: the whole idempotence contract rests on this.
    inventory.sort_by(|a, b| {
        (&a.unit.book, a.unit.kind, &a.unit.key, &a.id).cmp(&(&b.unit.book, b.unit.kind, &b.unit.key, &b.id))
    });

    // A deterministic ORDER is not the same guarantee as a unique HANDLE, and
    // every consumer that indexes this file by `id` needs the second one. This
    // instrument emits no output it cannot stand behind: a residual collision
    // is a hard failure here, loudly, rather than an ambiguity a downstream
    // before/after diff silently reports as a wiring_class transition. See
    // [`unit_id`].
    let mut minted: BTreeSet<&str> = BTreeSet::new();
    let mut collisions: Vec<&str> = Vec::new();
    for item in &inventory {
        if !minted.insert(item.id.as_str()) {
            collisions.push(item.id.as_str());
        }
    }
    if !collisions.is_empty() {
        eprintln!(
            "unit id uniqueness violated for {} id(s) -- the inventory's own contract. \
             First offenders: {}",
            collisions.len(),
            collisions.iter().take(10).copied().collect::<Vec<_>>().join(", ")
        );
        std::process::exit(1);
    }

    // The `static`/`derived` done rungs (operator directive 2026-08-13,
    // answering SD-32 decisions.md §2). Extracted into
    // `apply_done_rung_stamps` (launch-readiness remediation Step 4D) so the
    // stamping invariant it relies on -- an `Ambiguous`/`Display`/`Computed`
    // item present in BOTH verified sets is left unstamped, because the
    // stamp only ever means something on a `Static`/`Derived` unit -- has a
    // `#[test]` next to it rather than living only as inline behaviour.
    // Applied here, before `by_status`/`by_kind`/`by_book` are aggregated
    // below, so every rollup in this file's output -- corpus-wide totals,
    // per-book, per-kind -- agrees with the per-unit `status` field the
    // final `units` array carries; doing this only in the serializer would
    // leave the aggregates stale (caught by this generator's own
    // idempotence contract before it ever reached the dashboard).
    apply_done_rung_stamps(&mut inventory, &sweep_verified, &derived_fixture_verified);

    // --- aggregate ---------------------------------------------------------
    let mut by_kind: BTreeMap<&str, usize> = BTreeMap::new();
    let mut by_status: BTreeMap<&str, usize> = BTreeMap::new();
    let mut by_book: BTreeMap<&str, usize> = BTreeMap::new();
    let mut per_book_kind: BTreeMap<(&str, &str), BTreeMap<&str, usize>> = BTreeMap::new();
    // Reconciliation is keyed on the ENGINE's book, not the corpus directory:
    // the Core Rulebook's seven playable races physically live in the shared
    // `core_essentials` library its own `.pcc` pulls in, so a directory-keyed
    // reconciliation would report CRB as holding zero races against an engine
    // that holds seven. Both a declared-only and an all-origins count are
    // carried, because an engine table may or may not have taken the `.COPY=`
    // variants in and only showing one of the two hides which.
    let mut per_engine_kind_declared: BTreeMap<(String, &str), usize> = BTreeMap::new();
    let mut per_engine_kind_total: BTreeMap<(String, &str), usize> = BTreeMap::new();
    for k in Kind::ALL {
        by_kind.insert(k.id(), 0);
    }
    for (status, _) in STATUS_VOCABULARY {
        by_status.insert(status, 0);
    }
    for item in &inventory {
        *by_kind.entry(item.unit.kind.id()).or_default() += 1;
        *by_status.entry(item.verdict.status).or_default() += 1;
        *by_book.entry(item.unit.book.as_str()).or_default() += 1;
        *per_book_kind
            .entry((item.unit.book.as_str(), item.unit.kind.id()))
            .or_default()
            .entry(item.verdict.status)
            .or_default() += 1;
        // SD31-W5-INTEGRATE-001: this fallback used to key on `unit.book`
        // (the reporting field, which SD31-ATTRIB-001 made book-attribution
        // reflect the unit's TRUE source book, not the directory it was
        // physically walked from). `classify()`'s own engine-consumer
        // lookup already learned this the hard way (see its comment at
        // this file's `own_engine_book` binding) -- `unit.source_book` is
        // the one guaranteed to match the ENGINE's consumer table for a
        // relabelled unit; `unit.book` is purely the reporting field now.
        if let Some(engine_book) = item
            .verdict
            .engine_book
            .clone()
            .or_else(|| engine_book_for(&item.unit.source_book).map(|b| b.to_string()))
        {
            *per_engine_kind_total
                .entry((engine_book.clone(), item.unit.kind.id()))
                .or_default() += 1;
            if item.unit.origin == Origin::Declared {
                *per_engine_kind_declared
                    .entry((engine_book, item.unit.kind.id()))
                    .or_default() += 1;
            }
        }
    }
    let engine_records = engine_record_counts();

    // --- emit --------------------------------------------------------------
    let mut out = String::with_capacity(12 * 1024 * 1024);
    out.push_str("{\n");
    out.push_str(&format!("  \"generated_at\": {},\n", q(&real_now_iso8601())));
    out.push_str("  \"generated_by\": \"cargo run --bin v06_work_inventory\",\n");
    out.push_str("  \"schema_version\": 1,\n");
    out.push_str(&format!(
        "  \"corpus_root\": {},\n",
        q(&books_dir.to_string_lossy())
    ));
    out.push_str(&format!(
        "  \"additional_book_dirs\": [{}],\n",
        EXTRA_BOOK_DIRS
            .iter()
            .map(|extra| q(&corpus_root.join(extra).to_string_lossy()))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(
        "  \"contract\": \"Every field below is derived from the corpus or observed from the \
         engine. Nothing here is hand-maintained; two consecutive runs over an unchanged corpus \
         and engine differ only in `generated_at`. Every `units[].id` is unique: it is safe to \
         index this file by `id`, and a before/after comparison keyed on `id` compares like with \
         like. The generator exits non-zero rather than emit a duplicate.\",\n",
    );

    out.push_str("  \"status_vocabulary\": {\n");
    for (i, (status, meaning)) in STATUS_VOCABULARY.iter().enumerate() {
        out.push_str(&format!("    {}: {}", q(status), q(meaning)));
        out.push_str(if i + 1 < STATUS_VOCABULARY.len() { ",\n" } else { "\n" });
    }
    out.push_str("  },\n");

    out.push_str("  \"trap_rules\": [\n");
    for (i, rule) in TRAP_RULES.iter().enumerate() {
        out.push_str(&format!(
            "    {{ \"id\": {}, \"description\": {} }}",
            q(rule.id),
            q(rule.description)
        ));
        out.push_str(if i + 1 < TRAP_RULES.len() { ",\n" } else { "\n" });
    }
    out.push_str("  ],\n");

    out.push_str("  \"magnitude_tokens\": [");
    out.push_str(
        &MAGNITUDE_TOKENS
            .iter()
            .map(|t| q(t))
            .collect::<Vec<_>>()
            .join(", "),
    );
    out.push_str("],\n");

    out.push_str("  \"totals\": {\n");
    out.push_str(&format!("    \"units\": {},\n", inventory.len()));
    out.push_str(&format!("    \"books\": {},\n", books.len()));
    out.push_str("    \"by_kind\": {\n");
    for (i, (k, v)) in by_kind.iter().enumerate() {
        out.push_str(&format!("      {}: {}", q(k), v));
        out.push_str(if i + 1 < by_kind.len() { ",\n" } else { "\n" });
    }
    out.push_str("    },\n");
    out.push_str("    \"by_status\": {\n");
    for (i, (k, v)) in by_status.iter().enumerate() {
        out.push_str(&format!("      {}: {}", q(k), v));
        out.push_str(if i + 1 < by_status.len() { ",\n" } else { "\n" });
    }
    out.push_str("    },\n");
    out.push_str("    \"by_book\": {\n");
    for (i, (k, v)) in by_book.iter().enumerate() {
        out.push_str(&format!("      {}: {}", q(k), v));
        out.push_str(if i + 1 < by_book.len() { ",\n" } else { "\n" });
    }
    out.push_str("    }\n");
    out.push_str("  },\n");

    out.push_str("  \"books\": [\n");
    for (i, book) in books.iter().enumerate() {
        let enumeration = enumerations.get(&book.id);
        out.push_str("    {\n");
        out.push_str(&format!("      \"id\": {},\n", q(&book.id)));
        out.push_str(&format!("      \"scope\": {},\n", q(book.scope)));
        out.push_str(&format!(
            "      \"engine_rule_set\": {},\n",
            match book.rule_set {
                Some(rs) => q(rule_set_id(rs)),
                None => "null".to_string(),
            }
        ));
        out.push_str(&format!(
            "      \"pcc_includes\": [{}],\n",
            book.pcc_includes.iter().map(|b| q(b)).collect::<Vec<_>>().join(", ")
        ));
        out.push_str(&format!(
            "      \"included_by\": [{}],\n",
            included_by
                .get(&book.id)
                .unwrap_or(&empty)
                .iter()
                .map(|b| q(b))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "      \"files_enumerated\": {},\n",
            enumeration.map(|e| e.files_enumerated).unwrap_or(0)
        ));
        out.push_str(&format!(
            "      \"files_not_enumerated\": [{}],\n",
            enumeration
                .map(|e| e.files_not_enumerated.iter().map(|f| q(f)).collect::<Vec<_>>().join(", "))
                .unwrap_or_default()
        ));
        out.push_str("      \"trap_hits\": {");
        if let Some(e) = enumeration {
            out.push_str(
                &e.trap_hits
                    .iter()
                    .map(|(k, v)| format!("{}: {}", q(k), v))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
        out.push_str("},\n");
        out.push_str("      \"kinds\": {\n");
        let kinds: Vec<&str> = Kind::ALL
            .iter()
            .map(|k| k.id())
            .filter(|k| per_book_kind.contains_key(&(book.id.as_str(), *k)))
            .collect();
        for (j, kind) in kinds.iter().enumerate() {
            let statuses = &per_book_kind[&(book.id.as_str(), *kind)];
            let total: usize = statuses.values().sum();
            out.push_str(&format!(
                "        {}: {{ \"units\": {}, \"by_status\": {{{}}} }}",
                q(kind),
                total,
                statuses
                    .iter()
                    .map(|(s, n)| format!("{}: {}", q(s), n))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(if j + 1 < kinds.len() { ",\n" } else { "\n" });
        }
        out.push_str("      },\n");

        // Reconciliation: corpus-declared units against the engine's own table
        // length, for every kind either side has. A non-zero delta is a fact
        // about this codebase, not a defect in this generator -- see the
        // `trap_rules` and the per-kind notes in the two counts' own modules.
        out.push_str("      \"reconciliation\": [");
        if let Some(engine_id) = engine_book_for(&book.id) {
            let rows: Vec<String> = Kind::ALL
                .iter()
                .filter_map(|k| {
                    let kind = k.id();
                    let key = (engine_id.to_string(), kind);
                    let declared = per_engine_kind_declared.get(&key).copied();
                    let total = per_engine_kind_total.get(&key).copied();
                    let engine = engine_records.get(&(engine_id, kind)).copied();
                    if declared.is_none() && total.is_none() && engine.is_none() {
                        return None;
                    }
                    // `engine_records` is null -- never 0 -- for a kind the
                    // engine has no table concept for at all (class features,
                    // companions). A 0 there would read as "nothing ingested"
                    // when the truth is "there is no table to count", and that
                    // exact confusion is what made a previous dashboard render
                    // 41 real monsters as "Not started".
                    let c = declared.unwrap_or(0) as i64;
                    let (engine_field, delta_field) = match engine {
                        Some(e) => (e.to_string(), (c - e as i64).to_string()),
                        None => ("null".to_string(), "null".to_string()),
                    };
                    Some(format!(
                        "{{\"kind\": {}, \"corpus_units_total\": {}, \
                         \"corpus_units_declared\": {}, \"engine_records\": {}, \"delta\": {}}}",
                        q(kind),
                        total.unwrap_or(0),
                        c,
                        engine_field,
                        delta_field
                    ))
                })
                .collect();
            out.push_str(&rows.join(", "));
        }
        out.push_str("]\n");
        out.push_str(if i + 1 < books.len() { "    },\n" } else { "    }\n" });
    }
    out.push_str("  ],\n");

    // `--summary` emits everything above and drops the 38k-row `units` array.
    // The dashboard reads aggregates, and shipping 19 MB through a subprocess
    // pipe on every cron tick would make the refresh cost dwarf the compute it
    // is reporting on. The flag is declared in the payload so a consumer can
    // never mistake a summary for an empty inventory.
    if summary_only {
        out.push_str(&format!(
            "  \"units_omitted\": true,\n  \"units_omitted_count\": {},\n  \"units\": []\n}}\n",
            inventory.len()
        ));
        print!("{out}");
        return;
    }
    out.push_str("  \"units_omitted\": false,\n");

    // One unit per line: compact enough for a 38k-row document, and diffable
    // line-by-line so a bad entry shows up as one changed line.
    out.push_str("  \"units\": [\n");
    for (i, item) in inventory.iter().enumerate() {
        let wc_signals = format!(
            "[{}]",
            item.wiring_class_signals.iter().map(|s| q(s)).collect::<Vec<_>>().join(", ")
        );
        out.push_str(&format!(
            "    {{\"id\": {}, \"book\": {}, \"engine_book\": {}, \"kind\": {}, \"name\": {}, \
             \"corpus_key\": {}, \"origin\": {}, \"visible\": {}, \"type_facet\": {}, \
             \"source_file\": {}, \"source_line\": {}, \"magnitude_token_count\": {}, \
             \"status\": {}, \"evidence\": {}, \"reason\": {}, \"wiring_class\": {}, \
             \"wiring_class_reason\": {}, \"wiring_class_signals\": {}}}",
            q(&item.id),
            q(&item.unit.book),
            opt_q(&item.verdict.engine_book),
            q(item.unit.kind.id()),
            q(&item.unit.name),
            q(&item.unit.key),
            q(item.unit.origin.id()),
            item.unit.visible,
            opt_q(&item.unit.type_facet),
            q(&item.unit.provenance.file),
            item.unit.provenance.line,
            item.unit.magnitude_token_count,
            q(item.verdict.status),
            q(&item.verdict.evidence),
            opt_q(&item.verdict.reason),
            q(item.wiring_class.id()),
            q(&item.wiring_class_reason),
            wc_signals,
        ));
        out.push_str(if i + 1 < inventory.len() { ",\n" } else { "\n" });
    }
    out.push_str("  ]\n");
    out.push_str("}\n");

    if stdout_only {
        print!("{out}");
        return;
    }

    let output_path = repo_root.join(OUTPUT_RELATIVE_PATH);

    // Stamp-loss guard (operator directive 2026-08-14, hazard 1 of
    // SD-30-class-feature-archetype-bundle/state-goals-and-lessons.md §1.3):
    // a plain regen run -- neither `CORPUS_LITERAL_SWEEP_REPORT` nor
    // `DERIVED_FIXTURE_CHECK_REPORT` set -- silently overwrites every
    // `literal-verified`/`fixture-verified` status this file currently
    // carries with nothing, and the diff looks like an ordinary refresh.
    // Refuse to write over a real stamp loss unless the operator explicitly
    // opts in with `--allow-stamp-loss`; a missing/unreadable existing file
    // has nothing to lose and never blocks the write.
    let allow_stamp_loss = args.iter().any(|a| a == "--allow-stamp-loss");
    if let Ok(existing) = std::fs::read_to_string(&output_path) {
        let incoming_stamped: BTreeSet<String> = inventory
            .iter()
            .filter(|item| matches!(item.verdict.status, "literal-verified" | "fixture-verified"))
            .map(|item| item.id.clone())
            .collect();
        let lost = stamp_loss(&existing, &incoming_stamped);
        if !lost.is_empty() && !allow_stamp_loss {
            eprintln!(
                "refusing to write {}: this run would drop {} of the {} verification \
                 stamp(s) (literal-verified/fixture-verified) it currently carries. Set \
                 CORPUS_LITERAL_SWEEP_REPORT and DERIVED_FIXTURE_CHECK_REPORT to the sweep's \
                 and the fixture check's `--json-out` reports before regenerating (see \
                 loop-instruction.md DoD item 4), or pass --allow-stamp-loss to proceed anyway. \
                 First offenders: {}",
                output_path.display(),
                lost.len(),
                stamped_ids(&existing).len(),
                lost.iter().take(10).cloned().collect::<Vec<_>>().join(", ")
            );
            std::process::exit(1);
        }
    }

    if let Some(parent) = output_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(&output_path, &out) {
        eprintln!("could not write {}: {e}", output_path.display());
        std::process::exit(1);
    }
    print!("{out}");
}

/// 2026-08-14 incentive-fix investigation: proves `classify()`'s own
/// `text_only` signal agrees with `wiring_class::determine_closure`'s
/// %N-placeholder detection (`99efb504`) instead of contradicting it. See
/// `classify`'s doc comment for the real corpus lines that motivated this.
#[cfg(test)]
mod prose_magnitude_status_tests {
    use super::*;

    fn feat_unit(book: &str, key: &str, magnitude_token_count: usize) -> CorpusUnit {
        CorpusUnit {
            book: book.to_string(),
            source_book: book.to_string(),
            kind: Kind::Feat,
            key: key.to_string(),
            name: key.to_string(),
            origin: Origin::Declared,
            provenance: Provenance { file: "feats.lst".to_string(), line: 1 },
            magnitude_token_count,
            type_facet: None,
            visible: true,
        }
    }

    fn facts_with_feat_catalog(book: &'static str, key: &str) -> EngineFacts {
        let mut facts = EngineFacts::default();
        facts.feat_keys.entry(book).or_default().insert(key.to_string());
        facts
    }

    /// Before this fix: a feat with zero `MAGNITUDE_TOKENS` fields always
    /// read `text-complete`, even when `wiring_class` had already resolved a
    /// real formula in its DESC/BENEFIT prose (Zomok's Breath Weapon shape:
    /// `DC %1...|CON+18`). That is a direct contradiction of
    /// `status_vocabulary`'s promise that `text-complete` means "NO
    /// magnitude token at all, so there is no number to compute" -- there
    /// plainly is one here, `wiring_class` just found it in prose rather
    /// than in a `MAGNITUDE_TOKENS`-prefixed field.
    #[test]
    fn a_prose_formula_feat_does_not_read_text_complete() {
        let facts = facts_with_feat_catalog("core_rulebook", "Zomok's Breath Weapon");
        let verdict = classify(
            &feat_unit("core_rulebook", "Zomok's Breath Weapon", 0),
            &facts,
            &BTreeSet::new(),
            true, // wiring_class resolved prose_formula_segment/prose_expr
            true,
        );
        assert_ne!(verdict.status, "text-complete");
        // Honestly `unknown`/`held`, not silently promoted to `done`: the
        // fix must not manufacture a done-eligible status out of a formula
        // the corpus-literal/evaluator-fixture bar has not verified.
        assert_eq!(verdict.status, "unknown");
    }

    /// A feat with genuinely zero magnitude anywhere -- no
    /// `MAGNITUDE_TOKENS` field AND no prose formula -- must still read
    /// `text-complete`. The fix narrows the signal, it does not remove it.
    #[test]
    fn a_true_no_magnitude_feat_still_reads_text_complete() {
        let facts = facts_with_feat_catalog("core_rulebook", "Iron Will");
        let verdict = classify(
            &feat_unit("core_rulebook", "Iron Will", 0),
            &facts,
            &BTreeSet::new(),
            false,
            true, // has a real corpus description
        );
        assert_eq!(verdict.status, "text-complete");
    }

    /// A feat whose own `MAGNITUDE_TOKENS` field count is already nonzero
    /// must not regress just because `carries_prose_magnitude` is false --
    /// the two signals are additive (`||`), never exclusive.
    #[test]
    fn a_token_magnitude_feat_is_unaffected_by_the_prose_signal() {
        let facts = facts_with_feat_catalog("core_rulebook", "Power Attack");
        let verdict = classify(
            &feat_unit("core_rulebook", "Power Attack", 1),
            &facts,
            &BTreeSet::new(),
            false,
            true,
        );
        assert_ne!(verdict.status, "text-complete");
    }

    /// SD31-D7-PROSE-001 (Decision 7's condition 3, the PROXY WARNING): a
    /// zero-magnitude feat with NO real corpus description anywhere in its
    /// token closure must NOT read `text-complete` -- there is nothing to
    /// show a player, so this is not the completion Decision 7 describes.
    /// Corpus-wide re-derivation the day this test was written found 634
    /// units already `done` on the live board in exactly this shape.
    #[test]
    fn a_zero_magnitude_feat_with_no_real_description_does_not_read_text_complete() {
        let facts = facts_with_feat_catalog("core_rulebook", "Bare Chassis Feat");
        let verdict = classify(
            &feat_unit("core_rulebook", "Bare Chassis Feat", 0),
            &facts,
            &BTreeSet::new(),
            false,
            false, // no real DESC: text anywhere in the closure
        );
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "unknown");
        assert_eq!(
            verdict.evidence,
            "text_only_but_corpus_record_carries_no_description_to_show_a_player"
        );
    }
}

#[cfg(test)]
mod wiring_class_wiring_tests {
    use super::*;

    /// A scratch corpus directory, cleaned up on drop, so these tests never
    /// touch the real PCGen checkout `PCGEN_CORPUS_ROOT` would point at.
    struct ScratchBook {
        root: PathBuf,
    }

    impl ScratchBook {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_wiring_class_test_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(&root).unwrap();
            ScratchBook { root }
        }

        fn write(&self, filename: &str, contents: &str) {
            std::fs::write(self.root.join(filename), contents).unwrap();
        }
    }

    impl Drop for ScratchBook {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn mod_base_name_strips_category_and_class_qualifiers() {
        assert_eq!(mod_base_name("Foo"), "Foo");
        assert_eq!(mod_base_name("CATEGORY=Special Ability|Foo"), "Foo");
        assert_eq!(mod_base_name("CLASS:Bard"), "Bard");
    }

    #[test]
    fn build_mod_index_finds_a_mod_row_regardless_of_file_kind_recognition() {
        let book = ScratchBook::new("modindex");
        // `weird_file.lst` matches no `file_kind` the generator recognises,
        // but `build_mod_index` must still find its `.MOD` row -- it walks
        // the whole corpus tree independent of kind recognition, exactly
        // like the reference determinator's own `mod_index()`.
        book.write(
            "weird_file.lst",
            "Accursed.MOD\tBONUS:SAVE|Fortitude|CASTERLEVEL/2\n",
        );
        let mut book_paths = BTreeMap::new();
        book_paths.insert("test_book".to_string(), book.root.clone());

        let index = build_mod_index(&book_paths);
        let rows = index.get(&("test_book".to_string(), "Accursed".to_string()));
        assert!(rows.is_some(), "expected a .MOD row indexed under the base name");
        assert_eq!(rows.unwrap().len(), 1);
        assert!(rows.unwrap()[0].contains("CASTERLEVEL/2"));
    }

    #[test]
    fn token_closure_rows_unions_base_row_and_mod_rows() {
        let book = ScratchBook::new("closure");
        book.write("cr_feats.lst", "Accursed\tTYPE:General\tDESC:You are marked by a curse.\n");
        book.write("cr_feats_extra.lst", "Accursed.MOD\tBONUS:SAVE|Fortitude|CASTERLEVEL/2\n");
        let mut book_paths = BTreeMap::new();
        book_paths.insert("test_book".to_string(), book.root.clone());

        let mod_index = build_mod_index(&book_paths);
        let mut lines = CorpusLines::new(&book_paths);
        let unit = CorpusUnit {
            book: "test_book".to_string(),
            source_book: "test_book".to_string(),
            kind: Kind::Feat,
            key: "Accursed".to_string(),
            name: "Accursed".to_string(),
            origin: Origin::Declared,
            provenance: Provenance { file: "cr_feats.lst".to_string(), line: 1 },
            magnitude_token_count: 0,
            type_facet: None,
            visible: true,
        };

        let rows = token_closure_rows(
            &mut lines,
            &mod_index,
            &unit.book,
            &unit.provenance.file,
            unit.provenance.line,
            &unit.name,
            &unit.key,
        );
        assert_eq!(rows.len(), 2, "base row plus one .MOD row");
        let row_refs: Vec<Option<&str>> = rows.iter().map(|r| r.as_deref()).collect();
        let (class, _, sigs) = wiring_class::determine_closure(&row_refs);
        // The base row alone (no magnitude token) would be `display`; the
        // `.MOD` row's `BONUS:` promotes the unit to `derived` -- proves the
        // closure is really unioned, not just the base row re-read.
        assert_eq!(class, wiring_class::WiringClass::Derived);
        assert!(sigs.iter().any(|s| s.starts_with("derived:")));
    }

    #[test]
    fn token_closure_rows_is_none_for_a_line_past_end_of_file() {
        let book = ScratchBook::new("nolinerow");
        book.write("cr_feats.lst", "Only Line\tTYPE:General\n");
        let mut book_paths = BTreeMap::new();
        book_paths.insert("test_book".to_string(), book.root.clone());
        let mod_index: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
        let mut lines = CorpusLines::new(&book_paths);
        let unit = CorpusUnit {
            book: "test_book".to_string(),
            source_book: "test_book".to_string(),
            kind: Kind::Feat,
            key: "Ghost".to_string(),
            name: "Ghost".to_string(),
            origin: Origin::Declared,
            // Line 99 does not exist in a 1-line file.
            provenance: Provenance { file: "cr_feats.lst".to_string(), line: 99 },
            magnitude_token_count: 0,
            type_facet: None,
            visible: true,
        };
        let rows = token_closure_rows(
            &mut lines,
            &mod_index,
            &unit.book,
            &unit.provenance.file,
            unit.provenance.line,
            &unit.name,
            &unit.key,
        );
        assert_eq!(rows, vec![None]);
        let row_refs: Vec<Option<&str>> = rows.iter().map(|r| r.as_deref()).collect();
        let (class, reason, _) = wiring_class::determine_closure(&row_refs);
        assert_eq!(class, wiring_class::WiringClass::Ambiguous);
        assert_eq!(reason, "no_corpus_line");
    }
}

#[cfg(test)]
mod rule_set_mapping_tests {
    use super::*;

    /// Regression guard for the wildcard bug. SD-27 shipped ARG and PU into the
    /// engine — feat tables, class chassis, reach gating — but `rule_set_for`
    /// ended in `_ => None`, so the inventory reported both books as
    /// `not-started` / `future_state` / 0% proven. The compiler could not catch
    /// it because a wildcard arm absorbs new variants silently.
    #[test]
    fn every_compiled_rule_set_round_trips_through_its_corpus_dir() {
        for &rs in COMPILED_RULE_SETS {
            let dir = corpus_dir_for(rs);
            assert_eq!(
                rule_set_for(dir),
                Some(rs),
                "{dir} must map back to the rule set it was compiled from",
            );
        }
    }

    #[test]
    fn ingested_books_are_measurable() {
        assert_eq!(rule_set_for("advanced_race_guide"), Some(RuleSetId::Arg));
        assert_eq!(rule_set_for("pathfinder_unchained"), Some(RuleSetId::Pu));
    }

    /// The engine id and the corpus directory disagree for exactly one book.
    #[test]
    fn bestiary_directory_maps_despite_the_id_rename() {
        assert_eq!(rule_set_for("bestiary"), Some(RuleSetId::Bestiary1));
        assert_eq!(rule_set_id(RuleSetId::Bestiary1), "bestiary_1");
    }

    #[test]
    fn corpus_dirs_are_distinct() {
        let mut seen = std::collections::BTreeSet::new();
        for &rs in COMPILED_RULE_SETS {
            assert!(seen.insert(corpus_dir_for(rs)), "duplicate corpus dir for {rs:?}");
        }
        assert_eq!(seen.len(), COMPILED_RULE_SETS.len());
    }

    /// A book the engine has not compiled must still report honestly.
    #[test]
    fn uncompiled_books_stay_none() {
        // This test needs a book the engine genuinely has not compiled, and it
        // has now outlived THREE of them: `ultimate_psionics` moved to
        // compiled in SD28-E29 (`epic-29-upsi-complete`), `inner_sea_gods` in
        // SD-29 Epic 5 extend round 9 (`RuleSetId::Isg`), and
        // `occult_adventures` in SD31-E6-F2-003 (`RuleSetId::Oa`, its spell
        // family). The comment this replaces also stated a reason that was
        // wrong by the time it was read -- "`inner_sea_gods` ... (SD-30's own
        // book set, out of this bundle)" -- and `decisions.md §38` had
        // already re-scoped SD-29 corpus-wide.
        //
        // `adventurers_guide` is uncompiled by DERIVATION, not by assumption:
        // `corpus_dir_for` is exhaustive over `RuleSetId` and carries no arm
        // returning it, so no `COMPILED_RULE_SETS` member can map to it --
        // re-checked against the current match arm one by one (32 arms, none
        // return `"adventurers_guide"`), and the book genuinely has a corpus
        // directory (`data/corpus/adventurers_guide/`), so this is a real
        // uncompiled book, not a typo'd nonexistent one.
        assert_eq!(rule_set_for("adventurers_guide"), None);
    }
}

/// SD28-E14: observation-harness widening tests. F2 (equipment probe) with
/// a positive proof against the real on-disk corpus and negative proofs
/// that the probe does NOT promote a unit the engine genuinely does not
/// wire (F3's anti-gaming binding). F1 (spell probe) was deliberately absent
/// when this module was written -- see the doc comment at the bottom of this
/// module for why, and `mod spell_probe_tests` for the cycle that superseded
/// that reasoning once `epic-31-spell-wiring` gave the spell magnitude a wired
/// consumer to observe.
#[cfg(test)]
mod e14_harness_tests {
    use super::*;
    use codex::pcgen_import::ir_converter::convert_equipment_record;
    use codex::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn equipment_corpus_from(text: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("test.lst", text);
        let source_ref = SourceRef { lst_file: "test.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("test", source_ref);
        for record in result.entries {
            let record: &'static _ = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    // ----- F2: equipment probe -----

    /// The probe must ask its wiring question of **every** key the engine
    /// catalog holds, because `classify()` decides `known` from that same
    /// catalog (`facts.equipment_keys`, built from
    /// `equipment_catalog_rows()`). Any catalog key the probe never examines
    /// is a unit that can only ever report
    /// `equipment_table_entry_with_corpus_magnitude` — not because the engine
    /// computes nothing from it, but because nobody asked.
    ///
    /// This is the guard, not the fix: it fails against the previous
    /// four-hand-table key universe, which omitted every gap row and every
    /// book past `crb`/`apg`/`acg`/`beastiary1`.
    ///
    /// It deliberately pins **coverage of the question**, never the answer.
    /// `equipment_key_is_wired` — the bar — is untouched by it.
    #[test]
    fn the_probe_examines_every_key_the_engine_catalog_holds() {
        let universe = probe_equipment_key_universe();
        let unexamined: Vec<&str> = equipment_resolver::equipment_catalog_rows()
            .iter()
            .map(|row| row.key)
            .filter(|key| !universe.contains(key))
            .collect();
        assert!(
            unexamined.is_empty(),
            "{} equipment catalog key(s) are never asked the wiring question, \
             e.g. {:?}",
            unexamined.len(),
            &unexamined[..unexamined.len().min(10)]
        );
    }

    /// The observation is book-scoped, and a shared key is not a shared item.
    ///
    /// `Celestial Shield` is printed in BOTH Ultimate Equipment and the
    /// Advanced Race Guide under the same key, and the two rows are different
    /// items: ARG's is a heavy shield (`ACCHECK:0`, `SPELLFAILURE:0`), UE's is
    /// a light shield (`ACCHECK:-1`, `SPELLFAILURE:5`). Before SD-31
    /// `SD31-E6-F5-001`, only ARG had a `data/corpus/` directory at all, so a
    /// book-agnostic probe would have grounded UE's unit on ARG's numbers —
    /// the name-coincidence defect `modelled_race_of_race_trait` already
    /// records for `race_trait`.
    ///
    /// **UE now has a real `data/corpus/ultimate_equipment/equipment/`
    /// directory** (`cache_gen::ultimate_equipment`, dumping
    /// `rules_tables::ultimate_equipment::equipment_tables`), but that
    /// table's own doc comment documents 65 keys (55 equipment + 10
    /// equipmods) deliberately EXCLUDED as cross-book republished items --
    /// `Celestial Shield` is one of them (`Dogslicer` is the module's own
    /// spot-checked example of the same exclusion). So the assertion below
    /// still holds, now for the *correct*, book-scoped reason (UE's real
    /// corpus was read and genuinely does not carry this key) rather than
    /// the earlier, weaker reason (UE had no corpus to read at all).
    ///
    /// This pins the *attribution*, and it is a strictly HIGHER bar than the
    /// bare-key form it replaced: it can only ever withhold a grounding, never
    /// grant one.
    #[test]
    fn a_key_two_books_share_grounds_only_the_book_whose_corpus_was_read() {
        let wired = probe_equipment_effect_wiring(&repo_root());
        assert!(
            wired.contains(&(
                "advanced_race_guide".to_string(),
                "Celestial Shield".to_string()
            )),
            "ARG owns a real corpus record for this key and must still ground"
        );
        assert!(
            !wired.contains(&(
                "ultimate_equipment".to_string(),
                "Celestial Shield".to_string()
            )),
            "Ultimate Equipment has no corpus directory at all -- nothing observed \
             ITS record, so nothing may claim it"
        );
        // Structural, not just this one pair: no book without a
        // `data/corpus/<book>/equipment` directory may appear at all.
        let observable: BTreeSet<&'static str> = OBSERVABLE_BOOK_DIRS
            .iter()
            .filter_map(|dir| engine_book_for_corpus_dir(dir))
            .collect();
        let unobservable: BTreeSet<&String> = wired
            .iter()
            .map(|(book, _)| book)
            .filter(|book| !observable.contains(book.as_str()))
            .collect();
        assert!(
            unobservable.is_empty(),
            "these books have no loaded corpus yet appear in the probe result: {unobservable:?}"
        );
    }

    /// Every observable book's catalog keys really reach the probe.
    ///
    /// `probe_equipment_effect_wiring` looks the book up by the slug
    /// `engine_book_for_corpus_dir` returns and skips it on a miss. That miss
    /// is SILENT — a book whose two slugs stop agreeing simply stops being
    /// probed, and every one of its units quietly falls back to
    /// `ingested-magnitude`. Bestiary 1 is the standing trap: its corpus
    /// directory is `beastiary`, its engine book is `bestiary_1`, and the two
    /// only meet through `CORPUS_DIR_ALIASES`.
    #[test]
    fn every_observable_books_catalog_keys_reach_the_probe() {
        let keys_by_book = probe_equipment_keys_by_book();
        for dir in OBSERVABLE_BOOK_DIRS {
            let engine_book = engine_book_for_corpus_dir(dir).unwrap_or_else(|| {
                panic!("observable corpus dir {dir:?} resolves to no engine book")
            });
            let keys = keys_by_book.get(engine_book).unwrap_or_else(|| {
                panic!(
                    "corpus dir {dir:?} -> engine book {engine_book:?} has no catalog keys \
                     at all; the probe silently skips this whole book"
                )
            });
            assert!(
                !keys.is_empty(),
                "{engine_book} contributes an empty key set to the probe"
            );
        }
    }

    /// Control for the test above: proves the universe is genuinely wider
    /// than the four compiled tables it used to be built from, so that guard
    /// cannot pass vacuously by both sides shrinking together.
    #[test]
    fn the_probe_key_universe_is_wider_than_the_four_compiled_tables() {
        let universe = probe_equipment_key_universe();
        let mut four_tables: BTreeSet<&'static str> = BTreeSet::new();
        four_tables.extend(crb_equipment_tables::equipment_tables().iter().map(|e| e.key));
        four_tables.extend(apg::equipment_tables::EQUIPMENT_TABLE.iter().map(|e| e.key));
        four_tables.extend(acg::equipment_tables::equipment_tables().iter().map(|e| e.key));
        four_tables.extend(beastiary1::equipment_tables::EQUIPMENT_TABLE.iter().map(|e| e.key));
        // Printed, not pinned: the two sizes are the RED evidence for the
        // widening (how many catalog keys went unexamined), and pinning
        // either as a literal would turn every future table addition into an
        // unrelated red test — the count-pin hazard this repo already tracks.
        eprintln!(
            "probe key universe: {} keys; four compiled tables alone: {} keys; \
             previously unexamined: {}",
            universe.len(),
            four_tables.len(),
            universe.len() - four_tables.len()
        );
        assert!(
            universe.len() > four_tables.len(),
            "universe {} must exceed the four hand tables' {}",
            universe.len(),
            four_tables.len()
        );
        assert!(
            four_tables.iter().all(|k| universe.contains(k)),
            "the widened universe must still contain every key the four tables held"
        );
    }

    /// Positive: CRB's real, on-disk Padded Armor (Base) carries real
    /// AC/max-dex/spell-failure/ACP tokens, resolves against the real
    /// corpus, and the probe's own wiring check observes them.
    #[test]
    fn equipment_probe_promotes_a_real_armor_item_with_real_ac_tokens() {
        let roots = [BookCorpusRoot {
            book_id: "core_rulebook",
            dir: &repo_root().join("data/corpus/core_rulebook"),
        }];
        let corpus = load_equipment_corpus(&roots);
        assert!(equipment_key_is_wired("Padded Armor (Base)", &corpus));
    }

    /// Negative (F3's anti-gaming binding): a real corpus record that
    /// carries NO mechanical token at all -- no BONUS chain, no ACCHECK, no
    /// COST-derived enhancement, nothing `compute_equipment_effects` reads
    /// -- resolves (it is a genuine record) but must NOT be promoted. A
    /// probe that returns `true` for every resolvable item, permissive
    /// rather than observing a real delta, is exactly the failure mode this
    /// pins.
    #[test]
    fn equipment_probe_never_promotes_a_text_only_item_with_no_mechanical_tokens() {
        let text = "Plain Sack\tTYPE:Container\tCOST:0\n";
        let corpus = equipment_corpus_from(text);
        assert!(
            !equipment_key_is_wired("Plain Sack", &corpus),
            "an item with no armor/skill/ability/weapon-enhancement token must stay unwired"
        );

        // Control: proves the harness above is actually exercising the
        // record (it resolves), not merely failing to find it at all --
        // otherwise the negative result above would be meaningless.
        let selection = vec![EquipmentSelection {
            item_id: "Plain Sack".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }];
        let effects = compute_equipment_effects(&selection, &corpus);
        assert_eq!(effects.per_item.len(), 1, "the plain item must still resolve and appear per-item");
    }

    /// An item absent from the corpus entirely (never ingested) must also
    /// stay unwired -- distinct failure path from "resolves but inert".
    #[test]
    fn equipment_probe_never_promotes_an_item_absent_from_the_corpus() {
        let corpus = equipment_corpus_from("Something Else\tTYPE:Container\tCOST:0\n");
        assert!(!equipment_key_is_wired("Nonexistent Item Nobody Ingested", &corpus));
    }

    // ----- F1: spell probe -- NOT implemented, and this is the finding -----
    //
    // CONCLUSION SUPERSEDED 2026-08-13 by `mod spell_probe_tests` (SD-32
    // `spell-consumer-delta-probe`). The retracted-attempt account below still
    // stands and is why it is kept; the claim that no wired spell-magnitude
    // consumer exists no longer holds. `epic-31-spell-wiring` (2026-08-07)
    // wired `compute_spellbook_coverage` into
    // `pf1_adapter::resolve_unified_pilot_snapshot`.
    //
    // An earlier version of this cycle probed
    // `pilot_compute_corpus::compute_pilot_with_corpus`'s `school_coverage`
    // map (a spell lands there once it resolves and its `school` string
    // parses -- `pilot_compute_corpus.rs:189-205`). Independent review
    // (team-lead, 2026-08-06) correctly identified that this observes
    // resolution, not a magnitude: no spell field (level, DC, duration, ...)
    // is read into any computed number on that path. The 100% promotion
    // rate the wrong version produced (1,067 of 1,067) was the tell -- a
    // discriminating probe over real content does not do that (contrast
    // F2's 173-of-2,983, 5.8%).
    //
    // Per the dispatching brief's own instruction ("strengthen the negative
    // test to pin a present-but-non-mechanical spell as not promoted; if no
    // such test can be written because nothing distinguishes them, that is
    // itself the finding") -- no such test can be written. Every spell that
    // resolves against the on-disk corpus and has a recognized school
    // string lands in `school_coverage` identically; there is no field on
    // that path that varies between a spell with real mechanical content
    // and one without. That absence of a discriminating signal, not a
    // missing test, is the finding: **no currently-wired consumer reads a
    // spell's magnitude at all.** `spellbook::compute_spellbook_coverage`
    // does read a real magnitude (`SpellEffect.level`, feeding
    // `spell_save_dc`/slot math) and is wired into
    // `contract::PilotReceipt.spellbook` (`contract.rs:397`), but
    // `contract::build_pilot_receipt` is never called from
    // `apps/desktop/src-tauri/src/pf1_adapter.rs` or `character_hub.rs` --
    // confirmed by `grep -rn build_pilot_receipt apps/desktop/src-tauri/src`
    // returning nothing. That is exactly the "twin problem"
    // `decisions.md §29.1`/`§29.2` already names: a real computation that
    // never reaches the surface `pf1_adapter::resolve_unified_pilot_snapshot`
    // gates on. Wiring `contract.rs`'s spellbook output into that surface
    // is the remedy, and it is engine work outside this harness-widening
    // epic's scope -- see `artifacts/e14-harness-widening.md`. All 1,067
    // targeted spell units stay `ingested-magnitude`.
}

#[cfg(test)]
mod equipment_book_slug_tests {
    use super::*;

    /// SD-28-E15's own regression guard: `equipment_keys` is now derived
    /// directly from `equipment_resolver::equipment_catalog_rows()` rather
    /// than a hand-maintained parallel list, closing the divergence that
    /// silently misreported ~1,650 landed UE/UI/PU equipment units as
    /// `not-ingested`. This test exercises `equipment_book_slug_for`
    /// against every book code the resolver ACTUALLY returns today, so a
    /// ninth book added to the resolver without a matching arm here fails
    /// this test immediately (a panic on `cargo test`) instead of silently
    /// dropping that book's units from the inventory the way the old
    /// four-`.insert()`-call list did.
    #[test]
    fn equipment_book_slug_for_covers_every_catalog_book() {
        let codes: std::collections::BTreeSet<&'static str> =
            equipment_resolver::equipment_catalog_rows().iter().map(|row| row.book).collect();
        assert!(!codes.is_empty(), "the resolver must carry at least one book's rows");
        for code in codes {
            // Panics (failing this test) if `equipment_book_slug_for` does
            // not recognize `code` -- the exact failure mode this fix
            // replaces a silent one with.
            let slug = equipment_book_slug_for(code);
            assert!(!slug.is_empty(), "{code} resolved to an empty slug");
        }
    }

    /// The spell-family twin of `equipment_book_slug_for_covers_every_catalog_book`.
    /// A sixth book added to `spell_catalog_rows()` without a matching arm in
    /// `spell_book_slug_for` fails this test immediately (a panic on
    /// `cargo test`) instead of silently dropping that book's spell units
    /// from the inventory the way the old three-`.insert()`-call list did.
    #[test]
    fn spell_book_slug_for_covers_every_catalog_book() {
        let codes: std::collections::BTreeSet<&'static str> =
            spell_resolver::spell_catalog_rows().iter().map(|row| row.book).collect();
        assert!(!codes.is_empty(), "the registry must carry at least one book's rows");
        for code in codes {
            let slug = spell_book_slug_for(code);
            assert!(!slug.is_empty(), "{code} resolved to an empty slug");
        }
    }

    /// The specific defect this fix closes, pinned so it cannot regress.
    /// Before the consolidation the work inventory's `spell_levels` map held
    /// three books while the shipped Spell Catalog served five, so every ARG
    /// and UI spell was reported `not-ingested` despite already being on
    /// screen. Two real corpus keys, one per newly-joined book, must now be
    /// reachable through the registry under their own book slug.
    #[test]
    fn arg_and_ui_spell_keys_are_reachable_through_the_derived_map() {
        let rows = spell_resolver::spell_catalog_rows();
        assert!(
            rows.iter().any(|r| r.book == "ARG" && r.key == "Aboleth's Lung"),
            "Aboleth's Lung must be a real ARG row in spell_catalog_rows()"
        );
        assert!(
            rows.iter().any(|r| r.book == "UI" && r.key == "Absolution"),
            "Absolution must be a real UI row in spell_catalog_rows()"
        );
        assert_eq!(spell_book_slug_for("ARG"), "advanced_race_guide");
        assert_eq!(spell_book_slug_for("UI"), "ultimate_intrigue");
    }

    /// The consolidation must be a pure widening for the three books that
    /// were already mapped: every CRB/APG/ACG key the old hand-maintained
    /// inserts produced, with the same level-known flag, must still be
    /// present. This is what makes the change safe to land without moving
    /// any already-`ingested-magnitude` unit.
    #[test]
    fn registry_preserves_every_key_the_hand_maintained_map_carried() {
        let rows = spell_resolver::spell_catalog_rows();
        let derived = |slug: &str| -> BTreeMap<String, bool> {
            rows.iter()
                .filter(|r| spell_book_slug_for(r.book) == slug)
                .map(|r| (r.key.to_string(), r.level.is_some()))
                .collect()
        };

        let crb_expected: BTreeMap<String, bool> =
            crb_spell_list::SPELL_LIST.iter().map(|e| (e.key.to_string(), true)).collect();
        assert_eq!(derived("core_rulebook"), crb_expected);

        let apg_expected: BTreeMap<String, bool> = apg::spell_list::SPELL_LIST
            .iter()
            .map(|e| (e.key.to_string(), e.level.is_some()))
            .collect();
        assert_eq!(derived("advanced_players_guide"), apg_expected);

        let acg_expected: BTreeMap<String, bool> =
            acg::spell_list::SPELL_LIST.iter().map(|e| (e.key.to_string(), true)).collect();
        assert_eq!(derived("advanced_class_guide"), acg_expected);
    }

    /// The specific defect this fix closes, pinned so it cannot regress:
    /// UE's real corpus key `Abjurant Salt` (`ue_equip_magic_items.lst:954`,
    /// verified present in `ultimate_equipment::equipment_tables()`) must
    /// resolve into the `ultimate_equipment` bucket of the derived map.
    #[test]
    fn ultimate_equipment_key_is_reachable_through_the_derived_map() {
        let rows = equipment_resolver::equipment_catalog_rows();
        let found = rows
            .iter()
            .any(|row| row.book == "UE" && row.key == "Abjurant Salt");
        assert!(found, "Abjurant Salt must be a real UE row in equipment_catalog_rows()");
        assert_eq!(equipment_book_slug_for("UE"), "ultimate_equipment");
    }
}

#[cfg(test)]
mod race_trait_grounding_tests {
    use super::*;

    /// The seven races CRB's `race_traits()` table is keyed on, exactly as
    /// `gather_engine_facts` builds them (`race_name`, lowercase).
    fn modelled_races() -> BTreeSet<String> {
        RaceId::ALL.iter().map(|&r| race_name(r).to_string()).collect()
    }

    /// The regression this card exists to close
    /// (`docs/release/corpus-work-channels.md` §9.3, SD-28 §56). Each of these
    /// records was reported `grounded` by the old rule purely because its
    /// TRAIT NAME slug collided with a CRB trait's — re-derived from
    /// `docs/work-inventory.json` on 2026-08-11, where all of them carried
    /// `race_trait_record_grounded_by_race_traits`. None of these races is
    /// modelled by the engine at all, so none can ground on anything.
    #[test]
    fn a_race_the_engine_does_not_model_grounds_no_trait_however_its_name_collides() {
        let races = modelled_races();
        for key in [
            "Blue ~ Keen Senses",        // collided with elf.keen_senses
            "DuergarDSP ~ Hardy",        // collided with dwarf.hardy
            "DuergarDSP ~ Stability",    // collided with dwarf.stability
            "Forgeborn ~ Fearless",      // collided with halfling.fearless
            "Aquatic Elf ~ Elven Magic", // "Aquatic Elf" is not "Elf"
            "Svirfneblin ~ Stonecunning",
        ] {
            assert_eq!(
                modelled_race_of_race_trait(key, &races),
                None,
                "{key} names no modelled race and must not ground"
            );
        }
    }

    /// The other half of the ruling: the fix must not throw away the records
    /// that legitimately ground. A CRB race's own trait still resolves to that
    /// race, and ARG's heritage form carries its base race as an inner
    /// qualifier rather than the leading one.
    #[test]
    fn a_modelled_race_is_found_in_its_own_key_including_the_inner_heritage_qualifier() {
        let races = modelled_races();
        assert_eq!(
            modelled_race_of_race_trait("Dwarf ~ Greed", &races).map(String::as_str),
            Some("dwarf")
        );
        assert_eq!(
            modelled_race_of_race_trait("Half-Elf ~ Keen Senses", &races).map(String::as_str),
            Some("half-elf")
        );
        // ARG's `Saltbeard ~ Dwarf ~ Greed`: the leading segment is the
        // heritage, the base race sits in the middle.
        assert_eq!(
            modelled_race_of_race_trait("Saltbeard ~ Dwarf ~ Greed", &races).map(String::as_str),
            Some("dwarf")
        );
    }

    /// The trailing segment is the trait name, never the race. Without this
    /// exclusion a trait whose NAME equals a race name would nominate itself
    /// and re-open the very coincidence class this fix closes.
    #[test]
    fn the_trailing_trait_name_segment_is_never_read_as_the_race() {
        let races = modelled_races();
        assert_eq!(modelled_race_of_race_trait("Dwarf", &races), None);
        assert_eq!(modelled_race_of_race_trait("Orc ~ Human", &races), None);
    }

    // -----------------------------------------------------------------
    // The probe repair (SD-29 `decisions.md §43.5`).
    //
    // Everything above tests the CRB-table probe's *name-coincidence*
    // guard, and that guard is correct. What it cannot do is ground a
    // record belonging to a race CRB's compiled table never mentions --
    // and the product models 18 races off disk, not 7. The tests below
    // pin the replacement probe, which asks the race corpus the desktop
    // app actually loads whether it can APPLY the record to a player.
    // -----------------------------------------------------------------

    fn probe_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// The probe must load exactly the books the app loads. A probe over a
    /// wider list would ground records no player can reach; a narrower one
    /// would under-report. Neither list is hand-maintained here -- both are
    /// read from `race_catalog.rs`, so the pin cannot drift silently.
    #[test]
    fn the_probe_loads_exactly_the_books_the_desktop_app_loads() {
        let books = app_race_corpus_books(&probe_root());
        assert!(
            books.contains(&"core_rulebook".to_string())
                && books.contains(&"beastiary".to_string())
                && books.contains(&"monster_codex".to_string()),
            "the app's race corpus book list did not parse: {books:?}"
        );
        let src = std::fs::read_to_string(
            probe_root().join("apps/desktop/src-tauri/src/race_catalog.rs"),
        )
        .expect("the desktop race catalog source is readable from the repo root");
        for book in &books {
            assert!(
                src.contains(&format!("\"{book}\"")),
                "{book} is not named in race_catalog.rs at all"
            );
        }
    }

    /// The point of the repair. `Duergar ~ Ironskinned` belongs to a race
    /// `crb::race_traits()` has never heard of, so the CRB-table probe
    /// reports `race_trait_race_not_modelled` for it -- while `reach_gate`
    /// executes a passing claim against the same record and SD-29's own
    /// on-screen verification photographed it in the player's picker.
    #[test]
    fn a_reachable_trait_of_a_non_crb_race_is_observed_by_the_probe() {
        let reachable = probe_reachable_race_traits(&probe_root());
        assert_eq!(
            reachable.get(&("mc_abilities_race.lst".to_string(), 16)).map(String::as_str),
            Some("monster_codex"),
            "Duergar ~ Ironskinned reaches a player (reach_gate + on-screen, SD-29 \
             progress.md) and must be observed as reachable"
        );
        // The half the repair must not break: a record the OLD probe already
        // grounded stays grounded.
        assert_eq!(
            reachable.get(&("arg_abilities_race.lst".to_string(), 53)).map(String::as_str),
            Some("advanced_race_guide"),
            "Saltbeard ~ Dwarf ~ Greed is `grounded` in the shipped inventory and must stay so"
        );
    }

    /// The probe grounds on *applicability*, not on presence on disk.
    /// `Oversized Goblin` is ingested, loaded, and still unreachable:
    /// it carries no readable gate, so `race_resolver::classify` leaves it
    /// `TraitRole::Unclassified`, the role that never applies. It has a
    /// standing `OPEN_FINDINGS` entry naming its remedy for exactly that
    /// reason, and a probe that called it grounded would contradict the
    /// gate the same repo already ships.
    #[test]
    fn a_loaded_record_the_resolver_can_never_apply_is_not_observed_as_reachable() {
        let reachable = probe_reachable_race_traits(&probe_root());
        assert!(
            !reachable.contains_key(&("mc_abilities_race.lst".to_string(), 31)),
            "Oversized Goblin never applies (TraitRole::Unclassified) and must not ground"
        );
    }

    /// Every book the probe observes must resolve to an engine book, or its
    /// records ground against nothing however reachable they are.
    ///
    /// This is the assertion that would have caught the defect it was written
    /// for: Bestiary 1's 108 race-trait records were loaded, applied, and
    /// reachable, and every one of them still reported `not-ingested` —
    /// silently — because `data/corpus/beastiary` does not spell its book the
    /// way `corpus_dir_for` does. It also pins that `beastiary` is the ONLY
    /// book needing the alias, so a second divergence fails here rather than
    /// being absorbed.
    #[test]
    fn every_corpus_book_with_race_traits_resolves_to_an_engine_book() {
        let reachable = probe_reachable_race_traits(&probe_root());
        let books: BTreeSet<&str> = reachable.values().map(String::as_str).collect();
        assert!(!books.is_empty(), "the probe observed no books at all");
        for book in &books {
            assert!(
                engine_book_for_corpus_dir(book).is_some(),
                "corpus book {book} resolves to no engine book, so every one of its reachable \
                 race traits would report not-ingested"
            );
        }
        let aliased: BTreeSet<&str> =
            books.iter().copied().filter(|b| engine_book_for(b).is_none()).collect();
        assert_eq!(
            aliased,
            BTreeSet::from(["beastiary"]),
            "exactly one corpus directory is spelled differently from its PCGen source \
             directory; a second one is a new divergence, not a thing to absorb"
        );
    }

    /// The join is on the record's own source coordinates, never on its
    /// name. A race trait's display name is not unique corpus-wide -- the
    /// whole reason the name-coincidence defect above existed -- so the
    /// probe keys on `(source file basename, source line)`, which is an
    /// identity the ingest writes verbatim from the `.lst` row.
    #[test]
    fn the_probe_keys_on_source_coordinates_so_two_books_sharing_a_trait_name_stay_distinct() {
        let reachable = probe_reachable_race_traits(&probe_root());
        let mut books: BTreeSet<&str> = BTreeSet::new();
        for book in reachable.values() {
            books.insert(book.as_str());
        }
        assert!(
            books.len() >= 3,
            "the probe observed reachable records from only {books:?}; the loaded corpus \
             spans more books than that"
        );
        // No two entries share a source coordinate: the map's own key type
        // guarantees it, so what this asserts is that the probe found a
        // real population rather than silently collapsing to nothing.
        assert!(
            reachable.len() >= 200,
            "the probe observed only {} reachable race traits; 336 are on disk",
            reachable.len()
        );
    }

    // -----------------------------------------------------------------
    // The text-complete rung (SD31-D7-PROSE-001, Decision 7's done-bar).
    //
    // A zero-magnitude race trait the race corpus applies is `grounded`
    // above -- true, but not the strongest honest word for a record with
    // NO magnitude to ground. These tests pin the promotion to
    // `text-complete` (Decision 7's `done` bar) when, and ONLY when, the
    // SAME render function `race_trait_picker::build_menu` calls to serve
    // the real player-facing screen produces real, non-empty text.
    // -----------------------------------------------------------------

    fn race_trait_unit(file: &str, line: usize, key: &str, magnitude_token_count: usize) -> CorpusUnit {
        CorpusUnit {
            book: "advanced_race_guide".to_string(),
            source_book: "advanced_race_guide".to_string(),
            kind: Kind::RaceTrait,
            key: key.to_string(),
            name: key.to_string(),
            origin: Origin::Declared,
            provenance: Provenance { file: file.to_string(), line },
            magnitude_token_count,
            type_facet: None,
            visible: true,
        }
    }

    /// The proof case, against the REAL corpus and the REAL render path --
    /// not a synthetic fixture. `Feral ~ Languages` (`arg_abilities_race.lst`
    /// line 606) is one of the 146 zero-magnitude, race-corpus-applied
    /// records this rung exists for (re-derived 2026-08-16,
    /// `docs/work-inventory.json`): `magnitude_token_count == 0`,
    /// `wiring_class: display`, and — before this change — `status:
    /// grounded`, which `doneness_verdict` caps at `held` for `display`.
    #[test]
    fn a_real_zero_magnitude_applied_race_trait_reaches_text_complete_with_real_rendered_text() {
        let facts = EngineFacts {
            race_trait_probe: probe_race_trait_corpus(&probe_root()),
            ..Default::default()
        };
        let unit = race_trait_unit("arg_abilities_race.lst", 606, "Feral ~ Languages", 0);
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_eq!(verdict.status, "text-complete");
        assert_eq!(
            verdict.evidence,
            "race_trait_applied_by_the_race_corpus_and_rendered_with_real_text"
        );
        // The rendered text must be the REAL corpus prose, not invented
        // here -- pinned against the row's own DESC: token.
        let rendered = facts.race_trait_rendered_description(&unit).unwrap();
        assert!(
            rendered.contains("Feral orcs begin play speaking no languages"),
            "expected the real DESC: text, got {rendered:?}"
        );
    }

    /// PROVE THE RUNG CAN FAIL, case 1: a record the race corpus applies but
    /// whose rendered description comes back empty must NOT read
    /// `text-complete` -- Decision 7's condition 3 is "the prose is
    /// available to print", and empty prose is not that. Falls back to the
    /// pre-existing `grounded` verdict (still `held`, never `done`), not a
    /// new failure mode.
    #[test]
    fn an_applied_race_trait_with_an_empty_rendered_description_does_not_read_text_complete() {
        let coordinate = ("empty_desc_race.lst".to_string(), 1);
        let mut facts = EngineFacts::default();
        facts.race_trait_probe.loaded.insert(coordinate.clone());
        facts.race_trait_probe.reachable.insert(coordinate.clone(), "advanced_race_guide".to_string());
        facts.race_trait_probe.rendered.insert(coordinate, String::new());
        let unit = race_trait_unit("empty_desc_race.lst", 1, "Empty ~ Trait", 0);
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "grounded");
    }

    /// PROVE THE RUNG CAN FAIL, case 2: a record the race corpus applies but
    /// whose rendered description was never populated at all (no entry in
    /// `rendered`, distinct from an empty string -- the "we never resolved a
    /// render for this coordinate" case) must also not read `text-complete`.
    #[test]
    fn an_applied_race_trait_with_no_rendered_entry_at_all_does_not_read_text_complete() {
        let coordinate = ("no_render_race.lst".to_string(), 1);
        let mut facts = EngineFacts::default();
        facts.race_trait_probe.loaded.insert(coordinate.clone());
        facts.race_trait_probe.reachable.insert(coordinate, "advanced_race_guide".to_string());
        // deliberately no `rendered` entry
        let unit = race_trait_unit("no_render_race.lst", 1, "No Render ~ Trait", 0);
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "grounded");
    }

    /// PROVE THE RUNG CAN FAIL, case 3: a record that carries a real
    /// magnitude token (`text_only` false) must never read `text-complete`
    /// via this rung even with a perfectly real rendered description --
    /// condition 2 ("nothing to compute") is not met.
    #[test]
    fn a_magnitude_bearing_applied_race_trait_never_reads_text_complete() {
        let coordinate = ("has_magnitude_race.lst".to_string(), 1);
        let mut facts = EngineFacts::default();
        facts.race_trait_probe.loaded.insert(coordinate.clone());
        facts.race_trait_probe.reachable.insert(coordinate.clone(), "advanced_race_guide".to_string());
        facts
            .race_trait_probe
            .rendered
            .insert(coordinate, "You gain a +2 bonus to something real.".to_string());
        let unit = race_trait_unit("has_magnitude_race.lst", 1, "Has Magnitude ~ Trait", 1);
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "grounded");
    }

    /// PROVE THE RUNG CAN FAIL, case 4 (SD31-W5-INTEGRATE-001, confirmed
    /// adversarial finding): a record whose rendered description is the PI
    /// redaction placeholder `[redacted PI]` must NOT read `text-complete`.
    /// The string is non-empty, so the old `!rendered.trim().is_empty()`
    /// gate wrongly accepted it -- a player sees the literal marker, never
    /// the rulebook's prose, so Decision 7's condition 3 ("the prose is
    /// available to print... on the character sheet") is not met. Reuses
    /// the exact refusal `is_real_description_value` already applies to
    /// every other text_only->text-complete branch.
    #[test]
    fn an_applied_race_trait_whose_rendered_description_is_the_pi_redaction_marker_does_not_read_text_complete(
    ) {
        let coordinate = ("pi_redacted_race.lst".to_string(), 1);
        let mut facts = EngineFacts::default();
        facts.race_trait_probe.loaded.insert(coordinate.clone());
        facts.race_trait_probe.reachable.insert(coordinate.clone(), "core_essentials".to_string());
        facts
            .race_trait_probe
            .rendered
            .insert(coordinate, "[redacted PI]".to_string());
        let unit = race_trait_unit("pi_redacted_race.lst", 1, "Tiefling ~ Daemon-Spawn", 0);
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "grounded");
    }
}

// ---------------------------------------------------------------------------
// SD31-D7-PROSE-002: extend Decision 7's done-bar to `monster_ability`.
//
// Exactly the shape `race_trait_grounding_tests` above proves for
// `Kind::RaceTrait`, transplanted to `Kind::MonsterAbility`: a text_only
// (zero-magnitude) ability the engine's own chassis table holds is
// `grounded` today, which `doneness_verdict` caps at `held` for `display`
// wiring_class -- the exact structural blocker `decisions.md §7`'s
// "structural blocker" correction names. `MonsterAbilityRecord::description`
// (`monster_chassis.rs`) is parsed from the SAME `DESC:` token
// `closure_has_real_description` already checks, and reaches a real,
// player-facing screen unconditionally for every registered ability:
// `monster_catalog::serve_ability_description` -> `MonsterAbilityDto.
// description` -> `MonsterCatalogScreen.tsx`'s `ability.description`
// paragraph ("Only ability rows WITH an owner are registered" --
// `monster_chassis.rs`'s own module doc -- so a held ability is *always*
// shown under some monster's catalog entry; there is no held-but-unshown
// case). No new render path is built here; this only asks the SAME
// `has_real_description` closure check every other kind's rung already
// uses to gate the SAME promotion.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod monster_ability_text_complete_rung_tests {
    use super::*;

    fn monster_ability_unit(
        source_book: &str,
        file: &str,
        line: usize,
        key: &str,
        magnitude_token_count: usize,
    ) -> CorpusUnit {
        CorpusUnit {
            book: source_book.to_string(),
            source_book: source_book.to_string(),
            kind: Kind::MonsterAbility,
            key: key.to_string(),
            name: key.to_string(),
            origin: Origin::Declared,
            provenance: Provenance { file: file.to_string(), line },
            magnitude_token_count,
            type_facet: None,
            visible: true,
        }
    }

    fn facts_holding(engine_book: &'static str, key: &str) -> EngineFacts {
        let mut facts = EngineFacts::default();
        facts
            .chassis_monster_ability_keys
            .entry(engine_book)
            .or_default()
            .insert(key.to_lowercase());
        facts
    }

    /// The proof case, against the REAL corpus, not a synthetic fixture:
    /// `Air Elemental ~ Air Mastery` (`b1_abilities_race.lst` line 585,
    /// `docs/work-inventory.json` id
    /// `bestiary:monster_ability:air_elemental_air_mastery`) is
    /// `magnitude_token_count == 0`, `wiring_class: display`, and — before
    /// this change — `status: grounded`, capped at `held`. Its corpus row
    /// carries a real `DESC:` value ("Airborne creatures take a -1 penalty
    /// on attack and damage rolls against an air elemental."), which is the
    /// same text `MonsterAbilityRecord::description` serves onto the wire.
    #[test]
    fn a_real_zero_magnitude_held_monster_ability_reaches_text_complete_with_real_description() {
        let facts = facts_holding("bestiary_1", "Air Elemental ~ Air Mastery");
        let unit = monster_ability_unit(
            "bestiary",
            "b1_abilities_race.lst",
            585,
            "Air Elemental ~ Air Mastery",
            0,
        );
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_eq!(verdict.status, "text-complete");
        assert_eq!(
            verdict.evidence,
            "monster_ability_held_and_corpus_record_carries_real_description"
        );
    }

    /// PROVE THE RUNG CAN FAIL, case 1 (empty/absent description): a held,
    /// text_only ability whose closure carries no real `DESC:` value must
    /// stay `grounded`, never `text-complete` -- there is nothing for
    /// `MonsterCatalogScreen` to show, and the screen's own fallback
    /// paragraph ("...carries no rules text") says so honestly. This is the
    /// pre-existing behaviour, not a new failure mode.
    #[test]
    fn a_held_monster_ability_with_no_real_description_does_not_read_text_complete() {
        let facts = facts_holding("bestiary_1", "No Desc ~ Ability");
        let unit =
            monster_ability_unit("bestiary", "b1_abilities_race.lst", 1, "No Desc ~ Ability", 0);
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, false);
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "grounded");
    }

    /// PROVE THE RUNG CAN FAIL, case 2 (magnitude-bearing): an ability that
    /// carries a real magnitude token (`text_only` false) must never read
    /// `text-complete` via this rung even with a real description --
    /// condition 2 ("nothing to compute") is not met, and the unit's own
    /// magnitude is the thing owed a wiring path, not a description credit.
    #[test]
    fn a_magnitude_bearing_held_monster_ability_never_reads_text_complete() {
        let facts = facts_holding("bestiary_1", "Has Magnitude ~ Ability");
        let unit = monster_ability_unit(
            "bestiary",
            "b1_abilities_race.lst",
            2,
            "Has Magnitude ~ Ability",
            1,
        );
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "grounded");
    }

    /// PROVE THE RUNG CAN FAIL, case 3 (not held at all): an ability the
    /// engine's chassis table does not hold must stay `not-ingested`
    /// regardless of `has_real_description` -- a real description on a
    /// record nothing loads is not condition 3 ("the sheet must render
    /// it"), it is an unreachable string.
    #[test]
    fn an_unheld_monster_ability_does_not_read_text_complete_even_with_a_real_description() {
        let facts = facts_holding("bestiary_1", "Some Other ~ Ability");
        let unit = monster_ability_unit(
            "bestiary",
            "b1_abilities_race.lst",
            3,
            "Not Held ~ Ability",
            0,
        );
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "not-ingested");
    }

    /// PROVE THE RUNG CAN FAIL, case 4 (the flat-magnitude conservative
    /// default, `OPEN-ISSUES.md` rows 69/87): the one unit row 69's own
    /// hand-verified sample already confirmed carries a flat, non-scaling
    /// numeric printed only in prose within `monster_ability` --
    /// `bestiary_2:monster_ability:devilfish_water_dependency` -- must NOT
    /// read `text-complete` via this rung, even though it is otherwise
    /// text_only, held, and carries a real description. This is the
    /// operator-ruling-pending exclusion, not the general refusal shape the
    /// other cases above prove.
    #[test]
    fn the_named_flat_magnitude_monster_ability_does_not_read_text_complete_pending_ruling() {
        let facts = facts_holding("bestiary_2", "Devilfish ~ Water Dependency");
        let unit = monster_ability_unit(
            "bestiary_2",
            "b2_abilities_race.lst",
            409,
            "Devilfish ~ Water Dependency",
            0,
        );
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "grounded");
    }

    /// PROVE THE RUNG CAN FAIL, case 5 (CONFIRMED integration-cycle
    /// adversarial finding): a held, text_only ability whose OWN row
    /// declares a character-specific `description_variables` argument
    /// (e.g. `13+Con`, `CONSCORE`, `BreathWeaponDC`) must NOT read
    /// `text-complete` -- `serve_ability_description` renders with an
    /// EMPTY `PcgenDisplayValues`, so the argument is silently dropped and
    /// the player sees a hole in the sentence
    /// ("The psicrystal has power resistance ."). The real
    /// `ultimate_psionics:monster_ability:psicrystal_power_resistance`
    /// shape.
    #[test]
    fn a_held_monster_ability_whose_row_declares_a_description_variable_does_not_read_text_complete(
    ) {
        let mut facts = facts_holding("ultimate_psionics", "Psicrystal ~ Power Resistance");
        facts
            .chassis_monster_ability_unresolved_desc_keys
            .entry("ultimate_psionics")
            .or_default()
            .insert("psicrystal ~ power resistance".to_string());
        let unit = monster_ability_unit(
            "ultimate_psionics",
            "up_abilities_race.lst",
            77,
            "Psicrystal ~ Power Resistance",
            0,
        );
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "grounded");
    }

    /// PROVE THE RUNG CAN FAIL, case 6: the second shape of the same
    /// finding -- a literal, unresolved `%N` in the raw `DESC:` text even
    /// though `description_variables` is empty (a malformed citation whose
    /// argument tail never reached the record). Must ALSO refuse, or the
    /// player sees a raw `%1` on screen instead of a number.
    #[test]
    fn a_held_monster_ability_whose_description_leaks_a_literal_percent_digit_does_not_read_text_complete(
    ) {
        let mut facts = facts_holding("bestiary_1", "Pixie ~ Sleep");
        facts
            .chassis_monster_ability_unresolved_desc_keys
            .entry("bestiary_1")
            .or_default()
            .insert("pixie ~ sleep".to_string());
        let unit =
            monster_ability_unit("bestiary", "b1_abilities_race.lst", 12, "Pixie ~ Sleep", 0);
        let verdict = classify(&unit, &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "text-complete");
        assert_eq!(verdict.status, "grounded");
    }

    /// Direct unit test of the pure predicate function backing both cases
    /// above, against synthetic `MonsterAbilityRecord`s -- proves both
    /// shapes (declared variable list, bare `%N` with no list) independently
    /// and confirms a clean record (neither shape) is NOT flagged.
    #[test]
    fn monster_ability_desc_leaks_unresolved_argument_catches_both_shapes() {
        let declared_var = monster_chassis::MonsterAbilityRecord {
            key: "x",
            name: "x",
            facet: monster_chassis::MonsterAbilityFacet::SpecialQuality,
            delivery: None,
            traits: &[],
            description: Some("The creature has power resistance %1."),
            description_variables: &["13+Con"],
            source_page: None,
            owners: &[],
            source_file: "x.lst",
            source_line: 1,
        };
        assert!(monster_ability_desc_leaks_unresolved_argument(&declared_var));

        let bare_percent = monster_chassis::MonsterAbilityRecord {
            description: Some("Sleep; the target must succeed on a DC %1 Will save."),
            description_variables: &[],
            ..declared_var
        };
        assert!(monster_ability_desc_leaks_unresolved_argument(&bare_percent));

        let clean = monster_chassis::MonsterAbilityRecord {
            description: Some("A creature takes a -1 penalty on attack rolls."),
            description_variables: &[],
            ..declared_var
        };
        assert!(!monster_ability_desc_leaks_unresolved_argument(&clean));
    }
}

#[cfg(test)]
mod companion_ability_file_classification_tests {
    use super::*;

    /// A file whose basename carries BOTH `_abilities_race` and a
    /// companion/familiar marker holds abilities of a *companion creature*,
    /// not racial traits of a player race. Two such files exist corpus-wide
    /// (re-derived 2026-08-11 from `docs/work-inventory.json`:
    /// `isi_abilities_race_companion.lst` 9 units,
    /// `b4_abilities_race_ce_companion.lst` 2 units) and both were typed
    /// `race_trait` purely because `_abilities_race` is a substring tested
    /// before the companion markers. `isi_abilities_race_companion.lst`'s
    /// rows are Clockwork Spy / Clockwork Familiar construct abilities
    /// (`CATEGORY:Special Ability TYPE:ClockworkSpyRacialAbility...`).
    #[test]
    fn an_abilities_race_file_marked_companion_or_familiar_is_a_companion_file() {
        assert_eq!(file_kind("isi_abilities_race_companion.lst"), Some(Kind::Companion));
        assert_eq!(file_kind("b4_abilities_race_ce_companion.lst"), Some(Kind::Companion));
    }

    /// The narrowing must not swallow the genuine race-trait files, nor
    /// disturb the companion files that already classified correctly.
    #[test]
    fn plain_abilities_race_files_remain_race_traits() {
        assert_eq!(file_kind("mc_abilities_race.lst"), Some(Kind::RaceTrait));
        assert_eq!(file_kind("arg_abilities_race.lst"), Some(Kind::RaceTrait));
        // `_abilities_familiar_race` never matched `_abilities_race` in the
        // first place; it must still land on Companion.
        assert_eq!(file_kind("b2_abilities_familiar_race.lst"), Some(Kind::Companion));
        assert_eq!(file_kind("ce_abilities_familiar_race_cr.lst"), Some(Kind::Companion));
        assert_eq!(file_kind("isi_abilities_companion.lst"), Some(Kind::Companion));
    }
}

/// The `units[].id` uniqueness contract.
///
/// These tests exist because the contract `docs/work-inventory.json` prints at
/// the top of itself was, for `id`, false — and false in the one way that
/// corrupts measurement rather than merely annoying a reader. Twenty-nine ids
/// in the corpus-wide run carried two units each; twenty-seven of those pairs
/// disagreed about `wiring_class`. Any before/after comparison keyed on `id`
/// silently kept one row on one side and the other row on the other, which is
/// how a nineteen-unit `computed -> display` transition appeared in a diff of a
/// change that could not structurally cause one
/// (`docs/retro/events/wiring-classifier.jsonl`). The contract is now enforced
/// by [`unit_id`], by a hard exit in the generator, and here.
#[cfg(test)]
mod unit_id_uniqueness_tests {
    use super::*;

    /// Real colliding key pairs, read off the corpus-wide run that exposed
    /// them. Each entry is `(book, kind, key_a, key_b)`, and every pair shares
    /// one `slug` — which is the whole defect.
    const REAL_COLLISIONS: &[(&str, Kind, &str, &str)] = &[
        (
            "ultimate_psionics",
            Kind::ClassFeature,
            "Path Skill Acrobatics",
            "Path Skill ~ Acrobatics",
        ),
        ("core_rulebook", Kind::EquipmentModifier, "MITHRAL_ITEM", "Mithral (Item)"),
        (
            "core_rulebook",
            Kind::EquipmentModifier,
            "Intelligent Item Purpose (Slay All)",
            "Intelligent Item ~ Purpose / Slay All",
        ),
        (
            "advanced_race_guide",
            Kind::RaceTrait,
            "Half-Elf ~ Drow Blooded",
            "Half-Elf ~ Drow-Blooded",
        ),
        (
            "ultimate_combat",
            Kind::ClassFeature,
            "Master Of Many Styles ~ Perfect Style",
            "Master of Many Styles ~ Perfect Style",
        ),
        (
            "advanced_class_guide",
            Kind::ClassFeature,
            "Arcanist School Void",
            "Arcanist School ~ Void",
        ),
    ];

    /// The root cause, stated as a test rather than as a comment: `slug` is
    /// lossy, so it is NOT an identity, and an id built from it alone cannot be
    /// unique. If this ever starts failing, `slug` has been changed and the
    /// disambiguation below may no longer be reaching the cases it was built
    /// for — read `unit_id`'s doc comment before touching either.
    #[test]
    fn slug_collapses_genuinely_distinct_corpus_keys() {
        for (_, _, a, b) in REAL_COLLISIONS {
            assert_ne!(a, b, "these are two different corpus keys");
            assert_eq!(
                slug(a),
                slug(b),
                "slug() is expected to collapse {a:?} and {b:?} onto one string"
            );
        }
    }

    /// The contract itself: two distinct corpus keys never share an id.
    #[test]
    fn colliding_keys_get_distinct_ids() {
        for (book, kind, a, b) in REAL_COLLISIONS {
            let id_a = unit_id(book, *kind, a, true);
            let id_b = unit_id(book, *kind, b, true);
            assert_ne!(
                id_a, id_b,
                "{a:?} and {b:?} in {book}/{} must not share an id",
                kind.id()
            );
        }
    }

    /// The tie-break rule's defensibility, and the reason it is a key digest
    /// rather than an ordinal: a unit's id is a function of that unit's own
    /// identity and of nothing else. Ingesting a new row that happens to
    /// collide with an existing one must not renumber the existing one, and
    /// removing a sibling must not renumber its survivor. An `__1`/`__2`
    /// ordinal fails exactly this test.
    #[test]
    fn a_units_id_does_not_depend_on_which_siblings_it_collides_with() {
        let key = "Path Skill ~ Acrobatics";
        let alone = unit_id("ultimate_psionics", Kind::ClassFeature, key, true);
        // Every other colliding key in the corpus, real or hypothetical, is
        // irrelevant to this unit's id -- there is no argument to `unit_id`
        // through which a sibling could reach it.
        for sibling in ["Path Skill Acrobatics", "Path_Skill_Acrobatics", "path skill acrobatics"] {
            assert_eq!(slug(sibling), slug(key), "test setup: {sibling:?} must collide");
            let again = unit_id("ultimate_psionics", Kind::ClassFeature, key, true);
            assert_eq!(alone, again);
        }
    }

    /// Blast radius. A unit whose slug is unique keeps the exact id it carried
    /// before this fix existed, so no consumer owes a re-pin it did not earn.
    #[test]
    fn a_non_colliding_unit_keeps_the_unsuffixed_id() {
        assert_eq!(
            unit_id("core_rulebook", Kind::Feat, "Power Attack", false),
            "core_rulebook:feat:power_attack"
        );
        assert_eq!(
            unit_id("advanced_race_guide", Kind::RaceTrait, "Half-Elf ~ Drow-Blooded", false),
            "advanced_race_guide:race_trait:half_elf_drow_blooded"
        );
    }

    /// `__` is claimed as the delimiter on the grounds that `slug` can never
    /// produce it — it collapses every run of non-alphanumerics to a single
    /// `_`. If that ever stops being true, a suffixed id becomes ambiguous with
    /// an unsuffixed one and this test is the alarm.
    #[test]
    fn no_slug_can_contain_the_double_underscore_delimiter() {
        let adversarial = [
            "Path Skill ~ Acrobatics",
            "A__B",
            "A -- B",
            "  leading and trailing  ",
            "!!!",
            "Intelligent Item ~ Purpose / Slay All",
            "Weapon (+1) ~ Flaming / Burst",
        ];
        for s in adversarial {
            assert!(
                !slug(s).contains("__"),
                "slug({s:?}) = {:?} must not contain the reserved delimiter",
                slug(s)
            );
        }
        for (_, _, a, b) in REAL_COLLISIONS {
            assert!(!slug(a).contains("__"));
            assert!(!slug(b).contains("__"));
        }
    }

    /// The digest algorithm is pinned, because every disambiguated id is built
    /// from it and a silent change to it would rewrite those ids wholesale and
    /// break the byte-equality contract in the least visible way available.
    /// These are FNV-1a 64's own published reference vectors; a
    /// re-implementation that passes them is the specified function.
    #[test]
    fn key_digest_is_fnv1a_64_against_its_published_vectors() {
        assert_eq!(key_digest(""), "cbf29ce484222325");
        assert_eq!(key_digest("a"), "af63dc4c8601ec8c");
        assert_eq!(key_digest("foobar"), "85944171f73967e8");
    }

    /// The digest is a pure function of the key's bytes: same key, same
    /// sixteen characters, every call. Stated separately from the vectors
    /// because this is the property `unit_id` actually leans on.
    #[test]
    fn key_digest_is_stable_and_full_width() {
        for (_, _, a, b) in REAL_COLLISIONS {
            for key in [a, b] {
                let first = key_digest(key);
                assert_eq!(first.len(), 16, "digest for {key:?} must be 16 hex chars");
                assert!(first.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
                assert_eq!(first, key_digest(key));
            }
        }
    }

    /// End to end over the real collision set: mint every id the way the
    /// generator mints it and assert the whole set is unique. This is the
    /// property the generator's own hard exit enforces at run time.
    #[test]
    fn the_real_collision_set_mints_a_fully_unique_id_set() {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for (book, kind, a, b) in REAL_COLLISIONS {
            for key in [a, b] {
                assert!(
                    ids.insert(unit_id(book, *kind, key, true)),
                    "duplicate id minted for {key:?} in {book}/{}",
                    kind.id()
                );
            }
        }
        assert_eq!(ids.len(), REAL_COLLISIONS.len() * 2);
    }
}

/// SD-32 `spell-consumer-delta-probe`: the spell probe's own proofs.
///
/// This module is the answer to the `e14_harness_tests` module's closing note
/// ("F1: spell probe -- NOT implemented, and this is the finding"). That note
/// is superseded, not contradicted: its blocker was that
/// `spellbook::compute_spellbook_coverage` reached no surface the player
/// reads. `epic-31-spell-wiring` wired it to one. See the block comment above
/// [`probe_spell_key`].
#[cfg(test)]
mod spell_probe_tests {
    use super::*;
    use codex::rules_core::source_content::SourcePackageContent;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// A scratch book directory holding hand-written Shape B v1 spell JSON,
    /// cleaned up on drop. Loaded through the SAME `load_spell_corpus` the
    /// probe uses in production, so a negative case is exercised against the
    /// real loading path rather than a hand-assembled package.
    struct ScratchSpellBook {
        root: PathBuf,
    }

    impl ScratchSpellBook {
        fn new(name: &str, records: &[(&str, Option<&str>)]) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_spell_probe_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(root.join("spell")).unwrap();
            for (i, (key, school)) in records.iter().enumerate() {
                let school_json = match school {
                    Some(s) => format!("\"{s}\""),
                    None => "null".to_string(),
                };
                std::fs::write(
                    root.join("spell").join(format!("record_{i}.json")),
                    format!(
                        "{{\"data\":{{\"key\":{},\"school\":{school_json}}}}}",
                        q(key)
                    ),
                )
                .unwrap();
            }
            ScratchSpellBook { root }
        }

        fn corpus(&self) -> SourcePackageContent<'static> {
            let roots = [BookCorpusRoot { book_id: "scratch", dir: &self.root }];
            load_spell_corpus(&roots)
        }
    }

    impl Drop for ScratchSpellBook {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn fixture() -> CharacterInput {
        let path = repo_root().join(FIXTURE_RELATIVE_PATH);
        let text = std::fs::read_to_string(&path).expect("the shared pilot fixture is readable");
        load_character_input_fixture(&text)
            .character_input
            .expect("the shared pilot fixture loads")
    }

    fn crb_corpus() -> SourcePackageContent<'static> {
        let dir = repo_root().join("data/corpus/core_rulebook");
        let roots = [BookCorpusRoot { book_id: "core_rulebook", dir: &dir }];
        load_spell_corpus(&roots)
    }

    // ----- The delta's baseline is real -----

    /// The "delta" in consumer-delta. The probe's own posture, with NO spell
    /// selected, must carry no save DC at all — otherwise every DC it later
    /// observes would be unattributable to the spell that was added.
    #[test]
    fn the_probes_baseline_posture_carries_no_save_dc_at_all() {
        let corpus = crb_corpus();
        let baseline =
            compute_spellbook_coverage(&spell_probe_input(&fixture(), "class:wizard", None), &corpus);
        assert!(
            PilotSpellbookViewModel::from_coverage(&baseline).is_none(),
            "the no-spell baseline must project no spellbook block at all: {:?}",
            baseline.spell_save_dc
        );
    }

    // ----- Positive: a real magnitude on the surface the sheet renders -----

    /// `Shield` is a real CRB Abjuration record, 1st level, on the real Wizard
    /// spell list, present in `data/corpus/core_rulebook/spell/`. Selecting it
    /// alone must put `10 + 1 + 4 = 15` on
    /// `PilotSpellbookViewModel.spell_save_dc` — the field
    /// `pf1_adapter::resolve_unified_pilot_snapshot` puts on the snapshot and
    /// `CharacterSheet.tsx` renders.
    #[test]
    fn the_probe_observes_a_real_spells_save_dc_on_the_surface_the_sheet_renders() {
        let corpus = crb_corpus();
        let outcome = probe_spell_key(&fixture(), "Shield", &corpus, RuleSetId::Crb);
        assert_eq!(
            outcome,
            SpellProbeOutcome::Wired { class_id: "class:wizard", level: 1, dc: 15 },
            "Shield must produce a wizard save DC of 10 + level 1 + modifier 4"
        );
    }

    // ----- Validation against answers recorded independently in this repo ---

    /// The exercise this program's own recorded lesson demands: reproduce
    /// answers already recorded elsewhere before trusting the instrument on
    /// anything new.
    ///
    /// `pilot_compute.rs` computes the same PF1 DC formula in a completely
    /// separate implementation and emits it as
    /// `class_chassis.wizard.spell_save_dc.spell_level_<N>`. That ladder is
    /// this probe's oracle. The two are compared on the part that is actually
    /// in dispute — how much the SPELL'S OWN LEVEL moves the number — by
    /// subtracting each side's own casting-ability modifier, so the comparison
    /// does not silently depend on the two paths deriving the same modifier
    /// (the chassis applies racial bonuses; the probe pins a raw score).
    ///
    /// A probe that reported a constant, or that read some other spell's
    /// level, fails this at the first rung.
    #[test]
    fn probe_dcs_reproduce_the_independently_computed_chassis_dc_ladder() {
        let fixture = fixture();
        let corpus = crb_corpus();

        // The chassis's own ladder, harvested from a real computation.
        let computation = compute_pilot_base_chassis(&class_sweep_input(&fixture, "wizard", 20));
        let mut chassis: BTreeMap<u8, i16> = BTreeMap::new();
        for e in &computation.explanations {
            if let Some(rest) = e.id.strip_prefix("class_chassis.wizard.spell_save_dc.spell_level_")
                && let Ok(level) = rest.parse::<u8>()
            {
                chassis.insert(level, e.value);
            }
        }
        assert!(
            chassis.len() >= 5,
            "the wizard chassis must publish a DC ladder to compare against: {chassis:?}"
        );
        // The chassis's own modifier, read off its own 1st-level rung rather
        // than assumed: dc = 10 + 1 + modifier.
        let chassis_modifier = chassis[&1] - 11;

        // One real, unambiguous Wizard spell per level 1-5, each in
        // `crb::spell_list::SPELL_LIST` at that level.
        let anchors: &[(&str, u8)] = &[
            ("Alarm", 1),
            ("Acid Arrow", 2),
            ("Arcane Sight", 3),
            ("Arcane Eye", 4),
            ("Cone of Cold", 5),
        ];
        for &(key, level) in anchors {
            let outcome = probe_spell_key(&fixture, key, &corpus, RuleSetId::Crb);
            let SpellProbeOutcome::Wired { dc, level: observed_level, .. } = outcome else {
                panic!("{key} must be wired, got {outcome:?}");
            };
            assert_eq!(observed_level, level, "{key}'s level came from the wrong record");
            assert_eq!(
                i16::from(dc) - SPELL_PROBE_ABILITY_MODIFIER,
                chassis[&level] - chassis_modifier,
                "{key} (level {level}): the probe's level-dependent DC term must equal the \
                 independently computed chassis ladder's"
            );
        }
    }

    /// A magnitude, not a flag: two spells that differ ONLY in level must
    /// produce DCs that differ by exactly that difference. A probe reporting
    /// "some number appeared" passes the positive test above and fails this.
    #[test]
    fn the_observed_dc_moves_with_the_spells_own_level() {
        let fixture = fixture();
        let corpus = crb_corpus();
        let SpellProbeOutcome::Wired { dc: low, level: low_level, .. } =
            probe_spell_key(&fixture, "Alarm", &corpus, RuleSetId::Crb)
        else {
            panic!("Alarm must be wired");
        };
        let SpellProbeOutcome::Wired { dc: high, level: high_level, .. } =
            probe_spell_key(&fixture, "Cone of Cold", &corpus, RuleSetId::Crb)
        else {
            panic!("Cone of Cold must be wired");
        };
        assert_eq!(
            i16::from(high) - i16::from(low),
            i16::from(high_level) - i16::from(low_level),
            "the DC gap must be exactly the level gap"
        );
        assert!(high > low, "a 5th-level spell must not produce the same DC as a 1st-level one");
    }

    // ----- Negatives: what must stay ungrounded -----

    /// `Burning Hands (Acid)` is a real, ingested CRB record with a real
    /// school and level — but no class's own spell list holds it (it is an
    /// elemental-variant row). No player can put it in a spellbook, so any DC
    /// computed for it would be a magnitude nobody can see. It must not be
    /// promoted, and the reason must be the intended one.
    #[test]
    fn the_probe_never_promotes_a_record_no_casting_class_has() {
        assert_eq!(
            probe_spell_key(&fixture(), "Burning Hands (Acid)", &crb_corpus(), RuleSetId::Crb),
            SpellProbeOutcome::NoCastingClassHasIt
        );
        // Control: the base record IS promoted, which proves the refusal above
        // is about the variant row and not about the harness failing to find
        // anything at all.
        assert!(matches!(
            probe_spell_key(&fixture(), "Burning Hands", &crb_corpus(), RuleSetId::Crb),
            SpellProbeOutcome::Wired { .. }
        ));
    }

    /// A key on a real class list whose book corpus holds no such record must
    /// stay ungrounded — the "never ingested here" path, distinct from
    /// "ingested but inert".
    #[test]
    fn the_probe_never_promotes_a_spell_absent_from_the_books_own_corpus() {
        let book = ScratchSpellBook::new("absent", &[("Some Other Spell", Some("Evocation"))]);
        assert_eq!(
            probe_spell_key(&fixture(), "Fireball", &book.corpus(), RuleSetId::Crb),
            SpellProbeOutcome::AbsentFromBookCorpus
        );
    }

    /// A record that resolves against the corpus but that the engine's own
    /// per-school table store does not hold produces no `SpellEffect`, so no
    /// level and no DC. It must stay ungrounded. This is the population every
    /// non-CRB book's spells fall into: the spellbook engine's nine per-school
    /// resolvers read `crb::spell_list::SPELL_LIST` and nothing else.
    #[test]
    fn the_probe_never_promotes_a_record_the_table_store_does_not_hold() {
        // A real APG spell on the real Wizard list, resolved against a corpus
        // that holds it — but absent from the CRB table store.
        let book =
            ScratchSpellBook::new("notable", &[("Aggressive Thundercloud", Some("Evocation"))]);
        assert!(
            crb_wizard_spell_list::wizard_spell_level("Aggressive Thundercloud").is_some(),
            "this anchor must really be on the Wizard list, else the test proves nothing"
        );
        assert!(
            !crb_spell_list::SPELL_LIST.iter().any(|e| e.key == "Aggressive Thundercloud"),
            "this anchor must really be absent from the CRB table store"
        );
        assert_eq!(
            probe_spell_key(&fixture(), "Aggressive Thundercloud", &book.corpus(), RuleSetId::Crb),
            SpellProbeOutcome::NoTableEffect
        );
    }

    /// A record whose `SCHOOL:` the engine does not recognize dispatches to no
    /// school function at all and must stay ungrounded.
    #[test]
    fn the_probe_never_promotes_a_record_with_an_unrecognized_school() {
        let book = ScratchSpellBook::new("badschool", &[("Fireball", Some("Thaumaturgy"))]);
        assert_eq!(
            probe_spell_key(&fixture(), "Fireball", &book.corpus(), RuleSetId::Crb),
            SpellProbeOutcome::SchoolNotRecognized
        );
    }

    /// The `Celestial Shield` discipline, spell side: a book whose own record
    /// happens to share a name with a CRB spell must NOT ground on CRB's
    /// numbers. The magnitude's provenance has to be the claiming unit's own
    /// book.
    ///
    /// The probe's whole non-CRB population sits behind this gate today, and
    /// that is the honest result rather than a hedge: every per-school
    /// resolver stamps `RuleSetId::Crb` because every one of them reads CRB's
    /// table.
    #[test]
    fn the_probe_never_grounds_one_books_unit_on_another_books_table() {
        // A corpus record named exactly like CRB's `Shield`, but presented as
        // the Advanced Player's Guide's own record.
        let book = ScratchSpellBook::new("foreign", &[("Shield", Some("Abjuration"))]);
        let corpus = book.corpus();
        assert_eq!(
            probe_spell_key(&fixture(), "Shield", &corpus, RuleSetId::Apg),
            SpellProbeOutcome::ForeignBookTable,
            "an APG unit must not claim a magnitude CRB's table supplied"
        );
        // Control: the identical record, claimed by the book whose table
        // really answered, IS wired — so the refusal above is the provenance
        // gate and not a broken harness.
        assert!(matches!(
            probe_spell_key(&fixture(), "Shield", &corpus, RuleSetId::Crb),
            SpellProbeOutcome::Wired { .. }
        ));
    }

    // ----- Coverage of the question, not of the answer -----

    /// The probe must ask its question of every spell key the engine catalog
    /// holds for an observable book, for the same reason the equipment probe
    /// must (`the_probe_examines_every_key_the_engine_catalog_holds`): a key
    /// the probe never examines is a unit that can only ever report its
    /// pre-probe status, not because nothing was computed but because nobody
    /// asked. Pins coverage; says nothing about the answer.
    #[test]
    fn the_probe_examines_every_catalog_spell_key_of_every_observable_book() {
        let outcomes = probe_spell_effect_wiring(&fixture(), &repo_root());
        let keys_by_book = probe_spell_keys_by_book();
        let mut expected = 0usize;
        for dir_name in OBSERVABLE_BOOK_DIRS {
            let Some(engine_book) = engine_book_for_corpus_dir(dir_name) else { continue };
            if rule_set_for_engine_book(engine_book).is_none() {
                continue;
            }
            let Some(keys) = keys_by_book.get(engine_book) else { continue };
            for key in keys {
                assert!(
                    outcomes.contains_key(&(engine_book.to_string(), (*key).to_string())),
                    "{engine_book}/{key} was never asked the wiring question"
                );
                expected += 1;
            }
        }
        assert_eq!(outcomes.len(), expected, "the probe asked about keys outside the catalog");
        assert!(expected > 0, "the probe must examine at least one book's keys");
    }

    /// The anti-gaming guard the retracted SD-28-E14-F1 attempt failed: that
    /// version promoted 1,067 of 1,067 targets. A probe that says yes to
    /// everything it examines is measuring nothing. This pins that the probe
    /// discriminates over the real catalog — it does NOT pin a target count,
    /// which would be a bar this cycle could later be tempted to move.
    #[test]
    fn the_probe_refuses_a_real_share_of_the_catalog_it_examines() {
        let outcomes = probe_spell_effect_wiring(&fixture(), &repo_root());
        let wired =
            outcomes.values().filter(|o| matches!(o, SpellProbeOutcome::Wired { .. })).count();
        assert!(wired > 0, "the probe must reach something, else it is not an instrument");
        assert!(
            wired < outcomes.len(),
            "a probe that promotes every key it examines ({wired} of {}) is not observing a \
             delta — that is exactly the retracted 1,067-of-1,067 failure",
            outcomes.len()
        );
    }
}

/// SD-32 `ground-spell-units`: the proofs for wiring the spell probe's verdict
/// into `classify()`.
///
/// The probe-building cycle deliberately stopped short of this — it built and
/// proved the instrument and left `classify()`'s `Kind::Spell` arm
/// byte-identical, so no unit moved. This module is the second half: it pins
/// that a spell reaches `grounded` ONLY on the probe's own book-scoped
/// observation, and that every path that was not observed lands exactly where
/// it landed before.
///
/// Every assertion here is phrased as "this unit legitimately reaches its
/// bar", never "the count rises" (`decisions.md §1`).
#[cfg(test)]
mod spell_grounding_tests {
    use super::*;

    fn spell_unit(book: &str, key: &str) -> CorpusUnit {
        CorpusUnit {
            book: book.to_string(),
            source_book: book.to_string(),
            kind: Kind::Spell,
            key: key.to_string(),
            name: key.to_string(),
            origin: Origin::Declared,
            provenance: Provenance { file: "spells.lst".to_string(), line: 1 },
            magnitude_token_count: 1,
            type_facet: None,
            visible: true,
        }
    }

    /// Facts holding exactly one book's spell catalog entry, with a resolved
    /// level, so the unit under test is `ingested-magnitude` before any
    /// grounding question is asked.
    fn facts_with_catalog_level(book: &'static str, key: &str) -> EngineFacts {
        let mut facts = EngineFacts::default();
        facts.spell_levels.entry(book).or_default().insert(key.to_string(), true);
        facts
    }

    // ----- The promotion, and its exact evidence -----

    /// A spell the probe observed producing a real save DC on the surface the
    /// character sheet renders reaches `grounded`, carrying the probe's own
    /// evidence token — the spell-side sibling of
    /// `equipment_effect_probe_observed_computed_delta`.
    #[test]
    fn a_spell_the_probe_observed_reaches_grounded_on_the_probes_own_evidence() {
        let mut facts = facts_with_catalog_level("core_rulebook", "Shield");
        facts.spell_effect_wired.insert(("core_rulebook".to_string(), "Shield".to_string()));
        let verdict = classify(&spell_unit("core_rulebook", "Shield"), &facts, &BTreeSet::new(), false, true);
        assert_eq!(verdict.status, "grounded");
        assert_eq!(verdict.evidence, "spell_effect_probe_observed_computed_delta");
    }

    // ----- The refusals. These are the load-bearing half. -----

    /// A spell whose level the catalog resolves but which the probe did NOT
    /// observe stays exactly where it was. This is the guard against the
    /// retracted SD-28-E14-F1 shape, where "the engine knows this spell"
    /// silently became "a player sees its magnitude".
    #[test]
    fn a_catalog_spell_the_probe_did_not_observe_stays_ingested_magnitude() {
        let facts = facts_with_catalog_level("core_rulebook", "Alarm");
        let verdict = classify(&spell_unit("core_rulebook", "Alarm"), &facts, &BTreeSet::new(), false, true);
        assert_eq!(verdict.status, "ingested-magnitude");
        assert_eq!(verdict.evidence, "spell_list_entry_with_resolved_level");
    }

    /// The `Celestial Shield` discipline, spell side: the observation is
    /// `(engine_book, key)`, so a book whose OWN record was never probed
    /// cannot claim another book's magnitude just by sharing the name.
    ///
    /// Not hypothetical. `Shield` is a real CRB record, and the probe's own
    /// `the_probe_never_grounds_one_books_unit_on_another_books_table` proves
    /// an APG record of the same name is refused at the probe. This pins that
    /// `classify()` does not undo that refusal.
    #[test]
    fn one_books_observation_never_grounds_another_books_spell_of_the_same_name() {
        let mut facts = facts_with_catalog_level("advanced_players_guide", "Shield");
        facts.spell_effect_wired.insert(("core_rulebook".to_string(), "Shield".to_string()));
        let verdict =
            classify(&spell_unit("advanced_players_guide", "Shield"), &facts, &BTreeSet::new(), false, true);
        assert_eq!(verdict.status, "ingested-magnitude");
    }

    /// A spell with no resolved corpus level is `text-complete`, and an
    /// observation must not overrule that: the probe's verdict is consulted
    /// only on the branch that already had a magnitude to explain.
    #[test]
    fn an_observation_does_not_promote_a_spell_with_no_resolved_level() {
        let mut facts = EngineFacts::default();
        facts
            .spell_levels
            .entry("core_rulebook")
            .or_default()
            .insert("Prestidigitation".to_string(), false);
        facts
            .spell_effect_wired
            .insert(("core_rulebook".to_string(), "Prestidigitation".to_string()));
        let verdict =
            classify(&spell_unit("core_rulebook", "Prestidigitation"), &facts, &BTreeSet::new(), false, true);
        assert_eq!(verdict.status, "text-complete");
    }

    /// A spell absent from the catalog stays `not-ingested` even if some
    /// stale observation names it: the catalog gate runs first, and an
    /// observation can never manufacture ingestion.
    #[test]
    fn an_observation_never_manufactures_ingestion_for_an_uncatalogued_spell() {
        let mut facts = EngineFacts::default();
        facts
            .spell_effect_wired
            .insert(("core_rulebook".to_string(), "Invented Spell".to_string()));
        let verdict =
            classify(&spell_unit("core_rulebook", "Invented Spell"), &facts, &BTreeSet::new(), false, true);
        assert_eq!(verdict.status, "not-ingested");
    }

    // ----- The fact set is exactly the probe's `Wired` verdicts -----

    /// Only `SpellProbeOutcome::Wired` enters the fact set. Every refusal
    /// reason the probe can return is fed in here, so a refusal cannot be
    /// silently treated as a promotion.
    #[test]
    fn only_wired_outcomes_enter_the_fact_set() {
        let refusals = [
            SpellProbeOutcome::NoCastingClassHasIt,
            SpellProbeOutcome::AbsentFromBookCorpus,
            SpellProbeOutcome::SchoolNotRecognized,
            SpellProbeOutcome::NoTableEffect,
            SpellProbeOutcome::ForeignBookTable,
            SpellProbeOutcome::NoSaveDcOnViewModel,
            SpellProbeOutcome::DcDisagreesWithOracle { observed: 99, oracle: 15 },
            SpellProbeOutcome::BaselineAlreadyCarriesADc,
        ];
        let mut outcomes: BTreeMap<(String, String), SpellProbeOutcome> = BTreeMap::new();
        for (i, refusal) in refusals.into_iter().enumerate() {
            outcomes.insert(("core_rulebook".to_string(), format!("Refused {i}")), refusal);
        }
        outcomes.insert(
            ("core_rulebook".to_string(), "Shield".to_string()),
            SpellProbeOutcome::Wired { class_id: "class:wizard", level: 1, dc: 15 },
        );
        let wired = spell_effect_wired_from_outcomes(&outcomes);
        assert_eq!(wired, BTreeSet::from([("core_rulebook".to_string(), "Shield".to_string())]));
    }
}

/// SD-30 `probe-race-and-class`: the class consumer-delta probe's own proofs.
///
/// Built to the same standard as [`spell_probe_tests`], and for the same
/// reason: an instrument that has not been shown to refuse is not an
/// instrument. Every negative below is a way this probe can say "no" to a
/// class the membership test it replaces would have said "yes" to.
#[cfg(test)]
mod class_probe_tests {
    use super::*;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn fixture() -> CharacterInput {
        let path = repo_root().join(FIXTURE_RELATIVE_PATH);
        let text = std::fs::read_to_string(&path).expect("the shared pilot fixture is readable");
        load_character_input_fixture(&text)
            .character_input
            .expect("the shared pilot fixture loads")
    }

    /// Exactly the classes the engine models, built the way `engine_facts`
    /// builds `class_books`' key set.
    fn modelled_classes() -> BTreeSet<String> {
        let mut modelled = BTreeSet::new();
        for id in ClassId::ALL {
            modelled.insert(crb_class_name(*id).to_string());
        }
        for id in ApgClassId::ALL {
            modelled.insert(id.name().to_string());
        }
        for id in AcgClassId::ALL {
            modelled.insert(id.name().to_string());
        }
        modelled
    }

    fn snapshot_for(class_name: &str, level: u8) -> Option<PilotSnapshot> {
        let receipt =
            build_pilot_headless_receipt(&class_probe_input(&fixture(), class_name, level));
        PilotViewModel::from_receipt(&receipt).snapshot
    }

    // ----- Validation against answers stated independently of this engine ---

    /// The exercise this program's recorded lesson demands: reproduce an
    /// answer that is true independently of the code under test before
    /// trusting the instrument on anything new.
    ///
    /// PF1's base-attack-bonus progressions are a published rule, not
    /// something this repo decides: a full-BAB class has BAB equal to its
    /// level, a 3/4 class has `floor(3 * level / 4)`, a 1/2 class has
    /// `floor(level / 2)`. At level 20 that is Fighter +20, Cleric +15,
    /// Wizard +10. If the probe's own posture did not reproduce those three
    /// numbers, every "delta" it later reported would be measuring something
    /// other than the class.
    #[test]
    fn the_probes_posture_reproduces_pf1s_published_bab_progressions() {
        for (class_name, expected_bab) in [("fighter", 20i16), ("cleric", 15), ("wizard", 10)] {
            let snapshot = snapshot_for(class_name, 20)
                .unwrap_or_else(|| panic!("{class_name} at level 20 must project a snapshot"));
            assert_eq!(
                snapshot.base_attack_bonus, expected_bab,
                "{class_name} level 20 must carry PF1's published BAB of +{expected_bab}"
            );
        }
    }

    // ----- The delta's baseline is real -----

    /// The "delta" in consumer-delta. A character with no class levels at all
    /// must not already carry the numbers a class is about to be credited
    /// with — otherwise every delta observed later would be unattributable.
    #[test]
    fn the_classless_baseline_differs_from_every_modelled_class() {
        let fixture = fixture();
        let baseline = class_probe_baseline_numbers(&fixture);
        for class_name in modelled_classes() {
            let Some(snapshot) = snapshot_for(&class_name, 20) else { continue };
            assert_ne!(
                baseline.as_ref(),
                Some(&class_snapshot_numbers(&snapshot)),
                "{class_name} at level 20 must move the rendered numbers off the classless baseline"
            );
        }
    }

    // ----- Positive: a real class puts an attributable magnitude on screen --

    /// Fighter is modelled, reaches `Computed`, and carries explanation
    /// records no other modelled class produces.
    #[test]
    fn the_probe_observes_a_real_classs_magnitude_on_the_rendered_snapshot() {
        let fixture = fixture();
        let modelled = modelled_classes();
        let baseline = class_probe_baseline_numbers(&fixture);
        let outcome = probe_class_name(&fixture, "fighter", &modelled, baseline.as_ref());
        assert!(
            matches!(outcome, ClassProbeOutcome::Wired { .. }),
            "fighter must be observed wired, got {outcome:?}"
        );
    }

    // ----- Negative 1: a class the engine does not model -------------------

    /// The probe can never manufacture ingestion. A name no class enum
    /// carries is refused outright, whatever the corpus says about it.
    #[test]
    fn the_probe_never_promotes_a_class_the_engine_does_not_model() {
        let fixture = fixture();
        let modelled = modelled_classes();
        for absent in ["adept", "commoner", "aristocrat", "psion", "unchained_barbarian"] {
            assert_eq!(
                probe_class_name(&fixture, absent, &modelled, None),
                ClassProbeOutcome::NotModelledByEngine,
                "{absent} is modelled by no class enum and must be refused"
            );
        }
    }

    /// And the wired set is therefore always a subset of the modelled set.
    #[test]
    fn the_wired_set_never_exceeds_the_modelled_set() {
        let modelled = modelled_classes();
        let outcomes = probe_class_effect_wiring(&fixture(), &modelled);
        let wired = class_effect_wired_from_outcomes(&outcomes);
        assert!(
            wired.is_subset(&modelled),
            "the probe promoted a class the engine does not model: {:?}",
            wired.difference(&modelled).collect::<Vec<_>>()
        );
    }

    // ----- Negative 2: attribution by dot-segment, never by substring ------

    /// The corpus-identifier scope collision, in its class form. A substring
    /// test would credit Barbarian with Unchained Barbarian's magnitude and
    /// Rogue with Unchained Rogue's.
    #[test]
    fn a_longer_classs_explanation_never_counts_as_a_shorter_ones() {
        for (id, class_name) in [
            ("class_chassis.unchained_barbarian.replaces", "barbarian"),
            ("class_feature.pu.unchained_rogue.corpus_record.x", "rogue"),
            ("class_spell.acg.bloodrager.spells_known.spell_level_1", "rager"),
        ] {
            assert!(
                !explanation_names_class(id, class_name),
                "{id:?} must not be credited to {class_name:?}"
            );
        }
        // ...while the class that really owns the record still matches.
        assert!(explanation_names_class(
            "class_chassis.unchained_barbarian.replaces",
            "unchained_barbarian"
        ));
        assert!(explanation_names_class(
            "class_spell.acg.bloodrager.spells_known.spell_level_1",
            "bloodrager"
        ));
    }

    // ----- Negative 3: only an observed delta enters the fact set ----------

    /// Every refusal variant is refused, by hand and exhaustively. A future
    /// variant must be classified deliberately, not defaulted in.
    #[test]
    fn only_wired_outcomes_enter_the_class_fact_set() {
        let outcomes = BTreeMap::from([
            (
                "fighter".to_string(),
                ClassProbeOutcome::Wired { level: 1, attributed_explanations: 3 },
            ),
            ("adept".to_string(), ClassProbeOutcome::NotModelledByEngine),
            ("a".to_string(), ClassProbeOutcome::PipelinePanicked),
            ("b".to_string(), ClassProbeOutcome::NeverReachesComputed),
            ("c".to_string(), ClassProbeOutcome::NoSnapshotProjected),
            ("d".to_string(), ClassProbeOutcome::NoSnapshotDeltaVsClasslessBaseline),
            ("e".to_string(), ClassProbeOutcome::NoExplanationAttributedToThisClass),
        ]);
        assert_eq!(
            class_effect_wired_from_outcomes(&outcomes),
            BTreeSet::from(["fighter".to_string()]),
            "exactly the Wired verdict may ground a class unit"
        );
    }

    // ----- The classifier consults the probe, not enum membership ----------

    fn class_unit(book: &str, name: &str) -> CorpusUnit {
        CorpusUnit {
            book: book.to_string(),
            source_book: book.to_string(),
            kind: Kind::Class,
            key: name.to_string(),
            name: name.to_string(),
            origin: Origin::Declared,
            provenance: Provenance { file: "cr_classes.lst".to_string(), line: 1 },
            magnitude_token_count: 1,
            type_facet: None,
            visible: true,
        }
    }

    /// A class the probe observed reaches `grounded` on the probe's own
    /// evidence token.
    #[test]
    fn a_class_the_probe_observed_reaches_grounded_on_the_probes_own_evidence() {
        let mut facts = EngineFacts::default();
        facts.class_books.insert("fighter".to_string(), "core_rulebook");
        facts.class_effect_wired.insert("fighter".to_string());
        let verdict =
            classify(&class_unit("core_rulebook", "Fighter"), &facts, &BTreeSet::new(), false, true);
        assert_eq!(verdict.status, "grounded");
        assert_eq!(
            verdict.evidence,
            "class_probe_observed_computed_delta_on_the_rendered_snapshot"
        );
    }

    /// **The anti-gaming proof.** Enum membership alone no longer grounds a
    /// class. A class the engine models but the probe did NOT observe stays
    /// un-grounded, carrying an evidence token that says exactly that — this
    /// is the case the replaced membership test would have called `grounded`.
    #[test]
    fn a_modelled_class_the_probe_did_not_observe_is_not_grounded() {
        let mut facts = EngineFacts::default();
        facts.class_books.insert("fighter".to_string(), "core_rulebook");
        // deliberately NOT inserted into `class_effect_wired`
        let verdict =
            classify(&class_unit("core_rulebook", "Fighter"), &facts, &BTreeSet::new(), false, true);
        assert_ne!(verdict.status, "grounded", "membership alone must not ground a class");
        assert_eq!(
            verdict.evidence,
            "class_modelled_but_no_observed_delta_on_the_rendered_snapshot"
        );
    }

    /// An observation never manufactures ingestion for a class no enum names.
    #[test]
    fn an_observation_never_grounds_a_class_absent_from_every_class_enum() {
        let mut facts = EngineFacts::default();
        facts.class_effect_wired.insert("adept".to_string());
        let verdict = classify(&class_unit("core_rulebook", "Adept"), &facts, &BTreeSet::new(), false, true);
        assert_eq!(verdict.status, "not-ingested");
        assert_eq!(verdict.evidence, "class_absent_from_ClassId_ALL_and_book_class_id_enums");
    }
}

#[cfg(test)]
mod class_feature_consumer_delta_tests {
    use super::*;

    /// Every pool in [`CLASS_FEATURE_POOLS`] must name a `choice_set_id` the
    /// engine actually recognises. A pool naming a slot no engine code reads
    /// would make the probe report `no_consumer_delta` for a reason that is
    /// the probe's own fault rather than the engine's -- exactly the kind of
    /// confident-wrong number this binary exists to avoid.
    #[test]
    fn every_pool_names_a_choice_set_the_engine_source_declares() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut declared = BTreeSet::new();
        let mut stack = vec![repo_root.join("src/rules_core")];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("rules_core is readable").flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    let text = std::fs::read_to_string(&path).expect("source file is readable");
                    let mut rest = text.as_str();
                    while let Some(at) = rest.find("\"choice:") {
                        rest = &rest[at + 1..];
                        if let Some(end) = rest.find('"') {
                            declared.insert(rest[..end].to_string());
                        }
                    }
                }
            }
        }
        assert!(
            !declared.is_empty(),
            "found no `choice:` set ids in src/rules_core -- the scan itself is broken"
        );
        for (group, owner, choice_set_id, _) in CLASS_FEATURE_POOLS {
            assert!(
                declared.contains(*choice_set_id),
                "pool `{group}` names `{choice_set_id}`, which no file under src/rules_core declares"
            );
            assert!(!owner.is_empty(), "pool `{group}` names no owner class");
        }
    }

    /// The companion guarantee to the one above, and the reason it exists: a
    /// namespaced consumer matches `domain:good`, never bare `good`. A pool
    /// declaring a namespace the engine never writes would have its probe
    /// selections silently ignored, and the probe would then report
    /// `no_consumer_delta` as though the ENGINE held nothing -- an
    /// under-report indistinguishable from a real ceiling. Every non-empty
    /// namespace must appear in engine source as a written selection id.
    #[test]
    fn every_namespaced_pool_uses_a_namespace_the_engine_source_writes() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut source = String::new();
        let mut stack = vec![repo_root.join("src/rules_core")];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("rules_core is readable").flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    source.push_str(&std::fs::read_to_string(&path).expect("readable"));
                }
            }
        }
        for (group, _, _, namespace) in CLASS_FEATURE_POOLS {
            if namespace.is_empty() {
                continue;
            }
            assert!(
                source.contains(&format!("\"{namespace}")),
                "pool `{group}` declares selection namespace `{namespace}`, which no file under \
                 src/rules_core ever writes -- the probe's selections would be silently ignored"
            );
        }
    }

    /// The probe must empty the slot it is testing before measuring the
    /// baseline, because `canonical_seeds_for` pre-fills several of these
    /// pools. This pins the seeds that overlap the pool table, so a future
    /// seed added to a probed slot cannot silently reintroduce the defect.
    #[test]
    fn canonical_seeds_really_do_occupy_probed_slots() {
        let probed: BTreeSet<&str> =
            CLASS_FEATURE_POOLS.iter().map(|(_, _, choice_set_id, _)| *choice_set_id).collect();
        let mut overlapping = BTreeSet::new();
        for (class_name, _) in modelled_class_books() {
            for choice in canonical_seeds_for(&class_name).0 {
                if probed.contains(choice.choice_set_id.as_str()) {
                    overlapping.insert(choice.choice_set_id);
                }
            }
        }
        assert!(
            !overlapping.is_empty(),
            "no canonical seed occupies a probed slot -- if this ever becomes true the retain() \
             in probe_class_feature_key is dead code and should be reconsidered, not deleted \
             silently"
        );
    }

    /// The discriminator that makes this a consumer-delta probe rather than a
    /// "the selection was accepted" probe. A slot that merely COUNTS picks
    /// produces the identical delta for any selection id, and must be refused.
    #[test]
    fn a_slot_that_only_counts_picks_is_refused_not_promoted() {
        let baseline: Vec<(String, i16)> = Vec::new();
        let observed = vec![("bab".to_string(), 3i16)];
        let control = vec![("bab".to_string(), 3i16)];
        assert_eq!(
            classify_class_feature_delta(&baseline, &observed, &control),
            ClassFeatureProbeOutcome::DeltaNotAttributableToTheRecord {
                shared: vec!["bab".to_string()]
            }
        );
    }

    /// The promoting case: the record's own selection moves a fact, and a
    /// different selection in the same slot does not move it the same way.
    #[test]
    fn a_per_record_delta_is_promoted() {
        let baseline: Vec<(String, i16)> = Vec::new();
        let observed = vec![("rage.superstition".to_string(), 2i16)];
        let control: Vec<(String, i16)> = Vec::new();
        assert!(matches!(
            classify_class_feature_delta(&baseline, &observed, &control),
            ClassFeatureProbeOutcome::Wired { .. }
        ));
    }

    /// Selecting the record changed nothing a consumer renders.
    #[test]
    fn no_movement_at_all_is_no_consumer_delta() {
        let baseline = vec![("bab".to_string(), 1i16)];
        let observed = vec![("bab".to_string(), 1i16)];
        let control = vec![("bab".to_string(), 1i16)];
        assert!(matches!(
            classify_class_feature_delta(&baseline, &observed, &control),
            ClassFeatureProbeOutcome::NoConsumerDelta
        ));
    }

    fn rage_power_siblings() -> BTreeMap<String, BTreeSet<String>> {
        BTreeMap::from([(
            "Rage Power".to_string(),
            BTreeSet::from(["Superstition".to_string(), "Animal Fury".to_string()]),
        )])
    }

    /// A group whose prefix names no engine slot is refused before any
    /// computation happens -- no player selection can reach the record.
    #[test]
    fn an_unpooled_group_is_refused() {
        let fixture = load_probe_fixture(&PathBuf::from(env!("CARGO_MANIFEST_DIR")));
        let mut class_books: BTreeMap<String, &'static str> = BTreeMap::new();
        class_books.insert("barbarian".to_string(), "core_rulebook");
        assert_eq!(
            probe_class_feature_key(
                &fixture,
                &class_books,
                &rage_power_siblings(),
                "Refined Education ~ Appraise"
            ),
            ClassFeatureProbeOutcome::NoChoiceSlotOffersIt
        );
    }

    /// A pooled group whose owner class this engine does not model is refused
    /// as such, and never confused with "the engine applies nothing".
    #[test]
    fn a_pool_of_an_unmodelled_class_is_refused_as_such() {
        let fixture = load_probe_fixture(&PathBuf::from(env!("CARGO_MANIFEST_DIR")));
        let class_books: BTreeMap<String, &'static str> = BTreeMap::new();
        assert_eq!(
            probe_class_feature_key(
                &fixture,
                &class_books,
                &rage_power_siblings(),
                "Rage Power ~ Superstition"
            ),
            ClassFeatureProbeOutcome::OwnerClassNotModelled
        );
    }

    /// A pool with exactly one corpus member has no control, and is refused
    /// rather than promoted on an uncontrolled delta.
    #[test]
    fn a_pool_with_no_sibling_is_refused() {
        let fixture = load_probe_fixture(&PathBuf::from(env!("CARGO_MANIFEST_DIR")));
        let mut class_books: BTreeMap<String, &'static str> = BTreeMap::new();
        class_books.insert("barbarian".to_string(), "core_rulebook");
        let lonely = BTreeMap::from([(
            "Rage Power".to_string(),
            BTreeSet::from(["Superstition".to_string()]),
        )]);
        assert_eq!(
            probe_class_feature_key(&fixture, &class_books, &lonely, "Rage Power ~ Superstition"),
            ClassFeatureProbeOutcome::NoSiblingToControlAgainst
        );
    }
}

/// P0.1 (SD-30 pre-launch remediation, hazard 1 of
/// `state-goals-and-lessons.md` §1.3): the regenerator must never silently
/// overwrite a `literal-verified`/`fixture-verified` stamp with nothing. These
/// tests exercise the guard's pure decision function directly -- no corpus
/// checkout, no subprocess -- so the RED/GREEN cycle stays fast and CI-safe.
#[cfg(test)]
mod stamp_loss_guard_tests {
    use super::*;

    fn inventory_json(units: &[(&str, &str)]) -> String {
        let rows: Vec<String> = units
            .iter()
            .map(|(id, status)| format!("{{\"id\": \"{id}\", \"status\": \"{status}\"}}"))
            .collect();
        format!("{{\"units\": [{}]}}", rows.join(", "))
    }

    /// `stamped_ids` picks out exactly the two done-rung statuses, ignoring
    /// every ordinary status a unit might otherwise carry.
    #[test]
    fn stamped_ids_finds_only_the_two_done_rung_statuses() {
        let doc = inventory_json(&[
            ("a", "literal-verified"),
            ("b", "fixture-verified"),
            ("c", "grounded"),
            ("d", "not-ingested"),
        ]);
        let ids = stamped_ids(&doc);
        assert_eq!(ids, BTreeSet::from(["a".to_string(), "b".to_string()]));
    }

    /// The defect this guard exists to catch: a plain regen (no sweep/fixture
    /// reports, so `incoming_stamped` is empty) against an existing inventory
    /// that carries real stamps must report EVERY one of them as lost. Before
    /// the guard existed, nothing in this binary ever computed this set --
    /// the loss happened silently at `std::fs::write`.
    #[test]
    fn a_plain_regen_against_a_stamped_inventory_loses_every_stamp() {
        let existing = inventory_json(&[
            ("advanced_class_guide:class_feature:rage_power_abyssal_blood", "literal-verified"),
            ("core_rulebook:feat:power_attack", "fixture-verified"),
            ("core_rulebook:feat:cleave", "grounded"),
        ]);
        let incoming_stamped: BTreeSet<String> = BTreeSet::new();
        let lost = stamp_loss(&existing, &incoming_stamped);
        assert_eq!(
            lost,
            BTreeSet::from([
                "advanced_class_guide:class_feature:rage_power_abyssal_blood".to_string(),
                "core_rulebook:feat:power_attack".to_string(),
            ]),
            "a plain regen must be seen to drop both stamped units -- this is exactly the \
             silent loss hazard #1 describes"
        );
    }

    /// The honest case: a run that carries the sweep/fixture reports and
    /// reproduces every currently-stamped id loses nothing, and the guard's
    /// decision must reflect that -- it must not fire on ordinary, correct
    /// regeneration.
    #[test]
    fn a_regen_that_reproduces_every_stamp_loses_nothing() {
        let existing = inventory_json(&[
            ("a", "literal-verified"),
            ("b", "fixture-verified"),
        ]);
        let incoming_stamped: BTreeSet<String> =
            BTreeSet::from(["a".to_string(), "b".to_string(), "c".to_string()]);
        assert!(
            stamp_loss(&existing, &incoming_stamped).is_empty(),
            "a run that reproduces every existing stamp (plus a brand new one) must not \
             be treated as a loss"
        );
    }

    /// A partial loss -- one stamp reproduced, one not -- must be reported
    /// precisely, not rounded up to "all" or down to "none".
    #[test]
    fn a_partial_stamp_loss_is_reported_precisely() {
        let existing = inventory_json(&[
            ("a", "literal-verified"),
            ("b", "fixture-verified"),
        ]);
        let incoming_stamped: BTreeSet<String> = BTreeSet::from(["a".to_string()]);
        assert_eq!(stamp_loss(&existing, &incoming_stamped), BTreeSet::from(["b".to_string()]));
    }

    /// An unreadable/malformed existing document has nothing provable to
    /// lose -- the guard must treat that as "no loss", never as an error that
    /// blocks every future write.
    #[test]
    fn a_malformed_existing_document_reports_no_loss() {
        let incoming_stamped: BTreeSet<String> = BTreeSet::new();
        assert!(stamp_loss("not json at all", &incoming_stamped).is_empty());
        assert!(stamp_loss("", &incoming_stamped).is_empty());
    }
}

/// `OPEN-ISSUES.md` row 68 -- the book-attribution defect (SD31-ATTRIB-001).
/// `race_resolver.rs`'s module doc, citing `decisions.md §25.2`, states the
/// contract: a record's `book` is the corpus directory it was loaded from,
/// and `core_essentials` -- PCGen's own shared packaging directory, not a
/// book -- must never appear as attribution. Before this cycle's fix it did,
/// for ~1,610 units across 8 kinds: Core Rulebook reported ZERO races,
/// Advanced Race Guide reported ONE.
///
/// This is the mechanical gate proving the contract, and the first test
/// below is the "prove it fails" seed the card requires: it exercises
/// `enumerate_book` against a synthetic `core_essentials`-shaped scratch
/// directory carrying a Dwarf race row -- the exact CRB-race shape row 68
/// names -- and asserts the resulting unit's `book` is `"core_rulebook"`,
/// never `"core_essentials"`. Reverting `enumerate_file`'s `effective_book`
/// substitution (this module's own fix) makes this test fail immediately:
/// the unit would carry `book: "core_essentials"` instead, reproducing row
/// 68's defect exactly. `cargo test --locked --bin v06_work_inventory
/// core_essentials_book_attribution_tests` is this gate's own invocation,
/// picked up by `./scripts/verify.sh`'s `root-full`/`root-lib` stages via
/// `cargo test --workspace` with no separate wiring needed.
#[cfg(test)]
mod core_essentials_book_attribution_tests {
    use super::*;

    /// A scratch `core_essentials/`-shaped tree, cleaned up on drop, so
    /// these tests never touch the real PCGen checkout `PCGEN_CORPUS_ROOT`
    /// would point at (same pattern as `wiring_class_wiring_tests::ScratchBook`).
    struct ScratchCoreEssentials {
        root: PathBuf,
    }

    impl ScratchCoreEssentials {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "codex_book_attrib_test_{name}_{}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(&root).unwrap();
            ScratchCoreEssentials { root }
        }

        fn write(&self, rel_path: &str, contents: &str) {
            let path = self.root.join(rel_path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, contents).unwrap();
        }
    }

    impl Drop for ScratchCoreEssentials {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    /// **The seeded-mislabel proof.** A per-race row under
    /// `core_essentials/races/dwarf/` -- the real, in-scope CRB race row 68
    /// names by example -- must attribute to `core_rulebook`, not
    /// `core_essentials`. Before the fix this unit's `book` was
    /// unconditionally `"core_essentials"` (`enumerate_file` stamped
    /// `book.to_string()` verbatim); this test fails against that code.
    #[test]
    fn a_dwarf_race_row_attributes_to_core_rulebook_not_core_essentials() {
        let book = ScratchCoreEssentials::new("dwarf_race");
        book.write(
            "races/dwarf/dwarf_races.lst",
            "Dwarf\t\tSORTKEY:a_base_pc\tSTARTFEATS:1\tSOURCEPAGE:p.xx\tFACT:IsPC|true\n",
        );
        let enumeration = enumerate_book(&book.root, "core_essentials");
        let unit = enumeration
            .units
            .iter()
            .find(|u| u.name == "Dwarf")
            .expect("the Dwarf race row was enumerated");
        assert_eq!(
            unit.book, "core_rulebook",
            "a per-race row under core_essentials/races/dwarf/ must attribute to its true \
             source book (decisions.md §25.2), never stay core_essentials"
        );
    }

    /// A root-level shared `ce_*.lst` file carrying a `SOURCELONG:` header
    /// must attribute to the book that header names, cross-checked against
    /// that book's own real files -- not inferred from the filename (the
    /// `ce_races_familiar_cr.lst` -> `SOURCELONG:Bestiary` case this repo
    /// actually has, where the `_cr` suffix would mislead a name-based guess).
    #[test]
    fn a_root_level_file_attributes_via_its_own_sourcelong_header_not_its_filename() {
        let book = ScratchCoreEssentials::new("sourcelong_spell");
        book.write(
            "ce_spells.lst",
            "SOURCELONG:Bestiary\tSOURCESHORT:B1\n\nMage Hand\tSCHOOL:Transmutation\tCLASSES:Wizard\n",
        );
        let enumeration = enumerate_book(&book.root, "core_essentials");
        let unit = enumeration
            .units
            .iter()
            .find(|u| u.name == "Mage Hand")
            .expect("the spell row was enumerated");
        assert_eq!(unit.book, "bestiary");
    }

    /// A race this repo cannot attribute to exactly one in-scope book
    /// (`monkey_goblin`: natively declared by both `bestiary_6` and
    /// `inner_sea_bestiary`'s own `.pcc`, per this fix's own doc comment)
    /// must stay `core_essentials` -- the honest "not yet attributable"
    /// state -- rather than guess. Proves the resolver discriminates instead
    /// of re-attributing everything under `races/`.
    #[test]
    fn an_ambiguous_race_stays_unattributed_rather_than_guessed() {
        let book = ScratchCoreEssentials::new("ambiguous_race");
        book.write(
            "races/monkey_goblin/monkey_goblin_races.lst",
            "Goblin (Monkey)\t\tSOURCEPAGE:p.xx\tFACT:IsPC|true\n",
        );
        let enumeration = enumerate_book(&book.root, "core_essentials");
        let unit = enumeration
            .units
            .iter()
            .find(|u| u.name == "Goblin (Monkey)")
            .expect("the row was enumerated");
        assert_eq!(unit.book, "core_essentials");
    }

    /// A root-level file with no `SOURCELONG` header at all (the
    /// `ce_abilities_race.lst` shape -- PCGen's own consolidated
    /// Size/Vision/Universal-Monster-Rule reference table, confirmed by its
    /// own in-file comment to be book-agnostic engine bookkeeping) must also
    /// stay `core_essentials`, not be assigned an arbitrary default.
    #[test]
    fn a_file_with_no_sourcelong_header_stays_unattributed() {
        let book = ScratchCoreEssentials::new("no_header");
        book.write(
            "ce_abilities_race.lst",
            "Darkvision\t\tKEY:Darkvision\tCATEGORY:Special Ability\tTYPE:SpecialQuality\n",
        );
        let enumeration = enumerate_book(&book.root, "core_essentials");
        let unit = enumeration
            .units
            .iter()
            .find(|u| u.key == "Darkvision")
            .expect("the row was enumerated");
        assert_eq!(unit.book, "core_essentials");
    }

    /// A `.MOD` row inside a per-race `core_essentials` file that is rescued
    /// (`mod_only_rescue`, because its base is declared nowhere else in the
    /// synthetic corpus) must carry the SAME resolved true book as an
    /// ordinary declared row would -- the rescue path used to stamp
    /// `book.id.clone()` (always `"core_essentials"` for this enumeration)
    /// unconditionally, bypassing the fix for `origin: ModOnly` units
    /// (8 of the 1,610 in the real corpus, `OPEN-ISSUES.md` row 68's own
    /// re-derivation).
    #[test]
    fn a_mod_only_rescued_row_from_a_per_race_file_also_attributes_correctly() {
        let book = ScratchCoreEssentials::new("mod_rescue");
        book.write(
            "races/drow/drow_abilities_race.lst",
            "Universal Monster Rule ~ Light Blindness.MOD\tKEY:Universal Monster Rule ~ Light \
             Blindness\tCATEGORY:Special Ability\tTYPE:SpecialQuality\n",
        );
        let mut enumerations: BTreeMap<String, BookEnumeration> = BTreeMap::new();
        enumerations.insert(
            "core_essentials".to_string(),
            enumerate_book(&book.root, "core_essentials"),
        );
        let declared: BTreeSet<(Kind, String)> = BTreeSet::new();
        let mut rescued: BTreeSet<(Kind, String)> = BTreeSet::new();
        let targets = std::mem::take(&mut enumerations.get_mut("core_essentials").unwrap().mod_targets);
        for (kind, key, name, provenance, magnitudes, resolved_book) in targets {
            if declared.contains(&(kind, name.to_lowercase())) {
                continue;
            }
            if !rescued.insert((kind, name.to_lowercase())) {
                continue;
            }
            enumerations.get_mut("core_essentials").unwrap().units.push(CorpusUnit {
                book: resolved_book.map(str::to_string).unwrap_or_else(|| "core_essentials".to_string()),
                source_book: "core_essentials".to_string(),
                kind,
                key,
                name,
                origin: Origin::ModOnly,
                provenance,
                magnitude_token_count: magnitudes,
                type_facet: None,
                visible: true,
            });
        }
        let unit = enumerations["core_essentials"]
            .units
            .iter()
            .find(|u| u.origin == Origin::ModOnly)
            .expect("the .MOD row was rescued");
        assert_eq!(
            unit.book, "bestiary",
            "a rescued .MOD row from a per-race file must resolve the same true book an \
             ordinary declared row from the same file would"
        );
    }

    /// Pins the exact ambiguous-race list this fix's own doc comment claims,
    /// so a future book onboarding that resolves one of these cannot
    /// silently narrow the set without this test forcing an update (and a
    /// matching doc-comment edit).
    #[test]
    fn core_essentials_ambiguous_races_stay_unattributed() {
        let ambiguous = [
            "android",
            "aquatic_elf",
            "gathlain",
            "ghoran",
            "lashunta",
            "monkey_goblin",
            "syrinx",
            "triaxian",
        ];
        for slug in ambiguous {
            assert!(
                RACE_TRUE_BOOK.iter().all(|(s, _)| *s != slug),
                "{slug} is documented as genuinely ambiguous and must not appear in \
                 RACE_TRUE_BOOK without a corresponding doc-comment update"
            );
        }
        assert_eq!(
            RACE_TRUE_BOOK.len(),
            43,
            "43 unambiguous + 8 ambiguous == the real corpus's 51 core_essentials races \
             (OPEN-ISSUES.md row 68; gathlain moved from unambiguous to ambiguous by \
             SD31-W5-INTEGRATE-001's adversarial-review fix -- ultimate_wilderness also \
             natively declares it); a table-length change means the roster moved and both \
             this assertion and the doc comment above need re-deriving, not just bumping"
        );
    }
}
