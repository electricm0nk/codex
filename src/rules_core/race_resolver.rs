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
///
/// **This is deliberately absolute, not merely the common case.** SD-33 Epic
/// 6 (2026-08-26) investigated widening the re-role to also cover a record
/// that is a grant target AND independently sets its own
/// `sets_replace_flags` — Skinwalker's 36 kin replacement rows are exactly
/// that shape, since each carries a genuine `FACT:Skinwalker_Replace<Trait>
/// |True` of its own, unlike every other dependent row in the corpus
/// (`Feral ~ Languages`, `Throwback ~ Gillman ~ Type/Speed`, ...), none of
/// which carries a `FACT:` and so never contended for a role in the first
/// place. **That widening was reverted**: Monster Codex's `Oversized Goblin
/// ~ Ability Scores`/`~ Size` are the pre-existing counter-example — both
/// are named by `Oversized Goblin`'s own `ABILITY:...AUTOMATIC...` grant AND
/// carry their own `FACT:Goblin_ReplaceAbilityScores`/`ReplaceSize`, and
/// both are deliberately `Alternate` today (independently offered menu
/// rows, counted in the "8 Monster Codex" figure the tests above cite) —
/// proof this corpus's real, shipped design is "a record's own flag always
/// wins," full stop, not "being granted wins when both apply." Skinwalker's
/// 36 replacement rows follow the identical rule: `Alternate`, same as
/// `Oversized Goblin`'s two, not demoted.
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
                // `pi_field` is a comma-joined list when more than one field
                // was redacted (`ingest_race_traits.rs`'s own
                // `raw_tokens`-widening idiom: `f.split(',').any(|p| p ==
                // "raw_tokens")`) -- a record whose `raw_tokens` ALSO carried
                // PI (concatenated-identifier scan hits) stores
                // `"description,raw_tokens"`, not the bare `"description"`
                // this used to require byte-for-byte. An exact-equals check
                // here silently returned `false` for such a record even
                // though its `description` field genuinely is the marker,
                // which is exactly the class of defect this field's own doc
                // comment above describes (redaction "live and ineffective"
                // on the rendered surface) -- `render_description` would
                // fall through to the `DESC:` raw-token path for a record
                // this repo's OWN generator already knows is redacted.
                description_redacted: record
                    .pi_field
                    .as_deref()
                    .is_some_and(|f| f.split(',').any(|part| part == "description"))
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

    /// Every trait record, across every loaded race, whose `CATEGORY:` token
    /// equals `category` verbatim, sorted by key.
    ///
    /// A general-purpose seam for a shape that does not fit the
    /// default/alternate/flag-granted protocol at all — see
    /// [`adoptive_parentage_options`], its one caller today.
    pub fn traits_by_category(&self, category: &str) -> Vec<&RaceTraitRecord> {
        let mut out: Vec<&RaceTraitRecord> = self
            .traits
            .values()
            .flatten()
            .filter(|t| t.data.category.as_deref() == Some(category))
            .collect();
        out.sort_by(|a, b| a.data.key.cmp(&b.data.key));
        out
    }

    /// Every trait record, across every loaded race, carrying `token` as one
    /// of its (possibly several) `TYPE:` components, sorted by key.
    ///
    /// [`traits_by_category`]'s sibling seam: the Adopted-Race selector shape
    /// (`decisions.md §25`, [`adopted_race_choose_selectors`]) shares its
    /// `CATEGORY:Special Ability` with countless ordinary standard traits, so
    /// `traits_by_category` alone cannot select it — its own `TYPE:AdoptiveRace`
    /// component is the one thing no other trait in this corpus carries.
    pub fn traits_by_type_token(&self, token: &str) -> Vec<&RaceTraitRecord> {
        let mut out: Vec<&RaceTraitRecord> = self
            .traits
            .values()
            .flatten()
            .filter(|t| t.data.type_tokens.iter().any(|tt| tt == token))
            .collect();
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
    // Bestiary 2's 6, SD-31 Epic 1-F2 (2026-08-15).
    ("Fetchling", SizeCategory::Medium),   // TEMPLATE:SIZE_M
    ("Grippli", SizeCategory::Small),      // TEMPLATE:SIZE_S
    ("Ifrit", SizeCategory::Medium),       // TEMPLATE:SIZE_M
    ("Oread", SizeCategory::Medium),       // TEMPLATE:SIZE_M
    ("Sylph", SizeCategory::Medium),       // TEMPLATE:SIZE_M
    ("Undine", SizeCategory::Medium),      // TEMPLATE:SIZE_M
    // Bestiary 2's Dhampir, SD-32 card-11 T2b lane (2026-08-23).
    ("Dhampir", SizeCategory::Medium),     // TEMPLATE:SIZE_M
    // Bestiary 5's 1, SD-31 Epic 1 follow-on batch (2026-08-15).
    ("Skinwalker", SizeCategory::Medium),  // TEMPLATE:SIZE_M, over a chassis FACT:BaseSize|S
    // Advanced Race Guide's 6, SD-31-E6-F4-002 (2026-08-16).
    ("Catfolk", SizeCategory::Medium),     // TEMPLATE:SIZE_M
    ("Kitsune", SizeCategory::Medium),     // TEMPLATE:SIZE_M
    ("Ratfolk", SizeCategory::Small),      // TEMPLATE:SIZE_S
    ("Strix", SizeCategory::Medium),       // TEMPLATE:SIZE_M
    ("Suli", SizeCategory::Medium),        // TEMPLATE:SIZE_M
    ("Wayang", SizeCategory::Small),       // TEMPLATE:SIZE_S
    // Advanced Race Guide's 4-race follow-on, SD31-E6-F4-004 (2026-08-17).
    ("Gillman", SizeCategory::Medium),     // TEMPLATE:SIZE_M
    // Nagaji's `~ Size` row's `DESC:` and `~ Type` row's `DESC:` are
    // swapped in the upstream corpus (the Size row's prose describes the
    // reptilian subtype and vice versa) -- a real upstream data-quality
    // defect, not a project bug -- but both rows carry a genuine
    // `TEMPLATE:SIZE_M`, so the size itself is unambiguous.
    ("Nagaji", SizeCategory::Medium),      // TEMPLATE:SIZE_M
    ("Vanara", SizeCategory::Medium),      // TEMPLATE:SIZE_M
    ("Vishkanya", SizeCategory::Medium),   // TEMPLATE:SIZE_M
    // Advanced Race Guide's 2-race follow-on, SD31-E6-F4-007 (2026-08-17),
    // closing `arg_races.lst`'s full 37-row playable-race roster.
    ("Changeling", SizeCategory::Medium),  // TEMPLATE:SIZE_M
    ("Samsaran", SizeCategory::Medium),    // TEMPLATE:SIZE_M
    // Bestiary 6's 1, SD-31 wave-24 integration cycle (2026-08-20).
    ("Rougarou", SizeCategory::Medium),    // chassis FACT:BaseSize|M, DESC "Rougarous are Medium creatures"
];

/// PCGen's `CATEGORY:` value for the "Adoptive Parentage" ability shape —
/// `arg_abilities_race.lst`'s `###Block: Adoptive Parentage Options`
/// (`decisions.md §16` item 2, SD-32 card-11 T2b lane). Public so the ingest
/// binary and this module read the identical literal.
pub const ADOPTIVE_PARENTAGE_CATEGORY: &str = "Adoptive Parentage";

/// One trait this option grants, resolved against the adopted race's own
/// already-ingested trait set — never fabricated. `None` if the corpus does
/// not (yet) carry a record for the target key; see
/// [`AdoptiveParentageOption::unresolved_grants`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdoptiveParentageGrant {
    pub key: String,
    pub name: String,
}

/// One "Adoptive Parentage" ability: available to a character of **any**
/// race, and — when selected — grants a **named other race**'s own standard
/// traits outright.
///
/// This is a structurally different mechanic from [`TraitRole::Alternate`]
/// (which replaces content *within* the race a character already is) and
/// from [`TraitRole::FlagGranted`] (content granted *by another trait of the
/// same race*): here the granting record and its two grant targets belong to
/// the SAME race (e.g. all three of `"Dwarf"`, `"Dwarf ~ Weapon
/// Familiarity"` and `"Dwarf ~ Languages"` are filed under race key
/// `"Dwarf"`), but the *character* selecting it need not be that race at
/// all. [`RaceCorpus::resolve`] resolves one race's own trait set against
/// itself and has no notion of "any character, any race" — so this is
/// deliberately a standalone reader, not a `TraitRole` variant threaded
/// through `resolve`'s per-race pipeline (`decisions.md §16`'s "resolves the
/// selector to the race it adopts", read literally: the race, not a rebuild
/// of the resolver around a mechanic seven records use).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdoptiveParentageOption {
    /// The record's own corpus key — bare, e.g. `"Dwarf"` (PCGen states no
    /// explicit `KEY:` on these rows, so the display name doubles as both).
    pub key: String,
    pub name: String,
    pub book_id: String,
    /// The race this option adopts. Always a race with its own chassis
    /// record in the SAME loaded corpus (`decisions.md §16`'s ask) — every
    /// one of the 7 ARG rows this reads targets an already in-scope race, and
    /// [`RaceCorpus::chassis`] on this key is how a caller re-verifies that
    /// rather than trusting this struct's word for it.
    pub adopted_race: String,
    pub description: Option<String>,
    /// Grant targets the corpus resolves to a real, already-ingested trait
    /// record. Empty is a legitimate, honestly-reported answer, never
    /// papered over with a fabricated trait.
    pub grants: Vec<AdoptiveParentageGrant>,
    /// Grant targets this option's own `ABILITY:...AUTOMATIC` token names
    /// that do **not** resolve to a loaded record for the adopted race — a
    /// fact this struct surfaces rather than silently drops. Empty for every
    /// option this cycle ingests (both of ARG's two grant targets per race
    /// are already-ingested standard traits), but the field exists so a
    /// future book's adoptive-parentage row that names an unmodelled trait
    /// is a visible finding, not a quieter one.
    pub unresolved_grants: Vec<String>,
}

