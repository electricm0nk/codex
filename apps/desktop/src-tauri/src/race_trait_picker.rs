//! SD-27 Alternate Racial Traits picker — Tauri command adapter over the real
//! on-disk race corpus, read through `codex::rules_core::race_resolver`.
//!
//! # Why this module exists
//!
//! `race_catalog.rs` serves one row per **racial default** trait. ARG's 153
//! *alternate* racial traits — the whole point of the Advanced Race Guide —
//! were ingested (`data/corpus/advanced_race_guide/race_trait/`, 156 records)
//! and reached **no player surface at all**. This module is that surface's
//! backend: the selectable menu, and the real resolution of a selection.
//!
//! # The protocol is not re-implemented here
//!
//! `decisions.md §26`'s rule —
//!
//! > a standard trait applies **iff** no selected alternate trait has set its
//! > `suppressed_by_flag`
//!
//! — is implemented once, in [`RaceCorpus::resolve`]. [`resolve_selection`]
//! *calls* it and reports what it did. Nothing in this file decides whether a
//! trait applies, and nothing in the frontend does either: the suppression the
//! player sees is the resolver's own [`ResolvedRace::suppressions`] list,
//! carried to the screen verbatim.
//!
//! What this module adds is two readings the resolver deliberately does not
//! perform, both of which are transcription rather than interpretation:
//!
//! 1. **The replace map** ([`replacement_targets`]): for each flag an alternate
//!    sets, which standard trait declares that same flag as its
//!    `suppressed_by_flag`, and which replacement row declares it as a positive
//!    `PREFACT` gate. Matched on the flag string, never on trait names.
//! 2. **The mutual-exclusion guard** ([`exclusion_guard_flags`]): ARG rows carry
//!    a `PREMULT:1,[PREABILITY:…this ability…],[!PRE…:…<flag>=true]`
//!    self-exclusion clause — "you may not take a second trait replacing
//!    something you already replaced". `ingest_race_traits_arg.rs` deliberately
//!    preserves it verbatim in `raw_tokens` rather than laundering it into
//!    `suppressed_by_flag` (which is reserved for the standalone `!PREFACT`
//!    shape standard traits use), explicitly leaving it "for downstream
//!    resolvers". This is that downstream reader.
//!
//! # Corpus findings are reported, never hidden
//!
//! An alternate whose replace-flag matches no standard trait is a real data
//! fact, and [`AlternateTraitDto::unmatched_flags`] carries it to the screen
//! where a player can see it. It is not filtered out and the alternate is not
//! suppressed from the menu.
//!
//! **That list is empty today (2026-07-31), and closing it is what made this
//! screen's menu honest.** Nine of the 153 alternates — every Aasimar one —
//! used to sit in that position: offered as checkable rows that
//! `create_character` then refused, because no standard Aasimar row declared
//! the gate their flags fire. The gate was never missing from PCGen, only from
//! the ingest: Aasimar states it in
//! `core_essentials/races/aasimar/aasimar_abilities_globalvar.lst` as
//! `PREVAREQ:<Flag>,0` rather than on the trait row as `!PREFACT`.
//! `src/bin/ingest_races.rs` now reads that second source wherever the row
//! declares nothing, and cross-checks it against the row on the 166 rows where
//! both speak. See [`AlternateTraitDto::unmatched_flags`] for the state this
//! module now pins in both directions.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::race_resolver::{load_race_corpus, RaceCorpus, RaceTraitRecord, TraitRole};

use crate::ge08_workbench::codex_repo_root;

/// The corpus books that carry race content. Identical to
/// `race_catalog::RACE_CORPUS_BOOKS`; duplicated rather than imported because
/// that constant is private to its module and this cycle's write scope does
/// not include editing it. A book with no `race/` or `race_trait/` directory
/// contributes nothing and is not an error.
const RACE_CORPUS_BOOKS: &[&str] = &["core_rulebook", "beastiary", "advanced_race_guide"];

/// Maps a corpus book directory name to the short wire code the Equipment,
/// Spell and Race catalogs already emit. An unrecognized id passes through
/// verbatim rather than being silently relabelled.
fn book_code(book_id: &str) -> String {
    match book_id {
        "core_rulebook" => "CRB".to_string(),
        "beastiary" => "B1".to_string(),
        "advanced_race_guide" => "ARG".to_string(),
        other => other.to_string(),
    }
}

/// `Half-Elf` → `HalfElf`. The same identity rule `race_catalog.rs` uses, so a
/// race's `raceId` is the same string on both screens.
fn race_identity(race_name: &str) -> String {
    race_name.chars().filter(char::is_ascii_alphanumeric).collect()
}

/// A standard trait an alternate swaps out, or a replacement row it brings in.
/// `flag` names the corpus flag that made the link — the evidence, carried so
/// the screen can show *why* rather than assert a relationship.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedTraitDto {
    pub key: String,
    pub name: String,
    pub flag: String,
}

