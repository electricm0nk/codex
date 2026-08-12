//! Real, book-agnostic `data/corpus/<book>/race/` + `race_trait/` loader and
//! racial-trait resolver.
//!
//! Built 2026-07-31 for SD-27 (`decisions.md §25`, `§26`). Modelled directly on
//! this module's sibling [`corpus_loader`](crate::rules_core::corpus_loader):
//! same [`BookCorpusRoot`] input type (re-used, not re-declared), same
//! "walk the book's subdirectory, skip `LICENSE.json` and `_parity/`,
//! push a diagnostic instead of aborting on a bad record" shape.
//!
//! # What this resolves, and why it is transcription rather than invention
//!
//! PCGen already encodes the alternate-racial-trait swap declaratively
//! (`decisions.md §26`). A standard racial trait is gated on a *negated*
//! fact check naming its own replace-flag:
//!
//! ```text
//! Greed  KEY:Dwarf ~ Greed  TYPE:RacialTraits.Dwarf Racial Trait.Dwarf Racial Default.SpecialQuality
//!        !PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True
//! ```
//!
//! and ARG's alternates are what set those flags (`FACT:Dwarf_ReplaceHatred|True`).
//! The resolution rule is therefore exactly the one `decisions.md §26` states:
//!
//! > a standard trait applies **iff** no selected alternate trait has set its
//! > `suppressed_by_flag`
//!
//! This module implements that and nothing more clever. It does not interpret
//! `BONUS:` formulas (`decisions.md §24` rules those out) — it hands back each
//! resolved trait's raw tokens and bonus chains so downstream, hand-modelled
//! per-feature functions read them exactly as they read a raw-LST-parsed
//! record today.
//!
//! # The four roles a corpus trait can have
//!
//! Classification is read off the record, never assumed. In precedence order:
//!
//! | role | corpus signal | included when |
//! |---|---|---|
//! | [`TraitRole::Default`] | `is_racial_default` (`TYPE:...<Race> Racial Default...`) | always, unless its `suppressed_by_flag` fired |
//! | [`TraitRole::Alternate`] | `sets_replace_flags` non-empty | only when the caller selects it |
//! | [`TraitRole::FlagGranted`] | a *positive* `PREFACT:1,ABILITIES,X=True` token, **or** another trait's `ABILITY:<cat>\|AUTOMATIC\|<key>` naming it | when `X` fired, or when the naming trait applied (and its own suppressor did not) |
//! | [`TraitRole::Unclassified`] | none of the above | **never** — surfaced via [`RaceCorpus::unclassified_traits`] |
//!
//! `FlagGranted` is not a category anyone invented either: ARG's
//! `###Block: Replacement Racial Traits` rows carry a positive `PREFACT`
//! naming the very flag their parent alternate sets. `Saltbeard ~ Dwarf ~ Greed`
//! is gated `PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True`, and
//! `Dwarf ~ Saltbeard` sets that flag — so selecting Saltbeard suppresses the
//! CRB `Greed` and grants the seagoing one, in one motion, with no bespoke
//! wiring.
//!
//! PCGen states the same "you get this because you took that" fact a second
//! way, and this module reads both. Instead of a flag round-trip, the granting
//! alternate can name the replacement outright on its own row:
//!
//! ```text
//! Feral  KEY:Orc ~ Feral  ABILITY:Orc Racial Trait|AUTOMATIC|Feral ~ Languages
//!        FACT:Orc_ReplaceLanguages|True
//! ```
//!
//! `Feral ~ Languages` carries no `PREFACT` of its own, so nothing but that
//! token connects the two. [`link_automatic_grants`] resolves it after load —
//! it is a fact about a *pair* of records, which per-record classification
//! cannot see. Exactly two ARG rows depend on this shape and would otherwise
//! never reach a player, which is what it shipped as until 2026-07-31.
//!
//! `Unclassified` exists so that a row with no readable gate is *visible*
//! rather than silently included (which would double a bonus) or silently
//! dropped (which would lose content). No record lands there today — see
//! [`RaceCorpus::unclassified_traits`].
//!
//! # Book attribution
//!
//! A record's book is the corpus directory it was loaded from, which per
//! `decisions.md §25.2` is its *true* source book. `core_essentials/` appears
//! only inside [`RaceChassisRecord::source_path`], where it is the genuine
//! read location, and never as attribution.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::rules_core::corpus_loader::BookCorpusRoot;
use crate::rules_core::feat_effects::FeatDisplayValueDeltas;
use crate::rules_core::pcgen_desc::{render_pcgen_desc_tokens, PcgenDisplayValues, RenderedPcgenDesc};
use crate::rules_core::shape_b_v1::{
    validate_license, CorpusRecordV1, CorpusSource, RaceCacheData, RaceTraitCacheData, RawBonusChain, RawToken,
};
use crate::rules_core::size::SizeCategory;

/// Why one corpus file was skipped. A malformed record must not take down a
/// whole book's real data, and must not vanish silently either — every skip
/// lands here and [`RaceCorpus::diagnostics`] exposes the list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaceCorpusDiagnostic {
    pub path: String,
    pub message: String,
}

/// Which end of the replace-flag protocol a trait record sits on. See the
/// module docs' table for the corpus signal behind each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraitRole {
    /// A standard racial trait: applies by default, suppressible.
    Default,
    /// An ARG alternate: applies only when the caller selects it, and fires
    /// the flags in `sets_replace_flags` when it does.
    Alternate,
    /// Replacement content granted *by another trait* rather than chosen
    /// directly. Two corpus shapes land here, and they are the same fact
    /// stated two ways:
    ///
    /// * a *positive* `PREFACT:1,ABILITIES,X=True` gate on the replacement
    ///   row, where some alternate sets `X` (`Saltbeard ~ Dwarf ~ Greed`);
    /// * a direct `ABILITY:<category>|AUTOMATIC|<key>` token on the granting
    ///   alternate's own row, naming the replacement outright
    ///   (`Orc ~ Feral` -> `Feral ~ Languages`). See
    ///   [`RaceTraitRecord::granted_by_trait_key`].
    ///
    /// Both mean "you get this because you took that", both are resolved by
    /// [`RaceCorpus::resolve`], and neither is ever offered as a menu choice.
    FlagGranted,
    /// No readable gate. Never auto-applied; see the module docs.
    Unclassified,
}

/// One `data/corpus/<book>/race/<slug>.json` record, plus where it came from.
#[derive(Debug, Clone)]
pub struct RaceChassisRecord {
    pub book_id: String,
    pub data: RaceCacheData,
    /// The real LST path the record was ingested from, verbatim from
    /// `CorpusSource.path` (this is the one place `core_essentials/` legitimately
    /// appears — it is where the file physically lives).
    pub source_path: String,
    pub source_line: u32,
    /// The corpus JSON file this record was read from.
    pub corpus_path: PathBuf,
}

/// One `data/corpus/<book>/race_trait/<race>/<slug>.json` record, plus the
/// role classification derived from it.
#[derive(Debug, Clone)]
pub struct RaceTraitRecord {
    pub book_id: String,
    pub role: TraitRole,
    /// From a *positive* `PREFACT:1,ABILITIES,X=True` token: this trait is
    /// granted only when flag `X` has fired.
    pub requires_flag: Option<String>,
    /// The key of the trait record whose `ABILITY:<category>|AUTOMATIC|<key>`
    /// token names *this* record — PCGen's third grant shape. Set by
    /// [`load_race_corpus`]'s post-load pass, which is where it can be known:
    /// it is a fact about a *pair* of records and cannot be read off one.
    ///
    /// `None` for every record no other trait grants outright, including the
    /// [`TraitRole::FlagGranted`] rows that arrive through the positive
    /// `PREFACT` gate instead.
    pub granted_by_trait_key: Option<String>,
    /// True when this record's `description` was replaced by the PI marker at
    /// ingest time (`CorpusRecordV1::pi_field == Some("description")` and
    /// `pi_marker == Some("redacted")`).
    ///
    /// **This field exists because the redaction was being defeated on the
    /// shipped surface.** `pi_screening` redacts `data.description`, but
    /// [`RaceTraitRecord::render_description`] renders from the record's
    /// `DESC:` **raw tokens**, which hold the upstream prose verbatim — so
    /// every Inner Sea Races record whose description was redacted for naming
    /// a Golarion place was nevertheless rendering that place name into the
    /// Race Traits panel. Found by SD-29's race-trait lane round 3
    /// (`decisions.md §47`) when round 2's own RED gate was reproduced; the
    /// redaction had been live and ineffective on 12 records since round 2.
    /// A redacted record now serves its marker instead of re-rendering the
    /// text the screen exists to withhold.
    pub description_redacted: bool,
    pub data: RaceTraitCacheData,
    pub source_path: String,
    pub source_line: u32,
    pub corpus_path: PathBuf,
}

impl RaceTraitRecord {
    fn key(&self) -> &str {
        &self.data.key
    }

    /// This row's own display variables: every variable it both `DEFINE`s and
    /// finishes with unconditional integer `BONUS:VAR` tokens.
    ///
    /// PCGen writes a racial constant across two tokens —
    /// `DEFINE:Gnome_Hatred_AttackBonus|0` plus
    /// `BONUS:VAR|Gnome_Hatred_AttackBonus|1` is the number one — and the row's
    /// own `DESC:` then substitutes it as `%1`. Reading it back is
    /// transcription of a constant, not formula evaluation, so
    /// `decisions.md §24`'s ban on an interpreter is not engaged. This is the
    /// same reading `ingest_races::same_row_vars` already performs at ingest
    /// time, moved here so it can be combined with a *character's* feats
    /// instead of being frozen into the stored description.
    ///
    /// A variable stops resolving the instant any contribution stops being a
    /// same-row literal — a conditional `BONUS:VAR` carrying a trailing
    /// `PRE...` qualifier, an amount naming another variable, or a base
    /// declared in a different file. It is then absent rather than guessed,
    /// which leaves its `%N` dropped and reported exactly as before.
    ///
    /// `BONUS:` tokens live in `raw_bonus_chains`, not `raw_tokens` — the
    /// ingest splits them out, and reading `raw_tokens` for them silently finds
    /// nothing.
    pub fn same_row_display_values(&self) -> PcgenDisplayValues {
        // `Option<i64>` while accumulating so "declared but unresolvable" is
        // distinguishable from "never mentioned"; only the resolved ones are
        // published.
        let mut accumulator: BTreeMap<String, Option<i64>> = BTreeMap::new();

        for token in self.data.raw_tokens.iter().filter(|token| token.key == "DEFINE") {
            let Some((name, base)) = token.value.split_once('|') else { continue };
            accumulator.insert(name.trim().to_string(), base.trim().parse::<i64>().ok());
        }

        for chain in &self.data.raw_bonus_chains {
            let quals = &chain.qualifiers;
            if !quals.first().is_some_and(|q| q.eq_ignore_ascii_case("VAR")) {
                continue;
            }
            let (Some(names), Some(amount)) = (quals.get(1), quals.get(2)) else { continue };
            let conditional =
                quals[3.min(quals.len())..].iter().any(|q| q.starts_with("PRE") || q.starts_with("!PRE"));
            let amount = if conditional { None } else { amount.trim().parse::<i64>().ok() };
            for name in names.split(',') {
                let name = name.trim().to_string();
                match accumulator.get_mut(&name) {
                    // Never `DEFINE`d here, so the base lives elsewhere and
                    // this row cannot finish the variable on its own.
                    None => {
                        accumulator.insert(name, None);
                    }
                    Some(slot) => {
                        *slot = match (*slot, amount) {
                            (Some(current), Some(add)) => Some(current + add),
                            _ => None,
                        };
                    }
                }
            }
        }

        let mut values = PcgenDisplayValues::new();
        for (name, resolved) in accumulator {
            if let Some(value) = resolved {
                values.set(&name, value);
            }
        }
        values
    }

    /// This row's display variables with a character's feat contributions
    /// added — the values its description should actually render for them.
    ///
    /// A feat delta applies only to a variable this row itself resolves. That
    /// is deliberate and load-bearing: Great Hatred's `+1` belongs in
    /// `Gnome ~ Hatred`'s sentence and nowhere else, and a delta that found no
    /// base would otherwise invent one out of the feat alone.
    pub fn display_values_with(&self, deltas: &FeatDisplayValueDeltas) -> PcgenDisplayValues {
        let mut values = self.same_row_display_values();
        for (name, delta) in [
            ("Gnome_Hatred_AttackBonus", deltas.gnome_hatred_attack_bonus),
            ("Halfling_AdaptableLuck_Times", deltas.halfling_adaptable_luck_times),
            ("Halfling_AdaptableLuck_Bonus", deltas.halfling_adaptable_luck_bonus),
        ] {
            if delta == 0 {
                continue;
            }
            if let Some(base) = values.get(name) {
                values.set(name, base + i64::from(delta));
            }
        }
        values
    }

    /// This record's player-facing description, rendered against `values`.
    ///
    /// Reads the record's `DESC:` tokens rather than the stored `description`
    /// string, because the stored one is the *already-collapsed* result of
    /// resolving the row against itself at ingest time — the number is baked in
    /// and the gate branches are already chosen. Re-rendering from the tokens
    /// is what lets a feat change both.
    ///
    /// Falls back to the stored description for a record carrying no `DESC:`
    /// token, so this never returns less than the record already shipped.
    pub fn render_description(&self, values: &PcgenDisplayValues) -> RenderedPcgenDesc {
        // A PI-redacted record serves its stored marker and is never rendered
        // from its raw `DESC:` tokens. Those tokens hold the upstream prose
        // verbatim, so rendering them would put back exactly the Product
        // Identity the ingest-time screen removed -- which is what this
        // surface was doing for 12 Inner Sea Races records between SD-29's
        // race-trait rounds 2 and 3. See [`RaceTraitRecord::description_redacted`].
        if self.description_redacted {
            return RenderedPcgenDesc {
                text: self.data.description.clone().unwrap_or_default(),
                dropped_args: Vec::new(),
            };
        }
        let tokens: Vec<&str> = self
            .data
            .raw_tokens
            .iter()
            .filter(|token| token.key == "DESC")
            .map(|token| token.value.as_str())
            .collect();
        if tokens.is_empty() {
            return RenderedPcgenDesc {
                text: self.data.description.clone().unwrap_or_default(),
                dropped_args: Vec::new(),
            };
        }
        render_pcgen_desc_tokens(&tokens, values)
    }

    /// Every ability key this record grants outright through PCGen's
    /// `ABILITY:<category>|AUTOMATIC|<key>[|<key>...]` token.
    ///
    /// Returned verbatim and **unfiltered** — most of these name things that
    /// are not racial traits at all (`ABILITY:FEAT|AUTOMATIC|Endurance`,
    /// `ABILITY:Class Skill|AUTOMATIC|Survival`,
    /// `ABILITY:Spell-Like Ability|AUTOMATIC|Racial SLA ~ Invisibility`), and
    /// deciding which of them resolve to a loaded race-trait record is the
    /// caller's job, not this accessor's. Returning only the resolvable ones
    /// would hide the rest, and "we found content we cannot place" is a fact
    /// this module deliberately keeps visible.
    pub fn automatic_trait_grants(&self) -> Vec<String> {
        self.data
            .raw_tokens
            .iter()
            .filter(|token| token.key == "ABILITY")
            .flat_map(|token| automatic_grant_targets(&token.value))
            .collect()
    }
}

/// Every race chassis and racial trait from a set of books, indexed by race.
#[derive(Debug, Clone, Default)]
pub struct RaceCorpus {
    chassis: BTreeMap<String, RaceChassisRecord>,
    traits: BTreeMap<String, Vec<RaceTraitRecord>>,
    diagnostics: Vec<RaceCorpusDiagnostic>,
}

/// Where a resolved race's effective creature size came from.
///
/// Exists for the same reason [`SpeedSource`] does: the chassis row is not
/// the whole truth, and a caller is entitled to know which row won.
///
/// # `FACT:BaseSize` is not the race's default size
///
/// This is not an inference from the data — PCGen states it in the field's own
/// definition (`core_essentials/ce__datacontrols.lst:22`):
///
/// ```text
/// FACTDEF:RACE|BaseSize  DATAFORMAT:SIZEADJUSTMENT  REQUIRED:YES  VISIBLE:YES
///     EXPLANATION:All Races must have a Size - in the case of multiple sizes,
///                 use the SMALLEST allowed.
/// ```
///
/// So for a race with more than one legal size, `FACT:BaseSize` is the
/// **smallest allowed**, and the *default* is whatever the race's own
/// automatic `~ Size` racial trait applies via `TEMPLATE:SIZE_<code>`.
/// Reading `FACT:BaseSize` as "the race's size" is a misreading of a field
/// whose own `EXPLANATION:` says otherwise, and it is what made Aasimar and
/// Tiefling — both of which have an opt-in Small variant granted by other
/// books, and are Medium by default — resolve as Small.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SizeSource {
    /// The chassis row's own `FACT:BaseSize|<code>`. Correct only for a race
    /// with a single legal size (which is 16 of the 18 in-scope races).
    Chassis,
    /// A resolved trait's `TEMPLATE:SIZE_<code>`, which overrides the
    /// chassis. Not a nicety: **Aasimar's and Tiefling's chassis rows both
    /// carry `FACT:BaseSize|S` and both races are Medium**, with the real
    /// `TEMPLATE:SIZE_M` on their `~ Size` racial-default trait.
    Trait(String),
    /// Neither a readable `FACT:BaseSize` nor a readable `TEMPLATE:SIZE_` —
    /// size is unknown, not defaulted.
    Unknown,
}

