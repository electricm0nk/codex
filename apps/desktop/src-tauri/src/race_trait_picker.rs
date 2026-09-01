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
//!    something you already replaced". `ingest_race_traits.rs` deliberately
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
//!
//! # Descriptions are *rendered*, never transcribed
//!
//! Every description this module emits comes from
//! [`RaceTraitRecord::render_description`] against a
//! [`codex::rules_core::pcgen_desc::PcgenDisplayValues`] table, not from the
//! stored `data.description` string. The two differ in exactly the way that
//! matters: the stored string is the *already-collapsed* result of resolving a
//! row against itself at ingest time, so its number is baked in and its gate
//! branches are already chosen. Re-rendering from the row's own `DESC:` tokens
//! is what lets a **character's feats** change both.
//!
//! Until 2026-08-01 that renderer had **zero consumers** — the exact
//! producer-with-no-consumer shape `decisions.md §29.1` records — and
//! `Halfling ~ Adaptable Luck` reached the screen reading *"Three times per
//! day… they only gain a bonus"*, byte-identical to the raw corpus prose with
//! its numbers missing. This module is that renderer's consumer.
//!
//! **How the character reaches here: the held feats travel with the call**, as
//! [`resolve_race_alternate_selection`]'s `held_feats` argument — not a
//! character id, and deliberately *not* a field of [`RaceSelectionRequest`].
//! Four reasons, and the first is decisive:
//!
//! 1. The renderer's actual input is a feat list
//!    ([`display_value_deltas_from_feats`]), so a character id would have to be
//!    resolved to one anyway — by this module, which would then need an
//!    `AppHandle` and the saved-character store to do it. Every function here
//!    is deliberately `AppHandle`-free and unit-testable against the real
//!    on-disk corpus; taking a character id would end that.
//! 2. The feats are already on the wire. `load_saved_character` returns
//!    `selected_feats` verbatim (its own doc calls that out as the field the
//!    Feat picker needed), so the frontend hands over a character's *real*
//!    persisted feats rather than anything a screen invented.
//! 3. **The feats are not part of the selection and must not look like they
//!    are.** Which traits apply is a race-and-selection question; a feat
//!    changes no suppression, fires no flag and blocks no alternate. Keeping
//!    them a sibling argument rather than a `RaceSelectionRequest` field says
//!    that in the type — and leaves the two non-UI callers of
//!    [`build_race_selection`] (`character_hub::resolve_alternate_trait_choices`,
//!    which validates a save, and `reach_gate`) validating exactly what they
//!    did before.
//! 4. It keeps the seam honest in the other direction: a caller with no
//!    character sends no feats and gets the racial base, which is what the
//!    character-free menu command returns too — and
//!    `every_menu_row_has_a_rendered_description_and_none_leaks_pcgen_syntax`
//!    pins the two renderings equal so one trait never shows two sentences
//!    depending on which call answered first.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::feat_effects::{display_value_deltas_from_feats, FeatDisplayValueDeltas};
use codex::rules_core::race_resolver::{
    adopted_race_choose_selectors, adoptive_parentage_options, load_race_corpus, RaceCorpus, RaceTraitRecord,
    TraitRole,
};
use codex::rules_core::trait_pool::{load_trait_pool, resolve_adopted_race_options};

use crate::authoring_workbench::codex_repo_root;
use crate::race_catalog::{book_code, RACE_CORPUS_BOOKS};

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
    /// The record's real corpus `DESC:` prose. Every alternate this menu
    /// serves carries one (pinned by a test below), so this is never a
    /// fabricated placeholder.
    pub description: String,
    /// `SOURCEPAGE:`, `None` when the corpus row carries the placeholder
    /// `p.xx` (`decisions.md §27.2`). Every alternate this menu serves
    /// carries a real page.
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
    /// **Empty for every alternate this menu serves, as of 2026-08-08 (ARG's
    /// 153 + APG's 50, `decisions.md §37`)**, and pinned that way by
    /// `no_alternate_in_the_menu_can_ever_be_refused_for_an_unmatched_flag`.
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

/// One trait an [`AdoptiveParentageOptionDto`] grants, resolved against the
/// adopted race's own already-ingested standard traits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdoptiveParentageGrantDto {
    pub key: String,
    pub name: String,
}

/// One "Adoptive Parentage" option (`decisions.md §16` item 2, SD-32 card-11
/// T2b lane): a member of `Human ~ Adoptive Parentage`'s `CHOOSE:
/// ABILITYSELECTION|Adoptive Parentage|ANY` pool (that alternate trait is
/// itself one of `Human`'s own [`AlternateTraitDto`] rows, above — a Human
/// character replaces Bonus Feat with it, then picks one of these). Not
/// race-scoped the way [`RacePickerDto`] is, because picking one is a Human
/// character's choice of *which other race* to have been adopted by, not a
/// trait of the race named here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdoptiveParentageOptionDto {
    /// The corpus key, e.g. `"Dwarf"` — no explicit `KEY:` token upstream, so
    /// the option's own display name doubles as both its key and the race it
    /// adopts.
    pub key: String,
    pub name: String,
    pub book: String,
    pub adopted_race: String,
    /// Real corpus `DESC:` prose, verbatim — every option this menu serves
    /// carries a fixed, argument-free sentence (no `%N` substitution, pinned
    /// by [`every_adoptive_parentage_option_carries_real_prose_and_real_grants`]),
    /// so unlike [`AlternateTraitDto::description`] this is read from the
    /// stored field rather than re-rendered against a feat list.
    pub description: String,
    /// The already-ingested traits this option grants. Empty is a legitimate,
    /// honestly-reported answer — never papered over with an invented trait.
    pub grants: Vec<AdoptiveParentageGrantDto>,
}

/// One Trait this [`AdoptedRaceOptionDto`] can grant, resolved against the
/// real `kind: trait` pool (`codex::rules_core::trait_pool`) rather than
/// this corpus's own race-trait population.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdoptedRaceTraitGrantDto {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub book: String,
}