/// One selectable ARG alternate racial trait.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateTraitDto {
    /// The corpus key, e.g. `"Dwarf ~ Saltbeard"`. This is exactly what
    /// [`RaceCorpus::resolve`] takes as a selection, so the screen round-trips
    /// it unchanged.
    pub key: String,
    pub name: String,
    pub book: String,
    /// The record's real corpus `DESC:` prose. Every one of the 153 alternates
    /// carries one (pinned by a test below), so this is never a fabricated
    /// placeholder.
    pub description: String,
    /// `SOURCEPAGE:`, `None` when the corpus row carries the placeholder
    /// `p.xx` (`decisions.md §27.2`). All 153 alternates carry a real page.
    pub source_page: Option<String>,
    /// The `<Race>_Replace<Trait>` flags this alternate sets, verbatim.
    pub sets_flags: Vec<String>,
    /// The standard traits this alternate replaces — resolved by matching each
    /// set flag against standard rows' `suppressed_by_flag`, never guessed
    /// from names.
    pub replaces: Vec<LinkedTraitDto>,
    /// Replacement rows this alternate *grants*: rows gated on a positive
    /// `PREFACT` naming a flag this alternate sets. `Dwarf ~ Saltbeard` grants
    /// `Saltbeard ~ Dwarf ~ Greed`, the seagoing Greed.
    pub grants: Vec<LinkedTraitDto>,
    /// Set flags that match **no** standard trait and grant nothing — a data
    /// finding, surfaced rather than hidden.
    ///
    /// **Empty for all 153 alternates as of 2026-07-31**, and pinned that way
    /// by `no_alternate_in_the_menu_can_ever_be_refused_for_an_unmatched_flag`.
    /// It used to hold 5 distinct flags across 9 alternates, every one of them
    /// Aasimar's:
    /// `core_essentials/races/aasimar/aasimar_abilities_race.lst` contains zero
    /// `PREFACT` tokens, so none of Aasimar's 9 standard rows declared a gate
    /// for ARG's alternates to fire, and `create_character` refused all nine.
    /// Aasimar's gate lives in its sibling
    /// `aasimar_abilities_globalvar.lst` instead, as
    /// `ABILITY:Aasimar Racial Trait|AUTOMATIC|<trait>|PREVAREQ:<Flag>,0` —
    /// the same protocol inverted. `ingest_races` now reads it.
    ///
    /// The field and its rendering stay: a book ingested tomorrow can
    /// reintroduce the shape, and the menu must say so rather than offer a row
    /// that cannot work.
    ///
    /// Deliberately *not* in this list: `Duergar_ReplaceSLAInvisibility`. The
    /// corpus does name it — as the positive `PREFACT` gate on `Duergar ~
    /// Spell-Like Ability ~ Enlarge Person` — so the flag genuinely grants
    /// something and calling it unmatched would be false. What it fails to do
    /// is *suppress* the Invisibility row, because that row's gate names two
    /// flags and the single-valued `suppressed_by_flag` field holds only the
    /// first. [`multi_flag_gate_findings`] reports that, so the two very
    /// different defects are never conflated.
    pub unmatched_flags: Vec<String>,
    /// The flags this alternate's `PREMULT` self-exclusion guard names. While
    /// any of these is already set by a *different* selected alternate, this
    /// one may not be taken.
    pub exclusion_guard_flags: Vec<String>,
}

/// One standard racial trait, as the picker's left-hand column shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardTraitDto {
    pub key: String,
    pub name: String,
    pub book: String,
    pub description: String,
    /// The flag whose firing suppresses this trait, `None` when the corpus row
    /// declares no gate (Aasimar's nine, and every row from a race whose
    /// upstream file predates the flag protocol).
    pub suppressed_by_flag: Option<String>,
}

/// One race's full picker payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RacePickerDto {
    /// Alphanumeric identity, shared with `race_catalog.rs` (`HalfElf`).
    pub race_id: String,
    /// The corpus race key (`"Half-Elf"`) — what the resolver takes.
    pub race_key: String,
    pub race_name: String,
    /// The book the *race* came from (CRB or B1). The alternates are ARG's.
    pub book: String,
    pub standard_traits: Vec<StandardTraitDto>,
    pub alternates: Vec<AlternateTraitDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateRacialTraitsResponse {
    pub races: Vec<RacePickerDto>,
    /// Corpus files that could not be read, plus any failure to locate the
    /// corpus at all. Empty in a healthy checkout.
    pub diagnostics: Vec<String>,
    /// Corpus-quality findings the picker refuses to hide. Each is a plain
    /// sentence naming the affected records. Non-empty today; see
    /// [`multi_flag_gate_findings`].
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceSelectionRequest {
    pub race_key: String,
    #[serde(default)]
    pub selected_alternate_keys: Vec<String>,
}

/// A suppression that fired, both ends named.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuppressionDto {
    pub suppressed_trait_key: String,
    pub suppressed_trait_name: String,
    pub flag: String,
    pub set_by_trait_key: String,
    pub set_by_trait_name: String,
}

/// An alternate that the current selection has locked out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockedAlternateDto {
    pub key: String,
    pub name: String,
    /// The guard flag that is already set.
    pub flag: String,
    pub blocked_by_key: String,
    pub blocked_by_name: String,
}

/// A trait that survived resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppliedTraitDto {
    pub key: String,
    pub name: String,
    pub book: String,
    /// `"default"`, `"alternate"` or `"flagGranted"` — the resolver's own
    /// classification, not a re-derivation.
    pub role: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceSelectionResponse {
    pub race_id: String,
    pub race_key: String,
    pub race_name: String,
    pub book: String,
    pub applied_traits: Vec<AppliedTraitDto>,
    pub suppressions: Vec<SuppressionDto>,
    pub fired_flags: Vec<String>,
    /// Flags that fired but suppressed nothing and granted nothing.
    pub inert_flags: Vec<String>,
    /// Selection keys that matched no alternate for this race.
    pub unmatched_selections: Vec<String>,
    /// Alternates the current selection locks out, per the `PREMULT` guard.
    pub blocked_alternates: Vec<BlockedAlternateDto>,
    /// Selections that violate each other's guard. A UI that disables blocked
    /// options never produces one; a saved character or a scripted caller can,
    /// and it must be reported rather than silently accepted.
    pub conflicting_selections: Vec<BlockedAlternateDto>,
    /// Non-empty only when the request could not be served (unknown race), and
    /// never a silent empty payload.
    pub errors: Vec<String>,
}

fn corpus_root_dir() -> Result<PathBuf, String> {
    codex_repo_root().map(|root| root.join("data/corpus"))
}

