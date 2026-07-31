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
//! union of two independent live sources, so a new book has to defeat both to
//! slip through:
//!
//! 1. **The app's own ingest diagnostic.** `build_corpus_ingest_diagnostic()`
//!    counts every book's real tables and is already shipped to the player.
//!    Every `(book, kind)` pair it reports with a non-zero count must have a
//!    reach claim here.
//! 2. **The record slices in the source tree.** Every book ingest generates
//!    `pub const <NAME>: &[<RecordType>]` slices under
//!    `src/rules_core/rules_tables/`. Those are scanned directly off disk, so
//!    a family that was ingested but never wired into the diagnostic still
//!    shows up. A record type this module does not recognize is itself a
//!    failure — a genuinely new kind of content needs a decision about where
//!    it reaches, not a default.
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
        };
    }

    // Absent entirely is the more severe finding, and is checked first: a
    // family nothing serves has no surface to speak of.
    let seen: BTreeSet<String> = with_payload.union(identity_only).cloned().collect();
    let missing: Vec<String> = ingested.difference(&seen).cloned().collect();
    if !missing.is_empty() {
        return Reach::NotSurfaced {
            why: format!(
                "{} of {} ingested records never appear in `{}` (e.g. {})",
                missing.len(),
                ingested.len(),
                surface,
                sample(missing.into_iter())
            ),
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
    ("SpellListEntry", "spells"),
    ("EquipmentTableEntry", "equipment"),
    ("WeaponTableEntry", "weapons"),
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
];

/// Scan `src/rules_core/rules_tables/` for generated record slices.
///
/// Returns the families found, plus any record type this module does not
/// recognize. An unrecognized type is not skipped: a new kind of ingested
/// content is precisely the event this gate exists for, and defaulting it to
/// "probably fine" would reintroduce the whole defect class on book 5.
///
/// **Known blind spot, stated rather than papered over.** This reads
/// column-zero `pub const NAME: &[Type]` declarations only, so a book whose
/// records live inline inside an accessor function body is invisible to it.
/// `pathfinder_unchained` is exactly that shape — its records sit inside
/// `pub fn equipment_tables()` and `pub fn feat_tables()` — so this scanner
/// still does not see PU.
///
/// **The second half of that blind spot is closed (SD-27, 2026-07-31.)** PU
/// used to be absent from `corpus_ingest_diagnostic` as well, leaving
/// *neither* discovery source able to see it and this gate asserting nothing
/// about the book in either direction. The diagnostic now reports it (and
/// `advanced_race_guide`), so PU's families reach `full_inventory` through
/// source 1, and the remedy this comment used to name — "add PU to the ingest
/// diagnostic, then declare PU's claims so they are actually executed" — has
/// been carried out for `classes`, `feats` and `equipment`, whose claims are
/// declared in `reach_of` and executed against live IPC responses.
/// `class_features` is the one family with no executable claim; it has an
/// OPEN_FINDINGS entry stating exactly why.
///
/// Teaching this scanner the accessor-function shape is still worth doing:
/// two independent sources is the property that makes the inventory hard to
/// fool, and PU currently rests on one.
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

/// Extracts `Foo` from a line declaring `pub const NAME: &[Foo] = ...`.
///
/// Only top-level (column-zero) declarations count — an indented `pub const
/// ALL: &[ClassId]` inside an `impl` block is an enum roster, not an ingested
/// record slice. Tuple and primitive element types (`&[(&str, u8)]`,
/// `&[&str]`) are per-class index tables over records that already exist
/// elsewhere, not record families.
fn slice_element_type(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("pub const ")?;
    let (_name, rest) = rest.split_once(": &[")?;
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

        // `pathfinder_unchained/class_features` is deliberately absent: no
        // claim can be executed for it today, and a claim nobody executes is
        // the thing this gate exists to refuse. See its OPEN_FINDINGS entry.
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
        Err(why) => return Reach::NotSurfaced { why },
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
/// Usually that means the family reaches nothing at all (Bestiary 1's
/// monsters). It can also mean the records reach a surface but no automated
/// check can prove *which* of them do — `pathfinder_unchained/class_features`
/// is that case, and its entry says so in its first sentence. Both belong
/// here because the gate's rule is all-or-nothing by design: it refuses
/// partial credit, so anything short of a fully executed claim is a written
/// finding. Read the entry, never the label alone.
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
        "monsters",
        "Bestiary 1's 41 ingested monster stat blocks reach no surface. The only consumers are \
         `corpus_ingest_diagnostic` (a count) and `cache_gen::beastiary1` (a build-time JSON \
         generator); the React app contains no monster reference at all. The Pets tab does NOT \
         count — its companion stat block is computed by `pilot_compute`'s own \
         `ground_*_companion_stat_block`, not read from these tables. Remedy: a monster catalog \
         command and browser, mirroring `spell_catalog.rs` + SpellCatalogScreen.tsx.",
    ),
    (
        "pathfinder_unchained",
        "class_features",
        "Read the reach state precisely, because a one-line label overstates it: PU's 64 ingested \
         class-feature records DO influence a real player surface — the character sheet's \"Class \
         Features & Special Abilities\" section (CharacterSheet.tsx, via classFeaturesModel.ts) \
         renders the engine's `class_feature.pu.*` receipt rows, and pf1_adapter's \
         `every_unchained_class_reaches_computed_through_the_real_creation_path` proves each of \
         the four classes grounds at least one of its own. What cannot be claimed is WHICH of the \
         64 that covers, and the gate refuses partial credit. The blocker is an identity mismatch, \
         not a missing screen: `pilot_compute` names its receipt rows semantically \
         (`class_feature.pu.unchained_rogue.sneak_attack_dice`) while the corpus record is keyed \
         `Unchained Rogue ~ Sneak Attack`, so nothing can join the two without a hand-written \
         mapping — which would be exactly the unexecuted claim this file forbids. Each class does \
         emit a non-claim-blocking `class_feature.pu.<class>.other_features_deferred.unsupported` \
         row naming its own remainder, which the character sheet's \"Not computed\" lane renders. \
         CORRECTION (2026-07-31): that last sentence used to read \"so the deferred set is visible \
         to a player\", and it was false when written. The record was pushed on the DIAGNOSTIC \
         channel only; the sheet reads `LoadSavedCharacterResponse.explanations`, and a \
         non-claim-blocking diagnostic never reaches the frontend at all, because diagnostics \
         travel only on a Blocked outcome. Driven on screen by the SD-27 verify agent: an \
         Unchained Monk 10 Actions tab jumped straight from its grounded rows to UNIVERSAL LEVEL \
         BENEFITS with no \"Not computed\" section. `pilot_compute::push_deferred_class_features` \
         now emits the record on BOTH channels with one shared text, and \
         `tests/sd27_pu_deferred_features_reach_the_character_sheet.rs` pins it per class, so the \
         sentence above is now true and stays checkable. Remedy for the finding itself is \
         unchanged: carry the corpus feature key on the receipt explanation (or a `feature_key` \
         field beside `id`), then declare a real per-record claim here and delete this entry.",
    ),
];