/// Where a resolved race's effective walk speed came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpeedSource {
    /// The chassis row's own `MOVE:Walk,N`.
    Chassis,
    /// A resolved trait's `MOVE:Walk,N`, which overrides the chassis. This is
    /// not a nicety: Goblin's and Hobgoblin's chassis rows carry
    /// `MOVE:Walk,0` and their real 30 ft. lives only on their `Normal Speed`
    /// trait.
    Trait(String),
    /// No `MOVE:Walk` anywhere — speed is unknown, not defaulted.
    Unknown,
}

/// One suppression that fired, with both ends named so a caller can explain
/// the swap rather than just observe a missing trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suppression {
    pub suppressed_trait_key: String,
    pub flag: String,
    pub set_by_trait_key: String,
}

/// One trait that survived resolution, flattened for downstream reading.
#[derive(Debug, Clone)]
pub struct ResolvedTrait {
    pub key: String,
    pub name: String,
    pub book_id: String,
    pub role: TraitRole,
    pub type_tokens: Vec<String>,
    pub description: Option<String>,
    pub source_page: Option<String>,
    pub raw_tokens: Vec<RawToken>,
    pub raw_bonus_chains: Vec<RawBonusChain>,
}

impl ResolvedTrait {
    /// Every integer that appears as a bare numeric qualifier in this trait's
    /// `BONUS:` chains, in source order, deduplicated.
    ///
    /// This is a *reading*, not an interpretation: it does not decide what the
    /// number bonuses, does not sum anything, and does not resolve PCGen
    /// variables. `BONUS:SITUATION|Perception=...|Dwarf_StoneCunning_SkillBonus`
    /// contributes nothing; the companion
    /// `BONUS:VAR|Dwarf_StoneCunning_SkillBonus|2` contributes `2`. Callers that
    /// need a specific mechanical effect must hand-model it per
    /// `decisions.md §24` and read [`raw_bonus_chains`](Self::raw_bonus_chains)
    /// directly.
    pub fn declared_bonus_magnitudes(&self) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();
        for chain in &self.raw_bonus_chains {
            for qualifier in &chain.qualifiers {
                if let Ok(value) = qualifier.parse::<i32>()
                    && !out.contains(&value)
                {
                    out.push(value);
                }
            }
        }
        out
    }

    /// This trait's `MOVE:Walk,N` in feet, if it declares one.
    pub fn declared_walk_speed_ft(&self) -> Option<i32> {
        self.raw_tokens.iter().filter(|t| t.key == "MOVE").find_map(|t| walk_speed_from_move(&t.value))
    }

    /// The creature size this trait's `TEMPLATE:SIZE_<code>` assigns, if it
    /// carries one.
    ///
    /// This is transcription, not interpretation (`decisions.md §24`): PCGen's
    /// `SIZE_*` templates are defined in
    /// `core_essentials/ce_templates.lst:924-933` and each one's entire body
    /// *is* a size assignment —
    /// `SIZE_S  SIZE:S  VISIBLE:NO`, `SIZE_M  SIZE:M  VISIBLE:NO`, and so on
    /// for `F D T S M L H G C`, using the same single-letter code set as
    /// `FACT:BaseSize`. Reading `SIZE_M` off the row that declares it is
    /// reading a constant off the row that defines it.
    ///
    /// `SIZE_C+` (which maps to the non-`SizeCategory` code `P`) and any
    /// other unrecognized suffix yield `None` rather than a guess.
    pub fn declared_size(&self) -> Option<SizeCategory> {
        self.raw_tokens
            .iter()
            .filter(|t| t.key == "TEMPLATE")
            .find_map(|t| size_from_size_template(&t.value))
    }
}

/// A race resolved against a specific set of chosen alternate traits.
#[derive(Debug, Clone)]
pub struct ResolvedRace {
    pub race_key: String,
    pub name: String,
    pub book_id: String,
    /// The race's real creature size: the chassis' `FACT:BaseSize|<code>`,
    /// overridden by any resolved trait's `TEMPLATE:SIZE_<code>`. `None` —
    /// never a defaulted Medium — when neither is present or readable.
    pub size: Option<SizeCategory>,
    /// The chassis row's own `FACT:BaseSize`, before any trait override.
    /// Exposed alongside [`size`](Self::size) for the same reason
    /// [`chassis_walk_speed_ft`](Self::chassis_walk_speed_ft) is: the
    /// Aasimar/Tiefling `FACT:BaseSize|S`-but-actually-Medium case stays
    /// visible rather than being quietly corrected.
    pub chassis_size: Option<SizeCategory>,
    pub size_source: SizeSource,
    pub race_type: Option<String>,
    /// The chassis row's own `MOVE:Walk`, before any trait override. Exposed
    /// alongside [`walk_speed_ft`](Self::walk_speed_ft) so the Goblin/Hobgoblin
    /// `MOVE:Walk,0` case is visible rather than quietly corrected.
    pub chassis_walk_speed_ft: Option<i32>,
    pub walk_speed_ft: Option<i32>,
    pub speed_source: SpeedSource,
    /// Every trait that applies, sorted by key.
    pub traits: Vec<ResolvedTrait>,
    /// The union of the replace-flags the selected alternates set, sorted.
    pub fired_flags: Vec<String>,
    pub suppressions: Vec<Suppression>,
    /// Selection keys that matched no alternate trait for this race. A typo
    /// in a saved character must be reported, not silently ignored.
    pub unmatched_selections: Vec<String>,
    /// Flags that fired but suppressed nothing and granted nothing — an
    /// alternate whose swap target is missing from the loaded books. Empty
    /// when every selected alternate's counterpart is present.
    pub inert_flags: Vec<String>,
}

/// Loads every race chassis and racial trait from every given book's corpus
/// directory. A book with no `race/` or `race_trait/` subdirectory contributes
/// nothing and is not an error — books are wired in incrementally.
pub fn load_race_corpus(roots: &[BookCorpusRoot<'_>]) -> RaceCorpus {
    let mut corpus = RaceCorpus::default();
    for root in roots {
        corpus.load_chassis_dir(root);
        corpus.load_trait_dir(root);
    }
    for records in corpus.traits.values_mut() {
        records.sort_by(|a, b| a.data.key.cmp(&b.data.key));
        link_automatic_grants(records);
    }
    corpus
}

/// Resolves PCGen's third grant shape within one race's record set: an
/// `ABILITY:<category>|AUTOMATIC|<key>` token on one trait naming another.
///
/// Runs after load because it is a fact about a *pair* of records —
/// [`classify`] sees one record at a time and cannot know that something else
/// grants it. A record named this way, and gated no other way, would otherwise
/// stay [`TraitRole::Unclassified`] and never apply: exactly the state
/// `Feral ~ Languages` and `Scion of Humanity ~ Languages` shipped in.
///
/// Only grants that name a record loaded **for the same race** are linked. The
/// overwhelming majority of `AUTOMATIC` grants in the corpus name feats, class
/// skills, spell-like abilities or internal trackers, none of which live in
/// `race_trait/`; those simply find no target and are left alone.
///
/// A record that already has a readable gate is not re-roled — `Dwarf ~
/// Saltbeard` names `Saltbeard ~ Dwarf ~ Greed` *both* ways (direct grant and
/// positive `PREFACT`), and the flag reading is the more specific one.
fn link_automatic_grants(records: &mut [RaceTraitRecord]) {
    let known: BTreeSet<String> = records.iter().map(|r| r.data.key.clone()).collect();
    let mut granted_by: BTreeMap<String, String> = BTreeMap::new();
    for record in records.iter() {
        for target in record.automatic_trait_grants() {
            if target != record.data.key && known.contains(&target) {
                granted_by.entry(target).or_insert_with(|| record.data.key.clone());
            }
        }
    }
    for record in records.iter_mut() {
        let Some(granter) = granted_by.get(&record.data.key) else { continue };
        record.granted_by_trait_key = Some(granter.clone());
        if record.role == TraitRole::Unclassified {
            record.role = TraitRole::FlagGranted;
        }
    }
}

impl RaceCorpus {
    fn load_chassis_dir(&mut self, root: &BookCorpusRoot<'_>) {
        let dir = root.dir.join("race");
        if !dir.is_dir() {
            return;
        }
        for path in find_json_files(&dir) {
            let Some(record) = self.read_record::<RaceCacheData>(&path) else { continue };
            let key = record.data.key.clone();
            let (source_path, source_line) = lst_citation(&record.source);
            let chassis = RaceChassisRecord {
                book_id: root.book_id.to_string(),
                source_path,
                source_line,
                data: record.data,
                corpus_path: path.clone(),
            };
            if let Some(previous) = self.chassis.insert(key.clone(), chassis) {
                self.diagnostics.push(RaceCorpusDiagnostic {
                    path: path.display().to_string(),
                    message: format!(
                        "duplicate chassis for race {key:?}; already loaded from {}",
                        previous.corpus_path.display()
                    ),
                });
            }
        }
    }

    fn load_trait_dir(&mut self, root: &BookCorpusRoot<'_>) {
        let dir = root.dir.join("race_trait");
        if !dir.is_dir() {
            return;
        }
        for path in find_json_files(&dir) {
            let Some(record) = self.read_record::<RaceTraitCacheData>(&path) else { continue };
            let requires_flag = positive_prefact_flag(&record.data.raw_tokens);
            let role = classify(&record.data, requires_flag.is_some());
            let race_key = record.data.race_key.clone();
            let (source_path, source_line) = lst_citation(&record.source);
            self.traits.entry(race_key).or_default().push(RaceTraitRecord {
                book_id: root.book_id.to_string(),
                role,
                requires_flag,
                // Cross-record, so not knowable here; filled by
                // `load_race_corpus`'s post-load pass.
                granted_by_trait_key: None,
                description_redacted: record.pi_field.as_deref() == Some("description")
                    && record.pi_marker.as_deref()
                        == Some(crate::rules_core::shape_b_v1::PI_MARKER_REDACTED),
                source_path,
                source_line,
                data: record.data,
                corpus_path: path,
            });
        }
    }

    /// Reads one corpus file as a real, typed `CorpusRecordV1<T>` and license-
    /// validates it. Any failure becomes a diagnostic and `None` — never a
    /// panic and never a silently-dropped file.
    fn read_record<T: serde::de::DeserializeOwned>(&mut self, path: &Path) -> Option<CorpusRecordV1<T>> {
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(err) => {
                self.push_diag(path, format!("failed to read file: {err}"));
                return None;
            }
        };
        let record = match serde_json::from_str::<CorpusRecordV1<T>>(&text) {
            Ok(record) => record,
            Err(err) => {
                self.push_diag(path, format!("not a valid CorpusRecordV1 payload: {err}"));
                return None;
            }
        };
        if let Err(err) = validate_license(&record) {
            self.push_diag(path, format!("license validation failed: {err}"));
            return None;
        }
        Some(record)
    }

    fn push_diag(&mut self, path: &Path, message: String) {
        self.diagnostics.push(RaceCorpusDiagnostic { path: path.display().to_string(), message });
    }

    pub fn diagnostics(&self) -> &[RaceCorpusDiagnostic] {
        &self.diagnostics
    }

    /// Every race key that has a chassis record, sorted.
    pub fn race_keys(&self) -> Vec<&str> {
        self.chassis.keys().map(String::as_str).collect()
    }

    pub fn chassis(&self, race_key: &str) -> Option<&RaceChassisRecord> {
        self.chassis.get(race_key)
    }

    /// Every trait record for a race, standard and alternate, across every
    /// loaded book, sorted by key.
    pub fn traits_for(&self, race_key: &str) -> Vec<&RaceTraitRecord> {
        self.traits.get(race_key).map(|v| v.iter().collect()).unwrap_or_default()
    }

    pub fn default_traits(&self, race_key: &str) -> Vec<&RaceTraitRecord> {
        self.traits_for(race_key).into_iter().filter(|t| t.role == TraitRole::Default).collect()
    }

    /// The alternate traits a player may choose for a race — the selectable
    /// menu. Keyed by [`RaceTraitCacheData::key`], which is what
    /// [`RaceCorpus::resolve`] takes.
    pub fn alternate_traits(&self, race_key: &str) -> Vec<&RaceTraitRecord> {
        self.traits_for(race_key).into_iter().filter(|t| t.role == TraitRole::Alternate).collect()
    }

    /// Every trait, across every loaded race, that carries no readable gate:
    /// not a racial default, sets no replace-flag, and has no positive
    /// `PREFACT`. These are never auto-applied. They are exposed because
    /// "we found content we cannot place" is a fact a caller is entitled to,
    /// not something to swallow.
    pub fn unclassified_traits(&self) -> Vec<&RaceTraitRecord> {
        let mut out: Vec<&RaceTraitRecord> =
            self.traits.values().flatten().filter(|t| t.role == TraitRole::Unclassified).collect();
        out.sort_by(|a, b| a.data.key.cmp(&b.data.key));
        out
    }

    /// Matches a loose race identifier — a bare key (`"Half-Elf"`), a
    /// `race:`-prefixed character-input token (`"race:half-elf"`), or either
    /// in any case — to a loaded race key. `None` when nothing matches; a
    /// caller that wants a fallback must choose one at its own call site.
    pub fn resolve_key(&self, needle: &str) -> Option<&str> {
        let needle = needle.trim().strip_prefix("race:").unwrap_or(needle.trim());
        self.chassis.keys().find(|key| key.eq_ignore_ascii_case(needle)).map(String::as_str)
    }

    /// Resolves a race's effective trait set against a chosen set of alternate
    /// traits, implementing `decisions.md §26`'s protocol.
    ///
    /// `selected_alternate_keys` are [`RaceTraitCacheData::key`] values, e.g.
    /// `"Dwarf ~ Saltbeard"`. Pass `&[]` for the plain default race.
    ///
    /// `None` only when the race has no chassis record.
    pub fn resolve(&self, race_key: &str, selected_alternate_keys: &[&str]) -> Option<ResolvedRace> {
        let chassis = self.chassis.get(race_key)?;
        let records = self.traits_for(race_key);

        // 1. Match the selections, and fire their flags.
        let selected: BTreeSet<&str> = selected_alternate_keys.iter().copied().collect();
        let mut fired: BTreeMap<String, String> = BTreeMap::new(); // flag -> setting trait key
        let mut matched: BTreeSet<&str> = BTreeSet::new();
        for record in &records {
            if record.role == TraitRole::Alternate && selected.contains(record.key()) {
                matched.insert(record.key());
                for flag in &record.data.sets_replace_flags {
                    fired.entry(flag.clone()).or_insert_with(|| record.key().to_string());
                }
            }
        }
        let unmatched_selections: Vec<String> =
            selected.iter().filter(|k| !matched.contains(*k)).map(|k| (*k).to_string()).collect();

        // 2. Apply the protocol, per role.
        let mut applied: Vec<&RaceTraitRecord> = Vec::new();
        let mut suppressions: Vec<Suppression> = Vec::new();
        let mut used_flags: BTreeSet<String> = BTreeSet::new();
        for record in &records {
            // A suppressor fires regardless of role: a flag-granted
            // replacement can itself be replaced.
            if let Some(flag) = &record.data.suppressed_by_flag
                && let Some(setter) = fired.get(flag)
                && record.role != TraitRole::Alternate
            {
                used_flags.insert(flag.clone());
                suppressions.push(Suppression {
                    suppressed_trait_key: record.key().to_string(),
                    flag: flag.clone(),
                    set_by_trait_key: setter.clone(),
                });
                continue;
            }
            let include = match record.role {
                TraitRole::Default => true,
                TraitRole::Alternate => matched.contains(record.key()),
                TraitRole::FlagGranted => {
                    let required = record.requires_flag.as_deref().unwrap_or_default();
                    let granted = fired.contains_key(required);
                    if granted {
                        used_flags.insert(required.to_string());
                    }
                    granted
                }
                TraitRole::Unclassified => false,
            };
            if include {
                applied.push(record);
            }
        }

        // 2b. The third grant shape. A record named by an *applied* trait's
        //     `ABILITY:...|AUTOMATIC|<key>` token comes in with it.
        //
        //     Deliberately a single extra pass rather than a fixed point:
        //     nothing in the corpus grants transitively (the two granted rows
        //     carry no `ABILITY:` token of their own, which
        //     `the_ability_automatic_grant_shape_is_exactly_two_records_corpus_wide`
        //     re-derives), and a recursive resolver would be machinery built
        //     for content that does not exist.
        let granted_now: BTreeSet<String> =
            applied.iter().flat_map(|record| record.automatic_trait_grants()).collect();
        let already: BTreeSet<&str> = applied.iter().map(|record| record.key()).collect();
        let newly_granted: Vec<&RaceTraitRecord> = records
            .iter()
            .filter(|record| {
                record.role == TraitRole::FlagGranted
                    && !already.contains(record.key())
                    && granted_now.contains(record.key())
                    // A granted row is still suppressible: a flag that fired
                    // and names it wins over the grant.
                    && !record
                        .data
                        .suppressed_by_flag
                        .as_ref()
                        .is_some_and(|flag| fired.contains_key(flag))
            })
            .copied()
            .collect();
        applied.extend(newly_granted);
        // `records` arrives key-sorted, so this is a no-op for everything the
        // first pass appended and only places the newly granted rows.
        applied.sort_by(|a, b| a.key().cmp(b.key()));

        let inert_flags: Vec<String> = fired.keys().filter(|f| !used_flags.contains(*f)).cloned().collect();

        let traits: Vec<ResolvedTrait> = applied
            .iter()
            .map(|record| ResolvedTrait {
                key: record.data.key.clone(),
                name: record.data.name.clone(),
                book_id: record.book_id.clone(),
                role: record.role,
                type_tokens: record.data.type_tokens.clone(),
                description: record.data.description.clone(),
                source_page: record.data.source_page.clone(),
                raw_tokens: record.data.raw_tokens.clone(),
                raw_bonus_chains: record.data.raw_bonus_chains.clone(),
            })
            .collect();

        // 3. Speed: the chassis value, overridden by any resolved trait that
        //    declares its own `MOVE:Walk`.
        let chassis_walk_speed_ft = chassis.data.base_move_walk;
        let mut walk_speed_ft = chassis_walk_speed_ft;
        let mut speed_source = if chassis_walk_speed_ft.is_some() { SpeedSource::Chassis } else { SpeedSource::Unknown };
        for resolved in &traits {
            if let Some(ft) = resolved.declared_walk_speed_ft() {
                walk_speed_ft = Some(ft);
                speed_source = SpeedSource::Trait(resolved.key.clone());
            }
        }

        // 4. Size: exactly the same rule, on exactly the same grounds. The
        //    chassis' `FACT:BaseSize` is a starting point — by its own
        //    `FACTDEF` it is "the SMALLEST allowed" size, not the default one
        //    (see `SizeSource`) — and any resolved trait's
        //    `TEMPLATE:SIZE_<code>` overrides it. Aasimar and Tiefling are the
        //    two races where this changes the answer: both chassis rows say
        //    `S` because each has an opt-in Small variant in another book,
        //    and both races are Medium by default.
        let chassis_size = chassis.data.base_size.as_deref().and_then(SizeCategory::from_base_size_code);
        let mut size = chassis_size;
        let mut size_source = if chassis_size.is_some() { SizeSource::Chassis } else { SizeSource::Unknown };
        for resolved in &traits {
            if let Some(declared) = resolved.declared_size() {
                size = Some(declared);
                size_source = SizeSource::Trait(resolved.key.clone());
            }
        }

        Some(ResolvedRace {
            race_key: chassis.data.key.clone(),
            name: chassis.data.name.clone(),
            book_id: chassis.book_id.clone(),
            size,
            chassis_size,
            size_source,
            race_type: chassis.data.race_type.clone(),
            chassis_walk_speed_ft,
            walk_speed_ft,
            speed_source,
            traits,
            fired_flags: fired.keys().cloned().collect(),
            suppressions,
            unmatched_selections,
            inert_flags,
        })
    }
}