/// Loads the real race corpus once per process.
fn race_corpus() -> &'static Result<RaceCorpus, String> {
    static CORPUS: OnceLock<Result<RaceCorpus, String>> = OnceLock::new();
    CORPUS.get_or_init(|| {
        let corpus_root = corpus_root_dir()?;
        let book_dirs: Vec<PathBuf> = RACE_CORPUS_BOOKS.iter().map(|book| corpus_root.join(book)).collect();
        let roots: Vec<BookCorpusRoot<'_>> = RACE_CORPUS_BOOKS
            .iter()
            .zip(book_dirs.iter())
            .map(|(book_id, dir)| BookCorpusRoot { book_id, dir: dir.as_path() })
            .collect();
        Ok(load_race_corpus(&roots))
    })
}

/// Every `<X>_Replace<Y>` flag named inside a **negated** clause of an ARG
/// alternate's `PREMULT` self-exclusion guard, in source order, deduplicated.
///
/// The guard's shape, from `arg_abilities_race.lst:38`:
///
/// ```text
/// PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Magic Resistant],
///           [!PREFACT:1,ABILITIES,Dwarf_ReplaceHardy=true]
/// ```
///
/// Read: satisfied if you already have this ability **or** `Dwarf_ReplaceHardy`
/// is not set. The first branch is PCGen's way of letting an ability satisfy
/// its own prerequisite once granted; the operative constraint for a *new*
/// selection is the second.
///
/// Only bracket groups beginning `!` are read, and within them only clauses
/// whose left-hand side contains `_Replace` — so `CATEGORY=Special Ability` and
/// the ability key in the positive branch contribute nothing. Three ARG rows
/// (`Half-Elf ~ Wary`, `~ Drow-Blooded`, `~ Drow Magic`) write the negated
/// branch as `!PREABILITY:1,CATEGORY=Special Ability,<flag>=true` rather than
/// `!PREFACT` — an upstream token slip, since the operand is unmistakably a
/// fact flag and is the very flag each row sets. Matching on the negation plus
/// the `_Replace` operand rather than on the token name reads all three
/// correctly without inventing anything; [`AlternateRacialTraitsResponse::findings`]
/// reports the slip.
fn exclusion_guard_flags(record: &RaceTraitRecord) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for token in record.data.raw_tokens.iter().filter(|token| token.key == "PREMULT") {
        for group in negated_bracket_groups(&token.value) {
            for clause in group.split(',') {
                let Some((name, value)) = clause.split_once('=') else { continue };
                let name = name.trim();
                if !name.contains("_Replace") || !value.trim().eq_ignore_ascii_case("true") {
                    continue;
                }
                if !out.iter().any(|existing| existing == name) {
                    out.push(name.to_string());
                }
            }
        }
    }
    out
}

/// The contents of every `[...]` group in a `PREMULT` value whose first
/// character is `!`, with that `!` stripped. Nesting does not occur in this
/// token family, so a flat scan is exact rather than approximate.
fn negated_bracket_groups(value: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let bytes = value.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'[' {
            index += 1;
            continue;
        }
        let start = index + 1;
        let Some(offset) = value[start..].find(']') else { break };
        let group = &value[start..start + offset];
        if let Some(rest) = group.strip_prefix('!') {
            out.push(rest);
        }
        index = start + offset + 1;
    }
    out
}

/// For one alternate: the standard traits its flags suppress, the replacement
/// rows its flags grant, and the flags that do neither.
///
/// Matching is on the flag string only. A name-based match would be a guess;
/// this is the same string the corpus writes on both ends.
fn replacement_targets(
    alternate: &RaceTraitRecord,
    siblings: &[&RaceTraitRecord],
) -> (Vec<LinkedTraitDto>, Vec<LinkedTraitDto>, Vec<String>) {
    let mut replaces: Vec<LinkedTraitDto> = Vec::new();
    let mut grants: Vec<LinkedTraitDto> = Vec::new();
    let mut unmatched: Vec<String> = Vec::new();

    for flag in &alternate.data.sets_replace_flags {
        let mut hit = false;
        for sibling in siblings {
            if sibling.data.key == alternate.data.key {
                continue;
            }
            if sibling.role != TraitRole::Alternate && sibling.data.suppressed_by_flag.as_deref() == Some(flag.as_str())
            {
                replaces.push(LinkedTraitDto {
                    key: sibling.data.key.clone(),
                    name: sibling.data.name.clone(),
                    flag: flag.clone(),
                });
                hit = true;
            }
            if sibling.requires_flag.as_deref() == Some(flag.as_str()) {
                grants.push(LinkedTraitDto {
                    key: sibling.data.key.clone(),
                    name: sibling.data.name.clone(),
                    flag: flag.clone(),
                });
                hit = true;
            }
        }
        if !hit {
            unmatched.push(flag.clone());
        }
    }
    (replaces, grants, unmatched)
}

/// Standard rows whose `!PREFACT` gate names more than one flag, of which the
/// single-valued `RaceTraitCacheData::suppressed_by_flag` can hold only the
/// first. The second flag is therefore readable in `raw_tokens` but is *not*
/// acted on by the resolver, so an alternate setting it will not suppress that
/// row. Reporting this as a distinct finding keeps it from being mistaken for
/// [`AlternateTraitDto::unmatched_flags`]' upstream-gap case.
///
/// Exactly 2 rows are affected across the 175 standard-trait records
/// (`Duergar ~ Spell-Like Ability ~ Invisibility` and `~ Enlarge Person`); the
/// count is derived here, never asserted from a doc.
fn multi_flag_gate_findings(corpus: &RaceCorpus) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    for race_key in corpus.race_keys() {
        for record in corpus.traits_for(race_key) {
            if record.role == TraitRole::Alternate {
                continue;
            }
            for token in record.data.raw_tokens.iter().filter(|token| token.key == "!PREFACT") {
                let flags = negated_prefact_flags(&token.value);
                if flags.len() > 1 {
                    rows.push(format!("{} ({})", record.data.key, flags[1..].join(", ")));
                }
            }
        }
    }
    if rows.is_empty() {
        return Vec::new();
    }
    rows.sort();
    vec![format!(
        "{} standard trait row(s) declare a multi-flag `!PREFACT` gate whose trailing flags the \
         single-valued `suppressed_by_flag` field cannot hold; the resolver suppresses on the first \
         flag only: {}",
        rows.len(),
        rows.join("; ")
    )]
}

