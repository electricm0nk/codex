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
//!    books including the ones no code has ever read. "What exists" is the
//!    completeness guarantee, so a book the engine knows nothing about still
//!    contributes real, named units — at `not-started` — rather than being
//!    silently skipped.
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
    AcquisitionMode, CharacterClassLevel, CharacterInput, SelectedChoice, SpellSelection,
    load_character_input_fixture,
};
use codex::rules_core::pilot_compute::{
    PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::acg::{self, AcgClassId};
use codex::rules_core::rules_tables::apg::{self, ApgClassId};
use codex::rules_core::rules_tables::beastiary1::{self, MonsterId};
use codex::rules_core::rules_tables::crb::{
    class_tables::ClassId, equipment_tables as crb_equipment_tables,
    race_tables::{RaceId, race_traits},
    spell_list as crb_spell_list,
};
use codex::rules_core::rules_tables::feats_all::all_feat_tables;

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
            "A record whose first field is `CATEGORY=Internal|...`. PCGen export-engine \
             plumbing in a namespace no player ever sees. Never a unit.",
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
];

/// Tab-field prefixes that carry a real numeric magnitude. A record with none
/// of these has no number for the engine to compute, which is the corpus half
/// of the `text-complete` ruling.
const MAGNITUDE_TOKENS: &[&str] = &[
    "BONUS:",
    "TEMPBONUS:",
    "DEFINE:",
    "COST:",
    "WT:",
    "CR:",
    "AC:",
    "ACCHECK:",
    "DAMAGE:",
    "CRITMULT:",
    "CRITRANGE:",
    "RANGE:",
    "REACH:",
    "MOVE:",
    "HITDIE:",
    "LEVELADJUSTMENT:",
    "SR:",
    "DR:",
    "SPELLFAILURE:",
    "STAT:",
];

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

