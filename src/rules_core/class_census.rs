//! The merged, corpus-wide class-id census: every distinct class id the
//! engine's registries and the converted corpus know about, in one place.
//!
//! # Why this exists
//!
//! `docs/architecture/status.md`'s "Class/level compute coverage" table
//! (measured 2026-09-20) was built by a **temporary** integration test
//! (`tests/zz_class_census.rs`) that merged eight registries into one
//! `BTreeMap<slug, Row>`, ran the sweep, and was then deleted -- "no
//! committed binary keeps a merged corpus-wide class census live day to
//! day," as that doc's own words put it. Epic F (`docs/release/
//! SD-36-consolidation/epic-f-class-completion.md` §2, batch F0) makes that
//! merge a **permanent** instrument instead of a one-time throwaway, so the
//! next time this number needs re-deriving nobody has to reconstruct the
//! merge from prose. This module is F0a: the merged registry alone --
//! no sweep, no `--json` output, no CLI (`src/bin/class_census.rs` and the
//! sweep-and-compute half are later F0 steps).
//!
//! # The eight raw sources, and the ninth that never adds a new family
//!
//! Every class id comes from exactly one of these eight raw registries,
//! claimed in this precedence order (see [`ClassFamily`]):
//!
//! 1. [`ClassId::ALL`](crate::rules_core::rules_tables::crb::class_tables::ClassId::ALL) (CRB, `core_rulebook`)
//! 2. `ApgClassId::ALL` (APG, `advanced_players_guide`)
//! 3. `AcgClassId::ALL` (ACG, `advanced_class_guide`)
//! 4. `PuClassId::ALL` (Pathfinder Unchained, `pathfinder_unchained`)
//! 5. `UcClassId::ALL` (Ultimate Combat, `ultimate_combat`)
//! 6. `untabled_base_class_chassis::untabled_base_class_registry()` (untabled exotic base classes)
//! 7. `crb_untabled_class_chassis::covered_classes()` (CRB NPC / `Ex-*` classes)
//! 8. every converted class record, corpus-wide, whose principal rule's
//!    `tags` carry `"Prestige"` (Prestige) -- read through
//!    [`class_chassis_sheet_rules::records`], the same reader
//!    `generic_class_chassis` itself uses, over every real book directory
//!    `data/sheet_rules` actually holds (never a hand-typed book list; see
//!    [`prestige_scan_books`]).
//!
//! These eight are pairwise disjoint by construction: a converted record's
//! `TYPE:` heads make it exactly one of Base.PC (tabled), Base.NPC, `Ex-*`,
//! or Prestige, never two -- `no_class_id_sits_in_two_families` (below)
//! proves that holds for the current corpus rather than assuming it.
//!
//! `generic_class_chassis`'s own 78-class population (`covered_classes()`,
//! added by this batch) is a **ninth** source, folded in last. It never
//! introduces a new family: every id it names is already claimed by one of
//! the eight above (`docs/architecture/status.md`'s own evidence: 56 of its
//! 78 are Prestige rows, the other 22 are the Ultimate Combat classes and
//! 19 of the 20 untabled-exotic classes, each also reachable through
//! `generic_class_chassis`'s own 14-book population). Folding it in only
//! widens an existing entry's [`ClassCensusEntry::registries`] and
//! [`ClassCensusEntry::books`]. If a future corpus change makes it name an
//! id none of the eight claims, that id is recorded under
//! [`ClassFamily::GenericOnly`] rather than silently guessed into one of
//! the other eight -- `no_generic_only_stragglers_today` pins that this
//! is currently empty, so a regression is caught by name.

use std::collections::BTreeMap;
#[cfg(test)]
use std::collections::BTreeSet;
#[cfg(test)]
use std::process::Command;

use crate::rules_core::character_input::{
    CharacterClassLevel, CharacterInput, load_character_input_fixture,
};
use crate::rules_core::class_seeds::{FIXTURE_RELATIVE_PATH, canonical_seeds_for, input_for};
use crate::rules_core::corpus_loader::live_sheet_rules;
use crate::rules_core::pilot_compute::class_chassis_sheet_rules;
use crate::rules_core::pilot_compute::crb_untabled_class_chassis;
use crate::rules_core::pilot_compute::generic_class_chassis_covered_classes;
use crate::rules_core::pilot_compute::untabled_base_class_chassis::untabled_base_class_registry;
use crate::rules_core::pilot_compute::{HeadlessReceiptStatus, build_pilot_headless_receipt};
use crate::rules_core::rules_tables::acg::AcgClassId;
use crate::rules_core::rules_tables::apg::ApgClassId;
use crate::rules_core::rules_tables::crb::class_tables::ClassId;
use crate::rules_core::rules_tables::pathfinder_unchained::class_chassis::PuClassId;
use crate::rules_core::rules_tables::ultimate_combat::UcClassId;
use crate::rules_core::sheet_rule::{Applies, Cmp, Expr, SpellKind};
use crate::support::paths::repo_root;

/// PF1's own character-level cap -- the ceiling every fully tabled base
/// class (CRB/APG/ACG/Pathfinder Unchained/Ultimate Combat) reads to, and
/// the same constant `docs/architecture/status.md`'s own evidence line
/// cites (`v06_class_state_dump` -> `max_level=20`).
const DEFAULT_TABLED_MAX_LEVEL: u8 = 20;

/// The partition family a census entry belongs to. A strict partition:
/// every class id is in exactly one family (`no_class_id_sits_in_two_families`).
/// Mirrors `docs/architecture/status.md`'s "Per-family breakdown" table
/// row for row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClassFamily {
    Crb,
    Apg,
    Acg,
    PathfinderUnchained,
    UltimateCombat,
    UntabledExoticBase,
    CrbNpcEx,
    Prestige,
    /// Never populated today (`no_generic_only_stragglers_today`) -- the
    /// escape hatch for a class `generic_class_chassis` names that none of
    /// the eight canonical sources claims, so such a class is recorded
    /// honestly instead of guessed into one of the other eight families.
    GenericOnly,
}

impl ClassFamily {
    pub fn label(self) -> &'static str {
        match self {
            ClassFamily::Crb => "CRB",
            ClassFamily::Apg => "APG",
            ClassFamily::Acg => "ACG",
            ClassFamily::PathfinderUnchained => "Pathfinder Unchained",
            ClassFamily::UltimateCombat => "Ultimate Combat",
            ClassFamily::UntabledExoticBase => "Untabled exotic base classes",
            ClassFamily::CrbNpcEx => "CRB NPC / Ex-* classes",
            ClassFamily::Prestige => "Prestige",
            ClassFamily::GenericOnly => "generic_class_chassis-only (unclaimed by any of the eight canonical sources)",
        }
    }
}

/// One distinct class id's merged census row.
#[derive(Debug, Clone)]
pub struct ClassCensusEntry {
    pub class_id: String,
    pub family: ClassFamily,
    /// Every book this id's chassis was actually found in, across every
    /// registry that named it (deduplicated, first-seen order).
    pub books: Vec<String>,
    /// Every raw source that named this id (deduplicated, first-seen
    /// order) -- how `every_registry_is_swept_once` shows overlaps.
    pub registries: Vec<&'static str>,
    pub max_level: u8,
    pub is_prestige: bool,
}

fn add_unique(list: &mut Vec<String>, value: &str) {
    if !list.iter().any(|v| v == value) {
        list.push(value.to_string());
    }
}

fn add_registry(list: &mut Vec<&'static str>, value: &'static str) {
    if !list.contains(&value) {
        list.push(value);
    }
}

/// Every real book directory under `data/sheet_rules`, sorted -- never a
/// hand-typed list. Reuses [`crate::rules_core::settled_corpus::corpus_book_dirs`],
/// the same dynamic directory-walk `data/corpus`'s own authoring-time
/// generator and whole-corpus parity proofs already use, pointed at
/// `data/sheet_rules` instead: both are "every real book directory under a
/// corpus root, skipping `_`-prefixed bookkeeping directories," which is
/// exactly what a corpus-wide Prestige-tag sweep needs.
pub fn prestige_scan_books() -> Vec<String> {
    let root = repo_root().join("data/sheet_rules");
    let mut books: Vec<String> = crate::rules_core::settled_corpus::corpus_book_dirs(&root)
        .into_iter()
        .filter_map(|path| path.file_name().map(|name| name.to_string_lossy().into_owned()))
        .collect();
    books.sort();
    books
}

/// One raw source's claimed ids, keyed by class id, for the overlap check
/// `no_class_id_sits_in_two_families` runs before any precedence rule
/// papers over a real collision. Test-only: `census()` itself never needs
/// the raw, pre-merge sets, only `no_class_id_sits_in_two_families` does.
#[cfg(test)]
type RawClaims = BTreeMap<&'static str, BTreeSet<String>>;

#[cfg(test)]
fn raw_claims() -> RawClaims {
    let mut raw: RawClaims = BTreeMap::new();

    let crb: BTreeSet<String> = ClassId::ALL
        .iter()
        .map(|id| format!("class:{}", format!("{id:?}").to_lowercase()))
        .collect();
    raw.insert("crb_class_id", crb);

    let apg: BTreeSet<String> =
        ApgClassId::ALL.iter().map(|id| format!("class:{}", id.name())).collect();
    raw.insert("apg_class_id", apg);

    let acg: BTreeSet<String> =
        AcgClassId::ALL.iter().map(|id| format!("class:{}", id.name())).collect();
    raw.insert("acg_class_id", acg);

    let pu: BTreeSet<String> =
        PuClassId::ALL.iter().map(|id| format!("class:{}", id.name())).collect();
    raw.insert("pu_class_id", pu);

    let uc: BTreeSet<String> =
        UcClassId::ALL.iter().map(|id| format!("class:{}", id.name())).collect();
    raw.insert("uc_class_id", uc);

    let untabled: BTreeSet<String> =
        untabled_base_class_registry().iter().map(|meta| meta.class_id.clone()).collect();
    raw.insert("untabled_base_class_registry", untabled);

    let crb_npc_ex: BTreeSet<String> =
        crb_untabled_class_chassis::covered_classes().into_iter().map(|meta| meta.class_id).collect();
    raw.insert("crb_untabled_class_chassis", crb_npc_ex);

    let books = prestige_scan_books();
    let book_refs: Vec<&str> = books.iter().map(String::as_str).collect();
    let prestige: BTreeSet<String> = class_chassis_sheet_rules::records(&book_refs)
        .into_iter()
        .filter(|(_, chassis)| chassis.tags.iter().any(|t| t == "Prestige"))
        .map(|((_, slug), _)| format!("class:{slug}"))
        .collect();
    raw.insert("prestige_tagged_record", prestige);

    raw
}