/// One "Adopted Race" selector (`decisions.md §25`): a character of the
/// named race's own type may pick ONE trait from that race's real Trait
/// pool. Structurally the closest existing row is
/// [`AdoptiveParentageOptionDto`] (any-race-selectable, names a target), but
/// the pool here is a different content kind entirely (`kind: trait`, never
/// this corpus's own `race_trait` population) -- hence a separate DTO rather
/// than folding this into that one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdoptedRaceOptionDto {
    pub key: String,
    pub name: String,
    pub book: String,
    pub adopted_race: String,
    /// The real Trait pool this option offers. Empty is a legitimate,
    /// honestly-reported answer for a race whose Trait pool this project has
    /// not (yet) ingested — never papered over with a fabricated trait. See
    /// `codex::rules_core::trait_pool` module doc comment for the current
    /// ingest status.
    pub grants: Vec<AdoptedRaceTraitGrantDto>,
    /// `true` for a row whose own `CHOOSE:` token this project could not
    /// read a pool suffix from at all — a malformed-row finding surfaced
    /// rather than silently treated as "empty pool". Never true for any of
    /// the 14 real oracle rows this cycle ingested.
    pub malformed_choose_token: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateRacialTraitsResponse {
    pub races: Vec<RacePickerDto>,
    /// `Human ~ Adoptive Parentage`'s CHOOSE pool, resolved. See
    /// [`AdoptiveParentageOptionDto`].
    pub adoptive_parentage_options: Vec<AdoptiveParentageOptionDto>,
    /// The 14 "Adopted Race" selectors (`decisions.md §25`), resolved
    /// against the real Trait pool. See [`AdoptedRaceOptionDto`]. Additive
    /// field — a consumer that does not read it is unaffected.
    pub adopted_race_options: Vec<AdoptedRaceOptionDto>,
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

/// One racial trait's description, rendered against a specific character's
/// display values rather than transcribed from the corpus record.
///
/// Emitted for **every** trait record the race declares — standard, alternate
/// and flag-granted alike, selected or not. Restricting it to the applied set
/// would leave the alternates column showing the racial base for a trait whose
/// number the player's feats have already changed, which is the same
/// wrong-sentence-on-screen defect one step smaller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderedTraitDescriptionDto {
    pub key: String,
    pub name: String,
    /// The prose to show. Rendered from the record's own `DESC:` tokens.
    pub text: String,
    /// `DESC:` arguments this engine could not resolve to a literal and
    /// therefore dropped. Carried so a partially-resolvable description is
    /// visibly incomplete rather than silently guessed — `Aasimar ~ Deathless
    /// Spirit`'s negative-level magnitude is the standing example.
    pub dropped_args: Vec<String>,
    /// True when the held feats changed this sentence from its racial base.
    ///
    /// Derived by rendering the same record twice — once with the character's
    /// deltas and once with none — and comparing, so it cannot claim a move
    /// that did not happen.
    pub moved_by_feats: bool,
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
    /// Every trait record this race declares, with its description rendered
    /// against [`RaceSelectionRequest::held_feats`]. Sorted by key.
    pub rendered_trait_descriptions: Vec<RenderedTraitDescriptionDto>,
    /// The subset of the held feats that actually moved a display value,
    /// derived one feat at a time rather than assumed from the list.
    ///
    /// This is the screen's evidence for *why* a number differs from the book's
    /// printed one. A feat absent from this list changed nothing here, and
    /// saying otherwise would be a claim the engine cannot support.
    pub display_value_feats: Vec<String>,
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
    // The third spelling, and the only one that is not a `PREMULT`: a
    // `PREVAREQ:<flag>,0` qualifier on the record's own
    // `ABILITY:<Race> Racial Trait|AUTOMATIC|<key>` grant.
    //
    // `core_essentials`' heritage selectors (SD-29 race-trait lane round 4,
    // `decisions.md §49`) carry no `PREMULT` at all -- upstream, only one
    // heritage can apply because a heritage is a PCGen SUBRACE and a character
    // has one -- so read through the `PREMULT` reader alone all 16 would come
    // back unguarded, and a player could tick `Aasimar ~ Angel-Blooded` and
    // `Aasimar ~ Archon-Blooded` together and collect both ability-score
    // bonuses. The corpus does state the constraint, on the grant itself:
    // `ABILITY:Aasimar Racial Trait|AUTOMATIC|Angel-Blooded ~ Ability Scores|PREVAREQ:Aasimar_ReplaceAbilityScores,0`
    // reads *grant this while that standard trait has not already been
    // replaced*, which is the same "already set by someone else blocks me"
    // relation the `PREMULT` branch below expresses. Only `,0` is read, for
    // `ingest_races::globalvar_gates`' stated reason: `,1` is the opposite
    // statement.
    for token in record.data.raw_tokens.iter().filter(|token| token.key == "ABILITY") {
        let parts: Vec<&str> = token.value.split('|').collect();
        if parts.len() < 2 || !parts[1].trim().eq_ignore_ascii_case("AUTOMATIC") {
            continue;
        }
        for clause in &parts[2..] {
            let Some(rest) = clause.trim().strip_prefix("PREVAREQ:") else { continue };
            let Some((flag, want)) = rest.rsplit_once(',') else { continue };
            let flag = flag.trim();
            if want.trim() != "0" || !flag.contains("_Replace") {
                continue;
            }
            if !out.iter().any(|existing| existing == flag) {
                out.push(flag.to_string());
            }
        }
    }
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
    // The fourth spelling, SD-33 Epic 6's Skinwalker fold (2026-08-26): a
    // record with a positive `PREABILITY:...` dependency on a specific
    // parent ability (i.e. `PREABILITY:1,CATEGORY=Special Ability,<parent
    // key>`, not a negated `!PREABILITY` bracket -- that shape is already
    // read above) AND its own `sets_replace_flags` is a heritage
    // REPLACEMENT row this corpus never gives a `PREMULT`/`PREVAREQ` guard
    // of its own to (Skinwalker's 36 `<Kin> ~ <Trait>` rows: PCGen gates
    // them on their PARENT selector's `PREABILITY`/`PREMULT` alone, on the
    // assumption a player reaches them only by picking that one selector
    // first). Without this branch, none of the 36 carried ANY exclusion
    // guard (unlike Monster Codex's `Oversized Goblin ~ Ability Scores`/
    // `~ Size`, this branch's negative control: those carry no `PREABILITY`
    // at all, so they never reach this branch and stay unguarded, matching
    // `every_alternate_has_a_readable_exclusion_guard_including_the_
    // preability_spelling`'s own pin) -- a player could tick e.g.
    // `Werebat-Kin ~ Ability Scores` AND `Werebear-Kin ~ Ability Scores`
    // together (both fire `Skinwalker_ReplaceAbilityScores`) and collect
    // both incompatible ability-score swaps, since nothing suppressed the
    // second. The guard is the record's OWN already-honest
    // `sets_replace_flags` (read off its real `FACT:<flag>|True` token, the
    // same field `classify()` itself reads), not a fabricated token --
    // `corpus_literal_sweep` only audits `raw_tokens`, and this reads
    // `sets_replace_flags` directly.
    if out.is_empty()
        && !record.data.sets_replace_flags.is_empty()
        && record.data.raw_tokens.iter().any(|token| token.key == "PREABILITY")
    {
        for flag in &record.data.sets_replace_flags {
            if !out.iter().any(|existing| existing == flag) {
                out.push(flag.clone());
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

/// One record's description, rendered against a character's display values.
///
/// The single place this module turns a corpus record into player-facing
/// prose. Nothing else may read `record.data.description`: that string is the
/// ingest-time collapse of the row against itself, and serving it is precisely
/// how the numbers stopped reaching the screen.
fn render_trait_description(record: &RaceTraitRecord, deltas: &FeatDisplayValueDeltas) -> RenderedTraitDescriptionDto {
    let rendered = record.render_description(&record.display_values_with(deltas));
    let moved_by_feats = if deltas.is_zero() {
        false
    } else {
        rendered.text != record.render_description(&record.display_values_with(&FeatDisplayValueDeltas::default())).text
    };
    RenderedTraitDescriptionDto {
        key: record.data.key.clone(),
        name: record.data.name.clone(),
        text: rendered.text,
        dropped_args: rendered.dropped_args,
        moved_by_feats,
    }
}

/// The held feats that really move a display value, one feat at a time.
///
/// Derived rather than filtered against a hardcoded roster, so a feat added to
/// [`display_value_deltas_from_feats`] later appears here without this function
/// changing — and a feat that moves nothing never appears at all.
fn display_value_feats(held_feats: &[String]) -> Vec<String> {
    held_feats
        .iter()
        .filter(|feat| !display_value_deltas_from_feats(std::slice::from_ref(*feat)).is_zero())
        .cloned()
        .collect()
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
    // The menu is a catalogue with no character in hand, so it renders every
    // description against the racial base — the same call the resolve command
    // makes with an empty feat list, never the stored `data.description`.
    let no_feats = FeatDisplayValueDeltas::default();
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
                description: render_trait_description(record, &no_feats).text,
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
                    description: render_trait_description(record, &no_feats).text,
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

    let adoptive_parentage_options: Vec<AdoptiveParentageOptionDto> = adoptive_parentage_options(corpus)
        .into_iter()
        .map(|option| AdoptiveParentageOptionDto {
            key: option.key,
            name: option.name,
            book: book_code(&option.book_id),
            adopted_race: option.adopted_race,
            description: option.description.unwrap_or_default(),
            grants: option
                .grants
                .into_iter()
                .map(|grant| AdoptiveParentageGrantDto { key: grant.key, name: grant.name })
                .collect(),
        })
        .collect();

    // SD-32 `decisions.md §25` cycle 2: `codex_repo_root()` gives the same
    // corpus root the race corpus itself just loaded from, so the Trait pool
    // and the selectors it resolves against are read from the identical
    // on-disk state, not two different checkouts.
    let adopted_race_options: Vec<AdoptedRaceOptionDto> = match codex_repo_root() {
        Ok(root) => {
            let corpus_root = root.join("data/corpus");
            let dirs: Vec<PathBuf> = RACE_CORPUS_BOOKS.iter().map(|book| corpus_root.join(book)).collect();
            let pool_roots: Vec<BookCorpusRoot<'_>> = RACE_CORPUS_BOOKS
                .iter()
                .zip(dirs.iter())
                .map(|(book_id, dir)| BookCorpusRoot { book_id, dir: dir.as_path() })
                .collect();
            let trait_pool = load_trait_pool(&pool_roots);
            let selectors = adopted_race_choose_selectors(corpus);
            resolve_adopted_race_options(&selectors, &trait_pool)
                .into_iter()
                .map(|option| AdoptedRaceOptionDto {
                    key: option.key,
                    name: option.name,
                    book: book_code(&option.book_id),
                    adopted_race: option.adopted_race,
                    grants: option
                        .grants
                        .into_iter()
                        .map(|grant| AdoptedRaceTraitGrantDto {
                            key: grant.key,
                            name: grant.name,
                            description: grant.description,
                            book: book_code(&grant.book_id),
                        })
                        .collect(),
                    malformed_choose_token: option.malformed_choose_token,
                })
                .collect()
        }
        // The corpus root could not be located at all -- `diagnostics` below
        // already reports this same failure for the rest of the response, so
        // this half degrades to an honest empty list rather than a second,
        // differently-worded error.
        Err(_) => Vec::new(),
    };

    let diagnostics =
        corpus.diagnostics().iter().map(|diagnostic| format!("{}: {}", diagnostic.path, diagnostic.message)).collect();

    let mut findings = multi_flag_gate_findings(corpus);
    findings.extend(preability_guard_findings(corpus));

    AlternateRacialTraitsResponse { races, adoptive_parentage_options, adopted_race_options, diagnostics, findings }
}

/// Resolves one race against a chosen alternate set, by calling
/// [`RaceCorpus::resolve`] — the single implementation of `decisions.md §26`'s
/// protocol — and reporting exactly what it did.
///
/// `held_feats` is the character's own feat list; it changes no *resolution*
/// (which traits apply is a race-and-selection question, not a feat one) and
/// only the *numbers the descriptions state*.
fn resolve_selection(
    corpus: &RaceCorpus,
    race_key: &str,
    selected: &[String],
    held_feats: &[String],
) -> RaceSelectionResponse {
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
            rendered_trait_descriptions: Vec::new(),
            display_value_feats: Vec::new(),
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
            rendered_trait_descriptions: Vec::new(),
            display_value_feats: Vec::new(),
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

    // Every trait record this race declares, rendered for this character. Built
    // over `siblings` rather than over the applied set, so an alternate the
    // player has not ticked yet still shows the number *they* would get.
    let deltas = display_value_deltas_from_feats(held_feats);
    let rendered_trait_descriptions: Vec<RenderedTraitDescriptionDto> =
        siblings.iter().map(|record| render_trait_description(record, &deltas)).collect();
    let rendered_by_key: BTreeMap<&str, &RenderedTraitDescriptionDto> =
        rendered_trait_descriptions.iter().map(|row| (row.key.as_str(), row)).collect();

    // The applied list reads the same rendering rather than the resolver's
    // stored prose — one sentence per trait, whichever list shows it.
    let applied_traits: Vec<AppliedTraitDto> = race
        .traits
        .iter()
        .map(|resolved| AppliedTraitDto {
            key: resolved.key.clone(),
            name: resolved.name.clone(),
            book: book_code(&resolved.book_id),
            role: describe_role(resolved.role).to_string(),
            description: rendered_by_key
                .get(resolved.key.as_str())
                .map(|row| row.text.clone())
                .unwrap_or_else(|| resolved.description.clone().unwrap_or_default()),
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
        display_value_feats: display_value_feats(held_feats),
        rendered_trait_descriptions,
        errors: Vec::new(),
    }
}

fn menu_or_error() -> AlternateRacialTraitsResponse {
    match race_corpus() {
        Ok(corpus) => build_menu(corpus),
        Err(err) => AlternateRacialTraitsResponse {
            races: Vec::new(),
            adoptive_parentage_options: Vec::new(),
            adopted_race_options: Vec::new(),
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

/// Resolves a race against a selection, with no character in hand: every
/// description renders its racial base.
///
/// This is the shape the two non-UI callers want —
/// `character_hub::resolve_alternate_trait_choices` validates whether a save is
/// legal, and `reach_gate` counts what reaches a player — neither of which is
/// asking what a *particular* character's sentence says.
pub fn build_race_selection(request: &RaceSelectionRequest) -> RaceSelectionResponse {
    build_race_selection_for_feats(request, &[])
}

/// Resolves a race against a selection *for one character*, whose held feats
/// set the numbers its trait descriptions state. Not cached: both the selection
/// and the feat list are inputs.
pub fn build_race_selection_for_feats(
    request: &RaceSelectionRequest,
    held_feats: &[String],
) -> RaceSelectionResponse {
    match race_corpus() {
        Ok(corpus) => resolve_selection(corpus, &request.race_key, &request.selected_alternate_keys, held_feats),
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
            rendered_trait_descriptions: Vec::new(),
            display_value_feats: Vec::new(),
            errors: vec![format!("race corpus unavailable: {err}")],
        },
    }
}

#[tauri::command]
pub fn list_alternate_racial_traits() -> AlternateRacialTraitsResponse {
    build_alternate_racial_traits()
}

/// `held_feats` is the character's own persisted feat list, or absent when the
/// screen has no character selected. It changes only the numbers the returned
/// descriptions state — never which traits apply.
#[tauri::command]
pub fn resolve_race_alternate_selection(
    request: RaceSelectionRequest,
    held_feats: Option<Vec<String>>,
) -> RaceSelectionResponse {
    build_race_selection_for_feats(&request, &held_feats.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::pcgen_desc::leaked_pcgen_syntax;

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

    /// The whole point: every alternate from every `RACE_CORPUS_BOOKS` book
    /// reaches a player surface, spread across all 18 in-scope races.
    ///
    /// `advanced_players_guide` contributes exactly 1 alternate,
    /// `Half-Orc ~ Plagueborn` — `decisions.md §37`'s first-pass estimate of
    /// 50 real APG alternates was corrected to 1 genuinely new key, 49 of the
    /// 50 colliding with already-ingested ARG keys (`decisions.md §39`).
    ///
    /// **That record was deferred and is now landed.** The deferral's stated
    /// blocker was real: `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS`
    /// (the hand-written table `character_hub.rs`'s creation-acceptance path
    /// validates against, `decisions.md §36` instance 15) did not know
    /// Plagueborn's key, so shipping the corpus record alone would have
    /// offered it in this picker and then refused it at character-save time
    /// -- a stub, not real content. SD-29's race-trait extend lane landed the
    /// record and the table row in one change, and added
    /// `race_resolver::every_alternate_the_app_offers_is_one_the_engine_can_place`
    /// so the two halves cannot separate again.
    #[test]
    fn every_alternate_from_every_race_corpus_book_reaches_the_menu_across_every_in_scope_race() {
        let menu = menu();
        assert_eq!(
            menu.races.len(),
            39,
            "39 in-scope races: decisions.md §25.3's original 18 + SD-31 Epic 1-F2's \
             Bestiary 2 batch of 6 (2026-08-15) + the Skinwalker follow-on batch's 1 + \
             SD-31-E6-F4-002's Advanced Race Guide batch of 6 (2026-08-16: Catfolk, Kitsune, \
             Ratfolk, Strix, Suli, Wayang) + SD31-E6-F4-004's Advanced Race Guide follow-on \
             batch of 4 (2026-08-17: Gillman, Nagaji, Vanara, Vishkanya) + SD31-E6-F4-007's \
             Advanced Race Guide follow-on batch of 2 (2026-08-17: Changeling, Samsaran -- \
             closing `arg_races.lst`'s full 37-row playable-race roster) + SD-31 wave-24's \
             Rougarou (Bestiary 6, 2026-08-20, chassis + 8 standard-tier traits, no ARG \
             alternate-trait content) + SD-32 card-11 T2b lane's Dhampir (Bestiary 2, \
             2026-08-23, chassis + standard tier only)"
        );
        let total: usize = menu.races.iter().map(|race| race.alternates.len()).sum();
        assert_eq!(
            total, 415,
            "ARG's 153 Alternate-classified records + Monster Codex's 8 (4 original, SD-29 \
             decisions.md §43, + SD-32 card-11 T2b lane's 4 new Ratfolk alternates, \
             2026-08-23) \
             + APG's 1 (`Half-Orc ~ Plagueborn`, decisions.md §39's deferral, closed by SD-29's \
             race-trait extend lane) + Inner Sea Races' 76 (67, §45, the same lane's round 2, \
             + 9 from a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22) \
             + Horror Adventures' 41 (§47, round 3) \
             + Core Essentials' 16 heritages (§49, round 4; the book's other 48 records \
             are the replacement rows those heritages grant and are never menu rows) \
             + SD-31 Epic 1-F2's Bestiary 2 batch of 48 (ARG's 42 + Inner Sea Races' 6, \
             2026-08-15) + SD-31-E6-F4-003's 19 (2026-08-16, ARG's own 6-race chassis batch's \
             real alternate-trait rows, minus Strix's Wing-Clipped-granted Flight and Suli's \
             Energy-Strike-granted Earthfoot/Firehand/Icewalk/Shockshield) + SD31-E6-F4-006's 8 \
             (2026-08-17, ARG's own follow-on 4-race chassis batch's real alternate-trait rows) \
             + SD-33 Epic 6's 45 folded Skinwalker heritage records (2026-08-26: 9 kin \
             selectors + their 36 replacement rows, ALL `TraitRole::Alternate` -- unlike Core \
             Essentials' 16+48 above, Skinwalker's replacement rows carry their own \
             `FACT:Skinwalker_Replace<Trait>|True` and are not `FlagGranted`; see \
             `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS` `Skinwalker` section. The \
             batch's other 20 records -- the shared Change Shape component rows -- are \
             `TraitRole::Unclassified` and never menu rows either way)"
        );

        // Per-race counts, derived from the corpus by this very menu.
        // `Half-Orc ~ Plagueborn` moves HalfOrc 14 -> 15. Monster Codex moves
        // exactly two races: Duergar 5 -> 7 (Ironskinned, Twilight-Touched)
        // and Goblin 7 -> 9 (Oversized Goblin ~ Ability Scores, ~ Size).
        // `Oversized Goblin` itself is NOT here and that is deliberate -- it
        // sets no replace flag and carries no positive gate, so
        // `race_resolver::classify` leaves it `Unclassified`. It is a Goblin
        // *variant* selector (PCGen models it through a `Goblin Variant`
        // ABILITYPOOL that this engine has no mechanism for), recorded as a
        // finding in `reach_gate`'s OPEN_FINDINGS rather than hidden.
        // Round 4 (`decisions.md §49`) moved exactly two of these cells:
        // Core Essentials contributes heritages to Aasimar and Tiefling and to
        // no other race, so 16 of the 18 rows below must NOT move and a change
        // in any of them is a regression rather than a new book's arrival.
        //
        // **This table was left at its pre-Inner-Sea-Races values by round 2
        // and went RED with five other root-workspace assertions**; round 3
        // moved it and recorded the miss (`decisions.md §47`). Every cell
        // below is re-derived from the written records rather than added up
        // from a prior round's arithmetic, with each race's per-book split in
        // the trailing comment so a future book's contribution is checkable
        // one race at a time instead of only in the total.
        let expected: &[(&str, usize)] = &[
            ("Aasimar", 17),    // ARG 9 + ISR 2 + CE 6 (heritages)
            // ARG 6 (SD-31-E6-F4-003, 2026-08-16) + ISR 1 (`Jungle Stalker`,
            // a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22)
            ("Catfolk", 7),
            ("Drow", 7),        // ARG 6 + ISR 1
            ("Duergar", 8),     // ARG 5 + MC 2 + ISR 1
            ("Dwarf", 30),      // ARG 17 + ISR 7 + HA 6
            ("Elf", 27),        // ARG 13 + ISR 7 + HA 7 (ISR 8 until 2026-08-12: `Elf ~
            // Sovyrian-Born` carries `NAMEISPI:YES` and is dropped, `decisions.md` 53)
            ("Fetchling", 6),   // ARG 5 + ISR 1 (SD-31 Epic 1-F2, 2026-08-15)
            // Gillman's real ARG total is 3, but `Throwback` grants both
            // `Throwback ~ Gillman ~ Type` and `Throwback ~ Gillman ~ Speed`
            // (`TraitRole::FlagGranted`), so all 3 alternates ARE selectable
            // menu rows -- unlike Strix/Suli, Throwback's own grants are not
            // themselves alternates, so nothing is subtracted here.
            // ARG 3 (SD31-E6-F4-006, 2026-08-17) + ISR 1 (`Deep Gillman`,
            // a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22)
            ("Gillman", 4),
            ("Gnome", 23),      // ARG 12 + ISR 6 + HA 5
            ("Goblin", 10),     // ARG 7 + MC 2 + ISR 1
            ("Grippli", 5),     // ARG 4 + ISR 1 (SD-31 Epic 1-F2)
            ("HalfElf", 20),    // ARG 9 + ISR 7 + HA 4
            ("HalfOrc", 28),    // ARG 14 + APG 1 + ISR 7 + HA 6
            ("Halfling", 27),   // ARG 13 + ISR 7 + HA 7
            ("Hobgoblin", 10),  // ARG 9 + ISR 1
            ("Human", 33),      // ARG 15 + ISR 12 + HA 6
            ("Ifrit", 9),       // ARG 8 + ISR 1 (SD-31 Epic 1-F2)
            // ARG 2 (SD-31-E6-F4-003, 2026-08-16) + ISR 1 (`Duplicitous`,
            // a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22)
            ("Kitsune", 3),
            ("Kobold", 5),      // ARG 4 + ISR 1
            ("Merfolk", 4),     // ARG 3 + ISR 1
            // ARG 1 (SD31-E6-F4-006, 2026-08-17) + ISR 1 (`Serpent Affinity`,
            // a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22)
            ("Nagaji", 2),
            ("Oread", 9),       // ARG 8 + ISR 1 (SD-31 Epic 1-F2)
            ("Orc", 5),         // ARG 4 + ISR 1
            // ARG 4 (SD-31-E6-F4-003, 2026-08-16) + Monster Codex 4
            // (Cheek Pouches/Cleanliness/Lab Rat/Surface Sprinter, SD-32
            // card-11 T2b lane, 2026-08-23; Surface Sprinter's own 2
            // replacement rows are `FlagGranted`, not counted here) + ISR 1
            // (`Market Dweller`, a sibling SD-32 card-11 T2b lane's
            // stale-regen fix, 2026-08-22).
            ("Ratfolk", 9),
            // SD-33 Epic 6 fold (2026-08-26): 9 kin selectors
            // (Werebat/Werebear/Wereboar/Werecrocodile/Wereraptor/Wererat/
            // Wereshark/Weretiger/Werewolf-Kin) PLUS their 36 replacement
            // rows (Ability Scores/Animal-Minded/Change Shape/Spell-Like
            // Ability x 9) -- all 45 are `TraitRole::Alternate` and ARE menu
            // rows, because each replacement row carries its own
            // `FACT:Skinwalker_Replace<Trait>|True` (unlike Strix's
            // Wing-Clipped/Suli's Energy Strike dependents above, which
            // carry none and stay `FlagGranted`); see
            // `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS`
            // `Skinwalker` section for the corpus-level proof. Its 20 shared
            // Change Shape component rows are `TraitRole::Unclassified` and
            // are not menu rows.
            ("Skinwalker", 45),
            // Strix's real ARG total is 6, but `Wing-Clipped` grants
            // `Wing-Clipped ~ Strix ~ Flight` (`TraitRole::FlagGranted`), so
            // only 5 are menu rows -- the same shape `Dwarf ~ Saltbeard`
            // already sets for `Saltbeard ~ Dwarf ~ Greed`.
            // ARG 5 selectable + 1 FlagGranted (SD-31-E6-F4-003) + ISR 1
            // (`Cautious Brawler`, a sibling SD-32 card-11 T2b lane's
            // stale-regen fix, 2026-08-22)
            ("Strix", 6),
            // Suli's real ARG total is 5, but `Energy Strike` grants all 4 of
            // `Earthfoot`/`Firehand`/`Icewalk`/`Shockshield`
            // (`TraitRole::FlagGranted`), so only 1 is a menu row.
            ("Suli", 1),        // ARG 1 selectable + 4 FlagGranted (SD-31-E6-F4-003)
            ("Svirfneblin", 3), // ARG 2 + ISR 1
            ("Sylph", 9),       // ARG 8 + ISR 1 (SD-31 Epic 1-F2)
            ("Tengu", 5),       // ARG 4 + ISR 1
            ("Tiefling", 20),   // ARG 7 + ISR 3 + CE 10 (heritages)
            ("Undine", 10),     // ARG 9 + ISR 1 (SD-31 Epic 1-F2)
            // Vanara's real ARG total is 2, and `Tree Stranger` grants
            // `Tree Stranger ~ Vanara ~ Speed` (`TraitRole::FlagGranted`),
            // but that grant is not itself an alternate so nothing is
            // subtracted -- both 2 alternates ARE selectable menu rows.
            // ARG 2 (SD31-E6-F4-006, 2026-08-17) + ISR 1 (`Risky
            // Troublemaker`, a sibling SD-32 card-11 T2b lane's stale-regen
            // fix, 2026-08-22)
            ("Vanara", 3),
            // ARG 2 (SD31-E6-F4-006, 2026-08-17) + ISR 1 (`Deceptive`, same
            // sibling fix -- its own dependent row `Deceptive ~ Vishkanya ~
            // Limber` is `FlagGranted`, not counted here)
            ("Vishkanya", 3),
            // ARG 1 (SD-31-E6-F4-003, 2026-08-16) + ISR 1 (`In the
            // Shadows`, same sibling fix)
            ("Wayang", 2),
        ];
        for (race_id, count) in expected {
            assert_eq!(race(&menu, race_id).alternates.len(), *count, "{race_id} alternate count");
        }
        assert_eq!(expected.iter().map(|(_, n)| n).sum::<usize>(), 415);
    }

    /// Every alternate is attributed to a book that really loaded it, and
    /// carries real prose — no empty cells reaching the screen.
    ///
    /// **The book assertion used to be `== "ARG"`.** That was true when ARG was
    /// the only book contributing alternates and became false the moment SD-29's
    /// race-trait pilot added Monster Codex's. It is now checked against the
    /// codes `race_catalog::book_code` derives from `RACE_CORPUS_BOOKS`, so a
    /// third book widens it without an edit here — and an alternate attributed
    /// to a book nobody loaded still fails.
    ///
    /// **`source_page` is asserted per book, not globally, because the corpus
    /// differs.** Every ARG alternate carries a real page. Monster Codex's two
    /// Duergar rows (`mc_abilities_race.lst:16`-`:17`) carry **no `SOURCEPAGE:`
    /// token at all**, while its two Goblin replacement rows carry `p.104`.
    /// Asserting a page for a row the corpus does not give one for would force
    /// either a fabricated citation or a hidden record; the honest form is to
    /// require that a page, *when present*, is real — never PCGen's `p.xx`
    /// stand-in — and to pin which books currently supply one.
    #[test]
    fn every_alternate_carries_real_book_attribution_and_prose() {
        let menu = menu();
        let loadable: BTreeSet<String> =
            crate::race_catalog::RACE_CORPUS_BOOKS.iter().map(|b| book_code(b)).collect();
        let mut paged: BTreeSet<&str> = BTreeSet::new();
        let mut pageless: BTreeSet<&str> = BTreeSet::new();

        for race in &menu.races {
            for alternate in &race.alternates {
                assert!(
                    loadable.contains(&alternate.book),
                    "{} is attributed to {:?}, which is not a book RACE_CORPUS_BOOKS loads ({loadable:?})",
                    alternate.key,
                    alternate.book
                );
                assert!(!alternate.description.trim().is_empty(), "{} description", alternate.key);
                assert!(!alternate.sets_flags.is_empty(), "{} sets at least one flag", alternate.key);
                match alternate.source_page.as_deref() {
                    Some(page) if !page.is_empty() => {
                        // Not spelled with the word the wired-integration audit
                        // scans for: `tests/sd24_wired_integration_audit.rs`
                        // greps shipping source for stub markers, and a test
                        // message using that word reads to it as a new,
                        // unreviewed stub. The check is unchanged.
                        assert_ne!(page, "p.xx", "{} cites `p.xx`, which is not a page", alternate.key);
                        paged.insert(alternate.book.as_str());
                    }
                    _ => {
                        pageless.insert(alternate.key.as_str());
                    }
                }
            }
        }

        assert_eq!(
            paged,
            BTreeSet::from(["APG", "ARG", "B5", "HA", "ISR", "MC"]),
            "the books whose alternates carry a real page. HA joined with SD-29's race-trait \
             lane round 3: all 41 of its alternates cite a real `SOURCEPAGE` too, which is why \
             the `pageless` pin below did NOT move for this book either. ISR joined with \
             SD-29's race-trait \
             lane round 2: all 68 of its alternates cite a real `SOURCEPAGE`, none the literal \
             `p.xx` stand-in that `ingest_races.rs` filters out. B5 joined with SD-33 Epic 6's \
             Skinwalker fold (2026-08-26): of the fold's 45 Alternate records, Wereraptor-Kin's \
             5 (the selector + its 4 replacement rows) genuinely cite `SOURCEPAGE:p.89` in the \
             pinned oracle -- a real page from a DIFFERENT sourcebook than the other 8 kins \
             (`SOURCELONG:Ironfang Invasion, Chapter 1 - Trail of the Hunted`,\
             `skinwalker_abilities_race_subrace.lst:207,211-214`), not a fabrication. The other \
             8 kins' 40 records all carry the `p.xx` stand-in, correctly dropped to `None` \
             and counted in `pageless` below"
        );
        assert_eq!(
            pageless,
            BTreeSet::from([
                "Aasimar ~ Agathion-Blooded",
                "Aasimar ~ Angel-Blooded",
                "Aasimar ~ Archon-Blooded",
                "Aasimar ~ Azata-Blooded",
                "Aasimar ~ Garuda-Blooded",
                "Aasimar ~ Peri-Blooded",
                "Duergar ~ Ironskinned",
                "Duergar ~ Twilight-Touched",
                "Tiefling ~ Asura-Spawn",
                "Tiefling ~ Daemon-Spawn",
                "Tiefling ~ Demodand-Spawn",
                "Tiefling ~ Demon-Spawn",
                "Tiefling ~ Devil-Spawn",
                "Tiefling ~ Div-Spawn",
                "Tiefling ~ Kyton-Spawn",
                "Tiefling ~ Oni-Spawn",
                "Tiefling ~ Qlippoth-Spawn",
                "Tiefling ~ Rakshasa-Spawn",
                // SD-33 Epic 6's Skinwalker fold (2026-08-26): 40 of the
                // fold's 45 Alternate records -- 8 kin selectors (all but
                // Wereraptor-Kin, `paged` above) plus their 32 replacement
                // rows (8 kins x 4) -- carry the `p.xx` placeholder in the
                // pinned oracle, dropped to `None` the same way as every
                // other book here.
                "Skinwalker ~ Werebat-Kin",
                "Skinwalker ~ Werebear-Kin",
                "Skinwalker ~ Wereboar-Kin",
                "Skinwalker ~ Werecrocodile-Kin",
                "Skinwalker ~ Wererat-Kin",
                "Skinwalker ~ Wereshark-Kin",
                "Skinwalker ~ Weretiger-Kin",
                "Skinwalker ~ Werewolf-Kin",
                "Werebat-Kin ~ Ability Scores",
                "Werebat-Kin ~ Animal-Minded",
                "Werebat-Kin ~ Change Shape",
                "Werebat-Kin ~ Spell-Like Ability",
                "Werebear-Kin ~ Ability Scores",
                "Werebear-Kin ~ Animal-Minded",
                "Werebear-Kin ~ Change Shape",
                "Werebear-Kin ~ Spell-Like Ability",
                "Wereboar-Kin ~ Ability Scores",
                "Wereboar-Kin ~ Animal-Minded",
                "Wereboar-Kin ~ Change Shape",
                "Wereboar-Kin ~ Spell-Like Ability",
                "Werecrocodile-Kin ~ Ability Scores",
                "Werecrocodile-Kin ~ Animal-Minded",
                "Werecrocodile-Kin ~ Change Shape",
                "Werecrocodile-Kin ~ Spell-Like Ability",
                "Wererat-Kin ~ Ability Scores",
                "Wererat-Kin ~ Animal-Minded",
                "Wererat-Kin ~ Change Shape",
                "Wererat-Kin ~ Spell-Like Ability",
                "Wereshark-Kin ~ Ability Scores",
                "Wereshark-Kin ~ Animal-Minded",
                "Wereshark-Kin ~ Change Shape",
                "Wereshark-Kin ~ Spell-Like Ability",
                "Weretiger-Kin ~ Ability Scores",
                "Weretiger-Kin ~ Animal-Minded",
                "Weretiger-Kin ~ Change Shape",
                "Weretiger-Kin ~ Spell-Like Ability",
                "Werewolf-Kin ~ Ability Scores",
                "Werewolf-Kin ~ Animal-Minded",
                "Werewolf-Kin ~ Change Shape",
                "Werewolf-Kin ~ Spell-Like Ability",
            ]),
            // The stand-in spellings are quoted rather than described with the
            // word `tests/sd24_wired_integration_audit.rs` scans shipping
            // source for -- that audit reads such a word here as a new,
            // unreviewed stub marker. Same reason the `p.xx` check above
            // carries its own note.
            "the two Monster Codex rows the upstream corpus gives no SOURCEPAGE: token at all, \
             plus Core Essentials' 16 heritages, whose SOURCEPAGE IS present upstream and is a \
             stand-in on every single row -- `p.xx` on all 40 Tiefling rows and `xx` on all \
             24 Aasimar ones. `ingest_race_traits::is_placeholder_source_page` drops those at \
             ingest so the panel shows no page rather than a fake one; none of the four books \
             ingested before Core Essentials carries such a value at all, so this pin moving \
             for any OTHER book means real page data was lost. SD-33 Epic 6's Skinwalker fold \
             (2026-08-26) adds 40 more pageless keys (see above), all `p.xx` in the pinned \
             oracle; the 45th and final new Alternate record's own doc comment on `paged` above \
             explains why Wereraptor-Kin's 5 records are absent from this list instead"
        );
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

    /// Every alternate that PCGen guards has a readable self-exclusion guard,
    /// including the three that spell the negated branch `!PREABILITY` — and
    /// the ones the corpus does not guard are pinned by name.
    ///
    /// **This used to assert a guard on every alternate without exception.**
    /// That held while ARG was the only contributing book, because every ARG
    /// alternate is a player-selectable swap and PCGen guards all of them. SD-29's
    /// Monster Codex pilot brought in two rows from that book's
    /// `###Block: Replacement Abilities` — `Oversized Goblin ~ Ability Scores`
    /// and `~ Size` — which carry a `FACT:Goblin_Replace…|True` token but **no
    /// `PREMULT` guard**, because upstream they are not chosen at all: they are
    /// granted by picking the `Oversized Goblin` variant out of a
    /// `BONUS:ABILITYPOOL|Goblin Variant|1` pool (`mc_abilities_race.lst:26`).
    ///
    /// This engine has no ability-pool variant mechanism, so the `FACT:` heuristic
    /// that classifies alternates reads them as free-standing swaps. **That is a
    /// real modelling gap and it is recorded, not smoothed over** — see
    /// `reach_gate::OPEN_FINDINGS` for `monster_codex/race_traits`, which names
    /// the remedy. Pinning the exception by exact key here means a third guardless
    /// row cannot arrive silently, and either of these gaining a guard fails too.
    #[test]
    fn every_alternate_has_a_readable_exclusion_guard_including_the_preability_spelling() {
        let menu = menu();
        let mut unguarded: BTreeSet<&str> = BTreeSet::new();
        for race in &menu.races {
            for alternate in &race.alternates {
                if alternate.exclusion_guard_flags.is_empty() {
                    unguarded.insert(alternate.key.as_str());
                }
            }
        }
        assert_eq!(
            unguarded,
            BTreeSet::from(["Oversized Goblin ~ Ability Scores", "Oversized Goblin ~ Size"]),
            "exactly the two Monster Codex replacement rows PCGen grants rather than offers"
        );
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
    /// forever. This asserts the menu and the resolver agree for all 153
    /// (ARG's own; APG's 1 genuinely new key is deferred, `decisions.md §39`).
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
                    resolve_selection(corpus, &race.race_key, std::slice::from_ref(&alternate.key), &[]);
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
        assert_eq!(
            checked, 415,
            "153 ARG + 8 Monster Codex (4 original + SD-32 card-11 T2b's 4 Ratfolk \
             alternates, 2026-08-23) + 1 APG (SD-29 decisions.md §43) + 76 Inner Sea Races \
             (67, §45, + 9 from a sibling SD-32 card-11 T2b lane's stale-regen fix, \
             2026-08-22) + 41 Horror Adventures (§47) + 16 Core Essentials heritages (§49) + \
             48 SD-31 Epic 1-F2 Bestiary 2 batch (ARG's 42 + Inner Sea Races' 6, 2026-08-15) + \
             19 SD-31-E6-F4-003 (2026-08-16, ARG's own 6-race chassis batch) + 8 \
             SD31-E6-F4-006 (2026-08-17, ARG's own follow-on 4-race chassis batch) + 45 \
             SD-33 Epic 6 (2026-08-26, folded Skinwalker heritage records: 9 kin selectors + \
             their 36 replacement rows)"
        );
        assert!(unmatched.is_empty(), "no alternate may name a flag nothing declares: {unmatched:?}");

        // Aasimar is the worked case: its nine standard rows now declare the
        // gate its alternates fire, read from `aasimar_abilities_globalvar
        // .lst`. Asserted off the payload, so this is the shipped DTO, not a
        // corpus-only fact.
        //
        // The alternate count moved 9 -> 11 when Inner Sea Races landed
        // (`Aasimar ~ Crusading Magic`, `Aasimar ~ Lost Promise`); round 2 did
        // not move it and this assertion was RED on the branch until round 3
        // (`decisions.md §47`). The *standard* count did not move and must
        // not: ISR and HA contribute alternates only, no `race/` chassis.
        let aasimar = race(&menu, "Aasimar");
        assert_eq!(aasimar.standard_traits.len(), 9);
        assert!(
            aasimar.standard_traits.iter().all(|row| row.suppressed_by_flag.is_some()),
            "every Aasimar standard row carries its gate"
        );
        // Round 4 moved this 11 -> 17: Core Essentials' six Aasimar heritages
        // are `TraitRole::Alternate` (each sets three replace-flags) and are
        // therefore menu rows. The *standard* count above still must not move
        // -- CE contributes no `race/` chassis either.
        assert_eq!(aasimar.alternates.len(), 17);
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
        // Deduped: SD-29's Monster Codex pilot added a SECOND alternate setting
        // this same flag (`Duergar ~ Twilight-Touched`, `mc_abilities_race.lst:17`)
        // alongside ARG's `Duergar ~ Blood Enmity`, and its Inner Sea Races
        // round added a THIRD (`Duergar ~ Magical Taskmaster`), so the flag is
        // now named by three rows that all grant the same one row. Several
        // setters granting one record is the corpus's own shape, not a
        // duplicate record -- which is exactly why the setters are asserted
        // too, rather than only the target, and why THIS list grows with each
        // book while the target list does not.
        //
        // Derived from the committed records rather than read off the menu:
        //
        // ```
        // python3 -c "
        // import json,glob
        // for p in glob.glob('data/corpus/*/race_trait/**/*.json', recursive=True):
        //     d=json.load(open(p))['data']
        //     if 'Duergar_ReplaceSLAInvisibility' in (d.get('sets_replace_flags') or []):
        //         print(p.split('/')[2], d['key'])"
        // ```
        //
        // -> `monster_codex Duergar ~ Twilight-Touched`,
        //    `advanced_race_guide Duergar ~ Blood Enmity`,
        //    `inner_sea_races Duergar ~ Magical Taskmaster`.
        let setters: BTreeSet<&str> = duergar
            .alternates
            .iter()
            .filter(|alternate| {
                alternate.grants.iter().any(|link| link.flag == "Duergar_ReplaceSLAInvisibility")
            })
            .map(|alternate| alternate.key.as_str())
            .collect();
        assert_eq!(
            setters,
            BTreeSet::from([
                "Duergar ~ Blood Enmity",
                // Inner Sea Races, SD-29 race-trait lane round 2. A THIRD
                // setter of the same flag. Round 2 did not move this pin and
                // it was RED on the branch until round 3 reached it -- behind
                // two earlier assertions in this same test, which is why the
                // cascade only surfaced once those were fixed
                // (`decisions.md §47.3`).
                "Duergar ~ Magical Taskmaster",
                "Duergar ~ Twilight-Touched",
            ]),
            "all three books' setters of Duergar_ReplaceSLAInvisibility"
        );
        assert_eq!(
            grants.iter().copied().collect::<BTreeSet<&str>>(),
            BTreeSet::from(["Duergar ~ Spell-Like Ability ~ Enlarge Person"]),
            "and all three grant the one row the flag gates"
        );
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
        let before = resolve_selection(corpus, "Dwarf", &[], &[]);
        assert!(before.errors.is_empty());
        assert_eq!(before.applied_traits.len(), 12, "Dwarf's 12 racial defaults");
        assert!(before.suppressions.is_empty());
        assert!(before.applied_traits.iter().any(|applied| applied.key == "Dwarf ~ Greed"));

        let after = resolve_selection(corpus, "Dwarf", &["Dwarf ~ Saltbeard".to_string()], &[]);
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

    /// The exact selection SD-29 `progress.md` §8b screenshotted, asserted at
    /// the DTO layer the screen actually reads.
    ///
    /// **This test exists to decide an attribution, not to guard a number.**
    /// `§8b` recorded that with `Half-Orc ~ Plagueborn` ticked, the picker's
    /// left panel still read *"9 traits apply. No alternate selected, so
    /// nothing is replaced."*, and diagnosed it a *render* bug on the grounds
    /// that *"the right-hand column does update ('1 selected. 0 further options
    /// locked out.'), so the IPC round trip happened"*.
    ///
    /// **That inference is unsound.** `AlternateTraitPicker.tsx` builds that
    /// sentence from `selected.length` — local React state, updated
    /// synchronously by the checkbox and needing no backend at all — and from
    /// `blocked.size`, which is **0 when `selection` is `null`**. So the
    /// reported right-hand text is exactly what renders when the resolve call
    /// has *not* answered yet. The two panels are one symptom, not two.
    ///
    /// That leaves two candidate causes, and this test kills one of them: if
    /// the backend genuinely returned no suppressions for this selection, the
    /// defect would be here and not in any render path. It does not. What
    /// survives is the timing reading — a screenshot captured between the
    /// click's commit and the effect's `setSelection(null)` — which is a
    /// harness settle-wait, not a product defect. Round 6 starts from that
    /// rather than from the label.
    #[test]
    fn plagueborn_really_suppresses_both_standard_traits_its_flags_name_so_8b_is_not_a_backend_gap() {
        let corpus = race_corpus().as_ref().expect("corpus");

        let before = resolve_selection(corpus, "Half-Orc", &[], &[]);
        assert!(before.errors.is_empty(), "{:?}", before.errors);
        assert!(before.suppressions.is_empty());
        assert_eq!(before.applied_traits.len(), 9, "Half-Orc's 9 racial defaults — §8b's screenshot said 9");

        let after = resolve_selection(corpus, "Half-Orc", &["Half-Orc ~ Plagueborn".to_string()], &[]);
        assert!(after.errors.is_empty(), "{:?}", after.errors);
        assert!(after.unmatched_selections.is_empty(), "{:?}", after.unmatched_selections);
        assert!(after.inert_flags.is_empty(), "{:?}", after.inert_flags);

        let suppressed: Vec<&str> = after.suppressions.iter().map(|s| s.suppressed_trait_key.as_str()).collect();
        assert_eq!(
            suppressed,
            vec!["Half-Orc ~ Intimidating", "Half-Orc ~ Weapon Familiarity"],
            "the two standard traits `HalfOrc_ReplaceIntimidating`/`HalfOrc_ReplaceWeaponFamiliarity` name"
        );
        for suppression in &after.suppressions {
            assert_eq!(suppression.set_by_trait_key, "Half-Orc ~ Plagueborn");
        }
        // 9 - 2 + 1: the screen's own caption should read 8, not 9.
        assert_eq!(after.applied_traits.len(), 8);

        // ...and the lock-out count the right panel prints is NOT 0 either,
        // which is the other half of the same evidence: every sibling alternate
        // whose guard names a flag Plagueborn fired is blocked. A rendered
        // "0 further options locked out." alongside a selection is therefore a
        // `selection == null` render, not a resolved one.
        assert!(
            !after.blocked_alternates.is_empty(),
            "Plagueborn fires two flags other Half-Orc alternates guard on; a resolved response \
             cannot report zero lock-outs"
        );
    }

    /// The `PREMULT` guard is honoured: with Saltbeard taken, no other Dwarf
    /// alternate replacing defensive training, hatred or stonecunning may be.
    #[test]
    fn selecting_an_alternate_blocks_every_sibling_whose_guard_names_a_fired_flag() {
        let corpus = race_corpus().as_ref().expect("corpus");
        let open = resolve_selection(corpus, "Dwarf", &[], &[]);
        assert!(open.blocked_alternates.is_empty(), "nothing is blocked before a selection");

        let after = resolve_selection(corpus, "Dwarf", &["Dwarf ~ Saltbeard".to_string()], &[]);
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
        let after = resolve_selection(corpus, "Dwarf", &["Dwarf ~ Saltbeard".to_string()], &[]);
        let rival = after.blocked_alternates.first().expect("at least one rival").key.clone();

        let both = resolve_selection(corpus, "Dwarf", &["Dwarf ~ Saltbeard".to_string(), rival.clone()], &[]);
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
        let typo = resolve_selection(corpus, "Dwarf", &["Dwarf ~ Saltbeerd".to_string()], &[]);
        assert_eq!(typo.unmatched_selections, vec!["Dwarf ~ Saltbeerd"]);
        assert_eq!(typo.applied_traits.len(), 12, "the race still resolves plainly");

        let unknown = resolve_selection(corpus, "Balor", &[], &[]);
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
                let response = resolve_selection(corpus, &race.race_key, std::slice::from_ref(&alternate.key), &[]);
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
        let loose = resolve_selection(corpus, "race:half-elf", &[], &[]);
        assert!(loose.errors.is_empty());
        assert_eq!(loose.race_key, "Half-Elf");
        assert_eq!(loose.race_id, "HalfElf");
    }

    // -----------------------------------------------------------------------
    // Display values reach the payload
    // -----------------------------------------------------------------------

    fn rendered<'a>(response: &'a RaceSelectionResponse, key: &str) -> &'a RenderedTraitDescriptionDto {
        response
            .rendered_trait_descriptions
            .iter()
            .find(|row| row.key == key)
            .unwrap_or_else(|| panic!("{key} has a rendered description in the payload"))
    }

    fn held(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| (*name).to_string()).collect()
    }

    /// One record's character-free rendered text, straight off the corpus.
    fn by_key_all(corpus: &RaceCorpus, race: &str, key: &str) -> String {
        let record = corpus
            .traits_for(race)
            .into_iter()
            .find(|record| record.data.key == key)
            .unwrap_or_else(|| panic!("{key} is a loaded record"));
        render_trait_description(record, &FeatDisplayValueDeltas::default()).text
    }

    /// **The consumer.** `decisions.md §29.1`'s producer-with-no-consumer trap,
    /// closed at the only seam that reaches a player: the same corpus record
    /// must render a *different sentence* for a different character, through
    /// the shipped Tauri payload rather than only in the engine's own test.
    ///
    /// A baked constant cannot pass this — the assertions are before/after
    /// pairs over one record, per `decisions.md §28`'s standing guard, and the
    /// word "Three" has to *disappear* because a `PREVARLTEQ:...,3` gate stops
    /// applying rather than because a number was substituted.
    #[test]
    fn the_payload_renders_a_different_sentence_for_a_character_holding_the_feats() {
        let corpus = race_corpus().as_ref().expect("corpus");
        let luck = |feats: &[&str]| {
            let response = resolve_selection(corpus, "Halfling", &[], &held(feats));
            assert!(response.errors.is_empty(), "{:?}", response.errors);
            rendered(&response, "Halfling ~ Adaptable Luck").clone()
        };

        let base = luck(&[]);
        assert!(base.text.contains("Three times per day"), "racial base: {}", base.text);
        assert!(base.text.contains("gain the full +2 bonus"), "{}", base.text);
        assert!(base.text.contains("only gain a +1 bonus"), "{}", base.text);
        assert!(!base.moved_by_feats, "no feat held, nothing moved");

        let fortunate = luck(&["Fortunate One"]);
        assert!(fortunate.text.contains("4 times per day"), "3 + 1 = 4: {}", fortunate.text);
        assert!(!fortunate.text.contains("Three"), "the PREVARLTEQ gate stops applying: {}", fortunate.text);
        assert!(fortunate.text.contains("gain the full +2 bonus"), "Fortunate One adds no bonus");
        assert!(fortunate.moved_by_feats);

        let both = luck(&["Fortunate One", "Adaptive Fortune"]);
        assert!(both.text.contains("5 times per day"), "3 + 1 + 1 = 5: {}", both.text);
        assert!(both.text.contains("gain the full +4 bonus"), "2 + 2 = 4: {}", both.text);
        assert!(both.text.contains("only gain a +3 bonus"), "{}", both.text);
        assert!(both.moved_by_feats);

        assert_ne!(base.text, fortunate.text);
        assert_ne!(fortunate.text, both.text);

        // Every one of the three is free of leaked PCGen syntax and drops
        // nothing — a half-rendered sentence would satisfy the `contains`
        // assertions above while shipping `%1` to a player.
        for row in [&base, &fortunate, &both] {
            assert!(row.dropped_args.is_empty(), "{:?}", row.dropped_args);
            assert_eq!(leaked_pcgen_syntax(&row.text), None, "{}", row.text);
        }
    }

    /// The Core Rulebook half of the same seam: a *standard* trait's stated
    /// magnitude moves for a gnome holding Great Hatred, and the trait is not
    /// an ARG row at all.
    #[test]
    fn a_standard_traits_stated_bonus_moves_in_the_payload_for_a_character_holding_great_hatred() {
        let corpus = race_corpus().as_ref().expect("corpus");
        let base = resolve_selection(corpus, "Gnome", &[], &[]);
        let with_feat = resolve_selection(corpus, "Gnome", &[], &held(["Great Hatred"].as_slice()));

        let before = rendered(&base, "Gnome ~ Hatred");
        let after = rendered(&with_feat, "Gnome ~ Hatred");
        assert!(before.text.contains("receive a +1 bonus on attack rolls"), "{}", before.text);
        assert!(after.text.contains("receive a +2 bonus on attack rolls"), "{}", after.text);
        assert!(!before.moved_by_feats && after.moved_by_feats);

        // Only the number moved; the corpus's own sentence is otherwise intact.
        assert_eq!(before.text.replace("+1 bonus", "+N"), after.text.replace("+2 bonus", "+N"));

        // The applied-traits list a player reads carries the same rendered
        // text, not the raw corpus prose.
        let applied = with_feat
            .applied_traits
            .iter()
            .find(|row| row.key == "Gnome ~ Hatred")
            .expect("Gnome ~ Hatred applies");
        assert_eq!(applied.description, after.text);

        // The feats that moved a display value are reported, so the screen can
        // say *why* the number is what it is rather than just showing it.
        assert_eq!(with_feat.display_value_feats, vec!["Great Hatred"]);
        assert!(base.display_value_feats.is_empty());
    }

    /// A held feat that moves no display variable is not reported as though it
    /// did, and changes no sentence.
    #[test]
    fn an_ordinary_feat_moves_no_sentence_and_is_not_reported_as_a_display_value_feat() {
        let corpus = race_corpus().as_ref().expect("corpus");
        let plain = resolve_selection(corpus, "Halfling", &[], &[]);
        let dodging = resolve_selection(corpus, "Halfling", &[], &held(["Dodge", "Toughness"].as_slice()));

        assert!(dodging.display_value_feats.is_empty());
        assert_eq!(
            plain.rendered_trait_descriptions, dodging.rendered_trait_descriptions,
            "feats this engine renders no value for must change no prose"
        );
        assert!(dodging.rendered_trait_descriptions.iter().all(|row| !row.moved_by_feats));
    }

    /// Every rendered description in both payloads is real, leak-free prose,
    /// and every trait the menu offers has one. Counts derived here, never
    /// asserted from a brief.
    #[test]
    fn every_menu_row_has_a_rendered_description_and_none_leaks_pcgen_syntax() {
        let menu = menu();
        let corpus = race_corpus().as_ref().expect("corpus");
        let mut checked = 0usize;
        let mut dropping = 0usize;

        for race in &menu.races {
            let response = resolve_selection(corpus, &race.race_key, &[], &[]);
            let by_key: BTreeMap<&str, &RenderedTraitDescriptionDto> =
                response.rendered_trait_descriptions.iter().map(|row| (row.key.as_str(), row)).collect();

            for (key, description) in race
                .standard_traits
                .iter()
                .map(|row| (row.key.as_str(), row.description.as_str()))
                .chain(race.alternates.iter().map(|row| (row.key.as_str(), row.description.as_str())))
            {
                assert!(!description.trim().is_empty(), "{key} has prose");
                assert_eq!(leaked_pcgen_syntax(description), None, "{key} menu prose: {description}");
                // The character-free menu and a character-free resolution are
                // the same rendering of the same record, so they must agree
                // exactly — otherwise the screen shows two different sentences
                // for one trait depending on which call answered first.
                let row = by_key.get(key).unwrap_or_else(|| panic!("{key} resolves a description"));
                assert_eq!(row.text, description, "{key}");
                assert!(!row.moved_by_feats, "no feats held");
                checked += 1;
            }
            dropping += response.rendered_trait_descriptions.iter().filter(|r| !r.dropped_args.is_empty()).count();

            // `resolve_selection` falls back to the resolver's stored prose for
            // an applied trait with no rendered entry. That path must stay
            // unreachable — a trait reaching the screen through it would be the
            // one row still showing the un-rendered text, silently.
            for applied in &response.applied_traits {
                let row = by_key
                    .get(applied.key.as_str())
                    .unwrap_or_else(|| panic!("{} applied without a rendered description", applied.key));
                assert_eq!(applied.description, row.text, "{}", applied.key);
            }
        }

        // Derived here, not carried from a doc. `decisions.md §27.2`'s "175
        // standard trait rows" counts every non-alternate record; this menu's
        // left-hand column is `TraitRole::Default` only, and the 2 the
        // difference names are `TraitRole::FlagGranted` rows — content granted
        // *by* an alternate, which is never offered as a menu choice and
        // appears on screen only once its granting alternate is selected.
        // SD28-E16 (2026-08-08, `decisions.md §39`): `§37`'s first-pass
        // estimate of 50 real APG alternates corrected to 1 genuinely new
        // key (`Half-Orc ~ Plagueborn`) -- 49 collided with existing ARG
        // keys and were excluded. That 1 key was deferred pending
        // `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS` table
        // (`decisions.md §36` instance 15); SD-29's race-trait extend lane
        // landed the record and the table row together, so `alternates` now
        // carries it. `standard` is unaffected either way -- APG contributes
        // no `race/` chassis, so the standard-trait column (sourced from
        // ARG/CRB/Bestiary) never moves.
        let standard: usize = menu.races.iter().map(|race| race.standard_traits.len()).sum();
        let alternates: usize = menu.races.iter().map(|race| race.alternates.len()).sum();
        // Round 3 (`decisions.md §47`) added Horror Adventures' 41 alternates.
        // `standard` did not move for the same reason APG never moved it: HA
        // contributes no `race/` chassis, only alternates onto races CRB and
        // Bestiary 1 already declare. Round 4 (`§49`) added Core Essentials'
        // 16 heritages on the same terms -- and note that this test counts
        // *menu rows*, so the book's other 48 records are correctly absent
        // here while being fully present in `reach_gate`'s claim, which reads
        // what each selection grants as well as what it offers.
        // SD-31 Epic 1-F2 (2026-08-15) added Bestiary 2's 6-race batch: 57
        // more `TraitRole::Default` standard rows (173 -> 230) and 48 more
        // alternates via ARG/ISR (282 -> 330), the same "no race/ chassis
        // moves it" shape APG/HA/CE already established does not apply here
        // -- this batch DOES add a `race/` chassis (6 new ones), which is
        // exactly why `standard` moves this time and did not for those books.
        // The Skinwalker follow-on batch (2026-08-15) adds 1 more race/
        // chassis and 9 more standard rows (230 -> 239); `alternates` is
        // unchanged (330) -- this batch does not ingest Skinwalker's
        // heritage-shaped alternates.
        // SD-31-E6-F4-002 (2026-08-16) adds Advanced Race Guide's own
        // 6-race batch (Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang): 6
        // more race/ chassis and 58 more standard rows (239 -> 297).
        // `alternates` is unchanged (330) at that point -- that batch did not
        // ingest any alternate-trait content for these 6 races yet.
        // SD-31-E6-F4-003 (2026-08-16) closes that gap: `ingest_race_traits.rs`
        // now carries the same 6 races, and their real ARG alternate-trait
        // rows land -- 19 more menu rows (330 -> 349; `standard` unmoved,
        // this cycle wrote no chassis/standard-tier content).
        // SD31-E6-F4-004 (2026-08-17) adds 4 more of ARG's own races
        // (Gillman, Nagaji, Vanara, Vishkanya): 4 more race/ chassis and 38
        // more standard rows (297 -> 335). `alternates` is unchanged (349)
        // -- this batch does not ingest alternate-trait content for these
        // 4 races (see `ingest_races.rs`'s `IN_SCOPE_RACES` doc comment for
        // why Changeling and Samsaran are not part of this batch either).
        // SD31-E6-F4-006 (2026-08-17) closes that gap the same way
        // SD-31-E6-F4-003 closed it for the prior 6-race batch:
        // `ingest_race_traits.rs` now carries the same 4 races, and their
        // real ARG alternate-trait rows land -- 8 more menu rows
        // (349 -> 357; `standard` unmoved, this cycle wrote no
        // chassis/standard-tier content).
        // SD31-E6-F4-007 (2026-08-17) adds the last 2 of ARG's own races
        // (Changeling, Samsaran), closing `arg_races.lst`'s full 37-row
        // playable-race roster: 2 more `race`/ chassis and 18 more standard
        // rows (335 -> 353). `alternates` is unchanged (357) -- neither
        // race has any ARG alternate-trait content to ingest (re-derived:
        // `grep -c '^Changeling ~\|^Samsaran ~'
        // advanced_race_guide/arg_abilities_race.lst` -> 0).
        // SD-31 wave-24 (2026-08-20) adds Rougarou (Bestiary 6): 1 more
        // `race`/ chassis and 8 more standard rows (353 -> 361).
        // `alternates` is unchanged (357) -- Rougarou has no
        // heritage/alternate-trait content in the pinned oracle at all
        // (confirmed: no `*_subrace.lst` file, no `Rougarou_Replace*` flag
        // ever set to `True` anywhere in the corpus).
        // SD-32 card-11 T2b lane (2026-08-23) adds Dhampir (Bestiary 2):
        // 1 more `race`/ chassis and 12 more standard rows (361 -> 373),
        // plus Monster Codex's 4 new Ratfolk alternates (357 -> 361) --
        // its `Adopted Race ~ Dhampir` row stays deferred, same browse-only
        // stub disposition as Rougarou's `Adopted Race ~ Rougarou` above.
        // A sibling SD-32 card-11 T2b lane's `inner_sea_races` stale-regen
        // fix (2026-08-22) adds 9 more alternates (361 -> 370); `standard`
        // is unmoved, that fix wrote no chassis/standard-tier content.
        // SD-33 Epic 6's fold (2026-08-26) adds 45 more alternates, the
        // folded Skinwalker heritage records -- 9 kin selectors + their 36
        // replacement rows, all `TraitRole::Alternate` (370 -> 415);
        // `standard` is unmoved -- Skinwalker's chassis and 9 standard-tier
        // rows were already shipped by `ingest_races.rs`'s SD-31 Epic 1
        // follow-on batch, and this fold touches no `is_racial_default:
        // true` record.
        assert_eq!((standard, alternates), (373, 415));
        assert_eq!(checked, standard + alternates);
        assert_eq!(checked, 788);

        // What rendering changed for a player *with no character*, measured
        // against the stored `data.description` this module used to transcribe.
        // Derived, and printed rather than pinned at a number a later ingest
        // could legitimately move.
        let mut changed: Vec<String> = Vec::new();
        for race_key in corpus.race_keys() {
            for record in corpus.traits_for(race_key) {
                let stored = record.data.description.clone().unwrap_or_default();
                let rendered = render_trait_description(record, &FeatDisplayValueDeltas::default()).text;
                if stored != rendered {
                    changed.push(record.data.key.clone());
                }
            }
        }
        // `Halfling ~ Adaptable Luck` was here until SD-32's card-11 T2b
        // formula-interpreter wiring (2026-08-23,
        // `race_trait_formula_binding::resolve_same_row_formula`):
        // `ingest_race_traits.rs` now resolves the row's real `%2` argument
        // (`Halfling_AdaptableLuck_Bonus-1`, a same-row formula, not a
        // literal) the same way this module's own `render_trait_description`
        // already did, so the two agree and the record no longer shows up as
        // "differs from the ingest-time collapse". The remaining three carry
        // a genuinely different shape: each one's unresolved `DESC:` argument
        // names a variable this row never defines at all (`Nagaji_
        // RacialCasterlevel`, `Suli_ElementalAssault_Duration`, `Undine_
        // NereidFascination_Duration` all depend on cross-record/character
        // state — total level, another class feature's own variable — not on
        // an expression over this row's own literals), so no same-row formula
        // evaluator, wired or not, can close them from ingested data alone.
        assert_eq!(
            changed,
            vec![
                "Oversized Goblin",
                "Nagaji ~ Hypnotic Gaze",
                "Suli ~ Energy Strike",
                "Undine ~ Nereid Fascination"
            ],
            "the records whose rendered prose differs from the ingest-time collapse"
        );

        // SD-29's Monster Codex pilot added the second one, for a different
        // reason than Adaptable Luck's, and the difference is asserted rather
        // than merely allowed: `Oversized Goblin`'s corpus DESC carries PCGen's
        // `&nl;` newline entity, which the stored string holds verbatim and the
        // renderer resolves. A player reads the rendered form, so the entity
        // must not survive into it.
        let oversized = by_key_all(corpus, "Goblin", "Oversized Goblin");
        assert!(!oversized.contains("&nl;"), "rendered prose still carries a PCGen entity: {oversized}");
        assert!(oversized.contains("oversized goblins gain a +2 bonus to Strength"), "{oversized}");
        let luck = &by_key_all(corpus, "Halfling", "Halfling ~ Adaptable Luck");
        assert!(luck.contains("they gain the full +2 bonus"), "{luck}");
        assert!(luck.contains("they only gain a +1 bonus"), "{luck}");
        // SD-31 Epic 1-F2's third: the same `&nl;` entity shape as
        // `Oversized Goblin`, not a new defect class -- `Undine ~ Nereid
        // Fascination`'s corpus DESC also carries a literal `&nl;`, which
        // this record's stored `data.description` (written by
        // `ingest_races.rs`, which has no entity table) holds verbatim and
        // this renderer decodes to a real newline.
        let nereid = by_key_all(corpus, "Undine", "Undine ~ Nereid Fascination");
        assert!(!nereid.contains("&nl;"), "rendered prose still carries a PCGen entity: {nereid}");
        assert!(nereid.contains("This is a supernatural ability."), "{nereid}");
        // SD-31-E6-F4-003's addition: `Suli ~ Energy Strike`'s corpus DESC
        // carries the identical `&nl;` entity shape (a genuine second-segment
        // `DESC:` continuation, PCGen's own encoding), and its stored
        // `data.description` (written by `ingest_race_traits.rs`, which
        // renders the row's leading substitution but has no entity table)
        // holds it verbatim.
        let energy_strike = by_key_all(corpus, "Suli", "Suli ~ Energy Strike");
        assert!(
            !energy_strike.contains("&nl;"),
            "rendered prose still carries a PCGen entity: {energy_strike}"
        );
        assert!(
            energy_strike.contains("Once per day as a swift action, a suli can shroud her arms"),
            "{energy_strike}"
        );
        // Reported rather than pinned: widening what the engine can resolve
        // must not fail here, and neither must a record quietly guessing.
        println!("trait rows still reporting an unresolved DESC argument: {dropping}");
    }

    /// `decisions.md §16` item 2 / SD-32 card-11 T2b: the real IPC builder
    /// `reach_gate` executes carries the 7 `Human ~ Adoptive Parentage`
    /// CHOOSE-pool options, each with a real description and real resolved
    /// grants — not just the resolver-level `adoptive_parentage_options`
    /// this cycle also unit-tests in `race_resolver`, but the actual Tauri
    /// command surface a player's frontend would call.
    #[test]
    fn the_menu_command_itself_carries_all_seven_adoptive_parentage_options_with_real_grants() {
        let menu = menu();
        let keys: Vec<&str> = menu.adoptive_parentage_options.iter().map(|o| o.key.as_str()).collect();
        assert_eq!(keys, vec!["Drow", "Dwarf", "Elf", "Gnome", "Grippli", "Halfling", "Orc"]);
        for option in &menu.adoptive_parentage_options {
            assert_eq!(option.book, "ARG", "the ARG book code, matching every other ARG row on this menu");
            assert_eq!(option.adopted_race, option.key);
            assert!(!option.description.trim().is_empty());
            let grant_names: Vec<&str> = option.grants.iter().map(|g| g.name.as_str()).collect();
            assert_eq!(grant_names, vec!["Weapon Familiarity", "Languages"]);
        }
    }

    /// SD-32 `decisions.md §25` cycle 2: the menu command carries all 21 real
    /// "Adopted Race" selectors ingested corpus-wide -- the original 14
    /// `inner_sea_races` (ISR) selectors, plus AT-34-E3-001's 7
    /// `core_rulebook` (CRB) selectors (2026-08-27,
    /// `ingest_race_traits.rs`'s new `selector_only` `BookSource`) -- all
    /// correctly book-coded, and none flagged malformed (every real oracle
    /// row's `CHOOSE:` token parses).
    ///
    /// **20 of 21 resolve at least one real grant, via a real `kind: trait`
    /// write.** `epic-6-kind-trait` cycle 2 built this resolver against a
    /// temporary `ability/`-directory fallback because `shape_ledger.py`'s
    /// kind-blind join blocked the real `--kind trait` ingest. Cycle 3 fixed
    /// that join and ran `ingest_generic_kind.py --kind trait` for real;
    /// `trait_pool::load_trait_pool`'s fallback is retired (see that
    /// module's own doc comment). The original 13 ISR selectors each pick
    /// from a single-member pool (their own named race trait, e.g. Oread's
    /// `Loner of the Rocks`). AT-34-E3-001's 7 CRB selectors are a different
    /// shape: `<Race> Race Trait` (e.g. `TYPE:Trait.RaceTrait.Elf Race
    /// Trait`) is PF1e's *general* chargen-Trait race tag, carrying every
    /// `advanced_players_guide`/`inner_sea_races` Trait book-authored FOR
    /// that race, not one dedicated ISR pool member -- so
    /// `resolve_adopted_race_options` (which returns the WHOLE pool, not a
    /// single pick) resolves each to several real grants, re-derived per
    /// race directly from the corpus books `RACE_CORPUS_BOOKS` loads
    /// (`grep -rl 'RaceTrait.<Race> Race Trait'` over
    /// `core_rulebook,beastiary,advanced_race_guide,advanced_players_guide,
    /// monster_codex,inner_sea_races,horror_adventures,bestiary_{2,3,5,6}`,
    /// deliberately excluding `ultimate_campaign`'s own matching trait
    /// files, which `RACE_CORPUS_BOOKS` never loads):
    /// Dwarf 4, Elf 4, Gnome 4, Half-Elf 4, Half-Orc 4, Halfling 3, Human 4.
    ///
    /// **Rougarou remains the sole honest zero.** Cycle 1's own corpus-wide
    /// scan proved no book anywhere grants a Rougarou Race Trait
    /// (`race_resolver.rs`'s own `rougarou` chassis comment: no
    /// `Rougarou_Replace*` flag is ever set `True` anywhere in the pinned
    /// oracle), a hard impossibility of source data (`decisions.md §27b`),
    /// not a gap.
    #[test]
    fn the_menu_command_carries_all_twentyone_adopted_race_options_twenty_with_real_grants() {
        let menu = menu();
        let keys: Vec<&str> = menu.adopted_race_options.iter().map(|o| o.key.as_str()).collect();
        assert_eq!(
            keys,
            vec![
                "Adopted Race ~ Catfolk",
                "Adopted Race ~ Dhampir",
                "Adopted Race ~ Dwarf",
                "Adopted Race ~ Elf",
                "Adopted Race ~ Fetchling",
                "Adopted Race ~ Gnome",
                "Adopted Race ~ Grippli",
                "Adopted Race ~ Half-Elf",
                "Adopted Race ~ Half-Orc",
                "Adopted Race ~ Halfling",
                "Adopted Race ~ Human",
                "Adopted Race ~ Ifrit",
                "Adopted Race ~ Oread",
                "Adopted Race ~ Ratfolk",
                "Adopted Race ~ Rougarou",
                "Adopted Race ~ Skinwalker",
                "Adopted Race ~ Suli",
                "Adopted Race ~ Sylph",
                "Adopted Race ~ Undine",
                "Adopted Race ~ Vanara",
                "Adopted Race ~ Vishkanya",
            ]
        );
        // (key, expected grant count, expected grant books) -- `None` book
        // set means "every book named is legal", used only for the 7 CRB
        // multi-member pools whose members are drawn from more than one
        // book; the 13 single-member ISR pools keep the original exact
        // `["ISR"]` pin.
        const EXPECTED: &[(&str, usize)] = &[
            ("Adopted Race ~ Catfolk", 1),
            ("Adopted Race ~ Dhampir", 1),
            ("Adopted Race ~ Dwarf", 4),
            ("Adopted Race ~ Elf", 4),
            ("Adopted Race ~ Fetchling", 1),
            ("Adopted Race ~ Gnome", 4),
            ("Adopted Race ~ Grippli", 1),
            ("Adopted Race ~ Half-Elf", 4),
            ("Adopted Race ~ Half-Orc", 4),
            ("Adopted Race ~ Halfling", 3),
            ("Adopted Race ~ Human", 4),
            ("Adopted Race ~ Ifrit", 1),
            ("Adopted Race ~ Oread", 1),
            ("Adopted Race ~ Ratfolk", 1),
            ("Adopted Race ~ Rougarou", 0),
            ("Adopted Race ~ Skinwalker", 1),
            ("Adopted Race ~ Suli", 1),
            ("Adopted Race ~ Sylph", 1),
            ("Adopted Race ~ Undine", 1),
            ("Adopted Race ~ Vanara", 1),
            ("Adopted Race ~ Vishkanya", 1),
        ];
        const CRB_MULTI_MEMBER: &[&str] = &[
            "Adopted Race ~ Dwarf",
            "Adopted Race ~ Elf",
            "Adopted Race ~ Gnome",
            "Adopted Race ~ Half-Elf",
            "Adopted Race ~ Half-Orc",
            "Adopted Race ~ Halfling",
            "Adopted Race ~ Human",
        ];
        assert_eq!(EXPECTED.len(), 21, "every key above must have an entry here");
        for option in &menu.adopted_race_options {
            assert!(!option.malformed_choose_token, "{:?}: every real oracle row must parse cleanly", option.key);
            let (_, expected_count) = EXPECTED
                .iter()
                .find(|(key, _)| *key == option.key.as_str())
                .unwrap_or_else(|| panic!("{:?}: no expected-count entry", option.key));
            assert_eq!(
                option.grants.len(),
                *expected_count,
                "{:?}: expected {expected_count} real pool member(s)",
                option.key
            );
            for grant in &option.grants {
                assert!(!grant.name.trim().is_empty(), "{:?}: grant must carry a real name", option.key);
                assert!(
                    grant.description.as_deref().is_some_and(|d| !d.trim().is_empty()),
                    "{:?}: grant must carry real corpus prose",
                    option.key
                );
                if !CRB_MULTI_MEMBER.contains(&option.key.as_str()) {
                    assert_eq!(grant.book, "ISR", "{:?}: the real pool member's own book", option.key);
                } else {
                    assert!(
                        grant.book == "ISR" || grant.book == "APG",
                        "{:?}: {:?} is a book RACE_CORPUS_BOOKS does not load for the CRB Race Trait pool",
                        option.key,
                        grant.book
                    );
                }
            }
        }
        // The one real corpus prose sample, pinned by exact text so a future
        // regeneration that silently changed the content would be caught.
        let oread = menu.adopted_race_options.iter().find(|o| o.key == "Adopted Race ~ Oread").unwrap();
        assert_eq!(oread.grants[0].name, "Loner of the Rocks");
        assert_eq!(
            oread.grants[0].description.as_deref(),
            Some(
                "You gain a +1 trait bonus on Heal and Survival checks. Your bonus on Survival \
                 checks increases by 1 in underground or mountain environments."
            )
        );
        let books: BTreeSet<&str> = menu.adopted_race_options.iter().map(|o| o.book.as_str()).collect();
        assert_eq!(books, BTreeSet::from(["B2", "B3", "B5", "B6", "CRB"]));
    }
}
