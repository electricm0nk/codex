//! The reach gate: ingested content that no surface carries to the player
//! is a failure, not a silent omission.
//!
//! ## The defect this exists to make impossible
//!
//! Six separate times, content was ingested, computed, corpus-cited — and
//! never reached the player. Feats rendered as raw selection ids. 441 APG/ACG
//! spells reached no surface at all. Equipment weight and cost were computed
//! and then dropped at the IPC boundary. AC-by-source, the Pets tab and the
//! Weapons tab were placeholders sitting on complete data. 636 explanation
//! records never crossed IPC.
//!
//! Every one was found by accident and patched individually, and the next one
//! appeared anyway — because nothing in the build could tell the difference
//! between "this book's records reach a player" and "this book's records
//! exist". With 20 more books to ingest, that difference has to be checkable
//! by a machine.
//!
//! ## What this module generalizes
//!
//! `codex::rules_core::description_completion` already implements this idea
//! for exactly one thing: it refuses to call a zero-magnitude feat complete
//! unless (1) the text genuinely exists and (2) it is recorded on the field
//! the shipped app actually renders. Its `DescriptionSurface` enum is the key
//! move — a variant there is a *claim that the app displays that text today*,
//! traced end to end in its doc comment.
//!
//! This module is that idea at the level of a whole ingested record family
//! rather than one feat, and with one difference that matters: a claim here is
//! not merely documented, it is **executed**. Each claim runs the real IPC
//! builder the Tauri command returns and checks the records are in the
//! response. A doc comment cannot rot into a lie here, because the comment is
//! not what the test reads.
//!
//! ## How it stays honest as books are added
//!
//! The inventory of ingested content is never hand-listed here. It is the
//! union of three independent live sources, so a new book has to defeat all
//! three to slip through:
//!
//! 1. **The app's own ingest diagnostic.** `build_corpus_ingest_diagnostic()`
//!    counts every book's real tables and is already shipped to the player.
//!    Every `(book, kind)` pair it reports with a non-zero count must have a
//!    reach claim here.
//! 2. **The record slices and table accessors in the source tree.** Every book
//!    ingest generates either `pub const <NAME>: &[<RecordType>]` or
//!    `pub fn <name>() -> &'static [<RecordType>]` under
//!    `src/rules_core/rules_tables/`. Both are scanned directly off disk, so
//!    a family that was ingested but never wired into the diagnostic still
//!    shows up. A record type this module does not recognize is itself a
//!    failure — a genuinely new kind of content needs a decision about where
//!    it reaches, not a default.
//! 3. **The record files in `data/corpus/`.** Every ingest tool writes Shape B
//!    v1 JSON records to `data/corpus/<book>/<kind>/`, and the count of those
//!    files is the most direct statement of "this content was ingested" the
//!    repo makes. It is the only source that sees content the compiled tables
//!    never carry.
//!
//! ## Why source 3 exists — two blind spots, both of which had already hidden a book
//!
//! Source 2 originally read `pub const` declarations only, which made
//! `pathfinder_unchained` invisible to it: PU's records sit inside
//! `pub fn equipment_tables()` and `pub fn feat_tables()` accessor bodies. That
//! half is now closed by teaching the scanner the accessor shape.
//!
//! The second blind spot was worse, because it hid *the headline content of a
//! whole book*. ARG's 153 alternate racial traits are not a compiled table at
//! all — `decisions.md §24` rules out a formula interpreter, so they are read
//! from `data/corpus/advanced_race_guide/race_trait/` at runtime by
//! `codex::rules_core::race_resolver`, which lives outside `rules_tables/`
//! entirely. Neither discovery source could name them:
//! `corpus_ingest_diagnostic` reports ARG's feats, spells and equipment and not
//! its traits, and the source scan reads a directory they were never in. Every
//! reach test passed without the gate ever asking about the Advanced Race
//! Guide's reason to exist.
//!
//! Scanning `data/corpus/` closes it, and closes it for the general case: any
//! future book whose content is corpus-backed rather than compiled is visible
//! the moment its records land on disk, whether or not anyone remembers to
//! wire up a diagnostic.
//!
//! ## Why a mere reference does not satisfy it
//!
//! `corpus_ingest_diagnostic` carries a **count** of every book's records to
//! the player. Nothing renders the records. Treating that as reach would make
//! this gate pass on every one of the six defects above, so it is disqualified
//! by construction and `a_count_does_not_satisfy_the_gate` pins it.
//!
//! The same rule applies one level down. A record whose identity crosses the
//! boundary and nothing else is exactly the Feats-tab defect: the player saw
//! `feat:deflect_arrows` where the feat's name and text should have been. So
//! every claim names the field(s) the render path actually reads, and
//! [`assess`] rejects an entry that arrives carrying only its own key — for
//! **every** record, not merely most of them.
//!
//! ## Scope boundary
//!
//! This gate answers "do this book's records reach a surface at all, carrying
//! something the player can read". It deliberately does not check that a
//! surface is *correct*, that every field crosses, or that a React component
//! is mounted. Those are other tests' jobs. A gate that tried to prove
//! everything would be argued down to proving nothing.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::rules_tables::acg::AcgClassId;
use codex::rules_core::rules_tables::apg::ApgClassId;
use codex::rules_core::rules_tables::crb::class_tables::ClassId;
use codex::rules_core::rules_tables::crb::race_tables::RaceId;
use codex::rules_core::rules_tables::feats_all::all_feat_tables;
use codex::rules_core::rules_tables::pathfinder_unchained::class_chassis::PuClassId;
use codex::rules_core::rules_tables::{
    acg, advanced_race_guide as arg, apg, beastiary1, crb, pathfinder_unchained as pu, RuleSetId,
};

use crate::corpus_ingest_diagnostic::build_corpus_ingest_diagnostic;

// ---------------------------------------------------------------------------
// Identity
// ---------------------------------------------------------------------------

/// One ingested content family: a book and a kind of record in it, named
/// exactly the way the app's own ingest diagnostic names them (`"crb"` /
/// `"spells"`), so the two inventories join without a translation table
/// somebody has to remember to update.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Family {
    book: String,
    kind: String,
}

impl Family {
    fn new(book: &str, kind: &str) -> Self {
        Family {
            book: book.to_owned(),
            kind: kind.to_owned(),
        }
    }

    fn label(&self) -> String {
        format!("{}/{}", self.book, self.kind)
    }
}

// ---------------------------------------------------------------------------
// The verdict
// ---------------------------------------------------------------------------

/// Whether a family's records reach the player.
///
/// Deliberately not a `bool`, for the reason
/// `description_completion::ZeroMagnitudeResolution` gives: "reaches the
/// player, here is the surface" and "does not reach the player, here is
/// precisely what is missing" are different facts, and a codebase that
/// collapses them loses the ability to report the second one usefully.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Reach {
    /// Every ingested record appears in a real IPC response carrying at least
    /// one field the render path reads.
    Surfaced {
        /// The command whose response was actually executed to prove this.
        surface: &'static str,
        records: usize,
    },
    /// A surface carries this family, but some records arrive as an
    /// identifier and nothing else — the player gets a row with a name and
    /// empty columns.
    ///
    /// Deliberately distinct from `NotSurfaced`. Collapsing the two would say
    /// "APG spells reach no surface" about a book whose other 285 spells
    /// render completely, and a finding that overstates gets argued with
    /// instead of fixed. They also have different remedies: this one is fixed
    /// in the ingest or the record, not by wiring a command.
    BareRecords {
        surface: &'static str,
        records: usize,
        bare: BTreeSet<String>,
    },
    NotSurfaced {
        why: String,
        /// The ingested keys that appear in no response at all, when the
        /// verdict has that shape. Carried rather than only counted so
        /// [`UNREACHED_RECORD_FINDINGS`] can pin them by exact key, both ways
        /// — the same reason [`BARE_RECORD_FINDINGS`] exists. Empty when the
        /// family failed for a reason that is not about particular records
        /// (nothing ingested, a surface that could not be read at all).
        missing: BTreeSet<String>,
    },
}

impl Reach {
    fn is_surfaced(&self) -> bool {
        matches!(self, Reach::Surfaced { .. })
    }

    /// Whether a player-facing surface carries this family at all — true for
    /// both `Surfaced` and `BareRecords`, because in the latter case the
    /// command, the DTO and the render path all genuinely exist.
    fn has_a_surface(&self) -> bool {
        !matches!(self, Reach::NotSurfaced { .. })
    }
}

// ---------------------------------------------------------------------------
// The assessment core
//
// Split out from every probe so the payload rule has exactly one
// implementation, and so it can be exercised against synthetic inputs — a
// gate whose central rule is never tested against a failing case is a gate
// nobody has evidence bites.
// ---------------------------------------------------------------------------

/// Judge one family from three sets of record keys:
///
/// * `ingested` — every key the engine's own table holds. The denominator,
///   read from live data, never a remembered count.
/// * `with_payload` — keys that appear in the real IPC response carrying at
///   least one non-identity field the render path reads.
/// * `identity_only` — keys that appear in the response, but with nothing
///   beyond their own identity. These are the Feats-tab defect. They are
///   counted as **not** reaching the player, because a player looking at the
///   screen learns nothing from them.
fn assess(
    surface: &'static str,
    ingested: &BTreeSet<String>,
    with_payload: &BTreeSet<String>,
    identity_only: &BTreeSet<String>,
) -> Reach {
    if ingested.is_empty() {
        return Reach::NotSurfaced {
            why: "nothing is ingested for this family, so the inventory that named it is wrong"
                .to_owned(),
            missing: BTreeSet::new(),
        };
    }

    // Absent entirely is the more severe finding, and is checked first: a
    // family nothing serves has no surface to speak of.
    let seen: BTreeSet<String> = with_payload.union(identity_only).cloned().collect();
    let missing: BTreeSet<String> = ingested.difference(&seen).cloned().collect();
    if !missing.is_empty() {
        return Reach::NotSurfaced {
            why: format!(
                "{} of {} ingested records never appear in `{}` (e.g. {})",
                missing.len(),
                ingested.len(),
                surface,
                sample(missing.iter().cloned())
            ),
            missing,
        };
    }

    let bare: BTreeSet<String> = ingested.intersection(identity_only).cloned().collect();
    if !bare.is_empty() {
        return Reach::BareRecords {
            surface,
            records: ingested.len(),
            bare,
        };
    }

    Reach::Surfaced {
        surface,
        records: ingested.len(),
    }
}