/// `1,ABILITIES,A=True,B=True` → `["A", "B"]`.
fn negated_prefact_flags(value: &str) -> Vec<String> {
    let mut parts = value.split(',');
    if parts.next() != Some("1") {
        return Vec::new();
    }
    match parts.next() {
        Some(word) if word.eq_ignore_ascii_case("ABILITIES") => {}
        _ => return Vec::new(),
    }
    parts.filter_map(|clause| clause.split_once('=').map(|(flag, _)| flag.trim().to_string())).collect()
}

/// ARG rows that write their guard's negated branch as `!PREABILITY` instead of
/// `!PREFACT`. Derived, not asserted.
fn preability_guard_findings(corpus: &RaceCorpus) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    for race_key in corpus.race_keys() {
        for record in corpus.alternate_traits(race_key) {
            let uses_preability = record
                .data
                .raw_tokens
                .iter()
                .filter(|token| token.key == "PREMULT")
                .any(|token| negated_bracket_groups(&token.value).iter().any(|g| g.starts_with("PREABILITY:")));
            if uses_preability {
                rows.push(record.data.key.clone());
            }
        }
    }
    if rows.is_empty() {
        return Vec::new();
    }
    rows.sort();
    vec![format!(
        "{} ARG alternate(s) write the self-exclusion guard's negated branch as `!PREABILITY` where \
         the operand is a fact flag, not an ability key; read as a guard anyway because the operand \
         is the very flag the row sets: {}",
        rows.len(),
        rows.join("; ")
    )]
}

fn describe_role(role: TraitRole) -> &'static str {
    match role {
        TraitRole::Default => "default",
        TraitRole::Alternate => "alternate",
        TraitRole::FlagGranted => "flagGranted",
        TraitRole::Unclassified => "unclassified",
    }
}

fn build_menu(corpus: &RaceCorpus) -> AlternateRacialTraitsResponse {
    let mut races: Vec<RacePickerDto> = Vec::new();
    for race_key in corpus.race_keys() {
        let Some(chassis) = corpus.chassis(race_key) else { continue };
        let siblings = corpus.traits_for(race_key);

        let standard_traits: Vec<StandardTraitDto> = siblings
            .iter()
            .filter(|record| record.role == TraitRole::Default)
            .map(|record| StandardTraitDto {
                key: record.data.key.clone(),
                name: record.data.name.clone(),
                book: book_code(&record.book_id),
                description: record.data.description.clone().unwrap_or_default(),
                suppressed_by_flag: record.data.suppressed_by_flag.clone(),
            })
            .collect();

        let alternates: Vec<AlternateTraitDto> = siblings
            .iter()
            .filter(|record| record.role == TraitRole::Alternate)
            .map(|record| {
                let (replaces, grants, unmatched_flags) = replacement_targets(record, &siblings);
                AlternateTraitDto {
                    key: record.data.key.clone(),
                    name: record.data.name.clone(),
                    book: book_code(&record.book_id),
                    description: record.data.description.clone().unwrap_or_default(),
                    source_page: record.data.source_page.clone(),
                    sets_flags: record.data.sets_replace_flags.clone(),
                    replaces,
                    grants,
                    unmatched_flags,
                    exclusion_guard_flags: exclusion_guard_flags(record),
                }
            })
            .collect();

        races.push(RacePickerDto {
            race_id: race_identity(&chassis.data.name),
            race_key: chassis.data.key.clone(),
            race_name: chassis.data.name.clone(),
            book: book_code(&chassis.book_id),
            standard_traits,
            alternates,
        });
    }

    let diagnostics =
        corpus.diagnostics().iter().map(|diagnostic| format!("{}: {}", diagnostic.path, diagnostic.message)).collect();

    let mut findings = multi_flag_gate_findings(corpus);
    findings.extend(preability_guard_findings(corpus));

    AlternateRacialTraitsResponse { races, diagnostics, findings }
}