/// Every "Adoptive Parentage" option in a loaded corpus (`decisions.md §16`
/// item 2), resolved against the same corpus's own already-ingested trait
/// records — never fabricated, never assumed present.
///
/// Reads [`RaceCorpus::traits_by_category`] rather than iterating every
/// trait and checking role, because these records are deliberately
/// [`TraitRole::Unclassified`] (no readable default/replace/grant gate of
/// their own) — the general-purpose seam exists exactly so a shape like this
/// one, which the default/alternate/flag-granted vocabulary was never
/// written to describe, has somewhere to be read from instead of forcing a
/// new [`TraitRole`] variant through every exhaustive match in the crate for
/// seven records.
pub fn adoptive_parentage_options(corpus: &RaceCorpus) -> Vec<AdoptiveParentageOption> {
    let mut out = Vec::new();
    for record in corpus.traits_by_category(ADOPTIVE_PARENTAGE_CATEGORY) {
        let adopted_race = record.data.race_key.clone();
        let pool = corpus.traits_for(&adopted_race);
        let mut grants = Vec::new();
        let mut unresolved_grants = Vec::new();
        for target_key in record.automatic_trait_grants() {
            match pool.iter().find(|t| t.data.key == target_key) {
                Some(found) => grants.push(AdoptiveParentageGrant {
                    key: found.data.key.clone(),
                    name: found.data.name.clone(),
                }),
                None => unresolved_grants.push(target_key),
            }
        }
        out.push(AdoptiveParentageOption {
            key: record.data.key.clone(),
            name: record.data.name.clone(),
            book_id: record.book_id.clone(),
            adopted_race,
            description: record.data.description.clone(),
            grants,
            unresolved_grants,
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    out
}

/// PCGen's `TYPE:` value marking the "Adopted Race" selector shape
/// (`decisions.md §25`): `TYPE:AdoptiveRace`, no dot-components, distinct
/// from every other `TYPE:` this corpus carries. Public so
/// `ingest_race_traits.rs` and this module read the identical literal rather
/// than each carrying its own copy.
pub const ADOPTED_RACE_SELECTOR_TYPE: &str = "AdoptiveRace";

/// The literal `CHOOSE:` prefix an Adopted-Race selector row's pool token
/// carries; the pool's `<X> Race Trait` suffix follows verbatim.
pub const ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX: &str = "ABILITYSELECTION|Special Ability|TYPE=";

/// One "Adopted Race" selector (`decisions.md §25`): available to a
/// character of the race it names' own type (the row itself, e.g. Oread's,
/// carries no further race restriction beyond being filed under that race),
/// and — when selected — grants ONE trait from a named other content kind's
/// pool: PF1e's chargen Trait mechanic (`kind: trait`,
/// [`crate::rules_core::trait_pool`]), never this corpus's own race-trait
/// population. Structurally the closest existing shape is
/// [`AdoptiveParentageOption`] (any-race-selectable, names a target), but the
/// target pool is a different content kind entirely, which is why this is a
/// distinct struct rather than a variant of that one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdoptedRaceSelector {
    pub key: String,
    pub name: String,
    pub book_id: String,
    /// The race this selector is filed under and adopts.
    pub adopted_race: String,
    /// The `CHOOSE:` token's pool suffix, e.g. `"Oread Race Trait"` — matched
    /// against a `kind: trait` record's own `TYPE:Trait.RaceTrait.<X> Race
    /// Trait` third dot-segment by [`crate::rules_core::trait_pool`]. `None`
    /// for a malformed row this project refuses to guess at rather than
    /// resolving against nothing.
    pub pool_type_suffix: Option<String>,
}

/// Every "Adopted Race" selector in a loaded corpus (`decisions.md §25`'s
/// 14-unit population), read the same way [`adoptive_parentage_options`] reads
/// its own shape: nothing here is resolved against the Trait pool itself —
/// that is [`crate::rules_core::trait_pool`]'s job, kept separate because the
/// pool is a different content kind this module does not load.
pub fn adopted_race_choose_selectors(corpus: &RaceCorpus) -> Vec<AdoptedRaceSelector> {
    let mut out = Vec::new();
    for record in corpus.traits_by_type_token(ADOPTED_RACE_SELECTOR_TYPE) {
        let pool_type_suffix = record
            .data
            .raw_tokens
            .iter()
            .find(|t| t.key == "CHOOSE" && t.value.trim_start().starts_with(ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX))
            .map(|t| t.value.trim_start()[ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX.len()..].to_string());
        out.push(AdoptedRaceSelector {
            key: record.data.key.clone(),
            name: record.data.name.clone(),
            book_id: record.book_id.clone(),
            adopted_race: record.data.race_key.clone(),
            pool_type_suffix,
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    out
}

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
/// all 330 records [`RaceCorpus::alternate_traits`] classifies as
/// [`TraitRole::Alternate`] across the 24 in-scope races — ARG's 153, Monster
/// Codex's 4, the Advanced Player's Guide's 1, Inner Sea Races' 67 and Horror
/// Adventures' 41 (the last four landed by SD-29's race-trait lane), plus
/// SD-31 Epic 1-F2's Bestiary 2 batch of 48 (ARG's 42 + Inner Sea Races' 6,
/// 2026-08-15). The three records that are *not* standalone
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
    // Monster Codex's Ratfolk alternates (SD-32 card-11 T2b lane,
    // 2026-08-23): `mc_abilities_race.lst:34-52`. Ratfolk gained a chassis
    // in `ingest_races.rs`'s SD-31-E6-F4-002 batch (ARG-native), so these
    // four Monster Codex rows -- previously refused by `ingest_race_traits.
    // rs`'s `IN_SCOPE_RACES` filter under a stale "no chassis" premise --
    // now ingest. `Surface Sprinter` sets two flags on one row (it replaces
    // both darkvision/slow-speed at once); its two replacement rows
    // (`Ratfolk ~ Surface Sprinter ~ Speed`/`~ Vision`) are
    // `TraitRole::FlagGranted` via its own `ABILITY:...|AUTOMATIC|` token,
    // never entered here directly -- same convention Strix's Wing-Clipped
    // uses above.
    ("Ratfolk ~ Cheek Pouches", &["Ratfolk_ReplaceSwarming"]),
    ("Ratfolk ~ Cleanliness", &["Ratfolk_ReplaceRodentEmpathy"]),
    ("Ratfolk ~ Lab Rat", &["Ratfolk_ReplaceTinker"]),
    ("Ratfolk ~ Surface Sprinter", &["Ratfolk_ReplaceSpeed", "Ratfolk_ReplaceVision"]),
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
    // ---- Fetchling, Grippli, Ifrit, Oread, Sylph, Undine (SD-31 Epic 1-F2, 2026-08-15) ----
    // Bestiary 2's 6-race chassis batch. Values read verbatim off
    // `data/corpus/advanced_race_guide/race_trait/` for these races' new
    // `TraitRole::Alternate` rows, the same way every row above was.
    ("Fetchling ~ Emissary", &["Fetchling_ReplaceShadowBlending"]),
    ("Fetchling ~ Gloom Shimmer", &["Fetchling_ReplaceSpellLikeAbilities"]),
    ("Fetchling ~ Shadow Magic", &["Fetchling_ReplaceSkilled"]),
    ("Fetchling ~ Subtle Manipulator", &["Fetchling_ReplaceSpellLikeAbilities"]),
    ("Fetchling ~ World Walker", &["Fetchling_ReplaceSkilled"]),
    ("Grippli ~ Glider", &["Grippli_ReplaceSwampStride"]),
    ("Grippli ~ Jumper", &["Grippli_ReplaceCamouflage"]),
    ("Grippli ~ Princely", &["Grippli_ReplaceSwampStride", "Grippli_ReplaceWeaponFamiliarity"]),
    ("Grippli ~ Toxic Skin", &["Grippli_ReplaceSwampStride", "Grippli_ReplaceCamouflage"]),
    ("Ifrit ~ Desert Mirage", &["Ifrit_ReplaceFireAffinity"]),
    ("Ifrit ~ Efreeti Magic", &["Ifrit_ReplaceSpellLikeAbility"]),
    ("Ifrit ~ Fire in the Blood", &["Ifrit_ReplaceFireAffinity"]),
    ("Ifrit ~ Fire Insight", &["Ifrit_ReplaceFireAffinity"]),
    ("Ifrit ~ Fire-Starter", &["Ifrit_ReplaceFireAffinity"]),
    ("Ifrit ~ Forge-Hardened", &["Ifrit_ReplaceSpellLikeAbility"]),
    ("Ifrit ~ Hypnotic", &["Ifrit_ReplaceFireAffinity"]),
    ("Ifrit ~ Wildfire Heart", &["Ifrit_ReplaceEnergyResistance"]),
    ("Oread ~ Crystalline Form", &["Oread_ReplaceEarthAffinity"]),
    ("Oread ~ Earth Insight", &["Oread_ReplaceEarthAffinity"]),
    ("Oread ~ Ferrous Growth", &["Oread_ReplaceSpellLikeAbility"]),
    ("Oread ~ Fertile Soil", &["Oread_ReplaceEarthAffinity"]),
    ("Oread ~ Granite Skin", &["Oread_ReplaceEnergyResistance"]),
    ("Oread ~ Mountain-Born", &["Oread_ReplaceSpellLikeAbility"]),
    ("Oread ~ Stone in the Blood", &["Oread_ReplaceEarthAffinity"]),
    ("Oread ~ Treacherous Earth", &["Oread_ReplaceSpellLikeAbility"]),
    ("Sylph ~ Air Insight", &["Sylph_ReplaceAirAffinity"]),
    ("Sylph ~ Breeze-Kissed", &["Sylph_ReplaceAirAffinity"]),
    ("Sylph ~ Like the Wind", &["Sylph_ReplaceEnergyResistance"]),
    ("Sylph ~ Sky Speaker", &["Sylph_ReplaceSpellLikeAbility"]),
    ("Sylph ~ Storm in the Blood", &["Sylph_ReplaceAirAffinity"]),
    ("Sylph ~ Thunderous Resilience", &["Sylph_ReplaceEnergyResistance"]),
    ("Sylph ~ Weather Savvy", &["Sylph_ReplaceSpellLikeAbility"]),
    ("Sylph ~ Whispering Wind", &["Sylph_ReplaceSpellLikeAbility"]),
    ("Undine ~ Acid Breath", &["Undine_ReplaceSpellLikeAbility"]),
    ("Undine ~ Amphibious", &["Undine_ReplaceSpellLikeAbility"]),
    ("Undine ~ Deepsight", &["Undine_ReplaceVision"]),
    ("Undine ~ Flesh Chameleon", &["Undine_ReplaceEnergyResistance"]),
    ("Undine ~ Hydrated Vitality", &["Undine_ReplaceWaterAffinity"]),
    ("Undine ~ Nereid Fascination", &["Undine_ReplaceSpellLikeAbility"]),
    ("Undine ~ Ooze Breath", &["Undine_ReplaceSpellLikeAbility"]),
    ("Undine ~ Terrain Chameleon", &["Undine_ReplaceEnergyResistance"]),
    ("Undine ~ Water Sense", &["Undine_ReplaceEnergyResistance"]),
    // ---- Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang (SD-31-E6-F4-003, 2026-08-16) ----
    // Advanced Race Guide's own 6-race chassis batch (SD-31-E6-F4-002 built
    // the chassis; this cycle ingested the real alternate-trait rows
    // `arg_abilities_race.lst` carries for them). Values read verbatim off
    // `data/corpus/advanced_race_guide/race_trait/` for these races' new
    // `TraitRole::Alternate` rows, the same way every row above was. Suli's
    // `Energy Strike` sets two flags on one row (it replaces both elemental
    // assault and energy resistance); Strix's `Wing-Clipped` is the only
    // entry here that ALSO grants a dependent row (`Wing-Clipped ~ Strix ~
    // Flight`, `TraitRole::FlagGranted` via its own `ABILITY:...|AUTOMATIC|`
    // token, never itself in this table). Suli's `Earthfoot`/`Firehand`/
    // `Icewalk`/`Shockshield` are deliberately absent for the identical
    // reason: they carry no `FACT:` token of their own, `Energy Strike`
    // grants them via `ABILITY:...|AUTOMATIC|Suli ~ <name>|PREABILITY:...`.
    ("Catfolk ~ Cat's Claws", &["Catfolk_ReplaceNaturalHunter"]),
    ("Catfolk ~ Clever Cat", &["Catfolk_ReplaceNaturalHunter"]),
    ("Catfolk ~ Climber", &["Catfolk_ReplaceSprinter"]),
    ("Catfolk ~ Curiosity", &["Catfolk_ReplaceNaturalHunter"]),
    ("Catfolk ~ Nimble Faller", &["Catfolk_ReplaceSprinter"]),
    ("Catfolk ~ Scent", &["Catfolk_ReplaceVision"]),
    ("Kitsune ~ Fast Shifter", &["Kitsune_ReplaceKitsuneMagic"]),
    ("Kitsune ~ Gregarious", &["Kitsune_ReplaceAgile"]),
    ("Ratfolk ~ Cornered Fury", &["Ratfolk_ReplaceSwarming"]),
    ("Ratfolk ~ Scent", &["Ratfolk_ReplaceTinker"]),
    ("Ratfolk ~ Skulk", &["Ratfolk_ReplaceTinker"]),
    ("Ratfolk ~ Unnatural", &["Ratfolk_ReplaceRodentEmpathy"]),
    ("Strix ~ Dayguard", &["Strix_ReplaceNocturnal"]),
    ("Strix ~ Frightening", &["Strix_ReplaceNocturnal"]),
    ("Strix ~ Nimble", &["Strix_ReplaceSuspicious"]),
    ("Strix ~ Tough", &["Strix_ReplaceSuspicious"]),
    ("Strix ~ Wing-Clipped", &["Strix_ReplaceFlight"]),
    ("Suli ~ Energy Strike", &["Suli_ReplaceElementalAssault", "Suli_ReplaceEnergyResistance"]),
    ("Wayang ~ Dissolution's Child", &["Wayang_ReplaceShadowMagic"]),

    // ---- Gillman, Nagaji, Vanara, Vishkanya (SD31-E6-F4-006, 2026-08-17) ----
    // Advanced Race Guide's own follow-on chassis batch (SD31-E6-F4-004
    // built the standard-tier chassis; this cycle ingested the real
    // alternate-trait rows `arg_abilities_race.lst` carries for them).
    // Values read verbatim off
    // `data/corpus/advanced_race_guide/race_trait/` for these races' new
    // `TraitRole::Alternate` rows, the same way every row above was.
    // Gillman's `Throwback` sets four flags on one row (it replaces type,
    // speed, amphibious and water-dependent together) and is the only entry
    // here that ALSO grants two dependent rows (`Throwback ~ Gillman ~
    // Type`, `Throwback ~ Gillman ~ Speed`, both `TraitRole::FlagGranted`
    // via its own `ABILITY:...|AUTOMATIC|` token, never themselves in this
    // table). Vanara's `Tree Stranger` likewise grants one dependent row
    // (`Tree Stranger ~ Vanara ~ Speed`) the same way.
    ("Gillman ~ Riverfolk", &["Gillman_ReplaceWaterDependent"]),
    ("Gillman ~ Slime Hunter", &["Gillman_ReplaceEnchantmentResistance"]),
    (
        "Gillman ~ Throwback",
        &["Gillman_ReplaceType", "Gillman_ReplaceSpeed", "Gillman_ReplaceAmphibious", "Gillman_ReplaceWaterDependent"],
    ),
    ("Nagaji ~ Hypnotic Gaze", &["Nagaji_ReplaceSerpentsSense"]),
    ("Vanara ~ Tree Stranger", &["Vanara_ReplaceSpeed"]),
    ("Vanara ~ Whitecape", &["Vanara_ReplacePrehensileTail"]),
    ("Vishkanya ~ Sensual", &["Vishkanya_ReplaceKeenSenses"]),
    ("Vishkanya ~ Subtle Appearance", &["Vishkanya_ReplaceVision"]),

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
    // ---- Fetchling, Grippli, Ifrit, Oread, Sylph, Undine (SD-31 Epic 1-F2, 2026-08-15) ----
    // Bestiary 2's 6-race chassis batch. Values read verbatim off
    // `data/corpus/inner_sea_races/race_trait/` for these races' new
    // `TraitRole::Alternate` rows, the same way every row above was.
    ("Fetchling ~ Shadow Agent", &["Fetchling_ReplaceSkilled"]),
    ("Grippli ~ Defensive Training", &["Grippli_ReplaceSwampStride", "Grippli_ReplaceWeaponFamiliarity"]),
    ("Ifrit ~ Brazen Flame", &["Ifrit_ReplaceEnergyResistance", "Ifrit_ReplaceSpellLikeAbility"]),
    ("Oread ~ Isolated", &["Oread_ReplaceEnergyResistance", "Oread_ReplaceLanguages"]),
    ("Sylph ~ Secretive", &["Sylph_ReplaceSpellLikeAbility", "Sylph_ReplaceEnergyResistance"]),
    ("Undine ~ Triton Magic", &["Undine_ReplaceSpellLikeAbility"]),
    // ---- Catfolk, Gillman, Kitsune, Nagaji, Ratfolk, Strix, Vanara,
    // Vishkanya, Wayang (a sibling SD-32 card-11 T2b lane's `inner_sea_races`
    // stale-regen fix, 2026-08-22, closed the SAME "IN_SCOPE_RACES grew,
    // book never re-run" defect this lane found for `monster_codex`; this
    // batch adds the resulting 9 real alternate-trait rows' replace flags,
    // transcribed off `data/corpus/inner_sea_races/race_trait/`, SD-32
    // card-11 T2b lane, 2026-08-23). `Vishkanya ~ Deceptive` grants
    // `Deceptive ~ Vishkanya ~ Limber` (`TraitRole::FlagGranted` via its own
    // `ABILITY:...AUTOMATIC...` token), same convention as Strix's
    // Wing-Clipped above, so that dependent row is not entered here.
    ("Catfolk ~ Jungle Stalker", &["Catfolk_ReplaceCatsLuck", "Catfolk_ReplaceSprinter"]),
    ("Gillman ~ Deep Gillman", &["Gillman_ReplaceAmphibious", "Gillman_ReplaceEnchantmentResistance"]),
    ("Kitsune ~ Duplicitous", &["Kitsune_ReplaceKitsuneMagic"]),
    ("Nagaji ~ Serpent Affinity", &["Nagaji_ReplaceResistant"]),
    ("Ratfolk ~ Market Dweller", &["Ratfolk_ReplaceTinker"]),
    ("Strix ~ Cautious Brawler", &["Strix_ReplaceHatred", "Strix_ReplaceSuspicious"]),
    ("Vanara ~ Risky Troublemaker", &["Vanara_ReplacePrehensileTail"]),
    ("Vishkanya ~ Deceptive", &["Vishkanya_ReplaceLimber"]),
    ("Wayang ~ In the Shadows", &["Wayang_ReplaceLurker"]),

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

    // ================= Skinwalker (Bestiary 5) =================
    // SD-33 Epic 6 fold (2026-08-26), recovering SD31-E6-F4-005's lost
    // wave-11 lane: 9 kin selectors plus their 36 replacement rows, all 45
    // `TraitRole::Alternate` -- **unlike** Core Essentials' Aasimar/Tiefling
    // heritages just above, whose 48 replacement rows are `FlagGranted` (no
    // `FACT:` token of their own; the flag lives on the selector alone),
    // Skinwalker's oracle genuinely places a `FACT:Skinwalker_Replace<Trait>
    // |True` token on EACH replacement row too, so `classify()` correctly
    // reads all 45 as `Alternate` by the same generic rule that reads every
    // other row here, and `link_automatic_grants` does not demote them for
    // also being a grant target -- Monster Codex's `Oversized Goblin ~
    // Ability Scores`/`~ Size` are the pre-existing proof that a record's
    // own flag always wins (see that function's own doc comment). Values
    // are `RaceTraitCacheData::sets_replace_flags` read verbatim off
    // `data/corpus/bestiary_5/race_trait/skinwalker/`, and
    // `the_alternate_trait_flag_table_matches_the_corpus_for_every_alternate`
    // re-derives all 45 from those records rather than trusting this block.
    // Every kin's own 4 flag names are the SAME 4 (`Skinwalker_Replace
    // AbilityScores`/`AnimalMinded`/`ChangeShape`/`SpellLikeAbility`) --
    // PCGen scopes the names to the race, not the heritage, since only one
    // kin can ever be active on one character.
    //
    // The book's other 20 records are deliberately absent: they are the
    // shared, gate-free `Change Shape (<Option>)` component records every
    // kin's own `Change Shape` replacement row TYPE-pool-references, never
    // a player-facing standalone choice; see this module's own
    // `no_corpus_trait_is_left_without_a_readable_gate` test for why
    // `classify()` correctly leaves them `Unclassified`.
    ("Skinwalker ~ Werebat-Kin", &["Skinwalker_ReplaceAbilityScores", "Skinwalker_ReplaceAnimalMinded", "Skinwalker_ReplaceChangeShape", "Skinwalker_ReplaceSpellLikeAbility"]),
    ("Skinwalker ~ Werebear-Kin", &["Skinwalker_ReplaceAbilityScores", "Skinwalker_ReplaceAnimalMinded", "Skinwalker_ReplaceChangeShape", "Skinwalker_ReplaceSpellLikeAbility"]),
    ("Skinwalker ~ Wereboar-Kin", &["Skinwalker_ReplaceAbilityScores", "Skinwalker_ReplaceAnimalMinded", "Skinwalker_ReplaceChangeShape", "Skinwalker_ReplaceSpellLikeAbility"]),
    ("Skinwalker ~ Werecrocodile-Kin", &["Skinwalker_ReplaceAbilityScores", "Skinwalker_ReplaceAnimalMinded", "Skinwalker_ReplaceChangeShape", "Skinwalker_ReplaceSpellLikeAbility"]),
    ("Skinwalker ~ Wereraptor-Kin", &["Skinwalker_ReplaceAbilityScores", "Skinwalker_ReplaceAnimalMinded", "Skinwalker_ReplaceChangeShape", "Skinwalker_ReplaceSpellLikeAbility"]),
    ("Skinwalker ~ Wererat-Kin", &["Skinwalker_ReplaceAbilityScores", "Skinwalker_ReplaceAnimalMinded", "Skinwalker_ReplaceChangeShape", "Skinwalker_ReplaceSpellLikeAbility"]),
    ("Skinwalker ~ Wereshark-Kin", &["Skinwalker_ReplaceAbilityScores", "Skinwalker_ReplaceAnimalMinded", "Skinwalker_ReplaceChangeShape", "Skinwalker_ReplaceSpellLikeAbility"]),
    ("Skinwalker ~ Weretiger-Kin", &["Skinwalker_ReplaceAbilityScores", "Skinwalker_ReplaceAnimalMinded", "Skinwalker_ReplaceChangeShape", "Skinwalker_ReplaceSpellLikeAbility"]),
    ("Skinwalker ~ Werewolf-Kin", &["Skinwalker_ReplaceAbilityScores", "Skinwalker_ReplaceAnimalMinded", "Skinwalker_ReplaceChangeShape", "Skinwalker_ReplaceSpellLikeAbility"]),
    ("Werebat-Kin ~ Ability Scores", &["Skinwalker_ReplaceAbilityScores"]),
    ("Werebat-Kin ~ Animal-Minded", &["Skinwalker_ReplaceAnimalMinded"]),
    ("Werebat-Kin ~ Change Shape", &["Skinwalker_ReplaceChangeShape"]),
    ("Werebat-Kin ~ Spell-Like Ability", &["Skinwalker_ReplaceSpellLikeAbility"]),
    ("Werebear-Kin ~ Ability Scores", &["Skinwalker_ReplaceAbilityScores"]),
    ("Werebear-Kin ~ Animal-Minded", &["Skinwalker_ReplaceAnimalMinded"]),
    ("Werebear-Kin ~ Change Shape", &["Skinwalker_ReplaceChangeShape"]),
    ("Werebear-Kin ~ Spell-Like Ability", &["Skinwalker_ReplaceSpellLikeAbility"]),
    ("Wereboar-Kin ~ Ability Scores", &["Skinwalker_ReplaceAbilityScores"]),
    ("Wereboar-Kin ~ Animal-Minded", &["Skinwalker_ReplaceAnimalMinded"]),
    ("Wereboar-Kin ~ Change Shape", &["Skinwalker_ReplaceChangeShape"]),
    ("Wereboar-Kin ~ Spell-Like Ability", &["Skinwalker_ReplaceSpellLikeAbility"]),
    ("Werecrocodile-Kin ~ Ability Scores", &["Skinwalker_ReplaceAbilityScores"]),
    ("Werecrocodile-Kin ~ Animal-Minded", &["Skinwalker_ReplaceAnimalMinded"]),
    ("Werecrocodile-Kin ~ Change Shape", &["Skinwalker_ReplaceChangeShape"]),
    ("Werecrocodile-Kin ~ Spell-Like Ability", &["Skinwalker_ReplaceSpellLikeAbility"]),
    ("Wereraptor-Kin ~ Ability Scores", &["Skinwalker_ReplaceAbilityScores"]),
    ("Wereraptor-Kin ~ Animal-Minded", &["Skinwalker_ReplaceAnimalMinded"]),
    ("Wereraptor-Kin ~ Change Shape", &["Skinwalker_ReplaceChangeShape"]),
    ("Wereraptor-Kin ~ Spell-Like Ability", &["Skinwalker_ReplaceSpellLikeAbility"]),
    ("Wererat-Kin ~ Ability Scores", &["Skinwalker_ReplaceAbilityScores"]),
    ("Wererat-Kin ~ Animal-Minded", &["Skinwalker_ReplaceAnimalMinded"]),
    ("Wererat-Kin ~ Change Shape", &["Skinwalker_ReplaceChangeShape"]),
    ("Wererat-Kin ~ Spell-Like Ability", &["Skinwalker_ReplaceSpellLikeAbility"]),
    ("Wereshark-Kin ~ Ability Scores", &["Skinwalker_ReplaceAbilityScores"]),
    ("Wereshark-Kin ~ Animal-Minded", &["Skinwalker_ReplaceAnimalMinded"]),
    ("Wereshark-Kin ~ Change Shape", &["Skinwalker_ReplaceChangeShape"]),
    ("Wereshark-Kin ~ Spell-Like Ability", &["Skinwalker_ReplaceSpellLikeAbility"]),
    ("Weretiger-Kin ~ Ability Scores", &["Skinwalker_ReplaceAbilityScores"]),
    ("Weretiger-Kin ~ Animal-Minded", &["Skinwalker_ReplaceAnimalMinded"]),
    ("Weretiger-Kin ~ Change Shape", &["Skinwalker_ReplaceChangeShape"]),
    ("Weretiger-Kin ~ Spell-Like Ability", &["Skinwalker_ReplaceSpellLikeAbility"]),
    ("Werewolf-Kin ~ Ability Scores", &["Skinwalker_ReplaceAbilityScores"]),
    ("Werewolf-Kin ~ Animal-Minded", &["Skinwalker_ReplaceAnimalMinded"]),
    ("Werewolf-Kin ~ Change Shape", &["Skinwalker_ReplaceChangeShape"]),
    ("Werewolf-Kin ~ Spell-Like Ability", &["Skinwalker_ReplaceSpellLikeAbility"]),
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
    /// license-validates. 24 races: `decisions.md §25.3`'s original 18 plus
    /// SD-31 Epic 1-F2's Bestiary 2 batch of 6 (2026-08-15).
    #[test]
    fn all_twenty_five_in_scope_races_load_from_the_real_on_disk_corpus() {
        let corpus = all_books();
        assert_eq!(
            corpus.race_keys().len(),
            39,
            "39 in-scope races: CRB 7 + Bestiary 1's 11 + Bestiary 2's 7 (the original 6 plus \
             Dhampir, SD-32 card-11 T2b lane, 2026-08-23, chassis + the 11 unconditional \
             standard traits only -- its own heritage/subrace file stays deferred, same \
             precedent as Skinwalker below) + Bestiary 5's 1 \
             (Skinwalker, chassis + standard tier only) + Advanced Race Guide's 12 \
             (SD-31-E6-F4-002, 2026-08-16: Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang; \
             SD31-E6-F4-004, 2026-08-17: Gillman, Nagaji, Vanara, Vishkanya; SD31-E6-F4-007, \
             2026-08-17: Changeling, Samsaran -- the full `arg_races.lst` 37-row playable-race \
             roster, closed) + Bestiary 6's 1 (Rougarou, SD-31 wave-24, 2026-08-20, chassis + \
             standard tier only)"
        );
        assert_eq!(corpus.chassis("Dwarf").expect("Dwarf").book_id, "core_rulebook");
        assert_eq!(corpus.chassis("Tengu").expect("Tengu").book_id, "beastiary");
        assert_eq!(corpus.chassis("Fetchling").expect("Fetchling").book_id, "bestiary_2");
        assert_eq!(corpus.chassis("Skinwalker").expect("Skinwalker").book_id, "bestiary_5");
        assert_eq!(corpus.chassis("Catfolk").expect("Catfolk").book_id, "advanced_race_guide");
        // ARG contributed only traits, never a race chassis, until
        // SD-31-E6-F4-002 (2026-08-16, `decisions.md` Decision 10's Catfolk
        // worked example): it now declares 6 races of its own (Catfolk,
        // Kitsune, Ratfolk, Strix, Suli, Wayang). SD31-E6-F4-004
        // (2026-08-17) added 4 more (Gillman, Nagaji, Vanara, Vishkanya),
        // and SD31-E6-F4-007 (2026-08-17) added the last 2 (Changeling,
        // Samsaran), closing `arg_races.lst`'s full 37-row roster -- so the
        // assertion is exactly these 12 expected chassis.
        let arg_chassis: BTreeSet<&str> = corpus
            .chassis
            .values()
            .filter(|c| c.book_id == "advanced_race_guide")
            .map(|c| c.data.key.as_str())
            .collect();
        assert_eq!(
            arg_chassis,
            BTreeSet::from([
                "Catfolk", "Changeling", "Gillman", "Kitsune", "Nagaji", "Ratfolk", "Samsaran",
                "Strix", "Suli", "Vanara", "Vishkanya", "Wayang"
            ]),
            "ARG must contribute exactly this batch's 12 race chassis, no more, no fewer"
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
        // `Suli ~ Trusted Mediator` (Inner Sea Races, `isr_abilities_race.lst`,
        // landed by a sibling SD-32 card-11 T2b lane's `inner_sea_races`
        // stale-regen fix, 2026-08-22) is the third, same *kind* of residue as
        // `Human ~ Tribalistic Languages`: its own `!PREFACT` is wrapped inside
        // a `PREMULT` self-exclusion guard (see this module's doc comment on
        // why that is preserved verbatim rather than read as a standalone
        // suppressor), so it carries no standalone gate this engine reads.
        //
        // The 7 `Drow`/`Dwarf`/`Elf`/`Gnome`/`Grippli`/`Halfling`/`Orc` rows
        // (SD-32 card-11 T2b lane, 2026-08-23) are a **different kind of
        // residue than all three above**: they are not a data gap. Each is a
        // genuine `CHOOSE:ABILITYSELECTION|Adoptive Parentage|ANY` pool
        // member for `Human ~ Adoptive Parentage` (`arg_abilities_race.lst:
        // 257`, already ingested and already `TraitRole::Alternate`) — gated
        // by the CHOOSE on that OTHER row, not by any readable gate of their
        // own, which is exactly why `classify()` correctly leaves them
        // `Unclassified` rather than inventing a fifth role for seven
        // records. [`adoptive_parentage_options`] is the reader that
        // resolves them (to the race each one adopts, and the two
        // already-modelled traits it grants); this test's job is only to
        // confirm they carry no gate this engine would otherwise apply them
        // by, which would be wrong — nobody who has not picked `Human ~
        // Adoptive Parentage` and then chosen one of these seven gets it for
        // free.
        assert_eq!(
            unclassified,
            vec![
                // SD-32 `decisions.md §25` cycle 2, all 14: the "Adopted
                // Race" selector shape (`ingest_race_traits.rs`'s new
                // `selector_only` `BookSource`s). Same *kind* of residue as
                // the 7 Adoptive-Parentage-pool rows below -- gated by their
                // OWN `CHOOSE:ABILITYSELECTION|Special Ability|TYPE=<X> Race
                // Trait` pool, resolved by
                // `adopted_race_choose_selectors`/`crate::rules_core::
                // trait_pool::resolve_adopted_race_options`, never by a
                // readable `PREFACT`/default gate this classifier would
                // otherwise apply them by. Sorted by key, exactly as
                // `unclassified_traits()` sorts every other entry here.
                ("Catfolk", "Adopted Race ~ Catfolk"),
                ("Dhampir", "Adopted Race ~ Dhampir"),
                ("Fetchling", "Adopted Race ~ Fetchling"),
                ("Grippli", "Adopted Race ~ Grippli"),
                ("Ifrit", "Adopted Race ~ Ifrit"),
                ("Oread", "Adopted Race ~ Oread"),
                ("Ratfolk", "Adopted Race ~ Ratfolk"),
                ("Rougarou", "Adopted Race ~ Rougarou"),
                ("Skinwalker", "Adopted Race ~ Skinwalker"),
                ("Suli", "Adopted Race ~ Suli"),
                ("Sylph", "Adopted Race ~ Sylph"),
                ("Undine", "Adopted Race ~ Undine"),
                ("Vanara", "Adopted Race ~ Vanara"),
                ("Vishkanya", "Adopted Race ~ Vishkanya"),
                ("Drow", "Drow"),
                ("Dwarf", "Dwarf"),
                ("Elf", "Elf"),
                ("Gnome", "Gnome"),
                ("Grippli", "Grippli"),
                ("Halfling", "Halfling"),
                ("Human", "Human ~ Tribalistic Languages"),
                ("Orc", "Orc"),
                ("Goblin", "Oversized Goblin"),
                // SD-33 Epic 6 fold (2026-08-26): the 20 Skinwalker `Change
                // Shape (<Option>)` component records the folded heritage
                // batch adds. Each is a `VISIBLE:NO` mechanical helper
                // (`DEFINE`/internal-`ABILITY`/`TEMPBONUS` only) with no
                // `PREFACT`/`PREABILITY` gate of its own -- every kin's own
                // `Change Shape` replacement row reaches its options through
                // a TYPE pool (`ABILITY:Skinwalker Racial
                // Trait|AUTOMATIC|TYPE=Skinwalker Change Shape <Kin>`), which
                // is a real, resolvable grant path (the same shape
                // `race_trait_picker`'s Change Shape UI already reads for
                // every other race) but not one `classify()`/
                // `link_automatic_grants` read -- see the `Unclassified`
                // count's own comment in
                // `the_whole_corpus_classifies_into_the_four_roles_with_no_leftovers`.
                // A different kind of residue than the three named above:
                // not a data gap, a TYPE-pool grant this classifier does not
                // model, the `Oversized Goblin` shape at a larger scale.
                ("Skinwalker", "Skinwalker ~ Change Shape (Amphibious)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Base Speed Bonus)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Bite)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Charisma)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Climb Speed 20 Feet)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Climb Speed 30 Feet)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Distraction)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Endurance)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Ferocity)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Fly Speed Bonus)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Gore)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Hoof)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Perception Bonus)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Reduce Falling Damage)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Saves)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Scent)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (See In Darkness)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Swim Speed)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Talon)"),
                ("Skinwalker", "Skinwalker ~ Change Shape (Wisdom)"),
                ("Suli", "Suli ~ Trusted Mediator"),
            ]
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
            redacted, 42,
            "Inner Sea Races' 25 PI-redacted records + Core Essentials' 9 + SD-33 Epic 6's \
             folded Skinwalker heritage batch's 8 (2026-08-26), counted on disk. Skinwalker's \
             8 are the same `heritage row and its Ability Scores replacement row carry the \
             same prose, hitting twice` shape Core Essentials' Tiefling heritages already show \
             below: `Werebear-Kin`/`Wereboar-Kin`/`Werecrocodile-Kin`/`Weretiger-Kin`'s own \
             selector row and each one's `~ Ability Scores` replacement row (4 kins x 2 rows), \
             all four `DESCISPI:YES` in the pinned oracle -- the other five kins (Werebat, \
             Wererat, Wereshark, Werewolf, Wereraptor) carry no `DESCISPI:` declaration and hit \
             0 blacklist terms. \
             Inner Sea Races' 25 PI-redacted records + Core Essentials' 9. \
             ISR's 22 -> 25 by a sibling SD-32 card-11 T2b lane's stale-regen fix \
             (2026-08-22): `Catfolk ~ Jungle Stalker`, `Ratfolk ~ Market Dweller` and \
             `Suli ~ Trusted Mediator` (the row this module's own \
             `no_corpus_trait_is_left_without_a_readable_gate` test separately tracks as \
             `Unclassified`, not `Alternate` -- redaction is independent of trait role) each \
             name Golarion Product Identity in their prose; the other 6 of that batch's 9 new \
             alternates do not. \
             Horror Adventures added 0: it is a rules supplement, not a campaign setting. \
             ISR's 18 -> 22 by SD-31 Epic 1-F2 (2026-08-15): `Fetchling ~ Shadow Agent`, \
             `Grippli ~ Defensive Training`, `Ifrit ~ Brazen Flame` and `Undine ~ Triton \
             Magic` each name Golarion Product Identity in their prose and are correctly \
             redacted; Bestiary 2's other two ISR alternates (`Oread ~ Isolated`, `Sylph ~ \
             Secretive`) do not and are not. \
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
        // 173 -> 230 by SD-31 Epic 1-F2 (2026-08-15): Bestiary 2's 6-race
        // batch adds 57 new standard (`is_racial_default`) rows.
        // 230 -> 239 by the Skinwalker follow-on batch (2026-08-15): 9 new
        // standard-tier trait rows (chassis + default tier only -- the
        // heritage-shaped alternates are NOT ingested by this batch).
        // 239 -> 297 by SD-31-E6-F4-002 (2026-08-16): Advanced Race Guide's
        // own 6-race batch (Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang)
        // adds 58 new standard rows, the same flat chassis+standard-trait
        // shape as Bestiary 2/5 above, no heritage content.
        // 297 -> 335 by SD31-E6-F4-004 (2026-08-17): Advanced Race Guide's
        // 4-race follow-on batch (Gillman, Nagaji, Vanara, Vishkanya) adds
        // 38 new standard rows, same flat shape, no heritage content.
        // 335 -> 353 by SD31-E6-F4-007 (2026-08-17): Advanced Race Guide's
        // 2-race follow-on batch (Changeling, Samsaran) adds 18 new
        // standard rows, closing `arg_races.lst`'s full 37-row roster --
        // Changeling's 3 hag-mother heritage-choice sub-traits are
        // deliberately excluded (`ingest_races.rs`'s
        // `is_heritage_choice_subtrait`), not silently absorbed here.
        // 353 -> 361 by SD-31 wave-24 (2026-08-20): Rougarou (Bestiary 6)
        // adds 8 new standard rows, same flat shape, no heritage content.
        // 361 -> 373 by SD-32 card-11 T2b lane (2026-08-23): Dhampir
        // (Bestiary 2) adds its 12 unconditional standard-trait rows
        // (Ability Scores, Type, Size, Speed, Vision, Skilled, Undead
        // Resistance, Weakness, Negative Energy Affinity, Spell-Like
        // Ability, Resist Level Drain, Languages) -- same flat shape, its
        // heritage/subrace file stays deferred, same precedent as
        // Skinwalker/Rougarou above.
        assert_eq!(count(TraitRole::Default), 373);
        // 153 ARG + Monster Codex's 4 + the Advanced Player's Guide's 1
        // (`Half-Orc ~ Plagueborn`) + Inner Sea Races' 67 + Horror
        // Adventures' 41, all landed by SD-29's race-trait lane, + SD-31
        // Epic 1-F2's 48 (ARG's 42 + Inner Sea Races' 6).
        // 330 -> 349 by SD-31-E6-F4-003 (2026-08-16): the same 6 races' own
        // real ARG alternate-trait rows, minus Strix's Wing-Clipped-granted
        // Flight and Suli's Energy-Strike-granted Earthfoot/Firehand/
        // Icewalk/Shockshield (those 5 are `FlagGranted`, not `Alternate`).
        // 349 -> 357 by SD31-E6-F4-006 (2026-08-17): Gillman (3: Riverfolk,
        // Slime Hunter, Throwback), Nagaji (1: Hypnotic Gaze), Vanara (2:
        // Tree Stranger, Whitecape) and Vishkanya (2: Sensual, Subtle
        // Appearance)'s own real ARG alternate-trait rows -- 8 total, none
        // of them granting a further dependent row that would also count
        // here (Throwback's and Tree Stranger's grants are `FlagGranted`,
        // counted below instead).
        // 357 -> 361 by SD-32 card-11 T2b lane (2026-08-23): Monster
        // Codex's 4 new Ratfolk alternates (Cheek Pouches, Cleanliness,
        // Lab Rat, Surface Sprinter) -- Surface Sprinter's own two
        // replacement rows are `FlagGranted`, counted below instead.
        // 361 -> 370 by a sibling SD-32 card-11 T2b lane's `inner_sea_races`
        // stale-regen fix (2026-08-22): 9 new alternates (Catfolk ~ Jungle
        // Stalker, Gillman ~ Deep Gillman, Kitsune ~ Duplicitous, Nagaji ~
        // Serpent Affinity, Ratfolk ~ Market Dweller, Strix ~ Cautious
        // Brawler, Vanara ~ Risky Troublemaker, Vishkanya ~ Deceptive,
        // Wayang ~ In the Shadows) -- Vishkanya ~ Deceptive's own dependent
        // row is `FlagGranted`, counted below instead.
        // 370 -> 415 by SD-33 Epic 6 fold (2026-08-26), recovering
        // SD31-E6-F4-005's lost wave-11 lane: Skinwalker's 9 kin selectors
        // plus their 36 replacement rows, all 45 `Alternate` because every
        // one carries a genuine, per-row `FACT:Skinwalker_Replace<Trait>
        // |True` token in the pinned oracle -- a different shape from
        // Aasimar/Tiefling, whose replacement rows carry no `FACT:` token of
        // their own at all (flag lives on the selector alone; see the
        // 48-vs-16 comment above). `link_automatic_grants` does NOT demote
        // these to `FlagGranted` even though each is also a grant target --
        // Monster Codex's `Oversized Goblin ~ Ability Scores`/`~ Size` are
        // the pre-existing proof that a record's own flag always wins over
        // being granted, in this corpus's real, shipped design (see that
        // function's own doc comment).
        assert_eq!(count(TraitRole::Alternate), 415);
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
        // 58 -> 66 by SD-31 Epic 1-F2: 8 new records this batch's own rows
        // carry no readable gate on (`classify()` alone would call all 8
        // `Unclassified`), but every one is named by a sibling alternate's
        // `ABILITY:<cat>|AUTOMATIC|<key>` token, so `link_automatic_grants`
        // promotes all 8 to `FlagGranted` -- the exact `Feral ~ Languages`
        // shape this module's docs already name, at a larger scale than any
        // prior book: `Fetchling ~ Gloom Shimmer` grants `Gloom Shimmer ~
        // Spell-Like Abilities`, `Fetchling ~ Subtle Manipulator` grants
        // `Subtle Manipulator ~ Spell-Like Abilities`, `Fetchling ~ World
        // Walker` grants `World Walker ~ Skilled` (all three ARG), and
        // `Oread ~ Isolated` grants `Isolated ~ Oread ~ Languages` (Inner Sea
        // Races) -- plus the 4 already-`FlagGranted`-by-`classify()` `Mostly
        // Human ~ <Race> ~ Languages` rows (positive `PREFACT`), one of
        // which (Oread's) is granted by that same `Oread ~ Isolated`.
        // 66 -> 71 by SD-31-E6-F4-003 (2026-08-16): Strix's `Wing-Clipped`
        // grants `Wing-Clipped ~ Strix ~ Flight` (`ABILITY:Strix Racial
        // Trait|AUTOMATIC|...`), and Suli's `Energy Strike` grants all 4 of
        // `Earthfoot`/`Firehand`/`Icewalk`/`Shockshield` the same way, each
        // via an `ABILITY:...|AUTOMATIC|Suli ~ <name>|PREABILITY:...` token
        // whose trailing `PREABILITY` clause `link_automatic_grants` already
        // tolerates (same shape it already reads for the Fetchling/Oread
        // rows named above).
        // 71 -> 74 by SD31-E6-F4-006 (2026-08-17): Gillman's `Throwback`
        // grants both `Throwback ~ Gillman ~ Type` and `Throwback ~ Gillman
        // ~ Speed` (one `ABILITY:...|AUTOMATIC|` token naming two keys), and
        // Vanara's `Tree Stranger` grants `Tree Stranger ~ Vanara ~ Speed`
        // the same way -- 3 new dependent rows total.
        // 74 -> 76 by SD-32 card-11 T2b lane (2026-08-23): Monster Codex's
        // Ratfolk `Surface Sprinter` grants both `Ratfolk ~ Surface Sprinter
        // ~ Speed` and `Ratfolk ~ Surface Sprinter ~ Vision` (one
        // `ABILITY:...|AUTOMATIC|` token naming two keys) -- the identical
        // Gillman `Throwback` shape immediately above.
        // 76 -> 78 by a sibling SD-32 card-11 T2b lane's `inner_sea_races`
        // stale-regen fix (2026-08-22): `Vishkanya ~ Deceptive` grants
        // `Deceptive ~ Vishkanya ~ Limber` plus one more dependent row from
        // the same 9-alternate batch (re-derive:
        // `unclassified_traits()`/`alternate_traits()` diffed against the
        // 9 new ISR alternates' own `ABILITY:...AUTOMATIC...` tokens names
        // the second). Unmoved by SD-33 Epic 6's Skinwalker fold
        // (2026-08-26): its 36 kin replacement rows carry their own
        // `FACT:Skinwalker_Replace<Trait>|True` and stay `Alternate`, same
        // as Monster Codex's `Oversized Goblin ~ Ability Scores`/`~ Size`
        // precedent (see `link_automatic_grants`'s own doc comment); counted
        // above instead.
        assert_eq!(count(TraitRole::FlagGranted), 78);
        // `Oversized Goblin`, `Human ~ Tribalistic Languages` and (added by a
        // sibling SD-32 card-11 T2b lane's `inner_sea_races` stale-regen fix,
        // 2026-08-22) `Suli ~ Trusted Mediator` -- see
        // `no_corpus_trait_is_left_without_a_readable_gate`, which pins all
        // three by key and names each one's remedy. Unchanged by SD-31 Epic
        // 1-F2: every one of that batch's gate-free rows has a real granter
        // (see above), so none of them lands here.
        //
        // 3 -> 10 by SD-32 card-11 T2b lane (2026-08-23): the 7 `Human ~
        // Adoptive Parentage` CHOOSE-pool members (Drow/Dwarf/Elf/Gnome/
        // Grippli/Halfling/Orc). They carry no gate of their own by design
        // -- `no_corpus_trait_is_left_without_a_readable_gate` names why --
        // and `link_automatic_grants` cannot promote them either, because
        // its per-race grouping never crosses from `Human`'s trait group
        // into theirs.
        // 10 -> 24 by SD-32 `decisions.md §25` cycle 2 (2026-08-23): the 14
        // "Adopted Race" selector rows (`ingest_race_traits.rs`'s new
        // `selector_only` `BookSource`s -- bestiary_2 7, bestiary_3 5,
        // bestiary_5 1, bestiary_6 1). Same *kind* of residue as the 7
        // Adoptive-Parentage-pool members immediately above: no readable
        // gate of their own by design, gated instead by their OWN
        // `CHOOSE:ABILITYSELECTION|Special Ability|TYPE=<X> Race Trait`,
        // resolved by `adopted_race_choose_selectors`/
        // `crate::rules_core::trait_pool::resolve_adopted_race_options`, not
        // by this classifier.
        // 24 -> 44 by SD-33 Epic 6 fold (2026-08-26): the 20 new Skinwalker
        // `Change Shape (<Option>)` component records (Bite, Claw/Talon,
        // Gore, Hoof, Scent, ...). Each kin's own `Change Shape` replacement
        // row pool-references its options by TYPE
        // (`ABILITY:Skinwalker Racial Trait|AUTOMATIC|TYPE=Skinwalker Change
        // Shape <Kin>`), not by exact KEY -- `link_automatic_grants` only
        // promotes an exact-KEY match (see the 58-record comment above), so
        // a TYPE pool reference does not promote these to `FlagGranted`.
        // They carry no gate of their own either (`VISIBLE:NO` mechanical
        // helpers, no `PREFACT`/`PREABILITY`), so `classify()` alone leaves
        // them here -- reachable through the same `TYPE=` pool resolution
        // `race_trait_picker`'s Change Shape UI already reads for every
        // other race's identical shape, just not through this module's own
        // grant-linking pass.
        assert_eq!(count(TraitRole::Unclassified), 44);
        assert_eq!(
            corpus.traits.values().flatten().count(),
            910,
            "175 standard + 156 ARG + 5 Monster Codex + 1 APG + 71 Inner Sea Races \
             + 43 Horror Adventures + 64 Core Essentials heritage records (16 heritages \
             + the 48 replacement rows they grant) + SD-31 Epic 1-F2's 113 (57 standard \
             + 42 ARG alternates + 3 ARG grant-linked rows + 6 Inner Sea Races alternates \
             + 5 Inner Sea Races grant-linked/positive-gate rows, 2026-08-15) + the \
             Skinwalker follow-on batch's 9 standard-tier rows + SD-31-E6-F4-002's \
             Advanced Race Guide batch of 58 standard-tier rows (2026-08-16: Catfolk, \
             Kitsune, Ratfolk, Strix, Suli, Wayang; 637 -> 695) + SD-31-E6-F4-003's own \
             24-record alternate-trait batch for those same 6 races (2026-08-16: 695 -> 719) \
             + SD31-E6-F4-004's Advanced Race Guide follow-on batch of 38 standard-tier \
             rows (2026-08-17: Gillman, Nagaji, Vanara, Vishkanya; 719 -> 757) + \
             SD31-E6-F4-006's own 11-record alternate-trait batch for those same 4 races \
             (2026-08-17: 8 alternates + 3 grant-linked rows; 757 -> 768) + SD31-E6-F4-007's \
             Advanced Race Guide follow-on batch of 18 standard-tier rows (2026-08-17: \
             Changeling 9, Samsaran 9; 768 -> 786), closing `arg_races.lst`'s full 37-row \
             playable-race roster -- no new alternate-trait batch, neither race has any ARG \
             alternate content + SD-31 wave-24's Rougarou (Bestiary 6, 2026-08-20): 8 new \
             standard-tier rows, no heritage/alternate content (786 -> 794) + SD-32 card-11 \
             T2b lane's 18 (2026-08-23: Dhampir's 12 standard-tier rows + Monster Codex's 4 \
             new Ratfolk alternates + the 2 dependent rows Surface Sprinter grants; 794 -> 812) \
             + a sibling SD-32 card-11 T2b lane's `inner_sea_races` stale-regen fix \
             (2026-08-22): 9 new alternates + their 2 dependent rows + Suli ~ Trusted \
             Mediator (Unclassified) = 12 (812 -> 824) + this cycle's 7 `Human ~ Adoptive \
             Parentage` CHOOSE-pool members (Unclassified: Drow, Dwarf, Elf, Gnome, \
             Grippli, Halfling, Orc; `decisions.md §16` item 2, 2026-08-23; 824 -> 831) \
             + `decisions.md §25` cycle 2's 14 Adopted-Race selector records (Unclassified: \
             bestiary_2 7, bestiary_3 5, bestiary_5 1, bestiary_6 1; 2026-08-23; 831 -> 845) \
             + SD-33 Epic 6's fold of SD31-E6-F4-005's lost wave-11 Skinwalker heritage lane \
             (2026-08-26): 45 Alternate (9 kin selectors + their 36 replacement rows) + 20 \
             Unclassified (the shared `Change Shape (<Option>)` component records the kin \
             `Change Shape` rows TYPE-pool-reference; see the `Alternate`/`Unclassified` \
             assertions' own comments above) = 65 (845 -> 910)"
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
        //
        // SD-31 Epic 1-F2 (2026-08-15) moved this 93 -> 113: 20 brand-new
        // `<Race>_Replace*` flags across the 6 Bestiary 2 races' ARG+ISR
        // alternates, all in the batch's own new race namespaces (no B2
        // race's flag can collide with the original 18's, and none does --
        // re-derived on the written tree, 20 distinct flags across
        // `Fetchling_*`/`Grippli_*`/`Ifrit_*`/`Oread_*`/`Sylph_*`/`Undine_*`).
        // Every one of them is claimed by that same race's own standard row
        // (each B2 race's `ingest_races` batch wrote its `!PREFACT` gates
        // from the same globalvar reconciliation the original 18 use), so
        // the orphan-flag assertion above still does not move.
        //
        // SD-31-E6-F4-003 (2026-08-16) moved this 113 -> 127: 14 brand-new
        // `<Race>_Replace*` flags across Catfolk (3), Kitsune (2), Ratfolk
        // (3), Strix (3, including `Strix_ReplaceFlight`), Suli (2) and
        // Wayang (1)'s own real ARG alternate-trait rows -- all in those 6
        // races' own namespaces, none colliding with any earlier book's.
        // Every one is claimed by that same race's own standard row
        // (`ingest_races.rs`'s `SD-31-E6-F4-002` batch wrote each race's
        // `!PREFACT` gates from its own globalvar file), so the orphan-flag
        // assertion above still does not move.
        //
        // SD31-E6-F4-006 (2026-08-17) moved this 127 -> 137: 10 brand-new
        // `<Race>_Replace*` flags across Gillman (5: WaterDependent,
        // EnchantmentResistance, Type, Speed, Amphibious), Nagaji (1:
        // SerpentsSense), Vanara (2: Speed, PrehensileTail) and Vishkanya
        // (2: KeenSenses, Vision)'s own real ARG alternate-trait rows -- all
        // in those 4 races' own namespaces, none colliding with any earlier
        // book's. Every one is claimed by that same race's own standard row
        // (`ingest_races.rs`'s `SD31-E6-F4-004` batch wrote each race's
        // `!PREFACT` gates from its own globalvar file), so the orphan-flag
        // assertion above still does not move.
        //
        // SD-32 card-11 T2b lane (2026-08-23) moved this 137 -> 139: Monster
        // Codex's Ratfolk `Surface Sprinter` sets 2 brand-new flags,
        // `Ratfolk_ReplaceSpeed` and `Ratfolk_ReplaceVision` (its other
        // three new alternates -- Cheek Pouches/Cleanliness/Lab Rat -- reuse
        // `Ratfolk_ReplaceSwarming`/`ReplaceRodentEmpathy`/`ReplaceTinker`,
        // the SAME flags ARG's own Ratfolk alternates already claim: two
        // different books' alternates legitimately replacing the same base
        // trait share its one replace-flag by PCGen's own design). Every
        // flag is claimed by Ratfolk's own standard row (`ingest_races.rs`
        // wrote its `!PREFACT` gates from Ratfolk's globalvar file, as
        // every other race's do), so the orphan-flag assertion above still
        // does not move.
        //
        // A sibling SD-32 card-11 T2b lane's `inner_sea_races` stale-regen
        // fix (2026-08-22) moved this 139 -> 144: its 9 new alternates'
        // flags are mostly reuses of already-claimed names (two different
        // books' alternates legitimately sharing one base trait's
        // replace-flag), with 5 genuinely new (re-derive: diff this file's
        // `ALTERNATE_TRAIT_REPLACE_FLAGS` additions for
        // Catfolk/Gillman/Kitsune/Nagaji/Ratfolk/Strix/Vanara/Vishkanya/
        // Wayang against the flag set already present before them). Every
        // flag is claimed by its own race's standard row, so the
        // orphan-flag assertion above still does not move.
        //
        // SD-33 Epic 6 fold (2026-08-26) moved this 144 -> 148: the folded
        // Skinwalker heritage batch's 4 brand-new `Skinwalker_Replace*`
        // flags (AbilityScores, AnimalMinded, ChangeShape,
        // SpellLikeAbility) -- shared across all 9 kins (each kin's own
        // selector and its 4 replacement rows all name the SAME 4 flags,
        // not 9 distinct sets, since PCGen scopes the names to the race, not
        // the heritage), all in Skinwalker's own new namespace, none
        // colliding with any earlier race's. Every one is claimed by
        // Skinwalker's own pre-existing standard row (`ingest_races.rs`'s
        // SD-31 Epic 1 follow-on batch wrote the `!PREFACT:1,ABILITIES,
        // Skinwalker_Replace<Trait>=True` gates on `Skinwalker ~ Ability
        // Scores`/`~ Animal-Minded`/`~ Change Shape`/`~ Spell-Like Ability`
        // from the same chassis file), so the orphan-flag assertion above
        // still does not move.
        assert_eq!(all_flags.len(), 148);
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
            415,
            "153 ARG + 8 Monster Codex (the original 4 -- Duergar's Ironskinned/Twilight-\
             Touched, Goblin's the two Oversized replacement rows -- plus SD-32 card-11 T2b's \
             4 Ratfolk alternates, 2026-08-23: Cheek Pouches/Cleanliness/Lab Rat/Surface \
             Sprinter. Surface Sprinter's own two replacement rows, `~ Speed`/`~ Vision`, are \
             `FlagGranted` via its own `ABILITY:...AUTOMATIC...` token, same as Strix's \
             Wing-Clipped below, so they are not counted here) + 1 APG + 76 Inner Sea Races \
             (67 pre-existing + 9 from a sibling SD-32 card-11 T2b lane's stale-regen fix, \
             2026-08-22 -- Vishkanya ~ Deceptive's own dependent row is `FlagGranted`, not \
             counted here) + \
             41 Horror Adventures + \
             48 SD-31 Epic 1-F2 Bestiary 2 batch (ARG's 42 + Inner Sea Races' 6, 2026-08-15) + \
             SD-31-E6-F4-003's 19 (2026-08-16, ARG's own 6-race chassis batch's alternates, \
             minus Strix's Wing-Clipped-granted Flight and Suli's Energy-Strike-granted \
             Earthfoot/Firehand/Icewalk/Shockshield -- those 5 are `FlagGranted`, never \
             offered by the picker) + SD31-E6-F4-006's 8 (2026-08-17, ARG's own follow-on \
             4-race batch's alternates -- Gillman 3, Nagaji 1, Vanara 2, Vishkanya 2). \
             **282, not 283, since 2026-08-12** (SD-29 \
             `decisions.md` 53): Inner Sea Races' \
             `Elf ~ Sovyrian-Born` carries `NAMEISPI:YES`, PCGen's own declaration that the \
             record NAME is Product Identity, and a name cannot be redacted -- so the row is \
             dropped, not screened. \
             + SD-33 Epic 6's 45 folded Skinwalker heritage records (2026-08-26): 9 kin \
             selectors + their 36 replacement rows, all `TraitRole::Alternate` (see the \
             `Skinwalker` section comment on `ALTERNATE_TRAIT_REPLACE_FLAGS`); every one \
             resolves with no inert flag, same as the other 370."
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
    fn selecting_every_alternate_at_once_stays_consistent_for_every_in_scope_race() {
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
        // Dhampir gained a chassis + standard-tier traits, SD-32 card-11 T2b
        // lane (2026-08-23); it now resolves like any other B2 race.
        assert_eq!(corpus.resolve_key("race:dhampir"), Some("Dhampir"));
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
    fn the_hand_modelled_race_size_table_matches_the_corpus_for_all_in_scope_races() {
        let corpus = all_books();
        assert_eq!(
            RACE_SIZES.len(),
            39,
            "18 original + SD-31 Epic 1-F2's Bestiary 2 batch of 6 + the Skinwalker follow-on \
             batch + SD-31-E6-F4-002's Advanced Race Guide batch of 6 (2026-08-16) + \
             SD31-E6-F4-004's Advanced Race Guide follow-on batch of 4 (2026-08-17) + \
             SD31-E6-F4-007's Advanced Race Guide follow-on batch of 2 (2026-08-17: \
             Changeling, Samsaran), closing `arg_races.lst`'s full 37-row roster + SD-31 \
             wave-24's Rougarou (Bestiary 6, 2026-08-20) + SD-32 card-11 T2b lane's Dhampir \
             (Bestiary 2, 2026-08-23)"
        );
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
        // Dhampir gained a chassis + standard-tier traits, SD-32 card-11
        // T2b lane (2026-08-23); it now resolves like any other B2 race.
        assert_eq!(race_size_for_race_token("race:dhampir"), Some(SizeCategory::Medium));
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
            415,
            "153 ARG + 8 Monster Codex (4 original + SD-32 card-11 T2b's 4 Ratfolk \
             alternates, 2026-08-23) + 1 APG + 76 Inner Sea Races (67 pre-existing + 9 from \
             a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22) + \
             41 Horror Adventures + \
             48 SD-31 Epic 1-F2 Bestiary 2 batch (ARG's 42 + Inner Sea Races' 6) + \
             SD-31-E6-F4-003's 19 (2026-08-16, ARG's own 6-race chassis batch) + \
             SD31-E6-F4-006's 8 (2026-08-17, ARG's own follow-on 4-race chassis batch) \
             selectable alternates + SD-33 Epic 6's 45 folded Skinwalker heritage records \
             (2026-08-26: 9 kin selectors + their 36 replacement rows -- see the table's own \
             `Skinwalker` section comment for why all 45, not just the 9, are `Alternate` here)"
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
        // 370 -> 415 by SD-33 Epic 6's fold (2026-08-26): Skinwalker's 45
        // folded heritage records (see `ALTERNATE_TRAIT_REPLACE_FLAGS`'s own
        // `Skinwalker` section comment for why all 45, not just the 9 kin
        // selectors, are `Alternate` here).
        assert_eq!(selectable_alternate_trait_keys().len(), 415);
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

    /// `decisions.md §16` item 2 / SD-32 card-11 T2b: the "Adoptive
    /// Parentage" selector resolves to the race it adopts, corpus-wide, by
    /// class, not by instance. `arg_abilities_race.lst`'s `###Block:
    /// Adoptive Parentage Options` names exactly 7 rows against the pinned
    /// oracle (Dwarf, Elf, Gnome, Halfling, Orc, Drow, Grippli — verified
    /// directly, `grep -c` over the corrected census script). Every one of
    /// the 7 targets a race already chassis-modelled in this same corpus,
    /// and both of its two grant targets (`<Race> ~ Weapon Familiarity`,
    /// `<Race> ~ Languages`) are already-ingested standard traits — so this
    /// is a real, closed grant link, not a browse-only stub: unlike
    /// `bestiary_6`'s Rougarou row (proven empty corpus-wide, excluded, not
    /// ingested), this shape has real content on the other end and is
    /// therefore ingested.
    #[test]
    fn adoptive_parentage_resolves_all_seven_arg_options_to_a_modelled_race_with_real_grants() {
        let corpus = all_books();
        let options = adoptive_parentage_options(&corpus);
        let keys: Vec<&str> = options.iter().map(|o| o.key.as_str()).collect();
        assert_eq!(
            keys,
            vec!["Drow", "Dwarf", "Elf", "Gnome", "Grippli", "Halfling", "Orc"],
            "exactly ARG's 7 Adoptive Parentage rows, sorted by key"
        );
        for option in &options {
            assert_eq!(option.book_id, "advanced_race_guide");
            assert_eq!(
                option.adopted_race, option.key,
                "every ARG Adoptive Parentage row's own KEY is its target race's name"
            );
            assert!(
                corpus.chassis(&option.adopted_race).is_some(),
                "{:?} must resolve to a race with its own chassis record in this corpus",
                option.adopted_race
            );
            assert!(
                option.description.as_deref().is_some_and(|d| !d.trim().is_empty()),
                "{:?} must carry real corpus prose, not a fabricated placeholder",
                option.key
            );
            assert!(
                option.unresolved_grants.is_empty(),
                "{:?}: every ARG Adoptive Parentage grant target must resolve against this \
                 project's own already-ingested standard traits; a gap here would mean this \
                 option grants nothing real, {:?}",
                option.key,
                option.unresolved_grants
            );
            let grant_names: Vec<&str> = option.grants.iter().map(|g| g.name.as_str()).collect();
            assert_eq!(
                grant_names,
                vec!["Weapon Familiarity", "Languages"],
                "{:?}: ARG's own row grants exactly these two already-modelled traits, in this \
                 order (`ABILITY:<Race> Racial Trait|AUTOMATIC|<Race> ~ Weapon Familiarity|<Race> \
                 ~ Languages`, verbatim against the pinned oracle)",
                option.key
            );
        }
    }

    /// SD-32 `decisions.md §25` cycle 2: `adopted_race_choose_selectors`
    /// finds the real, on-disk 14-unit population this cycle's new
    /// `selector_only` `BookSource`s ingested -- the exact population named
    /// in the epic's own acceptance criterion (bestiary_2 7, bestiary_3 5,
    /// bestiary_5 1, bestiary_6 1).
    #[test]
    fn adopted_race_choose_selectors_finds_the_real_fourteen_unit_population() {
        let corpus = all_books();
        let selectors = adopted_race_choose_selectors(&corpus);
        let races: Vec<&str> = selectors.iter().map(|s| s.adopted_race.as_str()).collect();
        assert_eq!(
            races,
            vec![
                "Catfolk", "Dhampir", "Fetchling", "Grippli", "Ifrit", "Oread", "Ratfolk", "Rougarou",
                "Skinwalker", "Suli", "Sylph", "Undine", "Vanara", "Vishkanya",
            ],
            "exactly the 14 target races, sorted by key (\"Adopted Race ~ <Race>\")"
        );
        assert_eq!(selectors.len(), 14, "decisions.md §25's own population figure");
        for selector in &selectors {
            assert_eq!(selector.key, format!("Adopted Race ~ {}", selector.adopted_race));
            assert_eq!(
                selector.pool_type_suffix.as_deref(),
                Some(format!("{} Race Trait", selector.adopted_race)).as_deref(),
                "{:?}: every real oracle row's CHOOSE: pool suffix is \"<Race> Race Trait\", read, \
                 never guessed",
                selector.key
            );
        }
        // All 14 target races now resolve to a real chassis record --
        // `ingest_races.rs` has since given Dhampir/Skinwalker/Rougarou their
        // own chassis (SD-31 wave-24 and SD-32 card-11 T2b, both landed after
        // `epic-6-kind-trait_cycle-1_cycle_receipt.md §2` wrote the "three
        // races with no chassis" finding this test corrects). The selector
        // shape itself still needs no chassis to be admitted -- its pool
        // resolves against `crate::rules_core::trait_pool`, never
        // `RaceCorpus::traits_for` -- so `ingest_book`'s bypass stays even
        // though it currently has nothing left to bypass for.
        let without_chassis: Vec<&str> =
            selectors.iter().filter(|s| corpus.chassis(&s.adopted_race).is_none()).map(|s| s.adopted_race.as_str()).collect();
        assert!(without_chassis.is_empty(), "re-derive this test if a future book removes a chassis: {without_chassis:?}");
    }

    /// An option's target race resolving is not enough on its own —
    /// `traits_by_category` must be scoped to the exact category string, not
    /// a substring or a case-insensitive match, so a future book's
    /// differently-cased or differently-worded category never silently joins
    /// this population.
    #[test]
    fn traits_by_category_is_an_exact_match_not_a_substring() {
        let corpus = all_books();
        assert!(corpus.traits_by_category("Adoptive").is_empty());
        assert!(corpus.traits_by_category("adoptive parentage").is_empty());
        assert_eq!(corpus.traits_by_category(ADOPTIVE_PARENTAGE_CATEGORY).len(), 7);
    }
}