/// A few example keys for a failure message. A gate that reports only a count
/// makes the reader go find the records themselves, which is how a finding
/// gets deferred.
fn sample(keys: impl Iterator<Item = String>) -> String {
    let shown: Vec<String> = keys.take(4).collect();
    shown.join(", ")
}

// ---------------------------------------------------------------------------
// Inventory source 1 — the app's own live ingest diagnostic
// ---------------------------------------------------------------------------

/// Every `(book, kind)` the shipped ingest diagnostic reports as having real
/// records. This is the same data the Corpus Ingest panel shows a player.
fn diagnostic_inventory() -> BTreeSet<Family> {
    build_corpus_ingest_diagnostic()
        .into_iter()
        .flat_map(|book| {
            let book_id = book.book_id;
            book.content_kind_counts
                .into_iter()
                .filter(|(_, count)| *count > 0)
                .map(move |(kind, _)| Family::new(&book_id, &kind))
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Inventory source 2 — the record slices on disk
// ---------------------------------------------------------------------------

/// Repo root, from this crate's compile-time manifest dir rather than the
/// process's cwd — the same derivation `corpus_ingest_diagnostic` uses, and
/// for the same reason.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Element types that ARE a player-facing content family, mapped to the kind
/// name the ingest diagnostic uses.
const RECORD_TYPE_KINDS: &[(&str, &str)] = &[
    ("FeatTableEntry", "feats"),
    // Ultimate Campaign's own record type (SD28-E13) -- not
    // `crb::feats::FeatTableEntry`, see `ultimate_campaign::feat_tables`'s
    // own doc comment for why it declares its own type. Same family
    // (`"feats"`) as every other book's feat table.
    ("StoryFeatEntry", "feats"),
    ("SpellListEntry", "spells"),
    ("EquipmentTableEntry", "equipment"),
    ("WeaponTableEntry", "weapons"),
    // PU's per-class feature tables. Visible to this scanner only since it
    // learned the accessor-function shape: both sit inside
    // `pub fn features() -> &'static [..]`. The other two Unchained classes
    // expose theirs as an indented `impl` const, which is deliberately not a
    // record slice — the family is discovered anyway, three ways over.
    ("UnchainedBarbarianFeature", "class_features"),
    ("UnchainedMonkFeature", "class_features"),
];

/// Element types that are real ingested data but are **not** an independent
/// family — each is a facet of a family already covered, and reaches the
/// player through that family's own surface. Listed with the reason, because
/// "not a family" is a judgement and an unexplained exclusion list is how a
/// gate quietly stops covering things.
const SUPPORTING_RECORD_TYPES: &[(&str, &str)] = &[
    (
        "ClassWeaponProficiency",
        "a per-class facet of crb/weapons, not a separate record family: it says which of \
         WEAPON_TABLE's weapons a class may use, and has no identity of its own to render",
    ),
    (
        "GroundedAttack",
        "provenance for beastiary1/monsters' natural attacks — a citation attached to a monster \
         record, not a record a player selects or browses independently",
    ),
    (
        "RaceTraitEntry",
        "the pre-corpus hand-modelled CRB racial-trait index in \
         rules_tables/crb/race_tables.rs. The player-facing family it once served is now \
         `crb/race_traits`, read from data/corpus/core_rulebook/race_trait/ and served by \
         list_alternate_racial_traits; race_catalog.rs stopped importing it when that landed. \
         What is left is an engine-internal lookup (pilot_compute reads a CRB race's base walk \
         speed out of it) plus provenance rows in support_state_matrix — no identity a player \
         selects or browses",
    ),
];

/// Scan `src/rules_core/rules_tables/` for generated record slices.
///
/// Returns the families found, plus any record type this module does not
/// recognize. An unrecognized type is not skipped: a new kind of ingested
/// content is precisely the event this gate exists for, and defaulting it to
/// "probably fine" would reintroduce the whole defect class on book 5.
///
/// **The `pub const`-only blind spot is closed (SD-27, 2026-07-31).** This
/// used to read column-zero `pub const NAME: &[Type]` declarations only, which
/// made a book whose records live inside an accessor function body invisible
/// to it — `pathfinder_unchained` is exactly that shape, its records sitting
/// inside `pub fn equipment_tables()` and `pub fn feat_tables()`. PU was for a
/// time invisible to *both* discovery sources at once, and this gate asserted
/// nothing about the book in either direction. The diagnostic was taught to
/// report it first; [`slice_element_type`] now also reads
/// `pub fn name() -> &'static [Type]`, so PU no longer rests on one source.
fn scanned_inventory() -> (BTreeSet<Family>, Vec<String>) {
    let root = repo_root().join("src/rules_core/rules_tables");
    let mut families = BTreeSet::new();
    let mut unknown = Vec::new();

    for path in rust_files_under(&root) {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let Some(book) = book_of(&root, &path) else {
            // A slice directly under rules_tables/ belongs to no book; there
            // are none today and one would need its own decision.
            continue;
        };

        for line in text.lines() {
            let Some(element) = slice_element_type(line) else {
                continue;
            };
            if let Some((_, kind)) = RECORD_TYPE_KINDS.iter().find(|(ty, _)| *ty == element) {
                families.insert(Family::new(&book, kind));
            } else if !SUPPORTING_RECORD_TYPES.iter().any(|(ty, _)| *ty == element) {
                unknown.push(format!(
                    "{}: `{}` records in book `{}` — an ingested record type this gate does not \
                     know. Add it to RECORD_TYPE_KINDS with the surface that renders it, or to \
                     SUPPORTING_RECORD_TYPES with why it is a facet of another family.",
                    path.strip_prefix(repo_root())
                        .unwrap_or(&path)
                        .display(),
                    element,
                    book
                ));
            }
        }
    }

    (families, unknown)
}

fn rust_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(rust_files_under(&path));
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            found.push(path);
        }
    }
    found
}

/// The book directory a file belongs to (`crb`, `apg`, ...), i.e. the first
/// path component under `rules_tables/`.
fn book_of(root: &Path, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(root).ok()?;
    let mut components = relative.components();
    let first = components.next()?;
    // A file directly at this level, rather than inside a book directory,
    // belongs to no book — so requiring a second component is what makes
    // `first` a directory name.
    components.next()?;
    Some(first.as_os_str().to_string_lossy().into_owned())
}

/// Extracts `Foo` from a line declaring a record table, in either of the two
/// shapes the ingest tools generate:
///
/// ```text
/// pub const SPELL_LIST: &[SpellListEntry] = &[
/// pub fn equipment_tables() -> &'static [EquipmentTableEntry] {
/// ```
///
/// The second shape is why `pathfinder_unchained` was invisible to this
/// scanner: every one of PU's tables is behind an accessor, so a `pub const`
/// reader saw an ingested book as an empty directory.
///
/// Only top-level (column-zero) declarations count — an indented `pub const
/// ALL: &[ClassId]` inside an `impl` block is an enum roster, not an ingested
/// record slice. Tuple and primitive element types (`&[(&str, u8)]`,
/// `&[&str]`) are per-class index tables over records that already exist
/// elsewhere, not record families.
fn slice_element_type(line: &str) -> Option<&str> {
    let rest = if let Some(rest) = line.strip_prefix("pub const ") {
        rest.split_once(": &[")?.1
    } else if let Some(rest) = line.strip_prefix("pub fn ") {
        // `name() -> &'static [Type] {`. The lifetime is always spelled out in
        // a returned static slice, so requiring it costs nothing and keeps
        // this from matching an unrelated `-> &[u8]` helper.
        rest.split_once("-> &'static [")?.1
    } else {
        return None;
    };
    let (element, _) = rest.split_once(']')?;
    let element = element.trim();
    if element.is_empty()
        || !element
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase())
    {
        return None;
    }
    Some(element)
}

// ---------------------------------------------------------------------------
// Inventory source 3 — the record files in `data/corpus/`
// ---------------------------------------------------------------------------

/// Corpus book directory -> the `book_id` the ingest diagnostic uses, so all
/// three inventories join on one identity.
///
/// A translation table is not free, and this one is deliberately small and
/// fails closed: [`corpus_inventory`] reports an unmapped directory as an
/// error rather than skipping it, so adding a book without deciding what it is
/// called breaks the build instead of silently exempting the book.
const CORPUS_BOOK_IDS: &[(&str, &str)] = &[
    ("core_rulebook", "crb"),
    ("advanced_players_guide", "apg"),
    ("advanced_class_guide", "acg"),
    // Pre-existing spelling of the Bestiary 1 directory; `beastiary1` is what
    // the diagnostic and every existing claim call the book.
    ("beastiary", "beastiary1"),
    ("advanced_race_guide", "advanced_race_guide"),
    ("pathfinder_unchained", "pathfinder_unchained"),
];

/// Corpus content-kind directory (singular, as the ingest tools write it) ->
/// the plural kind name the diagnostic and every claim use. Fails closed for
/// the same reason [`CORPUS_BOOK_IDS`] does.
const CORPUS_KIND_NAMES: &[(&str, &str)] = &[
    ("class", "classes"),
    ("class_feature", "class_features"),
    ("equipment", "equipment"),
    ("feat", "feats"),
    ("monster", "monsters"),
    ("race", "races"),
    ("race_trait", "race_traits"),
    ("spell", "spells"),
];