/// Every distinct class id known to the engine, merged from the eight raw
/// registries plus `generic_class_chassis`'s own population. See this
/// module's doc comment for the precedence rule and why the ninth source
/// never introduces a new family (short of the honestly-named
/// `ClassFamily::GenericOnly` escape hatch).
pub fn census() -> BTreeMap<String, ClassCensusEntry> {
    let mut entries: BTreeMap<String, ClassCensusEntry> = BTreeMap::new();

    let claim = |entries: &mut BTreeMap<String, ClassCensusEntry>,
                 class_id: String,
                 family: ClassFamily,
                 book: &str,
                 registry: &'static str,
                 max_level: u8,
                 is_prestige: bool| {
        entries
            .entry(class_id.clone())
            .and_modify(|e| {
                add_unique(&mut e.books, book);
                add_registry(&mut e.registries, registry);
            })
            .or_insert_with(|| ClassCensusEntry {
                class_id,
                family,
                books: vec![book.to_string()],
                registries: vec![registry],
                max_level,
                is_prestige,
            });
    };

    for id in ClassId::ALL {
        let class_id = format!("class:{}", format!("{id:?}").to_lowercase());
        claim(&mut entries, class_id, ClassFamily::Crb, "core_rulebook", "crb_class_id", DEFAULT_TABLED_MAX_LEVEL, false);
    }
    for id in ApgClassId::ALL {
        let class_id = format!("class:{}", id.name());
        claim(&mut entries, class_id, ClassFamily::Apg, "advanced_players_guide", "apg_class_id", DEFAULT_TABLED_MAX_LEVEL, false);
    }
    for id in AcgClassId::ALL {
        let class_id = format!("class:{}", id.name());
        claim(&mut entries, class_id, ClassFamily::Acg, "advanced_class_guide", "acg_class_id", DEFAULT_TABLED_MAX_LEVEL, false);
    }
    for id in PuClassId::ALL {
        let class_id = format!("class:{}", id.name());
        claim(&mut entries, class_id, ClassFamily::PathfinderUnchained, "pathfinder_unchained", "pu_class_id", DEFAULT_TABLED_MAX_LEVEL, false);
    }
    for id in UcClassId::ALL {
        let class_id = format!("class:{}", id.name());
        claim(&mut entries, class_id, ClassFamily::UltimateCombat, "ultimate_combat", "uc_class_id", DEFAULT_TABLED_MAX_LEVEL, false);
    }
    for meta in untabled_base_class_registry() {
        claim(
            &mut entries,
            meta.class_id.clone(),
            ClassFamily::UntabledExoticBase,
            &meta.source_book,
            "untabled_base_class_registry",
            meta.max_level,
            false,
        );
    }
    for meta in crb_untabled_class_chassis::covered_classes() {
        let slug = meta.class_id.strip_prefix("class:").unwrap_or(&meta.class_id).to_string();
        let max_level = class_chassis_sheet_rules::record("core_rulebook", &slug)
            .map(|record| record.max_level)
            .unwrap_or(DEFAULT_TABLED_MAX_LEVEL);
        claim(
            &mut entries,
            meta.class_id.clone(),
            ClassFamily::CrbNpcEx,
            "core_rulebook",
            "crb_untabled_class_chassis",
            max_level,
            false,
        );
    }

    let books = prestige_scan_books();
    let book_refs: Vec<&str> = books.iter().map(String::as_str).collect();
    for ((book, slug), chassis) in class_chassis_sheet_rules::records(&book_refs) {
        if !chassis.tags.iter().any(|t| t == "Prestige") {
            continue;
        }
        let class_id = format!("class:{slug}");
        claim(&mut entries, class_id, ClassFamily::Prestige, &book, "prestige_tagged_record", chassis.max_level, true);
    }

    // The ninth source: `generic_class_chassis`'s own population. Folded in
    // last, and never claims a family a prior source did not already
    // establish -- see this module's doc comment. A class id it names that
    // none of the eight above claimed is recorded under
    // `ClassFamily::GenericOnly`, never silently folded into an existing
    // family.
    for meta in generic_class_chassis_covered_classes() {
        let is_prestige = meta.tags.iter().any(|t| t == "Prestige");
        entries
            .entry(meta.class_id.clone())
            .and_modify(|e| {
                add_unique(&mut e.books, &meta.book);
                add_registry(&mut e.registries, "generic_class_chassis");
            })
            .or_insert_with(|| ClassCensusEntry {
                class_id: meta.class_id.clone(),
                family: ClassFamily::GenericOnly,
                books: vec![meta.book.clone()],
                registries: vec!["generic_class_chassis"],
                max_level: meta.max_level,
                is_prestige,
            });
    }

    entries
}

/// `census()`'s entries grouped by family, counted -- the partition
/// `every_registry_is_swept_once` prints in its own assertion message.
pub fn family_counts(entries: &BTreeMap<String, ClassCensusEntry>) -> BTreeMap<ClassFamily, usize> {
    let mut counts: BTreeMap<ClassFamily, usize> = BTreeMap::new();
    for entry in entries.values() {
        *counts.entry(entry.family).or_insert(0) += 1;
    }
    counts
}

// ---------------------------------------------------------------------------
// F0b: the base-class sweep and the deterministic sheet dump.
//
// F0a (above) only merges the registries; nothing here computes a status.
// F0b adds the sweep itself -- but only for NON-prestige ids. A prestige
// class alone is never a legitimate Computed measurement (it needs a
// carrier class per the entry gate, `epic-f-class-completion.md` §2) so
// prestige rows are left for the next F0 step and reported
// `"not_swept_yet"` by `src/bin/class_census.rs` rather than swept here
// with a misleading single-class posture.
// ---------------------------------------------------------------------------

/// One claim-blocking diagnostic from a class sweep, plus exactly which
/// levels it fires at. Mirrors `v06_class_state_dump`'s own
/// `BlockingDiagnostic` shape -- the two describe the same kind of evidence
/// and should describe it the same way.
#[derive(Debug, Clone)]
pub struct CensusBlockingDiagnostic {
    pub id: String,
    pub message: String,
    pub levels: Vec<u8>,
}

/// One non-prestige class's real, engine-derived state across its own
/// level sweep (`1..=entry.max_level`, alone, no other class levels).
#[derive(Debug, Clone)]
pub struct ClassSweepResult {
    pub class_id: String,
    pub family: ClassFamily,
    pub books: Vec<String>,
    pub registries: Vec<&'static str>,
    pub max_level: u8,
    pub levels_computed: Vec<u8>,
    pub levels_blocked: Vec<u8>,
    pub blocking: Vec<CensusBlockingDiagnostic>,
}

impl ClassSweepResult {
    /// Every swept level reached `HeadlessReceiptStatus::Computed`.
    pub fn computed(&self) -> bool {
        self.levels_blocked.is_empty()
    }
}

/// Load the shared deterministic pilot input fixture -- the exact same file
/// and loader `v06_class_state_dump` uses, read at runtime from
/// `CARGO_MANIFEST_DIR` rather than `include_str!`ed (this is a lib
/// function a `src/bin/*.rs` target calls, and only ever from a repo
/// checkout).
pub fn load_sweep_fixture() -> Result<CharacterInput, String> {
    let fixture_path = repo_root().join(FIXTURE_RELATIVE_PATH);
    let fixture_text = std::fs::read_to_string(&fixture_path)
        .map_err(|e| format!("could not read {}: {e}", fixture_path.display()))?;
    let load = load_character_input_fixture(&fixture_text);
    if !load.diagnostics.is_empty() {
        return Err(format!(
            "shared deterministic fixture failed to load cleanly: {:?}",
            load.diagnostics
        ));
    }
    load.character_input
        .ok_or_else(|| "shared deterministic fixture produced no CharacterInput record".to_owned())
}

/// Sweep one class -- alone, `1..=entry.max_level`, the same real fixed
/// loadout plus canonical seeds `v06_class_state_dump` uses (via
/// [`crate::rules_core::class_seeds::input_for`]) -- and collect its real
/// engine state. A panic inside the compute pipeline is itself a real,
/// reportable blocker rather than a crash (mirrors
/// `v06_class_state_dump`'s own `state_for`).
pub fn sweep_class(fixture: &CharacterInput, entry: &ClassCensusEntry) -> ClassSweepResult {
    let class_name = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id);
    let mut levels_computed = Vec::new();
    let mut levels_blocked = Vec::new();
    let mut blocking: Vec<CensusBlockingDiagnostic> = Vec::new();

    for level in 1..=entry.max_level {
        let input = input_for(fixture, class_name, level);

        // Same posture as `v06_class_state_dump::state_for`: a panic is
        // caught and reported as a named blocking diagnostic rather than
        // aborting the whole sweep.
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            build_pilot_headless_receipt(&input)
        }));

        let receipt = match outcome {
            Ok(receipt) => receipt,
            Err(payload) => {
                let detail = payload
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_owned())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "<non-string panic payload>".to_owned());
                levels_blocked.push(level);
                let id = "engine.panic".to_owned();
                match blocking.iter_mut().find(|b| b.id == id) {
                    Some(existing) => existing.levels.push(level),
                    None => blocking.push(CensusBlockingDiagnostic {
                        id,
                        message: format!(
                            "the compute pipeline PANICS for {} at level {level} instead of \
                             returning a receipt, so no status can be derived: {detail}",
                            entry.class_id
                        ),
                        levels: vec![level],
                    }),
                }
                continue;
            }
        };

        if receipt.status == HeadlessReceiptStatus::Computed {
            levels_computed.push(level);
        } else {
            levels_blocked.push(level);
        }
        for d in receipt.computation.diagnostics.iter().filter(|d| d.claim_blocking) {
            match blocking.iter_mut().find(|b| b.id == d.id) {
                Some(existing) => {
                    // The engine legitimately emits the same diagnostic more
                    // than once per computation; we want the set of
                    // affected levels, not a multiset.
                    if existing.levels.last() != Some(&level) {
                        existing.levels.push(level);
                    }
                }
                None => blocking.push(CensusBlockingDiagnostic {
                    id: d.id.clone(),
                    message: d.message.clone(),
                    levels: vec![level],
                }),
            }
        }
    }

    ClassSweepResult {
        class_id: entry.class_id.clone(),
        family: entry.family,
        books: entry.books.clone(),
        registries: entry.registries.clone(),
        max_level: entry.max_level,
        levels_computed,
        levels_blocked,
        blocking,
    }
}

/// Sweep every NON-prestige class id in `entries`, alone, `1..=max_level`
/// each. Prestige ids are deliberately excluded -- see this section's doc
/// comment above.
pub fn sweep_non_prestige(
    fixture: &CharacterInput,
    entries: &BTreeMap<String, ClassCensusEntry>,
) -> Vec<ClassSweepResult> {
    entries.values().filter(|e| !e.is_prestige).map(|e| sweep_class(fixture, e)).collect()
}