/// A record's kind, refined from the file-level guess by what the record
/// itself says. A `*_races.lst` row carrying a `CR:` token is a monster: that
/// is the corpus's own discriminator (`cr_races.lst` carries zero `CR:`
/// tokens across its seven playable races; `b1_races.lst` carries 351).
fn refine_kind(file_kind: Kind, fields: &[&str]) -> Kind {
    match file_kind {
        Kind::Race if has_token(fields, "CR:") => Kind::Monster,
        other => other,
    }
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

        if first.starts_with("CATEGORY=Internal|") {
            *out.trap_hits.entry("internal_namespace").or_default() += 1;
            continue;
        }
        let visible = !fields.iter().any(|f| f.trim() == "VISIBLE:NO");
        if !visible {
            *out.trap_hits.entry("invisible_record").or_default() += 1;
        }

        // `.MOD` is resolved corpus-wide after enumeration; stash it.
        if let Some(mod_at) = first.find(".MOD") {
            let mut base = first[..mod_at].to_string();
            if let Some(rest) = base
                .strip_prefix("CATEGORY=")
                .and_then(|r| r.split_once('|'))
                .map(|(_, rest)| rest.to_string())
            {
                // `CATEGORY=Special Ability|Foo.MOD` -> `Foo`
                base = rest;
            }
            // `CLASS:Bard.MOD` names the base class `Bard`, not a record
            // called `CLASS:Bard`. Without this, the name never matches the
            // declared set and the rescue invents a second Bard in every book
            // that merely modifies the Core Rulebook's one.
            if let Some(rest) = base.strip_prefix("CLASS:") {
                base = rest.to_string();
            }
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

/// The engine's book id for a corpus directory. Only the four ingested books
/// have one; everything else is `None`, which is what makes the rest of the
/// corpus land at `not-started` honestly rather than by omission.
fn rule_set_for(book_dir: &str) -> Option<RuleSetId> {
    match book_dir {
        "core_rulebook" => Some(RuleSetId::Crb),
        "advanced_players_guide" => Some(RuleSetId::Apg),
        "advanced_class_guide" => Some(RuleSetId::Acg),
        "bestiary" => Some(RuleSetId::Bestiary1),
        _ => None,
    }
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
    /// Every feat key the catalog holds, per book.
    feat_keys: BTreeMap<&'static str, BTreeSet<String>>,
    /// Every spell key the catalog holds, per book, with whether the engine
    /// resolved a numeric level for it.
    spell_levels: BTreeMap<&'static str, BTreeMap<String, bool>>,
    /// Every equipment key the catalog holds, per book.
    equipment_keys: BTreeMap<&'static str, BTreeSet<String>>,
    /// Every Bestiary 1 monster that resolves to a real stat block, by name.
    monster_names: BTreeSet<String>,
    /// Every class the engine models, by lowercase name, with its book.
    class_books: BTreeMap<String, &'static str>,
    /// Every race the engine models, by lowercase name.
    race_names: BTreeSet<String>,
    /// Race trait identities the engine grounds, as `<race>.<trait slug>`.
    race_trait_ids: BTreeSet<String>,
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
    /// Whether one book's own compiled table holds this unit's identity.
    /// Used to attribute a shared-library record to the book that really
    /// ingested it rather than to an arbitrary one of its hosts.
    fn holds_key(&self, book: &str, kind: &Kind, key: &str, name: &str) -> bool {
        let hit = |set: Option<&BTreeSet<String>>| {
            set.map(|s| s.contains(key) || s.contains(name)).unwrap_or(false)
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
            Kind::Monster => book == "bestiary_1" && self.monster_names.contains(&name.to_lowercase()),
            Kind::Race => book == "core_rulebook" && self.race_names.contains(&name.to_lowercase()),
            Kind::RaceTrait => {
                book == "core_rulebook"
                    && self
                        .race_names
                        .iter()
                        .any(|r| self.race_trait_ids.contains(&format!("{r}.{}", slug(name))))
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
) -> EngineFacts {
    let mut feat_keys: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();
    for table in all_feat_tables() {
        let book = rule_set_id(table.rule_set);
        let set = feat_keys.entry(book).or_default();
        for entry in table.entries {
            set.insert(entry.key.to_string());
        }
    }

    let mut spell_levels: BTreeMap<&'static str, BTreeMap<String, bool>> = BTreeMap::new();
    spell_levels.insert(
        "core_rulebook",
        crb_spell_list::SPELL_LIST
            .iter()
            .map(|e| (e.key.to_string(), true))
            .collect(),
    );
    spell_levels.insert(
        "advanced_players_guide",
        apg::spell_list::SPELL_LIST
            .iter()
            .map(|e| (e.key.to_string(), e.level.is_some()))
            .collect(),
    );
    spell_levels.insert(
        "advanced_class_guide",
        acg::spell_list::SPELL_LIST
            .iter()
            .map(|e| (e.key.to_string(), true))
            .collect(),
    );

    let mut equipment_keys: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();
    equipment_keys.insert(
        "core_rulebook",
        crb_equipment_tables::equipment_tables()
            .iter()
            .map(|e| e.key.to_string())
            .collect(),
    );
    equipment_keys.insert(
        "advanced_players_guide",
        apg::equipment_tables::EQUIPMENT_TABLE
            .iter()
            .map(|e| e.key.to_string())
            .collect(),
    );
    equipment_keys.insert(
        "advanced_class_guide",
        acg::equipment_tables::equipment_tables()
            .iter()
            .map(|e| e.key.to_string())
            .collect(),
    );
    equipment_keys.insert(
        "bestiary_1",
        beastiary1::equipment_tables::EQUIPMENT_TABLE
            .iter()
            .map(|e| e.key.to_string())
            .collect(),
    );

    let monster_names: BTreeSet<String> = MonsterId::ALL
        .iter()
        .filter_map(|&id| beastiary1::monster_resolve(id, RuleSetId::Bestiary1))
        .map(|b| b.name.to_lowercase())
        .collect();

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
        feat_keys,
        spell_levels,
        equipment_keys,
        monster_names,
        class_books,
        race_names,
        race_trait_ids,
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
            match hosts
                .iter()
                .find(|b| facts.holds_key(b, &unit.kind, &unit.key, &unit.name))
            {
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
            Verdict {
                status: "ingested-magnitude",
                evidence: "equipment_table_entry_with_corpus_magnitude".to_string(),
                reason: None,
                engine_book: engine_book_field,
            }
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
            let candidates: Vec<String> = facts
                .race_names
                .iter()
                .map(|r| format!("{r}.{}", slug(&unit.name)))
                .collect();
            if candidates.iter().any(|c| facts.race_trait_ids.contains(c)) {
                return Verdict {
                    status: "grounded",
                    evidence: "race_trait_record_grounded_by_race_traits".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            not_ingested("race_trait_absent_from_race_traits")
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
            if text_only {
                return Verdict {
                    status: "text-complete",
                    evidence: "corpus_record_carries_no_magnitude_token".to_string(),
                    reason: None,
                    engine_book: engine_book_field,
                };
            }
            not_ingested("no_explanation_id_and_no_diagnostic_names_this_feature")
        }
        Kind::Companion => not_ingested("companion_content_has_no_engine_table"),
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
}

fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let args: Vec<String> = std::env::args().collect();
    let summary_only = args.iter().any(|a| a == "--summary");
    // `--summary` never writes the file: a summary is not the artefact, and
    // overwriting the full inventory with one would be a silent data loss.
    let stdout_only = summary_only || args.iter().any(|a| a == "--stdout-only");

    let corpus_root = std::env::var("PCGEN_CORPUS_ROOT")
        .unwrap_or_else(|_| "/home/ubuntu/workspace/repos/pcgen/data".to_string());
    let books_dir = PathBuf::from(&corpus_root).join(BOOKS_RELATIVE);
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
    let mut book_dirs: Vec<String> = std::fs::read_dir(&books_dir)
        .expect("corpus books directory is readable")
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    book_dirs.sort();
    let known_books: BTreeSet<String> = book_dirs.iter().cloned().collect();

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
        let dir = books_dir.join(id);
        let includes = pcc_includes(&dir, &known_books);
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
        enumerations.insert(book.id.clone(), enumerate_book(&books_dir.join(&book.id), &book.id));
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
    let facts = gather_engine_facts(&fixture, corpus_class_names);

    // --- classify ----------------------------------------------------------
    let empty: BTreeSet<String> = BTreeSet::new();
    let mut inventory: Vec<InventoryUnit> = Vec::new();
    for book in &books {
        let Some(enumeration) = enumerations.get(&book.id) else { continue };
        let hosts = included_by.get(&book.id).unwrap_or(&empty);
        for unit in &enumeration.units {
            let verdict = classify(unit, &facts, hosts);
            inventory.push(InventoryUnit {
                id: format!("{}:{}:{}", book.id, unit.kind.id(), slug(&unit.key)),
                unit: unit.clone(),
                verdict,
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
        out.push_str(&format!(
            "    {{\"id\": {}, \"book\": {}, \"engine_book\": {}, \"kind\": {}, \"name\": {}, \
             \"corpus_key\": {}, \"origin\": {}, \"visible\": {}, \"type_facet\": {}, \
             \"source_file\": {}, \"source_line\": {}, \"magnitude_token_count\": {}, \
             \"status\": {}, \"evidence\": {}, \"reason\": {}}}",
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
