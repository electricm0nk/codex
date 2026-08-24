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
    acg, adventurers_guide as ag, advanced_race_guide as arg, apg, beastiary1, crb,
    horror_adventures as ha, inner_sea_faiths as isf, inner_sea_gods as isg,
    inner_sea_magic as ism, inner_sea_temples as istem, occult_adventures as oa,
    pathfinder_unchained as pu, ultimate_combat as uc, ultimate_equipment as ue,
    ultimate_intrigue as ui, ultimate_magic as um, ultimate_psionics as upsi,
    ultimate_wilderness as uw, RuleSetId,
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
    // Ultimate Intrigue's own record type (SD28-E24) -- reuses the shared
    // `crb::feats::FeatCategory` enum but declares its own struct because
    // it also carries `pretext`/`benefit` as separate fields, the same
    // reason UCA's `StoryFeatEntry` above declares its own. Same family
    // (`"feats"`) as every other book's feat table.
    ("UiFeatEntry", "feats"),
    // Ultimate Wilderness's own record type (SD28-E26) -- own category
    // enum (Animal/Mount have no shared-enum equivalent), same reason
    // UCA's/UI's own types exist. Same family ("feats") as every other
    // book's feat table.
    ("UwFeatEntry", "feats"),
    // Ultimate Combat's own record type (SD28-E27) -- own category enum
    // (Style/Grit/Panache/Critical/CalledShot have no shared-enum
    // equivalent, or are kept deliberately distinct). Same family
    // ("feats") as every other book's feat table.
    ("UcFeatEntry", "feats"),
    // Ultimate Magic's own record type (SD28-E28) -- own category enum
    // (Critical/Masterpiece/Discovery have no shared-enum equivalent).
    // Same family ("feats") as every other book's feat table.
    ("UmFeatEntry", "feats"),
    // Ultimate Psionics' own record type (SD28-E29) -- own category
    // enum (Psionic/Metapsionic have no shared-enum equivalent). Same
    // family ("feats") as every other book's feat table.
    ("UpsiFeatEntry", "feats"),
    // SD-29 Epic 5: Bonus Bestiary's merged monster chassis. Two record
    // types, two families -- deliberately NOT one, per
    // `corpus-work-channels.md` §9.2: a monster ability is to a monster what a
    // race trait is to a race, and folding the abilities into `monsters` would
    // make the chassis count silently absorb the feature count.
    ("MonsterStatBlock", "monsters"),
    ("MonsterAbilityRecord", "monster_abilities"),
    // SD-29 Epic 7: the companion chassis. Two record types, ONE family --
    // deliberately unlike the monster pair above, and for the corpus's own
    // reason: `v06_work_inventory::file_kind` types a book's
    // `*_races_companion.lst` creature rows and its `*_abilities_companion.lst`
    // ability rows both as `Kind::Companion`, so there is one denominator on
    // disk (`data/corpus/<book>/companion/`) and splitting the claim would
    // judge two populations against one count.
    ("CompanionRecord", "companions"),
    ("CompanionAbilityRecord", "companions"),
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
    // SD28-C4.8/§60/§63: the tier-1 archetype-swap catalog, 406 records (403 + 3 Slayer archetypes, SD31-E4-F1-001)
    // across 7 books (acg, advanced_race_guide, apg, ultimate_combat,
    // ultimate_magic, ultimate_psionics, ultimate_wilderness). Its own
    // record family, not a facet of `class_feature` -- an archetype is
    // itself a selectable thing (`ARCHETYPE_CHOICE_ID`), not provenance
    // attached to another record.
    ("ArchetypeSwapEntry", "archetypes"),
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
    let dir = first.as_os_str().to_string_lossy().into_owned();
    Some(
        RULES_TABLES_BOOK_IDS
            .iter()
            .find(|(module, _)| *module == dir)
            .map(|(_, book_id)| (*book_id).to_owned())
            .unwrap_or(dir),
    )
}

/// `rules_tables` module directory -> the `book_id` this gate joins on, for the
/// modules whose directory name is not that id.
///
/// One entry, and it exists because one book is served by two modules.
/// `rules_tables::bestiary` holds Bestiary 1's chassis complement beside
/// `rules_tables::beastiary1`'s hand-modelled 46 (`decisions.md §58.3`), and
/// both write to `data/corpus/beastiary/`, which [`CORPUS_BOOK_IDS`] already
/// names `beastiary1`. Without this the source scan would invent a
/// `bestiary/monsters` family that no corpus directory and no diagnostic row
/// backs, and demand a claim for a book that already has one under its real
/// name — a phantom, which is the opposite of what this gate is for.
///
/// It is a rename, not an exemption: the module's records still have to be
/// claimed, under `beastiary1`.
const RULES_TABLES_BOOK_IDS: &[(&str, &str)] = &[("bestiary", "beastiary1")];

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
    // `SD31-E6-F10-002`: `docs/work-inventory.json`'s own book field for
    // Bestiary 1 equipment (derived from the corpus `.lst`'s own
    // `SOURCELONG:Bestiary`, `decisions.md §9`) is the correctly-spelled
    // `"bestiary"`, not the pre-existing `"beastiary"` misspelling above --
    // a SECOND, previously-empty corpus directory that this cycle's
    // `cache_gen::equipment_gap` `"B1"` routing populated for the first
    // time (`equipment_gap.rs::book_routing`). Same book id as the
    // misspelled directory (`equipment_gap_tables`'s rows already carry
    // `book: "B1"`, which `equipment_reach`'s existing `("beastiary1",
    // "equipment")` arm already unions in) so the two directories share
    // one reach claim rather than needing a second, duplicate arm.
    ("bestiary", "beastiary1"),
    ("advanced_race_guide", "advanced_race_guide"),
    ("pathfinder_unchained", "pathfinder_unchained"),
    // SD-29 Epic 5 pilot. Directory and book id are spelled the same, like
    // ARG's and PU's, so no rename is hidden here.
    ("bonus_bestiary", "bonus_bestiary"),
    // SD-29 Epic 6 pilot (race-trait lane). Spelled the same in both columns
    // for the same reason.
    ("monster_codex", "monster_codex"),
    // SD-29 Epic 6 round 2 (race-trait lane, extend). Same again.
    ("inner_sea_races", "inner_sea_races"),
    // SD-29 Epic 6 round 3 (race-trait lane, extend). Same again.
    ("horror_adventures", "horror_adventures"),
    // SD-29 Epic 5 extend, round 2 (monster lane). Same again -- the corpus
    // directory and the book id are the same string for both volumes.
    ("book_of_the_damned_volume_1", "book_of_the_damned_volume_1"),
    ("book_of_the_damned_volume_2", "book_of_the_damned_volume_2"),
    // SD-29 Epic 5 extend, round 3 (monster lane). Same again.
    ("inner_sea_world_guide", "inner_sea_world_guide"),
    // SD-29 Epic 7 (companion lane). Directory and book id are spelled the same
    // for both, like every SD-29 book before them.
    ("inner_sea_combat", "inner_sea_combat"),
    ("inner_sea_intrigue", "inner_sea_intrigue"),
    // SD-29 Epic 7 round 2 (companion lane, extend). Same again. Note these
    // three books' `companion` family is the ONLY one they contribute today:
    // B5 and B6 carry no monsters at all, and B2's 782 monster/monster_ability
    // units belong to the monster lane.
    ("bestiary_5", "bestiary_5"),
    ("bestiary_6", "bestiary_6"),
    ("bestiary_2", "bestiary_2"),
    // SD-29 Epic 5 extend, round 5 (monster lane). Same again -- and the first
    // bestiary in this list whose ONLY families are `monster`/`monster_ability`:
    // B5 and B6 carry no monsters, and B2 carries both lanes' families.
    ("bestiary_3", "bestiary_3"),
    // SD-29 Epic 5 extend, round 6 (monster lane). Bestiary 4 carries an 86-unit
    // `race_trait` family too, but this rule set compiles the two monster
    // families and those units stay `not-ingested`, which is their honest state.
    ("bestiary_4", "bestiary_4"),
    // SD-29 Epic 5 extend, round 7 (monster lane). Same again. Inner Sea
    // Bestiary carries a 4-unit `race_trait` family too; this rule set compiles
    // the two monster families and those units stay `not-ingested`.
    ("inner_sea_bestiary", "inner_sea_bestiary"),
    // SD-29 Epic 5 extend, round 9 (monster lane). Inner Sea Gods. The book
    // carries a large `race_trait` family too; this rule set compiles the two
    // monster families and those units stay `not-ingested`, which is their
    // honest state.
    ("inner_sea_gods", "inner_sea_gods"),
    // SD-29 Epic 7 extend, round 6 (companion lane). Ultimate Wilderness. The
    // book's `data/corpus/` directory is written by TWO lanes -- SD-28 Epic 26
    // put its 136 feats there -- and this entry names the directory, not the
    // family, so it is written once and covers both.
    ("ultimate_wilderness", "ultimate_wilderness"),
    // SD-29 Epic 5 extend, round 10 (monster lane). Ultimate Psionics -- the
    // first non-Paizo book in this table. Its `data/corpus/` directory is
    // written by TWO lanes (SD-28 E29 put its feats and equipment there), and
    // this entry names the directory rather than a family, so it covers both.
    ("ultimate_psionics", "ultimate_psionics"),
    // SD-29 Epic 7 round 9 (companion lane, final pass). Ultimate Magic. The
    // ONLY entry of the round: `advanced_race_guide`, `advanced_players_guide`
    // and `book_of_the_damned_volume_1` are already above, put there by earlier
    // lanes, and this table names the DIRECTORY rather than a family so no
    // second row is wanted for their companions.
    //
    // Ultimate Magic is different because it had no `data/corpus/ultimate_magic/`
    // directory at all before this round — SD-28 Epic 28 compiled its 144 feats
    // into `rules_tables` without ever writing corpus JSON for them — so the
    // companion family is the first content this book serves off disk.
    ("ultimate_magic", "ultimate_magic"),
    // SD-31 `epic-6-ingest-lanes` F5/F6 (`SD31-E6-F5-001`). Ultimate
    // Equipment had no `data/corpus/ultimate_equipment/` directory at all
    // before this cycle (`OPEN-ISSUES.md` row 12) despite already driving
    // the desktop equipment catalog via `equipment_resolver::
    // equipment_catalog_rows()`'s compiled table -- `cache_gen::
    // ultimate_equipment` (`gen_cache_ultimate_equipment`) writes the first
    // real corpus JSON for the book. `corpus_ingest_diagnostic.rs` already
    // names this book `"ultimate_equipment"` (`ultimate_equipment_counts`);
    // this entry names the directory the same way every other book here
    // does.
    ("ultimate_equipment", "ultimate_equipment"),
    // SD-31 `epic-5-chassis-sweep` F1 (`SD31-E5-F1-001`). `cache_gen::
    // class_feature` wrote the first-ever `data/corpus/<book>/` directory
    // at all for these six books (no LICENSE.json, no other kind's corpus
    // JSON, nothing -- `corpus_ingest_diagnostic.rs` has no entry for any
    // of them either, so no diagnostic book id exists to reuse; same-name
    // pairing, matching every other book above with no diagnostic entry).
    // `ultimate_combat`/`ultimate_intrigue` already drive real `reach_of`
    // claims (feats/equipment/archetypes) sourced directly from their
    // compiled `rules_tables` modules, never from `data/corpus/`, which is
    // why they could be claimed without ever needing a row here until now.
    // Both books' `data/corpus/` directories are now populated by TWO
    // independent SD-31 wave-4 lanes: `cache_gen::class_feature`'s
    // `class_feature/` subdirectory and `cache_gen::equipment_gap`'s
    // `equipment/` subdirectory (`SD31-E6-F5-002`) -- one registration
    // covers both, the mapping is directory-to-book-id, not kind-scoped.
    ("occult_adventures", "occult_adventures"),
    ("adventurers_guide", "adventurers_guide"),
    ("ultimate_combat", "ultimate_combat"),
    ("ultimate_intrigue", "ultimate_intrigue"),
    ("inner_sea_magic", "inner_sea_magic"),
    ("inner_sea_taverns", "inner_sea_taverns"),
    // `SD31-E6-F10-004`: `gen_cache_equipment_gap` wrote the first-ever
    // `data/corpus/mythic_adventures/` directory at all (no LICENSE.json,
    // no other kind's corpus JSON, nothing existed before this cycle) --
    // same-name pairing, matching every other book above with no
    // diagnostic entry.
    ("mythic_adventures", "mythic_adventures"),
    // Row-19 desktop reach/catalog reds (SD-32, 2026-08-24): four books the
    // T12 census/class-feature lanes wrote `data/corpus/` directories for
    // without ever registering here. Same-name pairing, matching every
    // other book above; `corpus_ingest_diagnostic.rs` already spells all
    // four this way (`ultimate_campaign`/`inner_sea_faiths`/
    // `inner_sea_temples` each appear as literal book ids there;
    // `core_essentials` has no diagnostic entry, so it gets the no-entry
    // same-name pairing the comments above use for that case).
    ("ultimate_campaign", "ultimate_campaign"),
    ("inner_sea_faiths", "inner_sea_faiths"),
    ("inner_sea_temples", "inner_sea_temples"),
    ("core_essentials", "core_essentials"),
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
    // SD-29 Epic 5 pilot: the features half of the merged monster chassis
    // (`corpus-work-channels.md` §9.2). Bonus Bestiary is the first book to
    // ingest it, so this is the entry's first appearance.
    ("monster_ability", "monster_abilities"),
    ("race", "races"),
    ("race_trait", "race_traits"),
    // SD-29 Epic 7 (companion lane). One kind for both of the corpus's
    // structural shapes -- see the `CompanionRecord` entry in
    // `RECORD_TYPE_FAMILIES` for why.
    ("companion", "companions"),
    ("spell", "spells"),
    // Row-19 desktop reach/catalog reds (SD-32, 2026-08-24). Twelve more
    // content-kind directories the census/class-feature lanes populated
    // with no classifier entry at all -- `corpus_inventory` panicked
    // before any reach test could even collect its family list.
    // Verified by direct inspection (not assumed): each holds real,
    // named, described records distinct from the primary kind sharing
    // its book (e.g. `beastiary/race_generic/hydra_pyrohydra.json` is a
    // genuine Pyrohydra variant, not a duplicate of `beastiary/race/`'s
    // rows; `ultimate_psionics/ability/deafening_static.json` carries the
    // real key `Dissonance ~ Deafening Static` with its own mechanical
    // description, not an internal grant). None of the twelve has a
    // `reach_of` arm yet -- each surfaces in
    // `every_ingested_family_is_accounted_for` per book below, which is
    // this bundle's forward-scope register entry for the new-catalog
    // work each one needs (`decisions.md §16`: naming a kind is not the
    // same disposition as closing its reach).
    ("ability", "abilities"),
    ("class_generic", "class_variants"),
    ("deity", "deities"),
    ("domain", "domains"),
    ("feat_generic", "generic_feats"),
    ("language", "languages"),
    ("monster_generic", "monster_variants"),
    ("power", "powers"),
    ("race_generic", "race_variants"),
    ("skill", "skills"),
    ("template", "templates"),
    ("trait_generic", "named_traits"),
];

/// Directories under a corpus book that hold no player-facing records. Listed
/// with the reason, for the same reason [`SUPPORTING_RECORD_TYPES`] is: an
/// unexplained exclusion is how a gate quietly stops covering something.
const NON_CONTENT_CORPUS_DIRS: &[(&str, &str)] = &[
    (
        "_parity",
        "an ingest-verification artifact (a book's parity report against its \
         upstream LST), not game content: nothing in it describes a rule, and a \
         player is not meant to read it",
    ),
    (
        // Row-19 desktop reach/catalog reds (SD-32, 2026-08-24). Verified
        // across every book that writes this directory (core_rulebook,
        // beastiary, bestiary_2/3/4/5/6, and every other monster-bearing
        // book): every sampled record carries `VISIBLE:NO` and a
        // `CATEGORY:Special Ability` grant whose sole job is composing a
        // creature TYPE's universal traits (`Chaotic Traits`, `Fire
        // Traits`, `Aquatic Traits`) into the monster chassis compute --
        // e.g. `core_rulebook/race_trait_generic/chaotic_traits.json`
        // grants `Aligned (Chaotic)` with `VISIBLE:NO`, the PCGen idiom
        // for "never render this row directly." A player never picks or
        // reads one of these; they reach the sheet already folded into
        // the monster's stat block by the chassis compute this record
        // feeds. Distinct from `ability`/`trait_generic` above, both of
        // which carry real named, player-legible records under the same
        // `_generic` naming convention -- this directory's contents were
        // checked and are not.
        "race_trait_generic",
        "hidden (VISIBLE:NO) universal-monster-type trait grants that feed \
         the monster chassis compute directly; not a player-legible record \
         and never rendered on its own",
    ),
];

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