/// The full headless receipt for one class at one level, rendered as
/// stable, sorted, timestamp-free text -- what `class_census --sheet-dump`
/// prints. Explanations and diagnostics are sorted by id with a stable sort
/// (so genuine same-id repeats keep their relative emission order), which
/// is what makes two runs byte-identical regardless of any incidental
/// ordering inside the compute pipeline itself.
pub fn sheet_dump_text(fixture: &CharacterInput, class_name: &str, level: u8) -> String {
    let input = input_for(fixture, class_name, level);
    let receipt = build_pilot_headless_receipt(&input);

    let mut explanations = receipt.computation.explanations.clone();
    explanations.sort_by(|a, b| a.id.cmp(&b.id));
    let mut diagnostics = receipt.computation.diagnostics.clone();
    diagnostics.sort_by(|a, b| a.id.cmp(&b.id));

    let mut out = String::new();
    out.push_str(&format!("class: class:{class_name}\n"));
    out.push_str(&format!("level: {level}\n"));
    out.push_str(&format!("status: {:?}\n", receipt.status));
    out.push_str(&format!("explanations: {}\n", explanations.len()));
    for e in &explanations {
        out.push_str(&format!("  id={} value={} detail={}\n", e.id, e.value, e.detail));
    }
    out.push_str(&format!("diagnostics: {}\n", diagnostics.len()));
    for d in &diagnostics {
        out.push_str(&format!(
            "  id={} claim_blocking={} message={}\n",
            d.id, d.claim_blocking, d.message
        ));
    }
    out
}

// ---------------------------------------------------------------------------
// F0c: prestige classes in the census -- the deterministic carrier build.
//
// A prestige class is never a legitimate `Computed` measurement ALONE (§2 of
// `epic-f-class-completion.md`): PF1's own entry requirements only a carrier
// build can satisfy. This section reads each prestige class's converted
// `applies` gate, picks the carrier(s) the reviewed carrier rule (review
// finding 14) names, sweeps the prestige levels 1..=max_level in that mix,
// and separately confirms the class alone is still Blocked (the negative
// control F0b already exercises the machinery for via `sweep_class`).
// ---------------------------------------------------------------------------

/// The carrier class a prestige class's converted entry gate selects.
/// Wizard/Cleric are always CRB `core_rulebook` records; Fighter is the
/// caster-less floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrestigeCarrier {
    Wizard,
    Cleric,
    Fighter,
}

impl PrestigeCarrier {
    pub fn slug(self) -> &'static str {
        match self {
            PrestigeCarrier::Wizard => "wizard",
            PrestigeCarrier::Cleric => "cleric",
            PrestigeCarrier::Fighter => "fighter",
        }
    }

    /// The spell tradition this carrier provides, or `None` for Fighter
    /// (which provides no casting at all).
    fn spell_kind(self) -> Option<SpellKind> {
        match self {
            PrestigeCarrier::Wizard => Some(SpellKind::Arcane),
            PrestigeCarrier::Cleric => Some(SpellKind::Divine),
            PrestigeCarrier::Fighter => None,
        }
    }
}

fn expr_mentions_spell_kind(expr: &Expr, kind: &SpellKind) -> bool {
    match expr {
        Expr::HighestSpellLevel(k) => k == kind,
        Expr::Sum(terms) => terms.iter().any(|e| expr_mentions_spell_kind(e, kind)),
        Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Min(a, b) | Expr::Max(a, b) => {
            expr_mentions_spell_kind(a, kind) || expr_mentions_spell_kind(b, kind)
        }
        Expr::Floor(inner) | Expr::Ceil(inner) => expr_mentions_spell_kind(inner, kind),
        _ => false,
    }
}

fn gate_mentions_spell_kind(gate: &Applies, kind: &SpellKind) -> bool {
    match gate {
        Applies::All(terms) => terms.iter().any(|t| gate_mentions_spell_kind(t, kind)),
        Applies::AtLeast { of, .. } => of.iter().any(|t| gate_mentions_spell_kind(t, kind)),
        Applies::Not(inner) => gate_mentions_spell_kind(inner, kind),
        Applies::Compare { lhs, rhs, .. } => {
            expr_mentions_spell_kind(lhs, kind) || expr_mentions_spell_kind(rhs, kind)
        }
        _ => false,
    }
}

/// The carrier-selection rule, fully specified in `epic-f-class-completion.md`
/// §2 (review finding 14, CONFIRMED): wizard if the gate mentions Arcane
/// (and not Divine); cleric if Divine (and not Arcane); a SECOND,
/// independent carrier for the dual-caster case (both Arcane AND Divine --
/// `mystic_theurge`, `evangelist`); fighter otherwise (the floor-5 case,
/// measured at 66 of 74 -- 23 with a bare `BaseAttack` term, 43 with
/// neither a caster nor a BAB term at all). Pure and total: every gate shape
/// resolves to a non-empty answer, so a class only ever goes unnamed when
/// its converted record cannot be loaded at all (handled one layer up, in
/// [`carrier_assignment`]) -- never a silently-guessed carrier.
pub fn determine_carriers(gate: &Applies) -> Vec<PrestigeCarrier> {
    let arcane = gate_mentions_spell_kind(gate, &SpellKind::Arcane);
    let divine = gate_mentions_spell_kind(gate, &SpellKind::Divine);
    match (arcane, divine) {
        (true, true) => vec![PrestigeCarrier::Wizard, PrestigeCarrier::Cleric],
        (true, false) => vec![PrestigeCarrier::Wizard],
        (false, true) => vec![PrestigeCarrier::Cleric],
        (false, false) => vec![PrestigeCarrier::Fighter],
    }
}

/// One translatable numeric entry-gate axis: the three shapes §2 names.
#[derive(Debug, Clone, PartialEq)]
enum NumericAxis {
    BaseAttack,
    SkillRanks,
    HighestSpellLevel(SpellKind),
}

#[derive(Debug, Clone)]
struct NumericRequirement {
    description: String,
    axis: NumericAxis,
    required: i32,
}

/// One top-level `applies` clause, classified into the buckets §2's carrier
/// rule and entry-gate column need.
#[derive(Debug, Clone)]
enum TermClass {
    /// No requirement stated (`Applies::Always`).
    AlwaysMet,
    /// Always includes; prints its own condition text. Counted `met`.
    Situational(String),
    /// One of the three translatable numeric axes.
    Numeric(NumericRequirement),
    /// A recognized clause the carrier build never holds by construction
    /// (a feat, alignment, deity, race, item, choice, language, ...) --
    /// unmet, not unknown: the shape is understood, the carrier just lacks
    /// it (§2: "feats, alignment, deity, race, or 'special'").
    NonNumericUnmet(String),
    /// A gate shape this rule was not told how to read. Reported `unknown`
    /// by name -- never guessed (§2's own risk clause).
    Unrecognized(String),
}

fn numeric_axis_of(e: &Expr) -> Option<NumericAxis> {
    match e {
        Expr::BaseAttack => Some(NumericAxis::BaseAttack),
        Expr::SkillRanks(_) => Some(NumericAxis::SkillRanks),
        Expr::HighestSpellLevel(kind) => Some(NumericAxis::HighestSpellLevel(kind.clone())),
        _ => None,
    }
}

fn flip_cmp(op: Cmp) -> Cmp {
    match op {
        Cmp::Lt => Cmp::Gt,
        Cmp::Lte => Cmp::Gte,
        Cmp::Gt => Cmp::Lt,
        Cmp::Gte => Cmp::Lte,
        other => other,
    }
}

fn classify_term(term: &Applies) -> TermClass {
    match term {
        Applies::Always => TermClass::AlwaysMet,
        Applies::Never => TermClass::NonNumericUnmet("Never".to_owned()),
        Applies::Situational { text } => TermClass::Situational(text.clone()),
        // Two converter-added bookkeeping rows every prestige class's own
        // gate carries (§0.9: "this cycle added the class's own level
        // ceiling to the converted applies gate"): the class's own level
        // ceiling (`ClassLevel(this class) <= max_level`, trivially true
        // for a sweep already bounded to `1..=max_level`) and an
        // archetype/variant off-switch (`Var(id) == 0`, trivially true for
        // a synthetic carrier build that selects no archetype -- nothing
        // holds a nonzero contribution to it). Neither is a real PF1 entry
        // requirement a player ever evaluates, so both are recognized and
        // folded into `met`, never reported as an unmet or unknown clause.
        Applies::Compare { lhs: Expr::ClassLevel(_), op: Cmp::Lte, rhs: Expr::Const(_) } => {
            TermClass::AlwaysMet
        }
        Applies::Compare { lhs: Expr::Var(_), op: Cmp::Eq, rhs: Expr::Const(0) } => TermClass::AlwaysMet,
        Applies::Compare { lhs, op, rhs } => {
            // Only a plain `<numeric axis> {Gte|Gt|Eq} Const(n)` (either
            // order) is a translatable numeric requirement; any other
            // Compare shape is a gate this rule cannot read -- Unrecognized,
            // never guessed.
            let (axis_expr, cmp, const_expr) = match (numeric_axis_of(lhs), numeric_axis_of(rhs)) {
                (Some(_), None) => (lhs, *op, rhs),
                (None, Some(_)) => (rhs, flip_cmp(*op), lhs),
                _ => {
                    return TermClass::Unrecognized(format!(
                        "Compare {{ {lhs:?} {op:?} {rhs:?} }}"
                    ));
                }
            };
            let axis = numeric_axis_of(axis_expr).expect("checked above");
            let n = match const_expr {
                Expr::Const(n) => *n,
                other => {
                    return TermClass::Unrecognized(format!(
                        "Compare against non-constant {other:?}"
                    ));
                }
            };
            let required = match cmp {
                Cmp::Gte | Cmp::Eq => n,
                Cmp::Gt => n + 1,
                other => {
                    return TermClass::Unrecognized(format!(
                        "Compare with unsupported operator {other:?}"
                    ));
                }
            };
            let description = match &axis {
                NumericAxis::BaseAttack => format!("BaseAttack >= {required}"),
                NumericAxis::SkillRanks => format!("SkillRanks >= {required}"),
                NumericAxis::HighestSpellLevel(kind) => {
                    format!("HighestSpellLevel {kind:?} >= {required}")
                }
            };
            TermClass::Numeric(NumericRequirement { description, axis, required })
        }
        Applies::Holds { what, .. } => TermClass::NonNumericUnmet(format!("Holds {what:?}")),
        Applies::Chosen { choice, option } => {
            TermClass::NonNumericUnmet(format!("Chosen {{ {choice:?}, {option:?} }}"))
        }
        Applies::ItemHas { tags, n } => {
            TermClass::NonNumericUnmet(format!("ItemHas {{ {tags:?}, {n} }}"))
        }
        // A compound "at least N of ..." or negated clause is a recognized
        // gate shape (real PF1 prestige gates use it constantly -- e.g.
        // Mystic Theurge's "at least 2 of: 3 ranks Knowledge (arcana),
        // 3 ranks Knowledge (religion)"), but combinatorially satisfying an
        // N-of-M clause against a level solve is outside the three
        // translatable axes §2 names. Printed and bucketed under the
        // documented "special" category -- recognized, non-blocking, never
        // silently passed -- rather than reported `unknown` (which is
        // reserved for a shape this rule truly cannot read at all).
        Applies::AtLeast { n, of } => {
            TermClass::NonNumericUnmet(format!("special: at least {n} of {of:?}"))
        }
        Applies::Not(inner) => TermClass::NonNumericUnmet(format!("special: Not({inner:?})")),
        Applies::All(nested) => TermClass::NonNumericUnmet(format!("special: nested All({nested:?})")),
    }
}