/// Every in-scope race's real creature size, keyed by its corpus race key.
///
/// # Why this is a hand-written table and not a corpus read
///
/// Its two consumers — `contract::to_pilot_receipt` and
/// `pilot_compute_corpus::compute_pilot_with_corpus` — are pure functions over
/// an already-loaded `SourcePackageContent`. Neither may touch the filesystem,
/// and `RaceCorpus` is a separate, disk-backed load. So this is the shape
/// `decisions.md §24` prescribes for exactly this situation: a small
/// hand-modelled pure function whose values were verified against the corpus,
/// pinned by a test that re-derives them from the real on-disk records
/// (`tests/sd27_race_size_resolution.rs`). If the corpus and this table ever
/// disagree, that test fails and names the race.
///
/// # Where each value comes from
///
/// The race's `~ Size` racial-default trait row's `TEMPLATE:SIZE_<code>` in
/// `core_essentials/races/<race>/<race>_abilities_race.lst`, which is what
/// [`RaceCorpus::resolve`] reports as the race's size. That is *not* always the
/// chassis row's `FACT:BaseSize` — which by its own `FACTDEF` is the smallest
/// *allowed* size rather than the default one (see [`SizeSource`]):
/// **Aasimar and Tiefling carry `FACT:BaseSize|S` and are Medium creatures.**
/// Human is the one in-scope race whose `~ Size` row carries no template, and
/// its chassis `FACT:BaseSize|M` is then the declaration.
///
/// This deliberately supersedes
/// `rules_tables::crb::race_tables::race_size_for_race_id`, which knew only the
/// 7 hardcoded CRB races and returned `None` for all 11 Bestiary 1 ones —
/// silently giving Goblin, Kobold and Svirfneblin (all Small) a Medium
/// creature's carrying capacity at both of its call sites.
const RACE_SIZES: &[(&str, SizeCategory)] = &[
    // Core Rulebook's 7.
    ("Dwarf", SizeCategory::Medium),       // TEMPLATE:SIZE_M
    ("Elf", SizeCategory::Medium),         // TEMPLATE:SIZE_M
    ("Gnome", SizeCategory::Small),        // TEMPLATE:SIZE_S
    ("Half-Elf", SizeCategory::Medium),    // TEMPLATE:SIZE_M
    ("Half-Orc", SizeCategory::Medium),    // TEMPLATE:SIZE_M
    ("Halfling", SizeCategory::Small),     // TEMPLATE:SIZE_S
    ("Human", SizeCategory::Medium),       // no template; chassis FACT:BaseSize|M
    // Bestiary 1's 11.
    ("Aasimar", SizeCategory::Medium),     // TEMPLATE:SIZE_M, over a chassis FACT:BaseSize|S
    ("Drow", SizeCategory::Medium),        // TEMPLATE:SIZE_M
    ("Duergar", SizeCategory::Medium),     // TEMPLATE:SIZE_M
    ("Goblin", SizeCategory::Small),       // TEMPLATE:SIZE_S
    ("Hobgoblin", SizeCategory::Medium),   // TEMPLATE:SIZE_M
    ("Kobold", SizeCategory::Small),       // TEMPLATE:SIZE_S
    ("Merfolk", SizeCategory::Medium),     // TEMPLATE:SIZE_M
    ("Orc", SizeCategory::Medium),         // TEMPLATE:SIZE_M
    ("Svirfneblin", SizeCategory::Small),  // TEMPLATE:SIZE_S
    ("Tengu", SizeCategory::Medium),       // TEMPLATE:SIZE_M
    ("Tiefling", SizeCategory::Medium),    // TEMPLATE:SIZE_M, over a chassis FACT:BaseSize|S
];

/// Creature size for a loose race identifier — a `race:<slug>` character-input
/// token, a bare corpus race key, or either in any case, matched by exactly the
/// rule [`RaceCorpus::resolve_key`] uses.
///
/// `None` for a race this repo has not ingested. **Deliberately not defaulted
/// to Medium here**: a caller that needs a fallback must choose one at its own
/// call site and say so, so the assumption stays visible instead of being
/// laundered through this function. See
/// `contract::encumbrance_size_for_race`, which does exactly that and emits a
/// claim-blocking diagnostic when it has to.
pub fn race_size_for_race_token(race_id: &str) -> Option<SizeCategory> {
    let needle = race_id.trim();
    let needle = needle.strip_prefix("race:").unwrap_or(needle);
    RACE_SIZES
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(needle))
        .map(|(_, size)| *size)
}