/// Every `data.key` under one exact directory, rather than under a whole
/// `<book>/<kind>/` tree. Used where a claim needs a SUBSET of a book's kind to
/// fail independently -- the Aasimar/Tiefling heritage records inside ARG's
/// much larger `race_trait/` tree are the case that needed it.
fn corpus_record_keys_in(dir: &Path) -> BTreeSet<String> {
    json_files_under(dir)
        .into_iter()
        .filter_map(|path| {
            let text = fs::read_to_string(&path).ok()?;
            let value: serde_json::Value = serde_json::from_str(&text).ok()?;
            value.get("data")?.get("key")?.as_str().map(str::to_owned)
        })
        .collect()
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
        // SD28-E24: Ultimate Intrigue's 104 feats joined
        // `feats_all::all_feat_tables()` under the `Ui` wire source, the
        // first record family of this book's from-scratch ingest. Every
        // record carries a non-empty `category` (General/Combat/Metamagic/
        // Teamwork) so `feats_reach`'s own check is satisfied for all 104.
        ("ultimate_intrigue", "feats") => Some(feats_reach(RuleSetId::Ui, "Ui")),
        // SD28-E26 slice 1: UW joined `feats_all::all_feat_tables()` under
        // the `Uw` wire source. Every record carries a non-empty category
        // (General/Combat/ItemCreation/Metamagic/Teamwork/Animal/Mount),
        // so `feats_reach`'s own check is satisfied for all 136.
        ("ultimate_wilderness", "feats") => Some(feats_reach(RuleSetId::Uw, "Uw")),
        // SD28-E27 slice 1: UC joined `feats_all::all_feat_tables()` under
        // the `Uc` wire source. Every record carries a non-empty category,
        // so `feats_reach`'s own check is satisfied for all 263.
        ("ultimate_combat", "feats") => Some(feats_reach(RuleSetId::Uc, "Uc")),
        // SD28-E28 slice 1: UM joined `feats_all::all_feat_tables()` under
        // the `Um` wire source. Every record carries a non-empty category,
        // so `feats_reach`'s own check is satisfied for all 144.
        ("ultimate_magic", "feats") => Some(feats_reach(RuleSetId::Um, "Um")),
        // SD28-E29 slice 1: UPsi joined `feats_all::all_feat_tables()`
        // under the `Upsi` wire source. Every record carries a
        // non-empty category, so `feats_reach`'s own check is
        // satisfied for all 221.
        ("ultimate_psionics", "feats") => Some(feats_reach(RuleSetId::Upsi, "Upsi")),
        // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
        // onboarding-precondition`, AT-32-G0-003): Inner Sea Taverns'
        // first reach claim of any kind, and its first compiled
        // `RuleSetId`. Unlike the other three SD-32 Gate 0 books, this
        // one has no `*_spells.lst` at all -- its first family is `feat`
        // via `gen_feat_gap_tables`'s `BOOK_INPUTS` (`istav_feats.lst`'s
        // 9 base declarations, same mechanism `RuleSetId::Mythic` uses),
        // joined into `feats_all::all_feat_tables()` and served by
        // `list_feat_catalog` under `source: "InnerSeaTaverns"` (the
        // `RuleSetId`'s own `{:?}` name -- `feat_catalog::
        // build_feat_catalog_for` derives `source` generically for every
        // book, so this claim is a real reach test rather than new wiring
        // of its own). Every one of the 9 records carries a non-empty
        // `category`, so `feats_reach`'s own check is satisfied for all 9.
        ("inner_sea_taverns", "feats") => {
            Some(feats_reach(RuleSetId::InnerSeaTaverns, "InnerSeaTaverns"))
        }

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
        // SD28-E24 slice 2: UI joined `build_spell_catalog` the same way ARG
        // did -- same command, same unfiltered "All books" render path.
        ("ultimate_intrigue", "spells") => Some(spells_reach(
            "UI",
            ui::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD31-E6-F2-002: UM joined `spell_resolver::spell_catalog_rows()` as
        // the catalog's 6th book, the same `build_spell_catalog`/"All books"
        // render path ARG and UI use. 10 of its 269 records carry neither a
        // `CLASSES:`/`DOMAINS:` level (`Restore Eidolon` and siblings), but
        // every one still carries a real `SCHOOL:` and/or `DESC:`, so
        // `has_payload` is satisfied regardless -- not a bare-record risk.
        // The genuine risk was the 15 `Masterpiece` bard-performance records,
        // which carry NEITHER a recognized `SCHOOL:` (`"Masterpiece"`, this
        // engine's 9-school enum does not model it) NOR a `DESC:` token --
        // only a real `level`, from their own `CLASSES:Bard=N` token. Caught
        // by this gate's own `bare_records_are_exactly_the_recorded_findings`
        // check the FIRST time this arm was added: a level-parsing bug in
        // `ingest_ultimate_magic_spells.rs` had silently dropped that level
        // too, making all 15 genuinely bare (fixed; see `OPEN-ISSUES.md` row
        // 47). With the level restored, `has_payload` is satisfied by
        // `level` alone for all 15, and this gate passes with zero
        // `OPEN_FINDINGS` entries needed for this family.
        ("ultimate_magic", "spells") => Some(spells_reach(
            "UM",
            um::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD31-E6-F2-003: OA joins `spell_resolver::spell_catalog_rows()` as
        // the catalog's 7th book, the same `build_spell_catalog`/"All books"
        // render path UM/UI/ARG use -- this book's FIRST reach claim of any
        // kind. One record (`Talismanic Implement`) carries no `CLASSES:`
        // token so no `level`, and two (`Repulsion` excluded as a cross-book
        // collision; `Share Language (Communal)`) carry neither `SCHOOL:`
        // nor `DESC:` of their own -- every shipped record still carries a
        // real `key` plus at least one of `school`/`level`/`description`,
        // so `has_payload` is satisfied for all 144, proven by this gate's
        // own `bare_records_are_exactly_the_recorded_findings` check below
        // rather than assumed.
        ("occult_adventures", "spells") => Some(spells_reach(
            "OA",
            oa::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD31-E6-F2-004: UC joins `spell_resolver::spell_catalog_rows()` as
        // the catalog's 8th book, the same `build_spell_catalog`/"All
        // books" render path UM/OA use. 3 records (`Life Conduit` and its
        // two named variants) carry no `CLASSES:` token so no `level`, but
        // every shipped record still carries a real `key` plus at least
        // one of `school`/`level`/`description`, so `has_payload` is
        // satisfied for all 146, proven by this gate's own
        // `bare_records_are_exactly_the_recorded_findings` check below
        // rather than assumed.
        ("ultimate_combat", "spells") => Some(spells_reach(
            "UC",
            uc::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD31-E6-F10-001: ISG joins `spell_resolver::spell_catalog_rows()`
        // as the catalog's 9th book, the same `build_spell_catalog`/"All
        // books" render path UM/OA/UC use -- proven live, not merely
        // asserted, by a DoD-8 on-screen capture of "Blade Snare" rendering
        // its real corpus prose under the "ISG" book badge
        // (`docs/release/SD-31-corpus-closure-grind/artifacts/
        // SD31-E6-F10-001/item8/spell-blade-snare.png`). 31 of its 92
        // records carry no `CLASSES:`/`DOMAINS:` level (mostly deity-boon
        // variant spells whose base entry lives elsewhere), but every
        // shipped record still carries a real `key` plus at least one of
        // `school`/`level`/`description`, so `has_payload` is satisfied
        // for all 92, proven by this gate's own
        // `bare_records_are_exactly_the_recorded_findings` check below
        // rather than assumed.
        ("inner_sea_gods", "spells") => Some(spells_reach(
            "ISG",
            isg::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD-31 wave-19 (`ultimate_wilderness` lane): UW joins
        // `spell_resolver::spell_catalog_rows()` as the catalog's 10th
        // book, the same `build_spell_catalog`/"All books" render path
        // UM/OA/UC/ISG use -- this book's first `spell`-kind reach claim.
        // Every one of the 61 base declarations carries a real `SCHOOL:`,
        // `CLASSES:`/`DOMAINS:` level and `DESC:`, so `has_payload` is
        // satisfied for all 61 (`src/bin/ingest_ultimate_wilderness_spells.rs`).
        ("ultimate_wilderness", "spells") => Some(spells_reach(
            "UW",
            uw::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD-31 wave-29 (`lane5-book-onboard` lane): Adventurer's Guide
        // joins `spell_resolver::spell_catalog_rows()` as the catalog's
        // 12th book, the same `build_spell_catalog`/"All books" render
        // path UM/OA/UC/ISG/UW use -- this book's FIRST reach claim of any
        // kind, and its first compiled `RuleSetId`. Every base declaration
        // carries a real `SCHOOL:`, `CLASSES:` level and/or `DESC:`, so
        // `has_payload` is satisfied, proven by this gate's own
        // `bare_records_are_exactly_the_recorded_findings` check below
        // rather than assumed (`src/bin/ingest_adventurers_guide_spells.rs`).
        ("adventurers_guide", "spells") => Some(spells_reach(
            "AG",
            ag::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
        // onboarding-precondition`, AT-32-G0-003): Inner Sea Faiths joins
        // `spell_resolver::spell_catalog_rows()` as the catalog's 13th
        // book, the same `build_spell_catalog`/"All books" render path
        // AG/UW/ISG use -- this book's FIRST reach claim of any kind, and
        // its first compiled `RuleSetId`. All 3 base declarations carry no
        // `CLASSES:`/`DOMAINS:` level (a real corpus gap -- this book's
        // `isf_spells.lst` names no class list at all), but every shipped
        // record still carries a real `SCHOOL:` and/or `DESC:`, so
        // `has_payload` is satisfied (`src/bin/
        // ingest_inner_sea_setting_spells.rs`).
        ("inner_sea_faiths", "spells") => Some(spells_reach(
            "ISF",
            isf::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
        // onboarding-precondition`, AT-32-G0-003): Inner Sea Magic joins
        // `spell_resolver::spell_catalog_rows()` as the catalog's 14th
        // book, the same "All books" render path -- this book's FIRST
        // reach claim of any kind and its first compiled `RuleSetId`
        // (its 218 `class_feature` units were already ingested corpus-wide
        // but were unreachable through the book-level gate before this
        // variant existed -- see `RuleSetId::InnerSeaMagic`'s own doc
        // comment). Every shipped record carries a real `SCHOOL:`,
        // `CLASSES:` level and/or `DESC:`, so `has_payload` is satisfied
        // (`src/bin/ingest_inner_sea_setting_spells.rs`).
        ("inner_sea_magic", "spells") => Some(spells_reach(
            "ISM",
            ism::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
        // onboarding-precondition`, AT-32-G0-003): Inner Sea Temples
        // joins `spell_resolver::spell_catalog_rows()` as the catalog's
        // 15th book, the same "All books" render path -- this book's
        // FIRST reach claim of any kind, and its first compiled
        // `RuleSetId`. Every one of the 21 base declarations carries a
        // real `SCHOOL:`, `CLASSES:` level and `DESC:`, so `has_payload`
        // is satisfied for all 21 (`src/bin/
        // ingest_inner_sea_setting_spells.rs`).
        ("inner_sea_temples", "spells") => Some(spells_reach(
            "ISTEM",
            istem::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD-32 card 11 (T9 onboarding, `decisions.md §19` sign-off):
        // Horror Adventures joins `spell_resolver::spell_catalog_rows()`
        // as the catalog's 16th book, this book's SECOND reach claim
        // (`companion`/`monster`/`monster_ability` already reach). 70 of
        // its 72 base declarations carry a real `SCHOOL:`, `CLASSES:`
        // level and `DESC:`, so `has_payload` is satisfied for those 70
        // (`src/bin/ingest_spells.rs`). The other 2 ("Green Caress",
        // "Verminous Transformation") are verbatim reprints of spells
        // Ultimate Wilderness already ships (earlier in the chain) -- the
        // resolver's cross-book dedup keeps only UW's copy, so they never
        // carry `book=="HA"` in the served catalog (the exact
        // `bestiary_6`/`ultimate_wilderness` shape above, at 2-of-72
        // instead of 2-of-2 -- see the matching `OPEN_FINDINGS` entry
        // below rather than claiming these 2 here).
        ("horror_adventures", "spells") => Some(spells_reach(
            "HA",
            ha::spell_list::SPELL_LIST
                .iter()
                .map(|entry| entry.key.to_owned())
                .filter(|k| k != "Green Caress" && k != "Verminous Transformation")
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
        // SD28-E24 slice 2: UI joined `build_equipment_catalog` under the
        // `UI` book code -- both `equipment_tables()` (91) and
        // `equipmod_tables()` (7) are served by that one code, mirroring
        // how every other book's equipment and equipment-modifier rows
        // share one book code rather than a separate one.
        ("ultimate_intrigue", "equipment") => Some(equipment_reach(
            "UI",
            ui::equipment_tables::equipment_tables()
                .iter()
                .chain(ui::equipment_tables::equipmod_tables())
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD28-E25 slice 1: UE joined `build_equipment_catalog` under the
        // `UE` book code, both `equipment_tables()` (1,380) and
        // `equipmod_tables()` (180) served by that one code.
        ("ultimate_equipment", "equipment") => Some(equipment_reach(
            "UE",
            ue::equipment_tables::equipment_tables()
                .iter()
                .chain(ue::equipment_tables::equipmod_tables())
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD28 item 5: UM and UPsi joined `build_equipment_catalog` under
        // their own `UM`/`UPSI` book codes -- previously absent from this
        // third, independent hand-maintained book chain even though
        // `equipment_resolver.rs`'s headless pricing/recognition chain
        // already carried both (`§55`, extended for UM/UPsi). Both books'
        // `equipment_tables()` and `equipmod_tables()` are served under one
        // code each, mirroring UI/UE.
        ("ultimate_magic", "equipment") => Some(equipment_reach(
            "UM",
            um::equipment_tables::equipment_tables()
                .iter()
                .chain(um::equipment_tables::equipmod_tables())
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        ("ultimate_psionics", "equipment") => Some(equipment_reach(
            "UPSI",
            upsi::equipment_tables::equipment_tables()
                .iter()
                .chain(upsi::equipment_tables::equipmod_tables())
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // SD28-C4.9: UC joined `build_equipment_catalog` under the `UC`
        // book code, both `equipment_tables()` (185) and `equipmod_tables()`
        // (19) served by that one code.
        ("ultimate_combat", "equipment") => Some(equipment_reach(
            "UC",
            uc::equipment_tables::equipment_tables()
                .iter()
                .chain(uc::equipment_tables::equipmod_tables())
                .map(|entry| entry.key.to_owned())
                .collect(),
        )),
        // UW has NO hand-authored equipment table at all — all 127 of its
        // catalog rows are corpus gap-lane rows, which `equipment_reach`
        // unions in itself. The empty seed set here is the literal truth of
        // this book's hand-authored coverage, not an omission.
        ("ultimate_wilderness", "equipment") => Some(equipment_reach("UW", BTreeSet::new())),
        // `SD31-E6-F10-003`: 8 further already-compiled books extended into
        // the corpus gap lane, same "no hand-authored table at all" shape
        // as `UW` above — every one of their catalog rows is a gap-lane
        // row, which `equipment_reach` unions in itself.
        ("occult_adventures", "equipment") => Some(equipment_reach("OA", BTreeSet::new())),
        ("horror_adventures", "equipment") => Some(equipment_reach("HA", BTreeSet::new())),
        ("inner_sea_races", "equipment") => Some(equipment_reach("ISR", BTreeSet::new())),
        ("inner_sea_world_guide", "equipment") => Some(equipment_reach("ISWG", BTreeSet::new())),
        ("monster_codex", "equipment") => Some(equipment_reach("MC", BTreeSet::new())),
        ("bestiary_2", "equipment") => Some(equipment_reach("B2", BTreeSet::new())),
        ("bestiary_3", "equipment") => Some(equipment_reach("B3", BTreeSet::new())),
        ("bestiary_4", "equipment") => Some(equipment_reach("B4", BTreeSet::new())),
        // `SD31-E6-F10-004`: 5 further already-compiled books extended into
        // the corpus gap lane (the ones `SD31-E6-F10-003` deliberately left
        // out, `OPEN-ISSUES.md` row 186 -- now reachable via a per-record
        // blacklist pre-filter, `gen_equipment_gap_tables.rs`'s
        // `blacklist_hit`). Same "no hand-authored table at all" shape as
        // every entry above.
        ("inner_sea_gods", "equipment") => Some(equipment_reach("ISG", BTreeSet::new())),
        ("mythic_adventures", "equipment") => Some(equipment_reach("MYTHIC", BTreeSet::new())),
        ("inner_sea_combat", "equipment") => Some(equipment_reach("ISC", BTreeSet::new())),
        ("inner_sea_intrigue", "equipment") => Some(equipment_reach("ISI", BTreeSet::new())),
        ("book_of_the_damned_volume_2", "equipment") => {
            Some(equipment_reach("BOTD2", BTreeSet::new()))
        }

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
        // SD-31 Epic 1-F2 (2026-08-15). Bestiary 2's 6-race chassis batch:
        // the first new race-catalog book since the original 18
        // (`decisions.md §25.3`). Same mechanism as CRB/B1 above -- no new
        // surface, `ingest_races` files this book's chassis+standard traits
        // exactly like the other two, so `list_race_catalog` already serves
        // it once `race_catalog::RACE_CORPUS_BOOKS` and `RACE_CATALOG_BOOKS`
        // both carry `"bestiary_2"`/`B2`.
        ("bestiary_2", "races") => Some(races_reach(
            "B2",
            crate::race_catalog::ingested_race_ids_for_book("bestiary_2"),
        )),
        // SD-31 Epic 1 follow-on batch (2026-08-15). Skinwalker's chassis:
        // same mechanism as CRB/B1/B2 above, once `race_catalog::
        // RACE_CORPUS_BOOKS` and `RACE_CATALOG_BOOKS` both carry
        // `"bestiary_5"`/`B5`.
        ("bestiary_5", "races") => Some(races_reach(
            "B5",
            crate::race_catalog::ingested_race_ids_for_book("bestiary_5"),
        )),
        // SD-31 wave-24 integration cycle (2026-08-20). Rougarou's chassis:
        // same mechanism as CRB/B1/B2/B5 above, once `race_catalog::
        // RACE_CORPUS_BOOKS` and `RACE_CATALOG_BOOKS` both carry
        // `"bestiary_6"`/`B6`.
        ("bestiary_6", "races") => Some(races_reach(
            "B6",
            crate::race_catalog::ingested_race_ids_for_book("bestiary_6"),
        )),
        // SD-31-E6-F4-002 (2026-08-16). Advanced Race Guide's own 6-race
        // chassis batch (Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang) --
        // same mechanism as CRB/B1/B2/B5 above, once `race_catalog::
        // RACE_CORPUS_BOOKS` and `RACE_CATALOG_BOOKS` both carry
        // `"advanced_race_guide"`/`ARG`. Before this batch ARG declared zero
        // races (`decisions.md §25`, `alternate_only_books_contribute_no_
        // catalog_rows_but_are_loaded_and_counted`), so this is a new claim,
        // not a widening of an existing one.
        ("advanced_race_guide", "races") => Some(races_reach(
            "ARG",
            crate::race_catalog::ingested_race_ids_for_book("advanced_race_guide"),
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
        // SD-31 Epic 1-F2 (2026-08-15). Bestiary 2's 57 standard racial-trait
        // records, filed under `bestiary_2` by `ingest_races` exactly like
        // CRB's and Bestiary 1's above -- same claim shape, no new surface.
        // (The 48 ARG/ISR *alternate* rows this batch also added are covered
        // by the `advanced_race_guide`/`inner_sea_races` claims below, since
        // `race_traits_reach` filters by the record's own `book_id`, which
        // for an alternate is the book it was actually read from.)
        ("bestiary_2", "race_traits") => Some(race_traits_reach("B2", "bestiary_2")),
        // SD-31 Epic 1 follow-on batch (2026-08-15). Skinwalker's 9
        // standard-tier racial-trait records, filed under `bestiary_5` by
        // `ingest_races` -- same claim shape as `bestiary_2` above. This
        // batch does NOT ingest Skinwalker's heritage-shaped alternates
        // (see `ingest_races.rs`'s `skinwalker` doc comment), so there is
        // no separate alternate-trait claim to add here yet.
        ("bestiary_5", "race_traits") => Some(race_traits_reach("B5", "bestiary_5")),
        // SD-31 wave-24 integration cycle (2026-08-20). Rougarou's 8
        // standard-tier racial-trait records, filed under `bestiary_6` by
        // `ingest_races` -- same claim shape as `bestiary_2`/`bestiary_5`
        // above. No heritage/alternate content exists for this race in the
        // pinned oracle (confirmed: no `*_subrace.lst` file), so there is no
        // separate alternate-trait claim to add here.
        ("bestiary_6", "race_traits") => Some(race_traits_reach("B6", "bestiary_6")),
        // SD-32 `decisions.md §25` cycle 2 (2026-08-23). Bestiary 3's 5
        // "Adopted Race" selector records, filed under `bestiary_3` by
        // `ingest_race_traits.rs`'s new `selector_only` `BookSource` -- the
        // book contributes no standard-tier trait content of its own (its
        // five races' chassis are ARG-native), so unlike `bestiary_2/5/6`
        // this claim's whole `ingested` set is these 5 selector records.
        ("bestiary_3", "race_traits") => Some(race_traits_reach("B3", "bestiary_3")),
        ("advanced_race_guide", "race_traits") => {
            Some(race_traits_reach("ARG", "advanced_race_guide"))
        }
        // SD28-E16 (2026-08-08, decisions.md §39, correcting §37's first
        // estimate of 50): APG's 1 genuinely new alternate racial trait
        // (Half-Orc ~ Plagueborn) -- 49 of the original 50 collided with
        // already-ingested ARG keys and were excluded at ingest time. Same
        // shape as ARG -- served through
        // `race_trait_picker::build_alternate_racial_traits`, not a compiled
        // table -- now that `advanced_players_guide` is in
        // `race_catalog::RACE_CORPUS_BOOKS`.
        ("apg", "race_traits") => Some(race_traits_reach("APG", "advanced_players_guide")),
        // SD-29 Epic 6 pilot (race-trait lane, 2026-08-11, `decisions.md §43`).
        // Monster Codex's 5 in-scope alternate racial traits -- Duergar's 2 and
        // Goblin's 3 -- served by exactly the same two commands ARG's and APG's
        // claims run, now that `monster_codex` is in
        // `race_catalog::RACE_CORPUS_BOOKS`. No new surface was built for this
        // book and none was needed: the picker is book-agnostic and reads
        // whatever the race corpus loads.
        //
        // This is also the claim that retires the `beastiary1/race_traits`
        // finding. `Duergar ~ Ironskinned` carries the only
        // `FACT:Duergar_ReplaceSLAEnlargePerson|True` token in the upstream
        // corpus, which is the positive `PREFACT` gate on Bestiary 1's
        // `Duergar ~ Spell-Like Ability ~ Invisibility` -- so selecting it
        // brings that row in as a `flagGranted` trait and the B1 claim above
        // goes from `NotSurfaced` to a plain pass. See
        // `tests/duergar_invisibility_sla_reaches_a_player_via_monster_codex.rs`.
        ("monster_codex", "race_traits") => Some(race_traits_reach("MC", "monster_codex")),
        // SD-29 Epic 6 round 2 (race-trait lane, extend, 2026-08-11,
        // `decisions.md §45`). Inner Sea Races' 71 in-scope records -- 67 of
        // them `TraitRole::Alternate`, the largest single contribution after
        // ARG's own 153 -- served by exactly the two commands ARG's, APG's and
        // Monster Codex's claims run, now that `inner_sea_races` is in
        // `race_catalog::RACE_CORPUS_BOOKS`. No new surface and no new
        // mechanism: the picker is book-agnostic and reads whatever the race
        // corpus loads, which is precisely why this book was the right one to
        // take next and why `decisions.md §44.4`'s successor queue -- which put
        // two mechanism-blocked books ahead of it -- was wrong.
        ("inner_sea_races", "race_traits") => Some(race_traits_reach("ISR", "inner_sea_races")),
        // SD-29 Epic 6 round 3 (race-trait lane, extend, 2026-08-12,
        // `decisions.md §47`). Horror Adventures' 43 in-scope records from
        // `ha_abilities_race.lst` -- 41 `TraitRole::Alternate` plus the two
        // `Deep Jungle Halfling ~ ...` rows the book's own
        // `Halfling ~ Deep Jungle` alternate grants -- served by exactly the
        // two commands ARG's, APG's, Monster Codex's and ISR's claims run.
        // Again no new surface and no new mechanism.
        //
        // **This is the first book in the lane whose whole ingested family
        // reaches, with no `OPEN_FINDINGS` shortfall.** That is a property of
        // the book's upstream data rather than of this round's care: HA's two
        // non-selectable rows are named by an
        // `ABILITY:Halfling Racial Trait|AUTOMATIC|` grant on the alternate
        // that replaces them, so the upstream transaction ISR's
        // `Human ~ Tribalistic Languages` leaves half-finished is complete
        // here. The absence of a finding is therefore evidence, not an
        // omission -- `horror_adventures_alternate_racial_traits_reach_a_player`
        // asserts the full pass by exact count rather than leaving it unstated.
        ("horror_adventures", "race_traits") => Some(race_traits_reach("HA", "horror_adventures")),
        // SD-29 Epic 6 round 4 (race-trait lane, extend, 2026-08-12,
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
        // SD-29 Epic 5 extend, round 8. Bestiary 1's monster ABILITIES — a
        // family this book has never had a claim for, because until round 8 it
        // had no ability table. The chassis registered under `corpus_book:
        // "beastiary"` writes them to `data/corpus/beastiary/monster_ability/`
        // and `list_monster_catalog` serves them flattened under the monster
        // that owns them, exactly as every other chassis book's are. The
        // monster half stays on `monsters_reach` above, which unions the two
        // tables serving that family; the ability half has only one table and
        // uses the shared helper unchanged.
        ("beastiary1", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("beastiary", "B1"))
        }

        // SD-29 Epic 5 pilot — Bonus Bestiary's two families, both served by
        // the same `list_monster_catalog` command the Bestiary 1 claim above
        // already runs, and both rendered by the same
        // apps/desktop/src/monsterCatalog/MonsterCatalogScreen.tsx.
        //
        // The abilities are NOT a second catalog: per
        // `corpus-work-channels.md` §9.2 a monster ability is to a monster what
        // a race trait is to a race, so the screen renders each one inside its
        // owning creature's row. That is why the two claims below judge the same
        // response from two different denominators — the chassis records and the
        // feature records are genuinely different populations on disk.
        ("bonus_bestiary", "monsters") => Some(chassis_monsters_reach("bonus_bestiary", "BB")),
        ("bonus_bestiary", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("bonus_bestiary", "BB"))
        }
        // SD-29 Epic 5 extend, round 1. Same two claim functions, a
        // different book: the judgement is a property of the chassis, not
        // of Bonus Bestiary, so registering a book is two arms here.
        ("monster_codex", "monsters") => Some(chassis_monsters_reach("monster_codex", "MC")),
        ("monster_codex", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("monster_codex", "MC"))
        }
        // SD-29 Epic 5 extend, round 2. Two more books, four more arms, and
        // the same two claim functions -- which is the registry rewrite's whole
        // point: a book's reach is judged by the chassis, not per book.
        ("book_of_the_damned_volume_1", "monsters") => {
            Some(chassis_monsters_reach("book_of_the_damned_volume_1", "BOTD1"))
        }
        ("book_of_the_damned_volume_1", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("book_of_the_damned_volume_1", "BOTD1"))
        }
        ("book_of_the_damned_volume_2", "monsters") => {
            Some(chassis_monsters_reach("book_of_the_damned_volume_2", "BOTD2"))
        }
        ("book_of_the_damned_volume_2", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("book_of_the_damned_volume_2", "BOTD2"))
        }
        // SD-29 Epic 5 extend, round 3. The first book whose claims cover a
        // SUBSET of the book's corpus rows: 5 of Inner Sea World Guide's 14
        // monster rows carry `NAMEISPI:YES` and are Product Identity, and 13 of
        // its 30 ability rows end up owned by no shipped monster. Both claims
        // assert what is served -- 9 and 14 -- rather than rounding up to the
        // corpus count, which is what claims of 14 and 30 would be doing.
        ("inner_sea_world_guide", "monsters") => {
            Some(chassis_monsters_reach("inner_sea_world_guide", "ISWG"))
        }
        ("inner_sea_world_guide", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("inner_sea_world_guide", "ISWG"))
        }
        // SD-29 Epic 5 extend, round 4. Bestiary 2, and the same two claim
        // functions again at 30x the record count -- 316 monsters and 402
        // abilities. The book already declares a `companions` claim below under
        // the same wire code; a book contributes one claim per FAMILY, and B2 is
        // the first in either lane to carry three.
        ("bestiary_2", "monsters") => Some(chassis_monsters_reach("bestiary_2", "B2")),
        ("bestiary_2", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("bestiary_2", "B2"))
        }
        // SD-29 Epic 5 extend, round 5. Bestiary 3 -- the same two claim
        // functions again. Unlike B2 this book contributes no `companions`
        // family, so these two are the whole of its reach.
        ("bestiary_3", "monsters") => Some(chassis_monsters_reach("bestiary_3", "B3")),
        ("bestiary_3", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("bestiary_3", "B3"))
        }
        // SD-29 Epic 5 extend, round 6. Bestiary 4 -- the same two claim
        // functions again. Like B3 and unlike B2 it contributes no `companions`
        // family, so these two are the whole of its reach.
        ("bestiary_4", "monsters") => Some(chassis_monsters_reach("bestiary_4", "B4")),
        ("bestiary_4", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("bestiary_4", "B4"))
        }
        // SD-29 Epic 5 extend, round 7. Inner Sea Bestiary -- the same two
        // claim functions again, and the first `campaign_setting/` book in this
        // lane whose whole reach is the two monster families.
        ("inner_sea_bestiary", "monsters") => {
            Some(chassis_monsters_reach("inner_sea_bestiary", "ISB"))
        }
        ("inner_sea_bestiary", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("inner_sea_bestiary", "ISB"))
        }
        // SD-29 Epic 5 extend, round 9. Inner Sea Gods -- the same two claim
        // functions again. The registration cost of a book whose files are
        // split across `support/` is paid entirely in the transcriber and the
        // generator; nothing about a reach claim changes.
        ("inner_sea_gods", "monsters") => Some(chassis_monsters_reach("inner_sea_gods", "ISG")),
        ("inner_sea_gods", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("inner_sea_gods", "ISG"))
        }
        // SD-29 Epic 5 extend, round 10. Ultimate Psionics -- the same two claim
        // functions again, under the wire code `UPSI` this app already serves
        // the book's equipment and feats with rather than its own
        // `SOURCESHORT:UP` (`monster_catalog::BOOK_UPSI`, `decisions.md §64.2`).
        // This book already had reach claims for `feats`, `equipment` and
        // `archetypes`; these are the first of its claims this lane owns.
        ("ultimate_psionics", "monsters") => {
            Some(chassis_monsters_reach("ultimate_psionics", "UPSI"))
        }
        ("ultimate_psionics", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("ultimate_psionics", "UPSI"))
        }
        // SD-29 Epic 5 extend, FINAL round. Horror Adventures -- the same two
        // claim functions, under the wire code `HA` this app already serves the
        // book's races and companions with (`monster_catalog::BOOK_HA`).
        //
        // This is the FOURTH family this book claims reach for, after
        // `race_traits` (Epic 6 round 3) and `companions` (Epic 7). The
        // denominator is `data/corpus/horror_adventures/monster*/`, which the
        // generator writes beside the two directories already there -- so the
        // book's other claims are untouched by this one.
        ("horror_adventures", "monsters") => {
            Some(chassis_monsters_reach("horror_adventures", "HA"))
        }
        ("horror_adventures", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("horror_adventures", "HA"))
        }
        // `decisions.md §20` no_record-to-zero, round 3. Five previously-
        // unregistered ZERO-monster books (`scripts/classify_monster_ability_
        // rows.py`'s "ZERO-monster books" line): no `("book", "monsters")`
        // claim exists because none has a `data/corpus/<book>/monster/`
        // directory at all (`monster_chassis::MONSTER_BOOKS`'s
        // `races_lsts: &[]` for each -- `monster_chassis.rs`), only
        // `monster_ability`. Each wire code is the one this app already
        // serves the book's OTHER families under (`monster_catalog::BOOK_UW`/
        // `BOOK_UI`/`BOOK_UM`/`BOOK_B6`/`BOOK_B5`).
        ("ultimate_wilderness", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("ultimate_wilderness", "UW"))
        }
        ("ultimate_intrigue", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("ultimate_intrigue", "UI"))
        }
        ("ultimate_magic", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("ultimate_magic", "UM"))
        }
        ("bestiary_6", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("bestiary_6", "B6"))
        }
        ("bestiary_5", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("bestiary_5", "B5"))
        }
        // `decisions.md §20` no_record-to-zero, round 4. The last two of the
        // original 8 ZERO-monster books -- same shape as round 3 immediately
        // above: no `("book", "monsters")` claim, only `monster_abilities`,
        // under the wire code each book already serves its OTHER families
        // with (`monster_catalog::BOOK_PU`/`BOOK_ARG`).
        ("pathfinder_unchained", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("pathfinder_unchained", "PU"))
        }
        ("advanced_race_guide", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("advanced_race_guide", "ARG"))
        }
        // `decisions.md §20` no_record-to-zero, round 5. The last of the
        // original 8 ZERO-monster books -- same shape as round 4 immediately
        // above: no `("book", "monsters")` claim, only `monster_abilities`,
        // under the wire code this book already serves its OTHER families
        // with (`monster_catalog::BOOK_MYTHIC`, this book's existing
        // `("mythic_adventures", "equipment")` arm above).
        ("mythic_adventures", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("mythic_adventures", "MYTHIC"))
        }
        // `decisions.md §27b` — EVERYTHING: overturns the repeatedly-
        // reconfirmed "correctly out of scope" disposition for this book's 5
        // `monster_ability` units AND its 1 `monster` row (Kami (Shikigami)).
        // Unlike `mythic_adventures` above, this book DOES carry a real
        // `("book", "monsters")` claim -- its chassis registration includes
        // one real `MonsterStatBlock` -- so both kinds get a claim here,
        // under the wire code this book already serves its OTHER families
        // with (`monster_catalog::BOOK_OA`, this book's existing
        // `("occult_adventures", "equipment")` arm above).
        ("occult_adventures", "monsters") => Some(chassis_monsters_reach("occult_adventures", "OA")),
        ("occult_adventures", "monster_abilities") => {
            Some(chassis_monster_abilities_reach("occult_adventures", "OA"))
        }

        // SD-29 Epic 7 (companion lane) -- the kind's first reach claims. Every
        // one is served by `list_companion_catalog` and rendered by
        // apps/desktop/src/companionCatalog/CompanionCatalogScreen.tsx,
        // reachable from the landing screen alongside the other catalogs.
        //
        // ONE claim per book, not two, because the corpus files creature rows
        // and ability rows under one kind -- see `CORPUS_KIND_NAMES`. The
        // judgement below therefore has to satisfy both shapes from one
        // denominator, which is exactly what `companions_reach` does.
        ("inner_sea_combat", "companions") => Some(companions_reach("inner_sea_combat", "ISC")),
        ("monster_codex", "companions") => Some(companions_reach("monster_codex", "MC")),
        ("inner_sea_intrigue", "companions") => {
            Some(companions_reach("inner_sea_intrigue", "ISI"))
        }
        ("horror_adventures", "companions") => Some(companions_reach("horror_adventures", "HA")),
        // SD-29 Epic 7 round 2 (companion lane, extend). The same one-claim-per-
        // book judgement; `companions_reach`'s denominator is the book's own
        // `data/corpus/<book>/companion/` directory, so Bestiary 5's two
        // Occult-Adventures-gated familiars are outside it by construction
        // rather than counted as a shortfall (`decisions.md §47.2`).
        ("bestiary_5", "companions") => Some(companions_reach("bestiary_5", "B5")),
        ("bestiary_6", "companions") => Some(companions_reach("bestiary_6", "B6")),
        ("bestiary_2", "companions") => Some(companions_reach("bestiary_2", "B2")),
        // SD-29 Epic 7 round 3. Bestiary 1's companions, and the only claim in
        // this file whose family id and corpus directory are different words:
        // the family is `beastiary1` (what the ingest diagnostic calls the
        // book), the corpus directory is `beastiary`, and the PCGen source
        // directory is `bestiary`. See `companion_chassis::COMPANION_BOOKS`.
        //
        // A SECOND claim for this book, beside `("beastiary1", "monsters")`.
        // The two judge different populations from the same book —
        // `Companion (Wolf)` is an advanceable companion row, `Wolf` is a stat
        // block — and `beastiary1`'s own
        // `the_companion_rows_are_not_this_module_s_monster_rows` pins that
        // they never collide.
        ("beastiary1", "companions") => Some(companions_reach("beastiary", "B1")),
        // SD-29 Epic 7 round 4. Bestiary 3's companions — the SECOND claim this
        // book carries, beside `("bestiary_3", "monsters")` the monster lane
        // wrote in `9595bd82`. Same both-families-from-one-book shape Bestiary 1
        // and Bestiary 2 already have (`decisions.md §51.5`).
        //
        // ALL 85 of the book's companion units ship, with no `OPEN_FINDINGS`
        // shortfall. That was not the plan: the round opened expecting 19
        // orphans and built the drop-and-record disposition `§50` prescribes,
        // then found the 19 were never orphans at all — their creature rows are
        // namespaced by `OUTPUTNAME:` (`KEY:Kyton (Augur)` displays as `Augur`,
        // and its abilities are keyed `Augur ~ …`). Reading that token is
        // ownership shape 5 (`decisions.md §56.1`).
        ("bestiary_3", "companions") => Some(companions_reach("bestiary_3", "B3")),
        // SD-29 Epic 7 round 5. Bestiary 4's companions — the SECOND claim this
        // book carries, beside `("bestiary_4", "monsters")` the monster lane
        // wrote in `52da4bc3`.
        //
        // 78 of the book's 80 companion units ship — its whole `reachable
        // remainder`. ZERO are dropped as orphans: the round opened with five on
        // the board and found every one owned across a `CATEGORY:Internal` relay
        // row that is not itself an inventory unit (`Familiar (Giant Flea)` ->
        // `Racial Traits ~ Flea (Giant)` -> `Flea (Giant) ~ Disease`), which is
        // ownership shape 6 (`decisions.md §59.1`). The two exclusions are
        // `.COPY=` DELTA rows (`§59.2`), which state a delta on a base record
        // rather than a record.
        //
        // No `OPEN_FINDINGS` entry, and that is the correct outcome rather than
        // an omission: this list is per FAMILY, `bestiary_4/companions` reaches
        // a player, and adding a surfaced family here would fail
        // `unsurfaced_families_are_exactly_the_recorded_findings` in the other
        // direction.
        ("bestiary_4", "companions") => Some(companions_reach("bestiary_4", "B4")),

        // SD-29 Epic 7 round 6. Ultimate Wilderness's companions — the SECOND
        // claim this book carries, beside the 136-feat catalog SD-28 Epic 26
        // landed, and the LARGEST companion block in the corpus: 169 creature
        // rows, more than every previously registered companion book combined.
        //
        // 327 of the book's 575 companion units ship — its whole `reachable
        // remainder` per `scripts/classify_companion_rows.py`. The other 248 are
        // NOT ingested and therefore not in this gate's denominator: unlike
        // every earlier book in this lane, the shortfall is a DIFFERENT KIND of
        // record wearing this kind's file name (30 `CATEGORY:Archetype` rows,
        // the 119 ability rows namespaced under their display names, and the 72
        // `Animal Trick`/`Animal Companion Feat` option-group rows), not rows
        // the transcriber failed to read. They get no `OPEN_FINDINGS` entry
        // because that list is per FAMILY and this family reaches a player;
        // `docs/work-inventory.json` is where they stay counted
        // (`decisions.md §61.2`).
        ("ultimate_wilderness", "companions") => {
            Some(companions_reach("ultimate_wilderness", "UW"))
        }

        // SD-29 Epic 7 round 8. Core Rulebook's companions and familiars — the
        // SIXTH family this book carries, beside its classes, races, spells,
        // equipment and race traits.
        //
        // The key on the left is the ENGINE book id `crb`, not the corpus
        // directory `core_rulebook`, and the two arguments to `companions_reach`
        // are the corpus directory and the wire code. All three spellings differ
        // here, exactly as they do for Bestiary 1 (`decisions.md §54.3`), and
        // `CORPUS_BOOK_IDS` already carried `("core_rulebook", "crb")` for the
        // book's five older families, so no entry was needed this round.
        //
        // 84 of the book's 170 companion units ship — its whole `reachable
        // remainder` per `scripts/classify_companion_rows.py`. The 86 that do
        // not are absent from this gate's denominator because they were never
        // ingested: 84 orphan ability rows and the 2 `cr_classes_companion.lst`
        // CLASS rows (`§65.1`). They stay counted in `docs/work-inventory.json`
        // and get no `OPEN_FINDINGS` entry, for the reason `§61.2` states —
        // that list is keyed by FAMILY, and this family does reach a player.
        //
        // The shortfall here is ONE finding, not 86: the orphans are the generic
        // `Animal Companion ~ …` / `Animal Trick ~ …` records, and they are
        // orphans precisely because they hang off the Animal Companion CLASS
        // rather than off any creature. Both groups need the same missing record
        // type, which is named in `crb/mod.rs`'s module doc rather than split
        // across 86 rows here.
        ("crb", "companions") => Some(companions_reach("core_rulebook", "CRB")),

        // SD-29 Epic 7 round 9 (companion lane, FINAL PASS). Four books at
        // once, and the shortfall behind all four is ONE finding.
        //
        // 52 units ship between them — Ultimate Magic 32, Advanced Race Guide
        // 14, Advanced Player's Guide 4, Book of the Damned Volume 1 2 — and
        // each figure is exactly the `reachable remainder`
        // `python3 scripts/classify_companion_rows.py <book>` prints for that
        // book. The 393 that do not ship are absent from this gate's
        // denominator because they were never ingested; they stay counted in
        // `docs/work-inventory.json` and get no `OPEN_FINDINGS` entry, for the
        // reason `§61.2` states — that list is keyed by FAMILY, and every one
        // of these four families does reach a player.
        //
        // THE ONE FINDING: 361 of the 393 are the summoner's evolution pool
        // (`Evolution ~ …`, `Temp Evolution ~ …`, `<Archetype> Eidolon ~ …`,
        // `WCEvolution ~ …`) and the bladebound magus's `Black Blade ~ …`
        // records. Both hang off a CLASS FEATURE rather than off any creature
        // row, which is the same missing record type round 8 named for Core
        // Rulebook's 84 `Animal Companion ~ …` orphans (`decisions.md §65`).
        // The remaining 32 are 27 Book-of-the-Damned `Imp Companion ~ …`
        // orphans of the same shape plus 5 `*_classes_companion.lst` CLASS rows
        // (`§65.1`), which is that same type seen from the other side.
        //
        // Note the key on the left is the ENGINE book id and the first argument
        // is the CORPUS directory. They differ only for the APG, whose module
        // has been `apg` since long before this lane; `CORPUS_BOOK_IDS` already
        // carried `("advanced_players_guide", "apg")` for the book's older
        // families, so no entry was needed this round.
        ("ultimate_magic", "companions") => Some(companions_reach("ultimate_magic", "UM")),
        ("advanced_race_guide", "companions") => {
            Some(companions_reach("advanced_race_guide", "ARG"))
        }
        ("apg", "companions") => Some(companions_reach("advanced_players_guide", "APG")),
        ("book_of_the_damned_volume_1", "companions") => {
            Some(companions_reach("book_of_the_damned_volume_1", "BOTD1"))
        }

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

        // Archetypes: SD28-C4.8/§60/§63's tier-1 archetype-swap catalog.
        // Honestly `NotSurfaced` for every book today, not a stub omission
        // -- the same disposition `§62` used for `companion` (0% where the
        // mechanism is genuinely absent). `archetype_resolver` reads a
        // real `SelectedChoice { choice_set_id: ARCHETYPE_CHOICE_ID, .. }`
        // and the engine grounds the swap correctly for the wired slots
        // (Alchemist's three, Fighter's Bravery), proven end-to-end by
        // `build_pilot_headless_receipt` in the reachability tests -- but
        // there is no desktop command or picker anywhere that lets a
        // player make that selection (no `archetype_catalog.rs`, no
        // choice-set surface on the character sheet). §43's engine-holds
        // vs player-reaches boundary is exactly this case: the engine
        // holds it, the product does not yet let anyone reach it.
        ("acg", "archetypes") => Some(archetypes_reach(
            "acg",
            acg::archetype_tables::archetype_swap_tables(),
        )),
        ("advanced_race_guide", "archetypes") => Some(archetypes_reach(
            "advanced_race_guide",
            arg::archetype_tables::archetype_swap_tables(),
        )),
        ("apg", "archetypes") => Some(archetypes_reach(
            "apg",
            apg::archetype_tables::archetype_swap_tables(),
        )),
        ("ultimate_combat", "archetypes") => Some(archetypes_reach(
            "ultimate_combat",
            uc::archetype_tables::archetype_swap_tables(),
        )),
        ("ultimate_magic", "archetypes") => Some(archetypes_reach(
            "ultimate_magic",
            um::archetype_tables::archetype_swap_tables(),
        )),
        ("ultimate_psionics", "archetypes") => Some(archetypes_reach(
            "ultimate_psionics",
            upsi::archetype_tables::archetype_swap_tables(),
        )),
        ("ultimate_wilderness", "archetypes") => Some(archetypes_reach(
            "ultimate_wilderness",
            uw::archetype_tables::archetype_swap_tables(),
        )),

        _ => None,
    }
}

/// The shared, honest verdict for every book's archetype-swap table -- see
/// the doc comment on the `reach_of` arms above for why this is
/// `NotSurfaced` rather than a claim of reach. `missing` carries every
/// ingested key, because none of them reach the player through any
/// existing desktop surface, not merely some.
fn archetypes_reach(
    book: &str,
    entries: &[codex::rules_core::rules_tables::archetype_swap::ArchetypeSwapEntry],
) -> Reach {
    let missing: BTreeSet<String> = entries.iter().map(|entry| entry.key.to_owned()).collect();
    Reach::NotSurfaced {
        why: format!(
            "{book}'s {} archetype-swap records are ingested but not surfaced -- no desktop \
             command or picker exists yet to let a player select an archetype at all, so none \
             of this book's records reach anyone regardless of what the engine can already \
             compute for a hand-constructed selection (see OPEN_FINDINGS for this family's \
             per-book detail)",
            entries.len()
        ),
        missing,
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
    // The corpus gap lane (`epic-4-proven-equip-mod`) ingested 769 equipment /
    // equipment-modifier records that no hand-authored per-book table holds.
    // They are unioned into the CLAIM, not merely into the catalog: a gate
    // that widened only what the surface serves, while leaving the ingested
    // set at the hand tables alone, would assert nothing about the new rows
    // and would go on passing if every one of them silently stopped reaching
    // the picker.
    let mut ingested = ingested;
    ingested.extend(
        codex::rules_core::rules_tables::equipment_gap_tables::equipment_gap_rows()
            .filter(|row| row.book == wire_book)
            .map(|row| row.key.to_owned()),
    );

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

    // `Human ~ Adoptive Parentage`'s CHOOSE pool (`decisions.md §16` item 2,
    // SD-32 card-11 T2b lane) — surfaced by the same command, in its own
    // top-level field rather than nested under a race, because picking one
    // is not a trait *of* the race it names (see
    // `AdoptiveParentageOptionDto`'s doc comment). A row with a real target
    // race and a resolved grant is payload; nothing in this cycle's batch
    // ever resolves to zero grants, but an empty `grants` list would count as
    // identity-only rather than silently as payload, the same rule every
    // other family in this function applies.
    for option in menu.adoptive_parentage_options.iter().filter(|option| option.book == wire_book) {
        if option.grants.is_empty() {
            identity_only.insert(option.key.clone());
        } else {
            with_payload.insert(option.key.clone());
        }
    }

    // SD-32 `decisions.md §25` cycle 2's "Adopted Race" selector rows -- the
    // same top-level-field shape the Adoptive Parentage block above uses, for
    // the identical reason (picking one is not a trait *of* the race it
    // names). **13 of 14 resolve `with_payload` today**:
    // `trait_pool::load_trait_pool`'s `ability/`-fallback (that module's own
    // doc comment) finds real, already-shipped `inner_sea_races` content for
    // every target race except Rougarou, whose pool is genuinely, corpus-
    // wide empty (`epic-6-kind-trait_cycle-1_cycle_receipt.md §1`'s own
    // finding, re-confirmed here) -- see `BARE_RECORD_FINDINGS`'s single
    // remaining entry and `race_trait_picker.rs`'s own
    // `the_menu_command_carries_all_fourteen_adopted_race_options_thirteen_
    // with_real_grants`.
    for option in menu.adopted_race_options.iter().filter(|option| option.book == wire_book) {
        if option.grants.is_empty() {
            identity_only.insert(option.key.clone());
        } else {
            with_payload.insert(option.key.clone());
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
/// Bestiary 1's monster family — **both** tables that serve it.
///
/// SD-29 Epic 5 round 8 (`decisions.md §58.3`) put a second table behind this
/// one family: SD-22's hand-modelled 46 write their identity as `data.id` (they
/// predate the `key` convention), the chassis's 284 write `data.key`. One family
/// key, one wire code, one screen — so one claim, over the union of both
/// denominators, rather than two claims the gate cannot both declare.
///
/// Unioning is also what keeps the claim honest. Reading only `data.id` would
/// judge 46 records and silently ignore 284; reading only `data.key` would do
/// the reverse. Either half alone is a claim that passes while checking a third
/// of the book.
fn monsters_reach() -> Reach {
    let mut ingested = corpus_record_ids("beastiary", "monster");
    ingested.extend(corpus_record_keys("beastiary", "monster"));

    let response = crate::monster_catalog::build_monster_catalog();
    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for entry in response.entries.iter().filter(|entry| entry.book == "B1") {
        // The catalog row prints the monster's name, its size and creature
        // type, its challenge rating, its movement and source page, its
        // `MONSTERCLASS:` token, its natural attacks and its abilities. `key`
        // is the corpus identity and the name is derived from it, so neither
        // counts as payload: a row reaches the player when it carries something
        // about the creature. The two tables fill different subsets of those
        // fields, so the rule is their union — the SD-22 half carries no
        // `speeds`/`monster_class`/`abilities` and the chassis half carries no
        // `natural_attacks` provenance, and neither absence is a failure to
        // reach.
        let has_payload = !entry.race_type.trim().is_empty()
            || !entry.size.trim().is_empty()
            || !entry.source_page.trim().is_empty()
            || !entry.speeds.is_empty()
            || entry.monster_class.is_some()
            || !entry.natural_attacks.is_empty()
            || !entry.abilities.is_empty();
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

/// One chassis book's monster stat blocks, judged against the real
/// `list_monster_catalog` response.
///
/// The denominator is the record files under `data/corpus/<book>/monster/`,
/// read from disk; the numerator is the served response. Neither side reads
/// the compiled `rules_tables` module, so a table that stopped reaching the
/// wire fails here instead of agreeing with itself.
///
/// Book-parameterized rather than duplicated per book: the payload rule below
/// is a property of the chassis DTO, and a copy per book is a copy that drifts.
fn chassis_monsters_reach(corpus_book: &str, wire_code: &str) -> Reach {
    let ingested = corpus_record_keys(corpus_book, "monster");

    let response = crate::monster_catalog::build_monster_catalog();
    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for entry in response.entries.iter().filter(|entry| entry.book == wire_code) {
        // Name and key are identity, so neither counts. A row reaches the
        // player when it carries something *about* the creature: its creature
        // type, its size, its source page, its movement, its `MONSTERCLASS:`
        // token, an attack, or an ability.
        let has_payload = !entry.race_type.trim().is_empty()
            || !entry.size.trim().is_empty()
            || !entry.source_page.trim().is_empty()
            || !entry.speeds.is_empty()
            || entry.monster_class.is_some()
            || !entry.natural_attacks.is_empty()
            || !entry.abilities.is_empty();
        if has_payload {
            with_payload.insert(entry.key.clone());
        } else {
            identity_only.insert(entry.key.clone());
        }
    }

    assess("list_monster_catalog", &ingested, &with_payload, &identity_only)
}

/// One companion book's `companion` records -- BOTH structural shapes -- judged
/// against the real `list_companion_catalog` response.
///
/// The denominator is every record file under `data/corpus/<book>/companion/`,
/// read from disk, which holds the book's creature rows and its ability rows
/// alike because the corpus files them under one kind. The numerator is the
/// served response: creatures at the top level, abilities flattened out of the
/// creatures that own them, which is exactly how the screen renders them.
/// Neither side reads the compiled `rules_tables` module, so a table that
/// stopped reaching the wire fails here instead of agreeing with itself.
///
/// A creature reaches the player when its row carries something *about* the
/// creature beyond its name; an ability reaches when its row says something
/// beyond its name. Both rules are stated below rather than shared, because the
/// two shapes genuinely have different fields.
fn companions_reach(corpus_book: &str, wire_code: &str) -> Reach {
    let ingested = corpus_record_keys(corpus_book, "companion");

    let response = crate::companion_catalog::build_companion_catalog();
    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for entry in response.entries.iter().filter(|entry| entry.book == wire_code) {
        let has_payload = entry.race_type.as_deref().is_some_and(|t| !t.trim().is_empty())
            || entry.size.as_deref().is_some_and(|s| !s.trim().is_empty())
            || entry.source_page.as_deref().is_some_and(|p| !p.trim().is_empty())
            || !entry.speeds.is_empty()
            || entry.monster_class.is_some()
            || entry.natural_armor.is_some()
            || !entry.natural_attacks.is_empty()
            // SD31-W15-COMPANION-001: the row's `BONUS:WEAPONPROF=…|DAMAGE|`
            // tokens, rendered as the PF1 rule they state. Added to the
            // predicate because it is genuine per-row payload a player reads —
            // NOT because any row needs it to reach: every row carrying one
            // already reaches on `natural_attacks`/`stat_adjustments`, and this
            // clause moving no record is asserted by
            // `the_damage_bonus_column_moves_no_record_into_reach` below.
            || !entry.natural_attack_damage_bonuses.is_empty()
            // SD31 wave 16: the row's `BONUS:SKILL|<skills>|<A>-<B>` tokens,
            // rendered as the PF1 rule they state. Added for the identical
            // reason override 2095's own comment states for the sibling
            // damage-bonus clause: genuine per-row payload a player reads,
            // NOT because any row needs it to reach — every companion row
            // carrying this token also carries `BONUS:STAT` adjustments, and
            // this clause moving no record is asserted by
            // `the_skill_bonus_column_moves_no_record_into_reach` below.
            || !entry.skill_ability_diff_bonuses.is_empty()
            || !entry.stat_adjustments.is_empty()
            || !entry.abilities.is_empty();
        if has_payload {
            with_payload.insert(entry.key.clone());
        } else {
            identity_only.insert(entry.key.clone());
        }
        for ability in &entry.abilities {
            let has_payload = ability.facet.is_some()
                || ability.delivery.is_some()
                || !ability.type_segments.is_empty()
                || ability.description.as_deref().is_some_and(|d| !d.trim().is_empty())
                // A row whose rules text is stated ONLY per condition still
                // shows a player rules text — `description` is `None` for it by
                // construction (`companion_chassis::CompanionDescriptionVariant`),
                // so a predicate reading only `description` would judge Ultimate
                // Wilderness's `Poison` and `Constrict` rows identity-only while
                // the screen renders four paragraphs under them.
                || ability.description_variants.iter().any(|v| !v.text.trim().is_empty())
                || !ability.stat_adjustments.is_empty()
                // wave 17: the row's DESC-embedded save-DC formula
                // (`<base>[+HD/2]+<ability>`), rendered as the PF1 rule it
                // states. Added for the identical reason the sibling
                // damage-bonus/skill-bonus clauses above state: genuine
                // per-row payload a player reads, NOT because any row needs
                // it to reach — every ability carrying this token also
                // carries real `description`/`description_variants` prose,
                // and this clause moving no record is asserted by
                // `the_save_dc_formula_column_moves_no_record_into_reach`
                // below.
                || !ability.save_dc_formulas.is_empty()
                || ability.source_page.is_some();
            if has_payload {
                with_payload.insert(ability.key.clone());
            } else {
                identity_only.insert(ability.key.clone());
            }
        }
    }

    assess("list_companion_catalog", &ingested, &with_payload, &identity_only)
}

/// Bonus Bestiary's `monster_ability` records, judged against the same real
/// `list_monster_catalog` response — flattened out of the monsters that own
/// them, which is exactly how the screen renders them.
///
/// An ability reaches the player when the row it prints says something beyond
/// its name: its facet, how it is delivered, its rules text, or its page. One
/// record in this book (`Magic Circle against Evil`) carries no `DESC:` at all,
/// so it reaches on facet + delivery alone — a real, checkable corpus fact
/// rather than a payload this gate invented for it.
fn chassis_monster_abilities_reach(corpus_book: &str, wire_code: &str) -> Reach {
    let ingested = corpus_record_keys(corpus_book, "monster_ability");

    let response = crate::monster_catalog::build_monster_catalog();
    let mut with_payload = BTreeSet::new();
    let mut identity_only = BTreeSet::new();
    for ability in response
        .entries
        .iter()
        .filter(|entry| entry.book == wire_code)
        .flat_map(|entry| entry.abilities.iter())
    {
        let has_payload = !ability.facet.trim().is_empty()
            || ability.delivery.is_some()
            || ability.description.as_deref().is_some_and(|d| !d.trim().is_empty())
            || ability.source_page.is_some();
        if has_payload {
            with_payload.insert(ability.key.clone());
        } else {
            identity_only.insert(ability.key.clone());
        }
    }

    assess("list_monster_catalog", &ingested, &with_payload, &identity_only)
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
        "monster_codex",
        "race_traits",
        "4 of Monster Codex's 5 ingested race-trait records reach a player through \
         `list_alternate_racial_traits` and `resolve_race_alternate_selection`, and this gate \
         refuses partial credit. ONE does not: `Oversized Goblin` \
         (`mc_abilities_race.lst:31`). Derived, not assumed: it carries no \
         `FACT:<flag>|True` token and no positive `PREFACT` gate, so \
         `race_resolver::classify` leaves it `TraitRole::Unclassified` -- the role that never \
         applies -- and it is correctly absent from the picker's alternate list. \
         \
         **It is not an ARG-shaped swap and no wiring would make it one.** Upstream it is one of \
         two Goblin VARIANTS (`Standard Goblin` and `Oversized Goblin`), chosen out of an ability \
         pool that `mc_abilities_race.lst:26` grants with \
         `CATEGORY=Internal|Racial Traits ~ Goblin.MOD  BONUS:ABILITYPOOL|Goblin Variant|1`. \
         Picking the variant is what grants its two replacement rows \
         (`Oversized Goblin ~ Ability Scores`, `~ Size`), which is also why those two are the \
         only alternates in the whole menu carrying no `PREMULT` self-exclusion guard \
         (`race_trait_picker::every_alternate_has_a_readable_exclusion_guard_including_the_preability_spelling` \
         pins them by name). \
         \
         REMEDY: an ability-pool variant mechanism -- a race-level choice of one row out of a \
         `BONUS:ABILITYPOOL|<Pool>|n` pool, whose selection grants the rows TYPEd for it. That is \
         a new mechanism, not a missing wire, and it is outside the race-trait lane's \
         replace-flag protocol. Until it exists, the two replacement rows are offered \
         individually in the picker where the rules would grant them together; that is the \
         visible consequence and it is recorded here rather than smoothed over. \
         Do NOT close this by deleting the record: the row is real corpus content for a modelled \
         race, and a record on disk that no selection can reach is exactly what this gate is for.",
    ),
    (
        "inner_sea_races",
        "race_traits",
        "88 of Inner Sea Races' 94 ingested race-trait records reach a player through \
         `list_alternate_racial_traits` and `resolve_race_alternate_selection`. SIX do not. \
         `Human ~ Tribalistic Languages` (`isr_abilities_race.lst:216`): the row carries no \
         `FACT:<flag>|True`, no positive `PREFACT`, no `PREABILITY` and no `!PREFACT`, and no \
         other row in the book names it -- \
         `grep -o 'ABILITY:[^\\t]*Tribalistic Languages' isr_abilities_race.lst` returns nothing, \
         where the same grep for `Junk Tinker ~ Skilled` returns its granter and that row is \
         therefore `TraitRole::FlagGranted`. So `race_resolver::classify` leaves it \
         `TraitRole::Unclassified`, the role that never applies. \
         \
         **This is an upstream data gap, not a wiring gap, and the distinction is evidenced.** \
         The alternate that logically owns it, `Human ~ Tribalistic` (`:210`), IS reachable and \
         IS selectable, and selecting it correctly fires `Human_ReplaceLanguages`, which \
         suppresses the standard `Human ~ Languages` row. Nothing then brings the replacement \
         in. The player sees a language trait removed and no replacement offered, which is the \
         visible consequence and is recorded here rather than smoothed over. \
         \
         REMEDY: either read `TEMPLATE:`-borne grants (the row's own \
         `TEMPLATE:Bonus Language ~ Common|...` chain is how upstream delivers its effect), or \
         model human ethnicities as the `PREABILITY:1,CATEGORY=Background,TYPE.HumanEthnicity` \
         gate on `:210` implies. Both are new mechanisms, not missing wires. \
         Do NOT close this by deleting the record: it is real corpus content for a modelled \
         race, and the same rule the `Oversized Goblin` entry above states applies here. \
         \
         Four more added 2026-08-22 (SD-32 card 11 T2b lane, re-running the book against the \
         pinned oracle after `IN_SCOPE_RACES` grew past 82 with no regen -- see the \
         count-pinning test's comment): `Mostly Human ~ Ifrit ~ Languages`, \
         `~ Sylph ~ Languages`, `~ Undine ~ Languages` (already-open, unchanged by this lane) \
         plus the newly-ingested `~ Suli ~ Languages` -- all four share one root cause, an \
         unmodelled Geneiekin heritage: their common granter, \
         `Geneiekin ~ Mostly Human.MOD` (`isr_abilities_race.lst:650-654`), carries a `TYPE:<Race> \
         Racial Trait` component this project's `is_mod_row` guard correctly excludes from \
         ingestion, so no selection can ever fire the `<Race>_ReplaceLanguages` flag these four \
         rows are gated on. And `Suli ~ Trusted Mediator` (`:1266`, also newly ingested): unlike \
         its structurally identical siblings `Ifrit ~ Brazen Flame`/`Oread ~ Isolated`/ \
         `Sylph ~ Secretive` (each of which sets its own `FACT:<Race>_Replace...|True` pair), this \
         row sets none at all -- a genuine upstream PCGen omission for Suli specifically, the \
         same shape as `Human ~ Tribalistic Languages` above. \
         REMEDY for the Geneiekin four: model the Geneiekin heritage the way Aasimar/Tiefling \
         already are (`OPEN-ISSUES.md` row 18) -- a new mechanism, not a missing wire. REMEDY for \
         `Suli ~ Trusted Mediator`: none available project-side; the upstream data itself omits \
         the flag. Do NOT close any of the six by deleting the record.",
    ),
    // SD28-C4.8/§60/§63: the tier-1 archetype-swap catalog, 406 records (403 + 3 Slayer archetypes, SD31-E4-F1-001)
    // across 7 books. `archetype_resolver::archetype_claiming_slot` grounds
    // the swap correctly in compute output for the wired slots (Alchemist's
    // Mutagen/Discovery/Poison Resistance, Fighter's Bravery -- proven via
    // `build_pilot_headless_receipt` in each book's own reachability test
    // module), but that is an engine capability exercised by a
    // hand-constructed `SelectedChoice`, not a player-reachable one: no
    // desktop command or picker (`archetype_catalog.rs`, a choice-set
    // surface on the character sheet) exists to let anyone actually pick an
    // archetype. Remedy: build that surface, then delete these seven
    // entries as their books gain a real claim (mirroring how the
    // equipment/feat/spell catalogs each closed their own OPEN_FINDINGS
    // entry when their picker landed).
    (
        "acg",
        "archetypes",
        "Gap: 90 ACG archetype-swap records are ingested (87 + 3 Slayer archetypes -- Bounty \
         Hunter, Deliverer, Stygian Slayer -- added by SD31-E4-F1-001, 2026-08-16) and the engine \
         can already ground a selected one's slot-swap correctly, but zero reach a player -- no \
         archetype-selection surface exists anywhere in the desktop app. Remedy: build an \
         archetype picker (archetype_catalog.rs plus a choice-set surface on the character sheet, \
         the same shape equipment/feat/spell catalogs already use) and wire the per-slot \
         supersession for this book's classes; delete this entry once ACG's archetypes reach the \
         picker.",
    ),
    (
        "advanced_race_guide",
        "archetypes",
        "Gap: 59 ARG archetype-swap records are ingested; the engine already grounds one of \
         them correctly (Plague Bringer's supersession of Alchemist's Poison Resistance, the \
         proof-of-concept for `archetype_claiming_slot`), but no picker exists to let a player \
         select ANY archetype, so even that one does not reach a player yet. Remedy: same as \
         ACG above -- an archetype picker plus per-slot wiring; delete once ARG's archetypes \
         reach the picker.",
    ),
    (
        "apg",
        "archetypes",
        "Gap: 80 APG archetype-swap records are ingested; the engine already grounds one of \
         them correctly (Archer's supersession of Fighter's Bravery, the second class this \
         epic's archetype-swap measurement wired), but no picker exists to let a player select \
         ANY archetype, so even that one does not reach a player yet. Remedy: same as ACG \
         above; delete once APG's archetypes reach the picker.",
    ),
    (
        "ultimate_combat",
        "archetypes",
        "Gap: 68 Ultimate Combat archetype-swap records are ingested (65 + 2 Gunslinger \
         archetypes, Pistolero and Mysterious Stranger, added by `SD31-E4-F1-002`, + 1 Ninja \
         archetype, Scout, added by `SD31-E4-F1-003`) with no archetype-selection surface to \
         reach a player through. Remedy: same as ACG above; delete once landed.",
    ),
    (
        "ultimate_magic",
        "archetypes",
        "Gap: 67 Ultimate Magic archetype-swap records are ingested with no archetype-selection \
         surface to reach a player through. Remedy: same as ACG above; delete once landed.",
    ),
    (
        "ultimate_psionics",
        "archetypes",
        "Gap: 15 Ultimate Psionics archetype-swap records are ingested with no \
         archetype-selection surface to reach a player through. Remedy: same as ACG above; \
         delete once landed.",
    ),
    (
        "ultimate_wilderness",
        "archetypes",
        "Gap: 30 Ultimate Wilderness archetype-swap records are ingested with no \
         archetype-selection surface to reach a player through. Remedy: same as ACG above; \
         delete once landed.",
    ),
    // SD-31 `epic-5-chassis-sweep` F1 (`SD31-E5-F1-001`, 2026-08-15). 21
    // books' `class_feature` corpus JSON went from absent to real, byte-
    // verified content in one cycle (`cache_gen::class_feature`,
    // `artifacts/SD31-E5-F1-001-lever-measurement.md` for the full proving
    // trace). No reach claim exists for any of them because none can,
    // honestly, today: `Kind::ClassFeature`'s doneness path
    // (`v06_work_inventory.rs`'s `classify()`) requires the ENGINE to
    // already compute the feature (a consumer-delta probe or a matching
    // `explanation_id`) before a record can ground at all -- a corpus
    // dump cannot manufacture that, only unlock the `literal-verified`
    // stamp for the rare subset (14 units, corpus-wide) already grounded
    // by pre-existing engine wiring. The remaining, overwhelming majority
    // of each book's class_feature content genuinely has no engine
    // chassis wiring yet -- that is `epic-4-mechanism`'s scope, which has
    // landed zero classes corpus-wide as of this cycle
    // (`SD-31-corpus-closure-grind/kanban.md`'s `epic-4-mechanism` row).
    // Remedy, identical for all 21: wire that class's chassis mechanism
    // (per-class, `epic-4-mechanism`), then either claim the family here
    // or delete the entry once its records reach a player. Record counts
    // below are this cycle's own written totals per book (primary-file
    // sourced only; see the same artifact for the nested-support-file and
    // `ultimate_psionics` exclusions).
    ("acg", "class_features", "Gap: 2395 Advanced Class Guide class_feature records are ingested corpus-wide; the engine's own per-class mechanism wiring (epic-4-mechanism) has not landed for any of ACG's classes yet, so no reach claim exists. Remedy: wire per-class chassis mechanisms, then claim or delete per class as they land."),
    ("advanced_race_guide", "class_features", "Gap: 643 Advanced Race Guide class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("apg", "class_features", "Gap: 2055 Advanced Player's Guide class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("bestiary_4", "class_features", "Gap: 4 Bestiary 4 class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("bestiary_6", "class_features", "Gap: 18 Bestiary 6 class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("book_of_the_damned_volume_1", "class_features", "Gap: 10 Book of the Damned Vol. 1 class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("book_of_the_damned_volume_2", "class_features", "Gap: 205 Book of the Damned Vol. 2 class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("crb", "class_features", "Gap: 959 Core Rulebook class_feature records are ingested corpus-wide; most CRB classes have SOME pre-existing engine wiring (14 of the 25 corpus-wide static-grounded units this cycle promoted are CRB's own already-computed features), but the great majority of CRB's own class_feature records still have no engine wiring -- epic-4-mechanism has not run a dedicated CRB pass. Remedy: same as ACG above, per remaining class/feature."),
    ("horror_adventures", "class_features", "Gap: 165 Horror Adventures class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("inner_sea_combat", "class_features", "Gap: 306 Inner Sea Combat class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("inner_sea_intrigue", "class_features", "Gap: 158 Inner Sea Intrigue class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("inner_sea_world_guide", "class_features", "Gap: 142 Inner Sea World Guide class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("monster_codex", "class_features", "Gap: 68 Monster Codex class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("ultimate_magic", "class_features", "Gap: 1069 Ultimate Magic class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("ultimate_wilderness", "class_features", "Gap: 753 Ultimate Wilderness class_feature records are ingested corpus-wide; no per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("occult_adventures", "class_features", "Gap: 979 Occult Adventures class_feature records are ingested corpus-wide (first corpus JSON this book has ever had, any kind); every one of OA's 6 base classes (Occultist, Spiritualist, Medium, Mesmerist, Kineticist, Psychic) measures 0/N wired-able per epic-3-measurement's own SD31-E3-F1-001 clearance table, so no reach claim can exist until real class-chassis wiring lands. Remedy: same as ACG above."),
    ("adventurers_guide", "class_features", "Gap: 651 Adventurer's Guide class_feature records are ingested corpus-wide (first corpus JSON this book has ever had, any kind); this book carries no compiled rule set at all today (`evidence: no_compiled_rule_set_for_book` on every sampled unit), so no reach claim can exist until the book itself is onboarded to the engine, not only its class_feature kind. Remedy: onboard the book's class chassis, then wire per-class mechanisms as ACG above."),
    ("ultimate_combat", "class_features", "Gap: 991 Ultimate Combat class_feature records are ingested corpus-wide (first data/corpus/ultimate_combat/ directory this book has ever had -- its existing feats/equipment/archetypes reach claims are sourced directly from the compiled rules_tables module, never from data/corpus/). Gunslinger (SD31-E4-F1-002), Ninja (SD31-E4-F1-003) and now Samurai (SD31-E4-F1-004: real chassis BAB/saves plus Challenge uses/day+damage bonus, Resolve uses/day, and Bonus Feat count, all in pilot_compute.rs; named_raw: 0 archetype content RE-VERIFIED not merely inherited, so no supersession branch was built) all now have real pilot_compute.rs chassis wiring, but zero of any of the three classes' records can reach `done` or `held` on the board: `v06_work_inventory.rs`'s `modelled_class_books()` names only CRB/APG/ACG, so no Ultimate Combat class_feature record is even classified against the wiring that exists (OPEN-ISSUES.md row 96, lane 1's file, not this card's). Remedy: register UcClassId::ALL into modelled_class_books() (row 96) -- all three UC classes are now wired and waiting on that single lane-1 fix."),
    ("ultimate_intrigue", "class_features", "Gap: 651 Ultimate Intrigue class_feature records are ingested corpus-wide (first data/corpus/ultimate_intrigue/ directory this book has ever had, same shape as Ultimate Combat above). No per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("inner_sea_magic", "class_features", "Gap: 198 Inner Sea Magic class_feature records are ingested corpus-wide (first corpus JSON this book has ever had, any kind). No per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("inner_sea_taverns", "class_features", "Gap: 11 Inner Sea Taverns class_feature records are ingested corpus-wide (first corpus JSON this book has ever had, any kind). No per-class mechanism wiring has landed for this book's classes yet. Remedy: same as ACG above."),
    ("bestiary_6", "spells", "Gap: Bestiary 6's own `rules_tables::bestiary_6::spell_list` table is real (2 records, transcribed from `b6_spells.lst`, byte-verified) and IS chained into `spell_resolver::spell_catalog_rows()` (SD-31 wave 24), but both of its rows are verbatim reprints of spells Ultimate Wilderness already ships (same `DESC:`, same Bestiary-6 `SOURCEPAGE:` citation inside `uw_spells.lst`). The resolver's cross-book dedup pass (added this same cycle to protect the pre-existing `no_key_is_served_twice_so_a_selection_resolves_unambiguously` product invariant in `apps/desktop/src-tauri/src/spell_catalog.rs`) keeps only the first-chained book's copy -- Ultimate Wilderness, registered in wave 19 -- so no row ever carries `book==\"B6\"` in the SERVED catalog, even though the content reaches a player under UW's own book label. Remedy: a real cross-book-reprint crediting design (Decision 10's Supersession Register, proposed but not applied, is the natural home for this policy question) so a book whose own content is verbatim-duplicated elsewhere can still claim its own reach without double-serving the catalog. See docs/release/SD-31-corpus-closure-grind/artifacts/BESTIARY-6-LEDGER.md."),
    ("beastiary1", "monster_abilities", "Gap: 180 of Bestiary 1's 709 `monster_ability` records (`decisions.md §20`, no_record-to-zero wave 2) ship with `owners: &[]` -- no monster row of this book claims them, because they are the corpus's own shared reference-library vocabulary (`Universal Monster Rule ~ X`, `Vampire ~ X`, `Lich ~ X`, `Regeneration ~ X`, `Immunity to X`, and similar generic special-ability text `scripts/transcribe_monster_tables.py`'s ownership pass finds no in-book stat block naming). They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: the same one `decisions.md §16`'s own guard names for the rest of this cycle's residual `monster_ability` population -- a per-record read of each shared-vocabulary entry to determine its REAL owning creature-type set (a Universal Monster Rule applies to every creature of a stated type, not to one specific stat block), which is domain content work, not a mechanism this cycle's generic ingest pass can close."),
    ("bestiary_2", "monster_abilities", "Gap: 85 of Bestiary 2's 656 `monster_ability` records (`decisions.md §20`, no_record-to-zero wave 2 follow-on) ship with `owners: &[]` -- no monster row of this book claims them, the identical shared reference-library / cross-file-namespaced-orphan shape `beastiary1`'s matching entry above describes, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied there. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: the same one `decisions.md §16`'s own guard names for the rest of this cycle's residual `monster_ability` population -- a per-record read of each shared-vocabulary entry to determine its REAL owning creature-type set, which is domain content work, not a mechanism this cycle's generic ingest pass can close."),
    ("bestiary_3", "monster_abilities", "Gap: 276 of Bestiary 3's 686 `monster_ability` records (10 added `decisions.md §27`/round 8, the `TYPE:`-facet-vocabulary-gap group's provisional-default closure) (`decisions.md §20`, no_record-to-zero wave 2 follow-on) ship with `owners: &[]` -- no monster row of this book claims them, the identical shared reference-library / cross-file-namespaced-orphan shape `beastiary1`'s matching entry above describes, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied there. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: the same one `decisions.md §16`'s own guard names for the rest of this cycle's residual `monster_ability` population -- a per-record read of each shared-vocabulary entry to determine its REAL owning creature-type set, which is domain content work, not a mechanism this cycle's generic ingest pass can close."),
    ("bestiary_4", "monster_abilities", "Gap: 187 of Bestiary 4's 806 `monster_ability` records (`decisions.md §20`, no_record-to-zero wave 2 follow-on) ship with `owners: &[]` -- no monster row of this book claims them, the identical shared reference-library / cross-file-namespaced-orphan shape `beastiary1`'s matching entry above describes, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied there. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: the same one `decisions.md §16`'s own guard names for the rest of this cycle's residual `monster_ability` population -- a per-record read of each shared-vocabulary entry to determine its REAL owning creature-type set, which is domain content work, not a mechanism this cycle's generic ingest pass can close."),
    ("horror_adventures", "monster_abilities", "Gap: 56 of Horror Adventures's 62 `monster_ability` records (`decisions.md §20`, no_record-to-zero wave 2 follow-on) ship with `owners: &[]` -- no monster row of this book claims them, the identical shared reference-library / cross-file-namespaced-orphan shape `beastiary1`'s matching entry above describes, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied there. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: the same one `decisions.md §16`'s own guard names for the rest of this cycle's residual `monster_ability` population -- a per-record read of each shared-vocabulary entry to determine its REAL owning creature-type set, which is domain content work, not a mechanism this cycle's generic ingest pass can close."),
    ("inner_sea_bestiary", "monster_abilities", "Gap: 35 of Inner Sea Bestiary's 187 `monster_ability` records (`decisions.md §20`, no_record-to-zero wave 2 follow-on) ship with `owners: &[]` -- no monster row of this book claims them, the identical shared reference-library / cross-file-namespaced-orphan shape `beastiary1`'s matching entry above describes, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied there. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: the same one `decisions.md §16`'s own guard names for the rest of this cycle's residual `monster_ability` population -- a per-record read of each shared-vocabulary entry to determine its REAL owning creature-type set, which is domain content work, not a mechanism this cycle's generic ingest pass can close."),
    ("inner_sea_gods", "monster_abilities", "Gap: 5 of Inner Sea Gods's 163 `monster_ability` records (`decisions.md §20`, no_record-to-zero wave 2 follow-on) ship with `owners: &[]` -- no monster row of this book claims them, the identical shared reference-library / cross-file-namespaced-orphan shape `beastiary1`'s matching entry above describes, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied there. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: the same one `decisions.md §16`'s own guard names for the rest of this cycle's residual `monster_ability` population -- a per-record read of each shared-vocabulary entry to determine its REAL owning creature-type set, which is domain content work, not a mechanism this cycle's generic ingest pass can close."),
    ("inner_sea_world_guide", "monster_abilities", "Gap: 16 of Inner Sea World Guide's 30 `monster_ability` records (`decisions.md §20`, no_record-to-zero wave 2 follow-on) ship with `owners: &[]` -- no monster row of this book claims them, the identical shared reference-library / cross-file-namespaced-orphan shape `beastiary1`'s matching entry above describes, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied there. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: the same one `decisions.md §16`'s own guard names for the rest of this cycle's residual `monster_ability` population -- a per-record read of each shared-vocabulary entry to determine its REAL owning creature-type set, which is domain content work, not a mechanism this cycle's generic ingest pass can close."),
    ("ultimate_psionics", "monster_abilities", "Gap: 64 of Ultimate Psionics's 191 `monster_ability` records (`decisions.md §20`, no_record-to-zero wave 2 follow-on) ship with `owners: &[]` -- no monster row of this book claims them, the identical shared reference-library / cross-file-namespaced-orphan shape `beastiary1`'s matching entry above describes, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied there. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: the same one `decisions.md §16`'s own guard names for the rest of this cycle's residual `monster_ability` population -- a per-record read of each shared-vocabulary entry to determine its REAL owning creature-type set, which is domain content work, not a mechanism this cycle's generic ingest pass can close."),
    ("ultimate_wilderness", "monster_abilities", "Gap: all 2 of Ultimate Wilderness's `monster_ability` records (`decisions.md §20` no_record-to-zero, round 3) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."),
    ("ultimate_intrigue", "monster_abilities", "Gap: all 6 of Ultimate Intrigue's `monster_ability` records (`decisions.md §20` no_record-to-zero, round 3) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."),
    ("ultimate_magic", "monster_abilities", "Gap: all 13 of Ultimate Magic's `monster_ability` records (`decisions.md §20` no_record-to-zero, round 3) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."),
    ("bestiary_6", "monster_abilities", "Gap: all 16 of Bestiary 6's `monster_ability` records (`decisions.md §20` no_record-to-zero, round 3) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."),
    ("bestiary_5", "monster_abilities", "Gap: all 40 of Bestiary 5's `monster_ability` records (39 `decisions.md §20` no_record-to-zero round 3, +1 `decisions.md §27b` round 9 -- `Traits Output ~ Sahkil`, closed via `parse_desc`'s new generalised sixth branch) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."),
    ("pathfinder_unchained", "monster_abilities", "Gap: all 72 of Pathfinder Unchained's `monster_ability` records (69 `decisions.md §20` no_record-to-zero round 4, +3 `decisions.md §27b` round 9 -- `Elemental ~ Unchained Eidolon LVL01/08/20`, the multi-DESC: shape `parse_desc` used to refuse, closed via its new generalised sixth branch) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry, reached this round via the book's own `gen_pathfinder_unchained()` generator function extended to also call `gen_monster_book`. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."),
    ("advanced_race_guide", "monster_abilities", "Gap: the 1 `monster_ability` record Advanced Race Guide's `arg_abilities_race.lst` contributes (`decisions.md §20` no_record-to-zero, round 4) ships with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own it, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry, reached this round via the book's own `gen_advanced_race_guide()` generator function extended to also call `gen_monster_book`. It is shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."),
    ("mythic_adventures", "monster_abilities", "Gap: all 21 of Mythic Adventures's `monster_ability` records (`decisions.md §20` no_record-to-zero, round 5) ship with `owners: &[]` -- this book has ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s \"ZERO-monster books\" line), so nothing can ever own an ability row, closed by the SAME generic mechanism (`scripts/transcribe_monster_tables.py`'s orphan pass) already applied to every other book in this registry, reached entirely through `gen_book_cache.rs`'s generic `monster_book_spec` fallback arm -- this book carries no hand-rolled generator function, unlike round 4's `pathfinder_unchained`/`advanced_race_guide`. All 21 of the book's orphan candidates shipped -- 0 refused, unlike round 4's `pathfinder_unchained` multi-DESC: residual. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: none needed -- this is the terminal state for a zero-monster book's ability rows; nothing can ever own them, so no further per-record work applies."),
    ("occult_adventures", "monster_abilities", "Gap: all 5 of Occult Adventures's `monster_ability` records (`decisions.md §27b` — EVERYTHING, overturning four cycles' worth of \"correctly out of scope\" for a negated `!PRECAMPAIGN:1,INCLUDES=Bestiary 3` gate this repo's campaign set fails, a REACHABILITY finding, not an ingest exemption) ship with `owners: &[]` -- no monster row in this generator's ownership pass claims any of the 5 by name (the two owning race rows reference them only via a CATEGORY:Internal umbrella row this generator does not resolve into per-record ownership: `Race Traits ~ Homunculus Companion` names 2 of the 3 Homunculus rows, `Poison` is not named at all; `Racial Traits ~ Kami (Shikigami)` grants by TYPE=, not by name), the identical shape every other zero-record-owner book in this registry already ships. They are shipped anyway, deliberately, because an un-ingested row's shape cannot be measured and Gate 1's DoD needs every unit's shape measured (`decisions.md §20`/`§27b`); `list_monster_catalog` only ever walks a monster's own `ability_keys` (`monster_catalog.rs`), so an owner-less record reaches no screen -- not a stub (a stub is a record a player's screen SHOWS empty; this reaches no screen at all), and its non-reach is proven and pinned by exact key in `UNREACHED_RECORD_FINDINGS` above, never assumed. Remedy: a per-record trace of each umbrella row's own grant logic (named references plus TYPE= auto-grants) to determine real ownership, which is domain content work, not a mechanism this cycle's generic ingest pass can close."),
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
const BARE_RECORD_FINDINGS: &[(&str, &str, &[&str])] = &[
    // SD-32 `decisions.md §25` cycle 2/3 (2026-08-23): `Adopted Race ~
    // Rougarou`, the one target race of the 14 whose Trait pool is
    // genuinely, corpus-wide empty. `trait_pool::load_trait_pool` finds real
    // `kind: trait` content (`data/corpus/*/trait_generic/`, no fallback --
    // that module's own doc comment) for the other 13, so this is the only
    // entry: re-confirms `epic-6-kind-trait_cycle-1_cycle_receipt.md §1`'s own
    // "Rougarou has zero hits corpus-wide" finding at the reach-gate layer,
    // and the pinned oracle itself types Rougarou's own `AdoptiveRace` row
    // `TYPE:Rougarou Race Trait|CATEGORY=Special Ability|No Race Trait
    // Available.MOD` (`rougarou_abilities_race.lst:30`) -- a hard
    // impossibility of source data (`decisions.md §27b`), not a gap. See
    // `race_trait_picker.rs`'s own
    // `the_menu_command_carries_all_fourteen_adopted_race_options_thirteen_
    // with_real_grants`.
    ("bestiary_6", "race_traits", &["Adopted Race ~ Rougarou"]),
];

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
/// it is a finding. A family with a *partial* shortfall is a different case:
/// the claim is declared, it returns [`Reach::NotSurfaced`], the family stays a
/// written finding, and the exact shortfall is pinned here. The property that
/// matters is preserved in both directions:
///
/// * another record that stops reaching changes this set and fails;
/// * fixing one of these fails too, until its key is deleted.
///
/// **Two race-trait entries have been retired by being fixed, not
/// reclassified**, and both are worth recording because they are the two ways
/// this list is supposed to shrink:
///
/// * `advanced_race_guide/race_traits`, at 154 of 156. Its two stragglers —
///   `Feral ~ Languages` and `Scion of Humanity ~ Languages` — now arrive
///   through `race_resolver`'s reading of the `ABILITY:<cat>|AUTOMATIC|<key>`
///   grant shape. Fixed by **code**: the resolver learned a grant shape it had
///   been ignoring.
/// * `beastiary1/race_traits`, at 107 of 108 (retired 2026-08-11, SD-29 Epic 6
///   pilot, `decisions.md §43`). Its one straggler,
///   `Duergar ~ Spell-Like Ability ~ Invisibility`, is gated on a positive
///   `PREFACT` naming `Duergar_ReplaceSLAEnlargePerson`, and the only row in
///   the whole upstream corpus that sets that flag is `Duergar ~ Ironskinned`
///   in `monster_codex/mc_abilities_race.lst:16`. Fixed by **data**: nothing
///   about the resolver changed, Monster Codex's racial traits were ingested
///   and the row became reachable by a selection a player can really make.
///   That is the shape this list was designed to distinguish from a wiring gap,
///   and it took an ingest rather than a screen to close.
///   `tests/duergar_invisibility_sla_reaches_a_player_via_monster_codex.rs`
///   holds it closed in both directions.
const UNREACHED_RECORD_FINDINGS: &[(&str, &str, &[&str])] = &[
    (
        "monster_codex",
        "race_traits",
        // A Goblin *variant* selector, not a swap: no replace flag, no
        // positive gate, so it is `TraitRole::Unclassified` and no selection
        // reaches it. Remedy in OPEN_FINDINGS above.
        &["Oversized Goblin"],
    ),
    (
        "inner_sea_races",
        "race_traits",
        // The replacement half of the `Human ~ Tribalistic` alternate. Its own
        // row (`isr_abilities_race.lst:216`) carries no gate of any kind and no
        // other row in the book names it, so it is `TraitRole::Unclassified`
        // and no selection reaches it. This is an upstream data gap, not a
        // wiring gap: selecting `Human ~ Tribalistic` correctly suppresses the
        // standard `Human ~ Languages` row, and nothing brings this one in to
        // take its place. Remedy in OPEN_FINDINGS above.
        //
        // The three `Mostly Human ~ <Race> ~ Languages` rows below are a
        // DIFFERENT shape of the same outcome, added by SD-31 Epic 1-F2
        // (2026-08-15).
        //
        // CORRECTED 2026-08-15 (`SD31-W2-INTEGRATE-001`, Finding 8; the
        // original comment below the correction is left visible per this
        // program's doc convention, and is WRONG about upstream). PCGen DOES
        // ship a granter for all four races (Ifrit/Oread/Sylph/Undine)
        // symmetrically: `isr_abilities_race.lst:650/651/652/653` are
        // `Geneiekin ~ Mostly Human.MOD` rows, each carrying
        // `FACT:<Race>_ReplaceLanguages|True` AND its own
        // `ABILITY:<Race> Racial Trait|AUTOMATIC|Mostly Human ~ <Race> ~
        // Languages|PREFACT:...` grant. Upstream is complete; the gap is
        // entirely project-side. The BASE row (`isr_abilities_race.lst:649`,
        // `Geneiekin ~ Mostly Human`) carries
        // `TYPE:RacialTraits.SpecialQuality.Special Quality` with no
        // race-scoped TYPE component, so `parse_row` never emits a
        // `<Race> Racial Trait` classification target for it -- and the
        // per-race grants live ONLY on the four `.MOD` rows, which
        // `is_mod_row` deliberately excludes from ingestion
        // (`ingest_race_traits.rs`'s own test doc already names these exact
        // rows, filtered "one step later" because "those 5 name races this
        // project does not model" -- a clause SD-31 Epic 1 made false for
        // 4 of the 5). Oread's sibling row reaches purely because
        // `Oread ~ Isolated` (a second, UNRELATED Inner Sea Races row) also
        // happens to set `Oread_ReplaceLanguages` -- that is an accident of
        // a second setter, not proof the other three lack one; the Geneiekin
        // path itself is unmodelled for all four. Concrete remedy: model the
        // Geneiekin heritage the way Aasimar/Tiefling already are (same
        // class of work as the already-deferred Dhampir/Skinwalker
        // heritage build, `OPEN-ISSUES.md` row 13) -- tracked as a follow-on
        // Epic 1 card, `OPEN-ISSUES.md` row 18, not an accepted permanent
        // gap.
        //
        // ORIGINAL (WRONG) COMMENT, preserved for the record: each row
        // carries a *positive* `PREFACT:1,ABILITIES,
        // <Race>_ReplaceLanguages=True`, so `race_resolver::classify` reads
        // them as `TraitRole::FlagGranted`, not `Unclassified` -- but no
        // Ifrit/Sylph/Undine alternate in either ARG or Inner Sea Races sets
        // the `<Race>_ReplaceLanguages` flag that would grant them (verified:
        // grep every `sets_replace_flags` value for these three races across
        // both books' ingested records -- none contains it). Oread's sibling
        // row (`Mostly Human ~ Oread ~ Languages`) DOES reach, because
        // `Oread ~ Isolated` (Inner Sea Races) sets `Oread_ReplaceLanguages` --
        // the presence of that one working case is what proves this is a real
        // upstream content gap for the other three, not a resolver defect: the
        // exact same mechanism reaches Oread's row and only Oread's.
        // [This inference is backwards -- see the correction above.]
        //
        // 79 of the book's 94 records reach a player; these five plus
        // `Human ~ Tribalistic Languages` are the six that do not.
        //
        // `Mostly Human ~ Suli ~ Languages` added 2026-08-22 (SD-32 card 11
        // T2b lane): its own row (`isr_abilities_race.lst:662`) was never
        // ingested before this lane re-ran the book (the same stale-regen
        // gap the count-pinning test above documents), and once ingested it
        // is unreached for the identical reason as its Ifrit/Sylph/Undine
        // siblings -- `isr_abilities_race.lst:654`'s `Geneiekin ~ Mostly
        // Human.MOD` granter for Suli is the same unmodelled-Geneiekin-
        // heritage shape, confirmed against the pinned oracle.
        //
        // `Suli ~ Trusted Mediator` added the same lane: its own row
        // (`isr_abilities_race.lst:1266`) carries a self-exclusion
        // `PREMULT` gate naming `Suli_ReplaceEnergyResistance`/
        // `Suli_ReplaceVision`, and its DESC prose states "This racial
        // trait replaces energy resistance and low-light vision" -- but
        // unlike every structurally identical sibling row (`Ifrit ~ Brazen
        // Flame`:823, `Oread ~ Isolated`:1010, `Sylph ~ Secretive`:1320,
        // each of which sets its own matching `FACT:<Race>_Replace...
        // |True` pair), this row sets **no** `FACT:` token at all. Verified
        // against the pinned oracle: a genuine upstream PCGen data
        // omission for Suli specifically, not a project-side parsing gap --
        // the same shape as `Human ~ Tribalistic Languages` above.
        &[
            "Human ~ Tribalistic Languages",
            "Mostly Human ~ Ifrit ~ Languages",
            "Mostly Human ~ Suli ~ Languages",
            "Mostly Human ~ Sylph ~ Languages",
            "Mostly Human ~ Undine ~ Languages",
            "Suli ~ Trusted Mediator",
        ],
    ),
    // SD28-C4.8/§60/§63: all 406 archetype-swap records across 7 books (403 + 3 Slayer archetypes, SD31-E4-F1-001) --
    // every key, because none reaches a player through any surface today
    // (no picker exists at all, see OPEN_FINDINGS). This is the "whole
    // family unreached" shape, not a partial shortfall.
    (
        "acg",
        "archetypes",
        &[
            "Alchemist Archetype ~ Inspired Chemist",
            "Arcanist Archetype ~ Blade Adept",
            "Arcanist Archetype ~ Blood Arcanist",
            "Arcanist Archetype ~ Brown-Fur Transmuter",
            "Arcanist Archetype ~ Eldritch Font",
            "Arcanist Archetype ~ Elemental Master",
            "Arcanist Archetype ~ Occultist",
            "Arcanist Archetype ~ School Savant",
            "Arcanist Archetype ~ Spell Specialist",
            "Arcanist Archetype ~ White Mage",
            "Bard Archetype ~ Flame Dancer",
            "Bard Archetype ~ Voice of the Wild",
            "Bloodrager Archetype ~ Blood Conduit",
            "Bloodrager Archetype ~ Bloodrider",
            "Bloodrager Archetype ~ Greenrager",
            "Bloodrager Archetype ~ Metamagic Rager",
            "Bloodrager Archetype ~ Rageshaper",
            "Bloodrager Archetype ~ Spelleater",
            "Bloodrager Archetype ~ Steelblood",
            "Brawler Archetype ~ Exemplar",
            "Brawler Archetype ~ Mutagenic Mauler",
            "Brawler Archetype ~ Shield Champion",
            "Brawler Archetype ~ Snakebite Striker",
            "Brawler Archetype ~ Steel-Breaker",
            "Brawler Archetype ~ Strangler",
            "Brawler Archetype ~ Wild Child",
            "Cavalier Archetype ~ Daring Champion",
            "Cleric Archetype ~ Ecclesitheurge",
            "Druid Archetype ~ Feral Shifter",
            "Druid Archetype ~ Nature Fang",
            "Druid Archetype ~ Wild Whisperer",
            "Fighter Archetype ~ Martial Master",
            "Fighter Archetype ~ Mutation Warrior",
            "Hunter Archetype ~ Divine Hunter",
            "Hunter Archetype ~ Feral Hunter",
            "Hunter Archetype ~ Packmaster",
            "Hunter Archetype ~ Primal Companion Hunter",
            "Hunter Archetype ~ Verminous Hunter",
            "Inquisitor Archetype ~ Sacred Huntsmaster",
            "Inquisitor Archetype ~ Sanctified Slayer",
            "Investigator Archetype ~ Empiricist",
            "Investigator Archetype ~ Infiltrator",
            "Investigator Archetype ~ Mastermind",
            "Investigator Archetype ~ Sleuth",
            "Investigator Archetype ~ Spiritualist",
            "Investigator Archetype ~ Steel Hound",
            "Monk Archetype ~ Kata Master",
            "Monk Archetype ~ Wildcat",
            "Oracle Archetype ~ Psychic Searcher",
            "Oracle Archetype ~ Spirit Guide",
            "Oracle Archetype ~ Warsighted",
            "Paladin Archetype ~ Holy Guide",
            "Paladin Archetype ~ Temple Champion",
            "Ranger Archetype ~ Divine Tracker",
            "Ranger Archetype ~ Hooded Champion",
            "Ranger Archetype ~ Wild Hunter",
            "Rogue Archetype ~ Counterfeit Mage",
            "Rogue Archetype ~ Underground Chemist",
            "Shaman Archetype ~ Animist",
            "Shaman Archetype ~ Possessed Shaman",
            "Shaman Archetype ~ Speaker for the Past",
            "Shaman Archetype ~ Spirit Warden",
            "Shaman Archetype ~ Unsworn Shaman",
            "Shaman Archetype ~ Visionary",
            "Shaman Archetype ~ Witch Doctor",
            "Skald Archetype ~ Fated Champion",
            "Skald Archetype ~ Herald of the Horn",
            "Skald Archetype ~ Spell Warrior",
            "Skald Archetype ~ Totemic Skald",
            // SD31-E4-F1-001 (2026-08-16): Slayer's own archetype block, added
            // to acg::archetype_tables this same cycle -- same "whole family
            // unreached" shape as every other entry here (no archetype picker
            // exists anywhere in the desktop app yet, see OPEN_FINDINGS's
            // "acg"/"archetypes" entry above).
            "Slayer Archetype ~ Bounty Hunter",
            "Slayer Archetype ~ Deliverer",
            "Slayer Archetype ~ Stygian Slayer",
            "Sorcerer Archetype ~ Eldritch Scrapper",
            "Sorcerer Archetype ~ Mongrel Mage",
            "Summoner Archetype ~ Naturalist",
            "Swashbuckler Archetype ~ Daring Infiltrator",
            "Swashbuckler Archetype ~ Flying Blade",
            "Swashbuckler Archetype ~ Inspired Blade",
            "Swashbuckler Archetype ~ Mouser",
            "Swashbuckler Archetype ~ Musketeer",
            "Swashbuckler Archetype ~ Mysterious Avenger",
            "Swashbuckler Archetype ~ Picaroon",
            "Warpriest Archetype ~ Champion of the Faith",
            "Warpriest Archetype ~ Cult Leader",
            "Warpriest Archetype ~ Disenchanter",
            "Warpriest Archetype ~ Divine Commander",
            "Warpriest Archetype ~ Forgepriest",
            "Warpriest Archetype ~ Sacred Fist",
            "Witch Archetype ~ Hex Channeler",
            "Witch Archetype ~ Mountain Witch",
        ],
    ),
    (
        "advanced_race_guide",
        "archetypes",
        &[
            "Alchemist Archetype ~ Bogborn Alchemist",
            "Alchemist Archetype ~ Bramble Brewer",
            "Alchemist Archetype ~ Deep Bomber",
            "Alchemist Archetype ~ Fire Bomber",
            "Alchemist Archetype ~ Plague Bringer",
            "Alchemist Archetype ~ Saboteur",
            "Barbarian Archetype ~ Feral Gnasher",
            "Barbarian Archetype ~ Hateful Rager",
            "Bard Archetype ~ Prankster",
            "Bard Archetype ~ Shadow Puppeteer",
            "Bard Archetype ~ Watersinger",
            "Cleric Archetype ~ Demonic Apostle",
            "Cleric Archetype ~ Fiendish Vessel",
            "Cleric Archetype ~ Forgemaster",
            "Druid Archetype ~ Feral Child",
            "Druid Archetype ~ Naga Aspirant",
            "Druid Archetype ~ Sky Druid",
            "Druid Archetype ~ Treesinger",
            "Druid Archetype ~ Undine Adept",
            "Fighter Archetype ~ Airborne Ambusher",
            "Fighter Archetype ~ Cavern Sniper",
            "Fighter Archetype ~ Dirty Fighter",
            "Fighter Archetype ~ Foehammer",
            "Inquisitor Archetype ~ Exarch",
            "Inquisitor Archetype ~ Immolator",
            "Inquisitor Archetype ~ Kinslayer",
            "Monk Archetype ~ Gray Disciple",
            "Monk Archetype ~ Ironskin Monk",
            "Monk Archetype ~ Nimble Guardian",
            "Monk Archetype ~ Student of Stone",
            "Monk Archetype ~ Treetop Monk",
            "Monk Archetype ~ Underfoot Adept",
            "Monk Archetype ~ Wanderer",
            "Oracle Archetype ~ Ancient Lorekeeper",
            "Oracle Archetype ~ Community Guardian",
            "Oracle Archetype ~ Purifier",
            "Oracle Archetype ~ Reincarnated Oracle",
            "Oracle Archetype ~ Shigenjo",
            "Paladin Archetype ~ Redeemer",
            "Paladin Archetype ~ Stonelord",
            "Paladin Archetype ~ Tranquil Guardian",
            "Ranger Archetype ~ Dusk Stalker",
            "Ranger Archetype ~ Wave Warden",
            "Ranger Archetype ~ Wild Shadow",
            "Rogue Archetype ~ Cat Burglar",
            "Rogue Archetype ~ Deadly Courtesan",
            "Rogue Archetype ~ Eldritch Raider",
            "Rogue Archetype ~ Filcher",
            "Rogue Archetype ~ Kitsune Trickster",
            "Rogue Archetype ~ Skulking Slayer",
            "Rogue Archetype ~ Swordmaster",
            "Summoner Archetype ~ Blood God Disciple",
            "Summoner Archetype ~ Shaitan Binder",
            "Witch Archetype ~ Bonded Witch",
            "Witch Archetype ~ Dreamweaver",
            "Witch Archetype ~ Scarred Witch Doctor",
            "Wizard Archetype ~ Cruoromancer",
            "Wizard Archetype ~ Spellbinder",
            "Wizard Archetype ~ Wind Listener",
        ],
    ),
    (
        "apg",
        "archetypes",
        &[
            "Barbarian Archetype ~ Breaker",
            "Barbarian Archetype ~ Brutal Pugilist",
            "Barbarian Archetype ~ Drunken Brute",
            "Barbarian Archetype ~ Elemental Kin",
            "Barbarian Archetype ~ Hurler",
            "Barbarian Archetype ~ Invulnerable Rager",
            "Barbarian Archetype ~ Mounted Fury",
            "Barbarian Archetype ~ Savage Barbarian",
            "Barbarian Archetype ~ Superstitious",
            "Bard Archetype ~ Arcane Duelist",
            "Bard Archetype ~ Archivist",
            "Bard Archetype ~ Court Bard",
            "Bard Archetype ~ Detective",
            "Bard Archetype ~ Magician",
            "Bard Archetype ~ Sandman",
            "Bard Archetype ~ Savage Skald",
            "Bard Archetype ~ Sea Singer",
            "Bard Archetype ~ Street Performer",
            "Druid Archetype ~ Aquatic Druid",
            "Druid Archetype ~ Arctic Druid",
            "Druid Archetype ~ Bear Shaman",
            "Druid Archetype ~ Blight Druid",
            "Druid Archetype ~ Cave Druid",
            "Druid Archetype ~ Desert Druid",
            "Druid Archetype ~ Eagle Shaman",
            "Druid Archetype ~ Jungle Druid",
            "Druid Archetype ~ Lion Shaman",
            "Druid Archetype ~ Mountain Druid",
            "Druid Archetype ~ Plains Druid",
            "Druid Archetype ~ Serpent Shaman",
            "Druid Archetype ~ Swamp Druid",
            "Druid Archetype ~ Urban Druid",
            "Druid Archetype ~ Wolf Shaman",
            "Fighter Archetype ~ Archer",
            "Fighter Archetype ~ Crossbowman",
            "Fighter Archetype ~ Free Hand Fighter",
            "Fighter Archetype ~ Mobile Fighter",
            "Fighter Archetype ~ Phalanx Soldier",
            "Fighter Archetype ~ Polearm Master",
            "Fighter Archetype ~ Roughrider",
            "Fighter Archetype ~ Savage Warrior",
            "Fighter Archetype ~ Shielded Fighter",
            "Fighter Archetype ~ Two-Handed Fighter",
            "Fighter Archetype ~ Two-Weapon Warrior",
            "Fighter Archetype ~ Weapon Master",
            "Monk Archetype ~ Drunken Master",
            "Monk Archetype ~ Hungry Ghost Monk",
            "Monk Archetype ~ Ki Mystic",
            "Monk Archetype ~ Monk of the Empty Hand",
            "Monk Archetype ~ Monk of the Four Winds",
            "Monk Archetype ~ Monk of the Healing Hand",
            "Monk Archetype ~ Monk of the Lotus",
            "Monk Archetype ~ Monk of the Sacred Mountain",
            "Monk Archetype ~ Weapon Adept",
            "Paladin Archetype ~ Divine Defender",
            "Paladin Archetype ~ Hospitaler",
            "Paladin Archetype ~ Sacred Servant",
            "Paladin Archetype ~ Shining Knight",
            "Paladin Archetype ~ Undead Scourge",
            "Paladin Archetype ~ Warrior of the Holy Light",
            "Ranger Archetype ~ Beast Master",
            "Ranger Archetype ~ Guide",
            "Ranger Archetype ~ Horse Lord",
            "Ranger Archetype ~ Infiltrator",
            "Ranger Archetype ~ Shapeshifter",
            "Ranger Archetype ~ Skirmisher",
            "Ranger Archetype ~ Spirit Ranger",
            "Ranger Archetype ~ Urban Ranger",
            "Rogue Archetype ~ Acrobat",
            "Rogue Archetype ~ Burglar",
            "Rogue Archetype ~ Cutpurse",
            "Rogue Archetype ~ Investigator",
            "Rogue Archetype ~ Poisoner",
            "Rogue Archetype ~ Rake",
            "Rogue Archetype ~ Scout",
            "Rogue Archetype ~ Sniper",
            "Rogue Archetype ~ Spy",
            "Rogue Archetype ~ Swashbuckler",
            "Rogue Archetype ~ Thug",
            "Rogue Archetype ~ Trapsmith",
        ],
    ),
    (
        "ultimate_combat",
        "archetypes",
        &[
            "Alchemist Archetype ~ Beastmorph",
            "Alchemist Archetype ~ Ragechemist",
            "Barbarian Archetype ~ Armored Hulk",
            "Barbarian Archetype ~ Scarred Rager",
            "Barbarian Archetype ~ Sea Reaver",
            "Barbarian Archetype ~ Titan Mauler",
            "Barbarian Archetype ~ True Primitive",
            "Barbarian Archetype ~ Urban Barbarian",
            "Barbarian Archetype ~ Wild Rager",
            "Bard Archetype ~ Archaeologist",
            "Bard Archetype ~ Daredevil",
            "Bard Archetype ~ Dervish Dancer",
            "Cavalier Archetype ~ Beast Rider",
            "Cavalier Archetype ~ Emissary",
            "Cavalier Archetype ~ Gendarme",
            "Cavalier Archetype ~ Honor Guard",
            "Cavalier Archetype ~ Luring Cavalier",
            "Cavalier Archetype ~ Musketeer",
            "Cavalier Archetype ~ Standard Bearer",
            "Cavalier Archetype ~ Strategist",
            "Cleric Archetype ~ Crusader",
            "Cleric Archetype ~ Divine Strategist",
            "Cleric Archetype ~ Evangelist",
            "Cleric Archetype ~ Merciful Healer",
            "Druid Archetype ~ Ape Shaman",
            "Druid Archetype ~ Bat Shaman",
            "Druid Archetype ~ Boar Shaman",
            "Druid Archetype ~ World Walker",
            "Fighter Archetype ~ Armor Master",
            "Fighter Archetype ~ Brawler",
            "Fighter Archetype ~ Cad",
            "Fighter Archetype ~ Dragoon",
            "Fighter Archetype ~ Gladiator",
            "Fighter Archetype ~ Tactician",
            "Fighter Archetype ~ Thunderstriker",
            "Fighter Archetype ~ Tower Shield Specialist",
            "Fighter Archetype ~ Unarmed Fighter",
            "Fighter Archetype ~ Unbreakable",
            // SD31-E4-F1-002: the first 2 of Gunslinger's own 4 archetypes,
            // added to `archetype_tables.rs` this cycle. Still `NotSurfaced`
            // (no archetype-selection surface exists, same as every other
            // record in this list) -- pinned here so the reach test does not
            // read their absence as a NEW, unrecorded regression.
            "Gunslinger Archetype ~ Mysterious Stranger",
            "Gunslinger Archetype ~ Pistolero",
            "Inquisitor Archetype ~ Iconoclast",
            "Inquisitor Archetype ~ Spellbreaker",
            "Inquisitor Archetype ~ Witch Hunter",
            // SD31-E4-F1-003: Ninja's one real archetype, added to
            // `archetype_tables.rs` this cycle. Still `NotSurfaced` (no
            // archetype-selection surface exists, same as every other
            // record in this list) -- pinned here so the reach test does
            // not read its absence as a NEW, unrecorded regression.
            "Ninja Archetype ~ Scout",
            "Paladin Archetype ~ Divine Hunter",
            "Paladin Archetype ~ Empyreal Knight",
            "Paladin Archetype ~ Holy Gun",
            "Paladin Archetype ~ Holy Tactician",
            "Paladin Archetype ~ Knight Of The Sepulcher",
            "Paladin Archetype ~ Sacred Shield",
            "Ranger Archetype ~ Battle Scout",
            "Ranger Archetype ~ Deep Walker",
            "Ranger Archetype ~ Falconer",
            "Ranger Archetype ~ Trophy Hunter",
            "Ranger Archetype ~ Warden",
            "Ranger Archetype ~ Wild Stalker",
            "Rogue Archetype ~ Bandit",
            "Rogue Archetype ~ Chameleon",
            "Rogue Archetype ~ Charlatan",
            "Rogue Archetype ~ Driver",
            "Rogue Archetype ~ Knife Master",
            "Rogue Archetype ~ Pirate",
            "Rogue Archetype ~ Roof Runner",
            "Rogue Archetype ~ Sanctified Rogue",
            "Rogue Archetype ~ Survivalist",
            "Wizard Archetype ~ Arcane Bomber",
            "Wizard Archetype ~ Siege Mage",
            "Wizard Archetype ~ Spellslinger",
        ],
    ),
    (
        "ultimate_magic",
        "archetypes",
        &[
            "Alchemist Archetype ~ Chirurgeon",
            "Alchemist Archetype ~ Clone Master",
            "Alchemist Archetype ~ Internal Alchemist",
            "Alchemist Archetype ~ Mindchemist",
            "Alchemist Archetype ~ Preservationist",
            "Alchemist Archetype ~ Psychonaut",
            "Alchemist Archetype ~ Reanimator",
            "Alchemist Archetype ~ Vivisectionist",
            "Bard Archetype ~ Animal Speaker",
            "Bard Archetype ~ Celebrity",
            "Bard Archetype ~ Demagogue",
            "Bard Archetype ~ Dirge Bard",
            "Bard Archetype ~ Geisha",
            "Bard Archetype ~ Songhealer",
            "Bard Archetype ~ Sound Striker",
            "Cleric Archetype ~ Cloistered Cleric",
            "Cleric Archetype ~ Separatist",
            "Cleric Archetype ~ Theologian",
            "Cleric Archetype ~ Undead Lord",
            "Druid Archetype ~ Dragon Shaman",
            "Druid Archetype ~ Menhir Savant",
            "Druid Archetype ~ Mooncaller",
            "Druid Archetype ~ Pack Lord",
            "Druid Archetype ~ Reincarnated Druid",
            "Druid Archetype ~ Saurian Shaman",
            "Druid Archetype ~ Shark Shaman",
            "Druid Archetype ~ Storm Druid",
            "Inquisitor Archetype ~ Exorcist",
            "Inquisitor Archetype ~ Heretic",
            "Inquisitor Archetype ~ Infiltrator",
            "Inquisitor Archetype ~ Preacher",
            "Inquisitor Archetype ~ Sin Eater",
            "Monk Archetype ~ Qinggong Monk Abundant Step",
            "Monk Archetype ~ Qinggong Monk Diamond Body",
            "Monk Archetype ~ Qinggong Monk Diamond Soul",
            "Monk Archetype ~ Qinggong Monk Empty Body",
            "Monk Archetype ~ Qinggong Monk High Jump",
            "Monk Archetype ~ Qinggong Monk Perfect Self",
            "Monk Archetype ~ Qinggong Monk Quivering Palm",
            "Monk Archetype ~ Qinggong Monk Slow Fall",
            "Monk Archetype ~ Qinggong Monk Timeless Body",
            "Monk Archetype ~ Qinggong Monk Tongue of the Sun and Moon",
            "Monk Archetype ~ Qinggong Monk Wholeness of Body",
            "Monk Archetype ~ Vow Monk",
            "Oracle Archetype ~ Dual-Cursed Oracle",
            "Oracle Archetype ~ Enlightened Philosopher",
            "Oracle Archetype ~ Planar Oracle",
            "Oracle Archetype ~ Possessed Oracle",
            "Oracle Archetype ~ Seer",
            "Paladin Archetype ~ Oath against Corruption",
            "Paladin Archetype ~ Oath against Fiends",
            "Paladin Archetype ~ Oath against Savagery",
            "Paladin Archetype ~ Oath against Undeath",
            "Paladin Archetype ~ Oath against the Wyrm",
            "Paladin Archetype ~ Oath of Charity",
            "Paladin Archetype ~ Oath of Chastity",
            "Paladin Archetype ~ Oath of Loyalty",
            "Paladin Archetype ~ Oath of Vengeance",
            "Ranger Archetype ~ Trapper",
            "Summoner Archetype ~ Broodmaster",
            "Summoner Archetype ~ Evolutionist",
            "Summoner Archetype ~ Master Summoner",
            "Witch Archetype ~ Beast-Bonded",
            "Witch Archetype ~ Gravewalker",
            "Witch Archetype ~ Hedge Witch",
            "Witch Archetype ~ Sea Witch",
            "Wizard Archetype ~ Scrollmaster",
        ],
    ),
    (
        "ultimate_psionics",
        "archetypes",
        &[
            "Barbarian Archetype ~ Raging Beast",
            "Bard Archetype ~ Thoughtsinger",
            "Druid Archetype ~ Gaean",
            "Druid Archetype ~ Serpent Lord",
            "Fighter Archetype ~ Ironborn",
            "Fighter Archetype ~ Psionic Fighter",
            "Monk Archetype ~ Disciple of the Raging Sea",
            "Monk Archetype ~ Enlightened Monk",
            "Paladin Archetype ~ Purifier",
            "Paladin Archetype ~ Sleeper's Guardian",
            "Ranger Archetype ~ Kinslayer",
            "Ranger Archetype ~ Pack Leader",
            "Rogue Archetype ~ Cerebral Infiltrator",
            "Rogue Archetype ~ Menteur",
            "Rogue Archetype ~ Reaving Raider",
        ],
    ),
    (
        "ultimate_wilderness",
        "archetypes",
        &[
            "Companion Archetype ~ Aberrant Companion",
            "Companion Archetype ~ Ambusher",
            "Companion Archetype ~ Augmented Companion",
            "Companion Archetype ~ Auspice",
            "Companion Archetype ~ Bodyguard",
            "Companion Archetype ~ Bully",
            "Companion Archetype ~ Daredevil",
            "Companion Archetype ~ Deathtouched Companion",
            "Companion Archetype ~ Draconic Companion",
            "Companion Archetype ~ Feytouched Companion",
            "Companion Archetype ~ Precocious Companion",
            "Companion Archetype ~ Racer",
            "Companion Archetype ~ Totem Guide",
            "Companion Archetype ~ Tracker",
            "Companion Archetype ~ Verdant Companion",
            "Companion Archetype ~ Wrecker",
            "Familiar Archetype ~ Ambassador",
            "Familiar Archetype ~ Animal Exemplar",
            "Familiar Archetype ~ Egotist",
            "Familiar Archetype ~ Emissary",
            "Familiar Archetype ~ Figment",
            "Familiar Archetype ~ Infiltrator",
            "Familiar Archetype ~ Mascot",
            "Familiar Archetype ~ Mauler",
            "Familiar Archetype ~ Pilferer",
            "Familiar Archetype ~ Prankster",
            "Familiar Archetype ~ Protector",
            "Familiar Archetype ~ Sage",
            "Familiar Archetype ~ Soulbound Familiar",
            "Familiar Archetype ~ Valet",
        ],
    ),
    (
        "beastiary1",
        "monster_abilities",
        // `decisions.md §20` (no_record-to-zero wave 2): 180 `monster_ability`
        // rows no monster row of `bestiary` claims (a shared reference-library
        // entry -- `Universal Monster Rule ~ X`, `Vampire ~ X`, `Lich ~ X`,
        // `Regeneration ~ X`, `Immunity to X`, and similar generic special-
        // ability text no single stat block in this book owns) now SHIP with
        // `owners: &[]` instead of being dropped by
        // `scripts/transcribe_monster_tables.py`'s orphan pass, because an
        // un-ingested row's shape cannot be measured and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks
        // a monster's own `ability_keys`
        // (`apps/desktop/src-tauri/src/monster_catalog.rs`), so an owner-less
        // record here reaches no screen at all -- not the stub class
        // `decisions.md §44.2` was written about, because a stub is a record
        // a player's screen SHOWS empty, and these reach no screen. Pinned by
        // exact key rather than by count so a NEW silent non-reach still
        // fails here; `bestiary::tests::
        // every_owner_less_ability_is_a_named_and_pinned_non_reach` pins the
        // identical set from the corpus side. Re-derive:
        // `python3 scripts/transcribe_monster_tables.py bestiary 2>&1 >/dev/null
        // | grep 'orphan ability row'`.
        //
        // **Correction, same cycle**: an EARLIER version of this entry was
        // pulled after empirically appearing to "already reach a player" --
        // that read was wrong. It compared against `corpus_record_keys`
        // BEFORE `gen_book_cache beastiary` had ever written these 180
        // records to `data/corpus/`, so the denominator simply did not
        // contain them yet; every one of the 180 looked "fixed" (recorded
        // but not live) only because none was live at all. Re-checked AFTER
        // the real regen (`reach_gate::tests::
        // bestiary_1_monsters_reach_the_monster_catalog_record_by_record`,
        // which panicked `NotSurfaced` naming all 180): they do not reach.
        &[
            "beastiary:monster_ability:ability_damage_ex",
            "beastiary:monster_ability:ability_damage_su",
            "beastiary:monster_ability:ability_drain_ex",
            "beastiary:monster_ability:ability_drain_su",
            "beastiary:monster_ability:afflicted_lycanthrope",
            "beastiary:monster_ability:air_mephit_breath_weapon",
            "beastiary:monster_ability:air_mephit_fast_healing",
            "beastiary:monster_ability:aligned",
            "beastiary:monster_ability:angel_protective_aura",
            "beastiary:monster_ability:angel_save_vs_poison",
            "beastiary:monster_ability:angel_truespeech",
            "beastiary:monster_ability:animated_object_additional_attack",
            "beastiary:monster_ability:animated_object_burrow_movement",
            "beastiary:monster_ability:animated_object_climb_movement",
            "beastiary:monster_ability:animated_object_constrict",
            "beastiary:monster_ability:animated_object_faster_burrow",
            "beastiary:monster_ability:animated_object_faster_climb",
            "beastiary:monster_ability:animated_object_faster_fly",
            "beastiary:monster_ability:animated_object_faster_swim",
            "beastiary:monster_ability:animated_object_faster_walk",
            "beastiary:monster_ability:animated_object_fly_movement",
            "beastiary:monster_ability:animated_object_grab",
            "beastiary:monster_ability:animated_object_metal_adamantine",
            "beastiary:monster_ability:animated_object_metal_common",
            "beastiary:monster_ability:animated_object_metal_mithral",
            "beastiary:monster_ability:animated_object_stone",
            "beastiary:monster_ability:animated_object_swim_movement",
            "beastiary:monster_ability:animated_object_trample",
            "beastiary:monster_ability:archon_aura_of_menace",
            "beastiary:monster_ability:archon_save_vs_poison",
            "beastiary:monster_ability:archon_teleport",
            "beastiary:monster_ability:archon_truespeech",
            "beastiary:monster_ability:azata_truespeech",
            "beastiary:monster_ability:bleed",
            "beastiary:monster_ability:bloody_skeleton_deathless",
            "beastiary:monster_ability:breath_weapon_cone_of_force",
            "beastiary:monster_ability:breath_weapon_cone_of_lightning",
            "beastiary:monster_ability:breath_weapon_cone_of_piercing",
            "beastiary:monster_ability:breath_weapon_cone_of_repulsion_gas",
            "beastiary:monster_ability:breath_weapon_cone_of_sleep",
            "beastiary:monster_ability:breath_weapon_cone_of_sonic",
            "beastiary:monster_ability:breath_weapon_prismatic_spray",
            "beastiary:monster_ability:burning_skeleton_fiery_aura",
            "beastiary:monster_ability:burning_skeleton_fiery_death",
            "beastiary:monster_ability:burning_skeleton_fire_damage",
            "beastiary:monster_ability:can_t_be_disarmed",
            "beastiary:monster_ability:celestial_creature_smite_evil",
            "beastiary:monster_ability:cold_iron_cobra_attacks",
            "beastiary:monster_ability:construct_traits_output",
            "beastiary:monster_ability:corrupting_gaze",
            "beastiary:monster_ability:corrupting_touch",
            "beastiary:monster_ability:curse",
            "beastiary:monster_ability:curse_of_lycanthropy",
            "beastiary:monster_ability:disease_ex",
            "beastiary:monster_ability:disease_su",
            "beastiary:monster_ability:distraction",
            "beastiary:monster_ability:dragon_traits_output",
            "beastiary:monster_ability:draining_touch",
            "beastiary:monster_ability:dust_mephit_breath_weapon",
            "beastiary:monster_ability:dust_mephit_fast_healing",
            "beastiary:monster_ability:earth_mephit_breath_weapon",
            "beastiary:monster_ability:earth_mephit_change_size",
            "beastiary:monster_ability:earth_mephit_fast_healing",
            "beastiary:monster_ability:elemental_traits_output",
            "beastiary:monster_ability:fey_traits_output",
            "beastiary:monster_ability:fiendish_creature_smite_good",
            "beastiary:monster_ability:fire_mephit_breath_weapon",
            "beastiary:monster_ability:fire_mephit_fast_healing",
            "beastiary:monster_ability:flight_ex",
            "beastiary:monster_ability:frightful_moan",
            "beastiary:monster_ability:gaze",
            "beastiary:monster_ability:guardian_naga_spells",
            "beastiary:monster_ability:gynosphinx_symbol_of_fear",
            "beastiary:monster_ability:gynosphinx_symbol_of_pain",
            "beastiary:monster_ability:gynosphinx_symbol_of_persuasion",
            "beastiary:monster_ability:gynosphinx_symbol_of_sleep",
            "beastiary:monster_ability:gynosphinx_symbol_of_stunning",
            "beastiary:monster_ability:half_celestial_smite_evil",
            "beastiary:monster_ability:half_fiend_smite_good",
            "beastiary:monster_ability:humanoid_traits_output",
            "beastiary:monster_ability:ice_mephit_breath_weapon",
            "beastiary:monster_ability:ice_mephit_fast_healing",
            "beastiary:monster_ability:immunity_to_ability_drain",
            "beastiary:monster_ability:immunity_to_bleed",
            "beastiary:monster_ability:immunity_to_bludgeoning_damage",
            "beastiary:monster_ability:immunity_to_death_from_massive_damage",
            "beastiary:monster_ability:immunity_to_emotion_effects",
            "beastiary:monster_ability:immunity_to_exhaustion",
            "beastiary:monster_ability:immunity_to_fatigue",
            "beastiary:monster_ability:immunity_to_language_dependent_spells",
            "beastiary:monster_ability:immunity_to_magic",
            "beastiary:monster_ability:immunity_to_magic_missile",
            "beastiary:monster_ability:immunity_to_magical_sleep",
            "beastiary:monster_ability:immunity_to_necromancy",
            "beastiary:monster_ability:immunity_to_negative_levels",
            "beastiary:monster_ability:immunity_to_nonlethal_damage",
            "beastiary:monster_ability:immunity_to_permanent_wounds",
            "beastiary:monster_ability:immunity_to_phantasms",
            "beastiary:monster_ability:immunity_to_piercing_damage",
            "beastiary:monster_ability:immunity_to_slashing_damage",
            "beastiary:monster_ability:immunity_to_strength_drain",
            "beastiary:monster_ability:immunity_to_weapon_damage",
            "beastiary:monster_ability:immunity_to_wind_effects",
            "beastiary:monster_ability:incorporeal",
            "beastiary:monster_ability:incorporeal_traits_output",
            "beastiary:monster_ability:lich_fear_aura",
            "beastiary:monster_ability:lich_paralyzing_touch",
            "beastiary:monster_ability:lich_rejuvenation",
            "beastiary:monster_ability:lycanthrope_change_shape",
            "beastiary:monster_ability:lycanthropic_empathy",
            "beastiary:monster_ability:magma_mephit_breath_weapon",
            "beastiary:monster_ability:magma_mephit_fast_healing",
            "beastiary:monster_ability:magma_mephit_magma_form",
            "beastiary:monster_ability:malevolence",
            "beastiary:monster_ability:mephit_summon",
            "beastiary:monster_ability:monstrous_humanoid_traits_output",
            "beastiary:monster_ability:natural_lycanthrope",
            "beastiary:monster_ability:ooze_mephit_breath_weapon",
            "beastiary:monster_ability:ooze_mephit_fast_healing",
            "beastiary:monster_ability:outsider_traits_output",
            "beastiary:monster_ability:paralysis_ex",
            "beastiary:monster_ability:paralysis_su",
            "beastiary:monster_ability:permanency_spell_arcane_sight",
            "beastiary:monster_ability:permanency_spell_comprehend_languages",
            "beastiary:monster_ability:permanency_spell_darkvision",
            "beastiary:monster_ability:permanency_spell_detect_magic",
            "beastiary:monster_ability:permanency_spell_enlarge_person",
            "beastiary:monster_ability:permanency_spell_greater_magic_fang",
            "beastiary:monster_ability:permanency_spell_magic_fang",
            "beastiary:monster_ability:permanency_spell_read_magic",
            "beastiary:monster_ability:permanency_spell_reduce_person",
            "beastiary:monster_ability:permanency_spell_resistance",
            "beastiary:monster_ability:permanency_spell_see_invisibility",
            "beastiary:monster_ability:permanency_spell_telepathic_bond",
            "beastiary:monster_ability:permanency_spell_tongues",
            "beastiary:monster_ability:plague_zombie_disease",
            "beastiary:monster_ability:poison_ex",
            "beastiary:monster_ability:poison_su",
            "beastiary:monster_ability:pseudodragon_poison",
            "beastiary:monster_ability:push",
            "beastiary:monster_ability:regeneration_acid",
            "beastiary:monster_ability:regeneration_acid_cold",
            "beastiary:monster_ability:regeneration_acid_cold_fire",
            "beastiary:monster_ability:regeneration_choose",
            "beastiary:monster_ability:regeneration_epic",
            "beastiary:monster_ability:regeneration_fire_or_good_spells",
            "beastiary:monster_ability:rejuvenation",
            "beastiary:monster_ability:resistance_to_acid_output",
            "beastiary:monster_ability:resistance_to_cold_output",
            "beastiary:monster_ability:resistance_to_electricity_output",
            "beastiary:monster_ability:resistance_to_fire_output",
            "beastiary:monster_ability:resistance_to_sonic_output",
            "beastiary:monster_ability:salt_mephit_breath_weapon",
            "beastiary:monster_ability:salt_mephit_dehydrate",
            "beastiary:monster_ability:salt_mephit_fast_healing",
            "beastiary:monster_ability:spirit_naga_spells",
            "beastiary:monster_ability:steam_mephit_boiling_rain",
            "beastiary:monster_ability:steam_mephit_breath_weapon",
            "beastiary:monster_ability:steam_mephit_fast_healing",
            "beastiary:monster_ability:svirfneblin_hatred",
            "beastiary:monster_ability:swarm_traits_output",
            "beastiary:monster_ability:telekinesis",
            "beastiary:monster_ability:undead_traits_output",
            "beastiary:monster_ability:universal_monster_rule_breath_weapon",
            "beastiary:monster_ability:universal_monster_rule_burn",
            "beastiary:monster_ability:universal_monster_rule_change_shape",
            "beastiary:monster_ability:universal_monster_rule_disease_extraordinary",
            "beastiary:monster_ability:universal_monster_rule_disease_supernatural",
            "beastiary:monster_ability:universal_monster_rule_fast_healing",
            "beastiary:monster_ability:universal_monster_rule_grab",
            "beastiary:monster_ability:universal_monster_rule_light_blindness",
            "beastiary:monster_ability:universal_monster_rule_light_sensitivity",
            "beastiary:monster_ability:universal_monster_rule_paralysis_extraordinary",
            "beastiary:monster_ability:universal_monster_rule_paralysis_supernatural",
            "beastiary:monster_ability:universal_monster_rule_poison_extraordinary",
            "beastiary:monster_ability:universal_monster_rule_poison_supernatural",
            "beastiary:monster_ability:universal_monster_rule_stench",
            "beastiary:monster_ability:universal_monster_rule_telepathy",
            "beastiary:monster_ability:universal_monster_rule_trip",
            "beastiary:monster_ability:universal_monster_rule_whirlwind",
            "beastiary:monster_ability:vampire_change_shape",
            "beastiary:monster_ability:vampire_children_of_the_night",
            "beastiary:monster_ability:vampire_create_spawn",
            "beastiary:monster_ability:vampire_energy_drain",
            "beastiary:monster_ability:vulnerability_to_acid",
            "beastiary:monster_ability:vulnerability_to_electricity",
            "beastiary:monster_ability:vulnerability_to_protection_from_evil",
            "beastiary:monster_ability:vulnerability_to_sonic",
            "beastiary:monster_ability:vulnerability_to_sunlight",
            "beastiary:monster_ability:water_dependency",
            "beastiary:monster_ability:water_mephit_breath_weapon",
            "beastiary:monster_ability:water_mephit_fast_healing",
            "beastiary:monster_ability:wererat",
            "beastiary:monster_ability:werewolf",
            "beastiary:monster_ability:zombie_death_burst",
            "beastiary:monster_ability:zombie_quick_strikes",
            "beastiary:monster_ability:zombie_staggered",
        ],
    ),
    (
        "bestiary_2",
        "monster_abilities",
        // `decisions.md §20` (no_record-to-zero wave 2 follow-on): 85
        // `monster_ability` rows no monster row of this book claims now SHIP
        // with `owners: &[]` instead of being dropped by
        // `scripts/transcribe_monster_tables.py`'s orphan pass, because an
        // un-ingested row's shape cannot be measured and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks
        // a monster's own `ability_keys`, so an owner-less record here reaches
        // no screen at all -- not the stub class `decisions.md §44.2` was
        // written about. Pinned by exact key rather than by count so a NEW
        // silent non-reach still fails here; `bestiary_2::tests::
        // every_owner_less_ability_is_a_named_and_pinned_non_reach` (digest
        // 0xfb07_0eb2_4302_5d02) pins the identical set from the corpus side. Re-derive:
        // `python3 scripts/transcribe_monster_tables.py bestiary_2 2>&1 >/dev/null
        // | grep 'orphan ability row'`.
        &[
            "bestiary_2:monster_ability:acid_draconal_breath_weapon",
            "bestiary_2:monster_ability:aeon_envisaging",
            "bestiary_2:monster_ability:aeon_extension_of_all",
            "bestiary_2:monster_ability:aeon_void_form",
            "bestiary_2:monster_ability:black_scorpion_poison",
            "bestiary_2:monster_ability:black_scorpion_rapid_stinging",
            "bestiary_2:monster_ability:cave_scorpion_poison",
            "bestiary_2:monster_ability:cold_draconal_breath_weapon",
            "bestiary_2:monster_ability:companion_favored_terrain",
            "bestiary_2:monster_ability:crab_companion_water_dependency",
            "bestiary_2:monster_ability:detect_poison_constant",
            "bestiary_2:monster_ability:detect_snares_and_pits_constant",
            "bestiary_2:monster_ability:detect_thoughts_constant",
            "bestiary_2:monster_ability:discern_lies_constant",
            "bestiary_2:monster_ability:draconal_acid",
            "bestiary_2:monster_ability:draconal_cold",
            "bestiary_2:monster_ability:draconal_fire",
            "bestiary_2:monster_ability:emotion_aura_courage_fear",
            "bestiary_2:monster_ability:emotion_aura_empathy_apathy",
            "bestiary_2:monster_ability:emotion_aura_hope_despair",
            "bestiary_2:monster_ability:entrap_su",
            "bestiary_2:monster_ability:entropic_creature_smite_law",
            "bestiary_2:monster_ability:fire_draconal_breath_weapon",
            "bestiary_2:monster_ability:giant_centipede_companion_poison",
            "bestiary_2:monster_ability:giant_spider_companion_poison",
            "bestiary_2:monster_ability:giant_tarantula_poison",
            "bestiary_2:monster_ability:giant_whiptail_centipede_poison",
            "bestiary_2:monster_ability:giant_whiptail_centipede_tail_slap",
            "bestiary_2:monster_ability:greensting_scorpion_companion_poison",
            "bestiary_2:monster_ability:greensting_scorpion_poison",
            "bestiary_2:monster_ability:inevitable_constructed",
            "bestiary_2:monster_ability:inevitable_truespeech",
            "bestiary_2:monster_ability:juju_zombie_immunity_to_magic_missile",
            "bestiary_2:monster_ability:nightshade_channel_energy",
            "bestiary_2:monster_ability:octopus_companion_ink_cloud",
            "bestiary_2:monster_ability:octopus_companion_jet",
            "bestiary_2:monster_ability:octopus_companion_poison",
            "bestiary_2:monster_ability:ogrekin_deformed_hand",
            "bestiary_2:monster_ability:ogrekin_fragile",
            "bestiary_2:monster_ability:ogrekin_light_sensitive",
            "bestiary_2:monster_ability:ogrekin_obese",
            "bestiary_2:monster_ability:ogrekin_oversized_limb",
            "bestiary_2:monster_ability:ogrekin_oversized_maw",
            "bestiary_2:monster_ability:ogrekin_quick_metabolism",
            "bestiary_2:monster_ability:ogrekin_stunted_legs",
            "bestiary_2:monster_ability:ogrekin_thick_skin",
            "bestiary_2:monster_ability:ogrekin_vestigal_limb",
            "bestiary_2:monster_ability:ogrekin_vestigal_twin",
            "bestiary_2:monster_ability:ogrekin_weak_mind",
            "bestiary_2:monster_ability:oni_change_shape",
            "bestiary_2:monster_ability:petitioner_larva",
            "bestiary_2:monster_ability:petitioner_the_chosen",
            "bestiary_2:monster_ability:petitioner_the_cleansed",
            "bestiary_2:monster_ability:petitioner_the_damned",
            "bestiary_2:monster_ability:petitioner_the_dead",
            "bestiary_2:monster_ability:petitioner_the_elect",
            "bestiary_2:monster_ability:petitioner_the_hunted",
            "bestiary_2:monster_ability:petitioner_the_remade",
            "bestiary_2:monster_ability:petitioner_the_shapeless",
            "bestiary_2:monster_ability:poison_fiendish_bile",
            "bestiary_2:monster_ability:poison_mysterious_blood",
            "bestiary_2:monster_ability:poison_vile_disjunction",
            "bestiary_2:monster_ability:poisonous_magma_ooze_poison",
            "bestiary_2:monster_ability:protean_change_shape",
            "bestiary_2:monster_ability:ravener_breath_weapon",
            "bestiary_2:monster_ability:ravener_critical_hits",
            "bestiary_2:monster_ability:ravener_soul_consumption",
            "bestiary_2:monster_ability:ravener_soul_magic",
            "bestiary_2:monster_ability:regeneration_chaotic",
            "bestiary_2:monster_ability:resolute_creature_smite_chaos",
            "bestiary_2:monster_ability:sound_mimicry",
            "bestiary_2:monster_ability:speak_with_plants_constant",
            "bestiary_2:monster_ability:titan_centipede_poison",
            "bestiary_2:monster_ability:trip_tail_slap",
            "bestiary_2:monster_ability:universal_monster_rule_all_around_vision",
            "bestiary_2:monster_ability:universal_monster_rule_amphibious",
            "bestiary_2:monster_ability:universal_monster_rule_earth_glide",
            "bestiary_2:monster_ability:universal_monster_rule_see_in_darkness",
            "bestiary_2:monster_ability:void_zombie_blood_drain",
            "bestiary_2:monster_ability:werebear",
            "bestiary_2:monster_ability:wereboar",
            "bestiary_2:monster_ability:weretiger",
            "bestiary_2:monster_ability:worm_that_walks_discorporate",
            "bestiary_2:monster_ability:worm_that_walks_squirming_embrace",
            "bestiary_2:monster_ability:worm_that_walks_tenacious",
        ],
    ),
    (
        "bestiary_3",
        "monster_abilities",
        // `decisions.md §20` (no_record-to-zero wave 2 follow-on): 266
        // `monster_ability` rows no monster row of this book claims now SHIP
        // with `owners: &[]` instead of being dropped by
        // `scripts/transcribe_monster_tables.py`'s orphan pass, because an
        // un-ingested row's shape cannot be measured and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks
        // a monster's own `ability_keys`, so an owner-less record here reaches
        // no screen at all -- not the stub class `decisions.md §44.2` was
        // written about. Pinned by exact key rather than by count so a NEW
        // silent non-reach still fails here; `bestiary_3::tests::
        // every_owner_less_ability_is_a_named_and_pinned_non_reach` (digest
        // 0x01b4_2774_3381_b829, updated `decisions.md §27`/round 8 — 10 new
        // owner-less keys from the `TYPE:`-facet-vocabulary-gap group's
        // provisional-default closure) pins the identical set from the
        // corpus side. Re-derive:
        // `python3 scripts/transcribe_monster_tables.py bestiary_3 2>&1 >/dev/null
        // | grep 'orphan ability row'`.
        &[
            "bestiary_3:monster_ability:adlet_shaman",
            "bestiary_3:monster_ability:aghash_cursed_gaze",
            "bestiary_3:monster_ability:aghash_sandstorm",
            "bestiary_3:monster_ability:aghash_spell_like_abilities",
            "bestiary_3:monster_ability:aghasura_spell_like_abilities",
            "bestiary_3:monster_ability:akvan_armor_training",
            "bestiary_3:monster_ability:akvan_aura_of_hopelessness",
            "bestiary_3:monster_ability:akvan_create_ghul",
            "bestiary_3:monster_ability:akvan_prince_crumbling_earth",
            "bestiary_3:monster_ability:akvan_prince_dying_ember",
            "bestiary_3:monster_ability:akvan_prince_gasping_wind",
            "bestiary_3:monster_ability:akvan_prince_thirsty_sea",
            "bestiary_3:monster_ability:akvan_prince_unbalanced_soul",
            "bestiary_3:monster_ability:akvan_shake_faith",
            "bestiary_3:monster_ability:akvan_spell_like_abilities",
            "bestiary_3:monster_ability:akvan_torturous_gullet",
            "bestiary_3:monster_ability:androsphinx_roar",
            "bestiary_3:monster_ability:androsphinx_spells",
            "bestiary_3:monster_ability:ash_giant_disease",
            "bestiary_3:monster_ability:ash_giant_oversized_weapon",
            "bestiary_3:monster_ability:ash_giant_vermin_empathy",
            "bestiary_3:monster_ability:asura_save_bonus",
            "bestiary_3:monster_ability:asurendra_create_asura",
            "bestiary_3:monster_ability:asurendra_death",
            "bestiary_3:monster_ability:asurendra_none",
            "bestiary_3:monster_ability:asurendra_sacrilege",
            "bestiary_3:monster_ability:asurendra_shaping",
            "bestiary_3:monster_ability:asurendra_spell_like_abilities",
            "bestiary_3:monster_ability:behemoth_ruinous",
            "bestiary_3:monster_ability:behemoth_unstoppable",
            "bestiary_3:monster_ability:behemoth_vulnerable_to_miracles_and_wishes",
            "bestiary_3:monster_ability:bone_golem_bone_prison",
            "bestiary_3:monster_ability:bone_golem_immunity_to_magic",
            "bestiary_3:monster_ability:brass_golem_brass_falchion",
            "bestiary_3:monster_ability:brass_golem_breath_weapon",
            "bestiary_3:monster_ability:brass_golem_death_throes",
            "bestiary_3:monster_ability:brass_golem_immunity_to_magic",
            "bestiary_3:monster_ability:brass_golem_spell_like_abilities",
            "bestiary_3:monster_ability:cairn_linnorm_breath_weapon",
            "bestiary_3:monster_ability:cairn_linnorm_death_curse",
            "bestiary_3:monster_ability:cairn_linnorm_poison",
            "bestiary_3:monster_ability:cannon_golem_alloyed",
            "bestiary_3:monster_ability:cannon_golem_blasting_critical",
            "bestiary_3:monster_ability:cannon_golem_cannon",
            "bestiary_3:monster_ability:cannon_golem_gun_training",
            "bestiary_3:monster_ability:cannon_golem_immunity_to_magic",
            "bestiary_3:monster_ability:cave_giant_axe_wielder",
            "bestiary_3:monster_ability:clockwork_difficult_to_create",
            "bestiary_3:monster_ability:clockwork_goliath_cannon",
            "bestiary_3:monster_ability:clockwork_goliath_self_destruction",
            "bestiary_3:monster_ability:clockwork_leviathan_breath_weapon",
            "bestiary_3:monster_ability:clockwork_leviathan_grind",
            "bestiary_3:monster_ability:clockwork_leviathan_swallow_whole",
            "bestiary_3:monster_ability:clockwork_servant_net",
            "bestiary_3:monster_ability:clockwork_servant_repair_clockwork",
            "bestiary_3:monster_ability:clockwork_soldier_efficient_winding",
            "bestiary_3:monster_ability:clockwork_soldier_latch",
            "bestiary_3:monster_ability:clockwork_soldier_proficient",
            "bestiary_3:monster_ability:clockwork_soldier_standby",
            "bestiary_3:monster_ability:clockwork_spy_record_audio",
            "bestiary_3:monster_ability:clockwork_spy_self_destruct",
            "bestiary_3:monster_ability:clockwork_swift_reactions",
            "bestiary_3:monster_ability:clockwork_winding",
            "bestiary_3:monster_ability:coloxus_droning_wings",
            "bestiary_3:monster_ability:coloxus_siphon",
            "bestiary_3:monster_ability:coloxus_spell_like_abilities",
            "bestiary_3:monster_ability:contract_devil_binding_contract",
            "bestiary_3:monster_ability:contract_devil_impale",
            "bestiary_3:monster_ability:contract_devil_infernal_contract",
            "bestiary_3:monster_ability:contract_devil_infernal_investment",
            "bestiary_3:monster_ability:contract_devil_spell_like_abilities",
            "bestiary_3:monster_ability:criosphinx_spell_like_abilities",
            "bestiary_3:monster_ability:criosphinx_swooping_charge",
            "bestiary_3:monster_ability:crucidaemon_chained_daggers",
            "bestiary_3:monster_ability:crucidaemon_spell_like_abilities",
            "bestiary_3:monster_ability:crucidaemon_trap_making",
            "bestiary_3:monster_ability:dandasuka_spell_like_abilities",
            "bestiary_3:monster_ability:dandasuka_spells",
            "bestiary_3:monster_ability:deadfall_scorpion_poison",
            "bestiary_3:monster_ability:deadfall_scorpion_sudden_strike",
            "bestiary_3:monster_ability:deep_sea_serpent_elusive",
            "bestiary_3:monster_ability:deep_sea_serpent_powerful_tail",
            "bestiary_3:monster_ability:deep_sea_serpent_surge",
            "bestiary_3:monster_ability:demodand_faith_stealing_strike",
            "bestiary_3:monster_ability:demodand_heretical_soul",
            "bestiary_3:monster_ability:desert_drake_dazzling_emergence",
            "bestiary_3:monster_ability:desert_drake_sandstorm_breath",
            "bestiary_3:monster_ability:desert_drake_savage_bite",
            "bestiary_3:monster_ability:desert_drake_speed_surge",
            "bestiary_3:monster_ability:desert_giant_sandwalking",
            "bestiary_3:monster_ability:desert_giant_scimitar_training",
            "bestiary_3:monster_ability:div_see_in_darkness",
            "bestiary_3:monster_ability:emperor_kirin",
            "bestiary_3:monster_ability:eremite_evisceration",
            "bestiary_3:monster_ability:eremite_graft_flesh",
            "bestiary_3:monster_ability:eremite_immune_to_pain",
            "bestiary_3:monster_ability:eremite_pain",
            "bestiary_3:monster_ability:eremite_shadow_traveler",
            "bestiary_3:monster_ability:eremite_spell_like_abilities",
            "bestiary_3:monster_ability:eremite_unnerving_gaze",
            "bestiary_3:monster_ability:fire_yai_fiery_missile",
            "bestiary_3:monster_ability:fire_yai_smoke_form",
            "bestiary_3:monster_ability:fire_yai_spell_like_abilities",
            "bestiary_3:monster_ability:fjord_linnorm_breath_weapon",
            "bestiary_3:monster_ability:fjord_linnorm_death_curse",
            "bestiary_3:monster_ability:fjord_linnorm_poison",
            "bestiary_3:monster_ability:fuath_congeal_water",
            "bestiary_3:monster_ability:fuath_spell_like_abilities",
            "bestiary_3:monster_ability:fuath_vulnerable_to_sunlight",
            "bestiary_3:monster_ability:fungus_leshy_puffball",
            "bestiary_3:monster_ability:fungus_leshy_spell_like_abilities",
            "bestiary_3:monster_ability:fungus_leshy_spores",
            "bestiary_3:monster_ability:ghawwas_boiling_sea",
            "bestiary_3:monster_ability:ghawwas_poison",
            "bestiary_3:monster_ability:ghawwas_rough_hide",
            "bestiary_3:monster_ability:ghawwas_spell_like_abilities",
            "bestiary_3:monster_ability:ghost_scorpion_poison",
            "bestiary_3:monster_ability:giant_crab_spider_poison",
            "bestiary_3:monster_ability:giant_owl_animal_telepathy",
            "bestiary_3:monster_ability:giant_owl_insightful_senses",
            "bestiary_3:monster_ability:giant_owl_piercing_stare",
            "bestiary_3:monster_ability:giant_porcupine_quills",
            "bestiary_3:monster_ability:giant_rot_grub_gnaw",
            "bestiary_3:monster_ability:giant_rot_grub_poison",
            "bestiary_3:monster_ability:giant_sea_anemone_anchored",
            "bestiary_3:monster_ability:giant_sea_anemone_poison",
            "bestiary_3:monster_ability:giant_sea_anemone_sightless",
            "bestiary_3:monster_ability:giant_skunk_musk",
            "bestiary_3:monster_ability:giant_vulture_diseased",
            "bestiary_3:monster_ability:gourd_leshy_ensnare",
            "bestiary_3:monster_ability:gourd_leshy_keepsake",
            "bestiary_3:monster_ability:gourd_leshy_seed",
            "bestiary_3:monster_ability:gourd_leshy_spell_like_abilities",
            "bestiary_3:monster_ability:hekatonkheires_hands_of_war",
            "bestiary_3:monster_ability:hekatonkheires_hundred_handed_whirlwind",
            "bestiary_3:monster_ability:hekatonkheires_planar_leap",
            "bestiary_3:monster_ability:hekatonkheires_spell_like_abilities",
            "bestiary_3:monster_ability:hekatonkheires_stunning_slam",
            "bestiary_3:monster_ability:hieracosphinx_shriek",
            "bestiary_3:monster_ability:ice_yai_icy_missile",
            "bestiary_3:monster_ability:ice_yai_spell_like_abilities",
            "bestiary_3:monster_ability:ice_yai_staggering_strikes",
            "bestiary_3:monster_ability:incubus_pain_redoubled",
            "bestiary_3:monster_ability:incubus_spell_like_abilities",
            "bestiary_3:monster_ability:interlocutor_spell_like_abilities",
            "bestiary_3:monster_ability:interlocutor_surgical_strikes",
            "bestiary_3:monster_ability:interlocutor_unnerving_gaze",
            "bestiary_3:monster_ability:jiang_shi_vampire",
            "bestiary_3:monster_ability:jinushigami_infused_quarterstaff",
            "bestiary_3:monster_ability:jinushigami_manipulate_terrain",
            "bestiary_3:monster_ability:jinushigami_spell_like_abilities",
            "bestiary_3:monster_ability:jinushigami_ward",
            "bestiary_3:monster_ability:jotund_troll_all_seeing_attacks",
            "bestiary_3:monster_ability:jotund_troll_cacophonous_roar",
            "bestiary_3:monster_ability:jotund_troll_multiple_minds",
            "bestiary_3:monster_ability:jungle_giant_archery_expert",
            "bestiary_3:monster_ability:jungle_giant_spell_storing",
            "bestiary_3:monster_ability:jungle_giant_warding_tattoos",
            "bestiary_3:monster_ability:kami_fast_healing",
            "bestiary_3:monster_ability:kami_merge_with_ward",
            "bestiary_3:monster_ability:kami_ward",
            "bestiary_3:monster_ability:kodama_distracting_gaze",
            "bestiary_3:monster_ability:kodama_spell_like_abilities",
            "bestiary_3:monster_ability:kuwa_spell_like_abilities",
            "bestiary_3:monster_ability:kyton_regeneration",
            "bestiary_3:monster_ability:kyton_unnerving_gaze",
            "bestiary_3:monster_ability:leaf_leshy_glide",
            "bestiary_3:monster_ability:leaf_leshy_seedpods",
            "bestiary_3:monster_ability:leaf_leshy_spell_like_abilities",
            "bestiary_3:monster_ability:leshy_change_shape",
            "bestiary_3:monster_ability:leshy_plantspeech",
            "bestiary_3:monster_ability:leshy_spell_like_abilities",
            "bestiary_3:monster_ability:leshy_verdant_burst",
            "bestiary_3:monster_ability:lunar_naga_hypnosis",
            "bestiary_3:monster_ability:lunar_naga_poison",
            "bestiary_3:monster_ability:lunar_naga_spells",
            "bestiary_3:monster_ability:maharaja_extra_initiative",
            "bestiary_3:monster_ability:maharaja_spells",
            "bestiary_3:monster_ability:marai_confusion",
            "bestiary_3:monster_ability:marai_energy_bolts",
            "bestiary_3:monster_ability:marai_spells",
            "bestiary_3:monster_ability:moss_troll_fear_of_fire",
            "bestiary_3:monster_ability:moss_troll_tree_climber",
            "bestiary_3:monster_ability:ogre_spider_poison",
            "bestiary_3:monster_ability:pairaka_disease",
            "bestiary_3:monster_ability:pairaka_lustful_dreams",
            "bestiary_3:monster_ability:pairaka_spell_like_abilities",
            "bestiary_3:monster_ability:rajadhiraja",
            "bestiary_3:monster_ability:rakshasa_change_shape",
            "bestiary_3:monster_ability:rakshasa_detect_thoughts",
            "bestiary_3:monster_ability:rift_drake_clinging_corrosion",
            "bestiary_3:monster_ability:rift_drake_savage_bite",
            "bestiary_3:monster_ability:rift_drake_speed_surge",
            "bestiary_3:monster_ability:river_drake_caustic_mucus",
            "bestiary_3:monster_ability:river_drake_speed_surge",
            "bestiary_3:monster_ability:rot_grub_swarm_infestation",
            "bestiary_3:monster_ability:royal_naga_change_shape",
            "bestiary_3:monster_ability:royal_naga_dual_gaze",
            "bestiary_3:monster_ability:royal_naga_spell_like_abilities",
            "bestiary_3:monster_ability:royal_naga_spells",
            "bestiary_3:monster_ability:schir_disease",
            "bestiary_3:monster_ability:schir_spell_like_abilities",
            "bestiary_3:monster_ability:seaweed_leshy_air_cyst",
            "bestiary_3:monster_ability:seaweed_leshy_spell_like_abilities",
            "bestiary_3:monster_ability:seaweed_leshy_water_jet",
            "bestiary_3:monster_ability:sepid_deflect_rays",
            "bestiary_3:monster_ability:sepid_rain_of_debris",
            "bestiary_3:monster_ability:sepid_spell_like_abilities",
            "bestiary_3:monster_ability:shikigami_improvised_weapon_mastery",
            "bestiary_3:monster_ability:shikigami_spell_like_abilities",
            "bestiary_3:monster_ability:shipwrecker_crab_powerful_claws",
            "bestiary_3:monster_ability:shira_consume_essence",
            "bestiary_3:monster_ability:shira_dusty_pelt",
            "bestiary_3:monster_ability:shira_spell_like_abilities",
            "bestiary_3:monster_ability:taiga_linnorm_breath_weapon",
            "bestiary_3:monster_ability:taiga_linnorm_death_curse",
            "bestiary_3:monster_ability:taiga_linnorm_poison",
            "bestiary_3:monster_ability:taiga_linnorm_spines",
            "bestiary_3:monster_ability:tataka_martial_artist",
            "bestiary_3:monster_ability:tataka_spell_like_abilities",
            "bestiary_3:monster_ability:tataka_spells",
            "bestiary_3:monster_ability:tempest_behemoth_gale",
            "bestiary_3:monster_ability:tempest_behemoth_scales",
            "bestiary_3:monster_ability:tempest_behemoth_spell_like_abilities",
            "bestiary_3:monster_ability:tempest_behemoth_thunderbolt",
            "bestiary_3:monster_ability:thalassic_behemoth_spell_like_abilities",
            "bestiary_3:monster_ability:thalassic_behemoth_water_jet",
            "bestiary_3:monster_ability:thriae_queen_launch_merope",
            "bestiary_3:monster_ability:thriae_queen_merope_coat",
            "bestiary_3:monster_ability:thriae_queen_poison",
            "bestiary_3:monster_ability:thriae_queen_spawn_soldiers",
            "bestiary_3:monster_ability:thriae_queen_spell_like_abilities",
            "bestiary_3:monster_ability:thriae_seer_merope_consumption",
            "bestiary_3:monster_ability:thriae_seer_mind_sting",
            "bestiary_3:monster_ability:thriae_seer_spell_like_abilities",
            "bestiary_3:monster_ability:thriae_soldier_merope_consumption",
            "bestiary_3:monster_ability:thriae_soldier_poison",
            "bestiary_3:monster_ability:thunder_behemoth_mighty_roar",
            "bestiary_3:monster_ability:thunder_behemoth_rock_spitting",
            "bestiary_3:monster_ability:thunder_behemoth_spell_like_abilities",
            "bestiary_3:monster_ability:thunder_behemoth_swallow_whole",
            "bestiary_3:monster_ability:tor_linnorm_breath_weapon",
            "bestiary_3:monster_ability:tor_linnorm_death_curse",
            "bestiary_3:monster_ability:tor_linnorm_lava_affinity",
            "bestiary_3:monster_ability:tor_linnorm_poison",
            "bestiary_3:monster_ability:toshigami_spell_like_abilities",
            "bestiary_3:monster_ability:toshigami_touch_of_ages",
            "bestiary_3:monster_ability:traits_output_asura",
            "bestiary_3:monster_ability:traits_output_behemoth",
            "bestiary_3:monster_ability:traits_output_clockwork",
            "bestiary_3:monster_ability:traits_output_demodand",
            "bestiary_3:monster_ability:traits_output_div",
            "bestiary_3:monster_ability:traits_output_kami",
            "bestiary_3:monster_ability:traits_output_kyton",
            "bestiary_3:monster_ability:traits_output_leshy",
            "bestiary_3:monster_ability:traits_output_rakshasa",
            "bestiary_3:monster_ability:tripurasura_change_shape",
            "bestiary_3:monster_ability:tripurasura_elusive",
            "bestiary_3:monster_ability:tripurasura_poison",
            "bestiary_3:monster_ability:tripurasura_spell_like_abilities",
            "bestiary_3:monster_ability:unfettered_eidolon_cha",
            "bestiary_3:monster_ability:unfettered_eidolon_con",
            "bestiary_3:monster_ability:unfettered_eidolon_dex",
            "bestiary_3:monster_ability:unfettered_eidolon_int",
            "bestiary_3:monster_ability:unfettered_eidolon_str",
            "bestiary_3:monster_ability:unfettered_eidolon_wis",
            "bestiary_3:monster_ability:upasunda_infused_weapons",
            "bestiary_3:monster_ability:upasunda_multiweapon_mastery",
            "bestiary_3:monster_ability:upasunda_spell_like_abilities",
            "bestiary_3:monster_ability:vishkanya_toxic_vishkanya_venom",
            "bestiary_3:monster_ability:void_yai_commanding_voice",
            "bestiary_3:monster_ability:void_yai_spell_like_abilities",
            "bestiary_3:monster_ability:void_yai_void_form",
            "bestiary_3:monster_ability:void_yai_void_missile",
            "bestiary_3:monster_ability:void_yai_void_trap",
            "bestiary_3:monster_ability:vulnudaemon_aura_of_doom",
            "bestiary_3:monster_ability:vulnudaemon_spell_like_abilities",
            "bestiary_3:monster_ability:water_naga_poison",
            "bestiary_3:monster_ability:water_naga_spells",
            "bestiary_3:monster_ability:water_yai_acidic_missile",
            "bestiary_3:monster_ability:water_yai_flowing_robes",
            "bestiary_3:monster_ability:water_yai_liquid_form",
            "bestiary_3:monster_ability:water_yai_spell_like_abilities",
            "bestiary_3:monster_ability:zuishin_healing_arrow",
            "bestiary_3:monster_ability:zuishin_holy_weapons",
            "bestiary_3:monster_ability:zuishin_spell_like_abilities",
        ],
    ),
    (
        "bestiary_4",
        "monster_abilities",
        // `decisions.md §20` (no_record-to-zero wave 2 follow-on): 187
        // `monster_ability` rows no monster row of this book claims now SHIP
        // with `owners: &[]` instead of being dropped by
        // `scripts/transcribe_monster_tables.py`'s orphan pass, because an
        // un-ingested row's shape cannot be measured and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks
        // a monster's own `ability_keys`, so an owner-less record here reaches
        // no screen at all -- not the stub class `decisions.md §44.2` was
        // written about. Pinned by exact key rather than by count so a NEW
        // silent non-reach still fails here; `bestiary_4::tests::
        // every_owner_less_ability_is_a_named_and_pinned_non_reach` (digest
        // 0x87ed_3d78_0aa9_ca92) pins the identical set from the corpus side. Re-derive:
        // `python3 scripts/transcribe_monster_tables.py bestiary_4 2>&1 >/dev/null
        // | grep 'orphan ability row'`.
        &[
            "bestiary_4:monster_ability:broken_soul_agonized_wail",
            "bestiary_4:monster_ability:broken_soul_baleful_gaze",
            "bestiary_4:monster_ability:broken_soul_stat_bonuses",
            "bestiary_4:monster_ability:broken_soul_torturous_touch",
            "bestiary_4:monster_ability:colossus_alternate_form",
            "bestiary_4:monster_ability:colossus_mythic_creation",
            "bestiary_4:monster_ability:colossus_mythic_quickening",
            "bestiary_4:monster_ability:colossus_mythic_resilience",
            "bestiary_4:monster_ability:colossus_pinning_stomp",
            "bestiary_4:monster_ability:colossus_selective_antimagic_aura",
            "bestiary_4:monster_ability:demon_lord_abyssal_resurrection",
            "bestiary_4:monster_ability:demon_lord_dagon_breath_weapon",
            "bestiary_4:monster_ability:demon_lord_dagon_command_aquatic_creature",
            "bestiary_4:monster_ability:demon_lord_dagon_poison",
            "bestiary_4:monster_ability:demon_lord_dagon_transformation",
            "bestiary_4:monster_ability:demon_lord_frightful_presence",
            "bestiary_4:monster_ability:demon_lord_grant_spells",
            "bestiary_4:monster_ability:demon_lord_kostchtchie_clutch_foe",
            "bestiary_4:monster_ability:demon_lord_kostchtchie_crushing_blow",
            "bestiary_4:monster_ability:demon_lord_kostchtchie_favored_enemy",
            "bestiary_4:monster_ability:demon_lord_kostchtchie_powerful_slam",
            "bestiary_4:monster_ability:demon_lord_kostchtchie_vengeful_strike",
            "bestiary_4:monster_ability:demon_lord_pazuzu_aura_of_locusts",
            "bestiary_4:monster_ability:demon_lord_pazuzu_avian_mastery",
            "bestiary_4:monster_ability:demon_lord_pazuzu_hear_name",
            "bestiary_4:monster_ability:demon_lord_pazuzu_poison",
            "bestiary_4:monster_ability:demon_lord_pazuzu_possession",
            "bestiary_4:monster_ability:demon_lord_pazuzu_profane_wishcraft",
            "bestiary_4:monster_ability:demon_lord_pazuzu_swarm_master",
            "bestiary_4:monster_ability:demon_lord_powerful_weaponry",
            "bestiary_4:monster_ability:demon_lord_regeneration",
            "bestiary_4:monster_ability:demon_lord_summon_demons",
            "bestiary_4:monster_ability:devilbound_creature_contract_bound",
            "bestiary_4:monster_ability:devilbound_creature_evil_aura",
            "bestiary_4:monster_ability:devilbound_creature_signatory_devil",
            "bestiary_4:monster_ability:devilbound_creature_summon_devil",
            "bestiary_4:monster_ability:divine_guardian_ability_healing",
            "bestiary_4:monster_ability:divine_guardian_aura",
            "bestiary_4:monster_ability:divine_guardian_blessed_life",
            "bestiary_4:monster_ability:divine_guardian_divine_swiftness",
            "bestiary_4:monster_ability:divine_guardian_sacred_site",
            "bestiary_4:monster_ability:drakainia_spawn_chameleon_scales",
            "bestiary_4:monster_ability:drakainia_spawn_covered_in_eyes",
            "bestiary_4:monster_ability:drakainia_spawn_favored_spawn",
            "bestiary_4:monster_ability:drakainia_spawn_impenetrable_skin",
            "bestiary_4:monster_ability:drakainia_spawn_multiple_heads",
            "bestiary_4:monster_ability:drakainia_spawn_poison_glands",
            "bestiary_4:monster_ability:drakainia_spawn_serpent_headed_tail",
            "bestiary_4:monster_ability:drakainia_spawn_sticky",
            "bestiary_4:monster_ability:drakainia_spawn_tentacles",
            "bestiary_4:monster_ability:drakainia_spawn_vestigial_companion",
            "bestiary_4:monster_ability:dread_gholdako_paralysis",
            "bestiary_4:monster_ability:ectoplasmic_creature_alterations",
            "bestiary_4:monster_ability:ectoplasmic_creature_horrifying_ooze",
            "bestiary_4:monster_ability:ectoplasmic_creature_phase_lurch",
            "bestiary_4:monster_ability:empyreal_lord_cernunnos_greater_slaying_arrow",
            "bestiary_4:monster_ability:empyreal_lord_cernunnos_horned_lord_s_charge",
            "bestiary_4:monster_ability:empyreal_lord_cernunnos_lightning_rod",
            "bestiary_4:monster_ability:empyreal_lord_cernunnos_perfect_archer",
            "bestiary_4:monster_ability:empyreal_lord_cernunnos_primal_aura",
            "bestiary_4:monster_ability:empyreal_lord_cernunnos_unbound",
            "bestiary_4:monster_ability:empyreal_lord_grant_spells",
            "bestiary_4:monster_ability:empyreal_lord_improved_resistance",
            "bestiary_4:monster_ability:empyreal_lord_korada_combat_style_master",
            "bestiary_4:monster_ability:empyreal_lord_korada_never_flat_footed",
            "bestiary_4:monster_ability:empyreal_lord_korada_primal_aura",
            "bestiary_4:monster_ability:empyreal_lord_korada_shatter_spells",
            "bestiary_4:monster_ability:empyreal_lord_korada_tranquil_master",
            "bestiary_4:monster_ability:empyreal_lord_powerful_weaponry",
            "bestiary_4:monster_ability:empyreal_lord_primal_aura",
            "bestiary_4:monster_ability:empyreal_lord_seed_of_life",
            "bestiary_4:monster_ability:empyreal_lord_vildeis_primal_aura",
            "bestiary_4:monster_ability:empyreal_lord_vildeis_smite_evil",
            "bestiary_4:monster_ability:empyreal_lord_vildeis_zealous_vision",
            "bestiary_4:monster_ability:formian_hive_mind",
            "bestiary_4:monster_ability:formian_telepathic_caster",
            "bestiary_4:monster_ability:fungal_creature_create_spawn",
            "bestiary_4:monster_ability:fungal_creature_fungal_blood_or_flesh",
            "bestiary_4:monster_ability:fungal_creature_fungal_metabolism",
            "bestiary_4:monster_ability:fungal_creature_fungal_spores",
            "bestiary_4:monster_ability:fungal_creature_poison_spore_cloud",
            "bestiary_4:monster_ability:fungal_creature_poisonous_blood",
            "bestiary_4:monster_ability:fungal_creature_rejuvenation",
            "bestiary_4:monster_ability:grab_medium",
            "bestiary_4:monster_ability:great_old_one_bokrug_critical_poisoning",
            "bestiary_4:monster_ability:great_old_one_bokrug_immortality",
            "bestiary_4:monster_ability:great_old_one_bokrug_poison",
            "bestiary_4:monster_ability:great_old_one_bokrug_spines",
            "bestiary_4:monster_ability:great_old_one_bokrug_toxic_breath",
            "bestiary_4:monster_ability:great_old_one_bokrug_unspeakable_presence",
            "bestiary_4:monster_ability:great_old_one_bokrug_vengeful_dreams",
            "bestiary_4:monster_ability:great_old_one_cthulhu_cleaving_claws",
            "bestiary_4:monster_ability:great_old_one_cthulhu_dreams_of_madness",
            "bestiary_4:monster_ability:great_old_one_cthulhu_greater_starflight",
            "bestiary_4:monster_ability:great_old_one_cthulhu_immortality",
            "bestiary_4:monster_ability:great_old_one_cthulhu_non_euclidean",
            "bestiary_4:monster_ability:great_old_one_cthulhu_unspeakable_presence",
            "bestiary_4:monster_ability:great_old_one_hastur_fulvous_dreams",
            "bestiary_4:monster_ability:great_old_one_hastur_immortality",
            "bestiary_4:monster_ability:great_old_one_hastur_reveal_visage",
            "bestiary_4:monster_ability:great_old_one_hastur_tattered_lash",
            "bestiary_4:monster_ability:great_old_one_hastur_unspeakable_presence",
            "bestiary_4:monster_ability:great_old_one_hastur_wish",
            "bestiary_4:monster_ability:great_old_one_hastur_yellow_sign",
            "bestiary_4:monster_ability:great_old_one_immortality",
            "bestiary_4:monster_ability:great_old_one_immunities",
            "bestiary_4:monster_ability:great_old_one_insanity",
            "bestiary_4:monster_ability:great_old_one_mythic",
            "bestiary_4:monster_ability:great_old_one_otherworldly_insight",
            "bestiary_4:monster_ability:great_old_one_overcome_damage_reduction",
            "bestiary_4:monster_ability:great_old_one_unspeakable_presence",
            "bestiary_4:monster_ability:hide_in_plain_sight",
            "bestiary_4:monster_ability:immunity_to_dismissal",
            "bestiary_4:monster_ability:kaiju_agyra_blinding_flash",
            "bestiary_4:monster_ability:kaiju_agyra_breath_weapon",
            "bestiary_4:monster_ability:kaiju_agyra_electrified_corpse",
            "bestiary_4:monster_ability:kaiju_agyra_hurricane",
            "bestiary_4:monster_ability:kaiju_agyra_rebirth",
            "bestiary_4:monster_ability:kaiju_agyra_storm_flier",
            "bestiary_4:monster_ability:kaiju_agyra_swift_flight",
            "bestiary_4:monster_ability:kaiju_agyra_thunderous_blast",
            "bestiary_4:monster_ability:kaiju_bezravnis_burrowing_charge",
            "bestiary_4:monster_ability:kaiju_bezravnis_heat_beam",
            "bestiary_4:monster_ability:kaiju_bezravnis_poison",
            "bestiary_4:monster_ability:kaiju_bezravnis_web",
            "bestiary_4:monster_ability:kaiju_hurl_foe",
            "bestiary_4:monster_ability:kaiju_massive",
            "bestiary_4:monster_ability:kaiju_mogaru_absorb_energy",
            "bestiary_4:monster_ability:kaiju_mogaru_breath_weapon",
            "bestiary_4:monster_ability:kaiju_mogaru_firebolts",
            "bestiary_4:monster_ability:kaiju_mogaru_reflexive_breath",
            "bestiary_4:monster_ability:kaiju_mogaru_sense_kaiju",
            "bestiary_4:monster_ability:kaiju_mogaru_susceptible_to_song",
            "bestiary_4:monster_ability:kaiju_overcome_damage_reduction",
            "bestiary_4:monster_ability:kaiju_recovery",
            "bestiary_4:monster_ability:mummified_creature_burst_of_vengeance",
            "bestiary_4:monster_ability:mummified_creature_dust_stroke",
            "bestiary_4:monster_ability:mummified_creature_fail_safe",
            "bestiary_4:monster_ability:mummified_creature_frightful_presence",
            "bestiary_4:monster_ability:mummified_creature_powerful_attacks",
            "bestiary_4:monster_ability:nightmare_creature_fear_aura",
            "bestiary_4:monster_ability:nightmare_creature_feign_death",
            "bestiary_4:monster_ability:nightmare_creature_frightful_presence",
            "bestiary_4:monster_ability:nightmare_creature_illusion_resistance",
            "bestiary_4:monster_ability:nightmare_creature_night_terrors",
            "bestiary_4:monster_ability:nightmare_creature_regeneration",
            "bestiary_4:monster_ability:nightmare_lord_dream_slave",
            "bestiary_4:monster_ability:nightmare_lord_nightmare_magic",
            "bestiary_4:monster_ability:pod_spawned_loss_of_magic",
            "bestiary_4:monster_ability:pod_spawned_mimic",
            "bestiary_4:monster_ability:powerful_blows_claw",
            "bestiary_4:monster_ability:powerful_blows_sting",
            "bestiary_4:monster_ability:powerful_blows_tentacle",
            "bestiary_4:monster_ability:protection_from_good_constant",
            "bestiary_4:monster_ability:psychopomp_spirit_touch",
            "bestiary_4:monster_ability:psychopomp_spiritsense",
            "bestiary_4:monster_ability:regeneration_deific_or_mythic",
            "bestiary_4:monster_ability:regeneration_epic_and_evil_energy",
            "bestiary_4:monster_ability:resistance_to_negative_energy_output",
            "bestiary_4:monster_ability:sanctuary_constant",
            "bestiary_4:monster_ability:shadow_creature_shadow_blend",
            "bestiary_4:monster_ability:shadow_lord_cloying_gloom_blast",
            "bestiary_4:monster_ability:shadow_lord_incorporeal_step",
            "bestiary_4:monster_ability:shadow_lord_planar_thinning",
            "bestiary_4:monster_ability:shadow_lord_shadow_summoning",
            "bestiary_4:monster_ability:shadow_lord_touch_attack",
            "bestiary_4:monster_ability:spawn_of_yog_sothoth_devastation",
            "bestiary_4:monster_ability:spawn_of_yog_sothoth_invisibility",
            "bestiary_4:monster_ability:star_spawn_of_cthulhu_immortality",
            "bestiary_4:monster_ability:star_spawn_of_cthulhu_limited_starflight",
            "bestiary_4:monster_ability:star_spawn_of_cthulhu_overwhelming_mind",
            "bestiary_4:monster_ability:traits_output_colossus",
            "bestiary_4:monster_ability:traits_output_demon_lord",
            "bestiary_4:monster_ability:traits_output_empyreal_lord",
            "bestiary_4:monster_ability:traits_output_formian",
            "bestiary_4:monster_ability:traits_output_great_old_one",
            "bestiary_4:monster_ability:traits_output_kaiju",
            "bestiary_4:monster_ability:traits_output_psychopomp",
            "bestiary_4:monster_ability:universal_monster_rule_darkvision_extraordinary",
            "bestiary_4:monster_ability:universal_monster_rule_darkvision_supernatural",
            "bestiary_4:monster_ability:vampire_nosferatu_blood_drain",
            "bestiary_4:monster_ability:vampire_nosferatu_dominate",
            "bestiary_4:monster_ability:vampire_nosferatu_escape",
            "bestiary_4:monster_ability:vampire_nosferatu_spider_climb",
            "bestiary_4:monster_ability:vampire_nosferatu_swarm_form",
            "bestiary_4:monster_ability:vampire_nosferatu_telekinesis",
            "bestiary_4:monster_ability:vampire_nosferatu_telepathy",
            "bestiary_4:monster_ability:vampire_nosferatu_weaknesses",
            "bestiary_4:monster_ability:werebat",
            "bestiary_4:monster_ability:werecrocodile",
            "bestiary_4:monster_ability:werecrocodile_death_roll",
            "bestiary_4:monster_ability:werecrocodile_sprint",
            "bestiary_4:monster_ability:wereshark",
            "bestiary_4:monster_ability:wyrwood_construct_traits",
        ],
    ),
    (
        "horror_adventures",
        "monster_abilities",
        // `decisions.md §20` (no_record-to-zero wave 2 follow-on): 56
        // `monster_ability` rows no monster row of this book claims now SHIP
        // with `owners: &[]` instead of being dropped by
        // `scripts/transcribe_monster_tables.py`'s orphan pass, because an
        // un-ingested row's shape cannot be measured and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks
        // a monster's own `ability_keys`, so an owner-less record here reaches
        // no screen at all -- not the stub class `decisions.md §44.2` was
        // written about. Pinned by exact key rather than by count so a NEW
        // silent non-reach still fails here; `horror_adventures::tests::
        // every_owner_less_ability_is_a_named_and_pinned_non_reach` (digest
        // 0x4db7_998b_4652_eb60) pins the identical set from the corpus side. Re-derive:
        // `python3 scripts/transcribe_monster_tables.py horror_adventures 2>&1 >/dev/null
        // | grep 'orphan ability row'`.
        &[
            "horror_adventures:monster_ability:accursed_creature_cursed",
            "horror_adventures:monster_ability:bestial_werewolf_change_shape",
            "horror_adventures:monster_ability:bestial_werewolf_feral_counter",
            "horror_adventures:monster_ability:cursed_lord_immortal_curse",
            "horror_adventures:monster_ability:cursed_lord_trapped",
            "horror_adventures:monster_ability:dread_lord_all_seeing",
            "horror_adventures:monster_ability:dread_lord_dream_dominion",
            "horror_adventures:monster_ability:dread_lord_fear_aura",
            "horror_adventures:monster_ability:dread_lord_landlocked",
            "horror_adventures:monster_ability:dread_lord_magical_mastery",
            "horror_adventures:monster_ability:dread_lord_master_of_the_four_winds",
            "horror_adventures:monster_ability:dread_lord_one_with_the_land",
            "horror_adventures:monster_ability:dread_lord_physical_mastery",
            "horror_adventures:monster_ability:dread_lord_plant_affinity",
            "horror_adventures:monster_ability:dread_lord_unquestioned_ruler",
            "horror_adventures:monster_ability:familial_lich_familial_possession",
            "horror_adventures:monster_ability:hellbound_creature_contract_bound",
            "horror_adventures:monster_ability:hellbound_creature_hellbound_summon",
            "horror_adventures:monster_ability:hive_blind",
            "horror_adventures:monster_ability:hive_corrosive_blood",
            "horror_adventures:monster_ability:hive_death_throes",
            "horror_adventures:monster_ability:hive_heat_adaptability",
            "horror_adventures:monster_ability:hive_hive_mind",
            "horror_adventures:monster_ability:implacable_stalker_fear_aura",
            "horror_adventures:monster_ability:implacable_stalker_gory_display",
            "horror_adventures:monster_ability:implacable_stalker_nightmare_resurrection",
            "horror_adventures:monster_ability:implacable_stalker_right_behind_you",
            "horror_adventures:monster_ability:implacable_stalker_sense_fear",
            "horror_adventures:monster_ability:implacable_stalker_terrifying_inevitability",
            "horror_adventures:monster_ability:kyton_apostle_agonizing_prayer",
            "horror_adventures:monster_ability:kyton_apostle_bleeding_touch",
            "horror_adventures:monster_ability:kyton_apostle_seductive_oration",
            "horror_adventures:monster_ability:kyton_apostle_shadow_traveler",
            "horror_adventures:monster_ability:kyton_apostle_spell_like_abilities",
            "horror_adventures:monster_ability:kyton_apostle_unnerving_gaze",
            "horror_adventures:monster_ability:lich_creature_touch_attack",
            "horror_adventures:monster_ability:lycanthropic_creature_change_shape",
            "horror_adventures:monster_ability:moonbound_werewolf_moonbound",
            "horror_adventures:monster_ability:moonbound_werewolf_moonbound_full_moon",
            "horror_adventures:monster_ability:moonbound_werewolf_moonbound_near_full_moon",
            "horror_adventures:monster_ability:moonbound_werewolf_moonbound_new_moon",
            "horror_adventures:monster_ability:moonbound_werewolf_moonbound_other_moon",
            "horror_adventures:monster_ability:possessed_creature_two_minds",
            "horror_adventures:monster_ability:promethean_creature_promethean_metabolism",
            "horror_adventures:monster_ability:shadowbound_creature_refuge_in_pain",
            "horror_adventures:monster_ability:shadowbound_creature_regretful_gaze",
            "horror_adventures:monster_ability:shadowbound_creature_weaver_of_lies",
            "horror_adventures:monster_ability:traits_output_deep_one",
            "horror_adventures:monster_ability:traits_output_hive",
            "horror_adventures:monster_ability:trompe_l_oeil_armor_class",
            "horror_adventures:monster_ability:trompe_l_oeil_attacks",
            "horror_adventures:monster_ability:trompe_l_oeil_autotelic",
            "horror_adventures:monster_ability:trompe_l_oeil_enter_painting",
            "horror_adventures:monster_ability:trompe_l_oeil_rejuvenation",
            "horror_adventures:monster_ability:trompe_l_oeil_skills",
            "horror_adventures:monster_ability:undead_phantom_channel_resistance",
            "horror_adventures:monster_ability:undead_phantom_cling_of_the_grave",
            "horror_adventures:monster_ability:undead_phantom_lifedrinker",
            "horror_adventures:monster_ability:undead_phantom_unnatural_aura",
            "horror_adventures:monster_ability:unknown_assume_likeness",
            "horror_adventures:monster_ability:unknown_dream_movements",
            "horror_adventures:monster_ability:unknown_hallucinatory_camouflage",
            "horror_adventures:monster_ability:unknown_psyche_erosion",
            "horror_adventures:monster_ability:unknown_spell_like_abilities",
            "horror_adventures:monster_ability:unknown_victimize",
        ],
    ),
    (
        "ultimate_wilderness",
        "monster_abilities",
        // `decisions.md §20` no_record-to-zero, round 3: all 2 of this book's
        // `monster_ability` records SHIP with `owners: &[]` -- ultimate_wilderness has ZERO
        // monster rows of its own (`scripts/classify_monster_ability_rows.py`'s
        // "ZERO-monster books" line), so nothing in this book can ever own an
        // ability row, and every one is shipped anyway per `decisions.md §20`:
        // an un-ingested row's shape cannot be measured, and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks a
        // monster's own `ability_keys`, so an owner-less record here reaches no
        // screen at all -- not the stub class `decisions.md §44.2` was written
        // about. Pinned by exact key rather than by count so a NEW silent
        // non-reach still fails here. Re-derive: `python3 scripts/transcribe_monster_tables.py ultimate_wilderness 2>&1 >/dev/null`.
        &[
            "ultimate_wilderness:monster_ability:plant_traits_output_pc",
            "ultimate_wilderness:monster_ability:traits_output_leshy_pc",
        ],
    ),
    (
        "ultimate_intrigue",
        "monster_abilities",
        // `decisions.md §20` no_record-to-zero, round 3: all 6 of this book's
        // `monster_ability` records SHIP with `owners: &[]` -- ultimate_intrigue has ZERO
        // monster rows of its own (`scripts/classify_monster_ability_rows.py`'s
        // "ZERO-monster books" line), so nothing in this book can ever own an
        // ability row, and every one is shipped anyway per `decisions.md §20`:
        // an un-ingested row's shape cannot be measured, and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks a
        // monster's own `ability_keys`, so an owner-less record here reaches no
        // screen at all -- not the stub class `decisions.md §44.2` was written
        // about. Pinned by exact key rather than by count so a NEW silent
        // non-reach still fails here. Re-derive: `python3 scripts/transcribe_monster_tables.py ultimate_intrigue 2>&1 >/dev/null`.
        &[
            "ultimate_intrigue:monster_ability:fey_unchained_eidolon_lvl01",
            "ultimate_intrigue:monster_ability:fey_unchained_eidolon_lvl04",
            "ultimate_intrigue:monster_ability:fey_unchained_eidolon_lvl08",
            "ultimate_intrigue:monster_ability:fey_unchained_eidolon_lvl12",
            "ultimate_intrigue:monster_ability:fey_unchained_eidolon_lvl16",
            "ultimate_intrigue:monster_ability:fey_unchained_eidolon_lvl20",
        ],
    ),
    (
        "ultimate_magic",
        "monster_abilities",
        // `decisions.md §20` no_record-to-zero, round 3: all 13 of this book's
        // `monster_ability` records SHIP with `owners: &[]` -- ultimate_magic has ZERO
        // monster rows of its own (`scripts/classify_monster_ability_rows.py`'s
        // "ZERO-monster books" line), so nothing in this book can ever own an
        // ability row, and every one is shipped anyway per `decisions.md §20`:
        // an un-ingested row's shape cannot be measured, and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks a
        // monster's own `ability_keys`, so an owner-less record here reaches no
        // screen at all -- not the stub class `decisions.md §44.2` was written
        // about. Pinned by exact key rather than by count so a NEW silent
        // non-reach still fails here. Re-derive: `python3 scripts/transcribe_monster_tables.py ultimate_magic 2>&1 >/dev/null`.
        &[
            "ultimate_magic:monster_ability:animated_object_augmented_critical_multiplier",
            "ultimate_magic:monster_ability:animated_object_augmented_critical_range",
            "ultimate_magic:monster_ability:animated_object_exceptional_reach",
            "ultimate_magic:monster_ability:animated_object_exceptional_reach_all",
            "ultimate_magic:monster_ability:animated_object_improved_attack_melee",
            "ultimate_magic:monster_ability:animated_object_improved_attack_ranged",
            "ultimate_magic:monster_ability:animated_object_piercing_attack",
            "ultimate_magic:monster_ability:animated_object_piercing_attack_all",
            "ultimate_magic:monster_ability:animated_object_ranged_attack",
            "ultimate_magic:monster_ability:animated_object_ranged_attack_all",
            "ultimate_magic:monster_ability:animated_object_slashing_attack",
            "ultimate_magic:monster_ability:animated_object_slashing_attack_all",
            "ultimate_magic:monster_ability:animated_object_trip",
        ],
    ),
    (
        "bestiary_6",
        "monster_abilities",
        // `decisions.md §20` no_record-to-zero, round 3: all 16 of this book's
        // `monster_ability` records SHIP with `owners: &[]` -- bestiary_6 has ZERO
        // monster rows of its own (`scripts/classify_monster_ability_rows.py`'s
        // "ZERO-monster books" line), so nothing in this book can ever own an
        // ability row, and every one is shipped anyway per `decisions.md §20`:
        // an un-ingested row's shape cannot be measured, and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks a
        // monster's own `ability_keys`, so an owner-less record here reaches no
        // screen at all -- not the stub class `decisions.md §44.2` was written
        // about. Pinned by exact key rather than by count so a NEW silent
        // non-reach still fails here. Re-derive: `python3 scripts/transcribe_monster_tables.py bestiary_6 2>&1 >/dev/null`.
        &[
            "bestiary_6:monster_ability:coral_capuchin_cursed_bite",
            "bestiary_6:monster_ability:coral_capuchin_moisture_dependency",
            "bestiary_6:monster_ability:deinotherium_sweep",
            "bestiary_6:monster_ability:devil_monkey_puncture_armor",
            "bestiary_6:monster_ability:dunkleosteus_gulp",
            "bestiary_6:monster_ability:elasmotherium_impaling_horn",
            "bestiary_6:monster_ability:giant_raven_scavenger",
            "bestiary_6:monster_ability:kentrosaurus_defensive_spikes",
            "bestiary_6:monster_ability:kentrosaurus_impaling_strike",
            "bestiary_6:monster_ability:mockingfey_mock",
            "bestiary_6:monster_ability:mockingfey_sla",
            "bestiary_6:monster_ability:mokele_mbembe_whip_tail",
            "bestiary_6:monster_ability:quetzalcoatlus_razor_sharp_beak",
            "bestiary_6:monster_ability:universal_monster_rule_ferocity",
            "bestiary_6:monster_ability:universal_monster_rule_negative_energy_affinity",
            "bestiary_6:monster_ability:universal_monster_rule_scent",
        ],
    ),
    (
        "bestiary_5",
        "monster_abilities",
        // `decisions.md §20` no_record-to-zero, round 3: all 39 of this book's
        // `monster_ability` records SHIP with `owners: &[]` -- bestiary_5 has ZERO
        // monster rows of its own (`scripts/classify_monster_ability_rows.py`'s
        // "ZERO-monster books" line), so nothing in this book can ever own an
        // ability row, and every one is shipped anyway per `decisions.md §20`:
        // an un-ingested row's shape cannot be measured, and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks a
        // monster's own `ability_keys`, so an owner-less record here reaches no
        // screen at all -- not the stub class `decisions.md §44.2` was written
        // about. Pinned by exact key rather than by count so a NEW silent
        // non-reach still fails here. Re-derive: `python3 scripts/transcribe_monster_tables.py bestiary_5 2>&1 >/dev/null`.
        &[
            "bestiary_5:monster_ability:aether_wysp_lesser_telekinesis",
            "bestiary_5:monster_ability:blue_whale_hold_breath",
            "bestiary_5:monster_ability:brain_mole_brain_drain",
            "bestiary_5:monster_ability:brain_mole_second_sight",
            "bestiary_5:monster_ability:brain_mole_shrouded_mind",
            "bestiary_5:monster_ability:brain_mole_sla",
            "bestiary_5:monster_ability:cameroceras_pressure_adaptation",
            "bestiary_5:monster_ability:chuspiki_air_blast",
            "bestiary_5:monster_ability:chuspiki_basic_aerokinesis",
            "bestiary_5:monster_ability:chuspiki_sla",
            "bestiary_5:monster_ability:chuspiki_wind_blessed",
            "bestiary_5:monster_ability:chuspiki_wind_form",
            "bestiary_5:monster_ability:digmaul_ball_tail",
            "bestiary_5:monster_ability:esipil_bewildering_assault",
            "bestiary_5:monster_ability:esipil_look_of_fear",
            "bestiary_5:monster_ability:esipil_sla",
            "bestiary_5:monster_ability:feather_fall_constant",
            "bestiary_5:monster_ability:kaprosuchus_ramming_snout",
            "bestiary_5:monster_ability:liminal_sprite_repartee",
            "bestiary_5:monster_ability:liminal_sprite_sla",
            "bestiary_5:monster_ability:liminal_sprite_versatile_performance",
            "bestiary_5:monster_ability:narwhal_hold_breath",
            "bestiary_5:monster_ability:narwhal_tusk",
            "bestiary_5:monster_ability:plesiosaurus_ambush_attack",
            "bestiary_5:monster_ability:sahkil_easy_to_call",
            "bestiary_5:monster_ability:sahkil_emotional_focus",
            "bestiary_5:monster_ability:sahkil_look_of_fear",
            "bestiary_5:monster_ability:sahkil_skip_between",
            "bestiary_5:monster_ability:sahkil_spirit_touch",
            "bestiary_5:monster_ability:therizinosaurus_sprint",
            "bestiary_5:monster_ability:therizinosaurus_sweeping_strike",
            "bestiary_5:monster_ability:traits_output_sahkil",
            "bestiary_5:monster_ability:troodon_easily_trained",
            "bestiary_5:monster_ability:universal_monster_rule_unnatural_aura",
            "bestiary_5:monster_ability:water_wysp_drench",
            "bestiary_5:monster_ability:wolliped_spit",
            "bestiary_5:monster_ability:wysp_living_battery",
            "bestiary_5:monster_ability:wysp_resonance",
            "bestiary_5:monster_ability:wysp_servitor",
            "bestiary_5:monster_ability:xiao_sla",
        ],
    ),
    (
        "pathfinder_unchained",
        "monster_abilities",
        // `decisions.md §20` no_record-to-zero, round 4: 69 of this book's
        // `monster_ability` records SHIP with `owners: &[]` -- pathfinder_unchained has
        // ZERO monster rows of its own (`scripts/classify_monster_ability_rows.py`'s
        // "ZERO-monster books" line), so nothing in this book can ever own an
        // ability row, and every one is shipped anyway per `decisions.md §20`:
        // an un-ingested row's shape cannot be measured, and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks a
        // monster's own `ability_keys`, so an owner-less record here reaches no
        // screen at all -- not the stub class `decisions.md §44.2` was written
        // about. Pinned by exact key rather than by count so a NEW silent
        // non-reach still fails here. A further 3 of the book's 72 orphan
        // candidates (`Elemental ~ Unchained Eidolon LVL01/08/20`) are a
        // multi-DESC: shape `parse_desc` refuses rather than mistranscribes --
        // real per-record work, not shipped this round, NOT in this list.
        // Re-derive: `python3 scripts/transcribe_monster_tables.py pathfinder_unchained 2>&1 >/dev/null`.
        &[
            "pathfinder_unchained:monster_ability:agathion_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:agathion_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:agathion_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:agathion_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:agathion_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:agathion_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:angel_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:angel_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:angel_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:angel_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:angel_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:angel_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:archon_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:archon_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:archon_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:archon_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:archon_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:archon_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:azata_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:azata_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:azata_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:azata_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:azata_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:azata_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:daemon_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:daemon_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:daemon_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:daemon_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:daemon_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:daemon_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:demon_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:demon_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:demon_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:demon_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:demon_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:demon_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:devil_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:devil_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:devil_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:devil_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:devil_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:devil_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:div_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:div_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:div_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:div_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:div_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:div_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:elemental_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:elemental_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:elemental_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:elemental_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:elemental_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:elemental_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:inevitable_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:inevitable_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:inevitable_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:inevitable_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:inevitable_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:inevitable_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:protean_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:protean_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:protean_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:protean_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:protean_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:protean_unchained_eidolon_lvl20",
            "pathfinder_unchained:monster_ability:psychopomp_unchained_eidolon_lvl01",
            "pathfinder_unchained:monster_ability:psychopomp_unchained_eidolon_lvl04",
            "pathfinder_unchained:monster_ability:psychopomp_unchained_eidolon_lvl08",
            "pathfinder_unchained:monster_ability:psychopomp_unchained_eidolon_lvl12",
            "pathfinder_unchained:monster_ability:psychopomp_unchained_eidolon_lvl16",
            "pathfinder_unchained:monster_ability:psychopomp_unchained_eidolon_lvl20",
        ],
    ),
    (
        "advanced_race_guide",
        "monster_abilities",
        // `decisions.md §20` no_record-to-zero, round 4: the 1 `monster_ability`
        // record this book contributes SHIPS with `owners: &[]` --
        // advanced_race_guide has ZERO monster rows of its own
        // (`scripts/classify_monster_ability_rows.py`'s "ZERO-monster books"
        // line), so nothing in this book can ever own it, and it is shipped
        // anyway per `decisions.md §20`: an un-ingested row's shape cannot be
        // measured, and Gate 1's DoD needs every unit's shape measured.
        // Pinned by exact key rather than by count so a NEW silent non-reach
        // still fails here. Re-derive:
        // `python3 scripts/transcribe_monster_tables.py advanced_race_guide 2>&1 >/dev/null`.
        &[
            "advanced_race_guide:monster_ability:grippli_toxic_skin_grippli_poison",
        ],
    ),
    (
        "mythic_adventures",
        "monster_abilities",
        // `decisions.md §20` no_record-to-zero, round 5: all 21 of this
        // book's `monster_ability` records SHIP with `owners: &[]` --
        // mythic_adventures has ZERO monster rows of its own
        // (`scripts/classify_monster_ability_rows.py`'s "ZERO-monster books"
        // line), so nothing in this book can ever own an ability row, and
        // every one is shipped anyway per `decisions.md §20`: an
        // un-ingested row's shape cannot be measured, and Gate 1's DoD
        // needs every unit's shape measured. Pinned by exact key rather
        // than by count so a NEW silent non-reach still fails here.
        // Re-derive: `python3 scripts/transcribe_monster_tables.py
        // mythic_adventures 2>&1 >/dev/null`.
        &[
            "mythic_adventures:monster_ability:block_attacks",
            "mythic_adventures:monster_ability:dragon_blood",
            "mythic_adventures:monster_ability:dragon_cantrips",
            "mythic_adventures:monster_ability:dragon_fury",
            "mythic_adventures:monster_ability:dual_initiative",
            "mythic_adventures:monster_ability:feral_savagery",
            "mythic_adventures:monster_ability:fortification",
            "mythic_adventures:monster_ability:greensight",
            "mythic_adventures:monster_ability:lingering_breath",
            "mythic_adventures:monster_ability:mistsight",
            "mythic_adventures:monster_ability:mythic_magic",
            "mythic_adventures:monster_ability:mythic_power",
            "mythic_adventures:monster_ability:poisonous_blood",
            "mythic_adventures:monster_ability:powerful_blows",
            "mythic_adventures:monster_ability:second_save",
            "mythic_adventures:monster_ability:simple_arcane_spellcasting",
            "mythic_adventures:monster_ability:simple_divine_spellcasting",
            "mythic_adventures:monster_ability:smother",
            "mythic_adventures:monster_ability:steal",
            "mythic_adventures:monster_ability:surge",
            "mythic_adventures:monster_ability:x_ray_vision",
        ],
    ),
    (
        "occult_adventures",
        "monster_abilities",
        // `decisions.md §27b` — EVERYTHING: all 5 of this book's
        // `monster_ability` records SHIP with `owners: &[]` -- no monster
        // row's ownership pass claims any of them by name (both owning race
        // rows reference their sub-abilities only via a CATEGORY:Internal
        // umbrella row this generator does not resolve into per-record
        // ownership; see this book's own `monster_data.rs` header). Shipped
        // anyway per `decisions.md §20`/`§27b`: an un-ingested row's shape
        // cannot be measured, and Gate 1's DoD needs every unit's shape
        // measured; the "correctly out of scope" disposition this book
        // carried across four prior cycles was a REACHABILITY finding (a
        // negated `PRECAMPAIGN` gate), never an ingest exemption. Pinned by
        // exact key rather than by count so a NEW silent non-reach still
        // fails here. Re-derive: `python3 scripts/transcribe_monster_tables.py
        // occult_adventures 2>&1 >/dev/null`.
        &[
            "occult_adventures:monster_ability:homunculus_companion_poison",
            "occult_adventures:monster_ability:homunculus_companion_sympathetic_alchemy",
            "occult_adventures:monster_ability:homunculus_companion_telepathic_link",
            "occult_adventures:monster_ability:shikigami_improvised_weapon_mastery",
            "occult_adventures:monster_ability:shikigami_spell_like_abilities",
        ],
    ),
    (
        "inner_sea_bestiary",
        "monster_abilities",
        // `decisions.md §20` (no_record-to-zero wave 2 follow-on): 28
        // `monster_ability` rows no monster row of this book claims now SHIP
        // with `owners: &[]` instead of being dropped by
        // `scripts/transcribe_monster_tables.py`'s orphan pass, because an
        // un-ingested row's shape cannot be measured and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks
        // a monster's own `ability_keys`, so an owner-less record here reaches
        // no screen at all -- not the stub class `decisions.md §44.2` was
        // written about. Pinned by exact key rather than by count so a NEW
        // silent non-reach still fails here; `inner_sea_bestiary::tests::
        // every_owner_less_ability_is_a_named_and_pinned_non_reach` (digest
        // 0x0a4e_0e1f_6775_49cd) pins the identical set from the corpus side. Re-derive:
        // `python3 scripts/transcribe_monster_tables.py inner_sea_bestiary 2>&1 >/dev/null
        // | grep 'orphan ability row'`.
        //
        // `decisions.md §24`/round 7 (+7): the 7 name-PI ability rows at
        // `isb_abilities_race.lst:312-318` this group closes are all
        // orphans -- their namespace shares no monster-kind unit in this
        // book at all, so they join this set under their Codex-generated
        // neutral key.
        &[
            "inner_sea_bestiary:monster_ability:codex_named_unit_monster_ability_inner_sea_bestiary_isb_abilities_race_lst_312",
            "inner_sea_bestiary:monster_ability:codex_named_unit_monster_ability_inner_sea_bestiary_isb_abilities_race_lst_313",
            "inner_sea_bestiary:monster_ability:codex_named_unit_monster_ability_inner_sea_bestiary_isb_abilities_race_lst_314",
            "inner_sea_bestiary:monster_ability:codex_named_unit_monster_ability_inner_sea_bestiary_isb_abilities_race_lst_315",
            "inner_sea_bestiary:monster_ability:codex_named_unit_monster_ability_inner_sea_bestiary_isb_abilities_race_lst_316",
            "inner_sea_bestiary:monster_ability:codex_named_unit_monster_ability_inner_sea_bestiary_isb_abilities_race_lst_317",
            "inner_sea_bestiary:monster_ability:codex_named_unit_monster_ability_inner_sea_bestiary_isb_abilities_race_lst_318",
            "inner_sea_bestiary:monster_ability:blighted_fey_cyth_v_sug_s_unity",
            "inner_sea_bestiary:monster_ability:blighted_fey_daughters_of_arlantia",
            "inner_sea_bestiary:monster_ability:blighted_fey_fungal_rejuvenation",
            "inner_sea_bestiary:monster_ability:blighted_fey_parasitic_bond",
            "inner_sea_bestiary:monster_ability:blighted_fey_tainted_blood",
            "inner_sea_bestiary:monster_ability:blighted_fey_thorn_throw",
            "inner_sea_bestiary:monster_ability:chemnosit_hungry_gaze",
            "inner_sea_bestiary:monster_ability:chemnosit_spines",
            "inner_sea_bestiary:monster_ability:immunity_to_bleed_effects",
            "inner_sea_bestiary:monster_ability:immunity_to_permanent_wounds",
            "inner_sea_bestiary:monster_ability:mana_wastes_mutant_acid_resistance",
            "inner_sea_bestiary:monster_ability:mana_wastes_mutant_acidic_pustules",
            "inner_sea_bestiary:monster_ability:mana_wastes_mutant_breath_weapon",
            "inner_sea_bestiary:monster_ability:mana_wastes_mutant_deformed_arm",
            "inner_sea_bestiary:monster_ability:mana_wastes_mutant_deformed_leg",
            "inner_sea_bestiary:monster_ability:mana_wastes_mutant_disease",
            "inner_sea_bestiary:monster_ability:mana_wastes_mutant_increased_speed",
            "inner_sea_bestiary:monster_ability:mana_wastes_mutant_shattered_hide",
            "inner_sea_bestiary:monster_ability:mana_wastes_mutant_warped_mind",
            "inner_sea_bestiary:monster_ability:noqual_golem_powerful_blows",
            "inner_sea_bestiary:monster_ability:psychopomp_spirit_sense",
            "inner_sea_bestiary:monster_ability:psychopomp_spirit_touch",
            "inner_sea_bestiary:monster_ability:resistance_to_mind_affecting_output",
            "inner_sea_bestiary:monster_ability:vetala_drain_prana",
            "inner_sea_bestiary:monster_ability:vetala_fast_healing",
            "inner_sea_bestiary:monster_ability:vetala_malevolence",
            "inner_sea_bestiary:monster_ability:vetala_possess_corpse",
            "inner_sea_bestiary:monster_ability:vetala_weakness_to_prayer",
            "inner_sea_bestiary:monster_ability:volnagur_blood_rage",
            "inner_sea_bestiary:monster_ability:volnagur_eye_rays",
            "inner_sea_bestiary:monster_ability:volnagur_shatter_silence",
        ],
    ),
    (
        "inner_sea_gods",
        "monster_abilities",
        // `decisions.md §20` (no_record-to-zero wave 2 follow-on): 2
        // `monster_ability` rows no monster row of this book claims now SHIP
        // with `owners: &[]` instead of being dropped by
        // `scripts/transcribe_monster_tables.py`'s orphan pass, because an
        // un-ingested row's shape cannot be measured and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks
        // a monster's own `ability_keys`, so an owner-less record here reaches
        // no screen at all -- not the stub class `decisions.md §44.2` was
        // written about. Pinned by exact key rather than by count so a NEW
        // silent non-reach still fails here; `inner_sea_gods::tests::
        // every_owner_less_ability_is_a_named_and_pinned_non_reach` (digest
        // 0x137d_2d8a_116e_9f2b) pins the identical set from the corpus side. Re-derive:
        // `python3 scripts/transcribe_monster_tables.py inner_sea_gods 2>&1 >/dev/null
        // | grep 'orphan ability row'`.
        //
        // `decisions.md §24`/round 7 (+3): the 3 name-PI ability rows at
        // `isg_abilities_races.lst:43/44/45` this group closes are all
        // orphans -- their namespace claims no monster-kind unit of this
        // book. The group's other 2 units (description-only PI,
        // `Grim White Stag ~ Bugle`/`Thyrlien ~ Starlight Blast` -- clean
        // names) are OWNED, so they do not join this set.
        &[
            "inner_sea_gods:monster_ability:codex_named_unit_monster_ability_inner_sea_gods_isg_abilities_races_lst_43",
            "inner_sea_gods:monster_ability:codex_named_unit_monster_ability_inner_sea_gods_isg_abilities_races_lst_44",
            "inner_sea_gods:monster_ability:codex_named_unit_monster_ability_inner_sea_gods_isg_abilities_races_lst_45",
            "inner_sea_gods:monster_ability:herald_always_armed",
            "inner_sea_gods:monster_ability:herald_emissary",
        ],
    ),
    (
        "inner_sea_world_guide",
        "monster_abilities",
        // `decisions.md §20` (no_record-to-zero wave 2 follow-on): 13
        // `monster_ability` rows no monster row of this book claims now SHIP
        // with `owners: &[]` instead of being dropped by
        // `scripts/transcribe_monster_tables.py`'s orphan pass, because an
        // un-ingested row's shape cannot be measured and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks
        // a monster's own `ability_keys`, so an owner-less record here reaches
        // no screen at all -- not the stub class `decisions.md §44.2` was
        // written about. Pinned by exact key rather than by count so a NEW
        // silent non-reach still fails here; `inner_sea_world_guide::tests::
        // every_owner_less_ability_is_a_named_and_pinned_non_reach` (digest
        // 0xc6a5_c8bd_fa9c_bb39) pins the identical set from the corpus side. Re-derive:
        // `python3 scripts/transcribe_monster_tables.py inner_sea_world_guide 2>&1 >/dev/null
        // | grep 'orphan ability row'`.
        //
        // `decisions.md §24`/round 7 (+3): the 3 name-PI ability rows at
        // `iswg_abilities_race.lst:24/25/27` this group closes are all
        // orphans -- their owning monster row (`iswg_races.lst:13`) is
        // itself `NAMEISPI:YES`-declared and still dropped (`monster`-kind,
        // outside `§24`'s ability-only scope this cycle), so nothing owns
        // them.
        &[
            "inner_sea_world_guide:monster_ability:codex_named_unit_monster_ability_inner_sea_world_guide_iswg_abilities_race_lst_24",
            "inner_sea_world_guide:monster_ability:codex_named_unit_monster_ability_inner_sea_world_guide_iswg_abilities_race_lst_25",
            "inner_sea_world_guide:monster_ability:codex_named_unit_monster_ability_inner_sea_world_guide_iswg_abilities_race_lst_27",
            "inner_sea_world_guide:monster_ability:clockwork_difficult_to_create",
            "inner_sea_world_guide:monster_ability:clockwork_swift_reactions",
            "inner_sea_world_guide:monster_ability:clockwork_winding",
            "inner_sea_world_guide:monster_ability:constant_desecrate",
            "inner_sea_world_guide:monster_ability:constant_water_breathing",
            "inner_sea_world_guide:monster_ability:nascent_demon_lord_aligned_strike",
            "inner_sea_world_guide:monster_ability:nascent_demon_lord_grant_spells",
            "inner_sea_world_guide:monster_ability:sandpoint_devil_bay",
            "inner_sea_world_guide:monster_ability:sandpoint_devil_hellfire_breath",
            "inner_sea_world_guide:monster_ability:sandpoint_devil_kick",
            "inner_sea_world_guide:monster_ability:treerazer_aura_of_corruption",
            "inner_sea_world_guide:monster_ability:treerazer_defoliation",
            "inner_sea_world_guide:monster_ability:treerazer_regeneration",
        ],
    ),
    (
        "ultimate_psionics",
        "monster_abilities",
        // `decisions.md §20` (no_record-to-zero wave 2 follow-on): 64
        // `monster_ability` rows no monster row of this book claims now SHIP
        // with `owners: &[]` instead of being dropped by
        // `scripts/transcribe_monster_tables.py`'s orphan pass, because an
        // un-ingested row's shape cannot be measured and Gate 1's DoD needs
        // every unit's shape measured. `list_monster_catalog` only ever walks
        // a monster's own `ability_keys`, so an owner-less record here reaches
        // no screen at all -- not the stub class `decisions.md §44.2` was
        // written about. Pinned by exact key rather than by count so a NEW
        // silent non-reach still fails here; `ultimate_psionics::tests::
        // every_owner_less_ability_is_a_named_and_pinned_non_reach` (digest
        // 0x20e3_944a_3d90_ea44) pins the identical set from the corpus side. Re-derive:
        // `python3 scripts/transcribe_monster_tables.py ultimate_psionics 2>&1 >/dev/null
        // | grep 'orphan ability row'`.
        &[
            "ultimate_psionics:monster_ability:astral_armor_spikes",
            "ultimate_psionics:monster_ability:astral_blindsight",
            "ultimate_psionics:monster_ability:astral_buff",
            "ultimate_psionics:monster_ability:astral_celerity",
            "ultimate_psionics:monster_ability:astral_cleave",
            "ultimate_psionics:monster_ability:astral_compact_form",
            "ultimate_psionics:monster_ability:astral_concussion",
            "ultimate_psionics:monster_ability:astral_constrict",
            "ultimate_psionics:monster_ability:astral_deflection",
            "ultimate_psionics:monster_ability:astral_dimension_slide",
            "ultimate_psionics:monster_ability:astral_dodge",
            "ultimate_psionics:monster_ability:astral_energy_bolt",
            "ultimate_psionics:monster_ability:astral_extra_attack",
            "ultimate_psionics:monster_ability:astral_extra_buff",
            "ultimate_psionics:monster_ability:astral_extreme_damage_reduction",
            "ultimate_psionics:monster_ability:astral_extreme_deflection",
            "ultimate_psionics:monster_ability:astral_fast_healing",
            "ultimate_psionics:monster_ability:astral_fly",
            "ultimate_psionics:monster_ability:astral_grab",
            "ultimate_psionics:monster_ability:astral_great_cleave",
            "ultimate_psionics:monster_ability:astral_greater_might",
            "ultimate_psionics:monster_ability:astral_heavy_deflection",
            "ultimate_psionics:monster_ability:astral_improved_buff",
            "ultimate_psionics:monster_ability:astral_improved_bull_rush",
            "ultimate_psionics:monster_ability:astral_improved_critical",
            "ultimate_psionics:monster_ability:astral_improved_damage_reduction",
            "ultimate_psionics:monster_ability:astral_improved_fly",
            "ultimate_psionics:monster_ability:astral_improved_might",
            "ultimate_psionics:monster_ability:astral_improved_slam_attack",
            "ultimate_psionics:monster_ability:astral_improved_swim",
            "ultimate_psionics:monster_ability:astral_might",
            "ultimate_psionics:monster_ability:astral_mobility",
            "ultimate_psionics:monster_ability:astral_muscle",
            "ultimate_psionics:monster_ability:astral_natural_invisibility",
            "ultimate_psionics:monster_ability:astral_poison_touch",
            "ultimate_psionics:monster_ability:astral_pounce",
            "ultimate_psionics:monster_ability:astral_power_attack",
            "ultimate_psionics:monster_ability:astral_power_resistance",
            "ultimate_psionics:monster_ability:astral_reach",
            "ultimate_psionics:monster_ability:astral_rend",
            "ultimate_psionics:monster_ability:astral_resistance_to_acid",
            "ultimate_psionics:monster_ability:astral_resistance_to_cold",
            "ultimate_psionics:monster_ability:astral_resistance_to_electricity",
            "ultimate_psionics:monster_ability:astral_resistance_to_fire",
            "ultimate_psionics:monster_ability:astral_resistance_to_sonic",
            "ultimate_psionics:monster_ability:astral_smite",
            "ultimate_psionics:monster_ability:astral_spring_attack",
            "ultimate_psionics:monster_ability:astral_stunning_fist",
            "ultimate_psionics:monster_ability:astral_swim",
            "ultimate_psionics:monster_ability:astral_tail_slap",
            "ultimate_psionics:monster_ability:astral_talons",
            "ultimate_psionics:monster_ability:astral_trample",
            "ultimate_psionics:monster_ability:astral_trip",
            "ultimate_psionics:monster_ability:astral_two_menu_a_abilities",
            "ultimate_psionics:monster_ability:astral_two_menu_b_abilities",
            "ultimate_psionics:monster_ability:astral_utility",
            "ultimate_psionics:monster_ability:astral_warrior_link",
            "ultimate_psionics:monster_ability:astral_whirlwind_attack",
            "ultimate_psionics:monster_ability:energy_touch_acid",
            "ultimate_psionics:monster_ability:energy_touch_cold",
            "ultimate_psionics:monster_ability:energy_touch_electricity",
            "ultimate_psionics:monster_ability:energy_touch_fire",
            "ultimate_psionics:monster_ability:horror_devastating_touch",
            "ultimate_psionics:monster_ability:horror_link",
        ],
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

    /// ANTI-GAMING (Decision 1(a)): the `naturalAttackDamageBonuses` column
    /// this cycle added to `companions_reach`'s payload predicate must not be
    /// what carries any record over the bar. Every companion row that states a
    /// `BONUS:WEAPONPROF=…|DAMAGE|` token also states natural attacks and stat
    /// adjustments, so the new clause is genuine payload a player reads and
    /// nothing more. Asserted rather than assumed — a clause that DID move
    /// records would be reach bought with a column, which is exactly the
    /// inflation this program forbids.
    #[test]
    fn the_damage_bonus_column_moves_no_record_into_reach() {
        let response = crate::companion_catalog::build_companion_catalog();
        let mut with_bonus = 0usize;
        for entry in &response.entries {
            if entry.natural_attack_damage_bonuses.is_empty() {
                continue;
            }
            with_bonus += 1;
            let reaches_without_the_new_clause = entry
                .race_type
                .as_deref()
                .is_some_and(|t| !t.trim().is_empty())
                || entry.size.as_deref().is_some_and(|s| !s.trim().is_empty())
                || entry.source_page.as_deref().is_some_and(|p| !p.trim().is_empty())
                || !entry.speeds.is_empty()
                || entry.monster_class.is_some()
                || entry.natural_armor.is_some()
                || !entry.natural_attacks.is_empty()
                || !entry.stat_adjustments.is_empty()
                || !entry.abilities.is_empty();
            assert!(
                reaches_without_the_new_clause,
                "{} would reach ONLY because of naturalAttackDamageBonuses -- that is reach \
                 bought with a column, not content a player can read",
                entry.key
            );
        }
        assert!(
            with_bonus > 0,
            "no served companion carries a damage-bonus token at all; this test would then be \
             asserting nothing"
        );
    }

    /// The `skillAbilityDiffBonuses` twin of
    /// `the_damage_bonus_column_moves_no_record_into_reach`, same reasoning:
    /// every companion row that states a `BONUS:SKILL|<skills>|<A>-<B>`
    /// token also states `BONUS:STAT` adjustments, so the new clause is
    /// genuine payload a player reads and nothing more.
    #[test]
    fn the_skill_bonus_column_moves_no_record_into_reach() {
        let response = crate::companion_catalog::build_companion_catalog();
        let mut with_skill_bonus = 0usize;
        for entry in &response.entries {
            if entry.skill_ability_diff_bonuses.is_empty() {
                continue;
            }
            with_skill_bonus += 1;
            let reaches_without_the_new_clause = entry
                .race_type
                .as_deref()
                .is_some_and(|t| !t.trim().is_empty())
                || entry.size.as_deref().is_some_and(|s| !s.trim().is_empty())
                || entry.source_page.as_deref().is_some_and(|p| !p.trim().is_empty())
                || !entry.speeds.is_empty()
                || entry.monster_class.is_some()
                || entry.natural_armor.is_some()
                || !entry.natural_attacks.is_empty()
                || !entry.natural_attack_damage_bonuses.is_empty()
                || !entry.stat_adjustments.is_empty()
                || !entry.abilities.is_empty();
            assert!(
                reaches_without_the_new_clause,
                "{} would reach ONLY because of skillAbilityDiffBonuses -- that is reach bought \
                 with a column, not content a player can read",
                entry.key
            );
        }
        assert!(
            with_skill_bonus > 0,
            "no served companion carries a skill-bonus token at all; this test would then be \
             asserting nothing"
        );
    }

    /// The `saveDcFormulas` twin of `the_damage_bonus_column_moves_no_record_
    /// into_reach`/`the_skill_bonus_column_moves_no_record_into_reach`, same
    /// reasoning but on the ABILITY-level predicate (this is the first of
    /// the three columns stated there rather than on the creature): every
    /// companion ability that states a DESC-embedded save-DC formula also
    /// states real `description`/`description_variants` prose (the DESC:
    /// token the formula's own argument list hangs off), so the new clause
    /// is genuine payload a player reads and nothing more.
    #[test]
    fn the_save_dc_formula_column_moves_no_record_into_reach() {
        let response = crate::companion_catalog::build_companion_catalog();
        let mut with_save_dc = 0usize;
        for entry in &response.entries {
            for ability in &entry.abilities {
                if ability.save_dc_formulas.is_empty() {
                    continue;
                }
                with_save_dc += 1;
                let reaches_without_the_new_clause = ability.facet.is_some()
                    || ability.delivery.is_some()
                    || !ability.type_segments.is_empty()
                    || ability.description.as_deref().is_some_and(|d| !d.trim().is_empty())
                    || ability.description_variants.iter().any(|v| !v.text.trim().is_empty())
                    || !ability.stat_adjustments.is_empty()
                    || ability.source_page.is_some();
                assert!(
                    reaches_without_the_new_clause,
                    "{} would reach ONLY because of saveDcFormulas -- that is reach bought with \
                     a column, not content a player can read",
                    ability.key
                );
            }
        }
        assert!(
            with_save_dc > 0,
            "no served companion ability carries a save-DC formula at all; this test would then \
             be asserting nothing"
        );
    }

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
        assert_eq!(
            ingested.len(),
            421,
            "ARG's 421 ingested race-trait records, counted on disk (156 -> 201 by SD-31 Epic \
             1-F2, 2026-08-15, Bestiary 2's 6-race batch; 201 -> 259 by SD-31-E6-F4-002, \
             2026-08-16, ingest_races.rs's own 6-race batch of 58 standard-tier records for \
             Catfolk/Kitsune/Ratfolk/Strix/Suli/Wayang, sharing this book directory with this \
             binary's alternate-trait output for the first time; 259 -> 283 by \
             SD-31-E6-F4-003, 2026-08-16, ingest_race_traits.rs's own real ARG alternate-trait \
             rows for those same 6 races -- Catfolk 6, Kitsune 2, Ratfolk 4, Strix 6, Suli 5, \
             Wayang 1 = 24; 283 -> 321 by SD31-E6-F4-004, 2026-08-17, ingest_races.rs's own \
             4-race follow-on batch of 38 standard-tier records for Gillman/Nagaji/Vanara/\
             Vishkanya; 321 -> 332 by SD31-E6-F4-006, 2026-08-17, ingest_race_traits.rs's own \
             real ARG alternate-trait rows for those same 4 races -- Gillman 5, Nagaji 1, \
             Vanara 3, Vishkanya 2 = 11; 332 -> 350 by SD31-E6-F4-007, 2026-08-17, \
             ingest_races.rs's own 2-race follow-on batch of 18 standard-tier records for \
             Changeling/Samsaran, closing arg_races.lst's full 37-row playable-race roster -- \
             no alternate-trait batch for these 2, neither race has ARG alternate content; \
             350 -> 414 by the Core Essentials removal, 2026-08-18, which re-filed Aasimar's \
             and Tiefling's 64 heritage records here -- 16 selectable heritages (6 Aasimar + \
             10 Tiefling) and the 48 `<Race> Racial Trait`-typed replacement rows they grant, \
             previously read out of data/corpus/core_essentials/race_trait/; 414 -> 421 by \
             SD-32 card-11 T2b lane, 2026-08-23, decisions.md §16 item 2: the 7 `Human ~ \
             Adoptive Parentage` CHOOSE-pool members (Drow, Dwarf, Elf, Gnome, Grippli, \
             Halfling, Orc), previously silently dropped by `parse_row` because they carry no \
             `TYPE:` at all -- see `race_resolver::adoptive_parentage_options` and \
             `AdoptiveParentageOptionDto`)"
        );
        match reach_of(&arg_traits).expect("ARG race traits have a declared claim") {
            Reach::Surfaced { records, .. } => assert_eq!(records, 421),
            other => panic!("every ARG race-trait record must reach a player, got {other:?}"),
        }
    }

    /// **Plagueborn's follow-up, landed.** `decisions.md §37`'s first estimate
    /// of 50 real APG alternates corrected to 1 genuinely new key
    /// (`Half-Orc ~ Plagueborn`, `decisions.md §39`); the other 49 collide
    /// with already-ingested ARG keys and are excluded at ingest time.
    ///
    /// That 1 record was held back — correctly — because
    /// `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS` table did not
    /// know its key, so shipping the corpus record alone would have offered
    /// it in the picker and refused it at character-save time. SD-29's
    /// race-trait extend lane landed both halves, and this test is the
    /// claim DoD item 2 requires for the book's family: it executes, it is
    /// not a pass-by-absence, and it accounts for the record.
    #[test]
    fn apgs_one_genuinely_new_alternate_racial_trait_reaches_a_player() {
        // `apg`, not `advanced_players_guide`: `CORPUS_BOOK_IDS` maps the corpus
        // directory to the book id every claim uses, and APG is one of the
        // entries where the two spellings differ.
        let apg_traits = Family::new("apg", "race_traits");
        assert!(
            corpus_inventory().0.contains(&apg_traits),
            "the data/corpus scan must see data/corpus/advanced_players_guide/race_trait/"
        );
        assert!(full_inventory().contains(&apg_traits), "and it must reach the gate's inventory");

        let ingested = corpus_record_keys("advanced_players_guide", "race_trait");
        assert_eq!(
            ingested.len(),
            1,
            "APG's 1 ingested race-trait record, counted on disk: {ingested:?}"
        );
        match reach_of(&apg_traits).expect("APG race traits have a declared claim") {
            Reach::Surfaced { records, .. } => assert_eq!(records, 1),
            other => panic!("APG's race-trait record must reach a player, got {other:?}"),
        }
    }

    /// **SD-29 race-trait lane, round 2 (`decisions.md §45`).** Inner Sea
    /// Races is the biggest single alternate-racial-trait ingest since ARG's
    /// own, and it needed no new mechanism at all — which is the finding this
    /// round recorded against `decisions.md §44.4`, whose successor queue
    /// ranked two mechanism-blocked books ahead of it.
    ///
    /// This is the claim DoD item 2 requires for the book's family: it
    /// executes against the live IPC builders, it is not a pass-by-absence,
    /// and it accounts for every record — including the one that does not
    /// reach, which is pinned by exact key in [`UNREACHED_RECORD_FINDINGS`]
    /// with its remedy in [`OPEN_FINDINGS`] rather than rounded away.
    #[test]
    fn inner_sea_races_alternate_racial_traits_reach_a_player() {
        let isr_traits = Family::new("inner_sea_races", "race_traits");
        assert!(
            corpus_inventory().0.contains(&isr_traits),
            "the data/corpus scan must see data/corpus/inner_sea_races/race_trait/"
        );
        assert!(full_inventory().contains(&isr_traits), "and it must reach the gate's inventory");

        let ingested = corpus_record_keys("inner_sea_races", "race_trait");
        assert_eq!(
            ingested.len(),
            94,
            "ISR's 94 ingested race-trait records, counted on disk. **Was 72 until 2026-08-12** \
             (SD-29 `decisions.md` 53): `Elf ~ Sovyrian-Born` carries `NAMEISPI:YES`, PCGen's \
             own declaration that the record NAME is Product Identity. A name cannot be \
             redacted, so the row is dropped at ingest rather than screened -- the same \
             ruling the monster lane reached for Inner Sea World Guide's five NAMEISPI rows. \
             71 -> 82 by SD-31 Epic 1-F2 (2026-08-15), Bestiary 2's 6-race batch. \
             82 -> 94 by SD-32 card 11 T2b lane (2026-08-22): `IN_SCOPE_RACES` in \
             `ingest_race_traits.rs` had grown to 34 races across three SD-31 waves but the \
             book was never re-run after, so 10 already-in-scope races' real alternate-trait \
             rows (Catfolk, Gillman, Kitsune, Nagaji, Ratfolk, Strix, Suli x2, Vanara, \
             Vishkanya x2, Wayang = 12) sat un-transcribed on disk -- a stale regen, not new \
             content or a new mechanism."
        );

        // 79 of 94 reach. The shortfall is `Human ~ Tribalistic Languages`
        // plus the five records `UNREACHED_RECORD_FINDINGS` names, pinned by
        // key, both ways, so a SEVENTH unreached record fails here and so
        // does any of these six silently starting to reach.
        match reach_of(&isr_traits).expect("ISR race traits have a declared claim") {
            Reach::NotSurfaced { missing, .. } => {
                let mut missing: Vec<&str> = missing.iter().map(String::as_str).collect();
                missing.sort_unstable();
                assert_eq!(
                    missing,
                    vec![
                        "Human ~ Tribalistic Languages",
                        "Mostly Human ~ Ifrit ~ Languages",
                        "Mostly Human ~ Suli ~ Languages",
                        "Mostly Human ~ Sylph ~ Languages",
                        "Mostly Human ~ Undine ~ Languages",
                        "Suli ~ Trusted Mediator",
                    ],
                    "exactly six ISR records are unreached, and they are the ones OPEN_FINDINGS names \
                     (`Mostly Human ~ Suli ~ Languages` and `Suli ~ Trusted Mediator` added \
                     2026-08-22, SD-32 card 11)"
                );
            }
            other => panic!("ISR's race-trait shortfall must be reported exactly, got {other:?}"),
        }
    }

    /// Horror Adventures' race traits reach a player, all 43 of them.
    ///
    /// SD-29 race-trait lane round 3 (`decisions.md §47`). The book was picked
    /// by running `scripts/classify_race_trait_rows.py` on it *before* the
    /// round committed to it, which is `decisions.md §45.1`'s method applied a
    /// second time rather than a queue transcribed from a doc.
    ///
    /// Unlike every other book this lane has taken, this one has **no**
    /// shortfall, and the assertion is written to make that a claim rather
    /// than a silence: it demands a plain `Reach::Surfaced`, so a future
    /// record that stops reaching fails here by name instead of quietly
    /// widening an already-tolerated `NotSurfaced`.
    #[test]
    fn horror_adventures_alternate_racial_traits_reach_a_player() {
        let ha_traits = Family::new("horror_adventures", "race_traits");
        assert!(
            corpus_inventory().0.contains(&ha_traits),
            "the data/corpus scan must see data/corpus/horror_adventures/race_trait/"
        );
        assert!(full_inventory().contains(&ha_traits), "and it must reach the gate's inventory");

        let ingested = corpus_record_keys("horror_adventures", "race_trait");
        assert_eq!(
            ingested.len(),
            43,
            "HA's 43 ingested race-trait records, counted on disk. Only \
             ha_abilities_race.lst is ingested: support/ha_abilities_race_oa.lst is loaded by \
             the pcc under PRECAMPAIGN:1,INCLUDES=Occult Adventures, a book this repo has not \
             ingested"
        );

        match reach_of(&ha_traits).expect("HA race traits have a declared claim") {
            Reach::Surfaced { .. } => {}
            other => panic!(
                "every HA race-trait record must reach a player; got {other:?}. HA's two \
                 non-selectable rows (`Deep Jungle Halfling ~ Languages`, `... ~ Poison Use`) \
                 are granted by name from `Halfling ~ Deep Jungle`, so unlike ISR's \
                 `Human ~ Tribalistic Languages` they are reachable"
            ),
        }
    }

    /// Aasimar's and Tiefling's heritage traits reach a player, all 64 of them.
    ///
    /// SD-29 race-trait lane round 4 (`decisions.md §49`) landed these under a
    /// `core_essentials` book id. `decisions.md §9` removed that label -- Core
    /// Essentials is not a Pathfinder book -- and the records were re-filed
    /// under `advanced_race_guide`, which is where the Aasimar/Tiefling variant
    /// heritages are actually published and where the same two races' other ARG
    /// alternate-trait records already lived. This test is unchanged in what it
    /// proves; only the book id and the wire code moved.
    ///
    /// They are the only records in the lane that are **majority granted rather
    /// than chosen**: 16 heritages a player picks and 48 replacement rows that
    /// arrive with whichever heritage was picked.
    ///
    /// The count is asserted in both halves, not just in total, because the
    /// two halves fail independently. Losing the heritage selectors would drop
    /// the total to 0; losing the grant link derived from
    /// `<race>_abilities_globalvar_subrace.lst` would drop it to 16 while
    /// leaving 16 perfectly selectable records that change nothing on the
    /// sheet -- the browse-only stub class `decisions.md §44.2` describes, and
    /// the precise failure this shape invites.
    #[test]
    fn aasimar_and_tiefling_heritage_racial_traits_reach_a_player() {
        let arg_traits = Family::new("advanced_race_guide", "race_traits");
        assert!(
            corpus_inventory().0.contains(&arg_traits),
            "the data/corpus scan must see data/corpus/advanced_race_guide/race_trait/"
        );
        assert!(full_inventory().contains(&arg_traits), "and it must reach the gate's inventory");

        // Counted on disk from the two directories the heritage records live
        // in, so this half still fails independently of ARG's other 350 rows.
        let heritage: BTreeSet<String> = ["aasimar", "tiefling"]
            .into_iter()
            .flat_map(|race| {
                corpus_record_keys_in(
                    &repo_root()
                        .join("data/corpus/advanced_race_guide/race_trait")
                        .join(race),
                )
            })
            .collect();
        let subrace: BTreeSet<&String> = heritage
            .iter()
            .filter(|key| {
                key.contains(" ~ ") && SUBRACE_HERITAGE_KEYS.iter().any(|k| key.contains(k))
            })
            .collect();
        assert_eq!(
            subrace.len(),
            64,
            "the 64 ingested heritage-trait records, counted on disk: 16 selectors (6 Aasimar + \
             10 Tiefling) and the 48 `<Race> Racial Trait`-typed replacement rows they grant. \
             races/skinwalker/ carries the same shape and is out of scope -- Skinwalker is not \
             one of the 18 races this project models. Got {subrace:?}"
        );

        // The half that a broken grant link would silently leave standing.
        let menu = crate::race_trait_picker::build_alternate_racial_traits();
        let selectable: Vec<String> = menu
            .races
            .iter()
            .flat_map(|race| race.alternates.iter())
            .filter(|row| subrace.iter().any(|key| *key == &row.key))
            .map(|row| row.key.clone())
            .collect();
        assert_eq!(
            selectable.len(),
            16,
            "16 heritages are offered, not 64: the other 48 are granted by whichever heritage \
             the player picks and are never menu rows. Got {selectable:?}"
        );

        match reach_of(&arg_traits).expect("ARG race traits have a declared claim") {
            Reach::Surfaced { .. } => {}
            other => panic!(
                "every heritage record must reach a player; got {other:?}. A shortfall here \
                 of exactly 48 means the `ABILITY:<Race> Racial Trait|AUTOMATIC|<key>` grant \
                 links derived from <race>_abilities_globalvar_subrace.lst stopped being \
                 written by src/bin/ingest_race_traits.rs"
            ),
        }
    }

    /// The 16 heritage names, spelled as the corpus spells them. Named rather
    /// than pattern-matched so the 64-record assertion above cannot be
    /// satisfied by some other 64 records that happen to land in the same two
    /// directories.
    const SUBRACE_HERITAGE_KEYS: &[&str] = &[
        "Agathion-Blooded",
        "Angel-Blooded",
        "Archon-Blooded",
        "Azata-Blooded",
        "Garuda-Blooded",
        "Peri-Blooded",
        "Asura-Spawn",
        "Daemon-Spawn",
        "Demodand-Spawn",
        "Demon-Spawn",
        "Devil-Spawn",
        "Div-Spawn",
        "Kyton-Spawn",
        "Qlippoth-Spawn",
        "Oni-Spawn",
        "Rakshasa-Spawn",
    ];

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

    /// The Monk shape ("a complete hand-authored table with a string->id
    /// dispatch link silently missing one entry") closure for the `races`
    /// and `monsters` kinds, corpus-wide -- not scoped to the hand-modelled
    /// subsets `RaceId::ALL` (7 CRB races) / `beastiary1::MonsterId::ALL`
    /// (46 Bestiary-1 stat blocks) that a prior wave's own unit tests
    /// covered (`SD-31-corpus-closure-grind/todo/sweeps.md` S1/S2,
    /// `artifacts/MEASURE-TWICE.md` T1: "Race/monster: partial only ...
    /// the other 280 chassis-served monster entries and 31 non-CRB races
    /// were never checked for this shape").
    ///
    /// Two things this test proves, mechanically, that the prior wave's
    /// narrower enum-scoped tests did not:
    ///
    /// 1. Every book with a `race/` or `monster/` directory in the real
    ///    corpus has a `reach_of` dispatch arm -- so no book can be
    ///    silently absent from the check itself (the book-level form of the
    ///    Monk shape).
    /// 2. The non-hand-modelled remainder of each kind (a race book beyond
    ///    CRB, or a monster book beyond Bestiary 1's 46) is served through
    ///    a corpus-derived id set (`race_catalog::ingested_race_ids_for_book`
    ///    reads `corpus.race_keys()`; the monster chassis modules generate
    ///    their roster from the same corpus rows) rather than a second,
    ///    separately hand-authored id table -- so there is no distinct
    ///    string->id dispatch layer for a variant to be missing *from*.
    ///    `every_ingested_family_is_accounted_for` (above) already proves
    ///    every such family reaches or is a written finding; this test
    ///    names races/monsters specifically so a future regression that
    ///    narrows either kind back to its hand-modelled subset fails here,
    ///    by name, rather than silently.
    #[test]
    fn dispatch_gap_race_and_monster_families_all_have_book_level_reach_arms() {
        let (families, unknown) = corpus_inventory();
        assert!(
            unknown.is_empty(),
            "corpus_inventory found directories this gate cannot name: {unknown:?}"
        );

        let mut race_books = Vec::new();
        let mut monster_books = Vec::new();
        for family in &families {
            if family.kind == "races" {
                race_books.push(family.book.clone());
            } else if family.kind == "monsters" {
                monster_books.push(family.book.clone());
            }
        }

        assert!(
            race_books.len() >= 6,
            "expected at least the 6 registered race books (crb, beastiary1, bestiary_2, \
             bestiary_5, bestiary_6, advanced_race_guide); corpus_inventory found {}: {:?}",
            race_books.len(),
            race_books
        );
        assert!(
            monster_books.len() >= 13,
            "expected at least the 13 registered monster books (beastiary1 plus the 12 \
             chassis-served books); corpus_inventory found {}: {:?}",
            monster_books.len(),
            monster_books
        );

        let mut missing_arms = Vec::new();
        for book in race_books.iter().chain(monster_books.iter()) {
            let kind = if race_books.contains(book) { "races" } else { "monsters" };
            let family = Family::new(book, kind);
            if reach_of(&family).is_none() {
                missing_arms.push(family.label());
            }
        }
        assert!(
            missing_arms.is_empty(),
            "book-level Monk-shape dispatch gap: these race/monster families exist in the \
             corpus but have no reach_of arm at all, so their records cannot even be checked \
             for reachability: {}",
            missing_arms.join(", ")
        );

        // The non-CRB / non-Bestiary-1 remainder must actually be present in
        // the population `races_reach`/the monster chassis check against --
        // i.e. this is a corpus-wide claim, not one that quietly stops at
        // the 7-race / 46-monster hand-modelled subset. `ingested_race_ids_
        // for_book` is corpus-derived (reads `corpus.race_keys()`, filtered
        // by book_id) so a non-empty result for a non-CRB book is itself
        // proof there is no separate hand-authored id table gating it.
        for book_id in ["bestiary_2", "bestiary_5", "bestiary_6", "advanced_race_guide"] {
            let ids = crate::race_catalog::ingested_race_ids_for_book(book_id);
            assert!(
                !ids.is_empty(),
                "{book_id}'s races are corpus-derived (ingested_race_ids_for_book), but the \
                 derived set came back empty -- either the book has no races (contradicts \
                 corpus_inventory finding it above) or the corpus-derivation path itself \
                 silently dropped every row, which is exactly the Monk-shape failure mode \
                 this test exists to catch"
            );
        }
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

    /// Bonus Bestiary's two families, per record, against their own corpus
    /// directories.
    ///
    /// Both sides come from genuinely different places -- the record files
    /// written by `gen_book_cache -- bonus_bestiary`, and the live
    /// `list_monster_catalog` response the screen renders -- so a table that
    /// stopped reaching the wire fails here instead of agreeing with itself.
    #[test]
    fn bonus_bestiary_monsters_and_abilities_reach_the_catalog_record_by_record() {
        let monsters = corpus_record_keys("bonus_bestiary", "monster");
        let abilities = corpus_record_keys("bonus_bestiary", "monster_ability");
        assert_eq!(monsters.len(), 14, "re-derived on disk this cycle");
        assert_eq!(abilities.len(), 17, "re-derived on disk this cycle");

        let response = crate::monster_catalog::build_monster_catalog();
        let served_monsters: BTreeSet<String> = response
            .entries
            .iter()
            .filter(|entry| entry.book == "BB")
            .map(|entry| entry.key.clone())
            .collect();
        let served_abilities: BTreeSet<String> = response
            .entries
            .iter()
            .filter(|entry| entry.book == "BB")
            .flat_map(|entry| entry.abilities.iter().map(|a| a.key.clone()))
            .collect();
        assert_eq!(served_monsters, monsters);
        assert_eq!(served_abilities, abilities);

        match reach_of(&Family::new("bonus_bestiary", "monsters")).expect("a claim is declared") {
            Reach::Surfaced { records, surface } => {
                assert_eq!(records, 14);
                assert_eq!(surface, "list_monster_catalog");
            }
            other => panic!("expected all 14 to reach, got {other:?}"),
        }
        match reach_of(&Family::new("bonus_bestiary", "monster_abilities"))
            .expect("a claim is declared")
        {
            Reach::Surfaced { records, surface } => {
                assert_eq!(records, 17);
                assert_eq!(surface, "list_monster_catalog");
            }
            other => panic!("expected all 17 to reach, got {other:?}"),
        }
    }

    /// Every companion book's one family, per record, against its own corpus
    /// directory — the `companion` kind's first reach claim in this repo.
    ///
    /// Deliberately **one** claim per book against a denominator that holds
    /// BOTH structural shapes, unlike the monster pair above: the corpus files
    /// creature rows and ability rows under one kind, so
    /// `data/corpus/<book>/companion/` is one population and a claim per shape
    /// would judge two numerators against one count.
    ///
    /// Counts re-derived on disk this cycle rather than transcribed:
    /// `for b in inner_sea_combat monster_codex inner_sea_intrigue horror_adventures
    /// bestiary_5 bestiary_6 bestiary_2; do echo -n "$b ";
    /// ls data/corpus/$b/companion/*.json | wc -l; done`
    /// -> 10, 15, 11, 2, 57, 26, 16 — which reproduce `docs/work-inventory.json`'s
    /// own companion-unit counts for the same books exactly.
    ///
    /// Bestiary 5's two `support/b5_races_companion_oa.lst` units
    /// (`Familiar (Brain Mole)`, `Familiar (Chuspiki)`) were excluded
    /// through 2026-08-23 on the premise that their `PRECAMPAIGN:1,Occult
    /// Adventures` gate named an uningested book (`decisions.md §47.2`).
    /// Row-19 desktop reach/catalog reds (SD-32, 2026-08-24): that premise
    /// is now false (`CORPUS_BOOK_IDS` carries `occult_adventures`) and
    /// `decisions.md §27b` separately overturned this exact exclusion
    /// shape; both rows are now transcribed
    /// (`rules_tables::bestiary_5::companion_data`) and reach the wire.
    #[test]
    fn every_ingested_companion_book_reaches_the_catalog_record_by_record() {
        let expected: &[(&str, &str, usize)] = &[
            ("inner_sea_combat", "ISC", 10),
            ("monster_codex", "MC", 15),
            ("inner_sea_intrigue", "ISI", 11),
            ("horror_adventures", "HA", 2),
            ("bestiary_5", "B5", 57),
            ("bestiary_6", "B6", 26),
            ("bestiary_2", "B2", 16),
        ];
        for &(book, wire_code, count) in expected {
            let ingested = corpus_record_keys(book, "companion");
            assert_eq!(ingested.len(), count, "{book}: re-derived on disk this cycle");

            let response = crate::companion_catalog::build_companion_catalog();
            let mut served: BTreeSet<String> = response
                .entries
                .iter()
                .filter(|entry| entry.book == wire_code)
                .map(|entry| entry.key.clone())
                .collect();
            served.extend(
                response
                    .entries
                    .iter()
                    .filter(|entry| entry.book == wire_code)
                    .flat_map(|entry| entry.abilities.iter().map(|a| a.key.clone())),
            );
            assert_eq!(served, ingested, "{book}: the wire and the corpus disagree");

            match reach_of(&Family::new(book, "companions")).expect("a claim is declared") {
                Reach::Surfaced { records, surface } => {
                    assert_eq!(records, count, "{book}");
                    assert_eq!(surface, "list_companion_catalog");
                }
                other => panic!("{book}: expected all {count} to reach, got {other:?}"),
            }
        }
    }

    /// Monster Codex's two families, per record, against their own corpus
    /// directories. Same structure as the Bonus Bestiary test above and the
    /// same two independent sides, run for the second chassis book.
    ///
    /// The counts are re-derived rather than transcribed:
    /// `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    /// print(sum(1 for u in d['units'] if u['book']=='monster_codex' and
    /// u['kind']=='monster'))"` -> 2, the same for `monster_ability` -> 3, and
    /// `ls data/corpus/monster_codex/monster*/ | grep -c json` agrees.
    ///
    /// **This is a small book on purpose, not a small ingest.** Monster Codex
    /// carries 2 monster rows in the whole corpus (`loop-instruction.md`'s
    /// corpus-shape notes); a cycle that ingested 2 records here has ingested
    /// the book's entire monster family, not a sample of it.
    #[test]
    fn monster_codex_monsters_and_abilities_reach_the_catalog_record_by_record() {
        let monsters = corpus_record_keys("monster_codex", "monster");
        let abilities = corpus_record_keys("monster_codex", "monster_ability");
        assert_eq!(monsters.len(), 2, "re-derived on disk this cycle");
        assert_eq!(abilities.len(), 3, "re-derived on disk this cycle");

        let response = crate::monster_catalog::build_monster_catalog();
        let served_monsters: BTreeSet<String> = response
            .entries
            .iter()
            .filter(|entry| entry.book == "MC")
            .map(|entry| entry.key.clone())
            .collect();
        let served_abilities: BTreeSet<String> = response
            .entries
            .iter()
            .filter(|entry| entry.book == "MC")
            .flat_map(|entry| entry.abilities.iter().map(|a| a.key.clone()))
            .collect();
        assert_eq!(served_monsters, monsters);
        assert_eq!(served_abilities, abilities);

        match reach_of(&Family::new("monster_codex", "monsters")).expect("a claim is declared") {
            Reach::Surfaced { records, surface } => {
                assert_eq!(records, 2);
                assert_eq!(surface, "list_monster_catalog");
            }
            other => panic!("expected both to reach, got {other:?}"),
        }
        match reach_of(&Family::new("monster_codex", "monster_abilities"))
            .expect("a claim is declared")
        {
            Reach::Surfaced { records, surface } => {
                assert_eq!(records, 3);
                assert_eq!(surface, "list_monster_catalog");
            }
            other => panic!("expected all 3 to reach, got {other:?}"),
        }
    }

    /// Both Book of the Damned volumes, per record, driven off the chassis
    /// registry rather than hand-listed.
    ///
    /// **This is the round's whole-book claim and it is only meaningful because
    /// neither book has an orphan ability row.** An orphan -- an ability row no
    /// monster row of the same book claims -- reaches no screen, so a book with
    /// orphans cannot satisfy a whole-family claim at all. Re-derived, not
    /// assumed:
    ///
    /// ```text
    /// python3 scripts/classify_monster_ability_rows.py \
    ///     book_of_the_damned_volume_1 book_of_the_damned_volume_2
    /// book                          mon  abil row-named prefix ORPHAN
    /// book_of_the_damned_volume_1     5    36        36      0      0
    /// book_of_the_damned_volume_2     4    17        17      0      0
    /// ```
    ///
    /// Counts on the other side come from the inventory's own units:
    /// `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    /// print(sum(1 for u in d['units'] if
    /// u['book']=='book_of_the_damned_volume_1' and u['kind']=='monster'))"`
    /// -> 5, `monster_ability` -> 36; volume 2 -> 4 and 17.
    #[test]
    fn both_book_of_the_damned_volumes_reach_the_catalog_record_by_record() {
        for (book, wire_code, monster_count, ability_count) in [
            ("book_of_the_damned_volume_1", "BOTD1", 5usize, 36usize),
            ("book_of_the_damned_volume_2", "BOTD2", 4, 17),
        ] {
            let monsters = corpus_record_keys(book, "monster");
            let abilities = corpus_record_keys(book, "monster_ability");
            assert_eq!(monsters.len(), monster_count, "{book}: re-derived on disk this cycle");
            assert_eq!(abilities.len(), ability_count, "{book}: re-derived on disk this cycle");

            let response = crate::monster_catalog::build_monster_catalog();
            let served_monsters: BTreeSet<String> = response
                .entries
                .iter()
                .filter(|entry| entry.book == wire_code)
                .map(|entry| entry.key.clone())
                .collect();
            let served_abilities: BTreeSet<String> = response
                .entries
                .iter()
                .filter(|entry| entry.book == wire_code)
                .flat_map(|entry| entry.abilities.iter().map(|a| a.key.clone()))
                .collect();
            assert_eq!(served_monsters, monsters, "{book}: served monsters");
            assert_eq!(served_abilities, abilities, "{book}: served abilities");

            match reach_of(&Family::new(book, "monsters")).expect("a claim is declared") {
                Reach::Surfaced { records, surface } => {
                    assert_eq!(records, monster_count);
                    assert_eq!(surface, "list_monster_catalog");
                }
                other => panic!("{book}: expected every monster to reach, got {other:?}"),
            }
            match reach_of(&Family::new(book, "monster_abilities")).expect("a claim is declared") {
                Reach::Surfaced { records, surface } => {
                    assert_eq!(records, ability_count);
                    assert_eq!(surface, "list_monster_catalog");
                }
                other => panic!("{book}: expected every ability to reach, got {other:?}"),
            }
        }
    }

    /// Inner Sea World Guide, per record — and the first book in this lane
    /// whose served set is deliberately SMALLER than its corpus row count, for
    /// two independent reasons.
    ///
    /// **Product Identity.** Five monster rows carry `NAMEISPI:YES`, PCGen's own
    /// per-record marker that the NAME is Product Identity, and three ability
    /// rows match `PI_BLACKLIST_TERMS` outright. A key cannot be redacted, so
    /// they are not ingested at all. The eight are pinned by corpus line, never
    /// by name, in `rules_tables::inner_sea_world_guide`'s
    /// `the_eight_product_identity_rows_are_not_records` — their names ARE the
    /// Product Identity, and a comment recording a removal has no need to
    /// instantiate what it removed.
    ///
    /// **Orphans.** Thirteen ability rows are then owned by no shipped monster —
    /// five against the whole book (`iswg_templates.lst` templates), eight
    /// cascading from the PI drops. Re-derived:
    ///
    /// ```text
    /// python3 scripts/classify_monster_ability_rows.py inner_sea_world_guide
    /// book                    mon  abil row-named prefix ORPHAN
    /// inner_sea_world_guide    14    30        25      0      5
    /// ```
    ///
    /// So the claim asserts **9 and 14** — every record that ships — rather than
    /// 14 and 30. The corpus unit counts are the inventory's own:
    /// `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    /// print(sum(1 for u in d['units'] if u['book']=='inner_sea_world_guide'
    /// and u['kind']=='monster'))"` -> 14, `monster_ability` -> 30.
    ///
    /// **The excluded rows are asserted absent from the response too.** A claim
    /// that only counted what arrived would pass equally well if a PI name had
    /// quietly been ingested, which is the outcome this exclusion exists to
    /// prevent.
    #[test]
    fn inner_sea_world_guide_reaches_the_catalog_for_every_linked_record() {
        let monsters = corpus_record_keys("inner_sea_world_guide", "monster");
        let abilities = corpus_record_keys("inner_sea_world_guide", "monster_ability");
        assert_eq!(
            monsters.len(),
            9,
            "the 9 shippable rows; the book's other 5 carry NAMEISPI:YES"
        );
        // 14 -> 27 (`decisions.md §20`, no_record-to-zero wave 2 follow-on,
        // +13 owner-less rows -- shape measurable, reachability NOT
        // claimed, pinned by exact key in `UNREACHED_RECORD_FINDINGS` under
        // this same key).
        // 27 -> 30 (`decisions.md §24`/round 7, +3): the 3 name-PI ability
        // rows this group closes now ship under a Codex-generated neutral
        // key instead of being dropped -- the corpus total (30, this
        // module doc's own classifier table above) and the shipped total
        // now agree, because nothing is PI-dropped at the ability level for
        // this book any more (only its 5 monster rows still are).
        assert_eq!(abilities.len(), 30, "every non-PI row on disk for this book/kind");
        let owner_less =
            recorded_unreached(&Family::new("inner_sea_world_guide", "monster_abilities"));
        assert_eq!(owner_less.len(), 16, "the owner-less rows this cycle's mechanism shipped");
        let owned_abilities: BTreeSet<String> =
            abilities.difference(&owner_less).cloned().collect();
        assert_eq!(
            owned_abilities.len(),
            14,
            "the 14 owned, non-PI, non-orphan rows; the book's other 16 are orphaned"
        );

        let response = crate::monster_catalog::build_monster_catalog();
        let served_monsters: BTreeSet<String> = response
            .entries
            .iter()
            .filter(|entry| entry.book == "ISWG")
            .map(|entry| entry.key.clone())
            .collect();
        let served_abilities: BTreeSet<String> = response
            .entries
            .iter()
            .filter(|entry| entry.book == "ISWG")
            .flat_map(|entry| entry.abilities.iter().map(|a| a.key.clone()))
            .collect();
        assert_eq!(served_monsters, monsters, "served monsters");
        assert_eq!(served_abilities, owned_abilities, "served abilities");

        // Not a count and not a name list: the property is checked against the
        // LIVE blacklist, so a term added to `PI_BLACKLIST_TERMS` later fails
        // here rather than shipping quietly, and this file never has to spell a
        // Product Identity name to say it is absent. The five `NAMEISPI:YES`
        // rows, which the term list does not name, are pinned by corpus line in
        // `rules_tables::inner_sea_world_guide`.
        for key in served_monsters.iter().chain(served_abilities.iter()) {
            let lower = key.to_ascii_lowercase();
            for term in codex::rules_core::pi_screening::PI_BLACKLIST_TERMS {
                assert!(
                    !lower.contains(&term.to_ascii_lowercase()),
                    "a served Inner Sea World Guide key matches a Product Identity term; \
                     the record must not be ingested at all"
                );
            }
        }
        for orphan in ["aligned_strike", "grant_spells", "winding", "swift_reactions", "difficult_to_create"] {
            assert!(
                !served_abilities.iter().any(|key| key.contains(orphan)),
                "{orphan} is owned by an iswg_templates.lst template and reaches no monster"
            );
        }

        match reach_of(&Family::new("inner_sea_world_guide", "monsters"))
            .expect("a claim is declared")
        {
            Reach::Surfaced { records, surface } => {
                assert_eq!(records, 9);
                assert_eq!(surface, "list_monster_catalog");
            }
            other => panic!("expected every monster to reach, got {other:?}"),
        }
        // **Superseded `decisions.md §20` (no_record-to-zero wave 2
        // follow-on).** 13 owner-less rows now ship for shape measurement;
        // reach itself does not move for them (`list_monster_catalog` only
        // ever walks a monster's own `ability_keys`), so the claim is
        // `NotSurfaced` naming exactly the 13 -- pinned by exact key in
        // `UNREACHED_RECORD_FINDINGS` under this same key.
        // 13 -> 16 (`decisions.md §24`/round 7, +3): see this test's own
        // earlier comment on the identical delta.
        match reach_of(&Family::new("inner_sea_world_guide", "monster_abilities"))
            .expect("a claim is declared")
        {
            Reach::NotSurfaced { missing, .. } => assert_eq!(
                missing.len(),
                16,
                "expected exactly the 16 owner-less records to be the whole shortfall (the \
                 other 14 all reach)"
            ),
            other => panic!("expected 16 named non-reaches (14 of 30 still reach), got {other:?}"),
        }
    }

    /// Bestiary 2, per record — the lane's largest book by an order of
    /// magnitude, and the first one whose whole monster set is Open Game
    /// Content.
    ///
    /// **Zero Product Identity rows**, which is not an assumption but a
    /// re-derivation: `grep -c 'NAMEISPI:YES' b2_races.lst b2_abilities_race.lst`
    /// → `0` and `0`, and the classifier's own PI column agrees:
    ///
    /// ```text
    /// python3 scripts/classify_monster_ability_rows.py bestiary_2
    /// book         mon  abil row-named prefix ORPHAN   PI
    /// bestiary_2   316   466       398      4     64    0
    /// ```
    ///
    /// `ogl-pi-blacklist.md` §2 predicts exactly this for a
    /// `roleplaying_game/` bestiary: classic SRD monster names are
    /// presumptively Open Game Content, and the Product Identity a bestiary
    /// carries is in the campaign-setting books' proper nouns, not here.
    ///
    /// **Two exclusions, neither of them Product Identity.** Two monster rows
    /// are `<Base>.COPY=<Variant>` derived rows — the only two in the whole
    /// corpus — and state a delta on another record rather than a stat block;
    /// 65 ability rows are owned by no shipped monster row of this book. Both
    /// classes are cited by line in `monster_data.rs`'s generated header and
    /// pinned in `rules_tables::bestiary_2`.
    ///
    /// The claim therefore asserts **314 and 401**, which is what is served,
    /// rather than rounding up to the corpus's 316 and 466.
    ///
    /// Corpus unit counts are the inventory's own, never a line count over the
    /// `.lst` (which reads 322 declared rows for the races file — six more than
    /// the inventory's 316, the difference being `.COPY=` rows its own trap
    /// filters drop):
    /// `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    /// print(sum(1 for u in d['units'] if u['book']=='bestiary_2'
    /// and u['kind']=='monster'))"` → 316, `monster_ability` → 466.
    #[test]
    fn bestiary_2_reaches_the_catalog_for_every_linked_record() {
        let monsters = corpus_record_keys("bestiary_2", "monster");
        let abilities = corpus_record_keys("bestiary_2", "monster_ability");
        assert_eq!(
            monsters.len(),
            314,
            "the 314 rows that state a stat block; the other 2 are `.COPY=` deltas"
        );
        // SD31-E6-F9-005 (transcription lane, wave 12): 401 -> 493 (+92 new
        // monster_ability records transcribed for this book). The prior
        // comment's "65 orphans" figure described a different, un-written
        // raw-candidate population this test does not itself assert on and
        // this cycle did not re-derive -- not restated here to avoid
        // quoting a number nobody has re-checked against the new total.
        // T9 `MonsterAbilityFacet` widening cycle: 493 -> 571 (+78 on disk,
        // via `gen_book_cache -- bestiary_2` catching disk up to the
        // widened Rust table -- 60 of the 78 are genuinely new records this
        // cycle's `transcribe_monster_tables.py` run shipped; the other 18
        // were the pre-existing `CATEGORY:Internal` bundle-owned rows this
        // very test's own `BUNDLE_OWNED_NO_JSON_TWIN_YET` list already named
        // as served-but-not-yet-on-disk -- `gen_book_cache` wrote their JSON
        // twin for the first time, closing that gap incidentally).
        // 571 -> 656 (`decisions.md §20`, no_record-to-zero wave 2
        // follow-on, +85 owner-less rows -- shape measurable, reachability
        // NOT claimed, pinned by exact key in `UNREACHED_RECORD_FINDINGS`
        // under this same key).
        // 656 -> 657 (`decisions.md §22`/round 6, +1): `Tick Swarm ~ Cling`
        // now ingests (a misspelled `TYPE:SpecialAttck` facet segment this
        // book's own parser previously refused). It is OWNED (by
        // `Tick Swarm`), so it joins `owned_abilities`, not `owner_less`.
        // 657 -> 665 (`decisions.md §27`/round 8, +8): the `TYPE:`-facet-
        // vocabulary-gap group closes via the provisional `SpecialQuality`
        // default -- all 8 are namespaced `<Monster> ~ <Ability>` keys whose
        // owner resolves through the existing prefix pass, so all 8 join
        // `owned_abilities`, none `owner_less`.
        // 665 -> 667 (`decisions.md §27b` round 9, +2): `Telepathy ~ Miles`/
        // `Voidworm ~ Change Shape` close via `parse_desc`'s new
        // generalised sixth branch -- both resolve a real owner, so both
        // join `owned_abilities`, none `owner_less`.
        assert_eq!(abilities.len(), 667, "every row on disk for this book/kind");
        let owner_less = recorded_unreached(&Family::new("bestiary_2", "monster_abilities"));
        assert_eq!(owner_less.len(), 85, "the owner-less rows this cycle's mechanism shipped");
        let owned_abilities: BTreeSet<String> =
            abilities.difference(&owner_less).cloned().collect();
        assert_eq!(owned_abilities.len(), 582, "the owned (reachable) rows on disk");

        let response = crate::monster_catalog::build_monster_catalog();
        let served_monsters: BTreeSet<String> = response
            .entries
            .iter()
            .filter(|entry| entry.book == "B2")
            .map(|entry| entry.key.clone())
            .collect();
        let served_abilities: BTreeSet<String> = response
            .entries
            .iter()
            .filter(|entry| entry.book == "B2")
            .flat_map(|entry| entry.abilities.iter().map(|a| a.key.clone()))
            .collect();
        assert_eq!(served_monsters, monsters, "served monsters");
        // `SD31-W21-MONSTER-001`: `served_abilities` now carries 18 MORE keys
        // than `abilities` (the `data/corpus/bestiary_2/monster_ability/*.json`
        // set) — every one owned via the `CATEGORY:Internal` bundle-row
        // ownership hop (`transcribe_monster_tables.py::
        // find_internal_bundle_ability_refs`), served by `list_monster_catalog`
        // straight from the compiled `monster_chassis::MONSTER_BOOKS` table
        // (`build_monster_catalog`, above), which never reads `data/corpus/`
        // at all. Real, on-screen content with no JSON twin yet — asserted as
        // a superset with the exact delta named, not silently widened to a
        // bare `is_subset` that could absorb an unrelated future regression.
        const BUNDLE_OWNED_NO_JSON_TWIN_YET: &[&str] = &[
            "bestiary_2:monster_ability:breath_weapon_cone_of_negative_energy",
            "bestiary_2:monster_ability:breath_weapon_cone_of_sound",
            "bestiary_2:monster_ability:brine_dragon_capsize",
            "bestiary_2:monster_ability:brine_dragon_desiccating_bite",
            "bestiary_2:monster_ability:brine_dragon_painful_strikes",
            "bestiary_2:monster_ability:cloud_dragon_cloud_breath",
            "bestiary_2:monster_ability:cloud_dragon_cloud_form",
            "bestiary_2:monster_ability:cloud_dragon_thundering_bite",
            "bestiary_2:monster_ability:crystal_dragon_razor_sharp",
            "bestiary_2:monster_ability:magma_dragon_magma_breath",
            "bestiary_2:monster_ability:magma_dragon_magma_tomb",
            "bestiary_2:monster_ability:magma_dragon_superheated",
            "bestiary_2:monster_ability:umbral_dragon_create_shadows",
            "bestiary_2:monster_ability:umbral_dragon_energy_drain",
            "bestiary_2:monster_ability:umbral_dragon_ghost_bane",
            "bestiary_2:monster_ability:umbral_dragon_shadow_breath",
            "bestiary_2:monster_ability:umbral_dragon_umbral_scion",
            "bestiary_2:monster_ability:water_breathing",
        ];
        let expected_served: BTreeSet<String> = owned_abilities
            .iter()
            .cloned()
            .chain(BUNDLE_OWNED_NO_JSON_TWIN_YET.iter().map(|s| s.to_string()))
            .collect();
        assert_eq!(served_abilities, expected_served, "served abilities");

        // The same live-blacklist property the Inner Sea World Guide claim
        // checks. Asserting "this book has no Product Identity" by citing a
        // grep that returned 0 today is a statement about today; this fails if
        // a per-book override ever adds a term that one of these 715 keys
        // matches.
        for key in served_monsters.iter().chain(served_abilities.iter()) {
            let lower = key.to_ascii_lowercase();
            for term in codex::rules_core::pi_screening::PI_BLACKLIST_TERMS {
                assert!(
                    !lower.contains(&term.to_ascii_lowercase()),
                    "a served Bestiary 2 key matches a Product Identity term; the record must \
                     not be ingested at all"
                );
            }
        }

        match reach_of(&Family::new("bestiary_2", "monsters")).expect("a claim is declared") {
            Reach::Surfaced { records, surface } => {
                assert_eq!(records, 314);
                assert_eq!(surface, "list_monster_catalog");
            }
            other => panic!("expected every monster to reach, got {other:?}"),
        }
        // **Superseded `decisions.md §20` (no_record-to-zero wave 2
        // follow-on).** 85 owner-less rows now ship for shape measurement
        // (SD31-E6-F9-005: 401 -> 493; T9 `MonsterAbilityFacet` widening:
        // 493 -> 571 owned; this cycle: +85 owner-less = 656 total). Reach
        // itself does not move for the 85 (`list_monster_catalog` only ever
        // walks a monster's own `ability_keys`), so the claim is
        // `NotSurfaced` naming exactly the 85 -- pinned by exact key in
        // `UNREACHED_RECORD_FINDINGS` under this same key.
        match reach_of(&Family::new("bestiary_2", "monster_abilities"))
            .expect("a claim is declared")
        {
            Reach::NotSurfaced { missing, .. } => assert_eq!(
                missing.len(),
                85,
                "expected exactly the 85 owner-less records to be the whole shortfall (the \
                 other 571 all reach)"
            ),
            other => {
                panic!("expected 85 named non-reaches (571 of 656 still reach), got {other:?}")
            }
        }
    }

    /// Bestiary 3, per record — the first book in this lane that loses NO
    /// monster row, and the cleanest by every screen the lane runs.
    ///
    /// ```text
    /// python3 scripts/classify_monster_ability_rows.py bestiary_3
    /// book         mon  abil row-named prefix ORPHAN   PI COPY
    /// bestiary_3   261    40         0     27     13    0    0
    /// ```
    ///
    /// **Zero Product Identity rows** — `grep -c 'NAMEISPI:YES' b3_races.lst
    /// b3_abilities_race.lst` → `0` and `0`, and the classifier's own PI column
    /// agrees, which `ogl-pi-blacklist.md` §2 predicts for a
    /// `roleplaying_game/` bestiary. **Zero `.COPY=` rows** — the only two in
    /// the corpus are Bestiary 2's.
    ///
    /// So the single exclusion class is the 13 orphan ability rows, and the
    /// claim asserts **261 and 27**: all 261 monsters, and the 27 abilities a
    /// monster row of this book actually owns.
    ///
    /// Corpus unit counts are the inventory's own, never a line count over the
    /// `.lst`:
    /// `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    /// print(sum(1 for u in d['units'] if u['book']=='bestiary_3'
    /// and u['kind']=='monster'))"` → 261, `monster_ability` → 40.
    #[test]
    fn bestiary_3_reaches_the_catalog_for_every_linked_record() {
        let monsters = corpus_record_keys("bestiary_3", "monster");
        let abilities = corpus_record_keys("bestiary_3", "monster_ability");
        assert_eq!(
            monsters.len(),
            261,
            "every one of this book's corpus monster rows ships; no PI row, no `.COPY=` delta"
        );
        // T9 `MonsterAbilityFacet` widening cycle: 27 -> 409. The widened
        // facet vocabulary and the multi-`TYPE:`-token parsing fix together
        // unlocked most of this book's population — `rules_tables::
        // bestiary_3::mod.rs`'s own comment carries the full derivation, and
        // `gen_book_cache -- bestiary_3` wrote all 409 to disk, closing the
        // `BUNDLE_OWNED_NO_JSON_TWIN_YET` gap below for every key it names.
        // 409 -> 675 (`decisions.md §20`, no_record-to-zero wave 2
        // follow-on, +266 owner-less rows -- shape measurable, reachability
        // NOT claimed, pinned by exact key in `UNREACHED_RECORD_FINDINGS`
        // under this same key).
        // 675 -> 686 (`decisions.md §27`/round 8, +11): the `TYPE:`-facet-
        // vocabulary-gap group closes via the provisional `SpecialQuality`
        // default. +1 owned (`Adlet ~ Spell-Like Abilities`); +10
        // owner-less (`Asurendra ~ None`, `Lunar/Royal/Water Naga ~
        // Spells`, `Unfettered Eidolon ~ Str/Dex/Con/Int/Wis/Cha`), added to
        // `UNREACHED_RECORD_FINDINGS` above.
        // 686 -> 696 (`decisions.md §27b` round 9, +10, all owner-less):
        // the multi-DESC: parse-refusal group closes -- Jiang-Shi Vampire
        // plus the 9 `Traits Output ~ <Kind>` rows, added to
        // `UNREACHED_RECORD_FINDINGS` above.
        assert_eq!(abilities.len(), 696, "every row on disk for this book/kind");
        let owner_less = recorded_unreached(&Family::new("bestiary_3", "monster_abilities"));
        assert_eq!(owner_less.len(), 286, "the owner-less rows this cycle's mechanism shipped");
        let owned_abilities: BTreeSet<String> =
            abilities.difference(&owner_less).cloned().collect();
        assert_eq!(
            owned_abilities.len(),
            410,
            "the 410 owned, reachable rows on disk after the T9 widening cycle plus round 8's +1"
        );

        let response = crate::monster_catalog::build_monster_catalog();
        let served_monsters: BTreeSet<String> = response
            .entries
            .iter()
            .filter(|entry| entry.book == "B3")
            .map(|entry| entry.key.clone())
            .collect();
        let served_abilities: BTreeSet<String> = response
            .entries
            .iter()
            .filter(|entry| entry.book == "B3")
            .flat_map(|entry| entry.abilities.iter().map(|a| a.key.clone()))
            .collect();
        assert_eq!(served_monsters, monsters, "served monsters");
        // `SD31-W21-MONSTER-001`: `served_abilities` now carries 9 MORE keys
        // than `abilities` (the `data/corpus/bestiary_3/monster_ability/*.json`
        // set) — every one owned via the `CATEGORY:Internal` bundle-row
        // ownership hop, served by `list_monster_catalog` straight from the
        // compiled `monster_chassis::MONSTER_BOOKS` table, which never reads
        // `data/corpus/` at all. Same shape as Bestiary 2's own claim above;
        // see its comment for the full explanation.
        const BUNDLE_OWNED_NO_JSON_TWIN_YET: &[&str] = &[
            "bestiary_3:monster_ability:adhukait_dance_of_disaster",
            "bestiary_3:monster_ability:adhukait_dual_mind",
            "bestiary_3:monster_ability:adhukait_spell_like_abilities",
            "bestiary_3:monster_ability:aghasura_attraction_aura",
            "bestiary_3:monster_ability:aghasura_dual_wielder",
            "bestiary_3:monster_ability:aghasura_infused_weapons",
            "bestiary_3:monster_ability:aghasura_poison",
            "bestiary_3:monster_ability:legion_archon_flames_of_faith",
            "bestiary_3:monster_ability:legion_archon_second_skin",
        ];
        let expected_served: BTreeSet<String> = owned_abilities
            .iter()
            .cloned()
            .chain(BUNDLE_OWNED_NO_JSON_TWIN_YET.iter().map(|s| s.to_string()))
            .collect();
        assert_eq!(served_abilities, expected_served, "served abilities");

        // The same live-blacklist property every chassis book's claim checks:
        // a grep that returned 0 today is a statement about today, and this
        // fails if a per-book override ever adds a term one of these 288 keys
        // matches.
        for key in served_monsters.iter().chain(served_abilities.iter()) {
            let lower = key.to_ascii_lowercase();
            for term in codex::rules_core::pi_screening::PI_BLACKLIST_TERMS {
                assert!(
                    !lower.contains(&term.to_ascii_lowercase()),
                    "a served Bestiary 3 key matches a Product Identity term; the record must \
                     not be ingested at all"
                );
            }
        }

        match reach_of(&Family::new("bestiary_3", "monsters")).expect("a claim is declared") {
            Reach::Surfaced { records, surface } => {
                assert_eq!(records, 261);
                assert_eq!(surface, "list_monster_catalog");
            }
            other => panic!("expected every monster to reach, got {other:?}"),
        }
        // **Superseded `decisions.md §20` (no_record-to-zero wave 2
        // follow-on).** 266 owner-less rows now ship for shape measurement
        // (T9 `MonsterAbilityFacet` widening: 27 -> 409 owned; this cycle:
        // +266 owner-less = 675 total). Reach itself does not move for the
        // 266 (`list_monster_catalog` only ever walks a monster's own
        // `ability_keys`), so the claim is `NotSurfaced` naming exactly the
        // 266 -- pinned by exact key in `UNREACHED_RECORD_FINDINGS` under
        // this same key.
        // 266 -> 276 (`decisions.md §27`/round 8, +10; see this test's own
        // comment above).
        // 276 -> 286 (`decisions.md §27b` round 9, +10, all owner-less; see
        // this test's own comment above).
        match reach_of(&Family::new("bestiary_3", "monster_abilities"))
            .expect("a claim is declared")
        {
            Reach::NotSurfaced { missing, .. } => assert_eq!(
                missing.len(),
                286,
                "expected exactly the 286 owner-less records to be the whole shortfall (the \
                 other 410 all reach)"
            ),
            other => {
                panic!("expected 276 named non-reaches (410 of 686 still reach), got {other:?}")
            }
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
        // Both tables serving this book, exactly as `monsters_reach` unions
        // them (SD-29 Epic 5 round 8, `decisions.md §58.3`). The two halves are
        // pinned separately so neither can vanish behind the other's total: 46
        // SD-22 records carrying `data.id`, 280 chassis records carrying
        // `data.key`.
        let sd22 = corpus_record_ids("beastiary", "monster");
        let chassis = corpus_record_keys("beastiary", "monster");
        assert_eq!(
            sd22.len(),
            46,
            "Bestiary 1's 46 SD-22 monster records, counted on disk (SD28-E16 subset 09 \
             raised this from 41)"
        );
        assert_eq!(
            chassis.len(),
            280,
            "the chassis complement, counted on disk -- see rules_tables::bestiary"
        );
        let mut ingested = sd22;
        ingested.extend(chassis);

        // Filtered to this book (SD-29 Epic 5): the catalog now serves Bonus
        // Bestiary from the same command, and comparing the whole response
        // against one book's directory would fail for a correct reason and
        // stop saying anything about Bestiary 1. The Bonus Bestiary half has
        // its own record-by-record test below, against its own directory.
        let served: BTreeSet<String> = crate::monster_catalog::build_monster_catalog()
            .entries
            .into_iter()
            .filter(|entry| entry.book == "B1")
            .map(|entry| entry.key)
            .collect();
        assert_eq!(
            served, ingested,
            "every record on disk must be served, and nothing may be served that is not on disk"
        );

        match reach_of(&Family::new("beastiary1", "monsters")).expect("a claim is declared") {
            Reach::Surfaced { records, .. } => assert_eq!(records, 326),
            other => panic!("expected all 326 to reach, got {other:?}"),
        }

        // The book's monster ABILITIES, a family it has had records for only
        // since round 8. Same shape as every other chassis book's claim.
        // SD31-E6-F9-005 (transcription lane, wave 12): 323 -> 399 (+76 new
        // monster_ability records transcribed for this book).
        // SD31-W23-MONSTER-001: 399 -> 522 (+123 on disk this cycle's `gen_
        // book_cache` run wrote -- 68 of those were wave 21's own +399->467
        // Rust-table delta, never previously cache-generated to JSON, plus
        // this cycle's own +55 cross-table-owner rows, `decisions.md §58.3`).
        // T9 `MonsterAbilityFacet` widening cycle: 522 -> 529 (+7 more owned,
        // reachable abilities shipped once the widened facet vocabulary and
        // the multi-`TYPE:`-token parsing fix landed;
        // `rules_tables::bestiary::mod.rs`'s own comment carries the full
        // derivation).
        // `decisions.md §20` (no_record-to-zero wave 2): 529 -> 709 on disk
        // (+180 owner-less records, no monster row of this book claims them,
        // now shipped for shape measurement rather than dropped as orphans --
        // `bestiary::tests::every_owner_less_ability_is_a_named_and_pinned_
        // non_reach`). Reach itself does NOT move: `list_monster_catalog`
        // only ever walks a monster's own `ability_keys`
        // (`monster_catalog.rs`), so an owner-less record surfaces nowhere.
        // The 180 are named, individually, in `UNREACHED_RECORD_FINDINGS`
        // under this same `("beastiary1", "monster_abilities")` key.
        //
        // (A same-cycle instrument note: an earlier draft of this comment
        // claimed the 180 already reached `Surfaced` — that reading came
        // from checking `unreached_records_are_exactly_the_recorded_findings`
        // BEFORE `gen_book_cache beastiary` had ever written these 180
        // records to `data/corpus/`, so the live `ingested` denominator did
        // not contain them yet and every one looked "fixed" for having
        // nothing to be un-fixed from. Re-checked after the real regen, via
        // this very test failing `NotSurfaced` naming all 180: they do not
        // reach. `decisions.md §17a`'s own lesson, paid twice in one cycle.)
        // 709 -> 710 (`decisions.md §22`/round 6, +1): `Spectre ~ Create Spawn`
        // now ingests (a comma-delimiter `TYPE:` row this book's own parser
        // previously refused). It is OWNED (by `Spectre`), so it joins the
        // 529-strong reaching set, not the 180 named non-reaches below —
        // `missing.len()` is unchanged.
        // 710 -> 711 (`decisions.md §27`/round 8, +1): `Morlock ~ Sneak
        // Attack` (`TYPE:Internal`, no facet/delivery) now ships with a
        // provisional `SpecialQuality` facet default. It is OWNED (by
        // `Morlock`), so it joins the reaching set too — `missing.len()` is
        // unchanged.
        // 711 -> 733 (`decisions.md §27b` round 9, +22): the multi-DESC:
        // parse-refusal group closes via `parse_desc`'s new generalised
        // sixth branch -- 5 land OWNED (join the reaching set, `missing.len()`
        // unchanged) and 17 land owner-less (join the shortfall below).
        let abilities = corpus_record_keys("beastiary", "monster_ability");
        assert_eq!(abilities.len(), 733, "the chassis's owned ability records on disk");
        match reach_of(&Family::new("beastiary1", "monster_abilities"))
            .expect("a claim is declared")
        {
            Reach::NotSurfaced { missing, .. } => assert_eq!(
                missing.len(),
                197,
                "expected exactly the 197 owner-less records to be the whole shortfall \
                 (the other 536 all reach — `assess` reports the family NotSurfaced as a \
                 whole the instant ANY record is missing, never a partial Surfaced)"
            ),
            other => panic!("expected 197 named non-reaches (536 of 733 still reach), got {other:?}"),
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