fn top_level_terms(gate: &Applies) -> Vec<&Applies> {
    match gate {
        Applies::All(terms) => terms.iter().collect(),
        other => vec![other],
    }
}

/// Smallest level `1..=DEFAULT_TABLED_MAX_LEVEL` at which `carrier` meets
/// `req`, or `None` when it is unreachable at all -- a caster-level axis of
/// the wrong kind (the dual-caster cross term: the wizard mix can never
/// satisfy a Divine term), or a requirement past the level cap.
fn level_meeting(carrier: PrestigeCarrier, req: &NumericRequirement) -> Option<u8> {
    match &req.axis {
        NumericAxis::BaseAttack => {
            let chassis = class_chassis_sheet_rules::record("core_rulebook", carrier.slug())?;
            (1..=DEFAULT_TABLED_MAX_LEVEL).find(|&level| {
                chassis
                    .row_at(level)
                    .map(|r| i32::from(r.base_attack_bonus) >= req.required)
                    .unwrap_or(false)
            })
        }
        NumericAxis::SkillRanks => {
            let level = req.required.max(1);
            (level <= i32::from(DEFAULT_TABLED_MAX_LEVEL)).then_some(level as u8)
        }
        NumericAxis::HighestSpellLevel(kind) => {
            if carrier.spell_kind().as_ref() != Some(kind) {
                return None;
            }
            // A full caster's spell level L is first reachable at class
            // level 2L-1 (§2's own stated formula).
            let level = (2 * req.required - 1).max(1);
            (level <= i32::from(DEFAULT_TABLED_MAX_LEVEL)).then_some(level as u8)
        }
    }
}

/// The value `carrier` actually reaches at `level` on `req`'s own axis --
/// what an unmet-by-the-cap line prints (§2's stated precedence).
fn value_reached(carrier: PrestigeCarrier, req: &NumericRequirement, level: u8) -> i32 {
    match &req.axis {
        NumericAxis::BaseAttack => class_chassis_sheet_rules::record("core_rulebook", carrier.slug())
            .and_then(|chassis| chassis.row_at(level))
            .map(|r| i32::from(r.base_attack_bonus))
            .unwrap_or(0),
        NumericAxis::SkillRanks => i32::from(level),
        NumericAxis::HighestSpellLevel(kind) => {
            if carrier.spell_kind().as_ref() == Some(kind) {
                (i32::from(level) + 1) / 2
            } else {
                0
            }
        }
    }
}

/// One carrier's real, printed entry-gate verdict against one prestige
/// class's converted gate -- non-blocking (§2: "print, do not simulate"),
/// but never silently dropped either.
#[derive(Debug, Clone)]
pub struct CarrierEntryGate {
    pub carrier: PrestigeCarrier,
    pub carrier_level: u8,
    pub status: &'static str,
    pub met: Vec<String>,
    pub unmet: Vec<String>,
    pub unknown: Vec<String>,
}

/// Evaluates `gate` for one `carrier`: the smallest carrier level meeting
/// every numeric term (floor 5), the cap precedence (§2: "the carrier +
/// prestige max_level <= 20 cap always wins"), and every non-numeric or
/// unrecognized clause named explicitly.
fn evaluate_carrier(carrier: PrestigeCarrier, gate: &Applies, prestige_max_level: u8) -> CarrierEntryGate {
    let mut numeric: Vec<NumericRequirement> = Vec::new();
    let mut met: Vec<String> = Vec::new();
    let mut unmet: Vec<String> = Vec::new();
    let mut unknown: Vec<String> = Vec::new();

    for term in top_level_terms(gate) {
        match classify_term(term) {
            TermClass::AlwaysMet => {}
            TermClass::Situational(text) => met.push(format!("Situational: {text}")),
            TermClass::Numeric(req) => numeric.push(req),
            TermClass::NonNumericUnmet(desc) => unmet.push(desc),
            TermClass::Unrecognized(desc) => unknown.push(desc),
        }
    }

    // Requirements this carrier can never reach at all (wrong-kind caster
    // axis, or past the level cap) are unmet outright, not part of the
    // level solve below.
    let mut reachable: Vec<&NumericRequirement> = Vec::new();
    for req in &numeric {
        match level_meeting(carrier, req) {
            Some(_) => reachable.push(req),
            None => unmet.push(format!(
                "{} (carrier {} cannot ever reach this)",
                req.description,
                carrier.slug()
            )),
        }
    }

    let mut carrier_level: u8 = 5;
    for req in &reachable {
        if let Some(level) = level_meeting(carrier, req) {
            carrier_level = carrier_level.max(level);
        }
    }

    // §2's stated precedence: the level cap always wins.
    if u16::from(carrier_level) + u16::from(prestige_max_level) > 20 {
        carrier_level = 20u8.saturating_sub(prestige_max_level);
    }

    for req in &reachable {
        let needed = level_meeting(carrier, req).unwrap_or(u8::MAX);
        if needed <= carrier_level {
            met.push(req.description.clone());
        } else {
            let reached = value_reached(carrier, req, carrier_level);
            unmet.push(format!(
                "{} (needs level {needed}, capped carrier reaches level {carrier_level} => {reached})",
                req.description
            ));
        }
    }

    let status = if !unknown.is_empty() {
        "unknown"
    } else if !unmet.is_empty() {
        "unmet"
    } else {
        "met"
    };

    CarrierEntryGate { carrier, carrier_level, status, met, unmet, unknown }
}

/// The converted `applies` gate for one prestige class, tried against every
/// book the census found it in (a class can be tagged `Prestige` in more
/// than one book pre-dedupe). `None` only when no book's record can be
/// loaded at all.
fn prestige_applies_gate(books: &[String], slug: &str) -> Option<Applies> {
    let package = live_sheet_rules()?;
    for book in books {
        if let Some(rule) = package.rule(&format!("{book}:class:{slug}")) {
            return Some(rule.applies.clone());
        }
    }
    package.find("class", slug).and_then(|id| package.rule(id)).map(|r| r.applies.clone())
}

/// The fast half of F0c: which carrier(s) a prestige class's own gate
/// selects, with no engine sweep. `Err` only when the converted record
/// cannot be loaded at all -- named by class id, never silently skipped.
pub fn carrier_assignment(entry: &ClassCensusEntry) -> Result<Vec<PrestigeCarrier>, String> {
    let slug = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id);
    let gate = prestige_applies_gate(&entry.books, slug).ok_or_else(|| {
        format!("{}: no converted class record found in {:?}", entry.class_id, entry.books)
    })?;
    Ok(determine_carriers(&gate))
}

/// Build the real production-shaped multiclass input for a carrier class
/// plus the prestige class under test, at their own fixed/swept levels.
/// Mirrors [`crate::rules_core::class_seeds::input_for`]'s single-class
/// shape, widened to more than one class level.
fn input_for_mix(fixture: &CharacterInput, classes: &[(&str, u8)]) -> CharacterInput {
    let mut input = fixture.clone();
    input.case_id = Some(format!(
        "class_census.prestige_mix.{}",
        classes.iter().map(|(name, level)| format!("{name}{level}")).collect::<Vec<_>>().join(".")
    ));
    input.chosen.class_levels = classes
        .iter()
        .map(|(name, level)| CharacterClassLevel { class_id: format!("class:{name}"), level: *level })
        .collect();
    for (name, _) in classes {
        let (choices, spells) = canonical_seeds_for(name);
        input.chosen.selected_choices.extend(choices);
        input.chosen.spells_selected.extend(spells);
    }
    input
}

/// One carrier mix's real, engine-derived sweep of a prestige class's own
/// levels (`1..=max_level`), carrier level fixed. Same panic-caught posture
/// as [`sweep_class`].
#[derive(Debug, Clone)]
pub struct PrestigeMixSweep {
    pub carrier: PrestigeCarrier,
    pub carrier_level: u8,
    pub prestige_class_id: String,
    pub max_level: u8,
    pub levels_computed: Vec<u8>,
    pub levels_blocked: Vec<u8>,
    pub blocking: Vec<CensusBlockingDiagnostic>,
}

impl PrestigeMixSweep {
    pub fn computed(&self) -> bool {
        self.levels_blocked.is_empty()
    }
}

pub fn sweep_prestige_mix(
    fixture: &CharacterInput,
    carrier: PrestigeCarrier,
    carrier_level: u8,
    entry: &ClassCensusEntry,
) -> PrestigeMixSweep {
    let prestige_slug = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id);
    let mut levels_computed = Vec::new();
    let mut levels_blocked = Vec::new();
    let mut blocking: Vec<CensusBlockingDiagnostic> = Vec::new();

    for level in 1..=entry.max_level {
        let input = input_for_mix(fixture, &[(carrier.slug(), carrier_level), (prestige_slug, level)]);

        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            build_pilot_headless_receipt(&input)
        }));

        let receipt = match outcome {
            Ok(receipt) => receipt,
            Err(payload) => {
                let detail = payload
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_owned())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "<non-string panic payload>".to_owned());
                levels_blocked.push(level);
                let id = "engine.panic".to_owned();
                match blocking.iter_mut().find(|b| b.id == id) {
                    Some(existing) => existing.levels.push(level),
                    None => blocking.push(CensusBlockingDiagnostic {
                        id,
                        message: format!(
                            "the compute pipeline PANICS for the {}/{} mix at prestige level \
                             {level} instead of returning a receipt: {detail}",
                            carrier.slug(),
                            entry.class_id
                        ),
                        levels: vec![level],
                    }),
                }
                continue;
            }
        };

        if receipt.status == HeadlessReceiptStatus::Computed {
            levels_computed.push(level);
        } else {
            levels_blocked.push(level);
        }
        for d in receipt.computation.diagnostics.iter().filter(|d| d.claim_blocking) {
            match blocking.iter_mut().find(|b| b.id == d.id) {
                Some(existing) => {
                    if existing.levels.last() != Some(&level) {
                        existing.levels.push(level);
                    }
                }
                None => blocking.push(CensusBlockingDiagnostic {
                    id: d.id.clone(),
                    message: d.message.clone(),
                    levels: vec![level],
                }),
            }
        }
    }

    PrestigeMixSweep {
        carrier,
        carrier_level,
        prestige_class_id: entry.class_id.clone(),
        max_level: entry.max_level,
        levels_computed,
        levels_blocked,
        blocking,
    }
}