/// Every alternate racial trait's `<Race>_Replace<Trait>` flag set, keyed
/// by the corpus record key a player selects.
///
/// # Why this is a hand-written table and not a corpus read
///
/// Exactly the situation [`RACE_SIZES`] documents, for exactly the same
/// reason: its consumer is `pilot_compute`, a pure function over an
/// already-loaded [`CharacterInput`](crate::rules_core::character_input::CharacterInput)
/// that may not touch the filesystem, while [`RaceCorpus`] is a separate
/// disk-backed load. `decisions.md §24` prescribes a small hand-modelled
/// function whose values were verified against the corpus and are pinned by a
/// test that re-derives them from the real on-disk records — here
/// [`the_alternate_trait_flag_table_matches_the_corpus_for_every_alternate`]
/// and `tests/sd27_alternate_racial_trait_reachability.rs`. If the corpus and
/// this table ever disagree, that test fails and names the trait.
///
/// # What the values are
///
/// `RaceTraitCacheData::sets_replace_flags`, verbatim and in source order, for
/// all 282 records [`RaceCorpus::alternate_traits`] classifies as
/// [`TraitRole::Alternate`] across the 18 in-scope races — ARG's 153, Monster
/// Codex's 4, the Advanced Player's Guide's 1, Inner Sea Races' 67 and Horror
/// Adventures' 41, the last four landed by SD-29's race-trait lane. The three records that are *not* standalone
/// choices — `Feral ~ Languages`, `Scion of Humanity ~ Languages` and
/// `Saltbeard ~ Dwarf ~ Greed`, all three [`TraitRole::FlagGranted`] — are
/// deliberately absent: a player never selects them, they are granted by the
/// alternate that names them, and putting them here would offer them as menu
/// items. `Oversized Goblin` is absent for the opposite reason: it is
/// [`TraitRole::Unclassified`] and never applies at all.
///
/// **The coverage claim is only as wide as the corpus the pin test loads.**
/// Both pin tests read the app's own `RACE_CORPUS_BOOKS`; when they were
/// pinned to three hardcoded book roots instead, four Monster Codex
/// alternates reached the player's picker while this table stayed silent
/// about them and `pilot_compute` refused every selection with a
/// claim-blocking `race.alternate_trait.unknown`. See
/// [`every_alternate_the_app_offers_is_one_the_engine_can_place`].
const ALTERNATE_TRAIT_REPLACE_FLAGS: &[(&str, &[&str])] = &[
    // ---- Aasimar ----
    ("Aasimar ~ Celestial Crusader", &["Aasimar_ReplaceCelestialResistance", "Aasimar_ReplaceSkilled"]),
    ("Aasimar ~ Deathless Spirit", &["Aasimar_ReplaceCelestialResistance"]),
    ("Aasimar ~ Exalted Resistance", &["Aasimar_ReplaceCelestialResistance"]),
    ("Aasimar ~ Halo", &["Aasimar_ReplaceVision"]),
    ("Aasimar ~ Heavenborn", &["Aasimar_ReplaceSkilled", "Aasimar_ReplaceSpellLikeAbility"]),
    ("Aasimar ~ Immortal Spark", &["Aasimar_ReplaceSkilled", "Aasimar_ReplaceSpellLikeAbility"]),
    ("Aasimar ~ Incorruptible", &["Aasimar_ReplaceSpellLikeAbility"]),
    ("Aasimar ~ Scion of Humanity", &["Aasimar_ReplaceLanguages"]),
    ("Aasimar ~ Truespeaker", &["Aasimar_ReplaceSkilled"]),
    // ---- Drow ----
    ("Drow ~ Ambitious Schemer", &["Drow_ReplaceKeenSenses"]),
    ("Drow ~ Ancestral Grudge", &["Drow_ReplacePoisonUse"]),
    ("Drow ~ Blasphemous Covenant", &["Drow_ReplaceKeenSenses", "Drow_ReplacePoisonUse"]),
    ("Drow ~ Darklands Stalker", &["Drow_ReplaceSpellLikeAbilities"]),
    ("Drow ~ Seducer", &["Drow_ReplaceDrowImmunities"]),
    ("Drow ~ Surface Infiltrator", &["Drow_ReplaceVision", "Drow_ReplaceLightBlindness"]),
    // ---- Duergar ----
    ("Duergar ~ Blood Enmity", &["Duergar_ReplaceSLAInvisibility"]),
    ("Duergar ~ Daysighted", &["Duergar_ReplaceLightSensitivity", "Duergar_ReplaceVision"]),
    ("Duergar ~ Deep Magic", &["Duergar_ReplaceSpellLikeAbilities"]),
    ("Duergar ~ Dwarf Traits (Replaces Duergar Immunities)", &["Duergar_ReplaceDuergarImmunities"]),
    ("Duergar ~ Dwarf Traits (Replaces Stability)", &["Duergar_ReplaceStability"]),
    // Monster Codex (SD-29's race-trait pilot). `mc_abilities_race.lst:16,17`.
    ("Duergar ~ Ironskinned", &["Duergar_ReplaceSLAEnlargePerson"]),
    ("Duergar ~ Twilight-Touched", &["Duergar_ReplaceSLAInvisibility"]),
    // ---- Dwarf ----
    ("Dwarf ~ Ancient Enmity", &["Dwarf_ReplaceHatred"]),
    ("Dwarf ~ Craftsman", &["Dwarf_ReplaceGreed"]),
    ("Dwarf ~ Deep Warrior", &["Dwarf_ReplaceDefensiveTraining"]),
    ("Dwarf ~ Giant Hunter", &["Dwarf_ReplaceHatred"]),
    ("Dwarf ~ Lorekeeper", &["Dwarf_ReplaceGreed"]),
    ("Dwarf ~ Magic Resistant", &["Dwarf_ReplaceHardy"]),
    ("Dwarf ~ Minesight", &["Dwarf_ReplaceVision"]),
    ("Dwarf ~ Mountaineer", &["Dwarf_ReplaceStability"]),
    ("Dwarf ~ Relentless", &["Dwarf_ReplaceStability"]),
    ("Dwarf ~ Rock Stepper", &["Dwarf_ReplaceStonecunning"]),
    (
        "Dwarf ~ Saltbeard",
        &[
        "Dwarf_ReplaceDefensiveTraining",
        "Dwarf_ReplaceHatred",
        "Dwarf_ReplaceStonecunning",
        "Dwarf_ReplaceGreed"
        ],
    ),
    (
        "Dwarf ~ Sky Sentinel",
        &[
        "Dwarf_ReplaceDefensiveTraining",
        "Dwarf_ReplaceHatred",
        "Dwarf_ReplaceStonecunning"
        ],
    ),
    ("Dwarf ~ Stonesinger", &["Dwarf_ReplaceStonecunning"]),
    ("Dwarf ~ Stubborn", &["Dwarf_ReplaceHardy"]),
    ("Dwarf ~ Surface Survivalist", &["Dwarf_ReplaceVision"]),
    (
        "Dwarf ~ Wyrmscourged",
        &[
        "Dwarf_ReplaceDefensiveTraining",
        "Dwarf_ReplaceHatred",
        "Dwarf_ReplaceStonecunning"
        ],
    ),
    ("Dwarf ~ Xenophobic", &["Dwarf_ReplaceLanguages"]),
    // ---- Elf ----
    ("Elf ~ Arcane Focus", &["Elf_ReplaceWeaponFamiliarity"]),
    ("Elf ~ Darkvision", &["Elf_ReplaceVision"]),
    ("Elf ~ Desert Runner", &["Elf_ReplaceElvenMagic"]),
    ("Elf ~ Dreamspeaker", &["Elf_ReplaceElvenImmunities"]),
    ("Elf ~ Elemental Resistance", &["Elf_ReplaceElvenImmunities"]),
    ("Elf ~ Envoy", &["Elf_ReplaceElvenMagic"]),
    ("Elf ~ Eternal Grudge", &["Elf_ReplaceElvenMagic"]),
    ("Elf ~ Fleet-Footed", &["Elf_ReplaceWeaponFamiliarity", "Elf_ReplaceKeenSenses"]),
    ("Elf ~ Lightbringer", &["Elf_ReplaceElvenImmunities", "Elf_ReplaceElvenMagic"]),
    ("Elf ~ Silent Hunter", &["Elf_ReplaceElvenMagic"]),
    ("Elf ~ Spirit of the Waters", &["Elf_ReplaceElvenMagic", "Elf_ReplaceWeaponFamiliarity"]),
    ("Elf ~ Urbanite", &["Elf_ReplaceKeenSenses"]),
    ("Elf ~ Woodcraft", &["Elf_ReplaceElvenMagic"]),
    // ---- Gnome ----
    ("Gnome ~ Academician", &["Gnome_ReplaceObsessive"]),
    ("Gnome ~ Bond to the Land", &["Gnome_ReplaceHatred", "Gnome_ReplaceDefensiveTraining"]),
    ("Gnome ~ Darkvision", &["Gnome_ReplaceVision", "Gnome_ReplaceKeenSenses"]),
    ("Gnome ~ Eternal Hope", &["Gnome_ReplaceDefensiveTraining", "Gnome_ReplaceHatred"]),
    ("Gnome ~ Explorer", &["Gnome_ReplaceHatred", "Gnome_ReplaceObsessive"]),
    ("Gnome ~ Fell Magic", &["Gnome_ReplaceGnomeMagic"]),
    ("Gnome ~ Gift of Tongues", &["Gnome_ReplaceDefensiveTraining", "Gnome_ReplaceHatred"]),
    ("Gnome ~ Knack With Poison", &["Gnome_ReplaceObsessive", "Gnome_ReplaceIllusionResistance"]),
    ("Gnome ~ Magical Linguist", &["Gnome_ReplaceGnomeMagic", "Gnome_ReplaceIllusionResistance"]),
    ("Gnome ~ Master Tinker", &["Gnome_ReplaceDefensiveTraining", "Gnome_ReplaceHatred"]),
    ("Gnome ~ Pyromaniac", &["Gnome_ReplaceGnomeMagic", "Gnome_ReplaceIllusionResistance"]),
    ("Gnome ~ Warden of Nature", &["Gnome_ReplaceDefensiveTraining", "Gnome_ReplaceHatred"]),
    // ---- Goblin ----
    ("Goblin ~ Cave Crawler", &["Goblin_ReplaceSpeed"]),
    ("Goblin ~ City Scavenger", &["Goblin_ReplaceSkilled"]),
    ("Goblin ~ Eat Anything", &["Goblin_ReplaceSkilled"]),
    ("Goblin ~ Hard Head Big Teeth", &["Goblin_ReplaceSkilled"]),
    ("Goblin ~ Over-Sized Ears", &["Goblin_ReplaceSkilled"]),
    ("Goblin ~ Tree Runner", &["Goblin_ReplaceSkilled"]),
    ("Goblin ~ Weapon Familiarity", &["Goblin_ReplaceSkilled"]),
    // Monster Codex (`mc_abilities_race.lst:35,36`). Upstream these two are
    // granted together by picking the `Oversized Goblin` variant out of a
    // `BONUS:ABILITYPOOL|Goblin Variant|1` pool, a mechanism this engine does
    // not model; `Oversized Goblin` itself is `TraitRole::Unclassified` and is
    // deliberately absent from this table. Until the variant mechanism exists
    // these two are individually selectable, which is recorded as a shortfall
    // in `reach_gate`'s `OPEN_FINDINGS`, not smoothed over.
    ("Oversized Goblin ~ Ability Scores", &["Goblin_ReplaceAbilityScores"]),
    ("Oversized Goblin ~ Size", &["Goblin_ReplaceSize"]),
    // ---- Half-Elf ----
    ("Half-Elf ~ Ancestral Arms", &["HalfElf_ReplaceAdaptability"]),
    ("Half-Elf ~ Arcane Training", &["HalfElf_ReplaceMultitalented"]),
    ("Half-Elf ~ Drow Magic", &["HalfElf_ReplaceAdaptability", "HalfElf_ReplaceMultitalented"]),
    ("Half-Elf ~ Drow-Blooded", &["HalfElf_ReplaceVision"]),
    ("Half-Elf ~ Dual Minded", &["HalfElf_ReplaceAdaptability"]),
    ("Half-Elf ~ Integrated", &["HalfElf_ReplaceAdaptability"]),
    ("Half-Elf ~ Sociable", &["HalfElf_ReplaceAdaptability"]),
    ("Half-Elf ~ Wary", &["HalfElf_ReplaceKeenSenses"]),
    ("Half-Elf ~ Water Child", &["HalfElf_ReplaceAdaptability", "HalfElf_ReplaceMultitalented"]),
    // ---- Half-Orc ----
    ("Half-Orc ~ Acute Darkvision", &["HalfOrc_ReplaceOrcFerocity"]),
    ("Half-Orc ~ Beastmaster", &["HalfOrc_ReplaceOrcFerocity"]),
    ("Half-Orc ~ Bestial", &["HalfOrc_ReplaceOrcFerocity"]),
    ("Half-Orc ~ Cavewight", &["HalfOrc_ReplaceIntimidating"]),
    ("Half-Orc ~ Chain Fighter", &["HalfOrc_ReplaceWeaponFamiliarity"]),
    ("Half-Orc ~ City-Raised", &["HalfOrc_ReplaceWeaponFamiliarity"]),
    ("Half-Orc ~ Forest Walker", &["HalfOrc_ReplaceVision"]),
    ("Half-Orc ~ Gatecrasher", &["HalfOrc_ReplaceOrcFerocity"]),
    // Advanced Player's Guide (`apg_abilities_race.lst:83`). The one APG
    // alternate whose key is not already published by ARG; SD-27
    // `decisions.md §39` deferred it precisely because this table did not
    // know it, and SD-29's race-trait extend lane closes that deferral.
    ("Half-Orc ~ Plagueborn", &["HalfOrc_ReplaceIntimidating", "HalfOrc_ReplaceWeaponFamiliarity"]),
    ("Half-Orc ~ Rock Climber", &["HalfOrc_ReplaceIntimidating"]),
    ("Half-Orc ~ Sacred Tattoo", &["HalfOrc_ReplaceOrcFerocity"]),
    ("Half-Orc ~ Scavenger", &["HalfOrc_ReplaceIntimidating"]),
    ("Half-Orc ~ Shaman's Apprentice", &["HalfOrc_ReplaceIntimidating"]),
    ("Half-Orc ~ Skilled", &["HalfOrc_ReplaceVision"]),
    ("Half-Orc ~ Toothy", &["HalfOrc_ReplaceOrcFerocity"]),
    // ---- Halfling ----
    ("Halfling ~ Adaptable Luck", &["Halfling_ReplaceHalflingLuck"]),
    ("Halfling ~ Craven", &["Halfling_ReplaceFearless", "Halfling_ReplaceHalflingLuck"]),
    ("Halfling ~ Fleet of Foot", &["Halfling_ReplaceSpeed", "Halfling_ReplaceSureFooted"]),
    ("Halfling ~ Ingratiating", &["Halfling_ReplaceKeenSenses", "Halfling_ReplaceSureFooted"]),
    ("Halfling ~ Low Blow", &["Halfling_ReplaceKeenSenses"]),
    ("Halfling ~ Outrider", &["Halfling_ReplaceSureFooted"]),
    ("Halfling ~ Polyglot", &["Halfling_ReplaceKeenSenses"]),
    ("Halfling ~ Practicality", &["Halfling_ReplaceSureFooted", "Halfling_ReplaceFearless"]),
    ("Halfling ~ Shiftless", &["Halfling_ReplaceSureFooted"]),
    ("Halfling ~ Swift as Shadows", &["Halfling_ReplaceSureFooted"]),
    ("Halfling ~ Underfoot", &["Halfling_ReplaceHalflingLuck"]),
    ("Halfling ~ Wanderlust", &["Halfling_ReplaceFearless", "Halfling_ReplaceHalflingLuck"]),
    ("Halfling ~ Warslinger", &["Halfling_ReplaceSureFooted"]),
    // ---- Hobgoblin ----
    ("Hobgoblin ~ Bandy-Legged", &["Hobgoblin_ReplaceSpeed"]),
    ("Hobgoblin ~ Battle-Hardened", &["Hobgoblin_ReplaceSneaky"]),
    ("Hobgoblin ~ Engineer", &["Hobgoblin_ReplaceSneaky"]),
    ("Hobgoblin ~ Fearsome", &["Hobgoblin_ReplaceSneaky"]),
    ("Hobgoblin ~ Magehunter", &["Hobgoblin_ReplaceSneaky"]),
    ("Hobgoblin ~ Pit Boss", &["Hobgoblin_ReplaceSneaky"]),
    ("Hobgoblin ~ Scarred", &["Hobgoblin_ReplaceVision"]),
    ("Hobgoblin ~ Slave Hunter", &["Hobgoblin_ReplaceSneaky"]),
    ("Hobgoblin ~ Unfit", &["Hobgoblin_ReplaceSneaky"]),
    // ---- Human ----
    ("Human ~ Adoptive Parentage", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Dual Talent", &["Human_ReplaceAbilityScores", "Human_ReplaceBonusFeat", "Human_ReplaceSkilled"]),
    ("Human ~ Eye for Talent", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Focused Study", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Heart of the Fields", &["Human_ReplaceSkilled"]),
    ("Human ~ Heart of the Mountain", &["Human_ReplaceSkilled"]),
    ("Human ~ Heart of the Sea", &["Human_ReplaceSkilled"]),
    ("Human ~ Heart of the Slums", &["Human_ReplaceSkilled"]),
    ("Human ~ Heart of the Snows", &["Human_ReplaceSkilled"]),
    ("Human ~ Heart of the Streets", &["Human_ReplaceSkilled"]),
    ("Human ~ Heart of the Sun", &["Human_ReplaceSkilled"]),
    ("Human ~ Heart of the Wilderness", &["Human_ReplaceSkilled"]),
    ("Human ~ Heroic", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Mixed Heritage", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Silver Tongued", &["Human_ReplaceSkilled"]),
    // ---- Kobold ----
    ("Kobold ~ Beast Bond", &["Kobold_ReplaceCrafty"]),
    ("Kobold ~ Dragon-Scaled", &["Kobold_ReplaceArmor"]),
    ("Kobold ~ Gliding Wings", &["Kobold_ReplaceCrafty"]),
    ("Kobold ~ Jester", &["Kobold_ReplaceCrafty"]),
    // ---- Merfolk ----
    ("Merfolk ~ Darkvision", &["Merfolk_ReplaceVision"]),
    ("Merfolk ~ Seasinger", &["Merfolk_ReplaceVision"]),
    ("Merfolk ~ Strongtail", &["Merfolk_ReplaceSpeed"]),
    // ---- Orc ----
    ("Orc ~ Dayrunner", &["Orc_ReplaceLightSensitivity"]),
    ("Orc ~ Feral", &["Orc_ReplaceWeaponFamiliarity", "Orc_ReplaceLanguages"]),
    ("Orc ~ Smeller", &["Orc_ReplaceWeaponFamiliarity", "Orc_ReplaceFerocity"]),
    ("Orc ~ Squalid", &["Orc_ReplaceFerocity"]),
    // ---- Svirfneblin ----
    ("Svirfneblin ~ Healthy", &["Svirfneblin_ReplaceFortunate"]),
    ("Svirfneblin ~ Stoneseer", &["Svirfneblin_ReplaceSvirfneblinMagic"]),
    // ---- Tengu ----
    ("Tengu ~ Carrion Sense", &["Tengu_ReplaceGiftedLinguist"]),
    ("Tengu ~ Claw Attack", &["Tengu_ReplaceSwordtrained"]),
    ("Tengu ~ Exotic Weapon Training", &["Tengu_ReplaceSwordtrained"]),
    ("Tengu ~ Glide", &["Tengu_ReplaceGiftedLinguist"]),
    // ---- Tiefling ----
    ("Tiefling ~ Beguiling Liar", &["Tiefling_ReplaceSkilled"]),
    ("Tiefling ~ Fiendish Sprinter", &["Tiefling_ReplaceSkilled"]),
    ("Tiefling ~ Maw or Claw", &["Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Prehensile Tail", &["Tiefling_ReplaceFiendishSorcery"]),
    ("Tiefling ~ Scaled Skin", &["Tiefling_ReplaceFiendishResistance"]),
    ("Tiefling ~ Soul Seer", &["Tiefling_ReplaceFiendishSorcery", "Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Vestigial Wings", &["Tiefling_ReplaceSkilled"]),

    // ================= Inner Sea Races =================
    // SD-29 race-trait lane, round 2. 68 of the book's 72 in-scope rows set
    // a `<Race>_Replace<Trait>` flag and are therefore `TraitRole::Alternate`;
    // every value below is `RaceTraitCacheData::sets_replace_flags` read
    // verbatim off `data/corpus/inner_sea_races/race_trait/`, and
    // `the_alternate_trait_flag_table_matches_the_corpus_for_every_alternate`
    // re-derives all 68 from those records rather than trusting this block.
    // The other 4 rows are deliberately absent: `Junk Tinker ~ Skilled` and
    // `Secret Magic ~ Merfolk ~ Speed` and `Pass for Human ~ Tiefling ~
    // Languages` are `TraitRole::FlagGranted` (a player never selects them),
    // and `Human ~ Tribalistic Languages` is `TraitRole::Unclassified` —
    // nothing in the upstream corpus grants it, which is recorded as a
    // shortfall in `reach_gate`'s `OPEN_FINDINGS` rather than smoothed over.
    // ---- Aasimar ----
    ("Aasimar ~ Crusading Magic", &["Aasimar_ReplaceSkilled", "Aasimar_ReplaceSpellLikeAbility"]),
    ("Aasimar ~ Lost Promise", &["Aasimar_ReplaceSpellLikeAbility"]),
    // ---- Drow ----
    ("Drow ~ Defensive Training", &["Drow_ReplaceDrowImmunities", "Drow_ReplaceKeenSenses", "Drow_ReplacePoisonUse"]),
    // ---- Duergar ----
    ("Duergar ~ Magical Taskmaster", &["Duergar_ReplaceSLAInvisibility"]),
    // ---- Dwarf ----
    ("Dwarf ~ Lasting Grudge", &["Dwarf_ReplaceDefensiveTraining", "Dwarf_ReplaceHatred"]),
    ("Dwarf ~ Siege Survivor", &["Dwarf_ReplaceGreed", "Dwarf_ReplaceHardy", "Dwarf_ReplaceHatred"]),
    ("Dwarf ~ Slag Child", &["Dwarf_ReplaceDefensiveTraining", "Dwarf_ReplaceHatred"]),
    ("Dwarf ~ Spell Smasher", &["Dwarf_ReplaceDefensiveTraining", "Dwarf_ReplaceHatred"]),
    ("Dwarf ~ Spiritual Support", &["Dwarf_ReplaceGreed", "Dwarf_ReplaceHardy"]),
    ("Dwarf ~ Stoic Negotiator", &["Dwarf_ReplaceDefensiveTraining", "Dwarf_ReplaceHatred", "Dwarf_ReplaceStonecunning"]),
    ("Dwarf ~ Unstoppable", &["Dwarf_ReplaceHardy"]),
    // ---- Elf ----
    ("Elf ~ Ageless Patience", &["Elf_ReplaceElvenMagic", "Elf_ReplaceKeenSenses"]),
    ("Elf ~ Disinterested Observer", &["Elf_ReplaceElvenMagic", "Elf_ReplaceWeaponFamiliarity"]),
    ("Elf ~ Elven Arrogance", &["Elf_ReplaceLanguages"]),
    ("Elf ~ Human-Raised", &["Elf_ReplaceElvenMagic", "Elf_ReplaceWeaponFamiliarity"]),
    ("Elf ~ Memories Beyond Death", &["Elf_ReplaceElvenImmunities", "Elf_ReplaceElvenMagic"]),
    ("Elf ~ Overwhelming Magic", &["Elf_ReplaceElvenMagic", "Elf_ReplaceWeaponFamiliarity"]),
    ("Elf ~ Retreat Magic", &["Elf_ReplaceElvenMagic"]),
    // ---- Gnome ----
    ("Gnome ~ Architectural Ingenuity", &["Gnome_ReplaceKeenSenses", "Gnome_ReplaceObsessive"]),
    ("Gnome ~ Dirty Trickster", &["Gnome_ReplaceHatred", "Gnome_ReplaceKeenSenses"]),
    ("Gnome ~ Intrepid Settler", &["Gnome_ReplaceIllusionResistance", "Gnome_ReplaceKeenSenses", "Gnome_ReplaceObsessive"]),
    ("Gnome ~ Utilitarian Magic", &["Gnome_ReplaceGnomeMagic"]),
    ("Gnome ~ Vivacious", &["Gnome_ReplaceGnomeMagic", "Gnome_ReplaceKeenSenses"]),
    ("Gnome ~ Wright", &["Gnome_ReplaceHatred", "Gnome_ReplaceObsessive"]),
    // ---- Goblin ----
    ("Goblin ~ Junk Tinker", &["Goblin_ReplaceSkilled"]),
    // ---- Half-Elf ----
    ("Half-Elf ~ Elf-Scorned", &["HalfElf_ReplaceElvenImmunities", "HalfElf_ReplaceMultitalented"]),
    ("Half-Elf ~ Eye for Opportunity", &["HalfElf_ReplaceAdaptability", "HalfElf_ReplaceKeenSenses"]),
    ("Half-Elf ~ Kindred-Raised", &["HalfElf_ReplaceAbilityScores", "HalfElf_ReplaceAdaptability", "HalfElf_ReplaceElvenImmunities", "HalfElf_ReplaceKeenSenses", "HalfElf_ReplaceMultitalented"]),
    ("Half-Elf ~ Reflexive Improvisation", &["HalfElf_ReplaceAdaptability", "HalfElf_ReplaceMultitalented"]),
    ("Half-Elf ~ Sea Legs", &["HalfElf_ReplaceAdaptability"]),
    ("Half-Elf ~ Sophisticate", &["HalfElf_ReplaceElvenImmunities"]),
    ("Half-Elf ~ Weapon Familiarity", &["HalfElf_ReplaceAdaptability"]),
    // ---- Half-Orc ----
    ("Half-Orc ~ Divided Attention", &["HalfOrc_ReplaceIntimidating", "HalfOrc_ReplaceOrcFerocity"]),
    ("Half-Orc ~ Human-Raised", &["HalfOrc_ReplaceOrcFerocity", "HalfOrc_ReplaceWeaponFamiliarity"]),
    ("Half-Orc ~ Orc Atavism", &["HalfOrc_ReplaceAbilityScores", "HalfOrc_ReplaceIntimidating", "HalfOrc_ReplaceOrcFerocity"]),
    ("Half-Orc ~ Overlooked Mastermind", &["HalfOrc_ReplaceIntimidating", "HalfOrc_ReplaceOrcFerocity", "HalfOrc_ReplaceWeaponFamiliarity"]),
    ("Half-Orc ~ Pariah", &["HalfOrc_ReplaceWeaponFamiliarity"]),
    ("Half-Orc ~ Sea Raider", &["HalfOrc_ReplaceVision"]),
    ("Half-Orc ~ War-Leader", &["HalfOrc_ReplaceVision", "HalfOrc_ReplaceOrcFerocity"]),
    // ---- Halfling ----
    ("Halfling ~ Caretaker", &["Halfling_ReplaceHalflingLuck", "Halfling_ReplaceSureFooted", "Halfling_ReplaceWeaponFamiliarity"]),
    ("Halfling ~ Driven Worker", &["Halfling_ReplaceSureFooted"]),
    ("Halfling ~ Human Shadow", &["Halfling_ReplaceKeenSenses", "Halfling_ReplaceSureFooted"]),
    ("Halfling ~ Secretive Survivor", &["Halfling_ReplaceSureFooted"]),
    ("Halfling ~ Skulker", &["Halfling_ReplaceFearless", "Halfling_ReplaceWeaponFamiliarity"]),
    ("Halfling ~ Unfettered", &["Halfling_ReplaceHalflingLuck", "Halfling_ReplaceKeenSenses"]),
    ("Halfling ~ Unlucky Halfling", &["Halfling_ReplaceHalflingLuck"]),
    // ---- Hobgoblin ----
    ("Hobgoblin ~ Authoritative", &["Hobgoblin_ReplaceSneaky"]),
    // ---- Human ----
    ("Human ~ Awareness", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Comprehensive Education", &["Human_ReplaceSkilled"]),
    ("Human ~ Industrious", &["Human_ReplaceSkilled"]),
    ("Human ~ Innovative", &["Human_ReplaceSkilled"]),
    ("Human ~ Institutional Memory", &["Human_ReplaceSkilled"]),
    ("Human ~ Military Tradition", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Practiced Hunter", &["Human_ReplaceSkilled"]),
    ("Human ~ Self-Made Fate", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Social Ties", &["Human_ReplaceSkilled"]),
    ("Human ~ Tribalistic", &["Human_ReplaceLanguages"]),
    ("Human ~ Unstoppable Magic", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Wayfarer", &["Human_ReplaceSkilled"]),
    // ---- Kobold ----
    ("Kobold ~ Dragon Affinity", &["Kobold_ReplaceArmor"]),
    // ---- Merfolk ----
    ("Merfolk ~ Secret Magic", &["Merfolk_ReplaceArmor", "Merfolk_ReplaceSpeed"]),
    // ---- Orc ----
    ("Orc ~ Reckless Climber", &["Orc_ReplaceFerocity"]),
    // ---- Svirfneblin ----
    ("Svirfneblin ~ Stalwart Watcher", &["Svirfneblin_ReplaceHatred", "Svirfneblin_ReplaceSkilled"]),
    // ---- Tengu ----
    ("Tengu ~ Deft Swords", &["Tengu_ReplaceNaturalWeapon", "Tengu_ReplaceSneaky"]),
    // ---- Tiefling ----
    ("Tiefling ~ Bullying", &["Tiefling_ReplaceSkilled"]),
    ("Tiefling ~ Light from the Darkness", &["Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Pass for Human", &["Tiefling_ReplaceType", "Tiefling_ReplaceLanguages"]),

    // ================= Horror Adventures =================
    // SD-29 race-trait lane, round 3. 41 of the book's 43 in-scope rows in
    // `ha_abilities_race.lst` set a `<Race>_Replace<Trait>` flag and are
    // therefore `TraitRole::Alternate`; every value below is
    // `RaceTraitCacheData::sets_replace_flags` read verbatim off
    // `data/corpus/horror_adventures/race_trait/`, and
    // `the_alternate_trait_flag_table_matches_the_corpus_for_every_alternate`
    // re-derives all 41 from those records rather than trusting this block.
    //
    // The other 2 rows are deliberately absent and are NOT a shortfall:
    // `Deep Jungle Halfling ~ Languages` and `Deep Jungle Halfling ~ Poison
    // Use` are `TraitRole::FlagGranted`. A player never selects them --
    // `Halfling ~ Deep Jungle` (the alternate below that fires
    // `Halfling_ReplaceLanguages`, `Halfling_ReplaceSureFooted` and
    // `Halfling_ReplaceWeaponFamiliarity`) grants them by name through
    // `ABILITY:Halfling Racial Trait|AUTOMATIC|Deep Jungle Halfling ~
    // Languages|Deep Jungle Halfling ~ Poison Use`
    // (`ha_abilities_race.lst:85`). That is the *opposite* of Inner Sea
    // Races' `Human ~ Tribalistic Languages`, whose owning alternate
    // suppresses a standard trait and brings nothing in
    // (SD-29 `decisions.md §45.4`): here the upstream transaction is
    // complete, so this book contributes no unreachable record and needs no
    // `OPEN_FINDINGS` entry. `no_corpus_trait_is_left_without_a_readable_gate`
    // is the assertion that proves it -- it did not move for this book.
    // ---- Dwarf ----
    ("Dwarf ~ Barrow Scholar", &["Dwarf_ReplaceStonecunning"]),
    ("Dwarf ~ Barrow Warden", &["Dwarf_ReplaceDefensiveTraining", "Dwarf_ReplaceHatred"]),
    ("Dwarf ~ Healthy", &["Dwarf_ReplaceHardy"]),
    ("Dwarf ~ Sense Aberration", &["Dwarf_ReplaceStonecunning"]),
    ("Dwarf ~ Tightfisted", &["Dwarf_ReplaceStability", "Dwarf_ReplaceStonecunning"]),
    ("Dwarf ~ Viscous Blood", &["Dwarf_ReplaceHardy"]),
    // ---- Elf ----
    ("Elf ~ Blightborn", &["Elf_ReplaceElvenImmunities"]),
    ("Elf ~ Creepy", &["Elf_ReplaceElvenMagic"]),
    ("Elf ~ Keeper of Secrets", &["Elf_ReplaceElvenMagic"]),
    ("Elf ~ Light against Darkness", &["Elf_ReplaceElvenMagic"]),
    ("Elf ~ Long-Limbed", &["Elf_ReplaceWeaponFamiliarity"]),
    ("Elf ~ Perfect", &["Elf_ReplaceElvenImmunities"]),
    ("Elf ~ Slender", &["Elf_ReplaceElvenImmunities"]),
    // ---- Gnome ----
    ("Gnome ~ Fairy Catcher", &["Gnome_ReplaceDefensiveTraining", "Gnome_ReplaceHatred", "Gnome_ReplaceKeenSenses"]),
    ("Gnome ~ Inquisitive", &["Gnome_ReplaceKeenSenses", "Gnome_ReplaceObsessive"]),
    ("Gnome ~ Shadow Dodger", &["Gnome_ReplaceDefensiveTraining", "Gnome_ReplaceIllusionResistance"]),
    ("Gnome ~ Shadow Foe", &["Gnome_ReplaceDefensiveTraining", "Gnome_ReplaceHatred"]),
    ("Gnome ~ Stalker", &["Gnome_ReplaceDefensiveTraining", "Gnome_ReplaceHatred", "Gnome_ReplaceObsessive"]),
    // ---- Half-Elf ----
    ("Half-Elf ~ Dreamer", &["HalfElf_ReplaceElvenImmunities"]),
    ("Half-Elf ~ Mismatched", &["HalfElf_ReplaceVision", "HalfElf_ReplaceKeenSenses"]),
    ("Half-Elf ~ Multidisciplined", &["HalfElf_ReplaceMultitalented"]),
    ("Half-Elf ~ Round Ears", &["HalfElf_ReplaceVision", "HalfElf_ReplaceKeenSenses", "HalfElf_ReplaceAdaptability"]),
    // ---- Half-Orc ----
    ("Half-Orc ~ Inured", &["HalfOrc_ReplaceOrcFerocity"]),
    ("Half-Orc ~ Monstrous Sympathy", &["HalfOrc_ReplaceIntimidating", "HalfOrc_ReplaceOrcFerocity"]),
    ("Half-Orc ~ Pain Tolerance", &["HalfOrc_ReplaceIntimidating", "HalfOrc_ReplaceOrcFerocity"]),
    ("Half-Orc ~ Projection", &["HalfOrc_ReplaceWeaponFamiliarity", "HalfOrc_ReplaceOrcFerocity"]),
    ("Half-Orc ~ Smog Sight", &["HalfOrc_ReplaceVision"]),
    ("Half-Orc ~ Stoic", &["HalfOrc_ReplaceIntimidating", "HalfOrc_ReplaceOrcFerocity"]),
    // ---- Halfling ----
    ("Halfling ~ Acquisitive", &["Halfling_ReplaceKeenSenses"]),
    ("Halfling ~ Attentive", &["Halfling_ReplaceKeenSenses"]),
    ("Halfling ~ Blessed", &["Halfling_ReplaceFearless"]),
    ("Halfling ~ Creepy Doll", &["Halfling_ReplaceKeenSenses", "Halfling_ReplaceSureFooted"]),
    ("Halfling ~ Deep Jungle", &["Halfling_ReplaceLanguages", "Halfling_ReplaceSureFooted", "Halfling_ReplaceWeaponFamiliarity"]),
    ("Halfling ~ Irrepressible", &["Halfling_ReplaceFearless"]),
    ("Halfling ~ Resourceful", &["Halfling_ReplaceSureFooted", "Halfling_ReplaceWeaponFamiliarity"]),
    // ---- Human ----
    ("Human ~ Aquatic Ancestry", &["Human_ReplaceSkilled"]),
    ("Human ~ Giant Ancestry", &["Human_ReplaceSkilled"]),
    ("Human ~ Piety", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Psychic Defense", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Rationalize", &["Human_ReplaceBonusFeat"]),
    ("Human ~ Reptilian Ancestry", &["Human_ReplaceBonusFeat"]),

    // ================= Core Essentials =================
    // SD-29 race-trait lane, round 4. The 16 Aasimar and Tiefling *heritage*
    // selectors, the only rows in this table whose replace-flags are not
    // stated on their own corpus row: Tiefling's ten carry no `FACT:` token at
    // all and Aasimar's six carry theirs redundantly. Both books state the
    // swap in `core_essentials/races/<race>/<race>_abilities_globalvar_subrace.lst`,
    // and `ingest_race_traits::subrace_grants` reads it there -- so every
    // value below is still `RaceTraitCacheData::sets_replace_flags` read
    // verbatim off `data/corpus/core_essentials/race_trait/`, and
    // `the_alternate_trait_flag_table_matches_the_corpus_for_every_alternate`
    // re-derives all 16 from those records rather than trusting this block.
    //
    // Every heritage of a race replaces the same three standard traits --
    // Ability Scores, Skilled and Spell-Like Ability -- which is what makes
    // them mutually exclusive and is exactly the relation
    // `race_trait_picker::exclusion_guard_flags` now reads from the
    // `PREVAREQ:<flag>,0` qualifier the ingest carries through.
    //
    // The book's other 48 records are deliberately absent: they are the
    // replacement rows the heritages grant, `TraitRole::FlagGranted`, and a
    // player never selects one.
    // ---- Aasimar ----
    ("Aasimar ~ Agathion-Blooded", &["Aasimar_ReplaceAbilityScores", "Aasimar_ReplaceSkilled", "Aasimar_ReplaceSpellLikeAbility"]),
    ("Aasimar ~ Angel-Blooded", &["Aasimar_ReplaceAbilityScores", "Aasimar_ReplaceSkilled", "Aasimar_ReplaceSpellLikeAbility"]),
    ("Aasimar ~ Archon-Blooded", &["Aasimar_ReplaceAbilityScores", "Aasimar_ReplaceSkilled", "Aasimar_ReplaceSpellLikeAbility"]),
    ("Aasimar ~ Azata-Blooded", &["Aasimar_ReplaceAbilityScores", "Aasimar_ReplaceSkilled", "Aasimar_ReplaceSpellLikeAbility"]),
    ("Aasimar ~ Garuda-Blooded", &["Aasimar_ReplaceAbilityScores", "Aasimar_ReplaceSkilled", "Aasimar_ReplaceSpellLikeAbility"]),
    ("Aasimar ~ Peri-Blooded", &["Aasimar_ReplaceAbilityScores", "Aasimar_ReplaceSkilled", "Aasimar_ReplaceSpellLikeAbility"]),
    // ---- Tiefling ----
    ("Tiefling ~ Asura-Spawn", &["Tiefling_ReplaceAbilityScores", "Tiefling_ReplaceSkilled", "Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Daemon-Spawn", &["Tiefling_ReplaceAbilityScores", "Tiefling_ReplaceSkilled", "Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Demodand-Spawn", &["Tiefling_ReplaceAbilityScores", "Tiefling_ReplaceSkilled", "Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Demon-Spawn", &["Tiefling_ReplaceAbilityScores", "Tiefling_ReplaceSkilled", "Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Devil-Spawn", &["Tiefling_ReplaceAbilityScores", "Tiefling_ReplaceSkilled", "Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Div-Spawn", &["Tiefling_ReplaceAbilityScores", "Tiefling_ReplaceSkilled", "Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Kyton-Spawn", &["Tiefling_ReplaceAbilityScores", "Tiefling_ReplaceSkilled", "Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Oni-Spawn", &["Tiefling_ReplaceAbilityScores", "Tiefling_ReplaceSkilled", "Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Qlippoth-Spawn", &["Tiefling_ReplaceAbilityScores", "Tiefling_ReplaceSkilled", "Tiefling_ReplaceSpellLikeAbility"]),
    ("Tiefling ~ Rakshasa-Spawn", &["Tiefling_ReplaceAbilityScores", "Tiefling_ReplaceSkilled", "Tiefling_ReplaceSpellLikeAbility"]),
];

/// The `<Race>_Replace<Trait>` flags a set of selected alternate racial traits
/// fires, sorted and deduplicated.
///
/// This is the pure, filesystem-free half of the `decisions.md §26` protocol:
/// `RaceCorpus::resolve` answers the same question by reading the corpus, and
/// this answers it from the pinned table so `pilot_compute` can gate a
/// standard trait's hand-modelled record on its own declared flag.
///
/// A key this table does not know contributes nothing and is reported by
/// [`unknown_alternate_trait_keys`] rather than silently ignored — a saved
/// character naming a trait the engine cannot place must be visible.
pub fn replace_flags_fired_by(selected_alternate_keys: &[String]) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    for key in selected_alternate_keys {
        let Some((_, flags)) = ALTERNATE_TRAIT_REPLACE_FLAGS.iter().find(|(k, _)| *k == key.as_str()) else {
            continue;
        };
        for flag in *flags {
            if !out.contains(flag) {
                out.push(flag);
            }
        }
    }
    out.sort_unstable();
    out
}

/// Whether any of the selected alternates fires `flag`.
///
/// The single predicate every hand-modelled standard-trait record gates on.
/// `flag` is the standard row's own `!PREFACT:1,ABILITIES,<flag>=True`
/// payload — i.e. `RaceTraitCacheData::suppressed_by_flag`, which
/// `tests/sd27_alternate_racial_trait_reachability.rs` pins against the corpus
/// for every gate the engine declares.
pub fn alternate_traits_fire_flag(selected_alternate_keys: &[String], flag: &str) -> bool {
    selected_alternate_keys.iter().any(|key| {
        ALTERNATE_TRAIT_REPLACE_FLAGS
            .iter()
            .any(|(candidate, flags)| *candidate == key.as_str() && flags.contains(&flag))
    })
}

/// Selection keys that name no alternate racial trait this table knows.
///
/// Never silently dropped: a typo, a trait from an un-ingested book, or a
/// character saved against a later corpus all land here, and the caller is
/// expected to raise them.
pub fn unknown_alternate_trait_keys(selected_alternate_keys: &[String]) -> Vec<String> {
    selected_alternate_keys
        .iter()
        .filter(|key| !ALTERNATE_TRAIT_REPLACE_FLAGS.iter().any(|(k, _)| *k == key.as_str()))
        .cloned()
        .collect()
}

/// Every alternate racial trait key a player may select, sorted.
pub fn selectable_alternate_trait_keys() -> Vec<&'static str> {
    ALTERNATE_TRAIT_REPLACE_FLAGS.iter().map(|(key, _)| *key).collect()
}

/// The LST file and line a record was ingested from. `CorpusSource` also
/// models web and same-book-fallback provenance, neither of which carries an
/// LST citation — those return `("", 0)` rather than a fabricated path.
fn lst_citation(source: &CorpusSource) -> (String, u32) {
    match source {
        CorpusSource::LstToken { path, line, .. }
        | CorpusSource::LstInheritedCopy { path, line, .. }
        | CorpusSource::LstCorrectedIngest { path, line, .. } => (path.clone(), *line),
        CorpusSource::WebSecondSource { .. } | CorpusSource::SameBookFallback { .. } => (String::new(), 0),
    }
}

/// Reads a *positive* `PREFACT:1,ABILITIES,<flag>=True` token — the gate on
/// ARG's replacement-content rows. The negated form is stored under the
/// distinct `!PREFACT` key by the ingest tools and is deliberately not matched
/// here; its payload already lives in
/// [`RaceTraitCacheData::suppressed_by_flag`].
/// `Orc Racial Trait|AUTOMATIC|Feral ~ Languages` -> `["Feral ~ Languages"]`;
/// `Dwarf Racial Trait|AUTOMATIC|Dwarf ~ Weapon Familiarity|Dwarf ~ Languages`
/// -> both keys.
///
/// The token's first field is the ability CATEGORY and the second is the
/// *nature* (`AUTOMATIC`, `VIRTUAL`, `NORMAL`); only `AUTOMATIC` grants
/// unconditionally, so any other nature yields nothing. Everything after that
/// is a `|`-separated key list terminated by PCGen's prerequisite qualifiers
/// (`PRESTAT:`, `PREABILITY:`, `PRELEVEL:`, `PREVAREQ:` and their negations),
/// which are dropped along with the rest of the list — a key list is a run,
/// not a set, and a qualifier applies to what follows it.
///
/// `%LIST` is skipped: it is PCGen's "whatever the player chose" placeholder
/// and names no concrete record.
fn automatic_grant_targets(value: &str) -> Vec<String> {
    let mut parts = value.split('|');
    let _category = parts.next();
    if !parts.next().is_some_and(|nature| nature.eq_ignore_ascii_case("AUTOMATIC")) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for part in parts {
        let part = part.trim();
        if part.starts_with("PRE") || part.starts_with("!PRE") {
            break;
        }
        if part.is_empty() || part == "%LIST" {
            continue;
        }
        out.push(part.to_string());
    }
    out
}

fn positive_prefact_flag(raw_tokens: &[RawToken]) -> Option<String> {
    raw_tokens
        .iter()
        .filter(|t| t.key == "PREFACT")
        .find_map(|t| first_ability_flag(&t.value))
}

/// `1,ABILITIES,Dwarf_ReplaceGreed=True` -> `Dwarf_ReplaceGreed`.
fn first_ability_flag(value: &str) -> Option<String> {
    let mut parts = value.split(',');
    if parts.next()? != "1" {
        return None;
    }
    if !parts.next()?.eq_ignore_ascii_case("ABILITIES") {
        return None;
    }
    let clause = parts.next()?;
    let (flag, _) = clause.split_once('=')?;
    Some(flag.to_string())
}

/// `SIZE_M` -> [`SizeCategory::Medium`]. Any `TEMPLATE:` payload that is not
/// one of `ce_templates.lst`'s nine `SIZE_<code>` rows yields `None` — a race
/// trait carries plenty of other templates, and `SIZE_C+` (whose body is
/// `SIZE:P`, a code `SizeCategory` does not model) must not be mistaken for
/// Colossal.
fn size_from_size_template(value: &str) -> Option<SizeCategory> {
    let code = value.trim().strip_prefix("SIZE_")?;
    if code.len() != 1 {
        return None;
    }
    SizeCategory::from_base_size_code(code)
}

/// `Walk,20` / `Walk,15,Swim,30` -> `20` / `15`. `None` when the token names
/// no walk movement at all.
fn walk_speed_from_move(value: &str) -> Option<i32> {
    let parts: Vec<&str> = value.split(',').collect();
    parts
        .windows(2)
        .find(|pair| pair[0].trim().eq_ignore_ascii_case("Walk"))
        .and_then(|pair| pair[1].trim().parse::<i32>().ok())
}

fn classify(data: &RaceTraitCacheData, has_positive_gate: bool) -> TraitRole {
    if data.is_racial_default {
        TraitRole::Default
    } else if !data.sets_replace_flags.is_empty() {
        TraitRole::Alternate
    } else if has_positive_gate {
        TraitRole::FlagGranted
    } else {
        TraitRole::Unclassified
    }
}

/// Same traversal rule as [`corpus_loader`](crate::rules_core::corpus_loader):
/// recurse, skip `_parity/` and `LICENSE.json`, take `*.json`. Duplicated
/// rather than shared because that module's copy is private and this cycle's
/// write scope does not include editing it.
fn find_json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = fs::read_dir(&current) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            if path.is_dir() {
                if file_name == "_parity" {
                    continue;
                }
                stack.push(path);
            } else if file_name == "LICENSE.json" {
                continue;
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The one test here that deliberately loads a SINGLE book —
    /// `a_race_resolves_from_its_own_book_alone_without_the_alternate_trait_book`
    /// — needs a hand-built root, and that is a real property rather than a
    /// stale scope. Its `arg()`/`b1()` siblings are gone: they existed only to
    /// feed the hardcoded `all_books()` list that
    /// [`app_loaded_books`] replaced.
    fn crb() -> BookCorpusRoot<'static> {
        BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") }
    }

    /// The books the shipped app really loads, read out of its own
    /// `RACE_CORPUS_BOOKS` declaration.
    ///
    /// **This used to be the hardcoded list `[crb(), b1(), arg()]`, and that
    /// is why the defect below shipped.** Every assertion in this module that
    /// says "for every alternate in the corpus" was silently scoped to three
    /// books, so when SD-29's race-trait pilot ingested a fourth
    /// (`monster_codex`), the flag table beneath went on claiming complete
    /// coverage of a corpus it no longer covered — and four alternates
    /// reached the player's picker that `pilot_compute` then refused with a
    /// claim-blocking `race.alternate_trait.unknown`. The pilot had already
    /// found and fixed the identical stale-root bug one file over
    /// (`tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs`, SD-29
    /// `progress.md`); this instance survived because nothing pointed the
    /// same question at this module.
    fn app_loaded_books() -> Vec<String> {
        let src = std::fs::read_to_string("apps/desktop/src-tauri/src/race_catalog.rs")
            .expect("the desktop race catalog source is readable from the repo root");
        let decl = src
            .split("pub(crate) const RACE_CORPUS_BOOKS: &[&str] =")
            .nth(1)
            .expect("RACE_CORPUS_BOOKS is declared in race_catalog.rs");
        let list = decl.split(';').next().expect("the declaration terminates");
        list.split('"').skip(1).step_by(2).map(str::to_owned).collect()
    }

    fn all_books() -> RaceCorpus {
        let books = app_loaded_books();
        let dirs: Vec<(String, PathBuf)> = books
            .into_iter()
            .map(|book| {
                let dir = PathBuf::from("data/corpus").join(&book);
                (book, dir)
            })
            .collect();
        let roots: Vec<BookCorpusRoot<'_>> = dirs
            .iter()
            .map(|(book, dir)| BookCorpusRoot { book_id: book.as_str(), dir: dir.as_path() })
            .collect();
        let corpus = load_race_corpus(&roots);
        assert!(corpus.diagnostics().is_empty(), "clean load expected: {:?}", corpus.diagnostics());
        corpus
    }

    /// **No alternate reaches the player's menu that the engine then refuses.**
    ///
    /// `race_trait_picker` offers every [`TraitRole::Alternate`] record in the
    /// loaded corpus, and `pilot_compute::explain_selected_alternate_racial_traits`
    /// raises a **claim-blocking** `race.alternate_trait.unknown` for any
    /// selection [`ALTERNATE_TRAIT_REPLACE_FLAGS`] does not know. A record in
    /// the first set and absent from the second is therefore an affordance
    /// that looks live and is not — the no-stub doctrine's exact shape.
    ///
    /// This is the invariant that
    /// [`the_alternate_trait_flag_table_matches_the_corpus_for_every_alternate`]
    /// was believed to enforce and did not, because both it and this module's
    /// corpus loader were pinned to three books while the app loaded five.
    #[test]
    fn every_alternate_the_app_offers_is_one_the_engine_can_place() {
        let corpus = all_books();
        let placeable: BTreeSet<&str> = selectable_alternate_trait_keys().into_iter().collect();
        let mut offered_but_refused: Vec<(String, String)> = Vec::new();
        for race_key in corpus.race_keys() {
            for record in corpus.alternate_traits(race_key) {
                if !placeable.contains(record.data.key.as_str()) {
                    offered_but_refused
                        .push((record.book_id.clone(), record.data.key.clone()));
                }
            }
        }
        assert!(
            offered_but_refused.is_empty(),
            "these alternates are offered by race_trait_picker and then refused by \
             pilot_compute with a claim-blocking race.alternate_trait.unknown: \
             {offered_but_refused:?}"
        );
    }

    /// Every in-scope race's chassis loads through the real typed
    /// `CorpusRecordV1<RaceCacheData>` (not an untyped `Value` probe) and
    /// license-validates. 18 races per `decisions.md §25.3`.
    #[test]
    fn all_eighteen_in_scope_races_load_from_the_real_on_disk_corpus() {
        let corpus = all_books();
        assert_eq!(corpus.race_keys().len(), 18, "18 in-scope races: CRB 7 + Bestiary 1's 11");
        assert_eq!(corpus.chassis("Dwarf").expect("Dwarf").book_id, "core_rulebook");
        assert_eq!(corpus.chassis("Tengu").expect("Tengu").book_id, "beastiary");
        // ARG contributes traits, never a race chassis (decisions.md §25.2:
        // ARG declares zero races of its own).
        assert!(
            corpus.chassis.values().all(|c| c.book_id != "advanced_race_guide"),
            "ARG must contribute no race chassis"
        );
    }

    /// A book root that does not exist contributes nothing and does not panic.
    #[test]
    fn a_nonexistent_book_dir_contributes_nothing_without_panicking() {
        let roots = [BookCorpusRoot { book_id: "nope", dir: Path::new("data/corpus/nope") }];
        let corpus = load_race_corpus(&roots);
        assert!(corpus.race_keys().is_empty());
        assert!(corpus.diagnostics().is_empty());
    }

    /// The unmodified default race: every racial default applies, nothing is
    /// suppressed, no flag fires.
    #[test]
    fn with_no_selection_every_racial_default_applies_and_nothing_is_suppressed() {
        let corpus = all_books();
        let dwarf = corpus.resolve("Dwarf", &[]).expect("Dwarf resolves");
        assert_eq!(dwarf.traits.len(), 12, "Dwarf's 12 CRB racial defaults");
        assert!(dwarf.traits.iter().all(|t| t.role == TraitRole::Default));
        assert!(dwarf.suppressions.is_empty());
        assert!(dwarf.fired_flags.is_empty());
        assert_eq!(dwarf.size, Some(SizeCategory::Medium));
        assert_eq!(dwarf.walk_speed_ft, Some(20));
    }

    /// The protocol itself, end to end on one real swap: selecting ARG's
    /// `Dwarf ~ Ancient Enmity` fires `Dwarf_ReplaceHatred`, which removes the
    /// CRB `Hatred` standard trait and nothing else.
    #[test]
    fn a_selected_alternate_suppresses_exactly_the_standard_trait_its_flag_names() {
        let corpus = all_books();
        let base = corpus.resolve("Dwarf", &[]).expect("Dwarf resolves");
        assert!(base.traits.iter().any(|t| t.name == "Hatred"), "Hatred is a Dwarf default");

        let swapped = corpus.resolve("Dwarf", &["Dwarf ~ Ancient Enmity"]).expect("Dwarf resolves");
        assert!(!swapped.traits.iter().any(|t| t.key == "Dwarf ~ Hatred"), "Hatred must be suppressed");
        assert!(
            swapped.traits.iter().any(|t| t.key == "Dwarf ~ Ancient Enmity"),
            "the chosen alternate must apply"
        );
        assert_eq!(swapped.fired_flags, vec!["Dwarf_ReplaceHatred".to_string()]);
        assert_eq!(
            swapped.suppressions,
            vec![Suppression {
                suppressed_trait_key: "Dwarf ~ Hatred".to_string(),
                flag: "Dwarf_ReplaceHatred".to_string(),
                set_by_trait_key: "Dwarf ~ Ancient Enmity".to_string(),
            }]
        );
        assert!(swapped.inert_flags.is_empty());
        // Net effect: one out, one in.
        assert_eq!(swapped.traits.len(), base.traits.len());
    }

    /// `FlagGranted` replacement content, which is the half a naive
    /// "suppress and stop" implementation loses. ARG's Saltbeard sets four
    /// flags at once; one of them, `Dwarf_ReplaceGreed`, also *grants*
    /// `Saltbeard ~ Dwarf ~ Greed` — a positive `PREFACT` row nobody selects
    /// directly.
    #[test]
    fn a_flag_that_grants_replacement_content_brings_it_in_without_being_selected() {
        let corpus = all_books();
        let saltbeard = corpus.resolve("Dwarf", &["Dwarf ~ Saltbeard"]).expect("Dwarf resolves");
        assert_eq!(
            saltbeard.fired_flags,
            vec![
                "Dwarf_ReplaceDefensiveTraining".to_string(),
                "Dwarf_ReplaceGreed".to_string(),
                "Dwarf_ReplaceHatred".to_string(),
                "Dwarf_ReplaceStonecunning".to_string(),
            ]
        );
        let suppressed: Vec<&str> =
            saltbeard.suppressions.iter().map(|s| s.suppressed_trait_key.as_str()).collect();
        assert_eq!(
            suppressed,
            vec!["Dwarf ~ Defensive Training", "Dwarf ~ Greed", "Dwarf ~ Hatred", "Dwarf ~ Stonecunning"]
        );
        // The seagoing Greed replaces the suppressed one, un-selected.
        let granted = saltbeard
            .traits
            .iter()
            .find(|t| t.key == "Saltbeard ~ Dwarf ~ Greed")
            .expect("the flag-granted replacement Greed must apply");
        assert_eq!(granted.role, TraitRole::FlagGranted);
        assert!(granted
            .description
            .as_deref()
            .unwrap_or_default()
            .contains("under the water"));
        // And it is absent when Saltbeard is not chosen.
        let base = corpus.resolve("Dwarf", &[]).expect("resolves");
        assert!(!base.traits.iter().any(|t| t.key == "Saltbeard ~ Dwarf ~ Greed"));
        assert!(base.traits.iter().any(|t| t.key == "Dwarf ~ Greed"));
    }

    /// Loading CRB alone — without ARG — still resolves the default race
    /// correctly. The resolver is book-agnostic in both directions: it does
    /// not require the alternate-trait book to be present.
    #[test]
    fn a_race_resolves_from_its_own_book_alone_without_the_alternate_trait_book() {
        let roots = [crb()];
        let corpus = load_race_corpus(&roots);
        let human = corpus.resolve("Human", &[]).expect("Human resolves");
        assert_eq!(human.traits.len(), 6);
        assert!(corpus.alternate_traits("Human").is_empty(), "no ARG loaded, so no alternates");
        // ...and a selection that cannot be matched is reported, not ignored.
        let with_bad = corpus.resolve("Human", &["Human ~ Heart of the Fields"]).expect("resolves");
        assert_eq!(with_bad.unmatched_selections, vec!["Human ~ Heart of the Fields".to_string()]);
        assert_eq!(with_bad.traits.len(), 6, "an unmatched selection changes nothing");
    }

    /// Goblin and Hobgoblin chassis rows carry `MOVE:Walk,0`; their real
    /// 30 ft. lives only on their `Normal Speed` trait. The resolver must let
    /// the trait win, and must say which one won.
    #[test]
    fn a_speed_trait_overrides_a_chassis_that_declares_zero_walk_speed() {
        let corpus = all_books();
        for race in ["Goblin", "Hobgoblin"] {
            let resolved = corpus.resolve(race, &[]).expect("resolves");
            assert_eq!(resolved.chassis_walk_speed_ft, Some(0), "{race} chassis really says 0");
            assert_eq!(resolved.walk_speed_ft, Some(30), "{race} real speed comes off its Speed trait");
            assert_eq!(resolved.speed_source, SpeedSource::Trait(format!("{race} ~ Speed")));
        }
        // And an alternate that replaces the speed slot changes it again.
        let bandy = corpus.resolve("Hobgoblin", &["Hobgoblin ~ Bandy-Legged"]).expect("resolves");
        assert_eq!(bandy.walk_speed_ft, Some(20));
        assert_eq!(bandy.speed_source, SpeedSource::Trait("Hobgoblin ~ Bandy-Legged".to_string()));
        assert!(!bandy.traits.iter().any(|t| t.key == "Hobgoblin ~ Speed"), "the 30 ft. trait is suppressed");
    }

    /// A race whose chassis declares a real speed and whose speed trait agrees
    /// still reports the trait as the source — the override is unconditional,
    /// not a "only when the chassis looks wrong" special case.
    #[test]
    fn the_speed_trait_is_the_source_even_when_it_agrees_with_the_chassis() {
        let corpus = all_books();
        let dwarf = corpus.resolve("Dwarf", &[]).expect("resolves");
        assert_eq!(dwarf.chassis_walk_speed_ft, Some(20));
        assert_eq!(dwarf.walk_speed_ft, Some(20));
        assert_eq!(dwarf.speed_source, SpeedSource::Trait("Dwarf ~ Speed".to_string()));
    }

    /// **This used to assert two.** `Feral ~ Languages` and
    /// `Scion of Humanity ~ Languages` were the corpus's only ungated rows
    /// until the `ABILITY:<category>|AUTOMATIC|<key>` grant shape was read
    /// (see [`link_automatic_grants`]); both are now
    /// [`TraitRole::FlagGranted`], granted by the alternate that names them.
    ///
    /// The test is kept, and kept asserting emptiness in both directions,
    /// because its job never was the number 2: it is the residue check. A row
    /// the ingest produces that no gate in this module can read must show up
    /// here rather than be silently dropped.
    #[test]
    fn no_corpus_trait_is_left_without_a_readable_gate() {
        let corpus = all_books();
        let unclassified: Vec<(&str, &str)> = corpus
            .unclassified_traits()
            .iter()
            .map(|t| (t.data.race_key.as_str(), t.data.key.as_str()))
            .collect();
        // Pinned by exact key, in both directions: a SECOND unclassified row
        // fails here, and so does this one disappearing. `Oversized Goblin`
        // (Monster Codex, `mc_abilities_race.lst:31`) carries no readable
        // gate at all -- upstream it is picked out of a
        // `BONUS:ABILITYPOOL|Goblin Variant|1` pool, a mechanism this engine
        // does not model -- so it is exactly the residue this test exists to
        // surface. It has its own `OPEN_FINDINGS`/`UNREACHED_RECORD_FINDINGS`
        // entries in `reach_gate` naming that remedy (SD-29 `decisions.md
        // §43`), and it is the one alternate deliberately absent from
        // `ALTERNATE_TRAIT_REPLACE_FLAGS`.
        //
        // `Human ~ Tribalistic Languages` (Inner Sea Races,
        // `isr_abilities_race.lst:216`, SD-29 race-trait lane round 2) is the
        // second, and it is the same *kind* of residue for a different upstream
        // reason: **nothing upstream grants it at all.** Its own row carries no
        // `PREFACT`, no `PREABILITY` and no `!PREFACT`, and no other row in the
        // book names it --
        // `grep -o 'ABILITY:[^\t]*Tribalistic Languages' isr_abilities_race.lst`
        // returns nothing, where the same grep for `Junk Tinker ~ Skilled` one
        // row-family over returns its granter and that row is therefore
        // `FlagGranted`. The alternate that logically owns it,
        // `Human ~ Tribalistic` (`:210`), only fires `Human_ReplaceLanguages`,
        // which *suppresses* the standard `Human ~ Languages` row without
        // bringing this replacement in. So the record is ingested, visible in
        // the corpus, and never applies -- an upstream data gap this engine
        // reports rather than papers over. `reach_gate`'s `OPEN_FINDINGS` names
        // the remedy.
        assert_eq!(
            unclassified,
            vec![("Human", "Human ~ Tribalistic Languages"), ("Goblin", "Oversized Goblin")]
        );

        // And the two that used to live here still do not auto-apply: they
        // arrive only through the alternate that grants them.
        for (race, key, granter) in [
            ("Orc", "Feral ~ Languages", "Orc ~ Feral"),
            ("Aasimar", "Scion of Humanity ~ Languages", "Aasimar ~ Scion of Humanity"),
        ] {
            let plain = corpus.resolve(race, &[]).expect("resolves");
            assert!(!plain.traits.iter().any(|t| t.key == key), "{key} must not auto-apply");
            let chosen = corpus.resolve(race, &[granter]).expect("resolves");
            assert!(
                chosen.traits.iter().any(|t| t.key == key),
                "{key} must arrive with {granter}"
            );
        }
    }

    /// **A PI-redacted record must never render the prose it was redacted for.**
    ///
    /// SD-29 race-trait lane round 3 (`decisions.md §47`). The ingest screens
    /// `data.description` and stores the marker, but the record's `DESC:` raw
    /// tokens keep the upstream prose verbatim, and `render_description` reads
    /// those tokens — so between round 2 and round 3 the Race Traits panel was
    /// rendering the exact Golarion place and nation names the screen had
    /// removed, for every one of Inner Sea Races' 12 redacted records.
    ///
    /// This asserts the property over the real corpus in both directions: a
    /// redacted record serves the marker, and there is at least one such record
    /// so the test cannot pass by finding nothing. The `!= stored` form is what
    /// the defect looked like from `race_trait_picker`'s own gate; this states
    /// it as the PI property it actually is.
    #[test]
    fn a_pi_redacted_description_is_never_rendered_back_from_its_raw_desc_tokens() {
        let corpus = all_books();
        let mut redacted = 0usize;
        for race_key in corpus.race_keys() {
            for record in corpus.traits_for(race_key) {
                if !record.description_redacted {
                    continue;
                }
                redacted += 1;
                let stored = record.data.description.clone().unwrap_or_default();
                assert_eq!(
                    stored, "[redacted PI]",
                    "{}: a redacted record's stored description is the marker",
                    record.data.key
                );
                let rendered = record.render_description(&record.display_values_with(
                    &crate::rules_core::race_resolver::FeatDisplayValueDeltas::default(),
                ));
                assert_eq!(
                    rendered.text, stored,
                    "{}: rendered prose must be the marker, not the raw DESC token the screen \
                     removed",
                    record.data.key
                );
            }
        }
        assert_eq!(
            redacted, 27,
            "Inner Sea Races' 18 PI-redacted records + Core Essentials' 9, counted on disk. \
             Horror Adventures added 0: it is a rules supplement, not a campaign setting. \
             **Was 12 + 8 = 20 until 2026-08-12** (SD-29 `decisions.md §53`), when the ingest \
             path learned to read PCGen's own per-record declaration `DESCISPI:YES` alongside \
             the 55-term blacklist. The blacklist had caught 18 of the 26 declared rows by \
             coincidence -- their prose happens to name a Golarion place the list knows -- and \
             published the other 8, whose Product Identity is `Kodar Mountains`, `Earthfall`, \
             `Ekujae`, `Gogpodda`, `Omesta`, `Droskar`, `Abaddon` and `Inner Sea`. Core \
             Essentials' original 8 are four Tiefling heritages named for outsider races that \
             are Golarion Product Identity -- Kyton-, Oni-, Devil- and Rakshasa-Spawn -- each \
             hitting twice because the heritage row and its Ability Scores replacement row \
             carry the same prose; the 9th is `Tiefling ~ Daemon-Spawn`, declared and not on \
             the list. Its 24 Aasimar records hit 0 terms and declare nothing"
        );
    }

    /// Role classification over the whole corpus, as a derived census. These
    /// numbers come from the loader itself; if the ingest changes shape, this
    /// is where it shows up.
    #[test]
    fn the_whole_corpus_classifies_into_the_four_roles_with_no_leftovers() {
        let corpus = all_books();
        let count = |role: TraitRole| corpus.traits.values().flatten().filter(|t| t.role == role).count();
        assert_eq!(count(TraitRole::Default), 173);
        // 153 ARG + Monster Codex's 4 + the Advanced Player's Guide's 1
        // (`Half-Orc ~ Plagueborn`) + Inner Sea Races' 67 + Horror
        // Adventures' 41, all landed by SD-29's race-trait lane.
        assert_eq!(count(TraitRole::Alternate), 282);
        // 5 + Inner Sea Races' 3: `Junk Tinker ~ Skilled` (named by an
        // `ABILITY:Goblin Racial Trait|AUTOMATIC|` grant) and the two rows
        // carrying a positive `PREFACT` gate, `Secret Magic ~ Merfolk ~ Speed`
        // and `Pass for Human ~ Tiefling ~ Languages`.
        //
        // + Horror Adventures' 2: `Deep Jungle Halfling ~ Languages` and
        // `Deep Jungle Halfling ~ Poison Use`, both named by an
        // `ABILITY:Halfling Racial Trait|AUTOMATIC|` grant on
        // `Halfling ~ Deep Jungle` -- the `Junk Tinker ~ Skilled` shape
        // exactly. That this count moved and `TraitRole::Unclassified` below
        // did NOT is the whole evidence that round 3's book shipped no
        // unreachable record.
        //
        // + Core Essentials' 48 (round 4): every replacement row of every
        // Aasimar and Tiefling heritage, each named by an
        // `ABILITY:<Race> Racial Trait|AUTOMATIC|` grant on the heritage that
        // supplies it -- the `Junk Tinker ~ Skilled` shape again, at scale.
        // This is the first book whose contribution to this census is larger
        // than its contribution to `Alternate` above (48 against 16), which is
        // the whole shape of a heritage: one thing a player picks, three
        // things they get.
        assert_eq!(count(TraitRole::FlagGranted), 58);
        // `Oversized Goblin` and `Human ~ Tribalistic Languages` -- see
        // `no_corpus_trait_is_left_without_a_readable_gate`, which pins both by
        // key and names each one's remedy.
        assert_eq!(count(TraitRole::Unclassified), 2);
        assert_eq!(
            corpus.traits.values().flatten().count(),
            515,
            "175 standard + 156 ARG + 5 Monster Codex + 1 APG + 71 Inner Sea Races \
             + 43 Horror Adventures + 64 Core Essentials heritage records (16 heritages \
             + the 48 replacement rows they grant)"
        );
    }

    /// **The corpus gap that was here is closed. This is what replaced it.**
    ///
    /// Every replace-flag an alternate fires must be claimed by some trait in
    /// the loaded books — otherwise the alternate applies, the standard trait
    /// it is supposed to replace stays, and the character silently gets both.
    /// Six flags were unclaimed until 2026-07-31, and the reason was never a
    /// bug in this resolver: those standard traits declare their gate in a
    /// *different file and a different token* than the ingest read.
    ///
    /// PCGen has two spellings of the same protocol. The common one puts the
    /// gate on the standard trait row itself in `<race>_abilities_race.lst`:
    ///
    /// ```text
    /// Greed  KEY:Dwarf ~ Greed  !PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True
    /// ```
    ///
    /// The other inverts it, in `<race>_abilities_globalvar.lst`, where a
    /// `.MOD` row *grants* the standard trait only while the variable is `0`:
    ///
    /// ```text
    /// CATEGORY=Special Ability|Aasimar ~ Default.MOD
    ///     ABILITY:Aasimar Racial Trait|AUTOMATIC|Aasimar ~ Vision|PREVAREQ:Aasimar_ReplaceVision,0
    /// ```
    ///
    /// Aasimar's 9 standard trait rows carry no `!PREFACT` at all, so all five
    /// Aasimar flags were unclaimed and its 9 ARG alternates were an
    /// affordance a player could tick and never use — `create_character`
    /// refused every one of them on `inert_flags`. `src/bin/ingest_races.rs`
    /// now reads the globalvar gate wherever the trait row declares none, and
    /// cross-checks the two sources on the 166 rows where both speak
    /// (`tests/sd27_aasimar_globalvar_gate_closes_the_dead_affordance.rs`).
    ///
    /// **One flag remains, and it is a different defect.**
    /// `Duergar_ReplaceSLAInvisibility` is declared by the corpus — its row's
    /// gate names three flags and the single-valued
    /// `RaceTraitCacheData::suppressed_by_flag` holds only the first. That is a
    /// schema limit, not a missing file; it is reported by
    /// `race_trait_picker::multi_flag_gate_findings`; and the alternate that
    /// fires it is not dead, because the flag grants
    /// `Duergar ~ Spell-Like Ability ~ Enlarge Person`.
    #[test]
    fn the_one_remaining_unclaimed_flag_is_a_schema_limit_not_a_missing_file() {
        let corpus = all_books();
        let claimed: BTreeSet<&str> = corpus
            .traits
            .values()
            .flatten()
            .filter_map(|t| t.data.suppressed_by_flag.as_deref().or(t.requires_flag.as_deref()))
            .collect();
        let mut orphan_flags: BTreeSet<&str> = BTreeSet::new();
        let mut affected_races: BTreeSet<&str> = BTreeSet::new();
        let mut affected_alternates: BTreeSet<&str> = BTreeSet::new();
        for record in corpus.traits.values().flatten() {
            for flag in &record.data.sets_replace_flags {
                if !claimed.contains(flag.as_str()) {
                    orphan_flags.insert(flag.as_str());
                    affected_races.insert(record.data.race_key.as_str());
                    affected_alternates.insert(record.data.key.as_str());
                }
            }
        }
        // Two flags, one cause. SD-29's race-trait lane added the second:
        // Monster Codex's `Duergar ~ Ironskinned` fires
        // `Duergar_ReplaceSLAEnlargePerson`, whose counterpart
        // `Duergar ~ Spell-Like Ability ~ Invisibility` is the OTHER end of
        // the very same truncated multi-flag gate — its row names three flags
        // and `suppressed_by_flag` keeps only the first. Identical schema
        // limit, identical proof below that neither is a dead affordance.
        assert_eq!(
            orphan_flags.iter().copied().collect::<Vec<_>>(),
            vec!["Duergar_ReplaceSLAEnlargePerson", "Duergar_ReplaceSLAInvisibility"],
            "the Aasimar five are closed; a new orphan flag is a new defect"
        );
        assert_eq!(affected_races.iter().copied().collect::<Vec<_>>(), vec!["Duergar"]);
        // Inner Sea Races' `Duergar ~ Magical Taskmaster` (SD-29 race-trait
        // lane, round 2) is the third alternate naming this same flag, and it
        // adds NO new orphan flag — the assertion above is unchanged, which is
        // the evidence that this is the same schema limit seen from one more
        // angle rather than a new defect. It gets the same grant-proof below,
        // not an exemption.
        assert_eq!(
            affected_alternates.iter().copied().collect::<Vec<_>>(),
            vec![
                "Duergar ~ Blood Enmity",
                "Duergar ~ Ironskinned",
                "Duergar ~ Magical Taskmaster",
                "Duergar ~ Twilight-Touched"
            ]
        );
        // ...and none is a dead affordance, because both flags grant.
        // `Twilight-Touched` fires the same `Duergar_ReplaceSLAInvisibility`
        // `Blood Enmity` does; two alternates may name one flag.
        for alternate in [
            "Duergar ~ Blood Enmity",
            "Duergar ~ Ironskinned",
            "Duergar ~ Magical Taskmaster",
            "Duergar ~ Twilight-Touched",
        ] {
            let resolved = corpus.resolve("Duergar", &[alternate]).expect("resolves");
            assert!(resolved.inert_flags.is_empty(), "{alternate}: {:?}", resolved.inert_flags);
        }
        // The bound that matters for the SD-27 mandatory guard.
        for race in ["Human", "Dwarf", "Elf", "Gnome", "Half-Elf", "Half-Orc", "Halfling"] {
            assert!(!affected_races.contains(race), "no CRB race may be affected, but {race} is");
        }
        // Total flags in play, derived: 90 distinct, 2 of them unclaimed.
        // 74 + Monster Codex's `Duergar_ReplaceSLAEnlargePerson`,
        // `Goblin_ReplaceAbilityScores`, `Goblin_ReplaceSize`. APG's
        // `Half-Orc ~ Plagueborn` fires two flags that ARG already declares,
        // so it adds a row without adding a flag.
        //
        // Inner Sea Races' 67 alternates add **13** distinct flags to that 77,
        // not 68: the great majority of them replace standard traits ARG's
        // alternates already replace (`Dwarf_ReplaceHatred`,
        // `Elf_ReplaceElvenMagic`, ...), which is what a second book of
        // alternates for the same 18 races should look like. The 13 are the
        // standard traits no previously-ingested alternate had ever replaced.
        // Horror Adventures' 41 alternates add exactly **1** distinct flag to
        // that 90 -- `Halfling_ReplaceLanguages`, fired by
        // `Halfling ~ Deep Jungle`. Every other flag its rows fire was already
        // declared by an ARG or ISR alternate replacing the same standard
        // trait, which is the same shape ISR showed one round earlier and the
        // reason a book's alternate count is a poor predictor of its flag
        // count. Re-derived on the written tree rather than reasoned about:
        // 29 distinct flags across this book's records, 28 of them already
        // present.
        //
        // That every one of the 91 except the two named above is *claimed* by a
        // real standard row is the assertion above, and it did not move --
        // `Halfling_ReplaceLanguages` is claimed by the standard
        // `Halfling ~ Languages` row that `Halfling ~ Deep Jungle` replaces.
        let all_flags: BTreeSet<&str> = corpus
            .traits
            .values()
            .flatten()
            .flat_map(|t| t.data.sets_replace_flags.iter().map(String::as_str))
            .collect();
        // Round 4 moved this 91 -> 93. Core Essentials' 16 heritages name six
        // distinct flags between them and only TWO are new to the corpus:
        // `Aasimar_ReplaceAbilityScores` and `Tiefling_ReplaceAbilityScores`.
        // No alternate in any earlier book replaces a race's ability-score
        // row, because an ordinary alternate racial trait never touches
        // ability scores and a heritage always does -- so the other four
        // (`*_ReplaceSkilled`, `*_ReplaceSpellLikeAbility`) were already
        // declared by ARG and ISR alternates replacing the same standard rows.
        // Both new flags are claimed by a real standard row
        // (`Aasimar ~ Ability Scores`, `Tiefling ~ Ability Scores`), which is
        // why the orphan-flag assertion above did not move.
        assert_eq!(all_flags.len(), 93);
    }

    /// **No alternate in the loaded corpus fires an inert flag any more.**
    ///
    /// This is the property that makes the picker's menu honest, because
    /// `character_hub::resolve_alternate_trait_choices` refuses to save a
    /// character on exactly this condition. Asserted over all 153 alternates,
    /// not over the nine that happened to be broken.
    #[test]
    fn no_alternate_the_picker_offers_fires_a_flag_that_suppresses_and_grants_nothing() {
        let corpus = all_books();
        let mut checked = 0usize;
        for race_key in corpus.race_keys() {
            for record in corpus.alternate_traits(race_key) {
                let key = record.data.key.clone();
                let resolved = corpus.resolve(race_key, &[key.as_str()]).expect("resolves");
                assert!(resolved.inert_flags.is_empty(), "{key}: {:?}", resolved.inert_flags);
                checked += 1;
            }
        }
        assert_eq!(
            checked,
            282,
            "153 ARG + 4 Monster Codex + 1 APG + 67 Inner Sea Races + 41 Horror Adventures. \
             **282, not 283, since 2026-08-12** (SD-29 `decisions.md` 53): Inner Sea Races' \
             `Elf ~ Sovyrian-Born` carries `NAMEISPI:YES`, PCGen's own declaration that the \
             record NAME is Product Identity, and a name cannot be redacted -- so the row is \
             dropped, not screened."
        );
    }

    /// The runtime machinery that reports an unmatched swap is still under
    /// test even though the real corpus no longer contains one.
    ///
    /// Driven against a synthetic two-record corpus written to a temp dir —
    /// the same technique
    /// [`a_malformed_record_produces_a_diagnostic_instead_of_taking_down_the_load`]
    /// uses — because the alternative is deleting the test along with the
    /// defect, and then nothing proves the resolver still *says so* the next
    /// time a book arrives with a gate nobody ingested.
    #[test]
    fn a_swap_with_no_counterpart_is_reported_as_an_inert_flag_not_silently_dropped() {
        let dir = std::env::temp_dir().join(format!("codex_inert_flag_{}", std::process::id()));
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(dir.join("race")).expect("temp dir");
        fs::create_dir_all(dir.join("race_trait")).expect("temp dir");

        let source = r#""source":{"kind":"lst_token","path":"synthetic.lst","sha256":"0","line":1,"record_key":"x"}"#;
        fs::write(
            dir.join("race/testrace.json"),
            format!(
                r#"{{"population":"in_scope","completeness":"chassis_only","ingested_at":"t","data":{{"key":"Testrace","name":"Testrace","base_size":null,"base_move_walk":30,"race_type":null,"type_tokens":[],"legs":2,"hands":2}},{source},"license":"OGL"}}"#
            ),
        )
        .expect("write");
        // A standard trait with NO gate, and an alternate that fires a flag
        // naming it. This is precisely the Aasimar shape as it was on disk
        // before the globalvar file was ingested.
        for (slug, body) in [
            (
                "standard",
                r#""key":"Testrace ~ Vision","name":"Vision","race_key":"Testrace","type_tokens":["Testrace Racial Default"],"is_racial_default":true,"suppressed_by_flag":null,"sets_replace_flags":[]"#,
            ),
            (
                "alternate",
                r#""key":"Testrace ~ Halo","name":"Halo","race_key":"Testrace","type_tokens":["Testrace Racial Trait"],"is_racial_default":false,"suppressed_by_flag":null,"sets_replace_flags":["Testrace_ReplaceVision"]"#,
            ),
        ] {
            fs::write(
                dir.join(format!("race_trait/{slug}.json")),
                format!(
                    r#"{{"population":"in_scope","completeness":"full","ingested_at":"t","data":{{{body}}},{source},"license":"OGL"}}"#
                ),
            )
            .expect("write");
        }

        let roots = [BookCorpusRoot { book_id: "synthetic", dir: &dir }];
        let corpus = load_race_corpus(&roots);
        assert!(corpus.diagnostics().is_empty(), "{:?}", corpus.diagnostics());

        let halo = corpus.resolve("Testrace", &["Testrace ~ Halo"]).expect("Testrace resolves");
        assert!(halo.traits.iter().any(|t| t.key == "Testrace ~ Halo"), "the alternate applies");
        assert_eq!(halo.fired_flags, vec!["Testrace_ReplaceVision".to_string()]);
        assert_eq!(
            halo.inert_flags,
            vec!["Testrace_ReplaceVision".to_string()],
            "the flag fired but suppressed nothing — reported, not hidden"
        );
        assert!(halo.suppressions.is_empty());
        // The un-suppressed standard trait is still there, which is exactly
        // what `inert_flags` is warning about.
        assert!(halo.traits.iter().any(|t| t.key == "Testrace ~ Vision"));
        fs::remove_dir_all(&dir).ok();

        // Contrast, against the real corpus: a swap with a real counterpart
        // reports no inert flag — including Aasimar's, which is what this
        // cycle changed.
        let corpus = all_books();
        assert!(corpus.resolve("Dwarf", &["Dwarf ~ Ancient Enmity"]).expect("resolves").inert_flags.is_empty());
        assert!(corpus.resolve("Aasimar", &["Aasimar ~ Halo"]).expect("resolves").inert_flags.is_empty());
    }

    /// Selecting every alternate a race offers at once is not a realistic
    /// character, but it is the maximal stress on the protocol: it must not
    /// panic, must not duplicate a trait, and must leave every selection
    /// matched.
    #[test]
    fn selecting_every_alternate_at_once_stays_consistent_for_all_eighteen_races() {
        let corpus = all_books();
        for race in corpus.race_keys() {
            let keys: Vec<String> =
                corpus.alternate_traits(race).iter().map(|t| t.data.key.clone()).collect();
            let refs: Vec<&str> = keys.iter().map(String::as_str).collect();
            let resolved = corpus.resolve(race, &refs).expect("resolves");
            assert!(resolved.unmatched_selections.is_empty(), "{race}: {:?}", resolved.unmatched_selections);
            let mut seen: BTreeSet<&str> = BTreeSet::new();
            for t in &resolved.traits {
                assert!(seen.insert(t.key.as_str()), "{race}: duplicate resolved trait {}", t.key);
            }
            // Every chosen alternate survives; every flag it fired is real.
            for key in &refs {
                assert!(resolved.traits.iter().any(|t| &t.key == key), "{race}: {key} must apply");
            }
        }
    }

    /// `resolve_key` accepts the shapes real call sites already have.
    #[test]
    fn resolve_key_matches_bare_keys_input_tokens_and_case_variants() {
        let corpus = all_books();
        assert_eq!(corpus.resolve_key("Half-Elf"), Some("Half-Elf"));
        assert_eq!(corpus.resolve_key("race:half-elf"), Some("Half-Elf"));
        assert_eq!(corpus.resolve_key("  race:HALF-ORC "), Some("Half-Orc"));
        assert_eq!(corpus.resolve_key("race:tiefling"), Some("Tiefling"));
        assert_eq!(corpus.resolve_key("race:dhampir"), None, "a B2 race is not ingested");
        assert_eq!(corpus.resolve_key(""), None);
    }

    /// A malformed corpus file becomes a diagnostic, not a panic and not a
    /// silent skip. Written to a temp dir so no real corpus file is touched.
    #[test]
    fn a_malformed_record_produces_a_diagnostic_instead_of_taking_down_the_load() {
        let dir = std::env::temp_dir().join(format!("codex_race_resolver_{}", std::process::id()));
        let race_dir = dir.join("race");
        fs::create_dir_all(&race_dir).expect("temp dir");
        fs::write(race_dir.join("broken.json"), "{ not json").expect("write");
        // ...and a well-formed-JSON-but-wrong-shape record.
        fs::write(race_dir.join("wrong_shape.json"), r#"{"population":"in_scope"}"#).expect("write");
        let roots = [BookCorpusRoot { book_id: "temp", dir: &dir }];
        let corpus = load_race_corpus(&roots);
        assert_eq!(corpus.diagnostics().len(), 2, "{:?}", corpus.diagnostics());
        assert!(corpus.race_keys().is_empty());
        fs::remove_dir_all(&dir).ok();
    }

    /// The size half of the "chassis row is not the whole truth" pattern
    /// this module already handles for speed. Aasimar's and Tiefling's
    /// chassis rows carry `FACT:BaseSize|S`; both races are Medium, and the
    /// declaration that says so is `TEMPLATE:SIZE_M` on their own
    /// `~ Size` racial-default trait.
    #[test]
    fn a_size_trait_template_overrides_a_chassis_base_size_that_disagrees() {
        let corpus = all_books();
        for race in ["Aasimar", "Tiefling"] {
            let resolved = corpus.resolve(race, &[]).expect("resolves");
            assert_eq!(resolved.chassis_size, Some(SizeCategory::Small), "{race} chassis really says S");
            assert_eq!(resolved.size, Some(SizeCategory::Medium), "{race} is Medium");
            assert_eq!(resolved.size_source, SizeSource::Trait(format!("{race} ~ Size")));
        }
        // Human's `~ Size` row carries no template at all, so its chassis
        // `FACT:BaseSize|M` is the declaration -- the chassis is still a
        // real source, not a discarded one.
        let human = corpus.resolve("Human", &[]).expect("resolves");
        assert_eq!(human.size, Some(SizeCategory::Medium));
        assert_eq!(human.size_source, SizeSource::Chassis);
    }

    /// `TEMPLATE:` is a busy token; only the nine `SIZE_<code>` rows of
    /// `ce_templates.lst` may be read as a size, and `SIZE_C+` (body
    /// `SIZE:P`) is not one of them.
    #[test]
    fn only_a_real_size_template_token_is_read_as_a_size() {
        assert_eq!(size_from_size_template("SIZE_M"), Some(SizeCategory::Medium));
        assert_eq!(size_from_size_template("SIZE_S"), Some(SizeCategory::Small));
        assert_eq!(size_from_size_template("SIZE_C"), Some(SizeCategory::Colossal));
        assert_eq!(size_from_size_template("SIZE_C+"), None, "its body is SIZE:P, not a modelled code");
        assert_eq!(size_from_size_template("Dragon Size Tracker"), None);
        assert_eq!(size_from_size_template("SIZE_"), None);
        assert_eq!(size_from_size_template("Half-Orc Language Template"), None);
        assert_eq!(size_from_size_template(""), None);
    }

    /// The hand-modelled token table and the corpus must agree for every
    /// race, or one of them is lying. `decisions.md §24` allows the table;
    /// this is the verification that keeps it honest.
    #[test]
    fn the_hand_modelled_race_size_table_matches_the_corpus_for_all_eighteen_races() {
        let corpus = all_books();
        assert_eq!(RACE_SIZES.len(), 18);
        for key in corpus.race_keys() {
            let resolved = corpus.resolve(key, &[]).expect("resolves");
            assert_eq!(
                race_size_for_race_token(key),
                resolved.size,
                "{key}: RACE_SIZES disagrees with the corpus"
            );
        }
        // The token forms real character inputs actually carry.
        assert_eq!(race_size_for_race_token("race:goblin"), Some(SizeCategory::Small));
        assert_eq!(race_size_for_race_token("race:half-elf"), Some(SizeCategory::Medium));
        assert_eq!(race_size_for_race_token("race:tiefling"), Some(SizeCategory::Medium));
        // A race outside the ingested 18 stays an honest absence.
        assert_eq!(race_size_for_race_token("race:dhampir"), None);
        assert_eq!(race_size_for_race_token(""), None);
    }

    /// The hand-modelled alternate-trait flag table and the corpus must agree
    /// for every one of the 153 selectable alternates, in both directions.
    /// `decisions.md §24` allows the table; this is what keeps it honest, the
    /// same way `the_hand_modelled_race_size_table_matches_the_corpus...` keeps
    /// [`RACE_SIZES`] honest.
    #[test]
    fn the_alternate_trait_flag_table_matches_the_corpus_for_every_alternate() {
        let corpus = all_books();
        let mut corpus_rows: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for race_key in corpus.race_keys() {
            for record in corpus.alternate_traits(race_key) {
                corpus_rows.insert(
                    record.data.key.as_str(),
                    record.data.sets_replace_flags.iter().map(String::as_str).collect(),
                );
            }
        }
        assert_eq!(
            corpus_rows.len(),
            282,
            "153 ARG + 4 Monster Codex + 1 APG + 67 Inner Sea Races + 41 Horror Adventures \
             selectable alternates"
        );
        assert_eq!(ALTERNATE_TRAIT_REPLACE_FLAGS.len(), corpus_rows.len(), "no table row is extra or missing");
        for (key, flags) in ALTERNATE_TRAIT_REPLACE_FLAGS {
            let from_corpus = corpus_rows.get(key).unwrap_or_else(|| panic!("{key} is a real alternate"));
            assert_eq!(&flags.to_vec(), from_corpus, "{key}: table flags disagree with the corpus");
        }
        // The three dependent rows a chosen alternate grants or drops are
        // deliberately absent — a player never selects one.
        for dependent in ["Feral ~ Languages", "Scion of Humanity ~ Languages", "Saltbeard ~ Dwarf ~ Greed"] {
            assert!(
                !ALTERNATE_TRAIT_REPLACE_FLAGS.iter().any(|(key, _)| *key == dependent),
                "{dependent} is not a standalone choice and must not be offered"
            );
        }
    }

    /// The three pure predicates the engine gates on, exercised against real
    /// corpus keys — including the "an unknown key changes nothing but is
    /// reported" posture.
    #[test]
    fn the_pure_flag_predicates_answer_the_same_question_the_disk_backed_resolver_does() {
        let corpus = all_books();
        let saltbeard = vec!["Dwarf ~ Saltbeard".to_string()];
        assert_eq!(
            replace_flags_fired_by(&saltbeard),
            corpus
                .resolve("Dwarf", &["Dwarf ~ Saltbeard"])
                .expect("resolves")
                .fired_flags
                .iter()
                .map(String::as_str)
                .collect::<Vec<&str>>()
        );
        assert!(alternate_traits_fire_flag(&saltbeard, "Dwarf_ReplaceGreed"));
        assert!(!alternate_traits_fire_flag(&saltbeard, "Dwarf_ReplaceHardy"));
        assert!(unknown_alternate_trait_keys(&saltbeard).is_empty());

        let typo = vec!["Dwarf ~ Saltbeerd".to_string()];
        assert!(replace_flags_fired_by(&typo).is_empty());
        assert_eq!(unknown_alternate_trait_keys(&typo), vec!["Dwarf ~ Saltbeerd".to_string()]);
        assert_eq!(selectable_alternate_trait_keys().len(), 282);
    }

    #[test]
    fn move_and_prefact_token_parsing_reads_the_real_token_forms() {
        assert_eq!(walk_speed_from_move("Walk,20"), Some(20));
        assert_eq!(walk_speed_from_move("Walk,15,Swim,30"), Some(15));
        assert_eq!(walk_speed_from_move("Walk,0"), Some(0));
        assert_eq!(walk_speed_from_move("Swim,50"), None, "no walk component");
        assert_eq!(first_ability_flag("1,ABILITIES,Dwarf_ReplaceGreed=True"), Some("Dwarf_ReplaceGreed".into()));
        assert_eq!(first_ability_flag("1,ABILITIES,Dwarf_ReplaceGreed=true"), Some("Dwarf_ReplaceGreed".into()));
        assert_eq!(first_ability_flag("1,SOMETHINGELSE,X=True"), None);
        assert_eq!(first_ability_flag("garbage"), None);
    }

    /// `declared_bonus_magnitudes` reads numbers, it does not interpret them.
    /// Pinned against a real record: Stonecunning's magnitude lives on its
    /// companion `BONUS:VAR` chain, not on the `BONUS:SITUATION` that names a
    /// variable.
    #[test]
    fn declared_bonus_magnitudes_reads_real_chains_including_the_indirect_var_form() {
        let corpus = all_books();
        let dwarf = corpus.resolve("Dwarf", &[]).expect("resolves");
        let stonecunning = dwarf.traits.iter().find(|t| t.name == "Stonecunning").expect("Stonecunning");
        assert_eq!(stonecunning.declared_bonus_magnitudes(), vec![2]);
        let ability = dwarf
            .traits
            .iter()
            .find(|t| t.type_tokens.iter().any(|tt| tt == "Racial Ability Scores"))
            .expect("ability scores trait");
        assert_eq!(ability.declared_bonus_magnitudes(), vec![2, -2]);
    }
}