/// Resolves one race against a chosen alternate set, by calling
/// [`RaceCorpus::resolve`] — the single implementation of `decisions.md §26`'s
/// protocol — and reporting exactly what it did.
fn resolve_selection(corpus: &RaceCorpus, race_key: &str, selected: &[String]) -> RaceSelectionResponse {
    let Some(resolved_key) = corpus.resolve_key(race_key) else {
        return RaceSelectionResponse {
            race_id: String::new(),
            race_key: race_key.to_string(),
            race_name: String::new(),
            book: String::new(),
            applied_traits: Vec::new(),
            suppressions: Vec::new(),
            fired_flags: Vec::new(),
            inert_flags: Vec::new(),
            unmatched_selections: Vec::new(),
            blocked_alternates: Vec::new(),
            conflicting_selections: Vec::new(),
            errors: vec![format!("no race in the loaded corpus matches {race_key:?}")],
        };
    };
    let resolved_key = resolved_key.to_string();

    let selection: Vec<&str> = selected.iter().map(String::as_str).collect();
    let Some(race) = corpus.resolve(&resolved_key, &selection) else {
        return RaceSelectionResponse {
            race_id: String::new(),
            race_key: resolved_key.clone(),
            race_name: String::new(),
            book: String::new(),
            applied_traits: Vec::new(),
            suppressions: Vec::new(),
            fired_flags: Vec::new(),
            inert_flags: Vec::new(),
            unmatched_selections: Vec::new(),
            blocked_alternates: Vec::new(),
            conflicting_selections: Vec::new(),
            errors: vec![format!("race {resolved_key:?} has no chassis record")],
        };
    };

    let siblings = corpus.traits_for(&resolved_key);
    let names: BTreeMap<&str, &str> =
        siblings.iter().map(|record| (record.data.key.as_str(), record.data.name.as_str())).collect();

    // Which selected alternate set each fired flag. `resolve` reports the flag
    // set, not the setter, so re-derive the attribution over the same records
    // it read — the same first-writer-wins order.
    let selected_set: BTreeSet<&str> = selection.iter().copied().collect();
    let mut setter_of_flag: BTreeMap<&str, &str> = BTreeMap::new();
    for record in &siblings {
        if record.role != TraitRole::Alternate || !selected_set.contains(record.data.key.as_str()) {
            continue;
        }
        for flag in &record.data.sets_replace_flags {
            setter_of_flag.entry(flag.as_str()).or_insert(record.data.key.as_str());
        }
    }

    let suppressions: Vec<SuppressionDto> = race
        .suppressions
        .iter()
        .map(|suppression| SuppressionDto {
            suppressed_trait_name: names
                .get(suppression.suppressed_trait_key.as_str())
                .copied()
                .unwrap_or(suppression.suppressed_trait_key.as_str())
                .to_string(),
            suppressed_trait_key: suppression.suppressed_trait_key.clone(),
            flag: suppression.flag.clone(),
            set_by_trait_name: names
                .get(suppression.set_by_trait_key.as_str())
                .copied()
                .unwrap_or(suppression.set_by_trait_key.as_str())
                .to_string(),
            set_by_trait_key: suppression.set_by_trait_key.clone(),
        })
        .collect();

    // The `PREMULT` self-exclusion guard, applied against what is already set.
    let mut blocked_alternates: Vec<BlockedAlternateDto> = Vec::new();
    let mut conflicting_selections: Vec<BlockedAlternateDto> = Vec::new();
    for record in &siblings {
        if record.role != TraitRole::Alternate {
            continue;
        }
        let is_selected = selected_set.contains(record.data.key.as_str());
        for flag in exclusion_guard_flags(record) {
            let Some(setter) = setter_of_flag.get(flag.as_str()) else { continue };
            if *setter == record.data.key.as_str() {
                continue;
            }
            let entry = BlockedAlternateDto {
                key: record.data.key.clone(),
                name: record.data.name.clone(),
                flag,
                blocked_by_name: names.get(setter).copied().unwrap_or(setter).to_string(),
                blocked_by_key: (*setter).to_string(),
            };
            if is_selected {
                conflicting_selections.push(entry);
            } else {
                blocked_alternates.push(entry);
            }
        }
    }

    let applied_traits: Vec<AppliedTraitDto> = race
        .traits
        .iter()
        .map(|resolved| AppliedTraitDto {
            key: resolved.key.clone(),
            name: resolved.name.clone(),
            book: book_code(&resolved.book_id),
            role: describe_role(resolved.role).to_string(),
            description: resolved.description.clone().unwrap_or_default(),
        })
        .collect();

    RaceSelectionResponse {
        race_id: race_identity(&race.name),
        race_key: race.race_key.clone(),
        race_name: race.name.clone(),
        book: book_code(&race.book_id),
        applied_traits,
        suppressions,
        fired_flags: race.fired_flags.clone(),
        inert_flags: race.inert_flags.clone(),
        unmatched_selections: race.unmatched_selections.clone(),
        blocked_alternates,
        conflicting_selections,
        errors: Vec::new(),
    }
}

fn menu_or_error() -> AlternateRacialTraitsResponse {
    match race_corpus() {
        Ok(corpus) => build_menu(corpus),
        Err(err) => AlternateRacialTraitsResponse {
            races: Vec::new(),
            diagnostics: vec![format!("race corpus unavailable: {err}")],
            findings: Vec::new(),
        },
    }
}

/// The full alternate-racial-trait menu across every in-scope race. Cached
/// per process, like `race_catalog::build_race_catalog`.
pub fn build_alternate_racial_traits() -> AlternateRacialTraitsResponse {
    static MENU: OnceLock<AlternateRacialTraitsResponse> = OnceLock::new();
    MENU.get_or_init(menu_or_error).clone()
}

/// Resolves a race against a selection. Not cached: the selection is the input.
pub fn build_race_selection(request: &RaceSelectionRequest) -> RaceSelectionResponse {
    match race_corpus() {
        Ok(corpus) => resolve_selection(corpus, &request.race_key, &request.selected_alternate_keys),
        Err(err) => RaceSelectionResponse {
            race_id: String::new(),
            race_key: request.race_key.clone(),
            race_name: String::new(),
            book: String::new(),
            applied_traits: Vec::new(),
            suppressions: Vec::new(),
            fired_flags: Vec::new(),
            inert_flags: Vec::new(),
            unmatched_selections: Vec::new(),
            blocked_alternates: Vec::new(),
            conflicting_selections: Vec::new(),
            errors: vec![format!("race corpus unavailable: {err}")],
        },
    }
}

#[tauri::command]
pub fn list_alternate_racial_traits() -> AlternateRacialTraitsResponse {
    build_alternate_racial_traits()
}