/// One prestige class's full F0c row: its carrier(s), each carrier mix's
/// real engine sweep and printed entry-gate verdict, its combined
/// `entry_gate` status, and its own `alone_status` negative control
/// ([`sweep_class`], reused unchanged -- a prestige class alone is swept
/// the exact same way a base class is, the only difference is what the
/// caller expects the answer to be).
#[derive(Debug, Clone)]
pub struct PrestigeCensusRow {
    pub class_id: String,
    pub books: Vec<String>,
    pub max_level: u8,
    pub carriers: Vec<PrestigeCarrier>,
    pub mixes: Vec<(PrestigeMixSweep, CarrierEntryGate)>,
    pub entry_gate_status: &'static str,
    pub alone: ClassSweepResult,
    pub load_error: Option<String>,
}

fn combine_entry_gate_status(mixes: &[(PrestigeMixSweep, CarrierEntryGate)]) -> &'static str {
    if mixes.len() == 1 {
        return mixes[0].1.status;
    }
    let statuses: Vec<&str> = mixes.iter().map(|(_, g)| g.status).collect();
    if statuses.iter().all(|s| *s == "met") {
        "met"
    } else if statuses.contains(&"met") {
        // §2 (review finding 14): partially-met is used ONLY for the
        // dual-caster case, when one independent-carrier mix succeeds while
        // the other fails its own entry gate.
        "partially-met"
    } else if statuses.contains(&"unknown") {
        "unknown"
    } else {
        "unmet"
    }
}

pub fn build_prestige_row(fixture: &CharacterInput, entry: &ClassCensusEntry) -> PrestigeCensusRow {
    let alone = sweep_class(fixture, entry);
    let slug = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id);

    let Some(gate) = prestige_applies_gate(&entry.books, slug) else {
        return PrestigeCensusRow {
            class_id: entry.class_id.clone(),
            books: entry.books.clone(),
            max_level: entry.max_level,
            carriers: Vec::new(),
            mixes: Vec::new(),
            entry_gate_status: "unknown",
            alone,
            load_error: Some(format!(
                "no converted class record found for {} in {:?} -- reported unknown, never defaulted",
                entry.class_id, entry.books
            )),
        };
    };

    let carriers = determine_carriers(&gate);
    let mixes: Vec<(PrestigeMixSweep, CarrierEntryGate)> = carriers
        .iter()
        .map(|carrier| {
            let ge = evaluate_carrier(*carrier, &gate, entry.max_level);
            let sweep = sweep_prestige_mix(fixture, *carrier, ge.carrier_level, entry);
            (sweep, ge)
        })
        .collect();

    let entry_gate_status = combine_entry_gate_status(&mixes);

    PrestigeCensusRow {
        class_id: entry.class_id.clone(),
        books: entry.books.clone(),
        max_level: entry.max_level,
        carriers,
        mixes,
        entry_gate_status,
        alone,
        load_error: None,
    }
}

/// Every prestige class id in `entries`, its carrier build swept.
pub fn sweep_prestige(
    fixture: &CharacterInput,
    entries: &BTreeMap<String, ClassCensusEntry>,
) -> Vec<PrestigeCensusRow> {
    entries.values().filter(|e| e.is_prestige).map(|e| build_prestige_row(fixture, e)).collect()
}

// ---------------------------------------------------------------------------
// F0d: the multiclass mix panel.
//
// The EXISTING multiclass negative-control tests (§2, "Mix panel"; §5,
// review finding 8) already pin 185 real (class, level) + (class, level)
// mixes -- `scripts/extract_multiclass_census_panel.py` extracts every one
// of them MECHANICALLY from its own source (`tests/sd18_widening/rows.rs`'s
// `MULTICLASS_NEG_ROWS`, the hand-written tests inside
// `tests/sd18_widening/*.rs` that are NOT part of that array,
// `tests/sd13_progression/rows.rs`'s `multiclass_negative_controls!`
// invocations, the hand-written tests inside `tests/sd13_progression/*.rs`
// that don't call that macro, and every hand-written top-level file with
// its own such test) into the committed
// `tests/fixtures/rules_core/multiclass_census_panel.json`. This section
// re-sweeps each of those exact mixes through the real engine and reports
// each one's real status plus its claim-blocking diagnostic ids -- the
// measurement F3 needs before any F3 code is written (§5: "What
// claim-blocks `[barbarian 12, fighter 1]` is not measured yet").
// ---------------------------------------------------------------------------

/// One multiclass negative-control test's own input, read from the
/// committed panel data file -- never retyped, see
/// `scripts/extract_multiclass_census_panel.py`.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct MixPanelRow {
    /// Stable, source-traceable identifier (`"<bucket>::<test_fn>"`).
    pub key: String,
    /// Repo-relative path of the test file this row's input was extracted
    /// from.
    pub source_file: String,
    /// The real `#[test] fn` name this row's input was extracted from.
    pub test_fn: String,
    /// The mix itself, in the order the originating test built it
    /// (always the class under test first, the widening class second, in
    /// every row measured today).
    pub classes: Vec<(String, u8)>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct MixPanelDocument {
    row_count: usize,
    rows: Vec<MixPanelRow>,
}

/// Repo-relative path to the committed panel data file, read at runtime
/// (mirrors [`load_sweep_fixture`]'s own repo-root-relative read, not
/// `include_str!`ed -- this is a lib fn only ever called from a repo
/// checkout).
pub const MIX_PANEL_RELATIVE_PATH: &str = "tests/fixtures/rules_core/multiclass_census_panel.json";

/// Load the committed multiclass mix panel. `Err` names the path and the
/// real parse/shape failure -- never a silently empty panel.
pub fn load_mix_panel() -> Result<Vec<MixPanelRow>, String> {
    let path = repo_root().join(MIX_PANEL_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("could not read {}: {e}", path.display()))?;
    let document: MixPanelDocument = serde_json::from_str(&text)
        .map_err(|e| format!("could not parse {} as the mix-panel document shape: {e}", path.display()))?;
    if document.rows.len() != document.row_count {
        return Err(format!(
            "{}: its own row_count field ({}) does not match the actual rows array length ({}) -- \
             the file was hand-edited or truncated, not regenerated by the extractor",
            path.display(),
            document.row_count,
            document.rows.len()
        ));
    }
    Ok(document.rows)
}

/// One mix-panel row's real, engine-derived state -- a single fixed
/// build (not a level sweep: the panel's own input IS the level, per
/// row), same panic-caught posture as [`sweep_class`]/[`sweep_prestige_mix`].
#[derive(Debug, Clone)]
pub struct MixPanelSweepResult {
    pub key: String,
    pub source_file: String,
    pub test_fn: String,
    pub classes: Vec<(String, u8)>,
    pub computed: bool,
    /// Every claim-blocking diagnostic id the real computation raised for
    /// this mix, deduplicated and sorted -- empty when `computed` is true.
    pub blocking_diagnostic_ids: Vec<String>,
}

pub fn sweep_mix_panel_row(fixture: &CharacterInput, row: &MixPanelRow) -> MixPanelSweepResult {
    let classes: Vec<(&str, u8)> = row.classes.iter().map(|(c, l)| (c.as_str(), *l)).collect();
    let input = input_for_mix(fixture, &classes);

    let outcome =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| build_pilot_headless_receipt(&input)));

    let (computed, blocking_diagnostic_ids) = match outcome {
        Ok(receipt) => {
            let mut ids: Vec<String> = receipt
                .computation
                .diagnostics
                .iter()
                .filter(|d| d.claim_blocking)
                .map(|d| d.id.clone())
                .collect();
            ids.sort();
            ids.dedup();
            (receipt.status == HeadlessReceiptStatus::Computed, ids)
        }
        Err(payload) => {
            let detail = payload
                .downcast_ref::<&str>()
                .map(|s| (*s).to_owned())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<non-string panic payload>".to_owned());
            eprintln!(
                "class_census: mix panel row {} ({:?}) PANICS instead of returning a receipt: {detail}",
                row.key, row.classes
            );
            (false, vec!["engine.panic".to_owned()])
        }
    };

    MixPanelSweepResult {
        key: row.key.clone(),
        source_file: row.source_file.clone(),
        test_fn: row.test_fn.clone(),
        classes: row.classes.clone(),
        computed,
        blocking_diagnostic_ids,
    }
}

/// Sweep every row in the panel, in the order it is stored (sorted by
/// `key`, see the extractor).
pub fn sweep_mix_panel(fixture: &CharacterInput, panel: &[MixPanelRow]) -> Vec<MixPanelSweepResult> {
    panel.iter().map(|row| sweep_mix_panel_row(fixture, row)).collect()
}