/// Directories under a corpus book that hold no player-facing records. Listed
/// with the reason, for the same reason [`SUPPORTING_RECORD_TYPES`] is: an
/// unexplained exclusion is how a gate quietly stops covering something.
const NON_CONTENT_CORPUS_DIRS: &[(&str, &str)] = &[(
    "_parity",
    "an ingest-verification artifact (a book's parity report against its \
     upstream LST), not game content: nothing in it describes a rule, and a \
     player is not meant to read it",
)];

/// Every `(book, kind)` with at least one Shape B v1 record on disk, plus any
/// directory this module cannot name.
fn corpus_inventory() -> (BTreeSet<Family>, Vec<String>) {
    let root = repo_root().join("data/corpus");
    let mut families = BTreeSet::new();
    let mut unknown = Vec::new();

    let Ok(books) = fs::read_dir(&root) else {
        unknown.push(format!("cannot read the corpus root {}", root.display()));
        return (families, unknown);
    };
    for book_entry in books.flatten() {
        let book_dir = book_entry.path();
        if !book_dir.is_dir() {
            continue;
        }
        let book_name = book_entry.file_name().to_string_lossy().into_owned();
        let Some((_, book_id)) = CORPUS_BOOK_IDS.iter().find(|(dir, _)| *dir == book_name) else {
            unknown.push(format!(
                "data/corpus/{book_name}/ is an ingested book this gate cannot name. Add it to \
                 CORPUS_BOOK_IDS with the `book_id` the ingest diagnostic uses."
            ));
            continue;
        };

        let Ok(kinds) = fs::read_dir(&book_dir) else { continue };
        for kind_entry in kinds.flatten() {
            let kind_dir = kind_entry.path();
            if !kind_dir.is_dir() {
                continue;
            }
            let kind_name = kind_entry.file_name().to_string_lossy().into_owned();
            if NON_CONTENT_CORPUS_DIRS.iter().any(|(dir, _)| *dir == kind_name) {
                continue;
            }
            let Some((_, kind)) = CORPUS_KIND_NAMES.iter().find(|(dir, _)| *dir == kind_name) else {
                unknown.push(format!(
                    "data/corpus/{book_name}/{kind_name}/ is an ingested content kind this gate \
                     cannot name. Add it to CORPUS_KIND_NAMES with the kind name the ingest \
                     diagnostic uses, or to NON_CONTENT_CORPUS_DIRS with why it holds no records."
                ));
                continue;
            };
            if json_files_under(&kind_dir).is_empty() {
                continue;
            }
            families.insert(Family::new(book_id, kind));
        }
    }

    (families, unknown)
}

fn json_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(json_files_under(&path));
        } else if path.extension().is_some_and(|ext| ext == "json") {
            found.push(path);
        }
    }
    found
}

/// Every record key on disk for one `(book directory, kind directory)`.
///
/// The denominator for a corpus-backed family's claim, read from the record
/// files themselves so it is never a remembered count and never the same data
/// the serving path returns.
fn corpus_record_keys(book_dir: &str, kind_dir: &str) -> BTreeSet<String> {
    corpus_record_field(book_dir, kind_dir, "key")
}

/// The same denominator for the record families whose Shape B v1 `data` object
/// carries its identity as `id` rather than `key`.
///
/// Bestiary 1's monster records are the one such family today: they were
/// written by the SD-22 monster ingest, which predates the `key` convention and
/// writes the canonical `beastiary1:monster:<slug>` identity as `data.id`. That
/// is not a defect to paper over here — reading the wrong field would silently
/// return an empty denominator and make the monsters' claim pass while checking
/// nothing, which is why `corpus_record_field` is asserted non-empty at every
/// call site below.
fn corpus_record_ids(book_dir: &str, kind_dir: &str) -> BTreeSet<String> {
    corpus_record_field(book_dir, kind_dir, "id")
}