#[tauri::command]
pub fn resolve_race_alternate_selection(request: RaceSelectionRequest) -> RaceSelectionResponse {
    build_race_selection(&request)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every count below was derived by running this module against the real
    /// on-disk corpus:
    ///
    /// ```text
    /// export CARGO_TARGET_DIR=$HOME/.cache/codex-rf-picker
    /// cd apps/desktop/src-tauri && cargo test --bin codex-desktop race_trait_picker
    /// ```
    fn menu() -> AlternateRacialTraitsResponse {
        let response = build_alternate_racial_traits();
        assert!(response.diagnostics.is_empty(), "clean corpus load expected: {:?}", response.diagnostics);
        response
    }

    fn race<'a>(menu: &'a AlternateRacialTraitsResponse, race_id: &str) -> &'a RacePickerDto {
        menu.races.iter().find(|race| race.race_id == race_id).unwrap_or_else(|| panic!("race {race_id} present"))
    }

    fn alternate<'a>(race: &'a RacePickerDto, key: &str) -> &'a AlternateTraitDto {
        race.alternates.iter().find(|alt| alt.key == key).unwrap_or_else(|| panic!("alternate {key} present"))
    }

    /// The whole point: every one of ARG's alternates reaches a player surface,
    /// spread across all 18 in-scope races.
    #[test]
    fn every_arg_alternate_reaches_the_menu_across_all_eighteen_in_scope_races() {
        let menu = menu();
        assert_eq!(menu.races.len(), 18, "18 in-scope races (decisions.md §25.3)");
        let total: usize = menu.races.iter().map(|race| race.alternates.len()).sum();
        assert_eq!(total, 153, "ARG's 153 Alternate-classified records");

        // Per-race counts, derived from the corpus by this very menu.
        let expected: &[(&str, usize)] = &[
            ("Aasimar", 9),
            ("Drow", 6),
            ("Duergar", 5),
            ("Dwarf", 17),
            ("Elf", 13),
            ("Gnome", 12),
            ("Goblin", 7),
            ("HalfElf", 9),
            ("HalfOrc", 14),
            ("Halfling", 13),
            ("Hobgoblin", 9),
            ("Human", 15),
            ("Kobold", 4),
            ("Merfolk", 3),
            ("Orc", 4),
            ("Svirfneblin", 2),
            ("Tengu", 4),
            ("Tiefling", 7),
        ];
        for (race_id, count) in expected {
            assert_eq!(race(&menu, race_id).alternates.len(), *count, "{race_id} alternate count");
        }
        assert_eq!(expected.iter().map(|(_, n)| n).sum::<usize>(), 153);
    }

    /// Every alternate is ARG's, and carries real prose and a real page —
    /// no empty cells reaching the screen.
    #[test]
    fn every_alternate_carries_arg_attribution_real_prose_and_a_real_page() {
        let menu = menu();
        for race in &menu.races {
            for alternate in &race.alternates {
                assert_eq!(alternate.book, "ARG", "{} book", alternate.key);
                assert!(!alternate.description.trim().is_empty(), "{} description", alternate.key);
                assert!(!alternate.sets_flags.is_empty(), "{} sets at least one flag", alternate.key);
                let page = alternate.source_page.as_deref().unwrap_or_default();
                assert!(!page.is_empty() && page != "p.xx", "{} real source page, got {page:?}", alternate.key);
            }
        }
    }

    /// The replace map is resolved through the flag, and the flag alone.
    /// `Dwarf ~ Saltbeard` sets 4 flags: 3 named by its own `PREMULT` guard
    /// and `Dwarf_ReplaceGreed`, which it sets but does not guard.
    #[test]
    fn saltbeard_replaces_the_four_standard_traits_its_flags_name_and_grants_its_replacement() {
        let menu = menu();
        let dwarf = race(&menu, "Dwarf");
        let saltbeard = alternate(dwarf, "Dwarf ~ Saltbeard");

        assert_eq!(
            saltbeard.sets_flags,
            vec![
                "Dwarf_ReplaceDefensiveTraining",
                "Dwarf_ReplaceHatred",
                "Dwarf_ReplaceStonecunning",
                "Dwarf_ReplaceGreed",
            ]
        );
        let replaced: Vec<&str> = saltbeard.replaces.iter().map(|link| link.key.as_str()).collect();
        assert_eq!(
            replaced,
            vec!["Dwarf ~ Defensive Training", "Dwarf ~ Hatred", "Dwarf ~ Stonecunning", "Dwarf ~ Greed"]
        );
        // Every link names the flag that made it — the evidence, not a guess.
        for link in &saltbeard.replaces {
            assert!(saltbeard.sets_flags.contains(&link.flag), "{} link flag is one it sets", link.key);
        }
        assert!(saltbeard.unmatched_flags.is_empty(), "{:?}", saltbeard.unmatched_flags);

        // ARG's replacement Greed is granted by the same flag that suppresses
        // the CRB one.
        let granted: Vec<&str> = saltbeard.grants.iter().map(|link| link.key.as_str()).collect();
        assert_eq!(granted, vec!["Saltbeard ~ Dwarf ~ Greed"]);
        assert_eq!(saltbeard.grants[0].flag, "Dwarf_ReplaceGreed");

        // The guard names 3 of the 4 — read off `PREMULT`, not off the flags.
        assert_eq!(
            saltbeard.exclusion_guard_flags,
            vec!["Dwarf_ReplaceDefensiveTraining", "Dwarf_ReplaceHatred", "Dwarf_ReplaceStonecunning"]
        );
    }

    /// Every alternate has a readable self-exclusion guard, including the
    /// three that spell the negated branch `!PREABILITY`.
    #[test]
    fn every_alternate_has_a_readable_exclusion_guard_including_the_preability_spelling() {
        let menu = menu();
        for race in &menu.races {
            for alternate in &race.alternates {
                assert!(!alternate.exclusion_guard_flags.is_empty(), "{} has a guard", alternate.key);
            }
        }
        let half_elf = race(&menu, "HalfElf");
        let wary = alternate(half_elf, "Half-Elf ~ Wary");
        assert_eq!(wary.exclusion_guard_flags, vec!["HalfElf_ReplaceKeenSenses"]);
        assert!(menu.findings.iter().any(|finding| finding.contains("Half-Elf ~ Wary")));
    }

    /// **No row in this menu can be ticked and then refused.**
    ///
    /// `character_hub::resolve_alternate_trait_choices` blocks a save on
    /// exactly one picker-visible condition: a chosen alternate firing a flag
    /// that suppresses and grants nothing. Nine rows — every Aasimar
    /// alternate — used to be in that state unconditionally, for every build,
    /// forever. This asserts the menu and the resolver agree for all 153.
    #[test]
    fn no_alternate_in_the_menu_can_ever_be_refused_for_an_unmatched_flag() {
        let menu = menu();
        let corpus = race_corpus().as_ref().expect("corpus");

        let mut unmatched: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut checked = 0usize;
        for race in &menu.races {
            for alternate in &race.alternates {
                for flag in &alternate.unmatched_flags {
                    unmatched.entry(flag.clone()).or_default().push(alternate.key.clone());
                }
                // The menu's own claim, and the resolver's answer to it.
                let resolved =
                    resolve_selection(corpus, &race.race_key, std::slice::from_ref(&alternate.key));
                assert!(
                    resolved.inert_flags.is_empty(),
                    "{} is offered and `create_character` would refuse it: {:?}",
                    alternate.key,
                    resolved.inert_flags
                );
                assert!(
                    !alternate.replaces.is_empty() || !alternate.grants.is_empty(),
                    "{} replaces and grants nothing — a row that changes no sheet",
                    alternate.key
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 153);
        assert!(unmatched.is_empty(), "no alternate may name a flag nothing declares: {unmatched:?}");

        // Aasimar is the worked case: its nine standard rows now declare the
        // gate its nine alternates fire, read from `aasimar_abilities_globalvar
        // .lst`. Asserted off the payload, so this is the shipped DTO, not a
        // corpus-only fact.
        let aasimar = race(&menu, "Aasimar");
        assert_eq!(aasimar.standard_traits.len(), 9);
        assert!(
            aasimar.standard_traits.iter().all(|row| row.suppressed_by_flag.is_some()),
            "every Aasimar standard row carries its gate"
        );
        assert_eq!(aasimar.alternates.len(), 9);
        for alternate in &aasimar.alternates {
            assert!(!alternate.replaces.is_empty(), "{} really replaces something", alternate.key);
        }

        // `Duergar_ReplaceSLAInvisibility` was never one of the nine and is
        // still not: the corpus names it — as the positive `PREFACT` gate on
        // `Duergar ~ Spell-Like Ability ~ Enlarge Person`, which the flag
        // therefore grants — so calling it unmatched would be false. Its real
        // defect (the *suppression* half is lost to a single-valued field) is
        // reported by `multi_flag_gate_findings` instead.
        let duergar = race(&menu, "Duergar");
        let grants: Vec<&str> = duergar
            .alternates
            .iter()
            .flat_map(|alternate| alternate.grants.iter())
            .filter(|link| link.flag == "Duergar_ReplaceSLAInvisibility")
            .map(|link| link.key.as_str())
            .collect();
        assert_eq!(grants, vec!["Duergar ~ Spell-Like Ability ~ Enlarge Person"]);
    }

    /// `Duergar_ReplaceSLAInvisibility` is *not* the same case: the corpus does
    /// name it, in the second position of a two-flag `!PREFACT` the
    /// single-valued field cannot hold. Reported separately.
    #[test]
    fn the_two_multi_flag_gate_rows_are_reported_as_their_own_distinct_finding() {
        let menu = menu();
        let finding = menu
            .findings
            .iter()
            .find(|finding| finding.contains("multi-flag"))
            .expect("multi-flag gate finding present");
        assert!(finding.starts_with("2 standard trait row(s)"), "got {finding}");
        assert!(finding.contains("Duergar ~ Spell-Like Ability ~ Invisibility (Duergar_ReplaceSLAInvisibility)"));
        assert!(finding.contains("Duergar ~ Spell-Like Ability ~ Enlarge Person (Duergar_ReplaceSLAEnlargePerson)"));
    }

    /// Selecting an alternate really suppresses the standard trait, because the
    /// resolver says so — this asserts against `RaceCorpus::resolve`'s output,
    /// which is the only implementation of the protocol.
    #[test]
    fn selecting_saltbeard_suppresses_the_four_standard_traits_and_swaps_in_its_own_greed() {
        let corpus = race_corpus().as_ref().expect("corpus");
        let before = resolve_selection(corpus, "Dwarf", &[]);
        assert!(before.errors.is_empty());
        assert_eq!(before.applied_traits.len(), 12, "Dwarf's 12 racial defaults");
        assert!(before.suppressions.is_empty());
        assert!(before.applied_traits.iter().any(|applied| applied.key == "Dwarf ~ Greed"));

        let after = resolve_selection(corpus, "Dwarf", &["Dwarf ~ Saltbeard".to_string()]);
        assert!(after.errors.is_empty());
        assert!(after.unmatched_selections.is_empty());
        assert!(after.inert_flags.is_empty());

        let suppressed: Vec<&str> = after.suppressions.iter().map(|s| s.suppressed_trait_key.as_str()).collect();
        assert_eq!(
            suppressed,
            vec!["Dwarf ~ Defensive Training", "Dwarf ~ Greed", "Dwarf ~ Hatred", "Dwarf ~ Stonecunning"]
        );
        for suppression in &after.suppressions {
            assert_eq!(suppression.set_by_trait_key, "Dwarf ~ Saltbeard");
            assert_eq!(suppression.set_by_trait_name, "Saltbeard");
        }

        let applied: BTreeSet<&str> = after.applied_traits.iter().map(|a| a.key.as_str()).collect();
        assert!(!applied.contains("Dwarf ~ Greed"), "the CRB Greed is gone");
        assert!(applied.contains("Saltbeard ~ Dwarf ~ Greed"), "ARG's Greed took its place");
        assert!(applied.contains("Dwarf ~ Saltbeard"), "the chosen alternate itself applies");
        assert_eq!(after.applied_traits.len(), 12 - 4 + 1 + 1);
    }

    /// The `PREMULT` guard is honoured: with Saltbeard taken, no other Dwarf
    /// alternate replacing defensive training, hatred or stonecunning may be.
    #[test]
    fn selecting_an_alternate_blocks_every_sibling_whose_guard_names_a_fired_flag() {
        let corpus = race_corpus().as_ref().expect("corpus");
        let open = resolve_selection(corpus, "Dwarf", &[]);
        assert!(open.blocked_alternates.is_empty(), "nothing is blocked before a selection");

        let after = resolve_selection(corpus, "Dwarf", &["Dwarf ~ Saltbeard".to_string()]);
        let blocked: BTreeSet<&str> = after.blocked_alternates.iter().map(|b| b.key.as_str()).collect();
        assert!(!blocked.is_empty(), "Saltbeard locks out its rivals");
        assert!(!blocked.contains("Dwarf ~ Saltbeard"), "an alternate never blocks itself");
        for entry in &after.blocked_alternates {
            assert_eq!(entry.blocked_by_key, "Dwarf ~ Saltbeard");
            assert!(after.fired_flags.contains(&entry.flag), "blocked on a flag that actually fired");
        }
        // Magic Resistant replaces Hardy, which Saltbeard does not touch.
        assert!(!blocked.contains("Dwarf ~ Magic Resistant"), "an unrelated swap stays available");

        // Every blocked alternate's guard really does name a fired flag —
        // asserted against the menu, so the two commands agree.
        let menu = menu();
        let dwarf = race(&menu, "Dwarf");
        for entry in &after.blocked_alternates {
            let alternate = alternate(dwarf, &entry.key);
            assert!(alternate.exclusion_guard_flags.contains(&entry.flag));
        }
    }

    /// A selection that violates a guard is reported, not silently accepted.
    ///
    /// The report names the pair once, from whichever end reads the flag it did
    /// not itself set — a flag has exactly one recorded setter, so a mutual
    /// exclusion surfaces as one entry naming both traits, not two.
    #[test]
    fn two_alternates_that_exclude_each_other_are_reported_as_conflicting() {
        let corpus = race_corpus().as_ref().expect("corpus");
        let after = resolve_selection(corpus, "Dwarf", &["Dwarf ~ Saltbeard".to_string()]);
        let rival = after.blocked_alternates.first().expect("at least one rival").key.clone();

        let both = resolve_selection(corpus, "Dwarf", &["Dwarf ~ Saltbeard".to_string(), rival.clone()]);
        let pair: BTreeSet<&str> = both
            .conflicting_selections
            .iter()
            .flat_map(|conflict| [conflict.key.as_str(), conflict.blocked_by_key.as_str()])
            .collect();
        assert!(
            pair.contains("Dwarf ~ Saltbeard") && pair.contains(rival.as_str()),
            "the illegal pair is surfaced: {:?}",
            both.conflicting_selections
        );
        // A selected trait is never also reported as merely "blocked".
        assert!(both.blocked_alternates.iter().all(|blocked| blocked.key != rival));
        assert!(both.blocked_alternates.iter().all(|blocked| blocked.key != "Dwarf ~ Saltbeard"));
    }

    /// A typo'd selection is reported rather than ignored, and an unknown race
    /// produces an error rather than an empty-looking success.
    #[test]
    fn a_bad_selection_or_race_reports_rather_than_returning_a_quiet_empty_payload() {
        let corpus = race_corpus().as_ref().expect("corpus");
        let typo = resolve_selection(corpus, "Dwarf", &["Dwarf ~ Saltbeerd".to_string()]);
        assert_eq!(typo.unmatched_selections, vec!["Dwarf ~ Saltbeerd"]);
        assert_eq!(typo.applied_traits.len(), 12, "the race still resolves plainly");

        let unknown = resolve_selection(corpus, "Balor", &[]);
        assert_eq!(unknown.applied_traits.len(), 0);
        assert_eq!(unknown.errors.len(), 1);
        assert!(unknown.errors[0].contains("Balor"));
    }

    /// The menu's `raceKey` round-trips into the resolve command for every
    /// race, and every alternate key the menu offers is a key the resolver
    /// accepts. A menu offering something the resolver rejects would be a dead
    /// affordance.
    #[test]
    fn every_menu_key_round_trips_through_the_resolve_command() {
        let corpus = race_corpus().as_ref().expect("corpus");
        let menu = menu();
        for race in &menu.races {
            for alternate in &race.alternates {
                let response = resolve_selection(corpus, &race.race_key, std::slice::from_ref(&alternate.key));
                assert!(response.errors.is_empty(), "{} resolves: {:?}", race.race_key, response.errors);
                assert!(
                    response.unmatched_selections.is_empty(),
                    "{} is a key the resolver accepts",
                    alternate.key
                );
                assert!(
                    response.applied_traits.iter().any(|applied| applied.key == alternate.key),
                    "{} actually applies once selected",
                    alternate.key
                );
                // Whatever the menu promised it replaces, the resolver really
                // suppressed — the picker cannot promise a swap that will not
                // happen.
                let suppressed: BTreeSet<&str> =
                    response.suppressions.iter().map(|s| s.suppressed_trait_key.as_str()).collect();
                for link in &alternate.replaces {
                    assert!(
                        suppressed.contains(link.key.as_str()),
                        "{} promised to replace {} and the resolver did not",
                        alternate.key,
                        link.key
                    );
                }
            }
        }
    }

    /// `resolve_key` is case- and prefix-tolerant, so a `race:half-elf`
    /// character-input token reaches the same race as the menu's `Half-Elf`.
    #[test]
    fn a_loose_race_identifier_reaches_the_same_race_as_the_menu_key() {
        let corpus = race_corpus().as_ref().expect("corpus");
        let loose = resolve_selection(corpus, "race:half-elf", &[]);
        assert!(loose.errors.is_empty());
        assert_eq!(loose.race_key, "Half-Elf");
        assert_eq!(loose.race_id, "HalfElf");
    }
}