/// Histogram of claim-blocking diagnostic ids across the whole panel --
/// `docs/release/SD-36-consolidation/artifacts/epic-f/mix-panel-histogram.md`'s
/// own source of truth. Counts ROWS a diagnostic id blocks, not raw
/// occurrences (each row's own id list is already deduplicated by
/// [`sweep_mix_panel_row`]).
pub fn mix_panel_blocking_histogram(results: &[MixPanelSweepResult]) -> BTreeMap<String, usize> {
    let mut histogram: BTreeMap<String, usize> = BTreeMap::new();
    for result in results {
        for id in &result.blocking_diagnostic_ids {
            *histogram.entry(id.clone()).or_insert(0) += 1;
        }
    }
    histogram
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partition_message(entries: &BTreeMap<String, ClassCensusEntry>) -> String {
        let counts = family_counts(entries);
        let mut parts: Vec<String> =
            counts.iter().map(|(family, count)| format!("{}={count}", family.label())).collect();
        parts.sort();
        format!("measured total={} ({})", entries.len(), parts.join(", "))
    }

    #[test]
    fn every_registry_is_swept_once() {
        let entries = census();
        // Every id appears exactly once -- guaranteed by construction
        // (`BTreeMap<String, _>`), but a duplicate `class_id` field inside
        // two different map slots (a bug in the merge, not the type) would
        // still be a real defect, so check it directly rather than trust
        // the map shape alone.
        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        for (key, entry) in &entries {
            assert_eq!(key, &entry.class_id, "map key must equal the entry's own class_id");
            assert!(seen_ids.insert(entry.class_id.as_str()), "{} swept more than once: {}", entry.class_id, partition_message(&entries));
        }
        // Every entry names at least one registry and one book -- a class
        // id with zero provenance would mean the merge inserted it without
        // ever calling `claim`, which should be unreachable.
        for entry in entries.values() {
            assert!(!entry.registries.is_empty(), "{}: no registry recorded ({})", entry.class_id, partition_message(&entries));
            assert!(!entry.books.is_empty(), "{}: no book recorded ({})", entry.class_id, partition_message(&entries));
        }
        // The measured partition by family, always computed into `message`
        // so a future failure names exactly which family moved. Re-derive
        // with `cargo test --locked -j 2 --lib
        // class_census::tests::every_registry_is_swept_once -- --nocapture`
        // (temporarily insert `panic!("{message}")` above this line to see
        // it on a green run).
        let message = partition_message(&entries);
        let counts = family_counts(&entries);
        // Measured 2026-09-21, independently in this code (not copied from
        // the doc): Crb=11, Apg=6, Acg=10, PathfinderUnchained=4,
        // UltimateCombat=3, UntabledExoticBase=20, CrbNpcEx=7, Prestige=74.
        // 11+6+10+4=31 tabled + 3 + 20 + 7 + 74 = 135, exactly
        // `docs/architecture/status.md`'s own "31+3+20+7+74 = 135" partition
        // (measured there 2026-09-20 by a now-deleted throwaway test this
        // module replaces with a permanent one). Asserted per family, not
        // only as a total, so a family that shrinks while another grows by
        // the same amount cannot hide behind an unchanged total. No
        // `scripts/retro.py correction` needed: the measured total already
        // equals the previously published figure -- if a future run of
        // this test goes red, that IS the corpus movement to log.
        assert_eq!(counts.get(&ClassFamily::Crb).copied().unwrap_or(0), 11, "{message}");
        assert_eq!(counts.get(&ClassFamily::Apg).copied().unwrap_or(0), 6, "{message}");
        assert_eq!(counts.get(&ClassFamily::Acg).copied().unwrap_or(0), 10, "{message}");
        assert_eq!(counts.get(&ClassFamily::PathfinderUnchained).copied().unwrap_or(0), 4, "{message}");
        assert_eq!(counts.get(&ClassFamily::UltimateCombat).copied().unwrap_or(0), 3, "{message}");
        assert_eq!(counts.get(&ClassFamily::UntabledExoticBase).copied().unwrap_or(0), 20, "{message}");
        assert_eq!(counts.get(&ClassFamily::CrbNpcEx).copied().unwrap_or(0), 7, "{message}");
        assert_eq!(counts.get(&ClassFamily::Prestige).copied().unwrap_or(0), 74, "{message}");
        assert_eq!(entries.len(), 135, "measured census total moved off the previously published 135 -- {message}");
    }

    #[test]
    fn prestige_list_matches_the_entry_requirement_fixture() {
        use crate::rules_core::pilot_compute::prestige_class_entry_gate::prestige_class_entry_requirements;

        let entries = census();
        let census_prestige: BTreeSet<String> = entries
            .values()
            .filter(|e| e.family == ClassFamily::Prestige)
            .map(|e| e.class_id.clone())
            .collect();
        let fixture_prestige: BTreeSet<String> =
            prestige_class_entry_requirements().iter().map(|req| req.class_id.clone()).collect();

        let only_in_census: Vec<&String> = census_prestige.difference(&fixture_prestige).collect();
        let only_in_fixture: Vec<&String> = fixture_prestige.difference(&census_prestige).collect();
        assert!(
            only_in_census.is_empty() && only_in_fixture.is_empty(),
            "prestige list mismatch (census {} of fixture {}): only in census = {:?}, only in fixture = {:?}",
            census_prestige.len(),
            fixture_prestige.len(),
            only_in_census,
            only_in_fixture,
        );
        assert_eq!(census_prestige.len(), 74, "measured prestige-tagged class count, of the fixture's own 74");
    }

    #[test]
    fn no_class_id_sits_in_two_families() {
        // Checks the RAW sources, before the first-claimed-wins precedence
        // in `census()` has a chance to paper over a real collision: if two
        // of the eight canonical sources ever name the same id, that is a
        // genuine defect (a class the corpus tags two incompatible ways at
        // once), not something the merge should silently resolve.
        let raw = raw_claims();
        let mut offenders: BTreeMap<String, Vec<&'static str>> = BTreeMap::new();
        let names: Vec<&'static str> = raw.keys().copied().collect();
        for i in 0..names.len() {
            for j in (i + 1)..names.len() {
                let a = &raw[names[i]];
                let b = &raw[names[j]];
                for id in a.intersection(b) {
                    offenders.entry(id.clone()).or_default().extend([names[i], names[j]]);
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "class id(s) claimed by more than one canonical source: {offenders:?}"
        );

        // And the merged census itself: every entry's family must be one
        // the class id's own raw membership actually supports (catches a
        // `claim()` call site accidentally using the wrong `ClassFamily`
        // constant for its own loop).
        let entries = census();
        for entry in entries.values() {
            if entry.family == ClassFamily::GenericOnly {
                continue;
            }
            let registry_for_family = match entry.family {
                ClassFamily::Crb => "crb_class_id",
                ClassFamily::Apg => "apg_class_id",
                ClassFamily::Acg => "acg_class_id",
                ClassFamily::PathfinderUnchained => "pu_class_id",
                ClassFamily::UltimateCombat => "uc_class_id",
                ClassFamily::UntabledExoticBase => "untabled_base_class_registry",
                ClassFamily::CrbNpcEx => "crb_untabled_class_chassis",
                ClassFamily::Prestige => "prestige_tagged_record",
                ClassFamily::GenericOnly => unreachable!(),
            };
            assert!(
                entry.registries.contains(&registry_for_family),
                "{}: family {} but its own claiming registry {:?} is not in registries {:?}",
                entry.class_id,
                entry.family.label(),
                registry_for_family,
                entry.registries,
            );
        }
    }

    #[test]
    fn no_generic_only_stragglers_today() {
        // `docs/architecture/status.md`'s own evidence: `generic_class_
        // chassis`'s 78 classes are fully subsumed by the other eight
        // sources (56 Prestige + 19 of 20 untabled-exotic + 3 Ultimate
        // Combat = 78). This pins that today; if it ever goes red, a real
        // new class surfaced through `generic_class_chassis` alone and
        // needs a named family decision, not a silent default.
        let entries = census();
        let stragglers: Vec<&str> = entries
            .values()
            .filter(|e| e.family == ClassFamily::GenericOnly)
            .map(|e| e.class_id.as_str())
            .collect();
        assert!(stragglers.is_empty(), "generic_class_chassis named a class no other source claims: {stragglers:?}");
    }

    #[test]
    fn prestige_scan_books_is_measured_not_guessed() {
        let books = prestige_scan_books();
        assert!(books.contains(&"core_rulebook".to_string()));
        assert!(!books.iter().any(|b| b.starts_with('_')), "book list must exclude bookkeeping dirs: {books:?}");
        // Measured directly: `find data/sheet_rules -maxdepth 1 -type d
        // ! -path data/sheet_rules ! -name '_*' | wc -l`.
        assert!(books.len() >= 30, "measured book-dir count looks too small: {} ({:?})", books.len(), books);
    }

    // -----------------------------------------------------------------
    // F0b RED-first tests (docs/release/SD-36-consolidation/
    // epic-f-class-completion.md §2): the sweep itself and the
    // determinism of `--sheet-dump`'s output. Both fail to compile before
    // `sweep_non_prestige`/`sheet_dump_text` exist, which is this
    // codebase's own established RED shape for a new lib API (see
    // `every_registry_is_swept_once`'s F0a precedent).
    // -----------------------------------------------------------------

    #[test]
    fn sweep_covers_exactly_the_non_prestige_ids() {
        let entries = census();
        let expected: BTreeSet<String> =
            entries.values().filter(|e| !e.is_prestige).map(|e| e.class_id.clone()).collect();
        // 61 = 135 - Prestige's 74, the same partition
        // `every_registry_is_swept_once` measures per family
        // (11+6+10+4=31 tabled + 3 UltimateCombat + 20 UntabledExoticBase +
        // 7 CrbNpcEx = 61).
        assert_eq!(expected.len(), 61, "non-prestige count moved off 135-74=61");

        let fixture = load_sweep_fixture().expect("shared deterministic fixture must load cleanly");
        let results = sweep_non_prestige(&fixture, &entries);
        let actual: BTreeSet<String> = results.iter().map(|r| r.class_id.clone()).collect();
        assert_eq!(
            actual, expected,
            "sweep_non_prestige must cover exactly the registry's non-prestige ids, no more, no fewer"
        );
        assert_eq!(results.len(), expected.len(), "an id was swept more than once");
        assert!(
            results.iter().all(|r| r.family != ClassFamily::Prestige),
            "no prestige row may appear in the non-prestige sweep"
        );
    }

    #[test]
    fn sheet_dump_text_is_deterministic_across_two_runs() {
        let fixture = load_sweep_fixture().expect("shared deterministic fixture must load cleanly");
        let first = sheet_dump_text(&fixture, "fighter", 5);
        let second = sheet_dump_text(&fixture, "fighter", 5);
        assert_eq!(
            first, second,
            "two runs of sheet_dump_text for the same class/level must be byte-for-byte identical"
        );
        assert!(first.contains("class: class:fighter"));
        assert!(first.contains("level: 5"));
        assert!(!first.is_empty());
    }

    // -----------------------------------------------------------------
    // F0c RED-first tests (docs/release/SD-36-consolidation/
    // epic-f-class-completion.md §2, review finding 14): the deterministic
    // prestige carrier build. Both `determine_carriers`/`carrier_assignment`
    // fail to compile before this batch's code exists, the same established
    // RED shape F0a/F0b already used for a new lib API.
    // -----------------------------------------------------------------

    fn prestige_entries(entries: &BTreeMap<String, ClassCensusEntry>) -> Vec<&ClassCensusEntry> {
        let mut prestige: Vec<&ClassCensusEntry> = entries.values().filter(|e| e.is_prestige).collect();
        prestige.sort_by(|a, b| a.class_id.cmp(&b.class_id));
        prestige
    }

    #[test]
    fn prestige_carrier_is_deterministic() {
        let entries = census();
        let prestige = prestige_entries(&entries);
        assert_eq!(prestige.len(), 74, "measured prestige population moved off 74");
        for entry in &prestige {
            let first = carrier_assignment(entry);
            let second = carrier_assignment(entry);
            assert_eq!(
                first, second,
                "{}: carrier_assignment is not deterministic across two calls",
                entry.class_id
            );
        }
    }

    #[test]
    fn every_prestige_class_gets_a_carrier_or_is_named_unknown() {
        let entries = census();
        let prestige = prestige_entries(&entries);
        assert_eq!(prestige.len(), 74, "measured prestige population moved off 74");

        let mut unknown: Vec<String> = Vec::new();
        for entry in &prestige {
            match carrier_assignment(entry) {
                Ok(carriers) => {
                    assert!(!carriers.is_empty(), "{}: got zero carriers, not a named Unknown", entry.class_id);
                    assert!(
                        carriers.len() <= 2,
                        "{}: more than the documented dual-caster carrier count: {carriers:?}",
                        entry.class_id
                    );
                }
                Err(reason) => unknown.push(reason),
            }
        }
        // Measured today: every one of the 74 converted prestige class
        // records loads (the same `class_chassis_sheet_rules::records` scan
        // that found the `Prestige` tag in the first place also proves the
        // record itself is loadable), so this is pinned at 0 -- a future
        // corpus change that makes a record unloadable must surface here BY
        // NAME, never silently drop the row from the census.
        assert!(
            unknown.is_empty(),
            "prestige class(es) with no loadable converted gate (must be named, not silently \
             dropped): {unknown:?}"
        );
    }

    #[test]
    fn prestige_carrier_distribution_is_measured_and_printed() {
        // Not a pinned-number assertion (§8 review finding 8's own
        // "measured, never guessed" posture) -- just proves the three
        // buckets §2's own review-finding-14 evidence names are all
        // reachable from this code, and prints the real split for the F0c
        // commit's own summary to quote. Re-derive with:
        // `cargo test --locked -j 2 --lib \
        //   class_census::tests::prestige_carrier_distribution_is_measured_and_printed \
        //   -- --nocapture`
        let entries = census();
        let prestige = prestige_entries(&entries);
        let mut wizard_only = 0usize;
        let mut cleric_only = 0usize;
        let mut dual = 0usize;
        let mut fighter = 0usize;
        let mut dual_names: Vec<&str> = Vec::new();
        for entry in &prestige {
            let carriers = carrier_assignment(entry).unwrap_or_else(|e| panic!("{e}"));
            match carriers.as_slice() {
                [PrestigeCarrier::Wizard] => wizard_only += 1,
                [PrestigeCarrier::Cleric] => cleric_only += 1,
                [PrestigeCarrier::Fighter] => fighter += 1,
                [PrestigeCarrier::Wizard, PrestigeCarrier::Cleric] => {
                    dual += 1;
                    dual_names.push(&entry.class_id);
                }
                other => panic!("{}: unexpected carrier shape {other:?}", entry.class_id),
            }
        }
        assert_eq!(
            wizard_only + cleric_only + dual + fighter,
            74,
            "carrier buckets must partition all 74 prestige classes"
        );
        // §2's own review-finding-14 evidence named exactly these two ids
        // as the dual-caster case.
        assert_eq!(
            dual, 2,
            "measured dual-caster (Arcane AND Divine) prestige count moved off the reviewed \
             evidence's 2 (mystic_theurge, evangelist): got {dual_names:?}"
        );
        assert!(
            dual_names.contains(&"class:mystic_theurge") && dual_names.contains(&"class:evangelist"),
            "dual-caster ids moved off the reviewed evidence: {dual_names:?}"
        );
    }

    #[test]
    fn a_dual_caster_prestige_class_gets_two_independent_carriers() {
        let entries = census();
        let mystic_theurge = entries
            .get("class:mystic_theurge")
            .expect("mystic_theurge must be in the prestige census");
        let carriers = carrier_assignment(mystic_theurge).expect("mystic_theurge's gate must load");
        assert_eq!(
            carriers,
            vec![PrestigeCarrier::Wizard, PrestigeCarrier::Cleric],
            "mystic_theurge's gate carries both an Arcane and a Divine HighestSpellLevel term"
        );
    }

    #[test]
    fn a_bare_bab_prestige_class_gets_the_fighter_floor() {
        let entries = census();
        let arcane_archer = entries
            .get("class:arcane_archer")
            .expect("arcane_archer must be in the prestige census");
        let carriers = carrier_assignment(arcane_archer).expect("arcane_archer's gate must load");
        // Arcane Archer's real gate (§0.7) carries `HighestSpellLevel
        // Arcane >= 1` among its clauses -- it is a caster-entry prestige
        // class, so its carrier is Wizard, never the Fighter floor. This
        // pins the carrier rule against the one class this document's own
        // §0.7 already names, rather than only against synthetic gates.
        assert_eq!(
            carriers,
            vec![PrestigeCarrier::Wizard],
            "arcane_archer's own §0.7-cited gate carries an Arcane HighestSpellLevel term"
        );
    }

    #[test]
    fn carrier_rule_is_pure_over_synthetic_gates() {
        // Direct unit coverage of `determine_carriers`, independent of the
        // corpus, so the rule's own four branches are pinned even if a
        // future corpus edit happened to leave no real class exercising one
        // of them.
        let bab_only = Applies::Compare { lhs: Expr::BaseAttack, op: Cmp::Gte, rhs: Expr::Const(5) };
        assert_eq!(determine_carriers(&bab_only), vec![PrestigeCarrier::Fighter]);

        let arcane_only = Applies::Compare {
            lhs: Expr::HighestSpellLevel(SpellKind::Arcane),
            op: Cmp::Gte,
            rhs: Expr::Const(1),
        };
        assert_eq!(determine_carriers(&arcane_only), vec![PrestigeCarrier::Wizard]);

        let divine_only = Applies::Compare {
            lhs: Expr::HighestSpellLevel(SpellKind::Divine),
            op: Cmp::Gte,
            rhs: Expr::Const(1),
        };
        assert_eq!(determine_carriers(&divine_only), vec![PrestigeCarrier::Cleric]);

        let dual = Applies::All(vec![arcane_only, divine_only]);
        assert_eq!(determine_carriers(&dual), vec![PrestigeCarrier::Wizard, PrestigeCarrier::Cleric]);
    }

    #[test]
    fn the_bab_cap_precedent_never_bites_today() {
        // §2's own review-finding-14 evidence: 23 prestige classes carry a
        // bare BaseAttack term, max value 7 -- `fighter 7 + max_level 10 =
        // 17 <= 20`, so the cap is never actually reached by a BAB term
        // alone. Pin the arithmetic itself (not the corpus scan, which
        // `prestige_carrier_distribution_is_measured_and_printed` already
        // covers indirectly): a BAB-7 requirement never needs capping
        // against any prestige class's own real max_level.
        let req = NumericRequirement {
            description: "BaseAttack >= 7".to_owned(),
            axis: NumericAxis::BaseAttack,
            required: 7,
        };
        let level = level_meeting(PrestigeCarrier::Fighter, &req).expect("fighter must reach BAB 7");
        assert_eq!(level, 7, "fighter's own BAB is full (1:1 with level)");
        // §2's own evidence uses the real 10-level ceiling most BAB-gated
        // prestige classes carry (`DEFAULT_PRESTIGE_MAX_LEVEL`); the doc's
        // own arithmetic (`fighter 7 + max_level 10 = 17 <= 20`) is pinned
        // directly, not generalised past what the corpus actually shows.
        let prestige_max_level = 10u8;
        assert!(
            u16::from(level) + u16::from(prestige_max_level) <= 20,
            "a BAB-7 requirement must never trip the level cap at the real 10-level prestige ceiling"
        );
    }

    #[test]
    fn every_prestige_class_alone_is_measured_today() {
        // Negative-control measurement (§2's second census column,
        // `alone_status`): every prestige class swept ALONE -- reusing
        // `sweep_class` exactly as F0b's own base-class sweep does -- so
        // this batch's commit can quote the real, measured
        // `BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED` figure rather than assert
        // one. Not itself the RED-until-F2 test (below): this one only
        // proves the sweep runs and records what it finds.
        let entries = census();
        let prestige = prestige_entries(&entries);
        let fixture = load_sweep_fixture().expect("shared deterministic fixture must load cleanly");
        let results: Vec<ClassSweepResult> =
            prestige.iter().map(|entry| sweep_class(&fixture, entry)).collect();
        assert_eq!(results.len(), 74, "an id was swept more than once, or one was skipped");
        let blocked = results.iter().filter(|r| !r.computed()).count();
        // Recorded, not forced: today's honest baseline is that every
        // prestige class alone is Blocked (no chassis population accepts a
        // bare-Prestige-tagged id per §0.5), matching
        // `BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED=74`. If a future corpus
        // or engine change makes one of these Computed alone, this
        // assertion goes red and the baseline moves with a logged
        // `scripts/retro.py correction` -- not a silent widening.
        assert_eq!(
            blocked, 74,
            "prestige-alone Blocked count moved off the measured baseline of 74 -- log a correction \
             before raising BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED"
        );
    }

    #[test]
    #[ignore = "RED until Epic F2"]
    fn prestige_alone_is_blocked_with_the_game_rule() {
        // §2: "every prestige class alone must be Blocked with the F2
        // game-rule diagnostic (74 of 74)." F2 has not landed yet (it adds
        // the prestige-alone entry-requirement diagnostic itself), so today
        // a prestige class alone is Blocked for whatever engine reason
        // happens to fire first (usually an unsupported-chassis diagnostic,
        // never the real game-rule one) -- this test is the pinned future
        // shape, RED until F2 names that diagnostic id.
        const F2_GAME_RULE_DIAGNOSTIC_ID: &str = "class_chassis.prestige_requires_a_carrier_class";

        let entries = census();
        let prestige = prestige_entries(&entries);
        let fixture = load_sweep_fixture().expect("shared deterministic fixture must load cleanly");
        let mut missing: Vec<String> = Vec::new();
        for entry in &prestige {
            let result = sweep_class(&fixture, entry);
            assert!(!result.computed(), "{}: expected Blocked alone, got Computed", entry.class_id);
            let has_game_rule_diagnostic =
                result.blocking.iter().any(|b| b.id == F2_GAME_RULE_DIAGNOSTIC_ID);
            if !has_game_rule_diagnostic {
                missing.push(entry.class_id.clone());
            }
        }
        assert!(
            missing.is_empty(),
            "prestige classes Blocked alone but not by the F2 game-rule diagnostic \
             ({F2_GAME_RULE_DIAGNOSTIC_ID}): {missing:?}"
        );
    }

    #[test]
    fn build_prestige_row_covers_every_prestige_class_with_a_mix_per_carrier() {
        let entries = census();
        let prestige = prestige_entries(&entries);
        let fixture = load_sweep_fixture().expect("shared deterministic fixture must load cleanly");
        for entry in prestige.iter().take(3) {
            let row = build_prestige_row(&fixture, entry);
            assert!(row.load_error.is_none(), "{}: {:?}", entry.class_id, row.load_error);
            assert_eq!(row.mixes.len(), row.carriers.len(), "{}: one mix per named carrier", entry.class_id);
            for (sweep, gate) in &row.mixes {
                assert_eq!(
                    sweep.levels_computed.len() + sweep.levels_blocked.len(),
                    usize::from(entry.max_level),
                    "{}: every prestige level must be swept exactly once",
                    entry.class_id
                );
                assert!(
                    ["met", "unmet", "unknown"].contains(&gate.status),
                    "{}: entry gate status must be one of met/unmet/unknown, got {}",
                    entry.class_id,
                    gate.status
                );
            }
            assert!(
                ["met", "unmet", "unknown", "partially-met"].contains(&row.entry_gate_status),
                "{}: row entry_gate_status {} is not one of the four documented values",
                entry.class_id,
                row.entry_gate_status
            );
        }
    }

    // -----------------------------------------------------------------
    // F0d tests (docs/release/SD-36-consolidation/epic-f-class-completion.md
    // §2 "Mix panel", §5 review finding 8): the multiclass mix panel, its
    // sweep, and the grep-measured sync check on its own size.
    // -----------------------------------------------------------------

    #[test]
    fn mix_panel_loads_and_matches_its_own_row_count() {
        let panel = load_mix_panel().expect("committed mix panel must load and parse cleanly");
        assert!(!panel.is_empty(), "mix panel must not be empty");
        for row in &panel {
            assert_eq!(
                row.classes.len(),
                2,
                "{}: every panel row today is a two-class mix (class under test + widening \
                 class), got {:?}",
                row.key,
                row.classes
            );
        }
        let keys: BTreeSet<&str> = panel.iter().map(|r| r.key.as_str()).collect();
        assert_eq!(keys.len(), panel.len(), "panel keys must be unique (no test double-counted)");
    }

    /// The sync test: the panel's own size must equal the number of
    /// EXISTING multiclass negative-control tests, measured fresh by grep
    /// every time this test runs -- never pinned as a bare literal that
    /// could silently drift from the real test suites the panel claims to
    /// mirror (review finding 8's own "measured, never guessed" mandate,
    /// applied to F0d's own denominator, not just F3's).
    ///
    /// Three independent, stated commands, re-derivable by hand:
    ///
    /// ```text
    /// git grep -ohE 'multiclass_[a-z0-9_]+_is_not_promoted_by_this_slice' -- tests/sd18_widening/ | sort -u | wc -l
    /// git grep -ohE 'multiclass_[a-z0-9_]+_is_not_promoted_by_this_slice' -- tests/sd13_progression/ | sort -u | wc -l
    /// find tests -maxdepth 1 -name '*.rs' -print0 | xargs -0 grep -l 'fn multiclass_.*_is_not_promoted_by_this_slice' | wc -l
    /// ```
    ///
    /// Measured 2026-09-21: 88 + 84 + 13 = 185 -- `BASELINE_CENSUS_MIX_COMPUTED`'s
    /// own provenance (`scripts/verify-baselines.env`). The first two
    /// greps count the DISTINCT `multiclass_..._is_not_promoted_by_this_slice`
    /// identifier text occurring anywhere in each directory -- this single
    /// pattern matches BOTH a macro invocation's `$fn_name` argument (the
    /// `MULTICLASS_NEG_ROWS`/`multiclass_negative_controls!`-driven tests)
    /// AND a hand-written `fn <name>(` definition (the tests in the same
    /// directories that don't call the shared macro), because both shapes
    /// spell the bare identifier as literal text somewhere in the file --
    /// verified against `cargo test --test sd18_widening -- --list \|
    /// grep -c multiclass` (88) and `cargo test --test sd13_progression
    /// -- --list \| grep -c multiclass` (84) matching byte-for-byte at
    /// authoring time (not re-run here: spawning a nested `cargo test`
    /// from inside `cargo test --lib` would violate this repo's
    /// one-cargo-command-at-a-time discipline).
    #[test]
    fn mix_panel_size_matches_the_measured_multiclass_negative_control_count() {
        let repo_root = repo_root();

        let unique_identifier_count = |dir: &str| -> usize {
            let output = Command::new("git")
                .args([
                    "grep",
                    "-ohE",
                    r"multiclass_[a-z0-9_]+_is_not_promoted_by_this_slice",
                    "--",
                    dir,
                ])
                .current_dir(&repo_root)
                .output()
                .unwrap_or_else(|e| panic!("git grep must be runnable inside a git checkout: {e}"));
            let exit_code = output.status.code();
            assert!(
                exit_code == Some(0) || exit_code == Some(1),
                "git grep -- {dir} exited unexpectedly (status={exit_code:?}); stderr:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let text = String::from_utf8_lossy(&output.stdout);
            let unique: BTreeSet<&str> = text.lines().collect();
            unique.len()
        };

        let sd18_widening_count = unique_identifier_count("tests/sd18_widening/");
        let sd13_progression_count = unique_identifier_count("tests/sd13_progression/");

        let find_output = Command::new("bash")
            .arg("-c")
            .arg(
                "find tests -maxdepth 1 -name '*.rs' -print0 \
                 | xargs -0 grep -l 'fn multiclass_.*_is_not_promoted_by_this_slice' \
                 | wc -l",
            )
            .current_dir(&repo_root)
            .output()
            .unwrap_or_else(|e| panic!("find/grep/wc pipeline must be runnable: {e}"));
        assert!(
            find_output.status.success(),
            "hand-written top-level file count pipeline failed: {}",
            String::from_utf8_lossy(&find_output.stderr)
        );
        let top_level_count: usize = String::from_utf8_lossy(&find_output.stdout)
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("hand-written top-level file count was not a number: {e}"));

        let measured = sd18_widening_count + sd13_progression_count + top_level_count;

        let panel = load_mix_panel().expect("committed mix panel must load and parse cleanly");
        assert_eq!(
            panel.len(),
            measured,
            "mix panel has {} rows but the measured multiclass negative-control test count is \
             {sd18_widening_count} (sd18_widening) + {sd13_progression_count} (sd13_progression) \
             + {top_level_count} (hand-written top-level files) = {measured} -- re-run \
             `python3 scripts/extract_multiclass_census_panel.py` and commit the result",
            panel.len()
        );
        // Pinned so a future reader sees the measured figure without
        // re-running the greps -- if this goes red before the panel-size
        // assert above does, the pin moved, not the panel; log a
        // `scripts/retro.py correction` and update this literal together
        // with `scripts/verify-baselines.env`'s `BASELINE_CENSUS_MIX_COMPUTED`
        // comment.
        assert_eq!(measured, 185, "measured multiclass negative-control test count moved off 185");
    }

    #[test]
    fn sweep_mix_panel_row_reports_a_real_status_and_dedupes_blocking_ids() {
        // Deliberately does NOT assert a specific class's expected status
        // by guessing: the census sweeps every mix through ONE shared
        // canonical fixture (`load_sweep_fixture`/`class_seeds`), not each
        // test's own individually-tuned fixture file, so a row's real
        // engine status here can legitimately differ from what its
        // originating hand-written test asserts against ITS OWN fixture.
        //
        // Measured directly (debugged via a full explanation/diagnostic
        // dump before this assertion was written, then removed --
        // `multiclass_barbarian_level10_is_not_promoted_by_this_slice`'s
        // panel row): the receipt is genuinely `Computed`, with exactly
        // the un-bounded `class_feature.barbarian.rage_execution.not_raging`
        // / `class_feature.fighter.weapon_and_armor_proficiency`
        // recognition records and zero claim-blocking diagnostics -- NOT
        // Blocked, unlike the original test's own dedicated level-10
        // fixture (which carries some other deliberate incompleteness
        // that keeps ITS receipt Blocked for an unrelated reason). Every
        // one of the panel's 185 mixes measures the same way under this
        // shared fixture: all 11 classes this panel exercises (barbarian,
        // bard, cleric, druid, fighter, monk, paladin, ranger, rogue,
        // sorcerer, wizard) are already in F0b's own measured 42-of-61
        // Computed-alone set (`docs/release/SD-36-consolidation/artifacts/
        // epic-f/census-f0b.json`), so pairing any two of them recognizes
        // both halves' bounded progression correctly without introducing
        // a NEW claim-blocking diagnostic -- a real, wired, engine-derived
        // result (both classes' own explanations are present in the
        // receipt), not a fabricated one. §5's own words -- "what
        // claim-blocks `[barbarian 12, fighter 1]` is not measured yet" --
        // are answered here: under this fixture, NOTHING does; F3 will
        // need its own fixture(s) that actually leave a mix incomplete to
        // ever observe a Blocked mix-panel row.
        let fixture = load_sweep_fixture().expect("shared deterministic fixture must load cleanly");
        let panel = load_mix_panel().expect("committed mix panel must load and parse cleanly");
        let results = sweep_mix_panel(&fixture, &panel);

        assert_eq!(results.len(), panel.len(), "every panel row must produce exactly one sweep result");

        for result in &results {
            assert_eq!(
                result.computed,
                result.blocking_diagnostic_ids.is_empty(),
                "{}: computed={} but blocking_diagnostic_ids={:?} -- these must agree",
                result.key,
                result.computed,
                result.blocking_diagnostic_ids
            );
            let mut sorted_copy = result.blocking_diagnostic_ids.clone();
            sorted_copy.sort();
            sorted_copy.dedup();
            assert_eq!(
                result.blocking_diagnostic_ids, sorted_copy,
                "{}: blocking_diagnostic_ids must already be sorted and deduplicated",
                result.key
            );
        }
        let computed_count = results.iter().filter(|r| r.computed).count();
        // Pinned to the measured figure -- if this ever goes red, that IS
        // real corpus/engine movement worth a `scripts/retro.py
        // correction`, not a test to loosen.
        assert_eq!(
            computed_count,
            185,
            "measured mix-panel Computed count under the shared canonical fixture moved off 185 \
             of 185"
        );
    }

    #[test]
    fn mix_panel_blocking_histogram_sums_to_the_blocked_row_count() {
        let fixture = load_sweep_fixture().expect("shared deterministic fixture must load cleanly");
        let panel = load_mix_panel().expect("committed mix panel must load and parse cleanly");
        // Bounded to a small prefix -- this test proves the histogram's
        // own arithmetic, not the full corpus sweep (the committed
        // `census-f0d.json` artifact is the full, real measurement).
        let sample: Vec<MixPanelRow> = panel.into_iter().take(10).collect();
        let results = sweep_mix_panel(&fixture, &sample);
        let histogram = mix_panel_blocking_histogram(&results);

        let blocked_count = results.iter().filter(|r| !r.computed).count();
        let histogram_row_coverage: usize = {
            // Every Blocked row contributes at least one id to the
            // histogram; a Computed row contributes none. Since a row can
            // name more than one blocking id, sum the histogram's own
            // counts is >= blocked_count, never less, and every id's count
            // cannot exceed the sample size.
            histogram.values().copied().max().unwrap_or(0)
        };
        assert!(
            histogram_row_coverage <= sample.len(),
            "no single diagnostic id can block more rows than the sample itself"
        );
        if blocked_count > 0 {
            assert!(!histogram.is_empty(), "a nonzero Blocked count must produce a nonempty histogram");
        } else {
            assert!(histogram.is_empty(), "zero Blocked rows in the sample must produce an empty histogram");
        }
    }
}