/// Records that reach a real surface carrying nothing but their own key.
///
/// Pinned by exact key, in both directions, for the same reason
/// [`OPEN_FINDINGS`] is: a thirteenth bare record fails the gate, and fixing
/// one of these fails it too until the key is deleted. A bare *count* would
/// let one record silently swap for another.
const BARE_RECORD_FINDINGS: &[(&str, &str, &[&str])] = &[(
    "apg",
    "spells",
    // These are `.COPY=`-style archetype/variant delta rows in
    // `apg_spells.lst`: they name a parent spell plus a qualifier and carry no
    // `SCHOOL:`, `CLASSES:` or `DESC:` token of their own, so they arrive at
    // `list_spell_catalog` with a key and three nulls and render as a row of
    // empty columns. The other 285 APG spells render completely.
    //
    // Remedy: resolve `.COPY=` inheritance at ingest so a delta row carries
    // its parent's school/level/text, or stop emitting delta rows into
    // `SPELL_LIST` as standalone records. `spell_catalog.rs`'s own module doc
    // already flags the underlying `.COPY=` handling as an open
    // ingest-fidelity question; this is what it costs a player.
    &[
        "Beast Shape I (Animals Only)",
        "Blindness/Deafness (Only Cause Blindness)",
        "Meteor Swarm (Dealing Cold Damage)",
        "Planar Ally (Agathions Only)",
        "Planar Ally (Archon Only)",
        "Planar Ally (Azata Only)",
        "Planar Binding (Daemons Only)",
        "Planar Binding (Demons Only)",
        "Planar Binding (Devils Only)",
        "Planar Binding (Inevitables Only)",
        "Planar Binding (Proteans Only)",
        // Not a variant row like the other eleven: `Wall of Thorms` is a
        // misspelling of `Wall of Thorns` carried verbatim from the corpus.
        // It lands here for the same reason — nothing resolves it, so it has
        // no school, level or text — and it is worth its own look, since a
        // key nothing can match is a different bug from an unresolved delta.
        "Wall of Thorms",
    ],
)];

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

/// The whole ingested inventory, from both independent sources.
fn full_inventory() -> BTreeSet<Family> {
    let mut inventory = diagnostic_inventory();
    inventory.extend(scanned_inventory().0);
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
    fn the_inventory_is_populated_from_both_live_sources() {
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

        // The two sources genuinely differ — the scan sees families the
        // diagnostic never counted. If they ever became identical the second
        // source would be adding nothing, and this asserts they have not.
        let scan_only: Vec<String> = from_scan
            .difference(&from_diagnostic)
            .map(Family::label)
            .collect();
        assert!(
            !scan_only.is_empty(),
            "the source scan added no family the diagnostic missed; verify it is still parsing \
             record slices"
        );
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
                Reach::NotSurfaced { why } => broken.push(format!("{}: {why}", family.label())),
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
    /// player, including Bestiary 1's 49 monsters. If a count satisfied this
    /// gate, every family would trivially pass and all six historical defects
    /// would have gone undetected. It does not: the diagnostic is not a
    /// consumer, and this pins that the monsters are still judged unreached
    /// even though the app demonstrably tells the player how many there are.
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

        assert!(
            reach_of(&monsters).is_none(),
            "the ingest diagnostic must never be accepted as a reach claim — it renders a \
             number, not the records"
        );
        assert!(
            recorded_findings().contains(&monsters),
            "{counted} monsters are counted at the player and rendered nowhere; that is a gap, \
             not reach"
        );
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
            Reach::NotSurfaced { why } => {
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