fn corpus_record_field(book_dir: &str, kind_dir: &str, field: &str) -> BTreeSet<String> {
    let dir = repo_root().join("data/corpus").join(book_dir).join(kind_dir);
    json_files_under(&dir)
        .into_iter()
        .filter_map(|path| {
            let text = fs::read_to_string(&path).ok()?;
            let value: serde_json::Value = serde_json::from_str(&text).ok()?;
            value.get("data")?.get(field)?.as_str().map(str::to_owned)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The claims
// ---------------------------------------------------------------------------

/// The reach claim for a family, executed against the live IPC boundary.
///
/// `None` means no claim is declared — the gate's hard failure. Every arm
/// below runs a real builder function; none consults a doc comment, a status
/// table or a recorded count.
fn reach_of(family: &Family) -> Option<Reach> {
    match (family.book.as_str(), family.kind.as_str()) {
        // Feats: `list_feat_catalog` serves all books' records with the
        // corpus category and `DESC:` text. The Feats tab and the Add Feat
        // picker both render from this response
        // (apps/desktop/src/characterHub/featsTabModel.ts
        // `resolveSelectedFeatEntries` -> `itemPickerFilter.ts`
        // `mapFeatCatalogEntries`, which folds category + description into the
        // `detail` line the sheet prints under each feat's name).
        ("crb", "feats") => Some(feats_reach(RuleSetId::Crb, "Crb")),
        ("apg", "feats") => Some(feats_reach(RuleSetId::Apg, "Apg")),
        ("acg", "feats") => Some(feats_reach(RuleSetId::Acg, "Acg")),
        // ARG joined `feats_all::all_feat_tables()` after the APG/ACG
        // widening, so the same command now carries its records too. This
        // claim replaces the OPEN_FINDINGS entry that recorded the gap.
        ("advanced_race_guide", "feats") => Some(feats_reach(RuleSetId::Arg, "Arg")),
        // PU joined `feats_all::all_feat_tables()` alongside ARG; the same
        // command carries its 17 records under the `Pu` wire source.
        ("pathfinder_unchained", "feats") => Some(feats_reach(RuleSetId::Pu, "Pu")),
        // SD28-E13: Ultimate Campaign's 23 Story Feats joined
        // `feats_all::all_feat_tables()` under the `Uca` wire source. All 23
        // records carry `category: "Story"`, so every one of them has a
        // payload per `feats_reach`'s own check (a non-empty category is
        // sufficient) -- including the 3 `deferred-with-reason` records,
        // which still carry real flavor text plus the deferral diagnostic
        // in `description`, not a bare identity.
        ("ultimate_campaign", "feats") => Some(feats_reach(RuleSetId::Uca, "Uca")),

        // Spells: `list_spell_catalog` serves all books. The Spell Catalog
        // screen renders school/level/description; the sheet's Add Spell
        // picker reads the same response
        // (apps/desktop/src/characterHub/spellsTabModel.ts).
        ("crb", "spells") => Some(spells_reach(
            "CRB",
            crb::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        ("apg", "spells") => Some(spells_reach(
            "APG",
            apg::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        ("acg", "spells") => Some(spells_reach(
            "ACG",
            acg::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // ARG joined `build_spell_catalog` in the SD-27 widening, so the same
        // command now carries its records. SpellCatalogScreen.tsx's default
        // "All books" view renders every entry the response carries, so an ARG
        // row reaches the player with its school, level and text even though
        // the screen's `BOOK_ORDER` chip row does not yet offer an ARG filter
        // (a completeness gap in the filter UI, not a reach gap).
        ("advanced_race_guide", "spells") => Some(spells_reach(
            "ARG",
            arg::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),

        // Equipment: `list_equipment_catalog` / `list_equipment` serve every
        // ingested book's table since the SD-27 widening of
        // `build_equipment_catalog`, rendered by apps/desktop/src/
        // equipmentCatalog/EquipmentCatalogScreen.tsx, which filters by
        // category and search only — never by book — so every book's rows
        // appear with their category label and cost.
        ("crb", "equipment") => Some(equipment_reach(
            "CRB",
            crb::equipment_tables::equipment_tables()
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        ("apg", "equipment") => Some(equipment_reach(
            "APG",
            apg::equipment_tables::EQUIPMENT_TABLE
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        ("acg", "equipment") => Some(equipment_reach(
            "ACG",
            acg::equipment_tables::equipment_tables()
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        ("beastiary1", "equipment") => Some(equipment_reach(
            "B1",
            beastiary1::equipment_tables::EQUIPMENT_TABLE
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        ("advanced_race_guide", "equipment") => Some(equipment_reach(
            "ARG",
            arg::equipment_tables::equipment_tables()
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        ("pathfinder_unchained", "equipment") => Some(equipment_reach(
            "PU",
            pu::equipment_tables::equipment_tables()
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),

        // Races: `list_race_catalog` serves every race's trait bundle, each
        // row carrying the trait's own name and derivation prose, rendered by
        // apps/desktop/src/raceCatalog/RaceCatalogScreen.tsx. The same corpus
        // backs `character_hub`'s creation roster, so a race claimed here is a
        // race a player can actually make.
        //
        // Bestiary 1's eleven races became visible to this gate only when
        // `corpus_ingest_diagnostic` started reporting a per-book race count
        // (SD-27, 2026-07-31). They were already reaching a player — the gate
        // simply had no `beastiary1`/`races` family to ask about, because the
        // panel it derives families from reported none.
        ("crb", "races") => Some(races_reach(
            "CRB",
            RaceId::ALL.iter().map(|id| format!("{id:?}")).collect(),
        )),
        ("beastiary1", "races") => Some(races_reach(
            "B1",
            crate::race_catalog::ingested_race_ids_for_book("beastiary"),
        )),

        // Racial traits: `list_alternate_racial_traits` serves every race's
        // standard rows and every ARG alternate, and
        // `resolve_race_alternate_selection` serves the rows a selection
        // grants. All three are rendered by apps/desktop/src/raceCatalog/
        // AlternateTraitPicker.tsx — standard rows in the left column (name +
        // book, struck through and captioned when a selection replaced them),
        // alternates in the right column as checkable rows carrying name,
        // book and the record's own corpus prose, and flag-granted rows
        // appended to the left column captioned "Granted by your selection".
        //
        // **This family is the reason source 3 exists.** ARG's 153 alternate
        // racial traits are the book's headline content and were invisible to
        // both other discovery sources: they are not a compiled table
        // (`decisions.md §24` rules out an interpreter, so `race_resolver`
        // reads them off disk at runtime) and `corpus_ingest_diagnostic`
        // reports ARG's feats, spells and equipment but not its traits. Every
        // reach test passed without ever asking about them.
        ("crb", "race_traits") => Some(race_traits_reach("CRB", "core_rulebook")),
        ("beastiary1", "race_traits") => Some(race_traits_reach("B1", "beastiary")),
        ("advanced_race_guide", "race_traits") => {
            Some(race_traits_reach("ARG", "advanced_race_guide"))
        }

        // Weapons: `list_weapon_targets` serves WEAPON_TABLE to the chooser
        // feat's "which weapon?" step, each row carrying the record's damage
        // die and threat range as a rendered detail line
        // (apps/desktop/src/characterHub/featTargetOptions.ts ->
        // ItemPickerModal.tsx).
        ("crb", "weapons") => Some(weapons_reach()),

        // Classes: every ingested class is offered by the character-creation
        // and level-up pickers, with the label the player reads, from
        // CLASS_OPTIONS in apps/desktop/src/characterHub/characterHubModel.ts
        // (rendered by CreateCharacterForm.tsx's `<select>` and
        // LevelUpDialog.tsx). Their level-by-level progression numbers reaching
        // the sheet is proven separately, and per level, by
        // `v06_class_state_dump` — see scripts/verify.sh's `class-dump` stage.
        ("crb", "classes") => Some(classes_reach(
            ClassId::ALL
                .iter()
                .map(|id| format!("class:{}", format!("{id:?}").to_lowercase()))
                .collect(),
        )),
        ("apg", "classes") => Some(classes_reach(
            ApgClassId::ALL
                .iter()
                .map(|id| format!("class:{}", id.name()))
                .collect(),
        )),
        ("acg", "classes") => Some(classes_reach(
            AcgClassId::ALL
                .iter()
                .map(|id| format!("class:{}", id.name()))
                .collect(),
        )),
        // Pathfinder Unchained's four replacement classes joined CLASS_OPTIONS
        // in the SD-27 wiring, under their own `class:unchained_*` ids so a
        // selection can never resolve the class they replace.
        ("pathfinder_unchained", "classes") => Some(classes_reach(
            PuClassId::ALL
                .iter()
                .map(|id| format!("class:{}", id.name()))
                .collect(),
        )),

        // Monsters: `list_monster_catalog` serves every Bestiary 1 stat block
        // with its challenge rating, size, creature type, land speed, source
        // page and natural attacks, rendered by apps/desktop/src/monsterCatalog/
        // MonsterCatalogScreen.tsx — reachable from the landing screen's
        // "Browse Monster Catalog" link, alongside the other catalogs.
        //
        // This claim replaces the OPEN_FINDINGS entry that recorded the gap.
        // The Pets tab still does not count and never did: its companion stat
        // block is computed by `pilot_compute`'s own
        // `ground_*_companion_stat_block`, not read from these tables.
        ("beastiary1", "monsters") => Some(monsters_reach()),

        // PU class features: each of the four Unchained classes emits one
        // roster row per ingested `class_feature` record the character holds,
        // carrying that record's own corpus `KEY:` token, and the character
        // sheet's "Class Features & Special Abilities" section renders them
        // (CharacterSheet.tsx via classFeaturesModel.ts).
        //
        // **This is the claim the OPEN_FINDINGS entry said could not be
        // written.** Its blocker was identity, not a missing screen: the
        // receipt rows were named semantically
        // (`class_feature.pu.unchained_rogue.sneak_attack_dice`) while the
        // corpus record is keyed `Unchained Rogue ~ Sneak Attack`, and joining
        // the two here would have been a hand-written mapping this file
        // forbids. `pilot_compute::pu_class_feature_citation` now puts the key
        // on the receipt at the point of emission, and
        // `pu_class_feature_cited_key` reads it back off the live response —
        // so the join is the engine's own statement, executed, not this
        // module's guess.
        ("pathfinder_unchained", "class_features") => Some(pu_class_features_reach()),

        _ => None,
    }
}

fn feats_reach(book: RuleSetId, wire_source: &str) -> Reach {
    let ingested: BTreeSet<String> = all_feat_tables()
        .iter()
        .filter(|table| table.rule_set == book)
        .flat_map(|table| table.entries.iter().map(|entry| entry.key.to_owned()))
        .collect();

    let response = crate::feat_catalog::build_feat_catalog();
    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for entry in response.entries.iter().filter(|e| e.source == wire_source) {
        // What the sheet actually prints under a feat's name is its category
        // and its corpus description. A record carrying neither would render
        // as a bare name — and for these tables `key == name` for all but a
        // handful of APG records, so the name alone is the identity.
        let has_payload = !entry.category.trim().is_empty()
            || entry
                .description
                .as_deref()
                .is_some_and(|text| !text.trim().is_empty());
        if has_payload {
            with_payload.insert(entry.key.clone());
        } else {
            identity_only.insert(entry.key.clone());
        }
    }

    assess(
        "list_feat_catalog",
        &ingested,
        &with_payload,
        &identity_only,
    )
}

fn spells_reach(wire_book: &str, ingested: BTreeSet<String>) -> Reach {
    let response = crate::spell_catalog::build_spell_catalog();
    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for entry in response.entries.iter().filter(|e| e.book == wire_book) {
        // School, level and description are the three things the catalog
        // screen and the Add Spell picker render. APG's table carries genuine
        // absences in each of them individually (16 records with no school,
        // 41 with no level, 12 with no description — see spell_catalog.rs), so
        // requiring all three would fail on honest corpus gaps. Requiring at
        // least one is the real bar: a spell arriving with none of them is a
        // name and nothing else.
        let has_payload = entry.school.as_deref().is_some_and(|s| !s.trim().is_empty())
            || entry.level.is_some()
            || entry
                .description
                .as_deref()
                .is_some_and(|d| !d.trim().is_empty());
        if has_payload {
            with_payload.insert(entry.key.clone());
        } else {
            identity_only.insert(entry.key.clone());
        }
    }

    assess(
        "list_spell_catalog",
        &ingested,
        &with_payload,
        &identity_only,
    )
}

/// Equipment reach for one book, judged against the real
/// `list_equipment_catalog` response filtered to that book's wire code.
///
/// Filtering by `wire_book` rather than scanning the whole response is what
/// makes the claim per-book honest: a key that only another book serves must
/// not count as this book's record arriving.
fn equipment_reach(wire_book: &str, ingested: BTreeSet<String>) -> Reach {
    let response = crate::equipment_catalog::build_equipment_catalog();
    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for entry in response.entries.iter().filter(|e| e.book == wire_book) {
        // `key` frequently equals `name` for equipment records, so the name is
        // not payload beyond identity. The category chip and the cost are what
        // the catalog screen adds.
        let has_payload = !entry.category.trim().is_empty() || entry.cost_gp.is_some();
        if has_payload {
            with_payload.insert(entry.key.clone());
        } else {
            identity_only.insert(entry.key.clone());
        }
    }

    assess(
        "list_equipment_catalog",
        &ingested,
        &with_payload,
        &identity_only,
    )
}

/// One book's races claim.
///
/// `book_code` is the catalog's wire code (`"CRB"`, `"B1"`) and narrows the
/// served rows to that book's own; `ingested` is the set the book is asserted
/// to declare, supplied by the caller from a source *other* than the catalog
/// so the two sides of the claim stay independent.
fn races_reach(book_code: &str, ingested: BTreeSet<String>) -> Reach {
    // One race contributes many trait rows, so a race counts as reaching the
    // player once any of its rows carries both a trait name and its derivation
    // prose, and counts as bare only if it appears with no such row at all.
    let response = crate::race_catalog::build_race_catalog();
    let mut with_payload = BTreeSet::new();
    let mut seen = BTreeSet::new();
    for entry in response.entries.iter().filter(|entry| entry.book == book_code) {
        seen.insert(entry.race_id.clone());
        if !entry.trait_name.trim().is_empty() && !entry.detail.trim().is_empty() {
            with_payload.insert(entry.race_id.clone());
        }
    }
    let identity_only: BTreeSet<String> = seen.difference(&with_payload).cloned().collect();

    assess(
        "list_race_catalog",
        &ingested,
        &with_payload,
        &identity_only,
    )
}

/// One book's racial-trait claim, executed against both live picker commands.
///
/// `wire_book` narrows the served rows to this book's own (`"ARG"`);
/// `book_dir` is the `data/corpus/` directory the denominator is read from, so
/// the two sides of the claim come from genuinely different places — the
/// record files on disk versus the IPC responses the screen renders.
///
/// Three surfaces are read, because a racial trait reaches a player by three
/// different routes and counting only the first would report the other two as
/// unreached:
///
/// 1. `list_alternate_racial_traits` -> `standardTraits[]`, the left column.
/// 2. the same response's `alternates[]`, the checkable right column.
/// 3. `resolve_race_alternate_selection` -> `appliedTraits[]` with role
///    `flagGranted`, which the left column appends. A replacement row like
///    `Saltbeard ~ Dwarf ~ Greed` is never in the menu — it arrives only
///    because a chosen alternate fired its flag — so it is reachable only
///    through a *selection*, and the claim makes that selection rather than
///    assuming the row is fine. `flagGranted` covers both of the resolver's
///    grant shapes: the positive `PREFACT` flag round-trip above, and an
///    alternate naming its replacement directly with
///    `ABILITY:<cat>|AUTOMATIC|<key>` (`Orc ~ Feral` -> `Feral ~ Languages`).
///
/// (3) resolves each alternate **on its own**, once per alternate, rather than
/// selecting the whole menu at once. Selecting everything looks like the
/// stronger probe and is actually weaker: with all five Duergar alternates
/// chosen, `Duergar ~ Deep Magic` fires `Duergar_ReplaceSpellLikeAbilities`,
/// which suppresses the two spell-like-ability rows that other selections
/// grant — so a row genuinely reachable by a real character reads as
/// unreachable. One-at-a-time answers the question that matters: is there a
/// choice a player can make that brings this row in.
fn race_traits_reach(wire_book: &'static str, book_dir: &str) -> Reach {
    let ingested = corpus_record_keys(book_dir, "race_trait");

    let menu = crate::race_trait_picker::build_alternate_racial_traits();
    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();

    for race in &menu.races {
        // The left column prints a standard row's name (and its book), never
        // its description — so the name is the payload, and it is genuinely
        // more than identity here: the key is `Aasimar ~ Skilled` and the name
        // is `Skilled`.
        for standard in race.standard_traits.iter().filter(|row| row.book == wire_book) {
            if standard.name.trim().is_empty() {
                identity_only.insert(standard.key.clone());
            } else {
                with_payload.insert(standard.key.clone());
            }
        }
        // An alternate is a checkable row carrying its name and its corpus
        // prose. A row with neither would be an unlabelled checkbox.
        for alternate in race.alternates.iter().filter(|row| row.book == wire_book) {
            if alternate.name.trim().is_empty() && alternate.description.trim().is_empty() {
                identity_only.insert(alternate.key.clone());
            } else {
                with_payload.insert(alternate.key.clone());
            }
        }

        for alternate in &race.alternates {
            let selection = crate::race_trait_picker::build_race_selection(
                &crate::race_trait_picker::RaceSelectionRequest {
                    race_key: race.race_key.clone(),
                    selected_alternate_keys: vec![alternate.key.clone()],
                },
            );
            for applied in selection
                .applied_traits
                .iter()
                .filter(|applied| applied.role == "flagGranted" && applied.book == wire_book)
            {
                if applied.name.trim().is_empty() {
                    identity_only.insert(applied.key.clone());
                } else {
                    with_payload.insert(applied.key.clone());
                }
            }
        }
    }

    assess(
        "list_alternate_racial_traits + resolve_race_alternate_selection",
        &ingested,
        &with_payload,
        &identity_only,
    )
}

/// Bestiary 1's monster stat blocks, judged against the real
/// `list_monster_catalog` response.
///
/// The denominator is read from `data/corpus/beastiary/monster/` rather than
/// from `MonsterId::ALL`, so the two sides of the claim come from genuinely
/// different places — the ingested record files on disk versus the IPC response
/// the screen renders. (Those records carry their identity as `data.id`; see
/// [`corpus_record_ids`].)
fn monsters_reach() -> Reach {
    let ingested = corpus_record_ids("beastiary", "monster");

    let response = crate::monster_catalog::build_monster_catalog();
    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for entry in response.entries.iter().filter(|entry| entry.book == "B1") {
        // The catalog row prints the monster's name, its size and creature
        // type, its challenge rating, its land speed and source page, and its
        // natural attacks. `key` is the `beastiary1:monster:<slug>` identity
        // and the name is derived from it, so neither counts as payload: a row
        // reaches the player when it carries something about the creature.
        let has_payload = !entry.race_type.trim().is_empty()
            || !entry.size.trim().is_empty()
            || !entry.source_page.trim().is_empty()
            || !entry.natural_attacks.is_empty();
        if has_payload {
            with_payload.insert(entry.key.clone());
        } else {
            identity_only.insert(entry.key.clone());
        }
    }

    assess(
        "list_monster_catalog",
        &ingested,
        &with_payload,
        &identity_only,
    )
}

/// Pathfinder Unchained's ingested `class_feature` records, judged per record
/// against the real explanation channel the character sheet reads.
///
/// # Why this executes the *whole* IPC path rather than the compute function
///
/// The sheet renders `LoadSavedCharacterResponse.explanations`. The two lines
/// below are verbatim what `character_hub::load_saved_character_at_root` runs to
/// produce that field, so a change that dropped class-feature rows anywhere
/// between the engine and the wire fails here. That distinction is not
/// hypothetical in this file's history: the per-class deferral notice was
/// pushed on the *diagnostic* channel for a whole cycle, and a diagnostic never
/// reaches the frontend at all unless the build comes back `Blocked`.
///
/// # Why level 20
///
/// `MAX_SUPPORTED_LEVEL` is 20 for all four Unchained classes, so a level-20
/// character holds every feature the class's progression ever grants. Probing
/// lower would report features the character genuinely does not have yet as
/// unreached, which is a fact about the character rather than about the wiring.
fn pu_class_features_reach() -> Reach {
    use codex::rules_core::character_input::{
        load_character_input_fixture, CharacterClassLevel,
    };
    use codex::rules_core::pilot_compute::pu_class_feature_cited_key;
    use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;

    let ingested = corpus_record_keys("pathfinder_unchained", "class_feature");

    let fixture_path = repo_root()
        .join("tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");
    let Ok(fixture_text) = fs::read_to_string(&fixture_path) else {
        return Reach::NotSurfaced {
            why: format!(
                "cannot read the shared deterministic character fixture at {}",
                fixture_path.display()
            ),
            missing: BTreeSet::new(),
        };
    };
    let Some(base_input) = load_character_input_fixture(&fixture_text).character_input else {
        return Reach::NotSurfaced {
            why: "the shared deterministic character fixture no longer loads".to_owned(),
            missing: BTreeSet::new(),
        };
    };

    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for class_id in PuClassId::ALL {
        let mut input = base_input.clone();
        input.case_id = Some(format!("reach_gate.pu_class_features.{}", class_id.name()));
        input.chosen.class_levels = vec![CharacterClassLevel {
            class_id: format!("class:{}", class_id.name()),
            level: 20,
        }];

        // Exactly what `load_saved_character_at_root` does to build
        // `LoadSavedCharacterResponse.explanations`.
        let receipt = compute_pilot_with_corpus(&input, crate::corpus_fixtures::corpus_fixture_bundle());
        let wire = crate::character_hub::map_explanations_dto(&receipt.base.explanations);

        for row in &wire {
            let Some(key) = pu_class_feature_cited_key(&row.detail) else {
                continue;
            };
            // The sheet renders `detail` verbatim and nothing else. A row whose
            // whole text is the citation would put a corpus key on screen and
            // call it a class feature — the Feats-tab defect in a new place.
            let without_citation = row.detail.replace(key, "");
            if without_citation.trim().len() > 60 {
                with_payload.insert(key.to_owned());
            } else {
                identity_only.insert(key.to_owned());
            }
        }
    }

    assess(
        "load_saved_character -> explanations (class_feature.pu.*)",
        &ingested,
        &with_payload,
        &identity_only,
    )
}

fn weapons_reach() -> Reach {
    let ingested: BTreeSet<String> = crb::weapon_tables::WEAPON_TABLE
        .iter()
        .map(|entry| entry.key.to_owned())
        .collect();

    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for target in crate::feat_catalog::build_weapon_target_list() {
        if target.detail.trim().is_empty() {
            identity_only.insert(target.key);
        } else {
            with_payload.insert(target.key);
        }
    }

    assess(
        "list_weapon_targets",
        &ingested,
        &with_payload,
        &identity_only,
    )
}

/// Classes reach the player as selectable options with a readable label.
///
/// Read off the real frontend module rather than a Rust mirror of it, because
/// the failure this guards against is precisely a book's classes existing in
/// the engine while the picker never offers them — which is a fact about the
/// frontend list, not about the engine.
fn classes_reach(ingested: BTreeSet<String>) -> Reach {
    let options = match class_options() {
        Ok(options) => options,
        Err(why) => return Reach::NotSurfaced { why, missing: BTreeSet::new() },
    };

    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for (id, label) in options {
        // The `<select>` renders `label`. An option carrying only an id would
        // print the raw `class:alchemist` token at the player.
        if label.trim().is_empty() {
            identity_only.insert(id);
        } else {
            with_payload.insert(id);
        }
    }

    assess(
        "CLASS_OPTIONS (characterHubModel.ts)",
        &ingested,
        &with_payload,
        &identity_only,
    )
}

/// Parses `CLASS_OPTIONS` out of the real frontend module: `id` -> `label`.
fn class_options() -> Result<BTreeMap<String, String>, String> {
    let path = repo_root().join("apps/desktop/src/characterHub/characterHubModel.ts");
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;

    let start = text
        .find("export const CLASS_OPTIONS")
        .ok_or_else(|| format!("{} no longer declares CLASS_OPTIONS", path.display()))?;
    let body = &text[start..];
    let end = body
        .find("\n];")
        .ok_or_else(|| format!("{}'s CLASS_OPTIONS is not terminated", path.display()))?;

    let mut options = BTreeMap::new();
    for line in body[..end].lines() {
        let Some(id) = quoted_after(line, "id:") else {
            continue;
        };
        let label = quoted_after(line, "label:").unwrap_or_default();
        options.insert(id, label);
    }

    if options.is_empty() {
        return Err(format!("{} parsed to zero class options", path.display()));
    }
    Ok(options)
}

/// The single-quoted string following `field` on this line, e.g.
/// `id: 'class:fighter',` -> `class:fighter`.
fn quoted_after(line: &str, field: &str) -> Option<String> {
    let rest = line.split_once(field)?.1;
    let rest = rest.trim_start();
    let rest = rest.strip_prefix('\'')?;
    let (value, _) = rest.split_once('\'')?;
    Some(value.to_owned())
}

// ---------------------------------------------------------------------------
// Open findings
// ---------------------------------------------------------------------------

/// Ingested families this gate cannot certify as reaching the player.
///
/// It can mean the family reaches nothing at all. It can also mean almost every
/// record reaches a player and one does not — the single entry left here is
/// that case, and it says so in its first sentence. Both belong here because
/// the gate's rule is all-or-nothing by design: it refuses partial credit, so
/// anything short of a fully executed claim is a written finding. Read the
/// entry, never the label alone.
///
/// **Two entries were deleted on 2026-07-31, because both were fixed rather
/// than reclassified**, and what each needed is worth recording since the next
/// book will hit the same two shapes:
///
/// * `beastiary1/monsters` — 41 stat blocks reaching nothing at all. It needed
///   a surface: `monster_catalog.rs` + `monsterCatalog/MonsterCatalogScreen.tsx`,
///   the remedy the entry itself named.
/// * `pathfinder_unchained/class_features` — 64 records that demonstrably
///   influenced the sheet, where *which* of them could not be claimed. It
///   needed an **identity**, not a screen: the receipt rows were named
///   semantically while the corpus records are keyed
///   `Unchained Rogue ~ Sneak Attack`. `pilot_compute` now emits one roster row
///   per record carrying that key verbatim, so the join is the engine's own
///   statement and the claim executes against it.
///
/// **This list is pinned in both directions and is not a suppression list.**
/// `unsurfaced_families_are_exactly_the_recorded_findings` computes the
/// unsurfaced set from live behaviour and requires it to *equal* this list, so:
///
/// * an ingested family that reaches nothing and is not listed here fails —
///   which is the case for every future book;
/// * a family listed here that someone actually surfaces also fails, until the
///   entry is deleted. The list can only shrink without a deliberate edit, and
///   an addition is a written finding in a reviewable diff rather than a
///   number quietly going up.
///
/// Each entry states the remedy, so this reads as a work queue rather than a
/// permanent exemption.
const OPEN_FINDINGS: &[(&str, &str, &str)] = &[
    (
        "beastiary1",
        "race_traits",
        "Read the numbers before the label: 107 of Bestiary 1's 108 ingested race-trait records \
         reach a player through `list_alternate_racial_traits`' standard-trait column and \
         `resolve_race_alternate_selection`'s granted rows, and this gate refuses partial credit. \
         ONE does not: `Duergar ~ Spell-Like Ability ~ Invisibility`, whose positive gate is \
         `Duergar_ReplaceSLAEnlargePerson`. Derived, not assumed: no record in `data/corpus/` sets \
         that flag, and `arg_abilities_race.lst` never mentions it (`grep -c \
         'Duergar_ReplaceSLAEnlargePerson|True'` -> 0). Its only setter anywhere in the PCGen \
         checkout is `Duergar ~ Ironskinned` in `monster_codex/mc_abilities_race.lst:16` — a book \
         this project has not registered, audited or ingested, Tier-1 but deferred by \
         `decisions.md §9` and assigned to SD-29's Bestiary bundle by `epic-breakdown.md:150`. So \
         this is not a wiring gap and there is nothing to wire: the row is upstream-unreachable \
         until Monster Codex is in scope. RE-VERIFIED 2026-07-31 rather than inherited: the claim \
         is now executable, not prose. `tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs` \
         derives the empty setter set from the on-disk corpus, proves no Duergar selection (one \
         at a time or all at once) reaches the row, and proves the MIRROR row does — \
         `Duergar ~ Blood Enmity` sets `Duergar_ReplaceSLAInvisibility` and really does grant \
         `Duergar ~ Spell-Like Ability ~ Enlarge Person` — which is what makes 'blocked' the right \
         word instead of 'broken'. That test goes RED the day Monster Codex is ingested, which is \
         how this entry closes. Do NOT close it by hiding the record — a record on disk that no \
         selection can reach is exactly what this gate is for.",
    ),
];

/// Records that reach a real surface carrying nothing but their own key.
///
/// Pinned by exact key, in both directions, for the same reason
/// [`OPEN_FINDINGS`] is: a bare record that is not listed fails the gate, and
/// fixing a listed one fails it too until the key is deleted. A bare *count*
/// would let one record silently swap for another.
///
/// **Empty as of 2026-07-31.** Its only entry was `apg`/`spells`, holding the
/// twelve records that arrived at `list_spell_catalog` with a key and three
/// nulls: eleven PCGen `.COPY=` delta rows whose base spell lives in CRB
/// rather than APG, plus `Wall of Thorms` — an upstream misspelling of
/// `Wall of Thorns.MOD` at `apg_spells.lst:1555`. `apg::spell_list` now
/// resolves all of them against their base record (the typo'd key is
/// preserved verbatim; only the content is filled), so the entry was deleted
/// rather than relaxed. See
/// `tests/sd27_apg_delta_spell_rows_resolve_against_their_base.rs`.
const BARE_RECORD_FINDINGS: &[(&str, &str, &[&str])] = &[];

/// Ingested records that appear in **no** response at all, for a family whose
/// other records do reach a player.
///
/// Pinned by exact key, in both directions, for the same reason
/// [`BARE_RECORD_FINDINGS`] is — and the two are deliberately different
/// findings: a bare record arrives and renders as empty columns, one of these
/// never arrives at all.
///
/// # Why a claim may be declared for a family that is also a written finding
///
/// A family whose records reach nothing at all has no `reach_of` claim to
/// declare — there is no response to execute against, which is the whole reason
/// it is a finding. `beastiary1/race_traits` is a different case. 107 of its 108
/// records demonstrably reach a live surface, and a claim that executes
/// against that surface is exactly what stops those 107 from silently falling
/// off — which is the defect this whole module exists for. Declaring no claim
/// would trade a caught regression for a tidier table.
///
/// So the claim is declared, it returns [`Reach::NotSurfaced`], the family
/// stays a written finding, and the exact shortfall is pinned here. The
/// property that matters is preserved in both directions:
///
/// * a 2nd B1 record that stops reaching changes this set and fails;
/// * fixing one of these fails too, until its key is deleted.
///
/// `advanced_race_guide/race_traits` used to be listed here for exactly this
/// reason, at 154 of 156. Its two stragglers — `Feral ~ Languages` and
/// `Scion of Humanity ~ Languages` — now arrive through
/// `race_resolver`'s reading of the `ABILITY:<cat>|AUTOMATIC|<key>` grant
/// shape, so the family is a plain claim and both entries are gone.
const UNREACHED_RECORD_FINDINGS: &[(&str, &str, &[&str])] = &[
    (
        "beastiary1",
        "race_traits",
        // Gated on `Duergar_ReplaceSLAEnlargePerson`, which nothing in any
        // ingested book sets; its only setter is in Monster Codex.
        &["Duergar ~ Spell-Like Ability ~ Invisibility"],
    ),
];

fn recorded_unreached(family: &Family) -> BTreeSet<String> {
    UNREACHED_RECORD_FINDINGS
        .iter()
        .find(|(book, kind, _)| *book == family.book && *kind == family.kind)
        .map(|(_, _, keys)| keys.iter().map(|key| (*key).to_owned()).collect())
        .unwrap_or_default()
}

fn recorded_bare(family: &Family) -> Option<BTreeSet<String>> {
    BARE_RECORD_FINDINGS
        .iter()
        .find(|(book, kind, _)| *book == family.book && *kind == family.kind)
        .map(|(_, _, keys)| keys.iter().map(|key| (*key).to_owned()).collect())
}

fn recorded_findings() -> BTreeSet<Family> {
    OPEN_FINDINGS
        .iter()
        .map(|(book, kind, _)| Family::new(book, kind))
        .collect()
}

fn finding_text(family: &Family) -> Option<&'static str> {
    OPEN_FINDINGS
        .iter()
        .find(|(book, kind, _)| *book == family.book && *kind == family.kind)
        .map(|(_, _, text)| *text)
}

/// The whole ingested inventory, from all three independent sources.
fn full_inventory() -> BTreeSet<Family> {
    let mut inventory = diagnostic_inventory();
    inventory.extend(scanned_inventory().0);
    inventory.extend(corpus_inventory().0);
    inventory
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// The inventory itself must be real. If either source silently returned
    /// nothing, every other test here would pass while checking no content at
    /// all — the exact "test that passes while asserting nothing" failure this
    /// gate was written in response to.
    #[test]
    fn the_inventory_is_populated_from_all_three_live_sources() {
        let from_diagnostic = diagnostic_inventory();
        assert!(
            from_diagnostic.len() >= 10,
            "the app's ingest diagnostic reported only {} populated families; it is the gate's \
             primary discovery source and cannot be near-empty",
            from_diagnostic.len()
        );

        let (from_scan, _) = scanned_inventory();
        assert!(
            from_scan.len() >= 10,
            "the rules_tables source scan found only {} record families; it reads \
             {} and cannot be near-empty",
            from_scan.len(),
            repo_root().join("src/rules_core/rules_tables").display()
        );

        let (from_corpus, unnamed) = corpus_inventory();
        assert!(unnamed.is_empty(), "unnamed corpus directories:\n  {}", unnamed.join("\n  "));
        assert!(
            from_corpus.len() >= 10,
            "the data/corpus scan found only {} record families; it reads {} and cannot be \
             near-empty",
            from_corpus.len(),
            repo_root().join("data/corpus").display()
        );

        // The sources genuinely differ. If any of them became a subset of the
        // others it would be adding nothing, and the inventory would rest on
        // fewer independent legs than it claims to.
        let scan_only: Vec<String> = from_scan
            .difference(&from_diagnostic)
            .map(Family::label)
            .collect();
        assert!(
            !scan_only.is_empty(),
            "the source scan added no family the diagnostic missed; verify it is still parsing \
             record slices"
        );

        let mut compiled: BTreeSet<Family> = from_diagnostic.clone();
        compiled.extend(from_scan.clone());
        let corpus_only: Vec<String> = from_corpus.difference(&compiled).map(Family::label).collect();
        assert!(
            !corpus_only.is_empty(),
            "the data/corpus scan added no family the other two missed; it is the only source \
             that can see corpus-backed content, and a version of it that adds nothing is a \
             version that would not have caught ARG's alternate racial traits"
        );
    }

    /// **The blind spot this gate had, named and pinned.**
    ///
    /// ARG's alternate racial traits are the Advanced Race Guide's headline
    /// content and were invisible to both original discovery sources at once,
    /// so all eleven reach tests passed while the gate asserted nothing about
    /// them. This proves the third source is what sees them — not by
    /// describing the gap, but by asking the other two sources directly and
    /// requiring them still not to know.
    #[test]
    fn args_alternate_racial_traits_are_visible_only_because_the_corpus_is_scanned() {
        let arg_traits = Family::new("advanced_race_guide", "race_traits");

        assert!(
            !diagnostic_inventory().contains(&arg_traits),
            "corpus_ingest_diagnostic now reports ARG's race traits; that is an improvement, but \
             this test's premise changed — update it rather than deleting it"
        );
        assert!(
            !scanned_inventory().0.contains(&arg_traits),
            "ARG's race traits became a compiled rules_tables slice; decisions.md §24 says they \
             are hand-modelled/corpus-read, so verify what changed"
        );
        assert!(
            corpus_inventory().0.contains(&arg_traits),
            "the data/corpus scan must see data/corpus/advanced_race_guide/race_trait/"
        );
        assert!(full_inventory().contains(&arg_traits), "and it must reach the gate's inventory");

        // And the family is genuinely asked about: a claim exists, it executes
        // against the real IPC builders, and it accounts for every record.
        //
        // This used to expect `Reach::NotSurfaced` with a pinned two-key
        // shortfall — `Feral ~ Languages` and `Scion of Humanity ~ Languages`,
        // the rows whose only gate is their parent alternate's
        // `ABILITY:<cat>|AUTOMATIC|<key>` token. `race_resolver` reads that
        // grant shape now, so all 156 reach and the expectation is `Surfaced`.
        let ingested = corpus_record_keys("advanced_race_guide", "race_trait");
        assert_eq!(ingested.len(), 156, "ARG's 156 ingested race-trait records, counted on disk");
        match reach_of(&arg_traits).expect("ARG race traits have a declared claim") {
            Reach::Surfaced { records, .. } => assert_eq!(records, 156),
            other => panic!("every ARG race-trait record must reach a player, got {other:?}"),
        }
    }

    /// The other half of the same blind spot: `pathfinder_unchained` hid
    /// behind accessor functions, so a `pub const`-only scanner reported an
    /// ingested book as an empty directory.
    #[test]
    fn pathfinder_unchaineds_tables_are_visible_to_the_source_scan_not_only_the_diagnostic() {
        let (from_scan, _) = scanned_inventory();
        for kind in ["feats", "equipment", "class_features"] {
            let family = Family::new("pathfinder_unchained", kind);
            assert!(
                from_scan.contains(&family),
                "the source scan must see pathfinder_unchained/{kind}; every PU table is behind a \
                 `pub fn ... -> &'static [..]` accessor, which is exactly what used to hide it"
            );
        }
        // PU's classes are an enum roster rather than a record slice, so the
        // scan does not see them and is not expected to — the diagnostic and
        // the corpus both do, which is why three sources rather than one.
        let classes = Family::new("pathfinder_unchained", "classes");
        assert!(diagnostic_inventory().contains(&classes));
        assert!(corpus_inventory().0.contains(&classes));
    }

    /// Every ingested record type must be one this gate has classified. A book
    /// that introduces a genuinely new kind of content (hexes, mysteries,
    /// traits) lands here first, before anyone can ship it unreachable.
    #[test]
    fn every_ingested_record_type_is_classified() {
        let (_, unknown) = scanned_inventory();
        assert!(
            unknown.is_empty(),
            "unclassified ingested record types:\n  {}",
            unknown.join("\n  ")
        );
    }

    /// The gate proper: every ingested family either reaches the player, or is
    /// a written finding. Nothing may be neither.
    #[test]
    fn every_ingested_family_is_accounted_for() {
        let recorded = recorded_findings();
        let mut undeclared = Vec::new();

        for family in full_inventory() {
            if reach_of(&family).is_some() || recorded.contains(&family) {
                continue;
            }
            undeclared.push(family.label());
        }

        assert!(
            undeclared.is_empty(),
            "ingested content with no declared consumer and no recorded finding: {}\n\n\
             Each needs either a reach claim in `reach_of` naming the command that carries its \
             records to a player, or an entry in OPEN_FINDINGS stating the gap and the remedy. \
             Ingesting a book without doing one of the two is the defect this gate exists to \
             stop.",
            undeclared.join(", ")
        );
    }

    /// Claims are executed, not trusted. A surface that stops carrying a
    /// book's records fails here even though its doc comment still says it
    /// does.
    ///
    /// A claim that comes back `NotSurfaced` is broken **unless every record it
    /// names is already pinned in [`UNREACHED_RECORD_FINDINGS`]** — one extra
    /// record failing to reach is a new defect and fails here, by key, with
    /// that key named.
    #[test]
    fn every_declared_claim_actually_carries_the_records() {
        let mut broken = Vec::new();
        let mut proven = 0usize;

        for family in full_inventory() {
            let Some(reach) = reach_of(&family) else {
                continue;
            };
            match reach {
                Reach::Surfaced { .. } => proven += 1,
                // Checked, by exact key, in
                // `bare_records_are_exactly_the_recorded_findings`.
                Reach::BareRecords { .. } => proven += 1,
                Reach::NotSurfaced { why, missing } => {
                    let recorded = recorded_unreached(&family);
                    let unrecorded: Vec<&String> = missing.difference(&recorded).collect();
                    if unrecorded.is_empty() && !recorded.is_empty() {
                        // Every shortfall is a written finding; the rest of the
                        // family is proven to reach, which is the regression
                        // this claim is here to catch.
                        proven += 1;
                    } else {
                        broken.push(format!("{}: {why} [unrecorded: {unrecorded:?}]", family.label()));
                    }
                }
            }
        }

        assert!(
            broken.is_empty(),
            "declared reach claims that do not hold against live IPC responses:\n  {}",
            broken.join("\n  ")
        );
        assert!(
            proven >= 10,
            "only {proven} claims were executed; the gate is barely checking anything"
        );
    }

    /// Records that reach no surface at all, pinned by exact key both ways: a
    /// new one fails, and a fixed one fails until its key is removed.
    ///
    /// The other half of
    /// [`every_declared_claim_actually_carries_the_records`], and the reason a
    /// partially-reaching family can carry an executed claim without the
    /// finding becoming a suppression.
    #[test]
    fn unreached_records_are_exactly_the_recorded_findings() {
        let mut families_with_a_pin = 0usize;
        for family in full_inventory() {
            let recorded = recorded_unreached(&family);
            let live = match reach_of(&family) {
                Some(Reach::NotSurfaced { missing, .. }) => missing,
                _ => BTreeSet::new(),
            };
            if !recorded.is_empty() {
                families_with_a_pin += 1;
            }

            let unrecorded: Vec<&String> = live.difference(&recorded).collect();
            assert!(
                unrecorded.is_empty(),
                "{}: {} ingested record(s) now reach no surface at all, with no recorded finding: \
                 {:?}",
                family.label(),
                unrecorded.len(),
                unrecorded
            );

            let fixed: Vec<&String> = recorded.difference(&live).collect();
            assert!(
                fixed.is_empty(),
                "{}: these records now reach a player — delete them from \
                 UNREACHED_RECORD_FINDINGS: {:?}",
                family.label(),
                fixed
            );
        }
        assert_eq!(
            families_with_a_pin,
            UNREACHED_RECORD_FINDINGS.len(),
            "every UNREACHED_RECORD_FINDINGS entry must name a family the inventory really has"
        );
    }

    /// The findings list is pinned from both sides, so it cannot rot into a
    /// suppression list: an unlisted gap fails, and a listed gap that got
    /// fixed also fails until the entry is removed.
    #[test]
    fn unsurfaced_families_are_exactly_the_recorded_findings() {
        let mut live_unsurfaced = BTreeSet::new();
        for family in full_inventory() {
            match reach_of(&family) {
                Some(reach) if reach.has_a_surface() => {}
                _ => {
                    live_unsurfaced.insert(family);
                }
            }
        }

        let recorded = recorded_findings();

        let newly_unsurfaced: Vec<String> = live_unsurfaced
            .difference(&recorded)
            .map(Family::label)
            .collect();
        assert!(
            newly_unsurfaced.is_empty(),
            "ingested content reaching no player surface, with no recorded finding: {}",
            newly_unsurfaced.join(", ")
        );

        let stale: Vec<String> = recorded
            .difference(&live_unsurfaced)
            .map(Family::label)
            .collect();
        assert!(
            stale.is_empty(),
            "these families now reach a surface — delete their OPEN_FINDINGS entries: {}",
            stale.join(", ")
        );

        // Every finding must say something. An entry with no remedy is a
        // suppression, which is what this list must never become.
        for family in &recorded {
            let text = finding_text(family).unwrap_or("");
            assert!(
                text.len() > 80,
                "the OPEN_FINDINGS entry for {} must state the gap and the remedy",
                family.label()
            );
        }
    }

    /// Records arriving at a real surface with nothing but their key are
    /// pinned by exact key, both ways: a new one fails, and a fixed one fails
    /// until its key is removed.
    #[test]
    fn bare_records_are_exactly_the_recorded_findings() {
        for family in full_inventory() {
            let recorded = recorded_bare(&family).unwrap_or_default();
            let live = match reach_of(&family) {
                Some(Reach::BareRecords { bare, .. }) => bare,
                _ => BTreeSet::new(),
            };

            let unrecorded: Vec<&String> = live.difference(&recorded).collect();
            assert!(
                unrecorded.is_empty(),
                "{}: {} record(s) now reach their surface carrying only a key, with no recorded \
                 finding — the player sees a name and empty columns: {:?}",
                family.label(),
                unrecorded.len(),
                unrecorded
            );

            let fixed: Vec<&String> = recorded.difference(&live).collect();
            assert!(
                fixed.is_empty(),
                "{}: these records now carry real fields — delete them from \
                 BARE_RECORD_FINDINGS: {:?}",
                family.label(),
                fixed
            );
        }
    }

    /// **The honesty rule, against the real live example.**
    ///
    /// `corpus_ingest_diagnostic` carries every book's record COUNT to the
    /// player, Bestiary 1's monsters included. If a count satisfied this gate,
    /// every family would trivially pass and all six historical defects would
    /// have gone undetected.
    ///
    /// This test used to say so by pinning `beastiary1/monsters` as an
    /// unreached finding *while* the diagnostic reported its count. Those 41
    /// records now reach a real screen, so that phrasing is spent — and
    /// rewriting it to pin some other family would only move the same argument
    /// somewhere less load-bearing. The rule itself is what matters and is what
    /// this now asserts directly: **no claim in this file may name the
    /// diagnostic as its surface**, and the family that most recently escaped
    /// the finding list did so by naming a command that returns records.
    #[test]
    fn a_count_does_not_satisfy_the_gate() {
        let monsters = Family::new("beastiary1", "monsters");

        let counted = build_corpus_ingest_diagnostic()
            .into_iter()
            .find(|book| book.book_id == "beastiary1")
            .and_then(|book| book.content_kind_counts.get("monsters").copied())
            .expect("the shipped diagnostic reports a Bestiary 1 monster count");
        assert!(
            counted > 0,
            "precondition: the app must actually be reporting a non-zero monster count"
        );

        // Not one claim, anywhere in the inventory, rests on the panel that
        // renders those counts.
        for family in full_inventory() {
            let surface = match reach_of(&family) {
                Some(Reach::Surfaced { surface, .. })
                | Some(Reach::BareRecords { surface, .. }) => surface,
                _ => continue,
            };
            assert!(
                !surface.contains("corpus_ingest_diagnostic")
                    && !surface.contains("ingest_diagnostic"),
                "{}'s claim names the ingest diagnostic as its surface — that renders a number, \
                 not the records, and would have passed on every one of the six historical \
                 defects",
                family.label()
            );
        }

        // And the monsters specifically: they are reached because a command
        // returns all 41 stat blocks, not because a panel counts them.
        match reach_of(&monsters).expect("Bestiary 1's monsters have a declared claim") {
            Reach::Surfaced { surface, records } => {
                assert_eq!(surface, "list_monster_catalog");
                assert_eq!(
                    records as u64,
                    u64::from(counted),
                    "the claim must account for every record the diagnostic counts"
                );
            }
            other => panic!("Bestiary 1's monsters reach the monster catalog, got {other:?}"),
        }
    }

    /// The monster claim, per record.
    ///
    /// The denominator is the record files on disk; the numerator is the live
    /// `list_monster_catalog` response. Neither is `MonsterId::ALL`, so a
    /// roster and a corpus that drifted apart fail here rather than agreeing
    /// with each other.
    #[test]
    fn bestiary_1_monsters_reach_the_monster_catalog_record_by_record() {
        let ingested = corpus_record_ids("beastiary", "monster");
        assert_eq!(
            ingested.len(),
            41,
            "Bestiary 1's 41 ingested monster records, counted on disk"
        );

        let served: BTreeSet<String> = crate::monster_catalog::build_monster_catalog()
            .entries
            .into_iter()
            .map(|entry| entry.key)
            .collect();
        assert_eq!(
            served, ingested,
            "every record on disk must be served, and nothing may be served that is not on disk"
        );

        match reach_of(&Family::new("beastiary1", "monsters")).expect("a claim is declared") {
            Reach::Surfaced { records, .. } => assert_eq!(records, 41),
            other => panic!("expected all 41 to reach, got {other:?}"),
        }
    }

    /// The class-feature claim, per record — the thing the deleted
    /// `pathfinder_unchained/class_features` finding said could not be written.
    ///
    /// The premise is checked rather than assumed: the corpus keys and the
    /// receipt ids really are different vocabularies, so this passing means the
    /// engine is carrying the key across, not that the two happened to match.
    #[test]
    fn pathfinder_unchaineds_class_features_are_claimed_per_corpus_record() {
        let family = Family::new("pathfinder_unchained", "class_features");
        let ingested = corpus_record_keys("pathfinder_unchained", "class_feature");
        assert_eq!(
            ingested.len(),
            64,
            "PU's 64 ingested class_feature records, counted on disk"
        );
        assert!(
            ingested.contains("Unchained Rogue ~ Sneak Attack"),
            "the finding's own worked example must still be one of the records"
        );

        match reach_of(&family).expect("PU class features have a declared claim") {
            Reach::Surfaced { surface, records } => {
                assert_eq!(surface, "load_saved_character -> explanations (class_feature.pu.*)");
                assert_eq!(records, 64);
            }
            other => panic!("expected all 64 to reach, got {other:?}"),
        }
    }

    /// **The payload rule, proven against a failing case.**
    ///
    /// A record whose identity crosses the boundary and nothing else is the
    /// Feats-tab defect verbatim. `assess` must reject it. Driving the real
    /// decision function with synthetic inputs is the only way to have
    /// evidence of that without breaking a shipped surface.
    #[test]
    fn identity_without_payload_is_not_reach() {
        let ingested: BTreeSet<String> =
            ["Deflect Arrows", "Power Attack"].iter().map(|k| k.to_string()).collect();

        // Everything arrived; nothing carried a renderable field.
        match assess("list_feat_catalog", &ingested, &BTreeSet::new(), &ingested) {
            Reach::BareRecords { bare, .. } => assert_eq!(
                bare, ingested,
                "the verdict must name every record the player would see bare"
            ),
            other => panic!("identity-only records must not read as reach, got {other:?}"),
        }

        // One record carries payload, the other does not. Partial is still not
        // reach — "most of the book renders" is how a gap survives a review.
        let one_bare: BTreeSet<String> = ["Power Attack"].iter().map(|k| k.to_string()).collect();
        let one_full: BTreeSet<String> =
            ["Deflect Arrows"].iter().map(|k| k.to_string()).collect();
        let partial = assess("list_feat_catalog", &ingested, &one_full, &one_bare);
        assert!(
            !partial.is_surfaced(),
            "a single identity-only record must stop the family reading as fully surfaced"
        );
        assert_eq!(
            partial,
            Reach::BareRecords {
                surface: "list_feat_catalog",
                records: 2,
                bare: one_bare,
            },
            "and it must name exactly which record, not just report a count"
        );

        // And the control: the same helper does report reach when every record
        // carries payload, so the rule above is not simply always-fail.
        assert!(
            assess("list_feat_catalog", &ingested, &ingested, &BTreeSet::new()).is_surfaced()
        );
    }

    /// The other half of honesty: a record that never arrives at all must fail
    /// too, and the reason must say how many and name some.
    #[test]
    fn records_missing_from_the_response_are_not_reach() {
        let ingested: BTreeSet<String> = ["Acid Splash", "Bless", "Cure Light Wounds"]
            .iter()
            .map(|k| k.to_string())
            .collect();
        let served: BTreeSet<String> = ["Bless"].iter().map(|k| k.to_string()).collect();

        match assess("list_spell_catalog", &ingested, &served, &BTreeSet::new()) {
            Reach::NotSurfaced { why, .. } => {
                assert!(why.contains("2 of 3"), "the reason must quantify: {why}");
                assert!(
                    why.contains("Acid Splash") || why.contains("Cure Light Wounds"),
                    "the reason must name records so the gap is actionable: {why}"
                );
            }
            other => panic!("a partially-served family must not read as reach, got {other:?}"),
        }
    }

    /// The scanner must recognise a real, generated record slice and ignore
    /// the index tables and enum rosters that share its syntax — otherwise the
    /// second discovery source is either blind or full of noise.
    #[test]
    fn the_record_slice_scanner_reads_real_declarations() {
        assert_eq!(
            slice_element_type("pub const SPELL_LIST: &[SpellListEntry] = &["),
            Some("SpellListEntry")
        );
        assert_eq!(
            slice_element_type("pub const WEAPON_TABLE: &[WeaponTableEntry] = &["),
            Some("WeaponTableEntry")
        );
        // Per-class index tables over records that live elsewhere.
        assert_eq!(
            slice_element_type("pub const WIZARD_SPELL_LIST: &[(&str, u8)] = &["),
            None
        );
        assert_eq!(
            slice_element_type("pub const FINESSEABLE_WEAPON_KEYS: &[&str] = &["),
            None
        );
        // An enum roster inside an `impl` block is indented, never a record
        // slice.
        assert_eq!(
            slice_element_type("    pub const ALL: &'static [ClassId] = &["),
            None
        );

        // The accessor shape every `pathfinder_unchained` table uses, and the
        // reason the book was invisible to this scanner.
        assert_eq!(
            slice_element_type("pub fn equipment_tables() -> &'static [EquipmentTableEntry] {"),
            Some("EquipmentTableEntry")
        );
        assert_eq!(
            slice_element_type("pub fn features() -> &'static [UnchainedMonkFeature] {"),
            Some("UnchainedMonkFeature")
        );
        // A primitive index accessor is not a record family, same as its
        // `pub const` counterpart.
        assert_eq!(
            slice_element_type("pub fn class_skills() -> &'static [&'static str] {"),
            None
        );
        // An indented accessor is inside an `impl`, and a borrowed-not-static
        // return is a helper over data that lives elsewhere.
        assert_eq!(
            slice_element_type("    pub fn features() -> &'static [UnchainedMonkFeature] {"),
            None
        );
        assert_eq!(slice_element_type("pub fn digest(&self) -> &[u8] {"), None);
    }

    /// A spot check that the gate is reading live data and not a constant: the
    /// feat claim must prove the actual ingested count, and that count must
    /// match the engine's own table.
    #[test]
    fn a_claim_reports_the_live_record_count() {
        let ingested = all_feat_tables()
            .iter()
            .filter(|table| table.rule_set == RuleSetId::Apg)
            .map(|table| table.entries.len())
            .sum::<usize>();

        match feats_reach(RuleSetId::Apg, "Apg") {
            Reach::Surfaced { records, surface } => {
                assert_eq!(surface, "list_feat_catalog");
                assert_eq!(
                    records, ingested,
                    "the claim must count the live table, not a remembered number"
                );
                assert!(records > 100, "APG's feat table is not this small");
            }
            other => panic!("APG feats reach the feat catalog today, got {other:?}"),
        }
    }
}

