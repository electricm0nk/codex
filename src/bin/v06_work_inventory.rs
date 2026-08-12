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
use codex::rules_core::corpus_loader::{BookCorpusRoot, load_equipment_corpus};
use codex::rules_core::race_resolver::{TraitRole, load_race_corpus};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::equipment_resolver;
use codex::rules_core::pilot_compute::{
    PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::acg::{self, AcgClassId};
use codex::rules_core::rules_tables::apg::{self, ApgClassId};
use codex::rules_core::rules_tables::beastiary1::{self, MonsterId};
use codex::rules_core::rules_tables::companion_chassis;
use codex::rules_core::rules_tables::monster_chassis;
use codex::rules_core::rules_tables::crb::{
    class_tables::ClassId, equipment_tables as crb_equipment_tables,
    race_tables::{RaceId, race_traits},
    spell_list as crb_spell_list,
};
use codex::rules_core::rules_tables::feats_all::all_feat_tables;
use codex::rules_core::spell_resolver;
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
    book: String,
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

/// Per-book enumeration bookkeeping the JSON reports verbatim.
#[derive(Debug, Default)]
struct BookEnumeration {
    units: Vec<CorpusUnit>,
    trap_hits: BTreeMap<&'static str, usize>,
    files_enumerated: usize,
    files_not_enumerated: BTreeSet<String>,
    /// `.MOD` target names seen in this book, kept so `mod_only_rescue` can
    /// run after the whole corpus is known.
    mod_targets: Vec<(Kind, String, String, Provenance, usize)>,
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
///   proved necessary for `race_trait`'s own declared counts.
fn refine_kind(file_kind: Kind, fields: &[&str]) -> Kind {
    match file_kind {
        Kind::Race if has_token(fields, "CR:") => Kind::Monster,
        Kind::RaceTrait => {
            let type_first_segment = token_value(fields, "TYPE:")
                .and_then(|t| t.split('.').next())
                .unwrap_or("");
            if MONSTER_ABILITY_TYPE_FACETS.contains(&type_first_segment) {
                Kind::MonsterAbility
            } else {
                Kind::RaceTrait
            }
        }
        other => other,
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

/// Enumerate one `.lst` file into `out`, recording every trap hit.
fn enumerate_file(path: &Path, book: &str, kind: Kind, text: &str, out: &mut BookEnumeration) {
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
            book: book.to_string(),
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
    equipment_effect_wired: BTreeSet<String>,
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
            Kind::Monster => {
                if book == "bestiary_1" {
                    return self.monster_names.contains(&name.to_lowercase());
                }
                hit_lowercase(self.chassis_monster_keys.get(book))
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
            if record.role == TraitRole::Unclassified {
                continue;
            }
            probe.reachable.insert(coordinate, record.book_id.clone());
        }
    }
    probe
}

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
fn probe_equipment_effect_wiring(repo_root: &Path) -> BTreeSet<String> {
    let mut wired = BTreeSet::new();
    let book_dirs: Vec<PathBuf> = book_corpus_roots(repo_root);
    let roots: Vec<BookCorpusRoot> = OBSERVABLE_BOOK_DIRS
        .iter()
        .zip(book_dirs.iter())
        .map(|(id, dir)| BookCorpusRoot { book_id: id, dir })
        .collect();
    let corpus = load_equipment_corpus(&roots);
    if corpus.is_empty() {
        return wired;
    }

    let mut keys: BTreeSet<&'static str> = BTreeSet::new();
    keys.extend(crb_equipment_tables::equipment_tables().iter().map(|e| e.key));
    keys.extend(apg::equipment_tables::EQUIPMENT_TABLE.iter().map(|e| e.key));
    keys.extend(acg::equipment_tables::equipment_tables().iter().map(|e| e.key));
    keys.extend(beastiary1::equipment_tables::EQUIPMENT_TABLE.iter().map(|e| e.key));

    for &key in &keys {
        if equipment_key_is_wired(key, &corpus) {
            wired.insert(key.to_string());
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
    for book in monster_chassis::MONSTER_BOOKS {
        chassis_monster_keys.insert(
            book.corpus_book,
            book.monsters.iter().map(|m| m.key.to_lowercase()).collect(),
        );
        chassis_monster_ability_keys.insert(
            book.corpus_book,
            book.monster_abilities.iter().map(|a| a.key.to_lowercase()).collect(),
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

    let race_names: BTreeSet<String> =
        RaceId::ALL.iter().map(|&r| race_name(r).to_string()).collect();
    let race_trait_ids: BTreeSet<String> = race_traits()
        .iter()
        .map(|t| format!("{}.{}", race_name(t.race_id), slug(t.trait_name)))
        .collect();
    let race_trait_probe = probe_race_trait_corpus(repo_root);

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
        feat_keys,
        spell_levels,
        equipment_keys,
        monster_names,
        chassis_monster_keys,
        chassis_monster_ability_keys,
        chassis_companion_keys,
        class_books,
        race_names,
        race_trait_ids,
        race_trait_probe,
        explanation_ids,
        diagnostics,
        corpus_class_names,
    }
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
        "ingested-magnitude",
        "The engine holds the record WITH its real numeric fields, but this generator observes no \
         consumer delta for this kind (spells, equipment). Strictly weaker than `grounded` and \
         deliberately a separate word: calling it grounded would be the same over-claim this \
         inventory exists to prevent.",
    ),
    (
        "text-complete",
        "The engine holds the record, and the corpus record carries NO magnitude token at all, so \
         there is no number to compute. Per the operator's standing ruling the description \
         reaching the player is the whole of the work.",
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
fn classify(unit: &CorpusUnit, facts: &EngineFacts, book_included_by: &BTreeSet<String>) -> Verdict {
    // A book with no compiled rule set has had nothing attempted -- unless it
    // is the shared library other books pull in, in which case the record's
    // real home is whichever ingested book includes it. The host is chosen by
    // asking each candidate's own tables whether they hold this key, so the
    // attribution is OBSERVED rather than picked arbitrarily; when no candidate
    // holds it the record is left unattributed rather than assigned to a host
    // at random.
    let own_engine_book = engine_book_for(&unit.book);
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

    let text_only = unit.magnitude_token_count == 0;
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
            if text_only {
                return Verdict {
                    status: "text-complete",
                    evidence: "in_catalog_and_corpus_record_carries_no_magnitude_token".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            Verdict {
                status: "unknown",
                evidence: "in_catalog_with_corpus_magnitude_but_no_observed_consumer".to_string(),
                reason: Some(format!(
                    "corpus record carries {} magnitude token(s) and the feat IS in the engine's \
                     catalog, but the feat-effect probe observed no computed delta across the \
                     swept postures. That is the probe's documented lower-bound behaviour: the \
                     effect may need a posture, an opponent or a combat action this engine does \
                     not model. Reported as unknown rather than deferred because no engine \
                     diagnostic is scoped to a feat, so there is no engine text to quote",
                    unit.magnitude_token_count
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
                // SD28-E14-F1: NOT promoted. See the doc comment above
                // `probe_equipment_effect_wiring` (the removed
                // `probe_spell_effect_wiring`'s replacement note) -- no
                // currently-wired consumer reads a spell's magnitude, so
                // every resolved-level spell stays `ingested-magnitude`
                // exactly as before this epic touched this arm.
                Some(true) => Verdict {
                    status: "ingested-magnitude",
                    evidence: "spell_list_entry_with_resolved_level".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                },
                Some(false) => Verdict {
                    status: "text-complete",
                    evidence: "spell_list_entry_with_description_but_no_corpus_level".to_string(),
                    reason: None,
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
            if text_only {
                return Verdict {
                    status: "text-complete",
                    evidence: "in_equipment_tables_and_corpus_record_carries_no_magnitude_token"
                        .to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            if facts.equipment_effect_wired.contains(&unit.key)
                || facts.equipment_effect_wired.contains(&unit.name)
            {
                return Verdict {
                    status: "grounded",
                    evidence: "equipment_effect_probe_observed_computed_delta".to_string(),
                    reason: None,
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
                return Verdict {
                    status: "grounded",
                    evidence: "race_trait_applied_by_the_race_corpus_the_app_loads".to_string(),
                    reason: None,
                    engine_book: if own_engine_book == Some(observed) {
                        engine_book_field
                    } else {
                        Some(observed.to_string())
                    },
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
            if facts.class_books.contains_key(&name) {
                return Verdict {
                    status: "grounded",
                    evidence: "class_modelled_and_swept_through_the_real_compute_pipeline"
                        .to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            not_ingested("class_absent_from_ClassId_ALL_and_book_class_id_enums")
        }
        Kind::ClassFeature => {
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

fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let args: Vec<String> = std::env::args().collect();
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

    let fixture_path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let fixture_text = match std::fs::read_to_string(&fixture_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("could not read {}: {e}", fixture_path.display());
            std::process::exit(1);
        }
    };
    let Some(fixture) = load_character_input_fixture(&fixture_text).character_input else {
        eprintln!("fixture {} did not load", fixture_path.display());
        std::process::exit(1);
    };

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
        let scope = if rule_set.is_some() {
            "in_scope"
        } else if id == "core_essentials" {
            "shared_library"
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
        for (kind, key, name, provenance, magnitudes) in targets {
            if declared.contains(&(kind, name.to_lowercase())) {
                continue;
            }
            if !rescued.insert((kind, name.to_lowercase())) {
                *enumeration.trap_hits.entry("duplicate_identity").or_default() += 1;
                continue;
            }
            *enumeration.trap_hits.entry("mod_only_rescue").or_default() += 1;
            enumeration.units.push(CorpusUnit {
                book: book.id.clone(),
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
    let facts = gather_engine_facts(&fixture, corpus_class_names, &repo_root);

    // --- wiring_class (GE-01) -----------------------------------------------
    // Built once, corpus-wide: the token closure index and a raw-line cache
    // shared by every unit's determination.
    let mod_index = build_mod_index(&book_paths);
    let mut corpus_lines = CorpusLines::new(&book_paths);

    // --- classify ----------------------------------------------------------
    let empty: BTreeSet<String> = BTreeSet::new();
    let mut inventory: Vec<InventoryUnit> = Vec::new();
    for book in &books {
        let Some(enumeration) = enumerations.get(&book.id) else { continue };
        let hosts = included_by.get(&book.id).unwrap_or(&empty);
        for unit in &enumeration.units {
            let verdict = classify(unit, &facts, hosts);
            let rows = token_closure_rows(
                &mut corpus_lines,
                &mod_index,
                &unit.book,
                &unit.provenance.file,
                unit.provenance.line,
                &unit.name,
                &unit.key,
            );
            let row_refs: Vec<Option<&str>> = rows.iter().map(|r| r.as_deref()).collect();
            let (wc_class, wc_reason, wc_signals) = wiring_class::determine_closure(&row_refs);
            inventory.push(InventoryUnit {
                id: format!("{}:{}:{}", book.id, unit.kind.id(), slug(&unit.key)),
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
        if let Some(engine_book) = item
            .verdict
            .engine_book
            .clone()
            .or_else(|| engine_book_for(&item.unit.book).map(|b| b.to_string()))
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
         and engine differ only in `generated_at`.\",\n",
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
    if let Some(parent) = output_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(&output_path, &out) {
        eprintln!("could not write {}: {e}", output_path.display());
        std::process::exit(1);
    }
    print!("{out}");
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
        // `ultimate_psionics` moved from uncompiled to compiled in SD28-E29
        // (`epic-29-upsi-complete`) -- `rule_set_for` now correctly returns
        // `Some(RuleSetId::Upsi)` for it, so it is no longer a valid
        // uncompiled example. `inner_sea_gods` remains genuinely
        // uncompiled (SD-30's own book set, out of this bundle).
        assert_eq!(rule_set_for("inner_sea_gods"), None);
    }
}

/// SD28-E14: observation-harness widening tests. F2 (equipment probe) with
/// a positive proof against the real on-disk corpus and negative proofs
/// that the probe does NOT promote a unit the engine genuinely does not
/// wire (F3's anti-gaming binding). F1 (spell probe) is deliberately absent
/// -- see the doc comment at the bottom of this module for why.
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
